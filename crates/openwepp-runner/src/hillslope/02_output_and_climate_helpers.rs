#[derive(Clone, Copy)]
struct HbpEventFixtureInput {
    hillslope_id: u32,
    nofe: u16,
    julian_day: u16,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    sediment_concentration_kg_m3: f64,
    particle_flow_fraction: f64,
    particle_diameter_m: f64,
}

#[derive(Clone, Copy)]
struct HbpEventPayloadInput {
    nofe: u16,
    sim_year_index: u32,
    calendar_year: i32,
    julian_day: u16,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    sediment_concentration_kg_m3: f64,
    particle_flow_fraction: f64,
}

#[derive(Clone, Copy)]
struct HbpHeaderInput {
    schema_major: u16,
    schema_minor: u16,
    hillslope_id: u32,
    nofe: u16,
    nyear: u32,
    begin_year: i32,
    julian_day: u16,
    particle_diameter_m: f64,
}

const WB13_DEEP_PERCOLATION_ROUNDOFF_TOLERANCE_M: f64 = 1.0e-11;

#[derive(Clone, Copy)]
struct Wb13OfePublicationContext<'a> {
    simulation_year: i32,
    sim_day_index: usize,
    calendar_day: &'a ClimateDayProjection,
    ofe_id: u16,
    upstream_runon_m: f64,
    routed_runoff_m: Option<f64>,
    runoff_geometry: Option<Wb13RunoffPublicationGeometry>,
}

#[derive(Clone, Copy, Debug)]
struct Wb13RunoffPublicationGeometry {
    ofe_length_m: f64,
    cumulative_length_m: f64,
}

impl Wb13RunoffPublicationGeometry {
    fn new(ofe_length_m: f64, cumulative_length_m: f64) -> Result<Self, HillslopeCliError> {
        if !ofe_length_m.is_finite() || ofe_length_m <= 0.0 {
            return Err(wb13_simout_failure(format!(
                "OFE runoff-publication length must be finite and > 0.0, observed {ofe_length_m}"
            )));
        }
        if !cumulative_length_m.is_finite() || cumulative_length_m < ofe_length_m {
            return Err(wb13_simout_failure(format!(
                "cumulative runoff-publication length must be finite and >= OFE length, observed cumulative={cumulative_length_m}, ofe={ofe_length_m}"
            )));
        }
        Ok(Self {
            ofe_length_m,
            cumulative_length_m,
        })
    }
}

fn build_hbp_output(
    output_pass: &Path,
    wb13_rows: &[SimulationOwnedWb13Row],
    runtime_surface: &HillslopeWritebackSurface,
    contributor_ofe_count: usize,
) -> Result<Vec<u8>, HillslopeCliError> {
    if wb13_rows.is_empty() {
        return Err(wb13_simout_failure(
            "WB13 surface emission requires at least one executed-day row",
        ));
    }

    let hillslope_id = parse_hillslope_id_from_output_pass_path(output_pass)?;
    let nofe = u16::try_from(contributor_ofe_count).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} contributor_ofe_count out of u16 range for HBP emission: {contributor_ofe_count}"
            ),
        }
    })?;
    if nofe == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} contributor_ofe_count must be >= 1 for HBP emission"
            ),
        });
    }

    let latest_row = wb13_rows
        .last()
        .ok_or_else(|| wb13_simout_failure("missing latest executed-day row for HBP emission"))?;

    let peak_runoff_m3_s = optional_non_negative_runtime_scalar(runtime_surface, "peakro", 0.0)?;
    let duration_seconds = optional_non_negative_runtime_scalar(runtime_surface, "watdur", 0.0)?;
    let total_detachment_kg =
        optional_non_negative_runtime_scalar(runtime_surface, "total_detachment_kg", 0.0)?;
    let total_deposition_kg =
        optional_non_negative_runtime_scalar(runtime_surface, "total_deposition_kg", 0.0)?;
    let sediment_concentration_kg_m3 = optional_non_negative_runtime_scalar(
        runtime_surface,
        "sediment_concentration_kg_m3_0001",
        0.0,
    )?;
    let particle_flow_fraction = 1.0;

    build_schema1_hbp_event_fixture(HbpEventFixtureInput {
        hillslope_id,
        nofe,
        julian_day: latest_row.wb13_row.julian_day,
        peak_runoff_m3_s,
        duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
        sediment_concentration_kg_m3,
        particle_flow_fraction,
        particle_diameter_m: HBP_DEFAULT_PARTICLE_DIAMETER_M,
    })
}

fn parse_hillslope_id_from_output_pass_path(path: &Path) -> Result<u32, HillslopeCliError> {
    let file_name = file_name_string(path);
    let stem = file_name
        .strip_suffix(".hbp")
        .or_else(|| file_name.strip_suffix(".HBP"))
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} outputs.pass must use .hbp extension, observed {}",
                path.display()
            ),
        })?;
    let Some(id_text) = stem.strip_prefix('H').or_else(|| stem.strip_prefix('h')) else {
        return Ok(1);
    };
    if id_text.is_empty() || !id_text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Ok(1);
    }

    let hillslope_id =
        id_text
            .parse::<u32>()
            .map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass",
                detail: format!(
                    "{SIMOUT_GUARD_ID} outputs.pass hillslope id is not a valid u32: {id_text}"
                ),
            })?;
    if hillslope_id == 0 {
        return Ok(1);
    }

    Ok(hillslope_id)
}

fn optional_non_negative_runtime_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
    default_value: f64,
) -> Result<f64, HillslopeCliError> {
    let value = runtime_surface_symbol_value(runtime_surface, symbol).unwrap_or(default_value);
    if !value.is_finite() || value < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} runtime symbol {symbol} must be finite and >= 0.0 for HBP emission, observed {value}"
            ),
        });
    }
    Ok(value)
}

fn build_schema1_hbp_event_fixture(
    input: HbpEventFixtureInput,
) -> Result<Vec<u8>, HillslopeCliError> {
    let mut file = append_hbp_common_prefix(HbpHeaderInput {
        schema_major: HBP_SUPPORTED_MAJOR_V1,
        schema_minor: 0,
        hillslope_id: input.hillslope_id,
        nofe: input.nofe,
        nyear: 1,
        begin_year: HBP_DEFAULT_CALENDAR_YEAR,
        julian_day: input.julian_day,
        particle_diameter_m: input.particle_diameter_m,
    })?;
    let payload = build_hbp_event_payload(HbpEventPayloadInput {
        nofe: input.nofe,
        sim_year_index: 1,
        calendar_year: HBP_DEFAULT_CALENDAR_YEAR,
        julian_day: input.julian_day,
        peak_runoff_m3_s: input.peak_runoff_m3_s,
        duration_seconds: input.duration_seconds,
        total_detachment_kg: input.total_detachment_kg,
        total_deposition_kg: input.total_deposition_kg,
        sediment_concentration_kg_m3: input.sediment_concentration_kg_m3,
        particle_flow_fraction: input.particle_flow_fraction,
    })?;
    let payload_crc = crc32c(&payload);

    let directory_start = file.len();
    let directory_len = 4 + 27;
    let payload_offset = directory_start + directory_len;
    let payload_offset_u64 =
        u64::try_from(payload_offset).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!("{SIMOUT_GUARD_ID} HBP payload offset exceeds u64: {payload_offset}"),
        })?;
    let payload_len_u32 =
        u32::try_from(payload.len()).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP payload length exceeds u32: {}",
                payload.len()
            ),
        })?;
    let mut directory = Vec::new();
    put_u32(&mut directory, 1);
    put_u32(&mut directory, 1);
    put_i32(&mut directory, HBP_DEFAULT_CALENDAR_YEAR);
    put_u16(&mut directory, input.julian_day);
    put_u8(&mut directory, 2);
    put_u64(&mut directory, payload_offset_u64);
    put_u32(&mut directory, payload_len_u32);
    put_u32(&mut directory, payload_crc);

    file.extend_from_slice(&directory);
    file.extend_from_slice(&payload);

    let directory_crc = crc32c(&directory);
    put_u32(&mut file, directory_crc);
    let file_crc_pos = file.len();
    put_u32(&mut file, 0);
    put_u32(&mut file, 1);
    file.extend_from_slice(HBP_FOOTER_MAGIC);
    let file_crc = crc32c(&file);
    put_u32_at(&mut file, file_crc_pos, file_crc);
    Ok(file)
}

fn build_hbp_event_payload(input: HbpEventPayloadInput) -> Result<Vec<u8>, HillslopeCliError> {
    let nofe = u32::from(input.nofe);
    let max_layers = 1u32;

    let mut payload = Vec::new();
    put_u32(&mut payload, input.sim_year_index);
    put_i32(&mut payload, input.calendar_year);
    put_u16(&mut payload, input.julian_day);
    put_u8(&mut payload, 2);
    put_u16(&mut payload, 0);
    put_u16(
        &mut payload,
        u16::try_from(HBP_REQUIRED_STATE_IDS.len()).map_err(|_| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass",
                detail: format!(
                    "{SIMOUT_GUARD_ID} HBP state registry length exceeds u16: {}",
                    HBP_REQUIRED_STATE_IDS.len()
                ),
            }
        })?,
    );
    put_f64(&mut payload, input.duration_seconds);
    put_f64(&mut payload, 0.5);
    put_f64(&mut payload, 0.8);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_f64(&mut payload, input.peak_runoff_m3_s);
    put_i64(&mut payload, scaled_i64(input.total_detachment_kg)?);
    put_i64(&mut payload, scaled_i64(input.total_deposition_kg)?);
    put_u32(&mut payload, 1);
    put_f64(&mut payload, input.sediment_concentration_kg_m3);
    put_u32(&mut payload, 1);
    put_f64(&mut payload, input.particle_flow_fraction);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);

    for state_id in HBP_REQUIRED_STATE_IDS {
        payload.extend_from_slice(&build_hbp_state_entry(*state_id, nofe, max_layers)?);
    }

    Ok(payload)
}

fn append_hbp_common_prefix(input: HbpHeaderInput) -> Result<Vec<u8>, HillslopeCliError> {
    let mut file = Vec::new();

    let mut header = Vec::new();
    header.extend_from_slice(HBP_MAGIC);
    put_u16(&mut header, input.schema_major);
    put_u16(&mut header, input.schema_minor);
    put_u8(&mut header, 1);
    let header_bytes_pos = header.len();
    put_u32(&mut header, 0);
    header.extend_from_slice(&[0u8; 32]);
    put_u8(&mut header, 1);
    put_string(&mut header, "openwepp-hillslope-cli")?;
    put_string(&mut header, "hs-cli")?;
    put_string(&mut header, "2026-05-29T00:00:00Z")?;
    put_string(&mut header, "metric-v1")?;
    header.extend_from_slice(&[0u8; 32]);
    let header_crc_pos = header.len();
    put_u32(&mut header, 0);
    let header_bytes =
        u32::try_from(header.len()).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP header byte count exceeds u32: {}",
                header.len()
            ),
        })?;
    put_u32_at(&mut header, header_bytes_pos, header_bytes);
    let header_crc = crc32c(&header);
    put_u32_at(&mut header, header_crc_pos, header_crc);
    file.extend_from_slice(&header);

    let npart = 1u16;
    let max_layers = 1u16;

    put_u32(&mut file, input.hillslope_id);
    put_u32(&mut file, input.nyear);
    put_i32(&mut file, input.begin_year);
    put_u16(&mut file, npart);
    put_u16(&mut file, input.nofe);
    put_u16(&mut file, max_layers);
    put_string(&mut file, "gregorian")?;
    put_u16(&mut file, 1);
    put_u8(&mut file, 1);

    put_string(&mut file, "p1.cli")?;
    put_i64(&mut file, 0);
    put_u32(&mut file, u32::from(npart));
    put_f64(&mut file, input.particle_diameter_m);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);

    put_u32(&mut file, input.nyear);
    put_u32(&mut file, 1);
    put_i32(&mut file, input.begin_year);
    put_u16(&mut file, 1);
    put_u16(&mut file, input.julian_day);
    put_u16(&mut file, input.julian_day);
    put_u8(&mut file, 0);

    put_u32(
        &mut file,
        u32::try_from(HBP_REQUIRED_STATE_IDS.len()).map_err(|_| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass",
                detail: format!(
                    "{SIMOUT_GUARD_ID} HBP state registry count exceeds u32: {}",
                    HBP_REQUIRED_STATE_IDS.len()
                ),
            }
        })?,
    );
    for state_id in HBP_REQUIRED_STATE_IDS {
        let (required_flag, representation_class, unit_class, rank, dims_kind) =
            expected_hbp_state_schema(*state_id).ok_or_else(|| {
                HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "outputs.pass",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} missing HBP state schema for required state {state_id}"
                    ),
                }
            })?;
        put_u16(&mut file, *state_id);
        put_u8(&mut file, required_flag);
        put_u8(&mut file, representation_class);
        put_u16(&mut file, unit_class);
        put_u8(&mut file, rank);
        put_u8(&mut file, dims_kind);
        put_string(&mut file, &format!("state_{state_id}"))?;
    }

    Ok(file)
}

fn expected_hbp_state_schema(state_id: u16) -> Option<(u8, u8, u16, u8, u8)> {
    match state_id {
        1 => Some((1, 1, 1, 1, HBP_DIM_NOFE)),
        2 | 3 | 4 | 5 | 100 | 101 | 102 | 210 | 900 | 901 => {
            Some((1, 1, 2, 2, HBP_DIM_NOFE_LAYERS))
        }
        6 | 7 => Some((1, 2, 3, 2, HBP_DIM_NOFE_LAYERS)),
        103 | 104 | 200 | 202 | 203 | 204 | 205 | 206 | 207 | 208 | 209 => {
            Some((1, 1, 2, 1, HBP_DIM_NOFE))
        }
        201 => Some((1, 2, 4, 1, HBP_DIM_NOFE)),
        300 => Some((1, 1, 5, 0, HBP_DIM_SCALAR)),
        _ => None,
    }
}

fn build_hbp_state_entry(
    state_id: u16,
    nofe: u32,
    max_layers: u32,
) -> Result<Vec<u8>, HillslopeCliError> {
    let (required_flag, representation_class, unit_class, rank, dims_kind) =
        expected_hbp_state_schema(state_id).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass",
                detail: format!(
                    "{SIMOUT_GUARD_ID} missing HBP state schema for required state {state_id}"
                ),
            }
        })?;
    let dims = hbp_state_dims(dims_kind, nofe, max_layers);
    assert_eq!(dims.len(), usize::from(rank));

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
    put_u32(
        &mut out,
        u32::try_from(entry.len()).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP state entry byte count exceeds u32 for state {state_id}: {}",
                entry.len()
            ),
        })?,
    );
    out.extend_from_slice(&entry);
    Ok(out)
}

fn hbp_state_dims(dims_kind: u8, nofe: u32, max_layers: u32) -> Vec<u32> {
    match dims_kind {
        HBP_DIM_SCALAR => vec![],
        HBP_DIM_NOFE => vec![nofe],
        HBP_DIM_NOFE_LAYERS => vec![nofe, max_layers],
        _ => panic!("unknown dims_kind {dims_kind}"),
    }
}

fn scaled_i64(value: f64) -> Result<i64, HillslopeCliError> {
    let scaled = value * HBP_SCALE_INV_I64;
    if !scaled.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!("{SIMOUT_GUARD_ID} HBP scaled integer is not finite for value {value}"),
        });
    }
    let rounded = scaled.round();
    if !(HBP_I64_MIN_F64..=HBP_I64_MAX_F64).contains(&rounded) {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!("{SIMOUT_GUARD_ID} HBP scaled integer overflow for value {value}"),
        });
    }
    let rounded_text = format!("{rounded:.0}");
    rounded_text
        .parse::<i64>()
        .map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!("{SIMOUT_GUARD_ID} HBP scaled integer parse failure for value {value}"),
        })
}

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

fn put_string(buf: &mut Vec<u8>, value: &str) -> Result<(), HillslopeCliError> {
    put_u32(
        buf,
        u32::try_from(value.len()).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP string length exceeds u32: {}",
                value.len()
            ),
        })?,
    );
    buf.extend_from_slice(value.as_bytes());
    Ok(())
}

fn put_u32_at(buf: &mut [u8], offset: usize, value: u32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn crc32c(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for value in data {
        crc ^= u32::from(*value);
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

fn build_hillslope_wat_rows(
    wb13_rows: &[SimulationOwnedWb13Row],
) -> Result<Vec<HillslopeWatRow>, HillslopeCliError> {
    let mut rows = Vec::with_capacity(wb13_rows.len());
    for wb13_row in wb13_rows {
        rows.push(build_hillslope_wat_row(wb13_row)?);
    }
    Ok(rows)
}

fn build_hillslope_wat_row(
    wb13_row: &SimulationOwnedWb13Row,
) -> Result<HillslopeWatRow, HillslopeCliError> {
    if wb13_row.sim_day_index <= 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!(
                "{SIMOUT_GUARD_ID} sim_day_index must be >= 1, observed {}",
                wb13_row.sim_day_index
            ),
        });
    }
    let year = i16::try_from(wb13_row.wb13_row.year).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!(
                "{SIMOUT_GUARD_ID} year out of i16 range: {}",
                wb13_row.wb13_row.year
            ),
        }
    })?;
    let julian = i16::try_from(wb13_row.wb13_row.julian_day).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!(
                "{SIMOUT_GUARD_ID} julian out of i16 range: {}",
                wb13_row.wb13_row.julian_day
            ),
        }
    })?;
    let ofe = i16::try_from(wb13_row.wb13_row.ofe).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!(
                "{SIMOUT_GUARD_ID} OFE out of i16 range: {}",
                wb13_row.wb13_row.ofe
            ),
        }
    })?;

    Ok(HillslopeWatRow {
        wepp_id: 1,
        ofe_id: ofe,
        year,
        sim_day_index: wb13_row.sim_day_index,
        julian,
        month: wb13_row.month,
        day_of_month: wb13_row.day_of_month,
        water_year: wb13_row.water_year,
        ofe,
        p: wb13_row.wb13_row.p,
        rm: wb13_row.wb13_row.rm,
        q: wb13_row.wb13_row.q,
        ep: wb13_row.wb13_row.ep,
        es: wb13_row.wb13_row.es,
        er: wb13_row.wb13_row.er,
        dp: wb13_row.wb13_row.dp,
        up_strm_q: wb13_row.wb13_row.upstrmq,
        sub_r_in: wb13_row.wb13_row.subrin,
        latqcc: wb13_row.wb13_row.latqcc,
        total_soil_water: wb13_row.wb13_row.total_soil,
        frozwt: wb13_row.wb13_row.frozwt,
        frdp: wb13_row.frdp_mm,
        snow_water: wb13_row.wb13_row.snow_water,
        qofe: wb13_row.wb13_row.qofe,
        tile: wb13_row.wb13_row.tile,
        irr: wb13_row.wb13_row.irr,
        area: wb13_row.wb13_row.area,
        soil_water_total: Some(wb13_row.wb13_row.soil_water_total),
        profile_depth: Some(wb13_row.wb13_row.profile_depth),
        profile_porosity_cap: Some(wb13_row.wb13_row.profile_porosity_cap),
        profile_fc_store: Some(wb13_row.wb13_row.profile_fc_store),
        profile_wp_store: Some(wb13_row.wb13_row.profile_wp_store),
        interception: Some(wb13_row.interception_mm),
        interception_storage: None,
    })
}

fn build_hillslope_pass_row(
    wepp_id: i32,
    wb13_row: &SimulationOwnedWb13Row,
) -> Result<HillslopePassRow, HillslopeCliError> {
    if wepp_id <= 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass_parquet",
            detail: format!("{SIMOUT_GUARD_ID} wepp_id must be > 0, observed {wepp_id}"),
        });
    }
    if wb13_row.sim_day_index <= 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass_parquet",
            detail: format!(
                "{SIMOUT_GUARD_ID} sim_day_index must be >= 1, observed {}",
                wb13_row.sim_day_index
            ),
        });
    }
    let area_m2 = wb13_row.wb13_row.area;
    if !area_m2.is_finite() || area_m2 <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass_parquet",
            detail: format!(
                "{SIMOUT_GUARD_ID} row area must be finite and > 0.0, observed {area_m2}"
            ),
        });
    }

    Ok(HillslopePassRow {
        wepp_id,
        year: i16::try_from(wb13_row.wb13_row.year).map_err(|_| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass_parquet",
                detail: format!(
                    "{SIMOUT_GUARD_ID} year out of i16 range: {}",
                    wb13_row.wb13_row.year
                ),
            }
        })?,
        sim_day_index: wb13_row.sim_day_index,
        julian: i16::try_from(wb13_row.wb13_row.julian_day).map_err(|_| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass_parquet",
                detail: format!(
                    "{SIMOUT_GUARD_ID} julian out of i16 range: {}",
                    wb13_row.wb13_row.julian_day
                ),
            }
        })?,
        month: wb13_row.month,
        day_of_month: wb13_row.day_of_month,
        water_year: wb13_row.water_year,
        runvol_m3: wb13_row.wb13_row.q * area_m2 / 1_000.0,
        sbrunv_m3: wb13_row.wb13_row.latqcc * area_m2 / 1_000.0,
        peakro_m3_s: 0.0,
        total_detachment_kg: 0.0,
        total_deposition_kg: 0.0,
        sediment_concentration_kg_m3: [0.0; 5],
    })
}

fn build_hillslope_pass_row_from_outlet_delivery(
    wepp_id: i32,
    outlet: &InternalPerOfeWb13Record,
) -> Result<HillslopePassRow, HillslopeCliError> {
    if !outlet.area_m2.is_finite() || outlet.area_m2 <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass_parquet",
            detail: format!(
                "{SIMOUT_GUARD_ID} outlet OFE area must be finite and > 0.0, observed {}",
                outlet.area_m2
            ),
        });
    }

    let mut row = build_hillslope_pass_row(wepp_id, &outlet.row)?;
    row.runvol_m3 = outlet.row.wb13_row.qofe * outlet.row.wb13_row.area / 1_000.0;
    row.sbrunv_m3 = outlet.row.wb13_row.latqcc * outlet.area_m2 / 1_000.0;
    Ok(row)
}

fn derive_mofe04_publication_area_from_slope(
    slope: &SlopeProfile,
) -> Result<f64, HillslopeCliError> {
    if slope.ofes.is_empty() {
        return Err(wb13_simout_failure(
            "slope profile contains no OFE entries for Area derivation",
        ));
    }

    let mut area = 0.0_f64;
    for (ofe_position, ofe) in slope.ofes.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        if !ofe.fwidth.is_finite() || ofe.fwidth <= 0.0 {
            return Err(wb13_simout_failure(format!(
                "OFE {ofe_index} fwidth must be > 0.0, observed {}",
                ofe.fwidth
            )));
        }
        if !ofe.slplen.is_finite() || ofe.slplen <= 0.0 {
            return Err(wb13_simout_failure(format!(
                "OFE {ofe_index} slplen must be > 0.0, observed {}",
                ofe.slplen
            )));
        }

        area += ofe.fwidth * ofe.slplen;
    }

    if !area.is_finite() || area <= 0.0 {
        return Err(wb13_simout_failure(format!(
            "aggregate OFE Area must be > 0.0, observed {area}"
        )));
    }

    Ok(area)
}

fn derive_profile_fc_store_from_authoritative_layers(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    let nsl_symbol = if runtime_surface_symbol_value(runtime_surface, "wb11_nsl").is_some() {
        "wb11_nsl"
    } else {
        "nsl"
    };
    let nsl = scalar_to_usize(
        nsl_symbol,
        require_runtime_surface_scalar(runtime_surface, nsl_symbol)?,
    )?;
    if nsl == 0 {
        return Err(wb13_simout_failure(
            "nsl must be >= 1 for ProfileFCStore layer aggregation",
        ));
    }

    let mut profile_fc_store_m = 0.0_f64;
    for layer_index in 1..=nsl {
        let preferred_thetfc_symbol = format!("wb19_thetfc_{layer_index:04}");
        let legacy_thetfc_symbol = format!("thetfc_{layer_index:04}");
        let thetfc_symbol =
            if runtime_surface_symbol_value(runtime_surface, &preferred_thetfc_symbol).is_some() {
                preferred_thetfc_symbol
            } else {
                legacy_thetfc_symbol
            };
        let preferred_dg_symbol = format!("wb19_dg_{layer_index:04}");
        let legacy_dg_symbol = format!("dg_{layer_index:04}");
        let dg_symbol =
            if runtime_surface_symbol_value(runtime_surface, &preferred_dg_symbol).is_some() {
                preferred_dg_symbol
            } else {
                legacy_dg_symbol
            };
        let thetfc = require_runtime_surface_scalar(runtime_surface, &thetfc_symbol)?;
        let dg = require_runtime_surface_scalar(runtime_surface, &dg_symbol)?;
        if thetfc < 0.0 {
            return Err(wb13_simout_failure(format!(
                "{thetfc_symbol} must be >= 0.0, observed {thetfc}"
            )));
        }
        if dg <= 0.0 {
            return Err(wb13_simout_failure(format!(
                "{dg_symbol} must be > 0.0, observed {dg}"
            )));
        }
        profile_fc_store_m += thetfc * dg;
    }

    if !profile_fc_store_m.is_finite() || profile_fc_store_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "ProfileFCStore layer aggregation must be finite and >= 0.0, observed {profile_fc_store_m}"
        )));
    }
    let profile_fc_tail_mm =
        require_runtime_surface_scalar(runtime_surface, "wb13_profile_fc_tail_mm")?;
    if !profile_fc_tail_mm.is_finite() {
        return Err(wb13_simout_failure(format!(
            "wb13_profile_fc_tail_mm must be finite, observed {profile_fc_tail_mm}"
        )));
    }
    if profile_fc_tail_mm < 0.0 {
        return Err(wb13_simout_failure(format!(
            "wb13_profile_fc_tail_mm must be >= 0.0, observed {profile_fc_tail_mm}"
        )));
    }

    let profile_fc_store_mm = profile_fc_store_m * 1_000.0 + profile_fc_tail_mm;
    if !profile_fc_store_mm.is_finite() || profile_fc_store_mm < 0.0 {
        return Err(wb13_simout_failure(format!(
            "ProfileFCStore combined layer+tail storage must be finite and >= 0.0, observed {profile_fc_store_mm}"
        )));
    }

    Ok(profile_fc_store_mm)
}

#[allow(clippy::too_many_lines)]
fn build_simulation_owned_wb13_row(
    runtime_surface: &HillslopeWritebackSurface,
    publication_area_m2: f64,
    simulation_year: i32,
    sim_day_index: usize,
    calendar_day: &ClimateDayProjection,
    _runtime_swe_before_m: f64,
) -> Result<SimulationOwnedWb13Row, HillslopeCliError> {
    build_simulation_owned_wb13_row_for_ofe(
        runtime_surface,
        publication_area_m2,
        Wb13OfePublicationContext {
            simulation_year,
            sim_day_index,
            calendar_day,
            ofe_id: 1,
            upstream_runon_m: 0.0,
            routed_runoff_m: None,
            runoff_geometry: None,
        },
    )
}

#[allow(clippy::too_many_lines)]
fn build_simulation_owned_wb13_row_for_ofe(
    runtime_surface: &HillslopeWritebackSurface,
    publication_area_m2: f64,
    context: Wb13OfePublicationContext<'_>,
) -> Result<SimulationOwnedWb13Row, HillslopeCliError> {
    if context.simulation_year <= 0 {
        return Err(wb13_simout_failure(format!(
            "simulation-year key must be >= 1, observed {}",
            context.simulation_year
        )));
    }
    if context.ofe_id == 0 {
        return Err(wb13_simout_failure("WB13 OFE id must be >= 1"));
    }
    if !context.upstream_runon_m.is_finite() || context.upstream_runon_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "UpStrmQ source must be finite and >= 0.0, observed {}",
            context.upstream_runon_m
        )));
    }
    if context
        .routed_runoff_m
        .is_some_and(|value| !value.is_finite() || value < 0.0)
    {
        return Err(wb13_simout_failure(format!(
            "QOFE source must be finite and >= 0.0, observed {:?}",
            context.routed_runoff_m
        )));
    }
    if context.routed_runoff_m.is_some() != context.runoff_geometry.is_some() {
        return Err(wb13_simout_failure(
            "per-OFE routed runoff publication requires both routed runoff and geometry",
        ));
    }

    let calendar_year = context.calendar_day.year;
    let month = context.calendar_day.month;
    let day_of_month = context.calendar_day.day_of_month;
    let julian_day = day_of_year(calendar_year, month, day_of_month)?;
    if julian_day != context.calendar_day.julian_day {
        return Err(wb13_simout_failure(format!(
            "calendar day projection mismatch: computed julian {julian_day} differs from projected {}",
            context.calendar_day.julian_day
        )));
    }

    let precipitation_m = require_runtime_surface_scalar(runtime_surface, "prcp")?;
    if precipitation_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "precipitation symbol prcp must be >= 0.0, observed {precipitation_m}"
        )));
    }
    let precipitation_mm = precipitation_m * 1_000.0;

    let _tmax = require_runtime_surface_scalar(runtime_surface, "tmax")?;
    let _tmin = require_runtime_surface_scalar(runtime_surface, "tmin")?;

    let profile_depth_mm =
        require_runtime_surface_scalar(runtime_surface, "wb13_profile_depth_mm")?;
    if profile_depth_mm <= 0.0 {
        return Err(wb13_simout_failure(format!(
            "wb13_profile_depth_mm must be > 0.0, observed {profile_depth_mm}"
        )));
    }
    let profile_porosity_cap =
        require_runtime_surface_scalar(runtime_surface, "wb13_profile_porosity_cap_mm")?;
    if profile_porosity_cap < 0.0 {
        return Err(wb13_simout_failure(format!(
            "wb13_profile_porosity_cap_mm must be >= 0.0, observed {profile_porosity_cap}"
        )));
    }
    let profile_fc_store_mm = derive_profile_fc_store_from_authoritative_layers(runtime_surface)?;
    let profile_wp_store_mm =
        require_runtime_surface_scalar(runtime_surface, "wb13_profile_wp_store_mm")?;
    if profile_wp_store_mm < 0.0 {
        return Err(wb13_simout_failure(format!(
            "wb13_profile_wp_store_mm must be >= 0.0, observed {profile_wp_store_mm}"
        )));
    }
    if profile_porosity_cap < profile_fc_store_mm {
        return Err(wb13_simout_failure(format!(
            "profile storage ordering invalid: ProfilePorosityCap ({profile_porosity_cap}) must be >= ProfileFCStore ({profile_fc_store_mm})"
        )));
    }
    if profile_fc_store_mm < profile_wp_store_mm {
        return Err(wb13_simout_failure(format!(
            "profile storage ordering invalid: ProfileFCStore ({profile_fc_store_mm}) must be >= ProfileWPStore ({profile_wp_store_mm})"
        )));
    }

    // SIMIMPL24 publication authority: Total-Soil must be WB11 runtime
    // aggregate lineage only (`wb11_soil_water` -> `watcon` -> `Total-Soil`).
    let wb11_soil_water_m = require_runtime_surface_scalar(runtime_surface, "wb11_soil_water")?;
    if wb11_soil_water_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "wb11_soil_water must be >= 0.0, observed {wb11_soil_water_m}"
        )));
    }
    let total_soil = wb11_soil_water_m * 1_000.0;

    let frozwt_m = require_runtime_surface_scalar(
        runtime_surface,
        "frost.runtime_frwatc_frozen_water_after_m",
    )?;
    if frozwt_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "frost.runtime_frwatc_frozen_water_after_m must be >= 0.0, observed {frozwt_m}"
        )));
    }
    let frozwt = frozwt_m * 1_000.0;

    let frdp_m = require_runtime_surface_scalar(runtime_surface, "frost.runtime_frdp_m")?;
    if frdp_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "frost.runtime_frdp_m must be >= 0.0, observed {frdp_m}"
        )));
    }
    let profile_depth_m = profile_depth_mm / 1_000.0;
    if frdp_m > profile_depth_m + 1.0e-9 {
        return Err(wb13_simout_failure(format!(
            "frost.runtime_frdp_m must be <= wb13_profile_depth_mm, observed {frdp_m} m > {profile_depth_m} m"
        )));
    }
    let frdp_mm = frdp_m * 1_000.0;

    let runtime_swe_m = require_runtime_surface_scalar(runtime_surface, "snow.runtime_swe")?;
    if runtime_swe_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "snow.runtime_swe must be >= 0.0, observed {runtime_swe_m}"
        )));
    }
    let snow_water = runtime_swe_m * 1_000.0;

    let irrigation_m = require_runtime_surface_scalar(runtime_surface, "Irr")?;
    if irrigation_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Irr must be >= 0.0, observed {irrigation_m}"
        )));
    }
    let routed_melt_m = require_runtime_flux_surface_scalar(runtime_surface, "snow.routed_melt_m")?;
    if routed_melt_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "snow.routed_melt_m must be >= 0.0, observed {routed_melt_m}"
        )));
    }
    let post_winter_rain_m =
        require_runtime_flux_surface_scalar(runtime_surface, "snow.post_winter_rain_m")?;
    if post_winter_rain_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "snow.post_winter_rain_m must be >= 0.0, observed {post_winter_rain_m}"
        )));
    }
    let rm_m = post_winter_rain_m + routed_melt_m + irrigation_m;
    if rm_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "RM source (snow.post_winter_rain_m + snow.routed_melt_m + Irr) must be >= 0.0, observed {rm_m}"
        )));
    }
    let rm = rm_m * 1_000.0;
    let irrigation_mm = irrigation_m * 1_000.0;

    let interception_i_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "I")?;
    if interception_i_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "I must be >= 0.0, observed {interception_i_m}"
        )));
    }
    let interception_mm = interception_i_m * 1_000.0;

    let q_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Q")?;
    if q_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Q must be >= 0.0, observed {q_m}"
        )));
    }
    let transpiration_ep_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Ep")?;
    if transpiration_ep_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Ep must be >= 0.0, observed {transpiration_ep_m}"
        )));
    }
    let evappm_pmet_branch =
        runtime_surface_symbol_value(runtime_surface, "wb11_et_seed_branch_evappm")
            .is_some_and(|value| value >= 0.5);
    let soil_evap_es_m_raw = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Es")?;
    if soil_evap_es_m_raw < -1.0e-12 {
        return Err(wb13_simout_failure(format!(
            "Es must be >= 0.0 within tolerance, observed {soil_evap_es_m_raw}"
        )));
    }
    let soil_evap_es_m = if soil_evap_es_m_raw < 0.0 {
        0.0
    } else {
        soil_evap_es_m_raw
    };
    let residue_evap_er_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Er")?;
    if residue_evap_er_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Er must be >= 0.0, observed {residue_evap_er_m}"
        )));
    }
    let base_dp_m = canonicalize_wb13_deep_percolation_source_m(
        "D",
        require_runtime_surface_scalar_prefer_flux(runtime_surface, "D")?,
    )?;
    let frost_watbtm_m = canonicalize_wb13_deep_percolation_source_m(
        "frost.runtime_watbtm_m",
        runtime_surface_symbol_value(runtime_surface, "frost.runtime_watbtm_m").unwrap_or(0.0),
    )?;
    let frost_watpdg_m =
        runtime_surface_symbol_value(runtime_surface, "frost.runtime_watpdg_m").unwrap_or(0.0);
    if frost_watpdg_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "frost.runtime_watpdg_m must be >= 0.0, observed {frost_watpdg_m}"
        )));
    }
    let dp_m =
        canonicalize_wb13_deep_percolation_publication_m(base_dp_m + frost_watbtm_m)?;
    let latqcc_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "q")?;
    if latqcc_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "q must be >= 0.0, observed {latqcc_m}"
        )));
    }
    let tile_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Qdd")?;
    if tile_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Qdd must be >= 0.0, observed {tile_m}"
        )));
    }
    let qd_source_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Qd")?;
    if qd_source_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Qd must be >= 0.0, observed {qd_source_m}"
        )));
    }
    let sub_r_in_m = runtime_surface_symbol_value(runtime_surface, "SubRIn").unwrap_or(0.0);
    if sub_r_in_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "SubRIn must be >= 0.0, observed {sub_r_in_m}"
        )));
    }
    let physical_q = q_m * 1_000.0;
    let (q, qofe) = if let (Some(routed_runoff_m), Some(geometry)) =
        (context.routed_runoff_m, context.runoff_geometry)
    {
        let efflen_m = require_runtime_surface_scalar(runtime_surface, "efflen")?;
        if !efflen_m.is_finite() || efflen_m <= 0.0 {
            return Err(wb13_simout_failure(format!(
                "efflen must be finite and > 0.0 for per-OFE runoff publication, observed {efflen_m}"
            )));
        }
        if efflen_m > geometry.cumulative_length_m + 1.0e-9 {
            return Err(wb13_simout_failure(format!(
                "efflen must not exceed cumulative runoff-publication length for OFE {}, observed efflen={} cumulative={}",
                context.ofe_id, efflen_m, geometry.cumulative_length_m
            )));
        }
        (
            routed_runoff_m * 1_000.0 * efflen_m / geometry.cumulative_length_m,
            routed_runoff_m * 1_000.0 * efflen_m / geometry.ofe_length_m,
        )
    } else {
        (physical_q, physical_q)
    };
    let ep = transpiration_ep_m * 1_000.0;
    let es = soil_evap_es_m * 1_000.0;
    let er = residue_evap_er_m * 1_000.0;
    let dp = dp_m * 1_000.0;
    let latqcc = latqcc_m * 1_000.0;
    let tile = tile_m * 1_000.0;
    let qd = qd_source_m * 1_000.0;
    let sub_r_in = sub_r_in_m * 1_000.0;
    if (qd - (latqcc + tile)).abs() > 1.0e-6 {
        return Err(wb13_simout_failure(format!(
            "Qd coupling closure violated: Qd ({qd}) must equal latqcc + Tile ({})",
            latqcc + tile
        )));
    }
    let area = publication_area_m2;
    let soil_water_total = total_soil;

    let row_surface = SummaryScalarSurface::from_pairs([
        ("P", precipitation_mm),
        ("RM", rm),
        ("Q", q),
        ("Ep", ep),
        ("Es", es),
        ("Er", er),
        ("Dp", dp),
        ("UpStrmQ", context.upstream_runon_m * 1_000.0),
        ("SubRIn", sub_r_in),
        ("latqcc", latqcc),
        ("Total-Soil", total_soil),
        ("frozwt", frozwt),
        ("frdp", frdp_mm),
        ("Snow-Water", snow_water),
        ("QOFE", qofe),
        ("Tile", tile),
        ("Irr", irrigation_mm),
        ("Area", area),
        (
            "wb11_et_seed_branch_evappm",
            if evappm_pmet_branch { 1.0 } else { 0.0 },
        ),
        (
            WB13_PER_OFE_PUBLICATION_POLICY_SYMBOL,
            if context.routed_runoff_m.is_some() {
                1.0
            } else {
                0.0
            },
        ),
        ("SoilWaterTotal", soil_water_total),
        ("ProfileDepth", profile_depth_mm),
        ("ProfilePorosityCap", profile_porosity_cap),
        ("ProfileFCStore", profile_fc_store_mm),
        ("ProfileWPStore", profile_wp_store_mm),
    ])
    .map_err(|error| {
        wb13_simout_failure(format!("failed building WB13 scalar surface: {error}"))
    })?;

    let wb13_row = Wb13DailyWaterBalanceRow::from_surface(
        context.ofe_id,
        julian_day,
        context.simulation_year,
        &row_surface,
    )
    .map_err(|error| wb13_simout_failure(format!("failed building WB13 row: {error}")))?;

    let month_i8 = i8::try_from(month).map_err(|_| {
        wb13_simout_failure(format!(
            "month out of i8 range for WB13 publication: {month}"
        ))
    })?;
    let day_of_month_i8 = i8::try_from(day_of_month).map_err(|_| {
        wb13_simout_failure(format!(
            "day-of-month out of i8 range for WB13 publication: {day_of_month}"
        ))
    })?;
    let water_year = if month >= 10 {
        calendar_year + 1
    } else {
        calendar_year
    };
    let water_year_i16 = i16::try_from(water_year).map_err(|_| {
        wb13_simout_failure(format!(
            "water-year out of i16 range for WB13 publication: {water_year}"
        ))
    })?;
    let sim_day_index_i32 = i32::try_from(context.sim_day_index).map_err(|_| {
        wb13_simout_failure(format!(
            "sim_day_index out of i32 range for WB13 publication: {}",
            context.sim_day_index
        ))
    })?;

    Ok(SimulationOwnedWb13Row {
        wb13_row,
        interception_mm,
        frdp_mm,
        month: month_i8,
        day_of_month: day_of_month_i8,
        water_year: water_year_i16,
        sim_day_index: sim_day_index_i32,
    })
}

fn runtime_surface_symbol_value(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Option<f64> {
    let key = BoundarySymbol::from(symbol);
    runtime_surface
        .state_surface
        .get(&key)
        .map(|value| value.as_f64())
        .or_else(|| {
            runtime_surface
                .flux_surface
                .get(&key)
                .map(|value| value.as_f64())
        })
}

fn runtime_surface_symbol_value_prefer_flux(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Option<f64> {
    let key = BoundarySymbol::from(symbol);
    runtime_surface
        .flux_surface
        .get(&key)
        .map(|value| value.as_f64())
        .or_else(|| {
            runtime_surface
                .state_surface
                .get(&key)
                .map(|value| value.as_f64())
        })
}

fn runtime_surface_flux_symbol_value(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Option<f64> {
    runtime_surface
        .flux_surface
        .get(&BoundarySymbol::from(symbol))
        .map(|value| value.as_f64())
}

fn runtime_surface_ofe_count(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<usize, HillslopeCliError> {
    if let Some(contributor_ofe_count) =
        runtime_surface_symbol_value(runtime_surface, "mofe.static_lane.contributor_ofe_count")
    {
        let count = scalar_to_usize(
            "mofe.static_lane.contributor_ofe_count",
            contributor_ofe_count,
        )?;
        if count == 0 {
            return Err(mofe_hourly_carry_failure(
                "mofe.static_lane.contributor_ofe_count must be >= 1 for MOFE hourly carry seeding",
            ));
        }
        return Ok(count);
    }
    if let Some(nelem) = runtime_surface_symbol_value(runtime_surface, "nelem") {
        let count = scalar_to_usize("nelem", nelem)?;
        if count == 0 {
            return Err(mofe_hourly_carry_failure(
                "nelem must be >= 1 for MOFE hourly carry seeding",
            ));
        }
        return Ok(count);
    }
    if let Some(nwsofe) = runtime_surface_symbol_value(runtime_surface, "nwsofe") {
        let count = scalar_to_usize("nwsofe", nwsofe)?;
        if count == 0 {
            return Err(mofe_hourly_carry_failure(
                "nwsofe must be >= 1 for MOFE hourly carry seeding",
            ));
        }
        return Ok(count);
    }
    Ok(1)
}

fn seed_mofe_hourly_carry_runtime_surface_inputs(
    runtime_surface: &mut HillslopeWritebackSurface,
    active: bool,
) -> Result<(), HillslopeCliError> {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(MOFE_HOURLY_CARRY_ARRAYS_ENABLED_SYMBOL),
        BoundaryValue::scalar(if active { 1.0 } else { 0.0 }),
    );
    if active {
        runtime_surface
            .state_surface
            .entry(BoundarySymbol::from(MOFE_HOURLY_UPSTREAM_AREA_RATIO_SYMBOL))
            .or_insert_with(|| BoundaryValue::scalar(1.0));
    }

    for root in MOFE_HOURLY_REQUIRED_ARRAYS {
        for hour in 1..=MOFE_HOURLY_CARRY_ARRAY_COUNT {
            let symbol = mofe_hourly_carry_hour_symbol(root, hour);
            if let Some(existing) = runtime_surface_symbol_value(runtime_surface, &symbol) {
                require_mofe_hourly_carry_non_negative(existing, &symbol)?;
            } else if active {
                runtime_surface
                    .state_surface
                    .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(0.0));
            }
        }
    }
    Ok(())
}

fn mofe_hourly_carry_hour_symbol(root: &str, hour: usize) -> String {
    format!("{root}_{hour:04}")
}

fn require_mofe_hourly_carry_non_negative(
    value: f64,
    symbol: &str,
) -> Result<(), HillslopeCliError> {
    if !value.is_finite() || value < 0.0 {
        return Err(mofe_hourly_carry_failure(format!(
            "{symbol} must be finite and >= 0.0, observed {value}"
        )));
    }
    Ok(())
}

fn parse_mofe03_binary_flag(symbol: &str, value: f64) -> Result<bool, HillslopeCliError> {
    if !value.is_finite() {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} must be finite, observed {value}"
        )));
    }
    if value.abs() <= MOFE03_WAVE2_ENABLE_TOLERANCE {
        return Ok(false);
    }
    if (value - 1.0).abs() <= MOFE03_WAVE2_ENABLE_TOLERANCE {
        return Ok(true);
    }
    Err(mofe03_wave2_seed_failure(format!(
        "{symbol} must be binary 0|1, observed {value}"
    )))
}

fn require_mofe03_runtime_surface_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    let Some(value) = runtime_surface_symbol_value(runtime_surface, symbol) else {
        return Err(mofe03_wave2_seed_failure(format!(
            "missing required runtime symbol {symbol}"
        )));
    };
    if !value.is_finite() {
        return Err(mofe03_wave2_seed_failure(format!(
            "runtime symbol {symbol} is non-finite ({value})"
        )));
    }
    Ok(value)
}

fn require_runtime_surface_scalar_prefer_flux(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    let value = runtime_surface_symbol_value_prefer_flux(runtime_surface, symbol)
        .ok_or_else(|| wb13_simout_failure(format!("missing required runtime symbol {symbol}")))?;
    if !value.is_finite() {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} must be finite, observed {value}"
        )));
    }
    Ok(value)
}

fn canonicalize_wb13_deep_percolation_source_m(
    symbol: &str,
    value: f64,
) -> Result<f64, HillslopeCliError> {
    if !value.is_finite() {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} must be finite, observed {value}"
        )));
    }
    if value < 0.0 {
        return Err(wb13_simout_failure(format!(
            "{symbol} must be >= 0.0, observed {value}"
        )));
    }
    Ok(canonicalize_wb13_deep_percolation_roundoff_m(value))
}

fn canonicalize_wb13_deep_percolation_publication_m(
    value: f64,
) -> Result<f64, HillslopeCliError> {
    if !value.is_finite() {
        return Err(wb13_simout_failure(format!(
            "Dp source sum must be finite, observed {value}"
        )));
    }
    if value < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Dp source sum must be >= 0.0, observed {value}"
        )));
    }
    Ok(canonicalize_wb13_deep_percolation_roundoff_m(value))
}

fn canonicalize_wb13_deep_percolation_roundoff_m(value: f64) -> f64 {
    if value <= WB13_DEEP_PERCOLATION_ROUNDOFF_TOLERANCE_M {
        0.0
    } else {
        value
    }
}

fn require_runtime_flux_surface_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    let key = BoundarySymbol::from(symbol);
    let value = runtime_surface
        .flux_surface
        .get(&key)
        .map(|value| value.as_f64())
        .ok_or_else(|| {
            wb13_simout_failure(format!("missing required runtime flux symbol {symbol}"))
        })?;
    if !value.is_finite() {
        return Err(wb13_simout_failure(format!(
            "runtime flux symbol {symbol} must be finite, observed {value}"
        )));
    }
    Ok(value)
}

fn require_mofe03_non_negative_seed_scalar(
    value: f64,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    if !value.is_finite() {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} seed value must be finite, observed {value}"
        )));
    }
    if value < 0.0 {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} seed value must be >= 0.0, observed {value}"
        )));
    }
    Ok(value)
}

fn require_mofe03_positive_seed_scalar(value: f64, symbol: &str) -> Result<f64, HillslopeCliError> {
    if !value.is_finite() {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} seed value must be finite, observed {value}"
        )));
    }
    if value <= 0.0 {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} seed value must be > 0.0, observed {value}"
        )));
    }
    Ok(value)
}

fn seed_mofe03_wave2_class_symbol(
    runtime_surface: &mut HillslopeWritebackSurface,
    root: &str,
    class_index: usize,
    seed_value: f64,
) -> Result<(), HillslopeCliError> {
    if !seed_value.is_finite() {
        return Err(mofe03_wave2_seed_failure(format!(
            "{root}_{class_index:04} seed value must be finite, observed {seed_value}"
        )));
    }

    let symbol = mofe03_erod14_class_symbol(root, class_index);
    let value = if let Some(existing) = runtime_surface_symbol_value(runtime_surface, &symbol) {
        if !existing.is_finite() {
            return Err(mofe03_wave2_seed_failure(format!(
                "{symbol} must be finite when present, observed {existing}"
            )));
        }
        existing
    } else {
        seed_value
    };

    runtime_surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    Ok(())
}

fn mofe03_erod14_class_symbol(root: &str, class_index: usize) -> String {
    format!("{root}_{class_index:04}")
}

fn mofe03_wave2_seed_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "mofe03_wave2_seed",
        detail: format!("{SIMPIPE_GUARD_ID} {}", detail.into()),
    }
}

fn mofe_hourly_carry_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "mofe_hourly_carry",
        detail: format!("{SIMPIPE_GUARD_ID} {}", detail.into()),
    }
}

fn require_runtime_surface_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    let value = runtime_surface_symbol_value(runtime_surface, symbol)
        .ok_or_else(|| wb13_simout_failure(format!("missing required runtime symbol {symbol}")))?;
    if !value.is_finite() {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} is non-finite ({value})"
        )));
    }
    Ok(value)
}

fn require_simimpl10_coupling_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    let value = runtime_surface_symbol_value(runtime_surface, symbol)
        .ok_or_else(|| simcoup_failure(format!("missing required coupling symbol {symbol}")))?;
    if !value.is_finite() {
        return Err(simcoup_failure(format!(
            "coupling symbol {symbol} is non-finite ({value})"
        )));
    }
    Ok(value)
}

fn parse_simimpl10_binary_flag(field: &str, value: f64) -> Result<bool, HillslopeCliError> {
    if value.abs() <= SIMIMPL10_FLAG_TOLERANCE {
        return Ok(false);
    }
    if (value - 1.0).abs() <= SIMIMPL10_FLAG_TOLERANCE {
        return Ok(true);
    }
    Err(simcoup_failure(format!(
        "{field} must be binary 0|1, observed {value}"
    )))
}

fn scalar_to_i32(symbol: &str, value: f64) -> Result<i32, HillslopeCliError> {
    if !value.is_finite() {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} is non-finite ({value})"
        )));
    }
    let rounded = value.round();
    if (rounded - value).abs() > 1.0e-9 {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} must be integral for WB13 publication, observed {value}"
        )));
    }
    if rounded < f64::from(i32::MIN) || rounded > f64::from(i32::MAX) {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} out of i32 range ({value})"
        )));
    }
    format!("{rounded:.0}")
        .parse::<i32>()
        .map_err(|error| wb13_simout_failure(format!("failed converting {symbol} to i32: {error}")))
}

fn scalar_to_usize(symbol: &str, value: f64) -> Result<usize, HillslopeCliError> {
    let int_value = scalar_to_i32(symbol, value)?;
    usize::try_from(int_value).map_err(|_| {
        wb13_simout_failure(format!(
            "runtime symbol {symbol} must be non-negative usize, observed {value}"
        ))
    })
}

fn usize_to_scalar(symbol: &str, value: usize) -> Result<f64, HillslopeCliError> {
    value.to_string().parse::<f64>().map_err(|error| {
        wb13_simout_failure(format!(
            "failed converting {symbol} count {value} to f64 for runtime seeding: {error}"
        ))
    })
}

fn wb13_primary_layer_symbol(root: &str, layer_index: usize) -> String {
    format!("{root}_{layer_index:04}")
}

fn mode_selection_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "mode_selection",
        detail: format!("{WUI_MODE_GUARD_ID} {}", detail.into()),
    }
}

fn timestep_policy_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "timestep_policy",
        detail: format!("{SIMMODE_TIMESTEP_GUARD_ID} {}", detail.into()),
    }
}

fn simcons_intake_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "adapter_boundary",
        detail: format!("{SIMCONS_INTAKE_GUARD_ID} {}", detail.into()),
    }
}

fn simcoup_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "coupling_vectors",
        detail: format!("{SIMCOUP_GUARD_ID} {}", detail.into()),
    }
}

fn wb13_simout_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "wb13_publication",
        detail: format!("{SIMOUT_GUARD_ID} {}", detail.into()),
    }
}

fn build_loss_output_json(
    run_name: &str,
    soil: &openwepp_input_contract::parsers::soil::SoilProfile,
    snow: &openwepp_input_contract::parsers::snow::SnowParseOutput,
    frost: &openwepp_input_contract::parsers::frost::FrostParseOutput,
    climate_span: &ClimateRunSpanSummary,
    executed_day_count: usize,
) -> Result<String, HillslopeCliError> {
    let payload = serde_json::json!({
        "schema": "openwepp-hillslope-loss-v1",
        "run_name": run_name,
        "first_day_year": climate_span.first_day.year,
        "first_day_julian": climate_span.first_day.julian_day,
        "last_day_year": climate_span.last_day.year,
        "last_day_julian": climate_span.last_day.julian_day,
        "precipitation_mm": climate_span.first_day.precipitation_mm,
        "climate_day_count": climate_span.days.len(),
        "executed_day_count": executed_day_count,
        "ofe_count": soil.ofes.len(),
        "snow_override_applied": snow.sidecar_present,
        "frost_wint_red": frost.wint_red,
    });

    serde_json::to_string_pretty(&payload)
        .map_err(|source| HillslopeCliError::ManifestSerialize { source })
}

fn build_optional_output_payload(
    run_name: &str,
    output_path: &Path,
    climate_span: &ClimateRunSpanSummary,
    executed_day_count: usize,
) -> String {
    let file_name = file_name_string(output_path);
    format!(
        "openwepp_optional_output_v1\nrun_name={run_name}\nfile={file_name}\nfirst_year={}\nfirst_day={}\nlast_year={}\nlast_day={}\nclimate_day_count={}\nexecuted_day_count={}\nprecipitation_mm={:.3}\n",
        climate_span.first_day.year,
        climate_span.first_day.julian_day,
        climate_span.last_day.year,
        climate_span.last_day.julian_day,
        climate_span.days.len(),
        executed_day_count,
        climate_span.first_day.precipitation_mm
    )
}

#[derive(Debug, Clone, Copy)]
struct ClimateDayProjection {
    year: i32,
    month: i32,
    day_of_month: i32,
    julian_day: u16,
    precipitation_mm: f64,
}

#[derive(Debug, Clone)]
struct ClimateRunSpanSummary {
    days: Vec<ClimateDayProjection>,
    first_day: ClimateDayProjection,
    last_day: ClimateDayProjection,
}

fn climate_day_projection(
    record: &ClimateDailyRecord,
) -> Result<ClimateDayProjection, HillslopeCliError> {
    match record {
        ClimateDailyRecord::NoBreakpoint(day) => {
            let julian_day = day_of_year(day.year, day.mon, day.day)?;
            Ok(ClimateDayProjection {
                year: day.year,
                month: day.mon,
                day_of_month: day.day,
                julian_day,
                precipitation_mm: (day.prcp * 1_000.0).max(0.0),
            })
        }
        ClimateDailyRecord::Breakpoint(day) => {
            let julian_day = day_of_year(day.year, day.mon, day.day)?;
            let prcp_mm = day
                .breakpoints
                .last()
                .map_or(0.0, |point| (point.pptcum * 1_000.0).max(0.0));
            Ok(ClimateDayProjection {
                year: day.year,
                month: day.mon,
                day_of_month: day.day,
                julian_day,
                precipitation_mm: prcp_mm,
            })
        }
    }
}

fn build_climate_run_span_summary(
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
) -> Result<ClimateRunSpanSummary, HillslopeCliError> {
    let mut days = Vec::with_capacity(climate.daily_records.len());
    for record in &climate.daily_records {
        days.push(climate_day_projection(record)?);
    }

    let Some(first_day) = days.first().copied() else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: "climate daily record set is empty".to_string(),
        });
    };
    let Some(last_day) = days.last().copied() else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: "climate daily record set is empty".to_string(),
        });
    };

    Ok(ClimateRunSpanSummary {
        days,
        first_day,
        last_day,
    })
}

fn simulation_year_from_calendar_year(
    calendar_year: i32,
    simulation_start_year: i32,
) -> Result<i32, HillslopeCliError> {
    let relative_year = calendar_year
        .checked_sub(simulation_start_year)
        .and_then(|offset| offset.checked_add(1))
        .ok_or_else(|| {
            wb13_simout_failure(format!(
                "simulation-year mapping overflow for calendar_year={calendar_year} and simulation_start_year={simulation_start_year}"
            ))
        })?;
    if relative_year <= 0 {
        return Err(wb13_simout_failure(format!(
            "simulation-year mapping must be >= 1, observed {relative_year} from calendar_year={calendar_year} and simulation_start_year={simulation_start_year}"
        )));
    }
    Ok(relative_year)
}

fn day_of_year(year: i32, month: i32, day: i32) -> Result<u16, HillslopeCliError> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: format!("invalid calendar date {year}-{month}-{day}"),
        });
    }

    let leap = is_leap_year(year);
    let month_lengths = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let max_day = month_lengths[usize::try_from(month - 1).unwrap_or(0)];
    if day > max_day {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: format!("invalid day-of-month {day} for month {month}"),
        });
    }

    let mut doy = day;
    for length in month_lengths
        .iter()
        .take(usize::try_from(month - 1).unwrap_or(0))
    {
        doy += *length;
    }

    u16::try_from(doy).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "climate",
        detail: format!("day-of-year out of u16 range: {doy}"),
    })
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
