//! Immutable LSE V3 identity, migration and state-candidate helpers.

#![allow(clippy::missing_errors_doc)]

use std::collections::BTreeSet;

use openwepp_kernel_contract::TransactionId;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyError, LandSurfaceEnergyState,
    LandSurfaceEnergyV2State, OfeId, Sha256Digest, SurfaceConfiguration, SurfaceHeatStorageMode,
    V3_MODEL_DEFINITION_SHA256, V3_MODEL_VERSION, canonical_digest,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
pub struct LandSurfaceEnergyV3State(pub LandSurfaceEnergyState);

impl LandSurfaceEnergyV3State {
    /// Validate V3 identity and the complete imported V2 scientific payload.
    pub fn validate(
        &self,
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<(), LseV3StateError> {
        configuration
            .validate_v3()
            .map_err(LseV3StateError::Configuration)?;
        if self.0.model_definition_sha256.as_str() != V3_MODEL_DEFINITION_SHA256
            || self.0.configuration_sha256 != configuration.configuration_sha256
            || self.0.owner_id != configuration.owner_id
        {
            return Err(LseV3StateError::Identity);
        }
        let expected: BTreeSet<_> = configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                ofe.tiles
                    .iter()
                    .map(move |tile| (ofe.ofe_id.clone(), tile.tile_id.clone()))
            })
            .collect();
        let actual: BTreeSet<_> = self
            .0
            .tiles
            .iter()
            .map(|tile| (tile.ofe_id.clone(), tile.tile_id.clone()))
            .collect();
        if actual != expected || actual.len() != self.0.tiles.len() {
            return Err(LseV3StateError::Topology);
        }
        for tile in &self.0.tiles {
            if !tile.surface_enthalpy_j_m2_tile_ground.is_finite()
                || !tile.surface_temperature_warm_start_k.is_finite()
                || !(200.0..=350.0).contains(&tile.surface_temperature_warm_start_k)
            {
                return Err(LseV3StateError::Domain);
            }
            let configured = configuration
                .ofes
                .iter()
                .find(|ofe| ofe.ofe_id == tile.ofe_id)
                .and_then(|ofe| ofe.tiles.iter().find(|item| item.tile_id == tile.tile_id))
                .ok_or(LseV3StateError::Topology)?;
            if configured.surface_heat_storage_mode == SurfaceHeatStorageMode::EquilibriumZero
                && tile.surface_enthalpy_j_m2_tile_ground != 0.0
            {
                return Err(LseV3StateError::Domain);
            }
            if matches!(
                configured.surface,
                SurfaceConfiguration::ForestLitter { .. }
            ) && configured.surface_heat_storage_mode == SurfaceHeatStorageMode::EquilibriumZero
            {
                return Err(LseV3StateError::Domain);
            }
        }
        if self.0.state_sha256
            != self
                .canonical_sha256()
                .map_err(LseV3StateError::Configuration)?
        {
            return Err(LseV3StateError::Identity);
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<Sha256Digest, LandSurfaceEnergyError> {
        let mut value = serde_json::to_value(&self.0)
            .map_err(|error| LandSurfaceEnergyError::MalformedSerialization(error.to_string()))?;
        let digest =
            value
                .get_mut("state_sha256")
                .ok_or(LandSurfaceEnergyError::MalformedSerialization(
                    "state_sha256 absent from serialized V3 state".into(),
                ))?;
        *digest = serde_json::Value::String(String::new());
        canonical_digest(&value)
    }

    pub fn from_json(
        bytes: &[u8],
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<Self, LseV3StateError> {
        let state: Self = serde_json::from_slice(bytes)
            .map_err(|error| LseV3StateError::Serialization(error.to_string()))?;
        state.validate(configuration)?;
        Ok(state)
    }

    pub fn to_json(&self) -> Result<Vec<u8>, LseV3StateError> {
        serde_json::to_vec(self).map_err(|error| LseV3StateError::Serialization(error.to_string()))
    }
}

/// Rebind the complete V2 configuration payload to the immutable V3 identity.
pub fn migrate_v2_configuration_to_v3(
    source: &LandSurfaceEnergyConfiguration,
) -> Result<LandSurfaceEnergyConfiguration, LseV3StateError> {
    source
        .validate_v2()
        .map_err(LseV3StateError::Configuration)?;
    let mut target = source.clone();
    target.model_version = V3_MODEL_VERSION.into();
    target.model_definition_sha256 = digest(V3_MODEL_DEFINITION_SHA256)?;
    target.configuration_sha256 = target
        .canonical_sha256()
        .map_err(LseV3StateError::Configuration)?;
    target
        .validate_v3()
        .map_err(LseV3StateError::Configuration)?;
    Ok(target)
}

/// Checked one-way V2-to-V3 state migration. No reverse production API is
/// provided, so a V3 state can never silently fall back to V2.
pub fn migrate_v2_state_to_v3(
    source_configuration: &LandSurfaceEnergyConfiguration,
    source: &LandSurfaceEnergyV2State,
    target_configuration: &LandSurfaceEnergyConfiguration,
) -> Result<LandSurfaceEnergyV3State, LseV3StateError> {
    source
        .validate(source_configuration)
        .map_err(|_| LseV3StateError::ImportedV2)?;
    target_configuration
        .validate_v3()
        .map_err(LseV3StateError::Configuration)?;
    let expected_configuration = migrate_v2_configuration_to_v3(source_configuration)?;
    if expected_configuration != *target_configuration {
        return Err(LseV3StateError::PayloadMismatch);
    }
    let mut state = source.0.clone();
    state.model_definition_sha256 = digest(V3_MODEL_DEFINITION_SHA256)?;
    state
        .configuration_sha256
        .clone_from(&target_configuration.configuration_sha256);
    let mut target = LandSurfaceEnergyV3State(state);
    target.0.state_sha256 = target
        .canonical_sha256()
        .map_err(LseV3StateError::Configuration)?;
    target.validate(target_configuration)?;
    Ok(target)
}

#[derive(Clone, Debug, PartialEq)]
pub struct V3TilePhaseUpdate {
    pub ofe_id: OfeId,
    pub tile_id: openwepp_kernel_contract::TileId,
    pub ending_sensible_energy_j_m2_tile: f64,
    pub ending_temperature_k: f64,
}

/// Construct a complete candidate without mutating the beginning state. The
/// caller commits it only after surface-owner and receipt joins pass.
pub fn build_v3_ending_state(
    beginning: &LandSurfaceEnergyV3State,
    configuration: &LandSurfaceEnergyConfiguration,
    transaction_id: TransactionId,
    updates: &[V3TilePhaseUpdate],
) -> Result<LandSurfaceEnergyV3State, LseV3StateError> {
    beginning.validate(configuration)?;
    if transaction_id.0 == 0
        || beginning
            .0
            .last_accepted_transaction_id
            .is_some_and(|prior| prior.0 >= transaction_id.0)
    {
        return Err(LseV3StateError::Lineage);
    }
    let mut seen = BTreeSet::new();
    let mut tiles = beginning.0.tiles.clone();
    for update in updates {
        if !seen.insert((update.ofe_id.clone(), update.tile_id.clone()))
            || !update.ending_sensible_energy_j_m2_tile.is_finite()
            || !update.ending_temperature_k.is_finite()
            || !(200.0..=350.0).contains(&update.ending_temperature_k)
        {
            return Err(LseV3StateError::Domain);
        }
        let tile = tiles
            .iter_mut()
            .find(|tile| tile.ofe_id == update.ofe_id && tile.tile_id == update.tile_id)
            .ok_or(LseV3StateError::Topology)?;
        tile.surface_enthalpy_j_m2_tile_ground = update.ending_sensible_energy_j_m2_tile;
        tile.surface_temperature_warm_start_k = update.ending_temperature_k;
    }
    let mut ending = LandSurfaceEnergyV3State(LandSurfaceEnergyState {
        model_definition_sha256: beginning.0.model_definition_sha256.clone(),
        configuration_sha256: beginning.0.configuration_sha256.clone(),
        state_sha256: digest(&"0".repeat(64))?,
        owner_id: beginning.0.owner_id.clone(),
        last_accepted_transaction_id: Some(transaction_id),
        tiles,
    });
    ending.0.state_sha256 = ending
        .canonical_sha256()
        .map_err(LseV3StateError::Configuration)?;
    ending.validate(configuration)?;
    Ok(ending)
}

fn digest(value: &str) -> Result<Sha256Digest, LseV3StateError> {
    Sha256Digest::try_new(value).map_err(LseV3StateError::Configuration)
}

#[derive(Debug, Error, PartialEq)]
pub enum LseV3StateError {
    #[error("LSEB-E-045: invalid LSE-V3 configuration/state: {0}")]
    Configuration(LandSurfaceEnergyError),
    #[error("LSEB-E-045: invalid LSE-V3 identity")]
    Identity,
    #[error("LSEB-E-045: imported LSE-V2 state rejected")]
    ImportedV2,
    #[error("LSEB-E-045: V2/V3 scientific payload mismatch")]
    PayloadMismatch,
    #[error("LSEB-E-045: invalid V3 tile topology")]
    Topology,
    #[error("LSEB-E-045: invalid V3 state domain")]
    Domain,
    #[error("LSEB-E-048: invalid V3 transaction lineage")]
    Lineage,
    #[error("LSEB-E-045: V3 serialization failed: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        V2_MODEL_DEFINITION_SHA256, V2_MODEL_VERSION, V2_VEGETATION_MODEL_DEFINITION_SHA256,
        V2_VEGETATION_MODEL_VERSION,
    };

    fn v1_runtime() -> (LandSurfaceEnergyConfiguration, LandSurfaceEnergyState) {
        let vectors: serde_json::Value = serde_json::from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/openwepp_snow_free_lse_v1_vectors.json"
        )))
        .expect("frozen vectors");
        let instances = &vectors["strict_schema_instances"];
        let configuration: LandSurfaceEnergyConfiguration =
            serde_json::from_value(instances["configuration"].clone()).expect("configuration");
        let mut state = LandSurfaceEnergyState {
            model_definition_sha256: Sha256Digest::try_new(crate::MODEL_DEFINITION_SHA256)
                .expect("V1 digest"),
            configuration_sha256: configuration.configuration_sha256.clone(),
            state_sha256: Sha256Digest::try_new("0".repeat(64)).expect("placeholder"),
            owner_id: configuration.owner_id.clone(),
            last_accepted_transaction_id: Some(TransactionId(1)),
            tiles: configuration
                .ofes
                .iter()
                .flat_map(|ofe| {
                    ofe.tiles.iter().map(move |tile| crate::TileState {
                        ofe_id: ofe.ofe_id.clone(),
                        tile_id: tile.tile_id.clone(),
                        surface_enthalpy_j_m2_tile_ground: 0.0,
                        surface_temperature_warm_start_k: crate::REFERENCE_TEMPERATURE_K,
                    })
                })
                .collect(),
        };
        state.state_sha256 = state.canonical_sha256().expect("V1 state digest");
        state.validate(&configuration).expect("V1 runtime");
        (configuration, state)
    }

    fn v2_runtime() -> (LandSurfaceEnergyConfiguration, LandSurfaceEnergyV2State) {
        let (mut configuration, mut state) = v1_runtime();
        configuration.model_version = V2_MODEL_VERSION.into();
        configuration.model_definition_sha256 =
            Sha256Digest::try_new(V2_MODEL_DEFINITION_SHA256).expect("V2 digest");
        configuration.vegetation_configuration.model_version = V2_VEGETATION_MODEL_VERSION.into();
        configuration
            .vegetation_configuration
            .model_definition_sha256 =
            Sha256Digest::try_new(V2_VEGETATION_MODEL_DEFINITION_SHA256).expect("vegetation");
        configuration.configuration_sha256 = configuration.canonical_sha256().expect("config");
        state.model_definition_sha256 =
            Sha256Digest::try_new(V2_MODEL_DEFINITION_SHA256).expect("V2 digest");
        state
            .configuration_sha256
            .clone_from(&configuration.configuration_sha256);
        state.state_sha256 = state.canonical_sha256().expect("state digest");
        let state = LandSurfaceEnergyV2State(state);
        configuration.validate_v2().expect("V2 config");
        state.validate(&configuration).expect("V2 state");
        (configuration, state)
    }

    #[test]
    fn successor_constants_are_distinct_from_v2() {
        assert_ne!(V3_MODEL_DEFINITION_SHA256, V2_MODEL_DEFINITION_SHA256);
        assert_eq!(V3_MODEL_VERSION, "OPENWEPP_SNOW_FREE_LSE_V3");
    }

    #[test]
    fn v2_to_v3_migration_preserves_scientific_values_and_rebinds_identity() {
        let (v2_configuration, v2_state) = v2_runtime();
        let v2_bytes = serde_json::to_vec(&v2_state).expect("V2 bytes");
        let v3_configuration =
            migrate_v2_configuration_to_v3(&v2_configuration).expect("V3 config");
        let v3_state = migrate_v2_state_to_v3(&v2_configuration, &v2_state, &v3_configuration)
            .expect("V3 state");
        assert_eq!(v3_state.0.tiles, v2_state.0.tiles);
        assert_eq!(
            v3_state.0.last_accepted_transaction_id,
            v2_state.0.last_accepted_transaction_id
        );
        assert_eq!(serde_json::to_vec(&v2_state).expect("V2 replay"), v2_bytes);
        assert_eq!(
            v3_state.0.model_definition_sha256.as_str(),
            V3_MODEL_DEFINITION_SHA256
        );
        v3_state.validate(&v3_configuration).expect("V3 validation");
        let replay = LandSurfaceEnergyV3State::from_json(
            &v3_state.to_json().expect("V3 bytes"),
            &v3_configuration,
        )
        .expect("V3 replay");
        assert_eq!(replay, v3_state);
    }

    #[test]
    fn mixed_v2_v3_identity_fails_without_mutating_source() {
        let (v2_configuration, v2_state) = v2_runtime();
        let before = serde_json::to_vec(&v2_state).expect("before");
        let mut wrong_target =
            migrate_v2_configuration_to_v3(&v2_configuration).expect("V3 configuration");
        wrong_target.model_version = V2_MODEL_VERSION.into();
        assert!(migrate_v2_state_to_v3(&v2_configuration, &v2_state, &wrong_target,).is_err());
        assert_eq!(serde_json::to_vec(&v2_state).expect("after"), before);
    }
}
