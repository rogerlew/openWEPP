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
    );
    if shadow {
        // SAFETY: restore the process env immediately after the single run.
        unsafe { std::env::remove_var("OPENWEPP_LANED_SHADOW") };
    }
    let report = report?;
    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&manifest_path).expect("manifest readable"))
            .expect("manifest parses");
    let pass_bytes = fs::read(&report.output_pass).expect("HBP bytes");
    let parquet_bytes = fs::read(report.output_pass.with_file_name("H2637.pass.parquet"))
        .expect("pass parquet bytes");
    Ok((run_dir, manifest, pass_bytes, parquet_bytes))
}

fn enable_native_cropland_routing_coefficients(run_dir: &Path) {
    let path = run_dir.join("p2637.man");
    let text = fs::read_to_string(&path).expect("management fixture readable");
    let mut patched = String::with_capacity(text.len() + 19 * 64);
    let mut inserted = 0_usize;
    for (line_index, line) in text.lines().enumerate() {
        let line = if line_index == 0 {
            "ow-lanuse-1".to_string()
        } else {
            line.replace("1 # Landuse - <Cropland>", "4 # Landuse - <NativeCropland>")
        };
        patched.push_str(&line);
        patched.push('\n');
        if line.starts_with("-40.00000 ") && line.trim_end().ends_with(" 0.00000") {
            patched.push_str("routing_coefficients\n");
            patched.push_str("500.0 0.0 0.0 0.0 0.0\n");
            inserted += 1;
        }
    }
    assert_eq!(inserted, 19, "all H2637 plant scenarios should be patched");
    fs::write(path, patched).expect("patched management fixture writable");
}

fn run_h2637_native_routing(
    tag: &str,
    shadow: bool,
) -> Result<(PathBuf, serde_json::Value, Vec<u8>, Vec<u8>), HillslopeCliError> {
    let run_dir = copy_fixture_to_temp(tag);
    enable_native_cropland_routing_coefficients(&run_dir);
    let output_dir = run_dir.join("output");
    let manifest_path = run_dir.join("manifest.json");
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
    );
    if shadow {
        // SAFETY: restore the process env immediately after the single run.
        unsafe { std::env::remove_var("OPENWEPP_LANED_SHADOW") };
    }
    let report = report?;
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

#[test]
#[ignore = "D12 H2637 shadow evidence: runs the full H2637 fixture twice"]
fn h2637_native_shadow_classifies_uniform_shape_after_d12() {
    let (_dir_off, manifest_off, pass_off, parquet_off) =
        run_h2637_native_routing("native_off", false)
            .expect("native-routed H2637 must run with Lane D shadow disabled");
    assert!(
        find_key(&manifest_off, "laned_shadow").is_none(),
        "no shadow keys when the shadow is off"
    );

    let (_dir_on, manifest_on, pass_on, parquet_on) = run_h2637_native_routing("native_on", true)
        .expect("native-routed H2637 must run with Lane D shadow enabled");
    assert_eq!(pass_on, pass_off, "shadow must preserve HBP bytes");
    assert_eq!(
        parquet_on, parquet_off,
        "shadow must preserve pass parquet bytes"
    );
    let days_seen = find_key(&manifest_on, "days_seen")
        .and_then(serde_json::Value::as_u64)
        .expect("manifest should report days_seen");
    let days_routed = find_key(&manifest_on, "days_routed")
        .and_then(serde_json::Value::as_u64)
        .expect("manifest should report days_routed");
    let days_uniform_shape = find_key(&manifest_on, "days_uniform_shape")
        .and_then(serde_json::Value::as_u64)
        .expect("manifest should report days_uniform_shape");
    let days_uniform_shape_with_routed_melt =
        find_key(&manifest_on, "days_uniform_shape_with_routed_melt")
            .and_then(serde_json::Value::as_u64)
            .expect("manifest should report days_uniform_shape_with_routed_melt");
    let days_uniform_shape_without_routed_melt =
        find_key(&manifest_on, "days_uniform_shape_without_routed_melt")
            .and_then(serde_json::Value::as_u64)
            .expect("manifest should report days_uniform_shape_without_routed_melt");
    assert_eq!(days_seen, 731);
    assert_eq!(days_routed, 622);
    assert_eq!(days_uniform_shape, 6);
    assert_eq!(days_uniform_shape_with_routed_melt, 0);
    assert_eq!(days_uniform_shape_without_routed_melt, 6);
}
