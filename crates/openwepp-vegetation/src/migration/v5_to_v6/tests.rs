use openwepp_kernel_contract::{
    MaterialDonorClass, MaterialReceiverClass, ResourceAmountBasis, ResourceOwnerId, TransactionId,
};

use super::*;
use crate::carbon_nitrogen::MaterialTransfer;
use crate::diagnostics::{
    CappedLayerNumericalOperands, CappedNumericalOperands, CappedResidualOperands,
    FixedAuthorizationIdentity, NormalizedResidual, SolveIdentity,
};
use crate::error::NumericalFailureCategory;
use sha2::{Digest, Sha256};

const V5_CONFIGURATION_BYTES: &[u8] =
    include_bytes!("../../../../../tests/fixtures/c3_woody_v5_diagnostic_configuration.json");
const V5_STATE_BYTES: &[u8] =
    include_bytes!("../../../../../tests/fixtures/c3_woody_v5_diagnostic_state.json");
const V6_DEFINITION_BYTES: &str =
    include_str!("../../../model-registry/openwepp_c3_woody_v6_definition.json");
const V6_VECTOR_BYTES: &str = include_str!(
    "../../../../../docs/work-packages/20260813-c3-woody-failure-diagnostic-portability-authority-001/artifacts/openwepp_c3_woody_v6_vectors.json"
);

struct Fixture {
    source_configuration: VegetationConfiguration,
    source_initial_state: CoupledOwnedState,
    source_state: CoupledOwnedState,
    source_diagnostics: IdentityBoundNumericalFailureDiagnostics,
    target_configuration: VegetationConfiguration,
}

#[allow(clippy::too_many_lines)]
fn fixture() -> Fixture {
    let mut source_configuration: VegetationConfiguration =
        serde_json::from_slice(V5_CONFIGURATION_BYTES).expect("historical V5 configuration");
    let mut source_initial_state: CoupledOwnedState =
        serde_json::from_slice(V5_STATE_BYTES).expect("historical V5 initial state");

    // Bind this complete migration fixture to the frozen V6 transition
    // projection instead of merely comparing a Rust-authored DTO with itself.
    source_configuration.topology_tiles[0].fraction = 0.38;
    source_configuration
        .topology_tiles
        .push(crate::config::TopologyTile {
            tile_id: openwepp_kernel_contract::TileId::try_new("tile-2")
                .expect("second transition tile"),
            fraction: 0.62,
        });
    let mut second_root = source_configuration.strata[0].root_layers[0].clone();
    source_configuration.strata[0].root_layers[0].root_fraction = 0.5;
    source_configuration.strata[0].root_layers[0].mineral_n_root_fraction = 0.5;
    second_root.layer_id = openwepp_kernel_contract::SoilLayerId::try_new("soil-2")
        .expect("second transition root layer");
    second_root.root_fraction = 0.5;
    second_root.mineral_n_root_fraction = 0.5;
    source_configuration.strata[0].root_layers.push(second_root);
    source_configuration.configuration_sha256.clear();
    source_configuration.configuration_sha256 = source_configuration
        .canonical_sha256()
        .expect("transition configuration digest");
    source_initial_state
        .configuration_sha256
        .clone_from(&source_configuration.configuration_sha256);
    let initial_lane = source_initial_state
        .occupancies
        .values_mut()
        .next()
        .expect("transition occupancy");
    initial_lane.root_node_potential_mm = -812.5;
    initial_lane.sun_ci_pa = 28.25;
    source_initial_state.state_sha256 = source_initial_state
        .canonical_sha256()
        .expect("transition initial state digest");
    source_configuration
        .initial_state_sha256
        .clone_from(&source_initial_state.state_sha256);

    let mut source_state = source_initial_state.clone();
    source_state.last_transaction_id = 7;
    for shared in source_state.strata.values_mut() {
        shared.last_transaction_id = 7;
    }
    for occupancy in source_state.occupancies.values_mut() {
        occupancy.last_accepted_transaction_id = Some(7);
    }
    let (stratum_id, shared) = source_state
        .strata
        .iter_mut()
        .next()
        .expect("configured stratum");
    shared.pending_transfers.push(MaterialTransfer {
        transaction_id: 7,
        owner_id: ResourceOwnerId::try_new(format!("stratum:{}", stratum_id.as_str()))
            .expect("owner identity"),
        proposal_id: 9,
        donor: MaterialDonorClass::Leaf,
        receiver: MaterialReceiverClass::Metabolic,
        carbon: 0.013,
        nitrogen: 0.0005,
        dry_matter: 0.026,
    });
    source_state.state_sha256 = source_state.canonical_sha256().expect("current V5 digest");
    assert!(valid_historical_state(
        &source_state,
        &source_configuration,
        V5_MODEL_SHA256
    ));

    let occupancy_id = source_state
        .occupancies
        .keys()
        .next()
        .expect("configured occupancy")
        .clone();
    let stratum = source_configuration
        .strata
        .iter()
        .find(|item| item.stratum_id == occupancy_id.stratum_id)
        .expect("occupancy stratum");
    let layers = stratum
        .root_layers
        .iter()
        .map(|root| CappedLayerNumericalOperands {
            layer_id: root.layer_id.clone(),
            cap_rate_kg_m2_tile_s: 1.0e-7,
            q_law_kg_m2_tile_s: 2.0e-7,
            q_final_kg_m2_tile_s: 1.0e-7,
            authorization_active_or_tie: true,
            soil_potential_mm: -1000.0,
            gravity_head_mm: 10.0,
            root_fraction: root.root_fraction,
            z3_m: 0.3,
            ksoil_m2_s: 1.0e-10,
            dxroot_m: 0.1,
            accessible: true,
            frozen: false,
        })
        .collect::<Vec<_>>();
    let active_water_caps = layers.iter().map(|layer| layer.layer_id.clone()).collect();
    let diagnostics = NumericalFailureDiagnostics {
        model_definition_sha256: V5_MODEL_SHA256.into(),
        transaction_id: TransactionId(8),
        occupancy_id: occupancy_id.clone(),
        pass: CoupledSolvePass::Capped,
        solve: SolveIdentity::HydraulicSystem,
        iterations: 7,
        residual_norms: vec![NormalizedResidual {
            identity: "q2_minus_capped_q3_sum".into(),
            value: 2.5,
        }],
        step_norm: Some(3_925.853_296_952_497),
        backtracking_count: 94,
        active_bounds: Vec::new(),
        active_water_caps,
        bracket: None,
        pivot_magnitude: Some(1.0e-9),
        matrix_norm: Some(1.0e-6),
        capped_operands: Some(CappedNumericalOperands {
            water_residual_scale_kg_m2_tile_s: 2.0e-7,
            psi_sunleaf_mm: -4000.0,
            psi_shadeleaf_mm: -3900.0,
            psi_stem_mm: -3500.0,
            psi_root_mm: -3000.0,
            beta_sun: 0.7,
            beta_shade: 0.8,
            emax_sun_kg_m2_s: 2.0e-7,
            emax_shade_kg_m2_s: 1.0e-7,
            gas_sun_kg_m2_s: 1.5e-7,
            gas_shade_kg_m2_s: 0.8e-7,
            q1_sun_kg_m2_s: 1.5e-7,
            q1_shade_kg_m2_s: 0.8e-7,
            q2_kg_m2_s: 2.3e-7,
            residuals: vec![CappedResidualOperands {
                identity: "q2_minus_capped_q3_sum".into(),
                raw_kg_m2_tile_s: 1.0e-12,
                scale_kg_m2_tile_s: 2.0e-7,
                tolerance: 1.0e-12,
                normalized: 1.0,
            }],
            layers,
        }),
        fixed_authorization_identity: Some(FixedAuthorizationIdentity {
            transaction_id: TransactionId(8),
            owner_id: ResourceOwnerId::try_new("migration-water-owner").expect("owner identity"),
            occupancy_id,
            basis: ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
        }),
    };
    let mut source_diagnostics = IdentityBoundNumericalFailureDiagnostics {
        model_definition_sha256: V5_MODEL_SHA256.into(),
        configuration_sha256: source_configuration.configuration_sha256.clone(),
        beginning_state_sha256: source_state.state_sha256.clone(),
        failure_category: NumericalFailureCategory::BacktrackingLimit,
        diagnostics_sha256: String::new(),
        diagnostics,
    };
    source_diagnostics.diagnostics_sha256 = source_diagnostics
        .canonical_sha256()
        .expect("V5 diagnostic digest");

    let mut target_configuration = source_configuration.clone();
    target_configuration.model_definition_sha256 = V6_MODEL_SHA256.into();
    target_configuration.configuration_sha256.clear();
    target_configuration.configuration_sha256 = target_configuration
        .canonical_sha256()
        .expect("V6 configuration digest");
    let migrated_initial =
        rebind_state(&source_initial_state, &target_configuration).expect("V6 initial state");
    target_configuration.initial_state_sha256 = migrated_initial.state_sha256;
    target_configuration
        .validate_historical(V6_MODEL_SHA256)
        .expect("complete V6 configuration");

    Fixture {
        source_configuration,
        source_initial_state,
        source_state,
        source_diagnostics,
        target_configuration,
    }
}

fn source_bytes(fixture: &Fixture) -> Vec<Vec<u8>> {
    [
        serde_json::to_vec(&fixture.source_configuration).expect("configuration bytes"),
        serde_json::to_vec(&fixture.source_initial_state).expect("initial-state bytes"),
        serde_json::to_vec(&fixture.source_state).expect("state bytes"),
        serde_json::to_vec(&fixture.source_diagnostics).expect("diagnostic bytes"),
    ]
    .to_vec()
}

fn assert_diagnostic_payload_rejected(
    fixture: &Fixture,
    diagnostic: &IdentityBoundNumericalFailureDiagnostics,
) {
    assert_eq!(
        migrate_v5_snapshot(
            &fixture.source_configuration,
            &fixture.source_initial_state,
            &fixture.source_state,
            diagnostic,
            &fixture.target_configuration,
        ),
        Err(V5ToV6MigrationError::InvalidV5DiagnosticPayload)
    );
}

fn nonidentity_payload_bytes(
    configuration: &VegetationConfiguration,
    initial_state: &CoupledOwnedState,
    state: &CoupledOwnedState,
    diagnostics: &IdentityBoundNumericalFailureDiagnostics,
) -> Vec<u8> {
    let mut configuration = serde_json::to_value(configuration).expect("configuration value");
    let mut initial_state = serde_json::to_value(initial_state).expect("initial-state value");
    let mut state = serde_json::to_value(state).expect("state value");
    let mut diagnostics = serde_json::to_value(diagnostics).expect("diagnostic value");
    for (value, identity_keys) in [
        (
            &mut configuration,
            &[
                "model_definition_sha256",
                "configuration_sha256",
                "initial_state_sha256",
            ][..],
        ),
        (
            &mut initial_state,
            &[
                "model_definition_sha256",
                "configuration_sha256",
                "state_sha256",
            ][..],
        ),
        (
            &mut state,
            &[
                "model_definition_sha256",
                "configuration_sha256",
                "state_sha256",
            ][..],
        ),
        (
            &mut diagnostics,
            &[
                "model_definition_sha256",
                "configuration_sha256",
                "beginning_state_sha256",
                "diagnostics_sha256",
            ][..],
        ),
    ] {
        let object = value.as_object_mut().expect("object payload");
        for key in identity_keys {
            object.remove(*key);
        }
    }
    diagnostics["diagnostics"]
        .as_object_mut()
        .expect("diagnostic payload")
        .remove("model_definition_sha256");
    serde_json::to_vec(&(configuration, initial_state, state, diagnostics))
        .expect("nonidentity payload bytes")
}

fn authority_transition_projection(
    configuration: &VegetationConfiguration,
    state: &CoupledOwnedState,
    diagnostics: &IdentityBoundNumericalFailureDiagnostics,
) -> serde_json::Value {
    let occupancy = state
        .occupancies
        .values()
        .next()
        .expect("transition occupancy");
    serde_json::json!({
        "configuration": {
            "root_layers": configuration.strata[0]
                .root_layers
                .iter()
                .map(|root| root.layer_id.as_str())
                .collect::<Vec<_>>(),
            "tile_fraction": configuration.topology_tiles[0].fraction,
        },
        "diagnostic": {
            "backtracking_count": diagnostics.diagnostics.backtracking_count,
            "step_norm": diagnostics.diagnostics.step_norm,
        },
        "state": {
            "root_node_potential_mm": occupancy.root_node_potential_mm,
            "sun_ci_pa": occupancy.sun_ci_pa,
        },
    })
}

#[test]
#[allow(clippy::too_many_lines)]
fn exact_transition_consumes_authority_and_rebinds_only_identities() {
    let authority: serde_json::Value =
        serde_json::from_str(V6_DEFINITION_BYTES).expect("released V6 definition");
    assert_eq!(
        authority["identity_transition"]["v5_to_v6"],
        "validate complete V5 identities; compare canonical non-identity scientific configuration/state/diagnostic payload bytes unchanged; bind distinct V6 model/configuration identities; derive distinct V6 configuration/state/diagnostic SHA-256 values"
    );
    let vectors: serde_json::Value =
        serde_json::from_str(V6_VECTOR_BYTES).expect("frozen V6 vectors");
    let transition = &vectors["identity_transition"];
    let authority_payload = serde_json::to_vec(&transition["non_identity_scientific_payload"])
        .expect("authority transition payload");
    let authority_payload_with_newline = [authority_payload, vec![b'\n']].concat();
    let authority_payload_sha256 = format!("{:x}", Sha256::digest(&authority_payload_with_newline));
    assert_eq!(
        authority_payload_sha256,
        "fb79edfe0f03365c10f2af299d1e9b0cb4a5ad3129e705943cae2594bc4fdfef"
    );
    assert_eq!(
        transition["non_identity_payload_bytes_sha256_before"],
        authority_payload_sha256
    );
    assert_eq!(
        transition["non_identity_payload_bytes_sha256_after"],
        authority_payload_sha256
    );
    assert_eq!(
        transition["source"]["diagnostic_sha256"],
        "33cf1cdfdd81625fa9bc8432ba26473cc7efc3135090ad7fd817b61331b8544d"
    );
    assert_eq!(
        transition["target"]["diagnostic_sha256"],
        "657f482cb83a9647a76ddbb353f5880f348aa158df50cbd60078a9df444ba6cd"
    );
    assert_ne!(
        transition["source"]["diagnostic_sha256"],
        transition["target"]["diagnostic_sha256"]
    );

    let fixture = fixture();
    assert_eq!(
        authority_transition_projection(
            &fixture.source_configuration,
            &fixture.source_state,
            &fixture.source_diagnostics,
        ),
        transition["non_identity_scientific_payload"]
    );
    let before = source_bytes(&fixture);
    let before_payload = nonidentity_payload_bytes(
        &fixture.source_configuration,
        &fixture.source_initial_state,
        &fixture.source_state,
        &fixture.source_diagnostics,
    );
    let migrated = migrate_v5_snapshot(
        &fixture.source_configuration,
        &fixture.source_initial_state,
        &fixture.source_state,
        &fixture.source_diagnostics,
        &fixture.target_configuration,
    )
    .expect("exact V5-to-V6 transition");
    assert_eq!(
        authority_transition_projection(
            &migrated.configuration,
            &migrated.state,
            &migrated.diagnostics,
        ),
        transition["non_identity_scientific_payload"]
    );

    assert_eq!(migrated.state.strata, fixture.source_state.strata);
    assert_eq!(migrated.state.occupancies, fixture.source_state.occupancies);
    assert_eq!(migrated.state.last_transaction_id, 7);
    assert_ne!(
        migrated.initial_state.state_sha256,
        fixture.source_initial_state.state_sha256
    );
    assert_eq!(
        migrated.diagnostics.failure_category,
        NumericalFailureCategory::BacktrackingLimit
    );
    let after_payload = nonidentity_payload_bytes(
        &migrated.configuration,
        &migrated.initial_state,
        &migrated.state,
        &migrated.diagnostics,
    );
    assert_eq!(after_payload, before_payload);
    assert_eq!(
        Sha256::digest(&after_payload),
        Sha256::digest(&before_payload)
    );
    assert_ne!(
        migrated.state.state_sha256,
        fixture.source_state.state_sha256
    );
    assert_ne!(
        migrated.diagnostics.diagnostics_sha256,
        fixture.source_diagnostics.diagnostics_sha256
    );

    let mut rebound_diagnostic = migrated.diagnostics;
    rebound_diagnostic.model_definition_sha256 = V5_MODEL_SHA256.into();
    rebound_diagnostic
        .configuration_sha256
        .clone_from(&fixture.source_diagnostics.configuration_sha256);
    rebound_diagnostic
        .beginning_state_sha256
        .clone_from(&fixture.source_diagnostics.beginning_state_sha256);
    rebound_diagnostic
        .diagnostics_sha256
        .clone_from(&fixture.source_diagnostics.diagnostics_sha256);
    rebound_diagnostic.diagnostics.model_definition_sha256 = V5_MODEL_SHA256.into();
    assert_eq!(
        serde_json::to_vec(&rebound_diagnostic).expect("rebound diagnostic bytes"),
        before[3]
    );
    assert_eq!(source_bytes(&fixture), before);
}

#[test]
fn stale_receipts_follow_typed_precedence_and_return_no_candidate() {
    let fixture = fixture();
    let before = source_bytes(&fixture);

    let mut stale_configuration = fixture.source_configuration.clone();
    stale_configuration.model_definition_sha256 = V6_MODEL_SHA256.into();
    stale_configuration.configuration_sha256 = "0".repeat(64);
    assert_eq!(
        migrate_v5_snapshot(
            &stale_configuration,
            &fixture.source_initial_state,
            &fixture.source_state,
            &fixture.source_diagnostics,
            &fixture.target_configuration,
        ),
        Err(V5ToV6MigrationError::InvalidV5ConfigurationIdentity)
    );

    let mut transaction_zero_current = fixture.source_initial_state.clone();
    transaction_zero_current
        .occupancies
        .values_mut()
        .next()
        .expect("occupancy")
        .sun_ci_pa += 0.25;
    transaction_zero_current.state_sha256 = transaction_zero_current
        .canonical_sha256()
        .expect("transaction-zero poison digest");
    let mut transaction_zero_diagnostic = fixture.source_diagnostics.clone();
    transaction_zero_diagnostic
        .beginning_state_sha256
        .clone_from(&transaction_zero_current.state_sha256);
    transaction_zero_diagnostic.diagnostics.transaction_id = TransactionId(1);
    transaction_zero_diagnostic
        .diagnostics
        .fixed_authorization_identity
        .as_mut()
        .expect("fixed identity")
        .transaction_id = TransactionId(1);
    transaction_zero_diagnostic.diagnostics_sha256 = transaction_zero_diagnostic
        .canonical_sha256()
        .expect("transaction-zero diagnostic digest");
    assert_eq!(
        migrate_v5_snapshot(
            &fixture.source_configuration,
            &fixture.source_initial_state,
            &transaction_zero_current,
            &transaction_zero_diagnostic,
            &fixture.target_configuration,
        ),
        Err(V5ToV6MigrationError::InvalidV5StateLineage)
    );

    let mut stale_initial = fixture.source_initial_state.clone();
    stale_initial.state_sha256 = "0".repeat(64);
    assert_eq!(
        migrate_v5_snapshot(
            &fixture.source_configuration,
            &stale_initial,
            &fixture.source_state,
            &fixture.source_diagnostics,
            &fixture.target_configuration,
        ),
        Err(V5ToV6MigrationError::InvalidV5InitialStateDigest)
    );

    let mut stale_state = fixture.source_state.clone();
    stale_state
        .occupancies
        .values_mut()
        .next()
        .expect("occupancy")
        .last_accepted_transaction_id = Some(6);
    stale_state.state_sha256 = stale_state.canonical_sha256().expect("poison digest");
    assert_eq!(
        migrate_v5_snapshot(
            &fixture.source_configuration,
            &fixture.source_initial_state,
            &stale_state,
            &fixture.source_diagnostics,
            &fixture.target_configuration,
        ),
        Err(V5ToV6MigrationError::InvalidV5StateLineage)
    );

    let mut stale_diagnostic = fixture.source_diagnostics.clone();
    stale_diagnostic.diagnostics.transaction_id = TransactionId(7);
    stale_diagnostic.diagnostics_sha256 = stale_diagnostic
        .canonical_sha256()
        .expect("poison diagnostic digest");
    assert_eq!(
        migrate_v5_snapshot(
            &fixture.source_configuration,
            &fixture.source_initial_state,
            &fixture.source_state,
            &stale_diagnostic,
            &fixture.target_configuration,
        ),
        Err(V5ToV6MigrationError::InvalidV5DiagnosticLineage)
    );

    assert_eq!(source_bytes(&fixture), before, "rollback sources changed");
}

#[test]
fn initial_receipt_target_payload_and_diagnostic_digest_fail_closed() {
    let fixture = fixture();

    let mut stale_diagnostic = fixture.source_diagnostics.clone();
    stale_diagnostic.diagnostics_sha256 = "0".repeat(64);
    assert_eq!(
        migrate_v5_snapshot(
            &fixture.source_configuration,
            &fixture.source_initial_state,
            &fixture.source_state,
            &stale_diagnostic,
            &fixture.target_configuration,
        ),
        Err(V5ToV6MigrationError::InvalidV5DiagnosticDigest)
    );

    let mut wrong_failure_category = fixture.source_diagnostics.clone();
    wrong_failure_category.failure_category = NumericalFailureCategory::BracketFailure;
    wrong_failure_category.diagnostics_sha256 = wrong_failure_category
        .canonical_sha256()
        .expect("failure-category poison digest");
    assert_diagnostic_payload_rejected(&fixture, &wrong_failure_category);

    for alias in [
        NumericalFailureCategory::Domain,
        NumericalFailureCategory::IterationLimit,
        NumericalFailureCategory::SingularPivot,
    ] {
        let mut aliased = fixture.source_diagnostics.clone();
        aliased.failure_category = alias;
        aliased.diagnostics_sha256 = aliased
            .canonical_sha256()
            .expect("aliased diagnostic digest");
        assert_diagnostic_payload_rejected(&fixture, &aliased);
    }

    let mut wrong_solve = fixture.source_diagnostics.clone();
    wrong_solve.diagnostics.solve = SolveIdentity::OuterGasEnergyHydraulicCoupling;
    wrong_solve.diagnostics_sha256 = wrong_solve
        .canonical_sha256()
        .expect("wrong-solve diagnostic digest");
    assert_diagnostic_payload_rejected(&fixture, &wrong_solve);

    let mut stale_initial_receipt = fixture.source_configuration.clone();
    stale_initial_receipt.initial_state_sha256 = "a".repeat(64);
    assert_eq!(
        migrate_v5_snapshot(
            &stale_initial_receipt,
            &fixture.source_initial_state,
            &fixture.source_state,
            &fixture.source_diagnostics,
            &fixture.target_configuration,
        ),
        Err(V5ToV6MigrationError::InvalidV5InitialStateLineage)
    );

    let mut wrong_target_receipt = fixture.target_configuration.clone();
    wrong_target_receipt.initial_state_sha256 = "b".repeat(64);
    assert_eq!(
        migrate_v5_snapshot(
            &fixture.source_configuration,
            &fixture.source_initial_state,
            &fixture.source_state,
            &fixture.source_diagnostics,
            &wrong_target_receipt,
        ),
        Err(V5ToV6MigrationError::InvalidV6InitialStateReceipt)
    );

    let mut changed_payload = fixture.target_configuration.clone();
    changed_payload.strata[0].height_m += 1.0;
    changed_payload.configuration_sha256 = changed_payload
        .canonical_sha256()
        .expect("changed target digest");
    assert_eq!(
        migrate_v5_snapshot(
            &fixture.source_configuration,
            &fixture.source_initial_state,
            &fixture.source_state,
            &fixture.source_diagnostics,
            &changed_payload,
        ),
        Err(V5ToV6MigrationError::ConfigurationPayloadMismatch)
    );
}
