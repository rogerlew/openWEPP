/// SC-INFILE-HBP-001 v0.2.0: the per-class arrays are `npart`-long
/// (production minor-1 writes `npart = 5`; the minor-0 legacy shape stays
/// single-class), and the paired ADR-0036 hourly surfaces select the
/// payload minor (`Some` => minor 1).
#[derive(Clone)]
struct HbpEventFixtureInput {
    hillslope_id: u32,
    nofe: u16,
    julian_day: u16,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    baseflow_volume_m3: f64,
    deep_seepage_volume_m3: f64,
    sediment_concentration_kg_m3: Vec<f64>,
    particle_flow_fraction: Vec<f64>,
    particle_diameter_m: Vec<f64>,
    hourly_runoff_volume_m3: Option<[f64; 24]>,
    hourly_sediment_mass_kg: Option<[f64; 24]>,
}

#[derive(Clone)]
struct HbpEventPayloadInput {
    nofe: u16,
    sim_year_index: u32,
    calendar_year: i32,
    julian_day: u16,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    baseflow_volume_m3: f64,
    deep_seepage_volume_m3: f64,
    sediment_concentration_kg_m3: Vec<f64>,
    particle_flow_fraction: Vec<f64>,
    hourly_runoff_volume_m3: Option<[f64; 24]>,
    hourly_sediment_mass_kg: Option<[f64; 24]>,
}

#[derive(Clone)]
struct HbpHeaderInput {
    schema_major: u16,
    schema_minor: u16,
    hillslope_id: u32,
    nofe: u16,
    nyear: u32,
    begin_year: i32,
    julian_day: u16,
    particle_diameter_m: Vec<f64>,
}

const DIRECT_WAT_WEPP_ID: i32 = 1;
const HBP_MAGIC: &[u8; 8] = b"WFPHBP01";
const HBP_FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
const HBP_SUPPORTED_MAJOR_V1: u16 = 1;
const HBP_DIM_SCALAR: u8 = 0;
const HBP_DIM_NOFE: u8 = 1;
const HBP_DIM_NOFE_LAYERS: u8 = 2;
const HBP_DEFAULT_CALENDAR_YEAR: i32 = 2004;
const HBP_DEFAULT_PARTICLE_DIAMETER_M: f64 = 0.001;
const HBP_SCALE_INV_I64: f64 = 1.0e9;
const HBP_I64_MIN_F64: f64 = -9_223_372_036_854_775_808.0;
const HBP_I64_MAX_F64: f64 = 9_223_372_036_854_775_807.0;
const HBP_REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208,
    209, 210, 300, 900, 901,
];
const MOFE04_PUBLICATION_OFE_POLICY: &str = "single-row-canonicalized-hillslope-aggregate";
const MF_PUBLICATION_OFE_POLICY: &str = "per-ofe-dynamic-water-balance-state";
const MOFE04_PUBLICATION_AREA_POLICY: &str = "sum-ofe-geometry-area";
const MF_STORAGE_LINEAGE_POLICY: &str = "per-ofe-dynamic-wb-state";
const MF_PER_OFE_STATE_POLICY: &str = "published-per-ofe-wb13-records";
const MF_IDENTITY_STATUS: &str = "pass-published-per-ofe-wb13-records";

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

fn build_schema1_hbp_event_fixture(
    input: HbpEventFixtureInput,
) -> Result<Vec<u8>, HillslopeCliError> {
    // ADR-0036 D2: the paired hourly surfaces select payload minor 1; a
    // half-present pair is a writer defect, fail closed.
    let schema_minor = match (
        input.hourly_runoff_volume_m3.is_some(),
        input.hourly_sediment_mass_kg.is_some(),
    ) {
        (true, true) => 1,
        (false, false) => 0,
        _ => {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass",
                detail: format!(
                    "{SIMOUT_GUARD_ID} HBP hourly surfaces must be written as a pair"
                ),
            });
        }
    };
    if input.sediment_concentration_kg_m3.len() != input.particle_flow_fraction.len()
        || input.sediment_concentration_kg_m3.len() != input.particle_diameter_m.len()
        || input.sediment_concentration_kg_m3.is_empty()
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP per-class arrays must be equal-length and non-empty"
            ),
        });
    }
    let mut file = append_hbp_common_prefix(&HbpHeaderInput {
        schema_major: HBP_SUPPORTED_MAJOR_V1,
        schema_minor,
        hillslope_id: input.hillslope_id,
        nofe: input.nofe,
        nyear: 1,
        begin_year: HBP_DEFAULT_CALENDAR_YEAR,
        julian_day: input.julian_day,
        particle_diameter_m: input.particle_diameter_m.clone(),
    })?;
    let payload = build_hbp_event_payload(&HbpEventPayloadInput {
        nofe: input.nofe,
        sim_year_index: 1,
        calendar_year: HBP_DEFAULT_CALENDAR_YEAR,
        julian_day: input.julian_day,
        peak_runoff_m3_s: input.peak_runoff_m3_s,
        duration_seconds: input.duration_seconds,
        total_detachment_kg: input.total_detachment_kg,
        total_deposition_kg: input.total_deposition_kg,
        baseflow_volume_m3: input.baseflow_volume_m3,
        deep_seepage_volume_m3: input.deep_seepage_volume_m3,
        sediment_concentration_kg_m3: input.sediment_concentration_kg_m3,
        particle_flow_fraction: input.particle_flow_fraction,
        hourly_runoff_volume_m3: input.hourly_runoff_volume_m3,
        hourly_sediment_mass_kg: input.hourly_sediment_mass_kg,
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

fn build_hbp_event_payload(input: &HbpEventPayloadInput) -> Result<Vec<u8>, HillslopeCliError> {
    let nofe = u32::from(input.nofe);
    let max_layers = 1u32;

    let mut payload = Vec::new();
    put_u32(&mut payload, input.sim_year_index);
    put_i32(&mut payload, input.calendar_year);
    put_u16(&mut payload, input.julian_day);
    put_u8(&mut payload, 2);
    put_u16(
        &mut payload,
        match (
            input.hourly_runoff_volume_m3.is_some(),
            input.hourly_sediment_mass_kg.is_some(),
        ) {
            (true, true) => 1,
            _ => 0,
        },
    );
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
    let npart_u32 = u32::try_from(input.sediment_concentration_kg_m3.len()).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP per-class count exceeds u32: {}",
                input.sediment_concentration_kg_m3.len()
            ),
        }
    })?;
    put_u32(&mut payload, npart_u32);
    for value in &input.sediment_concentration_kg_m3 {
        put_f64(&mut payload, *value);
    }
    put_u32(&mut payload, npart_u32);
    for value in &input.particle_flow_fraction {
        put_f64(&mut payload, *value);
    }
    // ADR-0036 D2 / SC-INFILE-HBP-001 §3a: the paired hourly surfaces sit
    // before the final groundwater/baseflow i64 pair. Strict consumption makes
    // any writer/parser placement divergence a typed failure.
    if let (Some(hourly_volume), Some(hourly_sediment)) = (
        &input.hourly_runoff_volume_m3,
        &input.hourly_sediment_mass_kg,
    ) {
        put_u32(&mut payload, 24);
        for value in hourly_volume {
            put_f64(&mut payload, *value);
        }
        put_u32(&mut payload, 24);
        for value in hourly_sediment {
            put_f64(&mut payload, *value);
        }
    }
    put_i64(&mut payload, scaled_i64(input.baseflow_volume_m3)?);
    put_i64(&mut payload, scaled_i64(input.deep_seepage_volume_m3)?);

    for state_id in HBP_REQUIRED_STATE_IDS {
        payload.extend_from_slice(&build_hbp_state_entry(*state_id, nofe, max_layers)?);
    }

    Ok(payload)
}

fn append_hbp_common_prefix(input: &HbpHeaderInput) -> Result<Vec<u8>, HillslopeCliError> {
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

    let npart = u16::try_from(input.particle_diameter_m.len()).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP npart exceeds u16: {}",
                input.particle_diameter_m.len()
            ),
        }
    })?;
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
    for diameter_m in &input.particle_diameter_m {
        put_f64(&mut file, *diameter_m);
    }
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

#[cfg(test)]
#[allow(dead_code)]
fn build_hbp_output_from_direct_publication(
    output_pass: &Path,
    publication: &DirectRunPublicationFrame,
) -> Result<Vec<u8>, HillslopeCliError> {
    let latest_row = publication
        .last_day()
        .ok_or_else(|| direct_publication_output_failure("missing latest direct publication row"))?;
    let sediment_row = direct_publication_last_hbp_sediment_row(publication).unwrap_or(latest_row);
    let nofe = u16::try_from(publication.identity.lane_count).map_err(|_| {
        direct_publication_output_failure(format!(
            "direct publication lane count out of u16 range: {}",
            publication.identity.lane_count
        ))
    })?;
    if nofe == 0 {
        return Err(direct_publication_output_failure(
            "direct publication lane count must be >= 1",
        ));
    }
    let sediment_concentration_kg_m3 = sediment_row
        .erosion
        .hbp_sediment_concentration_kg_m3
        .map_or_else(
            || {
                direct_publication_required_sediment_concentration(
                    sediment_row.erosion.sediment_concentration_kg_m3,
                )
                .map(|values| values[0])
            },
            |value| direct_publication_required_erosion_scalar(
                "erosion.hbp_sediment_concentration_kg_m3",
                Some(value),
            ),
        )?;

    build_schema1_hbp_event_fixture(HbpEventFixtureInput {
        hillslope_id: parse_hillslope_id_from_output_pass_path(output_pass)?,
        nofe,
        julian_day: latest_row.calendar.julian_day,
        peak_runoff_m3_s: direct_publication_required_erosion_scalar(
            "runoff.peak_runoff_m3_s or erosion.peak_runoff_m3_s",
            latest_row
                .runoff
                .peak_runoff_m3_s
                .or(latest_row.erosion.peak_runoff_m3_s),
        )?,
        duration_seconds: direct_publication_required_erosion_scalar(
            "runoff.runoff_duration_s or erosion.runoff_duration_s",
            latest_row
                .runoff
                .runoff_duration_s
                .or(latest_row.erosion.runoff_duration_s),
        )?,
        total_detachment_kg: direct_publication_required_erosion_scalar(
            "erosion.hbp_total_detachment_kg or erosion.total_detachment_kg",
            sediment_row
                .erosion
                .hbp_total_detachment_kg
                .or(sediment_row.erosion.total_detachment_kg),
        )?,
        total_deposition_kg: direct_publication_required_erosion_scalar(
            "erosion.hbp_total_deposition_kg or erosion.total_deposition_kg",
            sediment_row
                .erosion
                .hbp_total_deposition_kg
                .or(sediment_row.erosion.total_deposition_kg),
        )?,
        baseflow_volume_m3: latest_row.subsurface.groundwater_baseflow_m3,
        deep_seepage_volume_m3: latest_row.subsurface.groundwater_deep_seepage_m3,
        sediment_concentration_kg_m3: vec![sediment_concentration_kg_m3],
        particle_flow_fraction: vec![1.0],
        particle_diameter_m: vec![HBP_DEFAULT_PARTICLE_DIAMETER_M],
        hourly_runoff_volume_m3: None,
        hourly_sediment_mass_kg: None,
    })
}

#[cfg(test)]
#[allow(dead_code)]
fn direct_publication_last_hbp_sediment_row(
    publication: &DirectRunPublicationFrame,
) -> Option<&openwepp_hillslope_orchestrator::DirectPublicationDayRow> {
    publication
        .rows()
        .iter()
        .rev()
        .find(|row| direct_publication_row_has_hbp_sediment(row))
}

fn direct_publication_row_has_hbp_sediment(
    row: &openwepp_hillslope_orchestrator::DirectPublicationDayRow,
) -> bool {
    row.erosion
        .hbp_total_detachment_kg
        .or(row.erosion.total_detachment_kg)
        .is_some_and(|value| value > 0.0)
        || row
            .erosion
            .hbp_sediment_concentration_kg_m3
            .is_some_and(|value| value > 0.0)
        || row
            .erosion
            .sediment_concentration_kg_m3
            .is_some_and(|values| values.iter().any(|value| *value > 0.0))
}

fn direct_publication_required_erosion_scalar(
    field: &'static str,
    value: Option<f64>,
) -> Result<f64, HillslopeCliError> {
    let value = value.ok_or_else(|| {
        direct_publication_output_failure(format!(
            "direct publication row is missing producer-authoritative {field}"
        ))
    })?;
    if value.is_finite() && value >= 0.0 {
        return Ok(value);
    }
    Err(direct_publication_output_failure(format!(
        "direct publication row has invalid {field}: {value}"
    )))
}

fn direct_publication_required_sediment_concentration(
    value: Option<[f64; 5]>,
) -> Result<[f64; 5], HillslopeCliError> {
    let value = value.ok_or_else(|| {
        direct_publication_output_failure(
            "direct publication row is missing producer-authoritative erosion.sediment_concentration_kg_m3",
        )
    })?;
    for (index, scalar) in value.iter().enumerate() {
        if !scalar.is_finite() || *scalar < 0.0 {
            return Err(direct_publication_output_failure(format!(
                "direct publication row has invalid erosion.sediment_concentration_kg_m3[{index}]: {scalar}"
            )));
        }
    }
    Ok(value)
}

#[cfg(test)]
fn build_hillslope_wat_rows_from_direct_publication(
    publication: &DirectRunPublicationFrame,
) -> Result<Vec<HillslopeWatRow>, HillslopeCliError> {
    let simulation_start_year = publication
        .first_day()
        .ok_or_else(|| direct_publication_output_failure("missing first direct publication row"))?
        .calendar
        .year;
    publication
        .rows()
        .iter()
        .map(|row| build_hillslope_wat_row_from_direct_publication(row, simulation_start_year))
        .collect()
}

fn build_hillslope_wat_row_from_direct_publication(
    row: &openwepp_hillslope_orchestrator::DirectPublicationDayRow,
    simulation_start_year: i32,
) -> Result<HillslopeWatRow, HillslopeCliError> {
    let ofe = direct_publication_u32_to_i16("ofe", row.ofe_id)?;
    Ok(HillslopeWatRow {
        wepp_id: DIRECT_WAT_WEPP_ID,
        ofe_id: ofe,
        year: direct_publication_i32_to_i16(
            "year",
            simulation_year_from_calendar_year(row.calendar.year, simulation_start_year)?,
        )?,
        sim_day_index: row.sim_day_index,
        julian: direct_publication_u16_to_i16("julian", row.calendar.julian_day)?,
        month: row.calendar.month,
        day_of_month: row.calendar.day_of_month,
        water_year: row.calendar.water_year,
        ofe,
        p: row.climate.precipitation_mm,
        rm: row.liquid_input.rm_mm,
        q: row.runoff.q_mm,
        ep: row.evaporation.ep_mm,
        es: row.evaporation.es_mm,
        er: row.evaporation.er_mm,
        dp: row.subsurface.dp_mm,
        up_strm_q: row.transfer.upstream_surface_mm,
        sub_r_in: row.transfer.upstream_lateral_mm,
        latqcc: row.subsurface.latqcc_mm,
        base: Some(row.subsurface.groundwater_baseflow_mm),
        total_soil_water: row.storage.total_soil_mm,
        frozwt: row.storage.frozwt_mm,
        frdp: row.storage.frdp_mm.unwrap_or(0.0),
        snow_water: row.storage.snow_water_mm,
        snow_depth: Some(row.storage.snow_depth_mm),
        meltwater_temperature: row.water_temperature.meltwater_temperature_c,
        qofe: row.runoff.qofe_mm,
        tile: row.subsurface.tile_mm,
        irr: row.liquid_input.irrigation_mm,
        area: row.area_m2,
        soil_water_total: Some(row.storage.soil_water_total_mm),
        profile_depth: row.profile.depth_mm,
        profile_porosity_cap: row.profile.porosity_cap_mm,
        profile_fc_store: row.profile.fc_store_mm,
        profile_wp_store: row.profile.wp_store_mm,
        interception: Some(row.interception.interception_mm),
        interception_storage: row.interception.interception_storage_mm,
    })
}

#[cfg(test)]
fn build_hillslope_pass_rows_from_direct_publication(
    publication: &DirectRunPublicationFrame,
) -> Result<Vec<HillslopePassRow>, HillslopeCliError> {
    let simulation_start_year = publication
        .first_day()
        .ok_or_else(|| direct_publication_output_failure("missing first direct publication row"))?
        .calendar
        .year;
    let outlet_ofe_id = u32::try_from(publication.identity.lane_count).map_err(|_| {
        direct_publication_output_failure(format!(
            "direct publication lane count out of u32 range: {}",
            publication.identity.lane_count
        ))
    })?;
    publication
        .rows()
        .iter()
        .filter(|row| row.ofe_id == outlet_ofe_id)
        .map(|row| build_hillslope_pass_row_from_direct_publication(row, simulation_start_year))
        .collect()
}

fn build_hillslope_pass_row_from_direct_publication(
    row: &openwepp_hillslope_orchestrator::DirectPublicationDayRow,
    simulation_start_year: i32,
) -> Result<HillslopePassRow, HillslopeCliError> {
    Ok(HillslopePassRow {
        wepp_id: direct_publication_u32_to_i32("wepp_id", row.hillslope_id)?,
        year: direct_publication_i32_to_i16(
            "year",
            simulation_year_from_calendar_year(row.calendar.year, simulation_start_year)?,
        )?,
        sim_day_index: row.sim_day_index,
        julian: direct_publication_u16_to_i16("julian", row.calendar.julian_day)?,
        month: row.calendar.month,
        day_of_month: row.calendar.day_of_month,
        water_year: row.calendar.water_year,
        runvol_m3: row.runoff.runvol_m3,
        sbrunv_m3: row.subsurface.sbrunv_m3,
        peakro_m3_s: direct_publication_required_erosion_scalar(
            "runoff.peak_runoff_m3_s or erosion.peak_runoff_m3_s",
            row.runoff
                .peak_runoff_m3_s
                .or(row.erosion.peak_runoff_m3_s),
        )?,
        // SC-SED-001 1b-C: surface the Wave-1 sediment-continuity totals
        // when the single-OFE solve is active; `None` (disabled / multi-OFE
        // Wave-2 path) keeps the prior zeroed sediment columns, so
        // non-sediment columns stay byte-identical.
        total_detachment_kg: row.erosion.total_detachment_kg.unwrap_or(0.0),
        total_deposition_kg: row.erosion.total_deposition_kg.unwrap_or(0.0),
        sediment_concentration_kg_m3: row
            .erosion
            .sediment_concentration_kg_m3
            .unwrap_or([0.0; 5]),
    })
}

#[cfg(test)]
fn build_loss_output_json_from_direct_publication(
    publication: &DirectRunPublicationFrame,
    ofe_count: usize,
    snow_override_applied: bool,
    frost_wint_red: i32,
) -> Result<String, HillslopeCliError> {
    let first_day = publication
        .first_day()
        .ok_or_else(|| direct_publication_output_failure("missing first direct publication row"))?;
    let last_day = publication
        .last_day()
        .ok_or_else(|| direct_publication_output_failure("missing last direct publication row"))?;
    let payload = serde_json::json!({
        "schema": "openwepp-hillslope-loss-v1",
        "run_name": publication.metadata.run_name,
        "first_day_year": first_day.calendar.year,
        "first_day_julian": first_day.calendar.julian_day,
        "last_day_year": last_day.calendar.year,
        "last_day_julian": last_day.calendar.julian_day,
        "precipitation_mm": first_day.climate.precipitation_mm,
        "climate_day_count": publication.identity.day_count,
        "executed_day_count": publication.identity.day_count,
        "ofe_count": ofe_count,
        "snow_override_applied": snow_override_applied,
        "frost_wint_red": frost_wint_red,
    });

    serde_json::to_string_pretty(&payload)
        .map_err(|source| HillslopeCliError::ManifestSerialize { source })
}

#[cfg(test)]
fn build_manifest_text_from_direct_publication(
    publication: &DirectRunPublicationFrame,
) -> Result<String, HillslopeCliError> {
    if publication.rows().is_empty() {
        return Err(direct_publication_output_failure(
            "direct publication manifest requires at least one row",
        ));
    }
    Ok(format!(
        "direct_publication_frame_v1\nrun_name={}\nruntime_selection={}\noutput_policy={}\nrow_count={}\nlane_count={}\nday_count={}\n",
        publication.metadata.run_name,
        publication.metadata.runtime_selection,
        publication.metadata.output_policy,
        publication.rows().len(),
        publication.identity.lane_count,
        publication.identity.day_count
    ))
}

fn direct_publication_i32_to_i16(field: &'static str, value: i32) -> Result<i16, HillslopeCliError> {
    i16::try_from(value).map_err(|_| {
        direct_publication_output_failure(format!(
            "direct publication field {field} out of i16 range: {value}"
        ))
    })
}

fn direct_publication_u16_to_i16(field: &'static str, value: u16) -> Result<i16, HillslopeCliError> {
    i16::try_from(value).map_err(|_| {
        direct_publication_output_failure(format!(
            "direct publication field {field} out of i16 range: {value}"
        ))
    })
}

fn direct_publication_u32_to_i16(field: &'static str, value: u32) -> Result<i16, HillslopeCliError> {
    i16::try_from(value).map_err(|_| {
        direct_publication_output_failure(format!(
            "direct publication field {field} out of i16 range: {value}"
        ))
    })
}

fn direct_publication_u32_to_i32(field: &'static str, value: u32) -> Result<i32, HillslopeCliError> {
    i32::try_from(value).map_err(|_| {
        direct_publication_output_failure(format!(
            "direct publication field {field} out of i32 range: {value}"
        ))
    })
}

fn direct_publication_output_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!("{SIMOUT_GUARD_ID} {}", detail.into()),
    }
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
    effective_temperature_c: f64,
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
                precipitation_mm: day.prcp.max(0.0),
                effective_temperature_c: (day.tmax + day.tmin) * 0.5,
            })
        }
        ClimateDailyRecord::Breakpoint(day) => {
            let julian_day = day_of_year(day.year, day.mon, day.day)?;
            let prcp_mm = day
                .breakpoints
                .last()
                .map_or(0.0, |point| point.pptcum.max(0.0));
            Ok(ClimateDayProjection {
                year: day.year,
                month: day.mon,
                day_of_month: day.day,
                julian_day,
                precipitation_mm: prcp_mm,
                effective_temperature_c: (day.tmax + day.tmin) * 0.5,
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
