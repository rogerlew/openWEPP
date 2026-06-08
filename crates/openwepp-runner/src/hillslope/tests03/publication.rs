use super::*;

    #[test]
    fn hphys0216_wb13_fc_storage_guard_rejects_missing_layer_authority_symbol() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("thetfc_0001"));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("missing thetfc_0001 must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("missing required runtime symbol thetfc_0001"),
                    "expected missing thetfc_0001 typed guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0216d_wb13_fc_storage_guard_rejects_missing_tail_symbol() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("wb13_profile_fc_tail_mm"));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("missing wb13_profile_fc_tail_mm must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("missing required runtime symbol wb13_profile_fc_tail_mm"),
                    "expected missing wb13_profile_fc_tail_mm typed guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0207_wb13_wp_storage_guard_is_exercised_by_direct_row_builder_probe() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_wp_store_mm"),
            BoundaryValue::scalar(-1.0),
        );

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("negative wb13_profile_wp_store_mm must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("wb13_profile_wp_store_mm must be >= 0.0"),
                    "expected wb13_profile_wp_store_mm typed guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0209_wb13_wp_storage_guard_rejects_missing_authoritative_symbol() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("wb13_profile_wp_store_mm"));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("missing wb13_profile_wp_store_mm must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("missing required runtime symbol wb13_profile_wp_store_mm"),
                    "expected missing wb13_profile_wp_store_mm guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0216d_wb13_profile_fc_publication_uses_layer_plus_tail_authority() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_fc_store_mm"),
            BoundaryValue::scalar(100.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_fc_tail_mm"),
            BoundaryValue::scalar(5.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_wp_store_mm"),
            BoundaryValue::scalar(55.0),
        );

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("valid WB13 probe surface should publish row");

        assert!(
            (row.wb13_row.profile_fc_store - 80.0).abs() < 1.0e-12,
            "ProfileFCStore must follow authoritative layer aggregation plus explicit normalized-tail contribution"
        );
        assert!(
            (row.wb13_row.profile_wp_store - 55.0).abs() < 1.0e-12,
            "ProfileWPStore must follow wb13_profile_wp_store_mm storage authority"
        );
    }
    #[test]
    fn hphys0203_wb13_dp_guard_rejects_negative_deep_percolation_source() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(-1.0e-6));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("negative D must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("D must be >= 0.0"),
                    "expected D domain guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0233_wb13_dp_publication_prefers_flux_surface_over_stale_state_surface() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.030_000));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.000_200));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 publication should use flux-authoritative D");

        assert!(
            (row.wb13_row.dp - 0.2).abs() < 1.0e-12,
            "Dp must follow flux-surface D when both state and flux values are present"
        );
    }
    #[test]
    fn hphys0234_wb13_subhyd_publication_prefers_flux_surface_over_stale_state_surface() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.030_000));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("Qdd"),
            BoundaryValue::scalar(0.020_000),
        );
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.050_000));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.000_700));
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("Qdd"),
            BoundaryValue::scalar(0.000_200),
        );
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.000_900));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 publication should use flux-authoritative q/Qdd/Qd");

        assert!(
            (row.wb13_row.latqcc - 0.7).abs() < 1.0e-12,
            "latqcc must follow flux-surface q when both state and flux values are present"
        );
        assert!(
            (row.wb13_row.tile - 0.2).abs() < 1.0e-12,
            "Tile must follow flux-surface Qdd when both state and flux values are present"
        );
    }
    #[test]
    fn hphys0239_wb13_hydrology_publication_prefers_flux_surface_over_stale_state_surface() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(0.050_000));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.003_000));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Es"), BoundaryValue::scalar(0.002_000));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Er"), BoundaryValue::scalar(0.001_000));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(0.000_800));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.000_300));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Es"), BoundaryValue::scalar(0.000_150));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Er"), BoundaryValue::scalar(0.000_070));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 publication should use flux-authoritative Q/Ep/Es/Er");

        assert!(
            (row.wb13_row.q - 0.8).abs() < 1.0e-12,
            "Q must follow flux-surface value when both state and flux are present"
        );
        assert!(
            (row.wb13_row.ep - 0.3).abs() < 1.0e-12,
            "Ep must follow flux-surface value when both state and flux are present"
        );
        assert!(
            (row.wb13_row.es - 0.15).abs() < 1.0e-12,
            "Es must follow flux-surface value when both state and flux are present"
        );
        assert!(
            (row.wb13_row.er - 0.07).abs() < 1.0e-12,
            "Er must follow flux-surface value when both state and flux are present"
        );
    }
    #[test]
    fn hphys0281_wb13_publication_canonicalizes_roundoff_negative_es_without_evappm_clamp() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Es"), BoundaryValue::scalar(-1.0e-13));
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("wb11_et_seed_branch_evappm"));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 publication should snap within-tolerance negative Es roundoff");

        assert!(
            row.wb13_row.es.abs() < f64::EPSILON,
            "WB13 Es roundoff must canonicalize to zero without EVAPPM material-negative clamp behavior"
        );
    }
    #[test]
    fn hphys0250_scheduler_lifecycle_preserves_pl_runtime_sentinel_for_ep_lineage() {
        let source = include_str!("../mod.rs");
        let sentinel = "pl_schedule_slot_count";
        let forbidden_fragment = ["symbol.as_str() != ", "\"", sentinel, "\""].concat();

        assert!(
            !source.contains(&forbidden_fragment),
            "runner scheduler lifecycle must not strip {sentinel}; PL growth must remain active so rtd can feed final Ep lineage"
        );
    }
    #[test]
    fn fq3dc_annual_preplant_skip_preserves_pl_sentinel_for_later_activation() {
        let mut runtime_surface = HillslopeWritebackSurface {
            state_surface: BTreeMap::new(),
            flux_surface: BTreeMap::new(),
        };
        for (symbol, value) in [
            ("pl_schedule_slot_count", 1.0),
            ("pl_schedule_rotation_years", 7.0),
            ("pl_schedule_rotation_repeats", 1.0),
            ("year", 1.0),
            ("day", 1.0),
            ("pl_schedule_slot_0001_ofe_index", 1.0),
            ("pl_schedule_slot_0001_year_in_rotation", 1.0),
            ("pl_schedule_slot_0001_rotation_index", 1.0),
            ("pl_schedule_slot_0001_crop_slots", 1.0),
            ("pl_schedule_slot_0001_crop_0001_imngmt", 1.0),
            ("pl_growth_slot_0001_crop_0001_jdplt", 130.0),
            ("pl_growth_slot_0001_crop_0001_jdharv", 288.0),
        ] {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }

        let sentinel_value = pl_runtime_activation_sentinel_value(&runtime_surface);
        prepare_pl_runtime_activation_for_scheduler(&mut runtime_surface)
            .expect("pre-plant annual day should be a day-local scheduler skip");
        assert!(
            !runtime_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("pl_schedule_slot_count")),
            "pre-plant day should suppress PL phases for that day"
        );

        restore_pl_runtime_activation_sentinel_for_next_day(&mut runtime_surface, sentinel_value);
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("day"), BoundaryValue::scalar(153.0));

        prepare_pl_runtime_activation_for_scheduler(&mut runtime_surface)
            .expect("post-plant annual day should re-evaluate the carried PL schedule");
        assert!(
            runtime_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("pl_schedule_slot_count")),
            "carried annual schedule sentinel must be available after jdplt so Corn growth can engage ET"
        );
    }
    #[test]
    fn fq3dc_scheduler_calendar_day_symbol_uses_julian_day_for_pl_activation() {
        let mut runtime_surface = HillslopeWritebackSurface {
            state_surface: BTreeMap::new(),
            flux_surface: BTreeMap::new(),
        };
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("day"), BoundaryValue::scalar(2.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("year"), BoundaryValue::scalar(1990.0));
        let calendar_day = ClimateDayProjection {
            year: 1990,
            month: 6,
            day_of_month: 2,
            julian_day: 153,
            precipitation_mm: 0.0,
        };

        seed_scheduler_calendar_symbols(
            &mut runtime_surface,
            &SchedulerLifecycleContext {
                run_name: "calendar-probe",
                execution_lane: ExecutionLane::Hourly,
                publication_area_m2: 1.0,
                simulation_year: 1,
                sim_day_index: 153,
                calendar_day: &calendar_day,
                runtime_swe_before_m: 0.0,
                hphys0245_trace_config: None,
            },
        );

        let day = require_runtime_surface_scalar(&runtime_surface, "day")
            .expect("scheduler day symbol should exist");
        assert!(
            (day - 153.0).abs() < f64::EPSILON,
            "PL activation must consume Julian day, not day-of-month"
        );
        let year = require_runtime_surface_scalar(&runtime_surface, "year")
            .expect("scheduler year symbol should exist");
        assert!(
            (year - 1.0).abs() < f64::EPSILON,
            "PL activation must consume simulation year within the rotation"
        );
    }
    #[test]
    fn hphys0250_pl_activation_keeps_zero_date_perennial_slots_active() {
        let mut runtime_surface = HillslopeWritebackSurface {
            state_surface: BTreeMap::new(),
            flux_surface: BTreeMap::new(),
        };
        for (symbol, value) in [
            ("pl_schedule_slot_count", 1.0),
            ("pl_schedule_rotation_years", 4.0),
            ("pl_schedule_rotation_repeats", 1.0),
            ("year", 1.0),
            ("day", 1.0),
            ("pl_schedule_slot_0001_ofe_index", 1.0),
            ("pl_schedule_slot_0001_year_in_rotation", 1.0),
            ("pl_schedule_slot_0001_rotation_index", 1.0),
            ("pl_schedule_slot_0001_crop_slots", 1.0),
            ("pl_schedule_slot_0001_crop_0001_imngmt", 2.0),
            ("pl_growth_slot_0001_crop_0001_jdplt", 0.0),
            ("pl_growth_slot_0001_crop_0001_jdharv", 0.0),
            ("pl_growth_slot_0001_crop_0001_jdstop", 0.0),
        ] {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }

        prepare_pl_runtime_activation_for_scheduler(&mut runtime_surface)
            .expect("zero-date perennial PL slot should remain scheduler-active");

        assert!(
            runtime_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("pl_schedule_slot_count")),
            "zero-date perennial windows must keep PL activation sentinel for scheduler dispatch"
        );
    }
    #[test]
    fn hphys0250_wb13_ep_publication_consumes_final_root_uptake_flux() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.0));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.004_2));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 publication should consume final root-uptake flux Ep");

        assert!(
            (row.wb13_row.ep - 4.2).abs() < 1.0e-12,
            "WB13 Ep must use final post-root-uptake flux even when stale state Ep is present"
        );
    }
    #[test]
    fn hphys0289_wb13_rm_publication_consumes_routed_wmelt_not_raw_prcp_swe_delta() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.010));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.040),
        );
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.routed_melt_m"),
            BoundaryValue::scalar(0.002),
        );
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("S"), BoundaryValue::scalar(0.002));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Irr"), BoundaryValue::scalar(0.001));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.040,
        )
        .expect("valid WB13 probe surface should publish row");

        assert!(
            (row.wb13_row.rm - 3.0).abs() < 1.0e-12,
            "snow-active WB13 RM must equal routed wmelt + irrigation when winter cleared rain"
        );
    }
    #[test]
    fn hphys0289_wb13_rm_publication_requires_routed_wmelt_surface() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("snow.routed_melt_m"));
        runtime_surface
            .flux_surface
            .remove(&BoundarySymbol::from("snow.routed_melt_m"));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("missing routed wmelt must fail WB13 publication guard");

        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("snow.routed_melt_m"),
                    "expected missing routed wmelt guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0289_wb13_rm_publication_preserves_warm_rain_without_snow_partition() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.010));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.routed_melt_m"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.post_winter_rain_m"),
            BoundaryValue::scalar(0.010),
        );
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Irr"), BoundaryValue::scalar(0.001));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("warm rain without snow partition should publish row");

        assert!(
            (row.wb13_row.rm - 11.0).abs() < 1.0e-12,
            "snow-inactive WB13 RM must preserve post-winter rain plus irrigation"
        );
    }
    #[test]
    fn hphys0289_wb13_rm_publication_prefers_flux_routed_wmelt_over_stale_state() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.010));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.030),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.routed_melt_m"),
            BoundaryValue::scalar(0.020),
        );
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.routed_melt_m"),
            BoundaryValue::scalar(0.003),
        );
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Irr"), BoundaryValue::scalar(0.001));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.040,
        )
        .expect("valid WB13 probe surface should publish row");

        assert!(
            (row.wb13_row.rm - 4.0).abs() < 1.0e-12,
            "WB13 RM must prefer routed wmelt from flux surface over stale state surface"
        );
    }
    #[test]
    fn hphys0289_wb13_rm_publication_rejects_negative_routed_wmelt() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.routed_melt_m"),
            BoundaryValue::scalar(-1.0e-6),
        );

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("negative routed wmelt must fail WB13 publication guard");

        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("snow.routed_melt_m must be >= 0.0"),
                    "expected negative routed wmelt guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0291_wb13_rm_publication_rejects_state_only_routed_melt() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .flux_surface
            .remove(&BoundarySymbol::from("snow.routed_melt_m"));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.routed_melt_m"),
            BoundaryValue::scalar(0.010),
        );

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("state-only routed melt must fail WB13 publication guard");

        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("missing required runtime flux symbol snow.routed_melt_m"),
                    "expected missing producer flux guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0290_wb13_rm_publication_consumes_explicit_post_winter_rain() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.010));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.040),
        );
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.routed_melt_m"),
            BoundaryValue::scalar(0.002),
        );
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.post_winter_rain_m"),
            BoundaryValue::scalar(0.000_382_5),
        );
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Irr"), BoundaryValue::scalar(0.001));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.040,
        )
        .expect("valid WB13 probe surface should publish row");

        assert!(
            (row.wb13_row.rm - 3.382_5).abs() < 1.0e-12,
            "WB13 RM must equal explicit post-winter rain + routed wmelt + irrigation"
        );
    }
    #[test]
    fn hphys0290_wb13_rm_publication_requires_post_winter_rain_surface() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("snow.post_winter_rain_m"));
        runtime_surface
            .flux_surface
            .remove(&BoundarySymbol::from("snow.post_winter_rain_m"));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("missing post-winter rain must fail WB13 publication guard");

        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("snow.post_winter_rain_m"),
                    "expected missing post-winter rain guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0290_wb13_rm_publication_prefers_flux_post_winter_rain_over_stale_state() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.post_winter_rain_m"),
            BoundaryValue::scalar(0.010),
        );
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.post_winter_rain_m"),
            BoundaryValue::scalar(0.000_5),
        );
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.routed_melt_m"),
            BoundaryValue::scalar(0.002),
        );

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("valid WB13 probe surface should publish row");

        assert!(
            (row.wb13_row.rm - 2.5).abs() < 1.0e-12,
            "WB13 RM must prefer post-winter rain from flux surface over stale state surface"
        );
    }
    #[test]
    fn hphys0290_wb13_rm_publication_rejects_state_only_post_winter_rain() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .flux_surface
            .remove(&BoundarySymbol::from("snow.post_winter_rain_m"));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.post_winter_rain_m"),
            BoundaryValue::scalar(0.010),
        );

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("state-only post-winter rain must fail WB13 publication guard");

        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("missing required runtime flux symbol snow.post_winter_rain_m"),
                    "expected missing producer flux guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0290_wb13_rm_publication_rejects_negative_post_winter_rain() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.post_winter_rain_m"),
            BoundaryValue::scalar(-1.0e-6),
        );

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("negative post-winter rain must fail WB13 publication guard");

        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("snow.post_winter_rain_m must be >= 0.0"),
                    "expected negative post-winter rain guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0290_wb13_rm_publication_rejects_non_finite_post_winter_rain() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.post_winter_rain_m"),
            BoundaryValue::scalar(f64::NAN),
        );

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("non-finite post-winter rain must fail WB13 publication guard");

        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("runtime flux symbol snow.post_winter_rain_m must be finite"),
                    "expected non-finite post-winter rain guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0203_wb13_latqcc_guard_rejects_negative_lateral_source() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(-1.0e-6));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("negative q must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("q must be >= 0.0"),
                    "expected q domain guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0212_wb13_subhyd_coupling_guard_rejects_qd_mismatch() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.002));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qdd"), BoundaryValue::scalar(0.001));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.002_5));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("Qd mismatch must fail WB13 subsurface coupling guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("Qd coupling closure violated"),
                    "expected Qd coupling guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0212_wb13_subhyd_publication_uses_qdd_and_subrin_lineage() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.0015));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qdd"), BoundaryValue::scalar(0.0005));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.0020));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("SubRIn"),
            BoundaryValue::scalar(0.0008),
        );

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("valid Qd coupling surface should publish WB13 row");

        assert!(
            (row.wb13_row.latqcc - 1.5).abs() < 1.0e-12,
            "latqcc must follow q source symbol in mm/day lane"
        );
        assert!(
            (row.wb13_row.tile - 0.5).abs() < 1.0e-12,
            "Tile must follow Qdd source symbol in mm/day lane"
        );
        assert!(
            (row.wb13_row.subrin - 0.8).abs() < 1.0e-12,
            "SubRIn must follow SubRIn source symbol in mm/day lane"
        );
    }
    #[test]
    fn hphys0203_wb13_soil_water_total_closure_is_conservation_consistent() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.081),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_ws_frz"),
            BoundaryValue::scalar(0.003),
        );

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("valid WB13 probe surface should publish row");

        let closure_delta =
            row.wb13_row.soil_water_total - (row.wb13_row.total_soil + row.wb13_row.frozwt);
        assert!(
            closure_delta.abs() <= SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
            "SoilWaterTotal closure must remain conservation-consistent, observed delta={closure_delta}"
        );
    }
    #[test]
    fn hphys0203_wb13_profile_storage_perturbation_is_stable() {
        let baseline_surface = seeded_wb13_runtime_surface_probe();
        let baseline_row = build_simulation_owned_wb13_row(
            &baseline_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("baseline probe row should publish");

        let mut perturbed_surface = seeded_wb13_runtime_surface_probe();
        let baseline_thetfc = require_runtime_surface_scalar(&perturbed_surface, "thetfc_0001")
            .expect("seeded surface should include thetfc_0001");
        perturbed_surface.state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(baseline_thetfc + 1.0e-4),
        );
        let perturbed_row = build_simulation_owned_wb13_row(
            &perturbed_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("perturbed probe row should publish");

        assert!(
            perturbed_row.wb13_row.profile_porosity_cap >= perturbed_row.wb13_row.profile_fc_store
                && perturbed_row.wb13_row.profile_fc_store
                    >= perturbed_row.wb13_row.profile_wp_store,
            "bounded profile perturbation must preserve profile storage ordering"
        );
        assert!(
            perturbed_row.wb13_row.profile_fc_store >= baseline_row.wb13_row.profile_fc_store,
            "positive bounded FC perturbation should not decrease published ProfileFCStore"
        );
        assert!(
            (perturbed_row.wb13_row.profile_fc_store - baseline_row.wb13_row.profile_fc_store)
                <= 5.0,
            "bounded FC perturbation produced unstable ProfileFCStore response: baseline={}, perturbed={}",
            baseline_row.wb13_row.profile_fc_store,
            perturbed_row.wb13_row.profile_fc_store
        );
    }
    #[test]
    fn wshedimpl42_breakpoint_seed_uses_current_nbrkpt_not_stale_ninten() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("ibrkpt", 1.0),
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", -3.0),
            ("tmin", -6.9),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("nbrkpt", 3.0),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 36_479.88),
            ("timem_0003", 38_279.88),
            ("intsty_0001", 5.701_773_141_797_617e-8),
            ("intsty_0002", 5.111_111_111_111_11e-7),
            ("intsty_0003", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.55, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("seeding should accept current-day breakpoint cardinality");

        let ninten = require_runtime_surface_scalar(&runtime_surface, "ninten")
            .expect("ninten should be seeded");
        let nbrkpt = require_runtime_surface_scalar(&runtime_surface, "nbrkpt")
            .expect("nbrkpt should be seeded");
        let rainfall_input =
            require_runtime_surface_scalar(&runtime_surface, "wb12_rainfall_input")
                .expect("wb12_rainfall_input should be seeded");

        assert!(
            (ninten - 3.0).abs() < 1.0e-12,
            "ninten should track current-day breakpoint count"
        );
        assert!(
            (nbrkpt - 3.0).abs() < 1.0e-12,
            "nbrkpt should remain aligned with current-day breakpoint count"
        );
        assert!(
            (rainfall_input - 0.003).abs() < 1.0e-12,
            "rainfall seed should preserve full current-day breakpoint precipitation depth"
        );
    }
    #[test]
    fn hphys0250_wb11_seed_initializes_neutral_water_stress_for_decomposition() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("WB11 seed should publish neutral initial water stress");

        let water_stress = require_runtime_surface_scalar(&runtime_surface, "Ws")
            .expect("WB11 seed should publish Ws for pre-ET decomposition consumers");
        assert!(
            (water_stress - 1.0).abs() < 1.0e-12,
            "initial decomposition stress carryover must be neutral before ET computes same-day Ws"
        );
    }
    #[test]
    fn hphys0232_wb11_seed_daily_lane_sets_wb18_perc_lane_substeps_to_one() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("daily WB11 seed should succeed");

        let lane_substeps =
            require_runtime_surface_scalar(&runtime_surface, "wb18_perc_lane_substeps")
                .expect("daily WB11 seed should publish wb18_perc_lane_substeps");
        let wb19_lane_substeps =
            require_runtime_surface_scalar(&runtime_surface, "wb19_lateral_drain_lane_substeps")
                .expect("daily WB11 seed should publish wb19_lateral_drain_lane_substeps");
        assert!(
            (lane_substeps - 1.0).abs() < 1.0e-12,
            "daily lane must seed wb18_perc_lane_substeps=1"
        );
        assert!(
            (wb19_lane_substeps - 1.0).abs() < 1.0e-12,
            "daily lane must seed wb19_lateral_drain_lane_substeps=1"
        );
    }
    #[test]
    fn hphys0232_wb11_seed_hourly_lane_sets_wb18_perc_lane_substeps_to_twenty_four() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Hourly)
            .expect("hourly WB11 seed should succeed");

        let lane_substeps =
            require_runtime_surface_scalar(&runtime_surface, "wb18_perc_lane_substeps")
                .expect("hourly WB11 seed should publish wb18_perc_lane_substeps");
        let wb19_lane_substeps =
            require_runtime_surface_scalar(&runtime_surface, "wb19_lateral_drain_lane_substeps")
                .expect("hourly WB11 seed should publish wb19_lateral_drain_lane_substeps");
        assert!(
            (lane_substeps - 24.0).abs() < 1.0e-12,
            "hourly lane must seed wb18_perc_lane_substeps=24"
        );
        assert!(
            (wb19_lane_substeps - 24.0).abs() < 1.0e-12,
            "hourly lane must seed wb19_lateral_drain_lane_substeps=24"
        );
    }
    #[test]
    fn hphys0208_wb11_seed_uses_sat_por_cpm_layer_lineage() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("WB11 seeding should succeed for valid sat/por/cpm lineage");

        let theta = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_theta_0001")
            .expect("wb18_perc_theta_0001 should be seeded");
        let fc = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_fc_0001")
            .expect("wb18_perc_fc_0001 should be seeded");
        let ul = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_ul_0001")
            .expect("wb18_perc_ul_0001 should be seeded");
        let wb11_soil_water = require_runtime_surface_scalar(&runtime_surface, "wb11_soil_water")
            .expect("wb11_soil_water should be seeded");
        let wb11_drainable_storage =
            require_runtime_surface_scalar(&runtime_surface, "wb11_drainable_storage")
                .expect("wb11_drainable_storage should be seeded");

        let expected_fc = (0.30 - 0.12) * 0.25;
        let expected_ul = (0.45 - 0.12) * 0.25;
        let expected_theta = (((0.50 * 0.45) * 0.90) - 0.12) * 0.25;
        let expected_soilw = expected_theta + (0.12 * 0.25);

        assert!(
            (fc - expected_fc).abs() < 1.0e-12,
            "wb18_perc_fc must follow dg*(thetfc-thetdr)"
        );
        assert!(
            (ul - expected_ul).abs() < 1.0e-12,
            "wb18_perc_ul must follow (por-thetdr)*dg"
        );
        assert!(
            (theta - expected_theta).abs() < 1.0e-12,
            "wb18_perc_theta must follow (((sat*por)*cpm)-thetdr)*dg"
        );
        assert!(
            (wb11_soil_water - expected_soilw).abs() < 1.0e-12,
            "wb11_soil_water must follow Σ(st + thetdr*dg)"
        );
        assert!(
            wb11_drainable_storage.abs() < 1.0e-12,
            "wb11_drainable_storage must follow Σmax(st-fc,0)"
        );
    }
    #[test]
    fn auth12_wb11_seed_applies_cpm_for_disturbed_measured_fcwp_lineage() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("solwpv", 9002.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("WB11 seeding should succeed for disturbed measured FC/WP lineage");

        let theta = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_theta_0001")
            .expect("wb18_perc_theta_0001 should be seeded");
        let wb11_soil_water = require_runtime_surface_scalar(&runtime_surface, "wb11_soil_water")
            .expect("wb11_soil_water should be seeded");

        let expected_theta_without_cpm = ((0.50 * 0.45) - 0.12) * 0.25;
        let expected_theta_with_cpm = (((0.50 * 0.45) * 0.90) - 0.12) * 0.25;
        let expected_soilw = expected_theta_with_cpm + (0.12 * 0.25);

        assert!(
            (theta - expected_theta_with_cpm).abs() < 1.0e-12,
            "disturbed measured FC/WP lineage must apply sat*por*cpm scaling"
        );
        assert!(
            theta < expected_theta_without_cpm - 1.0e-12,
            "disturbed measured FC/WP lineage must not bypass cpm scaling"
        );
        assert!(
            (wb11_soil_water - expected_soilw).abs() < 1.0e-12,
            "wb11_soil_water must remain consistent with the disturbed measured FC/WP cpm-scaled saturation lineage"
        );
    }
    #[test]
    fn auth12_wb11_seed_applies_cpm_for_legacy_measured_theta_fcwp_lineage() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("solwpv", 7778.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("WB11 seeding should succeed for legacy measured-theta FC/WP lineage");

        let theta = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_theta_0001")
            .expect("wb18_perc_theta_0001 should be seeded");
        let wb11_soil_water = require_runtime_surface_scalar(&runtime_surface, "wb11_soil_water")
            .expect("wb11_soil_water should be seeded");

        let expected_theta_without_cpm = ((0.50 * 0.45) - 0.12) * 0.25;
        let expected_theta_with_cpm = (((0.50 * 0.45) * 0.90) - 0.12) * 0.25;
        let expected_soilw = expected_theta_with_cpm + (0.12 * 0.25);

        assert!(
            (theta - expected_theta_with_cpm).abs() < 1.0e-12,
            "legacy measured-theta FC/WP lineage must apply sat*por*cpm scaling"
        );
        assert!(
            theta < expected_theta_without_cpm - 1.0e-12,
            "legacy measured-theta FC/WP lineage must not bypass cpm scaling"
        );
        assert!(
            (wb11_soil_water - expected_soilw).abs() < 1.0e-12,
            "wb11_soil_water must remain consistent with the measured-theta cpm-scaled saturation lineage"
        );
    }
    #[test]
    fn hphys0212_wb11_seed_preserves_mutable_state_after_initialization() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("initial WB11 seed should succeed");

        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.012_345),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.100_123),
        );
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.001));

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("daily reseed should not reinitialize WB18/WB11 mutable state");

        let theta = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_theta_0001")
            .expect("wb18_perc_theta_0001 should remain available");
        let storage_initial =
            require_runtime_surface_scalar(&runtime_surface, "wb12_storage_initial")
                .expect("wb12_storage_initial should be refreshed each day");

        assert!(
            (theta - 0.012_345).abs() < 1.0e-12,
            "daily reseed must preserve mutable wb18_perc_theta state"
        );
        assert!(
            (storage_initial - 0.100_123).abs() < 1.0e-12,
            "wb12_storage_initial must follow carried wb11_soil_water each day"
        );
    }
    #[test]
    fn hphys0212_wb11_seed_rejects_enabled_drain_without_geometry() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_drain_enabled"),
            BoundaryValue::scalar(1.0),
        );

        let error = seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect_err("enabled drain without geometry symbols must fail WB11 seed");
        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { detail, .. } => {
                assert!(
                    detail.contains("missing required runtime symbol wb19_drain_depth"),
                    "expected missing wb19_drain_depth guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hphys0263_wb11_seed_uses_evappm_branch_when_pmetpara_selects_pmet() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 20.0),
            ("tmin", 10.0),
            ("tdpt", 8.0),
            ("rad", 20.0),
            ("radpot", 25.0),
            ("vwind", 2.0),
            ("elevm", 300.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 4.0),
            ("canhgt", 1.0),
            ("rtd", 0.2),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("pmetpara.mode.sidecar_present", 1.0),
            ("pmetpara.mode.iflget", 2.0),
            ("pmetpara.selected.kcb", 0.95),
            ("pmetpara.selected.rawp", 0.8),
            ("wb17_residue_interception", 0.000_2),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.80, true);
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_solthk_0001"),
            BoundaryValue::scalar(0.25),
        );

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("PMET-mode WB11 seed should succeed");

        let demand = require_runtime_surface_scalar(&runtime_surface, "wb11_et_demand")
            .expect("WB11 demand should be seeded");
        let evappm_branch =
            require_runtime_surface_scalar(&runtime_surface, "wb11_et_seed_branch_evappm")
                .expect("EVAPPM branch flag should be published");
        let priestley_branch = require_runtime_surface_scalar(
            &runtime_surface,
            "wb11_et_seed_branch_priestley_taylor",
        )
        .expect("Priestley branch flag should be published");
        let etorc = require_runtime_surface_scalar(&runtime_surface, "pmet.etorc_mm")
            .expect("migrated EVAPPM reference ET should be traced");
        let kcbcon = require_runtime_surface_scalar(&runtime_surface, "pmet.kcbcon")
            .expect("migrated EVAPPM basal canopy coefficient should be traced");

        assert!(
            (demand - 0.000_108_279_281_560_428_06).abs() < 1.0e-15,
            "WB11 demand must follow pinned evappm.for plant-transpiration demand"
        );
        assert!((evappm_branch - 1.0).abs() < 1.0e-12);
        assert!(priestley_branch.abs() < 1.0e-12);
        assert!((etorc - 0.139_042_184_372_870_16).abs() < 1.0e-12);
        assert!((kcbcon - 0.778_751_298_023_734_6).abs() < 1.0e-12);
    }
    #[test]
    fn hphys0281_wb11_evappm_seed_publishes_condensation_storage_return() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", -1.6),
            ("tmin", -14.6),
            ("tdpt", -1.0),
            ("rad", 200.0),
            ("radpot", 250.0),
            ("vwind", 3.0),
            ("elevm", 300.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 4.0),
            ("canhgt", 1.0),
            ("rtd", 0.2),
            ("prcp", 0.004_4),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("pmetpara.mode.sidecar_present", 1.0),
            ("pmetpara.mode.iflget", 2.0),
            ("pmetpara.selected.kcb", 0.95),
            ("pmetpara.selected.rawp", 0.8),
            ("wb17_residue_interception", 0.000_2),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.80, true);
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_solthk_0001"),
            BoundaryValue::scalar(0.25),
        );

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("supersaturated cold-day EVAPPM seed should not fail");

        let pmet_soil_evaporation = require_runtime_surface_scalar(&runtime_surface, "pmet.es_m")
            .expect("PMET soil evaporation should be published");
        let storage_return =
            require_runtime_surface_scalar(&runtime_surface, "pmet.es_storage_return_m")
                .expect("negative EVAPPM soil evaporation should publish a storage return");
        let storage_return_value = runtime_surface
            .state_surface
            .get(&BoundarySymbol::from("pmet.es_storage_return_m"))
            .expect("storage return boundary value should be present");
        let pmet_transpiration = require_runtime_surface_scalar(&runtime_surface, "pmet.ep_m")
            .expect("PMET transpiration should be published");
        let demand = require_runtime_surface_scalar(&runtime_surface, "wb11_et_demand")
            .expect("WB11 ET demand should be published");
        let etorc = require_runtime_surface_scalar(&runtime_surface, "pmet.etorc_mm")
            .expect("PMET reference ET diagnostic should be published");

        assert!(
            etorc < 0.0,
            "test vector must exercise condensation/reference-ET reversal, observed {etorc}"
        );
        assert!(
            pmet_soil_evaporation.abs() < f64::EPSILON,
            "material-negative PMET Es must publish as non-negative zero, observed {pmet_soil_evaporation}"
        );
        assert!(
            storage_return > 0.0,
            "negative raw EVAPPM Es magnitude must be carried as top-layer storage return"
        );
        assert_eq!(
            storage_return_value.unit_label(),
            "m",
            "storage return must publish as typed water-depth meters"
        );
        assert!(
            pmet_transpiration.abs() < f64::EPSILON,
            "condensation must not publish material-negative PMET transpiration, observed {pmet_transpiration}"
        );
        assert!(
            demand.abs() < f64::EPSILON,
            "WB11 PMET demand must follow canonicalized non-negative transpiration, observed {demand}"
        );
    }
    #[test]
    fn hphys0213_wb19_lateral_withdrawal_publishes_realized_flux_and_updates_wb11_soil_water() {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
        state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("solwpv"),
            BoundaryValue::scalar(2006.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb11_drainable_storage"),
            BoundaryValue::scalar(0.4),
        );
        state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.5),
        );
        state_surface.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
        state_surface.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(10.0));
        state_surface.insert(
            BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
            BoundaryValue::scalar(1.0e6),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.6),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_fc_0001"),
            BoundaryValue::scalar(0.2),
        );
        state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(0.2),
        );
        state_surface.insert(
            BoundarySymbol::from("thetdr_0001"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_ul_0001"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_ssc_0001"),
            BoundaryValue::scalar(1.0e-5),
        );
        state_surface.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.8));
        state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
        state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("coca_0001"),
            BoundaryValue::scalar(1.0),
        );

        let mut flux_surface = BTreeMap::new();
        flux_surface.insert(BoundarySymbol::from("Pe"), BoundaryValue::scalar(0.0));

        let request = HillslopeKernelRequest::with_phase_context(
            "lateral_transfer",
            HillslopeKernelPhaseClass::HydrologyLateralTransfer,
            HillslopeConsumerAdapter::Watbal,
            None,
            &state_surface,
            &flux_surface,
        );

        let mut kernel = Wb11HydrologyKernel;
        let response = kernel.run_hillslope_phase(&request);
        assert_eq!(
            response.status.message_id(),
            "HKERNEL-WB11-LAT-OK-001",
            "lateral transfer must complete nominally for valid drainable pool inputs"
        );

        let q_lateral = flux_field_scalar(&response.writeback.flux_updates, "q")
            .expect("lateral transfer should publish q");
        let soil_water_after =
            state_field_scalar(&response.writeback.state_updates, "wb11_soil_water")
                .expect("lateral transfer should publish wb11_soil_water");
        let drainable_after =
            state_field_scalar(&response.writeback.state_updates, "wb11_drainable_storage")
                .expect("lateral transfer should publish wb11_drainable_storage");

        assert!(
            (q_lateral - 0.4).abs() < 1.0e-12,
            "published q must match realized top-down withdrawal capped by available pool"
        );
        assert!(
            (soil_water_after - 0.1).abs() < 1.0e-12,
            "wb11_soil_water must be reduced by realized q withdrawal"
        );
        assert!(
            drainable_after.abs() < 1.0e-12,
            "wb11_drainable_storage must close to zero after full realized withdrawal"
        );
    }
    #[test]
    #[allow(clippy::too_many_lines)]
    fn hphys0213_wb19_drainage_withdrawal_publishes_realized_qdd_and_qd() {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("solwpv"),
            BoundaryValue::scalar(2006.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb11_drainable_storage"),
            BoundaryValue::scalar(0.4),
        );
        state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.5),
        );
        state_surface.insert(
            BoundarySymbol::from("wb11_drainage_coefficient"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb19_drain_enabled"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb19_drain_depth"),
            BoundaryValue::scalar(0.8),
        );
        state_surface.insert(
            BoundarySymbol::from("wb19_drain_spacing"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb19_drain_diameter"),
            BoundaryValue::scalar(0.1),
        );
        state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.6),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_fc_0001"),
            BoundaryValue::scalar(0.2),
        );
        state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(0.2),
        );
        state_surface.insert(
            BoundarySymbol::from("thetdr_0001"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_ul_0001"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_ssc_0001"),
            BoundaryValue::scalar(0.01),
        );
        state_surface.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.8));
        state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
        state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("coca_0001"),
            BoundaryValue::scalar(1.0),
        );

        let mut flux_surface = BTreeMap::new();
        flux_surface.insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.02));

        let request = HillslopeKernelRequest::with_phase_context(
            "drainage",
            HillslopeKernelPhaseClass::HydrologyDrainage,
            HillslopeConsumerAdapter::Perc,
            None,
            &state_surface,
            &flux_surface,
        );

        let mut kernel = Wb11HydrologyKernel;
        let response = kernel.run_hillslope_phase(&request);
        assert_eq!(
            response.status.message_id(),
            "HKERNEL-WB11-DRAIN-OK-001",
            "drainage phase must complete nominally for valid drain geometry inputs"
        );

        let qdd = flux_field_scalar(&response.writeback.flux_updates, "Qdd")
            .expect("drainage phase should publish Qdd");
        let qd = flux_field_scalar(&response.writeback.flux_updates, "Qd")
            .expect("drainage phase should publish Qd");
        let soil_water_after =
            state_field_scalar(&response.writeback.state_updates, "wb11_soil_water")
                .expect("drainage phase should publish wb11_soil_water");
        let drainable_after =
            state_field_scalar(&response.writeback.state_updates, "wb11_drainable_storage")
                .expect("drainage phase should publish wb11_drainable_storage");

        assert!(
            (qdd - 0.4).abs() < 1.0e-12,
            "published Qdd must match realized tile withdrawal capped by available drainable pool"
        );
        assert!(
            (qd - 0.42).abs() < 1.0e-12,
            "published Qd must follow q + Qdd coupling with realized Qdd"
        );
        assert!(
            (soil_water_after - 0.1).abs() < 1.0e-12,
            "wb11_soil_water must be reduced by realized Qdd withdrawal"
        );
        assert!(
            drainable_after.abs() < 1.0e-12,
            "wb11_drainable_storage must close to zero after realized drainage withdrawal"
        );
    }
    #[test]
    fn hphys0213_wb12_storage_reconciliation_accepts_realized_wb19_subsurface_flux() {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(
            BoundarySymbol::from("wb12_storage_initial"),
            BoundaryValue::scalar(0.5),
        );
        state_surface.insert(
            BoundarySymbol::from("wb12_storage_closure_tolerance"),
            BoundaryValue::scalar(1.0e-9),
        );
        state_surface.insert(
            BoundarySymbol::from("wb12_precip_input"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb12_storage_observed"),
            BoundaryValue::scalar(0.03),
        );

        let mut flux_surface = BTreeMap::new();
        flux_surface.insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("S"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("I"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("ET"), BoundaryValue::scalar(0.05));
        flux_surface.insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.42));

        let request = HillslopeKernelRequest::with_phase_context(
            "storage_reconciliation",
            HillslopeKernelPhaseClass::HydrologyStorageReconciliation,
            HillslopeConsumerAdapter::Watbal,
            None,
            &state_surface,
            &flux_surface,
        );

        let mut kernel = Wb11HydrologyKernel;
        let response = kernel.run_hillslope_phase(&request);
        assert_eq!(
            response.status.message_id(),
            "HKERNEL-WB12-STORAGE-OK-001",
            "storage reconciliation must accept non-negative closure under realized WB19 subsurface losses"
        );

        let storage_reconciled =
            state_field_scalar(&response.writeback.state_updates, "wb12_storage_reconciled")
                .expect("storage reconciliation should publish wb12_storage_reconciled");
        let closure_delta = flux_field_scalar(
            &response.writeback.flux_updates,
            "wb12_storage_closure_delta",
        )
        .expect("storage reconciliation should publish wb12_storage_closure_delta");

        assert!(
            (storage_reconciled - 0.03).abs() < 1.0e-12,
            "storage reconciliation must preserve WB12 conservation under realized WB19 Qd"
        );
        assert!(
            closure_delta.abs() < 1.0e-12,
            "closure delta must remain within configured tolerance for realized WB19 outputs"
        );
    }
    #[test]
    fn hphys0208_wb11_seed_hard_fails_missing_cpm_symbol() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, false);

        let error = seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect_err("missing cpm_0001 must fail WB11 seed");
        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { detail, .. } => {
                assert!(
                    detail.contains("missing required runtime symbol cpm_0001"),
                    "expected missing cpm_0001 guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }
    #[test]
    fn hillstab08_wb16_producer_single_ofe_projects_expected_alpha_lineage() {
        let mut runtime_surface = HillslopeWritebackSurface::default();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("nelem"), BoundaryValue::scalar(1.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));
        insert_wb16_ofe_projection_symbols(
            &mut runtime_surface,
            1,
            Wb16OfeSeedVector {
                avgslp: 0.04,
                slplen: 30.0,
                inrcov: 0.45,
                rilcov: 0.30,
                rrinit: 0.02,
                rspace: 1.20,
                width: 0.40,
                rtyp: 2.0,
                cancov: 0.50,
                canhgt: 1.00,
                bb_seed: 0.10,
                bbb_seed: 0.20,
                flivmx_seed: 0.60,
                hmax_seed: 2.00,
            },
        );

        let produced = produce_wb16_ealpha_from_runtime_surface(&mut runtime_surface)
            .expect("single-OFE WB16 producer should execute")
            .expect("single-OFE WB16 producer should return ealpha");
        let projected_primary_alpha =
            require_runtime_surface_scalar(&runtime_surface, "ofe1_alpha")
                .expect("producer should publish OFE alpha");
        let projected_equivalent_alpha = require_runtime_surface_scalar(&runtime_surface, "ealpha")
            .expect("producer should publish equivalent-plane alpha");
        let projected_frcteq = require_runtime_surface_scalar(&runtime_surface, "ofe1_frcteq")
            .expect("producer should publish OFE friction equivalent");

        let expected_frcteq = wb16_expected_frcteq(0.45, 0.30, 0.02, 1.20, 0.40, 0.60, 1.00, 2.00);
        let expected_alpha = ((0.04 * 8.0 * WB16_ACCGAV_M_S2) / expected_frcteq).sqrt();

        assert!(
            (projected_frcteq - expected_frcteq).abs() < 1.0e-12,
            "frcteq lineage should match baseline-authoritative chain"
        );
        assert!(
            (projected_primary_alpha - expected_alpha).abs() < 1.0e-12,
            "single-OFE alpha should match baseline-authoritative chain"
        );
        assert!(
            (projected_equivalent_alpha - expected_alpha).abs() < 1.0e-12,
            "single-OFE ealpha should equal alpha"
        );
        assert!(
            (produced - expected_alpha).abs() < 1.0e-12,
            "producer return value should match expected single-OFE ealpha"
        );
    }
    #[test]
    fn hillstab08_wb16_producer_multiofe_projects_expected_equivalent_plane_alpha() {
        let mut runtime_surface = HillslopeWritebackSurface::default();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("nelem"), BoundaryValue::scalar(2.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));
        insert_wb16_ofe_projection_symbols(
            &mut runtime_surface,
            1,
            Wb16OfeSeedVector {
                avgslp: 0.03,
                slplen: 20.0,
                inrcov: 0.50,
                rilcov: 0.25,
                rrinit: 0.02,
                rspace: 1.10,
                width: 0.30,
                rtyp: 2.0,
                cancov: 0.45,
                canhgt: 0.80,
                bb_seed: 0.10,
                bbb_seed: 0.20,
                flivmx_seed: 0.55,
                hmax_seed: 1.80,
            },
        );
        insert_wb16_ofe_projection_symbols(
            &mut runtime_surface,
            2,
            Wb16OfeSeedVector {
                avgslp: 0.06,
                slplen: 35.0,
                inrcov: 0.35,
                rilcov: 0.20,
                rrinit: 0.03,
                rspace: 1.30,
                width: 0.50,
                rtyp: 2.0,
                cancov: 0.40,
                canhgt: 0.70,
                bb_seed: 0.10,
                bbb_seed: 0.20,
                flivmx_seed: 0.50,
                hmax_seed: 1.70,
            },
        );

        let produced = produce_wb16_ealpha_from_runtime_surface(&mut runtime_surface)
            .expect("multi-OFE WB16 producer should execute")
            .expect("multi-OFE WB16 producer should return ealpha");
        let ofe1_alpha = require_runtime_surface_scalar(&runtime_surface, "ofe1_alpha")
            .expect("producer should publish first OFE alpha");
        let ofe2_alpha = require_runtime_surface_scalar(&runtime_surface, "ofe2_alpha")
            .expect("producer should publish second OFE alpha");
        let projected_ealpha = require_runtime_surface_scalar(&runtime_surface, "ealpha")
            .expect("producer should publish equivalent-plane alpha");

        let expected_ealpha =
            wb16_expected_multiofe_ealpha([20.0, 35.0], [ofe1_alpha, ofe2_alpha], 1.5);

        assert!(
            (projected_ealpha - expected_ealpha).abs() < 1.0e-12,
            "multi-OFE ealpha should match baseline-authoritative eplane projection"
        );
        assert!(
            (produced - expected_ealpha).abs() < 1.0e-12,
            "producer return value should match expected multi-OFE ealpha"
        );
    }
