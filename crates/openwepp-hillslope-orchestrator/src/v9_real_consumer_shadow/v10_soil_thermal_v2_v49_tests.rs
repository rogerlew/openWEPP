use super::*;
use super::direct_v10_soil_thermal_v2_tests::{
    accepted_bundle, align_complete_owner_transaction, native_v2_shadow_for_parent,
};
use crate::v9_real_consumer_shadow::v11_covered::owner_finalization::install_v2_soil_from_authenticated_prepared_beginning_v2;

fn v49_digest(byte: char) -> Sha256Digest {
    Sha256Digest::try_new(byte.to_string().repeat(64)).expect("V49 digest")
}

pub(crate) fn migrate_shadow_to_native_v2_for_parent_test(
    v1_shadow: DirectV10RealConsumerShadow,
    parent_duration_ns: u128,
) -> DirectV10RealConsumerShadow {
    let current_transaction = TransactionId(v1_shadow.vegetation_state.0.last_transaction_id);
    let predecessor_transaction = TransactionId(
        current_transaction
            .0
            .checked_sub(1)
            .expect("V50 native-V2 parent predecessor"),
    );
    let migrated = openwepp_land_surface_energy::migrate_soil_thermal_v1_to_v2(
        v1_shadow
            .inner
            .soil_thermal
            .v1()
            .expect("V50 native-V2 parent V1 beginning"),
        openwepp_land_surface_energy::SoilThermalV2MigrationIdentity {
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
            run_id: "v50-native-v2-real-finalizer".to_owned(),
            transaction_id: predecessor_transaction,
            support_start_ns: 0,
            support_end_ns: parent_duration_ns,
            receipt_chain_sha256: v49_digest('a'),
        },
    )
    .expect("V50 checked native-V2 parent migration");
    let prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
        &migrated,
        current_transaction,
        0,
        parent_duration_ns,
    )
    .expect("V50 native-V2 parent support");
    let seals = openwepp_land_surface_energy::seal_soil_thermal_receipt_free_owner_v2(&prepared)
        .expect("V50 native-V2 parent receipt-free seals");
    DirectV10RealConsumerShadow::try_new_v2(
        v1_shadow.vegetation_configuration.clone(),
        v1_shadow.vegetation_state.clone(),
        v1_shadow.inner.vegetation_owner_id.clone(),
        v1_shadow.lse_configuration.clone(),
        v1_shadow.lse_state.clone(),
        v1_shadow.inner.surface_configuration.clone(),
        v1_shadow.inner.layer_maps.clone(),
        prepared,
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
    .expect("V50 native-V2 parent shadow")
}

fn v49_r124_three_domain_fixture(
    receipt_chain: char,
) -> (
    DirectV10RealConsumerShadow,
    DirectV10RealConsumerShadow,
    openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
    DirectV10SoilThermalResidentV2,
    SoilThermalAcceptedCandidateV2,
    SoilThermalOrchestratorSealsV2,
) {
    let (mut candidate, first_prepared) = native_v2_shadow_for_parent(receipt_chain);
    let (first_accepted, first_seals) = accepted_bundle(
        &first_prepared,
        &candidate.inner.lse_configuration,
        f64::from_bits(1),
        receipt_chain,
    );
    let first_target = first_accepted.ending_owner.transaction_id;
    align_complete_owner_transaction(&mut candidate, first_target);
    candidate
        .install_soil_thermal_accepted_v2(
            first_prepared.beginning_owner(),
            first_accepted,
            first_seals,
        )
        .expect("V49 installed receipt-free first support");

    let prepared_42 = candidate
        .prepare_next_soil_thermal_support_v2(60_000_000_000, 1_860_000_000_000)
        .expect("V49 prepared source-42 predecessor");
    let (accepted_42, seals_42) = accepted_bundle(
        &prepared_42,
        &candidate.inner.lse_configuration,
        f64::from_bits(2),
        receipt_chain,
    );
    let beginning_41 = candidate.clone();
    let authority_42 = candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
            &beginning_41,
            prepared_42.beginning_owner(),
        )
        .expect("V49 exact target-42 authority");
    candidate
        .install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
            &beginning_41,
            prepared_42.beginning_owner(),
            authority_42,
            accepted_42,
            seals_42,
        )
        .expect("V49 installed target-42 predecessor");
    align_complete_owner_transaction(&mut candidate, TransactionId(42));

    let prepared_43 = candidate
        .prepare_next_soil_thermal_support_v2(1_860_000_000_000, 1_920_000_000_000)
        .expect("V49 prepared exact resident-43 support");
    let (accepted_43, seals_43) = accepted_bundle(
        &prepared_43,
        &candidate.inner.lse_configuration,
        f64::from_bits(3),
        receipt_chain,
    );
    let beginning_42 = candidate.clone();
    let authority_43 = candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
            &beginning_42,
            prepared_43.beginning_owner(),
        )
        .expect("V49 exact target-43 authority");
    candidate
        .install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
            &beginning_42,
            prepared_43.beginning_owner(),
            authority_43,
            accepted_43,
            seals_43,
        )
        .expect("V49 installed exact resident 43");

    let authoritative_beginning = candidate.clone();
    let resident = authoritative_beginning
        .soil_thermal_v2()
        .expect("V49 exact R124 resident");
    assert_eq!(resident.owner.transaction_id, TransactionId(43));
    assert_eq!(resident.owner.support_start_ns, 1_860_000_000_000);
    assert_eq!(resident.owner.support_end_ns, 1_920_000_000_000);
    assert_eq!(
        TransactionId(candidate.vegetation_state.0.last_transaction_id),
        TransactionId(42),
    );
    let prepared = candidate
        .prepare_next_soil_thermal_support_v2(1_920_000_000_000, 2_040_000_000_000)
        .expect("V49 exact R124 prepared target");
    assert_eq!(prepared.beginning_owner().transaction_id, TransactionId(44));
    assert_eq!(
        prepared.beginning_owner().expected_predecessor_transaction_id,
        Some(TransactionId(43)),
    );
    let (accepted, seals) = accepted_bundle(
        &prepared,
        &candidate.inner.lse_configuration,
        f64::from_bits(4),
        receipt_chain,
    );
    let ending = resident
        .accepted(prepared.beginning_owner(), accepted.clone(), seals.clone())
        .expect("V49 exact R124 accepted ending");
    (
        candidate,
        authoritative_beginning,
        prepared,
        ending,
        accepted,
        seals,
    )
}

fn v49_assert_rollback(
    candidate: &DirectV10RealConsumerShadow,
    before: &DirectV10RealConsumerShadow,
) {
    assert_eq!(candidate, before, "V49 refusal must preserve the full shadow");
    assert_eq!(
        candidate
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("V49 rollback soil bytes"),
        before
            .soil_thermal_resident()
            .canonical_active_owner_bytes()
            .expect("V49 expected rollback soil bytes"),
    );
    assert_eq!(candidate.vegetation_state, before.vegetation_state);
    assert_eq!(candidate.lse_state, before.lse_state);
    assert_eq!(candidate.inner.biogeochemistry, before.inner.biogeochemistry);
    assert_eq!(
        candidate.accepted_publication_history,
        before.accepted_publication_history,
    );
}

#[test]
fn v49_r124_three_domain_prepared_install_succeeds() {
    let (mut candidate, authoritative, prepared, ending, accepted, seals) =
        v49_r124_three_domain_fixture('1');
    let publication_before = candidate.accepted_publication_history.clone();
    let authority = candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
            &authoritative,
            prepared.beginning_owner(),
        )
        .expect("source42/resident43/target44 authority");
    assert_eq!(
        authority.physical_transaction_authority.source_transaction_id,
        TransactionId(42),
    );
    assert_eq!(
        authority
            .physical_transaction_authority
            .soil_thermal_transaction_id,
        TransactionId(44),
    );
    assert_eq!(authority.authoritative_resident.owner.transaction_id, TransactionId(43));
    install_v2_soil_from_authenticated_prepared_beginning_v1(
        &mut candidate,
        &authoritative,
        prepared.beginning_owner(),
        accepted,
        seals,
    )
        .expect("exact R124 three-domain install");
    assert_eq!(candidate.soil_thermal_v2().expect("installed V2"), &ending);
    assert_eq!(candidate.accepted_publication_history, publication_before);
    assert_eq!(candidate.vegetation_state.0.last_transaction_id, 42);
    assert_eq!(candidate.lse_state.0.last_accepted_transaction_id, Some(TransactionId(42)));
    assert_eq!(candidate.inner.biogeochemistry.last_transaction_id, 42);
}

#[test]
fn v49_repeated_same_parent_soil_successors_remain_exact() {
    let (mut candidate, authoritative, prepared, _, accepted, seals) =
        v49_r124_three_domain_fixture('2');
    let authority = candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
            &authoritative,
            prepared.beginning_owner(),
        )
        .expect("target44 authority");
    candidate
        .install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
            &authoritative,
            prepared.beginning_owner(),
            authority,
            accepted,
            seals,
        )
        .expect("installed target44");
    let beginning_44 = candidate.clone();
    let next = candidate
        .prepare_next_soil_thermal_support_v2(2_040_000_000_000, 2_160_000_000_000)
        .expect("prepared further same-parent successor");
    assert_eq!(next.beginning_owner().transaction_id, TransactionId(45));
    assert_eq!(
        next.beginning_owner().expected_predecessor_transaction_id,
        Some(TransactionId(44)),
    );
    let (accepted_45, seals_45) = accepted_bundle(
        &next,
        &candidate.inner.lse_configuration,
        f64::from_bits(5),
        '2',
    );
    let authority_45 = candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
            &beginning_44,
            next.beginning_owner(),
        )
        .expect("source42/resident44/target45 authority");
    candidate
        .install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
            &beginning_44,
            next.beginning_owner(),
            authority_45,
            accepted_45,
            seals_45,
        )
        .expect("installed target45 under unchanged source42");
    assert_eq!(candidate.soil_thermal_v2().expect("target45").owner.transaction_id, TransactionId(45));
    assert_eq!(candidate.vegetation_state.0.last_transaction_id, 42);
}

#[test]
fn v49_prepared_install_authority_refuses_resident_and_prepared_substitution() {
    let (mut candidate, authoritative, prepared, _, accepted, seals) =
        v49_r124_three_domain_fixture('3');
    let authority = candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
            &authoritative,
            prepared.beginning_owner(),
        )
        .expect("exact authority");
    let before = candidate.clone();
    for source_poison in 0..3 {
        let mut changed = candidate.clone();
        match source_poison {
            0 => changed.vegetation_state.0.last_transaction_id = 41,
            1 => changed.lse_state.0.last_accepted_transaction_id = Some(TransactionId(41)),
            _ => changed.inner.biogeochemistry.last_transaction_id = 41,
        }
        assert!(
            changed
                .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
                    &authoritative,
                    prepared.beginning_owner(),
                )
                .is_err(),
            "source owner {source_poison} divergence must refuse",
        );
    }
    let mut jointly_rebased = candidate.clone();
    align_complete_owner_transaction(&mut jointly_rebased, TransactionId(41));
    let jointly_rebased_before = jointly_rebased.clone();
    assert!(
        jointly_rebased
            .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
                &authoritative,
                prepared.beginning_owner(),
            )
            .is_err(),
        "jointly rebased outer owners must not replace the authoritative source",
    );
    v49_assert_rollback(&jointly_rebased, &jointly_rebased_before);
    let mut jointly_rebased_install = candidate.clone();
    align_complete_owner_transaction(&mut jointly_rebased_install, TransactionId(41));
    let jointly_rebased_install_before = jointly_rebased_install.clone();
    assert!(
        jointly_rebased_install
            .install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
                &authoritative,
                prepared.beginning_owner(),
                authority.clone(),
                accepted.clone(),
                seals.clone(),
            )
            .is_err(),
        "exact authority must not install into jointly rebased outer owners",
    );
    v49_assert_rollback(&jointly_rebased_install, &jointly_rebased_install_before);

    let authoritative_resident = authoritative
        .soil_thermal_v2()
        .expect("exact authoritative resident")
        .clone();
    let mut resident_transaction = authoritative_resident.clone();
    resident_transaction.owner.transaction_id = TransactionId(42);
    let mut resident_support = authoritative_resident.clone();
    resident_support.owner.support_start_ns -= 60_000_000_000;
    let mut resident_receipt = authoritative_resident.clone();
    resident_receipt.owner.receipt_chain_sha256 = v49_digest('c');
    let mut resident_state = authoritative_resident.clone();
    resident_state.owner.state.ofes[0].ordered_layers[0].temperature_k = f64::from_bits(
        resident_state.owner.state.ofes[0].ordered_layers[0]
            .temperature_k
            .to_bits()
            + 1,
    );
    resident_state.owner.state.reseal().expect("resident state poison reseal");
    let mut resident_layer = authoritative_resident.clone();
    resident_layer.owner.state.ofes[0].ordered_layers[0].last_accepted_transaction_id =
        Some(TransactionId(42));
    resident_layer.owner.state.reseal().expect("resident layer poison reseal");
    let mut resident_custody = authoritative_resident.clone();
    resident_custody
        .latest_accepted
        .as_mut()
        .expect("resident accepted custody")
        .predecessor
        .receipt_chain_sha256 = v49_digest('d');
    let mut resident_seal = authoritative_resident;
    resident_seal
        .latest_accepted
        .as_mut()
        .expect("resident accepted seal")
        .seals
        .orchestrator_seal_sha256 = v49_digest('e');
    for (label, resident_poison) in [
        ("transaction", resident_transaction),
        ("support", resident_support),
        ("receipt", resident_receipt),
        ("state", resident_state),
        ("layer", resident_layer),
        ("latest-accepted-custody", resident_custody),
        ("seal", resident_seal),
    ] {
        let mut changed_authoritative = authoritative.clone();
        changed_authoritative.inner.soil_thermal = DirectSoilThermalResident::V2(resident_poison);
        let candidate_before = candidate.clone();
        assert!(
            candidate
                .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
                    &changed_authoritative,
                    prepared.beginning_owner(),
                )
                .is_err(),
            "authoritative resident {label} poison must refuse",
        );
        v49_assert_rollback(&candidate, &candidate_before);
    }
    let (_, foreign, _, _, _, _) = v49_r124_three_domain_fixture('4');
    assert!(
        candidate
            .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
                &foreign,
                prepared.beginning_owner(),
            )
            .is_err(),
    );
    let mut predecessor = prepared.beginning_owner().clone();
    predecessor.expected_predecessor_transaction_id = Some(TransactionId(42));
    let mut target = prepared.beginning_owner().clone();
    target.transaction_id = TransactionId(45);
    let mut start = prepared.beginning_owner().clone();
    start.support_start_ns -= 60_000_000_000;
    let mut end = prepared.beginning_owner().clone();
    end.support_end_ns += 60_000_000_000;
    let mut receipt = prepared.beginning_owner().clone();
    receipt.receipt_chain_sha256 = v49_digest('5');
    let mut state = prepared.beginning_owner().clone();
    state.state.ofes[0].ordered_layers[0].temperature_k = f64::from_bits(
        state.state.ofes[0].ordered_layers[0].temperature_k.to_bits() + 1,
    );
    state.state.reseal().expect("resealed prepared state poison");
    let mut layer = prepared.beginning_owner().clone();
    layer.state.ofes[0].ordered_layers[0].last_accepted_transaction_id =
        Some(TransactionId(41));
    layer.state.reseal().expect("resealed prepared layer poison");
    for (label, poison) in [
        ("predecessor", predecessor),
        ("target", target),
        ("support-start", start),
        ("support-end", end),
        ("receipt", receipt),
        ("state", state),
        ("layer", layer),
    ] {
        assert!(
            candidate
                .install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
                    &authoritative,
                    &poison,
                    authority.clone(),
                    accepted.clone(),
                    seals.clone(),
                )
                .is_err(),
            "{label} prepared substitution must refuse",
        );
        v49_assert_rollback(&candidate, &before);
    }
}

#[test]
fn v49_prepared_install_authority_refuses_accepted_and_authority_substitution() {
    let (mut candidate, authoritative, prepared, _, accepted, seals) =
        v49_r124_three_domain_fixture('6');
    let authority = candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
            &authoritative,
            prepared.beginning_owner(),
        )
        .expect("exact authority");
    let before = candidate.clone();
    let mut target = accepted.clone();
    target.ending_owner.transaction_id = TransactionId(45);
    let mut predecessor = accepted.clone();
    predecessor.ending_owner.expected_predecessor_transaction_id = Some(TransactionId(42));
    let mut support = accepted.clone();
    support.ending_owner.support_end_ns += 60_000_000_000;
    let mut receipt = accepted.clone();
    receipt.ending_owner.receipt_chain_sha256 = v49_digest('7');
    let mut state = accepted.clone();
    state.ending_owner.state.ofes[0].ordered_layers[0].temperature_k = f64::from_bits(
        state.ending_owner.state.ofes[0].ordered_layers[0]
            .temperature_k
            .to_bits()
            + 1,
    );
    state.ending_owner.state.reseal().expect("resealed state poison");
    let mut layer = accepted.clone();
    layer.ending_owner.state.ofes[0].ordered_layers[0].last_accepted_transaction_id =
        Some(TransactionId(45));
    layer.ending_owner.state.reseal().expect("resealed layer poison");
    let mut seal = seals.clone();
    seal.orchestrator_seal_sha256 = v49_digest('8');
    for (label, accepted_poison, seal_poison) in [
        ("target", target, seals.clone()),
        ("predecessor", predecessor, seals.clone()),
        ("support", support, seals.clone()),
        ("receipt", receipt, seals.clone()),
        ("state", state, seals.clone()),
        ("layer", layer, seals.clone()),
        ("seal", accepted.clone(), seal),
    ] {
        assert!(
            candidate
                .install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
                    &authoritative,
                    prepared.beginning_owner(),
                    authority.clone(),
                    accepted_poison,
                    seal_poison,
                )
                .is_err(),
            "{label} accepted substitution must refuse",
        );
        v49_assert_rollback(&candidate, &before);
    }
    let mut foreign_source = authority.clone();
    foreign_source.physical_transaction_authority =
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            TransactionId(41),
            TransactionId(44),
        )
        .expect("foreign source authority");
    let mut foreign_target = authority.clone();
    foreign_target.physical_transaction_authority =
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            TransactionId(42),
            TransactionId(45),
        )
        .expect("foreign target authority");
    let mut resident = authority.clone();
    resident.authoritative_resident.owner.transaction_id = TransactionId(42);
    let mut resident_custody = authority.clone();
    resident_custody
        .authoritative_resident
        .latest_accepted
        .as_mut()
        .expect("accepted resident custody")
        .predecessor
        .receipt_chain_sha256 = v49_digest('a');
    let mut authority_prepared = authority;
    authority_prepared.prepared_beginning.receipt_chain_sha256 = v49_digest('b');
    for (label, authority_poison) in [
        ("source", foreign_source),
        ("target", foreign_target),
        ("resident", resident),
        ("resident-custody", resident_custody),
        ("prepared", authority_prepared),
    ] {
        assert!(
            candidate
                .install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
                    &authoritative,
                    prepared.beginning_owner(),
                    authority_poison,
                    accepted.clone(),
                    seals.clone(),
                )
                .is_err(),
            "{label} authority substitution must refuse",
        );
        v49_assert_rollback(&candidate, &before);
    }
}

#[test]
fn v49_prepared_install_rolls_back_noops_and_never_publishes() {
    let (mut candidate, authoritative, prepared, ending, accepted, seals) =
        v49_r124_three_domain_fixture('9');
    let before = candidate.clone();
    assert!(
        candidate
            .install_soil_thermal_accepted_v2(
                prepared.beginning_owner(),
                accepted.clone(),
                seals.clone(),
            )
            .is_err(),
        "generic missing-authority split must refuse",
    );
    assert_eq!(candidate, before);
    for pass in 0..2 {
        install_v2_soil_from_authenticated_prepared_beginning_v1(
            &mut candidate,
            &authoritative,
            prepared.beginning_owner(),
            accepted.clone(),
            seals.clone(),
        )
            .unwrap_or_else(|error| panic!("V49 install/no-op pass {pass}: {error}"));
    }
    assert_eq!(candidate.soil_thermal_v2().expect("V49 no-op resident"), &ending);
    assert_eq!(
        candidate.accepted_publication_history,
        before.accepted_publication_history,
    );
    assert_eq!(candidate.vegetation_state, before.vegetation_state);
    assert_eq!(candidate.lse_state, before.lse_state);
    assert_eq!(candidate.inner.biogeochemistry, before.inner.biogeochemistry);
}

#[test]
fn v50_mixed_beginning_uses_exact_envelope_source() {
    let (mut captured_beginning, _) = native_v2_shadow_for_parent('f');
    let first_prepared = captured_beginning
        .prepare_soil_thermal_support_v2(TransactionId(41), 0, 1_800_000_000_000)
        .expect("V50 exact full-parent resident41 support");
    let (first_accepted, first_seals) = accepted_bundle(
        &first_prepared,
        &captured_beginning.inner.lse_configuration,
        f64::from_bits(1),
        'f',
    );
    align_complete_owner_transaction(
        &mut captured_beginning,
        first_accepted.ending_owner.transaction_id,
    );
    captured_beginning
        .install_soil_thermal_accepted_v2(
            first_prepared.beginning_owner(),
            first_accepted,
            first_seals,
        )
        .expect("V50 accepted resident41 custody");
    captured_beginning.vegetation_state.0.last_transaction_id = 41;
    captured_beginning.lse_state.0.last_accepted_transaction_id = Some(TransactionId(40));
    captured_beginning.inner.biogeochemistry.last_transaction_id = 41;
    assert_eq!(
        captured_beginning
            .soil_thermal_v2()
            .expect("V50 captured soil resident")
            .owner
            .transaction_id,
        TransactionId(41),
    );
    let prepared = captured_beginning
        .prepare_next_soil_thermal_support_v2(1_800_000_000_000, 1_860_000_000_000)
        .expect("V50 exact resident41 to target42 prepared support");
    assert_eq!(
        prepared.beginning_owner().expected_predecessor_transaction_id,
        Some(TransactionId(41)),
    );
    assert_eq!(prepared.beginning_owner().transaction_id, TransactionId(42));
    let (accepted, seals) = accepted_bundle(
        &prepared,
        &captured_beginning.inner.lse_configuration,
        f64::from_bits(2),
        'f',
    );
    let ending = captured_beginning
        .soil_thermal_v2()
        .expect("V50 exact resident41")
        .accepted(prepared.beginning_owner(), accepted.clone(), seals.clone())
        .expect("V50 exact target42 ending");

    let authoritative_beginning = captured_beginning.clone();
    let mut candidate = captured_beginning;
    align_complete_owner_transaction(&mut candidate, TransactionId(42));
    assert_eq!(
        direct_soil_thermal_complete_source_transaction_v2(&candidate)
            .expect("V50 captured envelope ending source"),
        TransactionId(42),
    );
    let expected_non_soil_ending = candidate.clone();
    let transition = DirectSoilThermalOuterOwnerTransitionAuthorityV2 {
        authenticated_transition: DirectSoilThermalOuterOwnerTransitionSourceV2::TestOnly {
            source_transaction_id: TransactionId(42),
            expected_non_soil_ending: Box::new(expected_non_soil_ending),
        },
    };
    let publication_before = candidate.accepted_publication_history.clone();
    for pass in 0..2 {
        install_v2_soil_from_authenticated_prepared_beginning_v2(
            &mut candidate,
            &authoritative_beginning,
            prepared.beginning_owner(),
            transition.clone(),
            accepted.clone(),
            seals.clone(),
        )
        .unwrap_or_else(|error| {
            panic!("V50 exact R129 mixed beginning install/no-op pass {pass}: {error}")
        });
    }
    assert_eq!(candidate.soil_thermal_v2().expect("V50 installed V2"), &ending);
    assert_eq!(candidate.vegetation_state.0.last_transaction_id, 42);
    assert_eq!(
        candidate.lse_state.0.last_accepted_transaction_id,
        Some(TransactionId(42)),
    );
    assert_eq!(candidate.inner.biogeochemistry.last_transaction_id, 42);
    assert_eq!(candidate.accepted_publication_history, publication_before);
}

#[test]
fn v50_envelope_source_and_candidate_owner_poisons_refuse() {
    let (candidate, authoritative, prepared, _, accepted, seals) =
        v49_r124_three_domain_fixture('e');
    let before = candidate.clone();
    let expected_non_soil_ending = candidate.clone();
    let transition = DirectSoilThermalOuterOwnerTransitionAuthorityV2 {
        authenticated_transition: DirectSoilThermalOuterOwnerTransitionSourceV2::TestOnly {
            source_transaction_id: TransactionId(42),
            expected_non_soil_ending: Box::new(expected_non_soil_ending.clone()),
        },
    };

    let foreign_source = DirectSoilThermalOuterOwnerTransitionAuthorityV2 {
        authenticated_transition: DirectSoilThermalOuterOwnerTransitionSourceV2::TestOnly {
            source_transaction_id: TransactionId(41),
            expected_non_soil_ending: Box::new(expected_non_soil_ending.clone()),
        },
    };
    assert!(candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v4(
            &authoritative,
            prepared.beginning_owner(),
            &foreign_source,
        )
        .is_err(), "foreign envelope source anchor must refuse");
    for owner_poison in 0..3 {
        let mut changed = candidate.clone();
        match owner_poison {
            0 => changed.vegetation_state.0.last_transaction_id = 41,
            1 => changed.lse_state.0.last_accepted_transaction_id = Some(TransactionId(41)),
            _ => changed.inner.biogeochemistry.last_transaction_id = 41,
        }
        let changed_before = changed.clone();
        assert!(changed
                .authenticate_soil_thermal_prepared_beginning_install_authority_v4(
                    &authoritative,
                    prepared.beginning_owner(),
                    &transition,
                ).is_err(),
            "individual candidate source owner {owner_poison} poison must refuse",
        );
        v49_assert_rollback(&changed, &changed_before);
    }

    let mut jointly_rebased_candidate = candidate.clone();
    align_complete_owner_transaction(&mut jointly_rebased_candidate, TransactionId(41));
    assert!(jointly_rebased_candidate
            .authenticate_soil_thermal_prepared_beginning_install_authority_v4(
                &authoritative,
                prepared.beginning_owner(),
                &transition,
            ).is_err(),
        "joint candidate/envelope rebase must not replace the exact source anchor",
    );

    let mut foreign_same_transaction_ending = expected_non_soil_ending;
    foreign_same_transaction_ending
        .vegetation_configuration
        .configuration_sha256 = "f".repeat(64);
    let foreign_ending = DirectSoilThermalOuterOwnerTransitionAuthorityV2 {
        authenticated_transition: DirectSoilThermalOuterOwnerTransitionSourceV2::TestOnly {
            source_transaction_id: TransactionId(42),
            expected_non_soil_ending: Box::new(foreign_same_transaction_ending),
        },
    };
    assert!(candidate
            .authenticate_soil_thermal_prepared_beginning_install_authority_v4(
                &authoritative,
                prepared.beginning_owner(),
                &foreign_ending,
            ).is_err(),
        "same-transaction foreign reconstructed ending must refuse",
    );

    let substituted_transition = foreign_source;
    let mut attempted = candidate.clone();
    assert!(
        attempted
            .install_soil_thermal_accepted_v2_from_authenticated_beginning_v4(
                &authoritative,
                prepared.beginning_owner(),
                substituted_transition,
                candidate
                    .authenticate_soil_thermal_prepared_beginning_install_authority_v4(
                        &authoritative,
                        prepared.beginning_owner(),
                        &transition,
                    )
                    .expect("V50 exact prepared install authority"),
                accepted,
                seals,
            )
            .is_err(),
        "substituted opaque transition must refuse",
    );
    v49_assert_rollback(&attempted, &before);
}
