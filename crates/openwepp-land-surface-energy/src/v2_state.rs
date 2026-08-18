//! Nominal LSE-V2 state and explicit V1-to-V2 identity migration.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyError, LandSurfaceEnergyState,
    MODEL_DEFINITION_SHA256, V2_MODEL_DEFINITION_SHA256,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
pub struct LandSurfaceEnergyV2State(pub LandSurfaceEnergyState);

impl LandSurfaceEnergyV2State {
    /// # Errors
    /// Returns a typed identity or imported-payload error on any mismatch.
    pub fn validate(
        &self,
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<(), LseV2StateError> {
        configuration
            .validate_v2()
            .map_err(LseV2StateError::Configuration)?;
        if self.0.model_definition_sha256.as_str() != V2_MODEL_DEFINITION_SHA256
            || self.0.configuration_sha256 != configuration.configuration_sha256
            || self.0.owner_id != configuration.owner_id
            || self.0.state_sha256
                != self
                    .0
                    .canonical_sha256()
                    .map_err(LseV2StateError::Configuration)?
        {
            return Err(LseV2StateError::Identity);
        }
        let (v1_config, v1_state) = project_v2_runtime_to_v1_unchecked(
            configuration,
            self,
            &configuration.vegetation_configuration.configuration_sha256,
        )?;
        v1_state
            .validate(&v1_config)
            .map_err(LseV2StateError::ImportedV1)
    }
}

/// # Errors
/// Returns a typed error unless both identities and every imported value match.
pub fn migrate_v1_runtime_to_v2(
    source_configuration: &LandSurfaceEnergyConfiguration,
    source_state: &LandSurfaceEnergyState,
    target_configuration: &LandSurfaceEnergyConfiguration,
    target_vegetation_configuration_sha256: &crate::Sha256Digest,
) -> Result<LandSurfaceEnergyV2State, LseV2StateError> {
    source_configuration
        .validate()
        .map_err(LseV2StateError::Configuration)?;
    source_state
        .validate(source_configuration)
        .map_err(LseV2StateError::ImportedV1)?;
    target_configuration
        .validate_v2()
        .map_err(LseV2StateError::Configuration)?;
    require_vegetation_receipt(target_configuration, target_vegetation_configuration_sha256)?;
    if !payload_matches(source_configuration, target_configuration)? {
        return Err(LseV2StateError::PayloadMismatch);
    }
    let mut state = source_state.clone();
    state.model_definition_sha256 = digest(V2_MODEL_DEFINITION_SHA256)?;
    state
        .configuration_sha256
        .clone_from(&target_configuration.configuration_sha256);
    state.state_sha256 = state
        .canonical_sha256()
        .map_err(LseV2StateError::Configuration)?;
    let state = LandSurfaceEnergyV2State(state);
    state.validate(target_configuration)?;
    Ok(state)
}

/// # Errors
/// Returns a typed error when the V2 owner is invalid or cannot be rebound.
pub fn project_v2_runtime_to_v1(
    configuration: &LandSurfaceEnergyConfiguration,
    state: &LandSurfaceEnergyV2State,
    target_v1_vegetation_configuration_sha256: &crate::Sha256Digest,
) -> Result<(LandSurfaceEnergyConfiguration, LandSurfaceEnergyState), LseV2StateError> {
    state.validate(configuration)?;
    project_v2_runtime_to_v1_unchecked(
        configuration,
        state,
        target_v1_vegetation_configuration_sha256,
    )
}

/// # Errors
/// Returns a typed error on source, target, or bit-identity mismatch.
pub fn project_validated_v1_runtime_to_v2(
    source_configuration: &LandSurfaceEnergyConfiguration,
    state: &LandSurfaceEnergyState,
    target_configuration: &LandSurfaceEnergyConfiguration,
    target_vegetation_configuration_sha256: &crate::Sha256Digest,
) -> Result<LandSurfaceEnergyV2State, LseV2StateError> {
    source_configuration
        .validate()
        .map_err(LseV2StateError::Configuration)?;
    state
        .validate(source_configuration)
        .map_err(LseV2StateError::ImportedV1)?;
    target_configuration
        .validate_v2()
        .map_err(LseV2StateError::Configuration)?;
    require_vegetation_receipt(target_configuration, target_vegetation_configuration_sha256)?;
    if !payload_matches(source_configuration, target_configuration)? {
        return Err(LseV2StateError::PayloadMismatch);
    }
    let mut projected = state.clone();
    projected.model_definition_sha256 = digest(V2_MODEL_DEFINITION_SHA256)?;
    projected
        .configuration_sha256
        .clone_from(&target_configuration.configuration_sha256);
    projected.state_sha256 = projected
        .canonical_sha256()
        .map_err(LseV2StateError::Configuration)?;
    let projected = LandSurfaceEnergyV2State(projected);
    projected.validate(target_configuration)?;
    Ok(projected)
}

fn project_v2_runtime_to_v1_unchecked(
    configuration: &LandSurfaceEnergyConfiguration,
    state: &LandSurfaceEnergyV2State,
    target_v1_vegetation_configuration_sha256: &crate::Sha256Digest,
) -> Result<(LandSurfaceEnergyConfiguration, LandSurfaceEnergyState), LseV2StateError> {
    let mut config = configuration.clone();
    config.model_version = crate::MODEL_VERSION.into();
    config.model_definition_sha256 = digest(MODEL_DEFINITION_SHA256)?;
    config.vegetation_configuration.model_version = crate::VEGETATION_MODEL_VERSION.into();
    config.vegetation_configuration.model_definition_sha256 =
        digest(crate::VEGETATION_MODEL_DEFINITION_SHA256)?;
    config
        .vegetation_configuration
        .configuration_sha256
        .clone_from(target_v1_vegetation_configuration_sha256);
    config.configuration_sha256 = config
        .canonical_sha256()
        .map_err(LseV2StateError::Configuration)?;
    let mut projected = state.0.clone();
    projected.model_definition_sha256 = digest(MODEL_DEFINITION_SHA256)?;
    projected
        .configuration_sha256
        .clone_from(&config.configuration_sha256);
    projected.state_sha256 = projected
        .canonical_sha256()
        .map_err(LseV2StateError::Configuration)?;
    Ok((config, projected))
}

fn require_vegetation_receipt(
    configuration: &LandSurfaceEnergyConfiguration,
    expected: &crate::Sha256Digest,
) -> Result<(), LseV2StateError> {
    if &configuration.vegetation_configuration.configuration_sha256 != expected {
        return Err(LseV2StateError::VegetationIdentity);
    }
    Ok(())
}

fn payload_matches(
    source: &LandSurfaceEnergyConfiguration,
    target: &LandSurfaceEnergyConfiguration,
) -> Result<bool, LseV2StateError> {
    let mut source = source.clone();
    let mut target = target.clone();
    for value in [&mut source, &mut target] {
        value.model_version.clear();
        value.model_definition_sha256 = digest(&"0".repeat(64))?;
        value.configuration_sha256 = digest(&"0".repeat(64))?;
        value.vegetation_configuration.model_version.clear();
        value.vegetation_configuration.model_definition_sha256 = digest(&"0".repeat(64))?;
        value.vegetation_configuration.configuration_sha256 = digest(&"0".repeat(64))?;
    }
    serde_json::to_vec(&source)
        .and_then(|left| serde_json::to_vec(&target).map(|right| left == right))
        .map_err(|error| LseV2StateError::Serialization(error.to_string()))
}

fn digest(value: &str) -> Result<crate::Sha256Digest, LseV2StateError> {
    crate::Sha256Digest::try_new(value).map_err(LseV2StateError::Configuration)
}

#[derive(Debug, Error, PartialEq)]
pub enum LseV2StateError {
    #[error("LSE-E-111: invalid LSE-V2 configuration: {0}")]
    Configuration(LandSurfaceEnergyError),
    #[error("LSE-E-111: invalid LSE-V2 identity")]
    Identity,
    #[error("LSE-E-111: imported LSE-V1 state rejected: {0}")]
    ImportedV1(LandSurfaceEnergyError),
    #[error("LSE-E-111: V1/V2 nonidentity payload differs")]
    PayloadMismatch,
    #[error("LSE-E-110: V10 vegetation configuration receipt mismatch")]
    VegetationIdentity,
    #[error("LSE-E-111: canonical serialization failed: {0}")]
    Serialization(String),
}
