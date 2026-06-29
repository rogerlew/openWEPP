use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectDayFrame, DirectRunIdentity, DirectSnowCouplingInputs,
    DirectSnowHourlyForcing, SnowDensityModel, SnowMeltModel, Wb11HydrologyKernel,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = "docs/work-packages/20260626-snowdensity-07-runtime-opt-in-001/package.md";
const DIRECT_PUBLICATION_BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00_builders_and_authority.rs"
);
const DIRECT_PUBLICATION_SNOW_FROST_IMPL: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00a_snow_frost_authority_impl.rs"
);
const HOUR_COUNT: usize = 24;
const TOL: f64 = 1.0e-12;

fn read(path: &str) -> String {
    fs::read_to_string(Path::new(path)).expect("fixture document should be readable")
}

#[test]
fn snowdensity07_contract_and_package_bind_runtime_opt_in_authority() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 112",
        "INV-SNOWFREEZE-060",
        "OBL-SNOWFREEZE-P-035",
        "snow_density_model",
        "physics_bulk_density_compaction_v1",
        "SNOWDENSITY-07 Runtime Opt-In Addendum",
        "CoE SWE/liquid/routed-melt identity",
        "mutate only runtime physical depth and density",
    ] {
        assert!(
            contract.contains(marker),
            "missing contract marker: {marker}"
        );
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-07 Runtime Opt-In",
        "default-disabled selector",
        "CoE boundary depth/density carry remains separate",
        "surface-driven compatibility/default path still uses `legacy_wepp`",
    ] {
        assert!(package.contains(marker), "missing package marker: {marker}");
    }
}

fn cold_pack_inputs(model: SnowDensityModel) -> DirectActiveSnowPartitionInputs {
    let mut hourly = [DirectSnowHourlyForcing {
        air_temperature_c: -6.0,
        cloud_fraction: 1.0,
        ..DirectSnowHourlyForcing::zero()
    }; HOUR_COUNT];
    hourly[0].snowfall_m = 0.02;
    hourly[0].air_temperature_c = -4.0;

    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.02,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.20,
        runtime_depth_m: 1.00,
        runtime_density_kg_m3: 200.0,
        runtime_settle_day_count: 4.0,
        liquid_water_retained_m: 0.0,
        tmax_c: -4.0,
        tmin_c: -8.0,
        canopy_cover_fraction: 0.9,
        wind_m_s: 0.0,
        dewpoint_c: -6.0,
        snow_melt_model: SnowMeltModel::LegacyCoe,
        snow_density_model: model,
        stage3_liquid_routing_model:
            openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::Disabled,
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 1.0,
        coe_boundary_density_kg_m3: 200.0,
        coe_boundary_settle_day_count: 4.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: Vec::new(),
        underlying_surface_albedo: 0.2,
        hourly,
    }
}

#[test]
fn snowdensity07_opt_in_changes_only_runtime_density_depth_surface() {
    let legacy = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &cold_pack_inputs(SnowDensityModel::LegacyWepp),
    )
    .expect("legacy density path should compute");
    let opt_in = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &cold_pack_inputs(SnowDensityModel::PhysicsBulkDensityCompactionV1),
    )
    .expect("opt-in density path should compute");

    assert_eq!(legacy.snow_density_model, SnowDensityModel::LegacyWepp);
    assert_eq!(
        opt_in.snow_density_model,
        SnowDensityModel::PhysicsBulkDensityCompactionV1
    );

    assert!((opt_in.runtime_swe_after_m - legacy.runtime_swe_after_m).abs() <= TOL);
    assert!((opt_in.snow_coupling_signed_s_m - legacy.snow_coupling_signed_s_m).abs() <= TOL);
    assert!((opt_in.raw_melt_m - legacy.raw_melt_m).abs() <= TOL);
    assert!((opt_in.redistributed_melt_m - legacy.redistributed_melt_m).abs() <= TOL);
    assert!((opt_in.routed_melt_m - legacy.routed_melt_m).abs() <= TOL);
    assert!((opt_in.snowpack_swe_loss_m - legacy.snowpack_swe_loss_m).abs() <= TOL);
    assert!((opt_in.post_winter_rain_m - legacy.post_winter_rain_m).abs() <= TOL);
    assert_eq!(
        opt_in.snow_albedo_state_after,
        legacy.snow_albedo_state_after
    );

    assert!((opt_in.coe_boundary_depth_after_m - legacy.runtime_depth_after_m).abs() <= TOL);
    assert!(
        (opt_in.coe_boundary_density_after_kg_m3 - legacy.runtime_density_after_kg_m3).abs() <= TOL
    );
    assert!(
        (opt_in.coe_boundary_settle_day_count_after - legacy.runtime_settle_day_count_after).abs()
            <= TOL
    );
    assert!(opt_in.density_swe_identity_residual_m <= TOL);
    assert!(opt_in.density_unbounded_swe_residual_m.is_finite());

    assert!(
        (opt_in.runtime_depth_after_m - legacy.runtime_depth_after_m).abs() > 1.0e-6,
        "opt-in should publish a distinct physical snow depth"
    );
    assert!(
        (opt_in.runtime_density_after_kg_m3 - legacy.runtime_density_after_kg_m3).abs() > 1.0e-6,
        "opt-in should publish a distinct physical snow density"
    );
}

#[test]
fn snowdensity07_r4g_projects_runtime_and_boundary_carry_without_compat_edge() {
    let opt_in = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &cold_pack_inputs(SnowDensityModel::PhysicsBulkDensityCompactionV1),
    )
    .expect("opt-in density path should compute");
    let identity =
        DirectRunIdentity::new(607, 2026, 1, 1).expect("valid direct identity should construct");
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
        sublimation_m: opt_in.sublimation_m,
        post_winter_rain_m: opt_in.post_winter_rain_m,
        runtime_swe_after_m: opt_in.runtime_swe_after_m,
        runtime_depth_after_m: opt_in.runtime_depth_after_m,
        runtime_density_after_kg_m3: opt_in.runtime_density_after_kg_m3,
        runtime_settle_day_count_after: opt_in.runtime_settle_day_count_after,
        coe_boundary_depth_after_m: opt_in.coe_boundary_depth_after_m,
        coe_boundary_density_after_kg_m3: opt_in.coe_boundary_density_after_kg_m3,
        coe_boundary_settle_day_count_after: opt_in.coe_boundary_settle_day_count_after,
        liquid_holding_capacity_after_m: opt_in.liquid_holding_capacity_after_m,
        liquid_water_retained_after_m: opt_in.liquid_water_retained_after_m,
        liquid_water_released_m: opt_in.liquid_water_released_m,
        snow_albedo_state_after: opt_in.snow_albedo_state_after,
        snow_layers_after: opt_in.snow_layers_after.clone(),
        stage3_diagnostics: opt_in.stage3_diagnostics.boxed_when_enabled(),
    };

    let report = day
        .run_r4g_snow_coupling_span()
        .expect("R4G snow coupling should project opt-in density state");

    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);

    assert!((day.winter_column.snow.runtime_swe_m - opt_in.runtime_swe_after_m).abs() <= TOL);
    assert!((day.winter_column.snow.runtime_depth_m - opt_in.runtime_depth_after_m).abs() <= TOL);
    assert!(
        (day.winter_column.snow.runtime_density_kg_m3 - opt_in.runtime_density_after_kg_m3).abs()
            <= TOL
    );
    assert!(
        (day.winter_column.snow.coe_boundary_depth_m - opt_in.coe_boundary_depth_after_m).abs()
            <= TOL
    );
    assert!(
        (day.winter_column.snow.coe_boundary_density_kg_m3
            - opt_in.coe_boundary_density_after_kg_m3)
            .abs()
            <= TOL
    );
    assert!(
        (day.snow_coupling_downstream_operands.runtime_depth_after_m
            - opt_in.runtime_depth_after_m)
            .abs()
            <= TOL
    );
    assert!(
        (day.snow_coupling_downstream_operands
            .coe_boundary_depth_after_m
            - opt_in.coe_boundary_depth_after_m)
            .abs()
            <= TOL
    );

    let shadow = day
        .snow_coupling_shadow_projection
        .expect("R4G should emit a shadow projection");
    assert_eq!(shadow, report.snow_coupling_shadow_projection);
    assert!((shadow.runtime_density_after_kg_m3 - opt_in.runtime_density_after_kg_m3).abs() <= TOL);
    assert!(
        (shadow.coe_boundary_density_after_kg_m3 - opt_in.coe_boundary_density_after_kg_m3).abs()
            <= TOL
    );

    let carry = day
        .snow_runtime_carry
        .expect("R4G projected snow should update runtime carry");
    assert!((carry.runtime_depth_m - opt_in.runtime_depth_after_m).abs() <= TOL);
    assert!((carry.coe_boundary_depth_m - opt_in.coe_boundary_depth_after_m).abs() <= TOL);
}

#[test]
fn snowdensity07_runtime_opt_in_is_superseded_by_10_3_15_default_activation() {
    let builder = format!(
        "{}\n{}",
        read(DIRECT_PUBLICATION_BUILDER),
        read(DIRECT_PUBLICATION_SNOW_FROST_IMPL)
    );

    assert!(
        builder.contains("SNOWDENSITY09_DENSITY_MODEL_ENV")
            && builder.contains("Err(std::env::VarError::NotPresent)")
            && builder.contains("SnowDensityModel::PhysicsBulkDensityCompactionV1")
            && builder.contains("\"legacy_wepp\" => Ok"),
        "SNOWDENSITY-10.3.15 must make physics_bulk_density_compaction_v1 the absent-selector default while retaining legacy rollback"
    );
    assert!(
        builder.contains("snow_density_model: self.snow_density_model")
            && builder.contains("SnowDensityModel::PhysicsBulkDensityCompactionV1"),
        "surface-driven publication path must preserve selected density handoff"
    );
    assert!(
        builder.contains("SNOWDENSITY1037_MELT_MODEL_ENV")
            && builder.contains("SnowMeltModel::LegacyCoe")
            && builder.contains("SnowMeltModel::CoeLiquidHoldingCapacityV1"),
        "historical melt diagnostic hook and active default/rollback models must remain visible"
    );
}
