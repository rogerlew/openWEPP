use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, SnowDensityModel, SnowMeltModel,
    Wb11HydrologyKernel,
};
use openwepp_runner::CoeMeltModel;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/package.md";
const DIRECT_PUBLICATION_BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00_builders_and_authority.rs"
);
const DIRECT_PUBLICATION_SNOW_FROST_IMPL: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00a_snow_frost_authority_impl.rs"
);
const COE_MELT_SNOWBENCH: &str = "crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs";
const TOL: f64 = 1.0e-12;

fn repo_text(relative_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn assert_contains(haystack: &str, needle: &str, context: &str) {
    assert!(
        haystack.contains(needle),
        "{context} missing required marker: {needle}"
    );
}

#[test]
fn contract_and_package_bind_liquid_holding_capacity_candidate() {
    let contract = repo_text(CONTRACT);
    for marker in [
        "contract_version: 113",
        "REF-SNOWFREEZE-MARKS1998-LIQUID-CAPACITY",
        "REF-SNOWFREEZE-ANDERSON1976-LIQUID",
        "REF-SNOWFREEZE-SNOW17-PLWHC",
        "REF-SNOWFREEZE-SNOBAL-LIQUID-CAPACITY",
        "coe_liquid_holding_capacity_v1",
        "INV-SNOWFREEZE-067",
        "OBL-SNOWFREEZE-P-042",
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL",
        "SNOWDENSITY-10.3.8 Opt-In Liquid Holding-Capacity Addendum",
        "Persistent state",
        "Coupled WAT acceptance",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = repo_text(PACKAGE);
    for marker in [
        "Liquid Holding-Capacity Drainage",
        "coe_liquid_holding_capacity_v1",
        "in-repo authority",
        "persistent retained-liquid",
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL",
        "Closure may be `complete` only if",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

fn low_density_capacity_inputs(model: SnowMeltModel) -> DirectActiveSnowPartitionInputs {
    let mut hourly = [DirectSnowHourlyForcing {
        cloud_fraction: 1.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    hourly[0] = DirectSnowHourlyForcing {
        radiation_mj_m2: 10.0,
        air_temperature_c: 5.0,
        cloud_fraction: 1.0,
        ..DirectSnowHourlyForcing::zero()
    };

    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.0,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.2,
        runtime_depth_m: 1.0,
        runtime_density_kg_m3: 200.0,
        runtime_settle_day_count: 4.0,
        liquid_water_retained_m: 0.0,
        tmax_c: 5.0,
        tmin_c: 5.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 0.0,
        dewpoint_c: 0.0,
        snow_melt_model: model,
        snow_density_model: SnowDensityModel::LegacyWepp,
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
fn opt_in_retains_liquid_to_capacity_and_routes_excess() {
    let legacy = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &low_density_capacity_inputs(SnowMeltModel::LegacyCoe),
    )
    .expect("legacy CoE melt should compute");
    let candidate = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &low_density_capacity_inputs(SnowMeltModel::CoeLiquidHoldingCapacityV1),
    )
    .expect("capacity candidate should compute");

    assert!(legacy.raw_melt_m > 0.0);
    assert!((legacy.raw_melt_m - candidate.raw_melt_m).abs() <= TOL);
    assert!(legacy.snowpack_swe_loss_m.abs() <= TOL);
    assert!(legacy.liquid_water_retained_after_m.abs() <= TOL);

    assert!(candidate.liquid_holding_capacity_after_m > 0.0);
    assert!(candidate.liquid_water_retained_after_m > 0.0);
    assert!(candidate.liquid_water_released_m > 0.0);
    assert!(
        candidate.liquid_water_retained_after_m <= candidate.liquid_holding_capacity_after_m + TOL
    );
    assert!(
        candidate.snowpack_swe_loss_m > 0.0 && candidate.snowpack_swe_loss_m < candidate.raw_melt_m,
        "capacity candidate must route only excess liquid, not all positive melt"
    );
    assert!((candidate.snowpack_swe_loss_m - candidate.liquid_water_released_m).abs() <= TOL);
    assert!(
        (candidate.raw_melt_m
            - candidate.snowpack_swe_loss_m
            - candidate.liquid_water_retained_after_m)
            .abs()
            <= TOL,
        "raw melt must split into retained liquid plus routed excess"
    );
}

#[test]
fn activation_preserves_rollback_identity_and_selector_isolation() {
    let builder = format!(
        "{}\n{}",
        repo_text(DIRECT_PUBLICATION_BUILDER),
        repo_text(DIRECT_PUBLICATION_SNOW_FROST_IMPL)
    );
    for marker in [
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL",
        "snowdensity1015_default_snow_melt_model",
        "SnowMeltModel::LegacyCoe",
        "SnowMeltModel::CoeLiquidHoldingCapacityV1",
        "must be legacy_coe, coe_liquid_holding_capacity_v1, coe_open_sublimation_stage_a_v1, or coe_open_sublimation_stage_b_v1",
        "liquid_water_retained_before_m",
        "liquid_water_retained_after_m",
    ] {
        assert_contains(&builder, marker, "direct publication snow/frost sources");
    }
}

#[test]
fn snowbench_selector_and_diagnostic_columns_are_bound() {
    let parsed = CoeMeltModel::parse("coe_liquid_holding_capacity_v1")
        .expect("capacity selector should parse");

    assert_eq!(parsed, CoeMeltModel::CoeLiquidHoldingCapacityV1);
    assert_eq!(parsed.name(), "coe_liquid_holding_capacity_v1");

    let snowbench = repo_text(COE_MELT_SNOWBENCH);
    for marker in [
        "liquid_holding_capacity_m",
        "liquid_water_retained_m",
        "liquid_water_released_m",
        "total_liquid_water_released_m",
        "final_liquid_water_retained_m",
    ] {
        assert_contains(&snowbench, marker, COE_MELT_SNOWBENCH);
    }
}
