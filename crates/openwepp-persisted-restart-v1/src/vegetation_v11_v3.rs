//! Additive `OPENWEPP_C3_WOODY_V11_RESTART_V3` resource-custody restart.
//!
//! V3 embeds immutable V2 and exact typed custody vectors from the vegetation
//! checkpoint. Admission restores V2 first, then requires byte-exact prefix
//! equality; it never mints a second resource receipt identity.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use openwepp_vegetation::{
    V11_COMPLETE_OWNER_MANIFEST, V11AdmittedResourceFlux, V11CompleteOwnerCandidate,
    V11CoupledOwnedState, V11LseSupportReceiptEnvelope, V11OwnerEnvelope,
    V11ParentTransactionCheckpoint, V11ResourceDebit, V11SharedResourceKind,
    V11SharedResourceOwnerTransition,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{
    AdmittedVegetationV11RestartV2, CanonicalJsonError, VegetationV11RestartV2,
    VegetationV11RestartV2AdmissionContext, from_canonical_bytes, to_canonical_bytes,
};

pub const VEGETATION_V11_RESTART_V3_SCHEMA_ID: &str = "OPENWEPP_C3_WOODY_V11_RESTART_V3";

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum VegetationV11RestartV3Error {
    #[error("VEG-E-127: noncanonical V3 restart: {0}")]
    Canonical(String),
    #[error("VEG-E-127: embedded V2 admission failed")]
    V2,
    #[error("VEG-E-127: parent, support, cursor, or accepted receipt mismatch")]
    Position,
    #[error("VEG-E-127: typed resource-custody prefix mismatch")]
    Custody,
    #[error("VEG-E-127: terminal state or complete-owner mismatch")]
    Terminal,
}
impl From<CanonicalJsonError> for VegetationV11RestartV3Error {
    fn from(value: CanonicalJsonError) -> Self {
        Self::Canonical(value.to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV11SupportWireV3 {
    pub start_ns: String,
    pub end_ns: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV11AcceptedSlabWireV3 {
    pub slab_ordinal: u32,
    pub receipt_id: String,
    pub support: VegetationV11SupportWireV3,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV11CheckpointPositionV3 {
    pub accepted_until_ns: String,
    pub next_slab_ordinal: u32,
    pub next_event_ordinal: u32,
    pub accepted_slabs: Vec<VegetationV11AcceptedSlabWireV3>,
    pub accepted_event_receipt_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV11RestartV3 {
    pub schema: String,
    /// V2 remains immutable and is admitted, but is not a standalone
    /// production continuation for resource custody.
    pub v2_checkpoint: VegetationV11RestartV2,
    pub parent_transaction_id: String,
    pub parent_support: VegetationV11SupportWireV3,
    pub checkpoint_position: VegetationV11CheckpointPositionV3,
    #[serde(rename = "other_flux_receipts")]
    pub admitted_resource_fluxes: Vec<serde_json::Value>,
    #[serde(rename = "debit_receipts")]
    pub resource_debits: Vec<serde_json::Value>,
    #[serde(rename = "shared_owner_transitions")]
    pub shared_resource_transitions: Vec<serde_json::Value>,
    pub complete_owner_candidates: Vec<serde_json::Value>,
    pub terminal_complete_owners: Vec<serde_json::Value>,
    pub terminal_v11_state: serde_json::Value,
}

/// Isolated continuation; admission installs no owner and publishes no output.
pub struct AdmittedVegetationV11RestartV3 {
    pub v2: AdmittedVegetationV11RestartV2,
    /// Exact ordered LSE receipt envelopes retained inside the embedded parent
    /// checkpoint. These are projections of admitted checkpoint bytes, never
    /// caller-authored duplicates.
    pub lse_support_receipts: Vec<V11LseSupportReceiptEnvelope>,
    pub admitted_resource_fluxes: Vec<V11AdmittedResourceFlux>,
    pub resource_debits: Vec<V11ResourceDebit>,
    pub shared_resource_transitions: Vec<V11SharedResourceOwnerTransition>,
    pub complete_owner_candidates: Vec<V11CompleteOwnerCandidate>,
    canonical_restart: VegetationV11RestartV3,
}
impl AdmittedVegetationV11RestartV3 {
    #[must_use]
    pub const fn restart(&self) -> &VegetationV11RestartV3 {
        &self.canonical_restart
    }

    /// Consume the isolated V3 admission and continue through the already
    /// authenticated V2 parent runtime. Accepted prefix work is not replayed.
    #[must_use]
    pub fn into_continuation(self) -> AdmittedVegetationV11RestartV2 {
        self.v2
    }
}

impl VegetationV11RestartV3 {
    pub fn from_v2_checkpoint(
        v2_checkpoint: VegetationV11RestartV2,
        context: &VegetationV11RestartV2AdmissionContext<'_>,
    ) -> Result<Self, VegetationV11RestartV3Error> {
        let admitted = v2_checkpoint
            .clone()
            .admit(context)
            .map_err(|_| VegetationV11RestartV3Error::V2)?;
        let checkpoint = admitted.vegetation_parent.checkpoint();
        let accepted_slabs = checkpoint
            .accepted_segments
            .iter()
            .map(|segment| VegetationV11AcceptedSlabWireV3 {
                slab_ordinal: segment.slab_ordinal,
                receipt_id: digest_text(segment.receipt_id),
                support: VegetationV11SupportWireV3 {
                    start_ns: segment.support.start_ns().get().to_string(),
                    end_ns: segment.support.end_ns().get().to_string(),
                },
            })
            .collect::<Vec<_>>();
        let terminal_complete_owners = V11_COMPLETE_OWNER_MANIFEST
            .iter()
            .map(|owner| {
                checkpoint
                    .staged_complete_owners
                    .get(*owner)
                    .cloned()
                    .ok_or(VegetationV11RestartV3Error::Terminal)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let parent_support = embedded_v2_parent_support(&v2_checkpoint)?;
        let value = Self {
            schema: VEGETATION_V11_RESTART_V3_SCHEMA_ID.into(),
            parent_transaction_id: v2_checkpoint.parent_transaction_id.clone(),
            parent_support,
            checkpoint_position: VegetationV11CheckpointPositionV3 {
                accepted_until_ns: v2_checkpoint.accepted_until_ns.clone(),
                next_slab_ordinal: v2_checkpoint.next_slab_ordinal,
                next_event_ordinal: v2_checkpoint.next_event_ordinal,
                accepted_slabs,
                accepted_event_receipt_ids: v2_checkpoint
                    .accepted_event_receipts
                    .iter()
                    .map(|receipt| receipt.receipt_id.clone())
                    .collect(),
            },
            admitted_resource_fluxes: checkpoint
                .accepted_segments
                .iter()
                .flat_map(|segment| segment.admitted_resource_fluxes.iter().map(flux_wire))
                .collect(),
            resource_debits: checkpoint
                .accepted_segments
                .iter()
                .flat_map(|segment| segment.resource_debits.iter().map(debit_wire))
                .collect(),
            shared_resource_transitions: checkpoint
                .accepted_segments
                .iter()
                .flat_map(|segment| {
                    segment
                        .shared_resource_transitions
                        .iter()
                        .map(|transition| {
                            transition_wire(transition, &segment.complete_owner_candidates)
                        })
                })
                .collect(),
            complete_owner_candidates: checkpoint
                .accepted_segments
                .iter()
                .flat_map(|segment| segment.complete_owner_candidates.iter().map(candidate_wire))
                .collect(),
            terminal_complete_owners: terminal_complete_owners.iter().map(owner_wire).collect(),
            terminal_v11_state: state_wire(&checkpoint.staged_state)?,
            v2_checkpoint,
        };
        // Re-run admission so the builder and decoder have exactly one gate.
        value.clone().admit(context).map(|_| value)
    }

    pub fn to_canonical_json(&self) -> Result<Vec<u8>, VegetationV11RestartV3Error> {
        Ok(to_canonical_bytes(self)?)
    }

    pub fn admit_canonical_json(
        bytes: &[u8],
        context: &VegetationV11RestartV2AdmissionContext<'_>,
    ) -> Result<AdmittedVegetationV11RestartV3, VegetationV11RestartV3Error> {
        from_canonical_bytes::<Self>(bytes)?.admit(context)
    }

    pub fn admit(
        self,
        context: &VegetationV11RestartV2AdmissionContext<'_>,
    ) -> Result<AdmittedVegetationV11RestartV3, VegetationV11RestartV3Error> {
        if self.schema != VEGETATION_V11_RESTART_V3_SCHEMA_ID {
            return Err(VegetationV11RestartV3Error::Position);
        }
        let v2 = self
            .v2_checkpoint
            .clone()
            .admit(context)
            .map_err(|_| VegetationV11RestartV3Error::V2)?;
        let checkpoint = v2.vegetation_parent.checkpoint();
        self.validate_position(&v2, &checkpoint)?;
        self.validate_exact_prefix(&checkpoint)?;
        self.validate_terminal(&checkpoint)?;
        let admitted_resource_fluxes = checkpoint
            .accepted_segments
            .iter()
            .flat_map(|segment| segment.admitted_resource_fluxes.iter().cloned())
            .collect();
        let resource_debits = checkpoint
            .accepted_segments
            .iter()
            .flat_map(|segment| segment.resource_debits.iter().cloned())
            .collect();
        let shared_resource_transitions = checkpoint
            .accepted_segments
            .iter()
            .flat_map(|segment| segment.shared_resource_transitions.iter().cloned())
            .collect();
        let complete_owner_candidates = checkpoint
            .accepted_segments
            .iter()
            .flat_map(|segment| segment.complete_owner_candidates.iter().cloned())
            .collect();
        let lse_support_receipts = checkpoint
            .accepted_segments
            .iter()
            .map(|segment| segment.lse_support_receipt.clone())
            .collect();
        Ok(AdmittedVegetationV11RestartV3 {
            v2,
            lse_support_receipts,
            admitted_resource_fluxes,
            resource_debits,
            shared_resource_transitions,
            complete_owner_candidates,
            canonical_restart: self,
        })
    }

    fn validate_position(
        &self,
        _v2: &AdmittedVegetationV11RestartV2,
        checkpoint: &V11ParentTransactionCheckpoint,
    ) -> Result<(), VegetationV11RestartV3Error> {
        let expected_slabs = checkpoint
            .accepted_segments
            .iter()
            .map(|segment| VegetationV11AcceptedSlabWireV3 {
                slab_ordinal: segment.slab_ordinal,
                receipt_id: digest_text(segment.receipt_id),
                support: VegetationV11SupportWireV3 {
                    start_ns: segment.support.start_ns().get().to_string(),
                    end_ns: segment.support.end_ns().get().to_string(),
                },
            })
            .collect::<Vec<_>>();
        let expected_events = self
            .v2_checkpoint
            .accepted_event_receipts
            .iter()
            .map(|receipt| receipt.receipt_id.clone())
            .collect::<Vec<_>>();
        let expected_parent_support = embedded_v2_parent_support(&self.v2_checkpoint)?;
        if self.parent_transaction_id != self.v2_checkpoint.parent_transaction_id
            || self.parent_transaction_id != digest_text(checkpoint.parent_transaction_id)
            || self.checkpoint_position.accepted_until_ns != self.v2_checkpoint.accepted_until_ns
            || self.checkpoint_position.next_slab_ordinal != self.v2_checkpoint.next_slab_ordinal
            || self.checkpoint_position.next_event_ordinal != self.v2_checkpoint.next_event_ordinal
            || self.checkpoint_position.accepted_slabs != expected_slabs
            || self.checkpoint_position.accepted_event_receipt_ids != expected_events
            || self.parent_support != expected_parent_support
            || expected_slabs
                .last()
                .map(|slab| slab.support.end_ns.as_str())
                != Some(self.checkpoint_position.accepted_until_ns.as_str())
        {
            return Err(VegetationV11RestartV3Error::Position);
        }
        Ok(())
    }

    fn validate_exact_prefix(
        &self,
        checkpoint: &V11ParentTransactionCheckpoint,
    ) -> Result<(), VegetationV11RestartV3Error> {
        let expected_debits = checkpoint
            .accepted_segments
            .iter()
            .flat_map(|segment| segment.resource_debits.iter().map(debit_wire))
            .collect::<Vec<_>>();
        let expected_fluxes = checkpoint
            .accepted_segments
            .iter()
            .flat_map(|segment| segment.admitted_resource_fluxes.iter().map(flux_wire))
            .collect::<Vec<_>>();
        let expected_transitions = checkpoint
            .accepted_segments
            .iter()
            .flat_map(|segment| {
                segment
                    .shared_resource_transitions
                    .iter()
                    .map(|transition| {
                        transition_wire(transition, &segment.complete_owner_candidates)
                    })
            })
            .collect::<Vec<_>>();
        let expected_candidates = checkpoint
            .accepted_segments
            .iter()
            .flat_map(|segment| segment.complete_owner_candidates.iter().map(candidate_wire))
            .collect::<Vec<_>>();
        if self.resource_debits != expected_debits
            || self.admitted_resource_fluxes != expected_fluxes
            || self.shared_resource_transitions != expected_transitions
            || self.complete_owner_candidates != expected_candidates
            || self.complete_owner_candidates.len()
                != checkpoint.accepted_segments.len() * V11_COMPLETE_OWNER_MANIFEST.len()
        {
            return Err(VegetationV11RestartV3Error::Custody);
        }
        Ok(())
    }

    fn validate_terminal(
        &self,
        checkpoint: &V11ParentTransactionCheckpoint,
    ) -> Result<(), VegetationV11RestartV3Error> {
        let expected_owners = V11_COMPLETE_OWNER_MANIFEST
            .iter()
            .map(|owner| {
                checkpoint
                    .staged_complete_owners
                    .get(*owner)
                    .cloned()
                    .ok_or(VegetationV11RestartV3Error::Terminal)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let expected_owner_wires = expected_owners.iter().map(owner_wire).collect::<Vec<_>>();
        let expected_state_wire = state_wire(&checkpoint.staged_state)?;
        if self.terminal_complete_owners != expected_owner_wires
            || self.terminal_v11_state != expected_state_wire
        {
            return Err(VegetationV11RestartV3Error::Terminal);
        }
        Ok(())
    }
}

fn embedded_v2_parent_support(
    value: &VegetationV11RestartV2,
) -> Result<VegetationV11SupportWireV3, VegetationV11RestartV3Error> {
    let bytes = STANDARD
        .decode(&value.coupled_time_v2_canonical_base64)
        .map_err(|_| VegetationV11RestartV3Error::Position)?;
    let wire: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|_| VegetationV11RestartV3Error::Position)?;
    let support = wire
        .get("parent_support")
        .and_then(serde_json::Value::as_object)
        .ok_or(VegetationV11RestartV3Error::Position)?;
    let start_ns = support
        .get("start_ns")
        .and_then(serde_json::Value::as_str)
        .ok_or(VegetationV11RestartV3Error::Position)?;
    let end_ns = support
        .get("end_ns")
        .and_then(serde_json::Value::as_str)
        .ok_or(VegetationV11RestartV3Error::Position)?;
    Ok(VegetationV11SupportWireV3 {
        start_ns: start_ns.to_owned(),
        end_ns: end_ns.to_owned(),
    })
}

fn support_wire(support: openwepp_coupled_time::TimeSupport) -> serde_json::Value {
    serde_json::json!({
        "start_ns": support.start_ns().get().to_string(),
        "end_ns": support.end_ns().get().to_string(),
    })
}

fn resource_text(resource: &V11SharedResourceKind) -> &'static str {
    match resource {
        V11SharedResourceKind::Water => "water",
        V11SharedResourceKind::Ammonium => "ammonium",
        V11SharedResourceKind::Nitrate => "nitrate",
    }
}

fn debit_wire(value: &V11ResourceDebit) -> serde_json::Value {
    serde_json::json!({
        "receipt_id": digest_text(value.receipt_id),
        "parent_transaction_id": digest_text(value.parent_transaction_id),
        "segment_id": digest_text(value.segment_id),
        "accepted_slab_id": digest_text(value.accepted_slab_id),
        "support": support_wire(value.support),
        "owner_id": value.owner_id,
        "ofe_id": value.ofe_id,
        "tile_id": value.tile_id,
        "occupancy_id": value.occupancy_id,
        "layer_id": value.layer_id,
        "source_id": value.source_id,
        "amount_basis": value.amount_basis,
        "request_bits": format!("{:016x}", value.request.to_bits()),
        "authorization_bits": format!("{:016x}", value.authorization.to_bits()),
        "final_use_bits": format!("{:016x}", value.final_use.to_bits()),
    })
}

fn flux_wire(value: &V11AdmittedResourceFlux) -> serde_json::Value {
    let key = &value.shared_resource_key;
    serde_json::json!({
        "receipt_id": digest_text(value.receipt_id),
        "parent_transaction_id": digest_text(value.parent_transaction_id),
        "segment_id": digest_text(value.segment_id),
        "accepted_slab_id": digest_text(value.accepted_slab_id),
        "support": support_wire(value.support),
        "flux_class": value.flux_class,
        "direction": value.direction,
        "source_owner_id": value.source_owner_id,
        "receiver_owner_id": value.receiver_owner_id,
        "resource_id": resource_text(&key.resource),
        "ofe_id": key.ofe_id,
        "layer_id": key.layer_id,
        "source_id": key.source_id,
        "amount_basis": key.amount_basis,
        "amount_bits": format!("{:016x}", value.amount.to_bits()),
    })
}

fn transition_wire(
    value: &V11SharedResourceOwnerTransition,
    candidates: &[V11CompleteOwnerCandidate],
) -> serde_json::Value {
    let key = &value.shared_resource_key;
    let candidate_bytes = candidates
        .iter()
        .find(|candidate| candidate.owner_id == key.owner_id)
        .map_or(&[][..], |candidate| {
            candidate.ending_owner.state_bytes.as_slice()
        });
    serde_json::json!({
        "transition_id": digest_text(value.transition_id),
        "parent_transaction_id": digest_text(value.parent_transaction_id),
        "segment_id": digest_text(value.segment_id),
        "accepted_slab_id": digest_text(value.accepted_slab_id),
        "support": support_wire(value.support),
        "owner_id": key.owner_id,
        "ofe_id": key.ofe_id,
        "layer_id": key.layer_id,
        "source_id": key.source_id,
        "amount_basis": key.amount_basis,
        "beginning_bits": format!("{:016x}", value.beginning_amount.to_bits()),
        "ending_bits": format!("{:016x}", value.ending_amount.to_bits()),
        "debit_receipt_ids": value.debit_receipt_ids.iter().map(|id| digest_text(*id)).collect::<Vec<_>>(),
        "other_flux_receipt_ids": value.admitted_flux_receipt_ids.iter().map(|id| digest_text(*id)).collect::<Vec<_>>(),
        "owner_candidate_bytes_base64": STANDARD.encode(candidate_bytes),
        "owner_candidate_sha256": digest_text(value.owner_candidate_sha256),
    })
}

fn candidate_wire(value: &V11CompleteOwnerCandidate) -> serde_json::Value {
    let components = value.components.iter().map(|component| serde_json::json!({
        "component_id": component.shared_resource_key.source_id,
        "ending_bits": format!("{:016x}", component.ending_amount_bits),
        "debit_receipt_ids": component.debit_receipt_ids.iter().map(|id| digest_text(*id)).collect::<Vec<_>>(),
        "other_flux_receipt_ids": component.admitted_flux_receipt_ids.iter().map(|id| digest_text(*id)).collect::<Vec<_>>(),
    })).collect::<Vec<_>>();
    serde_json::json!({
        "parent_transaction_id": digest_text(value.parent_transaction_id),
        "segment_id": digest_text(value.segment_id),
        "accepted_slab_id": digest_text(value.accepted_slab_id),
        "owner_id": value.owner_id,
        "slab_ordinal": value.slab_ordinal,
        "support": support_wire(value.support),
        "components": components,
        "state_bytes_base64": STANDARD.encode(&value.ending_owner.state_bytes),
        "state_sha256": digest_text(value.ending_owner.state_sha256),
    })
}

fn owner_wire(value: &V11OwnerEnvelope) -> serde_json::Value {
    serde_json::json!({
        "owner_id": value.owner_id,
        "state_bytes_base64": STANDARD.encode(&value.state_bytes),
        "state_sha256": digest_text(value.state_sha256),
    })
}

fn state_wire(
    value: &V11CoupledOwnedState,
) -> Result<serde_json::Value, VegetationV11RestartV3Error> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| VegetationV11RestartV3Error::Canonical(error.to_string()))?;
    Ok(serde_json::json!({
        "state_canonical_base64": STANDARD.encode(&bytes),
        "state_sha256": format!("{:x}", Sha256::digest(&bytes)),
    }))
}

fn digest_text<T: Serialize>(value: T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_default()
}

/// Canonical helper for byte-pinning the embedded immutable V2 wire.
pub fn vegetation_v11_restart_v2_canonical_blob(
    value: &VegetationV11RestartV2,
) -> Result<(String, String), VegetationV11RestartV3Error> {
    let bytes = value
        .to_canonical_json()
        .map_err(|error| VegetationV11RestartV3Error::Canonical(error.to_string()))?;
    Ok((
        STANDARD.encode(&bytes),
        format!("{:x}", Sha256::digest(&bytes)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v3_schema_is_closed_and_not_a_v2_alias() {
        let value = serde_json::json!({
            "schema": VEGETATION_V11_RESTART_V3_SCHEMA_ID,
            "unknown": true
        });
        let bytes = to_canonical_bytes(&value).unwrap();
        assert!(from_canonical_bytes::<VegetationV11RestartV3>(&bytes).is_err());
        assert_ne!(
            VEGETATION_V11_RESTART_V3_SCHEMA_ID,
            crate::VEGETATION_V11_RESTART_V2_SCHEMA_ID
        );
    }
}
