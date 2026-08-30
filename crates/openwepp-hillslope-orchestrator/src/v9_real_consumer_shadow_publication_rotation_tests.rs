mod accepted_publication_rotation_tests {
    use super::*;

    #[test]
    fn sealed_prefix_is_exact_and_bounded_and_rotation_ack_poisons_reject() {
        let parent = ParentTransactionId::from_digest(Digest32::from_bytes([7; 32]));
        let stale_parent = ParentTransactionId::from_digest(Digest32::from_bytes([6; 32]));
        let full_day = TimeSupport::new(
            openwepp_coupled_time::ModelTimeNs::new(0),
            openwepp_coupled_time::ModelTimeNs::new(
                crate::snow_stage3_v11_attachment::STAGE3_V11_DAY_NS,
            ),
        )
        .expect("full day");
        let partial_day = TimeSupport::new(
            full_day.start_ns(),
            openwepp_coupled_time::ModelTimeNs::new(full_day.end_ns().get() - 1),
        )
        .expect("partial day");
        let boundary = |last_support, has_pending_pre_support_event| RotationDayBoundaryV1 {
            day_index: 0,
            first_day_index: 0,
            last_day_index: 0,
            all_resident_supports_match_day: true,
            first_support: full_day,
            last_support,
            has_pending_pre_support_event,
            cached_last_day_index: Some(0),
        };
        assert!(rotation_day_boundary_is_complete_v1(boundary(
            full_day, false,
        )));
        assert!(!rotation_day_boundary_is_complete_v1(boundary(
            partial_day,
            false,
        )));
        assert!(!rotation_day_boundary_is_complete_v1(boundary(
            full_day, true,
        )));
        let mut mixed_day = boundary(full_day, false);
        mixed_day.all_resident_supports_match_day = false;
        assert!(!rotation_day_boundary_is_complete_v1(mixed_day));

        let mut tail = AcceptedPublicationTailAuthorityV1::default();
        tail.support_count = 1435;
        tail.event_count = 52;
        tail.last_day_index = Some(0);
        tail.last_interval_index = Some(47);
        tail.last_support = Some(full_day);
        tail.last_parent_transaction_id = Some(parent);
        tail.last_accepted_slab_sha256 = Some(Digest32::from_bytes([8; 32]));
        tail.traversed_ending_owner_sha256 = Some(Digest32::from_bytes([9; 32]));
        tail.aggregate_authority_sha256 = Digest32::from_bytes([10; 32]);
        tail.event_ids
            .insert(openwepp_coupled_time::ReceiptId::from_digest(
                Digest32::from_bytes([11; 32]),
            ));
        tail.last_event_ordinal_by_parent.insert(stale_parent, 3);
        tail.last_event_ordinal_by_parent.insert(parent, 4);
        let bounded = bounded_sealed_prefix_tail_v1(tail.clone());
        assert_eq!(bounded.support_count, tail.support_count);
        assert_eq!(bounded.event_count, tail.event_count);
        assert_eq!(
            bounded.traversed_ending_owner_sha256,
            tail.traversed_ending_owner_sha256
        );
        assert_eq!(
            bounded.aggregate_authority_sha256,
            tail.aggregate_authority_sha256
        );
        assert!(bounded.event_ids.is_empty());
        assert_eq!(bounded.last_event_ordinal_by_parent.len(), 1);
        assert_eq!(bounded.last_event_ordinal_by_parent.get(&parent), Some(&4));

        let checkpoint_bytes = b"canonical-materialized-wb14-checkpoint".to_vec();
        let checkpoint = PersistentCanonicalWb14ReplayV1::from_bytes(checkpoint_bytes.clone());
        let rotated_history = AcceptedPublicationHistoryV1 {
            inner: std::sync::Arc::new(AcceptedPublicationHistoryInnerV1 {
                supports: Vec::new(),
                event_handoffs: Vec::new(),
                sealed_prefix_tail: bounded.clone(),
                wb14_replay_checkpoint: Some(checkpoint.clone()),
                last_child_replay_materialized: Some(checkpoint_bytes.clone().into()),
                tail_authority: bounded.clone(),
            }),
        };
        assert_eq!(
            rotated_history
                .validate_cached_tail_against_full_scan()
                .expect("bounded rotated history"),
            bounded.traversed_ending_owner_sha256,
        );
        let residency = rotated_history.retention_state();
        assert_eq!(residency.resident_support_count(), 0);
        assert_eq!(residency.resident_event_count(), 0);
        assert_eq!(residency.sealed_support_count(), 1435);
        assert_eq!(residency.sealed_event_count(), 52);
        assert_eq!(
            residency.wb14_checkpoint_sha256(),
            Some(digest_bytes(&checkpoint_bytes))
        );
        assert_eq!(checkpoint.materialize(), checkpoint_bytes);

        let base = Stage3RotatedPublicationDayEvidenceV1 {
            day_index: 0,
            canonical_support_event_bytes: vec![1, 2, 3, 4],
            canonical_uncompressed_sha256: digest_bytes(&[1, 2, 3, 4]),
            support_count: 1435,
            event_count: 52,
            beginning_owner_set_sha256: Digest32::from_bytes([5; 32]),
            ending_owner_set_sha256: Digest32::from_bytes([9; 32]),
            last_support: full_day,
            last_parent_transaction_id: parent,
            last_accepted_slab_sha256: Digest32::from_bytes([8; 32]),
            tail_authority_sha256: Digest32::from_bytes([10; 32]),
        };
        let mut omitted = base.clone();
        omitted.canonical_support_event_bytes.remove(1);
        omitted.canonical_uncompressed_sha256 =
            digest_bytes(&omitted.canonical_support_event_bytes);
        assert_ne!(base, omitted);
        let mut substituted = base.clone();
        substituted.ending_owner_set_sha256 = Digest32::from_bytes([12; 32]);
        assert_ne!(base, substituted);
        let mut reordered = base.clone();
        reordered.canonical_support_event_bytes.swap(0, 1);
        reordered.canonical_uncompressed_sha256 =
            digest_bytes(&reordered.canonical_support_event_bytes);
        assert_ne!(base, reordered);
    }
}
