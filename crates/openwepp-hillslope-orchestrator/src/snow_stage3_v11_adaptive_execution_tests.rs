mod adaptive_trial_failure_classification_tests {
    use super::*;

    fn below_carrier_domain() -> DirectSnowStage3V11AttachmentError {
        DirectSnowStage3V11AttachmentError::Stage3(
            DirectSnowStage3EvaluationError::TerminalNumerics(
                crate::SnowTerminalNumericsFailure::BelowCarrierDomain,
            ),
        )
    }

    fn nested_lse_failure(
        error: openwepp_land_surface_energy::LandSurfaceEnergyError,
    ) -> DirectSnowStage3V11AttachmentError {
        DirectSnowStage3V11AttachmentError::V11(
            openwepp_vegetation::v11::V11ExecutionError::Executor(
                DirectV11RealConsumerError::Runtime(
                    crate::v9_real_consumer_shadow::DirectV10RealConsumerError::Runtime(
                        crate::v9_real_consumer_shadow::DirectV9RealConsumerError::Physical(
                            crate::land_surface_energy_shadow::ExecuteV8LseRuntimeShadowError::Physical(
                                crate::land_surface_energy_shadow::LandSurfaceEnergyShadowError::LandSurface(error),
                            ),
                        ),
                    ),
                ),
            ),
        )
    }

    fn cold_open_snow_candidate() -> DirectSnowStage3V11AttachmentError {
        DirectSnowStage3V11AttachmentError::V11(
            openwepp_vegetation::v11::V11ExecutionError::Executor(
                DirectV11RealConsumerError::OpenSnowLowerBoundaryDomain {
                    lane_id: 1,
                    ofe_id: "ofe-1".to_owned(),
                    tile_id: "open".to_owned(),
                    start_ns: 32_400_000_000_000,
                    end_ns: 34_200_000_000_000,
                    snow_temperature_k: 184.926_030_570_090_23,
                    latent_heat_j_kg: 3_109_070.522,
                    sensible_outward_w_m2: -443.339_19,
                    vapor_outward_kg_m2_s: -1.874_396_6e-5,
                    net_longwave_w_m2: 187.574_365,
                    shortwave_absorbed_w_m2: 54.321_526,
                    albedo: 0.82,
                    beginning_stage3: "beginning".to_owned(),
                    forcing: "forcing".to_owned(),
                    exposure: "exposure".to_owned(),
                    optical: "optical".to_owned(),
                    longwave: "longwave".to_owned(),
                },
            ),
        )
    }

    fn below_absolute_zero_terminal_projection(mass_swe_m: f64) -> DirectSnowStage3EvaluationError {
        let temperature_c = -300.0;
        let layer = DirectSnowLayerState {
            mass_swe_m,
            thickness_m: mass_swe_m * 10.0,
            density_kg_m3: 100.0,
            settle_day_count: 0.0,
            temperature_c,
            liquid_water_m: 0.0,
            cold_content_j_m2: mass_swe_m * 1_000.0 * 2_100.0 * -temperature_c,
            refrozen_liquid_m: 0.0,
        };
        DirectSnowStage3EvaluationError::Kernel(Box::new(
            crate::hydrology::Wb11HydrologyKernelGuardError::SnowStage3Conductivity(Box::new(
                crate::hydrology::SnowStage3ConductivityError {
                    phase_class: openwepp_kernel_contract::HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                    source: openwepp_meteorology::MeteorologyError::BelowAbsoluteZero {
                        quantity: "temperature_c",
                        value_c: temperature_c,
                    },
                    layer_index: 0,
                    layer,
                    control_volume_layers: vec![layer],
                    control_volume_temperature:
                        openwepp_unit_boundary::TemperatureCelsius::try_new(temperature_c)
                            .expect("finite temperature snapshot"),
                    atmospheric_pressure_pa: 79_263.0,
                },
            )),
        ))
    }

    #[test]
    fn structural_trial_failure_propagates_without_refinement() {
        let result = adaptive_propagate_non_refinable_trial_failure_v1(
            Err::<u8, _>(DirectSnowStage3V11AttachmentError::Identity(
                "poisoned complete-owner trial",
            )),
            Ok(2),
        );
        assert!(matches!(
            result,
            Err(DirectSnowStage3V11AttachmentError::Identity(
                "poisoned complete-owner trial"
            ))
        ));
    }

    #[test]
    fn direct_invalid_composed_valid_retains_composed_for_typed_rejection_receipt() {
        let outcome = adaptive_propagate_non_refinable_trial_failure_v1(
            Err::<u8, _>(below_carrier_domain()),
            Ok(7),
        )
        .expect("refinable physical failure");
        assert!(matches!(
            outcome,
            AdaptiveTrialPairOutcomeV1::Refinable {
                direct: None,
                composed: Some(7)
            }
        ));
    }

    #[test]
    fn cold_coarse_open_snow_candidate_refines_when_composed_children_succeed() {
        let outcome = adaptive_propagate_non_refinable_trial_failure_v1(
            Err::<u8, _>(cold_open_snow_candidate()),
            Ok(9),
        )
        .expect("candidate-local coarse open-snow temperature refines");
        assert!(matches!(
            outcome,
            AdaptiveTrialPairOutcomeV1::Refinable {
                direct: None,
                composed: Some(9)
            }
        ));
    }

    #[test]
    fn cold_open_snow_candidate_at_exact_floor_fails_closed() {
        let error = accept_adaptive_floor_trial_v1::<()>(Err(cold_open_snow_candidate()))
            .expect_err("no admissible child below the exact 60-second floor");
        assert!(adaptive_refinable_trial_failure_v1(&error));
        assert!(
            error
                .to_string()
                .contains("snow_temperature_k=184.92603057009023")
        );
    }

    #[test]
    fn below_absolute_zero_terminal_projection_refines_only_above_floor() {
        let terminal_error = DirectSnowStage3V11AttachmentError::V11(
            openwepp_vegetation::v11::V11ExecutionError::Executor(
                DirectV11RealConsumerError::Stage3(below_absolute_zero_terminal_projection(
                    0.000_25,
                )),
            ),
        );
        let outcome =
            adaptive_propagate_non_refinable_trial_failure_v1(Ok(7), Err::<u8, _>(terminal_error))
                .expect("finite terminal-domain projection failure should refine");
        assert!(matches!(
            outcome,
            AdaptiveTrialPairOutcomeV1::Refinable {
                direct: Some(7),
                composed: None
            }
        ));

        let floor_error = DirectSnowStage3V11AttachmentError::Stage3(
            below_absolute_zero_terminal_projection(0.000_25),
        );
        let floor_error = accept_adaptive_floor_trial_v1::<()>(Err(floor_error))
            .expect_err("exact floor cannot refine below 60 seconds");
        assert!(adaptive_refinable_trial_failure_v1(&floor_error));
    }

    #[test]
    fn below_absolute_zero_resolved_projection_refines_above_floor() {
        let error = DirectSnowStage3V11AttachmentError::Stage3(
            below_absolute_zero_terminal_projection(0.002),
        );
        assert!(adaptive_refinable_trial_failure_v1(&error));
    }

    #[test]
    fn cold_conductivity_snapshot_identity_poison_remains_non_refinable() {
        let mut error = below_absolute_zero_terminal_projection(0.002);
        let DirectSnowStage3EvaluationError::Kernel(kernel) = &mut error else {
            panic!("kernel snapshot");
        };
        let crate::hydrology::Wb11HydrologyKernelGuardError::SnowStage3Conductivity(snapshot) =
            kernel.as_mut()
        else {
            panic!("conductivity snapshot");
        };
        snapshot.layer.mass_swe_m = 0.003;
        assert!(!adaptive_refinable_trial_failure_v1(
            &DirectSnowStage3V11AttachmentError::Stage3(error)
        ));
    }

    #[test]
    fn direct_owner_candidate_refinement_uses_same_narrow_classifier() {
        let error = DirectSnowStage3V11AttachmentError::Owner(
            DirectV11RealConsumerError::AdaptiveRefinement(
                "covered Stage-3 lower-boundary temperature below constitutive domain",
            ),
        );
        assert!(adaptive_refinable_trial_failure_v1(&error));
        let floor = accept_adaptive_floor_trial_v1::<()>(Err(error))
            .expect_err("exact floor remains fail-closed");
        assert!(floor.to_string().contains("below constitutive domain"));
    }

    #[test]
    fn non_refinable_composed_failure_has_deterministic_precedence() {
        let result = adaptive_propagate_non_refinable_trial_failure_v1(
            Err::<u8, _>(below_carrier_domain()),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(
                "poisoned receipt",
            )),
        );
        assert!(matches!(
            result,
            Err(DirectSnowStage3V11AttachmentError::Precipitation(
                "poisoned receipt"
            ))
        ));
    }

    #[test]
    fn trial_local_numerical_failures_refine_without_weakening_constitutive_guards() {
        assert!(adaptive_refinable_trial_failure_v1(&nested_lse_failure(
            openwepp_land_surface_energy::LandSurfaceEnergyError::NumericalAcceptedResidual,
        )));
        assert!(adaptive_refinable_v11_trial_failure_v1(
            &DirectV11RealConsumerError::CoveredBoundary(
                crate::snow_stage3_terminal_handoff::SnowStage3HandoffError::FixedPointIterationLimit,
            ),
        ));
        assert!(adaptive_refinable_trial_failure_v1(&nested_lse_failure(
            openwepp_land_surface_energy::LandSurfaceEnergyError::ComponentClosure(
                "Stage-3 lower-boundary/column operand join",
            ),
        )));

        for error in [
            openwepp_land_surface_energy::LandSurfaceEnergyError::UnsupportedDomain(
                "hydraulic_redistribution",
            ),
            openwepp_land_surface_energy::LandSurfaceEnergyError::ConstitutiveDomain(
                "covered occupancy liquid ledger",
            ),
            openwepp_land_surface_energy::LandSurfaceEnergyError::ComponentClosure(
                "unrelated closure defect",
            ),
        ] {
            assert!(!adaptive_refinable_trial_failure_v1(&nested_lse_failure(
                error
            )));
        }
        assert!(!adaptive_refinable_trial_failure_v1(
            &DirectSnowStage3V11AttachmentError::Identity("poisoned trial owner identity")
        ));
    }

    #[test]
    fn trial_context_preserves_support_phase_source_and_refinement_classification() {
        let support = TimeSupport::new(
            ModelTimeNs::new(7_440_000_000_000),
            ModelTimeNs::new(7_500_000_000_000),
        )
        .expect("60-second support");
        let error = contextualize_adaptive_trial_failure_v1::<()>(
            "covered direct",
            &[support],
            Err(DirectSnowStage3V11AttachmentError::V11(
                openwepp_vegetation::v11::V11ExecutionError::Executor(
                    DirectV11RealConsumerError::CoveredBoundary(
                        crate::snow_stage3_terminal_handoff::SnowStage3HandoffError::
                            FixedPointIterationLimit,
                    ),
                ),
            )),
            None,
        )
        .expect_err("contextualized trial failure");
        assert!(adaptive_refinable_trial_failure_v1(&error));
        assert_eq!(
            error.to_string(),
            "Stage-3/V11 adaptive covered direct trial failure at 7440000000000..7500000000000 ns (60000000000 ns): VEG-E-123: imported V10 segment execution failed: SC-SNOWENERGY-E-FIXED-POINT-001: bounded covered fixed-point iteration did not converge"
        );
    }
}

mod adaptive_covered_child_memo_authority_tests {
    use super::*;

    fn digest(seed: u8) -> Digest32 {
        digest_bytes(&[seed])
    }

    fn key() -> AdaptiveCoveredTrialMemoKeyV1 {
        let parent_support =
            TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
                .expect("parent support");
        let interval = ParentIntervalId::derive(digest(1), digest(2), digest(3), parent_support)
            .expect("parent interval");
        AdaptiveCoveredTrialMemoKeyV1 {
            parent_transaction_id: ParentTransactionId::derive(digest(4), 5, interval, digest(6))
                .expect("parent transaction"),
            support: TimeSupport::new(
                ModelTimeNs::new(60_000_000_000),
                ModelTimeNs::new(120_000_000_000),
            )
            .expect("child support"),
            child_ordinal: 7,
            beginning_complete_owner_set_sha256: digest(8),
            forcing_projection_sha256: digest(9),
            topology_sha256: digest(10),
            configuration_sha256: digest(11),
            pending_terminal_parcel_set_sha256: digest(12),
        }
    }

    fn alternate_parent_transaction() -> ParentTransactionId {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("alternate parent support");
        let interval = ParentIntervalId::derive(digest(21), digest(22), digest(23), support)
            .expect("alternate parent interval");
        ParentTransactionId::derive(digest(24), 25, interval, digest(26))
            .expect("alternate parent transaction")
    }

    #[test]
    fn covered_child_memo_requires_every_physical_and_authorization_key_field() {
        let exact = key();
        validate_adaptive_covered_trial_memo_key_v1(exact, exact)
            .expect("exact child memo authority");

        let mut poisons = Vec::new();
        let mut poison = exact;
        poison.parent_transaction_id = alternate_parent_transaction();
        poisons.push(poison);
        let mut poison = exact;
        poison.child_ordinal += 1;
        poisons.push(poison);
        let mut poison = exact;
        poison.support = TimeSupport::new(
            ModelTimeNs::new(60_000_000_000),
            ModelTimeNs::new(180_000_000_000),
        )
        .expect("changed support");
        poisons.push(poison);
        let mut poison = exact;
        poison.beginning_complete_owner_set_sha256 = digest(13);
        poisons.push(poison);
        let mut poison = exact;
        poison.forcing_projection_sha256 = digest(14);
        poisons.push(poison);
        let mut poison = exact;
        poison.topology_sha256 = digest(15);
        poisons.push(poison);
        let mut poison = exact;
        poison.configuration_sha256 = digest(16);
        poisons.push(poison);
        let mut poison = exact;
        poison.pending_terminal_parcel_set_sha256 = digest(17);
        poisons.push(poison);

        for poison in poisons {
            assert!(matches!(
                validate_adaptive_covered_trial_memo_key_v1(exact, poison),
                Err(DirectSnowStage3V11AttachmentError::Identity(
                    "adaptive covered physical trial memo authority"
                ))
            ));
        }
    }
}

mod solid_reappearance_publication_posture_tests {
    use super::*;

    #[test]
    fn open_parent_requires_transitioned_state_without_orphan_publication() {
        validate_solid_reappearance_publication_posture_v1(true, true, false, false)
            .expect("authenticated in-progress reappearance posture");
        for poisoned in [
            (true, false, false, false),
            (true, true, true, true),
            (true, true, true, false),
        ] {
            assert!(
                validate_solid_reappearance_publication_posture_v1(
                    poisoned.0, poisoned.1, poisoned.2, poisoned.3,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn accepted_support_requires_event_as_ordered_publication_tail() {
        validate_solid_reappearance_publication_posture_v1(false, true, true, true)
            .expect("atomic event/support publication posture");
        for poisoned in [(false, true, false, false), (false, true, true, false)] {
            assert!(
                validate_solid_reappearance_publication_posture_v1(
                    poisoned.0, poisoned.1, poisoned.2, poisoned.3,
                )
                .is_err()
            );
        }
    }
}

mod solid_reappearance_beginning_owner_tests {
    use super::*;

    fn snow_free_state() -> DirectSnowStage3PersistentState {
        let mut state = DirectSnowStage3PersistentState {
            schema_version: 1,
            terminal_event_model: None,
            fingerprint: 0,
            lane_id: 1,
            next_interval_index: 11,
            layers: Vec::new(),
            detached_retained_liquid_kg_m2: 0.0,
            initial_ice_kg_m2: 0.0,
            initial_retained_liquid_kg_m2: 0.0,
            cumulative_snowfall_kg_m2: 2.0,
            cumulative_external_liquid_kg_m2: 0.0,
            cumulative_deposition_kg_m2: 0.0,
            cumulative_sublimation_kg_m2: 0.0,
            cumulative_melt_kg_m2: 2.0,
            cumulative_unresolved_liquid_kg_m2: 0.0,
            cumulative_complete_energy_j_m2: 0.0,
            cumulative_cold_energy_change_j_m2: 0.0,
            cumulative_terminal_unallocated_energy_j_m2: 0.0,
        };
        state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&state);
        state
    }

    #[test]
    fn reappearance_joins_exact_v1_or_post_terminal_v4_without_losing_receipt_authority() {
        let lanes = BTreeMap::from([(1, snow_free_state())]);
        let v1 = canonical_stage3_snow_owner_bytes(&lanes).unwrap();
        validate_solid_reappearance_beginning_snow_owner_v1(&v1, &lanes)
            .expect("ordinary V1 beginning owner");

        let v4 = crate::snow_owner_v4::canonical_stage3_snow_owner_v4_bytes(
            &lanes,
            &BTreeMap::new(),
            &BTreeMap::from([(1, Digest32::from_bytes([21; 32]))]),
            &BTreeMap::from([(
                ("ofe-1".to_owned(), "ground".to_owned()),
                Digest32::from_bytes([22; 32]),
            )]),
        )
        .unwrap();
        validate_solid_reappearance_beginning_snow_owner_v1(&v4, &lanes)
            .expect("post-terminal V4 beginning owner");

        let mut substituted = lanes.clone();
        substituted.get_mut(&1).unwrap().next_interval_index += 1;
        assert!(validate_solid_reappearance_beginning_snow_owner_v1(&v4, &substituted).is_err());
        let mut omitted_receipt = v4;
        omitted_receipt.truncate(omitted_receipt.len() - 37);
        assert!(
            validate_solid_reappearance_beginning_snow_owner_v1(&omitted_receipt, &lanes).is_err()
        );
    }
}
