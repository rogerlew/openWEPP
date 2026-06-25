use std::fs;
use std::path::PathBuf;

use openwepp_runner::{PYSNOBAL_FORCING_COLUMNS, SnowbenchExportRequest, export_pysnobal_inputs};
use serde_json::Value;

#[test]
fn g0_exporter_emits_pysnobal_schema_and_required_anti_alias_lineage() {
    let output_dir = PathBuf::from("target/snowfrost_fidelity_g0_contract/site1");
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).expect("test output cleanup should succeed");
    }

    let report = export_pysnobal_inputs(&SnowbenchExportRequest {
        run_dir: PathBuf::from("tests/fixtures/snowfreeze_observed/site1_sleepers_south_field_vt"),
        run_file: None,
        output_dir: output_dir.clone(),
    })
    .expect("G0 PySnobal export should succeed for site1 fixture");

    assert_eq!(report.lane_count, 3);
    assert!(report.hourly_row_count > 24);
    assert!(report.total_snow_precip_mass_mm > 0.0);

    let lane_dir = output_dir.join("tg_neg2p5c_zg0p10m");
    assert_forcing_schema(&lane_dir);
    assert_config(&lane_dir);
    assert_lineage(&lane_dir);
    assert_audit(&lane_dir);
    assert_precipitation_reconstructs_audit(&lane_dir);
}

fn assert_forcing_schema(lane_dir: &std::path::Path) {
    let forcing =
        fs::read_to_string(lane_dir.join("forcing.csv")).expect("forcing csv should be present");
    let mut lines = forcing.lines();
    let header = lines.next().expect("forcing csv should have a header");
    assert_eq!(
        header,
        format!("Datetime,{}", PYSNOBAL_FORCING_COLUMNS.join(","))
    );
    assert!(
        lines.all(|line| {
            let values = line.split(',').collect::<Vec<_>>();
            values.len() == PYSNOBAL_FORCING_COLUMNS.len() + 1
                && values
                    .iter()
                    .skip(1)
                    .all(|value| value.parse::<f64>().map(f64::is_finite).unwrap_or(false))
        }),
        "forcing rows must be complete finite numeric PySnobal inputs"
    );
}

fn assert_config(lane_dir: &std::path::Path) {
    let config =
        fs::read_to_string(lane_dir.join("config.yaml")).expect("config should be present");
    assert!(config.contains("soil_temp_m: 0.10"));
    assert!(config.contains("roughness_length_m: 0.0050"));
}

fn assert_lineage(lane_dir: &std::path::Path) {
    let lineage: Value = serde_json::from_str(
        &fs::read_to_string(lane_dir.join("lineage.json")).expect("lineage json should be present"),
    )
    .expect("lineage should parse");
    let fields = lineage["fields"]
        .as_object()
        .expect("fields should be an object");
    for column in PYSNOBAL_FORCING_COLUMNS {
        assert!(fields.contains_key(column), "missing lineage for {column}");
        assert!(
            matches!(
                fields[column]["source_class"].as_str(),
                Some("mechanical" | "deterministic-derived" | "diagnostic-proxy")
            ),
            "missing or invalid source class for {column}"
        );
        assert!(
            fields[column]["rejected_aliases"]
                .as_array()
                .is_some_and(|aliases| !aliases.is_empty()),
            "missing rejected alias evidence for {column}"
        );
    }
    assert!(
        fields["precip_mass_mm"]["conversion"]
            .as_str()
            .expect("precip conversion should be a string")
            .contains("snowfall_depth_m * snow_density_kg_m3"),
        "snowfall depth must be converted to mass before PySnobal export"
    );
    assert!(
        fields["temp_ground_degC"]["rejected_aliases"]
            .as_array()
            .expect("ground temp aliases should be an array")
            .iter()
            .any(|value| value.as_str() == Some("frost.hourly.surface_temp_c_####")),
        "frost surface temperature must be rejected as PySnobal ground temperature"
    );
    assert!(
        fields["net_solar_Wm-2"]["rejected_aliases"]
            .as_array()
            .expect("net solar aliases should be an array")
            .iter()
            .any(|value| value.as_str() == Some("raw daily climate rad in langleys/day")),
        "daily langleys radiation must not alias hourly W m^-2 forcing"
    );
    assert!(
        fields["snow_precip_density_kgm-3"]["rejected_aliases"]
            .as_array()
            .expect("snow density aliases should be an array")
            .iter()
            .any(|value| value.as_str() == Some("WAT Snow-Water")),
        "WAT Snow-Water must not alias snow density or physical snow depth"
    );
}

fn assert_audit(lane_dir: &std::path::Path) {
    let audit: Value = serde_json::from_str(
        &fs::read_to_string(lane_dir.join("audit.json")).expect("audit json should be present"),
    )
    .expect("audit should parse");
    assert_eq!(audit["wat_snow_water_is_not_depth"].as_bool(), Some(true));
    assert_eq!(
        audit["frost_surface_temp_is_not_ground_temp"].as_bool(),
        Some(true)
    );
    assert_eq!(
        audit["daily_radiation_not_exported_as_hourly_wm2"].as_bool(),
        Some(true)
    );
    assert_eq!(audit["uniform_hourly_timestamps"].as_bool(), Some(true));
}

fn assert_precipitation_reconstructs_audit(lane_dir: &std::path::Path) {
    let forcing =
        fs::read_to_string(lane_dir.join("forcing.csv")).expect("forcing csv should be present");
    let mut lines = forcing.lines();
    let header = lines.next().expect("forcing csv should have a header");
    let columns = header.split(',').collect::<Vec<_>>();
    let precip_index = columns
        .iter()
        .position(|column| *column == "precip_mass_mm")
        .expect("precip_mass_mm column should exist");
    let snow_fraction_index = columns
        .iter()
        .position(|column| *column == "snow_precip_fraction")
        .expect("snow_precip_fraction column should exist");
    let mut total_precip = 0.0;
    let mut total_snow_precip = 0.0;
    for line in lines {
        let values = line.split(',').collect::<Vec<_>>();
        let precip = values[precip_index]
            .parse::<f64>()
            .expect("precipitation mass should parse");
        let snow_fraction = values[snow_fraction_index]
            .parse::<f64>()
            .expect("snow precipitation fraction should parse");
        total_precip += precip;
        total_snow_precip += precip * snow_fraction;
    }

    let audit: Value = serde_json::from_str(
        &fs::read_to_string(lane_dir.join("audit.json")).expect("audit json should be present"),
    )
    .expect("audit should parse");
    assert_close(
        total_precip,
        audit["total_precip_mass_mm"]
            .as_f64()
            .expect("audit precip total should be numeric"),
    );
    assert_close(
        total_snow_precip,
        audit["total_snow_precip_mass_mm"]
            .as_f64()
            .expect("audit snow precip total should be numeric"),
    );
}

fn assert_close(actual: f64, expected: f64) {
    let tolerance = 1.0e-6_f64.max(expected.abs() * 1.0e-10);
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {actual} to be within {tolerance} of {expected}"
    );
}
