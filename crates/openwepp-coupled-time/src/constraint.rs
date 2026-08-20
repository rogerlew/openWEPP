use serde::{Deserialize, Serialize};

use crate::{CoupledTimeError, Digest32, ModelTimeNs, ParentTransactionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConstraintClass {
    HardBoundary,
    EventBoundary,
    OutputBoundary,
    RestartBoundary,
    AdaptiveUpperBound,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepConstraintV1 {
    pub parent_transaction_id: ParentTransactionId,
    pub accepted_cursor_ns: ModelTimeNs,
    pub proposed_end_ns: ModelTimeNs,
    pub source_owner_id: String,
    pub class: ConstraintClass,
    pub constraint_digest: Digest32,
    pub compatibility_group_digest: Digest32,
    pub calendar_receipt: Digest32,
    pub forcing_receipt: Digest32,
}

pub fn reduce_constraints(
    constraints: &[StepConstraintV1],
    parent: ParentTransactionId,
    cursor: ModelTimeNs,
    parent_end: ModelTimeNs,
    pending_event: bool,
) -> Result<&StepConstraintV1, CoupledTimeError> {
    if constraints.is_empty() {
        return Err(CoupledTimeError::InvalidConstraint);
    }
    for value in constraints {
        if value.parent_transaction_id != parent || value.accepted_cursor_ns != cursor {
            return Err(CoupledTimeError::ParentMismatch);
        }
        if value.proposed_end_ns < cursor || value.proposed_end_ns > parent_end {
            return Err(CoupledTimeError::InvalidConstraint);
        }
        if value.proposed_end_ns == cursor
            && !(pending_event && value.class == ConstraintClass::EventBoundary)
        {
            return Err(CoupledTimeError::ZeroStepWithoutEvent);
        }
    }
    let earliest = constraints
        .iter()
        .map(|v| v.proposed_end_ns)
        .min()
        .ok_or(CoupledTimeError::InvalidConstraint)?;
    let coincident: Vec<_> = constraints
        .iter()
        .filter(|v| v.proposed_end_ns == earliest)
        .collect();
    let facts: Vec<_> = coincident
        .iter()
        .filter(|v| v.class != ConstraintClass::AdaptiveUpperBound)
        .collect();
    if let Some(first) = facts.first() {
        if facts.iter().any(|v| {
            v.calendar_receipt != first.calendar_receipt
                || v.forcing_receipt != first.forcing_receipt
                || v.compatibility_group_digest != first.compatibility_group_digest
        }) {
            return Err(CoupledTimeError::ConstraintConflict);
        }
    }
    coincident
        .into_iter()
        .min_by_key(|v| (v.class, &v.source_owner_id, v.constraint_digest))
        .ok_or(CoupledTimeError::InvalidConstraint)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetryControlV1 {
    pub accepted_state_digest: Digest32,
    pub attempt_ordinal: u32,
    pub last_proposal_end_ns: Option<ModelTimeNs>,
    pub retry_digest: Digest32,
}

impl RetryControlV1 {
    pub fn record_rejection(
        &mut self,
        proposal: ModelTimeNs,
        new_digest: Digest32,
    ) -> Result<(), CoupledTimeError> {
        if self.last_proposal_end_ns == Some(proposal) && self.retry_digest == new_digest {
            return Err(CoupledTimeError::RetryExhausted);
        }
        self.attempt_ordinal = self
            .attempt_ordinal
            .checked_add(1)
            .ok_or(CoupledTimeError::ArithmeticOverflow)?;
        self.last_proposal_end_ns = Some(proposal);
        self.retry_digest = new_digest;
        Ok(())
    }
}
