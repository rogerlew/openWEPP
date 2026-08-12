//! Explicit, offline migration boundaries for vegetation definitions and state.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, StratumId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::carbon_nitrogen::{ElementPool, MaterialTransfer, Tissue, TissuePool};
use crate::occupancy_state::{OccupancyStateError, OccupancyStateLanes};
use crate::{
    CoupledOwnedState, MODEL_SHA256, PhenologyPhase, StratumSharedState, VegetationConfiguration,
};

/// Immutable identity of the historical state schema accepted by this module.
pub const V1_MODEL_SHA256: &str =
    "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157";
/// Version of the offline, non-runtime `RHESSys` definition mapping table.
pub const RHESSYS_MAPPING_VERSION: &str = "RHESSYS_TO_OPENWEPP_C3_WOODY_V2_V1";

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

/// One required V2 numerical field at one exact occupancy.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OccupancyFieldRequirement {
    pub occupancy_id: OccupancyId,
    pub field: OccupancyMigrationField,
}

/// V2 occupancy fields whose values cannot be synthesized during migration.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OccupancyMigrationField {
    BetaHyd,
    CanopyAirSpecificHumidityKgKg,
    CanopyAirTemperatureK,
    CanopyLiquidKgH2oM2TileGround,
    DryStemTemperatureK,
    LastAcceptedTransactionId,
    RootPotentialMmByLayer,
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
    OccupancyMigrationField::RootPotentialMmByLayer,
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

/// V2 definition mapping with an exhaustive declaration of caller-required
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

/// Performs the only authority-admitted V1-to-V2 state migration.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn migrate_v1_state(
    source: &V1CoupledOwnedState,
    configuration: &VegetationConfiguration,
    warm_starts: &OccupancyStateLanes,
) -> MigrationResult {
    let expected = configuration.expected_occupancies();
    let mut unresolved = Vec::new();

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
            (Some(state), Some(layers)) => {
                if let Err(error) = state.validate(&layers, None) {
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
            to_model_definition_sha256: MODEL_SHA256.into(),
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
            previous_leaf_offset_flux: self.previous_leaf_offset_flux,
            previous_root_offset_flux: self.previous_root_offset_flux,
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
        to_model_definition_sha256: MODEL_SHA256.into(),
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
            _ => OccupancyMigrationField::RootPotentialMmByLayer,
        },
        OccupancyStateError::RootLayerIdentity { .. } => {
            OccupancyMigrationField::RootPotentialMmByLayer
        }
        OccupancyStateError::StaleTransaction { .. } => {
            OccupancyMigrationField::LastAcceptedTransactionId
        }
        OccupancyStateError::Schema(_) => OccupancyMigrationField::RootPotentialMmByLayer,
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
    use crate::occupancy_state::OccupancyState;
    use openwepp_kernel_contract::TileId;

    const CONFIG_BYTES: &[u8] =
        include_bytes!("../../../tests/fixtures/c3_woody_v1_diagnostic_configuration.json");
    const STATE_BYTES: &[u8] =
        include_bytes!("../../../tests/fixtures/c3_woody_v1_diagnostic_state.json");

    fn config() -> VegetationConfiguration {
        let mut value: VegetationConfiguration =
            serde_json::from_slice(CONFIG_BYTES).expect("configuration DTO");
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

    fn warm_start(config: &VegetationConfiguration, liquid: f64) -> OccupancyState {
        OccupancyState {
            beta_hyd: 0.7,
            canopy_air_specific_humidity_kg_kg: 0.01,
            canopy_air_temperature_k: 295.0,
            canopy_liquid_kg_h2o_m2_tile_ground: liquid,
            dry_stem_temperature_k: 294.0,
            last_accepted_transaction_id: None,
            root_potential_mm_by_layer: config.strata[0]
                .root_layers
                .iter()
                .map(|layer| (layer.layer_id.clone(), -5000.0))
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

    fn warm_starts(config: &VegetationConfiguration, liquid: f64) -> OccupancyStateLanes {
        config
            .expected_occupancies()
            .into_iter()
            .map(|id| (id, warm_start(config, liquid)))
            .collect()
    }

    fn bind_expected_initial_state(
        config: &mut VegetationConfiguration,
        source: &V1CoupledOwnedState,
        mut lanes: OccupancyStateLanes,
        migrated_liquid: f64,
    ) {
        for lane in lanes.values_mut() {
            lane.canopy_liquid_kg_h2o_m2_tile_ground = migrated_liquid;
        }
        let mut expected = CoupledOwnedState {
            model_definition_sha256: MODEL_SHA256.into(),
            configuration_sha256: config.configuration_sha256.clone(),
            state_sha256: String::new(),
            strata: source
                .strata
                .iter()
                .map(|(id, state)| (id.clone(), state.clone().into_shared()))
                .collect(),
            occupancies: lanes,
            last_transaction_id: source.last_transaction_id,
        };
        expected.state_sha256 = expected.canonical_sha256().expect("state digest");
        config
            .initial_state_sha256
            .clone_from(&expected.state_sha256);
        expected
            .validate(config)
            .expect("bound expected migration state validates");
    }

    #[test]
    fn rhessys_v2_mapping_reports_every_unsynthesizable_lane() {
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
    fn zero_store_sets_all_liquid_to_zero_without_copying_warm_starts() {
        let mut config = two_tile_config();
        let source = v1_state(0.0);
        let lanes = warm_starts(&config, 9.0);
        bind_expected_initial_state(&mut config, &source, lanes.clone(), 0.0);
        let result = migrate_v1_state(&source, &config, &lanes);
        let V1StateMigration::Complete(state) = result else {
            panic!("zero store migration must complete: {result:?}")
        };
        assert!(state.occupancies.values().all(|lane| {
            lane.canopy_liquid_kg_h2o_m2_tile_ground == 0.0
                && lane.sun_leaf_temperature_k == 296.0
                && lane.stem_potential_mm == -5500.0
                && lane.last_accepted_transaction_id.is_none()
        }));
        state.validate(&config).expect("complete state validates");
    }

    #[test]
    fn single_occupancy_uses_exact_stratum_coverage_conversion() {
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
        bind_expected_initial_state(&mut config, &source, lanes.clone(), 2.0);
        let result = migrate_v1_state(&source, &config, &lanes);
        let V1StateMigration::Complete(state) = result else {
            panic!("single occupancy migration must complete: {result:?}")
        };
        assert_eq!(
            state
                .occupancies
                .values()
                .next()
                .expect("lane")
                .canopy_liquid_kg_h2o_m2_tile_ground,
            2.0
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
}
