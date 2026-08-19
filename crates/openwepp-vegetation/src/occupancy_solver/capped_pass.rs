//! V5 fixed-authorization column-pass orchestration and finalized water uses.
//!
//! This module deliberately starts the second column pass from the immutable
//! beginning owner state. It binds the same whole-column radiation preparation
//! used by Stage A, supplies exact occupancy-local caps derived from typed
//! stand-ground authorizations, and routes each final occupancy release before
//! solving its descendant. It does not consume Stage-A candidate states.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{
    FinalizedUse, OccupancyId, ResourceAmountBasis, TileId, TransactionId, WaterResourceKey,
    validate_resource_protocol,
};

use crate::VegetationError;
use crate::column::{
    ColumnPassKind, OccupancyDiagnostics, OccupancyPassInput, OccupancyPassResult,
    OccupancyPassSolver, TileColumnsResult, execute_tile_columns,
};
use crate::config::VegetationConfiguration;
use crate::occupancy_solver::radiation::{
    OccupancyRadiation, PreparedRadiation, prepare_whole_column_radiation,
};
use crate::occupancy_solver::resources::{
    ValidatedWaterAuthorizations, WaterResourceBoundaryError,
};
use crate::occupancy_state::OccupancyState;
use crate::transaction::validate_candidate_inputs;
use crate::transaction::{CoupledOwnedState, SnowFreeForcing};

const WATER_STAND_BASIS: ResourceAmountBasis =
    ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval;

pub(crate) type FinalizedWaterUse = FinalizedUse<WaterResourceKey, f64>;

/// Complete fixed-cap pass result. Occupancy candidates remain uncommitted;
/// finalized uses retain the exact authorization identity and amount basis.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CappedColumnPass {
    pub columns: TileColumnsResult,
    pub finalized_water_uses: Vec<FinalizedWaterUse>,
    pub radiation: PreparedRadiation,
    pub diagnostics: BTreeMap<OccupancyId, OccupancyDiagnostics>,
    pub water_operands: Vec<CappedWaterLayerOperands>,
}

/// Complete V5 success-path ownership and constitutive operands in configured
/// occupancy/layer order. These are independently reconstructable; no closure
/// boolean or producer residual is accepted.
#[derive(Clone, Debug, PartialEq)]
pub struct CappedWaterLayerOperands {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub beginning_state_sha256: String,
    pub transaction_id: TransactionId,
    pub tile_fraction: f64,
    pub interval_s: f64,
    pub configured_layer_index: usize,
    pub key: WaterResourceKey,
    pub potential_request_kg_m2_stand_ground: f64,
    pub authorization_kg_m2_stand_ground: f64,
    pub authorization_kg_m2_tile_ground: f64,
    pub cap_rate_kg_m2_tile_s: f64,
    pub q_law_kg_m2_tile_s: f64,
    pub q_final_kg_m2_tile_s: f64,
    pub finalized_use_kg_m2_stand_ground: f64,
    pub authorization_active_or_tie: bool,
    pub vulnerability_demand_sun_kg_m2_s: f64,
    pub vulnerability_demand_shade_kg_m2_s: f64,
    pub beginning_occupancy_state: OccupancyState,
    pub candidate_occupancy_state: OccupancyState,
    pub coupled: crate::diagnostics::CappedNumericalOperands,
}

/// Constitutive boundary for one exact V5 occupancy under owner-fixed caps.
/// The evaluator receives local tile-ground caps through [`OccupancyPassInput`]
/// and must solve the complete coupled system from the supplied beginning lane.
pub(crate) trait CappedOccupancyEvaluator {
    fn solve_capped(
        &self,
        input: OccupancyPassInput<'_>,
        radiation: &OccupancyRadiation,
    ) -> Result<OccupancyPassResult, VegetationError>;
}

struct RadiationBoundCappedSolver<'a> {
    radiation: &'a BTreeMap<OccupancyId, OccupancyRadiation>,
    evaluator: &'a dyn CappedOccupancyEvaluator,
    owner_id: &'a openwepp_kernel_contract::ResourceOwnerId,
    transaction_id: TransactionId,
}

impl RadiationBoundCappedSolver<'_> {
    fn try_new<'a>(
        configuration: &VegetationConfiguration,
        prepared: &'a PreparedRadiation,
        evaluator: &'a dyn CappedOccupancyEvaluator,
        owner_id: &'a openwepp_kernel_contract::ResourceOwnerId,
        transaction_id: TransactionId,
    ) -> Result<RadiationBoundCappedSolver<'a>, VegetationError> {
        let expected = configuration.expected_occupancies();
        let actual = prepared
            .occupancies
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        if actual != expected
            || prepared
                .occupancies
                .iter()
                .any(|(key, value)| key != &value.occupancy_id)
        {
            return Err(VegetationError::Receipt(
                "V5 capped radiation occupancy identity".into(),
            ));
        }
        Ok(RadiationBoundCappedSolver {
            radiation: &prepared.occupancies,
            evaluator,
            owner_id,
            transaction_id,
        })
    }
}

impl OccupancyPassSolver for RadiationBoundCappedSolver<'_> {
    fn solve(&self, input: OccupancyPassInput<'_>) -> Result<OccupancyPassResult, VegetationError> {
        let Some(local_caps) = input.local_authorizations_kg_m2_tile_ground.as_ref() else {
            return Err(VegetationError::Receipt(
                "owner authorization absent during V5 capped pass".into(),
            ));
        };
        let expected_layers = input
            .stratum_config
            .root_layers
            .iter()
            .map(|root| root.layer_id.clone())
            .collect::<BTreeSet<_>>();
        let actual_layers = local_caps.keys().cloned().collect::<BTreeSet<_>>();
        if actual_layers != expected_layers
            || local_caps
                .values()
                .any(|amount| !amount.is_finite() || *amount < 0.0)
        {
            return Err(VegetationError::Receipt(
                "V5 capped local authorization identity".into(),
            ));
        }
        let radiation = self.radiation.get(input.occupancy_id).ok_or_else(|| {
            VegetationError::Receipt("V5 capped radiation occupancy identity".into())
        })?;
        if radiation.occupancy_id != *input.occupancy_id
            || radiation.conditional_lai_m2_m2_tile_ground.to_bits()
                != input.conditional_lai_m2_m2_tile_ground.to_bits()
            || radiation.conditional_wai_m2_m2_tile_ground.to_bits()
                != input.conditional_wai_m2_m2_tile_ground.to_bits()
        {
            return Err(VegetationError::Receipt(
                "V5 capped radiation area/occupancy identity".into(),
            ));
        }
        let occupancy_id = input.occupancy_id.clone();
        self.evaluator
            .solve_capped(input, radiation)
            .map_err(|error| {
                bind_fixed_authorization_failure(
                    error,
                    crate::diagnostics::FixedAuthorizationIdentity {
                        transaction_id: self.transaction_id,
                        owner_id: self.owner_id.clone(),
                        occupancy_id,
                        basis: WATER_STAND_BASIS,
                    },
                )
            })
    }
}

/// Attaches the already validated owner authorization identity to a rejected
/// constitutive solve without changing its typed category or numerical payload.
pub(crate) fn bind_fixed_authorization_failure(
    error: VegetationError,
    identity: crate::diagnostics::FixedAuthorizationIdentity,
) -> VegetationError {
    let VegetationError::NumericalFailure {
        category,
        mut diagnostics,
    } = error
    else {
        return error;
    };
    diagnostics.fixed_authorization_identity = Some(identity);
    VegetationError::NumericalFailure {
        category,
        diagnostics,
    }
}

/// Rebuilds every V5 tile column from the original beginning state under one
/// immutable typed authorization batch. No Stage-A candidate is an input.
#[allow(clippy::too_many_arguments)]
pub(crate) fn execute_capped_column_pass(
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    transaction_id: TransactionId,
    interval_s: f64,
    top_rain_kg_m2_tile_ground: &BTreeMap<TileId, f64>,
    authorizations: &ValidatedWaterAuthorizations,
    evaluator: &dyn CappedOccupancyEvaluator,
) -> Result<CappedColumnPass, VegetationError> {
    validate_candidate_transaction(beginning, transaction_id)?;
    validate_candidate_inputs(configuration, beginning, forcing)?;
    if !interval_s.is_finite() || interval_s <= 0.0 {
        return Err(VegetationError::Domain("V5 capped interval duration"));
    }
    if authorizations.transaction_id() != transaction_id {
        return Err(VegetationError::Receipt(
            "V5 capped authorization transaction identity".into(),
        ));
    }

    let tile_fractions = tile_fractions(configuration)?;
    // The column engine performs the stand-to-tile conversion exactly once,
    // immediately before the corresponding occupancy solve. Retaining stand
    // amounts here prevents a preconverted cap from being divided a second time.
    let stand_authorization_amounts = authorizations
        .authorizations()
        .iter()
        .map(|(key, authorization)| (key.clone(), authorization.amount))
        .collect::<BTreeMap<_, _>>();

    let radiation = prepare_whole_column_radiation(configuration, beginning, forcing)?;
    let solver = RadiationBoundCappedSolver::try_new(
        configuration,
        &radiation,
        evaluator,
        authorizations.owner_id(),
        transaction_id,
    )?;
    let columns = execute_tile_columns(
        configuration,
        beginning,
        forcing,
        transaction_id,
        top_rain_kg_m2_tile_ground,
        ColumnPassKind::Final {
            authorizations_kg_m2_stand_ground: &stand_authorization_amounts,
        },
        &solver,
    )?;
    let finalized_water_uses =
        collect_finalized_uses(&columns, authorizations, &tile_fractions, interval_s)?;
    let diagnostics = collect_diagnostics(&columns)?;
    let water_operands = collect_water_operands(
        &finalized_water_uses,
        authorizations,
        &tile_fractions,
        interval_s,
        &diagnostics,
        configuration,
        beginning,
        &columns,
    )?;
    let expected_operand_count = configuration
        .strata
        .iter()
        .map(|stratum| stratum.tile_ids.len() * stratum.root_layers.len())
        .sum();
    validate_capped_water_operands(&water_operands, expected_operand_count)?;
    Ok(CappedColumnPass {
        columns,
        finalized_water_uses,
        radiation,
        diagnostics,
        water_operands,
    })
}

fn validate_capped_water_operands(
    operands: &[CappedWaterLayerOperands],
    expected_count: usize,
) -> Result<(), VegetationError> {
    if operands.len() != expected_count {
        return Err(VegetationError::Receipt(
            "V5 capped operand cardinality".into(),
        ));
    }
    let mut identities = BTreeSet::new();
    for value in operands {
        let local = value.authorization_kg_m2_stand_ground / value.tile_fraction;
        let cap_rate = local / value.interval_s;
        let finalized = value.tile_fraction * value.q_final_kg_m2_tile_s * value.interval_s;
        let selected = if value.authorization_active_or_tie {
            value.cap_rate_kg_m2_tile_s
        } else {
            value.q_law_kg_m2_tile_s
        };
        if value.model_definition_sha256 != crate::MODEL_SHA256
            || !value.tile_fraction.is_finite()
            || value.tile_fraction <= 0.0
            || !value.interval_s.is_finite()
            || value.interval_s <= 0.0
            || local.to_bits() != value.authorization_kg_m2_tile_ground.to_bits()
            || cap_rate.to_bits() != value.cap_rate_kg_m2_tile_s.to_bits()
            || selected.to_bits() != value.q_final_kg_m2_tile_s.to_bits()
            || finalized.to_bits() != value.finalized_use_kg_m2_stand_ground.to_bits()
            || value.authorization_active_or_tie
                != (value.cap_rate_kg_m2_tile_s <= value.q_law_kg_m2_tile_s)
            || !(value.finalized_use_kg_m2_stand_ground <= value.authorization_kg_m2_stand_ground
                && value.authorization_kg_m2_stand_ground
                    <= value.potential_request_kg_m2_stand_ground)
            || value
                .coupled
                .layers
                .get(value.configured_layer_index)
                .map(|layer| &layer.layer_id)
                != Some(&value.key.layer_id)
            || !identities.insert(value.key.clone())
            || value.transaction_id.0
                != value
                    .beginning_occupancy_state
                    .last_accepted_transaction_id
                    .map_or(1, |accepted| accepted.saturating_add(1))
            || value.candidate_occupancy_state.last_accepted_transaction_id
                != value.beginning_occupancy_state.last_accepted_transaction_id
            || !valid_coupled_operands(
                &value.coupled,
                value.vulnerability_demand_sun_kg_m2_s,
                value.vulnerability_demand_shade_kg_m2_s,
            )
        {
            return Err(VegetationError::Receipt(
                "V5 independently reconstructed capped operands".into(),
            ));
        }
    }
    Ok(())
}

fn valid_coupled_operands(
    value: &crate::diagnostics::CappedNumericalOperands,
    vulnerability_demand_sun_kg_m2_s: f64,
    vulnerability_demand_shade_kg_m2_s: f64,
) -> bool {
    const IDENTITIES: [&str; 6] = [
        "sun_gas_minus_q1",
        "shade_gas_minus_q1",
        "sun_gas_minus_vulnerability_demand",
        "shade_gas_minus_vulnerability_demand",
        "q1_sum_minus_q2",
        "q2_minus_capped_q3_sum",
    ];
    let q3_sum = value
        .layers
        .iter()
        .map(|layer| layer.q_final_kg_m2_tile_s)
        .sum::<f64>();
    let raw = [
        value.gas_sun_kg_m2_s - value.q1_sun_kg_m2_s,
        value.gas_shade_kg_m2_s - value.q1_shade_kg_m2_s,
        value.gas_sun_kg_m2_s - vulnerability_demand_sun_kg_m2_s,
        value.gas_shade_kg_m2_s - vulnerability_demand_shade_kg_m2_s,
        value.q1_sun_kg_m2_s + value.q1_shade_kg_m2_s - value.q2_kg_m2_s,
        value.q2_kg_m2_s - q3_sum,
    ];
    let scale = value
        .layers
        .iter()
        .flat_map(|layer| {
            [
                layer.q_law_kg_m2_tile_s.abs(),
                layer.cap_rate_kg_m2_tile_s.abs(),
                layer.q_final_kg_m2_tile_s.abs(),
            ]
        })
        .chain([
            1.0e-12,
            value.emax_sun_kg_m2_s.abs(),
            value.emax_shade_kg_m2_s.abs(),
            value.q1_sun_kg_m2_s.abs(),
            value.q1_shade_kg_m2_s.abs(),
            value.q2_kg_m2_s.abs(),
        ])
        .fold(1.0e-12, f64::max);
    let tolerance = 1.0e-12 + 1.0e-9 * scale;
    scale.to_bits() == value.water_residual_scale_kg_m2_tile_s.to_bits()
        && value.residuals.len() == IDENTITIES.len()
        && value.residuals.iter().zip(IDENTITIES.iter().zip(raw)).all(
            |(actual, (identity, expected_raw))| {
                actual.identity == *identity
                    && actual.raw_kg_m2_tile_s.to_bits() == expected_raw.to_bits()
                    && actual.scale_kg_m2_tile_s.to_bits() == scale.to_bits()
                    && actual.tolerance.to_bits() == tolerance.to_bits()
                    && actual.normalized.to_bits() == (expected_raw / tolerance).to_bits()
            },
        )
        && value.layers.iter().all(|layer| {
            layer.authorization_active_or_tie
                == (layer.cap_rate_kg_m2_tile_s <= layer.q_law_kg_m2_tile_s)
                && layer.q_final_kg_m2_tile_s.to_bits()
                    == (if layer.authorization_active_or_tie {
                        layer.cap_rate_kg_m2_tile_s
                    } else {
                        layer.q_law_kg_m2_tile_s
                    })
                    .to_bits()
        })
}

#[allow(clippy::too_many_arguments)]
fn collect_water_operands(
    finalized: &[FinalizedWaterUse],
    authorizations: &ValidatedWaterAuthorizations,
    tile_fractions: &BTreeMap<TileId, f64>,
    interval_s: f64,
    diagnostics: &BTreeMap<OccupancyId, OccupancyDiagnostics>,
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    columns: &TileColumnsResult,
) -> Result<Vec<CappedWaterLayerOperands>, VegetationError> {
    let finalized = finalized
        .iter()
        .map(|value| (value.key.clone(), value.amount))
        .collect::<BTreeMap<_, _>>();
    let mut output = Vec::new();
    for (key, authorization) in authorizations.authorizations() {
        let request = authorizations
            .requests()
            .get(key)
            .ok_or_else(|| VegetationError::Receipt("V5 capped request identity".into()))?;
        let tile_fraction = *tile_fractions
            .get(&key.occupancy_id.tile_id)
            .ok_or_else(|| VegetationError::Receipt("V5 capped tile identity".into()))?;
        let coupled = diagnostics
            .get(&key.occupancy_id)
            .and_then(|value| value.capped_operands.as_ref())
            .ok_or_else(|| VegetationError::Receipt("V5 capped numerical operands".into()))?;
        let occupancy_diagnostics = diagnostics
            .get(&key.occupancy_id)
            .ok_or_else(|| VegetationError::Receipt("V5 capped diagnostic identity".into()))?;
        let vulnerability_demand_sun_kg_m2_s = occupancy_diagnostics
            .vulnerability_demand_sun_kg_m2_s
            .ok_or_else(|| VegetationError::Receipt("V5 sun vulnerability operand".into()))?;
        let vulnerability_demand_shade_kg_m2_s = occupancy_diagnostics
            .vulnerability_demand_shade_kg_m2_s
            .ok_or_else(|| VegetationError::Receipt("V5 shade vulnerability operand".into()))?;
        let (configured_layer_index, numerical) = coupled
            .layers
            .iter()
            .enumerate()
            .find(|(_, layer)| layer.layer_id == key.layer_id)
            .ok_or_else(|| VegetationError::Receipt("V5 capped layer operands".into()))?;
        let beginning_lane = beginning
            .occupancies
            .get(&key.occupancy_id)
            .ok_or_else(|| VegetationError::Receipt("V5 beginning occupancy identity".into()))?;
        let candidate_lane = columns
            .columns
            .iter()
            .flat_map(|column| &column.occupancy_results)
            .find(|occupancy| occupancy.occupancy_id == key.occupancy_id)
            .map(|occupancy| &occupancy.candidate_state)
            .ok_or_else(|| VegetationError::Receipt("V5 candidate occupancy identity".into()))?;
        output.push(CappedWaterLayerOperands {
            model_definition_sha256: beginning.model_definition_sha256.clone(),
            configuration_sha256: configuration.configuration_sha256.clone(),
            beginning_state_sha256: beginning.state_sha256.clone(),
            transaction_id: authorization.transaction_id,
            tile_fraction,
            interval_s,
            configured_layer_index,
            key: key.clone(),
            potential_request_kg_m2_stand_ground: request.amount,
            authorization_kg_m2_stand_ground: authorization.amount,
            authorization_kg_m2_tile_ground: authorization.amount / tile_fraction,
            cap_rate_kg_m2_tile_s: authorization.amount / (tile_fraction * interval_s),
            q_law_kg_m2_tile_s: numerical.q_law_kg_m2_tile_s,
            q_final_kg_m2_tile_s: numerical.q_final_kg_m2_tile_s,
            finalized_use_kg_m2_stand_ground: *finalized
                .get(key)
                .ok_or_else(|| VegetationError::Receipt("V5 finalized identity".into()))?,
            authorization_active_or_tie: numerical.authorization_active_or_tie,
            vulnerability_demand_sun_kg_m2_s,
            vulnerability_demand_shade_kg_m2_s,
            beginning_occupancy_state: beginning_lane.clone(),
            candidate_occupancy_state: candidate_lane.clone(),
            coupled: coupled.clone(),
        });
    }
    Ok(output)
}

fn validate_candidate_transaction(
    beginning: &CoupledOwnedState,
    transaction_id: TransactionId,
) -> Result<(), VegetationError> {
    let expected = beginning
        .last_transaction_id
        .checked_add(1)
        .ok_or_else(|| VegetationError::Receipt("V4 transaction identity overflow".into()))?;
    if transaction_id.0 != expected {
        return Err(VegetationError::Receipt(
            "nonsequential V5 capped-pass transaction identity".into(),
        ));
    }
    Ok(())
}

fn tile_fractions(
    configuration: &VegetationConfiguration,
) -> Result<BTreeMap<TileId, f64>, VegetationError> {
    let fractions = configuration
        .topology_tiles
        .iter()
        .map(|tile| (tile.tile_id.clone(), tile.fraction))
        .collect::<BTreeMap<_, _>>();
    if fractions.len() != configuration.topology_tiles.len()
        || fractions
            .values()
            .any(|fraction| !fraction.is_finite() || *fraction <= 0.0)
    {
        return Err(VegetationError::Domain("V5 capped tile fraction"));
    }
    Ok(fractions)
}

fn collect_finalized_uses(
    columns: &TileColumnsResult,
    authorizations: &ValidatedWaterAuthorizations,
    tile_fractions: &BTreeMap<TileId, f64>,
    interval_s: f64,
) -> Result<Vec<FinalizedWaterUse>, VegetationError> {
    let mut amounts = BTreeMap::new();
    for column in &columns.columns {
        for occupancy in &column.occupancy_results {
            for (key, amount) in &occupancy.stand_ground_layer_water_kg_m2 {
                if key.occupancy_id != occupancy.occupancy_id
                    || amounts.insert(key.clone(), *amount).is_some()
                {
                    return Err(VegetationError::Receipt(
                        "duplicate or mismatched V4 finalized water identity".into(),
                    ));
                }
            }
        }
    }
    let expected = authorizations
        .authorizations()
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let actual = amounts.keys().cloned().collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(VegetationError::Receipt(
            "V4 finalized water key set differs from authorization".into(),
        ));
    }

    authorizations
        .authorizations()
        .iter()
        .map(|(key, authorization)| {
            let raw = amounts.get(key).copied().ok_or_else(|| {
                VegetationError::Receipt("V4 finalized water authorization identity".into())
            })?;
            let amount = authorizations
                .validate_finalized_stand_amount(key, raw, tile_fractions, interval_s)
                .map_err(|error| resource_boundary_error(&error))?;
            if authorization.transaction_id != authorizations.transaction_id()
                || &authorization.owner_id != authorizations.owner_id()
                || authorization.key != *key
                || authorization.basis != WATER_STAND_BASIS
            {
                return Err(VegetationError::Receipt(
                    "V4 finalized water authorization identity".into(),
                ));
            }
            let finalized = FinalizedWaterUse {
                transaction_id: authorization.transaction_id,
                owner_id: authorization.owner_id.clone(),
                key: key.clone(),
                amount,
                basis: authorization.basis,
            };
            let request = authorizations.requests().get(key).ok_or_else(|| {
                VegetationError::Receipt("V4 finalized water request identity".into())
            })?;
            validate_resource_protocol(request, authorization, &finalized).map_err(
                |violation| {
                    VegetationError::Receipt(format!(
                        "V4 finalized water protocol violation: {violation:?}"
                    ))
                },
            )?;
            Ok(finalized)
        })
        .collect()
}

fn collect_diagnostics(
    columns: &TileColumnsResult,
) -> Result<BTreeMap<OccupancyId, OccupancyDiagnostics>, VegetationError> {
    let mut diagnostics = BTreeMap::new();
    for column in &columns.columns {
        for occupancy in &column.occupancy_results {
            if diagnostics
                .insert(
                    occupancy.occupancy_id.clone(),
                    occupancy.diagnostics.clone(),
                )
                .is_some()
            {
                return Err(VegetationError::Receipt(
                    "duplicate V5 capped diagnostic identity".into(),
                ));
            }
        }
    }
    Ok(diagnostics)
}

fn resource_boundary_error(error: &WaterResourceBoundaryError) -> VegetationError {
    error.clone().into()
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use openwepp_kernel_contract::{MaximumAuthorization, ResourceOwnerId, SoilLayerId, StratumId};

    use super::*;
    use crate::interception::{InterceptionInput, liquid_interception};
    use crate::occupancy_solver::resources::{OccupancyRootLayers, PotentialWaterRequestBatch};
    use crate::occupancy_state::OccupancyState;
    use crate::transaction::{SoilLayerForcing, StratumSharedState};

    #[derive(Clone, Debug, PartialEq)]
    struct SeenInput {
        occupancy_id: OccupancyId,
        beginning_store: f64,
        incident_rain: f64,
        local_cap: f64,
    }

    struct ControlledCappedEvaluator {
        seen: RefCell<Vec<SeenInput>>,
        fail_at: Option<OccupancyId>,
    }

    impl CappedOccupancyEvaluator for ControlledCappedEvaluator {
        #[allow(clippy::too_many_lines)]
        fn solve_capped(
            &self,
            input: OccupancyPassInput<'_>,
            radiation: &OccupancyRadiation,
        ) -> Result<OccupancyPassResult, VegetationError> {
            if self.fail_at.as_ref() == Some(input.occupancy_id) {
                return Err(VegetationError::InjectedFailure("controlled capped"));
            }
            assert_eq!(radiation.occupancy_id, *input.occupancy_id);
            let caps = input
                .local_authorizations_kg_m2_tile_ground
                .as_ref()
                .expect("capped pass supplies local authorizations");
            let local_cap = caps
                .values()
                .copied()
                .next()
                .expect("fixture has one root layer");
            self.seen.borrow_mut().push(SeenInput {
                occupancy_id: input.occupancy_id.clone(),
                beginning_store: input.occupancy_state.canopy_liquid_kg_h2o_m2_tile_ground,
                incident_rain: input.incident_rain_kg_m2_tile_ground,
                local_cap,
            });
            // A binding upper cap changes the final wet-surface state. The
            // resulting condensation and second drainage are routed to the
            // descendant by the real column engine.
            let vapor_amount =
                if input.occupancy_id.stratum_id.as_str() == "upper" && local_cap == 0.0 {
                    -1.0
                } else {
                    0.0
                };
            let liquid = liquid_interception(InterceptionInput {
                store0: input.occupancy_state.canopy_liquid_kg_h2o_m2_tile_ground,
                rain: input.incident_rain_kg_m2_tile_ground,
                vapor_amount,
                lai: input.conditional_lai_m2_m2_tile_ground,
                sai: input.conditional_wai_m2_m2_tile_ground,
                alpha_liq: input.stratum_config.alpha_liq,
                p_liq: input.stratum_config.p_liq_kg_m2_plant,
                stemflow_fraction: input.stratum_config.stemflow_fraction,
                leaf_temperature_k: input.occupancy_state.wet_surface_temperature_k,
            })?;
            let mut candidate_state = input.occupancy_state.clone();
            candidate_state.canopy_liquid_kg_h2o_m2_tile_ground = liquid.store1;
            let q = local_cap / input.interval_s;
            let beta = input.occupancy_state.beta_hyd;
            let emax = q / beta;
            let scale = emax.abs().max(q.abs()).max(1.0e-12);
            let tolerance = 1.0e-12 + 1.0e-9 * scale;
            Ok(OccupancyPassResult {
                candidate_state,
                liquid,
                local_layer_water_kg_m2_tile_ground: input
                    .stratum_config
                    .root_layers
                    .iter()
                    .map(|root| (root.layer_id.clone(), local_cap))
                    .collect(),
                carbon_operands: None,
                energy_proposal: None,
                diagnostics: OccupancyDiagnostics {
                    pass: crate::diagnostics::CoupledSolvePass::Capped,
                    ci_iterations_sun: 0,
                    ci_iterations_shade: 0,
                    energy_iterations: 0,
                    hydraulic_iterations: 0,
                    outer_iterations: 9,
                    normalized_residuals: vec![crate::diagnostics::NormalizedResidual {
                        identity: "controlled_capped".into(),
                        value: 1.0e-13,
                    }],
                    temperature_step_k: None,
                    potential_step_mm: None,
                    backtracking_count: 0,
                    wet_store_cap_active: false,
                    active_water_caps: input
                        .stratum_config
                        .root_layers
                        .iter()
                        .map(|root| root.layer_id.clone())
                        .collect(),
                    gas_hydraulic_mismatch_kg_m2_s: 0.0,
                    vulnerability_demand_sun_kg_m2_s: Some(q),
                    vulnerability_demand_shade_kg_m2_s: Some(0.0),
                    pivot_magnitude: None,
                    matrix_norm: None,
                    advanced_t10_k: None,
                    capped_operands: Some(crate::diagnostics::CappedNumericalOperands {
                        water_residual_scale_kg_m2_tile_s: scale,
                        psi_sunleaf_mm: input.occupancy_state.sun_leaf_potential_mm,
                        psi_shadeleaf_mm: input.occupancy_state.shade_leaf_potential_mm,
                        psi_stem_mm: input.occupancy_state.stem_potential_mm,
                        psi_root_mm: input.occupancy_state.root_node_potential_mm,
                        beta_sun: beta,
                        beta_shade: beta,
                        emax_sun_kg_m2_s: emax,
                        emax_shade_kg_m2_s: 0.0,
                        gas_sun_kg_m2_s: q,
                        gas_shade_kg_m2_s: 0.0,
                        q1_sun_kg_m2_s: q,
                        q1_shade_kg_m2_s: 0.0,
                        q2_kg_m2_s: q,
                        residuals: [
                            "sun_gas_minus_q1",
                            "shade_gas_minus_q1",
                            "sun_gas_minus_vulnerability_demand",
                            "shade_gas_minus_vulnerability_demand",
                            "q1_sum_minus_q2",
                            "q2_minus_capped_q3_sum",
                        ]
                        .into_iter()
                        .map(|identity| crate::diagnostics::CappedResidualOperands {
                            identity: identity.into(),
                            raw_kg_m2_tile_s: 0.0,
                            scale_kg_m2_tile_s: scale,
                            tolerance,
                            normalized: 0.0,
                        })
                        .collect(),
                        layers: input
                            .stratum_config
                            .root_layers
                            .iter()
                            .map(|root| crate::diagnostics::CappedLayerNumericalOperands {
                                layer_id: root.layer_id.clone(),
                                cap_rate_kg_m2_tile_s: q,
                                q_law_kg_m2_tile_s: q,
                                q_final_kg_m2_tile_s: q,
                                authorization_active_or_tie: true,
                                soil_potential_mm: 0.0,
                                gravity_head_mm: 0.0,
                                root_fraction: root.root_fraction,
                                z3_m: 0.0,
                                ksoil_m2_s: 0.0,
                                dxroot_m: 0.0,
                                accessible: true,
                                frozen: false,
                            })
                            .collect(),
                    }),
                },
            })
        }
    }

    fn stratum_id(value: &str) -> StratumId {
        StratumId::try_new(value).expect("stratum identity")
    }

    fn occupancy(stratum: &str, tile: &str) -> OccupancyId {
        OccupancyId {
            stratum_id: stratum_id(stratum),
            tile_id: TileId::try_new(tile).expect("tile identity"),
        }
    }

    fn owner() -> ResourceOwnerId {
        ResourceOwnerId::try_new("vegetation").expect("owner identity")
    }

    fn fixture() -> (VegetationConfiguration, CoupledOwnedState) {
        let (mut configuration, original) = crate::transaction::v7_identity_rebound_fixture();
        let tile_id = configuration.topology_tiles[0].tile_id.clone();
        let mut upper_config = configuration.strata.remove(0);
        upper_config.stratum_id = stratum_id("upper");
        upper_config.vertical_rank = 0;
        let mut lower_config = upper_config.clone();
        lower_config.stratum_id = stratum_id("lower");
        lower_config.vertical_rank = 1;
        lower_config.height_m *= 0.5;
        lower_config.crown_base_m *= 0.5;
        configuration.strata = vec![upper_config, lower_config];
        configuration.configuration_sha256 = configuration
            .canonical_sha256()
            .expect("configuration digest");

        let original_shared = original
            .strata
            .values()
            .next()
            .cloned()
            .expect("shared state");
        let original_lane = original
            .occupancies
            .values()
            .next()
            .cloned()
            .expect("occupancy state");
        let mut beginning = CoupledOwnedState {
            model_definition_sha256: original.model_definition_sha256,
            configuration_sha256: configuration.configuration_sha256.clone(),
            state_sha256: String::new(),
            strata: BTreeMap::<StratumId, StratumSharedState>::from([
                (stratum_id("upper"), original_shared.clone()),
                (stratum_id("lower"), original_shared),
            ]),
            occupancies: BTreeMap::<OccupancyId, OccupancyState>::from([
                (occupancy("upper", tile_id.as_str()), original_lane.clone()),
                (occupancy("lower", tile_id.as_str()), original_lane),
            ]),
            last_transaction_id: 0,
        };
        beginning.state_sha256 = beginning.canonical_sha256().expect("state digest");
        configuration.initial_state_sha256 = beginning.state_sha256.clone();
        configuration.validate().expect("two-rank configuration");
        beginning
            .validate(&configuration)
            .expect("two-rank beginning state");
        (configuration, beginning)
    }

    fn forcing() -> SnowFreeForcing {
        SnowFreeForcing {
            air_temperature_k: 298.15,
            pressure_pa: 101_325.0,
            co2_pa: 42.0,
            vapor_pressure_deficit_kpa: 1.2,
            wind_m_s: 3.7,
            rain_kg_m2: 0.0,
            direct_par_w_m2: 410.0,
            diffuse_par_w_m2: 83.0,
            direct_nir_w_m2: 355.0,
            diffuse_nir_w_m2: 101.0,
            solar_zenith_cosine: 0.67,
            ground_albedo_vis: 0.14,
            ground_albedo_nir: 0.31,
            longwave_down_w_m2: 350.0,
            longwave_up_w_m2: 390.0,
            specific_humidity: 0.01,
            reference_height_m: 20.0,
            soil_layers: vec![SoilLayerForcing {
                layer_id: SoilLayerId::try_new("soil-1").expect("layer identity"),
                water_beginning_kg_m2: 20.0,
                matric_potential_mm: -1_000.0,
                hydraulic_conductivity_mm_s: 1.0e-5,
                root_path_length_mm: 100.0,
                gravity_root_mm: 500.0,
                temperature_k: 295.0,
                accessible: true,
                frozen: false,
            }],
            root_zone_hydraulics: None,
            gsi: 1.0,
        }
    }

    fn authorization_batch(
        configuration: &VegetationConfiguration,
        upper_amount: f64,
        lower_amount: f64,
    ) -> ValidatedWaterAuthorizations {
        let layer_id = configuration.strata[0].root_layers[0].layer_id.clone();
        let tile_id = configuration.topology_tiles[0].tile_id.clone();
        let configured = vec![
            OccupancyRootLayers {
                occupancy_id: occupancy("upper", tile_id.as_str()),
                layer_ids: vec![layer_id.clone()],
            },
            OccupancyRootLayers {
                occupancy_id: occupancy("lower", tile_id.as_str()),
                layer_ids: vec![layer_id],
            },
        ];
        let amounts = configured
            .iter()
            .map(|lane| {
                (
                    WaterResourceKey {
                        occupancy_id: lane.occupancy_id.clone(),
                        layer_id: lane.layer_ids[0].clone(),
                    },
                    1.0,
                )
            })
            .collect::<BTreeMap<_, _>>();
        let requests = PotentialWaterRequestBatch::try_from_stand_amounts(
            TransactionId(1),
            owner(),
            &configured,
            &amounts,
        )
        .expect("potential request identities");
        let authorizations = requests
            .requests()
            .iter()
            .map(|request| MaximumAuthorization {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount: if request.key.occupancy_id.stratum_id.as_str() == "upper" {
                    upper_amount
                } else {
                    lower_amount
                },
                basis: request.basis,
            })
            .collect();
        ValidatedWaterAuthorizations::try_new(&requests, authorizations)
            .expect("validated authorizations")
    }

    fn execute(
        configuration: &VegetationConfiguration,
        beginning: &CoupledOwnedState,
        authorizations: &ValidatedWaterAuthorizations,
        evaluator: &ControlledCappedEvaluator,
    ) -> Result<CappedColumnPass, VegetationError> {
        execute_capped_column_pass(
            configuration,
            beginning,
            &forcing(),
            TransactionId(1),
            1_800.0,
            &BTreeMap::from([(configuration.topology_tiles[0].tile_id.clone(), 0.0)]),
            authorizations,
            evaluator,
        )
    }

    #[test]
    fn starts_from_original_state_and_preserves_typed_finalized_identity() {
        let (configuration, beginning) = fixture();
        let mut detached_potential_candidate = beginning.clone();
        for lane in detached_potential_candidate.occupancies.values_mut() {
            lane.canopy_liquid_kg_h2o_m2_tile_ground = 99.0;
        }
        let expected_beginning_stores = beginning
            .occupancies
            .values()
            .map(|lane| lane.canopy_liquid_kg_h2o_m2_tile_ground.to_bits())
            .collect::<Vec<_>>();
        let evaluator = ControlledCappedEvaluator {
            seen: RefCell::new(Vec::new()),
            fail_at: None,
        };
        let authorizations = authorization_batch(&configuration, 0.25, 0.5);
        let result =
            execute(&configuration, &beginning, &authorizations, &evaluator).expect("capped pass");

        let seen_stores = evaluator
            .seen
            .borrow()
            .iter()
            .map(|seen| seen.beginning_store.to_bits())
            .collect::<Vec<_>>();
        assert_eq!(seen_stores, expected_beginning_stores);
        assert_eq!(result.water_operands.len(), 2);
        for operand in &result.water_operands {
            assert_eq!(operand.model_definition_sha256, crate::MODEL_SHA256);
            assert_eq!(
                operand.configuration_sha256,
                configuration.configuration_sha256
            );
            assert_eq!(operand.beginning_state_sha256, beginning.state_sha256);
            assert!(
                operand.finalized_use_kg_m2_stand_ground
                    <= operand.authorization_kg_m2_stand_ground
            );
            assert!(
                operand.authorization_kg_m2_stand_ground
                    <= operand.potential_request_kg_m2_stand_ground
            );
            assert!(
                (operand.authorization_kg_m2_tile_ground
                    - operand.authorization_kg_m2_stand_ground
                        / configuration.topology_tiles[0].fraction)
                    .abs()
                    <= f64::EPSILON
            );
            assert!(
                (operand.cap_rate_kg_m2_tile_s - operand.authorization_kg_m2_tile_ground / 1_800.0)
                    .abs()
                    <= f64::EPSILON
            );
            assert!(
                (operand.finalized_use_kg_m2_stand_ground
                    - configuration.topology_tiles[0].fraction
                        * operand.q_final_kg_m2_tile_s
                        * 1_800.0)
                    .abs()
                    <= f64::EPSILON
            );
            assert_eq!(
                operand.beginning_occupancy_state,
                beginning.occupancies[&operand.key.occupancy_id]
            );
        }
        assert!(
            evaluator
                .seen
                .borrow()
                .iter()
                .all(|seen| seen.beginning_store.to_bits() != 99.0_f64.to_bits())
        );
        assert_eq!(result.finalized_water_uses.len(), 2);
        for finalized in &result.finalized_water_uses {
            let authorization = &authorizations.authorizations()[&finalized.key];
            assert_eq!(finalized.transaction_id, TransactionId(1));
            assert_eq!(finalized.owner_id, owner());
            assert_eq!(finalized.basis, WATER_STAND_BASIS);
            assert_eq!(finalized.amount.to_bits(), authorization.amount.to_bits());
        }
        assert_eq!(result.radiation.occupancies.len(), 2);
        assert_eq!(result.diagnostics.len(), 2);
    }

    #[test]
    fn independent_capped_operand_validator_rejects_tautology_and_lineage_poisons() {
        let (configuration, beginning) = fixture();
        let evaluator = ControlledCappedEvaluator {
            seen: RefCell::new(Vec::new()),
            fail_at: None,
        };
        let authorizations = authorization_batch(&configuration, 0.25, 0.5);
        let result = execute(&configuration, &beginning, &authorizations, &evaluator)
            .expect("canonical operands");

        let mut poisons = Vec::new();
        let mut wrong_conversion = result.water_operands.clone();
        wrong_conversion[0].authorization_kg_m2_tile_ground = f64::from_bits(
            wrong_conversion[0]
                .authorization_kg_m2_tile_ground
                .to_bits()
                + 1,
        );
        poisons.push(wrong_conversion);

        let mut producer_zero = result.water_operands.clone();
        producer_zero[0].coupled.residuals[0].raw_kg_m2_tile_s = 0.0;
        producer_zero[0].coupled.gas_sun_kg_m2_s =
            f64::from_bits(producer_zero[0].coupled.gas_sun_kg_m2_s.to_bits() + 1);
        poisons.push(producer_zero);

        let mut wrong_scale = result.water_operands.clone();
        wrong_scale[0].coupled.water_residual_scale_kg_m2_tile_s *= 2.0;
        poisons.push(wrong_scale);

        let mut wrong_branch = result.water_operands.clone();
        wrong_branch[0].coupled.layers[0].authorization_active_or_tie = false;
        poisons.push(wrong_branch);

        let mut wrong_lineage = result.water_operands.clone();
        wrong_lineage[0]
            .candidate_occupancy_state
            .last_accepted_transaction_id = Some(99);
        poisons.push(wrong_lineage);

        for poison in poisons {
            let expected_count = poison.len();
            assert!(validate_capped_water_operands(&poison, expected_count).is_err());
        }
    }

    #[test]
    fn upper_cap_changes_final_release_received_by_descendant() {
        let (configuration, beginning) = fixture();
        let unrestricted_evaluator = ControlledCappedEvaluator {
            seen: RefCell::new(Vec::new()),
            fail_at: None,
        };
        execute(
            &configuration,
            &beginning,
            &authorization_batch(&configuration, 0.5, 0.5),
            &unrestricted_evaluator,
        )
        .expect("unrestricted capped pass");
        let restricted_evaluator = ControlledCappedEvaluator {
            seen: RefCell::new(Vec::new()),
            fail_at: None,
        };
        let restricted = execute(
            &configuration,
            &beginning,
            &authorization_batch(&configuration, 0.0, 0.5),
            &restricted_evaluator,
        )
        .expect("restricted capped pass");

        let unrestricted_lower_rain = unrestricted_evaluator.seen.borrow()[1].incident_rain;
        let restricted_lower_rain = restricted_evaluator.seen.borrow()[1].incident_rain;
        assert_eq!(unrestricted_lower_rain.to_bits(), 0.0_f64.to_bits());
        assert!(restricted_lower_rain > unrestricted_lower_rain);
        assert!(
            restricted.columns.columns[0].occupancy_results[0]
                .liquid
                .second_drainage
                > 0.0
        );
        assert_eq!(
            restricted_evaluator.seen.borrow()[0].local_cap.to_bits(),
            0.0_f64.to_bits()
        );
    }

    #[test]
    fn evaluator_failure_rolls_back_every_beginning_lane_byte_identically() {
        let (configuration, beginning) = fixture();
        let before = serde_json::to_vec(&beginning).expect("serialize beginning");
        let evaluator = ControlledCappedEvaluator {
            seen: RefCell::new(Vec::new()),
            fail_at: Some(occupancy(
                "lower",
                configuration.topology_tiles[0].tile_id.as_str(),
            )),
        };
        assert_eq!(
            execute(
                &configuration,
                &beginning,
                &authorization_batch(&configuration, 0.0, 0.5),
                &evaluator,
            ),
            Err(VegetationError::InjectedFailure("controlled capped"))
        );
        assert_eq!(
            serde_json::to_vec(&beginning).expect("serialize beginning"),
            before
        );
    }

    #[test]
    fn rejects_wrong_transaction_and_interval_before_evaluation() {
        let (configuration, beginning) = fixture();
        let evaluator = ControlledCappedEvaluator {
            seen: RefCell::new(Vec::new()),
            fail_at: None,
        };
        let authorizations = authorization_batch(&configuration, 0.25, 0.5);
        let rain = BTreeMap::from([(configuration.topology_tiles[0].tile_id.clone(), 0.0)]);
        assert!(matches!(
            execute_capped_column_pass(
                &configuration,
                &beginning,
                &forcing(),
                TransactionId(2),
                1_800.0,
                &rain,
                &authorizations,
                &evaluator,
            ),
            Err(VegetationError::Receipt(message))
                if message == "nonsequential V5 capped-pass transaction identity"
        ));
        assert!(matches!(
            execute_capped_column_pass(
                &configuration,
                &beginning,
                &forcing(),
                TransactionId(1),
                0.0,
                &rain,
                &authorizations,
                &evaluator,
            ),
            Err(VegetationError::Domain("V5 capped interval duration"))
        ));
        assert!(evaluator.seen.borrow().is_empty());
    }
}
