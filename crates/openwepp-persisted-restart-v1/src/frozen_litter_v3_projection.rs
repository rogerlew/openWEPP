//! Validated projection/publication authority for frozen-litter V3 restart.

use openwepp_hillslope_orchestrator::{
    SurfaceLiquidCompleteOwnerProjectionV3, SurfaceLiquidConfigurationV2,
};
use openwepp_kernel_contract::TransactionId;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Sha256Hex, canonical_sha256};

pub const FROZEN_LITTER_PUBLICATION_AUTHORITY_V3_SCHEMA: &str =
    "OPENWEPP_FROZEN_LITTER_PUBLICATION_AUTHORITY_V3";
const MINIMUM_SUPPORT_NS: u128 = 60_000_000_000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenLitterPublicationAuthorityV3 {
    pub schema: String,
    pub version: u16,
    pub run_id: u64,
    pub transaction_id: TransactionId,
    pub predecessor_transaction_id: Option<TransactionId>,
    pub parent_support_start_ns: u128,
    pub parent_support_end_ns: u128,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub predecessor_receipt_chain_sha256: Sha256Hex,
    pub receipt_chain_sha256: Sha256Hex,
    pub complete_projection_sha256: Sha256Hex,
    pub publication_authority_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct PublicationAuthorityDigestBody<'a> {
    schema: &'a str,
    version: u16,
    run_id: u64,
    transaction_id: TransactionId,
    predecessor_transaction_id: Option<TransactionId>,
    parent_support_start_ns: u128,
    parent_support_end_ns: u128,
    support_start_ns: u128,
    support_end_ns: u128,
    predecessor_receipt_chain_sha256: &'a Sha256Hex,
    receipt_chain_sha256: &'a Sha256Hex,
    complete_projection_sha256: &'a Sha256Hex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedFrozenLitterProjectionV3 {
    pub authority: FrozenLitterPublicationAuthorityV3,
    pub ending_surface_owner_bytes: Vec<u8>,
    pub wb14_parent_working_state_bytes: Vec<u8>,
    pub soil_thermal_owner_envelope_bytes: Vec<u8>,
    pub soil_thermal_restart_identity_bytes: Vec<u8>,
}

/// Native validation boundary. Runtime integrations use
/// [`NativeFrozenLitterProjectionAuthorityV3`]; the trait exists so callers
/// can bind their independently retained native authority without duplicating
/// the complete-projection digest formula.
pub trait FrozenLitterProjectionSealAuthorityV3 {
    fn validate_projection(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        bytes: &[u8],
        authority: &FrozenLitterPublicationAuthorityV3,
    ) -> Result<ValidatedFrozenLitterProjectionV3, FrozenLitterProjectionRestartError>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NativeFrozenLitterProjectionAuthorityV3;

impl FrozenLitterProjectionSealAuthorityV3 for NativeFrozenLitterProjectionAuthorityV3 {
    fn validate_projection(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        bytes: &[u8],
        authority: &FrozenLitterPublicationAuthorityV3,
    ) -> Result<ValidatedFrozenLitterProjectionV3, FrozenLitterProjectionRestartError> {
        validate_frozen_litter_projection_v3(configuration, bytes, authority)
    }
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum FrozenLitterProjectionRestartError {
    #[error("projection_schema_or_canonical_bytes")]
    Projection,
    #[error("publication_authority")]
    Authority,
    #[error("projection_identity_join")]
    Identity,
}

impl FrozenLitterPublicationAuthorityV3 {
    pub fn from_projection(
        projection: &SurfaceLiquidCompleteOwnerProjectionV3,
    ) -> Result<Self, FrozenLitterProjectionRestartError> {
        let identity = projection.identity();
        let mut value = Self {
            schema: FROZEN_LITTER_PUBLICATION_AUTHORITY_V3_SCHEMA.to_owned(),
            version: 3,
            run_id: identity.run_id,
            transaction_id: identity.transaction_id,
            predecessor_transaction_id: identity.predecessor_transaction_id,
            parent_support_start_ns: identity.parent_support_start_ns,
            parent_support_end_ns: identity.parent_support_end_ns,
            support_start_ns: identity.support_start_ns,
            support_end_ns: identity.support_end_ns,
            predecessor_receipt_chain_sha256: wire_digest(
                &identity.predecessor_receipt_chain_sha256,
            )?,
            receipt_chain_sha256: wire_digest(&identity.receipt_chain_sha256)?,
            complete_projection_sha256: wire_digest(projection.projection_sha256())?,
            publication_authority_sha256: zero_digest()?,
        };
        value.publication_authority_sha256 = value.compute_digest()?;
        value.validate()?;
        Ok(value)
    }

    pub fn compute_digest(&self) -> Result<Sha256Hex, FrozenLitterProjectionRestartError> {
        wire_digest(
            &canonical_sha256(&PublicationAuthorityDigestBody {
                schema: &self.schema,
                version: self.version,
                run_id: self.run_id,
                transaction_id: self.transaction_id,
                predecessor_transaction_id: self.predecessor_transaction_id,
                parent_support_start_ns: self.parent_support_start_ns,
                parent_support_end_ns: self.parent_support_end_ns,
                support_start_ns: self.support_start_ns,
                support_end_ns: self.support_end_ns,
                predecessor_receipt_chain_sha256: &self.predecessor_receipt_chain_sha256,
                receipt_chain_sha256: &self.receipt_chain_sha256,
                complete_projection_sha256: &self.complete_projection_sha256,
            })
            .map_err(|_| FrozenLitterProjectionRestartError::Authority)?,
        )
    }

    pub fn validate(&self) -> Result<(), FrozenLitterProjectionRestartError> {
        if self.schema != FROZEN_LITTER_PUBLICATION_AUTHORITY_V3_SCHEMA
            || self.version != 3
            || self.transaction_id.0 == 0
            || self.parent_support_start_ns > self.support_start_ns
            || self.support_end_ns > self.parent_support_end_ns
            || self.support_start_ns >= self.support_end_ns
            || self.parent_support_start_ns >= self.parent_support_end_ns
            || (self.support_end_ns - self.support_start_ns) < MINIMUM_SUPPORT_NS
            || (self.support_end_ns - self.support_start_ns) % MINIMUM_SUPPORT_NS != 0
            || self
                .predecessor_transaction_id
                .is_some_and(|predecessor| predecessor.0 >= self.transaction_id.0)
            || self
                .receipt_chain_sha256
                .as_str()
                .chars()
                .all(|value| value == '0')
            || self
                .complete_projection_sha256
                .as_str()
                .chars()
                .all(|value| value == '0')
            || self.publication_authority_sha256 != self.compute_digest()?
        {
            return Err(FrozenLitterProjectionRestartError::Authority);
        }
        Ok(())
    }
}

pub fn validate_frozen_litter_projection_v3(
    configuration: &SurfaceLiquidConfigurationV2,
    bytes: &[u8],
    authority: &FrozenLitterPublicationAuthorityV3,
) -> Result<ValidatedFrozenLitterProjectionV3, FrozenLitterProjectionRestartError> {
    authority.validate()?;
    let projection =
        SurfaceLiquidCompleteOwnerProjectionV3::from_canonical_bytes(configuration, bytes)
            .map_err(|_| FrozenLitterProjectionRestartError::Projection)?;
    let replay_authority = FrozenLitterPublicationAuthorityV3::from_projection(&projection)?;
    if &replay_authority != authority {
        return Err(FrozenLitterProjectionRestartError::Identity);
    }
    Ok(ValidatedFrozenLitterProjectionV3 {
        authority: replay_authority,
        ending_surface_owner_bytes: projection.envelope_bytes().to_vec(),
        wb14_parent_working_state_bytes: projection.wb14_parent_working_state_bytes().to_vec(),
        soil_thermal_owner_envelope_bytes: projection.soil_thermal_owner_envelope_bytes().to_vec(),
        soil_thermal_restart_identity_bytes: projection
            .soil_thermal_restart_identity_bytes()
            .to_vec(),
    })
}

fn wire_digest(value: &str) -> Result<Sha256Hex, FrozenLitterProjectionRestartError> {
    Sha256Hex::try_new(value.to_owned()).map_err(|_| FrozenLitterProjectionRestartError::Authority)
}

fn zero_digest() -> Result<Sha256Hex, FrozenLitterProjectionRestartError> {
    wire_digest(&"0".repeat(64))
}
