#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::items_after_statements,
    clippy::match_same_arms,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::redundant_closure_for_method_calls,
    clippy::too_many_lines
)]

use std::fs;
use std::path::Path;

mod cursor;
mod error;
mod helpers;
mod internal_types;
mod layout_parser;
mod path;
mod payload_validator;
mod types;

use layout_parser::parse_layout;
use path::resolve_path;
use payload_validator::validate_payload;

use cursor::Cursor;
pub use error::{HbpFormatErrorCode, HbpParseError};
use helpers::{
    crc32c, decode_zlib_block, expected_dims, expected_state_schema, format_violation,
    key_in_year_table, map_cursor_err, scaled_i64_to_f64, u64_to_usize, validate_year_table,
};
use internal_types::{DirectoryEntry, EntryPayload, Layout, PayloadBlockEntry, YearEntry};
pub use types::{
    HbpDirectoryEntry, HbpLatestEventPayload, HbpLatestEventState, HbpNoEventKind,
    HbpNoEventPayload, HbpParseOptions, HbpParseResult, HbpPathResolution, HbpPayloadBlock,
    HbpPayloadLocator, HbpSchemaProfile, HbpWarning, HbpWarningCode, HbpYearEntry,
};

pub(super) const MAGIC: &[u8; 8] = b"WFPHBP01";
pub(super) const FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
pub(super) const SUPPORTED_MAJOR_V1: u16 = 1;
// SC-INFILE-HBP-001 v0.2.0 / ADR-0036: minor 1 adds the paired hourly
// EVENT surfaces (`hourly_runoff_volume_m3[24]` + `hourly_sediment_mass_kg[24]`).
pub(super) const SUPPORTED_MINOR_V1: u16 = 1;
pub(super) const SUPPORTED_MAJOR_V2: u16 = 2;
pub(super) const SUPPORTED_MINOR_V2: u16 = 1;
pub(super) const PAYLOAD_CODEC_ZLIB: u8 = 1;
pub(super) const DIM_SCALAR: u8 = 0;
pub(super) const DIM_NOFE: u8 = 1;
pub(super) const DIM_NOFE_LAYERS: u8 = 2;
pub(super) const SCALE_I64: f64 = 1e-9;

pub(super) const REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 300, 900, 901,
];

pub(super) fn parse_hbp_from_bytes_internal(
    bytes: &[u8],
    source_path: &Path,
    options: HbpParseOptions,
) -> Result<(HbpParseResult, Option<HbpLatestEventState>), HbpParseError> {
    let (resolved_path, path_resolution, warnings) = resolve_path(source_path)?;

    let layout = parse_layout(bytes)?;

    if let Some(expected_id) = options.expected_hillslope_id
        && layout.hillslope_id != expected_id
    {
        return Err(HbpParseError::HillslopeIdMismatch {
            expected: expected_id,
            found: layout.hillslope_id,
        });
    }

    let mut latest_event_state = None;
    for entry in &layout.entries {
        let payload_validation = validate_payload(bytes, &layout, entry)?;
        latest_event_state = Some(payload_validation.latest_event_state);
    }

    let year_entries = layout
        .years
        .iter()
        .map(|entry| HbpYearEntry {
            sim_year_index: entry.sim_year_index,
            calendar_year: entry.calendar_year,
            days_in_year: entry.days_in_year,
            first_julian_day: entry.first_julian_day,
            last_julian_day: entry.last_julian_day,
            single_storm_flag: entry.single_storm_flag,
        })
        .collect::<Vec<_>>();

    let directory_entries = layout
        .entries
        .iter()
        .map(|entry| {
            let payload = match entry.payload {
                EntryPayload::SchemaV1 {
                    payload_offset,
                    payload_length,
                    payload_crc32c,
                } => HbpPayloadLocator::Schema1 {
                    payload_offset_bytes: payload_offset as u64,
                    payload_length_bytes: payload_length as u32,
                    payload_crc32c,
                },
                EntryPayload::SchemaV2 {
                    payload_block_id,
                    day_in_block_index,
                    raw_payload_offset,
                    raw_payload_length,
                    raw_payload_crc32c,
                } => HbpPayloadLocator::Schema2 {
                    payload_block_id: payload_block_id as u32,
                    day_in_block_index,
                    raw_payload_offset_in_block_bytes: raw_payload_offset as u32,
                    raw_payload_length_bytes: raw_payload_length as u32,
                    raw_payload_crc32c,
                },
            };

            HbpDirectoryEntry {
                sim_year_index: entry.sim_year_index,
                calendar_year: entry.calendar_year,
                julian_day: entry.julian_day,
                event_kind: entry.event_kind,
                payload,
            }
        })
        .collect::<Vec<_>>();

    let payload_blocks = layout
        .payload_blocks
        .iter()
        .map(|entry| HbpPayloadBlock {
            payload_block_id: entry.payload_block_id,
            sim_year_index: entry.sim_year_index,
            block_day_slot_count: entry.block_day_slot_count,
            represented_day_count: entry.represented_day_count,
            stored_block_offset_bytes: entry.stored_block_offset as u64,
            stored_block_length_bytes: entry.stored_block_length as u32,
            raw_block_length_bytes: entry.raw_block_length as u32,
            payload_codec: entry.payload_codec,
            stored_block_crc32c: entry.stored_block_crc32c,
            raw_block_crc32c: entry.raw_block_crc32c,
        })
        .collect::<Vec<_>>();

    let schema_profile = if layout.schema_major == SUPPORTED_MAJOR_V2 {
        HbpSchemaProfile::Schema2x
    } else {
        HbpSchemaProfile::Schema1x
    };

    Ok((
        HbpParseResult {
            resolved_path,
            path_resolution,
            schema_profile,
            schema_major: layout.schema_major,
            schema_minor: layout.schema_minor,
            hillslope_id: layout.hillslope_id,
            nyear: layout.nyear,
            begin_year: layout.begin_year,
            npart: layout.npart,
            particle_diameter_m: layout.particle_diameter_m.clone(),
            nofe: layout.nofe,
            max_layers: layout.max_layers,
            simulation_mode: layout.simulation_mode,
            record_count: layout.entries.len() as u32,
            block_count: layout.payload_blocks.len() as u32,
            year_entries,
            directory_entries,
            payload_blocks,
            warnings,
        },
        latest_event_state,
    ))
}

pub fn parse_hbp_from_bytes(
    bytes: &[u8],
    source_path: &Path,
    options: HbpParseOptions,
) -> Result<HbpParseResult, HbpParseError> {
    let (parsed, _) = parse_hbp_from_bytes_internal(bytes, source_path, options)?;
    Ok(parsed)
}

pub fn parse_hbp_from_bytes_with_latest_event_payload(
    bytes: &[u8],
    source_path: &Path,
    options: HbpParseOptions,
) -> Result<(HbpParseResult, Option<HbpLatestEventPayload>), HbpParseError> {
    let (parsed, latest_event_state) =
        parse_hbp_from_bytes_with_latest_event_state(bytes, source_path, options)?;
    Ok((parsed, latest_payload_from_state(latest_event_state)))
}

fn latest_payload_from_state(
    latest_event_state: Option<HbpLatestEventState>,
) -> Option<HbpLatestEventPayload> {
    match latest_event_state {
        Some(HbpLatestEventState::EventPayload(payload)) => Some(payload),
        Some(HbpLatestEventState::NoEvent(_)) | None => None,
    }
}

pub fn parse_hbp_from_bytes_with_latest_event_state(
    bytes: &[u8],
    source_path: &Path,
    options: HbpParseOptions,
) -> Result<(HbpParseResult, Option<HbpLatestEventState>), HbpParseError> {
    parse_hbp_from_bytes_internal(bytes, source_path, options)
}

pub fn parse_hbp_from_path(
    path: impl AsRef<Path>,
    options: HbpParseOptions,
) -> Result<HbpParseResult, HbpParseError> {
    let (resolved_path, _path_resolution, _warnings) = resolve_path(path.as_ref())?;
    let bytes = fs::read(&resolved_path).map_err(|source| HbpParseError::InputOpenError {
        path: resolved_path.clone(),
        source,
    })?;
    parse_hbp_from_bytes(&bytes, path.as_ref(), options)
}

pub fn parse_hbp_from_path_with_latest_event_payload(
    path: impl AsRef<Path>,
    options: HbpParseOptions,
) -> Result<(HbpParseResult, Option<HbpLatestEventPayload>), HbpParseError> {
    let (parsed, latest_event_state) = parse_hbp_from_path_with_latest_event_state(path, options)?;
    Ok((parsed, latest_payload_from_state(latest_event_state)))
}

pub fn parse_hbp_from_path_with_latest_event_state(
    path: impl AsRef<Path>,
    options: HbpParseOptions,
) -> Result<(HbpParseResult, Option<HbpLatestEventState>), HbpParseError> {
    let (resolved_path, _path_resolution, _warnings) = resolve_path(path.as_ref())?;
    let bytes = fs::read(&resolved_path).map_err(|source| HbpParseError::InputOpenError {
        path: resolved_path.clone(),
        source,
    })?;
    parse_hbp_from_bytes_with_latest_event_state(&bytes, path.as_ref(), options)
}
