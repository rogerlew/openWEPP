//! Persisted V16 exact-surface-enthalpy successor for frozen-litter V3.
//!
//! The frozen-litter V3 checkpoint is nested byte-for-byte. Native exact-owner
//! and projection types retain their own schemas; this module only frames,
//! cross-joins, admits, and atomically installs them.

use openwepp_hillslope_orchestrator::{
    LseSurfaceEnthalpyEnergyCreditReceiptV1, LseSurfaceEnthalpyErrorV1,
    LseSurfaceEnthalpyOwnerCheckpointV1, LseSurfaceEnthalpyOwnerEnvelopeV1,
    LseSurfaceEnthalpyOwnerRestartV1, SurfaceLiquidCompleteOwnerProjectionV4,
};
use openwepp_kernel_contract::ResourceOwnerId;
use openwepp_land_surface_energy::{ExactDyadicEnthalpy, Sha256Digest};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{
    DirectFrozenLitterCheckpointV3, ExpectedFrozenLitterCheckpointContextV3,
    FrozenLitterCheckpointAdmissionErrorV3, IsolatedRestoredFrozenLitterCheckpointV3, Sha256Hex,
    admit_frozen_litter_checkpoint_v3, from_canonical_bytes, to_canonical_bytes,
};

pub const DIRECT_FROZEN_LITTER_EXACT_ENTHALPY_CHECKPOINT_V4_SCHEMA: &str =
    "OPENWEPP_DIRECT_FROZEN_LITTER_EXACT_ENTHALPY_CHECKPOINT_V4";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FrozenLitterExactEnthalpyCheckpointPostureV4 {
    ReceiptFree,
    AcceptedCredit,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeExactEnthalpyFrameV4 {
    pub type_tag: String,
    pub canonical_json: Vec<u8>,
    pub canonical_sha256: Sha256Hex,
}

impl NativeExactEnthalpyFrameV4 {
    fn encode<T: Serialize>(
        type_tag: &'static str,
        value: &T,
    ) -> Result<Self, FrozenLitterExactEnthalpyRestartErrorV4> {
        let canonical_json = serde_json::to_vec(value)
            .map_err(|_| FrozenLitterExactEnthalpyRestartErrorV4::Canonical)?;
        Ok(Self {
            type_tag: type_tag.to_owned(),
            canonical_sha256: sha(&canonical_json)?,
            canonical_json,
        })
    }

    fn decode<T>(
        &self,
        type_tag: &'static str,
    ) -> Result<T, FrozenLitterExactEnthalpyRestartErrorV4>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        if self.type_tag != type_tag || self.canonical_sha256 != sha(&self.canonical_json)? {
            return Err(FrozenLitterExactEnthalpyRestartErrorV4::NativeFrame);
        }
        let value: T = serde_json::from_slice(&self.canonical_json)
            .map_err(|_| FrozenLitterExactEnthalpyRestartErrorV4::NativeFrame)?;
        if serde_json::to_vec(&value)
            .map_err(|_| FrozenLitterExactEnthalpyRestartErrorV4::NativeFrame)?
            != self.canonical_json
        {
            return Err(FrozenLitterExactEnthalpyRestartErrorV4::NativeFrame);
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectFrozenLitterExactEnthalpyCheckpointV4 {
    pub schema: String,
    pub version: u16,
    pub posture: FrozenLitterExactEnthalpyCheckpointPostureV4,
    pub parent_v3_checkpoint_bytes: Vec<u8>,
    pub parent_v3_checkpoint_sha256: Sha256Hex,
    pub parent_v3_scientific_owner_sha256: Sha256Hex,
    pub beginning_exact_surface_owner: Option<NativeExactEnthalpyFrameV4>,
    pub exact_surface_owner: NativeExactEnthalpyFrameV4,
    pub exact_surface_restart: NativeExactEnthalpyFrameV4,
    pub exact_surface_checkpoint: NativeExactEnthalpyFrameV4,
    pub complete_owner_projection_v4: Option<NativeExactEnthalpyFrameV4>,
    pub payload_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct CheckpointDigestBodyV4<'a> {
    schema: &'a str,
    version: u16,
    posture: FrozenLitterExactEnthalpyCheckpointPostureV4,
    parent_v3_checkpoint_bytes: &'a [u8],
    parent_v3_checkpoint_sha256: &'a Sha256Hex,
    parent_v3_scientific_owner_sha256: &'a Sha256Hex,
    beginning_exact_surface_owner: &'a Option<NativeExactEnthalpyFrameV4>,
    exact_surface_owner: &'a NativeExactEnthalpyFrameV4,
    exact_surface_restart: &'a NativeExactEnthalpyFrameV4,
    exact_surface_checkpoint: &'a NativeExactEnthalpyFrameV4,
    complete_owner_projection_v4: &'a Option<NativeExactEnthalpyFrameV4>,
}

pub struct ExpectedFrozenLitterExactEnthalpyContextV4<'a> {
    pub parent_v3: ExpectedFrozenLitterCheckpointContextV3<'a>,
    pub exact_surface_owner_id: &'a ResourceOwnerId,
    pub accepted_support_beginning_lse_v3_state_sha256: &'a Sha256Digest,
    pub publication_history_beginning_lse_v3_state_sha256: &'a Sha256Digest,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RestoredFrozenLitterExactEnthalpyCheckpointV4 {
    pub persisted: DirectFrozenLitterExactEnthalpyCheckpointV4,
    pub parent_v3: IsolatedRestoredFrozenLitterCheckpointV3,
    pub beginning_exact_surface_owner: Option<LseSurfaceEnthalpyOwnerEnvelopeV1>,
    pub exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    pub exact_surface_restart: LseSurfaceEnthalpyOwnerRestartV1,
    pub exact_surface_checkpoint: LseSurfaceEnthalpyOwnerCheckpointV1,
    pub complete_owner_projection_v4: Option<SurfaceLiquidCompleteOwnerProjectionV4>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum FrozenLitterExactEnthalpyRestartErrorV4 {
    #[error("schema")]
    Schema,
    #[error("unsupported_version")]
    UnsupportedVersion,
    #[error("noncanonical_bytes")]
    Canonical,
    #[error("payload_digest")]
    PayloadDigest,
    #[error("parent_v3_checkpoint: {0}")]
    ParentV3(#[from] FrozenLitterCheckpointAdmissionErrorV3),
    #[error("native_frame")]
    NativeFrame,
    #[error("native_exact_owner")]
    NativeOwner,
    #[error("owner_restart_checkpoint_join")]
    SealJoin,
    #[error("projection_v4_join")]
    Projection,
    #[error("cross_owner_identity")]
    Identity,
    #[error("credit_replay")]
    Replay,
}

impl From<LseSurfaceEnthalpyErrorV1> for FrozenLitterExactEnthalpyRestartErrorV4 {
    fn from(_: LseSurfaceEnthalpyErrorV1) -> Self {
        Self::NativeOwner
    }
}

impl DirectFrozenLitterExactEnthalpyCheckpointV4 {
    pub fn receipt_free(
        parent_v3: DirectFrozenLitterCheckpointV3,
        exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
        context: &ExpectedFrozenLitterExactEnthalpyContextV4<'_>,
    ) -> Result<Self, FrozenLitterExactEnthalpyRestartErrorV4> {
        let restart = exact_surface_owner.restart()?;
        let checkpoint = exact_surface_owner.checkpoint(None)?;
        Self::new(
            parent_v3,
            None,
            exact_surface_owner,
            restart,
            checkpoint,
            None,
            FrozenLitterExactEnthalpyCheckpointPostureV4::ReceiptFree,
            context,
        )
    }

    pub fn accepted_credit(
        parent_v3: DirectFrozenLitterCheckpointV3,
        beginning_exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
        exact_surface_restart: LseSurfaceEnthalpyOwnerRestartV1,
        exact_surface_checkpoint: LseSurfaceEnthalpyOwnerCheckpointV1,
        complete_owner_projection_v4: SurfaceLiquidCompleteOwnerProjectionV4,
        context: &ExpectedFrozenLitterExactEnthalpyContextV4<'_>,
    ) -> Result<Self, FrozenLitterExactEnthalpyRestartErrorV4> {
        let exact_surface_owner = complete_owner_projection_v4.exact_surface_owner()?;
        Self::new(
            parent_v3,
            Some(beginning_exact_surface_owner),
            exact_surface_owner,
            exact_surface_restart,
            exact_surface_checkpoint,
            Some(complete_owner_projection_v4),
            FrozenLitterExactEnthalpyCheckpointPostureV4::AcceptedCredit,
            context,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        parent_v3: DirectFrozenLitterCheckpointV3,
        beginning_exact_surface_owner: Option<LseSurfaceEnthalpyOwnerEnvelopeV1>,
        exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
        exact_surface_restart: LseSurfaceEnthalpyOwnerRestartV1,
        exact_surface_checkpoint: LseSurfaceEnthalpyOwnerCheckpointV1,
        complete_owner_projection_v4: Option<SurfaceLiquidCompleteOwnerProjectionV4>,
        posture: FrozenLitterExactEnthalpyCheckpointPostureV4,
        context: &ExpectedFrozenLitterExactEnthalpyContextV4<'_>,
    ) -> Result<Self, FrozenLitterExactEnthalpyRestartErrorV4> {
        let parent_v3_checkpoint_bytes = to_canonical_bytes(&parent_v3)
            .map_err(|_| FrozenLitterExactEnthalpyRestartErrorV4::Canonical)?;
        let parent_v3_checkpoint_sha256 = sha(&parent_v3_checkpoint_bytes)?;
        let parent_v3_scientific_owner_sha256 =
            wire_digest(parent_v3.scientific.scientific_owner_sha256.as_str())?;
        let mut value = Self {
            schema: DIRECT_FROZEN_LITTER_EXACT_ENTHALPY_CHECKPOINT_V4_SCHEMA.to_owned(),
            version: 4,
            posture,
            parent_v3_checkpoint_bytes,
            parent_v3_checkpoint_sha256,
            parent_v3_scientific_owner_sha256,
            beginning_exact_surface_owner: beginning_exact_surface_owner
                .as_ref()
                .map(|owner| {
                    NativeExactEnthalpyFrameV4::encode("LseSurfaceEnthalpyOwnerEnvelopeV1", owner)
                })
                .transpose()?,
            exact_surface_owner: NativeExactEnthalpyFrameV4::encode(
                "LseSurfaceEnthalpyOwnerEnvelopeV1",
                &exact_surface_owner,
            )?,
            exact_surface_restart: NativeExactEnthalpyFrameV4::encode(
                "LseSurfaceEnthalpyOwnerRestartV1",
                &exact_surface_restart,
            )?,
            exact_surface_checkpoint: NativeExactEnthalpyFrameV4::encode(
                "LseSurfaceEnthalpyOwnerCheckpointV1",
                &exact_surface_checkpoint,
            )?,
            complete_owner_projection_v4: complete_owner_projection_v4
                .as_ref()
                .map(|projection| {
                    NativeExactEnthalpyFrameV4::encode(
                        "SurfaceLiquidCompleteOwnerProjectionV4",
                        projection,
                    )
                })
                .transpose()?,
            payload_sha256: zero_digest()?,
        };
        value.payload_sha256 = value.compute_digest()?;
        value.restore(context)?;
        Ok(value)
    }

    pub fn compute_digest(&self) -> Result<Sha256Hex, FrozenLitterExactEnthalpyRestartErrorV4> {
        let bytes = serde_json::to_vec(&CheckpointDigestBodyV4 {
            schema: &self.schema,
            version: self.version,
            posture: self.posture,
            parent_v3_checkpoint_bytes: &self.parent_v3_checkpoint_bytes,
            parent_v3_checkpoint_sha256: &self.parent_v3_checkpoint_sha256,
            parent_v3_scientific_owner_sha256: &self.parent_v3_scientific_owner_sha256,
            beginning_exact_surface_owner: &self.beginning_exact_surface_owner,
            exact_surface_owner: &self.exact_surface_owner,
            exact_surface_restart: &self.exact_surface_restart,
            exact_surface_checkpoint: &self.exact_surface_checkpoint,
            complete_owner_projection_v4: &self.complete_owner_projection_v4,
        })
        .map_err(|_| FrozenLitterExactEnthalpyRestartErrorV4::PayloadDigest)?;
        sha(&bytes)
    }

    pub fn seal(&mut self) -> Result<(), FrozenLitterExactEnthalpyRestartErrorV4> {
        self.payload_sha256 = self.compute_digest()?;
        Ok(())
    }

    fn restore(
        &self,
        context: &ExpectedFrozenLitterExactEnthalpyContextV4<'_>,
    ) -> Result<
        RestoredFrozenLitterExactEnthalpyCheckpointV4,
        FrozenLitterExactEnthalpyRestartErrorV4,
    > {
        if self.schema != DIRECT_FROZEN_LITTER_EXACT_ENTHALPY_CHECKPOINT_V4_SCHEMA {
            return Err(FrozenLitterExactEnthalpyRestartErrorV4::Schema);
        }
        if self.version != 4 {
            return Err(FrozenLitterExactEnthalpyRestartErrorV4::UnsupportedVersion);
        }
        if sha(&self.parent_v3_checkpoint_bytes)? != self.parent_v3_checkpoint_sha256
            || self.compute_digest()? != self.payload_sha256
        {
            return Err(FrozenLitterExactEnthalpyRestartErrorV4::PayloadDigest);
        }
        let parent_v3 = admit_frozen_litter_checkpoint_v3(
            &self.parent_v3_checkpoint_bytes,
            &context.parent_v3,
        )?;
        if parent_v3
            .persisted
            .scientific
            .scientific_owner_sha256
            .as_str()
            != self.parent_v3_scientific_owner_sha256.as_str()
        {
            return Err(FrozenLitterExactEnthalpyRestartErrorV4::Identity);
        }
        let exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1 = self
            .exact_surface_owner
            .decode("LseSurfaceEnthalpyOwnerEnvelopeV1")?;
        exact_surface_owner.validate()?;
        let beginning_exact_surface_owner = self
            .beginning_exact_surface_owner
            .as_ref()
            .map(|frame| frame.decode("LseSurfaceEnthalpyOwnerEnvelopeV1"))
            .transpose()?;
        let exact_surface_restart: LseSurfaceEnthalpyOwnerRestartV1 = self
            .exact_surface_restart
            .decode("LseSurfaceEnthalpyOwnerRestartV1")?;
        let exact_surface_checkpoint: LseSurfaceEnthalpyOwnerCheckpointV1 = self
            .exact_surface_checkpoint
            .decode("LseSurfaceEnthalpyOwnerCheckpointV1")?;
        validate_native_seals(
            &exact_surface_owner,
            &exact_surface_restart,
            &exact_surface_checkpoint,
        )?;
        validate_parent_join(
            &parent_v3,
            &exact_surface_owner,
            context.exact_surface_owner_id,
            context.parent_v3.scientific.lse_configuration,
            context.parent_v3.scientific.surface_liquid_configuration,
        )?;
        let complete_owner_projection_v4 =
            self.complete_owner_projection_v4
                .as_ref()
                .map(|frame| {
                    let projection: SurfaceLiquidCompleteOwnerProjectionV4 =
                        frame.decode("SurfaceLiquidCompleteOwnerProjectionV4")?;
                    projection.validate(
                        context.parent_v3.scientific.surface_liquid_configuration,
                        context
                            .accepted_support_beginning_lse_v3_state_sha256
                            .as_str(),
                    )?;
                    Ok::<
                        SurfaceLiquidCompleteOwnerProjectionV4,
                        FrozenLitterExactEnthalpyRestartErrorV4,
                    >(projection)
                })
                .transpose()?;
        match (self.posture, &complete_owner_projection_v4) {
            (FrozenLitterExactEnthalpyCheckpointPostureV4::ReceiptFree, None) => {
                if beginning_exact_surface_owner.is_some()
                    || exact_surface_checkpoint.receipt.is_some()
                    || exact_surface_owner.receipt_chain_sha256.as_str()
                        != "0000000000000000000000000000000000000000000000000000000000000000"
                    || exact_surface_owner
                        .records
                        .iter()
                        .any(|record| record.last_accepted_transaction_id.is_some())
                    || exact_surface_owner
                        .records
                        .iter()
                        .any(|record| record.enthalpy_carry != ExactDyadicEnthalpy::zero())
                {
                    return Err(FrozenLitterExactEnthalpyRestartErrorV4::SealJoin);
                }
            }
            (FrozenLitterExactEnthalpyCheckpointPostureV4::AcceptedCredit, Some(projection)) => {
                let beginning = beginning_exact_surface_owner
                    .as_ref()
                    .ok_or(FrozenLitterExactEnthalpyRestartErrorV4::SealJoin)?;
                let projection_owner = projection.exact_surface_owner()?;
                let projection_receipt = projection.exact_surface_receipt()?;
                projection_receipt.validate(beginning, &exact_surface_owner)?;
                validate_beginning_ending_join(beginning, &exact_surface_owner)?;
                if projection_owner != exact_surface_owner
                    || exact_surface_checkpoint.receipt.as_ref() != Some(&projection_receipt)
                    || projection
                        .projection_v3(context.parent_v3.scientific.surface_liquid_configuration)?
                        .canonical_bytes(context.parent_v3.scientific.surface_liquid_configuration)
                        .map_err(|_| FrozenLitterExactEnthalpyRestartErrorV4::Projection)?
                        != parent_v3
                            .persisted
                            .scientific
                            .complete_owner_projection_v3_bytes
                {
                    return Err(FrozenLitterExactEnthalpyRestartErrorV4::Projection);
                }
                validate_publication_join(&parent_v3, &projection_receipt)?;
            }
            _ => return Err(FrozenLitterExactEnthalpyRestartErrorV4::Projection),
        }
        Ok(RestoredFrozenLitterExactEnthalpyCheckpointV4 {
            persisted: self.clone(),
            parent_v3,
            beginning_exact_surface_owner,
            exact_surface_owner,
            exact_surface_restart,
            exact_surface_checkpoint,
            complete_owner_projection_v4,
        })
    }
}

pub fn admit_frozen_litter_exact_enthalpy_checkpoint_v4(
    bytes: &[u8],
    context: &ExpectedFrozenLitterExactEnthalpyContextV4<'_>,
) -> Result<RestoredFrozenLitterExactEnthalpyCheckpointV4, FrozenLitterExactEnthalpyRestartErrorV4>
{
    let checkpoint: DirectFrozenLitterExactEnthalpyCheckpointV4 = from_canonical_bytes(bytes)
        .map_err(|_| FrozenLitterExactEnthalpyRestartErrorV4::Canonical)?;
    if to_canonical_bytes(&checkpoint)
        .map_err(|_| FrozenLitterExactEnthalpyRestartErrorV4::Canonical)?
        != bytes
    {
        return Err(FrozenLitterExactEnthalpyRestartErrorV4::Canonical);
    }
    checkpoint.restore(context)
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectFrozenLitterExactEnthalpyRestartHostV4 {
    admitted: RestoredFrozenLitterExactEnthalpyCheckpointV4,
}

impl DirectFrozenLitterExactEnthalpyRestartHostV4 {
    #[must_use]
    pub const fn from_isolated(admitted: RestoredFrozenLitterExactEnthalpyCheckpointV4) -> Self {
        Self { admitted }
    }

    #[must_use]
    pub const fn admitted(&self) -> &RestoredFrozenLitterExactEnthalpyCheckpointV4 {
        &self.admitted
    }
}

pub fn admit_and_install_frozen_litter_exact_enthalpy_checkpoint_v4(
    target: &mut DirectFrozenLitterExactEnthalpyRestartHostV4,
    bytes: &[u8],
    context: &ExpectedFrozenLitterExactEnthalpyContextV4<'_>,
) -> Result<(), FrozenLitterExactEnthalpyRestartErrorV4> {
    let admitted = admit_frozen_litter_exact_enthalpy_checkpoint_v4(bytes, context)?;
    *target = DirectFrozenLitterExactEnthalpyRestartHostV4::from_isolated(admitted);
    Ok(())
}

pub fn advance_frozen_litter_exact_enthalpy_checkpoint_v4(
    target: &mut DirectFrozenLitterExactEnthalpyRestartHostV4,
    bytes: &[u8],
    context: &ExpectedFrozenLitterExactEnthalpyContextV4<'_>,
) -> Result<(), FrozenLitterExactEnthalpyRestartErrorV4> {
    let candidate = admit_frozen_litter_exact_enthalpy_checkpoint_v4(bytes, context)?;
    validate_successor_frames(
        &target.admitted.persisted.exact_surface_owner,
        candidate.persisted.beginning_exact_surface_owner.as_ref(),
    )?;
    let beginning = candidate
        .beginning_exact_surface_owner
        .as_ref()
        .ok_or(FrozenLitterExactEnthalpyRestartErrorV4::Replay)?;
    let current = &target.admitted.exact_surface_owner;
    if beginning != current {
        return Err(FrozenLitterExactEnthalpyRestartErrorV4::Replay);
    }
    *target = DirectFrozenLitterExactEnthalpyRestartHostV4::from_isolated(candidate);
    Ok(())
}

fn validate_successor_frames(
    current: &NativeExactEnthalpyFrameV4,
    candidate_beginning: Option<&NativeExactEnthalpyFrameV4>,
) -> Result<(), FrozenLitterExactEnthalpyRestartErrorV4> {
    if candidate_beginning != Some(current) {
        return Err(FrozenLitterExactEnthalpyRestartErrorV4::Replay);
    }
    Ok(())
}

fn validate_native_seals(
    owner: &LseSurfaceEnthalpyOwnerEnvelopeV1,
    restart: &LseSurfaceEnthalpyOwnerRestartV1,
    checkpoint: &LseSurfaceEnthalpyOwnerCheckpointV1,
) -> Result<(), FrozenLitterExactEnthalpyRestartErrorV4> {
    restart.validate()?;
    checkpoint.validate()?;
    if restart.owner != *owner || checkpoint.owner != *owner {
        return Err(FrozenLitterExactEnthalpyRestartErrorV4::SealJoin);
    }
    Ok(())
}

fn validate_beginning_ending_join(
    beginning: &LseSurfaceEnthalpyOwnerEnvelopeV1,
    ending: &LseSurfaceEnthalpyOwnerEnvelopeV1,
) -> Result<(), FrozenLitterExactEnthalpyRestartErrorV4> {
    beginning.validate()?;
    if beginning.owner_tag != ending.owner_tag
        || beginning.schema_sha256 != ending.schema_sha256
        || beginning.exact_carry_definition_sha256 != ending.exact_carry_definition_sha256
        || beginning.owner_id != ending.owner_id
        || beginning.run_id != ending.run_id
        || beginning.configuration_sha256 != ending.configuration_sha256
        || beginning.records.len() != ending.records.len()
        || beginning
            .records
            .iter()
            .zip(&ending.records)
            .any(|(left, right)| left.surface_key != right.surface_key)
    {
        return Err(FrozenLitterExactEnthalpyRestartErrorV4::Identity);
    }
    Ok(())
}

fn validate_parent_join(
    parent: &IsolatedRestoredFrozenLitterCheckpointV3,
    owner: &LseSurfaceEnthalpyOwnerEnvelopeV1,
    expected_owner_id: &ResourceOwnerId,
    lse_configuration: &openwepp_land_surface_energy::LandSurfaceEnergyConfiguration,
    surface_configuration: &openwepp_hillslope_orchestrator::SurfaceLiquidConfigurationV2,
) -> Result<(), FrozenLitterExactEnthalpyRestartErrorV4> {
    owner.validate_frozen_parent_join(
        lse_configuration,
        &parent.scientific.lse_v3,
        surface_configuration,
        &parent.scientific.surface_liquid_v2,
    )?;
    if &owner.owner_id != expected_owner_id {
        return Err(FrozenLitterExactEnthalpyRestartErrorV4::Identity);
    }
    Ok(())
}

fn validate_publication_join(
    parent: &IsolatedRestoredFrozenLitterCheckpointV3,
    receipt: &LseSurfaceEnthalpyEnergyCreditReceiptV1,
) -> Result<(), FrozenLitterExactEnthalpyRestartErrorV4> {
    let authority = &parent.persisted.scientific.publication_authority;
    if receipt.transaction_id != authority.transaction_id
        || receipt.predecessor_transaction_id != authority.predecessor_transaction_id
        || receipt.support_start_ns != authority.support_start_ns
        || receipt.support_end_ns != authority.support_end_ns
    {
        return Err(FrozenLitterExactEnthalpyRestartErrorV4::Identity);
    }
    Ok(())
}

fn sha(bytes: &[u8]) -> Result<Sha256Hex, FrozenLitterExactEnthalpyRestartErrorV4> {
    wire_digest(&format!("{:x}", Sha256::digest(bytes)))
}

fn wire_digest(value: &str) -> Result<Sha256Hex, FrozenLitterExactEnthalpyRestartErrorV4> {
    Sha256Hex::try_new(value.to_owned())
        .map_err(|_| FrozenLitterExactEnthalpyRestartErrorV4::PayloadDigest)
}

fn zero_digest() -> Result<Sha256Hex, FrozenLitterExactEnthalpyRestartErrorV4> {
    wire_digest(&"0".repeat(64))
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(deny_unknown_fields)]
    struct NativeProbe {
        high_bits: u64,
        carry: String,
    }

    #[test]
    fn v4_schema_is_additive_and_version_specific() {
        assert_eq!(
            DIRECT_FROZEN_LITTER_EXACT_ENTHALPY_CHECKPOINT_V4_SCHEMA,
            "OPENWEPP_DIRECT_FROZEN_LITTER_EXACT_ENTHALPY_CHECKPOINT_V4"
        );
        assert_ne!(
            DIRECT_FROZEN_LITTER_EXACT_ENTHALPY_CHECKPOINT_V4_SCHEMA,
            crate::DIRECT_FROZEN_LITTER_CHECKPOINT_V3_SCHEMA
        );
    }

    #[test]
    fn native_frames_require_exact_type_digest_and_canonical_reencoding() {
        let probe = NativeProbe {
            high_bits: (-34_315.421_541_136_02_f64).to_bits(),
            carry: "-1dc319224e55f@-109".to_owned(),
        };
        let frame = NativeExactEnthalpyFrameV4::encode("NativeProbe", &probe)
            .expect("canonical native frame");
        assert_eq!(
            frame
                .decode::<NativeProbe>("NativeProbe")
                .expect("exact replay"),
            probe
        );
        assert_eq!(
            frame.decode::<NativeProbe>("WrongType"),
            Err(FrozenLitterExactEnthalpyRestartErrorV4::NativeFrame)
        );

        let mut digest_poison = frame.clone();
        digest_poison.canonical_sha256 = wire_digest(&"f".repeat(64)).expect("poison digest");
        assert_eq!(
            digest_poison.decode::<NativeProbe>("NativeProbe"),
            Err(FrozenLitterExactEnthalpyRestartErrorV4::NativeFrame)
        );

        let mut bytes_poison = frame;
        bytes_poison.canonical_json.push(b' ');
        bytes_poison.canonical_sha256 = sha(&bytes_poison.canonical_json).expect("poison seal");
        assert_eq!(
            bytes_poison.decode::<NativeProbe>("NativeProbe"),
            Err(FrozenLitterExactEnthalpyRestartErrorV4::NativeFrame)
        );
    }

    #[test]
    fn checkpoint_wire_refuses_unknown_fields_before_any_host_mutation() {
        let bytes = br#"{"schema":"OPENWEPP_DIRECT_FROZEN_LITTER_EXACT_ENTHALPY_CHECKPOINT_V4","version":4,"unknown":true}"#;
        assert!(
            from_canonical_bytes::<DirectFrozenLitterExactEnthalpyCheckpointV4>(bytes).is_err()
        );
    }

    #[test]
    fn split_successor_join_refuses_replay_and_preserves_current_bytes() {
        let before = NativeExactEnthalpyFrameV4::encode(
            "LseSurfaceEnthalpyOwnerEnvelopeV1",
            &NativeProbe {
                high_bits: 1.0_f64.to_bits(),
                carry: "0".to_owned(),
            },
        )
        .expect("before-credit frame");
        let after = NativeExactEnthalpyFrameV4::encode(
            "LseSurfaceEnthalpyOwnerEnvelopeV1",
            &NativeProbe {
                high_bits: 1.0_f64.to_bits(),
                carry: "+1@-1074".to_owned(),
            },
        )
        .expect("after-credit frame");

        assert_eq!(validate_successor_frames(&before, Some(&before)), Ok(()));
        assert_eq!(
            validate_successor_frames(&after, Some(&before)),
            Err(FrozenLitterExactEnthalpyRestartErrorV4::Replay),
            "the same accepted credit cannot replay against its ending owner"
        );
        let rollback = after.clone();
        assert_eq!(
            validate_successor_frames(&after, None),
            Err(FrozenLitterExactEnthalpyRestartErrorV4::Replay)
        );
        assert_eq!(after, rollback, "refusal preserves the installed bytes");
    }
}
