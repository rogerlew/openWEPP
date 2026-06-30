use super::super::*;
use crate::hillslope::intake_lane_setup::{
    TypedWb11EvappmEtInput, TypedWb11EvappmEtLayerInput, TypedWb11PriestleyTaylorEtInput,
    absent_pmetpara_file, build_hillslope_runtime_surface_from_pmetpara,
    project_typed_pmetpara_runtime, project_typed_wb11_evappm_et_demand,
    project_typed_wb11_priestley_taylor_et_demand, publish_wb11_et_demand_seed,
};

const PMETPARA_STRICT_VALID_FOR_CORN: &str = "2
Corn,1.20,0.55,1,default
Wheat,1.05,0.45,2,cover
";
const MANAGEMENT_CANONICAL_NONZERO_98_4: &str = include_str!(
    "../../../../../../tests/fixtures/infile/management/canonical_cropland_nonzero_98_4.man"
);
const SOIL_VALID_9002: &str =
    include_str!("../../../../../../tests/fixtures/infile/soil/valid_9002.sol");

    #[test]
    fn typed_pmetpara_absent_projection_matches_surface_adapter() {
        let management = openwepp_input_contract::parsers::management::parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            openwepp_input_contract::parsers::management::ParseMode::Strict,
        )
        .expect("management fixture should parse");
        let mut typed_pmetpara = absent_pmetpara_file();
        let mut surface_pmetpara = absent_pmetpara_file();

        let typed = project_typed_pmetpara_runtime(
            &management,
            &mut typed_pmetpara,
            PmetparaParseMode::Strict,
        )
        .expect("typed absent PMET projection should succeed");
        let surface = build_hillslope_runtime_surface_from_pmetpara(
            &management,
            &mut surface_pmetpara,
            PmetparaParseMode::Strict,
        )
        .expect("surface PMET adapter should succeed");

        assert!(!typed.sidecar_present);
        assert_eq!(
            (if typed.sidecar_present { 1.0_f64 } else { 0.0_f64 }).to_bits(),
            require_runtime_surface_scalar(&surface, "pmetpara.mode.sidecar_present")
                .expect("PMET sidecar flag should be published")
                .to_bits()
        );
        assert_eq!(
            f64::from(typed.iflget).to_bits(),
            require_runtime_surface_scalar(&surface, "pmetpara.mode.iflget")
                .expect("PMET mode should be published")
                .to_bits()
        );
        assert!(
            runtime_surface_symbol_value(&surface, "pmetpara.selected.kcb").is_none(),
            "absent sidecar must not publish selected PMET crop values"
        );
    }

    #[test]
    fn typed_pmetpara_selected_projection_matches_surface_adapter() {
        let management = openwepp_input_contract::parsers::management::parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            openwepp_input_contract::parsers::management::ParseMode::Strict,
        )
        .expect("management fixture should parse");
        let options = openwepp_input_contract::parsers::pmetpara::PmetparaParseOptions {
            mode: openwepp_input_contract::parsers::pmetpara::ParseMode::Strict,
            require_sidecar: true,
        };
        let mut typed_pmetpara =
            openwepp_input_contract::parsers::pmetpara::parse_pmetpara_from_str(
                PMETPARA_STRICT_VALID_FOR_CORN,
                options,
            )
            .expect("PMET fixture should parse");
        let mut surface_pmetpara =
            openwepp_input_contract::parsers::pmetpara::parse_pmetpara_from_str(
                PMETPARA_STRICT_VALID_FOR_CORN,
                options,
            )
            .expect("PMET fixture should parse");

        let typed = project_typed_pmetpara_runtime(
            &management,
            &mut typed_pmetpara,
            PmetparaParseMode::Strict,
        )
        .expect("typed PMET projection should select active crop");
        let surface = build_hillslope_runtime_surface_from_pmetpara(
            &management,
            &mut surface_pmetpara,
            PmetparaParseMode::Strict,
        )
        .expect("surface PMET adapter should succeed");
        let selected = typed
            .selected
            .expect("strict PMET fixture should select active crop");

        assert_eq!(
            selected.kcb.to_bits(),
            require_runtime_surface_scalar(&surface, "pmetpara.selected.kcb")
                .expect("selected kcb should be published")
                .to_bits()
        );
        assert_eq!(
            selected.rawp.to_bits(),
            require_runtime_surface_scalar(&surface, "pmetpara.selected.rawp")
                .expect("selected rawp should be published")
                .to_bits()
        );
        assert_eq!(
            f64::from(selected.line_index).to_bits(),
            require_runtime_surface_scalar(&surface, "pmetpara.selected.line_index")
                .expect("selected line should be published")
                .to_bits()
        );
        assert_eq!(
            (if selected.fallback_first_row_used {
                1.0_f64
            } else {
                0.0_f64
            })
            .to_bits(),
            require_runtime_surface_scalar(&surface, "pmetpara.lookup.fallback_first_row_used")
                .expect("fallback flag should be published")
                .to_bits()
        );
    }

    #[test]
    fn typed_direct_layer_seed_matches_day_zero_surface_seed() {
        let soil = openwepp_input_contract::parsers::soil::parse_soil(
            SOIL_VALID_9002,
            openwepp_input_contract::parsers::soil::SoilParserOptions {
                mode: openwepp_input_contract::parsers::soil::ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("soil fixture should parse");
        let typed_soil = project_typed_soil_wb11_runtime(&soil)
            .expect("typed soil projection should build");
        let typed_seed = direct_production_typed_layer_seed(&typed_soil, ExecutionLane::Daily)
            .expect("typed direct layer seed should build");
        let mut surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("soil surface should build");
        for (symbol, value) in [
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("wb19_drain_enabled", 0.0),
        ] {
            surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }

        seed_wb11_runtime_surface_inputs(&mut surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");
        let surface_layers =
            direct_publication_layer_states(&surface).expect("surface layer states should build");

        assert_eq!(
            typed_seed.soil_water_m.to_bits(),
            require_runtime_surface_scalar(&surface, "wb11_soil_water")
                .expect("surface seed should publish soil water")
                .to_bits()
        );
        assert_eq!(
            typed_seed.field_capacity_m.to_bits(),
            require_runtime_surface_scalar(&surface, "wb11_field_capacity")
                .expect("surface seed should publish field capacity")
                .to_bits()
        );
        assert_eq!(
            typed_seed.drainable_storage_m.to_bits(),
            require_runtime_surface_scalar(&surface, "wb11_drainable_storage")
                .expect("surface seed should publish drainable storage")
                .to_bits()
        );
        assert_eq!(
            typed_seed.drainage_coefficient.to_bits(),
            require_runtime_surface_scalar(&surface, "wb11_drainage_coefficient")
                .expect("surface seed should publish drainage coefficient")
                .to_bits()
        );
        assert_eq!(typed_seed.layers.len(), surface_layers.len());
        for (typed, surface) in typed_seed.layers.iter().zip(surface_layers.iter()) {
            assert_eq!(typed.theta_m.to_bits(), surface.theta_m.to_bits());
            assert_eq!(
                typed.field_capacity_m.to_bits(),
                surface.field_capacity_m.to_bits()
            );
            assert_eq!(typed.upper_limit_m.to_bits(), surface.upper_limit_m.to_bits());
            assert_eq!(
                typed.conductivity_m_s.to_bits(),
                surface.conductivity_m_s.to_bits()
            );
            assert_eq!(typed.depth_m.to_bits(), surface.depth_m.to_bits());
            assert_eq!(
                typed.residual_theta.to_bits(),
                surface.residual_theta.to_bits()
            );
            assert_eq!(typed.frozen_depth_m.to_bits(), surface.frozen_depth_m.to_bits());
            assert_eq!(typed.frozen_water_m.to_bits(), surface.frozen_water_m.to_bits());
            assert_eq!(typed.porosity.to_bits(), surface.porosity.to_bits());
            assert_eq!(
                typed.field_capacity_theta.to_bits(),
                surface.field_capacity_theta.to_bits()
            );
            assert_eq!(typed.coca.to_bits(), surface.coca.to_bits());
            assert_eq!(
                typed.lateral_conductivity_m_s.to_bits(),
                surface.lateral_conductivity_m_s.to_bits()
            );
        }
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
    fn typed_day_zero_wb11_hyetograph_breakpoint_seed_matches_surface_seed() {
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

        let typed_projection = project_typed_wb11_hyetograph(
            0.003,
            "nbrkpt",
            3,
            None,
            &[
                TypedWb11HyetographInterval {
                    point_index: 1,
                    time_s: 0.0,
                    next_time_s: 36_479.88,
                    intensity_m_s: 5.701_773_141_797_617e-8,
                },
                TypedWb11HyetographInterval {
                    point_index: 2,
                    time_s: 36_479.88,
                    next_time_s: 38_279.88,
                    intensity_m_s: 5.111_111_111_111_11e-7,
                },
            ],
        )
        .expect("typed hyetograph projection should accept breakpoint cardinality");
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        let typed_point_count =
            usize_to_scalar("ninten", typed_projection.point_count)
                .expect("typed point count should be representable as scalar");
        assert_eq!(
            typed_point_count.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "ninten")
                .expect("surface seed should publish ninten")
                .to_bits()
        );
        assert_eq!(
            typed_projection.rainfall_input_m.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb12_rainfall_input")
                .expect("surface seed should publish rainfall input")
                .to_bits()
        );
    }

    #[test]
    fn cqr15_wb11_seed_zero_hyetograph_synthesizes_two_point_event() {
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
            ("prcp", 0.004),
            ("stmdur", 7_200.0),
            ("ninten", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("zero-cardinality hyetograph should synthesize a two-point event");

        let ninten = require_runtime_surface_scalar(&runtime_surface, "ninten")
            .expect("ninten should be synthesized");
        let nbrkpt = require_runtime_surface_scalar(&runtime_surface, "nbrkpt")
            .expect("nbrkpt should be aligned with ninten");
        let timem_0001 = require_runtime_surface_scalar(&runtime_surface, "timem_0001")
            .expect("first synthesized time should be published");
        let timem_0002 = require_runtime_surface_scalar(&runtime_surface, "timem_0002")
            .expect("second synthesized time should be published");
        let intsty_0001 = require_runtime_surface_scalar(&runtime_surface, "intsty_0001")
            .expect("first synthesized intensity should be published");
        let intsty_0002 = require_runtime_surface_scalar(&runtime_surface, "intsty_0002")
            .expect("second synthesized intensity should be published");
        let rainfall_input =
            require_runtime_surface_scalar(&runtime_surface, "wb12_rainfall_input")
                .expect("rainfall input should be seeded");

        assert!((ninten - 2.0).abs() < 1.0e-12);
        assert!((nbrkpt - 2.0).abs() < 1.0e-12);
        assert!(timem_0001.abs() < 1.0e-12);
        assert!((timem_0002 - 7_200.0).abs() < 1.0e-12);
        assert!((intsty_0001 - (0.004 / 7_200.0)).abs() < 1.0e-18);
        assert!(intsty_0002.abs() < 1.0e-12);
        assert!(
            (rainfall_input - 0.004).abs() < 1.0e-12,
            "synthesized hyetograph rainfall should match the daily precipitation depth"
        );
    }

    #[test]
    fn typed_day_zero_wb11_hyetograph_zero_event_matches_surface_seed() {
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
            ("prcp", 0.004),
            ("stmdur", 7_200.0),
            ("ninten", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        let typed_projection = project_typed_wb11_hyetograph(0.004, "ninten", 0, Some(7_200.0), &[])
            .expect("typed zero-cardinality hyetograph should synthesize event");
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        let synthesized = typed_projection
            .synthesized_zero_event
            .expect("typed projection should carry synthesized event");
        let typed_point_count =
            usize_to_scalar("ninten", typed_projection.point_count)
                .expect("typed point count should be representable as scalar");
        assert_eq!(
            typed_point_count.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "ninten")
                .expect("surface seed should publish ninten")
                .to_bits()
        );
        assert_eq!(
            typed_projection.rainfall_input_m.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb12_rainfall_input")
                .expect("surface seed should publish rainfall input")
                .to_bits()
        );
        assert_eq!(
            synthesized.time_0001_s.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "timem_0001")
                .expect("surface seed should publish first synthesized time")
                .to_bits()
        );
        assert_eq!(
            synthesized.time_0002_s.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "timem_0002")
                .expect("surface seed should publish second synthesized time")
                .to_bits()
        );
        assert_eq!(
            synthesized.intensity_0001_m_s.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "intsty_0001")
                .expect("surface seed should publish first synthesized intensity")
                .to_bits()
        );
        assert_eq!(
            synthesized.intensity_0002_m_s.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "intsty_0002")
                .expect("surface seed should publish second synthesized intensity")
                .to_bits()
        );
    }

    #[test]
    fn cqr15_wb11_seed_uses_hyetograph_total_when_it_exceeds_prcp() {
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
            ("prcp", 0.001),
            ("ninten", 3.0),
            ("timem_0001", 0.0),
            ("timem_0002", 10.0),
            ("timem_0003", 20.0),
            ("intsty_0001", 0.000_1),
            ("intsty_0002", 0.000_2),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("valid hyetograph should seed WB12 rainfall input");

        let rainfall_input =
            require_runtime_surface_scalar(&runtime_surface, "wb12_rainfall_input")
                .expect("rainfall input should be seeded");
        assert!(
            (rainfall_input - 0.003).abs() < 1.0e-12,
            "WB12 rainfall input must preserve the larger hyetograph-integrated depth"
        );
    }

    #[test]
    fn typed_day_zero_wb11_hyetograph_total_exceeds_prcp_matches_surface_seed() {
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
            ("prcp", 0.001),
            ("ninten", 3.0),
            ("timem_0001", 0.0),
            ("timem_0002", 10.0),
            ("timem_0003", 20.0),
            ("intsty_0001", 0.000_1),
            ("intsty_0002", 0.000_2),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        let typed_projection = project_typed_wb11_hyetograph(
            0.001,
            "ninten",
            3,
            None,
            &[
                TypedWb11HyetographInterval {
                    point_index: 1,
                    time_s: 0.0,
                    next_time_s: 10.0,
                    intensity_m_s: 0.000_1,
                },
                TypedWb11HyetographInterval {
                    point_index: 2,
                    time_s: 10.0,
                    next_time_s: 20.0,
                    intensity_m_s: 0.000_2,
                },
            ],
        )
        .expect("typed hyetograph projection should use integrated storm total");
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        assert_eq!(
            typed_projection.rainfall_input_m.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb12_rainfall_input")
                .expect("surface seed should publish rainfall input")
                .to_bits()
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
    fn typed_day_zero_wb11_optional_defaults_match_surface_seed() {
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

        let typed_projection = project_typed_wb11_optional_defaults(None, None);
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        assert!(typed_projection.residue_interception_was_defaulted);
        assert!(typed_projection.water_stress_was_defaulted);
        assert_eq!(
            typed_projection.residue_interception_m.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb17_residue_interception")
                .expect("surface seed should publish residue interception default")
                .to_bits()
        );
        assert_eq!(
            typed_projection.water_stress.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "Ws")
                .expect("surface seed should publish water stress default")
                .to_bits()
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
    fn typed_day_zero_wb11_lane_substeps_match_surface_seed() {
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
            ("wb19_lateral_anisotropy_ratio", 1.0),
            ("wb19_drain_enabled", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        let typed_projection = project_typed_wb11_lane_substeps(ExecutionLane::Daily, 2)
            .expect("typed lane substep projection should accept multi-OFE daily setup");
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        assert_eq!(
            typed_projection.wb18_perc_lane_substeps.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb18_perc_lane_substeps")
                .expect("surface seed should publish WB18 substeps")
                .to_bits()
        );
        assert_eq!(
            typed_projection
                .wb19_lateral_drain_lane_substeps
                .to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb19_lateral_drain_lane_substeps")
                .expect("surface seed should publish WB19 substeps")
                .to_bits()
        );
        assert!(
            typed_projection.mofe_hourly_carry_active,
            "multi-OFE typed projection should activate hourly carry"
        );
        assert!(
            (require_runtime_surface_scalar(&runtime_surface, "mofe_hourly_carry_arrays_enabled")
                .expect("surface seed should publish MOFE hourly carry enablement")
                - 1.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn typed_day_zero_mofe03_wave2_enabled_seed_matches_surface_seed() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 2.0),
            ("mofe.static_lane.contributor_ofe_count", 2.0),
            ("slplen", 50.0),
            ("tmax", -3.0),
            ("tmin", -6.9),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.0),
            ("nbrkpt", 2.0),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("Q", 0.02),
            ("UpStrmQ", 0.005),
            ("thetdr", 0.08),
            ("thetfc", 0.24),
            ("wb19_lateral_anisotropy_ratio", 1.0),
            ("wb19_drain_enabled", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.55, true);

        let typed_projection = project_typed_mofe03_wave2(TypedMofe03Wave2Input {
            wave2_enabled: true,
            slplen_m: 50.0,
            qout_m3_s: 0.02,
            qin_m3_s: 0.005,
            efflen_m: None,
            ssa_soil: None,
            beta: MOFE03_WAVE2_DEFAULT_BETA,
            theta: 0.16,
        })
        .expect("typed MOFE03 projection should compute");
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface seeding should compute MOFE03 Wave-2 projection");

        assert_mofe03_wave2_projection_matches_surface(&runtime_surface, &typed_projection);
    }

    fn assert_mofe03_wave2_projection_matches_surface(
        runtime_surface: &HillslopeWritebackSurface,
        typed_projection: &TypedMofe03Wave2Projection,
    ) {
        for (symbol, typed_value) in [
            ("erod14_xtop", typed_projection.xtop_m),
            ("erod14_xbot", typed_projection.xbot_m),
            ("erod14_xdetst", typed_projection.xdetst_m),
            ("erod14_ldtop", typed_projection.ldtop_kg_s_m),
            ("erod14_ldbot", typed_projection.ldbot_kg_s_m),
            ("erod14_lddend", typed_projection.lddend_kg),
            ("erod14_qout", typed_projection.qout_m3_s),
            ("erod14_qin", typed_projection.qin_m3_s),
            ("erod14_qostar", typed_projection.route_qostar_m),
            ("erod14_slplen", typed_projection.slplen_m),
            ("erod14_ktrato", typed_projection.ktrato),
            ("erod14_ainftc", typed_projection.aintc),
            ("erod14_binftc", typed_projection.bintc),
            ("erod14_cinftc", typed_projection.cintc),
            ("erod14_beta", typed_projection.beta),
            ("theta", typed_projection.theta),
            ("erod14_Qj_minus_1", typed_projection.qj_minus_1_m3_s),
            ("erod14_Vj", typed_projection.vj_m),
            ("erod14_Qj", typed_projection.qj_m3_s),
            ("erod14_Fh", typed_projection.fh_m),
            ("erod14_Fp", typed_projection.fp_m),
            ("erod14_case", typed_projection.case_value),
            ("erod14_ssa_soil", typed_projection.ssa_soil),
            ("qostar", typed_projection.route_qostar_m),
            ("xdetst", typed_projection.route_xdetst_m),
            ("lddend", typed_projection.route_lddend_kg),
            ("xu_0002", typed_projection.route_xu_m),
            ("xl_0002", typed_projection.route_xl_m),
            ("ainf_0002", typed_projection.route_ainf),
            ("binf_0002", typed_projection.route_binf),
            ("cinf_0002", typed_projection.route_cinf),
            ("ainftc_0002", typed_projection.route_ainftc),
            ("binftc_0002", typed_projection.route_binftc),
            ("cinftc_0002", typed_projection.route_cinftc),
        ] {
            assert_eq!(
                require_runtime_surface_scalar(runtime_surface, symbol)
                    .expect("surface symbol should be seeded")
                    .to_bits(),
                typed_value.to_bits(),
                "{symbol} should match typed MOFE03 Wave-2 projection"
            );
        }
        assert_eq!(
            require_runtime_surface_scalar(runtime_surface, "erod14_class_count")
                .expect("class count should be seeded")
                .to_bits(),
            usize_to_scalar("erod14_class_count", typed_projection.class_count)
                .expect("typed class count should convert")
                .to_bits()
        );
        for (class_offset, class_projection) in typed_projection.classes.iter().enumerate() {
            let class_index = class_offset + 1;
            for (root, typed_value) in [
                ("erod14_fall", class_projection.fall_m_s),
                ("erod14_frcflw", class_projection.frcflw),
                ("erod14_frac", class_projection.frac),
                ("erod14_fidel", class_projection.fidel),
                ("erod14_tcf1", class_projection.tcf1),
                ("erod14_ssa_class", class_projection.ssa_class),
            ] {
                let symbol = format!("{root}_{class_index:04}");
                assert_eq!(
                    require_runtime_surface_scalar(runtime_surface, symbol.as_str())
                        .expect("class symbol should be seeded")
                        .to_bits(),
                    typed_value.to_bits(),
                    "{symbol} should match typed MOFE03 Wave-2 projection"
                );
            }
        }
    }

    #[test]
    fn typed_day_zero_mofe03_wave2_disabled_seed_matches_surface_seed() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", -3.0),
            ("tmin", -6.9),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.0),
            ("nbrkpt", 2.0),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("wb19_lateral_anisotropy_ratio", 1.0),
            ("wb19_drain_enabled", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.55, true);

        let typed_projection = project_typed_mofe03_wave2(TypedMofe03Wave2Input {
            wave2_enabled: false,
            slplen_m: 0.0,
            qout_m3_s: 0.0,
            qin_m3_s: 0.0,
            efflen_m: None,
            ssa_soil: None,
            beta: 0.0,
            theta: 0.0,
        })
        .expect("disabled typed MOFE03 projection should compute");
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface seeding should compute disabled MOFE03 flag");

        assert!(!typed_projection.wave2_enabled);
        assert_eq!(
            require_runtime_surface_scalar(&runtime_surface, "erod14_wave2_enabled")
                .expect("disabled Wave-2 flag should be seeded")
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert!(
            runtime_surface_symbol_value(&runtime_surface, "erod14_class_count").is_none(),
            "disabled Wave-2 projection should not seed class inputs"
        );
    }

    #[test]
    fn typed_day_zero_wb12_reconciliation_seed_matches_surface_seed() {
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
            ("wb19_lateral_anisotropy_ratio", 1.0),
            ("wb19_drain_enabled", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        let typed_layers = vec![
            require_wb11_layer_seed_inputs(&runtime_surface, 1)
                .expect("test surface should provide WB11 typed layer input"),
        ];
        let typed_storage =
            project_typed_wb11_initial_storage(0.50, ExecutionLane::Daily, &typed_layers)
                .expect("typed storage projection should provide WB12 storage seed");
        let typed_projection = project_typed_wb12_reconciliation_seed(
            0.003,
            0.003,
            typed_storage.totals.soil_water,
            false,
        );
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        assert_eq!(
            typed_projection.rainfall_input_m.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb12_rainfall_input")
                .expect("surface seed should publish rainfall input")
                .to_bits()
        );
        assert_eq!(
            typed_projection.storage_initial_m.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb12_storage_initial")
                .expect("surface seed should publish storage initial")
                .to_bits()
        );
        assert_eq!(
            typed_projection.storage_observed_m.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb12_storage_observed")
                .expect("surface seed should publish storage observed")
                .to_bits()
        );
        assert_eq!(
            typed_projection.precip_input_m.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb12_precip_input")
                .expect("surface seed should publish precip input")
                .to_bits()
        );
        assert_eq!(
            typed_projection.runoff_carryover_m.unwrap_or(-1.0).to_bits(),
            runtime_surface
                .flux_surface
                .get(&BoundarySymbol::from("wb12_runoff_carryover"))
                .expect("single-OFE surface seed should publish daily carryover")
                .as_f64()
                .to_bits()
        );
        assert_eq!(
            typed_projection.forward_solver_lane_enabled.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb20_forward_solver_lane_enabled")
                .expect("surface seed should publish forward solver lane flag")
                .to_bits()
        );
    }

    #[test]
    fn typed_day_zero_wb11_efflen_and_m_match_surface_seed() {
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

        let typed_projection =
            project_typed_wb11_efflen_and_m(None, 50.0, None)
                .expect("typed efflen/m projection should seed from slplen/default m");
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        assert!(typed_projection.efflen_was_defaulted);
        assert!(typed_projection.exponent_was_defaulted);
        assert_eq!(
            typed_projection.efflen_m.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "efflen")
                .expect("surface seed should publish efflen")
                .to_bits()
        );
        assert_eq!(
            typed_projection.exponent_m.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "m")
                .expect("surface seed should publish m")
                .to_bits()
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
    fn typed_day_zero_wb11_initial_storage_projection_matches_surface_seed() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 2.0),
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
            ("wb19_lateral_anisotropy_ratio", 1.0),
            ("wb19_drain_enabled", 0.0),
            ("sat", 0.80),
            ("wb19_dg_0001", 0.25),
            ("wb19_thetfc_0001", 0.30),
            ("wb19_thetdr_0001", 0.12),
            ("ssc_0001", 1.0e-5),
            ("wb19_por_0001", 0.45),
            ("cpm_0001", 0.90),
            ("wb19_dg_0002", 0.55),
            ("wb19_thetfc_0002", 0.28),
            ("wb19_thetdr_0002", 0.10),
            ("ssc_0002", 2.0e-5),
            ("wb19_por_0002", 0.43),
            ("cpm_0002", 0.85),
        ]);

        let typed_inputs = (1..=2)
            .map(|layer_index| require_wb11_layer_seed_inputs(&runtime_surface, layer_index))
            .collect::<Result<Vec<_>, _>>()
            .expect("test surface should provide typed WB11 layer seed inputs");
        let typed_projection =
            project_typed_wb11_initial_storage(0.80, ExecutionLane::Daily, &typed_inputs)
                .expect("typed WB11 initial storage projection should succeed");

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        assert_eq!(
            typed_projection.saturation.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "sat")
                .expect("surface seed should publish sat")
                .to_bits()
        );
        for (layer_offset, projection) in typed_projection.layers.iter().enumerate() {
            let layer_index = layer_offset + 1;
            assert_eq!(
                projection.theta.to_bits(),
                require_runtime_surface_scalar(
                    &runtime_surface,
                    &format!("wb18_perc_theta_{layer_index:04}"),
                )
                .expect("surface seed should publish layer theta")
                .to_bits()
            );
            assert_eq!(
                projection.field_capacity.to_bits(),
                require_runtime_surface_scalar(
                    &runtime_surface,
                    &format!("wb18_perc_fc_{layer_index:04}"),
                )
                .expect("surface seed should publish layer field capacity")
                .to_bits()
            );
            assert_eq!(
                projection.upper_limit.to_bits(),
                require_runtime_surface_scalar(
                    &runtime_surface,
                    &format!("wb18_perc_ul_{layer_index:04}"),
                )
                .expect("surface seed should publish layer upper limit")
                .to_bits()
            );
        }
        assert_eq!(
            typed_projection.totals.soil_water.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb11_soil_water")
                .expect("surface seed should publish total soil water")
                .to_bits()
        );
        assert_eq!(
            typed_projection.totals.field_capacity.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb11_field_capacity")
                .expect("surface seed should publish field capacity")
                .to_bits()
        );
        assert_eq!(
            typed_projection.totals.drainable_storage.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb11_drainable_storage")
                .expect("surface seed should publish drainable storage")
                .to_bits()
        );
        assert_eq!(
            typed_projection.totals.drainage_coefficient.max(1.0e-6).to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "wb11_drainage_coefficient")
                .expect("surface seed should publish drainage coefficient")
            .to_bits()
        );
    }

    #[test]
    fn typed_day_zero_wb11_scalar_frost_depth_refresh_matches_surface_seed() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 2.0),
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
            ("wb19_lateral_anisotropy_ratio", 1.0),
            ("wb19_drain_enabled", 0.0),
            ("sat", 0.80),
            ("frost.runtime_frdp_m", 0.40),
            ("wb19_dg_0001", 0.25),
            ("wb19_thetfc_0001", 0.30),
            ("wb19_thetdr_0001", 0.12),
            ("ssc_0001", 1.0e-5),
            ("wb19_por_0001", 0.45),
            ("cpm_0001", 0.90),
            ("wb19_dg_0002", 0.55),
            ("wb19_thetfc_0002", 0.28),
            ("wb19_thetdr_0002", 0.10),
            ("ssc_0002", 2.0e-5),
            ("wb19_por_0002", 0.43),
            ("cpm_0002", 0.85),
        ]);
        let typed_projection = project_typed_wb11_frozen_depth_refresh(
            Some(0.40),
            &[
                TypedWb11FrozenDepthLayerInput {
                    depth_m: 0.25,
                    fine_frozen_depths_m: None,
                },
                TypedWb11FrozenDepthLayerInput {
                    depth_m: 0.55,
                    fine_frozen_depths_m: None,
                },
            ],
        )
        .expect("typed scalar frost depth refresh should succeed");

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        for (layer_offset, frozen_depth_m) in typed_projection.frozen_depths_m.iter().enumerate() {
            let layer_index = layer_offset + 1;
            assert_eq!(
                frozen_depth_m.to_bits(),
                require_runtime_surface_scalar(
                    &runtime_surface,
                    &format!("wb18_perc_frozen_depth_{layer_index:04}"),
                )
                .expect("surface seed should publish frozen depth")
                .to_bits()
            );
        }
    }

    #[test]
    fn typed_day_zero_wb11_fine_frost_depth_refresh_matches_surface_seed() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 2.0),
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
            ("wb19_lateral_anisotropy_ratio", 1.0),
            ("wb19_drain_enabled", 0.0),
            ("sat", 0.80),
            ("frost.runtime_frdp_m", 0.40),
            ("wb19_dg_0001", 0.25),
            ("wb19_thetfc_0001", 0.30),
            ("wb19_thetdr_0001", 0.12),
            ("ssc_0001", 1.0e-5),
            ("wb19_por_0001", 0.45),
            ("cpm_0001", 0.90),
            ("frost.runtime_nfine_0001", 2.0),
            ("frost.runtime_slfsd_m_0001_0001", 0.02),
            ("frost.runtime_slfsd_m_0001_0002", 0.03),
            ("wb19_dg_0002", 0.55),
            ("wb19_thetfc_0002", 0.28),
            ("wb19_thetdr_0002", 0.10),
            ("ssc_0002", 2.0e-5),
            ("wb19_por_0002", 0.43),
            ("cpm_0002", 0.85),
        ]);
        let typed_projection = project_typed_wb11_frozen_depth_refresh(
            Some(0.40),
            &[
                TypedWb11FrozenDepthLayerInput {
                    depth_m: 0.25,
                    fine_frozen_depths_m: Some(vec![0.02, 0.03]),
                },
                TypedWb11FrozenDepthLayerInput {
                    depth_m: 0.55,
                    fine_frozen_depths_m: None,
                },
            ],
        )
        .expect("typed fine frost depth refresh should succeed");

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        for (layer_offset, frozen_depth_m) in typed_projection.frozen_depths_m.iter().enumerate() {
            let layer_index = layer_offset + 1;
            assert_eq!(
                frozen_depth_m.to_bits(),
                require_runtime_surface_scalar(
                    &runtime_surface,
                    &format!("wb18_perc_frozen_depth_{layer_index:04}"),
                )
                .expect("surface seed should publish frozen depth")
                .to_bits()
            );
        }
    }

    #[test]
    fn typed_day_zero_wb16_ealpha_default_compatibility_matches_surface_seed() {
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

        let typed_projection = project_typed_wb16_ealpha_compatibility(false, false);
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        assert_eq!(
            typed_projection.default_ealpha.unwrap().to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "ealpha")
                .expect("surface seed should publish default ealpha")
                .to_bits()
        );
        assert_eq!(
            typed_projection.seeded_any_day_flag.to_bits(),
            require_runtime_surface_scalar(
                &runtime_surface,
                "wb16_ealpha_compatibility_seed_used",
            )
            .expect("surface seed should publish ealpha compatibility flag")
            .to_bits()
        );
    }

    #[test]
    fn typed_day_zero_wb16_ealpha_producer_matches_surface_seed() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("avgslp", 0.05),
            ("m", 1.5),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.2),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("inrcov", 0.2),
            ("rilcov", 0.1),
            ("rrinit", 0.01),
            ("rspace", 1.0),
            ("width", 0.15),
            ("rtyp", 1.0),
            ("bb", 0.5),
            ("bbb", 0.3),
            ("flivmx", 0.4),
            ("hmax", 1.2),
            ("erod14_wave2_enabled", 0.0),
            ("wb19_lateral_anisotropy_ratio", 1.0),
            ("wb19_drain_enabled", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        let typed_projection =
            project_typed_wb16_ealpha_producer(&TypedWb16EalphaProducerInput {
                exponent_m: 1.5,
                ofes: vec![TypedWb16OfeEalphaInput {
                    avgslp: 0.05,
                    slplen: 50.0,
                    inrcov: 0.2,
                    rilcov: 0.1,
                    rrinit: 0.01,
                    rspace: 1.0,
                    width: 0.15,
                    rtyp: 1.0,
                    cancov: 0.2,
                    bb: 0.5,
                    bbb: 0.3,
                    flivmx: 0.4,
                    hmax: 1.2,
                    rrc: None,
                    canhgt: None,
                }],
            })
            .expect("typed WB16 ealpha producer should compute");
        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should produce WB16 ealpha");

        let ofe = typed_projection.ofes[0];
        assert_eq!(
            ofe.frcteq.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "ofe1_frcteq")
                .expect("surface seed should publish OFE frcteq")
                .to_bits()
        );
        assert_eq!(
            ofe.alpha.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "ofe1_alpha")
                .expect("surface seed should publish OFE alpha")
                .to_bits()
        );
        assert_eq!(
            ofe.alpha.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "alpha")
                .expect("surface seed should publish primary alpha")
                .to_bits()
        );
        assert_eq!(
            typed_projection.ealpha.to_bits(),
            require_runtime_surface_scalar(&runtime_surface, "ealpha")
                .expect("surface seed should publish ealpha")
                .to_bits()
        );
        assert_eq!(
            require_runtime_surface_scalar(
                &runtime_surface,
                "wb16_ealpha_compatibility_seed_used",
            )
            .expect("producer path should clear default-compatibility flag")
            .to_bits(),
            0.0_f64.to_bits()
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
    fn cqr15_wb11_seed_rejects_non_binary_drain_enablement() {
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
            BoundaryValue::scalar(0.5),
        );

        let error = seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect_err("non-binary drain flag must fail WB11 seed");
        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { detail, .. } => {
                assert!(
                    detail.contains("wb19_drain_enabled must be 0 or 1, observed 0.5"),
                    "expected non-binary drain flag guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn cqr15_wb11_seed_rejects_nonpositive_slplen_when_efflen_missing() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 0.0),
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

        let error = seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect_err("missing efflen must require positive slplen");
        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { detail, .. } => {
                assert!(
                    detail.contains("slplen must be > 0.0 when seeding efflen, observed 0"),
                    "expected slplen guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn typed_day_zero_wb11_priestley_taylor_et_seed_matches_surface_seed() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.2),
            ("lai", 1.5),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        let typed_seed =
            project_typed_wb11_priestley_taylor_et_demand(TypedWb11PriestleyTaylorEtInput {
                tmax_c: 12.0,
                tmin_c: 2.0,
                radiation_ly: 43.0,
                soil_albedo: 0.3,
                canopy_cover_fraction: 0.2,
                leaf_area_index: 1.5,
            })
            .expect("typed Priestley-Taylor seed should compute");
        let mut typed_surface = HillslopeWritebackSurface::default();
        publish_wb11_et_demand_seed(&mut typed_surface, typed_seed)
            .expect("typed Priestley-Taylor seed should publish");

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        for symbol in [
            "wb11_et_demand",
            "wb11_et_seed_branch_priestley_taylor",
            "wb11_et_seed_branch_evappm",
        ] {
            assert_eq!(
                require_runtime_surface_scalar(&typed_surface, symbol)
                    .expect("typed seed should publish symbol")
                    .to_bits(),
                require_runtime_surface_scalar(&runtime_surface, symbol)
                    .expect("surface seed should publish symbol")
                    .to_bits(),
                "{symbol} should match between typed ET projection and surface seed"
            );
        }
    }

    #[test]
    fn typed_day_zero_wb11_evappm_et_seed_matches_surface_seed() {
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

        let typed_layers = vec![
            require_wb11_layer_seed_inputs(&runtime_surface, 1)
                .expect("test surface should provide WB11 layer input"),
        ];
        let typed_storage =
            project_typed_wb11_initial_storage(0.80, ExecutionLane::Daily, &typed_layers)
                .expect("typed storage should compute EVAPPM theta store");
        let typed_seed = project_typed_wb11_evappm_et_demand(&TypedWb11EvappmEtInput {
            tmax_c: 20.0,
            tmin_c: 10.0,
            dewpoint_c: 8.0,
            radiation_ly: 20.0,
            potential_radiation_ly: 25.0,
            wind_m_s: 2.0,
            elevation_m: 300.0,
            crop_coefficient: 0.95,
            readily_available_water_fraction: 0.8,
            leaf_area_index: 4.0,
            canopy_height_m: 1.0,
            root_depth_m: 0.2,
            canopy_cover_fraction: 0.0,
            residue_interception_m: 0.000_2,
            layers: vec![TypedWb11EvappmEtLayerInput {
                depth_m: 0.25,
                bottom_depth_m: Some(0.25),
                field_capacity_theta: 0.30,
                residual_theta: 0.12,
                theta_store_m: typed_storage.layers[0].theta,
            }],
        })
        .expect("typed EVAPPM seed should compute");
        let mut typed_surface = HillslopeWritebackSurface::default();
        publish_wb11_et_demand_seed(&mut typed_surface, typed_seed)
            .expect("typed EVAPPM seed should publish");

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("surface WB11 seed should succeed");

        for symbol in [
            "wb11_et_demand",
            "wb11_et_seed_branch_priestley_taylor",
            "wb11_et_seed_branch_evappm",
            "pmet.etorc_mm",
            "pmet.kcbcon",
            "pmet.es_storage_return_m",
            "pmet.ep_m",
        ] {
            assert_eq!(
                require_runtime_surface_scalar(&typed_surface, symbol)
                    .expect("typed seed should publish symbol")
                    .to_bits(),
                require_runtime_surface_scalar(&runtime_surface, symbol)
                    .expect("surface seed should publish symbol")
                    .to_bits(),
                "{symbol} should match between typed ET projection and surface seed"
            );
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
