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
pub fn erosion_consolidation_baselines(
    inputs: &ErosionConsolidationInputs,
) -> Result<ErosionConsolidationBaselines, DirectRuntimeError> {
    validate_finite("erosion.scon.sand", inputs.sand)?;
    validate_finite("erosion.scon.silt", inputs.silt)?;
    validate_finite("erosion.scon.orgmat", inputs.orgmat)?;
    validate_finite("erosion.scon.thetfc", inputs.thetfc)?;
    validate_finite("erosion.scon.rfg", inputs.rock_fragment_fraction)?;
    if inputs.ki <= 0.0 || inputs.kr <= 0.0 || inputs.shcrit <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.scon.nonpositive_baseline_erodibility",
        });
    }

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
