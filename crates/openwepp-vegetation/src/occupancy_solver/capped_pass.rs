//! V3 fixed-authorization column-pass orchestration and finalized water uses.
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
}

/// Constitutive boundary for one exact V3 occupancy under owner-fixed caps.
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
}

impl RadiationBoundCappedSolver<'_> {
    fn try_new<'a>(
        configuration: &VegetationConfiguration,
        prepared: &'a PreparedRadiation,
        evaluator: &'a dyn CappedOccupancyEvaluator,
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
                "V3 capped radiation occupancy identity".into(),
            ));
        }
        Ok(RadiationBoundCappedSolver {
            radiation: &prepared.occupancies,
            evaluator,
        })
    }
}

impl OccupancyPassSolver for RadiationBoundCappedSolver<'_> {
    fn solve(&self, input: OccupancyPassInput<'_>) -> Result<OccupancyPassResult, VegetationError> {
        let Some(local_caps) = input.local_authorizations_kg_m2_tile_ground.as_ref() else {
            return Err(VegetationError::Receipt(
                "owner authorization absent during V3 capped pass".into(),
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
                "V3 capped local authorization identity".into(),
            ));
        }
        let radiation = self.radiation.get(input.occupancy_id).ok_or_else(|| {
            VegetationError::Receipt("V3 capped radiation occupancy identity".into())
        })?;
        if radiation.occupancy_id != *input.occupancy_id
            || radiation.conditional_lai_m2_m2_tile_ground.to_bits()
                != input.conditional_lai_m2_m2_tile_ground.to_bits()
            || radiation.conditional_wai_m2_m2_tile_ground.to_bits()
                != input.conditional_wai_m2_m2_tile_ground.to_bits()
        {
            return Err(VegetationError::Receipt(
                "V3 capped radiation area/occupancy identity".into(),
            ));
        }
        self.evaluator.solve_capped(input, radiation)
    }
}

/// Rebuilds every V3 tile column from the original beginning state under one
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
        return Err(VegetationError::Domain("V3 capped interval duration"));
    }
    if authorizations.transaction_id() != transaction_id {
        return Err(VegetationError::Receipt(
            "V3 capped authorization transaction identity".into(),
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
    let solver = RadiationBoundCappedSolver::try_new(configuration, &radiation, evaluator)?;
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
    Ok(CappedColumnPass {
        columns,
        finalized_water_uses,
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
        .ok_or_else(|| VegetationError::Receipt("V3 transaction identity overflow".into()))?;
    if transaction_id.0 != expected {
        return Err(VegetationError::Receipt(
            "nonsequential V3 capped-pass transaction identity".into(),
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
        return Err(VegetationError::Domain("V3 capped tile fraction"));
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
                        "duplicate or mismatched V3 finalized water identity".into(),
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
            "V3 finalized water key set differs from authorization".into(),
        ));
    }

    authorizations
        .authorizations()
        .iter()
        .map(|(key, authorization)| {
            let raw = amounts.get(key).copied().ok_or_else(|| {
                VegetationError::Receipt("V3 finalized water authorization identity".into())
            })?;
            let amount = authorizations
                .normalize_finalized_stand_amount(key, raw, tile_fractions, interval_s)
                .map_err(|error| resource_boundary_error(&error))?;
            if authorization.transaction_id != authorizations.transaction_id()
                || &authorization.owner_id != authorizations.owner_id()
                || authorization.key != *key
                || authorization.basis != WATER_STAND_BASIS
            {
                return Err(VegetationError::Receipt(
                    "V3 finalized water authorization identity".into(),
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
                VegetationError::Receipt("V3 finalized water request identity".into())
            })?;
            validate_resource_protocol(request, authorization, &finalized).map_err(
                |violation| {
                    VegetationError::Receipt(format!(
                        "V3 finalized water protocol violation: {violation:?}"
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
                    "duplicate V3 capped diagnostic identity".into(),
                ));
            }
        }
    }
    Ok(diagnostics)
}

fn resource_boundary_error(error: &WaterResourceBoundaryError) -> VegetationError {
    VegetationError::Receipt(format!("V3 capped water boundary: {error}"))
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
            Ok(OccupancyPassResult {
                candidate_state,
                liquid,
                local_layer_water_kg_m2_tile_ground: input
                    .stratum_config
                    .root_layers
                    .iter()
                    .map(|root| (root.layer_id.clone(), local_cap))
                    .collect(),
                diagnostics: OccupancyDiagnostics {
                    solver_iterations: 9,
                    normalized_residuals: vec![1.0e-13],
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
        let mut configuration = VegetationConfiguration::parse_strict(include_bytes!(
            "../../../../tests/fixtures/c3_woody_v3_diagnostic_configuration.json"
        ))
        .expect("V3 configuration fixture");
        let original = CoupledOwnedState::parse_strict(
            include_bytes!("../../../../tests/fixtures/c3_woody_v3_diagnostic_state.json"),
            &configuration,
        )
        .expect("V3 state fixture");
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
                if message == "nonsequential V3 capped-pass transaction identity"
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
            Err(VegetationError::Domain("V3 capped interval duration"))
        ));
        assert!(evaluator.seen.borrow().is_empty());
    }
}
