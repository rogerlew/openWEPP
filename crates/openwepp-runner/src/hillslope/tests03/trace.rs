use super::*;

    #[test]
    fn hphys0245_trace_config_limits_requested_days() {
        let config = Hphys0245TraceConfig {
            path: PathBuf::from("trace.jsonl"),
            max_days: Some(30),
        };

        assert!(config.includes_day(1));
        assert!(config.includes_day(30));
        assert!(!config.includes_day(31));

        let unbounded = Hphys0245TraceConfig {
            path: PathBuf::from("trace.jsonl"),
            max_days: None,
        };
        assert!(unbounded.includes_day(31));
    }
    #[test]
    fn hphys0245_trace_row_captures_storage_and_percolation_symbols() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.25),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.10),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0002"),
            BoundaryValue::scalar(0.12),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("wb18_perc_pei_0001"),
            BoundaryValue::scalar(0.003),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("wb18_perc_pei_0002"),
            BoundaryValue::scalar(0.004),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.004));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Pe"), BoundaryValue::scalar(0.004));

        let row = build_hphys0245_trace_row(
            "H1",
            1,
            1,
            2013,
            1,
            "post_phase",
            Some("percolation_deep_seepage"),
            &surface,
            None,
            None,
        );

        assert_eq!(row.schema, HPHYS0245_TRACE_SCHEMA);
        assert_eq!(row.run_name, "H1");
        assert_eq!(row.boundary, "post_phase");
        assert_eq!(row.phase.as_deref(), Some("percolation_deep_seepage"));
        assert!((row.wb11_soil_water_m.expect("wb11") - 0.25).abs() < 1.0e-12);
        assert!((row.wb11_soil_water_mm.expect("wb11 mm") - 250.0).abs() < 1.0e-12);
        assert!((row.wb18_theta_sum_m.expect("theta sum") - 0.22).abs() < 1.0e-12);
        assert!((row.wb18_pei_sum_m.expect("pei sum") - 0.007).abs() < 1.0e-12);
        assert!((row.d_m.expect("D") - 0.004).abs() < 1.0e-12);
        assert!((row.pe_m.expect("Pe") - 0.004).abs() < 1.0e-12);
        assert!((row.wb11_minus_theta_sum_m.expect("delta") - 0.03).abs() < 1.0e-12);
    }

    #[test]
    fn fdhp01_c1b_wb18_guard_terms_prefer_wb19_layer_geometry() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_fc_0002"),
            BoundaryValue::scalar(0.028_96),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_ul_0002"),
            BoundaryValue::scalar(0.051_873_154_5),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0002"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_ssc_0002"),
            BoundaryValue::scalar(9.17e-6),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_dg_0002"),
            BoundaryValue::scalar(0.20),
        );
        surface
            .state_surface
            .insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.03));
        surface.state_surface.insert(
            BoundarySymbol::from("thetdr_0002"),
            BoundaryValue::scalar(0.1578),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_frozen_depth_0002"),
            BoundaryValue::scalar(0.20),
        );

        let terms = format_wb18_perc_guard_terms(&surface);

        assert!(
            terms.contains("invalid_layers=none"),
            "C1b guard diagnostics must validate frozen depth against preferred wb19_dg geometry when present, observed {terms}"
        );
    }

    #[test]
    fn hphys0259_trace_row_captures_wb19_lateral_diagnostics() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_q_lateral_potential"),
            BoundaryValue::scalar(0.120),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_q_lateral_target"),
            BoundaryValue::scalar(0.080),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_capacity_tdv"),
            BoundaryValue::scalar(0.080),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_tdvv"),
            BoundaryValue::scalar(0.080),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_q_lateral_unrealized"),
            BoundaryValue::scalar(0.020),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_withdrawal_0001"),
            BoundaryValue::scalar(0.030),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_withdrawal_0002"),
            BoundaryValue::scalar(0.050),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_capacity_active_count_0001"),
            BoundaryValue::scalar(24.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_conductivity_active_count_0001"),
            BoundaryValue::scalar(12.0),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.080));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Qdd"), BoundaryValue::scalar(0.010));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.090));

        let row = build_hphys0245_trace_row(
            "H39",
            1,
            1,
            2013,
            1,
            "post_phase",
            Some("lateral_transfer"),
            &surface,
            None,
            None,
        );

        assert_eq!(row.schema, HPHYS0245_TRACE_SCHEMA);
        assert_eq!(row.phase.as_deref(), Some("lateral_transfer"));
        assert!((row.wb19_q_lateral_potential_m.expect("potential") - 0.120).abs() < 1.0e-12);
        assert!((row.wb19_q_lateral_target_m.expect("target") - 0.080).abs() < 1.0e-12);
        assert!((row.wb19_lateral_capacity_tdv_m.expect("capacity tdv") - 0.080).abs() < 1.0e-12);
        assert!((row.wb19_tdvv_m.expect("tdvv") - 0.080).abs() < 1.0e-12);
        assert!((row.wb19_q_lateral_unrealized_m.expect("unrealized") - 0.020).abs() < 1.0e-12);
        assert_eq!(
            row.wb19_lateral_withdrawal_layers_m.get("0001").copied(),
            Some(0.030)
        );
        assert_eq!(
            row.wb19_lateral_withdrawal_layers_m.get("0002").copied(),
            Some(0.050)
        );
        assert_eq!(
            row.wb19_lateral_capacity_active_count_layers
                .get("0001")
                .copied(),
            Some(24.0)
        );
        assert_eq!(
            row.wb19_lateral_conductivity_active_count_layers
                .get("0001")
                .copied(),
            Some(12.0)
        );
        assert!((row.q_m.expect("q") - 0.080).abs() < 1.0e-12);
        assert!((row.qdd_m.expect("Qdd") - 0.010).abs() < 1.0e-12);
        assert!((row.qd_m.expect("Qd") - 0.090).abs() < 1.0e-12);
    }
    #[test]
    #[allow(clippy::too_many_lines)]
    fn hphys0260_trace_row_captures_wb17_wb18_storage_diagnostics() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.256),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.10),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0002"),
            BoundaryValue::scalar(0.12),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_thetdr_0001"),
            BoundaryValue::scalar(0.05),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_thetdr_0002"),
            BoundaryValue::scalar(0.07),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_dg_0001"),
            BoundaryValue::scalar(0.30),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_dg_0002"),
            BoundaryValue::scalar(0.40),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_fc_0001"),
            BoundaryValue::scalar(0.030),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_fc_0002"),
            BoundaryValue::scalar(0.040),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("coca_0001"),
            BoundaryValue::scalar(0.80),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_coca_0002"),
            BoundaryValue::scalar(0.75),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_frzw_0002"),
            BoundaryValue::scalar(0.005),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_frzw_0001"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_frozen_depth_0002"),
            BoundaryValue::scalar(0.10),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("UPi"), BoundaryValue::scalar(0.005));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Ui"), BoundaryValue::scalar(0.0025));
        surface.flux_surface.insert(
            BoundarySymbol::from("UPi_0001"),
            BoundaryValue::scalar(0.003),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("UPi_0002"),
            BoundaryValue::scalar(0.002),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("Ui_0001"),
            BoundaryValue::scalar(0.001),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("Ui_0002"),
            BoundaryValue::scalar(0.0015),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.0025));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Etp"), BoundaryValue::scalar(0.005));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Ws"), BoundaryValue::scalar(0.5));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.004));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Pe"), BoundaryValue::scalar(0.004));

        let row = build_hphys0245_trace_row(
            "H7",
            1,
            1,
            2013,
            1,
            "post_phase",
            Some("plant_root_uptake"),
            &surface,
            None,
            None,
        );

        assert_eq!(row.schema, HPHYS0245_TRACE_SCHEMA);
        assert_eq!(row.wb17_upi_layers_m.get("0001").copied(), Some(0.003));
        assert_eq!(row.wb17_upi_layers_m.get("0002").copied(), Some(0.002));
        assert_eq!(row.wb17_ui_layers_m.get("0001").copied(), Some(0.001));
        assert_eq!(row.wb17_ui_layers_m.get("0002").copied(), Some(0.0015));
        assert_eq!(row.wb18_thetdr_layers.get("0001").copied(), Some(0.05));
        assert_eq!(row.wb18_dg_layers_m.get("0002").copied(), Some(0.40));
        assert_eq!(row.wb18_fc_layers_m.get("0001").copied(), Some(0.030));
        assert_eq!(row.wb19_coca_layers.get("0001").copied(), Some(0.80));
        assert_eq!(row.wb19_coca_layers.get("0002").copied(), Some(0.75));
        assert_eq!(row.wb19_frzw_layers_m.get("0002").copied(), Some(0.005));
        assert!((row.wb19_drfc_layers_m["0001"] - 0.090).abs() < 1.0e-12);
        assert!((row.wb19_drfc_layers_m["0002"] - 0.140).abs() < 1.0e-12);
        assert!((row.wb19_fzdrfc_layers_m["0001"] - 0.090).abs() < 1.0e-12);
        assert!((row.wb19_fzdrfc_layers_m["0002"] - 0.135).abs() < 1.0e-12);
        assert_eq!(
            row.wb18_frozen_depth_layers_m.get("0002").copied(),
            Some(0.10)
        );
        assert!((row.wb18_recomputed_soil_water_m.expect("aggregate") - 0.256).abs() < 1.0e-12);
        assert!((row.wb18_recomputed_minus_wb11_m.expect("delta")).abs() < 1.0e-12);
        assert!((row.upi_m.expect("UPi") - 0.005).abs() < 1.0e-12);
        assert!((row.ui_m.expect("Ui") - 0.0025).abs() < 1.0e-12);
        assert!((row.ep_m.expect("Ep") - 0.0025).abs() < 1.0e-12);
        assert!((row.ws.expect("Ws") - 0.5).abs() < 1.0e-12);
        assert!((row.d_m.expect("D") - row.pe_m.expect("Pe")).abs() < 1.0e-12);
    }
    #[test]
    fn hphys0261_trace_row_captures_ep_initialization_magnitude_lineage() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.052),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_ul_0001"),
            BoundaryValue::scalar(0.113),
        );
        surface
            .state_surface
            .insert(BoundarySymbol::from("pltol"), BoundaryValue::scalar(0.33));
        surface.state_surface.insert(
            BoundarySymbol::from("swu_effective_pltol"),
            BoundaryValue::scalar(0.33),
        );
        surface
            .state_surface
            .insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(11.8));
        surface
            .state_surface
            .insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(1.8));
        surface.flux_surface.insert(
            BoundarySymbol::from("UPi_0001"),
            BoundaryValue::scalar(0.0001),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("Ui_0001"),
            BoundaryValue::scalar(0.0001),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("Etp"),
            BoundaryValue::scalar(0.000_385),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.000_385));

        let row = build_hphys0245_trace_row(
            "H1",
            1,
            1,
            2013,
            1,
            "post_phase",
            Some("plant_root_uptake"),
            &surface,
            None,
            None,
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["pl_pltol"], 0.33);
        assert_eq!(document["pl_swu_effective_pltol"], 0.33);
        assert_eq!(document["wb18_ul_layers_m"]["0001"], 0.113);
        assert!(
            (document["wb17_swu_stress_threshold_layers_m"]["0001"]
                .as_f64()
                .unwrap()
                - 0.03729)
                .abs()
                < 1.0e-12
        );
        assert!(
            document["wb17_swu_storage_to_threshold_layers"]["0001"]
                .as_f64()
                .unwrap()
                > 1.0
        );
    }
    #[test]
    fn hphys0262_trace_row_captures_pmet_demand_seeding_lineage() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.mode.sidecar_present"),
            BoundaryValue::scalar(1.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.mode.iflget"),
            BoundaryValue::scalar(2.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.selected.kcb"),
            BoundaryValue::scalar(0.95),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.selected.rawp"),
            BoundaryValue::scalar(0.80),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.selected.line_index"),
            BoundaryValue::scalar(39.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.lookup.fallback_first_row_used"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb11_et_demand"),
            BoundaryValue::scalar(0.000_385),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb11_et_seed_branch_priestley_taylor"),
            BoundaryValue::scalar(1.0),
        );

        let row = build_hphys0245_trace_row(
            "H39",
            1,
            1,
            2013,
            1,
            "post_seed",
            None,
            &surface,
            None,
            None,
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["pmet_sidecar_present"], 1.0);
        assert_eq!(document["pmet_iflget"], 2.0);
        assert_eq!(document["pmet_selected_kcb"], 0.95);
        assert_eq!(document["pmet_selected_rawp"], 0.80);
        assert_eq!(document["pmet_selected_line_index"], 39.0);
        assert_eq!(document["pmet_lookup_fallback_first_row_used"], 0.0);
        assert_eq!(document["wb11_et_demand_m"], 0.000_385);
        assert_eq!(document["wb11_et_seed_branch"], "evap_priestley_taylor");
    }
    #[test]
    fn hphys0262_projects_pmetpara_selected_crop_coefficients() {
        let fixture_dir = fixture_path("hillslope_run_dir");
        let management = parse_management_from_path(
            fixture_dir.join("case.man"),
            SidecarPolicy::Compat.as_management_parser_mode(),
        )
        .expect("fixture management should parse");
        let mut pmetpara = parse_pmetpara_file(
            fixture_dir.join("pmetpara.txt"),
            PmetparaParseOptions {
                mode: SidecarPolicy::Compat.as_pmetpara_parse_mode(),
                require_sidecar: true,
            },
        )
        .expect("fixture pmetpara should parse");

        let surface = crate::hillslope::intake_lane_setup::build_hillslope_runtime_surface_from_pmetpara(
            &management,
            &mut pmetpara,
            SidecarPolicy::Compat.as_pmetpara_parse_mode(),
        )
        .expect("pmetpara should project");

        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.mode.sidecar_present"),
            Some(1.0)
        );
        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.mode.iflget"),
            Some(2.0)
        );
        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.selected.kcb"),
            Some(1.20)
        );
        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.selected.rawp"),
            Some(0.55)
        );
        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.selected.line_index"),
            Some(1.0)
        );
        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.lookup.fallback_first_row_used"),
            Some(0.0)
        );
    }
    #[test]
    #[allow(clippy::too_many_lines)]
    fn hphys0245_trace_writer_serializes_jsonl_rows() {
        let temp_dir = std::env::temp_dir().join(format!(
            "openwepp_hphys0245_trace_writer_{}",
            std::process::id()
        ));
        let trace_path = temp_dir.join("trace.jsonl");
        let config = Hphys0245TraceConfig {
            path: trace_path.clone(),
            max_days: Some(1),
        };
        let row = Hphys0245TraceRow {
            schema: HPHYS0245_TRACE_SCHEMA,
            run_name: "H1".to_string(),
            sim_day_index: 1,
            simulation_year: 1,
            calendar_year: 2013,
            julian_day: 1,
            boundary: "post_seed".to_string(),
            phase: None,
            wb11_soil_water_m: Some(0.1),
            wb11_soil_water_mm: Some(100.0),
            wb12_infiltration_m: Some(0.003),
            wb12_rainfall_input_m: Some(0.004),
            wb12_runon_input_m: Some(0.001),
            wb12_depression_storage_delta_m: Some(0.0),
            wb12_partition_liquid_supply_m: Some(0.008),
            wb12_partition_residual_before_q_m: Some(0.005),
            wb14_soil_conductivity_m_s: Some(2.0e-6),
            wb14_frost_infcap_m_s: None,
            wb14_effective_conductivity_m_s: Some(2.0e-6),
            wb14_soil_layer_depth_m: Some(0.40),
            wb14_theta_residual: Some(0.05),
            wb14_theta_field_capacity: Some(0.20),
            wb14_matric_potential_m: Some(0.06),
            wb18_theta_sum_m: Some(0.08),
            wb18_theta_layers_m: BTreeMap::from([("0001".to_string(), 0.08)]),
            wb18_thetdr_layers: BTreeMap::from([("0001".to_string(), 0.05)]),
            wb18_dg_layers_m: BTreeMap::from([("0001".to_string(), 0.40)]),
            wb18_fc_layers_m: BTreeMap::from([("0001".to_string(), 0.06)]),
            wb19_coca_layers: BTreeMap::from([("0001".to_string(), 0.75)]),
            wb19_frzw_layers_m: BTreeMap::from([("0001".to_string(), 0.01)]),
            wb19_drfc_layers_m: BTreeMap::from([("0001".to_string(), 0.16)]),
            wb19_fzdrfc_layers_m: BTreeMap::from([("0001".to_string(), 0.15)]),
            wb18_frozen_depth_layers_m: BTreeMap::new(),
            wb18_recomputed_soil_water_m: Some(0.10),
            wb18_recomputed_minus_wb11_m: Some(0.0),
            wb18_pei_sum_m: Some(0.0),
            wb18_pei_layers_m: BTreeMap::new(),
            d_m: None,
            pe_m: None,
            wb13_dp_mm: None,
            wb13_total_soil_mm: None,
            wb13_soil_water_total_mm: None,
            snow_runtime_swe_m: Some(0.42),
            snow_runtime_depth_m: Some(1.20),
            snow_runtime_density_kg_m3: Some(350.0),
            snow_runtime_settle_day_count: Some(4.0),
            snow_runtime_swe_before_m: Some(0.40),
            snow_runtime_depth_before_m: Some(1.10),
            snow_runtime_density_before_kg_m3: Some(340.0),
            snow_runtime_settle_day_count_before: Some(3.0),
            snow_runtime_swe_delta_m: Some(0.02),
            snow_runtime_depth_delta_m: Some(0.10),
            snow_runtime_density_delta_kg_m3: Some(10.0),
            snow_runtime_settle_day_count_delta: Some(1.0),
            snow_s_m: Some(0.002),
            snow_routed_melt_m: Some(0.003),
            snow_post_winter_rain_m: Some(0.004),
            snow_hourly_rain_sum_m: Some(0.001),
            snow_hourly_rain_retained_sum_m: Some(0.0),
            snow_hourly_rain_released_sum_m: Some(0.0),
            snow_hourly_snowfall_depth_sum_m: Some(0.010),
            snow_hourly_snowfall_water_equiv_sum_m: Some(0.001),
            snow_hourly_melt_raw_sum_m: Some(0.003),
            snow_hourly_melt_sum_m: Some(0.003),
            snow_hourly_rain_m: BTreeMap::from([("0001".to_string(), 0.001)]),
            snow_hourly_snowfall_depth_m: BTreeMap::from([("0001".to_string(), 0.010)]),
            snow_hourly_stmtim_rain_m: BTreeMap::from([("0001".to_string(), 0.012)]),
            snow_hourly_stmtim_stmdur_s: BTreeMap::from([("0001".to_string(), 10_800.0)]),
            snow_hourly_stmtim_wntdur_h: BTreeMap::from([("0001".to_string(), 3.0)]),
            snow_hourly_stmtim_wnttim_h: BTreeMap::from([("0001".to_string(), 1.0)]),
            snow_hourly_stmtim_hrtemp_c: BTreeMap::from([("0001".to_string(), -2.0)]),
            snow_hourly_stmtim_rst_c: BTreeMap::from([("0001".to_string(), 0.0)]),
            snow_hourly_stmtim_hrrain_m: BTreeMap::from([("0001".to_string(), 0.0)]),
            snow_hourly_stmtim_hrsnow_m: BTreeMap::from([("0001".to_string(), 0.040)]),
            snow_hourly_stmtim_active_interval: BTreeMap::from([("0001".to_string(), 1.0)]),
            snow_hourly_stmtim_rain_branch: BTreeMap::from([("0001".to_string(), 0.0)]),
            snow_hourly_stmtim_snow_branch: BTreeMap::from([("0001".to_string(), 1.0)]),
            snow_hourly_depth_before_m: BTreeMap::from([("0001".to_string(), 1.10)]),
            snow_hourly_depth_available_m: BTreeMap::from([("0001".to_string(), 1.09)]),
            snow_hourly_depth_after_m: BTreeMap::from([("0001".to_string(), 1.08)]),
            snow_hourly_density_before_kg_m3: BTreeMap::from([("0001".to_string(), 340.0)]),
            snow_hourly_density_after_kg_m3: BTreeMap::from([("0001".to_string(), 350.0)]),
            snow_hourly_melt_raw_m: BTreeMap::from([("0001".to_string(), 0.003)]),
            snow_hourly_melt_m: BTreeMap::from([("0001".to_string(), 0.003)]),
            snow_hourly_melt_amelt_in: BTreeMap::from([("0001".to_string(), 0.10)]),
            snow_hourly_melt_bmelt_in: BTreeMap::from([("0001".to_string(), 0.20)]),
            snow_hourly_melt_cmelt_in: BTreeMap::from([("0001".to_string(), 0.30)]),
            snow_hourly_melt_dmelt_in: BTreeMap::from([("0001".to_string(), 0.40)]),
            snow_hourly_melt_hrtef_f: BTreeMap::from([("0001".to_string(), 36.0)]),
            snow_hourly_melt_hrdtf_f: BTreeMap::from([("0001".to_string(), 30.0)]),
            snow_hourly_melt_vwmph: BTreeMap::from([("0001".to_string(), 4.0)]),
            snow_hourly_melt_rainin: BTreeMap::from([("0001".to_string(), 0.01)]),
            snow_hourly_melt_wind_adjustment: BTreeMap::from([("0001".to_string(), 1.07)]),
            snow_hourly_melt_branch_active: BTreeMap::from([("0001".to_string(), 1.0)]),
            winter_hourly_air_temp_c: BTreeMap::from([("0001".to_string(), 2.0)]),
            winter_hourly_rad_mj_m2: BTreeMap::from([("0001".to_string(), 1.5)]),
            winter_hourly_cloud_fraction: BTreeMap::from([("0001".to_string(), 0.5)]),
            winter_hourly_dewpoint_c: BTreeMap::from([("0001".to_string(), -1.0)]),
            winter_hourly_wind_m_s: BTreeMap::from([("0001".to_string(), 2.0)]),
            snow_runtime_swe_closure_error_m: Some(0.0),
            wb13_p_mm: Some(10.0),
            wb13_rm_mm: Some(2.0),
            wb13_q_mm: Some(1.5),
            wb13_snow_water_mm: Some(420.0),
            wb11_minus_theta_sum_m: Some(0.02),
            pl_sumgdd: Some(42.0),
            pl_vdmt: Some(1.5),
            pl_cancov: Some(0.4),
            pl_lai: Some(1.2),
            pl_rtmass: Some(0.7),
            pl_rtd: Some(0.6),
            pl_hia: Some(0.2),
            pl_pltol: Some(0.33),
            pl_swu_effective_pltol: Some(0.33),
            pmet_sidecar_present: Some(1.0),
            pmet_iflget: Some(2.0),
            pmet_selected_kcb: Some(0.95),
            pmet_selected_rawp: Some(0.8),
            pmet_selected_line_index: Some(1.0),
            pmet_lookup_fallback_first_row_used: Some(0.0),
            wb11_et_demand_m: Some(0.003),
            wb11_et_seed_branch: Some("evappm_pmet".to_string()),
            pmet_etorc_mm: Some(3.5),
            pmet_rn_mj_m2: Some(4.2),
            pmet_fwv_m_s: Some(2.1),
            pmet_rhd_pct: Some(60.0),
            pmet_kcbadj: Some(0.95),
            pmet_kcbcon: Some(0.7),
            pmet_etke: Some(0.3),
            pmet_etkr: Some(1.0),
            pmet_etks: Some(0.8),
            pmet_tew_mm: Some(25.0),
            pmet_rew_mm: Some(8.0),
            pmet_wfevp_mm: Some(12.0),
            pmet_taw_mm: Some(40.0),
            pmet_raw_mm: Some(20.0),
            pmet_wftrp_mm: Some(30.0),
            pmet_es_m: Some(0.001),
            pmet_ep_m: Some(0.003),
            etp_m: Some(0.003),
            upi_m: Some(0.003),
            ui_m: Some(0.002),
            wb18_ul_layers_m: BTreeMap::from([("0001".to_string(), 0.24)]),
            wb17_swu_stress_threshold_layers_m: BTreeMap::from([("0001".to_string(), 0.0792)]),
            wb17_swu_storage_to_threshold_layers: BTreeMap::from([(
                "0001".to_string(),
                1.010_101_010_101_010_2,
            )]),
            wb17_upi_layers_m: BTreeMap::from([("0001".to_string(), 0.003)]),
            wb17_ui_layers_m: BTreeMap::from([("0001".to_string(), 0.002)]),
            ep_m: Some(0.002),
            ws: Some(0.8),
            wb19_q_lateral_potential_m: Some(0.12),
            wb19_q_lateral_target_m: Some(0.08),
            wb19_lateral_capacity_tdv_m: Some(0.08),
            wb19_tdvv_m: Some(0.08),
            wb19_q_lateral_unrealized_m: Some(0.0),
            wb19_lateral_withdrawal_layers_m: BTreeMap::from([("0001".to_string(), 0.08)]),
            wb19_lateral_capacity_active_count_layers: BTreeMap::from([("0001".to_string(), 24.0)]),
            wb19_lateral_conductivity_active_count_layers: BTreeMap::from([(
                "0001".to_string(),
                24.0,
            )]),
            q_m: Some(0.08),
            qdd_m: Some(0.01),
            qd_m: Some(0.09),
        };

        write_hphys0245_trace_jsonl(&config, &[row]).expect("trace writer should succeed");

        let payload = fs::read_to_string(&trace_path).expect("trace file should be readable");
        let lines = payload.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 1);
        let document: serde_json::Value =
            serde_json::from_str(lines[0]).expect("trace row should parse as JSON");
        assert_eq!(document["schema"], HPHYS0245_TRACE_SCHEMA);
        assert_eq!(document["boundary"], "post_seed");
        assert_eq!(document["wb18_theta_layers_m"]["0001"], 0.08);
        assert_eq!(document["wb18_thetdr_layers"]["0001"], 0.05);
        assert_eq!(document["wb18_dg_layers_m"]["0001"], 0.40);
        assert_eq!(document["wb18_fc_layers_m"]["0001"], 0.06);
        assert_eq!(document["wb19_coca_layers"]["0001"], 0.75);
        assert_eq!(document["wb19_frzw_layers_m"]["0001"], 0.01);
        assert_eq!(document["wb19_drfc_layers_m"]["0001"], 0.16);
        assert_eq!(document["wb19_fzdrfc_layers_m"]["0001"], 0.15);
        assert_eq!(document["wb18_recomputed_soil_water_m"], 0.10);
        assert_eq!(document["pl_pltol"], 0.33);
        assert_eq!(document["pl_swu_effective_pltol"], 0.33);
        assert_eq!(document["pmet_iflget"], 2.0);
        assert_eq!(document["pmet_selected_kcb"], 0.95);
        assert_eq!(document["wb11_et_seed_branch"], "evappm_pmet");
        assert_eq!(document["wb18_ul_layers_m"]["0001"], 0.24);
        assert_eq!(
            document["wb17_swu_stress_threshold_layers_m"]["0001"],
            0.0792
        );
        assert_eq!(document["wb17_upi_layers_m"]["0001"], 0.003);
        assert_eq!(document["wb17_ui_layers_m"]["0001"], 0.002);
        assert_eq!(document["pl_rtd"], 0.6);
        assert_eq!(document["ep_m"], 0.002);
        assert_eq!(document["snow_runtime_swe_m"], 0.42);
        assert_eq!(document["snow_runtime_swe_before_m"], 0.40);
        assert_eq!(document["snow_runtime_swe_delta_m"], 0.02);
        assert_eq!(document["snow_routed_melt_m"], 0.003);
        assert_eq!(document["snow_post_winter_rain_m"], 0.004);
        assert_eq!(document["snow_hourly_snowfall_water_equiv_sum_m"], 0.001);
        assert_eq!(document["snow_hourly_rain_released_sum_m"], 0.0);
        assert_eq!(document["snow_hourly_rain_m"]["0001"], 0.001);
        assert_eq!(document["snow_hourly_snowfall_depth_m"]["0001"], 0.010);
        assert_eq!(document["snow_hourly_stmtim_rain_m"]["0001"], 0.012);
        assert_eq!(document["snow_hourly_stmtim_stmdur_s"]["0001"], 10_800.0);
        assert_eq!(document["snow_hourly_stmtim_wntdur_h"]["0001"], 3.0);
        assert_eq!(document["snow_hourly_stmtim_wnttim_h"]["0001"], 1.0);
        assert_eq!(document["snow_hourly_stmtim_hrtemp_c"]["0001"], -2.0);
        assert_eq!(document["snow_hourly_stmtim_rst_c"]["0001"], 0.0);
        assert_eq!(document["snow_hourly_stmtim_hrrain_m"]["0001"], 0.0);
        assert_eq!(document["snow_hourly_stmtim_hrsnow_m"]["0001"], 0.040);
        assert_eq!(document["snow_hourly_stmtim_active_interval"]["0001"], 1.0);
        assert_eq!(document["snow_hourly_stmtim_rain_branch"]["0001"], 0.0);
        assert_eq!(document["snow_hourly_stmtim_snow_branch"]["0001"], 1.0);
        assert_eq!(document["snow_hourly_depth_before_m"]["0001"], 1.10);
        assert_eq!(document["snow_hourly_depth_available_m"]["0001"], 1.09);
        assert_eq!(document["snow_hourly_depth_after_m"]["0001"], 1.08);
        assert_eq!(document["snow_hourly_density_before_kg_m3"]["0001"], 340.0);
        assert_eq!(document["snow_hourly_density_after_kg_m3"]["0001"], 350.0);
        assert_eq!(document["wb12_infiltration_m"], 0.003);
        assert_eq!(document["wb12_partition_liquid_supply_m"], 0.008);
        assert_eq!(document["wb12_partition_residual_before_q_m"], 0.005);
        assert_eq!(document["wb14_effective_conductivity_m_s"], 2.0e-6);
        assert_eq!(document["wb14_matric_potential_m"], 0.06);
        assert_eq!(document["snow_hourly_melt_raw_m"]["0001"], 0.003);
        assert_eq!(document["snow_hourly_melt_m"]["0001"], 0.003);
        assert_eq!(document["snow_hourly_melt_amelt_in"]["0001"], 0.10);
        assert_eq!(document["winter_hourly_air_temp_c"]["0001"], 2.0);
        assert_eq!(document["snow_runtime_swe_closure_error_m"], 0.0);
        assert_eq!(document["wb13_rm_mm"], 2.0);
        assert_eq!(document["wb13_q_mm"], 1.5);
        assert_eq!(document["wb19_lateral_withdrawal_layers_m"]["0001"], 0.08);
        assert_eq!(document["q_m"], 0.08);

        fs::remove_dir_all(temp_dir).expect("temp trace directory should be removable");
    }
    #[test]
    #[allow(clippy::too_many_lines)]
    fn hphys0268_trace_row_captures_spring_snowpack_lineage() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.120),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_depth_m"),
            BoundaryValue::scalar(0.600),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_density_kg_m3"),
            BoundaryValue::scalar(200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_settle_day_count"),
            BoundaryValue::scalar(3.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.options.newsnw"),
            BoundaryValue::scalar(100.0),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("S"), BoundaryValue::scalar(0.002));
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_m_0001"),
            BoundaryValue::scalar(0.004),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.snowfall_m_0001"),
            BoundaryValue::scalar(0.010),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_m_0001"),
            BoundaryValue::scalar(0.003),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_raw_m_0001"),
            BoundaryValue::scalar(0.003),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_retained_m_0001"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.depth_before_m_0001"),
            BoundaryValue::scalar(0.600),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.depth_available_m_0001"),
            BoundaryValue::scalar(0.590),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.depth_after_m_0001"),
            BoundaryValue::scalar(0.580),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.density_before_kg_m3_0001"),
            BoundaryValue::scalar(190.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.density_after_kg_m3_0001"),
            BoundaryValue::scalar(200.0),
        );
        let wb13_row = SimulationOwnedWb13Row {
            wb13_row: Wb13DailyWaterBalanceRow {
                ofe: 1,
                julian_day: 99,
                year: 1,
                p: 10.0,
                rm: 12.0,
                q: 0.0,
                ep: 1.5,
                es: 0.2,
                er: 0.0,
                dp: 0.1,
                upstrmq: 0.0,
                subrin: 0.0,
                latqcc: 0.0,
                total_soil: 200.0,
                frozwt: 0.0,
                snow_water: 120.0,
                qofe: 0.0,
                tile: 0.0,
                irr: 0.0,
                area: 10_000.0,
                soil_water_total: 200.0,
                profile_depth: 1_000.0,
                profile_porosity_cap: 300.0,
                profile_fc_store: 220.0,
                profile_wp_store: 120.0,
            },
            interception_mm: 0.25,
            frdp_mm: 0.0,
            month: 4,
            day_of_month: 9,
            water_year: 1,
            sim_day_index: 99,
        };

        let row = build_hphys0245_trace_row(
            "H1",
            1,
            99,
            2013,
            99,
            "post_wb13",
            None,
            &surface,
            Some(&wb13_row),
            None,
        );

        assert!((row.snow_runtime_swe_m.expect("runtime swe") - 0.120).abs() < 1.0e-12);
        assert!((row.snow_runtime_depth_m.expect("runtime depth") - 0.600).abs() < 1.0e-12);
        assert!((row.snow_runtime_density_kg_m3.expect("runtime density") - 200.0).abs() < 1.0e-12);
        assert!(
            (row.snow_hourly_snowfall_water_equiv_sum_m
                .expect("snowfall water equivalent")
                - 0.001)
                .abs()
                < 1.0e-12
        );
        assert!((row.snow_hourly_rain_m["0001"] - 0.004).abs() < 1.0e-12);
        assert!((row.snow_hourly_snowfall_depth_m["0001"] - 0.010).abs() < 1.0e-12);
        assert!((row.snow_hourly_depth_before_m["0001"] - 0.600).abs() < 1.0e-12);
        assert!((row.snow_hourly_depth_available_m["0001"] - 0.590).abs() < 1.0e-12);
        assert!((row.snow_hourly_depth_after_m["0001"] - 0.580).abs() < 1.0e-12);
        assert!((row.snow_hourly_density_before_kg_m3["0001"] - 190.0).abs() < 1.0e-12);
        assert!((row.snow_hourly_density_after_kg_m3["0001"] - 200.0).abs() < 1.0e-12);
        assert!(
            (row.snow_runtime_swe_closure_error_m
                .expect("signed S closure")
                - 0.0)
                .abs()
                < 1.0e-12
        );
        assert!((row.wb13_p_mm.expect("WB13 P") - 10.0).abs() < 1.0e-12);
        assert!((row.wb13_rm_mm.expect("WB13 RM") - 12.0).abs() < 1.0e-12);
        assert!((row.wb13_snow_water_mm.expect("WB13 Snow-Water") - 120.0).abs() < 1.0e-12);
    }
    #[test]
    fn wbval06_hillslope_wat_row_publishes_daily_interception_flux() {
        let wb13_row = SimulationOwnedWb13Row {
            wb13_row: Wb13DailyWaterBalanceRow {
                ofe: 1,
                julian_day: 42,
                year: 2,
                p: 5.0,
                rm: 4.25,
                q: 0.0,
                ep: 0.5,
                es: 0.1,
                er: 0.0,
                dp: 0.05,
                upstrmq: 0.0,
                subrin: 0.0,
                latqcc: 0.0,
                total_soil: 200.0,
                frozwt: 0.0,
                snow_water: 0.0,
                qofe: 0.0,
                tile: 0.0,
                irr: 0.0,
                area: 10_000.0,
                soil_water_total: 200.0,
                profile_depth: 1_000.0,
                profile_porosity_cap: 300.0,
                profile_fc_store: 220.0,
                profile_wp_store: 120.0,
            },
            interception_mm: 0.75,
            frdp_mm: 0.0,
            month: 2,
            day_of_month: 11,
            water_year: 2,
            sim_day_index: 407,
        };

        let wat_row =
            build_hillslope_wat_row(&wb13_row).expect("valid WB13 row should publish WAT row");

        assert_eq!(wat_row.interception, Some(0.75));
        assert_eq!(wat_row.interception_storage, None);
    }
    #[test]
    fn hphys0288_trace_row_captures_rain_on_snow_release_without_snowpack_loss() {
        let mut surface = HillslopeWritebackSurface::default();
        surface
            .flux_surface
            .insert(BoundarySymbol::from("S"), BoundaryValue::scalar(-0.001));
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_m_0001"),
            BoundaryValue::scalar(0.003),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_retained_m_0001"),
            BoundaryValue::scalar(0.001),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_released_m_0001"),
            BoundaryValue::scalar(0.002),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_m_0001"),
            BoundaryValue::scalar(0.002),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.snowfall_m_0001"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.options.newsnw"),
            BoundaryValue::scalar(100.0),
        );

        let row = build_hphys0245_trace_row(
            "H39",
            1,
            142,
            2014,
            506,
            "post_snow",
            Some("snow_coupling"),
            &surface,
            None,
            None,
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["snow_hourly_rain_released_sum_m"], 0.002);
        assert_eq!(document["snow_hourly_melt_sum_m"], 0.002);
        assert_eq!(document["snow_runtime_swe_closure_error_m"], 0.0);
    }
    #[test]
    fn hphys0270_trace_row_captures_pre_day_snowpack_state() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.120),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_depth_m"),
            BoundaryValue::scalar(0.600),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_density_kg_m3"),
            BoundaryValue::scalar(200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_settle_day_count"),
            BoundaryValue::scalar(4.0),
        );
        let snow_runtime_before = Hphys0245SnowRuntimeBeforeState {
            swe_m: Some(0.150),
            depth_m: Some(0.750),
            density_kg_m3: Some(180.0),
            settle_day_count: Some(3.0),
        };

        let row = build_hphys0245_trace_row(
            "H39",
            1,
            115,
            2013,
            115,
            "post_wb13",
            None,
            &surface,
            None,
            Some(snow_runtime_before),
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["schema"], HPHYS0245_TRACE_SCHEMA);
        assert_eq!(document["snow_runtime_swe_before_m"], 0.150);
        assert_eq!(document["snow_runtime_depth_before_m"], 0.750);
        assert_eq!(document["snow_runtime_density_before_kg_m3"], 180.0);
        assert_eq!(document["snow_runtime_settle_day_count_before"], 3.0);
        assert!(
            (document["snow_runtime_swe_delta_m"]
                .as_f64()
                .expect("SWE delta")
                + 0.030)
                .abs()
                < 1.0e-12
        );
        assert!(
            (document["snow_runtime_depth_delta_m"]
                .as_f64()
                .expect("depth delta")
                + 0.150)
                .abs()
                < 1.0e-12
        );
        assert_eq!(document["snow_runtime_density_delta_kg_m3"], 20.0);
        assert_eq!(document["snow_runtime_settle_day_count_delta"], 1.0);
    }
    #[test]
    #[allow(clippy::too_many_lines)]
    fn hphys0271_trace_row_captures_melt_term_hourly_forcing_maps() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_raw_m_0001"),
            BoundaryValue::scalar(0.0254),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_m_0001"),
            BoundaryValue::scalar(0.0200),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_m_0001"),
            BoundaryValue::scalar(0.001),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.snowfall_m_0001"),
            BoundaryValue::scalar(0.004),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.depth_before_m_0001"),
            BoundaryValue::scalar(0.420),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.depth_available_m_0001"),
            BoundaryValue::scalar(0.415),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.depth_after_m_0001"),
            BoundaryValue::scalar(0.400),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.density_before_kg_m3_0001"),
            BoundaryValue::scalar(330.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.density_after_kg_m3_0001"),
            BoundaryValue::scalar(350.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_amelt_in_0001"),
            BoundaryValue::scalar(0.10),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_bmelt_in_0001"),
            BoundaryValue::scalar(0.20),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_cmelt_in_0001"),
            BoundaryValue::scalar(0.30),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_dmelt_in_0001"),
            BoundaryValue::scalar(0.40),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_hrtef_f_0001"),
            BoundaryValue::scalar(36.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_hrdtf_f_0001"),
            BoundaryValue::scalar(30.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_vwmph_0001"),
            BoundaryValue::scalar(4.47),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_rainin_0001"),
            BoundaryValue::scalar(0.02),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_wind_adjustment_0001"),
            BoundaryValue::scalar(1.07),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_branch_active_0001"),
            BoundaryValue::scalar(1.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("winter.hourly.air_temp_c_0001"),
            BoundaryValue::scalar(2.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("winter.hourly.rad_mj_m2_0001"),
            BoundaryValue::scalar(1.25),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("winter.hourly.cloud_fraction_0001"),
            BoundaryValue::scalar(0.5),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("winter.hourly.dewpoint_c_0001"),
            BoundaryValue::scalar(-1.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("winter.hourly.wind_m_s_0001"),
            BoundaryValue::scalar(2.0),
        );

        let row = build_hphys0245_trace_row(
            "H1",
            1,
            36,
            2013,
            36,
            "post_wb13",
            None,
            &surface,
            None,
            None,
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["schema"], HPHYS0245_TRACE_SCHEMA);
        assert_eq!(document["snow_hourly_rain_m"]["0001"], 0.001);
        assert_eq!(document["snow_hourly_snowfall_depth_m"]["0001"], 0.004);
        assert_eq!(document["snow_hourly_depth_before_m"]["0001"], 0.420);
        assert_eq!(document["snow_hourly_depth_available_m"]["0001"], 0.415);
        assert_eq!(document["snow_hourly_depth_after_m"]["0001"], 0.400);
        assert_eq!(document["snow_hourly_density_before_kg_m3"]["0001"], 330.0);
        assert_eq!(document["snow_hourly_density_after_kg_m3"]["0001"], 350.0);
        assert_eq!(document["snow_hourly_melt_raw_m"]["0001"], 0.0254);
        assert_eq!(document["snow_hourly_melt_m"]["0001"], 0.0200);
        assert_eq!(document["snow_hourly_melt_amelt_in"]["0001"], 0.10);
        assert_eq!(document["snow_hourly_melt_bmelt_in"]["0001"], 0.20);
        assert_eq!(document["snow_hourly_melt_cmelt_in"]["0001"], 0.30);
        assert_eq!(document["snow_hourly_melt_dmelt_in"]["0001"], 0.40);
        assert_eq!(document["snow_hourly_melt_hrtef_f"]["0001"], 36.0);
        assert_eq!(document["snow_hourly_melt_hrdtf_f"]["0001"], 30.0);
        assert_eq!(document["snow_hourly_melt_vwmph"]["0001"], 4.47);
        assert_eq!(document["snow_hourly_melt_rainin"]["0001"], 0.02);
        assert_eq!(document["snow_hourly_melt_wind_adjustment"]["0001"], 1.07);
        assert_eq!(document["snow_hourly_melt_branch_active"]["0001"], 1.0);
        assert_eq!(document["winter_hourly_air_temp_c"]["0001"], 2.0);
        assert_eq!(document["winter_hourly_rad_mj_m2"]["0001"], 1.25);
        assert_eq!(document["winter_hourly_cloud_fraction"]["0001"], 0.5);
        assert_eq!(document["winter_hourly_dewpoint_c"]["0001"], -1.0);
        assert_eq!(document["winter_hourly_wind_m_s"]["0001"], 2.0);
    }
    #[test]
    fn hphys0318_trace_row_captures_stmtim_control_surfaces() {
        let mut surface = HillslopeWritebackSurface::default();
        for (symbol, value) in [
            ("snow.hourly.stmtim.rain_m_0011", 0.0024),
            ("snow.hourly.stmtim.stmdur_s_0011", 10_800.0),
            ("snow.hourly.stmtim.wntdur_h_0011", 3.0),
            ("snow.hourly.stmtim.wnttim_h_0011", 10.0),
            ("snow.hourly.stmtim.hrtemp_c_0011", -1.5),
            ("snow.hourly.stmtim.rst_c_0011", 0.0),
            ("snow.hourly.stmtim.hrrain_m_0011", 0.0),
            ("snow.hourly.stmtim.hrsnow_m_0011", 0.008),
            ("snow.hourly.stmtim.active_interval_0011", 1.0),
            ("snow.hourly.stmtim.rain_branch_0011", 0.0),
            ("snow.hourly.stmtim.snow_branch_0011", 1.0),
        ] {
            surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }

        let row = build_hphys0245_trace_row(
            "H1",
            1,
            11,
            2013,
            11,
            "post_simimpl28",
            None,
            &surface,
            None,
            None,
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["schema"], HPHYS0245_TRACE_SCHEMA);
        assert_eq!(document["snow_hourly_stmtim_rain_m"]["0011"], 0.0024);
        assert_eq!(document["snow_hourly_stmtim_stmdur_s"]["0011"], 10_800.0);
        assert_eq!(document["snow_hourly_stmtim_wntdur_h"]["0011"], 3.0);
        assert_eq!(document["snow_hourly_stmtim_wnttim_h"]["0011"], 10.0);
        assert_eq!(document["snow_hourly_stmtim_hrtemp_c"]["0011"], -1.5);
        assert_eq!(document["snow_hourly_stmtim_rst_c"]["0011"], 0.0);
        assert_eq!(document["snow_hourly_stmtim_hrrain_m"]["0011"], 0.0);
        assert_eq!(document["snow_hourly_stmtim_hrsnow_m"]["0011"], 0.008);
        assert_eq!(document["snow_hourly_stmtim_active_interval"]["0011"], 1.0);
        assert_eq!(document["snow_hourly_stmtim_rain_branch"]["0011"], 0.0);
        assert_eq!(document["snow_hourly_stmtim_snow_branch"]["0011"], 1.0);
    }
