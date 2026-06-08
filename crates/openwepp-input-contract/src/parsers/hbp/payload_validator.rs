#![allow(clippy::wildcard_imports)]

use std::collections::HashSet;

use super::*;

pub(super) struct PayloadValidationResult {
    pub(super) latest_event_payload: Option<HbpLatestEventPayload>,
}

pub(super) fn validate_payload(
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
