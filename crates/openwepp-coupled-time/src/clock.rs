use serde::{Deserialize, Serialize};

use crate::{
    CoupledTimeError, Digest32, ModelTimeNs, ParentIntervalId, ParentTransactionId, ReceiptId,
    TimeSupport,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerState {
    pub owner_id: String,
    pub state_bytes: Vec<u8>,
    pub state_digest: Digest32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoupledClockStateV1 {
    pub parent_interval_id: ParentIntervalId,
    pub parent_transaction_id: ParentTransactionId,
    pub parent_support: TimeSupport,
    pub accepted_until: ModelTimeNs,
    pub segment_ordinal: u32,
    pub slab_ordinal: u32,
    pub event_ordinal: u32,
    pub last_accepted_step_ns: Option<u128>,
    pub complete_owner_set: Vec<OwnerState>,
    pub active_regime_id: String,
    pub active_participant_set: Vec<String>,
    pub accepted_event_receipts: Vec<ReceiptId>,
    pub scheduled_once_receipts: Vec<ReceiptId>,
    pub controller_policy_sha256: Digest32,
    pub controller_checkpoint: Vec<u8>,
}

impl CoupledClockStateV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        parent_interval_id: ParentIntervalId,
        parent_transaction_id: ParentTransactionId,
        parent_support: TimeSupport,
        owners: Vec<OwnerState>,
        regime: String,
        participants: Vec<String>,
        policy: Digest32,
        checkpoint: Vec<u8>,
    ) -> Result<Self, CoupledTimeError> {
        validate_owner_and_participant_sets(&owners, &participants)?;
        Ok(Self {
            parent_interval_id,
            parent_transaction_id,
            parent_support,
            accepted_until: parent_support.start_ns,
            segment_ordinal: 0,
            slab_ordinal: 0,
            event_ordinal: 0,
            last_accepted_step_ns: None,
            complete_owner_set: owners,
            active_regime_id: regime,
            active_participant_set: participants,
            accepted_event_receipts: Vec::new(),
            scheduled_once_receipts: Vec::new(),
            controller_policy_sha256: policy,
            controller_checkpoint: checkpoint,
        })
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.accepted_until == self.parent_support.end_ns
    }

    pub fn record_scheduled_once(&mut self, receipt: ReceiptId) -> Result<(), CoupledTimeError> {
        if self.scheduled_once_receipts.contains(&receipt) {
            return Err(CoupledTimeError::ScheduledOnceReplay);
        }
        self.scheduled_once_receipts.push(receipt);
        Ok(())
    }
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
