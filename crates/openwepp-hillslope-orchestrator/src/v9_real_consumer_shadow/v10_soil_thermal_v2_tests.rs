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

pub(super) fn native_v2_shadow_for_parent(
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

pub(super) fn accepted_bundle(
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

pub(super) fn align_complete_owner_transaction(
    candidate: &mut DirectV10RealConsumerShadow,
    transaction_id: TransactionId,
) {
    candidate.vegetation_state.0.last_transaction_id = transaction_id.0;
    candidate.lse_state.0.last_accepted_transaction_id = Some(transaction_id);
    candidate.inner.biogeochemistry.last_transaction_id = transaction_id.0;
}

fn v47_atomic_posture_fixture(
    receipt_chain: char,
) -> (
    DirectV10RealConsumerShadow,
    openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
    DirectV10SoilThermalResidentV2,
    SoilThermalAcceptedCandidateV2,
    SoilThermalOrchestratorSealsV2,
    TransactionId,
    TransactionId,
) {
    let (authoritative, first_prepared) = native_v2_shadow_for_parent(receipt_chain);
    let (first_accepted, first_seals) = accepted_bundle(
        &first_prepared,
        &authoritative.inner.lse_configuration,
        f64::from_bits(1),
        receipt_chain,
    );
    let first_target = first_accepted.ending_owner.transaction_id;
    let mut candidate = authoritative.clone();
    align_complete_owner_transaction(&mut candidate, first_target);
    candidate
        .install_soil_thermal_accepted_v2_from_beginning(
            &authoritative,
            first_prepared.beginning_owner(),
            first_accepted,
            first_seals,
        )
        .expect("installed first accepted soil child");
    let prepared = candidate
        .prepare_next_soil_thermal_support_v2(60_000_000_000, 1_800_000_000_000)
        .expect("prepared authenticated soil successor");
    let (accepted, seals) = accepted_bundle(
        &prepared,
        &candidate.inner.lse_configuration,
        f64::from_bits(2),
        receipt_chain,
    );
    let target = accepted.ending_owner.transaction_id;
    let source = accepted
        .ending_owner
        .expected_predecessor_transaction_id
        .expect("accepted successor predecessor");
    let resident = candidate
        .inner
        .soil_thermal
        .v2()
        .expect("V2 resident")
        .accepted(prepared.beginning_owner(), accepted.clone(), seals.clone())
        .expect("accepted resident fixture");
    (
        candidate, prepared, resident, accepted, seals, source, target,
    )
}

fn v48_r122_fixed_point_finalizer_fixture(
    receipt_chain: char,
) -> (
    DirectV10RealConsumerShadow,
    openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
    DirectV10SoilThermalResidentV2,
    SoilThermalAcceptedCandidateV2,
    SoilThermalOrchestratorSealsV2,
) {
    let (mut candidate, first_prepared, _, first_accepted, first_seals, _, source) =
        v47_atomic_posture_fixture(receipt_chain);
    assert_eq!(source, TransactionId(42));
    let authoritative = candidate.clone();
    let first_authority = candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v2(
            &authoritative,
            first_prepared.beginning_owner(),
        )
        .expect("R122 authenticated source-42 parent authority");
    candidate
        .install_soil_thermal_accepted_v2_from_authenticated_beginning(
            &authoritative,
            first_prepared.beginning_owner(),
            first_authority,
            first_accepted,
            first_seals,
        )
        .expect("R122 installed source-42 accepted parent");
    align_complete_owner_transaction(&mut candidate, source);
    let prepared = candidate
        .prepare_next_soil_thermal_support_v2(1_800_000_000_000, 1_980_000_000_000)
        .expect("R122 prepared exact composed final support");
    assert_eq!(prepared.beginning_owner().transaction_id, TransactionId(43));
    assert_eq!(
        prepared
            .beginning_owner()
            .expected_predecessor_transaction_id,
        Some(TransactionId(42)),
    );
    assert_eq!(
        prepared.beginning_owner().support_start_ns,
        1_800_000_000_000
    );
    assert_eq!(prepared.beginning_owner().support_end_ns, 1_980_000_000_000);
    let (accepted, seals) = accepted_bundle(
        &prepared,
        &candidate.inner.lse_configuration,
        f64::from_bits(2),
        receipt_chain,
    );
    let resident = candidate
        .soil_thermal_v2()
        .expect("R122 source resident")
        .accepted(prepared.beginning_owner(), accepted.clone(), seals.clone())
        .expect("R122 accepted target resident");
    (candidate, prepared, resident, accepted, seals)
}

#[test]
fn v47_atomic_transaction_posture_accepts_same_source_and_soil_target() {
    let (mut candidate, prepared, resident, accepted, seals, _, target) =
        v47_atomic_posture_fixture('1');
    align_complete_owner_transaction(&mut candidate, target);
    let posture = direct_soil_thermal_atomic_complete_owner_transaction_posture_v2(
        &candidate, &resident, None,
    )
    .expect("same source/soil target posture");
    assert_eq!(posture.source_transaction_id, target);
    assert_eq!(posture.soil_target_transaction_id, target);
    assert_eq!(
        posture.kind,
        DirectSoilThermalAtomicCompleteOwnerTransactionKindV2::SameSourceAndSoilTarget,
    );
    candidate
        .install_soil_thermal_accepted_v2(prepared.beginning_owner(), accepted, seals)
        .expect("ordinary same-source/same-target accepted install");
    assert_eq!(
        candidate
            .soil_thermal_v2()
            .expect("installed same-ID V2 resident"),
        &resident,
    );
}

#[test]
fn v47_atomic_transaction_posture_accepts_exact_authenticated_soil_successor() {
    let (mut candidate, _, resident, _, _, source, target) = v47_atomic_posture_fixture('2');
    assert_ne!(source, target);
    align_complete_owner_transaction(&mut candidate, source);
    let authority =
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            source, target,
        )
        .expect("native split authority");
    let posture = direct_soil_thermal_atomic_complete_owner_transaction_posture_v2(
        &candidate,
        &resident,
        Some(DirectSoilThermalAtomicInstallAuthorityV2::Physical(
            authority,
        )),
    )
    .expect("authenticated soil successor posture");
    assert_eq!(posture.source_transaction_id, source);
    assert_eq!(posture.soil_target_transaction_id, target);
    assert_eq!(
        posture.soil_expected_predecessor_transaction_id,
        Some(source),
    );
    assert_eq!(
        posture.kind,
        DirectSoilThermalAtomicCompleteOwnerTransactionKindV2::AuthenticatedSoilSuccessor,
    );
}

#[test]
fn v47_atomic_transaction_posture_refuses_foreign_swapped_or_missing_identity() {
    let (mut candidate, _, resident, _, _, source, target) = v47_atomic_posture_fixture('3');
    align_complete_owner_transaction(&mut candidate, source);
    assert!(
        direct_soil_thermal_atomic_complete_owner_transaction_posture_v2(
            &candidate, &resident, None,
        )
        .is_err(),
        "split custody requires explicit authority",
    );
    for authority in [
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            TransactionId(source.0 + 9),
            target,
        )
        .expect("foreign source authority"),
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            target, source,
        )
        .expect("swapped authority"),
    ] {
        assert!(
            direct_soil_thermal_atomic_complete_owner_transaction_posture_v2(
                &candidate,
                &resident,
                Some(DirectSoilThermalAtomicInstallAuthorityV2::Physical(
                    authority,
                )),
            )
            .is_err(),
        );
    }
    let mut missing_predecessor = resident;
    missing_predecessor
        .owner
        .expected_predecessor_transaction_id = None;
    assert!(
        direct_soil_thermal_atomic_complete_owner_transaction_posture_v2(
            &candidate,
            &missing_predecessor,
            Some(DirectSoilThermalAtomicInstallAuthorityV2::Physical(
                crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::
                    try_new(source, target)
                    .expect("exact split authority"),
            )),
        )
        .is_err(),
    );
}

#[test]
fn v47_atomic_transaction_posture_refuses_source_owner_disagreement() {
    let (mut candidate, _, resident, _, _, source, target) = v47_atomic_posture_fixture('4');
    align_complete_owner_transaction(&mut candidate, source);
    let authority =
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            source, target,
        )
        .expect("exact split authority");
    for poison in 0..3 {
        let mut changed = candidate.clone();
        match poison {
            0 => changed.vegetation_state.0.last_transaction_id = source.0 + 1,
            1 => {
                changed.lse_state.0.last_accepted_transaction_id = Some(TransactionId(source.0 + 1))
            }
            _ => changed.inner.biogeochemistry.last_transaction_id = source.0 + 1,
        }
        assert!(
            direct_soil_thermal_atomic_complete_owner_transaction_posture_v2(
                &changed,
                &resident,
                Some(DirectSoilThermalAtomicInstallAuthorityV2::Physical(
                    authority,
                )),
            )
            .is_err(),
        );
    }
}

#[test]
fn v47_atomic_install_rolls_back_and_never_publishes_on_refusal() {
    let (mut candidate, prepared, _, accepted, seals, source, target) =
        v47_atomic_posture_fixture('5');
    assert_ne!(source, target);
    align_complete_owner_transaction(&mut candidate, source);
    let before = candidate
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before generic split-install refusal");
    let source_before = (
        candidate.vegetation_state.0.last_transaction_id,
        candidate.lse_state.0.last_accepted_transaction_id,
        candidate.inner.biogeochemistry.last_transaction_id,
    );
    assert!(
        candidate
            .install_soil_thermal_accepted_v2(prepared.beginning_owner(), accepted, seals)
            .is_err(),
        "ordinary/public install must never admit split custody",
    );
    assert_eq!(
        candidate
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after generic split-install refusal"),
        before,
        "failed atomic install must retain native-V2 owner bytes",
    );
    assert_eq!(
        (
            candidate.vegetation_state.0.last_transaction_id,
            candidate.lse_state.0.last_accepted_transaction_id,
            candidate.inner.biogeochemistry.last_transaction_id,
        ),
        source_before,
        "failed atomic install must retain every source transaction",
    );
}

#[test]
fn v47_composed_second_child_installs_with_exact_source_target_predecessor_chain() {
    let (mut candidate, prepared, _, _, _, source, target) = v47_atomic_posture_fixture('6');
    assert_ne!(source, target);
    assert_eq!(
        prepared
            .beginning_owner()
            .expected_predecessor_transaction_id,
        Some(source),
    );
    align_complete_owner_transaction(&mut candidate, source);
    let authoritative_beginning = candidate.clone();
    let first_trial = unpublished_composed_trial(
        &prepared,
        &candidate.inner.lse_configuration,
        60_000_000_000,
        120_000_000_000,
        f64::from_bits(3),
        '7',
    );
    let continuation = candidate
        .prepare_soil_thermal_base_unpublished_continuation_v2(
            &prepared,
            &first_trial,
            &first_trial.ending_state().state_sha256,
            120_000_000_000,
            1_800_000_000_000,
        )
        .expect("authenticated successor continuation");
    let second_credit = continuation_credit(
        continuation.child_beginning_state(),
        120_000_000_000,
        1_800_000_000_000,
        '8',
    );
    let snow_owner = ResourceOwnerId::try_new("v47-snow").expect("snow owner");
    let second_operands = continuation
        .child_top_boundary_operands_v2(&[second_credit], &snow_owner)
        .expect("successor second-child operands");
    let result = candidate
        .advance_soil_thermal_unpublished_continuation_v2(&continuation, &second_operands)
        .expect("successor second-child continuation result");
    let accepted = result
        .compose_accepted_outer_candidate(&candidate.inner.lse_configuration)
        .expect("successor composed accepted ending");
    assert_eq!(accepted.ending_owner.transaction_id, target);
    assert_eq!(
        accepted.ending_owner.expected_predecessor_transaction_id,
        Some(source),
    );
    let seals = seal_soil_thermal_accepted_candidate_v2(prepared.beginning_owner(), &accepted)
        .expect("successor accepted seals");
    let v3_authority = candidate
        .authenticate_soil_thermal_unpublished_continuation_install_authority_v3(
            &authoritative_beginning,
            &result,
            prepared.beginning_owner(),
        )
        .expect("sequential continuation three-domain authority");
    let mut v3_candidate = candidate.clone();
    v3_candidate
        .install_soil_thermal_accepted_v2_from_unpublished_continuation_v3(
            &authoritative_beginning,
            &result,
            prepared.beginning_owner(),
            v3_authority,
            accepted.clone(),
            seals.clone(),
        )
        .expect("sequential continuation V3 install");
    assert_eq!(
        v3_candidate
            .soil_thermal_v2()
            .expect("V3 sequential resident")
            .owner(),
        &accepted.ending_owner,
    );
    let authority = candidate
        .authenticate_soil_thermal_unpublished_continuation_install_authority_v2(
            &result,
            prepared.beginning_owner(),
        )
        .expect("explicit successor install authority");
    assert_eq!(authority.source_transaction_id, source);
    assert_eq!(authority.soil_thermal_transaction_id, target);
    let authoritative_bytes_before = authoritative_beginning
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("authoritative beginning bytes before install");
    candidate
        .install_soil_thermal_accepted_v2_from_unpublished_continuation(
            &authoritative_beginning,
            &result,
            prepared.beginning_owner(),
            authority,
            accepted.clone(),
            seals,
        )
        .expect("atomic successor continuation install");
    assert_eq!(
        candidate
            .soil_thermal_v2()
            .expect("installed successor resident")
            .owner(),
        &accepted.ending_owner,
    );
    assert_eq!(
        TransactionId(candidate.vegetation_state.0.last_transaction_id),
        source,
    );
    assert_eq!(
        candidate.lse_state.0.last_accepted_transaction_id,
        Some(source)
    );
    assert_eq!(
        candidate.inner.biogeochemistry.last_transaction_id,
        source.0
    );
    assert_eq!(
        authoritative_beginning
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("unchanged authoritative beginning bytes"),
        authoritative_bytes_before,
        "unpublished continuation never publishes through the authoritative beginning",
    );
}

#[test]
fn v48_authenticated_prepared_beginning_installs_exact_split() {
    let (mut candidate, prepared, resident, accepted, seals) =
        v48_r122_fixed_point_finalizer_fixture('a');
    let source = TransactionId(42);
    let target = TransactionId(43);
    let authoritative_beginning = candidate.clone();
    let authoritative_soil_before = authoritative_beginning
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("R122 authoritative beginning soil bytes");
    let publication_before = candidate.accepted_publication_history.clone();
    install_v2_soil_from_authenticated_prepared_beginning_v1(
        &mut candidate,
        &authoritative_beginning,
        prepared.beginning_owner(),
        accepted,
        seals,
    )
    .expect("real fixed-point finalizer authenticated prepared install");
    assert_eq!(
        candidate.soil_thermal_v2().expect("installed V2"),
        &resident
    );
    assert_eq!(resident.owner.transaction_id, target);
    assert_eq!(
        resident.owner.expected_predecessor_transaction_id,
        Some(source)
    );
    assert_eq!(resident.owner.support_start_ns, 1_800_000_000_000);
    assert_eq!(resident.owner.support_end_ns, 1_980_000_000_000);
    assert_eq!(
        TransactionId(candidate.vegetation_state.0.last_transaction_id),
        source,
    );
    assert_eq!(
        candidate.lse_state.0.last_accepted_transaction_id,
        Some(source)
    );
    assert_eq!(
        candidate.inner.biogeochemistry.last_transaction_id,
        source.0
    );
    assert_eq!(candidate.accepted_publication_history, publication_before);
    assert_eq!(
        authoritative_beginning
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("R122 unchanged authoritative beginning soil bytes"),
        authoritative_soil_before,
    );
}

#[test]
fn v48_generic_install_remains_strict_same_id() {
    let (mut candidate, prepared, _, accepted, seals) = v48_r122_fixed_point_finalizer_fixture('b');
    let source = TransactionId(42);
    let target = TransactionId(43);
    assert_ne!(source, target);
    align_complete_owner_transaction(&mut candidate, source);
    let before = candidate
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("generic split refusal beginning bytes");
    assert!(
        candidate
            .install_soil_thermal_accepted_v2(prepared.beginning_owner(), accepted, seals)
            .is_err(),
    );
    assert_eq!(
        candidate
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("generic split refusal ending bytes"),
        before,
    );
}

#[test]
fn v48_prepared_beginning_authority_refuses_substitution() {
    let (mut candidate, prepared, _, accepted, seals) = v48_r122_fixed_point_finalizer_fixture('c');
    let source = TransactionId(42);
    let target = TransactionId(43);
    let authoritative_beginning = candidate.clone();
    let (_, foreign_prepared, _, _, _) = v48_r122_fixed_point_finalizer_fixture('d');
    assert!(
        candidate
            .authenticate_soil_thermal_prepared_beginning_install_authority_v2(
                &authoritative_beginning,
                foreign_prepared.beginning_owner(),
            )
            .is_err(),
        "foreign prepared receipt/owner custody must refuse",
    );
    let swapped =
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            target, source,
        )
        .expect("swapped explicit authority poison");
    assert!(
        candidate
            .install_soil_thermal_accepted_v2_from_authenticated_beginning(
                &authoritative_beginning,
                prepared.beginning_owner(),
                swapped,
                accepted,
                seals,
            )
            .is_err(),
        "swapped source/target authority must refuse",
    );
}

#[test]
fn v48_authenticated_final_install_rolls_back_on_refusal() {
    let (mut candidate, prepared, _, accepted, seals) = v48_r122_fixed_point_finalizer_fixture('e');
    let source = TransactionId(42);
    let target = TransactionId(43);
    let authoritative_beginning = candidate.clone();
    let soil_before = candidate
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("authenticated final-install beginning soil bytes");
    let sources_before = (
        candidate.vegetation_state.0.last_transaction_id,
        candidate.lse_state.0.last_accepted_transaction_id,
        candidate.inner.biogeochemistry.last_transaction_id,
    );
    let foreign =
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            TransactionId(source.0 + 9),
            target,
        )
        .expect("foreign explicit authority poison");
    assert!(
        candidate
            .install_soil_thermal_accepted_v2_from_authenticated_beginning(
                &authoritative_beginning,
                prepared.beginning_owner(),
                foreign,
                accepted,
                seals,
            )
            .is_err(),
    );
    assert_eq!(
        candidate
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("authenticated final-install rollback soil bytes"),
        soil_before,
    );
    assert_eq!(
        (
            candidate.vegetation_state.0.last_transaction_id,
            candidate.lse_state.0.last_accepted_transaction_id,
            candidate.inner.biogeochemistry.last_transaction_id,
        ),
        sources_before,
    );
}

#[test]
fn v48_authenticated_same_id_and_exact_noop_do_not_publish() {
    let (mut candidate, prepared, resident, accepted, seals) =
        v48_r122_fixed_point_finalizer_fixture('f');
    align_complete_owner_transaction(&mut candidate, TransactionId(43));
    let authoritative_beginning = candidate.clone();
    let publication_before = candidate.accepted_publication_history.clone();
    for pass in 0..2 {
        install_v2_soil_from_authenticated_prepared_beginning_v1(
            &mut candidate,
            &authoritative_beginning,
            prepared.beginning_owner(),
            accepted.clone(),
            seals.clone(),
        )
        .unwrap_or_else(|error| panic!("same-ID/no-op pass {pass}: {error}"));
    }
    assert_eq!(candidate.soil_thermal_v2().expect("same-ID V2"), &resident);
    assert_eq!(candidate.accepted_publication_history, publication_before);
}

#[test]
fn v48_authenticated_prepared_custody_poison_matrix() {
    let (mut candidate, prepared, _, accepted, seals) = v48_r122_fixed_point_finalizer_fixture('9');
    let authoritative_beginning = candidate.clone();
    let authority = candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v2(
            &authoritative_beginning,
            prepared.beginning_owner(),
        )
        .expect("exact R122 authority");
    let soil_before = candidate
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("R122 poison beginning soil bytes");
    let publication_before = candidate.accepted_publication_history.clone();
    let sources_before = (
        candidate.vegetation_state.0.last_transaction_id,
        candidate.lse_state.0.last_accepted_transaction_id,
        candidate.inner.biogeochemistry.last_transaction_id,
    );

    let mut prepared_support = prepared.beginning_owner().clone();
    prepared_support.support_end_ns += 60_000_000_000;
    let mut prepared_receipt = prepared.beginning_owner().clone();
    prepared_receipt.receipt_chain_sha256 = digest('6');
    for (label, prepared_poison) in [
        ("prepared-support", prepared_support),
        ("prepared-receipt", prepared_receipt),
    ] {
        assert!(
            candidate
                .install_soil_thermal_accepted_v2_from_authenticated_beginning(
                    &authoritative_beginning,
                    &prepared_poison,
                    authority,
                    accepted.clone(),
                    seals.clone(),
                )
                .is_err(),
            "{label} substitution must refuse",
        );
        assert_eq!(
            candidate
                .soil_thermal_resident()
                .canonical_active_owner_bytes()
                .expect("R122 prepared poison rollback soil bytes"),
            soil_before,
            "{label} refusal changed soil owner bytes",
        );
        assert_eq!(
            (
                candidate.vegetation_state.0.last_transaction_id,
                candidate.lse_state.0.last_accepted_transaction_id,
                candidate.inner.biogeochemistry.last_transaction_id,
            ),
            sources_before,
            "{label} refusal changed source owners",
        );
        assert_eq!(
            candidate.accepted_publication_history, publication_before,
            "{label} refusal published history",
        );
    }

    let mut target = accepted.clone();
    target.ending_owner.transaction_id = TransactionId(44);
    let mut predecessor = accepted.clone();
    predecessor.ending_owner.expected_predecessor_transaction_id = Some(TransactionId(41));
    let mut support = accepted.clone();
    support.ending_owner.support_end_ns += 60_000_000_000;
    let mut receipt = accepted.clone();
    receipt.ending_owner.receipt_chain_sha256 = digest('8');
    let mut state = accepted.clone();
    state.ending_owner.state.ofes[0].ordered_layers[0].temperature_k = f64::from_bits(
        state.ending_owner.state.ofes[0].ordered_layers[0]
            .temperature_k
            .to_bits()
            + 1,
    );
    state
        .ending_owner
        .state
        .reseal()
        .expect("resealed state poison");
    let mut layer = accepted.clone();
    layer.ending_owner.state.ofes[0].ordered_layers[0].last_accepted_transaction_id =
        Some(TransactionId(44));
    layer
        .ending_owner
        .state
        .reseal()
        .expect("resealed layer poison");
    let mut seal = seals.clone();
    seal.orchestrator_seal_sha256 = digest('7');

    for (label, accepted_poison, seals_poison) in [
        ("target", target, seals.clone()),
        ("predecessor", predecessor, seals.clone()),
        ("support", support, seals.clone()),
        ("receipt", receipt, seals.clone()),
        ("accepted-state", state, seals.clone()),
        ("accepted-layer", layer, seals.clone()),
        ("seal", accepted.clone(), seal),
    ] {
        assert!(
            candidate
                .install_soil_thermal_accepted_v2_from_authenticated_beginning(
                    &authoritative_beginning,
                    prepared.beginning_owner(),
                    authority,
                    accepted_poison,
                    seals_poison,
                )
                .is_err(),
            "{label} substitution must refuse",
        );
        assert_eq!(
            candidate
                .soil_thermal_resident()
                .canonical_active_owner_bytes()
                .expect("R122 poison rollback soil bytes"),
            soil_before,
            "{label} refusal changed soil owner bytes",
        );
        assert_eq!(
            candidate.accepted_publication_history, publication_before,
            "{label} refusal published history",
        );
        assert_eq!(
            (
                candidate.vegetation_state.0.last_transaction_id,
                candidate.lse_state.0.last_accepted_transaction_id,
                candidate.inner.biogeochemistry.last_transaction_id,
            ),
            sources_before,
            "{label} refusal changed source owners",
        );
    }
}

fn unpublished_composed_trial(
    original: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
    configuration: &LandSurfaceEnergyConfiguration,
    support_start_ns: u128,
    support_end_ns: u128,
    energy: f64,
    identity: char,
) -> openwepp_land_surface_energy::SoilThermalTrialStateV2 {
    let mut operand = exact_operand(original.beginning_owner(), energy);
    operand.debit_credit_identity_sha256 = digest(identity);
    let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        original.beginning_owner(),
        configuration,
        vec![operand],
    )
    .expect("composed expected operands");
    openwepp_land_surface_energy::advance_soil_thermal_composed_trial_v2(
        original,
        support_start_ns,
        support_end_ns,
        expected.accepted_operands(),
        expected.temperature_projections(),
    )
    .expect("unpublished composed trial")
}

fn continuation_credit(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnedStateV2,
    support_start_ns: i64,
    support_end_ns: i64,
    identity: char,
) -> SoilThermalTopBoundaryCreditV1 {
    SoilThermalTopBoundaryCreditV1 {
        lane_id: 1,
        ofe_id: beginning.ofes[0].ofe_id.clone(),
        first_layer_id: beginning.ofes[0].ordered_layers[0].layer_id.clone(),
        beginning_owner_id: beginning.owner_id.clone(),
        beginning_configuration_sha256: beginning.configuration_sha256.clone(),
        beginning_state_sha256: beginning.state_sha256.clone(),
        support_start_ns,
        support_end_ns,
        accepted_positive_downward_j_m2_ofe_ground: f64::from_bits(1),
        soil_thermal_credit_j_m2_ofe_ground: f64::from_bits(1),
        snow_soil_heat_receipt_sha256: digest(identity),
    }
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
    let derived_first = authoritative
        .prepare_next_soil_thermal_support_v2(0, 60_000_000_000)
        .expect("first support transaction from receipt-free custody");
    assert_eq!(
        derived_first.beginning_owner(),
        first_prepared.beginning_owner()
    );
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

    let successor_prepared = first_ending
        .prepare_next_soil_thermal_support_v2(60_000_000_000, 120_000_000_000)
        .expect("successor prepared from installed resident");
    let successor_transaction = successor_prepared.beginning_owner().transaction_id;
    assert_eq!(
        successor_transaction,
        TransactionId(first_transaction.0 + 1)
    );
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

    let third = successor_ending
        .prepare_next_soil_thermal_support_v2(120_000_000_000, 180_000_000_000)
        .expect("third support from second installed predecessor");
    assert_eq!(
        third.beginning_owner().transaction_id,
        TransactionId(successor_transaction.0 + 1)
    );
    assert_eq!(
        third.beginning_owner().expected_predecessor_transaction_id,
        Some(successor_transaction)
    );
    assert_eq!(
        third.beginning_owner().receipt_chain_sha256,
        successor_ending
            .soil_thermal_v2()
            .expect("successor resident")
            .owner()
            .receipt_chain_sha256
    );
    assert!(
        third
            .beginning_owner()
            .state
            .ofes
            .iter()
            .flat_map(|ofe| &ofe.ordered_layers)
            .any(|layer| layer.enthalpy_carry.sign != 0),
        "third beginning must retain the exact installed carry"
    );

    let before = successor_ending
        .inner
        .soil_thermal
        .canonical_active_owner_bytes()
        .expect("before support poison bytes");
    for (start, end) in [
        (60_000_000_000, 120_000_000_000),
        (120_000_000_001, 180_000_000_001),
        (120_000_000_000, 179_999_999_999),
    ] {
        assert!(
            successor_ending
                .prepare_next_soil_thermal_support_v2(start, end)
                .is_err(),
            "stale, noncontiguous, and sub-floor supports must fail closed"
        );
        assert_eq!(
            successor_ending
                .inner
                .soil_thermal
                .canonical_active_owner_bytes()
                .expect("after support poison bytes"),
            before,
            "preparation refusal must not mutate the installed resident"
        );
    }
}

fn v39_v47_second_child_continuation_transaction_behavior() {
    let (authoritative, _) = native_v2_shadow_for_parent('c');
    let original = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("original parent support");
    let transaction = original.beginning_owner().transaction_id;
    let first_prepared = authoritative
        .prepare_soil_thermal_support_v2(transaction, 0, 60_000_000_000)
        .expect("first child prepared");
    let first_trial = unpublished_composed_trial(
        &original,
        &authoritative.inner.lse_configuration,
        0,
        60_000_000_000,
        f64::from_bits(1),
        'd',
    );
    let first_candidate = DirectSoilThermalCandidate::from_v2(first_trial.clone())
        .expect("first unpublished candidate");
    let first_authenticated_operands = first_trial
        .layer_credits()
        .iter()
        .flat_map(|credit| credit.accepted_operands.iter().cloned())
        .collect::<Vec<_>>();
    let base_result = authoritative
        .authenticate_soil_thermal_base_unpublished_result_v2(
            &original,
            &first_trial,
            &first_authenticated_operands,
        )
        .expect("authenticated base unpublished result");
    assert_eq!(base_result.physical_trial(), &first_trial);
    assert_eq!(
        base_result
            .original_prepared()
            .beginning_owner()
            .support_start_ns,
        0
    );
    assert_eq!(
        base_result
            .original_prepared()
            .beginning_owner()
            .support_end_ns,
        60_000_000_000,
        "base result must retain only the authenticated first-child support",
    );
    assert_eq!(
        base_result.accumulated_operands(),
        first_authenticated_operands
    );
    let base_before = authoritative
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before base-result poisons");
    let mut operand_poison = first_authenticated_operands.clone();
    operand_poison[0].energy_j_m2_ofe_ground = f64::from_bits(2);
    assert!(
        authoritative
            .authenticate_soil_thermal_base_unpublished_result_v2(
                &original,
                &first_trial,
                &operand_poison,
            )
            .is_err(),
        "operand/carry substitution must fail closed",
    );
    let stale_support = authoritative
        .prepare_soil_thermal_support_v2(transaction, 1, 1_800_000_000_000)
        .expect("stale base-result support");
    assert!(
        authoritative
            .authenticate_soil_thermal_base_unpublished_result_v2(
                &stale_support,
                &first_trial,
                &first_authenticated_operands,
            )
            .is_err(),
        "stale original support must fail closed",
    );
    let foreign_transaction_support =
        openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
            authoritative
                .soil_thermal_v2()
                .expect("authoritative V2 resident")
                .owner(),
            TransactionId(transaction.0 + 1),
            0,
            1_800_000_000_000,
        )
        .expect("foreign transaction support shape");
    let foreign_transaction_trial = unpublished_composed_trial(
        &foreign_transaction_support,
        &authoritative.inner.lse_configuration,
        0,
        60_000_000_000,
        f64::from_bits(1),
        'f',
    );
    let foreign_transaction_operands = foreign_transaction_trial
        .layer_credits()
        .iter()
        .flat_map(|credit| credit.accepted_operands.iter().cloned())
        .collect::<Vec<_>>();
    assert!(
        authoritative
            .authenticate_soil_thermal_base_unpublished_result_v2(
                &foreign_transaction_support,
                &foreign_transaction_trial,
                &foreign_transaction_operands,
            )
            .is_err(),
        "foreign soil transaction must fail before private publication",
    );
    let (foreign, _) = native_v2_shadow_for_parent('e');
    let foreign_original = foreign
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("foreign original support");
    let foreign_trial = unpublished_composed_trial(
        &foreign_original,
        &foreign.inner.lse_configuration,
        0,
        60_000_000_000,
        f64::from_bits(1),
        'd',
    );
    let foreign_operands = foreign_trial
        .layer_credits()
        .iter()
        .flat_map(|credit| credit.accepted_operands.iter().cloned())
        .collect::<Vec<_>>();
    assert!(
        authoritative
            .authenticate_soil_thermal_base_unpublished_result_v2(
                &foreign_original,
                &foreign_trial,
                &foreign_operands,
            )
            .is_err(),
        "foreign state/receipt chain must fail closed",
    );
    assert_eq!(
        authoritative
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after base-result poisons"),
        base_before,
        "base-result construction and refusal must never install",
    );
    let physical_beginning = authoritative
        .prepare_soil_thermal_unpublished_physical_beginning_v2(
            &first_candidate,
            None,
            60_000_000_000,
            1_800_000_000_000,
        )
        .expect("authenticated unpublished physical beginning");
    assert_eq!(
        physical_beginning.predecessor_trial(),
        &first_trial,
        "physical beginning must retain the exact sealed trial"
    );
    assert_eq!(physical_beginning.support_start_ns(), 60_000_000_000);
    assert_eq!(physical_beginning.support_end_ns(), 1_800_000_000_000);
    for (poison_start, poison_end) in [
        (60_000_000_001, 1_800_000_000_000),
        (60_000_000_000, 119_999_999_999),
    ] {
        assert!(
            authoritative
                .prepare_soil_thermal_unpublished_physical_beginning_v2(
                    &first_candidate,
                    None,
                    poison_start,
                    poison_end,
                )
                .is_err(),
            "transaction/support poison must fail closed"
        );
    }
    let before = authoritative
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before continuation bytes");
    let second_context = authoritative
        .prepare_soil_thermal_base_unpublished_continuation_v2(
            &original,
            &first_trial,
            &first_trial.ending_state().state_sha256,
            60_000_000_000,
            1_800_000_000_000,
        )
        .expect("second child continuation");
    assert_eq!(
        second_context.child_beginning_state(),
        first_trial.ending_state()
    );
    let snow_owner = ResourceOwnerId::try_new("snow").expect("snow owner");
    let second_credit = continuation_credit(
        second_context.child_beginning_state(),
        60_000_000_000,
        1_800_000_000_000,
        'e',
    );
    let second_operands = second_context
        .child_top_boundary_operands_v2(&[second_credit], &snow_owner)
        .expect("second child top-boundary operands");
    let second_result = authoritative
        .advance_soil_thermal_unpublished_continuation_v2(&second_context, &second_operands)
        .expect("second composed trial");
    let second_trial = second_result.physical_trial();
    let terminal_suffix = second_trial
        .layer_credits()
        .iter()
        .flat_map(|credit| credit.accepted_operands.iter().cloned())
        .collect::<Vec<_>>();
    second_result
        .validate_terminal_operand_suffix(&terminal_suffix)
        .expect("exact terminal operand suffix");
    let mut substituted_suffix = terminal_suffix.clone();
    substituted_suffix[0].debit_credit_identity_sha256 = digest('8');
    assert!(
        second_result
            .validate_terminal_operand_suffix(&substituted_suffix)
            .is_err(),
        "terminal operand substitution must fail closed"
    );
    let mut reordered_suffix = terminal_suffix.clone();
    reordered_suffix[0].ordinal = 1;
    assert!(
        second_result
            .validate_terminal_operand_suffix(&reordered_suffix)
            .is_err(),
        "terminal operand reorder must fail closed"
    );
    assert_eq!(second_trial.support_start_ns(), 60_000_000_000);
    assert_eq!(second_trial.support_end_ns(), 1_800_000_000_000);
    assert_eq!(
        second_trial.beginning_state_sha256(),
        &first_trial.ending_state().state_sha256
    );
    assert_eq!(
        second_trial.transaction_id(),
        TransactionId(transaction.0 + 1)
    );
    assert_eq!(second_trial.predecessor_transaction_id(), Some(transaction));
    assert_eq!(
        second_result.accumulated_operands().len(),
        2,
        "final replay sidecar must retain both child operands"
    );
    assert_eq!(
        second_trial
            .layer_credits()
            .iter()
            .map(|credit| credit.accepted_operands.len())
            .sum::<usize>(),
        1,
        "sequential physical trial must contain only the second-child operand"
    );
    let first_expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        first_prepared.beginning_owner(),
        &authoritative.inner.lse_configuration,
        first_trial
            .layer_credits()
            .iter()
            .flat_map(|credit| credit.accepted_operands.iter().cloned())
            .collect(),
    )
    .expect("first accepted replay operands");
    let first_accepted = aggregate_soil_thermal_ending_v2(
        first_prepared.beginning_owner(),
        &authoritative.inner.lse_configuration,
        &first_expected,
    )
    .expect("first accepted replay");
    let accepted_second_prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
        &first_accepted.ending_owner,
        TransactionId(transaction.0 + 1),
        60_000_000_000,
        1_800_000_000_000,
    )
    .expect("fresh accepted second beginning");
    let accepted_second_expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        accepted_second_prepared.beginning_owner(),
        &authoritative.inner.lse_configuration,
        second_trial
            .layer_credits()
            .iter()
            .flat_map(|credit| credit.accepted_operands.iter().cloned())
            .collect(),
    )
    .expect("fresh accepted second operands");
    let accepted_second_trial = openwepp_land_surface_energy::advance_soil_thermal_trial_v2(
        &accepted_second_prepared,
        accepted_second_expected.accepted_operands(),
        accepted_second_expected.temperature_projections(),
    )
    .expect("fresh accepted second physical trial");
    assert_eq!(
        second_trial.ending_state(),
        accepted_second_trial.ending_state(),
        "private sequential continuation must match fresh accepted child physics"
    );
    assert_eq!(
        second_trial.layer_credits(),
        accepted_second_trial.layer_credits()
    );
    let first_seals =
        seal_soil_thermal_accepted_candidate_v2(first_prepared.beginning_owner(), &first_accepted)
            .expect("first accepted seals");
    let mut accepted_beginning = authoritative.clone();
    align_complete_owner_transaction(&mut accepted_beginning, transaction);
    accepted_beginning
        .install_soil_thermal_accepted_v2_from_beginning(
            &authoritative,
            first_prepared.beginning_owner(),
            first_accepted.clone(),
            first_seals,
        )
        .expect("first authoritative accepted resident");
    let accepted_second_physical = aggregate_soil_thermal_ending_v2(
        accepted_second_prepared.beginning_owner(),
        &authoritative.inner.lse_configuration,
        &accepted_second_expected,
    )
    .expect("independently accepted second child");
    let final_expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        original.beginning_owner(),
        &authoritative.inner.lse_configuration,
        second_result.accumulated_operands().to_vec(),
    )
    .expect("final accumulated replay operands");
    let accepted_second = second_result
        .compose_accepted_outer_candidate(&authoritative.inner.lse_configuration)
        .expect("final accumulated sequential composition");
    assert_eq!(
        accepted_second.expected_sources.accepted_operands(),
        final_expected.accepted_operands()
    );
    assert!(
        soil_thermal_v2_physical_ending_matches(
            &accepted_second.ending_owner.state,
            &accepted_second_physical.ending_owner.state,
        ),
        "final outer replay must retain the selected sequential physical ending"
    );
    let accepted_second_seals =
        seal_soil_thermal_accepted_candidate_v2(original.beginning_owner(), &accepted_second)
            .expect("second accepted seals");
    let second_transaction = original.beginning_owner().transaction_id;
    let candidate = || {
        let mut value = authoritative.clone();
        align_complete_owner_transaction(&mut value, second_transaction);
        value
    };
    let assert_refused = |mut value: DirectV10RealConsumerShadow,
                          sidecar: &DirectSoilThermalUnpublishedContinuationResultV2,
                          beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
                          accepted: SoilThermalAcceptedCandidateV2,
                          seals: SoilThermalOrchestratorSealsV2| {
        let before = value
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("before specialized refusal bytes");
        let result = value
            .authenticate_soil_thermal_unpublished_continuation_install_authority_v2(
                sidecar, beginning,
            )
            .and_then(|transaction_authority| {
                value.install_soil_thermal_accepted_v2_from_unpublished_continuation(
                    &authoritative,
                    sidecar,
                    beginning,
                    transaction_authority,
                    accepted,
                    seals,
                )
            });
        assert!(result.is_err());
        assert_eq!(
            value
                .soil_thermal_resident()
                .canonical_active_owner_bytes()
                .expect("after specialized refusal bytes"),
            before
        );
    };
    let mut substituted_sidecar = second_result.clone();
    substituted_sidecar.physical_trial = first_trial.clone();
    assert_refused(
        candidate(),
        &substituted_sidecar,
        original.beginning_owner(),
        accepted_second.clone(),
        accepted_second_seals.clone(),
    );
    let mut stale_original = second_result.clone();
    stale_original.original_prepared = first_prepared.clone();
    assert_refused(
        candidate(),
        &stale_original,
        original.beginning_owner(),
        accepted_second.clone(),
        accepted_second_seals.clone(),
    );
    let mut stale_support = original.beginning_owner().clone();
    stale_support.support_start_ns += 1;
    assert_refused(
        candidate(),
        &second_result,
        &stale_support,
        accepted_second.clone(),
        accepted_second_seals.clone(),
    );
    let mut stale_transaction = original.beginning_owner().clone();
    stale_transaction.transaction_id = TransactionId(stale_transaction.transaction_id.0 + 1);
    assert_refused(
        candidate(),
        &second_result,
        &stale_transaction,
        accepted_second.clone(),
        accepted_second_seals.clone(),
    );
    let mut stale_receipt = accepted_second.clone();
    stale_receipt
        .credit_receipt
        .predecessor_receipt_chain_sha256 = digest('7');
    stale_receipt
        .credit_receipt
        .reseal()
        .expect("reseal stale receipt chain");
    assert_refused(
        candidate(),
        &second_result,
        original.beginning_owner(),
        stale_receipt,
        accepted_second_seals.clone(),
    );
    let mut accepted_poison_operands = final_expected.accepted_operands().to_vec();
    let mut accepted_poison_operand = accepted_poison_operands[0].clone();
    accepted_poison_operand.energy_j_m2_ofe_ground = f64::from_bits(2);
    accepted_poison_operand.debit_credit_identity_sha256 = digest('0');
    accepted_poison_operands[0] = accepted_poison_operand;
    let accepted_poison_expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        original.beginning_owner(),
        &authoritative.inner.lse_configuration,
        accepted_poison_operands,
    )
    .expect("substituted accepted operand set");
    let accepted_poison = aggregate_soil_thermal_ending_v2(
        original.beginning_owner(),
        &authoritative.inner.lse_configuration,
        &accepted_poison_expected,
    )
    .expect("substituted accepted child");
    let accepted_poison_seals =
        seal_soil_thermal_accepted_candidate_v2(original.beginning_owner(), &accepted_poison)
            .expect("substituted accepted seals");
    assert_refused(
        candidate(),
        &second_result,
        original.beginning_owner(),
        accepted_poison,
        accepted_poison_seals,
    );
    let validated = candidate();
    let validated_before = validated
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before read-only selected-trial validation");
    validated
        .validate_soil_thermal_accepted_v2_from_unpublished_continuation(
            &second_trial,
            &second_result,
            original.beginning_owner(),
            &accepted_second,
        )
        .expect("typed selected unpublished continuation validation");
    assert!(
        validated
            .validate_soil_thermal_accepted_v2_from_unpublished_continuation(
                &first_trial,
                &second_result,
                original.beginning_owner(),
                &accepted_second,
            )
            .is_err()
    );
    assert_eq!(
        validated
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after read-only selected-trial validation"),
        validated_before,
        "typed selected-trial validation must not install or publish",
    );
    let mut installed = candidate();
    let installed_transaction_authority = installed
        .authenticate_soil_thermal_unpublished_continuation_install_authority_v2(
            &second_result,
            original.beginning_owner(),
        )
        .expect("authenticated split install transaction authority");
    installed
        .install_soil_thermal_accepted_v2_from_unpublished_continuation(
            &authoritative,
            &second_result,
            original.beginning_owner(),
            installed_transaction_authority,
            accepted_second.clone(),
            accepted_second_seals.clone(),
        )
        .expect("single specialized continuation install");
    assert_eq!(
        installed
            .soil_thermal_v2()
            .expect("installed sequential resident")
            .owner(),
        &accepted_second.ending_owner
    );
    let installed_view = installed.soil_thermal_resident().read_view();
    let sequential_view = DirectSoilThermalReadView::V2(second_trial.ending_state());
    assert_ne!(
        installed_view, sequential_view,
        "outer acceptance and sequential trial retain distinct custody identities",
    );
    assert!(
        installed_view.physically_equals(sequential_view),
        "outer acceptance must preserve the selected sequential high/carry/temperature topology",
    );
    let exact_noop_authority = installed.clone();
    let exact_noop_before = installed
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before exact accepted continuation no-op");
    let exact_noop_transaction_authority = installed
        .authenticate_soil_thermal_unpublished_continuation_install_authority_v2(
            &second_result,
            original.beginning_owner(),
        )
        .expect("authenticated exact-noop transaction authority");
    installed
        .install_soil_thermal_accepted_v2_from_unpublished_continuation(
            &exact_noop_authority,
            &second_result,
            original.beginning_owner(),
            exact_noop_transaction_authority,
            accepted_second.clone(),
            accepted_second_seals,
        )
        .expect("exact accepted continuation no-op");
    assert_eq!(
        installed
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after exact accepted continuation no-op"),
        exact_noop_before,
    );

    assert!(
        second_trial
            .ending_state()
            .ofes
            .iter()
            .flat_map(|ofe| &ofe.ordered_layers)
            .any(|layer| layer.enthalpy_carry.sign != 0),
        "composed trial must retain exact carry"
    );
    assert_eq!(
        authoritative
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after continuation bytes"),
        before,
        "unpublished continuation must not install or publish an owner"
    );
}

#[test]
fn v39_second_child_continuation_uses_authenticated_soil_transaction_and_refuses_foreign_custody() {
    v39_v47_second_child_continuation_transaction_behavior();
}

#[test]
fn unpublished_physical_beginning_rejects_same_support_predecessor_reuse() {
    let (authoritative, _) = native_v2_shadow_for_parent('a');
    let original = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("original parent support");
    let first_trial = unpublished_composed_trial(
        &original,
        &authoritative.inner.lse_configuration,
        0,
        900_000_000_000,
        f64::from_bits(1),
        'b',
    );
    let candidate =
        DirectSoilThermalCandidate::from_v2(first_trial).expect("first unpublished candidate");
    let before = authoritative
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before same-support rejection");
    assert!(
        authoritative
            .prepare_soil_thermal_unpublished_physical_beginning_v2(
                &candidate,
                None,
                0,
                900_000_000_000,
            )
            .is_err(),
        "a trial cannot also be the predecessor of the same child support"
    );
    assert_eq!(
        authoritative
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after same-support rejection"),
        before,
        "same-support refusal must not mutate resident custody"
    );
}

#[test]
fn unpublished_physical_beginning_uses_authenticated_prior_original_support_start() {
    let (authoritative, _) = native_v2_shadow_for_parent('6');
    let authenticated_original = authoritative
        .prepare_next_soil_thermal_support_v2(900_000_000_000, 1_500_000_000_000)
        .expect("authenticated shifted original support");
    let retained_trial = unpublished_composed_trial(
        &authenticated_original,
        &authoritative.inner.lse_configuration,
        900_000_000_000,
        1_200_000_000_000,
        f64::from_bits(1),
        '7',
    );
    let continuation = authoritative
        .prepare_soil_thermal_base_unpublished_continuation_v2(
            &authenticated_original,
            &retained_trial,
            &retained_trial.ending_state().state_sha256,
            1_200_000_000_000,
            1_500_000_000_000,
        )
        .expect("authenticated shifted continuation");
    let snow_owner = ResourceOwnerId::try_new("snow").expect("snow owner");
    let child_operands = continuation
        .child_top_boundary_operands_v2(
            &[continuation_credit(
                continuation.child_beginning_state(),
                1_200_000_000_000,
                1_500_000_000_000,
                '8',
            )],
            &snow_owner,
        )
        .expect("shifted continuation operands");
    let prior = authoritative
        .advance_soil_thermal_unpublished_continuation_v2(&continuation, &child_operands)
        .expect("authenticated shifted prior result");
    let candidate = DirectSoilThermalCandidate::from_v2(prior.physical_trial().clone())
        .expect("shifted retained candidate");
    let before = authoritative
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before shifted prior start");
    let beginning = authoritative
        .inner
        .soil_thermal
        .prepare_unpublished_physical_beginning_v2(
            &authoritative.inner.lse_configuration,
            &candidate,
            Some(&prior),
            1_500_000_000_000,
            1_800_000_000_000,
        )
        .expect("physical beginning from shifted prior authority");
    assert_eq!(beginning.predecessor_trial(), prior.physical_trial());
    assert_eq!(beginning.support_start_ns(), 1_500_000_000_000);
    assert_eq!(beginning.support_end_ns(), 1_800_000_000_000);
    assert_eq!(
        authoritative
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after shifted prior start"),
        before,
        "shifted prior preparation must not install resident custody"
    );
    let base_candidate = DirectSoilThermalCandidate::from_v2(retained_trial.clone())
        .expect("shifted base candidate");
    let base_beginning = authoritative
        .inner
        .soil_thermal
        .prepare_unpublished_physical_beginning_v2(
            &authoritative.inner.lse_configuration,
            &base_candidate,
            None,
            1_200_000_000_000,
            1_800_000_000_000,
        )
        .expect("physical beginning from shifted base authority");
    assert_eq!(base_beginning.predecessor_trial(), &retained_trial);
    assert_eq!(base_beginning.support_start_ns(), 1_200_000_000_000);
    assert_eq!(base_beginning.support_end_ns(), 1_800_000_000_000);
}

#[test]
fn unpublished_physical_beginning_rebuilds_same_support_prior_from_installed_resident() {
    let (authoritative, _) = native_v2_shadow_for_parent('a');
    let parent = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("installed parent support");
    let parent_transaction = parent.beginning_owner().transaction_id;
    let (accepted, seals) = accepted_bundle(
        &parent,
        &authoritative.inner.lse_configuration,
        f64::from_bits(1),
        'b',
    );
    let mut installed = authoritative.clone();
    align_complete_owner_transaction(&mut installed, parent_transaction);
    installed
        .install_soil_thermal_accepted_v2_from_beginning(
            &authoritative,
            parent.beginning_owner(),
            accepted,
            seals,
        )
        .expect("install parent resident");

    let retained_trial = unpublished_composed_trial(
        &parent,
        &authoritative.inner.lse_configuration,
        0,
        1_800_000_000_000,
        f64::from_bits(1),
        'b',
    );
    let retained = DirectSoilThermalCandidate::from_v2(retained_trial.clone())
        .expect("retained prior-support candidate");
    let fresh = installed
        .prepare_next_soil_thermal_support_v2(1_800_000_000_000, 3_600_000_000_000)
        .expect("fresh child support");
    let child_trial = unpublished_composed_trial(
        &fresh,
        &installed.inner.lse_configuration,
        1_800_000_000_000,
        3_600_000_000_000,
        f64::from_bits(1),
        'c',
    );
    let mut child_operand = exact_operand(fresh.beginning_owner(), f64::from_bits(1));
    child_operand.debit_credit_identity_sha256 = digest('c');
    let prior = installed
        .authenticate_soil_thermal_base_unpublished_result_v2(
            &fresh,
            &child_trial,
            &[child_operand],
        )
        .expect("same-support cached base result");
    let fixed_point_candidate = DirectSoilThermalCandidate::from_v2(child_trial.clone())
        .expect("same-support fixed-point candidate");
    assert!(
        installed
            .inner
            .soil_thermal
            .validate_unpublished_fixed_point_v2(
                &installed.inner.lse_configuration,
                &fixed_point_candidate,
                Some(&prior),
                1_800_000_000_000,
                3_600_000_000_000,
            )
            .expect("authenticate same-support fixed-point history")
    );
    assert!(
        installed
            .inner
            .soil_thermal
            .validate_unpublished_fixed_point_v2(
                &installed.inner.lse_configuration,
                &fixed_point_candidate,
                None,
                1_800_000_000_000,
                3_600_000_000_000,
            )
            .expect("authenticate candidate-only same-support base trial")
    );
    let before = installed
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before same-support rebuild");
    let beginning = installed
        .inner
        .soil_thermal
        .prepare_unpublished_physical_beginning_v2(
            &installed.inner.lse_configuration,
            &retained,
            Some(&prior),
            1_800_000_000_000,
            3_600_000_000_000,
        )
        .expect("same-support prior rebuild");
    assert_eq!(beginning.predecessor_trial(), &retained_trial);
    assert_eq!(beginning.authority().beginning_owner().support_start_ns, 0);
    assert_eq!(
        beginning.authority().beginning_owner().support_end_ns,
        3_600_000_000_000
    );
    assert_eq!(
        beginning.authority().beginning_owner().transaction_id,
        fresh.beginning_owner().transaction_id
    );
    assert_eq!(
        installed
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after same-support rebuild"),
        before
    );

    assert!(
        installed
            .inner
            .soil_thermal
            .validate_unpublished_fixed_point_v2(
                &installed.inner.lse_configuration,
                &fixed_point_candidate,
                Some(&prior),
                1_800_000_000_000,
                3_600_000_000_000,
            )
            .is_ok()
    );
    let mut substituted_prior = prior.clone();
    substituted_prior.physical_trial = retained_trial;
    assert!(
        installed
            .inner
            .soil_thermal
            .validate_unpublished_fixed_point_v2(
                &installed.inner.lse_configuration,
                &fixed_point_candidate,
                Some(&substituted_prior),
                1_800_000_000_000,
                3_600_000_000_000,
            )
            .is_err()
    );
    assert_eq!(
        installed
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after same-support poison"),
        before
    );
}

pub(super) fn unpublished_aggregate_candidate_only_behavior() {
    let (authoritative, _) = native_v2_shadow_for_parent('a');
    let parent = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("installed parent support");
    let parent_transaction = parent.beginning_owner().transaction_id;
    let (accepted, seals) = accepted_bundle(
        &parent,
        &authoritative.inner.lse_configuration,
        f64::from_bits(1),
        'b',
    );
    let mut installed = authoritative.clone();
    align_complete_owner_transaction(&mut installed, parent_transaction);
    installed
        .install_soil_thermal_accepted_v2_from_beginning(
            &authoritative,
            parent.beginning_owner(),
            accepted,
            seals,
        )
        .expect("install aggregate resident");

    let retained_prepared = installed
        .prepare_next_soil_thermal_support_v2(1_800_000_000_000, 1_860_000_000_000)
        .expect("retained trial support");
    let retained_trial = unpublished_composed_trial(
        &retained_prepared,
        &installed.inner.lse_configuration,
        1_800_000_000_000,
        1_860_000_000_000,
        f64::from_bits(1),
        'c',
    );
    let retained_candidate =
        DirectSoilThermalCandidate::from_v2(retained_trial.clone()).expect("retained candidate");
    let aggregate = installed
        .prepare_base_soil_thermal_unpublished_aggregate_support_v2(
            &retained_candidate,
            1_860_000_000_000,
            1_920_000_000_000,
        )
        .expect("authenticated aggregate support");
    assert_eq!(
        aggregate.beginning_owner().support_start_ns,
        installed
            .soil_thermal_v2()
            .expect("installed resident")
            .owner()
            .support_start_ns,
    );
    assert_eq!(
        aggregate.beginning_owner().support_end_ns,
        1_920_000_000_000
    );
    let continuation = installed
        .prepare_soil_thermal_base_unpublished_continuation_v2(
            &aggregate,
            &retained_trial,
            retained_candidate.state_sha256(),
            1_860_000_000_000,
            1_920_000_000_000,
        )
        .expect("aggregate base continuation");
    assert_eq!(continuation.retained_trial(), &retained_trial);
    assert_eq!(continuation.child_support_start_ns(), 1_860_000_000_000);
    assert_eq!(continuation.child_support_end_ns(), 1_920_000_000_000);
    let snow_owner = ResourceOwnerId::try_new("aggregate-snow").expect("snow owner");
    let child_credit = continuation_credit(
        continuation.child_beginning_state(),
        1_860_000_000_000,
        1_920_000_000_000,
        'd',
    );
    let child_operands = continuation
        .child_top_boundary_operands_v2(&[child_credit], &snow_owner)
        .expect("aggregate child operands");
    let result = installed
        .advance_soil_thermal_unpublished_continuation_v2(&continuation, &child_operands)
        .expect("aggregate child result");
    let selected = result.physical_trial();
    let beginning = result.original_prepared().beginning_owner();
    let resident = installed.soil_thermal_v2().expect("installed V2 resident");
    assert_eq!(
        beginning.support_start_ns,
        resident.owner().support_start_ns
    );
    assert_eq!(beginning.support_end_ns, selected.support_end_ns());
    assert!(resident.owner().support_end_ns <= selected.support_start_ns());
    assert!(selected.unpublished_predecessor_trial_sha256().is_some());
    assert!(
        selected
            .accepted_predecessor_receipt_chain_sha256()
            .is_none()
    );
    let accepted = result
        .compose_accepted_outer_candidate(&installed.inner.lse_configuration)
        .expect("aggregate accepted outer candidate");
    installed
        .validate_soil_thermal_accepted_v2_from_unpublished_continuation(
            selected, &result, beginning, &accepted,
        )
        .expect("aggregate accepted selection validation");
    let mut predecessor_substitution = result.clone();
    predecessor_substitution.physical_trial = retained_trial.clone();
    assert!(
        installed
            .validate_soil_thermal_accepted_v2_from_unpublished_continuation(
                predecessor_substitution.physical_trial(),
                &predecessor_substitution,
                beginning,
                &accepted,
            )
            .is_err(),
        "predecessor trial substitution must fail closed",
    );

    let before = installed
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before aggregate poisons");
    for (child_start_ns, child_end_ns) in [(1_860_000_000_001, 1_920_000_000_000)] {
        assert!(
            installed
                .prepare_base_soil_thermal_unpublished_aggregate_support_v2(
                    &retained_candidate,
                    child_start_ns,
                    child_end_ns,
                )
                .is_err(),
            "aggregate support rebind must fail closed",
        );
    }
    assert_eq!(
        installed
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after aggregate poisons"),
        before,
        "aggregate construction and refusal must not install resident custody",
    );

    let seals = seal_soil_thermal_accepted_candidate_v2(beginning, &accepted)
        .expect("aggregate accepted seals");
    let authority = installed
        .authenticate_soil_thermal_unpublished_continuation_install_authority_v3(
            &installed, &result, beginning,
        )
        .expect("aggregate three-domain install authority");
    assert!(
        installed
            .authenticate_soil_thermal_unpublished_continuation_install_authority_v3(
                &installed,
                &predecessor_substitution,
                beginning,
            )
            .is_err(),
        "foreign aggregate predecessor custody must refuse authority",
    );
    let mut published = installed.clone();
    published
        .install_soil_thermal_accepted_v2_from_unpublished_continuation_v3(
            &installed,
            &result,
            beginning,
            authority,
            accepted.clone(),
            seals,
        )
        .expect("single aggregate final install");
    assert_eq!(
        published
            .soil_thermal_v2()
            .expect("published V2 resident")
            .owner(),
        &accepted.ending_owner,
    );
    assert_eq!(
        installed
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("unchanged authoritative resident"),
        before,
        "final install must not mutate the authoritative beginning",
    );
}

#[test]
fn unpublished_continuation_lineage_poison_families_have_stable_closure_reasons() {
    let (authoritative, _) = native_v2_shadow_for_parent('c');
    let original_prepared = authoritative
        .prepare_next_soil_thermal_support_v2(0, 180_000_000_000)
        .expect("original parent support");
    let transaction = original_prepared.beginning_owner().transaction_id;
    let prior_prepared = authoritative
        .prepare_soil_thermal_support_v2(transaction, 0, 60_000_000_000)
        .expect("prior child support");
    let original = original_prepared.beginning_owner();
    let prior = prior_prepared.beginning_owner();

    let mut immutable_poison = prior.clone();
    immutable_poison.contract_version += 1;
    let mut transaction_poison = prior.clone();
    transaction_poison.transaction_id = TransactionId(transaction.0 + 1);
    let mut receipt_poison = prior.clone();
    receipt_poison.receipt_chain_sha256 = digest('d');
    let mut support_start_poison = prior.clone();
    support_start_poison.support_start_ns = 1;
    let mut prior_end_poison = prior.clone();
    prior_end_poison.support_end_ns = 59_000_000_000;
    let outer_end_poison = original.clone();
    let outer_end = 180_000_000_001;
    let mut width_floor_poison = original.clone();
    let width_floor_end = 60_000_000_000 + openwepp_land_surface_energy::MINIMUM_SUPPORT_NS - 1;
    width_floor_poison.support_end_ns = width_floor_end;
    let mut width_order_poison = original.clone();
    width_order_poison.support_end_ns = 60_000_000_000;
    let mut width_order_prior = prior.clone();
    width_order_prior.support_end_ns = 120_000_000_000;

    let cases = [
        (
            "immutable identity",
            original,
            &immutable_poison,
            60_000_000_000,
            180_000_000_000,
            V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY,
        ),
        (
            "transaction lineage",
            original,
            &transaction_poison,
            60_000_000_000,
            180_000_000_000,
            V2_UNPUBLISHED_CONTINUATION_TRANSACTION_LINEAGE,
        ),
        (
            "receipt lineage",
            original,
            &receipt_poison,
            60_000_000_000,
            180_000_000_000,
            V2_UNPUBLISHED_CONTINUATION_TRANSACTION_LINEAGE,
        ),
        (
            "support start",
            original,
            &support_start_poison,
            60_000_000_000,
            180_000_000_000,
            V2_UNPUBLISHED_CONTINUATION_SUPPORT_START,
        ),
        (
            "prior end",
            original,
            &prior_end_poison,
            60_000_000_000,
            180_000_000_000,
            V2_UNPUBLISHED_CONTINUATION_PRIOR_END,
        ),
        (
            "outer end",
            &outer_end_poison,
            prior,
            60_000_000_000,
            outer_end,
            V2_UNPUBLISHED_CONTINUATION_OUTER_END,
        ),
        (
            "width floor",
            &width_floor_poison,
            prior,
            60_000_000_000,
            width_floor_end,
            V2_UNPUBLISHED_CONTINUATION_WIDTH,
        ),
        (
            "width ordering",
            &width_order_poison,
            &width_order_prior,
            120_000_000_000,
            60_000_000_000,
            V2_UNPUBLISHED_CONTINUATION_WIDTH,
        ),
    ];
    let before = authoritative
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before lineage poison vector");
    for (label, original, prior, child_start, child_end, reason) in cases {
        let actual =
            validate_unpublished_continuation_lineage(original, prior, child_start, child_end)
                .expect_err(label);
        assert_eq!(
            actual,
            DirectV9RealConsumerError::OwnerClosure(reason),
            "stable reason for {label}"
        );
        assert_eq!(
            authoritative
                .soil_thermal_resident()
                .canonical_active_owner_bytes()
                .expect("after lineage poison vector"),
            before,
            "lineage refusal for {label} must preserve resident custody"
        );
    }
}

#[test]
fn unpublished_continuation_refuses_substitution_support_receipt_and_carry_poisons() {
    let (authoritative, _) = native_v2_shadow_for_parent('1');
    let original = authoritative
        .prepare_next_soil_thermal_support_v2(0, 180_000_000_000)
        .expect("original parent support");
    let transaction = original.beginning_owner().transaction_id;
    let prior = authoritative
        .prepare_soil_thermal_support_v2(transaction, 0, 60_000_000_000)
        .expect("prior child support");
    let retained = unpublished_composed_trial(
        &original,
        &authoritative.inner.lse_configuration,
        0,
        60_000_000_000,
        f64::from_bits(1),
        '2',
    );
    let substituted = unpublished_composed_trial(
        &original,
        &authoritative.inner.lse_configuration,
        0,
        60_000_000_000,
        f64::from_bits(2),
        '3',
    );
    let before = authoritative
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before poison bytes");
    let rejects = |prior: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
                   trial: &openwepp_land_surface_energy::SoilThermalTrialStateV2,
                   expected: &Sha256Digest,
                   start: u128,
                   end: u128| {
        assert!(
            authoritative
                .prepare_soil_thermal_unpublished_continuation_v2(
                    &original, prior, trial, expected, start, end,
                )
                .is_err()
        );
        assert_eq!(
            authoritative
                .soil_thermal_resident()
                .canonical_active_owner_bytes()
                .expect("after poison bytes"),
            before
        );
    };
    rejects(
        &prior,
        &substituted,
        &retained.ending_state().state_sha256,
        60_000_000_000,
        120_000_000_000,
    );
    rejects(
        &prior,
        &retained,
        &retained.ending_state().state_sha256,
        0,
        60_000_000_000,
    );
    rejects(
        &prior,
        &retained,
        &retained.ending_state().state_sha256,
        60_000_000_001,
        120_000_000_001,
    );

    let wrong_support = authoritative
        .prepare_soil_thermal_support_v2(transaction, 60_000_000_000, 120_000_000_000)
        .expect("cross-support prepared");
    rejects(
        &wrong_support,
        &retained,
        &retained.ending_state().state_sha256,
        120_000_000_000,
        180_000_000_000,
    );

    let (foreign, _) = native_v2_shadow_for_parent('4');
    let foreign_original = foreign
        .prepare_next_soil_thermal_support_v2(0, 180_000_000_000)
        .expect("foreign original");
    let foreign_prior = foreign
        .prepare_soil_thermal_support_v2(
            foreign_original.beginning_owner().transaction_id,
            0,
            60_000_000_000,
        )
        .expect("foreign prior");
    let foreign_trial = unpublished_composed_trial(
        &foreign_original,
        &foreign.inner.lse_configuration,
        0,
        60_000_000_000,
        f64::from_bits(1),
        '5',
    );
    rejects(
        &foreign_prior,
        &foreign_trial,
        &foreign_trial.ending_state().state_sha256,
        60_000_000_000,
        120_000_000_000,
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
        assert!(value
            .install_soil_thermal_accepted_v2_from_beginning(source, beginning, accepted, seals,)
            .is_err());
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
fn numerical_coordinate_projection_retains_authority_and_never_installs() {
    let (authoritative, _) = native_v2_shadow_for_parent('6');
    let prepared = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("coordinate prepared beginning");
    let coordinates = prepared
        .beginning_owner()
        .state
        .ofes
        .iter()
        .flat_map(|ofe| {
            ofe.ordered_layers.iter().enumerate().map(|(index, layer)| {
                openwepp_land_surface_energy::SoilThermalUnpublishedCoordinateV2 {
                    ofe_id: ofe.ofe_id.clone(),
                    layer_id: layer.layer_id.clone(),
                    proposed_total_enthalpy_j_m2_ofe_ground: 17.0 + index as f64,
                    proposed_temperature_k: 275.0,
                }
            })
        })
        .collect::<Vec<_>>();
    let before = authoritative
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before coordinate projection");
    let projected = authoritative
        .project_soil_thermal_unpublished_coordinates_v2(&prepared, &coordinates)
        .expect("orchestrator coordinate projection");
    let trial = projected.v2().expect("native projected trial");
    assert_eq!(
        trial.transaction_id(),
        prepared.beginning_owner().transaction_id
    );
    assert_eq!(
        trial.accepted_predecessor_receipt_chain_sha256(),
        Some(&prepared.beginning_owner().receipt_chain_sha256)
    );
    assert!(trial.numerical_coordinate_authority_sha256().is_some());
    assert!(trial.numerical_coordinate_set_sha256().is_some());
    assert_eq!(
        authoritative
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after coordinate projection"),
        before,
        "private coordinate projection must not install"
    );
    let top_coordinates = prepared
        .beginning_owner()
        .state
        .ofes
        .iter()
        .enumerate()
        .map(
            |(index, ofe)| openwepp_land_surface_energy::SoilThermalUnpublishedCoordinateV2 {
                ofe_id: ofe.ofe_id.clone(),
                layer_id: ofe.ordered_layers[0].layer_id.clone(),
                proposed_total_enthalpy_j_m2_ofe_ground: 31.0 + index as f64,
                proposed_temperature_k: 276.0,
            },
        )
        .collect::<Vec<_>>();
    let top_projected = authoritative
        .project_soil_thermal_unpublished_top_layer_coordinates_v2(&prepared, &top_coordinates)
        .expect("orchestrator top-layer coordinate projection");
    for (beginning_ofe, ending_ofe) in prepared.beginning_owner().state.ofes.iter().zip(
        &top_projected
            .v2()
            .expect("top-layer trial")
            .ending_state()
            .ofes,
    ) {
        assert_eq!(
            &ending_ofe.ordered_layers[1..],
            &beginning_ofe.ordered_layers[1..],
            "orchestrator wrapper must retain lower layers bit-exact"
        );
    }
    assert_eq!(
        authoritative
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after top-layer projection"),
        before
    );

    let (foreign, _) = native_v2_shadow_for_parent('7');
    let foreign_prepared = foreign
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("foreign coordinate beginning");
    let foreign_coordinates = foreign_prepared
        .beginning_owner()
        .state
        .ofes
        .iter()
        .flat_map(|ofe| {
            ofe.ordered_layers.iter().enumerate().map(|(index, layer)| {
                openwepp_land_surface_energy::SoilThermalUnpublishedCoordinateV2 {
                    ofe_id: ofe.ofe_id.clone(),
                    layer_id: layer.layer_id.clone(),
                    proposed_total_enthalpy_j_m2_ofe_ground: 17.0 + index as f64,
                    proposed_temperature_k: 275.0,
                }
            })
        })
        .collect::<Vec<_>>();
    assert!(
        authoritative
            .project_soil_thermal_unpublished_coordinates_v2(
                &foreign_prepared,
                &foreign_coordinates,
            )
            .is_err(),
        "foreign state/carry/receipt authority must fail closed"
    );
    let foreign_top_coordinates = foreign_prepared
        .beginning_owner()
        .state
        .ofes
        .iter()
        .map(
            |ofe| openwepp_land_surface_energy::SoilThermalUnpublishedCoordinateV2 {
                ofe_id: ofe.ofe_id.clone(),
                layer_id: ofe.ordered_layers[0].layer_id.clone(),
                proposed_total_enthalpy_j_m2_ofe_ground: 31.0,
                proposed_temperature_k: 276.0,
            },
        )
        .collect::<Vec<_>>();
    assert!(
        authoritative
            .project_soil_thermal_unpublished_top_layer_coordinates_v2(
                &foreign_prepared,
                &foreign_top_coordinates,
            )
            .is_err(),
        "foreign top-layer authority must fail closed"
    );
    assert_eq!(
        authoritative
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after coordinate authority poison"),
        before
    );
}

fn v43_top_coordinate_candidate(
    authoritative: &DirectV10RealConsumerShadow,
    prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
) -> DirectSoilThermalCandidate {
    let coordinates = prepared
        .beginning_owner()
        .state
        .ofes
        .iter()
        .enumerate()
        .map(
            |(index, ofe)| openwepp_land_surface_energy::SoilThermalUnpublishedCoordinateV2 {
                ofe_id: ofe.ofe_id.clone(),
                layer_id: ofe.ordered_layers[0].layer_id.clone(),
                proposed_total_enthalpy_j_m2_ofe_ground: 41.0 + index as f64,
                proposed_temperature_k: 274.0 + index as f64,
            },
        )
        .collect::<Vec<_>>();
    authoritative
        .project_soil_thermal_unpublished_top_layer_coordinates_v2(prepared, &coordinates)
        .expect("V43 top coordinate projection")
}

#[test]
fn v43_projected_fixed_point_accepts_typed_coordinate_custody() {
    let (authoritative, _) = native_v2_shadow_for_parent('6');
    let prepared = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("V43 prepared support");
    let projected = v43_top_coordinate_candidate(&authoritative, &prepared);
    assert!(
        authoritative
            .inner
            .soil_thermal
            .validate_unpublished_fixed_point_v2(
                &authoritative.inner.lse_configuration,
                &projected,
                None,
                0,
                1_800_000_000_000,
            )
            .expect("typed projected fixed-point custody")
    );
}

#[test]
fn v43_projected_fixed_point_retains_base_reconstruction_byte_lock() {
    let (authoritative, _) = native_v2_shadow_for_parent('6');
    let prepared = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("V43 ordinary base support");
    let trial = unpublished_composed_trial(
        &prepared,
        &authoritative.inner.lse_configuration,
        0,
        1_800_000_000_000,
        f64::from_bits(1),
        'b',
    );
    let candidate = DirectSoilThermalCandidate::from_v2(trial).expect("ordinary base candidate");
    let before = authoritative
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before ordinary base validation");
    assert!(
        authoritative
            .inner
            .soil_thermal
            .validate_unpublished_fixed_point_v2(
                &authoritative.inner.lse_configuration,
                &candidate,
                None,
                0,
                1_800_000_000_000,
            )
            .expect("ordinary base reconstruction")
    );
    assert_eq!(
        authoritative
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after ordinary base validation"),
        before
    );
}

#[test]
fn v43_projected_fixed_point_refuses_erased_mixed_or_foreign_custody() {
    let (authoritative, _) = native_v2_shadow_for_parent('6');
    let prepared = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("V43 authoritative support");
    let projected = v43_top_coordinate_candidate(&authoritative, &prepared);
    assert!(
        authoritative
            .authenticate_soil_thermal_base_unpublished_result_v2(
                &prepared,
                projected.v2().expect("projected trial"),
                &[],
            )
            .is_err(),
        "projected custody must not be erased into ordinary base physics"
    );
    let (foreign, _) = native_v2_shadow_for_parent('7');
    let foreign_prepared = foreign
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("V43 foreign support");
    let foreign_projected = v43_top_coordinate_candidate(&foreign, &foreign_prepared);
    assert!(
        authoritative
            .inner
            .soil_thermal
            .validate_unpublished_fixed_point_v2(
                &authoritative.inner.lse_configuration,
                &foreign_projected,
                None,
                0,
                1_800_000_000_000,
            )
            .is_err()
    );
}

#[test]
fn v43_projected_fixed_point_refuses_support_receipt_authority_or_order_poison() {
    let (authoritative, _) = native_v2_shadow_for_parent('6');
    let prepared = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("V43 authoritative support");
    let projected = v43_top_coordinate_candidate(&authoritative, &prepared);
    assert!(
        !authoritative
            .inner
            .soil_thermal
            .validate_unpublished_fixed_point_v2(
                &authoritative.inner.lse_configuration,
                &projected,
                None,
                60_000_000_000,
                1_800_000_000_000,
            )
            .expect("wrong support is ineligible")
    );
    let mut missing = prepared
        .beginning_owner()
        .state
        .ofes
        .iter()
        .map(
            |ofe| openwepp_land_surface_energy::SoilThermalUnpublishedCoordinateV2 {
                ofe_id: ofe.ofe_id.clone(),
                layer_id: ofe.ordered_layers[0].layer_id.clone(),
                proposed_total_enthalpy_j_m2_ofe_ground: 41.0,
                proposed_temperature_k: 274.0,
            },
        )
        .collect::<Vec<_>>();
    missing.pop();
    assert!(
        authoritative
            .project_soil_thermal_unpublished_top_layer_coordinates_v2(&prepared, &missing)
            .is_err(),
        "missing or reordered coordinate custody must fail closed"
    );
}

#[test]
fn v43_projected_fixed_point_cannot_advance_accept_install_or_publish() {
    let (authoritative, _) = native_v2_shadow_for_parent('6');
    let prepared = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("V43 authoritative support");
    let projected = v43_top_coordinate_candidate(&authoritative, &prepared);
    let before = authoritative
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("before projected refusal");
    assert!(
        authoritative
            .prepare_soil_thermal_unpublished_physical_beginning_v2(
                &projected,
                None,
                0,
                1_800_000_000_000,
            )
            .is_err(),
        "numerical projection cannot become a sequential physical beginning"
    );
    assert!(
        openwepp_land_surface_energy::compose_soil_thermal_accepted_from_unpublished_v2(
            &prepared,
            projected.v2().expect("projected trial"),
            &[],
            &[],
        )
        .is_err(),
        "numerical projection cannot become an accepted owner candidate"
    );
    assert_eq!(
        authoritative
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("after projected refusal"),
        before,
        "no refusal may install or publish the private projection"
    );
}

#[test]
fn v44_numerical_projection_selects_resident_v8_and_rejects_double_use() {
    let (authoritative, _) = native_v2_shadow_for_parent('6');
    let prepared = authoritative
        .prepare_next_soil_thermal_support_v2(0, 1_800_000_000_000)
        .expect("V44 prepared support");
    let projected = v43_top_coordinate_candidate(&authoritative, &prepared);
    let resident = &authoritative.inner.soil_thermal;
    assert!(
        !resident
            .read_view()
            .physically_equals(projected.read_view())
    );

    let selected = crate::v9_real_consumer_shadow::direct_v9_select_v8_soil_beginning_v44(
        resident,
        Some(&projected),
        true,
        crate::v9_real_consumer_shadow::DirectV9V8SoilBeginningSourceV44::ResidentAuthenticatedOwner,
        true,
    )
    .expect("numerical fixed point keeps resident V8 beginning");
    assert!(selected.physically_equals(resident.read_view()));
    assert!(!selected.physically_equals(projected.read_view()));

    assert!(crate::v9_real_consumer_shadow::direct_v9_select_v8_soil_beginning_v44(
        resident,
        Some(&projected),
        true,
        crate::v9_real_consumer_shadow::DirectV9V8SoilBeginningSourceV44::UnpublishedPhysicalCandidate,
        true,
    )
    .is_err());
    assert!(crate::v9_real_consumer_shadow::direct_v9_select_v8_soil_beginning_v44(
        resident,
        None,
        true,
        crate::v9_real_consumer_shadow::DirectV9V8SoilBeginningSourceV44::ResidentAuthenticatedOwner,
        true,
    )
    .is_err());
    assert!(crate::v9_real_consumer_shadow::direct_v9_select_v8_soil_beginning_v44(
        resident,
        Some(&projected),
        false,
        crate::v9_real_consumer_shadow::DirectV9V8SoilBeginningSourceV44::ResidentAuthenticatedOwner,
        true,
    )
    .is_err());

    let ordinary_candidate = crate::v9_real_consumer_shadow::direct_v9_select_v8_soil_beginning_v44(
        resident,
        Some(&projected),
        false,
        crate::v9_real_consumer_shadow::DirectV9V8SoilBeginningSourceV44::UnpublishedPhysicalCandidate,
        true,
    )
    .expect("ordinary unpublished physical candidate posture");
    assert!(ordinary_candidate.physically_equals(projected.read_view()));
}

include!("v10_soil_thermal_v2_resident_install_tests.rs");
