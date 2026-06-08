#[cfg(test)]
mod tests {
    use super::*;
    use crate::SidecarPolicy;
    use openwepp_input_contract::parsers::hbp::{HbpParseOptions, parse_hbp_from_path};
    use openwepp_input_contract::parsers::slope::{
        DatverSource, DistanceMode, SlopeOfe, SlopePoint, SlopeProfile,
    };
    use openwepp_kernel_contract::{
        HillslopeConsumerAdapter, HillslopeKernel, HillslopeKernelPhaseClass,
        HillslopeKernelRequest, WritebackField,
    };
    use std::fs;
    use std::path::Path;
    use std::sync::{Mutex, OnceLock};

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
        assert_json_i64(&manifest_json, "/wb13_publication/first_row_key/year", 1);
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
        let source = include_str!("mod.rs");
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

    fn wb11_seed_test_surface(symbols: &[(&str, f64)]) -> HillslopeWritebackSurface {
        let mut runtime_surface = HillslopeWritebackSurface::default();
        for (symbol, value) in symbols {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(*symbol), BoundaryValue::scalar(*value));
        }
        runtime_surface
    }

    fn state_field_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
        fields
            .iter()
            .find(|field| field.symbol.as_str() == symbol)
            .map(|field| field.value.as_f64())
    }

    fn flux_field_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
        fields
            .iter()
            .find(|field| field.symbol.as_str() == symbol)
            .map(|field| field.value.as_f64())
    }

    fn insert_wb11_primary_layer_lineage_symbols(
        runtime_surface: &mut HillslopeWritebackSurface,
        sat: f64,
        include_cpm: bool,
    ) {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.25));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("wb11_nsl"), BoundaryValue::scalar(1.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("wb19_nsl"), BoundaryValue::scalar(1.0));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_dg_0001"),
            BoundaryValue::scalar(0.25),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("por_0001"),
            BoundaryValue::scalar(0.45),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_por_0001"),
            BoundaryValue::scalar(0.45),
        );
        if include_cpm {
            runtime_surface.state_surface.insert(
                BoundarySymbol::from("cpm_0001"),
                BoundaryValue::scalar(0.90),
            );
        }
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("sat"), BoundaryValue::scalar(sat));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(0.30),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_thetfc_0001"),
            BoundaryValue::scalar(0.30),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetdr_0001"),
            BoundaryValue::scalar(0.12),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_thetdr_0001"),
            BoundaryValue::scalar(0.12),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("ssc_0001"),
            BoundaryValue::scalar(2.0e-6),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
            BoundaryValue::scalar(1.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_drain_enabled"),
            BoundaryValue::scalar(0.0),
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

    #[derive(Clone, Copy)]
    struct Wb16OfeSeedVector {
        avgslp: f64,
        slplen: f64,
        inrcov: f64,
        rilcov: f64,
        rrinit: f64,
        rspace: f64,
        width: f64,
        rtyp: f64,
        cancov: f64,
        canhgt: f64,
        bb_seed: f64,
        bbb_seed: f64,
        flivmx_seed: f64,
        hmax_seed: f64,
    }

    fn insert_wb16_ofe_projection_symbols(
        runtime_surface: &mut HillslopeWritebackSurface,
        ofe_index: usize,
        seed: Wb16OfeSeedVector,
    ) {
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_avgslp")),
            BoundaryValue::scalar(seed.avgslp),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_slplen")),
            BoundaryValue::scalar(seed.slplen),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_inrcov")),
            BoundaryValue::scalar(seed.inrcov),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rilcov")),
            BoundaryValue::scalar(seed.rilcov),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rrinit")),
            BoundaryValue::scalar(seed.rrinit),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rspace")),
            BoundaryValue::scalar(seed.rspace),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_width")),
            BoundaryValue::scalar(seed.width),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rtyp")),
            BoundaryValue::scalar(seed.rtyp),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_cancov")),
            BoundaryValue::scalar(seed.cancov),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_canhgt")),
            BoundaryValue::scalar(seed.canhgt),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_bb_seed")),
            BoundaryValue::scalar(seed.bb_seed),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_bbb_seed")),
            BoundaryValue::scalar(seed.bbb_seed),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_flivmx_seed")),
            BoundaryValue::scalar(seed.flivmx_seed),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_hmax_seed")),
            BoundaryValue::scalar(seed.hmax_seed),
        );
    }

    #[allow(clippy::too_many_arguments, clippy::similar_names)]
    fn wb16_expected_frcteq(
        inrcov: f64,
        rilcov: f64,
        rrinit: f64,
        rspace: f64,
        width: f64,
        flivmx_seed: f64,
        canhgt: f64,
        hmax_seed: f64,
    ) -> f64 {
        let inrfo = (3.024 - 5.042 * (-161.0 * rrinit).exp()).exp();
        let mut inrrou = 0.5 * inrfo.powf(1.128);
        if inrrou < WB16_INRFSO_CROPLAND {
            inrrou = WB16_INRFSO_CROPLAND;
        }
        let inrfro = inrrou - WB16_INRFSO_CROPLAND;
        let inrfco = if inrcov > 0.0 {
            14.5 * inrcov.powf(1.5544)
        } else {
            0.0
        };
        let frlive = if hmax_seed > 0.0 {
            (canhgt / hmax_seed) * flivmx_seed
        } else {
            0.0
        };
        let inrfto = inrfro + inrfco + WB16_INRFSO_CROPLAND + frlive;
        let frccov = if rilcov > 0.0 {
            4.5 * rilcov.powf(1.5544)
        } else {
            0.0
        };
        let frctrl = frccov + frlive + WB16_FRCSOL_CROPLAND;
        let width_ratio = width / rspace;
        if width_ratio < 1.0 {
            inrfto + width_ratio * (frctrl - inrfto)
        } else {
            inrfto
        }
    }

    fn wb16_expected_multiofe_ealpha(slplens: [f64; 2], alphas: [f64; 2], m: f64) -> f64 {
        let power2 = 1.0 / m;
        let power3 = power2 + 1.0;
        let sum_length = slplens.iter().sum::<f64>();
        let mut cumulative_length = 0.0;
        let mut storage_integral = 0.0;
        let mut last_power = 0.0;
        for (slope_length, alpha_value) in slplens.into_iter().zip(alphas) {
            cumulative_length += slope_length;
            let current_power = cumulative_length.powf(power3);
            storage_integral += (current_power - last_power) / alpha_value.powf(power2);
            last_power = current_power;
        }
        (sum_length / storage_integral).powf(m) * sum_length
    }

    #[allow(clippy::too_many_lines)]
    fn seeded_wb13_runtime_surface_probe() -> HillslopeWritebackSurface {
        let mut runtime_surface = HillslopeWritebackSurface::default();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.004));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(12.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(2.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.25));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.25));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(0.30),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetdr_0001"),
            BoundaryValue::scalar(0.12),
        );

        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_depth_mm"),
            BoundaryValue::scalar(250.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_porosity_cap_mm"),
            BoundaryValue::scalar(120.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_fc_store_mm"),
            BoundaryValue::scalar(75.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_fc_tail_mm"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_wp_store_mm"),
            BoundaryValue::scalar(30.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.075),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_ws_frz"),
            BoundaryValue::scalar(0.0),
        );
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
            BoundaryValue::scalar(0.0),
        );
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("S"), BoundaryValue::scalar(0.0));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("I"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Irr"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.000_20));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Es"), BoundaryValue::scalar(0.000_10));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Er"), BoundaryValue::scalar(0.000_05));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.000_10));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qdd"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("SubRIn"), BoundaryValue::scalar(0.0));
        runtime_surface
    }

    fn canonical_calendar_day_probe() -> ClimateDayProjection {
        ClimateDayProjection {
            year: 2000,
            month: 1,
            day_of_month: 1,
            julian_day: 1,
            precipitation_mm: 4.0,
        }
    }

    fn execute_fixture_run(prefix: &str) -> (HillslopeRunReport, PathBuf) {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);
        let output_dir = temp_run_dir.join("output");

        let report = execute_hillslope_run(
            &HillslopeRunRequest {
                run_dir: temp_run_dir.clone(),
                run_file: PathBuf::from("case.run"),
                output_dir,
                sidecar_policy: SidecarPolicy::Compat,
                legacy_sidecar_discovery: false,
                manifest_path: None,
            },
            &["openwepp-cli-hill".to_string()],
        )
        .expect("fixture run should complete");

        (report, temp_run_dir)
    }

    fn runner_execution_lock() -> &'static Mutex<()> {
        static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        RUN_LOCK.get_or_init(|| Mutex::new(()))
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

        let surface = build_hillslope_runtime_surface_from_pmetpara(
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
