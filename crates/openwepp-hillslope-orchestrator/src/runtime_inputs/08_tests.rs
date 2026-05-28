#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fmt::Write as _;

    use openwepp_input_contract::parsers::{
        climate::{CompatibilityOptions, ParserMode as ClimateParserMode, parse_climate_from_str},
        management::{
            ParseMode as ManagementParseMode, PlantScenarioData, YearlyCroplandBranch,
            YearlyPerennialData, YearlyScenarioData, parse_management_from_str,
        },
        slope::{SlopeParserOptions, parse_slope_str},
        soil::{ParserMode, SoilParserOptions, parse_soil},
    };
    use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};

    use super::{
        ClimateRuntimeInputError, HillslopeRuntimeInputError,
        build_hillslope_pl_runtime_surfaces_from_management,
        build_hillslope_runtime_surface_from_climate,
        build_hillslope_runtime_surface_from_climate_with_context,
        build_hillslope_runtime_surface_from_management,
        build_hillslope_runtime_surface_from_slope, build_hillslope_runtime_surface_from_soil,
    };

    const VALID_CLIMATE: &str =
        include_str!("../../../../tests/fixtures/infile/climate/strict_valid.cli");
    const LEGACY_DATVER_CLIMATE: &str =
        include_str!("../../../../tests/fixtures/infile/climate/legacy_datver_0.cli");
    const SINGLE_STORM_CLIMATE: &str =
        include_str!("../../../../tests/fixtures/infile/climate/single_storm_itemp2.cli");
    const BREAKPOINT_OVERFLOW_CLIMATE: &str =
        include_str!("../../../../tests/fixtures/infile/climate/breakpoint_overflow_51.cli");
    const WC1_BREAKPOINT_STMSTR_NONZERO: &str = include_str!(
        "../../../../tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli"
    );
    const WC1_BREAKPOINT_NBRKPT_42: &str = include_str!(
        "../../../../tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_nbrkpt_42.cli"
    );
    const WC1_UNPALATABLE_RIND_BREAKPOINT_NBRKPT_0: &str = include_str!(
        "../../../../tests/fixtures/infile/climate/wc1_unpalatable_rind_breakpoint_nbrkpt_0.cli"
    );
    const WC1_CANOGA_DAY1: &str =
        include_str!("../../../../tests/fixtures/infile/climate/wc1_canoga_day1.cli");
    const WC1_CANOGA_STMDUR_CAP: &str =
        include_str!("../../../../tests/fixtures/infile/climate/wc1_canoga_stmdur_cap.cli");
    const SLOPE_STRICT_VALID_CANONICAL: &str =
        include_str!("../../../../tests/fixtures/infile/slope/strict_valid_canonical.slp");
    const VALID_9002: &str = include_str!("../../../../tests/fixtures/infile/soil/valid_9002.sol");
    const VALID_7778: &str = include_str!("../../../../tests/fixtures/infile/soil/valid_7778.sol");
    const VALID_97_5: &str = include_str!("../../../../tests/fixtures/infile/soil/valid_97_5.sol");
    const MANAGEMENT_CANONICAL_NONZERO_98_4: &str = include_str!(
        "../../../../tests/fixtures/infile/management/canonical_cropland_nonzero_98_4.man"
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
            BoundarySymbol::from("snow.options.rst"),
            BoundaryValue::scalar(rst),
        );
        context.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.058));
        context.insert(BoundarySymbol::from("azm"), BoundaryValue::scalar(0.0));
        context
    }

    #[test]
    fn soil_runtime_surface_contains_canonical_state_symbols() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");

        let solthk = surface
            .state_surface
            .get(&BoundarySymbol::from("solthk"))
            .expect("solthk should be present")
            .as_f64();
        let dg = surface
            .state_surface
            .get(&BoundarySymbol::from("dg"))
            .expect("dg should be present")
            .as_f64();
        let thetdr = surface
            .state_surface
            .get(&BoundarySymbol::from("thetdr"))
            .expect("thetdr should be present")
            .as_f64();
        let thetfc = surface
            .state_surface
            .get(&BoundarySymbol::from("thetfc"))
            .expect("thetfc should be present")
            .as_f64();
        let nsl = surface
            .state_surface
            .get(&BoundarySymbol::from("nsl"))
            .expect("nsl should be present")
            .as_f64();
        let ssc = surface
            .state_surface
            .get(&BoundarySymbol::from("ssc"))
            .expect("ssc should be present")
            .as_f64();
        let dg_layer2 = surface
            .state_surface
            .get(&BoundarySymbol::from("dg_0002"))
            .expect("dg_0002 should be present")
            .as_f64();
        let solthk_layer2 = surface
            .state_surface
            .get(&BoundarySymbol::from("solthk_0002"))
            .expect("solthk_0002 should be present")
            .as_f64();
        let ssc_layer2 = surface
            .state_surface
            .get(&BoundarySymbol::from("ssc_0002"))
            .expect("ssc_0002 should be present")
            .as_f64();

        assert!((solthk - 0.25).abs() < 1e-12);
        assert!((dg - 0.1).abs() < 1e-12);
        assert!((thetdr - 0.05).abs() < 1e-12);
        assert!((thetfc - 0.31).abs() < 1e-12);
        assert!((nsl - 2.0).abs() < 1e-12);
        assert!((ssc - (15.0 / 3.6e6)).abs() < 1e-12);
        assert!((dg_layer2 - 0.15).abs() < 1e-12);
        assert!((solthk_layer2 - 0.25).abs() < 1e-12);
        assert!((ssc_layer2 - (8.0 / 3.6e6)).abs() < 1e-12);
    }

    #[test]
    fn soil_runtime_surface_projects_ksatadj_policy_symbols_for_9002() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");

        let solwpv = surface
            .state_surface
            .get(&BoundarySymbol::from("solwpv"))
            .expect("solwpv should be present")
            .as_f64();
        assert!((solwpv - 9002.0).abs() < 1e-12);

        let ksatadj = surface
            .state_surface
            .get(&BoundarySymbol::from("ksatadj"))
            .expect("ksatadj should be present")
            .as_f64();
        assert!((ksatadj - 1.0).abs() < 1e-12);

        let ksatfac = surface
            .state_surface
            .get(&BoundarySymbol::from("ksatfac"))
            .expect("ksatfac should be present")
            .as_f64();
        assert!((ksatfac - 0.20).abs() < 1e-12);

        let ksatrec = surface
            .state_surface
            .get(&BoundarySymbol::from("ksatrec"))
            .expect("ksatrec should be present")
            .as_f64();
        assert!((ksatrec - 0.001).abs() < 1e-12);

        let ofe_ksatadj = surface
            .state_surface
            .get(&BoundarySymbol::from("ofe1_ksatadj"))
            .expect("ofe1_ksatadj should be present")
            .as_f64();
        assert!((ofe_ksatadj - 1.0).abs() < 1e-12);
    }

    #[test]
    fn soil_runtime_surface_defaults_ksatadj_to_zero_without_policy_block() {
        let soil = parse_soil(
            VALID_7778,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("7778 soil fixture should parse");

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");

        let solwpv = surface
            .state_surface
            .get(&BoundarySymbol::from("solwpv"))
            .expect("solwpv should be present")
            .as_f64();
        assert!((solwpv - 7778.0).abs() < 1e-12);

        let ksatadj = surface
            .state_surface
            .get(&BoundarySymbol::from("ksatadj"))
            .expect("ksatadj should be present")
            .as_f64();
        assert!(ksatadj.abs() < 1e-12);

        assert!(
            !surface
                .state_surface
                .contains_key(&BoundarySymbol::from("ksatfac")),
            "ksatfac should be absent when datver policy block is absent"
        );
        assert!(
            !surface
                .state_surface
                .contains_key(&BoundarySymbol::from("ksatrec")),
            "ksatrec should be absent when datver policy block is absent"
        );
        assert!(
            !surface
                .state_surface
                .contains_key(&BoundarySymbol::from("lkeff")),
            "lkeff should be absent when datver policy block is absent"
        );
    }

    #[test]
    fn soil_runtime_surface_uses_measured_theta_fallback_for_7778() {
        let soil = parse_soil(
            VALID_7778,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("7778 soil fixture should parse");

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from 7778 measured theta fields");

        let thetdr = surface
            .state_surface
            .get(&BoundarySymbol::from("thetdr"))
            .expect("thetdr should be present")
            .as_f64();
        let thetfc = surface
            .state_surface
            .get(&BoundarySymbol::from("thetfc"))
            .expect("thetfc should be present")
            .as_f64();
        let layer2_thetdr = surface
            .state_surface
            .get(&BoundarySymbol::from("thetdr_0002"))
            .expect("thetdr_0002 should be present")
            .as_f64();
        let layer2_thetfc = surface
            .state_surface
            .get(&BoundarySymbol::from("thetfc_0002"))
            .expect("thetfc_0002 should be present")
            .as_f64();

        assert!((thetdr - 0.1009).abs() < 1e-12);
        assert!((thetfc - 0.3282).abs() < 1e-12);
        assert!((layer2_thetdr - 0.0950).abs() < 1e-12);
        assert!((layer2_thetfc - 0.3120).abs() < 1e-12);
    }

    #[test]
    fn soil_runtime_surface_rejects_missing_theta_fields() {
        let soil = parse_soil(VALID_97_5, SoilParserOptions::default())
            .expect("97.5 soil fixture should parse");

        let error = build_hillslope_runtime_surface_from_soil(&soil)
            .expect_err("missing theta fields must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-003");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::MissingThetaResidual
        ));
    }

    #[test]
    fn soil_runtime_surface_rejects_missing_saturated_conductivity() {
        let mut soil = parse_soil(VALID_9002, SoilParserOptions::default())
            .expect("9002 soil fixture should parse");
        soil.ofes[0].layers[0].ksat_mm_h = None;

        let error = build_hillslope_runtime_surface_from_soil(&soil)
            .expect_err("missing ksat must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-033");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::MissingSaturatedConductivity {
                ofe_index: 1,
                layer_index: 1
            }
        ));
    }

    #[test]
    fn slope_runtime_surface_contains_canonical_state_symbols() {
        let slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
            .expect("strict slope fixture should parse");
        let surface = build_hillslope_runtime_surface_from_slope(&slope)
            .expect("slope runtime surface should build");

        let nelem = surface
            .state_surface
            .get(&BoundarySymbol::from("nelem"))
            .expect("nelem should be present")
            .as_f64();
        let slplen = surface
            .state_surface
            .get(&BoundarySymbol::from("slplen"))
            .expect("slplen should be present")
            .as_f64();
        let nslpts = surface
            .state_surface
            .get(&BoundarySymbol::from("nslpts"))
            .expect("nslpts should be present")
            .as_f64();
        let avgslp = surface
            .state_surface
            .get(&BoundarySymbol::from("avgslp"))
            .expect("avgslp should be present")
            .as_f64();
        let azm = surface
            .state_surface
            .get(&BoundarySymbol::from("azm"))
            .expect("azm should be present")
            .as_f64();
        let xinput_2 = surface
            .state_surface
            .get(&BoundarySymbol::from("xinput_0002"))
            .expect("xinput_0002 should be present")
            .as_f64();
        let slpinp_2 = surface
            .state_surface
            .get(&BoundarySymbol::from("slpinp_0002"))
            .expect("slpinp_0002 should be present")
            .as_f64();
        let ofe2_avgslp = surface
            .state_surface
            .get(&BoundarySymbol::from("ofe2_avgslp"))
            .expect("ofe2_avgslp should be present")
            .as_f64();

        assert!((nelem - 2.0).abs() < 1e-12);
        assert!((slplen - 60.0).abs() < 1e-12);
        assert!((nslpts - 3.0).abs() < 1e-12);
        assert!((avgslp - 0.058).abs() < 1e-12);
        assert!((azm - 180.0).abs() < 1e-12);
        assert!((xinput_2 - 0.6).abs() < 1e-12);
        assert!((slpinp_2 - 0.08).abs() < 1e-12);
        assert!((ofe2_avgslp - 0.0425).abs() < 1e-12);
    }

    #[test]
    fn slope_runtime_surface_rejects_non_positive_derived_avgslp() {
        let mut slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
            .expect("strict slope fixture should parse");

        for point in &mut slope.ofes[0].points {
            point.slpinp = 0.0;
        }

        let error = build_hillslope_runtime_surface_from_slope(&slope)
            .expect_err("non-positive derived avgslp must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-023");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::NonPositiveDerivedAverageSlope {
                ofe_index: 1,
                value
            } if value.abs() < 1e-12
        ));
    }

    #[test]
    fn management_runtime_surfaces_project_required_pl_controls_and_seeds() {
        let management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("pl runtime surface projection should succeed");
        let merged_surface = build_hillslope_runtime_surface_from_management(&management)
            .expect("merged pl runtime state surface should build");

        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("lanuse")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("itype")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("pl_order_decomp_before_soil")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("jdharv")),
            Some(&BoundaryValue::scalar(288.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("jdplt")),
            Some(&BoundaryValue::scalar(130.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("rw")),
            Some(&BoundaryValue::scalar(0.762))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("resmgt")),
            Some(&BoundaryValue::scalar(6.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("sumrtm_seed")),
            Some(&BoundaryValue::scalar(0.50003))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("sumsrm_seed")),
            Some(&BoundaryValue::scalar(0.19997))
        );

        assert_eq!(
            pl_surfaces.pl_schedule_surface.get(&BoundarySymbol::from(
                "pl_schedule_slot_0001_crop_0001_itype"
            )),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            pl_surfaces
                .pl_growth_surface
                .get(&BoundarySymbol::from("pl_growth_slot_0001_crop_0001_rw")),
            Some(&BoundaryValue::scalar(0.762))
        );
        assert_eq!(
            pl_surfaces.pl_decomp_surface.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_resmgt"
            )),
            Some(&BoundaryValue::scalar(6.0))
        );
    }

    #[test]
    fn management_runtime_projection_rejects_out_of_range_initial_reference() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        management.schedule.ofe_initial_refs[0] = 0;

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("out-of-range initial reference must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-039");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::ManagementInitialReferenceOutOfRange {
                ofe_index: 1,
                initial_ref: 0,
                max_initial_ref: 1
            }
        ));
    }

    #[test]
    fn management_runtime_projection_rejects_unsupported_pl_landuse() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        management.registries.initials[0].meta.landuse = 2;

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("unsupported landuse must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-041");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::UnsupportedPlLanduse {
                section: "initial",
                value: 2
            }
        ));
    }

    #[test]
    fn management_runtime_projection_rejects_non_finite_row_width() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        match &mut cropland.branch {
            YearlyCroplandBranch::AnnualOrFallow(annual) => annual.rw = f64::NAN,
            YearlyCroplandBranch::Perennial(_) => panic!("fixture should use annual branch"),
        }

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("non-finite row width must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-043");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::NonFinitePlProjectionField {
                field: "rw",
                slot_index: 1,
                crop_slot_index: 1,
                value,
            } if value.is_nan()
        ));
    }

    #[test]
    fn management_runtime_projection_rejects_unsupported_perennial_option() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.imngmt = 2;
        cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
            jdharv: 288,
            jdplt: 130,
            jdstop: 0,
            rw: 0.762,
            mgtopt: 4,
            cut_days: Vec::new(),
            grazing_cycles: Vec::new(),
        });

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("unsupported perennial mgtopt must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-042");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::UnsupportedPlManagementOption {
                field: "mgtopt",
                value: 4,
                allowed: "1..3",
            }
        ));
    }

    #[test]
    fn management_runtime_projection_allows_zero_gddmax_sentinel_for_legacy_resolution() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let plant = &mut management.registries.plants[0];
        let PlantScenarioData::Cropland(cropland) = &mut plant.data;
        cropland.growth_line[5] = 0.0;

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("gddmax zero sentinel should project for runtime resolution");
        assert_eq!(
            pl_surfaces.pl_growth_surface.get(&BoundarySymbol::from(
                "pl_growth_slot_0001_crop_0001_gddmax"
            )),
            Some(&BoundaryValue::scalar(0.0))
        );
    }

    #[test]
    fn management_runtime_projection_allows_zero_oratea_orater_for_legacy_no_decay() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let plant = &mut management.registries.plants[0];
        let PlantScenarioData::Cropland(cropland) = &mut plant.data;
        cropland.residue_line[0] = 0.0;
        cropland.residue_line[1] = 0.0;

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("zero decomposition constants should project for legacy no-decay semantics");
        assert_eq!(
            pl_surfaces.pl_decomp_surface.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_oratea"
            )),
            Some(&BoundaryValue::scalar(0.0))
        );
        assert_eq!(
            pl_surfaces.pl_decomp_surface.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_orater"
            )),
            Some(&BoundaryValue::scalar(0.0))
        );
    }

    #[test]
    fn management_runtime_projection_rejects_negative_oratea_projection_field() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let plant = &mut management.registries.plants[0];
        let PlantScenarioData::Cropland(cropland) = &mut plant.data;
        cropland.residue_line[0] = -0.1;

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("negative decomposition constants must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-050");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "oratea",
                slot_index: 1,
                crop_slot_index: 1,
                value,
                ..
            } if (value + 0.1).abs() < 1e-12
        ));
    }

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

            assert!(rad.is_finite());
            assert!(temp.is_finite());
            assert!((0.0..=1.0).contains(&cloud));
            assert!(rain >= 0.0);
            assert!(snow >= 0.0);
            rain_total += rain;
            snow_total += snow;
        }

        assert!(rain_total > 0.0 || snow_total > 0.0);
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
}
