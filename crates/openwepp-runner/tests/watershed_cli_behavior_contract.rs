#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::match_same_arms,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

const MAGIC: &[u8; 8] = b"WFPHBP01";
const FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
const SUPPORTED_MAJOR_V1: u16 = 1;
const DIM_SCALAR: u8 = 0;
const DIM_NOFE: u8 = 1;
const DIM_NOFE_LAYERS: u8 = 2;
const SCALE_INV_I64: f64 = 1.0e9;

const REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 300, 900, 901,
];

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
    let output = run_watershed_cli(&run_dir, &output_dir, None);
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
fn watershed_cli_rejects_placeholder_watershed_output_emission() {
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
    let output = run_watershed_cli(&run_dir, &output_dir, Some("compat"));
    assert!(
        !output.status.success(),
        "watershed CLI should fail with explicit output guard until data-backed writers exist"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLIWAT-E-034"),
        "expected writer failure wrapper code in stderr, observed: {stderr}"
    );
    assert!(
        stderr.contains("OWSOUT-E-004"),
        "expected typed writer guard code in stderr, observed: {stderr}"
    );
}

fn watershed_execution_lock() -> &'static Mutex<()> {
    static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    RUN_LOCK.get_or_init(|| Mutex::new(()))
}

fn run_watershed_cli(
    run_dir: &Path,
    output_dir: &Path,
    policy: Option<&str>,
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
    command
        .output()
        .expect("watershed CLI process should execute")
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
"#,
    );

    for hillslope_id in hillslope_ids {
        runfile_payload.push_str(
            format!(
                r#"
[[inputs.hillslopes_block]]
hillslope_id = {hillslope_id}
pass_file = "H{hillslope_id}.hbp"
"#
            )
            .as_str(),
        );
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
    let nofe = 1u16;
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
    let nofe = 1u32;
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
    concentration: f64,
    fraction: f64,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
) -> Vec<u8> {
    let mut file = append_common_prefix(SUPPORTED_MAJOR_V1, 0, hillslope_id, 1, 2004, 1);
    let payload = build_event_payload(
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
    let bytes = build_schema1_event_fixture(
        hillslope_id,
        concentration,
        fraction,
        peak_runoff_m3_s,
        duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
    );
    fs::write(path, bytes).expect("HBP fixture should be writable");
}
