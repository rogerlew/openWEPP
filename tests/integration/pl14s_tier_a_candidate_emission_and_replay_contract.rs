use std::env;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use openwepp_comparator_metadata::{
    COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID, ComparatorConfidenceTier,
    ComparatorSurfaceClass, ComparatorTierRoutingRequest, route_comparator_tier_metadata,
};
use openwepp_runner::{HillslopeRunRequest, SidecarPolicy, execute_hillslope_run};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};

const PL14S_SEMANTIC_COMPARATOR_SCRIPT: &str = include_str!("../../tools/owcmp/semantic_wat.py");
const PL14S_REPLAY_SUITE_SCRIPT: &str = include_str!("../../tools/owcmp/pl14s_suite.py");
const PL14S_SUITE_README: &str = include_str!("../../tools/owcmp/README.md");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StrictLaneMode {
    Required,
    StrictEquivalentRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CandidateSourceClass {
    NativeRuntimeDat,
    ConversionDerivedDat,
    NativeRuntimeParquet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConversionSourceRowConsistencyVerdict {
    Ready,
    Hold,
}

fn strict_lane_mode(candidate_extension: &str) -> StrictLaneMode {
    if candidate_extension.eq_ignore_ascii_case(".dat") {
        StrictLaneMode::Required
    } else {
        StrictLaneMode::StrictEquivalentRequired
    }
}

fn candidate_source_class(
    candidate_extension: &str,
    conversion_derived_dat: bool,
) -> CandidateSourceClass {
    if candidate_extension.eq_ignore_ascii_case(".dat") {
        if conversion_derived_dat {
            CandidateSourceClass::ConversionDerivedDat
        } else {
            CandidateSourceClass::NativeRuntimeDat
        }
    } else {
        CandidateSourceClass::NativeRuntimeParquet
    }
}

fn conversion_derived_dat_row_consistency_verdict(
    common_row_count: usize,
    only_baseline_count: usize,
    only_candidate_count: usize,
) -> ConversionSourceRowConsistencyVerdict {
    if common_row_count == 0 || only_baseline_count > 0 || only_candidate_count > 0 {
        return ConversionSourceRowConsistencyVerdict::Hold;
    }
    ConversionSourceRowConsistencyVerdict::Ready
}

fn contains_all(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().all(|needle| haystack.contains(needle))
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn fixture_temp_dir(prefix: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    env::temp_dir().join(format!("{prefix}_{stamp}"))
}

fn dat_row(ofe: i32, julian: i32, year: i32, seed: i32) -> String {
    let tail = (0_i32..17_i32)
        .map(|offset| (seed + offset).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    format!("{ofe} {julian} {year} {tail}")
}

#[test]
fn pl14s_contract_conformance_routes_single_ofe_daily_lane_to_higher_confidence() {
    let metadata = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
        Some(1),
    ))
    .expect("single OFE replay lane should route");

    assert_eq!(
        metadata.surface_class,
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance
    );
    assert_eq!(
        metadata.confidence_tier,
        ComparatorConfidenceTier::HigherConfidence
    );
    assert_eq!(
        metadata.message_id,
        COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID
    );
}

#[test]
fn pl14s_contract_conformance_declares_semantic_report_and_provenance_schema_markers() {
    assert!(contains_all(
        PL14S_SEMANTIC_COMPARATOR_SCRIPT,
        &[
            "REPORT_SCHEMA_VERSION = \"pl14s-semantic-wat-v2\"",
            "duplicate row key",
            "\"Total-Soil\": \"Total-Soil\"",
            "\"Total-Soil Water\": \"Total-Soil\"",
            "baseline_only_columns",
            "candidate_only_columns",
            "investigation_columns_used",
            "candidate_column_alias_sources",
            "row_key_fields",
            "--candidate-partition-value",
            "--candidate-partition-column",
            "--candidate-year-offset",
        ]
    ));
    assert!(contains_all(
        PL14S_REPLAY_SUITE_SCRIPT,
        &[
            "\"suite_schema_version\": \"pl14s-legacy-suite-v2\"",
            "--candidate-surface-source-class",
            "--candidate-partition-value",
            "--candidate-partition-column",
            "--candidate-year-offset",
            "--baseline-year-policy",
            "--expected-common-row-count",
            "\"strict_lane_policy\"",
            "\"strict-equivalent-required\"",
            "\"native-runtime-dat\"",
            "\"conversion-derived-dat\"",
            "\"native-runtime-parquet\"",
            "\"common_row_count\"",
            "\"candidate_partition_value\"",
            "\"candidate_partition_column\"",
            "\"candidate_year_offset\"",
            "\"baseline_year_policy\"",
            "\"expected_common_row_count\"",
            "\"full_span_policy_ready\"",
            "\"conversion_source_row_consistency_ready\"",
            "\"conversion_source_row_consistency_blockers\"",
            "semantic_summary = load_semantic_summary",
        ]
    ));
    assert!(contains_all(
        PL14S_SUITE_README,
        &[
            "Strict comparator is required when candidate input is `.dat`",
            "`--candidate-surface-source-class`",
            "strict-equivalent-required",
            "conversion-derived-dat",
            "row-presence deltas",
            "top divergent rows",
            "--candidate-partition-value",
            "wepp_id",
            "--candidate-year-offset",
        ]
    ));
}

#[test]
fn pl14s_contract_conformance_enforces_strict_lane_required_vs_strict_equivalent_modes() {
    assert_eq!(strict_lane_mode(".dat"), StrictLaneMode::Required);
    assert_eq!(strict_lane_mode(".DAT"), StrictLaneMode::Required);
    assert_eq!(
        strict_lane_mode(".parquet"),
        StrictLaneMode::StrictEquivalentRequired
    );
}

#[test]
fn pl14s_contract_conformance_classifies_candidate_source_provenance() {
    assert_eq!(
        candidate_source_class(".dat", false),
        CandidateSourceClass::NativeRuntimeDat
    );
    assert_eq!(
        candidate_source_class(".dat", true),
        CandidateSourceClass::ConversionDerivedDat
    );
    assert_eq!(
        candidate_source_class(".parquet", false),
        CandidateSourceClass::NativeRuntimeParquet
    );
}

#[test]
fn pl14s_contract_conformance_requires_conversion_dat_row_consistency_for_evidence_readiness() {
    let collapsed = conversion_derived_dat_row_consistency_verdict(1, 1, 0);
    assert_eq!(collapsed, ConversionSourceRowConsistencyVerdict::Hold);

    let aligned = conversion_derived_dat_row_consistency_verdict(2, 0, 0);
    assert_eq!(aligned, ConversionSourceRowConsistencyVerdict::Ready);
}

#[test]
fn pl14s_contract_conformance_rejects_duplicate_row_keys_in_semantic_lane_inputs() {
    let temp_dir = fixture_temp_dir("pl14s_duplicate_keys");
    fs::create_dir_all(&temp_dir).expect("temporary directory should be creatable");

    let baseline_wat = temp_dir.join("baseline.wat.dat");
    let candidate_wat = temp_dir.join("candidate.wat.dat");
    let report_path = temp_dir.join("semantic_report.json");

    let baseline_payload = format!("{}\n{}\n", dat_row(1, 1, 2008, 10), dat_row(1, 1, 2008, 20));
    let candidate_payload = format!("{}\n", dat_row(1, 1, 2008, 30));

    fs::write(&baseline_wat, baseline_payload).expect("baseline fixture should be writable");
    fs::write(&candidate_wat, candidate_payload).expect("candidate fixture should be writable");

    let script_path = repo_root().join("tools").join("owcmp").join("owcmp");

    let output = Command::new("python3")
        .current_dir(repo_root())
        .arg(script_path)
        .arg("wat")
        .arg("semantic")
        .arg("--baseline-wat")
        .arg(&baseline_wat)
        .arg("--candidate-wat")
        .arg(&candidate_wat)
        .arg("--report-json")
        .arg(&report_path)
        .output()
        .expect("python3 should run semantic comparator");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "duplicate row keys must hard-fail semantic comparator"
    );
    assert!(
        stderr.contains("duplicate row key"),
        "error stream should mention duplicate row key; stderr={stderr}"
    );

    let _ = fs::remove_dir_all(&temp_dir);
}

#[derive(Debug, Clone, Copy)]
struct Wb13Snapshot {
    p: f64,
    rm: f64,
    total_soil: f64,
    frozwt: f64,
    snow_water: f64,
    soil_water_total: f64,
}

#[test]
fn simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage() {
    let report = execute_simimpl18_fixture_run("simimpl18_partition");
    let rows = load_wb13_rows(&report);
    assert!(rows.len() >= 2, "expected at least two WB13 rows");

    let day1 = rows[0];
    assert!(
        (day1.p - 4.4).abs() < 1.0e-6,
        "fixture day-1 precipitation should be 4.4 mm"
    );
    assert!(
        day1.rm.abs() < 1.0e-6,
        "cold all-snow day must publish RM=0; observed {}",
        day1.rm
    );
    assert!(
        (day1.snow_water - 4.4).abs() < 1.0e-6,
        "day-1 Snow-Water must follow runtime SWE accumulation (4.4 mm), not static control; observed {}",
        day1.snow_water
    );
}

#[test]
fn simimpl18_contract_requires_multi_day_storage_state_mutation() {
    let report = execute_simimpl18_fixture_run("simimpl18_storage_mutation");
    let rows = load_wb13_rows(&report);
    assert!(rows.len() >= 2, "expected at least two WB13 rows");

    let day1 = rows[0];
    let day2 = rows[1];

    let invariant_tuple = (day1.total_soil - day2.total_soil).abs() < 1.0e-9
        && (day1.frozwt - day2.frozwt).abs() < 1.0e-9
        && (day1.snow_water - day2.snow_water).abs() < 1.0e-9
        && (day1.soil_water_total - day2.soil_water_total).abs() < 1.0e-9;

    assert!(
        !invariant_tuple,
        "published storage tuple must mutate across varying forcing/thermal days"
    );
}

fn execute_simimpl18_fixture_run(prefix: &str) -> PathBuf {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let source_fixture_dir = repo_root()
        .join("tests")
        .join("fixtures")
        .join("cli01")
        .join("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);

    let climate_path = temp_run_dir.join("case.cli");
    let climate = fs::read_to_string(&climate_path).expect("fixture climate should be readable");
    let climate = climate
        .replace(
            "1 1 2000 10.0 2.0 0.25 3.0 12.0 2.0 200.0 3.0 180.0 -1.0",
            "1 1 2000 4.4 2.0 0.25 3.0 -1.6 -14.6 200.0 3.0 180.0 -1.0",
        )
        .replace(
            "2 1 2000 0.0 0.0 0.0 0.0 10.0 1.0 190.0 2.5 170.0 -2.0",
            "2 1 2000 0.0 0.0 0.0 0.0 12.0 2.0 190.0 2.5 170.0 -2.0",
        );
    fs::write(&climate_path, climate).expect("modified climate should be writable");

    let run_file_path = temp_run_dir.join("case.run");
    let runfile_payload = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "simimpl18-contract-fixture"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = true
pmetpara = "pmetpara.txt"

[outputs]
pass = "output/H5.hbp"
loss = "output/H5.loss.json"
wat = "output/H5.wat.parquet"
"#;
    fs::write(&run_file_path, runfile_payload).expect("runfile fixture should be writable");

    let output_dir = temp_run_dir.join("output");
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: true,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )
    .expect("simimpl18 fixture run should execute");

    report
        .optional_outputs
        .into_iter()
        .find(|path| path.file_name().and_then(|value| value.to_str()) == Some("H5.wat.parquet"))
        .expect("simimpl18 fixture run should emit H5.wat.parquet output")
}

fn load_wb13_rows(wat_path: &Path) -> Vec<Wb13Snapshot> {
    let file = File::open(wat_path)
        .unwrap_or_else(|error| panic!("wat parquet output should be readable: {error}"));
    let reader = SerializedFileReader::new(file)
        .unwrap_or_else(|error| panic!("wat parquet output should parse: {error}"));
    let rows = reader
        .get_row_iter(None)
        .unwrap_or_else(|error| panic!("wat parquet row iterator should open: {error}"));

    rows.map(|row| {
        let row = row.unwrap_or_else(|error| panic!("wat parquet row should decode: {error}"));
        Wb13Snapshot {
            p: row_f64_value(&row, "P"),
            rm: row_f64_value(&row, "RM"),
            total_soil: row_f64_value(&row, "Total-Soil"),
            frozwt: row_f64_value(&row, "frozwt"),
            snow_water: row_f64_value(&row, "Snow-Water"),
            soil_water_total: row_f64_value(&row, "SoilWaterTotal"),
        }
    })
    .collect()
}

fn row_index(row: &Row, column_name: &str) -> usize {
    row.get_column_iter()
        .enumerate()
        .find(|(_, (name, _))| name.as_str() == column_name)
        .map_or_else(
            || panic!("missing required wat parquet column '{column_name}'"),
            |(index, _)| index,
        )
}

fn row_f64_value(row: &Row, column_name: &str) -> f64 {
    let index = row_index(row, column_name);
    if let Ok(value) = row.get_double(index) {
        return value;
    }
    if let Ok(value) = row.get_float(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_int(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_short(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_long(index) {
        return value
            .to_string()
            .parse::<f64>()
            .unwrap_or_else(|error| panic!("i64 column '{column_name}' parse failure: {error}"));
    }
    panic!("column '{column_name}' does not decode as numeric");
}

fn runner_execution_lock() -> &'static Mutex<()> {
    static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    RUN_LOCK.get_or_init(|| Mutex::new(()))
}

fn copy_fixture_to_temp(source_dir: &Path, prefix: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let destination = env::temp_dir().join(format!("{prefix}_{stamp}"));
    copy_dir_recursive(source_dir, &destination);
    destination
}

fn copy_dir_recursive(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("destination directory should be creatable");
    for entry in fs::read_dir(source).expect("source directory should be readable") {
        let entry = entry.expect("directory entry should be readable");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).expect("file copy should succeed");
        }
    }
}
