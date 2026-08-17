//! Strict persistent state and offline V7-to-V8 migration.
//!
//! V7 execution remains available under its immutable public API. This module
//! defines the separate V8 state identity required by the coupled snow-free
//! land-surface transaction; it does not make V8 executable by itself.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, StratumId, TileId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::carbon_nitrogen::Tissue;
use crate::occupancy_state::OccupancyState;
use crate::{
    CoupledOwnedState, PhenologyType, StratumSharedState, VegetationConfiguration, VegetationError,
};

pub const V8_MODEL_VERSION: &str = "OPENWEPP_C3_WOODY_V8";
pub const V8_MODEL_SHA256: &str =
    "622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b";
pub const V8_MODEL_BYTES: &[u8] =
    include_bytes!("../model-registry/openwepp_c3_woody_v8_definition.json");

/// Validates and returns the immutable V8 model-definition bytes.
pub fn load_v8_model_definition() -> Result<crate::ModelDefinition, VegetationError> {
    let found = format!("{:x}", Sha256::digest(V8_MODEL_BYTES));
    if found != V8_MODEL_SHA256 {
        return Err(VegetationError::ModelDigestMismatch {
            expected: V8_MODEL_SHA256.into(),
            found,
        });
    }
    let value: serde_json::Value = serde_json::from_slice(V8_MODEL_BYTES)
        .map_err(|error| VegetationError::Schema(error.to_string()))?;
    if value
        .get("model_version")
        .and_then(serde_json::Value::as_str)
        != Some(V8_MODEL_VERSION)
    {
        return Err(VegetationError::Schema(
            "V8 model_version does not match registry identity".into(),
        ));
    }
    Ok(crate::ModelDefinition {
        version: V8_MODEL_VERSION,
        sha256: V8_MODEL_SHA256.into(),
        bytes: V8_MODEL_BYTES,
    })
}

/// The V8 occupancy-local lane. Canopy air is deliberately absent.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V8OccupancyState {
    pub beta_hyd: f64,
    pub canopy_liquid_kg_h2o_m2_tile_ground: f64,
    pub dry_stem_temperature_k: f64,
    pub last_accepted_transaction_id: Option<u128>,
    pub root_node_potential_mm: f64,
    pub shade_ci_pa: f64,
    pub shade_leaf_potential_mm: f64,
    pub shade_leaf_temperature_k: f64,
    pub stem_potential_mm: f64,
    pub sun_ci_pa: f64,
    pub sun_leaf_potential_mm: f64,
    pub sun_leaf_temperature_k: f64,
    pub wet_surface_temperature_k: f64,
}

impl V8OccupancyState {
    fn validate(&self, expected: Option<u128>) -> Result<(), V8StateError> {
        finite_fraction(self.beta_hyd, "beta_hyd")?;
        finite_nonnegative(
            self.canopy_liquid_kg_h2o_m2_tile_ground,
            "canopy_liquid_kg_h2o_m2_tile_ground",
        )?;
        for (value, field) in [
            (self.dry_stem_temperature_k, "dry_stem_temperature_k"),
            (self.shade_ci_pa, "shade_ci_pa"),
            (self.shade_leaf_temperature_k, "shade_leaf_temperature_k"),
            (self.sun_ci_pa, "sun_ci_pa"),
            (self.sun_leaf_temperature_k, "sun_leaf_temperature_k"),
            (self.wet_surface_temperature_k, "wet_surface_temperature_k"),
        ] {
            finite_positive(value, field)?;
        }
        for (value, field) in [
            (self.root_node_potential_mm, "root_node_potential_mm"),
            (self.shade_leaf_potential_mm, "shade_leaf_potential_mm"),
            (self.stem_potential_mm, "stem_potential_mm"),
            (self.sun_leaf_potential_mm, "sun_leaf_potential_mm"),
        ] {
            finite(value, field)?;
        }
        if self.last_accepted_transaction_id != expected {
            return Err(V8StateError::StaleTransaction {
                lane: "occupancy",
                expected,
                found: self.last_accepted_transaction_id,
            });
        }
        Ok(())
    }
}

/// One zero-storage canopy-air numerical lane shared by a covered OFE/tile.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V8TileCanopyAirState {
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub canopy_air_temperature_k: f64,
}

impl V8TileCanopyAirState {
    fn validate(&self) -> Result<(), V8StateError> {
        finite_nonnegative(
            self.canopy_air_specific_humidity_kg_kg,
            "canopy_air_specific_humidity_kg_kg",
        )?;
        finite_positive(self.canopy_air_temperature_k, "canopy_air_temperature_k")?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V8CoupledOwnedState {
    pub configuration_sha256: String,
    pub last_transaction_id: u128,
    pub model_definition_sha256: String,
    #[serde(with = "occupancy_map")]
    pub occupancies: BTreeMap<OccupancyId, V8OccupancyState>,
    pub state_sha256: String,
    #[serde(with = "stratum_map")]
    pub strata: BTreeMap<StratumId, StratumSharedState>,
    #[serde(with = "tile_air_map")]
    pub tile_canopy_air: BTreeMap<TileId, V8TileCanopyAirState>,
}

impl V8CoupledOwnedState {
    pub fn parse_strict(
        bytes: &[u8],
        config: &VegetationConfiguration,
    ) -> Result<Self, V8StateError> {
        let state: Self = serde_json::from_slice(bytes)
            .map_err(|error| V8StateError::Schema(error.to_string()))?;
        state.validate(config)?;
        Ok(state)
    }

    pub fn validate(&self, config: &VegetationConfiguration) -> Result<(), V8StateError> {
        config.validate_v8().map_err(V8StateError::Configuration)?;
        require_digest(&self.configuration_sha256, "configuration_sha256")?;
        require_digest(&self.state_sha256, "state_sha256")?;
        if self.model_definition_sha256 != V8_MODEL_SHA256
            || self.configuration_sha256 != config.configuration_sha256
        {
            return Err(V8StateError::Identity);
        }
        if self.state_sha256 != self.canonical_sha256() {
            return Err(V8StateError::Digest);
        }
        if self.last_transaction_id == 0 && config.initial_state_sha256 != self.state_sha256 {
            return Err(V8StateError::InitialStateReceipt);
        }

        let configured_strata = config
            .strata
            .iter()
            .map(|stratum| stratum.stratum_id.clone())
            .collect::<BTreeSet<_>>();
        if self.strata.keys().cloned().collect::<BTreeSet<_>>() != configured_strata {
            return Err(V8StateError::Topology("stratum set"));
        }
        if self.occupancies.keys().cloned().collect::<BTreeSet<_>>()
            != config.expected_occupancies()
        {
            return Err(V8StateError::Topology("occupancy set"));
        }
        let covered_tiles = config
            .expected_occupancies()
            .into_iter()
            .map(|occupancy| occupancy.tile_id)
            .collect::<BTreeSet<_>>();
        if self
            .tile_canopy_air
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
            != covered_tiles
        {
            return Err(V8StateError::Topology("OFE/tile canopy-air set"));
        }

        let expected_lineage = (self.last_transaction_id != 0).then_some(self.last_transaction_id);
        for lane in self.occupancies.values() {
            lane.validate(expected_lineage)?;
        }
        for lane in self.tile_canopy_air.values() {
            lane.validate()?;
        }
        validate_shared_states(&self.strata, config, self.last_transaction_id)?;
        Ok(())
    }

    #[must_use]
    pub fn canonical_sha256(&self) -> String {
        crate::transaction::state_canonical::v8_sha256(self)
    }

    /// Returns the V7 canonical typed digest stream extended by the V8 state
    /// delta, with `state_sha256` replaced by the empty string.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        crate::transaction::state_canonical::v8_bytes(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V7ToV8Snapshot {
    Initial,
    Current,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V7ToV8MigrationField {
    CanopyAirSpecificHumidityKgKg,
    CanopyAirTemperatureK,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct V7ToV8UnresolvedField {
    pub snapshot: V7ToV8Snapshot,
    pub tile_id: TileId,
    pub field: V7ToV8MigrationField,
    pub reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct V7ToV8MigrationReport {
    pub source_model_definition_sha256: String,
    pub target_model_definition_sha256: String,
    pub unresolved: Vec<V7ToV8UnresolvedField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct V7ToV8Migration {
    pub configuration: VegetationConfiguration,
    pub initial_state: V8CoupledOwnedState,
    pub state: V8CoupledOwnedState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum V7ToV8MigrationResult {
    Complete(Box<V7ToV8Migration>),
    Incomplete(V7ToV8MigrationReport),
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum V7ToV8MigrationError {
    #[error("invalid complete V7 source configuration")]
    InvalidSourceConfiguration,
    #[error("invalid complete V7 initial state")]
    InvalidSourceInitialState,
    #[error("invalid complete V7 current state")]
    InvalidSourceState,
    #[error("invalid V8 target configuration")]
    InvalidTargetConfiguration,
    #[error("V7/V8 non-identity configuration payload differs")]
    ConfigurationPayloadMismatch,
    #[error("V8 initial-state receipt mismatch")]
    InitialStateReceiptMismatch,
    #[error("constructed V8 state rejected")]
    TargetStateRejected,
}

pub fn migrate_v7_snapshot(
    source_configuration: &VegetationConfiguration,
    source_initial_state: &CoupledOwnedState,
    source_state: &CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
) -> Result<V7ToV8MigrationResult, V7ToV8MigrationError> {
    source_configuration
        .validate()
        .map_err(|_| V7ToV8MigrationError::InvalidSourceConfiguration)?;
    source_initial_state
        .validate(source_configuration)
        .map_err(|_| V7ToV8MigrationError::InvalidSourceInitialState)?;
    source_state
        .validate(source_configuration)
        .map_err(|_| V7ToV8MigrationError::InvalidSourceState)?;
    target_configuration
        .validate_v8()
        .map_err(|_| V7ToV8MigrationError::InvalidTargetConfiguration)?;
    if !configuration_payload_matches(source_configuration, target_configuration) {
        return Err(V7ToV8MigrationError::ConfigurationPayloadMismatch);
    }

    let initial = migrate_one_state(
        source_initial_state,
        target_configuration,
        V7ToV8Snapshot::Initial,
    );
    let current = migrate_one_state(source_state, target_configuration, V7ToV8Snapshot::Current);
    let mut unresolved = Vec::new();
    if let Err(fields) = &initial {
        unresolved.extend(fields.iter().cloned());
    }
    if let Err(fields) = &current {
        unresolved.extend(fields.iter().cloned());
    }
    unresolved.sort();
    unresolved.dedup();
    if !unresolved.is_empty() {
        return Ok(V7ToV8MigrationResult::Incomplete(V7ToV8MigrationReport {
            source_model_definition_sha256: crate::MODEL_SHA256.into(),
            target_model_definition_sha256: V8_MODEL_SHA256.into(),
            unresolved,
        }));
    }
    let initial_state = initial.map_err(|_| V7ToV8MigrationError::TargetStateRejected)?;
    if target_configuration.initial_state_sha256 != initial_state.state_sha256 {
        return Err(V7ToV8MigrationError::InitialStateReceiptMismatch);
    }
    let state = current.map_err(|_| V7ToV8MigrationError::TargetStateRejected)?;
    initial_state
        .validate(target_configuration)
        .map_err(|_| V7ToV8MigrationError::TargetStateRejected)?;
    state
        .validate(target_configuration)
        .map_err(|_| V7ToV8MigrationError::TargetStateRejected)?;
    Ok(V7ToV8MigrationResult::Complete(Box::new(V7ToV8Migration {
        configuration: target_configuration.clone(),
        initial_state,
        state,
    })))
}

fn migrate_one_state(
    source: &CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
    snapshot: V7ToV8Snapshot,
) -> Result<V8CoupledOwnedState, Vec<V7ToV8UnresolvedField>> {
    let mut tile_canopy_air = BTreeMap::new();
    let mut unresolved = Vec::new();
    for tile in &target_configuration.topology_tiles {
        let lanes = source
            .occupancies
            .iter()
            .filter(|(identity, _)| identity.tile_id == tile.tile_id)
            .map(|(_, lane)| lane)
            .collect::<Vec<_>>();
        if lanes.is_empty() {
            continue;
        }
        let first = lanes[0];
        let same_temperature = lanes.iter().all(|lane| {
            lane.canopy_air_temperature_k.to_bits() == first.canopy_air_temperature_k.to_bits()
        });
        let same_humidity = lanes.iter().all(|lane| {
            lane.canopy_air_specific_humidity_kg_kg.to_bits()
                == first.canopy_air_specific_humidity_kg_kg.to_bits()
        });
        if !same_humidity || !same_temperature {
            for field in [
                V7ToV8MigrationField::CanopyAirSpecificHumidityKgKg,
                V7ToV8MigrationField::CanopyAirTemperatureK,
            ] {
                unresolved.push(V7ToV8UnresolvedField {
                    snapshot,
                    tile_id: tile.tile_id.clone(),
                    field,
                    reason: "ambiguous_v7_occupancy_canopy_air",
                });
            }
            continue;
        }
        tile_canopy_air.insert(
            tile.tile_id.clone(),
            V8TileCanopyAirState {
                canopy_air_specific_humidity_kg_kg: first.canopy_air_specific_humidity_kg_kg,
                canopy_air_temperature_k: first.canopy_air_temperature_k,
            },
        );
    }
    if !unresolved.is_empty() {
        return Err(unresolved);
    }
    let occupancies = source
        .occupancies
        .iter()
        .map(|(identity, lane)| (identity.clone(), lane.into()))
        .collect();
    let mut result = V8CoupledOwnedState {
        configuration_sha256: target_configuration.configuration_sha256.clone(),
        last_transaction_id: source.last_transaction_id,
        model_definition_sha256: V8_MODEL_SHA256.into(),
        occupancies,
        state_sha256: String::new(),
        strata: source.strata.clone(),
        tile_canopy_air,
    };
    result.state_sha256 = result.canonical_sha256();
    Ok(result)
}

impl From<&OccupancyState> for V8OccupancyState {
    fn from(value: &OccupancyState) -> Self {
        Self {
            beta_hyd: value.beta_hyd,
            canopy_liquid_kg_h2o_m2_tile_ground: value.canopy_liquid_kg_h2o_m2_tile_ground,
            dry_stem_temperature_k: value.dry_stem_temperature_k,
            last_accepted_transaction_id: value.last_accepted_transaction_id,
            root_node_potential_mm: value.root_node_potential_mm,
            shade_ci_pa: value.shade_ci_pa,
            shade_leaf_potential_mm: value.shade_leaf_potential_mm,
            shade_leaf_temperature_k: value.shade_leaf_temperature_k,
            stem_potential_mm: value.stem_potential_mm,
            sun_ci_pa: value.sun_ci_pa,
            sun_leaf_potential_mm: value.sun_leaf_potential_mm,
            sun_leaf_temperature_k: value.sun_leaf_temperature_k,
            wet_surface_temperature_k: value.wet_surface_temperature_k,
        }
    }
}

fn configuration_payload_matches(
    source: &VegetationConfiguration,
    target: &VegetationConfiguration,
) -> bool {
    let mut source = source.clone();
    let mut target = target.clone();
    for value in [&mut source, &mut target] {
        value.model_definition_sha256.clear();
        value.configuration_sha256.clear();
        value.initial_state_sha256.clear();
    }
    source == target
}

#[allow(clippy::too_many_lines)] // Exact six-tissue V7 payload validation is intentionally contiguous.
fn validate_shared_states(
    states: &BTreeMap<StratumId, StratumSharedState>,
    config: &VegetationConfiguration,
    transaction_id: u128,
) -> Result<(), V8StateError> {
    let mut transfer_identities = BTreeSet::new();
    for (stratum_id, state) in states {
        if state.last_transaction_id != transaction_id {
            return Err(V8StateError::StaleSharedStratum);
        }
        let scalars = [
            state.retranslocation_n,
            state.nsc_c,
            state.xs_c,
            state.standing_dead.carbon,
            state.standing_dead.nitrogen,
            state.standing_dead_dm,
            state.onset_remaining_s,
            state.offset_remaining_s,
            state.previous_gsi,
            state.t10_k,
            state.leaf_area,
            state.root_area,
            state.stem_area,
        ];
        if scalars.iter().any(|value| !value.is_finite())
            || state.retranslocation_n < 0.0
            || state.nsc_c < 0.0
            || state.standing_dead.carbon < 0.0
            || state.standing_dead.nitrogen < 0.0
            || state.standing_dead_dm < 0.0
            || state.onset_remaining_s < 0.0
            || state.offset_remaining_s < 0.0
            || state.leaf_area < 0.0
            || state.root_area < 0.0
            || state.stem_area < 0.0
            || state.t10_k <= 0.0
            || !(0.0..=1.0).contains(&state.previous_gsi)
        {
            return Err(V8StateError::Domain("complete stratum state"));
        }
        let required = [
            Tissue::Leaf,
            Tissue::FineRoot,
            Tissue::LiveStem,
            Tissue::DeadStem,
            Tissue::LiveCoarseRoot,
            Tissue::DeadCoarseRoot,
        ];
        if state.tissues.len() != required.len()
            || required
                .iter()
                .any(|tissue| !state.tissues.contains_key(tissue))
        {
            return Err(V8StateError::Topology("six-tissue identity"));
        }
        for pool in state.tissues.values() {
            for value in [
                pool.display.carbon,
                pool.display.nitrogen,
                pool.storage.carbon,
                pool.storage.nitrogen,
                pool.transfer.carbon,
                pool.transfer.nitrogen,
            ] {
                if !value.is_finite() || value < 0.0 {
                    return Err(V8StateError::Domain("tissue pool"));
                }
            }
        }
        let expected_owner = format!("stratum:{}", stratum_id.as_str());
        for transfer in &state.pending_transfers {
            let identity = (
                transfer.transaction_id,
                transfer.owner_id.clone(),
                transfer.proposal_id,
            );
            if transfer.transaction_id == 0
                || transfer.transaction_id != transaction_id
                || transfer.owner_id.as_str() != expected_owner
                || transfer.proposal_id == 0
                || [transfer.carbon, transfer.nitrogen, transfer.dry_matter]
                    .iter()
                    .any(|value| !value.is_finite() || *value < 0.0)
                || !transfer_identities.insert(identity)
            {
                return Err(V8StateError::Domain("pending material transfer"));
            }
        }
        let stratum = config
            .strata
            .iter()
            .find(|candidate| candidate.stratum_id == *stratum_id)
            .ok_or(V8StateError::Topology("missing stratum configuration"))?;
        if stratum.phenology_type == PhenologyType::Evergreen
            && state.tissues.values().any(|pool| {
                pool.storage.carbon != 0.0
                    || pool.storage.nitrogen != 0.0
                    || pool.transfer.carbon != 0.0
                    || pool.transfer.nitrogen != 0.0
            })
        {
            return Err(V8StateError::Domain("V8 evergreen storage/transfer state"));
        }
        crate::transaction::validate_displayed_leaf_identity(state, stratum)
            .map_err(V8StateError::Configuration)?;
    }
    Ok(())
}

#[derive(Debug, Error, PartialEq)]
pub enum V8StateError {
    #[error("VEG-E-SCHEMA-001: invalid V8 state: {0}")]
    Schema(String),
    #[error("VEG-E-071: invalid V8 state field {0}")]
    Domain(&'static str),
    #[error("VEG-E-113: V8 model/configuration identity mismatch")]
    Identity,
    #[error("VEG-E-113: V8 state digest mismatch")]
    Digest,
    #[error("VEG-E-113: V8 initial-state receipt mismatch")]
    InitialStateReceipt,
    #[error("VEG-E-113: V8 topology mismatch: {0}")]
    Topology(&'static str),
    #[error("VEG-E-113: stale {lane} transaction: expected {expected:?}, found {found:?}")]
    StaleTransaction {
        lane: &'static str,
        expected: Option<u128>,
        found: Option<u128>,
    },
    #[error("VEG-E-113: stale shared-stratum transaction")]
    StaleSharedStratum,
    #[error("VEG-E-113: invalid V8 configuration: {0}")]
    Configuration(VegetationError),
}

fn require_digest(value: &str, field: &'static str) -> Result<(), V8StateError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(V8StateError::Domain(field));
    }
    Ok(())
}

fn finite(value: f64, field: &'static str) -> Result<(), V8StateError> {
    if !value.is_finite() {
        return Err(V8StateError::Domain(field));
    }
    Ok(())
}

fn finite_positive(value: f64, field: &'static str) -> Result<(), V8StateError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(V8StateError::Domain(field));
    }
    Ok(())
}

fn finite_nonnegative(value: f64, field: &'static str) -> Result<(), V8StateError> {
    if !value.is_finite() || value < 0.0 {
        return Err(V8StateError::Domain(field));
    }
    Ok(())
}

fn finite_fraction(value: f64, field: &'static str) -> Result<(), V8StateError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(V8StateError::Domain(field));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn v8_test_fixture() -> (VegetationConfiguration, V8CoupledOwnedState) {
    let (mut source_configuration, mut source_state) =
        crate::transaction::v7_identity_rebound_fixture();
    for lane in source_state.occupancies.values_mut() {
        lane.canopy_air_temperature_k = 295.25;
        lane.canopy_air_specific_humidity_kg_kg = 0.00925;
    }
    source_state.state_sha256 = source_state.canonical_sha256().expect("V7 state digest");
    source_configuration.initial_state_sha256 = source_state.state_sha256.clone();
    let mut target_configuration = source_configuration.clone();
    target_configuration.model_definition_sha256 = V8_MODEL_SHA256.into();
    target_configuration.configuration_sha256 = target_configuration
        .canonical_sha256()
        .expect("V8 configuration digest");
    target_configuration.initial_state_sha256 = "0".repeat(64);
    let provisional = migrate_one_state(
        &source_state,
        &target_configuration,
        V7ToV8Snapshot::Initial,
    )
    .expect("unambiguous V8 migration");
    target_configuration.initial_state_sha256 = provisional.state_sha256.clone();
    let V7ToV8MigrationResult::Complete(migration) = migrate_v7_snapshot(
        &source_configuration,
        &source_state,
        &source_state,
        &target_configuration,
    )
    .expect("valid V8 migration") else {
        panic!("V8 test fixture must migrate completely")
    };
    (target_configuration, migration.initial_state)
}

mod occupancy_map {
    use std::collections::BTreeMap;
    use std::fmt;
    use std::marker::PhantomData;

    use openwepp_kernel_contract::OccupancyId;
    use serde::de::{MapAccess, SeqAccess, Visitor};
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::V8OccupancyState;

    #[derive(Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    struct Entry {
        identity: OccupancyId,
        state: V8OccupancyState,
    }

    pub(super) fn serialize<S>(
        values: &BTreeMap<OccupancyId, V8OccupancyState>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(values.len()))?;
        for (identity, state) in values {
            sequence.serialize_element(&Entry {
                identity: identity.clone(),
                state: state.clone(),
            })?;
        }
        sequence.end()
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<OccupancyId, V8OccupancyState>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Entries(PhantomData<()>);
        impl<'de> Visitor<'de> for Entries {
            type Value = BTreeMap<OccupancyId, V8OccupancyState>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an identity-sorted V8 occupancy array")
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result = BTreeMap::new();
                let mut previous: Option<OccupancyId> = None;
                while let Some(entry) = sequence.next_element::<Entry>()? {
                    if previous
                        .as_ref()
                        .is_some_and(|value| value >= &entry.identity)
                    {
                        return Err(serde::de::Error::custom(
                            "V8 occupancies are duplicated or not identity-sorted",
                        ));
                    }
                    previous = Some(entry.identity.clone());
                    result.insert(entry.identity, entry.state);
                }
                Ok(result)
            }

            fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                Err(serde::de::Error::custom("V8 occupancies must be an array"))
            }
        }
        deserializer.deserialize_seq(Entries(PhantomData))
    }
}

mod tile_air_map {
    use std::collections::BTreeMap;
    use std::fmt;
    use std::marker::PhantomData;

    use serde::de::{SeqAccess, Visitor};
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use openwepp_kernel_contract::TileId;

    use super::V8TileCanopyAirState;

    #[derive(Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    struct Identity {
        tile_id: TileId,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    struct Entry {
        identity: Identity,
        state: V8TileCanopyAirState,
    }

    pub(super) fn serialize<S>(
        values: &BTreeMap<TileId, V8TileCanopyAirState>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(values.len()))?;
        for (identity, state) in values {
            sequence.serialize_element(&Entry {
                identity: Identity {
                    tile_id: identity.clone(),
                },
                state: state.clone(),
            })?;
        }
        sequence.end()
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<TileId, V8TileCanopyAirState>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Entries(PhantomData<()>);
        impl<'de> Visitor<'de> for Entries {
            type Value = BTreeMap<TileId, V8TileCanopyAirState>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an identity-sorted V8 tile-canopy-air array")
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result = BTreeMap::new();
                let mut previous: Option<TileId> = None;
                while let Some(entry) = sequence.next_element::<Entry>()? {
                    if previous
                        .as_ref()
                        .is_some_and(|value| value >= &entry.identity.tile_id)
                    {
                        return Err(serde::de::Error::custom(
                            "V8 tile canopy-air lanes are duplicated or not identity-sorted",
                        ));
                    }
                    previous = Some(entry.identity.tile_id.clone());
                    result.insert(entry.identity.tile_id, entry.state);
                }
                Ok(result)
            }
        }
        deserializer.deserialize_seq(Entries(PhantomData))
    }
}

mod stratum_map {
    use std::collections::BTreeMap;
    use std::fmt;
    use std::marker::PhantomData;

    use openwepp_kernel_contract::StratumId;
    use serde::de::{MapAccess, Visitor};
    use serde::{Deserializer, Serializer};

    use super::StratumSharedState;

    pub(super) fn serialize<S>(
        values: &BTreeMap<StratumId, StratumSharedState>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serde::Serialize::serialize(values, serializer)
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<StratumId, StratumSharedState>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Strata(PhantomData<()>);
        impl<'de> Visitor<'de> for Strata {
            type Value = BTreeMap<StratumId, StratumSharedState>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an identity-sorted V8 stratum map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut result = BTreeMap::new();
                let mut previous: Option<StratumId> = None;
                while let Some((identity, state)) = map.next_entry()? {
                    if previous.as_ref().is_some_and(|value| value >= &identity) {
                        return Err(serde::de::Error::custom(
                            "V8 strata are duplicated or not identity-sorted",
                        ));
                    }
                    previous = Some(identity.clone());
                    result.insert(identity, state);
                }
                Ok(result)
            }
        }
        deserializer.deserialize_map(Strata(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target_configuration(
        source: &VegetationConfiguration,
        initial_state_sha256: String,
    ) -> VegetationConfiguration {
        let mut target = source.clone();
        target.model_definition_sha256 = V8_MODEL_SHA256.into();
        target.configuration_sha256 = target.canonical_sha256().expect("V8 config digest");
        target.initial_state_sha256 = initial_state_sha256;
        target
    }

    fn migratable_fixture() -> (
        VegetationConfiguration,
        CoupledOwnedState,
        VegetationConfiguration,
    ) {
        let (source, mut state) = crate::transaction::v7_identity_rebound_fixture();
        for lane in state.occupancies.values_mut() {
            lane.canopy_air_temperature_k = 295.25;
            lane.canopy_air_specific_humidity_kg_kg = 0.00925;
        }
        state.state_sha256 = state.canonical_sha256().expect("source digest");
        let mut source = source;
        source.initial_state_sha256 = state.state_sha256.clone();
        let target = target_configuration(&source, "0".repeat(64));
        (source, state, target)
    }

    fn migrated_fixture() -> (VegetationConfiguration, V8CoupledOwnedState) {
        let (source, state, mut target) = migratable_fixture();
        let provisional = migrate_one_state(&state, &target, V7ToV8Snapshot::Initial)
            .expect("unambiguous migration");
        target.initial_state_sha256 = provisional.state_sha256.clone();
        let V7ToV8MigrationResult::Complete(migration) =
            migrate_v7_snapshot(&source, &state, &state, &target).expect("migration validates")
        else {
            panic!("expected complete migration")
        };
        (target, migration.initial_state)
    }

    #[test]
    fn v8_definition_has_exact_released_identity() {
        const RELEASED: &[u8] = include_bytes!(
            "../../../docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/openwepp_c3_woody_v8_definition.json"
        );
        let definition = load_v8_model_definition().expect("V8 definition");
        assert_eq!(definition.version, V8_MODEL_VERSION);
        assert_eq!(definition.sha256, V8_MODEL_SHA256);
        assert_eq!(definition.bytes, RELEASED);
    }

    #[test]
    fn migration_moves_bit_identical_air_to_one_shared_tile_lane() {
        let (target, state) = migrated_fixture();
        state.validate(&target).expect("strict V8 state");
        assert_eq!(state.tile_canopy_air.len(), 1);
        assert_eq!(state.occupancies.len(), 1);
        let lane = state.tile_canopy_air.values().next().expect("tile lane");
        assert_eq!(
            lane.canopy_air_temperature_k.to_bits(),
            295.25_f64.to_bits()
        );
        assert_eq!(
            lane.canopy_air_specific_humidity_kg_kg.to_bits(),
            0.00925_f64.to_bits()
        );
        let serialized = serde_json::to_value(&state).expect("serialize");
        let occupancy = &serialized["occupancies"][0]["state"];
        assert!(occupancy.get("canopy_air_temperature_k").is_none());
        assert!(
            occupancy
                .get("canopy_air_specific_humidity_kg_kg")
                .is_none()
        );
    }

    #[test]
    fn strict_round_trip_and_every_v8_lane_changes_digest() {
        let (target, state) = migrated_fixture();
        let bytes = serde_json::to_vec(&state).expect("serialize");
        let parsed = V8CoupledOwnedState::parse_strict(&bytes, &target).expect("strict parse");
        assert_eq!(parsed, state);
        assert_eq!(
            format!("{:x}", Sha256::digest(state.canonical_bytes())),
            state.state_sha256
        );

        let baseline = state.canonical_sha256();
        macro_rules! occupancy_scalar_changes_digest {
            ($($field:ident),+ $(,)?) => {
                $(
                    let mut candidate = state.clone();
                    let lane = candidate.occupancies.values_mut().next().expect("occupancy");
                    lane.$field = f64::from_bits(lane.$field.to_bits() ^ 1);
                    assert_ne!(candidate.canonical_sha256(), baseline, stringify!($field));
                )+
            };
        }
        occupancy_scalar_changes_digest!(
            beta_hyd,
            canopy_liquid_kg_h2o_m2_tile_ground,
            dry_stem_temperature_k,
            root_node_potential_mm,
            shade_ci_pa,
            shade_leaf_potential_mm,
            shade_leaf_temperature_k,
            stem_potential_mm,
            sun_ci_pa,
            sun_leaf_potential_mm,
            sun_leaf_temperature_k,
            wet_surface_temperature_k,
        );
        let mut occupancy_lineage = state.clone();
        occupancy_lineage
            .occupancies
            .values_mut()
            .next()
            .expect("occupancy")
            .last_accepted_transaction_id = Some(1);
        assert_ne!(occupancy_lineage.canonical_sha256(), baseline);

        for temperature in [false, true] {
            let mut tile = state.clone();
            let lane = tile.tile_canopy_air.values_mut().next().expect("tile");
            if temperature {
                lane.canopy_air_temperature_k =
                    f64::from_bits(lane.canopy_air_temperature_k.to_bits() ^ 1);
            } else {
                lane.canopy_air_specific_humidity_kg_kg =
                    f64::from_bits(lane.canopy_air_specific_humidity_kg_kg.to_bits() ^ 1);
            }
            assert_ne!(tile.canonical_sha256(), baseline);
        }
        let mut identity = state.clone();
        let (_, value) = identity.tile_canopy_air.pop_first().expect("tile lane");
        identity
            .tile_canopy_air
            .insert(TileId::try_new("other-tile").expect("tile"), value);
        assert_ne!(identity.canonical_sha256(), baseline);

        let mut shared = state.clone();
        shared.strata.values_mut().next().expect("stratum").nsc_c = 0.25;
        assert_ne!(shared.canonical_sha256(), baseline);
        let mut root_lineage = state.clone();
        root_lineage.last_transaction_id = 1;
        assert_ne!(root_lineage.canonical_sha256(), baseline);
    }

    #[test]
    fn ambiguous_migration_reports_both_fields_and_no_state() {
        let (mut source, mut state) = crate::transaction::v7_identity_rebound_fixture();
        let mut second = source.strata[0].clone();
        second.stratum_id = StratumId::try_new("tree-2").expect("stratum");
        second.vertical_rank = source.strata[0].vertical_rank + 1;
        second.height_m = source.strata[0].height_m / 2.0;
        second.crown_base_m = second.height_m / 2.0;
        source.strata.push(second.clone());
        source.configuration_sha256 = source.canonical_sha256().expect("config digest");
        let first_identity = state.occupancies.keys().next().expect("occupancy").clone();
        let mut second_identity = first_identity.clone();
        second_identity.stratum_id = second.stratum_id.clone();
        let mut second_lane = state
            .occupancies
            .get(&first_identity)
            .expect("lane")
            .clone();
        second_lane.canopy_air_temperature_k =
            f64::from_bits(second_lane.canopy_air_temperature_k.to_bits() ^ 1);
        state.occupancies.insert(second_identity, second_lane);
        state.strata.insert(
            second.stratum_id,
            state.strata.values().next().expect("shared").clone(),
        );
        state.configuration_sha256 = source.configuration_sha256.clone();
        state.state_sha256 = state.canonical_sha256().expect("source digest");
        source.initial_state_sha256 = state.state_sha256.clone();
        let target = target_configuration(&source, "0".repeat(64));
        let V7ToV8MigrationResult::Incomplete(report) =
            migrate_v7_snapshot(&source, &state, &state, &target).expect("ambiguity is a report")
        else {
            panic!("expected incomplete migration")
        };
        assert_eq!(report.unresolved.len(), 4);
        for snapshot in [V7ToV8Snapshot::Initial, V7ToV8Snapshot::Current] {
            assert!(report.unresolved.iter().any(|field| {
                field.snapshot == snapshot
                    && field.field == V7ToV8MigrationField::CanopyAirSpecificHumidityKgKg
            }));
            assert!(report.unresolved.iter().any(|field| {
                field.snapshot == snapshot
                    && field.field == V7ToV8MigrationField::CanopyAirTemperatureK
            }));
        }
    }

    #[test]
    fn strict_schema_rejects_v7_occupancy_air_and_missing_tile_lane() {
        let (target, state) = migrated_fixture();
        let mut value = serde_json::to_value(&state).expect("serialize");
        value["occupancies"][0]["state"]["canopy_air_temperature_k"] = serde_json::json!(295.25);
        let bytes = serde_json::to_vec(&value).expect("serialize poison");
        assert!(matches!(
            V8CoupledOwnedState::parse_strict(&bytes, &target),
            Err(V8StateError::Schema(_))
        ));

        let mut missing = state;
        missing.tile_canopy_air.clear();
        missing.last_transaction_id = 1;
        for shared in missing.strata.values_mut() {
            shared.last_transaction_id = 1;
        }
        for lane in missing.occupancies.values_mut() {
            lane.last_accepted_transaction_id = Some(1);
        }
        missing.state_sha256 = missing.canonical_sha256();
        assert_eq!(
            missing.validate(&target),
            Err(V8StateError::Topology("OFE/tile canopy-air set"))
        );
    }

    #[test]
    fn wrong_model_configuration_digest_and_lineage_reject() {
        let (target, state) = migrated_fixture();
        let mut wrong_model = state.clone();
        wrong_model.model_definition_sha256 = crate::MODEL_SHA256.into();
        wrong_model.state_sha256 = wrong_model.canonical_sha256();
        assert_eq!(wrong_model.validate(&target), Err(V8StateError::Identity));

        let mut wrong_config = state.clone();
        wrong_config.configuration_sha256 = "a".repeat(64);
        wrong_config.state_sha256 = wrong_config.canonical_sha256();
        assert_eq!(wrong_config.validate(&target), Err(V8StateError::Identity));

        let mut lineage_poison = state;
        lineage_poison.last_transaction_id = 7;
        lineage_poison.state_sha256 = lineage_poison.canonical_sha256();
        assert!(matches!(
            lineage_poison.validate(&target),
            Err(V8StateError::StaleTransaction { .. } | V8StateError::StaleSharedStratum)
        ));
    }

    #[test]
    fn duplicate_and_unsorted_identity_lanes_reject() {
        let (target, state) = migrated_fixture();
        let mut value = serde_json::to_value(&state).expect("serialize");
        let duplicate = value["occupancies"][0].clone();
        value["occupancies"]
            .as_array_mut()
            .expect("array")
            .push(duplicate);
        let bytes = serde_json::to_vec(&value).expect("serialize poison");
        assert!(matches!(
            V8CoupledOwnedState::parse_strict(&bytes, &target),
            Err(V8StateError::Schema(_))
        ));
    }

    #[test]
    fn v7_and_v8_configuration_identities_are_not_aliases() {
        let (source, state, target) = migratable_fixture();
        assert!(source.validate().is_ok());
        assert!(source.validate_v8().is_err());
        assert!(target.validate().is_err());
        assert!(target.validate_v8().is_ok());
        let provisional =
            migrate_one_state(&state, &target, V7ToV8Snapshot::Initial).expect("migration");
        assert_ne!(state.state_sha256, provisional.state_sha256);
    }

    #[test]
    fn migration_rejects_nonidentity_configuration_change() {
        let (source, state, mut target) = migratable_fixture();
        target.strata[0].leaf_dimension_m =
            f64::from_bits(target.strata[0].leaf_dimension_m.to_bits() ^ 1);
        target.configuration_sha256 = target.canonical_sha256().expect("target digest");
        assert_eq!(
            migrate_v7_snapshot(&source, &state, &state, &target),
            Err(V7ToV8MigrationError::ConfigurationPayloadMismatch)
        );
    }

    #[test]
    fn strict_schema_rejects_unknown_root_tile_and_missing_occupancy_fields() {
        let (target, state) = migrated_fixture();
        let mut poisons = Vec::new();

        let mut root = serde_json::to_value(&state).expect("serialize");
        root.as_object_mut()
            .expect("root")
            .insert("unknown".into(), serde_json::json!(1));
        poisons.push(root);

        let mut tile = serde_json::to_value(&state).expect("serialize");
        tile["tile_canopy_air"][0]["state"]
            .as_object_mut()
            .expect("tile state")
            .insert("last_accepted_transaction_id".into(), serde_json::json!(0));
        poisons.push(tile);

        let mut occupancy = serde_json::to_value(&state).expect("serialize");
        occupancy["occupancies"][0]["state"]
            .as_object_mut()
            .expect("occupancy state")
            .remove("beta_hyd");
        poisons.push(occupancy);

        for poison in poisons {
            let bytes = serde_json::to_vec(&poison).expect("serialize poison");
            assert!(matches!(
                V8CoupledOwnedState::parse_strict(&bytes, &target),
                Err(V8StateError::Schema(_))
            ));
        }
    }

    #[test]
    fn no_default_is_available_is_a_compile_time_surface() {
        fn requires_explicit_state(_: &V8CoupledOwnedState) {}
        let (_, state) = migrated_fixture();
        requires_explicit_state(&state);
    }

    #[test]
    fn accepted_state_restart_round_trip_preserves_lineage() {
        let (target, mut state) = migrated_fixture();
        state.last_transaction_id = 11;
        for shared in state.strata.values_mut() {
            shared.last_transaction_id = 11;
        }
        for lane in state.occupancies.values_mut() {
            lane.last_accepted_transaction_id = Some(11);
        }
        state.state_sha256 = state.canonical_sha256();
        state.validate(&target).expect("accepted state");
        let bytes = serde_json::to_vec(&state).expect("restart serialize");
        let restarted = V8CoupledOwnedState::parse_strict(&bytes, &target).expect("restart parse");
        assert_eq!(restarted, state);
    }

    #[test]
    fn migration_preserves_non_air_v7_payload_exactly() {
        let (source, state, mut target) = migratable_fixture();
        let provisional =
            migrate_one_state(&state, &target, V7ToV8Snapshot::Initial).expect("migration");
        target.initial_state_sha256 = provisional.state_sha256.clone();
        let V7ToV8MigrationResult::Complete(migration) =
            migrate_v7_snapshot(&source, &state, &state, &target).expect("migration")
        else {
            panic!("complete")
        };
        assert_eq!(migration.state.strata, state.strata);
        for (identity, old) in &state.occupancies {
            let new = migration.state.occupancies.get(identity).expect("lane");
            assert_eq!(new, &V8OccupancyState::from(old));
        }
    }
}
