//! Explicit V8-to-V9 successor identity and offline migration.
//!
//! V9 imports V8 state and runtime physics exactly. Its distinct identity binds
//! the reproducible non-Rust oracle environment and canonical vector bytes.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::v8_state::{V8_MODEL_SHA256, V8CoupledOwnedState, V8StateError};
use crate::{VegetationConfiguration, VegetationError};

pub const V9_MODEL_VERSION: &str = "OPENWEPP_C3_WOODY_V9";
pub const V9_MODEL_SHA256: &str =
    "f388aa883631d935e89368d8ca6e0275db4f6c00292ff0a6adf1936d7b71bcd0";
pub const V9_VECTOR_SHA256: &str =
    "f86770cce11235ba282b47e81de2fa5dc9af19c29dc3bd91c62256957c590633";
pub const V9_RUNTIME_DESCRIPTOR_SHA256: &str =
    "e0d05e49eabe43340e9fc7e251b319bcd08305d59af522298001b3c4f6bf951f";
pub const V9_MODEL_BYTES: &[u8] =
    include_bytes!("../model-registry/openwepp_c3_woody_v9_definition.json");
pub const V9_VECTOR_BYTES: &[u8] = include_bytes!(
    "../../../docs/work-packages/20260817-c3-woody-v3-v5-oracle-reconciliation-001/artifacts/v9/openwepp_c3_woody_v9_vectors.json"
);
pub const V9_RUNTIME_DESCRIPTOR_BYTES: &[u8] = include_bytes!(
    "../../../docs/work-packages/20260817-c3-woody-v3-v5-oracle-reconciliation-001/artifacts/v9/runtime_descriptor.json"
);

pub fn load_v9_model_definition() -> Result<crate::ModelDefinition, V9AuthorityError> {
    require_bound_bytes(V9_MODEL_BYTES, V9_MODEL_SHA256, "V9 model definition")?;
    require_bound_bytes(V9_VECTOR_BYTES, V9_VECTOR_SHA256, "V9 vectors")?;
    require_bound_bytes(
        V9_RUNTIME_DESCRIPTOR_BYTES,
        V9_RUNTIME_DESCRIPTOR_SHA256,
        "V9 runtime descriptor",
    )?;
    let value: serde_json::Value = serde_json::from_slice(V9_MODEL_BYTES)
        .map_err(|error| V9AuthorityError::Schema(error.to_string()))?;
    if value
        .get("model_version")
        .and_then(serde_json::Value::as_str)
        != Some(V9_MODEL_VERSION)
        || value
            .get("base_model_definition_sha256")
            .and_then(serde_json::Value::as_str)
            != Some(V8_MODEL_SHA256)
        || value["oracle_identity"]["runtime_descriptor_sha256"].as_str()
            != Some(V9_RUNTIME_DESCRIPTOR_SHA256)
    {
        return Err(V9AuthorityError::Schema(
            "invalid V9 successor bindings".into(),
        ));
    }
    Ok(crate::ModelDefinition {
        version: V9_MODEL_VERSION,
        sha256: V9_MODEL_SHA256.into(),
        bytes: V9_MODEL_BYTES,
    })
}

fn require_bound_bytes(
    bytes: &[u8],
    expected: &str,
    label: &'static str,
) -> Result<(), V9AuthorityError> {
    let found = format!("{:x}", Sha256::digest(bytes));
    if found != expected {
        return Err(V9AuthorityError::DigestMismatch {
            surface: label,
            expected: expected.into(),
            found,
        });
    }
    Ok(())
}

#[derive(Debug, Error, PartialEq)]
pub enum V9AuthorityError {
    #[error("VEG-E-115: invalid V9 authority schema: {0}")]
    Schema(String),
    #[error("VEG-E-115: V9 {surface} digest mismatch: expected {expected}, found {found}")]
    DigestMismatch {
        surface: &'static str,
        expected: String,
        found: String,
    },
}

/// A nominal V9 state. The transparent payload is the exact V8 state schema,
/// but it is accepted only with V9 model, configuration, and state receipts.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
pub struct V9CoupledOwnedState(pub V8CoupledOwnedState);

impl V9CoupledOwnedState {
    pub fn parse_strict(
        bytes: &[u8],
        config: &VegetationConfiguration,
    ) -> Result<Self, V9StateError> {
        let state: Self = serde_json::from_slice(bytes)
            .map_err(|error| V9StateError::Schema(error.to_string()))?;
        state.validate(config)?;
        Ok(state)
    }

    pub fn validate(&self, config: &VegetationConfiguration) -> Result<(), V9StateError> {
        config.validate_v9().map_err(V9StateError::Configuration)?;
        if self.0.model_definition_sha256 != V9_MODEL_SHA256
            || self.0.configuration_sha256 != config.configuration_sha256
        {
            return Err(V9StateError::Identity);
        }
        if self.0.state_sha256 != self.0.canonical_sha256() {
            return Err(V9StateError::Digest);
        }
        if self.0.last_transaction_id == 0 && config.initial_state_sha256 != self.0.state_sha256 {
            return Err(V9StateError::InitialStateReceipt);
        }

        // Validate the imported V8 payload under a local identity-rebound
        // shadow. This does not alias or mutate either historical identity.
        let mut shadow_config = config.clone();
        shadow_config.model_definition_sha256 = V8_MODEL_SHA256.into();
        shadow_config.configuration_sha256 = shadow_config
            .canonical_sha256()
            .map_err(V9StateError::Configuration)?;
        let mut shadow = self.0.clone();
        shadow.model_definition_sha256 = V8_MODEL_SHA256.into();
        shadow
            .configuration_sha256
            .clone_from(&shadow_config.configuration_sha256);
        shadow.state_sha256 = shadow.canonical_sha256();
        if shadow.last_transaction_id == 0 {
            shadow_config
                .initial_state_sha256
                .clone_from(&shadow.state_sha256);
        }
        shadow
            .validate(&shadow_config)
            .map_err(V9StateError::ImportedV8Payload)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V8ToV9Migration {
    pub configuration: VegetationConfiguration,
    pub initial_state: V9CoupledOwnedState,
    pub state: V9CoupledOwnedState,
}

#[derive(Debug, Error, PartialEq)]
pub enum V8ToV9MigrationError {
    #[error("VEG-E-116: invalid V8 source configuration")]
    InvalidSourceConfiguration,
    #[error("VEG-E-116: invalid V8 source initial state")]
    InvalidSourceInitialState,
    #[error("VEG-E-116: invalid V8 source current state")]
    InvalidSourceState,
    #[error("VEG-E-116: invalid V9 target configuration")]
    InvalidTargetConfiguration,
    #[error("VEG-E-116: V8/V9 non-identity configuration payload differs")]
    ConfigurationPayloadMismatch,
    #[error("VEG-E-116: V9 initial-state receipt mismatch")]
    InitialStateReceiptMismatch,
    #[error("VEG-E-116: constructed V9 state rejected")]
    TargetStateRejected,
}

pub fn migrate_v8_snapshot(
    source_configuration: &VegetationConfiguration,
    source_initial_state: &V8CoupledOwnedState,
    source_state: &V8CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
) -> Result<V8ToV9Migration, V8ToV9MigrationError> {
    source_configuration
        .validate_v8()
        .map_err(|_| V8ToV9MigrationError::InvalidSourceConfiguration)?;
    if source_initial_state.last_transaction_id != 0 {
        return Err(V8ToV9MigrationError::InvalidSourceInitialState);
    }
    source_initial_state
        .validate(source_configuration)
        .map_err(|_| V8ToV9MigrationError::InvalidSourceInitialState)?;
    source_state
        .validate(source_configuration)
        .map_err(|_| V8ToV9MigrationError::InvalidSourceState)?;
    target_configuration
        .validate_v9()
        .map_err(|_| V8ToV9MigrationError::InvalidTargetConfiguration)?;
    if !configuration_payload_matches(source_configuration, target_configuration)
        .map_err(|_| V8ToV9MigrationError::ConfigurationPayloadMismatch)?
    {
        return Err(V8ToV9MigrationError::ConfigurationPayloadMismatch);
    }
    let initial_state = rebind_state(source_initial_state, target_configuration);
    if target_configuration.initial_state_sha256 != initial_state.0.state_sha256 {
        return Err(V8ToV9MigrationError::InitialStateReceiptMismatch);
    }
    let state = rebind_state(source_state, target_configuration);
    initial_state
        .validate(target_configuration)
        .map_err(|_| V8ToV9MigrationError::TargetStateRejected)?;
    state
        .validate(target_configuration)
        .map_err(|_| V8ToV9MigrationError::TargetStateRejected)?;
    Ok(V8ToV9Migration {
        configuration: target_configuration.clone(),
        initial_state,
        state,
    })
}

/// Project a validated V9 runtime state onto the exact imported V8 physical
/// payload. The returned configuration and state are transient adapter values;
/// they do not alias or mutate either model identity.
pub fn project_v9_runtime_to_v8(
    configuration: &VegetationConfiguration,
    state: &V9CoupledOwnedState,
) -> Result<(VegetationConfiguration, V8CoupledOwnedState), V9StateError> {
    state.validate(configuration)?;
    let mut projected_configuration = configuration.clone();
    projected_configuration.model_definition_sha256 = V8_MODEL_SHA256.into();
    projected_configuration.configuration_sha256 = projected_configuration
        .canonical_sha256()
        .map_err(V9StateError::Configuration)?;
    let mut projected_state = state.0.clone();
    projected_state.model_definition_sha256 = V8_MODEL_SHA256.into();
    projected_state
        .configuration_sha256
        .clone_from(&projected_configuration.configuration_sha256);
    projected_state.state_sha256 = projected_state.canonical_sha256();
    if projected_state.last_transaction_id == 0 {
        projected_configuration
            .initial_state_sha256
            .clone_from(&projected_state.state_sha256);
    }
    projected_state
        .validate(&projected_configuration)
        .map_err(V9StateError::ImportedV8Payload)?;
    Ok((projected_configuration, projected_state))
}

/// Rebind an accepted transient V8 runtime state to the prospective V9
/// identity after proving the imported payload against the exact V9
/// configuration. No physical field is transformed.
pub fn project_v8_runtime_to_v9(
    state: &V8CoupledOwnedState,
    configuration: &VegetationConfiguration,
) -> Result<V9CoupledOwnedState, V9StateError> {
    configuration
        .validate_v9()
        .map_err(V9StateError::Configuration)?;
    let mut source_configuration = configuration.clone();
    source_configuration.model_definition_sha256 = V8_MODEL_SHA256.into();
    source_configuration.configuration_sha256 = source_configuration
        .canonical_sha256()
        .map_err(V9StateError::Configuration)?;
    state
        .validate(&source_configuration)
        .map_err(V9StateError::ImportedV8Payload)?;
    let mut projected = state.clone();
    projected.model_definition_sha256 = V9_MODEL_SHA256.into();
    projected
        .configuration_sha256
        .clone_from(&configuration.configuration_sha256);
    projected.state_sha256 = projected.canonical_sha256();
    let projected = V9CoupledOwnedState(projected);
    projected.validate(configuration)?;
    Ok(projected)
}

fn rebind_state(
    source: &V8CoupledOwnedState,
    target: &VegetationConfiguration,
) -> V9CoupledOwnedState {
    let mut state = source.clone();
    state.model_definition_sha256 = V9_MODEL_SHA256.into();
    state
        .configuration_sha256
        .clone_from(&target.configuration_sha256);
    state.state_sha256 = state.canonical_sha256();
    V9CoupledOwnedState(state)
}

fn configuration_payload_matches(
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
pub enum V9StateError {
    #[error("VEG-E-SCHEMA-001: invalid V9 state: {0}")]
    Schema(String),
    #[error("VEG-E-116: V9 model/configuration identity mismatch")]
    Identity,
    #[error("VEG-E-116: V9 state digest mismatch")]
    Digest,
    #[error("VEG-E-116: V9 initial-state receipt mismatch")]
    InitialStateReceipt,
    #[error("VEG-E-116: invalid V9 configuration: {0}")]
    Configuration(VegetationError),
    #[error("VEG-E-116: imported V8 payload rejected: {0}")]
    ImportedV8Payload(V8StateError),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn migrated_fixture() -> (
        VegetationConfiguration,
        V8CoupledOwnedState,
        V8ToV9Migration,
    ) {
        let (source_configuration, source_state) = crate::v8_state::v8_test_fixture();
        let mut target = source_configuration.clone();
        target.model_definition_sha256 = V9_MODEL_SHA256.into();
        target.configuration_sha256 = target.canonical_sha256().expect("V9 config digest");
        target.initial_state_sha256 = "0".repeat(64);
        target.initial_state_sha256 = rebind_state(&source_state, &target).0.state_sha256;
        let migration =
            migrate_v8_snapshot(&source_configuration, &source_state, &source_state, &target)
                .expect("exact V8-to-V9 migration");
        (source_configuration, source_state, migration)
    }

    #[test]
    fn definition_vectors_and_runtime_descriptor_are_exactly_bound() {
        let definition = load_v9_model_definition().expect("V9 authority bytes");
        assert_eq!(definition.version, V9_MODEL_VERSION);
        assert_eq!(definition.sha256, V9_MODEL_SHA256);
        assert_eq!(
            format!("{:x}", Sha256::digest(V9_VECTOR_BYTES)),
            V9_VECTOR_SHA256
        );
        assert_eq!(
            format!("{:x}", Sha256::digest(V9_RUNTIME_DESCRIPTOR_BYTES)),
            V9_RUNTIME_DESCRIPTOR_SHA256
        );
    }

    #[test]
    fn migration_preserves_payload_bits_and_rebinds_only_identity_receipts() {
        let (source_configuration, source, migration) = migrated_fixture();
        let target = &migration.configuration;
        migration.state.validate(target).expect("strict V9 state");
        assert_eq!(migration.state.0.occupancies, source.occupancies);
        assert_eq!(migration.state.0.strata, source.strata);
        assert_eq!(migration.state.0.tile_canopy_air, source.tile_canopy_air);
        assert_eq!(
            migration.state.0.last_transaction_id,
            source.last_transaction_id
        );
        assert_eq!(
            source_configuration.model_definition_sha256,
            V8_MODEL_SHA256
        );
        assert_eq!(source.model_definition_sha256, V8_MODEL_SHA256);
        assert_eq!(target.model_definition_sha256, V9_MODEL_SHA256);
        assert_eq!(migration.state.0.model_definition_sha256, V9_MODEL_SHA256);
        assert_ne!(source.configuration_sha256, target.configuration_sha256);
        assert_ne!(source.state_sha256, migration.state.0.state_sha256);
    }

    #[test]
    fn historical_and_successor_identities_are_not_aliases() {
        let (source_configuration, source, migration) = migrated_fixture();
        assert!(source_configuration.validate_v9().is_err());
        assert!(migration.configuration.validate_v8().is_err());
        assert!(
            V9CoupledOwnedState(source.clone())
                .validate(&source_configuration)
                .is_err()
        );
        assert!(
            migration
                .state
                .0
                .validate(&migration.configuration)
                .is_err()
        );
    }

    #[test]
    fn migration_rejects_nonidentity_payload_change() {
        let (source_configuration, source, migration) = migrated_fixture();
        let mut target = migration.configuration;
        target.strata[0].leaf_dimension_m =
            f64::from_bits(target.strata[0].leaf_dimension_m.to_bits() ^ 1);
        target.configuration_sha256 = target.canonical_sha256().expect("poison config digest");
        assert_eq!(
            migrate_v8_snapshot(&source_configuration, &source, &source, &target),
            Err(V8ToV9MigrationError::ConfigurationPayloadMismatch)
        );
    }

    #[test]
    fn migration_rejects_signed_zero_payload_poison() {
        let (mut source_configuration, mut source, _) = migrated_fixture();
        source_configuration.strata[0].atkin_intercept = 0.0;
        source_configuration.configuration_sha256 = source_configuration
            .canonical_sha256()
            .expect("source poison config digest");
        source
            .configuration_sha256
            .clone_from(&source_configuration.configuration_sha256);
        source.state_sha256 = source.canonical_sha256();
        source_configuration
            .initial_state_sha256
            .clone_from(&source.state_sha256);

        let mut target = source_configuration.clone();
        target.strata[0].atkin_intercept = -0.0;
        target.model_definition_sha256 = V9_MODEL_SHA256.into();
        target.configuration_sha256 = target.canonical_sha256().expect("target poison digest");
        target.initial_state_sha256 = "0".repeat(64);
        target.initial_state_sha256 = rebind_state(&source, &target).0.state_sha256;

        assert_ne!(0.0_f64.to_bits(), (-0.0_f64).to_bits());
        assert_eq!(
            migrate_v8_snapshot(&source_configuration, &source, &source, &target),
            Err(V8ToV9MigrationError::ConfigurationPayloadMismatch)
        );
    }

    #[test]
    fn migration_rejects_advanced_state_as_initial_snapshot() {
        let (source_configuration, source, migration) = migrated_fixture();
        let mut advanced = source.clone();
        advanced.last_transaction_id = 9;
        for shared in advanced.strata.values_mut() {
            shared.last_transaction_id = 9;
        }
        for lane in advanced.occupancies.values_mut() {
            lane.last_accepted_transaction_id = Some(9);
        }
        advanced.state_sha256 = advanced.canonical_sha256();
        advanced
            .validate(&source_configuration)
            .expect("valid advanced V8 state");
        assert_eq!(
            migrate_v8_snapshot(
                &source_configuration,
                &advanced,
                &advanced,
                &migration.configuration,
            ),
            Err(V8ToV9MigrationError::InvalidSourceInitialState)
        );
    }

    #[test]
    fn v9_authority_and_migration_errors_use_contract_guard_ids() {
        assert!(
            V9AuthorityError::Schema("poison".into())
                .to_string()
                .starts_with("VEG-E-115:")
        );
        assert!(V9StateError::Identity.to_string().starts_with("VEG-E-116:"));
        assert!(
            V8ToV9MigrationError::ConfigurationPayloadMismatch
                .to_string()
                .starts_with("VEG-E-116:")
        );
    }

    #[test]
    fn runtime_rebinding_rejects_wrong_v8_identity_receipts() {
        let (_, source, migration) = migrated_fixture();
        let target = migration.configuration;
        let mut wrong_model = source;
        wrong_model.model_definition_sha256 = V9_MODEL_SHA256.into();
        assert!(matches!(
            project_v8_runtime_to_v9(&wrong_model, &target),
            Err(V9StateError::ImportedV8Payload(_))
        ));
        let (source_configuration, mut wrong_configuration, _) = migrated_fixture();
        wrong_configuration.configuration_sha256 = "f".repeat(64);
        assert!(matches!(
            project_v8_runtime_to_v9(&wrong_configuration, &target),
            Err(V9StateError::ImportedV8Payload(_))
        ));
        let mut wrong_state = wrong_configuration;
        wrong_state.configuration_sha256 = source_configuration.configuration_sha256;
        wrong_state.state_sha256 = "e".repeat(64);
        assert!(matches!(
            project_v8_runtime_to_v9(&wrong_state, &target),
            Err(V9StateError::ImportedV8Payload(_))
        ));
    }
}
