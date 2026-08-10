//! HARNESS CONTRACT (CR-M1): the env-mutating tests in this file are
//! supported ONLY under nextest (process-per-test isolation). The stock
//! threaded `cargo test` harness races `set_var`/`remove_var` against
//! concurrent `getenv` (glibc UB). Every run helper neutralizes ALL Lane D
//! selector variables at entry (`OPENWEPP_LANED_SHADOW`,
//! `OPENWEPP_LANED_ACTIVE`, `OPENWEPP_LANED_ACTIVE_DISABLE`,
//! `OPENWEPP_LANED_ACTIVE_TRACE`,
//! `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M`, plus the abandoned implicit
//! selector env var) so inherited shell state cannot leak in (CR-M2, T3-QA-M3).
//!
//! Lane D activation guard — the H2637 legacy management/slope/soil executed
//! vector with a test-local frost-free temperature envelope
//! (`SC-OFEROUTE-001#INV-OFEROUTE-012`): the opt-in seam shadow must fail
//! closed when a legacy management lacks native `routing_coefficients`
//! authority. The temperature-only mutation prevents the unrelated WB16
//! partial-frost missing-clock guard from preempting this routing-boundary
//! test; precipitation and every routing input remain unchanged. The
//! shadow-off path remains runnable and produces no Lane D manifest block.

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
    warm_climate_for_routing_seam(&destination);
    destination
}

fn warm_climate_for_routing_seam(run_dir: &Path) {
    let path = run_dir.join("p2637.cli");
    let text = fs::read_to_string(&path).expect("climate fixture readable");
    let mut rendered = String::with_capacity(text.len());
    let mut warmed_rows = 0_usize;
    for line in text.lines() {
        let mut fields: Vec<&str> = line.split_whitespace().collect();
        let is_daily_row = fields.len() == 13
            && fields[0].parse::<u16>().is_ok()
            && fields[1].parse::<u8>().is_ok()
            && fields[2].parse::<i32>().is_ok();
        if is_daily_row {
            fields[7] = "20.0";
            fields[8] = "10.0";
            fields[12] = "8.0";
            rendered.push_str(&fields.join(" "));
            warmed_rows += 1;
        } else {
            rendered.push_str(line);
        }
        rendered.push('\n');
    }
    assert!(warmed_rows > 0, "climate fixture must contain daily rows");
    fs::write(path, rendered).expect("warmed climate fixture writable");
}

struct AbandonedImplicitEnvCleanup;

impl Drop for AbandonedImplicitEnvCleanup {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE_IMPLICIT") };
    }
}

struct LaneDSelectorEnvCleanup;

impl Drop for LaneDSelectorEnvCleanup {
    fn drop(&mut self) {
        clear_laned_selector_env();
    }
}

fn clear_laned_selector_env() {
    // SAFETY: these env-mutating tests are nextest-only process-isolated
    // tests; callers clear the variables before runner work starts.
    unsafe {
        std::env::remove_var("OPENWEPP_LANED_ACTIVE");
        std::env::remove_var("OPENWEPP_LANED_ACTIVE_DISABLE");
        std::env::remove_var("OPENWEPP_LANED_ACTIVE_TRACE");
        std::env::remove_var("OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M");
        std::env::remove_var("OPENWEPP_LANED_SHADOW");
        std::env::remove_var("OPENWEPP_LANED_ACTIVE_IMPLICIT");
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

#[test]
fn active_trace_selector_requires_active_before_outputs() {
    let run_dir = copy_fixture_to_temp("active_trace_without_active");
    let output_dir = run_dir.join("output");
    clear_laned_selector_env();
    // SAFETY: single-threaded test setup before runner work starts.
    unsafe { std::env::set_var("OPENWEPP_LANED_ACTIVE_TRACE", "1") };
    let _cleanup = LaneDSelectorEnvCleanup;

    let result = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("p2637.run.toml"),
            output_dir: output_dir.clone(),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: Some(run_dir.join("manifest.json")),
        },
        &["openwepp-cli-hill".to_string()],
    );

    match result.expect_err("trace selector without active must fail at startup") {
        HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
            assert_eq!(surface, "OPENWEPP_LANED_ACTIVE_TRACE");
            assert!(detail.contains("OPENWEPP_LANED_ACTIVE=1"));
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
    assert!(
        !output_dir.exists(),
        "trace-only selector must fail before output directory creation"
    );
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
    clear_laned_selector_env();
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

fn remove_first_native_routing_coefficient_block(run_dir: &Path) {
    let path = run_dir.join("p2637.man");
    let text = fs::read_to_string(&path).expect("management fixture readable");
    let mut patched = String::with_capacity(text.len());
    let mut lines = text.lines();
    let mut removed = false;
    while let Some(line) = lines.next() {
        if !removed && line == "routing_coefficients" {
            let coefficient_line = lines
                .next()
                .expect("routing_coefficients block should have coefficient line");
            assert_eq!(
                coefficient_line, "500.0 0.0 0.0 0.0 0.0",
                "test fixture should remove exactly the native routing coefficient line"
            );
            removed = true;
            continue;
        }
        patched.push_str(line);
        patched.push('\n');
    }
    assert!(
        removed,
        "one native routing coefficient block should be removed"
    );
    fs::write(path, patched).expect("mixed management fixture writable");
}

fn truncate_first_native_routing_coefficient_block(run_dir: &Path) {
    let path = run_dir.join("p2637.man");
    let text = fs::read_to_string(&path).expect("management fixture readable");
    let patched = text.replacen("500.0 0.0 0.0 0.0 0.0", "500.0 0.0 0.0 0.0", 1);
    assert_ne!(
        patched, text,
        "one native routing coefficient block should be truncated"
    );
    fs::write(path, patched).expect("malformed management fixture writable");
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
    // implicit selector (inherited shell state). Native coefficients are
    // default-active under SC-OFEROUTE-001 rev 46, so shadow diagnostics use
    // the explicit active-disable rollback selector.
    clear_laned_selector_env();
    // SAFETY: single-threaded test setup before any runner threads.
    unsafe { std::env::set_var("OPENWEPP_LANED_ACTIVE_DISABLE", "1") };
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
    // SAFETY: restore the process env immediately after the single run.
    unsafe { std::env::remove_var("OPENWEPP_LANED_ACTIVE_DISABLE") };
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
    assert!(
        find_key(&manifest_off, "laned_active").is_none(),
        "legacy no-coefficient default path must not attach active routing"
    );
    assert!(!pass_off.is_empty(), "shadow-off HBP should be written");
    assert!(
        !parquet_off.is_empty(),
        "shadow-off pass parquet should be written"
    );

    let err = run_h2637("on", true)
        .expect_err("legacy H2637 must fail closed when Lane D shadow lacks routing coefficients");
    clear_laned_selector_env();
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
// D15A/D16 (SC-OFEROUTE-001 rev 27/rev 46): ACTIVE production owner.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum NativeActiveSelector {
    Default,
    ExplicitActive,
    ExplicitDisable,
}

fn run_h2637_native_active(
    tag: &str,
    selector: NativeActiveSelector,
) -> Result<(PathBuf, serde_json::Value, Vec<u8>, Vec<u8>), HillslopeCliError> {
    let run_dir = copy_fixture_to_temp(tag);
    enable_native_cropland_routing_coefficients(&run_dir);
    let output_dir = run_dir.join("output");
    let manifest_path = run_dir.join("manifest.json");
    // CR-M2/T3-QA-M3: neutralize the sibling selectors (inherited shell
    // state) plus the abandoned implicit selector, so the "plain active"
    // evidence leg cannot fail on stale operator environment.
    clear_laned_selector_env();
    match selector {
        NativeActiveSelector::Default => {}
        NativeActiveSelector::ExplicitActive => {
            // SAFETY: single-threaded test setup before any runner threads.
            unsafe { std::env::set_var("OPENWEPP_LANED_ACTIVE", "1") };
        }
        NativeActiveSelector::ExplicitDisable => {
            // SAFETY: as above.
            unsafe { std::env::set_var("OPENWEPP_LANED_ACTIVE_DISABLE", "1") };
        }
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
    clear_laned_selector_env();
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
    let _cleanup = LaneDSelectorEnvCleanup;
    clear_laned_selector_env();
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
    let _cleanup = LaneDSelectorEnvCleanup;
    clear_laned_selector_env();
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
    let message = report
        .expect_err("active + shadow must fail closed")
        .to_string();
    assert!(
        message.contains("mutually exclusive"),
        "expected mutual-exclusion error, got {message}"
    );
}

#[test]
fn h2637_active_and_disable_are_mutually_exclusive() {
    let run_dir = copy_fixture_to_temp("active_disable_conflict");
    enable_native_cropland_routing_coefficients(&run_dir);
    let output_dir = run_dir.join("output");
    let _cleanup = LaneDSelectorEnvCleanup;
    clear_laned_selector_env();
    // SAFETY: single-threaded test setup before any runner threads.
    unsafe { std::env::set_var("OPENWEPP_LANED_ACTIVE", "1") };
    // SAFETY: as above.
    unsafe { std::env::set_var("OPENWEPP_LANED_ACTIVE_DISABLE", "1") };
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
    let message = report
        .expect_err("active + disable must fail closed")
        .to_string();
    assert!(
        message.contains("mutually exclusive"),
        "expected mutual-exclusion error, got {message}"
    );
}

fn run_h2637_default_expect_error(run_dir: &Path) -> String {
    let output_dir = run_dir.join("output");
    let _cleanup = LaneDSelectorEnvCleanup;
    clear_laned_selector_env();
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: run_dir.to_path_buf(),
            run_file: PathBuf::from("p2637.run.toml"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: Some(run_dir.join("manifest.json")),
        },
        &["openwepp-cli-hill".to_string()],
    );
    report
        .expect_err("default run must fail closed")
        .to_string()
}

#[test]
fn h2637_default_mixed_routing_coefficients_fails_closed() {
    let run_dir = copy_fixture_to_temp("default_mixed_coefficients");
    enable_native_cropland_routing_coefficients(&run_dir);
    remove_first_native_routing_coefficient_block(&run_dir);
    let message = run_h2637_default_expect_error(&run_dir);
    assert!(
        message.contains("conditional Lane D default activation"),
        "expected conditional default error, got {message}"
    );
    assert!(
        message.contains("with coefficients") && message.contains("without coefficients"),
        "expected mixed-authority counts, got {message}"
    );
}

#[test]
fn h2637_default_malformed_routing_coefficients_fails_closed() {
    let run_dir = copy_fixture_to_temp("default_malformed_coefficients");
    enable_native_cropland_routing_coefficients(&run_dir);
    truncate_first_native_routing_coefficient_block(&run_dir);
    let message = run_h2637_default_expect_error(&run_dir);
    assert!(
        message.contains("routing_coefficients"),
        "expected malformed routing_coefficients error, got {message}"
    );
}

#[test]
#[ignore = "D16 H2637 active-owner evidence: runs the full H2637 fixture three times"]
#[allow(clippy::too_many_lines)]
fn h2637_native_active_owner_routes_and_closes() {
    // Explicit disable on the SAME native-patched fixture: no active keys. This
    // test asserts only presence/absence; the INV-OFEROUTE-010 BYTE
    // comparison itself lives in the P4 gate evidence (SHA256 vs the
    // recorded package baseline), not in this test.
    let (_dir_off, manifest_off, pass_off, parquet_off) =
        run_h2637_native_active("active_off", NativeActiveSelector::ExplicitDisable)
            .expect("native-routed H2637 must run with the active owner disabled");
    assert!(
        find_key(&manifest_off, "laned_active").is_none(),
        "no active keys when the active owner is off"
    );
    assert!(!pass_off.is_empty() && !parquet_off.is_empty());

    // DEFAULT ACTIVE: coefficient-complete native management should attach
    // the active owner without OPENWEPP_LANED_ACTIVE=1.
    let (_dir_default, manifest_default, pass_default, parquet_default) =
        run_h2637_native_active("active_default", NativeActiveSelector::Default)
            .expect("native-routed H2637 must run active by default");

    // EXPLICIT ACTIVE: same owner as the default path.
    let (_dir_on, manifest_on, pass_on, parquet_on) =
        run_h2637_native_active("active_on", NativeActiveSelector::ExplicitActive)
            .expect("native-routed H2637 must run with the active owner enabled");
    assert_eq!(
        pass_default, pass_on,
        "default active and explicit active should write identical HBP bytes"
    );
    assert_eq!(
        parquet_default, parquet_on,
        "default active and explicit active should write identical pass parquet bytes"
    );
    let active_default =
        find_key(&manifest_default, "laned_active").expect("default manifest laned_active block");
    let default_days_routed = find_key(active_default, "days_routed")
        .and_then(serde_json::Value::as_u64)
        .expect("default days_routed");
    assert!(
        default_days_routed > 0,
        "default active must route event days"
    );
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
    let initial_groundwater_storage_m3 = find_key(active, "initial_groundwater_storage_m3")
        .and_then(serde_json::Value::as_f64)
        .expect("enabled run must publish initial groundwater storage");
    let terminal_groundwater_storage_m3 = find_key(active, "terminal_groundwater_storage_m3")
        .and_then(serde_json::Value::as_f64)
        .expect("enabled run must publish terminal groundwater storage");
    let terminal_groundwater_baseflow_m3 = find_key(active, "terminal_groundwater_baseflow_m3")
        .and_then(serde_json::Value::as_f64)
        .expect("enabled run must publish terminal groundwater baseflow");
    let terminal_groundwater_deep_seepage_m3 =
        find_key(active, "terminal_groundwater_deep_seepage_m3")
            .and_then(serde_json::Value::as_f64)
            .expect("enabled run must publish terminal groundwater deep seepage");
    let total_groundwater_recharge_m3 = find_key(active, "total_groundwater_recharge_m3")
        .and_then(serde_json::Value::as_f64)
        .expect("total_groundwater_recharge_m3");
    let total_groundwater_baseflow_m3 = find_key(active, "total_groundwater_baseflow_m3")
        .and_then(serde_json::Value::as_f64)
        .expect("total_groundwater_baseflow_m3");
    let total_groundwater_deep_seepage_m3 = find_key(active, "total_groundwater_deep_seepage_m3")
        .and_then(serde_json::Value::as_f64)
        .expect("total_groundwater_deep_seepage_m3");
    let recurrence_terminal_m3 = initial_groundwater_storage_m3 + total_groundwater_recharge_m3
        - (total_groundwater_baseflow_m3 - terminal_groundwater_baseflow_m3)
        - (total_groundwater_deep_seepage_m3 - terminal_groundwater_deep_seepage_m3);
    assert!(
        (terminal_groundwater_storage_m3 - recurrence_terminal_m3).abs()
            <= 1.0e-9 * terminal_groundwater_storage_m3.max(1.0),
        "published terminal storage must reconstruct with recurrence timing"
    );
    let post_export_storage_m3 = terminal_groundwater_storage_m3
        - terminal_groundwater_baseflow_m3
        - terminal_groundwater_deep_seepage_m3;
    let full_run_ledger_storage_m3 = initial_groundwater_storage_m3 + total_groundwater_recharge_m3
        - total_groundwater_baseflow_m3
        - total_groundwater_deep_seepage_m3;
    assert!(
        (post_export_storage_m3 - full_run_ledger_storage_m3).abs()
            <= 1.0e-9 * post_export_storage_m3.abs().max(1.0),
        "published groundwater operands must close the full run ledger"
    );
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
