#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::match_same_arms,
    clippy::unreadable_literal
)]

use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use flate2::Compression;
use flate2::write::ZlibEncoder;
use openwepp_input_contract::parsers::hbp::{
    HbpFormatErrorCode, HbpParseError, HbpParseMode, HbpParseOptions, HbpPathResolution,
    HbpSchemaProfile, HbpWarningCode as ParserHbpWarningCode, parse_hbp_from_bytes,
    parse_hbp_from_path,
};
use openwepp_legacy_bridge::hbp::{
    HbpAdapterRequest, HbpHeaderContract, HbpMagicSource, HbpWarningCode as BridgeHbpWarningCode,
    adapt_hbp_header,
};
use openwepp_legacy_bridge::policy::CompatibilityPolicy;

const MAGIC: &[u8; 8] = b"WFPHBP01";
const FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
const SUPPORTED_MAJOR_V1: u16 = 1;
const SUPPORTED_MAJOR_V2: u16 = 2;
const PAYLOAD_CODEC_ZLIB: u8 = 1;
const DIM_SCALAR: u8 = 0;
const DIM_NOFE: u8 = 1;
const DIM_NOFE_LAYERS: u8 = 2;
const DIR_V2_ROW_SIZE: usize = 29;
const TABLE_V2_ENTRY_SIZE: usize = 37;

const REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 300, 900, 901,
];

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

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

fn build_no_event_payload(
    sim_year_index: u32,
    calendar_year: i32,
    julian_day: u16,
    nofe: u32,
    max_layers: u32,
) -> Vec<u8> {
    let mut payload = Vec::new();
    put_u32(&mut payload, sim_year_index);
    put_i32(&mut payload, calendar_year);
    put_u16(&mut payload, julian_day);
    put_u8(&mut payload, 0); // NO_EVENT
    put_u16(&mut payload, 0); // payload_minor
    put_u16(&mut payload, REQUIRED_STATE_IDS.len() as u16);
    put_i64(&mut payload, 0); // baseflow_volume_m3
    put_i64(&mut payload, 0); // dissolved_storage_volume_m3

    for state_id in REQUIRED_STATE_IDS {
        payload.extend_from_slice(&build_state_entry(*state_id, nofe, max_layers));
    }

    payload
}

fn append_common_prefix(
    schema_major: u16,
    schema_minor: u16,
    nyear: u32,
    begin_year: i32,
    simulation_mode: u8,
    hillslope_id: u32,
) -> Vec<u8> {
    let mut file = Vec::new();

    let mut header = Vec::new();
    header.extend_from_slice(MAGIC);
    put_u16(&mut header, schema_major);
    put_u16(&mut header, schema_minor);
    put_u8(&mut header, 1); // little-endian

    let header_bytes_pos = header.len();
    put_u32(&mut header, 0); // header_bytes placeholder
    header.extend_from_slice(&[0u8; 32]); // compatibility_id
    put_u8(&mut header, 1); // artifact_role = hillslope_shard
    put_string(&mut header, "openwepp-hbp-test");
    put_string(&mut header, "run-integration-test");
    put_string(&mut header, "2026-05-21T00:00:00Z");
    put_string(&mut header, "metric-v1");
    header.extend_from_slice(&[0u8; 32]); // state_registry_id

    let header_crc_pos = header.len();
    put_u32(&mut header, 0); // header_crc32c placeholder
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
    put_u16(&mut file, 1); // event_enum_version
    put_u8(&mut file, simulation_mode);

    put_string(&mut file, "p1.cli");
    put_i64(&mut file, 0); // area scaled
    put_u32(&mut file, npart as u32);
    put_f64(&mut file, 0.001); // particle diameter
    put_f64(&mut file, 0.0); // srp
    put_f64(&mut file, 0.0); // slfp
    put_f64(&mut file, 0.0); // bfp
    put_f64(&mut file, 0.0); // scp

    put_u32(&mut file, nyear);
    for sim_year_index in 1..=nyear {
        put_u32(&mut file, sim_year_index);
        put_i32(&mut file, begin_year + (sim_year_index - 1) as i32);
        if schema_major == SUPPORTED_MAJOR_V2 {
            put_u16(&mut file, 366);
            put_u16(&mut file, 1);
            put_u16(&mut file, 366);
            put_u8(&mut file, 0);
        } else {
            put_u16(&mut file, 1);
            put_u16(&mut file, 1);
            put_u16(&mut file, 1);
            put_u8(&mut file, 0);
        }
    }

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

fn build_schema1_fixture(hillslope_id: u32) -> Vec<u8> {
    let mut file = append_common_prefix(SUPPORTED_MAJOR_V1, 0, 1, 2004, 1, hillslope_id);
    let payload = build_no_event_payload(1, 2004, 1, 1, 1);
    let payload_crc = crc32c(&payload);

    let directory_start = file.len();
    let directory_len = 4 + 27;
    let payload_offset = directory_start + directory_len;

    let mut directory = Vec::new();
    put_u32(&mut directory, 1); // record_count
    put_u32(&mut directory, 1); // sim_year_index
    put_i32(&mut directory, 2004);
    put_u16(&mut directory, 1); // julian day
    put_u8(&mut directory, 0); // NO_EVENT
    put_u64(&mut directory, payload_offset as u64);
    put_u32(&mut directory, payload.len() as u32);
    put_u32(&mut directory, payload_crc);

    file.extend_from_slice(&directory);
    file.extend_from_slice(&payload);

    let directory_crc = crc32c(&directory);
    put_u32(&mut file, directory_crc);

    let file_crc_pos = file.len();
    put_u32(&mut file, 0); // file crc placeholder
    put_u32(&mut file, 1); // footer record count
    file.extend_from_slice(FOOTER_MAGIC);

    let file_crc = crc32c(&file);
    put_u32_at(&mut file, file_crc_pos, file_crc);

    file
}

fn build_schema2_fixture(hillslope_id: u32) -> Vec<u8> {
    let nyear = 1u32;
    let begin_year = 2004i32;
    let mut file = append_common_prefix(SUPPORTED_MAJOR_V2, 0, nyear, begin_year, 1, hillslope_id);

    let mut raw_offsets = Vec::with_capacity(366);
    let mut raw_lengths = Vec::with_capacity(366);
    let mut raw_payload_crcs = Vec::with_capacity(366);
    let mut raw_block = Vec::new();

    for day in 1..=366u16 {
        let payload = build_no_event_payload(1, begin_year, day, 1, 1);
        raw_offsets.push(raw_block.len() as u32);
        raw_lengths.push(payload.len() as u32);
        raw_payload_crcs.push(crc32c(&payload));
        raw_block.extend_from_slice(&payload);
    }

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(&raw_block)
        .expect("schema2 raw block should compress");
    let stored_block = encoder.finish().expect("zlib encoder should finish");

    let stored_crc = crc32c(&stored_block);
    let raw_block_crc = crc32c(&raw_block);

    let directory_start = file.len();
    let directory_len = 4 + 366 * DIR_V2_ROW_SIZE;
    let table_start = directory_start + directory_len;
    let table_len = 4 + TABLE_V2_ENTRY_SIZE;
    let payload_block_region_start = table_start + table_len;
    let stored_block_offset = payload_block_region_start as u64;

    let mut directory = Vec::new();
    put_u32(&mut directory, 366);
    for day in 1..=366u16 {
        let index = (day - 1) as usize;
        put_u32(&mut directory, 1); // sim_year_index
        put_i32(&mut directory, begin_year);
        put_u16(&mut directory, day);
        put_u8(&mut directory, 0); // NO_EVENT
        put_u32(&mut directory, 0); // payload_block_id
        put_u16(&mut directory, day - 1); // day_in_block_index
        put_u32(&mut directory, raw_offsets[index]);
        put_u32(&mut directory, raw_lengths[index]);
        put_u32(&mut directory, raw_payload_crcs[index]);
    }

    let mut table = Vec::new();
    put_u32(&mut table, 1); // block_count
    put_u32(&mut table, 0); // payload_block_id
    put_u32(&mut table, 1); // sim_year_index
    put_u16(&mut table, 366); // block_day_slot_count
    put_u16(&mut table, 366); // represented_day_count
    put_u64(&mut table, stored_block_offset);
    put_u32(&mut table, stored_block.len() as u32);
    put_u32(&mut table, raw_block.len() as u32);
    put_u8(&mut table, PAYLOAD_CODEC_ZLIB);
    put_u32(&mut table, stored_crc);
    put_u32(&mut table, raw_block_crc);

    file.extend_from_slice(&directory);
    file.extend_from_slice(&table);
    file.extend_from_slice(&stored_block);

    let directory_crc = crc32c(&directory);
    let table_crc = crc32c(&table);
    put_u32(&mut file, directory_crc);
    put_u32(&mut file, table_crc);

    let file_crc_pos = file.len();
    put_u32(&mut file, 0); // file crc placeholder
    put_u32(&mut file, 366); // footer record_count
    put_u32(&mut file, 1); // footer block_count
    file.extend_from_slice(FOOTER_MAGIC);

    let file_crc = crc32c(&file);
    put_u32_at(&mut file, file_crc_pos, file_crc);

    file
}

fn unique_temp_path(file_name: &str) -> PathBuf {
    let run_id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let now_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("wall clock should be after epoch")
        .as_nanos();

    let directory = std::env::temp_dir().join(format!("openwepp_hbp_{now_ns}_{run_id}"));
    fs::create_dir_all(&directory).expect("temp directory should be creatable");
    directory.join(file_name)
}

fn write_fixture(path: &Path, bytes: &[u8]) {
    let parent = path.parent().expect("fixture path should have parent");
    fs::create_dir_all(parent).expect("fixture parent should be creatable");
    fs::write(path, bytes).expect("fixture bytes should write");
}

#[test]
fn strict_schema1_parse_succeeds_with_expected_hillslope_id() {
    let bytes = build_schema1_fixture(24);
    let parsed = parse_hbp_from_bytes(
        &bytes,
        Path::new("H24.hbp"),
        HbpParseOptions {
            mode: HbpParseMode::Strict,
            expected_hillslope_id: Some(24),
        },
    )
    .expect("schema1 fixture should parse");

    assert_eq!(parsed.schema_profile, HbpSchemaProfile::Schema1x);
    assert_eq!(parsed.schema_major, 1);
    assert_eq!(parsed.schema_minor, 0);
    assert_eq!(parsed.hillslope_id, 24);
    assert_eq!(parsed.nyear, 1);
    assert_eq!(parsed.record_count, 1);
    assert_eq!(parsed.block_count, 0);
    assert_eq!(parsed.directory_entries.len(), 1);
    assert!(parsed.payload_blocks.is_empty());
    assert_eq!(parsed.path_resolution, HbpPathResolution::Direct);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_schema2_parse_succeeds() {
    let bytes = build_schema2_fixture(7);
    let parsed = parse_hbp_from_bytes(
        &bytes,
        Path::new("H7.hbp"),
        HbpParseOptions {
            mode: HbpParseMode::Strict,
            expected_hillslope_id: Some(7),
        },
    )
    .expect("schema2 fixture should parse");

    assert_eq!(parsed.schema_profile, HbpSchemaProfile::Schema2x);
    assert_eq!(parsed.schema_major, 2);
    assert_eq!(parsed.schema_minor, 0);
    assert_eq!(parsed.record_count, 366);
    assert_eq!(parsed.block_count, 1);
    assert_eq!(parsed.payload_blocks.len(), 1);
    assert_eq!(parsed.directory_entries.len(), 366);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn compatibility_mode_derives_hbp_path_from_pass_dat() {
    let fixture = build_schema1_fixture(9);
    let temp_hbp_path = unique_temp_path("H9.hbp");
    write_fixture(&temp_hbp_path, &fixture);

    let legacy_path = temp_hbp_path
        .with_file_name("H9.pass.dat")
        .to_string_lossy()
        .to_string();

    let parsed = parse_hbp_from_path(
        &legacy_path,
        HbpParseOptions {
            mode: HbpParseMode::Compatibility,
            expected_hillslope_id: Some(9),
        },
    )
    .expect("compatibility mode should derive .hbp from .pass.dat path");

    assert_eq!(
        parsed.path_resolution,
        HbpPathResolution::DerivedFromLegacyPassDat
    );
    assert_eq!(parsed.resolved_path, temp_hbp_path);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ParserHbpWarningCode::HbpW001)
    );
}

#[test]
fn strict_mode_rejects_pass_dat_input_name() {
    let fixture = build_schema1_fixture(11);
    let temp_hbp_path = unique_temp_path("H11.hbp");
    write_fixture(&temp_hbp_path, &fixture);

    let legacy_path = temp_hbp_path
        .with_file_name("H11.pass.dat")
        .to_string_lossy()
        .to_string();

    let error = parse_hbp_from_path(
        &legacy_path,
        HbpParseOptions {
            mode: HbpParseMode::Strict,
            expected_hillslope_id: None,
        },
    )
    .expect_err("strict mode must reject .pass.dat naming");

    assert!(matches!(error, HbpParseError::InvalidProcessHbpName { .. }));
    assert_eq!(error.contract_error_id(), "HBP-E-001");
}

#[test]
fn forbidden_pass_suffix_is_rejected() {
    let error = parse_hbp_from_bytes(
        &build_schema1_fixture(1),
        Path::new("H1.pass.hbp"),
        HbpParseOptions::strict(),
    )
    .expect_err("forbidden pass suffix should fail");

    assert!(matches!(error, HbpParseError::InvalidProcessHbpName { .. }));
    assert_eq!(error.contract_error_id(), "HBP-E-001");
}

#[test]
fn non_enoent_open_error_is_typed_hbp_e_000() {
    let directory_path = unique_temp_path("H19.hbp");
    fs::create_dir_all(&directory_path).expect("directory path should be creatable");

    let error = parse_hbp_from_path(
        &directory_path,
        HbpParseOptions {
            mode: HbpParseMode::Strict,
            expected_hillslope_id: None,
        },
    )
    .expect_err("directory path should fail as non-ENOENT open error");

    assert!(matches!(error, HbpParseError::InputOpenError { .. }));
    assert_eq!(error.contract_error_id(), "HBP-E-000");
}

#[test]
fn bad_magic_maps_to_hbp_e_002() {
    let mut bytes = build_schema1_fixture(3);
    bytes[0] = b'B';

    let error = parse_hbp_from_bytes(&bytes, Path::new("H3.hbp"), HbpParseOptions::strict())
        .expect_err("bad magic should be rejected");

    assert!(matches!(
        error,
        HbpParseError::FormatViolation {
            code: HbpFormatErrorCode::HbpE002,
            ..
        }
    ));
    assert_eq!(error.contract_error_id(), "HBP-E-002");
}

#[test]
fn header_truncation_maps_to_hbp_e_013() {
    let bytes = MAGIC.to_vec();

    let error = parse_hbp_from_bytes(&bytes, Path::new("H1.hbp"), HbpParseOptions::strict())
        .expect_err("truncated header should fail");

    assert!(matches!(
        error,
        HbpParseError::FormatViolation {
            code: HbpFormatErrorCode::HbpE013,
            ..
        }
    ));
    assert_eq!(error.contract_error_id(), "HBP-E-013");
}

#[test]
fn footer_corruption_maps_to_hbp_e_012() {
    let mut bytes = build_schema1_fixture(6);
    let length = bytes.len();
    bytes[length - 1] ^= 0x01;

    let error = parse_hbp_from_bytes(&bytes, Path::new("H6.hbp"), HbpParseOptions::strict())
        .expect_err("footer corruption should fail");

    assert!(matches!(
        error,
        HbpParseError::FormatViolation {
            code: HbpFormatErrorCode::HbpE012,
            ..
        }
    ));
    assert_eq!(error.contract_error_id(), "HBP-E-012");
}

#[test]
fn expected_hillslope_id_mismatch_maps_to_hbp_e_014() {
    let bytes = build_schema1_fixture(15);

    let error = parse_hbp_from_bytes(
        &bytes,
        Path::new("H15.hbp"),
        HbpParseOptions {
            mode: HbpParseMode::Strict,
            expected_hillslope_id: Some(99),
        },
    )
    .expect_err("hillslope id mismatch should fail");

    assert!(matches!(error, HbpParseError::HillslopeIdMismatch { .. }));
    assert_eq!(error.contract_error_id(), "HBP-E-014");
}

fn bridge_hbp_contract() -> HbpHeaderContract {
    HbpHeaderContract::new(*b"HBP1", vec![*b"HBP0"], 8)
}

#[test]
fn parser_and_bridge_share_hbp_w_001_warning_id() {
    assert_eq!(
        ParserHbpWarningCode::HbpW001.as_str(),
        BridgeHbpWarningCode::LegacyMagicAliasApplied.message_id()
    );
    assert_eq!(ParserHbpWarningCode::HbpW001.as_str(), "HBP-W-001");
}

#[test]
fn strict_policy_rejects_legacy_forms_across_parser_and_bridge() {
    let parser_fixture = build_schema1_fixture(20);
    let parser_path = unique_temp_path("H20.hbp");
    write_fixture(&parser_path, &parser_fixture);
    let parser_legacy_path = parser_path
        .with_file_name("H20.pass.dat")
        .to_string_lossy()
        .to_string();

    let parser_error = parse_hbp_from_path(
        &parser_legacy_path,
        HbpParseOptions {
            mode: HbpParseMode::Strict,
            expected_hillslope_id: None,
        },
    )
    .expect_err("strict parser mode must reject .pass.dat legacy naming");
    assert_eq!(parser_error.contract_error_id(), "HBP-E-001");

    let bridge_error = adapt_hbp_header(&HbpAdapterRequest {
        policy: CompatibilityPolicy::Strict,
        contract: bridge_hbp_contract(),
        shard_bytes: b"HBP0DATA",
    })
    .expect_err("strict bridge mode must reject legacy magic aliases");
    assert_eq!(bridge_error.code(), "HBP-E-006");
}

#[test]
fn compatibility_policy_accepts_legacy_forms_with_hbp_w_001() {
    let parser_fixture = build_schema1_fixture(21);
    let parser_path = unique_temp_path("H21.hbp");
    write_fixture(&parser_path, &parser_fixture);
    let parser_legacy_path = parser_path
        .with_file_name("H21.pass.dat")
        .to_string_lossy()
        .to_string();

    let parser_parsed = parse_hbp_from_path(
        &parser_legacy_path,
        HbpParseOptions {
            mode: HbpParseMode::Compatibility,
            expected_hillslope_id: Some(21),
        },
    )
    .expect("compat parser mode should derive .hbp from .pass.dat");
    assert_eq!(
        parser_parsed.path_resolution,
        HbpPathResolution::DerivedFromLegacyPassDat
    );
    assert!(
        parser_parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ParserHbpWarningCode::HbpW001)
    );

    let bridge_response = adapt_hbp_header(&HbpAdapterRequest {
        policy: CompatibilityPolicy::Compat,
        contract: bridge_hbp_contract(),
        shard_bytes: b"HBP0DATA",
    })
    .expect("compat bridge mode should accept configured legacy alias");
    assert_eq!(bridge_response.magic_source, HbpMagicSource::LegacyAlias);
    assert_eq!(bridge_response.warnings.len(), 1);
    assert_eq!(
        bridge_response.warnings[0].code.message_id(),
        ParserHbpWarningCode::HbpW001.as_str()
    );
}

#[test]
fn required_state_registry_ids_are_covered_by_fixture_schema() {
    let mut set = HashSet::new();
    for state_id in REQUIRED_STATE_IDS {
        let schema = expected_state_schema(*state_id);
        assert!(schema.is_some(), "schema missing for state {state_id}");
        set.insert(*state_id);
    }
    assert_eq!(set.len(), REQUIRED_STATE_IDS.len());
}
