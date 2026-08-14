use std::fs;
use std::path::Path;

use openwepp_biogeochemistry::{BiogeochemistryState, MaterialPool, MineralLayer};
use openwepp_hillslope_orchestrator::vegetation_diagnostic::{
    DiagnosticError, DiagnosticOwnedState, DiagnosticWaterState,
    run_default_off_diagnostic_at_phase,
};
use openwepp_hillslope_orchestrator::vegetation_energy_owner::{
    CanopyHeatStorageMode, DiagnosticEnergyState, construct_energy_owner_candidate,
};
use openwepp_kernel_contract::{
    MaterialReceiverClass, MaximumAuthorization, OccupancyId, ResourceOwnerId, SoilLayerId,
    StratumId, TileId, TransactionId, WaterResourceKey,
};
use openwepp_vegetation::carbon_nitrogen::{
    CnParameters, ElementPool, GrowthNitrogenReceipt, PhenologyMode, ReceiverClass, Tissue,
    TissuePool, advance_phenology, carbon_offer, finalize_growth, material_transfer,
    nitrogen_demand,
};
use openwepp_vegetation::energy::{
    energy_residual, neutral_resistance, saturation_specific_humidity,
};
use openwepp_vegetation::interception::{InterceptionInput, liquid_interception};
use openwepp_vegetation::migration::{
    RhessysSource, V5_MODEL_SHA256, V6_MODEL_SHA256, migrate_definition_fields,
};
use openwepp_vegetation::occupancy_solver::resources::{
    OccupancyRootLayers, PotentialWaterRequestBatch, ValidatedWaterAuthorizations,
};
use openwepp_vegetation::photosynthesis::{FvcbInput, fvcb, medlyn};
use openwepp_vegetation::radiation::two_stream;
use openwepp_vegetation::{
    CoupledOwnedState, FailurePoint, MODEL_BYTES, MODEL_SHA256, NitrogenArbiter,
    NitrogenAuthorization, NitrogenRequest, PhenologyPhase, SnowFreeForcing, SoilLayerForcing,
    VegetationConfiguration, VegetationError, WaterArbiter, WaterArbitration,
    WaterAuthorizationReason, WaterOwnerCandidate, WaterOwnerSnapshot, WaterRequest, WaterUse,
    execute_candidate, execute_uncommitted_water_phase, load_model_definition,
    reconstruct_water_ending,
};
use sha2::{Digest, Sha256};

fn expected() -> serde_json::Value {
    serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v1_expected_vectors.json").expect("vector fixture"),
    )
    .expect("valid vector fixture")
}

fn expected_v5() -> serde_json::Value {
    serde_json::from_slice(
        &fs::read("docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v5_vectors.json")
            .expect("V5 capped-pass vector fixture"),
    )
    .expect("valid V5 vector fixture")
}

fn identity_rebound_v7_configuration() -> VegetationConfiguration {
    let mut configuration: VegetationConfiguration = serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v5_diagnostic_configuration.json")
            .expect("historical V5 configuration fixture"),
    )
    .expect("historical V5 configuration DTO");
    assert_eq!(configuration.model_definition_sha256, V5_MODEL_SHA256);

    for stratum in &mut configuration.strata {
        if stratum.phenology_type == openwepp_vegetation::PhenologyType::Evergreen {
            stratum.current_growth_fraction = 1.0;
        }
    }
    configuration.model_definition_sha256 = MODEL_SHA256.into();
    configuration.configuration_sha256 = configuration
        .canonical_sha256()
        .expect("V7 configuration digest");
    let bytes = serde_json::to_vec(&configuration).expect("identity-rebound V7 bytes");
    VegetationConfiguration::parse_strict(&bytes).expect("identity-rebound V7 configuration")
}

fn identity_rebound_v7_fixture() -> (VegetationConfiguration, CoupledOwnedState) {
    let mut configuration = identity_rebound_v7_configuration();
    let mut state: CoupledOwnedState = serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v5_diagnostic_state.json")
            .expect("historical V5 state fixture"),
    )
    .expect("historical V5 state DTO");
    state.model_definition_sha256 = MODEL_SHA256.into();
    state
        .configuration_sha256
        .clone_from(&configuration.configuration_sha256);
    state.state_sha256 = state.canonical_sha256().expect("V7 state digest");
    configuration
        .initial_state_sha256
        .clone_from(&state.state_sha256);
    state.validate(&configuration).expect("V7 state");
    (configuration, state)
}

fn two_stratum_competition_fixture() -> (VegetationConfiguration, CoupledOwnedState) {
    let (mut configuration, mut state) = identity_rebound_v7_fixture();
    let second_id = StratumId::try_new("tree-2").expect("second stratum");
    let tile_id = TileId::try_new("tile-1").expect("shared tile");

    let mut second_config = configuration.strata[0].clone();
    second_config.stratum_id = second_id.clone();
    second_config.vertical_rank = 1;
    second_config.height_m = 8.0;
    second_config.crown_base_m = 1.5;
    second_config.displacement_m = 4.8;
    configuration.strata.push(second_config);
    configuration.configuration_sha256.clear();
    configuration.configuration_sha256 = configuration
        .canonical_sha256()
        .expect("two-stratum configuration digest");

    let first_id = StratumId::try_new("tree-1").expect("first stratum");
    let second_shared = state.strata[&first_id].clone();
    state.strata.insert(second_id.clone(), second_shared);
    let first_occupancy = OccupancyId {
        stratum_id: first_id,
        tile_id: tile_id.clone(),
    };
    let second_occupancy = OccupancyId {
        stratum_id: second_id,
        tile_id,
    };
    let second_lane = state.occupancies[&first_occupancy].clone();
    state.occupancies.insert(second_occupancy, second_lane);
    state
        .configuration_sha256
        .clone_from(&configuration.configuration_sha256);
    state.state_sha256 = state.canonical_sha256().expect("two-stratum state digest");
    configuration
        .initial_state_sha256
        .clone_from(&state.state_sha256);
    state
        .validate(&configuration)
        .expect("two-stratum competition fixture");
    (configuration, state)
}

struct IntegrationWater;
impl WaterArbiter for IntegrationWater {
    fn authorize(&self, requests: &[WaterRequest]) -> Result<WaterArbitration, VegetationError> {
        let authorizations = requests
            .iter()
            .map(|request| MaximumAuthorization {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount: request.amount,
                basis: request.basis,
            })
            .collect::<Vec<_>>();
        let reasons: std::collections::BTreeMap<_, _> = requests
            .iter()
            .map(|request| {
                (
                    request.key.clone(),
                    if request.amount == 0.0 {
                        WaterAuthorizationReason::ZeroDemand
                    } else {
                        WaterAuthorizationReason::FullySupplied
                    },
                )
            })
            .collect();
        let snapshot = WaterOwnerSnapshot::try_new(
            requests[0].transaction_id,
            requests[0].owner_id.clone(),
            std::collections::BTreeMap::from([(requests[0].key.layer_id.clone(), 20.0)]),
            reasons.clone(),
        )?;
        WaterArbitration::try_new(snapshot, authorizations, reasons)
    }
    fn candidate_from_finalized_use(
        &self,
        transaction_id: TransactionId,
        arbitration: &WaterArbitration,
        finalized_uses: &[WaterUse],
    ) -> Result<WaterOwnerCandidate, VegetationError> {
        let ending = reconstruct_water_ending(arbitration.snapshot(), finalized_uses)?;
        WaterOwnerCandidate::try_new(
            transaction_id,
            arbitration.snapshot().owner_id().clone(),
            arbitration.snapshot().clone(),
            ending,
            finalized_uses.to_vec(),
        )
    }
}

struct IntegrationNitrogen;
impl NitrogenArbiter for IntegrationNitrogen {
    fn beginning_amount(
        &self,
        _key: &openwepp_kernel_contract::MineralNitrogenKey,
    ) -> Result<f64, VegetationError> {
        Ok(1.0)
    }

    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
        Ok(requests
            .iter()
            .map(|request| MaximumAuthorization {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount: request.amount,
                basis: request.basis,
            })
            .collect())
    }
}

fn public_water_forcing() -> SnowFreeForcing {
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
            layer_id: SoilLayerId::try_new("soil-1").expect("layer"),
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

fn rust_sources_below(path: &Path, sources: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(path).expect("production source directory") {
        let entry = entry.expect("production source entry");
        let path = entry.path();
        if path.is_dir() {
            rust_sources_below(&path, sources);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(path);
        }
    }
}

#[test]
fn v7_diagnostic_has_no_production_selector_or_legacy_pmet_gsi_entry_point() {
    let mut production_sources = Vec::new();
    rust_sources_below(
        Path::new("crates/openwepp-runner/src"),
        &mut production_sources,
    );
    rust_sources_below(
        Path::new("crates/openwepp-hillslope-orchestrator/src/direct_runtime"),
        &mut production_sources,
    );
    production_sources.sort();
    assert!(!production_sources.is_empty());

    for path in production_sources {
        let source = fs::read_to_string(&path).expect("production Rust source");
        for forbidden in [
            "run_default_off_diagnostic",
            "run_default_off_diagnostic_at_phase",
            "OPENWEPP_C3_WOODY_V7",
            "openwepp_vegetation::execute_candidate",
        ] {
            assert!(
                !source.contains(forbidden),
                "default-off vegetation diagnostic leaked into production source {} through {forbidden}",
                path.display()
            );
        }
    }

    let diagnostic =
        fs::read_to_string("crates/openwepp-hillslope-orchestrator/src/vegetation_diagnostic.rs")
            .expect("diagnostic source");
    assert!(diagnostic.contains("run_default_off_diagnostic"));
    assert!(diagnostic.contains("execute_candidate_with_failure"));
}

fn assert_fvcb_vector(
    input: FvcbInput,
    key: &str,
) -> openwepp_vegetation::photosynthesis::FvcbResult {
    let result = fvcb(input).expect("FvCB vector");
    let expected = expected();
    assert!((result.an - expected["photosynthesis"][key].as_f64().unwrap()).abs() < 1e-12);
    result
}

#[test]
fn v7_public_candidate_is_sealed_and_energy_owner_consumes_real_capped_operands() {
    let source = fs::read_to_string("crates/openwepp-vegetation/src/transaction.rs")
        .expect("transaction source");
    assert!(!source.contains("pub fn validate_and_commit"));
    assert!(!source.contains("NitrogenDemandOrdering"));
    let nitrogen_source = fs::read_to_string("crates/openwepp-vegetation/src/nitrogen_protocol.rs")
        .expect("nitrogen protocol source");
    assert!(!nitrogen_source.contains("FinalDemandExceedsPotential"));
    assert!(!nitrogen_source.contains("potential_total_demand.max"));
    assert!(!nitrogen_source.contains("final_total_demand.min(self.request_batch"));
    assert!(!nitrogen_source.contains("3.0e-7"));
    assert!(!nitrogen_source.contains("step_norm"));
    let water_source =
        fs::read_to_string("crates/openwepp-vegetation/src/water_phase.rs").expect("water source");
    assert!(water_source.contains("execute_potential_column_pass"));
    assert!(water_source.contains("execute_capped_column_pass"));
    assert!(!water_source.contains("validate_and_commit"));
    assert!(source.contains("BTreeMap<OccupancyId, OccupancyState>"));
    assert!(!source.contains("struct StratumState"));
    assert!(!source.contains("pub canopy_liquid: f64"));
    assert!(!source.contains("pub psi_root_mm: f64"));
    assert!(!source.contains("ledger_residuals: [0.0; 5]"));
    assert!(!source.contains("vapor_pressure_deficit_kpa *"));
    assert!(!source.contains("direct_par_w_m2 * 1e-9"));

    let (configuration, beginning) = identity_rebound_v7_fixture();
    let bytes = serde_json::to_vec(&beginning).expect("beginning bytes");
    let phase = execute_uncommitted_water_phase(
        &load_model_definition().expect("model"),
        &configuration,
        &beginning,
        &public_water_forcing(),
        &IntegrationWater,
    )
    .expect("public uncommitted water phase");
    let (requests, authorizations, uses) = phase.protocol();
    assert!(!requests.is_empty());
    assert_eq!(requests.len(), authorizations.len());
    assert_eq!(requests.len(), uses.len());
    assert_eq!(phase.water_operands().len(), uses.len());
    let candidate = execute_candidate(
        &load_model_definition().expect("model"),
        &configuration,
        &beginning,
        &public_water_forcing(),
        &IntegrationWater,
        &IntegrationNitrogen,
    )
    .expect("sealed public vegetation candidate");
    candidate
        .validate_sealed()
        .expect("sealed candidate identity");
    let energy_beginning = DiagnosticEnergyState {
        model_definition_sha256: MODEL_SHA256.into(),
        configuration_sha256: configuration.configuration_sha256.clone(),
        accepted_vegetation_state_sha256: beginning.state_sha256.clone(),
        last_transaction_id: 0,
        last_operands: None,
    };
    let energy_topology = openwepp_hillslope_orchestrator::vegetation_energy_owner::EnergyOwnerTopology::from_configuration(&configuration)
        .expect("energy owner topology");
    let energy_candidate = construct_energy_owner_candidate(
        &energy_beginning,
        &energy_topology,
        candidate.energy_proposals(),
        CanopyHeatStorageMode::EquilibriumZero,
    )
    .expect("independently reconstructed production energy candidate");
    assert_eq!(energy_candidate.transaction_id(), TransactionId(1));
    assert_eq!(
        energy_candidate.occupancy_receipts().len(),
        configuration.expected_occupancies().len()
    );
    assert_eq!(
        energy_candidate.ending().accepted_vegetation_state_sha256,
        candidate.ending_state().state_sha256
    );
    assert_eq!(serde_json::to_vec(&beginning).expect("after bytes"), bytes);
}

fn diagnostic_owned_state(
    configuration: &VegetationConfiguration,
    vegetation: CoupledOwnedState,
) -> DiagnosticOwnedState {
    let receivers = [
        MaterialReceiverClass::Metabolic,
        MaterialReceiverClass::Cellulose,
        MaterialReceiverClass::Lignin,
        MaterialReceiverClass::CoarseWoodyDebris,
    ]
    .into_iter()
    .map(|receiver| (receiver, MaterialPool::default()))
    .collect();
    DiagnosticOwnedState {
        energy: DiagnosticEnergyState {
            model_definition_sha256: MODEL_SHA256.into(),
            configuration_sha256: configuration.configuration_sha256.clone(),
            accepted_vegetation_state_sha256: vegetation.state_sha256.clone(),
            last_transaction_id: vegetation.last_transaction_id,
            last_operands: None,
        },
        vegetation,
        water: DiagnosticWaterState {
            liquid_kg_m2: std::collections::BTreeMap::from([(
                SoilLayerId::try_new("soil-1").expect("layer"),
                20.0,
            )]),
            last_transaction_id: 0,
        },
        biogeochemistry: BiogeochemistryState {
            layers: std::collections::BTreeMap::from([(
                "soil-1".into(),
                MineralLayer {
                    ammonium_n: 1.0,
                    nitrate_n: 1.0,
                },
            )]),
            receivers,
            last_transaction_id: 0,
        },
    }
}

#[test]
fn v7_default_off_diagnostic_commits_all_owners_once_and_rolls_back_every_phase() {
    let (configuration, vegetation) = identity_rebound_v7_fixture();
    let model = load_model_definition().expect("model");
    let forcing = public_water_forcing();
    let initial = diagnostic_owned_state(&configuration, vegetation);

    let mut accepted = initial.clone();
    let available = accepted.water.liquid_kg_m2.clone();
    let receipt = run_default_off_diagnostic_at_phase(
        &mut accepted,
        &model,
        &configuration,
        &forcing,
        &available,
        None,
    )
    .expect("atomic default-off transaction");
    assert_eq!(receipt.transaction_id, TransactionId(1));
    assert_eq!(accepted.vegetation.last_transaction_id, 1);
    assert_eq!(accepted.water.last_transaction_id, 1);
    assert_eq!(accepted.biogeochemistry.last_transaction_id, 1);
    assert_eq!(accepted.energy.last_transaction_id, 1);
    assert_eq!(
        accepted.energy.accepted_vegetation_state_sha256,
        accepted.vegetation.state_sha256
    );

    for failure in [
        FailurePoint::Validation,
        FailurePoint::Radiation,
        FailurePoint::Interception,
        FailurePoint::PotentialCoupledSolve,
        FailurePoint::WaterAuthorization,
        FailurePoint::CappedResolve,
        FailurePoint::NitrogenRequest,
        FailurePoint::NitrogenAuthorization,
        FailurePoint::Allocation,
        FailurePoint::ReceiverConstruction,
        FailurePoint::BiogeochemistryCandidate,
        FailurePoint::ProposalReceiptValidation,
        FailurePoint::EnergyOperandConstruction,
        FailurePoint::EnergyOwnerValidation,
        FailurePoint::ClosureValidation,
        FailurePoint::VegetationOwnerValidation,
        FailurePoint::WaterOwnerValidation,
        FailurePoint::BiogeochemistryOwnerValidation,
        FailurePoint::OwnerValidation,
        FailurePoint::CrossOwnerValidation,
        FailurePoint::CrossOwnerWaterCandidateMismatch,
        FailurePoint::CrossOwnerNitrogenProtocolMismatch,
        FailurePoint::CrossOwnerEnergyIdentityMismatch,
        FailurePoint::CrossOwnerTransactionMismatch,
        FailurePoint::CrossOwnerBeginningStateMismatch,
        FailurePoint::CrossOwnerMaterialReceiptMismatch,
        FailurePoint::BeforeCommit,
    ] {
        let mut owned = initial.clone();
        let before = serde_json::to_vec(&owned).expect("beginning owner bytes");
        let available = owned.water.liquid_kg_m2.clone();
        let error = run_default_off_diagnostic_at_phase(
            &mut owned,
            &model,
            &configuration,
            &forcing,
            &available,
            Some(failure),
        )
        .expect_err("injected or poisoned phase must reject");
        if matches!(
            failure,
            FailurePoint::CrossOwnerWaterCandidateMismatch
                | FailurePoint::CrossOwnerNitrogenProtocolMismatch
                | FailurePoint::CrossOwnerEnergyIdentityMismatch
                | FailurePoint::CrossOwnerTransactionMismatch
                | FailurePoint::CrossOwnerBeginningStateMismatch
        ) {
            assert!(
                matches!(error, DiagnosticError::OwnerEnvelopeIdentity(_)),
                "cross-owner poison {failure:?} must reach typed envelope validation: {error:?}"
            );
        } else if failure == FailurePoint::CrossOwnerMaterialReceiptMismatch {
            assert!(
                matches!(error, DiagnosticError::Biogeochemistry(_)),
                "material receipt poison must reach cross-owner validation: {error:?}"
            );
        } else {
            assert!(
                matches!(
                    error,
                    DiagnosticError::InjectedFailure | DiagnosticError::Vegetation(_)
                ),
                "failure point {failure:?} must reject at its intended phase: {error:?}"
            );
        }
        assert_eq!(
            serde_json::to_vec(&owned).expect("ending owner bytes"),
            before,
            "failure point {failure:?} must roll back every owner"
        );
    }
}

#[test]
fn v7_real_diagnostic_activates_shared_water_and_species_n_competition() {
    let (configuration, vegetation) = two_stratum_competition_fixture();
    let model = load_model_definition().expect("model");
    let forcing = public_water_forcing();
    let mut owned = diagnostic_owned_state(&configuration, vegetation);
    let layer = SoilLayerId::try_new("soil-1").expect("layer");
    owned.water.liquid_kg_m2.insert(layer.clone(), 0.05);
    owned
        .biogeochemistry
        .layers
        .get_mut("soil-1")
        .expect("mineral layer")
        .ammonium_n = 1.0e-6;
    owned
        .biogeochemistry
        .layers
        .get_mut("soil-1")
        .expect("mineral layer")
        .nitrate_n = 1.0e-6;
    let beginning = owned.clone();
    let available = owned.water.liquid_kg_m2.clone();

    let receipt = run_default_off_diagnostic_at_phase(
        &mut owned,
        &model,
        &configuration,
        &forcing,
        &available,
        None,
    )
    .expect("scarce four-owner diagnostic");

    assert!(receipt.water_used > 0.0);
    assert!(receipt.water_used <= 0.05);
    assert_eq!(receipt.water_partial_authorizations, 2);
    assert!(receipt.nitrogen_used > 0.0);
    assert!(receipt.nitrogen_used <= 2.0e-6);
    assert_eq!(receipt.nitrogen_partial_authorizations, 4);
    assert_eq!(owned.vegetation.strata.len(), 2);
    assert_eq!(owned.vegetation.occupancies.len(), 2);
    assert_eq!(
        owned.water.liquid_kg_m2[&layer].to_bits(),
        (0.05 - receipt.water_used).to_bits()
    );
    assert_eq!(owned.water.last_transaction_id, 1);
    assert_eq!(owned.biogeochemistry.last_transaction_id, 1);

    let mut rejected = beginning.clone();
    let before = serde_json::to_vec(&rejected).expect("scarce beginning bytes");
    let available = rejected.water.liquid_kg_m2.clone();
    let error = run_default_off_diagnostic_at_phase(
        &mut rejected,
        &model,
        &configuration,
        &forcing,
        &available,
        Some(FailurePoint::BeforeCommit),
    )
    .expect_err("scarce rollback injection");
    assert!(matches!(error, DiagnosticError::InjectedFailure));
    assert_eq!(
        serde_json::to_vec(&rejected).expect("scarce after bytes"),
        before
    );
}

#[test]
fn v7_empty_stand_executes_zero_demand_and_corrupted_energy_history_rolls_back() {
    let (mut configuration, mut vegetation) = identity_rebound_v7_fixture();
    configuration.strata.clear();
    configuration.configuration_sha256 = configuration
        .canonical_sha256()
        .expect("empty-stand configuration digest");
    vegetation.strata.clear();
    vegetation.occupancies.clear();
    vegetation
        .configuration_sha256
        .clone_from(&configuration.configuration_sha256);
    vegetation.state_sha256 = vegetation
        .canonical_sha256()
        .expect("empty-stand state digest");
    configuration
        .initial_state_sha256
        .clone_from(&vegetation.state_sha256);
    vegetation
        .validate(&configuration)
        .expect("canonical empty stand");

    let model = load_model_definition().expect("model");
    let forcing = public_water_forcing();
    let mut owned = diagnostic_owned_state(&configuration, vegetation);
    let beginning_water = owned.water.liquid_kg_m2.clone();
    let beginning_mineral = owned.biogeochemistry.layers.clone();
    let available = beginning_water.clone();
    let receipt = run_default_off_diagnostic_at_phase(
        &mut owned,
        &model,
        &configuration,
        &forcing,
        &available,
        None,
    )
    .expect("empty-stand zero-demand transaction");
    assert_eq!(receipt.transaction_id, TransactionId(1));
    assert_eq!(receipt.water_used.to_bits(), 0.0_f64.to_bits());
    assert_eq!(receipt.nitrogen_used.to_bits(), 0.0_f64.to_bits());
    assert!(owned.vegetation.strata.is_empty());
    assert!(owned.vegetation.occupancies.is_empty());
    assert_eq!(owned.water.liquid_kg_m2, beginning_water);
    assert_eq!(owned.biogeochemistry.layers, beginning_mineral);
    assert!(owned.energy.last_operands.is_some());

    let mut corrupted = owned.clone();
    corrupted
        .energy
        .last_operands
        .as_mut()
        .expect("accepted energy operands")
        .identity
        .transaction_id = TransactionId(0);
    let before = serde_json::to_vec(&corrupted).expect("corrupted beginning bytes");
    let available = corrupted.water.liquid_kg_m2.clone();
    assert!(matches!(
        run_default_off_diagnostic_at_phase(
            &mut corrupted,
            &model,
            &configuration,
            &forcing,
            &available,
            None,
        ),
        Err(DiagnosticError::Energy(_))
    ));
    assert_eq!(
        serde_json::to_vec(&corrupted).expect("corrupted ending bytes"),
        before
    );
}

#[test]
fn v7_configuration_state_and_migration_inputs_have_no_default_path() {
    for path in [
        "crates/openwepp-vegetation/src/config.rs",
        "crates/openwepp-vegetation/src/occupancy_state.rs",
        "crates/openwepp-vegetation/src/transaction.rs",
        "crates/openwepp-vegetation/src/migration.rs",
    ] {
        let source = fs::read_to_string(path).expect("V5 source");
        assert!(!source.contains("impl Default for VegetationConfiguration"));
        assert!(!source.contains("impl Default for OccupancyState"));
        assert!(!source.contains("impl Default for CoupledOwnedState"));
        assert!(!source.contains("impl Default for V1CoupledOwnedState"));
        assert!(!source.contains("impl Default for V3CoupledOwnedState"));
        assert!(!source.contains("impl Default for V3VegetationConfiguration"));
    }
}

#[test]
fn production_registry_is_byte_identical_to_v7_authority() {
    let authority = fs::read("docs/work-packages/20260813-c3-woody-storage-transfer-phenology-authority-001/artifacts/openwepp_c3_woody_v7_definition.json")
        .expect("authority definition");
    assert_eq!(MODEL_BYTES, authority);
    assert_eq!(format!("{:x}", Sha256::digest(MODEL_BYTES)), MODEL_SHA256);
    load_model_definition().expect("digest-bound model");
}

#[test]
fn admitted_fvcb_and_medlyn_vectors_match_oracle() {
    assert_fvcb_vector(
        FvcbInput {
            ci_pa: 30.0,
            oi_pa: 20_265.0,
            gamma_pa: 4.275,
            kc_pa: 40.49,
            ko_pa: 27_840.0,
            vcmax: 60.0,
            jmax: 110.0,
            tp: 0.167 * 60.0,
            rd: 1.2,
            par_abs: 0.0,
        },
        "zero_an",
    );
    let saturated = assert_fvcb_vector(
        FvcbInput {
            ci_pa: 30.0,
            oi_pa: 20_265.0,
            gamma_pa: 4.275,
            kc_pa: 40.49,
            ko_pa: 27_840.0,
            vcmax: 70.0,
            jmax: 120.0,
            tp: 0.167 * 70.0,
            rd: 1.2,
            par_abs: 1600.0,
        },
        "saturated_an",
    );
    assert_fvcb_vector(
        FvcbInput {
            ci_pa: 8.0,
            oi_pa: 20_265.0,
            gamma_pa: 4.275,
            kc_pa: 40.49,
            ko_pa: 27_840.0,
            vcmax: 35.0,
            jmax: 160.0,
            tp: 0.167 * 35.0,
            rd: 1.0,
            par_abs: 800.0,
        },
        "rubisco_an",
    );
    assert_fvcb_vector(
        FvcbInput {
            ci_pa: 30.0,
            oi_pa: 20_265.0,
            gamma_pa: 4.275,
            kc_pa: 40.49,
            ko_pa: 27_840.0,
            vcmax: 100.0,
            jmax: 70.0,
            tp: 0.167 * 100.0,
            rd: 1.0,
            par_abs: 45.0,
        },
        "electron_an",
    );
    let gs = medlyn(saturated.an, 25.0, 3.5, 1.4, 39.0, 101_325.0, 1.0).expect("Medlyn");
    let expected = expected();
    assert!(
        (gs - expected["photosynthesis"]["medlyn_gs"]
            .as_f64()
            .expect("Medlyn vector"))
        .abs()
            < 1e-8
    );
    assert!(
        fvcb(FvcbInput {
            vcmax: f64::NAN,
            ..zero_input()
        })
        .is_err()
    );
    assert!(
        fvcb(FvcbInput {
            vcmax: 0.0,
            ..zero_input()
        })
        .is_err()
    );
}

fn zero_input() -> FvcbInput {
    FvcbInput {
        ci_pa: 30.0,
        oi_pa: 20_265.0,
        gamma_pa: 4.275,
        kc_pa: 40.49,
        ko_pa: 27_840.0,
        vcmax: 60.0,
        jmax: 110.0,
        tp: 0.167 * 60.0,
        rd: 1.2,
        par_abs: 0.0,
    }
}

#[test]
fn liquid_interception_matches_fixed_oracle_and_closes() {
    let result = liquid_interception(InterceptionInput {
        store0: 0.2,
        rain: 3.7,
        vapor_amount: 0.42,
        lai: 3.2,
        sai: 0.9,
        alpha_liq: 0.73,
        p_liq: 0.22,
        stemflow_fraction: 0.13,
        leaf_temperature_k: 295.0,
    })
    .expect("liquid interception");
    assert!((result.store1 - 0.482).abs() < 1e-12);
    assert!((result.drainage() - 1.997_516_728_800_741).abs() < 1e-12);
    assert!(result.closure_residual.abs() < 1e-12);
    assert!(
        liquid_interception(InterceptionInput {
            leaf_temperature_k: 268.0,
            ..InterceptionInput {
                store0: 0.2,
                rain: 3.7,
                vapor_amount: 0.42,
                lai: 3.2,
                sai: 0.9,
                alpha_liq: 0.73,
                p_liq: 0.22,
                stemflow_fraction: 0.13,
                leaf_temperature_k: 295.0
            }
        })
        .is_err()
    );
}

#[test]
fn two_stream_rejects_beer_lambert_poison() {
    let expected = expected();
    let result =
        two_stream(3.2, 0.68, 0.1, 0.08, 0.05, 0.14, 620.0, 90.0).expect("two-stream radiation");
    assert!(
        (result.absorbed
            - expected["radiation"]["absorbed"]
                .as_f64()
                .expect("radiation vector"))
        .abs()
            < 1e-7
    );
    let beer = 710.0 * (1.0_f64 - (-0.5_f64 * 3.2).exp());
    assert!((result.absorbed - beer).abs() > 1.0);
    assert!(result.closure_residual.abs() < 1e-9);
    let direct_closure = 620.0
        - result.absorbed_direct
        - result.reflected_direct
        - (1.0 - 0.14) * result.terminal_from_direct;
    let diffuse_closure = 90.0
        - result.absorbed_diffuse
        - result.reflected_diffuse
        - (1.0 - 0.14) * result.terminal_from_diffuse;
    assert!(direct_closure.abs() < 1e-9);
    assert!(diffuse_closure.abs() < 1e-9);
}

#[test]
fn carbon_and_dry_material_remain_distinct() {
    let transfer = material_transfer(
        Tissue::Leaf,
        ReceiverClass::Metabolic,
        0.00432,
        0.000_100_285_714_285_714_27,
        0.48,
    )
    .expect("material");
    assert!((transfer.dry_matter() - 0.009).abs() < 1e-14);
    assert!((transfer.carbon() - transfer.dry_matter()).abs() > 1e-6);
}

fn cn_vector_parameters() -> CnParameters {
    CnParameters {
        growth_respiration_ratio: 0.11,
        a1_froot_leaf: 0.8,
        a2_croot_stem: 0.25,
        a3_stem_leaf: 0.35,
        a4_livewood_fraction: 0.2,
        current_growth_fraction: 0.6,
        cn_leaf: 30.0,
        cn_leaf_litter: 45.0,
        cn_froot: 45.0,
        cn_livewood: 55.0,
        cn_deadwood: 450.0,
        drymatter_carbon_fraction: 0.48,
        xs_recovery_days: 30.0,
        leaf_lifetime_s: 3.0 * 365.0 * 86_400.0,
        froot_lifetime_s: 2.0 * 365.0 * 86_400.0,
        livewood_turnover_s: 5.0 * 365.0 * 86_400.0,
        mortality_rate_s1: 0.01 / (365.0 * 86_400.0),
        leaf_litter_fractions: [0.2, 0.3, 0.5],
        froot_litter_fractions: [0.25, 0.35, 0.4],
    }
}

fn empty_tissues() -> std::collections::BTreeMap<Tissue, TissuePool> {
    [
        Tissue::Leaf,
        Tissue::FineRoot,
        Tissue::LiveStem,
        Tissue::DeadStem,
        Tissue::LiveCoarseRoot,
        Tissue::DeadCoarseRoot,
    ]
    .into_iter()
    .map(|tissue| (tissue, TissuePool::default()))
    .collect()
}

#[test]
fn six_tissue_allocation_and_phenology_match_oracle_vectors() {
    let expected = expected();
    let parameters = cn_vector_parameters();
    let mut tissues = empty_tissues();
    let offer = carbon_offer(0.018, 0.006, -0.030, 0.004, 86_400.0, 30.0).expect("carbon offer");
    let internal_n = 0.00007;
    let demand = nitrogen_demand(offer.offer, internal_n, &parameters).expect("nitrogen demand");
    let external_use = 0.000_274_527_112_063_062_8;
    let growth = finalize_growth(
        &mut tissues,
        &offer,
        GrowthNitrogenReceipt {
            final_total_demand: demand.demand,
            internal_use: internal_n.min(demand.demand),
            external_use,
            internal_remaining: internal_n - internal_n.min(demand.demand),
        },
        &parameters,
    )
    .expect("growth finalization");
    assert!(
        (growth.tissue_carbon[0]
            - expected["carbon_nitrogen"]["leaf_growth"]
                .as_f64()
                .expect("leaf vector"))
        .abs()
            < 1e-14
    );
    assert!(
        (growth.growth_respiration
            - expected["carbon_nitrogen"]["growth_respiration"]
                .as_f64()
                .expect("respiration vector"))
        .abs()
            < 1e-14
    );

    let mut phenology_tissues = empty_tissues();
    phenology_tissues
        .get_mut(&Tissue::Leaf)
        .expect("leaf")
        .transfer = ElementPool {
        carbon: 0.012,
        nitrogen: 0.0004,
    };
    let first = advance_phenology(
        &mut phenology_tissues,
        PhenologyMode::SeasonalDeciduous,
        PhenologyPhase::Dormant,
        3.0 * 86_400.0,
        0.0,
        0.35,
        0.65,
        86_400.0,
        0.60,
        0.30,
        3.0 * 86_400.0,
        3.0 * 86_400.0,
        &parameters,
    )
    .expect("onset day one");
    assert_eq!(first.phase, PhenologyPhase::Onset);
    assert!(
        (phenology_tissues[&Tissue::Leaf].display.carbon
            - expected["phenology"]["onset_day_1_display"]
                .as_f64()
                .expect("onset vector"))
        .abs()
            < 1e-14
    );
    let equality = advance_phenology(
        &mut empty_tissues(),
        PhenologyMode::SeasonalDeciduous,
        PhenologyPhase::Dormant,
        0.0,
        0.0,
        0.35,
        0.60,
        86_400.0,
        0.60,
        0.30,
        3.0 * 86_400.0,
        3.0 * 86_400.0,
        &parameters,
    )
    .expect("threshold equality");
    assert_eq!(equality.phase, PhenologyPhase::Dormant);
}

#[test]
fn schema_and_migration_fail_closed_without_defaults() {
    assert!(VegetationConfiguration::parse_strict(br"{}").is_err());
    assert!(VegetationConfiguration::parse_strict(br#"{"unknown":1}"#).is_err());
    let mut mutated = identity_rebound_v7_configuration();
    mutated.strata[0].stem_rho_vis += 0.01;
    assert!(mutated.validate().is_err());
    let source = RhessysSource {
        source_path: "synthetic.epc".into(),
        raw_bytes: "leaf_cn 28".into(),
        fields: std::collections::BTreeMap::from([("leaf_cn".into(), serde_json::json!(28.0))]),
    };
    let report = migrate_definition_fields(
        &source,
        &std::collections::BTreeMap::new(),
        &["cn_leaf".into(), "p50_leaf_mm".into()],
        &std::collections::BTreeMap::from([("leaf_cn".into(), "cn_leaf".into())]),
    );
    assert_eq!(report.unresolved_required_fields, vec!["p50_leaf_mm"]);
    assert!(report.canonical_configuration_sha256.is_none());
}

#[test]
fn energy_and_aerodynamic_domains_are_explicit() {
    assert!(neutral_resistance(30.0, 12.0, 1.0, 0.2, 2.4).expect("neutral") > 0.0);
    assert!(neutral_resistance(30.0, 12.0, 1.0, 0.2, 0.0).is_err());
    assert!(saturation_specific_humidity(296.0, 101_325.0).expect("qsat") > 0.0);
    assert!(
        energy_residual(100.0, 50.0, 0.0, 50.0)
            .expect("closure")
            .abs()
            < f64::EPSILON
    );
}

#[test]
fn historical_v1_through_v6_cannot_enter_the_v7_public_parsers() {
    let config = identity_rebound_v7_configuration();
    let (_, v7_state) = identity_rebound_v7_fixture();
    let mut v6_config = config.clone();
    v6_config.model_definition_sha256 = V6_MODEL_SHA256.into();
    v6_config.configuration_sha256.clear();
    v6_config.configuration_sha256 = v6_config.canonical_sha256().expect("V6 config digest");
    let mut v6_state = v7_state;
    v6_state.model_definition_sha256 = V6_MODEL_SHA256.into();
    v6_state.configuration_sha256 = v6_config.configuration_sha256.clone();
    v6_state.state_sha256 = v6_state.canonical_sha256().expect("V6 state digest");
    v6_config.initial_state_sha256 = v6_state.state_sha256.clone();
    assert!(
        VegetationConfiguration::parse_strict(
            &serde_json::to_vec(&v6_config).expect("V6 config bytes")
        )
        .is_err()
    );
    assert!(
        CoupledOwnedState::parse_strict(
            &serde_json::to_vec(&v6_state).expect("V6 state bytes"),
            &config
        )
        .is_err()
    );
    for path in [
        "tests/fixtures/c3_woody_v1_diagnostic_state.json",
        "tests/fixtures/c3_woody_v2_diagnostic_state.json",
        "tests/fixtures/c3_woody_v3_diagnostic_state.json",
        "tests/fixtures/c3_woody_v4_diagnostic_state.json",
        "tests/fixtures/c3_woody_v5_diagnostic_state.json",
    ] {
        let result = CoupledOwnedState::parse_strict(
            &fs::read(path).expect("historical state fixture"),
            &config,
        );
        assert!(result.is_err(), "historical state was accepted: {path}");
    }
    for path in [
        "tests/fixtures/c3_woody_v1_diagnostic_configuration.json",
        "tests/fixtures/c3_woody_v2_diagnostic_configuration.json",
        "tests/fixtures/c3_woody_v3_diagnostic_configuration.json",
        "tests/fixtures/c3_woody_v4_diagnostic_configuration.json",
        "tests/fixtures/c3_woody_v5_diagnostic_configuration.json",
    ] {
        assert!(
            VegetationConfiguration::parse_strict(
                &fs::read(path).expect("historical configuration fixture"),
            )
            .is_err(),
            "historical configuration was accepted: {path}"
        );
    }
}

#[test]
#[allow(clippy::float_cmp)] // Committed V5 vectors require exact binary64 reconstruction.
fn v5_committed_cap_vectors_reconstruct_conversions_and_exact_tie_without_python() {
    let expected = expected_v5();
    let family = &expected["families"]["controlled_layer_complementarity"];
    let fraction = family["tile_fraction"].as_f64().expect("tile fraction");
    let dt = family["dt_s"].as_f64().expect("interval");
    for layer in family["layers"].as_array().expect("layer operands") {
        let authorization = layer["authorization_kg_m2_stand_ground"].as_f64().unwrap();
        let cap_rate = authorization / (fraction * dt);
        let q_law = layer["q_law_kg_m2_tile_s"].as_f64().unwrap();
        let q = q_law.min(cap_rate);
        assert_eq!(cap_rate, layer["cap_rate_kg_m2_tile_s"].as_f64().unwrap());
        assert_eq!(q, layer["q_final_kg_m2_tile_s"].as_f64().unwrap());
        assert_eq!(
            fraction * q * dt,
            layer["finalized_use_kg_m2_stand_ground"].as_f64().unwrap()
        );
        assert_eq!(
            layer["branch"],
            if cap_rate <= q_law {
                "authorization_active_or_tie"
            } else {
                "constitutive_law"
            }
        );
    }
    let tie = &expected["families"]["exact_and_near_tie"]["cases"][1];
    assert_eq!(tie["cap_rate_f64_hex"], tie["q_law_f64_hex"]);
    assert_eq!(tie["branch"], "authorization_active_or_tie");
    assert_eq!(tie["dq_final_d_root_potential"].as_f64(), Some(0.0));
}

#[test]
#[allow(clippy::float_cmp)] // The ownership boundary preserves exact admitted amounts.
fn public_water_boundary_preserves_v5_identity_and_one_time_tile_conversion() {
    let expected = expected_v5();
    let family = &expected["families"]["controlled_layer_complementarity"];
    let transaction_id = TransactionId(u128::from(
        family["identity"]["transaction_id"]
            .as_u64()
            .expect("transaction id"),
    ));
    let owner_id =
        ResourceOwnerId::try_new(family["identity"]["owner_id"].as_str().expect("owner id"))
            .expect("typed owner");
    let occupancy_id = OccupancyId {
        stratum_id: StratumId::try_new(
            family["identity"]["stratum_id"]
                .as_str()
                .expect("stratum id"),
        )
        .expect("typed stratum"),
        tile_id: TileId::try_new(family["identity"]["tile_id"].as_str().expect("tile id"))
            .expect("typed tile"),
    };
    let layers = family["layers"].as_array().expect("layer operands");
    let configured = vec![OccupancyRootLayers {
        occupancy_id: occupancy_id.clone(),
        layer_ids: layers
            .iter()
            .map(|layer| SoilLayerId::try_new(layer["layer_id"].as_str().unwrap()).unwrap())
            .collect(),
    }];
    let amounts = layers
        .iter()
        .map(|layer| {
            let key = WaterResourceKey {
                occupancy_id: occupancy_id.clone(),
                layer_id: SoilLayerId::try_new(layer["layer_id"].as_str().unwrap()).unwrap(),
            };
            let authorization = layer["authorization_kg_m2_stand_ground"].as_f64().unwrap();
            (key, authorization.max(1.0))
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let requests = PotentialWaterRequestBatch::try_from_stand_amounts(
        transaction_id,
        owner_id.clone(),
        &configured,
        &amounts,
    )
    .expect("typed V5 request batch");
    let authorizations = requests
        .requests()
        .iter()
        .zip(layers)
        .map(|(request, layer)| MaximumAuthorization {
            transaction_id,
            owner_id: owner_id.clone(),
            key: request.key.clone(),
            amount: layer["authorization_kg_m2_stand_ground"].as_f64().unwrap(),
            basis: request.basis,
        })
        .collect();
    let validated = ValidatedWaterAuthorizations::try_new(&requests, authorizations)
        .expect("exact authorization correspondence");
    let fraction = family["tile_fraction"].as_f64().unwrap();
    let tile_fractions =
        std::collections::BTreeMap::from([(occupancy_id.tile_id.clone(), fraction)]);
    let local = validated
        .to_local_cap_map(&tile_fractions)
        .expect("one stand-to-tile conversion");
    for layer in layers {
        let key = WaterResourceKey {
            occupancy_id: occupancy_id.clone(),
            layer_id: SoilLayerId::try_new(layer["layer_id"].as_str().unwrap()).unwrap(),
        };
        assert_eq!(
            local[&key],
            layer["authorization_kg_m2_tile_ground"].as_f64().unwrap()
        );
    }
}
