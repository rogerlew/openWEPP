//! Wave-1 daily erodibility adjustment producer tests (erosion port
//! Increment-1b-B portable subset, SC-SED-001 INV-SED-007). Each producer
//! is checked against the hand-evaluated legacy equation.
#![allow(clippy::doc_markdown)]

use crate::{
    DirectRuntimeError, ErosionAdjustmentInputs, ErosionConsolidationBaselines,
    ErosionConsolidationInputs, ErosionFrostRegime, erosion_adjustment_factors,
    erosion_consolidation_baselines,
};

fn clay_loam_consolidation_inputs() -> ErosionConsolidationInputs {
    // McKenzie clay-loam fixture: sand 0.25, silt 0.45, orgmat 0.05,
    // ki = 1.5e6, kr = 6e-5, shcrit = 0.5, thetfc ~ 0.283 (fixture layer).
    ErosionConsolidationInputs {
        sand: 0.25,
        silt: 0.45,
        orgmat: 0.05,
        thetfc: 0.2833,
        rock_fragment_fraction: 0.05,
        ki: 1.5e6,
        kr: 6.0e-5,
        shcrit: 0.5,
    }
}

#[test]
fn consolidation_baselines_match_legacy_scon_clamps() {
    let baselines = erosion_consolidation_baselines(&clay_loam_consolidation_inputs())
        .expect("scon baselines must resolve");
    // Hand-eval interrill kconsd = 1000·(3042 − 3166·0.25 − 8816·0.05 −
    // 2477·0.2833) = 1000·(3042 − 791.5 − 440.8 − 701.7) = 1.108e6,
    // in-range; kicrat = 1.108e6/1.5e6 = 0.739, in [0.1, 1.0].
    let kconsd_ki: f64 = 1000.0 * (3042.0 - 3166.0 * 0.25 - 8816.0 * 0.05 - 2477.0 * 0.2833);
    let expected_kicrat = (kconsd_ki / 1.5e6).clamp(0.1, 1.0);
    assert!((baselines.kicrat - expected_kicrat).abs() < 1.0e-9);
    // Rill kconsd = 0.00035 − 0.0014·0.2833 + 0.00068·0.45 + 0.0049·0.05
    // = 0.000501, in-range; krcrat = 0.000501/6e-5 = 8.35 → clamp to 1.0.
    assert!((baselines.krcrat - 1.0).abs() < 1.0e-12);
    // tc kconsd = 8.37 − 11.8·0.2833 − 4.9·0.25 = 3.802; tccrat =
    // 3.802/0.5 = 7.6 → clamp to 4.0.
    assert!((baselines.tccrat - 4.0).abs() < 1.0e-12);
    assert!((baselines.bconsd - 0.02).abs() < 1.0e-12);
}

#[test]
fn consolidation_baselines_fail_closed_on_invalid_erodibility() {
    let mut inputs = clay_loam_consolidation_inputs();
    inputs.ki = 0.0;
    assert!(matches!(
        erosion_consolidation_baselines(&inputs),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
    let mut nan_inputs = clay_loam_consolidation_inputs();
    nan_inputs.thetfc = f64::NAN;
    assert!(matches!(
        erosion_consolidation_baselines(&nan_inputs),
        Err(DirectRuntimeError::NonFiniteDirectValue { .. })
    ));
}

fn bare_forest_adjustment_inputs(
    baselines: ErosionConsolidationBaselines,
    frost_regime: ErosionFrostRegime,
) -> ErosionAdjustmentInputs {
    // Bare high-severity burn: no cover, no roots, no residue.
    ErosionAdjustmentInputs {
        canopy_cover_fraction: 0.0,
        canopy_height_m: 0.0,
        interrill_cover_fraction: 0.0,
        live_root_mass_kg_m2: 0.0,
        dead_root_mass_kg_m2: 0.0,
        buried_residue_mass_kg_m2: 0.0,
        days_since_disturbance: 0.0,
        avg_slope: 0.43,
        ridge_height_m: 0.0,
        rill_spacing_m: 1.0,
        random_roughness_m: 0.006,
        baselines,
        frost_regime,
    }
}

#[test]
fn adjustment_factors_bare_unfrozen_day_zero_recovers_baseline_shape() {
    let baselines =
        erosion_consolidation_baselines(&clay_loam_consolidation_inputs()).expect("baselines");
    let inputs = bare_forest_adjustment_inputs(baselines, ErosionFrostRegime::Unfrozen);
    let factors = erosion_adjustment_factors(&inputs).expect("factors must resolve");

    // Day-zero (daydis = 0 → produc = 0 → decay = 1): ckiasc = kicrat +
    // (1 − kicrat)·1 = 1.0; ckrasc = 1.0; ctcasc = 1.0. Bare, unfrozen:
    // all cover/root subfactors = 1.0 except the slope factor.
    // ckiasa = 1.05 − 0.85·exp(−4·sin(0.43)).
    let ckiasa = 1.05 - 0.85 * (-4.0 * 0.43_f64.sin()).exp();
    // ctcarr = 1 + 8·(0.006 − 0.006) = 1.0. So kiadjf = ckiasa,
    // kradjf = 1.0, tcadjf = 1.0.
    assert!((factors.kiadjf - ckiasa).abs() < 1.0e-9);
    assert!((factors.kradjf - 1.0).abs() < 1.0e-9);
    assert!((factors.tcadjf - 1.0).abs() < 1.0e-9);
}

#[test]
fn adjustment_factors_consolidate_toward_baselines_over_time() {
    let baselines =
        erosion_consolidation_baselines(&clay_loam_consolidation_inputs()).expect("baselines");
    // Fully consolidated (daydis large → produc >= 10 → sealing = baseline).
    let mut aged = bare_forest_adjustment_inputs(baselines, ErosionFrostRegime::Unfrozen);
    aged.days_since_disturbance = 1000.0;
    let factors = erosion_adjustment_factors(&aged).expect("aged factors");
    // ckiasc collapses to kicrat (< 1), so kiadjf drops below the
    // day-zero value; ctcasc → tccrat (> 1) so tcadjf rises toward the cap.
    let ckiasa = 1.05 - 0.85 * (-4.0 * 0.43_f64.sin()).exp();
    assert!(factors.kiadjf < ckiasa);
    assert!((factors.kiadjf - baselines.kicrat * ckiasa).abs() < 1.0e-9);
    assert!(factors.tcadjf > 1.0);
    // tcadjf = tccrat (4.0) capped at 2.0.
    assert!((factors.tcadjf - 2.0).abs() < 1.0e-9);
}

#[test]
fn adjustment_factors_floor_and_cap_hold() {
    let baselines =
        erosion_consolidation_baselines(&clay_loam_consolidation_inputs()).expect("baselines");
    // Heavy cover + roots would drive ki/kr subfactors toward zero; the
    // 0.03 floor must hold.
    let mut heavy = bare_forest_adjustment_inputs(baselines, ErosionFrostRegime::Unfrozen);
    heavy.interrill_cover_fraction = 1.0;
    heavy.live_root_mass_kg_m2 = 5.0;
    heavy.dead_root_mass_kg_m2 = 5.0;
    heavy.buried_residue_mass_kg_m2 = 5.0;
    let factors = erosion_adjustment_factors(&heavy).expect("heavy factors");
    assert!((factors.kiadjf - 0.03).abs() < 1.0e-12);
    assert!((factors.kradjf - 0.03).abs() < 1.0e-12);
    assert!(factors.tcadjf <= 2.0 + 1.0e-12);
}

#[test]
fn adjustment_factors_frozen_surface_zeros_erodibility() {
    let baselines =
        erosion_consolidation_baselines(&clay_loam_consolidation_inputs()).expect("baselines");
    let inputs = bare_forest_adjustment_inputs(baselines, ErosionFrostRegime::FrozenSurface);
    let factors = erosion_adjustment_factors(&inputs).expect("frozen factors");
    // ckiaft = ckraft = 0 → kiadjf/kradjf collapse to the 0.03 floor;
    // tcaft = 1 so tcadjf follows the consolidation/roughness only.
    assert!((factors.kiadjf - 0.03).abs() < 1.0e-12);
    assert!((factors.kradjf - 0.03).abs() < 1.0e-12);
}

#[test]
fn adjustment_factors_thawing_regime_fails_closed_on_winter_fcycle() {
    let baselines =
        erosion_consolidation_baselines(&clay_loam_consolidation_inputs()).expect("baselines");
    let inputs = bare_forest_adjustment_inputs(baselines, ErosionFrostRegime::Thawing);
    // The actively-thawing branch needs the winter fcycle counter, absent
    // from the direct runtime: fail-closed, never a fabricated 1.0.
    assert!(matches!(
        erosion_adjustment_factors(&inputs),
        Err(DirectRuntimeError::MissingDirectUpstream { .. })
    ));
}

#[test]
fn adjustment_factors_fail_closed_on_nan_and_negative_inputs() {
    let baselines =
        erosion_consolidation_baselines(&clay_loam_consolidation_inputs()).expect("baselines");
    let mut nan_inputs = bare_forest_adjustment_inputs(baselines, ErosionFrostRegime::Unfrozen);
    nan_inputs.canopy_cover_fraction = f64::NAN;
    assert!(matches!(
        erosion_adjustment_factors(&nan_inputs),
        Err(DirectRuntimeError::NonFiniteDirectValue { .. })
    ));
    let mut negative_inputs =
        bare_forest_adjustment_inputs(baselines, ErosionFrostRegime::Unfrozen);
    negative_inputs.live_root_mass_kg_m2 = -1.0;
    assert!(matches!(
        erosion_adjustment_factors(&negative_inputs),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
}
