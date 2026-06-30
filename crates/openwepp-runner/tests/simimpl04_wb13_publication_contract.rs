use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use openwepp_runner::{
    HillslopeDefaultRuntimeActivation, HillslopeRunReport, HillslopeRunRequest,
    HillslopeRuntimeSelection, HillslopeRuntimeSelectionPolicy, SidecarPolicy,
    execute_hillslope_run_with_runtime_policy,
};

#[test]
fn simimpl04_contract_requires_simulation_owned_wb13_publication_provenance() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "simimpl04-simout"
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
plot = "output/H5.plot.parquet"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report(runfile, "simimpl04_simout");

    let manifest_json = read_manifest_json(&report);

    assert_json_string(
        &manifest_json,
        "/wb13_publication/source",
        "direct-publication-frame",
    );
    assert_json_bool(
        &manifest_json,
        "/wb13_publication/projection_fallback_used",
        false,
    );
    assert_json_string(
        &manifest_json,
        "/wb13_publication/guard_id",
        "HS-SIMOUT-E-001",
    );
    assert_json_array_len(
        &manifest_json,
        "/wb13_publication/replay_candidate_surfaces",
        0,
    );
}

#[test]
fn simimpl14_contract_requires_continuous_wb13_span_and_simulation_year_row_keys() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "simimpl14-continuous-span"
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
plot = "output/H5.plot.parquet"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report(runfile, "simimpl14_span");

    let manifest_json = read_manifest_json(&report);
    assert_json_i64(&manifest_json, "/execution_provenance/climate_day_count", 2);
    assert_json_i64(
        &manifest_json,
        "/execution_provenance/executed_day_count",
        2,
    );
    assert_json_i64(&manifest_json, "/wb13_publication/row_count", 2);
    assert_json_bool(
        &manifest_json,
        "/wb13_publication/sim_day_index_monotonic",
        true,
    );
    assert_json_i64(&manifest_json, "/wb13_publication/first_row_key/year", 2000);
    assert_json_i64(
        &manifest_json,
        "/wb13_publication/last_row_key/julian_day",
        2,
    );
}

#[test]
fn simimpl14_contract_requires_run_span_truthful_loss_output_summary() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "simimpl14-loss-span"
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
plot = "output/H5.plot.parquet"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report(runfile, "simimpl14_loss");
    let loss_text = fs::read_to_string(&report.output_loss).unwrap_or_else(|error| {
        panic!(
            "loss output should be readable at {}: {error}",
            report.output_loss.display()
        )
    });
    let loss_json: serde_json::Value =
        serde_json::from_str(&loss_text).expect("loss output should parse as JSON");

    assert_json_i64(&loss_json, "/climate_day_count", 2);
    assert_json_i64(&loss_json, "/executed_day_count", 2);
    assert_json_i64(&loss_json, "/first_day_julian", 1);
    assert_json_i64(&loss_json, "/last_day_julian", 2);
}

fn read_manifest_json(report: &HillslopeRunReport) -> serde_json::Value {
    let manifest_text = fs::read_to_string(&report.manifest_path).unwrap_or_else(|error| {
        panic!(
            "manifest should be readable at {}: {error}",
            report.manifest_path.display()
        )
    });
    serde_json::from_str(&manifest_text)
        .unwrap_or_else(|error| panic!("manifest should parse as JSON: {error}"))
}

fn assert_json_string(document: &serde_json::Value, pointer: &str, expected: &str) {
    let observed = document
        .pointer(pointer)
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("missing string JSON pointer {pointer}"));
    assert_eq!(observed, expected, "unexpected value at {pointer}");
}

fn assert_json_bool(document: &serde_json::Value, pointer: &str, expected: bool) {
    let observed = document
        .pointer(pointer)
        .and_then(serde_json::Value::as_bool)
        .unwrap_or_else(|| panic!("missing bool JSON pointer {pointer}"));
    assert_eq!(observed, expected, "unexpected value at {pointer}");
}

fn assert_json_i64(document: &serde_json::Value, pointer: &str, expected: i64) {
    let observed = document
        .pointer(pointer)
        .and_then(serde_json::Value::as_i64)
        .unwrap_or_else(|| panic!("missing integer JSON pointer {pointer}"));
    assert_eq!(observed, expected, "unexpected value at {pointer}");
}

fn assert_json_array_len(document: &serde_json::Value, pointer: &str, expected: usize) {
    let observed = document
        .pointer(pointer)
        .and_then(serde_json::Value::as_array)
        .unwrap_or_else(|| panic!("missing array JSON pointer {pointer}"))
        .len();
    assert_eq!(observed, expected, "unexpected array length at {pointer}");
}

fn execute_fixture_with_runfile_report(
    runfile_payload: &str,
    prefix: &str,
) -> (HillslopeRunReport, PathBuf) {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);
    let run_file_path = temp_run_dir.join("case.run");
    fs::write(&run_file_path, runfile_payload).expect("runfile fixture should be writable");

    let output_dir = temp_run_dir.join("output");
    let report = execute_hillslope_run_with_runtime_policy(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    )
    .expect("fixture run should succeed before WB13 provenance assertions");

    (report, temp_run_dir)
}

fn runner_execution_lock() -> &'static Mutex<()> {
    static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    RUN_LOCK.get_or_init(|| Mutex::new(()))
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/cli01")
        .join(name)
}

fn copy_fixture_to_temp(source_dir: &Path, prefix: &str) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix epoch should be before now")
        .as_nanos();
    let destination = std::env::temp_dir().join(format!("{prefix}_{timestamp}"));

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
