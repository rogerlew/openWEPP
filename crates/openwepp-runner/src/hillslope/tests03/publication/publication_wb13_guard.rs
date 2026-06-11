use super::super::*;

    #[test]
    fn fdhp01_wb13_frozwt_guard_rejects_missing_exchange_store_symbol() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("frost.runtime_frwatc_frozen_water_after_m"));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("missing exchanged frozen store must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("missing required runtime symbol frost.runtime_frwatc_frozen_water_after_m"),
                    "expected missing exchanged-store guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

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
            .state_surface
            .insert(BoundarySymbol::from("Es"), BoundaryValue::scalar(-1.0e-13));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect(
            "near-zero negative WB13 soil evaporation should canonicalize to zero without EVAPPM clamp",
        );

        assert!(
            row.wb13_row.es.abs() < 1.0e-12,
            "near-zero negative ES should canonicalize to zero without EVAPPM branch"
        );
    }
