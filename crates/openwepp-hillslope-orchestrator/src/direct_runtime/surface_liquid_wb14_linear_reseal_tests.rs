use super::*;
use openwepp_land_surface_energy::OfeId;

fn identity_fixture() -> DirectWb14ImmutableIdentityV1 {
    DirectWb14ImmutableIdentityV1 {
        schema_sha256: [41; 32],
        ofe_id_sha256: [12; 32],
        production_lane_id: 4,
        surface_liquid_configuration_sha256: [42; 32],
        wb14_configuration_sha256: [43; 32],
        wb14_model_definition_sha256: [44; 32],
        effective_conductivity_m_s_bits: 1.1e-7_f64.to_bits(),
        matric_potential_m_bits: 0.12_f64.to_bits(),
        storage_capacity_m_bits: 0.02_f64.to_bits(),
    }
}

fn parent_fixture_with_identity(
    identity: DirectWb14ImmutableIdentityV1,
) -> DirectWb14ParentIntervalV1 {
    let cursor = DirectWb14PersistentCursorV1 {
        day_index: 3,
        next_interval_index: 7,
        cumulative_supply_m: 0.002,
        cumulative_infiltration_m: 0.001,
    };
    DirectWb14ParentIntervalV1::begin(
        wb14_parent_authority_v1(
            [29; 32],
            10_000_000_000_000,
            11_800_000_000_000,
            [17; 32],
            cursor,
            identity,
        )
        .expect("authority"),
        cursor,
    )
    .expect("valid parent")
}

fn child_inputs(
    parent: &DirectWb14ParentIntervalV1,
    duration_s: f64,
) -> DirectWb14ContinuationIntervalInputs {
    DirectWb14ContinuationIntervalInputs {
        cumulative_supply_m: parent.working().cumulative_supply_m,
        cumulative_infiltration_m: parent.working().cumulative_infiltration_m,
        interval_supply_m: 0.000_01,
        interval_duration_s: duration_s,
        effective_conductivity_m_s: 1.1e-7,
        matric_potential_m: 0.12,
        storage_capacity_m: 0.02,
    }
}

fn bound_parent_with_child_count(
    identity: DirectWb14ImmutableIdentityV1,
    child_count: usize,
) -> DirectWb14ParentIntervalV1 {
    let mut parent = parent_fixture_with_identity(identity);
    let duration_s = 1_800_u64 / u64::try_from(child_count).expect("bounded child count");
    for index in 0..child_count {
        let start = parent.working().accepted_until_ns;
        let end = start + u128::from(duration_s) * 1_000_000_000;
        let slab_byte = u8::try_from(index + 1).expect("bounded slab byte");
        parent = parent
            .accept_child_transitions_with_slab(
                parent.working().next_child_ordinal,
                start,
                end,
                parent.working().receipt_chain_sha256,
                [slab_byte; 32],
                [61; 32],
                [62; 32],
                [63; 32],
                duration_s as f64,
                &[child_inputs(&parent, duration_s as f64)],
            )
            .expect("accepted bound child")
            .0;
    }
    parent
}

#[test]
fn linear_final_slab_reseal_matches_reference_and_counts_real_work() {
    for child_count in [1, 30] {
        let source = bound_parent_with_child_count(identity_fixture(), child_count);
        let reference = source
            .rebind_final_accepted_slab_reference_for_test([91; 32])
            .expect("test-only reference reseal");

        reset_wb14_reseal_audit();
        let actual = source
            .rebind_final_accepted_slab([91; 32])
            .expect("linear reseal");
        assert_eq!(
            wb14_reseal_audit(),
            DirectWb14ResealAuditV1 {
                input_validations: 1,
                child_traversals: child_count,
                final_validations: 1,
            }
        );
        assert_eq!(actual, reference);
        assert_eq!(
            serde_json::to_vec(&actual).expect("linear canonical bytes"),
            serde_json::to_vec(&reference).expect("reference canonical bytes")
        );
    }
}

#[test]
fn linear_final_slab_reseal_failure_is_transactional_for_zero_one_and_many_children() {
    for child_count in [0, 1, 30] {
        let source = if child_count == 0 {
            parent_fixture_with_identity(identity_fixture())
        } else {
            bound_parent_with_child_count(identity_fixture(), child_count)
        };
        let before = serde_json::to_vec(&source).expect("source bytes");
        reset_wb14_reseal_audit();
        assert!(source.rebind_final_accepted_slab([0; 32]).is_err());
        assert_eq!(
            wb14_reseal_audit(),
            DirectWb14ResealAuditV1 {
                input_validations: 1,
                child_traversals: 0,
                final_validations: 0,
            }
        );
        assert_eq!(
            serde_json::to_vec(&source).expect("unchanged source bytes"),
            before
        );
    }

    let source = bound_parent_with_child_count(identity_fixture(), 30);
    let mut poison = source.clone();
    poison
        .receipts
        .get_mut_for_test(17)
        .expect("poison receipt")
        .pending_routed_parcels_after_sha256[0] ^= 1;
    let before = serde_json::to_vec(&poison).expect("poison bytes");
    reset_wb14_reseal_audit();
    assert!(poison.rebind_final_accepted_slab([91; 32]).is_err());
    assert_eq!(
        wb14_reseal_audit(),
        DirectWb14ResealAuditV1 {
            input_validations: 1,
            child_traversals: 0,
            final_validations: 0,
        }
    );
    assert_eq!(
        serde_json::to_vec(&poison).expect("unchanged poison bytes"),
        before
    );
}

#[test]
fn linear_reseal_rejects_transition_amount_owner_and_order_poisons() {
    let original = bound_parent_with_child_count(identity_fixture(), 2);
    let rebound = original
        .rebind_final_accepted_slab([33; 32])
        .expect("exact final-slab reseal");
    assert!(
        original
            .receipts
            .iter()
            .take(1)
            .eq(rebound.receipts.iter().take(1))
    );
    assert_eq!(
        rebound
            .receipts
            .last()
            .expect("rebound final child")
            .accepted_coupled_slab_sha256,
        [33; 32]
    );
    rebound.finalize().expect("rebound parent finalization");

    let mut transition = original.clone();
    transition
        .receipts
        .get_mut_for_test(1)
        .expect("transition receipt")
        .transitions[0]
        .interval_duration_s_bits ^= 1;
    assert!(transition.rebind_final_accepted_slab([33; 32]).is_err());
    let mut amount = original.clone();
    amount
        .receipts
        .get_mut_for_test(1)
        .expect("amount receipt")
        .interval_supply_m_bits ^= 1;
    assert!(amount.rebind_final_accepted_slab([33; 32]).is_err());
    let mut owner = original.clone();
    owner
        .receipts
        .get_mut_for_test(1)
        .expect("owner receipt")
        .child_beginning_complete_owner_set_sha256[0] ^= 1;
    assert!(owner.rebind_final_accepted_slab([33; 32]).is_err());
    let mut order = original;
    order.receipts.swap_for_test(0, 1);
    assert!(order.rebind_final_accepted_slab([33; 32]).is_err());
}

#[test]
fn multi_ofe_linear_reseal_matches_reference_with_one_set_validation_pair() {
    let mut second_identity = identity_fixture();
    second_identity.ofe_id_sha256 = [13; 32];
    second_identity.production_lane_id = 5;
    let first = bound_parent_with_child_count(identity_fixture(), 30);
    let second = bound_parent_with_child_count(second_identity, 30);
    let prior = first
        .coupled_child_binding_v1()
        .expect("first coupled binding");
    assert_eq!(
        second
            .coupled_child_binding_v1()
            .expect("second coupled binding"),
        prior
    );
    let target = super::super::DirectWb14CoupledChildBindingV1 {
        accepted_slab_sha256: [92; 32],
        ..prior
    };
    let ofe_a = OfeId::try_new("linear-a").expect("OFE A");
    let ofe_b = OfeId::try_new("linear-b").expect("OFE B");
    let source_rows = vec![(ofe_a.clone(), first), (ofe_b.clone(), second)];
    let source_bytes = serde_json::to_vec(&source_rows).expect("multi-OFE source");

    let expected_rows = source_rows
        .iter()
        .map(|(ofe_id, authority)| {
            authority
                .rebind_final_accepted_slab_reference_for_test([92; 32])
                .map(|rebound| (ofe_id.clone(), rebound))
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("multi-OFE reference reseal");
    let expected_child = serde_json::to_vec(&expected_rows).expect("reference child bytes");
    let expected_parent = serde_json::to_vec(
        &expected_rows
            .iter()
            .map(|(ofe_id, authority)| {
                authority
                    .finalize()
                    .map(|finalization| (ofe_id.clone(), finalization))
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("reference parent finalizations"),
    )
    .expect("reference parent bytes");

    reset_wb14_reseal_audit();
    let (actual_child, actual_parent) =
        crate::direct_runtime::surface_liquid_ingress::rebind_wb14_replay_to_accepted_slab(
            &source_bytes,
            true,
            target,
        )
        .expect("multi-OFE linear reseal");
    assert_eq!(
        wb14_reseal_audit(),
        DirectWb14ResealAuditV1 {
            input_validations: 1,
            child_traversals: 60,
            final_validations: 1,
        }
    );
    assert_eq!(actual_child, expected_child);
    assert_eq!(actual_parent, Some(expected_parent));
}
