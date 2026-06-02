#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::match_same_arms,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

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
    let mut command = Command::new(env!("CARGO_BIN_EXE_openwepp-cli-watershed"));
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
    command
        .output()
        .expect("watershed CLI process should execute")
}

fn assert_all_watershed_outputs_exist(output_dir: &Path) {
    let expected_outputs = [
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
    ];

    for output_name in expected_outputs {
        let output_path = output_dir.join("interchange").join(output_name);
        assert!(
            output_path.is_file(),
            "missing expected watershed parquet output {}",
            output_path.display()
        );
    }
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
}

fn build_watershed_fixture_dir(prefix: &str) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix epoch should be before now")
        .as_nanos();
    let destination = std::env::temp_dir().join(format!("{prefix}_{timestamp}"));
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

fn copy_fixture_file(source: &Path, destination: &Path) {
    fs::copy(source, destination).unwrap_or_else(|error| {
        panic!(
            "fixture copy should succeed ({} -> {}): {error}",
            source.display(),
            destination.display()
        )
    });
}

fn prepare_output_guard_fixture(run_dir: &Path) {
    fs::write(run_dir.join("pw0.str"), "94.301\n2 1 0 0 0 0 0 0 0 0\n")
        .expect("channel-only structure fixture should be writable");
    fs::write(run_dir.join("chan.inp"), "3 600\n0.000001\n1\n2\n")
        .expect("channel-only chan.inp fixture should be writable");
}

fn write_watershed_runfile(run_dir: &Path, hillslope_ids: &[u32]) {
    write_watershed_runfile_with_manifest(run_dir, hillslope_ids, false);
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
