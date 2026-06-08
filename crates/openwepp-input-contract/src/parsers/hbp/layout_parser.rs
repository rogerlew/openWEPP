use std::collections::HashSet;

#[allow(clippy::wildcard_imports)]
use super::*;

pub(super) fn parse_layout(data: &[u8]) -> Result<Layout, HbpParseError> {
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
