//! Strict, solve-free projection of accepted V8/LSE/hydrology owner inputs.
//!
//! This boundary deliberately stops before maximum-demand evaluation, water
//! arbitration, or a candidate solve. In particular, Emax is not a caller
//! input: it is produced later by the admitted coupled solver.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, ResourceOwnerId, TileId, TransactionId};
use openwepp_land_surface_energy::UnderCanopyGeometry;
use openwepp_land_surface_energy::{
    BandDirectionalFluxes, BiochemicalConstants, ComponentId, CoveredColumnInputs,
    CoveredColumnShortwaveInputs, CoveredOccupancyInputs, CoveredOccupancyShortwaveInputs,
    ExactDyadicEnthalpy, LandSurfaceEnergyConfiguration, LandSurfaceEnergyError,
    LandSurfaceEnergyState, LandSurfaceForcing, LeafBiochemicalInputs, OfeId, OpenNeutralGeometry,
    OpenSurfaceProblem, PreparedSoilThermalSupportV2, RequestingComponent, RootHydraulicLayer,
    RootRuntimeIdentity, RuntimeTileIdentity, SoilInterfaceLayer, SoilThermalNodeOperands,
    SoilThermalSnapshot, SourceId, Stage3SnowCoveredLowerBoundary,
    Stage3SnowOpticalBoundaryReceiptInputs, Stage3SnowOpticalBoundaryReceiptV1,
    StandGroundWaterAmountBasis, SurfaceClassKind, SurfaceConfiguration, SurfaceHeatStorageMode,
    SurfaceStorageBranch, TileConfiguration, TileState, TurbulenceConfiguration, WaterSourceType,
};
use openwepp_vegetation::carbon_nitrogen::{Tissue, atkin_rd25, update_t10};
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
    V8TileCanopyAirState, VegetationConfiguration, VegetationError, validate_v8_component_bindings,
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

const ROOT_ZONE_OWNER_ID: &str = "root-zone-hydraulic-owner-v1";
const ROOT_ZONE_MODEL_SHA256: &str =
    "65c90f388ef939aa84e6d53919411d389a8612c57de647c47b8a538ad1ba60e4";

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
    #[error("root-zone water above pore capacity")]
    WaterAbovePoreCapacity,
    #[error("rooted frozen layer is unsupported")]
    FrozenRootedLayerUnsupported,
    #[error("rooted inaccessible layer is unsupported")]
    InaccessibleRootedLayer,
    #[error("root-zone owner join failure: {0}")]
    RootOwnerJoin(&'static str),
    #[error("root-zone receipt digest failure")]
    RootReceiptDigest,
    #[error("root-zone scalar domain failure")]
    RootDomain,
}

/// Authenticated, immutable soil-thermal beginning used by one provisional
/// physical evaluation. The V2 arm is a native snapshot of the prepared
/// support and is never projected through the frozen V1 owner schema.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum V8SoilThermalPhysicalBeginning {
    V1(SoilThermalSnapshot),
    V2(Box<PreparedSoilThermalSupportV2>),
}

impl V8SoilThermalPhysicalBeginning {
    pub(crate) fn try_from_v1(
        beginning: &SoilThermalSnapshot,
    ) -> Result<Self, V8InputProjectionError> {
        beginning.validate()?;
        Ok(Self::V1(beginning.clone()))
    }

    pub(crate) fn try_from_v2(
        prepared: &PreparedSoilThermalSupportV2,
    ) -> Result<Self, V8InputProjectionError> {
        prepared
            .beginning_owner()
            .validate()
            .map_err(|_| V8InputProjectionError::Identity("prepared V2 soil beginning"))?;
        Ok(Self::V2(Box::new(prepared.clone())))
    }

    pub(crate) fn validate(&self) -> Result<(), V8InputProjectionError> {
        match self {
            Self::V1(beginning) => beginning.validate().map_err(Into::into),
            Self::V2(beginning) => beginning
                .beginning_owner()
                .state
                .validate()
                .map_err(|_| V8InputProjectionError::Identity("native V2 soil beginning")),
        }
    }

    pub(crate) fn owner_id(&self) -> &ResourceOwnerId {
        match self {
            Self::V1(beginning) => &beginning.owner_id,
            Self::V2(beginning) => &beginning.beginning_owner().state.owner_id,
        }
    }

    pub(crate) fn configuration_sha256(&self) -> &Sha256Digest {
        match self {
            Self::V1(beginning) => &beginning.configuration_sha256,
            Self::V2(beginning) => &beginning.beginning_owner().state.configuration_sha256,
        }
    }

    pub(crate) fn state_sha256(&self) -> &Sha256Digest {
        match self {
            Self::V1(beginning) => &beginning.state_sha256,
            Self::V2(beginning) => &beginning.beginning_owner().state.state_sha256,
        }
    }

    pub(crate) fn snapshot_sha256(&self) -> Result<Sha256Digest, V8InputProjectionError> {
        match self {
            Self::V1(beginning) => Ok(beginning.snapshot_sha256.clone()),
            Self::V2(beginning) => beginning
                .beginning_owner()
                .snapshot()
                .map(|snapshot| snapshot.snapshot_sha256)
                .map_err(|_| V8InputProjectionError::Identity("native V2 soil snapshot digest")),
        }
    }

    pub(crate) fn finalization_beginning(
        &self,
    ) -> openwepp_land_surface_energy::SoilThermalFinalizationBeginning<'_> {
        match self {
            Self::V1(beginning) => {
                openwepp_land_surface_energy::SoilThermalFinalizationBeginning::V1(beginning)
            }
            Self::V2(beginning) => {
                openwepp_land_surface_energy::SoilThermalFinalizationBeginning::V2(
                    beginning.physical_read_view(),
                )
            }
        }
    }

    pub(crate) fn ordered_ofes(&self) -> Vec<V8ProjectedSoilThermalOfe> {
        match self {
            Self::V1(beginning) => beginning
                .ofes
                .iter()
                .map(|ofe| V8ProjectedSoilThermalOfe {
                    ofe_id: ofe.ofe_id.clone(),
                    ordered_layers: ofe
                        .ordered_layers
                        .iter()
                        .map(|layer| V8ProjectedSoilThermalLayer {
                            layer_id: layer.layer_id.clone(),
                            temperature_k: layer.temperature_k,
                            enthalpy_hi_j_m2_ofe_ground: layer.enthalpy_j_m2_ofe_ground,
                            enthalpy_carry: ExactDyadicEnthalpy::zero(),
                        })
                        .collect(),
                })
                .collect(),
            Self::V2(beginning) => beginning
                .beginning_owner()
                .state
                .ofes
                .iter()
                .map(|ofe| V8ProjectedSoilThermalOfe {
                    ofe_id: ofe.ofe_id.clone(),
                    ordered_layers: ofe
                        .ordered_layers
                        .iter()
                        .map(|layer| V8ProjectedSoilThermalLayer {
                            layer_id: layer.layer_id.clone(),
                            temperature_k: layer.temperature_k,
                            enthalpy_hi_j_m2_ofe_ground: layer.enthalpy_hi_j_m2_ofe_ground,
                            enthalpy_carry: layer.enthalpy_carry.clone(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V8ProjectedSoilThermalOfe {
    pub(crate) ofe_id: OfeId,
    pub(crate) ordered_layers: Vec<V8ProjectedSoilThermalLayer>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V8ProjectedSoilThermalLayer {
    pub(crate) layer_id: openwepp_kernel_contract::SoilLayerId,
    pub(crate) temperature_k: f64,
    pub(crate) enthalpy_hi_j_m2_ofe_ground: f64,
    pub(crate) enthalpy_carry: ExactDyadicEnthalpy,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct V10RootZoneReceiptKey {
    pub ofe_id: OfeId,
    pub production_lane_index: usize,
    pub production_lane_id: u32,
    pub occupancy_id: OccupancyId,
    pub stratum_id: openwepp_kernel_contract::StratumId,
    pub layer_id: openwepp_kernel_contract::SoilLayerId,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V10RootZoneLayerReceipt {
    pub key: V10RootZoneReceiptKey,
    pub matric_potential_mm: f64,
    pub hydraulic_conductivity_mm_s: f64,
    pub root_path_length_mm: f64,
    pub gravity_root_mm: f64,
    pub lateral_root_length_m: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V10RootZoneReceiptSet {
    owner_id: String,
    model_definition_sha256: String,
    root_configuration_sha256: String,
    hydrology_configuration_sha256: Sha256Digest,
    vegetation_configuration_sha256: String,
    vegetation_root_bindings_sha256: String,
    lse_configuration_sha256: Sha256Digest,
    hydrology_snapshot_sha256: Sha256Digest,
    transaction_id: TransactionId,
    day_index: usize,
    interval_index: u8,
    receipts: BTreeMap<V10RootZoneReceiptKey, V10RootZoneLayerReceipt>,
    receipt_sha256: String,
}

impl V10RootZoneReceiptSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn try_new(
        root_configuration_sha256: String,
        hydrology_configuration_sha256: Sha256Digest,
        vegetation_configuration_sha256: String,
        lse_configuration_sha256: Sha256Digest,
        hydrology_snapshot_sha256: Sha256Digest,
        transaction_id: TransactionId,
        day_index: usize,
        interval_index: u8,
        receipts: Vec<V10RootZoneLayerReceipt>,
    ) -> Result<Self, V8InputProjectionError> {
        let mut indexed = BTreeMap::new();
        for receipt in receipts {
            if indexed.insert(receipt.key.clone(), receipt).is_some() {
                return Err(V8InputProjectionError::RootOwnerJoin(
                    "duplicate OFE/lane/occupancy/layer receipt",
                ));
            }
        }
        let vegetation_root_bindings_sha256 = root_binding_digest(&indexed);
        let mut value = Self {
            owner_id: ROOT_ZONE_OWNER_ID.into(),
            model_definition_sha256: ROOT_ZONE_MODEL_SHA256.into(),
            root_configuration_sha256,
            hydrology_configuration_sha256,
            vegetation_configuration_sha256,
            vegetation_root_bindings_sha256,
            lse_configuration_sha256,
            hydrology_snapshot_sha256,
            transaction_id,
            day_index,
            interval_index,
            receipts: indexed,
            receipt_sha256: String::new(),
        };
        value.receipt_sha256 = value.compute_digest();
        Ok(value)
    }

    fn compute_digest(&self) -> String {
        let mut digest = Sha256::new();
        for value in [
            self.owner_id.as_bytes(),
            self.model_definition_sha256.as_bytes(),
            self.root_configuration_sha256.as_bytes(),
            self.hydrology_configuration_sha256.as_str().as_bytes(),
            self.vegetation_configuration_sha256.as_bytes(),
            self.vegetation_root_bindings_sha256.as_bytes(),
            self.lse_configuration_sha256.as_str().as_bytes(),
            self.hydrology_snapshot_sha256.as_str().as_bytes(),
        ] {
            digest.update((value.len() as u64).to_le_bytes());
            digest.update(value);
        }
        digest.update(self.transaction_id.0.to_le_bytes());
        digest.update((self.day_index as u64).to_le_bytes());
        digest.update([self.interval_index]);
        digest.update((self.receipts.len() as u64).to_le_bytes());
        for (key, receipt) in &self.receipts {
            for value in [
                key.ofe_id.as_str(),
                key.occupancy_id.tile_id.as_str(),
                key.occupancy_id.stratum_id.as_str(),
                key.stratum_id.as_str(),
                key.layer_id.as_str(),
            ] {
                digest.update((value.len() as u64).to_le_bytes());
                digest.update(value.as_bytes());
            }
            digest.update((key.production_lane_index as u64).to_le_bytes());
            digest.update(key.production_lane_id.to_le_bytes());
            for value in [
                receipt.matric_potential_mm,
                receipt.hydraulic_conductivity_mm_s,
                receipt.root_path_length_mm,
                receipt.gravity_root_mm,
                receipt.lateral_root_length_m,
            ] {
                digest.update(value.to_bits().to_le_bytes());
            }
        }
        format!("{:x}", digest.finalize())
    }

    fn get(
        &self,
        key: &V10RootZoneReceiptKey,
    ) -> Result<&V10RootZoneLayerReceipt, V8InputProjectionError> {
        if self.receipt_sha256 != self.compute_digest() {
            return Err(V8InputProjectionError::RootReceiptDigest);
        }
        self.receipts
            .get(key)
            .ok_or(V8InputProjectionError::RootOwnerJoin(
                "missing qualified receipt",
            ))
    }
}

fn root_binding_digest(
    receipts: &BTreeMap<V10RootZoneReceiptKey, V10RootZoneLayerReceipt>,
) -> String {
    let mut digest = Sha256::new();
    for (key, receipt) in receipts {
        for value in [
            key.ofe_id.as_str(),
            key.occupancy_id.tile_id.as_str(),
            key.occupancy_id.stratum_id.as_str(),
            key.stratum_id.as_str(),
            key.layer_id.as_str(),
        ] {
            digest.update((value.len() as u64).to_le_bytes());
            digest.update(value.as_bytes());
        }
        digest.update((key.production_lane_index as u64).to_le_bytes());
        digest.update(key.production_lane_id.to_le_bytes());
        digest.update(receipt.lateral_root_length_m.to_bits().to_le_bytes());
        digest.update([1]);
    }
    format!("{:x}", digest.finalize())
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
    root_zone_hydraulics: Option<V10RootZoneReceiptSet>,
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
            root_zone_hydraulics: None,
        };
        value.forcing_sha256 = value.canonical_sha256();
        value.validate_digest()?;
        Ok(value)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn try_new_with_root_zone(
        vegetation_configuration_sha256: String,
        vegetation_beginning_state_sha256: String,
        lse_configuration_sha256: Sha256Digest,
        lse_forcing_sha256: Sha256Digest,
        hydrology_snapshot_sha256: Sha256Digest,
        soil_thermal_snapshot_sha256: Sha256Digest,
        transaction_id: TransactionId,
        forcing: SnowFreeForcing,
        root_zone_hydraulics: V10RootZoneReceiptSet,
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
            root_zone_hydraulics: Some(root_zone_hydraulics),
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
        match &self.root_zone_hydraulics {
            None => digest.update([0]),
            Some(receipts) => {
                digest.update([1]);
                digest.update(receipts.receipt_sha256.as_bytes());
            }
        }
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
pub(crate) struct V8ProjectedColumnRadiation {
    pub visible_direct: ColumnRadiationResult,
    pub visible_diffuse: ColumnRadiationResult,
    pub near_infrared_direct: ColumnRadiationResult,
    pub near_infrared_diffuse: ColumnRadiationResult,
    pub stage3_optical: Option<Stage3SnowOpticalBoundaryReceiptV1>,
}

/// One real production layer joined to the V8 hydraulic forcing lane.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V8ProjectedRootLayer {
    pub forcing: SoilLayerForcing,
    pub source: RealHydrologySourceKey,
    pub fact: RealHydrologyLayerFact,
}

/// Immutable source operands for one ordered V8 occupancy.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V8ProjectedOccupancyInput {
    pub(crate) ofe_id: OfeId,
    pub(crate) production_lane_index: usize,
    pub(crate) production_lane_id: u32,
    pub(crate) occupancy_id: OccupancyId,
    pub(crate) vertical_rank: u32,
    pub(crate) conditional_lai_m2_m2_tile_ground: f64,
    pub(crate) conditional_sai_m2_m2_tile_ground: f64,
    pub(crate) stratum_configuration: StratumConfiguration,
    pub(crate) shared_state: StratumSharedState,
    pub(crate) occupancy_state: V8OccupancyState,
    pub(crate) root_layers: Vec<V8ProjectedRootLayer>,
}

/// Exact ground owner inputs for one LSE tile.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V8ProjectedGroundInput {
    pub(crate) configuration: TileConfiguration,
    pub(crate) soil_interface_layers: Vec<SoilInterfaceLayer>,
    pub(crate) state: TileState,
    pub(crate) surface_liquid: DirectSurfaceLiquidStateRecord,
    pub(crate) soil_thermal: V8ProjectedSoilThermalOfe,
    pub(crate) top_hydrology: RealHydrologyLayerFact,
}

/// Fully projected, solve-free runtime input for one OFE/tile.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V8ProjectedTileRuntimeInput {
    pub(crate) identity: RuntimeTileIdentity,
    pub(crate) ofe_id: OfeId,
    pub(crate) tile_id: TileId,
    pub(crate) transaction_id: TransactionId,
    pub(crate) interval_s: f64,
    pub(crate) tile_fraction: f64,
    pub(crate) forcing: LandSurfaceForcing,
    pub(crate) vegetation_forcing: SnowFreeForcing,
    pub(crate) root_zone_hydraulics: Option<V10RootZoneReceiptSet>,
    pub(crate) canopy_air_state: Option<V8TileCanopyAirState>,
    pub(crate) radiation: V8ProjectedColumnRadiation,
    pub(crate) ground: V8ProjectedGroundInput,
    pub(crate) occupancies: Vec<V8ProjectedOccupancyInput>,
}

/// Cross-owner lineage plus every per-tile input. This type cannot solve or
/// commit and contains no caller-provided numerical trial or Emax value.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ValidatedV8RuntimeInputProjection {
    pub(crate) vegetation_configuration_sha256: String,
    pub(crate) vegetation_state_sha256: String,
    pub(crate) lse_configuration_sha256: Sha256Digest,
    pub(crate) lse_state_sha256: Sha256Digest,
    pub(crate) lse_forcing_sha256: Sha256Digest,
    pub(crate) hydrology_snapshot_sha256: Sha256Digest,
    pub(crate) soil_thermal_snapshot_sha256: Sha256Digest,
    pub(crate) soil_thermal_beginning: V8SoilThermalPhysicalBeginning,
    pub(crate) transaction_id: TransactionId,
    pub(crate) tiles: Vec<V8ProjectedTileRuntimeInput>,
}

#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum V8SolverReadyTilePhysics {
    Open(OpenSurfaceProblem),
    /// The open tile's snow surface is owned by Stage 3. LSE retains the tile
    /// only so the complete configured topology and unchanged owner state are
    /// carried through the heterogeneous transaction.
    Stage3OpenSnow(TileState),
    Covered(CoveredColumnInputs),
}

/// Opaque solver-ready tile. Every physical operand and numerical warm start
/// is derived from the validated projection; consumers supply no raw arrays.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V8SolverReadyTileInput {
    pub(crate) identity: RuntimeTileIdentity,
    pub(crate) physics: V8SolverReadyTilePhysics,
    pub(crate) root_identities: Vec<RootRuntimeIdentity>,
    pub(crate) soil_sources:
        BTreeMap<openwepp_land_surface_energy::GroundWaterKey, RealHydrologySourceKey>,
    pub(crate) beginning_trial: Vec<f64>,
    pub(crate) vegetation_bindings: Vec<V8ComponentOccupancyBinding>,
    pub(crate) soil_thermal: V8SoilThermalPhysicalBeginning,
}

impl ValidatedV8RuntimeInputProjection {
    #[cfg(test)]
    pub(crate) fn solver_ready_tiles(
        &self,
        vegetation_owner_id: &ResourceOwnerId,
    ) -> Result<Vec<V8SolverReadyTileInput>, V8InputProjectionError> {
        self.solver_ready_tiles_with_authority_and_lower_boundaries(
            vegetation_owner_id,
            openwepp_land_surface_energy::CoveredColumnAuthority::HistoricalV8,
            None,
        )
    }

    pub(crate) fn solver_ready_tiles_with_authority_and_lower_boundaries(
        &self,
        vegetation_owner_id: &ResourceOwnerId,
        authority: openwepp_land_surface_energy::CoveredColumnAuthority,
        lower_boundaries: Option<&BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>>,
    ) -> Result<Vec<V8SolverReadyTileInput>, V8InputProjectionError> {
        self.tiles
            .iter()
            .map(|tile| {
                let mut ready =
                    tile.solver_ready(vegetation_owner_id, &self.soil_thermal_beginning)?;
                if authority == openwepp_land_surface_energy::CoveredColumnAuthority::V11SnowCovered
                    && matches!(ready.physics, V8SolverReadyTilePhysics::Open(_))
                {
                    let boundaries = lower_boundaries.ok_or(V8InputProjectionError::Identity(
                        "missing Stage-3 lower-boundary set",
                    ))?;
                    if boundaries.contains_key(&(tile.ofe_id.clone(), tile.tile_id.clone())) {
                        ready.physics =
                            V8SolverReadyTilePhysics::Stage3OpenSnow(tile.ground.state.clone());
                    }
                }
                if let V8SolverReadyTilePhysics::Covered(column) = &mut ready.physics {
                    if authority
                        == openwepp_land_surface_energy::CoveredColumnAuthority::V11SnowCovered
                    {
                        let boundaries = lower_boundaries.ok_or(
                            V8InputProjectionError::Identity("missing Stage-3 lower-boundary set"),
                        )?;
                        if let Some(boundary) =
                            boundaries.get(&(tile.ofe_id.clone(), tile.tile_id.clone()))
                        {
                            column.authority = authority;
                            column.stage3_lower_boundary = Some(boundary.clone());
                            // The typed tile may have been projected with a
                            // different provisional Stage-3 boundary. Re-seal
                            // optical custody from this tile's exact current
                            // two-stream results and the boundary being bound;
                            // retaining the earlier optical receipt mixes two
                            // carrier iterations and breaks directional ground
                            // absorption closure.
                            column.stage3_optical = Some(stage3_optical_receipt(
                                &tile.ofe_id,
                                &tile.tile_id,
                                &tile.radiation.visible_direct,
                                &tile.radiation.visible_diffuse,
                                &tile.radiation.near_infrared_direct,
                                &tile.radiation.near_infrared_diffuse,
                                boundary,
                            )?);
                        }
                    } else {
                        column.authority = authority;
                    }
                }
                Ok(ready)
            })
            .collect()
    }
}

impl V8ProjectedTileRuntimeInput {
    fn solver_ready(
        &self,
        vegetation_owner_id: &ResourceOwnerId,
        soil_thermal_beginning: &V8SoilThermalPhysicalBeginning,
    ) -> Result<V8SolverReadyTileInput, V8InputProjectionError> {
        self.validate_preflight()?;
        let ground = self.ground_problem()?;
        if self.occupancies.is_empty() {
            return Ok(V8SolverReadyTileInput {
                identity: self.identity.clone(),
                beginning_trial: open_beginning_trial(&ground),
                physics: V8SolverReadyTilePhysics::Open(ground),
                root_identities: Vec::new(),
                soil_sources: BTreeMap::new(),
                vegetation_bindings: Vec::new(),
                soil_thermal: soil_thermal_beginning.clone(),
            });
        }
        let (column, roots, sources, bindings, trial) =
            self.covered_problem(vegetation_owner_id, ground)?;
        Ok(V8SolverReadyTileInput {
            identity: self.identity.clone(),
            physics: V8SolverReadyTilePhysics::Covered(column),
            root_identities: roots,
            soil_sources: sources,
            beginning_trial: trial,
            vegetation_bindings: bindings,
            soil_thermal: soil_thermal_beginning.clone(),
        })
    }

    fn validate_preflight(&self) -> Result<(), V8InputProjectionError> {
        if self.identity.ofe_id != self.ofe_id
            || self.identity.tile_id != self.tile_id
            || self.identity.transaction_id != self.transaction_id
            || self.identity.interval_s.to_bits() != self.interval_s.to_bits()
            || self.identity.tile_fraction.to_bits() != self.tile_fraction.to_bits()
            || self.ground.configuration.tile_id != self.tile_id
            || self.ground.state.ofe_id != self.ofe_id
            || self.ground.state.tile_id != self.tile_id
            || self.ground.soil_thermal.ofe_id != self.ofe_id
        {
            return Err(V8InputProjectionError::Identity("projected tile identity"));
        }
        if self.ground.soil_interface_layers.len() != self.ground.soil_thermal.ordered_layers.len()
            || self
                .ground
                .soil_interface_layers
                .iter()
                .zip(&self.ground.soil_thermal.ordered_layers)
                .any(|(configured, thermal)| configured.layer_id != thermal.layer_id)
        {
            return Err(V8InputProjectionError::Topology(
                "soil-thermal configured ordering",
            ));
        }
        for (index, occupancy) in self.occupancies.iter().enumerate() {
            if occupancy.occupancy_id.tile_id != self.ground.configuration.vegetation_tile_id
                || occupancy.occupancy_id.stratum_id != occupancy.stratum_configuration.stratum_id
                || occupancy.vertical_rank != occupancy.stratum_configuration.vertical_rank
                || index > 0 && self.occupancies[index - 1].vertical_rank >= occupancy.vertical_rank
            {
                return Err(V8InputProjectionError::Topology(
                    "configured occupancy vertical ordering",
                ));
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn ground_problem(&self) -> Result<OpenSurfaceProblem, V8InputProjectionError> {
        let configuration = &self.ground.configuration;
        let terminal_shortwave_w_m2_tile = BandDirectionalFluxes {
            direct_vis: self.radiation.visible_direct.terminal_direct
                + self.radiation.visible_direct.terminal_diffuse,
            diffuse_vis: self.radiation.visible_diffuse.terminal_direct
                + self.radiation.visible_diffuse.terminal_diffuse,
            direct_nir: self.radiation.near_infrared_direct.terminal_direct
                + self.radiation.near_infrared_direct.terminal_diffuse,
            diffuse_nir: self.radiation.near_infrared_diffuse.terminal_direct
                + self.radiation.near_infrared_diffuse.terminal_diffuse,
        };
        let soil_nodes = self
            .ground
            .soil_interface_layers
            .iter()
            .zip(&self.ground.soil_thermal.ordered_layers)
            .map(|(layer, thermal)| SoilThermalNodeOperands {
                layer_id: layer.layer_id.as_str().into(),
                depth_m: layer.thickness_m,
                conductivity_w_m_k: layer.thermal_conductivity_w_m_k,
                heat_capacity_j_m2_k: layer.areal_heat_capacity_j_m2_k,
                beginning_temperature_k: thermal.temperature_k,
            })
            .collect();
        let (class, emissivity, depth, conductivity, heat_capacity, capacity, bare) =
            match &configuration.surface {
                SurfaceConfiguration::BareMineralSoil {
                    dry_areal_heat_capacity_j_m2_k,
                    mineral_skin_thickness_m,
                    mineral_skin_thermal_conductivity_w_m_k,
                    top_layer_porosity_m3_m3,
                    top_layer_saturated_matric_potential_mm,
                    top_layer_clapp_hornberger_b,
                    top_layer_initial_water_content_m3_m3,
                    ..
                } => {
                    // Bare-soil water is owned by the exact OFE hydrology
                    // snapshot. The vegetation forcing layer is a root uptake
                    // surface and cannot represent distinct same-named layers
                    // on multiple OFEs.
                    let water = self.ground.top_hydrology.liquid_supply_kg_m2;
                    let frozen = self.ground.top_hydrology.frozen;
                    let (liquid, ice) = if frozen { (0.0, water) } else { (water, 0.0) };
                    (
                        SurfaceClassKind::BareMineralSoil,
                        1.0,
                        *mineral_skin_thickness_m,
                        *mineral_skin_thermal_conductivity_w_m_k,
                        *dry_areal_heat_capacity_j_m2_k,
                        None,
                        Some(openwepp_land_surface_energy::BareSoilParameters {
                            top_layer_liquid_kg_m2: liquid,
                            top_layer_ice_kg_m2: ice,
                            porosity: *top_layer_porosity_m3_m3,
                            saturated_matric_potential_mm: *top_layer_saturated_matric_potential_mm,
                            clapp_hornberger_b: *top_layer_clapp_hornberger_b,
                            theta_initial: *top_layer_initial_water_content_m3_m3,
                        }),
                    )
                }
                SurfaceConfiguration::ForestLitter {
                    liquid_capacity_kg_m2_tile_ground,
                    thickness_m,
                    dry_density_kg_m3,
                    dry_specific_heat_j_kg_k,
                } => (
                    SurfaceClassKind::ForestLitter,
                    1.0,
                    *thickness_m,
                    project_forest_litter_conductivity(
                        self.ground.surface_liquid.liquid_kg_m2_tile,
                        *thickness_m,
                    )?,
                    thickness_m * dry_density_kg_m3 * dry_specific_heat_j_kg_k,
                    Some(*liquid_capacity_kg_m2_tile_ground),
                    None,
                ),
            };
        let open_geometry = match &configuration.turbulence {
            TurbulenceConfiguration::OpenNeutral {
                reference_height_m,
                roughness_momentum_m,
                roughness_heat_m,
                roughness_vapor_m,
            } => OpenNeutralGeometry {
                reference_height_m: *reference_height_m,
                roughness_momentum_m: *roughness_momentum_m,
                roughness_heat_m: *roughness_heat_m,
                roughness_vapor_m: *roughness_vapor_m,
            },
            TurbulenceConfiguration::CoveredNeutral {
                ground_exchange_roughness_m,
                canopy_to_reference,
                ..
            } => OpenNeutralGeometry {
                reference_height_m: canopy_to_reference.reference_height_m,
                roughness_momentum_m: *ground_exchange_roughness_m,
                roughness_heat_m: *ground_exchange_roughness_m,
                roughness_vapor_m: *ground_exchange_roughness_m,
            },
        };
        Ok(OpenSurfaceProblem {
            interval_s: self.interval_s,
            tile_fraction: self.tile_fraction,
            class,
            storage_branch: match configuration.surface_heat_storage_mode {
                SurfaceHeatStorageMode::FiniteCapacity => SurfaceStorageBranch::FiniteCapacity,
                SurfaceHeatStorageMode::EquilibriumZero => SurfaceStorageBranch::EquilibriumZero,
            },
            terminal_shortwave_w_m2_tile,
            surface_vis_albedo: configuration.surface_vis_albedo,
            surface_nir_albedo: configuration.surface_nir_albedo,
            surface_emissivity: emissivity,
            surface_depth_m: depth,
            surface_conductivity_w_m_k: conductivity,
            surface_dry_heat_capacity_j_m2_k: heat_capacity,
            litter_capacity_kg_m2_tile: capacity,
            open_geometry,
            air_temperature_k: self.forcing.air_temperature_k,
            air_specific_humidity_kg_kg: self.forcing.air_specific_humidity_kg_kg,
            air_pressure_pa: self.forcing.air_pressure_pa,
            reference_wind_m_s: self.forcing.reference_wind_m_s,
            atmospheric_downward_longwave_w_m2: self.forcing.atmospheric_downward_longwave_w_m2,
            surface_liquid_kg_m2_tile: self.ground.surface_liquid.liquid_kg_m2_tile,
            surface_enthalpy_j_m2_tile: self.ground.state.surface_enthalpy_j_m2_tile_ground,
            surface_temperature_warm_start_k: self.ground.state.surface_temperature_warm_start_k,
            bare_soil: bare,
            soil_nodes,
        })
    }

    #[allow(clippy::type_complexity, clippy::too_many_lines)]
    fn covered_problem(
        &self,
        vegetation_owner_id: &ResourceOwnerId,
        ground: OpenSurfaceProblem,
    ) -> Result<
        (
            CoveredColumnInputs,
            Vec<RootRuntimeIdentity>,
            BTreeMap<openwepp_land_surface_energy::GroundWaterKey, RealHydrologySourceKey>,
            Vec<V8ComponentOccupancyBinding>,
            Vec<f64>,
        ),
        V8InputProjectionError,
    > {
        let canopy = self
            .canopy_air_state
            .as_ref()
            .ok_or(V8InputProjectionError::Topology(
                "missing covered canopy-air state",
            ))?;
        let mut rows = Vec::new();
        let mut sw_rows = Vec::new();
        let mut roots = Vec::new();
        let mut sources = BTreeMap::new();
        let mut bindings = Vec::new();
        let mut trial = Vec::new();
        for (index, occupancy) in self.occupancies.iter().enumerate() {
            let config = &occupancy.stratum_configuration;
            let component = tile_qualified_component_id(&occupancy.occupancy_id)?;
            bindings.push(V8ComponentOccupancyBinding {
                component_id: component.clone(),
                occupancy_id: occupancy.occupancy_id.clone(),
                vertical_rank: occupancy.vertical_rank,
            });
            let layers = [
                &self.radiation.visible_direct.layers[index],
                &self.radiation.visible_diffuse.layers[index],
                &self.radiation.near_infrared_direct.layers[index],
                &self.radiation.near_infrared_diffuse.layers[index],
            ];
            let bands =
                |field: fn(&openwepp_vegetation::radiation::OwnedLayerAbsorption) -> f64| {
                    BandDirectionalFluxes {
                        direct_vis: field(layers[0]),
                        diffuse_vis: field(layers[1]),
                        direct_nir: field(layers[2]),
                        diffuse_nir: field(layers[3]),
                    }
                };
            let sun_bands = bands(|v| v.absorbed_leaf_sun);
            let shade_bands = bands(|v| v.absorbed_leaf_shade);
            let stem_bands = bands(|v| v.absorbed_stem);
            // E01 owns the photosynthetic sun/shade leaf-class areas. The
            // other directional/band solves retain their own absorption
            // operands and are not alternate owners of this classification.
            let sun_area = layers[0].leaf_sun_area;
            let shade_area = layers[0].leaf_shade_area;
            let leaf_n = occupancy
                .shared_state
                .tissues
                .get(&Tissue::Leaf)
                .ok_or(V8InputProjectionError::Topology("missing leaf tissue"))?
                .display
                .nitrogen;
            let leaf_n_area = if occupancy.shared_state.leaf_area == 0.0 {
                if leaf_n != 0.0 {
                    return Err(V8InputProjectionError::Identity(
                        "leaf nitrogen without area",
                    ));
                }
                0.0
            } else {
                leaf_n / occupancy.shared_state.leaf_area
            };
            let rd25 = if occupancy.shared_state.leaf_area == 0.0 {
                0.0
            } else {
                let advanced_t10_k = update_t10(
                    occupancy.shared_state.t10_k,
                    self.vegetation_forcing.air_temperature_k,
                    self.interval_s,
                )?;
                atkin_rd25(
                    leaf_n,
                    occupancy.shared_state.leaf_area,
                    advanced_t10_k,
                    config.atkin_intercept,
                )?
            };
            let leaf = |area: f64, flux: BandDirectionalFluxes| LeafBiochemicalInputs {
                leaf_area_m2_m2_tile: area,
                absorbed_shortwave_w_m2_tile: flux.total(),
                absorbed_par_w_m2_leaf: if area == 0.0 {
                    0.0
                } else {
                    (flux.direct_vis + flux.diffuse_vis) / area
                },
                vcmax25: if area == 0.0 {
                    0.0
                } else {
                    leaf_n_area * config.rubisco_n_efficiency
                },
                jmax25: if area == 0.0 {
                    0.0
                } else {
                    leaf_n_area * config.electron_n_efficiency
                },
                rd25: if area == 0.0 { 0.0 } else { rd25 },
            };
            let u_star = canopy_surface_friction_velocity(
                self.vegetation_forcing.wind_m_s,
                self.vegetation_forcing.reference_height_m,
                config.displacement_m,
                config.z0m_m,
            )?;
            let root_layers = occupancy
                .root_layers
                .iter()
                .map(|root| {
                    let configured = config
                        .root_layers
                        .iter()
                        .find(|v| v.layer_id == root.forcing.layer_id)
                        .ok_or(V8InputProjectionError::Topology("unconfigured root layer"))?;
                    let authority = if configured.root_fraction > 0.0 {
                        self.root_zone_hydraulics
                            .as_ref()
                            .map(|receipts| {
                                receipts.get(&V10RootZoneReceiptKey {
                                    ofe_id: occupancy.ofe_id.clone(),
                                    production_lane_index: occupancy.production_lane_index,
                                    production_lane_id: occupancy.production_lane_id,
                                    occupancy_id: occupancy.occupancy_id.clone(),
                                    stratum_id: config.stratum_id.clone(),
                                    layer_id: root.forcing.layer_id.clone(),
                                })
                            })
                            .transpose()?
                    } else {
                        None
                    };
                    if authority.is_some_and(|receipt| {
                        receipt.lateral_root_length_m.to_bits()
                            != configured.lateral_root_length_m.to_bits()
                    }) {
                        return Err(V8InputProjectionError::Identity(
                            "root-zone lateral-root identity",
                        ));
                    }
                    Ok(RootHydraulicLayer {
                        layer_id: root.forcing.layer_id.as_str().into(),
                        accessible: authority.map_or(root.forcing.accessible, |_| true),
                        frozen: authority.map_or(root.forcing.frozen, |_| false),
                        root_fraction: configured.root_fraction,
                        soil_potential_mm: authority.map_or_else(
                            || root.forcing.matric_potential_mm,
                            |value| value.matric_potential_mm,
                        ),
                        gravity_head_mm: authority.map_or_else(
                            || root.forcing.gravity_root_mm,
                            |value| value.gravity_root_mm,
                        ),
                        z3_m: authority.map_or_else(
                            || root.forcing.root_path_length_mm / 1000.0,
                            |value| value.root_path_length_mm / 1000.0,
                        ),
                        dxroot_m: configured.lateral_root_length_m,
                        ksoil_m2_s: authority.map_or_else(
                            || root.forcing.hydraulic_conductivity_mm_s / 1000.0,
                            |value| value.hydraulic_conductivity_mm_s / 1000.0,
                        ),
                    })
                })
                .collect::<Result<Vec<_>, V8InputProjectionError>>()?;
            for root in &occupancy.root_layers {
                let source_id = SourceId::try_new(root.forcing.layer_id.as_str())?;
                let identity = RootRuntimeIdentity {
                    solver_occupancy_id: component.as_str().into(),
                    requesting_owner_id: vegetation_owner_id.clone(),
                    occupancy_id: ComponentId::try_new(component.as_str())?,
                    layer_id: root.forcing.layer_id.clone(),
                    source_id: source_id.clone(),
                };
                let key = openwepp_land_surface_energy::GroundWaterKey {
                    transaction_id: self.transaction_id,
                    requesting_owner_id: vegetation_owner_id.clone(),
                    requesting_component: RequestingComponent::VegetationRoot,
                    ofe_id: self.ofe_id.clone(),
                    requesting_tile_id: self.tile_id.clone(),
                    occupancy_id: Some(identity.occupancy_id.clone()),
                    surface_id: None,
                    surface_class: None,
                    source_type: WaterSourceType::SoilLayerLiquid,
                    source_id,
                    source_tile_id: None,
                    soil_layer_id: Some(root.forcing.layer_id.clone()),
                    amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
                };
                if sources.insert(key, root.source.clone()).is_some() {
                    return Err(V8InputProjectionError::Topology("duplicate root source"));
                }
                roots.push(identity);
            }
            rows.push(CoveredOccupancyInputs {
                occupancy_id: component.as_str().into(),
                medlyn_g1_kpa_sqrt: config.g1_sqrt_kpa,
                g0_umol_m2_s: config.g0_umol_h2o_m2_s,
                sun: leaf(sun_area, sun_bands),
                shade: leaf(shade_area, shade_bands),
                biochemical: biochemical(config),
                stem_area_m2_m2_tile: occupancy.conditional_sai_m2_m2_tile_ground,
                stem_absorbed_shortwave_w_m2_tile: stem_bands.total(),
                beginning_canopy_liquid_kg_m2_tile: occupancy
                    .occupancy_state
                    .canopy_liquid_kg_h2o_m2_tile_ground,
                liquid_interception_fraction: config.alpha_liq,
                liquid_capacity_kg_m2_plant: config.p_liq_kg_m2_plant,
                stemflow_fraction: config.stemflow_fraction,
                gb_leaf_m_s: leaf_boundary_conductance(u_star, config.leaf_dimension_m)?,
                gb_wet_m_s: leaf_boundary_conductance(u_star, config.wet_surface_dimension_m)?,
                gb_stem_m_s: leaf_boundary_conductance(u_star, config.stem_dimension_m)?,
                lai: occupancy.conditional_lai_m2_m2_tile_ground,
                sai: occupancy.conditional_sai_m2_m2_tile_ground,
                clumping_index: config.clumping_index,
                k1_sun_max_s1: config.k1a_max_s1,
                k1_shade_max_s1: config.k1b_max_s1,
                k2_max: config.k2_max_m_s,
                k3_max_m_s: config.k3_max_m_s,
                height_m: config.height_m,
                root_to_leaf_area: config.root_to_leaf_area,
                p50_leaf_mm: config.p50_leaf_mm,
                p50_xylem_mm: config.p50_stem_mm,
                p50_root_mm: config.p50_root_mm,
                vulnerability_exponent: config.vulnerability_shape,
                root_layers,
            });
            sw_rows.push(CoveredOccupancyShortwaveInputs {
                occupancy_id: component.as_str().into(),
                sun_leaf_absorbed_w_m2_tile: sun_bands,
                shade_leaf_absorbed_w_m2_tile: shade_bands,
                stem_absorbed_w_m2_tile: stem_bands,
            });
            let lane = &occupancy.occupancy_state;
            trial.extend([
                lane.sun_leaf_potential_mm,
                lane.shade_leaf_potential_mm,
                lane.stem_potential_mm,
                lane.root_node_potential_mm,
                lane.beta_hyd,
                lane.beta_hyd,
                lane.sun_leaf_temperature_k,
                lane.shade_leaf_temperature_k,
                lane.wet_surface_temperature_k,
                lane.dry_stem_temperature_k,
            ]);
        }
        trial.extend([
            canopy.canopy_air_temperature_k,
            canopy.canopy_air_specific_humidity_kg_kg,
            ground.surface_temperature_warm_start_k,
        ]);
        trial.extend(ground.soil_nodes.iter().map(|v| v.beginning_temperature_k));
        let (geometry, rah, raw) = covered_aerodynamics(
            &self.ground.configuration.turbulence,
            self.forcing.reference_wind_m_s,
        )?;
        let fluxes = |field: fn(&ColumnRadiationResult) -> f64| BandDirectionalFluxes {
            direct_vis: field(&self.radiation.visible_direct),
            diffuse_vis: field(&self.radiation.visible_diffuse),
            direct_nir: field(&self.radiation.near_infrared_direct),
            diffuse_nir: field(&self.radiation.near_infrared_diffuse),
        };
        let shortwave = CoveredColumnShortwaveInputs {
            incident_w_m2_tile: fluxes(|v| v.incident),
            top_reflected_w_m2_tile: fluxes(|v| v.top_reflected),
            ground_absorbed_by_incident_w_m2_tile: fluxes(|v| v.ground_absorbed),
            occupancies: sw_rows,
        };
        Ok((
            CoveredColumnInputs {
                authority: openwepp_land_surface_energy::CoveredColumnAuthority::HistoricalV8,
                interval_s: self.interval_s,
                tile_fraction: self.tile_fraction,
                pressure_pa: self.vegetation_forcing.pressure_pa,
                air_temperature_k: self.vegetation_forcing.air_temperature_k,
                air_specific_humidity_kg_kg: self.vegetation_forcing.specific_humidity,
                reference_wind_m_s: self.vegetation_forcing.wind_m_s,
                atmospheric_downward_longwave_w_m2: self.vegetation_forcing.longwave_down_w_m2,
                ca_pa: self.vegetation_forcing.co2_pa,
                canopy_to_atmosphere_heat_resistance_s_m: rah,
                canopy_to_atmosphere_vapor_resistance_s_m: raw,
                latent_heat_j_kg: LATENT_HEAT_VAPORIZATION,
                top_rain_kg_m2_tile: covered_precipitation_for_tile(
                    &self.forcing,
                    &self.ofe_id,
                    &self.tile_id,
                ),
                under_canopy_geometry: geometry,
                ground,
                occupancies: rows,
                shortwave,
                stage3_lower_boundary: None,
                stage3_optical: self.radiation.stage3_optical.clone(),
            },
            roots,
            sources,
            bindings,
            trial,
        ))
    }
}

fn biochemical(c: &StratumConfiguration) -> BiochemicalConstants {
    BiochemicalConstants {
        ha_vcmax_j_mol: c.ha_vcmax,
        hd_vcmax_j_mol: c.hd_vcmax,
        entropy_vcmax_j_mol_k: c.entropy_vcmax,
        ha_jmax_j_mol: c.ha_jmax,
        hd_jmax_j_mol: c.hd_jmax,
        entropy_jmax_j_mol_k: c.entropy_jmax,
        kc25_pa: c.kc25_pa,
        ha_kc_j_mol: c.ha_kc,
        ko25_pa: c.ko25_pa,
        ha_ko_j_mol: c.ha_ko,
        gamma25_pa: c.gamma25_pa,
        ha_gamma_j_mol: c.ha_gamma,
        oxygen_partial_pressure_pa: 21_230.0,
        tp_vcmax_ratio: c.tp_vcmax_ratio,
        electron_quantum_yield: 0.3,
        par_photon_umol_per_j: 4.6,
        electron_curvature: 0.9,
        ac_aj_curvature: 0.98,
        ag_ap_curvature: 0.95,
    }
}

fn covered_aerodynamics(
    t: &TurbulenceConfiguration,
    wind: f64,
) -> Result<(UnderCanopyGeometry, f64, f64), V8InputProjectionError> {
    match t {
        TurbulenceConfiguration::CoveredNeutral {
            canopy_height_m,
            ground_exchange_roughness_m,
            leaf_area_index_m2_m2_tile_ground,
            canopy_to_reference,
        } => Ok((
            UnderCanopyGeometry {
                canopy_height_m: *canopy_height_m,
                canopy_roughness_m: *ground_exchange_roughness_m,
                reference_height_m: canopy_to_reference.reference_height_m,
                leaf_area_index: *leaf_area_index_m2_m2_tile_ground,
            },
            neutral_resistance(
                canopy_to_reference.reference_height_m,
                canopy_to_reference.displacement_m,
                canopy_to_reference.roughness_momentum_m,
                canopy_to_reference.roughness_heat_m,
                wind,
            )?,
            neutral_resistance(
                canopy_to_reference.reference_height_m,
                canopy_to_reference.displacement_m,
                canopy_to_reference.roughness_momentum_m,
                canopy_to_reference.roughness_vapor_m,
                wind,
            )?,
        )),
        TurbulenceConfiguration::OpenNeutral { .. } => Err(V8InputProjectionError::Topology(
            "covered occupancy with open turbulence",
        )),
    }
}

fn open_beginning_trial(problem: &OpenSurfaceProblem) -> Vec<f64> {
    std::iter::once(problem.surface_temperature_warm_start_k)
        .chain(
            problem
                .soil_nodes
                .iter()
                .map(|node| node.beginning_temperature_k),
        )
        .collect()
}

fn project_forest_litter_conductivity(
    beginning_litter_liquid_kg_m2_tile: f64,
    litter_thickness_m: f64,
) -> Result<f64, V8InputProjectionError> {
    const WATER_DENSITY_KG_M3: f64 = 1_000.0;
    if !beginning_litter_liquid_kg_m2_tile.is_finite()
        || beginning_litter_liquid_kg_m2_tile < 0.0
        || !litter_thickness_m.is_finite()
        || litter_thickness_m <= 0.0
    {
        return Err(V8InputProjectionError::Topology(
            "forest-litter conductivity operands",
        ));
    }
    Ok(
        0.1 + 0.03 * beginning_litter_liquid_kg_m2_tile
            / (WATER_DENSITY_KG_M3 * litter_thickness_m),
    )
}

fn covered_precipitation_for_tile(
    forcing: &LandSurfaceForcing,
    ofe_id: &OfeId,
    tile_id: &TileId,
) -> f64 {
    forcing
        .precipitation_parcels
        .iter()
        .filter(|parcel| {
            parcel.parcel_kind == openwepp_land_surface_energy::LiquidParcelKind::Precipitation
                && &parcel.destination_ofe_id == ofe_id
                && &parcel.destination_tile_id == tile_id
        })
        .map(|parcel| parcel.amount_kg_m2_destination_tile_ground)
        .fold(0.0, |total, amount| total + amount)
}

fn validate_covered_precipitation_join(
    forcing: &LandSurfaceForcing,
    ofe_id: &OfeId,
    tile_id: &TileId,
    vegetation_rain_kg_m2: f64,
) -> Result<(), V8InputProjectionError> {
    let canonical = covered_precipitation_for_tile(forcing, ofe_id, tile_id);
    if canonical.to_bits() == vegetation_rain_kg_m2.to_bits() {
        Ok(())
    } else {
        Err(V8InputProjectionError::Identity(
            "covered precipitation owner join",
        ))
    }
}

fn tile_qualified_component_id(
    occupancy: &OccupancyId,
) -> Result<V8LseComponentId, V8InputProjectionError> {
    Ok(V8LseComponentId::try_new(format!(
        "{}::{}",
        occupancy.stratum_id.as_str(),
        occupancy.tile_id.as_str()
    ))?)
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn project_v8_runtime_inputs_with_carriers(
    vegetation_configuration: &VegetationConfiguration,
    vegetation_state: &V8CoupledOwnedState,
    vegetation_owner_id: &ResourceOwnerId,
    biogeochemistry_owner_id: &ResourceOwnerId,
    beginning_biogeochemistry_state_sha256: &Sha256Digest,
    canopy_forcing: &V8CanopyForcingReceipt,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    lse_state: &LandSurfaceEnergyState,
    lse_forcing: &LandSurfaceForcing,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    soil_thermal: &V8SoilThermalPhysicalBeginning,
    day_index: usize,
    interval_index: u8,
    authenticated_duration_s_bits: Option<u64>,
    covered_lower_boundaries: Option<&BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>>,
    covered_destinations: Option<&BTreeSet<(OfeId, TileId)>>,
) -> Result<ValidatedV8RuntimeInputProjection, V8InputProjectionError> {
    vegetation_configuration.validate_v8()?;
    let configured_bindings = vegetation_configuration
        .strata
        .iter()
        .flat_map(|stratum| {
            stratum.tile_ids.iter().map(|tile_id| {
                let occupancy_id = OccupancyId {
                    stratum_id: stratum.stratum_id.clone(),
                    tile_id: tile_id.clone(),
                };
                Ok(V8ComponentOccupancyBinding {
                    component_id: tile_qualified_component_id(&occupancy_id)?,
                    occupancy_id,
                    vertical_rank: stratum.vertical_rank,
                })
            })
        })
        .collect::<Result<Vec<_>, V8InputProjectionError>>()?;
    validate_v8_component_bindings(&configured_bindings, vegetation_configuration)?;
    vegetation_state
        .validate(vegetation_configuration)
        .map_err(|_| V8InputProjectionError::Identity("invalid V8 vegetation state"))?;
    match lse_configuration.model_version.as_str() {
        openwepp_land_surface_energy::MODEL_VERSION => lse_configuration.validate()?,
        openwepp_land_surface_energy::V2_MODEL_VERSION => lse_configuration.validate_v2()?,
        _ => {
            return Err(V8InputProjectionError::Identity(
                "unsupported LSE configuration version",
            ));
        }
    }
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
        day_index,
        interval_index,
    )?;
    validate_forcing_join(
        vegetation_configuration,
        canopy_forcing.forcing(),
        lse_forcing,
        authenticated_duration_s_bits,
    )?;
    if let V8SoilThermalPhysicalBeginning::V2(prepared) = soil_thermal {
        let owner = prepared.beginning_owner();
        let support_duration_ns = owner
            .support_end_ns
            .checked_sub(owner.support_start_ns)
            .ok_or(V8InputProjectionError::Identity(
                "native V2 soil support bounds",
            ))?;
        let forcing_duration = std::time::Duration::try_from_secs_f64(lse_forcing.interval_s)
            .map_err(|_| V8InputProjectionError::Identity("native V2 forcing duration"))?;
        if owner.transaction_id != transaction_id
            || forcing_duration.as_nanos() != support_duration_ns
        {
            return Err(V8InputProjectionError::Identity(
                "native V2 soil support join",
            ));
        }
    }
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

    let soil_thermal_ofes = soil_thermal.ordered_ofes();
    let mut tiles = Vec::new();
    for ofe in &lse_configuration.ofes {
        let thermal = soil_thermal_ofes
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
            let covered = vegetation_configuration
                .strata
                .iter()
                .any(|stratum| stratum.tile_ids.contains(&tile.vegetation_tile_id));
            if covered {
                let vegetation_tile = vegetation_configuration
                    .topology_tiles
                    .iter()
                    .find(|value| value.tile_id == tile.vegetation_tile_id)
                    .ok_or(V8InputProjectionError::Topology("missing vegetation tile"))?;
                if tile.fraction_ofe_ground.to_bits() != vegetation_tile.fraction.to_bits() {
                    return Err(V8InputProjectionError::Identity("tile fraction mismatch"));
                }
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
            let tile_lse_forcing = lse_forcing.clone();
            let tile_vegetation_forcing = canopy_forcing.forcing().clone();
            // The covered adopter independently requires the complete
            // destination set for each active Stage-3 lane. Vegetation-covered
            // tiles on inactive lanes retain this ordinary LSE projection.
            if covered {
                validate_covered_precipitation_join(
                    lse_forcing,
                    &ofe.ofe_id,
                    &tile.tile_id,
                    canopy_forcing.forcing().rain_kg_m2,
                )?;
            }
            let destination = (ofe.ofe_id.clone(), tile.tile_id.clone());
            let stage3_covered_destination =
                covered_destinations.is_none_or(|destinations| destinations.contains(&destination));
            let covered_lower_boundary = if covered && stage3_covered_destination {
                covered_lower_boundaries.and_then(|boundaries| boundaries.get(&destination))
            } else {
                None
            };
            let (radiation, occupancies) = project_column(
                vegetation_configuration,
                vegetation_state,
                &tile_vegetation_forcing,
                canopy_forcing.root_zone_hydraulics.as_ref(),
                &ofe.ofe_id,
                &tile.vegetation_tile_id,
                tile.surface_vis_albedo,
                tile.surface_nir_albedo,
                lane.production_lane_index,
                lane.production_lane_id,
                soil_adapter,
                covered_lower_boundary,
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
                    vegetation_owner_id: vegetation_owner_id.clone(),
                    biogeochemistry_owner_id: biogeochemistry_owner_id.clone(),
                    configuration_sha256: lse_configuration.configuration_sha256.clone(),
                    beginning_lse_state_sha256: lse_state.state_sha256.clone(),
                    beginning_hydrology_snapshot_sha256: hydrology_snapshot_sha256.clone(),
                    beginning_soil_thermal_state_sha256: soil_thermal.state_sha256().clone(),
                    beginning_vegetation_state_sha256: Sha256Digest::try_new(
                        vegetation_state.state_sha256.clone(),
                    )
                    .map_err(|_| V8InputProjectionError::Identity("vegetation state digest"))?,
                    beginning_biogeochemistry_state_sha256: beginning_biogeochemistry_state_sha256
                        .clone(),
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
                forcing: tile_lse_forcing,
                vegetation_forcing: tile_vegetation_forcing,
                root_zone_hydraulics: canopy_forcing.root_zone_hydraulics.clone(),
                canopy_air_state,
                radiation,
                ground: V8ProjectedGroundInput {
                    configuration: tile.clone(),
                    soil_interface_layers: ofe.soil_interface_layers.clone(),
                    state: tile_state.clone(),
                    surface_liquid: surface_record.clone(),
                    soil_thermal: thermal.clone(),
                    top_hydrology: soil_adapter
                        .owner
                        .layer_facts()
                        .get(&RealHydrologySourceKey {
                            ofe_lane:
                                crate::vegetation_real_hydrology_shadow::RealHydrologyOfeLaneId {
                                    lane_index: lane.production_lane_index,
                                    lane_id: lane.production_lane_id,
                                },
                            layer_id: ofe.soil_interface_layers[0].layer_id.clone(),
                        })
                        .cloned()
                        .ok_or(V8InputProjectionError::Topology(
                            "missing top live hydrology owner",
                        ))?,
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
        soil_thermal_snapshot_sha256: soil_thermal.snapshot_sha256()?,
        soil_thermal_beginning: soil_thermal.clone(),
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
    soil_thermal: &V8SoilThermalPhysicalBeginning,
    transaction_id: TransactionId,
    canopy_forcing: &V8CanopyForcingReceipt,
    hydrology_snapshot_sha256: &Sha256Digest,
    day_index: usize,
    interval_index: u8,
) -> Result<(), V8InputProjectionError> {
    let cross_owner = |valid: bool, detail: &'static str| {
        valid
            .then_some(())
            .ok_or(V8InputProjectionError::Identity(detail))
    };
    cross_owner(
        lse_configuration
            .vegetation_configuration
            .configuration_sha256
            .as_str()
            == vegetation_configuration.configuration_sha256,
        "cross-owner vegetation configuration",
    )?;
    cross_owner(
        vegetation_state.configuration_sha256 == vegetation_configuration.configuration_sha256,
        "cross-owner vegetation state configuration",
    )?;
    cross_owner(
        lse_configuration
            .soil_thermal_configuration
            .configuration_sha256
            == *soil_thermal.configuration_sha256(),
        "cross-owner soil-thermal configuration",
    )?;
    cross_owner(
        lse_forcing.transaction_id == transaction_id,
        "cross-owner LSE forcing transaction",
    )?;
    cross_owner(
        soil_adapter.owner.transaction_id() == transaction_id,
        "cross-owner hydrology transaction",
    )?;
    cross_owner(
        soil_adapter.owner.interval_s().to_bits() == lse_forcing.interval_s.to_bits(),
        "cross-owner hydrology support",
    )?;
    cross_owner(
        canopy_forcing.vegetation_configuration_sha256
            == vegetation_configuration.configuration_sha256,
        "cross-owner canopy vegetation configuration",
    )?;
    cross_owner(
        canopy_forcing.vegetation_beginning_state_sha256 == vegetation_state.state_sha256,
        "cross-owner canopy vegetation state",
    )?;
    cross_owner(
        canopy_forcing.lse_configuration_sha256 == lse_configuration.configuration_sha256,
        "cross-owner canopy LSE configuration",
    )?;
    cross_owner(
        canopy_forcing.lse_forcing_sha256 == lse_forcing.forcing_sha256,
        "cross-owner canopy LSE forcing",
    )?;
    cross_owner(
        &canopy_forcing.hydrology_snapshot_sha256 == hydrology_snapshot_sha256,
        "cross-owner canopy hydrology snapshot",
    )?;
    cross_owner(
        canopy_forcing.soil_thermal_snapshot_sha256 == soil_thermal.snapshot_sha256()?,
        "cross-owner canopy soil-thermal snapshot",
    )?;
    cross_owner(
        canopy_forcing.transaction_id == transaction_id,
        "cross-owner canopy transaction",
    )?;
    if let Some(root) = &canopy_forcing.root_zone_hydraulics {
        if root.owner_id != ROOT_ZONE_OWNER_ID
            || root.model_definition_sha256 != ROOT_ZONE_MODEL_SHA256
        {
            return Err(V8InputProjectionError::RootOwnerJoin("root model identity"));
        }
        if root.vegetation_configuration_sha256 != vegetation_configuration.configuration_sha256
            || root.lse_configuration_sha256 != lse_configuration.configuration_sha256
            || root.hydrology_configuration_sha256
                != lse_configuration
                    .hydrology_configuration
                    .configuration_sha256
        {
            return Err(V8InputProjectionError::RootOwnerJoin(
                "root configuration identities",
            ));
        }
        if &root.hydrology_snapshot_sha256 != hydrology_snapshot_sha256
            || root.transaction_id != transaction_id
            || root.day_index != day_index
            || root.interval_index != interval_index
        {
            return Err(V8InputProjectionError::RootOwnerJoin(
                "root interval lineage",
            ));
        }
        if root.vegetation_root_bindings_sha256 != root_binding_digest(&root.receipts)
            || root.receipt_sha256 != root.compute_digest()
        {
            return Err(V8InputProjectionError::RootReceiptDigest);
        }
    }
    Ok(())
}

fn validate_forcing_join(
    vegetation_configuration: &VegetationConfiguration,
    vegetation: &SnowFreeForcing,
    lse: &LandSurfaceForcing,
    authenticated_duration_s_bits: Option<u64>,
) -> Result<(), V8InputProjectionError> {
    let expected_duration_bits =
        authenticated_duration_s_bits.unwrap_or_else(|| vegetation_configuration.dt_s.to_bits());
    let exact = expected_duration_bits == lse.interval_s.to_bits()
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
    root_zone_hydraulics: Option<&V10RootZoneReceiptSet>,
    ofe_id: &OfeId,
    tile_id: &TileId,
    surface_vis_albedo: f64,
    surface_nir_albedo: f64,
    lane_index: usize,
    lane_id: u32,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    covered_lower_boundary: Option<&Stage3SnowCoveredLowerBoundary>,
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
    let effective_vis_albedo =
        covered_lower_boundary.map_or(surface_vis_albedo, |boundary| boundary.snow_vis_albedo);
    let effective_nir_albedo =
        covered_lower_boundary.map_or(surface_nir_albedo, |boundary| boundary.snow_nir_albedo);
    let visible_direct = solve_mixed_column(
        &layers_vis,
        RadiationBand::Visible,
        IncidentComponent::Direct,
        forcing.solar_zenith_cosine,
        effective_vis_albedo,
        forcing.direct_par_w_m2,
    )?;
    let visible_diffuse = solve_mixed_column(
        &layers_vis,
        RadiationBand::Visible,
        IncidentComponent::Diffuse,
        forcing.solar_zenith_cosine,
        effective_vis_albedo,
        forcing.diffuse_par_w_m2,
    )?;
    let near_infrared_direct = solve_mixed_column(
        &layers_nir,
        RadiationBand::NearInfrared,
        IncidentComponent::Direct,
        forcing.solar_zenith_cosine,
        effective_nir_albedo,
        forcing.direct_nir_w_m2,
    )?;
    let near_infrared_diffuse = solve_mixed_column(
        &layers_nir,
        RadiationBand::NearInfrared,
        IncidentComponent::Diffuse,
        forcing.solar_zenith_cosine,
        effective_nir_albedo,
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
            if root_zone_hydraulics.is_none()
                && (layer_forcing.water_beginning_kg_m2.to_bits()
                    != fact.liquid_supply_kg_m2.to_bits()
                    || layer_forcing.frozen != fact.frozen)
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
            ofe_id: ofe_id.clone(),
            production_lane_index: lane_index,
            production_lane_id: lane_id,
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
    let stage3_optical = covered_lower_boundary
        .map(|boundary| {
            stage3_optical_receipt(
                ofe_id,
                tile_id,
                &visible_direct,
                &visible_diffuse,
                &near_infrared_direct,
                &near_infrared_diffuse,
                boundary,
            )
        })
        .transpose()?;
    Ok((
        V8ProjectedColumnRadiation {
            visible_direct,
            visible_diffuse,
            near_infrared_direct,
            near_infrared_diffuse,
            stage3_optical,
        },
        occupancies,
    ))
}

fn stage3_optical_receipt(
    ofe_id: &OfeId,
    tile_id: &TileId,
    visible_direct: &ColumnRadiationResult,
    visible_diffuse: &ColumnRadiationResult,
    near_infrared_direct: &ColumnRadiationResult,
    near_infrared_diffuse: &ColumnRadiationResult,
    boundary: &Stage3SnowCoveredLowerBoundary,
) -> Result<Stage3SnowOpticalBoundaryReceiptV1, V8InputProjectionError> {
    let terminal =
        |result: &ColumnRadiationResult| result.terminal_direct + result.terminal_diffuse;
    let terminal_w_m2_tile = BandDirectionalFluxes {
        direct_vis: terminal(visible_direct),
        diffuse_vis: terminal(visible_diffuse),
        direct_nir: terminal(near_infrared_direct),
        diffuse_nir: terminal(near_infrared_diffuse),
    };
    let absorbed_w_m2_tile = BandDirectionalFluxes {
        direct_vis: visible_direct.ground_absorbed,
        diffuse_vis: visible_diffuse.ground_absorbed,
        direct_nir: near_infrared_direct.ground_absorbed,
        diffuse_nir: near_infrared_diffuse.ground_absorbed,
    };
    let reflected_w_m2_tile = BandDirectionalFluxes {
        direct_vis: boundary.snow_vis_albedo * terminal_w_m2_tile.direct_vis,
        diffuse_vis: boundary.snow_vis_albedo * terminal_w_m2_tile.diffuse_vis,
        direct_nir: boundary.snow_nir_albedo * terminal_w_m2_tile.direct_nir,
        diffuse_nir: boundary.snow_nir_albedo * terminal_w_m2_tile.diffuse_nir,
    };
    Ok(Stage3SnowOpticalBoundaryReceiptV1::try_new(
        Stage3SnowOpticalBoundaryReceiptInputs {
            ofe_id: ofe_id.clone(),
            tile_id: tile_id.clone(),
            terminal_w_m2_tile,
            absorbed_w_m2_tile,
            reflected_w_m2_tile,
            snow_vis_albedo: boundary.snow_vis_albedo,
            snow_nir_albedo: boundary.snow_nir_albedo,
            stage3_albedo_state_sha256: boundary.stage3_albedo_state_sha256.clone(),
            forcing_receipt_sha256: boundary.forcing_receipt_sha256.clone(),
        },
    )?)
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

    fn precipitation_parcel(
        id: &str,
        destination_tile: &str,
        amount: f64,
    ) -> openwepp_land_surface_energy::LiquidParcel {
        openwepp_land_surface_energy::LiquidParcel {
            parcel_kind: openwepp_land_surface_energy::LiquidParcelKind::Precipitation,
            parcel_id: openwepp_land_surface_energy::ParcelId::try_new(id).expect("parcel"),
            source_owner_id: ResourceOwnerId::try_new("meteorology").expect("owner"),
            source_ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            source_tile_id: TileId::try_new("atmosphere").expect("tile"),
            destination_ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            destination_tile_id: TileId::try_new(destination_tile).expect("tile"),
            start_s: 0.0,
            end_s: 1_800.0,
            amount_kg_m2_destination_tile_ground: amount,
            temperature_provider:
                openwepp_land_surface_energy::LiquidTemperatureProvider::HarderPomeroyHourly,
            temperature_k: Some(280.0),
            specific_liquid_enthalpy_j_kg: Some(28_770.0),
            source_state_sha256: Some(
                Sha256Digest::try_new("e".repeat(64)).expect("source digest"),
            ),
        }
    }

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
        assert!(validate_forcing_join(&configuration, &baseline, &lse, None).is_ok());
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
                    assert!(
                        validate_forcing_join(&wrong_configuration, &baseline, &lse, None).is_err()
                    );
                    continue;
                }
            };
            *field = f64::from_bits(field.to_bits() + 1);
            assert!(validate_forcing_join(&configuration, &poison, &lse, None).is_err());
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

    #[test]
    #[allow(clippy::too_many_lines)]
    fn litter_conductivity_uses_beginning_litter_water_not_top_soil_conductivity() {
        use openwepp_kernel_contract::SoilLayerId;
        use openwepp_land_surface_energy::{SurfaceClass, SurfaceId};

        let beginning_litter_water = 2.0_f64;
        let litter_thickness = 0.04_f64;
        let deliberately_distinct_top_soil_conductivity = 1.1_f64;
        let digest = || Sha256Digest::try_new("a".repeat(64)).expect("digest");
        let owner = |value: &str| ResourceOwnerId::try_new(value).expect("owner");
        let ofe_id = OfeId::try_new("ofe-1").expect("OFE");
        let tile_id = TileId::try_new("forest").expect("tile");
        let surface_id = SurfaceId::try_new("surface:forest").expect("surface");
        let source_id = SourceId::try_new("liquid:forest").expect("source");
        let layer_id = SoilLayerId::try_new("soil-1").expect("layer");
        let configuration: TileConfiguration = serde_json::from_value(serde_json::json!({
            "tile_id": "forest",
            "fraction_ofe_ground": 1.0,
            "vegetation_tile_id": "forest",
            "surface_vis_albedo": 0.12,
            "surface_nir_albedo": 0.24,
            "surface_heat_storage_mode": "finite_capacity",
            "turbulence": {
                "mode": "open_neutral",
                "reference_height_m": 2.0,
                "roughness_momentum_m": 0.1,
                "roughness_heat_m": 0.01,
                "roughness_vapor_m": 0.01
            },
            "surface": {
                "surface_class": "forest_litter",
                "liquid_capacity_kg_m2_tile_ground": 6.0,
                "thickness_m": litter_thickness,
                "dry_density_kg_m3": 24.0,
                "dry_specific_heat_j_kg_k": 3370.5
            }
        }))
        .expect("tile configuration");
        let radiation = |band, incident| {
            solve_mixed_column(&[], band, IncidentComponent::Direct, 0.67, 0.12, incident)
                .expect("empty column radiation")
        };
        let identity = RuntimeTileIdentity {
            transaction_id: TransactionId(1),
            lse_owner_id: owner("lse"),
            hydrology_owner_id: owner("hydrology"),
            soil_thermal_owner_id: owner("soil-thermal"),
            vegetation_owner_id: owner("vegetation"),
            biogeochemistry_owner_id: owner("biogeochemistry"),
            configuration_sha256: digest(),
            beginning_lse_state_sha256: digest(),
            beginning_hydrology_snapshot_sha256: digest(),
            beginning_soil_thermal_state_sha256: digest(),
            beginning_vegetation_state_sha256: digest(),
            beginning_biogeochemistry_state_sha256: digest(),
            ofe_id: ofe_id.clone(),
            tile_id: tile_id.clone(),
            surface_id: surface_id.clone(),
            surface_class: SurfaceClass::ForestLitter,
            ground_source_type: WaterSourceType::LitterLiquid,
            ground_source_id: source_id.clone(),
            ground_source_tile_id: Some(tile_id.clone()),
            ground_soil_layer_id: None,
            tile_fraction: 1.0,
            interval_s: 1_800.0,
        };
        let tile = V8ProjectedTileRuntimeInput {
            identity,
            ofe_id: ofe_id.clone(),
            tile_id: tile_id.clone(),
            transaction_id: TransactionId(1),
            interval_s: 1_800.0,
            tile_fraction: 1.0,
            forcing: lse_forcing(),
            vegetation_forcing: vegetation_forcing(),
            root_zone_hydraulics: None,
            canopy_air_state: None,
            radiation: V8ProjectedColumnRadiation {
                visible_direct: radiation(RadiationBand::Visible, 410.0),
                visible_diffuse: radiation(RadiationBand::Visible, 83.0),
                near_infrared_direct: radiation(RadiationBand::NearInfrared, 355.0),
                near_infrared_diffuse: radiation(RadiationBand::NearInfrared, 101.0),
                stage3_optical: None,
            },
            ground: V8ProjectedGroundInput {
                configuration,
                soil_interface_layers: vec![SoilInterfaceLayer {
                    layer_id: layer_id.clone(),
                    thickness_m: 0.08,
                    thermal_conductivity_w_m_k: deliberately_distinct_top_soil_conductivity,
                    areal_heat_capacity_j_m2_k: 120_000.0,
                }],
                state: TileState {
                    ofe_id: ofe_id.clone(),
                    tile_id: tile_id.clone(),
                    surface_enthalpy_j_m2_tile_ground: 439_352.808,
                    surface_temperature_warm_start_k: 295.0,
                },
                surface_liquid: DirectSurfaceLiquidStateRecord {
                    key: crate::DirectSurfaceLiquidStoreKey {
                        run_id: 1,
                        ofe_id: ofe_id.clone(),
                        tile_id: tile_id.clone(),
                        surface_id,
                        surface_class: SurfaceClass::ForestLitter,
                        source_type: WaterSourceType::LitterLiquid,
                        source_id,
                    },
                    liquid_kg_m2_tile: beginning_litter_water,
                    last_accepted_transaction_id: None,
                },
                soil_thermal: V8ProjectedSoilThermalOfe {
                    ofe_id,
                    ordered_layers: vec![V8ProjectedSoilThermalLayer {
                        layer_id: layer_id.clone(),
                        temperature_k: 293.0,
                        enthalpy_hi_j_m2_ofe_ground: 1.0,
                        enthalpy_carry: ExactDyadicEnthalpy::zero(),
                    }],
                },
                top_hydrology: RealHydrologyLayerFact {
                    source: RealHydrologySourceKey {
                        ofe_lane: crate::vegetation_real_hydrology_shadow::RealHydrologyOfeLaneId {
                            lane_index: 0,
                            lane_id: 1,
                        },
                        layer_id: layer_id.clone(),
                    },
                    liquid_supply_kg_m2: 1.0,
                    frozen: false,
                    liquid_water_depth_m: 0.01,
                    layer_thickness_m: 0.08,
                    porosity: 0.4,
                    saturated_conductivity_m_s: 1.0e-6,
                },
            },
            occupancies: Vec::new(),
        };
        let projection = |tile| ValidatedV8RuntimeInputProjection {
            vegetation_configuration_sha256: "b".repeat(64),
            vegetation_state_sha256: "c".repeat(64),
            lse_configuration_sha256: digest(),
            lse_state_sha256: digest(),
            lse_forcing_sha256: digest(),
            hydrology_snapshot_sha256: digest(),
            soil_thermal_snapshot_sha256: digest(),
            soil_thermal_beginning: V8SoilThermalPhysicalBeginning::V1(SoilThermalSnapshot {
                owner_id: owner("soil-thermal"),
                configuration_sha256: digest(),
                state_sha256: digest(),
                snapshot_sha256: digest(),
                last_accepted_transaction_id: None,
                ofes: vec![openwepp_land_surface_energy::SoilThermalOfeSnapshot {
                    ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                    ordered_layers: vec![openwepp_land_surface_energy::SoilThermalLayerSnapshot {
                        layer_id: openwepp_kernel_contract::SoilLayerId::try_new("soil-1")
                            .expect("layer"),
                        temperature_k: 293.0,
                        enthalpy_j_m2_ofe_ground: 1.0,
                    }],
                }],
            }),
            transaction_id: TransactionId(1),
            tiles: vec![tile],
        };
        let conductivity = |tile| {
            let ready = projection(tile)
                .solver_ready_tiles(&owner("vegetation"))
                .expect("solver-ready projection");
            match &ready[0].physics {
                V8SolverReadyTilePhysics::Open(problem) => (
                    problem.surface_conductivity_w_m_k,
                    problem.soil_nodes[0].conductivity_w_m_k,
                ),
                V8SolverReadyTilePhysics::Covered(_) => panic!("empty occupancy is open"),
                V8SolverReadyTilePhysics::Stage3OpenSnow(_) => {
                    panic!("historical V8 cannot project Stage-3 pass-through")
                }
            }
        };

        let (projected, soil) = conductivity(tile.clone());
        let expected = 0.1 + 0.03 * beginning_litter_water / (1_000.0 * litter_thickness);
        assert_eq!(projected.to_bits(), expected.to_bits());
        assert_eq!(
            soil.to_bits(),
            deliberately_distinct_top_soil_conductivity.to_bits()
        );
        assert_ne!(projected.to_bits(), soil.to_bits());

        let mut changed_store = tile;
        changed_store.ground.surface_liquid.liquid_kg_m2_tile += 0.25;
        let (poisoned, unchanged_soil) = conductivity(changed_store);
        assert_ne!(poisoned.to_bits(), projected.to_bits());
        assert_eq!(unchanged_soil.to_bits(), soil.to_bits());
    }

    #[test]
    fn shared_stratum_component_identity_is_tile_qualified() {
        let occupancy = |tile: &str| OccupancyId {
            stratum_id: openwepp_kernel_contract::StratumId::try_new("shared").expect("stratum"),
            tile_id: TileId::try_new(tile).expect("tile"),
        };
        let left = tile_qualified_component_id(&occupancy("left")).expect("left component");
        let right = tile_qualified_component_id(&occupancy("right")).expect("right component");
        assert_ne!(left, right);
        assert_eq!(left.as_str(), "shared::left");
        assert_eq!(right.as_str(), "shared::right");
        assert_root_receipts_are_qualified_by_ofe_and_production_lane();
    }

    #[test]
    fn covered_precipitation_join_rejects_missing_extra_wrong_destination_and_one_bit() {
        let ofe = OfeId::try_new("ofe-1").expect("OFE");
        let forest = TileId::try_new("forest").expect("tile");
        let mut forcing = lse_forcing();
        let rain = 0.25_f64;

        assert!(validate_covered_precipitation_join(&forcing, &ofe, &forest, rain).is_err());
        forcing.precipitation_parcels = vec![precipitation_parcel("rain-1", "forest", rain)];
        assert!(validate_covered_precipitation_join(&forcing, &ofe, &forest, rain).is_ok());

        assert!(
            validate_covered_precipitation_join(
                &forcing,
                &ofe,
                &forest,
                f64::from_bits(rain.to_bits() + 1),
            )
            .is_err()
        );
        forcing
            .precipitation_parcels
            .push(precipitation_parcel("rain-2", "forest", 0.01));
        assert!(validate_covered_precipitation_join(&forcing, &ofe, &forest, rain).is_err());
        forcing.precipitation_parcels = vec![precipitation_parcel("rain-1", "open", rain)];
        assert!(validate_covered_precipitation_join(&forcing, &ofe, &forest, rain).is_err());
    }

    fn assert_root_receipts_are_qualified_by_ofe_and_production_lane() {
        let occupancy = OccupancyId {
            stratum_id: openwepp_kernel_contract::StratumId::try_new("tree").unwrap(),
            tile_id: TileId::try_new("forest").unwrap(),
        };
        let layer = openwepp_kernel_contract::SoilLayerId::try_new("soil-1").unwrap();
        let key = |ofe: &str, lane_index: usize, lane_id: u32| V10RootZoneReceiptKey {
            ofe_id: OfeId::try_new(ofe).unwrap(),
            production_lane_index: lane_index,
            production_lane_id: lane_id,
            occupancy_id: occupancy.clone(),
            stratum_id: occupancy.stratum_id.clone(),
            layer_id: layer.clone(),
        };
        let receipt = |key: V10RootZoneReceiptKey, potential: f64| V10RootZoneLayerReceipt {
            key,
            matric_potential_mm: potential,
            hydraulic_conductivity_mm_s: 0.1,
            root_path_length_mm: 150.0,
            gravity_root_mm: -50.0,
            lateral_root_length_m: 0.02,
        };
        let left = key("ofe-1", 0, 10);
        let right = key("ofe-2", 1, 20);
        let set = V10RootZoneReceiptSet::try_new(
            "a".repeat(64),
            Sha256Digest::try_new("b".repeat(64)).unwrap(),
            "c".repeat(64),
            Sha256Digest::try_new("d".repeat(64)).unwrap(),
            Sha256Digest::try_new("e".repeat(64)).unwrap(),
            TransactionId(7),
            0,
            0,
            vec![
                receipt(left.clone(), -100.0),
                receipt(right.clone(), -250.0),
            ],
        )
        .unwrap();
        assert_eq!(
            set.get(&left).unwrap().matric_potential_mm.to_bits(),
            (-100.0_f64).to_bits()
        );
        assert_eq!(
            set.get(&right).unwrap().matric_potential_mm.to_bits(),
            (-250.0_f64).to_bits()
        );
        assert!(set.get(&key("ofe-2", 0, 10)).is_err());
    }
}
