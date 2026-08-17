//! Strict, solve-free projection of accepted V8/LSE/hydrology owner inputs.
//!
//! This boundary deliberately stops before maximum-demand evaluation, water
//! arbitration, or a candidate solve. In particular, Emax is not a caller
//! input: it is produced later by the admitted coupled solver.

use std::collections::BTreeMap;

use openwepp_kernel_contract::{OccupancyId, ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use openwepp_land_surface_energy::{
    BandDirectionalFluxes, BiochemicalConstants, ComponentId, CoveredColumnInputs,
    CoveredColumnShortwaveInputs, CoveredOccupancyInputs, CoveredOccupancyShortwaveInputs,
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyError, LandSurfaceEnergyState,
    LandSurfaceForcing, LeafBiochemicalInputs, OfeId, OpenNeutralGeometry, OpenSurfaceProblem,
    RootHydraulicLayer, RootRuntimeIdentity, RuntimeTileIdentity, SoilInterfaceLayer,
    SoilThermalNodeOperands, SoilThermalOfeSnapshot, SoilThermalSnapshot, SourceId,
    SurfaceClassKind, SurfaceConfiguration, SurfaceHeatStorageMode, SurfaceStorageBranch,
    TileConfiguration, TileState, TurbulenceConfiguration,
};
use openwepp_vegetation::carbon_nitrogen::{Tissue, atkin_rd25};
use openwepp_vegetation::energy::{
    LATENT_HEAT_VAPORIZATION, canopy_surface_friction_velocity, leaf_boundary_conductance,
    neutral_resistance,
};
use openwepp_vegetation::radiation::{
    ColumnRadiationResult, IncidentComponent, MixedLayer, RadiationBand, SurfaceOptics,
    solve_mixed_column,
};
use openwepp_vegetation::{
    SnowFreeForcing, SoilLayerForcing, StratumConfiguration, StratumSharedState, V8_MODEL_SHA256,
    V8ComponentOccupancyBinding, V8CoupledOwnedState, V8LseComponentId, V8OccupancyState,
    V8TileCanopyAirState, VegetationConfiguration, VegetationError,
};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{
    DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidStateRecord,
    vegetation_real_hydrology_shadow::{RealHydrologyLayerFact, RealHydrologySourceKey},
};

use super::{
    LandSurfaceEnergyRealHydrologyAdapter, Sha256Digest,
    unified_beginning_hydrology_snapshot_sha256,
};

#[derive(Clone, Debug, Error, PartialEq)]
pub enum V8InputProjectionError {
    #[error("V8 input projection identity failure: {0}")]
    Identity(&'static str),
    #[error("V8 input projection topology failure: {0}")]
    Topology(&'static str),
    #[error(transparent)]
    Vegetation(#[from] VegetationError),
    #[error(transparent)]
    LandSurface(#[from] LandSurfaceEnergyError),
    #[error(transparent)]
    Shadow(#[from] super::LandSurfaceEnergyShadowError),
}

/// Digest-bound canopy and root-hydraulic forcing joined to the exact LSE and
/// owner snapshots. Fields are private so a caller cannot mutate an accepted
/// receipt after construction.
#[derive(Clone, Debug, PartialEq)]
pub struct V8CanopyForcingReceipt {
    model_definition_sha256: String,
    vegetation_configuration_sha256: String,
    vegetation_beginning_state_sha256: String,
    lse_configuration_sha256: Sha256Digest,
    lse_forcing_sha256: Sha256Digest,
    hydrology_snapshot_sha256: Sha256Digest,
    soil_thermal_snapshot_sha256: Sha256Digest,
    transaction_id: TransactionId,
    forcing_sha256: String,
    forcing: SnowFreeForcing,
}

impl V8CanopyForcingReceipt {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        vegetation_configuration_sha256: String,
        vegetation_beginning_state_sha256: String,
        lse_configuration_sha256: Sha256Digest,
        lse_forcing_sha256: Sha256Digest,
        hydrology_snapshot_sha256: Sha256Digest,
        soil_thermal_snapshot_sha256: Sha256Digest,
        transaction_id: TransactionId,
        forcing: SnowFreeForcing,
    ) -> Result<Self, V8InputProjectionError> {
        let mut value = Self {
            model_definition_sha256: V8_MODEL_SHA256.into(),
            vegetation_configuration_sha256,
            vegetation_beginning_state_sha256,
            lse_configuration_sha256,
            lse_forcing_sha256,
            hydrology_snapshot_sha256,
            soil_thermal_snapshot_sha256,
            transaction_id,
            forcing_sha256: String::new(),
            forcing,
        };
        value.forcing_sha256 = value.canonical_sha256();
        value.validate_digest()?;
        Ok(value)
    }

    #[must_use]
    pub fn forcing(&self) -> &SnowFreeForcing {
        &self.forcing
    }

    fn validate_digest(&self) -> Result<(), V8InputProjectionError> {
        if self.model_definition_sha256 != V8_MODEL_SHA256
            || self.forcing_sha256 != self.canonical_sha256()
        {
            return Err(V8InputProjectionError::Identity("V8 canopy forcing digest"));
        }
        Ok(())
    }

    fn canonical_sha256(&self) -> String {
        let mut digest = Sha256::new();
        for value in [
            self.model_definition_sha256.as_bytes(),
            self.vegetation_configuration_sha256.as_bytes(),
            self.vegetation_beginning_state_sha256.as_bytes(),
            self.lse_configuration_sha256.as_str().as_bytes(),
            self.lse_forcing_sha256.as_str().as_bytes(),
            self.hydrology_snapshot_sha256.as_str().as_bytes(),
            self.soil_thermal_snapshot_sha256.as_str().as_bytes(),
        ] {
            digest.update((value.len() as u64).to_le_bytes());
            digest.update(value);
        }
        digest.update(self.transaction_id.0.to_le_bytes());
        hash_snow_free_forcing(&mut digest, &self.forcing);
        format!("{:x}", digest.finalize())
    }
}

fn hash_snow_free_forcing(digest: &mut Sha256, forcing: &SnowFreeForcing) {
    for value in [
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
    ] {
        digest.update(value.to_bits().to_le_bytes());
    }
    digest.update((forcing.soil_layers.len() as u64).to_le_bytes());
    for layer in &forcing.soil_layers {
        let identity = layer.layer_id.as_str().as_bytes();
        digest.update((identity.len() as u64).to_le_bytes());
        digest.update(identity);
        for value in [
            layer.water_beginning_kg_m2,
            layer.matric_potential_mm,
            layer.hydraulic_conductivity_mm_s,
            layer.root_path_length_mm,
            layer.gravity_root_mm,
            layer.temperature_k,
        ] {
            digest.update(value.to_bits().to_le_bytes());
        }
        digest.update([u8::from(layer.accessible), u8::from(layer.frozen)]);
    }
}

/// Four exact E01--E03 column solutions for one ground tile.
#[derive(Clone, Debug, PartialEq)]
pub struct V8ProjectedColumnRadiation {
    pub visible_direct: ColumnRadiationResult,
    pub visible_diffuse: ColumnRadiationResult,
    pub near_infrared_direct: ColumnRadiationResult,
    pub near_infrared_diffuse: ColumnRadiationResult,
}

/// One real production layer joined to the V8 hydraulic forcing lane.
#[derive(Clone, Debug, PartialEq)]
pub struct V8ProjectedRootLayer {
    pub forcing: SoilLayerForcing,
    pub source: RealHydrologySourceKey,
    pub fact: RealHydrologyLayerFact,
}

/// Immutable source operands for one ordered V8 occupancy.
#[derive(Clone, Debug, PartialEq)]
pub struct V8ProjectedOccupancyInput {
    pub occupancy_id: OccupancyId,
    pub vertical_rank: u32,
    pub conditional_lai_m2_m2_tile_ground: f64,
    pub conditional_sai_m2_m2_tile_ground: f64,
    pub stratum_configuration: StratumConfiguration,
    pub shared_state: StratumSharedState,
    pub occupancy_state: V8OccupancyState,
    pub root_layers: Vec<V8ProjectedRootLayer>,
}

/// Exact ground owner inputs for one LSE tile.
#[derive(Clone, Debug, PartialEq)]
pub struct V8ProjectedGroundInput {
    pub configuration: TileConfiguration,
    pub soil_interface_layers: Vec<SoilInterfaceLayer>,
    pub state: TileState,
    pub surface_liquid: DirectSurfaceLiquidStateRecord,
    pub soil_thermal: SoilThermalOfeSnapshot,
}

/// Fully projected, solve-free runtime input for one OFE/tile.
#[derive(Clone, Debug, PartialEq)]
pub struct V8ProjectedTileRuntimeInput {
    pub identity: RuntimeTileIdentity,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub transaction_id: TransactionId,
    pub interval_s: f64,
    pub tile_fraction: f64,
    pub forcing: LandSurfaceForcing,
    pub vegetation_forcing: SnowFreeForcing,
    pub canopy_air_state: Option<V8TileCanopyAirState>,
    pub radiation: V8ProjectedColumnRadiation,
    pub ground: V8ProjectedGroundInput,
    pub occupancies: Vec<V8ProjectedOccupancyInput>,
}

/// Cross-owner lineage plus every per-tile input. This type cannot solve or
/// commit and contains no caller-provided numerical trial or Emax value.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidatedV8RuntimeInputProjection {
    pub vegetation_configuration_sha256: String,
    pub vegetation_state_sha256: String,
    pub lse_configuration_sha256: Sha256Digest,
    pub lse_state_sha256: Sha256Digest,
    pub lse_forcing_sha256: Sha256Digest,
    pub hydrology_snapshot_sha256: Sha256Digest,
    pub soil_thermal_snapshot_sha256: Sha256Digest,
    pub transaction_id: TransactionId,
    pub tiles: Vec<V8ProjectedTileRuntimeInput>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum V8SolverReadyTilePhysics {
    Open(OpenSurfaceProblem),
    Covered(CoveredColumnInputs),
}

/// Opaque solver-ready tile. Every physical operand and numerical warm start
/// is derived from the validated projection; consumers supply no raw arrays.
#[derive(Clone, Debug, PartialEq)]
pub struct V8SolverReadyTileInput {
    pub identity: RuntimeTileIdentity,
    pub physics: V8SolverReadyTilePhysics,
    pub root_identities: Vec<RootRuntimeIdentity>,
    pub soil_sources:
        BTreeMap<openwepp_land_surface_energy::GroundWaterKey, RealHydrologySourceKey>,
    pub beginning_trial: Vec<f64>,
    pub vegetation_bindings: Vec<V8ComponentOccupancyBinding>,
    pub soil_thermal: SoilThermalOfeSnapshot,
}

impl ValidatedV8RuntimeInputProjection {
    pub fn solver_ready_tiles(
        &self,
        vegetation_owner_id: &ResourceOwnerId,
    ) -> Result<Vec<V8SolverReadyTileInput>, V8InputProjectionError> {
        self.tiles
            .iter()
            .map(|tile| tile.solver_ready(vegetation_owner_id))
            .collect()
    }
}

/// Derive all solve inputs from validated owner configuration/state and the
/// canonical forcing DTOs. No LAI/SAI, rank, warm start, ground state, soil
/// state, or hydraulic source operand is accepted independently.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn project_v8_runtime_inputs(
    vegetation_configuration: &VegetationConfiguration,
    vegetation_state: &V8CoupledOwnedState,
    canopy_forcing: &V8CanopyForcingReceipt,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    lse_state: &LandSurfaceEnergyState,
    lse_forcing: &LandSurfaceForcing,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    soil_thermal: &SoilThermalSnapshot,
) -> Result<ValidatedV8RuntimeInputProjection, V8InputProjectionError> {
    vegetation_configuration.validate_v8()?;
    vegetation_state
        .validate(vegetation_configuration)
        .map_err(|_| V8InputProjectionError::Identity("invalid V8 vegetation state"))?;
    lse_configuration.validate()?;
    lse_state.validate(lse_configuration)?;
    let transaction_id = TransactionId(
        vegetation_state
            .last_transaction_id
            .checked_add(1)
            .ok_or(V8InputProjectionError::Identity("transaction overflow"))?,
    );
    lse_state.validate_transaction_lineage(transaction_id)?;
    lse_forcing.validate(transaction_id)?;
    soil_thermal.validate()?;
    let hydrology_snapshot_sha256 =
        unified_beginning_hydrology_snapshot_sha256(soil_adapter, surface_configuration)?;
    canopy_forcing.validate_digest()?;
    validate_cross_owner_lineage(
        vegetation_configuration,
        vegetation_state,
        lse_configuration,
        lse_forcing,
        soil_adapter,
        soil_thermal,
        transaction_id,
        canopy_forcing,
        &hydrology_snapshot_sha256,
    )?;
    validate_forcing_join(
        vegetation_configuration,
        canopy_forcing.forcing(),
        lse_forcing,
    )?;
    let surface_state = soil_adapter
        .owner
        .beginning_frame()
        .surface_liquid_shadow
        .as_deref()
        .ok_or(V8InputProjectionError::Identity(
            "missing surface-liquid beginning owner",
        ))?;
    surface_state
        .validate(surface_configuration)
        .map_err(|_| V8InputProjectionError::Identity("invalid surface-liquid beginning owner"))?;

    let mut tiles = Vec::new();
    for ofe in &lse_configuration.ofes {
        let thermal = soil_thermal
            .ofes
            .iter()
            .find(|value| value.ofe_id == ofe.ofe_id)
            .ok_or(V8InputProjectionError::Topology("missing soil-thermal OFE"))?;
        let lane = surface_configuration
            .ofe_bindings
            .iter()
            .find(|value| value.ofe_id == ofe.ofe_id)
            .ok_or(V8InputProjectionError::Topology(
                "missing production OFE binding",
            ))?;
        for tile in &ofe.tiles {
            let vegetation_tile = vegetation_configuration
                .topology_tiles
                .iter()
                .find(|value| value.tile_id == tile.vegetation_tile_id)
                .ok_or(V8InputProjectionError::Topology("missing vegetation tile"))?;
            if tile.fraction_ofe_ground.to_bits() != vegetation_tile.fraction.to_bits() {
                return Err(V8InputProjectionError::Identity("tile fraction mismatch"));
            }
            let tile_state = lse_state
                .tiles
                .iter()
                .find(|value| value.ofe_id == ofe.ofe_id && value.tile_id == tile.tile_id)
                .ok_or(V8InputProjectionError::Topology("missing LSE tile state"))?;
            let surface_record = surface_state
                .records
                .iter()
                .find(|value| value.key.ofe_id == ofe.ofe_id && value.key.tile_id == tile.tile_id)
                .ok_or(V8InputProjectionError::Topology(
                    "missing surface-liquid tile state",
                ))?;
            let (radiation, occupancies) = project_column(
                vegetation_configuration,
                vegetation_state,
                canopy_forcing.forcing(),
                &tile.vegetation_tile_id,
                lane.production_lane_index,
                lane.production_lane_id,
                soil_adapter,
            )?;
            let canopy_air_state = vegetation_state
                .tile_canopy_air
                .get(&tile.vegetation_tile_id)
                .cloned();
            if occupancies.is_empty() != canopy_air_state.is_none() {
                return Err(V8InputProjectionError::Topology(
                    "canopy-air/occupancy topology mismatch",
                ));
            }
            tiles.push(V8ProjectedTileRuntimeInput {
                identity: RuntimeTileIdentity {
                    transaction_id,
                    lse_owner_id: lse_configuration.owner_id.clone(),
                    hydrology_owner_id: lse_configuration.hydrology_configuration.owner_id.clone(),
                    soil_thermal_owner_id: lse_configuration
                        .soil_thermal_configuration
                        .owner_id
                        .clone(),
                    configuration_sha256: lse_configuration.configuration_sha256.clone(),
                    beginning_lse_state_sha256: lse_state.state_sha256.clone(),
                    beginning_hydrology_snapshot_sha256: hydrology_snapshot_sha256.clone(),
                    beginning_soil_thermal_state_sha256: soil_thermal.state_sha256.clone(),
                    ofe_id: ofe.ofe_id.clone(),
                    tile_id: tile.tile_id.clone(),
                    surface_id: surface_record.key.surface_id.clone(),
                    surface_class: surface_record.key.surface_class,
                    ground_source_type: surface_record.key.source_type,
                    ground_source_id: surface_record.key.source_id.clone(),
                    ground_source_tile_id: Some(tile.tile_id.clone()),
                    ground_soil_layer_id: None,
                    tile_fraction: tile.fraction_ofe_ground,
                    interval_s: lse_forcing.interval_s,
                },
                ofe_id: ofe.ofe_id.clone(),
                tile_id: tile.tile_id.clone(),
                transaction_id,
                interval_s: lse_forcing.interval_s,
                tile_fraction: tile.fraction_ofe_ground,
                forcing: lse_forcing.clone(),
                vegetation_forcing: canopy_forcing.forcing().clone(),
                canopy_air_state,
                radiation,
                ground: V8ProjectedGroundInput {
                    configuration: tile.clone(),
                    soil_interface_layers: ofe.soil_interface_layers.clone(),
                    state: tile_state.clone(),
                    surface_liquid: surface_record.clone(),
                    soil_thermal: thermal.clone(),
                },
                occupancies,
            });
        }
    }
    tiles
        .sort_by(|left, right| (&left.ofe_id, &left.tile_id).cmp(&(&right.ofe_id, &right.tile_id)));
    Ok(ValidatedV8RuntimeInputProjection {
        vegetation_configuration_sha256: vegetation_configuration.configuration_sha256.clone(),
        vegetation_state_sha256: vegetation_state.state_sha256.clone(),
        lse_configuration_sha256: lse_configuration.configuration_sha256.clone(),
        lse_state_sha256: lse_state.state_sha256.clone(),
        lse_forcing_sha256: lse_forcing.forcing_sha256.clone(),
        hydrology_snapshot_sha256,
        soil_thermal_snapshot_sha256: soil_thermal.snapshot_sha256.clone(),
        transaction_id,
        tiles,
    })
}

#[allow(clippy::too_many_arguments)]
fn validate_cross_owner_lineage(
    vegetation_configuration: &VegetationConfiguration,
    vegetation_state: &V8CoupledOwnedState,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    lse_forcing: &LandSurfaceForcing,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    soil_thermal: &SoilThermalSnapshot,
    transaction_id: TransactionId,
    canopy_forcing: &V8CanopyForcingReceipt,
    hydrology_snapshot_sha256: &Sha256Digest,
) -> Result<(), V8InputProjectionError> {
    if lse_configuration
        .vegetation_configuration
        .configuration_sha256
        .as_str()
        != vegetation_configuration.configuration_sha256
        || vegetation_state.configuration_sha256 != vegetation_configuration.configuration_sha256
        || lse_configuration
            .soil_thermal_configuration
            .configuration_sha256
            != soil_thermal.configuration_sha256
        || lse_forcing.transaction_id != transaction_id
        || soil_adapter.owner.transaction_id() != transaction_id
        || soil_adapter.owner.interval_s().to_bits() != lse_forcing.interval_s.to_bits()
        || canopy_forcing.vegetation_configuration_sha256
            != vegetation_configuration.configuration_sha256
        || canopy_forcing.vegetation_beginning_state_sha256 != vegetation_state.state_sha256
        || canopy_forcing.lse_configuration_sha256 != lse_configuration.configuration_sha256
        || canopy_forcing.lse_forcing_sha256 != lse_forcing.forcing_sha256
        || &canopy_forcing.hydrology_snapshot_sha256 != hydrology_snapshot_sha256
        || canopy_forcing.soil_thermal_snapshot_sha256 != soil_thermal.snapshot_sha256
        || canopy_forcing.transaction_id != transaction_id
    {
        return Err(V8InputProjectionError::Identity("cross-owner lineage"));
    }
    Ok(())
}

fn validate_forcing_join(
    vegetation_configuration: &VegetationConfiguration,
    vegetation: &SnowFreeForcing,
    lse: &LandSurfaceForcing,
) -> Result<(), V8InputProjectionError> {
    let exact = vegetation_configuration.dt_s.to_bits() == lse.interval_s.to_bits()
        && vegetation.air_temperature_k.to_bits() == lse.air_temperature_k.to_bits()
        && vegetation.pressure_pa.to_bits() == lse.air_pressure_pa.to_bits()
        && vegetation.wind_m_s.to_bits() == lse.reference_wind_m_s.to_bits()
        && vegetation.specific_humidity.to_bits() == lse.air_specific_humidity_kg_kg.to_bits()
        && vegetation.direct_par_w_m2.to_bits() == lse.direct_vis_w_m2.to_bits()
        && vegetation.diffuse_par_w_m2.to_bits() == lse.diffuse_vis_w_m2.to_bits()
        && vegetation.direct_nir_w_m2.to_bits() == lse.direct_nir_w_m2.to_bits()
        && vegetation.diffuse_nir_w_m2.to_bits() == lse.diffuse_nir_w_m2.to_bits()
        && vegetation.longwave_down_w_m2.to_bits()
            == lse.atmospheric_downward_longwave_w_m2.to_bits();
    if !exact {
        return Err(V8InputProjectionError::Identity("forcing operand join"));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn project_column(
    configuration: &VegetationConfiguration,
    state: &V8CoupledOwnedState,
    forcing: &SnowFreeForcing,
    tile_id: &TileId,
    lane_index: usize,
    lane_id: u32,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
) -> Result<(V8ProjectedColumnRadiation, Vec<V8ProjectedOccupancyInput>), V8InputProjectionError> {
    let mut strata = configuration
        .strata
        .iter()
        .filter(|value| value.tile_ids.contains(tile_id))
        .collect::<Vec<_>>();
    strata.sort_by_key(|value| value.vertical_rank);
    let mut layers_vis = Vec::new();
    let mut layers_nir = Vec::new();
    let mut identities = Vec::new();
    let mut areas = Vec::new();
    for stratum in &strata {
        let shared = state
            .strata
            .get(&stratum.stratum_id)
            .ok_or(V8InputProjectionError::Topology("missing shared stratum"))?;
        let coverage = configuration.stratum_coverage(&stratum.stratum_id)?;
        let lai = shared.leaf_area / coverage;
        let sai = shared.stem_area / coverage;
        identities.push(OccupancyId {
            stratum_id: stratum.stratum_id.clone(),
            tile_id: tile_id.clone(),
        });
        areas.push((lai, sai));
        layers_vis.push(mixed_layer(stratum, lai, sai, true));
        layers_nir.push(mixed_layer(stratum, lai, sai, false));
    }
    let visible_direct = solve_mixed_column(
        &layers_vis,
        RadiationBand::Visible,
        IncidentComponent::Direct,
        forcing.solar_zenith_cosine,
        forcing.ground_albedo_vis,
        forcing.direct_par_w_m2,
    )?;
    let visible_diffuse = solve_mixed_column(
        &layers_vis,
        RadiationBand::Visible,
        IncidentComponent::Diffuse,
        forcing.solar_zenith_cosine,
        forcing.ground_albedo_vis,
        forcing.diffuse_par_w_m2,
    )?;
    let near_infrared_direct = solve_mixed_column(
        &layers_nir,
        RadiationBand::NearInfrared,
        IncidentComponent::Direct,
        forcing.solar_zenith_cosine,
        forcing.ground_albedo_nir,
        forcing.direct_nir_w_m2,
    )?;
    let near_infrared_diffuse = solve_mixed_column(
        &layers_nir,
        RadiationBand::NearInfrared,
        IncidentComponent::Diffuse,
        forcing.solar_zenith_cosine,
        forcing.ground_albedo_nir,
        forcing.diffuse_nir_w_m2,
    )?;
    let forcing_by_layer = forcing
        .soil_layers
        .iter()
        .map(|value| (&value.layer_id, value))
        .collect::<BTreeMap<_, _>>();
    let mut occupancies = Vec::new();
    for ((identity, stratum), (lai, sai)) in identities.iter().zip(strata).zip(areas) {
        let occupancy_state = state
            .occupancies
            .get(identity)
            .ok_or(V8InputProjectionError::Topology("missing occupancy state"))?;
        let shared_state = &state.strata[&identity.stratum_id];
        let mut roots = Vec::new();
        for root in &stratum.root_layers {
            let source = RealHydrologySourceKey {
                ofe_lane: crate::vegetation_real_hydrology_shadow::RealHydrologyOfeLaneId {
                    lane_index,
                    lane_id,
                },
                layer_id: root.layer_id.clone(),
            };
            let fact = soil_adapter.owner.layer_facts().get(&source).ok_or(
                V8InputProjectionError::Topology("missing real hydrology layer"),
            )?;
            let layer_forcing =
                forcing_by_layer
                    .get(&root.layer_id)
                    .ok_or(V8InputProjectionError::Topology(
                        "missing hydraulic forcing layer",
                    ))?;
            if layer_forcing.water_beginning_kg_m2.to_bits() != fact.liquid_supply_kg_m2.to_bits()
                || layer_forcing.frozen != fact.frozen
            {
                return Err(V8InputProjectionError::Identity(
                    "hydraulic owner operand join",
                ));
            }
            roots.push(V8ProjectedRootLayer {
                forcing: (*layer_forcing).clone(),
                source: source.clone(),
                fact: fact.clone(),
            });
        }
        occupancies.push(V8ProjectedOccupancyInput {
            occupancy_id: identity.clone(),
            vertical_rank: stratum.vertical_rank,
            conditional_lai_m2_m2_tile_ground: lai,
            conditional_sai_m2_m2_tile_ground: sai,
            stratum_configuration: stratum.clone(),
            shared_state: shared_state.clone(),
            occupancy_state: occupancy_state.clone(),
            root_layers: roots,
        });
    }
    Ok((
        V8ProjectedColumnRadiation {
            visible_direct,
            visible_diffuse,
            near_infrared_direct,
            near_infrared_diffuse,
        },
        occupancies,
    ))
}

fn mixed_layer(stratum: &StratumConfiguration, lai: f64, sai: f64, visible: bool) -> MixedLayer {
    let (leaf_rho, leaf_tau, stem_rho, stem_tau) = if visible {
        (
            stratum.leaf_rho_vis,
            stratum.leaf_tau_vis,
            stratum.stem_rho_vis,
            stratum.stem_tau_vis,
        )
    } else {
        (
            stratum.leaf_rho_nir,
            stratum.leaf_tau_nir,
            stratum.stem_rho_nir,
            stratum.stem_tau_nir,
        )
    };
    MixedLayer {
        leaf_area: lai,
        stem_area: sai,
        clumping_index: stratum.clumping_index,
        leaf_angle_chi: stratum.leaf_angle_chi,
        leaf_optics: SurfaceOptics {
            reflectance: leaf_rho,
            transmittance: leaf_tau,
        },
        stem_optics: SurfaceOptics {
            reflectance: stem_rho,
            transmittance: stem_tau,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vegetation_forcing() -> SnowFreeForcing {
        SnowFreeForcing {
            air_temperature_k: 296.0,
            pressure_pa: 101_325.0,
            co2_pa: 42.0,
            vapor_pressure_deficit_kpa: 1.0,
            wind_m_s: 3.7,
            rain_kg_m2: 0.0,
            direct_par_w_m2: 410.0,
            diffuse_par_w_m2: 83.0,
            direct_nir_w_m2: 355.0,
            diffuse_nir_w_m2: 101.0,
            solar_zenith_cosine: 0.67,
            ground_albedo_vis: 0.12,
            ground_albedo_nir: 0.24,
            longwave_down_w_m2: 395.0,
            longwave_up_w_m2: 0.0,
            specific_humidity: 0.0102,
            reference_height_m: 24.0,
            soil_layers: Vec::new(),
            gsi: 1.0,
        }
    }

    fn lse_forcing() -> LandSurfaceForcing {
        LandSurfaceForcing {
            forcing_sha256: Sha256Digest::try_new("a".repeat(64)).expect("digest"),
            transaction_id: TransactionId(1),
            interval_s: 1_800.0,
            air_temperature_k: 296.0,
            air_specific_humidity_kg_kg: 0.0102,
            air_pressure_pa: 101_325.0,
            reference_wind_m_s: 3.7,
            neutral_stability: true,
            snow_present_at_beginning: false,
            snow_present_at_end: false,
            snow_terminal_payload_present: false,
            direct_vis_w_m2: 410.0,
            diffuse_vis_w_m2: 83.0,
            direct_nir_w_m2: 355.0,
            diffuse_nir_w_m2: 101.0,
            atmospheric_downward_longwave_w_m2: 395.0,
            precipitation_parcels: Vec::new(),
            runon_parcels: Vec::new(),
        }
    }

    fn configuration() -> VegetationConfiguration {
        VegetationConfiguration {
            model_definition_sha256: V8_MODEL_SHA256.into(),
            configuration_sha256: "b".repeat(64),
            initial_state_sha256: "c".repeat(64),
            area_m2: 1.0,
            timestamp: "2026-08-15T00:00:00Z".into(),
            dt_s: 1_800.0,
            topology_tiles: Vec::new(),
            strata: Vec::new(),
        }
    }

    #[test]
    fn forcing_families_are_bit_exact_and_independent() {
        let configuration = configuration();
        let baseline = vegetation_forcing();
        let lse = lse_forcing();
        assert!(validate_forcing_join(&configuration, &baseline, &lse).is_ok());
        for mutate in 0..10 {
            let mut poison = baseline.clone();
            let field = match mutate {
                0 => &mut poison.air_temperature_k,
                1 => &mut poison.pressure_pa,
                2 => &mut poison.wind_m_s,
                3 => &mut poison.specific_humidity,
                4 => &mut poison.direct_par_w_m2,
                5 => &mut poison.diffuse_par_w_m2,
                6 => &mut poison.direct_nir_w_m2,
                7 => &mut poison.diffuse_nir_w_m2,
                8 => &mut poison.longwave_down_w_m2,
                _ => {
                    let mut wrong_configuration = configuration.clone();
                    wrong_configuration.dt_s = f64::from_bits(configuration.dt_s.to_bits() + 1);
                    assert!(validate_forcing_join(&wrong_configuration, &baseline, &lse).is_err());
                    continue;
                }
            };
            *field = f64::from_bits(field.to_bits() + 1);
            assert!(validate_forcing_join(&configuration, &poison, &lse).is_err());
        }
    }

    #[test]
    fn canopy_receipt_digest_detects_one_bit_in_each_forcing_family() {
        let digest = || Sha256Digest::try_new("a".repeat(64)).expect("digest");
        let receipt = V8CanopyForcingReceipt::try_new(
            "b".repeat(64),
            "c".repeat(64),
            digest(),
            digest(),
            digest(),
            digest(),
            TransactionId(1),
            vegetation_forcing(),
        )
        .expect("receipt");
        assert!(receipt.validate_digest().is_ok());
        for mutate in 0..4 {
            let mut poison = receipt.clone();
            let field = match mutate {
                0 => &mut poison.forcing.co2_pa,
                1 => &mut poison.forcing.solar_zenith_cosine,
                2 => &mut poison.forcing.vapor_pressure_deficit_kpa,
                _ => &mut poison.forcing.reference_height_m,
            };
            *field = f64::from_bits(field.to_bits() + 1);
            assert!(poison.validate_digest().is_err());
        }
    }
}
