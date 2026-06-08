    #[test]
    fn climate_runtime_surface_contains_canonical_daily_symbols() {
        let climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("climate runtime surface should build");

        let datver = surface
            .state_surface
            .get(&BoundarySymbol::from("datver"))
            .expect("datver should exist")
            .as_f64();
        let iclig = surface
            .state_surface
            .get(&BoundarySymbol::from("iclig"))
            .expect("iclig should exist")
            .as_f64();
        let prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("prcp"))
            .expect("prcp should exist")
            .as_f64();
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("stmdur should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist")
            .as_f64();
        let ninten = surface
            .state_surface
            .get(&BoundarySymbol::from("ninten"))
            .expect("ninten should exist")
            .as_f64();
        let timem_first = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0001"))
            .expect("timem_0001 should exist")
            .as_f64();
        let intsty_first = surface
            .state_surface
            .get(&BoundarySymbol::from("intsty_0001"))
            .expect("intsty_0001 should exist")
            .as_f64();
        let obmaxt_0001 = surface
            .state_surface
            .get(&BoundarySymbol::from("obmaxt_0001"))
            .expect("obmaxt_0001 should exist")
            .as_f64();
        let obmint_0012 = surface
            .state_surface
            .get(&BoundarySymbol::from("obmint_0012"))
            .expect("obmint_0012 should exist")
            .as_f64();

        assert!((datver - 5.3).abs() < 1e-12);
        assert!((iclig - 1.0).abs() < 1e-12);
        assert!((prcp - 0.01).abs() < 1e-12);
        assert!((stmdur - 7_200.0).abs() < 1e-12);
        assert!((ip - 2.1).abs() < 1e-12);
        assert!(ninten >= 2.0);
        assert!(timem_first.abs() < 1e-12);
        assert!(intsty_first.is_finite());
        assert!((obmaxt_0001 - 1.0).abs() < 1e-12);
        assert!((obmint_0012 - 6.0).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_with_context_emits_simimpl28_hourly_forcing_symbols() {
        let climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        let context = simimpl28_winter_context(0.0);
        let surface =
            build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &context)
                .expect("contextual climate runtime surface should build");

        let mut rain_total = 0.0;
        let mut snow_total = 0.0;
        for hour in 1..=24 {
            let rad = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "winter.hourly.rad_mj_m2_{hour:04}"
                )))
                .expect("hourly winter radiation symbol should exist")
                .as_f64();
            let temp = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "winter.hourly.air_temp_c_{hour:04}"
                )))
                .expect("hourly winter air temperature symbol should exist")
                .as_f64();
            let cloud = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "winter.hourly.cloud_fraction_{hour:04}"
                )))
                .expect("hourly winter cloud symbol should exist")
                .as_f64();
            let rain = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.rain_m_{hour:04}"
                )))
                .expect("hourly rain symbol should exist")
                .as_f64();
            let snow = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.snowfall_m_{hour:04}"
                )))
                .expect("hourly snowfall symbol should exist")
                .as_f64();
            let stmtim_rain = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.rain_m_{hour:04}"
                )))
                .expect("stmtim rain control symbol should exist")
                .as_f64();
            let stmtim_hrrain = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.hrrain_m_{hour:04}"
                )))
                .expect("stmtim hrrain output symbol should exist")
                .as_f64();
            let stmtim_hrsnow = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.hrsnow_m_{hour:04}"
                )))
                .expect("stmtim hrsnow output symbol should exist")
                .as_f64();

            assert!(rad.is_finite());
            assert!(temp.is_finite());
            assert!((0.0..=1.0).contains(&cloud));
            assert!(stmtim_rain >= 0.0);
            assert!((stmtim_hrrain - rain).abs() < 1.0e-12);
            assert!((stmtim_hrsnow - snow).abs() < 1.0e-12);
            assert!(rain >= 0.0);
            assert!(snow >= 0.0);
            rain_total += rain;
            snow_total += snow;
        }

        assert!(rain_total > 0.0 || snow_total > 0.0);
    }

    #[test]
    fn climate_runtime_surface_with_context_uses_single_radly_to_radmj_conversion() {
        let climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        let context = simimpl28_winter_context(0.0);
        let surface =
            build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &context)
                .expect("contextual climate runtime surface should build");
        let daily_radmj = 200.0 * 0.04184;
        let hourly_radiation = (1..=24)
            .map(|hour| {
                surface
                    .state_surface
                    .get(&BoundarySymbol::from(format!(
                        "winter.hourly.rad_mj_m2_{hour:04}"
                    )))
                    .expect("hourly winter radiation symbol should exist")
                    .as_f64()
            })
            .collect::<Vec<_>>();
        let max_hourly_radiation = hourly_radiation.iter().copied().fold(0.0, f64::max);
        let total_hourly_radiation = hourly_radiation.iter().sum::<f64>();

        assert!(hourly_radiation.iter().all(|value| value.is_finite()));
        assert!(hourly_radiation.iter().all(|value| *value >= 0.0));
        assert!(
            max_hourly_radiation < daily_radmj,
            "hourly radiation must be MJ-scale; max={max_hourly_radiation}, daily_radmj={daily_radmj}"
        );
        assert!(
            total_hourly_radiation < daily_radmj * 1.25,
            "hourly radiation sum must remain proportional to single-converted daily radiation; sum={total_hourly_radiation}, daily_radmj={daily_radmj}"
        );
    }

    #[test]
    fn climate_runtime_surface_with_context_near_isothermal_radiation_is_radmj_over_24() {
        let near_isothermal_climate = VALID_CLIMATE.replace("12.0 2.0 200.0", "1.0 0.5 200.0");
        let climate = parse_climate_from_str(&near_isothermal_climate, ClimateParserMode::Strict)
            .expect("near-isothermal climate fixture should parse");
        let context = simimpl28_winter_context(0.0);
        let surface =
            build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &context)
                .expect("near-isothermal contextual climate runtime surface should build");
        let expected_hourly_radmj = (200.0 * 0.04184) / 24.0;

        for hour in 1..=24 {
            let hrrad_mj_m2 = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "winter.hourly.rad_mj_m2_{hour:04}"
                )))
                .expect("hourly winter radiation symbol should exist")
                .as_f64();
            assert!(
                (hrrad_mj_m2 - expected_hourly_radmj).abs() < 1e-12,
                "hour {hour} expected {expected_hourly_radmj}, got {hrrad_mj_m2}"
            );
        }
    }

    #[test]
    fn wbval02_rejects_daily_radiation_above_baseline_sunmap_potential() {
        let wbval02_climate = VALID_CLIMATE
            .replace(
                "45.0 -120.0 1000.0 30 2000 1 CLIGEN 5.30 --seed 123",
                "43.73 -111.12 1859.0 40 1990 100 CLIGEN 5.32300 --seed 0",
            )
            .replace(
                "1 1 2000 10.0 2.0 0.25 3.0 12.0 2.0 200.0 3.0 180.0 -1.0",
                "18 2 1990 0.0 0.0 0.0 0.0 -5.0 -20.6 486.0 2.9 347.0 -20.8",
            )
            .replace(
                "2 1 2000 0.0 0.0 0.0 0.0 10.0 1.0 190.0 2.5 170.0 -2.0",
                "19 2 1990 0.0 0.0 0.0 0.0 1.1 -19.4 503.0 2.4 59.0 -19.3",
            );
        let climate = parse_climate_from_str(&wbval02_climate, ClimateParserMode::Strict)
            .expect("WBVAL02 source-radiation fixture should parse");
        let context = simimpl28_winter_context(0.0);
        let error =
            build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &context)
                .expect_err("daily source radiation above sunmap potential should fail closed");

        match error {
            ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                symbol,
                value,
                allowed,
            } => {
                assert_eq!(symbol, "radly");
                assert!((value - 486.0).abs() < 1e-12);
                assert!(
                    allowed.contains("baseline sunmap horizontal daily potential"),
                    "unexpected allowed domain {allowed}"
                );
            }
            other => panic!("expected daily source-radiation guard error, got {other:?}"),
        }
    }

    #[test]
    fn climate_runtime_surface_with_context_rejects_physically_impossible_radiation() {
        let high_radiation_climate =
            VALID_CLIMATE.replace("12.0 2.0 200.0", "12.0 2.0 6000.0");
        let climate = parse_climate_from_str(&high_radiation_climate, ClimateParserMode::Strict)
            .expect("high finite radiation climate fixture should parse");
        let context = simimpl28_winter_context(0.0);
        let error =
            build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &context)
                .expect_err("physically impossible hourly radiation should fail closed");

        match error {
            ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                symbol,
                value,
                allowed,
            } => {
                assert_eq!(symbol, "radly");
                assert!((value - 6000.0).abs() < 1e-12);
                assert!(
                    allowed.contains("baseline sunmap horizontal daily potential"),
                    "unexpected allowed domain {allowed}"
                );
            }
            other => panic!("expected physical radiation guard error, got {other:?}"),
        }
    }

    #[test]
    fn climate_runtime_surface_with_context_respects_rst_partition_branches() {
        let climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");

        let warm_context = simimpl28_winter_context(-100.0);
        let cold_context = simimpl28_winter_context(100.0);
        let warm_surface =
            build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &warm_context)
                .expect("warm-context climate surface should build");
        let cold_surface =
            build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &cold_context)
                .expect("cold-context climate surface should build");

        let warm_rain = (1..=24)
            .map(|hour| {
                warm_surface
                    .state_surface
                    .get(&BoundarySymbol::from(format!(
                        "snow.hourly.rain_m_{hour:04}"
                    )))
                    .expect("hourly rain symbol should exist")
                    .as_f64()
            })
            .sum::<f64>();
        let warm_snow = (1..=24)
            .map(|hour| {
                warm_surface
                    .state_surface
                    .get(&BoundarySymbol::from(format!(
                        "snow.hourly.snowfall_m_{hour:04}"
                    )))
                    .expect("hourly snowfall symbol should exist")
                    .as_f64()
            })
            .sum::<f64>();
        let cold_rain = (1..=24)
            .map(|hour| {
                cold_surface
                    .state_surface
                    .get(&BoundarySymbol::from(format!(
                        "snow.hourly.rain_m_{hour:04}"
                    )))
                    .expect("hourly rain symbol should exist")
                    .as_f64()
            })
            .sum::<f64>();
        let cold_snow = (1..=24)
            .map(|hour| {
                cold_surface
                    .state_surface
                    .get(&BoundarySymbol::from(format!(
                        "snow.hourly.snowfall_m_{hour:04}"
                    )))
                    .expect("hourly snowfall symbol should exist")
                    .as_f64()
            })
            .sum::<f64>();

        assert!(warm_rain > 0.0);
        assert!(warm_snow.abs() < 1e-12);
        assert!(cold_snow > 0.0);
        assert!(cold_rain.abs() < 1e-12);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn hphys0318_stmtim_control_surfaces_publish_branch_inputs_and_outputs() {
        let climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        let context = simimpl28_winter_context(100.0);
        let surface =
            build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &context)
                .expect("cold-context climate surface should build");
        let prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("prcp"))
            .expect("daily prcp should exist")
            .as_f64();
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("daily stmdur should exist")
            .as_f64();

        let mut active_count = 0;
        let mut snowfall_total = 0.0;
        let mut stmtim_hrsnow_total = 0.0;
        for hour in 1..=24 {
            let suffix = format!("{hour:04}");
            let hourly_rain = surface
                .state_surface
                .get(&BoundarySymbol::from(format!("snow.hourly.rain_m_{suffix}")))
                .expect("hourly rain should exist")
                .as_f64();
            let hourly_snow = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.snowfall_m_{suffix}"
                )))
                .expect("hourly snowfall should exist")
                .as_f64();
            let hourly_temp = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "winter.hourly.air_temp_c_{suffix}"
                )))
                .expect("hourly air temperature should exist")
                .as_f64();
            let stmtim_rain = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.rain_m_{suffix}"
                )))
                .expect("stmtim rain should exist")
                .as_f64();
            let stmtim_stmdur = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.stmdur_s_{suffix}"
                )))
                .expect("stmtim stmdur should exist")
                .as_f64();
            let stmtim_hrtemp = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.hrtemp_c_{suffix}"
                )))
                .expect("stmtim hrtemp should exist")
                .as_f64();
            let stmtim_rst = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.rst_c_{suffix}"
                )))
                .expect("stmtim rst should exist")
                .as_f64();
            let stmtim_hrrain = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.hrrain_m_{suffix}"
                )))
                .expect("stmtim hrrain should exist")
                .as_f64();
            let stmtim_hrsnow = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.hrsnow_m_{suffix}"
                )))
                .expect("stmtim hrsnow should exist")
                .as_f64();
            let active_interval = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.active_interval_{suffix}"
                )))
                .expect("stmtim active flag should exist")
                .as_f64();
            let rain_branch = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.rain_branch_{suffix}"
                )))
                .expect("stmtim rain branch flag should exist")
                .as_f64();
            let snow_branch = surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.stmtim.snow_branch_{suffix}"
                )))
                .expect("stmtim snow branch flag should exist")
                .as_f64();

            assert!((stmtim_rain - prcp).abs() < 1.0e-12);
            assert!((stmtim_stmdur - stmdur).abs() < 1.0e-12);
            assert!((stmtim_hrtemp - hourly_temp).abs() < 1.0e-12);
            assert!((stmtim_rst - 100.0).abs() < 1.0e-12);
            assert!((stmtim_hrrain - hourly_rain).abs() < 1.0e-12);
            assert!((stmtim_hrsnow - hourly_snow).abs() < 1.0e-12);
            let active_interval_is_one = (active_interval - 1.0).abs() < 1.0e-12;
            let snow_branch_is_one = (snow_branch - 1.0).abs() < 1.0e-12;
            assert!(active_interval.abs() < 1.0e-12 || active_interval_is_one);
            assert!(rain_branch.abs() < 1.0e-12);
            assert!(snow_branch.abs() < 1.0e-12 || snow_branch_is_one);

            if active_interval_is_one {
                active_count += 1;
                assert!(snow_branch_is_one);
                assert!(stmtim_hrsnow > 0.0);
            } else {
                assert!(snow_branch.abs() < 1.0e-12);
                assert!(stmtim_hrrain.abs() < 1.0e-12);
                assert!(stmtim_hrsnow.abs() < 1.0e-12);
            }

            snowfall_total += hourly_snow;
            stmtim_hrsnow_total += stmtim_hrsnow;
        }

        assert!(active_count > 0);
        assert!((stmtim_hrsnow_total - snowfall_total).abs() < 1.0e-12);
    }

    #[test]
    fn hphys0320_stmtim_normalizes_zero_start_before_active_interval() {
        let partition = simimpl28_stmtim_hourly_partition(
            0.00082, 38_040.0, 11.0, 0.0, 0.0, -11.594,
        )
        .expect("HPHYS0320 finite breakpoint start should normalize");

        assert!((partition.wntdur_h - 11.0).abs() < 1.0e-12);
        assert!((partition.wnttim_h - 1.0).abs() < 1.0e-12);
        assert!(partition.active_interval);
        assert!(!partition.rain_branch);
        assert!(partition.snow_branch);
        assert!(partition.hrrain_m.abs() < 1.0e-12);
        assert!((partition.hrsnow_m - 0.000_745_454_545_454_545_5).abs() < 1.0e-12);
    }

    #[test]
    fn hphys0320_stmtim_nonfinite_start_time_fails_closed() {
        let error = simimpl28_stmtim_hourly_partition(
            0.00082,
            38_040.0,
            11.0,
            f64::NAN,
            0.0,
            -11.594,
        )
        .expect_err("non-finite start time must fail closed");

        match error {
            ClimateRuntimeInputError::NonFiniteField { field, value } => {
                assert_eq!(field, "wnttim");
                assert!(value.is_nan());
            }
            other => panic!("expected wnttim non-finite error, got {other:?}"),
        }
    }

    #[test]
    fn climate_runtime_surface_with_context_uses_cold_trigger_without_snow_sidecar() {
        let cold_climate = VALID_CLIMATE.replace("12.0 2.0 200.0", "-1.0 -3.0 200.0");
        let climate = parse_climate_from_str(&cold_climate, ClimateParserMode::Strict)
            .expect("strict cold climate fixture should parse");
        let mut context = simimpl28_winter_context(0.0);
        context.insert(
            BoundarySymbol::from("snow.options.snow_file_present"),
            BoundaryValue::scalar(0.0),
        );
        context.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.0),
        );

        let surface =
            build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &context)
                .expect("cold-trigger winter surface should build without snow sidecar");
        let snow_total = (1..=24)
            .map(|hour| {
                surface
                    .state_surface
                    .get(&BoundarySymbol::from(format!(
                        "snow.hourly.snowfall_m_{hour:04}"
                    )))
                    .expect("hourly snowfall symbol should exist")
                    .as_f64()
            })
            .sum::<f64>();

        assert!(snow_total > 0.0);
    }

    #[test]
    fn climate_runtime_surface_with_context_rejects_missing_required_winter_symbol() {
        let climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        let mut context = simimpl28_winter_context(0.0);
        context.remove(&BoundarySymbol::from("azm"));

        let error =
            build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &context)
                .expect_err("missing azm under active winter synthesis must fail");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-016");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::MissingRuntimeContextSymbol { symbol } if symbol == "azm"
        ));
    }

    #[test]
    fn breakpoint_runtime_surface_projects_stmstr_elapsed_timem_and_mxint() {
        let climate =
            parse_climate_from_str(WC1_BREAKPOINT_STMSTR_NONZERO, ClimateParserMode::Strict)
                .expect("curated wc1 breakpoint fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("breakpoint runtime surface should build");

        let stmstr = surface
            .state_surface
            .get(&BoundarySymbol::from("stmstr"))
            .expect("stmstr should exist")
            .as_f64();
        let prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("prcp"))
            .expect("prcp should exist")
            .as_f64();
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("stmdur should exist")
            .as_f64();
        let mxint = surface
            .state_surface
            .get(&BoundarySymbol::from("mxint"))
            .expect("mxint should exist")
            .as_f64();
        let timem_1 = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0001"))
            .expect("timem_0001 should exist")
            .as_f64();
        let timem_2 = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0002"))
            .expect("timem_0002 should exist")
            .as_f64();
        let intsty_5 = surface
            .state_surface
            .get(&BoundarySymbol::from("intsty_0005"))
            .expect("intsty_0005 should exist")
            .as_f64();

        let times_h = [4.8667_f64, 17.2667, 19.4333, 21.3667, 23.9833];
        let pptcum_mm = [0.0_f64, 2.01, 4.02, 6.04, 7.35];
        let expected_stmdur = (times_h[4] - times_h[0]) * 3_600.0;
        let expected_timem_2 = (times_h[1] - times_h[0]) * 3_600.0;
        let mut expected_mxint: f64 = 0.0;
        for index in 1..times_h.len() {
            let drain_m = (pptcum_mm[index] - pptcum_mm[index - 1]) * 0.001;
            let delta_time_s = (times_h[index] - times_h[index - 1]) * 3_600.0;
            expected_mxint = expected_mxint.max(drain_m / delta_time_s);
        }

        assert!((stmstr - 4.8667).abs() < 1e-12);
        assert!((prcp - 0.00735).abs() < 1e-12);
        assert!((stmdur - expected_stmdur).abs() < 1e-6);
        assert!((mxint - expected_mxint).abs() < 1e-12);
        assert!(timem_1.abs() < 1e-12);
        assert!((timem_2 - expected_timem_2).abs() < 1e-6);
        assert!(intsty_5.abs() < 1e-12);
    }

    #[test]
    fn breakpoint_runtime_surface_supports_curated_wc1_42_point_event_shape() {
        let climate = parse_climate_from_str(WC1_BREAKPOINT_NBRKPT_42, ClimateParserMode::Strict)
            .expect("42-point wc1 fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("42-point breakpoint surface should build");

        let nbrkpt = surface
            .state_surface
            .get(&BoundarySymbol::from("nbrkpt"))
            .expect("nbrkpt should exist")
            .as_f64();
        let timem_first = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0001"))
            .expect("timem_0001 should exist")
            .as_f64();
        let timem_last = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0042"))
            .expect("timem_0042 should exist")
            .as_f64();
        let intsty_last = surface
            .state_surface
            .get(&BoundarySymbol::from("intsty_0042"))
            .expect("intsty_0042 should exist")
            .as_f64();

        assert!((nbrkpt - 42.0).abs() < 1e-12);
        assert!(timem_first.abs() < 1e-12);
        assert!(timem_last > timem_first);
        assert!(intsty_last.abs() < 1e-12);
    }

    #[test]
    fn breakpoint_runtime_surface_accepts_curated_wc1_zero_breakpoint_dry_day() {
        let climate = parse_climate_from_str(
            WC1_UNPALATABLE_RIND_BREAKPOINT_NBRKPT_0,
            ClimateParserMode::Strict,
        )
        .expect("wc1 zero-breakpoint fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("zero-breakpoint dry day should project runtime surface");

        let nbrkpt = surface
            .state_surface
            .get(&BoundarySymbol::from("nbrkpt"))
            .expect("nbrkpt should exist")
            .as_f64();
        let prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("prcp"))
            .expect("prcp should exist")
            .as_f64();
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("stmdur should exist")
            .as_f64();
        let mxint = surface
            .state_surface
            .get(&BoundarySymbol::from("mxint"))
            .expect("mxint should exist")
            .as_f64();
        let stmstr = surface
            .state_surface
            .get(&BoundarySymbol::from("stmstr"))
            .expect("stmstr should exist")
            .as_f64();

        assert!(nbrkpt.abs() < 1e-12);
        assert!(prcp.abs() < 1e-12);
        assert!(stmdur.abs() < 1e-12);
        assert!(mxint.abs() < 1e-12);
        assert!(stmstr.abs() < 1e-12);
        assert!(
            !surface
                .state_surface
                .contains_key(&BoundarySymbol::from("timem_0001"))
        );
        assert!(
            !surface
                .state_surface
                .contains_key(&BoundarySymbol::from("intsty_0001"))
        );
    }

    #[test]
    fn climate_runtime_surface_accepts_breakpoint_cardinality_at_1500_boundary() {
        let climate =
            parse_climate_from_str(&build_breakpoint_fixture(1_500), ClimateParserMode::Strict)
                .expect("strict parser should accept 1500 breakpoint rows");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("runtime seam should accept 1500 breakpoint rows");

        let nbrkpt = surface
            .state_surface
            .get(&BoundarySymbol::from("nbrkpt"))
            .expect("nbrkpt should exist")
            .as_f64();
        assert!((nbrkpt - 1_500.0).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_rejects_breakpoint_cardinality_over_1500_even_with_parser_override()
    {
        let climate = parse_climate_from_str(
            &build_breakpoint_fixture(1_501),
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: false,
                allow_breakpoint_cardinality_override: true,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("compat parser should allow >1500 breakpoint rows with explicit override");

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("runtime seam must reject >1500 breakpoint rows");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-011");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
                value: 1_501,
                max: 1_500
            }
        ));
    }

    #[test]
    fn climate_runtime_surface_supports_explicit_datver_zero_override() {
        let climate = parse_climate_from_str(LEGACY_DATVER_CLIMATE, ClimateParserMode::Strict)
            .expect("legacy datver fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("seam should accept explicit datver=0.0 override");

        let iclig = surface
            .state_surface
            .get(&BoundarySymbol::from("iclig"))
            .expect("iclig should exist for datver override")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist for datver override")
            .as_f64();
        assert!((iclig - 0.0).abs() < 1e-12);
        assert!((ip - 2.0).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_applies_timep_floor_for_wet_nonconstant_events() {
        let climate = parse_climate_from_str(WC1_CANOGA_DAY1, ClimateParserMode::Strict)
            .expect("wc1 fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("wc1 runtime surface should build");

        let timep = surface
            .state_surface
            .get(&BoundarySymbol::from("timep"))
            .expect("timep should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist")
            .as_f64();
        assert!((timep - 0.01).abs() < 1e-12);
        assert!((ip - 2.94).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_caps_storm_duration_to_23_999_hours() {
        let climate = parse_climate_from_str(WC1_CANOGA_STMDUR_CAP, ClimateParserMode::Strict)
            .expect("wc1 duration-cap fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("duration-cap fixture should build runtime surface");

        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("stmdur should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist")
            .as_f64();
        assert!((stmdur - (23.999 * 3_600.0)).abs() < 1e-9);
        assert!((ip - 22.589).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_rejects_pre4_nonzero_datver_branch() {
        let mut climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        climate.datver = 3.9;

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("pre-4 nonzero branch must be rejected");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-001");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::UnsupportedDatver { datver } if (datver - 3.9).abs() < 1e-12
        ));
    }

    #[test]
    fn climate_runtime_surface_rejects_single_storm_even_in_compat_parser_mode() {
        let climate = parse_climate_from_str(
            SINGLE_STORM_CLIMATE,
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: true,
                allow_breakpoint_cardinality_override: false,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("compat parser should accept itemp=2 when explicitly enabled");

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("runtime seam must reject single-storm itemp=2");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-002");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::UnsupportedItemp { itemp: 2 }
        ));
    }

    #[test]
    fn climate_runtime_surface_rejects_duplicate_breakpoint_times() {
        let mut climate = parse_climate_from_str(
            BREAKPOINT_OVERFLOW_CLIMATE,
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: false,
                allow_breakpoint_cardinality_override: true,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("breakpoint fixture should parse in compatibility mode");

        let day = climate
            .daily_records
            .first_mut()
            .expect("one breakpoint day expected");
        match day {
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::Breakpoint(record) => {
                let first_timem = record
                    .breakpoints
                    .first()
                    .expect("first breakpoint point should exist")
                    .timem;
                record
                    .breakpoints
                    .get_mut(1)
                    .expect("second breakpoint point should exist")
                    .timem = first_timem;
            }
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::NoBreakpoint(_) => {
                panic!("expected breakpoint daily record")
            }
        }

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("duplicate breakpoint timem must fail seam guard");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-009");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::NonMonotoneBreakpointTime { .. }
        ));
    }

    #[test]
    fn climate_runtime_surface_rejects_negative_breakpoint_drain() {
        let mut climate = parse_climate_from_str(
            BREAKPOINT_OVERFLOW_CLIMATE,
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: false,
                allow_breakpoint_cardinality_override: true,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("breakpoint fixture should parse in compatibility mode");

        let day = climate
            .daily_records
            .first_mut()
            .expect("one breakpoint day expected");
        match day {
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::Breakpoint(record) => {
                record
                    .breakpoints
                    .first_mut()
                    .expect("first breakpoint point should exist")
                    .pptcum = 0.02;
                record
                    .breakpoints
                    .get_mut(1)
                    .expect("second breakpoint point should exist")
                    .pptcum = 0.01;
            }
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::NoBreakpoint(_) => {
                panic!("expected breakpoint daily record")
            }
        }

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("negative breakpoint drain must fail seam guard");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-006");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::NegativeField {
                field: "drain",
                value
            } if value < 0.0
        ));
    }
