use openwepp_coupled_time::*;
use proptest::prelude::*;
fn d(v: u8) -> Digest32 {
    Digest32::from_bytes([v; 32])
}
fn t(v: u128) -> ModelTimeNs {
    ModelTimeNs::new(v)
}
fn owner(id: &str, state: &[u8]) -> OwnerState {
    OwnerState::new(id.into(), state.to_vec()).unwrap()
}
fn clock(end: u128) -> CoupledClockStateV1 {
    let owners = vec![owner("A", b"a"), owner("B", b"b")];
    let support = TimeSupport::new(t(0), t(end)).unwrap();
    let authority = ParentAuthorityV1::new(
        d(1),
        d(2),
        d(3),
        0,
        support,
        complete_owner_set_digest(&owners).unwrap(),
    )
    .unwrap();
    CoupledClockStateV1::new(
        authority,
        owners,
        "r0".into(),
        vec!["A".into()],
        d(4),
        vec![],
    )
    .unwrap()
}
fn constraint(clock: &CoupledClockStateV1, end: u128) -> ConstraintReductionReceiptV1 {
    let value = StepConstraintV1::new(
        clock_parent(clock),
        clock.accepted_until(),
        t(end),
        "A".into(),
        ConstraintClass::HardBoundary,
        d(6),
        d(2),
        d(3),
    )
    .unwrap();
    reduce_constraints(
        &[value],
        clock_parent(clock),
        clock.accepted_until(),
        t(10),
        None,
    )
    .unwrap()
}
fn clock_parent(clock: &CoupledClockStateV1) -> ParentTransactionId {
    let owners = complete_owner_set_digest(clock.owners()).unwrap();
    ParentTransactionId::derive(
        d(1),
        0,
        ParentIntervalId::derive(d(1), d(2), d(3), TimeSupport::new(t(0), t(10)).unwrap()).unwrap(),
        owners,
    )
    .unwrap()
}
fn segment(clock: &CoupledClockStateV1) -> SegmentId {
    SegmentId::derive(
        clock_parent(clock),
        0,
        TimeSupport::new(t(0), t(10)).unwrap(),
        digest_bytes(b"r0"),
        digest_bytes(b"A\0"),
    )
    .unwrap()
}

#[test]
fn canonical_tick_wire_rejects_numbers_and_leading_zero() {
    assert_eq!(serde_json::to_string(&t(12)).unwrap(), "\"12\"");
    assert!(serde_json::from_str::<ModelTimeNs>("12").is_err());
    assert!(serde_json::from_str::<ModelTimeNs>("\"012\"").is_err());
    assert!(serde_json::from_str::<TimeSupport>(r#"{"start_ns":"2","end_ns":"2"}"#).is_err());
}
#[test]
fn rejected_slab_is_atomic_and_authenticated() {
    let c = clock(10);
    let before = c.clone();
    let con = constraint(&c, 10);
    let bad = CoupledSlabCandidateV1::new(
        &c,
        segment(&c),
        TimeSupport::new(t(0), t(10)).unwrap(),
        &con,
        vec![owner("A", b"x"), owner("B", b"mutated inactive")],
        vec![LedgerEntryV1::new("water".into(), "kg".into(), d(7), d(7), d(8)).unwrap()],
    );
    assert_eq!(bad.unwrap_err(), CoupledTimeError::OwnerCandidate);
    assert_eq!(c, before);
}
#[test]
fn accepted_receipt_authenticates_reduction_and_atomic_parent_outbox() {
    let mut c = clock(10);
    let con = constraint(&c, 10);
    let candidate = CoupledSlabCandidateV1::new(
        &c,
        segment(&c),
        TimeSupport::new(t(0), t(10)).unwrap(),
        &con,
        vec![owner("A", b"x"), owner("B", b"b")],
        vec![LedgerEntryV1::new("water".into(), "kg".into(), d(7), d(7), d(8)).unwrap()],
    )
    .unwrap();
    let receipt = accept_slab(&mut c, candidate).unwrap();
    let mut reduction = DiagnosticReductionV1::new("peak".into(), "m3/s".into()).unwrap();
    reduction
        .fold_accepted_operand(4.0, AcceptedReductionOperandV1::from_slab(&receipt))
        .unwrap();
    assert!(reduction.fold_accepted(9.0, &receipt).is_err());
    let record = PublicationRecordV1::new(
        receipt.id(),
        receipt.support(),
        digest_bytes(b"4"),
        b"4".to_vec(),
        "m3/s".into(),
        "A".into(),
    )
    .unwrap();
    let candidate = ParentCommitCandidateV1::new(&c, vec![record]).unwrap();
    let commit = commit_parent(c, candidate).unwrap();
    assert!(ParentCommitCandidateV1::new(commit.clock(), vec![]).is_err());
    let _ = commit;
}
#[test]
fn event_queue_orders_and_replay_is_closed() {
    let mut c = clock(10);
    let event = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "A".into(),
        d(10),
        vec![owner("A", b"terminal"), owner("B", b"b")],
        vec!["A".into()],
        "r1".into(),
        vec!["B".into()],
        vec![LedgerEntryV1::new("melt".into(), "kg".into(), d(11), d(11), d(12)).unwrap()],
    )
    .unwrap();
    let mut q = EventQueueV1::new(t(0), vec![event]).unwrap();
    let receipt = q.apply_next(&mut c).unwrap().unwrap();
    assert_eq!(receipt.tick(), t(0));
    assert_eq!(receipt.ordinal(), 0);
    assert_eq!(receipt.event_context_digest(), d(10));
    assert_eq!(receipt, c.accepted_event_receipts()[0]);
    assert_eq!(
        receipt.ending_owner_set_digest(),
        complete_owner_set_digest(c.owners()).unwrap()
    );
    assert_eq!(receipt.parent_transaction_id(), c.parent_transaction_id());
    assert!(q.apply_next(&mut c).unwrap().is_none());
}

#[test]
fn event_transition_rejects_nonexistent_mutation_set_member() {
    let mut c = clock(10);
    let event = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "A".into(),
        d(10),
        vec![owner("A", b"terminal"), owner("B", b"b")],
        vec!["A".into(), "nonexistent".into()],
        "r1".into(),
        vec!["B".into()],
        vec![LedgerEntryV1::new("melt".into(), "kg".into(), d(11), d(11), d(12)).unwrap()],
    )
    .unwrap();
    let mut queue = EventQueueV1::new(t(0), vec![event]).unwrap();
    assert!(matches!(
        queue.apply_next(&mut c),
        Err(CoupledTimeError::EventTransition)
    ));
    assert!(c.accepted_event_receipts().is_empty());
    assert_eq!(c.event_ordinal(), 0);
}

#[test]
fn same_tick_event_proposals_chain_against_accepted_state() {
    let mut c = clock(10);
    let slab_constraint = constraint(&c, 5);
    let slab = CoupledSlabCandidateV1::new(
        &c,
        segment(&c),
        TimeSupport::new(t(0), t(5)).unwrap(),
        &slab_constraint,
        vec![owner("A", b"a"), owner("B", b"b")],
        vec![LedgerEntryV1::new("water".into(), "kg".into(), d(18), d(18), d(19)).unwrap()],
    )
    .unwrap();
    let slab_receipt = accept_slab(&mut c, slab).unwrap();
    let first = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "A".into(),
        d(20),
        vec![owner("A", b"terminal"), owner("B", b"b")],
        vec!["A".into()],
        "r1".into(),
        vec!["B".into()],
        vec![LedgerEntryV1::new("melt".into(), "kg".into(), d(21), d(21), d(22)).unwrap()],
    )
    .unwrap();
    let second = EventProposalV1::new(
        EventClass::BoundaryModeTransition,
        "B".into(),
        d(23),
        vec![owner("A", b"terminal"), owner("B", b"active")],
        vec!["B".into()],
        "r2".into(),
        vec!["B".into()],
        vec![LedgerEntryV1::new("mode".into(), "kg".into(), d(24), d(24), d(25)).unwrap()],
    )
    .unwrap();
    let mut queue = EventQueueV1::new(t(5), vec![second, first]).unwrap();
    let a = queue.apply_next(&mut c).unwrap().unwrap();
    let b = queue.apply_next(&mut c).unwrap().unwrap();
    assert_ne!(a.id(), b.id());
    assert!(queue.apply_next(&mut c).unwrap().is_none());
    let scheduled_id = c
        .record_scheduled_once("daily".into(), t(5), d(26))
        .unwrap();
    let scheduled = c
        .scheduled_once_receipts()
        .iter()
        .find(|receipt| receipt.id() == scheduled_id)
        .unwrap();
    let mut sum = DiagnosticReductionV1::new_sum("sum".into(), "kg".into()).unwrap();
    sum.fold_accepted_operand(0.5, AcceptedReductionOperandV1::from_slab(&slab_receipt))
        .unwrap();
    sum.fold_accepted_operand(1.25, AcceptedReductionOperandV1::from_event(&a))
        .unwrap();
    sum.fold_accepted_operand(2.5, AcceptedReductionOperandV1::from_event(&b))
        .unwrap();
    sum.fold_accepted_operand(4.0, AcceptedReductionOperandV1::from_scheduled(scheduled))
        .unwrap();
    assert_eq!(sum.maximum(), Some(8.25));
    let mut signed_zero = DiagnosticReductionV1::new_sum("zero".into(), "kg".into()).unwrap();
    signed_zero
        .fold_accepted_operand(-0.0, AcceptedReductionOperandV1::from_event(&a))
        .unwrap();
    assert_eq!(signed_zero.maximum().unwrap().to_bits(), 0.0_f64.to_bits());
    let restart = CoupledTimeRestartV2::new(d(50), d(51), c, sum, None, vec![]).unwrap();
    let bytes = restart.to_canonical_json().unwrap();
    let restored = CoupledTimeRestartV2::from_canonical_json(&bytes, d(50), d(51), d(4)).unwrap();
    let (_, reductions, _, _) = restored.into_parts();
    assert_eq!(
        reductions[0].maximum().unwrap().to_bits(),
        8.25_f64.to_bits()
    );
}

#[test]
fn coincident_constraints_and_pending_event_are_authenticated() {
    let c = clock(10);
    let hard = StepConstraintV1::new(
        clock_parent(&c),
        t(0),
        t(5),
        "A".into(),
        ConstraintClass::HardBoundary,
        d(30),
        d(2),
        d(3),
    )
    .unwrap();
    let output = StepConstraintV1::new(
        clock_parent(&c),
        t(0),
        t(5),
        "B".into(),
        ConstraintClass::OutputBoundary,
        d(30),
        d(2),
        d(3),
    )
    .unwrap();
    let receipt = reduce_constraints(&[output, hard], clock_parent(&c), t(0), t(10), None).unwrap();
    assert_eq!(receipt.proposed_end(), t(5));
    let zero = StepConstraintV1::new(
        clock_parent(&c),
        t(0),
        t(0),
        "A".into(),
        ConstraintClass::EventBoundary,
        d(30),
        d(2),
        d(3),
    )
    .unwrap();
    let proposal = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "A".into(),
        d(31),
        vec![owner("A", b"terminal"), owner("B", b"b")],
        vec!["A".into()],
        "r1".into(),
        vec!["B".into()],
        vec![LedgerEntryV1::new("event".into(), "kg".into(), d(32), d(32), d(33)).unwrap()],
    )
    .unwrap();
    let queue = EventQueueV1::new(t(0), vec![proposal]).unwrap();
    let join = queue.pending_event_join(&c).unwrap().unwrap();
    assert!(
        reduce_constraints(
            std::slice::from_ref(&zero),
            clock_parent(&c),
            t(0),
            t(10),
            None
        )
        .is_err()
    );
    assert!(reduce_constraints(&[zero], clock_parent(&c), t(0), t(10), Some(&join)).is_ok());
}
#[test]
fn canonical_restart_roundtrip_and_poison_rejection() {
    let c = clock(10);
    let restart = CoupledTimeRestartV2::new(
        d(20),
        d(21),
        c,
        DiagnosticReductionV1::new("peak".into(), "m3/s".into()).unwrap(),
        None,
        vec![],
    )
    .unwrap();
    let bytes = restart.to_canonical_json().unwrap();
    assert!(CoupledTimeRestartV2::from_canonical_json(&bytes, d(20), d(21), d(4)).is_ok());
    assert!(CoupledTimeRestartV2::from_canonical_json(&bytes, d(20), d(22), d(4)).is_err());
    let mut poison = bytes.clone();
    poison.push(b' ');
    assert!(CoupledTimeRestartV2::from_canonical_json(&poison, d(20), d(21), d(4)).is_err());
}

#[test]
fn scheduled_execution_key_rejects_different_result_replay() {
    let mut c = clock(10);
    c.record_scheduled_once("daily".into(), t(0), d(40))
        .unwrap();
    assert_eq!(
        c.record_scheduled_once("daily".into(), t(0), d(41))
            .unwrap_err(),
        CoupledTimeError::ScheduledOnceReplay
    );
}

#[test]
fn empty_sum_roundtrips_as_null_and_rejects_zero_sentinel() {
    let restart = CoupledTimeRestartV2::new(
        d(60),
        d(61),
        clock(10),
        DiagnosticReductionV1::new_sum("sum".into(), "kg".into()).unwrap(),
        None,
        vec![],
    )
    .unwrap();
    let bytes = restart.to_canonical_json().unwrap();
    assert!(
        bytes
            .windows(b"\"value_bits\":null".len())
            .any(|w| w == b"\"value_bits\":null")
    );
    let restored = CoupledTimeRestartV2::from_canonical_json(&bytes, d(60), d(61), d(4)).unwrap();
    let (_, reductions, _, _) = restored.into_parts();
    assert_eq!(reductions[0].maximum(), None);
    let text = String::from_utf8(bytes).unwrap();
    let poison = text.replace("\"value_bits\":null", "\"value_bits\":\"0000000000000000\"");
    assert!(
        CoupledTimeRestartV2::from_canonical_json(poison.as_bytes(), d(60), d(61), d(4)).is_err()
    );
}
#[test]
fn identity_kat_is_stable() {
    let support = TimeSupport::new(t(0), t(1_800_000_000_000)).unwrap();
    let a = ParentIntervalId::derive(d(1), d(2), d(3), support).unwrap();
    let b = ParentIntervalId::derive(d(1), d(2), d(3), support).unwrap();
    assert_eq!(a, b);
    assert_ne!(
        a,
        ParentIntervalId::derive(d(1), d(2), d(4), support).unwrap()
    );
}
proptest! {#[test]fn support_duration(start in any::<u64>(),len in 1_u64..u64::MAX){let s=TimeSupport::new(t(u128::from(start)),t(u128::from(start)+u128::from(len))).unwrap();prop_assert_eq!(s.duration_ns(),u128::from(len));}}
