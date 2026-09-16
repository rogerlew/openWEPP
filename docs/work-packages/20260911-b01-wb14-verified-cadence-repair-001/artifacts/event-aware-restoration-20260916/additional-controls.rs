    #[test]
    fn recorded_archive_manifest_rejects_semantic_order_and_namespace_changes() {
        let export = std::path::PathBuf::from(std::env::var_os("OPENWEPP_B01_MEMBER_EXPORT").expect("export"));
        let seed = std::env::var_os("OPENWEPP_B01_PINNED_SEED").expect("seed");
        let pin = extract_pinned_seed_configuration_v1(std::path::Path::new(&seed)).expect("pin");
        let _native_audit = crate::v9_real_consumer_shadow::begin_covered_native_physical_path_audit_v1();
        let _probe_audit = ProbeAudit::begin();
        let summary: serde_json::Value = serde_json::from_slice(&std::fs::read(export.join("summary.json")).expect("summary")).expect("summary JSON");
        let mut entries = Vec::new();
        for entry in summary["rows"].as_array().expect("rows") {
            if entry["kinds"] != serde_json::json!(["b01_wb14_committed_day_archive_record_v1"]) { continue; }
            let row = decode_member_backed_row_v1(&export, &export.join(entry["event_file"].as_str().expect("event file")), &BTreeSet::new()).expect("archive row");
            let archive: crate::Stage3CommittedDayArchiveEntryV1 = serde_json::from_value(row["archive_entry"].clone()).expect("entry");
            archive.validate().expect("each original entry is independently sealed");
            entries.push(archive);
        }
        assert_eq!(entries.len(), 4);
        let empty = crate::Stage3CommittedDayArchiveManifestV1::empty(pin.run_identity, pin.topology_identity).expect("empty manifest");
        let mut positive = empty.clone();
        for entry in &entries { positive.append(entry.clone()).expect("recorded order"); }
        positive.validate().expect("complete canonical manifest");
        let day = decode_member_backed_row_v1(&export, &export.join("rows/108033.events.jsonl"), &BTreeSet::new()).expect("day");
        let prefix: crate::Stage3ArchivedReceiptPrefixV1 = serde_json::from_value(day["archived_receipt_prefix"].clone()).expect("prefix");
        assert_eq!(positive.ordered_day_chain_sha256, prefix.ordered_day_chain_sha256);
        assert_eq!(positive.archive_content_root_sha256, prefix.archive_content_root_sha256);
        assert_eq!(positive.committed_day_count, prefix.archived_day_count);
        for (name, sequence) in [("omitted first", vec![1]), ("duplicate", vec![0,0]), ("reordered", vec![0,2,1]), ("omitted middle", vec![0,1,3])] {
            let mut candidate = empty.clone();
            let result = sequence.into_iter().try_for_each(|index| candidate.append(entries[index].clone()));
            println!("manifest poison {name}: {result:?}");
            assert!(matches!(result, Err(crate::DirectSnowStage3V11AttachmentError::Identity("archive manifest append order or prior root"))));
            candidate.validate().expect("failed append preserves admitted prefix");
        }
        for (name, run, topology) in [("run namespace", digest_bytes(b"foreign-run"), pin.topology_identity), ("topology namespace", pin.run_identity, digest_bytes(b"foreign-topology"))] {
            let mut foreign = crate::Stage3CommittedDayArchiveManifestV1::empty(run, topology).expect("valid foreign namespace");
            let result = foreign.append(entries[0].clone());
            println!("manifest poison {name}: {result:?}");
            assert!(matches!(result, Err(crate::DirectSnowStage3V11AttachmentError::Identity("archive manifest append order or prior root"))));
        }
    }

    #[test]
    fn archive_first_positive_beginning_rejects_pre_event_owner_substitution() {
        let export = std::path::PathBuf::from(std::env::var_os("OPENWEPP_B01_MEMBER_EXPORT").expect("export"));
        let seed = std::env::var_os("OPENWEPP_B01_PINNED_SEED").expect("seed");
        let pin = extract_pinned_seed_configuration_v1(std::path::Path::new(&seed)).expect("pin");
        let _native_audit = crate::v9_real_consumer_shadow::begin_covered_native_physical_path_audit_v1();
        let _probe_audit = ProbeAudit::begin();
        openwepp_land_surface_energy::snow_accuracy_policy::Policy::B01.install().expect("policy");
        let target = restore_snow_free_clock_parent_from_member_export(&export).expect("target");
        let bytes: Vec<u8> = serde_json::from_value(target.recorded_row["vegetation_configuration_canonical_json"].clone()).expect("vegetation bytes");
        let vegetation: openwepp_vegetation::VegetationConfigurationV11 = serde_json::from_slice(&bytes).expect("vegetation");
        assert_eq!(vegetation.imported_v10, pin.vegetation);
        let row = decode_member_backed_row_v1(&export, &export.join("rows/27431.events.jsonl"), &BTreeSet::new()).expect("archive row");
        let entry = serde_json::from_value(row["archive_entry"].clone()).expect("entry");
        let raw = std::fs::read(export.join("rows/27431.canonical_record.bin")).expect("actual archive");
        let restored = authenticate_archived_day_with_configuration_v1(entry, raw, pin.run_identity, pin.topology_identity, &vegetation, &pin.lse).expect("fully authenticated archive zero");
        validate_archive_first_positive_join(&restored.parent_receipt, &restored.publication).expect("positive cross-object join");
        assert_ne!(restored.entry.beginning_owner_set_sha256, restored.publication.first_positive_beginning_owner, "day zero has an admitted event before its first support");
        let mut substituted = restored.publication.clone();
        substituted.first_positive_beginning_owner = restored.entry.beginning_owner_set_sha256;
        let result = validate_archive_first_positive_join(&restored.parent_receipt, &substituted);
        println!("archive beginning semantic substitution: {result:?}");
        assert_eq!(result.as_ref().map_err(String::as_str), Err("archive parent first-positive beginning/publication join"));
    }
