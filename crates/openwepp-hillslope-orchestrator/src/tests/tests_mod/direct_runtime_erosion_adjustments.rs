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
    // Non-positive baseline erodibility is a typed domain error.
    for zero_field in ["ki", "kr", "shcrit"] {
        let mut inputs = clay_loam_consolidation_inputs();
        match zero_field {
            "ki" => inputs.ki = 0.0,
            "kr" => inputs.kr = 0.0,
            _ => inputs.shcrit = 0.0,
        }
        assert!(matches!(
            erosion_consolidation_baselines(&inputs),
            Err(DirectRuntimeError::DirectDomainViolation { .. })
        ));
    }
    // NaN baseline erodibility must be a typed error, not a NaN ratio
    // (the `<= 0.0` check alone misses NaN).
    for nan_field in ["ki", "kr", "shcrit"] {
        let mut inputs = clay_loam_consolidation_inputs();
        match nan_field {
            "ki" => inputs.ki = f64::NAN,
            "kr" => inputs.kr = f64::NAN,
            _ => inputs.shcrit = f64::NAN,
        }
        assert!(matches!(
            erosion_consolidation_baselines(&inputs),
            Err(DirectRuntimeError::NonFiniteDirectValue { .. })
        ));
    }
    // NaN field capacity is a typed error.
    let mut nan_inputs = clay_loam_consolidation_inputs();
    nan_inputs.thetfc = f64::NAN;
    assert!(matches!(
        erosion_consolidation_baselines(&nan_inputs),
        Err(DirectRuntimeError::NonFiniteDirectValue { .. })
    ));
}

#[test]
fn consolidation_baselines_fail_closed_on_out_of_range_texture() {
    // Texture / rock-fraction / field-capacity inputs outside [0, 1] must
    // be typed errors, not silently absorbed by the scon clamps.
    type Mutator = fn(&mut ErosionConsolidationInputs);
    let cases: [(&str, Mutator); 6] = [
        ("sand>1", |i| i.sand = 1.5),
        ("sand<0", |i| i.sand = -0.1),
        ("silt<0", |i| i.silt = -0.1),
        ("orgmat>1", |i| i.orgmat = 2.0),
        ("thetfc>1", |i| i.thetfc = 1.5),
        ("rfg<0", |i| i.rock_fragment_fraction = -0.2),
    ];
    for (label, mutate) in cases {
        let mut inputs = clay_loam_consolidation_inputs();
        mutate(&mut inputs);
        assert!(
            matches!(
                erosion_consolidation_baselines(&inputs),
                Err(DirectRuntimeError::DirectDomainViolation { .. })
            ),
            "out-of-range texture case {label} must fail closed"
        );
    }
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

use crate::{
    DirectErosionConsolidationCarry, DirectRuntimeError as AdjustError, ErosionFrostInputs,
    ErosionIfrostCarry, ErosionRfcumInputs, advance_erosion_consolidation,
    resolve_erosion_frost_regime,
};

fn frost_inputs(
    frost_depth_m: f64,
    thaw_depth_m: f64,
    surface_layer_water: f64,
    surface_layer_thetfc: f64,
) -> ErosionFrostInputs {
    ErosionFrostInputs {
        frost_depth_m,
        thaw_depth_m,
        surface_layer_water,
        surface_layer_thetfc,
    }
}

/// Forest rfcum inputs: precip only (no irrigation, no tillage).
fn forest_rfcum(precipitation_m: f64, mean_temperature_c: f64) -> ErosionRfcumInputs {
    ErosionRfcumInputs {
        precipitation_m,
        irrigation_depth_m: 0.0,
        mean_temperature_c,
        irrigation_is_furrow: false,
        tillage_surface_disturbance: None,
    }
}

#[test]
fn frost_regime_resolves_the_three_soil_for_branches() {
    // Frozen surface: frdp > 0, thdp <= 0.
    let (regime, ifrost) = resolve_erosion_frost_regime(
        &frost_inputs(0.05, 0.0, 0.4, 0.28),
        ErosionIfrostCarry::unfrozen(),
    )
    .expect("frozen regime");
    assert_eq!(regime, ErosionFrostRegime::FrozenSurface);
    assert_eq!(ifrost, ErosionIfrostCarry(1));

    // Unfrozen and dry (pwater <= thetfc): factors are 1.0.
    let (regime, ifrost) =
        resolve_erosion_frost_regime(&frost_inputs(0.0, 0.0, 0.2, 0.28), ErosionIfrostCarry(1))
            .expect("unfrozen regime");
    assert_eq!(regime, ErosionFrostRegime::Unfrozen);
    assert_eq!(ifrost, ErosionIfrostCarry(0));

    // Thawing: not frozen, wet (pwater > thetfc), and the prior surface
    // was frozen -> the ifrost==2 branch (fail-closed downstream).
    let (regime, ifrost) =
        resolve_erosion_frost_regime(&frost_inputs(0.02, 0.03, 0.4, 0.28), ErosionIfrostCarry(1))
            .expect("thawing regime");
    assert_eq!(regime, ErosionFrostRegime::Thawing);
    assert_eq!(ifrost, ErosionIfrostCarry(2));

    // Wet but prior unfrozen -> stays unfrozen (thaw only from frozen).
    let (regime, _) = resolve_erosion_frost_regime(
        &frost_inputs(0.0, 0.0, 0.4, 0.28),
        ErosionIfrostCarry::unfrozen(),
    )
    .expect("unfrozen regime");
    assert_eq!(regime, ErosionFrostRegime::Unfrozen);
}

#[test]
fn frost_regime_fails_closed_on_nan_and_invalid_ifrost() {
    assert!(matches!(
        resolve_erosion_frost_regime(
            &frost_inputs(f64::NAN, 0.0, 0.4, 0.28),
            ErosionIfrostCarry::unfrozen()
        ),
        Err(AdjustError::NonFiniteDirectValue { .. })
    ));
    assert!(matches!(
        resolve_erosion_frost_regime(
            &frost_inputs(0.0, 0.0, f64::NAN, 0.28),
            ErosionIfrostCarry(1)
        ),
        Err(AdjustError::NonFiniteDirectValue { .. })
    ));
    // An out-of-range ifrost carry (> 2) is a typed error.
    assert!(matches!(
        resolve_erosion_frost_regime(&frost_inputs(0.0, 0.0, 0.4, 0.28), ErosionIfrostCarry(7)),
        Err(AdjustError::DirectDomainViolation { .. })
    ));
}

#[test]
fn consolidation_carry_accumulates_and_ages() {
    // Seed from a management initial daydis.
    let mut carry = DirectErosionConsolidationCarry::seed(0.0);
    assert_eq!(carry.rfcum_m, 0.0);
    assert_eq!(carry.daydis, 0.0);

    // Warm rainy day: rfcum accumulates; daydis does NOT increment yet
    // (prior rfcum was 0, below the 0.01 onset).
    carry = advance_erosion_consolidation(carry, &forest_rfcum(0.02, 10.0)).expect("warm day");
    assert!((carry.rfcum_m - 0.02).abs() < 1.0e-12);
    assert_eq!(carry.daydis, 0.0);

    // Next warm day: prior rfcum (0.02) > 0.01, so daydis increments.
    carry = advance_erosion_consolidation(carry, &forest_rfcum(0.0, 10.0)).expect("next warm day");
    assert!((carry.daydis - 1.0).abs() < 1.0e-12);

    // A sub-freezing day does not accumulate rain but still ages.
    let before = carry;
    carry = advance_erosion_consolidation(carry, &forest_rfcum(0.05, -3.0)).expect("cold day");
    assert!((carry.rfcum_m - before.rfcum_m).abs() < 1.0e-12);
    assert!((carry.daydis - (before.daydis + 1.0)).abs() < 1.0e-12);
}

#[test]
fn consolidation_carry_tillage_resets_age_and_rfcum() {
    let carry = DirectErosionConsolidationCarry {
        rfcum_m: 0.5,
        daydis: 200.0,
    };
    // A tillage day with surdis = 0.75 scales daydis by (1 - 0.75) and
    // resets rfcum (then accumulates today's warm rain).
    let tilled = advance_erosion_consolidation(
        carry,
        &ErosionRfcumInputs {
            precipitation_m: 0.01,
            irrigation_depth_m: 0.0,
            mean_temperature_c: 8.0,
            irrigation_is_furrow: false,
            tillage_surface_disturbance: Some(0.75),
        },
    )
    .expect("tillage day");
    assert!((tilled.daydis - 50.0).abs() < 1.0e-9);
    assert!((tilled.rfcum_m - 0.01).abs() < 1.0e-12);
}

#[test]
fn consolidation_carry_irrigation_split_matches_legacy() {
    // Sprinkler / none (irsyst <= 1): irrigation always adds to rfcum even
    // on a sub-freezing day, while precipitation is temperature-gated
    // (soil.for:837-845).
    let carry = DirectErosionConsolidationCarry::seed(0.0);
    let cold_irrigated = advance_erosion_consolidation(
        carry,
        &ErosionRfcumInputs {
            precipitation_m: 0.03,
            irrigation_depth_m: 0.01,
            mean_temperature_c: -2.0,
            irrigation_is_furrow: false,
            tillage_surface_disturbance: None,
        },
    )
    .expect("cold sprinkler-irrigated day");
    // Cold: precip excluded, irrigation included -> rfcum = 0.01.
    assert!((cold_irrigated.rfcum_m - 0.01).abs() < 1.0e-12);

    // Furrow (irsyst == 2): irrigation water is excluded from rfcum; only
    // warm-day precipitation counts.
    let furrow = advance_erosion_consolidation(
        DirectErosionConsolidationCarry::seed(0.0),
        &ErosionRfcumInputs {
            precipitation_m: 0.03,
            irrigation_depth_m: 0.01,
            mean_temperature_c: -2.0,
            irrigation_is_furrow: true,
            tillage_surface_disturbance: None,
        },
    )
    .expect("cold furrow-irrigated day");
    // Cold + furrow: neither precip nor irrigation -> rfcum = 0.
    assert_eq!(furrow.rfcum_m, 0.0);
}

#[test]
fn consolidation_carry_fails_closed_on_nan_and_negative() {
    let carry = DirectErosionConsolidationCarry::seed(0.0);
    assert!(matches!(
        advance_erosion_consolidation(carry, &forest_rfcum(f64::NAN, 10.0)),
        Err(AdjustError::NonFiniteDirectValue { .. })
    ));
    assert!(matches!(
        advance_erosion_consolidation(carry, &forest_rfcum(-0.01, 10.0)),
        Err(AdjustError::DirectDomainViolation { .. })
    ));
    // surdis out of [0, 1] is a typed error.
    assert!(matches!(
        advance_erosion_consolidation(
            carry,
            &ErosionRfcumInputs {
                precipitation_m: 0.01,
                irrigation_depth_m: 0.0,
                mean_temperature_c: 8.0,
                irrigation_is_furrow: false,
                tillage_surface_disturbance: Some(1.5),
            },
        ),
        Err(AdjustError::DirectDomainViolation { .. })
    ));
}
