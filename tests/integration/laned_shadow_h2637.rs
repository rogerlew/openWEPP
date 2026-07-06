//! Lane D activation guard — the REAL-H2637 legacy executed vector
//! (`SC-OFEROUTE-001#INV-OFEROUTE-012`): the opt-in seam shadow must fail
//! closed when a legacy management lacks native `routing_coefficients`
//! authority. The shadow-off path remains runnable and produces no Lane D
//! manifest block.

use std::fs;
use std::path::{Path, PathBuf};

use openwepp_runner::{
    HillslopeCliError, HillslopeRunRequest, SidecarPolicy, execute_hillslope_run,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/laned_shadow_h2637")
}

fn copy_fixture_to_temp(tag: &str) -> PathBuf {
    let destination =
        std::env::temp_dir().join(format!("laned_shadow_h2637_{tag}_{}", std::process::id()));
    if destination.exists() {
        fs::remove_dir_all(&destination).expect("stale run dir removable");
    }
    fs::create_dir_all(&destination).expect("run dir");
    for entry in fs::read_dir(fixture_dir()).expect("fixture dir") {
        let entry = entry.expect("fixture entry");
        fs::copy(entry.path(), destination.join(entry.file_name())).expect("fixture copy");
    }
    destination
}

fn run_h2637(
    tag: &str,
    shadow: bool,
) -> Result<(PathBuf, serde_json::Value, Vec<u8>, Vec<u8>), HillslopeCliError> {
    let run_dir = copy_fixture_to_temp(tag);
    let output_dir = run_dir.join("output");
    let manifest_path = run_dir.join("manifest.json");
    // nextest runs each test in its own process, and both runs execute
    // serially inside this one test — the env mutation cannot leak.
    if shadow {
        // SAFETY: single-threaded test setup before any runner threads.
        unsafe { std::env::set_var("OPENWEPP_LANED_SHADOW", "1") };
    } else {
        // SAFETY: as above.
        unsafe { std::env::remove_var("OPENWEPP_LANED_SHADOW") };
    }
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("p2637.run.toml"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: Some(manifest_path.clone()),
        },
        &["openwepp-cli-hill".to_string()],
    )?;
    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&manifest_path).expect("manifest readable"))
            .expect("manifest parses");
    let pass_bytes = fs::read(&report.output_pass).expect("HBP bytes");
    let parquet_bytes = fs::read(report.output_pass.with_file_name("H2637.pass.parquet"))
        .expect("pass parquet bytes");
    Ok((run_dir, manifest, pass_bytes, parquet_bytes))
}

fn find_key<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    match value {
        serde_json::Value::Object(map) => map
            .get(key)
            .or_else(|| map.values().find_map(|nested| find_key(nested, key))),
        serde_json::Value::Array(items) => items.iter().find_map(|item| find_key(item, key)),
        _ => None,
    }
}

#[test]
fn h2637_legacy_shadow_fails_closed_without_routing_coefficients() {
    let (_dir_off, manifest_off, pass_off, parquet_off) =
        run_h2637("off", false).expect("legacy H2637 must run with Lane D shadow disabled");
    assert!(
        find_key(&manifest_off, "laned_shadow").is_none(),
        "no shadow keys when the shadow is off"
    );
    assert!(!pass_off.is_empty(), "shadow-off HBP should be written");
    assert!(
        !parquet_off.is_empty(),
        "shadow-off pass parquet should be written"
    );

    let err = run_h2637("on", true)
        .expect_err("legacy H2637 must fail closed when Lane D shadow lacks routing coefficients");
    // SAFETY: restore the process env for any later in-process work.
    unsafe { std::env::remove_var("OPENWEPP_LANED_SHADOW") };
    let message = err.to_string();
    assert!(
        message.contains("routing coefficient extension"),
        "expected routing coefficient extension error, got {message}"
    );
    assert!(
        message.contains("OPENWEPP_LANED_SHADOW"),
        "expected Lane D shadow opt-in error, got {message}"
    );
}
