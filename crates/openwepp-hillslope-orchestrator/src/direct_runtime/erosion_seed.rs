//! Wave-1 operand assembly (erosion port Increment-1b-C, the flip core).
//!
//! Assembles the per-OFE-day [`DirectWave1ContinuityInputs`] the solver
//! consumes from (a) a per-lane **static** operand seed built once at
//! seed time and (b) the **daily** hydrology / cover / frost state read
//! from the frame. This is the production replacement for the
//! Increment-1 `::zero()` seed and the `erod16` test-harness operand
//! chain: it drives the 1b-A producers (particle/transport/hydraulics/
//! delivery/detinr, effint/effdrr) and the 1b-B adjustment producers
//! (`kiadjf`/`kradjf`/`tcadjf`, thaw fail-closed) with the real runtime
//! surfaces. Source-intent: `contin.for`/`param.for`/`xinflo.for` call
//! order. Fail-closed throughout (no fabricated operands).

use super::{
    DirectRuntimeError, DirectWave1ContinuityInputs, DirectWave1SlopeSegment,
    ErosionAdjustmentInputs, ErosionConsolidationBaselines, ErosionExcessInterval,
    ErosionFrostRegime, ErosionParticleClass, ErosionRillCoverInputs, ErosionShearSlopes,
    erosion_adjustment_factors, erosion_detinr, erosion_effective_intensity,
    erosion_interrill_delivery_ratio, erosion_rill_hydraulics, erosion_transport_coefficients,
    validate_finite, wave1_day_routes_sediment,
};

/// Per-lane **static** erosion operand seed — everything that does not
/// vary day to day. Built once at seed time from the parsed soil / slope /
/// management surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectWave1OperandSeed {
    /// Gate: when false the whole Wave-1 continuity path is inactive
    /// (production seed stays disabled until this is set).
    pub enabled: bool,
    /// Whether the OFE is cropland (`lanuse == 1`) — selects the interrill
    /// delivery branch. Forest burns run non-cropland (`intdr = 1`).
    pub is_cropland: bool,
    /// Normalized slope segments (`profil.for` fit).
    pub segments: Vec<DirectWave1SlopeSegment>,
    /// OFE slope length (m).
    pub slplen_m: f64,
    /// Effective flow-path length (m).
    pub efflen_m: f64,
    /// Contouring length (m); equals `slplen` without contours.
    pub cntlen_m: f64,
    /// Rill spacing (m).
    pub rspace_m: f64,
    /// Hillslope field width (m).
    pub field_width_m: f64,
    /// Average OFE slope gradient (`avgslp`).
    pub avg_slope: f64,
    /// Normalized end-slope gradient basis (`slpend = (a_n + b_n)·avgslp`).
    pub slpend: f64,
    /// Surface-layer sand fraction (for the yalin sandy adjustment).
    pub sand: f64,
    /// Five erosion particle classes + per-class fall velocity.
    pub classes: [ErosionParticleClass; 5],
    /// Effective particle fall velocity (`veleff`).
    pub veleff_m_s: f64,
    /// `scon.for` consolidation baselines.
    pub baselines: ErosionConsolidationBaselines,
    /// Baseline rill erodibility `kr` (s/m).
    pub kr_s_m: f64,
    /// Baseline interrill erodibility `ki`.
    pub ki: f64,
    /// Baseline critical shear `shcrit` (Pa).
    pub shcrit_pa: f64,
    /// Static rill-friction cover constants that do not vary daily
    /// (`hmax`, `flivmx`); the daily `rilcov`/`canhgt` come from the
    /// daily state.
    pub hmax_m: f64,
    pub flivmx: f64,
}

impl DirectWave1OperandSeed {
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            is_cropland: false,
            segments: Vec::new(),
            slplen_m: 0.0,
            efflen_m: 0.0,
            cntlen_m: 0.0,
            rspace_m: 0.0,
            field_width_m: 0.0,
            avg_slope: 0.0,
            slpend: 0.0,
            sand: 0.0,
            classes: [ErosionParticleClass {
                dia_m: 0.0,
                spg: 0.0,
                frac: 0.0,
                fall_m_s: 0.0,
            }; 5],
            veleff_m_s: 0.0,
            baselines: ErosionConsolidationBaselines {
                kicrat: 1.0,
                krcrat: 1.0,
                tccrat: 1.0,
                bconsd: 0.02,
            },
            kr_s_m: 0.0,
            ki: 0.0,
            shcrit_pa: 0.0,
            hmax_m: 0.0,
            flivmx: 0.0,
        }
    }
}

/// The **daily** hydrology / cover / frost state the per-day assembly
/// reads from the frame.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectWave1DailyState {
    /// Peak runoff rate (m/s) — WB16 `peakro`.
    pub peakro_m_s: f64,
    /// Event runoff depth (m) — WB16 `q_runoff`.
    pub runoff_depth_m: f64,
    /// Runoff duration (s) — WB16 `runoff_duration_s` (`effdrn`).
    pub effdrn_s: f64,
    /// Unit inflow discharge from the upstream OFE (m^2/s); 0 for OFE-1.
    pub qin_m2_s: f64,
    /// Rainfall-excess intervals for `effint`/`effdrr` (`reid.for`).
    pub excess_intervals: Vec<ErosionExcessInterval>,
    /// Daily canopy cover fraction.
    pub canopy_cover_fraction: f64,
    /// Daily canopy height (m).
    pub canopy_height_m: f64,
    /// Daily interrill ground cover fraction.
    pub interrill_cover_fraction: f64,
    /// Daily rill cover fraction (`rilcov`) for the friction factor.
    pub rill_cover_fraction: f64,
    /// Daily live-root mass, top 15 cm (`rtm15`).
    pub live_root_mass_kg_m2: f64,
    /// Daily dead-root mass, summed pools (`Σ rtm`).
    pub dead_root_mass_kg_m2: f64,
    /// Daily buried-residue mass, summed pools (`Σ smrm`).
    pub buried_residue_mass_kg_m2: f64,
    /// Daily random roughness (m) (`rrc`).
    pub random_roughness_m: f64,
    /// **Persistent** rill width from the prior storm day (m) — the Gilley
    /// width is state grown by `shears` and reset only at tillage /
    /// disturbance (`shears.for:83-89`; legacy `width(iplane)`). The
    /// assembly grows it at today's shear discharge and returns the grown
    /// value in `DirectWave1ContinuityInputs.width_m` for the caller to
    /// carry forward. 0.0 at the first storm after a disturbance.
    pub rill_width_prior_m: f64,
    /// Days since the last disturbance (`daydis`; runtime accumulator).
    pub days_since_disturbance: f64,
    /// Resolved frost regime for the freeze-thaw factors.
    pub frost_regime: ErosionFrostRegime,
    /// Whether interrill detachment is suppressed today (snow cover /
    /// melt-only): legacy `param.for:530` `theta = 0`.
    pub theta_suppressed: bool,
    /// Rainfall-turbulence factor `beta` (0.5 with rain / 1.0 dry).
    pub beta: f64,
    /// Nondimensional inflow sediment load (`strldn`); 0 for OFE-1.
    pub strldn: f64,
}

/// Assemble the per-OFE-day [`DirectWave1ContinuityInputs`] from the
/// static seed + the daily state. Runs the full operand pipeline:
/// rill hydraulics -> transport coefficients -> effint/effdrr -> interrill
/// delivery -> detinr -> daily erodibility adjustments. Fail-closed: any
/// producer's typed error propagates (no fabricated operands).
///
/// **Activation ordering (legacy `contin.for`):** the routed-operand
/// pipeline (`frcfac`/`shears`/`param`) runs only on days that actually
/// route sediment. On non-routed days (no runoff / below the `passby`
/// gate) the assembly returns early with the activation operands and
/// zeroed routed operands — the solver then gates to inactive without
/// requiring them. This mirrors [`compute_direct_wave1_continuity`]'s own
/// split so the assembly never hard-errors on ordinary dry days (e.g.
/// `peakro = 0` would otherwise fail the zero-width rill-hydraulics guard).
pub fn assemble_wave1_continuity_inputs(
    seed: &DirectWave1OperandSeed,
    daily: &DirectWave1DailyState,
) -> Result<DirectWave1ContinuityInputs, DirectRuntimeError> {
    validate_finite("erosion.assemble.peakro", daily.peakro_m_s)?;
    validate_finite("erosion.assemble.runoff_depth", daily.runoff_depth_m)?;

    // Gate BEFORE computing routed operands (legacy gate-before-`param`).
    // Non-routed days return the inert payload; the solver gates inactive.
    if !seed.enabled || !wave1_day_routes_sediment(daily.runoff_depth_m, daily.peakro_m_s) {
        return Ok(inert_continuity_inputs(seed, daily));
    }

    // Unit outflow discharge (`xinflo.for:150`) and shear discharge
    // (`xinflo.for:186`).
    let qout_m2_s = daily.peakro_m_s * seed.efflen_m;
    let qshear_m2_s = qout_m2_s * seed.rspace_m;

    // Rill hydraulics (frcfac + shears) -> shrsol/shrend + grown width.
    // The width seed is the PERSISTENT prior-storm width (`shears.for`
    // grows it monotonically between disturbances), not 0.0 each day.
    let cover = ErosionRillCoverInputs {
        rilcov: daily.rill_cover_fraction,
        canhgt_m: daily.canopy_height_m,
        hmax_m: seed.hmax_m,
        flivmx: seed.flivmx,
    };
    let slopes = ErosionShearSlopes {
        cnslp: seed.avg_slope,
        slpend: seed.slpend,
    };
    let hydraulics = erosion_rill_hydraulics(
        qshear_m2_s,
        &slopes,
        &cover,
        daily.rill_width_prior_m,
        seed.rspace_m,
    )?;

    // Transport coefficients (shield/yalin/trcoef).
    let transport = erosion_transport_coefficients(
        hydraulics.shrsol_pa,
        hydraulics.shrend_pa,
        &seed.classes,
        seed.sand,
    )?;

    // Effective rainfall intensity / excess duration (reid.for). When
    // there is no excess period both are zero (inert interrill supply).
    let effective = erosion_effective_intensity(&daily.excess_intervals)?;

    // Daily erodibility adjustments (soil.for chain; thaw fail-closed).
    let adjustments = erosion_adjustment_factors(&ErosionAdjustmentInputs {
        canopy_cover_fraction: daily.canopy_cover_fraction,
        canopy_height_m: daily.canopy_height_m,
        interrill_cover_fraction: daily.interrill_cover_fraction,
        live_root_mass_kg_m2: daily.live_root_mass_kg_m2,
        dead_root_mass_kg_m2: daily.dead_root_mass_kg_m2,
        buried_residue_mass_kg_m2: daily.buried_residue_mass_kg_m2,
        days_since_disturbance: daily.days_since_disturbance,
        avg_slope: seed.avg_slope,
        ridge_height_m: 0.0,
        rill_spacing_m: seed.rspace_m,
        random_roughness_m: daily.random_roughness_m,
        baselines: seed.baselines,
        frost_regime: daily.frost_regime,
    })?;

    // Interrill delivery ratio + detinr (param.for). `effint`/`effdrr`
    // and `kiadjf` are the daily-varying interrill drivers.
    let intdr = erosion_interrill_delivery_ratio(
        seed.is_cropland,
        daily.random_roughness_m,
        &seed.classes,
    )?;
    let detinr = erosion_detinr(
        seed.ki,
        adjustments.kiadjf,
        effective.effint_m_s,
        daily.runoff_depth_m,
        effective.effdrr_s,
        intdr,
        seed.rspace_m,
        hydraulics.width_m,
    )?;

    // `param.for:396`: surface frozen to the top zeros rill erodibility
    // via the solver's `surface_frozen` flag.
    let surface_frozen = daily.frost_regime == ErosionFrostRegime::FrozenSurface;

    Ok(DirectWave1ContinuityInputs {
        enabled: seed.enabled,
        segments: seed.segments.clone(),
        peakro_m_s: daily.peakro_m_s,
        runoff_depth_m: daily.runoff_depth_m,
        qin_m2_s: daily.qin_m2_s,
        efflen_m: seed.efflen_m,
        slplen_m: seed.slplen_m,
        cntlen_m: seed.cntlen_m,
        rspace_m: seed.rspace_m,
        width_m: hydraulics.width_m,
        field_width_m: seed.field_width_m,
        effdrn_s: daily.effdrn_s,
        effdrr_s: effective.effdrr_s,
        kr_s_m: seed.kr_s_m,
        kradjf: adjustments.kradjf,
        shcrit_pa: seed.shcrit_pa,
        tcadjf: adjustments.tcadjf,
        detinr_kg_s_m2: detinr,
        shrsol_pa: hydraulics.shrsol_pa,
        tcend_kg_s_m: transport.tcend_kg_s_m,
        ktrato: transport.ktrato,
        veleff_m_s: seed.veleff_m_s,
        beta: daily.beta,
        strldn: daily.strldn,
        surface_frozen,
        theta_suppressed: daily.theta_suppressed,
    })
}

/// The inert-day continuity payload: static geometry + the activation
/// operands (`peakro`/`runoff`) that the solver gates on, with the routed
/// operands zeroed. `compute_direct_wave1_continuity` returns the inactive
/// state from these without inspecting the zeroed routed fields (the
/// gate-before-validation split), so no fabricated operands are required.
fn inert_continuity_inputs(
    seed: &DirectWave1OperandSeed,
    daily: &DirectWave1DailyState,
) -> DirectWave1ContinuityInputs {
    DirectWave1ContinuityInputs {
        enabled: seed.enabled,
        segments: seed.segments.clone(),
        peakro_m_s: daily.peakro_m_s,
        runoff_depth_m: daily.runoff_depth_m,
        qin_m2_s: daily.qin_m2_s,
        efflen_m: seed.efflen_m,
        slplen_m: seed.slplen_m,
        cntlen_m: seed.cntlen_m,
        rspace_m: seed.rspace_m,
        // Carry the prior width unchanged (no growth on a non-routed day).
        width_m: daily.rill_width_prior_m,
        field_width_m: seed.field_width_m,
        effdrn_s: 0.0,
        effdrr_s: 0.0,
        kr_s_m: 0.0,
        kradjf: 0.0,
        shcrit_pa: 0.0,
        tcadjf: 0.0,
        detinr_kg_s_m2: 0.0,
        shrsol_pa: 0.0,
        tcend_kg_s_m: 0.0,
        ktrato: 0.0,
        veleff_m_s: 0.0,
        beta: 0.0,
        strldn: 0.0,
        surface_frozen: false,
        theta_suppressed: false,
    }
}
