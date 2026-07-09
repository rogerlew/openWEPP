#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::match_same_arms,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

use std::fmt::Write as _;
use std::fs::{self, File};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

use openwepp_input_contract::parsers::hbp::{
    HbpParseOptions, parse_hbp_from_path_with_latest_event_payload,
};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};

const MAGIC: &[u8; 8] = b"WFPHBP01";
const FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
const SUPPORTED_MAJOR_V1: u16 = 1;
const DIM_SCALAR: u8 = 0;
const DIM_NOFE: u8 = 1;
const DIM_NOFE_LAYERS: u8 = 2;
const SCALE_INV_I64: f64 = 1.0e9;
const DELICATE_GAME_BASELINE_EBE_PATH: &str = "/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/outputs/wepp_dcc52a6/ebe_pw0.txt";

const REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 300, 900, 901,
];

#[derive(Debug, Clone, Copy, PartialEq)]
struct BaselineEbeDailyRow {
    day_of_month: i8,
    month: i8,
    simulation_year: i16,
    runoff_volume_m3: f64,
    peak_runoff_m3_s: f64,
}

#[test]
fn watershed_cli_rejects_negative_hbp_payload_via_ws10_domain_guards() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_negative_payload");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        -0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_hbp_fixture(
        run_dir.join("H2.hbp"),
        2,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1, 2]);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, None, false);
    assert!(
        !output.status.success(),
        "watershed CLI should fail on domain-invalid contributor payload"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-019") || stderr.contains("CLIWAT-E-020"),
        "expected watershed execution wrapper failure code in stderr, observed: {stderr}"
    );
    assert!(
        stderr.contains("WKERNEL-WS10-CHANNEL-E-003")
            || stderr.contains("WKERNEL-WS10-IMPOUNDMENT-E-003"),
        "expected WS10 domain guard message in stderr, observed: {stderr}"
    );
}

#[test]
fn watershed_cli_emits_watershed_output_parquet_files() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_output_guard");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1]);
    prepare_output_guard_fixture(&run_dir);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        output.status.success(),
        "watershed CLI should emit watershed parquet outputs; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_all_watershed_outputs_exist(&output_dir);
}

#[test]
fn wshedw2_watershed_cli_rejects_zero_negative_and_invalid_jobs_values() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw2_invalid_jobs");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1]);

    let output_dir = run_dir.join("out");
    let zero = run_watershed_cli_with_options(
        &run_dir,
        &output_dir,
        Some("compat"),
        false,
        Some("0"),
        None,
    );
    assert!(
        !zero.status.success(),
        "watershed CLI should reject --jobs 0"
    );
    let zero_stderr = String::from_utf8_lossy(&zero.stderr);
    assert!(
        zero_stderr.contains("CLIWAT-E-041"),
        "expected jobs validator code for --jobs 0, observed: {zero_stderr}"
    );

    let invalid = run_watershed_cli_with_options(
        &run_dir,
        &output_dir,
        Some("compat"),
        false,
        Some("not-a-number"),
        None,
    );
    assert!(
        !invalid.status.success(),
        "watershed CLI should reject non-integer --jobs"
    );
    let invalid_stderr = String::from_utf8_lossy(&invalid.stderr);
    assert!(
        invalid_stderr.contains("CLIWAT-E-041"),
        "expected jobs validator code for invalid --jobs, observed: {invalid_stderr}"
    );

    let negative = run_watershed_cli_with_options(
        &run_dir,
        &output_dir,
        Some("compat"),
        false,
        Some("-1"),
        None,
    );
    assert!(
        !negative.status.success(),
        "watershed CLI should reject negative --jobs"
    );
    let negative_stderr = String::from_utf8_lossy(&negative.stderr);
    assert!(
        negative_stderr.contains("CLIWAT-E-041"),
        "expected jobs validator code for negative --jobs, observed: {negative_stderr}"
    );
}

#[test]
fn wshedw2_watershed_cli_serial_supervisor_generates_pass_inventory_and_routes() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw2_serial_supervisor");
    prepare_output_guard_fixture(&run_dir);
    write_hillslope_source_runfile_fixture(&run_dir, 1);
    write_generated_watershed_runfile(&run_dir, &[1]);

    let output_dir = run_dir.join("out");
    let hill_binary = Path::new(env!("CARGO_BIN_EXE_openwepp-cli-hill"));
    let output = run_watershed_cli_with_options(
        &run_dir,
        &output_dir,
        Some("compat"),
        false,
        Some("1"),
        Some(hill_binary),
    );
    assert!(
        output.status.success(),
        "watershed CLI should execute generated hillslope job, validate pass inventory, and route; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let job_root = output_dir.join("hillslope-jobs/H1");
    for relative in [
        "H1.run.toml",
        "H1.hbp",
        "H1.manifest.json",
        "H1.stdout.log",
        "H1.stderr.log",
        "H1.timing.json",
    ] {
        let path = job_root.join(relative);
        assert!(
            path.is_file(),
            "serial supervisor should write per-job artifact {}",
            path.display()
        );
    }
    assert!(
        !run_dir.join("H1.hbp").exists(),
        "generated mode should not rely on a pre-existing pass in the shared run directory"
    );
    assert_all_watershed_outputs_exist(&output_dir);

    let ebe_row = read_first_parquet_row(&output_dir.join("interchange/ebe_pw0.parquet"));
    let emitted_peak = row_f64_value(&ebe_row, "peak_runoff");
    let emitted_runoff_volume = row_f64_value(&ebe_row, "runoff_volume");
    assert!(
        emitted_peak > 0.0 && emitted_runoff_volume > 0.0,
        "watershed output should consume non-zero generated pass payload; peak={emitted_peak}, runoff_volume={emitted_runoff_volume}"
    );
}

#[test]
fn wshedw2_watershed_cli_generated_mode_accepts_relative_output_dir() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw2_relative_output_dir");
    prepare_output_guard_fixture(&run_dir);
    write_hillslope_source_runfile_fixture(&run_dir, 1);
    write_generated_watershed_runfile(&run_dir, &[1]);

    let relative_output_dir = Path::new("relative-out");
    let hill_binary = Path::new(env!("CARGO_BIN_EXE_openwepp-cli-hill"));
    let output = run_watershed_cli_with_current_dir(
        &run_dir,
        relative_output_dir,
        Some("compat"),
        false,
        Some("1"),
        Some(hill_binary),
        &run_dir,
    );
    assert!(
        output.status.success(),
        "generated watershed run should accept relative --output-dir; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_all_watershed_outputs_exist(&run_dir.join(relative_output_dir));
}

#[test]
fn wshedw7_watershed_cli_generated_mode_accepts_relative_run_dir() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw7_relative_run_dir");
    prepare_output_guard_fixture(&run_dir);
    write_hillslope_source_runfile_fixture(&run_dir, 1);
    write_generated_watershed_runfile(&run_dir, &[1]);

    let parent = run_dir
        .parent()
        .expect("temporary fixture directory should have a parent");
    let relative_run_dir = Path::new(
        run_dir
            .file_name()
            .expect("temporary fixture directory should have a final component"),
    );
    let output_dir = run_dir.join("out-relative-run-dir");
    let hill_binary = Path::new(env!("CARGO_BIN_EXE_openwepp-cli-hill"));
    let output = run_watershed_cli_with_current_dir(
        relative_run_dir,
        &output_dir,
        Some("compat"),
        false,
        Some("1"),
        Some(hill_binary),
        parent,
    );
    assert!(
        output.status.success(),
        "generated watershed run should accept relative --run-dir and canonicalize child inputs; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_all_watershed_outputs_exist(&output_dir);
}

#[test]
fn wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let run_dir = repo_root.join("tests/fixtures/watershed/p102-sediment-active/runs");
    let hill_binary = Path::new(env!("CARGO_BIN_EXE_openwepp-cli-hill"));
    let jobs1_output_dir = unique_temp_dir("wshedw7r_p102_jobs1");
    let jobs4_output_dir = unique_temp_dir("wshedw7r_p102_jobs4");

    for (jobs, output_dir) in [("1", &jobs1_output_dir), ("4", &jobs4_output_dir)] {
        let output = run_watershed_cli_with_options(
            &run_dir,
            output_dir,
            Some("compat"),
            false,
            Some(jobs),
            Some(hill_binary),
        );
        assert!(
            output.status.success(),
            "p102 sediment-active watershed run should complete for --jobs {jobs}; stderr={}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_all_watershed_outputs_exist(output_dir);
    }

    assert_watershed_outputs_row_equivalent(&jobs1_output_dir, &jobs4_output_dir);

    let (hbp, latest_event_payload) = parse_hbp_from_path_with_latest_event_payload(
        jobs1_output_dir.join("hillslope-jobs/H1/H1.hbp"),
        HbpParseOptions {
            expected_hillslope_id: Some(1),
        },
    )
    .expect("generated p102 HBP should parse through the production HBP parser");
    let payload = latest_event_payload.expect("generated p102 HBP should contain an EventPayload");

    assert_eq!(hbp.schema_major, 1);
    assert_eq!(hbp.schema_minor, 1);
    assert_eq!(hbp.nofe, 2);
    assert_eq!(hbp.npart, 5);
    assert_eq!(payload.hourly_runoff_volume_m3.len(), 24);
    assert_eq!(payload.hourly_sediment_mass_kg.len(), 24);
    assert!(payload.total_detachment_kg > 0.0);
    assert!(payload.total_deposition_kg > 0.0);
    assert!(
        payload
            .particle_flow_fraction
            .iter()
            .any(|fraction| *fraction > 0.0),
        "p102 HBP should carry nonzero sediment class fractions"
    );

    let exported_sediment_kg = payload.total_detachment_kg - payload.total_deposition_kg;
    let hourly_sediment_kg = payload.hourly_sediment_mass_kg.iter().sum::<f64>();
    assert_relative_close(
        hourly_sediment_kg,
        exported_sediment_kg,
        1.0e-9,
        1.0e-6,
        "HBP hourly sediment export",
    );

    let totalwatsed3_row =
        read_first_parquet_row(&jobs1_output_dir.join("interchange/totalwatsed3.parquet"));
    let ebe_row = read_first_parquet_row(&jobs1_output_dir.join("interchange/ebe_pw0.parquet"));
    assert_relative_close(
        row_f64_value(&totalwatsed3_row, "tdet"),
        payload.total_detachment_kg,
        1.0e-9,
        1.0e-6,
        "totalwatsed3 tdet",
    );
    assert_relative_close(
        row_f64_value(&totalwatsed3_row, "tdep"),
        payload.total_deposition_kg,
        1.0e-9,
        1.0e-6,
        "totalwatsed3 tdep",
    );

    let sediment_yield_kg = row_f64_value(&ebe_row, "sediment_yield");
    assert!(sediment_yield_kg > 0.0);
    assert_relative_close(
        row_f64_value(&totalwatsed3_row, "sed_del"),
        sediment_yield_kg,
        1.0e-9,
        1.0e-12,
        "totalwatsed3 sed_del",
    );
    assert!(
        (sediment_yield_kg - exported_sediment_kg).abs() > 1.0,
        "sed_del should be routed sediment yield, not a detachment-minus-deposition alias"
    );
}

#[test]
fn hbp_latest_event_payload_exposes_groundwater_baseflow_and_deep_seepage() {
    let run_dir = build_watershed_fixture_dir("hbp_groundwater_payload");
    let hbp_path = run_dir.join("H1.hbp");
    let bytes = build_schema1_event_fixture_with_groundwater(1, 1, 2.5, 0.75);
    fs::write(&hbp_path, bytes).expect("HBP groundwater fixture should be writable");

    let (_, latest_event_payload) = parse_hbp_from_path_with_latest_event_payload(
        &hbp_path,
        HbpParseOptions {
            expected_hillslope_id: Some(1),
        },
    )
    .expect("groundwater HBP fixture should parse");
    let payload = latest_event_payload.expect("fixture should include EVENT payload");

    assert_relative_close(
        payload.baseflow_volume_m3,
        2.5,
        1.0e-12,
        1.0e-12,
        "HBP gwbfv",
    );
    assert_relative_close(
        payload.deep_seepage_volume_m3,
        0.75,
        1.0e-12,
        1.0e-12,
        "HBP gwdsv",
    );
}

#[test]
#[cfg(unix)]
fn wshedw2_watershed_cli_rejects_stale_generated_pass_when_child_does_not_publish() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw2_stale_generated_pass");
    prepare_output_guard_fixture(&run_dir);
    write_hillslope_source_runfile_fixture(&run_dir, 1);
    write_generated_watershed_runfile(&run_dir, &[1]);

    let output_dir = run_dir.join("out");
    let job_root = output_dir.join("hillslope-jobs/H1");
    fs::create_dir_all(&job_root).expect("stale job root should be writable");
    write_hbp_fixture(
        job_root.join("H1.hbp"),
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_hillslope_manifest_fixture(job_root.join("H1.manifest.json"), 1, 1_800.0);
    let fake_child = write_successful_noop_hillslope_binary(&run_dir);

    let output = run_watershed_cli_with_options(
        &run_dir,
        &output_dir,
        Some("compat"),
        false,
        Some("1"),
        Some(&fake_child),
    );
    assert!(
        !output.status.success(),
        "generated mode should remove stale pass artifacts and fail when child publishes no pass"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-045") && stderr.contains("missing"),
        "expected missing generated pass inventory failure after stale cleanup, observed: {stderr}"
    );
}

#[test]
fn wshedw2_watershed_cli_requires_explicit_reuse_mode() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw2_explicit_reuse");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1]);
    prepare_output_guard_fixture(&run_dir);
    let runfile_path = run_dir.join("case.run");
    let runfile_payload =
        fs::read_to_string(&runfile_path).expect("reuse runfile should be readable");
    fs::write(
        &runfile_path,
        runfile_payload.replace("use_existing_pass_file = true\n", ""),
    )
    .expect("reuse runfile should be mutable");

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        !output.status.success(),
        "watershed CLI should require explicit use_existing_pass_file for reuse mode"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-026") && stderr.contains("use_existing_pass_file"),
        "expected explicit reuse selector failure, observed: {stderr}"
    );
}

#[test]
fn wshedw2_watershed_cli_rejects_ambiguous_reuse_block_with_run_file() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw2_ambiguous_reuse");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_hillslope_source_runfile_fixture(&run_dir, 1);
    write_watershed_runfile(&run_dir, &[1]);
    prepare_output_guard_fixture(&run_dir);
    let runfile_path = run_dir.join("case.run");
    let runfile_payload =
        fs::read_to_string(&runfile_path).expect("reuse runfile should be readable");
    fs::write(
        &runfile_path,
        runfile_payload.replace(
            "use_existing_pass_file = true\n",
            "use_existing_pass_file = true\nrun_file = \"H1.source.run\"\n",
        ),
    )
    .expect("reuse runfile should be mutable");

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        !output.status.success(),
        "watershed CLI should reject reuse blocks that also declare a run_file"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-026") && stderr.contains("cannot combine run_file"),
        "expected ambiguous reuse failure, observed: {stderr}"
    );
}

#[test]
fn wshedw2_watershed_cli_rejects_pass_without_latest_event_payload() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw2_no_latest_event_payload");
    write_hbp_no_event_fixture(run_dir.join("H1.hbp"), 1);
    write_watershed_runfile(&run_dir, &[1]);
    prepare_output_guard_fixture(&run_dir);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        !output.status.success(),
        "watershed CLI should fail closed when pass inventory lacks latest EventPayload"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-045") && stderr.contains("NoEvent"),
        "expected pass-inventory NoEvent authority failure, observed: {stderr}"
    );
}

#[test]
fn wshedw3_watershed_cli_worker_pool_matches_jobs1_outputs_and_isolates_artifacts() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let jobs1_output_dir = run_generated_multi_hillslope_case("wshedw3_jobs1_identity", "1");
    let jobs3_output_dir = run_generated_multi_hillslope_case("wshedw3_jobs3_identity", "3");

    assert_watershed_outputs_row_equivalent(&jobs1_output_dir, &jobs3_output_dir);

    for hillslope_id in [1, 2, 3] {
        let job_root = jobs3_output_dir.join(format!("hillslope-jobs/H{hillslope_id}"));
        for relative in [
            format!("H{hillslope_id}.run.toml"),
            format!("H{hillslope_id}.hbp"),
            format!("H{hillslope_id}.manifest.json"),
            format!("H{hillslope_id}.stdout.log"),
            format!("H{hillslope_id}.stderr.log"),
            format!("H{hillslope_id}.timing.json"),
            format!("H{hillslope_id}.freshness"),
        ] {
            let path = job_root.join(relative);
            assert!(
                path.is_file(),
                "worker pool should isolate generated job artifact {}",
                path.display()
            );
        }
        let timing = fs::read_to_string(job_root.join(format!("H{hillslope_id}.timing.json")))
            .expect("worker timing JSON should be readable");
        assert!(
            timing.contains(r#""worker_concurrency": 3"#),
            "worker timing should record requested bounded concurrency, observed: {timing}"
        );
    }
}

#[test]
#[cfg(unix)]
fn wshedw3_worker_pool_stops_pending_jobs_after_child_failure_and_skips_routing() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw3_child_failure_policy");
    prepare_multi_hillslope_output_guard_fixture(&run_dir);
    for hillslope_id in [1, 2, 3] {
        write_hillslope_source_runfile_fixture(&run_dir, hillslope_id);
    }
    write_generated_watershed_runfile(&run_dir, &[1, 2, 3]);
    let fake_child = write_w3_failure_hillslope_binary(&run_dir);

    let output_dir = run_dir.join("out");
    let stale_pending_root = output_dir.join("hillslope-jobs/H3");
    fs::create_dir_all(&stale_pending_root).expect("stale pending job root should be writable");
    fs::write(
        stale_pending_root.join("H3.stdout.log"),
        "stale stdout from prior run\n",
    )
    .expect("stale pending stdout should be writable");
    fs::write(
        stale_pending_root.join("H3.stderr.log"),
        "stale stderr from prior run\n",
    )
    .expect("stale pending stderr should be writable");
    let output = run_watershed_cli_with_options(
        &run_dir,
        &output_dir,
        Some("compat"),
        false,
        Some("2"),
        Some(&fake_child),
    );
    assert!(
        !output.status.success(),
        "worker pool should fail closed when a child exits non-zero"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-043") && stderr.contains("skipped_pending=1"),
        "expected worker-pool failure with pending job skipped, observed: {stderr}"
    );
    assert!(
        !output_dir.join("interchange/ebe_pw0.parquet").exists(),
        "worker-pool child failure must skip watershed routing/publication"
    );

    assert!(
        output_dir
            .join("hillslope-jobs/H1/H1.timing.json")
            .is_file(),
        "in-flight H1 should be waited on and record timing"
    );
    assert!(
        output_dir
            .join("hillslope-jobs/H2/H2.timing.json")
            .is_file(),
        "failed H2 should record timing before supervisor returns"
    );
    assert!(
        !output_dir.join("hillslope-jobs/H3/H3.stdout.log").exists()
            && !output_dir.join("hillslope-jobs/H3/H3.stderr.log").exists()
            && !output_dir.join("hillslope-jobs/H3/H3.timing.json").exists()
            && !output_dir.join("hillslope-jobs/H3/H3.freshness").exists(),
        "pending H3 should not launch after the first hard failure or retain stale logs"
    );
    assert!(
        !run_dir.join("unexpected-h3-launch.txt").exists(),
        "fake child records H3 launch only if the worker-pool failure policy regresses"
    );
}

#[test]
#[cfg(unix)]
fn wshedw3_worker_pool_removes_stale_generated_passes_and_fails_inventory_before_routing() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshedw3_stale_generated_pass_parallel");
    prepare_multi_hillslope_output_guard_fixture(&run_dir);
    for hillslope_id in [1, 2, 3] {
        write_hillslope_source_runfile_fixture(&run_dir, hillslope_id);
    }
    write_generated_watershed_runfile(&run_dir, &[1, 2, 3]);

    let output_dir = run_dir.join("out");
    for hillslope_id in [1, 2, 3] {
        let job_root = output_dir.join(format!("hillslope-jobs/H{hillslope_id}"));
        fs::create_dir_all(&job_root).expect("stale job root should be writable");
        write_hbp_fixture(
            job_root.join(format!("H{hillslope_id}.hbp")),
            hillslope_id,
            0.25,
            1.0,
            5.0,
            4.0,
            1_800.0,
            1_200.0,
        );
        write_hillslope_manifest_fixture(
            job_root.join(format!("H{hillslope_id}.manifest.json")),
            1,
            1_800.0,
        );
    }

    let fake_child = write_successful_noop_hillslope_binary(&run_dir);
    let output = run_watershed_cli_with_options(
        &run_dir,
        &output_dir,
        Some("compat"),
        false,
        Some("2"),
        Some(&fake_child),
    );
    assert!(
        !output.status.success(),
        "worker pool should fail at pass inventory when children publish no passes"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-045")
            && stderr.contains("missing")
            && !stderr.contains("CLIWAT-E-043"),
        "expected missing generated pass inventory failure after successful worker pool, observed: {stderr}"
    );
    assert!(
        !output_dir.join("interchange/ebe_pw0.parquet").exists(),
        "missing generated pass inventory must skip watershed routing/publication"
    );
    for hillslope_id in [1, 2, 3] {
        let job_root = output_dir.join(format!("hillslope-jobs/H{hillslope_id}"));
        assert!(
            job_root
                .join(format!("H{hillslope_id}.timing.json"))
                .is_file(),
            "successful noop child H{hillslope_id} should be launched before inventory validation"
        );
        assert!(
            !job_root.join(format!("H{hillslope_id}.hbp")).exists()
                && !job_root
                    .join(format!("H{hillslope_id}.manifest.json"))
                    .exists(),
            "stale generated pass artifacts for H{hillslope_id} should be removed before child launch"
        );
    }
}

#[test]
fn wshedw5_public_cli_uses_typed_network_and_publication_frames() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let cli_source = fs::read_to_string(
        repo_root.join("crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs"),
    )
    .expect("watershed CLI source should be readable");
    let frame_source = fs::read_to_string(
        repo_root.join("crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs"),
    )
    .expect("watershed network frame source should be readable");
    let dispatch_source = fs::read_to_string(
        repo_root.join("crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs"),
    )
    .expect("watershed dispatch source should be readable");
    let direct_kernel_source = fs::read_to_string(
        repo_root.join("crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs"),
    )
    .expect("direct watershed kernel source should be readable");
    let kernel_helpers_source = fs::read_to_string(
        repo_root.join("crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs"),
    )
    .expect("watershed kernel helper source should be readable");
    let kernel_diagnostics_source = fs::read_to_string(
        repo_root.join("crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs"),
    )
    .expect("watershed kernel diagnostic source should be readable");
    let kernel_validation_source = fs::read_to_string(
        repo_root.join("crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs"),
    )
    .expect("watershed kernel validation source should be readable");
    let kernel_routing_source = fs::read_to_string(
        repo_root.join("crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs"),
    )
    .expect("watershed kernel routing source should be readable");
    let types_source = fs::read_to_string(
        repo_root.join("crates/openwepp-watershed-orchestrator/src/lib_mod/types.rs"),
    )
    .expect("watershed type source should be readable");
    let kernel_core_source = fs::read_to_string(
        repo_root.join("crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/kernel_core.rs"),
    )
    .expect("watershed kernel core source should be readable");
    let runtime_inputs_source = fs::read_to_string(
        repo_root.join("crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs"),
    )
    .expect("watershed runtime input source should be readable");

    for required in [
        "WatershedNetworkFrame::from_parsed_inputs",
        "network_frame.add_hillslope_contribution",
        "execute_watershed_dispatch_with_frame",
        "publish_typed_routing_report",
        "write_typed_publication_parquet_outputs",
    ] {
        assert!(
            cli_source.contains(required),
            "public watershed CLI should contain typed watershed handoff marker {required}"
        );
    }

    for forbidden in [
        "BoundarySymbol",
        "BoundaryValue",
        "WatershedWritebackSurface",
        "compatibility_writeback_surface",
        "harvest_compatibility_routing_report",
        "execute_watershed_dispatch_with_kernel",
        "build_watershed_output_row_seed",
        "publication_frame_to_row_seed",
        "write_watershed_interchange_outputs",
        "WatershedInterchangeRowSeed",
        "build_default_chaninp_surface",
        ".writeback_surface",
        "state_surface.insert",
        "flux_surface.insert",
    ] {
        assert!(
            !cli_source.contains(forbidden),
            "public watershed CLI should not directly use old symbol-map surface marker {forbidden}"
        );
    }

    for required in [
        "pub struct WatershedNetworkFrame",
        "pub struct WatershedPublicationFrame",
        "pub struct HillslopeContribution",
        "pub fn publish_typed_routing_report",
        "collect_dispatch_ids_from_steps",
    ] {
        assert!(
            frame_source.contains(required),
            "typed watershed frame source should contain frame-native marker {required}"
        );
    }

    let typed_dispatch_body = source_body(
        &dispatch_source,
        "pub fn execute_watershed_dispatch_with_frame",
        "type DependencyMap",
    );
    for forbidden in [
        "WatershedWritebackSurface",
        "BoundarySymbol",
        "BoundaryValue",
        "KernelWritebackPayload",
        "WatershedKernelRequest",
        "state_surface",
        "flux_surface",
    ] {
        assert!(
            !typed_dispatch_body.contains(forbidden),
            "typed production dispatch body should not use old surface marker {forbidden}"
        );
        assert!(
            !direct_kernel_source.contains(forbidden),
            "direct watershed kernel source should not use old surface marker {forbidden}"
        );
    }

    for (label, source) in [
        ("frame", frame_source.as_str()),
        ("dispatch", dispatch_source.as_str()),
        ("types", types_source.as_str()),
        ("kernel_core", kernel_core_source.as_str()),
        ("kernel_helpers", kernel_helpers_source.as_str()),
        ("kernel_diagnostics", kernel_diagnostics_source.as_str()),
        ("kernel_validation", kernel_validation_source.as_str()),
        ("kernel_routing", kernel_routing_source.as_str()),
        ("kernel_direct", direct_kernel_source.as_str()),
        ("runtime_inputs", runtime_inputs_source.as_str()),
    ] {
        for forbidden in [
            "WatershedWritebackSurface",
            "WatershedKernelExecutionReport",
            "WatershedKernelStepReport",
            "WatershedKernelRequest",
            "impl WatershedKernel for Ws10ChannelImpoundmentKernel",
            "execute_watershed_dispatch_with_kernel",
            "execute_watershed_dispatch_with_gate_and_kernel",
            "compatibility_writeback_surface",
            "harvest_compatibility_routing_report",
            "build_watershed_runtime_surface",
            "seed_watershed_runtime_surface",
            "WatershedClimateRuntime",
        ] {
            assert!(
                !source.contains(forbidden),
                "{label} source should not retain deleted watershed runtime marker {forbidden}"
            );
        }
    }

    for required in [
        "compute_muskingum_cunge_state",
        "compute_variable_muskingum_cunge_state",
        "route_impoundment_stage_over_duration",
        "impoundment_outflow_at_stage",
        "ws18_trncap",
        "ws20_route_case12_segment_family_core",
        "direct_ws20_crfrac",
    ] {
        assert!(
            direct_kernel_source.contains(required),
            "direct watershed kernel should call actual routing physics marker {required}"
        );
    }
}

#[test]
fn watershed_cli_accepts_explicit_zero_impoundment_file_when_structure_has_none() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_zero_impoundments");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1]);
    prepare_output_guard_fixture(&run_dir);
    fs::write(run_dir.join("pw0.imp"), "99.1\n0\n")
        .expect("zero-impoundment fixture should be writable");

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "watershed CLI should accept explicit jpond=0 when structure declares no impoundments; stderr={stderr}"
    );
    assert!(
        !stderr.contains("CLIWAT-E-010"),
        "no-impoundment input should proceed past the impoundment parse seam; stderr={stderr}"
    );
    assert_all_watershed_outputs_exist(&output_dir);
}

#[test]
fn wshed03_watershed_cli_end_to_end_vector_requires_non_stub_parquet_emission() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("wshed03_cli_non_stub_parquet");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1]);
    prepare_output_guard_fixture(&run_dir);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        output.status.success(),
        "watershed CLI should complete and emit non-placeholder parquet outputs once WSHED08 lands; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_all_watershed_outputs_exist(&output_dir);
}

#[test]
fn wshedimpl14_baseline_authoritative_cli_lane_replays_baseline_ebe_signature() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let baseline_path = Path::new(DELICATE_GAME_BASELINE_EBE_PATH);
    if !baseline_path.is_file() {
        eprintln!(
            "WSHEDIMPL14 comparator lane skipped: missing baseline fixture {}",
            baseline_path.display()
        );
        return;
    }
    let baseline_row = parse_baseline_ebe_first_row(baseline_path);
    assert!(
        baseline_row.peak_runoff_m3_s > 0.0,
        "baseline comparator seed requires positive peak runoff"
    );

    let duration_seconds = baseline_row.runoff_volume_m3 / baseline_row.peak_runoff_m3_s;
    assert!(
        duration_seconds.is_finite() && duration_seconds > 0.0,
        "derived baseline comparator duration must be finite and positive; observed {duration_seconds}"
    );

    let run_dir = build_watershed_fixture_dir("wshedimpl14_baseline_comparator_lane");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        0.25,
        1.0,
        baseline_row.peak_runoff_m3_s,
        duration_seconds,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1]);
    prepare_output_guard_fixture(&run_dir);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        output.status.success(),
        "baseline-authoritative watershed CLI lane should complete successfully; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_all_watershed_outputs_exist(&output_dir);

    let ebe_row = read_first_parquet_row(&output_dir.join("interchange/ebe_pw0.parquet"));
    let chan_out_row = read_first_parquet_row(&output_dir.join("interchange/chan.out.parquet"));

    let emitted_month = row_i32_value(&ebe_row, "month");
    let emitted_day_of_month = row_i32_value(&ebe_row, "day_of_month");
    let emitted_simulation_year = row_i32_value(&ebe_row, "simulation_year");
    let emitted_sim_day_index = row_i32_value(&ebe_row, "sim_day_index");
    let emitted_peak = row_f64_value(&ebe_row, "peak_runoff");
    let emitted_runoff_volume = row_f64_value(&ebe_row, "runoff_volume");
    let emitted_chan_out_peak = row_f64_value(&chan_out_row, "Peak_Discharge (m^3/s)");

    assert_eq!(
        emitted_month,
        i32::from(baseline_row.month),
        "baseline comparator lane requires month-key continuity"
    );
    assert_eq!(
        emitted_day_of_month,
        i32::from(baseline_row.day_of_month),
        "baseline comparator lane requires day-of-month key continuity"
    );
    assert_eq!(
        emitted_simulation_year,
        i32::from(baseline_row.simulation_year),
        "baseline comparator lane requires simulation-year key continuity"
    );
    assert_eq!(
        emitted_sim_day_index, 1,
        "baseline comparator lane expects single-step topology dispatch for this fixture"
    );
    assert_relative_close(
        emitted_peak,
        baseline_row.peak_runoff_m3_s,
        1.0e-6,
        1.0e-8,
        "baseline comparator peak runoff signature",
    );
    assert_relative_close(
        emitted_runoff_volume,
        baseline_row.runoff_volume_m3,
        1.0e-6,
        1.0e-3,
        "baseline comparator runoff-volume signature",
    );
    assert_relative_close(
        emitted_chan_out_peak,
        emitted_peak,
        1.0e-10,
        1.0e-10,
        "branch-execution publication continuity (chan.out vs ebe peak)",
    );
}

#[test]
fn watershed_cli_rejects_missing_applicability_selector_block() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_missing_applicability");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1]);

    let runfile_path = run_dir.join("case.run");
    let runfile_payload =
        fs::read_to_string(&runfile_path).expect("runfile payload should be readable");
    let mutated_payload = runfile_payload.replace(
        r"
[inputs.applicability]
chapter13_small_watershed_intent = true
allow_partial_area_response = false
allow_headcutting = false
allow_bank_sloughing = false
allow_perennial_streams = false
",
        "\n",
    );
    fs::write(&runfile_path, mutated_payload).expect("runfile payload should be writable");

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, None, false);
    assert!(
        !output.status.success(),
        "watershed CLI should reject runfiles missing applicability selectors"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-040"),
        "expected CLIWAT-E-040 applicability validator failure; stderr={stderr}"
    );
}

#[test]
fn watershed_cli_rejects_disallowed_perennial_stream_selector() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_invalid_applicability");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1]);

    let runfile_path = run_dir.join("case.run");
    let runfile_payload =
        fs::read_to_string(&runfile_path).expect("runfile payload should be readable");
    let mutated_payload = runfile_payload.replace(
        "allow_perennial_streams = false",
        "allow_perennial_streams = true",
    );
    fs::write(&runfile_path, mutated_payload).expect("runfile payload should be writable");

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, None, false);
    assert!(
        !output.status.success(),
        "watershed CLI should reject disallowed perennial stream selector"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-040"),
        "expected CLIWAT-E-040 applicability validator failure; stderr={stderr}"
    );
}

fn watershed_execution_lock() -> &'static Mutex<()> {
    static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    RUN_LOCK.get_or_init(|| Mutex::new(()))
}

fn source_body<'a>(source: &'a str, start_marker: &str, end_marker: &str) -> &'a str {
    let start = source
        .find(start_marker)
        .unwrap_or_else(|| panic!("source should contain start marker {start_marker}"));
    let tail = &source[start..];
    let end = tail
        .find(end_marker)
        .unwrap_or_else(|| panic!("source should contain end marker {end_marker}"));
    &tail[..end]
}

fn parse_baseline_ebe_first_row(path: &Path) -> BaselineEbeDailyRow {
    let payload = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("baseline fixture should be readable: {error}"));
    for line in payload.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some(first) = trimmed.as_bytes().first() else {
            continue;
        };
        if !first.is_ascii_digit() {
            continue;
        }

        let fields: Vec<&str> = trimmed.split_whitespace().collect();
        if fields.len() < 6 {
            continue;
        }

        let day_of_month = fields[0].parse::<i8>();
        let month = fields[1].parse::<i8>();
        let simulation_year = fields[2].parse::<i16>();
        let runoff_volume_m3 = fields[4].parse::<f64>();
        let peak_runoff_m3_s = fields[5].parse::<f64>();
        if let (
            Ok(day_of_month),
            Ok(month),
            Ok(simulation_year),
            Ok(runoff_volume_m3),
            Ok(peak_runoff_m3_s),
        ) = (
            day_of_month,
            month,
            simulation_year,
            runoff_volume_m3,
            peak_runoff_m3_s,
        ) {
            return BaselineEbeDailyRow {
                day_of_month,
                month,
                simulation_year,
                runoff_volume_m3,
                peak_runoff_m3_s,
            };
        }
    }

    panic!(
        "baseline fixture {} does not contain a parseable daily EBE data row",
        path.display()
    );
}

fn read_first_parquet_row(path: &Path) -> Row {
    let file = File::open(path).unwrap_or_else(|error| {
        panic!(
            "parquet output should be readable ({}): {error}",
            path.display()
        )
    });
    let reader = SerializedFileReader::new(file).unwrap_or_else(|error| {
        panic!("parquet output should parse ({}): {error}", path.display())
    });
    let mut rows = reader.get_row_iter(None).unwrap_or_else(|error| {
        panic!(
            "parquet row iterator should open ({}): {error}",
            path.display()
        )
    });
    rows.next()
        .unwrap_or_else(|| {
            panic!(
                "expected at least one row in parquet output {}",
                path.display()
            )
        })
        .unwrap_or_else(|error| {
            panic!(
                "first parquet row should decode ({}): {error}",
                path.display()
            )
        })
}

fn row_index(row: &Row, column_name: &str) -> usize {
    row.get_column_iter()
        .enumerate()
        .find(|(_, (name, _))| name.as_str() == column_name)
        .map_or_else(
            || panic!("missing required parquet column '{column_name}'"),
            |(index, _)| index,
        )
}

fn row_f64_value(row: &Row, column_name: &str) -> f64 {
    let index = row_index(row, column_name);
    if let Ok(value) = row.get_double(index) {
        return value;
    }
    if let Ok(value) = row.get_float(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_int(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_short(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_long(index) {
        return value as f64;
    }
    panic!("column '{column_name}' does not decode as numeric");
}

fn row_i32_value(row: &Row, column_name: &str) -> i32 {
    let index = row_index(row, column_name);
    if let Ok(value) = row.get_byte(index) {
        return i32::from(value);
    }
    if let Ok(value) = row.get_int(index) {
        return value;
    }
    if let Ok(value) = row.get_short(index) {
        return i32::from(value);
    }
    if let Ok(value) = row.get_ubyte(index) {
        return i32::from(value);
    }
    if let Ok(value) = row.get_ushort(index) {
        return i32::from(value);
    }
    if let Ok(value) = row.get_uint(index) {
        return i32::try_from(value)
            .unwrap_or_else(|_| panic!("column '{column_name}' value {value} out of i32 range"));
    }
    if let Ok(value) = row.get_long(index) {
        return i32::try_from(value)
            .unwrap_or_else(|_| panic!("column '{column_name}' value {value} out of i32 range"));
    }
    panic!("column '{column_name}' does not decode as integer");
}

fn assert_relative_close(
    observed: f64,
    expected: f64,
    relative_tolerance: f64,
    absolute_tolerance: f64,
    label: &str,
) {
    let delta = (observed - expected).abs();
    let scale = expected.abs().max(1.0);
    let tolerance = absolute_tolerance.max(relative_tolerance * scale);
    assert!(
        delta <= tolerance,
        "{label} mismatch: expected {expected}, observed {observed}, delta {delta} exceeds tolerance {tolerance}"
    );
}

fn run_watershed_cli(
    run_dir: &Path,
    output_dir: &Path,
    policy: Option<&str>,
    legacy_sidecar_discovery: bool,
) -> std::process::Output {
    run_watershed_cli_with_options(
        run_dir,
        output_dir,
        policy,
        legacy_sidecar_discovery,
        None,
        None,
    )
}

fn run_watershed_cli_with_options(
    run_dir: &Path,
    output_dir: &Path,
    policy: Option<&str>,
    legacy_sidecar_discovery: bool,
    jobs: Option<&str>,
    hillslope_binary: Option<&Path>,
) -> std::process::Output {
    run_watershed_cli_with_current_dir(
        run_dir,
        output_dir,
        policy,
        legacy_sidecar_discovery,
        jobs,
        hillslope_binary,
        Path::new(env!("CARGO_MANIFEST_DIR")),
    )
}

fn run_watershed_cli_with_current_dir(
    run_dir: &Path,
    output_dir: &Path,
    policy: Option<&str>,
    legacy_sidecar_discovery: bool,
    jobs: Option<&str>,
    hillslope_binary: Option<&Path>,
    current_dir: &Path,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_openwepp-cli-watershed"));
    command.current_dir(current_dir);
    command
        .arg("--run-dir")
        .arg(run_dir)
        .arg("--run-file")
        .arg("case.run")
        .arg("--output-dir")
        .arg(output_dir);
    if let Some(policy_name) = policy {
        command.arg("--policy").arg(policy_name);
    }
    if legacy_sidecar_discovery {
        command.arg("--legacy-sidecar-discovery");
    }
    if let Some(jobs) = jobs {
        command.arg("--jobs").arg(jobs);
    }
    if let Some(hillslope_binary) = hillslope_binary {
        command.arg("--hillslope-binary").arg(hillslope_binary);
    }
    command
        .output()
        .expect("watershed CLI process should execute")
}

fn assert_all_watershed_outputs_exist(output_dir: &Path) {
    for output_name in watershed_output_relative_paths() {
        let output_path = output_dir.join("interchange").join(output_name);
        assert!(
            output_path.is_file(),
            "missing expected watershed parquet output {}",
            output_path.display()
        );
    }
}

fn assert_watershed_outputs_row_equivalent(serial_output_dir: &Path, parallel_output_dir: &Path) {
    for output_name in watershed_output_relative_paths() {
        let serial_path = serial_output_dir.join("interchange").join(output_name);
        let parallel_path = parallel_output_dir.join("interchange").join(output_name);
        let serial_rows = read_all_parquet_rows(&serial_path);
        let parallel_rows = read_all_parquet_rows(&parallel_path);
        assert_eq!(
            serial_rows, parallel_rows,
            "watershed output {output_name} should have identical decoded row order and values between --jobs 1 and --jobs N"
        );
    }
}

fn read_all_parquet_rows(path: &Path) -> Vec<String> {
    let file = File::open(path).unwrap_or_else(|error| {
        panic!(
            "parquet output should be readable ({}): {error}",
            path.display()
        )
    });
    let reader = SerializedFileReader::new(file).unwrap_or_else(|error| {
        panic!("parquet output should parse ({}): {error}", path.display())
    });
    reader
        .get_row_iter(None)
        .unwrap_or_else(|error| {
            panic!(
                "parquet row iterator should open ({}): {error}",
                path.display()
            )
        })
        .map(|row| {
            format!(
                "{:?}",
                row.unwrap_or_else(|error| {
                    panic!("parquet row should decode ({}): {error}", path.display())
                })
            )
        })
        .collect()
}

fn watershed_output_relative_paths() -> &'static [&'static str] {
    &[
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
    ]
}

fn run_generated_multi_hillslope_case(prefix: &str, jobs: &str) -> PathBuf {
    let run_dir = build_watershed_fixture_dir(prefix);
    prepare_multi_hillslope_output_guard_fixture(&run_dir);
    for hillslope_id in [1, 2, 3] {
        write_hillslope_source_runfile_fixture(&run_dir, hillslope_id);
    }
    write_generated_watershed_runfile(&run_dir, &[1, 2, 3]);

    let output_dir = run_dir.join("out");
    let hill_binary = Path::new(env!("CARGO_BIN_EXE_openwepp-cli-hill"));
    let output = run_watershed_cli_with_options(
        &run_dir,
        &output_dir,
        Some("compat"),
        false,
        Some(jobs),
        Some(hill_binary),
    );
    assert!(
        output.status.success(),
        "generated multi-hillslope watershed run should complete for --jobs {jobs}; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_all_watershed_outputs_exist(&output_dir);
    output_dir
}

#[test]
fn watershed_cli_legacy_discovery_matches_hillslope_unknown_sidecar_behavior() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_legacy_sidecar_discovery");
    write_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1]);
    prepare_output_guard_fixture(&run_dir);

    let runfile_path = run_dir.join("case.run");
    let runfile_payload = fs::read_to_string(&runfile_path)
        .expect("legacy sidecar runfile payload should be readable");
    let mutated_payload = runfile_payload.replace(
        "chaninp = \"chan.inp\"\n",
        "chaninp = \"configured_chan_should_be_ignored.inp\"\ntcr = \"configured_tcr_should_be_ignored.txt\"\n",
    );
    fs::write(&runfile_path, mutated_payload)
        .expect("legacy sidecar runfile payload should be writable");
    fs::write(run_dir.join("random_notes.txt"), "unknown sidecar payload")
        .expect("unknown sidecar fixture should be writable");

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), true);
    assert!(
        output.status.success(),
        "watershed CLI should succeed in legacy discovery mode and emit outputs; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("legacy-sidecar-discovery is active; ignoring configured inputs.chaninp"),
        "expected legacy override warning for configured chaninp in stderr, observed: {stderr}"
    );
    assert!(
        stderr.contains("legacy-sidecar-discovery is active; ignoring configured inputs.tcr"),
        "expected legacy override warning for configured tcr in stderr, observed: {stderr}"
    );
    assert!(
        stderr.contains("LSB-W-002 ignored unknown sidecar random_notes.txt"),
        "expected unknown sidecar warning parity with hillslope in stderr, observed: {stderr}"
    );
    assert!(
        !stderr.contains("CLIWAT-E-029"),
        "legacy discovery should ignore configured sidecar path validation, observed: {stderr}"
    );
    assert_all_watershed_outputs_exist(&output_dir);
}

#[test]
fn watershed_cli_mofe05_rejects_multiofe_contributor_without_manifest_metadata() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_mofe05_missing_manifest");
    write_hbp_fixture_with_nofe(
        run_dir.join("H1.hbp"),
        1,
        3,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile(&run_dir, &[1]);
    prepare_output_guard_fixture(&run_dir);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        !output.status.success(),
        "watershed CLI should hard-fail when multi-OFE contributor metadata is missing"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-036"),
        "expected MOFE05 missing-metadata intake guard code in stderr, observed: {stderr}"
    );
}

#[test]
fn watershed_cli_mofe05_rejects_multiofe_contributor_manifest_shape_violation() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_mofe05_manifest_shape_violation");
    write_hbp_fixture_with_nofe(
        run_dir.join("H1.hbp"),
        1,
        3,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile_with_manifest(&run_dir, &[1], true);
    prepare_output_guard_fixture(&run_dir);
    write_hillslope_manifest_fixture_missing_count(run_dir.join("H1.manifest.json"));

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        !output.status.success(),
        "watershed CLI should hard-fail malformed MOFE05 contributor metadata"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-037"),
        "expected MOFE05 metadata-shape intake guard code in stderr, observed: {stderr}"
    );
}

#[test]
fn watershed_cli_mofe05_rejects_multiofe_manifest_count_mismatch() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_mofe05_manifest_count_mismatch");
    write_hbp_fixture_with_nofe(
        run_dir.join("H1.hbp"),
        1,
        3,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile_with_manifest(&run_dir, &[1], true);
    prepare_output_guard_fixture(&run_dir);
    write_hillslope_manifest_fixture(run_dir.join("H1.manifest.json"), 2, 3_600.0);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        !output.status.success(),
        "watershed CLI should hard-fail contributor metadata count mismatch"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-037"),
        "expected MOFE05 metadata consistency guard code in stderr, observed: {stderr}"
    );
    assert!(
        stderr.contains("contributor_ofe_count"),
        "expected contributor count mismatch detail in stderr, observed: {stderr}"
    );
}

#[test]
fn watershed_cli_mofe05_accepts_valid_multiofe_metadata_and_emits_outputs() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_mofe05_valid_manifest");
    write_hbp_fixture_with_nofe(
        run_dir.join("H1.hbp"),
        1,
        3,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile_with_manifest(&run_dir, &[1], true);
    write_hillslope_manifest_fixture(run_dir.join("H1.manifest.json"), 3, 3_600.0);
    prepare_output_guard_fixture(&run_dir);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        output.status.success(),
        "watershed CLI should proceed past metadata intake and emit watershed outputs; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("CLIWAT-E-036") && !stderr.contains("CLIWAT-E-037"),
        "valid contributor metadata should not trigger MOFE05 intake guard codes, observed: {stderr}"
    );
    assert_all_watershed_outputs_exist(&output_dir);

    let totalwatsed3_row =
        read_first_parquet_row(&output_dir.join("interchange/totalwatsed3.parquet"));
    let area = row_f64_value(&totalwatsed3_row, "Area");
    let runoff = row_f64_value(&totalwatsed3_row, "Runoff");
    let q = row_f64_value(&totalwatsed3_row, "Q");
    let runvol = row_f64_value(&totalwatsed3_row, "runvol");
    assert_relative_close(area, 3_600.0, 0.0, 1.0e-12, "manifest publication area");
    assert_relative_close(q, runvol, 0.0, 1.0e-12, "manifest-area totalwatsed3 Q");
    assert_relative_close(
        runoff,
        runvol / 3_600.0 * 1_000.0,
        1.0e-12,
        1.0e-18,
        "manifest-area totalwatsed3 Runoff",
    );
}

#[test]
fn watershed_cli_mf_accepts_valid_per_ofe_publication_metadata() {
    let _execution_guard = watershed_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let run_dir = build_watershed_fixture_dir("ws_cli_mf_valid_per_ofe_manifest");
    write_hbp_fixture_with_nofe(
        run_dir.join("H1.hbp"),
        1,
        3,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
    );
    write_watershed_runfile_with_manifest(&run_dir, &[1], true);
    write_hillslope_manifest_per_ofe_fixture(run_dir.join("H1.manifest.json"), 3, 3_600.0, 6);
    prepare_output_guard_fixture(&run_dir);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"), false);
    assert!(
        output.status.success(),
        "watershed CLI should accept M-F per-OFE publication metadata; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("CLIWAT-E-037"),
        "valid M-F per-OFE publication metadata should not trigger manifest guard codes, observed: {stderr}"
    );
    assert_all_watershed_outputs_exist(&output_dir);
}

fn build_watershed_fixture_dir(prefix: &str) -> PathBuf {
    let destination = unique_temp_dir(prefix);
    fs::create_dir_all(&destination).expect("fixture directory should be creatable");

    copy_fixture_file(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/infile/watershed_structure/strict_valid_two_rows.str"),
        &destination.join("pw0.str"),
    );
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
    fs::write(destination.join("chan.inp"), "3 600\n0.000001\n1\n3\n")
        .expect("chan.inp fixture should be writable");

    destination
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

#[cfg(unix)]
fn write_successful_noop_hillslope_binary(run_dir: &Path) -> PathBuf {
    let path = run_dir.join("noop-openwepp-cli-hill.sh");
    fs::write(&path, "#!/bin/sh\nexit 0\n").expect("fake hillslope binary should be writable");
    let mut permissions = fs::metadata(&path)
        .expect("fake hillslope binary metadata should be readable")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).expect("fake hillslope binary should be executable");
    path
}

#[cfg(unix)]
fn write_w3_failure_hillslope_binary(run_dir: &Path) -> PathBuf {
    let path = run_dir.join("w3-failing-openwepp-cli-hill.sh");
    let launch_marker = run_dir.join("unexpected-h3-launch.txt");
    let payload = format!(
        r#"#!/bin/sh
runfile=""
while [ "$#" -gt 0 ]; do
  if [ "$1" = "--run-file" ]; then
    shift
    runfile="$1"
  fi
  shift
done
case "$runfile" in
  *H1.run.toml)
    sleep 1
    exit 0
    ;;
  *H2.run.toml)
    exit 23
    ;;
  *H3.run.toml)
    echo "$runfile" >> "{launch_marker}"
    exit 0
    ;;
  *)
    exit 0
    ;;
esac
"#,
        launch_marker = launch_marker.display()
    );
    fs::write(&path, payload).expect("failing fake hillslope binary should be writable");
    let mut permissions = fs::metadata(&path)
        .expect("failing fake hillslope binary metadata should be readable")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions)
        .expect("failing fake hillslope binary should be executable");
    path
}

fn prepare_output_guard_fixture(run_dir: &Path) {
    fs::write(run_dir.join("pw0.str"), "94.301\n2 1 0 0 0 0 0 0 0 0\n")
        .expect("channel-only structure fixture should be writable");
    fs::write(run_dir.join("chan.inp"), "3 600\n0.000001\n1\n2\n")
        .expect("channel-only chan.inp fixture should be writable");
}

fn prepare_multi_hillslope_output_guard_fixture(run_dir: &Path) {
    fs::write(run_dir.join("pw0.str"), "94.301\n2 1 2 3 0 0 0 0 0 0\n")
        .expect("multi-hillslope channel-only structure fixture should be writable");
    fs::write(run_dir.join("chan.inp"), "3 600\n0.000001\n1\n2\n")
        .expect("channel-only chan.inp fixture should be writable");
}

fn write_watershed_runfile(run_dir: &Path, hillslope_ids: &[u32]) {
    write_watershed_runfile_with_manifest(run_dir, hillslope_ids, false);
}

fn write_generated_watershed_runfile(run_dir: &Path, hillslope_ids: &[u32]) {
    let mut runfile_payload = String::from(
        r#"
schema = "openwepp-watershed-runfile-v1"
run_name = "wshedw2-serial-supervisor-contract"
unit_system = "metric"

[inputs]
pw0_str = "pw0.str"
pw0_chn = "pw0.chn"
pw0_imp = "pw0.imp"
pw0_man = "pw0.man"
pw0_slp = "pw0.slp"
pw0_cli = "pw0.cli"
pw0_sol = "pw0.sol"
chaninp = "chan.inp"

[inputs.applicability]
chapter13_small_watershed_intent = true
allow_partial_area_response = false
allow_headcutting = false
allow_bank_sloughing = false
allow_perennial_streams = false
"#,
    );

    for hillslope_id in hillslope_ids {
        write!(
            &mut runfile_payload,
            r#"
[[inputs.hillslopes_block]]
hillslope_id = {hillslope_id}
run_file = "H{hillslope_id}.source.run"
use_existing_pass_file = false
"#
        )
        .expect("generated watershed runfile block should format");
    }

    runfile_payload.push_str(
        r#"
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
"#,
    );
    fs::write(run_dir.join("case.run"), runfile_payload).expect("runfile should be writable");
}

fn write_watershed_runfile_with_manifest(
    run_dir: &Path,
    hillslope_ids: &[u32],
    include_manifest_file: bool,
) {
    let mut runfile_payload = String::from(
        r#"
schema = "openwepp-watershed-runfile-v1"
run_name = "ws-cli-behavior-contract"
unit_system = "metric"

[inputs]
pw0_str = "pw0.str"
pw0_chn = "pw0.chn"
pw0_imp = "pw0.imp"
pw0_man = "pw0.man"
pw0_slp = "pw0.slp"
pw0_cli = "pw0.cli"
pw0_sol = "pw0.sol"
chaninp = "chan.inp"

[inputs.applicability]
chapter13_small_watershed_intent = true
allow_partial_area_response = false
allow_headcutting = false
allow_bank_sloughing = false
allow_perennial_streams = false
"#,
    );

    for hillslope_id in hillslope_ids {
        let mut block = format!(
            r#"
[[inputs.hillslopes_block]]
hillslope_id = {hillslope_id}
pass_file = "H{hillslope_id}.hbp"
use_existing_pass_file = true
"#
        );
        if include_manifest_file {
            block.push_str("manifest_file = \"H");
            block.push_str(hillslope_id.to_string().as_str());
            block.push_str(".manifest.json\"\n");
        }
        runfile_payload.push_str(&block);
    }

    runfile_payload.push_str(
        r#"
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
"#,
    );
    fs::write(run_dir.join("case.run"), runfile_payload).expect("runfile should be writable");
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
run_name = "wshedw2-hillslope-{hillslope_id}"
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

fn write_hillslope_manifest_fixture(path: PathBuf, contributor_ofe_count: usize, area_m2: f64) {
    let carry_active = contributor_ofe_count > 1;
    let payload = format!(
        r#"{{
  "schema": "openwepp-hillslope-run-manifest-v1",
  "wb13_publication": {{
    "publication_ofe_policy": "single-row-canonicalized-hillslope-aggregate",
    "contributor_ofe_count": {contributor_ofe_count},
    "area_policy": "sum-ofe-geometry-area",
    "publication_area_m2": {area_m2}
  }},
  "mofe_hourly_carry": {{
    "policy": "baseline-wathour-24-slot-copy-forward",
    "active": {carry_active},
    "substep_count": 24,
    "required_arrays": ["ui_SUrunf", "ui_SCrunf", "ui_LfUrf", "ui_LfCrf"],
    "upstream_carry_total_m": 0.0,
    "current_carry_total_m": 0.0
  }}
}}
"#
    );
    fs::write(path, payload).expect("hillslope manifest fixture should be writable");
}

fn write_hillslope_manifest_per_ofe_fixture(
    path: PathBuf,
    contributor_ofe_count: usize,
    area_m2: f64,
    row_count: usize,
) {
    let carry_active = contributor_ofe_count > 1;
    let day_count = row_count / contributor_ofe_count;
    let payload = format!(
        r#"{{
  "schema": "openwepp-hillslope-run-manifest-v1",
  "wb13_publication": {{
    "publication_ofe_policy": "per-ofe-dynamic-water-balance-state",
    "contributor_ofe_count": {contributor_ofe_count},
    "area_policy": "sum-ofe-geometry-area",
    "publication_area_m2": {area_m2},
    "storage_lineage_policy": "per-ofe-dynamic-wb-state",
    "per_ofe_state_policy": "published-per-ofe-wb13-records",
    "transfer_identity_status": "pass-published-per-ofe-wb13-records",
    "per_element_identity_status": "pass-published-per-ofe-wb13-records",
    "aggregate_identity_status": "pass-published-per-ofe-wb13-records",
    "row_count": {row_count},
    "per_ofe_record_count": {row_count},
    "per_ofe_expected_record_count": {row_count},
    "per_ofe_internal_day_count": {day_count},
    "hillslope_total_identity_max_abs_mm": 1.0e-13,
    "sim_day_index_monotonic": true,
    "first_row_key": {{
      "ofe": 1
    }},
    "last_row_key": {{
      "ofe": {contributor_ofe_count}
    }}
  }},
  "mofe_hourly_carry": {{
    "policy": "baseline-wathour-24-slot-copy-forward",
    "active": {carry_active},
    "substep_count": 24,
    "required_arrays": ["ui_SUrunf", "ui_SCrunf", "ui_LfUrf", "ui_LfCrf"],
    "upstream_carry_total_m": 0.0,
    "current_carry_total_m": 0.0
  }}
}}
"#
    );
    fs::write(path, payload).expect("hillslope per-OFE manifest fixture should be writable");
}

fn write_hillslope_manifest_fixture_missing_count(path: PathBuf) {
    let payload = r#"{
  "schema": "openwepp-hillslope-run-manifest-v1",
  "wb13_publication": {
    "publication_ofe_policy": "single-row-canonicalized-hillslope-aggregate",
    "area_policy": "sum-ofe-geometry-area",
    "publication_area_m2": 3600.0
  }
}
"#;
    fs::write(path, payload).expect("hillslope manifest fixture should be writable");
}

fn put_u8(buf: &mut Vec<u8>, value: u8) {
    buf.push(value);
}

fn put_u16(buf: &mut Vec<u8>, value: u16) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_u32(buf: &mut Vec<u8>, value: u32) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_i32(buf: &mut Vec<u8>, value: i32) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_u64(buf: &mut Vec<u8>, value: u64) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_i64(buf: &mut Vec<u8>, value: i64) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_f64(buf: &mut Vec<u8>, value: f64) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_string(buf: &mut Vec<u8>, value: &str) {
    put_u32(buf, value.len() as u32);
    buf.extend_from_slice(value.as_bytes());
}

fn put_u32_at(buf: &mut [u8], offset: usize, value: u32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn crc32c(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for value in data {
        crc ^= *value as u32;
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ 0x82F63B78;
            } else {
                crc >>= 1;
            }
            crc &= 0xFFFF_FFFF;
        }
    }
    crc ^ 0xFFFF_FFFF
}

fn expected_state_schema(state_id: u16) -> Option<(u8, u8, u16, u8, u8)> {
    match state_id {
        1 => Some((1, 1, 1, 1, DIM_NOFE)),
        2 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        3 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        4 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        5 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        6 => Some((1, 2, 3, 2, DIM_NOFE_LAYERS)),
        7 => Some((1, 2, 3, 2, DIM_NOFE_LAYERS)),
        100 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        101 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        102 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        103 => Some((1, 1, 2, 1, DIM_NOFE)),
        104 => Some((1, 1, 2, 1, DIM_NOFE)),
        200 => Some((1, 1, 2, 1, DIM_NOFE)),
        201 => Some((1, 2, 4, 1, DIM_NOFE)),
        202 => Some((1, 1, 2, 1, DIM_NOFE)),
        203 => Some((1, 1, 2, 1, DIM_NOFE)),
        204 => Some((1, 1, 2, 1, DIM_NOFE)),
        205 => Some((1, 1, 2, 1, DIM_NOFE)),
        206 => Some((1, 1, 2, 1, DIM_NOFE)),
        207 => Some((1, 1, 2, 1, DIM_NOFE)),
        208 => Some((1, 1, 2, 1, DIM_NOFE)),
        209 => Some((1, 1, 2, 1, DIM_NOFE)),
        210 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        300 => Some((1, 1, 5, 0, DIM_SCALAR)),
        900 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        901 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        _ => None,
    }
}

fn state_dims(dims_kind: u8, nofe: u32, max_layers: u32) -> Vec<u32> {
    match dims_kind {
        DIM_SCALAR => vec![],
        DIM_NOFE => vec![nofe],
        DIM_NOFE_LAYERS => vec![nofe, max_layers],
        _ => panic!("unknown dims_kind {dims_kind}"),
    }
}

fn build_state_entry(state_id: u16, nofe: u32, max_layers: u32) -> Vec<u8> {
    let (required_flag, representation_class, unit_class, rank, dims_kind) =
        expected_state_schema(state_id).expect("required state schema should exist");

    let dims = state_dims(dims_kind, nofe, max_layers);
    assert_eq!(dims.len(), rank as usize);

    let mut entry = Vec::new();
    put_u8(&mut entry, required_flag);
    put_u8(&mut entry, representation_class);
    put_u16(&mut entry, unit_class);
    put_u8(&mut entry, rank);
    for dim in &dims {
        put_u32(&mut entry, *dim);
    }

    let value_count = dims.iter().copied().product::<u32>().max(1) as usize;
    match representation_class {
        1 => {
            for _ in 0..value_count {
                put_i64(&mut entry, 0);
            }
        }
        2 => {
            for _ in 0..value_count {
                put_f64(&mut entry, 0.0);
            }
        }
        _ => panic!("unsupported representation class"),
    }

    let mut out = Vec::new();
    put_u16(&mut out, state_id);
    put_u32(&mut out, entry.len() as u32);
    out.extend_from_slice(&entry);
    out
}

fn append_common_prefix(
    schema_major: u16,
    schema_minor: u16,
    hillslope_id: u32,
    nofe: u16,
    nyear: u32,
    begin_year: i32,
    simulation_mode: u8,
) -> Vec<u8> {
    let mut file = Vec::new();

    let mut header = Vec::new();
    header.extend_from_slice(MAGIC);
    put_u16(&mut header, schema_major);
    put_u16(&mut header, schema_minor);
    put_u8(&mut header, 1);
    let header_bytes_pos = header.len();
    put_u32(&mut header, 0);
    header.extend_from_slice(&[0u8; 32]);
    put_u8(&mut header, 1);
    put_string(&mut header, "openwepp-watershed-cli-test");
    put_string(&mut header, "ws-cli-test");
    put_string(&mut header, "2026-05-25T00:00:00Z");
    put_string(&mut header, "metric-v1");
    header.extend_from_slice(&[0u8; 32]);
    let header_crc_pos = header.len();
    put_u32(&mut header, 0);
    let header_bytes = header.len() as u32;
    put_u32_at(&mut header, header_bytes_pos, header_bytes);
    let header_crc = crc32c(&header);
    put_u32_at(&mut header, header_crc_pos, header_crc);
    file.extend_from_slice(&header);

    let npart = 1u16;
    let max_layers = 1u16;

    put_u32(&mut file, hillslope_id);
    put_u32(&mut file, nyear);
    put_i32(&mut file, begin_year);
    put_u16(&mut file, npart);
    put_u16(&mut file, nofe);
    put_u16(&mut file, max_layers);
    put_string(&mut file, "gregorian");
    put_u16(&mut file, 1);
    put_u8(&mut file, simulation_mode);

    put_string(&mut file, "p1.cli");
    put_i64(&mut file, 0);
    put_u32(&mut file, npart as u32);
    put_f64(&mut file, 0.001);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);

    put_u32(&mut file, nyear);
    put_u32(&mut file, 1);
    put_i32(&mut file, begin_year);
    put_u16(&mut file, 1);
    put_u16(&mut file, 1);
    put_u16(&mut file, 1);
    put_u8(&mut file, 0);

    put_u32(&mut file, REQUIRED_STATE_IDS.len() as u32);
    for state_id in REQUIRED_STATE_IDS {
        let (required_flag, representation_class, unit_class, rank, dims_kind) =
            expected_state_schema(*state_id).expect("required state schema should exist");
        put_u16(&mut file, *state_id);
        put_u8(&mut file, required_flag);
        put_u8(&mut file, representation_class);
        put_u16(&mut file, unit_class);
        put_u8(&mut file, rank);
        put_u8(&mut file, dims_kind);
        put_string(&mut file, &format!("state_{state_id}"));
    }

    file
}

fn scaled_i64(value: f64) -> i64 {
    let scaled = value * SCALE_INV_I64;
    assert!(scaled.is_finite());
    assert!(scaled >= i64::MIN as f64 && scaled <= i64::MAX as f64);
    scaled.round() as i64
}

fn build_event_payload(
    nofe: u16,
    sim_year_index: u32,
    calendar_year: i32,
    julian_day: u16,
    concentration: f64,
    fraction: f64,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
) -> Vec<u8> {
    build_event_payload_with_groundwater(
        nofe,
        sim_year_index,
        calendar_year,
        julian_day,
        concentration,
        fraction,
        peak_runoff_m3_s,
        duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
        0.0,
        0.0,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_event_payload_with_groundwater(
    nofe: u16,
    sim_year_index: u32,
    calendar_year: i32,
    julian_day: u16,
    concentration: f64,
    fraction: f64,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    baseflow_volume_m3: f64,
    deep_seepage_volume_m3: f64,
) -> Vec<u8> {
    let nofe = u32::from(nofe);
    let max_layers = 1u32;

    let mut payload = Vec::new();
    put_u32(&mut payload, sim_year_index);
    put_i32(&mut payload, calendar_year);
    put_u16(&mut payload, julian_day);
    put_u8(&mut payload, 2);
    put_u16(&mut payload, 0);
    put_u16(&mut payload, REQUIRED_STATE_IDS.len() as u16);
    put_f64(&mut payload, duration_seconds);
    put_f64(&mut payload, 0.5);
    put_f64(&mut payload, 0.8);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_f64(&mut payload, peak_runoff_m3_s);
    put_i64(&mut payload, scaled_i64(total_detachment_kg));
    put_i64(&mut payload, scaled_i64(total_deposition_kg));
    put_u32(&mut payload, 1);
    put_f64(&mut payload, concentration);
    put_u32(&mut payload, 1);
    put_f64(&mut payload, fraction);
    put_i64(&mut payload, scaled_i64(baseflow_volume_m3));
    put_i64(&mut payload, scaled_i64(deep_seepage_volume_m3));

    for state_id in REQUIRED_STATE_IDS {
        payload.extend_from_slice(&build_state_entry(*state_id, nofe, max_layers));
    }

    payload
}

fn build_no_event_payload(
    nofe: u16,
    sim_year_index: u32,
    calendar_year: i32,
    julian_day: u16,
) -> Vec<u8> {
    let nofe = u32::from(nofe);
    let max_layers = 1u32;

    let mut payload = Vec::new();
    put_u32(&mut payload, sim_year_index);
    put_i32(&mut payload, calendar_year);
    put_u16(&mut payload, julian_day);
    put_u8(&mut payload, 0);
    put_u16(&mut payload, 0);
    put_u16(&mut payload, REQUIRED_STATE_IDS.len() as u16);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);

    for state_id in REQUIRED_STATE_IDS {
        payload.extend_from_slice(&build_state_entry(*state_id, nofe, max_layers));
    }

    payload
}

fn build_schema1_event_fixture(
    hillslope_id: u32,
    nofe: u16,
    concentration: f64,
    fraction: f64,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
) -> Vec<u8> {
    let mut file = append_common_prefix(SUPPORTED_MAJOR_V1, 0, hillslope_id, nofe, 1, 2004, 1);
    let payload = build_event_payload(
        nofe,
        1,
        2004,
        1,
        concentration,
        fraction,
        peak_runoff_m3_s,
        duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
    );
    let payload_crc = crc32c(&payload);

    let directory_start = file.len();
    let directory_len = 4 + 27;
    let payload_offset = directory_start + directory_len;
    let mut directory = Vec::new();
    put_u32(&mut directory, 1);
    put_u32(&mut directory, 1);
    put_i32(&mut directory, 2004);
    put_u16(&mut directory, 1);
    put_u8(&mut directory, 2);
    put_u64(&mut directory, payload_offset as u64);
    put_u32(&mut directory, payload.len() as u32);
    put_u32(&mut directory, payload_crc);

    file.extend_from_slice(&directory);
    file.extend_from_slice(&payload);

    let directory_crc = crc32c(&directory);
    put_u32(&mut file, directory_crc);
    let file_crc_pos = file.len();
    put_u32(&mut file, 0);
    put_u32(&mut file, 1);
    file.extend_from_slice(FOOTER_MAGIC);
    let file_crc = crc32c(&file);
    put_u32_at(&mut file, file_crc_pos, file_crc);
    file
}

fn build_schema1_event_fixture_with_groundwater(
    hillslope_id: u32,
    nofe: u16,
    baseflow_volume_m3: f64,
    deep_seepage_volume_m3: f64,
) -> Vec<u8> {
    let mut file = append_common_prefix(SUPPORTED_MAJOR_V1, 0, hillslope_id, nofe, 1, 2004, 1);
    let payload = build_event_payload_with_groundwater(
        nofe,
        1,
        2004,
        1,
        0.25,
        1.0,
        5.0,
        4.0,
        1_800.0,
        1_200.0,
        baseflow_volume_m3,
        deep_seepage_volume_m3,
    );
    let payload_crc = crc32c(&payload);

    let directory_start = file.len();
    let directory_len = 4 + 27;
    let payload_offset = directory_start + directory_len;
    let mut directory = Vec::new();
    put_u32(&mut directory, 1);
    put_u32(&mut directory, 1);
    put_i32(&mut directory, 2004);
    put_u16(&mut directory, 1);
    put_u8(&mut directory, 2);
    put_u64(&mut directory, payload_offset as u64);
    put_u32(&mut directory, payload.len() as u32);
    put_u32(&mut directory, payload_crc);

    file.extend_from_slice(&directory);
    file.extend_from_slice(&payload);

    let directory_crc = crc32c(&directory);
    put_u32(&mut file, directory_crc);
    let file_crc_pos = file.len();
    put_u32(&mut file, 0);
    put_u32(&mut file, 1);
    file.extend_from_slice(FOOTER_MAGIC);
    let file_crc = crc32c(&file);
    put_u32_at(&mut file, file_crc_pos, file_crc);
    file
}

fn build_schema1_no_event_fixture(hillslope_id: u32, nofe: u16) -> Vec<u8> {
    let mut file = append_common_prefix(SUPPORTED_MAJOR_V1, 0, hillslope_id, nofe, 1, 2004, 1);
    let payload = build_no_event_payload(nofe, 1, 2004, 1);
    let payload_crc = crc32c(&payload);

    let directory_start = file.len();
    let directory_len = 4 + 27;
    let payload_offset = directory_start + directory_len;
    let mut directory = Vec::new();
    put_u32(&mut directory, 1);
    put_u32(&mut directory, 1);
    put_i32(&mut directory, 2004);
    put_u16(&mut directory, 1);
    put_u8(&mut directory, 0);
    put_u64(&mut directory, payload_offset as u64);
    put_u32(&mut directory, payload.len() as u32);
    put_u32(&mut directory, payload_crc);

    file.extend_from_slice(&directory);
    file.extend_from_slice(&payload);

    let directory_crc = crc32c(&directory);
    put_u32(&mut file, directory_crc);
    let file_crc_pos = file.len();
    put_u32(&mut file, 0);
    put_u32(&mut file, 1);
    file.extend_from_slice(FOOTER_MAGIC);
    let file_crc = crc32c(&file);
    put_u32_at(&mut file, file_crc_pos, file_crc);
    file
}

fn write_hbp_fixture(
    path: PathBuf,
    hillslope_id: u32,
    concentration: f64,
    fraction: f64,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
) {
    write_hbp_fixture_with_nofe(
        path,
        hillslope_id,
        1,
        concentration,
        fraction,
        peak_runoff_m3_s,
        duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
    );
}

fn write_hbp_fixture_with_nofe(
    path: PathBuf,
    hillslope_id: u32,
    nofe: u16,
    concentration: f64,
    fraction: f64,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
) {
    let bytes = build_schema1_event_fixture(
        hillslope_id,
        nofe,
        concentration,
        fraction,
        peak_runoff_m3_s,
        duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
    );
    fs::write(path, bytes).expect("HBP fixture should be writable");
}

fn write_hbp_no_event_fixture(path: PathBuf, hillslope_id: u32) {
    let bytes = build_schema1_no_event_fixture(hillslope_id, 1);
    fs::write(path, bytes).expect("HBP no-event fixture should be writable");
}
