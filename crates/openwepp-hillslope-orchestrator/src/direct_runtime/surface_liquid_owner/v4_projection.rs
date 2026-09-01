//! Canonical V16 complete-owner projection.
//!
//! Projection V3 bytes are nested unchanged and joined to the authoritative
//! exact-surface owner/receipt. No V3 serialization is reinterpreted.

#![allow(clippy::missing_errors_doc)]

use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{LandSurfaceEnergyV3State, V3_MODEL_DEFINITION_SHA256};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::land_surface_energy_shadow::v3_execution::reconstruct_surface_energy_operands_v1;
use crate::{
    SurfaceLiquidCompleteOwnerProjectionV3, SurfaceLiquidConfigurationV2,
    SurfaceLiquidOwnerEnvelopeV2,
};

use super::surface_liquid_owner_v3_exact_enthalpy::{
    LseSurfaceEnthalpyEnergyCreditReceiptV1, LseSurfaceEnthalpyErrorV1,
    LseSurfaceEnthalpyOwnerEnvelopeV1,
};

pub const SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V4_SCHEMA: &str =
    "OPENWEPP_SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V4";
pub const SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V4_SCHEMA_SHA256: &str =
    "38f38916a2172e018b9c758410c69b694ef5c8cd7da2abdf8c4d8bb8c69a775b";
const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceLiquidCompleteOwnerProjectionIdentityV4 {
    pub transaction_id: TransactionId,
    pub predecessor_transaction_id: Option<TransactionId>,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub projection_v3_sha256: String,
    pub beginning_lse_v3_state_sha256: String,
    pub beginning_exact_surface_owner_state_sha256: String,
    pub exact_surface_owner_state_sha256: String,
    pub exact_surface_receipt_sha256: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceLiquidCompleteOwnerProjectionV4 {
    pub schema: String,
    pub schema_sha256: String,
    pub identity: SurfaceLiquidCompleteOwnerProjectionIdentityV4,
    projection_v3_bytes: Vec<u8>,
    beginning_lse_v3_state_bytes: Vec<u8>,
    beginning_exact_surface_owner_bytes: Vec<u8>,
    exact_surface_owner_bytes: Vec<u8>,
    exact_surface_receipt_bytes: Vec<u8>,
    projection_sha256: String,
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn parse_canonical_exact_surface_receipt(
    bytes: &[u8],
) -> Result<LseSurfaceEnthalpyEnergyCreditReceiptV1, LseSurfaceEnthalpyErrorV1> {
    let receipt: LseSurfaceEnthalpyEnergyCreditReceiptV1 = serde_json::from_slice(bytes)
        .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?;
    let canonical = serde_json::to_vec(&receipt)
        .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?;
    if canonical != bytes {
        return Err(LseSurfaceEnthalpyErrorV1::Serialization(
            "noncanonical nested exact-surface receipt bytes".to_owned(),
        ));
    }
    Ok(receipt)
}

fn parse_canonical_beginning_lse_v3(
    bytes: &[u8],
) -> Result<LandSurfaceEnergyV3State, LseSurfaceEnthalpyErrorV1> {
    let state: LandSurfaceEnergyV3State = serde_json::from_slice(bytes)
        .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?;
    let canonical = state
        .to_json()
        .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("nested beginning LSE V3 bytes"))?;
    let recomputed = state
        .canonical_sha256()
        .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("nested beginning LSE V3 digest"))?;
    if canonical != bytes
        || recomputed != state.0.state_sha256
        || state.0.model_definition_sha256.as_str() != V3_MODEL_DEFINITION_SHA256
    {
        return Err(LseSurfaceEnthalpyErrorV1::Identity(
            "nested beginning LSE V3 canonical identity",
        ));
    }
    Ok(state)
}

impl SurfaceLiquidCompleteOwnerProjectionV4 {
    pub fn new(
        configuration: &SurfaceLiquidConfigurationV2,
        projection_v3: &SurfaceLiquidCompleteOwnerProjectionV3,
        beginning_lse_v3: &LandSurfaceEnergyV3State,
        beginning_exact_surface_owner: &LseSurfaceEnthalpyOwnerEnvelopeV1,
        exact_surface_owner: &LseSurfaceEnthalpyOwnerEnvelopeV1,
        exact_surface_receipt: &LseSurfaceEnthalpyEnergyCreditReceiptV1,
    ) -> Result<Self, LseSurfaceEnthalpyErrorV1> {
        beginning_exact_surface_owner.validate()?;
        exact_surface_owner.validate()?;
        let v3_identity = projection_v3.identity();
        if v3_identity.transaction_id != exact_surface_receipt.transaction_id
            || v3_identity.predecessor_transaction_id
                != exact_surface_receipt.predecessor_transaction_id
            || v3_identity.support_start_ns != exact_surface_receipt.support_start_ns
            || v3_identity.support_end_ns != exact_surface_receipt.support_end_ns
            || exact_surface_owner.state_sha256 != exact_surface_receipt.ending_owner_state_sha256
            || exact_surface_owner.receipt_chain_sha256 != exact_surface_receipt.receipt_sha256
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "projection V3/exact owner/receipt join",
            ));
        }
        let projection_v3_bytes = projection_v3
            .canonical_bytes(configuration)
            .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("projection V3 bytes"))?;
        let beginning_lse_v3_state_bytes = beginning_lse_v3
            .to_json()
            .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("beginning LSE V3 bytes"))?;
        parse_canonical_beginning_lse_v3(&beginning_lse_v3_state_bytes)?;
        let beginning_exact_surface_owner_bytes =
            beginning_exact_surface_owner.canonical_bytes()?;
        let exact_surface_owner_bytes = exact_surface_owner.canonical_bytes()?;
        let exact_surface_receipt_bytes = serde_json::to_vec(exact_surface_receipt)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?;
        let mut value = Self {
            schema: SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V4_SCHEMA.to_owned(),
            schema_sha256: SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V4_SCHEMA_SHA256.to_owned(),
            identity: SurfaceLiquidCompleteOwnerProjectionIdentityV4 {
                transaction_id: exact_surface_receipt.transaction_id,
                predecessor_transaction_id: exact_surface_receipt.predecessor_transaction_id,
                support_start_ns: exact_surface_receipt.support_start_ns,
                support_end_ns: exact_surface_receipt.support_end_ns,
                projection_v3_sha256: projection_v3.projection_sha256().to_owned(),
                beginning_lse_v3_state_sha256: beginning_lse_v3.0.state_sha256.to_string(),
                beginning_exact_surface_owner_state_sha256: beginning_exact_surface_owner
                    .state_sha256
                    .to_string(),
                exact_surface_owner_state_sha256: exact_surface_owner.state_sha256.to_string(),
                exact_surface_receipt_sha256: exact_surface_receipt.receipt_sha256.to_string(),
            },
            projection_v3_bytes,
            beginning_lse_v3_state_bytes,
            beginning_exact_surface_owner_bytes,
            exact_surface_owner_bytes,
            exact_surface_receipt_bytes,
            projection_sha256: ZERO_SHA256.to_owned(),
        };
        value.projection_sha256 = value.recomputed_sha256()?;
        value.validate(configuration, beginning_lse_v3.0.state_sha256.as_str())?;
        Ok(value)
    }

    fn recomputed_sha256(&self) -> Result<String, LseSurfaceEnthalpyErrorV1> {
        let mut value = self.clone();
        ZERO_SHA256.clone_into(&mut value.projection_sha256);
        let bytes = serde_json::to_vec(&value)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?;
        Ok(sha256(&bytes))
    }

    #[allow(clippy::too_many_lines)]
    pub fn validate(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        expected_beginning_lse_v3_state_sha256: &str,
    ) -> Result<(), LseSurfaceEnthalpyErrorV1> {
        if self.schema != SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V4_SCHEMA
            || self.schema_sha256 != SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V4_SCHEMA_SHA256
            || self.identity.support_start_ns >= self.identity.support_end_ns
            || self.identity.beginning_lse_v3_state_sha256 != expected_beginning_lse_v3_state_sha256
            || self.projection_sha256 != self.recomputed_sha256()?
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "projection V4 schema, support, or digest",
            ));
        }
        let projection_v3 = SurfaceLiquidCompleteOwnerProjectionV3::from_canonical_bytes(
            configuration,
            &self.projection_v3_bytes,
        )
        .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("nested projection V3"))?;
        let beginning_lse_v3 =
            parse_canonical_beginning_lse_v3(&self.beginning_lse_v3_state_bytes)?;
        let beginning_owner = LseSurfaceEnthalpyOwnerEnvelopeV1::from_canonical_bytes(
            &self.beginning_exact_surface_owner_bytes,
        )?;
        let owner = LseSurfaceEnthalpyOwnerEnvelopeV1::from_canonical_bytes(
            &self.exact_surface_owner_bytes,
        )?;
        let receipt = parse_canonical_exact_surface_receipt(&self.exact_surface_receipt_bytes)?;
        let v3_identity = projection_v3.identity();
        let litter_receipts = projection_v3
            .replay_litter_phase_receipts(configuration)
            .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("nested litter receipt replay"))?;
        let ingress_receipts = projection_v3
            .replay_current_ingress_receipts()
            .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("nested ingress receipt replay"))?;
        let expected_operands = reconstruct_surface_energy_operands_v1(
            configuration,
            self.identity.transaction_id,
            self.identity.predecessor_transaction_id,
            self.identity.support_start_ns,
            self.identity.support_end_ns,
            &litter_receipts,
            &ingress_receipts,
        )
        .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("nested exact operand replay"))?;
        let ending_surface = SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
            configuration.parent(),
            Some(configuration),
            projection_v3.envelope_bytes(),
        )
        .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("nested ending surface owner"))?;
        let ending_surface_records = ending_surface
            .v2_state()
            .ok_or(LseSurfaceEnthalpyErrorV1::Identity(
                "nested ending surface owner is not V2",
            ))?
            .records();
        if owner.records.len() != ending_surface_records.len()
            || owner.records.iter().any(|exact| {
                ending_surface_records
                    .iter()
                    .find(|record| record.key == exact.surface_key)
                    .is_none_or(|record| {
                        record.surface_enthalpy_j_m2_tile.to_bits()
                            != exact.enthalpy_hi_j_m2_tile.to_bits()
                    })
            })
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "exact high mirrors do not match nested V3 ending surface owner",
            ));
        }
        if beginning_owner.records.len() != beginning_lse_v3.0.tiles.len()
            || beginning_owner.records.iter().any(|exact| {
                beginning_lse_v3
                    .0
                    .tiles
                    .iter()
                    .find(|tile| {
                        tile.ofe_id == exact.surface_key.ofe_id
                            && tile.tile_id == exact.surface_key.tile_id
                    })
                    .is_none_or(|tile| {
                        tile.surface_enthalpy_j_m2_tile_ground.to_bits()
                            != exact.enthalpy_hi_j_m2_tile.to_bits()
                    })
            })
            || beginning_owner.records.iter().any(|record| {
                record.last_accepted_transaction_id
                    != beginning_lse_v3.0.last_accepted_transaction_id
            })
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "exact high mirrors do not match nested beginning LSE V3 owner",
            ));
        }
        if projection_v3.projection_sha256() != self.identity.projection_v3_sha256
            || v3_identity.transaction_id != self.identity.transaction_id
            || v3_identity.predecessor_transaction_id != self.identity.predecessor_transaction_id
            || v3_identity.support_start_ns != self.identity.support_start_ns
            || v3_identity.support_end_ns != self.identity.support_end_ns
            || beginning_lse_v3.0.state_sha256.as_str()
                != self.identity.beginning_lse_v3_state_sha256
            || beginning_owner.frozen_lse_v3_state_sha256 != beginning_lse_v3.0.state_sha256
            || beginning_owner.state_sha256.as_str()
                != self.identity.beginning_exact_surface_owner_state_sha256
            || beginning_owner.frozen_surface_owner_v2_sha256.as_str()
                != v3_identity.beginning_surface_owner_sha256
            || owner.state_sha256.as_str() != self.identity.exact_surface_owner_state_sha256
            || owner.frozen_surface_owner_v2_sha256.as_str() != projection_v3.envelope_sha256()
            || receipt.receipt_sha256.as_str() != self.identity.exact_surface_receipt_sha256
            || owner.receipt_chain_sha256 != receipt.receipt_sha256
            || receipt.ending_owner_state_sha256 != owner.state_sha256
            || receipt.transaction_id != self.identity.transaction_id
            || receipt.predecessor_transaction_id != self.identity.predecessor_transaction_id
            || receipt.support_start_ns != self.identity.support_start_ns
            || receipt.support_end_ns != self.identity.support_end_ns
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "nested projection/owner/receipt replay",
            ));
        }
        receipt.validate_independent(&beginning_owner, &owner, &expected_operands)?;
        Ok(())
    }

    pub fn canonical_bytes(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<Vec<u8>, LseSurfaceEnthalpyErrorV1> {
        self.validate(configuration, &self.identity.beginning_lse_v3_state_sha256)?;
        serde_json::to_vec(self)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))
    }

    pub fn from_canonical_bytes(
        configuration: &SurfaceLiquidConfigurationV2,
        bytes: &[u8],
        expected_beginning_lse_v3_state_sha256: &str,
    ) -> Result<Self, LseSurfaceEnthalpyErrorV1> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?;
        value.validate(configuration, expected_beginning_lse_v3_state_sha256)?;
        if serde_json::to_vec(&value)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?
            != bytes
        {
            return Err(LseSurfaceEnthalpyErrorV1::Serialization(
                "noncanonical projection V4 bytes".to_owned(),
            ));
        }
        Ok(value)
    }

    pub fn projection_v3(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<SurfaceLiquidCompleteOwnerProjectionV3, LseSurfaceEnthalpyErrorV1> {
        SurfaceLiquidCompleteOwnerProjectionV3::from_canonical_bytes(
            configuration,
            &self.projection_v3_bytes,
        )
        .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("nested projection V3"))
    }

    pub fn exact_surface_owner(
        &self,
    ) -> Result<LseSurfaceEnthalpyOwnerEnvelopeV1, LseSurfaceEnthalpyErrorV1> {
        LseSurfaceEnthalpyOwnerEnvelopeV1::from_canonical_bytes(&self.exact_surface_owner_bytes)
    }

    pub fn beginning_exact_surface_owner(
        &self,
    ) -> Result<LseSurfaceEnthalpyOwnerEnvelopeV1, LseSurfaceEnthalpyErrorV1> {
        LseSurfaceEnthalpyOwnerEnvelopeV1::from_canonical_bytes(
            &self.beginning_exact_surface_owner_bytes,
        )
    }

    pub fn exact_surface_receipt(
        &self,
    ) -> Result<LseSurfaceEnthalpyEnergyCreditReceiptV1, LseSurfaceEnthalpyErrorV1> {
        parse_canonical_exact_surface_receipt(&self.exact_surface_receipt_bytes)
    }

    #[must_use]
    pub fn projection_sha256(&self) -> &str {
        &self.projection_sha256
    }
}

#[cfg(all(test, feature = "restart-authority-evidence"))]
mod tests {
    use crate::land_surface_energy_shadow::accepted_negative_zero_v4_evidence_v1;

    use super::{SurfaceLiquidCompleteOwnerProjectionV4, ZERO_SHA256};

    fn reseal_with_receipt_bytes(
        projection: &SurfaceLiquidCompleteOwnerProjectionV4,
        receipt_bytes: Vec<u8>,
    ) -> SurfaceLiquidCompleteOwnerProjectionV4 {
        let mut poisoned = projection.clone();
        poisoned.exact_surface_receipt_bytes = receipt_bytes;
        ZERO_SHA256.clone_into(&mut poisoned.projection_sha256);
        poisoned.projection_sha256 = poisoned
            .recomputed_sha256()
            .expect("reseal outer projection around nested poison");
        poisoned
    }

    fn reseal_projection(
        mut poisoned: SurfaceLiquidCompleteOwnerProjectionV4,
    ) -> SurfaceLiquidCompleteOwnerProjectionV4 {
        ZERO_SHA256.clone_into(&mut poisoned.projection_sha256);
        poisoned.projection_sha256 = poisoned
            .recomputed_sha256()
            .expect("reseal outer projection around nested poison");
        poisoned
    }

    fn swap_first_two_object_members(bytes: &[u8]) -> Vec<u8> {
        let first_comma = bytes
            .iter()
            .position(|byte| *byte == b',')
            .expect("first receipt member delimiter");
        let second_comma = bytes[first_comma + 1..]
            .iter()
            .position(|byte| *byte == b',')
            .map(|offset| first_comma + 1 + offset)
            .expect("second receipt member delimiter");
        let mut reordered = Vec::with_capacity(bytes.len());
        reordered.push(b'{');
        reordered.extend_from_slice(&bytes[first_comma + 1..second_comma]);
        reordered.push(b',');
        reordered.extend_from_slice(&bytes[1..first_comma]);
        reordered.extend_from_slice(&bytes[second_comma..]);
        reordered
    }

    fn duplicate_first_object_member(bytes: &[u8]) -> Vec<u8> {
        let first_comma = bytes
            .iter()
            .position(|byte| *byte == b',')
            .expect("first receipt member delimiter");
        let mut duplicated = Vec::with_capacity(bytes.len() + first_comma);
        duplicated.extend_from_slice(&bytes[..=first_comma]);
        duplicated.extend_from_slice(&bytes[1..=first_comma]);
        duplicated.extend_from_slice(&bytes[first_comma + 1..]);
        duplicated
    }

    fn assert_nested_poison_refused_without_mutation(
        projection: &SurfaceLiquidCompleteOwnerProjectionV4,
        receipt_bytes: Vec<u8>,
        configuration: &crate::SurfaceLiquidConfigurationV2,
        expected_beginning_lse_v3_state_sha256: &str,
    ) {
        let poisoned = reseal_with_receipt_bytes(projection, receipt_bytes);
        let before = poisoned.clone();
        poisoned
            .validate(configuration, expected_beginning_lse_v3_state_sha256)
            .expect_err("outer-resealed noncanonical nested receipt must fail closed");
        poisoned
            .exact_surface_receipt()
            .expect_err("receipt accessor must not expose noncanonical nested bytes");
        assert_eq!(
            poisoned, before,
            "rejected validation must not mutate state"
        );
    }

    #[test]
    fn projection_v4_refuses_outer_resealed_noncanonical_nested_receipts_atomically() {
        let evidence = accepted_negative_zero_v4_evidence_v1();
        let canonical = serde_json::to_vec(&evidence.exact_surface_receipt)
            .expect("canonical exact-surface receipt bytes");
        assert_eq!(
            evidence
                .projection_v4
                .exact_surface_receipt()
                .expect("canonical nested receipt"),
            evidence.exact_surface_receipt
        );

        let mut whitespace = canonical.clone();
        whitespace.push(b' ');
        assert_nested_poison_refused_without_mutation(
            &evidence.projection_v4,
            whitespace,
            &evidence.surface_configuration,
            evidence.beginning_lse_state.0.state_sha256.as_str(),
        );

        let reordered = swap_first_two_object_members(&canonical);
        assert_ne!(reordered, canonical);
        let parsed_reordered: super::LseSurfaceEnthalpyEnergyCreditReceiptV1 =
            serde_json::from_slice(&reordered).expect("reordered receipt remains valid JSON");
        assert_eq!(parsed_reordered, evidence.exact_surface_receipt);
        assert_nested_poison_refused_without_mutation(
            &evidence.projection_v4,
            reordered,
            &evidence.surface_configuration,
            evidence.beginning_lse_state.0.state_sha256.as_str(),
        );

        let duplicated = duplicate_first_object_member(&canonical);
        assert_nested_poison_refused_without_mutation(
            &evidence.projection_v4,
            duplicated,
            &evidence.surface_configuration,
            evidence.beginning_lse_state.0.state_sha256.as_str(),
        );
    }

    #[test]
    fn projection_v4_refuses_outer_resealed_beginning_lse_parent_substitution() {
        let evidence = accepted_negative_zero_v4_evidence_v1();
        let mut substituted: openwepp_land_surface_energy::LandSurfaceEnergyV3State =
            serde_json::from_slice(&evidence.projection_v4.beginning_lse_v3_state_bytes)
                .expect("nested beginning LSE V3");
        substituted.0.tiles[0].surface_temperature_warm_start_k += 0.25;
        substituted.0.state_sha256 = substituted
            .canonical_sha256()
            .expect("reseal substituted beginning LSE V3");

        let mut poisoned = evidence.projection_v4.clone();
        poisoned.beginning_lse_v3_state_bytes =
            substituted.to_json().expect("substituted LSE bytes");
        poisoned.identity.beginning_lse_v3_state_sha256 = substituted.0.state_sha256.to_string();
        let poisoned = reseal_projection(poisoned);
        let before = poisoned.clone();
        poisoned
            .validate(
                &evidence.surface_configuration,
                evidence.beginning_lse_state.0.state_sha256.as_str(),
            )
            .expect_err("outer-resealed beginning LSE parent substitution must fail closed");
        assert_eq!(
            poisoned, before,
            "rejected validation must not mutate state"
        );
    }
}
