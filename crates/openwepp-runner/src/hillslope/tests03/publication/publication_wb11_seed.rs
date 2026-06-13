use super::super::*;

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
    fn mofe01_mb_wb11_seed_purges_stale_daily_carryover_for_mofe_hourly_arrays() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 2.0),
            ("erod14_wave2_enabled", 0.0),
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
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("wb12_runoff_carryover"),
            BoundaryValue::scalar(0.25),
        );

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("MOFE WB11 seed should succeed");

        let mofe_enabled =
            require_runtime_surface_scalar(&runtime_surface, "mofe_hourly_carry_arrays_enabled")
                .expect("MOFE seed should publish carry-array enablement");
        assert!(
            (mofe_enabled - 1.0).abs() < 1.0e-12,
            "multi-OFE seed must enable MOFE hourly arrays"
        );
        assert!(
            !runtime_surface
                .flux_surface
                .contains_key(&BoundarySymbol::from("wb12_runoff_carryover")),
            "MOFE hourly array lanes must not present stale daily aggregate carryover to WB14"
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
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("wb19_solthk_0001"), BoundaryValue::scalar(0.25));

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("PMET-mode WB11 seed should succeed");

        let es_storage_return_m = require_runtime_surface_scalar(
            &runtime_surface,
            "pmet.es_storage_return_m",
        )
        .expect("PMET seed diagnostics must publish condensation storage-return meter depth");
        assert!(
            es_storage_return_m.is_finite() && es_storage_return_m >= 0.0,
            "PMET diagnostics must publish a finite non-negative condensation storage-return"
        );
        assert!(
            es_storage_return_m < 1.0e-3,
            "PMET condensation storage-return should remain bounded under this seeded daily EVAPPM lane"
        );
    }
