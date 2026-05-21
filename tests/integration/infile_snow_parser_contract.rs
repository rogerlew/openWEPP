use std::path::PathBuf;

use openwepp_input_contract::parsers::snow::{
    ParseMode, SnowParseError, SnowParseOptions, SnowWarningCode, parse_snow_file,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/snow").join(name)
}

#[test]
fn strict_mode_parses_canonical_three_record_file() {
    let parsed = parse_snow_file(
        fixture_path("strict_valid.txt"),
        SnowParseOptions::default(),
    )
    .expect("strict canonical snow file should parse");

    assert!(parsed.sidecar_present);
    assert!(!parsed.defaults_applied);
    assert!((parsed.rst - 0.0).abs() < 1e-12);
    assert!((parsed.newsnw - 100.0).abs() < 1e-12);
    assert!((parsed.ssd - 250.0).abs() < 1e-12);
    assert_eq!(parsed.surplus_record_count, 0);
    assert!(parsed.trailing_token_lines.is_empty());
    assert!(!parsed.prefix_variant_detected);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_mode_missing_file_uses_defaults_without_warning() {
    let parsed = parse_snow_file(
        fixture_path("does_not_exist.txt"),
        SnowParseOptions::default(),
    )
    .expect("strict mode missing file should use default branch");

    assert!(!parsed.sidecar_present);
    assert!(parsed.defaults_applied);
    assert!((parsed.rst - 0.0).abs() < 1e-12);
    assert!((parsed.newsnw - 100.0).abs() < 1e-12);
    assert!((parsed.ssd - 250.0).abs() < 1e-12);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn compatibility_mode_missing_file_emits_default_warning() {
    let parsed = parse_snow_file(
        fixture_path("does_not_exist.txt"),
        SnowParseOptions {
            mode: ParseMode::Compatibility,
        },
    )
    .expect("compat mode missing file should use default branch");

    assert!(!parsed.sidecar_present);
    assert!(parsed.defaults_applied);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == SnowWarningCode::SnowW001)
    );
}

#[test]
fn strict_mode_rejects_trailing_tokens() {
    let err = parse_snow_file(
        fixture_path("strict_trailing_tokens_invalid.txt"),
        SnowParseOptions::default(),
    )
    .expect_err("strict mode must reject trailing tokens");

    assert!(matches!(
        err,
        SnowParseError::StrictTrailingTokenError { .. }
    ));
    assert_eq!(err.contract_error_id(), "SNOW-E-007");
}

#[test]
fn compatibility_mode_accepts_trailing_tokens_with_provenance_warning() {
    let parsed = parse_snow_file(
        fixture_path("compat_trailing_tokens.txt"),
        SnowParseOptions {
            mode: ParseMode::Compatibility,
        },
    )
    .expect("compat mode should accept trailing tokens");

    assert!((parsed.rst + 1.5).abs() < 1e-12);
    assert_eq!(parsed.trailing_token_lines, vec![1, 2, 3]);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == SnowWarningCode::SnowW002)
    );
}

#[test]
fn strict_mode_rejects_surplus_records() {
    let err = parse_snow_file(
        fixture_path("strict_surplus_records_invalid.txt"),
        SnowParseOptions::default(),
    )
    .expect_err("strict mode must reject surplus records");

    assert!(matches!(
        err,
        SnowParseError::StrictSurplusRecordError { .. }
    ));
    assert_eq!(err.contract_error_id(), "SNOW-E-006");
}

#[test]
fn compatibility_mode_accepts_surplus_records_with_warning() {
    let parsed = parse_snow_file(
        fixture_path("compat_surplus_records.txt"),
        SnowParseOptions {
            mode: ParseMode::Compatibility,
        },
    )
    .expect("compat mode should ignore surplus records");

    assert_eq!(parsed.surplus_record_count, 1);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == SnowWarningCode::SnowW003)
    );
}

#[test]
fn strict_mode_rejects_record_count_underflow() {
    let err = parse_snow_file(
        fixture_path("strict_missing_record_invalid.txt"),
        SnowParseOptions::default(),
    )
    .expect_err("missing third record must fail");

    assert!(matches!(err, SnowParseError::MissingRecordError { .. }));
    assert_eq!(err.contract_error_id(), "SNOW-E-002");
}

#[test]
fn strict_mode_rejects_non_numeric_token() {
    let err = parse_snow_file(
        fixture_path("strict_non_numeric_invalid.txt"),
        SnowParseOptions::default(),
    )
    .expect_err("non-numeric token must fail");

    assert!(matches!(err, SnowParseError::TokenParseError { .. }));
    assert_eq!(err.contract_error_id(), "SNOW-E-001");
}

#[test]
fn strict_mode_rejects_non_finite_values() {
    let err = parse_snow_file(
        fixture_path("strict_nonfinite_invalid.txt"),
        SnowParseOptions::default(),
    )
    .expect_err("NaN token must fail finite guard");

    assert!(matches!(err, SnowParseError::NonFiniteError { .. }));
    assert_eq!(err.contract_error_id(), "SNOW-E-003");
}

#[test]
fn strict_mode_rejects_non_positive_density_values() {
    let err = parse_snow_file(
        fixture_path("strict_nonpositive_density_invalid.txt"),
        SnowParseOptions::default(),
    )
    .expect_err("non-positive density values must fail domain guards");

    assert!(matches!(err, SnowParseError::FieldRangeError { .. }));
    assert_eq!(err.contract_error_id(), "SNOW-E-004");
}

#[test]
fn prefixed_variant_is_rejected_in_both_modes() {
    for mode in [ParseMode::Strict, ParseMode::Compatibility] {
        let err = parse_snow_file(
            fixture_path("prefixed_variant_rejected.txt"),
            SnowParseOptions { mode },
        )
        .expect_err("prefixed variant must be rejected");

        assert!(matches!(
            err,
            SnowParseError::UnsupportedPrefixVariantError { .. }
        ));
        assert_eq!(err.contract_error_id(), "SNOW-E-008");
    }
}
