use std::path::PathBuf;

use openwepp_input_contract::parsers::slope::{
    DatverSource, DistanceMode, SlopeParserError, SlopeParserMode, SlopeParserOptions,
    parse_slope_file, parse_slope_str,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/infile/slope")
        .join(name)
}

#[test]
fn strict_mode_accepts_canonical_datver_profile() {
    let parsed = parse_slope_file(
        &fixture_path("strict_valid_canonical.slp"),
        SlopeParserOptions::strict(),
    )
    .expect("strict canonical parse should succeed");

    assert_eq!(parsed.datver_source, DatverSource::Header);
    assert_eq!(parsed.ofe_count, 2);
    assert_eq!(parsed.ofes[0].elevation, None);
    assert_eq!(parsed.ofes[1].elevation, None);
    assert_eq!(parsed.ofes[0].distance_mode, DistanceMode::Normalized);
    assert_eq!(parsed.ofes[1].distance_mode, DistanceMode::Normalized);
}

#[test]
fn strict_mode_accepts_peridot_2023_3_profile() {
    let parsed = parse_slope_file(
        &fixture_path("strict_valid_peridot_2023_3.slp"),
        SlopeParserOptions::strict(),
    )
    .expect("strict peridot 2023.3 parse should succeed");

    assert_eq!(parsed.datver_source, DatverSource::Header);
    assert!((parsed.datver - 2023.3).abs() < 1e-9);
    assert_eq!(parsed.ofe_count, 1);
    assert_eq!(parsed.ofes[0].distance_mode, DistanceMode::Normalized);
    assert_eq!(parsed.ofes[0].elevation, Some(1450.0));
}

#[test]
fn strict_mode_rejects_missing_datver_header() {
    let error = parse_slope_file(
        &fixture_path("compat_legacy_no_datver.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("strict mode must reject missing datver header");

    assert!(matches!(error, SlopeParserError::MissingDatverHeaderError));
}

#[test]
fn compatibility_mode_accepts_missing_datver_header() {
    let parsed = parse_slope_file(
        &fixture_path("compat_legacy_no_datver.slp"),
        SlopeParserOptions::compatibility(),
    )
    .expect("compat mode should accept missing datver");

    assert_eq!(parsed.datver_source, DatverSource::LegacyCompatImputed);
    assert_eq!(parsed.ofe_count, 2);
}

#[test]
fn strict_mode_rejects_non_canonical_datver() {
    let src = "96.9\n1\n180 20\n2 100\n0 0.05 1 0.05\n";
    let error = parse_slope_str(src, SlopeParserOptions::strict())
        .expect_err("strict mode must reject non-97.5 datver");

    match error {
        SlopeParserError::UnsupportedDatver {
            datver,
            mode,
            canonical_datver,
            ..
        } => {
            assert!((datver - 96.9).abs() < 1e-9);
            assert_eq!(mode, SlopeParserMode::Strict);
            assert!((canonical_datver - 97.5).abs() < 1e-9);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn compatibility_mode_accepts_legacy_threshold_datver() {
    let src = "91.5\n1\n180 20\n2 100\n0 0.05 1 0.05\n";
    let parsed = parse_slope_str(src, SlopeParserOptions::compatibility())
        .expect("compat mode should accept datver at threshold");

    assert_eq!(parsed.ofe_count, 1);
}

#[test]
fn compatibility_mode_rejects_datver_below_threshold() {
    let src = "91.49\n1\n180 20\n2 100\n0 0.05 1 0.05\n";
    let error = parse_slope_str(src, SlopeParserOptions::compatibility())
        .expect_err("compat mode must reject datver below threshold");

    assert!(matches!(error, SlopeParserError::UnsupportedDatver { .. }));
}

#[test]
fn parser_rejects_mixed_distance_mode() {
    let error = parse_slope_file(
        &fixture_path("invalid_mixed_distance_mode.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("mixed distance mode must fail");

    assert!(matches!(
        error,
        SlopeParserError::DistanceModeMixError { .. }
    ));
}

#[test]
fn parser_rejects_missing_terminal_endpoint() {
    let error = parse_slope_file(
        &fixture_path("invalid_missing_endpoint.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("missing terminal endpoint must fail");

    assert!(matches!(
        error,
        SlopeParserError::EndpointConstraintError { .. }
    ));
}

#[test]
fn parser_rejects_cross_ofe_boundary_slope_discontinuity() {
    let error = parse_slope_file(
        &fixture_path("invalid_cross_ofe_boundary.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("cross-OFE discontinuity must fail");

    assert!(matches!(
        error,
        SlopeParserError::CrossOfeBoundaryError { .. }
    ));
}

#[test]
fn parser_rejects_nslpts_less_than_two() {
    let error = parse_slope_file(
        &fixture_path("invalid_nslpts_lt2.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("nslpts < 2 must fail");

    match error {
        SlopeParserError::FieldRangeError { field, .. } => assert_eq!(field, "nslpts"),
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn parser_rejects_non_numeric_tokens() {
    let error = parse_slope_file(
        &fixture_path("invalid_token.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("non-numeric token must fail");

    assert!(matches!(error, SlopeParserError::TokenParseError { .. }));
}

#[test]
fn parser_rejects_peridot_metadata_arity_violation() {
    let error = parse_slope_file(
        &fixture_path("invalid_peridot_metadata_arity.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("peridot metadata arity violation must fail");

    assert!(matches!(error, SlopeParserError::RecordCountError { .. }));
}

#[test]
fn parser_rejects_peridot_pair_cardinality_violation() {
    let error = parse_slope_file(
        &fixture_path("invalid_peridot_pair_cardinality.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("peridot pair cardinality violation must fail");

    assert!(matches!(error, SlopeParserError::RecordCountError { .. }));
}

#[test]
fn parser_rejects_peridot_non_numeric_pair_token() {
    let error = parse_slope_file(
        &fixture_path("invalid_peridot_pair_token.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("peridot non-numeric pair token must fail");

    assert!(matches!(error, SlopeParserError::TokenParseError { .. }));
}

#[test]
fn parse_file_returns_missing_file_error() {
    let error = parse_slope_file(
        &fixture_path("does_not_exist.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("missing file must produce typed error");

    assert!(matches!(error, SlopeParserError::InputFileMissing { .. }));
}
