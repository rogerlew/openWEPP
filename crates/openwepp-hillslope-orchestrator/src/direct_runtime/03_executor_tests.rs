#[cfg(test)]
mod cqr_executor_tests {
    use super::*;

    fn day_frame(day_index: usize, day_count: usize) -> DirectDayFrame {
        let identity = DirectRunIdentity::new(91, 501, 2, day_count)
            .expect("valid executor characterization identity");
        DirectDayFrame::seed(identity, 0, day_index).expect("valid executor day frame")
    }

    fn calendar_input() -> DirectPublicationDayInput {
        DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
            year: 2026,
            julian_day: 1,
            month: 1,
            day_of_month: 1,
            water_year: 2026,
        })
    }

    #[test]
    fn cqr_apply_publication_input_covers_optional_payloads_and_guard_priority() {
        let mut day = day_frame(0, 1);
        let mut input = calendar_input();
        input.precipitation_m = 0.01;
        input.effective_temperature_c = 2.0;
        input.interception_m = 0.001;
        input.canopy_cover_fraction = Some(0.5);
        input.initial_soil_water_m = Some(0.2);
        input.storage_input_inputs = Some(day.storage_input_inputs);
        input.liquid_input_inputs = Some(day.liquid_input_inputs);
        input.percolation_inputs = Some(DirectPercolationInputs::neutral());
        input.infiltration_depression_inputs = Some(day.infiltration_depression_inputs.clone());
        input.subsurface_compute_inputs = Some(DirectSubsurfaceComputeInputs::neutral());
        input.decomposition_inputs = Some(day.decomposition_inputs);
        input.residue_partition_inputs = Some(day.residue_partition_inputs);
        input.annual_growth_inputs = Some(day.annual_growth_inputs);
        input.perennial_growth_inputs = Some(day.perennial_growth_inputs);
        let mut et = day.evapotranspiration_compute_inputs.clone();
        et.stage_state = None;
        input.evapotranspiration_compute_inputs = Some(et);
        input.snow_coupling_inputs = Some(day.snow_coupling_inputs.clone());
        input.hydrology_projection_inputs = Some(day.hydrology_projection_inputs);
        input.erosion_inputs = Some(day.erosion_inputs.clone());
        input.frost_storage_liquid_delta_m = Some(-0.001);
        input.frost_layer_carry_projection = Some(Vec::new());
        DirectFrameExecutor::apply_publication_day_input(&mut day, &input)
            .expect("complete optional publication payload");
        assert!((day.forcing.precipitation_m - 0.01).abs() < f64::EPSILON);
        assert!((day.water.soil_water_m - 0.2).abs() < f64::EPSILON);
        assert_eq!(day.frost_storage_liquid_delta_m, Some(-0.001));

        let mut fallback = calendar_input();
        fallback.precipitation_m = 0.02;
        DirectFrameExecutor::apply_publication_day_input(&mut day, &fallback)
            .expect("precipitation fallback liquid handoff");
        assert!((day.liquid_input_inputs.liquid_input_handoff_m - 0.02).abs() < f64::EPSILON);

        let mut invalid = calendar_input();
        invalid.precipitation_m = -1.0;
        invalid.effective_temperature_c = f64::NAN;
        assert!(matches!(
            DirectFrameExecutor::apply_publication_day_input(&mut day, &invalid),
            Err(DirectRuntimeError::NegativeDirectValue {
                field: "publication_input.precipitation_m"
            })
        ));
        invalid.precipitation_m = 0.0;
        assert!(matches!(
            DirectFrameExecutor::apply_publication_day_input(&mut day, &invalid),
            Err(DirectRuntimeError::NonFiniteDirectValue {
                field: "publication_input.effective_temperature_c"
            })
        ));
        invalid.effective_temperature_c = 0.0;
        invalid.canopy_cover_fraction = Some(1.1);
        assert!(matches!(
            DirectFrameExecutor::apply_publication_day_input(&mut day, &invalid),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "publication_input.canopy_cover_fraction"
            })
        ));
        invalid.canopy_cover_fraction = Some(f64::NAN);
        assert!(matches!(
            DirectFrameExecutor::apply_publication_day_input(&mut day, &invalid),
            Err(DirectRuntimeError::NonFiniteDirectValue {
                field: "publication_input.canopy_cover_fraction"
            })
        ));
        invalid.canopy_cover_fraction = None;
        invalid.initial_soil_water_m = Some(-0.1);
        assert!(matches!(
            DirectFrameExecutor::apply_publication_day_input(&mut day, &invalid),
            Err(DirectRuntimeError::NegativeDirectValue {
                field: "publication_input.initial_soil_water_m"
            })
        ));
        invalid.initial_soil_water_m = None;
        invalid.frost_storage_liquid_delta_m = Some(f64::NAN);
        assert!(matches!(
            DirectFrameExecutor::apply_publication_day_input(&mut day, &invalid),
            Err(DirectRuntimeError::NonFiniteDirectValue {
                field: "publication_input.frost_storage_liquid_delta_m"
            })
        ));
    }

    #[test]
    fn cqr_executor_mode_and_phase_status_cover_both_lifecycle_arms() {
        let executor = DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect);
        assert_eq!(executor.mode(), DirectExecutorMode::ProductionDirect);

        let mut counters = DirectExecutionCounters::default();
        counters.record_phase_status(
            DirectPhaseKind::Normalization,
            DirectPhaseLifecycleStatus::Executed,
        );
        counters.record_phase_status(
            DirectPhaseKind::StorageBounds,
            DirectPhaseLifecycleStatus::Hold,
        );
        let counts = counters.phase_status_counts();
        assert!(counts.iter().any(|count| {
            count.phase == DirectPhaseKind::Normalization
                && count.status == DirectPhaseLifecycleStatus::Executed
                && count.count == 1
        }));
        assert!(counts.iter().any(|count| {
            count.phase == DirectPhaseKind::StorageBounds
                && count.status == DirectPhaseLifecycleStatus::Hold
                && count.count == 1
        }));
    }

    #[test]
    fn cqr_active_selector_and_first_day_empty_layer_payloads_take_guarded_branches() {
        let identity = DirectRunIdentity::new(94, 501, 1, 1)
            .expect("valid selector characterization identity");
        let mut frame = DirectRunFrame::skeleton(identity).expect("selector frame");
        frame.laned_active = Some(Box::new(laned_active::DirectLanedActiveConfig {
            lanes: vec![laned_active::DirectLanedActiveLaneConfig {
                slplen_m: 10.0,
                width_m: 10.0,
                mean_gradient: 0.01,
                skin_friction_coefficient_ko: 500.0,
                form_drag_coefficient: 0.0,
                roughness_element_height_m: 0.0,
                roughness_concentration: 0.0,
                vegetation_drag_coefficient: 0.0,
                canopy_height_m: None,
            }],
            mesh_policy: laned_active::DirectLanedActiveMeshPolicy::FixedCells { cells: 10 },
            max_dt_s: 300.0,
            trace_enabled: false,
            trace_detail_filter: None,
            step_trace_enabled: false,
        }));
        assert!(matches!(
            DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly).run_skeleton(&mut frame),
            Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_selector",
                ..
            })
        ));

        let mut day = day_frame(0, 1);
        day.percolation_inputs = DirectPercolationInputs::neutral();
        day.subsurface_compute_inputs = DirectSubsurfaceComputeInputs::neutral();
        let mut input = calendar_input();
        let mut percolation = DirectPercolationInputs::neutral();
        percolation.layers.clear();
        input.percolation_inputs = Some(percolation);
        let mut subsurface = DirectSubsurfaceComputeInputs::neutral();
        subsurface.layers.clear();
        input.subsurface_compute_inputs = Some(subsurface);
        DirectFrameExecutor::apply_publication_day_input(&mut day, &input)
            .expect("first-day empty layers inherit seeded layers");
        assert_eq!(day.percolation_inputs.layers.len(), 1);
        assert_eq!(day.subsurface_compute_inputs.layers.len(), 1);
    }

    #[test]
    fn cqr_publication_layer_inputs_cover_carry_forward_and_cardinality_guards() {
        let mut day = day_frame(1, 2);
        day.percolation_inputs = DirectPercolationInputs::neutral();
        day.subsurface_compute_inputs = DirectSubsurfaceComputeInputs::neutral();
        day.water.soil_water_m = 0.25;

        let mut input = calendar_input();
        let mut percolation = DirectPercolationInputs::neutral();
        percolation.layers.clear();
        input.percolation_inputs = Some(percolation);
        let mut subsurface = DirectSubsurfaceComputeInputs::neutral();
        subsurface.layers.clear();
        input.subsurface_compute_inputs = Some(subsurface);
        DirectFrameExecutor::apply_publication_day_input(&mut day, &input)
            .expect("empty later-day layer payloads carry forward");
        assert_eq!(day.percolation_inputs.layers.len(), 1);
        assert!((day.percolation_inputs.soil_water_initial_m - 0.25).abs() < f64::EPSILON);
        assert_eq!(day.subsurface_compute_inputs.layers.len(), 1);

        let mut bad_percolation = DirectPercolationInputs::neutral();
        bad_percolation
            .layers
            .push(DirectSubsurfaceLayerState::neutral());
        input.percolation_inputs = Some(bad_percolation);
        assert!(matches!(
            DirectFrameExecutor::apply_publication_percolation_input(&mut day, &input),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "publication_input.percolation_layers"
            })
        ));

        let mut bad_subsurface = DirectSubsurfaceComputeInputs::neutral();
        bad_subsurface
            .layers
            .push(DirectSubsurfaceLayerInputs::neutral());
        input.subsurface_compute_inputs = Some(bad_subsurface);
        assert!(matches!(
            DirectFrameExecutor::apply_publication_subsurface_input(&mut day, &input),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "publication_input.subsurface_layers"
            })
        ));
    }

    fn erosion_publisher_fixture() -> (DirectRunFrame, DirectDayFrame) {
        let identity =
            DirectRunIdentity::new(92, 501, 2, 1).expect("valid erosion publisher identity");
        let frame = DirectRunFrame::skeleton(identity).expect("erosion publisher frame");
        let mut day = DirectDayFrame::seed(identity, 0, 0).expect("erosion publisher day");
        day.erosion_inputs.wave1_operand_seed.enabled = true;
        day.erosion_inputs.wave1_operand_seed.efflen_m = 2.0;
        day.erosion_inputs.wave1_operand_seed.field_width_m = 4.0;
        day.wave1_hourly_plan
            .push((0, DirectWave1ContinuityInputs::zero()));
        (frame, day)
    }

    #[test]
    fn cqr_erosion_inflow_publisher_covers_errors_and_downstream_publication() {
        let (mut frame, mut day) = erosion_publisher_fixture();
        let mut inactive = day.clone();
        inactive.erosion_inputs.wave1_operand_seed.enabled = false;
        assert!(
            !DirectFrameExecutor::publish_erosion_inflow_to_downstream(&mut frame, &inactive)
                .expect("inactive erosion lane")
        );
        let downstream_lane_id = frame.lanes[0].downstream_lane_id;
        frame.lanes[0].downstream_lane_id = 0;
        assert!(
            !DirectFrameExecutor::publish_erosion_inflow_to_downstream(&mut frame, &day)
                .expect("terminal erosion lane")
        );
        frame.lanes[0].downstream_lane_id = downstream_lane_id;
        assert!(matches!(
            DirectFrameExecutor::publish_erosion_inflow_to_downstream(&mut frame, &day),
            Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "E.3 erosion inflow publisher peak-runoff producer"
            })
        ));
        day.peak_runoff_shadow_projection = Some(DirectPeakRunoffShadowProjection {
            lane_index: 0,
            day_index: 0,
            q_runoff_m: 0.024,
            peak_runoff_rate_m_s: 1.0,
            runoff_duration_s: 3600.0,
            peak_hour_index: Some(0),
            method_branch: 1.0,
            tstar: 0.0,
            qpstar: 0.0,
            vstar: 0.0,
        });
        assert!(matches!(
            DirectFrameExecutor::publish_erosion_inflow_to_downstream(&mut frame, &day),
            Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "E.3 erosion inflow publisher continuity state"
            })
        ));
        day.erosion.wave1_continuity = Some(Box::new(DirectWave1ContinuityState::inactive()));
        assert!(matches!(
            DirectFrameExecutor::publish_erosion_inflow_to_downstream(&mut frame, &day),
            Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "E.3 erosion inflow publisher hourly sediment surface"
            })
        ));
        day.erosion_downstream_operands
            .publication
            .hourly_sediment_mass_kg = Some([3.6; 24]);
        day.wave1_hourly_weights = [1.0 / 24.0; 24];
        assert!(
            DirectFrameExecutor::publish_erosion_inflow_to_downstream(&mut frame, &day)
                .expect("erosion inflow publication")
        );
        let intake = frame.lanes[1]
            .erosion_inflow_intake
            .as_ref()
            .expect("downstream erosion intake");
        assert!((intake.hourly_qsout_kg_m_s[0] - 0.00025).abs() < f64::EPSILON);
        assert!(
            intake
                .exit_fractions
                .iter()
                .all(|value| value.abs() < f64::EPSILON)
        );

        day.erosion_downstream_operands
            .publication
            .sediment_concentration_kg_m3 = Some([1.0, 1.0, 2.0, 0.0, 0.0]);
        DirectFrameExecutor::publish_erosion_inflow_to_downstream(&mut frame, &day)
            .expect("positive concentration publication");
        let expected = [0.25, 0.25, 0.5, 0.0, 0.0];
        let actual = frame.lanes[1]
            .erosion_inflow_intake
            .as_ref()
            .expect("updated intake")
            .exit_fractions;
        assert!(
            actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| (actual - expected).abs() < f64::EPSILON)
        );

        day.erosion_inputs.wave1_operand_seed.field_width_m = 0.0;
        assert!(matches!(
            DirectFrameExecutor::publish_erosion_inflow_to_downstream(&mut frame, &day),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.inflow_publisher.geometry"
            })
        ));
    }

    #[test]
    fn cqr_day_execution_failure_preserves_context_and_runoff_diagnostics() {
        let mut day = day_frame(0, 1);
        let ordinary = DirectFrameExecutor::day_execution_failure(
            &day,
            1,
            2,
            &DirectRuntimeError::DirectDomainViolation { field: "ordinary" },
        );
        assert!(matches!(
            ordinary,
            DirectRuntimeError::DirectDayExecutionFailure {
                lane_index: 1,
                day_index: 2,
                ..
            }
        ));

        day.runoff_partition_inputs.liquid_input_m = 0.1;
        day.runoff_partition_inputs.runon_input_m = 0.2;
        day.storage_reconciliation_inputs.precip_input_m = 0.3;
        let detailed = DirectFrameExecutor::day_execution_failure(
            &day,
            0,
            0,
            &DirectRuntimeError::NegativeDirectValue {
                field: "runoff_partition.partition_runoff_m",
            },
        );
        let DirectRuntimeError::DirectDayExecutionFailure { detail, .. } = detailed else {
            panic!("expected wrapped day failure")
        };
        assert!(detail.contains("liquid_input_m=0.1"));
        assert!(detail.contains("runon_input_m=0.2"));
        assert!(detail.contains("storage_precip_input_m=0.3"));
    }

    #[test]
    fn cqr_stage3_active_day_defers_to_the_exact_committed_owner() {
        assert!(DirectFrameExecutor::ordinary_laned_active_stream_selected(
            true, false,
        ));
        assert!(
            !DirectFrameExecutor::ordinary_laned_active_stream_selected(true, true),
            "a constitutive Stage-3 day must not execute ordinary WB16/Lane D before commit",
        );
        assert!(!DirectFrameExecutor::ordinary_laned_active_stream_selected(
            false, true,
        ));
        assert!(!DirectFrameExecutor::ordinary_laned_active_stream_selected(
            false, false,
        ));
    }

    #[test]
    fn laned_active_public_hourly_pair_preserves_all_24_bins_and_peak() {
        let mut routed = [0.0; 24];
        routed[2] = 0.125;
        routed[7] = 0.625;
        routed[19] = 0.25;
        let mut publication = DirectPublicationErosionOperands::zero_authority();

        DirectFrameExecutor::bind_laned_active_public_hourly_pair(
            Some(routed),
            false,
            &mut publication,
        )
        .expect("authenticated routed surface binds to public HBP operands");

        let actual = publication
            .hourly_runoff_fraction
            .expect("public hourly runoff surface");
        assert_eq!(actual.map(f64::to_bits), routed.map(f64::to_bits));
        assert_eq!(
            actual
                .iter()
                .enumerate()
                .max_by(|left, right| left.1.total_cmp(right.1))
                .map(|(hour, _)| hour),
            Some(7),
            "the authenticated routed peak hour must remain unchanged",
        );
        assert_eq!(publication.hourly_sediment_mass_kg, Some([0.0; 24]));
    }

    #[test]
    fn laned_active_public_hourly_pair_preserves_matching_nonzero_sediment() {
        let routed = [1.0 / 24.0; 24];
        let sediment = core::array::from_fn(|hour| hour as f64 + 0.25);
        let mut publication = DirectPublicationErosionOperands::zero_authority();
        publication.hourly_runoff_fraction = Some(routed);
        publication.hourly_sediment_mass_kg = Some(sediment);

        DirectFrameExecutor::bind_laned_active_public_hourly_pair(
            Some(routed),
            true,
            &mut publication,
        )
        .expect("matching Wave-1 surfaces remain authoritative");

        assert_eq!(publication.hourly_runoff_fraction, Some(routed));
        assert_eq!(publication.hourly_sediment_mass_kg, Some(sediment));
    }

    #[test]
    fn laned_active_public_hourly_pair_mismatch_rolls_back() {
        let routed = [1.0 / 24.0; 24];
        let mut publication = DirectPublicationErosionOperands::zero_authority();
        publication.hourly_runoff_fraction = Some([0.0; 24]);
        publication.hourly_sediment_mass_kg = Some([0.5; 24]);
        let before = publication;

        assert!(matches!(
            DirectFrameExecutor::bind_laned_active_public_hourly_pair(
                Some(routed),
                true,
                &mut publication,
            ),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "laned_active.publication.hourly_runoff_fraction"
            })
        ));
        assert_eq!(publication, before, "a mismatch must not mutate the row");
    }

    #[test]
    fn laned_active_public_hourly_pair_missing_sources_roll_back() {
        let routed = [1.0 / 24.0; 24];
        let mut publication = DirectPublicationErosionOperands::zero_authority();
        let before = publication;

        assert!(matches!(
            DirectFrameExecutor::bind_laned_active_public_hourly_pair(
                None,
                false,
                &mut publication,
            ),
            Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "laned_active authenticated routed hourly surface"
            })
        ));
        assert_eq!(
            publication, before,
            "missing routing must not mutate the row"
        );

        assert!(matches!(
            DirectFrameExecutor::bind_laned_active_public_hourly_pair(
                Some(routed),
                true,
                &mut publication,
            ),
            Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "laned_active Wave-1 hourly sediment surface"
            })
        ));
        assert_eq!(
            publication, before,
            "missing Wave-1 sediment must not install routed runoff alone"
        );
    }

    #[test]
    fn laned_active_public_hourly_pair_invalid_sediment_rolls_back() {
        let routed = [1.0 / 24.0; 24];
        let mut publication = DirectPublicationErosionOperands::zero_authority();
        publication.hourly_sediment_mass_kg = Some([0.0; 24]);
        publication
            .hourly_sediment_mass_kg
            .as_mut()
            .expect("surface")[11] = -0.25;
        let before = publication;

        assert!(matches!(
            DirectFrameExecutor::bind_laned_active_public_hourly_pair(
                Some(routed),
                true,
                &mut publication,
            ),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "laned_active.publication.hourly_sediment_mass_kg"
            })
        ));
        assert_eq!(
            publication, before,
            "invalid nonzero sediment must not partially install runoff"
        );
    }

    fn laned_active_public_routing_fixture(
        source_m3: f64,
        outlet_m3: f64,
        mesh_end_storage_m3: f64,
        clamp_m3: f64,
        routed_weights: [f64; 24],
    ) -> laned_active::DirectLanedActiveDayRouting {
        laned_active::DirectLanedActiveDayRouting {
            canopy_height_m_consumed: Some(1.5),
            source_m3,
            outlet_m3,
            mesh_end_storage_m3,
            clamp_m3,
            tail_fold_m3: 0.0,
            routed_weights,
            uniform_shape: false,
            erosion_source_shape_degenerate: false,
            trace_detail: None,
        }
    }

    fn laned_active_public_runoff_fixture() -> DirectPublicationRunoffOperands {
        DirectPublicationRunoffOperands {
            q_mm: 7.0,
            qofe_mm: 8.0,
            runvol_m3: 9.0,
            peak_runoff_m3_s: Some(10.0),
            runoff_duration_s: Some(11.0),
        }
    }

    #[test]
    fn laned_active_terminal_public_runoff_uses_exact_outlet_and_hourly_peak() {
        let mut weights = [0.0; 24];
        weights[3] = 0.25;
        weights[17] = 0.75;
        let routing = laned_active_public_routing_fixture(12.0, 12.0, 0.0, 0.0, weights);
        let mut runoff = laned_active_public_runoff_fixture();

        DirectFrameExecutor::bind_laned_active_public_runoff_operands(&routing, true, &mut runoff)
            .expect("terminal routed public runoff");

        assert_eq!(runoff.q_mm.to_bits(), 7.0_f64.to_bits());
        assert_eq!(runoff.qofe_mm.to_bits(), 8.0_f64.to_bits());
        assert_eq!(runoff.runoff_duration_s, Some(11.0));
        assert_eq!(runoff.runvol_m3.to_bits(), 12.0_f64.to_bits());
        assert_eq!(
            runoff.peak_runoff_m3_s.map(f64::to_bits),
            Some((12.0_f64 * 0.75 / 3_600.0).to_bits()),
        );
    }

    #[test]
    fn laned_active_nonterminal_public_runoff_preserves_accepted_scalars() {
        let routing = laned_active_public_routing_fixture(2.0, 2.0, 0.0, 0.0, [0.0; 24]);
        let mut runoff = laned_active_public_runoff_fixture();
        let before = runoff;

        DirectFrameExecutor::bind_laned_active_public_runoff_operands(&routing, false, &mut runoff)
            .expect("nonterminal accepted runoff remains unchanged");

        assert_eq!(runoff, before);
    }

    #[test]
    fn laned_active_terminal_public_runoff_invalid_weights_roll_back() {
        let routing = laned_active_public_routing_fixture(2.0, 2.0, 0.0, 0.0, [0.01; 24]);
        let mut runoff = laned_active_public_runoff_fixture();
        let before = runoff;

        assert!(matches!(
            DirectFrameExecutor::bind_laned_active_public_runoff_operands(
                &routing,
                true,
                &mut runoff,
            ),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "laned_active.publication.hourly_runoff_fraction_sum"
            })
        ));
        assert_eq!(runoff, before, "invalid weights must not mutate runoff");
    }

    #[test]
    fn laned_active_public_day_ledger_closes_storage_and_clamp_or_rejects() {
        let upper = laned_active_public_routing_fixture(0.25, 0.25, 0.03125, 0.0625, [0.0; 24]);
        let mut terminal_weights = [0.0; 24];
        terminal_weights[9] = 1.0;
        let terminal =
            laned_active_public_routing_fixture(0.5, 0.8125, 0.09375, 0.125, terminal_weights);
        DirectFrameExecutor::validate_laned_active_public_day_ledger(&[&upper, &terminal])
            .expect("source plus clamp closes to terminal outlet plus storage");

        let invalid_terminal =
            laned_active_public_routing_fixture(0.5, 0.75, 0.09375, 0.125, terminal_weights);
        assert!(matches!(
            DirectFrameExecutor::validate_laned_active_public_day_ledger(&[
                &upper,
                &invalid_terminal,
            ]),
            Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "laned_active.publication.source_clamp_outlet_storage"
            })
        ));
    }

    #[test]
    fn stage3_publication_requires_routing_only_for_the_authoritative_active_posture() {
        DirectFrameExecutor::validate_optional_laned_active_public_day_ledger(
            &[None, None],
            false,
        )
        .expect("inactive Stage-3 publication has no Lane-D ledger");
        assert!(matches!(
            DirectFrameExecutor::validate_optional_laned_active_public_day_ledger(&[None], true),
            Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "laned_active authenticated routed public day ledger"
            })
        ));

        let mut weights = [0.0; 24];
        weights[0] = 1.0;
        let active = laned_active_public_routing_fixture(1.0, 1.0, 0.0, 0.0, weights);
        DirectFrameExecutor::validate_optional_laned_active_public_day_ledger(
            &[Some(&active)],
            true,
        )
        .expect("active Stage-3 publication requires and validates the routed ledger");
        assert!(matches!(
            DirectFrameExecutor::validate_optional_laned_active_public_day_ledger(
                &[Some(&active)],
                false,
            ),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "publication.unexpected_inactive_laned_active_routing"
            })
        ));
    }

    #[test]
    fn cqr_day_execution_failure_enriches_aggregate_storage_context() {
        let identity =
            DirectRunIdentity::new(93, 501, 1, 1).expect("valid aggregate diagnostic identity");
        let mut frame = DirectRunFrame::skeleton(identity).expect("aggregate diagnostic frame");
        frame.lanes[0].area_m2 = 100.0;
        let input = calendar_input();
        let mut enriched = false;
        DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
            .run_publication_stream_with_interleaved_day_inputs_and_day_frames(
                &mut frame,
                DirectPublicationRunMetadata {
                    run_name: "cqr_aggregate_diagnostic".to_string(),
                    runtime_selection: "direct".to_string(),
                    output_policy: "test".to_string(),
                },
                |_, _, _| Ok(input.clone()),
                |_, day_frame| {
                    let wrapped = DirectFrameExecutor::day_execution_failure(
                        day_frame,
                        0,
                        0,
                        &DirectRuntimeError::DirectClosureToleranceExceeded {
                            field: "hydrology_projection.aggregate_storage_delta_m",
                        },
                    );
                    let DirectRuntimeError::DirectDayExecutionFailure { detail, .. } = wrapped
                    else {
                        panic!("expected wrapped aggregate diagnostic")
                    };
                    enriched = detail.contains("aggregate_storage_from_layers_m=")
                        && detail.contains("frozen_layer_storage_m=");
                    Ok(())
                },
            )
            .expect("aggregate diagnostic publication stream");
        assert!(enriched);
    }
}
