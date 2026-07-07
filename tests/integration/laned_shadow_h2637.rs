//! HARNESS CONTRACT (CR-M1): the env-mutating tests in this file are
//! supported ONLY under nextest (process-per-test isolation). The stock
//! threaded `cargo test` harness races `set_var`/`remove_var` against
//! concurrent `getenv` (glibc UB). Every run helper neutralizes ALL Lane D
//! selector variables at entry (`OPENWEPP_LANED_SHADOW`,
//! `OPENWEPP_LANED_ACTIVE`, plus the abandoned implicit selector env var) so
//! inherited shell state cannot leak in (CR-M2, T3-QA-M3).
//!
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

struct AbandonedImplicitEnvCleanup;

impl Drop for AbandonedImplicitEnvCleanup {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE_IMPLICIT") };
    }
}

#[test]
fn abandoned_implicit_selector_env_fails_closed_at_startup() {
    unsafe { std::env::set_var("OPENWEPP_LANED_ACTIVE_IMPLICIT", "0") };
    let _cleanup = AbandonedImplicitEnvCleanup;

    let result = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: PathBuf::from("/tmp/openwepp_missing_run_dir_for_abandoned_selector_guard"),
            run_file: PathBuf::from("case.run"),
            output_dir: PathBuf::from("/tmp/openwepp_missing_output_for_abandoned_selector_guard"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    );

    match result.expect_err("abandoned implicit selector env must fail before run setup") {
        HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
            assert_eq!(surface, "OPENWEPP_LANED_ACTIVE_IMPLICIT");
            assert!(detail.contains("ADR-0037"));
            assert!(detail.contains("abandoned"));
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

fn run_h2637(
    tag: &str,
    shadow: bool,
) -> Result<(PathBuf, serde_json::Value, Vec<u8>, Vec<u8>), HillslopeCliError> {
    let run_dir = copy_fixture_to_temp(tag);
    let output_dir = run_dir.join("output");
    let manifest_path = run_dir.join("manifest.json");
    // CR-M2/T3-QA-M3: neutralize ALL sibling selector variables so inherited
    // shell state cannot turn a baseline leg into a shadow/active run or
    // stale abandoned-selector startup failure.
    // SAFETY: single-threaded test setup before any runner threads.
    unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE") };
    // SAFETY: as above.
    unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE_IMPLICIT") };
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
    // CR-M2/T3-QA-M3: neutralize the sibling selectors and the abandoned
    // implicit selector (inherited shell state).
    // SAFETY: single-threaded test setup before any runner threads.
    unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE") };
    // SAFETY: as above.
    unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE_IMPLICIT") };
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

// ---------------------------------------------------------------------------
// D15A (SC-OFEROUTE-001 rev 27): the opt-in ACTIVE production owner.
// ---------------------------------------------------------------------------

fn run_h2637_native_active(
    tag: &str,
    active: bool,
) -> Result<(PathBuf, serde_json::Value, Vec<u8>, Vec<u8>), HillslopeCliError> {
    let run_dir = copy_fixture_to_temp(tag);
    enable_native_cropland_routing_coefficients(&run_dir);
    let output_dir = run_dir.join("output");
    let manifest_path = run_dir.join("manifest.json");
    // CR-M2/T3-QA-M3: neutralize the sibling selectors (inherited shell
    // state) plus the abandoned implicit selector, so the "plain active"
    // evidence leg cannot fail on stale operator environment.
    // SAFETY: single-threaded test setup before any runner threads.
    unsafe { std::env::remove_var("OPENWEPP_LANED_SHADOW") };
    // SAFETY: as above.
    unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE_IMPLICIT") };
    if active {
        // SAFETY: single-threaded test setup before any runner threads.
        unsafe { std::env::set_var("OPENWEPP_LANED_ACTIVE", "1") };
    } else {
        // SAFETY: as above.
        unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE") };
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
    if active {
        // SAFETY: restore the process env immediately after the single run.
        unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE") };
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

#[test]
fn h2637_active_fails_closed_without_routing_coefficients() {
    // Legacy management (no native routing_coefficients): the ACTIVE
    // selector must fail closed before streaming, mirroring the shadow's
    // rev-20 guard.
    let run_dir = copy_fixture_to_temp("active_legacy");
    let output_dir = run_dir.join("output");
    // SAFETY: single-threaded test setup before any runner threads.
    unsafe { std::env::set_var("OPENWEPP_LANED_ACTIVE", "1") };
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("p2637.run.toml"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: Some(run_dir.join("manifest.json")),
        },
        &["openwepp-cli-hill".to_string()],
    );
    // SAFETY: restore the process env for any later in-process work.
    unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE") };
    let message = report
        .expect_err("active selector must fail closed")
        .to_string();
    assert!(
        message.contains("routing coefficient extension"),
        "expected routing coefficient extension error, got {message}"
    );
    assert!(
        message.contains("OPENWEPP_LANED_ACTIVE"),
        "expected active opt-in error, got {message}"
    );
}

#[test]
fn h2637_active_and_shadow_are_mutually_exclusive() {
    let run_dir = copy_fixture_to_temp("active_shadow_conflict");
    enable_native_cropland_routing_coefficients(&run_dir);
    let output_dir = run_dir.join("output");
    // SAFETY: single-threaded test setup before any runner threads.
    unsafe { std::env::set_var("OPENWEPP_LANED_ACTIVE", "1") };
    // SAFETY: as above.
    unsafe { std::env::set_var("OPENWEPP_LANED_SHADOW", "1") };
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("p2637.run.toml"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: Some(run_dir.join("manifest.json")),
        },
        &["openwepp-cli-hill".to_string()],
    );
    // SAFETY: restore the process env for any later in-process work.
    unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE") };
    // SAFETY: as above.
    unsafe { std::env::remove_var("OPENWEPP_LANED_SHADOW") };
    let message = report
        .expect_err("active + shadow must fail closed")
        .to_string();
    assert!(
        message.contains("mutually exclusive"),
        "expected mutual-exclusion error, got {message}"
    );
}

#[test]
#[ignore = "D15A H2637 active-owner evidence: runs the full H2637 fixture twice"]
fn h2637_native_active_owner_routes_and_closes() {
    // Default/off on the SAME native-patched fixture: no active keys. This
    // test asserts only presence/absence; the INV-OFEROUTE-010 BYTE
    // comparison itself lives in the P4 gate evidence (SHA256 vs the
    // recorded package baseline), not in this test.
    let (_dir_off, manifest_off, pass_off, parquet_off) =
        run_h2637_native_active("active_off", false)
            .expect("native-routed H2637 must run with the active owner disabled");
    assert!(
        find_key(&manifest_off, "laned_active").is_none(),
        "no active keys when the active owner is off"
    );
    assert!(!pass_off.is_empty() && !parquet_off.is_empty());

    // ACTIVE: routing owns the surface-water path (rev 27). The run must
    // complete with the day-closure hard-fails live, which means every
    // routed day satisfied the rev-27 tolerances.
    let (_dir_on, manifest_on, _pass_on, _parquet_on) = run_h2637_native_active("active_on", true)
        .expect("native-routed H2637 must run with the active owner enabled");
    let active = find_key(&manifest_on, "laned_active").expect("manifest laned_active block");
    let days_seen = find_key(active, "days_seen")
        .and_then(serde_json::Value::as_u64)
        .expect("days_seen");
    let days_routed = find_key(active, "days_routed")
        .and_then(serde_json::Value::as_u64)
        .expect("days_routed");
    assert_eq!(days_seen, 731);
    assert!(days_routed > 0, "active owner must route event days");
    let total_source_m3 = find_key(active, "total_source_m3")
        .and_then(serde_json::Value::as_f64)
        .expect("total_source_m3");
    let total_routed_outlet_m3 = find_key(active, "total_routed_outlet_m3")
        .and_then(serde_json::Value::as_f64)
        .expect("total_routed_outlet_m3");
    assert!(total_source_m3 > 0.0 && total_routed_outlet_m3 > 0.0);
    let max_supply = find_key(active, "max_supply_reconstruction_rel")
        .and_then(serde_json::Value::as_f64)
        .expect("max_supply_reconstruction_rel");
    let max_cascade = find_key(active, "max_day_cascade_residual_rel")
        .and_then(serde_json::Value::as_f64)
        .expect("max_day_cascade_residual_rel");
    let max_identity = find_key(active, "max_day_identity_residual_rel")
        .and_then(serde_json::Value::as_f64)
        .expect("max_day_identity_residual_rel");
    assert!(max_supply <= 1.0e-9, "supply reconstruction {max_supply}");
    assert!(max_cascade <= 1.0e-9, "day cascade residual {max_cascade}");
    assert!(
        max_identity <= 1.0e-6,
        "day identity residual {max_identity}"
    );
    // The shadow block must be absent on an active run (mutual exclusion).
    assert!(
        find_key(&manifest_on, "laned_shadow").is_none(),
        "no shadow keys on an active run"
    );
}
