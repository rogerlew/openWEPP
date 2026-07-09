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
    HbpFormatErrorCode, HbpLatestEventState, HbpNoEventKind, HbpParseError, HbpParseOptions,
    HbpPathResolution, HbpSchemaProfile, HbpWarningCode as ParserHbpWarningCode,
    parse_hbp_from_bytes, parse_hbp_from_bytes_with_latest_event_payload,
    parse_hbp_from_bytes_with_latest_event_state, parse_hbp_from_path,
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

fn put_u16_at(buf: &mut [u8], offset: usize, value: u16) {
    buf[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn put_u64_at(buf: &mut [u8], offset: usize, value: u64) {
    buf[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn put_f64_at(buf: &mut [u8], offset: usize, value: f64) {
    buf[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn read_u32_at(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(buf[offset..offset + 4].try_into().expect("u32 window"))
}

fn read_u64_at(buf: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(buf[offset..offset + 8].try_into().expect("u64 window"))
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

fn build_schema1_duplicate_key_fixture(hillslope_id: u32) -> Vec<u8> {
    let mut file = append_common_prefix(SUPPORTED_MAJOR_V1, 0, 1, 2004, 1, hillslope_id);
    let year_start = year_count_offset(&file) + 4;
    put_u16_at(&mut file, year_start + 8, 2);
    put_u16_at(&mut file, year_start + 12, 2);

    let payload = build_no_event_payload(1, 2004, 1, 1, 1);
    let payload_crc = crc32c(&payload);

    let directory_start = file.len();
    let directory_len = 4 + 2 * 27;
    let first_payload_offset = directory_start + directory_len;
    let second_payload_offset = first_payload_offset + payload.len();

    let mut directory = Vec::new();
    put_u32(&mut directory, 2);
    for payload_offset in [first_payload_offset, second_payload_offset] {
        put_u32(&mut directory, 1);
        put_i32(&mut directory, 2004);
        put_u16(&mut directory, 1);
        put_u8(&mut directory, 0);
        put_u64(&mut directory, payload_offset as u64);
        put_u32(&mut directory, payload.len() as u32);
        put_u32(&mut directory, payload_crc);
    }

    file.extend_from_slice(&directory);
    file.extend_from_slice(&payload);
    file.extend_from_slice(&payload);

    let directory_crc = crc32c(&directory);
    put_u32(&mut file, directory_crc);

    let file_crc_pos = file.len();
    put_u32(&mut file, 0);
    put_u32(&mut file, 2);
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

fn header_bytes(bytes: &[u8]) -> usize {
    read_u32_at(bytes, 13) as usize
}

fn skip_encoded_string(bytes: &[u8], offset: usize) -> usize {
    let length = read_u32_at(bytes, offset) as usize;
    offset + 4 + length
}

fn simulation_mode_offset(bytes: &[u8]) -> usize {
    let mut offset = header_bytes(bytes) + 4 + 4 + 4 + 2 + 2 + 2;
    offset = skip_encoded_string(bytes, offset);
    offset + 2
}

fn particle_count_offset(bytes: &[u8]) -> usize {
    let mut offset = simulation_mode_offset(bytes) + 1;
    offset = skip_encoded_string(bytes, offset);
    offset + 8
}

fn particle_diameter_offset(bytes: &[u8]) -> usize {
    particle_count_offset(bytes) + 4
}

fn year_count_offset(bytes: &[u8]) -> usize {
    let particle_count_offset = particle_count_offset(bytes);
    let particle_count = read_u32_at(bytes, particle_count_offset) as usize;
    particle_count_offset + 4 + 8 * particle_count + 8 * 4
}

fn registry_count_offset(bytes: &[u8]) -> usize {
    let year_count_offset = year_count_offset(bytes);
    let year_count = read_u32_at(bytes, year_count_offset) as usize;
    year_count_offset + 4 + 15 * year_count
}

fn registry_state_id_offsets(bytes: &[u8]) -> Vec<usize> {
    let mut offset = registry_count_offset(bytes);
    let registry_count = read_u32_at(bytes, offset) as usize;
    offset += 4;

    let mut offsets = Vec::with_capacity(registry_count);
    for _ in 0..registry_count {
        offsets.push(offset);
        offset += 2 + 1 + 1 + 2 + 1 + 1;
        offset = skip_encoded_string(bytes, offset);
    }
    offsets
}

fn directory_start(bytes: &[u8]) -> usize {
    let offsets = registry_state_id_offsets(bytes);
    let last = *offsets.last().expect("registry must contain states");
    let mut offset = last + 2 + 1 + 1 + 2 + 1 + 1;
    offset = skip_encoded_string(bytes, offset);
    offset
}

fn directory_end(bytes: &[u8], schema_major: u16) -> usize {
    let start = directory_start(bytes);
    let record_count = read_u32_at(bytes, start) as usize;
    let row_size = if schema_major == SUPPORTED_MAJOR_V2 {
        DIR_V2_ROW_SIZE
    } else {
        27
    };
    start + 4 + record_count * row_size
}

fn schema1_footer_start(bytes: &[u8]) -> usize {
    bytes.len() - 20
}

fn schema2_table_start(bytes: &[u8]) -> usize {
    directory_end(bytes, SUPPORTED_MAJOR_V2)
}

fn schema2_table_end(bytes: &[u8]) -> usize {
    let table_start = schema2_table_start(bytes);
    let block_count = read_u32_at(bytes, table_start) as usize;
    table_start + 4 + block_count * TABLE_V2_ENTRY_SIZE
}

fn schema2_footer_start(bytes: &[u8]) -> usize {
    bytes.len() - 28
}

fn refresh_schema1_file_crc(bytes: &mut [u8]) {
    let file_crc_pos = schema1_footer_start(bytes) + 4;
    put_u32_at(bytes, file_crc_pos, 0);
    let file_crc = crc32c(bytes);
    put_u32_at(bytes, file_crc_pos, file_crc);
}

fn refresh_schema1_directory_crc(bytes: &mut [u8]) {
    let directory_start = directory_start(bytes);
    let directory_end = directory_end(bytes, SUPPORTED_MAJOR_V1);
    let footer_start = schema1_footer_start(bytes);
    put_u32_at(
        bytes,
        footer_start,
        crc32c(&bytes[directory_start..directory_end]),
    );
}

fn refresh_schema1_crcs(bytes: &mut [u8]) {
    refresh_schema1_directory_crc(bytes);
    refresh_schema1_file_crc(bytes);
}

fn schema1_payload_range(bytes: &[u8]) -> (usize, usize) {
    let directory_pos = directory_start(bytes);
    let payload_offset = read_u64_at(bytes, directory_pos + 4 + 11) as usize;
    let payload_length = read_u32_at(bytes, directory_pos + 4 + 19) as usize;
    (payload_offset, payload_offset + payload_length)
}

fn refresh_schema1_payload_crc(bytes: &mut [u8]) {
    let directory_pos = directory_start(bytes);
    let (payload_start, payload_end) = schema1_payload_range(bytes);
    let payload_crc = crc32c(&bytes[payload_start..payload_end]);
    put_u32_at(bytes, directory_pos + 4 + 23, payload_crc);
}

fn mutate_schema1_payload(bytes: &mut [u8], mutate: impl FnOnce(&mut [u8])) {
    let (payload_start, payload_end) = schema1_payload_range(bytes);
    mutate(&mut bytes[payload_start..payload_end]);
    refresh_schema1_payload_crc(bytes);
    refresh_schema1_crcs(bytes);
}

fn refresh_schema2_directory_crc(bytes: &mut [u8]) {
    let directory_start = directory_start(bytes);
    let directory_end = directory_end(bytes, SUPPORTED_MAJOR_V2);
    let footer_start = schema2_footer_start(bytes);
    put_u32_at(
        bytes,
        footer_start,
        crc32c(&bytes[directory_start..directory_end]),
    );
}

fn refresh_schema2_table_crc(bytes: &mut [u8]) {
    let table_start = schema2_table_start(bytes);
    let table_end = schema2_table_end(bytes);
    let footer_start = schema2_footer_start(bytes);
    put_u32_at(
        bytes,
        footer_start + 4,
        crc32c(&bytes[table_start..table_end]),
    );
}

fn refresh_schema2_file_crc(bytes: &mut [u8]) {
    let file_crc_pos = schema2_footer_start(bytes) + 8;
    put_u32_at(bytes, file_crc_pos, 0);
    let file_crc = crc32c(bytes);
    put_u32_at(bytes, file_crc_pos, file_crc);
}

fn refresh_schema2_crcs(bytes: &mut [u8]) {
    refresh_schema2_directory_crc(bytes);
    refresh_schema2_table_crc(bytes);
    refresh_schema2_file_crc(bytes);
}

fn assert_layout_format_error(bytes: &[u8], code: HbpFormatErrorCode, detail: &str) {
    let error = parse_hbp_from_bytes(bytes, Path::new("H1.hbp"), HbpParseOptions::strict())
        .expect_err("mutated layout fixture should fail");
    match error {
        HbpParseError::FormatViolation {
            code: actual_code,
            detail: actual_detail,
        } => {
            assert_eq!(actual_code, code);
            assert!(
                actual_detail.contains(detail),
                "expected detail containing '{detail}', got '{actual_detail}'"
            );
        }
        other => panic!("expected format violation, got {other:?}"),
    }
}

fn assert_truncated_layout_error(bytes: &[u8], truncate_at: usize, detail: &str) {
    let mut truncated = bytes.to_vec();
    truncated.truncate(truncate_at);
    assert_layout_format_error(&truncated, HbpFormatErrorCode::HbpE013, detail);
}

#[test]
fn payload_validator_crc_and_schema2_raw_payload_crc_guards_are_typed() {
    let mut bytes = build_schema1_fixture(1);
    let (payload_start, _) = schema1_payload_range(&bytes);
    bytes[payload_start] ^= 0x01;
    refresh_schema1_file_crc(&mut bytes);
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE012, "payload crc mismatch");

    let mut bytes = build_schema2_fixture(1);
    let directory_pos = directory_start(&bytes);
    let raw_payload_crc_pos = directory_pos + 4 + 25;
    put_u32_at(&mut bytes, raw_payload_crc_pos, 123);
    refresh_schema2_directory_crc(&mut bytes);
    refresh_schema2_file_crc(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE012,
        "raw payload crc mismatch",
    );
}

#[test]
fn payload_validator_header_guards_are_typed() {
    let mut bytes = build_schema1_fixture(1);
    mutate_schema1_payload(&mut bytes, |payload| {
        put_u16_at(payload, 8, 2);
    });
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE010,
        "payload and directory key mismatch",
    );

    // SC-INFILE-HBP-001 v0.2.0: payload minor 1 is now SUPPORTED (the
    // ADR-0036 hourly extension), so the unsupported-minor guard probes
    // the new boundary (minor 2) — a newer payload is still rejected
    // loudly, never silently mis-parsed.
    let mut bytes = build_schema1_fixture(1);
    mutate_schema1_payload(&mut bytes, |payload| {
        put_u16_at(payload, 11, 2);
    });
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE013,
        "unsupported payload minor",
    );

    // A NO-EVENT payload claiming minor 1 parses (there is no runoff body
    // to mis-read); the runoff-body fail-closed behavior for a minor-1
    // claim without the hourly block is exercised by the strict
    // count-prefixed reads (count-mismatch on the reserved i64 region) —
    // covered end-to-end by the p61 minor-1 round-trip.
}

#[test]
fn latest_event_state_represents_no_event_without_synthesizing_event_payload() {
    let bytes = build_schema1_fixture(1);
    let (_, latest_event_state) = parse_hbp_from_bytes_with_latest_event_state(
        &bytes,
        Path::new("H1.hbp"),
        HbpParseOptions {
            expected_hillslope_id: Some(1),
        },
    )
    .expect("schema-1 no-event fixture should parse");
    let HbpLatestEventState::NoEvent(no_event) =
        latest_event_state.expect("fixture should expose latest no-event state")
    else {
        panic!("latest HBP state should be NoEvent");
    };
    assert_eq!(no_event.source_event_kind, HbpNoEventKind::NoEvent);
    assert_eq!(no_event.julian_day, 1);
    assert!(no_event.baseflow_volume_m3.abs() <= f64::EPSILON);
    assert!(no_event.deep_seepage_volume_m3.abs() <= f64::EPSILON);

    let (_, latest_event_payload) = parse_hbp_from_bytes_with_latest_event_payload(
        &bytes,
        Path::new("H1.hbp"),
        HbpParseOptions {
            expected_hillslope_id: Some(1),
        },
    )
    .expect("compat latest-event-payload API should parse no-event fixture");
    assert!(
        latest_event_payload.is_none(),
        "compat EventPayload API must not synthesize runoff payload from NoEvent state"
    );
}

#[test]
fn payload_validator_state_snapshot_guards_are_typed() {
    let first_state_offset = 15 + 16;

    let mut bytes = build_schema1_fixture(1);
    mutate_schema1_payload(&mut bytes, |payload| {
        let second_state_offset =
            first_state_offset + 6 + read_u32_at(payload, first_state_offset + 2) as usize;
        put_u16_at(payload, second_state_offset, 1);
    });
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE013, "duplicate state id");

    let mut bytes = build_schema1_fixture(1);
    mutate_schema1_payload(&mut bytes, |payload| {
        let length_offset = first_state_offset + 2;
        let entry_length = read_u32_at(payload, length_offset);
        put_u32_at(payload, length_offset, entry_length + 1);
    });
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE013,
        "state entry length mismatch",
    );

    let mut bytes = build_schema1_fixture(1);
    mutate_schema1_payload(&mut bytes, |payload| {
        put_u16_at(payload, first_state_offset, 902);
    });
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE013,
        "required state id missing: 1",
    );
}

#[test]
fn layout_header_control_fields_fail_closed_with_specific_codes() {
    let mut bytes = build_schema1_fixture(1);
    put_u16_at(&mut bytes, 8, 9);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE003,
        "unsupported schema major",
    );

    // SC-INFILE-HBP-001 v0.2.0: header minor 1 is now supported (the
    // ADR-0036 hourly extension); the unsupported-minor guards probe the
    // new boundary (minor 2).
    let mut bytes = build_schema1_fixture(1);
    put_u16_at(&mut bytes, 10, 2);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE004,
        "unsupported schema minor",
    );

    let mut bytes = build_schema2_fixture(1);
    put_u16_at(&mut bytes, 10, 2);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE004,
        "unsupported schema minor",
    );

    let mut bytes = build_schema1_fixture(1);
    bytes[12] = 2;
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE005,
        "unsupported endianness",
    );

    let mut bytes = build_schema1_fixture(1);
    let invalid_header_bytes = (bytes.len() + 1) as u32;
    put_u32_at(&mut bytes, 13, invalid_header_bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE006,
        "header length exceeds file length",
    );

    let mut bytes = build_schema1_fixture(1);
    bytes[49] = 2;
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE006,
        "unsupported artifact role",
    );

    let mut bytes = build_schema1_fixture(1);
    bytes[17] ^= 0x01;
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE007, "header crc mismatch");

    let mut bytes = build_schema1_fixture(1);
    let invalid_header_bytes = (header_bytes(&bytes) + 1) as u32;
    put_u32_at(&mut bytes, 13, invalid_header_bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE006,
        "header length mismatch",
    );
}

#[test]
fn layout_cursor_truncation_guards_report_typed_contexts() {
    let schema1 = build_schema1_fixture(1);
    let schema1_row = directory_start(&schema1) + 4;
    assert_truncated_layout_error(&schema1, schema1_row + 11, "day directory");
    assert_truncated_layout_error(&schema1, schema1_row + 19, "day directory");
    assert_truncated_layout_error(&schema1, schema1_row + 23, "day directory");

    let schema2 = build_schema2_fixture(1);
    let schema2_row = directory_start(&schema2) + 4;
    assert_truncated_layout_error(&schema2, schema2_row + 11, "day directory");
    assert_truncated_layout_error(&schema2, schema2_row + 15, "day directory");
    assert_truncated_layout_error(&schema2, schema2_row + 17, "day directory");
    assert_truncated_layout_error(&schema2, schema2_row + 21, "day directory");
    assert_truncated_layout_error(&schema2, schema2_row + 25, "day directory");

    let table_pos = schema2_table_start(&schema2);
    assert_truncated_layout_error(&schema2, table_pos, "payload block table");
    assert_truncated_layout_error(&schema2, table_pos + 4, "payload block table");
    assert_truncated_layout_error(&schema2, table_pos + 8, "payload block table");
    assert_truncated_layout_error(&schema2, table_pos + 12, "payload block table");
    assert_truncated_layout_error(&schema2, table_pos + 14, "payload block table");
    assert_truncated_layout_error(&schema2, table_pos + 16, "payload block table");
    assert_truncated_layout_error(&schema2, table_pos + 24, "payload block table");
    assert_truncated_layout_error(&schema2, table_pos + 28, "payload block table");
    assert_truncated_layout_error(&schema2, table_pos + 32, "payload block table");
    assert_truncated_layout_error(&schema2, table_pos + 33, "payload block table");
    assert_truncated_layout_error(&schema2, table_pos + 37, "payload block table");
}

#[test]
fn layout_dimension_metadata_and_year_table_guards_are_typed() {
    let mut bytes = build_schema2_fixture(1);
    let simulation_mode_offset = simulation_mode_offset(&bytes);
    bytes[simulation_mode_offset] = 0;
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE008,
        "schema 2.0 requires simulation_mode = 1",
    );

    let mut bytes = build_schema1_fixture(1);
    let particle_diameter_offset = particle_diameter_offset(&bytes);
    put_f64_at(&mut bytes, particle_diameter_offset, 0.0);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE006,
        "particle_diameter_m must be finite and > 0",
    );

    let mut bytes = build_schema1_fixture(1);
    let particle_count_offset = particle_count_offset(&bytes);
    put_u32_at(&mut bytes, particle_count_offset, 0);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE006,
        "event sediment count mismatch",
    );

    let mut bytes = build_schema1_fixture(1);
    let year_count_pos = year_count_offset(&bytes);
    put_u32_at(&mut bytes, year_count_pos, 2);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE008,
        "year table count mismatch",
    );

    let mut bytes = build_schema1_fixture(1);
    let year_count_pos = year_count_offset(&bytes);
    put_u32_at(&mut bytes, year_count_pos + 4, 2);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE008,
        "year table sim_year_index must be one-based and ordered",
    );

    let mut bytes = build_schema1_fixture(1);
    let year_count_pos = year_count_offset(&bytes);
    put_u16_at(&mut bytes, year_count_pos + 12, 0);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE008,
        "year table days_in_year must be positive",
    );
}

#[test]
fn layout_registry_guards_reject_duplicate_mismatch_and_missing_required_ids() {
    let mut bytes = build_schema1_fixture(1);
    let state_offsets = registry_state_id_offsets(&bytes);
    put_u16_at(&mut bytes, state_offsets[1], REQUIRED_STATE_IDS[0]);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE009,
        "duplicate registry state id",
    );

    let mut bytes = build_schema1_fixture(1);
    let state_offsets = registry_state_id_offsets(&bytes);
    bytes[state_offsets[0] + 2] = 0;
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE009,
        "state registry block does not match canonical schema",
    );

    let mut bytes = build_schema1_fixture(1);
    let state_offsets = registry_state_id_offsets(&bytes);
    let last_state_offset = *state_offsets.last().expect("state offsets");
    put_u16_at(&mut bytes, last_state_offset, 902);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE009,
        "required state id missing in registry",
    );
}

#[test]
fn schema1_directory_and_footer_layout_guards_are_typed() {
    let mut bytes = build_schema1_fixture(1);
    let directory_pos = directory_start(&bytes);
    put_u32_at(&mut bytes, directory_pos, 0);
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE010, "empty day directory");

    let mut bytes = build_schema1_fixture(1);
    let year_start = year_count_offset(&bytes) + 4;
    put_u16_at(&mut bytes, year_start + 8, 2);
    put_u16_at(&mut bytes, year_start + 12, 2);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE010,
        "directory record count must equal sum of year-table days",
    );

    let mut bytes = build_schema1_fixture(1);
    let directory_pos = directory_start(&bytes);
    put_u16_at(&mut bytes, directory_pos + 4 + 8, 2);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE010,
        "directory key is outside the year table",
    );

    let mut bytes = build_schema1_fixture(1);
    let directory_pos = directory_start(&bytes);
    put_u32_at(&mut bytes, directory_pos + 4 + 19, 0);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE010,
        "payload length must be positive",
    );

    let mut bytes = build_schema1_fixture(1);
    let directory_pos = directory_start(&bytes);
    put_u64_at(&mut bytes, directory_pos + 4 + 11, 1);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE010,
        "payload offsets are not deterministic",
    );

    let mut bytes = build_schema1_fixture(1);
    let footer_count_offset = schema1_footer_start(&bytes) + 8;
    put_u32_at(&mut bytes, footer_count_offset, 2);
    refresh_schema1_file_crc(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE012,
        "footer record count must equal sum of year-table days",
    );

    let mut bytes = build_schema1_fixture(1);
    let directory_pos = directory_start(&bytes);
    bytes[directory_pos + 4 + 10] = 1;
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE012,
        "directory crc mismatch",
    );

    let mut bytes = build_schema1_fixture(1);
    let file_crc_offset = schema1_footer_start(&bytes) + 4;
    bytes[file_crc_offset] ^= 0x01;
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE012, "file crc mismatch");

    let mut bytes = build_schema1_fixture(1);
    let footer_magic_offset = schema1_footer_start(&bytes) + 12;
    bytes[footer_magic_offset] ^= 0x01;
    refresh_schema1_file_crc(&mut bytes);
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE012, "bad footer magic");

    let mut bytes = build_schema1_fixture(1);
    bytes.truncate(bytes.len() - 1);
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE013, "truncated payload");

    let bytes = build_schema1_duplicate_key_fixture(1);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE010,
        "directory keys must be deterministic and strictly ordered",
    );
}

#[test]
fn schema2_block_table_and_footer_layout_guards_are_typed() {
    let mut bytes = build_schema2_fixture(1);
    let table_pos = schema2_table_start(&bytes);
    put_u32_at(&mut bytes, table_pos, 2);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x block count must equal year table count",
    );

    let mut bytes = build_schema2_fixture(1);
    let table_pos = schema2_table_start(&bytes);
    put_u32_at(&mut bytes, table_pos + 4, 1);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x payload_block_id must be contiguous and ordered",
    );

    let mut bytes = build_schema2_fixture(1);
    let table_pos = schema2_table_start(&bytes);
    put_u32_at(&mut bytes, table_pos + 8, 2);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x payload block sim_year_index mismatch",
    );

    let mut bytes = build_schema2_fixture(1);
    let table_pos = schema2_table_start(&bytes);
    put_u16_at(&mut bytes, table_pos + 12, 365);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.0 payload block day counts must be 366",
    );

    let mut bytes = build_schema2_fixture(1);
    let table_pos = schema2_table_start(&bytes);
    bytes[table_pos + 32] = 0;
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x payload codec is unsupported",
    );

    let mut bytes = build_schema2_fixture(1);
    let table_pos = schema2_table_start(&bytes);
    put_u32_at(&mut bytes, table_pos + 24, 0);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x payload block lengths must be positive",
    );

    let mut bytes = build_schema2_fixture(1);
    let footer_magic_offset = schema2_footer_start(&bytes) + 20;
    bytes[footer_magic_offset] ^= 0x01;
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE012, "bad footer magic");

    let mut bytes = build_schema2_fixture(1);
    bytes.truncate(schema2_table_end(&bytes) + 20);
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE013, "truncated payload");
}

#[test]
fn schema2_checksum_raw_block_and_day_slice_guards_are_typed() {
    schema2_checksum_and_footer_guards_are_typed();
    schema2_payload_block_bounds_are_typed();
    schema2_day_slice_bounds_are_typed();
}

fn schema2_checksum_and_footer_guards_are_typed() {
    let mut bytes = build_schema2_fixture(1);
    let footer_record_count = schema2_footer_start(&bytes) + 12;
    put_u32_at(&mut bytes, footer_record_count, 365);
    refresh_schema2_file_crc(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE012,
        "footer record count must equal sum of year-table days",
    );

    let mut bytes = build_schema2_fixture(1);
    let stored_block_offset = read_u64_at(&bytes, schema2_table_start(&bytes) + 16) as usize;
    bytes[stored_block_offset] ^= 0x01;
    refresh_schema2_file_crc(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE012,
        "schema 2.x stored block crc mismatch",
    );

    let mut bytes = build_schema2_fixture(1);
    let table_pos = schema2_table_start(&bytes);
    put_u32_at(&mut bytes, table_pos + 37, 123);
    refresh_schema2_table_crc(&mut bytes);
    refresh_schema2_file_crc(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE012,
        "schema 2.x raw block crc mismatch",
    );

    let mut bytes = build_schema2_fixture(1);
    let footer_block_count = schema2_footer_start(&bytes) + 16;
    put_u32_at(&mut bytes, footer_block_count, 2);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE012,
        "schema 2.x footer block count mismatch",
    );

    let mut bytes = build_schema2_fixture(1);
    let directory_pos = directory_start(&bytes);
    bytes[directory_pos + 4 + 10] = 1;
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE012,
        "directory crc mismatch",
    );

    let mut bytes = build_schema2_fixture(1);
    let table_pos = schema2_table_start(&bytes);
    bytes[table_pos + 33] ^= 0x01;
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE012,
        "payload block table crc mismatch",
    );

    let mut bytes = build_schema2_fixture(1);
    let file_crc_offset = schema2_footer_start(&bytes) + 8;
    bytes[file_crc_offset] ^= 0x01;
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE012, "file crc mismatch");
}

fn schema2_payload_block_bounds_are_typed() {
    let mut bytes = build_schema2_fixture(1);
    let table_pos = schema2_table_start(&bytes);
    put_u64_at(&mut bytes, table_pos + 16, u64::MAX);
    refresh_schema2_table_crc(&mut bytes);
    refresh_schema2_file_crc(&mut bytes);
    assert_layout_format_error(&bytes, HbpFormatErrorCode::HbpE013, "truncated payload");

    let mut bytes = build_schema2_fixture(1);
    let table_pos = schema2_table_start(&bytes);
    let invalid_block_offset = (bytes.len() - 10) as u64;
    put_u64_at(&mut bytes, table_pos + 16, invalid_block_offset);
    refresh_schema2_table_crc(&mut bytes);
    refresh_schema2_file_crc(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE013,
        "schema 2.x stored payload block exceeds file bounds",
    );
}

fn schema2_day_slice_bounds_are_typed() {
    let mut bytes = build_schema2_fixture(1);
    let directory_pos = directory_start(&bytes);
    put_u32_at(&mut bytes, directory_pos + 4 + 11, 1);
    refresh_schema2_crcs(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x directory block id is out of range",
    );

    let mut bytes = build_schema2_fixture(1);
    let directory_pos = directory_start(&bytes);
    put_u16_at(&mut bytes, directory_pos + 4 + 15, 1);
    refresh_schema2_crcs(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.0 day_in_block_index must equal julian_day - 1",
    );

    let mut bytes = build_schema2_fixture(1);
    let directory_pos = directory_start(&bytes);
    put_u32_at(&mut bytes, directory_pos + 4 + DIR_V2_ROW_SIZE + 17, 0);
    refresh_schema2_crcs(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x day slices overlap in raw block",
    );

    let mut bytes = build_schema2_fixture(1);
    let directory_pos = directory_start(&bytes);
    put_u16_at(&mut bytes, directory_pos + 4 + 15, 366);
    refresh_schema2_crcs(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x day_in_block_index is out of range",
    );

    let mut bytes = build_schema2_fixture(1);
    let directory_pos = directory_start(&bytes);
    put_u32_at(&mut bytes, directory_pos + 4 + 17, u32::MAX);
    refresh_schema2_crcs(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x day slice exceeds raw block bounds",
    );

    let mut bytes = build_schema2_fixture(1);
    let directory_pos = directory_start(&bytes);
    let second_row_offset = directory_pos + 4 + DIR_V2_ROW_SIZE;
    let original_offset = read_u32_at(&bytes, second_row_offset + 17);
    put_u32_at(&mut bytes, second_row_offset + 17, original_offset + 1);
    refresh_schema2_crcs(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x day slices must cover raw block without gaps",
    );

    let mut bytes = build_schema2_fixture(1);
    let directory_pos = directory_start(&bytes);
    let last_row_offset = directory_pos + 4 + 365 * DIR_V2_ROW_SIZE;
    let last_length = read_u32_at(&bytes, last_row_offset + 21);
    put_u32_at(&mut bytes, last_row_offset + 21, last_length - 1);
    refresh_schema2_crcs(&mut bytes);
    assert_layout_format_error(
        &bytes,
        HbpFormatErrorCode::HbpE011,
        "schema 2.x day slices must cover raw block without gaps",
    );
}

#[test]
fn strict_schema1_parse_succeeds_with_expected_hillslope_id() {
    let bytes = build_schema1_fixture(24);
    let parsed = parse_hbp_from_bytes(
        &bytes,
        Path::new("H24.hbp"),
        HbpParseOptions {
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
fn legacy_pass_dat_input_is_rejected() {
    let fixture = build_schema1_fixture(9);
    let temp_hbp_path = unique_temp_path("H9.hbp");
    write_fixture(&temp_hbp_path, &fixture);

    let legacy_path = temp_hbp_path
        .with_file_name("H9.pass.dat")
        .to_string_lossy()
        .to_string();

    let error = parse_hbp_from_path(
        &legacy_path,
        HbpParseOptions {
            expected_hillslope_id: None,
        },
    )
    .expect_err("legacy .pass.dat naming must be rejected");
    assert!(matches!(error, HbpParseError::InvalidProcessHbpName { .. }));
    assert_eq!(error.contract_error_id(), "HBP-E-001");
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
fn parser_rejects_legacy_path_while_bridge_compatibility_warning_remains() {
    let parser_fixture = build_schema1_fixture(21);
    let parser_path = unique_temp_path("H21.hbp");
    write_fixture(&parser_path, &parser_fixture);
    let parser_legacy_path = parser_path
        .with_file_name("H21.pass.dat")
        .to_string_lossy()
        .to_string();

    let parser_error = parse_hbp_from_path(
        &parser_legacy_path,
        HbpParseOptions {
            expected_hillslope_id: Some(21),
        },
    )
    .expect_err("legacy parser path must fail even when bridge compat is allowed");
    assert_eq!(parser_error.contract_error_id(), "HBP-E-001");

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
