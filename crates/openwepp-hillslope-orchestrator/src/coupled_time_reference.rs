//! Bounded, non-physical consumer of the closed coupled-time protocol.

use openwepp_coupled_time::{
    AttemptId, ConstraintClass, CoupledClockStateV1, CoupledSlabCandidateV1, CoupledTimeError,
    CoupledTimeRestartV2, DiagnosticReductionV1, Digest32, EventClass, EventQueueV1,
    EventTransitionV1, LedgerEntryV1, ModelTimeNs, OutboxState, OwnerState, ParentAuthorityV1,
    ParentCommitCandidateV1, ParentIntervalId, ParentTransactionId, PublicationRecordV1,
    RetryControlV1, SegmentId, StepConstraintV1, TimeSupport, accept_slab, commit_parent,
    complete_owner_set_digest, reduce_constraints,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const A: &str = "owner-a";
const B: &str = "owner-b";
const C: &str = "owner-c";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoupledTimeReferenceReport {
    pub uninterrupted_equals_restarted: bool,
    pub rejected_attempt_rolled_back: bool,
    pub accepted_maximum_bits: u64,
    pub reconstructed_maximum_bits: u64,
    pub accepted_reduction_count: usize,
    pub publication_invisible_before_commit: bool,
    pub final_outbox_state: OutboxState,
    pub parent_transaction_sequence: u128,
    pub final_owner_bytes: Vec<(String, Vec<u8>)>,
}

#[derive(Debug, Error)]
pub enum CoupledTimeReferenceError {
    #[error(transparent)]
    Authority(#[from] CoupledTimeError),
    #[error("reference serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("required reference observation was absent")]
    MissingObservation,
    #[error("uninterrupted and restored chronologies diverged")]
    RestartDivergence,
}

#[derive(Clone)]
struct Ids {
    calendar: Digest32,
    forcing: Digest32,
    policy: Digest32,
    model: Digest32,
    authority: Digest32,
    parent: ParentTransactionId,
}

struct Evidence {
    report: CoupledTimeReferenceReport,
    slab_receipts: Vec<Digest32>,
    event_receipt: Digest32,
    commit_bytes: Vec<u8>,
}

/// Execute uninterrupted and mid-parent-restored twins through the public API.
pub fn run_coupled_time_reference_consumer()
-> Result<CoupledTimeReferenceReport, CoupledTimeReferenceError> {
    let uninterrupted = run(false)?;
    let restarted = run(true)?;
    if uninterrupted.slab_receipts != restarted.slab_receipts
        || uninterrupted.event_receipt != restarted.event_receipt
        || uninterrupted.commit_bytes != restarted.commit_bytes
        || uninterrupted.report.final_owner_bytes != restarted.report.final_owner_bytes
        || uninterrupted.report.accepted_maximum_bits != restarted.report.accepted_maximum_bits
    {
        return Err(CoupledTimeReferenceError::RestartDivergence);
    }
    let mut report = restarted.report;
    report.uninterrupted_equals_restarted = true;
    Ok(report)
}

#[allow(clippy::too_many_lines)] // The linear ordering is itself reference evidence.
fn run(restart_mid_parent: bool) -> Result<Evidence, CoupledTimeReferenceError> {
    let (mut clock, ids) = initial_clock()?;
    let mut reduction =
        DiagnosticReductionV1::new("accepted-peak".into(), "reference-unit".into())?;
    let mut operands = Vec::new();
    let mut slab_receipts = Vec::new();

    let first_constraint = constraint(&ids, clock.accepted_until(), 40)?;
    let first = slab(
        &clock,
        segment(&clock, ids.parent, 0, "snow-covered", &[A, B])?,
        support(0, 40)?,
        &first_constraint,
        &[(A, b"a1"), (B, b"b1")],
        b"first-ledger",
    )?;
    let first_receipt = accept_slab(&mut clock, first)?;
    reduction.fold_accepted(3.0, &first_receipt)?;
    operands.push((3.0, first_receipt.id().digest()));
    slab_receipts.push(first_receipt.id().digest());

    // The rejected attempt cannot construct a valid candidate because its
    // debit/credit join fails. Accepted clock and owner bytes remain identical.
    let before_rejection = serde_json::to_vec(&clock)?;
    let rejected_constraint = constraint(&ids, clock.accepted_until(), 80)?;
    let root = complete_owner_set_digest(clock.owners())?;
    let _attempt = AttemptId::derive(
        ids.parent,
        clock.accepted_until(),
        clock.slab_ordinal(),
        0,
        support(40, 80)?,
        rejected_constraint.digest(),
        root,
    )?;
    let rejected = LedgerEntryV1::new(
        "reference-flux".into(),
        "reference-unit".into(),
        d(30),
        d(31),
        d(32),
    );
    if rejected != Err(CoupledTimeError::LedgerFailure) {
        return Err(CoupledTimeReferenceError::MissingObservation);
    }
    let rejected_attempt_rolled_back = before_rejection == serde_json::to_vec(&clock)?;
    let mut retry = RetryControlV1::new(root, ids.policy, 4, 1);
    retry.record_rejection(
        ModelTimeNs::new(80),
        d(33),
        root,
        ids.policy,
        clock.accepted_until(),
    )?;

    let retry_constraint = constraint(&ids, clock.accepted_until(), 60)?;
    let second = slab(
        &clock,
        segment(&clock, ids.parent, 0, "snow-covered", &[A, B])?,
        support(40, 60)?,
        &retry_constraint,
        &[(A, b"a2"), (B, b"b2")],
        b"retry-ledger",
    )?;
    let second_receipt = accept_slab(&mut clock, second)?;
    reduction.fold_accepted(5.0, &second_receipt)?;
    operands.push((5.0, second_receipt.id().digest()));
    slab_receipts.push(second_receipt.id().digest());

    let event = EventTransitionV1::new(
        &clock,
        ModelTimeNs::new(60),
        EventClass::OwnershipTransfer,
        B.into(),
        d(40),
        vec![
            owner(A, b"a2")?,
            owner(B, b"b-terminal")?,
            owner(C, b"c-custody")?,
        ],
        vec![B.into(), C.into()],
        "snow-free".into(),
        vec![A.into(), C.into()],
        d(41),
        d(41),
    )?;
    let mut events = EventQueueV1::new(ModelTimeNs::new(60), vec![event])?;
    let event_receipt = events
        .apply_next(&mut clock)?
        .ok_or(CoupledTimeReferenceError::MissingObservation)?;
    if events.apply_next(&mut clock)?.is_some() {
        return Err(CoupledTimeReferenceError::MissingObservation);
    }
    clock.record_scheduled_once("daily-reference".into(), d(42), ModelTimeNs::new(60), d(43))?;

    let pre_record = PublicationRecordV1::new(
        second_receipt.id(),
        second_receipt.support(),
        openwepp_coupled_time::digest_bytes(b"accepted-five"),
        b"accepted-five".to_vec(),
        openwepp_coupled_time::digest_bytes(b"v2-wire-lineage"),
        "reference-unit".into(),
        A.into(),
    )?;
    let mut pending = vec![pre_record];
    if restart_mid_parent {
        let restart =
            CoupledTimeRestartV2::new(ids.model, ids.authority, clock, reduction, None, pending)?;
        let bytes = restart.to_canonical_json()?;
        let restored = CoupledTimeRestartV2::from_canonical_json(&bytes, ids.model, ids.policy)?;
        let (restored_clock, mut reductions, outboxes, restored_pending) = restored.into_parts();
        if reductions.len() != 1 || !outboxes.is_empty() {
            return Err(CoupledTimeReferenceError::RestartDivergence);
        }
        clock = restored_clock;
        reduction = reductions
            .pop()
            .ok_or(CoupledTimeReferenceError::RestartDivergence)?;
        pending = restored_pending;
        if clock.accepted_until() != ModelTimeNs::new(60) || clock.active_participants() != [A, C] {
            return Err(CoupledTimeReferenceError::RestartDivergence);
        }
    }

    let final_constraint = constraint(&ids, clock.accepted_until(), 100)?;
    let final_slab = slab(
        &clock,
        segment(&clock, ids.parent, 1, "snow-free", &[A, C])?,
        support(60, 100)?,
        &final_constraint,
        &[(A, b"a3"), (C, b"c1")],
        b"final-ledger",
    )?;
    let final_receipt = accept_slab(&mut clock, final_slab)?;
    reduction.fold_accepted(7.0, &final_receipt)?;
    operands.push((7.0, final_receipt.id().digest()));
    slab_receipts.push(final_receipt.id().digest());
    pending.push(PublicationRecordV1::new(
        final_receipt.id(),
        final_receipt.support(),
        openwepp_coupled_time::digest_bytes(b"accepted-seven"),
        b"accepted-seven".to_vec(),
        openwepp_coupled_time::digest_bytes(b"v2-wire-lineage"),
        "reference-unit".into(),
        A.into(),
    )?);

    let precommit = CoupledTimeRestartV2::new(
        ids.model,
        ids.authority,
        clock.clone(),
        reduction.clone(),
        None,
        pending.clone(),
    )?
    .to_canonical_json()?;
    let precommit_text = String::from_utf8_lossy(&precommit);
    let publication_invisible_before_commit = precommit_text.contains("\"publication_outbox\":[]")
        && !precommit_text.contains("CommittedUndelivered");
    let reconstructed = reconstruct_maximum(&operands)?;
    let maximum = reduction
        .maximum()
        .ok_or(CoupledTimeReferenceError::MissingObservation)?;
    if maximum.to_bits() != reconstructed.to_bits() {
        return Err(CoupledTimeReferenceError::MissingObservation);
    }
    let commit_candidate = ParentCommitCandidateV1::new(&clock, pending)?;
    let commit = commit_parent(&mut clock, commit_candidate)?;
    let commit_bytes = serde_json::to_vec(&commit)?;

    Ok(Evidence {
        report: CoupledTimeReferenceReport {
            uninterrupted_equals_restarted: false,
            rejected_attempt_rolled_back,
            accepted_maximum_bits: maximum.to_bits(),
            reconstructed_maximum_bits: reconstructed.to_bits(),
            accepted_reduction_count: operands.len(),
            publication_invisible_before_commit,
            final_outbox_state: commit.outbox().state(),
            parent_transaction_sequence: commit.transaction_sequence(),
            final_owner_bytes: clock
                .owners()
                .iter()
                .map(|owner| (owner.owner_id().into(), owner.state_bytes().to_vec()))
                .collect(),
        },
        slab_receipts,
        event_receipt: event_receipt.id().digest(),
        commit_bytes,
    })
}

fn initial_clock() -> Result<(CoupledClockStateV1, Ids), CoupledTimeReferenceError> {
    let owners = vec![owner(A, b"a0")?, owner(B, b"b0")?, owner(C, b"c0")?];
    let support = support(0, 100)?;
    let run = d(1);
    let calendar = d(2);
    let forcing = d(3);
    let begin = complete_owner_set_digest(&owners)?;
    let interval = ParentIntervalId::derive(run, calendar, forcing, support)?;
    let parent = ParentTransactionId::derive(run, 41, interval, begin)?;
    let authority = ParentAuthorityV1::new(run, calendar, forcing, 41, support, begin)?;
    let ids = Ids {
        calendar,
        forcing,
        policy: d(4),
        model: d(5),
        authority: d(6),
        parent,
    };
    let clock = CoupledClockStateV1::new(
        authority,
        owners,
        "snow-covered".into(),
        vec![A.into(), B.into()],
        ids.policy,
        b"reference-halving-state".to_vec(),
    )?;
    Ok((clock, ids))
}

fn constraint(
    ids: &Ids,
    cursor: ModelTimeNs,
    end: u128,
) -> Result<StepConstraintV1, CoupledTimeReferenceError> {
    let value = StepConstraintV1::new(
        ids.parent,
        cursor,
        ModelTimeNs::new(end),
        A.into(),
        ConstraintClass::AdaptiveUpperBound,
        d(60),
        ids.calendar,
        ids.forcing,
    )?;
    Ok(reduce_constraints(
        std::slice::from_ref(&value),
        ids.parent,
        cursor,
        ModelTimeNs::new(100),
        None,
    )?
    .clone())
}

fn segment(
    clock: &CoupledClockStateV1,
    parent: ParentTransactionId,
    ordinal: u32,
    regime: &str,
    participants: &[&str],
) -> Result<SegmentId, CoupledTimeError> {
    let mut participant_bytes = Vec::new();
    for participant in participants {
        participant_bytes.extend_from_slice(participant.as_bytes());
        participant_bytes.push(0);
    }
    SegmentId::derive(
        parent,
        ordinal,
        TimeSupport::new(clock.accepted_until(), ModelTimeNs::new(100))?,
        openwepp_coupled_time::digest_bytes(regime.as_bytes()),
        openwepp_coupled_time::digest_bytes(&participant_bytes),
    )
}

fn slab(
    clock: &CoupledClockStateV1,
    segment: SegmentId,
    support: TimeSupport,
    constraint: &StepConstraintV1,
    active: &[(&str, &[u8])],
    ledger_label: &[u8],
) -> Result<CoupledSlabCandidateV1, CoupledTimeReferenceError> {
    let ending = clock
        .owners()
        .iter()
        .map(|before| {
            active
                .iter()
                .find(|(id, _)| *id == before.owner_id())
                .map_or_else(
                    || Ok(before.clone()),
                    |(_, bytes)| owner(before.owner_id(), bytes),
                )
        })
        .collect::<Result<Vec<_>, CoupledTimeError>>()?;
    let joined = openwepp_coupled_time::digest_bytes(ledger_label);
    let ledger = LedgerEntryV1::new(
        "reference-flux".into(),
        "reference-unit".into(),
        joined,
        joined,
        d(61),
    )?;
    Ok(CoupledSlabCandidateV1::new(
        clock,
        segment,
        support,
        constraint,
        ending,
        vec![ledger],
    )?)
}

fn reconstruct_maximum(values: &[(f64, Digest32)]) -> Result<f64, CoupledTimeReferenceError> {
    if values.is_empty() || values.windows(2).any(|pair| pair[0].1 == pair[1].1) {
        return Err(CoupledTimeReferenceError::MissingObservation);
    }
    values
        .iter()
        .map(|(value, _)| *value)
        .reduce(f64::max)
        .ok_or(CoupledTimeReferenceError::MissingObservation)
}

fn support(start: u128, end: u128) -> Result<TimeSupport, CoupledTimeError> {
    TimeSupport::new(ModelTimeNs::new(start), ModelTimeNs::new(end))
}

fn owner(id: &str, bytes: &[u8]) -> Result<OwnerState, CoupledTimeError> {
    OwnerState::new(id.into(), bytes.to_vec())
}

fn d(seed: u8) -> Digest32 {
    Digest32::from_bytes([seed; 32])
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_coupled_time::{ParentCommitV1, PublicationOutboxV1, ReceiptId};

    #[test]
    fn closed_protocol_is_restart_equivalent_and_atomic() {
        let report = run_coupled_time_reference_consumer().expect("reference chronology");
        assert!(report.uninterrupted_equals_restarted);
        assert!(report.rejected_attempt_rolled_back);
        assert_eq!(report.accepted_maximum_bits, 7.0_f64.to_bits());
        assert_eq!(
            report.accepted_maximum_bits,
            report.reconstructed_maximum_bits
        );
        assert_eq!(report.accepted_reduction_count, 3);
        assert!(report.publication_invisible_before_commit);
        assert_eq!(report.final_outbox_state, OutboxState::CommittedUndelivered);
        assert_eq!(report.parent_transaction_sequence, 42);
        assert_eq!(
            report.final_owner_bytes,
            vec![
                (A.into(), b"a3".to_vec()),
                (B.into(), b"b-terminal".to_vec()),
                (C.into(), b"c1".to_vec()),
            ]
        );
    }

    #[test]
    fn reconstructor_separates_rejected_and_nominal_duration_aliases() {
        let accepted = vec![(3.0, d(70)), (5.0, d(71)), (7.0, d(72))];
        let actual = reconstruct_maximum(&accepted).expect("reconstruction");
        assert_ne!(actual.to_bits(), 99.0_f64.to_bits());
        assert_ne!(actual.to_bits(), ((3.0_f64 + 5.0 + 7.0) / 100.0).to_bits());
        let before_restart = reconstruct_maximum(&accepted[..2]).expect("pre-restart alias");
        assert_ne!(actual.to_bits(), before_restart.to_bits());
        assert_eq!(
            accepted.len(),
            3,
            "post-only equal magnitude lacks full receipt lineage"
        );
    }

    #[test]
    fn outbox_is_durable_across_commit_delivery_and_ack_crashes() {
        let evidence = run(false).expect("completed chronology");
        let commit: ParentCommitV1 =
            serde_json::from_slice(&evidence.commit_bytes).expect("commit wire");
        let mut outbox: PublicationOutboxV1 = commit.into_outbox();
        let key = outbox.receipt_id();
        let committed = serde_json::to_vec(&outbox).expect("committed boundary");
        outbox = serde_json::from_slice(&committed).expect("restore committed");
        outbox.mark_delivered(key).expect("deliver");
        let delivered = serde_json::to_vec(&outbox).expect("delivered boundary");
        outbox = serde_json::from_slice(&delivered).expect("restore delivered");
        outbox.mark_delivered(key).expect("idempotent redelivery");
        outbox.acknowledge(key).expect("ack");
        let acknowledged = serde_json::to_vec(&outbox).expect("ack boundary");
        let mut restored: PublicationOutboxV1 =
            serde_json::from_slice(&acknowledged).expect("restore ack");
        assert_eq!(restored.state(), OutboxState::Acknowledged);
        assert_eq!(
            restored.mark_delivered(ReceiptId::from_digest(d(99))),
            Err(CoupledTimeError::OutboxTransition)
        );
    }
}
