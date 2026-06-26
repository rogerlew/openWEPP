use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectDayFrame, DirectRunIdentity, DirectSnowCouplingInputs,
    DirectSnowHourlyForcing, SnowAlbedoModel, SnowAlbedoState, SnowAlbedoUpdateInputs,
    SnowMeltModel, Wb11HydrologyKernel, Wb11HydrologyKernelGuardError, update_snow_albedo_state,
};
use openwepp_kernel_contract::HillslopeKernelPhaseClass;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-05d-opt-in-coe-melt-implementation-001/package.md";
const HOUR_COUNT: usize = 24;
const TOL: f64 = 1.0e-12;

fn read(path: &str) -> String {
    fs::read_to_string(Path::new(path)).expect("fixture document should be readable")
}

#[test]
fn snowdensity05d_contract_markers_bind_opt_in_melt_wiring() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 79",
        "INV-SNOWFREEZE-055",
        "OBL-SNOWFREEZE-P-030",
        "snow_melt_shortwave_absorbed_fraction",
        "SNOWDENSITY-05D Opt-In CoE Melt Implementation Addendum",
        "amelt = 0.0607 * hrrad * (1 - snow_albedo) * (1 - cancov)",
        "legacy_coe` remains the default",
        "WB12 signed liquid forcing, and WB13 routed liquid forcing",
    ] {
        assert!(
            contract.contains(marker),
            "missing contract marker: {marker}"
        );
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-05D Opt-In CoE Melt Implementation",
        "legacy_coe` default path is identity",
        "independently reconstructs",
        "Do not close this package merely because the selector compiles",
    ] {
        assert!(package.contains(marker), "missing package marker: {marker}");
    }
}

fn warm_radiation_inputs(model: SnowMeltModel) -> DirectActiveSnowPartitionInputs {
    let mut hourly = [DirectSnowHourlyForcing {
        cloud_fraction: 1.0,
        ..DirectSnowHourlyForcing::zero()
    }; HOUR_COUNT];
    hourly[0].radiation_mj_m2 = 10.0;

    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.0,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.4,
        runtime_depth_m: 1.0,
        runtime_density_kg_m3: 400.0,
        runtime_settle_day_count: 4.0,
        tmax_c: 0.0,
        tmin_c: 0.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 0.0,
        dewpoint_c: 0.0,
        snow_melt_model: model,
        snow_albedo_model: (model == SnowMeltModel::CoeShortwaveAlbedoV1)
            .then_some(SnowAlbedoModel::Brock2000TemperatureAgeV1),
        snow_albedo_state: (model == SnowMeltModel::CoeShortwaveAlbedoV1).then_some(
            SnowAlbedoState {
                model: SnowAlbedoModel::Brock2000TemperatureAgeV1,
                albedo: 0.6,
                accumulated_positive_temperature_c_day: 16.0,
            },
        ),
        underlying_surface_albedo: 0.2,
        hourly,
    }
}

#[test]
fn snowdensity05d_opt_in_changes_only_shortwave_amelt_operand() {
    let legacy = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        warm_radiation_inputs(SnowMeltModel::LegacyCoe),
    )
    .expect("legacy CoE melt should compute");
    let opt_in = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        warm_radiation_inputs(SnowMeltModel::CoeShortwaveAlbedoV1),
    )
    .expect("opt-in CoE melt should compute with prior albedo state");

    let opt_in_hour_one_albedo = update_snow_albedo_state(SnowAlbedoUpdateInputs {
        melt_model: SnowMeltModel::CoeShortwaveAlbedoV1,
        albedo_model: Some(SnowAlbedoModel::Brock2000TemperatureAgeV1),
        previous_state: warm_radiation_inputs(SnowMeltModel::CoeShortwaveAlbedoV1)
            .snow_albedo_state,
        snow_water_equivalent_m: 0.4,
        fresh_snow_water_equivalent_m: 0.0,
        positive_temperature_c_day_increment: 0.0,
        underlying_surface_albedo: 0.2,
    })
    .expect("hour-one albedo update should reconstruct")
    .state
    .expect("active snow should have albedo state")
    .albedo;
    let legacy_expected_raw_m = 0.0607 * 10.0 * 0.0254;
    let opt_in_expected_raw_m = 0.0607 * 10.0 * (1.0 - opt_in_hour_one_albedo) * 0.0254;

    assert!((legacy.raw_melt_m - legacy_expected_raw_m).abs() <= TOL);
    assert!((legacy.redistributed_melt_m - legacy.raw_melt_m).abs() <= TOL);
    assert!((legacy.routed_melt_m - legacy.raw_melt_m).abs() <= TOL);
    assert!((legacy.snowpack_swe_loss_m - legacy.raw_melt_m).abs() <= TOL);
    assert!((legacy.snow_coupling_signed_s_m - legacy.raw_melt_m).abs() <= TOL);
    assert!((legacy.runtime_swe_after_m - (0.4 - legacy.snowpack_swe_loss_m)).abs() <= TOL);
    assert!(legacy.post_winter_rain_m.abs() <= TOL);
    assert_eq!(legacy.snow_albedo_state_after, None);

    assert!((opt_in.raw_melt_m - opt_in_expected_raw_m).abs() <= TOL);
    assert!((opt_in.redistributed_melt_m - opt_in.raw_melt_m).abs() <= TOL);
    assert!((opt_in.routed_melt_m - opt_in.raw_melt_m).abs() <= TOL);
    assert!((opt_in.snowpack_swe_loss_m - opt_in.raw_melt_m).abs() <= TOL);
    assert!((opt_in.snow_coupling_signed_s_m - opt_in.raw_melt_m).abs() <= TOL);
    assert!((opt_in.runtime_swe_after_m - (0.4 - opt_in.snowpack_swe_loss_m)).abs() <= TOL);
    assert!(opt_in.raw_melt_m < legacy.raw_melt_m);
}

#[test]
fn snowdensity05d_missing_active_opt_in_albedo_state_fails_closed() {
    let mut inputs = warm_radiation_inputs(SnowMeltModel::CoeShortwaveAlbedoV1);
    inputs.snow_albedo_state = None;

    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(inputs)
        .expect_err("active opt-in snow without prior albedo state must fail closed");

    assert!(matches!(
        error,
        Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
            phase_class: HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            ..
        }
    ));
}

#[test]
fn snowdensity05d_direct_runtime_projects_routed_melt_and_albedo_carry() {
    let opt_in = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        warm_radiation_inputs(SnowMeltModel::CoeShortwaveAlbedoV1),
    )
    .expect("opt-in CoE melt should compute");
    let identity =
        DirectRunIdentity::new(505, 2026, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: opt_in.snow_coupling_signed_s_m,
        snow_state_projected: true,
        active_snow_coupling: opt_in.active_snow_coupling,
        raw_melt_m: opt_in.raw_melt_m,
        redistributed_melt_m: opt_in.redistributed_melt_m,
        routed_melt_m: opt_in.routed_melt_m,
        snowpack_swe_loss_m: opt_in.snowpack_swe_loss_m,
        post_winter_rain_m: opt_in.post_winter_rain_m,
        runtime_swe_after_m: opt_in.runtime_swe_after_m,
        runtime_depth_after_m: opt_in.runtime_depth_after_m,
        runtime_density_after_kg_m3: opt_in.runtime_density_after_kg_m3,
        runtime_settle_day_count_after: opt_in.runtime_settle_day_count_after,
        snow_albedo_state_after: opt_in.snow_albedo_state_after,
    };
    day.run_r4g_snow_coupling_span()
        .expect("direct snow-coupling span should project opt-in carry");

    assert!((day.snow_coupling.raw_melt_m - opt_in.raw_melt_m).abs() <= TOL);
    assert!((day.snow_coupling.redistributed_melt_m - opt_in.redistributed_melt_m).abs() <= TOL);
    assert!((day.snow_coupling.routed_melt_m - opt_in.routed_melt_m).abs() <= TOL);
    assert!(
        (day.storage_reconciliation_inputs.snow_coupling_m - opt_in.snow_coupling_signed_s_m).abs()
            <= TOL
    );
    assert!(
        (day.snow_coupling_downstream_operands.routed_melt_m - opt_in.routed_melt_m).abs() <= TOL
    );
    assert_eq!(
        day.snow_coupling_shadow_projection
            .expect("shadow projection")
            .snow_albedo_state_after,
        opt_in.snow_albedo_state_after
    );
    assert_eq!(
        day.winter_column.snow.snow_albedo_state,
        opt_in.snow_albedo_state_after
    );
    assert_eq!(
        day.snow_runtime_carry
            .expect("snow carry")
            .snow_albedo_state,
        opt_in.snow_albedo_state_after
    );
}
