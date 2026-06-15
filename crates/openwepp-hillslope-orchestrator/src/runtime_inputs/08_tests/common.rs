    use std::collections::BTreeMap;
    use std::fmt::Write as _;

    use openwepp_input_contract::parsers::{
        climate::{CompatibilityOptions, ParserMode as ClimateParserMode, parse_climate_from_str},
        management::{
            DrainScenario, InitialScenarioData, ParseMode as ManagementParseMode,
            PlantScenarioData, ScenarioMeta, YearlyAnnualExtension, YearlyAnnualFallowData,
            YearlyCroplandBranch, YearlyPerennialData, YearlyPerennialGrazingCycle,
            YearlyScenarioData,
            parse_management_from_str,
        },
        slope::{SlopeParserOptions, parse_slope_str},
        soil::{ParserMode, SoilDatver, SoilParserOptions, parse_soil},
    };
    use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};

    use super::{
        ClimateRuntimeInputError, HillslopeRuntimeInputError,
        build_hillslope_pl_runtime_surfaces_from_management,
        build_hillslope_runtime_surface_from_climate,
        build_hillslope_runtime_surface_from_climate_with_context,
        build_hillslope_runtime_surface_from_management,
        build_hillslope_runtime_surface_from_slope, build_hillslope_runtime_surface_from_soil,
        legacy_correct_layer_moisture, legacy_expand_soil_layers_to_200mm,
        simimpl28_stmtim_hourly_partition, LegacySoilLayerSeed,
    };

    const VALID_CLIMATE: &str =
        include_str!("../../../../../tests/fixtures/infile/climate/strict_valid.cli");
    const LEGACY_DATVER_CLIMATE: &str =
        include_str!("../../../../../tests/fixtures/infile/climate/legacy_datver_0.cli");
    const SINGLE_STORM_CLIMATE: &str =
        include_str!("../../../../../tests/fixtures/infile/climate/single_storm_itemp2.cli");
    const BREAKPOINT_OVERFLOW_CLIMATE: &str =
        include_str!("../../../../../tests/fixtures/infile/climate/breakpoint_overflow_51.cli");
    const WC1_BREAKPOINT_STMSTR_NONZERO: &str = include_str!(
        "../../../../../tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli"
    );
    const WC1_BREAKPOINT_NBRKPT_42: &str = include_str!(
        "../../../../../tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_nbrkpt_42.cli"
    );
    const WC1_UNPALATABLE_RIND_BREAKPOINT_NBRKPT_0: &str = include_str!(
        "../../../../../tests/fixtures/infile/climate/wc1_unpalatable_rind_breakpoint_nbrkpt_0.cli"
    );
    const WC1_CANOGA_DAY1: &str =
        include_str!("../../../../../tests/fixtures/infile/climate/wc1_canoga_day1.cli");
    const WC1_CANOGA_STMDUR_CAP: &str =
        include_str!("../../../../../tests/fixtures/infile/climate/wc1_canoga_stmdur_cap.cli");
    const SLOPE_STRICT_VALID_CANONICAL: &str =
        include_str!("../../../../../tests/fixtures/infile/slope/strict_valid_canonical.slp");
    const VALID_9002: &str = include_str!("../../../../../tests/fixtures/infile/soil/valid_9002.sol");
    const VALID_7778: &str = include_str!("../../../../../tests/fixtures/infile/soil/valid_7778.sol");
    const VALID_97_5: &str = include_str!("../../../../../tests/fixtures/infile/soil/valid_97_5.sol");
    const MANAGEMENT_CANONICAL_NONZERO_98_4: &str = include_str!(
        "../../../../../tests/fixtures/infile/management/canonical_cropland_nonzero_98_4.man"
    );

    fn build_breakpoint_fixture(nbrkpt: usize) -> String {
        let mut climate = format!(
            "5.30\n1 1 0\nTEST STATION 1500\nDAY MON YEAR NBRKPT TMAX TMIN RAD VWIND WIND TDPT\n45.0 -120.0 1000.0 30 2000 1\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n1 1 2000 {nbrkpt} 11.0 1.0 180.0 2.0 170.0 -2.0\n"
        );
        if nbrkpt == 0 {
            return climate;
        }
        let denom_u32 = u32::try_from((nbrkpt - 1).max(1))
            .expect("breakpoint fixture helper expects small cardinalities");
        let denom = f64::from(denom_u32);
        for index in 0..nbrkpt {
            let idx_u32 = u32::try_from(index)
                .expect("breakpoint fixture helper expects small cardinalities");
            let idx = f64::from(idx_u32);
            let timem = (24.0 * idx) / denom;
            let pptcum = (120.0 * idx) / denom;
            writeln!(&mut climate, "{timem:.4} {pptcum:.3}")
                .expect("writing synthetic breakpoint fixture should succeed");
        }
        climate
    }

    fn simimpl28_winter_context(rst: f64) -> BTreeMap<BoundarySymbol, BoundaryValue> {
        let mut context = BTreeMap::new();
        context.insert(
            BoundarySymbol::from("snow.options.snow_file_present"),
            BoundaryValue::scalar(1.0),
        );
        context.insert(
            BoundarySymbol::from("frost.options.frost_file_present"),
            BoundaryValue::scalar(0.0),
        );
        context.insert(
            BoundarySymbol::from("frost.options.wintRed"),
            BoundaryValue::scalar(0.0),
        );
        context.insert(
            BoundarySymbol::from("snow.options.rst"),
            BoundaryValue::scalar(rst),
        );
        context.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.01),
        );
        context.insert(
            BoundarySymbol::from("frost.runtime_dfrost"),
            BoundaryValue::scalar(0.0),
        );
        context.insert(
            BoundarySymbol::from("frost.runtime_ws_frz"),
            BoundaryValue::scalar(0.0),
        );
        context.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.058));
        context.insert(BoundarySymbol::from("azm"), BoundaryValue::scalar(0.0));
        context
    }

    #[derive(Clone, Copy, Debug)]
    struct Wb13NormalizedProfileExpectation {
        depth: f64,
        porosity_cap: f64,
        fc_store: f64,
        wp_store: f64,
    }

    fn normalized_corrected_layers_from_ofe(
        soil_datver: SoilDatver,
        ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
    ) -> Vec<(f64, f64, f64, f64, f64, f64)> {
        let fc_wp_policy = super::fc_wp_rock_multiplier_policy(soil_datver);
        let seeds = ofe
            .layers
            .iter()
            .enumerate()
            .map(|(layer_position, layer)| LegacySoilLayerSeed {
                depth_mm: layer.depth_mm,
                bulk_density_g_cm3: layer.bulk_density_g_cm3.unwrap_or_else(|| {
                    panic!(
                        "fixture layer {} must include bulk_density_g_cm3 for normalization test",
                        layer_position + 1
                    )
                }),
                fc_measured: layer.fc_measured.unwrap_or_else(|| {
                    panic!(
                        "fixture layer {} must include fc_measured for normalization test",
                        layer_position + 1
                    )
                }),
                wp_measured: layer.wp_measured.unwrap_or_else(|| {
                    panic!(
                        "fixture layer {} must include wp_measured for normalization test",
                        layer_position + 1
                    )
                }),
                sand_pct: layer.sand_pct,
                clay_pct: layer.clay_pct,
                orgmat_pct: layer.orgmat_pct,
                cec_meq_100g: layer.cec_meq_100g,
                rock_frag_pct: layer.rock_frag_pct,
                fc_wp_rock_multiplier_policy: fc_wp_policy,
            })
            .collect::<Vec<_>>();

        legacy_expand_soil_layers_to_200mm(&seeds)
            .expect("fixture should produce normalized correction layers")
            .into_iter()
            .map(|layer| {
                let corrected = legacy_correct_layer_moisture(layer)
                    .expect("fixture normalized layers should yield corrected moisture");
                (
                    corrected.thickness_m,
                    corrected.porosity,
                    corrected.cpm,
                    corrected.coca,
                    corrected.thetfc,
                    corrected.thetdr,
                )
            })
            .collect::<Vec<_>>()
    }

    fn expected_wb13_profile_symbols_from_normalized_correction(
        soil_datver: SoilDatver,
        ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
    ) -> Wb13NormalizedProfileExpectation {
        let mut expectation = Wb13NormalizedProfileExpectation {
            depth: 0.0,
            porosity_cap: 0.0,
            fc_store: 0.0,
            wp_store: 0.0,
        };
        for (thickness_m, porosity, _cpm, _coca, thetfc, thetdr) in
            normalized_corrected_layers_from_ofe(soil_datver, ofe)
        {
            let thickness_mm = thickness_m * 1_000.0;
            expectation.depth += thickness_mm;
            expectation.porosity_cap += porosity * thickness_mm;
            expectation.fc_store += thetfc * thickness_mm;
            expectation.wp_store += thetdr * thickness_mm;
        }
        expectation
    }

    fn aggregated_profile_storage_from_layer_symbols(
        surface: &crate::HillslopeWritebackSurface,
    ) -> (f64, f64) {
        let nsl_raw = surface
            .state_surface
            .get(&BoundarySymbol::from("wb11_nsl"))
            .expect("wb11_nsl should be present")
            .as_f64();
        let nsl = format!("{nsl_raw:.0}")
            .parse::<usize>()
            .expect("nsl should round-trip into usize");
        assert!(nsl >= 1, "nsl must be >= 1");

        let mut aggregated_fc_store_mm = 0.0_f64;
        let mut aggregated_wp_store_mm = 0.0_f64;
        for layer_index in 1..=nsl {
            let dg = surface
                .state_surface
                .get(&BoundarySymbol::from(format!("wb19_dg_{layer_index:04}")))
                .unwrap_or_else(|| panic!("wb19_dg_{layer_index:04} should be present"))
                .as_f64();
            let thetfc = surface
                .state_surface
                .get(&BoundarySymbol::from(format!("wb19_thetfc_{layer_index:04}")))
                .unwrap_or_else(|| panic!("wb19_thetfc_{layer_index:04} should be present"))
                .as_f64();
            let thetdr = surface
                .state_surface
                .get(&BoundarySymbol::from(format!("wb19_thetdr_{layer_index:04}")))
                .unwrap_or_else(|| panic!("wb19_thetdr_{layer_index:04} should be present"))
                .as_f64();
            aggregated_fc_store_mm += thetfc * dg * 1_000.0;
            aggregated_wp_store_mm += thetdr * dg * 1_000.0;
        }

        (aggregated_fc_store_mm, aggregated_wp_store_mm)
    }

    fn soil_runtime_scalar(surface: &crate::HillslopeWritebackSurface, symbol: &str) -> f64 {
        surface
            .state_surface
            .get(&BoundarySymbol::from(symbol))
            .unwrap_or_else(|| panic!("{symbol} should be present"))
            .as_f64()
    }
