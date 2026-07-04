#![allow(clippy::wildcard_imports)]

use std::collections::HashSet;

use super::*;

pub(super) struct PayloadValidationResult {
    pub(super) latest_event_payload: Option<HbpLatestEventPayload>,
}

struct PayloadHeader {
    sim_year_index: u32,
    calendar_year: i32,
    julian_day: u16,
    event_kind: u8,
    payload_schema_minor: u16,
    state_snapshot_count: usize,
}

struct StateEnvelope {
    state_id: u16,
    entry_end: usize,
}

struct StateSchemaFields {
    required_flag: u8,
    representation_class: u8,
    unit_class: u16,
    rank: usize,
    dims: Vec<u32>,
}

pub(super) fn validate_payload(
    data: &[u8],
    layout: &Layout,
    entry: &DirectoryEntry,
) -> Result<PayloadValidationResult, HbpParseError> {
    let payload = extract_payload(data, layout, entry)?;
    let mut cursor = Cursor::new(&payload, 0);

    let header = read_payload_header(&mut cursor)?;
    validate_payload_key(&header, entry)?;
    validate_payload_minor(layout.schema_major, header.payload_schema_minor)?;

    let latest_event_payload = parse_latest_event_payload(&mut cursor, layout, &header)?;
    let state_ids_seen =
        validate_state_snapshots(&payload, &mut cursor, layout, header.state_snapshot_count)?;

    validate_payload_consumed(&cursor, payload.len())?;
    validate_required_state_ids(&state_ids_seen)?;

    Ok(PayloadValidationResult {
        latest_event_payload,
    })
}

fn extract_payload(
    data: &[u8],
    layout: &Layout,
    entry: &DirectoryEntry,
) -> Result<Vec<u8>, HbpParseError> {
    match entry.payload {
        EntryPayload::SchemaV1 {
            payload_offset,
            payload_length,
            payload_crc32c,
        } => extract_schema1_payload(data, payload_offset, payload_length, payload_crc32c),
        EntryPayload::SchemaV2 {
            payload_block_id,
            raw_payload_offset,
            raw_payload_length,
            raw_payload_crc32c,
            ..
        } => extract_schema2_payload(
            layout,
            payload_block_id,
            raw_payload_offset,
            raw_payload_length,
            raw_payload_crc32c,
        ),
    }
}

fn extract_schema1_payload(
    data: &[u8],
    payload_offset: usize,
    payload_length: usize,
    payload_crc32c: u32,
) -> Result<Vec<u8>, HbpParseError> {
    let payload_end = payload_offset
        .checked_add(payload_length)
        .ok_or_else(|| format_violation(HbpFormatErrorCode::HbpE013, "truncated payload"))?;
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
    Ok(payload.to_vec())
}

fn extract_schema2_payload(
    layout: &Layout,
    payload_block_id: usize,
    raw_payload_offset: usize,
    raw_payload_length: usize,
    raw_payload_crc32c: u32,
) -> Result<Vec<u8>, HbpParseError> {
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
    Ok(payload.to_vec())
}

fn read_payload_header(cursor: &mut Cursor<'_>) -> Result<PayloadHeader, HbpParseError> {
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

    Ok(PayloadHeader {
        sim_year_index,
        calendar_year,
        julian_day,
        event_kind,
        payload_schema_minor,
        state_snapshot_count,
    })
}

fn validate_payload_key(
    header: &PayloadHeader,
    entry: &DirectoryEntry,
) -> Result<(), HbpParseError> {
    if (
        header.sim_year_index,
        header.calendar_year,
        header.julian_day,
        header.event_kind,
    ) != (
        entry.sim_year_index,
        entry.calendar_year,
        entry.julian_day,
        entry.event_kind,
    ) {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE010,
            "payload and directory key mismatch",
        ));
    }
    Ok(())
}

fn supported_payload_minor(schema_major: u16) -> Result<u16, HbpParseError> {
    match schema_major {
        SUPPORTED_MAJOR_V1 => Ok(SUPPORTED_MINOR_V1),
        SUPPORTED_MAJOR_V2 => Ok(SUPPORTED_MINOR_V2),
        _ => Err(format_violation(
            HbpFormatErrorCode::HbpE003,
            "unsupported schema major",
        )),
    }
}

fn validate_payload_minor(
    schema_major: u16,
    payload_schema_minor: u16,
) -> Result<(), HbpParseError> {
    let supported_payload_minor = supported_payload_minor(schema_major)?;
    if payload_schema_minor > supported_payload_minor {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "unsupported payload minor",
        ));
    }
    Ok(())
}

fn parse_latest_event_payload(
    cursor: &mut Cursor<'_>,
    layout: &Layout,
    header: &PayloadHeader,
) -> Result<Option<HbpLatestEventPayload>, HbpParseError> {
    match header.event_kind {
        0 => parse_no_event_payload(cursor),
        1 => parse_non_runoff_event_payload(cursor),
        2 => parse_runoff_event_payload(cursor, layout, header),
        _ => Err(format_violation(
            HbpFormatErrorCode::HbpE010,
            "unsupported event kind",
        )),
    }
}

fn parse_no_event_payload(
    cursor: &mut Cursor<'_>,
) -> Result<Option<HbpLatestEventPayload>, HbpParseError> {
    let _baseflow = read_payload_i64(cursor)?;
    let _dissolved = read_payload_i64(cursor)?;
    Ok(None)
}

fn parse_non_runoff_event_payload(
    cursor: &mut Cursor<'_>,
) -> Result<Option<HbpLatestEventPayload>, HbpParseError> {
    read_payload_i64_values(cursor, 6)?;
    Ok(None)
}

fn parse_runoff_event_payload(
    cursor: &mut Cursor<'_>,
    layout: &Layout,
    header: &PayloadHeader,
) -> Result<Option<HbpLatestEventPayload>, HbpParseError> {
    let duration_seconds = read_payload_f64(cursor)?;
    let _time_of_concentration_hours = read_payload_f64(cursor)?;
    let _overland_flow_alpha = read_payload_f64(cursor)?;
    read_payload_i64_values(cursor, 6)?;
    let peak_runoff_m3_s = read_payload_f64(cursor)?;
    let total_detachment_scaled = read_payload_i64(cursor)?;
    let total_deposition_scaled = read_payload_i64(cursor)?;

    let sediment_concentration_kg_m3 = read_counted_payload_f64_values(
        cursor,
        usize::from(layout.npart),
        "event sediment count mismatch",
    )?;
    let particle_flow_fraction = read_counted_payload_f64_values(
        cursor,
        usize::from(layout.npart),
        "event particle fraction count mismatch",
    )?;

    // SC-INFILE-HBP-001 §3a (ADR-0036): payload minor >= 1 carries the
    // paired hourly surfaces BEFORE the reserved trailing i64s. Strict
    // consumption makes any writer/parser placement divergence a typed
    // failure, never a silent shift.
    let (hourly_runoff_volume_m3, hourly_sediment_mass_kg) = if header.payload_schema_minor >= 1 {
        let volumes = read_counted_payload_f64_values(
            cursor,
            HOURLY_SURFACE_SLOT_COUNT,
            "event hourly runoff-volume count mismatch",
        )?;
        let sediment = read_counted_payload_f64_values(
            cursor,
            HOURLY_SURFACE_SLOT_COUNT,
            "event hourly sediment-mass count mismatch",
        )?;
        validate_hourly_surface(&volumes, "hourly_runoff_volume_m3")?;
        validate_hourly_surface(&sediment, "hourly_sediment_mass_kg")?;
        (volumes, sediment)
    } else {
        (Vec::new(), Vec::new())
    };

    read_payload_i64_values(cursor, 2)?;

    Ok(Some(HbpLatestEventPayload {
        sim_year_index: header.sim_year_index,
        calendar_year: header.calendar_year,
        julian_day: header.julian_day,
        duration_seconds,
        peak_runoff_m3_s,
        total_detachment_kg: scaled_i64_to_f64(total_detachment_scaled)? * SCALE_I64,
        total_deposition_kg: scaled_i64_to_f64(total_deposition_scaled)? * SCALE_I64,
        particle_diameter_m: layout.particle_diameter_m.clone(),
        sediment_concentration_kg_m3,
        particle_flow_fraction,
        hourly_runoff_volume_m3,
        hourly_sediment_mass_kg,
    }))
}

/// SC-INFILE-HBP-001 §3a hourly-surface time base (24 × 1 h).
const HOURLY_SURFACE_SLOT_COUNT: usize = 24;

/// `G-HBP-011`: minor-1 hourly-surface structural closure — every element
/// finite and non-negative (`HBP-E-015`).
fn validate_hourly_surface(values: &[f64], surface: &str) -> Result<(), HbpParseError> {
    for value in values {
        if !value.is_finite() || *value < 0.0 {
            return Err(format_violation(
                HbpFormatErrorCode::HbpE015,
                format!("{surface} elements must be finite and non-negative"),
            ));
        }
    }
    Ok(())
}

fn read_payload_i64(cursor: &mut Cursor<'_>) -> Result<i64, HbpParseError> {
    cursor
        .i64()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))
}

fn read_payload_f64(cursor: &mut Cursor<'_>) -> Result<f64, HbpParseError> {
    cursor
        .f64()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))
}

fn read_payload_i64_values(cursor: &mut Cursor<'_>, count: usize) -> Result<(), HbpParseError> {
    for _ in 0..count {
        let _ = read_payload_i64(cursor)?;
    }
    Ok(())
}

fn read_counted_payload_f64_values(
    cursor: &mut Cursor<'_>,
    expected_count: usize,
    mismatch_detail: &'static str,
) -> Result<Vec<f64>, HbpParseError> {
    let count = cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "payload", msg))?
        as usize;
    validate_payload_value_count(count, expected_count, mismatch_detail)?;
    read_payload_f64_values(cursor, count)
}

fn validate_payload_value_count(
    count: usize,
    expected_count: usize,
    mismatch_detail: &'static str,
) -> Result<(), HbpParseError> {
    if count != expected_count {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            mismatch_detail,
        ));
    }
    Ok(())
}

fn read_payload_f64_values(
    cursor: &mut Cursor<'_>,
    count: usize,
) -> Result<Vec<f64>, HbpParseError> {
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        values.push(read_payload_f64(cursor)?);
    }
    Ok(values)
}

fn validate_state_snapshots(
    payload: &[u8],
    cursor: &mut Cursor<'_>,
    layout: &Layout,
    state_snapshot_count: usize,
) -> Result<HashSet<u16>, HbpParseError> {
    let mut state_ids_seen: HashSet<u16> = HashSet::new();
    for _ in 0..state_snapshot_count {
        validate_state_snapshot(payload, cursor, layout, &mut state_ids_seen)?;
    }
    Ok(state_ids_seen)
}

fn validate_state_snapshot(
    payload: &[u8],
    cursor: &mut Cursor<'_>,
    layout: &Layout,
    state_ids_seen: &mut HashSet<u16>,
) -> Result<(), HbpParseError> {
    let envelope = read_state_envelope(payload, cursor)?;
    validate_unique_state_id(state_ids_seen, envelope.state_id)?;

    let mut state_cursor = Cursor::new(payload, cursor.pos);
    let fields = read_state_schema_fields(&mut state_cursor)?;
    validate_state_schema(envelope.state_id, &fields, layout)?;
    read_state_values(
        &mut state_cursor,
        fields.representation_class,
        state_value_count(&fields.dims),
    )?;
    validate_state_cursor_end(&state_cursor, envelope.entry_end)?;
    validate_required_state_flag(fields.required_flag)?;

    cursor.pos = envelope.entry_end;
    Ok(())
}

fn read_state_envelope(
    payload: &[u8],
    cursor: &mut Cursor<'_>,
) -> Result<StateEnvelope, HbpParseError> {
    let state_id = read_state_u16(cursor)?;
    let entry_length = read_state_u32(cursor)? as usize;
    let entry_end = cursor
        .pos
        .checked_add(entry_length)
        .ok_or_else(|| format_violation(HbpFormatErrorCode::HbpE013, "truncated state entry"))?;
    validate_state_entry_bounds(entry_end, payload.len())?;
    Ok(StateEnvelope {
        state_id,
        entry_end,
    })
}

fn validate_state_entry_bounds(entry_end: usize, payload_len: usize) -> Result<(), HbpParseError> {
    if entry_end > payload_len {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "truncated state entry",
        ));
    }
    Ok(())
}

fn validate_unique_state_id(
    state_ids_seen: &mut HashSet<u16>,
    state_id: u16,
) -> Result<(), HbpParseError> {
    if !state_ids_seen.insert(state_id) {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "duplicate state id",
        ));
    }
    Ok(())
}

fn read_state_schema_fields(cursor: &mut Cursor<'_>) -> Result<StateSchemaFields, HbpParseError> {
    let required_flag = read_state_u8(cursor)?;
    let representation_class = read_state_u8(cursor)?;
    let unit_class = read_state_u16(cursor)?;
    let rank = read_state_u8(cursor)? as usize;
    let dims = read_state_dims(cursor, rank)?;

    Ok(StateSchemaFields {
        required_flag,
        representation_class,
        unit_class,
        rank,
        dims,
    })
}

fn read_state_dims(cursor: &mut Cursor<'_>, rank: usize) -> Result<Vec<u32>, HbpParseError> {
    let mut dims = Vec::with_capacity(rank);
    for _ in 0..rank {
        dims.push(read_state_u32(cursor)?);
    }
    Ok(dims)
}

fn validate_state_schema(
    state_id: u16,
    fields: &StateSchemaFields,
    layout: &Layout,
) -> Result<(), HbpParseError> {
    if let Some(expected) = expected_state_schema(state_id) {
        validate_expected_state_schema(fields, expected, layout)?;
    }
    Ok(())
}

fn validate_expected_state_schema(
    fields: &StateSchemaFields,
    expected: (u8, u8, u16, u8, u8),
    layout: &Layout,
) -> Result<(), HbpParseError> {
    validate_expected_required_flag(fields.required_flag, expected.0)?;
    validate_expected_representation(fields.representation_class, expected.1)?;
    validate_expected_unit_class(fields.unit_class, expected.2)?;
    validate_expected_rank(fields.rank, expected.3)?;
    validate_expected_dims(&fields.dims, expected.4, layout)
}

fn validate_expected_required_flag(actual: u8, expected: u8) -> Result<(), HbpParseError> {
    if actual != expected {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "state required flag does not match registry",
        ));
    }
    Ok(())
}

fn validate_expected_representation(actual: u8, expected: u8) -> Result<(), HbpParseError> {
    if actual != expected {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "state representation does not match registry",
        ));
    }
    Ok(())
}

fn validate_expected_unit_class(actual: u16, expected: u16) -> Result<(), HbpParseError> {
    if actual != expected {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "state unit class does not match registry",
        ));
    }
    Ok(())
}

fn validate_expected_rank(actual: usize, expected: u8) -> Result<(), HbpParseError> {
    if actual as u8 != expected {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "state rank does not match registry",
        ));
    }
    Ok(())
}

fn validate_expected_dims(
    actual: &[u32],
    expected_kind: u8,
    layout: &Layout,
) -> Result<(), HbpParseError> {
    let expected_dims = expected_dims(expected_kind, layout)?;
    if actual != expected_dims {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "state dimensions do not match registry",
        ));
    }
    Ok(())
}

fn state_value_count(dims: &[u32]) -> usize {
    let mut value_count: usize = 1;
    for dim in dims {
        value_count = value_count.saturating_mul(*dim as usize);
    }
    value_count
}

fn read_state_values(
    cursor: &mut Cursor<'_>,
    representation_class: u8,
    value_count: usize,
) -> Result<(), HbpParseError> {
    match representation_class {
        1 => read_state_i64_values(cursor, value_count),
        2 => read_state_f64_values(cursor, value_count),
        _ => Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "unsupported state representation",
        )),
    }
}

fn read_state_i64_values(cursor: &mut Cursor<'_>, count: usize) -> Result<(), HbpParseError> {
    for _ in 0..count {
        let _ = cursor
            .i64()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))?;
    }
    Ok(())
}

fn read_state_f64_values(cursor: &mut Cursor<'_>, count: usize) -> Result<(), HbpParseError> {
    for _ in 0..count {
        let _ = cursor
            .f64()
            .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))?;
    }
    Ok(())
}

fn validate_state_cursor_end(
    state_cursor: &Cursor<'_>,
    entry_end: usize,
) -> Result<(), HbpParseError> {
    if state_cursor.pos != entry_end {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "state entry length mismatch",
        ));
    }
    Ok(())
}

fn validate_required_state_flag(required_flag: u8) -> Result<(), HbpParseError> {
    if required_flag != 1 {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "required state marked optional",
        ));
    }
    Ok(())
}

fn read_state_u8(cursor: &mut Cursor<'_>) -> Result<u8, HbpParseError> {
    cursor
        .u8()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))
}

fn read_state_u16(cursor: &mut Cursor<'_>) -> Result<u16, HbpParseError> {
    cursor
        .u16()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))
}

fn read_state_u32(cursor: &mut Cursor<'_>) -> Result<u32, HbpParseError> {
    cursor
        .u32()
        .map_err(|msg| map_cursor_err(HbpFormatErrorCode::HbpE013, "state entry", msg))
}

fn validate_payload_consumed(cursor: &Cursor<'_>, payload_len: usize) -> Result<(), HbpParseError> {
    if cursor.pos != payload_len {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            "payload has trailing bytes",
        ));
    }
    Ok(())
}

fn validate_required_state_ids(state_ids_seen: &HashSet<u16>) -> Result<(), HbpParseError> {
    if let Some(missing) = REQUIRED_STATE_IDS
        .iter()
        .find(|state_id| !state_ids_seen.contains(state_id))
    {
        return Err(format_violation(
            HbpFormatErrorCode::HbpE013,
            format!("required state id missing: {missing}"),
        ));
    }
    Ok(())
}
