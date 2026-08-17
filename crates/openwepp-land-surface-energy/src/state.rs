#![allow(clippy::missing_errors_doc)]

use std::collections::BTreeSet;

use openwepp_kernel_contract::{ResourceOwnerId, TileId, TransactionId};
use serde::{Deserialize, Serialize};

use crate::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyError, MODEL_DEFINITION_SHA256, OfeId,
    Sha256Digest, SurfaceConfiguration, SurfaceHeatStorageMode, canonical_digest, require_finite,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TileState {
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub surface_enthalpy_j_m2_tile_ground: f64,
    pub surface_temperature_warm_start_k: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LandSurfaceEnergyState {
    pub model_definition_sha256: Sha256Digest,
    pub configuration_sha256: Sha256Digest,
    pub state_sha256: Sha256Digest,
    pub owner_id: ResourceOwnerId,
    pub last_accepted_transaction_id: Option<TransactionId>,
    pub tiles: Vec<TileState>,
}

impl LandSurfaceEnergyState {
    pub fn validate_schema(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.model_definition_sha256.as_str() != MODEL_DEFINITION_SHA256 {
            return Err(LandSurfaceEnergyError::Identity {
                field: "state.model_definition_sha256",
                expected: MODEL_DEFINITION_SHA256.into(),
                found: self.model_definition_sha256.to_string(),
            });
        }
        if self.tiles.is_empty() {
            return Err(LandSurfaceEnergyError::topology_cardinality(
                "empty state tile set",
            ));
        }
        let mut identities = BTreeSet::new();
        for tile in &self.tiles {
            if !identities.insert((tile.ofe_id.clone(), tile.tile_id.clone())) {
                return Err(LandSurfaceEnergyError::topology_cardinality(
                    "duplicate tile state",
                ));
            }
            require_finite(
                tile.surface_enthalpy_j_m2_tile_ground,
                "state.surface_enthalpy_j_m2_tile_ground",
            )?;
            require_finite(
                tile.surface_temperature_warm_start_k,
                "state.surface_temperature_warm_start_k",
            )?;
            if !(200.0..=350.0).contains(&tile.surface_temperature_warm_start_k) {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "state.surface_temperature_warm_start_k",
                ));
            }
        }
        let computed = self.canonical_sha256()?;
        if self.state_sha256 != computed {
            return Err(LandSurfaceEnergyError::Identity {
                field: "state_sha256",
                expected: computed.to_string(),
                found: self.state_sha256.to_string(),
            });
        }
        Ok(())
    }

    pub fn validate(
        &self,
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<(), LandSurfaceEnergyError> {
        self.validate_schema()?;
        if self.model_definition_sha256.as_str() != MODEL_DEFINITION_SHA256 {
            return Err(LandSurfaceEnergyError::Identity {
                field: "state.model_definition_sha256",
                expected: MODEL_DEFINITION_SHA256.into(),
                found: self.model_definition_sha256.to_string(),
            });
        }
        if self.configuration_sha256 != configuration.configuration_sha256 {
            return Err(LandSurfaceEnergyError::Identity {
                field: "state.configuration_sha256",
                expected: configuration.configuration_sha256.to_string(),
                found: self.configuration_sha256.to_string(),
            });
        }
        if self.owner_id != configuration.owner_id {
            return Err(LandSurfaceEnergyError::Identity {
                field: "state.owner_id",
                expected: configuration.owner_id.as_str().into(),
                found: self.owner_id.as_str().into(),
            });
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
        let mut actual = BTreeSet::new();
        for tile in &self.tiles {
            if !actual.insert((tile.ofe_id.clone(), tile.tile_id.clone())) {
                return Err(LandSurfaceEnergyError::topology_cardinality(
                    "duplicate tile state",
                ));
            }
            require_finite(
                tile.surface_enthalpy_j_m2_tile_ground,
                "state.surface_enthalpy_j_m2_tile_ground",
            )?;
            require_finite(
                tile.surface_temperature_warm_start_k,
                "state.surface_temperature_warm_start_k",
            )?;
            if !(200.0..=350.0).contains(&tile.surface_temperature_warm_start_k) {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "state.surface_temperature_warm_start_k",
                ));
            }
            let configured_tile = configuration
                .ofes
                .iter()
                .find(|ofe| ofe.ofe_id == tile.ofe_id)
                .and_then(|ofe| {
                    ofe.tiles
                        .iter()
                        .find(|candidate| candidate.tile_id == tile.tile_id)
                })
                .ok_or(LandSurfaceEnergyError::topology_cardinality(
                    "extra tile state",
                ))?;
            if configured_tile.surface_heat_storage_mode == SurfaceHeatStorageMode::EquilibriumZero
                && tile.surface_enthalpy_j_m2_tile_ground != 0.0
            {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "equilibrium-zero surface enthalpy",
                ));
            }
            if let SurfaceConfiguration::ForestLitter { .. } = configured_tile.surface {
                if configured_tile.surface_heat_storage_mode
                    == SurfaceHeatStorageMode::EquilibriumZero
                {
                    return Err(LandSurfaceEnergyError::UnsupportedDomain(
                        "equilibrium-zero forest litter state",
                    ));
                }
            }
        }
        if actual != expected {
            return Err(LandSurfaceEnergyError::topology_cardinality(
                "state tile set mismatch",
            ));
        }
        let computed = self.canonical_sha256()?;
        if self.state_sha256 != computed {
            return Err(LandSurfaceEnergyError::Identity {
                field: "state_sha256",
                expected: computed.to_string(),
                found: self.state_sha256.to_string(),
            });
        }
        Ok(())
    }

    pub fn validate_transaction_lineage(
        &self,
        candidate: TransactionId,
    ) -> Result<(), LandSurfaceEnergyError> {
        if candidate.0 == 0 {
            return Err(LandSurfaceEnergyError::StateLineage(
                "zero candidate transaction",
            ));
        }
        if let Some(previous) = self.last_accepted_transaction_id {
            if previous.0 >= candidate.0 {
                return Err(LandSurfaceEnergyError::StateLineage(
                    "candidate transaction is not newer than accepted state",
                ));
            }
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<Sha256Digest, LandSurfaceEnergyError> {
        let mut value = serde_json::to_value(self)
            .map_err(|error| LandSurfaceEnergyError::MalformedSerialization(error.to_string()))?;
        let digest =
            value
                .get_mut("state_sha256")
                .ok_or(LandSurfaceEnergyError::MalformedSerialization(
                    "state_sha256 absent from serialized state".into(),
                ))?;
        *digest = serde_json::Value::String(String::new());
        canonical_digest(&value)
    }

    pub fn from_json(
        bytes: &[u8],
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<Self, LandSurfaceEnergyError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| LandSurfaceEnergyError::MalformedSerialization(error.to_string()))?;
        value.validate(configuration)?;
        Ok(value)
    }
}
