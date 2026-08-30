#[cfg(test)]
mod subslab_owner_adjacency_tests {
    use super::*;

    fn digest(byte: u8) -> Digest32 {
        Digest32::from_bytes([byte; 32])
    }

    fn handoff(
        id: u8,
        parent: u8,
        ordinal: u32,
        beginning: u8,
        ending: u8,
    ) -> AcceptedOwnerHandoffChronologyV1 {
        AcceptedOwnerHandoffChronologyV1 {
            id: openwepp_coupled_time::ReceiptId::from_digest(digest(id)),
            parent_transaction_id: ParentTransactionId::from_digest(digest(parent)),
            tick: ModelTimeNs::new(10),
            ordinal,
            beginning_owner_sha256: digest(beginning),
            ending_owner_sha256: digest(ending),
        }
    }

    fn validate(handoffs: &[AcceptedOwnerHandoffChronologyV1]) -> bool {
        validate_subslab_owner_adjacency_v1(
            handoffs,
            ModelTimeNs::new(10),
            digest(1),
            digest(2),
            digest(3),
            digest(5),
        )
        .is_ok()
    }

    fn support(start_ns: u128, end_ns: u128) -> TimeSupport {
        TimeSupport::new(ModelTimeNs::new(start_ns), ModelTimeNs::new(end_ns))
            .expect("positive support")
    }

    fn validate_retained_pair(
        preceding: TimeSupport,
        following: TimeSupport,
        handoffs: &[AcceptedOwnerHandoffChronologyV1],
    ) -> bool {
        validate_retained_subslab_pair_after_complete_publication_v1(
            handoffs,
            preceding,
            following,
            digest(1),
            digest(2),
            digest(3),
            digest(5),
        )
        .is_ok()
    }

    #[test]
    fn pre_support_event_bridges_subslabs_and_poisons_fail_closed() {
        let event = handoff(10, 2, 0, 3, 5);
        assert!(validate(&[event]));
        assert!(!validate(&[]), "omitted owner handoff");
        assert!(!validate(&[handoff(10, 2, 0, 4, 5)]), "stale predecessor");
        assert!(
            !validate(&[handoff(10, 2, 0, 3, 4)]),
            "substituted ending owner"
        );
        assert!(
            !validate(&[handoff(10, 4, 0, 3, 5)]),
            "cross-parent handoff"
        );
        let ordered = [handoff(10, 2, 0, 3, 4), handoff(11, 2, 1, 4, 5)];
        assert!(validate(&ordered));
        assert!(!validate(&[ordered[1], ordered[0]]), "reordered handoffs");
        let duplicate = [handoff(10, 2, 0, 3, 4), handoff(10, 2, 1, 4, 5)];
        assert!(!validate(&duplicate), "duplicate accepted-event receipt id");
    }

    #[test]
    fn sparse_retained_subslabs_defer_only_sealed_gap_to_complete_publication_history() {
        let preceding = support(0, 10);
        let following = support(20, 30);
        assert!(
            validate_retained_pair(preceding, following, &[]),
            "a snow-free gap is owned by the already-validated complete publication history",
        );

        let touching = support(10, 20);
        assert!(validate_retained_pair(
            preceding,
            touching,
            &[handoff(10, 2, 0, 3, 5)],
        ));
        assert!(
            !validate_retained_pair(preceding, touching, &[]),
            "touching subslabs may not omit their owner handoff",
        );
        assert!(
            !validate_retained_pair(preceding, touching, &[handoff(10, 2, 0, 3, 4)],),
            "touching subslabs may not substitute the following owner",
        );
        assert!(
            !validate_retained_pair(support(0, 11), support(10, 20), &[]),
            "overlapping retained subslabs remain fail-closed",
        );
        assert!(
            !validate_retained_pair(support(20, 30), support(10, 20), &[]),
            "reordered retained subslabs remain fail-closed",
        );
    }

    #[test]
    fn parent_receipt_snow_owner_uses_exact_coupled_authority_and_rejects_omission() {
        let coupled = [openwepp_coupled_time::OwnerState::new(
            "snow".to_owned(),
            b"receipt-bearing-v4-snow-owner".to_vec(),
        )
        .expect("snow owner")];
        let mut bytes = BTreeMap::new();
        bind_parent_receipt_snow_owner_bytes_v1(&mut bytes, &coupled)
            .expect("coupled snow projection");
        assert_eq!(
            bytes.get("snow").map(Vec::as_slice),
            Some(b"receipt-bearing-v4-snow-owner".as_slice()),
        );

        let substituted = [openwepp_coupled_time::OwnerState::new(
            "snow".to_owned(),
            b"substituted-snow-owner".to_vec(),
        )
        .expect("substituted snow owner")];
        let mut substituted_bytes = BTreeMap::new();
        bind_parent_receipt_snow_owner_bytes_v1(&mut substituted_bytes, &substituted)
            .expect("substituted projection");
        assert_ne!(
            bytes, substituted_bytes,
            "snow owner substitution must be visible"
        );

        assert!(
            bind_parent_receipt_snow_owner_bytes_v1(&mut BTreeMap::new(), &[]).is_err(),
            "omitted coupled snow owner",
        );
    }
}
