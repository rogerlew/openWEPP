#[cfg(feature = "test-fixture-authority")]
use openwepp_runner::{
    HillslopeRunRequest, SidecarPolicy, Stage3TestFixtureSeedBinding, Stage3TestFixtureSeedProfile,
    author_stage3_v11_owner_seed_fixture,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

fn watershed_execution_lock() -> &'static Mutex<()> {
    static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    RUN_LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn wshedw10_watershed_cli_absent_chaninp_uses_typed_legacy_defaults() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw10_absent_chaninp_default");
    write_hillslope_source_runfile_fixture(&run_dir, 1);
    bind_stage3_owner_seed(&run_dir, 1);
    write_watershed_runfile_without_chaninp(&run_dir);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir);
    assert!(
        output.status.success(),
        "watershed CLI should apply typed WSHED-W10 chan.inp defaults; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("sidecar-warning: chan.inp CHN-W-001"),
        "expected typed chan.inp missing-default warning in stderr, observed: {stderr}"
    );
    assert!(
        !stderr.contains("dtchr=3600"),
        "old hidden channel-global fallback must not appear in stderr: {stderr}"
    );
    assert!(
        !stderr.contains("CLIWAT-E-029"),
        "unconfigured absent chan.inp should not be treated as bad configured path: {stderr}"
    );
    assert_all_watershed_outputs_exist(&output_dir);
}

#[cfg(feature = "test-fixture-authority")]
fn bind_stage3_owner_seed(run_dir: &Path, hillslope_id: u32) {
    author_stage3_v11_owner_seed_fixture(
        &HillslopeRunRequest {
            run_dir: run_dir.to_path_buf(),
            run_file: PathBuf::from(format!("H{hillslope_id}.source.run")),
            output_dir: run_dir.join("stage3-fixture-output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        Stage3TestFixtureSeedProfile::CompleteOwner,
        Stage3TestFixtureSeedBinding::ExplicitRunfile,
    )
    .expect("WSHED-W10 fixture should bind an exact explicit Stage-3 owner seed");
}

#[cfg(not(feature = "test-fixture-authority"))]
fn bind_stage3_owner_seed(_run_dir: &Path, _hillslope_id: u32) {}

fn build_watershed_fixture_dir(prefix: &str) -> PathBuf {
    let destination = unique_temp_dir(prefix);
    fs::create_dir_all(&destination).expect("fixture directory should be creatable");

    fs::write(destination.join("pw0.str"), "94.301\n2 1 0 0 0 0 0 0 0 0\n")
        .expect("channel-only structure fixture should be writable");
    copy_fixture_file(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/infile/watershed_channel/strict_sidecar_required.chn"),
        &destination.join("pw0.chn"),
    );
    copy_fixture_file(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/infile/watershed_impoundment/strict_valid_minimal.imp"),
        &destination.join("pw0.imp"),
    );
    copy_fixture_file(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/cli01/hillslope_run_dir/case.man"),
        &destination.join("pw0.man"),
    );
    copy_fixture_file(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/cli01/hillslope_run_dir/case.slp"),
        &destination.join("pw0.slp"),
    );
    copy_fixture_file(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/cli01/hillslope_run_dir/case.cli"),
        &destination.join("pw0.cli"),
    );
    copy_fixture_file(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/cli01/hillslope_run_dir/case.sol"),
        &destination.join("pw0.sol"),
    );

    destination
}

fn write_hillslope_source_runfile_fixture(run_dir: &Path, hillslope_id: u32) {
    let fixture_root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/cli01/hillslope_run_dir");
    for (source, destination) in [
        ("case.man", format!("H{hillslope_id}.man")),
        ("case.slp", format!("H{hillslope_id}.slp")),
        ("case.cli", format!("H{hillslope_id}.cli")),
        ("case.sol", format!("H{hillslope_id}.sol")),
        ("pmetpara.txt", "pmetpara.txt".to_string()),
        ("snow.txt", "snow.txt".to_string()),
        ("frost.txt", "frost.txt".to_string()),
        ("wepp_ui.txt", "wepp_ui.txt".to_string()),
    ] {
        copy_fixture_file(&fixture_root.join(source), &run_dir.join(destination));
    }

    let payload = format!(
        r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "wshedw10-hillslope-{hillslope_id}"
unit_system = "metric"

[inputs]
soil = "H{hillslope_id}.sol"
management = "H{hillslope_id}.man"
slope = "H{hillslope_id}.slp"
climate = "H{hillslope_id}.cli"
wepp_ui = true
pmetpara = "pmetpara.txt"

[outputs]
pass = "unused/H{hillslope_id}.hbp"
loss = "unused/H{hillslope_id}.loss.json"
wat = "unused/H{hillslope_id}.wat.parquet"
plot = "unused/H{hillslope_id}.plot.parquet"
"#
    );
    fs::write(run_dir.join(format!("H{hillslope_id}.source.run")), payload)
        .expect("hillslope source runfile should be writable");
}

fn write_watershed_runfile_without_chaninp(run_dir: &Path) {
    let payload = r#"
schema = "openwepp-watershed-runfile-v1"
run_name = "wshedw10-absent-chaninp-contract"
unit_system = "metric"

[inputs]
pw0_str = "pw0.str"
pw0_chn = "pw0.chn"
pw0_imp = "pw0.imp"
pw0_man = "pw0.man"
pw0_slp = "pw0.slp"
pw0_cli = "pw0.cli"
pw0_sol = "pw0.sol"

[inputs.applicability]
chapter13_small_watershed_intent = true
allow_partial_area_response = false
allow_headcutting = false
allow_bank_sloughing = false
allow_perennial_streams = false

[[inputs.hillslopes_block]]
hillslope_id = 1
run_file = "H1.source.run"
use_existing_pass_file = false

[outputs]
ebe_pw0 = "interchange/ebe_pw0.parquet"
chan_out = "interchange/chan.out.parquet"
chanwb = "interchange/chanwb.parquet"
chnwb = "interchange/chnwb.parquet"
soil_pw0 = "interchange/soil_pw0.parquet"
totalwatsed3 = "interchange/totalwatsed3.parquet"
loss_hill = "interchange/loss_pw0.hill.parquet"
loss_chn = "interchange/loss_pw0.chn.parquet"
loss_out = "interchange/loss_pw0.out.parquet"
loss_class_data = "interchange/loss_pw0.class_data.parquet"
loss_all_years_hill = "interchange/loss_pw0.all_years.hill.parquet"
loss_all_years_chn = "interchange/loss_pw0.all_years.chn.parquet"
loss_all_years_out = "interchange/loss_pw0.all_years.out.parquet"
loss_all_years_class_data = "interchange/loss_pw0.all_years.class_data.parquet"
"#;
    fs::write(run_dir.join("case.run"), payload).expect("runfile should be writable");
}

fn run_watershed_cli(run_dir: &Path, output_dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_openwepp-cli-watershed"))
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")))
        .arg("--run-dir")
        .arg(run_dir)
        .arg("--run-file")
        .arg("case.run")
        .arg("--output-dir")
        .arg(output_dir)
        .arg("--policy")
        .arg("compat")
        .arg("--hillslope-binary")
        .arg(env!("CARGO_BIN_EXE_openwepp-cli-hill"))
        .output()
        .expect("watershed CLI process should execute")
}

fn assert_all_watershed_outputs_exist(output_dir: &Path) {
    for output_name in [
        "ebe_pw0.parquet",
        "chan.out.parquet",
        "chanwb.parquet",
        "chnwb.parquet",
        "soil_pw0.parquet",
        "totalwatsed3.parquet",
        "loss_pw0.hill.parquet",
        "loss_pw0.chn.parquet",
        "loss_pw0.out.parquet",
        "loss_pw0.class_data.parquet",
        "loss_pw0.all_years.hill.parquet",
        "loss_pw0.all_years.chn.parquet",
        "loss_pw0.all_years.out.parquet",
        "loss_pw0.all_years.class_data.parquet",
    ] {
        let output_path = output_dir.join("interchange").join(output_name);
        assert!(
            output_path.is_file(),
            "missing expected watershed parquet output {}",
            output_path.display()
        );
    }
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix epoch should be before now")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}_{timestamp}"))
}

fn copy_fixture_file(source: &Path, destination: &Path) {
    fs::copy(source, destination).unwrap_or_else(|error| {
        panic!(
            "fixture copy should succeed ({} -> {}): {error}",
            source.display(),
            destination.display()
        )
    });
}
