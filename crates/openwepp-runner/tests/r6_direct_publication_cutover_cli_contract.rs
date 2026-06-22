use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn r6_direct_publication_cutover_cli_flag_writes_direct_outputs_and_manifest() {
    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, "r6_cli_cutover_candidate");
    enable_r6j_pass_parquet_output(&temp_run_dir);
    let output_dir = temp_run_dir.join("output");

    let output = Command::new(env!("CARGO_BIN_EXE_openwepp-cli-hill"))
        .arg("--run-dir")
        .arg(&temp_run_dir)
        .arg("--run-file")
        .arg("case.run")
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--direct-publication-frame-cutover")
        .output()
        .expect("openwepp-cli-hill should be invokable");

    assert!(
        output.status.success(),
        "unexpected stderr: {}",
        stderr(&output)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("R6-DIRECT-PUBLICATION-PARITY"),
        "R6J cutover should not fail parity gates: {stderr}"
    );
    assert!(
        !stderr.contains("HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY"),
        "PMET layer ULP blocker should be cleared: {stderr}"
    );
    assert!(
        !stderr.contains("HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT"),
        "day-state carry blocker should be cleared: {stderr}"
    );
    assert!(
        !stderr.contains("HOLD-R6E-PRODUCTION-DIRECT-RUNTIME-INPUT-BINDING-ABSENT"),
        "input binding blocker should be cleared: {stderr}"
    );

    for output_name in [
        "H5.hbp",
        "H5.loss.json",
        "H5.pass.parquet",
        "H5.wat.parquet",
        "H5.plot.parquet",
        "openwepp_hillslope_run_manifest.json",
    ] {
        assert!(
            output_dir.join(output_name).is_file(),
            "CLI cutover candidate must write {output_name}"
        );
    }
    let manifest_text = fs::read_to_string(output_dir.join("openwepp_hillslope_run_manifest.json"))
        .expect("manifest should be readable");
    let manifest_json: serde_json::Value =
        serde_json::from_str(&manifest_text).expect("manifest should parse");
    assert_eq!(
        manifest_json
            .pointer("/execution_provenance/publication_source")
            .and_then(serde_json::Value::as_str),
        Some("direct-publication-frame")
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/source")
            .and_then(serde_json::Value::as_str),
        Some("direct-publication-frame")
    );
    assert_json_i64(
        &manifest_json,
        "/direct_runtime_counters/run_frame_constructions",
        0,
    );
    assert_json_i64(&manifest_json, "/direct_runtime_counters/skeleton_runs", 0);
    assert_json_i64(
        &manifest_json,
        "/direct_runtime_counters/publication_capture_runs",
        0,
    );
    assert_json_i64(
        &manifest_json,
        "/direct_runtime_counters/compatibility_edge_invocations",
        0,
    );
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn assert_json_i64(document: &serde_json::Value, pointer: &str, expected: i64) {
    assert_eq!(
        document
            .pointer(pointer)
            .and_then(serde_json::Value::as_i64),
        Some(expected),
        "unexpected value at {pointer}"
    );
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

fn enable_r6j_pass_parquet_output(run_dir: &Path) {
    let run_file = run_dir.join("case.run");
    let mut text = fs::read_to_string(&run_file).expect("case.run should be readable");
    if !text.contains("pass_parquet") {
        text.push_str("pass_parquet = \"output/H5.pass.parquet\"\n");
    }
    fs::write(&run_file, text).expect("case.run should be writable");
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
