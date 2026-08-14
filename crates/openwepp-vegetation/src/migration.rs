//! Explicit, offline migration boundaries for vegetation definitions and state.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, SoilLayerId, StratumId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::carbon_nitrogen::{ElementPool, MaterialTransfer, Tissue, TissuePool};
use crate::occupancy_state::{OccupancyState, OccupancyStateError, OccupancyStateLanes};
use crate::{
    CoupledOwnedState, MODEL_SHA256, PhenologyPhase, StratumConfiguration, StratumSharedState,
    TopologyTile, VegetationConfiguration,
};

mod v5_to_v6;
mod v6_to_v7;
#[cfg(test)]
pub(crate) use v5_to_v6::validate_v5_initial_fixture;
pub use v5_to_v6::{
    IdentityBoundNumericalFailureDiagnostics, V5ToV6Migration, V5ToV6MigrationError,
    migrate_v5_snapshot,
};
pub use v6_to_v7::{
    V6ToV7ElementIdentity, V6ToV7Migration, V6ToV7MigrationError, V6ToV7MigrationField,
    V6ToV7MigrationReport, V6ToV7MigrationResult, V6ToV7PoolIdentity, V6ToV7UnresolvedField,
    migrate_v6_snapshot,
};

/// Immutable identity of the historical state schema accepted by this module.
pub const V1_MODEL_SHA256: &str =
    "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157";
/// Immutable identity of the historical V2 topology/state definition.
pub const V2_MODEL_SHA256: &str =
    "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3";
/// Immutable identity of the historical V3 constitutive definition.
pub const V3_MODEL_SHA256: &str =
    "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852";
/// Immutable identity of the historical V4 shared-state definition.
pub const V4_MODEL_SHA256: &str =
    "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437";
/// Immutable identity of the historical V5 capped-pass definition.
pub const V5_MODEL_SHA256: &str =
    "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3";
/// Immutable identity of the historical V6 diagnostic-portability definition.
pub const V6_MODEL_SHA256: &str =
    "a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426";
/// Version of the offline, non-runtime `RHESSys` definition mapping table.
pub const RHESSYS_MAPPING_VERSION: &str = "RHESSYS_TO_OPENWEPP_C3_WOODY_V3_V1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RhessysSource {
    pub source_path: String,
    pub raw_bytes: String,
    pub fields: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct FieldProvenance {
    pub canonical_field: String,
    pub raw_field: Option<String>,
    pub source_sha256: String,
}

/// Definition-mapping receipt. It is never accepted as executable state.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct RhessysMigrationReport {
    pub mapping_version: String,
    pub source_path: String,
    pub source_sha256: String,
    pub mapped: BTreeMap<String, serde_json::Value>,
    pub provenance: Vec<FieldProvenance>,
    pub unresolved_required_fields: Vec<String>,
    pub unresolved_occupancy_numerical_fields: Vec<OccupancyFieldRequirement>,
    pub canonical_configuration_sha256: Option<String>,
}

/// One required V3 numerical field at one exact occupancy.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OccupancyFieldRequirement {
    pub occupancy_id: OccupancyId,
    pub field: OccupancyMigrationField,
}

/// V3 occupancy fields whose values cannot be synthesized during migration.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OccupancyMigrationField {
    BetaHyd,
    CanopyAirSpecificHumidityKgKg,
    CanopyAirTemperatureK,
    CanopyLiquidKgH2oM2TileGround,
    DryStemTemperatureK,
    LastAcceptedTransactionId,
    RootNodePotentialMm,
    ShadeCiPa,
    ShadeLeafPotentialMm,
    ShadeLeafTemperatureK,
    StemPotentialMm,
    SunCiPa,
    SunLeafPotentialMm,
    SunLeafTemperatureK,
    WetSurfaceTemperatureK,
}

const WARM_START_FIELDS: [OccupancyMigrationField; 14] = [
    OccupancyMigrationField::BetaHyd,
    OccupancyMigrationField::CanopyAirSpecificHumidityKgKg,
    OccupancyMigrationField::CanopyAirTemperatureK,
    OccupancyMigrationField::DryStemTemperatureK,
    OccupancyMigrationField::LastAcceptedTransactionId,
    OccupancyMigrationField::RootNodePotentialMm,
    OccupancyMigrationField::ShadeCiPa,
    OccupancyMigrationField::ShadeLeafPotentialMm,
    OccupancyMigrationField::ShadeLeafTemperatureK,
    OccupancyMigrationField::StemPotentialMm,
    OccupancyMigrationField::SunCiPa,
    OccupancyMigrationField::SunLeafPotentialMm,
    OccupancyMigrationField::SunLeafTemperatureK,
    OccupancyMigrationField::WetSurfaceTemperatureK,
];

/// Maps an offline `RHESSys` evidence object without admitting `RHESSys` aliases to
/// the strict runtime parser.
#[must_use]
pub fn migrate_definition_fields(
    source: &RhessysSource,
    supplements: &BTreeMap<String, serde_json::Value>,
    required: &[String],
    mapping: &BTreeMap<String, String>,
) -> RhessysMigrationReport {
    migrate(source, supplements, required, mapping, &BTreeSet::new())
}

/// V3 definition mapping with an exhaustive declaration of caller-required
/// occupancy numerical fields. No occupancy value is sourced or defaulted.
#[must_use]
pub fn migrate(
    source: &RhessysSource,
    supplements: &BTreeMap<String, serde_json::Value>,
    required: &[String],
    mapping: &BTreeMap<String, String>,
    expected_occupancies: &BTreeSet<OccupancyId>,
) -> RhessysMigrationReport {
    let source_sha = format!("{:x}", Sha256::digest(source.raw_bytes.as_bytes()));
    let mut mapped = BTreeMap::new();
    let mut provenance = Vec::new();
    for (raw, canonical) in mapping {
        if let Some(value) = source.fields.get(raw) {
            mapped.insert(canonical.clone(), value.clone());
            provenance.push(FieldProvenance {
                canonical_field: canonical.clone(),
                raw_field: Some(raw.clone()),
                source_sha256: source_sha.clone(),
            });
        }
    }
    for (key, value) in supplements {
        mapped.insert(key.clone(), value.clone());
        provenance.push(FieldProvenance {
            canonical_field: key.clone(),
            raw_field: None,
            source_sha256: source_sha.clone(),
        });
    }
    let unresolved_required_fields = required
        .iter()
        .filter(|key| !mapped.contains_key(*key))
        .cloned()
        .collect::<Vec<_>>();
    let unresolved_occupancy_numerical_fields = expected_occupancies
        .iter()
        .flat_map(|occupancy_id| {
            WARM_START_FIELDS
                .into_iter()
                .chain([OccupancyMigrationField::CanopyLiquidKgH2oM2TileGround])
                .map(|field| OccupancyFieldRequirement {
                    occupancy_id: occupancy_id.clone(),
                    field,
                })
        })
        .collect::<Vec<_>>();
    let canonical_configuration_sha256 = if unresolved_required_fields.is_empty()
        && unresolved_occupancy_numerical_fields.is_empty()
    {
        serde_json::to_vec(&mapped)
            .ok()
            .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
    } else {
        None
    };
    RhessysMigrationReport {
        mapping_version: RHESSYS_MAPPING_VERSION.into(),
        source_path: source.source_path.clone(),
        source_sha256: source_sha,
        mapped,
        provenance,
        unresolved_required_fields,
        unresolved_occupancy_numerical_fields,
        canonical_configuration_sha256,
    }
}

/// Historical V1 shared stratum state. This is the sole location where the old
/// shared liquid and numerical representation remains expressible.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V1StratumState {
    pub canopy_liquid: f64,
    pub psi_root_mm: f64,
    pub psi_stem_mm: f64,
    pub psi_sun_mm: f64,
    pub psi_shade_mm: f64,
    pub tissues: BTreeMap<Tissue, TissuePool>,
    pub retranslocation_n: f64,
    pub nsc_c: f64,
    pub xs_c: f64,
    pub standing_dead: ElementPool,
    pub standing_dead_dm: f64,
    pub phase: PhenologyPhase,
    pub onset_remaining_s: f64,
    pub offset_remaining_s: f64,
    pub previous_leaf_offset_flux: f64,
    pub previous_root_offset_flux: f64,
    pub previous_gsi: f64,
    pub pending_transfers: Vec<MaterialTransfer>,
    pub t10_k: f64,
    pub leaf_area: f64,
    pub root_area: f64,
    pub stem_area: f64,
    pub last_transaction_id: u128,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V1CoupledOwnedState {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub state_sha256: String,
    pub strata: BTreeMap<StratumId, V1StratumState>,
    pub last_transaction_id: u128,
}

/// Historical V3 configuration DTO. The constitutive/configuration fields are
/// unchanged in V4, but V3 identity bytes are never admitted by the V4 parser.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V3VegetationConfiguration {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub initial_state_sha256: String,
    pub area_m2: f64,
    pub timestamp: String,
    pub dt_s: f64,
    pub topology_tiles: Vec<TopologyTile>,
    pub strata: Vec<StratumConfiguration>,
}

impl V3VegetationConfiguration {
    /// Parses the historical schema without treating V3 bytes as executable V4.
    pub fn parse_strict(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    fn canonical_sha256(&self) -> Result<String, serde_json::Error> {
        let mut canonical = self.clone();
        canonical.configuration_sha256.clear();
        canonical.initial_state_sha256.clear();
        serde_json::to_vec(&canonical).map(|bytes| format!("{:x}", Sha256::digest(bytes)))
    }
}

/// Historical V3 shared state. These two offset fields are retained here only
/// so their exact source bytes can be validated and then removed.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V3StratumSharedState {
    #[serde(with = "v3_tissue_map")]
    pub tissues: BTreeMap<Tissue, TissuePool>,
    pub retranslocation_n: f64,
    pub nsc_c: f64,
    pub xs_c: f64,
    pub standing_dead: ElementPool,
    pub standing_dead_dm: f64,
    pub phase: PhenologyPhase,
    pub onset_remaining_s: f64,
    pub offset_remaining_s: f64,
    pub previous_leaf_offset_flux: f64,
    pub previous_root_offset_flux: f64,
    pub previous_gsi: f64,
    pub pending_transfers: Vec<MaterialTransfer>,
    pub t10_k: f64,
    pub leaf_area: f64,
    pub root_area: f64,
    pub stem_area: f64,
    pub last_transaction_id: u128,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V3CoupledOwnedState {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub state_sha256: String,
    #[serde(with = "v3_stratum_state_map")]
    pub strata: BTreeMap<StratumId, V3StratumSharedState>,
    #[serde(with = "v3_occupancy_state_map")]
    pub occupancies: BTreeMap<OccupancyId, OccupancyState>,
    pub last_transaction_id: u128,
}

mod v3_stratum_state_map {
    use std::collections::BTreeMap;
    use std::fmt;

    use openwepp_kernel_contract::StratumId;
    use serde::{
        Deserializer, Serializer,
        de::{Error as _, MapAccess, Visitor},
    };

    use super::V3StratumSharedState;

    pub(super) fn serialize<S>(
        states: &BTreeMap<StratumId, V3StratumSharedState>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(states.iter())
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<StratumId, V3StratumSharedState>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StateVisitor;
        impl<'de> Visitor<'de> for StateVisitor {
            type Value = BTreeMap<StratumId, V3StratumSharedState>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a V3 stratum state map with unique identities")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut states = BTreeMap::new();
                while let Some((stratum_id, state)) =
                    map.next_entry::<StratumId, V3StratumSharedState>()?
                {
                    if states.insert(stratum_id.clone(), state).is_some() {
                        return Err(A::Error::custom(format!(
                            "duplicate V3 stratum identity {stratum_id:?}"
                        )));
                    }
                }
                Ok(states)
            }
        }
        deserializer.deserialize_map(StateVisitor)
    }
}

mod v3_tissue_map {
    use std::collections::BTreeMap;
    use std::fmt;

    use serde::{
        Deserializer, Serializer,
        de::{Error as _, MapAccess, Visitor},
    };

    use crate::carbon_nitrogen::{Tissue, TissuePool};

    pub(super) fn serialize<S>(
        tissues: &BTreeMap<Tissue, TissuePool>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(tissues.iter())
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<Tissue, TissuePool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TissueVisitor;
        impl<'de> Visitor<'de> for TissueVisitor {
            type Value = BTreeMap<Tissue, TissuePool>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a V3 tissue map with unique identities")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut tissues = BTreeMap::new();
                while let Some((tissue, pool)) = map.next_entry::<Tissue, TissuePool>()? {
                    if tissues.insert(tissue, pool).is_some() {
                        return Err(A::Error::custom(format!(
                            "duplicate V3 tissue identity {tissue:?}"
                        )));
                    }
                }
                Ok(tissues)
            }
        }
        deserializer.deserialize_map(TissueVisitor)
    }
}

impl V3CoupledOwnedState {
    /// Parses the historical V3 array-of-pairs occupancy encoding and rejects
    /// duplicate identities before a map can overwrite them.
    pub fn parse_strict(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    fn canonical_sha256(&self) -> Result<String, serde_json::Error> {
        let mut canonical = self.clone();
        canonical.state_sha256.clear();
        serde_json::to_vec(&canonical).map(|bytes| format!("{:x}", Sha256::digest(bytes)))
    }
}

mod v3_occupancy_state_map {
    use std::collections::BTreeMap;

    use openwepp_kernel_contract::OccupancyId;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

    use crate::occupancy_state::OccupancyState;

    pub(super) fn serialize<S>(
        lanes: &BTreeMap<OccupancyId, OccupancyState>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        lanes.iter().collect::<Vec<_>>().serialize(serializer)
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<OccupancyId, OccupancyState>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pairs = Vec::<(OccupancyId, OccupancyState)>::deserialize(deserializer)?;
        let mut lanes = BTreeMap::new();
        for (occupancy_id, lane) in pairs {
            if lanes.insert(occupancy_id.clone(), lane).is_some() {
                return Err(D::Error::custom(format!(
                    "duplicate V3 occupancy identity {occupancy_id:?}"
                )));
            }
        }
        Ok(lanes)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V3ToV4MigrationIssue {
    InvalidV3ModelIdentity,
    InvalidV3ConfigurationDigest,
    InvalidV3StateDigest,
    V3V4ConfigurationMismatch,
    InvalidV4Configuration,
    MissingStratum,
    ExtraStratum,
    MissingOccupancy,
    ExtraOccupancy,
    InvalidTransactionLineage,
    InvalidSharedState,
    InvalidLegacyOffset,
    InvalidDisplayedAreaCache,
    InvalidOccupancyState,
    InvalidPendingTransfer,
    InvalidTissueIdentity,
    TargetStateRejected,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct V3ToV4UnresolvedField {
    pub stratum_id: Option<StratumId>,
    pub occupancy_id: Option<OccupancyId>,
    pub field: String,
    pub issue: V3ToV4MigrationIssue,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct V3ToV4MigrationReport {
    pub from_model_definition_sha256: String,
    pub to_model_definition_sha256: String,
    pub unresolved: Vec<V3ToV4UnresolvedField>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum V3ToV4MigrationResult {
    Complete(CoupledOwnedState),
    Incomplete(V3ToV4MigrationReport),
}

/// Typed failures for the exact identity-only V4-to-V5 migration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum V4ToV5MigrationError {
    InvalidV4ConfigurationIdentity,
    InvalidV4ConfigurationDigest,
    InvalidV4StateIdentity,
    InvalidV4StateDigest,
    InvalidV4StateLineage,
    InvalidV5Configuration,
    ConfigurationPayloadMismatch,
    TargetStateRejected,
}

/// Rebinds a complete V4 state to V5 without changing any state payload field.
///
/// V5 imports the V4 configuration and state schema unchanged. This offline
/// boundary therefore validates the complete V4 receipts first, requires an
/// independently digest-bound V5 configuration with byte-equivalent payload,
/// and changes only the three identity digests.
pub fn migrate_v4_state(
    source_configuration: &VegetationConfiguration,
    source: &CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
) -> Result<CoupledOwnedState, V4ToV5MigrationError> {
    if source_configuration.model_definition_sha256 != V4_MODEL_SHA256 {
        return Err(V4ToV5MigrationError::InvalidV4ConfigurationIdentity);
    }
    if source_configuration.canonical_sha256().ok().as_ref()
        != Some(&source_configuration.configuration_sha256)
    {
        return Err(V4ToV5MigrationError::InvalidV4ConfigurationDigest);
    }
    if source.model_definition_sha256 != V4_MODEL_SHA256
        || source.configuration_sha256 != source_configuration.configuration_sha256
    {
        return Err(V4ToV5MigrationError::InvalidV4StateIdentity);
    }
    if source.canonical_sha256().ok().as_ref() != Some(&source.state_sha256) {
        return Err(V4ToV5MigrationError::InvalidV4StateDigest);
    }
    if source.last_transaction_id == 0
        && source_configuration.initial_state_sha256 != source.state_sha256
    {
        return Err(V4ToV5MigrationError::InvalidV4StateLineage);
    }
    if !valid_historical_configuration(target_configuration, V5_MODEL_SHA256) {
        return Err(V4ToV5MigrationError::InvalidV5Configuration);
    }
    if !v4_v5_configuration_payload_matches(source_configuration, target_configuration) {
        return Err(V4ToV5MigrationError::ConfigurationPayloadMismatch);
    }

    let mut migrated = source.clone();
    migrated.model_definition_sha256 = V5_MODEL_SHA256.into();
    migrated
        .configuration_sha256
        .clone_from(&target_configuration.configuration_sha256);
    migrated.state_sha256 = migrated
        .canonical_sha256()
        .map_err(|_| V4ToV5MigrationError::TargetStateRejected)?;
    if !valid_historical_state(&migrated, target_configuration, V5_MODEL_SHA256) {
        return Err(V4ToV5MigrationError::TargetStateRejected);
    }
    Ok(migrated)
}

fn v4_v5_configuration_payload_matches(
    source: &VegetationConfiguration,
    target: &VegetationConfiguration,
) -> bool {
    identity_only_configuration_payload_matches(source, target)
}

fn identity_only_configuration_payload_matches(
    source: &VegetationConfiguration,
    target: &VegetationConfiguration,
) -> bool {
    let Ok(mut source_value) = serde_json::to_value(source) else {
        return false;
    };
    let Ok(mut target_value) = serde_json::to_value(target) else {
        return false;
    };
    for value in [&mut source_value, &mut target_value] {
        if let Some(object) = value.as_object_mut() {
            object.remove("model_definition_sha256");
            object.remove("configuration_sha256");
            object.remove("initial_state_sha256");
        }
    }
    match (
        serde_json::to_vec(&source_value),
        serde_json::to_vec(&target_value),
    ) {
        (Ok(source_bytes), Ok(target_bytes)) => source_bytes == target_bytes,
        _ => false,
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationIssue {
    InvalidV1ModelIdentity,
    MissingV1Stratum,
    ExtraV1Stratum,
    MissingWarmStart,
    ExtraWarmStart,
    NonNullWarmStartTransaction,
    InvalidWarmStart,
    InvalidV1Liquid,
    UnresolvedMultiTileLiquid,
    AmbiguousV2LayerRootWarmStarts,
    InvalidV2ModelIdentity,
    MissingV2Occupancy,
    ExtraV2Occupancy,
    V2RootLayerIdentity,
    InvalidV3Configuration,
    SuccessorMigrationRequired,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UnresolvedMigrationField {
    pub occupancy_id: Option<OccupancyId>,
    pub stratum_id: Option<StratumId>,
    pub field: OccupancyMigrationField,
    pub issue: MigrationIssue,
}

/// Exhaustive, deterministically ordered V1 state migration failure receipt.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MigrationReport {
    pub from_model_definition_sha256: String,
    pub to_model_definition_sha256: String,
    pub unresolved: Vec<UnresolvedMigrationField>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MigrationResult {
    Complete(CoupledOwnedState),
    Incomplete(MigrationReport),
}

/// Compatibility name that makes the source schema explicit at call sites.
pub type V1StateMigration = MigrationResult;

/// Historical V2 occupancy state retained only as an offline migration DTO.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V2OccupancyState {
    pub beta_hyd: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub canopy_air_temperature_k: f64,
    pub canopy_liquid_kg_h2o_m2_tile_ground: f64,
    pub dry_stem_temperature_k: f64,
    pub last_accepted_transaction_id: Option<u128>,
    pub root_potential_mm_by_layer: Vec<(SoilLayerId, f64)>,
    pub shade_ci_pa: f64,
    pub shade_leaf_potential_mm: f64,
    pub shade_leaf_temperature_k: f64,
    pub stem_potential_mm: f64,
    pub sun_ci_pa: f64,
    pub sun_leaf_potential_mm: f64,
    pub sun_leaf_temperature_k: f64,
    pub wet_surface_temperature_k: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V2OccupancyStateSet {
    pub model_definition_sha256: String,
    #[serde(with = "v2_occupancy_state_map")]
    pub occupancies: BTreeMap<OccupancyId, V2OccupancyState>,
}

impl V2OccupancyStateSet {
    /// Parses the historical array-of-pairs representation without allowing
    /// duplicate occupancy identities to be overwritten by a map decoder.
    pub fn parse_strict(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

mod v2_occupancy_state_map {
    use std::collections::BTreeMap;

    use openwepp_kernel_contract::OccupancyId;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

    use super::V2OccupancyState;

    pub(super) fn serialize<S>(
        lanes: &BTreeMap<OccupancyId, V2OccupancyState>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        lanes.iter().collect::<Vec<_>>().serialize(serializer)
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<OccupancyId, V2OccupancyState>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pairs = Vec::<(OccupancyId, V2OccupancyState)>::deserialize(deserializer)?;
        let mut lanes = BTreeMap::new();
        for (occupancy_id, lane) in pairs {
            if lanes.insert(occupancy_id.clone(), lane).is_some() {
                return Err(D::Error::custom(format!(
                    "duplicate V2 occupancy identity {occupancy_id:?}"
                )));
            }
        }
        Ok(lanes)
    }
}

/// Result of migrating historical V2 occupancy lanes to the V3 common root node.
#[derive(Clone, Debug, PartialEq)]
pub enum V2OccupancyMigration {
    Complete(OccupancyStateLanes),
    Incomplete(MigrationReport),
}

/// Migrates V2 root warm starts only when every layer carries identical bits.
#[must_use]
pub fn migrate_v2_occupancy_lanes(
    source: &V2OccupancyStateSet,
    configuration: &VegetationConfiguration,
    expected_previous_transaction_id: Option<u128>,
) -> V2OccupancyMigration {
    let mut migrated = OccupancyStateLanes::new();
    let mut unresolved = v2_identity_issues(source, configuration);
    unresolved.push(UnresolvedMigrationField {
        occupancy_id: None,
        stratum_id: None,
        field: OccupancyMigrationField::RootNodePotentialMm,
        issue: MigrationIssue::SuccessorMigrationRequired,
    });
    if configuration.validate().is_err() {
        unresolved.push(UnresolvedMigrationField {
            occupancy_id: None,
            stratum_id: None,
            field: OccupancyMigrationField::RootNodePotentialMm,
            issue: MigrationIssue::InvalidV3Configuration,
        });
    }
    for (occupancy_id, lane) in &source.occupancies {
        if !lane.root_potential_mm_by_layer.is_empty()
            && !v2_layer_identity_matches(occupancy_id, lane, configuration)
        {
            unresolved.push(occupancy_issue(
                occupancy_id,
                OccupancyMigrationField::RootNodePotentialMm,
                MigrationIssue::V2RootLayerIdentity,
            ));
        }
        migrate_v2_lane(
            occupancy_id,
            lane,
            expected_previous_transaction_id,
            &mut migrated,
            &mut unresolved,
        );
    }
    unresolved.sort();
    unresolved.dedup();
    if unresolved.is_empty() {
        V2OccupancyMigration::Complete(migrated)
    } else {
        V2OccupancyMigration::Incomplete(MigrationReport {
            from_model_definition_sha256: source.model_definition_sha256.clone(),
            to_model_definition_sha256: V3_MODEL_SHA256.into(),
            unresolved,
        })
    }
}

fn v2_identity_issues(
    source: &V2OccupancyStateSet,
    configuration: &VegetationConfiguration,
) -> Vec<UnresolvedMigrationField> {
    let mut unresolved = Vec::new();
    if source.model_definition_sha256 != V2_MODEL_SHA256 {
        unresolved.push(UnresolvedMigrationField {
            occupancy_id: None,
            stratum_id: None,
            field: OccupancyMigrationField::RootNodePotentialMm,
            issue: MigrationIssue::InvalidV2ModelIdentity,
        });
    }
    let expected = configuration.expected_occupancies();
    for occupancy_id in &expected {
        if !source.occupancies.contains_key(occupancy_id) {
            unresolved.push(occupancy_issue(
                occupancy_id,
                OccupancyMigrationField::RootNodePotentialMm,
                MigrationIssue::MissingV2Occupancy,
            ));
        }
    }
    for occupancy_id in source
        .occupancies
        .keys()
        .filter(|occupancy_id| !expected.contains(*occupancy_id))
    {
        unresolved.push(occupancy_issue(
            occupancy_id,
            OccupancyMigrationField::RootNodePotentialMm,
            MigrationIssue::ExtraV2Occupancy,
        ));
    }
    unresolved
}

fn v2_layer_identity_matches(
    occupancy_id: &OccupancyId,
    lane: &V2OccupancyState,
    configuration: &VegetationConfiguration,
) -> bool {
    let expected_layers = configuration
        .strata
        .iter()
        .find(|stratum| stratum.stratum_id == occupancy_id.stratum_id)
        .map(|stratum| {
            let mut layers = stratum
                .root_layers
                .iter()
                .map(|root| root.layer_id.clone())
                .collect::<Vec<_>>();
            layers.sort();
            layers
        });
    let found_layers = lane
        .root_potential_mm_by_layer
        .iter()
        .map(|(layer, _)| layer.clone())
        .collect::<Vec<_>>();
    expected_layers.as_ref() == Some(&found_layers)
}

fn migrate_v2_lane(
    occupancy_id: &OccupancyId,
    lane: &V2OccupancyState,
    expected_previous_transaction_id: Option<u128>,
    migrated: &mut OccupancyStateLanes,
    unresolved: &mut Vec<UnresolvedMigrationField>,
) {
    let common = lane
        .root_potential_mm_by_layer
        .first()
        .map(|entry| entry.1)
        .filter(|value| value.is_finite())
        .filter(|first| {
            lane.root_potential_mm_by_layer
                .iter()
                .all(|(_, value)| value.is_finite() && value.to_bits() == first.to_bits())
        });
    let Some(root_node_potential_mm) = common else {
        unresolved.push(occupancy_issue(
            occupancy_id,
            OccupancyMigrationField::RootNodePotentialMm,
            MigrationIssue::AmbiguousV2LayerRootWarmStarts,
        ));
        return;
    };
    let candidate = OccupancyState {
        beta_hyd: lane.beta_hyd,
        canopy_air_specific_humidity_kg_kg: lane.canopy_air_specific_humidity_kg_kg,
        canopy_air_temperature_k: lane.canopy_air_temperature_k,
        canopy_liquid_kg_h2o_m2_tile_ground: lane.canopy_liquid_kg_h2o_m2_tile_ground,
        dry_stem_temperature_k: lane.dry_stem_temperature_k,
        last_accepted_transaction_id: lane.last_accepted_transaction_id,
        root_node_potential_mm,
        shade_ci_pa: lane.shade_ci_pa,
        shade_leaf_potential_mm: lane.shade_leaf_potential_mm,
        shade_leaf_temperature_k: lane.shade_leaf_temperature_k,
        stem_potential_mm: lane.stem_potential_mm,
        sun_ci_pa: lane.sun_ci_pa,
        sun_leaf_potential_mm: lane.sun_leaf_potential_mm,
        sun_leaf_temperature_k: lane.sun_leaf_temperature_k,
        wet_surface_temperature_k: lane.wet_surface_temperature_k,
    };
    if let Err(error) = candidate.validate(expected_previous_transaction_id) {
        unresolved.push(occupancy_issue(
            occupancy_id,
            field_for_state_error(&error),
            MigrationIssue::InvalidWarmStart,
        ));
    } else {
        migrated.insert(occupancy_id.clone(), candidate);
    }
}

/// Performs the authority-admitted, removal-only V3-to-V4 migration.
///
/// No partial V4 state is returned. The complete historical source and the
/// caller-supplied V4 configuration are validated before retained fields are
/// copied, the two unowned offset values are discarded, and the V4 digest is
/// bound.
#[must_use]
pub fn migrate_v3_state(
    source_configuration: &V3VegetationConfiguration,
    source: &V3CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
) -> V3ToV4MigrationResult {
    let mut unresolved = Vec::new();
    validate_v3_migration_identities(
        source_configuration,
        source,
        target_configuration,
        &mut unresolved,
    );
    validate_v3_membership(source_configuration, source, &mut unresolved);
    let expected_previous = (source.last_transaction_id != 0).then_some(source.last_transaction_id);
    for (occupancy_id, lane) in &source.occupancies {
        if lane.validate(expected_previous).is_err() {
            unresolved.push(v3_occupancy_issue(
                occupancy_id,
                "occupancy_state",
                V3ToV4MigrationIssue::InvalidOccupancyState,
            ));
        }
    }
    for (stratum_id, state) in &source.strata {
        validate_v3_shared_state(
            stratum_id,
            state,
            source.last_transaction_id,
            target_configuration,
            &mut unresolved,
        );
    }
    unresolved.sort();
    unresolved.dedup();
    if !unresolved.is_empty() {
        return incomplete_v3_migration(source, unresolved);
    }
    construct_v4_migration(source, target_configuration)
}

fn validate_v3_migration_identities(
    source_configuration: &V3VegetationConfiguration,
    source: &V3CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
    unresolved: &mut Vec<V3ToV4UnresolvedField>,
) {
    push_global_issue(
        unresolved,
        source_configuration.model_definition_sha256 != V3_MODEL_SHA256,
        "configuration.model_definition_sha256",
        V3ToV4MigrationIssue::InvalidV3ModelIdentity,
    );
    push_global_issue(
        unresolved,
        source.model_definition_sha256 != V3_MODEL_SHA256,
        "state.model_definition_sha256",
        V3ToV4MigrationIssue::InvalidV3ModelIdentity,
    );
    push_global_issue(
        unresolved,
        source_configuration.canonical_sha256().ok().as_ref()
            != Some(&source_configuration.configuration_sha256),
        "configuration.configuration_sha256",
        V3ToV4MigrationIssue::InvalidV3ConfigurationDigest,
    );
    push_global_issue(
        unresolved,
        source.canonical_sha256().ok().as_ref() != Some(&source.state_sha256),
        "state.state_sha256",
        V3ToV4MigrationIssue::InvalidV3StateDigest,
    );
    push_global_issue(
        unresolved,
        source.configuration_sha256 != source_configuration.configuration_sha256,
        "state.configuration_sha256",
        V3ToV4MigrationIssue::InvalidV3ConfigurationDigest,
    );
    push_global_issue(
        unresolved,
        source.last_transaction_id == 0
            && source_configuration.initial_state_sha256 != source.state_sha256,
        "configuration.initial_state_sha256",
        V3ToV4MigrationIssue::InvalidV3StateDigest,
    );
    push_global_issue(
        unresolved,
        !valid_historical_v4_configuration(target_configuration),
        "target_configuration",
        V3ToV4MigrationIssue::InvalidV4Configuration,
    );
    push_global_issue(
        unresolved,
        !v3_v4_configuration_payload_matches(source_configuration, target_configuration),
        "configuration.constitutive_payload",
        V3ToV4MigrationIssue::V3V4ConfigurationMismatch,
    );
}

fn validate_v3_membership(
    source_configuration: &V3VegetationConfiguration,
    source: &V3CoupledOwnedState,
    unresolved: &mut Vec<V3ToV4UnresolvedField>,
) {
    let expected_strata = source_configuration
        .strata
        .iter()
        .map(|stratum| stratum.stratum_id.clone())
        .collect::<BTreeSet<_>>();
    let source_strata = source.strata.keys().cloned().collect::<BTreeSet<_>>();
    for stratum_id in expected_strata.difference(&source_strata) {
        unresolved.push(v3_shared_issue(
            stratum_id,
            "stratum",
            V3ToV4MigrationIssue::MissingStratum,
        ));
    }
    for stratum_id in source_strata.difference(&expected_strata) {
        unresolved.push(v3_shared_issue(
            stratum_id,
            "stratum",
            V3ToV4MigrationIssue::ExtraStratum,
        ));
    }

    let expected_occupancies = v3_expected_occupancies(source_configuration);
    let source_occupancies = source.occupancies.keys().cloned().collect::<BTreeSet<_>>();
    for occupancy_id in expected_occupancies.difference(&source_occupancies) {
        unresolved.push(v3_occupancy_issue(
            occupancy_id,
            "occupancy",
            V3ToV4MigrationIssue::MissingOccupancy,
        ));
    }
    for occupancy_id in source_occupancies.difference(&expected_occupancies) {
        unresolved.push(v3_occupancy_issue(
            occupancy_id,
            "occupancy",
            V3ToV4MigrationIssue::ExtraOccupancy,
        ));
    }
}

fn construct_v4_migration(
    source: &V3CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
) -> V3ToV4MigrationResult {
    let strata = source
        .strata
        .iter()
        .map(|(stratum_id, state)| (stratum_id.clone(), state.to_v4_shared()))
        .collect();
    let mut migrated = CoupledOwnedState {
        model_definition_sha256: V4_MODEL_SHA256.into(),
        configuration_sha256: target_configuration.configuration_sha256.clone(),
        state_sha256: String::new(),
        strata,
        occupancies: source.occupancies.clone(),
        last_transaction_id: source.last_transaction_id,
    };
    let digest = migrated.canonical_sha256();
    match digest {
        Ok(digest) => migrated.state_sha256 = digest,
        Err(_) => {
            return incomplete_v3_migration(
                source,
                vec![v3_global_issue(
                    "target_state.state_sha256",
                    V3ToV4MigrationIssue::TargetStateRejected,
                )],
            );
        }
    }
    if !valid_historical_v4_state(&migrated, target_configuration) {
        return incomplete_v3_migration(
            source,
            vec![v3_global_issue(
                "target_state",
                V3ToV4MigrationIssue::TargetStateRejected,
            )],
        );
    }
    V3ToV4MigrationResult::Complete(migrated)
}

fn valid_historical_v4_configuration(configuration: &VegetationConfiguration) -> bool {
    valid_historical_configuration(configuration, V4_MODEL_SHA256)
}

fn valid_historical_configuration(
    configuration: &VegetationConfiguration,
    expected_model_sha256: &str,
) -> bool {
    configuration
        .validate_historical(expected_model_sha256)
        .is_ok()
}

fn valid_historical_v4_state(
    state: &CoupledOwnedState,
    configuration: &VegetationConfiguration,
) -> bool {
    valid_historical_state(state, configuration, V4_MODEL_SHA256)
}

fn valid_historical_state(
    state: &CoupledOwnedState,
    configuration: &VegetationConfiguration,
    expected_model_sha256: &str,
) -> bool {
    state
        .validate_historical(configuration, expected_model_sha256)
        .is_ok()
}

impl V3StratumSharedState {
    fn to_v4_shared(&self) -> StratumSharedState {
        StratumSharedState {
            tissues: self.tissues.clone(),
            retranslocation_n: self.retranslocation_n,
            nsc_c: self.nsc_c,
            xs_c: self.xs_c,
            standing_dead: self.standing_dead,
            standing_dead_dm: self.standing_dead_dm,
            phase: self.phase,
            onset_remaining_s: self.onset_remaining_s,
            offset_remaining_s: self.offset_remaining_s,
            previous_gsi: self.previous_gsi,
            pending_transfers: self.pending_transfers.clone(),
            t10_k: self.t10_k,
            leaf_area: self.leaf_area,
            root_area: self.root_area,
            stem_area: self.stem_area,
            last_transaction_id: self.last_transaction_id,
        }
    }
}

fn validate_v3_shared_state(
    stratum_id: &StratumId,
    state: &V3StratumSharedState,
    transaction_id: u128,
    target_configuration: &VegetationConfiguration,
    unresolved: &mut Vec<V3ToV4UnresolvedField>,
) {
    if state.last_transaction_id != transaction_id {
        unresolved.push(v3_shared_issue(
            stratum_id,
            "last_transaction_id",
            V3ToV4MigrationIssue::InvalidTransactionLineage,
        ));
    }
    for (field, value) in [
        ("previous_leaf_offset_flux", state.previous_leaf_offset_flux),
        ("previous_root_offset_flux", state.previous_root_offset_flux),
    ] {
        if !value.is_finite() {
            unresolved.push(v3_shared_issue(
                stratum_id,
                field,
                V3ToV4MigrationIssue::InvalidLegacyOffset,
            ));
        }
    }
    validate_v3_tissues(stratum_id, state, unresolved);
    validate_v3_pending_transfers(stratum_id, state, transaction_id, unresolved);
    validate_v3_area_caches(stratum_id, state, target_configuration, unresolved);
}

fn validate_v3_tissues(
    stratum_id: &StratumId,
    state: &V3StratumSharedState,
    unresolved: &mut Vec<V3ToV4UnresolvedField>,
) {
    let required = BTreeSet::from([
        Tissue::Leaf,
        Tissue::FineRoot,
        Tissue::LiveStem,
        Tissue::DeadStem,
        Tissue::LiveCoarseRoot,
        Tissue::DeadCoarseRoot,
    ]);
    if state.tissues.keys().copied().collect::<BTreeSet<_>>() != required {
        unresolved.push(v3_shared_issue(
            stratum_id,
            "tissues",
            V3ToV4MigrationIssue::InvalidTissueIdentity,
        ));
    }
    let scalars = [
        ("retranslocation_n", state.retranslocation_n, true),
        ("nsc_c", state.nsc_c, true),
        ("xs_c", state.xs_c, false),
        ("standing_dead.carbon", state.standing_dead.carbon, true),
        ("standing_dead.nitrogen", state.standing_dead.nitrogen, true),
        ("standing_dead_dm", state.standing_dead_dm, true),
        ("onset_remaining_s", state.onset_remaining_s, true),
        ("offset_remaining_s", state.offset_remaining_s, true),
        ("previous_gsi", state.previous_gsi, true),
        ("t10_k", state.t10_k, true),
        ("leaf_area", state.leaf_area, true),
        ("root_area", state.root_area, true),
        ("stem_area", state.stem_area, true),
    ];
    for (field, value, nonnegative) in scalars {
        let invalid = !value.is_finite()
            || (nonnegative && value < 0.0)
            || (field == "t10_k" && value <= 0.0)
            || (field == "previous_gsi" && value > 1.0);
        if invalid {
            unresolved.push(v3_shared_issue(
                stratum_id,
                field,
                V3ToV4MigrationIssue::InvalidSharedState,
            ));
        }
    }
    for (tissue, pool) in &state.tissues {
        for (subpool, element, value) in [
            ("display", "carbon", pool.display.carbon),
            ("display", "nitrogen", pool.display.nitrogen),
            ("storage", "carbon", pool.storage.carbon),
            ("storage", "nitrogen", pool.storage.nitrogen),
            ("transfer", "carbon", pool.transfer.carbon),
            ("transfer", "nitrogen", pool.transfer.nitrogen),
        ] {
            if !value.is_finite() || value < 0.0 {
                unresolved.push(v3_shared_issue(
                    stratum_id,
                    &format!("tissues.{tissue:?}.{subpool}.{element}"),
                    V3ToV4MigrationIssue::InvalidSharedState,
                ));
            }
        }
    }
}

fn validate_v3_pending_transfers(
    stratum_id: &StratumId,
    state: &V3StratumSharedState,
    transaction_id: u128,
    unresolved: &mut Vec<V3ToV4UnresolvedField>,
) {
    let mut identities = BTreeSet::new();
    let expected_owner = format!("stratum:{}", stratum_id.as_str());
    for (index, transfer) in state.pending_transfers.iter().enumerate() {
        let duplicate = !identities.insert((
            transfer.transaction_id,
            transfer.owner_id.clone(),
            transfer.proposal_id,
        ));
        if transfer.transaction_id == 0
            || transfer.transaction_id != transaction_id
            || transfer.proposal_id == 0
            || transfer.owner_id.as_str() != expected_owner
            || duplicate
            || [transfer.carbon, transfer.nitrogen, transfer.dry_matter]
                .iter()
                .any(|value| !value.is_finite() || *value < 0.0)
        {
            unresolved.push(v3_shared_issue(
                stratum_id,
                &format!("pending_transfers[{index}]"),
                V3ToV4MigrationIssue::InvalidPendingTransfer,
            ));
        }
    }
}

fn validate_v3_area_caches(
    stratum_id: &StratumId,
    state: &V3StratumSharedState,
    target_configuration: &VegetationConfiguration,
    unresolved: &mut Vec<V3ToV4UnresolvedField>,
) {
    let Some(stratum) = target_configuration
        .strata
        .iter()
        .find(|candidate| candidate.stratum_id == *stratum_id)
    else {
        return;
    };
    let Some(leaf) = state.tissues.get(&Tissue::Leaf) else {
        return;
    };
    let Ok((leaf_area, stem_area, root_area)) =
        crate::transaction::displayed_leaf_derived_areas(leaf.display.carbon, stratum)
    else {
        unresolved.push(v3_shared_issue(
            stratum_id,
            "displayed_area_caches",
            V3ToV4MigrationIssue::InvalidDisplayedAreaCache,
        ));
        return;
    };
    for (field, found, expected) in [
        ("leaf_area", state.leaf_area, leaf_area),
        ("stem_area", state.stem_area, stem_area),
        ("root_area", state.root_area, root_area),
    ] {
        if found.to_bits() != expected.to_bits() {
            unresolved.push(v3_shared_issue(
                stratum_id,
                field,
                V3ToV4MigrationIssue::InvalidDisplayedAreaCache,
            ));
        }
    }
    if leaf_area == 0.0 && leaf.display.nitrogen != 0.0 {
        unresolved.push(v3_shared_issue(
            stratum_id,
            "tissues.leaf.display.nitrogen",
            V3ToV4MigrationIssue::InvalidSharedState,
        ));
    }
}

fn v3_expected_occupancies(config: &V3VegetationConfiguration) -> BTreeSet<OccupancyId> {
    config
        .strata
        .iter()
        .flat_map(|stratum| {
            stratum.tile_ids.iter().map(|tile_id| OccupancyId {
                stratum_id: stratum.stratum_id.clone(),
                tile_id: tile_id.clone(),
            })
        })
        .collect()
}

fn v3_v4_configuration_payload_matches(
    source: &V3VegetationConfiguration,
    target: &VegetationConfiguration,
) -> bool {
    let Ok(mut source_value) = serde_json::to_value(source) else {
        return false;
    };
    let Ok(mut target_value) = serde_json::to_value(target) else {
        return false;
    };
    for value in [&mut source_value, &mut target_value] {
        if let Some(object) = value.as_object_mut() {
            object.remove("model_definition_sha256");
            object.remove("configuration_sha256");
            object.remove("initial_state_sha256");
        }
    }
    source_value == target_value
}

fn push_global_issue(
    unresolved: &mut Vec<V3ToV4UnresolvedField>,
    condition: bool,
    field: &str,
    issue: V3ToV4MigrationIssue,
) {
    if condition {
        unresolved.push(v3_global_issue(field, issue));
    }
}

fn v3_global_issue(field: &str, issue: V3ToV4MigrationIssue) -> V3ToV4UnresolvedField {
    V3ToV4UnresolvedField {
        stratum_id: None,
        occupancy_id: None,
        field: field.into(),
        issue,
    }
}

fn v3_shared_issue(
    stratum_id: &StratumId,
    field: &str,
    issue: V3ToV4MigrationIssue,
) -> V3ToV4UnresolvedField {
    V3ToV4UnresolvedField {
        stratum_id: Some(stratum_id.clone()),
        occupancy_id: None,
        field: field.into(),
        issue,
    }
}

fn v3_occupancy_issue(
    occupancy_id: &OccupancyId,
    field: &str,
    issue: V3ToV4MigrationIssue,
) -> V3ToV4UnresolvedField {
    V3ToV4UnresolvedField {
        stratum_id: Some(occupancy_id.stratum_id.clone()),
        occupancy_id: Some(occupancy_id.clone()),
        field: field.into(),
        issue,
    }
}

fn incomplete_v3_migration(
    source: &V3CoupledOwnedState,
    mut unresolved: Vec<V3ToV4UnresolvedField>,
) -> V3ToV4MigrationResult {
    unresolved.sort();
    unresolved.dedup();
    V3ToV4MigrationResult::Incomplete(V3ToV4MigrationReport {
        from_model_definition_sha256: source.model_definition_sha256.clone(),
        to_model_definition_sha256: V4_MODEL_SHA256.into(),
        unresolved,
    })
}

/// Performs the authority-admitted V1-to-V3 state migration.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn migrate_v1_state(
    source: &V1CoupledOwnedState,
    configuration: &VegetationConfiguration,
    warm_starts: &OccupancyStateLanes,
) -> MigrationResult {
    let expected = configuration.expected_occupancies();
    // The released operation was V1-to-V3. V4 removed two fields carried by
    // the V1 DTO, so invoking this historical API with an executable V4
    // configuration must remain fail-closed and proceed through explicit V3
    // bytes plus `migrate_v3_state`; it may not invent a direct V1-to-V4 map.
    let mut unresolved = vec![UnresolvedMigrationField {
        occupancy_id: None,
        stratum_id: None,
        field: OccupancyMigrationField::CanopyLiquidKgH2oM2TileGround,
        issue: MigrationIssue::SuccessorMigrationRequired,
    }];

    if source.model_definition_sha256 != V1_MODEL_SHA256 {
        unresolved.push(UnresolvedMigrationField {
            occupancy_id: None,
            stratum_id: None,
            field: OccupancyMigrationField::CanopyLiquidKgH2oM2TileGround,
            issue: MigrationIssue::InvalidV1ModelIdentity,
        });
    }

    let expected_strata = configuration
        .strata
        .iter()
        .map(|stratum| stratum.stratum_id.clone())
        .collect::<BTreeSet<_>>();
    let source_strata = source.strata.keys().cloned().collect::<BTreeSet<_>>();
    for stratum_id in expected_strata.difference(&source_strata) {
        unresolved.push(shared_stratum_issue(
            stratum_id,
            MigrationIssue::MissingV1Stratum,
        ));
    }
    for stratum_id in source_strata.difference(&expected_strata) {
        unresolved.push(shared_stratum_issue(
            stratum_id,
            MigrationIssue::ExtraV1Stratum,
        ));
    }

    for occupancy_id in &expected {
        let root_layers = configuration
            .strata
            .iter()
            .find(|stratum| stratum.stratum_id == occupancy_id.stratum_id)
            .map(|stratum| {
                let mut layer_ids = stratum
                    .root_layers
                    .iter()
                    .map(|layer| layer.layer_id.clone())
                    .collect::<Vec<_>>();
                layer_ids.sort();
                layer_ids
            });
        match (warm_starts.get(occupancy_id), root_layers) {
            (None, _) => unresolved.extend(WARM_START_FIELDS.map(|field| {
                occupancy_issue(occupancy_id, field, MigrationIssue::MissingWarmStart)
            })),
            (Some(state), Some(_layers)) => {
                if let Err(error) = state.validate(None) {
                    let issue = if matches!(error, OccupancyStateError::StaleTransaction { .. }) {
                        MigrationIssue::NonNullWarmStartTransaction
                    } else {
                        MigrationIssue::InvalidWarmStart
                    };
                    unresolved.push(occupancy_issue(
                        occupancy_id,
                        field_for_state_error(&error),
                        issue,
                    ));
                }
            }
            (Some(_), None) => {}
        }
    }
    for occupancy_id in warm_starts.keys().filter(|id| !expected.contains(*id)) {
        unresolved.extend(
            WARM_START_FIELDS
                .map(|field| occupancy_issue(occupancy_id, field, MigrationIssue::ExtraWarmStart)),
        );
    }
    let mut migrated_occupancies = warm_starts.clone();
    for stratum in &configuration.strata {
        let Some(v1) = source.strata.get(&stratum.stratum_id) else {
            continue;
        };
        let occupancies = expected
            .iter()
            .filter(|id| id.stratum_id == stratum.stratum_id)
            .collect::<Vec<_>>();
        if !v1.canopy_liquid.is_finite() || v1.canopy_liquid < 0.0 {
            unresolved.push(shared_stratum_issue(
                &stratum.stratum_id,
                MigrationIssue::InvalidV1Liquid,
            ));
        } else if v1.canopy_liquid == 0.0 {
            for occupancy_id in occupancies {
                if let Some(state) = migrated_occupancies.get_mut(occupancy_id) {
                    state.canopy_liquid_kg_h2o_m2_tile_ground = 0.0;
                }
            }
        } else if occupancies.len() == 1 {
            let occupancy_id = occupancies[0];
            if let Some(state) = migrated_occupancies.get_mut(occupancy_id) {
                match configuration.stratum_coverage(&stratum.stratum_id) {
                    Ok(coverage) if coverage.is_finite() && coverage > 0.0 => {
                        state.canopy_liquid_kg_h2o_m2_tile_ground = v1.canopy_liquid / coverage;
                    }
                    _ => unresolved.push(occupancy_issue(
                        occupancy_id,
                        OccupancyMigrationField::CanopyLiquidKgH2oM2TileGround,
                        MigrationIssue::InvalidWarmStart,
                    )),
                }
            }
        } else {
            for occupancy_id in occupancies {
                unresolved.push(occupancy_issue(
                    occupancy_id,
                    OccupancyMigrationField::CanopyLiquidKgH2oM2TileGround,
                    MigrationIssue::UnresolvedMultiTileLiquid,
                ));
            }
        }
    }

    unresolved.sort();
    unresolved.dedup();
    if !unresolved.is_empty() {
        return MigrationResult::Incomplete(MigrationReport {
            from_model_definition_sha256: source.model_definition_sha256.clone(),
            to_model_definition_sha256: V3_MODEL_SHA256.into(),
            unresolved,
        });
    }

    let strata = source
        .strata
        .iter()
        .map(|(id, state)| (id.clone(), state.clone().into_shared()))
        .collect();
    let mut migrated = CoupledOwnedState {
        model_definition_sha256: MODEL_SHA256.into(),
        configuration_sha256: configuration.configuration_sha256.clone(),
        state_sha256: String::new(),
        strata,
        occupancies: migrated_occupancies,
        last_transaction_id: source.last_transaction_id,
    };
    match migrated.canonical_sha256() {
        Ok(digest) => {
            migrated.state_sha256 = digest;
            if migrated.validate(configuration).is_ok() {
                MigrationResult::Complete(migrated)
            } else {
                incomplete_invalid_warm_starts(source, &expected)
            }
        }
        Err(_) => incomplete_invalid_warm_starts(source, &expected),
    }
}

impl V1StratumState {
    fn into_shared(self) -> StratumSharedState {
        StratumSharedState {
            tissues: self.tissues,
            retranslocation_n: self.retranslocation_n,
            nsc_c: self.nsc_c,
            xs_c: self.xs_c,
            standing_dead: self.standing_dead,
            standing_dead_dm: self.standing_dead_dm,
            phase: self.phase,
            onset_remaining_s: self.onset_remaining_s,
            offset_remaining_s: self.offset_remaining_s,
            previous_gsi: self.previous_gsi,
            pending_transfers: self.pending_transfers,
            t10_k: self.t10_k,
            leaf_area: self.leaf_area,
            root_area: self.root_area,
            stem_area: self.stem_area,
            last_transaction_id: self.last_transaction_id,
        }
    }
}

fn incomplete_invalid_warm_starts(
    source: &V1CoupledOwnedState,
    expected: &BTreeSet<OccupancyId>,
) -> MigrationResult {
    MigrationResult::Incomplete(MigrationReport {
        from_model_definition_sha256: source.model_definition_sha256.clone(),
        to_model_definition_sha256: V3_MODEL_SHA256.into(),
        unresolved: expected
            .iter()
            .flat_map(|id| {
                WARM_START_FIELDS
                    .map(|field| occupancy_issue(id, field, MigrationIssue::InvalidWarmStart))
            })
            .collect(),
    })
}

fn occupancy_issue(
    occupancy_id: &OccupancyId,
    field: OccupancyMigrationField,
    issue: MigrationIssue,
) -> UnresolvedMigrationField {
    UnresolvedMigrationField {
        occupancy_id: Some(occupancy_id.clone()),
        stratum_id: Some(occupancy_id.stratum_id.clone()),
        field,
        issue,
    }
}

fn field_for_state_error(error: &OccupancyStateError) -> OccupancyMigrationField {
    match error {
        OccupancyStateError::Domain(field) => match *field {
            "beta_hyd" => OccupancyMigrationField::BetaHyd,
            "canopy_air_specific_humidity_kg_kg" => {
                OccupancyMigrationField::CanopyAirSpecificHumidityKgKg
            }
            "canopy_air_temperature_k" => OccupancyMigrationField::CanopyAirTemperatureK,
            "canopy_liquid_kg_h2o_m2_tile_ground" => {
                OccupancyMigrationField::CanopyLiquidKgH2oM2TileGround
            }
            "dry_stem_temperature_k" => OccupancyMigrationField::DryStemTemperatureK,
            "shade_ci_pa" => OccupancyMigrationField::ShadeCiPa,
            "shade_leaf_potential_mm" => OccupancyMigrationField::ShadeLeafPotentialMm,
            "shade_leaf_temperature_k" => OccupancyMigrationField::ShadeLeafTemperatureK,
            "stem_potential_mm" => OccupancyMigrationField::StemPotentialMm,
            "sun_ci_pa" => OccupancyMigrationField::SunCiPa,
            "sun_leaf_potential_mm" => OccupancyMigrationField::SunLeafPotentialMm,
            "sun_leaf_temperature_k" => OccupancyMigrationField::SunLeafTemperatureK,
            "wet_surface_temperature_k" => OccupancyMigrationField::WetSurfaceTemperatureK,
            _ => OccupancyMigrationField::RootNodePotentialMm,
        },
        OccupancyStateError::StaleTransaction { .. } => {
            OccupancyMigrationField::LastAcceptedTransactionId
        }
        OccupancyStateError::Schema(_) => OccupancyMigrationField::RootNodePotentialMm,
    }
}

fn shared_stratum_issue(stratum_id: &StratumId, issue: MigrationIssue) -> UnresolvedMigrationField {
    UnresolvedMigrationField {
        occupancy_id: None,
        stratum_id: Some(stratum_id.clone()),
        field: OccupancyMigrationField::CanopyLiquidKgH2oM2TileGround,
        issue,
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::RootLayer;
    use crate::occupancy_state::OccupancyState;
    use openwepp_kernel_contract::{ResourceOwnerId, TileId};

    const CONFIG_BYTES: &[u8] =
        include_bytes!("../../../tests/fixtures/c3_woody_v1_diagnostic_configuration.json");
    const STATE_BYTES: &[u8] =
        include_bytes!("../../../tests/fixtures/c3_woody_v1_diagnostic_state.json");
    const V4_CONFIG_BYTES: &[u8] =
        include_bytes!("../../../tests/fixtures/c3_woody_v4_diagnostic_configuration.json");
    const V4_STATE_BYTES: &[u8] =
        include_bytes!("../../../tests/fixtures/c3_woody_v4_diagnostic_state.json");
    const V5_CONFIG_BYTES: &[u8] =
        include_bytes!("../../../tests/fixtures/c3_woody_v5_diagnostic_configuration.json");
    const V5_STATE_BYTES: &[u8] =
        include_bytes!("../../../tests/fixtures/c3_woody_v5_diagnostic_state.json");
    const V4_AUTHORITY_VECTOR_BYTES: &[u8] = include_bytes!(
        "../../../docs/work-packages/20260812-c3-woody-shared-state-authority-001/artifacts/openwepp_c3_woody_v4_vectors.json"
    );
    const V4_AUTHORITY_DEFINITION_BYTES: &[u8] = include_bytes!(
        "../../../docs/work-packages/20260812-c3-woody-shared-state-authority-001/artifacts/openwepp_c3_woody_v4_definition.json"
    );
    const FINAL_V4_AUTHORITY_SHA256: &str =
        "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437";

    fn v4_to_v5_fixture() -> (
        VegetationConfiguration,
        CoupledOwnedState,
        VegetationConfiguration,
        CoupledOwnedState,
    ) {
        let source_configuration =
            serde_json::from_slice(V4_CONFIG_BYTES).expect("historical V4 configuration");
        let source = serde_json::from_slice(V4_STATE_BYTES).expect("historical V4 state");
        let target_configuration: VegetationConfiguration =
            serde_json::from_slice(V5_CONFIG_BYTES).expect("released V5 configuration");
        assert!(valid_historical_configuration(
            &target_configuration,
            V5_MODEL_SHA256
        ));
        let expected: CoupledOwnedState =
            serde_json::from_slice(V5_STATE_BYTES).expect("released V5 state");
        assert!(valid_historical_state(
            &expected,
            &target_configuration,
            V5_MODEL_SHA256
        ));
        (source_configuration, source, target_configuration, expected)
    }

    #[test]
    fn v4_to_v5_changes_only_model_configuration_and_state_identities() {
        let (source_configuration, source, target_configuration, expected) = v4_to_v5_fixture();
        let source_bytes = serde_json::to_vec(&source).expect("V4 source bytes");
        let actual = migrate_v4_state(&source_configuration, &source, &target_configuration)
            .expect("identity-only migration");
        assert_eq!(actual, expected);
        assert_eq!(actual.strata, source.strata);
        assert_eq!(actual.occupancies, source.occupancies);
        assert_eq!(actual.last_transaction_id, source.last_transaction_id);
        assert_eq!(actual.model_definition_sha256, V5_MODEL_SHA256);
        assert_eq!(
            actual.configuration_sha256,
            target_configuration.configuration_sha256
        );
        assert_ne!(actual.state_sha256, source.state_sha256);
        let mut rebound = actual;
        rebound
            .model_definition_sha256
            .clone_from(&source.model_definition_sha256);
        rebound
            .configuration_sha256
            .clone_from(&source.configuration_sha256);
        rebound.state_sha256.clone_from(&source.state_sha256);
        assert_eq!(
            serde_json::to_vec(&rebound).expect("identity-rebound V5 bytes"),
            source_bytes
        );
        assert_eq!(
            serde_json::to_vec(&source).expect("unchanged V4 source bytes"),
            source_bytes
        );
    }

    #[test]
    fn v4_to_v5_rejects_stale_or_mismatched_receipts_without_candidate() {
        let (source_configuration, source, target_configuration, _) = v4_to_v5_fixture();

        let mut wrong_source_identity = source_configuration.clone();
        wrong_source_identity.model_definition_sha256 = MODEL_SHA256.into();
        assert_eq!(
            migrate_v4_state(&wrong_source_identity, &source, &target_configuration),
            Err(V4ToV5MigrationError::InvalidV4ConfigurationIdentity)
        );

        let mut stale_configuration = source_configuration.clone();
        stale_configuration.configuration_sha256 = "0".repeat(64);
        assert_eq!(
            migrate_v4_state(&stale_configuration, &source, &target_configuration),
            Err(V4ToV5MigrationError::InvalidV4ConfigurationDigest)
        );

        let mut stale_state = source.clone();
        stale_state.state_sha256 = "0".repeat(64);
        assert_eq!(
            migrate_v4_state(&source_configuration, &stale_state, &target_configuration),
            Err(V4ToV5MigrationError::InvalidV4StateDigest)
        );

        let mut wrong_state_identity = source.clone();
        wrong_state_identity.model_definition_sha256 = MODEL_SHA256.into();
        assert_eq!(
            migrate_v4_state(
                &source_configuration,
                &wrong_state_identity,
                &target_configuration
            ),
            Err(V4ToV5MigrationError::InvalidV4StateIdentity)
        );

        let mut historical_target = target_configuration.clone();
        historical_target.model_definition_sha256 = V4_MODEL_SHA256.into();
        historical_target.configuration_sha256 = historical_target
            .canonical_sha256()
            .expect("historical target digest");
        assert_eq!(
            migrate_v4_state(&source_configuration, &source, &historical_target),
            Err(V4ToV5MigrationError::InvalidV5Configuration)
        );

        let mut mismatched_target = target_configuration;
        mismatched_target.strata[0].height_m += 1.0;
        mismatched_target.configuration_sha256 = mismatched_target
            .canonical_sha256()
            .expect("mismatched target digest");
        assert_eq!(
            migrate_v4_state(&source_configuration, &source, &mismatched_target),
            Err(V4ToV5MigrationError::ConfigurationPayloadMismatch)
        );
    }

    fn config() -> VegetationConfiguration {
        let mut raw: serde_json::Value =
            serde_json::from_slice(CONFIG_BYTES).expect("configuration JSON");
        raw["strata"][0]
            .as_object_mut()
            .expect("stratum object")
            .remove("rd_leaf_n_rate");
        let mut value: VegetationConfiguration =
            serde_json::from_value(raw).expect("V3 configuration DTO");
        value.model_definition_sha256 = MODEL_SHA256.into();
        value.configuration_sha256.clear();
        value.initial_state_sha256 = "0".repeat(64);
        value.configuration_sha256 = value.canonical_sha256().expect("digest");
        value
    }

    fn two_tile_config() -> VegetationConfiguration {
        let mut value = config();
        let second = TileId::try_new("tile-2").expect("tile ID");
        value.topology_tiles[0].fraction = 0.3;
        value.topology_tiles.push(crate::TopologyTile {
            tile_id: second.clone(),
            fraction: 0.7,
        });
        value.strata[0].tile_ids.push(second);
        value.configuration_sha256.clear();
        value.configuration_sha256 = value.canonical_sha256().expect("digest");
        value
    }

    fn v1_state(liquid: f64) -> V1CoupledOwnedState {
        let mut value: V1CoupledOwnedState =
            serde_json::from_slice(STATE_BYTES).expect("historical V1 state DTO");
        value
            .strata
            .values_mut()
            .next()
            .expect("stratum")
            .canopy_liquid = liquid;
        value
    }

    fn warm_start(_config: &VegetationConfiguration, liquid: f64) -> OccupancyState {
        OccupancyState {
            beta_hyd: 0.7,
            canopy_air_specific_humidity_kg_kg: 0.01,
            canopy_air_temperature_k: 295.0,
            canopy_liquid_kg_h2o_m2_tile_ground: liquid,
            dry_stem_temperature_k: 294.0,
            last_accepted_transaction_id: None,
            root_node_potential_mm: -5000.0,
            shade_ci_pa: 25.0,
            shade_leaf_potential_mm: -6000.0,
            shade_leaf_temperature_k: 295.0,
            stem_potential_mm: -5500.0,
            sun_ci_pa: 26.0,
            sun_leaf_potential_mm: -6200.0,
            sun_leaf_temperature_k: 296.0,
            wet_surface_temperature_k: 294.5,
        }
    }

    fn warm_starts(config: &VegetationConfiguration, liquid: f64) -> OccupancyStateLanes {
        config
            .expected_occupancies()
            .into_iter()
            .map(|id| (id, warm_start(config, liquid)))
            .collect()
    }

    fn v3_to_v4_fixture() -> (
        V3VegetationConfiguration,
        V3CoupledOwnedState,
        VegetationConfiguration,
        CoupledOwnedState,
    ) {
        let mut target_config: VegetationConfiguration =
            serde_json::from_slice(V4_CONFIG_BYTES).expect("released V4 configuration schema");
        target_config.model_definition_sha256 = V4_MODEL_SHA256.into();
        target_config.configuration_sha256.clear();
        target_config.configuration_sha256 = target_config
            .canonical_sha256()
            .expect("current V4 configuration digest");
        let mut target_state: CoupledOwnedState =
            serde_json::from_slice(V4_STATE_BYTES).expect("released V4 state schema");
        target_state.model_definition_sha256 = V4_MODEL_SHA256.into();
        target_state
            .configuration_sha256
            .clone_from(&target_config.configuration_sha256);
        target_state.state_sha256 = target_state
            .canonical_sha256()
            .expect("released V4 state digest under current canonical encoder");
        target_config
            .initial_state_sha256
            .clone_from(&target_state.state_sha256);

        let mut config_value = serde_json::to_value(&target_config).expect("configuration value");
        config_value["model_definition_sha256"] = serde_json::json!(V3_MODEL_SHA256);
        config_value["configuration_sha256"] = serde_json::json!("");
        config_value["initial_state_sha256"] = serde_json::json!("0".repeat(64));
        let mut source_config: V3VegetationConfiguration =
            serde_json::from_value(config_value).expect("historical V3 configuration");
        source_config.configuration_sha256 = source_config
            .canonical_sha256()
            .expect("historical configuration digest");

        let mut state_value = serde_json::to_value(&target_state).expect("state value");
        state_value["model_definition_sha256"] = serde_json::json!(V3_MODEL_SHA256);
        state_value["configuration_sha256"] =
            serde_json::json!(source_config.configuration_sha256.clone());
        state_value["state_sha256"] = serde_json::json!("");
        state_value["occupancies"] = serde_json::Value::Array(
            state_value["occupancies"]
                .as_array()
                .expect("structural V4 occupancies")
                .iter()
                .map(|entry| {
                    serde_json::Value::Array(vec![
                        entry["identity"].clone(),
                        entry["state"].clone(),
                    ])
                })
                .collect(),
        );
        for shared in state_value["strata"]
            .as_object_mut()
            .expect("stratum map")
            .values_mut()
        {
            shared["previous_leaf_offset_flux"] = serde_json::json!(0.125);
            shared["previous_root_offset_flux"] = serde_json::json!(-0.25);
        }
        let mut source_state: V3CoupledOwnedState =
            serde_json::from_value(state_value).expect("historical V3 state");
        source_state.state_sha256 = source_state
            .canonical_sha256()
            .expect("historical state digest");
        source_config
            .initial_state_sha256
            .clone_from(&source_state.state_sha256);
        (source_config, source_state, target_config, target_state)
    }

    fn authority_two_stratum_fixture() -> (
        V3VegetationConfiguration,
        V3CoupledOwnedState,
        VegetationConfiguration,
        CoupledOwnedState,
    ) {
        let vectors: serde_json::Value =
            serde_json::from_slice(V4_AUTHORITY_VECTOR_BYTES).expect("V4 authority vectors");
        let migration = &vectors["v3_to_v4_migration"];
        let v3_whole = &migration["v3_whole_state"];
        let expected_whole = &migration["expected_v4_whole_state"];
        let mut target: VegetationConfiguration =
            serde_json::from_slice(V4_CONFIG_BYTES).expect("released V4 configuration schema");
        target.model_definition_sha256 = V4_MODEL_SHA256.into();
        let tile_id = TileId::try_new("tile-a").expect("tile ID");
        target.topology_tiles[0].tile_id.clone_from(&tile_id);
        let mut canopy = target.strata[0].clone();
        canopy.stratum_id = StratumId::try_new("canopy").expect("stratum ID");
        canopy.vertical_rank = 0;
        canopy.tile_ids = vec![tile_id.clone()];
        canopy.sla_m2_per_kg_c = 5.0;
        canopy.sai_relation = 0.35;
        canopy.root_to_leaf_area = 1.25;
        let mut understory = canopy.clone();
        understory.stratum_id = StratumId::try_new("understory").expect("stratum ID");
        understory.vertical_rank = 1;
        understory.height_m = canopy.height_m * 0.5;
        understory.crown_base_m = 0.0;
        target.strata = vec![canopy, understory];
        target.initial_state_sha256 = "0".repeat(64);
        target.configuration_sha256.clear();
        target.configuration_sha256 = target.canonical_sha256().expect("V4 digest");

        let mut config_value = serde_json::to_value(&target).expect("configuration value");
        config_value["model_definition_sha256"] = serde_json::json!(V3_MODEL_SHA256);
        config_value["configuration_sha256"] = serde_json::json!("");
        let mut source_config: V3VegetationConfiguration =
            serde_json::from_value(config_value).expect("V3 configuration");
        source_config.configuration_sha256 = source_config
            .canonical_sha256()
            .expect("V3 configuration digest");

        let mut source_strata = BTreeMap::new();
        let mut expected_strata = BTreeMap::new();
        for name in ["canopy", "understory"] {
            let id = StratumId::try_new(name).expect("stratum ID");
            source_strata.insert(
                id.clone(),
                serde_json::from_value(v3_whole["strata"][name].clone())
                    .expect("committed authority V3 shared state"),
            );
            expected_strata.insert(
                id,
                serde_json::from_value(expected_whole["strata"][name].clone())
                    .expect("committed authority V4 shared state"),
            );
        }
        let mut occupancies = BTreeMap::new();
        for entry in v3_whole["occupancies"]
            .as_array()
            .expect("structural authority occupancy array")
        {
            let id: OccupancyId = serde_json::from_value(entry["identity"].clone())
                .expect("authority occupancy identity");
            let state: OccupancyState =
                serde_json::from_value(entry["state"].clone()).expect("authority occupancy state");
            assert!(occupancies.insert(id, state).is_none());
        }
        let mut source = V3CoupledOwnedState {
            model_definition_sha256: V3_MODEL_SHA256.into(),
            configuration_sha256: source_config.configuration_sha256.clone(),
            state_sha256: String::new(),
            strata: source_strata,
            occupancies: occupancies.clone(),
            last_transaction_id: 41,
        };
        source.state_sha256 = source.canonical_sha256().expect("V3 state digest");
        source_config
            .initial_state_sha256
            .clone_from(&source.state_sha256);
        let mut expected = CoupledOwnedState {
            model_definition_sha256: FINAL_V4_AUTHORITY_SHA256.into(),
            configuration_sha256: target.configuration_sha256.clone(),
            state_sha256: String::new(),
            strata: expected_strata,
            occupancies,
            last_transaction_id: v3_whole["last_transaction_id"]
                .as_u64()
                .map(u128::from)
                .expect("committed V3 transaction identity"),
        };
        expected.state_sha256 = expected.canonical_sha256().expect("V4 state digest");
        (source_config, source, target, expected)
    }

    #[test]
    fn v3_to_v4_matches_independent_two_stratum_authority_vector() {
        assert_eq!(
            format!("{:x}", Sha256::digest(V4_AUTHORITY_DEFINITION_BYTES)),
            FINAL_V4_AUTHORITY_SHA256
        );
        assert_eq!(V4_MODEL_SHA256, FINAL_V4_AUTHORITY_SHA256);
        let (source_config, source, target_config, expected) = authority_two_stratum_fixture();
        let source_bytes = serde_json::to_vec(&source).expect("source bytes");
        let actual = match migrate_v3_state(&source_config, &source, &target_config) {
            V3ToV4MigrationResult::Complete(actual) => actual,
            V3ToV4MigrationResult::Incomplete(report) => {
                panic!("independent two-stratum authority vector must migrate: {report:?}")
            }
        };
        assert_eq!(actual, expected);
        assert_eq!(actual.strata.len(), 2);
        assert_eq!(actual.occupancies.len(), 2);
        assert_eq!(
            serde_json::to_vec(&source).expect("unchanged source bytes"),
            source_bytes
        );
    }

    #[test]
    fn v3_to_v4_removes_only_offsets_and_binds_exact_target_digest() {
        let (source_config, source, target_config, expected) = v3_to_v4_fixture();
        let actual = match migrate_v3_state(&source_config, &source, &target_config) {
            V3ToV4MigrationResult::Complete(actual) => actual,
            V3ToV4MigrationResult::Incomplete(report) => {
                panic!("released exact migration must complete: {report:?}")
            }
        };
        assert_eq!(actual, expected);
        assert_eq!(actual.model_definition_sha256, V4_MODEL_SHA256);
        assert_eq!(
            actual.configuration_sha256,
            target_config.configuration_sha256
        );
        for (stratum_id, migrated) in &actual.strata {
            let historical = &source.strata[stratum_id];
            assert_eq!(migrated.tissues, historical.tissues);
            assert_eq!(migrated.pending_transfers, historical.pending_transfers);
            assert_eq!(migrated.leaf_area.to_bits(), historical.leaf_area.to_bits());
            assert_eq!(migrated.root_area.to_bits(), historical.root_area.to_bits());
            assert_eq!(migrated.stem_area.to_bits(), historical.stem_area.to_bits());
        }
        assert_eq!(actual.occupancies, source.occupancies);
    }

    #[test]
    fn v3_to_v4_reports_all_invalid_owners_without_partial_state() {
        let (source_config, mut source, target_config, _) = v3_to_v4_fixture();
        for state in source.strata.values_mut() {
            state.leaf_area = f64::from_bits(state.leaf_area.to_bits() ^ 1);
            state.previous_root_offset_flux = f64::NAN;
        }
        for lane in source.occupancies.values_mut() {
            lane.last_accepted_transaction_id = Some(99);
        }
        source.state_sha256 = source.canonical_sha256().expect("poison digest");
        let V3ToV4MigrationResult::Incomplete(report) =
            migrate_v3_state(&source_config, &source, &target_config)
        else {
            panic!("invalid source must not return partial V4 state")
        };
        assert_eq!(
            report
                .unresolved
                .iter()
                .filter(|field| field.issue == V3ToV4MigrationIssue::InvalidDisplayedAreaCache)
                .count(),
            source.strata.len()
        );
        assert_eq!(
            report
                .unresolved
                .iter()
                .filter(|field| field.issue == V3ToV4MigrationIssue::InvalidLegacyOffset)
                .count(),
            source.strata.len()
        );
        assert_eq!(
            report
                .unresolved
                .iter()
                .filter(|field| field.issue == V3ToV4MigrationIssue::InvalidOccupancyState)
                .count(),
            source.occupancies.len()
        );
        assert!(report.unresolved.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn v3_to_v4_rejects_transfer_lineage_identity_duplicates_and_rolls_back() {
        let (source_config, mut source, target_config, _) = authority_two_stratum_fixture();
        let canopy = StratumId::try_new("canopy").expect("stratum ID");
        let transfer = source.strata[&canopy].pending_transfers[0].clone();
        let shared = source.strata.get_mut(&canopy).expect("canopy state");
        shared.pending_transfers[0].transaction_id = 42;
        shared.pending_transfers[0].owner_id =
            ResourceOwnerId::try_new("stratum:understory").expect("owner identity");
        shared.pending_transfers.push(transfer);
        source.state_sha256 = source.canonical_sha256().expect("poison digest");
        let before = serde_json::to_vec(&source).expect("beginning bytes");
        let V3ToV4MigrationResult::Incomplete(report) =
            migrate_v3_state(&source_config, &source, &target_config)
        else {
            panic!("invalid transfer identities must not produce partial state")
        };
        assert!(report.unresolved.iter().any(|field| {
            field.stratum_id.as_ref() == Some(&canopy)
                && field.issue == V3ToV4MigrationIssue::InvalidPendingTransfer
        }));
        assert_eq!(
            serde_json::to_vec(&source).expect("source after failure"),
            before
        );
    }

    #[test]
    fn v3_to_v4_rejects_negative_stores_and_timers_exhaustively() {
        let (source_config, mut source, target_config, _) = authority_two_stratum_fixture();
        for state in source.strata.values_mut() {
            state.retranslocation_n = -1.0;
            state.nsc_c = -1.0;
            state.standing_dead.carbon = -1.0;
            state.standing_dead.nitrogen = -1.0;
            state.standing_dead_dm = -1.0;
            state.onset_remaining_s = -1.0;
            state.offset_remaining_s = -1.0;
            state.previous_gsi = -1.0;
            state.t10_k = -1.0;
            state
                .tissues
                .get_mut(&Tissue::Leaf)
                .expect("leaf")
                .storage
                .carbon = -1.0;
        }
        source.state_sha256 = source.canonical_sha256().expect("poison digest");
        let V3ToV4MigrationResult::Incomplete(report) =
            migrate_v3_state(&source_config, &source, &target_config)
        else {
            panic!("negative stores and timers must reject all-or-none")
        };
        for stratum_id in source.strata.keys() {
            assert!(report.unresolved.iter().any(|field| {
                field.stratum_id.as_ref() == Some(stratum_id)
                    && field.issue == V3ToV4MigrationIssue::InvalidSharedState
            }));
        }
    }

    #[test]
    fn v3_to_v4_rejects_identity_payload_and_membership_poisons() {
        let (mut source_config, mut source, mut target_config, _) = v3_to_v4_fixture();
        source_config.model_definition_sha256 = "0".repeat(64);
        source.configuration_sha256 = "1".repeat(64);
        source.occupancies.pop_first();
        target_config.dt_s += 1.0;
        target_config.configuration_sha256.clear();
        target_config.configuration_sha256 = target_config
            .canonical_sha256()
            .expect("poison target digest");
        let V3ToV4MigrationResult::Incomplete(report) =
            migrate_v3_state(&source_config, &source, &target_config)
        else {
            panic!("identity and payload poisons must reject")
        };
        let issues = report
            .unresolved
            .iter()
            .map(|field| field.issue)
            .collect::<BTreeSet<_>>();
        assert!(issues.contains(&V3ToV4MigrationIssue::InvalidV3ModelIdentity));
        assert!(issues.contains(&V3ToV4MigrationIssue::InvalidV3ConfigurationDigest));
        assert!(issues.contains(&V3ToV4MigrationIssue::InvalidV3StateDigest));
        assert!(issues.contains(&V3ToV4MigrationIssue::V3V4ConfigurationMismatch));
        assert!(issues.contains(&V3ToV4MigrationIssue::MissingOccupancy));
    }

    type IdentityPoisonCase = (
        &'static str,
        V3ToV4MigrationIssue,
        fn(&mut V3CoupledOwnedState),
        bool,
    );

    fn identity_poison_cases_a() -> [IdentityPoisonCase; 6] {
        [
            (
                "wrong_model",
                V3ToV4MigrationIssue::InvalidV3ModelIdentity,
                |state| state.model_definition_sha256 = "0".repeat(64),
                false,
            ),
            (
                "wrong_configuration",
                V3ToV4MigrationIssue::InvalidV3ConfigurationDigest,
                |state| state.configuration_sha256 = "5".repeat(64),
                false,
            ),
            (
                "wrong_state_digest",
                V3ToV4MigrationIssue::InvalidV3StateDigest,
                |state| state.state_sha256 = "0".repeat(64),
                false,
            ),
            (
                "missing_occupancy",
                V3ToV4MigrationIssue::MissingOccupancy,
                |state| {
                    state.occupancies.remove(&OccupancyId {
                        stratum_id: StratumId::try_new("understory").expect("stratum ID"),
                        tile_id: TileId::try_new("tile-a").expect("tile ID"),
                    });
                },
                true,
            ),
            (
                "extra_occupancy",
                V3ToV4MigrationIssue::ExtraOccupancy,
                |state| {
                    let lane = state.occupancies.values().next().expect("lane").clone();
                    state.occupancies.insert(
                        OccupancyId {
                            stratum_id: StratumId::try_new("extra").expect("stratum ID"),
                            tile_id: TileId::try_new("tile-a").expect("tile ID"),
                        },
                        lane,
                    );
                },
                true,
            ),
            (
                "shared_lineage",
                V3ToV4MigrationIssue::InvalidTransactionLineage,
                |state| {
                    state
                        .strata
                        .get_mut(&StratumId::try_new("canopy").expect("stratum ID"))
                        .expect("canopy")
                        .last_transaction_id = 40;
                },
                true,
            ),
        ]
    }

    fn identity_poison_cases_b() -> [IdentityPoisonCase; 5] {
        [
            (
                "occupancy_lineage",
                V3ToV4MigrationIssue::InvalidOccupancyState,
                |state| {
                    state
                        .occupancies
                        .values_mut()
                        .next()
                        .expect("lane")
                        .last_accepted_transaction_id = Some(40);
                },
                true,
            ),
            (
                "missing_stratum",
                V3ToV4MigrationIssue::MissingStratum,
                |state| {
                    state
                        .strata
                        .remove(&StratumId::try_new("understory").expect("stratum ID"));
                },
                true,
            ),
            (
                "extra_stratum",
                V3ToV4MigrationIssue::ExtraStratum,
                |state| {
                    let shared = state.strata.values().next().expect("stratum").clone();
                    state
                        .strata
                        .insert(StratumId::try_new("extra").expect("stratum ID"), shared);
                },
                true,
            ),
            (
                "wrong_transfer_owner",
                V3ToV4MigrationIssue::InvalidPendingTransfer,
                |state| {
                    state
                        .strata
                        .get_mut(&StratumId::try_new("understory").expect("stratum ID"))
                        .expect("understory")
                        .pending_transfers[0]
                        .owner_id =
                        ResourceOwnerId::try_new("stratum:canopy").expect("owner identity");
                },
                true,
            ),
            (
                "wrong_transfer_transaction",
                V3ToV4MigrationIssue::InvalidPendingTransfer,
                |state| {
                    state
                        .strata
                        .get_mut(&StratumId::try_new("canopy").expect("stratum ID"))
                        .expect("canopy")
                        .pending_transfers[0]
                        .transaction_id = 40;
                },
                true,
            ),
        ]
    }

    fn exercise_identity_poison(
        name: &str,
        issue: V3ToV4MigrationIssue,
        mutation: fn(&mut V3CoupledOwnedState),
        refresh_digest: bool,
    ) {
        let (source_config, mut source, target_config, _) = authority_two_stratum_fixture();
        mutation(&mut source);
        if refresh_digest {
            source.state_sha256 = source.canonical_sha256().expect("poison digest");
        }
        let V3ToV4MigrationResult::Incomplete(report) =
            migrate_v3_state(&source_config, &source, &target_config)
        else {
            panic!("released poison {name} must reject")
        };
        assert!(
            report.unresolved.iter().any(|field| field.issue == issue),
            "released poison {name} did not produce {issue:?}: {:?}",
            report.unresolved
        );
    }

    #[test]
    fn every_committed_v3_identity_poison_is_independently_exercised() {
        let vectors: serde_json::Value =
            serde_json::from_slice(V4_AUTHORITY_VECTOR_BYTES).expect("V4 authority vectors");
        let released = vectors["v3_to_v4_migration"]["identity_poisons"]
            .as_object()
            .expect("released identity-poison map");
        assert_eq!(released.len(), 12);
        assert!(
            released
                .values()
                .all(|result| result["candidate"].is_null())
        );
        let mut exercised = BTreeSet::new();
        for (name, issue, mutation, refresh) in identity_poison_cases_a()
            .into_iter()
            .chain(identity_poison_cases_b())
        {
            assert!(released.contains_key(name), "released poison {name}");
            exercise_identity_poison(name, issue, mutation, refresh);
            exercised.insert(name);
        }

        // A duplicate cannot be represented by the production BTreeMap. Exercise
        // the released poison at the strict historical DTO boundary instead.
        let (_, source, _, _) = authority_two_stratum_fixture();
        let mut duplicate = serde_json::to_value(&source).expect("V3 state JSON");
        let lane = duplicate["occupancies"][0].clone();
        duplicate["occupancies"]
            .as_array_mut()
            .expect("historical occupancy sequence")
            .push(lane);
        assert!(
            V3CoupledOwnedState::parse_strict(
                &serde_json::to_vec(&duplicate).expect("duplicate poison bytes")
            )
            .is_err()
        );
        exercised.insert("duplicate_occupancy");

        assert_eq!(
            exercised,
            released.keys().map(String::as_str).collect::<BTreeSet<_>>()
        );
    }

    #[test]
    fn v3_strict_parser_rejects_unknown_and_duplicate_occupancy_fields() {
        let (source_config, source, _, _) = v3_to_v4_fixture();
        let config_bytes = serde_json::to_vec(&source_config).expect("V3 config bytes");
        assert_eq!(
            V3VegetationConfiguration::parse_strict(&config_bytes)
                .expect("strict V3 configuration"),
            source_config
        );
        let mut config_value: serde_json::Value =
            serde_json::from_slice(&config_bytes).expect("configuration JSON");
        config_value["unknown"] = serde_json::json!(1);
        assert!(
            V3VegetationConfiguration::parse_strict(
                &serde_json::to_vec(&config_value).expect("bytes")
            )
            .is_err()
        );
        let bytes = serde_json::to_vec(&source).expect("V3 bytes");
        assert_eq!(
            V3CoupledOwnedState::parse_strict(&bytes).expect("strict V3"),
            source
        );
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).expect("JSON");
        value["strata"]
            .as_object_mut()
            .expect("strata")
            .values_mut()
            .next()
            .expect("stratum")["unknown"] = serde_json::json!(1);
        assert!(
            V3CoupledOwnedState::parse_strict(&serde_json::to_vec(&value).expect("bytes")).is_err()
        );

        let mut duplicate: serde_json::Value = serde_json::from_slice(&bytes).expect("JSON");
        let lane = duplicate["occupancies"][0].clone();
        duplicate["occupancies"]
            .as_array_mut()
            .expect("lanes")
            .push(lane);
        assert!(
            V3CoupledOwnedState::parse_strict(&serde_json::to_vec(&duplicate).expect("bytes"))
                .is_err()
        );

        let (stratum_id, shared) = source.strata.first_key_value().expect("stratum");
        let stratum_entry = format!(
            "{}:{}",
            serde_json::to_string(stratum_id).expect("stratum key"),
            serde_json::to_string(shared).expect("stratum state")
        );
        let state_text = String::from_utf8(bytes.clone()).expect("UTF-8 JSON");
        let duplicate_stratum = state_text.replacen(
            &stratum_entry,
            &format!("{stratum_entry},{stratum_entry}"),
            1,
        );
        assert!(V3CoupledOwnedState::parse_strict(duplicate_stratum.as_bytes()).is_err());

        let leaf = shared.tissues.get(&Tissue::Leaf).expect("leaf tissue");
        let tissue_entry = format!(
            "\"leaf\":{}",
            serde_json::to_string(leaf).expect("leaf pool")
        );
        let duplicate_tissue =
            state_text.replacen(&tissue_entry, &format!("{tissue_entry},{tissue_entry}"), 1);
        assert!(V3CoupledOwnedState::parse_strict(duplicate_tissue.as_bytes()).is_err());
    }

    #[test]
    fn rhessys_v3_mapping_reports_every_unsynthesizable_lane() {
        let config = two_tile_config();
        let source = RhessysSource {
            source_path: "synthetic.epc".into(),
            raw_bytes: "leaf_cn 28".into(),
            fields: BTreeMap::from([("leaf_cn".into(), serde_json::json!(28.0))]),
        };
        let report = migrate(
            &source,
            &BTreeMap::new(),
            &["cn_leaf".into(), "p50_leaf_mm".into()],
            &BTreeMap::from([("leaf_cn".into(), "cn_leaf".into())]),
            &config.expected_occupancies(),
        );
        assert_eq!(report.mapping_version, RHESSYS_MAPPING_VERSION);
        assert_eq!(report.unresolved_required_fields, ["p50_leaf_mm"]);
        assert_eq!(
            report.unresolved_occupancy_numerical_fields.len(),
            2 * (WARM_START_FIELDS.len() + 1)
        );
        assert!(report.canonical_configuration_sha256.is_none());
    }

    #[test]
    fn v1_zero_store_requires_explicit_v3_successor_migration() {
        let config = two_tile_config();
        let source = v1_state(0.0);
        let lanes = warm_starts(&config, 9.0);
        let result = migrate_v1_state(&source, &config, &lanes);
        let V1StateMigration::Incomplete(report) = result else {
            panic!("direct V1-to-V4 migration must remain fail-closed: {result:?}")
        };
        assert!(
            report
                .unresolved
                .iter()
                .any(|field| { field.issue == MigrationIssue::SuccessorMigrationRequired })
        );
        assert_eq!(report.to_model_definition_sha256, V3_MODEL_SHA256);
    }

    #[test]
    fn v1_single_occupancy_requires_explicit_v3_successor_migration() {
        let mut config = config();
        config.topology_tiles[0].fraction = 0.4;
        config.topology_tiles.push(crate::TopologyTile {
            tile_id: TileId::try_new("unoccupied").expect("tile ID"),
            fraction: 0.6,
        });
        config.configuration_sha256.clear();
        config.configuration_sha256 = config.canonical_sha256().expect("digest");
        let source = v1_state(0.8);
        let lanes = warm_starts(&config, 99.0);
        let result = migrate_v1_state(&source, &config, &lanes);
        let V1StateMigration::Incomplete(report) = result else {
            panic!("direct V1-to-V4 migration must remain fail-closed: {result:?}")
        };
        assert!(
            report
                .unresolved
                .iter()
                .any(|field| { field.issue == MigrationIssue::SuccessorMigrationRequired })
        );
    }

    #[test]
    fn nonzero_multi_tile_is_always_exhaustively_unresolved() {
        let config = two_tile_config();
        let result = migrate_v1_state(&v1_state(0.8), &config, &warm_starts(&config, 0.4));
        let V1StateMigration::Incomplete(report) = result else {
            panic!("multi-tile aggregate liquid must not complete")
        };
        let liquid = report
            .unresolved
            .iter()
            .filter(|item| item.issue == MigrationIssue::UnresolvedMultiTileLiquid)
            .collect::<Vec<_>>();
        assert_eq!(liquid.len(), 2);
        assert!(liquid.iter().all(|item| {
            item.occupancy_id.is_some()
                && item.stratum_id.is_some()
                && item.field == OccupancyMigrationField::CanopyLiquidKgH2oM2TileGround
        }));
    }

    #[test]
    fn missing_warm_starts_report_every_field_for_every_occupancy() {
        let config = two_tile_config();
        let V1StateMigration::Incomplete(report) =
            migrate_v1_state(&v1_state(0.0), &config, &BTreeMap::new())
        else {
            panic!("missing warm starts must not complete")
        };
        assert_eq!(
            report
                .unresolved
                .iter()
                .filter(|item| item.issue == MigrationIssue::MissingWarmStart)
                .count(),
            2 * WARM_START_FIELDS.len()
        );
    }

    #[test]
    fn rejects_nonnull_warm_start_and_wrong_v1_identity() {
        let config = config();
        let mut source = v1_state(0.0);
        source.model_definition_sha256 = MODEL_SHA256.into();
        let mut lanes = warm_starts(&config, 0.0);
        lanes
            .values_mut()
            .next()
            .expect("lane")
            .last_accepted_transaction_id = Some(1);
        let V1StateMigration::Incomplete(report) = migrate_v1_state(&source, &config, &lanes)
        else {
            panic!("identity and lineage poison must reject")
        };
        assert!(
            report
                .unresolved
                .iter()
                .any(|item| item.issue == MigrationIssue::InvalidV1ModelIdentity)
        );
        assert!(
            report
                .unresolved
                .iter()
                .any(|item| item.issue == MigrationIssue::NonNullWarmStartTransaction)
        );
    }

    #[test]
    fn rejects_nonfinite_or_negative_v1_liquid_without_conversion() {
        let config = config();
        for liquid in [f64::NAN, -0.1] {
            let V1StateMigration::Incomplete(report) =
                migrate_v1_state(&v1_state(liquid), &config, &warm_starts(&config, 0.0))
            else {
                panic!("invalid V1 liquid must not complete")
            };
            assert!(
                report
                    .unresolved
                    .iter()
                    .any(|item| item.issue == MigrationIssue::InvalidV1Liquid)
            );
        }
    }

    #[test]
    fn v1_dto_rejects_unknown_old_state_fields() {
        let mut value: serde_json::Value = serde_json::from_slice(STATE_BYTES).expect("JSON");
        value["strata"]["tree-1"]["unknown_warm_start"] = serde_json::json!(1.0);
        assert!(serde_json::from_value::<V1CoupledOwnedState>(value).is_err());
    }

    fn historical_v2_lane(root_values: &[f64]) -> V2OccupancyState {
        V2OccupancyState {
            beta_hyd: 0.7,
            canopy_air_specific_humidity_kg_kg: 0.01,
            canopy_air_temperature_k: 295.0,
            canopy_liquid_kg_h2o_m2_tile_ground: 0.1,
            dry_stem_temperature_k: 294.0,
            last_accepted_transaction_id: None,
            root_potential_mm_by_layer: root_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    (
                        SoilLayerId::try_new(format!("soil-{index}")).expect("layer ID"),
                        *value,
                    )
                })
                .collect(),
            shade_ci_pa: 25.0,
            shade_leaf_potential_mm: -6000.0,
            shade_leaf_temperature_k: 295.0,
            stem_potential_mm: -5500.0,
            sun_ci_pa: 26.0,
            sun_leaf_potential_mm: -6200.0,
            sun_leaf_temperature_k: 296.0,
            wet_surface_temperature_k: 294.5,
        }
    }

    fn bind_valid_root_layers(configuration: &mut VegetationConfiguration, count: usize) {
        for stratum in &mut configuration.strata {
            if stratum.phenology_type == crate::PhenologyType::Evergreen {
                stratum.current_growth_fraction = 1.0;
            }
        }
        let count = count.max(1);
        let fraction = 1.0 / f64::from(u32::try_from(count).expect("small test roots"));
        configuration.strata[0].root_layers = (0..count)
            .map(|index| RootLayer {
                layer_id: SoilLayerId::try_new(format!("soil-{index}")).expect("layer ID"),
                root_fraction: fraction,
                mineral_n_root_fraction: fraction,
                lateral_root_length_m: 0.001,
            })
            .collect();
        configuration.configuration_sha256.clear();
        configuration.configuration_sha256 = configuration
            .canonical_sha256()
            .expect("configuration digest");
        configuration
            .validate()
            .expect("valid migration configuration");
    }

    #[test]
    fn v2_identical_root_bits_still_require_explicit_v3_successor() {
        let mut configuration = config();
        bind_valid_root_layers(&mut configuration, 2);
        let id = OccupancyId {
            stratum_id: StratumId::try_new("tree-1").expect("stratum"),
            tile_id: TileId::try_new("tile-1").expect("tile"),
        };
        let source = V2OccupancyStateSet {
            model_definition_sha256: V2_MODEL_SHA256.into(),
            occupancies: BTreeMap::from([(id.clone(), historical_v2_lane(&[-5000.0, -5000.0]))]),
        };
        let V2OccupancyMigration::Incomplete(report) =
            migrate_v2_occupancy_lanes(&source, &configuration, None)
        else {
            panic!("V2-to-V4 cannot silently cross the V3 successor boundary")
        };
        assert!(report.unresolved.iter().any(|item| {
            item.occupancy_id.is_none() && item.issue == MigrationIssue::SuccessorMigrationRequired
        }));
    }

    #[test]
    fn v2_ambiguous_root_vectors_report_every_occupancy_without_normalization() {
        let cases = [
            vec![],
            vec![-5000.0, -5001.0],
            vec![0.0, -0.0],
            vec![f64::NAN],
        ];
        for roots in cases {
            let mut configuration = config();
            bind_valid_root_layers(&mut configuration, roots.len());
            let id = OccupancyId {
                stratum_id: StratumId::try_new("tree-1").expect("stratum"),
                tile_id: TileId::try_new("tile-1").expect("tile"),
            };
            let source = V2OccupancyStateSet {
                model_definition_sha256: V2_MODEL_SHA256.into(),
                occupancies: BTreeMap::from([(id.clone(), historical_v2_lane(&roots))]),
            };
            let V2OccupancyMigration::Incomplete(report) =
                migrate_v2_occupancy_lanes(&source, &configuration, None)
            else {
                panic!("ambiguous V2 roots must remain unresolved")
            };
            assert_eq!(report.from_model_definition_sha256, V2_MODEL_SHA256);
            assert!(report.unresolved.iter().any(|item| {
                item.occupancy_id.as_ref() == Some(&id)
                    && item.field == OccupancyMigrationField::RootNodePotentialMm
                    && item.issue == MigrationIssue::AmbiguousV2LayerRootWarmStarts
            }));
            assert!(report.unresolved.iter().any(|item| {
                item.occupancy_id.is_none()
                    && item.issue == MigrationIssue::SuccessorMigrationRequired
            }));
        }
    }

    #[test]
    fn v2_strict_parser_accepts_array_pairs_and_rejects_duplicate_occupancies() {
        let configuration = config();
        let id = configuration
            .expected_occupancies()
            .into_iter()
            .next()
            .expect("occupancy");
        let source = V2OccupancyStateSet {
            model_definition_sha256: V2_MODEL_SHA256.into(),
            occupancies: BTreeMap::from([(id, historical_v2_lane(&[-5000.0]))]),
        };
        let bytes = serde_json::to_vec(&source).expect("serialize V2 state set");
        assert_eq!(
            V2OccupancyStateSet::parse_strict(&bytes).expect("historical array pairs"),
            source
        );
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).expect("JSON");
        let pair = value["occupancies"][0].clone();
        value["occupancies"]
            .as_array_mut()
            .expect("pairs")
            .push(pair);
        assert!(
            V2OccupancyStateSet::parse_strict(&serde_json::to_vec(&value).expect("duplicate JSON"))
                .is_err()
        );
    }

    #[test]
    fn v2_migration_rejects_missing_lanes_wrong_layers_and_model_identity() {
        let configuration = config();
        let empty = V2OccupancyStateSet {
            model_definition_sha256: "0".repeat(64),
            occupancies: BTreeMap::new(),
        };
        let V2OccupancyMigration::Incomplete(report) =
            migrate_v2_occupancy_lanes(&empty, &configuration, None)
        else {
            panic!("missing lane and wrong identity must fail")
        };
        assert!(
            report
                .unresolved
                .iter()
                .any(|item| item.issue == MigrationIssue::InvalidV2ModelIdentity)
        );
        assert!(
            report
                .unresolved
                .iter()
                .any(|item| item.issue == MigrationIssue::MissingV2Occupancy)
        );

        let id = configuration
            .expected_occupancies()
            .into_iter()
            .next()
            .expect("occupancy");
        let wrong_layer = V2OccupancyStateSet {
            model_definition_sha256: V2_MODEL_SHA256.into(),
            occupancies: BTreeMap::from([(id, historical_v2_lane(&[-5000.0]))]),
        };
        let V2OccupancyMigration::Incomplete(report) =
            migrate_v2_occupancy_lanes(&wrong_layer, &configuration, None)
        else {
            panic!("wrong V2 layer identity must fail")
        };
        assert!(
            report
                .unresolved
                .iter()
                .any(|item| item.issue == MigrationIssue::V2RootLayerIdentity)
        );
    }
}
