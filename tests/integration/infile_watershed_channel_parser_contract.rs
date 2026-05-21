use std::path::PathBuf;

use openwepp_input_contract::parsers::watershed_channel::{
    ChannelWarningCode, WatershedChannelParseError, WatershedChannelParseMode,
    WatershedChannelParseOptions, parse_watershed_channel_from_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/watershed_channel").join(name)
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
    assert_eq!(channel.ishape, 1);
    assert_eq!(channel.icntrl, 4);
    assert!(channel.has_rating_curve);
    assert!(channel.rating_curve.is_some());
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
fn compatibility_mode_normalizes_legacy_ishape() {
    let parsed = parse_watershed_channel_from_path(
        fixture_path("compat_ishape_normalized.chn"),
        WatershedChannelParseOptions {
            mode: WatershedChannelParseMode::Compatibility,
            ..WatershedChannelParseOptions::default()
        },
    )
    .expect("compat mode should normalize ishape>2");

    assert_eq!(parsed.channels[0].ishape, 2);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ChannelWarningCode::ChnW003)
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
