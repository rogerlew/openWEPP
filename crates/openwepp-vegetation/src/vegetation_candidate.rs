//! Sealed V7 vegetation-owner candidate construction.
//!
//! The candidate contains an accepted-shape vegetation state plus immutable
//! owner protocols, material proposals, and independently validated vegetation
//! ledgers. It deliberately has no commit method and is not publicly exposed.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, ResourceOwnerId, StratumId, TransactionId};

use crate::VegetationError;
use crate::carbon_nitrogen::{MaterialTransfer, Tissue};
use crate::config::{StratumConfiguration, VegetationConfiguration};
use crate::diagnostics::CoupledSolvePass;
use crate::persistent_phase::UncommittedNitrogenPhase;
use crate::transaction::{CoupledOwnedState, StratumSharedState};
use crate::vegetation_ledger::{
    VegetationCarbonLedger, VegetationDryMaterialLedger, VegetationDryMaterialTransferOperand,
    VegetationLedgerIdentity, VegetationNitrogenLedger, validate_vegetation_ledgers,
};
use crate::water_phase::UncommittedWaterPhase;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UncommittedVegetationCandidate {
    transaction_id: TransactionId,
    beginning_state_sha256: String,
    ending_state: CoupledOwnedState,
    water_phase: UncommittedWaterPhase,
    nitrogen_phase: UncommittedNitrogenPhase,
    material_proposals: Vec<MaterialTransfer>,
    carbon_ledgers: Vec<VegetationCarbonLedger>,
    nitrogen_ledgers: Vec<VegetationNitrogenLedger>,
    dry_material_ledgers: Vec<VegetationDryMaterialLedger>,
}

impl UncommittedVegetationCandidate {
    #[cfg(test)]
    pub(crate) fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[cfg(test)]
    pub(crate) fn beginning_state_sha256(&self) -> &str {
        &self.beginning_state_sha256
    }

    #[cfg(test)]
    pub(crate) fn ending_state(&self) -> &CoupledOwnedState {
        &self.ending_state
    }

    #[cfg(test)]
    pub(crate) fn water_phase(&self) -> &UncommittedWaterPhase {
        &self.water_phase
    }

    #[cfg(test)]
    pub(crate) fn nitrogen_phase(&self) -> &UncommittedNitrogenPhase {
        &self.nitrogen_phase
    }

    #[cfg(test)]
    pub(crate) fn material_proposals(&self) -> &[MaterialTransfer] {
        &self.material_proposals
    }

    #[cfg(test)]
    pub(crate) fn carbon_ledgers(&self) -> &[VegetationCarbonLedger] {
        &self.carbon_ledgers
    }

    #[cfg(test)]
    pub(crate) fn nitrogen_ledgers(&self) -> &[VegetationNitrogenLedger] {
        &self.nitrogen_ledgers
    }

    #[cfg(test)]
    pub(crate) fn dry_material_ledgers(&self) -> &[VegetationDryMaterialLedger] {
        &self.dry_material_ledgers
    }

    pub(crate) fn validate_sealed(&self) -> Result<(), VegetationError> {
        if self.transaction_id != self.water_phase.transaction_id()
            || self.transaction_id != self.nitrogen_phase.transaction_id()
            || self.beginning_state_sha256 != self.water_phase.beginning_state_sha256()
            || self.ending_state.last_transaction_id != self.transaction_id.0
            || self.material_proposals.iter().any(|proposal| {
                proposal.transaction_id != self.transaction_id.0 || proposal.proposal_id == 0
            })
        {
            return Err(VegetationError::V7CandidateRollback(
                "sealed owner identity",
            ));
        }
        let expected_strata = self
            .ending_state
            .strata
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        validate_vegetation_ledgers(
            &expected_strata,
            self.transaction_id,
            &self.beginning_state_sha256,
            &self.ending_state.state_sha256,
            &self.carbon_ledgers,
            &self.nitrogen_ledgers,
            &self.dry_material_ledgers,
        )
    }
}

pub(crate) fn construct_uncommitted_vegetation_candidate(
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    water_phase: &UncommittedWaterPhase,
    nitrogen_phase: &UncommittedNitrogenPhase,
) -> Result<UncommittedVegetationCandidate, VegetationError> {
    configuration.validate()?;
    beginning.validate(configuration)?;
    let transaction_id = water_phase.transaction_id();
    let expected_transaction_id = beginning.last_transaction_id.checked_add(1).ok_or(
        VegetationError::V7CandidateRollback("transaction identity overflow"),
    )?;
    if nitrogen_phase.transaction_id() != transaction_id
        || nitrogen_phase.source_water_phase() != water_phase
        || water_phase.beginning_state_sha256() != beginning.state_sha256
        || transaction_id.0 != expected_transaction_id
    {
        return Err(VegetationError::V7CandidateRollback(
            "phase or beginning-state identity mismatch",
        ));
    }

    let material_proposals = bind_material_proposals(transaction_id, nitrogen_phase)?;
    let ending_strata = construct_ending_strata(configuration, transaction_id, nitrogen_phase)?;
    let ending_occupancies = construct_ending_occupancies(
        configuration,
        beginning,
        transaction_id,
        water_phase.final_columns(),
    )?;
    let mut ending_state = CoupledOwnedState {
        model_definition_sha256: beginning.model_definition_sha256.clone(),
        configuration_sha256: beginning.configuration_sha256.clone(),
        state_sha256: String::new(),
        strata: ending_strata,
        occupancies: ending_occupancies,
        last_transaction_id: transaction_id.0,
    };
    ending_state.state_sha256 = ending_state.canonical_sha256()?;
    ending_state.validate(configuration)?;

    let (carbon_ledgers, nitrogen_ledgers, dry_material_ledgers) = construct_ledgers(
        configuration,
        beginning,
        &ending_state,
        nitrogen_phase,
        &material_proposals,
    )?;
    let expected_strata = configuration
        .strata
        .iter()
        .map(|stratum| stratum.stratum_id.clone())
        .collect::<BTreeSet<_>>();
    validate_vegetation_ledgers(
        &expected_strata,
        transaction_id,
        &beginning.state_sha256,
        &ending_state.state_sha256,
        &carbon_ledgers,
        &nitrogen_ledgers,
        &dry_material_ledgers,
    )?;

    Ok(UncommittedVegetationCandidate {
        transaction_id,
        beginning_state_sha256: beginning.state_sha256.clone(),
        ending_state,
        water_phase: water_phase.clone(),
        nitrogen_phase: nitrogen_phase.clone(),
        material_proposals,
        carbon_ledgers,
        nitrogen_ledgers,
        dry_material_ledgers,
    })
}

fn construct_ending_strata(
    configuration: &VegetationConfiguration,
    transaction_id: TransactionId,
    nitrogen_phase: &UncommittedNitrogenPhase,
) -> Result<BTreeMap<StratumId, StratumSharedState>, VegetationError> {
    let expected = configuration
        .strata
        .iter()
        .map(|stratum| stratum.stratum_id.clone())
        .collect::<BTreeSet<_>>();
    if nitrogen_phase
        .strata()
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        != expected
    {
        return Err(VegetationError::V7CandidateRollback(
            "stratum identity mismatch",
        ));
    }
    configuration
        .strata
        .iter()
        .map(|stratum| {
            let preallocation = nitrogen_phase.strata().get(&stratum.stratum_id).ok_or(
                VegetationError::V7CandidateRollback("vegetation candidate stratum"),
            )?;
            let mut candidate = preallocation.candidate_after_growth.clone();
            if !candidate.pending_transfers.is_empty() {
                return Err(VegetationError::V7Candidate("unresolved material transfer"));
            }
            update_derived_areas(&mut candidate, stratum)?;
            candidate.last_transaction_id = transaction_id.0;
            Ok((stratum.stratum_id.clone(), candidate))
        })
        .collect()
}

fn update_derived_areas(
    state: &mut StratumSharedState,
    configuration: &StratumConfiguration,
) -> Result<(), VegetationError> {
    let leaf = state
        .tissues
        .get(&Tissue::Leaf)
        .ok_or(VegetationError::V7Candidate(
            "vegetation candidate leaf tissue",
        ))?;
    (state.leaf_area, state.stem_area, state.root_area) =
        crate::transaction::displayed_leaf_derived_areas(leaf.display.carbon, configuration)
            .map_err(|_| VegetationError::V7Candidate("displayed-leaf area identity"))?;
    Ok(())
}

fn construct_ending_occupancies(
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    transaction_id: TransactionId,
    final_columns: &crate::column::TileColumnsResult,
) -> Result<BTreeMap<OccupancyId, crate::occupancy_state::OccupancyState>, VegetationError> {
    let expected = configuration.expected_occupancies();
    let mut ending = BTreeMap::new();
    for column in &final_columns.columns {
        for result in &column.occupancy_results {
            let beginning_lane = beginning.occupancies.get(&result.occupancy_id).ok_or(
                VegetationError::CappedCandidateRollback("vegetation candidate occupancy identity"),
            )?;
            if result.occupancy_id.tile_id != column.tile_id
                || result.diagnostics.pass != CoupledSolvePass::Capped
                || result.candidate_state.last_accepted_transaction_id
                    != beginning_lane.last_accepted_transaction_id
            {
                return Err(VegetationError::CappedCandidateRollback(
                    "final occupancy result identity",
                ));
            }
            let mut lane = result.candidate_state.clone();
            lane.last_accepted_transaction_id = Some(transaction_id.0);
            lane.validate(Some(transaction_id.0)).map_err(|_| {
                VegetationError::CappedCandidateRollback("invalid final occupancy state")
            })?;
            if ending.insert(result.occupancy_id.clone(), lane).is_some() {
                return Err(VegetationError::CappedCandidateRollback(
                    "duplicate final occupancy",
                ));
            }
        }
    }
    if ending.keys().cloned().collect::<BTreeSet<_>>() != expected {
        return Err(VegetationError::CappedCandidateRollback(
            "final occupancy set mismatch",
        ));
    }
    Ok(ending)
}

fn bind_material_proposals(
    transaction_id: TransactionId,
    nitrogen_phase: &UncommittedNitrogenPhase,
) -> Result<Vec<MaterialTransfer>, VegetationError> {
    let mut amounts = nitrogen_phase
        .strata()
        .iter()
        .flat_map(|(stratum_id, preallocation)| {
            preallocation
                .material_transfers
                .iter()
                .copied()
                .enumerate()
                .map(move |(source_sequence, amount)| (stratum_id.clone(), source_sequence, amount))
        })
        .collect::<Vec<_>>();
    amounts.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| left.2.donor().cmp(&right.2.donor()))
            .then_with(|| left.2.receiver().cmp(&right.2.receiver()))
            .then_with(|| left.1.cmp(&right.1))
    });
    amounts
        .into_iter()
        .enumerate()
        .map(|(index, (stratum_id, _, amount))| {
            let proposal_id = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(VegetationError::V7CandidateRollback(
                    "material proposal identity overflow",
                ))?;
            let owner_id = ResourceOwnerId::try_new(format!("stratum:{}", stratum_id.as_str()))
                .map_err(|_| VegetationError::V7CandidateRollback("material proposal owner"))?;
            amount.bind(transaction_id, &owner_id, proposal_id)
        })
        .collect()
}

type CandidateLedgers = (
    Vec<VegetationCarbonLedger>,
    Vec<VegetationNitrogenLedger>,
    Vec<VegetationDryMaterialLedger>,
);

fn construct_ledgers(
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    ending: &CoupledOwnedState,
    nitrogen_phase: &UncommittedNitrogenPhase,
    proposals: &[MaterialTransfer],
) -> Result<CandidateLedgers, VegetationError> {
    let mut carbon_ledgers = Vec::new();
    let mut nitrogen_ledgers = Vec::new();
    let mut dry_material_ledgers = Vec::new();
    for stratum in &configuration.strata {
        let before = beginning.strata.get(&stratum.stratum_id).ok_or(
            VegetationError::V7CandidateRollback("vegetation ledger beginning stratum"),
        )?;
        let after =
            ending
                .strata
                .get(&stratum.stratum_id)
                .ok_or(VegetationError::V7CandidateRollback(
                    "vegetation ledger ending stratum",
                ))?;
        let preallocation = nitrogen_phase.strata().get(&stratum.stratum_id).ok_or(
            VegetationError::V7CandidateRollback("vegetation ledger preallocation"),
        )?;
        let owner = format!("stratum:{}", stratum.stratum_id.as_str());
        let stratum_proposals = proposals
            .iter()
            .filter(|proposal| proposal.owner_id.as_str() == owner)
            .collect::<Vec<_>>();
        let outgoing_carbon = stratum_proposals.iter().map(|value| value.carbon).sum();
        let outgoing_nitrogen = stratum_proposals.iter().map(|value| value.nitrogen).sum();
        let identity = VegetationLedgerIdentity {
            transaction_id: nitrogen_phase.transaction_id(),
            stratum_id: stratum.stratum_id.clone(),
            beginning_state_sha256: beginning.state_sha256.clone(),
            ending_state_sha256: ending.state_sha256.clone(),
        };
        carbon_ledgers.push(VegetationCarbonLedger {
            identity: identity.clone(),
            beginning_physical_vegetation_kg_c_m2: total_physical_carbon(before),
            beginning_xs_c_kg_c_m2: before.xs_c,
            gross_primary_production_kg_c_m2: preallocation
                .final_carbon_operands
                .gross_primary_production_kg_c_m2,
            maintenance_respiration_kg_c_m2: preallocation.final_maintenance_respiration_kg_c_m2,
            growth_respiration_kg_c_m2: preallocation.growth_finalization.growth_respiration,
            outgoing_material_kg_c_m2: outgoing_carbon,
            ending_physical_vegetation_kg_c_m2: total_physical_carbon(after),
            ending_xs_c_kg_c_m2: after.xs_c,
        });
        nitrogen_ledgers.push(VegetationNitrogenLedger {
            identity: identity.clone(),
            beginning_vegetation_kg_n_m2: total_nitrogen(before),
            finalized_external_mineral_n_kg_m2: preallocation.nitrogen_finalization.external_use,
            outgoing_material_kg_n_m2: outgoing_nitrogen,
            ending_vegetation_kg_n_m2: total_nitrogen(after),
        });
        let transfers = stratum_proposals
            .iter()
            .map(|proposal| VegetationDryMaterialTransferOperand {
                proposal_id: proposal.proposal_id,
                donor: proposal.donor,
                receiver: proposal.receiver,
                carbon_kg_m2: proposal.carbon,
                nitrogen_kg_m2: proposal.nitrogen,
                drymatter_carbon_fraction: stratum.drymatter_carbon_fraction,
                proposed_dry_matter_kg_m2: proposal.dry_matter,
            })
            .collect::<Vec<_>>();
        dry_material_ledgers.push(VegetationDryMaterialLedger {
            identity,
            outgoing_dry_matter_kg_m2: transfers
                .iter()
                .map(|transfer| transfer.proposed_dry_matter_kg_m2)
                .sum(),
            transfers,
        });
    }
    Ok((carbon_ledgers, nitrogen_ledgers, dry_material_ledgers))
}

fn total_physical_carbon(state: &StratumSharedState) -> f64 {
    state
        .tissues
        .values()
        .map(|pool| pool.display.carbon + pool.storage.carbon + pool.transfer.carbon)
        .sum::<f64>()
        + state.nsc_c
        + state.standing_dead.carbon
}

fn total_nitrogen(state: &StratumSharedState) -> f64 {
    state
        .tissues
        .values()
        .map(|pool| pool.display.nitrogen + pool.storage.nitrogen + pool.transfer.nitrogen)
        .sum::<f64>()
        + state.retranslocation_n
        + state.standing_dead.nitrogen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_occupancy_set_rejects_missing_and_potential_only_results() {
        let (configuration, beginning) = crate::transaction::v7_identity_rebound_fixture();
        let mut columns = crate::column::TileColumnsResult {
            columns: Vec::new(),
        };
        assert!(
            construct_ending_occupancies(&configuration, &beginning, TransactionId(1), &columns,)
                .is_err()
        );

        let occupancy_id = configuration
            .expected_occupancies()
            .into_iter()
            .next()
            .expect("occupancy");
        let beginning_lane = beginning.occupancies[&occupancy_id].clone();
        columns.columns.push(crate::column::TileColumnResult {
            tile_id: occupancy_id.tile_id.clone(),
            occupancy_results: vec![crate::column::RoutedOccupancyResult {
                occupancy_id,
                candidate_state: beginning_lane,
                liquid: crate::interception::InterceptionResult {
                    store1: 0.0,
                    evaporation: 0.0,
                    condensation: 0.0,
                    throughfall: 0.0,
                    stemflow: 0.0,
                    initial_drainage: 0.0,
                    second_drainage: 0.0,
                    wet_fraction: 0.0,
                    closure_residual: 0.0,
                },
                release: crate::column::OccupancyLiquidRelease {
                    throughfall_kg_m2_tile_ground: 0.0,
                    initial_drainage_kg_m2_tile_ground: 0.0,
                    second_drainage_kg_m2_tile_ground: 0.0,
                    stemflow_kg_m2_tile_ground: 0.0,
                },
                stand_ground_layer_water_kg_m2: BTreeMap::new(),
                carbon_operands: None,
                diagnostics: crate::column::OccupancyDiagnostics {
                    pass: CoupledSolvePass::Potential,
                    ci_iterations_sun: 0,
                    ci_iterations_shade: 0,
                    energy_iterations: 0,
                    hydraulic_iterations: 0,
                    outer_iterations: 0,
                    normalized_residuals: Vec::new(),
                    temperature_step_k: None,
                    potential_step_mm: None,
                    backtracking_count: 0,
                    wet_store_cap_active: false,
                    active_water_caps: Vec::new(),
                    gas_hydraulic_mismatch_kg_m2_s: 0.0,
                    vulnerability_demand_sun_kg_m2_s: None,
                    vulnerability_demand_shade_kg_m2_s: None,
                    pivot_magnitude: None,
                    matrix_norm: None,
                    advanced_t10_k: None,
                    capped_operands: None,
                },
            }],
            ground_throughfall_kg_m2_tile_ground: 0.0,
            ground_drainage_kg_m2_tile_ground: 0.0,
            ground_stemflow_kg_m2_tile_ground: 0.0,
            ledger: crate::column::TileLiquidLedger {
                tile_id: configuration.topology_tiles[0].tile_id.clone(),
                tile_fraction: configuration.topology_tiles[0].fraction,
                top_rain_kg_m2_tile_ground: 0.0,
                occupancies: Vec::new(),
                ground_throughfall_kg_m2_tile_ground: 0.0,
                ground_drainage_kg_m2_tile_ground: 0.0,
                ground_stemflow_kg_m2_tile_ground: 0.0,
            },
        });
        assert!(
            construct_ending_occupancies(&configuration, &beginning, TransactionId(1), &columns,)
                .is_err()
        );

        columns.columns[0].occupancy_results[0].diagnostics.pass = CoupledSolvePass::Capped;
        let accepted =
            construct_ending_occupancies(&configuration, &beginning, TransactionId(1), &columns)
                .expect("complete capped occupancy set");
        assert!(
            accepted
                .values()
                .all(|lane| lane.last_accepted_transaction_id == Some(1))
        );

        let duplicate = columns.columns[0].occupancy_results[0].clone();
        columns.columns[0].occupancy_results.push(duplicate);
        assert!(
            construct_ending_occupancies(&configuration, &beginning, TransactionId(1), &columns,)
                .is_err()
        );
    }
}
