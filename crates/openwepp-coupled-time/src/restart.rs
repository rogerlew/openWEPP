use serde::{Deserialize, Serialize};

use crate::{CoupledClockStateV1, CoupledTimeError, Digest32, ReceiptId};

pub const RESTART_SCHEMA_ID: &str = "OPENWEPP_COUPLED_TIME_RESTART_V1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationRecordV1 {
    pub record_id: ReceiptId,
    pub payload: Vec<u8>,
    pub support_lineage_digest: Digest32,
    pub units: String,
    pub source_owner_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutboxState {
    Staged,
    CommittedUndelivered,
    DeliveredUnacknowledged,
    Acknowledged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationOutboxV1 {
    pub receipt_id: ReceiptId,
    pub sequence: u128,
    pub state: OutboxState,
    pub records: Vec<PublicationRecordV1>,
}

impl PublicationOutboxV1 {
    pub fn commit(&mut self, parent_complete: bool) -> Result<(), CoupledTimeError> {
        if !parent_complete {
            return Err(CoupledTimeError::PublicationBeforeParentCommit);
        }
        if self.state != OutboxState::Staged {
            return Err(CoupledTimeError::OutboxTransition);
        }
        self.state = OutboxState::CommittedUndelivered;
        Ok(())
    }
    pub fn mark_delivered(&mut self) -> Result<(), CoupledTimeError> {
        if !matches!(
            self.state,
            OutboxState::CommittedUndelivered | OutboxState::DeliveredUnacknowledged
        ) {
            return Err(CoupledTimeError::OutboxTransition);
        }
        self.state = OutboxState::DeliveredUnacknowledged;
        Ok(())
    }
    pub fn acknowledge(&mut self) -> Result<(), CoupledTimeError> {
        if self.state != OutboxState::DeliveredUnacknowledged {
            return Err(CoupledTimeError::OutboxTransition);
        }
        self.state = OutboxState::Acknowledged;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoupledTimeRestartV1 {
    pub schema_id: String,
    pub model_definition_sha256: Digest32,
    pub clock: CoupledClockStateV1,
    pub reduction_state: crate::DiagnosticReductionV1,
    pub publication_outbox: Option<PublicationOutboxV1>,
    pub pending_publication_records: Vec<PublicationRecordV1>,
}

impl CoupledTimeRestartV1 {
    pub fn validate(
        &self,
        expected_model: Digest32,
        expected_policy: Digest32,
    ) -> Result<(), CoupledTimeError> {
        if self.schema_id != RESTART_SCHEMA_ID
            || self.model_definition_sha256 != expected_model
            || self.clock.controller_policy_sha256 != expected_policy
        {
            return Err(CoupledTimeError::RestartInvalid);
        }
        if self.clock.accepted_until < self.clock.parent_support.start_ns
            || self.clock.accepted_until > self.clock.parent_support.end_ns
        {
            return Err(CoupledTimeError::RestartInvalid);
        }
        Ok(())
    }
}
