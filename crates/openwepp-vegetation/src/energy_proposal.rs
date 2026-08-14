//! Immutable component-level energy proposals for an independent owner.
//!
//! This module carries solved physical operands and exact radiation boundaries.
//! It never carries or accepts a producer-supplied energy residual.

use std::collections::BTreeSet;

use openwepp_kernel_contract::{OccupancyId, TileId, TransactionId};

use crate::VegetationError;
use crate::column::OccupancyEnergyProposal;
use crate::config::VegetationConfiguration;
use crate::diagnostics::CoupledSolvePass;
use crate::radiation::{ColumnRadiationResult, IncidentComponent, RadiationBand};
use crate::transaction::CoupledOwnedState;
use crate::water_phase::UncommittedWaterPhase;

#[derive(Clone, Debug, PartialEq)]
pub struct EnergyProposalIdentity {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub beginning_state_sha256: String,
    pub ending_state_sha256: String,
    pub transaction_id: TransactionId,
    pub interval_s: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RadiationBoundaryProposal {
    pub band: RadiationBand,
    pub component: IncidentComponent,
    pub incident_w_m2_tile: f64,
    pub reflected_w_m2_tile: f64,
    pub terminal_direct_w_m2_tile: f64,
    pub terminal_diffuse_w_m2_tile: f64,
    pub ground_albedo: f64,
    pub ground_absorbed_w_m2_tile: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TileEnergyBoundaryProposal {
    pub tile_id: TileId,
    pub tile_fraction: f64,
    pub components: Vec<RadiationBoundaryProposal>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnergyProposalBatch {
    pub identity: EnergyProposalIdentity,
    pub occupancies: Vec<OccupancyEnergyProposal>,
    pub tile_boundaries: Vec<TileEnergyBoundaryProposal>,
}

pub fn construct_energy_proposal_batch(
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    ending_state_sha256: &str,
    water_phase: &UncommittedWaterPhase,
) -> Result<EnergyProposalBatch, VegetationError> {
    configuration.validate()?;
    beginning.validate(configuration)?;
    if water_phase.beginning_state_sha256() != beginning.state_sha256
        || beginning
            .last_transaction_id
            .checked_add(1)
            .is_none_or(|expected| water_phase.transaction_id().0 != expected)
        || ending_state_sha256.len() != 64
        || !ending_state_sha256
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(VegetationError::V7CandidateRollback(
            "energy proposal phase lineage",
        ));
    }
    let expected = configuration.expected_occupancies();
    let mut actual = BTreeSet::<OccupancyId>::new();
    let mut occupancies = Vec::with_capacity(expected.len());
    for column in &water_phase.final_columns().columns {
        let tile_fraction = configuration
            .topology_tiles
            .iter()
            .find(|tile| tile.tile_id == column.tile_id)
            .ok_or(VegetationError::V7CandidateRollback(
                "energy proposal tile identity",
            ))?
            .fraction;
        for result in &column.occupancy_results {
            let proposal =
                result
                    .energy_proposal
                    .clone()
                    .ok_or(VegetationError::V7CandidateRollback(
                        "missing capped occupancy energy proposal",
                    ))?;
            if result.diagnostics.pass != CoupledSolvePass::Capped
                || proposal.transaction_id != water_phase.transaction_id()
                || proposal.occupancy_id != result.occupancy_id
                || proposal.occupancy_id.tile_id != column.tile_id
                || proposal.tile_fraction.to_bits() != tile_fraction.to_bits()
                || proposal.interval_s.to_bits() != water_phase.interval_s().to_bits()
                || !actual.insert(proposal.occupancy_id.clone())
            {
                return Err(VegetationError::V7CandidateRollback(
                    "capped occupancy energy identity",
                ));
            }
            occupancies.push(proposal);
        }
    }
    if actual != expected {
        return Err(VegetationError::V7CandidateRollback(
            "energy proposal occupancy set",
        ));
    }
    occupancies.sort_by(|left, right| left.occupancy_id.cmp(&right.occupancy_id));

    let mut tile_boundaries = Vec::with_capacity(configuration.topology_tiles.len());
    for tile in &configuration.topology_tiles {
        let radiation = water_phase
            .final_radiation()
            .columns
            .get(&tile.tile_id)
            .ok_or(VegetationError::V7CandidateRollback(
                "energy radiation tile identity",
            ))?;
        let components = [
            &radiation.visible_direct,
            &radiation.visible_diffuse,
            &radiation.near_infrared_direct,
            &radiation.near_infrared_diffuse,
        ]
        .into_iter()
        .map(boundary_component)
        .collect::<Result<Vec<_>, _>>()?;
        tile_boundaries.push(TileEnergyBoundaryProposal {
            tile_id: tile.tile_id.clone(),
            tile_fraction: tile.fraction,
            components,
        });
    }
    Ok(EnergyProposalBatch {
        identity: EnergyProposalIdentity {
            model_definition_sha256: beginning.model_definition_sha256.clone(),
            configuration_sha256: beginning.configuration_sha256.clone(),
            beginning_state_sha256: beginning.state_sha256.clone(),
            ending_state_sha256: ending_state_sha256.into(),
            transaction_id: water_phase.transaction_id(),
            interval_s: water_phase.interval_s(),
        },
        occupancies,
        tile_boundaries,
    })
}

fn boundary_component(
    result: &ColumnRadiationResult,
) -> Result<RadiationBoundaryProposal, VegetationError> {
    let values = [
        result.incident,
        result.top_reflected,
        result.terminal_direct,
        result.terminal_diffuse,
        result.ground_albedo,
        result.ground_absorbed,
    ];
    if values
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
    {
        return Err(VegetationError::V7CandidateRollback(
            "energy radiation boundary operand",
        ));
    }
    Ok(RadiationBoundaryProposal {
        band: result.band,
        component: result.component,
        incident_w_m2_tile: result.incident,
        reflected_w_m2_tile: result.top_reflected,
        terminal_direct_w_m2_tile: result.terminal_direct,
        terminal_diffuse_w_m2_tile: result.terminal_diffuse,
        ground_albedo: result.ground_albedo,
        ground_absorbed_w_m2_tile: result.ground_absorbed,
    })
}
