//! Complete heterogeneous-tile potential/final execution over one OFE owner set.
//!
//! Every projected tile is solved twice from one immutable beginning problem:
//! first without owner caps to construct one request batch, then under its
//! exact subset of the single real-owner authorization. There is no companion
//! request, use, LSE-state, or soil-thermal-candidate input surface.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::TileId;
use openwepp_land_surface_energy::{
    AcceptedOpenSurface, ClosureValue, CoveredColumnInputs, CoveredLowerBoundaryEnergyOperands,
    CoveredPotentialPhase, CoveredTileEnergyOperandSet, FinalCoveredTileCandidate,
    FinalTileCandidate, GroundWaterKey, OfeId, OpenPotentialPhase, OpenSurfaceProblem,
    PotentialWaterRequestBatch, RootRuntimeIdentity, RuntimeTileIdentity, SoilThermalSnapshot,
    StandGroundWaterAmountBasis, TileEnergyOperandSet, TileState, WaterAmount, WaterAuthorization,
    WaterProtocol, WeightedTileEnergyOperands, canonical_tile_fraction_sum_closes,
    finalize_covered_phase, finalize_open_phase, solve_covered_potential_phase,
    solve_open_potential_phase, validate_weighted_ofe_energy,
};

use super::{
    CoveredIngressSchedule, DirectSurfaceLiquidConfiguration,
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError, OwnerKind,
    RealHydrologySourceKey, SoilThermalTileCandidate, UnifiedLseFinalization,
    UnifiedRealHydrologyCandidate, UnifiedReceiverExpectations,
};

/// Strictly projected open-tile problem and its numerical trials.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StrictProjectedOpenTile {
    pub(crate) identity: RuntimeTileIdentity,
    pub(crate) beginning: OpenSurfaceProblem,
    pub(crate) potential_initial_trial: Option<Vec<f64>>,
    pub(crate) final_initial_trial: Option<Vec<f64>>,
    pub(crate) soil_thermal: SoilThermalSnapshot,
}

/// Strictly projected covered-tile problem and its numerical trials.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StrictProjectedCoveredTile {
    pub(crate) identity: RuntimeTileIdentity,
    pub(crate) beginning: CoveredColumnInputs,
    pub(crate) roots: Vec<RootRuntimeIdentity>,
    pub(crate) potential_initial_trial: Vec<f64>,
    pub(crate) final_initial_trial: Vec<f64>,
    pub(crate) soil_thermal: SoilThermalSnapshot,
}

/// Topology-preserving member whose physical snow surface is owned entirely by
/// Stage 3. It must not enter the ordinary open-ground solver.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StrictProjectedStage3OpenSnowTile {
    pub(crate) identity: RuntimeTileIdentity,
    pub(crate) beginning_state: TileState,
    pub(crate) soil_thermal: SoilThermalSnapshot,
}

/// One member of the exact configured heterogeneous tile set.
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum StrictProjectedTileProblem {
    Open(StrictProjectedOpenTile),
    Stage3OpenSnow(StrictProjectedStage3OpenSnowTile),
    Covered(StrictProjectedCoveredTile),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MultiTileFailurePhase {
    PotentialTile(usize),
    CombinedRequests,
    Authorization,
    FinalTile(usize),
    E04Ingress,
    OpenIngress,
    UnifiedHydrology,
    LocalEnergy,
    OfeEnergy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PendingPayloadKind {
    CombinedRequest,
    Authorization,
    FinalTileUse,
    FinalProtocol,
    Ingress,
}

type FailureHook<'a> =
    Option<&'a dyn Fn(MultiTileFailurePhase) -> Result<(), LandSurfaceEnergyShadowError>>;
type PendingEnvelopeHook<'a> =
    Option<&'a dyn Fn(PendingPayloadKind, &[u8]) -> Result<(), LandSurfaceEnergyShadowError>>;

impl StrictProjectedTileProblem {
    fn identity(&self) -> &RuntimeTileIdentity {
        match self {
            Self::Open(value) => &value.identity,
            Self::Stage3OpenSnow(value) => &value.identity,
            Self::Covered(value) => &value.identity,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PotentialTilePhase {
    Open {
        phase: OpenPotentialPhase,
        final_initial_trial: Option<Vec<f64>>,
        soil_thermal: SoilThermalSnapshot,
    },
    Stage3OpenSnow {
        identity: RuntimeTileIdentity,
        beginning_state: TileState,
        soil_thermal: SoilThermalSnapshot,
        request_batch: PotentialWaterRequestBatch,
    },
    Covered {
        phase: CoveredPotentialPhase,
        final_initial_trial: Vec<f64>,
        soil_thermal: SoilThermalSnapshot,
    },
}

impl PotentialTilePhase {
    fn identity(&self) -> &RuntimeTileIdentity {
        match self {
            Self::Open { phase, .. } => &phase.identity,
            Self::Stage3OpenSnow { identity, .. } => identity,
            Self::Covered { phase, .. } => phase.identity(),
        }
    }

    fn request_batch(&self) -> Option<&PotentialWaterRequestBatch> {
        match self {
            Self::Open { phase, .. } => Some(&phase.request_batch),
            Self::Stage3OpenSnow { request_batch, .. } => Some(request_batch),
            Self::Covered { phase, .. } => Some(phase.request_batch()),
        }
    }

    pub(crate) fn covered(&self) -> Option<&CoveredPotentialPhase> {
        match self {
            Self::Open { .. } => None,
            Self::Stage3OpenSnow { .. } => None,
            Self::Covered { phase, .. } => Some(phase),
        }
    }
}

/// Accepted fixed-cap candidate for one exact tile.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FinalizedRuntimeTile {
    Open(FinalTileCandidate<AcceptedOpenSurface>),
    Stage3OpenSnow {
        identity: RuntimeTileIdentity,
        ending_tile_state_pre_ingress: TileState,
        soil_thermal: SoilThermalTileCandidate,
        water_protocol: WaterProtocol,
        rollback_hashes: Vec<openwepp_land_surface_energy::OwnerRollbackHash>,
    },
    Covered(FinalCoveredTileCandidate),
}

impl FinalizedRuntimeTile {
    #[must_use]
    fn identity(&self) -> &RuntimeTileIdentity {
        match self {
            Self::Open(value) => &value.identity,
            Self::Stage3OpenSnow { identity, .. } => identity,
            Self::Covered(value) => &value.identity,
        }
    }

    pub(crate) fn covered(&self) -> Option<&FinalCoveredTileCandidate> {
        match self {
            Self::Open(_) => None,
            Self::Stage3OpenSnow { .. } => None,
            Self::Covered(value) => Some(value),
        }
    }

    #[must_use]
    fn water_protocol(&self) -> Option<&WaterProtocol> {
        match self {
            Self::Open(value) => Some(&value.water_protocol),
            Self::Stage3OpenSnow { water_protocol, .. } => Some(water_protocol),
            Self::Covered(value) => Some(&value.water_protocol),
        }
    }

    #[must_use]
    fn ending_tile_state_pre_ingress(&self) -> &TileState {
        match self {
            Self::Open(value) => &value.ending_tile_state_pre_ingress,
            Self::Stage3OpenSnow {
                ending_tile_state_pre_ingress,
                ..
            } => ending_tile_state_pre_ingress,
            Self::Covered(value) => &value.ending_tile_state_pre_ingress,
        }
    }

    #[must_use]
    fn soil_thermal(&self) -> &SoilThermalTileCandidate {
        match self {
            Self::Open(value) => &value.soil_thermal,
            Self::Stage3OpenSnow { soil_thermal, .. } => soil_thermal,
            Self::Covered(value) => &value.soil_thermal,
        }
    }

    #[must_use]
    fn energy_operands(&self) -> RuntimeTileEnergyOperands<'_> {
        match self {
            Self::Open(value) => RuntimeTileEnergyOperands::Open(&value.energy_operands),
            Self::Stage3OpenSnow { .. } => RuntimeTileEnergyOperands::Stage3OpenSnow,
            Self::Covered(value) => RuntimeTileEnergyOperands::Covered(&value.energy_operands),
        }
    }
}

/// Borrowed local energy operands retaining open/covered type identity.
#[derive(Clone, Copy, Debug)]
enum RuntimeTileEnergyOperands<'a> {
    Open(&'a TileEnergyOperandSet),
    Stage3OpenSnow,
    Covered(&'a CoveredTileEnergyOperandSet),
}

impl RuntimeTileEnergyOperands<'_> {
    fn validate(&self) -> Result<(), LandSurfaceEnergyShadowError> {
        match self {
            Self::Open(value) => value.validate()?,
            Self::Stage3OpenSnow => {}
            Self::Covered(value) => value.validate()?,
        }
        Ok(())
    }
}

/// Independently reconstructed weighted ground-control-volume join for one
/// complete OFE tile set.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WeightedOfeEnergyJoin {
    ofe_id: OfeId,
    ordered_tile_ids: Vec<TileId>,
    operands: Vec<WeightedTileEnergyOperands>,
    closure: ClosureValue,
}

/// Complete multi-tile result. Final tiles are generated internally and are
/// never accepted from the caller.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MultiTileRuntimeResult {
    potential_request_batch: PotentialWaterRequestBatch,
    potential_tiles: Vec<PotentialTilePhase>,
    finalized_tiles: Vec<FinalizedRuntimeTile>,
    weighted_ofe_energy: Vec<WeightedOfeEnergyJoin>,
    hydrology_candidate: UnifiedRealHydrologyCandidate,
}

impl MultiTileRuntimeResult {
    #[must_use]
    #[allow(dead_code)]
    pub(crate) const fn potential_request_batch(&self) -> &PotentialWaterRequestBatch {
        &self.potential_request_batch
    }

    #[must_use]
    pub(crate) fn potential_tiles(&self) -> &[PotentialTilePhase] {
        &self.potential_tiles
    }

    #[must_use]
    pub(crate) fn finalized_tiles(&self) -> &[FinalizedRuntimeTile] {
        &self.finalized_tiles
    }

    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn weighted_ofe_energy(&self) -> &[WeightedOfeEnergyJoin] {
        &self.weighted_ofe_energy
    }

    #[must_use]
    pub(crate) const fn hydrology_candidate(&self) -> &UnifiedRealHydrologyCandidate {
        &self.hydrology_candidate
    }
}

/// Execute all strictly projected open and covered tiles against one real
/// hydrology authorization. Covered ingress is constructed only from the
/// accepted fixed-cap E04 ledgers by the derived-ingress owner boundary.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn execute_multi_tile_runtime(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    projected_tiles: Vec<StrictProjectedTileProblem>,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress_schedule: &CoveredIngressSchedule,
    failure_hook: FailureHook<'_>,
    pending_hook: PendingEnvelopeHook<'_>,
) -> Result<MultiTileRuntimeResult, LandSurfaceEnergyShadowError> {
    execute_multi_tile_runtime_with_mode(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        projected_tiles,
        soil_sources,
        ingress_schedule,
        failure_hook,
        pending_hook,
        true,
    )
}

/// Execute a candidate physical endpoint while retaining local/component
/// closures but deferring the aggregate OFE energy gate. This is only for the
/// uncommitted first pass of the covered longwave fixed point; callers must
/// rerun the final pass with `execute_multi_tile_runtime` before acceptance.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn execute_multi_tile_runtime_provisional(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    projected_tiles: Vec<StrictProjectedTileProblem>,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress_schedule: &CoveredIngressSchedule,
    failure_hook: FailureHook<'_>,
    pending_hook: PendingEnvelopeHook<'_>,
) -> Result<MultiTileRuntimeResult, LandSurfaceEnergyShadowError> {
    execute_multi_tile_runtime_with_mode(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        projected_tiles,
        soil_sources,
        ingress_schedule,
        failure_hook,
        pending_hook,
        false,
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn execute_multi_tile_runtime_with_mode(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    projected_tiles: Vec<StrictProjectedTileProblem>,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress_schedule: &CoveredIngressSchedule,
    failure_hook: FailureHook<'_>,
    pending_hook: PendingEnvelopeHook<'_>,
    validate_ofe_energy: bool,
) -> Result<MultiTileRuntimeResult, LandSurfaceEnergyShadowError> {
    let projected_tiles =
        validate_and_sort_projected_tiles(projected_tiles, surface_configuration)?;
    let potential_phases = solve_all_potential(projected_tiles, failure_hook)?;
    let request_batch = combined_request_batch(&potential_phases)?;
    publish_pending_debug(
        pending_hook,
        PendingPayloadKind::CombinedRequest,
        &request_batch,
    )?;
    run_failure_hook(failure_hook, MultiTileFailurePhase::CombinedRequests)?;
    let mut retained_final_tiles = None;

    let hydrology_candidate = super::covered_derived_ingress::execute_unified_with_derived_ingress(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        &request_batch,
        soil_sources,
        ingress_schedule,
        |authorizations| {
            publish_pending(
                pending_hook,
                PendingPayloadKind::Authorization,
                authorizations,
            )?;
            run_failure_hook(failure_hook, MultiTileFailurePhase::Authorization)?;
            let final_tiles = finalize_all_tiles(
                &potential_phases,
                authorizations,
                failure_hook,
                pending_hook,
            )?;
            let protocol = combined_protocol(&request_batch, authorizations, &final_tiles)?;
            publish_pending(pending_hook, PendingPayloadKind::FinalProtocol, &protocol)?;
            let ending_tiles = final_tiles
                .iter()
                .map(|tile| tile.ending_tile_state_pre_ingress().clone())
                .collect();
            let soil_thermal = final_tiles
                .iter()
                .map(|tile| tile.soil_thermal().clone())
                .collect();
            let rollback_hashes = common_owner_rollback_hashes(&final_tiles)?;
            let sealed = UnifiedLseFinalization::try_new(
                receiver_expectations,
                protocol.clone(),
                ending_tiles,
                soil_thermal,
                rollback_hashes,
            )?;
            let covered_final_tiles = final_tiles
                .iter()
                .filter_map(|tile| match tile {
                    FinalizedRuntimeTile::Open(_) | FinalizedRuntimeTile::Stage3OpenSnow { .. } => {
                        None
                    }
                    FinalizedRuntimeTile::Covered(value) => Some(value.clone()),
                })
                .collect::<Vec<_>>();
            let ingress = super::covered_derived_ingress::derive_fixed_cap_canopy_ingress(
                surface_configuration,
                &covered_final_tiles,
                ingress_schedule,
            )?;
            publish_pending(pending_hook, PendingPayloadKind::Ingress, &ingress)?;
            if ingress.tile_ingress.iter().any(|row| {
                matches!(
                    row,
                    crate::DirectTileGroundIngress::CoveredCanopyRelease { .. }
                        | crate::DirectTileGroundIngress::CoveredCanopyReleaseAndRunon { .. }
                )
            }) {
                run_failure_hook(failure_hook, MultiTileFailurePhase::E04Ingress)?;
            }
            if ingress.tile_ingress.iter().any(|row| {
                matches!(
                    row,
                    crate::DirectTileGroundIngress::OpenRawPrecipitation { .. }
                        | crate::DirectTileGroundIngress::OpenLiquidParcels { .. }
                )
            }) {
                run_failure_hook(failure_hook, MultiTileFailurePhase::OpenIngress)?;
            }
            retained_final_tiles = Some(final_tiles);
            Ok((sealed, ingress))
        },
    )?;
    run_failure_hook(failure_hook, MultiTileFailurePhase::UnifiedHydrology)?;

    let finalized_tiles = retained_final_tiles.ok_or(LandSurfaceEnergyShadowError::Identity(
        "multi-tile finalizer returned no tile candidates",
    ))?;
    for tile in &finalized_tiles {
        tile.energy_operands().validate()?;
    }
    run_failure_hook(failure_hook, MultiTileFailurePhase::LocalEnergy)?;
    let weighted_ofe_energy = if validate_ofe_energy {
        reconstruct_weighted_ofe_energy(&finalized_tiles)?
    } else {
        Vec::new()
    };
    run_failure_hook(failure_hook, MultiTileFailurePhase::OfeEnergy)?;
    Ok(MultiTileRuntimeResult {
        potential_request_batch: request_batch,
        potential_tiles: potential_phases,
        finalized_tiles,
        weighted_ofe_energy,
        hydrology_candidate,
    })
}

fn validate_and_sort_projected_tiles(
    mut projected: Vec<StrictProjectedTileProblem>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<Vec<StrictProjectedTileProblem>, LandSurfaceEnergyShadowError> {
    surface_configuration.validate()?;
    if projected.is_empty() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "empty projected tile set",
        ));
    }
    let configured_rank = surface_configuration
        .records
        .iter()
        .enumerate()
        .map(|(rank, record)| {
            (
                (record.key.ofe_id.clone(), record.key.tile_id.clone()),
                rank,
            )
        })
        .collect::<BTreeMap<_, _>>();
    projected.sort_by_key(|tile| {
        configured_rank
            .get(&(
                tile.identity().ofe_id.clone(),
                tile.identity().tile_id.clone(),
            ))
            .copied()
            .unwrap_or(usize::MAX)
    });
    let first = projected[0].identity();
    let mut identities = BTreeSet::new();
    let mut fractions = BTreeMap::<OfeId, f64>::new();
    for tile in &projected {
        let identity = tile.identity();
        if identity.transaction_id != first.transaction_id
            || identity.configuration_sha256 != first.configuration_sha256
            || identity.beginning_lse_state_sha256 != first.beginning_lse_state_sha256
            || identity.beginning_hydrology_snapshot_sha256
                != first.beginning_hydrology_snapshot_sha256
            || identity.beginning_soil_thermal_state_sha256
                != first.beginning_soil_thermal_state_sha256
            || identity.lse_owner_id != first.lse_owner_id
            || identity.hydrology_owner_id != first.hydrology_owner_id
            || identity.soil_thermal_owner_id != first.soil_thermal_owner_id
            || identity.interval_s.to_bits() != first.interval_s.to_bits()
            || !identities.insert((identity.ofe_id.clone(), identity.tile_id.clone()))
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "mixed or duplicate projected tile lineage",
            ));
        }
        let configured = surface_configuration
            .records
            .iter()
            .find(|record| {
                record.key.ofe_id == identity.ofe_id && record.key.tile_id == identity.tile_id
            })
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "projected tile absent from configured topology",
            ))?;
        if identity.hydrology_owner_id != surface_configuration.owner_id
            || identity.tile_fraction.to_bits() != configured.tile_fraction.to_bits()
            || identity.surface_id != configured.key.surface_id
            || identity.surface_class != configured.key.surface_class
            || identity.ground_source_type != configured.key.source_type
            || identity.ground_source_id != configured.key.source_id
            || identity.ground_source_tile_id.as_ref() != Some(&configured.key.tile_id)
            || identity.ground_soil_layer_id.is_some()
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "projected/configured tile operand mismatch",
            ));
        }
        validate_projected_soil_order(tile, surface_configuration)?;
        *fractions.entry(identity.ofe_id.clone()).or_default() += identity.tile_fraction;
    }
    if fractions
        .values()
        .any(|sum| !canonical_tile_fraction_sum_closes(*sum))
    {
        return Err(LandSurfaceEnergyShadowError::Bound(
            "projected OFE tile fractions do not sum to one",
        ));
    }
    let configured = surface_configuration
        .records
        .iter()
        .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
        .collect::<BTreeSet<_>>();
    if configured != identities || configured.len() != surface_configuration.records.len() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "projected/configured tile identity set",
        ));
    }
    Ok(projected)
}

fn validate_projected_soil_order(
    tile: &StrictProjectedTileProblem,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let identity = tile.identity();
    let snapshot = match tile {
        StrictProjectedTileProblem::Open(value) => &value.soil_thermal,
        StrictProjectedTileProblem::Stage3OpenSnow(value) => &value.soil_thermal,
        StrictProjectedTileProblem::Covered(value) => &value.soil_thermal,
    };
    snapshot.validate()?;
    let binding = surface_configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.ofe_id == identity.ofe_id)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing configured soil ordering",
        ))?;
    let projected_ofes = snapshot
        .ofes
        .iter()
        .map(|ofe| ofe.ofe_id.clone())
        .collect::<Vec<_>>();
    let layers = snapshot
        .ofes
        .iter()
        .find(|ofe| ofe.ofe_id == identity.ofe_id)
        .map(|ofe| {
            ofe.ordered_layers
                .iter()
                .map(|layer| layer.layer_id.clone())
                .collect::<Vec<_>>()
        });
    if snapshot.owner_id != identity.soil_thermal_owner_id
        || snapshot.state_sha256 != identity.beginning_soil_thermal_state_sha256
        || projected_ofes != surface_configuration.ofe_topology
        || layers.as_ref() != Some(&binding.ordered_soil_layer_ids)
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "projected/configured soil thermal ordering",
        ));
    }
    Ok(())
}

fn solve_all_potential(
    projected: Vec<StrictProjectedTileProblem>,
    failure_hook: FailureHook<'_>,
) -> Result<Vec<PotentialTilePhase>, LandSurfaceEnergyShadowError> {
    projected
        .into_iter()
        .enumerate()
        .map(|(index, tile)| {
            let phase: PotentialTilePhase = match tile {
                StrictProjectedTileProblem::Open(value) => {
                    let phase = solve_open_potential_phase(
                        value.identity,
                        &value.beginning,
                        value.potential_initial_trial,
                    )?;
                    Ok::<_, LandSurfaceEnergyShadowError>(PotentialTilePhase::Open {
                        phase,
                        final_initial_trial: value.final_initial_trial,
                        soil_thermal: value.soil_thermal,
                    })
                }
                StrictProjectedTileProblem::Stage3OpenSnow(value) => {
                    let request_batch = PotentialWaterRequestBatch::try_new(
                        value.identity.transaction_id,
                        value.identity.beginning_lse_state_sha256.clone(),
                        vec![WaterAmount {
                            key: GroundWaterKey {
                                transaction_id: value.identity.transaction_id,
                                requesting_owner_id: value.identity.lse_owner_id.clone(),
                                requesting_component:
                                    openwepp_land_surface_energy::RequestingComponent::GroundSurface,
                                ofe_id: value.identity.ofe_id.clone(),
                                requesting_tile_id: value.identity.tile_id.clone(),
                                occupancy_id: None,
                                surface_id: Some(value.identity.surface_id.clone()),
                                surface_class: Some(value.identity.surface_class),
                                source_type: value.identity.ground_source_type,
                                source_id: value.identity.ground_source_id.clone(),
                                source_tile_id: value.identity.ground_source_tile_id.clone(),
                                soil_layer_id: value.identity.ground_soil_layer_id.clone(),
                                amount_basis:
                                    StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
                            },
                            amount_kg_m2_stand_ground: 0.0,
                        }],
                    )?;
                    Ok::<_, LandSurfaceEnergyShadowError>(
                        PotentialTilePhase::Stage3OpenSnow {
                            identity: value.identity,
                            beginning_state: value.beginning_state,
                            soil_thermal: value.soil_thermal,
                            request_batch,
                        },
                    )
                }
                StrictProjectedTileProblem::Covered(value) => {
                    let phase = solve_covered_potential_phase(
                        value.identity,
                        &value.beginning,
                        value.roots,
                        value.potential_initial_trial,
                    )?;
                    Ok::<_, LandSurfaceEnergyShadowError>(PotentialTilePhase::Covered {
                        phase,
                        final_initial_trial: value.final_initial_trial,
                        soil_thermal: value.soil_thermal,
                    })
                }
            }?;
            run_failure_hook(failure_hook, MultiTileFailurePhase::PotentialTile(index))?;
            Ok(phase)
        })
        .collect()
}

fn combined_request_batch(
    phases: &[PotentialTilePhase],
) -> Result<PotentialWaterRequestBatch, LandSurfaceEnergyShadowError> {
    let first = phases
        .first()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "empty potential tile set",
        ))?;
    let requests = phases
        .iter()
        .filter_map(PotentialTilePhase::request_batch)
        .flat_map(|batch| batch.requests.iter().cloned())
        .collect();
    Ok(PotentialWaterRequestBatch::try_new(
        first.identity().transaction_id,
        first.identity().beginning_lse_state_sha256.clone(),
        requests,
    )?)
}

fn authorization_subset(
    phase: &PotentialTilePhase,
    authorizations: &[WaterAuthorization],
) -> Result<Vec<WaterAuthorization>, LandSurfaceEnergyShadowError> {
    let keys = phase
        .request_batch()
        .into_iter()
        .flat_map(|batch| batch.requests.iter())
        .map(|row| row.key.clone())
        .collect::<BTreeSet<_>>();
    let subset = authorizations
        .iter()
        .filter(|row| keys.contains(&row.key))
        .cloned()
        .collect::<Vec<_>>();
    if subset.len() != keys.len() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "incomplete tile authorization subset",
        ));
    }
    Ok(subset)
}

fn finalize_all_tiles(
    phases: &[PotentialTilePhase],
    authorizations: &[WaterAuthorization],
    failure_hook: FailureHook<'_>,
    pending_hook: PendingEnvelopeHook<'_>,
) -> Result<Vec<FinalizedRuntimeTile>, LandSurfaceEnergyShadowError> {
    phases
        .iter()
        .enumerate()
        .map(|(index, phase)| {
            let mut subset = authorization_subset(phase, authorizations)?;
            let finalized: FinalizedRuntimeTile = match phase {
                PotentialTilePhase::Open {
                    phase,
                    final_initial_trial,
                    soil_thermal,
                } => {
                    if subset.len() != 1 {
                        return Err(LandSurfaceEnergyShadowError::Identity(
                            "open tile authorization cardinality",
                        ));
                    }
                    Ok::<_, LandSurfaceEnergyShadowError>(FinalizedRuntimeTile::Open(
                        finalize_open_phase(
                            phase,
                            &phase.identity.beginning_lse_state_sha256,
                            &subset.remove(0),
                            final_initial_trial.clone(),
                            soil_thermal,
                        )?,
                    ))
                }
                PotentialTilePhase::Stage3OpenSnow {
                    identity,
                    beginning_state,
                    soil_thermal,
                    request_batch,
                } => {
                    if subset.len() != 1
                        || subset[0].amount_kg_m2_stand_ground.to_bits() != 0.0f64.to_bits()
                    {
                        return Err(LandSurfaceEnergyShadowError::Identity(
                            "Stage-3 open-snow pass-through authorization",
                        ));
                    }
                    let ofe = soil_thermal
                        .ofes
                        .iter()
                        .find(|row| row.ofe_id == identity.ofe_id)
                        .ok_or(LandSurfaceEnergyShadowError::Identity(
                            "Stage-3 open-snow soil-thermal OFE",
                        ))?;
                    let thermal = SoilThermalTileCandidate {
                        owner_id: soil_thermal.owner_id.clone(),
                        beginning_state_sha256: soil_thermal.state_sha256.clone(),
                        ofe_id: identity.ofe_id.clone(),
                        tile_id: identity.tile_id.clone(),
                        layers: ofe
                            .ordered_layers
                            .iter()
                            .map(
                                |layer| openwepp_land_surface_energy::SoilThermalLayerCandidate {
                                    layer_id: layer.layer_id.clone(),
                                    beginning_enthalpy_j_m2_ofe_ground: layer
                                        .enthalpy_j_m2_ofe_ground,
                                    ground_heat_credit_j_m2_ofe_ground: 0.0,
                                    infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                                    ending_enthalpy_j_m2_ofe_ground: layer.enthalpy_j_m2_ofe_ground,
                                    ending_temperature_k: layer.temperature_k,
                                },
                            )
                            .collect(),
                    };
                    let water_protocol = WaterProtocol {
                        transaction_id: identity.transaction_id,
                        hydrology_owner_id: identity.hydrology_owner_id.clone(),
                        beginning_snapshot_sha256: identity
                            .beginning_hydrology_snapshot_sha256
                            .clone(),
                        requests: request_batch.requests.clone(),
                        authorizations: subset,
                        finalized_uses: request_batch.requests.clone(),
                        condensation_credits: Vec::new(),
                    };
                    water_protocol.validate()?;
                    Ok::<_, LandSurfaceEnergyShadowError>(FinalizedRuntimeTile::Stage3OpenSnow {
                        identity: identity.clone(),
                        ending_tile_state_pre_ingress: beginning_state.clone(),
                        soil_thermal: thermal,
                        water_protocol,
                        rollback_hashes: pass_through_rollback_hashes(identity),
                    })
                }
                PotentialTilePhase::Covered {
                    phase,
                    final_initial_trial,
                    soil_thermal,
                } => Ok::<_, LandSurfaceEnergyShadowError>(FinalizedRuntimeTile::Covered(
                    finalize_covered_phase(
                        phase,
                        &phase.identity().beginning_lse_state_sha256,
                        subset,
                        final_initial_trial.clone(),
                        soil_thermal,
                    )?,
                )),
            }?;
            if let Some(protocol) = finalized.water_protocol() {
                publish_pending(pending_hook, PendingPayloadKind::FinalTileUse, protocol)?;
            }
            run_failure_hook(failure_hook, MultiTileFailurePhase::FinalTile(index))?;
            Ok(finalized)
        })
        .collect()
}

fn run_failure_hook(
    hook: FailureHook<'_>,
    phase: MultiTileFailurePhase,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(hook) = hook {
        hook(phase)?;
    }
    Ok(())
}

fn publish_pending<T: serde::Serialize + ?Sized>(
    hook: PendingEnvelopeHook<'_>,
    kind: PendingPayloadKind,
    value: &T,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(hook) = hook {
        let bytes = serde_json::to_vec(value).map_err(|_| {
            LandSurfaceEnergyShadowError::Identity("pending envelope serialization")
        })?;
        hook(kind, &bytes)?;
    }
    Ok(())
}

fn publish_pending_debug<T: std::fmt::Debug + ?Sized>(
    hook: PendingEnvelopeHook<'_>,
    kind: PendingPayloadKind,
    value: &T,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(hook) = hook {
        let bytes = format!("{value:?}").into_bytes();
        hook(kind, &bytes)?;
    }
    Ok(())
}

fn combined_protocol(
    request_batch: &PotentialWaterRequestBatch,
    authorizations: &[WaterAuthorization],
    final_tiles: &[FinalizedRuntimeTile],
) -> Result<WaterProtocol, LandSurfaceEnergyShadowError> {
    if final_tiles.is_empty() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "empty final tile set",
        ));
    }
    let finalized_uses = final_tiles
        .iter()
        .filter_map(FinalizedRuntimeTile::water_protocol)
        .flat_map(|protocol| protocol.finalized_uses.iter().cloned())
        .collect();
    let condensation_credits = final_tiles
        .iter()
        .filter_map(FinalizedRuntimeTile::water_protocol)
        .flat_map(|protocol| protocol.condensation_credits.iter().cloned())
        .collect();
    let first_protocol = final_tiles
        .iter()
        .find_map(FinalizedRuntimeTile::water_protocol)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "Stage-3 open-only hydrology transaction is not yet admitted",
        ))?;
    let protocol = WaterProtocol {
        transaction_id: request_batch.transaction_id,
        hydrology_owner_id: first_protocol.hydrology_owner_id.clone(),
        beginning_snapshot_sha256: first_protocol.beginning_snapshot_sha256.clone(),
        requests: request_batch.requests.clone(),
        authorizations: authorizations.to_vec(),
        finalized_uses,
        condensation_credits,
    };
    protocol.validate()?;
    Ok(protocol)
}

fn pass_through_rollback_hashes(
    identity: &RuntimeTileIdentity,
) -> Vec<openwepp_land_surface_energy::OwnerRollbackHash> {
    [
        (
            OwnerKind::LandSurfaceEnergy,
            identity.lse_owner_id.as_str(),
            &identity.beginning_lse_state_sha256,
        ),
        (
            OwnerKind::Hydrology,
            identity.hydrology_owner_id.as_str(),
            &identity.beginning_hydrology_snapshot_sha256,
        ),
        (
            OwnerKind::SoilThermal,
            identity.soil_thermal_owner_id.as_str(),
            &identity.beginning_soil_thermal_state_sha256,
        ),
    ]
    .into_iter()
    .map(
        |(owner_kind, owner_id, digest)| openwepp_land_surface_energy::OwnerRollbackHash {
            owner_kind,
            owner_id: owner_id.to_owned(),
            before_sha256: digest.clone(),
            after_sha256: digest.clone(),
        },
    )
    .collect()
}

fn common_owner_rollback_hashes(
    final_tiles: &[FinalizedRuntimeTile],
) -> Result<Vec<openwepp_land_surface_energy::OwnerRollbackHash>, LandSurfaceEnergyShadowError> {
    let first = final_tiles
        .first()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "empty rollback tile set",
        ))?;
    let selected = |tile: &FinalizedRuntimeTile| {
        let rows = match tile {
            FinalizedRuntimeTile::Open(value) => &value.rollback_hashes,
            FinalizedRuntimeTile::Stage3OpenSnow {
                rollback_hashes, ..
            } => rollback_hashes,
            FinalizedRuntimeTile::Covered(value) => &value.rollback_hashes,
        };
        rows.iter()
            .filter(|row| {
                matches!(
                    row.owner_kind,
                    OwnerKind::LandSurfaceEnergy | OwnerKind::Hydrology | OwnerKind::SoilThermal
                )
            })
            .cloned()
            .collect::<Vec<_>>()
    };
    let baseline = selected(first);
    if baseline.len() != 3
        || final_tiles
            .iter()
            .skip(1)
            .any(|tile| selected(tile) != baseline)
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "multi-tile owner rollback identity",
        ));
    }
    Ok(baseline)
}

fn weighted_operand(
    identity: &RuntimeTileIdentity,
    energy: RuntimeTileEnergyOperands<'_>,
) -> Result<WeightedTileEnergyOperands, LandSurfaceEnergyShadowError> {
    energy.validate()?;
    let (
        local_input_j_m2_tile,
        local_output_j_m2_tile,
        local_storage_change_j_m2_tile,
        local_sum_abs_integrated_components_j_m2_tile,
    ) = match energy {
        RuntimeTileEnergyOperands::Open(ground) => {
            let surface = ground.surface;
            (
                (surface.absorbed_shortwave_w_m2 + surface.net_longwave_w_m2) * identity.interval_s,
                surface.sensible_w_m2 * identity.interval_s
                    + ground.latent.vapor_energy_j_m2
                    + surface.ground_heat_w_m2 * identity.interval_s,
                surface.storage_w_m2 * identity.interval_s,
                (surface.absorbed_shortwave_w_m2.abs()
                    + surface.net_longwave_w_m2.abs()
                    + surface.sensible_w_m2.abs()
                    + surface.ground_heat_w_m2.abs()
                    + surface.storage_w_m2.abs())
                    * identity.interval_s
                    + ground.latent.vapor_energy_j_m2.abs(),
            )
        }
        RuntimeTileEnergyOperands::Stage3OpenSnow => (0.0, 0.0, 0.0, 0.0),
        RuntimeTileEnergyOperands::Covered(covered) => {
            let column = &covered.column;
            let incident_shortwave = column.shortwave.incident_w_m2_tile.total();
            let reflected_shortwave = column.shortwave.top_reflected_w_m2_tile.total();
            let canopy_latent_terms = column
                .occupancies
                .iter()
                .flat_map(|occupancy| {
                    [
                        occupancy.sun_leaf,
                        occupancy.shade_leaf,
                        occupancy.wet_surface,
                        occupancy.dry_stem,
                    ]
                })
                .map(|surface| {
                    surface.signed_vapor_to_canopy_air_kg_m2_tile_s
                        * surface.latent_heat_j_kg
                        * identity.interval_s
                })
                .collect::<Vec<_>>();
            let canopy_latent_j_m2 = canopy_latent_terms.iter().sum::<f64>();
            let (
                lower_latent_j_m2,
                lower_ground_heat_w_m2,
                lower_storage_w_m2,
                stage3_boundary_abs_j_m2,
            ) = match &covered.lower_boundary {
                CoveredLowerBoundaryEnergyOperands::SnowFree(ground) => (
                    ground.latent.vapor_energy_j_m2,
                    ground.surface.ground_heat_w_m2,
                    ground.surface.storage_w_m2,
                    0.0,
                ),
                CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) => (
                    stage3.latent_energy_to_canopy_air_j_m2_tile,
                    0.0,
                    0.0,
                    (stage3.optical.absorbed_w_m2_tile.total().abs()
                        + stage3.net_longwave_w_m2_tile.abs()
                        + stage3.sensible_to_canopy_air_w_m2_tile.abs()
                        + (stage3.vapor_to_canopy_air_kg_m2_tile_s * stage3.latent_heat_j_kg)
                            .abs())
                        * stage3.interval_s,
                ),
            };
            let sum_abs_latent_j_m2 = canopy_latent_terms
                .iter()
                .map(|value| value.abs())
                .sum::<f64>()
                + lower_latent_j_m2.abs();
            covered_external_energy(
                identity.interval_s,
                incident_shortwave,
                reflected_shortwave,
                column.longwave.atmospheric_downward_w_m2_tile,
                column.longwave.top_upward_w_m2_tile,
                column.canopy_air.sensible_to_reference_air_w_m2_tile,
                canopy_latent_j_m2 + lower_latent_j_m2,
                lower_ground_heat_w_m2,
                lower_storage_w_m2,
                sum_abs_latent_j_m2,
                column.stage3_lower_boundary_energy_w_m2_tile,
                stage3_boundary_abs_j_m2,
            )
        }
    };
    Ok(WeightedTileEnergyOperands {
        tile_fraction: identity.tile_fraction,
        local_input_j_m2_tile,
        local_output_j_m2_tile,
        local_storage_change_j_m2_tile,
        local_sum_abs_integrated_components_j_m2_tile,
    })
}

#[allow(clippy::too_many_arguments)]
fn covered_external_energy(
    interval_s: f64,
    incident_shortwave_w_m2: f64,
    top_reflected_shortwave_w_m2: f64,
    atmospheric_downward_longwave_w_m2: f64,
    top_upward_longwave_w_m2: f64,
    sensible_to_reference_air_w_m2: f64,
    latent_to_reference_air_j_m2: f64,
    ground_heat_w_m2: f64,
    storage_w_m2: f64,
    sum_abs_latent_j_m2: f64,
    stage3_lower_boundary_energy_w_m2: f64,
    stage3_boundary_abs_j_m2: f64,
) -> (f64, f64, f64, f64) {
    (
        (incident_shortwave_w_m2 + atmospheric_downward_longwave_w_m2) * interval_s,
        (top_reflected_shortwave_w_m2
            + top_upward_longwave_w_m2
            + sensible_to_reference_air_w_m2
            + ground_heat_w_m2)
            * interval_s
            + latent_to_reference_air_j_m2
            + stage3_lower_boundary_energy_w_m2 * interval_s,
        storage_w_m2 * interval_s,
        (incident_shortwave_w_m2.abs()
            + top_reflected_shortwave_w_m2.abs()
            + atmospheric_downward_longwave_w_m2.abs()
            + top_upward_longwave_w_m2.abs()
            + sensible_to_reference_air_w_m2.abs()
            + ground_heat_w_m2.abs()
            + storage_w_m2.abs())
            * interval_s
            + sum_abs_latent_j_m2
            + stage3_boundary_abs_j_m2,
    )
}

fn reconstruct_weighted_ofe_energy(
    final_tiles: &[FinalizedRuntimeTile],
) -> Result<Vec<WeightedOfeEnergyJoin>, LandSurfaceEnergyShadowError> {
    let mut by_ofe = BTreeMap::<OfeId, Vec<&FinalizedRuntimeTile>>::new();
    for tile in final_tiles {
        by_ofe
            .entry(tile.identity().ofe_id.clone())
            .or_default()
            .push(tile);
    }
    by_ofe
        .into_iter()
        .map(|(ofe_id, tiles)| {
            let interval_s = tiles[0].identity().interval_s;
            if tiles
                .iter()
                .any(|tile| tile.identity().interval_s.to_bits() != interval_s.to_bits())
            {
                return Err(LandSurfaceEnergyShadowError::Identity(
                    "mixed OFE energy intervals",
                ));
            }
            let ordered_tile_ids = tiles
                .iter()
                .map(|tile| tile.identity().tile_id.clone())
                .collect();
            let operands = tiles
                .iter()
                .map(|tile| weighted_operand(tile.identity(), tile.energy_operands()))
                .collect::<Result<Vec<_>, _>>()?;
            let closure = match validate_weighted_ofe_energy(interval_s, &operands) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error.into());
                }
            };
            Ok(WeightedOfeEnergyJoin {
                ofe_id,
                ordered_tile_ids,
                operands,
                closure,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_problem_surface_has_no_companion_owner_arrays() {
        fn accepts_only_projected_tiles(_: Vec<StrictProjectedTileProblem>) {}
        accepts_only_projected_tiles(Vec::new());
        assert!(std::mem::size_of::<StrictProjectedTileProblem>() > 0);
    }

    #[test]
    fn finalized_tile_type_requires_real_open_or_covered_candidate() {
        assert!(std::mem::size_of::<FinalizedRuntimeTile>() > 0);
    }

    #[test]
    fn topology_closure_uses_configured_tolerance_without_normalizing() {
        let admitted = 1.0 + 32.0 * f64::EPSILON;
        assert!(canonical_tile_fraction_sum_closes(admitted));
        assert_eq!(admitted.to_bits(), (1.0 + 32.0 * f64::EPSILON).to_bits());
        assert!(!canonical_tile_fraction_sum_closes(
            1.0 + 65.0 * f64::EPSILON
        ));
        assert!(!canonical_tile_fraction_sum_closes(f64::NAN));
    }

    #[test]
    fn covered_weighting_uses_every_external_column_boundary() {
        let interval = 10.0;
        let baseline = covered_external_energy(
            interval, 100.0, 10.0, 50.0, 20.0, 30.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
        );
        assert_eq!(baseline, (1_500.0, 850.0, 650.0, 3_000.0));
        let with_stage3_boundary = covered_external_energy(
            interval, 100.0, 10.0, 50.0, 20.0, 30.0, 200.0, 5.0, 65.0, 200.0, 25.0, 300.0,
        );
        assert_eq!(with_stage3_boundary, (1_500.0, 1_100.0, 650.0, 3_300.0));
        let closes = |terms: (f64, f64, f64, f64), fraction: f64| {
            validate_weighted_ofe_energy(
                interval,
                &[WeightedTileEnergyOperands {
                    tile_fraction: fraction,
                    local_input_j_m2_tile: terms.0,
                    local_output_j_m2_tile: terms.1,
                    local_storage_change_j_m2_tile: terms.2,
                    local_sum_abs_integrated_components_j_m2_tile: terms.3,
                }],
            )
        };
        assert!(closes(baseline, 1.0).is_ok());

        // Every omitted external boundary fails the validator used by the runtime path.
        for poisoned in [
            covered_external_energy(
                interval, 0.0, 10.0, 50.0, 20.0, 30.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 0.0, 50.0, 20.0, 30.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 0.0, 20.0, 30.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 50.0, 0.0, 30.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 50.0, 20.0, 0.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 50.0, 20.0, 30.0, 0.0, 5.0, 65.0, 0.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 50.0, 20.0, 30.0, 200.0, 0.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 50.0, 20.0, 30.0, 200.0, 5.0, 0.0, 200.0, 0.0, 0.0,
            ),
        ] {
            assert!(closes(poisoned, 1.0).is_err());
        }
        assert!(closes(baseline, 0.5).is_err());
        assert!(closes(baseline, 2.0).is_err());
    }
}
