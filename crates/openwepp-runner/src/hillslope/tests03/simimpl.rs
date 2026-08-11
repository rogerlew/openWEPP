use super::*;
use crate::hillslope::intake_lane_setup::build_execution_lane_context;

    #[test]
    fn simimpl09_timestep_policy_scaffolds_subhourly_without_physics_enablement() {
        let policy = TimestepPolicy::scaffold_subhourly(900);
        assert_eq!(policy.policy_name(), SUBHOURLY_EXECUTION_LANE);
        assert_eq!(policy.timestep_seconds(), 900);
        assert!(!policy.physics_enabled());
    }
    #[test]
    fn simimpl09_lane_context_matches_mode_selection_tuple() {
        let mode_selection = HillslopeModeSelectionProvenance {
            wepp_ui: WeppUiModeSelectionProvenance {
                requested: 1,
                effective: 1,
                selected_lane: HOURLY_EXECUTION_LANE.to_string(),
                mode_divergence: false,
                guard_id: WUI_MODE_GUARD_ID.to_string(),
            },
        };
        let lane_context = build_execution_lane_context(&mode_selection)
            .expect("hourly mode-selection tuple should map to hourly lane context");
        assert_eq!(lane_context.lane, ExecutionLane::Hourly);
        assert_eq!(lane_context.requested_mode, HOURLY_EXECUTION_LANE);
        assert_eq!(lane_context.effective_mode, HOURLY_EXECUTION_LANE);
        assert_eq!(
            lane_context.timestep_policy.timestep_seconds(),
            HOURLY_TIMESTEP_SECONDS
        );
        assert!(lane_context.timestep_policy.physics_enabled());
    }
    #[test]
    fn simimpl11_area_derives_from_aggregate_ofe_geometry() {
        let slope = SlopeProfile {
            datver: 2023.3,
            datver_source: DatverSource::Header,
            ofe_count: 2,
            ofes: vec![
                SlopeOfe {
                    index: 0,
                    azm: 180.0,
                    fwidth: 30.0,
                    elevation: None,
                    nslpts: 2,
                    slplen: 60.0,
                    distance_mode: DistanceMode::Normalized,
                    points: vec![
                        SlopePoint {
                            xinput: 0.0,
                            slpinp: 0.02,
                        },
                        SlopePoint {
                            xinput: 1.0,
                            slpinp: 0.06,
                        },
                    ],
                },
                SlopeOfe {
                    index: 1,
                    azm: 180.0,
                    fwidth: 30.0,
                    elevation: None,
                    nslpts: 2,
                    slplen: 40.0,
                    distance_mode: DistanceMode::Normalized,
                    points: vec![
                        SlopePoint {
                            xinput: 0.0,
                            slpinp: 0.06,
                        },
                        SlopePoint {
                            xinput: 1.0,
                            slpinp: 0.03,
                        },
                    ],
                },
            ],
        };

        let observed = derive_mofe04_publication_area_from_slope(&slope)
            .expect("valid aggregate OFE geometry should yield area");
        assert!((observed - 3_000.0).abs() < 1.0e-12);
    }
    #[test]
    fn simimpl14_contract_gate_continuous_wb13_span_and_keys() {
        let (report, _temp_run_dir) = execute_fixture_run("simimpl14_contract_span");
        let pass_parse = parse_hbp_from_path(&report.output_pass, HbpParseOptions::strict())
            .unwrap_or_else(|error| {
                panic!(
                    "pass output should be parseable binary HBP at {}: {error}",
                    report.output_pass.display()
                )
            });
        assert!(pass_parse.record_count >= 1);
        assert!(pass_parse.warnings.is_empty());

        let manifest_json = read_manifest_json(&report);
        assert_json_i64(&manifest_json, "/execution_provenance/climate_day_count", 2);
        assert_json_i64(
            &manifest_json,
            "/execution_provenance/executed_day_count",
            2,
        );
        assert_json_i64(&manifest_json, "/wb13_publication/row_count", 2);
        assert_json_i64(&manifest_json, "/wb13_publication/first_row_key/year", 2000);
        assert_eq!(
            manifest_json
                .pointer("/wat5_output/enabled")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
        assert_eq!(
            manifest_json
                .pointer("/wat5_output/selector")
                .and_then(serde_json::Value::as_str),
            Some("run_file.outputs.wat_subhourly_presence")
        );
        assert!(
            manifest_json
                .pointer("/wat5_output/output_path")
                .is_some_and(serde_json::Value::is_null)
        );
        let monotonic = manifest_json
            .pointer("/wb13_publication/sim_day_index_monotonic")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or_else(|| {
                panic!("missing bool JSON pointer /wb13_publication/sim_day_index_monotonic")
            });
        assert!(monotonic, "sim_day_index must be monotonic");
    }
    #[test]
    fn simimpl14_contract_gate_loss_output_is_run_span_truthful() {
        let (report, _temp_run_dir) = execute_fixture_run("simimpl14_contract_loss");
        let loss_text = fs::read_to_string(&report.output_loss).unwrap_or_else(|error| {
            panic!(
                "loss output should be readable at {}: {error}",
                report.output_loss.display()
            )
        });
        let loss_json: serde_json::Value =
            serde_json::from_str(&loss_text).expect("loss output should parse as JSON");

        assert_json_i64(&loss_json, "/climate_day_count", 2);
        assert_json_i64(&loss_json, "/executed_day_count", 2);
        assert_json_i64(&loss_json, "/first_day_julian", 1);
        assert_json_i64(&loss_json, "/last_day_julian", 2);
    }
