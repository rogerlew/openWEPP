use super::*;
use crate::land_surface_energy_shadow::endpoint_fixture;
use openwepp_land_surface_energy::{
    SoilThermalAcceptedEnergyOperandV2, SoilThermalEnergyOperandKindV2,
    SoilThermalV2MigrationIdentity, migrate_soil_thermal_v1_to_v2,
    seal_soil_thermal_receipt_free_owner_v2,
};

fn digest(byte: char) -> Sha256Digest {
    Sha256Digest::try_new(byte.to_string().repeat(64)).expect("digest")
}

fn prepared_fixture() -> (
    openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
    LandSurfaceEnergyConfiguration,
) {
    let fixture = endpoint_fixture();
    let accepted = migrate_soil_thermal_v1_to_v2(
        &fixture.thermal,
        SoilThermalV2MigrationIdentity {
            model_version: fixture
                .lse_configuration
                .soil_thermal_configuration
                .model_version
                .clone(),
            model_definition_sha256: fixture
                .lse_configuration
                .soil_thermal_configuration
                .model_definition_sha256
                .clone(),
            run_id: "direct-v10-native-soil".to_owned(),
            transaction_id: TransactionId(40),
            support_start_ns: 0,
            support_end_ns: 60_000_000_000,
            receipt_chain_sha256: digest('a'),
        },
    )
    .expect("checked V1 to V2 migration");
    let prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
        &accepted,
        TransactionId(41),
        60_000_000_000,
        120_000_000_000,
    )
    .expect("prepared native V2 support");
    (prepared, fixture.lse_configuration)
}

fn exact_operand(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    energy: f64,
) -> SoilThermalAcceptedEnergyOperandV2 {
    SoilThermalAcceptedEnergyOperandV2 {
        ofe_id: beginning.state.ofes[0].ofe_id.clone(),
        layer_id: beginning.state.ofes[0].ordered_layers[0].layer_id.clone(),
        source_kind: SoilThermalEnergyOperandKindV2::SoilInternal,
        source_owner_id: ResourceOwnerId::try_new("native-v2-physical-source")
            .expect("source owner"),
        debit_credit_identity_sha256: digest('b'),
        ordinal: 0,
        units: "J m^-2 OFE-ground".to_owned(),
        basis: "ofe_ground".to_owned(),
        energy_j_m2_ofe_ground: energy,
    }
}

fn native_v2_shadow_for_parent(
    receipt_chain: char,
) -> (
    DirectV10RealConsumerShadow,
    openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
) {
    let (v1_shadow, _) = super::tests::v10_shadow_fixture();
    let current_transaction = TransactionId(v1_shadow.vegetation_state.0.last_transaction_id);
    let support_transaction = TransactionId(current_transaction.0 + 1);
    let migrated = migrate_soil_thermal_v1_to_v2(
        v1_shadow
            .inner
            .soil_thermal
            .v1()
            .expect("V1 fixture resident"),
        SoilThermalV2MigrationIdentity {
            model_version: v1_shadow
                .inner
                .lse_configuration
                .soil_thermal_configuration
                .model_version
                .clone(),
            model_definition_sha256: v1_shadow
                .inner
                .lse_configuration
                .soil_thermal_configuration
                .model_definition_sha256
                .clone(),
            run_id: format!("direct-v10-authoritative-{receipt_chain}"),
            transaction_id: support_transaction,
            support_start_ns: 0,
            support_end_ns: 1_800_000_000_000,
            receipt_chain_sha256: digest(receipt_chain),
        },
    )
    .expect("checked V2 migration");
    let prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
        &migrated,
        support_transaction,
        0,
        60_000_000_000,
    )
    .expect("first prepared V2 support");
    let seals = seal_soil_thermal_receipt_free_owner_v2(
        &openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
            &migrated,
            support_transaction,
            0,
            1_800_000_000_000,
        )
        .expect("receipt-free parent support"),
    )
    .expect("receipt-free seals");
    let parent_prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
        &migrated,
        support_transaction,
        0,
        1_800_000_000_000,
    )
    .expect("parent prepared V2 support");
    let shadow = DirectV10RealConsumerShadow::try_new_v2(
        v1_shadow.vegetation_configuration.clone(),
        v1_shadow.vegetation_state.clone(),
        v1_shadow.inner.vegetation_owner_id.clone(),
        v1_shadow.lse_configuration.clone(),
        v1_shadow.lse_state.clone(),
        v1_shadow.inner.surface_configuration.clone(),
        v1_shadow.inner.layer_maps.clone(),
        parent_prepared,
        seals,
        v1_shadow.inner.biogeochemistry.clone(),
        v1_shadow.inner.hydrology_frame.clone(),
        v1_shadow.inner.next_day_index,
        v1_shadow.gsi_owner_configuration.clone(),
        v1_shadow.gsi_state.clone(),
        v1_shadow.provider_static_configuration.clone(),
        v1_shadow.provider_cursor.clone(),
        v1_shadow.root_zone_hydraulic_configuration.clone(),
    )
    .expect("native V2 authoritative host");
    (shadow, prepared)
}

fn accepted_bundle(
    prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
    configuration: &LandSurfaceEnergyConfiguration,
    energy: f64,
    identity: char,
) -> (
    SoilThermalAcceptedCandidateV2,
    SoilThermalOrchestratorSealsV2,
) {
    let beginning = prepared.beginning_owner();
    let mut operand = exact_operand(beginning, energy);
    operand.debit_credit_identity_sha256 = digest(identity);
    let expected =
        SoilThermalExpectedAcceptedOperandSetV2::try_new(beginning, configuration, vec![operand])
            .expect("expected accepted operands");
    let accepted = aggregate_soil_thermal_ending_v2(beginning, configuration, &expected)
        .expect("accepted exact-carry candidate");
    let seals = seal_soil_thermal_accepted_candidate_v2(beginning, &accepted)
        .expect("accepted exact-carry seals");
    (accepted, seals)
}

fn align_complete_owner_transaction(
    candidate: &mut DirectV10RealConsumerShadow,
    transaction_id: TransactionId,
) {
    candidate.vegetation_state.0.last_transaction_id = transaction_id.0;
    candidate.lse_state.0.last_accepted_transaction_id = Some(transaction_id);
    candidate.inner.biogeochemistry.last_transaction_id = transaction_id.0;
}

#[test]
fn authoritative_beginning_installs_once_and_reuses_exact_ending_without_second_install() {
    let (authoritative, prepared) = native_v2_shadow_for_parent('4');
    let transaction = prepared.beginning_owner().transaction_id;
    let (accepted, seals) = accepted_bundle(
        &prepared,
        &authoritative.inner.lse_configuration,
        f64::from_bits(1),
        '5',
    );
    let mut candidate = authoritative.clone();
    align_complete_owner_transaction(&mut candidate, transaction);
    let before_soil = candidate
        .inner
        .soil_thermal
        .canonical_active_owner_bytes()
        .expect("candidate beginning soil bytes");
    let before_vegetation = candidate.vegetation_state.clone();
    let before_lse = candidate.lse_state.clone();
    let before_bgc = candidate.inner.biogeochemistry.clone();
    candidate
        .install_soil_thermal_accepted_v2_from_beginning(
            &authoritative,
            prepared.beginning_owner(),
            accepted.clone(),
            seals.clone(),
        )
        .expect("single authoritative install");
    let once = candidate
        .inner
        .soil_thermal
        .canonical_active_owner_bytes()
        .expect("candidate accepted soil bytes");
    assert_ne!(before_soil, once);
    assert_eq!(candidate.vegetation_state, before_vegetation);
    assert_eq!(candidate.lse_state, before_lse);
    assert_eq!(candidate.inner.biogeochemistry, before_bgc);
    assert!(
        candidate
            .soil_thermal_v2()
            .expect("accepted V2 resident")
            .owner()
            .state
            .ofes
            .iter()
            .flat_map(|ofe| &ofe.ordered_layers)
            .any(|layer| layer.enthalpy_carry.sign != 0),
        "first support must retain nonzero exact carry"
    );

    candidate
        .install_soil_thermal_accepted_v2_from_beginning(
            &authoritative,
            prepared.beginning_owner(),
            accepted,
            seals,
        )
        .expect("exact reused ending is an authenticated no-op");
    assert_eq!(
        candidate
            .inner
            .soil_thermal
            .canonical_active_owner_bytes()
            .expect("reused candidate soil bytes"),
        once,
        "reused exact ending must not perform a second install"
    );
}

#[test]
fn authoritative_successor_support_retains_nonzero_carry_and_one_install_custody() {
    let (authoritative, first_prepared) = native_v2_shadow_for_parent('6');
    let first_transaction = first_prepared.beginning_owner().transaction_id;
    let (first_accepted, first_seals) = accepted_bundle(
        &first_prepared,
        &authoritative.inner.lse_configuration,
        f64::from_bits(1),
        '7',
    );
    let mut first_ending = authoritative.clone();
    align_complete_owner_transaction(&mut first_ending, first_transaction);
    first_ending
        .install_soil_thermal_accepted_v2_from_beginning(
            &authoritative,
            first_prepared.beginning_owner(),
            first_accepted,
            first_seals,
        )
        .expect("first accepted support");

    let successor_transaction = TransactionId(first_transaction.0 + 1);
    let successor_prepared = first_ending
        .prepare_soil_thermal_support_v2(successor_transaction, 60_000_000_000, 120_000_000_000)
        .expect("successor prepared from installed resident");
    assert!(
        successor_prepared
            .beginning_owner()
            .state
            .ofes
            .iter()
            .flat_map(|ofe| &ofe.ordered_layers)
            .any(|layer| layer.enthalpy_carry.sign != 0),
        "successor beginning must retain first-support carry"
    );
    let (successor_accepted, successor_seals) = accepted_bundle(
        &successor_prepared,
        &first_ending.inner.lse_configuration,
        f64::from_bits(1),
        '8',
    );
    let mut successor_ending = first_ending.clone();
    align_complete_owner_transaction(&mut successor_ending, successor_transaction);
    successor_ending
        .install_soil_thermal_accepted_v2_from_beginning(
            &first_ending,
            successor_prepared.beginning_owner(),
            successor_accepted,
            successor_seals,
        )
        .expect("successor authoritative install");
    assert_eq!(
        successor_ending
            .soil_thermal_v2()
            .expect("successor resident")
            .owner()
            .state
            .last_accepted_transaction_id,
        Some(successor_transaction)
    );
}

#[test]
fn authoritative_install_refuses_stale_and_substituted_beginnings_with_byte_rollback() {
    let (authoritative, prepared) = native_v2_shadow_for_parent('9');
    let transaction = prepared.beginning_owner().transaction_id;
    let (accepted, seals) = accepted_bundle(
        &prepared,
        &authoritative.inner.lse_configuration,
        f64::from_bits(1),
        'a',
    );
    let candidate = || {
        let mut value = authoritative.clone();
        align_complete_owner_transaction(&mut value, transaction);
        value
    };
    let assert_refused = |mut value: DirectV10RealConsumerShadow,
                          source: &DirectV10RealConsumerShadow,
                          beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
                          accepted: SoilThermalAcceptedCandidateV2,
                          seals: SoilThermalOrchestratorSealsV2| {
        let before = value
            .inner
            .soil_thermal
            .canonical_active_owner_bytes()
            .expect("before refusal soil bytes");
        assert!(
            value
                .install_soil_thermal_accepted_v2_from_beginning(
                    source, beginning, accepted, seals,
                )
                .is_err()
        );
        assert_eq!(
            value
                .inner
                .soil_thermal
                .canonical_active_owner_bytes()
                .expect("after refusal soil bytes"),
            before
        );
    };

    let mut stale_state = prepared.beginning_owner().clone();
    stale_state.state.state_sha256 = digest('b');
    assert_refused(
        candidate(),
        &authoritative,
        &stale_state,
        accepted.clone(),
        seals.clone(),
    );

    let mut stale_transaction = prepared.beginning_owner().clone();
    stale_transaction.transaction_id = TransactionId(transaction.0 + 1);
    assert_refused(
        candidate(),
        &authoritative,
        &stale_transaction,
        accepted.clone(),
        seals.clone(),
    );

    let mut stale_support = prepared.beginning_owner().clone();
    stale_support.support_start_ns = 1_800_000_000_000;
    stale_support.support_end_ns = 1_860_000_000_000;
    assert_refused(
        candidate(),
        &authoritative,
        &stale_support,
        accepted.clone(),
        seals.clone(),
    );

    let mut stale_chain = prepared.beginning_owner().clone();
    stale_chain.receipt_chain_sha256 = digest('c');
    assert_refused(
        candidate(),
        &authoritative,
        &stale_chain,
        accepted.clone(),
        seals.clone(),
    );

    let mut stale_carry = prepared.beginning_owner().clone();
    stale_carry.state.ofes[0].ordered_layers[0].enthalpy_carry =
        openwepp_land_surface_energy::ExactDyadicEnthalpy::try_new(1, "1", -100)
            .expect("canonical nonzero carry");
    stale_carry.state.reseal().expect("reseal carry poison");
    assert_refused(
        candidate(),
        &authoritative,
        &stale_carry,
        accepted.clone(),
        seals.clone(),
    );

    let (wrong_authority, _) = native_v2_shadow_for_parent('d');
    assert_refused(
        candidate(),
        &wrong_authority,
        prepared.beginning_owner(),
        accepted.clone(),
        seals.clone(),
    );

    let mut substituted_target = wrong_authority.clone();
    align_complete_owner_transaction(&mut substituted_target, transaction);
    assert_refused(
        substituted_target,
        &authoritative,
        prepared.beginning_owner(),
        accepted,
        seals,
    );
}

#[test]
fn receipt_free_resident_has_exactly_one_v2_owner_and_no_v1_projection() {
    let (prepared, _) = prepared_fixture();
    let seals = seal_soil_thermal_receipt_free_owner_v2(&prepared).expect("receipt-free seals");
    let resident =
        DirectSoilThermalResident::try_new_v2(prepared.clone(), seals).expect("native V2 resident");

    assert!(resident.v1().is_err());
    assert_eq!(
        resident.v2().expect("V2").owner(),
        prepared.beginning_owner()
    );
    assert!(resident.v2().expect("V2").latest_accepted().is_none());
    assert!(resident.v2().expect("V2").receipt_free_seals().is_some());
    let bytes = resident
        .canonical_active_owner_bytes()
        .expect("canonical V2");
    let text = String::from_utf8(bytes).expect("JSON");
    assert!(text.contains("OPENWEPP_DIRECT_V10_SOIL_THERMAL_RESIDENT_V2"));
    assert!(text.contains("OPENWEPP_SOIL_THERMAL_OWNER_V2"));
    assert!(!text.contains("snapshot_sha256"));
}

#[test]
fn accepted_exact_carry_is_joined_into_the_next_support_read_view() {
    let (prepared, configuration) = prepared_fixture();
    let beginning = prepared.beginning_owner();
    let resident = DirectSoilThermalResident::try_new_v2(
        prepared.clone(),
        seal_soil_thermal_receipt_free_owner_v2(&prepared).expect("receipt-free seals"),
    )
    .expect("native V2 resident");
    let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        beginning,
        &configuration,
        vec![exact_operand(beginning, f64::from_bits(1))],
    )
    .expect("expected operands");
    let accepted = aggregate_soil_thermal_ending_v2(beginning, &configuration, &expected)
        .expect("exact accepted candidate");
    let accepted_seals =
        seal_soil_thermal_accepted_candidate_v2(beginning, &accepted).expect("accepted seals");
    let accepted_resident = resident
        .v2()
        .expect("V2")
        .accepted(beginning, accepted, accepted_seals)
        .expect("accepted native resident");
    let next = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
        accepted_resident.owner(),
        TransactionId(42),
        120_000_000_000,
        180_000_000_000,
    )
    .expect("next support");
    let ofe = &next.beginning_owner().state.ofes[0];
    let layer = &ofe.ordered_layers[0];
    let exact = next
        .physical_read_view()
        .exact_layer_enthalpy(&ofe.ofe_id, &layer.layer_id)
        .expect("exact physical read");

    assert_ne!(layer.enthalpy_carry.sign, 0);
    assert_eq!(
        exact,
        openwepp_land_surface_energy::ExactDyadicEnthalpy::exact_sum([
            &openwepp_land_surface_energy::ExactDyadicEnthalpy::from_f64(
                layer.enthalpy_hi_j_m2_ofe_ground,
            )
            .expect("high term"),
            &layer.enthalpy_carry,
        ])
        .expect("exact sum")
    );
}

#[test]
fn acceptance_poison_preserves_resident_bytes() {
    let (prepared, configuration) = prepared_fixture();
    let beginning = prepared.beginning_owner();
    let resident = DirectSoilThermalResident::try_new_v2(
        prepared.clone(),
        seal_soil_thermal_receipt_free_owner_v2(&prepared).expect("receipt-free seals"),
    )
    .expect("native V2 resident");
    let before = resident
        .canonical_active_owner_bytes()
        .expect("before bytes");
    let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        beginning,
        &configuration,
        vec![exact_operand(beginning, 0.25)],
    )
    .expect("expected operands");
    let accepted = aggregate_soil_thermal_ending_v2(beginning, &configuration, &expected)
        .expect("accepted candidate");
    let mut poison =
        seal_soil_thermal_accepted_candidate_v2(beginning, &accepted).expect("accepted seals");
    poison.expected_operand_set_sha256 = digest('f');

    assert!(
        resident
            .v2()
            .expect("V2")
            .accepted(beginning, accepted, poison)
            .is_err()
    );
    assert_eq!(
        resident
            .canonical_active_owner_bytes()
            .expect("rollback bytes"),
        before
    );
}

#[test]
fn direct_v10_try_new_v2_is_single_resident_and_poisoned_install_is_atomic() {
    let (v1_shadow, _) = super::tests::v10_shadow_fixture();
    let current_transaction = TransactionId(v1_shadow.vegetation_state.0.last_transaction_id);
    let support_transaction = TransactionId(current_transaction.0 + 1);
    let migrated = migrate_soil_thermal_v1_to_v2(
        v1_shadow
            .inner
            .soil_thermal
            .v1()
            .expect("V1 fixture resident"),
        SoilThermalV2MigrationIdentity {
            model_version: v1_shadow
                .inner
                .lse_configuration
                .soil_thermal_configuration
                .model_version
                .clone(),
            model_definition_sha256: v1_shadow
                .inner
                .lse_configuration
                .soil_thermal_configuration
                .model_definition_sha256
                .clone(),
            run_id: "direct-v10-single-resident".to_owned(),
            transaction_id: current_transaction,
            support_start_ns: 0,
            support_end_ns: 60_000_000_000,
            receipt_chain_sha256: digest('c'),
        },
    )
    .expect("checked V2 migration");
    let prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
        &migrated,
        support_transaction,
        60_000_000_000,
        120_000_000_000,
    )
    .expect("prepared V2 support");
    let receipt_free_seals =
        seal_soil_thermal_receipt_free_owner_v2(&prepared).expect("receipt-free seals");
    let mut v2_shadow = DirectV10RealConsumerShadow::try_new_v2(
        v1_shadow.vegetation_configuration.clone(),
        v1_shadow.vegetation_state.clone(),
        v1_shadow.inner.vegetation_owner_id.clone(),
        v1_shadow.lse_configuration.clone(),
        v1_shadow.lse_state.clone(),
        v1_shadow.inner.surface_configuration.clone(),
        v1_shadow.inner.layer_maps.clone(),
        prepared.clone(),
        receipt_free_seals,
        v1_shadow.inner.biogeochemistry.clone(),
        v1_shadow.inner.hydrology_frame.clone(),
        v1_shadow.inner.next_day_index,
        v1_shadow.gsi_owner_configuration.clone(),
        v1_shadow.gsi_state.clone(),
        v1_shadow.provider_static_configuration.clone(),
        v1_shadow.provider_cursor.clone(),
        v1_shadow.root_zone_hydraulic_configuration.clone(),
    )
    .expect("DirectV10 native V2 constructor");
    assert!(v2_shadow.inner.soil_thermal.v1().is_err());
    assert!(v2_shadow.soil_thermal_v2().is_ok());
    let owner_bytes = v2_shadow
        .canonical_owner_state_bytes()
        .expect("complete canonical owner bytes");
    let soil_bytes = owner_bytes.get("soil_thermal").expect("soil owner bytes");
    let soil_text = std::str::from_utf8(soil_bytes).expect("soil JSON");
    assert!(soil_text.contains("OPENWEPP_DIRECT_V10_SOIL_THERMAL_RESIDENT_V2"));
    assert!(!soil_text.contains("snapshot_sha256"));
    let vegetation_envelope = v11_soil_thermal_owner_envelope(&v2_shadow.inner.soil_thermal)
        .expect("V11 native V2 soil owner envelope");
    assert_eq!(
        vegetation_envelope.state_bytes, *soil_bytes,
        "V11 support custody must carry the canonical active V2 owner bytes"
    );

    let beginning = prepared.beginning_owner();
    let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        beginning,
        &v2_shadow.inner.lse_configuration,
        vec![exact_operand(beginning, 0.25)],
    )
    .expect("expected operands");
    let accepted =
        aggregate_soil_thermal_ending_v2(beginning, &v2_shadow.inner.lse_configuration, &expected)
            .expect("accepted candidate");
    let mut poison =
        seal_soil_thermal_accepted_candidate_v2(beginning, &accepted).expect("accepted seals");
    poison.orchestrator_seal_sha256 = digest('f');
    let before = v2_shadow
        .canonical_owner_state_bytes()
        .expect("beginning owner bytes");
    assert!(
        v2_shadow
            .install_soil_thermal_accepted_v2(beginning, accepted, poison)
            .is_err()
    );
    assert_eq!(
        v2_shadow
            .canonical_owner_state_bytes()
            .expect("rollback owner bytes"),
        before
    );
}
