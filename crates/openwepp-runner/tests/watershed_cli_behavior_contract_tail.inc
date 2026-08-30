fn build_schema1_event_then_no_event_fixture(hillslope_id: u32, nofe: u16) -> Vec<u8> {
    let mut file = append_common_prefix(SUPPORTED_MAJOR_V1, 0, hillslope_id, nofe, 1, 2004, 1, 2);
    let event_payload =
        build_event_payload(nofe, 1, 2004, 1, 0.25, 1.0, 5.0, 4.0, 1_800.0, 1_200.0);
    let no_event_payload = build_no_event_payload(nofe, 1, 2004, 2);
    let event_crc = crc32c(&event_payload);
    let no_event_crc = crc32c(&no_event_payload);

    let directory_start = file.len();
    let directory_len = 4 + 2 * 27;
    let event_payload_offset = directory_start + directory_len;
    let no_event_payload_offset = event_payload_offset + event_payload.len();
    let mut directory = Vec::new();
    put_u32(&mut directory, 2);
    put_u32(&mut directory, 1);
    put_i32(&mut directory, 2004);
    put_u16(&mut directory, 1);
    put_u8(&mut directory, 2);
    put_u64(&mut directory, event_payload_offset as u64);
    put_u32(&mut directory, event_payload.len() as u32);
    put_u32(&mut directory, event_crc);
    put_u32(&mut directory, 1);
    put_i32(&mut directory, 2004);
    put_u16(&mut directory, 2);
    put_u8(&mut directory, 0);
    put_u64(&mut directory, no_event_payload_offset as u64);
    put_u32(&mut directory, no_event_payload.len() as u32);
    put_u32(&mut directory, no_event_crc);

    file.extend_from_slice(&directory);
    file.extend_from_slice(&event_payload);
    file.extend_from_slice(&no_event_payload);

    let directory_crc = crc32c(&directory);
    put_u32(&mut file, directory_crc);
    let file_crc_pos = file.len();
    put_u32(&mut file, 0);
    put_u32(&mut file, 2);
    file.extend_from_slice(FOOTER_MAGIC);
    let file_crc = crc32c(&file);
    put_u32_at(&mut file, file_crc_pos, file_crc);
    file
}

fn write_hbp_fixture(
    path: PathBuf,
    hillslope_id: u32,
    concentration: f64,
    fraction: f64,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
) {
    write_hbp_fixture_with_nofe(
        path,
        hillslope_id,
        1,
        concentration,
        fraction,
        peak_runoff_m3_s,
        duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
    );
}

fn write_hbp_fixture_with_nofe(
    path: PathBuf,
    hillslope_id: u32,
    nofe: u16,
    concentration: f64,
    fraction: f64,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
) {
    let bytes = build_schema1_event_fixture(
        hillslope_id,
        nofe,
        concentration,
        fraction,
        peak_runoff_m3_s,
        duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
    );
    fs::write(path, bytes).expect("HBP fixture should be writable");
}

fn write_hbp_no_event_fixture(path: PathBuf, hillslope_id: u32) {
    let bytes = build_schema1_no_event_fixture(hillslope_id, 1);
    fs::write(path, bytes).expect("HBP no-event fixture should be writable");
}

fn write_hbp_no_event_fixture_with_groundwater(
    path: PathBuf,
    hillslope_id: u32,
    baseflow_volume_m3: f64,
    deep_seepage_volume_m3: f64,
) {
    let bytes = build_schema1_no_event_fixture_with_groundwater(
        hillslope_id,
        1,
        baseflow_volume_m3,
        deep_seepage_volume_m3,
    );
    fs::write(path, bytes).expect("HBP no-event fixture should be writable");
}

fn write_hbp_event_then_no_event_fixture(path: PathBuf, hillslope_id: u32) {
    let bytes = build_schema1_event_then_no_event_fixture(hillslope_id, 1);
    fs::write(path, bytes).expect("HBP event-then-no-event fixture should be writable");
}
