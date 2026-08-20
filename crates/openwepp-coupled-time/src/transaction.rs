use serde::{Deserialize, Serialize};

use crate::{
    AcceptedSlabId, CoupledClockStateV1, CoupledTimeError, Digest32, OwnerState, ReceiptId,
    TimeSupport, digest_bytes,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerCandidateV1 {
    pub owner_id: String,
    pub beginning_state_digest: Digest32,
    pub ending_state_bytes: Vec<u8>,
    pub ending_state_digest: Digest32,
    pub ledger_digest: Digest32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoupledSlabCandidateV1 {
    pub accepted_slab_id: AcceptedSlabId,
    pub support: TimeSupport,
    pub duration_s_bits: u64,
    pub candidates: Vec<OwnerCandidateV1>,
    pub global_ledger_digest: Digest32,
    pub ledgers_closed: bool,
    pub receipt_id: ReceiptId,
}

pub fn accept_slab(
    clock: &mut CoupledClockStateV1,
    slab: &CoupledSlabCandidateV1,
) -> Result<(), CoupledTimeError> {
    if slab.support.start_ns != clock.accepted_until
        || slab.support.end_ns > clock.parent_support.end_ns
        || slab.duration_s_bits != slab.support.duration_s_bits()
    {
        return Err(CoupledTimeError::ParentMismatch);
    }
    if !slab.ledgers_closed {
        return Err(CoupledTimeError::LedgerFailure);
    }
    if slab.candidates.len() != clock.active_participant_set.len() {
        return Err(CoupledTimeError::OwnerCandidate);
    }
    let mut next = clock.complete_owner_set.clone();
    for (participant, candidate) in clock.active_participant_set.iter().zip(&slab.candidates) {
        if participant != &candidate.owner_id
            || digest_bytes(&candidate.ending_state_bytes) != candidate.ending_state_digest
        {
            return Err(CoupledTimeError::OwnerCandidate);
        }
        let owner = next
            .binary_search_by(|o| o.owner_id.cmp(participant))
            .map_err(|_| CoupledTimeError::OwnerCandidate)?;
        if next[owner].state_digest != candidate.beginning_state_digest {
            return Err(CoupledTimeError::OwnerCandidate);
        }
        next[owner] = OwnerState {
            owner_id: participant.clone(),
            state_bytes: candidate.ending_state_bytes.clone(),
            state_digest: candidate.ending_state_digest,
        };
    }
    let duration = slab.support.duration_ns();
    let next_ordinal = clock
        .slab_ordinal
        .checked_add(1)
        .ok_or(CoupledTimeError::ArithmeticOverflow)?;
    clock.complete_owner_set = next;
    clock.accepted_until = slab.support.end_ns;
    clock.last_accepted_step_ns = Some(duration);
    clock.slab_ordinal = next_ordinal;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticReductionV1 {
    pub maximum: Option<f64>,
    pub accepted_receipts: Vec<ReceiptId>,
}

impl DiagnosticReductionV1 {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            maximum: None,
            accepted_receipts: Vec::new(),
        }
    }
    pub fn fold_accepted(
        &mut self,
        value: f64,
        receipt: ReceiptId,
    ) -> Result<(), CoupledTimeError> {
        if !value.is_finite() {
            return Err(CoupledTimeError::LedgerFailure);
        }
        self.maximum = Some(self.maximum.map_or(value, |old| old.max(value)));
        self.accepted_receipts.push(receipt);
        Ok(())
    }
}

impl Default for DiagnosticReductionV1 {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalOperatorClass {
    AlgebraicRate,
    SupportIntegral,
    SequentialStateTransition,
    ThresholdEvent,
    ScheduledOnce,
    DiagnosticReduction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentCommitV1 {
    pub transaction_sequence: u128,
    pub receipt_id: ReceiptId,
    pub ending_owner_set: Vec<OwnerState>,
}

/// Finalize exactly once after the accepted cursor reaches the parent end.
pub fn finalize_parent(
    clock: &CoupledClockStateV1,
    transaction_sequence: u128,
    receipt_id: ReceiptId,
) -> Result<ParentCommitV1, CoupledTimeError> {
    if !clock.is_complete() {
        return Err(CoupledTimeError::ParentNotFinalizable);
    }
    let successor = transaction_sequence
        .checked_add(1)
        .ok_or(CoupledTimeError::ArithmeticOverflow)?;
    Ok(ParentCommitV1 {
        transaction_sequence: successor,
        receipt_id,
        ending_owner_set: clock.complete_owner_set.clone(),
    })
}
