//! Canonical restart framing for the immutable surface-owner V2 envelope.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::v2::{
    SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerEnvelopeV2, decode_hex, encode_hex,
};
use super::{DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError, ZERO_SHA256};

const RESTART_SCHEMA_NAME: &str = "OPENWEPP_SURFACE_LIQUID_OWNER_RESTART_V2";

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceLiquidOwnerRestartV2 {
    schema_sha256: String,
    model_definition_sha256: String,
    parent_identity_sha256: String,
    envelope_sha256: String,
    restart_sha256: String,
    envelope: SurfaceLiquidOwnerEnvelopeV2,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidOwnerRestartV2 {
    schema_sha256: String,
    model_definition_sha256: String,
    parent_identity_sha256: String,
    envelope_sha256: String,
    restart_sha256: String,
    envelope_bytes_hex: String,
}

impl SurfaceLiquidOwnerRestartV2 {
    pub fn new(
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
        envelope: SurfaceLiquidOwnerEnvelopeV2,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        envelope.canonical_bytes(v1_configuration, v2_configuration)?;
        let mut value = Self {
            schema_sha256: sha256(RESTART_SCHEMA_NAME.as_bytes()),
            model_definition_sha256: envelope.model_definition_sha256().into(),
            parent_identity_sha256: envelope.parent_identity_sha256().into(),
            envelope_sha256: envelope.envelope_sha256().into(),
            restart_sha256: ZERO_SHA256.into(),
            envelope,
        };
        value.restart_sha256 = value.recomputed_sha256(v1_configuration, v2_configuration)?;
        value.validate(v1_configuration, v2_configuration)?;
        Ok(value)
    }

    #[must_use]
    pub fn restart_sha256(&self) -> &str {
        &self.restart_sha256
    }

    #[must_use]
    pub const fn envelope(&self) -> &SurfaceLiquidOwnerEnvelopeV2 {
        &self.envelope
    }

    pub fn canonical_bytes(
        &self,
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        self.validate(v1_configuration, v2_configuration)?;
        self.canonical_bytes_with_digest(v1_configuration, v2_configuration, &self.restart_sha256)
    }

    pub fn from_canonical_bytes(
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
        bytes: &[u8],
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let wire: CanonicalSurfaceLiquidOwnerRestartV2 = serde_json::from_slice(bytes)
            .map_err(|_| DirectSurfaceLiquidError::Schema("surface-owner restart V2 parse"))?;
        let envelope = SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
            v1_configuration,
            v2_configuration,
            &decode_hex(&wire.envelope_bytes_hex)?,
        )?;
        let value = Self {
            schema_sha256: wire.schema_sha256,
            model_definition_sha256: wire.model_definition_sha256,
            parent_identity_sha256: wire.parent_identity_sha256,
            envelope_sha256: wire.envelope_sha256,
            restart_sha256: wire.restart_sha256,
            envelope,
        };
        value.validate(v1_configuration, v2_configuration)?;
        if value.canonical_bytes(v1_configuration, v2_configuration)? != bytes {
            return Err(DirectSurfaceLiquidError::Schema(
                "noncanonical surface-owner restart V2 bytes",
            ));
        }
        Ok(value)
    }

    fn validate(
        &self,
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
    ) -> Result<(), DirectSurfaceLiquidError> {
        self.envelope
            .canonical_bytes(v1_configuration, v2_configuration)?;
        if self.schema_sha256 != sha256(RESTART_SCHEMA_NAME.as_bytes())
            || self.model_definition_sha256 != self.envelope.model_definition_sha256()
            || self.parent_identity_sha256 != self.envelope.parent_identity_sha256()
            || self.envelope_sha256 != self.envelope.envelope_sha256()
            || self.restart_sha256 == ZERO_SHA256
            || self.restart_sha256 != self.recomputed_sha256(v1_configuration, v2_configuration)?
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner restart V2 identity mismatch",
            ));
        }
        Ok(())
    }

    fn recomputed_sha256(
        &self,
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
    ) -> Result<String, DirectSurfaceLiquidError> {
        Ok(sha256(&self.canonical_bytes_with_digest(
            v1_configuration,
            v2_configuration,
            ZERO_SHA256,
        )?))
    }

    fn canonical_bytes_with_digest(
        &self,
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
        digest: &str,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        let envelope_bytes = self
            .envelope
            .canonical_bytes(v1_configuration, v2_configuration)?;
        serde_json::to_vec(&CanonicalSurfaceLiquidOwnerRestartV2 {
            schema_sha256: self.schema_sha256.clone(),
            model_definition_sha256: self.model_definition_sha256.clone(),
            parent_identity_sha256: self.parent_identity_sha256.clone(),
            envelope_sha256: self.envelope_sha256.clone(),
            restart_sha256: digest.into(),
            envelope_bytes_hex: encode_hex(&envelope_bytes),
        })
        .map_err(|_| DirectSurfaceLiquidError::Schema("surface-owner restart V2 serialization"))
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
