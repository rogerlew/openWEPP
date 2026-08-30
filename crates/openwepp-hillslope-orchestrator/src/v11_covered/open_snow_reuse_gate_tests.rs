mod covered_ordinary_physical_reuse_gate_tests {
    use super::*;

    fn authority(
        input: u8,
        beginning: u8,
        authorization: u8,
    ) -> CoveredOrdinaryPhysicalAuthorityV1 {
        CoveredOrdinaryPhysicalAuthorityV1 {
            physical_input_sha256: Digest32::from_bytes([input; 32]),
            beginning_authority_sha256: Digest32::from_bytes([beginning; 32]),
            accepted_authorization_sha256: Digest32::from_bytes([authorization; 32]),
        }
    }

    #[test]
    fn rainy_and_reappearance_reuse_gate_rejects_every_authority_substitution_and_terminal_mode() {
        let exact = authority(1, 2, 3);
        assert!(validate_covered_ordinary_physical_reuse_gate_v1(exact, exact, false).is_ok());
        assert!(
            validate_covered_ordinary_physical_reuse_gate_v1(exact, authority(4, 2, 3), false)
                .is_err(),
            "physical input substitution",
        );
        assert!(
            validate_covered_ordinary_physical_reuse_gate_v1(exact, authority(1, 4, 3), false)
                .is_err(),
            "beginning owner substitution",
        );
        assert!(
            validate_covered_ordinary_physical_reuse_gate_v1(exact, authority(1, 2, 4), false)
                .is_err(),
            "accepted authorization substitution",
        );
        assert!(
            validate_covered_ordinary_physical_reuse_gate_v1(exact, exact, true).is_err(),
            "terminal candidates must execute the physical stack",
        );
    }
}

mod covered_terminal_reuse_trial_binding_tests {
    use super::*;

    fn binding(seed: u8) -> crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
        crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            proposed_upper_bound_s_bits: 1_800.0_f64.to_bits(),
            coupled_parent_transaction_sha256: [seed; 32],
            accepted_slab_sha256: [seed.wrapping_add(1); 32],
            parent_beginning_complete_owner_set_sha256: [seed.wrapping_add(2); 32],
            parent_support_start_ns: 0,
            parent_support_end_ns: 1_800_000_000_000,
            child_support_start_ns: 0,
            child_support_end_ns: 1_800_000_000_000,
        }
    }

    #[test]
    fn terminal_reuse_allows_only_sealed_discovery_trial_identity_rebinding() {
        let accepted = binding(1);
        let replay_trial = Digest32::from_bytes([9; 32]);
        let replay_beginning = Digest32::from_bytes([10; 32]);
        let final_physical_child = TimeSupport::new(
            openwepp_coupled_time::ModelTimeNs::new(900_000_000_000),
            openwepp_coupled_time::ModelTimeNs::new(1_800_000_000_000),
        )
        .expect("final physical child support");
        let observed = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            accepted_slab_sha256: *replay_trial.as_bytes(),
            parent_beginning_complete_owner_set_sha256: *replay_beginning.as_bytes(),
            child_support_start_ns: final_physical_child.start_ns().get(),
            child_support_end_ns: final_physical_child.end_ns().get(),
            ..accepted
        };
        validate_covered_terminal_reuse_trial_binding_v1(
            observed,
            accepted,
            final_physical_child,
            replay_trial,
            replay_beginning,
        )
        .expect("exact discovery-local trial authorization");

        let mut transaction_poison = observed;
        transaction_poison.coupled_parent_transaction_sha256 = [11; 32];
        assert!(
            validate_covered_terminal_reuse_trial_binding_v1(
                transaction_poison,
                accepted,
                final_physical_child,
                replay_trial,
                replay_beginning,
            )
            .is_err()
        );
        let mut support_poison = observed;
        support_poison.child_support_end_ns -= 1;
        assert!(
            validate_covered_terminal_reuse_trial_binding_v1(
                support_poison,
                accepted,
                final_physical_child,
                replay_trial,
                replay_beginning,
            )
            .is_err()
        );
        let mut trial_poison = observed;
        trial_poison.accepted_slab_sha256 = [12; 32];
        assert!(
            validate_covered_terminal_reuse_trial_binding_v1(
                trial_poison,
                accepted,
                final_physical_child,
                replay_trial,
                replay_beginning,
            )
            .is_err()
        );
    }
}
