use std::path::PathBuf;

use openwepp_input_contract::parsers::watershed_channel::{
    ChannelWarningCode, WatershedChannelParseError, WatershedChannelParseMode,
    WatershedChannelParseOptions, parse_watershed_channel_from_path,
    parse_watershed_channel_from_str,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/watershed_channel").join(name)
}

fn canonical_input_with_token(line_number: usize, token_index: usize, replacement: &str) -> String {
    let mut lines: Vec<String> =
        include_str!("../fixtures/infile/watershed_channel/strict_valid_single_channel.chn")
            .lines()
            .map(str::to_string)
            .collect();
    let mut tokens: Vec<&str> = lines[line_number - 1].split_whitespace().collect();
    tokens[token_index] = replacement;
    lines[line_number - 1] = tokens.join(" ");
    lines.join("\n")
}

fn canonical_input_with_line(line_number: usize, replacement: &str) -> String {
    let mut lines: Vec<String> =
        include_str!("../fixtures/infile/watershed_channel/strict_valid_single_channel.chn")
            .lines()
            .map(str::to_string)
            .collect();
    lines[line_number - 1] = replacement.to_string();
    lines.join("\n")
}

#[test]
fn warning_codes_have_stable_public_strings() {
    for (code, expected) in [
        (ChannelWarningCode::ChnW001, "CHN-W-001"),
        (ChannelWarningCode::ChnW002, "CHN-W-002"),
        (ChannelWarningCode::ChnW003, "CHN-W-003"),
        (ChannelWarningCode::ChnW004, "CHN-W-004"),
        (ChannelWarningCode::ChnW005, "CHN-W-005"),
    ] {
        assert_eq!(code.as_str(), expected);
        assert_eq!(code.to_string(), expected);
    }
}

#[test]
fn parse_errors_have_stable_ids_messages_and_sources() {
    let errors = [
        (
            WatershedChannelParseError::Io {
                path: PathBuf::from("missing.chn"),
                source: std::io::Error::new(std::io::ErrorKind::NotFound, "fixture io"),
            },
            "CHN-E-000",
            "CHN-E-000: failed to open/read 'missing.chn': fixture io",
            true,
        ),
        (
            WatershedChannelParseError::TokenParse {
                line: 2,
                field: "nchan",
                token: "nope".to_string(),
            },
            "CHN-E-001",
            "CHN-E-001: line 2 failed to parse field 'nchan' from token 'nope'",
            false,
        ),
        (
            WatershedChannelParseError::RecordClosure {
                context: "comments",
                expected: 3,
                found: 2,
            },
            "CHN-E-002",
            "CHN-E-002: record closure error in comments; expected 3 line(s), found 2",
            false,
        ),
        (
            WatershedChannelParseError::UnsupportedDatver {
                line: 1,
                value: 94.0,
            },
            "CHN-E-003",
            "CHN-E-003: line 1 unsupported datver '94'",
            false,
        ),
        (
            WatershedChannelParseError::EnumDomain {
                line: 3,
                field: "ipeak",
                value: 9,
            },
            "CHN-E-004",
            "CHN-E-004: line 3 enum-domain violation for 'ipeak' with value 9",
            false,
        ),
    ];

    for (error, expected_id, expected_message, has_source) in errors {
        assert_eq!(error.contract_error_id(), expected_id);
        assert_eq!(error.to_string(), expected_message);
        assert_eq!(std::error::Error::source(&error).is_some(), has_source);
    }
}

#[test]
fn remaining_parse_errors_have_stable_ids_messages_and_no_sources() {
    let errors = [
        (
            WatershedChannelParseError::FieldRange {
                line: 2,
                field: "nchan",
                value: 0.0,
                rule: "> 0",
            },
            "CHN-E-005",
            "CHN-E-005: line 2 field 'nchan' value 0 violates > 0",
            false,
        ),
        (
            WatershedChannelParseError::RatingCurveClosure {
                line: 15,
                channel_id: 1,
                reason: "missing row",
            },
            "CHN-E-006",
            "CHN-E-006: line 15 channel 1 rating-curve closure failure: missing row",
            false,
        ),
        (
            WatershedChannelParseError::ChannelCountMismatch {
                declared: 1,
                expected: 2,
            },
            "CHN-E-007",
            "CHN-E-007: channel count mismatch declared=1 expected=2",
            false,
        ),
        (
            WatershedChannelParseError::RequiredSidecarMissing {
                sidecar: "chan.inp",
                ipeak: 3,
            },
            "CHN-E-008",
            "CHN-E-008: ipeak=3 requires sidecar 'chan.inp'",
            false,
        ),
        (
            WatershedChannelParseError::InvariantViolation {
                line: 14,
                context: "control closure",
            },
            "CHN-E-009",
            "CHN-E-009: line 14 invariant violation: control closure",
            false,
        ),
    ];

    for (error, expected_id, expected_message, has_source) in errors {
        assert_eq!(error.contract_error_id(), expected_id);
        assert_eq!(error.to_string(), expected_message);
        assert_eq!(std::error::Error::source(&error).is_some(), has_source);
    }
}

#[test]
fn path_parser_preserves_missing_file_context_and_io_source() {
    let path = fixture_path("does-not-exist.chn");
    let error = parse_watershed_channel_from_path(&path, WatershedChannelParseOptions::default())
        .expect_err("missing channel input should fail closed");

    assert!(matches!(
        &error,
        WatershedChannelParseError::Io {
            path: error_path,
            ..
        } if error_path == &path
    ));
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn all_real_token_families_reject_nan_and_infinities() {
    let fields = [
        (1, 0, "ver"),
        (4, 0, "lw"),
        (12, 0, "geom_line"),
        (12, 1, "geom_line"),
        (13, 0, "erod_line"),
        (13, 1, "erod_line"),
        (13, 2, "erod_line"),
        (13, 3, "erod_line"),
        (13, 4, "erod_line"),
        (14, 0, "control_line"),
        (14, 1, "control_line"),
        (14, 2, "control_line"),
        (15, 0, "rccoef"),
        (15, 1, "rcexp"),
        (15, 2, "rcoset"),
    ];

    for (line, token_index, field) in fields {
        for value in ["NaN", "inf", "-inf"] {
            let input = canonical_input_with_token(line, token_index, value);
            let error =
                parse_watershed_channel_from_str(&input, WatershedChannelParseOptions::default())
                    .expect_err("every non-finite float field should fail closed");
            assert!(
                matches!(
                    &error,
                    WatershedChannelParseError::FieldRange {
                        field: error_field,
                        rule: "finite",
                        ..
                    } if *error_field == field
                ),
                "field={field} value={value} error={error:?}"
            );
        }
    }
}

#[test]
fn integer_and_record_cardinality_errors_are_exact() {
    for (input, line, field) in [
        ("99.1\nnope\n".to_string(), 2, "nchan"),
        (canonical_input_with_line(1, "99.1 1"), 1, "ver"),
        (canonical_input_with_line(2, "1 2"), 2, "nchan"),
        (canonical_input_with_line(12, "19.99"), 12, "geom_line"),
        (
            canonical_input_with_line(13, "0.04 0.000001 19.0 900.0"),
            13,
            "erod_line",
        ),
        (
            canonical_input_with_line(14, "0.02 4.0"),
            14,
            "control_line",
        ),
    ] {
        let error =
            parse_watershed_channel_from_str(&input, WatershedChannelParseOptions::default())
                .expect_err("malformed integer or record cardinality must fail closed");
        assert!(
            matches!(
                error,
                WatershedChannelParseError::TokenParse {
                    line: error_line,
                    field: error_field,
                    ..
                } if error_line == line && error_field == field
            ),
            "line={line} field={field}"
        );
    }
}

#[test]
fn integer_enum_and_count_boundaries_are_exact() {
    for (line, value, field, is_enum) in [
        (2, "0", "nchan", false),
        (3, "0", "ipeak", true),
        (3, "6", "ipeak", true),
        (8, "0", "ishape", true),
        (8, "4", "ishape", true),
        (9, "-1", "icntrl", true),
        (9, "5", "icntrl", true),
        (10, "0", "ienslp", true),
        (10, "3", "ienslp", true),
        (11, "-1", "flgout", true),
        (11, "2", "flgout", true),
    ] {
        let input = canonical_input_with_token(line, 0, value);
        let error =
            parse_watershed_channel_from_str(&input, WatershedChannelParseOptions::default())
                .expect_err("out-of-domain integer field should fail closed");
        if is_enum {
            assert!(matches!(
                error,
                WatershedChannelParseError::EnumDomain {
                    field: error_field,
                    ..
                } if error_field == field
            ));
        } else {
            assert!(matches!(
                error,
                WatershedChannelParseError::FieldRange {
                    field: error_field,
                    rule: "> 0",
                    ..
                } if error_field == field
            ));
        }
    }
}

#[test]
fn all_channel_real_domain_families_are_exact() {
    for (line, token_index, value, field) in [
        (4, 0, "0", "lw"),
        (12, 0, "0", "chnz"),
        (12, 1, "0", "chnnbr"),
        (13, 0, "0", "chnn"),
        (13, 1, "-0.000001", "chnk"),
        (13, 2, "-0.000001", "chntcr"),
        (13, 3, "-0.000001", "chnedm"),
        (13, 4, "-0.000001", "chneds"),
        (14, 0, "-0.000001", "ctlslp"),
        (14, 1, "0", "ctlz"),
        (14, 2, "0", "ctln"),
        (15, 0, "0", "rccoef"),
        (15, 1, "0", "rcexp"),
        (15, 2, "-0.000001", "rcoset"),
    ] {
        let input = canonical_input_with_token(line, token_index, value);
        let error =
            parse_watershed_channel_from_str(&input, WatershedChannelParseOptions::default())
                .expect_err("at-or-across float guard should fail closed");
        assert!(matches!(
            error,
            WatershedChannelParseError::FieldRange {
                field: error_field,
                ..
            } if error_field == field
        ));
    }
}

#[test]
fn valid_enum_and_nonnegative_boundaries_parse() {
    for (line, token_index, value) in [
        (3, 0, "1"),
        (3, 0, "5"),
        (8, 0, "3"),
        (10, 0, "2"),
        (11, 0, "1"),
        (13, 1, "0"),
        (13, 2, "0"),
        (13, 3, "0"),
        (13, 4, "0"),
        (14, 0, "0"),
        (15, 2, "0"),
    ] {
        let input = canonical_input_with_token(line, token_index, value);
        parse_watershed_channel_from_str(&input, WatershedChannelParseOptions::default())
            .expect("valid enum and nonnegative boundary should parse");
    }
}

#[test]
fn compatibility_boundaries_and_low_ishape_are_exact() {
    let options = WatershedChannelParseOptions {
        mode: WatershedChannelParseMode::Compatibility,
        ..WatershedChannelParseOptions::default()
    };

    let accepted = canonical_input_with_token(1, 0, "94.301");
    let parsed = parse_watershed_channel_from_str(&accepted, options)
        .expect("compatibility minimum datver should parse");
    assert_eq!(parsed.warnings[0].code, ChannelWarningCode::ChnW001);

    let rejected = canonical_input_with_token(1, 0, "94.3");
    assert!(matches!(
        parse_watershed_channel_from_str(&rejected, options),
        Err(WatershedChannelParseError::UnsupportedDatver { .. })
    ));

    let low_ishape = canonical_input_with_token(8, 0, "0");
    assert!(matches!(
        parse_watershed_channel_from_str(&low_ishape, options),
        Err(WatershedChannelParseError::EnumDomain {
            line: 8,
            field: "ishape",
            value: 0,
        })
    ));
}

#[test]
fn truncated_channel_records_preserve_error_priority() {
    let lines: Vec<&str> =
        include_str!("../fixtures/infile/watershed_channel/strict_valid_single_channel.chn")
            .lines()
            .collect();

    for retained_lines in 0..lines.len() {
        let truncated = lines[..retained_lines].join("\n");
        let error =
            parse_watershed_channel_from_str(&truncated, WatershedChannelParseOptions::default())
                .expect_err("missing required record should fail closed");
        if retained_lines == lines.len() - 1 {
            assert!(matches!(
                error,
                WatershedChannelParseError::RatingCurveClosure {
                    line: 14,
                    channel_id: 1,
                    reason: "icntrl==4 requires rating_curve_line",
                }
            ));
        } else {
            assert!(matches!(
                error,
                WatershedChannelParseError::RecordClosure { .. }
            ));
        }
    }
}

#[test]
fn trailing_blank_physical_lines_are_ignored() {
    let input = format!(
        "{}\n\n   \n\t\n",
        include_str!("../fixtures/infile/watershed_channel/strict_valid_single_channel.chn")
    );
    let parsed = parse_watershed_channel_from_str(&input, WatershedChannelParseOptions::default())
        .expect("blank trailing physical lines should preserve canonical EOF closure");
    assert_eq!(parsed.channels.len(), 1);
}

#[test]
fn strict_mode_parses_canonical_single_channel_rating_curve_profile() {
    let parsed = parse_watershed_channel_from_path(
        fixture_path("strict_valid_single_channel.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect("strict valid profile should parse");

    assert!((parsed.datver - 99.1).abs() < 1e-12);
    assert_eq!(parsed.nchan, 1);
    assert_eq!(parsed.ipeak, 2);
    assert!(!parsed.sidecar_required);
    assert!(parsed.warnings.is_empty());
    assert_eq!(parsed.channels.len(), 1);

    let channel = &parsed.channels[0];
    assert_eq!(channel.channel_id, 1);
    assert_eq!(channel.comment_1, "channel 1 comment a");
    assert_eq!(channel.comment_2, "channel 1 comment b");
    assert_eq!(channel.comment_3, "channel 1 comment c");
    assert_eq!(channel.ishape, 1);
    assert_eq!(channel.icntrl, 4);
    assert_eq!(channel.ienslp, 1);
    assert_eq!(channel.flgout, 0);
    assert!((channel.chnz - 19.99).abs() < 1e-12);
    assert!((channel.chnnbr - 0.03).abs() < 1e-12);
    assert!((channel.chnn - 0.04).abs() < 1e-12);
    assert!((channel.chnk - 0.000_001).abs() < 1e-12);
    assert!((channel.chntcr - 19.0).abs() < 1e-12);
    assert!((channel.chnedm - 900.0).abs() < 1e-12);
    assert!((channel.chneds - 0.000_1).abs() < 1e-12);
    assert!((channel.ctlslp_input - 0.02).abs() < 1e-12);
    assert!((channel.ctlz_input - 4.0).abs() < 1e-12);
    assert!((channel.ctln_input - 0.04).abs() < 1e-12);
    assert!((channel.ctlslp_effective - 0.02).abs() < 1e-12);
    assert!((channel.ctlz_effective - 4.0).abs() < 1e-12);
    assert!((channel.ctln_effective - 0.04).abs() < 1e-12);
    assert!(channel.has_rating_curve);
    let rating_curve = channel
        .rating_curve
        .as_ref()
        .expect("icntrl=4 should retain the rating-curve triple");
    assert!((rating_curve.rccoef - 1.25).abs() < 1e-12);
    assert!((rating_curve.rcexp - 1.5).abs() < 1e-12);
    assert!((rating_curve.rcoset - 0.1).abs() < 1e-12);
    assert!(!channel.control_override_applied);
}

#[test]
fn strict_mode_rejects_unsupported_datver() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_invalid_datver.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("strict mode should reject non-canonical datver");

    assert!(matches!(
        err,
        WatershedChannelParseError::UnsupportedDatver { .. }
    ));
    assert_eq!(err.contract_error_id(), "CHN-E-003");
}

#[test]
fn compatibility_mode_accepts_legacy_datver_with_warning() {
    let parsed = parse_watershed_channel_from_path(
        fixture_path("compat_legacy_datver.chn"),
        WatershedChannelParseOptions {
            mode: WatershedChannelParseMode::Compatibility,
            ..WatershedChannelParseOptions::default()
        },
    )
    .expect("compat mode should accept legacy datver window");

    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ChannelWarningCode::ChnW001)
    );
}

#[test]
fn strict_mode_rejects_ishape_out_of_domain() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_ishape_out_of_domain.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("strict mode should reject unsupported ishape");

    assert!(matches!(err, WatershedChannelParseError::EnumDomain { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-004");
}

#[test]
fn strict_mode_rejects_ienslp_out_of_domain() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_ienslp_out_of_domain.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("strict mode should reject unsupported ienslp");

    assert!(matches!(err, WatershedChannelParseError::EnumDomain { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-004");
}

#[test]
fn strict_mode_rejects_icntrl_out_of_domain() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_icntrl_out_of_domain.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("strict mode should reject unsupported icntrl");

    assert!(matches!(err, WatershedChannelParseError::EnumDomain { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-004");
}

#[test]
fn strict_mode_rejects_flgout_out_of_domain() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_flgout_out_of_domain.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("strict mode should reject unsupported flgout");

    assert!(matches!(err, WatershedChannelParseError::EnumDomain { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-004");
}

#[test]
fn strict_mode_rejects_chnn_less_than_chnnbr() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_chnn_less_than_chnnbr.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("strict mode should reject chnn below chnnbr");

    assert!(matches!(err, WatershedChannelParseError::FieldRange { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-005");
}

#[test]
fn compatibility_mode_normalizes_legacy_ishape() {
    let parsed = parse_watershed_channel_from_path(
        fixture_path("compat_ishape_normalized.chn"),
        WatershedChannelParseOptions {
            mode: WatershedChannelParseMode::Compatibility,
            ..WatershedChannelParseOptions::default()
        },
    )
    .expect("compat mode should normalize ishape>3 to naturally eroded class");

    assert_eq!(parsed.channels[0].ishape, 3);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ChannelWarningCode::ChnW003)
    );
}

#[test]
fn strict_mode_accepts_naturally_eroded_ishape_class() {
    let parsed = parse_watershed_channel_from_path(
        fixture_path("strict_ishape_naturally_eroded.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect("strict mode should accept ishape=3");

    assert_eq!(parsed.channels[0].ishape, 3);
    assert!(
        parsed.warnings.is_empty(),
        "strict parse should not emit compatibility normalization warnings"
    );
}

#[test]
fn strict_mode_rejects_missing_rating_curve_line_for_icntrl4() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_missing_rating_curve.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("missing rating-curve line should fail");

    assert!(matches!(
        err,
        WatershedChannelParseError::RatingCurveClosure { .. }
    ));
    assert_eq!(err.contract_error_id(), "CHN-E-006");
}

#[test]
fn strict_mode_rejects_rating_curve_rccoef_non_positive() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_rating_curve_rccoef_non_positive.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("non-positive rccoef should fail");

    assert!(matches!(err, WatershedChannelParseError::FieldRange { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-005");
}

#[test]
fn strict_mode_rejects_rating_curve_rcoset_negative() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_rating_curve_rcoset_negative.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("negative rcoset should fail");

    assert!(matches!(err, WatershedChannelParseError::FieldRange { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-005");
}

#[test]
fn strict_mode_requires_chan_inp_sidecar_when_ipeak_gt_2() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_sidecar_required.chn"),
        WatershedChannelParseOptions {
            chan_inp_present: false,
            ..WatershedChannelParseOptions::default()
        },
    )
    .expect_err("strict mode should fail missing chan.inp sidecar");

    assert!(matches!(
        err,
        WatershedChannelParseError::RequiredSidecarMissing { .. }
    ));
    assert_eq!(err.contract_error_id(), "CHN-E-008");
}

#[test]
fn compatibility_mode_warns_for_missing_chan_inp_sidecar() {
    let parsed = parse_watershed_channel_from_path(
        fixture_path("compat_sidecar_missing_warn.chn"),
        WatershedChannelParseOptions {
            mode: WatershedChannelParseMode::Compatibility,
            chan_inp_present: false,
            ..WatershedChannelParseOptions::default()
        },
    )
    .expect("compat mode should allow missing chan.inp with warning");

    assert!(parsed.sidecar_required);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ChannelWarningCode::ChnW002)
    );
}

#[test]
fn strict_mode_enforces_cross_file_channel_count_closure() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_valid_single_channel.chn"),
        WatershedChannelParseOptions {
            expected_channel_count: Some(2),
            ..WatershedChannelParseOptions::default()
        },
    )
    .expect_err("nchan mismatch must fail");

    assert!(matches!(
        err,
        WatershedChannelParseError::ChannelCountMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "CHN-E-007");
}

#[test]
fn strict_mode_requires_slplst_override_for_icntrl0_closure() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_icntrl0_requires_slplst.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("strict mode should require slplst override for icntrl=0");

    assert!(matches!(
        err,
        WatershedChannelParseError::InvariantViolation { .. }
    ));
    assert_eq!(err.contract_error_id(), "CHN-E-009");
}

#[test]
fn compatibility_mode_applies_icntrl0_override_with_warning() {
    let parsed = parse_watershed_channel_from_path(
        fixture_path("strict_icntrl0_requires_slplst.chn"),
        WatershedChannelParseOptions {
            mode: WatershedChannelParseMode::Compatibility,
            slplst_override: Some(0.35),
            ..WatershedChannelParseOptions::default()
        },
    )
    .expect("compat mode should apply control override");

    let channel = &parsed.channels[0];
    assert!(channel.control_override_applied);
    assert!((channel.ctlslp_effective - 0.35).abs() < 1e-12);
    assert!((channel.ctlz_effective - channel.chnz).abs() < 1e-12);
    assert!((channel.ctln_effective - channel.chnn).abs() < 1e-12);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ChannelWarningCode::ChnW004)
    );
}

#[test]
fn strict_mode_rejects_extra_trailing_records() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_extra_record.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("extra trailing records must fail");

    assert!(matches!(
        err,
        WatershedChannelParseError::RecordClosure { .. }
    ));
    assert_eq!(err.contract_error_id(), "CHN-E-002");
}

#[test]
fn final_no_rating_residuals_use_structural_rating_classification() {
    for mode in [
        WatershedChannelParseMode::Strict,
        WatershedChannelParseMode::Compatibility,
    ] {
        for fixture in [
            "strict_extra_rating_two_tokens.chn",
            "strict_extra_rating_four_tokens.chn",
            "strict_extra_rating_invalid_domain.chn",
        ] {
            let error = parse_watershed_channel_from_path(
                fixture_path(fixture),
                WatershedChannelParseOptions {
                    mode,
                    ..WatershedChannelParseOptions::default()
                },
            )
            .expect_err("generic residual records must not become rating closure errors");
            assert!(matches!(
                error,
                WatershedChannelParseError::RecordClosure {
                    context: "extra_records",
                    expected: 14,
                    found: 15,
                }
            ));
            assert_eq!(error.contract_error_id(), "CHN-E-002");
        }

        let error = parse_watershed_channel_from_path(
            fixture_path("strict_extra_rating_three_tokens.chn"),
            WatershedChannelParseOptions {
                mode,
                ..WatershedChannelParseOptions::default()
            },
        )
        .expect_err("a uniquely recognized prohibited rating record must fail");
        assert!(matches!(
            error,
            WatershedChannelParseError::RatingCurveClosure {
                line: 15,
                channel_id: 1,
                reason: "icntrl!=4 prohibits structurally recognized rating_curve_line",
            }
        ));
        assert_eq!(error.contract_error_id(), "CHN-E-006");
    }
}

#[test]
fn multi_channel_extra_rating_is_recognized_only_by_unique_suffix_closure() {
    for mode in [
        WatershedChannelParseMode::Strict,
        WatershedChannelParseMode::Compatibility,
    ] {
        let error = parse_watershed_channel_from_path(
            fixture_path("strict_multi_extra_rating_boundary.chn"),
            WatershedChannelParseOptions {
                mode,
                ..WatershedChannelParseOptions::default()
            },
        )
        .expect_err("the inserted rating record must not shift the next channel block");
        assert!(matches!(
            error,
            WatershedChannelParseError::RatingCurveClosure {
                line: 15,
                channel_id: 1,
                reason: "icntrl!=4 prohibits structurally recognized rating_curve_line",
            }
        ));
    }
}

#[test]
fn exact_numeric_rating_shaped_comment_is_not_reclassified() {
    for mode in [
        WatershedChannelParseMode::Strict,
        WatershedChannelParseMode::Compatibility,
    ] {
        let parsed = parse_watershed_channel_from_path(
            fixture_path("strict_multi_numeric_comment.chn"),
            WatershedChannelParseOptions {
                mode,
                ..WatershedChannelParseOptions::default()
            },
        )
        .expect("canonical retained suffix must take precedence over rating shape");
        assert_eq!(parsed.channels.len(), 2);
        assert_eq!(parsed.channels[1].comment_1, "1.25 1.50 0.10");
        assert_eq!(parsed.channels[1].comment_2, "1.25 1.50 0.10 channel two");
        assert_eq!(parsed.channels[1].icntrl, 1);
        assert!(parsed.channels[1].rating_curve.is_none());
    }
}

#[test]
fn neither_suffix_layout_preserves_the_ordinary_retained_error() {
    for mode in [
        WatershedChannelParseMode::Strict,
        WatershedChannelParseMode::Compatibility,
    ] {
        let error = parse_watershed_channel_from_path(
            fixture_path("strict_multi_extra_rating_neither_suffix.chn"),
            WatershedChannelParseOptions {
                mode,
                ..WatershedChannelParseOptions::default()
            },
        )
        .expect_err("neither valid layout must retain ordinary parser precedence");
        assert!(matches!(
            error,
            WatershedChannelParseError::TokenParse {
                line: 18,
                field: "ishape",
                ref token,
            } if token == "channel 2 comment c"
        ));
        assert_eq!(error.contract_error_id(), "CHN-E-001");
    }
}

#[test]
fn duplicate_rating_after_enabled_branch_remains_generic_extra_input() {
    for mode in [
        WatershedChannelParseMode::Strict,
        WatershedChannelParseMode::Compatibility,
    ] {
        let error = parse_watershed_channel_from_path(
            fixture_path("strict_duplicate_enabled_rating.chn"),
            WatershedChannelParseOptions {
                mode,
                ..WatershedChannelParseOptions::default()
            },
        )
        .expect_err("a duplicate enabled rating record is generic residual input");
        assert!(matches!(
            error,
            WatershedChannelParseError::RecordClosure {
                context: "extra_records",
                expected: 15,
                found: 16,
            }
        ));
        assert_eq!(error.contract_error_id(), "CHN-E-002");
    }
}

#[test]
fn strict_mode_rejects_non_numeric_tokens() {
    let err = parse_watershed_channel_from_path(
        fixture_path("strict_non_numeric.chn"),
        WatershedChannelParseOptions::default(),
    )
    .expect_err("non-numeric token must fail");

    assert!(matches!(err, WatershedChannelParseError::TokenParse { .. }));
    assert_eq!(err.contract_error_id(), "CHN-E-001");
}

#[test]
fn tcr_overlay_presence_is_exported_as_compatibility_warning_marker() {
    let parsed = parse_watershed_channel_from_path(
        fixture_path("strict_valid_single_channel.chn"),
        WatershedChannelParseOptions {
            mode: WatershedChannelParseMode::Compatibility,
            tcr_overlay_present: true,
            ..WatershedChannelParseOptions::default()
        },
    )
    .expect("tcr overlay marker should parse");

    assert!(parsed.tcr_overlay_present);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ChannelWarningCode::ChnW005)
    );
}
