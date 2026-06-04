use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("openwepp_{name}_{nanos}"))
}

fn run_guard(path: &Path) -> std::process::Output {
    Command::new("python3")
        .arg(repo_root().join("tools/release/check_raw_unit_conversions.py"))
        .arg("--path")
        .arg(path)
        .output()
        .expect("raw unit conversion guard should execute")
}

#[test]
fn hphys0276_raw_unit_conversion_guard_rejects_unauthorized_literal() {
    let dir = unique_temp_dir("hphys0276_bad");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let source = dir.join("bad.rs");
    fs::write(
        &source,
        "fn convert(rad_ly_d: f64) -> f64 { rad_ly_d * 0.04184 }\n",
    )
    .expect("fixture should be written");

    let output = run_guard(&dir);
    assert!(
        !output.status.success(),
        "raw guard should reject unauthorized literal; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("langley_to_mj_m2"),
        "stderr should identify literal class: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0276_raw_unit_conversion_guard_rejects_equivalent_rust_spellings() {
    let dir = unique_temp_dir("hphys0276_alt");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let source = dir.join("alternate.rs");
    fs::write(
        &source,
        "fn conversions(rad_ly_d: f64, depth_m: f64, seconds: f64) -> f64 {
    (rad_ly_d * 0.041_84) + (depth_m * 1e3) + (seconds * 0.000_277_78)
}
fn suffixes(wind_m_s: f64) -> f64 {
    (wind_m_s * 3600f64) / 1_609_f64
}
",
    )
    .expect("fixture should be written");

    let output = run_guard(&dir);
    assert!(
        !output.status.success(),
        "raw guard should reject equivalent spellings; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    for literal_class in [
        "langley_to_mj_m2",
        "mm_m_scale",
        "hour_second_scale",
        "legacy_snow_melt_scale",
    ] {
        assert!(
            stderr.contains(literal_class),
            "stderr should contain {literal_class}: {stderr}"
        );
    }

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0276_raw_unit_conversion_guard_accepts_helper_based_source() {
    let dir = unique_temp_dir("hphys0276_good");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let source = dir.join("good.rs");
    fs::write(
        &source,
        "fn convert(rad_ly_d: f64) -> Result<f64, openwepp_unit_boundary::BoundaryError> {
    openwepp_unit_boundary::conversions::langleys_per_day_to_megajoules_per_square_meter_per_day(rad_ly_d)
}
",
    )
    .expect("fixture should be written");

    let output = run_guard(&dir);
    assert!(
        output.status.success(),
        "raw guard should accept helper-based source; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0276_raw_unit_conversion_guard_does_not_overapply_allow_marker() {
    let dir = unique_temp_dir("hphys0276_allow_scope");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let source = dir.join("allow_scope.rs");
    fs::write(
        &source,
        "// UNIT-CONVERSION-ALLOW: mm_m_scale numerical convergence threshold, not unit conversion.
fn bad(rad_ly_d: f64) -> f64 { rad_ly_d * 0.04184 }
",
    )
    .expect("fixture should be written");

    let output = run_guard(&dir);
    assert!(
        !output.status.success(),
        "allow marker for one class must not hide another; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("langley_to_mj_m2"),
        "stderr should identify unauthorized class: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0276_raw_unit_conversion_guard_accepts_documented_exception() {
    let dir = unique_temp_dir("hphys0276_allow");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let source = dir.join("allow.rs");
    fs::write(
        &source,
        "// UNIT-CONVERSION-ALLOW: mm_m_scale numerical convergence threshold, not unit conversion.
fn threshold(value: f64) -> bool { value > 0.001 }
",
    )
    .expect("fixture should be written");

    let output = run_guard(&dir);
    assert!(
        output.status.success(),
        "raw guard should accept documented exception; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}
