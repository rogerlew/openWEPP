//! Canonical complete-owner projection for an accepted frozen-litter child.
//!
//! This is an immutable serialization/join surface. It does not execute LSE,
//! current ingress, WB14, or soil-thermal physics and cannot publish an owner.

use std::collections::BTreeSet;

use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{
    LitterPhaseReceipt, SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2,
    litter_phase_receipt_from_json, litter_phase_receipt_json,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::v2_ingress_adapter::DirectWb14ParentWorkingStateV2;
use super::{
    DirectSurfaceLiquidError, SurfaceClass, SurfaceLiquidConfigurationV2,
    SurfaceLiquidOwnerEnvelopeV2, ZERO_SHA256, is_sha256,
};
use crate::direct_runtime::surface_liquid_ingress::DirectSurfaceLiquidParcelReceipt;

pub const SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V3_SCHEMA: &str =
    "OPENWEPP_SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V3";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceLiquidCompleteOwnerProjectionIdentityV3 {
    pub run_id: u64,
    pub transaction_id: TransactionId,
    pub predecessor_transaction_id: Option<TransactionId>,
    pub parent_support_start_ns: u128,
    pub parent_support_end_ns: u128,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub beginning_surface_owner_sha256: String,
    pub phase_adjusted_surface_owner_sha256: String,
    pub predecessor_receipt_chain_sha256: String,
    pub receipt_chain_sha256: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceLiquidCompleteOwnerProjectionV3 {
    schema_sha256: String,
    model_definition_sha256: String,
    configuration_sha256: String,
    parent_identity_sha256: String,
    identity: SurfaceLiquidCompleteOwnerProjectionIdentityV3,
    envelope_sha256: String,
    envelope_bytes: Vec<u8>,
    wb14_parent_working_state_bytes: Vec<u8>,
    litter_vapor_receipt_bytes: Vec<Vec<u8>>,
    litter_phase_receipt_bytes: Vec<Vec<u8>>,
    current_ingress_receipt_bytes: Vec<Vec<u8>>,
    soil_thermal_owner_state_sha256: String,
    soil_thermal_receipt_chain_sha256: String,
    soil_thermal_owner_envelope_bytes: Vec<u8>,
    soil_thermal_restart_identity_bytes: Vec<u8>,
    projection_sha256: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidCompleteOwnerProjectionV3 {
    schema: String,
    schema_sha256: String,
    model_definition_sha256: String,
    configuration_sha256: String,
    parent_identity_sha256: String,
    run_id: u64,
    transaction_id: TransactionId,
    predecessor_transaction_id: Option<TransactionId>,
    parent_support_start_ns: u128,
    parent_support_end_ns: u128,
    support_start_ns: u128,
    support_end_ns: u128,
    beginning_surface_owner_sha256: String,
    phase_adjusted_surface_owner_sha256: String,
    predecessor_receipt_chain_sha256: String,
    receipt_chain_sha256: String,
    envelope_sha256: String,
    envelope_bytes: Vec<u8>,
    wb14_parent_working_state_bytes: Vec<u8>,
    litter_vapor_receipt_bytes: Vec<Vec<u8>>,
    litter_phase_receipt_bytes: Vec<Vec<u8>>,
    current_ingress_receipt_bytes: Vec<Vec<u8>>,
    soil_thermal_owner_state_sha256: String,
    soil_thermal_receipt_chain_sha256: String,
    soil_thermal_owner_envelope_bytes: Vec<u8>,
    soil_thermal_restart_identity_bytes: Vec<u8>,
    projection_sha256: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Wb14ParentWorkingStateV2Frame {
    schema: String,
    surface_configuration_sha256: String,
    surface_model_definition_sha256: String,
    liquid_arithmetic_bytes: Vec<u8>,
    persistent_beginning_owner_bytes: Vec<u8>,
    candidate_owner_bytes: Vec<u8>,
}

fn projection_schema_sha256() -> String {
    sha256(SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V3_SCHEMA.as_bytes())
}

fn identity_failure(detail: &'static str) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::Identity(detail)
}

fn schema_failure(detail: &'static str) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::Schema(detail)
}

fn parse_u128_field(
    value: &serde_json::Value,
    field: &'static str,
) -> Result<u128, DirectSurfaceLiquidError> {
    value
        .get(field)
        .and_then(serde_json::Value::as_u64)
        .map(u128::from)
        .ok_or_else(|| schema_failure("WB14 parent V2 support field"))
}

fn exact_json_bytes<T: Serialize>(
    value: &T,
    detail: &'static str,
) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
    serde_json::to_vec(value).map_err(|_| schema_failure(detail))
}

impl SurfaceLiquidCompleteOwnerProjectionV3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        configuration: &SurfaceLiquidConfigurationV2,
        identity: SurfaceLiquidCompleteOwnerProjectionIdentityV3,
        envelope: &SurfaceLiquidOwnerEnvelopeV2,
        wb14_parent_working_state_bytes: &[u8],
        litter_phase_receipts: &[LitterPhaseReceipt],
        current_ingress_receipts: &[DirectSurfaceLiquidParcelReceipt],
        soil_thermal_owner: &SoilThermalOwnerEnvelopeV2,
        soil_thermal_restart: &SoilThermalOwnerRestartV2,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let envelope_bytes =
            envelope.canonical_bytes(configuration.parent(), Some(configuration))?;
        let litter_phase_receipt_bytes = litter_phase_receipts
            .iter()
            .map(litter_phase_receipt_json)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| identity_failure("invalid accepted litter phase receipt"))?;
        let litter_vapor_receipt_bytes = litter_phase_receipts
            .iter()
            .map(|receipt| exact_json_bytes(&receipt.vapor, "litter vapor receipt serialization"))
            .collect::<Result<Vec<_>, _>>()?;
        let current_ingress_receipt_bytes = current_ingress_receipts
            .iter()
            .map(|receipt| exact_json_bytes(receipt, "current-ingress receipt serialization"))
            .collect::<Result<Vec<_>, _>>()?;
        soil_thermal_owner
            .validate()
            .map_err(|_| identity_failure("invalid soil-thermal V2 owner envelope"))?;
        let soil_thermal_owner_envelope_bytes =
            exact_json_bytes(soil_thermal_owner, "soil-thermal owner V2 serialization")?;
        let soil_thermal_restart_identity_bytes = exact_json_bytes(
            soil_thermal_restart,
            "soil-thermal restart V2 serialization",
        )?;
        let mut value = Self {
            schema_sha256: projection_schema_sha256(),
            model_definition_sha256: envelope.model_definition_sha256().into(),
            configuration_sha256: configuration.configuration_sha256().into(),
            parent_identity_sha256: envelope.parent_identity_sha256().into(),
            identity,
            envelope_sha256: envelope.envelope_sha256().into(),
            envelope_bytes,
            wb14_parent_working_state_bytes: wb14_parent_working_state_bytes.to_vec(),
            litter_vapor_receipt_bytes,
            litter_phase_receipt_bytes,
            current_ingress_receipt_bytes,
            soil_thermal_owner_state_sha256: soil_thermal_owner.state.state_sha256.to_string(),
            soil_thermal_receipt_chain_sha256: soil_thermal_owner.receipt_chain_sha256.to_string(),
            soil_thermal_owner_envelope_bytes,
            soil_thermal_restart_identity_bytes,
            projection_sha256: ZERO_SHA256.into(),
        };
        value.identity.receipt_chain_sha256 = value.recomputed_receipt_chain_sha256()?;
        value.projection_sha256 = value.recomputed_sha256()?;
        value.validate(configuration)?;
        Ok(value)
    }

    #[must_use]
    pub fn projection_sha256(&self) -> &str {
        &self.projection_sha256
    }

    #[must_use]
    pub const fn identity(&self) -> &SurfaceLiquidCompleteOwnerProjectionIdentityV3 {
        &self.identity
    }

    #[must_use]
    pub fn envelope_bytes(&self) -> &[u8] {
        &self.envelope_bytes
    }

    #[must_use]
    pub fn wb14_parent_working_state_bytes(&self) -> &[u8] {
        &self.wb14_parent_working_state_bytes
    }

    pub fn canonical_bytes(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        self.validate(configuration)?;
        self.canonical_bytes_with_digest(&self.projection_sha256)
    }

    pub fn from_canonical_bytes(
        configuration: &SurfaceLiquidConfigurationV2,
        bytes: &[u8],
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let wire: CanonicalSurfaceLiquidCompleteOwnerProjectionV3 =
            serde_json::from_slice(bytes)
                .map_err(|_| schema_failure("complete-owner projection V3 parse"))?;
        if wire.schema != SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V3_SCHEMA {
            return Err(schema_failure("complete-owner projection V3 schema tag"));
        }
        let value = Self {
            schema_sha256: wire.schema_sha256,
            model_definition_sha256: wire.model_definition_sha256,
            configuration_sha256: wire.configuration_sha256,
            parent_identity_sha256: wire.parent_identity_sha256,
            identity: SurfaceLiquidCompleteOwnerProjectionIdentityV3 {
                run_id: wire.run_id,
                transaction_id: wire.transaction_id,
                predecessor_transaction_id: wire.predecessor_transaction_id,
                parent_support_start_ns: wire.parent_support_start_ns,
                parent_support_end_ns: wire.parent_support_end_ns,
                support_start_ns: wire.support_start_ns,
                support_end_ns: wire.support_end_ns,
                beginning_surface_owner_sha256: wire.beginning_surface_owner_sha256,
                phase_adjusted_surface_owner_sha256: wire.phase_adjusted_surface_owner_sha256,
                predecessor_receipt_chain_sha256: wire.predecessor_receipt_chain_sha256,
                receipt_chain_sha256: wire.receipt_chain_sha256,
            },
            envelope_sha256: wire.envelope_sha256,
            envelope_bytes: wire.envelope_bytes,
            wb14_parent_working_state_bytes: wire.wb14_parent_working_state_bytes,
            litter_vapor_receipt_bytes: wire.litter_vapor_receipt_bytes,
            litter_phase_receipt_bytes: wire.litter_phase_receipt_bytes,
            current_ingress_receipt_bytes: wire.current_ingress_receipt_bytes,
            soil_thermal_owner_state_sha256: wire.soil_thermal_owner_state_sha256,
            soil_thermal_receipt_chain_sha256: wire.soil_thermal_receipt_chain_sha256,
            soil_thermal_owner_envelope_bytes: wire.soil_thermal_owner_envelope_bytes,
            soil_thermal_restart_identity_bytes: wire.soil_thermal_restart_identity_bytes,
            projection_sha256: wire.projection_sha256,
        };
        value.validate(configuration)?;
        if value.canonical_bytes(configuration)? != bytes {
            return Err(schema_failure(
                "noncanonical complete-owner projection V3 bytes",
            ));
        }
        Ok(value)
    }

    #[allow(clippy::too_many_lines)]
    fn validate(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        let identity = &self.identity;
        if self.schema_sha256 != projection_schema_sha256()
            || self.model_definition_sha256
                != configuration.model_definition().model_definition_sha256()
            || self.configuration_sha256 != configuration.configuration_sha256()
            || self.parent_identity_sha256 != configuration.configuration_sha256()
            || identity.run_id != configuration.parent().run_id
            || identity.transaction_id.0 == 0
            || identity.support_start_ns >= identity.support_end_ns
            || identity.parent_support_start_ns > identity.support_start_ns
            || identity.support_end_ns > identity.parent_support_end_ns
            || identity.parent_support_start_ns >= identity.parent_support_end_ns
            || !is_sha256(&identity.beginning_surface_owner_sha256)
            || !is_sha256(&identity.phase_adjusted_surface_owner_sha256)
            || !is_sha256(&identity.predecessor_receipt_chain_sha256)
            || !is_sha256(&identity.receipt_chain_sha256)
            || identity.beginning_surface_owner_sha256 == ZERO_SHA256
            || identity.phase_adjusted_surface_owner_sha256 == ZERO_SHA256
            || identity.receipt_chain_sha256 == ZERO_SHA256
            || identity.receipt_chain_sha256 != self.recomputed_receipt_chain_sha256()?
        {
            return Err(identity_failure("complete-owner projection V3 identity"));
        }

        let envelope = SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
            configuration.parent(),
            Some(configuration),
            &self.envelope_bytes,
        )?;
        if envelope.v2_state().is_none()
            || envelope.envelope_sha256() != self.envelope_sha256
            || envelope.model_definition_sha256() != self.model_definition_sha256
            || envelope.parent_identity_sha256() != self.parent_identity_sha256
        {
            return Err(identity_failure(
                "complete-owner projection V3 envelope join",
            ));
        }

        let wb14 = DirectWb14ParentWorkingStateV2::from_restart_bytes(
            configuration,
            &self.wb14_parent_working_state_bytes,
        )?;
        let wb14_wire: Wb14ParentWorkingStateV2Frame =
            serde_json::from_slice(&self.wb14_parent_working_state_bytes)
                .map_err(|_| schema_failure("WB14 parent V2 frame parse"))?;
        if wb14_wire.schema.trim().is_empty()
            || wb14_wire.surface_configuration_sha256 != self.configuration_sha256
            || wb14_wire.surface_model_definition_sha256 != self.model_definition_sha256
            || wb14.candidate_owner() != &envelope
            || wb14_wire.candidate_owner_bytes != self.envelope_bytes
        {
            return Err(identity_failure("complete-owner projection V3 WB14 join"));
        }
        self.validate_wb14_support(&wb14_wire)?;
        Self::validate_wb14_ice_carry(configuration, &wb14_wire)?;

        let litter_receipts = self.validate_litter_receipts(configuration)?;
        if litter_receipts.first().is_some_and(|first| {
            first.identity.transaction_id != identity.transaction_id
                || first.identity.support_start_ns != identity.support_start_ns
                || first.identity.support_end_ns != identity.support_end_ns
        }) {
            return Err(identity_failure(
                "complete-owner projection V3 litter support",
            ));
        }
        self.validate_current_ingress_receipts()?;
        self.validate_soil_thermal_join()?;

        if !is_sha256(&self.projection_sha256)
            || self.projection_sha256 == ZERO_SHA256
            || self.projection_sha256 != self.recomputed_sha256()?
        {
            return Err(identity_failure("complete-owner projection V3 digest"));
        }
        Ok(())
    }

    fn validate_wb14_support(
        &self,
        wire: &Wb14ParentWorkingStateV2Frame,
    ) -> Result<(), DirectSurfaceLiquidError> {
        let liquid: serde_json::Value = serde_json::from_slice(&wire.liquid_arithmetic_bytes)
            .map_err(|_| schema_failure("WB14 liquid-arithmetic frame parse"))?;
        let parent_start = parse_u128_field(&liquid, "parent_support_start_ns")?;
        let parent_end = parse_u128_field(&liquid, "parent_support_end_ns")?;
        let accepted_until = parse_u128_field(&liquid, "accepted_until_ns")?;
        if parent_start != self.identity.parent_support_start_ns
            || parent_end != self.identity.parent_support_end_ns
            || accepted_until != self.identity.support_end_ns
        {
            return Err(identity_failure(
                "complete-owner projection V3 WB14 support",
            ));
        }
        Ok(())
    }

    fn validate_wb14_ice_carry(
        configuration: &SurfaceLiquidConfigurationV2,
        wire: &Wb14ParentWorkingStateV2Frame,
    ) -> Result<(), DirectSurfaceLiquidError> {
        let beginning = SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
            configuration.parent(),
            Some(configuration),
            &wire.persistent_beginning_owner_bytes,
        )?;
        let ending = SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
            configuration.parent(),
            Some(configuration),
            &wire.candidate_owner_bytes,
        )?;
        let beginning = beginning
            .v2_state()
            .ok_or_else(|| identity_failure("WB14 parent beginning cross-version owner"))?;
        let ending = ending
            .v2_state()
            .ok_or_else(|| identity_failure("WB14 parent candidate cross-version owner"))?;
        if beginning.records().len() != ending.records().len()
            || beginning
                .records()
                .iter()
                .zip(ending.records())
                .any(|(left, right)| {
                    left.key != right.key
                        || left.litter_ice_kg_m2_tile.to_bits()
                            != right.litter_ice_kg_m2_tile.to_bits()
                })
        {
            return Err(DirectSurfaceLiquidError::Closure(
                "WB14 parent V2 donated or substituted litter ice",
            ));
        }
        Ok(())
    }

    fn validate_litter_receipts(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<Vec<LitterPhaseReceipt>, DirectSurfaceLiquidError> {
        let expected: Vec<_> = configuration
            .parent()
            .records
            .iter()
            .filter(|record| record.key.surface_class == SurfaceClass::ForestLitter)
            .collect();
        if expected.is_empty()
            || expected.len() != self.litter_phase_receipt_bytes.len()
            || expected.len() != self.litter_vapor_receipt_bytes.len()
        {
            return Err(identity_failure("litter receipt omission or cardinality"));
        }
        let mut receipt_digests = BTreeSet::new();
        let mut lse_configuration_sha256 = None;
        let receipts = self
            .litter_phase_receipt_bytes
            .iter()
            .map(|bytes| {
                litter_phase_receipt_from_json(bytes)
                    .map_err(|_| identity_failure("invalid litter phase receipt replay"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        for ((configured, receipt), vapor_bytes) in expected
            .into_iter()
            .zip(&receipts)
            .zip(&self.litter_vapor_receipt_bytes)
        {
            let receipt_identity = &receipt.identity;
            if receipt_identity.transaction_id != self.identity.transaction_id
                || receipt_identity.ofe_id != configured.key.ofe_id
                || receipt_identity.tile_id != configured.key.tile_id
                || receipt_identity.surface_owner_id != configuration.parent().owner_id
                || receipt_identity.beginning_surface_owner_sha256.as_str()
                    != self.identity.beginning_surface_owner_sha256
                || receipt_identity.candidate_surface_owner_sha256.as_str()
                    != self.identity.phase_adjusted_surface_owner_sha256
                || receipt_identity.support_start_ns != self.identity.support_start_ns
                || receipt_identity.support_end_ns != self.identity.support_end_ns
                || exact_json_bytes(&receipt.vapor, "litter vapor receipt replay")? != *vapor_bytes
                || !receipt_digests.insert(receipt.receipt_sha256.clone())
            {
                return Err(identity_failure("ordered litter receipt identity"));
            }
            match &lse_configuration_sha256 {
                Some(expected_digest)
                    if expected_digest != &receipt_identity.lse_configuration_sha256 =>
                {
                    return Err(identity_failure("mixed LSE configuration receipt identity"));
                }
                None => {
                    lse_configuration_sha256 =
                        Some(receipt_identity.lse_configuration_sha256.clone());
                }
                Some(_) => {}
            }
        }
        Ok(receipts)
    }

    fn validate_current_ingress_receipts(&self) -> Result<(), DirectSurfaceLiquidError> {
        let duration_s = std::time::Duration::from_nanos(
            u64::try_from(self.identity.support_end_ns - self.identity.support_start_ns)
                .map_err(|_| identity_failure("projection support exceeds u64 nanoseconds"))?,
        )
        .as_secs_f64();
        let mut exact_receipts = BTreeSet::new();
        for bytes in &self.current_ingress_receipt_bytes {
            let receipt: DirectSurfaceLiquidParcelReceipt = serde_json::from_slice(bytes)
                .map_err(|_| schema_failure("current-ingress receipt parse"))?;
            if exact_json_bytes(&receipt, "current-ingress receipt replay")? != *bytes
                || receipt.transaction_id != self.identity.transaction_id
                || receipt.origin_store_key.run_id != self.identity.run_id
                || receipt.recipient_store_key.run_id != self.identity.run_id
                || !receipt.start_s.is_finite()
                || !receipt.end_s.is_finite()
                || receipt.start_s < 0.0
                || receipt.end_s <= receipt.start_s
                || receipt.end_s > duration_s
                || !receipt.mass_kg_m2_basis_ofe_ground.is_finite()
                || receipt.mass_kg_m2_basis_ofe_ground < 0.0
                || !receipt.temperature_k.is_finite()
                || !receipt.enthalpy_j_m2_basis_ofe_ground.is_finite()
                || !exact_receipts.insert(bytes.clone())
            {
                return Err(identity_failure(
                    "current-ingress receipt identity or replay",
                ));
            }
        }
        Ok(())
    }

    fn validate_soil_thermal_join(&self) -> Result<(), DirectSurfaceLiquidError> {
        let owner: SoilThermalOwnerEnvelopeV2 =
            serde_json::from_slice(&self.soil_thermal_owner_envelope_bytes)
                .map_err(|_| schema_failure("soil-thermal owner V2 parse"))?;
        owner
            .validate()
            .map_err(|_| identity_failure("soil-thermal owner V2 validation"))?;
        if exact_json_bytes(&owner, "soil-thermal owner V2 replay")?
            != self.soil_thermal_owner_envelope_bytes
            || owner.run_id != self.identity.run_id.to_string()
            || owner.transaction_id != self.identity.transaction_id
            || owner.expected_predecessor_transaction_id != self.identity.predecessor_transaction_id
            || owner.support_start_ns != self.identity.support_start_ns
            || owner.support_end_ns != self.identity.support_end_ns
            || owner.state.state_sha256.as_str() != self.soil_thermal_owner_state_sha256
            || owner.receipt_chain_sha256.as_str() != self.soil_thermal_receipt_chain_sha256
        {
            return Err(identity_failure("soil-thermal owner V2 projection join"));
        }
        let restart: SoilThermalOwnerRestartV2 =
            serde_json::from_slice(&self.soil_thermal_restart_identity_bytes)
                .map_err(|_| schema_failure("soil-thermal restart V2 parse"))?;
        if exact_json_bytes(&restart, "soil-thermal restart V2 replay")?
            != self.soil_thermal_restart_identity_bytes
            || restart.owner_tag != owner.owner_tag
            || restart.schema_sha256 != owner.schema_sha256
            || restart.exact_carry_definition_sha256 != owner.exact_carry_definition_sha256
            || restart.parent_v1_state_sha256 != owner.parent_v1_state_sha256
            || restart.owner_state_sha256 != owner.state.state_sha256
            || restart.last_accepted_transaction_id != owner.state.last_accepted_transaction_id
            || restart.receipt_chain_sha256 != owner.receipt_chain_sha256
            || restart.restart_sha256.as_str() == ZERO_SHA256
        {
            return Err(identity_failure("soil-thermal restart V2 identity join"));
        }
        Ok(())
    }

    fn recomputed_sha256(&self) -> Result<String, DirectSurfaceLiquidError> {
        Ok(sha256(&self.canonical_bytes_with_digest(ZERO_SHA256)?))
    }

    fn recomputed_receipt_chain_sha256(&self) -> Result<String, DirectSurfaceLiquidError> {
        let mut bytes = Vec::new();
        append_frame(
            &mut bytes,
            b"OPENWEPP_SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V3_RECEIPT_CHAIN",
        )?;
        append_frame(
            &mut bytes,
            self.identity.predecessor_receipt_chain_sha256.as_bytes(),
        )?;
        for (vapor, phase) in self
            .litter_vapor_receipt_bytes
            .iter()
            .zip(&self.litter_phase_receipt_bytes)
        {
            append_frame(&mut bytes, vapor)?;
            append_frame(&mut bytes, phase)?;
        }
        for ingress in &self.current_ingress_receipt_bytes {
            append_frame(&mut bytes, ingress)?;
        }
        append_frame(&mut bytes, &self.wb14_parent_working_state_bytes)?;
        append_frame(&mut bytes, &self.envelope_bytes)?;
        append_frame(&mut bytes, &self.soil_thermal_owner_envelope_bytes)?;
        append_frame(&mut bytes, &self.soil_thermal_restart_identity_bytes)?;
        Ok(sha256(&bytes))
    }

    fn canonical_bytes_with_digest(
        &self,
        digest: &str,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        exact_json_bytes(
            &CanonicalSurfaceLiquidCompleteOwnerProjectionV3 {
                schema: SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V3_SCHEMA.into(),
                schema_sha256: self.schema_sha256.clone(),
                model_definition_sha256: self.model_definition_sha256.clone(),
                configuration_sha256: self.configuration_sha256.clone(),
                parent_identity_sha256: self.parent_identity_sha256.clone(),
                run_id: self.identity.run_id,
                transaction_id: self.identity.transaction_id,
                predecessor_transaction_id: self.identity.predecessor_transaction_id,
                parent_support_start_ns: self.identity.parent_support_start_ns,
                parent_support_end_ns: self.identity.parent_support_end_ns,
                support_start_ns: self.identity.support_start_ns,
                support_end_ns: self.identity.support_end_ns,
                beginning_surface_owner_sha256: self
                    .identity
                    .beginning_surface_owner_sha256
                    .clone(),
                phase_adjusted_surface_owner_sha256: self
                    .identity
                    .phase_adjusted_surface_owner_sha256
                    .clone(),
                predecessor_receipt_chain_sha256: self
                    .identity
                    .predecessor_receipt_chain_sha256
                    .clone(),
                receipt_chain_sha256: self.identity.receipt_chain_sha256.clone(),
                envelope_sha256: self.envelope_sha256.clone(),
                envelope_bytes: self.envelope_bytes.clone(),
                wb14_parent_working_state_bytes: self.wb14_parent_working_state_bytes.clone(),
                litter_vapor_receipt_bytes: self.litter_vapor_receipt_bytes.clone(),
                litter_phase_receipt_bytes: self.litter_phase_receipt_bytes.clone(),
                current_ingress_receipt_bytes: self.current_ingress_receipt_bytes.clone(),
                soil_thermal_owner_state_sha256: self.soil_thermal_owner_state_sha256.clone(),
                soil_thermal_receipt_chain_sha256: self.soil_thermal_receipt_chain_sha256.clone(),
                soil_thermal_owner_envelope_bytes: self.soil_thermal_owner_envelope_bytes.clone(),
                soil_thermal_restart_identity_bytes: self
                    .soil_thermal_restart_identity_bytes
                    .clone(),
                projection_sha256: digest.into(),
            },
            "complete-owner projection V3 serialization",
        )
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn append_frame(bytes: &mut Vec<u8>, value: &[u8]) -> Result<(), DirectSurfaceLiquidError> {
    let length = u64::try_from(value.len())
        .map_err(|_| schema_failure("complete-owner projection V3 frame length"))?;
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(value);
    Ok(())
}

#[cfg(test)]
#[path = "v3_projection_tests.rs"]
mod tests;
