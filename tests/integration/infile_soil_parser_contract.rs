use openwepp_hillslope_orchestrator::runtime_inputs::project_typed_soil_wb11_runtime;
use openwepp_input_contract::parsers::soil::{
    DisturbedPolicy, ParserMode, SoilDatver, SoilErrorCode, SoilParserOptions, TopologyScope,
    parse_soil,
};

const VALID_97_5: &str = include_str!("../fixtures/infile/soil/valid_97_5.sol");
const VALID_9002: &str = include_str!("../fixtures/infile/soil/valid_9002.sol");
const VALID_9002_POLICY_FIRST_COMPAT: &str =
    include_str!("../fixtures/infile/soil/valid_9002_policy_first_compat.sol");
const UNKNOWN_DATVER: &str = include_str!("../fixtures/infile/soil/unknown_datver.sol");
const INVALID_LAYER_ARITY_9002: &str =
    include_str!("../fixtures/infile/soil/invalid_layer_arity_9002.sol");
const INVALID_NON_MONOTONE_DEPTH: &str =
    include_str!("../fixtures/infile/soil/invalid_non_monotone_depth.sol");
const ALIAS_97_0: &str = include_str!("../fixtures/infile/soil/alias_97_0.sol");
const COMPAT_QUOTED_HEADER_7778: &str =
    include_str!("../fixtures/infile/soil/compat_quoted_header_7778.sol");
const COMPAT_QUOTED_HEADER_7778_PER_OFE_RESTRICTIVE: &str =
    include_str!("../fixtures/infile/soil/compat_quoted_header_7778_per_ofe_restrictive.sol");
const COMPAT_QUOTED_HEADER_9002_POLICY_FIRST: &str =
    include_str!("../fixtures/infile/soil/compat_quoted_header_9002_policy_first.sol");
const COMPAT_QUOTED_POLICY_ROW_9002: &str =
    include_str!("../fixtures/infile/soil/compat_quoted_policy_row_9002.sol");
const HARMONIC_SSC_9002: &str = "\
9002
Three-layer conductivity normalization probe
1 1
SOIL_H SILT_LOAM 3 0.20 0.55 900000 0.005 4.2 10.5
1 forest silt_loam 0.20 0.001
100 1.25 12.0 1.00 0.30 0.15 35 25 2.0 15 5 0.05 0.45 0.02 1.40 120 0.16 0.31
300 1.30 6.0 2.00 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30
500 1.35 30.0 0.50 0.27 0.13 31 29 1.6 13 9 0.07 0.42 0.04 1.30 100 0.14 0.29
1 500 0.8
";

fn strict() -> SoilParserOptions {
    SoilParserOptions::default()
}

fn assert_close(observed: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(
        (observed - expected).abs() <= tolerance,
        "{label}: observed {observed}, expected {expected}"
    );
}

#[test]
fn strict_parses_97_5_base_profile() {
    let parsed = parse_soil(VALID_97_5, strict()).expect("valid 97.5 profile should parse");

    assert_eq!(parsed.datver, SoilDatver::V97_5);
    assert_eq!(parsed.ntemp, 1);
    assert!(parsed.ksflag);
    assert!(!parsed.datver_alias_applied);
    assert_eq!(parsed.ofes.len(), 1);
    assert_eq!(parsed.ofes[0].layers.len(), 2);
    assert!(parsed.restrictive_layer.is_none());
}

#[test]
fn strict_parses_9002_profile_with_policy_and_restrictive_footer() {
    let parsed = parse_soil(VALID_9002, strict()).expect("valid 9002 profile should parse");

    assert_eq!(parsed.datver, SoilDatver::V9002);
    assert_eq!(parsed.ofes.len(), 1);
    assert_eq!(parsed.ofes[0].layers.len(), 2);
    assert!(parsed.ofes[0].policy.is_some());

    let restrictive = parsed
        .restrictive_layer
        .as_ref()
        .expect("9002 must include restrictive-layer footer");
    assert!(restrictive.slflag);
    assert!((restrictive.ui_bdrkth_mm - 500.0).abs() < 1e-9);
}

#[test]
fn strict_9002_soil_parser_preserves_policy_and_measured_layer_values() {
    let parsed = parse_soil(VALID_9002, strict()).expect("valid 9002 profile should parse");
    let ofe = &parsed.ofes[0];
    let policy = ofe.policy.as_ref().expect("9002 policy row must parse");
    match policy {
        DisturbedPolicy::V9002 {
            ksatadj,
            luse,
            stext,
            ksatfac_mm_h,
            ksatrec_per_day,
        } => {
            assert!(*ksatadj);
            assert_eq!(luse, "forest");
            assert_eq!(stext, "silt_loam");
            assert_close(*ksatfac_mm_h, 0.20, 1.0e-12, "parsed ksatfac");
            assert_close(*ksatrec_per_day, 0.001, 1.0e-12, "parsed ksatrec");
        }
        _ => panic!("9002 fixture must project a V9002 disturbed policy"),
    }

    assert_eq!(ofe.layers.len(), 2);
    assert_close(
        ofe.layers[0].fc_measured.expect("fc field"),
        0.30,
        1.0e-12,
        "parsed layer 1 measured fc",
    );
    assert_close(
        ofe.layers[0].wp_measured.expect("wp field"),
        0.15,
        1.0e-12,
        "parsed layer 1 measured wp",
    );
    assert_close(
        ofe.layers[0].rock_frag_pct,
        5.0,
        1.0e-12,
        "parsed layer 1 rock fragments",
    );
}

#[test]
fn strict_9002_soil_parser_projects_corrected_theta_to_typed_state() {
    let parsed = parse_soil(VALID_9002, strict()).expect("valid 9002 profile should parse");
    let typed = project_typed_soil_wb11_runtime(&parsed)
        .expect("parsed soil should project to typed WB11 runtime state");

    assert_eq!(typed.nsl, 2);
    assert_close(typed.sat, 0.55, 1.0e-12, "typed saturation");
    assert_close(typed.salb, 0.20, 1.0e-12, "typed salb");
    assert!(typed.ksatadj);
    assert_eq!(typed.ksatfac_mm_h, Some(0.20));
    assert_eq!(typed.ksatrec_per_day, Some(0.001));
    assert_close(
        typed
            .restrictive_layer
            .expect("restrictive layer should project")
            .kslast_m_s,
        0.8 / 3.6e6,
        1.0e-15,
        "typed restrictive conductivity",
    );

    let layer1 = typed.layers[0];
    assert_close(layer1.dg_m, 0.2, 1.0e-12, "typed layer 1 dg");
    assert_close(
        layer1.cpm,
        0.970_204_479_065_238_5,
        1.0e-12,
        "typed layer 1 rock-fragment multiplier",
    );
    assert_close(
        layer1.porosity,
        0.496_842_468_867_924_5,
        1.0e-12,
        "typed layer 1 rock-corrected porosity",
    );
    assert_close(
        layer1.thetfc,
        0.281_359_298_928_919_2,
        1.0e-12,
        "typed layer 1 corrected field capacity",
    );
    assert_close(
        layer1.thetdr,
        0.140_679_649_464_459_6,
        1.0e-12,
        "typed layer 1 corrected wilting point",
    );
    assert!(layer1.porosity >= layer1.thetfc);
    assert!(layer1.thetfc >= layer1.thetdr);

    assert_close(
        typed.profile_porosity_cap_mm.expect("profile porosity cap"),
        196.933_353_433_962_28,
        1.0e-9,
        "typed profile porosity cap",
    );
    assert_close(
        typed.profile_fc_store_mm.expect("profile fc store"),
        110.277_729_478_603_25,
        1.0e-9,
        "typed profile field-capacity store",
    );
    assert_close(
        typed.profile_wp_store_mm.expect("profile wp store"),
        55.138_864_739_301_624,
        1.0e-9,
        "typed profile wilting-point store",
    );
}

#[test]
fn strict_9002_soil_parser_projects_harmonic_vertical_ssc_below_top_interval() {
    let parsed = parse_soil(HARMONIC_SSC_9002, strict()).expect("harmonic probe should parse");
    let typed = project_typed_soil_wb11_runtime(&parsed)
        .expect("harmonic probe should project to typed runtime state");

    assert_eq!(typed.nsl, 3);
    let top_layer = typed.layers[0];
    assert_close(
        top_layer.ssc_m_s,
        12.0 / 3.6e6,
        1.0e-15,
        "top interval vertical ssc uses top source conductivity",
    );
    assert_close(
        top_layer.lateral_ssh_m_s,
        18.0 / 3.6e6,
        1.0e-15,
        "top interval lateral ssh remains arithmetic over source conductivity",
    );

    let second_layer = typed.layers[1];
    let rejected_arithmetic_vertical_m_s = 18.0 / 3.6e6;
    assert_close(
        second_layer.ssc_m_s,
        10.0 / 3.6e6,
        1.0e-15,
        "second normalized interval vertical ssc is harmonic",
    );
    assert!(
        (second_layer.ssc_m_s - rejected_arithmetic_vertical_m_s).abs() > 1.0e-6,
        "second interval vertical ssc must not regress to arithmetic averaging"
    );
    assert_close(
        second_layer.lateral_ssh_m_s,
        13.5 / 3.6e6,
        1.0e-15,
        "second normalized interval lateral ssh stays arithmetic",
    );
}

#[test]
fn compatibility_parses_9002_policy_first_variant() {
    let strict_parsed = parse_soil(VALID_9002_POLICY_FIRST_COMPAT, strict())
        .expect("strict should accept canonical policy-first variant");
    assert_eq!(strict_parsed.datver, SoilDatver::V9002);
    assert_eq!(strict_parsed.ofes.len(), 1);
    assert_eq!(strict_parsed.ofes[0].layers.len(), 2);
    assert!(strict_parsed.ofes[0].policy.is_some());

    let compat_options = SoilParserOptions {
        mode: ParserMode::Compatibility,
        allow_legacy_aliases: true,
        expected_topology_count: None,
        topology_scope: None,
    };
    let parsed = parse_soil(VALID_9002_POLICY_FIRST_COMPAT, compat_options)
        .expect("compat should accept policy-first variant");

    assert_eq!(parsed.datver, SoilDatver::V9002);
    assert_eq!(parsed.ofes.len(), 1);
    assert_eq!(parsed.ofes[0].layers.len(), 2);
    assert!(parsed.ofes[0].policy.is_some());
}

#[test]
fn strict_rejects_unknown_datver_with_sol_e_003() {
    let err = parse_soil(UNKNOWN_DATVER, strict()).expect_err("unknown datver must fail");
    assert_eq!(err.code, SoilErrorCode::SolE003);
}

#[test]
fn strict_rejects_layer_arity_mismatch_with_sol_e_006() {
    let err = parse_soil(INVALID_LAYER_ARITY_9002, strict())
        .expect_err("invalid layer row arity must fail");
    assert_eq!(err.code, SoilErrorCode::SolE006);
}

#[test]
fn strict_rejects_non_monotone_layer_depth_with_sol_e_009() {
    let err = parse_soil(INVALID_NON_MONOTONE_DEPTH, strict())
        .expect_err("non-monotone depth sequence must fail");
    assert_eq!(err.code, SoilErrorCode::SolE009);
}

#[test]
fn compatibility_accepts_explicit_datver_alias_only_when_enabled() {
    let strict_err = parse_soil(ALIAS_97_0, strict()).expect_err("strict should reject 97.0 alias");
    assert_eq!(strict_err.code, SoilErrorCode::SolE003);

    let compat_no_alias = SoilParserOptions {
        mode: ParserMode::Compatibility,
        allow_legacy_aliases: false,
        expected_topology_count: None,
        topology_scope: None,
    };
    let compat_err = parse_soil(ALIAS_97_0, compat_no_alias)
        .expect_err("compat without alias map should reject");
    assert_eq!(compat_err.code, SoilErrorCode::SolE003);

    let compat_with_alias = SoilParserOptions {
        mode: ParserMode::Compatibility,
        allow_legacy_aliases: true,
        expected_topology_count: None,
        topology_scope: None,
    };
    let parsed = parse_soil(ALIAS_97_0, compat_with_alias)
        .expect("compat with explicit alias map should accept 97.0");

    assert_eq!(parsed.datver, SoilDatver::V97_5);
    assert!(parsed.datver_alias_applied);
}

#[test]
fn returns_sol_e_007_on_cross_file_topology_count_mismatch() {
    let options = SoilParserOptions {
        mode: ParserMode::Strict,
        allow_legacy_aliases: false,
        expected_topology_count: Some(2),
        topology_scope: Some(TopologyScope::Hillslope),
    };

    let err = parse_soil(VALID_97_5, options)
        .expect_err("ntemp mismatch against topology should return cross-file error");
    assert_eq!(err.code, SoilErrorCode::SolE007);
}

#[test]
fn compatibility_accepts_quoted_7778_soil_header_form() {
    let strict_parsed = parse_soil(COMPAT_QUOTED_HEADER_7778, strict())
        .expect("strict should accept quoted 7778 header");
    assert_eq!(strict_parsed.datver, SoilDatver::V7778);
    assert_eq!(strict_parsed.ofes.len(), 1);
    assert_eq!(strict_parsed.ofes[0].layers.len(), 2);
    assert!((strict_parsed.ofes[0].avke - 0.0).abs() < 1e-12);

    let compat_options = SoilParserOptions {
        mode: ParserMode::Compatibility,
        allow_legacy_aliases: true,
        expected_topology_count: None,
        topology_scope: None,
    };
    let parsed = parse_soil(COMPAT_QUOTED_HEADER_7778, compat_options)
        .expect("compatibility mode should accept quoted 7778 header");

    assert_eq!(parsed.datver, SoilDatver::V7778);
    assert_eq!(parsed.ofes.len(), 1);
    assert_eq!(parsed.ofes[0].layers.len(), 2);
    assert_eq!(
        parsed.ofes[0].slid,
        "Hummington gravelly loam, 50 to 75 percent slopes"
    );
    assert_eq!(parsed.ofes[0].texid, "GR-L");
    assert!((parsed.ofes[0].avke - 0.0).abs() < 1e-12);
}

#[test]
fn strict_accepts_quoted_7778_with_per_ofe_restrictive_rows() {
    let parsed = parse_soil(COMPAT_QUOTED_HEADER_7778_PER_OFE_RESTRICTIVE, strict())
        .expect("strict should accept per-OFE restrictive-row form");

    assert_eq!(parsed.datver, SoilDatver::V7778);
    assert_eq!(parsed.ofes.len(), 2);
    let restrictive = parsed
        .restrictive_layer
        .as_ref()
        .expect("strict normalization should produce restrictive layer");
    assert!(restrictive.slflag);
    assert!((restrictive.ui_bdrkth_mm - 10000.0).abs() < 1e-9);
    assert!((restrictive.kslast_mm_h - 0.001).abs() < 1e-9);
}

#[test]
fn compatibility_accepts_quoted_7778_with_per_ofe_restrictive_rows() {
    let compat_options = SoilParserOptions {
        mode: ParserMode::Compatibility,
        allow_legacy_aliases: true,
        expected_topology_count: None,
        topology_scope: None,
    };
    let parsed = parse_soil(
        COMPAT_QUOTED_HEADER_7778_PER_OFE_RESTRICTIVE,
        compat_options,
    )
    .expect("compatibility mode should accept per-OFE restrictive-row form");

    assert_eq!(parsed.datver, SoilDatver::V7778);
    assert_eq!(parsed.ofes.len(), 2);
    let restrictive = parsed
        .restrictive_layer
        .as_ref()
        .expect("compatibility normalization should produce restrictive layer");
    assert!(restrictive.slflag);
    assert!((restrictive.ui_bdrkth_mm - 10000.0).abs() < 1e-9);
    assert!((restrictive.kslast_mm_h - 0.001).abs() < 1e-9);
}

#[test]
fn compatibility_accepts_quoted_9002_policy_first_header_form() {
    let strict_parsed = parse_soil(COMPAT_QUOTED_HEADER_9002_POLICY_FIRST, strict())
        .expect("strict should accept quoted 9002 policy-first header");
    assert_eq!(strict_parsed.datver, SoilDatver::V9002);
    assert_eq!(strict_parsed.ofes.len(), 1);
    assert_eq!(strict_parsed.ofes[0].layers.len(), 2);
    assert_eq!(strict_parsed.ofes[0].slid, "SOIL B with spaces");
    assert_eq!(strict_parsed.ofes[0].texid, "CLAY LOAM");
    assert!((strict_parsed.ofes[0].avke - 0.0).abs() < 1e-12);

    let compat_options = SoilParserOptions {
        mode: ParserMode::Compatibility,
        allow_legacy_aliases: true,
        expected_topology_count: None,
        topology_scope: None,
    };
    let parsed = parse_soil(COMPAT_QUOTED_HEADER_9002_POLICY_FIRST, compat_options)
        .expect("compatibility mode should accept quoted 9002 policy-first header");

    assert_eq!(parsed.datver, SoilDatver::V9002);
    assert_eq!(parsed.ofes.len(), 1);
    assert_eq!(parsed.ofes[0].layers.len(), 2);
    assert_eq!(parsed.ofes[0].slid, "SOIL B with spaces");
    assert_eq!(parsed.ofes[0].texid, "CLAY LOAM");
    assert!((parsed.ofes[0].avke - 0.0).abs() < 1e-12);
}

#[test]
fn strict_accepts_quoted_9002_policy_row_with_whitespace_luse() {
    let parsed = parse_soil(COMPAT_QUOTED_POLICY_ROW_9002, strict())
        .expect("strict should accept quoted 9002 policy rows");

    assert_eq!(parsed.datver, SoilDatver::V9002);
    assert_eq!(parsed.ofes.len(), 1);
    assert_eq!(
        parsed.ofes[0].slid,
        "Andic Xerochrepts, 60 to 90 percent slopes"
    );
    assert_eq!(parsed.ofes[0].texid, "ASHY-L");
    assert!((parsed.ofes[0].avke - 0.0).abs() < 1e-12);
}

#[test]
fn compatibility_accepts_quoted_9002_policy_row_with_whitespace_luse() {
    let compat_options = SoilParserOptions {
        mode: ParserMode::Compatibility,
        allow_legacy_aliases: true,
        expected_topology_count: None,
        topology_scope: None,
    };
    let parsed = parse_soil(COMPAT_QUOTED_POLICY_ROW_9002, compat_options)
        .expect("compatibility mode should accept quoted 9002 policy rows");

    assert_eq!(parsed.datver, SoilDatver::V9002);
    assert_eq!(parsed.ofes.len(), 1);
    assert_eq!(
        parsed.ofes[0].slid,
        "Andic Xerochrepts, 60 to 90 percent slopes"
    );
    assert_eq!(parsed.ofes[0].texid, "ASHY-L");
    assert!((parsed.ofes[0].avke - 0.0).abs() < 1e-12);
}
