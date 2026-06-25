use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

const OBSERVATION_ROOT: &str = "tests/fixtures/snowfreeze_observed/observations";
const HARNESS_SOURCE: &str = include_str!("../../tools/snowfreeze_observed/observed_harness.py");
const PACKAGE: &str = include_str!(
    "../../docs/work-packages/20260624-snowfreeze-observed-frost-depth-harness-001/package.md"
);

#[test]
fn snowfreeze_observed_manifest_binds_external_authority_and_source_statuses() {
    let manifest = read_manifest();

    assert_eq!(
        manifest.get("schema").and_then(Value::as_str),
        Some("snowfreeze-observed-manifest-v1")
    );
    assert_eq!(
        manifest.get("measurement_contract").and_then(Value::as_str),
        Some("SC-SNOWFREEZE-001 INV-SNOWFREEZE-047")
    );
    assert_eq!(
        manifest
            .pointer("/snow_depth_control/modeled_status")
            .and_then(Value::as_str),
        Some("UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC"),
        "WAT Snow-Water must not be promoted into a snow-depth control"
    );

    let sites = manifest
        .get("sites")
        .and_then(Value::as_array)
        .expect("manifest sites should be an array");
    assert_eq!(sites.len(), 5);

    assert_site(&manifest, "site1_sleepers_south_field_vt", "acquired", 300);
    assert_site(&manifest, "site2_sleepers_w9_hardwood_vt", "acquired", 150);
    assert_site(&manifest, "site3_scan_mandan_nd", "acquired", 10_000);
    assert_site(&manifest, "site4_ggd498_morris_mn", "acquired", 200);
    assert_site(
        &manifest,
        "site5_reynolds_creek_us_rls_id",
        "acquired",
        4_000,
    );

    for source in manifest
        .get("sources")
        .and_then(Value::as_array)
        .expect("manifest sources should be an array")
    {
        let source_id = source
            .get("source_id")
            .and_then(Value::as_str)
            .expect("source_id should be present");
        let provenance_path = observation_root()
            .join("provenance")
            .join(format!("{source_id}.json"));
        assert!(
            provenance_path.is_file(),
            "provenance file missing for source {source_id}: {}",
            provenance_path.display()
        );
    }
}

#[test]
fn snowfreeze_observed_csv_schema_preserves_measurement_correspondence() {
    let site1_rows = read_csv_records("site1_sleepers_south_field_vt");
    assert!(
        site1_rows.iter().any(|row| {
            field(row, "method") == "frost_tube"
                && !field(row, "observed_frost_depth_m").is_empty()
                && !field(row, "observed_snow_depth_m").is_empty()
        }),
        "Sleepers frost-tube site must expose paired frost depth and snow depth rows"
    );

    let site3_rows = read_csv_records("site3_scan_mandan_nd");
    assert!(
        site3_rows.iter().take(100).all(|row| {
            field(row, "method") == "soil_temperature_zero_c_isotherm"
                && field(row, "observed_frost_depth_m").is_empty()
                && !field(row, "observed_isotherm_depth_m").is_empty()
        }),
        "SCAN soil-temperature rows must remain timing/upper-bound isotherm rows, not magnitude frost-depth targets"
    );

    let site5_rows = read_csv_records("site5_reynolds_creek_us_rls_id");
    assert!(
        site5_rows.iter().take(100).all(|row| {
            field(row, "method") == "soil_temperature_zero_c_isotherm"
                && field(row, "observed_frost_depth_m").is_empty()
                && !field(row, "observed_isotherm_depth_m").is_empty()
        }),
        "Reynolds rows must remain timing/upper-bound isotherm rows"
    );
}

#[test]
fn snowfreeze_observed_harness_documents_no_defect_without_snow_depth_control() {
    for expected in [
        "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047",
        "UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC",
        "WAT Snow-Water is SWE and is not a snow-depth diagnostic.",
        "direct-production-executor",
        "isotherm_upper_bound_count",
        "censored_excluded_count",
    ] {
        assert!(
            HARNESS_SOURCE.contains(expected) || PACKAGE.contains(expected),
            "harness/package must retain external-observation posture marker {expected}"
        );
    }
}

fn read_manifest() -> Value {
    let path = observation_root().join("manifest.json");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("manifest should be readable: {error}"));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("manifest should parse: {error}"))
}

fn assert_site(manifest: &Value, site_id: &str, expected_status: &str, minimum_rows: u64) {
    let site = manifest
        .get("sites")
        .and_then(Value::as_array)
        .and_then(|sites| {
            sites
                .iter()
                .find(|site| site.get("site_id").and_then(Value::as_str) == Some(site_id))
        })
        .unwrap_or_else(|| panic!("manifest missing site {site_id}"));
    assert_eq!(
        site.get("status").and_then(Value::as_str),
        Some(expected_status)
    );
    let row_count = site
        .get("normalized_row_count")
        .and_then(Value::as_u64)
        .expect("site normalized_row_count should be numeric");
    assert!(
        row_count >= minimum_rows,
        "site {site_id} row_count {row_count} below expected floor {minimum_rows}"
    );
    let observation_file = site
        .get("observation_file")
        .and_then(Value::as_str)
        .expect("site observation_file should be present");
    assert!(
        observation_root().join(observation_file).is_file(),
        "site {site_id} observation file is missing"
    );
}

fn read_csv_records(site_id: &str) -> Vec<Vec<String>> {
    let path = observation_root()
        .join("sites")
        .join(format!("{site_id}.csv"));
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("CSV should be readable at {}: {error}", path.display()));
    let mut lines = text.lines();
    let header = lines.next().expect("CSV should include header");
    assert_eq!(parse_csv_line(header), expected_columns());
    lines.map(parse_csv_line).collect()
}

fn field<'a>(row: &'a [String], name: &str) -> &'a str {
    let index = expected_columns()
        .iter()
        .position(|column| column == name)
        .unwrap_or_else(|| panic!("unknown expected column {name}"));
    row.get(index).map_or_else(
        || panic!("row missing column {name}: {row:?}"),
        String::as_str,
    )
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                current.push('"');
                let _ = chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(std::mem::take(&mut current));
            }
            _ => current.push(ch),
        }
    }
    fields.push(current);
    fields
}

fn expected_columns() -> Vec<String> {
    [
        "site_id",
        "source_id",
        "date",
        "water_year",
        "method",
        "authority_role",
        "observed_frost_depth_m",
        "observed_isotherm_depth_m",
        "observed_snow_depth_m",
        "censoring",
        "quality_flag",
        "source_record_id",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn observation_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(OBSERVATION_ROOT)
}
