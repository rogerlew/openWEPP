//! Phase-free V3 adoption for the heterogeneous multi-tile runtime.
//!
//! This seam deliberately ends before current ingress/WB14.  The caller must
//! first install the accepted litter phase through `v3_execution`, then pass
//! the derived ingress to that atomic V2-owner coordinator.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::TileId;
use openwepp_land_surface_energy::{
    BeginningLitterPhaseState, CoveredColumnInputs, GroundWaterKey, LandSurfaceEnergyConfiguration,
    LandSurfaceEnergyV3State, LitterPhaseConfiguration, OfeId, PotentialCoveredVegetationOperands,
    PotentialWaterRequestBatch, RequestingComponent, RootRuntimeIdentity, RuntimeTileIdentity,
    Sha256Digest, SurfaceClass, SurfaceConfiguration, V3CoveredPotentialPhase,
    V3FixedFinalCoveredCandidate, V3PhaseSpecificVaporAuthorization, WaterAuthorization,
    WaterAuthorizationReason, WaterProtocol, WaterSourceType,
    finalize_covered_phase_with_soil_thermal_beginning,
    finalize_open_phase_with_soil_thermal_beginning, finalize_v3_covered_phase,
    solve_covered_potential_phase, solve_open_potential_phase, solve_v3_covered_potential_phase,
};
use openwepp_vegetation::{
    V8ComponentOccupancyBinding, V8CoupledOwnedState, V8PersistentForcingReceipt,
    VegetationConfiguration,
};

use crate::{
    DirectSurfaceLiquidIngressInput, DirectTileGroundIngress, SurfaceLiquidConfigurationV2,
    SurfaceLiquidOwnerEnvelopeV2,
};

use super::covered_derived_ingress::derive_release_from_ledgers;
use super::multi_tile_runtime::{
    FinalizedRuntimeTile, PotentialTilePhase, StrictProjectedCoveredTile, StrictProjectedOpenTile,
    StrictProjectedStage3OpenSnowTile, StrictProjectedTileProblem,
};
use super::v3_input_projection::FrozenLitterV3PhaseFreeInput;
use super::v8_input_projection::{
    V8SolverReadyTileInput, V8SolverReadyTilePhysics, ValidatedV8RuntimeInputProjection,
};
use super::{
    CoveredIngressSchedule, DirectSurfaceLiquidConfiguration,
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError, MixedRealHydrologyRequest,
    RealHydrologySourceKey, V8SoilThermalPhysicalBeginning,
};

/// Solver-ready projection after every forest-litter row has been rebound to
/// the native LSE-V3/surface-V2 beginning. Non-litter rows retain their exact
/// predecessor representation.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StrictProjectedV3RuntimeInputs {
    pub(crate) tiles: Vec<StrictProjectedV3TileProblem>,
    pub(crate) soil_sources: BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    pub(crate) vegetation_bindings: Vec<V8ComponentOccupancyBinding>,
}

/// A forest-litter tile whose potential and fixed-final evaluations must both
/// use the V3 phase-free residual.  There is no legacy-litter representation.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StrictProjectedV3ForestLitterTile {
    pub identity: RuntimeTileIdentity,
    pub beginning: CoveredColumnInputs,
    pub roots: Vec<RootRuntimeIdentity>,
    pub potential_initial_trial: Vec<f64>,
    pub final_initial_trial: Vec<f64>,
    pub soil_thermal: V8SoilThermalPhysicalBeginning,
    pub litter_configuration: LitterPhaseConfiguration,
    pub litter_beginning: BeginningLitterPhaseState,
}

/// Exact configured member of a V3-capable heterogeneous tile set.
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum StrictProjectedV3TileProblem {
    Legacy(StrictProjectedTileProblem),
    FrozenForestLitter(StrictProjectedV3ForestLitterTile),
}

impl StrictProjectedV3TileProblem {
    fn identity(&self) -> &RuntimeTileIdentity {
        match self {
            Self::Legacy(value) => match value {
                StrictProjectedTileProblem::Open(value) => &value.identity,
                StrictProjectedTileProblem::Stage3OpenSnow(value) => &value.identity,
                StrictProjectedTileProblem::Covered(value) => &value.identity,
            },
            Self::FrozenForestLitter(value) => &value.identity,
        }
    }
}

fn find_surface_v2_records<'a>(
    identity: &RuntimeTileIdentity,
    configuration: &'a SurfaceLiquidConfigurationV2,
    owner: &'a SurfaceLiquidOwnerEnvelopeV2,
) -> Result<
    (
        &'a crate::SurfaceLiquidConfigurationRecordV2,
        &'a crate::SurfaceLiquidStateRecordV2,
    ),
    LandSurfaceEnergyShadowError,
> {
    let state = owner
        .v2_state()
        .ok_or(LandSurfaceEnergyShadowError::UnsupportedCustody(
            "frozen litter requires a native surface V2 owner",
        ))?;
    let configured = configuration
        .records()
        .iter()
        .find(|row| {
            row.key.ofe_id == identity.ofe_id
                && row.key.tile_id == identity.tile_id
                && row.key.surface_id == identity.surface_id
        })
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "V3 surface configuration tile",
        ))?;
    let beginning = state
        .records()
        .iter()
        .find(|row| row.key == configured.key)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "V3 surface beginning tile",
        ))?;
    Ok((configured, beginning))
}

fn replace_ground_temperature_trial(
    mut trial: Vec<f64>,
    soil_node_count: usize,
    temperature_k: f64,
) -> Result<Vec<f64>, LandSurfaceEnergyShadowError> {
    let index = trial.len().checked_sub(soil_node_count + 1).ok_or(
        LandSurfaceEnergyShadowError::Identity("V3 covered warm-start layout"),
    )?;
    let value = trial
        .get_mut(index)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "V3 covered ground warm start",
        ))?;
    *value = temperature_k;
    Ok(trial)
}

/// Rebind the already validated V8 structural projection to the exact native
/// V3 beginnings. This function performs no solve and has no V3-to-legacy
/// downgrade arm.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn project_native_frozen_litter_v3_solver_inputs(
    projection: &ValidatedV8RuntimeInputProjection,
    vegetation_owner_id: &openwepp_kernel_contract::ResourceOwnerId,
    authority: openwepp_land_surface_energy::CoveredColumnAuthority,
    lower_boundaries: Option<
        &BTreeMap<(OfeId, TileId), openwepp_land_surface_energy::Stage3SnowCoveredLowerBoundary>,
    >,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    lse_beginning: &LandSurfaceEnergyV3State,
    surface_configuration: &SurfaceLiquidConfigurationV2,
    surface_beginning: &SurfaceLiquidOwnerEnvelopeV2,
) -> Result<StrictProjectedV3RuntimeInputs, LandSurfaceEnergyShadowError> {
    lse_beginning
        .validate(lse_configuration)
        .map_err(|_| LandSurfaceEnergyShadowError::Identity("native V3 LSE configuration/state"))?;
    surface_beginning
        .canonical_bytes(surface_configuration.parent(), Some(surface_configuration))?;
    let solver_ready = projection
        .solver_ready_tiles_with_authority_and_lower_boundaries(
            vegetation_owner_id,
            authority,
            lower_boundaries,
        )
        .map_err(|_| LandSurfaceEnergyShadowError::Identity("native V3 solver-ready projection"))?;
    let mut tiles = Vec::with_capacity(solver_ready.len());
    let mut soil_sources = BTreeMap::new();
    let mut vegetation_bindings = Vec::new();
    for tile in solver_ready {
        for (key, source) in tile.soil_sources {
            if soil_sources.insert(key, source).is_some() {
                return Err(LandSurfaceEnergyShadowError::Identity(
                    "duplicate V3 projected soil source",
                ));
            }
        }
        let V8SolverReadyTileInput {
            mut identity,
            physics,
            root_identities,
            soil_sources: _,
            beginning_trial,
            vegetation_bindings: tile_vegetation_bindings,
            soil_thermal,
        } = tile;
        vegetation_bindings.extend(tile_vegetation_bindings);
        // The validated predecessor projection supplies only structural and
        // forcing operands. Every solver identity is rebound to the native V3
        // configuration/state before either potential or fixed-final solve.
        identity.configuration_sha256 = lse_configuration.configuration_sha256.clone();
        identity.beginning_lse_state_sha256 = lse_beginning.0.state_sha256.clone();
        let projected = match (identity.surface_class, physics) {
            (SurfaceClass::ForestLitter, V8SolverReadyTilePhysics::Covered(mut beginning)) => {
                let configured = lse_configuration
                    .ofes
                    .iter()
                    .find(|ofe| ofe.ofe_id == identity.ofe_id)
                    .and_then(|ofe| {
                        ofe.tiles
                            .iter()
                            .find(|configured| configured.tile_id == identity.tile_id)
                    })
                    .ok_or(LandSurfaceEnergyShadowError::Identity(
                        "V3 LSE forest-litter configuration tile",
                    ))?;
                let SurfaceConfiguration::ForestLitter {
                    liquid_capacity_kg_m2_tile_ground,
                    thickness_m,
                    dry_density_kg_m3,
                    dry_specific_heat_j_kg_k,
                } = configured.surface
                else {
                    return Err(LandSurfaceEnergyShadowError::Identity(
                        "V3 runtime class/configuration forest-litter join",
                    ));
                };
                let (surface_configured, surface_state) =
                    find_surface_v2_records(&identity, surface_configuration, surface_beginning)?;
                let lse_state = lse_beginning
                    .0
                    .tiles
                    .iter()
                    .find(|state| {
                        state.ofe_id == identity.ofe_id && state.tile_id == identity.tile_id
                    })
                    .ok_or(LandSurfaceEnergyShadowError::Identity(
                        "V3 LSE forest-litter beginning tile",
                    ))?;
                let ice_capacity = surface_configured.litter_ice_capacity_kg_m2_tile.ok_or(
                    LandSurfaceEnergyShadowError::Identity("V3 litter ice capacity"),
                )?;
                let surface_depth = surface_configured
                    .litter_depth_m
                    .ok_or(LandSurfaceEnergyShadowError::Identity("V3 litter depth"))?;
                let dry_heat_capacity = thickness_m * dry_density_kg_m3 * dry_specific_heat_j_kg_k;
                if surface_depth.to_bits() != thickness_m.to_bits()
                    || lse_state.surface_enthalpy_j_m2_tile_ground.to_bits()
                        != surface_state.surface_enthalpy_j_m2_tile.to_bits()
                    || surface_configured.key.surface_class != SurfaceClass::ForestLitter
                    || surface_configured.key.source_type != WaterSourceType::LitterLiquid
                {
                    return Err(LandSurfaceEnergyShadowError::Identity(
                        "V3 native litter beginning/configuration join",
                    ));
                }
                beginning.ground.surface_liquid_kg_m2_tile = surface_state.liquid_kg_m2_tile;
                beginning.ground.surface_enthalpy_j_m2_tile =
                    surface_state.surface_enthalpy_j_m2_tile;
                beginning.ground.surface_temperature_warm_start_k =
                    lse_state.surface_temperature_warm_start_k;
                beginning.ground.surface_conductivity_w_m_k =
                    super::v8_input_projection::project_forest_litter_conductivity(
                        surface_state.liquid_kg_m2_tile,
                        thickness_m,
                    )
                    .map_err(|_| {
                        LandSurfaceEnergyShadowError::Operand(
                            "native V3 forest-litter conductivity",
                        )
                    })?;
                let beginning_trial = replace_ground_temperature_trial(
                    beginning_trial,
                    beginning.ground.soil_nodes.len(),
                    lse_state.surface_temperature_warm_start_k,
                )?;
                StrictProjectedV3TileProblem::FrozenForestLitter(
                    StrictProjectedV3ForestLitterTile {
                        identity,
                        beginning,
                        roots: root_identities,
                        potential_initial_trial: beginning_trial.clone(),
                        final_initial_trial: beginning_trial,
                        soil_thermal,
                        litter_configuration: LitterPhaseConfiguration {
                            litter_depth_m: thickness_m,
                            dry_heat_capacity_j_m2_k: dry_heat_capacity,
                            liquid_capacity_kg_m2_tile: liquid_capacity_kg_m2_tile_ground,
                            ice_capacity_kg_m2_tile: ice_capacity,
                        },
                        litter_beginning: BeginningLitterPhaseState {
                            liquid_kg_m2_tile: surface_state.liquid_kg_m2_tile,
                            ice_kg_m2_tile: surface_state.litter_ice_kg_m2_tile,
                            sensible_energy_j_m2_tile: surface_state.surface_enthalpy_j_m2_tile,
                            temperature_k: lse_state.surface_temperature_warm_start_k,
                        },
                    },
                )
            }
            (SurfaceClass::ForestLitter, _) => {
                return Err(LandSurfaceEnergyShadowError::UnsupportedCustody(
                    "forest litter must enter the native V3 covered solver",
                ));
            }
            (_, V8SolverReadyTilePhysics::Open(beginning)) => StrictProjectedV3TileProblem::Legacy(
                StrictProjectedTileProblem::Open(StrictProjectedOpenTile {
                    identity,
                    beginning,
                    potential_initial_trial: Some(beginning_trial.clone()),
                    final_initial_trial: Some(beginning_trial),
                    soil_thermal,
                }),
            ),
            (_, V8SolverReadyTilePhysics::Stage3OpenSnow(beginning_state)) => {
                StrictProjectedV3TileProblem::Legacy(StrictProjectedTileProblem::Stage3OpenSnow(
                    StrictProjectedStage3OpenSnowTile {
                        identity,
                        beginning_state,
                        soil_thermal,
                    },
                ))
            }
            (_, V8SolverReadyTilePhysics::Covered(beginning)) => {
                StrictProjectedV3TileProblem::Legacy(StrictProjectedTileProblem::Covered(
                    StrictProjectedCoveredTile {
                        identity,
                        beginning,
                        roots: root_identities,
                        potential_initial_trial: beginning_trial.clone(),
                        final_initial_trial: beginning_trial,
                        soil_thermal,
                    },
                ))
            }
        };
        tiles.push(projected);
    }
    Ok(StrictProjectedV3RuntimeInputs {
        tiles,
        soil_sources,
        vegetation_bindings,
    })
}

#[derive(Clone, Debug, PartialEq)]
enum V3PotentialTile {
    Legacy(PotentialTilePhase),
    FrozenForestLitter {
        phase: V3CoveredPotentialPhase,
        final_initial_trial: Vec<f64>,
        soil_thermal: V8SoilThermalPhysicalBeginning,
        litter_configuration: LitterPhaseConfiguration,
        litter_beginning: BeginningLitterPhaseState,
        covered_beginning: CoveredColumnInputs,
        root_identities: Vec<RootRuntimeIdentity>,
        occupancy_ids: Vec<String>,
    },
}

impl V3PotentialTile {
    fn identity(&self) -> &RuntimeTileIdentity {
        match self {
            Self::Legacy(value) => match value {
                PotentialTilePhase::Open { phase, .. } => &phase.identity,
                PotentialTilePhase::Stage3OpenSnow { identity, .. } => identity,
                PotentialTilePhase::Covered { phase, .. } => phase.identity(),
            },
            Self::FrozenForestLitter { phase, .. } => phase.identity(),
        }
    }

    fn request_batch(&self) -> &PotentialWaterRequestBatch {
        match self {
            Self::Legacy(PotentialTilePhase::Open { phase, .. }) => &phase.request_batch,
            Self::Legacy(PotentialTilePhase::Stage3OpenSnow { request_batch, .. }) => request_batch,
            Self::Legacy(PotentialTilePhase::Covered { phase, .. }) => phase.request_batch(),
            Self::FrozenForestLitter { phase, .. } => phase.request_batch(),
        }
    }
}

/// Uncommitted potential/request transaction.  Fields remain sealed so a
/// caller cannot replace a potential phase between authorization and final.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V3MultiTilePotentialCandidate {
    request_batch: PotentialWaterRequestBatch,
    tiles: Vec<V3PotentialTile>,
}

impl V3MultiTilePotentialCandidate {
    #[must_use]
    pub(crate) const fn request_batch(&self) -> &PotentialWaterRequestBatch {
        &self.request_batch
    }
}

fn surface_v2_state_for_request<'a>(
    request: &openwepp_land_surface_energy::WaterAmount,
    configuration: &'a SurfaceLiquidConfigurationV2,
    owner: &'a SurfaceLiquidOwnerEnvelopeV2,
) -> Result<
    (
        &'a crate::DirectSurfaceLiquidConfigurationRecord,
        &'a crate::SurfaceLiquidStateRecordV2,
    ),
    LandSurfaceEnergyShadowError,
> {
    let key = &request.key;
    if key.requesting_component != RequestingComponent::GroundSurface
        || key.occupancy_id.is_some()
        || key.soil_layer_id.is_some()
        || key.source_tile_id.as_ref() != Some(&key.requesting_tile_id)
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "V3 ground request identity",
        ));
    }
    let configured = configuration
        .parent()
        .records
        .iter()
        .find(|row| {
            row.key.ofe_id == key.ofe_id
                && row.key.tile_id == key.requesting_tile_id
                && Some(&row.key.surface_id) == key.surface_id.as_ref()
                && Some(row.key.surface_class) == key.surface_class
                && row.key.source_type == key.source_type
                && row.key.source_id == key.source_id
        })
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "V3 ground request surface store",
        ))?;
    let state = owner
        .v2_state()
        .ok_or(LandSurfaceEnergyShadowError::UnsupportedCustody(
            "V3 authorization requires native surface V2",
        ))?
        .records()
        .iter()
        .find(|row| row.key == configured.key)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "V3 ground request surface beginning",
        ))?;
    Ok((configured, state))
}

fn authorization_reason(request: f64, authorization: f64, supply: f64) -> WaterAuthorizationReason {
    if request == 0.0 {
        WaterAuthorizationReason::ZeroSupply
    } else if authorization.to_bits() == request.to_bits() {
        WaterAuthorizationReason::FullSupply
    } else if supply == 0.0 {
        WaterAuthorizationReason::DrySource
    } else {
        WaterAuthorizationReason::ProportionalSupply
    }
}

/// Authorize the immutable potential batch against the actual production-soil
/// adapter and the native phase-resolved surface V2 owner. Named litter phases
/// remain separate until their exact aggregate protocol amount is constructed.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn authorize_v3_multitile_potential(
    potential: &V3MultiTilePotentialCandidate,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    surface_configuration: &SurfaceLiquidConfigurationV2,
    surface_beginning: &SurfaceLiquidOwnerEnvelopeV2,
) -> Result<
    (
        Vec<WaterAuthorization>,
        BTreeMap<(OfeId, TileId), V3PhaseSpecificVaporAuthorization>,
        super::MixedRealHydrologyArbitration,
    ),
    LandSurfaceEnergyShadowError,
> {
    surface_beginning
        .canonical_bytes(surface_configuration.parent(), Some(surface_configuration))?;
    potential.request_batch.validate()?;
    let frozen = potential
        .tiles
        .iter()
        .filter_map(|tile| match tile {
            V3PotentialTile::FrozenForestLitter { phase, .. } => Some((
                (
                    phase.identity().ofe_id.clone(),
                    phase.identity().tile_id.clone(),
                ),
                phase,
            )),
            V3PotentialTile::Legacy(_) => None,
        })
        .collect::<BTreeMap<_, _>>();
    let mut soil_requests = Vec::new();
    let mut authorizations = BTreeMap::<GroundWaterKey, WaterAuthorization>::new();
    let mut phase_authorizations = BTreeMap::new();
    for request in &potential.request_batch.requests {
        if request.key.source_type == WaterSourceType::SoilLayerLiquid {
            let source = soil_sources.get(&request.key).cloned().ok_or(
                LandSurfaceEnergyShadowError::Identity("missing V3 projected soil source"),
            )?;
            soil_requests.push(MixedRealHydrologyRequest {
                request: request.clone(),
                source,
            });
            continue;
        }
        let (configured, state) =
            surface_v2_state_for_request(request, surface_configuration, surface_beginning)?;
        let frozen_key = (
            request.key.ofe_id.clone(),
            request.key.requesting_tile_id.clone(),
        );
        let (amount, supply) = if let Some(phase) = frozen.get(&frozen_key) {
            if request.key.source_type != WaterSourceType::LitterLiquid
                || configured.key.surface_class != SurfaceClass::ForestLitter
            {
                return Err(LandSurfaceEnergyShadowError::Identity(
                    "V3 phase authorization surface class/source",
                ));
            }
            let interval_s = phase.identity().interval_s;
            let potential_vapor = phase.accepted().evaluation.vapor.finalized;
            let liquid_potential = potential_vapor.liquid_signed_rate_kg_m2_s.max(0.0);
            let ice_potential = potential_vapor.ice_signed_rate_kg_m2_s.max(0.0);
            let authorization = V3PhaseSpecificVaporAuthorization {
                liquid_outbound_rate_kg_m2_s: liquid_potential
                    .min(state.liquid_kg_m2_tile / interval_s),
                ice_outbound_rate_kg_m2_s: ice_potential
                    .min(state.litter_ice_kg_m2_tile / interval_s),
            };
            let amount = authorization.aggregate_outbound_kg_m2_stand_ground(
                phase.identity().tile_fraction,
                interval_s,
            )?;
            if amount > request.amount_kg_m2_stand_ground
                || phase_authorizations
                    .insert(frozen_key, authorization)
                    .is_some()
            {
                return Err(LandSurfaceEnergyShadowError::Bound(
                    "V3 named phase authorization/request join",
                ));
            }
            let supply =
                (state.liquid_kg_m2_tile + state.litter_ice_kg_m2_tile) * configured.tile_fraction;
            (amount, supply)
        } else {
            let supply = state.liquid_kg_m2_tile * configured.tile_fraction;
            (request.amount_kg_m2_stand_ground.min(supply), supply)
        };
        if !amount.is_finite() || amount < 0.0 || !supply.is_finite() || supply < 0.0 {
            return Err(LandSurfaceEnergyShadowError::Operand(
                "V3 surface authorization domain",
            ));
        }
        let authorization = WaterAuthorization {
            key: request.key.clone(),
            amount_kg_m2_stand_ground: amount,
            reason: authorization_reason(request.amount_kg_m2_stand_ground, amount, supply),
        };
        if authorizations
            .insert(request.key.clone(), authorization)
            .is_some()
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "duplicate V3 surface authorization",
            ));
        }
    }
    let soil = soil_adapter.authorize(&soil_requests)?;
    for row in &soil.authorizations {
        if authorizations
            .insert(row.authorization.key.clone(), row.authorization.clone())
            .is_some()
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "duplicate V3 soil authorization",
            ));
        }
    }
    let ordered = potential
        .request_batch
        .requests
        .iter()
        .map(|request| {
            authorizations
                .remove(&request.key)
                .ok_or(LandSurfaceEnergyShadowError::Identity(
                    "missing ordered V3 authorization",
                ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if !authorizations.is_empty() || phase_authorizations.len() != frozen.len() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "V3 authorization cardinality",
        ));
    }
    Ok((ordered, phase_authorizations, soil))
}

/// Fixed-final V3 tile retained with the exact inputs needed by the atomic
/// post-solve phase coordinator and the soil carrier needed by the later join.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AcceptedV3ForestLitterTile {
    pub fixed_final: V3FixedFinalCoveredCandidate,
    pub phase_free_input: FrozenLitterV3PhaseFreeInput,
    pub soil_thermal: V8SoilThermalPhysicalBeginning,
    pub covered_beginning: CoveredColumnInputs,
    pub root_identities: Vec<RootRuntimeIdentity>,
    pub occupancy_ids: Vec<String>,
}

/// Typed pre-ingress boundary.  No surface resource, current ingress, WB14,
/// phase transfer, owner mutation, or persisted diagnostic is constructed.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V3MultiTileAcceptedFixedFinalCandidate {
    pub legacy_tiles: Vec<FinalizedRuntimeTile>,
    pub frozen_litter_tiles: Vec<AcceptedV3ForestLitterTile>,
    pub water_protocol: WaterProtocol,
    pub soil_arbitration: super::MixedRealHydrologyArbitration,
    pub receiver_expectations: super::UnifiedReceiverExpectations,
    pub soil_sources: BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    pub potential_vegetation_operands: Vec<PotentialCoveredVegetationOperands>,
    pub vegetation_bindings: Vec<V8ComponentOccupancyBinding>,
    pub vegetation_configuration: VegetationConfiguration,
    pub vegetation_beginning: V8CoupledOwnedState,
    pub persistent_forcing: V8PersistentForcingReceipt,
    pub derived_current_ingress: DirectSurfaceLiquidIngressInput,
}

impl V3MultiTileAcceptedFixedFinalCandidate {
    pub(crate) fn unified_finalization(
        &self,
        ending_lse_state: &LandSurfaceEnergyV3State,
    ) -> Result<super::UnifiedLseFinalization, LandSurfaceEnergyShadowError> {
        let mut soil_thermal = Vec::new();
        let mut rollback_sets = Vec::new();
        for tile in &self.legacy_tiles {
            match tile {
                FinalizedRuntimeTile::Open(value) => {
                    soil_thermal.push(value.soil_thermal.clone());
                    rollback_sets.push(value.rollback_hashes.clone());
                }
                FinalizedRuntimeTile::Stage3OpenSnow {
                    soil_thermal: soil,
                    rollback_hashes,
                    ..
                } => {
                    soil_thermal.push(soil.clone());
                    rollback_sets.push(rollback_hashes.clone());
                }
                FinalizedRuntimeTile::Covered(value) => {
                    soil_thermal.push(value.soil_thermal.clone());
                    rollback_sets.push(value.rollback_hashes.clone());
                }
            }
        }
        for tile in &self.frozen_litter_tiles {
            soil_thermal.push(
                tile.fixed_final
                    .complete_physical_candidate
                    .soil_thermal
                    .clone(),
            );
            rollback_sets.push(
                tile.fixed_final
                    .complete_physical_candidate
                    .rollback_hashes
                    .clone(),
            );
        }
        let rank = ending_lse_state
            .0
            .tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| ((tile.ofe_id.clone(), tile.tile_id.clone()), index))
            .collect::<BTreeMap<_, _>>();
        soil_thermal.sort_by_key(|tile| {
            rank.get(&(tile.ofe_id.clone(), tile.tile_id.clone()))
                .copied()
                .unwrap_or(usize::MAX)
        });
        if soil_thermal.len() != rank.len()
            || soil_thermal
                .iter()
                .any(|tile| !rank.contains_key(&(tile.ofe_id.clone(), tile.tile_id.clone())))
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "V3 unified soil/LSE topology",
            ));
        }
        let selected = |rows: &[openwepp_land_surface_energy::OwnerRollbackHash]| {
            rows.iter()
                .filter(|row| {
                    matches!(
                        row.owner_kind,
                        openwepp_land_surface_energy::OwnerKind::LandSurfaceEnergy
                            | openwepp_land_surface_energy::OwnerKind::Hydrology
                            | openwepp_land_surface_energy::OwnerKind::SoilThermal
                    )
                })
                .cloned()
                .collect::<Vec<_>>()
        };
        let first = rollback_sets.first().map(|rows| selected(rows)).ok_or(
            LandSurfaceEnergyShadowError::Identity("empty V3 rollback tile set"),
        )?;
        if first.len() != 3
            || rollback_sets
                .iter()
                .skip(1)
                .any(|rows| selected(rows) != first)
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "V3 multi-tile owner rollback identity",
            ));
        }
        super::UnifiedLseFinalization::try_new(
            &self.receiver_expectations,
            self.water_protocol.clone(),
            ending_lse_state.0.tiles.clone(),
            soil_thermal,
            first,
        )
    }
}

fn validate_topology(
    projected: &mut [StrictProjectedV3TileProblem],
    configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<(), LandSurfaceEnergyShadowError> {
    configuration.validate()?;
    if projected.is_empty() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "empty V3 projected tile set",
        ));
    }
    let rank = configuration
        .records
        .iter()
        .enumerate()
        .map(|(index, row)| ((row.key.ofe_id.clone(), row.key.tile_id.clone()), index))
        .collect::<BTreeMap<_, _>>();
    projected.sort_by_key(|tile| {
        rank.get(&(
            tile.identity().ofe_id.clone(),
            tile.identity().tile_id.clone(),
        ))
        .copied()
        .unwrap_or(usize::MAX)
    });
    let first = projected[0].identity().clone();
    let mut identities = BTreeSet::new();
    let mut fractions = BTreeMap::<OfeId, f64>::new();
    for tile in projected.iter() {
        let identity = tile.identity();
        let configured = configuration
            .records
            .iter()
            .find(|row| row.key.ofe_id == identity.ofe_id && row.key.tile_id == identity.tile_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "V3 projected tile absent from topology",
            ))?;
        if identity.transaction_id != first.transaction_id
            || identity.configuration_sha256 != first.configuration_sha256
            || identity.beginning_lse_state_sha256 != first.beginning_lse_state_sha256
            || identity.beginning_hydrology_snapshot_sha256
                != first.beginning_hydrology_snapshot_sha256
            || identity.beginning_soil_thermal_state_sha256
                != first.beginning_soil_thermal_state_sha256
            || identity.interval_s.to_bits() != first.interval_s.to_bits()
            || identity.interval_s < 60.0
            || (identity.interval_s / 60.0).fract() != 0.0
            || identity.tile_fraction.to_bits() != configured.tile_fraction.to_bits()
            || identity.surface_id != configured.key.surface_id
            || !identities.insert((identity.ofe_id.clone(), identity.tile_id.clone()))
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "mixed, off-grid, or duplicate V3 projected lineage",
            ));
        }
        match tile {
            StrictProjectedV3TileProblem::FrozenForestLitter(value) => {
                if identity.surface_class != SurfaceClass::ForestLitter
                    || value.beginning.ground.class
                        != openwepp_land_surface_energy::SurfaceClassKind::ForestLitter
                    || value.litter_beginning.liquid_kg_m2_tile.to_bits()
                        != value.beginning.ground.surface_liquid_kg_m2_tile.to_bits()
                {
                    return Err(LandSurfaceEnergyShadowError::Identity(
                        "V3 forest-litter projection identity",
                    ));
                }
            }
            StrictProjectedV3TileProblem::Legacy(_)
                if identity.surface_class == SurfaceClass::ForestLitter =>
            {
                return Err(LandSurfaceEnergyShadowError::UnsupportedCustody(
                    "forest litter cannot enter the legacy multi-tile solve",
                ));
            }
            StrictProjectedV3TileProblem::Legacy(_) => {}
        }
        *fractions.entry(identity.ofe_id.clone()).or_default() += identity.tile_fraction;
    }
    let configured = configuration
        .records
        .iter()
        .map(|row| (row.key.ofe_id.clone(), row.key.tile_id.clone()))
        .collect::<BTreeSet<_>>();
    if configured != identities
        || fractions
            .values()
            .any(|sum| !openwepp_land_surface_energy::canonical_tile_fraction_sum_closes(*sum))
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "V3 projected topology or OFE fraction closure",
        ));
    }
    Ok(())
}

/// Execute exactly one potential solve per tile from immutable beginnings.
pub(crate) fn prepare_v3_multitile_potential(
    configuration: &DirectSurfaceLiquidConfiguration,
    mut projected: Vec<StrictProjectedV3TileProblem>,
) -> Result<V3MultiTilePotentialCandidate, LandSurfaceEnergyShadowError> {
    validate_topology(&mut projected, configuration)?;
    let mut tiles = Vec::with_capacity(projected.len());
    for tile in projected {
        let potential = match tile {
            StrictProjectedV3TileProblem::Legacy(StrictProjectedTileProblem::Open(value)) => {
                V3PotentialTile::Legacy(PotentialTilePhase::Open {
                    phase: solve_open_potential_phase(
                        value.identity,
                        &value.beginning,
                        value.potential_initial_trial,
                    )?,
                    final_initial_trial: value.final_initial_trial,
                    soil_thermal: value.soil_thermal,
                })
            }
            StrictProjectedV3TileProblem::Legacy(StrictProjectedTileProblem::Stage3OpenSnow(_)) => {
                return Err(LandSurfaceEnergyShadowError::UnsupportedCustody(
                    "Stage-3 snow is outside the frozen-litter V3 seam",
                ));
            }
            StrictProjectedV3TileProblem::Legacy(StrictProjectedTileProblem::Covered(value)) => {
                V3PotentialTile::Legacy(PotentialTilePhase::Covered {
                    phase: solve_covered_potential_phase(
                        value.identity,
                        &value.beginning,
                        value.roots,
                        value.potential_initial_trial,
                    )?,
                    final_initial_trial: value.final_initial_trial,
                    soil_thermal: value.soil_thermal,
                })
            }
            StrictProjectedV3TileProblem::FrozenForestLitter(value) => {
                let covered_beginning = value.beginning.clone();
                let root_identities = value.roots.clone();
                let occupancy_ids = value
                    .beginning
                    .occupancies
                    .iter()
                    .map(|row| row.occupancy_id.clone())
                    .collect();
                V3PotentialTile::FrozenForestLitter {
                    phase: solve_v3_covered_potential_phase(
                        value.identity,
                        &value.beginning,
                        value.roots,
                        &value.potential_initial_trial,
                        value.litter_configuration,
                        value.litter_beginning,
                    )?,
                    final_initial_trial: value.final_initial_trial,
                    soil_thermal: value.soil_thermal,
                    litter_configuration: value.litter_configuration,
                    litter_beginning: value.litter_beginning,
                    covered_beginning,
                    root_identities,
                    occupancy_ids,
                }
            }
        };
        tiles.push(potential);
    }
    let first = tiles.first().ok_or(LandSurfaceEnergyShadowError::Identity(
        "empty V3 potential tile set",
    ))?;
    let requests = tiles
        .iter()
        .flat_map(|tile| tile.request_batch().requests.iter().cloned())
        .collect();
    let request_batch = PotentialWaterRequestBatch::try_new(
        first.identity().transaction_id,
        first.identity().beginning_lse_state_sha256.clone(),
        requests,
    )?;
    Ok(V3MultiTilePotentialCandidate {
        request_batch,
        tiles,
    })
}

fn authorization_subset(
    batch: &PotentialWaterRequestBatch,
    authorizations: &[WaterAuthorization],
) -> Result<Vec<WaterAuthorization>, LandSurfaceEnergyShadowError> {
    let keys = batch
        .requests
        .iter()
        .map(|row| row.key.clone())
        .collect::<BTreeSet<GroundWaterKey>>();
    let subset = authorizations
        .iter()
        .filter(|row| keys.contains(&row.key))
        .cloned()
        .collect::<Vec<_>>();
    if subset.len() != keys.len() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "incomplete V3 tile authorization subset",
        ));
    }
    Ok(subset)
}

fn combined_protocol(
    batch: &PotentialWaterRequestBatch,
    authorizations: Vec<WaterAuthorization>,
    protocols: &[&WaterProtocol],
) -> Result<WaterProtocol, LandSurfaceEnergyShadowError> {
    let first = protocols
        .first()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "empty V3 fixed-final protocol set",
        ))?;
    let protocol = WaterProtocol {
        transaction_id: batch.transaction_id,
        hydrology_owner_id: first.hydrology_owner_id.clone(),
        beginning_snapshot_sha256: first.beginning_snapshot_sha256.clone(),
        requests: batch.requests.clone(),
        authorizations,
        finalized_uses: protocols
            .iter()
            .flat_map(|row| row.finalized_uses.iter().cloned())
            .collect(),
        condensation_credits: protocols
            .iter()
            .flat_map(|row| row.condensation_credits.iter().cloned())
            .collect(),
    };
    protocol.validate()?;
    Ok(protocol)
}

fn derive_ingress(
    configuration: &DirectSurfaceLiquidConfiguration,
    schedule: &CoveredIngressSchedule,
    legacy: &[FinalizedRuntimeTile],
    frozen: &[AcceptedV3ForestLitterTile],
) -> Result<DirectSurfaceLiquidIngressInput, LandSurfaceEnergyShadowError> {
    let mut seen = BTreeSet::new();
    let mut rows = schedule.open_tile_ingress.clone();
    for row in &rows {
        let identity = match row {
            DirectTileGroundIngress::OpenRawPrecipitation {
                ofe_id,
                tile_id,
                surface_id,
                ..
            }
            | DirectTileGroundIngress::OpenLiquidParcels {
                ofe_id,
                tile_id,
                surface_id,
                ..
            } => (ofe_id.clone(), tile_id.clone(), surface_id.clone()),
            _ => {
                return Err(LandSurfaceEnergyShadowError::Identity(
                    "caller supplied covered ingress to V3 pre-ingress seam",
                ));
            }
        };
        if !seen.insert(identity) {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "duplicate open V3 ingress",
            ));
        }
    }
    let mut push_covered = |identity: &RuntimeTileIdentity,
                            release: crate::DirectCanopyLiquidRelease|
     -> Result<(), LandSurfaceEnergyShadowError> {
        if schedule.transaction_id != identity.transaction_id
            || schedule.interval_s.to_bits() != identity.interval_s.to_bits()
            || !seen.insert((
                identity.ofe_id.clone(),
                identity.tile_id.clone(),
                identity.surface_id.clone(),
            ))
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "V3 derived ingress identity or duplicate",
            ));
        }
        let runon = schedule
            .covered_runon
            .get(&(identity.ofe_id.clone(), identity.tile_id.clone()))
            .cloned()
            .unwrap_or_default();
        rows.push(if runon.is_empty() {
            DirectTileGroundIngress::CoveredCanopyRelease {
                ofe_id: identity.ofe_id.clone(),
                tile_id: identity.tile_id.clone(),
                surface_id: identity.surface_id.clone(),
                release,
            }
        } else {
            DirectTileGroundIngress::CoveredCanopyReleaseAndRunon {
                ofe_id: identity.ofe_id.clone(),
                tile_id: identity.tile_id.clone(),
                surface_id: identity.surface_id.clone(),
                release,
                runon_parcels: runon,
            }
        });
        Ok(())
    };
    for tile in legacy {
        if let FinalizedRuntimeTile::Covered(tile) = tile {
            tile.vegetation_operands.validate()?;
            push_covered(
                &tile.identity,
                derive_release_from_ledgers(
                    tile.vegetation_operands
                        .occupancies
                        .iter()
                        .map(|row| (row.occupancy_id.as_str(), &row.liquid)),
                    tile.vegetation_operands
                        .ground_canopy_release_kg_m2_tile_ground,
                    tile.vegetation_operands.ground_stemflow_kg_m2_tile_ground,
                    schedule.interval_s,
                )?,
            )?;
        }
    }
    for tile in frozen {
        let evaluation = &tile.fixed_final.accepted_fixed_final.evaluation.predecessor;
        let configured_ids = tile
            .covered_beginning
            .occupancies
            .iter()
            .map(|row| row.occupancy_id.clone())
            .collect::<Vec<_>>();
        validate_occupancy_identity_order(
            &configured_ids,
            &tile.occupancy_ids,
            evaluation.occupancies.len(),
        )?;
        push_covered(
            &tile.fixed_final.identity,
            derive_release_from_ledgers(
                tile.occupancy_ids
                    .iter()
                    .zip(&evaluation.occupancies)
                    .map(|(identity, row)| (identity.as_str(), &row.liquid)),
                evaluation.ground_canopy_release_kg_m2_tile,
                evaluation.ground_stemflow_kg_m2_tile,
                schedule.interval_s,
            )?,
        )?;
    }
    let configured = configuration
        .records
        .iter()
        .map(|row| {
            (
                row.key.ofe_id.clone(),
                row.key.tile_id.clone(),
                row.key.surface_id.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    if configured != seen {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "V3 pre-ingress topology is incomplete",
        ));
    }
    Ok(DirectSurfaceLiquidIngressInput {
        transaction_id: schedule.transaction_id,
        day_index: schedule.day_index,
        interval_index: schedule.interval_index,
        interval_s: schedule.interval_s,
        tile_ingress: rows,
        wb14_parameters: schedule.wb14_parameters.clone(),
    })
}

fn validate_occupancy_identity_order(
    configured: &[String],
    retained: &[String],
    ledger_count: usize,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let unique = retained.iter().collect::<BTreeSet<_>>();
    if retained != configured || retained.len() != ledger_count || unique.len() != retained.len() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "V3 covered occupancy identity/order/cardinality",
        ));
    }
    Ok(())
}

/// Run the one fixed-final solve per tile and stop before current ingress.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn finalize_v3_multitile_fixed_final(
    configuration: &DirectSurfaceLiquidConfiguration,
    potential: V3MultiTilePotentialCandidate,
    expected_beginning_lse_state_sha256: &Sha256Digest,
    authorizations: Vec<WaterAuthorization>,
    mut phase_authorizations: BTreeMap<(OfeId, TileId), V3PhaseSpecificVaporAuthorization>,
    soil_arbitration: super::MixedRealHydrologyArbitration,
    receiver_expectations: super::UnifiedReceiverExpectations,
    soil_sources: BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    vegetation_bindings: Vec<V8ComponentOccupancyBinding>,
    vegetation_configuration: VegetationConfiguration,
    vegetation_beginning: V8CoupledOwnedState,
    persistent_forcing: V8PersistentForcingReceipt,
    schedule: &CoveredIngressSchedule,
) -> Result<V3MultiTileAcceptedFixedFinalCandidate, LandSurfaceEnergyShadowError> {
    let potential_vegetation_operands = potential
        .tiles
        .iter()
        .filter_map(|tile| match tile {
            V3PotentialTile::Legacy(PotentialTilePhase::Covered { phase, .. }) => {
                Some(phase.potential_vegetation_operands.clone())
            }
            V3PotentialTile::FrozenForestLitter { phase, .. } => {
                Some(phase.potential_vegetation_operands.clone())
            }
            _ => None,
        })
        .collect();
    let mut legacy = Vec::new();
    let mut frozen = Vec::new();
    let mut protocols = Vec::new();
    for tile in potential.tiles {
        match tile {
            V3PotentialTile::Legacy(PotentialTilePhase::Open {
                phase,
                final_initial_trial,
                soil_thermal,
            }) => {
                let subset = authorization_subset(&phase.request_batch, &authorizations)?;
                if subset.len() != 1 {
                    return Err(LandSurfaceEnergyShadowError::Identity(
                        "open authorization cardinality",
                    ));
                }
                let final_tile = finalize_open_phase_with_soil_thermal_beginning(
                    &phase,
                    expected_beginning_lse_state_sha256,
                    &subset[0],
                    final_initial_trial,
                    soil_thermal.finalization_beginning(),
                )?;
                legacy.push(FinalizedRuntimeTile::Open(final_tile));
            }
            V3PotentialTile::Legacy(PotentialTilePhase::Stage3OpenSnow { .. }) => {
                return Err(LandSurfaceEnergyShadowError::UnsupportedCustody(
                    "Stage-3 snow completion is outside the frozen-litter V3 seam",
                ));
            }
            V3PotentialTile::Legacy(PotentialTilePhase::Covered {
                phase,
                final_initial_trial,
                soil_thermal,
            }) => {
                let subset = authorization_subset(phase.request_batch(), &authorizations)?;
                let final_tile = finalize_covered_phase_with_soil_thermal_beginning(
                    &phase,
                    expected_beginning_lse_state_sha256,
                    subset,
                    final_initial_trial,
                    soil_thermal.finalization_beginning(),
                )?;
                legacy.push(FinalizedRuntimeTile::Covered(final_tile));
            }
            V3PotentialTile::FrozenForestLitter {
                phase,
                final_initial_trial,
                soil_thermal,
                litter_configuration,
                litter_beginning,
                covered_beginning,
                root_identities,
                occupancy_ids,
            } => {
                let subset = authorization_subset(phase.request_batch(), &authorizations)?;
                let key = (
                    phase.identity().ofe_id.clone(),
                    phase.identity().tile_id.clone(),
                );
                let phase_authorization = phase_authorizations.remove(&key).ok_or(
                    LandSurfaceEnergyShadowError::Identity(
                        "missing V3 phase-specific vapor authorization",
                    ),
                )?;
                let fixed_final = finalize_v3_covered_phase(
                    &phase,
                    expected_beginning_lse_state_sha256,
                    subset,
                    phase_authorization,
                    &final_initial_trial,
                    soil_thermal.finalization_beginning(),
                )?;
                let phase_free_input = FrozenLitterV3PhaseFreeInput::from_accepted_fixed_final(
                    fixed_final.identity.ofe_id.clone(),
                    fixed_final.identity.tile_id.clone(),
                    litter_configuration,
                    litter_beginning,
                    &fixed_final.accepted_fixed_final.evaluation,
                );
                frozen.push(AcceptedV3ForestLitterTile {
                    fixed_final,
                    phase_free_input,
                    soil_thermal,
                    covered_beginning,
                    root_identities,
                    occupancy_ids,
                });
            }
        }
    }
    if !phase_authorizations.is_empty() || frozen.is_empty() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "unexpected or absent V3 phase-specific authorization",
        ));
    }
    for tile in &legacy {
        protocols.push(
            tile.water_protocol()
                .ok_or(LandSurfaceEnergyShadowError::Identity(
                    "missing legacy water protocol",
                ))?,
        );
    }
    protocols.extend(frozen.iter().map(|tile| &tile.fixed_final.water_protocol));
    let water_protocol = combined_protocol(&potential.request_batch, authorizations, &protocols)?;
    let derived_current_ingress = derive_ingress(configuration, schedule, &legacy, &frozen)?;
    Ok(V3MultiTileAcceptedFixedFinalCandidate {
        legacy_tiles: legacy,
        frozen_litter_tiles: frozen,
        water_protocol,
        soil_arbitration,
        receiver_expectations,
        soil_sources,
        potential_vegetation_operands,
        vegetation_bindings,
        vegetation_configuration,
        vegetation_beginning,
        persistent_forcing,
        derived_current_ingress,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_boundary_is_pre_ingress_and_crate_private() {
        let source = include_str!("v3_multitile_adoption.rs");
        assert!(source.contains("pub(crate) struct V3MultiTileAcceptedFixedFinalCandidate"));
        assert!(!source.contains(concat!(
            "pub struct ",
            "V3MultiTileAcceptedFixedFinalCandidate"
        )));
        assert!(!source.contains(concat!("execute_surface_liquid", "_ingress")));
        assert!(!source.contains(concat!("execute_litter_phase", "_v3")));
    }

    #[test]
    fn forest_litter_has_no_legacy_solve_branch() {
        let source = include_str!("v3_multitile_adoption.rs");
        let legacy_guard = source
            .find("forest litter cannot enter the legacy multi-tile solve")
            .expect("legacy forest-litter guard");
        let v3_solve = source
            .find("solve_v3_covered_potential_phase")
            .expect("V3 potential binding");
        assert!(legacy_guard > v3_solve);
        assert!(source.contains("fixed_final.accepted_fixed_final.evaluation"));
    }

    #[test]
    fn exact_sixty_second_grid_is_bound_before_solve() {
        let source = include_str!("v3_multitile_adoption.rs");
        assert!(source.contains("identity.interval_s < 60.0"));
        assert!(source.contains("(identity.interval_s / 60.0).fract() != 0.0"));
    }

    #[test]
    fn wrong_occupancy_identity_and_order_fail_closed() {
        let first = String::from("canopy-a");
        let second = String::from("canopy-b");
        let configured = vec![first.clone(), second.clone()];
        assert!(validate_occupancy_identity_order(&configured, &configured, 2).is_ok());
        assert!(
            validate_occupancy_identity_order(&configured, &[second.clone(), first.clone()], 2,)
                .is_err()
        );
        assert!(
            validate_occupancy_identity_order(&configured, &[first.clone(), first], 2).is_err()
        );
        assert!(validate_occupancy_identity_order(&configured, &configured, 1).is_err());
    }
}
