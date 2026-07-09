use std::collections::BTreeSet;
use std::path::PathBuf;

use openwepp_input_contract::parsers::chaninp::{
    ChaninpFile, ChaninpParseError, ChaninpParseOptions, ChaninpParseOutcome, ChaninpWarningCode,
    ParseMode, parse_chaninp_from_path,
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

    let compat = parse_chaninp_from_path(
        fixture_path("compat_nchnum_clamped.chaninp"),
        ChaninpParseOptions::compatibility(4, 2),
        &valid_ids(),
    )
    .expect("compatibility should clamp nchnum to nchan");

    let options = compat.options.expect("options should exist");
    assert_eq!(options.nchnum_input, 2);
    assert_eq!(options.nchnum_norm, 2);
    assert!(
        compat
            .warnings
            .iter()
            .any(|warning| warning.code == ChaninpWarningCode::ChnW004)
    );
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
