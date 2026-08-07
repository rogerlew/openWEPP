use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, SnowAlbedoUpdateInputs,
    SnowDensityModel, SnowMeltModel, Wb11HydrologyKernel, update_snow_albedo_state,
};
use openwepp_runner::CoeMeltModel;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = "docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/package.md";
const TOOL: &str = "tools/snowfreeze_observed/winter_thaw_melt_response_correction.py";
const COUPLED_TOOL: &str = "tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py";
const REPORT_JSON: &str = "docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/artifacts/winter-thaw-melt-response-correction.json";
const REPORT_MD: &str = "docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/artifacts/winter-thaw-melt-response-correction.md";
const COUPLED_REPORT_JSON: &str = "docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/artifacts/coupled-wat-melt-response.json";
const DIRECT_PUBLICATION_BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00c_day_input_builder_impl.rs"
);
const DIRECT_PUBLICATION_SNOW_FROST_IMPL: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00a_snow_frost_authority_impl.rs"
);
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
fn contract_and_package_bind_thaw_state_loss_candidate() {
    let contract = repo_text(CONTRACT);
    for marker in [
        "contract_id: SC-SNOWFREEZE-001",
        "coe_winter_thaw_state_loss_v1",
        "INV-SNOWFREEZE-066",
        "OBL-SNOWFREEZE-P-041",
        "OPENWEPP_SNOWDENSITY1037_MELT_MODEL",
        "SNOWDENSITY-10.3.7 Opt-In Winter-Thaw State-Loss Addendum",
        "positive-thaw application branch",
        "paired Sleepers/Harvard event-window under-ablation",
        "must not require or consume albedo state",
        "Coupled WAT acceptance",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = repo_text(PACKAGE);
    for marker in [
        "Winter-Thaw Melt Response Correction",
        "Correction Authority Envelope",
        "coe_winter_thaw_state_loss_v1",
        "Default activation, parser/runfile/user CLI selectors",
        "Closure may be `complete` only if",
        "OPENWEPP_SNOWDENSITY1037_MELT_MODEL",
        "coupled WAT",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

fn low_density_positive_thaw_inputs(model: SnowMeltModel) -> DirectActiveSnowPartitionInputs {
    let mut hourly = [DirectSnowHourlyForcing {
        cloud_fraction: 1.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    hourly[0] = DirectSnowHourlyForcing {
        radiation_mj_m2: 10.0,
        air_temperature_c: 2.0,
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
        tmax_c: 2.0,
        tmin_c: 2.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 0.0,
        dewpoint_c: 0.0,
        snow_melt_model: model,
        snow_density_model: SnowDensityModel::LegacyWepp,
        stage3_liquid_routing_model:
            openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::Disabled,
        surface_energy_options:
            openwepp_hillslope_orchestrator::DirectSnowSurfaceEnergyOptions::default(),
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
fn opt_in_routes_positive_low_density_thaw_melt_to_state_loss() {
    let legacy = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &low_density_positive_thaw_inputs(SnowMeltModel::LegacyCoe),
    )
    .expect("legacy CoE melt should compute");
    let candidate = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &low_density_positive_thaw_inputs(SnowMeltModel::CoeWinterThawStateLossV1),
    )
    .expect("candidate CoE melt should compute");

    assert!(legacy.solid_to_liquid_ledger().raw_signed_melt_m > 0.0);
    assert!(
        (legacy.solid_to_liquid_ledger().raw_signed_melt_m
            - candidate.solid_to_liquid_ledger().raw_signed_melt_m)
            .abs()
            <= TOL
    );
    assert!(legacy.solid_to_liquid_ledger().snowpack_swe_loss_m.abs() <= TOL);
    assert!(legacy.solid_to_liquid_ledger().liquid_handoff_m.abs() <= TOL);
    assert!(
        legacy.runtime_density_after_kg_m3
            > low_density_positive_thaw_inputs(SnowMeltModel::LegacyCoe).runtime_density_kg_m3
    );

    assert!(
        (candidate.solid_to_liquid_ledger().snowpack_swe_loss_m
            - candidate.solid_to_liquid_ledger().raw_signed_melt_m)
            .abs()
            <= TOL
    );
    assert!(
        (candidate.solid_to_liquid_ledger().liquid_handoff_m
            - candidate.solid_to_liquid_ledger().raw_signed_melt_m)
            .abs()
            <= TOL
    );
    assert!(
        candidate.runtime_swe_after_m < legacy.runtime_swe_after_m,
        "candidate must realize positive thaw melt as SWE state loss"
    );
    assert!(
        candidate.runtime_depth_after_m <= legacy.runtime_depth_after_m + TOL,
        "candidate must not reduce less depth than the legacy density-only branch"
    );
    assert!(
        candidate.runtime_density_after_kg_m3 < 350.0,
        "candidate remains an explicit opt-in exception to the legacy density gate"
    );
    assert_eq!(candidate.snow_albedo_state_after, None);
}

#[test]
fn opt_in_state_loss_is_conservation_closed_and_routed() {
    let inputs = low_density_positive_thaw_inputs(SnowMeltModel::CoeWinterThawStateLossV1);
    let outcome = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect("candidate CoE melt should compute");

    let available_swe_m = inputs.runtime_swe_m
        + inputs
            .hourly
            .iter()
            .map(|hour| hour.snowfall_m * 0.1)
            .sum::<f64>()
        + outcome.rain_retained_m;
    assert!(
        outcome.solid_to_liquid_ledger().snowpack_swe_loss_m <= available_swe_m + TOL,
        "candidate must not release more SWE than available pack/input storage"
    );
    assert!(
        (available_swe_m
            - outcome.solid_to_liquid_ledger().snowpack_swe_loss_m
            - outcome.runtime_swe_after_m)
            .abs()
            <= TOL,
        "candidate SWE state must close exactly"
    );
    assert!(
        (outcome.solid_to_liquid_ledger().liquid_handoff_m
            - outcome.solid_to_liquid_ledger().rain_released_m
            - outcome.solid_to_liquid_ledger().snowpack_swe_loss_m)
            .abs()
            <= TOL,
        "SWE state loss must appear in routed liquid after separating released rain"
    );
}

#[test]
fn thaw_state_loss_candidate_does_not_require_albedo_state() {
    let outcome = update_snow_albedo_state(SnowAlbedoUpdateInputs {
        melt_model: SnowMeltModel::CoeWinterThawStateLossV1,
        albedo_model: None,
        previous_state: None,
        snow_water_equivalent_m: 0.2,
        fresh_snow_water_equivalent_m: 0.0,
        positive_temperature_c_day_increment: 1.0,
        underlying_surface_albedo: 0.2,
    })
    .expect("candidate must not require albedo state");

    assert!(!outcome.active);
    assert_eq!(outcome.state, None);
    assert_eq!(outcome.melt_model_id, "coe_winter_thaw_state_loss_v1");
}

#[test]
fn snowbench_selector_parses_candidate() {
    let parsed = CoeMeltModel::parse("coe_winter_thaw_state_loss_v1")
        .expect("candidate selector should parse");

    assert_eq!(parsed, CoeMeltModel::CoeWinterThawStateLossV1);
    assert_eq!(parsed.name(), "coe_winter_thaw_state_loss_v1");
}

#[test]
fn diagnostic_tool_and_report_record_improvement_gate() {
    let tool = repo_text(TOOL);
    for marker in [
        "legacy_coe",
        "coe_winter_thaw_state_loss_v1",
        "under_ablation_interval_count",
        "aggregate_depth_loss_deficit_m",
        "WINTER-THAW-MELT-RESPONSE-CANDIDATE-",
        "does not change production defaults",
    ] {
        assert_contains(&tool, marker, TOOL);
    }

    let report_json = repo_text(REPORT_JSON);
    for marker in [
        "snowdensity10-3-7-winter-thaw-melt-response-correction-v1",
        "legacy_coe",
        "coe_winter_thaw_state_loss_v1",
        "opt_in_vs_legacy",
        "default_activation_changed",
        "parser_runfile_user_cli_selector_added",
        "aggregate_depth_loss_deficit_m",
        "conservation_gate",
        "candidate_conservation_passed",
    ] {
        assert_contains(&report_json, marker, REPORT_JSON);
    }

    let report_md = repo_text(REPORT_MD);
    for marker in [
        "Evidence mode: Static/Ran.",
        "Default activation changed: `False`",
        "Rain heat and sub-canopy longwave remain out of scope",
        "Conservation Gate",
    ] {
        assert_contains(&report_md, marker, REPORT_MD);
    }
}

#[test]
fn direct_production_exposes_only_package_bound_melt_diagnostic_env() {
    let builder = format!(
        "{}\n{}",
        repo_text(DIRECT_PUBLICATION_BUILDER),
        repo_text(DIRECT_PUBLICATION_SNOW_FROST_IMPL)
    );
    for marker in [
        "OPENWEPP_SNOWDENSITY1037_MELT_MODEL",
        "snowdensity1037_diagnostic_snow_melt_model",
        "SnowMeltModel::LegacyCoe",
        "SnowMeltModel::CoeWinterThawStateLossV1",
        "\\\"snow_melt_model\\\":\\\"{}\\\"",
        "snow_melt_model: self.snow_melt_model",
        "must be legacy_coe or coe_winter_thaw_state_loss_v1",
    ] {
        assert_contains(&builder, marker, "direct publication snow/frost sources");
    }
}

#[test]
fn coupled_gate_tool_and_report_record_real_wat_evidence() {
    let tool = repo_text(COUPLED_TOOL);
    for marker in [
        "openwepp-cli-hill",
        "--direct-production-executor",
        "OPENWEPP_SNOWDENSITY1037_MELT_MODEL",
        "OPENWEPP_R7H_SNOW_TRACE_PATH",
        "coe_winter_thaw_state_loss_v1",
        "coupled_no_worse_gate_passed",
        "coupled-wat-melt-response.json",
    ] {
        assert_contains(&tool, marker, COUPLED_TOOL);
    }

    if !Path::new(COUPLED_REPORT_JSON).is_file() {
        return;
    }
    let report_json = repo_text(COUPLED_REPORT_JSON);
    for marker in [
        "snowdensity10-3-7-coupled-wat-melt-response-v1",
        "WINTER-THAW-COUPLED-WAT-",
        "coupled_no_worse_gate_passed",
        "candidate_trace_selected_count",
        "default_snow_control_fail_count",
        "candidate_snow_control_fail_count",
    ] {
        assert_contains(&report_json, marker, COUPLED_REPORT_JSON);
    }
}
