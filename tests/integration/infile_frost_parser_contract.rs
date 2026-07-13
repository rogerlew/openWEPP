use std::path::PathBuf;

use openwepp_input_contract::parsers::frost::{
    FrostParseError, FrostWarningCode, ParseMode, parse_frost_from_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/infile/frost")
        .join(name)
}

#[test]
fn strict_mode_parses_two_line_frost_file() {
    let parsed =
        parse_frost_from_path(fixture_path("strict_valid_two_line.txt"), ParseMode::Strict)
            .expect("strict valid two-line frost file should parse");

    assert!(parsed.frost_file_present);
    assert!(parsed.line2_present);
    assert_eq!(parsed.wint_red, 1);
    assert_eq!(parsed.fine_top, 10);
    assert_eq!(parsed.fine_bot, 8);
    assert!((parsed.ksnowf - 0.10).abs() < 1e-12);
    assert!((parsed.kfactor3 - 0.50).abs() < 1e-12);
    assert!(!parsed.legacy_clamp_applied);
    assert!(parsed.legacy_clamp_fields.is_empty());
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_mode_rejects_missing_line2_record() {
    let err = parse_frost_from_path(fixture_path("compat_line2_missing.txt"), ParseMode::Strict)
        .expect_err("strict mode must reject missing line2");

    assert_eq!(err.contract_error_id(), "FROST-E-002");
    assert!(matches!(err, FrostParseError::FrostE002 { .. }));
}

#[test]
fn compatibility_mode_defaults_missing_line2_with_warnings() {
    let parsed = parse_frost_from_path(
        fixture_path("compat_line2_missing.txt"),
        ParseMode::Compatibility,
    )
    .expect("compat mode should allow missing line2");

    assert!(parsed.frost_file_present);
    assert!(!parsed.line2_present);
    assert!(parsed.legacy_clamp_applied);
    assert!(parsed.legacy_clamp_fields.contains(&"ksnowf"));
    assert!(parsed.legacy_clamp_fields.contains(&"kfactor3"));

    let codes: Vec<_> = parsed.warnings.iter().map(|warning| warning.code).collect();
    assert!(codes.contains(&FrostWarningCode::FrostW002));
    assert!(codes.contains(&FrostWarningCode::FrostW003));
}

#[test]
fn strict_mode_rejects_line1_arity_violation() {
    let err = parse_frost_from_path(
        fixture_path("strict_line1_arity_invalid.txt"),
        ParseMode::Strict,
    )
    .expect_err("line1 arity violation must fail");

    assert_eq!(err.contract_error_id(), "FROST-E-001");
    assert!(matches!(err, FrostParseError::FrostE001 { .. }));
}

#[test]
fn strict_mode_rejects_line2_token_parse_error() {
    let err = parse_frost_from_path(
        fixture_path("strict_line2_token_invalid.txt"),
        ParseMode::Strict,
    )
    .expect_err("line2 token parse failure must fail in strict mode");

    assert_eq!(err.contract_error_id(), "FROST-E-002");
    assert!(matches!(err, FrostParseError::FrostE002 { .. }));
}

#[test]
fn strict_mode_rejects_out_of_range_values() {
    let err = parse_frost_from_path(fixture_path("strict_out_of_range.txt"), ParseMode::Strict)
        .expect_err("out-of-range values must fail in strict mode");

    assert_eq!(err.contract_error_id(), "FROST-E-004");
    assert!(matches!(err, FrostParseError::FrostE004 { .. }));
}

#[test]
fn compatibility_mode_clamps_out_of_range_values_and_marks_provenance() {
    let parsed = parse_frost_from_path(
        fixture_path("compat_out_of_range_clamped.txt"),
        ParseMode::Compatibility,
    )
    .expect("compat mode should clamp out-of-range values");

    assert_eq!(parsed.wint_red, 1);
    assert_eq!(parsed.fine_top, 10);
    assert_eq!(parsed.fine_bot, 10);
    assert!((parsed.ksnowf - 1.0).abs() < 1e-12);
    assert!((parsed.kresf - 1.0).abs() < 1e-12);
    assert!((parsed.ksoilf - 1.0).abs() < 1e-12);
    assert!((parsed.kfactor1 - 0.00001).abs() < 1e-12);
    assert!((parsed.kfactor2 - 0.00001).abs() < 1e-12);
    assert!((parsed.kfactor3 - 0.5).abs() < 1e-12);

    assert!(parsed.legacy_clamp_applied);
    for field in [
        "wintRed", "fineTop", "fineBot", "ksnowf", "kresf", "ksoilf", "kfactor1", "kfactor2",
        "kfactor3",
    ] {
        assert!(parsed.legacy_clamp_fields.contains(&field));
    }

    let codes: Vec<_> = parsed.warnings.iter().map(|warning| warning.code).collect();
    assert!(codes.contains(&FrostWarningCode::FrostW003));
}

#[test]
fn prefixed_variant_is_rejected_in_both_modes() {
    for mode in [ParseMode::Strict, ParseMode::Compatibility] {
        let err = parse_frost_from_path(fixture_path("prefixed_variant_rejected.txt"), mode)
            .expect_err("prefixed variant must fail in strict and compatibility modes");

        assert_eq!(err.contract_error_id(), "FROST-E-006");
        assert!(matches!(err, FrostParseError::FrostE006 { .. }));
    }
}

#[test]
fn compatibility_mode_defaults_line2_on_arity_failure() {
    let parsed = parse_frost_from_path(
        fixture_path("compat_line2_arity_invalid.txt"),
        ParseMode::Compatibility,
    )
    .expect("compat mode should default line2 on arity failure");

    assert!(!parsed.line2_present);
    assert!(parsed.legacy_clamp_applied);
    let codes: Vec<_> = parsed.warnings.iter().map(|warning| warning.code).collect();
    assert!(codes.contains(&FrostWarningCode::FrostW002));
    assert!(codes.contains(&FrostWarningCode::FrostW003));
}

#[test]
fn missing_file_returns_defaults_with_mode_specific_warning_behavior() {
    let strict = parse_frost_from_path(fixture_path("does_not_exist_frost.txt"), ParseMode::Strict)
        .expect("missing file defaults branch should succeed in strict mode");
    assert!(!strict.frost_file_present);
    assert!(!strict.line2_present);
    assert_eq!(strict.wint_red, 1);
    assert!(strict.warnings.is_empty());

    let compat = parse_frost_from_path(
        fixture_path("does_not_exist_frost.txt"),
        ParseMode::Compatibility,
    )
    .expect("missing file defaults branch should succeed in compatibility mode");
    assert!(!compat.frost_file_present);
    assert!(!compat.line2_present);
    assert_eq!(compat.wint_red, 1);
    let codes: Vec<_> = compat.warnings.iter().map(|warning| warning.code).collect();
    assert!(codes.contains(&FrostWarningCode::FrostW001));
}

#[test]
fn frost_error_display_preserves_all_runner_visible_details() {
    let cases = [
        (
            FrostParseError::FrostE000 {
                path: PathBuf::from("bad/frost.txt"),
                message: "permission denied".to_string(),
            },
            "FROST-E-000",
            "failed to open/read frost file 'bad/frost.txt': permission denied",
        ),
        (
            FrostParseError::FrostE001 {
                line: 2,
                field: "wintRed",
                message: "missing token".to_string(),
            },
            "FROST-E-001",
            "line 2 parse error for wintRed: missing token",
        ),
        (
            FrostParseError::FrostE002 {
                line: 3,
                field: "fineTop",
                message: "bad arity".to_string(),
            },
            "FROST-E-002",
            "line 3 parse error for fineTop: bad arity",
        ),
        (
            FrostParseError::FrostE003 {
                line: 4,
                field: "ksnowf",
                value: "NaN".to_string(),
            },
            "FROST-E-003",
            "line 4: non-finite value 'NaN' for field ksnowf",
        ),
        (
            FrostParseError::FrostE004 {
                line: 5,
                field: "kfactor1",
                value: -1.0,
                allowed: "[0,1]",
            },
            "FROST-E-004",
            "line 5: value -1 for field kfactor1 is out of range ([0,1])",
        ),
        (
            FrostParseError::FrostE005 {
                message: "line closure".to_string(),
            },
            "FROST-E-005",
            "closure invariant failure: line closure",
        ),
        (
            FrostParseError::FrostE006 {
                line: 1,
                token: "datver=1".to_string(),
            },
            "FROST-E-006",
            "line 1: unsupported prefixed/version-like leading token 'datver=1'",
        ),
    ];

    for (error, expected_id, expected_display) in cases {
        assert_eq!(error.contract_error_id(), expected_id);
        assert_eq!(error.to_string(), expected_display);
    }
}
