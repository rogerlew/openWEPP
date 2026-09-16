from pathlib import Path
R=Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916/crates/openwepp-hillslope-orchestrator/src')
p=R/'snow_stage3_v11_member_restoration.rs';s=p.read_text().replace('fn pin_hydrology(', 'pub(super) fn pin_hydrology(').replace('fn restore_pinned_surface(', 'pub(super) fn restore_pinned_surface(')
s=s.replace('for (lane, pinned) in inputs.lanes.iter().zip(lanes) {','for (lane, pinned) in inputs.lanes.iter().zip(lanes) {\n        pin_lane_geometry(lane, pinned)?;')
i=s.index('fn load_row(')
s=s[:i]+'''fn pin_lane_geometry(lane: &crate::DirectLaneConstructorInputs, pinned: &Value) -> Result<(), String> {
    for (name, actual) in [
        ("upstream_area_ratio", lane.upstream_area_ratio),
        ("area_m2", lane.area_m2),
        ("runoff_publication_q_scale", lane.runoff_publication_q_scale),
        ("runoff_publication_qofe_scale", lane.runoff_publication_qofe_scale),
        ("runoff_publication_efflen_m", lane.runoff_publication_efflen_m),
        ("runoff_publication_cumulative_length_m", lane.runoff_publication_cumulative_length_m),
        ("runoff_publication_ofe_length_m", lane.runoff_publication_ofe_length_m),
    ] {
        if actual.to_bits() != hex_float(required(pinned, name)?)?.to_bits() {
            return Err(format!("seed immutable lane {name}"));
        }
    }
    if pinned["upstream_lane_id"] != lane.upstream_lane_id || pinned["downstream_lane_id"] != lane.downstream_lane_id {
        return Err("seed immutable lane topology".into());
    }
    Ok(())
}

pub(super) fn native_authority_joins(native: &Value, position: u128, expected_gsi: &Value) -> Result<(bool, bool), String> {
    let captured_position: u128 = decode(required(native, "accepted_interval_count")?)?;
    let captured_gsi: openwepp_coupled_time::Digest32 = decode(required(native, "provider_gsi_receipt_sha256")?)?;
    let expected_gsi: openwepp_coupled_time::Digest32 = decode(expected_gsi)?;
    Ok((captured_position == position, captured_gsi == expected_gsi))
}

'''+s[i:]
a=s.index('    let position_matches = native[');b=s.index('    if native["next_day_index"]',a);positionlog=s[a:b];s=s[:a]+s[b:]
a=s.index('    let gsi_matches = native["provider_gsi_receipt_sha256"] == expected_gsi;')
s=s[:a]+'    let (position_matches, gsi_matches) = native_authority_joins(native, position, &expected_gsi)?;\n'+positionlog[positionlog.index('    println!'):]+s[a:].replace('    let gsi_matches = native["provider_gsi_receipt_sha256"] == expected_gsi;\n','',1)
p.write_text(s)
p=R/'snow_stage3_v11_current_context_capture.rs';s=p.read_text()
i=s.index('    #[test]\n    fn current_phase_members_validate_without_physics()')
s=s[:i]+'''    #[test]
    fn provider_authority_scalars_reject_semantic_substitutions_without_installation() {
        let export = std::path::PathBuf::from(std::env::var_os("OPENWEPP_B01_MEMBER_EXPORT").expect("export"));
        let _native_audit = crate::v9_real_consumer_shadow::begin_covered_native_physical_path_audit_v1();
        let _probe_audit = ProbeAudit::begin();
        let day = decode_member_backed_row_v1(&export, &export.join("rows/108033.events.jsonl"), &BTreeSet::new()).expect("day");
        let prefix: crate::Stage3ArchivedReceiptPrefixV1 = serde_json::from_value(day["archived_receipt_prefix"].clone()).expect("prefix");
        prefix.validate().expect("prefix validates");
        let target = restore_snow_free_clock_parent_from_member_export(&export).expect("target");
        let interval: u128 = serde_json::from_value(target.recorded_row["interval_index"].clone()).expect("interval");
        let position = prefix.next_parent_sequence.checked_add(interval).expect("derived position");
        let expected_gsi = day["provider_constructor_operands"]["gsi_receipt_sha256"].clone();
        // This is a scalar-join fixture, never a native object or installable capability.
        let mut operands = serde_json::json!({"accepted_interval_count": position, "provider_gsi_receipt_sha256": expected_gsi});
        assert_eq!(member_restoration::native_authority_joins(&operands, position, &expected_gsi).expect("well-typed scalar fixture"), (true,true));
        operands["accepted_interval_count"] = serde_json::json!(position+1);
        assert_eq!(member_restoration::native_authority_joins(&operands, position, &expected_gsi).expect("well-typed position poison"), (false,true));
        operands["accepted_interval_count"] = serde_json::json!(position);
        operands["provider_gsi_receipt_sha256"] = serde_json::json!(digest_bytes(b"foreign-gsi-receipt"));
        assert_eq!(member_restoration::native_authority_joins(&operands, position, &expected_gsi).expect("well-typed GSI poison"), (true,false));
        println!("provider scalar join controls: matching, shifted position and foreign GSI checked; no native installation");
    }

'''+s[i:]
a=s.index('        assert_eq!(capture_direct_run_constructor_inputs(&typed), *raw);')+len('        assert_eq!(capture_direct_run_constructor_inputs(&typed), *raw);')
s=s[:a]+'''
        let seed = std::env::var_os("OPENWEPP_B01_PINNED_SEED").expect("seed");
        let seed: serde_json::Value = serde_json::from_slice(&std::fs::read(seed).expect("seed bytes")).expect("seed JSON");
        member_restoration::pin_hydrology(&seed, &typed).expect("independent immutable hydrology authority");
        let mut poisoned = typed.clone();
        poisoned.lanes[0].area_m2 += 1.0;
        assert_eq!(member_restoration::pin_hydrology(&seed, &poisoned).as_ref().map_err(String::as_str), Err("seed immutable lane area_m2"));
        member_restoration::restore_pinned_surface(&seed, &restored.recorded_row, &restored.clock).expect("seed surface and prefix configuration join");
        let mut wrong_surface = restored.recorded_row.clone();
        wrong_surface["phase_diagnostics"]["prefix_validation_surface_configuration_canonical_json"] = serde_json::json!([]);
        assert_eq!(member_restoration::restore_pinned_surface(&seed, &wrong_surface, &restored.clock).err().as_deref(), Some("prefix surface configuration authority join"));
'''+s[a:]
p.write_text(s)
