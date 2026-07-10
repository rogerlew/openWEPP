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
fn strict_mode_rejects_shared_geometry_multi_ofe_form() {
    let error = parse_slope_file(
        &fixture_path("compat_shared_geom_multi_ofe.slp"),
        SlopeParserOptions::strict(),
    )
    .expect_err("strict mode must reject shared-geometry compatibility form");

    assert!(matches!(
        error,
        SlopeParserError::TokenParseError { .. } | SlopeParserError::RecordCountError { .. }
    ));
}

#[test]
fn compatibility_mode_accepts_shared_geometry_multi_ofe_form() {
    let parsed = parse_slope_file(
        &fixture_path("compat_shared_geom_multi_ofe.slp"),
        SlopeParserOptions::compatibility(),
    )
    .expect("compatibility mode should accept shared-geometry compatibility form");

    assert_eq!(parsed.ofe_count, 2);
    assert_eq!(parsed.ofes[0].distance_mode, DistanceMode::Normalized);
    assert_eq!(parsed.ofes[1].distance_mode, DistanceMode::Normalized);
    assert!((parsed.ofes[0].azm - 263.1992).abs() < 1e-9);
    assert!((parsed.ofes[1].azm - 263.1992).abs() < 1e-9);
    assert!((parsed.ofes[0].fwidth - 86.9).abs() < 1e-9);
    assert!((parsed.ofes[1].fwidth - 86.9).abs() < 1e-9);
}

#[test]
fn compatibility_mode_accepts_near_endpoint_terminal_distance() {
    let src = "97.5\n2\n180.0 25.0\n2 60.0\n0.0000, 0.1200 0.9996, 0.1200\n2 60.0\n0.0000, 0.1200 1.0000, 0.1200\n";
    let parsed = parse_slope_str(src, SlopeParserOptions::compatibility())
        .expect("compatibility mode should accept near-endpoint terminal closure");

    assert_eq!(parsed.ofe_count, 2);
    assert_eq!(parsed.ofes[0].distance_mode, DistanceMode::Normalized);
    assert_eq!(parsed.ofes[1].distance_mode, DistanceMode::Normalized);
}

#[test]
fn compatibility_mode_accepts_cross_ofe_boundary_discontinuity() {
    let src = "97.5\n2\n180.0 25.0\n2 60.0\n0.0000, 0.1200 1.0000, 0.4200\n2 60.0\n0.0000, 0.1100 1.0000, 0.2100\n";
    let parsed = parse_slope_str(src, SlopeParserOptions::compatibility())
        .expect("compatibility mode should not hard-fail cross-OFE boundary mismatch");

    assert_eq!(parsed.ofe_count, 2);
    assert!((parsed.ofes[0].points[1].slpinp - 0.4200).abs() < 1e-9);
    assert!((parsed.ofes[1].points[0].slpinp - 0.1100).abs() < 1e-9);
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

#[test]
fn slope_parser_error_display_strings_remain_stable() {
    let cases = [
        (
            SlopeParserError::InputFileMissing {
                path: PathBuf::from("missing.slp"),
            },
            "missing slope file: missing.slp",
        ),
        (
            SlopeParserError::InputFileOpenError {
                path: PathBuf::from("blocked.slp"),
                message: "permission denied".to_string(),
            },
            "failed to open slope file blocked.slp: permission denied",
        ),
        (
            SlopeParserError::TokenParseError {
                line: 2,
                column: 4,
                token: "abc".to_string(),
                expected: "real",
            },
            "token parse error at line 2, column 4: expected real, got 'abc'",
        ),
        (
            SlopeParserError::RecordCountError {
                context: "missing nelem after datver".to_string(),
            },
            "record count error: missing nelem after datver",
        ),
        (
            SlopeParserError::MissingDatverHeaderError,
            "missing required datver header in strict mode",
        ),
        (
            SlopeParserError::UnsupportedDatver {
                datver: 96.9,
                mode: SlopeParserMode::Strict,
                canonical_datver: 97.5,
                compatibility_min_datver: 91.5,
            },
            "unsupported datver 96.9 for mode Strict (strict requires 97.5 or 2023.3, compat min 91.5)",
        ),
        (
            SlopeParserError::FieldRangeError {
                field: "fwidth",
                value: 0.0,
                expected: "> 0 and finite",
                guard_id: "G-SLP-003",
                ofe_index: Some(1),
            },
            "field range error [G-SLP-003] for fwidth in OFE 2: got 0, expected > 0 and finite",
        ),
        (
            SlopeParserError::FieldRangeError {
                field: "fwidth",
                value: 0.0,
                expected: "> 0 and finite",
                guard_id: "G-SLP-003",
                ofe_index: None,
            },
            "field range error [G-SLP-003] for fwidth: got 0, expected > 0 and finite",
        ),
        (
            SlopeParserError::DistanceModeMixError {
                ofe_index: 0,
                message: "mixed scale".to_string(),
            },
            "distance mode mix in OFE 1: mixed scale",
        ),
        (
            SlopeParserError::EndpointConstraintError {
                ofe_index: 0,
                message: "bad endpoint".to_string(),
            },
            "endpoint constraint in OFE 1: bad endpoint",
        ),
        (
            SlopeParserError::CrossOfeBoundaryError {
                left_ofe_index: 0,
                right_ofe_index: 1,
                left_terminal_slope: 0.42,
                right_initial_slope: 0.11,
                tolerance: 0.000_001,
            },
            "cross-OFE boundary slope mismatch OFE 1 -> OFE 2 (0.42 vs 0.11, tol 0.000001)",
        ),
        (
            SlopeParserError::InvariantViolation {
                guard_id: "G-SLP-007",
                message: "missing terminal slope".to_string(),
            },
            "invariant violation [G-SLP-007]: missing terminal slope",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn parse_slope_str_reports_top_level_record_count_errors() {
    let empty =
        parse_slope_str("", SlopeParserOptions::strict()).expect_err("empty input should fail");
    assert_eq!(
        empty.to_string(),
        "record count error: slope file contains no numeric records"
    );

    let missing_nelem = parse_slope_str("97.5\n", SlopeParserOptions::strict())
        .expect_err("datver without nelem should fail");
    assert_eq!(
        missing_nelem.to_string(),
        "record count error: missing nelem after datver"
    );

    let trailing = parse_slope_str(
        "97.5\n1\n180 20\n2 100\n0 0.05 1 0.05\nextra\n",
        SlopeParserOptions::strict(),
    )
    .expect_err("trailing token should fail");
    assert_eq!(
        trailing.to_string(),
        "record count error: unexpected trailing tokens beginning at line 6, column 1"
    );
}

#[test]
fn default_options_match_strict_mode_and_directory_open_errors_are_typed() {
    assert_eq!(SlopeParserOptions::default(), SlopeParserOptions::strict());

    let error = parse_slope_file(&fixture_path(""), SlopeParserOptions::strict())
        .expect_err("directory path should produce open error, not missing file");

    assert!(matches!(error, SlopeParserError::InputFileOpenError { .. }));
}

#[test]
fn parser_rejects_nonpositive_counts_and_widths() {
    let zero_nelem = parse_slope_str("97.5\n0\n", SlopeParserOptions::strict())
        .expect_err("zero nelem should fail");
    match zero_nelem {
        SlopeParserError::FieldRangeError {
            field, guard_id, ..
        } => {
            assert_eq!(field, "nelem");
            assert_eq!(guard_id, "G-SLP-002");
        }
        other => panic!("unexpected error: {other:?}"),
    }

    let zero_per_ofe_width = parse_slope_str(
        "97.5\n1\n180 0\n2 100\n0 0.05 1 0.05\n",
        SlopeParserOptions::strict(),
    )
    .expect_err("zero per-OFE width should fail");
    match zero_per_ofe_width {
        SlopeParserError::FieldRangeError {
            field, ofe_index, ..
        } => {
            assert_eq!(field, "fwidth");
            assert_eq!(ofe_index, Some(0));
        }
        other => panic!("unexpected error: {other:?}"),
    }

    let zero_shared_width = parse_slope_str(
        "2\n180 0\n2 60\n0 0.1 1 0.1\n2 60\n0 0.1 1 0.1\n",
        SlopeParserOptions::compatibility(),
    )
    .expect_err("zero shared width should fail");
    assert!(matches!(
        zero_shared_width,
        SlopeParserError::FieldRangeError {
            field: "fwidth",
            ..
        }
    ));
}

#[test]
fn parser_rejects_nonfinite_geometry_fields() {
    let nonfinite_elevation = parse_slope_str(
        "2023.3\n1\n180 20 NaN\n2 100\n0 0.05 1 0.05\n",
        SlopeParserOptions::strict(),
    )
    .expect_err("nonfinite peridot elevation should fail");
    assert!(matches!(
        nonfinite_elevation,
        SlopeParserError::FieldRangeError {
            field: "elevation",
            guard_id: "G-SLP-009",
            ..
        }
    ));

    let nonfinite_slplen = parse_slope_str(
        "97.5\n1\n180 20\n2 NaN\n0 0.05 1 0.05\n",
        SlopeParserOptions::strict(),
    )
    .expect_err("nonfinite slope length should fail");
    assert!(matches!(
        nonfinite_slplen,
        SlopeParserError::FieldRangeError {
            field: "slplen",
            guard_id: "G-SLP-004",
            ..
        }
    ));

    let nonfinite_xinput = parse_slope_str(
        "97.5\n1\n180 20\n2 100\nNaN 0.05 1 0.05\n",
        SlopeParserOptions::strict(),
    )
    .expect_err("nonfinite xinput should fail");
    assert!(matches!(
        nonfinite_xinput,
        SlopeParserError::FieldRangeError {
            field: "xinput",
            guard_id: "G-SLP-006",
            ..
        }
    ));

    let nonfinite_slpinp = parse_slope_str(
        "97.5\n1\n180 20\n2 100\n0 NaN 1 0.05\n",
        SlopeParserOptions::strict(),
    )
    .expect_err("nonfinite slpinp should fail");
    assert!(matches!(
        nonfinite_slpinp,
        SlopeParserError::FieldRangeError {
            field: "slpinp",
            guard_id: "G-SLP-006",
            ..
        }
    ));
}

#[test]
fn parser_rejects_start_and_monotonic_endpoint_violations() {
    let nonzero_start = parse_slope_str(
        "97.5\n1\n180 20\n2 100\n0.1 0.05 1 0.05\n",
        SlopeParserOptions::strict(),
    )
    .expect_err("first xinput must be zero");
    assert!(matches!(
        nonzero_start,
        SlopeParserError::EndpointConstraintError { .. }
    ));

    let decreasing = parse_slope_str(
        "97.5\n1\n180 20\n3 100\n0 0.05 0.8 0.05 0.7 0.05\n",
        SlopeParserOptions::strict(),
    )
    .expect_err("decreasing xinput should fail");
    assert!(matches!(
        decreasing,
        SlopeParserError::EndpointConstraintError { .. }
    ));
}

#[test]
fn parser_accepts_absolute_distance_mode_without_fractional_mix() {
    let parsed = parse_slope_str(
        "97.5\n1\n180 20\n3 100\n0 0.05 50 0.05 100 0.05\n",
        SlopeParserOptions::strict(),
    )
    .expect("absolute distance mode without fractional mix should parse");

    assert_eq!(parsed.ofes[0].distance_mode, DistanceMode::Absolute);
    assert_eq!(parsed.ofes[0].points.len(), 3);
}
