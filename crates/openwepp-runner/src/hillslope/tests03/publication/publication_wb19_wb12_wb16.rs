use super::super::*;

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
            BoundarySymbol::from("wb12_runon_input"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb12_storage_observed"),
            BoundaryValue::scalar(0.03),
        );
        state_surface.insert(
            BoundarySymbol::from("mofe_hourly_carry_arrays_enabled"),
            BoundaryValue::scalar(1.0),
        );

        let mut flux_surface = BTreeMap::new();
        flux_surface.insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("S"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("I"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("ET"), BoundaryValue::scalar(0.05));
        flux_surface.insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.42));
        flux_surface.insert(
            BoundarySymbol::from("wb12_runoff_carryover"),
            BoundaryValue::scalar(0.0),
        );

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
        let final_soil_water =
            state_field_scalar(&response.writeback.state_updates, "wb11_soil_water")
                .expect("storage reconciliation should publish final wb11_soil_water");
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
            (final_soil_water - storage_reconciled).abs() < 1.0e-12,
            "final WB11 soil water must follow WB12 reconciled storage for WB13 publication"
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
        insert_wb16_ofe_projection_symbols(
            &mut runtime_surface,
            2,
            Wb16OfeSeedVector {
                avgslp: 0.08,
                slplen: 20.0,
                inrcov: 0.35,
                rilcov: 0.25,
                rrinit: 0.03,
                rspace: 1.50,
                width: 0.30,
                rtyp: 2.00,
                cancov: 0.40,
                canhgt: 1.20,
                bb_seed: 0.12,
                bbb_seed: 0.22,
                flivmx_seed: 0.72,
                hmax_seed: 1.75,
            },
        );

        let produced = produce_wb16_ealpha_from_runtime_surface(&mut runtime_surface)
            .expect("multi-OFE WB16 producer should execute")
            .expect("multi-OFE WB16 producer should return ealpha");
        let alpha_ofe1 =
            require_runtime_surface_scalar(&runtime_surface, "ofe1_alpha")
                .expect("producer should publish ofe1 alpha");
        let alpha_ofe2 =
            require_runtime_surface_scalar(&runtime_surface, "ofe2_alpha")
                .expect("producer should publish ofe2 alpha");
        let projected_alpha = require_runtime_surface_scalar(&runtime_surface, "ealpha")
            .expect("producer should publish equivalent-plane alpha");

        let frcteq_ofe1 = wb16_expected_frcteq(0.45, 0.30, 0.02, 1.20, 0.40, 0.60, 1.00, 2.00);
        let frcteq_ofe2 = wb16_expected_frcteq(0.35, 0.25, 0.03, 1.50, 0.30, 0.72, 1.20, 1.75);
        let alpha_ofe1_expected = ((0.04 * 8.0 * WB16_ACCGAV_M_S2) / frcteq_ofe1).sqrt();
        let alpha_ofe2_expected = ((0.08 * 8.0 * WB16_ACCGAV_M_S2) / frcteq_ofe2).sqrt();
        let expected_equivalent_alpha = wb16_expected_multiofe_ealpha(
            [30.0, 20.0],
            [alpha_ofe1_expected, alpha_ofe2_expected],
            1.5,
        );

        assert!(
            (alpha_ofe1 - alpha_ofe1_expected).abs() < 1.0e-12,
            "first OFE alpha should match baseline-authoritative chain"
        );
        assert!(
            (alpha_ofe2 - alpha_ofe2_expected).abs() < 1.0e-12,
            "second OFE alpha should match baseline-authoritative chain"
        );
        assert!(
            (projected_alpha - expected_equivalent_alpha).abs() < 1.0e-12,
            "multi-OFE equivalent-plane alpha should match projected multiofe chain"
        );
        assert!(
            (produced - expected_equivalent_alpha).abs() < 1.0e-12,
            "producer return value should match expected multi-OFE ealpha"
        );
    }
