//! V4 occupancy-local tile-column ordering, liquid routing, and area conversion.
//!
//! This module deliberately contains no constitutive approximation. An
//! [`OccupancyPassSolver`] supplies a complete occupancy result; this engine
//! validates its identities and water operands, routes releases within the
//! same tile, and reconstructs closure independently. The public coupled
//! transaction remains fail-closed until the exact E11--E15 solver is wired.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, SoilLayerId, TileId, TransactionId, WaterResourceKey};

use crate::VegetationError;
use crate::config::{StratumConfiguration, VegetationConfiguration};
use crate::diagnostics::{CappedNumericalOperands, CoupledSolvePass, NormalizedResidual};
use crate::interception::InterceptionResult;
use crate::occupancy_state::OccupancyState;
use crate::transaction::{CoupledOwnedState, SnowFreeForcing, StratumSharedState};

/// One validated topology column in deterministic top-to-bottom order.
#[derive(Clone, Debug, PartialEq)]
pub struct TileColumn {
    pub tile_id: TileId,
    pub tile_fraction: f64,
    pub occupancies: Vec<OccupancyId>,
}

/// The internal pass being executed. Final-pass amounts remain stand-ground
/// maximum authorizations until this engine converts them back to tile basis.
#[derive(Clone, Copy, Debug)]
pub enum ColumnPassKind<'a> {
    Potential,
    Final {
        authorizations_kg_m2_stand_ground: &'a BTreeMap<WaterResourceKey, f64>,
    },
}

/// Immutable inputs supplied to one occupancy-local constitutive solve.
#[derive(Clone, Debug)]
pub struct OccupancyPassInput<'a> {
    pub transaction_id: TransactionId,
    /// Exact validated configuration interval; no evaluator may supply an
    /// independent or fixture-local duration.
    pub interval_s: f64,
    pub occupancy_id: &'a OccupancyId,
    pub tile_fraction: f64,
    pub coverage: f64,
    pub conditional_lai_m2_m2_tile_ground: f64,
    pub conditional_wai_m2_m2_tile_ground: f64,
    pub incident_rain_kg_m2_tile_ground: f64,
    pub local_authorizations_kg_m2_tile_ground: Option<BTreeMap<SoilLayerId, f64>>,
    pub shared_state: &'a StratumSharedState,
    pub occupancy_state: &'a OccupancyState,
    pub stratum_config: &'a StratumConfiguration,
    pub forcing: &'a SnowFreeForcing,
}

/// Complete accepted coupled-solver diagnostics retained across the routing
/// seam. Controlled topology solvers populate explicit zero/not-applicable
/// values; production evaluators supply the canonical E11--E15 payload.
#[derive(Clone, Debug, PartialEq)]
pub struct OccupancyDiagnostics {
    pub pass: CoupledSolvePass,
    pub ci_iterations_sun: u32,
    pub ci_iterations_shade: u32,
    pub energy_iterations: u32,
    pub hydraulic_iterations: u32,
    pub outer_iterations: u32,
    pub normalized_residuals: Vec<NormalizedResidual>,
    pub temperature_step_k: Option<f64>,
    pub potential_step_mm: Option<f64>,
    pub backtracking_count: u32,
    pub wet_store_cap_active: bool,
    pub active_water_caps: Vec<SoilLayerId>,
    pub gas_hydraulic_mismatch_kg_m2_s: f64,
    pub vulnerability_demand_sun_kg_m2_s: Option<f64>,
    pub vulnerability_demand_shade_kg_m2_s: Option<f64>,
    pub pivot_magnitude: Option<f64>,
    pub matrix_norm: Option<f64>,
    /// Candidate ten-day acclimation temperature calculated once before
    /// Atkin Rd25 and retained for the later shared-state finalizer.
    pub advanced_t10_k: Option<f64>,
    pub capped_operands: Option<CappedNumericalOperands>,
}

/// One solver-produced occupancy candidate. All water amounts are tile-ground
/// interval amounts until the column engine performs the one stand conversion.
#[derive(Clone, Debug, PartialEq)]
pub struct OccupancyPassResult {
    pub candidate_state: OccupancyState,
    pub liquid: InterceptionResult,
    pub local_layer_water_kg_m2_tile_ground: Vec<(SoilLayerId, f64)>,
    /// Exact accepted FvCB/Rd operands. Controlled routing solvers leave this
    /// absent; production potential and capped evaluators must populate it.
    pub carbon_operands: Option<OccupancyCarbonOperands>,
    pub diagnostics: OccupancyDiagnostics,
}

/// Accepted class-resolved operands needed to aggregate E16/E17 once per
/// shared stratum without re-running or approximating the nonlinear solve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OccupancyCarbonOperands {
    pub advanced_t10_k: f64,
    pub sun_leaf_area_m2_m2_tile_ground: f64,
    pub shade_leaf_area_m2_m2_tile_ground: f64,
    pub sun_gross_assimilation_umol_co2_m2_leaf_s: f64,
    pub shade_gross_assimilation_umol_co2_m2_leaf_s: f64,
    pub sun_dark_respiration_umol_co2_m2_leaf_s: f64,
    pub shade_dark_respiration_umol_co2_m2_leaf_s: f64,
}

/// Constitutive seam used by the internal routing engine. Controlled test
/// solvers prove topology only; they are not E11--E15 implementation evidence.
pub trait OccupancyPassSolver {
    fn solve(&self, input: OccupancyPassInput<'_>) -> Result<OccupancyPassResult, VegetationError>;
}

/// Releases routed by the column engine after accepting one occupancy result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OccupancyLiquidRelease {
    pub throughfall_kg_m2_tile_ground: f64,
    pub initial_drainage_kg_m2_tile_ground: f64,
    pub second_drainage_kg_m2_tile_ground: f64,
    pub stemflow_kg_m2_tile_ground: f64,
}

/// Authoritative operands for independent occupancy canopy-liquid closure.
#[derive(Clone, Debug, PartialEq)]
pub struct OccupancyLiquidLedger {
    pub occupancy_id: OccupancyId,
    pub beginning_store_kg_m2_tile_ground: f64,
    pub incident_rain_kg_m2_tile_ground: f64,
    pub condensation_kg_m2_tile_ground: f64,
    pub ending_store_kg_m2_tile_ground: f64,
    pub evaporation_kg_m2_tile_ground: f64,
    pub throughfall_kg_m2_tile_ground: f64,
    pub stemflow_kg_m2_tile_ground: f64,
    pub initial_drainage_kg_m2_tile_ground: f64,
    pub second_drainage_kg_m2_tile_ground: f64,
}

impl OccupancyLiquidLedger {
    #[must_use]
    pub fn residual_kg_m2_tile_ground(&self) -> f64 {
        self.beginning_store_kg_m2_tile_ground
            + self.incident_rain_kg_m2_tile_ground
            + self.condensation_kg_m2_tile_ground
            - self.ending_store_kg_m2_tile_ground
            - self.evaporation_kg_m2_tile_ground
            - self.throughfall_kg_m2_tile_ground
            - self.stemflow_kg_m2_tile_ground
            - self.initial_drainage_kg_m2_tile_ground
            - self.second_drainage_kg_m2_tile_ground
    }

    fn operand_sum(&self) -> f64 {
        self.beginning_store_kg_m2_tile_ground
            + self.incident_rain_kg_m2_tile_ground
            + self.condensation_kg_m2_tile_ground
            + self.ending_store_kg_m2_tile_ground
            + self.evaporation_kg_m2_tile_ground
            + self.throughfall_kg_m2_tile_ground
            + self.stemflow_kg_m2_tile_ground
            + self.initial_drainage_kg_m2_tile_ground
            + self.second_drainage_kg_m2_tile_ground
    }
}

/// One occupancy result after routing and the exact one-time area conversion.
#[derive(Clone, Debug, PartialEq)]
pub struct RoutedOccupancyResult {
    pub occupancy_id: OccupancyId,
    pub candidate_state: OccupancyState,
    pub liquid: InterceptionResult,
    pub release: OccupancyLiquidRelease,
    pub stand_ground_layer_water_kg_m2: BTreeMap<WaterResourceKey, f64>,
    pub carbon_operands: Option<OccupancyCarbonOperands>,
    pub diagnostics: OccupancyDiagnostics,
}

/// Exposed tile-column operands. Internal releases cancel only during this
/// reconstruction; they are never pre-aggregated before descendant solves.
#[derive(Clone, Debug, PartialEq)]
pub struct TileLiquidLedger {
    pub tile_id: TileId,
    pub tile_fraction: f64,
    pub top_rain_kg_m2_tile_ground: f64,
    pub occupancies: Vec<OccupancyLiquidLedger>,
    pub ground_throughfall_kg_m2_tile_ground: f64,
    pub ground_drainage_kg_m2_tile_ground: f64,
    pub ground_stemflow_kg_m2_tile_ground: f64,
}

impl TileLiquidLedger {
    #[must_use]
    pub fn residual_kg_m2_tile_ground(&self) -> f64 {
        let beginning = self
            .occupancies
            .iter()
            .map(|ledger| ledger.beginning_store_kg_m2_tile_ground)
            .sum::<f64>();
        let condensation = self
            .occupancies
            .iter()
            .map(|ledger| ledger.condensation_kg_m2_tile_ground)
            .sum::<f64>();
        let ending = self
            .occupancies
            .iter()
            .map(|ledger| ledger.ending_store_kg_m2_tile_ground)
            .sum::<f64>();
        let evaporation = self
            .occupancies
            .iter()
            .map(|ledger| ledger.evaporation_kg_m2_tile_ground)
            .sum::<f64>();
        beginning + self.top_rain_kg_m2_tile_ground + condensation
            - ending
            - evaporation
            - self.ground_throughfall_kg_m2_tile_ground
            - self.ground_drainage_kg_m2_tile_ground
            - self.ground_stemflow_kg_m2_tile_ground
    }

    fn operand_sum(&self) -> f64 {
        self.top_rain_kg_m2_tile_ground
            + self
                .occupancies
                .iter()
                .map(OccupancyLiquidLedger::operand_sum)
                .sum::<f64>()
            + self.ground_throughfall_kg_m2_tile_ground
            + self.ground_drainage_kg_m2_tile_ground
            + self.ground_stemflow_kg_m2_tile_ground
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TileColumnResult {
    pub tile_id: TileId,
    pub occupancy_results: Vec<RoutedOccupancyResult>,
    pub ground_throughfall_kg_m2_tile_ground: f64,
    pub ground_drainage_kg_m2_tile_ground: f64,
    pub ground_stemflow_kg_m2_tile_ground: f64,
    pub ledger: TileLiquidLedger,
}

/// Complete internal pass result. The weighted residual is reconstructed only
/// from exposed tile ledgers.
#[derive(Clone, Debug, PartialEq)]
pub struct TileColumnsResult {
    pub columns: Vec<TileColumnResult>,
}

impl TileColumnsResult {
    #[must_use]
    pub fn weighted_stand_residual_kg_m2(&self) -> f64 {
        self.columns
            .iter()
            .map(|column| column.ledger.tile_fraction * column.ledger.residual_kg_m2_tile_ground())
            .sum()
    }
}

/// Builds every tile column, including empty tiles, in canonical tile-ID order.
pub fn build_tile_columns(
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
) -> Result<Vec<TileColumn>, VegetationError> {
    config.validate()?;
    beginning.validate(config)?;
    let mut columns = config
        .topology_tiles
        .iter()
        .map(|tile| {
            let mut strata = config
                .strata
                .iter()
                .filter(|stratum| stratum.tile_ids.contains(&tile.tile_id))
                .collect::<Vec<_>>();
            strata.sort_by(|left, right| {
                left.vertical_rank
                    .cmp(&right.vertical_rank)
                    .then_with(|| left.stratum_id.cmp(&right.stratum_id))
            });
            TileColumn {
                tile_id: tile.tile_id.clone(),
                tile_fraction: tile.fraction,
                occupancies: strata
                    .into_iter()
                    .map(|stratum| OccupancyId {
                        stratum_id: stratum.stratum_id.clone(),
                        tile_id: tile.tile_id.clone(),
                    })
                    .collect(),
            }
        })
        .collect::<Vec<_>>();
    columns.sort_by(|left, right| left.tile_id.cmp(&right.tile_id));
    Ok(columns)
}

/// Executes all tile columns without mutating beginning state. Rain is supplied
/// by exact tile identity so heterogeneous forcing remains distinguishable;
/// the candidate transaction identity is carried into every occupancy solve.
pub fn execute_tile_columns(
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    transaction_id: TransactionId,
    top_rain_kg_m2_tile_ground: &BTreeMap<TileId, f64>,
    pass: ColumnPassKind<'_>,
    solver: &dyn OccupancyPassSolver,
) -> Result<TileColumnsResult, VegetationError> {
    let columns = build_tile_columns(config, beginning)?;
    validate_rain_identity(&columns, top_rain_kg_m2_tile_ground)?;
    validate_authorization_identity(config, pass)?;

    let mut results = Vec::with_capacity(columns.len());
    for column in &columns {
        let top_rain = *top_rain_kg_m2_tile_ground
            .get(&column.tile_id)
            .ok_or(VegetationError::Domain("tile rain identity"))?;
        results.push(execute_tile_column(
            config,
            beginning,
            forcing,
            transaction_id,
            column,
            top_rain,
            pass,
            solver,
        )?);
    }
    let result = TileColumnsResult { columns: results };
    validate_weighted_stand_closure(&result)?;
    Ok(result)
}

#[allow(clippy::too_many_arguments)]
fn execute_tile_column(
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    transaction_id: TransactionId,
    column: &TileColumn,
    top_rain: f64,
    pass: ColumnPassKind<'_>,
    solver: &dyn OccupancyPassSolver,
) -> Result<TileColumnResult, VegetationError> {
    let context = ColumnExecutionContext {
        config,
        beginning,
        forcing,
        transaction_id,
        column,
        pass,
        solver,
    };
    let mut descendant_rain = top_rain;
    let mut ground_stemflow = 0.0;
    let mut routed = Vec::with_capacity(column.occupancies.len());
    let mut ledgers = Vec::with_capacity(column.occupancies.len());

    for occupancy_id in &column.occupancies {
        let (accepted, ledger) = context.solve_occupancy(occupancy_id, descendant_rain)?;
        descendant_rain = accepted.release.throughfall_kg_m2_tile_ground
            + accepted.release.initial_drainage_kg_m2_tile_ground
            + accepted.release.second_drainage_kg_m2_tile_ground;
        ground_stemflow += accepted.release.stemflow_kg_m2_tile_ground;
        routed.push(accepted);
        ledgers.push(ledger);
    }

    finish_tile_column(column, top_rain, ground_stemflow, routed, ledgers)
}

struct ColumnExecutionContext<'a> {
    config: &'a VegetationConfiguration,
    beginning: &'a CoupledOwnedState,
    forcing: &'a SnowFreeForcing,
    transaction_id: TransactionId,
    column: &'a TileColumn,
    pass: ColumnPassKind<'a>,
    solver: &'a dyn OccupancyPassSolver,
}

impl ColumnExecutionContext<'_> {
    fn solve_occupancy(
        &self,
        occupancy_id: &OccupancyId,
        incident_rain: f64,
    ) -> Result<(RoutedOccupancyResult, OccupancyLiquidLedger), VegetationError> {
        let stratum_config = self
            .config
            .strata
            .iter()
            .find(|stratum| stratum.stratum_id == occupancy_id.stratum_id)
            .ok_or(VegetationError::Domain("column stratum identity"))?;
        let shared_state = self
            .beginning
            .strata
            .get(&occupancy_id.stratum_id)
            .ok_or(VegetationError::Domain("column shared state identity"))?;
        let occupancy_state = self
            .beginning
            .occupancies
            .get(occupancy_id)
            .ok_or(VegetationError::Domain("column occupancy state identity"))?;
        let coverage = self.config.stratum_coverage(&occupancy_id.stratum_id)?;
        let result = self.solver.solve(OccupancyPassInput {
            transaction_id: self.transaction_id,
            interval_s: self.config.dt_s,
            occupancy_id,
            tile_fraction: self.column.tile_fraction,
            coverage,
            conditional_lai_m2_m2_tile_ground: shared_state.leaf_area / coverage,
            conditional_wai_m2_m2_tile_ground: shared_state.stem_area / coverage,
            incident_rain_kg_m2_tile_ground: incident_rain,
            local_authorizations_kg_m2_tile_ground: local_authorizations(
                self.pass,
                occupancy_id,
                stratum_config,
                self.column,
            )?,
            shared_state,
            occupancy_state,
            stratum_config,
            forcing: self.forcing,
        })?;
        validate_occupancy_result(
            occupancy_id,
            occupancy_state,
            stratum_config,
            self.column,
            incident_rain,
            self.pass,
            &result,
        )?;
        accept_occupancy_result(
            occupancy_id,
            occupancy_state,
            self.column.tile_fraction,
            incident_rain,
            result,
        )
    }
}

fn accept_occupancy_result(
    occupancy_id: &OccupancyId,
    occupancy_state: &OccupancyState,
    tile_fraction: f64,
    incident_rain: f64,
    result: OccupancyPassResult,
) -> Result<(RoutedOccupancyResult, OccupancyLiquidLedger), VegetationError> {
    let release = OccupancyLiquidRelease {
        throughfall_kg_m2_tile_ground: result.liquid.throughfall,
        initial_drainage_kg_m2_tile_ground: result.liquid.initial_drainage,
        second_drainage_kg_m2_tile_ground: result.liquid.second_drainage,
        stemflow_kg_m2_tile_ground: result.liquid.stemflow,
    };
    let ledger = OccupancyLiquidLedger {
        occupancy_id: occupancy_id.clone(),
        beginning_store_kg_m2_tile_ground: occupancy_state.canopy_liquid_kg_h2o_m2_tile_ground,
        incident_rain_kg_m2_tile_ground: incident_rain,
        condensation_kg_m2_tile_ground: result.liquid.condensation,
        ending_store_kg_m2_tile_ground: result.liquid.store1,
        evaporation_kg_m2_tile_ground: result.liquid.evaporation,
        throughfall_kg_m2_tile_ground: result.liquid.throughfall,
        stemflow_kg_m2_tile_ground: result.liquid.stemflow,
        initial_drainage_kg_m2_tile_ground: result.liquid.initial_drainage,
        second_drainage_kg_m2_tile_ground: result.liquid.second_drainage,
    };
    let independent_residual = ledger.residual_kg_m2_tile_ground();
    validate_water_closure(
        "occupancy liquid",
        independent_residual,
        ledger.operand_sum(),
    )?;
    let stand_ground_layer_water_kg_m2 = result
        .local_layer_water_kg_m2_tile_ground
        .iter()
        .map(|(layer_id, amount)| {
            (
                WaterResourceKey {
                    occupancy_id: occupancy_id.clone(),
                    layer_id: layer_id.clone(),
                },
                tile_fraction * amount,
            )
        })
        .collect();
    let mut accepted_liquid = result.liquid;
    accepted_liquid.closure_residual = independent_residual;
    Ok((
        RoutedOccupancyResult {
            occupancy_id: occupancy_id.clone(),
            candidate_state: result.candidate_state,
            liquid: accepted_liquid,
            release,
            stand_ground_layer_water_kg_m2,
            carbon_operands: result.carbon_operands,
            diagnostics: result.diagnostics,
        },
        ledger,
    ))
}

fn finish_tile_column(
    column: &TileColumn,
    top_rain: f64,
    ground_stemflow: f64,
    routed: Vec<RoutedOccupancyResult>,
    ledgers: Vec<OccupancyLiquidLedger>,
) -> Result<TileColumnResult, VegetationError> {
    let ground_throughfall = if column.occupancies.is_empty() {
        top_rain
    } else {
        routed
            .last()
            .map_or(0.0, |result| result.release.throughfall_kg_m2_tile_ground)
    };
    let ground_drainage = if column.occupancies.is_empty() {
        0.0
    } else {
        routed.last().map_or(0.0, |result| {
            result.release.initial_drainage_kg_m2_tile_ground
                + result.release.second_drainage_kg_m2_tile_ground
        })
    };
    let ledger = TileLiquidLedger {
        tile_id: column.tile_id.clone(),
        tile_fraction: column.tile_fraction,
        top_rain_kg_m2_tile_ground: top_rain,
        occupancies: ledgers,
        ground_throughfall_kg_m2_tile_ground: ground_throughfall,
        ground_drainage_kg_m2_tile_ground: ground_drainage,
        ground_stemflow_kg_m2_tile_ground: ground_stemflow,
    };
    validate_water_closure(
        "tile column liquid",
        ledger.residual_kg_m2_tile_ground(),
        ledger.operand_sum(),
    )?;
    Ok(TileColumnResult {
        tile_id: column.tile_id.clone(),
        occupancy_results: routed,
        ground_throughfall_kg_m2_tile_ground: ground_throughfall,
        ground_drainage_kg_m2_tile_ground: ground_drainage,
        ground_stemflow_kg_m2_tile_ground: ground_stemflow,
        ledger,
    })
}

fn local_authorizations(
    pass: ColumnPassKind<'_>,
    occupancy_id: &OccupancyId,
    stratum: &StratumConfiguration,
    column: &TileColumn,
) -> Result<Option<BTreeMap<SoilLayerId, f64>>, VegetationError> {
    let ColumnPassKind::Final {
        authorizations_kg_m2_stand_ground,
    } = pass
    else {
        return Ok(None);
    };
    let local = stratum
        .root_layers
        .iter()
        .map(|root| {
            let key = WaterResourceKey {
                occupancy_id: occupancy_id.clone(),
                layer_id: root.layer_id.clone(),
            };
            authorizations_kg_m2_stand_ground
                .get(&key)
                .copied()
                .map(|amount| (root.layer_id.clone(), amount / column.tile_fraction))
                .ok_or(VegetationError::Domain("water authorization identity"))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    Ok(Some(local))
}

fn validate_occupancy_result(
    occupancy_id: &OccupancyId,
    beginning: &OccupancyState,
    stratum: &StratumConfiguration,
    _column: &TileColumn,
    incident_rain: f64,
    pass: ColumnPassKind<'_>,
    result: &OccupancyPassResult,
) -> Result<(), VegetationError> {
    let liquid_values = [
        result.liquid.store1,
        result.liquid.evaporation,
        result.liquid.condensation,
        result.liquid.throughfall,
        result.liquid.stemflow,
        result.liquid.initial_drainage,
        result.liquid.second_drainage,
        result.liquid.wet_fraction,
    ];
    if liquid_values
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
        || result.liquid.wet_fraction > 1.0
        || result
            .candidate_state
            .canopy_liquid_kg_h2o_m2_tile_ground
            .to_bits()
            != result.liquid.store1.to_bits()
        || !valid_diagnostics(pass, stratum, &result.diagnostics)
    {
        return Err(VegetationError::Domain("occupancy pass result"));
    }
    let mut configured_layers = stratum
        .root_layers
        .iter()
        .map(|root| root.layer_id.clone())
        .collect::<Vec<_>>();
    configured_layers.sort();
    result
        .candidate_state
        .validate(beginning.last_accepted_transaction_id)
        .map_err(|error| VegetationError::Receipt(error.to_string()))?;

    let mut layers = BTreeSet::new();
    for (layer_id, amount) in &result.local_layer_water_kg_m2_tile_ground {
        if !layers.insert(layer_id) || !configured_layers.contains(layer_id) {
            return Err(VegetationError::Domain("occupancy water layer identity"));
        }
        if !amount.is_finite() || *amount < 0.0 {
            return Err(VegetationError::Domain("occupancy water amount"));
        }
        if let ColumnPassKind::Final {
            authorizations_kg_m2_stand_ground,
        } = pass
        {
            let key = WaterResourceKey {
                occupancy_id: occupancy_id.clone(),
                layer_id: layer_id.clone(),
            };
            let _authorization = authorizations_kg_m2_stand_ground
                .get(&key)
                .ok_or(VegetationError::Domain("water authorization identity"))?;
            // Amount adjudication belongs to the typed resource boundary,
            // which has the frozen interval-aware hydraulic tolerance. This
            // routing layer proves exact authorization identity only; an
            // exact `>` here would reject representational roundoff before
            // that canonical normalization can run.
        }
    }
    if layers.len() != configured_layers.len()
        || configured_layers
            .iter()
            .any(|layer_id| !layers.contains(layer_id))
    {
        return Err(VegetationError::Domain("occupancy water layer identity"));
    }
    if !incident_rain.is_finite() || incident_rain < 0.0 {
        return Err(VegetationError::Domain("occupancy incident rain"));
    }
    Ok(())
}

fn valid_diagnostics(
    pass: ColumnPassKind<'_>,
    stratum: &StratumConfiguration,
    diagnostics: &OccupancyDiagnostics,
) -> bool {
    let expected_pass = match pass {
        ColumnPassKind::Potential => CoupledSolvePass::Potential,
        ColumnPassKind::Final { .. } => CoupledSolvePass::Capped,
    };
    let mut residual_identities = BTreeSet::new();
    let configured_layers = stratum
        .root_layers
        .iter()
        .map(|root| root.layer_id.clone())
        .collect::<Vec<_>>();
    let mut active_caps = BTreeSet::new();
    diagnostics.pass == expected_pass
        && diagnostics.normalized_residuals.iter().all(|residual| {
            !residual.identity.is_empty()
                && residual.value.is_finite()
                && residual_identities.insert(&residual.identity)
        })
        && diagnostics
            .temperature_step_k
            .is_none_or(|value| value.is_finite() && value >= 0.0)
        && diagnostics
            .potential_step_mm
            .is_none_or(|value| value.is_finite() && value >= 0.0)
        && diagnostics.gas_hydraulic_mismatch_kg_m2_s.is_finite()
        && diagnostics
            .pivot_magnitude
            .is_none_or(|value| value.is_finite() && value >= 0.0)
        && diagnostics
            .matrix_norm
            .is_none_or(|value| value.is_finite() && value >= 0.0)
        && diagnostics
            .advanced_t10_k
            .is_none_or(|value| value.is_finite() && value > 0.0)
        && diagnostics
            .active_water_caps
            .iter()
            .all(|layer| configured_layers.contains(layer) && active_caps.insert(layer))
        && diagnostics.active_water_caps
            == configured_layers
                .iter()
                .filter(|layer| diagnostics.active_water_caps.contains(layer))
                .cloned()
                .collect::<Vec<_>>()
        && (diagnostics.pass == CoupledSolvePass::Capped
            || diagnostics.active_water_caps.is_empty())
}

fn validate_rain_identity(
    columns: &[TileColumn],
    rain: &BTreeMap<TileId, f64>,
) -> Result<(), VegetationError> {
    let expected = columns
        .iter()
        .map(|column| column.tile_id.clone())
        .collect::<BTreeSet<_>>();
    let actual = rain.keys().cloned().collect::<BTreeSet<_>>();
    if expected != actual
        || rain
            .values()
            .any(|value| !value.is_finite() || *value < 0.0)
    {
        return Err(VegetationError::Domain("tile rain identity"));
    }
    Ok(())
}

fn validate_authorization_identity(
    config: &VegetationConfiguration,
    pass: ColumnPassKind<'_>,
) -> Result<(), VegetationError> {
    let ColumnPassKind::Final {
        authorizations_kg_m2_stand_ground,
    } = pass
    else {
        return Ok(());
    };
    let expected = config
        .strata
        .iter()
        .flat_map(|stratum| {
            stratum.tile_ids.iter().flat_map(move |tile_id| {
                stratum
                    .root_layers
                    .iter()
                    .map(move |root| WaterResourceKey {
                        occupancy_id: OccupancyId {
                            stratum_id: stratum.stratum_id.clone(),
                            tile_id: tile_id.clone(),
                        },
                        layer_id: root.layer_id.clone(),
                    })
            })
        })
        .collect::<BTreeSet<_>>();
    let actual = authorizations_kg_m2_stand_ground
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    if expected != actual
        || authorizations_kg_m2_stand_ground
            .values()
            .any(|amount| !amount.is_finite() || *amount < 0.0)
    {
        return Err(VegetationError::Domain("water authorization identity"));
    }
    Ok(())
}

fn validate_weighted_stand_closure(result: &TileColumnsResult) -> Result<(), VegetationError> {
    let residual = result.weighted_stand_residual_kg_m2();
    let operand_sum = result
        .columns
        .iter()
        .map(|column| column.ledger.tile_fraction * column.ledger.operand_sum())
        .sum();
    validate_water_closure("weighted stand liquid", residual, operand_sum)
}

fn validate_water_closure(
    ledger: &'static str,
    residual: f64,
    operand_sum: f64,
) -> Result<(), VegetationError> {
    if !residual.is_finite() || !operand_sum.is_finite() {
        return Err(VegetationError::Domain("liquid ledger operand"));
    }
    let tolerance = 1e-12 + 64.0 * f64::EPSILON * operand_sum;
    if residual.abs() > tolerance {
        return Err(VegetationError::Closure { ledger, residual });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use openwepp_kernel_contract::{OccupancyId, SoilLayerId, StratumId, TileId};

    use super::*;
    use crate::MODEL_SHA256;
    use crate::carbon_nitrogen::MaterialTransfer;
    use crate::config::{RootLayer, TopologyTile};
    use crate::interception::{InterceptionInput, liquid_interception};
    use crate::transaction::{SoilLayerForcing, StratumSharedState};

    #[derive(Clone, Debug, PartialEq)]
    struct SeenInput {
        occupancy_id: OccupancyId,
        incident_rain: f64,
        conditional_lai: f64,
        conditional_wai: f64,
        local_authorizations: Option<BTreeMap<SoilLayerId, f64>>,
    }

    struct ControlledSolver {
        vapor_by_occupancy: BTreeMap<OccupancyId, f64>,
        fail_at: Option<OccupancyId>,
        store0_override: Option<f64>,
        rain_override: Option<f64>,
        closure_residual_override: Option<f64>,
        seen: RefCell<Vec<SeenInput>>,
    }

    impl ControlledSolver {
        fn new(vapor_by_occupancy: BTreeMap<OccupancyId, f64>) -> Self {
            Self {
                vapor_by_occupancy,
                fail_at: None,
                store0_override: None,
                rain_override: None,
                closure_residual_override: None,
                seen: RefCell::new(Vec::new()),
            }
        }
    }

    impl OccupancyPassSolver for ControlledSolver {
        fn solve(
            &self,
            input: OccupancyPassInput<'_>,
        ) -> Result<OccupancyPassResult, VegetationError> {
            if self.fail_at.as_ref() == Some(input.occupancy_id) {
                return Err(VegetationError::InjectedFailure("occupancy column"));
            }
            self.seen.borrow_mut().push(SeenInput {
                occupancy_id: input.occupancy_id.clone(),
                incident_rain: input.incident_rain_kg_m2_tile_ground,
                conditional_lai: input.conditional_lai_m2_m2_tile_ground,
                conditional_wai: input.conditional_wai_m2_m2_tile_ground,
                local_authorizations: input.local_authorizations_kg_m2_tile_ground.clone(),
            });
            let vapor_amount = self
                .vapor_by_occupancy
                .get(input.occupancy_id)
                .copied()
                .unwrap_or(0.0);
            let mut liquid = liquid_interception(InterceptionInput {
                store0: self
                    .store0_override
                    .unwrap_or(input.occupancy_state.canopy_liquid_kg_h2o_m2_tile_ground),
                rain: self
                    .rain_override
                    .unwrap_or(input.incident_rain_kg_m2_tile_ground),
                vapor_amount,
                lai: input.conditional_lai_m2_m2_tile_ground,
                sai: input.conditional_wai_m2_m2_tile_ground,
                alpha_liq: input.stratum_config.alpha_liq,
                p_liq: input.stratum_config.p_liq_kg_m2_plant,
                stemflow_fraction: input.stratum_config.stemflow_fraction,
                leaf_temperature_k: input.occupancy_state.wet_surface_temperature_k,
            })?;
            if let Some(residual) = self.closure_residual_override {
                liquid.closure_residual = residual;
            }
            let mut candidate_state = input.occupancy_state.clone();
            candidate_state.canopy_liquid_kg_h2o_m2_tile_ground = liquid.store1;
            let local_layer_water_kg_m2_tile_ground = input
                .stratum_config
                .root_layers
                .iter()
                .enumerate()
                .map(|(index, root)| {
                    let index = u32::try_from(index).expect("test layer count fits u32");
                    let potential = 0.01 * (f64::from(index) + 1.0);
                    let amount = input
                        .local_authorizations_kg_m2_tile_ground
                        .as_ref()
                        .and_then(|caps| caps.get(&root.layer_id))
                        .map_or(potential, |cap| potential.min(*cap));
                    (root.layer_id.clone(), amount)
                })
                .collect();
            Ok(OccupancyPassResult {
                candidate_state,
                liquid,
                local_layer_water_kg_m2_tile_ground,
                carbon_operands: None,
                diagnostics: OccupancyDiagnostics {
                    pass: match input.local_authorizations_kg_m2_tile_ground {
                        Some(_) => CoupledSolvePass::Capped,
                        None => CoupledSolvePass::Potential,
                    },
                    ci_iterations_sun: 0,
                    ci_iterations_shade: 0,
                    energy_iterations: 0,
                    hydraulic_iterations: 0,
                    outer_iterations: 3,
                    normalized_residuals: vec![NormalizedResidual {
                        identity: "controlled_routing".into(),
                        value: 0.25,
                    }],
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
            })
        }
    }

    fn stratum_id(value: &str) -> StratumId {
        StratumId::try_new(value).expect("stratum ID")
    }

    fn tile_id(value: &str) -> TileId {
        TileId::try_new(value).expect("tile ID")
    }

    fn layer_id(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("layer ID")
    }

    fn occupancy(stratum: &str, tile: &str) -> OccupancyId {
        OccupancyId {
            stratum_id: stratum_id(stratum),
            tile_id: tile_id(tile),
        }
    }

    fn shared_state() -> StratumSharedState {
        crate::transaction::v6_identity_rebound_fixture()
            .1
            .strata
            .remove(&stratum_id("tree-1"))
            .expect("V4 shared state")
    }

    fn lane(store: f64, _root_layers: &[&str]) -> OccupancyState {
        OccupancyState {
            beta_hyd: 0.75,
            canopy_air_specific_humidity_kg_kg: 0.009,
            canopy_air_temperature_k: 295.0,
            canopy_liquid_kg_h2o_m2_tile_ground: store,
            dry_stem_temperature_k: 294.0,
            last_accepted_transaction_id: None,
            root_node_potential_mm: -5_000.0,
            shade_ci_pa: 27.0,
            shade_leaf_potential_mm: -7_000.0,
            shade_leaf_temperature_k: 295.0,
            stem_potential_mm: -6_000.0,
            sun_ci_pa: 25.0,
            sun_leaf_potential_mm: -7_500.0,
            sun_leaf_temperature_k: 296.0,
            wet_surface_temperature_k: 295.0,
        }
    }

    fn fixture() -> (VegetationConfiguration, CoupledOwnedState) {
        let (mut config, _) = crate::transaction::v6_identity_rebound_fixture();
        config.initial_state_sha256 = "0".repeat(64);
        config.topology_tiles = vec![
            TopologyTile {
                tile_id: tile_id("tile-a"),
                fraction: 0.3,
            },
            TopologyTile {
                tile_id: tile_id("tile-b"),
                fraction: 0.5,
            },
            TopologyTile {
                tile_id: tile_id("tile-empty"),
                fraction: 0.2,
            },
        ];
        let mut upper = config.strata.remove(0);
        upper.stratum_id = stratum_id("upper");
        upper.tile_ids = vec![tile_id("tile-a"), tile_id("tile-b")];
        let mut second_root: RootLayer = upper.root_layers[0].clone();
        upper.root_layers[0].root_fraction = 0.5;
        upper.root_layers[0].mineral_n_root_fraction = 0.5;
        second_root.layer_id = layer_id("soil-2");
        second_root.root_fraction = 0.5;
        second_root.mineral_n_root_fraction = 0.5;
        upper.root_layers.push(second_root.clone());
        let mut lower = upper.clone();
        lower.stratum_id = stratum_id("lower");
        lower.tile_ids = vec![tile_id("tile-a")];
        lower.vertical_rank = 1;
        lower.height_m = upper.height_m / 2.0;
        lower.crown_base_m = lower.height_m / 2.0;
        lower.root_layers = vec![second_root];
        lower.root_layers[0].root_fraction = 1.0;
        lower.root_layers[0].mineral_n_root_fraction = 1.0;
        config.strata = vec![upper, lower];
        config.configuration_sha256 = config.canonical_sha256().expect("config digest");

        let shared = shared_state();
        let mut state = CoupledOwnedState {
            model_definition_sha256: MODEL_SHA256.into(),
            configuration_sha256: config.configuration_sha256.clone(),
            state_sha256: String::new(),
            strata: BTreeMap::from([
                (stratum_id("upper"), shared.clone()),
                (stratum_id("lower"), shared),
            ]),
            occupancies: BTreeMap::from([
                (
                    occupancy("upper", "tile-a"),
                    lane(0.15, &["soil-1", "soil-2"]),
                ),
                (
                    occupancy("upper", "tile-b"),
                    lane(0.05, &["soil-1", "soil-2"]),
                ),
                (occupancy("lower", "tile-a"), lane(0.10, &["soil-2"])),
            ]),
            last_transaction_id: 0,
        };
        state.state_sha256 = state.canonical_sha256().expect("state digest");
        config.initial_state_sha256 = state.state_sha256.clone();
        (config, state)
    }

    fn forcing() -> SnowFreeForcing {
        SnowFreeForcing {
            air_temperature_k: 296.0,
            pressure_pa: 101_325.0,
            co2_pa: 40.0,
            vapor_pressure_deficit_kpa: 1.0,
            wind_m_s: 2.0,
            rain_kg_m2: 0.0,
            direct_par_w_m2: 100.0,
            diffuse_par_w_m2: 50.0,
            direct_nir_w_m2: 120.0,
            diffuse_nir_w_m2: 60.0,
            solar_zenith_cosine: 0.5,
            ground_albedo_vis: 0.1,
            ground_albedo_nir: 0.2,
            longwave_down_w_m2: 300.0,
            longwave_up_w_m2: 350.0,
            specific_humidity: 0.01,
            reference_height_m: 20.0,
            soil_layers: vec![
                SoilLayerForcing {
                    layer_id: layer_id("soil-1"),
                    water_beginning_kg_m2: 10.0,
                    matric_potential_mm: -1_000.0,
                    hydraulic_conductivity_mm_s: 1e-5,
                    root_path_length_mm: 1.0,
                    gravity_root_mm: 1.0,
                    temperature_k: 290.0,
                    accessible: true,
                    frozen: false,
                },
                SoilLayerForcing {
                    layer_id: layer_id("soil-2"),
                    water_beginning_kg_m2: 10.0,
                    matric_potential_mm: -2_000.0,
                    hydraulic_conductivity_mm_s: 1e-5,
                    root_path_length_mm: 1.0,
                    gravity_root_mm: 1.0,
                    temperature_k: 290.0,
                    accessible: true,
                    frozen: false,
                },
            ],
            gsi: 0.5,
        }
    }

    fn rain() -> BTreeMap<TileId, f64> {
        BTreeMap::from([
            (tile_id("tile-a"), 2.0),
            (tile_id("tile-b"), 0.7),
            (tile_id("tile-empty"), 1.3),
        ])
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() <= 1e-12, "{actual} != {expected}");
    }

    #[test]
    fn columns_are_stable_top_to_bottom_and_empty_tile_routes_rain_to_ground() {
        let (config, state) = fixture();
        let columns = build_tile_columns(&config, &state).expect("columns");
        assert_eq!(
            columns
                .iter()
                .map(|column| column.tile_id.as_str())
                .collect::<Vec<_>>(),
            vec!["tile-a", "tile-b", "tile-empty"]
        );
        assert_eq!(
            columns[0].occupancies,
            vec![occupancy("upper", "tile-a"), occupancy("lower", "tile-a")]
        );

        let solver = ControlledSolver::new(BTreeMap::new());
        let result = execute_tile_columns(
            &config,
            &state,
            &forcing(),
            TransactionId(1),
            &rain(),
            ColumnPassKind::Potential,
            &solver,
        )
        .expect("routing");
        let empty = result
            .columns
            .iter()
            .find(|column| column.tile_id == tile_id("tile-empty"))
            .expect("empty tile");
        assert_close(empty.ground_throughfall_kg_m2_tile_ground, 1.3);
        assert_close(empty.ground_drainage_kg_m2_tile_ground, 0.0);
        assert_close(empty.ground_stemflow_kg_m2_tile_ground, 0.0);
    }

    #[test]
    fn one_occupancy_reduces_to_direct_e04_and_converts_water_once() {
        let (config, state) = fixture();
        let solver = ControlledSolver::new(BTreeMap::new());
        let result = execute_tile_columns(
            &config,
            &state,
            &forcing(),
            TransactionId(1),
            &rain(),
            ColumnPassKind::Potential,
            &solver,
        )
        .expect("routing");
        let tile_b = result
            .columns
            .iter()
            .find(|column| column.tile_id == tile_id("tile-b"))
            .expect("tile b");
        let direct = liquid_interception(InterceptionInput {
            store0: 0.05,
            rain: 0.7,
            vapor_amount: 0.0,
            lai: 2.0 / 0.8,
            sai: 0.7 / 0.8,
            alpha_liq: config.strata[0].alpha_liq,
            p_liq: config.strata[0].p_liq_kg_m2_plant,
            stemflow_fraction: config.strata[0].stemflow_fraction,
            leaf_temperature_k: 295.0,
        })
        .expect("direct E04");
        assert_eq!(tile_b.occupancy_results[0].liquid, direct);
        let key = WaterResourceKey {
            occupancy_id: occupancy("upper", "tile-b"),
            layer_id: layer_id("soil-1"),
        };
        assert_close(
            tile_b.occupancy_results[0].stand_ground_layer_water_kg_m2[&key],
            0.5 * 0.01,
        );
    }

    #[test]
    fn two_ranks_route_throughfall_and_both_drainages_but_bypass_stemflow() {
        let (config, state) = fixture();
        let solver = ControlledSolver::new(BTreeMap::from([(occupancy("upper", "tile-a"), -1.0)]));
        let result = execute_tile_columns(
            &config,
            &state,
            &forcing(),
            TransactionId(1),
            &rain(),
            ColumnPassKind::Potential,
            &solver,
        )
        .expect("routing");
        let tile_a = result
            .columns
            .iter()
            .find(|column| column.tile_id == tile_id("tile-a"))
            .expect("tile a");
        let upper = &tile_a.occupancy_results[0];
        let seen = solver.seen.borrow();
        let lower_input = seen
            .iter()
            .find(|input| input.occupancy_id == occupancy("lower", "tile-a"))
            .expect("lower input");
        assert!(upper.release.second_drainage_kg_m2_tile_ground > 0.0);
        assert_close(
            lower_input.incident_rain,
            upper.release.throughfall_kg_m2_tile_ground
                + upper.release.initial_drainage_kg_m2_tile_ground
                + upper.release.second_drainage_kg_m2_tile_ground,
        );
        assert!(upper.release.stemflow_kg_m2_tile_ground > 0.0);
        assert!(
            (lower_input.incident_rain
                - (upper.release.throughfall_kg_m2_tile_ground
                    + upper.release.initial_drainage_kg_m2_tile_ground
                    + upper.release.second_drainage_kg_m2_tile_ground
                    + upper.release.stemflow_kg_m2_tile_ground))
                .abs()
                > 1e-6
        );
        assert_close(
            tile_a.ground_stemflow_kg_m2_tile_ground,
            tile_a
                .occupancy_results
                .iter()
                .map(|item| item.release.stemflow_kg_m2_tile_ground)
                .sum(),
        );
    }

    #[test]
    fn releases_never_cross_tile_identity_and_distinct_tile_rain_is_retained() {
        let (config, state) = fixture();
        let solver = ControlledSolver::new(BTreeMap::new());
        execute_tile_columns(
            &config,
            &state,
            &forcing(),
            TransactionId(1),
            &rain(),
            ColumnPassKind::Potential,
            &solver,
        )
        .expect("routing");
        let seen = solver.seen.borrow();
        assert_close(
            seen.iter()
                .find(|input| input.occupancy_id == occupancy("upper", "tile-a"))
                .expect("tile a upper")
                .incident_rain,
            2.0,
        );
        assert_close(
            seen.iter()
                .find(|input| input.occupancy_id == occupancy("upper", "tile-b"))
                .expect("tile b upper")
                .incident_rain,
            0.7,
        );
    }

    #[test]
    fn conditional_area_and_local_column_weighted_closure_are_reconstructed() {
        let (config, state) = fixture();
        let solver = ControlledSolver::new(BTreeMap::new());
        let result = execute_tile_columns(
            &config,
            &state,
            &forcing(),
            TransactionId(1),
            &rain(),
            ColumnPassKind::Potential,
            &solver,
        )
        .expect("routing");
        let seen = solver.seen.borrow();
        let upper = seen
            .iter()
            .find(|input| input.occupancy_id == occupancy("upper", "tile-a"))
            .expect("upper");
        assert_close(upper.conditional_lai, 2.0 / 0.8);
        assert_close(upper.conditional_wai, 0.7 / 0.8);
        assert!((upper.conditional_lai - 2.0).abs() > 1e-6);
        let accepted_upper = &result
            .columns
            .iter()
            .find(|column| column.tile_id == tile_id("tile-a"))
            .expect("tile a")
            .occupancy_results[0]
            .liquid;
        let stand_area_poison = liquid_interception(InterceptionInput {
            store0: 0.15,
            rain: 2.0,
            vapor_amount: 0.0,
            lai: 2.0,
            sai: 0.7,
            alpha_liq: config.strata[0].alpha_liq,
            p_liq: config.strata[0].p_liq_kg_m2_plant,
            stemflow_fraction: config.strata[0].stemflow_fraction,
            leaf_temperature_k: 295.0,
        })
        .expect("stand-area poison computes");
        assert!(
            (accepted_upper.throughfall - stand_area_poison.throughfall).abs() > 1e-6,
            "stand-ground LAI must not alias conditional tile-area LAI"
        );
        for column in &result.columns {
            for occupancy in &column.ledger.occupancies {
                assert_close(occupancy.residual_kg_m2_tile_ground(), 0.0);
            }
            assert_close(column.ledger.residual_kg_m2_tile_ground(), 0.0);
        }
        assert_close(result.weighted_stand_residual_kg_m2(), 0.0);
    }

    #[test]
    fn tile_input_order_does_not_change_canonical_columns_or_weighted_totals() {
        let (config, state) = fixture();
        let solver = ControlledSolver::new(BTreeMap::new());
        let expected = execute_tile_columns(
            &config,
            &state,
            &forcing(),
            TransactionId(1),
            &rain(),
            ColumnPassKind::Potential,
            &solver,
        )
        .expect("routing");

        let mut reordered_config = config.clone();
        reordered_config.topology_tiles.reverse();
        reordered_config.configuration_sha256 = reordered_config
            .canonical_sha256()
            .expect("configuration digest");
        let mut reordered_state = state.clone();
        reordered_state.configuration_sha256 = reordered_config.configuration_sha256.clone();
        reordered_state.state_sha256 = reordered_state.canonical_sha256().expect("state digest");
        reordered_config.initial_state_sha256 = reordered_state.state_sha256.clone();
        let actual = execute_tile_columns(
            &reordered_config,
            &reordered_state,
            &forcing(),
            TransactionId(1),
            &rain(),
            ColumnPassKind::Potential,
            &ControlledSolver::new(BTreeMap::new()),
        )
        .expect("reordered routing");
        assert_eq!(
            expected
                .columns
                .iter()
                .map(|column| &column.tile_id)
                .collect::<Vec<_>>(),
            actual
                .columns
                .iter()
                .map(|column| &column.tile_id)
                .collect::<Vec<_>>()
        );
        assert_close(
            expected.weighted_stand_residual_kg_m2(),
            actual.weighted_stand_residual_kg_m2(),
        );
        let expected_ground = expected
            .columns
            .iter()
            .map(|column| {
                column.ledger.tile_fraction
                    * (column.ground_throughfall_kg_m2_tile_ground
                        + column.ground_drainage_kg_m2_tile_ground
                        + column.ground_stemflow_kg_m2_tile_ground)
            })
            .sum::<f64>();
        let actual_ground = actual
            .columns
            .iter()
            .map(|column| {
                column.ledger.tile_fraction
                    * (column.ground_throughfall_kg_m2_tile_ground
                        + column.ground_drainage_kg_m2_tile_ground
                        + column.ground_stemflow_kg_m2_tile_ground)
            })
            .sum::<f64>();
        assert_close(expected_ground, actual_ground);
    }

    #[test]
    fn wrong_store_and_aggregate_first_rain_fail_independent_closure() {
        let (config, state) = fixture();
        let wrong_store = ControlledSolver {
            // Replicating one lane's full store into every occupancy must not
            // close against the other authoritative beginning stores.
            store0_override: Some(0.15),
            ..ControlledSolver::new(BTreeMap::new())
        };
        let wrong_store_result = execute_tile_columns(
            &config,
            &state,
            &forcing(),
            TransactionId(1),
            &rain(),
            ColumnPassKind::Potential,
            &wrong_store,
        );
        assert!(
            matches!(wrong_store_result, Err(VegetationError::Closure { .. })),
            "{wrong_store_result:?}"
        );

        let aggregate_first = ControlledSolver {
            rain_override: Some(1.0),
            ..ControlledSolver::new(BTreeMap::new())
        };
        assert!(matches!(
            execute_tile_columns(
                &config,
                &state,
                &forcing(),
                TransactionId(1),
                &rain(),
                ColumnPassKind::Potential,
                &aggregate_first,
            ),
            Err(VegetationError::Closure { .. })
        ));
    }

    #[test]
    fn producer_supplied_closure_residual_is_ignored_and_reconstructed() {
        let (config, state) = fixture();
        let solver = ControlledSolver {
            closure_residual_override: Some(9_999.0),
            ..ControlledSolver::new(BTreeMap::new())
        };
        let result = execute_tile_columns(
            &config,
            &state,
            &forcing(),
            TransactionId(1),
            &rain(),
            ColumnPassKind::Potential,
            &solver,
        )
        .expect("independently closed routing");
        assert!(result.columns.iter().all(|column| {
            column
                .occupancy_results
                .iter()
                .all(|occupancy| occupancy.liquid.closure_residual.abs() <= 1e-12)
        }));
    }

    #[test]
    fn missing_duplicate_and_wrong_rain_identity_fail_before_routing() {
        let (mut config, state) = fixture();
        let mut missing = state.clone();
        missing.occupancies.remove(&occupancy("upper", "tile-b"));
        missing.state_sha256 = missing.canonical_sha256().expect("state digest");
        config.initial_state_sha256 = missing.state_sha256.clone();
        assert!(build_tile_columns(&config, &missing).is_err());

        let (mut duplicate_config, duplicate_state) = fixture();
        duplicate_config.strata[0].tile_ids.push(tile_id("tile-a"));
        duplicate_config.configuration_sha256 = duplicate_config
            .canonical_sha256()
            .expect("configuration digest");
        assert!(build_tile_columns(&duplicate_config, &duplicate_state).is_err());

        let (config, state) = fixture();
        let mut wrong_rain = rain();
        wrong_rain.remove(&tile_id("tile-b"));
        assert_eq!(
            execute_tile_columns(
                &config,
                &state,
                &forcing(),
                TransactionId(1),
                &wrong_rain,
                ColumnPassKind::Potential,
                &ControlledSolver::new(BTreeMap::new()),
            ),
            Err(VegetationError::Domain("tile rain identity"))
        );
    }

    #[test]
    fn final_pass_back_converts_caps_and_rejects_missing_or_excess_use() {
        let (config, state) = fixture();
        let authorizations = config
            .strata
            .iter()
            .flat_map(|stratum| {
                stratum.tile_ids.iter().flat_map(|tile_id| {
                    stratum.root_layers.iter().map(|root| {
                        (
                            WaterResourceKey {
                                occupancy_id: OccupancyId {
                                    stratum_id: stratum.stratum_id.clone(),
                                    tile_id: tile_id.clone(),
                                },
                                layer_id: root.layer_id.clone(),
                            },
                            0.001,
                        )
                    })
                })
            })
            .collect::<BTreeMap<_, _>>();
        let solver = ControlledSolver::new(BTreeMap::new());
        let result = execute_tile_columns(
            &config,
            &state,
            &forcing(),
            TransactionId(1),
            &rain(),
            ColumnPassKind::Final {
                authorizations_kg_m2_stand_ground: &authorizations,
            },
            &solver,
        )
        .expect("fixed-cap controlled routing");
        let seen = solver.seen.borrow();
        let upper_a = seen
            .iter()
            .find(|input| input.occupancy_id == occupancy("upper", "tile-a"))
            .expect("upper a");
        assert_close(
            upper_a.local_authorizations.as_ref().expect("local caps")[&layer_id("soil-1")],
            0.001 / 0.3,
        );
        let routed = result
            .columns
            .iter()
            .find(|column| column.tile_id == tile_id("tile-a"))
            .expect("tile a");
        let key = WaterResourceKey {
            occupancy_id: occupancy("upper", "tile-a"),
            layer_id: layer_id("soil-1"),
        };
        assert_close(
            routed.occupancy_results[0].stand_ground_layer_water_kg_m2[&key],
            0.001,
        );

        let mut missing = authorizations.clone();
        missing.remove(&key);
        assert_eq!(
            execute_tile_columns(
                &config,
                &state,
                &forcing(),
                TransactionId(1),
                &rain(),
                ColumnPassKind::Final {
                    authorizations_kg_m2_stand_ground: &missing,
                },
                &ControlledSolver::new(BTreeMap::new()),
            ),
            Err(VegetationError::Domain("water authorization identity"))
        );
    }

    #[test]
    fn injected_occupancy_failure_preserves_every_beginning_lane_byte() {
        let (config, state) = fixture();
        let before = serde_json::to_vec(&state).expect("beginning bytes");
        let mut solver = ControlledSolver::new(BTreeMap::new());
        solver.fail_at = Some(occupancy("lower", "tile-a"));
        assert_eq!(
            execute_tile_columns(
                &config,
                &state,
                &forcing(),
                TransactionId(1),
                &rain(),
                ColumnPassKind::Potential,
                &solver,
            ),
            Err(VegetationError::InjectedFailure("occupancy column"))
        );
        assert_eq!(serde_json::to_vec(&state).expect("ending bytes"), before);
        assert!(
            state
                .strata
                .values()
                .all(|shared| shared.pending_transfers == Vec::<MaterialTransfer>::new())
        );
    }
}
