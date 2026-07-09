use std::path::PathBuf;

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
    pub baseflow_volume_m3: f64,
    pub deep_seepage_volume_m3: f64,
    pub particle_diameter_m: Vec<f64>,
    pub sediment_concentration_kg_m3: Vec<f64>,
    pub particle_flow_fraction: Vec<f64>,
    /// SC-INFILE-HBP-001 §3a (payload minor >= 1): hour-integrated exit
    /// runoff volume (m³); empty on minor-0 payloads.
    pub hourly_runoff_volume_m3: Vec<f64>,
    /// SC-INFILE-HBP-001 §3a (payload minor >= 1): hour-integrated exported
    /// sediment mass (kg) on the same time base; empty on minor-0 payloads.
    pub hourly_sediment_mass_kg: Vec<f64>,
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
