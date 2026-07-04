//! Wave-1 daily erodibility adjustment producers (erosion port
//! Increment-1b-B, portable subset).
//!
//! Source-intent port (ADR-0024, baseline `dac3c950`) of the `soil.for`
//! daily erodibility adjustment chain (`kiadjf`/`kradjf`/`tcadjf`) and its
//! `scon.for` consolidation baselines. Governing contract: `SC-SED-001`
//! (INV-SED-007 — consistent adjusted Chapter-7 soil parameters).
//!
//! These are pure producers: they take the resolved daily state
//! (consolidation age, cover / root / residue masses, the frost regime)
//! and return the composite adjustment factors. The **stateful** parts —
//! the `daydis`/`rfcum` accumulators that age the consolidation, and the
//! prior-`ifrost` tracking that resolves the frost regime — are runtime
//! wiring supplied by the caller, not part of these producers.
//!
//! Winter boundary (confirmed hold): the freeze-thaw factors are `1.0`
//! when the surface is unfrozen and `(0, 0, 1)` when the surface is frozen
//! (erosion is zeroed there by the solver's `surface_frozen` path anyway),
//! but the **actively-thawing** regime needs the winter `fcycle`
//! freeze-thaw cycle counter, which is absent from the direct runtime
//! (produced only in the legacy winter subsystem). The thawing regime is
//! therefore **fail-closed** here — a typed error naming the missing
//! producer — so the eventual production enable is safe by construction:
//! correct wherever the thaw branch is inert, a loud typed failure (never
//! fabricated `1.0`) where it is not.

// Legacy symbol continuity: the `kconsd`/`*crat` and
// `ckiaft`/`ckraft`/`ckiasc`/`ckrasc` subfactor names are the pinned
// `soil.for`/`scon.for` symbols (AGENTS.md naming continuity).
#![allow(clippy::similar_names)]

use super::{DirectRuntimeError, validate_finite};

// `scon.for` consolidation-baseline clamps.
const KICRAT_MIN: f64 = 0.1;
const KICRAT_MAX: f64 = 1.0;
const KRCRAT_MIN: f64 = 0.05;
const KRCRAT_MAX: f64 = 1.0;
const TCCRAT_MIN: f64 = 1.0;
const TCCRAT_MAX: f64 = 4.0;
const KCONSD_KI_MIN: f64 = 10_000.0;
const KCONSD_KI_MAX: f64 = 2_000_000.0;
const KCONSD_KR_MIN: f64 = 0.000_01;
const KCONSD_KR_MAX: f64 = 0.004;
const KCONSD_TC_MIN: f64 = 0.3;
const KCONSD_TC_MAX: f64 = 7.0;
// `scon.for:659` consolidation rate.
const BCONSD: f64 = 0.02;
// `soil.for:944` consolidation underflow trap.
const PRODUC_UNDERFLOW: f64 = 10.0;
// `soil.for:1026,1096` adjustment-factor floors; `:1100` tcadjf cap.
const ADJ_FACTOR_FLOOR: f64 = 0.03;
const TCADJF_CAP: f64 = 2.0;
// `soil.for:1023` interrill slope-factor cap (45-degree row sideslope).
const SLOPE_FACTOR_DENOM_CAP: f64 = 0.707;
// `soil.for:1008` steep-slope guard: the pinned baseline literal 0.7854
// rad (45 deg); kept as the source literal, not `FRAC_PI_4`.
#[allow(clippy::approx_constant)]
const SLOPE_FACTOR_ANGLE_CAP: f64 = 0.7854;

/// `scon.for` consolidation baselines: the fully-consolidated erodibility
/// ratios the daily chain relaxes toward. Pure function of surface-layer
/// texture and the (scon-corrected) field capacity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionConsolidationBaselines {
    /// `kicrat` — interrill consolidation ratio (`scon.for:651`).
    pub kicrat: f64,
    /// `krcrat` — rill consolidation ratio (`scon.for:747`).
    pub krcrat: f64,
    /// `tccrat` — critical-shear consolidation ratio (`scon.for` tc block).
    pub tccrat: f64,
    /// `bconsd` — consolidation rate (`scon.for:659`).
    pub bconsd: f64,
}

/// Texture + erodibility inputs for the `scon.for` baselines.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionConsolidationInputs {
    pub sand: f64,
    pub silt: f64,
    pub orgmat: f64,
    /// Scon-corrected surface-layer field capacity (the Profile-FC
    /// lineage — corrected seed, not raw per-layer symbols).
    pub thetfc: f64,
    /// Surface-layer rock fragment fraction (`rfg`).
    pub rock_fragment_fraction: f64,
    /// Baseline interrill erodibility `ki`.
    pub ki: f64,
    /// Baseline rill erodibility `kr`.
    pub kr: f64,
    /// Baseline critical shear `shcrit`.
    pub shcrit: f64,
}

/// `scon.for`: compute the consolidation baselines from texture + `thetfc`.
///
/// Fail-closed at the producer boundary: the texture / field-capacity /
/// rock-fraction inputs are fraction-domain-validated to `[0, 1]` (NaN or
/// out-of-range is a typed error, not silently absorbed by the `scon`
/// clamps); the baseline erodibilities `ki`/`kr`/`shcrit` are validated
/// finite **and** strictly positive (the earlier `<= 0.0` check missed
/// NaN, which would divide to a NaN ratio); and the three output ratios
/// are re-validated finite.
pub fn erosion_consolidation_baselines(
    inputs: &ErosionConsolidationInputs,
) -> Result<ErosionConsolidationBaselines, DirectRuntimeError> {
    validate_fraction_domain("erosion.scon.sand", inputs.sand)?;
    validate_fraction_domain("erosion.scon.silt", inputs.silt)?;
    validate_fraction_domain("erosion.scon.orgmat", inputs.orgmat)?;
    validate_fraction_domain("erosion.scon.thetfc", inputs.thetfc)?;
    validate_fraction_domain("erosion.scon.rfg", inputs.rock_fragment_fraction)?;
    validate_positive_baseline("erosion.scon.ki", inputs.ki)?;
    validate_positive_baseline("erosion.scon.kr", inputs.kr)?;
    validate_positive_baseline("erosion.scon.shcrit", inputs.shcrit)?;

    // Interrill: kconsd = 1000·(3042 − 3166·sand − 8816·orgmat − 2477·fc).
    let kconsd_ki = (1000.0
        * (3042.0 - 3166.0 * inputs.sand - 8816.0 * inputs.orgmat - 2477.0 * inputs.thetfc))
        .clamp(KCONSD_KI_MIN, KCONSD_KI_MAX);
    let kicrat = (kconsd_ki / inputs.ki).clamp(KICRAT_MIN, KICRAT_MAX);

    // Rill: kconsd = 0.00035 − 0.0014·fc + 0.00068·silt + 0.0049·rfg.
    let kconsd_kr = (0.000_35 - 0.0014 * inputs.thetfc
        + 0.000_68 * inputs.silt
        + 0.0049 * inputs.rock_fragment_fraction)
        .clamp(KCONSD_KR_MIN, KCONSD_KR_MAX);
    let krcrat = (kconsd_kr / inputs.kr).clamp(KRCRAT_MIN, KRCRAT_MAX);

    // Critical shear: kconsd = 8.37 − 11.8·fc − 4.9·sand.
    let kconsd_tc =
        (8.37 - 11.8 * inputs.thetfc - 4.9 * inputs.sand).clamp(KCONSD_TC_MIN, KCONSD_TC_MAX);
    let tccrat = (kconsd_tc / inputs.shcrit).clamp(TCCRAT_MIN, TCCRAT_MAX);

    validate_finite("erosion.scon.kicrat", kicrat)?;
    validate_finite("erosion.scon.krcrat", krcrat)?;
    validate_finite("erosion.scon.tccrat", tccrat)?;

    Ok(ErosionConsolidationBaselines {
        kicrat,
        krcrat,
        tccrat,
        bconsd: BCONSD,
    })
}

/// Frost regime for the freeze-thaw adjustment branch (`soil.for:866`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErosionFrostRegime {
    /// Surface unfrozen and at/below field capacity: factors are `1.0`.
    Unfrozen,
    /// Surface frozen (`frdp > 0 && thdp <= 0`): interrill/rill factors
    /// are `0.0`, `tcaft = 1.0` (erosion is zeroed by the solver anyway).
    FrozenSurface,
    /// Actively thawing (`ifrost == 2`): the matric-potential + `fcycle`
    /// branch — **blocked** (winter `fcycle` absent). Fail-closed.
    Thawing,
}

/// The `ifrost` state carried between days (`soil.for` `ifrost(iplane)`).
/// `0` = unfrozen, `1` = frozen surface, `2` = actively thawing. The
/// frost regime for a day is resolved from the frost/thaw depths, the
/// surface-layer water vs field capacity, and the **prior** day's
/// `ifrost` (the thaw regime is only entered from a previously-frozen
/// surface).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErosionIfrostCarry(pub u8);

impl ErosionIfrostCarry {
    /// Day-zero seed (unfrozen).
    #[must_use]
    pub const fn unfrozen() -> Self {
        Self(0)
    }
}

/// Surface-layer state needed to resolve the frost regime
/// (`soil.for:858-872`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionFrostInputs {
    /// Frost depth (m) — `frdp`.
    pub frost_depth_m: f64,
    /// Thaw depth (m) — `thdp`.
    pub thaw_depth_m: f64,
    /// Effective surface-layer water content — legacy `pwater`
    /// (`soil.for:858-865`, frozen-fraction-adjusted).
    pub surface_layer_water: f64,
    /// Surface-layer field capacity — `thetfc(1)`.
    pub surface_layer_thetfc: f64,
}

/// `soil.for:866-872`: resolve the erosion frost regime and the new
/// `ifrost` carry from the surface-layer frost state and the prior
/// `ifrost`. Pure: the caller tracks and threads the carry.
///
/// - `frdp > 0 && thdp <= 0` → `FrozenSurface` (`ifrost = 1`).
/// - else `pwater <= thetfc` → `Unfrozen` (`ifrost = 0`).
/// - else if the prior surface was frozen/thawing (`ifrost > 0`) →
///   `Thawing` (`ifrost = 2`, the winter-`fcycle` branch — fail-closed
///   downstream in [`erosion_adjustment_factors`]).
/// - else → `Unfrozen`.
///
/// Fail-closed: the depths / water / field capacity must be finite
/// (NaN is a typed error, never a silently mis-branched regime) and the
/// prior `ifrost` must be a valid `0..=2` carry.
pub fn resolve_erosion_frost_regime(
    inputs: &ErosionFrostInputs,
    prior_ifrost: ErosionIfrostCarry,
) -> Result<(ErosionFrostRegime, ErosionIfrostCarry), DirectRuntimeError> {
    validate_finite("erosion.frost.frost_depth_m", inputs.frost_depth_m)?;
    validate_finite("erosion.frost.thaw_depth_m", inputs.thaw_depth_m)?;
    validate_finite(
        "erosion.frost.surface_layer_water",
        inputs.surface_layer_water,
    )?;
    validate_finite(
        "erosion.frost.surface_layer_thetfc",
        inputs.surface_layer_thetfc,
    )?;
    if prior_ifrost.0 > 2 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.frost.prior_ifrost",
        });
    }
    let resolved = if inputs.frost_depth_m > 0.0 && inputs.thaw_depth_m <= 0.0 {
        (ErosionFrostRegime::FrozenSurface, ErosionIfrostCarry(1))
    } else if inputs.surface_layer_water <= inputs.surface_layer_thetfc {
        (ErosionFrostRegime::Unfrozen, ErosionIfrostCarry(0))
    } else if prior_ifrost.0 > 0 {
        (ErosionFrostRegime::Thawing, ErosionIfrostCarry(2))
    } else {
        (ErosionFrostRegime::Unfrozen, ErosionIfrostCarry(0))
    };
    Ok(resolved)
}

/// Freeze-thaw adjustment factors `(ckiaft, ckraft, tcaft)`.
fn erosion_freeze_thaw_factors(
    regime: ErosionFrostRegime,
) -> Result<(f64, f64, f64), DirectRuntimeError> {
    match regime {
        ErosionFrostRegime::Unfrozen => Ok((1.0, 1.0, 1.0)),
        ErosionFrostRegime::FrozenSurface => Ok((0.0, 0.0, 1.0)),
        // The thawing branch needs the winter `fcycle` freeze-thaw cycle
        // counter (matric-potential `acyc`), absent from the direct
        // runtime. Fail-closed rather than fabricate `1.0`.
        ErosionFrostRegime::Thawing => Err(DirectRuntimeError::MissingDirectUpstream {
            upstream: "winter fcycle freeze-thaw cycle counter (soil.for ifrost==2 thaw branch)",
        }),
    }
}

// `soil.for:356` consolidation-onset threshold: `daydis` increments only
// once cumulative rain-since-disturbance exceeds 0.01 m.
const RFCUM_CONSOLIDATION_ONSET_M: f64 = 0.01;

/// Persistent consolidation-age carry (`soil.for` `rfcum`/`daydis`): the
/// cumulative rain since the last disturbance and the day count that ages
/// the consolidation. Threaded per-lane across days.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectErosionConsolidationCarry {
    /// Cumulative rain-since-disturbance (m) — `rfcum`.
    pub rfcum_m: f64,
    /// Days since disturbance — `daydis` (feeds `produc = bconsd·daydis`).
    pub daydis: f64,
}

impl DirectErosionConsolidationCarry {
    /// Day-zero seed from the management initial condition (`daydi1`);
    /// `rfcum` starts at 0.
    #[must_use]
    pub fn seed(initial_daydis: f64) -> Self {
        Self {
            rfcum_m: 0.0,
            daydis: initial_daydis.max(0.0),
        }
    }
}

/// Daily inputs for the consolidation-carry advance (`soil.for:833-846`
/// `rfcum` accumulation + `:324`/`:413` tillage reset).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionRfcumInputs {
    /// Daily precipitation depth (m) — temperature-gated into `rfcum`.
    pub precipitation_m: f64,
    /// Daily irrigation depth (m) — `irdept`. 0 for forest.
    pub irrigation_depth_m: f64,
    /// Mean daily temperature (C) — gates precipitation (`tave > 0`).
    pub mean_temperature_c: f64,
    /// Whether irrigation is furrow (`irsyst == 2`): furrow water is
    /// **excluded** from `rfcum` (`soil.for:840`); sprinkler / none
    /// (`irsyst <= 1`) always adds `irdept` regardless of temperature.
    pub irrigation_is_furrow: bool,
    /// `Some(surdis)` on a tillage day: age-scale `daydis` by
    /// `(1 - surdis)` and reset `rfcum`. Forest never tills.
    pub tillage_surface_disturbance: Option<f64>,
}

/// `soil.for`: advance the consolidation carry one day. On a non-tillage
/// day `daydis` increments when the **prior** `rfcum` exceeds the onset
/// threshold (the legacy increment uses `rfcum` before today's rainfall is
/// added, `soil.for:356` before `:833`), then today's liquid input is
/// accumulated: precipitation only when `tave > 0`, and (non-furrow)
/// irrigation always. A tillage day scales `daydis` by `(1 - surdis)` and
/// resets `rfcum` (`soil.for:324`, `:413`).
///
/// Fail-closed: all inputs must be finite and nonnegative (NaN would be
/// silently canonicalized by `.max(0.0)`), `surdis` must be in `[0, 1]`,
/// and the prior carry must be finite.
pub fn advance_erosion_consolidation(
    prior: DirectErosionConsolidationCarry,
    inputs: &ErosionRfcumInputs,
) -> Result<DirectErosionConsolidationCarry, DirectRuntimeError> {
    validate_finite("erosion.rfcum.prior_rfcum", prior.rfcum_m)?;
    validate_finite("erosion.rfcum.prior_daydis", prior.daydis)?;
    validate_finite("erosion.rfcum.precipitation_m", inputs.precipitation_m)?;
    validate_finite(
        "erosion.rfcum.irrigation_depth_m",
        inputs.irrigation_depth_m,
    )?;
    validate_finite(
        "erosion.rfcum.mean_temperature_c",
        inputs.mean_temperature_c,
    )?;
    if inputs.precipitation_m < 0.0
        || inputs.irrigation_depth_m < 0.0
        || prior.rfcum_m < 0.0
        || prior.daydis < 0.0
    {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.rfcum.negative_input",
        });
    }

    // `soil.for:833-845`: precipitation only when warm; irrigation always
    // for sprinkler/none, never for furrow.
    let mut today_input_m = 0.0;
    if inputs.mean_temperature_c > 0.0 {
        today_input_m += inputs.precipitation_m;
    }
    if !inputs.irrigation_is_furrow {
        today_input_m += inputs.irrigation_depth_m;
    }

    if let Some(surdis) = inputs.tillage_surface_disturbance {
        validate_finite("erosion.rfcum.surdis", surdis)?;
        if !(0.0..=1.0).contains(&surdis) {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.rfcum.surdis",
            });
        }
        // Tillage: age-reset (`daydis *= 1 - surdis`) and `rfcum` reset,
        // then today's input accumulates onto the reset.
        return Ok(DirectErosionConsolidationCarry {
            rfcum_m: today_input_m,
            daydis: (1.0 - surdis) * prior.daydis,
        });
    }

    let daydis = if prior.rfcum_m > RFCUM_CONSOLIDATION_ONSET_M {
        prior.daydis + 1.0
    } else {
        prior.daydis
    };
    Ok(DirectErosionConsolidationCarry {
        rfcum_m: prior.rfcum_m + today_input_m,
        daydis,
    })
}

/// Resolved daily inputs for the cropland erodibility adjustment chain
/// (`soil.for:925-1100`). The masses/covers are daily state surfaces; the
/// consolidation age `daydis` and the frost regime are runtime-resolved.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionAdjustmentInputs {
    /// Canopy cover fraction (`cancov`).
    pub canopy_cover_fraction: f64,
    /// Canopy height (m) (`canhgt`).
    pub canopy_height_m: f64,
    /// Interrill ground cover fraction (`inrcov`).
    pub interrill_cover_fraction: f64,
    /// Live root mass in the top 15 cm (`rtm15`).
    pub live_root_mass_kg_m2: f64,
    /// Dead root mass, summed over pools (`Σ rtm`).
    pub dead_root_mass_kg_m2: f64,
    /// Buried (incorporated) residue mass, summed over pools (`Σ smrm`).
    pub buried_residue_mass_kg_m2: f64,
    /// Days since the last soil disturbance (`daydis`; runtime accumulator).
    pub days_since_disturbance: f64,
    /// OFE average slope gradient (rad-equivalent tangent) (`avgslp`).
    pub avg_slope: f64,
    /// Ridge height (m) (`rh`); 0 for forest (no tillage ridges).
    pub ridge_height_m: f64,
    /// Rill spacing (m) (`rspace`).
    pub rill_spacing_m: f64,
    /// Random roughness (m) (`rrc`).
    pub random_roughness_m: f64,
    /// Consolidation baselines from `erosion_consolidation_baselines`.
    pub baselines: ErosionConsolidationBaselines,
    /// Resolved frost regime for the freeze-thaw factors.
    pub frost_regime: ErosionFrostRegime,
}

/// Composite daily erodibility adjustment factors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErosionAdjustmentFactors {
    pub kiadjf: f64,
    pub kradjf: f64,
    pub tcadjf: f64,
}

/// `soil.for:925-1100` (cropland branch): compute `kiadjf`/`kradjf`/
/// `tcadjf` from the resolved daily state and the consolidation baselines.
/// Fail-closed on NaN / negative inputs and on the actively-thawing frost
/// regime (winter `fcycle` block).
pub fn erosion_adjustment_factors(
    inputs: &ErosionAdjustmentInputs,
) -> Result<ErosionAdjustmentFactors, DirectRuntimeError> {
    validate_adjustment_inputs(inputs)?;
    let (ckiaft, ckraft, tcaft) = erosion_freeze_thaw_factors(inputs.frost_regime)?;

    // Consolidation age (`produc = bconsd·daydis`) and the sealing blend.
    let produc = inputs.baselines.bconsd * inputs.days_since_disturbance;
    let (ckiasc, ckrasc, ctcasc) = if produc < PRODUC_UNDERFLOW {
        let decay = (-produc).exp();
        (
            inputs.baselines.kicrat + (1.0 - inputs.baselines.kicrat) * decay,
            inputs.baselines.krcrat + (1.0 - inputs.baselines.krcrat) * decay,
            inputs.baselines.tccrat - (inputs.baselines.tccrat - 1.0) * decay,
        )
    } else {
        (
            inputs.baselines.kicrat,
            inputs.baselines.krcrat,
            inputs.baselines.tccrat,
        )
    };

    // Interrill cover / root subfactors (`soil.for:930-1020`).
    let ckiacc = if inputs.canopy_height_m > 0.0 {
        1.0 - (2.941 * inputs.canopy_cover_fraction / inputs.canopy_height_m)
            * (1.0 - (-0.34 * inputs.canopy_height_m).exp())
    } else {
        1.0 - inputs.canopy_cover_fraction
    };
    let ckiagc = (-2.5 * inputs.interrill_cover_fraction).exp();
    let ckialr = (-0.56 * inputs.live_root_mass_kg_m2).exp();
    let ckiadr = (-0.56 * inputs.dead_root_mass_kg_m2).exp();
    let ckiasa = erosion_interrill_slope_factor(inputs);

    // Rill cover / root subfactors (`soil.for:1030-1096`).
    let ckrbgb = (-0.40 * inputs.buried_residue_mass_kg_m2).exp();
    let ckradr = (-2.2 * inputs.dead_root_mass_kg_m2).exp();
    let ckralr = (-3.5 * inputs.live_root_mass_kg_m2).exp();
    let ctcarr = 1.0 + 8.0 * (inputs.random_roughness_m - 0.006);

    // Composites with the legacy floors / cap.
    let kiadjf =
        (ckiacc * ckiagc * ckialr * ckiadr * ckiasc * ckiaft * ckiasa).max(ADJ_FACTOR_FLOOR);
    let kradjf = (ckrbgb * ckrasc * ckraft * ckradr * ckralr).max(ADJ_FACTOR_FLOOR);
    let tcadjf = (tcaft * ctcasc * ctcarr).min(TCADJF_CAP);

    validate_finite("erosion.adjust.kiadjf", kiadjf)?;
    validate_finite("erosion.adjust.kradjf", kradjf)?;
    validate_finite("erosion.adjust.tcadjf", tcadjf)?;
    Ok(ErosionAdjustmentFactors {
        kiadjf,
        kradjf,
        tcadjf,
    })
}

/// `soil.for:1000-1025`: interrill slope adjustment
/// `ckiasa = 1.05 − 0.85·exp(−4·denom)`, `denom` the effective row/OFE
/// sideslope sine, capped at the 45-degree sideslope (`0.707`).
fn erosion_interrill_slope_factor(inputs: &ErosionAdjustmentInputs) -> f64 {
    let half_spacing = inputs.rill_spacing_m / 2.0;
    let ridge_ratio = if half_spacing > 0.0 {
        inputs.ridge_height_m / half_spacing
    } else {
        0.0
    };
    let mut denom = if ridge_ratio > inputs.avg_slope {
        // Ridge (tilled-row) sideslope dominates.
        inputs.ridge_height_m
            / (half_spacing * half_spacing + inputs.ridge_height_m * inputs.ridge_height_m).sqrt()
    } else if inputs.avg_slope < SLOPE_FACTOR_ANGLE_CAP {
        inputs.avg_slope.sin()
    } else {
        SLOPE_FACTOR_DENOM_CAP
    };
    if denom > SLOPE_FACTOR_DENOM_CAP {
        denom = SLOPE_FACTOR_DENOM_CAP;
    }
    1.05 - 0.85 * (-4.0 * denom).exp()
}

/// Fraction-domain guard for a `[0, 1]` texture / field-capacity /
/// rock-fraction input: finite (NaN → typed error) and within `[0, 1]`.
fn validate_fraction_domain(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    Ok(())
}

/// Baseline-erodibility guard: finite (NaN → typed error) and strictly
/// positive (a zero/negative divisor would poison the consolidation
/// ratio).
fn validate_positive_baseline(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    Ok(())
}

fn validate_adjustment_inputs(inputs: &ErosionAdjustmentInputs) -> Result<(), DirectRuntimeError> {
    for (field, value) in [
        ("canopy_cover_fraction", inputs.canopy_cover_fraction),
        ("canopy_height_m", inputs.canopy_height_m),
        ("interrill_cover_fraction", inputs.interrill_cover_fraction),
        ("live_root_mass_kg_m2", inputs.live_root_mass_kg_m2),
        ("dead_root_mass_kg_m2", inputs.dead_root_mass_kg_m2),
        (
            "buried_residue_mass_kg_m2",
            inputs.buried_residue_mass_kg_m2,
        ),
        ("days_since_disturbance", inputs.days_since_disturbance),
        ("avg_slope", inputs.avg_slope),
        ("ridge_height_m", inputs.ridge_height_m),
        ("rill_spacing_m", inputs.rill_spacing_m),
        ("random_roughness_m", inputs.random_roughness_m),
    ] {
        validate_finite("erosion.adjust.input", value)?;
        if value < 0.0 {
            let _ = field;
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.adjust.negative_input",
            });
        }
    }
    validate_finite("erosion.adjust.kicrat", inputs.baselines.kicrat)?;
    validate_finite("erosion.adjust.krcrat", inputs.baselines.krcrat)?;
    validate_finite("erosion.adjust.tccrat", inputs.baselines.tccrat)?;
    validate_finite("erosion.adjust.bconsd", inputs.baselines.bconsd)?;
    Ok(())
}
