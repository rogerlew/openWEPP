use flate2::read::ZlibDecoder;

use super::{
    DIM_NOFE, DIM_NOFE_LAYERS, DIM_SCALAR, DirectoryEntry, HbpFormatErrorCode, HbpParseError,
    Layout, SUPPORTED_MAJOR_V2, YearEntry,
};

pub(super) fn format_violation(
    code: HbpFormatErrorCode,
    detail: impl Into<String>,
) -> HbpParseError {
    HbpParseError::FormatViolation {
        code,
        detail: detail.into(),
    }
}

pub(super) fn map_cursor_err(
    code: HbpFormatErrorCode,
    context: &'static str,
    msg: &'static str,
) -> HbpParseError {
    format_violation(code, format!("{context}: {msg}"))
}

pub(super) fn crc32c(data: &[u8]) -> u32 {
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

pub(super) fn expected_state_schema(state_id: u16) -> Option<(u8, u8, u16, u8, u8)> {
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

pub(super) fn expected_dims(dims_kind: u8, layout: &Layout) -> Result<Vec<u32>, HbpParseError> {
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

pub(super) fn decode_zlib_block(
    source: &[u8],
    expected_raw_length: usize,
) -> Result<Vec<u8>, HbpParseError> {
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

pub(super) fn u64_to_usize(value: u64, field_name: &str) -> Result<usize, HbpParseError> {
    usize::try_from(value).map_err(|_| {
        format_violation(
            HbpFormatErrorCode::HbpE013,
            format!("{field_name} exceeds platform limits"),
        )
    })
}

pub(super) fn scaled_i64_to_f64(value: i64) -> Result<f64, HbpParseError> {
    value.to_string().parse::<f64>().map_err(|_| {
        format_violation(
            HbpFormatErrorCode::HbpE013,
            "failed converting scaled i64 payload value to f64",
        )
    })
}

pub(super) fn key_in_year_table(entry: &DirectoryEntry, years: &[YearEntry]) -> bool {
    years.iter().any(|year| {
        entry.sim_year_index == year.sim_year_index
            && entry.calendar_year == year.calendar_year
            && entry.julian_day >= year.first_julian_day
            && entry.julian_day <= year.last_julian_day
    })
}

pub(super) fn validate_year_table(
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
