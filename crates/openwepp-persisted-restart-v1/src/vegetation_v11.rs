//! Additive `OPENWEPP_C3_WOODY_V11_RESTART_V2` admission and restoration.
//!
//! The reviewed reduced V1 shape is intentionally absent from production. V2
//! embeds the complete typed vegetation parent checkpoint and the released
//! coupled-time V2 checkpoint without changing either predecessor wire.

use std::collections::{BTreeMap, BTreeSet};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use openwepp_coupled_time::{CoupledTimeRestartV2, Digest32, ParentTransactionId};
use openwepp_vegetation::{
    V11_COMPLETE_OWNER_MANIFEST, V11BgcDebitScope, V11OwnerEnvelope, V11ParentTransaction,
    V11ParentTransactionCheckpoint, VegetationConfigurationV11,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{CanonicalJsonError, from_canonical_bytes, to_canonical_bytes};

pub const VEGETATION_V11_RESTART_V2_SCHEMA_ID: &str = "OPENWEPP_C3_WOODY_V11_RESTART_V2";
/// V2 remains byte-stable and admissible as V3's embedded predecessor, but it
/// lacks typed shared-resource transitions and is never a standalone
/// production continuation for V11 resource custody.
pub const VEGETATION_V11_RESTART_V2_RESOURCE_CUSTODY_PRODUCTION_ADMITTED: bool = false;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum VegetationV11RestartV2Error {
    #[error("VEG-E-127: noncanonical restart bytes: {0}")]
    Canonical(String),
    #[error("VEG-E-127: restart authority or identity mismatch: {0}")]
    Identity(&'static str),
    #[error("VEG-E-127: coupled-time V2 admission failed")]
    CoupledTime,
    #[error("VEG-E-127: vegetation parent checkpoint admission failed")]
    ParentCheckpoint,
    #[error("VEG-E-127: complete owner custody mismatch")]
    Owners,
    #[error("VEG-E-127: event or scheduled chronology mismatch")]
    Chronology,
    #[error("VEG-E-127: reduction or publication chronology mismatch")]
    Publication,
}
impl From<CanonicalJsonError> for VegetationV11RestartV2Error {
    fn from(value: CanonicalJsonError) -> Self {
        Self::Canonical(value.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct VegetationV11RestartV2AdmissionContext<'a> {
    pub authority_sha256: Digest32,
    pub configuration: &'a VegetationConfigurationV11,
    pub coupled_model_sha256: Digest32,
    pub coupled_authority_sha256: Digest32,
    pub controller_policy_sha256: Digest32,
    pub bgc_debit_scope: Option<&'a V11BgcDebitScope>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV11OwnerWireV2 {
    pub owner_id: String,
    pub state_canonical_base64: String,
    pub state_sha256: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV11EventReceiptWireV2 {
    pub kind: String,
    pub ordinal: u32,
    pub parent_transaction_id: String,
    pub payload_canonical_base64: String,
    pub payload_sha256: String,
    pub receipt_id: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV11ReductionOperandWireV2 {
    pub operand_id: String,
    pub source_receipt_id: String,
    pub support_start_ns: String,
    pub support_end_ns: String,
    pub value_bits: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV11PublicationWireV2 {
    pub record_id: String,
    pub source_reduction_id: String,
    pub payload_canonical_base64: String,
    pub payload_sha256: String,
}
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum VegetationV11OutboxStateV2 {
    CommittedUndelivered,
    Delivered,
    Acknowledged,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV11OutboxWireV2 {
    pub outbox_id: String,
    pub record_id: String,
    pub state: VegetationV11OutboxStateV2,
    pub delivery_count: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV11RestartV2 {
    pub schema: String,
    pub authority_sha256: String,
    pub configuration_sha256: String,
    pub parent_transaction_id: String,
    pub parent_transaction_sequence: String,
    pub next_parent_transaction_sequence: String,
    pub active_segment_id: String,
    pub active_regime_id: String,
    pub accepted_until_ns: String,
    pub next_slab_ordinal: u32,
    pub next_event_ordinal: u32,
    pub active_participant_ids: Vec<String>,
    pub controller_policy_sha256: String,
    pub coupled_time_v2_canonical_base64: String,
    pub coupled_time_v2_sha256: String,
    pub parent_checkpoint_canonical_base64: String,
    pub parent_checkpoint_sha256: String,
    pub beginning_complete_owners: Vec<VegetationV11OwnerWireV2>,
    pub staged_complete_owners: Vec<VegetationV11OwnerWireV2>,
    pub accepted_event_receipts: Vec<VegetationV11EventReceiptWireV2>,
    pub scheduled_execution_keys: Vec<String>,
    pub reduction_operands: Vec<VegetationV11ReductionOperandWireV2>,
    pub pending_publication_records: Vec<VegetationV11PublicationWireV2>,
    pub publication_outbox: Vec<VegetationV11OutboxWireV2>,
}

pub struct AdmittedVegetationV11RestartV2 {
    pub coupled_time: CoupledTimeRestartV2,
    pub vegetation_parent: V11ParentTransaction,
    pub event_receipts: Vec<VegetationV11EventReceiptWireV2>,
    pub scheduled_execution_keys: Vec<String>,
    pub reduction_operands: Vec<VegetationV11ReductionOperandWireV2>,
    pub pending_publication_records: Vec<VegetationV11PublicationWireV2>,
    pub publication_outbox: Vec<VegetationV11OutboxWireV2>,
    canonical_restart: VegetationV11RestartV2,
}
impl AdmittedVegetationV11RestartV2 {
    #[must_use]
    pub const fn restart(&self) -> &VegetationV11RestartV2 {
        &self.canonical_restart
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct EventBodyV2 {
    tick_ns: String,
    from_participants: Vec<String>,
    to_participants: Vec<String>,
    source_owner: String,
    receiver_owner: String,
    transfer_bits: String,
}

impl VegetationV11RestartV2 {
    pub fn to_canonical_json(&self) -> Result<Vec<u8>, VegetationV11RestartV2Error> {
        Ok(to_canonical_bytes(self)?)
    }
    pub fn admit_canonical_json(
        bytes: &[u8],
        context: &VegetationV11RestartV2AdmissionContext<'_>,
    ) -> Result<AdmittedVegetationV11RestartV2, VegetationV11RestartV2Error> {
        from_canonical_bytes::<Self>(bytes)?.admit(context)
    }
    #[allow(clippy::too_many_lines)]
    pub fn admit(
        self,
        context: &VegetationV11RestartV2AdmissionContext<'_>,
    ) -> Result<AdmittedVegetationV11RestartV2, VegetationV11RestartV2Error> {
        if self.schema != VEGETATION_V11_RESTART_V2_SCHEMA_ID {
            return Err(VegetationV11RestartV2Error::Identity("schema"));
        }
        require_digest(
            &self.authority_sha256,
            context.authority_sha256,
            "authority",
        )?;
        if self.configuration_sha256 != context.configuration.configuration_sha256 {
            return Err(VegetationV11RestartV2Error::Identity("configuration"));
        }
        require_digest(
            &self.controller_policy_sha256,
            context.controller_policy_sha256,
            "controller policy",
        )?;
        let current = decimal_u128(&self.parent_transaction_sequence)?;
        if current.checked_add(1) != Some(decimal_u128(&self.next_parent_transaction_sequence)?) {
            return Err(VegetationV11RestartV2Error::Identity("parent sequence"));
        }
        let coupled_bytes = decode_blob(
            &self.coupled_time_v2_canonical_base64,
            &self.coupled_time_v2_sha256,
        )?;
        let coupled_time = CoupledTimeRestartV2::from_canonical_json(
            &coupled_bytes,
            context.coupled_model_sha256,
            context.coupled_authority_sha256,
            context.controller_policy_sha256,
        )
        .map_err(|_| VegetationV11RestartV2Error::CoupledTime)?;
        let coupled_wire: Value = serde_json::from_slice(&coupled_bytes)
            .map_err(|_| VegetationV11RestartV2Error::CoupledTime)?;
        self.validate_coupled_joins(&coupled_wire, &coupled_time)?;
        let checkpoint_bytes = decode_blob(
            &self.parent_checkpoint_canonical_base64,
            &self.parent_checkpoint_sha256,
        )?;
        let checkpoint: V11ParentTransactionCheckpoint = serde_json::from_slice(&checkpoint_bytes)
            .map_err(|_| VegetationV11RestartV2Error::ParentCheckpoint)?;
        if serde_json::to_vec(&checkpoint)
            .map_err(|_| VegetationV11RestartV2Error::ParentCheckpoint)?
            != checkpoint_bytes
        {
            return Err(VegetationV11RestartV2Error::ParentCheckpoint);
        }
        self.validate_checkpoint_joins(&checkpoint, current)?;
        let vegetation_parent = V11ParentTransaction::restore_with_bgc_scope(
            context.configuration,
            checkpoint,
            context.bgc_debit_scope,
        )
        .map_err(|_| VegetationV11RestartV2Error::ParentCheckpoint)?;
        self.validate_events()?;
        self.validate_scheduled()?;
        self.validate_publication()?;
        Ok(AdmittedVegetationV11RestartV2 {
            coupled_time,
            vegetation_parent,
            event_receipts: self.accepted_event_receipts.clone(),
            scheduled_execution_keys: self.scheduled_execution_keys.clone(),
            reduction_operands: self.reduction_operands.clone(),
            pending_publication_records: self.pending_publication_records.clone(),
            publication_outbox: self.publication_outbox.clone(),
            canonical_restart: self,
        })
    }
    fn validate_coupled_joins(
        &self,
        value: &Value,
        admitted: &CoupledTimeRestartV2,
    ) -> Result<(), VegetationV11RestartV2Error> {
        let clock = admitted.clock();
        let participant_set = self.active_participant_ids.iter().collect::<BTreeSet<_>>();
        if participant_set.len() != self.active_participant_ids.len()
            || self.active_participant_ids.is_empty()
            || clock.accepted_until().get() != decimal_u128(&self.accepted_until_ns)?
            || clock.slab_ordinal() != self.next_slab_ordinal
            || clock.active_participants() != self.active_participant_ids
            || value["parent_transaction_id"].as_str() != Some(&self.parent_transaction_id)
            || value["next_event_ordinal"].as_u64() != Some(u64::from(self.next_event_ordinal))
            || value["active_segment"]["segment_id"].as_str() != Some(&self.active_segment_id)
            || value["active_segment"]["regime_id"].as_str() != Some(&self.active_regime_id)
            || value["parent_transaction_sequence"].as_str()
                != Some(&self.parent_transaction_sequence)
            || value["next_parent_transaction_sequence"].as_str()
                != Some(&self.next_parent_transaction_sequence)
            || value["controller_policy_sha256"].as_str() != Some(&self.controller_policy_sha256)
        {
            return Err(VegetationV11RestartV2Error::CoupledTime);
        }
        Ok(())
    }
    fn validate_checkpoint_joins(
        &self,
        checkpoint: &V11ParentTransactionCheckpoint,
        current: u128,
    ) -> Result<(), VegetationV11RestartV2Error> {
        if digest_text(checkpoint.parent_transaction_id) != self.parent_transaction_id
            || checkpoint.accepted_until_ns != decimal_u128(&self.accepted_until_ns)?
            || checkpoint.accepted_segments.len() != self.next_slab_ordinal as usize
            || checkpoint
                .beginning_state
                .last_parent_transaction_id
                .checked_add(1)
                != Some(current)
            || checkpoint
                .staged_state
                .last_parent_transaction_id
                .checked_add(1)
                != Some(current)
        {
            return Err(VegetationV11RestartV2Error::ParentCheckpoint);
        }
        let beginning = owner_map(&self.beginning_complete_owners)?;
        let staged = owner_map(&self.staged_complete_owners)?;
        let mut predecessor = &checkpoint.beginning_state;
        for segment in &checkpoint.accepted_segments {
            if segment.beginning_state_sha256 != predecessor.state_sha256 {
                return Err(VegetationV11RestartV2Error::ParentCheckpoint);
            }
            predecessor = &segment.ending_state;
        }
        if checkpoint.beginning_complete_owners != beginning
            || checkpoint.staged_complete_owners != staged
            || predecessor != &checkpoint.staged_state
            || checkpoint
                .accepted_segments
                .last()
                .map(|s| &s.ending_resource_owners)
                != Some(&staged)
        {
            return Err(VegetationV11RestartV2Error::Owners);
        }
        Ok(())
    }
    fn validate_events(&self) -> Result<(), VegetationV11RestartV2Error> {
        if self.accepted_event_receipts.len() != self.next_event_ordinal as usize {
            return Err(VegetationV11RestartV2Error::Chronology);
        }
        let mut ids = BTreeSet::new();
        for (ordinal, r) in self.accepted_event_receipts.iter().enumerate() {
            let ordinal =
                u32::try_from(ordinal).map_err(|_| VegetationV11RestartV2Error::Chronology)?;
            if r.kind != "event"
                || r.ordinal != ordinal
                || r.parent_transaction_id != self.parent_transaction_id
                || !ids.insert(r.receipt_id.clone())
            {
                return Err(VegetationV11RestartV2Error::Chronology);
            }
            validate_sha(&r.receipt_id)?;
            let payload = decode_blob(&r.payload_canonical_base64, &r.payload_sha256)?;
            let expected_receipt = sha256(
                [
                    b"event".as_slice(),
                    &r.ordinal.to_be_bytes(),
                    self.parent_transaction_id.as_bytes(),
                    &payload,
                ]
                .concat(),
            );
            if r.receipt_id != expected_receipt {
                return Err(VegetationV11RestartV2Error::Chronology);
            }
            let b: EventBodyV2 = from_canonical_bytes(&payload)?;
            if decimal_u128(&b.tick_ns)? > decimal_u128(&self.accepted_until_ns)?
                || b.to_participants != self.active_participant_ids
                || b.from_participants.is_empty()
                || b.source_owner.is_empty()
                || b.receiver_owner.is_empty()
                || b.source_owner == b.receiver_owner
            {
                return Err(VegetationV11RestartV2Error::Chronology);
            }
            f64_bits(&b.transfer_bits)?;
        }
        Ok(())
    }
    fn validate_scheduled(&self) -> Result<(), VegetationV11RestartV2Error> {
        let mut prior = None;
        for k in &self.scheduled_execution_keys {
            validate_sha(k)?;
            if prior.as_ref().is_some_and(|p: &String| p >= k) {
                return Err(VegetationV11RestartV2Error::Chronology);
            }
            prior = Some(k.clone());
        }
        Ok(())
    }
    fn validate_publication(&self) -> Result<(), VegetationV11RestartV2Error> {
        let mut operand_ids = BTreeSet::new();
        let mut previous_operand = None;
        for o in &self.reduction_operands {
            validate_sha(&o.source_receipt_id)?;
            let start = decimal_u128(&o.support_start_ns)?;
            let end = decimal_u128(&o.support_end_ns)?;
            if start >= end || end > decimal_u128(&self.accepted_until_ns)? {
                return Err(VegetationV11RestartV2Error::Publication);
            }
            f64_bits(&o.value_bits)?;
            let expected = hash_json(
                &serde_json::json!({"source_receipt_id":o.source_receipt_id,"support_start_ns":o.support_start_ns,"support_end_ns":o.support_end_ns,"value_bits":o.value_bits}),
            )?;
            if o.operand_id != expected
                || previous_operand
                    .as_ref()
                    .is_some_and(|prior: &String| prior >= &o.operand_id)
                || !operand_ids.insert(o.operand_id.clone())
            {
                return Err(VegetationV11RestartV2Error::Publication);
            }
            previous_operand = Some(o.operand_id.clone());
        }
        let reduction_id = hash_json(&operand_ids.iter().collect::<Vec<_>>())?;
        let mut records = BTreeSet::new();
        let mut previous_record = None;
        for r in &self.pending_publication_records {
            let payload = decode_blob(&r.payload_canonical_base64, &r.payload_sha256)?;
            let _: Value = from_canonical_bytes(&payload)?;
            let expected = sha256([reduction_id.as_bytes(), r.payload_sha256.as_bytes()].concat());
            if r.source_reduction_id != reduction_id
                || r.record_id != expected
                || previous_record
                    .as_ref()
                    .is_some_and(|prior: &String| prior >= &r.record_id)
                || !records.insert(r.record_id.clone())
            {
                return Err(VegetationV11RestartV2Error::Publication);
            }
            previous_record = Some(r.record_id.clone());
        }
        let mut outbox = BTreeSet::new();
        let mut bound = BTreeSet::new();
        for r in &self.publication_outbox {
            let state = match r.state {
                VegetationV11OutboxStateV2::CommittedUndelivered => "CommittedUndelivered",
                VegetationV11OutboxStateV2::Delivered => "Delivered",
                VegetationV11OutboxStateV2::Acknowledged => "Acknowledged",
            };
            let expected = hash_json(
                &serde_json::json!({"delivery_count":r.delivery_count,"parent_transaction_id":self.parent_transaction_id,"record_id":r.record_id,"state":state}),
            )?;
            if r.outbox_id != expected
                || !records.contains(&r.record_id)
                || !outbox.insert(r.outbox_id.clone())
                || !bound.insert(r.record_id.clone())
                || (matches!(r.state, VegetationV11OutboxStateV2::CommittedUndelivered)
                    && r.delivery_count != 0)
                || (!matches!(r.state, VegetationV11OutboxStateV2::CommittedUndelivered)
                    && r.delivery_count == 0)
            {
                return Err(VegetationV11RestartV2Error::Publication);
            }
        }
        if bound != records {
            return Err(VegetationV11RestartV2Error::Publication);
        }
        Ok(())
    }
}

fn owner_map(
    values: &[VegetationV11OwnerWireV2],
) -> Result<BTreeMap<String, V11OwnerEnvelope>, VegetationV11RestartV2Error> {
    if values.len() != V11_COMPLETE_OWNER_MANIFEST.len() {
        return Err(VegetationV11RestartV2Error::Owners);
    }
    let mut map = BTreeMap::new();
    for (expected, v) in V11_COMPLETE_OWNER_MANIFEST.iter().zip(values) {
        if v.owner_id != *expected {
            return Err(VegetationV11RestartV2Error::Owners);
        }
        let bytes = decode_blob(&v.state_canonical_base64, &v.state_sha256)?;
        let _: Value =
            from_canonical_bytes(&bytes).map_err(|_| VegetationV11RestartV2Error::Owners)?;
        let envelope = V11OwnerEnvelope::try_new(v.owner_id.clone(), bytes)
            .map_err(|_| VegetationV11RestartV2Error::Owners)?;
        envelope
            .to_owner_state()
            .map_err(|_| VegetationV11RestartV2Error::Owners)?;
        if map.insert(v.owner_id.clone(), envelope).is_some() {
            return Err(VegetationV11RestartV2Error::Owners);
        }
    }
    Ok(map)
}
fn decode_blob(text: &str, digest: &str) -> Result<Vec<u8>, VegetationV11RestartV2Error> {
    validate_sha(digest)?;
    let bytes = STANDARD
        .decode(text)
        .map_err(|_| VegetationV11RestartV2Error::Identity("base64"))?;
    if STANDARD.encode(&bytes) != text || sha256(&bytes) != digest {
        return Err(VegetationV11RestartV2Error::Identity("payload digest"));
    }
    Ok(bytes)
}
fn require_digest(
    text: &str,
    expected: Digest32,
    field: &'static str,
) -> Result<(), VegetationV11RestartV2Error> {
    let parsed: Digest32 = serde_json::from_value(Value::String(text.into()))
        .map_err(|_| VegetationV11RestartV2Error::Identity(field))?;
    if parsed != expected {
        return Err(VegetationV11RestartV2Error::Identity(field));
    }
    Ok(())
}
fn validate_sha(text: &str) -> Result<(), VegetationV11RestartV2Error> {
    serde_json::from_value::<Digest32>(Value::String(text.into()))
        .map(|_| ())
        .map_err(|_| VegetationV11RestartV2Error::Identity("sha256"))
}
fn decimal_u128(text: &str) -> Result<u128, VegetationV11RestartV2Error> {
    if text.is_empty() || (text.len() > 1 && text.starts_with('0')) {
        return Err(VegetationV11RestartV2Error::Identity("u128"));
    }
    text.parse()
        .map_err(|_| VegetationV11RestartV2Error::Identity("u128"))
}
fn f64_bits(text: &str) -> Result<u64, VegetationV11RestartV2Error> {
    if text.len() != 16
        || !text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(VegetationV11RestartV2Error::Identity("f64 bits"));
    }
    u64::from_str_radix(text, 16).map_err(|_| VegetationV11RestartV2Error::Identity("f64 bits"))
}
fn digest_text(value: ParentTransactionId) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned))
        .unwrap_or_default()
}
fn hash_json<T: Serialize>(value: &T) -> Result<String, VegetationV11RestartV2Error> {
    Ok(sha256(&to_canonical_bytes(value)?))
}
fn sha256(bytes: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(bytes.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(byte: u8) -> String {
        format!("{byte:02x}").repeat(32)
    }

    fn wire() -> VegetationV11RestartV2 {
        VegetationV11RestartV2 {
            schema: VEGETATION_V11_RESTART_V2_SCHEMA_ID.into(),
            authority_sha256: d(1),
            configuration_sha256: d(2),
            parent_transaction_id: d(3),
            parent_transaction_sequence: "4".into(),
            next_parent_transaction_sequence: "5".into(),
            active_segment_id: d(4),
            active_regime_id: "snow-free".into(),
            accepted_until_ns: "600000000000".into(),
            next_slab_ordinal: 1,
            next_event_ordinal: 0,
            active_participant_ids: vec!["vegetation".into()],
            controller_policy_sha256: d(5),
            coupled_time_v2_canonical_base64: STANDARD.encode(b"{}"),
            coupled_time_v2_sha256: sha256(b"{}"),
            parent_checkpoint_canonical_base64: STANDARD.encode(b"{}"),
            parent_checkpoint_sha256: sha256(b"{}"),
            beginning_complete_owners: vec![],
            staged_complete_owners: vec![],
            accepted_event_receipts: vec![],
            scheduled_execution_keys: vec![],
            reduction_operands: vec![],
            pending_publication_records: vec![],
            publication_outbox: vec![],
        }
    }

    #[test]
    fn v2_wire_is_canonical_closed_and_v1_has_no_runtime_type() {
        let bytes = wire().to_canonical_json().unwrap();
        assert_eq!(
            from_canonical_bytes::<VegetationV11RestartV2>(&bytes).unwrap(),
            wire()
        );
        let text = String::from_utf8(bytes).unwrap();
        let unknown = text.replacen('{', "{\"unknown\":0,", 1);
        assert!(from_canonical_bytes::<VegetationV11RestartV2>(unknown.as_bytes()).is_err());
        assert!(!text.contains("OPENWEPP_C3_WOODY_V11_RESTART_V1"));
    }

    #[test]
    fn canonical_blob_and_numeric_poisons_reject() {
        assert_eq!(decode_blob("e30=", &sha256(b"{}")).unwrap(), b"{}");
        assert!(decode_blob("e30", &sha256(b"{}")).is_err());
        assert!(decode_blob("e30=", &d(9)).is_err());
        assert!(decimal_u128("01").is_err());
        assert!(f64_bits("3ff000000000000G").is_err());
    }
}
