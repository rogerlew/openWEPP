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

use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use flate2::read::ZlibDecoder;

const MAGIC: &[u8; 8] = b"WFPHBP01";
const FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
const SUPPORTED_MAJOR_V1: u16 = 1;
const SUPPORTED_MINOR_V1: u16 = 0;
const SUPPORTED_MAJOR_V2: u16 = 2;
const SUPPORTED_MINOR_V2: u16 = 0;
const PAYLOAD_CODEC_ZLIB: u8 = 1;
const DIM_SCALAR: u8 = 0;
const DIM_NOFE: u8 = 1;
const DIM_NOFE_LAYERS: u8 = 2;
const SCALE_I64: f64 = 1e-9;

const REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 300, 900, 901,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbpPathResolution {
    Direct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbpSchemaProfile {
    Schema1x,
    Schema2x,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HbpParseOptions {
    pub expected_hillslope_id: Option<u32>,
}

impl HbpParseOptions {
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            expected_hillslope_id: None,
        }
    }
}

impl Default for HbpParseOptions {
    fn default() -> Self {
        Self::strict()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbpWarningCode {
    HbpW001,
}

impl HbpWarningCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HbpW001 => "HBP-W-001",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbpWarning {
    pub code: HbpWarningCode,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbpYearEntry {
    pub sim_year_index: u32,
    pub calendar_year: i32,
    pub days_in_year: u16,
    pub first_julian_day: u16,
    pub last_julian_day: u16,
    pub single_storm_flag: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HbpPayloadLocator {
    Schema1 {
        payload_offset_bytes: u64,
        payload_length_bytes: u32,
        payload_crc32c: u32,
    },
    Schema2 {
        payload_block_id: u32,
        day_in_block_index: u16,
        raw_payload_offset_in_block_bytes: u32,
        raw_payload_length_bytes: u32,
        raw_payload_crc32c: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbpDirectoryEntry {
    pub sim_year_index: u32,
    pub calendar_year: i32,
    pub julian_day: u16,
    pub event_kind: u8,
    pub payload: HbpPayloadLocator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbpPayloadBlock {
    pub payload_block_id: u32,
    pub sim_year_index: u32,
    pub block_day_slot_count: u16,
    pub represented_day_count: u16,
    pub stored_block_offset_bytes: u64,
    pub stored_block_length_bytes: u32,
    pub raw_block_length_bytes: u32,
    pub payload_codec: u8,
    pub stored_block_crc32c: u32,
    pub raw_block_crc32c: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HbpLatestEventPayload {
    pub sim_year_index: u32,
    pub calendar_year: i32,
    pub julian_day: u16,
    pub duration_seconds: f64,
    pub peak_runoff_m3_s: f64,
    pub total_detachment_kg: f64,
    pub total_deposition_kg: f64,
    pub particle_diameter_m: Vec<f64>,
    pub sediment_concentration_kg_m3: Vec<f64>,
    pub particle_flow_fraction: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HbpParseResult {
    pub resolved_path: PathBuf,
    pub path_resolution: HbpPathResolution,
    pub schema_profile: HbpSchemaProfile,
    pub schema_major: u16,
    pub schema_minor: u16,
    pub hillslope_id: u32,
    pub nyear: u32,
    pub begin_year: i32,
    pub npart: u16,
    pub particle_diameter_m: Vec<f64>,
    pub nofe: u16,
    pub max_layers: u16,
    pub simulation_mode: u8,
    pub record_count: u32,
    pub block_count: u32,
    pub year_entries: Vec<HbpYearEntry>,
    pub directory_entries: Vec<HbpDirectoryEntry>,
    pub payload_blocks: Vec<HbpPayloadBlock>,
    pub warnings: Vec<HbpWarning>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbpFormatErrorCode {
    HbpE002,
    HbpE003,
    HbpE004,
    HbpE005,
    HbpE006,
    HbpE007,
    HbpE008,
    HbpE009,
    HbpE010,
    HbpE011,
    HbpE012,
    HbpE013,
    HbpE014,
}

impl HbpFormatErrorCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HbpE002 => "HBP-E-002",
            Self::HbpE003 => "HBP-E-003",
            Self::HbpE004 => "HBP-E-004",
            Self::HbpE005 => "HBP-E-005",
            Self::HbpE006 => "HBP-E-006",
            Self::HbpE007 => "HBP-E-007",
            Self::HbpE008 => "HBP-E-008",
            Self::HbpE009 => "HBP-E-009",
            Self::HbpE010 => "HBP-E-010",
            Self::HbpE011 => "HBP-E-011",
            Self::HbpE012 => "HBP-E-012",
            Self::HbpE013 => "HBP-E-013",
            Self::HbpE014 => "HBP-E-014",
        }
    }
}

#[derive(Debug)]
pub enum HbpParseError {
    InputOpenError {
        path: PathBuf,
        source: io::Error,
    },
    InvalidProcessHbpName {
        input_path: PathBuf,
        reason: String,
    },
    HillslopeIdMismatch {
        expected: u32,
        found: u32,
    },
    FormatViolation {
        code: HbpFormatErrorCode,
        detail: String,
    },
}

impl HbpParseError {
    #[must_use]
    pub const fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "HBP-E-000",
            Self::InvalidProcessHbpName { .. } => "HBP-E-001",
            Self::HillslopeIdMismatch { .. } => "HBP-E-014",
            Self::FormatViolation { code, .. } => code.as_str(),
        }
    }
}

impl fmt::Display for HbpParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{}: failed to open/read HBP shard '{}': {}",
                self.contract_error_id(),
                path.display(),
                source
            ),
            Self::InvalidProcessHbpName { input_path, reason } => write!(
                f,
                "{}: invalid process HBP name '{}': {}",
                self.contract_error_id(),
                input_path.display(),
                reason
            ),
            Self::HillslopeIdMismatch { expected, found } => write!(
                f,
                "{}: hillslope id mismatch (expected {}, found {})",
                self.contract_error_id(),
                expected,
                found
            ),
            Self::FormatViolation { code, detail } => {
                write!(f, "{}: {}", code.as_str(), detail)
            }
        }
    }
}

impl std::error::Error for HbpParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            Self::InvalidProcessHbpName { .. }
            | Self::HillslopeIdMismatch { .. }
            | Self::FormatViolation { .. } => None,
        }
    }
}

#[derive(Clone, Copy)]
struct YearEntry {
    sim_year_index: u32,
    calendar_year: i32,
    days_in_year: u16,
    first_julian_day: u16,
    last_julian_day: u16,
    single_storm_flag: u8,
}

#[derive(Clone, Copy)]
struct DirectoryEntry {
    sim_year_index: u32,
    calendar_year: i32,
    julian_day: u16,
    event_kind: u8,
    payload: EntryPayload,
}

#[derive(Clone, Copy)]
enum EntryPayload {
    SchemaV1 {
        payload_offset: usize,
        payload_length: usize,
        payload_crc32c: u32,
    },
    SchemaV2 {
        payload_block_id: usize,
        day_in_block_index: u16,
        raw_payload_offset: usize,
        raw_payload_length: usize,
        raw_payload_crc32c: u32,
    },
}

#[derive(Clone, Copy)]
struct PayloadBlockEntry {
    payload_block_id: u32,
    sim_year_index: u32,
    block_day_slot_count: u16,
    represented_day_count: u16,
    stored_block_offset: usize,
    stored_block_length: usize,
    raw_block_length: usize,
    payload_codec: u8,
    stored_block_crc32c: u32,
    raw_block_crc32c: u32,
}

struct Layout {
    schema_major: u16,
    schema_minor: u16,
    hillslope_id: u32,
    nyear: u32,
    begin_year: i32,
    npart: u16,
    particle_diameter_m: Vec<f64>,
    nofe: u16,
    max_layers: u16,
    simulation_mode: u8,
    years: Vec<YearEntry>,
    entries: Vec<DirectoryEntry>,
    payload_blocks: Vec<PayloadBlockEntry>,
    raw_payload_blocks: Vec<Vec<u8>>,
}

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8], pos: usize) -> Self {
        Self { data, pos }
    }

    fn require(&self, count: usize) -> Result<(), &'static str> {
        if self.pos + count > self.data.len() {
            return Err("truncated payload");
        }
        Ok(())
    }

    fn u8(&mut self) -> Result<u8, &'static str> {
        self.require(1)?;
        let value = self.data[self.pos];
        self.pos += 1;
        Ok(value)
    }

    fn u16(&mut self) -> Result<u16, &'static str> {
        self.require(2)?;
        let value = u16::from_le_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(value)
    }

    fn u32(&mut self) -> Result<u32, &'static str> {
        self.require(4)?;
        let value = u32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    fn i32(&mut self) -> Result<i32, &'static str> {
        self.require(4)?;
        let value = i32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    fn u64(&mut self) -> Result<u64, &'static str> {
        self.require(8)?;
        let value = u64::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(value)
    }

    fn i64(&mut self) -> Result<i64, &'static str> {
        self.require(8)?;
        let value = i64::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(value)
    }

    fn f64(&mut self) -> Result<f64, &'static str> {
        self.require(8)?;
        let value = f64::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(value)
    }

    fn raw(&mut self, count: usize) -> Result<&'a [u8], &'static str> {
        self.require(count)?;
        let start = self.pos;
        let end = start + count;
        self.pos = end;
        Ok(&self.data[start..end])
    }

    fn string(&mut self) -> Result<String, &'static str> {
        let len = self.u32()? as usize;
        let raw = self.raw(len)?;
        std::str::from_utf8(raw)
            .map(|value| value.to_string())
            .map_err(|_| "invalid utf8 string")
    }
}

fn format_violation(code: HbpFormatErrorCode, detail: impl Into<String>) -> HbpParseError {
    HbpParseError::FormatViolation {
        code,
        detail: detail.into(),
    }
}

fn map_cursor_err(
    code: HbpFormatErrorCode,
    context: &'static str,
    msg: &'static str,
) -> HbpParseError {
    format_violation(code, format!("{context}: {msg}"))
}

fn crc32c(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for value in data {
        crc ^= *value as u32;
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ 0x82F6_3B78;
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

fn expected_dims(dims_kind: u8, layout: &Layout) -> Result<Vec<u32>, HbpParseError> {
    match dims_kind {
        DIM_SCALAR => Ok(vec![]),
        DIM_NOFE => Ok(vec![u32::from(layout.nofe)]),
        DIM_NOFE_LAYERS => Ok(vec![u32::from(layout.nofe), u32::from(layout.max_layers)]),
        _ => Err(format_violation(
            HbpFormatErrorCode::HbpE009,
            "unknown registry dimension kind",
        )),
    }
}

fn decode_zlib_block(source: &[u8], expected_raw_length: usize) -> Result<Vec<u8>, HbpParseError> {
    let mut decoder = ZlibDecoder::new(source);
    let mut raw = Vec::new();
    use std::io::Read;
    decoder.read_to_end(&mut raw).map_err(|_| {
        format_violation(HbpFormatErrorCode::HbpE011, "schema 2.x zlib decode failed")
    })?;
    if raw.len() != expected_raw_length {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE011,
            "schema 2.x zlib decoded length mismatch",
        ));
    }
    Ok(raw)
}

fn u64_to_usize(value: u64, field_name: &str) -> Result<usize, HbpParseError> {
    usize::try_from(value).map_err(|_| {
        format_violation(
            HbpFormatErrorCode::HbpE013,
            format!("{field_name} exceeds platform limits"),
        )
    })
}

fn scaled_i64_to_f64(value: i64) -> Result<f64, HbpParseError> {
    value.to_string().parse::<f64>().map_err(|_| {
        format_violation(
            HbpFormatErrorCode::HbpE013,
            "failed converting scaled i64 payload value to f64",
        )
    })
}

fn key_in_year_table(entry: &DirectoryEntry, years: &[YearEntry]) -> bool {
    years.iter().any(|year| {
        entry.sim_year_index == year.sim_year_index
            && entry.calendar_year == year.calendar_year
            && entry.julian_day >= year.first_julian_day
            && entry.julian_day <= year.last_julian_day
    })
}

fn validate_year_table(
    years: &[YearEntry],
    nyear: u32,
    schema_major: u16,
    simulation_mode: u8,
) -> Result<u32, HbpParseError> {
    if years.len() != nyear as usize {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE008,
            "year table count mismatch",
        ));
    }

    if schema_major == SUPPORTED_MAJOR_V2 && simulation_mode != 1 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE008,
            "schema 2.0 requires simulation_mode = 1",
        ));
    }

    let mut expected_record_count = 0u32;
    for (index, year) in years.iter().enumerate() {
        if year.sim_year_index != (index + 1) as u32 {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE008,
                "year table sim_year_index must be one-based and ordered",
            ));
        }
        if year.days_in_year < 1 {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE008,
                "year table days_in_year must be positive",
            ));
        }
        if year.first_julian_day < 1 || year.last_julian_day < year.first_julian_day {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE008,
                "year table julian-day range is invalid",
            ));
        }
        if year.days_in_year != (year.last_julian_day - year.first_julian_day + 1) {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE008,
                "year table days_in_year must match julian-day range",
            ));
        }
        if schema_major == SUPPORTED_MAJOR_V2 {
            if year.days_in_year != 366 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE008,
                    "schema 2.0 year table days_in_year must be 366",
                ));
            }
            if year.first_julian_day != 1 || year.last_julian_day != 366 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE008,
                    "schema 2.0 year table range must be 1..366",
                ));
            }
            if year.single_storm_flag != 0 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE008,
                    "schema 2.0 single_storm_flag must be 0",
                ));
            }
        }
        expected_record_count += u32::from(year.days_in_year);
    }

    Ok(expected_record_count)
}

fn parse_layout(data: &[u8]) -> Result<Layout, HbpParseError> {
    let mut cursor = Cursor::new(data, 0);

    let magic = cursor
        .raw(8)
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;
    if magic != MAGIC {
        return Err(format_violation(HbpFormatErrorCode::HbpE002, "bad magic"));
    }

    let schema_major = cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;
    let schema_minor = cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;

    match schema_major {
        SUPPORTED_MAJOR_V1 => {
            if schema_minor > SUPPORTED_MINOR_V1 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE004,
                    "unsupported schema minor",
                ));
            }
        }
        SUPPORTED_MAJOR_V2 => {
            if schema_minor > SUPPORTED_MINOR_V2 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE004,
                    "unsupported schema minor",
                ));
            }
        }
        _ => {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE003,
                "unsupported schema major",
            ));
        }
    }

    let endianness = cursor
        .u8()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;
    if endianness != 1 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE005,
            "unsupported endianness",
        ));
    }

    let header_bytes = cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?
        as usize;
    if header_bytes > data.len() {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE006,
            "header length exceeds file length",
        ));
    }

    let _compatibility_id = cursor
        .raw(32)
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;
    let artifact_role = cursor
        .u8()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;
    if artifact_role != 1 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE006,
            "unsupported artifact role",
        ));
    }
    let _producer = cursor
        .string()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;
    let _run_id = cursor
        .string()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;
    let _created_utc = cursor
        .string()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;
    let _unit_policy_id = cursor
        .string()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;
    let _state_registry_id = cursor
        .raw(32)
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;

    let header_crc_pos = cursor.pos;
    let header_crc = cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "header", msg))?;

    if cursor.pos != header_bytes {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE006,
            "header length mismatch",
        ));
    }

    let mut header_region = data[..header_bytes].to_vec();
    header_region[header_crc_pos..header_crc_pos + 4].fill(0);
    if crc32c(&header_region) != header_crc {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE007,
            "header crc mismatch",
        ));
    }

    let hillslope_id = cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "dimension", msg))?;
    let nyear = cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "dimension", msg))?;
    let begin_year = cursor
        .i32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "dimension", msg))?;
    let npart = cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "dimension", msg))?;
    let nofe = cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "dimension", msg))?;
    let max_layers = cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "dimension", msg))?;
    let _calendar_policy_id = cursor
        .string()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "dimension", msg))?;
    let _event_enum_version = cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "dimension", msg))?;
    let simulation_mode = cursor
        .u8()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "dimension", msg))?;

    let _climate_file_name = cursor
        .string()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "metadata", msg))?;
    let _hillslope_area_i64 = cursor
        .i64()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "metadata", msg))?;

    let particle_count = cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "metadata", msg))?
        as usize;
    let mut particle_diameter_m = Vec::with_capacity(particle_count);
    for _ in 0..particle_count {
        let diameter_m = cursor
            .f64()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "metadata", msg))?;
        if !diameter_m.is_finite() || diameter_m <= 0.0 {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE006,
                "particle_diameter_m must be finite and > 0",
            ));
        }
        particle_diameter_m.push(diameter_m);
    }
    if particle_count != usize::from(npart) {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE006,
            "event sediment count mismatch",
        ));
    }

    let _srp = cursor
        .f64()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "metadata", msg))?;
    let _slfp = cursor
        .f64()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "metadata", msg))?;
    let _bfp = cursor
        .f64()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "metadata", msg))?;
    let _scp = cursor
        .f64()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "metadata", msg))?;

    let year_count = cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "year table", msg))?;
    let mut years = Vec::with_capacity(year_count as usize);
    for _ in 0..year_count {
        years.push(YearEntry {
            sim_year_index: cursor
                .u32()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "year table", msg))?,
            calendar_year: cursor
                .i32()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "year table", msg))?,
            days_in_year: cursor
                .u16()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "year table", msg))?,
            first_julian_day: cursor
                .u16()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "year table", msg))?,
            last_julian_day: cursor
                .u16()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "year table", msg))?,
            single_storm_flag: cursor
                .u8()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "year table", msg))?,
        });
    }

    let expected_record_count = validate_year_table(&years, nyear, schema_major, simulation_mode)?;

    let registry_count = cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state registry", msg))?;
    let mut registry_state_ids = Vec::with_capacity(registry_count as usize);
    let mut registry_seen: HashSet<u16> = HashSet::new();

    for _ in 0..registry_count {
        let state_id = cursor
            .u16()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state registry", msg))?;
        let required_flag = cursor
            .u8()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state registry", msg))?;
        let representation_class = cursor
            .u8()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state registry", msg))?;
        let unit_class = cursor
            .u16()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state registry", msg))?;
        let rank = cursor
            .u8()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state registry", msg))?;
        let dims_kind = cursor
            .u8()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state registry", msg))?;
        let _name = cursor
            .string()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state registry", msg))?;

        if !registry_seen.insert(state_id) {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE009,
                "duplicate registry state id",
            ));
        }

        if let Some(expected) = expected_state_schema(state_id)
            && expected
                != (
                    required_flag,
                    representation_class,
                    unit_class,
                    rank,
                    dims_kind,
                )
        {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE009,
                "state registry block does not match canonical schema",
            ));
        }

        registry_state_ids.push(state_id);
    }

    let registry_set: HashSet<u16> = registry_state_ids.into_iter().collect();
    if let Some(missing) = REQUIRED_STATE_IDS
        .iter()
        .find(|state_id| !registry_set.contains(state_id))
    {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE009,
            format!("required state id missing in registry: {missing}"),
        ));
    }

    let directory_start = cursor.pos;
    let record_count = cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg))?
        as usize;

    let mut entries = Vec::with_capacity(record_count);
    for _ in 0..record_count {
        let sim_year_index = cursor
            .u32()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg))?;
        let calendar_year = cursor
            .i32()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg))?;
        let julian_day = cursor
            .u16()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg))?;
        let event_kind = cursor
            .u8()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg))?;

        let payload = match schema_major {
            SUPPORTED_MAJOR_V1 => {
                let payload_offset = u64_to_usize(
                    cursor.u64().map_err(|msg| {
                        map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg)
                    })?,
                    "payload_offset_bytes",
                )?;
                let payload_length = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg)
                })? as usize;
                let payload_crc32c = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg)
                })?;
                if payload_length < 1 {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE010,
                        "payload length must be positive",
                    ));
                }
                EntryPayload::SchemaV1 {
                    payload_offset,
                    payload_length,
                    payload_crc32c,
                }
            }
            SUPPORTED_MAJOR_V2 => {
                let payload_block_id = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg)
                })? as usize;
                let day_in_block_index = cursor.u16().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg)
                })?;
                let raw_payload_offset = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg)
                })? as usize;
                let raw_payload_length = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg)
                })? as usize;
                let raw_payload_crc32c = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "day directory", msg)
                })?;
                EntryPayload::SchemaV2 {
                    payload_block_id,
                    day_in_block_index,
                    raw_payload_offset,
                    raw_payload_length,
                    raw_payload_crc32c,
                }
            }
            _ => {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE003,
                    "unsupported schema major",
                ));
            }
        };

        entries.push(DirectoryEntry {
            sim_year_index,
            calendar_year,
            julian_day,
            event_kind,
            payload,
        });
    }

    let directory_end = cursor.pos;

    if entries.is_empty() {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE010,
            "empty day directory",
        ));
    }
    if record_count != expected_record_count as usize {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE010,
            "directory record count must equal sum of year-table days",
        ));
    }

    let mut previous_key: Option<(u32, u16)> = None;
    for entry in &entries {
        let key = (entry.sim_year_index, entry.julian_day);
        if !key_in_year_table(entry, &years) {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE010,
                "directory key is outside the year table",
            ));
        }
        if let Some(previous) = previous_key
            && key <= previous
        {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE010,
                "directory keys must be deterministic and strictly ordered",
            ));
        }
        previous_key = Some(key);
    }

    match schema_major {
        SUPPORTED_MAJOR_V1 => {
            let mut expected_payload_offset = directory_end;
            for entry in &entries {
                let EntryPayload::SchemaV1 {
                    payload_offset,
                    payload_length,
                    ..
                } = entry.payload
                else {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE003,
                        "unsupported schema major",
                    ));
                };
                if payload_offset != expected_payload_offset {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE010,
                        "payload offsets are not deterministic",
                    ));
                }
                expected_payload_offset = expected_payload_offset
                    .checked_add(payload_length)
                    .ok_or_else(|| {
                        format_violation(HbpFormatErrorCode::HbpE013, "truncated payload")
                    })?;
            }

            let footer_start = expected_payload_offset;
            let footer_end = footer_start + 20;
            if footer_end > data.len() {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE013,
                    "truncated payload",
                ));
            }

            let mut footer_cursor = Cursor::new(data, footer_start);
            let directory_crc = footer_cursor
                .u32()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "footer", msg))?;
            let file_crc_pos = footer_cursor.pos;
            let file_crc = footer_cursor
                .u32()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "footer", msg))?;
            let footer_record_count = footer_cursor
                .u32()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "footer", msg))?;
            let footer_magic = footer_cursor
                .raw(8)
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "footer", msg))?;

            if crc32c(&data[directory_start..directory_end]) != directory_crc {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "directory crc mismatch",
                ));
            }

            let mut file_region = data.to_vec();
            file_region[file_crc_pos..file_crc_pos + 4].fill(0);
            if crc32c(&file_region) != file_crc {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "file crc mismatch",
                ));
            }

            if footer_record_count != expected_record_count {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "footer record count must equal sum of year-table days",
                ));
            }

            if footer_magic != FOOTER_MAGIC {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "bad footer magic",
                ));
            }

            Ok(Layout {
                schema_major,
                schema_minor,
                hillslope_id,
                nyear,
                begin_year,
                npart,
                particle_diameter_m,
                nofe,
                max_layers,
                simulation_mode,
                years,
                entries,
                payload_blocks: Vec::new(),
                raw_payload_blocks: Vec::new(),
            })
        }
        SUPPORTED_MAJOR_V2 => {
            let payload_block_table_start = cursor.pos;
            let payload_block_count = cursor.u32().map_err(|msg| {
                map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
            })? as usize;
            if payload_block_count != nyear as usize {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE011,
                    "schema 2.x block count must equal year table count",
                ));
            }

            let mut payload_blocks = Vec::with_capacity(payload_block_count);
            for block_index in 0..payload_block_count {
                let payload_block_id = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
                })?;
                if payload_block_id != block_index as u32 {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.x payload_block_id must be contiguous and ordered",
                    ));
                }
                let block_sim_year_index = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
                })?;
                if block_sim_year_index != (block_index + 1) as u32 {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.x payload block sim_year_index mismatch",
                    ));
                }
                let block_day_slot_count = cursor.u16().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
                })?;
                let represented_day_count = cursor.u16().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
                })?;
                if block_day_slot_count != 366 || represented_day_count != 366 {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.0 payload block day counts must be 366",
                    ));
                }
                let stored_block_offset = u64_to_usize(
                    cursor.u64().map_err(|msg| {
                        map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
                    })?,
                    "stored_block_offset_bytes",
                )?;
                let stored_block_length = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
                })? as usize;
                let raw_block_length = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
                })? as usize;
                let payload_codec = cursor.u8().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
                })?;
                if payload_codec != PAYLOAD_CODEC_ZLIB {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.x payload codec is unsupported",
                    ));
                }
                let stored_block_crc32c = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
                })?;
                let raw_block_crc32c = cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "payload block table", msg)
                })?;
                if stored_block_length < 1 || raw_block_length < 1 {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.x payload block lengths must be positive",
                    ));
                }

                payload_blocks.push(PayloadBlockEntry {
                    payload_block_id,
                    sim_year_index: block_sim_year_index,
                    block_day_slot_count,
                    represented_day_count,
                    stored_block_offset,
                    stored_block_length,
                    raw_block_length,
                    payload_codec,
                    stored_block_crc32c,
                    raw_block_crc32c,
                });
            }

            let payload_block_table_end = cursor.pos;
            if data.len() < 28 || payload_block_table_end > data.len() - 28 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE013,
                    "truncated payload",
                ));
            }

            let footer_start = data.len() - 28;
            let mut footer_cursor = Cursor::new(data, footer_start);
            let directory_crc = footer_cursor
                .u32()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "footer", msg))?;
            let payload_block_table_crc = footer_cursor
                .u32()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "footer", msg))?;
            let file_crc_pos = footer_cursor.pos;
            let file_crc = footer_cursor
                .u32()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "footer", msg))?;
            let footer_record_count = footer_cursor
                .u32()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "footer", msg))?;
            let footer_block_count = footer_cursor
                .u32()
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "footer", msg))?;
            let footer_magic = footer_cursor
                .raw(8)
                .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "footer", msg))?;

            if footer_magic != FOOTER_MAGIC {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "bad footer magic",
                ));
            }
            if footer_record_count != expected_record_count {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "footer record count must equal sum of year-table days",
                ));
            }
            if footer_record_count != 366 * nyear {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "schema 2.0 record count must equal 366 * nyear",
                ));
            }
            if footer_block_count != payload_block_count as u32 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "schema 2.x footer block count mismatch",
                ));
            }

            if crc32c(&data[directory_start..directory_end]) != directory_crc {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "directory crc mismatch",
                ));
            }

            if crc32c(&data[payload_block_table_start..payload_block_table_end])
                != payload_block_table_crc
            {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "payload block table crc mismatch",
                ));
            }

            let mut file_region = data.to_vec();
            file_region[file_crc_pos..file_crc_pos + 4].fill(0);
            if crc32c(&file_region) != file_crc {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "file crc mismatch",
                ));
            }

            let mut raw_payload_blocks = Vec::with_capacity(payload_blocks.len());
            for block in &payload_blocks {
                let stored_end = block
                    .stored_block_offset
                    .checked_add(block.stored_block_length)
                    .ok_or_else(|| {
                        format_violation(HbpFormatErrorCode::HbpE013, "truncated payload")
                    })?;
                if stored_end > data.len() {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE013,
                        "schema 2.x stored payload block exceeds file bounds",
                    ));
                }
                let stored = &data[block.stored_block_offset..stored_end];
                if crc32c(stored) != block.stored_block_crc32c {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE012,
                        "schema 2.x stored block crc mismatch",
                    ));
                }
                let raw = decode_zlib_block(stored, block.raw_block_length)?;
                if crc32c(&raw) != block.raw_block_crc32c {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE012,
                        "schema 2.x raw block crc mismatch",
                    ));
                }
                raw_payload_blocks.push(raw);
            }

            let mut per_block_ranges: Vec<Vec<(u16, usize, usize)>> =
                vec![Vec::new(); payload_blocks.len()];

            for entry in &entries {
                let EntryPayload::SchemaV2 {
                    payload_block_id,
                    day_in_block_index,
                    raw_payload_offset,
                    raw_payload_length,
                    ..
                } = entry.payload
                else {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE003,
                        "unsupported schema major",
                    ));
                };

                if payload_block_id >= payload_blocks.len() {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.x directory block id is out of range",
                    ));
                }

                let block = payload_blocks[payload_block_id];
                if block.sim_year_index != entry.sim_year_index {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.x block sim_year_index must match directory key",
                    ));
                }

                if day_in_block_index >= block.block_day_slot_count {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.x day_in_block_index is out of range",
                    ));
                }

                if day_in_block_index + 1 != entry.julian_day {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.0 day_in_block_index must equal julian_day - 1",
                    ));
                }

                let raw_end = raw_payload_offset
                    .checked_add(raw_payload_length)
                    .ok_or_else(|| {
                        format_violation(
                            HbpFormatErrorCode::HbpE011,
                            "schema 2.x day slice exceeds raw block bounds",
                        )
                    })?;
                if raw_end > raw_payload_blocks[payload_block_id].len() {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.x day slice exceeds raw block bounds",
                    ));
                }

                per_block_ranges[payload_block_id].push((
                    day_in_block_index,
                    raw_payload_offset,
                    raw_end,
                ));
            }

            for (block_id, ranges) in per_block_ranges.iter_mut().enumerate() {
                let block = payload_blocks[block_id];
                if ranges.len() != usize::from(block.block_day_slot_count) {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.0 payload block must represent 366 day slots",
                    ));
                }

                ranges.sort_by_key(|row| row.0);

                for (expected, (day_slot, _, _)) in ranges.iter().enumerate() {
                    if usize::from(*day_slot) != expected {
                        return Err(format_violation(
                            HbpFormatErrorCode::HbpE011,
                            "schema 2.x day slots must terminate at index 365",
                        ));
                    }
                }

                let mut previous_end = 0usize;
                for (_, start, end) in ranges.iter() {
                    if *start < previous_end {
                        return Err(format_violation(
                            HbpFormatErrorCode::HbpE011,
                            "schema 2.x day slices overlap in raw block",
                        ));
                    }
                    if *start > previous_end {
                        return Err(format_violation(
                            HbpFormatErrorCode::HbpE011,
                            "schema 2.x day slices must cover raw block without gaps",
                        ));
                    }
                    previous_end = *end;
                }

                if previous_end != raw_payload_blocks[block_id].len() {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.x day slices must cover raw block without gaps",
                    ));
                }
            }

            Ok(Layout {
                schema_major,
                schema_minor,
                hillslope_id,
                nyear,
                begin_year,
                npart,
                particle_diameter_m,
                nofe,
                max_layers,
                simulation_mode,
                years,
                entries,
                payload_blocks,
                raw_payload_blocks,
            })
        }
        _ => Err(format_violation(
            HbpFormatErrorCode::HbpE003,
            "unsupported schema major",
        )),
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
struct PayloadValidationResult {
    latest_event_payload: Option<HbpLatestEventPayload>,
}

fn validate_payload(
    data: &[u8],
    layout: &Layout,
    entry: &DirectoryEntry,
) -> Result<PayloadValidationResult, HbpParseError> {
    let payload = match entry.payload {
        EntryPayload::SchemaV1 {
            payload_offset,
            payload_length,
            payload_crc32c,
        } => {
            let payload_end = payload_offset.checked_add(payload_length).ok_or_else(|| {
                format_violation(HbpFormatErrorCode::HbpE013, "truncated payload")
            })?;
            if payload_end > data.len() {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE013,
                    "truncated payload",
                ));
            }
            let payload = &data[payload_offset..payload_end];
            if crc32c(payload) != payload_crc32c {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "payload crc mismatch",
                ));
            }
            payload.to_vec()
        }
        EntryPayload::SchemaV2 {
            payload_block_id,
            raw_payload_offset,
            raw_payload_length,
            raw_payload_crc32c,
            ..
        } => {
            if payload_block_id >= layout.raw_payload_blocks.len() {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE011,
                    "schema 2.x directory block id is out of range",
                ));
            }
            let raw_block = &layout.raw_payload_blocks[payload_block_id];
            let payload_end = raw_payload_offset
                .checked_add(raw_payload_length)
                .ok_or_else(|| {
                    format_violation(
                        HbpFormatErrorCode::HbpE011,
                        "schema 2.x day slice exceeds raw block bounds",
                    )
                })?;
            if payload_end > raw_block.len() {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE011,
                    "schema 2.x day slice exceeds raw block bounds",
                ));
            }
            let payload = &raw_block[raw_payload_offset..payload_end];
            if crc32c(payload) != raw_payload_crc32c {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE012,
                    "raw payload crc mismatch",
                ));
            }
            payload.to_vec()
        }
    };

    let mut cursor = Cursor::new(&payload, 0);

    let sim_year_index = cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
    let calendar_year = cursor
        .i32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
    let julian_day = cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
    let event_kind = cursor
        .u8()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
    let payload_schema_minor = cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
    let state_snapshot_count = cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?
        as usize;

    if (sim_year_index, calendar_year, julian_day, event_kind)
        != (
            entry.sim_year_index,
            entry.calendar_year,
            entry.julian_day,
            entry.event_kind,
        )
    {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE010,
            "payload and directory key mismatch",
        ));
    }

    let supported_payload_minor = match layout.schema_major {
        SUPPORTED_MAJOR_V1 => SUPPORTED_MINOR_V1,
        SUPPORTED_MAJOR_V2 => SUPPORTED_MINOR_V2,
        _ => {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE003,
                "unsupported schema major",
            ));
        }
    };

    if payload_schema_minor > supported_payload_minor {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "unsupported payload minor",
        ));
    }

    let latest_event_payload =
        match event_kind {
            0 => {
                let _baseflow = cursor
                    .i64()
                    .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
                let _dissolved = cursor
                    .i64()
                    .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
                None
            }
            1 => {
                for _ in 0..6 {
                    let _ = cursor.i64().map_err(|msg| {
                        map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg)
                    })?;
                }
                None
            }
            2 => {
                let duration_seconds = cursor
                    .f64()
                    .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
                let _time_of_concentration_hours = cursor
                    .f64()
                    .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
                let _overland_flow_alpha = cursor
                    .f64()
                    .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
                for _ in 0..6 {
                    let _ = cursor.i64().map_err(|msg| {
                        map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg)
                    })?;
                }
                let peak_runoff_m3_s = cursor
                    .f64()
                    .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
                let total_detachment_scaled = cursor
                    .i64()
                    .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;
                let total_deposition_scaled = cursor
                    .i64()
                    .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?;

                let sediment_count = cursor
                    .u32()
                    .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?
                    as usize;
                if sediment_count != usize::from(layout.npart) {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE013,
                        "event sediment count mismatch",
                    ));
                }
                let mut sediment_concentration_kg_m3 = Vec::with_capacity(sediment_count);
                for _ in 0..sediment_count {
                    sediment_concentration_kg_m3.push(cursor.f64().map_err(|msg| {
                        map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg)
                    })?);
                }

                let fraction_count = cursor
                    .u32()
                    .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?
                    as usize;
                if fraction_count != usize::from(layout.npart) {
                    return Err(format_violation(
                        HbpFormatErrorCode::HbpE013,
                        "event particle fraction count mismatch",
                    ));
                }
                let mut particle_flow_fraction = Vec::with_capacity(fraction_count);
                for _ in 0..fraction_count {
                    particle_flow_fraction.push(cursor.f64().map_err(|msg| {
                        map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg)
                    })?);
                }

                for _ in 0..2 {
                    let _ = cursor.i64().map_err(|msg| {
                        map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg)
                    })?;
                }

                Some(HbpLatestEventPayload {
                    sim_year_index,
                    calendar_year,
                    julian_day,
                    duration_seconds,
                    peak_runoff_m3_s,
                    total_detachment_kg: scaled_i64_to_f64(total_detachment_scaled)? * SCALE_I64,
                    total_deposition_kg: scaled_i64_to_f64(total_deposition_scaled)? * SCALE_I64,
                    particle_diameter_m: layout.particle_diameter_m.clone(),
                    sediment_concentration_kg_m3,
                    particle_flow_fraction,
                })
            }
            _ => {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE010,
                    "unsupported event kind",
                ));
            }
        };

    let mut state_ids_seen: HashSet<u16> = HashSet::new();

    for _ in 0..state_snapshot_count {
        let state_id = cursor
            .u16()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))?;
        let entry_length = cursor
            .u32()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))?
            as usize;
        let entry_end = cursor.pos.checked_add(entry_length).ok_or_else(|| {
            format_violation(HbpFormatErrorCode::HbpE013, "truncated state entry")
        })?;
        if entry_end > payload.len() {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE013,
                "truncated state entry",
            ));
        }

        if !state_ids_seen.insert(state_id) {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE013,
                "duplicate state id",
            ));
        }

        let mut state_cursor = Cursor::new(&payload, cursor.pos);
        let required_flag = state_cursor
            .u8()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))?;
        let representation_class = state_cursor
            .u8()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))?;
        let unit_class = state_cursor
            .u16()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))?;
        let rank = state_cursor
            .u8()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))?
            as usize;

        let mut dims = Vec::with_capacity(rank);
        for _ in 0..rank {
            dims.push(
                state_cursor.u32().map_err(|msg| {
                    map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg)
                })?,
            );
        }

        if let Some(expected) = expected_state_schema(state_id) {
            if required_flag != expected.0 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE013,
                    "state required flag does not match registry",
                ));
            }
            if representation_class != expected.1 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE013,
                    "state representation does not match registry",
                ));
            }
            if unit_class != expected.2 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE013,
                    "state unit class does not match registry",
                ));
            }
            if rank as u8 != expected.3 {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE013,
                    "state rank does not match registry",
                ));
            }

            let expected_dims = expected_dims(expected.4, layout)?;
            if dims != expected_dims {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE013,
                    "state dimensions do not match registry",
                ));
            }
        }

        let mut value_count: usize = 1;
        for dim in &dims {
            value_count = value_count.saturating_mul(*dim as usize);
        }

        match representation_class {
            1 => {
                for _ in 0..value_count {
                    let _ = state_cursor.i64().map_err(|msg| {
                        map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg)
                    })?;
                }
            }
            2 => {
                for _ in 0..value_count {
                    let _ = state_cursor.f64().map_err(|msg| {
                        map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg)
                    })?;
                }
            }
            _ => {
                return Err(format_violation(
                    HbpFormatErrorCode::HbpE013,
                    "unsupported state representation",
                ));
            }
        }

        if state_cursor.pos != entry_end {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE013,
                "state entry length mismatch",
            ));
        }

        if required_flag != 1 {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE013,
                "required state marked optional",
            ));
        }

        cursor.pos = entry_end;
    }

    if cursor.pos != payload.len() {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "payload has trailing bytes",
        ));
    }

    if let Some(missing) = REQUIRED_STATE_IDS
        .iter()
        .find(|state_id| !state_ids_seen.contains(state_id))
    {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            format!("required state id missing: {missing}"),
        ));
    }

    Ok(PayloadValidationResult {
        latest_event_payload,
    })
}

fn has_forbidden_pass_suffix(path_str: &str) -> bool {
    let lower = path_str.to_ascii_lowercase();
    lower.ends_with(".pass.hbp") || lower.ends_with(".pass.dat.hbp")
}

fn resolve_path(
    input_path: &Path,
) -> Result<(PathBuf, HbpPathResolution, Vec<HbpWarning>), HbpParseError> {
    let raw = input_path.to_string_lossy();
    let lower = raw.to_ascii_lowercase();

    if has_forbidden_pass_suffix(&raw) {
        return Err(HbpParseError::InvalidProcessHbpName {
            input_path: input_path.to_path_buf(),
            reason: "use H*.hbp; rejecting H*.pass.hbp and H*.pass.dat.hbp".to_string(),
        });
    }

    if lower.ends_with(".hbp") {
        return Ok((
            input_path.to_path_buf(),
            HbpPathResolution::Direct,
            Vec::new(),
        ));
    }

    if lower.ends_with(".pass.dat") {
        return Err(HbpParseError::InvalidProcessHbpName {
            input_path: input_path.to_path_buf(),
            reason: "legacy .pass.dat naming is unsupported; use direct H*.hbp naming".to_string(),
        });
    }

    Err(HbpParseError::InvalidProcessHbpName {
        input_path: input_path.to_path_buf(),
        reason: "invalid process HBP name; use H*.hbp".to_string(),
    })
}

fn parse_hbp_from_bytes_internal(
    bytes: &[u8],
    source_path: &Path,
    options: HbpParseOptions,
) -> Result<(HbpParseResult, Option<HbpLatestEventPayload>), HbpParseError> {
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

    let mut latest_event_payload = None;
    for entry in &layout.entries {
        let payload_validation = validate_payload(bytes, &layout, entry)?;
        if let Some(event_payload) = payload_validation.latest_event_payload {
            latest_event_payload = Some(event_payload);
        }
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
        latest_event_payload,
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
    let (resolved_path, _path_resolution, _warnings) = resolve_path(path.as_ref())?;
    let bytes = fs::read(&resolved_path).map_err(|source| HbpParseError::InputOpenError {
        path: resolved_path.clone(),
        source,
    })?;
    parse_hbp_from_bytes_with_latest_event_payload(&bytes, path.as_ref(), options)
}
