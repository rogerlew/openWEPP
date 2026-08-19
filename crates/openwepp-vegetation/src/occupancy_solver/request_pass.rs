//! V4 owner-uncapped column-pass orchestration and typed water requests.
//!
//! This module binds the complete whole-column radiation preparation to an
//! occupancy constitutive evaluator, executes the routing-only column engine,
//! and publishes one exact stand-ground request per configured occupancy/root
//! layer. It contains no substitute constitutive equations and is deliberately
//! crate-private while the authorization-capped second pass remains pending.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{
    OccupancyId, ResourceOwnerId, TileId, TransactionId, WaterResourceKey,
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
    OccupancyRootLayers, PotentialWaterRequestBatch, WaterResourceBoundaryError,
};
use crate::transaction::validate_candidate_inputs;
use crate::transaction::{CoupledOwnedState, SnowFreeForcing};

/// Complete owner-uncapped pass result. Candidate occupancy states remain
/// unaccepted; the typed request batch is the only resource-boundary output.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PotentialColumnPass {
    pub columns: TileColumnsResult,
    pub water_requests: PotentialWaterRequestBatch,
    pub radiation: PreparedRadiation,
    pub diagnostics: BTreeMap<OccupancyId, OccupancyDiagnostics>,
}

/// Constitutive boundary for one exact V4 occupancy in the owner-uncapped
/// pass. The evaluator must consume the supplied whole-column radiation result
/// and return a complete routing-visible candidate; this orchestrator never
/// invents gas, energy, hydraulic, interception, or demand values.
pub(crate) trait PotentialOccupancyEvaluator {
    fn solve_potential(
        &self,
        input: OccupancyPassInput<'_>,
        radiation: &OccupancyRadiation,
    ) -> Result<OccupancyPassResult, VegetationError>;
}

struct RadiationBoundPotentialSolver<'a> {
    radiation: &'a BTreeMap<OccupancyId, OccupancyRadiation>,
    evaluator: &'a dyn PotentialOccupancyEvaluator,
}

impl RadiationBoundPotentialSolver<'_> {
    fn try_new<'a>(
        configuration: &VegetationConfiguration,
        prepared: &'a PreparedRadiation,
        evaluator: &'a dyn PotentialOccupancyEvaluator,
    ) -> Result<RadiationBoundPotentialSolver<'a>, VegetationError> {
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
                "V4 potential radiation occupancy identity".into(),
            ));
        }
        Ok(RadiationBoundPotentialSolver {
            radiation: &prepared.occupancies,
            evaluator,
        })
    }
}

impl OccupancyPassSolver for RadiationBoundPotentialSolver<'_> {
    fn solve(&self, input: OccupancyPassInput<'_>) -> Result<OccupancyPassResult, VegetationError> {
        if input.local_authorizations_kg_m2_tile_ground.is_some() {
            return Err(VegetationError::Receipt(
                "owner authorization supplied during V4 potential pass".into(),
            ));
        }
        let radiation = self.radiation.get(input.occupancy_id).ok_or_else(|| {
            VegetationError::Receipt("V4 potential radiation occupancy identity".into())
        })?;
        if radiation.occupancy_id != *input.occupancy_id
            || radiation.conditional_lai_m2_m2_tile_ground.to_bits()
                != input.conditional_lai_m2_m2_tile_ground.to_bits()
            || radiation.conditional_wai_m2_m2_tile_ground.to_bits()
                != input.conditional_wai_m2_m2_tile_ground.to_bits()
        {
            return Err(VegetationError::Receipt(
                "V4 potential radiation area/occupancy identity".into(),
            ));
        }
        self.evaluator.solve_potential(input, radiation)
    }
}

/// Executes the complete V4 owner-uncapped tile-column pass and constructs the
/// typed stand-ground request batch without mutating beginning state.
#[allow(clippy::too_many_arguments)]
pub(crate) fn execute_potential_column_pass(
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    transaction_id: TransactionId,
    owner_id: ResourceOwnerId,
    top_rain_kg_m2_tile_ground: &BTreeMap<TileId, f64>,
    evaluator: &dyn PotentialOccupancyEvaluator,
) -> Result<PotentialColumnPass, VegetationError> {
    validate_candidate_transaction(beginning, transaction_id)?;
    validate_candidate_inputs(configuration, beginning, forcing)?;
    let radiation = prepare_whole_column_radiation(configuration, beginning, forcing)?;
    let solver = RadiationBoundPotentialSolver::try_new(configuration, &radiation, evaluator)?;
    let columns = execute_tile_columns(
        configuration,
        beginning,
        forcing,
        transaction_id,
        top_rain_kg_m2_tile_ground,
        ColumnPassKind::Potential,
        &solver,
    )?;
    let configured_root_layers = configured_root_layers(configuration)?;
    let amounts = collect_stand_ground_amounts(&columns)?;
    let water_requests = PotentialWaterRequestBatch::try_from_stand_amounts(
        transaction_id,
        owner_id,
        &configured_root_layers,
        &amounts,
    )
    .map_err(|error| resource_boundary_error(&error))?;
    let diagnostics = collect_diagnostics(&columns)?;
    Ok(PotentialColumnPass {
        columns,
        water_requests,
        radiation,
        diagnostics,
    })
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
            "nonsequential V4 potential-pass transaction identity".into(),
        ));
    }
    Ok(())
}

fn configured_root_layers(
    configuration: &VegetationConfiguration,
) -> Result<Vec<OccupancyRootLayers>, VegetationError> {
    let strata = configuration
        .strata
        .iter()
        .map(|stratum| (&stratum.stratum_id, stratum))
        .collect::<BTreeMap<_, _>>();
    configuration
        .expected_occupancies()
        .into_iter()
        .map(|occupancy_id| {
            let stratum = strata
                .get(&occupancy_id.stratum_id)
                .ok_or(VegetationError::Domain(
                    "V4 potential request stratum identity",
                ))?;
            Ok(OccupancyRootLayers {
                occupancy_id,
                layer_ids: stratum
                    .root_layers
                    .iter()
                    .map(|root| root.layer_id.clone())
                    .collect(),
            })
        })
        .collect()
}

fn collect_stand_ground_amounts(
    columns: &TileColumnsResult,
) -> Result<BTreeMap<WaterResourceKey, f64>, VegetationError> {
    let mut amounts = BTreeMap::new();
    for column in &columns.columns {
        for occupancy in &column.occupancy_results {
            for (key, amount) in &occupancy.stand_ground_layer_water_kg_m2 {
                if key.occupancy_id != occupancy.occupancy_id
                    || amounts.insert(key.clone(), *amount).is_some()
                {
                    return Err(VegetationError::Receipt(
                        "duplicate or mismatched V4 potential water identity".into(),
                    ));
                }
            }
        }
    }
    Ok(amounts)
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
                    "duplicate V4 potential diagnostic identity".into(),
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

    use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId};

    use super::*;
    use crate::config::TopologyTile;
    use crate::interception::{InterceptionInput, liquid_interception};
    use crate::transaction::SoilLayerForcing;

    struct ControlledEvaluator {
        seen: RefCell<Vec<(OccupancyId, TransactionId)>>,
        fail: bool,
        local_amount_kg_m2_tile: f64,
    }

    impl PotentialOccupancyEvaluator for ControlledEvaluator {
        fn solve_potential(
            &self,
            input: OccupancyPassInput<'_>,
            radiation: &OccupancyRadiation,
        ) -> Result<OccupancyPassResult, VegetationError> {
            self.seen
                .borrow_mut()
                .push((input.occupancy_id.clone(), input.transaction_id));
            if self.fail {
                return Err(VegetationError::InjectedFailure("controlled potential"));
            }
            assert_eq!(radiation.occupancy_id, *input.occupancy_id);
            let liquid = liquid_interception(InterceptionInput {
                store0: input.occupancy_state.canopy_liquid_kg_h2o_m2_tile_ground,
                rain: input.incident_rain_kg_m2_tile_ground,
                vapor_amount: 0.0,
                lai: input.conditional_lai_m2_m2_tile_ground,
                sai: input.conditional_wai_m2_m2_tile_ground,
                alpha_liq: input.stratum_config.alpha_liq,
                p_liq: input.stratum_config.p_liq_kg_m2_plant,
                stemflow_fraction: input.stratum_config.stemflow_fraction,
                leaf_temperature_k: input.occupancy_state.sun_leaf_temperature_k,
            })?;
            let mut candidate_state = input.occupancy_state.clone();
            candidate_state.canopy_liquid_kg_h2o_m2_tile_ground = liquid.store1;
            Ok(OccupancyPassResult {
                candidate_state,
                liquid,
                local_layer_water_kg_m2_tile_ground: input
                    .stratum_config
                    .root_layers
                    .iter()
                    .map(|root| (root.layer_id.clone(), self.local_amount_kg_m2_tile))
                    .collect(),
                carbon_operands: None,
                energy_proposal: None,
                diagnostics: OccupancyDiagnostics {
                    pass: crate::diagnostics::CoupledSolvePass::Potential,
                    ci_iterations_sun: 0,
                    ci_iterations_shade: 0,
                    energy_iterations: 0,
                    hydraulic_iterations: 0,
                    outer_iterations: 7,
                    normalized_residuals: vec![crate::diagnostics::NormalizedResidual {
                        identity: "controlled_potential".into(),
                        value: 1.0e-13,
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

    fn fixture() -> (VegetationConfiguration, CoupledOwnedState) {
        crate::transaction::v7_identity_rebound_fixture()
    }

    fn forcing(rain_kg_m2: f64) -> SnowFreeForcing {
        SnowFreeForcing {
            air_temperature_k: 298.15,
            pressure_pa: 101_325.0,
            co2_pa: 42.0,
            vapor_pressure_deficit_kpa: 1.2,
            wind_m_s: 3.7,
            rain_kg_m2,
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

    fn owner() -> ResourceOwnerId {
        ResourceOwnerId::try_new("vegetation").expect("owner identity")
    }

    fn fractional_fixture() -> (VegetationConfiguration, CoupledOwnedState) {
        let (mut configuration, mut beginning) = fixture();
        configuration.topology_tiles[0].fraction = 0.25;
        configuration.topology_tiles.push(TopologyTile {
            tile_id: TileId::try_new("tile-empty").expect("tile identity"),
            fraction: 0.75,
        });
        configuration.configuration_sha256 = configuration
            .canonical_sha256()
            .expect("configuration digest");
        beginning.configuration_sha256 = configuration.configuration_sha256.clone();
        beginning.state_sha256 = beginning.canonical_sha256().expect("state digest");
        configuration.initial_state_sha256 = beginning.state_sha256.clone();
        configuration.validate().expect("fractional configuration");
        beginning
            .validate(&configuration)
            .expect("fractional beginning state");
        (configuration, beginning)
    }

    #[test]
    fn binds_radiation_and_converts_local_demand_to_typed_stand_request_once() {
        let (configuration, beginning) = fractional_fixture();
        let tile_id = configuration.topology_tiles[0].tile_id.clone();
        let empty_tile_id = configuration.topology_tiles[1].tile_id.clone();
        let rain = BTreeMap::from([(tile_id, 0.8), (empty_tile_id, 0.2)]);
        let evaluator = ControlledEvaluator {
            seen: RefCell::new(Vec::new()),
            fail: false,
            local_amount_kg_m2_tile: 0.25,
        };
        let result = execute_potential_column_pass(
            &configuration,
            &beginning,
            &forcing(0.8),
            TransactionId(1),
            owner(),
            &rain,
            &evaluator,
        )
        .expect("controlled potential pass");

        assert_eq!(result.water_requests.requests().len(), 1);
        let request = &result.water_requests.requests()[0];
        assert_eq!(request.transaction_id, TransactionId(1));
        assert_eq!(request.owner_id, owner());
        assert_eq!(request.amount.to_bits(), 0.0625_f64.to_bits());
        assert_eq!(result.radiation.occupancies.len(), 1);
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.columns.columns.len(), 2);
        assert_eq!(evaluator.seen.borrow().len(), 1);
    }

    #[test]
    fn rejects_stale_transaction_before_evaluation() {
        let (configuration, beginning) = fixture();
        let evaluator = ControlledEvaluator {
            seen: RefCell::new(Vec::new()),
            fail: false,
            local_amount_kg_m2_tile: 0.0,
        };
        let tile_id = configuration.topology_tiles[0].tile_id.clone();
        assert!(matches!(
            execute_potential_column_pass(
                &configuration,
                &beginning,
                &forcing(0.0),
                TransactionId(0),
                owner(),
                &BTreeMap::from([(tile_id, 0.0)]),
                &evaluator,
            ),
            Err(VegetationError::Receipt(message))
                if message == "nonsequential V4 potential-pass transaction identity"
        ));
        assert!(evaluator.seen.borrow().is_empty());

        assert!(matches!(
            execute_potential_column_pass(
                &configuration,
                &beginning,
                &forcing(0.0),
                TransactionId(2),
                owner(),
                &BTreeMap::from([(
                    configuration.topology_tiles[0].tile_id.clone(),
                    0.0,
                )]),
                &evaluator,
            ),
            Err(VegetationError::Receipt(message))
                if message == "nonsequential V4 potential-pass transaction identity"
        ));
        assert!(evaluator.seen.borrow().is_empty());
    }

    #[test]
    fn evaluator_failure_leaves_beginning_state_byte_identical() {
        let (configuration, beginning) = fixture();
        let before = serde_json::to_vec(&beginning).expect("serialize beginning state");
        let evaluator = ControlledEvaluator {
            seen: RefCell::new(Vec::new()),
            fail: true,
            local_amount_kg_m2_tile: 0.0,
        };
        let tile_id = configuration.topology_tiles[0].tile_id.clone();
        assert_eq!(
            execute_potential_column_pass(
                &configuration,
                &beginning,
                &forcing(0.0),
                TransactionId(1),
                owner(),
                &BTreeMap::from([(tile_id, 0.0)]),
                &evaluator,
            ),
            Err(VegetationError::InjectedFailure("controlled potential"))
        );
        assert_eq!(
            serde_json::to_vec(&beginning).expect("serialize beginning state"),
            before
        );
    }

    #[test]
    fn complete_validation_rejects_soil_seam_before_evaluator() {
        let (configuration, beginning) = fixture();
        let evaluator = ControlledEvaluator {
            seen: RefCell::new(Vec::new()),
            fail: false,
            local_amount_kg_m2_tile: 0.0,
        };
        let rain = BTreeMap::from([(configuration.topology_tiles[0].tile_id.clone(), 0.0)]);
        let mut cases = Vec::new();
        let mut missing = forcing(0.0);
        missing.soil_layers.clear();
        cases.push(missing);
        let mut duplicate = forcing(0.0);
        duplicate.soil_layers.push(duplicate.soil_layers[0].clone());
        cases.push(duplicate);
        let mut invalid = forcing(0.0);
        invalid.soil_layers[0].root_path_length_mm = 0.0;
        cases.push(invalid);
        for forcing in cases {
            assert!(
                execute_potential_column_pass(
                    &configuration,
                    &beginning,
                    &forcing,
                    TransactionId(1),
                    owner(),
                    &rain,
                    &evaluator,
                )
                .is_err()
            );
            assert!(evaluator.seen.borrow().is_empty());
        }
    }
}
