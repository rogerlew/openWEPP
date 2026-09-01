//! Canonical frozen-litter V3 checkpoint framing and admission.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    FrozenLitterExpectedScientificContextV3, FrozenLitterScientificOwnerRestartError,
    FrozenLitterScientificOwnerStateV3, RestoredFrozenLitterScientificOwnerV3, Sha256Hex,
    canonical_sha256, from_canonical_bytes,
};

pub const DIRECT_FROZEN_LITTER_CHECKPOINT_V3_SCHEMA: &str =
    "OPENWEPP_DIRECT_FROZEN_LITTER_CHECKPOINT_V3";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectFrozenLitterCheckpointV3 {
    pub schema: String,
    pub version: u16,
    pub parent_v2_checkpoint_sha256: Sha256Hex,
    pub run_identity_sha256: Sha256Hex,
    pub topology_sha256: Sha256Hex,
    pub scientific: FrozenLitterScientificOwnerStateV3,
    pub payload_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct CheckpointDigestBody<'a> {
    schema: &'a str,
    version: u16,
    parent_v2_checkpoint_sha256: &'a Sha256Hex,
    run_identity_sha256: &'a Sha256Hex,
    topology_sha256: &'a Sha256Hex,
    scientific: &'a FrozenLitterScientificOwnerStateV3,
}

pub struct ExpectedFrozenLitterCheckpointContextV3<'a> {
    pub parent_v2_checkpoint_sha256: &'a Sha256Hex,
    pub run_identity_sha256: &'a Sha256Hex,
    pub topology_sha256: &'a Sha256Hex,
    pub scientific: FrozenLitterExpectedScientificContextV3<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IsolatedRestoredFrozenLitterCheckpointV3 {
    pub persisted: DirectFrozenLitterCheckpointV3,
    pub scientific: RestoredFrozenLitterScientificOwnerV3,
}

#[derive(Debug, Error, Clone, Copy, Eq, PartialEq)]
pub enum FrozenLitterCheckpointAdmissionErrorV3 {
    #[error("schema")]
    Schema,
    #[error("unsupported_version")]
    UnsupportedVersion,
    #[error("noncanonical_bytes")]
    NoncanonicalBytes,
    #[error("payload_digest")]
    PayloadDigest,
    #[error("parent_v2_checkpoint")]
    ParentV2Checkpoint,
    #[error("run_identity")]
    RunIdentity,
    #[error("topology_identity")]
    TopologyIdentity,
    #[error("scientific_owner: {0}")]
    ScientificOwner(#[from] FrozenLitterScientificOwnerRestartError),
}

impl DirectFrozenLitterCheckpointV3 {
    pub fn new(
        parent_v2_checkpoint_sha256: Sha256Hex,
        run_identity_sha256: Sha256Hex,
        topology_sha256: Sha256Hex,
        scientific: FrozenLitterScientificOwnerStateV3,
    ) -> Result<Self, FrozenLitterCheckpointAdmissionErrorV3> {
        if is_zero_digest(&parent_v2_checkpoint_sha256) {
            return Err(FrozenLitterCheckpointAdmissionErrorV3::ParentV2Checkpoint);
        }
        if is_zero_digest(&run_identity_sha256) {
            return Err(FrozenLitterCheckpointAdmissionErrorV3::RunIdentity);
        }
        if is_zero_digest(&topology_sha256) {
            return Err(FrozenLitterCheckpointAdmissionErrorV3::TopologyIdentity);
        }
        let mut value = Self {
            schema: DIRECT_FROZEN_LITTER_CHECKPOINT_V3_SCHEMA.to_owned(),
            version: 3,
            parent_v2_checkpoint_sha256,
            run_identity_sha256,
            topology_sha256,
            scientific,
            payload_sha256: zero_digest()?,
        };
        value.payload_sha256 = value.compute_digest()?;
        Ok(value)
    }

    pub fn compute_digest(&self) -> Result<Sha256Hex, FrozenLitterCheckpointAdmissionErrorV3> {
        wire_digest(
            &canonical_sha256(&CheckpointDigestBody {
                schema: &self.schema,
                version: self.version,
                parent_v2_checkpoint_sha256: &self.parent_v2_checkpoint_sha256,
                run_identity_sha256: &self.run_identity_sha256,
                topology_sha256: &self.topology_sha256,
                scientific: &self.scientific,
            })
            .map_err(|_| FrozenLitterCheckpointAdmissionErrorV3::PayloadDigest)?,
        )
    }

    pub fn seal(&mut self) -> Result<(), FrozenLitterCheckpointAdmissionErrorV3> {
        self.payload_sha256 = self.compute_digest()?;
        Ok(())
    }
}

pub fn admit_frozen_litter_checkpoint_v3(
    bytes: &[u8],
    context: &ExpectedFrozenLitterCheckpointContextV3<'_>,
) -> Result<IsolatedRestoredFrozenLitterCheckpointV3, FrozenLitterCheckpointAdmissionErrorV3> {
    let checkpoint: DirectFrozenLitterCheckpointV3 = from_canonical_bytes(bytes)
        .map_err(|_| FrozenLitterCheckpointAdmissionErrorV3::NoncanonicalBytes)?;
    if checkpoint.schema != DIRECT_FROZEN_LITTER_CHECKPOINT_V3_SCHEMA {
        return Err(FrozenLitterCheckpointAdmissionErrorV3::Schema);
    }
    if checkpoint.version != 3 {
        return Err(FrozenLitterCheckpointAdmissionErrorV3::UnsupportedVersion);
    }
    if checkpoint.payload_sha256 != checkpoint.compute_digest()? {
        return Err(FrozenLitterCheckpointAdmissionErrorV3::PayloadDigest);
    }
    if checkpoint.parent_v2_checkpoint_sha256 != *context.parent_v2_checkpoint_sha256
        || is_zero_digest(&checkpoint.parent_v2_checkpoint_sha256)
    {
        return Err(FrozenLitterCheckpointAdmissionErrorV3::ParentV2Checkpoint);
    }
    if &checkpoint.run_identity_sha256 != context.run_identity_sha256
        || is_zero_digest(&checkpoint.run_identity_sha256)
    {
        return Err(FrozenLitterCheckpointAdmissionErrorV3::RunIdentity);
    }
    if &checkpoint.topology_sha256 != context.topology_sha256
        || is_zero_digest(&checkpoint.topology_sha256)
    {
        return Err(FrozenLitterCheckpointAdmissionErrorV3::TopologyIdentity);
    }
    let scientific = checkpoint.scientific.validate(&context.scientific)?;
    Ok(IsolatedRestoredFrozenLitterCheckpointV3 {
        persisted: checkpoint,
        scientific,
    })
}

fn wire_digest(value: &str) -> Result<Sha256Hex, FrozenLitterCheckpointAdmissionErrorV3> {
    Sha256Hex::try_new(value.to_owned())
        .map_err(|_| FrozenLitterCheckpointAdmissionErrorV3::PayloadDigest)
}

fn zero_digest() -> Result<Sha256Hex, FrozenLitterCheckpointAdmissionErrorV3> {
    wire_digest(&"0".repeat(64))
}

fn is_zero_digest(value: &Sha256Hex) -> bool {
    value.as_str().chars().all(|character| character == '0')
}
