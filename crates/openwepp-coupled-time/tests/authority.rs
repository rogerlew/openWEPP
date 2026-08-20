use openwepp_coupled_time::*;
use proptest::prelude::*;

fn d(byte: u8) -> Digest32 {
    Digest32([byte; 32])
}
fn owner(id: &str, state: &[u8]) -> OwnerState {
    OwnerState {
        owner_id: id.into(),
        state_bytes: state.to_vec(),
        state_digest: digest_bytes(state),
    }
}

#[test]
fn support_is_positive_and_duration_bits_are_common() {
    assert_eq!(
        TimeSupport::new(ModelTimeNs(2), ModelTimeNs(2)),
        Err(CoupledTimeError::InvalidSupport)
    );
    let s = TimeSupport::new(ModelTimeNs(0), ModelTimeNs(1_800_000_000_000)).unwrap();
    assert_eq!(s.duration_s_bits(), 1800.0_f64.to_bits());
}

#[test]
fn event_quantization_is_ties_even_and_checked() {
    assert_eq!(
        quantize_seconds_to_tick(ModelTimeNs(0), ModelTimeNs(10), 0.0).unwrap(),
        ModelTimeNs(0)
    );
    assert_eq!(
        quantize_seconds_to_tick(ModelTimeNs(0), ModelTimeNs(10), 1.0e-9).unwrap(),
        ModelTimeNs(1)
    );
    assert!(quantize_seconds_to_tick(ModelTimeNs(0), ModelTimeNs(10), f64::NAN).is_err());
}

#[test]
fn rejected_candidate_leaves_clock_byte_identical() {
    let support = TimeSupport::new(ModelTimeNs(0), ModelTimeNs(10)).unwrap();
    let mut clock = CoupledClockStateV1::new(
        ParentIntervalId(d(1)),
        ParentTransactionId(d(2)),
        support,
        vec![owner("A", b"a"), owner("B", b"b")],
        "snow".into(),
        vec!["A".into()],
        d(3),
        vec![],
    )
    .unwrap();
    let before = clock.clone();
    let slab = CoupledSlabCandidateV1 {
        accepted_slab_id: AcceptedSlabId(d(4)),
        support,
        duration_s_bits: support.duration_s_bits(),
        candidates: vec![OwnerCandidateV1 {
            owner_id: "A".into(),
            beginning_state_digest: digest_bytes(b"a"),
            ending_state_bytes: b"x".to_vec(),
            ending_state_digest: digest_bytes(b"x"),
            ledger_digest: d(5),
        }],
        global_ledger_digest: d(6),
        ledgers_closed: false,
        receipt_id: ReceiptId(d(7)),
    };
    assert_eq!(
        accept_slab(&mut clock, &slab),
        Err(CoupledTimeError::LedgerFailure)
    );
    assert_eq!(clock, before);
}

#[test]
fn event_changes_participants_without_advancing_time_and_cannot_replay() {
    let support = TimeSupport::new(ModelTimeNs(0), ModelTimeNs(10)).unwrap();
    let mut clock = CoupledClockStateV1::new(
        ParentIntervalId(d(1)),
        ParentTransactionId(d(2)),
        support,
        vec![owner("A", b"a"), owner("B", b"b"), owner("C", b"c")],
        "snow".into(),
        vec!["A".into(), "B".into()],
        d(3),
        vec![],
    )
    .unwrap();
    let event = EventTransitionV1 {
        event_id: EventId(d(4)),
        tick_ns: ModelTimeNs(0),
        class: EventClass::OwnershipTransfer,
        source_owner_id: "B".into(),
        event_context_digest: d(5),
        beginning_owner_set_digest: d(6),
        ending_owners: vec![
            owner("A", b"a"),
            owner("B", b"terminal"),
            owner("C", b"liquid"),
        ],
        successor_regime_id: "snow-free".into(),
        successor_participants: vec!["A".into(), "C".into()],
        ledger_digest: d(7),
        ledger_closed: true,
        receipt_id: ReceiptId(d(8)),
    };
    apply_event(&mut clock, &event).unwrap();
    assert_eq!(clock.accepted_until, ModelTimeNs(0));
    assert_eq!(clock.active_participant_set, ["A", "C"]);
    let before = clock.clone();
    assert_eq!(
        apply_event(&mut clock, &event),
        Err(CoupledTimeError::EventTransition)
    );
    assert_eq!(clock, before);
}

proptest! {
    #[test]
    fn support_duration_exact_for_all_ordered_ticks(start in any::<u64>(), len in 1_u64..u64::MAX) {
        let end = u128::from(start) + u128::from(len);
        let support = TimeSupport::new(ModelTimeNs(u128::from(start)), ModelTimeNs(end)).unwrap();
        prop_assert_eq!(support.duration_ns(), u128::from(len));
    }
}
