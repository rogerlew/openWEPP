#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::too_many_arguments,
    clippy::unreadable_literal
)]

use std::fmt::Write as _;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;

use openwepp_input_contract::parsers::hbp::{
    HbpParseOptions, parse_hbp_from_path_with_latest_event_payload,
};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};

const MAGIC: &[u8; 8] = b"WFPHBP01";
const FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
const SUPPORTED_MAJOR_V1: u16 = 1;
const SUPPORTED_MINOR_V1: u16 = 1;
const DIM_SCALAR: u8 = 0;
const DIM_NOFE: u8 = 1;
const DIM_NOFE_LAYERS: u8 = 2;
const SCALE_INV_I64: f64 = 1.0e9;

const REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 300, 900, 901,
];

#[test]
fn mt3_watershed_cli_hbp_hourly_pair_reaches_channel_consumer() {
    let mut spike_runoff = [0.0_f64; 24];
    spike_runoff[10] = 7_200.0;
    let mut spike_sediment = [0.0_f64; 24];
    spike_sediment[10] = 240.0;

    let mut spread_runoff = [0.0_f64; 24];
    let mut spread_sediment = [0.0_f64; 24];
    for hour in 8..12 {
        spread_runoff[hour] = 1_800.0;
        spread_sediment[hour] = 60.0;
    }

    let spike = run_hourly_fixture("mt3_cli_spike", spike_runoff, spike_sediment);
    let spread = run_hourly_fixture("mt3_cli_spread", spread_runoff, spread_sediment);

    assert_relative_close(
        spike.hbp_hourly_runoff_sum_m3,
        spread.hbp_hourly_runoff_sum_m3,
        1.0e-9,
        "HBP hourly runoff total",
    );
    assert_relative_close(
        spike.hbp_hourly_sediment_sum_kg,
        spread.hbp_hourly_sediment_sum_kg,
        1.0e-9,
        "HBP hourly sediment total",
    );
    assert!(spike.ebe_runoff_volume_m3 > 0.0);
    assert!(spread.ebe_runoff_volume_m3 > 0.0);
    assert!(
        (spike.ebe_peak_runoff_m3_s - spread.ebe_peak_runoff_m3_s).abs() > 1.0e-9,
        "identical scalar HBP fields must not erase hourly water timing"
    );
    assert!(
        (spike.ebe_sediment_yield_kg - spread.ebe_sediment_yield_kg).abs() > 1.0e-9,
        "identical scalar/daily HBP fields must not erase hourly sediment timing"
    );
}

struct HourlyFixtureOutput {
    hbp_hourly_runoff_sum_m3: f64,
    hbp_hourly_sediment_sum_kg: f64,
    ebe_peak_runoff_m3_s: f64,
    ebe_runoff_volume_m3: f64,
    ebe_sediment_yield_kg: f64,
}

fn run_hourly_fixture(
    prefix: &str,
    hourly_runoff_volume_m3: [f64; 24],
    hourly_sediment_mass_kg: [f64; 24],
) -> HourlyFixtureOutput {
    let run_dir = build_watershed_fixture_dir(prefix);
    write_hourly_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        2.0,
        3_600.0,
        240.0,
        0.0,
        hourly_runoff_volume_m3,
        hourly_sediment_mass_kg,
    );
    write_watershed_runfile(&run_dir, &[1]);

    let (hbp, latest_event_payload) = parse_hbp_from_path_with_latest_event_payload(
        run_dir.join("H1.hbp"),
        HbpParseOptions {
            expected_hillslope_id: Some(1),
        },
    )
    .expect("schema-1.1 HBP fixture should parse");
    let payload = latest_event_payload.expect("schema-1.1 fixture should contain EVENT payload");
    assert_eq!(hbp.schema_major, 1);
    assert_eq!(hbp.schema_minor, 1);
    assert_eq!(payload.hourly_runoff_volume_m3.len(), 24);
    assert_eq!(payload.hourly_sediment_mass_kg.len(), 24);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir);
    assert!(
        output.status.success(),
        "watershed CLI should consume schema-1.1 HBP hourly pair; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let ebe_row = read_first_parquet_row(&output_dir.join("interchange/ebe_pw0.parquet"));
    HourlyFixtureOutput {
        hbp_hourly_runoff_sum_m3: payload.hourly_runoff_volume_m3.iter().sum(),
        hbp_hourly_sediment_sum_kg: payload.hourly_sediment_mass_kg.iter().sum(),
        ebe_peak_runoff_m3_s: row_f64_value(&ebe_row, "peak_runoff"),
        ebe_runoff_volume_m3: row_f64_value(&ebe_row, "runoff_volume"),
        ebe_sediment_yield_kg: row_f64_value(&ebe_row, "sediment_yield"),
    }
}

fn build_watershed_fixture_dir(prefix: &str) -> PathBuf {
    let destination = unique_temp_dir(prefix);
    fs::create_dir_all(&destination).expect("fixture directory should be creatable");

    copy_fixture_file(
        &repo_root().join("tests/fixtures/infile/watershed_structure/strict_valid_two_rows.str"),
        &destination.join("pw0.str"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/infile/watershed_channel/strict_sidecar_required.chn"),
        &destination.join("pw0.chn"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/infile/watershed_impoundment/strict_valid_minimal.imp"),
        &destination.join("pw0.imp"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/cli01/hillslope_run_dir/case.man"),
        &destination.join("pw0.man"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/cli01/hillslope_run_dir/case.slp"),
        &destination.join("pw0.slp"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/cli01/hillslope_run_dir/case.cli"),
        &destination.join("pw0.cli"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/cli01/hillslope_run_dir/case.sol"),
        &destination.join("pw0.sol"),
    );
    fs::write(destination.join("pw0.str"), "94.301\n2 1 0 0 0 0 0 0 0 0\n")
        .expect("channel-only structure fixture should be writable");
    fs::write(destination.join("chan.inp"), "3 600\n0.000001\n1\n2\n")
        .expect("channel-only chan.inp fixture should be writable");

    destination
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
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

fn write_watershed_runfile(run_dir: &Path, hillslope_ids: &[u32]) {
    let mut runfile_payload = String::from(
        r#"
schema = "openwepp-watershed-runfile-v1"
run_name = "mt3-hbp-hourly-consumer-contract"
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
pass_file = "H{hillslope_id}.hbp"
use_existing_pass_file = true
"#
        )
        .expect("watershed runfile block should format");
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
loss_all_years_class_data = "interchange/loss_pw0.class_data.parquet"
"#,
    );
    fs::write(run_dir.join("case.run"), runfile_payload).expect("runfile should be writable");
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
        .output()
        .expect("watershed CLI should launch")
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
        .unwrap_or_else(|| panic!("expected at least one row in {}", path.display()))
        .unwrap_or_else(|error| panic!("first parquet row should decode: {error}"))
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

fn assert_relative_close(observed: f64, expected: f64, tolerance: f64, label: &str) {
    let delta = (observed - expected).abs();
    assert!(
        delta <= tolerance,
        "{label} mismatch: expected {expected}, observed {observed}, delta {delta}"
    );
}

fn write_hourly_hbp_fixture(
    path: PathBuf,
    hillslope_id: u32,
    scalar_peak_runoff_m3_s: f64,
    scalar_duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    hourly_runoff_volume_m3: [f64; 24],
    hourly_sediment_mass_kg: [f64; 24],
) {
    let bytes = build_schema1_1_event_fixture(
        hillslope_id,
        scalar_peak_runoff_m3_s,
        scalar_duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
        hourly_runoff_volume_m3,
        hourly_sediment_mass_kg,
    );
    fs::write(path, bytes).expect("HBP fixture should be writable");
}

fn build_schema1_1_event_fixture(
    hillslope_id: u32,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    hourly_runoff_volume_m3: [f64; 24],
    hourly_sediment_mass_kg: [f64; 24],
) -> Vec<u8> {
    let mut file = append_common_prefix(hillslope_id);
    let payload = build_event_payload(
        duration_seconds,
        peak_runoff_m3_s,
        total_detachment_kg,
        total_deposition_kg,
        hourly_runoff_volume_m3,
        hourly_sediment_mass_kg,
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

fn append_common_prefix(hillslope_id: u32) -> Vec<u8> {
    let mut file = Vec::new();
    let mut header = Vec::new();
    header.extend_from_slice(MAGIC);
    put_u16(&mut header, SUPPORTED_MAJOR_V1);
    put_u16(&mut header, SUPPORTED_MINOR_V1);
    put_u8(&mut header, 1);
    let header_bytes_pos = header.len();
    put_u32(&mut header, 0);
    header.extend_from_slice(&[0u8; 32]);
    put_u8(&mut header, 1);
    put_string(&mut header, "openwepp-mt3-test");
    put_string(&mut header, "mt3-hourly-consumer");
    put_string(&mut header, "2026-07-09T00:00:00Z");
    put_string(&mut header, "metric-v1");
    header.extend_from_slice(&[0u8; 32]);
    let header_crc_pos = header.len();
    put_u32(&mut header, 0);
    let header_bytes = header.len() as u32;
    put_u32_at(&mut header, header_bytes_pos, header_bytes);
    let header_crc = crc32c(&header);
    put_u32_at(&mut header, header_crc_pos, header_crc);
    file.extend_from_slice(&header);

    let npart = 1_u16;
    let nofe = 1_u16;
    let max_layers = 1_u16;

    put_u32(&mut file, hillslope_id);
    put_u32(&mut file, 1);
    put_i32(&mut file, 2004);
    put_u16(&mut file, npart);
    put_u16(&mut file, nofe);
    put_u16(&mut file, max_layers);
    put_string(&mut file, "gregorian");
    put_u16(&mut file, 1);
    put_u8(&mut file, 1);

    put_string(&mut file, "p1.cli");
    put_i64(&mut file, 0);
    put_u32(&mut file, u32::from(npart));
    put_f64(&mut file, 0.001);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);

    put_u32(&mut file, 1);
    put_u32(&mut file, 1);
    put_i32(&mut file, 2004);
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

fn build_event_payload(
    duration_seconds: f64,
    peak_runoff_m3_s: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    hourly_runoff_volume_m3: [f64; 24],
    hourly_sediment_mass_kg: [f64; 24],
) -> Vec<u8> {
    let mut payload = Vec::new();
    put_u32(&mut payload, 1);
    put_i32(&mut payload, 2004);
    put_u16(&mut payload, 1);
    put_u8(&mut payload, 2);
    put_u16(&mut payload, SUPPORTED_MINOR_V1);
    put_u16(&mut payload, REQUIRED_STATE_IDS.len() as u16);
    put_f64(&mut payload, duration_seconds);
    put_f64(&mut payload, 0.5);
    put_f64(&mut payload, 0.8);
    for _ in 0..6 {
        put_i64(&mut payload, 0);
    }
    put_f64(&mut payload, peak_runoff_m3_s);
    put_i64(&mut payload, scaled_i64(total_detachment_kg));
    put_i64(&mut payload, scaled_i64(total_deposition_kg));
    put_u32(&mut payload, 1);
    put_f64(&mut payload, 0.25);
    put_u32(&mut payload, 1);
    put_f64(&mut payload, 1.0);
    put_u32(&mut payload, 24);
    for volume_m3 in hourly_runoff_volume_m3 {
        put_f64(&mut payload, volume_m3);
    }
    put_u32(&mut payload, 24);
    for sediment_kg in hourly_sediment_mass_kg {
        put_f64(&mut payload, sediment_kg);
    }
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);

    for state_id in REQUIRED_STATE_IDS {
        payload.extend_from_slice(&build_state_entry(*state_id));
    }

    payload
}

fn expected_state_schema(state_id: u16) -> Option<(u8, u8, u16, u8, u8)> {
    match state_id {
        1 => Some((1, 1, 1, 1, DIM_NOFE)),
        2..=5 | 100..=102 | 210 | 900 | 901 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        6 | 7 => Some((1, 2, 3, 2, DIM_NOFE_LAYERS)),
        103 | 104 | 200 | 202..=209 => Some((1, 1, 2, 1, DIM_NOFE)),
        201 => Some((1, 2, 4, 1, DIM_NOFE)),
        300 => Some((1, 1, 5, 0, DIM_SCALAR)),
        _ => None,
    }
}

fn build_state_entry(state_id: u16) -> Vec<u8> {
    let nofe = 1_u32;
    let max_layers = 1_u32;
    let (required_flag, representation_class, unit_class, rank, dims_kind) =
        expected_state_schema(state_id).expect("required state schema should exist");
    let dims = state_dims(dims_kind, nofe, max_layers);
    assert_eq!(dims.len(), usize::from(rank));

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

fn state_dims(dims_kind: u8, nofe: u32, max_layers: u32) -> Vec<u32> {
    match dims_kind {
        DIM_SCALAR => vec![],
        DIM_NOFE => vec![nofe],
        DIM_NOFE_LAYERS => vec![nofe, max_layers],
        _ => panic!("unknown dims_kind {dims_kind}"),
    }
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

fn scaled_i64(value: f64) -> i64 {
    let scaled = value * SCALE_INV_I64;
    assert!(scaled.is_finite());
    assert!(scaled >= i64::MIN as f64 && scaled <= i64::MAX as f64);
    scaled.round() as i64
}

fn crc32c(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for value in data {
        crc ^= u32::from(*value);
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
