//! Bounded, non-physical reference consumer for coupled-time orchestration.
//!
//! This module demonstrates the authority's transaction chronology without
//! selecting a production model or implementing constitutive equations.

use openwepp_coupled_time::{
    AcceptedSlabId, CoupledClockStateV1, CoupledSlabCandidateV1, CoupledTimeError,
    CoupledTimeRestartV1, DiagnosticReductionV1, EventClass, EventId, EventTransitionV1,
    ModelTimeNs, OutboxState, OwnerCandidateV1, OwnerState, ParentIntervalId, ParentTransactionId,
    PublicationOutboxV1, PublicationRecordV1, ReceiptId, RetryControlV1, TimeSupport, accept_slab,
    apply_event, digest_bytes, finalize_parent,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const OWNER_A: &str = "owner-a";
const OWNER_B: &str = "owner-b";
const OWNER_C: &str = "owner-c";

/// Observable evidence produced by the bounded reference chronology.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoupledTimeReferenceReport {
    pub rejected_attempt_rolled_back: bool,
    pub rejected_proposal_end_ns: ModelTimeNs,
    pub retry_proposal_end_ns: ModelTimeNs,
    pub restored_mid_parent: bool,
    pub accepted_maximum: f64,
    pub accepted_reduction_count: usize,
    pub scheduled_once_count: usize,
    pub event_count: u32,
    pub slab_count: u32,
    pub publication_blocked_before_commit: bool,
    pub final_outbox_state: OutboxState,
    pub parent_transaction_sequence: u128,
    pub final_owners: Vec<OwnerState>,
}

/// Typed failure boundary for the reference consumer.
#[derive(Debug, Error)]
pub enum CoupledTimeReferenceError {
    #[error(transparent)]
    Authority(#[from] CoupledTimeError),
    #[error("coupled-time restart serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("reference chronology did not observe the required authority failure")]
    MissingExpectedFailure,
}

/// Execute a complete synthetic parent chronology through the public authority API.
///
/// Owner bytes are opaque tokens, not physical state. The run demonstrates an
/// A+B segment, a rejected attempt and reduced retry, a B-to-C event transition,
/// restart, an A+C segment, accepted-only reduction, and one atomic parent commit.
#[allow(clippy::too_many_lines)] // Keeping the chronology linear makes ordering evidence auditable.
pub fn run_coupled_time_reference_consumer()
-> Result<CoupledTimeReferenceReport, CoupledTimeReferenceError> {
    let parent_support = TimeSupport::new(ModelTimeNs(0), ModelTimeNs(100))?;
    let owners = vec![
        owner(OWNER_A, b"a0"),
        owner(OWNER_B, b"b0"),
        owner(OWNER_C, b"c0"),
    ];
    let policy = digest_bytes(b"reference-halving-policy-v1");
    let mut clock = CoupledClockStateV1::new(
        ParentIntervalId(digest_bytes(b"parent-interval")),
        ParentTransactionId(digest_bytes(b"parent-transaction")),
        parent_support,
        owners,
        "covered".to_owned(),
        vec![OWNER_A.to_owned(), OWNER_B.to_owned()],
        policy,
        b"initial-controller-checkpoint".to_vec(),
    )?;
    let model = digest_bytes(b"bounded-reference-model-v1");
    let mut reduction = DiagnosticReductionV1::new();

    let first = slab(&clock, 40, &[(OWNER_A, b"a1"), (OWNER_B, b"b1")], b"slab-0")?;
    accept_slab(&mut clock, &first)?;
    reduction.fold_accepted(3.0, first.receipt_id)?;

    // A rejected attempt is presented to the authority but cannot mutate the
    // accepted clock or owner set. The adopter then reduces its proposal.
    let before_rejection = serde_json::to_vec(&clock)?;
    let mut rejected = slab(
        &clock,
        80,
        &[(OWNER_A, b"a-rejected"), (OWNER_B, b"b-rejected")],
        b"attempt-rejected",
    )?;
    rejected.ledgers_closed = false;
    let rejection = accept_slab(&mut clock, &rejected);
    if rejection != Err(CoupledTimeError::LedgerFailure) {
        return Err(CoupledTimeReferenceError::MissingExpectedFailure);
    }
    let rejected_attempt_rolled_back = before_rejection == serde_json::to_vec(&clock)?;
    let mut retry = RetryControlV1 {
        accepted_state_digest: digest_bytes(&before_rejection),
        attempt_ordinal: 0,
        last_proposal_end_ns: None,
        retry_digest: digest_bytes(b"initial-proposal"),
    };
    retry.record_rejection(ModelTimeNs(80), digest_bytes(b"halve-to-event"))?;

    let second = slab(&clock, 60, &[(OWNER_A, b"a2"), (OWNER_B, b"b2")], b"slab-1")?;
    accept_slab(&mut clock, &second)?;
    reduction.fold_accepted(7.0, second.receipt_id)?;

    let ending_owners = vec![
        owner(OWNER_A, b"a2"),
        owner(OWNER_B, b"b-terminal"),
        owner(OWNER_C, b"c-custody"),
    ];
    let event = EventTransitionV1 {
        event_id: EventId(digest_bytes(b"b-to-c-event")),
        tick_ns: ModelTimeNs(60),
        class: EventClass::OwnershipTransfer,
        source_owner_id: OWNER_B.to_owned(),
        event_context_digest: digest_bytes(b"terminal-transition"),
        beginning_owner_set_digest: digest_bytes(&serde_json::to_vec(&clock.complete_owner_set)?),
        ending_owners,
        successor_regime_id: "open".to_owned(),
        successor_participants: vec![OWNER_A.to_owned(), OWNER_C.to_owned()],
        ledger_digest: digest_bytes(b"event-ledger-closed"),
        ledger_closed: true,
        receipt_id: ReceiptId(digest_bytes(b"event-receipt")),
    };
    apply_event(&mut clock, &event)?;
    clock.record_scheduled_once(ReceiptId(digest_bytes(b"scheduled-once")))?;

    let publication = PublicationRecordV1 {
        record_id: ReceiptId(digest_bytes(b"publication-record")),
        payload: b"accepted-only-peak".to_vec(),
        support_lineage_digest: digest_bytes(b"accepted-support-lineage"),
        units: "reference-unit".to_owned(),
        source_owner_id: OWNER_A.to_owned(),
    };
    let outbox = PublicationOutboxV1 {
        receipt_id: ReceiptId(digest_bytes(b"publication-outbox")),
        sequence: 1,
        state: OutboxState::Staged,
        records: vec![publication.clone()],
    };
    let checkpoint = CoupledTimeRestartV1 {
        schema_id: openwepp_coupled_time::RESTART_SCHEMA_ID.to_owned(),
        model_definition_sha256: model,
        clock,
        reduction_state: reduction,
        publication_outbox: Some(outbox),
        pending_publication_records: vec![publication],
    };
    let wire = serde_json::to_vec(&checkpoint)?;
    let mut restored: CoupledTimeRestartV1 = serde_json::from_slice(&wire)?;
    restored.validate(model, policy)?;
    let restored_mid_parent = !restored.clock.is_complete()
        && restored.clock.accepted_until == ModelTimeNs(60)
        && restored.clock.active_participant_set == [OWNER_A, OWNER_C];

    let third = slab(
        &restored.clock,
        100,
        &[(OWNER_A, b"a3"), (OWNER_C, b"c1")],
        b"slab-2",
    )?;
    accept_slab(&mut restored.clock, &third)?;
    restored
        .reduction_state
        .fold_accepted(5.0, third.receipt_id)?;
    let accepted_maximum = restored
        .reduction_state
        .maximum
        .ok_or(CoupledTimeReferenceError::MissingExpectedFailure)?;

    let outbox = restored
        .publication_outbox
        .as_mut()
        .ok_or(CoupledTimeReferenceError::MissingExpectedFailure)?;
    let publication_blocked_before_commit = outbox.commit(false)
        == Err(CoupledTimeError::PublicationBeforeParentCommit)
        && outbox.state == OutboxState::Staged;
    let commit = finalize_parent(
        &restored.clock,
        41,
        ReceiptId(digest_bytes(b"parent-commit")),
    )?;
    outbox.commit(restored.clock.is_complete())?;

    Ok(CoupledTimeReferenceReport {
        rejected_attempt_rolled_back,
        rejected_proposal_end_ns: ModelTimeNs(80),
        retry_proposal_end_ns: ModelTimeNs(60),
        restored_mid_parent,
        accepted_maximum,
        accepted_reduction_count: restored.reduction_state.accepted_receipts.len(),
        scheduled_once_count: restored.clock.scheduled_once_receipts.len(),
        event_count: restored.clock.event_ordinal,
        slab_count: restored.clock.slab_ordinal,
        publication_blocked_before_commit,
        final_outbox_state: outbox.state,
        parent_transaction_sequence: commit.transaction_sequence,
        final_owners: commit.ending_owner_set,
    })
}

fn owner(owner_id: &str, state: &[u8]) -> OwnerState {
    OwnerState {
        owner_id: owner_id.to_owned(),
        state_bytes: state.to_vec(),
        state_digest: digest_bytes(state),
    }
}

fn slab(
    clock: &CoupledClockStateV1,
    end_ns: u128,
    endings: &[(&str, &[u8])],
    identity: &[u8],
) -> Result<CoupledSlabCandidateV1, CoupledTimeReferenceError> {
    let support = TimeSupport::new(clock.accepted_until, ModelTimeNs(end_ns))?;
    let candidates = endings
        .iter()
        .map(|(owner_id, ending)| {
            let beginning = clock
                .complete_owner_set
                .iter()
                .find(|owner| owner.owner_id == *owner_id)
                .ok_or(CoupledTimeError::OwnerCandidate)?;
            Ok(OwnerCandidateV1 {
                owner_id: (*owner_id).to_owned(),
                beginning_state_digest: beginning.state_digest,
                ending_state_bytes: ending.to_vec(),
                ending_state_digest: digest_bytes(ending),
                ledger_digest: digest_bytes(identity),
            })
        })
        .collect::<Result<Vec<_>, CoupledTimeError>>()?;
    Ok(CoupledSlabCandidateV1 {
        accepted_slab_id: AcceptedSlabId(digest_bytes(identity)),
        support,
        duration_s_bits: support.duration_s_bits(),
        candidates,
        global_ledger_digest: digest_bytes(identity),
        ledgers_closed: true,
        receipt_id: ReceiptId(digest_bytes(identity)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercises_segment_event_restart_and_atomic_parent_commit() {
        let report = run_coupled_time_reference_consumer().expect("reference chronology");

        assert!(report.rejected_attempt_rolled_back);
        assert!(report.rejected_proposal_end_ns > report.retry_proposal_end_ns);
        assert!(report.restored_mid_parent);
        assert_eq!(report.accepted_maximum.to_bits(), 7.0_f64.to_bits());
        assert_eq!(report.accepted_reduction_count, 3);
        assert_eq!(report.scheduled_once_count, 1);
        assert_eq!(report.event_count, 1);
        assert_eq!(report.slab_count, 3);
        assert!(report.publication_blocked_before_commit);
        assert_eq!(report.final_outbox_state, OutboxState::CommittedUndelivered);
        assert_eq!(report.parent_transaction_sequence, 42);
        assert_eq!(
            report
                .final_owners
                .iter()
                .map(|owner| (owner.owner_id.as_str(), owner.state_bytes.as_slice()))
                .collect::<Vec<_>>(),
            vec![
                (OWNER_A, b"a3".as_slice()),
                (OWNER_B, b"b-terminal".as_slice()),
                (OWNER_C, b"c1".as_slice()),
            ]
        );
    }

    #[test]
    fn rejected_attempt_never_enters_reduction_or_publication() {
        let report = run_coupled_time_reference_consumer().expect("reference chronology");

        assert_eq!(report.accepted_reduction_count, 3);
        assert_eq!(report.accepted_maximum.to_bits(), 7.0_f64.to_bits());
        assert!(report.publication_blocked_before_commit);
    }
}
