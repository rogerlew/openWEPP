use std::collections::BTreeSet;
use std::error::Error as _;
use std::io;
use std::path::PathBuf;

use openwepp_input_contract::parsers::chaninp::{
    ChaninpFile, ChaninpParseError, ChaninpParseOptions, ChaninpParseOutcome, ChaninpWarningCode,
    ParseMode, parse_chaninp_from_path, parse_chaninp_from_str,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/chaninp").join(name)
}

fn valid_ids() -> BTreeSet<i32> {
    BTreeSet::from([4, 5, 6])
}

fn assert_wshedw10_default_options(parsed: &ChaninpFile) {
    let options = parsed
        .options
        .as_ref()
        .expect("WSHED-W10 default branch exports explicit options");
    assert_eq!(options.ichout, 0);
    assert!((options.dtchr_input_s - 60.0).abs() < 1.0e-9);
    assert_eq!(options.dtchr_norm_s, 60);
    assert_eq!(options.ntchr, 1_440);
    assert!(options.cbase_m3_s_m2.abs() < 1.0e-12);
    assert_eq!(options.nchnum_input, 0);
    assert_eq!(options.nchnum_norm, 0);
    assert!(options.ichnum_input.is_empty());
    assert!(options.ichnum_norm.is_empty());
    assert!(!options.chan_output_enabled);
}

#[test]
fn strict_mode_parses_canonical_payload() {
    let parsed = parse_chaninp_from_path(
        fixture_path("strict_valid.chaninp"),
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect("strict valid chaninp should parse");

    assert!(parsed.chaninp_required);
    assert_eq!(parsed.parse_outcome, ChaninpParseOutcome::ParsedBranch);
    assert!(parsed.line_count_closed);
    assert!(parsed.warnings.is_empty());

    let options = parsed
        .options
        .expect("required branch should export options");
    assert_eq!(options.ichout, 3);
    assert!((options.dtchr_input_s - 600.0).abs() < 1e-9);
    assert_eq!(options.dtchr_norm_s, 600);
    assert_eq!(options.ntchr, 144);
    assert!((options.cbase_m3_s_m2 - 0.000_001).abs() < 1e-12);
    assert_eq!(options.nchnum_input, 2);
    assert_eq!(options.nchnum_norm, 2);
    assert_eq!(options.ichnum_input, vec![4, 5]);
    assert_eq!(options.ichnum_norm, vec![4, 5]);
    assert!(options.chan_output_enabled);
}

#[test]
fn wshedw11d_strict_zero_count_closes_after_three_records_and_retains_dtchr() {
    let parsed = parse_chaninp_from_str(
        "3 600\n0.0\n0\n",
        ChaninpParseOptions::strict(4, 2),
        &valid_ids(),
    )
    .expect("canonical nchnum=0 payload should close after record 3");

    assert_eq!(parsed.parse_outcome, ChaninpParseOutcome::ParsedBranch);
    assert!(parsed.line_count_closed);
    assert!(parsed.warnings.is_empty());
    let options = parsed.options.expect("wave branch should expose options");
    assert!((options.dtchr_input_s - 600.0).abs() < 1.0e-9);
    assert_eq!(options.dtchr_norm_s, 600);
    assert_eq!(options.ntchr, 144);
    assert_eq!(options.nchnum_input, 0);
    assert_eq!(options.nchnum_norm, 0);
    assert!(options.ichnum_input.is_empty());
    assert!(options.ichnum_norm.is_empty());
    assert!(!options.chan_output_enabled);
}

#[test]
fn wshedw11d_compat_zero_count_is_parsed_without_default_aliasing() {
    let parsed = parse_chaninp_from_str(
        "3 3600\n0.0\n0\n",
        ChaninpParseOptions::compatibility(5, 2),
        &valid_ids(),
    )
    .expect("canonical compatibility input should remain a parsed branch");

    assert_eq!(parsed.parse_outcome, ChaninpParseOutcome::ParsedBranch);
    assert!(parsed.warnings.is_empty());
    let options = parsed.options.expect("wave branch should expose options");
    assert!((options.dtchr_input_s - 3600.0).abs() < 1.0e-9);
    assert_eq!(options.dtchr_norm_s, 3600);
    assert_eq!(options.ntchr, 24);
    assert_eq!(options.nchnum_norm, 0);
    assert!(options.ichnum_norm.is_empty());
    assert!(!options.chan_output_enabled);
}

#[test]
fn wshedw11d_strict_zero_count_rejects_extra_nonempty_record() {
    let error = parse_chaninp_from_str(
        "3 600\n0.0\n0\n4\n",
        ChaninpParseOptions::strict(4, 2),
        &valid_ids(),
    )
    .expect_err("nchnum=0 closes after record 3");

    assert!(matches!(error, ChaninpParseError::ChnE002 { .. }));
    assert_eq!(error.contract_error_id(), "CHN-E-002");
}

#[test]
fn ipeak_le_two_returns_not_applicable_without_file_dependency() {
    let parsed = parse_chaninp_from_path(
        fixture_path("does_not_exist.chaninp"),
        ChaninpParseOptions::strict(2, 3),
        &valid_ids(),
    )
    .expect("ipeak<=2 should not require sidecar");

    assert!(!parsed.chaninp_required);
    assert_eq!(parsed.parse_outcome, ChaninpParseOutcome::NotApplicable);
    assert!(parsed.options.is_none());
}

#[test]
fn strict_mode_missing_required_file_is_chn_e_009() {
    let err = parse_chaninp_from_path(
        fixture_path("does_not_exist.chaninp"),
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect_err("strict mode must reject missing chaninp");

    assert!(matches!(err, ChaninpParseError::ChnE009 { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-009");
}

#[test]
fn compatibility_missing_file_defaults_with_chn_w_001() {
    let parsed = parse_chaninp_from_path(
        fixture_path("does_not_exist.chaninp"),
        ChaninpParseOptions::compatibility(4, 3),
        &valid_ids(),
    )
    .expect("compatibility should default missing sidecar");

    assert!(parsed.chaninp_required);
    assert_eq!(parsed.parse_outcome, ChaninpParseOutcome::DefaultedCompat);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ChaninpWarningCode::ChnW001)
    );

    assert_wshedw10_default_options(&parsed);
}

#[test]
fn strict_mode_non_enoent_open_error_is_chn_e_000() {
    let err = parse_chaninp_from_path(
        fixture_path("."),
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect_err("strict mode should reject non-ENOENT open errors");

    assert!(matches!(err, ChaninpParseError::ChnE000 { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-000");
}

#[test]
fn compatibility_non_enoent_open_error_collapses_with_chn_w_002() {
    let parsed = parse_chaninp_from_path(
        fixture_path("."),
        ChaninpParseOptions::compatibility(4, 3),
        &valid_ids(),
    )
    .expect("compatibility should collapse non-ENOENT open error");

    assert_eq!(
        parsed.parse_outcome,
        ChaninpParseOutcome::OpenErrorCollapsedCompat
    );
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ChaninpWarningCode::ChnW002)
    );
    assert_wshedw10_default_options(&parsed);
}

#[test]
fn prefixed_variant_is_rejected_in_both_modes() {
    for mode in [ParseMode::Strict, ParseMode::Compatibility] {
        let err = parse_chaninp_from_path(
            fixture_path("strict_prefixed_variant.chaninp"),
            ChaninpParseOptions {
                mode,
                ..ChaninpParseOptions::strict(4, 3)
            },
            &valid_ids(),
        )
        .expect_err("prefixed/datver variant must fail");

        assert!(matches!(err, ChaninpParseError::ChnE008 { .. }));
        assert_eq!(err.contract_error_id(), "CHN-E-008");
    }
}

#[test]
fn strict_mode_rejects_invalid_ichout_domain() {
    let err = parse_chaninp_from_path(
        fixture_path("strict_invalid_ichout.chaninp"),
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect_err("strict mode should reject ichout outside 0..3");

    assert!(matches!(err, ChaninpParseError::ChnE004 { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-004");
}

#[test]
fn compatibility_normalizes_ichout_to_writer_subset_with_warning() {
    let parsed_two = parse_chaninp_from_path(
        fixture_path("compat_ichout_two_normalized.chaninp"),
        ChaninpParseOptions::compatibility(4, 3),
        &valid_ids(),
    )
    .expect("compatibility should normalize ichout=2");

    let options_two = parsed_two.options.expect("options should exist");
    assert_eq!(options_two.ichout, 3);
    assert!(
        parsed_two
            .warnings
            .iter()
            .any(|warning| warning.code == ChaninpWarningCode::ChnW004)
    );

    let parsed_zero = parse_chaninp_from_path(
        fixture_path("compat_ichout_zero_normalized.chaninp"),
        ChaninpParseOptions::compatibility(4, 3),
        &valid_ids(),
    )
    .expect("compatibility should normalize ichout=0");

    let options_zero = parsed_zero.options.expect("options should exist");
    assert_eq!(options_zero.ichout, 1);
}

#[test]
fn strict_rejects_dtchr_out_of_range_and_compatibility_normalizes() {
    let strict_err = parse_chaninp_from_path(
        fixture_path("strict_dtchr_out_of_range.chaninp"),
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect_err("strict mode should reject dtchr below bound");
    assert!(matches!(strict_err, ChaninpParseError::ChnE004 { .. }));

    let compat = parse_chaninp_from_path(
        fixture_path("compat_dtchr_clamped.chaninp"),
        ChaninpParseOptions::compatibility(4, 3),
        &valid_ids(),
    )
    .expect("compatibility should clamp dtchr and continue");

    let options = compat.options.expect("options should exist");
    assert!((options.dtchr_input_s - 60.0).abs() < 1e-9);
    assert_eq!(options.ntchr, 1_440);
    assert_eq!(options.dtchr_norm_s, 60);
    assert!(
        compat
            .warnings
            .iter()
            .any(|warning| warning.code == ChaninpWarningCode::ChnW004)
    );
}

#[test]
fn strict_rejects_negative_cbase_and_compatibility_clamps() {
    let strict_err = parse_chaninp_from_path(
        fixture_path("strict_negative_cbase.chaninp"),
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect_err("strict mode should reject negative cbase");
    assert!(matches!(strict_err, ChaninpParseError::ChnE004 { .. }));

    let compat = parse_chaninp_from_path(
        fixture_path("compat_cbase_negative_clamped.chaninp"),
        ChaninpParseOptions::compatibility(4, 3),
        &valid_ids(),
    )
    .expect("compatibility should clamp negative cbase to zero");

    let options = compat.options.expect("options should exist");
    assert!(options.cbase_m3_s_m2.abs() < 1e-12);
    assert!(
        compat
            .warnings
            .iter()
            .any(|warning| warning.code == ChaninpWarningCode::ChnW004)
    );
}

#[test]
fn strict_enforces_topology_closure_and_compatibility_clamps_count() {
    let strict_err = parse_chaninp_from_path(
        fixture_path("strict_nchnum_exceeds_nchan.chaninp"),
        ChaninpParseOptions::strict(4, 2),
        &valid_ids(),
    )
    .expect_err("strict mode should reject nchnum>nchan");
    assert!(matches!(strict_err, ChaninpParseError::ChnE005 { .. }));

    for options in [
        ChaninpParseOptions::strict(4, 2),
        ChaninpParseOptions::compatibility(4, 2),
    ] {
        let error = parse_chaninp_from_path(
            fixture_path("compat_nchnum_clamped.chaninp"),
            options,
            &valid_ids(),
        )
        .expect_err("both modes must validate raw record cardinality before clamping");
        assert!(matches!(
            error,
            ChaninpParseError::ChnE002 {
                line: 4,
                field: "line4",
                expected: 99,
                found: 2,
            }
        ));
        assert_eq!(error.contract_error_id(), "CHN-E-002");
    }
}

#[test]
fn compatibility_retains_raw_count_then_normalizes_closed_record() {
    let compat = parse_chaninp_from_path(
        fixture_path("compat_nchnum_raw_closed.chaninp"),
        ChaninpParseOptions::compatibility(4, 2),
        &valid_ids(),
    )
    .expect("raw-count-closed compatibility input should normalize after parsing");

    let options = compat.options.expect("options should exist");
    assert_eq!(options.nchnum_input, 99);
    assert_eq!(options.ichnum_input.len(), 99);
    assert_eq!(options.nchnum_norm, 2);
    assert_eq!(options.ichnum_norm, vec![4, 5]);
    assert!(
        compat
            .warnings
            .iter()
            .any(|warning| warning.code == ChaninpWarningCode::ChnW004)
    );
}

#[test]
fn compatibility_warns_for_unknown_id_in_discarded_raw_tail() {
    let parsed = parse_chaninp_from_str(
        "3 600\n0.000001\n3\n4 5 99\n",
        ChaninpParseOptions::compatibility(4, 2),
        &BTreeSet::from([4, 5]),
    )
    .expect("raw-count-closed tail should remain observable");

    assert!(parsed.unknown_ichnum_retained_warning_emitted);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ChaninpWarningCode::ChnW005)
    );
    let options = parsed.options.expect("options should exist");
    assert_eq!(options.nchnum_input, 3);
    assert_eq!(options.ichnum_input, vec![4, 5, 99]);
    assert_eq!(options.nchnum_norm, 2);
    assert_eq!(options.ichnum_norm, vec![4, 5]);
}

#[test]
fn strict_rejects_unknown_ichnum_and_compatibility_retains_with_w005() {
    let strict_err = parse_chaninp_from_path(
        fixture_path("strict_unknown_ichnum.chaninp"),
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect_err("strict mode should reject unknown topology id");
    assert!(matches!(strict_err, ChaninpParseError::ChnE005 { .. }));

    let compat = parse_chaninp_from_path(
        fixture_path("compat_unknown_ichnum_retained.chaninp"),
        ChaninpParseOptions::compatibility(4, 3),
        &valid_ids(),
    )
    .expect("compatibility should retain unknown ichnum with warning");

    assert!(compat.unknown_ichnum_retained_warning_emitted);
    assert!(
        compat
            .warnings
            .iter()
            .any(|warning| warning.code == ChaninpWarningCode::ChnW005)
    );

    let options = compat.options.expect("options should exist");
    assert_eq!(options.ichnum_norm, vec![4, 99]);
}

#[test]
fn strict_mode_enforces_line4_arity() {
    let err = parse_chaninp_from_path(
        fixture_path("strict_line4_arity_mismatch.chaninp"),
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect_err("strict mode should enforce nchnum to line4 arity closure");

    assert!(matches!(err, ChaninpParseError::ChnE002 { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-002");
}

#[test]
fn line4_wrong_arity_precedes_invalid_id_token_in_both_modes() {
    for options in [
        ChaninpParseOptions::strict(4, 3),
        ChaninpParseOptions::compatibility(4, 3),
    ] {
        let error = parse_chaninp_from_str("3 600\n0\n3\n4 invalid\n", options, &valid_ids())
            .expect_err("raw line-4 arity must fail before token parsing in both modes");
        assert!(matches!(
            error,
            ChaninpParseError::ChnE002 {
                line: 4,
                field: "line4",
                expected: 3,
                found: 2,
            }
        ));
    }
}

#[test]
fn compatibility_collapses_parse_failure_to_default_branch() {
    let parsed = parse_chaninp_from_path(
        fixture_path("compat_parse_failure_defaults.chaninp"),
        ChaninpParseOptions::compatibility(4, 3),
        &valid_ids(),
    )
    .expect("compatibility should collapse parse failure");

    assert_eq!(parsed.parse_outcome, ChaninpParseOutcome::DefaultedCompat);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ChaninpWarningCode::ChnW003)
    );
    assert_wshedw10_default_options(&parsed);
}

#[test]
fn strict_mode_captures_trailing_token_provenance() {
    let parsed = parse_chaninp_from_path(
        fixture_path("strict_trailing_tokens.chaninp"),
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect("strict parse should preserve trailing-token provenance");

    assert_eq!(parsed.trailing_token_lines, vec![1, 2, 3]);
}

#[test]
fn cbase_namespace_is_explicit_and_not_gwcoeff_alias() {
    let parsed = parse_chaninp_from_path(
        fixture_path("strict_valid.chaninp"),
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect("strict valid chaninp should parse");

    let options = parsed.options.expect("options should exist");
    assert!(options.cbase_m3_s_m2 > 0.0);
    assert!((options.cbase_m3_s_m2 - 0.000_001).abs() < 1e-12);
}

#[test]
fn warning_codes_have_stable_ids_and_display() {
    let cases = [
        (ChaninpWarningCode::ChnW001, "CHN-W-001"),
        (ChaninpWarningCode::ChnW002, "CHN-W-002"),
        (ChaninpWarningCode::ChnW003, "CHN-W-003"),
        (ChaninpWarningCode::ChnW004, "CHN-W-004"),
        (ChaninpWarningCode::ChnW005, "CHN-W-005"),
    ];
    for (code, expected) in cases {
        assert_eq!(code.as_str(), expected);
        assert_eq!(code.to_string(), expected);
    }
}

#[test]
fn every_error_variant_has_stable_id_display_and_no_nested_source() {
    let errors = [
        ChaninpParseError::ChnE000 {
            path: PathBuf::from("denied.chaninp"),
            source: io::Error::new(io::ErrorKind::PermissionDenied, "denied"),
        },
        ChaninpParseError::ChnE001 {
            line: 1,
            field: "ichout",
            token: "x".into(),
        },
        ChaninpParseError::ChnE002 {
            line: 4,
            field: "line4",
            expected: 2,
            found: 1,
        },
        ChaninpParseError::ChnE003 {
            line: 1,
            field: "dtchr",
            token: "NaN".into(),
        },
        ChaninpParseError::ChnE004 {
            line: 2,
            field: "cbase",
            value: -1.0,
            rule: "non-negative",
        },
        ChaninpParseError::ChnE005 {
            line: 3,
            field: "nchnum",
            value: 4,
            detail: "too many",
        },
        ChaninpParseError::ChnE006 {
            dtchr_norm_s: 0,
            ntchr: 0,
            context: "not positive",
        },
        ChaninpParseError::ChnE007 {
            context: "invariant",
        },
        ChaninpParseError::ChnE008 {
            line: 1,
            token: "datver".into(),
        },
        ChaninpParseError::ChnE009 {
            path: PathBuf::from("missing.chaninp"),
        },
    ];

    for (index, error) in errors.into_iter().enumerate() {
        assert_eq!(error.contract_error_id(), format!("CHN-E-{index:03}"));
        assert!(error.to_string().starts_with(error.contract_error_id()));
        assert!(error.source().is_none());
    }
}

#[test]
fn options_constructors_and_not_applicable_string_path_are_exact() {
    let strict = ChaninpParseOptions::strict(2, 17);
    let compatibility = ChaninpParseOptions::compatibility(2, 17);
    assert_eq!(strict.mode, ParseMode::Strict);
    assert_eq!(compatibility.mode, ParseMode::Compatibility);
    assert_eq!(strict.ipeak, 2);
    assert_eq!(strict.nchan, 17);
    assert_eq!(strict.dtchr_lower_bound_s, 60);
    assert_eq!(strict.dtchr_upper_bound_s, 3_600);
    assert_eq!(strict.mxtchr, 1_440);

    for options in [strict, compatibility] {
        let first = parse_chaninp_from_str("not even a chan.inp", options, &valid_ids())
            .expect("ipeak<=2 bypasses parsing");
        let second = parse_chaninp_from_str("", options, &valid_ids())
            .expect("not-applicable result is deterministic");
        assert_eq!(first, second);
        assert!(!first.chaninp_required);
        assert_eq!(first.ipeak, 2);
        assert_eq!(first.nchan, 17);
        assert!(first.line_count_closed);
        assert!(first.trailing_token_lines.is_empty());
        assert!(!first.unknown_ichnum_retained_warning_emitted);
        assert!(first.warnings.is_empty());
        assert!(first.options.is_none());
    }
}

#[test]
fn record_shape_failures_cover_missing_short_extra_and_conditional_line4() {
    let cases = [
        ("", "file", 3, 0),
        ("word\n0\n0\n", "line1", 2, 1),
        ("3 600\n0\n2\n", "line4", 2, 0),
        ("3 600\n0\n2\n4\n", "line4", 2, 1),
        ("3 600\n0\n1\n4\n5\n", "file", 4, 5),
        ("3 600\n0\n-1\n4\n", "file", 3, 4),
    ];
    for (input, field, expected, found) in cases {
        let error = parse_chaninp_from_str(input, ChaninpParseOptions::strict(4, 3), &valid_ids())
            .expect_err("malformed record shape must fail closed");
        assert!(matches!(
            error,
            ChaninpParseError::ChnE002 {
                field: actual,
                expected: actual_expected,
                found: actual_found,
                ..
            } if actual == field && actual_expected == expected && actual_found == found
        ));
    }
}

#[test]
fn token_parse_and_nonfinite_failures_are_field_specific() {
    let parse_failures = [
        ("x 600\n0\n0\n", "ichout"),
        ("3 x\n0\n0\n", "dtchr"),
        ("3 600\nx\n0\n", "cbase"),
        ("3 600\n0\nx\n", "nchnum"),
        ("3 600\n0\n1\nx\n", "ichnum"),
    ];
    for (input, field) in parse_failures {
        let error = parse_chaninp_from_str(input, ChaninpParseOptions::strict(4, 3), &valid_ids())
            .expect_err("invalid token must fail");
        assert!(matches!(
            error,
            ChaninpParseError::ChnE001 { field: actual, .. } if actual == field
        ));
    }

    for (input, field) in [
        ("3 NaN\n0\n0\n", "dtchr"),
        ("3 inf\n0\n0\n", "dtchr"),
        ("3 -inf\n0\n0\n", "dtchr"),
        ("3 600\nNaN\n0\n", "cbase"),
        ("3 600\ninf\n0\n", "cbase"),
        ("3 600\n-inf\n0\n", "cbase"),
    ] {
        let error = parse_chaninp_from_str(input, ChaninpParseOptions::strict(4, 3), &valid_ids())
            .expect_err("non-finite token must fail");
        assert!(matches!(
            error,
            ChaninpParseError::ChnE003 { field: actual, .. } if actual == field
        ));
    }
}

#[test]
fn compatibility_parse_failures_default_with_stable_w003_structure() {
    for input in [
        "",
        "x 600\n0\n0\n",
        "3 NaN\n0\n0\n",
        "3 600\n0\nnot-a-count\n",
    ] {
        let parsed = parse_chaninp_from_str(
            input,
            ChaninpParseOptions::compatibility(4, 3),
            &valid_ids(),
        )
        .expect("ordinary compatibility parse errors default");
        assert_eq!(parsed.parse_outcome, ChaninpParseOutcome::DefaultedCompat);
        assert!(!parsed.line_count_closed);
        assert_eq!(parsed.warnings.len(), 1);
        assert_eq!(parsed.warnings[0].code, ChaninpWarningCode::ChnW003);
        assert_eq!(parsed.warnings[0].line, None);
        assert_wshedw10_default_options(&parsed);
    }
}

#[test]
fn prefixed_detection_covers_numeric_named_and_version_tokens() {
    for token in ["95.7", "datver", "VERSION_1", "v1", "V2.5"] {
        let input = format!("{token}\n3 600\n0\n0\n");
        for options in [
            ChaninpParseOptions::strict(4, 3),
            ChaninpParseOptions::compatibility(4, 3),
        ] {
            let error = parse_chaninp_from_str(&input, options, &valid_ids())
                .expect_err("prefixed variant is never a compatibility fallback");
            assert!(matches!(
                error,
                ChaninpParseError::ChnE008 { line: 1, ref token } if token == input.lines().next().expect("first line")
            ));
        }
    }

    let error = parse_chaninp_from_str(
        "word\n0\n0\n",
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect_err("a non-prefix singleton proceeds to normal record validation");
    assert!(matches!(
        error,
        ChaninpParseError::ChnE002 { field: "line1", .. }
    ));
}

#[test]
fn comma_blank_and_trailing_tokens_preserve_physical_line_provenance() {
    let parsed = parse_chaninp_from_str(
        "\n  3,600,ignored\n\n 0.25,tail\n 2,tail\n 4,5\n\n",
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect("commas and blank physical records are accepted");
    assert_eq!(parsed.trailing_token_lines, vec![2, 4, 5]);
    let options = parsed.options.expect("options are present");
    assert_eq!(options.ichnum_input, vec![4, 5]);
    assert_eq!(options.nchnum_input, 2);
}

#[test]
fn strict_domain_boundaries_and_both_dtchr_limits_are_closed() {
    for dtchr in [60, 3_600] {
        let input = format!("0 {dtchr}\n0\n0\n");
        let parsed =
            parse_chaninp_from_str(&input, ChaninpParseOptions::strict(4, 0), &BTreeSet::new())
                .expect("inclusive timestep boundary must parse");
        let options = parsed.options.expect("options are present");
        assert!((options.dtchr_input_s - f64::from(dtchr)).abs() <= f64::EPSILON);
        assert!(!options.chan_output_enabled);
    }

    for input in ["-1 600\n0\n0\n", "4 600\n0\n0\n"] {
        let error =
            parse_chaninp_from_str(input, ChaninpParseOptions::strict(4, 0), &BTreeSet::new())
                .expect_err("strict ichout is closed to 0..=3");
        assert!(matches!(
            error,
            ChaninpParseError::ChnE004 {
                field: "ichout",
                ..
            }
        ));
    }
    for dtchr in [59, 3_601] {
        let input = format!("0 {dtchr}\n0\n0\n");
        let error =
            parse_chaninp_from_str(&input, ChaninpParseOptions::strict(4, 0), &BTreeSet::new())
                .expect_err("strict timestep lies within both inclusive limits");
        assert!(matches!(
            error,
            ChaninpParseError::ChnE004 { field: "dtchr", .. }
        ));
    }
}

#[test]
fn compatibility_normalization_covers_extremes_capping_and_no_warning_paths() {
    for (ichout, expected) in [(-7, 1), (1, 1), (3, 3), (99, 3)] {
        let input = format!("{ichout} 3600\n0\n0\n");
        let parsed = parse_chaninp_from_str(
            &input,
            ChaninpParseOptions::compatibility(4, 0),
            &BTreeSet::new(),
        )
        .expect("compatibility ichout normalization must parse");
        assert_eq!(parsed.options.expect("options").ichout, expected);
    }

    let mut high = ChaninpParseOptions::compatibility(4, 0);
    high.mxtchr = 10;
    let capped = parse_chaninp_from_str("3 60\n0\n0\n", high, &BTreeSet::new())
        .expect("compatibility caps ntchr");
    let options = capped.options.expect("options");
    assert_eq!(options.ntchr, 10);
    assert_eq!(options.dtchr_norm_s, 8_640);
    assert!(capped.warnings.iter().any(|warning| {
        warning.code == ChaninpWarningCode::ChnW004 && warning.message.contains("capped ntchr")
    }));

    let upper = parse_chaninp_from_str(
        "3 9999\n0\n0\n",
        ChaninpParseOptions::compatibility(4, 0),
        &BTreeSet::new(),
    )
    .expect("compatibility clamps the upper timestep bound");
    let options = upper.options.expect("options");
    assert!((options.dtchr_input_s - 3_600.0).abs() <= f64::EPSILON);
    assert_eq!(options.ntchr, 24);
}

#[test]
fn custom_invalid_timestep_options_surface_typed_closure_errors() {
    let mut division_by_zero = ChaninpParseOptions::strict(4, 0);
    division_by_zero.dtchr_lower_bound_s = 0;
    let error = parse_chaninp_from_str("0 0\n0\n0\n", division_by_zero, &BTreeSet::new())
        .expect_err("zero timestep creates a non-finite ntchr closure");
    assert!(matches!(error, ChaninpParseError::ChnE006 { ntchr: 0, .. }));

    let mut zero_capacity = ChaninpParseOptions::strict(4, 0);
    zero_capacity.mxtchr = 0;
    let error = parse_chaninp_from_str("0 60\n0\n0\n", zero_capacity, &BTreeSet::new())
        .expect_err("zero timestep capacity cannot normalize");
    assert!(matches!(error, ChaninpParseError::ChnE006 { ntchr: 0, .. }));

    let mut longer_than_a_day = ChaninpParseOptions::strict(4, 0);
    longer_than_a_day.dtchr_upper_bound_s = i32::MAX;
    longer_than_a_day.mxtchr = 10;
    let parsed =
        parse_chaninp_from_str("0 2147483647\n0\n0\n", longer_than_a_day, &BTreeSet::new())
            .expect("an explicitly widened upper bound normalizes to one daily step");
    assert_eq!(parsed.options.expect("options").ntchr, 1);

    let mut subsecond = ChaninpParseOptions::strict(4, 0);
    subsecond.dtchr_lower_bound_s = 0;
    subsecond.mxtchr = 1_000_000;
    let error = parse_chaninp_from_str("0 0.1\n0\n0\n", subsecond, &BTreeSet::new())
        .expect_err("subsecond input cannot round to a positive integer normalized timestep");
    assert!(matches!(
        error,
        ChaninpParseError::ChnE006 {
            dtchr_norm_s: 0,
            ntchr: 864_000,
            ..
        }
    ));
}

#[test]
fn raw_negative_count_is_retained_then_normalized_only_in_compatibility() {
    let strict = parse_chaninp_from_str(
        "3 600\n0\n-2\n",
        ChaninpParseOptions::strict(4, 3),
        &valid_ids(),
    )
    .expect_err("strict mode rejects a negative raw count");
    assert!(matches!(
        strict,
        ChaninpParseError::ChnE004 {
            field: "nchnum",
            ..
        }
    ));

    let compat = parse_chaninp_from_str(
        "3 600\n0\n-2\n",
        ChaninpParseOptions::compatibility(4, 3),
        &valid_ids(),
    )
    .expect("compatibility retains raw count and normalizes to zero");
    let options = compat.options.expect("options");
    assert_eq!(options.nchnum_input, -2);
    assert_eq!(options.nchnum_norm, 0);
    assert!(options.ichnum_input.is_empty());
    assert!(options.ichnum_norm.is_empty());
    assert!(!options.chan_output_enabled);
    assert!(
        compat.warnings.iter().any(|warning| {
            warning.code == ChaninpWarningCode::ChnW004 && warning.line == Some(3)
        })
    );
}

#[test]
fn topology_conversion_and_duplicate_unknowns_are_deterministic() {
    let huge_nchan = usize::try_from(i64::from(i32::MAX) + 1).expect("64-bit test target");
    let strict_error = parse_chaninp_from_str(
        "3 600\n0\n0\n",
        ChaninpParseOptions {
            nchan: huge_nchan,
            ..ChaninpParseOptions::strict(4, 0)
        },
        &BTreeSet::new(),
    )
    .expect_err("strict mode surfaces an unrepresentable topology cardinality");
    assert!(matches!(strict_error, ChaninpParseError::ChnE007 { .. }));

    let compatibility = parse_chaninp_from_str(
        "3 600\n0\n0\n",
        ChaninpParseOptions {
            nchan: huge_nchan,
            ..ChaninpParseOptions::compatibility(4, 0)
        },
        &BTreeSet::new(),
    )
    .expect("compatibility collapses an internal cardinality parse failure");
    assert_eq!(
        compatibility.parse_outcome,
        ChaninpParseOutcome::DefaultedCompat
    );
    assert_eq!(compatibility.warnings[0].code, ChaninpWarningCode::ChnW003);

    let first = parse_chaninp_from_str(
        "3 600\n0\n3\n99 100 4\n",
        ChaninpParseOptions::compatibility(4, 3),
        &BTreeSet::from([4]),
    )
    .expect("compatibility retains multiple unknown ids");
    let second = parse_chaninp_from_str(
        "3 600\n0\n3\n99 100 4\n",
        ChaninpParseOptions::compatibility(4, 3),
        &BTreeSet::from([4]),
    )
    .expect("repeat parse is deterministic");
    assert_eq!(first, second);
    assert_eq!(
        first
            .warnings
            .iter()
            .filter(|warning| warning.code == ChaninpWarningCode::ChnW005)
            .count(),
        1
    );
    assert_eq!(
        first.options.expect("options").ichnum_norm,
        vec![99, 100, 4]
    );
}
