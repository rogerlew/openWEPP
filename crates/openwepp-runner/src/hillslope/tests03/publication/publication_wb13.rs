use super::super::*;

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
    fn hphys0203_wb13_soil_water_total_preserves_watcon_alias() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.081),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_ws_frz"),
            BoundaryValue::scalar(0.020),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_frwatc_frozen_water_after_m"),
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

        assert!(
            (row.wb13_row.frozwt - 3.0).abs() <= SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
            "frozwt must follow the exchanged-store diagnostic, observed {}",
            row.wb13_row.frozwt
        );

        let closure_delta = row.wb13_row.soil_water_total - row.wb13_row.total_soil;
        assert!(
            closure_delta.abs() <= SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
            "SoilWaterTotal must remain a hydout-equivalent Total-Soil alias, observed delta={closure_delta}"
        );
    }
    #[test]
    fn hphys0203_wb13_profile_storage_perturbation_is_stable() {
        let runtime_surface = seeded_wb13_runtime_surface_probe();
        let mut perturbed_runtime_surface = runtime_surface.clone();
        perturbed_runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("thetfc_0001"), BoundaryValue::scalar(0.300_000_001));

        let baseline_row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("baseline WB13 probe should publish row");
        let perturbed_row = build_simulation_owned_wb13_row(
            &perturbed_runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("perturbed FC profile should remain publication-stable");

        let fc_perturbation_delta = (perturbed_row.wb13_row.profile_fc_store - baseline_row.wb13_row.profile_fc_store).abs();
        assert!(
            (0.0..1.0e-3).contains(&fc_perturbation_delta),
            "ProfileFCStore perturbation should be bounded and monotonic, observed delta={fc_perturbation_delta}"
        );
        assert!(
            perturbed_row.wb13_row.profile_fc_store >= perturbed_row.wb13_row.profile_wp_store,
            "ProfileFCStore ordering should remain valid after small profile perturbation"
        );
    }
