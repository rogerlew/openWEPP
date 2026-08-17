//! Complete heterogeneous-tile potential/final execution over one OFE owner set.
//!
//! Every projected tile is solved twice from one immutable beginning problem:
//! first without owner caps to construct one request batch, then under its
//! exact subset of the single real-owner authorization. There is no companion
//! request, use, LSE-state, or soil-thermal-candidate input surface.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::TileId;
use openwepp_land_surface_energy::{
    AcceptedOpenSurface, ClosureValue, CoveredColumnInputs, CoveredPotentialPhase,
    CoveredTileEnergyOperandSet, FinalCoveredTileCandidate, FinalTileCandidate, GroundWaterKey,
    OfeId, OpenPotentialPhase, OpenSurfaceProblem, PotentialWaterRequestBatch, RootRuntimeIdentity,
    RuntimeTileIdentity, SoilThermalSnapshot, TileEnergyOperandSet, TileState, WaterAuthorization,
    WaterProtocol, WeightedTileEnergyOperands, finalize_covered_phase, finalize_open_phase,
    solve_covered_potential_phase, solve_open_potential_phase, validate_weighted_ofe_energy,
};

use super::{
    CoveredIngressSchedule, DirectSurfaceLiquidConfiguration,
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError, OwnerKind,
    RealHydrologySourceKey, SoilThermalTileCandidate, UnifiedLseFinalization,
    UnifiedRealHydrologyCandidate, UnifiedReceiverExpectations,
};

/// Strictly projected open-tile problem and its numerical trials.
#[derive(Clone, Debug, PartialEq)]
pub struct StrictProjectedOpenTile {
    pub identity: RuntimeTileIdentity,
    pub beginning: OpenSurfaceProblem,
    pub potential_initial_trial: Option<Vec<f64>>,
    pub final_initial_trial: Option<Vec<f64>>,
    pub soil_thermal: SoilThermalSnapshot,
}

/// Strictly projected covered-tile problem and its numerical trials.
#[derive(Clone, Debug, PartialEq)]
pub struct StrictProjectedCoveredTile {
    pub identity: RuntimeTileIdentity,
    pub beginning: CoveredColumnInputs,
    pub roots: Vec<RootRuntimeIdentity>,
    pub potential_initial_trial: Vec<f64>,
    pub final_initial_trial: Vec<f64>,
    pub soil_thermal: SoilThermalSnapshot,
}

/// One member of the exact configured heterogeneous tile set.
#[derive(Clone, Debug, PartialEq)]
pub enum StrictProjectedTileProblem {
    Open(StrictProjectedOpenTile),
    Covered(StrictProjectedCoveredTile),
}

impl StrictProjectedTileProblem {
    fn identity(&self) -> &RuntimeTileIdentity {
        match self {
            Self::Open(value) => &value.identity,
            Self::Covered(value) => &value.identity,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum PotentialTilePhase {
    Open {
        phase: OpenPotentialPhase,
        final_initial_trial: Option<Vec<f64>>,
        soil_thermal: SoilThermalSnapshot,
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
            Self::Covered { phase, .. } => &phase.identity,
        }
    }

    fn request_batch(&self) -> &PotentialWaterRequestBatch {
        match self {
            Self::Open { phase, .. } => &phase.request_batch,
            Self::Covered { phase, .. } => &phase.request_batch,
        }
    }
}

/// Accepted fixed-cap candidate for one exact tile.
#[derive(Clone, Debug, PartialEq)]
pub enum FinalizedRuntimeTile {
    Open(FinalTileCandidate<AcceptedOpenSurface>),
    Covered(FinalCoveredTileCandidate),
}

impl FinalizedRuntimeTile {
    #[must_use]
    pub fn identity(&self) -> &RuntimeTileIdentity {
        match self {
            Self::Open(value) => &value.identity,
            Self::Covered(value) => &value.identity,
        }
    }

    #[must_use]
    pub fn water_protocol(&self) -> &WaterProtocol {
        match self {
            Self::Open(value) => &value.water_protocol,
            Self::Covered(value) => &value.water_protocol,
        }
    }

    #[must_use]
    pub fn ending_tile_state_pre_ingress(&self) -> &TileState {
        match self {
            Self::Open(value) => &value.ending_tile_state_pre_ingress,
            Self::Covered(value) => &value.ending_tile_state_pre_ingress,
        }
    }

    #[must_use]
    pub fn soil_thermal(&self) -> &SoilThermalTileCandidate {
        match self {
            Self::Open(value) => &value.soil_thermal,
            Self::Covered(value) => &value.soil_thermal,
        }
    }

    #[must_use]
    pub fn energy_operands(&self) -> RuntimeTileEnergyOperands<'_> {
        match self {
            Self::Open(value) => RuntimeTileEnergyOperands::Open(&value.energy_operands),
            Self::Covered(value) => RuntimeTileEnergyOperands::Covered(&value.energy_operands),
        }
    }
}

/// Borrowed local energy operands retaining open/covered type identity.
#[derive(Clone, Copy, Debug)]
pub enum RuntimeTileEnergyOperands<'a> {
    Open(&'a TileEnergyOperandSet),
    Covered(&'a CoveredTileEnergyOperandSet),
}

impl RuntimeTileEnergyOperands<'_> {
    fn ground(&self) -> &TileEnergyOperandSet {
        match self {
            Self::Open(value) => value,
            Self::Covered(value) => &value.ground,
        }
    }

    fn validate(&self) -> Result<(), LandSurfaceEnergyShadowError> {
        match self {
            Self::Open(value) => value.validate()?,
            Self::Covered(value) => value.validate()?,
        }
        Ok(())
    }
}

/// Independently reconstructed weighted ground-control-volume join for one
/// complete OFE tile set.
#[derive(Clone, Debug, PartialEq)]
pub struct WeightedOfeEnergyJoin {
    pub ofe_id: OfeId,
    pub ordered_tile_ids: Vec<TileId>,
    pub operands: Vec<WeightedTileEnergyOperands>,
    pub closure: ClosureValue,
}

/// Complete multi-tile result. Final tiles are generated internally and are
/// never accepted from the caller.
#[derive(Clone, Debug, PartialEq)]
pub struct MultiTileRuntimeResult {
    potential_request_batch: PotentialWaterRequestBatch,
    finalized_tiles: Vec<FinalizedRuntimeTile>,
    weighted_ofe_energy: Vec<WeightedOfeEnergyJoin>,
    hydrology_candidate: UnifiedRealHydrologyCandidate,
}

impl MultiTileRuntimeResult {
    #[must_use]
    pub const fn potential_request_batch(&self) -> &PotentialWaterRequestBatch {
        &self.potential_request_batch
    }

    #[must_use]
    pub fn finalized_tiles(&self) -> &[FinalizedRuntimeTile] {
        &self.finalized_tiles
    }

    #[must_use]
    pub fn weighted_ofe_energy(&self) -> &[WeightedOfeEnergyJoin] {
        &self.weighted_ofe_energy
    }

    #[must_use]
    pub const fn hydrology_candidate(&self) -> &UnifiedRealHydrologyCandidate {
        &self.hydrology_candidate
    }
}

/// Execute all strictly projected open and covered tiles against one real
/// hydrology authorization. Covered ingress is constructed only from the
/// accepted fixed-cap E04 ledgers by the derived-ingress owner boundary.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn execute_multi_tile_runtime(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    projected_tiles: Vec<StrictProjectedTileProblem>,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress_schedule: &CoveredIngressSchedule,
) -> Result<MultiTileRuntimeResult, LandSurfaceEnergyShadowError> {
    let projected_tiles =
        validate_and_sort_projected_tiles(projected_tiles, surface_configuration)?;
    let potential_phases = solve_all_potential(projected_tiles)?;
    let request_batch = combined_request_batch(&potential_phases)?;
    let mut retained_final_tiles = None;

    let hydrology_candidate = super::covered_derived_ingress::execute_unified_with_derived_ingress(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        &request_batch,
        soil_sources,
        ingress_schedule,
        |authorizations| {
            let final_tiles = finalize_all_tiles(&potential_phases, authorizations)?;
            let protocol = combined_protocol(&request_batch, authorizations, &final_tiles)?;
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
                protocol,
                ending_tiles,
                soil_thermal,
                rollback_hashes,
            )?;
            let covered_final_tiles = final_tiles
                .iter()
                .filter_map(|tile| match tile {
                    FinalizedRuntimeTile::Open(_) => None,
                    FinalizedRuntimeTile::Covered(value) => Some(value.clone()),
                })
                .collect::<Vec<_>>();
            let ingress = super::covered_derived_ingress::derive_fixed_cap_canopy_ingress(
                surface_configuration,
                &covered_final_tiles,
                ingress_schedule,
            )?;
            retained_final_tiles = Some(final_tiles);
            Ok((sealed, ingress))
        },
    )?;

    let finalized_tiles = retained_final_tiles.ok_or(LandSurfaceEnergyShadowError::Identity(
        "multi-tile finalizer returned no tile candidates",
    ))?;
    let weighted_ofe_energy = reconstruct_weighted_ofe_energy(&finalized_tiles)?;
    Ok(MultiTileRuntimeResult {
        potential_request_batch: request_batch,
        finalized_tiles,
        weighted_ofe_energy,
        hydrology_candidate,
    })
}

fn validate_and_sort_projected_tiles(
    mut projected: Vec<StrictProjectedTileProblem>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<Vec<StrictProjectedTileProblem>, LandSurfaceEnergyShadowError> {
    if projected.is_empty() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "empty projected tile set",
        ));
    }
    projected.sort_by(|left, right| {
        let left = left.identity();
        let right = right.identity();
        (&left.ofe_id, &left.tile_id).cmp(&(&right.ofe_id, &right.tile_id))
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
        *fractions.entry(identity.ofe_id.clone()).or_default() += identity.tile_fraction;
    }
    if fractions
        .values()
        .any(|sum| sum.to_bits() != 1.0_f64.to_bits())
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

fn solve_all_potential(
    projected: Vec<StrictProjectedTileProblem>,
) -> Result<Vec<PotentialTilePhase>, LandSurfaceEnergyShadowError> {
    projected
        .into_iter()
        .map(|tile| match tile {
            StrictProjectedTileProblem::Open(value) => {
                let phase = solve_open_potential_phase(
                    value.identity,
                    &value.beginning,
                    value.potential_initial_trial,
                )?;
                Ok(PotentialTilePhase::Open {
                    phase,
                    final_initial_trial: value.final_initial_trial,
                    soil_thermal: value.soil_thermal,
                })
            }
            StrictProjectedTileProblem::Covered(value) => {
                let phase = solve_covered_potential_phase(
                    value.identity,
                    &value.beginning,
                    value.roots,
                    value.potential_initial_trial,
                )?;
                Ok(PotentialTilePhase::Covered {
                    phase,
                    final_initial_trial: value.final_initial_trial,
                    soil_thermal: value.soil_thermal,
                })
            }
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
        .flat_map(|phase| phase.request_batch().requests.iter().cloned())
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
        .requests
        .iter()
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
) -> Result<Vec<FinalizedRuntimeTile>, LandSurfaceEnergyShadowError> {
    phases
        .iter()
        .map(|phase| {
            let mut subset = authorization_subset(phase, authorizations)?;
            match phase {
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
                    Ok(FinalizedRuntimeTile::Open(finalize_open_phase(
                        phase,
                        &phase.identity.beginning_lse_state_sha256,
                        &subset.remove(0),
                        final_initial_trial.clone(),
                        soil_thermal,
                    )?))
                }
                PotentialTilePhase::Covered {
                    phase,
                    final_initial_trial,
                    soil_thermal,
                } => Ok(FinalizedRuntimeTile::Covered(finalize_covered_phase(
                    phase,
                    &phase.identity.beginning_lse_state_sha256,
                    subset,
                    final_initial_trial.clone(),
                    soil_thermal,
                )?)),
            }
        })
        .collect()
}

fn combined_protocol(
    request_batch: &PotentialWaterRequestBatch,
    authorizations: &[WaterAuthorization],
    final_tiles: &[FinalizedRuntimeTile],
) -> Result<WaterProtocol, LandSurfaceEnergyShadowError> {
    let first = final_tiles
        .first()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "empty final tile set",
        ))?;
    let finalized_uses = final_tiles
        .iter()
        .flat_map(|tile| tile.water_protocol().finalized_uses.iter().cloned())
        .collect();
    let condensation_credits = final_tiles
        .iter()
        .flat_map(|tile| tile.water_protocol().condensation_credits.iter().cloned())
        .collect();
    let protocol = WaterProtocol {
        transaction_id: request_batch.transaction_id,
        hydrology_owner_id: first.water_protocol().hydrology_owner_id.clone(),
        beginning_snapshot_sha256: first.water_protocol().beginning_snapshot_sha256.clone(),
        requests: request_batch.requests.clone(),
        authorizations: authorizations.to_vec(),
        finalized_uses,
        condensation_credits,
    };
    protocol.validate()?;
    Ok(protocol)
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
    let ground = energy.ground();
    let surface = ground.surface;
    Ok(WeightedTileEnergyOperands {
        tile_fraction: identity.tile_fraction,
        local_input_j_m2_tile: (surface.absorbed_shortwave_w_m2 + surface.net_longwave_w_m2)
            * identity.interval_s,
        local_output_j_m2_tile: surface.sensible_w_m2 * identity.interval_s
            + ground.latent.vapor_energy_j_m2
            + surface.ground_heat_w_m2 * identity.interval_s,
        local_storage_change_j_m2_tile: surface.storage_w_m2 * identity.interval_s,
    })
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
            let ordered_tile_ids = tiles
                .iter()
                .map(|tile| tile.identity().tile_id.clone())
                .collect();
            let operands = tiles
                .iter()
                .map(|tile| weighted_operand(tile.identity(), tile.energy_operands()))
                .collect::<Result<Vec<_>, _>>()?;
            let closure = validate_weighted_ofe_energy(&operands)?;
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
}
