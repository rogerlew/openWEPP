//! Explicit default-off V9/LSE consumer over the real direct scheduler owner.
//!
//! This module owns only isolated shadow state. It has no production commit,
//! selector, publication, or output API.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_biogeochemistry::BiogeochemistryState;
use openwepp_kernel_contract::{
    MineralNitrogenKey, MineralNitrogenSpecies, ResourceAmountBasis, ResourceOwnerId,
    TransactionId, authorize_proportionally,
};
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyState, LandSurfaceForcing, Sha256Digest,
    SoilThermalLayerSnapshot, SoilThermalOfeSnapshot, SoilThermalSnapshot,
    SoilThermalTileCandidate, build_lse_ending_state,
};
use openwepp_vegetation::{
    NitrogenArbiter, NitrogenAuthorization, NitrogenRequest, SnowFreeForcing, V9CoupledOwnedState,
    V9StateError, VegetationConfiguration, VegetationError, project_v8_runtime_to_v9,
    project_v9_runtime_to_v8,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::land_surface_energy_shadow::{
    ExecuteV8LseRuntimeShadowError, LandSurfaceEnergyRealHydrologyAdapter,
    UncommittedCoveredV8OwnerEnvelope, V8CanopyForcingReceipt, execute_v8_lse_runtime_shadow,
    unified_beginning_hydrology_snapshot_sha256,
};
use crate::vegetation_real_hydrology_shadow::{
    RealHydrologyLaneLayerMap, RealHydrologyShadowAdapter,
};
use crate::{DirectOfeWb14Parameters, DirectRunFrame, DirectSurfaceLiquidConfiguration};

const INTERVALS_PER_DAY: usize = 48;
const INTERVAL_S: f64 = 1_800.0;

#[derive(Clone, Debug, PartialEq)]
pub struct DirectV9ShadowIntervalInput {
    pub lse_forcing: LandSurfaceForcing,
    pub vegetation_forcing: SnowFreeForcing,
    pub wb14_parameters: Vec<DirectOfeWb14Parameters>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectV9ShadowDayInput {
    pub day_index: usize,
    pub intervals: Vec<DirectV9ShadowIntervalInput>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectV9RealConsumerShadow {
    vegetation_configuration: VegetationConfiguration,
    vegetation_state: V9CoupledOwnedState,
    vegetation_owner_id: ResourceOwnerId,
    lse_configuration: LandSurfaceEnergyConfiguration,
    lse_state: LandSurfaceEnergyState,
    surface_configuration: DirectSurfaceLiquidConfiguration,
    layer_maps: Vec<RealHydrologyLaneLayerMap>,
    soil_thermal: SoilThermalSnapshot,
    biogeochemistry: BiogeochemistryState,
    hydrology_frame: DirectRunFrame,
    next_day_index: usize,
    accepted_interval_count: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectV9ShadowDayReceipt {
    pub day_index: usize,
    pub accepted_interval_count: usize,
    pub first_transaction_id: TransactionId,
    pub last_transaction_id: TransactionId,
    pub beginning_shadow_sha256: String,
    pub ending_shadow_sha256: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum DirectV9RealConsumerError {
    #[error("V9 real-consumer identity failure: {0}")]
    Identity(&'static str),
    #[error("V9 real-consumer unsupported domain: {0}")]
    Unsupported(&'static str),
    #[error("V9 real-consumer owner closure failure: {0}")]
    OwnerClosure(&'static str),
    #[error(transparent)]
    Vegetation(#[from] VegetationError),
    #[error(transparent)]
    V9(#[from] V9StateError),
    #[error(transparent)]
    Physical(#[from] ExecuteV8LseRuntimeShadowError),
    #[error("V9 real-consumer adapter failure: {0}")]
    Adapter(String),
}

impl DirectV9RealConsumerShadow {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        vegetation_configuration: VegetationConfiguration,
        vegetation_state: V9CoupledOwnedState,
        vegetation_owner_id: ResourceOwnerId,
        lse_configuration: LandSurfaceEnergyConfiguration,
        lse_state: LandSurfaceEnergyState,
        surface_configuration: DirectSurfaceLiquidConfiguration,
        layer_maps: Vec<RealHydrologyLaneLayerMap>,
        soil_thermal: SoilThermalSnapshot,
        biogeochemistry: BiogeochemistryState,
        hydrology_frame: DirectRunFrame,
        next_day_index: usize,
    ) -> Result<Self, DirectV9RealConsumerError> {
        vegetation_state.validate(&vegetation_configuration)?;
        let (v8_configuration, v8_state) =
            project_v9_runtime_to_v8(&vegetation_configuration, &vegetation_state)?;
        lse_configuration
            .validate()
            .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        lse_state
            .validate(&lse_configuration)
            .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        soil_thermal
            .validate()
            .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        if lse_configuration
            .vegetation_configuration
            .configuration_sha256
            .as_str()
            != v8_configuration.configuration_sha256
        {
            return Err(DirectV9RealConsumerError::Identity(
                "initial V9/V8/LSE configuration join",
            ));
        }
        if lse_state
            .last_accepted_transaction_id
            .is_some_and(|value| value.0 != v8_state.last_transaction_id)
        {
            return Err(DirectV9RealConsumerError::Identity(
                "initial vegetation/LSE transaction lineage",
            ));
        }
        if next_day_index >= hydrology_frame.identity.day_count
            || surface_configuration.run_id != hydrology_frame.identity.run_id
        {
            return Err(DirectV9RealConsumerError::Identity(
                "initial scheduler/surface owner identity",
            ));
        }
        let value = Self {
            vegetation_configuration,
            vegetation_state,
            vegetation_owner_id,
            lse_configuration,
            lse_state,
            surface_configuration,
            layer_maps,
            soil_thermal,
            biogeochemistry,
            hydrology_frame,
            next_day_index,
            accepted_interval_count: 0,
        };
        value.validate_complete_owner_set()?;
        Ok(value)
    }

    #[must_use]
    pub const fn next_day_index(&self) -> usize {
        self.next_day_index
    }

    #[must_use]
    pub const fn accepted_interval_count(&self) -> u64 {
        self.accepted_interval_count
    }

    #[must_use]
    pub const fn vegetation_state(&self) -> &V9CoupledOwnedState {
        &self.vegetation_state
    }

    #[must_use]
    pub const fn lse_state(&self) -> &LandSurfaceEnergyState {
        &self.lse_state
    }

    #[must_use]
    pub const fn soil_thermal(&self) -> &SoilThermalSnapshot {
        &self.soil_thermal
    }

    #[must_use]
    pub const fn biogeochemistry(&self) -> &BiogeochemistryState {
        &self.biogeochemistry
    }

    #[must_use]
    pub const fn hydrology_frame(&self) -> &DirectRunFrame {
        &self.hydrology_frame
    }

    pub fn execute_day(
        &mut self,
        production_frame: &DirectRunFrame,
        input: &DirectV9ShadowDayInput,
    ) -> Result<DirectV9ShadowDayReceipt, DirectV9RealConsumerError> {
        if input.day_index != self.next_day_index
            || input.day_index >= production_frame.identity.day_count
            || production_frame.identity != self.hydrology_frame.identity
        {
            return Err(DirectV9RealConsumerError::Identity(
                "scheduler day or production frame identity",
            ));
        }
        if input.intervals.len() != INTERVALS_PER_DAY {
            return Err(DirectV9RealConsumerError::Unsupported(
                "a shadow day requires exactly 48 intervals",
            ));
        }
        let beginning_shadow_sha256 = self.canonical_sha256()?;
        let first_transaction_id = input.intervals[0].lse_forcing.transaction_id;
        let last_transaction_id = input.intervals[INTERVALS_PER_DAY - 1]
            .lse_forcing
            .transaction_id;
        let mut candidate = self.clone();
        for (interval_index, interval) in input.intervals.iter().enumerate() {
            candidate.execute_interval(input.day_index, interval_index, interval)?;
        }
        candidate.next_day_index = candidate
            .next_day_index
            .checked_add(1)
            .ok_or(DirectV9RealConsumerError::Identity("shadow day overflow"))?;
        candidate.validate_complete_owner_set()?;
        let ending_shadow_sha256 = candidate.canonical_sha256()?;
        *self = candidate;
        Ok(DirectV9ShadowDayReceipt {
            day_index: input.day_index,
            accepted_interval_count: INTERVALS_PER_DAY,
            first_transaction_id,
            last_transaction_id,
            beginning_shadow_sha256,
            ending_shadow_sha256,
        })
    }

    fn execute_interval(
        &mut self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
    ) -> Result<(), DirectV9RealConsumerError> {
        let transaction_id = TransactionId(
            self.vegetation_state
                .0
                .last_transaction_id
                .checked_add(1)
                .ok_or(DirectV9RealConsumerError::Identity(
                    "vegetation transaction overflow",
                ))?,
        );
        let interval_index = u8::try_from(interval_index)
            .map_err(|_| DirectV9RealConsumerError::Identity("interval index overflow"))?;
        if input.lse_forcing.transaction_id != transaction_id
            || input.lse_forcing.interval_s.to_bits() != INTERVAL_S.to_bits()
            || input.lse_forcing.snow_present_at_beginning
            || input.lse_forcing.snow_present_at_end
            || input.lse_forcing.snow_terminal_payload_present
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "forcing transaction, cadence, or snow domain",
            ));
        }
        input
            .lse_forcing
            .validate(transaction_id)
            .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        let (v8_configuration, v8_beginning) =
            project_v9_runtime_to_v8(&self.vegetation_configuration, &self.vegetation_state)?;
        if self
            .lse_configuration
            .vegetation_configuration
            .configuration_sha256
            .as_str()
            != v8_configuration.configuration_sha256
        {
            return Err(DirectV9RealConsumerError::Identity(
                "V9/V8/LSE configuration join",
            ));
        }
        let hydrology = RealHydrologyShadowAdapter::try_from_day_start(
            &self.hydrology_frame,
            day_index,
            transaction_id,
            INTERVAL_S,
            self.surface_configuration.owner_id.clone(),
            &self.layer_maps,
        )
        .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hydrology);
        let hydrology_snapshot =
            unified_beginning_hydrology_snapshot_sha256(&soil_adapter, &self.surface_configuration)
                .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        let forcing_sha256 = input
            .lse_forcing
            .canonical_sha256()
            .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        let vegetation_forcing = project_live_vegetation_forcing(
            &input.vegetation_forcing,
            &hydrology,
            &self.soil_thermal,
        )?;
        let canopy_forcing = V8CanopyForcingReceipt::try_new(
            v8_configuration.configuration_sha256.clone(),
            v8_beginning.state_sha256.clone(),
            self.lse_configuration.configuration_sha256.clone(),
            forcing_sha256,
            hydrology_snapshot,
            self.soil_thermal.snapshot_sha256.clone(),
            transaction_id,
            vegetation_forcing,
        )
        .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        let nitrogen = BiogeochemistryNitrogenArbiter::new(&self.biogeochemistry);
        let envelope = execute_v8_lse_runtime_shadow(
            &v8_configuration,
            &v8_beginning,
            &self.vegetation_owner_id,
            &canopy_forcing,
            &self.lse_configuration,
            &self.lse_state,
            &input.lse_forcing,
            &soil_adapter,
            &self.surface_configuration,
            day_index,
            interval_index,
            &input.wb14_parameters,
            &self.soil_thermal,
            &nitrogen,
            &self.biogeochemistry,
        )?;
        self.accept_envelope(transaction_id, &envelope)
    }

    fn accept_envelope(
        &mut self,
        transaction_id: TransactionId,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
    ) -> Result<(), DirectV9RealConsumerError> {
        envelope
            .validate()
            .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        let vegetation_state = project_v8_runtime_to_v9(
            envelope.vegetation().ending_state(),
            &self.vegetation_configuration,
        )?;
        let lse_state = build_lse_ending_state(
            &self.lse_state,
            transaction_id,
            envelope.hydrology().ending_lse_tile_states().to_vec(),
        )
        .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        let soil_thermal = aggregate_soil_thermal_ending(
            &self.soil_thermal,
            &self.lse_configuration,
            transaction_id,
            envelope.hydrology().soil_thermal_candidates(),
        )?;
        self.vegetation_state = vegetation_state;
        self.lse_state = lse_state;
        self.soil_thermal = soil_thermal;
        self.biogeochemistry = envelope.biogeochemistry().ending().clone();
        self.hydrology_frame = envelope.hydrology().ending_frame().clone();
        self.accepted_interval_count = self.accepted_interval_count.checked_add(1).ok_or(
            DirectV9RealConsumerError::Identity("accepted interval count overflow"),
        )?;
        Ok(())
    }

    fn validate_complete_owner_set(&self) -> Result<(), DirectV9RealConsumerError> {
        self.vegetation_state
            .validate(&self.vegetation_configuration)?;
        self.lse_state
            .validate(&self.lse_configuration)
            .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        self.soil_thermal
            .validate()
            .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        if self.surface_configuration.ofe_bindings.len() != self.hydrology_frame.lanes.len()
            || self.layer_maps.len() != self.hydrology_frame.lanes.len()
            || self.biogeochemistry.last_transaction_id
                != self.vegetation_state.0.last_transaction_id
        {
            return Err(DirectV9RealConsumerError::Identity(
                "incomplete or mixed complete-owner state",
            ));
        }
        Ok(())
    }

    fn canonical_sha256(&self) -> Result<String, DirectV9RealConsumerError> {
        #[derive(Serialize)]
        struct ShadowBytes<'a> {
            vegetation: &'a V9CoupledOwnedState,
            lse: &'a LandSurfaceEnergyState,
            soil_thermal: &'a SoilThermalSnapshot,
            biogeochemistry: &'a BiogeochemistryState,
            hydrology_debug: String,
            next_day_index: usize,
            accepted_interval_count: u64,
        }
        let bytes = serde_json::to_vec(&ShadowBytes {
            vegetation: &self.vegetation_state,
            lse: &self.lse_state,
            soil_thermal: &self.soil_thermal,
            biogeochemistry: &self.biogeochemistry,
            hydrology_debug: format!("{:?}", self.hydrology_frame),
            next_day_index: self.next_day_index,
            accepted_interval_count: self.accepted_interval_count,
        })
        .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }
}

fn project_live_vegetation_forcing(
    provider: &SnowFreeForcing,
    hydrology: &RealHydrologyShadowAdapter,
    soil_thermal: &SoilThermalSnapshot,
) -> Result<SnowFreeForcing, DirectV9RealConsumerError> {
    let mut forcing = provider.clone();
    for layer in &mut forcing.soil_layers {
        let water_values = hydrology
            .layer_facts()
            .iter()
            .filter(|(source, _)| source.layer_id == layer.layer_id)
            .map(|(_, fact)| fact.liquid_supply_kg_m2)
            .collect::<Vec<_>>();
        let temperature_values = soil_thermal
            .ofes
            .iter()
            .filter_map(|ofe| {
                ofe.ordered_layers
                    .iter()
                    .find(|candidate| candidate.layer_id == layer.layer_id)
                    .map(|candidate| candidate.temperature_k)
            })
            .collect::<Vec<_>>();
        let water = common_provider_value(&water_values, "vegetation soil-water projection")?;
        let temperature = common_provider_value(
            &temperature_values,
            "vegetation soil-temperature projection",
        )?;
        layer.water_beginning_kg_m2 = water;
        layer.temperature_k = temperature;
    }
    Ok(forcing)
}

fn common_provider_value(
    values: &[f64],
    detail: &'static str,
) -> Result<f64, DirectV9RealConsumerError> {
    let first = values
        .first()
        .copied()
        .ok_or(DirectV9RealConsumerError::Identity(detail))?;
    if values
        .iter()
        .any(|value| value.to_bits() != first.to_bits())
    {
        return Err(DirectV9RealConsumerError::Unsupported(detail));
    }
    Ok(first)
}

struct BiogeochemistryNitrogenArbiter {
    available: BTreeMap<MineralNitrogenKey, f64>,
}

impl BiogeochemistryNitrogenArbiter {
    fn new(state: &BiogeochemistryState) -> Self {
        let mut available = BTreeMap::new();
        for (layer_id, layer) in &state.layers {
            if let Ok(layer_id) = openwepp_kernel_contract::SoilLayerId::try_new(layer_id.clone()) {
                available.insert(
                    MineralNitrogenKey {
                        layer_id: layer_id.clone(),
                        species: MineralNitrogenSpecies::Ammonium,
                    },
                    layer.ammonium_n,
                );
                available.insert(
                    MineralNitrogenKey {
                        layer_id,
                        species: MineralNitrogenSpecies::Nitrate,
                    },
                    layer.nitrate_n,
                );
            }
        }
        Self { available }
    }
}

impl NitrogenArbiter for BiogeochemistryNitrogenArbiter {
    fn beginning_amount(&self, key: &MineralNitrogenKey) -> Result<f64, VegetationError> {
        self.available
            .get(key)
            .copied()
            .ok_or(VegetationError::Domain("unknown nitrogen inventory"))
    }

    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
        authorize_proportionally(
            requests,
            &self.available,
            ResourceAmountBasis::NitrogenKgPerSquareMeterInterval,
        )
        .map_err(VegetationError::from)
    }
}

fn aggregate_soil_thermal_ending(
    beginning: &SoilThermalSnapshot,
    configuration: &LandSurfaceEnergyConfiguration,
    transaction_id: TransactionId,
    candidates: &[SoilThermalTileCandidate],
) -> Result<SoilThermalSnapshot, DirectV9RealConsumerError> {
    validate_soil_thermal_candidate_set(configuration, candidates)?;
    let mut ofes = Vec::with_capacity(beginning.ofes.len());
    for beginning_ofe in &beginning.ofes {
        ofes.push(aggregate_soil_thermal_ofe(
            beginning,
            beginning_ofe,
            configuration,
            candidates,
        )?);
    }
    let state_sha256 = digest_soil_state(&beginning.owner_id, transaction_id, &ofes)?;
    let snapshot_sha256 = digest_soil_snapshot(
        &beginning.owner_id,
        &beginning.configuration_sha256,
        &state_sha256,
        transaction_id,
        &ofes,
    )?;
    let ending = SoilThermalSnapshot {
        owner_id: beginning.owner_id.clone(),
        configuration_sha256: beginning.configuration_sha256.clone(),
        state_sha256,
        snapshot_sha256,
        last_accepted_transaction_id: Some(transaction_id),
        ofes,
    };
    ending
        .validate()
        .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
    Ok(ending)
}

fn validate_soil_thermal_candidate_set(
    configuration: &LandSurfaceEnergyConfiguration,
    candidates: &[SoilThermalTileCandidate],
) -> Result<(), DirectV9RealConsumerError> {
    let configured_tiles = configuration
        .ofes
        .iter()
        .flat_map(|ofe| {
            ofe.tiles.iter().map(move |tile| {
                (
                    (ofe.ofe_id.clone(), tile.tile_id.clone()),
                    tile.fraction_ofe_ground,
                )
            })
        })
        .collect::<BTreeMap<_, _>>();
    let actual_tiles = candidates
        .iter()
        .map(|candidate| (candidate.ofe_id.clone(), candidate.tile_id.clone()))
        .collect::<BTreeSet<_>>();
    if actual_tiles.len() != candidates.len()
        || actual_tiles != configured_tiles.keys().cloned().collect()
    {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal tile candidate set",
        ));
    }
    Ok(())
}

fn aggregate_soil_thermal_ofe(
    beginning: &SoilThermalSnapshot,
    beginning_ofe: &SoilThermalOfeSnapshot,
    configuration: &LandSurfaceEnergyConfiguration,
    candidates: &[SoilThermalTileCandidate],
) -> Result<SoilThermalOfeSnapshot, DirectV9RealConsumerError> {
    let configured_ofe = configuration
        .ofes
        .iter()
        .find(|ofe| ofe.ofe_id == beginning_ofe.ofe_id)
        .ok_or(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal OFE configuration",
        ))?;
    let tile_candidates = candidates
        .iter()
        .filter(|candidate| candidate.ofe_id == beginning_ofe.ofe_id)
        .collect::<Vec<_>>();
    if tile_candidates.len() != configured_ofe.tiles.len() {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal OFE tile cardinality",
        ));
    }
    let mut ordered_layers = Vec::with_capacity(beginning_ofe.ordered_layers.len());
    for (layer_index, beginning_layer) in beginning_ofe.ordered_layers.iter().enumerate() {
        let configured_layer = configured_ofe
            .soil_interface_layers
            .get(layer_index)
            .ok_or(DirectV9RealConsumerError::OwnerClosure(
                "soil-thermal configured layer order",
            ))?;
        if configured_layer.layer_id != beginning_layer.layer_id
            || !configured_layer.areal_heat_capacity_j_m2_k.is_finite()
            || configured_layer.areal_heat_capacity_j_m2_k <= 0.0
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "soil-thermal layer identity or capacity",
            ));
        }
        let mut ending_enthalpy = beginning_layer.enthalpy_j_m2_ofe_ground;
        for candidate in &tile_candidates {
            if candidate.owner_id != beginning.owner_id
                || candidate.beginning_state_sha256 != beginning.state_sha256
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "soil-thermal candidate owner lineage",
                ));
            }
            let layer = candidate.layers.get(layer_index).ok_or(
                DirectV9RealConsumerError::OwnerClosure("soil-thermal candidate layer cardinality"),
            )?;
            if layer.layer_id != beginning_layer.layer_id
                || layer.beginning_enthalpy_j_m2_ofe_ground.to_bits()
                    != beginning_layer.enthalpy_j_m2_ofe_ground.to_bits()
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "soil-thermal candidate beginning layer",
                ));
            }
            ending_enthalpy +=
                layer.ending_enthalpy_j_m2_ofe_ground - layer.beginning_enthalpy_j_m2_ofe_ground;
        }
        let ending_temperature_k = beginning_layer.temperature_k
            + (ending_enthalpy - beginning_layer.enthalpy_j_m2_ofe_ground)
                / configured_layer.areal_heat_capacity_j_m2_k;
        if !ending_enthalpy.is_finite() || !(200.0..=350.0).contains(&ending_temperature_k) {
            return Err(DirectV9RealConsumerError::Unsupported(
                "aggregated soil-thermal ending domain",
            ));
        }
        ordered_layers.push(SoilThermalLayerSnapshot {
            layer_id: beginning_layer.layer_id.clone(),
            temperature_k: ending_temperature_k,
            enthalpy_j_m2_ofe_ground: ending_enthalpy,
        });
    }
    Ok(SoilThermalOfeSnapshot {
        ofe_id: beginning_ofe.ofe_id.clone(),
        ordered_layers,
    })
}

fn digest_soil_state(
    owner_id: &ResourceOwnerId,
    transaction_id: TransactionId,
    ofes: &[SoilThermalOfeSnapshot],
) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    digest_serialized(&(owner_id, transaction_id, ofes))
}

fn digest_soil_snapshot(
    owner_id: &ResourceOwnerId,
    configuration_sha256: &Sha256Digest,
    state_sha256: &Sha256Digest,
    transaction_id: TransactionId,
    ofes: &[SoilThermalOfeSnapshot],
) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    digest_serialized(&(
        owner_id,
        configuration_sha256,
        state_sha256,
        transaction_id,
        ofes,
    ))
}

fn digest_serialized<T: Serialize>(value: &T) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))?;
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|error| DirectV9RealConsumerError::Adapter(error.to_string()))
}

#[cfg(test)]
mod tests {
    use openwepp_land_surface_energy::OfeId;
    use openwepp_vegetation::{V9_MODEL_SHA256, V9CoupledOwnedState};

    use super::*;
    use crate::land_surface_energy_shadow::{EndpointFixture, endpoint_fixture};
    use crate::{
        DirectExecutorMode, DirectFrameExecutor, DirectPublicationCalendarDay,
        DirectPublicationDayInput, DirectPublicationRunMetadata,
    };

    fn v9_configuration_and_state(
        fixture: &EndpointFixture,
    ) -> (VegetationConfiguration, V9CoupledOwnedState) {
        let mut configuration = fixture.vegetation_configuration.clone();
        configuration.model_definition_sha256 = V9_MODEL_SHA256.into();
        configuration.configuration_sha256 = configuration
            .canonical_sha256()
            .expect("V9 configuration digest");
        let mut state = fixture.vegetation_state.clone();
        state.model_definition_sha256 = V9_MODEL_SHA256.into();
        state
            .configuration_sha256
            .clone_from(&configuration.configuration_sha256);
        state.state_sha256 = state.canonical_sha256();
        let state = V9CoupledOwnedState(state);
        state.validate(&configuration).expect("V9 fixture state");
        (configuration, state)
    }

    fn shadow_fixture() -> (DirectV9RealConsumerShadow, EndpointFixture) {
        let fixture = endpoint_fixture();
        let (configuration, state) = v9_configuration_and_state(&fixture);
        let shadow = DirectV9RealConsumerShadow::try_new(
            configuration,
            state,
            ResourceOwnerId::try_new("vegetation-v8").expect("owner"),
            fixture.lse_configuration.clone(),
            fixture.lse_state.clone(),
            fixture.surface_configuration.clone(),
            fixture.hydrology.layer_maps().to_vec(),
            fixture.thermal.clone(),
            fixture.biogeochemistry.clone(),
            fixture.hydrology.beginning_frame().clone(),
            0,
        )
        .expect("shadow fixture");
        (shadow, fixture)
    }

    fn day_input(fixture: &EndpointFixture) -> DirectV9ShadowDayInput {
        let base_vegetation = fixture.receipt.forcing().clone();
        let intervals = (0..INTERVALS_PER_DAY)
            .map(|index| {
                let mut forcing = fixture.forcing.clone();
                forcing.transaction_id = TransactionId(41 + index as u128);
                forcing.forcing_sha256 = forcing.canonical_sha256().expect("forcing digest");
                DirectV9ShadowIntervalInput {
                    lse_forcing: forcing,
                    vegetation_forcing: base_vegetation.clone(),
                    wb14_parameters: vec![DirectOfeWb14Parameters {
                        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                        effective_conductivity_m_s: 1e-6,
                        matric_potential_m: 0.1,
                        infiltration_storage_capacity_m: 0.04,
                    }],
                }
            })
            .collect();
        DirectV9ShadowDayInput {
            day_index: 0,
            intervals,
        }
    }

    #[test]
    fn forty_eight_interval_day_replaces_only_complete_shadow_state() {
        let (mut shadow, fixture) = shadow_fixture();
        let production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let receipt = shadow
            .execute_day(&production, &day_input(&fixture))
            .expect("complete shadow day");
        assert_eq!(receipt.accepted_interval_count, 48);
        assert_eq!(receipt.first_transaction_id, TransactionId(41));
        assert_eq!(receipt.last_transaction_id, TransactionId(88));
        assert_eq!(shadow.accepted_interval_count(), 48);
        assert_eq!(shadow.vegetation_state().0.last_transaction_id, 88);
        assert_eq!(production, production_before);
        assert_ne!(
            receipt.beginning_shadow_sha256,
            receipt.ending_shadow_sha256
        );
    }

    #[test]
    fn failed_late_interval_rolls_back_every_shadow_and_production_byte() {
        let (mut shadow, fixture) = shadow_fixture();
        let production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let shadow_before = shadow.clone();
        let mut input = day_input(&fixture);
        input.intervals[47].lse_forcing.snow_present_at_end = true;
        assert!(matches!(
            shadow.execute_day(&production, &input),
            Err(DirectV9RealConsumerError::Unsupported(_))
        ));
        assert_eq!(shadow, shadow_before);
        assert_eq!(production, production_before);
    }

    #[test]
    fn retained_half_day_restart_is_byte_identical_to_uninterrupted_day() {
        let (mut uninterrupted, fixture) = shadow_fixture();
        let input = day_input(&fixture);
        for (index, interval) in input.intervals.iter().enumerate() {
            uninterrupted
                .execute_interval(0, index, interval)
                .expect("uninterrupted interval");
        }
        let (mut first_half, _) = shadow_fixture();
        for (index, interval) in input.intervals[..24].iter().enumerate() {
            first_half
                .execute_interval(0, index, interval)
                .expect("first restart half");
        }
        let mut restarted = first_half.clone();
        for (index, interval) in input.intervals[24..].iter().enumerate() {
            restarted
                .execute_interval(0, index + 24, interval)
                .expect("second restart half");
        }
        assert_eq!(restarted, uninterrupted);
        assert_eq!(
            restarted.canonical_sha256().expect("restarted bytes"),
            uninterrupted
                .canonical_sha256()
                .expect("uninterrupted bytes")
        );
    }

    #[test]
    fn explicit_scheduler_consumer_advances_shadow_without_changing_production() {
        let (mut shadow, fixture) = shadow_fixture();
        let mut baseline = fixture.hydrology.beginning_frame().clone();
        let mut observed = baseline.clone();
        let production_input =
            DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
                year: 2026,
                julian_day: 1,
                month: 1,
                day_of_month: 1,
                water_year: 2026,
            });
        let metadata = DirectPublicationRunMetadata {
            run_name: "v9-real-consumer-shadow".into(),
            runtime_selection: "direct-default-off-shadow-test".into(),
            output_policy: "test-only".into(),
        };
        let executor = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly);
        let mut baseline_rows = Vec::new();
        let baseline_report = executor
            .run_publication_stream_with_interleaved_day_inputs_and_day_frames(
                &mut baseline,
                metadata.clone(),
                |_, _, _| Ok(production_input.clone()),
                |row, _| {
                    baseline_rows.push(row.clone());
                    Ok(())
                },
            )
            .expect("baseline production run");
        let shadow_input = day_input(&fixture);
        let mut observed_rows = Vec::new();
        let observed_report = executor
            .run_publication_stream_with_v9_real_consumer_shadow(
                &mut observed,
                metadata,
                |_, _, _| Ok(production_input.clone()),
                |_, _| Ok(shadow_input.clone()),
                |row, _| {
                    observed_rows.push(row.clone());
                    Ok(())
                },
                &mut shadow,
            )
            .expect("explicit default-off shadow run");
        assert_eq!(observed, baseline);
        assert_eq!(observed_rows, baseline_rows);
        assert_eq!(observed_report, baseline_report);
        assert_eq!(shadow.accepted_interval_count(), INTERVALS_PER_DAY as u64);
    }

    #[test]
    fn downstream_scheduler_failure_discards_production_and_complete_shadow_candidate() {
        let (mut shadow, fixture) = shadow_fixture();
        let mut production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let shadow_before = shadow.clone();
        let production_input =
            DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
                year: 2026,
                julian_day: 1,
                month: 1,
                day_of_month: 1,
                water_year: 2026,
            });
        let error = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
            .run_publication_stream_with_v9_real_consumer_shadow(
                &mut production,
                DirectPublicationRunMetadata {
                    run_name: "v9-shadow-rollback".into(),
                    runtime_selection: "direct-default-off-shadow-test".into(),
                    output_policy: "test-only".into(),
                },
                |_, _, _| Ok(production_input.clone()),
                |_, _| Ok(day_input(&fixture)),
                |_, _| {
                    Err(crate::DirectRuntimeError::PublicationSinkFailure {
                        detail: "injected after shadow day".into(),
                    })
                },
                &mut shadow,
            )
            .expect_err("injected downstream failure");
        assert!(matches!(
            error,
            crate::DirectRuntimeError::PublicationSinkFailure { .. }
        ));
        assert_eq!(production, production_before);
        assert_eq!(shadow, shadow_before);
    }
}
