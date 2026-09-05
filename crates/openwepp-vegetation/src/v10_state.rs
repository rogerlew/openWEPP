//! Explicit V9-to-V10 successor identity and default-off migration.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::v9_state::{V9_MODEL_SHA256, V9CoupledOwnedState, V9StateError};
use crate::{VegetationConfiguration, VegetationError};

pub const V10_MODEL_VERSION: &str = "OPENWEPP_C3_WOODY_V10";
pub const V10_MODEL_SHA256: &str =
    "0c42b025b6f9282d85afd5c8819ec9cc60d66a2b79ac6d5922bfdcc8026dd182";
pub const V10_MODEL_BYTES: &[u8] =
    include_bytes!("../model-registry/openwepp_c3_woody_v10_definition.json");

pub fn load_v10_model_definition() -> Result<crate::ModelDefinition, V10StateError> {
    let found = format!("{:x}", Sha256::digest(V10_MODEL_BYTES));
    if found != V10_MODEL_SHA256 {
        return Err(V10StateError::AuthorityDigest { found });
    }
    let value: serde_json::Value = serde_json::from_slice(V10_MODEL_BYTES)
        .map_err(|error| V10StateError::Schema(error.to_string()))?;
    if value["model_version"].as_str() != Some(V10_MODEL_VERSION)
        || value["base_model_definition_sha256"].as_str() != Some(V9_MODEL_SHA256)
    {
        return Err(V10StateError::AuthoritySchema);
    }
    Ok(crate::ModelDefinition {
        version: V10_MODEL_VERSION,
        sha256: V10_MODEL_SHA256.into(),
        bytes: V10_MODEL_BYTES,
    })
}

/// Nominal V10 state. Its physical payload is imported bit-identically from
/// V9 and is accepted only under V10 configuration/state receipts.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
pub struct V10CoupledOwnedState(pub crate::V8CoupledOwnedState);

impl V10CoupledOwnedState {
    pub fn validate(&self, config: &VegetationConfiguration) -> Result<(), V10StateError> {
        config
            .validate_v10()
            .map_err(V10StateError::Configuration)?;
        if self.0.model_definition_sha256 != V10_MODEL_SHA256
            || self.0.configuration_sha256 != config.configuration_sha256
        {
            return Err(V10StateError::Identity);
        }
        if self.0.state_sha256 != self.0.canonical_sha256() {
            return Err(V10StateError::Digest);
        }
        if self.0.last_transaction_id == 0 && config.initial_state_sha256 != self.0.state_sha256 {
            return Err(V10StateError::InitialStateReceipt);
        }
        let (v9_config, v9_state) = project_v10_runtime_to_v9_unchecked(config, self)?;
        V9CoupledOwnedState(v9_state)
            .validate(&v9_config)
            .map_err(V10StateError::ImportedV9Payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V9ToV10Migration {
    pub configuration: VegetationConfiguration,
    pub state: V10CoupledOwnedState,
}

pub fn migrate_v9_runtime(
    source_configuration: &VegetationConfiguration,
    source_state: &V9CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
) -> Result<V9ToV10Migration, V10MigrationError> {
    source_state
        .validate(source_configuration)
        .map_err(|_| V10MigrationError::InvalidSource)?;
    target_configuration
        .validate_v10()
        .map_err(|_| V10MigrationError::InvalidTarget)?;
    if !payload_matches(source_configuration, target_configuration)
        .map_err(|_| V10MigrationError::PayloadMismatch)?
    {
        return Err(V10MigrationError::PayloadMismatch);
    }
    let mut state = source_state.0.clone();
    state.model_definition_sha256 = V10_MODEL_SHA256.into();
    state
        .configuration_sha256
        .clone_from(&target_configuration.configuration_sha256);
    state.state_sha256 = state.canonical_sha256();
    let state = V10CoupledOwnedState(state);
    state
        .validate(target_configuration)
        .map_err(|_| V10MigrationError::TargetRejected)?;
    Ok(V9ToV10Migration {
        configuration: target_configuration.clone(),
        state,
    })
}

pub fn project_v10_runtime_to_v9(
    configuration: &VegetationConfiguration,
    state: &V10CoupledOwnedState,
) -> Result<(VegetationConfiguration, V9CoupledOwnedState), V10StateError> {
    state.validate(configuration)?;
    let (configuration, state) = project_v10_runtime_to_v9_unchecked(configuration, state)?;
    Ok((configuration, V9CoupledOwnedState(state)))
}

pub fn project_v9_runtime_to_v10(
    state: &V9CoupledOwnedState,
    configuration: &VegetationConfiguration,
) -> Result<V10CoupledOwnedState, V10StateError> {
    configuration
        .validate_v10()
        .map_err(V10StateError::Configuration)?;
    let mut source = configuration.clone();
    source.model_definition_sha256 = V9_MODEL_SHA256.into();
    source.configuration_sha256 = source
        .canonical_sha256()
        .map_err(V10StateError::Configuration)?;
    if state.0.last_transaction_id == 0 {
        source
            .initial_state_sha256
            .clone_from(&state.0.state_sha256);
    }
    state
        .validate(&source)
        .map_err(V10StateError::ImportedV9Payload)?;
    let mut projected = state.0.clone();
    projected.model_definition_sha256 = V10_MODEL_SHA256.into();
    projected
        .configuration_sha256
        .clone_from(&configuration.configuration_sha256);
    projected.state_sha256 = projected.canonical_sha256();
    if projected.last_transaction_id == 0
        && configuration.initial_state_sha256 != projected.state_sha256
    {
        return Err(V10StateError::InitialStateReceipt);
    }
    // The target configuration and the exact reconstructed V9 source were
    // fully validated above. Every V10 identity field and the canonical state
    // digest are derived here from those immutable values, so calling public
    // `validate` would immediately reverse the same payload to V9 and repeat
    // both validations without crossing a mutation or trust boundary.
    let projected = V10CoupledOwnedState(projected);
    Ok(projected)
}

pub(crate) fn project_v10_runtime_to_v9_unchecked(
    configuration: &VegetationConfiguration,
    state: &V10CoupledOwnedState,
) -> Result<(VegetationConfiguration, crate::V8CoupledOwnedState), V10StateError> {
    let mut config = configuration.clone();
    config.model_definition_sha256 = V9_MODEL_SHA256.into();
    config.configuration_sha256 = config
        .canonical_sha256()
        .map_err(V10StateError::Configuration)?;
    let mut projected = state.0.clone();
    projected.model_definition_sha256 = V9_MODEL_SHA256.into();
    projected
        .configuration_sha256
        .clone_from(&config.configuration_sha256);
    projected.state_sha256 = projected.canonical_sha256();
    if projected.last_transaction_id == 0 {
        config
            .initial_state_sha256
            .clone_from(&projected.state_sha256);
    }
    Ok((config, projected))
}

fn payload_matches(
    source: &VegetationConfiguration,
    target: &VegetationConfiguration,
) -> Result<bool, serde_json::Error> {
    let mut source = source.clone();
    let mut target = target.clone();
    for value in [&mut source, &mut target] {
        value.model_definition_sha256.clear();
        value.configuration_sha256.clear();
        value.initial_state_sha256.clear();
    }
    Ok(serde_json::to_vec(&source)? == serde_json::to_vec(&target)?)
}

#[derive(Debug, Error, PartialEq)]
pub enum V10MigrationError {
    #[error("VEG-E-120: invalid V9 migration source")]
    InvalidSource,
    #[error("VEG-E-120: invalid V10 migration target")]
    InvalidTarget,
    #[error("VEG-E-120: nonidentity migration payload")]
    PayloadMismatch,
    #[error("VEG-E-120: constructed V10 state rejected")]
    TargetRejected,
}

#[derive(Debug, Error, PartialEq)]
pub enum V10StateError {
    #[error("VEG-E-120: V10 authority digest mismatch: {found}")]
    AuthorityDigest { found: String },
    #[error("VEG-E-120: invalid V10 authority schema")]
    AuthoritySchema,
    #[error("VEG-E-120: invalid V10 state schema: {0}")]
    Schema(String),
    #[error("VEG-E-120: invalid V10 configuration: {0}")]
    Configuration(VegetationError),
    #[error("VEG-E-120: V10 identity mismatch")]
    Identity,
    #[error("VEG-E-120: V10 state digest mismatch")]
    Digest,
    #[error("VEG-E-120: V10 initial-state receipt mismatch")]
    InitialStateReceipt,
    #[error("VEG-E-120: imported V9 payload rejected: {0}")]
    ImportedV9Payload(V9StateError),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (
        VegetationConfiguration,
        V9CoupledOwnedState,
        VegetationConfiguration,
        V10CoupledOwnedState,
    ) {
        let (v8_config, v8_state) = crate::v8_state::v8_test_fixture();
        let mut v9_config = v8_config.clone();
        v9_config.model_definition_sha256 = V9_MODEL_SHA256.into();
        v9_config.configuration_sha256 = v9_config.canonical_sha256().expect("V9 config");
        let mut v9_payload = v8_state.clone();
        v9_payload.model_definition_sha256 = V9_MODEL_SHA256.into();
        v9_payload
            .configuration_sha256
            .clone_from(&v9_config.configuration_sha256);
        v9_payload.state_sha256 = v9_payload.canonical_sha256();
        v9_config
            .initial_state_sha256
            .clone_from(&v9_payload.state_sha256);
        let v9_state = V9CoupledOwnedState(v9_payload);
        v9_state.validate(&v9_config).expect("valid V9 fixture");

        let mut v10_config = v9_config.clone();
        v10_config.model_definition_sha256 = V10_MODEL_SHA256.into();
        v10_config.configuration_sha256 = v10_config.canonical_sha256().expect("V10 config");
        let mut v10_payload = v9_state.0.clone();
        v10_payload.model_definition_sha256 = V10_MODEL_SHA256.into();
        v10_payload
            .configuration_sha256
            .clone_from(&v10_config.configuration_sha256);
        v10_payload.state_sha256 = v10_payload.canonical_sha256();
        v10_config
            .initial_state_sha256
            .clone_from(&v10_payload.state_sha256);
        let migration =
            migrate_v9_runtime(&v9_config, &v9_state, &v10_config).expect("V9-to-V10 migration");
        (v9_config, v9_state, v10_config, migration.state)
    }

    #[test]
    fn definition_and_identity_migration_are_exactly_bound() {
        assert_eq!(
            load_v10_model_definition().expect("V10 definition").sha256,
            V10_MODEL_SHA256
        );
        let (v9_config, v9_state, v10_config, v10_state) = fixture();
        v10_state.validate(&v10_config).expect("valid V10 state");
        assert_eq!(v10_state.0.occupancies, v9_state.0.occupancies);
        assert_eq!(v10_state.0.strata, v9_state.0.strata);
        assert!(v10_state.validate(&v9_config).is_err());
        assert!(v9_state.validate(&v10_config).is_err());
    }

    #[test]
    fn migration_rejects_one_bit_payload_change() {
        let (v9_config, v9_state, mut v10_config, _) = fixture();
        v10_config.strata[0].leaf_dimension_m =
            f64::from_bits(v10_config.strata[0].leaf_dimension_m.to_bits() ^ 1);
        v10_config.configuration_sha256 = v10_config.canonical_sha256().expect("poison digest");
        assert_eq!(
            migrate_v9_runtime(&v9_config, &v9_state, &v10_config),
            Err(V10MigrationError::PayloadMismatch)
        );
    }
}
