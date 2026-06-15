use std::collections::HashSet;

#[allow(clippy::wildcard_imports)]
use super::*;

#[derive(Clone, Copy)]
struct ParsedHeader {
    schema_major: u16,
    schema_minor: u16,
}

struct ParsedMetadata {
    hillslope_id: u32,
    nyear: u32,
    begin_year: i32,
    npart: u16,
    particle_diameter_m: Vec<f64>,
    nofe: u16,
    max_layers: u16,
    simulation_mode: u8,
    years: Vec<YearEntry>,
    expected_record_count: u32,
}

struct ParsedDirectory {
    start: usize,
    end: usize,
    entries: Vec<DirectoryEntry>,
}

struct PayloadBlockTable {
    start: usize,
    end: usize,
    blocks: Vec<PayloadBlockEntry>,
}

struct RegistryEntry {
    state_id: u16,
    required_flag: u8,
    representation_class: u8,
    unit_class: u16,
    rank: u8,
    dims_kind: u8,
}

pub(super) fn parse_layout(data: &[u8]) -> Result<Layout, HbpParseError> {
    let mut cursor = Cursor::new(data, 0);

    let header = parse_header(data, &mut cursor)?;
    let metadata = parse_metadata(&mut cursor, header.schema_major)?;
    parse_state_registry(&mut cursor)?;
    let directory = parse_directory(
        &mut cursor,
        header.schema_major,
        &metadata.years,
        metadata.expected_record_count,
    )?;

    match header.schema_major {
        SUPPORTED_MAJOR_V1 => parse_schema1_layout(data, header, metadata, directory),
        SUPPORTED_MAJOR_V2 => parse_schema2_layout(data, &mut cursor, header, metadata, directory),
        _ => Err(format_violation(
            HbpFormatErrorCode::HbpE003,
            "unsupported schema major",
        )),
    }
}

fn parse_header(data: &[u8], cursor: &mut Cursor<'_>) -> Result<ParsedHeader, HbpParseError> {
    let magic = read_raw(cursor, 8, "header")?;
    if magic != MAGIC {
        return Err(format_violation(HbpFormatErrorCode::HbpE002, "bad magic"));
    }

    let schema_major = read_u16(cursor, "header")?;
    let schema_minor = read_u16(cursor, "header")?;
    validate_schema_version(schema_major, schema_minor)?;

    let endianness = read_u8(cursor, "header")?;
    if endianness != 1 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE005,
            "unsupported endianness",
        ));
    }

    let header_bytes = read_u32(cursor, "header")? as usize;
    if header_bytes > data.len() {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE006,
            "header length exceeds file length",
        ));
    }

    read_raw(cursor, 32, "header")?;
    let artifact_role = read_u8(cursor, "header")?;
    if artifact_role != 1 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE006,
            "unsupported artifact role",
        ));
    }
    read_string(cursor, "header")?;
    read_string(cursor, "header")?;
    read_string(cursor, "header")?;
    read_string(cursor, "header")?;
    read_raw(cursor, 32, "header")?;

    let header_crc_pos = cursor.pos;
    let header_crc = read_u32(cursor, "header")?;
    validate_header_crc(data, cursor.pos, header_bytes, header_crc_pos, header_crc)?;

    Ok(ParsedHeader {
        schema_major,
        schema_minor,
    })
}

fn validate_schema_version(schema_major: u16, schema_minor: u16) -> Result<(), HbpParseError> {
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
    Ok(())
}

fn validate_header_crc(
    data: &[u8],
    cursor_pos: usize,
    header_bytes: usize,
    header_crc_pos: usize,
    header_crc: u32,
) -> Result<(), HbpParseError> {
    if cursor_pos != header_bytes {
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
    Ok(())
}

fn parse_metadata(
    cursor: &mut Cursor<'_>,
    schema_major: u16,
) -> Result<ParsedMetadata, HbpParseError> {
    let hillslope_id = read_u32(cursor, "dimension")?;
    let nyear = read_u32(cursor, "dimension")?;
    let begin_year = read_i32(cursor, "dimension")?;
    let npart = read_u16(cursor, "dimension")?;
    let nofe = read_u16(cursor, "dimension")?;
    let max_layers = read_u16(cursor, "dimension")?;
    read_string(cursor, "dimension")?;
    read_u16(cursor, "dimension")?;
    let simulation_mode = read_u8(cursor, "dimension")?;

    read_string(cursor, "metadata")?;
    read_i64(cursor, "metadata")?;
    let particle_diameter_m = parse_particle_diameters(cursor, npart)?;
    read_f64(cursor, "metadata")?;
    read_f64(cursor, "metadata")?;
    read_f64(cursor, "metadata")?;
    read_f64(cursor, "metadata")?;

    let years = parse_year_table(cursor)?;
    let expected_record_count = validate_year_table(&years, nyear, schema_major, simulation_mode)?;

    Ok(ParsedMetadata {
        hillslope_id,
        nyear,
        begin_year,
        npart,
        particle_diameter_m,
        nofe,
        max_layers,
        simulation_mode,
        years,
        expected_record_count,
    })
}

fn parse_particle_diameters(
    cursor: &mut Cursor<'_>,
    npart: u16,
) -> Result<Vec<f64>, HbpParseError> {
    let particle_count = read_u32(cursor, "metadata")? as usize;
    let mut particle_diameter_m = Vec::with_capacity(particle_count);
    for _ in 0..particle_count {
        let diameter_m = read_f64(cursor, "metadata")?;
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
    Ok(particle_diameter_m)
}

fn parse_year_table(cursor: &mut Cursor<'_>) -> Result<Vec<YearEntry>, HbpParseError> {
    let year_count = read_u32(cursor, "year table")?;
    let mut years = Vec::with_capacity(year_count as usize);
    for _ in 0..year_count {
        years.push(YearEntry {
            sim_year_index: read_u32(cursor, "year table")?,
            calendar_year: read_i32(cursor, "year table")?,
            days_in_year: read_u16(cursor, "year table")?,
            first_julian_day: read_u16(cursor, "year table")?,
            last_julian_day: read_u16(cursor, "year table")?,
            single_storm_flag: read_u8(cursor, "year table")?,
        });
    }
    Ok(years)
}

fn parse_state_registry(cursor: &mut Cursor<'_>) -> Result<(), HbpParseError> {
    let registry_count = read_u32(cursor, "state registry")?;
    let mut registry_state_ids = Vec::with_capacity(registry_count as usize);
    let mut registry_seen: HashSet<u16> = HashSet::new();

    for _ in 0..registry_count {
        let entry = parse_state_registry_entry(cursor)?;
        if !registry_seen.insert(entry.state_id) {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE009,
                "duplicate registry state id",
            ));
        }
        validate_state_registry_entry(&entry)?;
        registry_state_ids.push(entry.state_id);
    }

    validate_required_state_ids(registry_state_ids)
}

fn parse_state_registry_entry(cursor: &mut Cursor<'_>) -> Result<RegistryEntry, HbpParseError> {
    let state_id = read_u16(cursor, "state registry")?;
    let required_flag = read_u8(cursor, "state registry")?;
    let representation_class = read_u8(cursor, "state registry")?;
    let unit_class = read_u16(cursor, "state registry")?;
    let rank = read_u8(cursor, "state registry")?;
    let dims_kind = read_u8(cursor, "state registry")?;
    read_string(cursor, "state registry")?;

    Ok(RegistryEntry {
        state_id,
        required_flag,
        representation_class,
        unit_class,
        rank,
        dims_kind,
    })
}

fn validate_state_registry_entry(entry: &RegistryEntry) -> Result<(), HbpParseError> {
    if let Some(expected) = expected_state_schema(entry.state_id)
        && expected
            != (
                entry.required_flag,
                entry.representation_class,
                entry.unit_class,
                entry.rank,
                entry.dims_kind,
            )
    {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE009,
            "state registry block does not match canonical schema",
        ));
    }
    Ok(())
}

fn validate_required_state_ids(registry_state_ids: Vec<u16>) -> Result<(), HbpParseError> {
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
    Ok(())
}

fn parse_directory(
    cursor: &mut Cursor<'_>,
    schema_major: u16,
    years: &[YearEntry],
    expected_record_count: u32,
) -> Result<ParsedDirectory, HbpParseError> {
    let start = cursor.pos;
    let record_count = read_u32(cursor, "day directory")? as usize;
    let mut entries = Vec::with_capacity(record_count);

    for _ in 0..record_count {
        entries.push(parse_directory_entry(cursor, schema_major)?);
    }

    let end = cursor.pos;
    validate_directory_entries(&entries, record_count, expected_record_count, years)?;
    Ok(ParsedDirectory {
        start,
        end,
        entries,
    })
}

fn parse_directory_entry(
    cursor: &mut Cursor<'_>,
    schema_major: u16,
) -> Result<DirectoryEntry, HbpParseError> {
    let sim_year_index = read_u32(cursor, "day directory")?;
    let calendar_year = read_i32(cursor, "day directory")?;
    let julian_day = read_u16(cursor, "day directory")?;
    let event_kind = read_u8(cursor, "day directory")?;
    let payload = match schema_major {
        SUPPORTED_MAJOR_V1 => parse_schema1_entry_payload(cursor)?,
        SUPPORTED_MAJOR_V2 => parse_schema2_entry_payload(cursor)?,
        _ => {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE003,
                "unsupported schema major",
            ));
        }
    };

    Ok(DirectoryEntry {
        sim_year_index,
        calendar_year,
        julian_day,
        event_kind,
        payload,
    })
}

fn parse_schema1_entry_payload(cursor: &mut Cursor<'_>) -> Result<EntryPayload, HbpParseError> {
    let payload_offset = read_u64_usize(cursor, "day directory", "payload_offset_bytes")?;
    let payload_length = read_u32(cursor, "day directory")? as usize;
    let payload_crc32c = read_u32(cursor, "day directory")?;
    if payload_length < 1 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE010,
            "payload length must be positive",
        ));
    }
    Ok(EntryPayload::SchemaV1 {
        payload_offset,
        payload_length,
        payload_crc32c,
    })
}

fn parse_schema2_entry_payload(cursor: &mut Cursor<'_>) -> Result<EntryPayload, HbpParseError> {
    Ok(EntryPayload::SchemaV2 {
        payload_block_id: read_u32(cursor, "day directory")? as usize,
        day_in_block_index: read_u16(cursor, "day directory")?,
        raw_payload_offset: read_u32(cursor, "day directory")? as usize,
        raw_payload_length: read_u32(cursor, "day directory")? as usize,
        raw_payload_crc32c: read_u32(cursor, "day directory")?,
    })
}

fn validate_directory_entries(
    entries: &[DirectoryEntry],
    record_count: usize,
    expected_record_count: u32,
    years: &[YearEntry],
) -> Result<(), HbpParseError> {
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
    for entry in entries {
        let key = (entry.sim_year_index, entry.julian_day);
        if !key_in_year_table(entry, years) {
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
    Ok(())
}

fn parse_schema1_layout(
    data: &[u8],
    header: ParsedHeader,
    metadata: ParsedMetadata,
    directory: ParsedDirectory,
) -> Result<Layout, HbpParseError> {
    let footer_start = validate_schema1_payload_offsets(directory.end, &directory.entries)?;
    validate_schema1_footer(
        data,
        directory.start,
        directory.end,
        footer_start,
        metadata.expected_record_count,
    )?;
    Ok(build_layout(
        header,
        metadata,
        directory,
        Vec::new(),
        Vec::new(),
    ))
}

fn validate_schema1_payload_offsets(
    directory_end: usize,
    entries: &[DirectoryEntry],
) -> Result<usize, HbpParseError> {
    let mut expected_payload_offset = directory_end;
    for entry in entries {
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
            .ok_or_else(|| format_violation(HbpFormatErrorCode::HbpE013, "truncated payload"))?;
    }
    Ok(expected_payload_offset)
}

fn validate_schema1_footer(
    data: &[u8],
    directory_start: usize,
    directory_end: usize,
    footer_start: usize,
    expected_record_count: u32,
) -> Result<(), HbpParseError> {
    let footer_end = footer_start + 20;
    if footer_end > data.len() {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "truncated payload",
        ));
    }

    let mut footer_cursor = Cursor::new(data, footer_start);
    let directory_crc = read_u32(&mut footer_cursor, "footer")?;
    let file_crc_pos = footer_cursor.pos;
    let file_crc = read_u32(&mut footer_cursor, "footer")?;
    let footer_record_count = read_u32(&mut footer_cursor, "footer")?;
    let footer_magic = read_raw(&mut footer_cursor, 8, "footer")?;

    validate_directory_crc(data, directory_start, directory_end, directory_crc)?;
    validate_file_crc(data, file_crc_pos, file_crc)?;
    validate_footer_record_count(footer_record_count, expected_record_count)?;
    validate_footer_magic(footer_magic)
}

fn parse_schema2_layout(
    data: &[u8],
    cursor: &mut Cursor<'_>,
    header: ParsedHeader,
    metadata: ParsedMetadata,
    directory: ParsedDirectory,
) -> Result<Layout, HbpParseError> {
    let payload_block_table = parse_payload_block_table(cursor, metadata.nyear)?;
    validate_schema2_footer(
        data,
        &directory,
        &payload_block_table,
        metadata.nyear,
        metadata.expected_record_count,
    )?;
    let raw_payload_blocks = decode_raw_payload_blocks(data, &payload_block_table.blocks)?;
    validate_schema2_day_slices(
        &directory.entries,
        &payload_block_table.blocks,
        &raw_payload_blocks,
    )?;

    Ok(build_layout(
        header,
        metadata,
        directory,
        payload_block_table.blocks,
        raw_payload_blocks,
    ))
}

fn parse_payload_block_table(
    cursor: &mut Cursor<'_>,
    nyear: u32,
) -> Result<PayloadBlockTable, HbpParseError> {
    let start = cursor.pos;
    let payload_block_count = read_u32(cursor, "payload block table")? as usize;
    if payload_block_count != nyear as usize {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE011,
            "schema 2.x block count must equal year table count",
        ));
    }

    let mut blocks = Vec::with_capacity(payload_block_count);
    for block_index in 0..payload_block_count {
        blocks.push(parse_payload_block_entry(cursor, block_index)?);
    }

    Ok(PayloadBlockTable {
        start,
        end: cursor.pos,
        blocks,
    })
}

fn parse_payload_block_entry(
    cursor: &mut Cursor<'_>,
    block_index: usize,
) -> Result<PayloadBlockEntry, HbpParseError> {
    let payload_block_id = read_u32(cursor, "payload block table")?;
    if payload_block_id != block_index as u32 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE011,
            "schema 2.x payload_block_id must be contiguous and ordered",
        ));
    }

    let block_sim_year_index = read_u32(cursor, "payload block table")?;
    if block_sim_year_index != (block_index + 1) as u32 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE011,
            "schema 2.x payload block sim_year_index mismatch",
        ));
    }

    let block_day_slot_count = read_u16(cursor, "payload block table")?;
    let represented_day_count = read_u16(cursor, "payload block table")?;
    validate_payload_block_day_counts(block_day_slot_count, represented_day_count)?;

    let stored_block_offset =
        read_u64_usize(cursor, "payload block table", "stored_block_offset_bytes")?;
    let stored_block_length = read_u32(cursor, "payload block table")? as usize;
    let raw_block_length = read_u32(cursor, "payload block table")? as usize;
    let payload_codec = read_u8(cursor, "payload block table")?;
    validate_payload_codec(payload_codec)?;

    let stored_block_crc32c = read_u32(cursor, "payload block table")?;
    let raw_block_crc32c = read_u32(cursor, "payload block table")?;
    validate_payload_block_lengths(stored_block_length, raw_block_length)?;

    Ok(PayloadBlockEntry {
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
    })
}

fn validate_payload_block_day_counts(
    block_day_slot_count: u16,
    represented_day_count: u16,
) -> Result<(), HbpParseError> {
    if block_day_slot_count != 366 || represented_day_count != 366 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE011,
            "schema 2.0 payload block day counts must be 366",
        ));
    }
    Ok(())
}

fn validate_payload_codec(payload_codec: u8) -> Result<(), HbpParseError> {
    if payload_codec != PAYLOAD_CODEC_ZLIB {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE011,
            "schema 2.x payload codec is unsupported",
        ));
    }
    Ok(())
}

fn validate_payload_block_lengths(
    stored_block_length: usize,
    raw_block_length: usize,
) -> Result<(), HbpParseError> {
    if stored_block_length < 1 || raw_block_length < 1 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE011,
            "schema 2.x payload block lengths must be positive",
        ));
    }
    Ok(())
}

fn validate_schema2_footer(
    data: &[u8],
    directory: &ParsedDirectory,
    payload_block_table: &PayloadBlockTable,
    nyear: u32,
    expected_record_count: u32,
) -> Result<(), HbpParseError> {
    if data.len() < 28 || payload_block_table.end > data.len() - 28 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "truncated payload",
        ));
    }

    let footer_start = data.len() - 28;
    let mut footer_cursor = Cursor::new(data, footer_start);
    let directory_crc = read_u32(&mut footer_cursor, "footer")?;
    let payload_block_table_crc = read_u32(&mut footer_cursor, "footer")?;
    let file_crc_pos = footer_cursor.pos;
    let file_crc = read_u32(&mut footer_cursor, "footer")?;
    let footer_record_count = read_u32(&mut footer_cursor, "footer")?;
    let footer_block_count = read_u32(&mut footer_cursor, "footer")?;
    let footer_magic = read_raw(&mut footer_cursor, 8, "footer")?;

    validate_footer_magic(footer_magic)?;
    validate_footer_record_count(footer_record_count, expected_record_count)?;
    validate_schema2_record_count(footer_record_count, nyear)?;
    validate_schema2_footer_block_count(footer_block_count, payload_block_table.blocks.len())?;
    validate_directory_crc(data, directory.start, directory.end, directory_crc)?;
    validate_payload_block_table_crc(data, payload_block_table, payload_block_table_crc)?;
    validate_file_crc(data, file_crc_pos, file_crc)
}

fn validate_footer_magic(footer_magic: &[u8]) -> Result<(), HbpParseError> {
    if footer_magic != FOOTER_MAGIC {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE012,
            "bad footer magic",
        ));
    }
    Ok(())
}

fn validate_footer_record_count(
    footer_record_count: u32,
    expected_record_count: u32,
) -> Result<(), HbpParseError> {
    if footer_record_count != expected_record_count {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE012,
            "footer record count must equal sum of year-table days",
        ));
    }
    Ok(())
}

fn validate_schema2_record_count(
    footer_record_count: u32,
    nyear: u32,
) -> Result<(), HbpParseError> {
    if footer_record_count != 366 * nyear {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE012,
            "schema 2.0 record count must equal 366 * nyear",
        ));
    }
    Ok(())
}

fn validate_schema2_footer_block_count(
    footer_block_count: u32,
    payload_block_count: usize,
) -> Result<(), HbpParseError> {
    if footer_block_count != payload_block_count as u32 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE012,
            "schema 2.x footer block count mismatch",
        ));
    }
    Ok(())
}

fn validate_directory_crc(
    data: &[u8],
    directory_start: usize,
    directory_end: usize,
    directory_crc: u32,
) -> Result<(), HbpParseError> {
    if crc32c(&data[directory_start..directory_end]) != directory_crc {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE012,
            "directory crc mismatch",
        ));
    }
    Ok(())
}

fn validate_payload_block_table_crc(
    data: &[u8],
    payload_block_table: &PayloadBlockTable,
    payload_block_table_crc: u32,
) -> Result<(), HbpParseError> {
    if crc32c(&data[payload_block_table.start..payload_block_table.end]) != payload_block_table_crc
    {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE012,
            "payload block table crc mismatch",
        ));
    }
    Ok(())
}

fn validate_file_crc(data: &[u8], file_crc_pos: usize, file_crc: u32) -> Result<(), HbpParseError> {
    let mut file_region = data.to_vec();
    file_region[file_crc_pos..file_crc_pos + 4].fill(0);
    if crc32c(&file_region) != file_crc {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE012,
            "file crc mismatch",
        ));
    }
    Ok(())
}

fn decode_raw_payload_blocks(
    data: &[u8],
    payload_blocks: &[PayloadBlockEntry],
) -> Result<Vec<Vec<u8>>, HbpParseError> {
    let mut raw_payload_blocks = Vec::with_capacity(payload_blocks.len());
    for block in payload_blocks {
        let stored = stored_block_slice(data, block)?;
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
    Ok(raw_payload_blocks)
}

fn stored_block_slice<'a>(
    data: &'a [u8],
    block: &PayloadBlockEntry,
) -> Result<&'a [u8], HbpParseError> {
    let stored_end = block
        .stored_block_offset
        .checked_add(block.stored_block_length)
        .ok_or_else(|| format_violation(HbpFormatErrorCode::HbpE013, "truncated payload"))?;
    if stored_end > data.len() {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "schema 2.x stored payload block exceeds file bounds",
        ));
    }
    Ok(&data[block.stored_block_offset..stored_end])
}

fn validate_schema2_day_slices(
    entries: &[DirectoryEntry],
    payload_blocks: &[PayloadBlockEntry],
    raw_payload_blocks: &[Vec<u8>],
) -> Result<(), HbpParseError> {
    let mut per_block_ranges: Vec<Vec<(u16, usize, usize)>> =
        vec![Vec::new(); payload_blocks.len()];

    for entry in entries {
        let (payload_block_id, day_in_block_index, raw_payload_offset, raw_payload_length) =
            schema2_entry_payload(entry)?;
        validate_schema2_entry_payload_key(
            entry,
            payload_blocks,
            payload_block_id,
            day_in_block_index,
        )?;
        let raw_end = validate_schema2_raw_range(
            raw_payload_blocks,
            payload_block_id,
            raw_payload_offset,
            raw_payload_length,
        )?;
        per_block_ranges[payload_block_id].push((day_in_block_index, raw_payload_offset, raw_end));
    }

    for (block_id, ranges) in per_block_ranges.iter_mut().enumerate() {
        validate_schema2_block_ranges(block_id, ranges, payload_blocks, raw_payload_blocks)?;
    }
    Ok(())
}

fn schema2_entry_payload(
    entry: &DirectoryEntry,
) -> Result<(usize, u16, usize, usize), HbpParseError> {
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

    Ok((
        payload_block_id,
        day_in_block_index,
        raw_payload_offset,
        raw_payload_length,
    ))
}

fn validate_schema2_entry_payload_key(
    entry: &DirectoryEntry,
    payload_blocks: &[PayloadBlockEntry],
    payload_block_id: usize,
    day_in_block_index: u16,
) -> Result<(), HbpParseError> {
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
    Ok(())
}

fn validate_schema2_raw_range(
    raw_payload_blocks: &[Vec<u8>],
    payload_block_id: usize,
    raw_payload_offset: usize,
    raw_payload_length: usize,
) -> Result<usize, HbpParseError> {
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
    Ok(raw_end)
}

fn validate_schema2_block_ranges(
    block_id: usize,
    ranges: &mut [(u16, usize, usize)],
    payload_blocks: &[PayloadBlockEntry],
    raw_payload_blocks: &[Vec<u8>],
) -> Result<(), HbpParseError> {
    let block = payload_blocks[block_id];
    if ranges.len() != usize::from(block.block_day_slot_count) {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE011,
            "schema 2.0 payload block must represent 366 day slots",
        ));
    }

    ranges.sort_by_key(|row| row.0);
    validate_schema2_day_slot_sequence(ranges)?;
    validate_schema2_raw_coverage(ranges, raw_payload_blocks[block_id].len())
}

fn validate_schema2_day_slot_sequence(ranges: &[(u16, usize, usize)]) -> Result<(), HbpParseError> {
    for (expected, (day_slot, _, _)) in ranges.iter().enumerate() {
        if usize::from(*day_slot) != expected {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE011,
                "schema 2.x day slots must terminate at index 365",
            ));
        }
    }
    Ok(())
}

fn validate_schema2_raw_coverage(
    ranges: &[(u16, usize, usize)],
    raw_block_length: usize,
) -> Result<(), HbpParseError> {
    let mut previous_end = 0usize;
    for (_, start, end) in ranges {
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

    if previous_end != raw_block_length {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE011,
            "schema 2.x day slices must cover raw block without gaps",
        ));
    }
    Ok(())
}

fn build_layout(
    header: ParsedHeader,
    metadata: ParsedMetadata,
    directory: ParsedDirectory,
    payload_blocks: Vec<PayloadBlockEntry>,
    raw_payload_blocks: Vec<Vec<u8>>,
) -> Layout {
    Layout {
        schema_major: header.schema_major,
        schema_minor: header.schema_minor,
        hillslope_id: metadata.hillslope_id,
        nyear: metadata.nyear,
        begin_year: metadata.begin_year,
        npart: metadata.npart,
        particle_diameter_m: metadata.particle_diameter_m,
        nofe: metadata.nofe,
        max_layers: metadata.max_layers,
        simulation_mode: metadata.simulation_mode,
        years: metadata.years,
        entries: directory.entries,
        payload_blocks,
        raw_payload_blocks,
    }
}

fn read_u8(cursor: &mut Cursor<'_>, context: &'static str) -> Result<u8, HbpParseError> {
    cursor
        .u8()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, context, msg))
}

fn read_u16(cursor: &mut Cursor<'_>, context: &'static str) -> Result<u16, HbpParseError> {
    cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, context, msg))
}

fn read_u32(cursor: &mut Cursor<'_>, context: &'static str) -> Result<u32, HbpParseError> {
    cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, context, msg))
}

fn read_i32(cursor: &mut Cursor<'_>, context: &'static str) -> Result<i32, HbpParseError> {
    cursor
        .i32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, context, msg))
}

fn read_u64(cursor: &mut Cursor<'_>, context: &'static str) -> Result<u64, HbpParseError> {
    cursor
        .u64()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, context, msg))
}

fn read_i64(cursor: &mut Cursor<'_>, context: &'static str) -> Result<i64, HbpParseError> {
    cursor
        .i64()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, context, msg))
}

fn read_f64(cursor: &mut Cursor<'_>, context: &'static str) -> Result<f64, HbpParseError> {
    cursor
        .f64()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, context, msg))
}

fn read_raw<'a>(
    cursor: &mut Cursor<'a>,
    count: usize,
    context: &'static str,
) -> Result<&'a [u8], HbpParseError> {
    cursor
        .raw(count)
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, context, msg))
}

fn read_string(cursor: &mut Cursor<'_>, context: &'static str) -> Result<String, HbpParseError> {
    cursor
        .string()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, context, msg))
}

fn read_u64_usize(
    cursor: &mut Cursor<'_>,
    context: &'static str,
    field_name: &'static str,
) -> Result<usize, HbpParseError> {
    u64_to_usize(read_u64(cursor, context)?, field_name)
}
