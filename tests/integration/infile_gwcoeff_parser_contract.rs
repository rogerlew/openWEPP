use std::path::PathBuf;

use openwepp_input_contract::parsers::gwcoeff::{
    GwcoeffCrossFileContext, GwcoeffOpenResult, GwcoeffParseError, GwcoeffParseOptions,
    GwcoeffParseOutcome, GwcoeffWarningCode, NamespaceBinding, ParseMode, parse_gwcoeff_from_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/gwcoeff").join(name)
}

fn assert_opt_close(actual: Option<f64>, expected: f64) {
    let value = actual.expect("expected optional value to be present");
    assert!(
        (value - expected).abs() < 1e-12,
        "actual={value} expected={expected}"
    );
}

#[test]
fn strict_mode_parses_valid_gwcoeff_with_trailing_tokens() {
    let parsed = parse_gwcoeff_from_path(
        fixture_path("strict_valid_with_trailing_text.txt"),
        GwcoeffParseOptions::strict(),
    )
    .expect("strict valid gwcoeff fixture should parse");

    assert!(parsed.gwcoeff_file_present);
    assert_eq!(parsed.lr_bf, 1);
    assert_eq!(parsed.parse_outcome, GwcoeffParseOutcome::ParsedBranch);
    assert_eq!(parsed.open_result, GwcoeffOpenResult::OpenSuccess);
    assert_eq!(parsed.trailing_token_lines, vec![1, 2, 3, 4]);
    assert_opt_close(parsed.igwstrd, 200.0);
    assert_opt_close(parsed.bfcoeff, 0.04);
    assert_opt_close(parsed.dscoeff, 0.0);
    assert_opt_close(parsed.bftharea, 1.0);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_mode_missing_file_is_optional_absence_branch_without_defaults() {
    let parsed = parse_gwcoeff_from_path(
        fixture_path("does_not_exist.txt"),
        GwcoeffParseOptions::strict(),
    )
    .expect("strict mode should accept optional absence branch");

    assert!(!parsed.gwcoeff_file_present);
    assert_eq!(parsed.lr_bf, 0);
    assert_eq!(parsed.parse_outcome, GwcoeffParseOutcome::MissingBranch);
    assert_eq!(parsed.open_result, GwcoeffOpenResult::Missing);
    assert_eq!(parsed.igwstrd, None);
    assert_eq!(parsed.bfcoeff, None);
    assert_eq!(parsed.dscoeff, None);
    assert_eq!(parsed.bftharea, None);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn compatibility_mode_missing_file_emits_warning() {
    let parsed = parse_gwcoeff_from_path(
        fixture_path("does_not_exist.txt"),
        GwcoeffParseOptions::compatibility(),
    )
    .expect("compat mode should accept optional absence branch");

    assert_eq!(parsed.open_result, GwcoeffOpenResult::Missing);
    assert_eq!(parsed.parse_outcome, GwcoeffParseOutcome::MissingBranch);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == GwcoeffWarningCode::GwW001)
    );
}

#[test]
fn strict_mode_non_enoent_open_error_is_typed_failure() {
    let err = parse_gwcoeff_from_path(
        PathBuf::from("tests/fixtures/infile"),
        GwcoeffParseOptions::strict(),
    )
    .expect_err("strict mode should hard-fail on non-ENOENT open error");

    assert!(matches!(err, GwcoeffParseError::InputOpenError { .. }));
    assert_eq!(err.contract_error_id(), "GW-E-000");
}

#[test]
fn compatibility_mode_collapses_non_enoent_open_error_with_warning() {
    let parsed = parse_gwcoeff_from_path(
        PathBuf::from("tests/fixtures/infile"),
        GwcoeffParseOptions::compatibility(),
    )
    .expect("compat mode should collapse non-ENOENT open error");

    assert_eq!(
        parsed.open_result,
        GwcoeffOpenResult::OpenErrorCollapsedCompat
    );
    assert_eq!(parsed.parse_outcome, GwcoeffParseOutcome::MissingBranch);
    assert_eq!(parsed.lr_bf, 0);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == GwcoeffWarningCode::GwW001)
    );
}

#[test]
fn strict_mode_rejects_missing_record_count() {
    let err = parse_gwcoeff_from_path(
        fixture_path("invalid_missing_line4.txt"),
        GwcoeffParseOptions::strict(),
    )
    .expect_err("missing line 4 should fail record closure");

    assert!(matches!(err, GwcoeffParseError::RecordCountError { .. }));
    assert_eq!(err.contract_error_id(), "GW-E-002");
}

#[test]
fn strict_mode_rejects_non_numeric_tokens() {
    let err = parse_gwcoeff_from_path(
        fixture_path("invalid_non_numeric_line2.txt"),
        GwcoeffParseOptions::strict(),
    )
    .expect_err("non-numeric line token should fail");

    assert!(matches!(err, GwcoeffParseError::TokenParseError { .. }));
    assert_eq!(err.contract_error_id(), "GW-E-001");
}

#[test]
fn strict_mode_rejects_non_finite_tokens() {
    let err = parse_gwcoeff_from_path(
        fixture_path("invalid_nonfinite_line3.txt"),
        GwcoeffParseOptions::strict(),
    )
    .expect_err("non-finite token should fail");

    assert!(matches!(err, GwcoeffParseError::FieldFiniteError { .. }));
    assert_eq!(err.contract_error_id(), "GW-E-003");
}

#[test]
fn strict_mode_rejects_negative_domain_values() {
    let err = parse_gwcoeff_from_path(
        fixture_path("invalid_negative_bftharea.txt"),
        GwcoeffParseOptions::strict(),
    )
    .expect_err("negative bftharea should fail domain guard");

    assert!(matches!(err, GwcoeffParseError::FieldRangeError { .. }));
    assert_eq!(err.contract_error_id(), "GW-E-004");
}

#[test]
fn strict_mode_rejects_prefixed_variant() {
    let err = parse_gwcoeff_from_path(
        fixture_path("invalid_prefixed_variant.txt"),
        GwcoeffParseOptions::strict(),
    )
    .expect_err("prefixed/datver-like variant should fail");

    assert!(matches!(
        err,
        GwcoeffParseError::UnsupportedPrefixedVariant { .. }
    ));
    assert_eq!(err.contract_error_id(), "GW-E-007");
}

#[test]
fn compatibility_mode_keeps_record_closure_guard_for_extra_nonempty_line() {
    let err = parse_gwcoeff_from_path(
        fixture_path("compat_extra_line_record_count_error.txt"),
        GwcoeffParseOptions::compatibility(),
    )
    .expect_err("extra non-empty record should fail closure in compatibility mode");

    assert!(matches!(err, GwcoeffParseError::RecordCountError { .. }));
    assert_eq!(err.contract_error_id(), "GW-E-002");
}

#[test]
fn namespace_conflation_is_rejected_with_typed_cross_file_error() {
    let err = parse_gwcoeff_from_path(
        fixture_path("strict_valid_numeric_only.txt"),
        GwcoeffParseOptions {
            mode: ParseMode::Strict,
            cross_file: GwcoeffCrossFileContext {
                namespace_binding: NamespaceBinding::ConflatedWithChaninp,
            },
        },
    )
    .expect_err("namespace conflation must be rejected");

    assert!(matches!(
        err,
        GwcoeffParseError::CoefficientNamespaceConflation { .. }
    ));
    assert_eq!(err.contract_error_id(), "GW-E-005");
}
