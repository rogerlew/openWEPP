use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use openwepp_runner::{
    HillslopeRunReport, HillslopeRunRequest, SidecarPolicy, execute_hillslope_run,
};

#[test]
#[allow(clippy::too_many_lines)]
fn simimpl04_contract_requires_wepp_ui_requested_effective_lane_closure_manifest() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "simimpl04-simmode"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = true
pmetpara = "pmetpara.txt"

[inputs.snow]
rst = 0.0
newsnw = 100.0
ssd = 250.0

[inputs.frost]
wintRed = 1
fineTop = 10
fineBot = 8
ksnowf = 0.10
kresf = 0.20
ksoilf = 0.30
kfactor1 = 0.00001
kfactor2 = 0.00002
kfactor3 = 0.50

[outputs]
pass = "output/H5.hbp"
loss = "output/H5.loss.json"
wat = "output/H5.wat.parquet"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report(runfile, "simimpl04_simmode");

    let manifest_json = read_manifest_json(&report);

    assert_json_i64(&manifest_json, "/mode_selection/wepp_ui/requested", 1);
    assert_json_i64(&manifest_json, "/mode_selection/wepp_ui/effective", 1);
    assert_json_string(
        &manifest_json,
        "/mode_selection/wepp_ui/selected_lane",
        "hourly",
    );
    assert_json_bool(
        &manifest_json,
        "/mode_selection/wepp_ui/mode_divergence",
        false,
    );
    assert_json_string(
        &manifest_json,
        "/mode_selection/wepp_ui/guard_id",
        "WUI-E-005",
    );

    assert_json_string(&manifest_json, "/timestep_policy/policy", "hourly");
    assert_json_string(&manifest_json, "/timestep_policy/scheduler_mode", "hourly");
    assert_json_string(&manifest_json, "/timestep_policy/requested_mode", "hourly");
    assert_json_string(&manifest_json, "/timestep_policy/effective_mode", "hourly");
    assert_json_i64(&manifest_json, "/timestep_policy/timestep_seconds", 3600);
    assert_json_bool(&manifest_json, "/timestep_policy/physics_enabled", true);
    assert_json_bool(
        &manifest_json,
        "/timestep_policy/subhourly_scaffold_available",
        true,
    );
    assert_json_string(
        &manifest_json,
        "/timestep_policy/guard_id",
        "HS-SIMMODE-E-001",
    );

    assert_json_string(&manifest_json, "/adapter_boundary/selected_lane", "hourly");
    assert_json_string(&manifest_json, "/adapter_boundary/scheduler_mode", "hourly");
    assert_json_string(&manifest_json, "/adapter_boundary/requested_mode", "hourly");
    assert_json_string(&manifest_json, "/adapter_boundary/effective_mode", "hourly");
    assert_json_string(
        &manifest_json,
        "/adapter_boundary/adopt_profile",
        "SIMIMPL08-adopt-only",
    );
    assert_json_bool(
        &manifest_json,
        "/adapter_boundary/reject_surfaces_excluded",
        true,
    );
    assert_json_bool(
        &manifest_json,
        "/adapter_boundary/defer_surfaces_excluded",
        true,
    );
    assert_json_string(
        &manifest_json,
        "/adapter_boundary/guard_id",
        "HS-SIMCONS-E-001",
    );

    assert_json_string(
        &manifest_json,
        "/coupling_vectors/guard_id",
        "HS-SIMCOUP-E-001",
    );
    assert_json_bool(&manifest_json, "/coupling_vectors/winter/active", false);
    assert_json_bool(
        &manifest_json,
        "/coupling_vectors/winter/snow_file_present",
        true,
    );
    assert_json_bool(&manifest_json, "/coupling_vectors/frsoil/active", true);
    assert_json_bool(
        &manifest_json,
        "/coupling_vectors/frsoil/frost_file_present",
        true,
    );
    assert_json_bool(
        &manifest_json,
        "/coupling_vectors/frsoil/wint_red_enabled",
        true,
    );
    assert_json_bool(
        &manifest_json,
        "/coupling_vectors/soil/infcap_within_ssc",
        true,
    );
    assert_json_string(
        &manifest_json,
        "/coupling_vectors/hydout_equivalent/source",
        "simulation-owned",
    );
    assert_json_bool(
        &manifest_json,
        "/coupling_vectors/hydout_equivalent/closure_within_tolerance",
        true,
    );
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
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )
    .expect("fixture run should succeed before mode-closure assertions");

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
