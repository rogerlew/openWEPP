use std::{io, path::PathBuf};

use openwepp_input_contract::parsers::tcr::{
    TcrChannelContext, TcrOpenResult, TcrParseError, TcrParseMode, TcrParseOptions,
    TcrParseOutcome, TcrWarningCode, parse_tcr_from_path, parse_tcr_from_str,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/tcr").join(name)
}

#[test]
fn tcr_error_display_preserves_all_contract_identities() {
    let cases = [
        (
            TcrParseError::InputOpenError {
                path: PathBuf::from("inputs/channel.tcr"),
                source: io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"),
            },
            "TCR-E-000",
            "TCR-E-000: could not open/read 'inputs/channel.tcr': permission denied",
            true,
        ),
        (
            TcrParseError::TokenParseError {
                line: 2,
                field: "taumax",
                token: "bad".to_string(),
            },
            "TCR-E-001",
            "TCR-E-001: line 2 token parse error for taumax from 'bad'",
            false,
        ),
        (
            TcrParseError::RecordCountError {
                expected: 4,
                found: 3,
            },
            "TCR-E-002",
            "TCR-E-002: expected 4 records, found 3",
            false,
        ),
        (
            TcrParseError::NonFiniteError {
                line: 3,
                field: "kch",
                value: f64::NAN,
            },
            "TCR-E-003",
            "TCR-E-003: line 3 non-finite value for kch (NaN)",
            false,
        ),
        (
            TcrParseError::DomainError {
                line: 4,
                field: "nch",
                value: -1.0,
                allowed: "> 0",
            },
            "TCR-E-004",
            "TCR-E-004: line 4 domain violation for nch (-1); expected > 0",
            false,
        ),
        (
            TcrParseError::CrossFileDependencyError {
                field: "channel_context",
                message: "missing channel context".to_string(),
            },
            "TCR-E-005",
            "TCR-E-005: cross-file dependency error for channel_context: missing channel context",
            false,
        ),
        (
            TcrParseError::UnsupportedPrefixedVariant {
                line: 1,
                token: "datver".to_string(),
            },
            "TCR-E-007",
            "TCR-E-007: line 1 unsupported prefixed/datver-like variant token 'datver'",
            false,
        ),
        (
            TcrParseError::CurveDomainError {
                index: 1,
                channel_id: 12,
                slope: 0.2,
                denominator: 0.0,
                message: "denominator must be positive",
            },
            "TCR-E-008",
            "TCR-E-008: channel index 1 id 12 slope 0.2 denominator 0 invalid (denominator must be positive)",
            false,
        ),
        (
            TcrParseError::RelationalInvariantError {
                taumin: 70.0,
                taumax: 35.0,
            },
            "TCR-E-009",
            "TCR-E-009: relational invariant violated (taumin <= taumax) for taumin=70, taumax=35",
            false,
        ),
    ];

    for (error, expected_id, expected_display, has_source) in cases {
        assert_eq!(error.contract_error_id(), expected_id);
        assert_eq!(error.to_string(), expected_display);
        assert_eq!(std::error::Error::source(&error).is_some(), has_source);
    }
}

fn strict_options_with_context() -> TcrParseOptions {
    TcrParseOptions {
        mode: TcrParseMode::Strict,
        channel_context: Some(TcrChannelContext {
            nchan: 2,
            channel_element_ids: vec![11, 12],
            chnslp_terminal: vec![0.01, 0.2],
            chntcr_from_channel_file: vec![5.0, 6.0],
        }),
    }
}

fn compat_options_with_context() -> TcrParseOptions {
    TcrParseOptions {
        mode: TcrParseMode::Compatibility,
        channel_context: Some(TcrChannelContext {
            nchan: 2,
            channel_element_ids: vec![11, 12],
            chnslp_terminal: vec![0.01, 0.2],
            chntcr_from_channel_file: vec![5.0, 6.0],
        }),
    }
}

fn assert_close(lhs: f64, rhs: f64) {
    assert!((lhs - rhs).abs() <= 1e-9, "lhs={lhs} rhs={rhs}");
}

#[test]
fn strict_mode_parses_valid_tcr_and_applies_override_curve() {
    let parsed = parse_tcr_from_path(
        fixture_path("strict_valid_labeled.tcr"),
        strict_options_with_context(),
    )
    .expect("strict mode must parse valid labeled records");

    assert_eq!(parsed.tcrflg, 1);
    assert!(parsed.tcr_file_present);
    assert_eq!(parsed.parse_outcome, TcrParseOutcome::ParsedBranch);
    assert_eq!(parsed.open_result, TcrOpenResult::OpenSuccess);
    assert!(parsed.line_count_closed);
    assert_eq!(parsed.trailing_token_lines, vec![1, 2, 3, 4]);
    assert!(parsed.chntcr_override_applied);
    assert_eq!(parsed.effective_chntcr.len(), 2);
    assert!(parsed.warnings.is_empty());

    assert_close(parsed.effective_chntcr[0], 46.666_666_666_666_664);
    assert_close(parsed.effective_chntcr[1], 66.818_181_818_181_81);
}

#[test]
fn strict_mode_missing_file_uses_optional_absence_branch_without_warning() {
    let parsed = parse_tcr_from_path(
        fixture_path("missing.tcr"),
        TcrParseOptions {
            mode: TcrParseMode::Strict,
            channel_context: None,
        },
    )
    .expect("strict mode accepts optional missing sidecar");

    assert_eq!(parsed.tcrflg, 0);
    assert!(!parsed.tcr_file_present);
    assert_eq!(parsed.parse_outcome, TcrParseOutcome::MissingBranch);
    assert_eq!(parsed.open_result, TcrOpenResult::Missing);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn compatibility_mode_missing_file_emits_tcr_w001() {
    let parsed = parse_tcr_from_path(
        fixture_path("missing.tcr"),
        TcrParseOptions {
            mode: TcrParseMode::Compatibility,
            channel_context: None,
        },
    )
    .expect("compatibility mode accepts missing sidecar");

    assert_eq!(parsed.tcrflg, 0);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == TcrWarningCode::TcrW001)
    );
}

#[test]
fn strict_mode_non_enoent_open_error_is_typed_tcr_e_000() {
    let err = parse_tcr_from_path(
        fixture_path("."),
        TcrParseOptions {
            mode: TcrParseMode::Strict,
            channel_context: None,
        },
    )
    .expect_err("strict mode must reject non-ENOENT open errors");

    assert!(matches!(err, TcrParseError::InputOpenError { .. }));
    assert_eq!(err.contract_error_id(), "TCR-E-000");
}

#[test]
fn compatibility_mode_non_enoent_open_error_collapses_with_tcr_w002() {
    let parsed = parse_tcr_from_path(
        fixture_path("."),
        TcrParseOptions {
            mode: TcrParseMode::Compatibility,
            channel_context: None,
        },
    )
    .expect("compatibility mode collapses open errors to missing branch");

    assert_eq!(parsed.tcrflg, 0);
    assert_eq!(
        parsed.parse_outcome,
        TcrParseOutcome::OpenErrorCollapsedCompat
    );
    assert_eq!(parsed.open_result, TcrOpenResult::OpenErrorCollapsedCompat);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == TcrWarningCode::TcrW002)
    );
}

#[test]
fn strict_mode_rejects_blank_present_file_with_tcr_e_002() {
    let err = parse_tcr_from_path(
        fixture_path("compat_blank_newline.tcr"),
        strict_options_with_context(),
    )
    .expect_err("strict mode rejects blank present sidecar");

    assert!(matches!(err, TcrParseError::RecordCountError { .. }));
    assert_eq!(err.contract_error_id(), "TCR-E-002");
}

#[test]
fn compatibility_mode_accepts_blank_present_file_as_missing_branch() {
    let parsed = parse_tcr_from_path(
        fixture_path("compat_blank_newline.tcr"),
        compat_options_with_context(),
    )
    .expect("compatibility mode accepts producer-edge blank sidecar");

    assert_eq!(parsed.tcrflg, 0);
    assert_eq!(parsed.parse_outcome, TcrParseOutcome::MissingBranch);
    assert_eq!(parsed.open_result, TcrOpenResult::Missing);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == TcrWarningCode::TcrW001)
    );
}

#[test]
fn strict_mode_rejects_prefixed_variant_with_tcr_e_007() {
    let err = parse_tcr_from_path(
        fixture_path("invalid_prefix_variant.tcr"),
        strict_options_with_context(),
    )
    .expect_err("prefixed/datver variant should be rejected");

    assert!(matches!(
        err,
        TcrParseError::UnsupportedPrefixedVariant { .. }
    ));
    assert_eq!(err.contract_error_id(), "TCR-E-007");
}

#[test]
fn strict_mode_rejects_domain_kch_zero_with_tcr_e_004() {
    let err = parse_tcr_from_path(
        fixture_path("invalid_kch_zero.tcr"),
        strict_options_with_context(),
    )
    .expect_err("kch must be > 0");

    assert!(matches!(
        err,
        TcrParseError::DomainError { field: "kch", .. }
    ));
    assert_eq!(err.contract_error_id(), "TCR-E-004");
}

#[test]
fn strict_mode_rejects_relational_invariant_with_tcr_e_009() {
    let err = parse_tcr_from_path(
        fixture_path("compat_relational_inversion.tcr"),
        strict_options_with_context(),
    )
    .expect_err("strict mode rejects taumin > taumax");

    assert!(matches!(
        err,
        TcrParseError::RelationalInvariantError { .. }
    ));
    assert_eq!(err.contract_error_id(), "TCR-E-009");
}

#[test]
fn compatibility_mode_accepts_relational_invariant_with_tcr_w003() {
    let parsed = parse_tcr_from_path(
        fixture_path("compat_relational_inversion.tcr"),
        compat_options_with_context(),
    )
    .expect("compatibility mode preserves taumin > taumax with warning");

    assert!(parsed.taumin_taumax_relational_warning_emitted);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == TcrWarningCode::TcrW003)
    );
}

#[test]
fn strict_mode_requires_cross_file_context_when_sidecar_present() {
    let err = parse_tcr_from_path(
        fixture_path("strict_valid_labeled.tcr"),
        TcrParseOptions {
            mode: TcrParseMode::Strict,
            channel_context: None,
        },
    )
    .expect_err("present sidecar requires cross-file context");

    assert!(matches!(
        err,
        TcrParseError::CrossFileDependencyError { .. }
    ));
    assert_eq!(err.contract_error_id(), "TCR-E-005");
}

#[test]
fn strict_mode_rejects_context_cardinality_mismatch_with_tcr_e_005() {
    let err = parse_tcr_from_path(
        fixture_path("strict_valid_labeled.tcr"),
        TcrParseOptions {
            mode: TcrParseMode::Strict,
            channel_context: Some(TcrChannelContext {
                nchan: 2,
                channel_element_ids: vec![1],
                chnslp_terminal: vec![0.1, 0.2],
                chntcr_from_channel_file: vec![5.0, 6.0],
            }),
        },
    )
    .expect_err("context vector lengths must close to nchan");

    assert!(matches!(
        err,
        TcrParseError::CrossFileDependencyError { .. }
    ));
    assert_eq!(err.contract_error_id(), "TCR-E-005");
}

#[test]
fn strict_mode_rejects_denominator_curve_domain_with_tcr_e_008() {
    let err = parse_tcr_from_str(
        "35\n70\n0.02\n0.5\n",
        TcrParseOptions {
            mode: TcrParseMode::Strict,
            channel_context: Some(TcrChannelContext {
                nchan: 1,
                channel_element_ids: vec![1],
                chnslp_terminal: vec![-0.01],
                chntcr_from_channel_file: vec![9.0],
            }),
        },
    )
    .expect_err("negative slope with fractional exponent should violate denominator guard");

    assert!(matches!(err, TcrParseError::CurveDomainError { .. }));
    assert_eq!(err.contract_error_id(), "TCR-E-008");
}

#[test]
fn strict_mode_rejects_token_parse_error_with_tcr_e_001() {
    let err = parse_tcr_from_path(
        fixture_path("invalid_non_numeric_line2.tcr"),
        strict_options_with_context(),
    )
    .expect_err("non-numeric token must fail typed parse");

    assert!(matches!(
        err,
        TcrParseError::TokenParseError {
            field: "taumax",
            ..
        }
    ));
    assert_eq!(err.contract_error_id(), "TCR-E-001");
}

#[test]
fn strict_mode_rejects_missing_line_with_tcr_e_002() {
    let err = parse_tcr_from_path(
        fixture_path("invalid_missing_line4.tcr"),
        strict_options_with_context(),
    )
    .expect_err("missing required record must fail closure");

    assert!(matches!(
        err,
        TcrParseError::RecordCountError {
            expected: 4,
            found: 3
        }
    ));
    assert_eq!(err.contract_error_id(), "TCR-E-002");
}
