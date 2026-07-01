#[cfg(test)]
mod tests {
    use super::*;
    use crate::SidecarPolicy;
    use crate::hillslope::HillslopeDirectClimateDayForcing;
    use openwepp_hillslope_orchestrator::{
        DIRECT_R3B_PHASE_SPAN_COUNT, DIRECT_R3C_PHASE_SPAN_COUNT,
        DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT, DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT,
        DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT, DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT,
        DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT,
        DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT,
        DIRECT_R4A_PHASE_SPAN_COUNT, DIRECT_R4B_PHASE_SPAN_COUNT, DIRECT_R4C_PHASE_SPAN_COUNT,
        DIRECT_R4G_PHASE_SPAN_COUNT, DIRECT_R4I_PHASE_SPAN_COUNT,
        DIRECT_R4J_PHASE_SPAN_COUNT, DIRECT_R4K_PHASE_SPAN_COUNT, DIRECT_R4L_PHASE_SPAN_COUNT,
        DIRECT_R4M_PHASE_SPAN_COUNT, DIRECT_R4N_PHASE_SPAN_COUNT,
        DIRECT_R4O_PHASE_SPAN_COUNT, DIRECT_R4PQZ_PHASE_SPAN_COUNT,
        reset_direct_runtime_audit_counters,
        DirectPublicationCalendarDay, DirectPublicationClimateOperands, DirectPublicationDayRow,
        DirectPublicationErosionOperands, DirectPublicationEvaporationOperands,
        DirectPublicationInterceptionOperands, DirectPublicationLiquidInputOperands,
        DirectPublicationProfileOperands, DirectPublicationRunMetadata,
        DirectPublicationRunoffOperands, DirectPublicationStorageOperands,
        DirectPublicationSubsurfaceOperands, DirectPublicationTransferOperands,
        DirectPublicationWaterTemperatureOperands, DirectRunIdentity, DirectRunPublicationFrame,
    };
    use openwepp_input_contract::parsers::hbp::{HbpParseOptions, parse_hbp_from_path};
    use openwepp_input_contract::parsers::slope::{
        DatverSource, DistanceMode, SlopeOfe, SlopePoint, SlopeProfile,
    };
    use std::fs;
    use std::path::Path;
    use std::sync::{Mutex, OnceLock};

    mod simimpl {
        include!("tests03/simimpl.rs");
    }
    include!("tests03/direct_publication_source_guards.rs");



    fn canonical_calendar_day_probe() -> ClimateDayProjection {
        ClimateDayProjection {
            year: 2000,
            month: 1,
            day_of_month: 1,
            julian_day: 1,
            precipitation_mm: 4.0,
            effective_temperature_c: 2.0,
        }
    }

    fn execute_fixture_run(prefix: &str) -> (HillslopeRunReport, PathBuf) {
        execute_fixture_run_with_runtime_selection(
            prefix,
            HillslopeRuntimeSelection::DirectProductionExecutor,
        )
    }

    fn execute_fixture_run_with_runtime_selection(
        prefix: &str,
        runtime_selection: HillslopeRuntimeSelection,
    ) -> (HillslopeRunReport, PathBuf) {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        execute_fixture_run_with_runtime_selection_unlocked(prefix, runtime_selection)
    }

    fn execute_fixture_run_with_runtime_selection_unlocked(
        prefix: &str,
        runtime_selection: HillslopeRuntimeSelection,
    ) -> (HillslopeRunReport, PathBuf) {
        execute_fixture_run_with_runtime_policy_unlocked(
            prefix,
            HillslopeRuntimeSelectionPolicy::new(
                runtime_selection,
                HillslopeDefaultRuntimeActivation::default(),
            ),
        )
    }

    fn execute_fixture_run_with_runtime_policy_unlocked(
        prefix: &str,
        runtime_policy: HillslopeRuntimeSelectionPolicy,
    ) -> (HillslopeRunReport, PathBuf) {
        execute_fixture_run_with_runtime_policy_and_legacy_discovery_unlocked(
            prefix,
            runtime_policy,
            false,
        )
    }

    fn execute_fixture_run_with_runtime_policy_and_legacy_discovery_unlocked(
        prefix: &str,
        runtime_policy: HillslopeRuntimeSelectionPolicy,
        legacy_sidecar_discovery: bool,
    ) -> (HillslopeRunReport, PathBuf) {
        reset_direct_runtime_audit_counters();

        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);
        let output_dir = temp_run_dir.join("output");

        let report = execute_hillslope_run_with_runtime_policy(
            &HillslopeRunRequest {
                run_dir: temp_run_dir.clone(),
                run_file: PathBuf::from("case.run"),
                output_dir,
                sidecar_policy: SidecarPolicy::Compat,
                legacy_sidecar_discovery,
                manifest_path: None,
            },
            &["openwepp-cli-hill".to_string()],
            runtime_policy,
        )
        .expect("fixture run should complete");

        (report, temp_run_dir)
    }

    fn r5c_day_span_run_count() -> u64 {
        22
    }

    fn r5c_day_phase_entry_count() -> u64 {
        (DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT
            + DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT
            + DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT
            + DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT
            + DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT
            + DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT
            + DIRECT_R4A_PHASE_SPAN_COUNT
            + 3
            + DIRECT_R4B_PHASE_SPAN_COUNT
            + DIRECT_R4C_PHASE_SPAN_COUNT
            + DIRECT_R4G_PHASE_SPAN_COUNT
            + DIRECT_R4I_PHASE_SPAN_COUNT
            + DIRECT_R4J_PHASE_SPAN_COUNT
            + DIRECT_R4K_PHASE_SPAN_COUNT
            + DIRECT_R4L_PHASE_SPAN_COUNT
            + DIRECT_R4M_PHASE_SPAN_COUNT
            + DIRECT_R4N_PHASE_SPAN_COUNT
            + DIRECT_R4O_PHASE_SPAN_COUNT
            + DIRECT_R4PQZ_PHASE_SPAN_COUNT
            + 3
            + DIRECT_R3B_PHASE_SPAN_COUNT) as u64
    }

    #[test]
    fn r7e_default_candidate_uses_direct_production_manifest() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (report, _temp_run_dir) = execute_fixture_run_with_runtime_policy_unlocked(
            "r7e_default_candidate_direct_default",
            HillslopeRuntimeSelectionPolicy::default(),
        );

        assert!(report.output_pass.is_file());
        assert!(report.output_loss.is_file());
        let manifest_json = read_manifest_json(&report);
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/requested")
                .and_then(serde_json::Value::as_str),
            Some("default-candidate")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/selected")
                .and_then(serde_json::Value::as_str),
            Some("direct-production-executor")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/default_activation_gate")
                .and_then(serde_json::Value::as_str),
            Some("direct-production-candidate")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/output_policy")
                .and_then(serde_json::Value::as_str),
            Some("direct-production-executor/direct-publication-frame")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/compatibility_rollback_available")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/rollback_runtime")
                .and_then(serde_json::Value::as_str),
            Some("none")
        );
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/scheduler_kernel_executed")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
        assert!(
            manifest_json
                .pointer("/runtime_selection/fallback_reason")
                .is_none()
                || manifest_json
                    .pointer("/runtime_selection/fallback_reason")
                    .is_some_and(serde_json::Value::is_null),
            "default direct production should not report a fallback reason"
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/compatibility_edge_invocations",
            0,
        );
    }

    #[test]
    fn r7e_default_candidate_activation_selects_direct_runtime_manifest() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (report, _temp_run_dir) = execute_fixture_run_with_runtime_policy_unlocked(
            "r7e_default_candidate_activation",
            HillslopeRuntimeSelectionPolicy::new(
                HillslopeRuntimeSelection::DefaultCandidate,
                HillslopeDefaultRuntimeActivation::DirectProductionCandidate,
            ),
        );

        assert!(report.output_pass.is_file());
        assert!(report.output_loss.is_file());
        let manifest_json = read_manifest_json(&report);
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/requested")
                .and_then(serde_json::Value::as_str),
            Some("default-candidate")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/selected")
                .and_then(serde_json::Value::as_str),
            Some("direct-production-executor")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/default_activation_gate")
                .and_then(serde_json::Value::as_str),
            Some("direct-production-candidate")
        );
        assert!(
            manifest_json
                .pointer("/runtime_selection/fallback_reason")
                .is_none()
                || manifest_json
                    .pointer("/runtime_selection/fallback_reason")
                    .is_some_and(serde_json::Value::is_null),
            "activated default candidate should not report a fallback reason"
        );
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/scheduler_kernel_executed")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/compatibility_edge_invocations",
            0,
        );
    }

    #[test]
    fn r7e_default_candidate_legacy_sidecar_discovery_uses_direct_manifest() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (report, _temp_run_dir) =
            execute_fixture_run_with_runtime_policy_and_legacy_discovery_unlocked(
                "r7e_default_candidate_legacy_sidecar_discovery_direct",
                HillslopeRuntimeSelectionPolicy::new(
                    HillslopeRuntimeSelection::DefaultCandidate,
                    HillslopeDefaultRuntimeActivation::DirectProductionCandidate,
                ),
                true,
            );

        assert!(report.output_pass.is_file());
        assert!(report.output_loss.is_file());
        let manifest_json = read_manifest_json(&report);
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/requested")
                .and_then(serde_json::Value::as_str),
            Some("default-candidate")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/selected")
                .and_then(serde_json::Value::as_str),
            Some("direct-production-executor")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/selection_reason")
                .and_then(serde_json::Value::as_str),
            Some("default-candidate-direct-production-single-authority")
        );
        assert!(
            manifest_json
                .pointer("/runtime_selection/fallback_reason")
                .is_none()
                || manifest_json
                    .pointer("/runtime_selection/fallback_reason")
                    .is_some_and(serde_json::Value::is_null),
            "legacy sidecar discovery direct default must not report a fallback reason"
        );
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/scheduler_kernel_executed")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/compatibility_edge_invocations",
            0,
        );
    }

    #[test]
    fn r7e_runtime_selection_policy_resolves_only_direct_modes() {
        let default_activated = HillslopeRuntimeSelectionPolicy::default().resolve();
        assert_eq!(
            default_activated.requested(),
            HillslopeRuntimeSelection::DefaultCandidate
        );
        assert_eq!(
            default_activated.selected(),
            HillslopeRuntimeSelection::DirectProductionExecutor
        );
        assert_eq!(
            default_activated.default_activation(),
            HillslopeDefaultRuntimeActivation::DirectProductionCandidate
        );
        assert_eq!(default_activated.fallback_reason(), None);

        let explicit_direct = HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::DirectProductionCandidate,
        )
        .resolve();
        assert_eq!(
            explicit_direct.selection_reason(),
            "explicit-direct-production"
        );
        assert_eq!(
            explicit_direct.selected(),
            HillslopeRuntimeSelection::DirectProductionExecutor
        );
    }

    #[test]
    fn r7c_direct_production_executor_reports_no_day_input_compatibility_edges() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (report, _temp_run_dir) = execute_fixture_run_with_runtime_selection_unlocked(
            "r7c_direct_production_executor",
            HillslopeRuntimeSelection::DirectProductionExecutor,
        );

        assert!(report.output_pass.is_file());
        assert!(report.output_loss.is_file());
        let manifest_json = read_manifest_json(&report);
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/scheduler_kernel_executed")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/publication_source")
                .and_then(serde_json::Value::as_str),
            Some("direct-publication-frame")
        );
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/scheduler_status_message_id")
                .and_then(serde_json::Value::as_str),
            Some("R7C-DIRECT-PRODUCTION-EXECUTOR")
        );
        let expected_day_frames = manifest_json
            .pointer("/execution_provenance/climate_day_count")
            .and_then(serde_json::Value::as_u64)
            .expect("R7C fixture manifest must include direct climate day count");
        let expected_phase_spans = 1 + expected_day_frames * r5c_day_span_run_count();
        let expected_phase_entries =
            DIRECT_R3C_PHASE_SPAN_COUNT as u64 + expected_day_frames * r5c_day_phase_entry_count();
        assert_json_i64(&manifest_json, "/direct_runtime_counters/run_frame_constructions", 1);
        assert_json_i64(&manifest_json, "/direct_runtime_counters/executor_constructions", 1);
        assert_json_i64(&manifest_json, "/direct_runtime_counters/skeleton_runs", 0);
        assert_json_i64(&manifest_json, "/direct_runtime_counters/publication_capture_runs", 1);
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/day_frame_constructions",
            i64::try_from(expected_day_frames).expect("fixture day frame count fits i64"),
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/day_frame_commits",
            i64::try_from(expected_day_frames).expect("fixture day frame count fits i64"),
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/phase_span_runs",
            i64::try_from(expected_phase_spans).expect("fixture phase span count fits i64"),
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/direct_phase_entries",
            i64::try_from(expected_phase_entries).expect("fixture phase entry count fits i64"),
        );
        for counter in [
            "/direct_runtime_counters/direct_compute_operations",
            "/direct_runtime_counters/direct_state_mutations",
            "/direct_runtime_counters/downstream_operand_productions",
            "/direct_runtime_counters/shadow_projections",
        ] {
            let value = manifest_json
                .pointer(counter)
                .and_then(serde_json::Value::as_i64)
                .unwrap_or_else(|| panic!("manifest must include {counter}"));
            assert!(value > 0, "{counter} must be nonzero for R7C direct production");
        }
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/compatibility_edge_invocations",
            0,
        );
    }

    #[test]
    fn r6f_wat_hold_marker_is_reserved_for_exact_producer_gap_fields() {
        let compatibility = r6f_wat_marker_sample_row();
        let mut direct = compatibility.clone();
        direct.wepp_id = 19;
        direct.year = 2026;
        direct.es = 0.0;
        direct.total_soil_water = 0.0;
        direct.soil_water_total = None;
        direct.profile_depth = None;
        direct.profile_porosity_cap = None;
        direct.profile_fc_store = None;
        direct.profile_wp_store = None;

        let reduced_fields = reduced_wat_mismatch_fields(
            std::slice::from_ref(&direct),
            std::slice::from_ref(&compatibility),
        );
        assert_eq!(
            reduced_fields,
            vec![
                "wepp_id",
                "year",
                "Es",
                "Total-Soil",
                "SoilWaterTotal",
                "ProfileDepth",
                "ProfilePorosityCap",
                "ProfileFCStore",
                "ProfileWPStore",
            ]
        );
        assert!(r6f_wat_direct_process_producer_authority_gap(
            &reduced_fields
        ));

        let mut unrelated_direct = direct;
        unrelated_direct.p = 9.5;
        let unrelated_fields =
            reduced_wat_mismatch_fields(&[unrelated_direct], &[compatibility]);
        assert!(unrelated_fields.contains(&"P"));
        assert!(!r6f_wat_direct_process_producer_authority_gap(
            &unrelated_fields
        ));
    }

    #[test]
    fn r6g_wat_hold_marker_is_reserved_for_exact_pmet_day_state_carry_fields() {
        let compatibility = r6f_wat_marker_sample_row();
        let mut direct = compatibility.clone();
        direct.es = 0.9;
        direct.total_soil_water = 99.9;
        direct.soil_water_total = Some(99.9);

        let reduced_fields = reduced_wat_mismatch_fields(
            std::slice::from_ref(&direct),
            std::slice::from_ref(&compatibility),
        );
        assert_eq!(reduced_fields, vec!["Es", "Total-Soil", "SoilWaterTotal"]);
        assert!(r6g_wat_direct_et_storage_producer_gap(&reduced_fields));
        assert!(!r6g_wat_pmet_day_state_carry_gap(
            std::slice::from_ref(&direct),
            std::slice::from_ref(&compatibility),
            &reduced_fields
        ));

        let mut unrelated_direct = direct;
        unrelated_direct.dp = 2.0;
        let unrelated_fields =
            reduced_wat_mismatch_fields(&[unrelated_direct], &[compatibility]);
        assert!(unrelated_fields.contains(&"Dp"));
        assert!(!r6g_wat_direct_et_storage_producer_gap(
            &unrelated_fields
        ));
    }

    #[test]
    fn r6h_wat_hold_marker_is_reserved_for_exact_pmet_layer_ulp_gap() {
        let compatibility_first = r6f_wat_marker_sample_row();
        let direct_first = compatibility_first.clone();
        let mut compatibility_second = compatibility_first.clone();
        compatibility_second.sim_day_index = 2;
        compatibility_second.es = 0.767_760_184_372_260_8;
        let mut direct_second = compatibility_second.clone();
        direct_second.es = 0.767_760_184_372_260_5;

        let direct_rows = [direct_first, direct_second.clone()];
        let compatibility_rows = [compatibility_first, compatibility_second.clone()];
        let reduced_fields = reduced_wat_mismatch_fields(&direct_rows, &compatibility_rows);
        assert_eq!(reduced_fields, vec!["Es"]);
        assert!(r6h_wat_pmet_layer_carry_ulp_gap(
            &direct_rows,
            &compatibility_rows,
            &reduced_fields
        ));

        direct_second.es = 0.5;
        let direct_rows = [direct_rows[0].clone(), direct_second];
        let reduced_fields = reduced_wat_mismatch_fields(&direct_rows, &compatibility_rows);
        assert_eq!(reduced_fields, vec!["Es"]);
        assert!(!r6h_wat_pmet_layer_carry_ulp_gap(
            &direct_rows,
            &compatibility_rows,
            &reduced_fields
        ));

        let mut direct_first = compatibility_rows[0].clone();
        direct_first.es += 1.0e-13;
        let direct_rows = [direct_first, compatibility_rows[1].clone()];
        let reduced_fields = reduced_wat_mismatch_fields(&direct_rows, &compatibility_rows);
        assert_eq!(reduced_fields, vec!["Es"]);
        assert!(!r6h_wat_pmet_layer_carry_ulp_gap(
            &direct_rows,
            &compatibility_rows,
            &reduced_fields
        ));

        let mut compatibility_third = compatibility_rows[1].clone();
        compatibility_third.sim_day_index = 3;
        compatibility_third.es = 0.9;
        let mut direct_second = compatibility_rows[1].clone();
        direct_second.es = 0.767_760_184_372_260_5;
        let mut direct_third = compatibility_third.clone();
        direct_third.es = 0.5;
        let direct_rows = [
            compatibility_rows[0].clone(),
            direct_second,
            direct_third,
        ];
        let compatibility_rows = [
            compatibility_rows[0].clone(),
            compatibility_rows[1].clone(),
            compatibility_third,
        ];
        let reduced_fields = reduced_wat_mismatch_fields(&direct_rows, &compatibility_rows);
        assert_eq!(reduced_fields, vec!["Es"]);
        assert!(!r6h_wat_pmet_layer_carry_ulp_gap(
            &direct_rows,
            &compatibility_rows,
            &reduced_fields
        ));
    }

    #[test]
    fn r7h_growth_authority_selects_current_ofe_slot_not_primary_ofe() {
        let mut day = canonical_calendar_day_probe();
        day.julian_day = 165;
        let forcing = r7g_snow_forcing(20.0, 10.0);
        let authority = DirectProductionGrowthAuthority {
            active: true,
            rotation_years: 1,
            rotation_repeats: 1,
            slots: vec![
                DirectProductionGrowthSlotAuthority {
                    ofe_index: 1,
                    year_in_rotation: 1,
                    rotation_index: 1,
                    crops: vec![r7h_growth_crop_with_bb(0.11)],
                },
                DirectProductionGrowthSlotAuthority {
                    ofe_index: 2,
                    year_in_rotation: 1,
                    rotation_index: 1,
                    crops: vec![r7h_growth_crop_with_bb(0.22)],
                },
            ],
            monthly_temperature_max_c: [20.0; 12],
            monthly_temperature_min_c: [10.0; 12],
            soil_depth_m: 1.0,
        };

        let (annual_inputs, perennial_inputs) = authority
            .inputs(
                &day,
                1,
                2,
                &forcing,
                openwepp_hillslope_orchestrator::DirectGrowthStateSurface {
                    sumgdd: 1.0,
                    live_biomass_kg_m2: 0.1,
                    interception_live_biomass_kg_m2: 0.1,
                    canopy_cover_fraction: 0.2,
                    leaf_area_index: 0.3,
                    root_mass_kg_m2: 0.4,
                    root_depth_m: 0.5,
                    harvest_index: 0.0,
                },
                1.0,
                &DirectEvapotranspirationComputeInputs::zero(),
            )
            .expect("OFE-specific active growth selection should succeed");

        assert_eq!(annual_inputs.bb.to_bits(), 0.22_f64.to_bits());
        assert_eq!(
            perennial_inputs.active_context,
            DirectGrowthActiveContext::Inactive
        );
    }

    #[test]
    fn r7h_growth_authority_accepts_unambiguous_lane_local_slot() {
        let mut day = canonical_calendar_day_probe();
        day.julian_day = 165;
        let forcing = r7g_snow_forcing(20.0, 10.0);
        let authority = DirectProductionGrowthAuthority {
            active: true,
            rotation_years: 1,
            rotation_repeats: 1,
            slots: vec![DirectProductionGrowthSlotAuthority {
                ofe_index: 1,
                year_in_rotation: 1,
                rotation_index: 1,
                crops: vec![r7h_growth_crop_with_bb(0.33)],
            }],
            monthly_temperature_max_c: [20.0; 12],
            monthly_temperature_min_c: [10.0; 12],
            soil_depth_m: 1.0,
        };

        let (annual_inputs, _) = authority
            .inputs(
                &day,
                1,
                4,
                &forcing,
                openwepp_hillslope_orchestrator::DirectGrowthStateSurface {
                    sumgdd: 1.0,
                    live_biomass_kg_m2: 0.1,
                    interception_live_biomass_kg_m2: 0.1,
                    canopy_cover_fraction: 0.2,
                    leaf_area_index: 0.3,
                    root_mass_kg_m2: 0.4,
                    root_depth_m: 0.5,
                    harvest_index: 0.0,
                },
                1.0,
                &DirectEvapotranspirationComputeInputs::zero(),
            )
            .expect("lane-local OFE-1 authority should apply to its lane");

        assert_eq!(annual_inputs.bb.to_bits(), 0.33_f64.to_bits());
    }

    fn r7h_growth_crop_with_bb(bb: f64) -> DirectProductionGrowthCropAuthority {
        DirectProductionGrowthCropAuthority {
            schedule_imngmt: 1,
            imngmt: 1,
            jdharv: 300,
            jdplt: 1,
            jdstop: 0,
            btemp: 0.0,
            otemp: 30.0,
            gddmax: 1_000.0,
            dlai: 0.8,
            dropfc: 1.0,
            decfct: 1.0,
            spriod: 1.0,
            bb,
            beinp: 1.0,
            extnct: 0.5,
            hi: 0.4,
            xmxlai: 3.0,
            rsr: 0.2,
            rtmmax: 1.0,
            rdmax: 1.0,
            oratea: 0.0,
            orater: 0.0,
        }
    }

    fn r6f_wat_marker_sample_row() -> HillslopeWatRow {
        HillslopeWatRow {
            wepp_id: 1,
            ofe_id: 1,
            year: 1987,
            sim_day_index: 1,
            julian: 1,
            month: 1,
            day_of_month: 1,
            water_year: 1987,
            ofe: 1,
            p: 10.0,
            rm: 0.0,
            q: 0.0,
            ep: 1.0,
            es: 1.0,
            er: 0.1,
            dp: 0.1,
            up_strm_q: 0.0,
            sub_r_in: 0.0,
            latqcc: 0.0,
            total_soil_water: 100.0,
            frozwt: 0.0,
            frdp: 0.0,
            snow_water: 0.0,
            snow_depth: None,
            meltwater_temperature: None,
            qofe: 0.0,
            tile: 0.0,
            irr: 0.0,
            area: 1.0,
            soil_water_total: Some(100.0),
            profile_depth: Some(1_200.0),
            profile_porosity_cap: Some(400.0),
            profile_fc_store: Some(300.0),
            profile_wp_store: Some(150.0),
            interception: Some(0.25),
            interception_storage: None,
        }
    }

    #[test]
    fn r6b_absent_operand_detector_suppresses_marker_for_nonzero_direct_operands() {
        let identity =
            DirectRunIdentity::new(19, 2637, 1, 1).expect("valid direct identity should construct");
        let zero_row = DirectPublicationDayRow {
            run_id: 19,
            hillslope_id: 2637,
            lane_id: 1,
            ofe_id: 1,
            lane_index: 0,
            day_index: 0,
            sim_day_index: 1,
            calendar: DirectPublicationCalendarDay {
                year: 2026,
                julian_day: 172,
                month: 6,
                day_of_month: 21,
                water_year: 2026,
            },
            area_m2: 400.0,
            climate: DirectPublicationClimateOperands {
                precipitation_mm: 0.0,
            },
            liquid_input: DirectPublicationLiquidInputOperands {
                rm_mm: 0.0,
                irrigation_mm: 0.0,
            },
            runoff: DirectPublicationRunoffOperands {
                q_mm: 0.0,
                qofe_mm: 0.0,
                runvol_m3: 0.0,
                peak_runoff_m3_s: None,
                runoff_duration_s: None,
            },
            evaporation: DirectPublicationEvaporationOperands {
                ep_mm: 0.0,
                es_mm: 0.0,
                er_mm: 0.0,
                total_evapotranspiration_mm: 0.0,
            },
            subsurface: DirectPublicationSubsurfaceOperands {
                dp_mm: 0.0,
                latqcc_mm: 0.0,
                tile_mm: 0.0,
                sbrunv_m3: 0.0,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: 0.0,
                upstream_lateral_mm: 0.0,
            },
            storage: DirectPublicationStorageOperands {
                total_soil_mm: 0.0,
                soil_water_total_mm: 0.0,
                frozwt_mm: 0.0,
                frdp_mm: None,
                snow_water_mm: 0.0,
                snow_depth_mm: 0.0,
            },
            water_temperature: r6_test_no_water_temperature(),
            profile: r6_test_empty_profile(),
            interception: DirectPublicationInterceptionOperands {
                interception_mm: 0.0,
                interception_storage_mm: None,
            },
            erosion: DirectPublicationErosionOperands::absent_authority(),
        };

        assert!(direct_publication_has_only_zero_or_absent_operands(
            &r6b_publication_frame_with_row(identity, zero_row.clone())
        ));
        assert!(direct_publication_lacks_parity_grade_output_producers(
            &r6b_publication_frame_with_row(identity, zero_row.clone())
        ));

        let mut climate_only_row = zero_row.clone();
        climate_only_row.climate.precipitation_mm = 12.5;
        assert!(!direct_publication_has_only_zero_or_absent_operands(
            &r6b_publication_frame_with_row(identity, climate_only_row.clone())
        ));
        assert!(direct_publication_lacks_parity_grade_output_producers(
            &r6b_publication_frame_with_row(identity, climate_only_row)
        ));

        let mut scalar_row = zero_row.clone();
        scalar_row.runoff.q_mm = 1.0;
        assert!(!direct_publication_has_only_zero_or_absent_operands(
            &r6b_publication_frame_with_row(identity, scalar_row.clone())
        ));
        assert!(!direct_publication_lacks_parity_grade_output_producers(
            &r6b_publication_frame_with_row(identity, scalar_row)
        ));

        let mut optional_row = zero_row.clone();
        optional_row.runoff.peak_runoff_m3_s = Some(0.25);
        assert!(!direct_publication_has_only_zero_or_absent_operands(
            &r6b_publication_frame_with_row(identity, optional_row)
        ));

        let mut erosion_row = zero_row;
        erosion_row.erosion.sediment_concentration_kg_m3 = Some([0.0, 0.1, 0.0, 0.0, 0.0]);
        assert!(!direct_publication_has_only_zero_or_absent_operands(
            &r6b_publication_frame_with_row(identity, erosion_row)
        ));
    }

    fn r6b_publication_frame_with_row(
        identity: DirectRunIdentity,
        row: DirectPublicationDayRow,
    ) -> DirectRunPublicationFrame {
        DirectRunPublicationFrame {
            identity,
            metadata: DirectPublicationRunMetadata {
                run_name: "r6b_absent_operands".to_string(),
                runtime_selection: "direct-production-executor".to_string(),
                output_policy: "test".to_string(),
            },
            rows: vec![row],
        }
    }

    #[test]
    fn cqr_row7_retained_publication_frame_validator_covers_count_and_identity_guards() {
        let identity =
            DirectRunIdentity::new(42, 2637, 2, 2).expect("valid direct identity should construct");
        let metadata = DirectPublicationRunMetadata {
            run_name: "cqr_row7_retained".to_string(),
            runtime_selection: "direct-production-executor".to_string(),
            output_policy: "test".to_string(),
        };
        let frame = DirectRunPublicationFrame {
            identity,
            metadata: metadata.clone(),
            rows: vec![
                r6j_multiofe_publication_row(1, 1),
                r6j_multiofe_publication_row(2, 1),
                r6j_multiofe_publication_row(1, 2),
                r6j_multiofe_publication_row(2, 2),
            ],
        };
        validate_retained_direct_publication_frame(&frame)
            .expect("complete retained frame should validate");

        let mut bad_count = frame.clone();
        bad_count.rows.pop();
        assert!(validate_retained_direct_publication_frame(&bad_count).is_err());

        let mut bad_identity = frame;
        bad_identity.rows[0].lane_index = 3;
        assert!(validate_retained_direct_publication_frame(&bad_identity).is_err());
    }

    #[test]
    fn r6a_direct_projection_consumers_read_publication_frame_operands() {
        let frame = r6a_direct_projection_fixture_frame();

        let wat_rows = build_hillslope_wat_rows_from_direct_publication(&frame)
            .expect("direct WAT projection should build");
        let pass_rows = build_hillslope_pass_rows_from_direct_publication(&frame)
            .expect("direct PASS projection should build");
        let loss = build_loss_output_json_from_direct_publication(&frame, 1, false, 0)
            .expect("direct loss projection should build");
        let manifest = build_manifest_text_from_direct_publication(&frame)
            .expect("direct manifest projection should build");

        assert_eq!(wat_rows[0].q.to_bits(), 12.5_f64.to_bits());
        assert_eq!(wat_rows[0].qofe.to_bits(), 10.0_f64.to_bits());
        assert_eq!(wat_rows[0].rm.to_bits(), 8.25_f64.to_bits());
        assert_eq!(pass_rows[0].runvol_m3.to_bits(), 4.0_f64.to_bits());
        assert_eq!(pass_rows[0].peakro_m3_s.to_bits(), 0.75_f64.to_bits());
        let loss_json: serde_json::Value =
            serde_json::from_str(&loss).expect("direct loss projection should be JSON");
        assert_eq!(loss_json["run_name"], "r6a_projection");
        assert!(manifest.contains("row_count=1"));
    }

    fn r6a_direct_projection_fixture_frame() -> DirectRunPublicationFrame {
        let identity =
            DirectRunIdentity::new(19, 2637, 1, 1).expect("valid direct identity should construct");
        let row = DirectPublicationDayRow {
            run_id: 19,
            hillslope_id: 2637,
            lane_id: 1,
            ofe_id: 1,
            lane_index: 0,
            day_index: 0,
            sim_day_index: 1,
            calendar: DirectPublicationCalendarDay {
                year: 2026,
                julian_day: 172,
                month: 6,
                day_of_month: 21,
                water_year: 2026,
            },
            area_m2: 400.0,
            climate: DirectPublicationClimateOperands {
                precipitation_mm: 7.5,
            },
            liquid_input: DirectPublicationLiquidInputOperands {
                rm_mm: 8.25,
                irrigation_mm: 1.25,
            },
            runoff: DirectPublicationRunoffOperands {
                q_mm: 12.5,
                qofe_mm: 10.0,
                runvol_m3: 4.0,
                peak_runoff_m3_s: Some(0.75),
                runoff_duration_s: Some(1800.0),
            },
            evaporation: DirectPublicationEvaporationOperands {
                ep_mm: 2.0,
                es_mm: 3.0,
                er_mm: 4.0,
                total_evapotranspiration_mm: 9.0,
            },
            subsurface: DirectPublicationSubsurfaceOperands {
                dp_mm: 1.5,
                latqcc_mm: 2.5,
                tile_mm: 0.5,
                sbrunv_m3: 1.0,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: 0.25,
                upstream_lateral_mm: 0.125,
            },
            storage: DirectPublicationStorageOperands {
                total_soil_mm: 110.0,
                soil_water_total_mm: 105.0,
                frozwt_mm: 1.0,
                frdp_mm: Some(2.0),
                snow_water_mm: 3.0,
                snow_depth_mm: 4.0,
            },
            water_temperature: r6_test_no_water_temperature(),
            profile: DirectPublicationProfileOperands {
                depth_mm: Some(1000.0),
                porosity_cap_mm: Some(450.0),
                fc_store_mm: Some(300.0),
                wp_store_mm: Some(150.0),
            },
            interception: DirectPublicationInterceptionOperands {
                interception_mm: 0.75,
                interception_storage_mm: Some(0.5),
            },
            erosion: DirectPublicationErosionOperands {
                peak_runoff_m3_s: Some(0.75),
                runoff_duration_s: Some(1800.0),
                total_detachment_kg: Some(2.25),
                total_deposition_kg: Some(1.25),
                hbp_total_detachment_kg: Some(2.25),
                hbp_total_deposition_kg: Some(1.25),
                hbp_sediment_concentration_kg_m3: Some(0.1),
                sediment_concentration_kg_m3: Some([0.1, 0.2, 0.3, 0.4, 0.5]),
            },
        };
        DirectRunPublicationFrame {
            identity,
            metadata: DirectPublicationRunMetadata {
                run_name: "r6a_projection".to_string(),
                runtime_selection: "direct-production-executor".to_string(),
                output_policy: "test".to_string(),
            },
            rows: vec![row],
        }
    }

    #[test]
    fn r6j_direct_manifest_provenance_accepts_multiofe_direct_rows() {
        let identity =
            DirectRunIdentity::new(42, 2637, 2, 2).expect("valid direct identity should construct");
        let frame = DirectRunPublicationFrame {
            identity,
            metadata: DirectPublicationRunMetadata {
                run_name: "r6j_multiofe_manifest".to_string(),
                runtime_selection: "direct-production-executor".to_string(),
                output_policy: "test".to_string(),
            },
            rows: vec![
                r6j_multiofe_publication_row(1, 1),
                r6j_multiofe_publication_row(2, 1),
                r6j_multiofe_publication_row(1, 2),
                r6j_multiofe_publication_row(2, 2),
            ],
        };

        let (wb13_publication, mofe_hourly_carry) =
            build_direct_publication_manifest_provenance(&frame)
                .expect("direct multi-OFE manifest provenance should build");
        assert_eq!(wb13_publication.source, "direct-publication-frame");
        assert_eq!(wb13_publication.contributor_ofe_count, 2);
        assert_eq!(wb13_publication.row_count, 4);
        assert_eq!(wb13_publication.publication_area_m2.to_bits(), 1200.0_f64.to_bits());
        assert_eq!(wb13_publication.per_ofe_record_count, 4);
        assert_eq!(wb13_publication.per_ofe_internal_day_count, 2);
        assert_eq!(wb13_publication.per_ofe_expected_record_count, 4);
        assert_eq!(wb13_publication.first_row_key.ofe, 1);
        assert_eq!(wb13_publication.first_row_key.sim_day_index, 1);
        assert_eq!(wb13_publication.last_row_key.ofe, 2);
        assert_eq!(wb13_publication.last_row_key.sim_day_index, 2);
        assert!(mofe_hourly_carry.active);
        assert_eq!(mofe_hourly_carry.substep_count, 24);

        let wat_rows = build_hillslope_wat_rows_from_direct_publication(&frame)
            .expect("direct multi-OFE WAT rows should build");
        assert_eq!(wat_rows.len(), 4);
        assert_eq!(wat_rows[0].ofe_id, 1);
        assert_eq!(wat_rows[1].ofe_id, 2);
        assert_eq!(wat_rows[2].sim_day_index, 2);
        assert_eq!(wat_rows[3].ofe_id, 2);
        let pass_rows = build_hillslope_pass_rows_from_direct_publication(&frame)
            .expect("direct multi-OFE PASS rows should build");
        assert_eq!(pass_rows.len(), 2);
        assert_eq!(pass_rows[0].year, 1);
        assert_eq!(pass_rows[0].sim_day_index, 1);
        assert_eq!(pass_rows[1].sim_day_index, 2);
    }

    fn r6j_multiofe_publication_row(ofe_id: u32, sim_day_index: i32) -> DirectPublicationDayRow {
        let lane_index = usize::try_from(ofe_id - 1).expect("test OFE id should fit usize");
        let day_index = usize::try_from(sim_day_index - 1).expect("test day should fit usize");
        let offset = f64::from(ofe_id) + f64::from(sim_day_index) / 10.0;
        DirectPublicationDayRow {
            run_id: 42,
            hillslope_id: 2637,
            lane_id: ofe_id,
            ofe_id,
            lane_index,
            day_index,
            sim_day_index,
            calendar: DirectPublicationCalendarDay {
                year: 2026,
                julian_day: u16::try_from(sim_day_index).expect("test day should fit u16"),
                month: 1,
                day_of_month: i8::try_from(sim_day_index).expect("test day should fit i8"),
                water_year: 2026,
            },
            area_m2: 400.0 * f64::from(ofe_id),
            climate: DirectPublicationClimateOperands {
                precipitation_mm: 7.5 + offset,
            },
            liquid_input: DirectPublicationLiquidInputOperands {
                rm_mm: 8.25 + offset,
                irrigation_mm: 1.25,
            },
            runoff: DirectPublicationRunoffOperands {
                q_mm: 12.5 + offset,
                qofe_mm: 10.0 + offset,
                runvol_m3: 4.0 + offset,
                peak_runoff_m3_s: Some(0.75 + offset),
                runoff_duration_s: Some(1800.0 + offset),
            },
            evaporation: DirectPublicationEvaporationOperands {
                ep_mm: 2.0 + offset,
                es_mm: 3.0 + offset,
                er_mm: 4.0 + offset,
                total_evapotranspiration_mm: 9.0 + offset,
            },
            subsurface: DirectPublicationSubsurfaceOperands {
                dp_mm: 1.5 + offset,
                latqcc_mm: 2.5 + offset,
                tile_mm: 0.5 + offset,
                sbrunv_m3: 1.0 + offset,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: 0.25 + offset,
                upstream_lateral_mm: 0.125 + offset,
            },
            storage: DirectPublicationStorageOperands {
                total_soil_mm: 110.0 + offset,
                soil_water_total_mm: 105.0 + offset,
                frozwt_mm: 1.0 + offset,
                frdp_mm: Some(2.0 + offset),
                snow_water_mm: 3.0 + offset,
                snow_depth_mm: 4.0 + offset,
            },
            water_temperature: r6_test_no_water_temperature(),
            profile: DirectPublicationProfileOperands {
                depth_mm: Some(1000.0 + offset),
                porosity_cap_mm: Some(450.0 + offset),
                fc_store_mm: Some(300.0 + offset),
                wp_store_mm: Some(150.0 + offset),
            },
            interception: DirectPublicationInterceptionOperands {
                interception_mm: 0.75 + offset,
                interception_storage_mm: Some(0.5 + offset),
            },
            erosion: DirectPublicationErosionOperands {
                peak_runoff_m3_s: Some(0.75 + offset),
                runoff_duration_s: Some(1800.0 + offset),
                total_detachment_kg: Some(2.25 + offset),
                total_deposition_kg: Some(1.25 + offset),
                hbp_total_detachment_kg: Some(2.25 + offset),
                hbp_total_deposition_kg: Some(1.25 + offset),
                hbp_sediment_concentration_kg_m3: Some(0.1 + offset),
                sediment_concentration_kg_m3: Some([0.1, 0.2, 0.3, 0.4, 0.5]),
            },
        }
    }

    fn cqr_row7_snow_lane_state(
        layers: Vec<openwepp_hillslope_orchestrator::DirectSnowLayerState>,
    ) -> DirectSnowLaneState {
        let runtime_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum();
        let runtime_depth_m = layers.iter().map(|layer| layer.thickness_m).sum();
        DirectSnowLaneState {
            runtime_swe_m,
            runtime_depth_m,
            runtime_density_kg_m3: if runtime_depth_m > 0.0 {
                runtime_swe_m / runtime_depth_m * 1_000.0
            } else {
                0.0
            },
            runtime_settle_day_count: 0.0,
            coe_boundary_depth_m: runtime_depth_m,
            coe_boundary_density_kg_m3: 0.0,
            coe_boundary_settle_day_count: 0.0,
            liquid_water_retained_m: 0.0,
            snow_albedo_state: None,
            layers,
        }
    }

    #[test]
    fn cqr_row7_snow_frost_insulation_helpers_cover_layered_and_sturm_paths() {
        let layers = vec![
            openwepp_hillslope_orchestrator::DirectSnowLayerState::new(0.03, 0.10, 300.0, 0.0),
            openwepp_hillslope_orchestrator::DirectSnowLayerState::new(0.08, 0.20, 400.0, 0.0),
        ];
        let snow = cqr_row7_snow_lane_state(layers);
        let (depth_m, density_kg_m3) =
            layered_snow_frost_insulation_depth_density(&snow, 1.0)
                .expect("layered snow insulation should compute");
        assert_eq!(depth_m.to_bits(), snow.runtime_depth_m.to_bits());
        assert!(density_kg_m3.is_finite() && density_kg_m3 > 0.0);

        let dry_snow = cqr_row7_snow_lane_state(Vec::new());
        assert_eq!(
            layered_snow_frost_insulation_depth_density(&dry_snow, 1.0)
                .expect("zero snow should be inert"),
            (0.0, 0.0)
        );

        let mut missing_layers = dry_snow.clone();
        missing_layers.runtime_swe_m = 0.01;
        missing_layers.runtime_depth_m = 0.05;
        assert!(layered_snow_frost_insulation_depth_density(&missing_layers, 1.0).is_err());

        let mut mismatch = snow.clone();
        mismatch.runtime_depth_m += 0.01;
        assert!(layered_snow_frost_insulation_depth_density(&mismatch, 1.0).is_err());

        let low_density_conductivity =
            sturm1997_snow_conductivity_w_m_k(100.0, 1.0).expect("low density branch");
        let high_density_conductivity =
            sturm1997_snow_conductivity_w_m_k(300.0, 1.0).expect("high density branch");
        assert!(low_density_conductivity > 0.0);
        assert!(high_density_conductivity > low_density_conductivity);
        assert!(sturm1997_snow_conductivity_w_m_k(-1.0, 1.0).is_err());
        assert!(sturm1997_snow_conductivity_w_m_k(100.0, 0.0).is_err());

        let inverted = invert_sturm1997_snow_density_kg_m3(high_density_conductivity, 1.0)
            .expect("conductivity should invert to density");
        assert!((inverted - 300.0).abs() < 1.0e-6);
        assert!(invert_sturm1997_snow_density_kg_m3(0.0, 1.0).is_err());
    }

    #[test]
    fn cqr_row7_snow_selector_parsers_cover_defaults_variants_and_rejections() {
        assert_eq!(
            parse_snowdensity1015_default_snow_density_model(None)
                .expect("missing density selector should default"),
            openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkDensityCompactionV1
        );
        assert_eq!(
            parse_snowdensity1015_default_snow_density_model(Some("legacy_wepp"))
                .expect("legacy density selector should parse"),
            openwepp_hillslope_orchestrator::SnowDensityModel::LegacyWepp
        );
        assert!(
            parse_snowdensity1015_default_snow_density_model(Some("bad_density")).is_err()
        );

        assert_eq!(
            parse_snowdensity1037_diagnostic_snow_melt_model(None)
                .expect("missing diagnostic melt selector should default"),
            openwepp_hillslope_orchestrator::SnowMeltModel::LegacyCoe
        );
        assert_eq!(
            parse_snowdensity1037_diagnostic_snow_melt_model(Some(
                "coe_winter_thaw_state_loss_v1",
            ))
            .expect("diagnostic melt selector should parse"),
            openwepp_hillslope_orchestrator::SnowMeltModel::CoeWinterThawStateLossV1
        );
        assert!(parse_snowdensity1037_diagnostic_snow_melt_model(Some("bad_melt")).is_err());

        assert_eq!(
            parse_snowdensity1015_default_snow_melt_model(None)
                .expect("missing default melt selector should default"),
            openwepp_hillslope_orchestrator::SnowMeltModel::CoeLiquidHoldingCapacityV1
        );
        assert_eq!(
            parse_snowdensity1015_default_snow_melt_model(Some(
                "coe_open_sublimation_stage_b_v1",
            ))
            .expect("default melt selector should parse stage B"),
            openwepp_hillslope_orchestrator::SnowMeltModel::CoeOpenSublimationStageBV1
        );
        assert!(parse_snowdensity1015_default_snow_melt_model(Some("bad_default_melt")).is_err());
    }

    fn cqr_row7_test_climate_file() -> ClimateFile {
        ClimateFile {
            datver: 5.3,
            mode: openwepp_input_contract::parsers::climate::ClimateModeFlags {
                itemp: 1,
                breakpoint_enabled: false,
                iwind: 0,
            },
            station_id: "cqr-row7".to_string(),
            metadata: openwepp_input_contract::parsers::climate::ClimateMetadata {
                deglat: 45.0,
                deglon: -116.0,
                elev: 1_500.0,
                obsyrs: 1,
                ibyear: 2020,
                numyr: 1,
                generator_cmd: None,
            },
            monthly: openwepp_input_contract::parsers::climate::ClimateMonthlyStats {
                obmaxt: [0.0; 12],
                obmint: [0.0; 12],
                radave: [0.0; 12],
                obrain: [0.0; 12],
            },
            daily_records: vec![
                ClimateDailyRecord::NoBreakpoint(
                    openwepp_input_contract::parsers::climate::NoBreakpointDay {
                        day: 1,
                        mon: 1,
                        year: 2020,
                        prcp: 0.012,
                        stmdur: 3_600.0,
                        timep: 0.5,
                        ip: 10.0,
                        tmax: -2.0,
                        tmin: -8.0,
                        rad: 120.0,
                        vwind: 2.0,
                        wind: 180.0,
                        tdpt: -10.0,
                    },
                ),
                ClimateDailyRecord::NoBreakpoint(
                    openwepp_input_contract::parsers::climate::NoBreakpointDay {
                        day: 2,
                        mon: 1,
                        year: 2020,
                        prcp: 0.006,
                        stmdur: 3_600.0,
                        timep: 0.5,
                        ip: 10.0,
                        tmax: 2.0,
                        tmin: -4.0,
                        rad: 140.0,
                        vwind: 4.0,
                        wind: 180.0,
                        tdpt: -6.0,
                    },
                ),
            ],
        }
    }

    fn cqr_row7_climate_span(days: Vec<ClimateDayProjection>) -> ClimateRunSpanSummary {
        let first_day = *days.first().expect("test climate span should be nonempty");
        let last_day = *days.last().expect("test climate span should be nonempty");
        ClimateRunSpanSummary {
            days,
            first_day,
            last_day,
        }
    }

    #[test]
    fn cqr_row7_sturm1995_climate_normals_cover_nominal_and_guard_paths() {
        let climate = cqr_row7_test_climate_file();
        let climate_request = build_hillslope_climate_runtime_request(&climate)
            .expect("test climate should build runtime request");
        let span = cqr_row7_climate_span(vec![
            ClimateDayProjection {
                year: 2020,
                month: 1,
                day_of_month: 1,
                julian_day: 1,
                precipitation_mm: 12.0,
                effective_temperature_c: -5.0,
            },
            ClimateDayProjection {
                year: 2020,
                month: 1,
                day_of_month: 2,
                julian_day: 2,
                precipitation_mm: 6.0,
                effective_temperature_c: -1.0,
            },
        ]);

        let normals = direct_production_sturm1995_climate_normals(&climate_request, &span)
            .expect("valid climate span should produce Sturm normals");
        assert!(normals.cooling_degree_month_c > 0.0);
        assert!(normals.snowfall_precipitation_rate_mm_day > 0.0);
        assert_eq!(normals.winter_wind_m_s.to_bits(), 3.0_f64.to_bits());

        let empty_span = ClimateRunSpanSummary {
            days: Vec::new(),
            first_day: span.first_day,
            last_day: span.last_day,
        };
        assert!(direct_production_sturm1995_climate_normals(&climate_request, &empty_span).is_err());

        let bad_month_span = cqr_row7_climate_span(vec![ClimateDayProjection {
            month: 13,
            ..span.first_day
        }]);
        assert!(
            direct_production_sturm1995_climate_normals(&climate_request, &bad_month_span).is_err()
        );
    }

    fn cqr_row7_insert_projection_scalar(
        surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
        symbol: String,
        value: f64,
    ) {
        surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }

    fn cqr_row7_growth_projection() -> openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces {
        let mut projection =
            openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces {
                pl_schedule_surface: BTreeMap::new(),
                pl_growth_surface: BTreeMap::new(),
                pl_decomp_surface: BTreeMap::new(),
            };
        cqr_row7_insert_projection_scalar(
            &mut projection.pl_schedule_surface,
            direct_growth_schedule_slot_crop_symbol(1, 1, "imngmt"),
            2.0,
        );
        for (root, value) in [
            ("imngmt", 2.0),
            ("jdplt", 0.0),
            ("jdharv", 100.0),
            ("jdstop", 200.0),
            ("mgtopt", 1.0),
            ("btemp", 0.0),
            ("otemp", 20.0),
            ("gddmax", 100.0),
            ("dlai", 10.0),
            ("dropfc", 0.5),
            ("decfct", 0.8),
            ("spriod", 1.0),
            ("bb", 1.0),
            ("beinp", 1.0),
            ("extnct", 0.5),
            ("hi", 0.4),
            ("xmxlai", 2.0),
            ("rsr", 0.2),
            ("rtmmax", 1.0),
            ("rdmax", 1.0),
        ] {
            cqr_row7_insert_projection_scalar(
                &mut projection.pl_growth_surface,
                direct_growth_slot_crop_symbol(1, 1, root),
                value,
            );
        }
        for (root, value) in [("oratea", 0.0), ("orater", 0.03)] {
            cqr_row7_insert_projection_scalar(
                &mut projection.pl_decomp_surface,
                direct_decomp_slot_crop_symbol(1, 1, root),
                value,
            );
        }
        projection
    }

    fn cqr_row7_growth_surface(live_biomass_kg_m2: f64) -> DirectGrowthStateSurface {
        DirectGrowthStateSurface {
            live_biomass_kg_m2,
            ..DirectGrowthStateSurface::zero()
        }
    }

    #[test]
    fn cqr_row7_growth_crop_and_surface_litter_projection_cover_schedule_paths() {
        let projection = cqr_row7_growth_projection();
        let crop = direct_production_typed_growth_crop_authority(&projection, 1, 1)
            .expect("typed growth crop authority should project");
        assert!(crop.uses_fall_litter_drop_schedule());
        assert_eq!(
            crop.surface_decomposition_rate().to_bits(),
            FOREST_LITTER_FALLBACK_DECAY_RATE_PER_DAY.to_bits()
        );

        let mut annual_projection = cqr_row7_growth_projection();
        cqr_row7_insert_projection_scalar(
            &mut annual_projection.pl_schedule_surface,
            direct_growth_schedule_slot_crop_symbol(1, 1, "imngmt"),
            1.0,
        );
        cqr_row7_insert_projection_scalar(
            &mut annual_projection.pl_growth_surface,
            direct_growth_slot_crop_symbol(1, 1, "jdplt"),
            10.0,
        );
        let annual_crop = direct_production_typed_growth_crop_authority(&annual_projection, 1, 1)
            .expect("annual crop authority should project without fall stop management");
        assert_eq!(annual_crop.schedule_imngmt, 1);
        assert_eq!(annual_crop.jdplt, 10);
        assert_eq!(annual_crop.jdstop, 0);

        let mut invalid_schedule = cqr_row7_growth_projection();
        cqr_row7_insert_projection_scalar(
            &mut invalid_schedule.pl_schedule_surface,
            direct_growth_schedule_slot_crop_symbol(1, 1, "imngmt"),
            4.0,
        );
        assert!(direct_production_typed_growth_crop_authority(&invalid_schedule, 1, 1).is_err());

        let mut missing_scalar = cqr_row7_growth_projection();
        missing_scalar.pl_growth_surface.remove(&BoundarySymbol::from(
            direct_growth_slot_crop_symbol(1, 1, "btemp"),
        ));
        assert!(direct_production_typed_growth_crop_authority(&missing_scalar, 1, 1).is_err());

        let residue = DirectProductionResidueCoverState {
            surface_residue_kg_m2: 0.0,
            root_residue_kg_m2: 0.0,
            pending_surface_litter_kg_m2: 0.2,
            residue_depth_m: 0.0,
        };
        let before = cqr_row7_growth_surface(2.0);
        let after = cqr_row7_growth_surface(1.5);

        let in_window =
            direct_production_surface_litter_projection(Some(&crop), 100, residue, before, after)
                .expect("in-window litter should drop");
        assert_eq!(in_window.surface_litter_input_kg_m2.to_bits(), 0.7_f64.to_bits());
        assert_eq!(in_window.pending_surface_litter_after_kg_m2.to_bits(), 0.0_f64.to_bits());

        let out_of_window =
            direct_production_surface_litter_projection(Some(&crop), 1, residue, before, after)
                .expect("out-of-window litter should remain pending");
        assert_eq!(out_of_window.surface_litter_input_kg_m2.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            out_of_window.pending_surface_litter_after_kg_m2.to_bits(),
            0.7_f64.to_bits()
        );

        let no_crop =
            direct_production_surface_litter_projection(None, 1, residue, before, after)
                .expect("non-scheduled crop should emit daily litter directly");
        assert_eq!(no_crop.surface_litter_input_kg_m2.to_bits(), 0.5_f64.to_bits());
    }

    fn cqr_row7_forcing(rad_ly: f64, tmax_c: f64, tmin_c: f64) -> HillslopeDirectClimateDayForcing {
        HillslopeDirectClimateDayForcing {
            prcp_m: 0.0,
            tmax_c,
            tmin_c,
            rad_ly,
            vwind_m_s: 1.0,
            wind_deg: 180.0,
            tdpt_c: tmin_c,
            timem_s: Vec::new(),
            intsty_m_s: Vec::new(),
        }
    }

    #[test]
    fn cqr_row7_priestley_taylor_demand_covers_canopy_and_guard_paths() {
        let authority = DirectProductionPriestleyTaylorAuthority { salb: 0.2 };
        let forcing = cqr_row7_forcing(160.0, 12.0, 2.0);
        let bare = authority
            .compute_demand(&forcing, 0.0, 0.0)
            .expect("bare soil ET demand should compute");
        let canopy = authority
            .compute_demand(&forcing, 2.0, 0.6)
            .expect("canopy ET demand should compute");
        assert!(bare.is_finite() && bare > 0.0);
        assert!(canopy.is_finite() && canopy > 0.0);

        assert!(authority.compute_demand(&cqr_row7_forcing(-1.0, 12.0, 2.0), 0.0, 0.0).is_err());
        assert!(
            DirectProductionPriestleyTaylorAuthority { salb: 1.2 }
                .compute_demand(&forcing, 0.0, 0.0)
                .is_err()
        );
        assert!(
            authority
                .compute_demand(&cqr_row7_forcing(160.0, -300.0, -300.0), 0.0, 0.0)
                .is_err()
        );
    }

    fn cqr_row7_subsurface_layer(theta_m: f64) -> DirectSubsurfaceLayerState {
        let mut layer = DirectSubsurfaceLayerState::neutral();
        layer.theta_m = theta_m;
        layer.residual_theta = 0.10;
        layer.depth_m = 0.20;
        layer
    }

    #[test]
    fn cqr_row7_no_final_frost_rebalance_covers_credit_debit_and_guards() {
        let mut credit_layers = vec![cqr_row7_subsurface_layer(0.02)];
        rebalance_direct_production_no_final_frost_layers_to_storage(0, &mut credit_layers, 0.07)
            .expect("positive delta should credit top layer");
        assert!((credit_layers[0].theta_m - 0.05).abs() < 1.0e-12);

        let mut debit_layers = vec![
            cqr_row7_subsurface_layer(0.05),
            cqr_row7_subsurface_layer(0.04),
        ];
        rebalance_direct_production_no_final_frost_layers_to_storage(0, &mut debit_layers, 0.05)
            .expect("negative delta should debit layers");
        let aggregate: f64 = debit_layers
            .iter()
            .map(|layer| layer.theta_m + layer.residual_theta * layer.depth_m)
            .sum();
        assert!((aggregate - 0.05).abs() < 1.0e-12);

        let mut unchanged_layers = vec![cqr_row7_subsurface_layer(0.02)];
        rebalance_direct_production_no_final_frost_layers_to_storage(
            0,
            &mut unchanged_layers,
            0.04,
        )
        .expect("zero delta should leave storage unchanged");
        assert_eq!(unchanged_layers[0].theta_m.to_bits(), 0.02_f64.to_bits());

        let mut empty = Vec::new();
        assert!(
            rebalance_direct_production_no_final_frost_layers_to_storage(0, &mut empty, 0.01)
                .is_err()
        );

        let mut cannot_debit = vec![cqr_row7_subsurface_layer(0.0)];
        assert!(
            rebalance_direct_production_no_final_frost_layers_to_storage(
                0,
                &mut cannot_debit,
                0.0
            )
            .is_err()
        );

        let mut invalid_top_credit = vec![cqr_row7_subsurface_layer(-1.0)];
        assert!(
            rebalance_direct_production_no_final_frost_layers_to_storage(
                0,
                &mut invalid_top_credit,
                0.0
            )
            .is_err()
        );

        let mut invalid = vec![cqr_row7_subsurface_layer(f64::NAN)];
        assert!(
            rebalance_direct_production_no_final_frost_layers_to_storage(0, &mut invalid, 0.01)
                .is_err()
        );
    }

    fn runner_execution_lock() -> &'static Mutex<()> {
        static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        RUN_LOCK.get_or_init(|| Mutex::new(()))
    }

    fn r6_test_no_water_temperature() -> DirectPublicationWaterTemperatureOperands {
        DirectPublicationWaterTemperatureOperands {
            meltwater_temperature_c: None,
        }
    }

    fn r6_test_empty_profile() -> DirectPublicationProfileOperands {
        DirectPublicationProfileOperands {
            depth_mm: None,
            porosity_cap_mm: None,
            fc_store_mm: None,
            wp_store_mm: None,
        }
    }

    fn fixture_path(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/cli01")
            .join(name)
    }

    fn copy_fixture_to_temp(source_dir: &Path, prefix: &str) -> PathBuf {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("unix epoch should be before now")
            .as_nanos();
        let destination = std::env::temp_dir().join(format!("{prefix}_{timestamp}"));
        copy_dir_recursive(source_dir, &destination);
        destination
    }

    fn copy_dir_recursive(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).expect("destination directory should be creatable");

        for entry in fs::read_dir(source).expect("source directory should be readable") {
            let entry = entry.expect("directory entry should be readable");
            let source_path = entry.path();
            let destination_path = destination.join(entry.file_name());
            if source_path.is_dir() {
                copy_dir_recursive(&source_path, &destination_path);
            } else {
                fs::copy(&source_path, &destination_path).expect("file copy should succeed");
            }
        }
    }















    fn read_manifest_json(report: &HillslopeRunReport) -> serde_json::Value {
        let manifest_text = fs::read_to_string(&report.manifest_path).unwrap_or_else(|error| {
            panic!(
                "manifest should be readable at {}: {error}",
                report.manifest_path.display()
            )
        });
        serde_json::from_str(&manifest_text)
            .unwrap_or_else(|error| panic!("manifest should parse as JSON: {error}"))
    }

    fn assert_json_i64(document: &serde_json::Value, pointer: &str, expected: i64) {
        let observed = document
            .pointer(pointer)
            .and_then(serde_json::Value::as_i64)
            .unwrap_or_else(|| panic!("missing integer JSON pointer {pointer}"));
        assert_eq!(observed, expected, "unexpected value at {pointer}");
    }
}
