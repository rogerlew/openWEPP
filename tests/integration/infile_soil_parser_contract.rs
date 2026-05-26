use openwepp_input_contract::parsers::soil::{
    ParserMode, SoilDatver, SoilErrorCode, SoilParserOptions, TopologyScope, parse_soil,
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

fn strict() -> SoilParserOptions {
    SoilParserOptions::default()
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
fn compatibility_parses_9002_policy_first_variant() {
    let strict_err = parse_soil(VALID_9002_POLICY_FIRST_COMPAT, strict())
        .expect_err("strict should reject policy-first variant");
    assert_eq!(strict_err.code, SoilErrorCode::SolE006);

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
    let strict_err = parse_soil(COMPAT_QUOTED_HEADER_7778, strict())
        .expect_err("strict should reject quoted 7778 compatibility header");
    assert_eq!(strict_err.code, SoilErrorCode::SolE006);

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
fn strict_rejects_quoted_7778_with_per_ofe_restrictive_rows() {
    let strict_err = parse_soil(COMPAT_QUOTED_HEADER_7778_PER_OFE_RESTRICTIVE, strict())
        .expect_err("strict should reject per-OFE restrictive-row compatibility form");
    assert_eq!(strict_err.code, SoilErrorCode::SolE006);
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
    let strict_err = parse_soil(COMPAT_QUOTED_HEADER_9002_POLICY_FIRST, strict())
        .expect_err("strict should reject quoted 9002 policy-first compatibility form");
    assert_eq!(strict_err.code, SoilErrorCode::SolE006);

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
