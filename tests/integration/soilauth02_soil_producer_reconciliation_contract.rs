use openwepp_input_contract::parsers::soil::{
    DisturbedPolicy, ParserMode, SoilDatver, SoilParserOptions, parse_soil,
};

const CANONICAL_9002_POLICY_FIRST: &str =
    include_str!("../fixtures/infile/soil/compat_quoted_header_9002_policy_first.sol");
const CANONICAL_7778_PER_OFE_RESTRICTIVE: &str =
    include_str!("../fixtures/infile/soil/compat_quoted_header_7778_per_ofe_restrictive.sol");
const CANONICAL_9002_DOUBLE_QUOTED_POLICY: &str =
    include_str!("../fixtures/infile/soil/canonical_9002_double_quoted_policy.sol");

fn strict() -> SoilParserOptions {
    SoilParserOptions::default()
}

fn compat() -> SoilParserOptions {
    SoilParserOptions {
        mode: ParserMode::Compatibility,
        allow_legacy_aliases: true,
        expected_topology_count: None,
        topology_scope: None,
    }
}

#[test]
fn canonical_9002_policy_first_no_avke_parses_in_strict_and_compat() {
    for options in [strict(), compat()] {
        let parsed = parse_soil(CANONICAL_9002_POLICY_FIRST, options)
            .expect("canonical 9002 policy-first fixture should parse");
        assert_eq!(parsed.datver, SoilDatver::V9002);
        assert_eq!(parsed.ofes.len(), 1);
        assert!((parsed.ofes[0].avke - 0.0).abs() < 1e-12);
        assert!(parsed.ofes[0].policy.is_some());
    }
}

#[test]
fn canonical_7778_per_ofe_restrictive_parses_in_strict_and_compat() {
    for options in [strict(), compat()] {
        let parsed = parse_soil(CANONICAL_7778_PER_OFE_RESTRICTIVE, options)
            .expect("canonical 7778 per-ofe restrictive fixture should parse");
        assert_eq!(parsed.datver, SoilDatver::V7778);
        assert_eq!(parsed.ofes.len(), 2);
        let restrictive = parsed
            .restrictive_layer
            .as_ref()
            .expect("restrictive row must normalize to profile-level state");
        assert!(restrictive.slflag);
        assert!((restrictive.ui_bdrkth_mm - 10000.0).abs() < 1e-9);
        assert!((restrictive.kslast_mm_h - 0.001).abs() < 1e-9);
    }
}

#[test]
fn canonical_double_quoted_policy_tokens_parse_in_strict_and_compat() {
    for options in [strict(), compat()] {
        let parsed = parse_soil(CANONICAL_9002_DOUBLE_QUOTED_POLICY, options)
            .expect("double-quoted policy tokens should parse");
        assert_eq!(parsed.datver, SoilDatver::V9002);
        let policy = parsed.ofes[0]
            .policy
            .as_ref()
            .expect("9002 policy row must be present");
        match policy {
            DisturbedPolicy::V9002 { luse, stext, .. } => {
                assert_eq!(luse, "farmer's field");
                assert_eq!(stext, "silt loam");
            }
            _ => panic!("expected 9002 policy variant"),
        }
    }
}
