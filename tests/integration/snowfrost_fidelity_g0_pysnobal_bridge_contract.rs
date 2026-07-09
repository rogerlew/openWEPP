use std::fs;
use std::path::PathBuf;

use openwepp_hillslope_output::hillslope_wat::{
    HillslopeWatRow, InterchangeVersion, write_hillslope_wat_parquet,
};
use openwepp_runner::{
    PYSNOBAL_FORCING_COLUMNS, SnowbenchExportRequest, export_openwepp_snow_csv_from_wat,
    export_pysnobal_inputs,
};
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
        include_openwepp_snow_projection: false,
    })
    .expect("G0 PySnobal export should succeed for site1 fixture");

    assert_eq!(report.lane_count, 3);
    assert!(report.hourly_row_count > 24);
    assert!(report.total_snow_precip_mass_mm > 0.0);
    assert_eq!(
        report.canopy_source,
        "direct_production_day_input.growth_state_for_publication.cancov"
    );
    assert_eq!(report.canopy_series_summary.day_count, report.day_count);
    assert!(
        (0.0..=1.0).contains(&report.canopy_series_summary.min),
        "canopy min must be bounded"
    );
    assert!(
        (0.0..=1.0).contains(&report.canopy_series_summary.max),
        "canopy max must be bounded"
    );
    let canopy_series = fs::read_to_string(output_dir.join("canopy_series.csv"))
        .expect("canopy_series.csv should be emitted");
    assert!(canopy_series.starts_with("date,day_index,canopy_cover_fraction,source\n"));
    assert_eq!(
        canopy_series
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count()
            - 1,
        report.day_count
    );

    let lane_dir = output_dir.join("tg_neg2p5c_zg0p10m");
    assert_forcing_schema(&lane_dir);
    assert_config(&lane_dir);
    assert_lineage(&lane_dir);
    assert_audit(&lane_dir);
    assert_precipitation_reconstructs_audit(&lane_dir);
}

#[test]
fn g1_openwepp_snow_projection_extracts_wat_swe_and_physical_depth() {
    let output_dir = PathBuf::from("target/snowfrost_fidelity_g1_contract/openwepp_snow");
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).expect("test output cleanup should succeed");
    }
    fs::create_dir_all(&output_dir).expect("test output directory should be created");
    let wat_path = output_dir.join("sample.wat.parquet");
    write_hillslope_wat_parquet(
        &wat_path,
        &[sample_wat_row()],
        InterchangeVersion::default(),
    )
    .expect("sample WAT parquet should write");

    let rows = export_openwepp_snow_csv_from_wat(&wat_path, &output_dir)
        .expect("openwepp snow projection should read sample WAT");

    assert_eq!(rows, 1);
    assert_openwepp_snow_comparison_rows(&output_dir);
}

fn sample_wat_row() -> HillslopeWatRow {
    HillslopeWatRow {
        wepp_id: 1,
        ofe_id: 1,
        year: 1980,
        sim_day_index: 1,
        julian: 1,
        month: 1,
        day_of_month: 1,
        water_year: 1980,
        ofe: 1,
        p: 0.0,
        rm: 0.0,
        q: 0.0,
        ep: 0.0,
        es: 0.0,
        er: 0.0,
        dp: 0.0,
        up_strm_q: 0.0,
        sub_r_in: 0.0,
        latqcc: 0.0,
        base: Some(0.0),
        total_soil_water: 0.0,
        frozwt: 0.0,
        frdp: 0.0,
        snow_water: 42.0,
        snow_depth: Some(210.0),
        meltwater_temperature: None,
        qofe: 0.0,
        tile: 0.0,
        irr: 0.0,
        area: 1.0,
        soil_water_total: Some(0.0),
        profile_depth: Some(0.0),
        profile_porosity_cap: Some(0.0),
        profile_fc_store: Some(0.0),
        profile_wp_store: Some(0.0),
        interception: Some(0.0),
        interception_storage: None,
    }
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

fn assert_openwepp_snow_comparison_rows(output_dir: &std::path::Path) {
    let snow_csv = fs::read_to_string(output_dir.join("openwepp_snow.csv"))
        .expect("openwepp snow comparison csv should be present");
    let mut lines = snow_csv.lines();
    assert_eq!(
        lines.next(),
        Some("date,sim_day_index,Snow-Water_mm,Snow-Depth_mm,source")
    );
    let rows = lines.collect::<Vec<_>>();
    assert!(
        rows.iter()
            .any(|row| row.contains(",openwepp_compatibility_wat")),
        "openwepp snow csv should contain compatibility WAT rows"
    );
    assert!(
        rows.iter()
            .any(|row| row.contains(",42.000000000000,210.000000000000,")),
        "openwepp snow csv should preserve Snow-Water and Snow-Depth millimeter values"
    );

    let availability: Value = serde_json::from_str(
        &fs::read_to_string(output_dir.join("openwepp_snow_availability.json"))
            .expect("openwepp snow availability json should be present"),
    )
    .expect("openwepp snow availability should parse");
    assert_eq!(
        availability["status"].as_str(),
        Some("EXPORTED_FROM_COMPATIBILITY_WAT")
    );
    assert!(
        availability["row_count"]
            .as_u64()
            .is_some_and(|count| count > 0),
        "openwepp snow availability should record exported row count"
    );
}

fn assert_close(actual: f64, expected: f64) {
    let tolerance = 1.0e-6_f64.max(expected.abs() * 1.0e-10);
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {actual} to be within {tolerance} of {expected}"
    );
}
