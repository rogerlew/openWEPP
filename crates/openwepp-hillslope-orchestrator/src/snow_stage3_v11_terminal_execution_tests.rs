mod terminal_exact_installation_source_guards {
    use super::{
        AcceptedPreterminalNonEventDispositionV1, TerminalClosureOperandsV1,
        accepted_preterminal_non_event_disposition_v1, localized_terminal_candidate_offsets_v1,
        reconstruct_terminal_closure_v1,
        replay_preterminal_microsteps_before_terminal_localization_v1,
        terminal_liquid_thermodynamics_v1,
    };
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};

    fn support(start_ns: u128, end_ns: u128) -> TimeSupport {
        TimeSupport::new(ModelTimeNs::new(start_ns), ModelTimeNs::new(end_ns))
            .expect("preterminal support")
    }

    fn non_event_disposition<'a>(
        microstep_supports: &'a [TimeSupport],
    ) -> AcceptedPreterminalNonEventDispositionV1<'a> {
        AcceptedPreterminalNonEventDispositionV1 {
            support: support(0, 1_800_000_000_000),
            event_occurred: false,
            terminal_entry_offset_seconds: 0.0,
            requested_seconds: 1_800.0,
            evaluated_seconds: 1_800.0,
            unevaluated_seconds: 0.0,
            hour_offset_seconds: 1_800.0,
            ending_is_supported_snow_domain: true,
            microstep_supports,
            microstep_states_are_exact: true,
        }
    }

    #[test]
    fn adaptive_preterminal_non_event_accepts_exact_full_resolved_support() {
        let one = [support(0, 1_800_000_000_000)];
        assert!(accepted_preterminal_non_event_disposition_v1(
            &non_event_disposition(&one)
        ));
        let tiled = [
            support(0, 60_000_000_000),
            support(60_000_000_000, 1_800_000_000_000),
        ];
        assert!(accepted_preterminal_non_event_disposition_v1(
            &non_event_disposition(&tiled)
        ));
    }

    #[test]
    fn terminal_discovery_localizes_before_any_microstep_can_seal_as_accepted_liquid() {
        assert!(
            replay_preterminal_microsteps_before_terminal_localization_v1(false, 2, 0.0, false)
                .expect("zero-solid trace reuse")
        );
        assert!(
            !replay_preterminal_microsteps_before_terminal_localization_v1(true, 2, 0.0, false)
                .expect("event trace exclusion")
        );
        assert!(
            !replay_preterminal_microsteps_before_terminal_localization_v1(false, 0, 0.0, false)
                .expect("empty trace exclusion")
        );
        let source = include_str!("snow_stage3_v11_terminal_execution.rs");
        for authority in [
            "tag: \"exact_event\"",
            "tag: \"discovery_event\"",
            "tag: \"discovery_state\"",
            "tag: \"discovery_owner_set\"",
        ] {
            assert!(source.contains(authority));
        }
    }

    #[test]
    fn positive_solid_forcing_excludes_enclosing_trace_reuse_and_requires_its_sealed_parcel() {
        assert!(
            !replay_preterminal_microsteps_before_terminal_localization_v1(false, 30, 0.001, true,)
                .expect("positive solid falls through to exact support")
        );
        assert!(
            replay_preterminal_microsteps_before_terminal_localization_v1(false, 30, 0.001, false,)
                .is_err(),
            "omitting the sealed solid parcel must fail closed",
        );
        assert!(
            replay_preterminal_microsteps_before_terminal_localization_v1(false, 30, 0.0, true)
                .is_err(),
            "substituting a solid parcel into zero-solid forcing must fail closed",
        );
        assert!(
            replay_preterminal_microsteps_before_terminal_localization_v1(
                false,
                30,
                f64::NAN,
                false,
            )
            .is_err(),
            "nonfinite forcing cannot authorize either path",
        );
    }

    #[test]
    fn terminal_discovery_offsets_bind_entry_plus_localized_duration() {
        assert_eq!(
            localized_terminal_candidate_offsets_v1(120.0, 60.0, 180.0, 60.0, 60.0),
            Some([180.0; 3]),
        );
        assert_eq!(
            localized_terminal_candidate_offsets_v1(120.0, 60.0, 60.0, 60.0, 60.0),
            None,
            "evaluated duration cannot substitute for the support-relative event offset",
        );
        assert_eq!(
            localized_terminal_candidate_offsets_v1(120.0, 60.0, 180.0, 61.0, 60.0),
            None,
            "reversed discovery bracket must fail closed",
        );
        assert_eq!(
            localized_terminal_candidate_offsets_v1(120.0, 60.0, 180.0, f64::NAN, 60.0),
            None,
            "omitted/nonfinite discovery authority must fail closed",
        );
    }

    #[test]
    fn adaptive_preterminal_non_event_rejects_timing_domain_state_and_order_poisons() {
        let one = [support(0, 1_800_000_000_000)];
        let mut poisoned = non_event_disposition(&one);
        poisoned.ending_is_supported_snow_domain = false;
        assert!(!accepted_preterminal_non_event_disposition_v1(&poisoned));
        let mut poisoned = non_event_disposition(&one);
        poisoned.microstep_states_are_exact = false;
        assert!(!accepted_preterminal_non_event_disposition_v1(&poisoned));
        let mut poisoned = non_event_disposition(&one);
        poisoned.unevaluated_seconds = 60.0;
        assert!(!accepted_preterminal_non_event_disposition_v1(&poisoned));
        let omitted = [];
        assert!(!accepted_preterminal_non_event_disposition_v1(
            &non_event_disposition(&omitted)
        ));
        let substituted = [support(60_000_000_000, 1_800_000_000_000)];
        assert!(!accepted_preterminal_non_event_disposition_v1(
            &non_event_disposition(&substituted)
        ));
        let reordered = [
            support(900_000_000_000, 1_800_000_000_000),
            support(0, 900_000_000_000),
        ];
        assert!(!accepted_preterminal_non_event_disposition_v1(
            &non_event_disposition(&reordered)
        ));
    }

    fn exactly_closed_terminal_operands() -> TerminalClosureOperandsV1 {
        TerminalClosureOperandsV1 {
            start_ice_kg_m2: 0.6,
            start_liquid_kg_m2: 0.1,
            end_ice_kg_m2: 0.0,
            end_liquid_kg_m2: 0.7,
            complete_energy_j_m2: crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG * 0.6,
            cold_energy_change_j_m2: 0.0,
            refrozen_kg_m2: 0.0,
            deposition_kg_m2: 0.0,
            sublimation_kg_m2: 0.0,
            melt_kg_m2: 0.6,
            unallocated_energy_j_m2: 0.0,
            external_liquid_kg_m2: 0.0,
        }
    }

    #[test]
    fn adaptive_preterminal_closure_is_reconstructed_from_exact_operands() {
        let residuals = reconstruct_terminal_closure_v1(exactly_closed_terminal_operands())
            .expect("exact terminal closure operands");
        assert_eq!(residuals.map(f64::to_bits), [0_u64; 3]);
    }

    #[test]
    fn adaptive_preterminal_closure_rejects_solid_liquid_and_energy_poison() {
        let mut solid = exactly_closed_terminal_operands();
        solid.end_ice_kg_m2 = 1.0e-6;
        let mut liquid = exactly_closed_terminal_operands();
        liquid.end_liquid_kg_m2 += 1.0e-6;
        let mut energy = exactly_closed_terminal_operands();
        energy.complete_energy_j_m2 += 1.0;
        let mut nonfinite = exactly_closed_terminal_operands();
        nonfinite.melt_kg_m2 = f64::NAN;
        for poisoned in [solid, liquid, energy, nonfinite] {
            assert!(reconstruct_terminal_closure_v1(poisoned).is_err());
        }
    }

    #[test]
    fn adaptive_preterminal_closure_rejects_projected_beginning_substitution() {
        // Exact operands captured from an accepted cooling/refreeze
        // microstep. The integrator beginning closes; substituting the
        // separately projected persistent-layer beginning does not.
        let exact = TerminalClosureOperandsV1 {
            start_ice_kg_m2: 9.778326352318553e-5,
            start_liquid_kg_m2: 7.586522929954605e-4,
            end_ice_kg_m2: 2.239750465697593e-4,
            end_liquid_kg_m2: 2.275956962399042e-3,
            complete_energy_j_m2: -42.11270002967555,
            cold_energy_change_j_m2: 0.0,
            refrozen_kg_m2: 1.2623711040070634e-4,
            deposition_kg_m2: 0.0,
            sublimation_kg_m2: 4.53273541325663e-8,
            melt_kg_m2: 0.0,
            unallocated_energy_j_m2: 0.0,
            external_liquid_kg_m2: 1.6435417798042879e-3,
        };
        reconstruct_terminal_closure_v1(exact).expect("exact accepted-step beginning closes");
        let mut projected = exact;
        projected.start_ice_kg_m2 = 7.465837666080662e-5;
        assert!(reconstruct_terminal_closure_v1(projected).is_err());
    }

    #[test]
    fn terminal_liquid_enthalpy_binds_exact_unallocated_energy_and_rejects_too_hot() {
        let mass = 0.6;
        let expected_specific_enthalpy = 12_000.0;
        let (temperature, specific_enthalpy) =
            terminal_liquid_thermodynamics_v1(mass, mass * expected_specific_enthalpy)
                .expect("bounded terminal liquid enthalpy");
        assert!((specific_enthalpy - expected_specific_enthalpy).abs() <= 1.0e-9);
        assert!((specific_enthalpy * mass - mass * expected_specific_enthalpy).abs() <= 1.0e-9);
        assert_eq!(
            temperature.to_bits(),
            (273.15 + expected_specific_enthalpy / 4_218.0).to_bits()
        );
        let too_hot_specific_enthalpy = 4_218.0 * (350.0 - 273.15) + 1.0;
        assert!(terminal_liquid_thermodynamics_v1(mass, mass * too_hot_specific_enthalpy).is_err());
    }

    #[test]
    fn exact_terminal_installation_uses_only_precomputed_executor() {
        let source = include_str!("snow_stage3_v11_terminal_execution.rs");
        assert!(source.contains("with_precomputed_terminal_accepted_endpoint"));
        assert!(source.contains("with_precomputed_terminal_provisional_endpoint"));
        assert!(source.contains("missing final accepted publication append"));
        let executor = include_str!("v11_covered/execution.rs");
        assert!(executor.contains("if self.last_publication_retained == Some(false)"));
        assert!(!source.contains(&["with_terminal", "_endpoint_mode"].concat()));
        assert!(!source.contains(&["evaluate_stage3_", "persistent_support(",].concat()));
        assert!(!source.contains(&["let Ok((", "parent"].concat()));
    }

    #[test]
    fn obsolete_terminal_consumer_and_duplicate_ordinal_authority_are_absent() {
        let attachment = include_str!("snow_stage3_v11_attachment.rs");
        let receipts = include_str!("snow_stage3_v11_attachment_receipts.rs");
        let persistent = include_str!(
            "hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/persistent_state.rs"
        );
        assert!(!attachment.contains("accepted_event_ordinal"));
        assert!(!receipts.contains("ending_event_ordinal"));
        assert!(!persistent.contains("consume_stage3_terminal_liquid_v1"));
    }

    #[test]
    fn terminal_zero_event_and_subminimum_paths_are_fail_closed() {
        let source = include_str!("snow_stage3_v11_terminal_execution.rs");
        assert!(source.contains("terminal zero-event V4 owner installation"));
        assert!(source.contains("terminal event creates positive subminimum support"));
    }

    #[test]
    fn parent_end_zero_duration_receiver_cannot_invoke_a_physical_lse_solve() {
        let source = include_str!("snow_stage3_v11_terminal_boundary_receiver.rs");
        assert!(source.contains("accept_zero_duration_terminal_receiver"));
        assert!(!source.contains("execute_real_v11_parent"));
        assert!(!source.contains("execute_direct_v11_segment"));
        assert!(!source.contains("with_terminal_receiver_parcels"));
    }

    #[test]
    fn deferred_native_v2_preterminal_join_reauthenticates_the_retained_trial() {
        let source = include_str!("snow_stage3_v11_terminal_execution.rs");
        let validator = source
            .split("fn validate_native_v2_preterminal_installation_v1(")
            .nth(1)
            .expect("native V2 preterminal validator")
            .split("fn advance_preterminal_state_v1(")
            .next()
            .expect("native V2 preterminal validator body");
        assert!(validator.contains("DeferredNativeV2SoilCustodyV1::try_new("));
        assert!(validator.contains("deferred_custody.candidate() != endpoint_soil"));
        assert!(validator.contains("deferred_custody.continuation() != endpoint_continuation"));
        assert!(!validator.contains("installed_soil_view.physically_equals(endpoint_soil_view)"));
        assert!(!validator.contains("installed_soil_view == endpoint_soil_view"));
    }

    #[test]
    fn deferred_native_v2_stack_input_remains_the_immutable_iterate_beginning() {
        let source = include_str!("snow_stage3_v11_terminal_execution.rs");
        let evaluator = source
            .split("fn evaluate_covered_terminal_candidate_with_evidence_v1")
            .nth(1)
            .expect("covered terminal evaluator")
            .split("fn prepare_exact_terminal_endpoint_v1")
            .next()
            .expect("covered terminal evaluator body");
        let stack_binding = evaluator
            .find("try_with_deferred_native_v2_soil_custody(custody.clone())")
            .expect("deferred stack binding");
        let provider = evaluator
            .find("let provider_result")
            .expect("carrier provider execution");
        assert!(stack_binding < provider);
        assert!(!evaluator[..provider].contains("request.beginning_joint.receipt_sha256() =="));
    }

    #[test]
    fn terminal_batch_non_event_paths_preserve_deferred_native_v2_child_custody() {
        let source = include_str!("snow_stage3_v11_terminal_execution_batch.rs");
        assert_eq!(
            source
                .matches("deferred_native_v2_soil_custody: None")
                .count(),
            0,
            "a completed physical child must not clear custody needed to authenticate its successor",
        );
        assert!(source.contains(
            "let (parent, consumer, clock, stage3, receipt, deferred_native_v2_soil_custody) = *outcome;"
        ));
    }

    #[test]
    fn touched_real_consumer_host_remains_below_hard_source_ceiling() {
        let source = include_str!("v9_real_consumer_shadow.rs");
        assert!(source.lines().count() < 3_000);
        assert!(source.contains("include!(\"v9_real_consumer_shadow_v10_accessors.rs\")"));
    }
}
