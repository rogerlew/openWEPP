use std::fs;
use std::path::Path;

use arrow_schema::DataType;
use openwepp_hillslope_output::hillslope_wat::{InterchangeVersion, hillslope_wat_schema};
use openwepp_sim_contract::units::validate_output_schema_unit;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = "docs/work-packages/20260629-paradigm-2-multilayer-promotion-001/package.md";
const DIRECT_PUBLICATION: &str =
    "crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs";
const RUNNER_WAT: &str = "crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs";
const RUNNER_BINS: &str = "crates/openwepp-runner/src/bin";
const HILLSLOPE_PASS_SCHEMA: &str = "crates/openwepp-hillslope-output/src/hillslope_pass.rs";

#[test]
fn promotion_contract_and_package_ratify_supported_opt_in_capability() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 112",
        "REF-SNOWFREEZE-PARADIGM2-PROMOTION",
        "INV-SNOWFREEZE-082",
        "OBL-SNOWFREEZE-P-057",
        "production-supported opt-in water-temperature capability",
        "MeltwaterTemperature",
        "HBP binary/watershed serialization and full in-stream routing are not authorized",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "PARADIGM-2 Multilayer Promotion",
        "Activation posture",
        "Selector exposure",
        "Production-supported internal selector",
        "WAT parquet",
        "HBP/watershed serialization remains deferred",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn meltwater_temperature_is_a_supported_nullable_wat_output() {
    let schema = hillslope_wat_schema(InterchangeVersion::default())
        .expect("hillslope WAT schema should build");
    let field = schema
        .fields()
        .iter()
        .find(|field| field.name() == "MeltwaterTemperature")
        .expect("MeltwaterTemperature field should be present");

    assert_eq!(field.data_type(), &DataType::Float64);
    assert!(field.is_nullable());
    assert_eq!(
        field.metadata().get("units").map(String::as_str),
        Some("degC")
    );
    assert!(
        field
            .metadata()
            .get("description")
            .is_some_and(|value| value.contains("meltwater flux temperature source")),
        "MeltwaterTemperature should document the supported source"
    );
    assert_eq!(
        validate_output_schema_unit("hillslope_wat", "MeltwaterTemperature", "degC")
            .expect("unit registry should cover MeltwaterTemperature"),
        "degC"
    );
}

#[test]
fn direct_publication_consumes_stage3_diagnostics_and_default_path_is_null() {
    let publication = read(DIRECT_PUBLICATION);
    for marker in [
        "DirectPublicationWaterTemperatureOperands",
        "snow_coupling_shadow_projection",
        "stage3_diagnostics",
        "meltwater_temperature_c",
        "publication.water_temperature.meltwater_temperature_c",
    ] {
        assert_contains(&publication, marker, DIRECT_PUBLICATION);
    }

    let runner_wat = read(RUNNER_WAT);
    assert_contains(
        &runner_wat,
        "meltwater_temperature: row.water_temperature.meltwater_temperature_c",
        RUNNER_WAT,
    );
    assert_contains(&runner_wat, "meltwater_temperature: None", RUNNER_WAT);
}

#[test]
fn selector_remains_internal_and_hbp_temperature_serialization_is_deferred() {
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
            !text.contains("layered_thermal_liquid_v1"),
            "water-temperature selector must remain internal until stream-temperature wiring: {}",
            path.display()
        );
    }

    let hbp_schema = read(HILLSLOPE_PASS_SCHEMA);
    assert!(
        !hbp_schema.contains("MeltwaterTemperature")
            && !hbp_schema.contains("meltwater_temperature"),
        "promotion package must not add HBP/watershed temperature serialization"
    );
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
