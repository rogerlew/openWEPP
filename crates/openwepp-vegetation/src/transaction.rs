use std::collections::BTreeMap;

use openwepp_kernel_contract::{
    FinalizedUse, MaximumAuthorization, MineralNitrogenKey, OccupancyId, ResourceRequest,
    SoilLayerId, StratumId, TransactionId, WaterResourceKey,
};
use serde::{Deserialize, Serialize};

use crate::carbon_nitrogen::{ElementPool, MaterialTransfer, Tissue, TissuePool};
use crate::ledger::FiveLedgerOperands;
use crate::occupancy_state::OccupancyState;
use crate::{MODEL_SHA256, ModelDefinition, VegetationConfiguration, VegetationError};
use sha2::{Digest, Sha256};

mod occupancy_state_map {
    use std::collections::BTreeMap;

    use openwepp_kernel_contract::OccupancyId;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
        let entries = Vec::<(OccupancyId, OccupancyState)>::deserialize(deserializer)?;
        let expected_len = entries.len();
        let lanes = entries.into_iter().collect::<BTreeMap<_, _>>();
        if lanes.len() != expected_len {
            return Err(serde::de::Error::custom(
                "duplicate V2 occupancy state identity",
            ));
        }
        Ok(lanes)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhenologyPhase {
    Dormant,
    Onset,
    Active,
    Offset,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StratumSharedState {
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
pub struct CoupledOwnedState {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub state_sha256: String,
    pub strata: BTreeMap<StratumId, StratumSharedState>,
    #[serde(with = "occupancy_state_map")]
    pub occupancies: BTreeMap<OccupancyId, OccupancyState>,
    pub last_transaction_id: u128,
}

impl CoupledOwnedState {
    pub fn parse_strict(
        bytes: &[u8],
        config: &VegetationConfiguration,
    ) -> Result<Self, VegetationError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| VegetationError::Schema(error.to_string()))?;
        value.validate(config)?;
        Ok(value)
    }

    #[allow(clippy::too_many_lines)]
    pub fn validate(&self, config: &VegetationConfiguration) -> Result<(), VegetationError> {
        config.validate()?;
        if self.model_definition_sha256.len() != 64
            || self.configuration_sha256.len() != 64
            || self.state_sha256.len() != 64
        {
            return Err(VegetationError::Domain("state identity"));
        }
        if self.model_definition_sha256 != MODEL_SHA256
            || self.model_definition_sha256 != config.model_definition_sha256
            || self.configuration_sha256 != config.configuration_sha256
        {
            return Err(VegetationError::Receipt(
                "model/configuration state identity".into(),
            ));
        }
        if self.state_sha256 != self.canonical_sha256()? {
            return Err(VegetationError::Receipt(
                "state digest does not match canonical bytes".into(),
            ));
        }
        if self.last_transaction_id == 0 && config.initial_state_sha256 != self.state_sha256 {
            return Err(VegetationError::Receipt(
                "configuration initial-state identity".into(),
            ));
        }
        let configured_strata = config
            .strata
            .iter()
            .map(|stratum| stratum.stratum_id.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let state_strata = self
            .strata
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        if state_strata != configured_strata {
            return Err(VegetationError::Domain(
                "configuration/state stratum identity",
            ));
        }
        let expected_occupancies = config.expected_occupancies();
        let state_occupancies = self
            .occupancies
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        if state_occupancies != expected_occupancies {
            return Err(VegetationError::Domain("V2 occupancy state identity"));
        }
        let expected_previous_transaction_id =
            (self.last_transaction_id != 0).then_some(self.last_transaction_id);
        for (occupancy_id, state) in &self.occupancies {
            let stratum = config
                .strata
                .iter()
                .find(|stratum| stratum.stratum_id == occupancy_id.stratum_id)
                .ok_or(VegetationError::Domain("V2 occupancy stratum identity"))?;
            let mut root_layer_ids = stratum
                .root_layers
                .iter()
                .map(|root| root.layer_id.clone())
                .collect::<Vec<_>>();
            root_layer_ids.sort();
            state
                .validate(&root_layer_ids, expected_previous_transaction_id)
                .map_err(|error| VegetationError::Receipt(error.to_string()))?;
        }
        for state in self.strata.values() {
            if state.last_transaction_id != self.last_transaction_id {
                return Err(VegetationError::Receipt(
                    "shared stratum transaction lineage".into(),
                ));
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
                state.previous_leaf_offset_flux,
                state.previous_root_offset_flux,
                state.previous_gsi,
                state.t10_k,
                state.leaf_area,
                state.root_area,
                state.stem_area,
            ];
            if scalars.iter().any(|value| !value.is_finite())
                || state.retranslocation_n < 0.0
                || state.nsc_c < 0.0
                || state.leaf_area < 0.0
                || state.root_area < 0.0
                || state.stem_area < 0.0
                || state.t10_k <= 0.0
                || !(0.0..=1.0).contains(&state.previous_gsi)
            {
                return Err(VegetationError::Domain("complete stratum state"));
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
                return Err(VegetationError::Domain("six-tissue identity"));
            }
            for transfer in &state.pending_transfers {
                if transfer.transaction_id == 0
                    || transfer.owner_id.trim().is_empty()
                    || [transfer.carbon, transfer.nitrogen, transfer.dry_matter]
                        .iter()
                        .any(|value| !value.is_finite() || *value < 0.0)
                {
                    return Err(VegetationError::Domain("pending material transfer"));
                }
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
                        return Err(VegetationError::Domain("tissue pool"));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<String, VegetationError> {
        let mut canonical = self.clone();
        canonical.state_sha256.clear();
        let bytes = serde_json::to_vec(&canonical)
            .map_err(|error| VegetationError::Schema(error.to_string()))?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnowFreeForcing {
    pub air_temperature_k: f64,
    pub pressure_pa: f64,
    pub co2_pa: f64,
    pub vapor_pressure_deficit_kpa: f64,
    pub wind_m_s: f64,
    pub rain_kg_m2: f64,
    pub direct_par_w_m2: f64,
    pub diffuse_par_w_m2: f64,
    pub direct_nir_w_m2: f64,
    pub diffuse_nir_w_m2: f64,
    pub solar_zenith_cosine: f64,
    pub ground_albedo_vis: f64,
    pub ground_albedo_nir: f64,
    pub longwave_down_w_m2: f64,
    pub longwave_up_w_m2: f64,
    pub specific_humidity: f64,
    pub reference_height_m: f64,
    pub soil_layers: Vec<SoilLayerForcing>,
    pub gsi: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoilLayerForcing {
    pub layer_id: SoilLayerId,
    pub water_beginning_kg_m2: f64,
    pub matric_potential_mm: f64,
    pub hydraulic_conductivity_mm_s: f64,
    pub root_path_length_mm: f64,
    pub gravity_root_mm: f64,
    pub temperature_k: f64,
    pub accessible: bool,
    pub frozen: bool,
}

pub type WaterRequest = ResourceRequest<WaterResourceKey, f64>;
pub type WaterAuthorization = MaximumAuthorization<WaterResourceKey, f64>;
pub type WaterUse = FinalizedUse<WaterResourceKey, f64>;
pub type NitrogenRequest = ResourceRequest<MineralNitrogenKey, f64>;
pub type NitrogenAuthorization = MaximumAuthorization<MineralNitrogenKey, f64>;
pub type NitrogenUse = FinalizedUse<MineralNitrogenKey, f64>;

pub trait WaterArbiter {
    fn beginning_amount(&self, key: &WaterResourceKey) -> Result<f64, VegetationError>;
    fn authorize(
        &self,
        requests: &[WaterRequest],
    ) -> Result<Vec<WaterAuthorization>, VegetationError>;
}
pub trait NitrogenArbiter {
    fn beginning_amount(&self, key: &MineralNitrogenKey) -> Result<f64, VegetationError>;
    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutionDiagnostics {
    pub transaction_id: TransactionId,
    pub solver_iterations: u32,
    pub normalized_residuals: Vec<f64>,
    pub active_bounds: Vec<String>,
    pub authorization_activity: bool,
    pub temperature_step_k: f64,
    pub potential_step_mm: f64,
    pub backtracking_count: u32,
    pub wet_store_cap_active: bool,
    pub gas_hydraulic_mismatch_kg_m2_s: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnergyOwnerOperands {
    pub incident_shortwave_w_m2: f64,
    pub reflected_shortwave_w_m2: f64,
    pub terminal_shortwave_w_m2: f64,
    pub incident_longwave_j_m2: f64,
    pub emitted_longwave_j_m2: f64,
    pub sensible_j_m2: f64,
    pub transpiration_kg_m2: f64,
    pub wet_phase_change_kg_m2: f64,
    pub interval_s: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CoupledCandidate {
    beginning_state_sha256: String,
    state: CoupledOwnedState,
    water_requests: Vec<WaterRequest>,
    water_authorizations: Vec<WaterAuthorization>,
    water_uses: Vec<WaterUse>,
    nitrogen_requests: Vec<NitrogenRequest>,
    nitrogen_authorizations: Vec<NitrogenAuthorization>,
    nitrogen_uses: Vec<NitrogenUse>,
    material_transfers: Vec<MaterialTransfer>,
    ledger_operands: crate::ledger::FiveLedgerOperands,
    energy_owner_operands: EnergyOwnerOperands,
    diagnostics: ExecutionDiagnostics,
}
impl CoupledCandidate {
    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.diagnostics.transaction_id
    }
    #[must_use]
    pub fn water_uses(&self) -> &[WaterUse] {
        &self.water_uses
    }
    #[must_use]
    pub fn nitrogen_protocol(
        &self,
    ) -> (&[NitrogenRequest], &[NitrogenAuthorization], &[NitrogenUse]) {
        (
            &self.nitrogen_requests,
            &self.nitrogen_authorizations,
            &self.nitrogen_uses,
        )
    }
    #[must_use]
    pub fn material_transfers(&self) -> &[MaterialTransfer] {
        &self.material_transfers
    }
    #[must_use]
    pub fn ledger_operands(&self) -> &FiveLedgerOperands {
        &self.ledger_operands
    }
    #[must_use]
    pub fn energy_owner_operands(&self) -> &EnergyOwnerOperands {
        &self.energy_owner_operands
    }
    #[must_use]
    pub fn water_protocol(&self) -> (&[WaterRequest], &[WaterAuthorization], &[WaterUse]) {
        (
            &self.water_requests,
            &self.water_authorizations,
            &self.water_uses,
        )
    }
    #[must_use]
    pub fn diagnostics(&self) -> &ExecutionDiagnostics {
        &self.diagnostics
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CommitReceipt {
    pub transaction_id: TransactionId,
    pub ending_state_sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailurePoint {
    Validation,
    Radiation,
    Interception,
    PotentialCoupledSolve,
    WaterAuthorization,
    CappedResolve,
    NitrogenRequest,
    NitrogenAuthorization,
    Allocation,
    ReceiverConstruction,
    ClosureValidation,
    BeforeCommit,
    OwnerValidation,
}

/// Validates the complete public V2 state surface, then fails closed until the
/// occupancy-local E04 column transaction is implemented.
pub fn execute_candidate(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    _water: &dyn WaterArbiter,
    _nitrogen: &dyn NitrogenArbiter,
) -> Result<CoupledCandidate, VegetationError> {
    validate_execution(model, config, beginning, forcing)?;
    Err(VegetationError::Unsupported(
        "V2 occupancy-local E04 transaction routing is implementation-incomplete",
    ))
}

/// Failure-injection entry point retained while V2 routing is incomplete.
pub fn execute_candidate_with_failure(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water: &dyn WaterArbiter,
    nitrogen: &dyn NitrogenArbiter,
    failure: Option<FailurePoint>,
) -> Result<CoupledCandidate, VegetationError> {
    if failure == Some(FailurePoint::Validation) {
        return Err(VegetationError::InjectedFailure("validation"));
    }
    execute_candidate(model, config, beginning, forcing, water, nitrogen)
}

#[allow(clippy::too_many_lines)]
fn validate_execution(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
) -> Result<(), VegetationError> {
    if model.version != crate::MODEL_VERSION || model.sha256 != MODEL_SHA256 {
        return Err(VegetationError::ModelDigestMismatch {
            expected: MODEL_SHA256.into(),
            found: model.sha256.clone(),
        });
    }
    config.validate()?;
    beginning.validate(config)?;
    if beginning
        .strata
        .values()
        .any(|stratum| !stratum.pending_transfers.is_empty())
    {
        return Err(VegetationError::Receipt(
            "unresolved beginning-state material transfer".into(),
        ));
    }
    if beginning.model_definition_sha256 != model.sha256
        || beginning.configuration_sha256 != config.configuration_sha256
    {
        return Err(VegetationError::Receipt(
            "model/configuration state identity".into(),
        ));
    }
    if beginning.last_transaction_id == 0 && beginning.state_sha256 != config.initial_state_sha256 {
        return Err(VegetationError::Receipt(
            "initial-state digest does not match configuration".into(),
        ));
    }
    for stratum in &config.strata {
        let state = beginning
            .strata
            .get(&stratum.stratum_id)
            .ok_or(VegetationError::Domain("missing stratum state"))?;
        let expected_lai = tissue_carbon(state, Tissue::Leaf)? * stratum.sla_m2_per_kg_c;
        let derived_stem_area = expected_lai * stratum.sai_relation;
        let derived_root_area = (expected_lai + derived_stem_area) * stratum.root_to_leaf_area;
        if (state.leaf_area - expected_lai).abs() > 1e-14 + 64.0 * f64::EPSILON * expected_lai.abs()
        {
            return Err(VegetationError::Domain("leaf-C/SLA state identity"));
        }
        if (state.stem_area - derived_stem_area).abs()
            > 1e-14 + 64.0 * f64::EPSILON * derived_stem_area.abs()
        {
            return Err(VegetationError::Domain("leaf-area/SAI state identity"));
        }
        if (state.root_area - derived_root_area).abs()
            > 1e-14 + 64.0 * f64::EPSILON * derived_root_area.abs()
        {
            return Err(VegetationError::Domain("leaf/SAI/root-area state identity"));
        }
    }
    let values = [
        forcing.air_temperature_k,
        forcing.pressure_pa,
        forcing.co2_pa,
        forcing.vapor_pressure_deficit_kpa,
        forcing.wind_m_s,
        forcing.rain_kg_m2,
        forcing.direct_par_w_m2,
        forcing.diffuse_par_w_m2,
        forcing.direct_nir_w_m2,
        forcing.diffuse_nir_w_m2,
        forcing.solar_zenith_cosine,
        forcing.ground_albedo_vis,
        forcing.ground_albedo_nir,
        forcing.longwave_down_w_m2,
        forcing.longwave_up_w_m2,
        forcing.specific_humidity,
        forcing.reference_height_m,
        forcing.gsi,
    ];
    if values.iter().any(|value| !value.is_finite())
        || !(273.15..=373.15).contains(&forcing.air_temperature_k)
        || forcing.pressure_pa <= 0.0
        || forcing.co2_pa <= 0.0
        || forcing.vapor_pressure_deficit_kpa <= 0.0
        || forcing.wind_m_s <= 0.0
        || forcing.rain_kg_m2 < 0.0
        || forcing.direct_par_w_m2 < 0.0
        || forcing.diffuse_par_w_m2 < 0.0
        || forcing.direct_nir_w_m2 < 0.0
        || forcing.diffuse_nir_w_m2 < 0.0
        || (forcing.direct_par_w_m2 + forcing.direct_nir_w_m2 > 0.0
            && forcing.solar_zenith_cosine <= 0.0)
        || !(0.0..=1.0).contains(&forcing.ground_albedo_vis)
        || !(0.0..=1.0).contains(&forcing.ground_albedo_nir)
        || forcing.specific_humidity <= 0.0
        || forcing.reference_height_m <= 0.0
        || !(0.0..=1.0).contains(&forcing.gsi)
    {
        return Err(VegetationError::Domain("complete snow-free forcing"));
    }
    let forcing_ids = forcing
        .soil_layers
        .iter()
        .map(|layer| layer.layer_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if forcing_ids.len() != forcing.soil_layers.len()
        || config
            .strata
            .iter()
            .flat_map(|s| &s.root_layers)
            .any(|root| !forcing_ids.contains(root.layer_id.as_str()))
    {
        return Err(VegetationError::Domain("soil layer topology"));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::items_after_test_module)]
mod milestone_one_tests {
    use super::*;
    use crate::{RootLayer, StratumConfiguration, TopologyTile};
    use openwepp_kernel_contract::TileId;

    type LaneMutation = fn(&mut OccupancyState);

    fn stratum_id(value: &str) -> StratumId {
        StratumId::try_new(value).expect("stratum ID")
    }

    fn tile_id(value: &str) -> TileId {
        TileId::try_new(value).expect("tile ID")
    }

    fn layer_id(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("layer ID")
    }

    fn occupancy(stratum: &str, tile: &str) -> OccupancyId {
        OccupancyId {
            stratum_id: stratum_id(stratum),
            tile_id: tile_id(tile),
        }
    }

    fn fixture_config() -> VegetationConfiguration {
        let mut config: VegetationConfiguration = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v1_diagnostic_configuration.json"
        ))
        .expect("historical configuration shape");
        config.model_definition_sha256 = MODEL_SHA256.into();
        config.initial_state_sha256 = "0".repeat(64);
        config.topology_tiles = vec![
            TopologyTile {
                tile_id: tile_id("tile-a"),
                fraction: 0.35,
            },
            TopologyTile {
                tile_id: tile_id("tile-b"),
                fraction: 0.65,
            },
        ];
        let mut upper = config.strata.remove(0);
        upper.stratum_id = stratum_id("upper");
        upper.tile_ids = vec![tile_id("tile-a"), tile_id("tile-b")];
        let mut second_root: RootLayer = upper.root_layers[0].clone();
        upper.root_layers[0].root_fraction = 0.5;
        upper.root_layers[0].mineral_n_root_fraction = 0.5;
        second_root.layer_id = layer_id("soil-2");
        second_root.root_fraction = 0.5;
        second_root.mineral_n_root_fraction = 0.5;
        upper.root_layers.push(second_root.clone());
        let mut lower: StratumConfiguration = upper.clone();
        lower.stratum_id = stratum_id("lower");
        lower.tile_ids = vec![tile_id("tile-a")];
        lower.vertical_rank = 1;
        lower.height_m = upper.height_m / 2.0;
        lower.crown_base_m = lower.height_m / 2.0;
        lower.root_layers = vec![second_root];
        lower.root_layers[0].root_fraction = 1.0;
        lower.root_layers[0].mineral_n_root_fraction = 1.0;
        config.strata = vec![upper, lower];
        config.configuration_sha256 = config.canonical_sha256().expect("config digest");
        config
    }

    fn shared_state() -> StratumSharedState {
        let mut raw: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v1_diagnostic_state.json"
        ))
        .expect("historical state JSON");
        let state = raw
            .pointer_mut("/strata/tree-1")
            .and_then(serde_json::Value::as_object_mut)
            .expect("historical stratum");
        for field in [
            "canopy_liquid",
            "psi_root_mm",
            "psi_stem_mm",
            "psi_sun_mm",
            "psi_shade_mm",
        ] {
            state.remove(field);
        }
        serde_json::from_value(serde_json::Value::Object(state.clone())).expect("shared state")
    }

    fn lane(seed: f64, roots: &[&str]) -> OccupancyState {
        OccupancyState {
            beta_hyd: 0.5 + seed / 100.0,
            canopy_air_specific_humidity_kg_kg: 0.009 + seed / 10_000.0,
            canopy_air_temperature_k: 294.0 + seed,
            canopy_liquid_kg_h2o_m2_tile_ground: seed / 100.0,
            dry_stem_temperature_k: 293.0 + seed,
            last_accepted_transaction_id: None,
            root_potential_mm_by_layer: roots
                .iter()
                .enumerate()
                .map(|(index, id)| {
                    let index = u32::try_from(index).expect("test root-layer count fits u32");
                    (layer_id(id), -5_000.0 - seed - f64::from(index))
                })
                .collect(),
            shade_ci_pa: 27.0 + seed,
            shade_leaf_potential_mm: -7_000.0 - seed,
            shade_leaf_temperature_k: 294.5 + seed,
            stem_potential_mm: -6_000.0 - seed,
            sun_ci_pa: 25.0 + seed,
            sun_leaf_potential_mm: -7_500.0 - seed,
            sun_leaf_temperature_k: 295.0 + seed,
            wet_surface_temperature_k: 293.5 + seed,
        }
    }

    fn fixture() -> (VegetationConfiguration, CoupledOwnedState) {
        let mut config = fixture_config();
        let shared = shared_state();
        let mut state = CoupledOwnedState {
            model_definition_sha256: MODEL_SHA256.into(),
            configuration_sha256: config.configuration_sha256.clone(),
            state_sha256: String::new(),
            strata: BTreeMap::from([
                (stratum_id("upper"), shared.clone()),
                (stratum_id("lower"), shared),
            ]),
            occupancies: BTreeMap::from([
                (
                    occupancy("upper", "tile-a"),
                    lane(1.0, &["soil-1", "soil-2"]),
                ),
                (
                    occupancy("upper", "tile-b"),
                    lane(2.0, &["soil-1", "soil-2"]),
                ),
                (occupancy("lower", "tile-a"), lane(3.0, &["soil-2"])),
            ]),
            last_transaction_id: 0,
        };
        state.state_sha256 = state.canonical_sha256().expect("state digest");
        config.initial_state_sha256 = state.state_sha256.clone();
        (config, state)
    }

    fn refresh_state(state: &mut CoupledOwnedState, config: &mut VegetationConfiguration) {
        state.state_sha256 = state.canonical_sha256().expect("state digest");
        if state.last_transaction_id == 0 {
            config.initial_state_sha256 = state.state_sha256.clone();
        }
    }

    #[test]
    fn complete_two_tile_two_stratum_state_is_exact() {
        let (config, state) = fixture();
        state.validate(&config).expect("complete V2 state");
        assert_eq!(
            state
                .occupancies
                .keys()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>(),
            config.expected_occupancies()
        );
        assert_eq!(state.strata.len(), 2);
        assert_eq!(state.occupancies.len(), 3);
        let bytes = serde_json::to_vec(&state).expect("canonical bytes");
        assert_eq!(CoupledOwnedState::parse_strict(&bytes, &config), Ok(state));
    }

    #[test]
    fn occupancy_set_rejects_missing_extra_wrong_membership_and_duplicate() {
        let (mut config, state) = fixture();
        let mut missing = state.clone();
        missing.occupancies.remove(&occupancy("upper", "tile-b"));
        refresh_state(&mut missing, &mut config);
        assert_eq!(
            missing.validate(&config),
            Err(VegetationError::Domain("V2 occupancy state identity"))
        );

        let mut extra = state.clone();
        extra
            .occupancies
            .insert(occupancy("lower", "tile-b"), lane(4.0, &["soil-2"]));
        refresh_state(&mut extra, &mut config);
        assert_eq!(
            extra.validate(&config),
            Err(VegetationError::Domain("V2 occupancy state identity"))
        );

        let mut missing_stratum = state.clone();
        missing_stratum.strata.remove(&stratum_id("lower"));
        refresh_state(&mut missing_stratum, &mut config);
        assert_eq!(
            missing_stratum.validate(&config),
            Err(VegetationError::Domain(
                "configuration/state stratum identity"
            ))
        );

        let mut extra_stratum = state.clone();
        extra_stratum
            .strata
            .insert(stratum_id("absent"), shared_state());
        refresh_state(&mut extra_stratum, &mut config);
        assert_eq!(
            extra_stratum.validate(&config),
            Err(VegetationError::Domain(
                "configuration/state stratum identity"
            ))
        );

        let mut value = serde_json::to_value(&state).expect("state value");
        let lanes = value
            .get_mut("occupancies")
            .and_then(serde_json::Value::as_array_mut)
            .expect("lanes");
        lanes.push(lanes[0].clone());
        let bytes = serde_json::to_vec(&value).expect("duplicate bytes");
        assert!(
            matches!(CoupledOwnedState::parse_strict(&bytes, &config), Err(VegetationError::Schema(message)) if message.contains("duplicate V2 occupancy"))
        );
    }

    #[test]
    fn identity_and_historical_v1_inputs_fail_closed() {
        let (mut config, state) = fixture();
        let mut poison = state.clone();
        poison.model_definition_sha256 = "1".repeat(64);
        refresh_state(&mut poison, &mut config);
        assert!(matches!(
            poison.validate(&config),
            Err(VegetationError::Receipt(_))
        ));
        let mut poison = state.clone();
        poison.configuration_sha256 = "2".repeat(64);
        poison.state_sha256 = poison.canonical_sha256().expect("digest");
        assert!(matches!(
            poison.validate(&config),
            Err(VegetationError::Receipt(_))
        ));
        let mut poison = state;
        poison.state_sha256 = "3".repeat(64);
        assert!(matches!(
            poison.validate(&config),
            Err(VegetationError::Receipt(_))
        ));
        assert!(matches!(
            CoupledOwnedState::parse_strict(
                include_bytes!("../../../tests/fixtures/c3_woody_v1_diagnostic_state.json"),
                &config
            ),
            Err(VegetationError::Schema(_))
        ));
    }

    #[test]
    fn complete_state_digest_binds_order_identity_and_every_lane_field() {
        let (config, state) = fixture();
        let original = state.canonical_sha256().expect("digest");
        assert_eq!(
            original,
            "70d05bcda1e31aa82e9444cf73b032f20a47f6894c663ca07103bf36a0a7d77a"
        );
        let bytes = serde_json::to_vec(&state).expect("bytes");
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).expect("value");
        value["occupancies"]
            .as_array_mut()
            .expect("lanes")
            .reverse();
        let mut reordered: CoupledOwnedState = serde_json::from_value(value).expect("state");
        reordered.state_sha256 = reordered.canonical_sha256().expect("digest");
        assert_eq!(reordered.state_sha256, original);
        reordered.validate(&config).expect("order independent");

        let mutations: [LaneMutation; 15] = [
            |s| s.beta_hyd += 0.01,
            |s| s.canopy_air_specific_humidity_kg_kg += 0.001,
            |s| s.canopy_air_temperature_k += 0.1,
            |s| s.canopy_liquid_kg_h2o_m2_tile_ground += 0.01,
            |s| s.dry_stem_temperature_k += 0.1,
            |s| s.last_accepted_transaction_id = Some(1),
            |s| s.root_potential_mm_by_layer[0].1 -= 1.0,
            |s| s.shade_ci_pa += 0.1,
            |s| s.shade_leaf_potential_mm -= 1.0,
            |s| s.shade_leaf_temperature_k += 0.1,
            |s| s.stem_potential_mm -= 1.0,
            |s| s.sun_ci_pa += 0.1,
            |s| s.sun_leaf_potential_mm -= 1.0,
            |s| s.sun_leaf_temperature_k += 0.1,
            |s| s.wet_surface_temperature_k += 0.1,
        ];
        for mutate in mutations {
            let mut changed = state.clone();
            mutate(
                changed
                    .occupancies
                    .get_mut(&occupancy("upper", "tile-a"))
                    .expect("lane"),
            );
            assert_ne!(changed.canonical_sha256().expect("digest"), original);
        }
        for mutate in [
            |s: &mut CoupledOwnedState| s.configuration_sha256 = "4".repeat(64),
            |s: &mut CoupledOwnedState| {
                let lane = s
                    .occupancies
                    .remove(&occupancy("upper", "tile-a"))
                    .expect("lane");
                s.occupancies.insert(occupancy("upper", "tile-b-x"), lane);
            },
        ] {
            let mut changed = state.clone();
            mutate(&mut changed);
            assert_ne!(changed.canonical_sha256().expect("digest"), original);
        }
    }

    #[test]
    fn root_layers_units_and_transaction_lineage_are_exact() {
        let (mut config, state) = fixture();
        let mut reordered = state.clone();
        reordered
            .occupancies
            .get_mut(&occupancy("upper", "tile-a"))
            .expect("lane")
            .root_potential_mm_by_layer
            .reverse();
        refresh_state(&mut reordered, &mut config);
        assert!(matches!(
            reordered.validate(&config),
            Err(VegetationError::Receipt(_))
        ));

        let mut accepted = state.clone();
        accepted.last_transaction_id = 7;
        for shared in accepted.strata.values_mut() {
            shared.last_transaction_id = 7;
        }
        for lane in accepted.occupancies.values_mut() {
            lane.last_accepted_transaction_id = Some(7);
        }
        accepted.state_sha256 = accepted.canonical_sha256().expect("digest");
        accepted.validate(&config).expect("preceding transaction");
        accepted
            .occupancies
            .get_mut(&occupancy("upper", "tile-a"))
            .expect("lane")
            .last_accepted_transaction_id = Some(6);
        accepted.state_sha256 = accepted.canonical_sha256().expect("digest");
        assert!(matches!(
            accepted.validate(&config),
            Err(VegetationError::Receipt(_))
        ));

        let mut value = serde_json::to_value(&state).expect("value");
        let lane = &mut value["occupancies"][0][1];
        lane["stem_potential_mpa"] = lane["stem_potential_mm"].take();
        assert!(serde_json::from_value::<CoupledOwnedState>(value).is_err());
    }

    #[test]
    fn public_transaction_validates_v2_state_then_fails_closed_before_e04() {
        struct NoArbiter;
        impl WaterArbiter for NoArbiter {
            fn beginning_amount(&self, _: &WaterResourceKey) -> Result<f64, VegetationError> {
                Ok(0.0)
            }
            fn authorize(
                &self,
                _: &[WaterRequest],
            ) -> Result<Vec<WaterAuthorization>, VegetationError> {
                Ok(Vec::new())
            }
        }
        impl NitrogenArbiter for NoArbiter {
            fn beginning_amount(&self, _: &MineralNitrogenKey) -> Result<f64, VegetationError> {
                Ok(0.0)
            }
            fn authorize(
                &self,
                _: &[NitrogenRequest],
            ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
                Ok(Vec::new())
            }
        }
        let (config, state) = fixture();
        let model = crate::load_model_definition().expect("model");
        let forcing = SnowFreeForcing {
            air_temperature_k: 296.0,
            pressure_pa: 101_325.0,
            co2_pa: 40.0,
            vapor_pressure_deficit_kpa: 1.0,
            wind_m_s: 2.0,
            rain_kg_m2: 0.0,
            direct_par_w_m2: 0.0,
            diffuse_par_w_m2: 0.0,
            direct_nir_w_m2: 0.0,
            diffuse_nir_w_m2: 0.0,
            solar_zenith_cosine: 0.5,
            ground_albedo_vis: 0.1,
            ground_albedo_nir: 0.2,
            longwave_down_w_m2: 300.0,
            longwave_up_w_m2: 350.0,
            specific_humidity: 0.01,
            reference_height_m: 20.0,
            soil_layers: vec![
                SoilLayerForcing {
                    layer_id: layer_id("soil-1"),
                    water_beginning_kg_m2: 10.0,
                    matric_potential_mm: -1000.0,
                    hydraulic_conductivity_mm_s: 1e-5,
                    root_path_length_mm: 1.0,
                    gravity_root_mm: 1.0,
                    temperature_k: 290.0,
                    accessible: true,
                    frozen: false,
                },
                SoilLayerForcing {
                    layer_id: layer_id("soil-2"),
                    water_beginning_kg_m2: 10.0,
                    matric_potential_mm: -2000.0,
                    hydraulic_conductivity_mm_s: 1e-5,
                    root_path_length_mm: 1.0,
                    gravity_root_mm: 1.0,
                    temperature_k: 290.0,
                    accessible: true,
                    frozen: false,
                },
            ],
            gsi: 0.5,
        };
        assert_eq!(
            execute_candidate(&model, &config, &state, &forcing, &NoArbiter, &NoArbiter),
            Err(VegetationError::Unsupported(
                "V2 occupancy-local E04 transaction routing is implementation-incomplete"
            ))
        );
    }
}

fn tissue_carbon(state: &StratumSharedState, tissue: Tissue) -> Result<f64, VegetationError> {
    let pool = state
        .tissues
        .get(&tissue)
        .ok_or(VegetationError::Domain("missing tissue"))?;
    Ok(pool.display.carbon + pool.storage.carbon + pool.transfer.carbon)
}

/// V2 commit remains unavailable until occupancy-local candidate routing can
/// construct a fully validated candidate.
pub fn validate_and_commit(
    _beginning: &mut CoupledOwnedState,
    _candidate: CoupledCandidate,
) -> Result<CommitReceipt, VegetationError> {
    Err(VegetationError::Unsupported(
        "V2 occupancy-local E04 transaction routing is implementation-incomplete",
    ))
}

/// Failure-injection commit entry point retained while V2 routing is incomplete.
pub fn validate_and_commit_with_failure(
    beginning: &mut CoupledOwnedState,
    candidate: CoupledCandidate,
    failure: Option<FailurePoint>,
) -> Result<CommitReceipt, VegetationError> {
    if failure == Some(FailurePoint::OwnerValidation) {
        return Err(VegetationError::InjectedFailure("owner validation"));
    }
    if failure == Some(FailurePoint::BeforeCommit) {
        return Err(VegetationError::InjectedFailure("before commit"));
    }
    validate_and_commit(beginning, candidate)
}
