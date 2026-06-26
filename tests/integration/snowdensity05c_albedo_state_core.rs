use std::fs;

use openwepp_hillslope_orchestrator::{
    SnowAlbedoError, SnowAlbedoModel, SnowAlbedoUpdateInputs, SnowMeltModel,
    update_snow_albedo_state,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-05c-albedo-state-core-001/package.md";
const HANDOFF: &str =
    "docs/work-packages/20260626-snowdensity-05c-albedo-state-core-001/artifacts/worker-handoff.md";
const AUTHORITY: &str = "docs/work-packages/20260626-snowdensity-05c-albedo-state-core-001/artifacts/albedo-authority-ledger.md";

#[test]
fn snowdensity05c_contract_ratifies_albedo_state_core() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 83",
        "INV-SNOWFREEZE-054",
        "SNOWDENSITY-05C albedo-state core",
        "`brock2000_temperature_age_v1`",
        "`snow_albedo_accumulated_positive_temperature_c_day`",
        "`snow_albedo_fresh_snow_reset_water_equiv_m`",
        "`0.713 - 0.112 * log10(Ta)`",
        "`underlying_albedo + 0.442 * exp(-0.058 * Ta)`",
        "`d_star = 0.024 m water equivalent`",
        "`0.85`",
        "missing opt-in albedo state",
        "OBL-SNOWFREEZE-P-029",
        "SNOWDENSITY-05C Albedo State Core Addendum",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }
}

#[test]
fn snowdensity05c_brock_core_bounds_decay_and_resets() {
    let reset = update_snow_albedo_state(SnowAlbedoUpdateInputs {
        melt_model: SnowMeltModel::CoeShortwaveAlbedoV1,
        albedo_model: Some(SnowAlbedoModel::Brock2000TemperatureAgeV1),
        previous_state: None,
        snow_water_equivalent_m: 0.20,
        fresh_snow_water_equivalent_m: 0.01,
        positive_temperature_c_day_increment: 0.0,
        underlying_surface_albedo: 0.20,
    })
    .expect("fresh snowfall may initialize opt-in albedo state");
    let fresh = reset
        .state
        .expect("active snow cover should publish albedo state");
    assert_eq!(fresh.model, SnowAlbedoModel::Brock2000TemperatureAgeV1);
    assert!((0.0..=0.85).contains(&fresh.albedo));
    assert!(fresh.albedo >= 0.80);

    let aged = update_snow_albedo_state(SnowAlbedoUpdateInputs {
        melt_model: SnowMeltModel::CoeShortwaveAlbedoV1,
        albedo_model: Some(SnowAlbedoModel::Brock2000TemperatureAgeV1),
        previous_state: Some(fresh),
        snow_water_equivalent_m: 0.20,
        fresh_snow_water_equivalent_m: 0.0,
        positive_temperature_c_day_increment: 80.0,
        underlying_surface_albedo: 0.20,
    })
    .expect("positive temperature age should update opt-in state")
    .state
    .expect("active snow cover should retain albedo state");
    assert!(aged.albedo < fresh.albedo);
    assert!(aged.albedo > 0.0);

    let refreshed = update_snow_albedo_state(SnowAlbedoUpdateInputs {
        melt_model: SnowMeltModel::CoeShortwaveAlbedoV1,
        albedo_model: Some(SnowAlbedoModel::Brock2000TemperatureAgeV1),
        previous_state: Some(aged),
        snow_water_equivalent_m: 0.20,
        fresh_snow_water_equivalent_m: 0.01,
        positive_temperature_c_day_increment: 0.0,
        underlying_surface_albedo: 0.20,
    })
    .expect("fresh snowfall should reset accumulated positive temperature")
    .state
    .expect("active snow cover should retain albedo state");
    assert!(refreshed.albedo > aged.albedo);
    assert!(refreshed.accumulated_positive_temperature_c_day <= 1.0e-12);
}

#[test]
fn snowdensity05c_core_fails_closed_when_opt_in_state_is_missing() {
    let err = update_snow_albedo_state(SnowAlbedoUpdateInputs {
        melt_model: SnowMeltModel::CoeShortwaveAlbedoV1,
        albedo_model: Some(SnowAlbedoModel::Brock2000TemperatureAgeV1),
        previous_state: None,
        snow_water_equivalent_m: 0.20,
        fresh_snow_water_equivalent_m: 0.0,
        positive_temperature_c_day_increment: 1.0,
        underlying_surface_albedo: 0.20,
    })
    .expect_err("aged opt-in snowpack requires previous albedo state");
    assert!(matches!(err, SnowAlbedoError::MissingRequiredAlbedoState));
}

#[test]
fn snowdensity05c_core_is_inactive_for_legacy_coe_default() {
    let outcome = update_snow_albedo_state(SnowAlbedoUpdateInputs {
        melt_model: SnowMeltModel::LegacyCoe,
        albedo_model: None,
        previous_state: None,
        snow_water_equivalent_m: 0.20,
        fresh_snow_water_equivalent_m: 0.0,
        positive_temperature_c_day_increment: 1.0,
        underlying_surface_albedo: 0.20,
    })
    .expect("legacy_coe must not require opt-in albedo state");
    assert!(!outcome.active);
    assert!(outcome.state.is_none());
}

#[test]
fn snowdensity05c_package_closes_with_05d_handoff() {
    let package = read(PACKAGE);
    for marker in [
        "Status: complete.",
        "Package type: contract/albedo-state-core implementation package.",
        "Closure: COMPLETE-05C-ALBEDO-STATE-CORE.",
        "No routed-melt acceptance or `coe_shortwave_albedo_v1` production wiring",
        "Subagent authorization: not used.",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let authority = read(AUTHORITY);
    for marker in [
        "Brock et al. (2000)",
        "`brock2000_temperature_age_v1`",
        "`Ta`",
        "`0.713 - 0.112 * log10(Ta)`",
        "`underlying_albedo + 0.442 * exp(-0.058 * Ta)`",
        "`d_star = 0.024 m water equivalent`",
        "No SNOTEL site fitting",
    ] {
        assert_contains(&authority, marker, AUTHORITY);
    }

    let handoff = read(HANDOFF);
    for marker in [
        "Next recommended package: `SNOWDENSITY-05D Opt-In CoE Melt Implementation`",
        "Consume `SnowAlbedoState` only when `snow_melt_model = coe_shortwave_albedo_v1`.",
        "Preserve `legacy_coe` default behavior.",
        "Do not alter the 05B radiation-source binding.",
    ] {
        assert_contains(&handoff, marker, HANDOFF);
    }
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "expected {path} to contain marker: {marker}"
    );
}
