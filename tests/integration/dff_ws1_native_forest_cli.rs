//! DFF-WS1 Increment-2 end-to-end verification: run a native openWEPP forest
//! (`ow-lanuse-1`) hillslope through the production hillslope runner and prove
//! it parses, reconciles against its disturbed `.sol` policy, resolves an
//! explicit PMET record (no compatibility fallback), and produces outputs.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use openwepp_runner::{HillslopeRunRequest, SidecarPolicy, execute_hillslope_run};

fn runner_execution_lock() -> &'static Mutex<()> {
    static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    RUN_LOCK.get_or_init(|| Mutex::new(()))
}

fn native_forest_fixture_dir() -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("integration file parent exists")
        .parent()
        .expect("tests directory exists")
        .join("fixtures")
        .join("dff_ws1_native_forest")
        .join("hjandrews_conifer_forest")
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

fn copy_fixture_to_temp(prefix: &str) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix epoch should be before now")
        .as_nanos();
    let destination = std::env::temp_dir().join(format!("{prefix}_{timestamp}"));
    copy_dir_recursive(&native_forest_fixture_dir(), &destination);
    destination
}

#[test]
fn native_forest_hillslope_runs_end_to_end_with_pmet_hit_and_reconciliation() {
    let _guard = runner_execution_lock()
        .lock()
        .expect("runner execution lock should be acquirable");

    let run_dir = copy_fixture_to_temp("dff_ws1_native_forest");
    let output_dir = run_dir.join("output");
    let manifest_path = run_dir.join("manifest.json");

    // The run succeeding at all proves the native forest path end-to-end: the
    // `ow-lanuse-1` `.man` parses, the `.man` forest class reconciles with the
    // `.sol` `DisturbedPolicy` (luse=forest), the PMET record resolves, the
    // forest projection emits the growth-symbol surface, and the kernel runs.
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("p2.run.toml"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: Some(manifest_path.clone()),
        },
        &["openwepp-cli-hill".to_string()],
    )
    .expect("native forest hillslope should run end-to-end");

    assert!(report.output_pass.is_file(), "HBP pass output should exist");
    assert!(report.output_loss.is_file(), "loss output should exist");

    // The PMET sidecar carries an explicit `Tah_4899` record matching the forest
    // plant name, so no compatibility first-row fallback warning should fire.
    assert!(
        report
            .sidecar_warnings
            .iter()
            .all(|warning| !warning.to_ascii_lowercase().contains("first-row")),
        "native forest must resolve an explicit PMET record, not the fallback: {:?}",
        report.sidecar_warnings
    );

    // Confirm the run went through the direct production executor (where the
    // forest reconciliation + PMET authority live).
    let manifest = fs::read_to_string(&manifest_path).expect("manifest should be readable");
    assert!(
        manifest.contains("R7C-DIRECT-PRODUCTION-EXECUTOR"),
        "run should execute on the direct production path"
    );

    fs::remove_dir_all(&run_dir).ok();
}
