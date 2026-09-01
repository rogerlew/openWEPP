//! BGC receiving-owner construction and the heterogeneous covered V8 envelope.
//!
//! Projection and vegetation persistence are completed before this boundary.
//! This module validates the already sealed candidates, joins root D/A/F to the
//! actual real-hydrology owner, and lets BGC construct its own receipts. It
//! exposes no partial or whole-owner commit API.

#![allow(dead_code)]

use std::collections::BTreeMap;

use openwepp_biogeochemistry::{
    BiogeochemistryError, BiogeochemistryOwnerCandidate, BiogeochemistryState, MaterialPool,
    MaterialProposal, MaterialReceipt, TransformationsMode, construct_biogeochemistry_candidate,
};
use openwepp_kernel_contract::{TileId, TransactionId};
use openwepp_land_surface_energy::{
    AcceptedCoveredVegetationOperands, CoveredLowerBoundaryEnergyOperands, GroundWaterKey, OfeId,
    RequestingComponent, Stage3SnowOpticalBoundaryReceiptV1, WaterAmount,
};
use openwepp_vegetation::{
    NitrogenArbiter, UncommittedV8VegetationCandidate, V8ComponentOccupancyBinding,
    V8CoupledOwnedState, V8PersistentForcingReceipt, VegetationConfiguration, VegetationError,
    carbon_nitrogen::MaterialTransfer, construct_uncommitted_v8_vegetation_candidate,
    execute_uncommitted_v8_persistent_phase, execute_uncommitted_v8_persistent_phase_v11,
};
use thiserror::Error;

use super::{
    CoveredForestShadowResult, UnifiedRealHydrologyCandidate,
    multi_tile_runtime::MultiTileRuntimeResult,
    v8_projection::{
        V8CoveredProjection, project_covered_forest_v8_passes, project_multi_tile_v8_passes_v11,
    },
};

pub(crate) type FixedCapCanopyReleasesByDestination = BTreeMap<
    (OfeId, TileId),
    (
        crate::DirectCanopyLiquidRelease,
        openwepp_coupled_time::Digest32,
    ),
>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum V8OwnerFailurePhase {
    Persistent,
    VegetationCandidate,
    BiogeochemistryCandidate,
    EnvelopeValidation,
}

type OwnerFailureHook<'a> =
    Option<&'a dyn Fn(V8OwnerFailurePhase) -> Result<(), CoveredV8OwnerEnvelopeError>>;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum CoveredV8OwnerEnvelopeError {
    #[error("V8 covered-owner identity failure: {0}")]
    Identity(&'static str),
    #[error(transparent)]
    LandSurface(#[from] openwepp_land_surface_energy::LandSurfaceEnergyError),
    #[error(transparent)]
    Vegetation(#[from] VegetationError),
    #[error(transparent)]
    Biogeochemistry(#[from] BiogeochemistryError),
    #[error(transparent)]
    Projection(#[from] super::V8ProjectionError),
}

/// Complete uncommitted covered owner set. LSE and soil-thermal candidates are
/// retained inside `physical`; the hydrology candidate is the actual unified
/// owner result rather than a reconstructed inventory.
#[derive(Clone, Debug, PartialEq)]
pub struct UncommittedCoveredV8OwnerEnvelope {
    transaction_id: TransactionId,
    vegetation: UncommittedV8VegetationCandidate,
    physical: CoveredV8PhysicalOwner,
    biogeochemistry: BiogeochemistryOwnerCandidate,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CoveredLseIterationState {
    pub canopy_air_temperature_k: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub snow_temperature_k: f64,
    pub snow_sensible_w_m2: f64,
    pub snow_vapor_kg_m2_s: f64,
    pub snow_latent_w_m2: f64,
    pub snow_net_longwave_w_m2: f64,
    pub component_temperatures_k: Vec<(String, [f64; 4])>,
    pub component_carrier_surfaces: Vec<CoveredCarrierComponentState>,
    pub canopy_sensible_w_m2: f64,
    pub canopy_vapor_kg_m2_s: f64,
    pub sensible_to_reference_air_w_m2: f64,
    pub vapor_to_reference_air_kg_m2_s: f64,
    pub shared_heat_residual_w_m2: f64,
    pub shared_heat_tolerance_w_m2: f64,
    pub shared_vapor_residual_kg_m2_s: f64,
    pub shared_vapor_tolerance_kg_m2_s: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CoveredCarrierComponentState {
    pub vertical_occupancy_ordinal: u32,
    pub occupancy_id: String,
    pub component_ordinal: u8,
    pub surface_area_m2_m2_tile: f64,
    pub emissive_area_m2_m2_tile: f64,
    pub heat_conductance_m_s_tile: f64,
    pub vapor_conductance_m_s_tile: f64,
    pub vapor_authorization_kg_m2_tile_s: Option<f64>,
    pub temperature_k: f64,
    pub specific_humidity_kg_kg: f64,
    pub sensible_to_canopy_air_w_m2: f64,
    pub vapor_to_canopy_air_kg_m2_s: f64,
}

#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
enum CoveredV8PhysicalOwner {
    Legacy(CoveredForestShadowResult),
    MultiTile(MultiTileRuntimeResult),
    FrozenLitterV3(UnifiedRealHydrologyCandidate),
}

/// Unpublished physical result used only while solving the covered Stage-3
/// fixed point. It contains no vegetation/BGC candidate, V8 projection
/// receipt, accepted publication, or complete-owner envelope.
pub(crate) struct ProvisionalCoveredV8PhysicalEvaluationV1 {
    transaction_id: TransactionId,
    physical: MultiTileRuntimeResult,
}

impl ProvisionalCoveredV8PhysicalEvaluationV1 {
    pub(crate) fn try_new(
        physical: MultiTileRuntimeResult,
    ) -> Result<Self, CoveredV8OwnerEnvelopeError> {
        let transaction_id = physical.hydrology_candidate().transaction_id();
        if physical.finalized_tiles().is_empty() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "empty provisional covered physical evaluation",
            ));
        }
        Ok(Self {
            transaction_id,
            physical,
        })
    }

    #[must_use]
    pub(crate) const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub(crate) fn hydrology(&self) -> &UnifiedRealHydrologyCandidate {
        self.physical.hydrology_candidate()
    }

    pub(crate) fn fixed_cap_canopy_releases_by_destination(
        &self,
        interval_s: f64,
    ) -> Result<FixedCapCanopyReleasesByDestination, CoveredV8OwnerEnvelopeError> {
        fixed_cap_canopy_releases_from_multi_tile(&self.physical, interval_s)
    }

    pub(crate) fn covered_lse_iteration_state_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), CoveredLseIterationState>, CoveredV8OwnerEnvelopeError>
    {
        covered_lse_iteration_states_from_multi_tile(&self.physical)
    }

    pub(crate) fn covered_snow_longwave_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
        covered_snow_longwave_from_multi_tile(&self.physical)
    }

    pub(crate) fn covered_snow_shortwave_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
        covered_snow_shortwave_from_multi_tile(&self.physical)
    }
}

fn fixed_cap_canopy_releases_from_multi_tile(
    physical: &MultiTileRuntimeResult,
    interval_s: f64,
) -> Result<FixedCapCanopyReleasesByDestination, CoveredV8OwnerEnvelopeError> {
    let mut releases = BTreeMap::new();
    for tile in physical.finalized_tiles() {
        let Some(covered) = tile.covered() else {
            continue;
        };
        covered.vegetation_operands.validate()?;
        let release = super::covered_derived_ingress::derive_release_from_ledgers(
            covered
                .vegetation_operands
                .occupancies
                .iter()
                .map(|row| (row.occupancy_id.as_str(), &row.liquid)),
            covered
                .vegetation_operands
                .ground_canopy_release_kg_m2_tile_ground,
            covered
                .vegetation_operands
                .ground_stemflow_kg_m2_tile_ground,
            interval_s,
        )
        .map_err(|_| CoveredV8OwnerEnvelopeError::Identity("fixed-cap release reconstruction"))?;
        let source_identity = openwepp_coupled_time::digest_bytes(
            &serde_json::to_vec(&covered.vegetation_operands).map_err(|_| {
                CoveredV8OwnerEnvelopeError::Identity("fixed-cap release source framing")
            })?,
        );
        let destination = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        if releases
            .insert(destination, (release, source_identity))
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate fixed-cap release destination",
            ));
        }
    }
    Ok(releases)
}

fn covered_lse_iteration_states_from_multi_tile(
    physical: &MultiTileRuntimeResult,
) -> Result<BTreeMap<(OfeId, TileId), CoveredLseIterationState>, CoveredV8OwnerEnvelopeError> {
    let mut states = BTreeMap::new();
    for tile in physical.finalized_tiles() {
        let Some(covered) = tile.covered() else {
            continue;
        };
        let lower = match &covered.energy_operands.lower_boundary {
            CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(value) => value,
            CoveredLowerBoundaryEnergyOperands::SnowFree(_) => continue,
        };
        let column = &covered.energy_operands.column;
        let component_temperatures_k = column
            .occupancies
            .iter()
            .map(|occupancy| {
                (
                    occupancy.occupancy_id.clone(),
                    [
                        occupancy.sun_leaf.surface_temperature_k,
                        occupancy.shade_leaf.surface_temperature_k,
                        occupancy.wet_surface.surface_temperature_k,
                        occupancy.dry_stem.surface_temperature_k,
                    ],
                )
            })
            .collect();
        let component_carrier_surfaces = column
            .occupancies
            .iter()
            .enumerate()
            .flat_map(|(vertical_ordinal, occupancy)| {
                [
                    occupancy.sun_leaf,
                    occupancy.shade_leaf,
                    occupancy.wet_surface,
                    occupancy.dry_stem,
                ]
                .into_iter()
                .enumerate()
                .map(|(ordinal, surface)| CoveredCarrierComponentState {
                    vertical_occupancy_ordinal: vertical_ordinal as u32,
                    occupancy_id: occupancy.occupancy_id.clone(),
                    component_ordinal: ordinal as u8,
                    surface_area_m2_m2_tile: surface.surface_area_m2_m2_tile,
                    emissive_area_m2_m2_tile: surface.emissive_area_m2_m2_tile,
                    heat_conductance_m_s_tile: surface.heat_conductance_m_s_tile,
                    vapor_conductance_m_s_tile: surface.vapor_conductance_m_s_tile,
                    vapor_authorization_kg_m2_tile_s: surface.vapor_authorization_kg_m2_tile_s,
                    temperature_k: surface.surface_temperature_k,
                    specific_humidity_kg_kg: surface.surface_specific_humidity_kg_kg,
                    sensible_to_canopy_air_w_m2: surface.sensible_to_canopy_air_w_m2_tile,
                    vapor_to_canopy_air_kg_m2_s: surface.signed_vapor_to_canopy_air_kg_m2_tile_s,
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let key = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        if states
            .insert(
                key,
                CoveredLseIterationState {
                    canopy_air_temperature_k: column.canopy_air.canopy_air_temperature_k,
                    canopy_air_specific_humidity_kg_kg: column
                        .canopy_air
                        .canopy_air_specific_humidity_kg_kg,
                    snow_temperature_k: lower.snow_temperature_k,
                    snow_sensible_w_m2: column.canopy_air.ground_sensible_to_canopy_air_w_m2_tile,
                    snow_vapor_kg_m2_s: column.canopy_air.ground_vapor_to_canopy_air_kg_m2_tile_s,
                    snow_latent_w_m2: column.canopy_air.ground_vapor_to_canopy_air_kg_m2_tile_s
                        * lower.latent_heat_j_kg,
                    snow_net_longwave_w_m2: column.longwave.ground_net_w_m2_tile,
                    component_temperatures_k,
                    component_carrier_surfaces,
                    canopy_sensible_w_m2: column.canopy_air.canopy_sensible_w_m2_tile,
                    canopy_vapor_kg_m2_s: column.canopy_air.canopy_vapor_kg_m2_tile_s,
                    sensible_to_reference_air_w_m2: column
                        .canopy_air
                        .sensible_to_reference_air_w_m2_tile,
                    vapor_to_reference_air_kg_m2_s: column
                        .canopy_air
                        .vapor_to_reference_air_kg_m2_tile_s,
                    shared_heat_residual_w_m2: column.canopy_air.shared_heat_residual_w_m2_tile,
                    shared_heat_tolerance_w_m2: column.canopy_air.shared_heat_tolerance_w_m2_tile,
                    shared_vapor_residual_kg_m2_s: column
                        .canopy_air
                        .shared_vapor_residual_kg_m2_tile_s,
                    shared_vapor_tolerance_kg_m2_s: column
                        .canopy_air
                        .shared_vapor_tolerance_kg_m2_tile_s,
                },
            )
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered iteration destination",
            ));
        }
    }
    Ok(states)
}

fn covered_snow_longwave_from_multi_tile(
    physical: &MultiTileRuntimeResult,
) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
    let mut receipts = BTreeMap::new();
    for tile in physical.finalized_tiles() {
        let Some(covered) = tile.covered() else {
            continue;
        };
        let key = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        let value = covered.energy_operands.column.longwave.ground_net_w_m2_tile;
        if receipts.insert(key, value).is_some() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered longwave destination",
            ));
        }
    }
    if receipts.is_empty() {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "empty covered longwave destination set",
        ));
    }
    Ok(receipts)
}

fn covered_snow_shortwave_from_multi_tile(
    physical: &MultiTileRuntimeResult,
) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
    let mut receipts = BTreeMap::new();
    for tile in physical.finalized_tiles() {
        let Some(covered) = tile.covered() else {
            continue;
        };
        let key = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        let value = match &covered.energy_operands.lower_boundary {
            CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) => {
                stage3.optical.absorbed_w_m2_tile.total()
            }
            CoveredLowerBoundaryEnergyOperands::SnowFree(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered Stage-3 optical receipt for shortwave",
                ));
            }
        };
        if receipts.insert(key, value).is_some() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered shortwave destination",
            ));
        }
    }
    if receipts.is_empty() {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "empty covered shortwave destination set",
        ));
    }
    Ok(receipts)
}

impl CoveredV8PhysicalOwner {
    fn hydrology(&self) -> &UnifiedRealHydrologyCandidate {
        match self {
            Self::Legacy(value) => value.hydrology_candidate(),
            Self::MultiTile(value) => value.hydrology_candidate(),
            Self::FrozenLitterV3(value) => value,
        }
    }
}

impl UncommittedCoveredV8OwnerEnvelope {
    #[must_use]
    pub const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub const fn vegetation(&self) -> &UncommittedV8VegetationCandidate {
        &self.vegetation
    }

    #[must_use]
    pub fn hydrology(&self) -> &UnifiedRealHydrologyCandidate {
        self.physical.hydrology()
    }

    pub(crate) fn fixed_cap_canopy_releases_by_destination(
        &self,
        interval_s: f64,
    ) -> Result<FixedCapCanopyReleasesByDestination, CoveredV8OwnerEnvelopeError> {
        match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => {
                fixed_cap_canopy_releases_from_multi_tile(value, interval_s)
            }
            CoveredV8PhysicalOwner::Legacy(_) => Err(CoveredV8OwnerEnvelopeError::Identity(
                "fixed-cap release requires multi-tile physical owner",
            )),
            CoveredV8PhysicalOwner::FrozenLitterV3(_) => {
                Err(CoveredV8OwnerEnvelopeError::Identity(
                    "fixed-cap release is unavailable after V3 finalization",
                ))
            }
        }
    }

    pub(crate) fn covered_lse_iteration_state_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), CoveredLseIterationState>, CoveredV8OwnerEnvelopeError>
    {
        let physical = match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => value,
            CoveredV8PhysicalOwner::Legacy(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered iteration state requires multi-tile physical owner",
                ));
            }
            CoveredV8PhysicalOwner::FrozenLitterV3(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered iteration state is unavailable after V3 finalization",
                ));
            }
        };
        let mut states = BTreeMap::new();
        for tile in physical.finalized_tiles() {
            let Some(covered) = tile.covered() else {
                continue;
            };
            let lower = match &covered.energy_operands.lower_boundary {
                CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(value) => value,
                CoveredLowerBoundaryEnergyOperands::SnowFree(_) => continue,
            };
            let column = &covered.energy_operands.column;
            let component_temperatures_k = column
                .occupancies
                .iter()
                .map(|occupancy| {
                    (
                        occupancy.occupancy_id.clone(),
                        [
                            occupancy.sun_leaf.surface_temperature_k,
                            occupancy.shade_leaf.surface_temperature_k,
                            occupancy.wet_surface.surface_temperature_k,
                            occupancy.dry_stem.surface_temperature_k,
                        ],
                    )
                })
                .collect();
            let component_carrier_surfaces = column
                .occupancies
                .iter()
                .enumerate()
                .flat_map(|(vertical_ordinal, occupancy)| {
                    [
                        occupancy.sun_leaf,
                        occupancy.shade_leaf,
                        occupancy.wet_surface,
                        occupancy.dry_stem,
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(ordinal, surface)| CoveredCarrierComponentState {
                        vertical_occupancy_ordinal: vertical_ordinal as u32,
                        occupancy_id: occupancy.occupancy_id.clone(),
                        component_ordinal: ordinal as u8,
                        surface_area_m2_m2_tile: surface.surface_area_m2_m2_tile,
                        emissive_area_m2_m2_tile: surface.emissive_area_m2_m2_tile,
                        heat_conductance_m_s_tile: surface.heat_conductance_m_s_tile,
                        vapor_conductance_m_s_tile: surface.vapor_conductance_m_s_tile,
                        vapor_authorization_kg_m2_tile_s: surface.vapor_authorization_kg_m2_tile_s,
                        temperature_k: surface.surface_temperature_k,
                        specific_humidity_kg_kg: surface.surface_specific_humidity_kg_kg,
                        sensible_to_canopy_air_w_m2: surface.sensible_to_canopy_air_w_m2_tile,
                        vapor_to_canopy_air_kg_m2_s: surface
                            .signed_vapor_to_canopy_air_kg_m2_tile_s,
                    })
                    .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let key = (
                covered.identity.ofe_id.clone(),
                covered.identity.tile_id.clone(),
            );
            if states
                .insert(
                    key,
                    CoveredLseIterationState {
                        canopy_air_temperature_k: column.canopy_air.canopy_air_temperature_k,
                        canopy_air_specific_humidity_kg_kg: column
                            .canopy_air
                            .canopy_air_specific_humidity_kg_kg,
                        snow_temperature_k: lower.snow_temperature_k,
                        snow_sensible_w_m2: column
                            .canopy_air
                            .ground_sensible_to_canopy_air_w_m2_tile,
                        snow_vapor_kg_m2_s: column
                            .canopy_air
                            .ground_vapor_to_canopy_air_kg_m2_tile_s,
                        snow_latent_w_m2: column.canopy_air.ground_vapor_to_canopy_air_kg_m2_tile_s
                            * lower.latent_heat_j_kg,
                        snow_net_longwave_w_m2: column.longwave.ground_net_w_m2_tile,
                        component_temperatures_k,
                        component_carrier_surfaces,
                        canopy_sensible_w_m2: column.canopy_air.canopy_sensible_w_m2_tile,
                        canopy_vapor_kg_m2_s: column.canopy_air.canopy_vapor_kg_m2_tile_s,
                        sensible_to_reference_air_w_m2: column
                            .canopy_air
                            .sensible_to_reference_air_w_m2_tile,
                        vapor_to_reference_air_kg_m2_s: column
                            .canopy_air
                            .vapor_to_reference_air_kg_m2_tile_s,
                        shared_heat_residual_w_m2: column.canopy_air.shared_heat_residual_w_m2_tile,
                        shared_heat_tolerance_w_m2: column
                            .canopy_air
                            .shared_heat_tolerance_w_m2_tile,
                        shared_vapor_residual_kg_m2_s: column
                            .canopy_air
                            .shared_vapor_residual_kg_m2_tile_s,
                        shared_vapor_tolerance_kg_m2_s: column
                            .canopy_air
                            .shared_vapor_tolerance_kg_m2_tile_s,
                    },
                )
                .is_some()
            {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "duplicate covered iteration destination",
                ));
            }
        }
        Ok(states)
    }

    #[must_use]
    pub const fn biogeochemistry(&self) -> &BiogeochemistryOwnerCandidate {
        &self.biogeochemistry
    }

    pub(crate) fn covered_snow_longwave_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
        let physical = match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => value,
            CoveredV8PhysicalOwner::Legacy(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered longwave requires multi-tile physical owner",
                ));
            }
            CoveredV8PhysicalOwner::FrozenLitterV3(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered longwave is unavailable after V3 finalization",
                ));
            }
        };
        let mut receipts = BTreeMap::new();
        for tile in physical.finalized_tiles() {
            let Some(covered) = tile.covered() else {
                continue;
            };
            let key = (
                covered.identity.ofe_id.clone(),
                covered.identity.tile_id.clone(),
            );
            let value = covered.energy_operands.column.longwave.ground_net_w_m2_tile;
            if receipts.insert(key, value).is_some() {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "duplicate covered longwave destination",
                ));
            }
        }
        if receipts.is_empty() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "empty covered longwave destination set",
            ));
        }
        Ok(receipts)
    }

    pub(crate) fn covered_snow_shortwave_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
        let physical = match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => value,
            CoveredV8PhysicalOwner::Legacy(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered shortwave requires multi-tile physical owner",
                ));
            }
            CoveredV8PhysicalOwner::FrozenLitterV3(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered shortwave is unavailable after V3 finalization",
                ));
            }
        };
        let mut receipts = BTreeMap::new();
        for tile in physical.finalized_tiles() {
            let Some(covered) = tile.covered() else {
                continue;
            };
            let key = (
                covered.identity.ofe_id.clone(),
                covered.identity.tile_id.clone(),
            );
            let value = match &covered.energy_operands.lower_boundary {
                CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) => {
                    stage3.optical.absorbed_w_m2_tile.total()
                }
                CoveredLowerBoundaryEnergyOperands::SnowFree(_) => {
                    return Err(CoveredV8OwnerEnvelopeError::Identity(
                        "covered Stage-3 optical receipt for shortwave",
                    ));
                }
            };
            if receipts.insert(key, value).is_some() {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "duplicate covered shortwave destination",
                ));
            }
        }
        if receipts.is_empty() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "empty covered shortwave destination set",
            ));
        }
        Ok(receipts)
    }

    pub(crate) fn covered_snow_optical_by_destination(
        &self,
    ) -> Result<
        BTreeMap<(OfeId, TileId), Stage3SnowOpticalBoundaryReceiptV1>,
        CoveredV8OwnerEnvelopeError,
    > {
        let physical = match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => value,
            CoveredV8PhysicalOwner::Legacy(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered optical receipt requires multi-tile physical owner",
                ));
            }
            CoveredV8PhysicalOwner::FrozenLitterV3(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered optical receipt is unavailable after V3 finalization",
                ));
            }
        };
        let mut receipts = BTreeMap::new();
        for tile in physical.finalized_tiles() {
            let Some(covered) = tile.covered() else {
                continue;
            };
            let optical = match &covered.energy_operands.lower_boundary {
                CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) => {
                    stage3.optical.clone()
                }
                CoveredLowerBoundaryEnergyOperands::SnowFree(_) => {
                    return Err(CoveredV8OwnerEnvelopeError::Identity(
                        "covered optical receipt for snow-free payload",
                    ));
                }
            };
            let key = (
                covered.identity.ofe_id.clone(),
                covered.identity.tile_id.clone(),
            );
            if receipts.insert(key, optical).is_some() {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "duplicate covered optical receipt",
                ));
            }
        }
        if receipts.is_empty() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "empty covered optical receipt set",
            ));
        }
        Ok(receipts)
    }

    pub fn validate(&self) -> Result<(), CoveredV8OwnerEnvelopeError> {
        self.vegetation.validate_sealed()?;
        self.biogeochemistry.validate()?;
        validate_owner_transaction_identity(
            self.transaction_id,
            self.vegetation.transaction_id(),
            self.physical.hydrology().transaction_id(),
            self.biogeochemistry.transaction_id(),
        )?;
        compare_material_receipts(&self.vegetation, &self.biogeochemistry)
    }
}

fn validate_owner_transaction_identity(
    envelope: TransactionId,
    vegetation: TransactionId,
    physical: TransactionId,
    biogeochemistry: TransactionId,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    if envelope != vegetation || envelope != physical || envelope != biogeochemistry {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "heterogeneous transaction identity",
        ));
    }
    Ok(())
}

/// Construct BGC independently from the sealed vegetation protocol and join it
/// to the already executed physical owners. No water authorization, physical
/// solve, vegetation persistence, or owner mutation is reachable here.
#[allow(clippy::too_many_arguments)]
pub(crate) fn construct_covered_v8_owner_envelope(
    physical: CoveredForestShadowResult,
    bindings: &[V8ComponentOccupancyBinding],
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    let projected = project_covered_forest_v8_passes(
        &physical,
        bindings,
        vegetation_configuration,
        vegetation_beginning,
    )?;
    let persistent = execute_uncommitted_v8_persistent_phase(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        persistent_forcing,
        nitrogen,
    )?;
    let vegetation = construct_uncommitted_v8_vegetation_candidate(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        projected.final_state(),
        &persistent,
    )?;
    validate_actual_root_protocol(
        &physical.final_tile().vegetation_operands,
        physical.hydrology_candidate(),
    )?;
    join_covered_v8_owner_envelope(
        CoveredV8PhysicalOwner::Legacy(physical),
        vegetation,
        biogeochemistry_beginning,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn construct_multi_tile_v8_owner_envelope(
    physical: MultiTileRuntimeResult,
    projected: &V8CoveredProjection,
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
    failure_hook: OwnerFailureHook<'_>,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    let persistent = execute_uncommitted_v8_persistent_phase(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        persistent_forcing,
        nitrogen,
    )?;
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::Persistent)?;
    let vegetation = construct_uncommitted_v8_vegetation_candidate(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        projected.final_state(),
        &persistent,
    )?;
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::VegetationCandidate)?;
    join_covered_v8_owner_envelope(
        CoveredV8PhysicalOwner::MultiTile(physical),
        vegetation,
        biogeochemistry_beginning,
        failure_hook,
    )
}

/// V11-only owner construction over the authenticated common-slab duration.
#[allow(clippy::too_many_arguments)]
pub(crate) fn construct_multi_tile_v8_owner_envelope_v11(
    physical: MultiTileRuntimeResult,
    projected: &V8CoveredProjection,
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
    failure_hook: OwnerFailureHook<'_>,
    duration_s_bits: u64,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    let persistent = execute_uncommitted_v8_persistent_phase_v11(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        persistent_forcing,
        nitrogen,
        duration_s_bits,
    )?;
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::Persistent)?;
    let vegetation = construct_uncommitted_v8_vegetation_candidate(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        projected.final_state(),
        &persistent,
    )?;
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::VegetationCandidate)?;
    join_covered_v8_owner_envelope(
        CoveredV8PhysicalOwner::MultiTile(physical),
        vegetation,
        biogeochemistry_beginning,
        failure_hook,
    )
}

/// Complete the vegetation/BGC owner join from an already accepted native V3
/// physical transaction. This is projection-only: it cannot invoke an LSE
/// solve, water authorization, surface mutation, or WB14 execution.
#[allow(clippy::too_many_arguments)]
pub(crate) fn construct_frozen_litter_v3_owner_envelope_v11(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
    hydrology: UnifiedRealHydrologyCandidate,
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
    duration_s_bits: u64,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    let potentials = fixed
        .potential_vegetation_operands
        .iter()
        .collect::<Vec<_>>();
    let mut finals = fixed
        .legacy_tiles
        .iter()
        .filter_map(|tile| match tile {
            super::multi_tile_runtime::FinalizedRuntimeTile::Covered(value) => {
                Some(&value.vegetation_operands)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    finals.extend(fixed.frozen_litter_tiles.iter().map(|tile| {
        &tile
            .fixed_final
            .complete_physical_candidate
            .vegetation_operands
    }));
    let projected = project_multi_tile_v8_passes_v11(
        &potentials,
        &finals,
        &fixed.vegetation_bindings,
        &hydrology,
        vegetation_configuration,
        vegetation_beginning,
        duration_s_bits,
    )?;
    let persistent = execute_uncommitted_v8_persistent_phase_v11(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        persistent_forcing,
        nitrogen,
        duration_s_bits,
    )?;
    let vegetation = construct_uncommitted_v8_vegetation_candidate(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        projected.final_state(),
        &persistent,
    )?;
    join_covered_v8_owner_envelope(
        CoveredV8PhysicalOwner::FrozenLitterV3(hydrology),
        vegetation,
        biogeochemistry_beginning,
        None,
    )
}

fn join_covered_v8_owner_envelope(
    physical: CoveredV8PhysicalOwner,
    vegetation: UncommittedV8VegetationCandidate,
    biogeochemistry_beginning: &BiogeochemistryState,
    failure_hook: OwnerFailureHook<'_>,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    vegetation.validate_sealed()?;
    if vegetation.transaction_id() != physical.hydrology().transaction_id()
        || biogeochemistry_beginning.last_transaction_id.checked_add(1)
            != Some(vegetation.transaction_id().0)
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "beginning owner lineage",
        ));
    }
    let proposals = vegetation
        .material_proposals()
        .iter()
        .map(|value| MaterialProposal {
            transaction_id: value.transaction_id,
            owner_id: value.owner_id.clone(),
            donor: value.donor,
            receiver: value.receiver,
            proposal_id: value.proposal_id,
            amounts: MaterialPool {
                carbon: value.carbon,
                nitrogen: value.nitrogen,
                dry_matter: value.dry_matter,
            },
        })
        .collect::<Vec<_>>();
    let (requests, authorizations, uses) = vegetation.nitrogen_protocol();
    let biogeochemistry = construct_biogeochemistry_candidate(
        biogeochemistry_beginning,
        vegetation.transaction_id(),
        requests,
        authorizations,
        uses,
        &proposals,
        TransformationsMode::Disabled,
    )?;
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::BiogeochemistryCandidate)?;
    compare_material_receipts(&vegetation, &biogeochemistry)?;
    let envelope = UncommittedCoveredV8OwnerEnvelope {
        transaction_id: vegetation.transaction_id(),
        vegetation,
        physical,
        biogeochemistry,
    };
    envelope.validate()?;
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::EnvelopeValidation)?;
    Ok(envelope)
}

fn run_owner_failure_hook(
    hook: OwnerFailureHook<'_>,
    phase: V8OwnerFailurePhase,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    if let Some(hook) = hook {
        hook(phase)?;
    }
    Ok(())
}

fn validate_actual_root_protocol(
    source: &AcceptedCoveredVegetationOperands,
    hydrology: &UnifiedRealHydrologyCandidate,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    let requests = rows_by_key(&hydrology.arbitration().requests)?;
    let authorizations = authorization_rows_by_key(&hydrology.arbitration().authorizations)?;
    let uses = rows_by_key(hydrology.finalized_uses())?;
    let mut expected_keys = std::collections::BTreeSet::new();
    for occupancy in &source.occupancies {
        for root in &occupancy.root_water {
            let key = &root.key;
            expected_keys.insert(key.clone());
            if key.transaction_id != source.transaction_id
                || key.requesting_owner_id != source.vegetation_owner_id
                || key.requesting_component != RequestingComponent::VegetationRoot
                || key.ofe_id != source.ofe_id
                || key.requesting_tile_id != source.tile_id
                || key.occupancy_id.as_ref() != Some(&occupancy.occupancy_id)
                || key.soil_layer_id.is_none()
                || requests.get(key).map(|value| value.to_bits())
                    != Some(root.request_kg_m2_stand_ground.to_bits())
                || authorizations.get(key).map(|value| value.to_bits())
                    != Some(root.authorization_kg_m2_stand_ground.to_bits())
                || uses.get(key).map(|value| value.to_bits())
                    != Some(root.finalized_use_kg_m2_stand_ground.to_bits())
            {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "root D/A/F actual-owner correspondence",
                ));
            }
        }
    }
    let actual_root_keys = requests
        .keys()
        .filter(|key| {
            key.requesting_owner_id == source.vegetation_owner_id
                && key.requesting_component == RequestingComponent::VegetationRoot
                && key.ofe_id == source.ofe_id
                && key.requesting_tile_id == source.tile_id
        })
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let actual_authorization_keys = authorizations
        .keys()
        .filter(|key| actual_root_keys.contains(*key))
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let actual_use_keys = uses
        .keys()
        .filter(|key| actual_root_keys.contains(*key))
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    if expected_keys != actual_root_keys
        || expected_keys != actual_authorization_keys
        || expected_keys != actual_use_keys
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "root D/A/F actual-owner key set",
        ));
    }
    Ok(())
}

fn rows_by_key(
    rows: &[WaterAmount],
) -> Result<BTreeMap<GroundWaterKey, f64>, CoveredV8OwnerEnvelopeError> {
    let mut values = BTreeMap::new();
    for row in rows {
        if values
            .insert(row.key.clone(), row.amount_kg_m2_stand_ground)
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate actual hydrology water row",
            ));
        }
    }
    Ok(values)
}

fn authorization_rows_by_key(
    rows: &[openwepp_land_surface_energy::WaterAuthorization],
) -> Result<BTreeMap<GroundWaterKey, f64>, CoveredV8OwnerEnvelopeError> {
    let mut values = BTreeMap::new();
    for row in rows {
        if values
            .insert(row.key.clone(), row.amount_kg_m2_stand_ground)
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate actual hydrology authorization row",
            ));
        }
    }
    Ok(values)
}

fn compare_material_receipts(
    vegetation: &UncommittedV8VegetationCandidate,
    biogeochemistry: &BiogeochemistryOwnerCandidate,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    compare_material_receipt_rows(vegetation.material_proposals(), biogeochemistry.receipts())
}

fn compare_material_receipt_rows(
    proposals: &[MaterialTransfer],
    receipts: &[MaterialReceipt],
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    if proposals.len() != receipts.len()
        || proposals.iter().zip(receipts).any(|(proposal, receipt)| {
            proposal.transaction_id != receipt.transaction_id
                || proposal.owner_id != receipt.owner_id
                || proposal.donor != receipt.donor
                || proposal.receiver != receipt.receiver
                || proposal.proposal_id != receipt.proposal_id
                || proposal.carbon.to_bits() != receipt.amounts.carbon.to_bits()
                || proposal.nitrogen.to_bits() != receipt.amounts.nitrogen.to_bits()
                || proposal.dry_matter.to_bits() != receipt.amounts.dry_matter.to_bits()
        })
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "vegetation proposal/BGC receipt correspondence",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use openwepp_kernel_contract::{
        MaterialDonorClass, MaterialReceiverClass, ResourceOwnerId, SoilLayerId, TileId,
        TransactionId,
    };
    use openwepp_land_surface_energy::{
        ComponentId, OfeId, RequestingComponent, SourceId, StandGroundWaterAmountBasis,
        WaterSourceType,
    };

    use super::*;

    fn root_key() -> GroundWaterKey {
        GroundWaterKey {
            transaction_id: TransactionId(9),
            requesting_owner_id: ResourceOwnerId::try_new("vegetation-v8").expect("owner"),
            requesting_component: RequestingComponent::VegetationRoot,
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            requesting_tile_id: TileId::try_new("tile-1").expect("tile"),
            occupancy_id: Some(ComponentId::try_new("component-1").expect("component")),
            surface_id: None,
            surface_class: None,
            source_type: WaterSourceType::SoilLayerLiquid,
            source_id: SourceId::try_new("soil-1").expect("source"),
            source_tile_id: None,
            soil_layer_id: Some(SoilLayerId::try_new("soil-1").expect("layer")),
            amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        }
    }

    #[test]
    fn actual_owner_rows_preserve_complete_key_and_exact_amount() {
        let row = WaterAmount {
            key: root_key(),
            amount_kg_m2_stand_ground: 0.125,
        };
        let mapped = rows_by_key(std::slice::from_ref(&row)).expect("unique row");
        assert_eq!(mapped.len(), 1);
        assert_eq!(
            mapped[&row.key].to_bits(),
            row.amount_kg_m2_stand_ground.to_bits()
        );
    }

    #[test]
    fn duplicate_actual_owner_key_is_rejected_before_join() {
        let row = WaterAmount {
            key: root_key(),
            amount_kg_m2_stand_ground: 0.125,
        };
        assert_eq!(
            rows_by_key(&[row.clone(), row]),
            Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate actual hydrology water row"
            ))
        );
    }

    #[test]
    fn v50_envelope_transaction_join_refuses_each_owner_substitution() {
        let exact = TransactionId(42);
        validate_owner_transaction_identity(exact, exact, exact, exact)
            .expect("exact V50 envelope transaction join");
        for (label, envelope, vegetation, physical, biogeochemistry) in [
            ("envelope", TransactionId(41), exact, exact, exact),
            ("vegetation", exact, TransactionId(41), exact, exact),
            ("physical", exact, exact, TransactionId(41), exact),
            ("biogeochemistry", exact, exact, exact, TransactionId(41)),
        ] {
            assert_eq!(
                validate_owner_transaction_identity(
                    envelope,
                    vegetation,
                    physical,
                    biogeochemistry,
                ),
                Err(CoveredV8OwnerEnvelopeError::Identity(
                    "heterogeneous transaction identity"
                )),
                "{label} transaction substitution must refuse",
            );
        }
    }

    #[test]
    fn v50_envelope_material_receipt_substitution_refuses() {
        let owner = ResourceOwnerId::try_new("v50-envelope-owner").expect("owner");
        let proposal = MaterialTransfer {
            transaction_id: 42,
            owner_id: owner.clone(),
            proposal_id: 7,
            donor: MaterialDonorClass::Leaf,
            receiver: MaterialReceiverClass::Metabolic,
            carbon: 0.0048,
            nitrogen: 0.0001,
            dry_matter: 0.01,
        };
        let receipt = MaterialReceipt {
            transaction_id: proposal.transaction_id,
            owner_id: owner,
            donor: proposal.donor,
            receiver: proposal.receiver,
            proposal_id: proposal.proposal_id,
            amounts: MaterialPool {
                carbon: proposal.carbon,
                nitrogen: proposal.nitrogen,
                dry_matter: proposal.dry_matter,
            },
        };
        compare_material_receipt_rows(
            std::slice::from_ref(&proposal),
            std::slice::from_ref(&receipt),
        )
        .expect("exact V50 material receipt join");
        for (label, mut poison) in [
            ("transaction", receipt.clone()),
            ("owner", receipt.clone()),
            ("proposal", receipt.clone()),
            ("carbon", receipt.clone()),
        ] {
            match label {
                "transaction" => poison.transaction_id = 41,
                "owner" => {
                    poison.owner_id = ResourceOwnerId::try_new("foreign-owner").expect("owner")
                }
                "proposal" => poison.proposal_id = 8,
                _ => poison.amounts.carbon = f64::from_bits(poison.amounts.carbon.to_bits() + 1),
            }
            assert_eq!(
                compare_material_receipt_rows(
                    std::slice::from_ref(&proposal),
                    std::slice::from_ref(&poison),
                ),
                Err(CoveredV8OwnerEnvelopeError::Identity(
                    "vegetation proposal/BGC receipt correspondence"
                )),
                "{label} material receipt substitution must refuse",
            );
        }
    }
}
