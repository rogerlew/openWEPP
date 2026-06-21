use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn r6_direct_publication_cutover_cli_flag_fails_closed_before_outputs() {
    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, "r6_cli_cutover_candidate");
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
        !output.status.success(),
        "R6 cutover candidate must fail closed until parity gates pass"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("R6-DIRECT-PUBLICATION-PARITY"),
        "unexpected stderr: {stderr}"
    );
    assert!(
        stderr.contains("HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT"),
        "unexpected stderr: {stderr}"
    );

    for output_name in [
        "H5.hbp",
        "H5.loss.json",
        "H5.wat.parquet",
        "H5.plot.parquet",
        "openwepp_hillslope_run_manifest.json",
    ] {
        assert!(
            !output_dir.join(output_name).exists(),
            "fail-closed CLI cutover candidate must not write {output_name}"
        );
    }
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
