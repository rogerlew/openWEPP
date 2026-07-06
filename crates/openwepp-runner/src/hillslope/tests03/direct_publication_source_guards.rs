    #[test]
    fn r7c_direct_production_source_excludes_compatibility_entrypoints() {
        let source = include_str!("../05_runner_execution_and_outputs.rs");
        let direct_body = source
            .split("fn execute_hillslope_direct_production_days")
            .nth(1)
            .and_then(|tail| tail.split("\nfn ").next())
            .expect("R7C direct production function body must be present");
        for forbidden in [
            "execute_hillslope_climate_days(",
            "execute_with_kernel",
            "HillslopeKernelRequest",
            "DirectPublicationDayInputBuilder",
            "new_with_seed_surfaces_and_erosion_guard",
            "record_direct_runtime_compatibility_edge_invocation",
        ] {
            assert!(
                !direct_body.contains(forbidden),
                "R7C direct production body must not contain compatibility entrypoint {forbidden}"
            );
        }
    }

    #[test]
    fn compatibility_runtime_deletion_removes_obsolete_transition_modes() {
        let sources = [
            include_str!("../../api.rs"),
            include_str!("../../bin/openwepp-cli-hill.rs"),
            include_str!("../04_direct_publication.rs"),
            include_str!("../05_runner_execution_and_outputs.rs"),
            include_str!("../direct_publication/day_input_and_helpers/00_builders_and_authority.rs"),
            include_str!("../direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs"),
            include_str!("../direct_publication/day_input_and_helpers/00d_authority_runtime_impl.rs"),
        ]
        .join("\n");
        for forbidden in [
            "DirectSkeletonNoop",
            "DirectSkeletonShadowOnly",
            "DirectPublicationFrameShadow",
            "DirectPublicationFrameCutover",
            "--direct-runtime-skeleton",
            "--direct-publication-frame-shadow",
            "--direct-publication-frame-cutover",
            "DirectPublicationDayInputBuilder",
            "build_retained_direct_publication_frame",
            "RetainedDirectPublicationRequest",
            "build_direct_publication_execution_from_simulation_outputs",
        ] {
            assert!(
                !sources.contains(forbidden),
                "deleted compatibility transition surface reappeared: {forbidden}"
            );
        }
    }

    #[test]
    fn r7f_production_direct_uses_typed_day_input_builder() {
        let source = include_str!("../05_runner_execution_and_outputs.rs");
        let direct_body = source
            .split("fn execute_hillslope_direct_production_days")
            .nth(1)
            .and_then(|tail| tail.split("\nfn ").next())
            .expect("R7F direct production function body must be present");
        assert!(
            direct_body.contains("DirectProductionDayInputBuilder::new"),
            "production direct must use the typed R7F day-input builder"
        );
        assert!(
            !direct_body.contains("DirectPublicationDayInputBuilder"),
            "production direct must not use the compatibility-shaped publication day-input builder"
        );
    }

    #[test]
    fn typed_seed_authority_direct_setup_skips_symbol_map_seed_surface() {
        let source = include_str!("../00_runner_intake_and_lane_setup.rs");
        let setup_body = source
            .split("fn build_static_hillslope_runtime_setup")
            .nth(1)
            .and_then(|tail| tail.split("\nfn ").next())
            .expect("typed seed authority setup function body must be present");
        assert!(
            setup_body.contains("HillslopeRuntimeSelection::DirectProductionExecutor"),
            "direct production setup must assert the terminal direct-only runtime"
        );
        assert!(
            !setup_body.contains("HillslopeWritebackSurface::default()"),
            "direct production setup must not construct a symbol-map seed surface"
        );
        for forbidden in [
            "build_static_runtime_surface_parts(",
            "build_persistent_lane_state(",
            "build_hillslope_runtime_surface_from_soil",
            "build_hillslope_runtime_surface_from_management",
            "build_hillslope_runtime_surface_from_slope",
            "build_hillslope_runtime_surface_from_snow",
            "build_hillslope_runtime_surface_from_frost",
        ] {
            assert!(
                !setup_body.contains(forbidden),
                "direct production setup branch must not call symbol-map seed surface builder {forbidden}"
            );
        }
    }

    #[test]
    fn r7f_typed_day_input_hot_loop_excludes_runtime_surface_reads() {
        let source = direct_publication_day_input_and_helpers_source();
        let impl_body = source
            .split("impl<'a> DirectProductionDayInputBuilder<'a>")
            .nth(1)
            .expect("R7F typed production builder impl must be present");
        let build_body = impl_body
            .split("    fn build(")
            .nth(1)
            .and_then(|tail| tail.split("\n    fn lane_authority").next())
            .expect("R7F typed production builder hot-loop build body must be present");
        assert!(
            build_body.contains("direct_day_forcing"),
            "R7F hot loop must use typed climate forcing"
        );
        assert!(
            build_body.contains("frame.lanes"),
            "R7F hot loop must read committed direct lane state"
        );
        for forbidden in [
            "HillslopeWritebackSurface",
            "BoundarySymbol",
            "BoundaryValue",
            "merge_runtime_surfaces",
            "require_runtime_surface_scalar(",
            "runtime_surface_symbol_value(",
            "DirectPublicationDayInputBuilder",
            "record_direct_runtime_compatibility_edge_invocation",
        ] {
            assert!(
                !build_body.contains(forbidden),
                "R7F hot-loop build body must not contain runtime-surface or compatibility read {forbidden}"
            );
        }
    }

    #[test]
    fn r7g_runfile_mode_uses_sibling_snow_and_frost_sidecars() {
        let temp_dir = std::env::temp_dir().join(format!(
            "openwepp-r7g-sidecars-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        fs::create_dir_all(&temp_dir).expect("temp sidecar directory should be created");
        let snow_path = temp_dir.join("snow.txt");
        let frost_path = temp_dir.join("frost.txt");
        fs::write(&snow_path, "-2.0\n100.0\n250.0\n")
            .expect("snow sidecar fixture should be written");
        fs::write(&frost_path, "1 10 10\n1.0 1.0 1.0 0.00001 0.00001 0.5\n")
            .expect("frost sidecar fixture should be written");

        let request = HillslopeRunRequest {
            run_dir: temp_dir.clone(),
            run_file: std::path::PathBuf::from("run.toml"),
            output_dir: temp_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };
        let sidecar_overrides = RunfileSidecarOverrides::default();
        let mut resolved_sidecars = BTreeMap::new();
        let mut snow_input_path = None;
        let mut frost_input_path = None;

        let snow = parse_runfile_snow_sidecar(
            &request,
            &sidecar_overrides,
            &mut resolved_sidecars,
            &mut snow_input_path,
        )
        .expect("sibling snow sidecar should parse");
        let frost = parse_runfile_frost_sidecar(
            &request,
            &sidecar_overrides,
            &mut resolved_sidecars,
            &mut frost_input_path,
        )
        .expect("sibling frost sidecar should parse");

        assert!(snow.sidecar_present);
        assert!(!snow.defaults_applied);
        assert!((snow.rst - -2.0).abs() < 1.0e-12);
        assert_eq!(snow_input_path.as_ref(), Some(&snow_path));
        assert_eq!(
            resolved_sidecars.get("snow").map(String::as_str),
            Some(snow_path.to_str().expect("snow path should be UTF-8"))
        );
        assert!(frost.frost_file_present);
        assert_eq!(frost.wint_red, 1);
        assert_eq!(frost_input_path.as_ref(), Some(&frost_path));
        assert_eq!(
            resolved_sidecars.get("frost").map(String::as_str),
            Some(frost_path.to_str().expect("frost path should be UTF-8"))
        );

        fs::remove_dir_all(&temp_dir).expect("temp sidecar directory should be removed");
    }

    #[test]
    fn r7g_direct_production_hands_active_frost_context_to_r4a_without_material_gate() {
        let source = direct_publication_day_input_and_helpers_source();
        let impl_body = source
            .split("impl<'a> DirectProductionDayInputBuilder<'a>")
            .nth(1)
            .expect("R7G typed production builder impl must be present");
        let build_body = impl_body
            .split("    fn build(")
            .nth(1)
            .and_then(|tail| tail.split("\n    fn build_lane_authority").next())
            .expect("R7G typed production builder build body must be present");

        assert!(
            !build_body.contains("frost_runoff_surface")
                && !build_body.contains("frost_liquid_partition"),
            "production active frost contexts must not hand deleted frost bridge fields to R4A"
        );
        assert!(
            build_body
                .contains("day_input.winter_frost_compute_inputs = Some(frost_context.compute_inputs)"),
            "active frost contexts must hand typed winter frost compute inputs to R4A"
        );
        assert!(
            build_body.contains("frozen_infiltration_capacity_m_s"),
            "WB14 may consume the typed pre-runoff frozen infiltration capacity scalar"
        );
        assert!(
            !build_body.contains("direct_publication_frost_partition_has_material_state")
                && !build_body.contains("direct_production_frost_partition_requires_r4a"),
            "R7G must not gate R4A frost handoff on precomputed material state"
        );
    }

    #[test]
    fn r7g_direct_production_frost_uses_prior_snowpack_not_same_day_projection() {
        let source = direct_publication_day_input_and_helpers_source();
        let impl_body = source
            .split("impl DirectProductionSnowFrostAuthority")
            .nth(1)
            .expect("R7G snow/frost authority impl must be present");
        let typed_partition_body = impl_body
            .split("    fn typed_winter_frost_compute_inputs(")
            .nth(1)
            .and_then(|tail| tail.split("\n    fn typed_winter_frost_thermal_inputs").next())
            .expect("R7G typed winter frost compute helper body must be present");
        let typed_thermal_body = impl_body
            .split("    fn typed_winter_frost_thermal_inputs(")
            .nth(1)
            .and_then(|tail| tail.split("\n    fn snow_frost_insulation_depth_density").next())
            .expect("R7G typed winter frost thermal helper body must be present");
        let insulation_body = impl_body
            .split("    fn snow_frost_insulation_depth_density(")
            .nth(1)
            .and_then(|tail| tail.split("\n    fn compute_typed_winter_frost_outcome").next())
            .expect("R7G snow/frost insulation helper body must be present");
        let frost_body = impl_body
            .split("    fn frost_day_context(")
            .nth(1)
            .and_then(|tail| tail.split("\n    fn active_frost_forcing").next())
            .expect("R7G typed frost day context body must be present");

        assert!(
            typed_partition_body.contains("thermal: Self::typed_winter_frost_thermal_inputs(context)?"),
            "frost compute inputs must route thermal state through the typed prior-snow helper"
        );
        assert!(
            typed_thermal_body.contains("Self::snow_frost_insulation_depth_density(context)?"),
            "frost thermal inputs must consume the snow/frost insulation helper"
        );
        assert!(
            insulation_body.contains("context.snow_lane_state.runtime_depth_m"),
            "frost forcing must see prior snow depth; legacy winter.for calls frostN before snowd"
        );
        assert!(
            insulation_body.contains("context.snow_lane_state.runtime_density_kg_m3"),
            "frost forcing must see prior snow density from the winter-column snow state"
        );
        assert!(
            !frost_body.contains("snow_liquid.runtime_depth_after_m")
                && !frost_body.contains("snow_liquid.runtime_density_after_kg_m3"),
            "same-day direct snow projection must not insulate the same day's frost solve"
        );
    }

    #[test]
    fn r7h_direct_production_winter_hourly_forcing_uses_shared_context_geometry() {
        let source = direct_publication_day_input_and_helpers_source();
        let builder_struct = source
            .split("struct DirectProductionDayInputBuilder")
            .nth(1)
            .and_then(|tail| tail.split("\n}").next())
            .expect("R7H production builder struct must be present");
        assert!(
            builder_struct.contains("winter_hourly_geometry: DirectProductionWinterHourlyGeometry"),
            "production direct must carry shared climate-context winter hourly geometry"
        );

        let impl_body = source
            .split("impl DirectProductionSnowFrostAuthority")
            .nth(1)
            .expect("R7H snow/frost authority impl must be present");
        for helper_name in ["frost_hourly_forcing", "snow_liquid_partition"] {
            let helper_body = impl_body
                .split(&format!("    fn {helper_name}("))
                .nth(1)
                .and_then(|tail| tail.split("\n    fn ").next())
                .expect("R7H winter hourly helper body must be present");
            assert!(
                helper_body.contains("avg_slope: winter_hourly_geometry.avg_slope")
                    && helper_body.contains("azimuth: winter_hourly_geometry.azimuth"),
                "{helper_name} must source winter hourly radiation geometry from shared climate context"
            );
            assert!(
                !helper_body.contains("avg_slope: self.avg_slope")
                    && !helper_body.contains("azimuth: self.azimuth"),
                "{helper_name} must not use lane-local slope/aspect for winter hourly radiation"
            );
        }
    }

    #[test]
    fn snowdensity1035b_direct_snow_consumer_receives_phase_selector() {
        let source = direct_publication_day_input_and_helpers_source();
        let authority_struct = source
            .split("struct DirectProductionSnowFrostAuthority")
            .nth(1)
            .and_then(|tail| tail.split("\n}").next())
            .expect("direct snow/frost authority struct must be present");
        assert!(
            authority_struct.contains("snow_phase_model"),
            "direct snow/frost authority must carry the selected snow phase model"
        );

        let impl_body = source
            .split("impl DirectProductionSnowFrostAuthority")
            .nth(1)
            .expect("direct snow/frost authority impl must be present");
        let snow_body = impl_body
            .split("    fn snow_liquid_partition(")
            .nth(1)
            .and_then(|tail| {
                tail.split(
                    "\n        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed",
                )
                .next()
            })
            .expect("snow liquid partition helper body must be present");
        assert!(
            snow_body.contains("snow_phase_model: self.snow_phase_model"),
            "real direct snow consumer forcing must receive the selected opt-in phase model"
        );

        let frost_body = impl_body
            .split("    fn frost_hourly_forcing(")
            .nth(1)
            .and_then(|tail| tail.split("\n    fn typed_winter_frost_compute_inputs").next())
            .expect("frost hourly forcing helper body must be present");
        assert!(
            frost_body.contains(
                "snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel::LegacyRst",
            ),
            "frost hourly forcing must remain on legacy phase partition in 10.3.5b"
        );
    }

    #[test]
    fn r7g_direct_production_reads_winter_column_snow_not_runtime_carry() {
        let source = direct_publication_day_input_and_helpers_source();
        let helper = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs";

        for forbidden in [
            "lane.snow_runtime_carry",
            "current_snow_runtime_carry",
            "initial_snow_runtime_carry",
            "snow_runtime_carry.map_or",
        ] {
            assert!(
                !source.contains(forbidden),
                "{helper} must not use stale DirectSnowRuntimeCarry authority: {forbidden}"
            );
        }

        assert!(
            source.contains("lane.winter_column.snow"),
            "{helper} must read prior direct snowpack from DirectWinterColumnState"
        );
        assert!(
            source.contains("snow_state_projected: authority.snow_frost.snow_state_projected(&snow_lane_state)"),
            "{helper} must derive projection status from controls plus winter-column snow state"
        );
    }

    #[test]
    fn r7g_direct_production_reads_winter_column_frost_and_deletes_bridge() {
        let source = direct_publication_day_input_and_helpers_source();
        let builder_source = direct_publication_day_input_and_helpers_source();
        let helper = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs";

        assert!(
            !builder_source.contains("lane.frost_runtime_carry"),
            "{helper} must not read DirectFrostRuntimeCarry as prior frost authority"
        );
        assert!(
            builder_source.contains("lane.winter_column.frost"),
            "{helper} must read prior direct frost state from DirectWinterColumnState"
        );

        let frost_body = builder_source
            .split("    fn frost_day_context(")
            .nth(1)
            .and_then(|tail| tail.split("\n    fn active_frost_forcing").next())
            .expect("R7G typed frost day context body must be present");
        assert!(
            frost_body.contains("typed_winter_frost_compute_inputs")
                && frost_body.contains("DirectWinterFrostComputeInputs"),
            "{helper} must compute production frost through typed active-frost inputs"
        );
        for forbidden in [
            "DirectFrostRunoffSurface",
            "DirectFrostLiquidPartition",
            "frost_runoff_surface",
            "frost_liquid_partition",
            "compute_frost_liquid_partition",
            "HillslopeKernelRequest",
        ] {
            assert!(
                !source.contains(forbidden),
                "{helper} production frost path must delete compatibility bridge symbol: {forbidden}"
            );
        }
        assert!(
            source.contains("compute_direct_winter_frost_partition"),
            "{helper} must use the typed winter frost kernel outcome name"
        );
    }

    #[test]
    fn laned_shadow_consumes_live_dynamic_friction_operands() {
        let runner_source = include_str!("../05_runner_execution_and_outputs.rs");
        let shadow_source = include_str!("../laned_shadow.rs");
        let builder_source = direct_publication_day_input_and_helpers_source();

        assert!(
            runner_source
                .contains("run_publication_stream_with_interleaved_day_inputs_and_day_frames"),
            "Lane D shadow must use the frame-aware stream so dynamic operands come from the executed day frame"
        );
        assert!(
            runner_source.contains("laned_shadow_lane_day_operands(day_frame)")
                && runner_source.contains("collector")
                && runner_source.contains("observe_row(row, operands)"),
            "Lane D shadow consumer must pass live day-frame operands into the collector"
        );
        assert!(
            builder_source.contains("day_frame.wb14_hourly_rainfall_m")
                && builder_source.contains(
                    "day_frame.evapotranspiration_compute_inputs.leaf_area_index",
                )
                && builder_source.contains("authority.evapotranspiration.canopy_height_m"),
            "Lane D shadow operand builder must source I, LAI, and h_c from live frame/typed authority"
        );
        assert!(
            builder_source.contains("leaf_area_index > 0.0 && canopy_height_m <= 0.0"),
            "Lane D shadow must fail closed when vegetation is active but canhgt is missing/non-positive"
        );
        assert!(
            shadow_source.contains(
                "seam_source_rates_from_hourly_depths(&operands.hourly_rainfall_m)",
            ),
            "Lane D shadow must convert WB14 hourly rainfall depth to rate for skin intensity"
        );
        assert!(
            shadow_source.contains("cell.leaf_area_index = operands.leaf_area_index")
                && shadow_source.contains("cell.canopy_height_m = operands.canopy_height_m"),
            "Lane D shadow must write dynamic vegetation operands into CellParameters"
        );
        assert!(
            !shadow_source.contains("let intensity = |_ofe: usize, _t: f64| 0.0"),
            "Lane D shadow must not retain the missing-source all-lane I=0 placeholder"
        );
    }

    fn r7g_snow_forcing(tmax_c: f64, tmin_c: f64) -> HillslopeDirectClimateDayForcing {
        HillslopeDirectClimateDayForcing {
            prcp_m: 0.0,
            tmax_c,
            tmin_c,
            rad_ly: 0.0,
            vwind_m_s: 0.0,
            wind_deg: 0.0,
            tdpt_c: 0.0,
            timem_s: Vec::new(),
            intsty_m_s: Vec::new(),
        }
    }

    fn direct_publication_day_input_and_helpers_source() -> String {
        [
            include_str!("../direct_publication/day_input_and_helpers/00_builders_and_authority.rs"),
            include_str!(
                "../direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs",
            ),
            include_str!(
                "../direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs",
            ),
            include_str!(
                "../direct_publication/day_input_and_helpers/00d_authority_runtime_impl.rs",
            ),
            include_str!("../direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs"),
            include_str!(
                "../direct_publication/day_input_and_helpers/02_publication_and_manifest_helpers.rs",
            ),
            include_str!(
                "../direct_publication/day_input_and_helpers/00b_ksatadj_authority_impl.rs",
            ),
        ]
        .join("\n")
    }
