pub(super) struct YearEntry {
    pub(super) sim_year_index: u32,
    pub(super) calendar_year: i32,
    pub(super) days_in_year: u16,
    pub(super) first_julian_day: u16,
    pub(super) last_julian_day: u16,
    pub(super) single_storm_flag: u8,
}

#[derive(Clone, Copy)]
pub(super) struct DirectoryEntry {
    pub(super) sim_year_index: u32,
    pub(super) calendar_year: i32,
    pub(super) julian_day: u16,
    pub(super) event_kind: u8,
    pub(super) payload: EntryPayload,
}

#[derive(Clone, Copy)]
pub(super) enum EntryPayload {
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
pub(super) struct PayloadBlockEntry {
    pub(super) payload_block_id: u32,
    pub(super) sim_year_index: u32,
    pub(super) block_day_slot_count: u16,
    pub(super) represented_day_count: u16,
    pub(super) stored_block_offset: usize,
    pub(super) stored_block_length: usize,
    pub(super) raw_block_length: usize,
    pub(super) payload_codec: u8,
    pub(super) stored_block_crc32c: u32,
    pub(super) raw_block_crc32c: u32,
}

pub(super) struct Layout {
    pub(super) schema_major: u16,
    pub(super) schema_minor: u16,
    pub(super) hillslope_id: u32,
    pub(super) nyear: u32,
    pub(super) begin_year: i32,
    pub(super) npart: u16,
    pub(super) particle_diameter_m: Vec<f64>,
    pub(super) nofe: u16,
    pub(super) max_layers: u16,
    pub(super) simulation_mode: u8,
    pub(super) years: Vec<YearEntry>,
    pub(super) entries: Vec<DirectoryEntry>,
    pub(super) payload_blocks: Vec<PayloadBlockEntry>,
    pub(super) raw_payload_blocks: Vec<Vec<u8>>,
}
