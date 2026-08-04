use std::fs;
use std::path::Path;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const BUILDER: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs";
const SNOW_FROST_IMPL: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs";
const RUNNER_BINS: &str = "crates/openwepp-runner/src/bin";
const PACKAGE: &str =
    "docs/work-packages/20260628-paradigm-2-stage-2-snow-frost-insulation-profile-001/package.md";
const TOOL: &str = "tools/snowfreeze_observed/paradigm2_stage2_insulation_profile.py";

#[test]
fn stage2_contract_package_and_trace_are_bound() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 123",
        "REF-SNOWFREEZE-PARADIGM2-STAGE2",
        "INV-SNOWFREEZE-079",
        "OBL-SNOWFREEZE-P-054",
        "snow_frost_insulation_model",
        "snow_layer_insulation_resistance",
        "snow_frost_effective_density",
        "layered_resistance_v1",
        "Sturm et al. 1997",
        "frost-primary",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "PARADIGM-2 Stage 2",
        "OPENWEPP_SNOWFROST_STAGE2_INSULATION_MODEL=layered_resistance_v1",
        "Status: `HOLD-GATE-FAILURE-NON-PROMOTION`",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let tool = read(TOOL);
    assert_contains(
        &tool,
        "paradigm2-stage2-snow-frost-insulation-profile-v1",
        TOOL,
    );
    assert_contains(&tool, "snow_layer_density_gradient_after_kg_m3", TOOL);
    assert_contains(&tool, "physics_bulk_multilayer_density_v1", TOOL);

    let builder = format!("{}\n{}", read(BUILDER), read(SNOW_FROST_IMPL));
    assert_contains(
        &builder,
        "OPENWEPP_SNOWFROST_STAGE2_INSULATION_MODEL",
        BUILDER,
    );
    for marker in [
        "snow_layer_surface_density_before_kg_m3",
        "snow_layer_basal_density_before_kg_m3",
        "snow_layer_density_gradient_before_kg_m3",
        "snow_layer_surface_density_after_kg_m3",
        "snow_layer_basal_density_after_kg_m3",
        "snow_layer_density_gradient_after_kg_m3",
    ] {
        assert_contains(&builder, marker, BUILDER);
    }

    let snow_frost_impl = read(SNOW_FROST_IMPL);
    for marker in [
        "SNOWFROST_STAGE2_INSULATION_MODEL_ENV",
        "bulk_depth_density",
        "layered_resistance_v1",
        "layered_snow_frost_insulation_depth_density",
        "sturm1997_snow_conductivity_w_m_k",
    ] {
        assert_contains(&snow_frost_impl, marker, SNOW_FROST_IMPL);
    }
}

#[test]
fn stage2_internal_selector_not_user_cli_exposed() {
    assert_runner_bins_do_not_expose_selector();
}

fn assert_runner_bins_do_not_expose_selector() {
    for entry in fs::read_dir(RUNNER_BINS)
        .unwrap_or_else(|err| panic!("failed to read runner binary directory: {err}"))
    {
        let path = entry
            .unwrap_or_else(|err| panic!("failed to read runner binary entry: {err}"))
            .path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
            continue;
        }
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        assert!(
            !text.contains("layered_resistance_v1"),
            "Stage 2 selector must remain package-bound, not CLI/user-facing: {}",
            path.display()
        );
    }
}

fn read(path: &str) -> String {
    fs::read_to_string(Path::new(path)).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "expected {path} to contain marker: {marker}"
    );
}
