//! Wave-1 event/transport operand producers (erosion port Increment-1b-A).
//!
//! Source-intent port (ADR-0024, baseline `dac3c950`) of the legacy
//! erosion operand chain that feeds the normalized continuity solve in
//! [`super::erosion_continuity`]: particle composition (`prtcmp.for`),
//! fall velocity (`falvel.for` + the drag tables from `inidat.for`),
//! the effective particle surface (`param.for`), sediment transport
//! (`shield.for`/`yalin.for`/`trcoef.for`), rill hydraulics
//! (`frcfac.for`/`shears.for`), and the interrill detachment rate
//! (`param.for`). Governing contract: `SC-SED-001` (INV-SED-004 hydrologic
//! inputs, INV-SED-005 shear partition, INV-SED-006 transport capacity,
//! INV-SED-007 normalized parameters). Legacy is source-intent authority,
//! never a magnitude oracle (ADR-0017).
//!
//! These producers are pure and typed. The static pieces (particle
//! classes, effective particle) depend only on soil texture and run once
//! per lane; the transport/hydraulics pieces (`shrsol`, `kt`/`ktrato`,
//! `tcend`, `detinr`) depend on the daily peak-runoff discharge and run
//! per OFE-day. Fail-closed: invalid domains are typed errors, never
//! defaults.

use super::{DirectRuntimeError, validate_finite};

/// Number of erosion particle-size classes (`prtcmp.for` primary
/// clay / silt / small-aggregate / large-aggregate / sand).
pub const EROSION_PARTICLE_CLASS_COUNT: usize = 5;

// Legacy physical constants (`inidat.for:1054-1151`).
const ACCGAV: f64 = 9.807;
const WTDENS: f64 = 9807.0;
const KINVIS: f64 = 1.0e-6;
const MSDENS: f64 = 1000.0;
// `shears.for:71` Chezy depth-iteration tolerance (Baffaut 1996).
const SHEARS_DEPTH_TOL: f64 = 5.0e-6;
// `shears.for:76` slope floor.
const SHEARS_SLOPE_FLOOR: f64 = 0.000_001;
// `shears.for:119` wetted-perimeter guard.
const SHEARS_WP_FLOOR: f64 = 1.0e-12;
// `param.for:234` transport-capacity normalization floor.
const TCEND_FLOOR: f64 = 1.0e-10;
// `trcoef.for` zero-coefficient floor.
const TRCOEF_FLOOR: f64 = 1.0e-9;
// `yalin.for:116` zero-total protection.
const YALIN_ZERO_TOTAL: f64 = 1000.0;
// `yalin.for:143` sandy transport adjustment floor (INV-SED-006).
const YALIN_SANDY_ADJ_FLOOR: f64 = 0.30;
// `param.for:417-419` interrill roughness delivery bounds.
const RIF_SLOPE: f64 = -23.0;
const RIF_INTERCEPT: f64 = 1.14;

/// One rainfall hyetograph interval resolved against its infiltration
/// excess, for the effective-intensity/duration producer (`reid.for`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionExcessInterval {
    /// Interval duration (s).
    pub duration_s: f64,
    /// Raw rainfall intensity over the interval (m/s) — the `rrate` that
    /// drives interrill detachment energy (`reid.for` `sumint`).
    pub rainfall_intensity_m_s: f64,
    /// Infiltration excess generated in the interval (m); `> 0` marks an
    /// excess period (`reid.for` `re(i) > 0`).
    pub excess_m: f64,
    /// Whether snowmelt drove the interval; on snowmelt intervals the
    /// rainfall intensity is excluded from `sumint` (`reid.for:69`
    /// `if (smrate.le.0.0)`), but the interval still counts toward
    /// `durre`.
    pub snowmelt_active: bool,
}

/// Effective rainfall intensity and rainfall-excess duration
/// (`reid.for`/`grna.for:607`): `effdrr = durre` = Σ durations of
/// excess-producing intervals; `effint = sumint/durre` = the mean
/// rainfall intensity over excess periods excluding snowmelt intervals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionEffectiveIntensity {
    pub effint_m_s: f64,
    pub effdrr_s: f64,
}

/// `reid.for`: accumulate `durre`/`sumint` over the rainfall-excess
/// intervals and return `effint = sumint/durre`, `effdrr = durre`.
///
/// This is the faithful production form of the operands that the
/// Increment-1b-A integration test previously approximated as
/// `effint = runoff/effdrr`, `effdrr = effdrn` — that approximation
/// substituted the mean *excess* rate for the mean *rainfall* intensity,
/// which understates the interrill `Di = Ki·I·q` driver. Here `sumint`
/// uses the raw rainfall intensity (`rrate`) over excess periods, matching
/// legacy. Returns zeros when there is no excess period (no interrill
/// supply). Fail-closed on NaN / negative inputs.
pub fn erosion_effective_intensity(
    intervals: &[ErosionExcessInterval],
) -> Result<ErosionEffectiveIntensity, DirectRuntimeError> {
    let mut durre = 0.0_f64;
    let mut sumint = 0.0_f64;
    for interval in intervals {
        validate_finite("erosion.effint.duration_s", interval.duration_s)?;
        validate_finite(
            "erosion.effint.rainfall_intensity_m_s",
            interval.rainfall_intensity_m_s,
        )?;
        validate_finite("erosion.effint.excess_m", interval.excess_m)?;
        if interval.duration_s < 0.0
            || interval.rainfall_intensity_m_s < 0.0
            || interval.excess_m < 0.0
        {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.effint.negative_input",
            });
        }
        // Excess period (`reid.for:52` `if (re(i).gt.0.)`).
        if interval.excess_m > 0.0 {
            durre += interval.duration_s;
            // Rainfall intensity contributes to sumint only when snowmelt
            // is not driving the interval (`reid.for:69`).
            if !interval.snowmelt_active {
                sumint += interval.duration_s * interval.rainfall_intensity_m_s;
            }
        }
    }
    if durre <= 0.0 {
        return Ok(ErosionEffectiveIntensity {
            effint_m_s: 0.0,
            effdrr_s: 0.0,
        });
    }
    let effint_m_s = sumint / durre;
    validate_finite("erosion.effint.effint_m_s", effint_m_s)?;
    Ok(ErosionEffectiveIntensity {
        effint_m_s,
        effdrr_s: durre,
    })
}

/// One erosion particle-size class (`prtcmp.for` surface).
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct ErosionParticleClass {
    /// Equivalent particle diameter (m).
    pub dia_m: f64,
    /// Particle specific gravity.
    pub spg: f64,
    /// Detached mass fraction of this class.
    pub frac: f64,
    /// Still-water fall velocity (m/s).
    pub fall_m_s: f64,
    /// Primary-clay mass fraction of this class (`prtcmp.for:208-…`).
    pub frcly: f64,
    /// Primary-silt mass fraction of this class.
    pub frslt: f64,
    /// Primary-sand mass fraction of this class.
    pub frsnd: f64,
    /// Organic-matter mass fraction of this class.
    pub frorg: f64,
}

/// Soil texture triple (mass fractions of the surface layer) plus the
/// rill-friction cover surfaces needed by the operand producers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionTextureInputs {
    pub sand: f64,
    pub clay: f64,
    pub silt: f64,
    pub orgmat: f64,
}

/// `falvel.for`: particle fall velocity from the drag-coefficient table
/// (`inidat.for:1017-1034`), with the Stokes small-particle branch.
/// Table overflow (larger than the tabulated range) uses the last
/// tabulated Reynolds value, matching the legacy warning-and-continue.
// The `cdre`/`cdre2` tables are pinned legacy literals (the ln-decade
// values coincide with LN_10 multiples but are the drag-table entries).
#[allow(clippy::approx_constant)]
#[must_use]
pub fn erosion_falvel(spg: f64, dia_m: f64) -> f64 {
    const CDRE: [f64; 9] = [
        -6.907_75, -4.605_17, -2.302_58, 0.0, 2.302_58, 4.605_17, 6.907_75, 9.210_34, 11.512_92,
    ];
    const CDRE2: [f64; 9] = [
        -4.509_86, -1.514_13, 0.788_46, 3.126_76, 6.040_25, 9.305_65, 13.081_54, 17.504_39,
        22.291_88,
    ];
    // rtsid = cd*re^2 drag parameter (`falvel.for:104`).
    let rtsid = ((spg - 1.0) * ACCGAV * dia_m.powi(3) / (KINVIS * KINVIS)) * (8.0 / 6.0);
    if rtsid >= 0.024 {
        let target = rtsid.ln();
        for i in 1..9 {
            if CDRE2[i] > target {
                let rey = ((target - CDRE2[i - 1]) / (CDRE2[i] - CDRE2[i - 1])
                    * (CDRE[i] - CDRE[i - 1])
                    + CDRE[i - 1])
                    .exp();
                return rey * KINVIS / dia_m;
            }
        }
        CDRE[8].exp() * KINVIS / dia_m
    } else {
        (dia_m * dia_m * (spg - 1.0) * ACCGAV) / (KINVIS * 18.0)
    }
}

/// `prtcmp.for`: five-class particle composition from surface-layer
/// texture, including the large-aggregate clay-enrichment correction
/// re-entry (`jflag`) and the mm→m diameter conversion at `:333`.
// Legacy naming continuity (`frac`/`fracs`/`frclyt`) and the single
// straight-line port of the `prtcmp.for` fraction block.
#[allow(clippy::similar_names)]
pub fn erosion_particle_composition(
    texture: &ErosionTextureInputs,
) -> Result<[ErosionParticleClass; EROSION_PARTICLE_CLASS_COUNT], DirectRuntimeError> {
    validate_particle_texture(texture)?;
    let sand = texture.sand;
    let clay = texture.clay;
    let silt = texture.silt;
    let (dia_mm, spg, frac1, frac3, frac5, frcly3) =
        particle_primary_fractions_and_diameters(sand, clay, silt);
    let fracs = converge_particle_fractions(clay, silt, frac1, frac3, frac5, frcly3);
    let mineralogy = particle_mineralogy(texture, &fracs, frcly3)?;
    assemble_particle_classes(&dia_mm, &spg, &fracs, &mineralogy)
}

fn validate_particle_texture(texture: &ErosionTextureInputs) -> Result<(), DirectRuntimeError> {
    for (field, value) in [
        ("erosion.prtcmp.sand", texture.sand),
        ("erosion.prtcmp.clay", texture.clay),
        ("erosion.prtcmp.silt", texture.silt),
        ("erosion.prtcmp.orgmat", texture.orgmat),
    ] {
        validate_finite(field, value)?;
        if !(0.0..=1.0).contains(&value) {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.prtcmp.texture",
            });
        }
    }
    Ok(())
}

#[allow(clippy::similar_names)]
fn particle_primary_fractions_and_diameters(
    sand: f64,
    clay: f64,
    silt: f64,
) -> ([f64; 5], [f64; 5], f64, f64, f64, f64) {
    let mut dia_mm = [0.002_f64, 0.010, 0.030, 0.300, 0.200];
    let spg = [2.60_f64, 2.65, 1.80, 1.60, 2.65];
    if clay > 0.15 {
        dia_mm[3] = 2.0 * clay;
    }
    let frac1 = if clay > 0.0 && clay < 1.0 {
        0.26 * clay
    } else if clay <= 0.0 {
        0.0001
    } else {
        0.9996
    };
    let mut frac5 = sand * (1.0 - clay).powi(5);
    if frac5 <= 0.0 {
        frac5 = 0.0001;
    }
    let mut frac3;
    if clay >= 1.0 {
        dia_mm[2] = 0.180;
        frac3 = 0.0001;
    } else if clay <= 0.25 {
        dia_mm[2] = 0.030;
        frac3 = 1.8 * clay;
        if frac3 <= 0.0 {
            frac3 = 0.0001;
        }
    } else if clay < 0.60 {
        dia_mm[2] = 0.20 * (clay - 0.25) + 0.030;
        if clay >= 0.50 {
            frac3 = 0.6 * clay;
        } else {
            frac3 = 0.45 - 0.6 * (clay - 0.25);
        }
    } else {
        dia_mm[2] = 0.1;
        frac3 = 0.6 * clay;
    }
    let frcly3 = if clay > 0.0 && silt > 0.0 {
        clay / (clay + silt)
    } else {
        0.0
    };
    (dia_mm, spg, frac1, frac3, frac5, frcly3)
}

fn converge_particle_fractions(
    clay: f64,
    silt: f64,
    frac1: f64,
    mut frac3: f64,
    frac5: f64,
    frcly3: f64,
) -> [f64; EROSION_PARTICLE_CLASS_COUNT] {
    let mut fractions = [0.0_f64; EROSION_PARTICLE_CLASS_COUNT];
    for pass in 0..2 {
        let mut frac2 = silt - frac3;
        let mut frac3_local = frac3;
        if frac2 <= 0.0 {
            frac2 = 0.0001;
            frac3_local = silt - frac2;
            if frac3_local <= 0.0 {
                frac3_local = 0.0001;
            }
        }
        let mut frac4 = 1.0 - frac1 - frac2 - frac3_local - frac5;
        fractions = [frac1, frac2, frac3_local, frac4, frac5];
        if frac4 <= 0.0 {
            let crct = 1.0 / (1.0 + frac4.abs() + 0.0001);
            fractions[3] = 0.0001;
            for value in &mut fractions {
                *value *= crct;
            }
            frac4 = fractions[3];
        }

        if pass == 1 {
            break;
        }
        // Large-aggregate clay-content correction (`prtcmp.for:288-300`).
        let frcly4 = if frac4 > 0.0001 {
            let value = (clay - fractions[0] - frcly3 * fractions[2]) / frac4;
            if (0.0..=1.0).contains(&value) {
                value
            } else {
                0.0
            }
        } else {
            0.0
        };
        let target_clay_fraction = 0.5 * clay;
        let frcly1 = 0.95 * target_clay_fraction;
        if clay < 1.0 && frcly4 < frcly1 && (frcly3 - target_clay_fraction).abs() > 0.0 {
            let f1f2f5 = fractions[0] + fractions[1] + fractions[4];
            frac3 = (clay - target_clay_fraction - fractions[0] + target_clay_fraction * f1f2f5)
                / (frcly3 - target_clay_fraction);
            if frac3 <= 0.0 {
                frac3 = 0.0001;
            }
            continue;
        }
        break;
    }
    fractions
}

fn particle_mineralogy(
    texture: &ErosionTextureInputs,
    fracs: &[f64; EROSION_PARTICLE_CLASS_COUNT],
    frcly3: f64,
) -> Result<[(f64, f64, f64, f64); EROSION_PARTICLE_CLASS_COUNT], DirectRuntimeError> {
    let sand = texture.sand;
    let clay = texture.clay;
    let silt = texture.silt;
    let ratiom = if clay > 0.0 {
        texture.orgmat / clay
    } else if silt > 0.0 {
        texture.orgmat / silt
    } else {
        texture.orgmat / sand
    };
    validate_finite("erosion.prtcmp.ratiom", ratiom)?;
    let clamp_unit = |value: f64| {
        if (0.0..=1.0).contains(&value) {
            value
        } else {
            0.0
        }
    };
    let (frcly4, frslt4, frsnd4) = if fracs[3] > 0.0001 {
        (
            clamp_unit((clay - fracs[0] - frcly3 * fracs[2]) / fracs[3]),
            clamp_unit(
                (silt
                    - fracs[1]
                    - (if clay > 0.0 && silt > 0.0 {
                        silt / (clay + silt)
                    } else {
                        0.0
                    }) * fracs[2])
                    / fracs[3],
            ),
            clamp_unit((sand - fracs[4]) / fracs[3]),
        )
    } else {
        (0.0, 0.0, 0.0)
    };
    Ok([
        // class 1 — primary clay
        (1.0, 0.0, 0.0, if clay > 0.0 { ratiom } else { 0.0 }),
        // class 2 — primary silt
        (
            0.0,
            1.0,
            0.0,
            if clay > 0.0 {
                0.0
            } else if silt > 0.0 {
                ratiom
            } else {
                0.0
            },
        ),
        // class 3 — small aggregate
        if clay > 0.0 && silt > 0.0 {
            (
                clay / (clay + silt),
                silt / (clay + silt),
                0.0,
                (clay / (clay + silt)) * ratiom,
            )
        } else {
            (0.0, 0.0, 0.0, 0.0)
        },
        // class 4 — large aggregate (back-out from the converged fracs)
        (frcly4, frslt4, frsnd4, frcly4 * ratiom),
        // class 5 — primary sand
        (
            0.0,
            0.0,
            1.0,
            if clay > 0.0 || silt > 0.0 {
                0.0
            } else {
                ratiom
            },
        ),
    ])
}

fn assemble_particle_classes(
    dia_mm: &[f64; EROSION_PARTICLE_CLASS_COUNT],
    spg: &[f64; EROSION_PARTICLE_CLASS_COUNT],
    fracs: &[f64; EROSION_PARTICLE_CLASS_COUNT],
    mineralogy: &[(f64, f64, f64, f64); EROSION_PARTICLE_CLASS_COUNT],
) -> Result<[ErosionParticleClass; EROSION_PARTICLE_CLASS_COUNT], DirectRuntimeError> {
    let mut classes = [ErosionParticleClass {
        dia_m: 0.0,
        spg: 0.0,
        frac: 0.0,
        fall_m_s: 0.0,
        frcly: 0.0,
        frslt: 0.0,
        frsnd: 0.0,
        frorg: 0.0,
    }; EROSION_PARTICLE_CLASS_COUNT];
    for (index, class) in classes.iter_mut().enumerate() {
        let dia_m = dia_mm[index] / 1000.0;
        let fall = erosion_falvel(spg[index], dia_m);
        validate_finite("erosion.prtcmp.fall", fall)?;
        let (frcly_class, frslt_class, frsnd_class, frorg_class) = mineralogy[index];
        *class = ErosionParticleClass {
            dia_m,
            spg: spg[index],
            frac: fracs[index],
            fall_m_s: fall,
            frcly: frcly_class,
            frslt: frslt_class,
            frsnd: frsnd_class,
            frorg: frorg_class,
        };
    }
    Ok(classes)
}

/// `param.for:558-579`: effective particle diameter and specific gravity
/// as the fraction-weighted log means of the three smallest classes (the
/// clay / silt / small-aggregate classes).
pub fn erosion_effective_particle(
    classes: &[ErosionParticleClass; EROSION_PARTICLE_CLASS_COUNT],
) -> Result<(f64, f64), DirectRuntimeError> {
    let mut diaeff = 0.0;
    let mut spgeff = 0.0;
    let mut sumf = 0.0;
    for class in classes.iter().take(3) {
        diaeff += class.frac * class.dia_m.ln();
        spgeff += class.frac * class.spg.ln();
        sumf += class.frac;
    }
    if sumf <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.effective_particle.sumf",
        });
    }
    let diaeff = (diaeff / sumf).exp();
    let spgeff = (spgeff / sumf).exp();
    validate_finite("erosion.effective_particle.diaeff", diaeff)?;
    validate_finite("erosion.effective_particle.spgeff", spgeff)?;
    Ok((diaeff, spgeff))
}

/// `shield.for`: dimensionless critical shear from the Shields diagram,
/// including the legacy mixed linear/log extrapolation above the table.
#[must_use]
pub fn erosion_shield(reyn: f64) -> f64 {
    const Y: [f64; 8] = [0.0772, 0.0579, 0.04, 0.035, 0.034, 0.045, 0.055, 0.057];
    const R: [f64; 8] = [1.0, 2.0, 4.0, 8.0, 12.0, 100.0, 400.0, 1000.0];
    let ycr = if reyn < R[0] {
        let slope = (Y[1].ln() - Y[0].ln()) / (R[1].ln() - R[0].ln());
        Y[0].ln() - slope * (R[0].ln() - reyn.ln())
    } else if reyn > R[7] {
        let slope = (Y[7].ln() - Y[6].ln()) / (R[7].ln() - R[6].ln());
        Y[7] + slope * (reyn.ln() - R[7].ln())
    } else {
        let mut value = Y[7].ln();
        for i in 1..8 {
            if reyn >= R[i - 1] && reyn <= R[i] {
                let slope = (Y[i].ln() - Y[i - 1].ln()) / (R[i].ln() - R[i - 1].ln());
                value = Y[i - 1].ln() + slope * (reyn.ln() - R[i - 1].ln());
                break;
            }
        }
        value
    };
    ycr.exp()
}

/// `yalin.for`: total sediment transport capacity at a shear stress
/// (kg m^-1 s^-1) with the class-fraction weighting and the sandy-soil
/// adjustment (INV-SED-006 floor lives inside this routine).
pub fn erosion_yalin(
    effsh: f64,
    classes: &[ErosionParticleClass; EROSION_PARTICLE_CLASS_COUNT],
    sand: f64,
) -> Result<f64, DirectRuntimeError> {
    Ok(erosion_yalin_with_class_shares(effsh, classes, sand)?.0)
}

/// `yalin.for` with the per-class transport shares exposed
/// (`tcf1(k) = ws(k)/tottc`, `yalin.for:150-160`). The sandy adjustment
/// scales the total AND redistributes `ws` proportionally
/// (`yalin.for:141-148`), so the shares are the pre-adjustment
/// per-class proportions — matching legacy exactly. E.4: `tcf1` drives
/// the per-class deposition re-proportion (`enrich.for` do-30).
pub fn erosion_yalin_with_class_shares(
    effsh: f64,
    classes: &[ErosionParticleClass; EROSION_PARTICLE_CLASS_COUNT],
    sand: f64,
) -> Result<(f64, [f64; EROSION_PARTICLE_CLASS_COUNT]), DirectRuntimeError> {
    if effsh.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.yalin.effsh",
        });
    }
    let yalcon = 0.635;
    let vstar = (effsh / MSDENS).sqrt();
    let mut t = 0.0;
    let mut deltas = [0.0_f64; EROSION_PARTICLE_CLASS_COUNT];
    let mut p = [0.0_f64; EROSION_PARTICLE_CLASS_COUNT];
    for (index, class) in classes.iter().enumerate() {
        let reyn = vstar * class.dia_m / KINVIS;
        let ycrit = erosion_shield(reyn);
        let delta = vstar * vstar / ((class.spg - 1.0) * ACCGAV * class.dia_m * ycrit) - 1.0;
        if delta > 0.0 {
            let sigma = delta * 2.45 * class.spg.powf(-0.4) * ycrit.sqrt();
            deltas[index] = delta;
            p[index] = yalcon * delta * (1.0 - (1.0 + sigma).ln() / sigma);
            t += delta;
        }
    }
    if t == 0.0 {
        t = YALIN_ZERO_TOTAL;
    }
    let mut ws = [0.0_f64; EROSION_PARTICLE_CLASS_COUNT];
    let mut tottc = 0.0;
    #[allow(clippy::cast_precision_loss)]
    let npart = EROSION_PARTICLE_CLASS_COUNT as f64;
    for (index, class) in classes.iter().enumerate() {
        let coef = vstar * MSDENS * class.dia_m * class.spg;
        ws[index] = p[index] * (deltas[index] / t) * coef * (class.frac * npart);
        tottc += ws[index];
    }
    // Sandy transport adjustment (`yalin.for:141-148`, INV-SED-006):
    // scales the total; `ws` redistributes proportionally, so the class
    // shares are unchanged by it.
    if sand > 0.5 {
        let adjtc = (0.3 + 0.7 * (-12.52 * (sand - 0.5)).exp()).max(YALIN_SANDY_ADJ_FLOOR);
        tottc *= adjtc;
    }
    validate_finite("erosion.yalin.tottc", tottc)?;
    let ws_total: f64 = ws.iter().sum();
    let mut tcf1 = [0.0_f64; EROSION_PARTICLE_CLASS_COUNT];
    if ws_total > 0.0 {
        for (share, value) in tcf1.iter_mut().zip(ws.iter()) {
            *share = value / ws_total;
        }
    }
    Ok((tottc.max(0.0), tcf1))
}

/// `trcoef.for`: transport coefficient `kt = tottc / shear^1.5`, floored.
pub fn erosion_trcoef(
    shear: f64,
    classes: &[ErosionParticleClass; EROSION_PARTICLE_CLASS_COUNT],
    sand: f64,
) -> Result<f64, DirectRuntimeError> {
    if shear.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.trcoef.shear",
        });
    }
    let kt = erosion_yalin(shear, classes, sand)? / shear.powf(1.5);
    Ok(if kt == 0.0 { TRCOEF_FLOOR } else { kt })
}

/// Normalized transport coefficients derived from the two shear stresses
/// (`param.for:215-234`): `kt` (average slope), `kt2` (mean of end and
/// average), `ktrato = kt2/kt`, and `tcend = kt*shrsol^1.5` (floored).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionTransportCoefficients {
    pub kt: f64,
    pub kt2: f64,
    pub ktrato: f64,
    pub tcend_kg_s_m: f64,
}

/// `param.for:215-234`: build the normalized transport coefficients from
/// the average-slope and end-slope shear stresses.
pub fn erosion_transport_coefficients(
    shrsol_pa: f64,
    shrend_pa: f64,
    classes: &[ErosionParticleClass; EROSION_PARTICLE_CLASS_COUNT],
    sand: f64,
) -> Result<ErosionTransportCoefficients, DirectRuntimeError> {
    let kt = erosion_trcoef(shrsol_pa, classes, sand)?;
    let kt2 = erosion_trcoef(0.5 * (shrend_pa + shrsol_pa), classes, sand)?;
    if kt.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.transport.kt",
        });
    }
    let ktrato = kt2 / kt;
    let tcend = (kt * shrsol_pa.powf(1.5)).max(TCEND_FLOOR);
    validate_finite("erosion.transport.ktrato", ktrato)?;
    validate_finite("erosion.transport.tcend", tcend)?;
    Ok(ErosionTransportCoefficients {
        kt,
        kt2,
        ktrato,
        tcend_kg_s_m: tcend,
    })
}

/// Rill hydraulics for a runoff event (`frcfac.for` cropland rill
/// friction + `shears.for`). The Gilley rill width is persistent state
/// grown across a runoff event and capped at the rill spacing; it is
/// passed in and updated so the caller can carry it between events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionRillHydraulics {
    pub width_m: f64,
    pub shrsol_pa: f64,
    pub shrend_pa: f64,
}

/// Cropland rill friction (`frcfac.for:218-236`): `frcsol = 1.11`,
/// `frccov = 4.5*rilcov^1.5544`, `frlive = (canhgt/hmax)*flivmx`,
/// `frctrl = frccov + frlive + frcsol`. Fail-closed: cover inputs must be
/// finite and nonnegative (NaN / negative cover or height is a typed
/// domain error, not a silent zero-cover fallthrough). The legacy zero
/// branches (`rilcov <= 0` → no cover friction; `hmax <= 0` → no live
/// friction) are preserved for the exact-zero case only.
/// `sheart.for`: shear at a given discharge/slope in the CURRENT rill
/// width and friction context, with NO width growth — the inter-OFE
/// boundary shear basis (`param.for:187-189`). Returns the shear floored
/// at the legacy 1e-6 Pa.
pub fn erosion_sheart(
    q_m2_s: f64,
    slope: f64,
    cover: &ErosionRillCoverInputs,
    width_m: f64,
    rspace_m: f64,
) -> Result<f64, DirectRuntimeError> {
    let (frcsol, frctrl) =
        erosion_rill_friction(cover.rilcov, cover.canhgt_m, cover.hmax_m, cover.flivmx)?;
    let (shear_pa, _) = erosion_shears(q_m2_s, slope, width_m, rspace_m, false, frcsol, frctrl)?;
    Ok(shear_pa.max(1.0e-6))
}

fn erosion_rill_friction(
    rilcov: f64,
    canhgt: f64,
    hmax: f64,
    flivmx: f64,
) -> Result<(f64, f64), DirectRuntimeError> {
    validate_finite("erosion.frcfac.rilcov", rilcov)?;
    validate_finite("erosion.frcfac.canhgt", canhgt)?;
    validate_finite("erosion.frcfac.hmax", hmax)?;
    validate_finite("erosion.frcfac.flivmx", flivmx)?;
    if rilcov < 0.0 || canhgt < 0.0 || hmax < 0.0 || flivmx < 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.frcfac.negative_cover",
        });
    }
    let frcsol = 1.11_f64;
    let frccov = if rilcov > 0.0 {
        4.5 * rilcov.powf(1.5544)
    } else {
        0.0
    };
    let frlive = if hmax > 0.0 {
        (canhgt / hmax) * flivmx
    } else {
        0.0
    };
    Ok((frcsol, frccov + frlive + frcsol))
}

/// `shears.for`: rill flow shear via the Chezy uniform-flow depth
/// iteration. `q` is the shear discharge `qshear`, `sslope` the local
/// gradient. Returns the shear stress (Pa) and the (possibly grown) rill
/// width.
fn erosion_shears(
    q: f64,
    sslope: f64,
    mut width_m: f64,
    rspace_m: f64,
    rwflag: bool,
    frcsol: f64,
    frctrl: f64,
) -> Result<(f64, f64), DirectRuntimeError> {
    let q = q.abs();
    let sslope = if sslope <= 0.0 {
        SHEARS_SLOPE_FLOOR
    } else {
        sslope
    };
    // Gilley rill-width growth (`shears.for:83-89`).
    if rwflag {
        let wdthck = 1.13 * q.powf(0.303);
        if width_m < wdthck {
            width_m = wdthck;
        }
    }
    if width_m > rspace_m {
        width_m = rspace_m;
    }
    if width_m.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater)
        || frctrl.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater)
    {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.shears.width_or_friction",
        });
    }
    let chezch = (8.0 * ACCGAV / frctrl).sqrt();
    let depth = if q <= 0.0 {
        0.0
    } else {
        let u = (q / chezch / sslope.sqrt()).powf(2.0 / 3.0) / width_m;
        let mut depth = 0.2 * q.powf(0.36);
        // Bounded fixed-point iteration (legacy loops to the 5e-6 tol; a
        // hard cap prevents a non-converging input from spinning).
        let mut converged = false;
        for _ in 0..1_000 {
            let dz = depth;
            depth = u * (width_m + dz + dz).powf(1.0 / 3.0);
            if (dz / depth - 1.0).abs() <= SHEARS_DEPTH_TOL {
                converged = true;
                break;
            }
        }
        if !converged {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "erosion.shears.depth_iteration",
            });
        }
        depth
    };
    let xsarea = depth * width_m;
    let wp = width_m + 2.0 * depth;
    let hydrad = if wp > SHEARS_WP_FLOOR {
        xsarea / wp
    } else {
        0.0
    };
    // `shears.for:133-134`: shear = wtdens*sin(atan(S))*Rh*frcsol/frctrl.
    let sinang = sslope.atan().sin();
    let shear = WTDENS * sinang * hydrad * frcsol / frctrl;
    validate_finite("erosion.shears.shear", shear)?;
    Ok((shear, width_m))
}

/// Cover surfaces that feed the rill friction factors (`frcfac.for`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionRillCoverInputs {
    pub rilcov: f64,
    pub canhgt_m: f64,
    pub hmax_m: f64,
    pub flivmx: f64,
}

/// Slope gradients for the two shear evaluations (`param.for:167-209`).
/// `cnslp` is the average OFE gradient; `slpend` the actual end gradient.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionShearSlopes {
    pub cnslp: f64,
    pub slpend: f64,
}

/// Compute the rill hydraulics for one runoff event.
///
/// `qshear = qout*rspace` (`xinflo.for:186`); `shrsol` at the average
/// gradient and `shrend` at the end gradient (`param.for:201-209`), both
/// floored at 1e-6. The rill-width growth is applied once at `qshear`
/// (the legacy `shears` is invoked with `qshear` for both evaluations).
pub fn erosion_rill_hydraulics(
    qshear_m2_s: f64,
    slopes: &ErosionShearSlopes,
    cover: &ErosionRillCoverInputs,
    width_seed_m: f64,
    rspace_m: f64,
) -> Result<ErosionRillHydraulics, DirectRuntimeError> {
    validate_finite("erosion.rill.qshear", qshear_m2_s)?;
    validate_finite("erosion.rill.width_seed", width_seed_m)?;
    validate_finite("erosion.rill.cnslp", slopes.cnslp)?;
    validate_finite("erosion.rill.slpend", slopes.slpend)?;
    if width_seed_m < 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.rill.width_seed",
        });
    }
    if rspace_m.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.rill.rspace",
        });
    }
    let (frcsol, frctrl) =
        erosion_rill_friction(cover.rilcov, cover.canhgt_m, cover.hmax_m, cover.flivmx)?;
    // Grow the rill width at the shear discharge (`shears.for` rwflag=1),
    // then evaluate both shears on the grown width.
    let (shrsol_raw, width_m) = erosion_shears(
        qshear_m2_s,
        slopes.cnslp,
        width_seed_m,
        rspace_m,
        true,
        frcsol,
        frctrl,
    )?;
    let (shrend_raw, _) = erosion_shears(
        qshear_m2_s,
        slopes.slpend,
        width_m,
        rspace_m,
        false,
        frcsol,
        frctrl,
    )?;
    // `param.for:202,209`: floor both shear surfaces at 1e-6.
    let shrsol_pa = shrsol_raw.max(0.000_001);
    let shrend_pa = shrend_raw.max(0.000_001);
    Ok(ErosionRillHydraulics {
        width_m,
        shrsol_pa,
        shrend_pa,
    })
}

/// Interrill delivery ratio (`param.for:412-459`). Cropland uses the
/// random-roughness `rif` model with per-class delivery `drinti`;
/// non-cropland delivers all interrill sediment (`intdr = 1`).
pub fn erosion_interrill_delivery_ratio(
    is_cropland: bool,
    rrc_m: f64,
    classes: &[ErosionParticleClass; EROSION_PARTICLE_CLASS_COUNT],
) -> Result<f64, DirectRuntimeError> {
    if !is_cropland {
        return Ok(1.0);
    }
    let mut rif = RIF_SLOPE * rrc_m + RIF_INTERCEPT;
    rif = rif.clamp(0.0, 1.0);
    let mut intdr = 0.0;
    for class in classes {
        let drinti = if class.fall_m_s < 0.01 {
            let bz = 0.1286 + 2209.0 * class.fall_m_s;
            let az = (0.0672 + 659.0 * class.fall_m_s).exp();
            az * rif.powf(bz)
        } else {
            2.5 * rif - 1.5
        };
        let drinti = drinti.clamp(0.0, 1.0);
        intdr += class.frac * drinti;
    }
    validate_finite("erosion.interrill.intdr", intdr)?;
    Ok(intdr)
}

/// `param.for:463-518`: interrill detachment rate
/// `detinr = ki*kiadjf*effint*qi*intdr*rspace/width`, with
/// `qi = runoff/effdrr`.
///
/// Fail-closed domain handling, distinguishing the legacy exact-zero
/// cases from invalid inputs: every argument must be finite (NaN is a
/// typed error), the erodibility/geometry inputs must be nonnegative
/// (negative width, duration, `ki`, `rspace`, delivery, runoff, or
/// intensity is a typed domain violation — not a silent zero). Only an
/// **exact** zero rill width or zero rainfall-excess duration returns
/// `0.0` (the legacy `width.gt.0.0` / `effdrr.gt.0.0` else-branches:
/// no rill area or no excess period physically implies no interrill
/// supply).
#[allow(clippy::too_many_arguments)]
pub fn erosion_detinr(
    ki: f64,
    kiadjf: f64,
    effint_m_s: f64,
    runoff_depth_m: f64,
    effdrr_s: f64,
    intdr: f64,
    rspace_m: f64,
    width_m: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_finite("erosion.detinr.ki", ki)?;
    validate_finite("erosion.detinr.kiadjf", kiadjf)?;
    validate_finite("erosion.detinr.effint_m_s", effint_m_s)?;
    validate_finite("erosion.detinr.runoff_depth_m", runoff_depth_m)?;
    validate_finite("erosion.detinr.effdrr_s", effdrr_s)?;
    validate_finite("erosion.detinr.intdr", intdr)?;
    validate_finite("erosion.detinr.rspace_m", rspace_m)?;
    validate_finite("erosion.detinr.width_m", width_m)?;
    // Negative inputs are invalid domains (fail-closed), distinct from the
    // legacy exact-zero width/duration branches.
    if width_m < 0.0
        || effdrr_s < 0.0
        || ki < 0.0
        || kiadjf < 0.0
        || rspace_m < 0.0
        || intdr < 0.0
        || runoff_depth_m < 0.0
        || effint_m_s < 0.0
    {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.detinr.negative_input",
        });
    }
    // Legacy exact-zero cases: no rill area or no excess period.
    if width_m == 0.0 || effdrr_s == 0.0 {
        return Ok(0.0);
    }
    let qi = runoff_depth_m / effdrr_s;
    let detinr = ki * kiadjf * effint_m_s * qi * intdr * rspace_m / width_m;
    validate_finite("erosion.detinr", detinr)?;
    Ok(detinr)
}
