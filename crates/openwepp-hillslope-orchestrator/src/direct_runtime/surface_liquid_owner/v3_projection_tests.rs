//! Focused tests for the canonical V3 complete-owner projection.

use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::{
    BeginningLitterPhaseState, ExactDyadicEnthalpy, FinalizedLitterVapor, LitterPhaseConfiguration,
    LitterPhaseReceipt, LitterPhaseTransactionIdentity, LitterPhaseTransactionInput,
    LitterVaporEnvironment, Sha256Digest, SoilThermalLayerSnapshot, SoilThermalOfeSnapshot,
    SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2, SoilThermalSnapshot,
    SoilThermalV2MigrationIdentity, V3PhaseFreeSurfaceEnergyLedger, evaluate_raw_litter_vapor,
    execute_litter_phase_v3, migrate_soil_thermal_v1_to_v2, saturation_specific_humidity,
};

use super::*;
use crate::direct_runtime::surface_liquid_ingress::{
    DirectCanopyLiquidRelease, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectSurfaceLiquidIngressInput, DirectTileGroundIngress,
};
use crate::direct_runtime::surface_liquid_owner::v2_ingress_adapter::{
    execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding,
    prepare_surface_liquid_resource_candidate_v2,
};
use crate::direct_runtime::{
    DirectSurfaceLiquidConfiguration, SurfaceLiquidOwnedStateV2, SurfaceLiquidOwnerClosureRecordV2,
    SurfaceLiquidOwnerModelDefinitionV2,
};

fn digest(byte: char) -> String {
    std::iter::repeat_n(byte, 64).collect()
}

fn typed_digest(byte: char) -> Sha256Digest {
    Sha256Digest::try_new(digest(byte)).expect("test digest")
}

fn configuration_v2() -> SurfaceLiquidConfigurationV2 {
    let base = super::super::tests::configuration();
    let mut litter = base
        .records
        .iter()
        .find(|record| record.key.surface_class == SurfaceClass::ForestLitter)
        .expect("litter record")
        .clone();
    litter.tile_fraction = 1.0;
    let parent = DirectSurfaceLiquidConfiguration::new(
        base.owner_id,
        base.run_id,
        base.ofe_topology,
        base.ofe_bindings,
        vec![litter],
    )
    .expect("one-litter parent configuration");
    let depths = parent
        .records
        .iter()
        .map(|record| (record.key.clone(), 0.03125))
        .collect();
    let model = SurfaceLiquidOwnerModelDefinitionV2::new(digest('1'), digest('2'), digest('3'))
        .expect("V2 model");
    SurfaceLiquidConfigurationV2::new(parent, model, &depths).expect("V2 configuration")
}

fn owner_v2(configuration: &SurfaceLiquidConfigurationV2) -> SurfaceLiquidOwnerEnvelopeV2 {
    let liquid = configuration
        .parent()
        .records
        .iter()
        .map(|record| (record.key.clone(), 0.25))
        .collect();
    let ice = configuration
        .parent()
        .records
        .iter()
        .map(|record| (record.key.clone(), 0.375))
        .collect();
    let enthalpy = configuration
        .parent()
        .records
        .iter()
        .map(|record| (record.key.clone(), 0.0))
        .collect();
    let state = SurfaceLiquidOwnedStateV2::new_initial(configuration, &liquid, &ice, &enthalpy, 3)
        .expect("initial V2 state");
    SurfaceLiquidOwnerEnvelopeV2::wrap_v2(configuration, state).expect("V2 envelope")
}

fn zero_closure(owner: &SurfaceLiquidOwnerEnvelopeV2) -> Vec<SurfaceLiquidOwnerClosureRecordV2> {
    owner
        .v2_state()
        .expect("V2 state")
        .records()
        .iter()
        .map(|record| SurfaceLiquidOwnerClosureRecordV2 {
            key: record.key.clone(),
            liquid_debit_kg_m2_tile: 0.0,
            liquid_credit_kg_m2_tile: 0.0,
            ice_debit_kg_m2_tile: 0.0,
            ice_credit_kg_m2_tile: 0.0,
        })
        .collect()
}

fn amount(mass: f64, interval_s: f64) -> DirectIngressAmount {
    DirectIngressAmount {
        mass_kg_m2_tile_ground: mass,
        temperature_k: 273.15,
        specific_liquid_enthalpy_j_kg: 0.0,
        start_s: 0.0,
        end_s: interval_s,
    }
}

fn ingress_input(
    configuration: &SurfaceLiquidConfigurationV2,
    transaction_id: TransactionId,
    interval_s: f64,
) -> DirectSurfaceLiquidIngressInput {
    let record = &configuration.parent().records[0];
    DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s,
        tile_ingress: vec![DirectTileGroundIngress::CoveredCanopyRelease {
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            surface_id: record.key.surface_id.clone(),
            release: DirectCanopyLiquidRelease {
                throughfall: amount(0.20, interval_s),
                initial_drainage: amount(0.05, interval_s),
                second_drainage: amount(0.025, interval_s),
                stemflow: amount(0.0125, interval_s),
            },
        }],
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: record.key.ofe_id.clone(),
            effective_conductivity_m_s: 1.0e-12,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.2,
        }],
    }
}

fn litter_receipt(
    configuration: &SurfaceLiquidConfigurationV2,
    beginning: &SurfaceLiquidOwnerEnvelopeV2,
    transaction_id: TransactionId,
    support_start_ns: u128,
    support_end_ns: u128,
) -> LitterPhaseReceipt {
    let record = &configuration.parent().records[0];
    let extension = &configuration.records()[0];
    let phase_configuration = LitterPhaseConfiguration {
        litter_depth_m: extension.litter_depth_m.expect("litter depth"),
        dry_heat_capacity_j_m2_k: 3_235.68,
        liquid_capacity_kg_m2_tile: record.capacity_kg_m2_tile,
        ice_capacity_kg_m2_tile: extension
            .litter_ice_capacity_kg_m2_tile
            .expect("ice capacity"),
    };
    let state = BeginningLitterPhaseState {
        liquid_kg_m2_tile: 0.25,
        ice_kg_m2_tile: 0.375,
        sensible_energy_j_m2_tile: 0.0,
        temperature_k: 273.15,
    };
    let humidity = saturation_specific_humidity(273.15, 93_000.0).expect("saturation");
    let environment = LitterVaporEnvironment {
        accepted_phase_free_temperature_k: 273.15,
        air_density_kg_m3: 1.1,
        air_pressure_pa: 93_000.0,
        recipient_specific_humidity_kg_kg: humidity,
        litter_to_canopy_resistance_s_m: 80.0,
    };
    let raw =
        evaluate_raw_litter_vapor(phase_configuration, state, environment).expect("raw vapor");
    assert_eq!(raw.raw_liquid_signed_rate_kg_m2_s.to_bits(), 0);
    assert_eq!(raw.raw_ice_signed_rate_kg_m2_s.to_bits(), 0);
    let envelope_sha =
        Sha256Digest::try_new(beginning.envelope_sha256()).expect("surface-owner digest");
    execute_litter_phase_v3(&LitterPhaseTransactionInput {
        identity: LitterPhaseTransactionIdentity {
            lse_configuration_sha256: typed_digest('4'),
            transaction_id,
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            surface_owner_id: configuration.parent().owner_id.clone(),
            beginning_surface_owner_sha256: envelope_sha.clone(),
            candidate_surface_owner_sha256: envelope_sha,
            support_start_ns,
            support_end_ns,
        },
        configuration: phase_configuration,
        beginning: state,
        vapor_environment: environment,
        finalized_vapor: FinalizedLitterVapor {
            liquid_signed_rate_kg_m2_s: 0.0,
            ice_signed_rate_kg_m2_s: 0.0,
        },
        phase_free_surface_energy: V3PhaseFreeSurfaceEnergyLedger {
            beginning_sensible_energy_j_m2: 0.0,
            ending_sensible_energy_j_m2: 0.0,
            absorbed_shortwave_w_m2: 0.0,
            net_longwave_w_m2: 0.0,
            sensible_to_canopy_air_w_m2: 0.0,
            liquid_vapor_energy_w_m2: 0.0,
            ice_vapor_energy_w_m2: 0.0,
            ground_heat_w_m2: 0.0,
            storage_w_m2: 0.0,
            reconstructed_energy_residual_w_m2: 0.0,
        },
    })
    .expect("accepted litter candidate")
    .receipt
}

fn soil_owner_and_restart(
    configuration: &SurfaceLiquidConfigurationV2,
    transaction_id: TransactionId,
    support_start_ns: u128,
    support_end_ns: u128,
) -> (SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2) {
    let ofe = configuration.parent().ofe_topology[0].clone();
    let layer = configuration.parent().ofe_bindings[0]
        .infiltration_soil_thermal_layer_id
        .clone();
    let snapshot = SoilThermalSnapshot {
        owner_id: ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        configuration_sha256: typed_digest('5'),
        state_sha256: typed_digest('6'),
        snapshot_sha256: typed_digest('7'),
        last_accepted_transaction_id: Some(TransactionId(17)),
        ofes: vec![SoilThermalOfeSnapshot {
            ofe_id: ofe,
            ordered_layers: vec![SoilThermalLayerSnapshot {
                layer_id: layer,
                temperature_k: 273.15,
                enthalpy_j_m2_ofe_ground: 0.0,
            }],
        }],
    };
    let owner = migrate_soil_thermal_v1_to_v2(
        &snapshot,
        SoilThermalV2MigrationIdentity {
            model_version: "OPENWEPP_SOIL_THERMAL_TEST_V2".into(),
            model_definition_sha256: typed_digest('8'),
            run_id: configuration.parent().run_id.to_string(),
            transaction_id,
            support_start_ns,
            support_end_ns,
            receipt_chain_sha256: typed_digest('9'),
        },
    )
    .expect("soil V2 owner");
    let restart = SoilThermalOwnerRestartV2 {
        owner_tag: owner.owner_tag.clone(),
        schema_sha256: owner.schema_sha256.clone(),
        exact_carry_definition_sha256: owner.exact_carry_definition_sha256.clone(),
        parent_v1_state_sha256: owner.parent_v1_state_sha256.clone(),
        owner_state_sha256: owner.state.state_sha256.clone(),
        last_accepted_transaction_id: owner.state.last_accepted_transaction_id,
        receipt_chain_sha256: owner.receipt_chain_sha256.clone(),
        restart_sha256: typed_digest('a'),
    };
    (owner, restart)
}

struct ProjectionFixture {
    configuration: SurfaceLiquidConfigurationV2,
    projection: SurfaceLiquidCompleteOwnerProjectionV3,
    legacy_envelope_bytes: Vec<u8>,
}

fn fixture_with_soil_transaction_id(
    soil_thermal_transaction_id: TransactionId,
) -> ProjectionFixture {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let legacy_envelope_bytes = beginning
        .canonical_bytes(configuration.parent(), Some(&configuration))
        .expect("frozen V2 bytes");
    let transaction_id = TransactionId(703);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction_id,
        &zero_closure(&beginning),
    )
    .expect("resource candidate");
    let ingress = ingress_input(&configuration, transaction_id, 900.0);
    let candidate = execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding(
        &configuration,
        &resource,
        &ingress,
        None,
        false,
        None,
    )
    .expect("accepted child");
    let parent_bytes = candidate
        .parent_working_state()
        .expect("in-progress parent")
        .restart_bytes(&configuration)
        .expect("canonical WB14 parent V2 bytes");
    let parent_wire: serde_json::Value =
        serde_json::from_slice(&parent_bytes).expect("WB14 parent frame");
    let liquid_bytes: Vec<u8> =
        serde_json::from_value(parent_wire["liquid_arithmetic_bytes"].clone())
            .expect("liquid arithmetic bytes");
    let liquid: serde_json::Value =
        serde_json::from_slice(&liquid_bytes).expect("liquid arithmetic frame");
    let parent_start = u128::from(
        liquid["parent_support_start_ns"]
            .as_u64()
            .expect("parent start"),
    );
    let parent_end = u128::from(
        liquid["parent_support_end_ns"]
            .as_u64()
            .expect("parent end"),
    );
    let support_end = u128::from(
        liquid["accepted_until_ns"]
            .as_u64()
            .expect("accepted until"),
    );
    let receipt = litter_receipt(
        &configuration,
        &beginning,
        transaction_id,
        parent_start,
        support_end,
    );
    let (soil_owner, soil_restart) = soil_owner_and_restart(
        &configuration,
        soil_thermal_transaction_id,
        parent_start,
        support_end,
    );
    let identity = SurfaceLiquidCompleteOwnerProjectionIdentityV3 {
        run_id: configuration.parent().run_id,
        transaction_id,
        soil_thermal_run_id: soil_owner.run_id.clone(),
        soil_thermal_transaction_id: soil_owner.transaction_id,
        predecessor_transaction_id: None,
        soil_thermal_predecessor_transaction_id: soil_owner.expected_predecessor_transaction_id,
        parent_support_start_ns: parent_start,
        parent_support_end_ns: parent_end,
        support_start_ns: parent_start,
        support_end_ns: support_end,
        beginning_surface_owner_sha256: beginning.envelope_sha256().into(),
        phase_adjusted_surface_owner_sha256: beginning.envelope_sha256().into(),
        predecessor_receipt_chain_sha256: digest('b'),
        receipt_chain_sha256: ZERO_SHA256.into(),
    };
    let projection = SurfaceLiquidCompleteOwnerProjectionV3::new(
        &configuration,
        identity,
        candidate.ending_owner(),
        &beginning,
        Some(&parent_bytes),
        &[receipt],
        candidate.inner().receipts(),
        &soil_owner,
        &soil_restart,
    )
    .expect("complete-owner projection V3");
    ProjectionFixture {
        configuration,
        projection,
        legacy_envelope_bytes,
    }
}

fn fixture() -> ProjectionFixture {
    fixture_with_soil_transaction_id(TransactionId(703))
}

#[test]
fn canonical_roundtrip_binds_every_exact_frame_and_preserves_v2_bytes() {
    let fixture = fixture();
    let bytes = fixture
        .projection
        .canonical_bytes(&fixture.configuration)
        .expect("canonical projection");
    let replay = SurfaceLiquidCompleteOwnerProjectionV3::from_canonical_bytes(
        &fixture.configuration,
        &bytes,
    )
    .expect("projection replay");
    assert_eq!(replay, fixture.projection);
    assert_eq!(
        replay
            .canonical_bytes(&fixture.configuration)
            .expect("replayed bytes"),
        bytes
    );
    assert_ne!(replay.projection_sha256(), ZERO_SHA256);
    assert_eq!(
        replay.identity().soil_thermal_predecessor_transaction_id,
        Some(TransactionId(17))
    );
    assert_ne!(
        replay.identity().soil_thermal_predecessor_transaction_id,
        replay.identity().predecessor_transaction_id
    );
    let restored = SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
        fixture.configuration.parent(),
        Some(&fixture.configuration),
        &fixture.legacy_envelope_bytes,
    )
    .expect("legacy V2 replay");
    assert_eq!(
        restored
            .canonical_bytes(fixture.configuration.parent(), Some(&fixture.configuration))
            .expect("unchanged V2 bytes"),
        fixture.legacy_envelope_bytes
    );
}

#[test]
fn split_physical_source_and_soil_target_are_bound_without_rebasing() {
    let fixture = fixture_with_soil_transaction_id(TransactionId(704));
    assert_eq!(
        fixture.projection.identity().transaction_id,
        TransactionId(703)
    );
    assert_eq!(
        fixture.projection.identity().soil_thermal_transaction_id,
        TransactionId(704)
    );
    let bytes = fixture
        .projection
        .canonical_bytes(&fixture.configuration)
        .expect("split-authority projection bytes");
    SurfaceLiquidCompleteOwnerProjectionV3::from_canonical_bytes(&fixture.configuration, &bytes)
        .expect("split physical-source/soil-target replay");

    let mut swapped = fixture.projection.clone();
    swapped.identity.soil_thermal_transaction_id = swapped.identity.transaction_id;
    swapped.projection_sha256 = swapped
        .recomputed_sha256()
        .expect("resealed swapped poison");
    assert!(swapped.validate(&fixture.configuration).is_err());

    let mut stale = fixture.projection.clone();
    stale.identity.soil_thermal_transaction_id = TransactionId(702);
    stale.projection_sha256 = stale.recomputed_sha256().expect("resealed stale poison");
    assert!(stale.validate(&fixture.configuration).is_err());

    let mut rebased = fixture.projection.clone();
    rebased.identity.transaction_id = rebased.identity.soil_thermal_transaction_id;
    rebased.projection_sha256 = rebased.recomputed_sha256().expect("resealed rebase poison");
    assert!(rebased.validate(&fixture.configuration).is_err());
}

#[test]
fn pre_soil_target_projection_bytes_fail_closed() {
    let fixture = fixture();
    let bytes = fixture
        .projection
        .canonical_bytes(&fixture.configuration)
        .expect("canonical projection");
    let mut wire: serde_json::Value = serde_json::from_slice(&bytes).expect("projection wire");
    wire.as_object_mut()
        .expect("projection object")
        .remove("soil_thermal_transaction_id");
    let omitted = serde_json::to_vec(&wire).expect("old pre-production wire");
    assert!(
        SurfaceLiquidCompleteOwnerProjectionV3::from_canonical_bytes(
            &fixture.configuration,
            &omitted,
        )
        .is_err()
    );
}

#[test]
fn promoted_owner_support_contains_exact_child_local_candidate_support() {
    let candidate = SurfaceLiquidCandidateOnlyUnpublishedSoilV1 {
        original_prepared_owner_sha256: typed_digest('1').to_string(),
        soil_thermal_run_id: "soil-run".into(),
        predecessor_unpublished_trial_sha256: typed_digest('2').to_string(),
        physical_beginning_state_sha256: typed_digest('3').to_string(),
        soil_thermal_transaction_id: TransactionId(43),
        soil_thermal_predecessor_transaction_id: Some(TransactionId(42)),
        soil_thermal_receipt_chain_sha256: typed_digest('4').to_string(),
        original_support_start_ns: 1_800,
        original_support_end_ns: 1_920,
        child_support_start_ns: 1_860,
        child_support_end_ns: 1_920,
    };
    assert!(publishable_soil_support_matches(
        120,
        1_920,
        1_860,
        1_920,
        Some(&candidate),
    ));
    assert!(!publishable_soil_support_matches(
        1_801,
        1_920,
        1_860,
        1_920,
        Some(&candidate),
    ));
    assert!(!publishable_soil_support_matches(
        120,
        1_921,
        1_860,
        1_920,
        Some(&candidate),
    ));
    assert!(publishable_soil_support_matches(
        1_860, 1_920, 1_860, 1_920, None,
    ));
    assert!(!publishable_soil_support_matches(
        120, 1_920, 1_860, 1_920, None,
    ));
}

#[test]
fn finalized_wb14_projection_requires_exact_parent_end_and_no_open_parent_bytes() {
    let fixture = fixture();
    let mut finalized = fixture.projection.clone();
    finalized.identity.parent_support_end_ns = finalized.identity.support_end_ns;
    finalized.wb14_parent_finalized = true;
    finalized.wb14_parent_working_state_bytes.clear();
    finalized.identity.receipt_chain_sha256 = finalized
        .recomputed_receipt_chain_sha256()
        .expect("finalized chain");
    finalized.projection_sha256 = finalized.recomputed_sha256().expect("finalized projection");
    finalized
        .validate(&fixture.configuration)
        .expect("closed WB14 projection");

    let mut early = finalized.clone();
    early.identity.parent_support_end_ns = early
        .identity
        .support_end_ns
        .checked_add(60_000_000_000)
        .expect("later parent end");
    early.projection_sha256 = early.recomputed_sha256().expect("resealed early poison");
    assert!(early.validate(&fixture.configuration).is_err());

    let mut omitted = finalized;
    omitted.wb14_parent_finalized = false;
    omitted.identity.receipt_chain_sha256 = omitted
        .recomputed_receipt_chain_sha256()
        .expect("resealed omitted chain");
    omitted.projection_sha256 = omitted
        .recomputed_sha256()
        .expect("resealed omitted parent poison");
    assert!(omitted.validate(&fixture.configuration).is_err());
}

#[test]
fn omission_reorder_and_replay_are_rejected_even_if_projection_is_resealed() {
    let fixture = fixture();
    let mut omitted = fixture.projection.clone();
    omitted.litter_phase_receipt_bytes.clear();
    omitted.identity.receipt_chain_sha256 = omitted
        .recomputed_receipt_chain_sha256()
        .expect("resealed chain");
    omitted.projection_sha256 = omitted.recomputed_sha256().expect("resealed projection");
    assert!(omitted.validate(&fixture.configuration).is_err());

    let mut reordered = fixture.projection.clone();
    reordered.current_ingress_receipt_bytes.reverse();
    reordered.projection_sha256 = reordered.recomputed_sha256().expect("resealed projection");
    assert!(reordered.validate(&fixture.configuration).is_err());

    let mut replay = fixture.projection.clone();
    replay
        .current_ingress_receipt_bytes
        .push(replay.current_ingress_receipt_bytes[0].clone());
    replay.identity.receipt_chain_sha256 = replay
        .recomputed_receipt_chain_sha256()
        .expect("resealed chain");
    replay.projection_sha256 = replay.recomputed_sha256().expect("resealed projection");
    assert!(replay.validate(&fixture.configuration).is_err());
}

#[test]
fn mixed_identity_cross_version_and_digest_poisons_fail_closed() {
    let fixture = fixture();
    let mut mixed = fixture.projection.clone();
    mixed.identity.transaction_id = TransactionId(999);
    mixed.projection_sha256 = mixed.recomputed_sha256().expect("resealed projection");
    assert!(mixed.validate(&fixture.configuration).is_err());

    let mut cross_version = fixture.projection.clone();
    let v1 = super::super::tests::state(fixture.configuration.parent());
    let envelope =
        SurfaceLiquidOwnerEnvelopeV2::wrap_v1(fixture.configuration.parent(), v1, digest('3'))
            .expect("V1 envelope variant");
    cross_version.envelope_bytes = envelope
        .canonical_bytes(fixture.configuration.parent(), None)
        .expect("V1 envelope bytes");
    cross_version.envelope_sha256 = envelope.envelope_sha256().into();
    cross_version.identity.receipt_chain_sha256 = cross_version
        .recomputed_receipt_chain_sha256()
        .expect("resealed chain");
    cross_version.projection_sha256 = cross_version
        .recomputed_sha256()
        .expect("resealed projection");
    assert!(cross_version.validate(&fixture.configuration).is_err());

    let mut digest_poison = fixture.projection.clone();
    digest_poison.projection_sha256 = digest('f');
    assert!(digest_poison.validate(&fixture.configuration).is_err());
}

#[test]
fn soil_carry_substitution_and_wb14_ice_donation_are_rejected() {
    let fixture = fixture();
    let mut carry = fixture.projection.clone();
    let SurfaceLiquidV3SoilCustodyV1::Publishable {
        owner_envelope_bytes,
        ..
    } = &mut carry.soil_custody
    else {
        panic!("ordinary fixture must retain publishable soil custody");
    };
    let mut owner: SoilThermalOwnerEnvelopeV2 =
        serde_json::from_slice(owner_envelope_bytes).expect("soil owner frame");
    owner.state.ofes[0].ordered_layers[0].enthalpy_carry =
        ExactDyadicEnthalpy::from_f64(0.25).expect("exact carry");
    *owner_envelope_bytes = serde_json::to_vec(&owner).expect("poisoned soil frame");
    carry.identity.receipt_chain_sha256 = carry
        .recomputed_receipt_chain_sha256()
        .expect("resealed chain");
    carry.projection_sha256 = carry.recomputed_sha256().expect("resealed projection");
    assert!(carry.validate(&fixture.configuration).is_err());

    let mut ice = fixture.projection.clone();
    let mut wb14: serde_json::Value =
        serde_json::from_slice(&ice.wb14_parent_working_state_bytes).expect("WB14 parent frame");
    let candidate_bytes: Vec<u8> = serde_json::from_value(wb14["candidate_owner_bytes"].clone())
        .expect("candidate owner bytes");
    let mut candidate: serde_json::Value =
        serde_json::from_slice(&candidate_bytes).expect("candidate envelope");
    let state_hex = candidate["state_bytes_hex"]
        .as_str()
        .expect("state bytes hex");
    let state_bytes = super::super::v2::decode_hex(state_hex).expect("state bytes");
    let mut state: serde_json::Value =
        serde_json::from_slice(&state_bytes).expect("candidate state");
    state["records"][0]["litter_ice_kg_m2_tile_bits"] =
        serde_json::Value::String(format!("{:016x}", 0.5_f64.to_bits()));
    let poisoned_state = serde_json::to_vec(&state).expect("poisoned state bytes");
    candidate["state_bytes_hex"] =
        serde_json::Value::String(super::super::v2::encode_hex(&poisoned_state));
    wb14["candidate_owner_bytes"] =
        serde_json::to_value(serde_json::to_vec(&candidate).expect("poisoned envelope bytes"))
            .expect("candidate bytes value");
    ice.wb14_parent_working_state_bytes = serde_json::to_vec(&wb14).expect("poisoned WB14 frame");
    ice.identity.receipt_chain_sha256 = ice
        .recomputed_receipt_chain_sha256()
        .expect("resealed chain");
    ice.projection_sha256 = ice.recomputed_sha256().expect("resealed projection");
    assert!(ice.validate(&fixture.configuration).is_err());
}
