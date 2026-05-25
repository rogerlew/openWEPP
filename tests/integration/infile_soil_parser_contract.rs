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
