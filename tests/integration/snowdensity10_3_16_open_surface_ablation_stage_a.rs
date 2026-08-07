use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, SnowDensityModel, SnowMeltModel,
    Wb11HydrologyKernel,
};
use openwepp_runner::CoeMeltModel;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = concat!(
    "docs/work-packages/20260627-snowdensity-10-3-16-open-surface-ablation-stage-a-001/",
    "package.md"
);
const BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00c_day_input_builder_impl.rs"
);
const SNOWBENCH: &str = "crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs";
const TOL: f64 = 1.0e-12;

fn repo_text(relative_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "{path} missing required marker: {marker}"
    );
}

#[test]
fn contract_and_package_bind_stage_a_without_activation() {
    let contract = repo_text(CONTRACT);
    for marker in [
        "contract_id: SC-SNOWFREEZE-001",
        "REF-SNOWFREEZE-MARKS1998-TURBULENT",
        "REF-SNOWFREEZE-MARKS1999-SUBLIMATION",
        "snow_sublimation",
        "coe_open_sublimation_stage_a_v1",
        "INV-SNOWFREEZE-073",
        "OBL-SNOWFREEZE-P-048",
        "SNOWDENSITY-10.3.16 Open-Surface Ablation Stage A Addendum",
        "must close `HOLD` or non-promotion and must not activate",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = repo_text(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.16",
        "Stage A only",
        "do not read PySnobal/libsnobal C",
        "no two-layer surface structure",
        "If any gate is missing or worse, close HOLD/non-promotion",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

fn dry_windy_open_inputs(model: SnowMeltModel) -> DirectActiveSnowPartitionInputs {
    let mut hourly = [DirectSnowHourlyForcing {
        cloud_fraction: 0.25,
        air_temperature_c: -2.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    for hour in &mut hourly {
        hour.radiation_mj_m2 = 0.2;
        hour.air_temperature_c = -2.0;
        hour.cloud_fraction = 0.25;
    }

    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.0,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.45,
        runtime_depth_m: 1.0,
        runtime_density_kg_m3: 450.0,
        runtime_settle_day_count: 30.0,
        liquid_water_retained_m: 0.0,
        tmax_c: -2.0,
        tmin_c: -2.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 6.0,
        dewpoint_c: -18.0,
        snow_melt_model: model,
        snow_density_model: SnowDensityModel::LegacyWepp,
        stage3_liquid_routing_model:
            openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::Disabled,
        surface_energy_options:
            openwepp_hillslope_orchestrator::DirectSnowSurfaceEnergyOptions::default(),
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 1.0,
        coe_boundary_density_kg_m3: 450.0,
        coe_boundary_settle_day_count: 30.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: Vec::new(),
        underlying_surface_albedo: 0.2,
        hourly,
    }
}

#[test]
fn stage_a_exports_vapor_without_routing_it_as_liquid() {
    let base = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &dry_windy_open_inputs(SnowMeltModel::CoeLiquidHoldingCapacityV1),
    )
    .expect("activated capacity model should compute");
    let candidate = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &dry_windy_open_inputs(SnowMeltModel::CoeOpenSublimationStageAV1),
    )
    .expect("Stage A sublimation candidate should compute");

    assert!(candidate.sublimation_m > 0.0);
    assert!(
        (candidate.solid_to_liquid_ledger().raw_signed_melt_m
            - base.solid_to_liquid_ledger().raw_signed_melt_m)
            .abs()
            <= TOL
    );
    assert!(
        (candidate
            .solid_to_liquid_ledger()
            .redistributed_positive_melt_m
            - base.solid_to_liquid_ledger().redistributed_positive_melt_m)
            .abs()
            <= TOL
    );
    assert!(
        (candidate.solid_to_liquid_ledger().liquid_handoff_m
            - base.solid_to_liquid_ledger().liquid_handoff_m)
            .abs()
            <= TOL
    );
    assert!((candidate.post_winter_rain_m - base.post_winter_rain_m).abs() <= TOL);
    assert!(
        (candidate.solid_to_liquid_ledger().snowpack_swe_loss_m
            - base.solid_to_liquid_ledger().snowpack_swe_loss_m)
            .abs()
            <= TOL
    );
    assert!(candidate.runtime_swe_after_m < base.runtime_swe_after_m);
    assert!(candidate.runtime_depth_after_m < base.runtime_depth_after_m);

    let available_swe_m = dry_windy_open_inputs(SnowMeltModel::CoeOpenSublimationStageAV1)
        .runtime_swe_m
        + candidate.rain_retained_m;
    let closure = available_swe_m
        - candidate.solid_to_liquid_ledger().snowpack_swe_loss_m
        - candidate.sublimation_m
        - candidate.runtime_swe_after_m;
    assert!(
        closure.abs() <= TOL,
        "snow-state closure must include vapor export, got {closure}"
    );
}

#[test]
fn selector_and_trace_are_opt_in_only() {
    let builder = repo_text(BUILDER);
    for marker in [
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL",
        "SnowMeltModel::CoeLiquidHoldingCapacityV1",
        "SnowMeltModel::CoeOpenSublimationStageAV1",
        "coe_open_sublimation_stage_a_v1",
        "must be legacy_coe, coe_liquid_holding_capacity_v1, coe_open_sublimation_stage_a_v1, or coe_open_sublimation_stage_b_v1",
        "\\\"sublimation_m\\\":{}",
    ] {
        assert_contains(&builder, marker, BUILDER);
    }
    assert_contains(
        &builder,
        "Err(std::env::VarError::NotPresent)",
        "absent selector must remain explicitly handled",
    );
    assert!(
        !builder.contains("snow-melt-model"),
        "Stage A must not add parser/runfile/user CLI selector surface"
    );

    let parsed = CoeMeltModel::parse("coe_open_sublimation_stage_a_v1")
        .expect("snowbench selector should parse");
    assert_eq!(parsed, CoeMeltModel::CoeOpenSublimationStageAV1);
    assert_eq!(parsed.name(), "coe_open_sublimation_stage_a_v1");

    let snowbench = repo_text(SNOWBENCH);
    for marker in [
        "INV-SNOWFREEZE-073",
        "total_sublimation_m",
        "sublimation_m",
        "- partition.sublimation_m",
    ] {
        assert_contains(&snowbench, marker, SNOWBENCH);
    }
}
