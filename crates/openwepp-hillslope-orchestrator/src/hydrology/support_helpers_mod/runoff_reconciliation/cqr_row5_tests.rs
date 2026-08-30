#[cfg(test)]
mod cqr_row5_tests {
    use super::*;

    #[test]
    fn child1_term_coupling_020_exhaustion_fails_closed_with_complete_diagnostics() {
        let common = CoveredTerminalEndingSnowHintV1 {
            ice_kg_m2: 0.2,
            liquid_kg_m2: 0.01,
            cold_content_j_m2: -125.0,
            surface_temperature_c: -1.0e-9,
        };
        let mut previous = None;
        let mut four_component_break = false;
        let mut final_pair = None;
        let mut evidence = CaptureState::default();
        for iteration in 0..32_u32 {
            let next = CoveredTerminalEndingSnowHintV1 {
                surface_temperature_c: if iteration % 2 == 0 { -1.0e-9 } else { 1.0e-9 },
                ..common
            };
            if let Some(prior) = previous {
                let comparisons = terminal_coupling_comparisons(prior, next);
                assert!(comparisons[..3].iter().all(|comparison| comparison.4));
                assert!(!comparisons[3].4);
                four_component_break |= terminal_coupling_four_component_converged(prior, next);
                final_pair = Some((prior, next));
            }
            evidence
                .coupling_iterations
                .push(CapturedCouplingIteration {
                    hook: TerminalCouplingIterationHook {
                        request: coupling_test_request(iteration, previous),
                        outgoing: next,
                        comparisons: previous
                            .map(|prior| terminal_coupling_comparisons(prior, next)),
                        converged: previous.is_some_and(|prior| {
                            terminal_coupling_four_component_converged(prior, next)
                        }),
                    },
                });
            previous = Some(next);
        }

        assert!(
            !four_component_break,
            "all 32 iterations exhaust the live four-component loop"
        );
        let (prior, next) = final_pair.expect("iteration pair");
        assert!(terminal_coupling_post_loop_three_component_converged(
            prior, next
        ));
        assert!(!terminal_coupling_four_component_converged(prior, next));
        evidence
            .coupling_selections
            .push(TerminalCouplingSelectionHook {
                request: coupling_test_request(31, Some(prior)),
                reason: TerminalCouplingSelectionReason::IterationLoopExhausted,
                post_loop_three_component_check: true,
            });
        let result = require_terminal_coupling_live_convergence(four_component_break);
        assert!(matches!(
            result,
            Err(DirectSnowStage3EvaluationError::TerminalCustody(
                "covered terminal coupled trial nonconvergence"
            ))
        ));
        assert_eq!(evidence.coupling_iterations.len(), 32);
        assert_eq!(evidence.coupling_selections.len(), 1);
        assert_eq!(
            evidence.coupling_selections[0].reason,
            TerminalCouplingSelectionReason::IterationLoopExhausted
        );
        assert!(evidence.selected_trials.is_empty());
    }

    fn coupling_test_request(
        coupling_iteration: u32,
        ending_snow_hint: Option<CoveredTerminalEndingSnowHintV1>,
    ) -> CoveredTerminalTrialRequestV1 {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(600_000_000))
            .expect("test support");
        let beginning_joint = CoveredTerminalJointTrialStateV1::try_new(
            JointTrialAuthorityV1 {
                source_owner_set_sha256: Digest32::from_bytes([1; 32]),
                lane_id: 1,
                source_snow_owner_sha256: Digest32::from_bytes([2; 32]),
                interval_index: 0,
                state_support: support,
                accepted_predecessors: Vec::new(),
            },
            BTreeMap::from([
                ("vegetation".to_owned(), vec![1]),
                ("snow".to_owned(), vec![2]),
                ("land_surface_energy".to_owned(), vec![3]),
                ("hydrology".to_owned(), vec![4]),
                ("bgc".to_owned(), vec![5]),
                ("soil_thermal".to_owned(), vec![6]),
                ("surface_liquid".to_owned(), vec![7]),
            ]),
        )
        .expect("test joint");
        CoveredTerminalTrialRequestV1 {
            lane_id: 1,
            support,
            role: CoveredTerminalTrialRoleV1::Full,
            attempt_ordinal: 0,
            coupling_iteration,
            ice_kg_m2: 0.2,
            liquid_kg_m2: 0.01,
            cold_content_j_m2: -125.0,
            surface_temperature_c: -1.0e-9,
            snow_depth_m: 0.02,
            snow_density_kg_m3: 100.0,
            beginning_stage3_state: Box::new(
                Wb11HydrologyKernel::initialize_stage3_persistent_state_with_terminal_event(
                    1,
                    vec![DirectSnowLayerState {
                        mass_swe_m: 0.0002,
                        thickness_m: 0.002,
                        density_kg_m3: 100.0,
                        settle_day_count: 0.0,
                        temperature_c: -1.0,
                        liquid_water_m: 0.00001,
                        cold_content_j_m2: 420.0,
                        refrozen_liquid_m: 0.0,
                    }],
                    DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
                )
                .expect("test terminal state"),
            ),
            ending_snow_hint,
            beginning_joint,
        }
    }

    #[test]
    fn eb04w2b_storage_guard_enforces_exact_tolerance_and_nonfinite_rejection() {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        for residual_m in [
            -SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M,
            0.0,
            SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M,
        ] {
            Wb11HydrologyKernel::validate_direct_snow_storage_residual(phase_class, residual_m)
                .expect("exact-tolerance daily snow closure residual must be accepted");
        }

        for residual_m in [
            f64::from_bits(SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M.to_bits() + 1),
            -f64::from_bits(SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M.to_bits() + 1),
        ] {
            let error =
                Wb11HydrologyKernel::validate_direct_snow_storage_residual(phase_class, residual_m)
                    .expect_err("over-tolerance daily snow closure residual must fail closed");
            assert!(matches!(
                error,
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
            ));
            assert_eq!(error.code(), "HKERNEL-WB14-RUNOFF-E-003");
            assert!(
                error
                    .to_string()
                    .contains("snow.daily_storage_closure_residual_m")
            );
        }

        let error =
            Wb11HydrologyKernel::validate_direct_snow_storage_residual(phase_class, f64::NAN)
                .expect_err("non-finite daily snow closure residual must fail closed");
        assert!(matches!(
            error,
            Wb11HydrologyKernelGuardError::NonFiniteStateSymbol { .. }
        ));
        assert_eq!(error.code(), "HKERNEL-WB14-RUNOFF-E-002");
        assert!(
            error
                .to_string()
                .contains("snow.daily_storage_closure_residual_m")
        );
    }

    #[test]
    fn eb04c_lower_volume_threshold_is_strict_on_native_swe() {
        let threshold = STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
        let just_below = f64::from_bits(threshold.to_bits() - 1);
        let just_above = f64::from_bits(threshold.to_bits() + 1);

        assert!(Wb11HydrologyKernel::stage3_lower_volume_is_subresolution_swe_m(just_below));
        assert!(!Wb11HydrologyKernel::stage3_lower_volume_is_subresolution_swe_m(threshold));
        assert!(!Wb11HydrologyKernel::stage3_lower_volume_is_subresolution_swe_m(just_above));
    }

    #[test]
    fn partial_sublimation_retains_mass_resolved_subnanometer_swe_remainder() {
        let original_mass_swe_m = 1.0e-6;
        let represented_remainder_swe_m = 5.0e-10;
        let requested_m = original_mass_swe_m - represented_remainder_swe_m;
        let mut layer = DirectSnowLayerState::new(original_mass_swe_m, 2.0e-6, 500.0, 8.0);
        layer.liquid_water_m = 2.0e-7;
        layer.refrozen_liquid_m = 1.0e-7;
        let mut layers = vec![layer];
        let original_cold_content_j_m2 = 2.1;
        let mut cold_content_by_layer = vec![original_cold_content_j_m2];
        let mut active_layer_count = 1;

        let (removed_m, exported_j_m2, removed_layer_count) =
            Wb11HydrologyKernel::remove_stage3_active_sublimation(
                requested_m,
                &mut layers,
                &mut cold_content_by_layer,
                &mut active_layer_count,
            );

        assert_eq!(layers.len(), 1);
        assert_eq!(active_layer_count, 1);
        assert_eq!(removed_layer_count, 0);
        assert!(snow_density_layer_has_resolved_mass(layers[0].mass_swe_m));
        assert!((layers[0].mass_swe_m - represented_remainder_swe_m).abs() <= 1.0e-18);
        assert!((removed_m + layers[0].mass_swe_m - original_mass_swe_m).abs() <= 1.0e-18);
        assert!(
            (exported_j_m2 + cold_content_by_layer[0] - original_cold_content_j_m2).abs()
                <= 1.0e-12
        );
        assert!((layers[0].liquid_water_m - 1.0e-10).abs() <= 1.0e-18);
        assert!((layers[0].refrozen_liquid_m - 5.0e-11).abs() <= 1.0e-18);
    }

    #[test]
    fn subthreshold_sublimation_is_an_exact_material_debit() {
        let original_mass_swe_m = 1.834_693_035_757_419_8e-3;
        let requested_m = 9.716_528_046_178_411e-14;
        assert!(requested_m < WB11_ZERO_THRESHOLD);
        let mut layers = vec![DirectSnowLayerState::new(
            original_mass_swe_m,
            original_mass_swe_m * 10.0,
            100.0,
            0.0,
        )];
        let mut cold_content_by_layer = vec![0.0];
        let mut active_layer_count = 1;

        let (removed_m, exported_j_m2, removed_layer_count) =
            Wb11HydrologyKernel::remove_stage3_active_sublimation(
                requested_m,
                &mut layers,
                &mut cold_content_by_layer,
                &mut active_layer_count,
            );

        assert_eq!(removed_m.to_bits(), requested_m.to_bits());
        assert_eq!(exported_j_m2.to_bits(), 0.0_f64.to_bits());
        assert_eq!(removed_layer_count, 0);
        assert_eq!(active_layer_count, 1);
        assert!((layers[0].mass_swe_m + removed_m - original_mass_swe_m).abs() <= 1.0e-18);
    }

    #[test]
    fn stage3_target_trim_preserves_coupled_mass_resolved_remainder() {
        let original_mass_swe_m = 2.0e-6;
        let represented_remainder_swe_m = 5.0e-10;
        let removal_m = original_mass_swe_m - represented_remainder_swe_m;
        let mut surface = DirectSnowLayerState::new(original_mass_swe_m, 4.0e-6, 500.0, 9.0);
        surface.temperature_c = -4.0;
        surface.liquid_water_m = 4.0e-7;
        surface.cold_content_j_m2 = 16.8;
        surface.refrozen_liquid_m = 2.0e-7;
        let lower = DirectSnowLayerState::new(0.1, 0.2, 500.0, 20.0);
        let target_swe_m = surface.mass_swe_m + lower.mass_swe_m - removal_m;
        let mut layers = vec![surface, lower];

        Wb11HydrologyKernel::adjust_stage3_layer_swe_to_target(
            &mut layers,
            target_swe_m,
            0.2,
            500.0,
            20.0,
        );

        assert_eq!(layers.len(), 2);
        let retained = layers[0];
        let retained_fraction = retained.mass_swe_m / original_mass_swe_m;
        assert!((retained.mass_swe_m - represented_remainder_swe_m).abs() <= 1.0e-15);
        assert!(snow_density_layer_has_resolved_mass(retained.mass_swe_m));
        assert!(
            (retained.liquid_water_m - surface.liquid_water_m * retained_fraction).abs() <= 1.0e-18
        );
        assert!(
            (retained.refrozen_liquid_m - surface.refrozen_liquid_m * retained_fraction).abs()
                <= 1.0e-18
        );
        assert!(
            (retained.cold_content_j_m2 - surface.cold_content_j_m2 * retained_fraction).abs()
                <= 1.0e-15
        );
        assert_eq!(
            retained.density_kg_m3.to_bits(),
            surface.density_kg_m3.to_bits()
        );
        assert_eq!(
            retained.temperature_c.to_bits(),
            surface.temperature_c.to_bits()
        );
        assert_eq!(
            retained.settle_day_count.to_bits(),
            surface.settle_day_count.to_bits()
        );
        let reconstructed_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        assert!((reconstructed_swe_m - target_swe_m).abs() <= 1.0e-15);
    }

    #[test]
    fn stage3_target_trim_continues_below_residual_tolerance_across_layers() {
        let mut removed = DirectSnowLayerState::new(2.0e-6, 4.0e-6, 500.0, 9.0);
        removed.liquid_water_m = 4.0e-7;
        removed.cold_content_j_m2 = 16.8;
        removed.refrozen_liquid_m = 2.0e-7;
        let mut retained = DirectSnowLayerState::new(2.0e-9, 4.0e-9, 500.0, 12.0);
        retained.temperature_c = -3.0;
        retained.liquid_water_m = 8.0e-10;
        retained.cold_content_j_m2 = 4.2e-3;
        retained.refrozen_liquid_m = 4.0e-10;
        let target_swe_m = 1.5e-9;
        let mut layers = vec![removed, retained];

        Wb11HydrologyKernel::adjust_stage3_layer_swe_to_target(
            &mut layers,
            target_swe_m,
            3.0e-9,
            500.0,
            12.0,
        );

        assert_eq!(layers.len(), 1);
        let result = layers[0];
        let retained_fraction = 0.75;
        assert!((result.mass_swe_m - target_swe_m).abs() <= 1.0e-18);
        assert!(snow_density_layer_has_resolved_mass(result.mass_swe_m));
        assert!(
            (result.liquid_water_m - retained.liquid_water_m * retained_fraction).abs() <= 1.0e-18
        );
        assert!(
            (result.refrozen_liquid_m - retained.refrozen_liquid_m * retained_fraction).abs()
                <= 1.0e-18
        );
        assert!(
            (result.cold_content_j_m2 - retained.cold_content_j_m2 * retained_fraction).abs()
                <= 1.0e-15
        );
        assert_eq!(
            result.density_kg_m3.to_bits(),
            retained.density_kg_m3.to_bits()
        );
        assert_eq!(
            result.temperature_c.to_bits(),
            retained.temperature_c.to_bits()
        );
        assert_eq!(
            result.settle_day_count.to_bits(),
            retained.settle_day_count.to_bits()
        );
    }

    #[test]
    fn snow_density_guard_error_maps_all_error_variants() {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let replay_layers = [DirectSnowLayerState::new(0.2, 0.4, 500.0, 2.0)];
        let cases = [
            SnowDensityError::NonFiniteInput {
                symbol: "row5.nonfinite",
                value: f64::NAN,
            },
            SnowDensityError::OutOfRangeInput {
                symbol: "row5.range",
                value: -1.0,
                minimum: Some(0.0),
                maximum: Some(1.0),
            },
            SnowDensityError::MissingClimateClassAssignment { model: "sturm2010" },
            SnowDensityError::MissingSturmDayOfYear { model: "sturm2010" },
            SnowDensityError::MissingClimateClassDensityParameters { class: "alpine" },
            SnowDensityError::LayerAggregateMismatch {
                symbol: "prior_layers.thickness_m",
                value: 0.4,
                expected: 0.5,
            },
            SnowDensityError::DiagnosticClosureViolation {
                residual_kg_m3: 2.0e-9,
                tolerance_kg_m3: 1.0e-9,
            },
        ];

        let mapped = cases
            .iter()
            .map(|error| {
                Wb11HydrologyKernel::snow_density_guard_error(
                    phase_class,
                    error,
                    0.2,
                    0.5,
                    &replay_layers,
                )
            })
            .collect::<Vec<_>>();

        assert!(matches!(
            mapped[0],
            Wb11HydrologyKernelGuardError::NonFiniteStateSymbol { .. }
        ));
        assert!(matches!(
            mapped[1],
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
        ));
        assert!(matches!(
            mapped[2],
            Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol { .. }
        ));
        assert!(mapped[2].to_string().contains("snow_climate_class"));
        assert!(
            mapped[3]
                .to_string()
                .contains("sturm2010_density_day_of_year")
        );
        assert!(
            mapped[4]
                .to_string()
                .contains("sturm2010_density_parameters")
        );
        assert!(matches!(
            mapped[5],
            Wb11HydrologyKernelGuardError::SnowLayerAggregateMismatch(_)
        ));
        assert!(matches!(
            mapped[6],
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
        ));
        if let Wb11HydrologyKernelGuardError::SnowLayerAggregateMismatch(snapshot) = &mapped[5] {
            assert!((snapshot.replay_value() - snapshot.value).abs() <= f64::EPSILON);
            assert!((snapshot.replay_value() - snapshot.expected).abs() > f64::EPSILON);
            assert!((snapshot.expected - snapshot.prior_depth_m).abs() <= f64::EPSILON);
            let replay_swe_m = snapshot
                .prior_layers
                .iter()
                .map(|layer| layer.mass_swe_m)
                .sum::<f64>();
            assert!((replay_swe_m - snapshot.prior_swe_m).abs() <= f64::EPSILON);
        }
        assert!(
            mapped[5]
                .to_string()
                .contains("prior_layers.thickness_m=0.4 does not match expected 0.5")
        );
    }
}
