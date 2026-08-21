//! Canonical persisted restart for the default-off Child 2C handoff.

use openwepp_hillslope_orchestrator::snow_stage3_terminal_handoff::SnowStage3HandoffRuntime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Sha256Hex, canonical_sha256, from_canonical_bytes, to_canonical_bytes};

const SCHEMA: &str = "OPENWEPP_SNOW_STAGE3_TERMINAL_HANDOFF_RESTART_V1";
const VERSION: u16 = 1;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3HandoffRestartV1 {
    pub schema: String,
    pub version: u16,
    pub runtime: SnowStage3HandoffRuntime,
    pub payload_sha256: Sha256Hex,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SnowStage3HandoffRestartError {
    #[error("schema")]
    Schema,
    #[error("version")]
    Version,
    #[error("payload_digest")]
    PayloadDigest,
    #[error("runtime")]
    Runtime,
    #[error("canonical")]
    Canonical,
}

#[derive(Serialize)]
struct DigestInput<'a> {
    schema: &'a str,
    version: u16,
    runtime: &'a SnowStage3HandoffRuntime,
}

impl SnowStage3HandoffRestartV1 {
    pub fn project(
        runtime: &SnowStage3HandoffRuntime,
    ) -> Result<Self, SnowStage3HandoffRestartError> {
        runtime
            .validate_restored()
            .map_err(|_| SnowStage3HandoffRestartError::Runtime)?;
        let mut checkpoint = Self {
            schema: SCHEMA.into(),
            version: VERSION,
            runtime: runtime.clone(),
            payload_sha256: Sha256Hex::try_new("0".repeat(64))
                .map_err(|_| SnowStage3HandoffRestartError::PayloadDigest)?,
        };
        checkpoint.seal()?;
        Ok(checkpoint)
    }

    fn compute_digest(&self) -> Result<Sha256Hex, SnowStage3HandoffRestartError> {
        Sha256Hex::try_new(
            canonical_sha256(&DigestInput {
                schema: &self.schema,
                version: self.version,
                runtime: &self.runtime,
            })
            .map_err(|_| SnowStage3HandoffRestartError::Canonical)?,
        )
        .map_err(|_| SnowStage3HandoffRestartError::PayloadDigest)
    }

    fn seal(&mut self) -> Result<(), SnowStage3HandoffRestartError> {
        self.payload_sha256 = self.compute_digest()?;
        Ok(())
    }

    pub fn to_canonical_json(&self) -> Result<Vec<u8>, SnowStage3HandoffRestartError> {
        to_canonical_bytes(self).map_err(|_| SnowStage3HandoffRestartError::Canonical)
    }

    pub fn admit(bytes: &[u8]) -> Result<SnowStage3HandoffRuntime, SnowStage3HandoffRestartError> {
        let checkpoint: Self =
            from_canonical_bytes(bytes).map_err(|_| SnowStage3HandoffRestartError::Canonical)?;
        if checkpoint.schema != SCHEMA {
            return Err(SnowStage3HandoffRestartError::Schema);
        }
        if checkpoint.version != VERSION {
            return Err(SnowStage3HandoffRestartError::Version);
        }
        if checkpoint.compute_digest()? != checkpoint.payload_sha256 {
            return Err(SnowStage3HandoffRestartError::PayloadDigest);
        }
        checkpoint
            .runtime
            .validate_restored()
            .map_err(|_| SnowStage3HandoffRestartError::Runtime)?;
        Ok(checkpoint.runtime)
    }
}
