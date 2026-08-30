#[cfg(test)]
mod tests {
    use super::*;
    use crate::SidecarPolicy;
    use crate::hillslope::HillslopeDirectClimateDayForcing;
    use openwepp_hillslope_orchestrator::{
        DIRECT_R3B_PHASE_SPAN_COUNT, DIRECT_R3C_PHASE_SPAN_COUNT, DIRECT_R4A_PHASE_SPAN_COUNT,
        DIRECT_R4B_PHASE_SPAN_COUNT, DIRECT_R4C_PHASE_SPAN_COUNT, DIRECT_R4G_PHASE_SPAN_COUNT,
        DIRECT_R4I_PHASE_SPAN_COUNT, DIRECT_R4J_PHASE_SPAN_COUNT, DIRECT_R4K_PHASE_SPAN_COUNT,
        DIRECT_R4L_PHASE_SPAN_COUNT, DIRECT_R4M_PHASE_SPAN_COUNT, DIRECT_R4N_PHASE_SPAN_COUNT,
        DIRECT_R4O_PHASE_SPAN_COUNT, DIRECT_R4PQZ_PHASE_SPAN_COUNT,
        DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT, DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT,
        DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT, DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT,
        DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT, DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT,
        DirectPublicationCalendarDay, DirectPublicationClimateOperands, DirectPublicationDayRow,
        DirectPublicationErosionOperands, DirectPublicationEvaporationOperands,
        DirectPublicationInterceptionOperands, DirectPublicationLiquidInputOperands,
        DirectPublicationProfileOperands, DirectPublicationRunMetadata,
        DirectPublicationRunoffOperands, DirectPublicationStorageOperands,
        DirectPublicationSubsurfaceOperands, DirectPublicationTransferOperands,
        DirectPublicationWaterTemperatureOperands, DirectRunIdentity, DirectRunPublicationFrame,
        reset_direct_runtime_audit_counters,
    };
    use openwepp_input_contract::parsers::hbp::{
        HbpParseOptions, parse_hbp_from_bytes_with_latest_event_payload, parse_hbp_from_path,
    };
    use openwepp_input_contract::parsers::slope::{
        DatverSource, DistanceMode, SlopeOfe, SlopePoint, SlopeProfile,
    };
    use std::fs;
    use std::path::Path;
    use std::sync::{Mutex, OnceLock};

    mod simimpl {
        include!("tests03/simimpl.rs");
    }
    mod cqr_growth_authority {
        include!("tests03/cqr_growth_authority.rs");
    }
    mod cqr_laned_active_outputs {
        include!("tests03/cqr_laned_active_outputs.rs");
    }
    include!("tests03/direct_publication_source_guards.rs");
    include!("tests03/canopy_litter_boundary_helpers.rs");
    include!("tests03/eb04w2b_warm_snow.rs");
    include!("tests03/stage3_evaluation_publication_parity.rs");
    include!("tests03/stage3_runner_qualification.rs");
    include!("tests03/wat5_output_transaction.rs");

    #[test]
    fn stage3_v11_canopy_postures_admit_open_crop_and_native_forest_authority() {
        assert_eq!(
            stage3_v11_nonforest_canopy_cover(0.0)
                .expect("open canopy is an admitted limiting posture")
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            stage3_v11_nonforest_canopy_cover(0.37)
                .expect("management-derived crop canopy is admitted")
                .to_bits(),
            0.37_f64.to_bits()
        );
        let forest = Stage3V11NativeForestCanopyOperands {
            evergreen_fraction: 0.25,
            summer_foliar_biomass_kg_m2: 0.8,
            structural_canopy_cover_fraction: 0.1,
            canopy_cover_coefficient_m2_kg: 1.2,
        };
        let expected_forest = 0.1_f64
            .max(1.0 - (-1.2_f64 * (0.8 * (0.25 + 0.75 * 0.4))).exp())
            .min(openwepp_plant_phenology::FOREST_CANOPY_COVER_CAP);
        assert_eq!(
            stage3_v11_canopy_cover_from_authority(Some(forest), 0.4, 0.0)
                .expect("native forest canopy retains its GSI-derived authority")
                .to_bits(),
            expected_forest.to_bits()
        );
        for invalid in [-f64::EPSILON, 1.0, f64::NAN, f64::INFINITY] {
            assert!(stage3_v11_nonforest_canopy_cover(invalid).is_err());
        }
    }

    #[test]
    fn wat5_named_unit_conversions_validate_known_vectors_and_domains() {
        assert!((wat5_depth_mm(0.001).expect("depth conversion") - 1.0).abs() <= f64::EPSILON);
        assert!((wat5_rate_mm_h(1.0e-6).expect("rate conversion") - 3.6).abs() <= f64::EPSILON);
        assert!(
            (wat5_signed_depth_mm(-1.0e-6).expect("signed residual conversion") + 0.001).abs()
                <= f64::EPSILON
        );
        assert!(wat5_depth_mm(-1.0).is_err());
        assert!(wat5_rate_mm_h(f64::NAN).is_err());
        assert!(wat5_signed_depth_mm(f64::INFINITY).is_err());
    }

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

        let temp_run_dir = prepare_explicit_stage3_fixture_dir(prefix, false);
        let output_dir = temp_run_dir.join("output");

        let report =
            crate::hillslope::snow_stage3_v11_production_seed::with_explicit_test_owner_seed(
                || {
                    execute_hillslope_run_with_runtime_policy(
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
                    .expect("fixture run should complete")
                },
            );

        (report, temp_run_dir)
    }

    #[test]
    fn runner_management_intake_dispatches_canonical_yaml_path() {
        let management_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man.yaml",
        );
        let request = HillslopeRunRequest {
            run_dir: PathBuf::new(),
            run_file: PathBuf::new(),
            output_dir: PathBuf::new(),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };

        let management = parse_hillslope_management_input(&request, &management_path)
            .expect("runner intake should dispatch YAML management path");
        assert_eq!(management.datver, "ow-lanuse-1");
        let openwepp_input_contract::parsers::management::PlantScenarioData::Forest(plant) =
            &management.registries.plants[0].data
        else {
            panic!("expected YAML native forest plant");
        };
        let routing = plant
            .routing
            .expect("YAML route coefficients should reach runner intake output");
        assert!((routing.skin_friction_coefficient_ko - 500.0).abs() <= 1.0e-12);
    }

    #[test]
    fn canopy_research_trace_configuration_and_io_fail_closed() {
        assert!(
            canopy_research_trace_config_from_values(None, None, None)
                .expect("an absent selector should be valid")
                .is_none(),
            "the research trace must remain default-off"
        );
        let path = Some(std::ffi::OsString::from("trace.jsonl"));
        let missing_site =
            canopy_research_trace_config_from_values(path.clone(), None, Some("arm".to_string()))
                .expect_err("an enabled trace without site identity must fail");
        assert!(missing_site.to_string().contains("SITE_ID"));
        let missing_arm =
            canopy_research_trace_config_from_values(path, Some("site".to_string()), None)
                .expect_err("an enabled trace without arm identity must fail");
        assert!(missing_arm.to_string().contains("ARM_ID"));

        let invalid_parent = std::env::temp_dir()
            .join("openwepp-canopy-research-missing-parent")
            .join("trace.jsonl");
        let io_error = write_canopy_research_trace_line(
            invalid_parent.as_os_str(),
            b"{\"schema\":\"probe\"}\n",
        )
        .expect_err("a missing trace parent must fail closed");
        assert!(io_error.to_string().contains("failed opening"));

        let nonfinite = serde_json::json!({
            "required": serde_json::Value::Null,
            "residue": {
                "needle_litter_input_kg_m2": serde_json::Value::Null,
                "fine_woody_litter_input_kg_m2": serde_json::Value::Null
            },
            "consumers": {
                "erosion_canopy_cover_fraction": serde_json::Value::Null,
                "frost_residue_depth_m": serde_json::Value::Null
            }
        });
        assert!(
            validate_canopy_research_trace_value(&nonfinite)
                .expect_err("a null required number must fail")
                .to_string()
                .contains("required")
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn native_height_contract_vectors_advance_real_gsi_state() {
        use openwepp_plant_phenology::{
            ForestCanopyParameters, ForestCanopyState, GsiDailyForcing, GsiDate, GsiParameters,
        };

        let parameters =
            |evergreen_fraction: f64, structural_biomass_kg_m2: f64| ForestCanopyParameters {
                gsi: GsiParameters {
                    minimum_temperature_inactive_c: 0.0,
                    minimum_temperature_unconstrained_c: 10.0,
                    vapor_pressure_deficit_unconstrained_pa: 1_000.0,
                    vapor_pressure_deficit_inactive_pa: 4_000.0,
                    photoperiod_inactive_hours: 0.0,
                    photoperiod_unconstrained_hours: 12.0,
                },
                summer_foliar_biomass_kg_m2: 0.8,
                maximum_leaf_area_index: 5.0,
                evergreen_fraction,
                structural_canopy_cover_fraction: 0.0,
                structural_biomass_kg_m2,
                canopy_cover_coefficient_m2_kg: 2.0,
            };
        let forcing = |ordinal_day: u16, minimum_temperature_c: f64| GsiDailyForcing {
            minimum_temperature_c,
            vapor_pressure_deficit_pa: 500.0,
            latitude_degrees: 0.0,
            date: GsiDate {
                year: 2027,
                ordinal_day,
            },
        };
        let independent_height =
            |foliar: f64, structural: f64| (1.0 - (-3.0_f64 * (foliar + structural)).exp()) * 0.2;

        let mut deciduous = ForestCanopyState::new_uninitialized();
        let off = deciduous
            .advance(parameters(0.0, 0.0), forcing(1, -1.0))
            .expect("TV-PLANT-GSI-HC-001 exact inactive state");
        assert_eq!(off.gsi.growing_season_index.to_bits(), 0.0_f64.to_bits());
        assert_eq!(off.canopy.leaf_area_index.to_bits(), 0.0_f64.to_bits());
        let off_height = openwepp_hillslope_orchestrator::direct_native_canopy_height_m(
            off.canopy.live_foliar_biomass_kg_m2,
            off.canopy.structural_biomass_kg_m2,
            3.0,
            0.2,
        )
        .expect("inactive deciduous height");
        assert_eq!(off_height.to_bits(), 0.0_f64.to_bits());

        let on = deciduous
            .advance(parameters(0.0, 0.0), forcing(2, 10.0))
            .expect("TV-PLANT-GSI-HC-001 exact zero-to-positive transition");
        assert!(on.gsi.growing_season_index > 0.0);
        assert!(on.canopy.leaf_area_index > 0.0);
        let on_height = openwepp_hillslope_orchestrator::direct_native_canopy_height_m(
            on.canopy.live_foliar_biomass_kg_m2,
            on.canopy.structural_biomass_kg_m2,
            3.0,
            0.2,
        )
        .expect("active deciduous height");
        assert_eq!(
            on_height.to_bits(),
            independent_height(
                on.canopy.live_foliar_biomass_kg_m2,
                on.canopy.structural_biomass_kg_m2,
            )
            .to_bits()
        );
        assert!(on_height > 0.0);

        let mut leaf_off = on;
        for ordinal_day in 3..=23 {
            leaf_off = deciduous
                .advance(parameters(0.0, 0.0), forcing(ordinal_day, -1.0))
                .expect("TV-PLANT-GSI-HC-002 positive-to-zero sequence");
        }
        assert_eq!(
            leaf_off.gsi.growing_season_index.to_bits(),
            0.0_f64.to_bits()
        );
        assert!(leaf_off.canopy.leaf_off_litter_kg_m2 > 0.0);
        assert_eq!(leaf_off.canopy.leaf_area_index.to_bits(), 0.0_f64.to_bits());
        let leaf_off_height = openwepp_hillslope_orchestrator::direct_native_canopy_height_m(
            leaf_off.canopy.live_foliar_biomass_kg_m2,
            leaf_off.canopy.structural_biomass_kg_m2,
            3.0,
            0.2,
        )
        .expect("deciduous leaf-off height");
        assert_eq!(leaf_off_height.to_bits(), 0.0_f64.to_bits());

        let mut structural = ForestCanopyState::new_uninitialized();
        let structural_off = structural
            .advance(parameters(0.0, 0.1), forcing(1, -1.0))
            .expect("TV-PLANT-GSI-HC-002 structural leaf-off");
        assert_eq!(
            structural_off.canopy.leaf_area_index.to_bits(),
            0.0_f64.to_bits()
        );
        let structural_height = openwepp_hillslope_orchestrator::direct_native_canopy_height_m(
            structural_off.canopy.live_foliar_biomass_kg_m2,
            structural_off.canopy.structural_biomass_kg_m2,
            3.0,
            0.2,
        )
        .expect("structural leaf-off height");
        assert!(structural_height > 0.0);

        let mut evergreen = ForestCanopyState::new_uninitialized();
        let evergreen_off = evergreen
            .advance(parameters(0.25, 0.1), forcing(1, -1.0))
            .expect("TV-PLANT-GSI-HC-003 evergreen GSI21-zero floor");
        assert_eq!(
            evergreen_off.gsi.growing_season_index.to_bits(),
            0.0_f64.to_bits()
        );
        assert!(evergreen_off.canopy.live_foliar_biomass_kg_m2 > 0.0);
        assert!(evergreen_off.canopy.leaf_area_index > 0.0);
        let evergreen_height = openwepp_hillslope_orchestrator::direct_native_canopy_height_m(
            evergreen_off.canopy.live_foliar_biomass_kg_m2,
            evergreen_off.canopy.structural_biomass_kg_m2,
            3.0,
            0.2,
        )
        .expect("evergreen floor height");
        assert!(evergreen_height > 0.0);
    }

    #[test]
    fn structural_height_underflow_leaves_native_gsi_state_uncommitted() {
        use openwepp_plant_phenology::{
            ForestCanopyParameters, ForestCanopyState, GsiDailyForcing, GsiDate, GsiParameters,
        };

        let parameters = ForestCanopyParameters {
            gsi: GsiParameters::generalized(),
            summer_foliar_biomass_kg_m2: f64::MIN_POSITIVE,
            maximum_leaf_area_index: 1.0,
            evergreen_fraction: 0.0,
            structural_canopy_cover_fraction: 0.0,
            structural_biomass_kg_m2: f64::MIN_POSITIVE,
            canopy_cover_coefficient_m2_kg: 1.0,
        };
        let forcing = GsiDailyForcing {
            minimum_temperature_c: -100.0,
            vapor_pressure_deficit_pa: 0.0,
            latitude_degrees: 0.0,
            date: GsiDate {
                year: 2027,
                ordinal_day: 1,
            },
        };
        let mut preview = ForestCanopyState::new_uninitialized();
        let daily = preview
            .advance(parameters, forcing)
            .expect("the provisional GSI candidate itself is valid");
        assert_eq!(daily.canopy.leaf_area_index.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            daily.canopy.live_foliar_biomass_kg_m2.to_bits(),
            0.0_f64.to_bits()
        );
        assert!(daily.canopy.structural_biomass_kg_m2 > 0.0);

        let mut committed = Some(ForestCanopyState::new_uninitialized());
        let snapshot = committed.clone();
        let error = advance_native_canopy_with_checked_height(
            &mut committed,
            parameters,
            forcing,
            f64::MIN_POSITIVE,
            0.2,
        )
        .expect_err("TV-PLANT-GSI-HC-005 structural-only underflow must fail");
        assert!(error.to_string().contains("growth.native_canopy_height_m"));
        assert_eq!(
            committed, snapshot,
            "failed provisional height must not mutate the committed GSI state"
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn native_forest_yaml_executes_through_the_direct_production_consumer() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(
            &source_fixture_dir,
            "canopy_phenology_02_native_forest_direct",
        );
        let management_source = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man.yaml",
        );
        fs::copy(&management_source, temp_run_dir.join("case.man.yaml"))
            .expect("native forest management should copy into the run directory");
        install_authenticated_litter_forcing(&temp_run_dir);
        let run_path = temp_run_dir.join("case.run");
        let run_text = fs::read_to_string(&run_path).expect("run file should be readable");
        fs::write(
            &run_path,
            run_text.replace(
                "management = \"case.man\"",
                "management = \"case.man.yaml\"",
            ),
        )
        .expect("run file should select native forest YAML");
        let soil_path = temp_run_dir.join("case.sol");
        let soil_text = fs::read_to_string(&soil_path).expect("soil file should be readable");
        fs::write(
            &soil_path,
            soil_text.replace("1 forest silt_loam", "1 forest_high_sev_fire silt_loam"),
        )
        .expect("soil class should match the native forest fixture");
        fs::write(
            temp_run_dir.join("pmetpara.txt"),
            "3\nCORN,1.20,0.55,1,default\nWHEAT,1.05,0.45,2,cover\nForest_High_Severity_Fire,0.20,0.45,1,native forest\n",
        )
        .expect("native forest PMET authority should be explicit");
        let climate_path = temp_run_dir.join("case.cli");
        let climate_text = fs::read_to_string(&climate_path).expect("climate should be readable");
        fs::write(
            &climate_path,
            climate_text
                .replace("45.0 -120.0 1000.0", "0.0 -120.0 1000.0")
                .replace(
                    "1 1 2000 10.0 2.0 0.25 3.0 12.0 2.0 200.0 3.0 180.0 -1.0",
                    "1 1 2000 10.0 2.0 0.25 3.0 20.0 10.0 200.0 3.0 180.0 9.0",
                )
                .replace(
                    "2 1 2000 0.0 0.0 0.0 0.0 10.0 1.0 190.0 2.5 170.0 -2.0",
                    "2 1 2000 0.0 0.0 0.0 0.0 0.0 -5.0 190.0 2.5 170.0 -6.0",
                ),
        )
        .expect("native forest climate cycle should be explicit");

        let research_trace_path = temp_run_dir.join("canopy-research.jsonl");
        set_canopy_research_trace_test_config(Some(CanopyResearchTraceConfig {
            path: research_trace_path.clone().into_os_string(),
            site_id: "test_site".to_string(),
            arm_id: "native_forest".to_string(),
        }));
        reset_direct_runtime_audit_counters();
        reset_native_canopy_runtime_traces();
        let request = HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir: temp_run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };
        test_fixture_authority::author_stage3_v11_owner_seed_fixture(
            &request,
            test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
            test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
        )
        .expect("native-forest fixture should bind its exact Stage-3 owner seed");
        let report = execute_hillslope_run_with_runtime_policy(
            &request,
            &["openwepp-cli-hill".to_string()],
            HillslopeRuntimeSelectionPolicy::default(),
        )
        .expect("native GSI forest should execute through direct production");
        set_canopy_research_trace_test_config(None);

        assert!(report.output_pass.is_file());
        let research_text = fs::read_to_string(&research_trace_path)
            .expect("enabled research trace must exercise the actual JSONL writer");
        let first_research: serde_json::Value = serde_json::from_str(
            research_text
                .lines()
                .next()
                .expect("enabled research trace must contain a daily record"),
        )
        .expect("research trace must be valid JSON");
        assert_eq!(
            first_research
                .pointer("/schema")
                .and_then(serde_json::Value::as_str),
            Some("openwepp-canopy-research-daily-v1")
        );
        assert_eq!(
            first_research
                .pointer("/site_id")
                .and_then(serde_json::Value::as_str),
            Some("test_site")
        );
        assert_eq!(
            first_research
                .pointer("/arm_id")
                .and_then(serde_json::Value::as_str),
            Some("native_forest")
        );
        assert_eq!(
            first_research
                .pointer("/residue/needle_litter_status")
                .and_then(serde_json::Value::as_str),
            Some("complete")
        );
        assert_eq!(
            first_research
                .pointer("/residue/needle_litter_source_mode")
                .and_then(serde_json::Value::as_str),
            Some("prescribed_scenario")
        );
        assert_eq!(
            first_research
                .pointer("/residue/fine_woody_litter_status")
                .and_then(serde_json::Value::as_str),
            Some("complete")
        );
        let research_number = |path: &str| {
            first_research
                .pointer(path)
                .and_then(serde_json::Value::as_f64)
                .unwrap_or_else(|| panic!("research trace must expose numeric {path}"))
        };
        assert_eq!(
            research_number("/residue/needle_litter_input_kg_m2").to_bits(),
            0.002_f64.to_bits()
        );
        assert_eq!(
            research_number("/residue/fine_woody_litter_input_kg_m2").to_bits(),
            0.003_f64.to_bits()
        );
        let total_litter = research_number("/residue/total_litter_input_kg_m2");
        let reconstructed_sources =
            research_number("/residue/leaf_litter_input_kg_m2") + 0.002 + 0.003;
        assert!((total_litter - reconstructed_sources).abs() <= 1.0e-15);
        let decay = research_number("/residue/surface_decay_factor");
        for (before, after) in [
            (
                "/residue/surface_residue_before_kg_m2",
                "/residue/surface_residue_after_kg_m2",
            ),
            (
                "/residue/interrill_ground_residue_before_kg_m2",
                "/residue/interrill_ground_residue_after_kg_m2",
            ),
            (
                "/residue/rill_ground_residue_before_kg_m2",
                "/residue/rill_ground_residue_after_kg_m2",
            ),
        ] {
            let independently_reconstructed = (research_number(before) + total_litter) * decay;
            assert!(
                (research_number(after) - independently_reconstructed).abs() <= 1.0e-12,
                "{after} must consume the same independently reconstructed daily source"
            );
        }
        let interrill_mass = research_number("/residue/interrill_ground_residue_after_kg_m2");
        let rill_mass = research_number("/residue/rill_ground_residue_after_kg_m2");
        let weight = research_number("/residue/rescov_interrill_weight");
        let cover_factor = research_number("/residue/residue_cover_factor_m2_kg");
        let expected_interrill_cover = (1.0 - (-cover_factor * interrill_mass).exp()).min(0.999);
        let expected_rill_cover = (1.0 - (-cover_factor * rill_mass).exp()).min(0.999);
        let expected_weighted_mass = weight * interrill_mass + (1.0 - weight) * rill_mass;
        let expected_composite_cover =
            weight * expected_interrill_cover + (1.0 - weight) * expected_rill_cover;
        for (label, actual, expected) in [
            (
                "interrill cover reconstruction",
                research_number("/residue/interrill_cover_fraction"),
                expected_interrill_cover,
            ),
            (
                "rill cover reconstruction",
                research_number("/residue/rill_cover_fraction"),
                expected_rill_cover,
            ),
            (
                "weighted ground-mass reconstruction",
                research_number("/residue/weighted_ground_residue_after_kg_m2"),
                expected_weighted_mass,
            ),
            (
                "composite cover reconstruction",
                research_number("/residue/composite_cover_fraction"),
                expected_composite_cover,
            ),
            (
                "residue-depth conversion",
                research_number("/residue/residue_depth_m"),
                research_number("/residue/surface_residue_after_kg_m2")
                    * research_number("/residue/residue_depth_conversion_m_per_kg_m2"),
            ),
        ] {
            assert!(
                (actual - expected).abs() <= 1.0e-12,
                "{label}: expected {expected:.17}, observed {actual:.17}"
            );
        }
        assert_eq!(
            first_research.pointer("/consumers/erosion_interrill_cover_fraction"),
            first_research.pointer("/residue/interrill_cover_fraction")
        );
        assert_eq!(
            first_research.pointer("/consumers/erosion_rill_cover_fraction"),
            first_research.pointer("/residue/rill_cover_fraction")
        );
        assert_eq!(
            first_research.pointer("/canopy/total_foliar_biomass_kg_m2"),
            first_research.pointer("/consumers/growth_live_foliar_biomass_kg_m2")
        );
        assert_eq!(
            first_research.pointer("/canopy/leaf_off_transfer_kg_m2"),
            first_research.pointer("/residue/leaf_litter_input_kg_m2")
        );
        let manifest = read_manifest_json(&report);
        assert_eq!(
            manifest
                .pointer("/runtime_selection/selected")
                .and_then(serde_json::Value::as_str),
            Some("direct-production-executor")
        );

        let traces = take_native_canopy_consumer_traces();
        assert!(
            !traces.is_empty(),
            "the real direct run must produce native canopy consumer traces"
        );
        let assert_close = |label: &str, actual: f64, expected: f64| {
            assert!(
                (actual - expected).abs() <= 1.0e-12,
                "{label}: expected {expected:.17}, observed {actual:.17}"
            );
        };
        let first = traces[0];
        assert_eq!(first.builder.needle_litter_status, "complete");
        assert_eq!(
            first.builder.needle_litter_source_mode,
            "prescribed_scenario"
        );
        assert_eq!(first.builder.fine_woody_litter_status, "complete");
        assert_eq!(
            first.builder.fine_woody_litter_source_mode,
            "prescribed_scenario"
        );
        assert_close(
            "cold-start leaf-on allocation",
            first.builder.canopy.leaf_on_allocation_kg_m2,
            0.0,
        );
        assert_close(
            "cold-start leaf-off litter",
            first.builder.canopy.leaf_off_litter_kg_m2,
            0.0,
        );

        let mut observed_erosion_consumer = false;
        let mut observed_leaf_off_frost_consumer = false;
        for trace in &traces {
            let canopy = trace.builder.canopy;
            assert_close(
                "growth live foliar biomass",
                trace.growth_state_after.live_biomass_kg_m2,
                canopy.live_foliar_biomass_kg_m2,
            );
            assert_close(
                "growth interception biomass",
                trace.growth_state_after.interception_live_biomass_kg_m2,
                canopy.live_foliar_biomass_kg_m2,
            );
            assert_close(
                "growth leaf area index",
                trace.growth_state_after.leaf_area_index,
                canopy.leaf_area_index,
            );
            assert_close(
                "growth canopy cover",
                trace.growth_state_after.canopy_cover_fraction,
                canopy.canopy_cover_fraction,
            );
            let total_aboveground_biomass_kg_m2 =
                canopy.live_foliar_biomass_kg_m2 + canopy.structural_biomass_kg_m2;
            assert!(total_aboveground_biomass_kg_m2.is_finite());
            let expected_canopy_height_m =
                (1.0 - (-3.0 * total_aboveground_biomass_kg_m2).exp()) * 0.2;
            assert!(expected_canopy_height_m.is_finite());
            assert_close(
                "growth canopy height",
                trace.growth_state_after.canopy_height_m,
                expected_canopy_height_m,
            );
            assert_close(
                "snow canopy argument",
                trace.builder.snow_canopy_cover_fraction,
                canopy.canopy_cover_fraction,
            );
            assert_close(
                "WB15 canopy argument",
                trace.builder.interception_inputs.canopy_cover_fraction,
                canopy.canopy_cover_fraction,
            );
            assert_close(
                "WB15 leaf-area argument",
                trace.builder.interception_inputs.leaf_area_index,
                canopy.leaf_area_index,
            );
            assert_close(
                "WB15 foliar-biomass argument",
                trace
                    .builder
                    .interception_inputs
                    .interception_live_biomass_kg_m2,
                canopy.live_foliar_biomass_kg_m2,
            );
            let independently_computed_interception =
                compute_direct_canopy_interception(trace.builder.interception_inputs)
                    .expect("the traced WB15 inputs must independently recompute");
            assert_close(
                "WB15 independently reconstructed interception",
                trace.interception_m,
                independently_computed_interception.interception_m,
            );
            assert_close(
                "WB15 builder/executor interception",
                trace.interception_m,
                trace.builder.interception_state.interception_m,
            );
            assert_close(
                "ET leaf area index",
                trace.et_leaf_area_index,
                canopy.leaf_area_index,
            );
            assert_close(
                "ET canopy cover",
                trace.et_canopy_cover_fraction,
                canopy.canopy_cover_fraction,
            );
            assert_close(
                "ET canopy height",
                trace.et_canopy_height_m,
                expected_canopy_height_m,
            );
            assert_close(
                "Lane D shadow canopy height",
                trace.laned_shadow_canopy_height_m_consumed,
                expected_canopy_height_m,
            );
            if let Some(laned_active_height_m) = trace.laned_active_canopy_height_m_consumed {
                assert_close(
                    "Lane D active canopy height",
                    laned_active_height_m,
                    expected_canopy_height_m,
                );
                assert_close(
                    "Lane D active/shadow canopy height",
                    laned_active_height_m,
                    trace.laned_shadow_canopy_height_m_consumed,
                );
            }
            assert_close(
                "three-source litter decomposition handoff",
                trace.decomposition_litter_kg_m2,
                canopy.leaf_off_litter_kg_m2
                    + trace
                        .builder
                        .needle_litter_input_kg_m2
                        .expect("complete needle forcing publishes a number")
                    + trace
                        .builder
                        .fine_woody_litter_input_kg_m2
                        .expect("complete fine-woody forcing publishes a number"),
            );
            assert_close(
                "surface residue consumer",
                trace.decomposition_surface_residue_kg_m2,
                trace.builder.projected_surface_residue_kg_m2,
            );
            assert_close(
                "residue depth consumer",
                trace.decomposition_residue_depth_m,
                trace.builder.projected_residue_depth_m,
            );
            if let Some(frost_residue_depth_m) = trace.builder.frost_residue_depth_m {
                let consumed = trace
                    .frost_residue_depth_m_consumed
                    .expect("the real frost span must consume the traced thermal inputs");
                let canopy_height_consumed = trace
                    .frost_canopy_height_m_consumed
                    .expect("the real frost span must consume dynamic canopy height");
                assert_close(
                    "frost thermal residue depth",
                    consumed,
                    trace.builder.projected_residue_depth_m,
                );
                assert_close(
                    "frost builder/consumer residue depth",
                    consumed,
                    frost_residue_depth_m,
                );
                assert_close(
                    "frost dynamic canopy height",
                    canopy_height_consumed,
                    expected_canopy_height_m,
                );
                assert_close(
                    "frost builder/consumer canopy height",
                    canopy_height_consumed,
                    trace
                        .builder
                        .frost_canopy_height_m
                        .expect("active frost builder must expose canopy height"),
                );
                if canopy.leaf_off_litter_kg_m2 > 1.0e-12 {
                    observed_leaf_off_frost_consumer = true;
                }
            }
            if let Some(erosion_canopy_cover_fraction) = trace.erosion_canopy_cover_fraction {
                observed_erosion_consumer = true;
                assert_close(
                    "erosion canopy cover",
                    erosion_canopy_cover_fraction,
                    canopy.canopy_cover_fraction,
                );
                assert_close(
                    "erosion canopy height",
                    trace
                        .erosion_canopy_height_m
                        .expect("active erosion must consume post-growth canopy height"),
                    expected_canopy_height_m,
                );
            }
        }
        assert!(
            observed_erosion_consumer,
            "the real direct run must exercise the active erosion canopy consumer"
        );
        assert!(
            observed_leaf_off_frost_consumer,
            "the nonzero leaf-off day must reach the active frost residue-depth consumer"
        );
        assert!(
            traces
                .iter()
                .any(|trace| trace.builder.canopy.leaf_off_litter_kg_m2 > 1.0e-12),
            "the real direct run must exercise a nonzero leaf-off residue handoff"
        );

        fs::copy(&management_source, temp_run_dir.join("case.man.yaml"))
            .expect("unrepresented-source management should restore");
        let incomplete_trace_path = temp_run_dir.join("canopy-research-incomplete.jsonl");
        set_canopy_research_trace_test_config(Some(CanopyResearchTraceConfig {
            path: incomplete_trace_path.clone().into_os_string(),
            site_id: "test_site".to_string(),
            arm_id: "native_forest_incomplete_sources".to_string(),
        }));
        let incomplete_request = HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir: temp_run_dir.join("output-incomplete"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };
        test_fixture_authority::author_stage3_v11_owner_seed_fixture(
            &incomplete_request,
            test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
            test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
        )
        .expect("changed management fixture should reseal its exact Stage-3 owner seed");
        execute_hillslope_run_with_runtime_policy(
            &incomplete_request,
            &["openwepp-cli-hill".to_string()],
            HillslopeRuntimeSelectionPolicy::default(),
        )
        .expect("explicit unrepresented litter sources should remain executable and disclosed");
        set_canopy_research_trace_test_config(None);
        let incomplete_text =
            fs::read_to_string(incomplete_trace_path).expect("incomplete trace should exist");
        let incomplete: serde_json::Value = serde_json::from_str(
            incomplete_text
                .lines()
                .next()
                .expect("incomplete trace should contain a record"),
        )
        .expect("incomplete trace should be JSON");
        for tissue in ["needle", "fine_woody"] {
            assert_eq!(
                incomplete
                    .pointer(&format!("/residue/{tissue}_litter_status"))
                    .and_then(serde_json::Value::as_str),
                Some("not_represented")
            );
            assert_eq!(
                incomplete.pointer(&format!("/residue/{tissue}_litter_input_kg_m2")),
                Some(&serde_json::Value::Null),
                "unrepresented tissue must not publish a numeric zero"
            );
        }
        assert_eq!(
            incomplete
                .pointer("/residue/source_completeness")
                .and_then(serde_json::Value::as_str),
            Some("incomplete")
        );

        install_authenticated_litter_forcing(&temp_run_dir);
        let boundary_path = temp_run_dir.join("case.man.yaml");
        let boundary = fs::read_to_string(&boundary_path)
            .expect("authenticated boundary management should read")
            .replace("support_end: \"2000-01-02\"", "support_end: \"2000-01-01\"")
            .replace(
                "ca265fed6835ac2edd0ef88dcf306999820cf8ffbaec83c6da7428a71c5b1e1a",
                "3143a1e140f0e45046aad80cbd94c54dafa60699a20504052d1d0c33514d6b73",
            )
            .replace(
                "b3a5cef1814e4b2729e7e2ee534f2144b50dce03504e973d080830fdfd1ce23c",
                "3ec7442078217509fa5c91642dc6f874456c16bc324592215bd678ab848bbc47",
            );
        fs::write(&boundary_path, boundary).expect("narrow support management should write");
        fs::write(
            temp_run_dir.join("needle.csv"),
            "date,deposited_kg_m2\n2000-01-01,0.002\n",
        )
        .expect("narrow needle forcing should write");
        fs::write(
            temp_run_dir.join("fine-woody.csv"),
            "date,deposited_kg_m2\n2000-01-01,0.003\n",
        )
        .expect("narrow fine-woody forcing should write");
        let outside_request = HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir: temp_run_dir.join("output-outside-support"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };
        test_fixture_authority::author_stage3_v11_owner_seed_fixture(
            &outside_request,
            test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
            test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
        )
        .expect("narrow-support fixture should reseal its exact Stage-3 owner seed");
        let outside_error = execute_hillslope_run_with_runtime_policy(
            &outside_request,
            &["openwepp-cli-hill".to_string()],
            HillslopeRuntimeSelectionPolicy::default(),
        )
        .expect_err("runtime access outside authenticated support must fail");
        assert!(
            outside_error
                .to_string()
                .contains("outside authenticated support")
        );
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
                .is_none_or(serde_json::Value::is_null),
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
                .is_none_or(serde_json::Value::is_null),
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
                .is_none_or(serde_json::Value::is_null),
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
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/run_frame_constructions",
            1,
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/executor_constructions",
            1,
        );
        assert_json_i64(&manifest_json, "/direct_runtime_counters/skeleton_runs", 0);
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/publication_capture_runs",
            1,
        );
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
            assert!(
                value > 0,
                "{counter} must be nonzero for R7C direct production"
            );
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
        let unrelated_fields = reduced_wat_mismatch_fields(&[unrelated_direct], &[compatibility]);
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
        let unrelated_fields = reduced_wat_mismatch_fields(&[unrelated_direct], &[compatibility]);
        assert!(unrelated_fields.contains(&"Dp"));
        assert!(!r6g_wat_direct_et_storage_producer_gap(&unrelated_fields));
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
        let direct_rows = [compatibility_rows[0].clone(), direct_second, direct_third];
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
                    canopy_height_m: 0.15,
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
                    canopy_height_m: 0.15,
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
            bbb: 1.2,
            hmax: 1.1,
            beinp: 1.0,
            extnct: 0.5,
            hi: 0.4,
            xmxlai: 3.0,
            rsr: 0.2,
            rtmmax: 1.0,
            rdmax: 1.0,
            oratea: 0.0,
            orater: 0.0,
            forest_phenology: None,
            forest_litter_forcing: None,
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
            base: Some(0.0),
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
    #[allow(clippy::too_many_lines)]
    fn r6b_absent_operand_detector_suppresses_marker_for_nonzero_direct_operands() {
        let identity =
            DirectRunIdentity::new(19, 2637, 1, 1).expect("valid direct identity should construct");
        let zero_row = DirectPublicationDayRow {
            dc01_surface_hourly_weights: [0.0; 24],
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
                groundwater_baseflow_mm: 0.0,
                groundwater_baseflow_m3: 0.0,
                groundwater_deep_seepage_mm: 0.0,
                groundwater_deep_seepage_m3: 0.0,
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
        assert_eq!(wat_rows[0].qofe.to_bits(), 12.5_f64.to_bits()); // MOFEFID-B02: QOFE == Q
        assert_eq!(wat_rows[0].rm.to_bits(), 8.25_f64.to_bits());
        assert_eq!(wat_rows[0].base, Some(6.0));
        assert_eq!(pass_rows[0].runvol_m3.to_bits(), 4.0_f64.to_bits());
        assert_eq!(pass_rows[0].peakro_m3_s.to_bits(), 0.75_f64.to_bits());
        let loss_json: serde_json::Value =
            serde_json::from_str(&loss).expect("direct loss projection should be JSON");
        assert_eq!(loss_json["run_name"], "r6a_projection");
        assert!(manifest.contains("row_count=1"));
    }

    #[test]
    fn hb06_wat_projection_consumes_typed_wb13_guard_and_preserves_optional_profile() {
        let mut mismatch = r6a_direct_projection_fixture_frame();
        mismatch.rows[0].runoff.qofe_mm += 1.0;
        let error = build_hillslope_wat_rows_from_direct_publication(&mismatch)
            .expect_err("typed WB13 QOFE identity must guard runner WAT projection");
        assert!(error.to_string().contains("QOFE"));

        let mut storage_mismatch = r6a_direct_projection_fixture_frame();
        storage_mismatch.rows[0].storage.soil_water_total_mm -= 1.0;
        let error = build_hillslope_wat_rows_from_direct_publication(&storage_mismatch)
            .expect_err("typed WB13 storage-alias identity must guard runner WAT projection");
        assert!(error.to_string().contains("SoilWaterTotal"));

        let mut optional = r6a_direct_projection_fixture_frame();
        optional.rows[0].profile.depth_mm = None;
        optional.rows[0].profile.porosity_cap_mm = None;
        optional.rows[0].profile.fc_store_mm = None;
        optional.rows[0].profile.wp_store_mm = None;
        let error = build_hillslope_wat_rows_from_direct_publication(&optional)
            .expect_err("missing canonical WB13 profile fields must fail typed projection");
        assert!(error.to_string().contains("ProfileDepth"));

        let mut partial = r6a_direct_projection_fixture_frame();
        partial.rows[0].profile.fc_store_mm = None;
        let error = build_hillslope_wat_rows_from_direct_publication(&partial)
            .expect_err("partial canonical WB13 profile fields must fail typed projection");
        assert!(error.to_string().contains("ProfileFCStore"));
    }

    #[test]
    fn r6a_direct_hbp_writer_serializes_groundwater_payload_operands() {
        let frame = r6a_direct_projection_fixture_frame();
        let hbp_path = Path::new("H2637.hbp");
        let hbp_bytes = build_hbp_output_from_direct_publication(hbp_path, &frame)
            .expect("direct HBP projection should build");

        let (_parsed, latest_event_payload) = parse_hbp_from_bytes_with_latest_event_payload(
            &hbp_bytes,
            hbp_path,
            HbpParseOptions {
                expected_hillslope_id: Some(2637),
            },
        )
        .expect("direct HBP projection should parse");
        let payload = latest_event_payload.expect("direct HBP should expose latest event payload");

        assert!(
            (payload.baseflow_volume_m3 - 2.4).abs() <= 1.0e-9,
            "expected baseflow 2.4 m3, got {}",
            payload.baseflow_volume_m3
        );
        assert!(
            (payload.deep_seepage_volume_m3 - 1.2).abs() <= 1.0e-9,
            "expected deep seepage 1.2 m3, got {}",
            payload.deep_seepage_volume_m3
        );
    }

    fn r6a_direct_projection_fixture_frame() -> DirectRunPublicationFrame {
        let identity =
            DirectRunIdentity::new(19, 2637, 1, 1).expect("valid direct identity should construct");
        let row = DirectPublicationDayRow {
            dc01_surface_hourly_weights: [0.0; 24],
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
                qofe_mm: 12.5,
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
                groundwater_baseflow_mm: 6.0,
                groundwater_baseflow_m3: 2.4,
                groundwater_deep_seepage_mm: 3.0,
                groundwater_deep_seepage_m3: 1.2,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: 0.25,
                upstream_lateral_mm: 0.125,
            },
            storage: DirectPublicationStorageOperands {
                total_soil_mm: 110.0,
                soil_water_total_mm: 110.0,
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
                peak_runoff_rate_m_s: Some(0.75),
                runoff_duration_s: Some(1800.0),
                total_detachment_kg: Some(2.25),
                total_deposition_kg: Some(1.25),
                hbp_total_detachment_kg: Some(2.25),
                hbp_total_deposition_kg: Some(1.25),
                hbp_sediment_concentration_kg_m3: Some(0.1),
                sediment_concentration_kg_m3: Some([0.1, 0.2, 0.3, 0.4, 0.5]),
                hourly_runoff_fraction: None,
                hourly_sediment_mass_kg: None,
                enrichment_ratio: None,
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
        assert_eq!(
            wb13_publication.publication_area_m2.to_bits(),
            1200.0_f64.to_bits()
        );
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
            dc01_surface_hourly_weights: [0.0; 24],
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
                qofe_mm: 12.5 + offset,
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
                groundwater_baseflow_mm: 0.0,
                groundwater_baseflow_m3: 0.0,
                groundwater_deep_seepage_mm: 0.0,
                groundwater_deep_seepage_m3: 0.0,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: 0.25 + offset,
                upstream_lateral_mm: 0.125 + offset,
            },
            storage: DirectPublicationStorageOperands {
                total_soil_mm: 110.0 + offset,
                soil_water_total_mm: 110.0 + offset,
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
                peak_runoff_rate_m_s: Some(0.75 + offset),
                runoff_duration_s: Some(1800.0 + offset),
                total_detachment_kg: Some(2.25 + offset),
                total_deposition_kg: Some(1.25 + offset),
                hbp_total_detachment_kg: Some(2.25 + offset),
                hbp_total_deposition_kg: Some(1.25 + offset),
                hbp_sediment_concentration_kg_m3: Some(0.1 + offset),
                sediment_concentration_kg_m3: Some([0.1, 0.2, 0.3, 0.4, 0.5]),
                hourly_runoff_fraction: None,
                hourly_sediment_mass_kg: None,
                enrichment_ratio: None,
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
        let (depth_m, density_kg_m3) = layered_snow_frost_insulation_depth_density(&snow, 1.0)
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
        assert!(
            direct_production_sturm1995_climate_normals(&climate_request, &empty_span).is_err()
        );

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

    fn cqr_row7_growth_projection()
    -> openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces {
        let mut projection =
            openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces {
                pl_schedule_surface: BTreeMap::new(),
                pl_growth_surface: BTreeMap::new(),
                pl_decomp_surface: BTreeMap::new(),
                forest_litter_forcing: BTreeMap::new(),
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
            ("bbb", 1.2),
            ("hmax", 1.1),
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

    fn cqr_row7_add_native_phenology(
        projection: &mut openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    ) {
        for (root, value) in [
            ("forest_phenology_model", 1.0),
            ("forest_summer_foliar_biomass_kg_m2", 0.2),
            ("forest_evergreen_fraction", 0.2),
            ("forest_structural_canopy_cover_fraction", 0.2),
            ("forest_structural_biomass_kg_m2", 0.1),
            ("forest_minimum_temperature_inactive_c", -2.0),
            ("forest_minimum_temperature_unconstrained_c", 5.0),
            ("forest_vapor_pressure_deficit_unconstrained_pa", 900.0),
            ("forest_vapor_pressure_deficit_inactive_pa", 4_100.0),
            ("forest_photoperiod_inactive_hours", 10.0),
            ("forest_photoperiod_unconstrained_hours", 11.0),
        ] {
            cqr_row7_insert_projection_scalar(
                &mut projection.pl_growth_surface,
                direct_growth_slot_crop_symbol(1, 1, root),
                value,
            );
        }
    }

    #[test]
    fn native_forest_runtime_authority_rejects_gaps_and_stays_active_across_years() {
        let mut continuous = cqr_row7_growth_projection();
        for (root, value) in [("jdharv", 0.0), ("jdstop", 0.0)] {
            cqr_row7_insert_projection_scalar(
                &mut continuous.pl_growth_surface,
                direct_growth_slot_crop_symbol(1, 1, root),
                value,
            );
        }
        cqr_row7_add_native_phenology(&mut continuous);
        let crop = direct_production_typed_growth_crop_authority(&continuous, 1, 1)
            .expect("continuous native forest authority");
        for _runtime_year in 1..=3 {
            for runtime_day in 1..=365 {
                assert!(crop.active_on_day(runtime_day));
            }
        }

        let mut seasonal_perennial = continuous.clone();
        cqr_row7_insert_projection_scalar(
            &mut seasonal_perennial.pl_growth_surface,
            direct_growth_slot_crop_symbol(1, 1, "jdstop"),
            200.0,
        );
        assert!(direct_production_typed_growth_crop_authority(&seasonal_perennial, 1, 1).is_err());

        let mut annual = continuous;
        cqr_row7_insert_projection_scalar(
            &mut annual.pl_schedule_surface,
            direct_growth_schedule_slot_crop_symbol(1, 1, "imngmt"),
            1.0,
        );
        cqr_row7_insert_projection_scalar(
            &mut annual.pl_growth_surface,
            direct_growth_slot_crop_symbol(1, 1, "jdplt"),
            1.0,
        );
        assert!(direct_production_typed_growth_crop_authority(&annual, 1, 1).is_err());
    }

    fn cqr_row7_growth_surface(live_biomass_kg_m2: f64) -> DirectGrowthStateSurface {
        DirectGrowthStateSurface {
            live_biomass_kg_m2,
            ..DirectGrowthStateSurface::zero()
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
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
        missing_scalar
            .pl_growth_surface
            .remove(&BoundarySymbol::from(direct_growth_slot_crop_symbol(
                1, 1, "btemp",
            )));
        assert!(direct_production_typed_growth_crop_authority(&missing_scalar, 1, 1).is_err());

        let residue = DirectProductionResidueCoverState {
            interrill_ground_residue_kg_m2: 0.0,
            rill_ground_residue_kg_m2: 0.0,
            surface_residue_kg_m2: 0.0,
            root_residue_kg_m2: 0.0,
            pending_surface_litter_kg_m2: 0.2,
            residue_depth_m: 0.0,
        };
        let before = cqr_row7_growth_surface(2.0);
        let after = cqr_row7_growth_surface(1.5);
        let calendar_day = canonical_calendar_day_probe();

        let in_window = direct_production_surface_litter_projection(
            Some(&crop),
            &calendar_day,
            1,
            100,
            residue,
            before,
            after,
            None,
        )
        .expect("in-window litter should drop");
        assert_eq!(
            in_window.surface_litter_input_kg_m2.to_bits(),
            0.7_f64.to_bits()
        );
        assert_eq!(
            in_window.pending_surface_litter_after_kg_m2.to_bits(),
            0.0_f64.to_bits()
        );

        let out_of_window = direct_production_surface_litter_projection(
            Some(&crop),
            &calendar_day,
            1,
            1,
            residue,
            before,
            after,
            None,
        )
        .expect("out-of-window litter should remain pending");
        assert_eq!(
            out_of_window.surface_litter_input_kg_m2.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            out_of_window.pending_surface_litter_after_kg_m2.to_bits(),
            0.7_f64.to_bits()
        );

        let no_crop = direct_production_surface_litter_projection(
            None,
            &calendar_day,
            1,
            1,
            residue,
            before,
            after,
            None,
        )
        .expect("non-scheduled crop should emit daily litter directly");
        let native_residue = DirectProductionResidueCoverState {
            pending_surface_litter_kg_m2: 0.0,
            ..residue
        };
        let native = direct_production_surface_litter_projection(
            Some(&crop),
            &calendar_day,
            1,
            1,
            native_residue,
            before,
            after,
            Some(0.125),
        )
        .expect("native GSI litter should bypass the fall window");
        assert_eq!(
            native.surface_litter_input_kg_m2.to_bits(),
            0.125_f64.to_bits()
        );
        assert_eq!(
            native.pending_surface_litter_after_kg_m2.to_bits(),
            0.0_f64.to_bits()
        );
        assert!(
            direct_production_surface_litter_projection(
                Some(&crop),
                &calendar_day,
                1,
                1,
                residue,
                before,
                after,
                Some(0.125),
            )
            .is_err(),
            "native GSI must reject the compatibility pending bucket"
        );
        assert_eq!(
            no_crop.surface_litter_input_kg_m2.to_bits(),
            0.5_f64.to_bits()
        );
    }

    fn cqr_row7_forcing(rad_ly: f64, tmax_c: f64, tmin_c: f64) -> HillslopeDirectClimateDayForcing {
        HillslopeDirectClimateDayForcing {
            day: 1,
            month: 1,
            year: 2000,
            prcp_m: 0.0,
            tmax_c,
            tmin_c,
            rad_ly,
            vwind_m_s: 1.0,
            wind_deg: 180.0,
            tdpt_c: tmin_c,
            breakpoint_storm_start_h: None,
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

        assert!(
            authority
                .compute_demand(&cqr_row7_forcing(-1.0, 12.0, 2.0), 0.0, 0.0)
                .is_err()
        );
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
            rebalance_direct_production_no_final_frost_layers_to_storage(0, &mut cannot_debit, 0.0)
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

    fn zero_winter_frost_outcome()
    -> openwepp_hillslope_orchestrator::DirectWinterFrostPartitionOutcome {
        openwepp_hillslope_orchestrator::DirectWinterFrostPartitionOutcome {
            active_frost_coupling: false,
            dthaw_after_m: 0.0,
            nft_after: 0.0,
            infcap_frz_m_s: 0.0,
            soil_water_after_frwatc_m: None,
            frwatc_soil_water_before_m: 0.0,
            frwatc_soil_water_after_m: 0.0,
            frwatc_frozen_water_before_m: 0.0,
            frwatc_frozen_water_after_m: 0.0,
            frwatc_freeze_debit_m: 0.0,
            frwatc_thaw_credit_m: 0.0,
            frwatc_net_liquid_delta_m: 0.0,
            frozen_water_after_m: 0.0,
            frost_depth_after_m: 0.0,
            thdp_after_m: 0.0,
            tfrdp_after_m: 0.0,
            tthawd_after_m: 0.0,
            fgthwd_flag_after: 0.0,
            total_fine_layer_count: 0.0,
            conductivity_tilled_w_m_k: 0.0,
            conductivity_untilled_w_m_k: 0.0,
            conductivity_residue_w_m_k: 0.0,
            shadow_total_water_before_m: 0.0,
            shadow_total_water_after_m: 0.0,
            shadow_wb_delta_m: 0.0,
            shadow_frwatc_residual_m: 0.0,
            watpdg_m: 0.0,
            watbtm_m: 0.0,
            layer_projection: Vec::new(),
            layer_shadow_projection: Vec::new(),
            fine_layer_projection: Vec::new(),
        }
    }

    fn stale_frost_layer() -> openwepp_hillslope_orchestrator::DirectSubsurfaceLayerState {
        openwepp_hillslope_orchestrator::DirectSubsurfaceLayerState {
            theta_m: 0.180,
            field_capacity_m: 0.200,
            upper_limit_m: 0.500,
            conductivity_m_s: 1.0e-6,
            depth_m: 0.400,
            residual_theta: 0.050,
            frozen_depth_m: 0.100,
            frozen_water_m: 0.005,
            porosity: 0.5,
            field_capacity_theta: 0.25,
            coca: 1.0,
            lateral_conductivity_m_s: 1.0e-6,
        }
    }

    // WP-2 relocated contract (Codex review, accepted candidate): with the
    // single-solve rewire, clearing stale coarse-layer frost projections on
    // no-final-frost days is the runner authority channel's job
    // (same_day_frost_hydrology_layers with clear_no_final_hydrology_layers),
    // no longer R4A's. Pin it here at its owning boundary.
    #[test]
    fn same_day_frost_hydrology_layers_clear_stale_projection_and_preserve_target() {
        let layers = vec![stale_frost_layer()];
        let target_soil_water_m = 0.230;
        let outcome = zero_winter_frost_outcome();

        let cleared = direct_production_same_day_frost_hydrology_layers(
            0,
            &layers,
            &outcome,
            target_soil_water_m,
            true,
        )
        .expect("no-final-frost clear should succeed");
        assert_eq!(cleared.len(), 1);
        assert_eq!(cleared[0].frozen_depth_m.to_bits(), 0.0_f64.to_bits());
        assert_eq!(cleared[0].frozen_water_m.to_bits(), 0.0_f64.to_bits());
        let aggregate_m: f64 = cleared
            .iter()
            .map(|layer| layer.theta_m + layer.residual_theta * layer.depth_m)
            .sum();
        assert!(
            (aggregate_m - target_soil_water_m).abs() <= 1.0e-9,
            "cleared layers must rebalance liquid aggregate to the lane target: {aggregate_m} vs {target_soil_water_m}"
        );
    }

    #[test]
    fn intval_material_thaw_preserves_layer_basis_until_frwatc_projection() {
        let frozen_depth_m = 0.026_064_975_283_605_526;
        let residual_theta = 0.05;
        let depth_m = 0.4;
        let pre_frost_liquid_m = residual_theta * (depth_m - frozen_depth_m);
        let thaw_credit_m = residual_theta * frozen_depth_m;
        let mut layer = stale_frost_layer();
        layer.theta_m = 0.0;
        layer.depth_m = depth_m;
        layer.residual_theta = residual_theta;
        layer.frozen_depth_m = frozen_depth_m;
        layer.frozen_water_m = thaw_credit_m;
        let layers = vec![layer];
        let mut outcome = zero_winter_frost_outcome();
        outcome.active_frost_coupling = true;
        outcome.soil_water_after_frwatc_m = Some(pre_frost_liquid_m + thaw_credit_m);
        outcome.frwatc_soil_water_before_m = pre_frost_liquid_m;
        outcome.frwatc_soil_water_after_m = pre_frost_liquid_m + thaw_credit_m;
        outcome.frwatc_frozen_water_before_m = thaw_credit_m;
        outcome.frwatc_thaw_credit_m = thaw_credit_m;
        outcome.frwatc_net_liquid_delta_m = thaw_credit_m;

        let unchanged = direct_production_same_day_frost_hydrology_layers(
            0,
            &layers,
            &outcome,
            pre_frost_liquid_m,
            true,
        )
        .expect("material thaw outcome must retain its layer basis until R4W ingress");

        assert_eq!(unchanged, layers);
    }

    #[test]
    fn same_day_frost_hydrology_layers_keep_stale_projection_when_frost_is_final() {
        let layers = vec![stale_frost_layer()];
        let mut outcome = zero_winter_frost_outcome();
        outcome.frost_depth_after_m = 0.050;
        outcome.frozen_water_after_m = 0.002;

        let unchanged =
            direct_production_same_day_frost_hydrology_layers(0, &layers, &outcome, 0.230, true)
                .expect("final-frozen outcome must not clear the layer projection");
        assert_eq!(
            unchanged[0].frozen_depth_m.to_bits(),
            layers[0].frozen_depth_m.to_bits()
        );
        assert_eq!(
            unchanged[0].frozen_water_m.to_bits(),
            layers[0].frozen_water_m.to_bits()
        );
        assert_eq!(unchanged[0].theta_m.to_bits(), layers[0].theta_m.to_bits());
    }
}
