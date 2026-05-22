use std::path::PathBuf;

use openwepp_input_contract::parsers::phosphorus::{
    ParseMode, PhosphorusParseError, PhosphorusParseOptions, PhosphorusWarningCode,
    parse_phosphorus_file,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/phosphorus").join(name)
}

fn assert_opt_close(actual: Option<f64>, expected: f64) {
    let value = actual.expect("expected optional value to be present");
    assert!(
        (value - expected).abs() < 1e-12,
        "actual={value} expected={expected}"
    );
}

fn assert_vec_close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (index, (lhs, rhs)) in actual.iter().zip(expected.iter()).enumerate() {
        assert!(
            (lhs - rhs).abs() < 1e-12,
            "index={index} actual={lhs} expected={rhs}"
        );
    }
}

#[test]
fn strict_mode_parses_canonical_fixture_and_fanout_closes() {
    let parsed = parse_phosphorus_file(
        fixture_path("strict_valid_canonical.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Strict,
            require_sidecar: true,
            expected_hillslope_count: Some(3),
        },
    )
    .expect("strict mode should parse canonical phosphorus sidecar");

    assert!(parsed.sidecar_present);
    assert_eq!(parsed.p_flag, 1);
    assert!(parsed.header_literal_match);
    assert!(parsed.line_count_closed);
    assert!(parsed.warnings.is_empty());

    assert_opt_close(parsed.srp_mg_l, 0.01);
    assert_opt_close(parsed.slfp_mg_l, 0.005);
    assert_opt_close(parsed.bfp_mg_l, 0.002);
    assert_opt_close(parsed.scp_mg_kg, 250.0);

    assert_vec_close(&parsed.tmpsrp_mg_l, &[0.01, 0.01, 0.01]);
    assert_vec_close(&parsed.tmpslfp_mg_l, &[0.005, 0.005, 0.005]);
    assert_vec_close(&parsed.tmpbfp_mg_l, &[0.002, 0.002, 0.002]);
    assert_vec_close(&parsed.tmpscp_mg_kg, &[250.0, 250.0, 250.0]);
}

#[test]
fn strict_mode_accepts_numeric_leading_lines_with_trailing_tokens() {
    let parsed = parse_phosphorus_file(
        fixture_path("strict_valid_trailing_tokens.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Strict,
            require_sidecar: true,
            expected_hillslope_count: None,
        },
    )
    .expect("strict mode should accept canonical numeric-leading rows with trailing text");

    assert_eq!(parsed.trailing_token_lines, vec![2, 3, 4, 5]);
}

#[test]
fn strict_mode_rejects_header_literal_mismatch() {
    let err = parse_phosphorus_file(
        fixture_path("compat_header_variant.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Strict,
            require_sidecar: true,
            expected_hillslope_count: None,
        },
    )
    .expect_err("strict mode should reject non-canonical header");

    assert!(matches!(
        err,
        PhosphorusParseError::HeaderLiteralMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "PHOS-E-007");
}

#[test]
fn compatibility_mode_accepts_noncanonical_header_with_warning() {
    let parsed = parse_phosphorus_file(
        fixture_path("compat_header_variant.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Compatibility,
            require_sidecar: true,
            expected_hillslope_count: None,
        },
    )
    .expect("compat mode should accept non-canonical header");

    assert!(!parsed.header_literal_match);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == PhosphorusWarningCode::PhosW002)
    );
}

#[test]
fn optional_missing_sidecar_in_strict_mode_defaults_disabled_without_warning() {
    let parsed = parse_phosphorus_file(
        fixture_path("does_not_exist.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Strict,
            require_sidecar: false,
            expected_hillslope_count: None,
        },
    )
    .expect("missing optional sidecar should return disabled-state output");

    assert!(!parsed.sidecar_present);
    assert_eq!(parsed.p_flag, 0);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn optional_missing_sidecar_in_compat_mode_emits_warning() {
    let parsed = parse_phosphorus_file(
        fixture_path("does_not_exist.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Compatibility,
            require_sidecar: false,
            expected_hillslope_count: None,
        },
    )
    .expect("compat mode should keep optional missing-sidecar branch");

    assert!(!parsed.sidecar_present);
    assert_eq!(parsed.p_flag, 0);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == PhosphorusWarningCode::PhosW001)
    );
}

#[test]
fn required_missing_sidecar_is_typed_open_error() {
    let err = parse_phosphorus_file(
        fixture_path("does_not_exist.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Strict,
            require_sidecar: true,
            expected_hillslope_count: None,
        },
    )
    .expect_err("required missing sidecar should fail");

    assert!(matches!(err, PhosphorusParseError::InputOpenError { .. }));
    assert_eq!(err.contract_error_id(), "PHOS-E-000");
}

#[test]
fn short_record_count_is_typed() {
    let err = parse_phosphorus_file(
        fixture_path("malformed_short_record_count.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Strict,
            require_sidecar: true,
            expected_hillslope_count: None,
        },
    )
    .expect_err("record-count mismatch should fail");

    assert!(matches!(err, PhosphorusParseError::RecordCountError { .. }));
    assert_eq!(err.contract_error_id(), "PHOS-E-002");
}

#[test]
fn non_numeric_concentration_is_typed() {
    let err = parse_phosphorus_file(
        fixture_path("malformed_non_numeric.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Strict,
            require_sidecar: true,
            expected_hillslope_count: None,
        },
    )
    .expect_err("non-numeric concentration token should fail");

    assert!(matches!(err, PhosphorusParseError::TokenParseError { .. }));
    assert_eq!(err.contract_error_id(), "PHOS-E-001");
}

#[test]
fn negative_concentration_is_typed() {
    let err = parse_phosphorus_file(
        fixture_path("malformed_negative.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Strict,
            require_sidecar: true,
            expected_hillslope_count: None,
        },
    )
    .expect_err("negative concentrations should fail");

    assert!(matches!(err, PhosphorusParseError::FieldRangeError { .. }));
    assert_eq!(err.contract_error_id(), "PHOS-E-004");
}

#[test]
fn non_finite_concentration_is_typed() {
    let err = parse_phosphorus_file(
        fixture_path("malformed_non_finite.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Strict,
            require_sidecar: true,
            expected_hillslope_count: None,
        },
    )
    .expect_err("non-finite concentrations should fail");

    assert!(matches!(err, PhosphorusParseError::FieldFiniteError { .. }));
    assert_eq!(err.contract_error_id(), "PHOS-E-003");
}

#[test]
fn w4dr_009_non_negative_only_policy_accepts_large_positive_values() {
    let parsed = parse_phosphorus_file(
        fixture_path("large_non_negative_values.txt"),
        PhosphorusParseOptions {
            mode: ParseMode::Strict,
            require_sidecar: true,
            expected_hillslope_count: Some(2),
        },
    )
    .expect("large non-negative concentrations are accepted under current policy");

    assert_opt_close(parsed.srp_mg_l, 1_000_000.0);
    assert_opt_close(parsed.scp_mg_kg, 1_000_000_000.0);
    assert_vec_close(&parsed.tmpscp_mg_kg, &[1_000_000_000.0, 1_000_000_000.0]);
}
