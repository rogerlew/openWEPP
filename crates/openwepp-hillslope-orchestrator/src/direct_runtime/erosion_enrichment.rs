//! Wave-1 particle-class enrichment (E.4 / Increment 3): the `enrich.for`
//! port. The flow composition `frcflw` is per-quantum solver state,
//! blended at every enrichment call point (`enrich.for` do-10),
//! re-proportioned through each deposition region (do-30 analytic
//! per-class solution + the label-50 `sedmax` reproportion loop), and
//! summarized at the OFE end as the specific-surface-area enrichment
//! ratio (`iendfg` arm). Source-intent: `enrich.for` +
//! `route.for:235/250/448/473` call points. The TOTAL load remains the
//! routing authority (`SC-SED-001` hold criterion: the class solve
//! re-normalizes to the total, never redefines it).

use super::{DirectRuntimeError, ErosionParticleClass, validate_finite};

/// Class-count alias (the enrichment arrays are all 5-class).
const CLASS_COUNT: usize = 5;
/// Legacy `phi` caps (`enrich.for`: ±100000).
const ENRICH_MAX_PHI: f64 = 100_000.0;
/// Legacy `pkro` significance threshold (`enrich.for`: 1e-15).
const ENRICH_PKRO_ZERO: f64 = 1.0e-15;
/// Legacy `term4a` underflow floor (`enrich.for`: 1e-8).
const ENRICH_TERM4A_FLOOR: f64 = 1.0e-8;
/// Legacy per-class load floor after normalization (`enrich.for`: 1e-15).
const ENRICH_GEND_FLOOR: f64 = 1.0e-15;
/// Legacy `ldtop` activation threshold for the do-10 blend (1e-5).
const ENRICH_BLEND_LDTOP_MIN: f64 = 1.0e-5;
/// Documented deviation: the legacy label-50 reproportion loop
/// (`go to 50` while any class caps at `sedmax`) has no iteration bound;
/// a bounded port fails closed instead of spinning.
const ENRICH_MAX_REPROPORTION_ITERS: usize = 64;
/// Legacy SSA constants (`enrich.for:187-190`), m²/g-class scale.
const SSA_SAND: f64 = 0.05;
const SSA_SILT: f64 = 4.0;
const SSA_CLAY: f64 = 20.0;
const SSA_ORG: f64 = 1000.0;

/// Static per-quantum enrichment operands (assembled with the quantum).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct Wave1EnrichmentInputs {
    /// The five classes with mineralogy (`prtcmp` per-OFE lineage).
    pub classes: [ErosionParticleClass; CLASS_COUNT],
    /// Per-class transport shares at the normalizing shear
    /// (`yalin.for:157` `tcf1`).
    pub tcf1: [f64; CLASS_COUNT],
    /// Interrill-detached composition. Non-cropland `fidel = frac`
    /// (`param.for:452-458`) — exact on the enabled no-tillage scope;
    /// the cropland `drinti` composition is a labeled extension point.
    pub fidel: [f64; CLASS_COUNT],
    /// Surface-soil specific surface area (`enrich.for:148-151`
    /// `ssasol`), per-OFE.
    pub ssasol: f64,
    /// Upstream flow composition for the `route.for:142-160`
    /// initialization; `None` for a no-inflow quantum (local `frac`).
    pub inflow_fractions: Option<[f64; CLASS_COUNT]>,
}

/// Mutable per-quantum enrichment state threaded through the route.
#[derive(Debug, Clone, PartialEq)]
pub struct Wave1EnrichmentState {
    /// Flow composition `frcflw` (unit-sum while flow exists).
    pub frcflw: [f64; CLASS_COUNT],
    /// Load at the end of the previous deposition region / top of the
    /// current detachment stretch (`route.for` `lddend`).
    pub lddend: f64,
    /// Distance at the start of the current detachment stretch
    /// (`route.for` `xdetst`).
    pub xdetst: f64,
    /// Specific-surface-area enrichment ratio, set by the terminal call.
    pub enrichment_ratio: Option<f64>,
}

impl Wave1EnrichmentState {
    /// `route.for:117/123` + `:142-160`: initialize the flow composition —
    /// inflow present carries the upstream composition, else the local
    /// detached `frac`; no outflow zeroes the composition.
    #[must_use]
    pub fn initialize(
        inputs: &Wave1EnrichmentInputs,
        qout_positive: bool,
        qin_positive: bool,
        strldn: f64,
    ) -> Self {
        let frcflw = if !qout_positive {
            [0.0; CLASS_COUNT]
        } else if qin_positive {
            match inputs.inflow_fractions {
                Some(fractions) => fractions,
                None => core::array::from_fn(|index| inputs.classes[index].frac),
            }
        } else {
            core::array::from_fn(|index| inputs.classes[index].frac)
        };
        Self {
            frcflw,
            lddend: strldn,
            xdetst: 0.0,
            enrichment_ratio: None,
        }
    }

    /// `enrich.for` do-10: blend the composition of the load reaching a
    /// call point from the inflow-surviving, rill-detached, and
    /// interrill-detached contributions.
    fn blend(&mut self, inputs: &Wave1EnrichmentInputs, xtop: f64, ldtop: f64, theta: f64) {
        if ldtop <= ENRICH_BLEND_LDTOP_MIN {
            return;
        }
        let intlod = theta * (xtop - self.xdetst);
        let rillod = (ldtop - self.lddend - intlod).max(0.0);
        for (index, fraction) in self.frcflw.iter_mut().enumerate() {
            *fraction = (*fraction * self.lddend
                + inputs.classes[index].frac * rillod
                + inputs.fidel[index] * intlod)
                / ldtop;
        }
    }

    /// One deposition-region enrichment call (`route.for:235/250/448`
    /// sites): do-10 blend, then the do-30 per-class analytic
    /// re-proportion over `[xtop, xbot]` with the label-50 `sedmax`
    /// reproportion. Caller updates `lddend`/`xdetst` afterwards, exactly
    /// like the legacy call sites.
    // Single straight-line port of the `enrich.for` do-30 + label-50
    // block; splitting it would scatter the legacy control flow.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub fn deposition_region(
        &mut self,
        inputs: &Wave1EnrichmentInputs,
        operands: &Wave1EnrichmentRegionOperands,
        xtop: f64,
        xbot: f64,
        ldtop: f64,
        ldbot: f64,
    ) -> Result<(), DirectRuntimeError> {
        self.blend(inputs, xtop, ldtop, operands.theta);
        if operands.qout_m2_s <= 0.0 {
            return Ok(());
        }

        let pkro = operands.pkro;
        let tmpvr2 = xbot + operands.qostar;
        let tmpvr3 = xtop + operands.qostar;
        let tmpvr4 = tmpvr2 * tmpvr2;
        let tmpvr5 = tmpvr3 * tmpvr3;

        let mut gend = [0.0_f64; CLASS_COUNT];
        let mut gu = [0.0_f64; CLASS_COUNT];
        let mut ftheta = [0.0_f64; CLASS_COUNT];
        let mut sumg = 0.0;
        for index in 0..CLASS_COUNT {
            let tmpvr1 = operands.ktrato * inputs.tcf1[index];
            let aa = tmpvr1 * operands.atc;
            let bb = tmpvr1 * operands.btc;
            let cc = tmpvr1 * operands.ctc;
            ftheta[index] = inputs.fidel[index] * operands.theta;
            gu[index] = self.frcflw[index] * ldtop;
            let phi = if pkro.abs() > ENRICH_PKRO_ZERO {
                (operands.beta * inputs.classes[index].fall_m_s / pkro)
                    .clamp(-ENRICH_MAX_PHI, ENRICH_MAX_PHI)
            } else if operands.qostar >= 0.0 {
                ENRICH_MAX_PHI
            } else {
                -ENRICH_MAX_PHI
            };
            let mut ratio = tmpvr3 / tmpvr2;
            if operands.qostar >= 0.0 && ratio > 1.0 {
                ratio = 1.0;
            }
            let (ratio2, expon) = super::wave1_undflo(ratio, phi);
            let coef1 = phi * aa / (phi + 2.0);
            let coef2 = (phi * bb + ftheta[index] - 2.0 * aa * phi * operands.qostar) / (1.0 + phi);
            let term1 = coef1 * tmpvr4;
            let term2 = coef2 * tmpvr2;
            let term3 = aa * operands.qostar * operands.qostar - bb * operands.qostar + cc;
            // Legacy `term4a`/`term4b` — renamed for the lint, same math.
            let mut survivor_factor = ratio2.powf(expon);
            if survivor_factor < ENRICH_TERM4A_FLOOR {
                survivor_factor = 0.0;
            }
            let survivor_load = gu[index] - coef1 * tmpvr5 - coef2 * tmpvr3 - term3;
            gend[index] = (term1 + term2 + term3 + survivor_factor * survivor_load).max(0.0);
            validate_finite("erosion.enrich.gend", gend[index])?;
            sumg += gend[index];
        }

        if sumg > 0.0 {
            // Normalize to the routed total (`enrich.for` do-40): the
            // TOTAL load stays authority; the class solve only shapes it.
            let mut sedmax = [0.0_f64; CLASS_COUNT];
            let mut floor_applied = false;
            for index in 0..CLASS_COUNT {
                gend[index] *= ldbot / sumg;
                if gend[index] < ENRICH_GEND_FLOOR {
                    gend[index] = ENRICH_GEND_FLOOR;
                    floor_applied = true;
                }
                sedmax[index] = gu[index] + ftheta[index] * (xbot - xtop);
            }
            // The pinned absolute floor can inflate a trace-load class
            // vector above `ldbot`. Restore the routed total before the
            // label-50 caps so its remaining-mass redistribution cannot
            // manufacture negative class mass. Keep the ordinary no-floor
            // path bitwise unchanged.
            if floor_applied {
                let floored_total: f64 = gend.iter().sum();
                let mass_scale = ldbot / floored_total;
                for class_load in &mut gend {
                    *class_load *= mass_scale;
                }
            }
            // Label-50 reproportion: cap classes at their maximum
            // available mass; redistribute the shortfall over the
            // uncapped classes proportionally; iterate until stable.
            let mut iterations = 0;
            loop {
                let mut ratbot = 0.0;
                let mut total = 0.0;
                let mut capped = false;
                for index in 0..CLASS_COUNT {
                    if gend[index] > sedmax[index] {
                        gend[index] = sedmax[index];
                        capped = true;
                    } else if gend[index] < sedmax[index] {
                        ratbot += gend[index];
                    }
                    total += gend[index];
                }
                if !capped {
                    break;
                }
                if ratbot > 0.0 {
                    let shortfall = ldbot - total;
                    for index in 0..CLASS_COUNT {
                        if gend[index] < sedmax[index] {
                            gend[index] += shortfall * gend[index] / ratbot;
                        }
                    }
                }
                iterations += 1;
                if iterations >= ENRICH_MAX_REPROPORTION_ITERS {
                    return Err(DirectRuntimeError::DirectDomainViolation {
                        field: "erosion.enrich.reproportion_iterations",
                    });
                }
            }
            let total: f64 = gend.iter().sum();
            if total > 0.0 {
                for (index, fraction) in self.frcflw.iter_mut().enumerate() {
                    *fraction = gend[index] / total;
                }
            } else {
                self.frcflw = [0.0; CLASS_COUNT];
            }
        } else {
            self.frcflw = [0.0; CLASS_COUNT];
        }
        Ok(())
    }

    /// The terminal OFE-end call (`route.for:473`, `iendfg = 1`):
    /// do-10 blend at `xtop = 1` with `ldtop = ldbot = ldlast`, then the
    /// specific-surface-area enrichment ratio.
    pub fn terminal(
        &mut self,
        inputs: &Wave1EnrichmentInputs,
        theta: f64,
        ldlast: f64,
        qout_positive: bool,
    ) -> Result<(), DirectRuntimeError> {
        self.blend(inputs, 1.0, ldlast, theta);
        let mut sumssa = 0.0;
        for (index, fraction) in self.frcflw.iter_mut().enumerate() {
            if !qout_positive {
                *fraction = 0.0;
            }
            let class = &inputs.classes[index];
            let ssased = *fraction
                * ((class.frsnd * SSA_SAND + class.frslt * SSA_SILT + class.frcly * SSA_CLAY)
                    / (1.0 + class.frorg)
                    + class.frorg * SSA_ORG / 1.73);
            sumssa += ssased;
        }
        if inputs.ssasol <= 0.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.enrich.ssasol",
            });
        }
        let ratio = sumssa / inputs.ssasol + 0.005;
        validate_finite("erosion.enrich.enrato", ratio)?;
        self.enrichment_ratio = Some(ratio);
        Ok(())
    }

    /// Composition sanity envelope (`TOL-SED-006`, `INV-SED-017` (c)):
    /// legacy NEVER re-normalizes after a do-10 blend — when `rillod`
    /// floors on a transport-capacity-limited stretch the blend sum
    /// legitimately exceeds 1 by percent scale, and the legacy ER
    /// consumes that raw sum (`enrich.for` has no gate at all). Only a
    /// do-30 re-proportion normalizes exactly. The gate here is
    /// therefore a CORRUPTION envelope (`[0.5, 1.5]` while flow
    /// exists), not a closure law; the published per-class split keeps
    /// its own tight `TOL-SED-005` closure by normalizing at the
    /// publication boundary.
    pub fn validate_unit_sum(&self, qout_positive: bool) -> Result<(), DirectRuntimeError> {
        if !qout_positive {
            return Ok(());
        }
        let sum: f64 = self.frcflw.iter().sum();
        if sum != 0.0 && !(0.5..=1.5).contains(&sum) {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "erosion.enrich.frcflw_unit_sum",
            });
        }
        Ok(())
    }
}

/// The per-region operands the route supplies at each enrichment call
/// point (segment transport coefficients + the day drivers).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wave1EnrichmentRegionOperands {
    pub atc: f64,
    pub btc: f64,
    pub ctc: f64,
    pub ktrato: f64,
    pub qostar: f64,
    pub theta: f64,
    pub beta: f64,
    /// The deposition driver `(qout − qin)/slplen` — the SAME operand the
    /// total-load `phi` uses (`param.for:593-625`).
    pub pkro: f64,
    pub qout_m2_s: f64,
}

/// `enrich.for:148-151`: the surface-soil specific surface area from the
/// OFE's texture + organic matter — the ER denominator, per-OFE.
pub fn erosion_surface_soil_ssa(
    sand: f64,
    silt: f64,
    clay: f64,
    orgmat: f64,
) -> Result<f64, DirectRuntimeError> {
    for (field, value) in [
        ("erosion.enrich.ssa_sand", sand),
        ("erosion.enrich.ssa_silt", silt),
        ("erosion.enrich.ssa_clay", clay),
        ("erosion.enrich.ssa_orgmat", orgmat),
    ] {
        validate_finite(field, value)?;
        if value < 0.0 {
            return Err(DirectRuntimeError::DirectDomainViolation { field });
        }
    }
    let ssasol = orgmat * SSA_ORG / 1.73
        + (sand * SSA_SAND + silt * SSA_SILT + clay * SSA_CLAY) / (1.0 + orgmat);
    validate_finite("erosion.enrich.ssasol", ssasol)?;
    Ok(ssasol)
}
