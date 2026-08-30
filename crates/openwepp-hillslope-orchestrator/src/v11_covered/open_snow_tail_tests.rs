mod component_carrier_tests {
    use super::*;
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};

    fn make_boundary(optical: u8) -> FinalStage3CanopyBoundaryReceiptV1 {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_000_000_000))
            .expect("support");
        FinalStage3CanopyBoundaryReceiptV1::try_new(FinalStage3CanopyBoundaryReceiptInputs {
            support,
            destination: (
                OfeId::try_new("ofe-1").expect("OFE"),
                TileId::try_new("forest").expect("tile"),
            ),
            beginning_v11_state_sha256: Digest32::from_bytes([1; 32]),
            beginning_stage3_state_sha256: Digest32::from_bytes([2; 32]),
            ending_v8_physical_candidate_sha256: Digest32::from_bytes([3; 32]),
            ending_stage3_state_sha256: Digest32::from_bytes([4; 32]),
            provisional_carrier_receipt_sha256: Digest32::from_bytes([5; 32]),
            optical_receipt_sha256: Digest32::from_bytes([optical; 32]),
            reciprocal_longwave_receipt_sha256: Digest32::from_bytes([7; 32]),
            sensible_to_canopy_air_w_m2: 2.0,
            vapor_to_canopy_air_kg_m2_s: 1.0e-6,
            latent_energy_to_canopy_air_j_m2: 2.5,
            snow_temperature_k: 270.0,
            latent_heat_j_kg: 2_500_000.0,
            snow_absorbed_shortwave_w_m2: 10.0,
            snow_net_longwave_w_m2: -5.0,
        })
        .expect("boundary")
    }

    fn state() -> CoveredLseIterationState {
        CoveredLseIterationState {
            canopy_air_temperature_k: 290.0,
            canopy_air_specific_humidity_kg_kg: 0.01,
            snow_temperature_k: 270.0,
            snow_sensible_w_m2: 2.0,
            snow_vapor_kg_m2_s: 1.0e-6,
            snow_latent_w_m2: 2.5,
            snow_net_longwave_w_m2: -5.0,
            component_temperatures_k: vec![("canopy".into(), [292.0; 4])],
            component_carrier_surfaces: (0_u8..4)
                .map(|component_ordinal| CoveredCarrierComponentState {
                    vertical_occupancy_ordinal: 0,
                    occupancy_id: "canopy".into(),
                    component_ordinal,
                    surface_area_m2_m2_tile: 0.25,
                    emissive_area_m2_m2_tile: 0.25,
                    heat_conductance_m_s_tile: 0.25,
                    vapor_conductance_m_s_tile: if component_ordinal == 3 { 0.0 } else { 0.25 },
                    vapor_authorization_kg_m2_tile_s: None,
                    temperature_k: 292.0,
                    specific_humidity_kg_kg: 0.011,
                    sensible_to_canopy_air_w_m2: 0.75,
                    vapor_to_canopy_air_kg_m2_s: if component_ordinal == 3 {
                        0.0
                    } else if component_ordinal == 2 {
                        1.0e-6
                    } else {
                        0.5e-6
                    },
                })
                .collect(),
            canopy_sensible_w_m2: 3.0,
            canopy_vapor_kg_m2_s: 2.0e-6,
            sensible_to_reference_air_w_m2: 5.0,
            vapor_to_reference_air_kg_m2_s: 3.0e-6,
            shared_heat_residual_w_m2: 0.0,
            shared_heat_tolerance_w_m2: 1.0e-6,
            shared_vapor_residual_kg_m2_s: 0.0,
            shared_vapor_tolerance_kg_m2_s: 1.0e-12,
        }
    }

    #[test]
    fn component_carrier_rejects_stale_inner_seal_and_fresh_boundary_substitution() {
        let boundary = make_boundary(6);
        let mut receipt = ComponentResolvedCarrierReceiptV1::try_new(
            boundary.destination.clone(),
            &state(),
            &boundary,
        )
        .expect("component receipt");
        receipt.components[0].temperature_k += 1.0;
        assert!(receipt.validate(&boundary).is_err());

        let alternate_boundary = make_boundary(8);
        let receipt = ComponentResolvedCarrierReceiptV1::try_new(
            boundary.destination.clone(),
            &state(),
            &boundary,
        )
        .expect("component receipt");
        assert!(receipt.validate(&alternate_boundary).is_err());
    }

    #[test]
    fn component_carrier_retains_lse_shared_air_residual_authority_and_poisons() {
        let boundary = make_boundary(6);
        let mut physical = state();
        physical.sensible_to_reference_air_w_m2 -= 2.7e-7;
        physical.shared_heat_residual_w_m2 = physical.canopy_sensible_w_m2
            + physical.snow_sensible_w_m2
            - physical.sensible_to_reference_air_w_m2;
        physical.shared_heat_tolerance_w_m2 = 3.0e-7;
        let accepted = ComponentResolvedCarrierReceiptV1::try_new(
            boundary.destination.clone(),
            &physical,
            &boundary,
        )
        .expect("sealed LSE residual within its exact allowance");
        accepted
            .validate(&boundary)
            .expect("independently reconstructed LSE residual");

        let mut substituted = physical.clone();
        substituted.shared_heat_residual_w_m2 = 0.0;
        assert!(
            ComponentResolvedCarrierReceiptV1::try_new(
                boundary.destination.clone(),
                &substituted,
                &boundary,
            )
            .is_err()
        );

        let mut omitted = physical.clone();
        omitted.shared_heat_tolerance_w_m2 = 0.0;
        assert!(
            ComponentResolvedCarrierReceiptV1::try_new(
                boundary.destination.clone(),
                &omitted,
                &boundary,
            )
            .is_err()
        );

        let mut excessive = physical.clone();
        excessive.shared_heat_tolerance_w_m2 = 1.0e-7;
        assert!(
            ComponentResolvedCarrierReceiptV1::try_new(
                boundary.destination.clone(),
                &excessive,
                &boundary,
            )
            .is_err()
        );

        let mut stale_digest = accepted;
        stale_digest.shared_heat_tolerance_w_m2 = 4.0e-7;
        assert!(stale_digest.validate(&boundary).is_err());
    }

    #[test]
    fn component_carrier_uses_vertical_order_not_lexical_occupancy_order() {
        let boundary = make_boundary(6);
        let mut physical = state();
        let upper = physical
            .component_carrier_surfaces
            .iter()
            .cloned()
            .map(|mut component| {
                component.occupancy_id = "z-upper".into();
                component.surface_area_m2_m2_tile *= 0.5;
                component.emissive_area_m2_m2_tile *= 0.5;
                component.heat_conductance_m_s_tile *= 0.5;
                component.vapor_conductance_m_s_tile *= 0.5;
                component.sensible_to_canopy_air_w_m2 *= 0.5;
                component.vapor_to_canopy_air_kg_m2_s *= 0.5;
                component
            })
            .collect::<Vec<_>>();
        let lower = upper
            .iter()
            .cloned()
            .map(|mut component| {
                component.vertical_occupancy_ordinal = 1;
                component.occupancy_id = "a-lower".into();
                component
            })
            .collect::<Vec<_>>();
        physical.component_carrier_surfaces = upper.into_iter().chain(lower).collect();
        ComponentResolvedCarrierReceiptV1::try_new(
            boundary.destination.clone(),
            &physical,
            &boundary,
        )
        .expect("physical vertical order is authoritative");
    }

    #[test]
    fn component_carrier_rejects_duplicate_occupancy_across_vertical_ordinals() {
        let boundary = make_boundary(6);
        let mut physical = state();
        let mut duplicate = physical.component_carrier_surfaces.clone();
        for component in &mut duplicate {
            component.vertical_occupancy_ordinal = 1;
        }
        physical.component_carrier_surfaces.extend(duplicate);
        physical.canopy_sensible_w_m2 *= 2.0;
        physical.canopy_vapor_kg_m2_s *= 2.0;
        physical.sensible_to_reference_air_w_m2 =
            physical.canopy_sensible_w_m2 + physical.snow_sensible_w_m2;
        physical.vapor_to_reference_air_kg_m2_s =
            physical.canopy_vapor_kg_m2_s + physical.snow_vapor_kg_m2_s;
        assert!(
            ComponentResolvedCarrierReceiptV1::try_new(
                boundary.destination.clone(),
                &physical,
                &boundary,
            )
            .is_err()
        );
    }
}

#[cfg(test)]
mod precomputed_terminal_accepted_executor_tests {
    use super::*;

    fn accepted_branch_source() -> &'static str {
        include_str!("open_snow_terminal_accepted_endpoint.rs")
    }

    #[test]
    fn accepted_branch_seals_then_finalizes_then_publishes() {
        let source = accepted_branch_source();
        let seal = source
            .find("seal_accepted_carrier_evidence_v1")
            .expect("seal evidence");
        let ledger = source
            .find("self.physical_outcome_ledgers")
            .expect("terminal ledger");
        let finalize = source
            .find("finalize_v11_imported_segment")
            .expect("finalize");
        let publish = source
            .find("self.last_support_receipt =")
            .expect("publication");
        assert!(seal < ledger && ledger < finalize && finalize < publish);
        assert!(!source[..publish].contains("self.ending ="));
    }

    #[test]
    fn identity_poisons_precede_acceptance_and_physics_rerun_is_absent() {
        let source = accepted_branch_source();
        let seal = source
            .find("seal_accepted_carrier_evidence_v1")
            .expect("seal evidence");
        let preflight = &source[..seal];
        for poison in [
            "accepted_slab_sha256",
            "physical_child_ordinal",
            "beginning_pending_terminal_parcels",
            "wb14_child_receipt_set_sha256",
            "terminal_snow_soil_trial_receipts",
            "stage3_has_represented_ice",
        ] {
            assert!(preflight.contains(poison), "missing poison guard: {poison}");
        }
        for forbidden in [
            "evaluate_stage3_persistent_support",
            "evaluate_stage3_terminal_support",
            "execute_covered_carrier_phase_v1",
        ] {
            assert!(!source.contains(forbidden), "reran physics: {forbidden}");
        }
    }

    #[test]
    fn pre_finalize_failure_has_no_publication_or_new_parcel_mutation() {
        let source = accepted_branch_source();
        let finalize = source
            .find("finalize_v11_imported_segment")
            .expect("finalize");
        let before_finalize = &source[..finalize];
        for forbidden in [
            "self.last_",
            "self.ending =",
            "self.ending_stage3_by_lane =",
            "self.pending_terminal_parcels.insert",
            "self.pending_terminal_parcels =",
        ] {
            assert!(
                !before_finalize.contains(forbidden),
                "pre-finalize rollback surface mutated: {forbidden}"
            );
        }
        assert!(source.contains("endpoint.beginning_pending_terminal_parcels"));
    }

    #[test]
    fn terminal_endpoint_timing_composes_prefix_and_terminal_suffix() {
        assert!(accepted_terminal_endpoint_timing_v1(
            300.0, 1_500.0, 0.0, 1_800.0, 1_800.0, 1_800.0,
        ));
        assert!(accepted_terminal_endpoint_timing_v1(
            300.0, 600.0, 0.0, 900.0, 900.0, 900.0,
        ));
        assert!(accepted_terminal_endpoint_timing_v1(
            0.0, 1_800.0, 0.0, 1_800.0, 1_800.0, 1_800.0,
        ));
        assert!(!accepted_terminal_endpoint_timing_v1(
            0.0, 1_500.0, 0.0, 1_500.0, 1_800.0, 1_800.0,
        ));
        assert!(!accepted_terminal_endpoint_timing_v1(
            300.0, 1_500.0, 300.0, 1_800.0, 1_800.0, 1_800.0,
        ));
    }

    #[test]
    fn terminal_endpoint_ledger_rejects_prefix_omission_and_substitution() {
        let prefix = TerminalEndpointExternalLedgerV1 {
            energy_j_m2: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            vapor_kg_m2: 7.0,
        };
        let terminal = TerminalEndpointExternalLedgerV1 {
            energy_j_m2: [10.0, 20.0, 30.0, 40.0, 50.0, 60.0],
            vapor_kg_m2: 70.0,
        };
        let complete = prefix.ordered_add(terminal);
        assert!(prefix.ordered_add(terminal).matches(complete));
        assert!(!terminal.matches(complete));

        let substituted_terminal = TerminalEndpointExternalLedgerV1 {
            energy_j_m2: [10.0, 20.0, 30.0, 40.0, 60.0, 50.0],
            vapor_kg_m2: 70.0,
        };
        assert!(!prefix.ordered_add(substituted_terminal).matches(complete));
        assert!(
            !prefix
                .ordered_add(prefix)
                .ordered_add(terminal)
                .matches(complete)
        );
    }
}

#[cfg(test)]
#[path = "open_snow_convergence_tests.rs"]
mod covered_convergence_policy_tests;
