use serde::{Deserialize, Serialize};

use crate::{
    CoupledTimeError, Digest32, ModelTimeNs, ParentIntervalId, ParentTransactionId, ReceiptId,
    TimeSupport,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerState {
    pub(crate) owner_id: String,
    pub(crate) state_bytes: Vec<u8>,
    pub(crate) state_digest: Digest32,
}
impl OwnerState {
    pub fn new(owner_id: String, state_bytes: Vec<u8>) -> Result<Self, CoupledTimeError> {
        if owner_id.is_empty() {
            return Err(CoupledTimeError::NonCanonicalIdentity);
        }
        let state_digest = crate::digest_bytes(&state_bytes);
        Ok(Self {
            owner_id,
            state_bytes,
            state_digest,
        })
    }
    #[must_use]
    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }
    #[must_use]
    pub fn state_bytes(&self) -> &[u8] {
        &self.state_bytes
    }
    #[must_use]
    pub const fn state_digest(&self) -> Digest32 {
        self.state_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoupledClockStateV1 {
    pub(crate) run_identity: Digest32,
    pub(crate) calendar_receipt: Digest32,
    pub(crate) forcing_receipt: Digest32,
    #[serde(with = "crate::restart::u128_string")]
    pub(crate) parent_transaction_sequence: u128,
    pub(crate) committed: bool,
    pub(crate) begin_owner_set_digest: Digest32,
    pub(crate) begin_clock_digest: Digest32,
    pub(crate) accepted_clock_digest: Digest32,
    pub(crate) parent_interval_id: ParentIntervalId,
    pub(crate) parent_transaction_id: ParentTransactionId,
    pub(crate) parent_support: TimeSupport,
    pub(crate) accepted_until: ModelTimeNs,
    pub(crate) segment_ordinal: u32,
    pub(crate) slab_ordinal: u32,
    pub(crate) event_ordinal: u32,
    pub(crate) last_accepted_step_ns: Option<u128>,
    pub(crate) complete_owner_set: Vec<OwnerState>,
    pub(crate) active_regime_id: String,
    pub(crate) active_segment_start: ModelTimeNs,
    pub(crate) active_participant_set: Vec<String>,
    pub(crate) accepted_slab_receipts: Vec<crate::AcceptedSlabReceiptV1>,
    pub(crate) accepted_event_receipts: Vec<crate::AcceptedEventReceiptV1>,
    pub(crate) scheduled_once_receipts: Vec<ScheduledOnceReceiptV1>,
    pub(crate) controller_policy_sha256: Digest32,
    pub(crate) controller_checkpoint: Vec<u8>,
}

impl CoupledClockStateV1 {
    pub fn new(
        authority: ParentAuthorityV1,
        owners: Vec<OwnerState>,
        regime: String,
        participants: Vec<String>,
        policy: Digest32,
        checkpoint: Vec<u8>,
    ) -> Result<Self, CoupledTimeError> {
        validate_owner_and_participant_sets(&owners, &participants)?;
        let owner_digest = crate::transaction::owner_set_digest(&owners)?;
        let interval = ParentIntervalId::derive(
            authority.run_identity,
            authority.calendar_receipt,
            authority.forcing_receipt,
            authority.parent_support,
        )?;
        let transaction = ParentTransactionId::derive(
            authority.run_identity,
            authority.transaction_sequence,
            interval,
            owner_digest,
        )?;
        if interval != authority.parent_interval_id
            || transaction != authority.parent_transaction_id
        {
            return Err(CoupledTimeError::ParentMismatch);
        }
        let begin_clock_digest = crate::digest_bytes(
            &[
                transaction.digest().as_bytes(),
                owner_digest.as_bytes(),
                authority
                    .parent_support
                    .start_ns()
                    .get()
                    .to_be_bytes()
                    .as_slice(),
            ]
            .concat(),
        );
        Ok(Self {
            run_identity: authority.run_identity,
            calendar_receipt: authority.calendar_receipt,
            forcing_receipt: authority.forcing_receipt,
            parent_transaction_sequence: authority.transaction_sequence,
            committed: false,
            begin_owner_set_digest: owner_digest,
            begin_clock_digest,
            accepted_clock_digest: begin_clock_digest,
            parent_interval_id: authority.parent_interval_id,
            parent_transaction_id: authority.parent_transaction_id,
            parent_support: authority.parent_support,
            accepted_until: authority.parent_support.start_ns(),
            segment_ordinal: 0,
            slab_ordinal: 0,
            event_ordinal: 0,
            last_accepted_step_ns: None,
            complete_owner_set: owners,
            active_regime_id: regime,
            active_segment_start: authority.parent_support.start_ns(),
            active_participant_set: participants,
            accepted_slab_receipts: Vec::new(),
            accepted_event_receipts: Vec::new(),
            scheduled_once_receipts: Vec::new(),
            controller_policy_sha256: policy,
            controller_checkpoint: checkpoint,
        })
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.accepted_until == self.parent_support.end_ns()
    }

    #[must_use]
    pub const fn accepted_until(&self) -> ModelTimeNs {
        self.accepted_until
    }
    #[must_use]
    pub fn owners(&self) -> &[OwnerState] {
        &self.complete_owner_set
    }
    #[must_use]
    pub fn active_participants(&self) -> &[String] {
        &self.active_participant_set
    }
    #[must_use]
    pub const fn slab_ordinal(&self) -> u32 {
        self.slab_ordinal
    }

    pub fn record_scheduled_once(
        &mut self,
        operation_id: String,
        boundary_id: Digest32,
        boundary: ModelTimeNs,
        result: Digest32,
    ) -> Result<ReceiptId, CoupledTimeError> {
        if boundary != self.accepted_until
            || self
                .scheduled_once_receipts
                .iter()
                .any(|r| r.operation_id == operation_id && r.boundary == boundary)
        {
            return Err(CoupledTimeError::ScheduledOnceReplay);
        }
        let tick = boundary.get().to_be_bytes();
        let digest = crate::framed_sha256(
            "event-receipt",
            &[
                crate::FramedField {
                    tag: "event_id",
                    value: boundary_id.as_bytes(),
                },
                crate::FramedField {
                    tag: "tick_ns",
                    value: &tick,
                },
                crate::FramedField {
                    tag: "ordinal",
                    value: &self.event_ordinal.to_be_bytes(),
                },
                crate::FramedField {
                    tag: "begin_owner_set",
                    value: result.as_bytes(),
                },
                crate::FramedField {
                    tag: "end_owner_set",
                    value: result.as_bytes(),
                },
                crate::FramedField {
                    tag: "ledger_digest",
                    value: result.as_bytes(),
                },
            ],
        )?;
        let receipt = ReceiptId::from_digest(digest);
        self.scheduled_once_receipts.push(ScheduledOnceReceiptV1 {
            operation_id,
            boundary_id,
            result_sha256: result,
            boundary,
            receipt_id: receipt,
        });
        Ok(receipt)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentAuthorityV1 {
    run_identity: Digest32,
    calendar_receipt: Digest32,
    forcing_receipt: Digest32,
    transaction_sequence: u128,
    parent_support: TimeSupport,
    parent_interval_id: ParentIntervalId,
    parent_transaction_id: ParentTransactionId,
}
impl ParentAuthorityV1 {
    pub fn new(
        run: Digest32,
        calendar: Digest32,
        forcing: Digest32,
        sequence: u128,
        support: TimeSupport,
        begin_owners: Digest32,
    ) -> Result<Self, CoupledTimeError> {
        let interval = ParentIntervalId::derive(run, calendar, forcing, support)?;
        let transaction = ParentTransactionId::derive(run, sequence, interval, begin_owners)?;
        Ok(Self {
            run_identity: run,
            calendar_receipt: calendar,
            forcing_receipt: forcing,
            transaction_sequence: sequence,
            parent_support: support,
            parent_interval_id: interval,
            parent_transaction_id: transaction,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledOnceReceiptV1 {
    pub(crate) operation_id: String,
    pub(crate) boundary_id: Digest32,
    pub(crate) boundary: ModelTimeNs,
    pub(crate) receipt_id: ReceiptId,
    pub(crate) result_sha256: Digest32,
}

pub(crate) fn validate_owner_and_participant_sets(
    owners: &[OwnerState],
    participants: &[String],
) -> Result<(), CoupledTimeError> {
    if owners.is_empty()
        || owners.windows(2).any(|w| w[0].owner_id >= w[1].owner_id)
        || participants.windows(2).any(|w| w[0] >= w[1])
    {
        return Err(CoupledTimeError::NonCanonicalIdentity);
    }
    if participants.iter().any(|p| {
        owners
            .binary_search_by(|o| o.owner_id.as_str().cmp(p))
            .is_err()
    }) {
        return Err(CoupledTimeError::InvalidParticipantSet);
    }
    if owners
        .iter()
        .any(|owner| crate::digest_bytes(&owner.state_bytes) != owner.state_digest)
    {
        return Err(CoupledTimeError::NonCanonicalIdentity);
    }
    Ok(())
}
