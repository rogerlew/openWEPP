use serde::{Deserialize, Serialize};

use crate::{
    CoupledTimeError, Digest32, EventId, ModelTimeNs, ParentTransactionId, PendingEventJoinV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConstraintClass {
    HardBoundary,
    EventBoundary,
    OutputBoundary,
    RestartBoundary,
    AdaptiveUpperBound,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StepConstraintV1 {
    pub(crate) parent_transaction_id: ParentTransactionId,
    pub(crate) accepted_cursor_ns: ModelTimeNs,
    pub(crate) proposed_end_ns: ModelTimeNs,
    pub(crate) source_owner_id: String,
    pub(crate) class: ConstraintClass,
    pub(crate) constraint_digest: Digest32,
    pub(crate) compatibility_group_digest: Digest32,
    pub(crate) calendar_receipt: Digest32,
    pub(crate) forcing_receipt: Digest32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConstraintReductionReceiptV1 {
    parent_transaction_id: ParentTransactionId,
    accepted_cursor_ns: ModelTimeNs,
    selected_end_ns: ModelTimeNs,
    coincident: Vec<StepConstraintV1>,
    pending_event: Option<EventId>,
    digest: Digest32,
}

impl ConstraintReductionReceiptV1 {
    #[must_use]
    pub const fn digest(&self) -> Digest32 {
        self.digest
    }
    #[must_use]
    pub const fn proposed_end(&self) -> ModelTimeNs {
        self.selected_end_ns
    }
    pub(crate) fn validate_identity(&self) -> Result<(), CoupledTimeError> {
        let rebuilt = reduce_constraints(
            &self.coincident,
            self.parent_transaction_id,
            self.accepted_cursor_ns,
            self.selected_end_ns,
            self.pending_event
                .map(|event_id| PendingEventJoinV1 { event_id })
                .as_ref(),
        )?;
        if rebuilt != *self {
            return Err(CoupledTimeError::InvalidConstraint);
        }
        Ok(())
    }
    pub(crate) fn matches_clock(&self, parent: ParentTransactionId, cursor: ModelTimeNs) -> bool {
        self.parent_transaction_id == parent && self.accepted_cursor_ns == cursor
    }
}
impl StepConstraintV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        parent: ParentTransactionId,
        cursor: ModelTimeNs,
        end: ModelTimeNs,
        source: String,
        class: ConstraintClass,
        compatibility: Digest32,
        calendar: Digest32,
        forcing: Digest32,
    ) -> Result<Self, CoupledTimeError> {
        if source.is_empty() {
            return Err(CoupledTimeError::InvalidConstraint);
        }
        let cursor_b = cursor.get().to_be_bytes();
        let end_b = end.get().to_be_bytes();
        let class_b = format!("{class:?}");
        let constraint_digest = crate::framed_sha256(
            "constraint",
            &[
                crate::FramedField {
                    tag: "parent_transaction_id",
                    value: parent.digest().as_bytes(),
                },
                crate::FramedField {
                    tag: "cursor_ns",
                    value: &cursor_b,
                },
                crate::FramedField {
                    tag: "end_ns",
                    value: &end_b,
                },
                crate::FramedField {
                    tag: "class",
                    value: class_b.as_bytes(),
                },
                crate::FramedField {
                    tag: "source_owner_id",
                    value: source.as_bytes(),
                },
                crate::FramedField {
                    tag: "compatibility_group",
                    value: compatibility.as_bytes(),
                },
            ],
        )?;
        Ok(Self {
            parent_transaction_id: parent,
            accepted_cursor_ns: cursor,
            proposed_end_ns: end,
            source_owner_id: source,
            class,
            constraint_digest,
            compatibility_group_digest: compatibility,
            calendar_receipt: calendar,
            forcing_receipt: forcing,
        })
    }
    #[must_use]
    pub const fn digest(&self) -> Digest32 {
        self.constraint_digest
    }
    #[must_use]
    pub const fn proposed_end(&self) -> ModelTimeNs {
        self.proposed_end_ns
    }

    pub(crate) fn validate_identity(&self) -> Result<(), CoupledTimeError> {
        let reconstructed = Self::new(
            self.parent_transaction_id,
            self.accepted_cursor_ns,
            self.proposed_end_ns,
            self.source_owner_id.clone(),
            self.class,
            self.compatibility_group_digest,
            self.calendar_receipt,
            self.forcing_receipt,
        )?;
        if reconstructed.constraint_digest != self.constraint_digest {
            return Err(CoupledTimeError::InvalidConstraint);
        }
        Ok(())
    }
}

pub fn reduce_constraints(
    constraints: &[StepConstraintV1],
    parent: ParentTransactionId,
    cursor: ModelTimeNs,
    parent_end: ModelTimeNs,
    pending_event: Option<&PendingEventJoinV1>,
) -> Result<ConstraintReductionReceiptV1, CoupledTimeError> {
    if constraints.is_empty() {
        return Err(CoupledTimeError::InvalidConstraint);
    }
    for value in constraints {
        value.validate_identity()?;
        if value.parent_transaction_id != parent || value.accepted_cursor_ns != cursor {
            return Err(CoupledTimeError::ParentMismatch);
        }
        if value.proposed_end_ns < cursor || value.proposed_end_ns > parent_end {
            return Err(CoupledTimeError::InvalidConstraint);
        }
        if value.proposed_end_ns == cursor
            && !(pending_event.is_some() && value.class == ConstraintClass::EventBoundary)
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
    if let Some(first) = coincident.first() {
        if coincident.iter().any(|v| {
            v.calendar_receipt != first.calendar_receipt
                || v.forcing_receipt != first.forcing_receipt
                || (v.class != ConstraintClass::AdaptiveUpperBound
                    && first.class != ConstraintClass::AdaptiveUpperBound
                    && v.compatibility_group_digest != first.compatibility_group_digest)
        }) {
            return Err(CoupledTimeError::ConstraintConflict);
        }
    }
    let mut coincident: Vec<_> = coincident.into_iter().cloned().collect();
    coincident.sort_by_key(|v| (v.class, v.source_owner_id.clone(), v.constraint_digest));
    if earliest == cursor
        && (!coincident
            .iter()
            .all(|v| v.class == ConstraintClass::EventBoundary)
            || pending_event.is_none())
    {
        return Err(CoupledTimeError::ZeroStepWithoutEvent);
    }
    let mut bytes = Vec::new();
    for value in &coincident {
        bytes.extend_from_slice(value.digest().as_bytes());
    }
    if let Some(event) = pending_event {
        bytes.extend_from_slice(event.event_id.digest().as_bytes());
    }
    let digest = crate::digest_bytes(&bytes);
    Ok(ConstraintReductionReceiptV1 {
        parent_transaction_id: parent,
        accepted_cursor_ns: cursor,
        selected_end_ns: earliest,
        coincident,
        pending_event: pending_event.map(|event| event.event_id),
        digest,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetryControlV1 {
    accepted_state_digest: Digest32,
    controller_policy_digest: Digest32,
    attempt_ordinal: u32,
    last_proposal_end_ns: Option<ModelTimeNs>,
    retry_digest: Digest32,
    max_attempts: u32,
    minimum_step_ns: u128,
}

impl RetryControlV1 {
    #[must_use]
    pub const fn new(
        root: Digest32,
        policy: Digest32,
        max_attempts: u32,
        minimum_step_ns: u128,
    ) -> Self {
        Self {
            accepted_state_digest: root,
            controller_policy_digest: policy,
            attempt_ordinal: 0,
            last_proposal_end_ns: None,
            retry_digest: Digest32::zero(),
            max_attempts,
            minimum_step_ns,
        }
    }
    pub fn record_rejection(
        &mut self,
        proposal: ModelTimeNs,
        new_digest: Digest32,
        accepted_root: Digest32,
        policy: Digest32,
        cursor: ModelTimeNs,
    ) -> Result<(), CoupledTimeError> {
        if accepted_root != self.accepted_state_digest || policy != self.controller_policy_digest {
            return Err(CoupledTimeError::ControllerPolicyMismatch);
        }
        if proposal.get().saturating_sub(cursor.get()) < self.minimum_step_ns
            || self.attempt_ordinal >= self.max_attempts
        {
            return Err(CoupledTimeError::RetryExhausted);
        }
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
