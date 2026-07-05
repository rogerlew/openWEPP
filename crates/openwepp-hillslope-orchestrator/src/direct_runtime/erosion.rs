use crate::constants::WB11_ZERO_THRESHOLD;

use super::{
    DIRECT_AUDIT, DIRECT_R7D6_EROSION_PHASE_SPAN_COUNT, DirectDayFrame,
    DirectErosionConsolidationCarry, DirectErosionInflowIntake, DirectErosionRuntimeCarry,
    DirectPublicationErosionOperands, DirectRuntimeError, DirectWave1ContinuityInputs,
    DirectWave1ContinuityState, DirectWave1DailyState, DirectWave1OperandSeed,
    ErosionExcessInterval, ErosionFrostInputs, ErosionFrostRegime, ErosionParticleClass,
    ErosionRfcumInputs, Wave1InflowOperands, advance_erosion_consolidation,
    assemble_wave1_continuity_inputs, assemble_wave1_continuity_inputs_quantum,
    compute_direct_wave1_continuity, compute_direct_wave1_continuity_quantum,
    resolve_erosion_frost_regime, validate_finite, validate_nonnegative_direct_m,
    wave1_day_routes_sediment,
};

// SC-SED-001 1b-C: one hourly bin (`DC01_HOUR_BIN_COUNT` × 1 h).
const EROSION_HOUR_BIN_S: f64 = 3600.0;

/// ADR-0036: the single-hour interval slice of the WB14 surfaces for one
/// solve quantum — the hour's excess/rainfall depths on the hour's own
/// time base (`reid.for` operand basis, hour-filtered).
fn build_erosion_hour_interval(
    hourly_excess_m: &[f64; 24],
    hourly_rainfall_m: &[f64; 24],
    hour: usize,
) -> Vec<ErosionExcessInterval> {
    let excess_m = hourly_excess_m[hour];
    let rainfall_m = hourly_rainfall_m[hour];
    if excess_m <= 0.0 && rainfall_m <= 0.0 {
        return Vec::new();
    }
    vec![ErosionExcessInterval {
        duration_s: EROSION_HOUR_BIN_S,
        rainfall_intensity_m_s: rainfall_m.max(0.0) / EROSION_HOUR_BIN_S,
        excess_m: excess_m.max(0.0),
        snowmelt_active: false,
    }]
}

// E.1 per-class publication: the seeded `prtcmp` composition must sum to
// unity before it can split the toe concentration. The bound is the
// SC-SED-001 `TOL-SED-005` class-fraction closure tolerance
// (`abs(Σ frac - 1) <= 1e-9`); the split below then normalizes by the
// validated sum so the published class sum equals the scalar toe
// concentration to f64 rounding regardless of the admitted drift.
const WAVE1_CLASS_FRACTION_SUM_TOL: f64 = 1.0e-9;

/// Build the erosion rainfall-excess intervals for one day from the WB14
/// per-hour excess + rainfall surfaces (`reid.for` basis for `effint`/
/// `effdrr`). One 1 h interval per hour that saw excess or rainfall; the
/// `effint` producer selects the excess-period hours itself. Snowmelt
/// exclusion is a recorded follow-up (`snowmelt_active = false` first cut).
fn build_erosion_excess_intervals(
    hourly_excess_m: &[f64; 24],
    hourly_rainfall_m: &[f64; 24],
) -> Vec<ErosionExcessInterval> {
    let mut intervals = Vec::new();
    for hour in 0..24 {
        let excess_m = hourly_excess_m[hour];
        let rainfall_m = hourly_rainfall_m[hour];
        if excess_m <= 0.0 && rainfall_m <= 0.0 {
            continue;
        }
        intervals.push(ErosionExcessInterval {
            duration_s: EROSION_HOUR_BIN_S,
            rainfall_intensity_m_s: rainfall_m / EROSION_HOUR_BIN_S,
            excess_m,
            snowmelt_active: false,
        });
    }
    intervals
}

const DIRECT_EROSION_CLASS_LIMIT: usize = 5;
const DIRECT_EROD13_CONTINUITY_TOLERANCE: f64 = 1.0e-9;
const DIRECT_EROD13_MIN_TCADJF: f64 = 0.30;
const DIRECT_EROD14_CASE_TOLERANCE: f64 = 1.0e-12;
const DIRECT_EROD14_MAX_PHI: f64 = 100_000.0;
const DIRECT_EROD14_PKRO_ZERO_THRESHOLD: f64 = 1.0e-15;
const DIRECT_EROD14_ATTENUATION_FLOOR: f64 = 1.0e-8;
const DIRECT_EROD14_CLASS_FLOOR: f64 = 1.0e-15;
const DIRECT_EROD14_MAX_REPROPORTION_ITERS: usize = 64;
const DIRECT_EROD14_ENRICHMENT_RATIO_OFFSET: f64 = 0.005;
const DIRECT_EROD14_QOSTAR_DELTA_THRESHOLD: f64 = 1.0e-10;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErosionInputs {
    pub wave1_enabled: bool,
    pub wave2_enabled: bool,
    pub wave1: DirectErod13Inputs,
    /// Wave-1 spatial sediment-continuity payload (SC-SED-001, single-OFE
    /// `route`/`erod`/`runge` lineage). `wave1_continuity.enabled` gates the
    /// spatial solve independently of the pointwise `wave1_enabled` check.
    /// Boxed to keep `DirectDayConstructorInputs` inside its R7B type-size
    /// bound; the box is only cloned on days where an erosion wave is
    /// enabled (the pre-r7d8 flag check short-circuits otherwise).
    pub wave1_continuity: Box<DirectWave1ContinuityInputs>,
    /// Per-lane **static** Wave-1 operand seed (SC-SED-001 1b-C): the
    /// texture / geometry / cover-constant operands the per-day assembly
    /// (`assemble_wave1_continuity_inputs`) combines with the daily frame
    /// state to build `wave1_continuity`. Boxed with `wave1_continuity` to
    /// stay inside the R7B type-size bound; `seed.enabled` gates the
    /// assembly (disabled until the production flip).
    pub wave1_operand_seed: Box<DirectWave1OperandSeed>,
    pub wave2: DirectErod14Inputs,
}

impl DirectErosionInputs {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            wave1_enabled: false,
            wave2_enabled: false,
            wave1: DirectErod13Inputs::zero(),
            wave1_continuity: Box::new(DirectWave1ContinuityInputs::zero()),
            wave1_operand_seed: Box::new(DirectWave1OperandSeed::disabled()),
            wave2: DirectErod14Inputs::zero(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectErod13Inputs {
    pub ie_m_s: f64,
    pub te_s: f64,
    pub fs: f64,
    pub ft: f64,
    pub taufe_pa: f64,
    pub q_m2_s: f64,
    pub g_kg_s_m: f64,
    pub di_kg_s_m2: f64,
    pub beta: f64,
    pub vf_m_s: f64,
    pub dgdx_kg_s_m2: f64,
    pub cntlen_m: f64,
    pub kr_s_m: f64,
    pub kradjf: f64,
    pub tcadjf: f64,
    pub shrsol_pa: f64,
    pub tcend_kg_s_m: f64,
    pub shcrit_pa: f64,
    pub detinr_kg_s_m2: f64,
    pub effdrr_m: f64,
    pub effdrn_m: f64,
    pub veleff_m_s: f64,
    pub pkro_m3_s: f64,
    pub tc_k: f64,
    pub tc_m: f64,
    pub q_runoff_m: f64,
    pub peakro_m3_s: f64,
    pub watdur_s: f64,
}

impl DirectErod13Inputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            ie_m_s: 0.0,
            te_s: 0.0,
            fs: 0.0,
            ft: 0.0,
            taufe_pa: 0.0,
            q_m2_s: 0.0,
            g_kg_s_m: 0.0,
            di_kg_s_m2: 0.0,
            beta: 0.0,
            vf_m_s: 0.0,
            dgdx_kg_s_m2: 0.0,
            cntlen_m: 0.0,
            kr_s_m: 0.0,
            kradjf: 0.0,
            tcadjf: 0.0,
            shrsol_pa: 0.0,
            tcend_kg_s_m: 0.0,
            shcrit_pa: 0.0,
            detinr_kg_s_m2: 0.0,
            effdrr_m: 0.0,
            effdrn_m: 0.0,
            veleff_m_s: 0.0,
            pkro_m3_s: 0.0,
            tc_k: 0.0,
            tc_m: 0.0,
            q_runoff_m: 0.0,
            peakro_m3_s: 0.0,
            watdur_s: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectErod13State {
    pub tau_f_pa: f64,
    pub dc_kg_s_m2: f64,
    pub tc_kg_s_m: f64,
    pub df_kg_s_m2: f64,
    pub eta: f64,
    pub taucn: f64,
    pub theta: f64,
    pub phi: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErod14Inputs {
    pub xtop_m: f64,
    pub xbot_m: f64,
    pub xdetst_m: f64,
    pub ldtop_kg_s_m: f64,
    pub ldbot_kg_s_m: f64,
    pub lddend_kg: f64,
    pub qout_m3_s: f64,
    pub qin_m3_s: f64,
    pub qostar_m: f64,
    pub hbp_sediment_concentration_scale: f64,
    pub slplen_m: f64,
    pub ktrato: f64,
    pub aintc: f64,
    pub bintc: f64,
    pub cintc: f64,
    pub beta: f64,
    pub qj_minus_1_m3_s: f64,
    pub vj_m: f64,
    pub qj_m3_s: f64,
    pub fh_m: f64,
    pub fp_m: f64,
    pub case_value: f64,
    pub peak_runoff_m3_s: f64,
    pub runoff_duration_s: f64,
    pub ssa_soil: f64,
    pub theta: f64,
    pub classes: Vec<DirectErod14ClassInputs>,
}

impl DirectErod14Inputs {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            xtop_m: 0.0,
            xbot_m: 0.0,
            xdetst_m: 0.0,
            ldtop_kg_s_m: 0.0,
            ldbot_kg_s_m: 0.0,
            lddend_kg: 0.0,
            qout_m3_s: 0.0,
            qin_m3_s: 0.0,
            qostar_m: 0.0,
            hbp_sediment_concentration_scale: 1.0,
            slplen_m: 0.0,
            ktrato: 0.0,
            aintc: 0.0,
            bintc: 0.0,
            cintc: 0.0,
            beta: 0.0,
            qj_minus_1_m3_s: 0.0,
            vj_m: 0.0,
            qj_m3_s: 0.0,
            fh_m: 0.0,
            fp_m: 0.0,
            case_value: 0.0,
            peak_runoff_m3_s: 0.0,
            runoff_duration_s: 0.0,
            ssa_soil: 0.0,
            theta: 0.0,
            classes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectErod14ClassInputs {
    pub fall_m_s: f64,
    pub frcflw: f64,
    pub frac: f64,
    pub fidel: f64,
    pub tcf1: f64,
    pub ssa_class: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErod14State {
    pub class_count: usize,
    pub sumg_kg: f64,
    pub er: f64,
    pub lddend_kg: f64,
    pub qout_m3_s: f64,
    pub peak_runoff_m3_s: f64,
    pub runoff_duration_s: f64,
    pub classes: Vec<DirectErod14ClassState>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectErod14ClassState {
    pub gend_kg: f64,
    pub sedmax_kg: f64,
    pub particle_flow_fraction: f64,
    pub sed_frac: f64,
    pub sediment_concentration_kg_m3: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErosionState {
    pub wave1: Option<DirectErod13State>,
    /// Boxed for the same R7B day-frame type-size reason as the input
    /// payload; `None` on lanes/days without the Wave-1 spatial solve.
    pub wave1_continuity: Option<Box<DirectWave1ContinuityState>>,
    pub wave2: Option<DirectErod14State>,
    pub publication_authority: bool,
    pub publication: DirectPublicationErosionOperands,
}

impl DirectErosionState {
    #[must_use]
    pub fn inactive() -> Self {
        Self {
            wave1: None,
            wave1_continuity: None,
            wave2: None,
            publication_authority: false,
            publication: DirectPublicationErosionOperands::zero_authority(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErosionDownstreamOperands {
    pub publication_authority: bool,
    pub qout_handoff_authority: bool,
    pub publication: DirectPublicationErosionOperands,
    pub qout_m3_s: f64,
    pub particle_flow_fraction: [f64; DIRECT_EROSION_CLASS_LIMIT],
}

impl DirectErosionDownstreamOperands {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            publication_authority: false,
            qout_handoff_authority: false,
            publication: DirectPublicationErosionOperands::zero_authority(),
            qout_m3_s: 0.0,
            particle_flow_fraction: [0.0; DIRECT_EROSION_CLASS_LIMIT],
        }
    }

    fn from_state(state: &DirectErosionState) -> Self {
        let mut particle_flow_fraction = [0.0; DIRECT_EROSION_CLASS_LIMIT];
        let mut qout_m3_s = 0.0;
        if let Some(wave2) = &state.wave2 {
            qout_m3_s = wave2.qout_m3_s;
            for (index, class_state) in wave2.classes.iter().enumerate() {
                if index < DIRECT_EROSION_CLASS_LIMIT {
                    particle_flow_fraction[index] = class_state.particle_flow_fraction;
                }
            }
        }
        Self {
            publication_authority: state.publication_authority,
            qout_handoff_authority: true,
            publication: state.publication,
            qout_m3_s,
            particle_flow_fraction,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErosionShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub wave1_active: bool,
    pub wave2_active: bool,
    pub publication_authority: bool,
    pub publication: DirectPublicationErosionOperands,
    pub qout_m3_s: f64,
    pub particle_flow_fraction: [f64; DIRECT_EROSION_CLASS_LIMIT],
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErosionSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub erosion_shadow_projection: DirectErosionShadowProjection,
}

impl DirectDayFrame {
    pub fn run_r7d6_erosion_span(&mut self) -> Result<DirectErosionSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R7D6_EROSION_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        // SC-SED-001 1b-C: assemble the per-day Wave-1 continuity inputs
        // (advancing the persistent carry) before the solve. No-op when the
        // seed is disabled.
        self.r7d8_assemble_wave1_continuity_from_frame()?;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let erosion = self.compute_r7d6_erosion()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.erosion = erosion;
        DIRECT_AUDIT.record_direct_state_mutation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.erosion_downstream_operands =
            DirectErosionDownstreamOperands::from_state(&self.erosion);
        DIRECT_AUDIT.record_downstream_operand_production();

        let erosion_shadow_projection = DirectErosionShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            wave1_active: self.erosion.wave1.is_some() || self.erosion.wave1_continuity.is_some(),
            wave2_active: self.erosion.wave2.is_some(),
            publication_authority: self.erosion_downstream_operands.publication_authority,
            publication: self.erosion_downstream_operands.publication,
            qout_m3_s: self.erosion_downstream_operands.qout_m3_s,
            particle_flow_fraction: self.erosion_downstream_operands.particle_flow_fraction,
        };
        self.erosion_shadow_projection = Some(erosion_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectErosionSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            erosion_shadow_projection,
        })
    }

    /// SC-SED-001 1b-C: assemble the per-day Wave-1 continuity inputs from
    /// the static operand seed + the daily frame surfaces + the persistent
    /// carry, advancing the carry. Runs every day when the seed is enabled
    /// (the consolidation age advances daily per `soil.for`, independent of
    /// runoff); the assembly itself gates internally on routing days. No-op
    /// when the seed is disabled.
    /// Gate 2 of the R7D8 assembly: resolve `ifrost` from DIMENSIONLESS
    /// top-layer water (`theta_m / depth_m`) vs field-capacity theta. The
    /// surface soil layer is REQUIRED on an active-erosion day — a missing
    /// one is a real missing upstream, not silently unfrozen — and the RAW
    /// (non-canonicalized) values pass to `resolve_erosion_frost_regime`,
    /// which fail-closes on non-finite / negative rather than letting a
    /// `.max(0.0)` mask them into a plausible unfrozen state.
    fn r7d8_resolve_erosion_frost_regime(
        &self,
        carry: &mut DirectErosionRuntimeCarry,
    ) -> Result<ErosionFrostRegime, DirectRuntimeError> {
        let surface_layer = self.subsurface_compute.layer_state_after.first().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R7D erosion frost-regime surface soil layer",
            },
        )?;
        if surface_layer.depth_m <= 0.0 || !surface_layer.depth_m.is_finite() {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.frost.surface_layer_depth_m",
            });
        }
        let surface_layer_water = surface_layer.theta_m / surface_layer.depth_m;
        let surface_layer_thetfc = surface_layer.field_capacity_theta;
        // An absent frost carry means no frost coupling (non-winter) — that
        // is legitimately unfrozen (0 depths); a present carry passes raw.
        let (frost_depth_m, thaw_depth_m) = self
            .frost_runtime_carry
            .as_ref()
            .map_or((0.0, 0.0), |frost| (frost.dfrost_m, frost.dthaw_m));
        let (frost_regime, new_ifrost) = resolve_erosion_frost_regime(
            &ErosionFrostInputs {
                frost_depth_m,
                thaw_depth_m,
                surface_layer_water,
                surface_layer_thetfc,
            },
            carry.ifrost,
        )?;
        carry.ifrost = new_ifrost;
        Ok(frost_regime)
    }

    /// GAP-SED-THAW: the actively-thawing regime (`soil.for ifrost == 2`)
    /// needs the winter `fcycle` freeze-thaw cycle counter, which the
    /// direct runtime does not produce. Rather than fabricate it
    /// (forbidden) or fail the run, produce NO erosion on a thaw day —
    /// a documented winter-subsystem under-estimate, not fabricated
    /// physics. The continuity stays ENABLED (publication authority
    /// holds) with zero runoff so the solver gates inactive; the
    /// persistent carries stay advanced (age/ifrost lineage faithful).
    /// ADR-0036: the hourly WATER surface still publishes on a thaw
    /// runoff day (the hydrograph exists; only the sediment is skipped) —
    /// otherwise the serialized `Σ V_h = runvol` closure would fail
    /// exactly on GAP-SED-THAW days.
    fn r7d8_thaw_day_skip(&mut self, carry: DirectErosionRuntimeCarry) {
        let mut inert = DirectWave1ContinuityInputs::zero();
        inert.enabled = self.erosion_inputs.wave1_operand_seed.enabled;
        *self.erosion_inputs.wave1_continuity = inert;
        let thaw_saturation_carry_m = self
            .subsurface_compute_shadow_projection
            .as_ref()
            .map_or([0.0; 24], |subsurface| subsurface.hourly_saturation_carry_m);
        self.wave1_hourly_weights = super::runoff::dc01_surface_runoff_hourly_weights(
            self.peak_runoff_shadow_projection
                .as_ref()
                .map_or(0.0, |peak| peak.q_runoff_m),
            &self.wb14_hourly_excess_m,
            &thaw_saturation_carry_m,
        );
        self.wave1_hourly_plan.clear();
        self.erosion_runtime_carry = carry;
    }

    fn r7d8_assemble_wave1_continuity_from_frame(&mut self) -> Result<(), DirectRuntimeError> {
        if !self.erosion_inputs.wave1_operand_seed.enabled {
            return Ok(());
        }

        let mut carry = self.erosion_runtime_carry;

        // Seed the consolidation age from `daydi1` on the first day (the
        // lane carry starts inert); it advances daily thereafter.
        if self.day_index == 0 {
            carry.consolidation = DirectErosionConsolidationCarry::seed(
                self.erosion_inputs.wave1_operand_seed.initial_daydis,
            )?;
        }

        // Gate 1: advance `rfcum`/`daydis` EVERY day (`soil.for` runs daily,
        // independent of runoff). Forest: no irrigation, no tillage.
        carry.consolidation = advance_erosion_consolidation(
            carry.consolidation,
            &ErosionRfcumInputs {
                precipitation_m: self.forcing.precipitation_m,
                irrigation_depth_m: 0.0,
                mean_temperature_c: self.forcing.effective_temperature_c,
                irrigation_is_furrow: false,
                tillage_surface_disturbance: None,
            },
        )?;

        let frost_regime = self.r7d8_resolve_erosion_frost_regime(&mut carry)?;

        // Winter boundary (confirmed hold): the actively-thawing regime
        // (`soil.for ifrost == 2`) needs the winter `fcycle` freeze-thaw
        // cycle counter, which the direct runtime does not produce. Rather
        // than fabricate it (forbidden) or fail the whole run, produce NO
        // erosion on a thaw day — a documented winter-subsystem
        // under-estimate, not fabricated physics. The persistent carries
        // still advanced above (so the age/ifrost lineage stays faithful);
        // only this day's sediment is skipped. Surfacing thaw-day erosion
        // is a winter-subsystem work package (`GAP-SED-THAW`).
        if frost_regime == ErosionFrostRegime::Thawing {
            self.r7d8_thaw_day_skip(carry);
            return Ok(());
        }

        // Gate 3: build `DirectWave1DailyState` ONLY from verified frame
        // surfaces.
        let peak = self.peak_runoff_shadow_projection.as_ref();
        let growth = if self.perennial_growth_inputs.active_context.is_active() {
            &self.perennial_growth.state_after
        } else {
            &self.annual_growth.state_after
        };
        let canopy_height_m = self
            .evapotranspiration_compute_inputs
            .pmet_compute
            .as_ref()
            .map_or(0.0, |pmet| pmet.canopy_height_m);
        let residue = &self.residue_partition;
        let seed = &self.erosion_inputs.wave1_operand_seed;
        let precipitation_m = self.forcing.precipitation_m;

        let daily = DirectWave1DailyState {
            peakro_m_s: peak.map_or(0.0, |p| p.peak_runoff_m3_s),
            runoff_depth_m: peak.map_or(0.0, |p| p.q_runoff_m),
            effdrn_s: peak.map_or(0.0, |p| p.runoff_duration_s),
            qin_m2_s: 0.0,
            inflow: None,
            excess_intervals: build_erosion_excess_intervals(
                &self.wb14_hourly_excess_m,
                &self.wb14_hourly_rainfall_m,
            ),
            canopy_cover_fraction: growth.canopy_cover_fraction,
            canopy_height_m,
            interrill_cover_fraction: residue.cover_fraction,
            rill_cover_fraction: residue.cover_fraction,
            live_root_mass_kg_m2: growth.root_mass_kg_m2,
            dead_root_mass_kg_m2: residue.root_residue_kg_m2,
            buried_residue_mass_kg_m2: residue.buried_residue_kg_m2,
            random_roughness_m: seed.random_roughness_m,
            rill_width_prior_m: carry.rill_width_m,
            days_since_disturbance: carry.consolidation.daydis,
            frost_regime,
            // Snow-cover interrill suppression is a recorded follow-up.
            theta_suppressed: false,
            beta: if precipitation_m > 0.0 { 0.5 } else { 1.0 },
            strldn: 0.0,
        };

        let continuity = assemble_wave1_continuity_inputs(seed, &daily)?;

        // ADR-0036 / INV-SED-013: the hydrograph-resolved hourly plan. The
        // weights (the shared DC01 shape authority) are a WATER surface —
        // populated on every runoff day, sub-`passby` included, because the
        // serialized `V_h` hydrograph exists independently of whether the
        // day routes sediment. The PLAN is sediment-gated: the day-level
        // `passby` event gate runs on the DAY totals, and active days then
        // assemble one solve quantum per hydraulically-active hour
        // (`w_h > 0`; production `qin_h = 0` until the E.3 handoff).
        let saturation_carry_m = self
            .subsurface_compute_shadow_projection
            .as_ref()
            .map_or([0.0; 24], |subsurface| subsurface.hourly_saturation_carry_m);
        let weights = super::runoff::dc01_surface_runoff_hourly_weights(
            daily.runoff_depth_m,
            &self.wb14_hourly_excess_m,
            &saturation_carry_m,
        );
        self.wave1_hourly_weights = weights;
        let hourly_width_m = Self::build_wave1_hourly_plan(
            seed,
            &daily,
            &self.wb14_hourly_excess_m,
            &self.wb14_hourly_rainfall_m,
            &weights,
            carry.rill_width_m,
            self.erosion_inflow_intake.as_deref(),
            &mut self.wave1_hourly_plan,
        )?;
        carry.rill_width_m = if self.wave1_hourly_plan.is_empty() {
            // Non-routed day: the inert daily payload carries the prior
            // width unchanged (same value either way).
            continuity.width_m
        } else {
            hourly_width_m
        };
        // INV-SED-015: on a production lane the hourly plan is the ONLY
        // publication authority. A routed day whose every hour fell below
        // the negligibility floor must not fall through to the daily
        // peak-form solve — degrade it to the inert (zero-sediment) day.
        if self.wave1_hourly_plan.is_empty()
            && wave1_day_routes_sediment(daily.runoff_depth_m, daily.peakro_m_s)
        {
            let mut inert = DirectWave1ContinuityInputs::zero();
            inert.enabled = seed.enabled;
            *self.erosion_inputs.wave1_continuity = inert;
        } else {
            *self.erosion_inputs.wave1_continuity = continuity;
        }
        self.erosion_runtime_carry = carry;
        Ok(())
    }

    fn compute_r7d6_erosion(&self) -> Result<DirectErosionState, DirectRuntimeError> {
        // Check the enable flags before r7d8 clones the inputs (the Wave-2
        // class Vec makes that clone a per-OFE-day allocation).
        if !self.erosion_inputs.wave1_enabled
            && !self.erosion_inputs.wave2_enabled
            && !self.erosion_inputs.wave1_continuity.enabled
        {
            return Ok(DirectErosionState::inactive());
        }
        let erosion_inputs = self.r7d8_erosion_inputs_with_runoff_authority()?;

        let wave1 = if erosion_inputs.wave1_enabled {
            Some(compute_direct_erod13(&erosion_inputs.wave1)?)
        } else {
            None
        };
        // ADR-0036 D1: the hydrograph-resolved plan is the production solve
        // when present (a routed day always builds a non-empty plan, so the
        // daily-payload solve below stays reachable only for crafted/test
        // payloads and the INV-SED-015 comparator arm — never as production
        // publication authority on a routed day).
        let mut wave1_hourly_sediment_kg: Option<[f64; 24]> = None;
        let wave1_continuity = if erosion_inputs.wave1_continuity.enabled {
            if self.wave1_hourly_plan.is_empty() {
                let state = compute_direct_wave1_continuity(&erosion_inputs.wave1_continuity)?;
                if erosion_inputs.wave1_operand_seed.enabled {
                    // Enabled production lane, non-routed day: the hourly
                    // sediment surface is authoritatively zero.
                    wave1_hourly_sediment_kg = Some([0.0; 24]);
                }
                Some(Box::new(state))
            } else {
                let (aggregate, hourly_sediment) = self.solve_wave1_hourly_plan(
                    &erosion_inputs.wave1_continuity,
                    erosion_inputs.wave1_operand_seed.field_width_m,
                )?;
                wave1_hourly_sediment_kg = Some(hourly_sediment);
                Some(Box::new(aggregate))
            }
        } else {
            None
        };
        let wave2 = if erosion_inputs.wave2_enabled {
            let theta = wave1
                .as_ref()
                .map_or(erosion_inputs.wave2.theta, |state| state.theta);
            Some(compute_direct_erod14(&erosion_inputs.wave2, theta)?)
        } else {
            None
        };
        let mut publication = if let Some(wave2) = &wave2 {
            direct_erod15_publication_projection(wave2, &erosion_inputs.wave2)?
        } else if let Some(continuity) = wave1_continuity.as_deref().filter(|state| state.active) {
            direct_wave1_publication_projection(
                continuity,
                &erosion_inputs.wave1_continuity,
                &erosion_inputs.wave1_operand_seed.classes,
            )?
        } else {
            DirectPublicationErosionOperands::zero_authority()
        };
        // ADR-0036 D2: the hourly surfaces ride the Wave-1 publication —
        // the runoff fraction is the shared water shape (populated on every
        // enabled-lane day, sub-passby and thaw days included), the
        // sediment surface comes from the plan solve (all-zero when no
        // sediment routed). Wave-2 multi-OFE lanes stay `None` (minor-0).
        if erosion_inputs.wave1_operand_seed.enabled && !erosion_inputs.wave2_enabled {
            publication.hourly_runoff_fraction = Some(self.wave1_hourly_weights);
            publication.hourly_sediment_mass_kg = wave1_hourly_sediment_kg.or(Some([0.0; 24]));
        }

        Ok(DirectErosionState {
            wave1,
            wave1_continuity,
            wave2,
            publication_authority: erosion_inputs.wave2_enabled
                || erosion_inputs.wave1_continuity.enabled,
            publication,
        })
    }

    /// ADR-0036 / INV-SED-013: build the per-hydraulically-active-hour
    /// solve plan on the shared shape authority. The hourly chain owns the
    /// persistent rill width: hour `h` seeds hour `h+1`; the end-of-chain
    /// width is returned for the carry (the daily-assembled payload keeps
    /// the legacy day-basis width for the comparator arm only).
    #[allow(clippy::too_many_arguments)]
    fn build_wave1_hourly_plan(
        seed: &DirectWave1OperandSeed,
        daily: &DirectWave1DailyState,
        hourly_excess_m: &[f64; 24],
        hourly_rainfall_m: &[f64; 24],
        weights: &[f64; 24],
        start_width_m: f64,
        intake: Option<&DirectErosionInflowIntake>,
        plan: &mut Vec<(usize, DirectWave1ContinuityInputs)>,
    ) -> Result<f64, DirectRuntimeError> {
        plan.clear();
        let mut hourly_width_m = start_width_m;
        // Day gate: the local passby event gate, OR any upstream inflow
        // hour (INV-SED-013 — a locally-dry lane with routed upstream
        // sediment must still solve to deposit it).
        let intake_active =
            intake.is_some_and(|intake| intake.hourly_qout_m2_s.iter().any(|qout| *qout > 0.0));
        if !wave1_day_routes_sediment(daily.runoff_depth_m, daily.peakro_m_s) && !intake_active {
            return Ok(hourly_width_m);
        }
        for (hour, weight) in weights.iter().enumerate() {
            // Plan inclusion: positive-weight hours above the runtime's own
            // negligible-runoff bound (the WB16 "too small to produce a
            // peak" class — a trace-weight hour cannot route sediment but
            // CAN underflow the routed-operand domain), OR hours with
            // positive upstream inflow (the full-reinfiltration quantum).
            let hour_qin_m2_s = intake.map_or(0.0, |intake| intake.hourly_qout_m2_s[hour]);
            let local_active = *weight > 0.0
                && daily.runoff_depth_m * weight
                    > crate::constants::WB16_RUNOFF_NEAR_ZERO_THRESHOLD;
            if !local_active && hour_qin_m2_s <= 0.0 {
                continue;
            }
            let mut hour_state = daily.clone();
            hour_state.peakro_m_s = if local_active {
                daily.runoff_depth_m * weight / EROSION_HOUR_BIN_S
            } else {
                0.0
            };
            hour_state.runoff_depth_m = if local_active {
                daily.runoff_depth_m * weight
            } else {
                0.0
            };
            hour_state.effdrn_s = EROSION_HOUR_BIN_S;
            hour_state.excess_intervals =
                build_erosion_hour_interval(hourly_excess_m, hourly_rainfall_m, hour);
            hour_state.beta = if hourly_rainfall_m[hour] > 0.0 {
                0.5
            } else {
                1.0
            };
            hour_state.rill_width_prior_m = hourly_width_m;
            hour_state.inflow = intake.and_then(|intake| {
                if hour_qin_m2_s > 0.0 {
                    Some(Wave1InflowOperands {
                        qin_m2_s: hour_qin_m2_s,
                        qsout_kg_m_s: intake.hourly_qsout_kg_m_s[hour],
                        prior_slpend: intake.prior_slpend,
                        prior_cnslp: intake.prior_cnslp,
                        prior_end_shear: intake.prior_end_shear,
                        prior_end_transport: intake.prior_end_transport,
                    })
                } else {
                    None
                }
            });
            let hour_inputs = assemble_wave1_continuity_inputs_quantum(seed, &hour_state, true)?;
            hourly_width_m = hour_inputs.width_m;
            plan.push((hour, hour_inputs));
        }
        Ok(hourly_width_m)
    }

    /// ADR-0036 D1: solve every hydraulically-active hour quantum
    /// (passby-exempt — the day-level event gate already ran) and fold the
    /// results into the day-aggregate continuity state (`INV-SED-014`:
    /// daily totals are the hour sums). The per-point diagnostic grids on
    /// the aggregate come from the max-export hour (a representative
    /// profile; the totals are sums, never the representative's own).
    fn solve_wave1_hourly_plan(
        &self,
        day_inputs: &DirectWave1ContinuityInputs,
        field_width_m: f64,
    ) -> Result<(DirectWave1ContinuityState, [f64; 24]), DirectRuntimeError> {
        let mut hourly_sediment_kg = [0.0_f64; 24];
        let mut aggregate: Option<DirectWave1ContinuityState> = None;
        let mut exported_kg_m_sum = 0.0_f64;
        let mut inflow_kg_m_sum = 0.0_f64;
        let mut detach_kg_sum = 0.0_f64;
        let mut depos_kg_sum = 0.0_f64;
        let mut interrill_kg_m2_sum = 0.0_f64;
        let mut closure_residual_sum = 0.0_f64;
        let mut flux_residual_sum = 0.0_f64;
        let mut flux_scale_sum = 0.0_f64;
        let mut max_export_kg_m = -1.0_f64;

        for (hour, hour_inputs) in &self.wave1_hourly_plan {
            let state = compute_direct_wave1_continuity_quantum(hour_inputs, true)?;
            if !state.active {
                // A planned quantum is active by construction (`w_h > 0`
                // implies positive hour flow); an inactive return means the
                // plan and the solver disagree — fail closed.
                return Err(DirectRuntimeError::DirectDomainViolation {
                    field: "erosion.wave1.hourly_plan_inactive_quantum",
                });
            }
            hourly_sediment_kg[*hour] = state.exported_sediment_kg_m * field_width_m;
            exported_kg_m_sum += state.exported_sediment_kg_m;
            inflow_kg_m_sum += state.inflow_sediment_kg_m;
            detach_kg_sum += state.total_detachment_kg;
            depos_kg_sum += state.total_deposition_kg;
            interrill_kg_m2_sum += state.interrill_contribution_kg_m2;
            closure_residual_sum += state.publication_closure_residual_kg_m;
            flux_residual_sum += state.flux_closure_residual;
            flux_scale_sum += state.flux_closure_scale;
            if state.exported_sediment_kg_m > max_export_kg_m {
                max_export_kg_m = state.exported_sediment_kg_m;
                aggregate = Some(state);
            }
        }

        let mut aggregate = aggregate.ok_or(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.hourly_plan_empty",
        })?;
        aggregate.exported_sediment_kg_m = exported_kg_m_sum;
        aggregate.inflow_sediment_kg_m = inflow_kg_m_sum;
        aggregate.total_detachment_kg = detach_kg_sum;
        aggregate.total_deposition_kg = depos_kg_sum;
        aggregate.interrill_contribution_kg_m2 = interrill_kg_m2_sum;
        aggregate.publication_closure_residual_kg_m = closure_residual_sum;
        aggregate.flux_closure_residual = flux_residual_sum;
        aggregate.flux_closure_scale = flux_scale_sum;
        // The day toe concentration re-forms on the DAY totals
        // (`sloss.for:314` basis, preserving the E.1 reconstruction
        // identity `tdet = Σ sedcon × runvol` on zero-deposition days).
        aggregate.sediment_concentration_kg_m3 = if day_inputs.peakro_m_s <= 0.0 {
            0.0
        } else {
            exported_kg_m_sum / (day_inputs.runoff_depth_m * day_inputs.efflen_m)
        };
        validate_finite(
            "erosion.wave1.hourly_day_concentration",
            aggregate.sediment_concentration_kg_m3,
        )?;
        Ok((aggregate, hourly_sediment_kg))
    }

    fn r7d8_erosion_inputs_with_runoff_authority(
        &self,
    ) -> Result<DirectErosionInputs, DirectRuntimeError> {
        let mut inputs = self.erosion_inputs.clone();
        if !inputs.wave1_enabled && !inputs.wave2_enabled && !inputs.wave1_continuity.enabled {
            return Ok(inputs);
        }
        let peak_runoff = self.peak_runoff_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R7D6 WB16 peak-duration producer",
            },
        )?;
        if inputs.wave1_enabled {
            inputs.wave1.q_runoff_m = peak_runoff.q_runoff_m;
            inputs.wave1.peakro_m3_s = peak_runoff.peak_runoff_m3_s;
            inputs.wave1.watdur_s = peak_runoff.runoff_duration_s;
        }
        // SC-SED-001 1b-C: `wave1_continuity` is now fully populated by the
        // per-day assembly (`r7d8_assemble_wave1_continuity_from_frame`,
        // run before this in the erosion span), which sources the runoff
        // authority itself. The Increment-1 runoff-only threading stopgap
        // is removed.
        if inputs.wave2_enabled {
            inputs.wave2.peak_runoff_m3_s = peak_runoff.peak_runoff_m3_s;
            inputs.wave2.runoff_duration_s = peak_runoff.runoff_duration_s;
            let runoff = self.runoff_shadow_projection.as_ref().ok_or(
                DirectRuntimeError::MissingDirectUpstream {
                    upstream: "R7D8 direct runoff producer for EROD14 qout",
                },
            )?;
            if runoff.q_runoff_m > WB11_ZERO_THRESHOLD {
                inputs.wave2.qout_m3_s = runoff.q_runoff_m;
            }
            inputs.wave2.qin_m3_s = if self.lane_index == 0 {
                0.0
            } else if self
                .upstream_erosion_downstream_operands
                .qout_handoff_authority
            {
                self.upstream_erosion_downstream_operands.qout_m3_s
            } else {
                return Err(DirectRuntimeError::MissingDirectUpstream {
                    upstream: "R7D8 prior-lane erosion qout for EROD14 qin",
                });
            };
            // DC01 (INV-RUNOFFPART-031 / INV-RUNOFFPART-030 hold): with runon
            // re-infiltration, a downslope OFE can absorb upstream inflow, so
            // qout < qin is physically expected (decreasing-flow case). Full
            // decreasing-flow sediment cases remain in the INV-030 hold; the
            // bounded interim behavior clamps qin to qout and counts the
            // occurrence for the manifest.
            if inputs.wave2.qin_m3_s > inputs.wave2.qout_m3_s + WB11_ZERO_THRESHOLD {
                DIRECT_AUDIT.record_erod14_qin_clamped_event();
                inputs.wave2.qin_m3_s = inputs.wave2.qout_m3_s;
            }
            apply_erod14_prior_particle_flow_fraction(
                &mut inputs.wave2,
                &self.prior_erosion_downstream_operands,
            )?;
            inputs.wave2.qostar_m =
                direct_erod14_qostar(inputs.wave2.qout_m3_s, inputs.wave2.qin_m3_s)?;
        }
        Ok(inputs)
    }
}

fn apply_erod14_prior_particle_flow_fraction(
    inputs: &mut DirectErod14Inputs,
    prior_operands: &DirectErosionDownstreamOperands,
) -> Result<(), DirectRuntimeError> {
    let mut fraction_sum = 0.0;
    for (index, class) in inputs.classes.iter_mut().enumerate() {
        let fraction = if prior_operands.publication_authority {
            prior_operands.particle_flow_fraction[index]
        } else {
            0.0
        };
        validate_fraction("erosion.erod14.prior_particle_flow_fraction", fraction)?;
        class.frcflw = fraction;
        fraction_sum += fraction;
    }
    if prior_operands.publication_authority
        && fraction_sum > WB11_ZERO_THRESHOLD
        && (fraction_sum - 1.0).abs() > DIRECT_EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD
    {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.erod14.prior_particle_flow_fraction_sum",
        });
    }
    Ok(())
}

fn compute_direct_erod13(
    inputs: &DirectErod13Inputs,
) -> Result<DirectErod13State, DirectRuntimeError> {
    validate_erod13_inputs(inputs)?;
    let tau_f_pa = inputs.taufe_pa * (inputs.fs / inputs.ft);
    validate_nonnegative_derived("erosion.erod13.tau_f_pa", tau_f_pa)?;
    let eta =
        (inputs.cntlen_m * inputs.kr_s_m * inputs.kradjf * inputs.shrsol_pa) / inputs.tcend_kg_s_m;
    validate_nonnegative_derived("erosion.erod13.eta", eta)?;
    let taucn = (inputs.tcadjf * inputs.shcrit_pa) / inputs.shrsol_pa;
    validate_nonnegative_derived("erosion.erod13.taucn", taucn)?;
    let theta = ((inputs.cntlen_m * inputs.detinr_kg_s_m2) / inputs.tcend_kg_s_m)
        * (inputs.effdrr_m / inputs.effdrn_m);
    validate_nonnegative_derived("erosion.erod13.theta", theta)?;
    let phi = (inputs.beta * inputs.veleff_m_s) / inputs.pkro_m3_s;
    validate_nonnegative_derived("erosion.erod13.phi", phi)?;
    let tc_kg_s_m = inputs.tcadjf * inputs.tc_k * tau_f_pa.powf(inputs.tc_m);
    validate_nonnegative_derived("erosion.erod13.tc_kg_s_m", tc_kg_s_m)?;

    let (detachment_capacity_kg_s_m2, net_detachment_flux_kg_s_m2) =
        direct_erod13_fluxes(inputs, tau_f_pa, taucn, eta, tc_kg_s_m)?;
    let expected_dgdx = net_detachment_flux_kg_s_m2 + inputs.di_kg_s_m2;
    if (inputs.dgdx_kg_s_m2 - expected_dgdx).abs()
        > DIRECT_EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD
    {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.erod13.dgdx",
        });
    }

    Ok(DirectErod13State {
        tau_f_pa,
        dc_kg_s_m2: detachment_capacity_kg_s_m2,
        tc_kg_s_m,
        df_kg_s_m2: net_detachment_flux_kg_s_m2,
        eta,
        taucn,
        theta,
        phi,
    })
}

fn validate_erod13_inputs(inputs: &DirectErod13Inputs) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("erosion.erod13.ie_m_s", inputs.ie_m_s)?;
    validate_min("erosion.erod13.te_s", inputs.te_s, WB11_ZERO_THRESHOLD)?;
    validate_nonnegative_direct_m("erosion.erod13.fs", inputs.fs)?;
    validate_min("erosion.erod13.ft", inputs.ft, WB11_ZERO_THRESHOLD)?;
    validate_max("erosion.erod13.fs", inputs.fs, inputs.ft)?;
    validate_nonnegative_direct_m("erosion.erod13.taufe_pa", inputs.taufe_pa)?;
    validate_nonnegative_direct_m("erosion.erod13.q_m2_s", inputs.q_m2_s)?;
    validate_nonnegative_direct_m("erosion.erod13.g_kg_s_m", inputs.g_kg_s_m)?;
    validate_nonnegative_direct_m("erosion.erod13.di_kg_s_m2", inputs.di_kg_s_m2)?;
    validate_nonnegative_direct_m("erosion.erod13.beta", inputs.beta)?;
    validate_nonnegative_direct_m("erosion.erod13.vf_m_s", inputs.vf_m_s)?;
    validate_finite("erosion.erod13.dgdx_kg_s_m2", inputs.dgdx_kg_s_m2)?;
    validate_min(
        "erosion.erod13.cntlen_m",
        inputs.cntlen_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min("erosion.erod13.kr_s_m", inputs.kr_s_m, WB11_ZERO_THRESHOLD)?;
    validate_min("erosion.erod13.kradjf", inputs.kradjf, WB11_ZERO_THRESHOLD)?;
    validate_min(
        "erosion.erod13.tcadjf",
        inputs.tcadjf,
        DIRECT_EROD13_MIN_TCADJF,
    )?;
    validate_min(
        "erosion.erod13.shrsol_pa",
        inputs.shrsol_pa,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min(
        "erosion.erod13.tcend_kg_s_m",
        inputs.tcend_kg_s_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_nonnegative_direct_m("erosion.erod13.shcrit_pa", inputs.shcrit_pa)?;
    validate_nonnegative_direct_m("erosion.erod13.detinr_kg_s_m2", inputs.detinr_kg_s_m2)?;
    validate_min(
        "erosion.erod13.effdrr_m",
        inputs.effdrr_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min(
        "erosion.erod13.effdrn_m",
        inputs.effdrn_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_nonnegative_direct_m("erosion.erod13.veleff_m_s", inputs.veleff_m_s)?;
    validate_min(
        "erosion.erod13.pkro_m3_s",
        inputs.pkro_m3_s,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min("erosion.erod13.tc_k", inputs.tc_k, WB11_ZERO_THRESHOLD)?;
    validate_min("erosion.erod13.tc_m", inputs.tc_m, WB11_ZERO_THRESHOLD)?;
    validate_min(
        "erosion.erod13.q_runoff_m",
        inputs.q_runoff_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min(
        "erosion.erod13.peakro_m3_s",
        inputs.peakro_m3_s,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min(
        "erosion.erod13.watdur_s",
        inputs.watdur_s,
        WB11_ZERO_THRESHOLD,
    )?;

    let expected_watdur_s = inputs.q_runoff_m / inputs.peakro_m3_s;
    if (inputs.watdur_s - expected_watdur_s).abs()
        > DIRECT_EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD
    {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.erod13.watdur_s",
        });
    }
    Ok(())
}

fn direct_erod13_fluxes(
    inputs: &DirectErod13Inputs,
    tau_f_pa: f64,
    taucn: f64,
    eta: f64,
    tc_kg_s_m: f64,
) -> Result<(f64, f64), DirectRuntimeError> {
    if tau_f_pa > taucn && inputs.g_kg_s_m < tc_kg_s_m {
        validate_min("erosion.erod13.tc_kg_s_m", tc_kg_s_m, WB11_ZERO_THRESHOLD)?;
        let dc = eta * (tau_f_pa - taucn);
        validate_nonnegative_derived("erosion.erod13.dc_kg_s_m2", dc)?;
        let df = dc * ((tc_kg_s_m - inputs.g_kg_s_m) / tc_kg_s_m);
        validate_nonnegative_derived("erosion.erod13.df_kg_s_m2", df)?;
        return Ok((dc, df));
    }
    if inputs.g_kg_s_m > tc_kg_s_m {
        validate_min("erosion.erod13.q_m2_s", inputs.q_m2_s, WB11_ZERO_THRESHOLD)?;
        let df = -((inputs.beta * inputs.vf_m_s / inputs.q_m2_s) * (inputs.g_kg_s_m - tc_kg_s_m));
        if !df.is_finite() || df > WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.erod13.df_kg_s_m2",
            });
        }
        return Ok((0.0, df));
    }
    Ok((0.0, 0.0))
}

#[derive(Debug, Clone)]
struct DirectErod14WorkingClassState {
    fall: Vec<f64>,
    frcflw: Vec<f64>,
    tcf1: Vec<f64>,
    ssa_class: Vec<f64>,
    ftheta: Vec<f64>,
    gu: Vec<f64>,
    gend: Vec<f64>,
    sedmax: Vec<f64>,
    sed_frac: Vec<f64>,
}

fn compute_direct_erod14(
    inputs: &DirectErod14Inputs,
    theta: f64,
) -> Result<DirectErod14State, DirectRuntimeError> {
    validate_erod14_inputs(inputs, theta)?;
    let mut classes = load_direct_erod14_class_state(inputs, theta);

    if inputs.qout_m3_s <= WB11_ZERO_THRESHOLD {
        let class_states = (0..inputs.classes.len())
            .map(|_| DirectErod14ClassState {
                gend_kg: 0.0,
                sedmax_kg: 0.0,
                particle_flow_fraction: 0.0,
                sed_frac: 0.0,
                sediment_concentration_kg_m3: 0.0,
            })
            .collect::<Vec<_>>();
        return Ok(DirectErod14State {
            class_count: inputs.classes.len(),
            sumg_kg: 0.0,
            er: 0.0,
            lddend_kg: inputs.lddend_kg.max(0.0),
            qout_m3_s: inputs.qout_m3_s,
            peak_runoff_m3_s: inputs.peak_runoff_m3_s,
            runoff_duration_s: inputs.runoff_duration_s,
            classes: class_states,
        });
    }

    let pkro = direct_erod14_pkro(inputs)?;
    let mut sumg = direct_erod14_project_initial_gend(inputs, &mut classes, pkro)?;
    direct_erod14_reproportion_to_ldbot(inputs, &mut classes, &mut sumg)?;
    let sumg = direct_erod14_validate_sumg_and_caps(&classes)?;
    direct_erod14_update_transport_fractions(&mut classes, sumg)?;
    let er = direct_erod14_enrichment_ratio(inputs, &classes, sumg)?;
    let class_states = direct_erod14_final_class_states(inputs, &classes)?;

    Ok(DirectErod14State {
        class_count: inputs.classes.len(),
        sumg_kg: sumg.max(0.0),
        er,
        lddend_kg: inputs.lddend_kg.max(0.0),
        qout_m3_s: inputs.qout_m3_s,
        peak_runoff_m3_s: inputs.peak_runoff_m3_s,
        runoff_duration_s: inputs.runoff_duration_s,
        classes: class_states,
    })
}

fn validate_erod14_inputs(
    inputs: &DirectErod14Inputs,
    theta: f64,
) -> Result<(), DirectRuntimeError> {
    if inputs.classes.is_empty() || inputs.classes.len() > DIRECT_EROSION_CLASS_LIMIT {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.erod14.class_count",
        });
    }
    validate_nonnegative_direct_m("erosion.erod14.xtop_m", inputs.xtop_m)?;
    validate_min("erosion.erod14.xbot_m", inputs.xbot_m, inputs.xtop_m)?;
    validate_nonnegative_direct_m("erosion.erod14.xdetst_m", inputs.xdetst_m)?;
    validate_max("erosion.erod14.xdetst_m", inputs.xdetst_m, inputs.xtop_m)?;
    validate_nonnegative_direct_m("erosion.erod14.ldtop_kg_s_m", inputs.ldtop_kg_s_m)?;
    validate_nonnegative_direct_m("erosion.erod14.ldbot_kg_s_m", inputs.ldbot_kg_s_m)?;
    validate_nonnegative_direct_m("erosion.erod14.lddend_kg", inputs.lddend_kg)?;
    validate_max(
        "erosion.erod14.lddend_kg",
        inputs.lddend_kg,
        inputs.ldtop_kg_s_m,
    )?;
    validate_nonnegative_direct_m("erosion.erod14.qout_m3_s", inputs.qout_m3_s)?;
    validate_nonnegative_direct_m("erosion.erod14.qin_m3_s", inputs.qin_m3_s)?;
    validate_min(
        "erosion.erod14.hbp_sediment_concentration_scale",
        inputs.hbp_sediment_concentration_scale,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min(
        "erosion.erod14.slplen_m",
        inputs.slplen_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min("erosion.erod14.ktrato", inputs.ktrato, WB11_ZERO_THRESHOLD)?;
    validate_finite("erosion.erod14.aintc", inputs.aintc)?;
    validate_finite("erosion.erod14.bintc", inputs.bintc)?;
    validate_finite("erosion.erod14.cintc", inputs.cintc)?;
    validate_nonnegative_direct_m("erosion.erod14.beta", inputs.beta)?;
    validate_nonnegative_direct_m("erosion.erod14.qj_minus_1_m3_s", inputs.qj_minus_1_m3_s)?;
    validate_nonnegative_direct_m("erosion.erod14.vj_m", inputs.vj_m)?;
    validate_nonnegative_direct_m("erosion.erod14.qj_m3_s", inputs.qj_m3_s)?;
    validate_nonnegative_direct_m("erosion.erod14.fh_m", inputs.fh_m)?;
    validate_nonnegative_direct_m("erosion.erod14.fp_m", inputs.fp_m)?;
    validate_nonnegative_direct_m("erosion.erod14.peak_runoff_m3_s", inputs.peak_runoff_m3_s)?;
    validate_nonnegative_direct_m("erosion.erod14.runoff_duration_s", inputs.runoff_duration_s)?;
    validate_min(
        "erosion.erod14.ssa_soil",
        inputs.ssa_soil,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_nonnegative_direct_m("erosion.erod14.theta", theta)?;
    validate_erod14_case(inputs)?;
    for class in &inputs.classes {
        validate_nonnegative_direct_m("erosion.erod14.class.fall_m_s", class.fall_m_s)?;
        validate_fraction("erosion.erod14.class.frcflw", class.frcflw)?;
        validate_fraction("erosion.erod14.class.frac", class.frac)?;
        validate_fraction("erosion.erod14.class.fidel", class.fidel)?;
        validate_nonnegative_direct_m("erosion.erod14.class.tcf1", class.tcf1)?;
        validate_min(
            "erosion.erod14.class.ssa_class",
            class.ssa_class,
            WB11_ZERO_THRESHOLD,
        )?;
    }
    Ok(())
}

fn validate_erod14_case(inputs: &DirectErod14Inputs) -> Result<(), DirectRuntimeError> {
    let case_rounded = inputs.case_value.round();
    if (inputs.case_value - case_rounded).abs() > WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.erod14.case_value",
        });
    }
    let case_range = 1.0 - WB11_ZERO_THRESHOLD..=4.0 + WB11_ZERO_THRESHOLD;
    if !case_range.contains(&case_rounded) {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.erod14.case_value",
        });
    }
    let case_number = erod14_case_number(case_rounded)?;
    let qj_minus_1 = inputs.qj_minus_1_m3_s;
    let vj = inputs.vj_m;
    let qj = inputs.qj_m3_s;
    let fh = inputs.fh_m;
    let fp = inputs.fp_m;
    let case_matches = match case_number {
        1 => erod14_case_is_zero(qj_minus_1) && erod14_case_is_zero(vj) && erod14_case_is_zero(qj),
        2 => {
            qj_minus_1 > DIRECT_EROD14_CASE_TOLERANCE
                && vj > DIRECT_EROD14_CASE_TOLERANCE
                && qj > DIRECT_EROD14_CASE_TOLERANCE
        }
        3 => {
            qj_minus_1 > DIRECT_EROD14_CASE_TOLERANCE
                && erod14_case_is_zero(vj)
                && (fh - fp) > DIRECT_EROD14_CASE_TOLERANCE
                && qj > DIRECT_EROD14_CASE_TOLERANCE
        }
        4 => {
            qj_minus_1 > DIRECT_EROD14_CASE_TOLERANCE
                && erod14_case_is_zero(vj)
                && (fh - fp) <= DIRECT_EROD14_CASE_TOLERANCE
                && erod14_case_is_zero(qj)
        }
        _ => false,
    };
    if case_matches {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.erod14.case_value",
        })
    }
}

fn erod14_case_is_zero(value: f64) -> bool {
    value.abs() <= DIRECT_EROD14_CASE_TOLERANCE
}

fn erod14_case_number(case_rounded: f64) -> Result<i32, DirectRuntimeError> {
    for case_number in 1_i32..=4_i32 {
        if (case_rounded - f64::from(case_number)).abs() <= WB11_ZERO_THRESHOLD {
            return Ok(case_number);
        }
    }
    Err(DirectRuntimeError::DirectDomainViolation {
        field: "erosion.erod14.case_value",
    })
}

fn load_direct_erod14_class_state(
    inputs: &DirectErod14Inputs,
    theta: f64,
) -> DirectErod14WorkingClassState {
    let class_count = inputs.classes.len();
    let mut state = DirectErod14WorkingClassState {
        fall: Vec::with_capacity(class_count),
        frcflw: Vec::with_capacity(class_count),
        tcf1: Vec::with_capacity(class_count),
        ssa_class: Vec::with_capacity(class_count),
        ftheta: Vec::with_capacity(class_count),
        gu: Vec::with_capacity(class_count),
        gend: vec![0.0; class_count],
        sedmax: vec![0.0; class_count],
        sed_frac: vec![0.0; class_count],
    };
    for class in &inputs.classes {
        state.fall.push(class.fall_m_s);
        state.frcflw.push(class.frcflw);
        state.tcf1.push(class.tcf1);
        state.ssa_class.push(class.ssa_class);
        state.ftheta.push(class.fidel * theta);
        state.gu.push(class.frcflw * inputs.ldtop_kg_s_m);
    }
    state
}

fn direct_erod14_pkro(inputs: &DirectErod14Inputs) -> Result<f64, DirectRuntimeError> {
    let pkro = (inputs.qout_m3_s - inputs.qin_m3_s) / inputs.slplen_m;
    validate_finite("erosion.erod14.pkro", pkro)?;
    Ok(pkro)
}

fn direct_erod14_qostar(qout: f64, qin: f64) -> Result<f64, DirectRuntimeError> {
    validate_finite("erosion.erod14.qout_for_qostar", qout)?;
    validate_finite("erosion.erod14.qin_for_qostar", qin)?;
    let del = qout - qin;
    let qostar = if qout <= 0.0 {
        0.0
    } else if del.abs() > DIRECT_EROD14_QOSTAR_DELTA_THRESHOLD {
        if qin <= 0.0 { 0.0 } else { qin / del }
    } else if del >= 0.0 {
        qin / DIRECT_EROD14_QOSTAR_DELTA_THRESHOLD
    } else {
        -qin / DIRECT_EROD14_QOSTAR_DELTA_THRESHOLD
    };
    let qostar = if (qostar + 1.0).abs() <= WB11_ZERO_THRESHOLD {
        -1.001
    } else {
        qostar
    };
    validate_finite("erosion.erod14.qostar", qostar)?;
    Ok(qostar)
}

fn direct_erod14_project_initial_gend(
    inputs: &DirectErod14Inputs,
    classes: &mut DirectErod14WorkingClassState,
    pkro: f64,
) -> Result<f64, DirectRuntimeError> {
    let tmpvr2 = inputs.xbot_m + inputs.qostar_m;
    let tmpvr3 = inputs.xtop_m + inputs.qostar_m;
    validate_min(
        "erosion.erod14.qostar_tmpvr2",
        tmpvr2.abs(),
        WB11_ZERO_THRESHOLD,
    )?;
    let mut sumg = 0.0;
    for index in 0..classes.gend.len() {
        let gend = direct_erod14_project_class_gend(inputs, classes, index, pkro, tmpvr2, tmpvr3)?;
        classes.gend[index] = gend;
        sumg += gend;
    }
    validate_finite("erosion.erod14.sumg", sumg)?;
    Ok(sumg)
}

fn direct_erod14_project_class_gend(
    inputs: &DirectErod14Inputs,
    classes: &DirectErod14WorkingClassState,
    index: usize,
    pkro: f64,
    tmpvr2: f64,
    tmpvr3: f64,
) -> Result<f64, DirectRuntimeError> {
    let tmpvr4 = tmpvr2 * tmpvr2;
    let tmpvr5 = tmpvr3 * tmpvr3;
    let tmpvr1 = inputs.ktrato * classes.tcf1[index];
    let aa = tmpvr1 * inputs.aintc;
    let bb = tmpvr1 * inputs.bintc;
    let cc = tmpvr1 * inputs.cintc;
    let phi = direct_erod14_phi(inputs, classes.fall[index], pkro);
    let attenuation_factor = direct_erod14_attenuation_factor(inputs, phi, tmpvr2, tmpvr3)?;
    let denom_coef1 = phi + 2.0;
    let denom_coef2 = phi + 1.0;
    validate_min(
        "erosion.erod14.denominator",
        denom_coef1.abs().min(denom_coef2.abs()),
        WB11_ZERO_THRESHOLD,
    )?;
    let coef1 = phi * aa / denom_coef1;
    let coef2 = (phi * bb + classes.ftheta[index] - 2.0 * aa * phi * inputs.qostar_m) / denom_coef2;
    let term1 = coef1 * tmpvr4;
    let term2 = coef2 * tmpvr2;
    let term3 = aa * inputs.qostar_m * inputs.qostar_m - bb * inputs.qostar_m + cc;
    let attenuation_tail = classes.gu[index] - coef1 * tmpvr5 - coef2 * tmpvr3 - term3;
    let gend = term1 + term2 + term3 + attenuation_factor * attenuation_tail;
    validate_finite("erosion.erod14.class.gend", gend)?;
    Ok(gend.max(0.0))
}

fn direct_erod14_phi(inputs: &DirectErod14Inputs, fall: f64, pkro: f64) -> f64 {
    let phi = if pkro.abs() > DIRECT_EROD14_PKRO_ZERO_THRESHOLD {
        (inputs.beta * fall) / pkro
    } else if inputs.qostar_m >= 0.0 {
        DIRECT_EROD14_MAX_PHI
    } else {
        -DIRECT_EROD14_MAX_PHI
    };
    phi.clamp(-DIRECT_EROD14_MAX_PHI, DIRECT_EROD14_MAX_PHI)
}

fn direct_erod14_attenuation_factor(
    inputs: &DirectErod14Inputs,
    phi: f64,
    tmpvr2: f64,
    tmpvr3: f64,
) -> Result<f64, DirectRuntimeError> {
    let mut ratio = tmpvr3 / tmpvr2;
    if inputs.qostar_m >= 0.0 && ratio > 1.0 {
        ratio = 1.0;
    }
    validate_nonnegative_direct_m("erosion.erod14.attenuation_ratio", ratio)?;
    let attenuation_factor = ratio.powf(phi);
    validate_finite("erosion.erod14.attenuation_factor", attenuation_factor)?;
    Ok(if attenuation_factor < DIRECT_EROD14_ATTENUATION_FLOOR {
        0.0
    } else {
        attenuation_factor
    })
}

fn direct_erod14_reproportion_to_ldbot(
    inputs: &DirectErod14Inputs,
    classes: &mut DirectErod14WorkingClassState,
    sumg: &mut f64,
) -> Result<(), DirectRuntimeError> {
    if *sumg <= 0.0 {
        return Ok(());
    }
    for index in 0..classes.gend.len() {
        classes.gend[index] = classes.gend[index] * inputs.ldbot_kg_s_m / *sumg;
        classes.sedmax[index] =
            classes.gu[index] + classes.ftheta[index] * (inputs.xbot_m - inputs.xtop_m);
        validate_nonnegative_direct_m("erosion.erod14.class.sedmax", classes.sedmax[index])?;
        if classes.gend[index] < DIRECT_EROD14_CLASS_FLOOR {
            classes.gend[index] = DIRECT_EROD14_CLASS_FLOOR;
        }
    }

    let mut converged = false;
    for _ in 0..DIRECT_EROD14_MAX_REPROPORTION_ITERS {
        if direct_erod14_reproportion_iteration(inputs, classes, sumg)? {
            converged = true;
            break;
        }
    }
    if converged {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.erod14.reproportion",
        })
    }
}

fn direct_erod14_reproportion_iteration(
    inputs: &DirectErod14Inputs,
    classes: &mut DirectErod14WorkingClassState,
    sumg: &mut f64,
) -> Result<bool, DirectRuntimeError> {
    let mut ratbot = 0.0;
    *sumg = 0.0;
    let mut adjusted = false;

    for index in 0..classes.gend.len() {
        if classes.gend[index] > classes.sedmax[index] + WB11_ZERO_THRESHOLD {
            classes.gend[index] = classes.sedmax[index];
            adjusted = true;
        } else if classes.gend[index] < classes.sedmax[index] - WB11_ZERO_THRESHOLD {
            ratbot += classes.gend[index];
        }
        *sumg += classes.gend[index];
    }
    validate_finite("erosion.erod14.sumg", *sumg)?;
    if !adjusted {
        return Ok(true);
    }
    if ratbot <= WB11_ZERO_THRESHOLD {
        return Ok(false);
    }

    let gdeficit = inputs.ldbot_kg_s_m - *sumg;
    for index in 0..classes.gend.len() {
        if classes.gend[index] < classes.sedmax[index] - WB11_ZERO_THRESHOLD {
            let updated = classes.gend[index] + gdeficit * classes.gend[index] / ratbot;
            validate_finite("erosion.erod14.class.gend", updated)?;
            classes.gend[index] = updated;
        }
    }
    Ok(false)
}

fn direct_erod14_validate_sumg_and_caps(
    classes: &DirectErod14WorkingClassState,
) -> Result<f64, DirectRuntimeError> {
    let mut sumg = 0.0;
    for index in 0..classes.gend.len() {
        sumg += classes.gend[index];
        validate_finite("erosion.erod14.sumg", sumg)?;
        if classes.gend[index] > classes.sedmax[index] + WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "erosion.erod14.class.sedmax",
            });
        }
    }
    validate_nonnegative_direct_m("erosion.erod14.sumg", sumg)?;
    Ok(sumg)
}

fn direct_erod14_update_transport_fractions(
    classes: &mut DirectErod14WorkingClassState,
    sumg: f64,
) -> Result<(), DirectRuntimeError> {
    if sumg > 0.0 {
        for index in 0..classes.gend.len() {
            classes.frcflw[index] = classes.gend[index] / sumg;
            classes.sed_frac[index] = classes.frcflw[index];
        }
        let sed_frac_sum = classes.sed_frac.iter().copied().sum::<f64>();
        if (sed_frac_sum - 1.0).abs() > DIRECT_EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "erosion.erod14.sed_frac_sum",
            });
        }
    } else {
        for index in 0..classes.gend.len() {
            classes.frcflw[index] = 0.0;
            classes.sed_frac[index] = 0.0;
        }
    }
    Ok(())
}

fn direct_erod14_enrichment_ratio(
    inputs: &DirectErod14Inputs,
    classes: &DirectErod14WorkingClassState,
    sumg: f64,
) -> Result<f64, DirectRuntimeError> {
    let mut sumssa = 0.0;
    for index in 0..classes.sed_frac.len() {
        sumssa += classes.sed_frac[index] * classes.ssa_class[index];
        validate_finite("erosion.erod14.sumssa", sumssa)?;
    }
    let er = if sumg > 0.0 {
        (sumssa / inputs.ssa_soil) + DIRECT_EROD14_ENRICHMENT_RATIO_OFFSET
    } else {
        0.0
    };
    validate_nonnegative_direct_m("erosion.erod14.er", er)?;
    Ok(er)
}

fn direct_erod14_final_class_states(
    inputs: &DirectErod14Inputs,
    classes: &DirectErod14WorkingClassState,
) -> Result<Vec<DirectErod14ClassState>, DirectRuntimeError> {
    let mut states = Vec::with_capacity(classes.gend.len());
    for index in 0..classes.gend.len() {
        let concentration = if inputs.qout_m3_s > WB11_ZERO_THRESHOLD {
            classes.gend[index] / inputs.qout_m3_s
        } else {
            0.0
        };
        validate_nonnegative_direct_m("erosion.erod15.concentration", concentration)?;
        states.push(DirectErod14ClassState {
            gend_kg: classes.gend[index],
            sedmax_kg: classes.sedmax[index],
            particle_flow_fraction: classes.frcflw[index],
            sed_frac: classes.sed_frac[index],
            sediment_concentration_kg_m3: concentration,
        });
    }
    Ok(states)
}

/// Publication projection for the single-OFE Wave-1 continuity solve
/// (INV-SED-010 totals; the scalar HBP concentration carries the
/// total-toe concentration from `sloss.for:314`).
///
/// The per-class array follows the legacy composition
/// `sedcon(i) = conc_total · frcflw(i)` (`sloss.for:305-317`). Pre-
/// enrichment (E.4/2d), the exiting fractions are the `prtcmp` detached
/// composition `frac` — the `route.for:142-160` initialization, which on
/// the enabled scope (single-OFE, zero inflow, non-cropland
/// `fidel = frac`, `param.for:452-458`) is exact whenever the profile
/// does not deposit. On depositing days the class *distribution* is the
/// un-enriched first cut (labeled INV-SED-011 scope limit; the class
/// *sum* — the mass the watershed consumer reconstructs — equals the
/// scalar toe concentration to f64 rounding: the composition is gated at
/// the `TOL-SED-005` closure tolerance and the split is normalized by
/// the validated sum).
pub(crate) fn direct_wave1_publication_projection(
    state: &DirectWave1ContinuityState,
    inputs: &DirectWave1ContinuityInputs,
    classes: &[ErosionParticleClass; DIRECT_EROSION_CLASS_LIMIT],
) -> Result<DirectPublicationErosionOperands, DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "erosion.wave1.publication.total_detachment_kg",
        state.total_detachment_kg,
    )?;
    validate_nonnegative_direct_m(
        "erosion.wave1.publication.total_deposition_kg",
        state.total_deposition_kg,
    )?;
    validate_nonnegative_direct_m(
        "erosion.wave1.publication.sediment_concentration_kg_m3",
        state.sediment_concentration_kg_m3,
    )?;

    // Fail-closed: an unseeded/zeroed class table must error, not publish
    // zeros as if it were a composition (`prtcmp` guarantees Σ frac = 1).
    let mut frac_sum = 0.0;
    for class in classes {
        validate_nonnegative_direct_m("erosion.wave1.publication.class_fraction", class.frac)?;
        frac_sum += class.frac;
    }
    if (frac_sum - 1.0).abs() > WAVE1_CLASS_FRACTION_SUM_TOL {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.publication.class_fraction_sum",
        });
    }

    // Normalize by the TOL-SED-005-validated sum: the published class sum
    // equals the scalar toe concentration to f64 rounding (the division is
    // a <= 1e-9 adjustment inside the closure tolerance, not a correction
    // of an invalid composition — those already failed above).
    let mut sediment_concentration_kg_m3 = [0.0; DIRECT_EROSION_CLASS_LIMIT];
    for (index, class) in classes.iter().enumerate() {
        let concentration = (class.frac / frac_sum) * state.sediment_concentration_kg_m3;
        validate_nonnegative_direct_m(
            "erosion.wave1.publication.class_concentration",
            concentration,
        )?;
        sediment_concentration_kg_m3[index] = concentration;
    }

    Ok(DirectPublicationErosionOperands {
        peak_runoff_m3_s: Some(inputs.peakro_m_s),
        runoff_duration_s: Some(inputs.effdrn_s),
        total_detachment_kg: Some(state.total_detachment_kg),
        total_deposition_kg: Some(state.total_deposition_kg),
        hbp_total_detachment_kg: Some(state.total_detachment_kg),
        hbp_total_deposition_kg: Some(state.total_deposition_kg),
        hbp_sediment_concentration_kg_m3: Some(state.sediment_concentration_kg_m3),
        sediment_concentration_kg_m3: Some(sediment_concentration_kg_m3),
        hourly_runoff_fraction: None,
        hourly_sediment_mass_kg: None,
    })
}

fn direct_erod15_publication_projection(
    wave2: &DirectErod14State,
    inputs: &DirectErod14Inputs,
) -> Result<DirectPublicationErosionOperands, DirectRuntimeError> {
    let mut sediment_concentration_kg_m3 = [0.0; DIRECT_EROSION_CLASS_LIMIT];
    let mut hbp_sediment_concentration_kg_m3 = 0.0;
    for (index, class_state) in wave2.classes.iter().enumerate() {
        if index < DIRECT_EROSION_CLASS_LIMIT {
            sediment_concentration_kg_m3[index] = class_state.sediment_concentration_kg_m3;
        }
        if index == 0 {
            hbp_sediment_concentration_kg_m3 =
                class_state.sediment_concentration_kg_m3 / inputs.hbp_sediment_concentration_scale;
        }
        validate_finite(
            "erosion.erod15.class_concentration",
            class_state.sediment_concentration_kg_m3,
        )?;
    }
    validate_nonnegative_direct_m("erosion.erod15.total_detachment_kg", wave2.sumg_kg)?;
    validate_nonnegative_direct_m("erosion.erod15.total_deposition_kg", wave2.lddend_kg)?;
    validate_nonnegative_direct_m(
        "erosion.erod15.hbp_sediment_concentration_kg_m3",
        hbp_sediment_concentration_kg_m3,
    )?;
    Ok(DirectPublicationErosionOperands {
        peak_runoff_m3_s: Some(wave2.peak_runoff_m3_s),
        runoff_duration_s: Some(wave2.runoff_duration_s),
        total_detachment_kg: Some(wave2.sumg_kg),
        total_deposition_kg: Some(wave2.lddend_kg),
        hbp_total_detachment_kg: Some(wave2.sumg_kg),
        hbp_total_deposition_kg: Some(wave2.lddend_kg),
        hbp_sediment_concentration_kg_m3: Some(hbp_sediment_concentration_kg_m3),
        sediment_concentration_kg_m3: Some(sediment_concentration_kg_m3),
        hourly_runoff_fraction: None,
        hourly_sediment_mass_kg: None,
    })
}

fn validate_nonnegative_derived(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value < -WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::NegativeDirectValue { field });
    }
    Ok(())
}

fn validate_min(field: &'static str, value: f64, minimum: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value < minimum - WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    Ok(())
}

fn validate_max(field: &'static str, value: f64, maximum: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value > maximum + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    Ok(())
}

fn validate_fraction(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(field, value)?;
    validate_max(field, value, 1.0)
}
