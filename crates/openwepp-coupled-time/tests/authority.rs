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

#[test]
fn pristine_restart_roundtrip_preserves_absent_last_step_marker() {
    let restart = CoupledTimeRestartV2::new(
        d(90),
        d(91),
        clock(10),
        DiagnosticReductionV1::new("pristine".into(), "1".into()).unwrap(),
        None,
        vec![],
    )
    .unwrap();
    let bytes = restart.to_canonical_json().unwrap();
    let restored = CoupledTimeRestartV2::from_canonical_json(&bytes, d(90), d(91), d(4)).unwrap();
    assert_eq!(restored, restart);
    assert_eq!(restored.to_canonical_json().unwrap(), bytes);
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

fn candidate_for_initial_clock(c: &CoupledClockStateV1) -> CoupledSlabCandidateV1 {
    let con = constraint(c, 10);
    CoupledSlabCandidateV1::new(
        c,
        segment(c),
        TimeSupport::new(t(0), t(10)).unwrap(),
        &con,
        vec![owner("A", b"accepted"), owner("B", b"b")],
        vec![LedgerEntryV1::new("water".into(), "kg".into(), d(80), d(80), d(81)).unwrap()],
    )
    .unwrap()
}

#[test]
fn validated_slab_proof_accepts_on_an_exact_in_process_clock_clone() {
    let source = clock(10);
    let candidate = candidate_for_initial_clock(&source);
    let mut exact_clone = source.clone();

    let receipt = accept_slab(&mut exact_clone, candidate).unwrap();

    assert_eq!(receipt.support(), TimeSupport::new(t(0), t(10)).unwrap());
    assert_eq!(source.accepted_until(), t(0));
    assert_eq!(exact_clone.accepted_until(), t(10));
}

#[test]
fn validated_slab_proof_rejects_an_independently_constructed_equivalent_clock_atomically() {
    let source = clock(10);
    let candidate = candidate_for_initial_clock(&source);
    let mut foreign = clock(10);
    assert_eq!(foreign, source);
    let before = foreign.clone();

    assert_eq!(
        accept_slab(&mut foreign, candidate),
        Err(CoupledTimeError::OwnerCandidate)
    );
    assert_eq!(foreign, before);
}

#[test]
fn validated_slab_proof_rejects_segment_and_scheduled_once_revision_changes_atomically() {
    let mut changed_segment = clock(10);
    let segment_candidate = candidate_for_initial_clock(&changed_segment);
    changed_segment.admit_active_segment_end(t(5)).unwrap();
    let segment_before = changed_segment.clone();
    assert_eq!(
        accept_slab(&mut changed_segment, segment_candidate),
        Err(CoupledTimeError::ParentMismatch)
    );
    assert_eq!(changed_segment, segment_before);

    let mut changed_scheduled = clock(10);
    let scheduled_candidate = candidate_for_initial_clock(&changed_scheduled);
    changed_scheduled
        .record_scheduled_once("daily".into(), t(0), d(82))
        .unwrap();
    let scheduled_before = changed_scheduled.clone();
    assert_eq!(
        accept_slab(&mut changed_scheduled, scheduled_candidate),
        Err(CoupledTimeError::OwnerCandidate)
    );
    assert_eq!(changed_scheduled, scheduled_before);

    let mut scheduled_left = clock(10);
    let mut scheduled_right = scheduled_left.clone();
    scheduled_left
        .record_scheduled_once("left".into(), t(0), d(87))
        .unwrap();
    scheduled_right
        .record_scheduled_once("right".into(), t(0), d(88))
        .unwrap();
    let divergent_candidate = candidate_for_initial_clock(&scheduled_left);
    let divergent_before = scheduled_right.clone();
    assert_eq!(
        accept_slab(&mut scheduled_right, divergent_candidate),
        Err(CoupledTimeError::OwnerCandidate)
    );
    assert_eq!(scheduled_right, divergent_before);
}

#[test]
fn validated_slab_proof_rejects_reuse_after_prior_slab_acceptance_atomically() {
    let mut c = clock(10);
    let candidate = candidate_for_initial_clock(&c);
    accept_slab(&mut c, candidate.clone()).unwrap();
    let before = c.clone();

    assert_eq!(
        accept_slab(&mut c, candidate),
        Err(CoupledTimeError::ParentMismatch)
    );
    assert_eq!(c, before);
}

#[test]
fn restart_invalidates_pre_restart_proof_and_fresh_validation_succeeds() {
    let source = clock(10);
    let pre_restart_candidate = candidate_for_initial_clock(&source);
    let restart = CoupledTimeRestartV2::new(
        d(83),
        d(84),
        source,
        DiagnosticReductionV1::new("proof".into(), "1".into()).unwrap(),
        None,
        vec![],
    )
    .unwrap();
    let bytes = restart.to_canonical_json().unwrap();
    let restored = CoupledTimeRestartV2::from_canonical_json(&bytes, d(83), d(84), d(4)).unwrap();
    let (mut restored_clock, _, _, _) = restored.into_parts();
    let before = restored_clock.clone();

    assert_eq!(
        accept_slab(&mut restored_clock, pre_restart_candidate),
        Err(CoupledTimeError::OwnerCandidate)
    );
    assert_eq!(restored_clock, before);

    let fresh = candidate_for_initial_clock(&restored_clock);
    accept_slab(&mut restored_clock, fresh).unwrap();
    assert_eq!(restored_clock.accepted_until(), t(10));
}

#[test]
fn live_validation_authority_is_omitted_from_clock_candidate_and_restart_wire() {
    let c = clock(10);
    let candidate = candidate_for_initial_clock(&c);
    let clock_json = serde_json::to_string(&c).unwrap();
    let candidate_json = serde_json::to_string(&candidate).unwrap();
    assert!(!clock_json.contains("incarnation"));
    assert!(!candidate_json.contains("validation_proof"));

    let restart = CoupledTimeRestartV2::new(
        d(85),
        d(86),
        c,
        DiagnosticReductionV1::new("wire".into(), "1".into()).unwrap(),
        None,
        vec![],
    )
    .unwrap();
    let bytes = restart.to_canonical_json().unwrap();
    let text = std::str::from_utf8(&bytes).unwrap();
    assert!(!text.contains("incarnation"));
    assert!(!text.contains("validation_proof"));
}

#[test]
fn slab_acceptance_source_consumes_the_proof_without_reconstruction_or_payload_clone() {
    let source = include_str!("../src/transaction.rs");
    let start = source.find("pub fn accept_slab(").unwrap();
    let tail = &source[start..];
    let end = tail
        .find("\n#[derive(Debug, Clone, PartialEq, Serialize)]")
        .unwrap();
    let body = &tail[..end];
    for forbidden in [
        "CoupledSlabCandidateV1::new",
        "owner_set_digest(",
        "ledger_digest(",
        "serde_json",
        ".clone()",
        ".accepted_slab_receipts\n        .iter()",
    ] {
        assert!(
            !body.contains(forbidden),
            "accept_slab reintroduced forbidden duplicate work: {forbidden}"
        );
    }
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

fn clock_after_receipt_bearing_ownership_noop() -> CoupledClockStateV1 {
    let mut c = clock(10);
    let predecessor_segment = c.active_segment_id();
    let proposal = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "A".into(),
        d(110),
        c.owners().to_vec(),
        vec![],
        "r0".into(),
        vec!["A".into()],
        vec![
            LedgerEntryV1::new(
                "runoff-custody".into(),
                "kg-and-j".into(),
                d(111),
                d(111),
                d(112),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let mut queue = EventQueueV1::new(t(0), vec![proposal]).unwrap();
    let accepted = queue.apply_next(&mut c).unwrap().unwrap();
    assert_eq!(accepted.ordinal(), 0);
    assert_eq!(c.event_ordinal(), 1);
    assert_ne!(c.active_segment_id(), predecessor_segment);
    assert_eq!(c.accepted_event_receipts(), &[accepted]);
    assert!(queue.apply_next(&mut c).unwrap().is_none());
    c
}

fn assert_missing_custody_rejects(c: &mut CoupledClockStateV1) {
    let false_noop = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "A".into(),
        d(113),
        c.owners().to_vec(),
        vec![],
        "r0".into(),
        vec!["A".into()],
        vec![
            LedgerEntryV1::new(
                "missing-custody".into(),
                "kg-and-j".into(),
                Digest32::zero(),
                Digest32::zero(),
                d(114),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    assert_eq!(
        EventQueueV1::new(t(0), vec![false_noop])
            .unwrap()
            .apply_next(c)
            .unwrap_err(),
        CoupledTimeError::EventCycle,
    );
    assert_eq!(c.event_ordinal(), 1);
}

fn assert_zero_lineage_rejects(c: &mut CoupledClockStateV1) {
    let zero_lineage = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "A".into(),
        d(115),
        c.owners().to_vec(),
        vec![],
        "r0".into(),
        vec!["A".into()],
        vec![
            LedgerEntryV1::new(
                "zero-lineage".into(),
                "kg-and-j".into(),
                d(116),
                d(116),
                Digest32::zero(),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    assert_eq!(
        EventQueueV1::new(t(0), vec![zero_lineage])
            .unwrap()
            .apply_next(c)
            .unwrap_err(),
        CoupledTimeError::EventCycle,
    );
}

fn assert_wrong_class_rejects(c: &mut CoupledClockStateV1) {
    let wrong_class = EventProposalV1::new(
        EventClass::DiagnosticMarker,
        "A".into(),
        d(117),
        c.owners().to_vec(),
        vec![],
        "r0".into(),
        vec!["A".into()],
        vec![
            LedgerEntryV1::new(
                "wrong-class".into(),
                "kg-and-j".into(),
                d(118),
                d(118),
                d(119),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    assert_eq!(
        EventQueueV1::new(t(0), vec![wrong_class])
            .unwrap()
            .apply_next(c)
            .unwrap_err(),
        CoupledTimeError::EventCycle,
    );
}

#[test]
fn receipt_bearing_ownership_noop_advances_once_and_false_noops_reject() {
    let mut c = clock_after_receipt_bearing_ownership_noop();
    assert_missing_custody_rejects(&mut c);
    assert_zero_lineage_rejects(&mut c);
    assert_wrong_class_rejects(&mut c);
    assert_eq!(c.event_ordinal(), 1);
}

#[test]
fn parent_end_event_installs_typed_boundary_without_successor_support() {
    let mut c = clock(10);
    let con = constraint(&c, 10);
    let slab = CoupledSlabCandidateV1::new(
        &c,
        segment(&c),
        TimeSupport::new(t(0), t(10)).unwrap(),
        &con,
        vec![owner("A", b"a"), owner("B", b"b")],
        vec![LedgerEntryV1::new("water".into(), "kg".into(), d(13), d(13), d(14)).unwrap()],
    )
    .unwrap();
    accept_slab(&mut c, slab).unwrap();
    let predecessor_owner_set = complete_owner_set_digest(c.owners()).unwrap();
    let expected_boundary = SegmentId::derive_terminal_event_boundary(
        c.parent_transaction_id(),
        c.parent_support(),
        t(10),
        0,
        predecessor_owner_set,
    )
    .unwrap();
    let event = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "A".into(),
        d(15),
        vec![owner("A", b"terminal"), owner("B", b"b")],
        vec!["A".into()],
        "terminal".into(),
        vec!["B".into()],
        vec![LedgerEntryV1::new("melt".into(), "kg".into(), d(16), d(16), d(17)).unwrap()],
    )
    .unwrap();
    let receipt = EventQueueV1::new(t(10), vec![event])
        .unwrap()
        .apply_next(&mut c)
        .unwrap()
        .unwrap();
    assert_eq!(receipt.tick(), t(10));
    assert!(c.is_complete());
    assert_eq!(c.active_segment_id(), expected_boundary);

    let restart = CoupledTimeRestartV2::new(
        d(18),
        d(19),
        c,
        DiagnosticReductionV1::new("terminal".into(), "1".into()).unwrap(),
        None,
        vec![],
    )
    .unwrap();
    let bytes = restart.to_canonical_json().unwrap();
    let restored = CoupledTimeRestartV2::from_canonical_json(&bytes, d(18), d(19), d(4)).unwrap();
    assert_eq!(restored.to_canonical_json().unwrap(), bytes);

    let text = String::from_utf8(bytes).unwrap();
    let boundary = serde_json::to_string(&expected_boundary).unwrap();
    let malformed_boundary = serde_json::to_string(&SegmentId::from_digest(d(20))).unwrap();
    let malformed = text.replacen(
        &format!("\"segment_id\":{boundary}"),
        &format!("\"segment_id\":{malformed_boundary}"),
        1,
    );
    assert_ne!(malformed, text);
    assert!(
        CoupledTimeRestartV2::from_canonical_json(malformed.as_bytes(), d(18), d(19), d(4),)
            .is_err()
    );
    let nonterminal = text.replacen(
        "\"start_ns\":\"10\",\"end_ns\":\"10\"",
        "\"start_ns\":\"9\",\"end_ns\":\"9\"",
        1,
    );
    assert_ne!(nonterminal, text);
    assert!(
        CoupledTimeRestartV2::from_canonical_json(nonterminal.as_bytes(), d(18), d(19), d(4),)
            .is_err()
    );
}

#[test]
fn terminal_event_boundary_identity_rejects_nonterminal_tick_and_binds_every_authority() {
    let support = TimeSupport::new(t(0), t(10)).unwrap();
    let parent = ParentTransactionId::from_digest(d(30));
    let owner_set = d(31);
    assert!(matches!(
        SegmentId::derive_terminal_event_boundary(parent, support, t(0), 0, owner_set),
        Err(CoupledTimeError::InvalidSupport)
    ));
    let canonical =
        SegmentId::derive_terminal_event_boundary(parent, support, t(10), 0, owner_set).unwrap();
    let changed_parent = SegmentId::derive_terminal_event_boundary(
        ParentTransactionId::from_digest(d(32)),
        support,
        t(10),
        0,
        owner_set,
    )
    .unwrap();
    let shifted_support = TimeSupport::new(t(1), t(10)).unwrap();
    let changed_support =
        SegmentId::derive_terminal_event_boundary(parent, shifted_support, t(10), 0, owner_set)
            .unwrap();
    let changed_ordinal =
        SegmentId::derive_terminal_event_boundary(parent, support, t(10), 1, owner_set).unwrap();
    let changed_owner =
        SegmentId::derive_terminal_event_boundary(parent, support, t(10), 0, d(33)).unwrap();
    for poison in [
        changed_parent,
        changed_support,
        changed_ordinal,
        changed_owner,
    ] {
        assert_ne!(canonical, poison);
    }
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
fn accepted_event_receipt_validates_exact_retained_ledger_entries() {
    let mut c = clock(10);
    let slab_constraint = constraint(&c, 10);
    let slab = CoupledSlabCandidateV1::new(
        &c,
        segment(&c),
        TimeSupport::new(t(0), t(10)).unwrap(),
        &slab_constraint,
        vec![owner("A", b"a"), owner("B", b"b")],
        vec![LedgerEntryV1::new("water".into(), "kg".into(), d(18), d(18), d(19)).unwrap()],
    )
    .unwrap();
    accept_slab(&mut c, slab).unwrap();

    let retained_ledger =
        vec![LedgerEntryV1::new("handoff".into(), "kg".into(), d(20), d(20), d(21)).unwrap()];
    let event = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "A".into(),
        d(22),
        vec![owner("A", b"accepted"), owner("B", b"b")],
        vec!["A".into()],
        "accepted".into(),
        vec!["B".into()],
        retained_ledger.clone(),
    )
    .unwrap();
    let accepted = EventQueueV1::new(t(10), vec![event])
        .unwrap()
        .apply_next(&mut c)
        .unwrap()
        .unwrap();

    accepted.validate_ledger_entries(&retained_ledger).unwrap();
    let substituted =
        vec![LedgerEntryV1::new("handoff".into(), "kg".into(), d(20), d(20), d(23)).unwrap()];
    assert_eq!(
        accepted.validate_ledger_entries(&substituted),
        Err(CoupledTimeError::EventTransition),
    );
    assert_eq!(
        accepted.validate_ledger_entries(&[]),
        Err(CoupledTimeError::LedgerFailure),
    );
    let extra = vec![
        LedgerEntryV1::new("handoff".into(), "kg".into(), d(20), d(20), d(21)).unwrap(),
        LedgerEntryV1::new("runoff".into(), "kg".into(), d(24), d(24), d(25)).unwrap(),
    ];
    assert_eq!(
        accepted.validate_ledger_entries(&extra),
        Err(CoupledTimeError::EventTransition),
    );
    let reordered = vec![extra[1].clone(), extra[0].clone()];
    assert_eq!(
        accepted.validate_ledger_entries(&reordered),
        Err(CoupledTimeError::LedgerFailure),
    );
}

#[test]
fn same_tick_accepted_event_order_is_ordinal_not_context_digest_order() {
    let mut c = clock(10);
    let slab_constraint = constraint(&c, 10);
    let slab = CoupledSlabCandidateV1::new(
        &c,
        segment(&c),
        TimeSupport::new(t(0), t(10)).unwrap(),
        &slab_constraint,
        vec![owner("A", b"a"), owner("B", b"b")],
        vec![LedgerEntryV1::new("water".into(), "kg".into(), d(70), d(70), d(71)).unwrap()],
    )
    .unwrap();
    accept_slab(&mut c, slab).unwrap();

    for (context, state, ledger) in [
        (d(200), b"first".as_slice(), d(72)),
        (d(100), b"second".as_slice(), d(73)),
    ] {
        let event = EventProposalV1::new(
            EventClass::OwnershipTransfer,
            "A".into(),
            context,
            vec![owner("A", state), owner("B", b"b")],
            vec!["A".into()],
            "same-tick".into(),
            vec!["B".into()],
            vec![LedgerEntryV1::new("handoff".into(), "kg".into(), ledger, ledger, d(74)).unwrap()],
        )
        .unwrap();
        EventQueueV1::new(t(10), vec![event])
            .unwrap()
            .apply_next(&mut c)
            .unwrap()
            .unwrap();
    }
    assert_eq!(
        c.accepted_event_receipts()[0].event_context_digest(),
        d(200)
    );
    assert_eq!(
        c.accepted_event_receipts()[1].event_context_digest(),
        d(100)
    );

    let restart = CoupledTimeRestartV2::new(
        d(75),
        d(76),
        c,
        DiagnosticReductionV1::new("same-tick".into(), "1".into()).unwrap(),
        None,
        vec![],
    )
    .unwrap();
    let bytes = restart.to_canonical_json().unwrap();
    let restored = CoupledTimeRestartV2::from_canonical_json(&bytes, d(75), d(76), d(4)).unwrap();
    assert_eq!(restored.to_canonical_json().unwrap(), bytes);

    let mut reordered: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    reordered["accepted_event_receipts"]
        .as_array_mut()
        .unwrap()
        .swap(0, 1);
    assert!(
        CoupledTimeRestartV2::from_canonical_json(
            &serde_json::to_vec(&reordered).unwrap(),
            d(75),
            d(76),
            d(4),
        )
        .is_err()
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
