use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use openwepp_input_contract::parsers::{
    chaninp::{ChaninpParseOptions, ParseMode, parse_chaninp_from_str},
    climate::{CompatibilityOptions, ParserMode as ClimateParserMode, parse_climate_from_str},
    slope::{SlopeParserOptions, parse_slope_str},
    watershed_channel::{WatershedChannelParseOptions, parse_watershed_channel_from_str},
    watershed_impoundment::{
        WatershedImpoundmentParseOptions, parse_watershed_impoundment_from_str,
    },
};
use openwepp_kernel_contract::BoundarySymbol;

use super::types::{WatershedClimateRuntimeInputError, WatershedRuntimeInputError};
use super::{
    chaninp::{
        build_watershed_runtime_surface_from_chaninp,
        seed_watershed_runtime_surface_from_slope_channel_profile,
        seed_watershed_runtime_surface_from_watershed_channel,
        seed_watershed_runtime_surface_from_watershed_impoundment,
    },
    climate::build_watershed_runtime_surface_from_climate_assignments,
};
use crate::WatershedWritebackSurface;

const STRICT_VALID_CLIMATE: &str =
    include_str!("../../../../tests/fixtures/infile/climate/strict_valid.cli");
const LEGACY_DATVER_CLIMATE: &str =
    include_str!("../../../../tests/fixtures/infile/climate/legacy_datver_0.cli");
const BREAKPOINT_OVERFLOW_CLIMATE: &str =
    include_str!("../../../../tests/fixtures/infile/climate/breakpoint_overflow_51.cli");
const WC1_BREAKPOINT_STMSTR_NONZERO: &str = include_str!(
    "../../../../tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli"
);
const WC1_BREAKPOINT_NBRKPT_42: &str = include_str!(
    "../../../../tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_nbrkpt_42.cli"
);
const WC1_UNPALATABLE_RIND_BREAKPOINT_NBRKPT_0: &str = include_str!(
    "../../../../tests/fixtures/infile/climate/wc1_unpalatable_rind_breakpoint_nbrkpt_0.cli"
);
const WC1_CANOGA_DAY1: &str =
    include_str!("../../../../tests/fixtures/infile/climate/wc1_canoga_day1.cli");
const WC1_CANOGA_STMDUR_CAP: &str =
    include_str!("../../../../tests/fixtures/infile/climate/wc1_canoga_stmdur_cap.cli");
const STRICT_VALID_CHANINP: &str =
    include_str!("../../../../tests/fixtures/infile/chaninp/strict_valid.chaninp");
const STRICT_VALID_WATERSHED_CHANNEL: &str = include_str!(
    "../../../../tests/fixtures/infile/watershed_channel/strict_valid_single_channel.chn"
);
const STRICT_ISHAPE_NATURALLY_ERODED_WATERSHED_CHANNEL: &str = include_str!(
    "../../../../tests/fixtures/infile/watershed_channel/strict_ishape_naturally_eroded.chn"
);
const STRICT_VALID_SLOPE: &str =
    include_str!("../../../../tests/fixtures/infile/slope/strict_valid_canonical.slp");
const STRICT_VALID_WATERSHED_IMPOUNDMENT: &str = include_str!(
    "../../../../tests/fixtures/infile/watershed_impoundment/strict_valid_minimal.imp"
);
const STRICT_VALID_WATERSHED_IMPOUNDMENT_ACTIVE: &str = include_str!(
    "../../../../tests/fixtures/infile/watershed_impoundment/strict_valid_active_payloads.imp"
);

fn build_breakpoint_fixture(nbrkpt: usize) -> String {
    let mut climate = format!(
        "5.30\n1 1 0\nTEST STATION 1500\nDAY MON YEAR NBRKPT TMAX TMIN RAD VWIND WIND TDPT\n45.0 -120.0 1000.0 30 2000 1\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n1 1 2000 {nbrkpt} 11.0 1.0 180.0 2.0 170.0 -2.0\n"
    );
    if nbrkpt == 0 {
        return climate;
    }
    let denom_u32 = u32::try_from((nbrkpt - 1).max(1))
        .expect("breakpoint fixture helper expects small cardinalities");
    let denom = f64::from(denom_u32);
    for index in 0..nbrkpt {
        let idx_u32 =
            u32::try_from(index).expect("breakpoint fixture helper expects small cardinalities");
        let idx = f64::from(idx_u32);
        let timem = (24.0 * idx) / denom;
        let pptcum = (120.0 * idx) / denom;
        writeln!(&mut climate, "{timem:.4} {pptcum:.3}")
            .expect("writing synthetic breakpoint fixture should succeed");
    }
    climate
}

#[test]
fn chaninp_runtime_surface_contains_required_symbols() {
    let valid_channel_element_ids = BTreeSet::from([4, 5]);
    let parsed = parse_chaninp_from_str(
        STRICT_VALID_CHANINP,
        ChaninpParseOptions::strict(3, 2),
        &valid_channel_element_ids,
    )
    .expect("strict chan.inp fixture should parse");

    let surface = build_watershed_runtime_surface_from_chaninp(&parsed)
        .expect("runtime surface should build from strict parsed branch");

    let dtchr = surface
        .state_surface
        .get(&BoundarySymbol::from("dtchr"))
        .expect("dtchr should be present")
        .as_f64();
    let ntchr = surface
        .state_surface
        .get(&BoundarySymbol::from("ntchr"))
        .expect("ntchr should be present")
        .as_f64();
    let cbase = surface
        .flux_surface
        .get(&BoundarySymbol::from("cbase"))
        .expect("cbase should be present")
        .as_f64();

    assert!((dtchr - 600.0).abs() < 1e-12);
    assert!((ntchr - 144.0).abs() < 1e-12);
    assert!((cbase - 0.000_001).abs() < 1e-12);
}

#[test]
fn chaninp_runtime_surface_rejects_compat_defaulted_parse_outcome() {
    let valid_channel_element_ids = BTreeSet::from([4, 5]);
    let parsed = parse_chaninp_from_str(
        "invalid\nbranch\nfor\nstrict",
        ChaninpParseOptions {
            mode: ParseMode::Compatibility,
            ..ChaninpParseOptions::compatibility(3, 2)
        },
        &valid_channel_element_ids,
    )
    .expect("compat parser should return defaulted branch instead of hard failure");

    let error = build_watershed_runtime_surface_from_chaninp(&parsed)
        .expect_err("defaulted compat branch is not runtime consumable");
    assert_eq!(error.code(), "WS-RUNTIME-E-001");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ParseOutcomeNotRuntimeReady { .. }
    ));
}

#[test]
#[allow(clippy::similar_names, clippy::too_many_lines)]
fn watershed_channel_runtime_seed_projects_ws10_symbols() {
    let parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");

    let mut surface = WatershedWritebackSurface::default();
    seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect("ws10 channel runtime seed should project symbols");

    let chnn = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_chnn"))
        .expect("ws10_channel_1_chnn should be present")
        .as_f64();
    let ctlslp = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_ctlslp"))
        .expect("ws10_channel_1_ctlslp should be present")
        .as_f64();
    let conductivity = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_chnk"))
        .expect("ws10_channel_1_chnk should be present")
        .as_f64();
    let ishape = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_ishape"))
        .expect("ws10_channel_1_ishape should be present")
        .as_f64();
    let icntrl = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_icntrl"))
        .expect("ws10_channel_1_icntrl should be present")
        .as_f64();
    let flgout = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_flgout"))
        .expect("ws10_channel_1_flgout should be present")
        .as_f64();
    let chnz = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_chnz"))
        .expect("ws10_channel_1_chnz should be present")
        .as_f64();
    let chnnbr = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_chnnbr"))
        .expect("ws10_channel_1_chnnbr should be present")
        .as_f64();
    let chntcr = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_chntcr"))
        .expect("ws10_channel_1_chntcr should be present")
        .as_f64();
    let chnedm = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_chnedm"))
        .expect("ws10_channel_1_chnedm should be present")
        .as_f64();
    let chneds = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_chneds"))
        .expect("ws10_channel_1_chneds should be present")
        .as_f64();
    let ctlz = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_ctlz"))
        .expect("ws10_channel_1_ctlz should be present")
        .as_f64();
    let ctln = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_ctln"))
        .expect("ws10_channel_1_ctln should be present")
        .as_f64();
    let rccoef = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_rccoef"))
        .expect("ws10_channel_1_rccoef should be present")
        .as_f64();
    let rcexp = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_rcexp"))
        .expect("ws10_channel_1_rcexp should be present")
        .as_f64();
    let rcoset = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_rcoset"))
        .expect("ws10_channel_1_rcoset should be present")
        .as_f64();

    assert!((chnn - 0.04).abs() < 1e-12);
    assert!((ctlslp - 0.02).abs() < 1e-12);
    assert!((conductivity - 0.000_001).abs() < 1e-12);
    assert!((ishape - 1.0).abs() < 1e-12);
    assert!((icntrl - 4.0).abs() < 1e-12);
    assert!((flgout - 0.0).abs() < 1e-12);
    assert!((chnz - 19.99).abs() < 1e-12);
    assert!((chnnbr - 0.03).abs() < 1e-12);
    assert!((chntcr - 19.0).abs() < 1e-12);
    assert!((chnedm - 900.0).abs() < 1e-12);
    assert!((chneds - 0.0001).abs() < 1e-12);
    assert!((ctlz - 4.0).abs() < 1e-12);
    assert!((ctln - 0.04).abs() < 1e-12);
    assert!((rccoef - 1.25).abs() < 1e-12);
    assert!((rcexp - 1.5).abs() < 1e-12);
    assert!((rcoset - 0.1).abs() < 1e-12);
}

#[test]
fn watershed_channel_runtime_seed_projects_naturally_eroded_ishape() {
    let parsed = parse_watershed_channel_from_str(
        STRICT_ISHAPE_NATURALLY_ERODED_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict naturally eroded watershed channel fixture should parse");

    let mut surface = WatershedWritebackSurface::default();
    seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect("ws10 channel runtime seed should project naturally eroded ishape");

    let ishape = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_ishape"))
        .expect("ws10_channel_1_ishape should be present")
        .as_f64();
    assert!((ishape - 3.0).abs() < 1e-12);
}

#[test]
fn watershed_channel_runtime_seed_rejects_out_of_domain_ishape() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    parsed.channels[0].ishape = 4;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("out-of-domain ishape must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_ishape"
    ));
}

#[test]
fn watershed_channel_runtime_seed_rejects_out_of_domain_ienslp() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    parsed.channels[0].ienslp = 3;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("out-of-domain ienslp must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_ienslp"
    ));
}

#[test]
fn watershed_channel_runtime_seed_rejects_out_of_domain_icntrl() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    parsed.channels[0].icntrl = 5;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("out-of-domain icntrl must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_icntrl"
    ));
}

#[test]
fn watershed_channel_runtime_seed_rejects_out_of_domain_flgout() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    parsed.channels[0].flgout = 2;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("out-of-domain flgout must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_flgout"
    ));
}

#[test]
fn watershed_channel_runtime_seed_rejects_missing_rating_curve_payload_for_icntrl4() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    parsed.channels[0].rating_curve = None;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("missing rating curve payload for icntrl=4 must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_rating_curve"
    ));
}

#[test]
fn watershed_channel_runtime_seed_rejects_rating_curve_payload_when_icntrl_not4() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    parsed.channels[0].icntrl = 3;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("rating curve payload with icntrl!=4 must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_rating_curve"
    ));
}

#[test]
fn watershed_channel_runtime_seed_rejects_out_of_domain_rccoef() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    let rating_curve = parsed.channels[0]
        .rating_curve
        .as_mut()
        .expect("strict valid fixture should have rating curve");
    rating_curve.rccoef = 0.0;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("non-positive rccoef must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_rccoef"
    ));
}

#[test]
fn watershed_channel_runtime_seed_rejects_out_of_domain_rcexp() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    let rating_curve = parsed.channels[0]
        .rating_curve
        .as_mut()
        .expect("strict valid fixture should have rating curve");
    rating_curve.rcexp = 0.0;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("non-positive rcexp must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_rcexp"
    ));
}

#[test]
fn watershed_channel_runtime_seed_rejects_out_of_domain_rcoset() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    let rating_curve = parsed.channels[0]
        .rating_curve
        .as_mut()
        .expect("strict valid fixture should have rating curve");
    rating_curve.rcoset = -0.1;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("negative rcoset must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_rcoset"
    ));
}

#[test]
fn watershed_channel_runtime_seed_rejects_chnn_less_than_chnnbr() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    parsed.channels[0].chnn = parsed.channels[0].chnnbr - 0.001;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("chnn below chnnbr must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_chnn"
    ));
}

#[test]
fn watershed_channel_runtime_seed_rejects_out_of_domain_symbol() {
    let mut parsed = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    parsed.channels[0].chnn = 0.0;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
        .expect_err("non-positive channel roughness must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_1_chnn"
    ));
}

#[test]
fn watershed_channel_slope_runtime_seed_projects_ws17_segment_symbols() {
    let channel = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    let slope = parse_slope_str(STRICT_VALID_SLOPE, SlopeParserOptions::strict())
        .expect("strict slope fixture should parse");

    let mut surface = WatershedWritebackSurface::default();
    seed_watershed_runtime_surface_from_slope_channel_profile(&mut surface, &channel, &slope)
        .expect("ws17 slope-to-channel seeding should project segment symbols");

    let nslpts = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_nslpts"))
        .expect("ws10_channel_1_nslpts should be present")
        .as_f64();
    let x2 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_x_0002"))
        .expect("ws10_channel_1_x_0002 should be present")
        .as_f64();
    let x3 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_x_0003"))
        .expect("ws10_channel_1_x_0003 should be present")
        .as_f64();
    let slope2 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_slope_0002"))
        .expect("ws10_channel_1_slope_0002 should be present")
        .as_f64();
    let depa2 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_depa_0002"))
        .expect("ws10_channel_1_depa_0002 should be present")
        .as_f64();
    let wida2 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_wida_0002"))
        .expect("ws10_channel_1_wida_0002 should be present")
        .as_f64();

    assert!((nslpts - 3.0).abs() < 1.0e-12);
    assert!((x2 - 36.0).abs() < 1.0e-12);
    assert!((x3 - 60.0).abs() < 1.0e-12);
    assert!((slope2 - 0.08).abs() < 1.0e-12);
    assert!((depa2 - 2_952.9).abs() < 1.0e-9);
    assert!((wida2 - 98.43).abs() < 1.0e-12);
}

#[test]
fn watershed_channel_slope_runtime_seed_rejects_profile_count_mismatch() {
    let mut channel = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    let mut second = channel.channels[0].clone();
    second.channel_id = 2;
    channel.channels.push(second);

    let slope = parse_slope_str(
        "97.5\n1\n180.0 30.0\n3 60.0\n0.0 0.02 0.6 0.08 1.0 0.06\n",
        SlopeParserOptions::strict(),
    )
    .expect("single-profile slope fixture should parse");

    let mut surface = WatershedWritebackSurface::default();
    let error =
        seed_watershed_runtime_surface_from_slope_channel_profile(&mut surface, &channel, &slope)
            .expect_err("slope profile count mismatch must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-010");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_channel_2_nslpts"
    ));
}

#[test]
fn watershed_impoundment_runtime_seed_projects_ws10_symbols() {
    let parsed = parse_watershed_impoundment_from_str(
        STRICT_VALID_WATERSHED_IMPOUNDMENT,
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect("strict watershed impoundment fixture should parse");

    let mut surface = WatershedWritebackSurface::default();
    seed_watershed_runtime_surface_from_watershed_impoundment(&mut surface, &parsed)
        .expect("ws10 impoundment runtime seed should project symbols");

    let h = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_h"))
        .expect("ws10_impoundment_1_h should be present")
        .as_f64();
    let hfull = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_hfull"))
        .expect("ws10_impoundment_1_hfull should be present")
        .as_f64();
    let deltat = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_deltat"))
        .expect("ws10_impoundment_1_deltat should be present")
        .as_f64();
    let a0 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_a0"))
        .expect("ws10_impoundment_1_a0 should be present")
        .as_f64();
    let a1 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_a1"))
        .expect("ws10_impoundment_1_a1 should be present")
        .as_f64();
    let a2 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_a2"))
        .expect("ws10_impoundment_1_a2 should be present")
        .as_f64();
    let l0 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_l0"))
        .expect("ws10_impoundment_1_l0 should be present")
        .as_f64();
    let l1 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_l1"))
        .expect("ws10_impoundment_1_l1 should be present")
        .as_f64();
    let l2 = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_l2"))
        .expect("ws10_impoundment_1_l2 should be present")
        .as_f64();
    let ha = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_ha"))
        .expect("ws10_impoundment_1_ha should be present")
        .as_f64();
    let f01_ha = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_f01_ha"))
        .expect("ws10_impoundment_1_f01_ha should be present")
        .as_f64();
    let f15_b = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_f15_b"))
        .expect("ws10_impoundment_1_f15_b should be present")
        .as_f64();

    assert!((h - 0.70).abs() < 1e-12);
    assert!((hfull - 0.75).abs() < 1e-12);
    assert!((deltat - 1.0).abs() < 1e-12);
    assert!((a0 - 100.0).abs() < 1e-12);
    assert!(a1 > 0.0);
    assert!(a2 > 0.0);
    assert!((l0 - 20.0).abs() < 1e-12);
    assert!(l1 > 0.0);
    assert!(l2 > 0.0);
    assert!((ha - 0.75).abs() < 1e-12);
    assert!((f01_ha - 0.75).abs() < 1e-12);
    assert!(f15_b.abs() <= 1.0e-12);
}

#[test]
fn watershed_impoundment_runtime_seed_projects_active_structure_coefficients() {
    let parsed = parse_watershed_impoundment_from_str(
        STRICT_VALID_WATERSHED_IMPOUNDMENT_ACTIVE,
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect("strict watershed impoundment fixture should parse");
    assert!(
        parsed.items[0].structure_flags.has_drop_spillway,
        "fixture should carry active outlet structures"
    );

    let mut surface = WatershedWritebackSurface::default();
    seed_watershed_runtime_surface_from_watershed_impoundment(&mut surface, &parsed)
        .expect("active structure payloads should project runtime coefficient symbols");

    let a = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_a"))
        .expect("ws10_impoundment_1_a should be present")
        .as_f64();
    let c = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_c"))
        .expect("ws10_impoundment_1_c should be present")
        .as_f64();
    let e = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_e"))
        .expect("ws10_impoundment_1_e should be present")
        .as_f64();
    let f01_b = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_f01_b"))
        .expect("ws10_impoundment_1_f01_b should be present")
        .as_f64();
    let f04_a = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_f04_a"))
        .expect("ws10_impoundment_1_f04_a should be present")
        .as_f64();
    let f10_d = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_f10_d"))
        .expect("ws10_impoundment_1_f10_d should be present")
        .as_f64();
    let f13_b = surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_f13_b"))
        .expect("ws10_impoundment_1_f13_b should be present")
        .as_f64();

    assert!(a.is_finite() && a > 0.0);
    assert!(c.is_finite() && c > 0.0);
    assert!(e.is_finite() && e > 0.0);
    assert!(f01_b.is_finite() && f01_b > 0.0);
    assert!(f04_a.is_finite() && f04_a > 0.0);
    assert!(f10_d.is_finite() && f10_d > 0.0);
    assert!(f13_b.is_finite() && f13_b > 0.0);
}

#[test]
fn watershed_impoundment_runtime_seed_rejects_h_above_hfull() {
    let mut parsed = parse_watershed_impoundment_from_str(
        STRICT_VALID_WATERSHED_IMPOUNDMENT,
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect("strict watershed impoundment fixture should parse");
    parsed.items[0].h = parsed.items[0].hfull + 0.1;

    let mut surface = WatershedWritebackSurface::default();
    let error = seed_watershed_runtime_surface_from_watershed_impoundment(&mut surface, &parsed)
        .expect_err("impoundment stage above hfull must fail");

    assert_eq!(error.code(), "WS-RUNTIME-E-012");
    assert!(matches!(
        error,
        WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain { symbol, .. }
        if symbol == "ws10_impoundment_1_h"
    ));
}

#[test]
fn climate_runtime_surface_contains_per_hillslope_symbols() {
    let climate = parse_climate_from_str(STRICT_VALID_CLIMATE, ClimateParserMode::Strict)
        .expect("strict climate fixture should parse");
    let assignments = BTreeMap::from([(1_u32, climate.clone()), (3_u32, climate)]);

    let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("watershed climate runtime surface should build");

    let nclimhs = surface
        .state_surface
        .get(&BoundarySymbol::from("nclimhs"))
        .expect("nclimhs should be present")
        .as_f64();
    let hs1_prcp = surface
        .state_surface
        .get(&BoundarySymbol::from("hs1_prcp"))
        .expect("hs1_prcp should be present")
        .as_f64();
    let hs3_stmdur = surface
        .state_surface
        .get(&BoundarySymbol::from("hs3_stmdur"))
        .expect("hs3_stmdur should be present")
        .as_f64();
    let hs1_ip = surface
        .state_surface
        .get(&BoundarySymbol::from("hs1_ip"))
        .expect("hs1_ip should be present")
        .as_f64();
    let hs1_ninten = surface
        .state_surface
        .get(&BoundarySymbol::from("hs1_ninten"))
        .expect("hs1_ninten should be present")
        .as_f64();

    assert!((nclimhs - 2.0).abs() < 1e-12);
    assert!((hs1_prcp - 0.01).abs() < 1e-12);
    assert!((hs3_stmdur - 7_200.0).abs() < 1e-12);
    assert!((hs1_ip - 2.1).abs() < 1e-12);
    assert!(hs1_ninten >= 2.0);
}

#[test]
fn breakpoint_runtime_surface_projects_stmstr_elapsed_timem_and_mxint() {
    let climate = parse_climate_from_str(WC1_BREAKPOINT_STMSTR_NONZERO, ClimateParserMode::Strict)
        .expect("curated wc1 breakpoint fixture should parse");
    let assignments = BTreeMap::from([(21_u32, climate)]);
    let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("breakpoint runtime assignments should build");

    let stmstr = surface
        .state_surface
        .get(&BoundarySymbol::from("hs21_stmstr"))
        .expect("hs21_stmstr should exist")
        .as_f64();
    let prcp = surface
        .state_surface
        .get(&BoundarySymbol::from("hs21_prcp"))
        .expect("hs21_prcp should exist")
        .as_f64();
    let stmdur = surface
        .state_surface
        .get(&BoundarySymbol::from("hs21_stmdur"))
        .expect("hs21_stmdur should exist")
        .as_f64();
    let mxint = surface
        .state_surface
        .get(&BoundarySymbol::from("hs21_mxint"))
        .expect("hs21_mxint should exist")
        .as_f64();
    let timem_1 = surface
        .state_surface
        .get(&BoundarySymbol::from("hs21_timem_0001"))
        .expect("hs21_timem_0001 should exist")
        .as_f64();
    let timem_2 = surface
        .state_surface
        .get(&BoundarySymbol::from("hs21_timem_0002"))
        .expect("hs21_timem_0002 should exist")
        .as_f64();
    let intsty_5 = surface
        .state_surface
        .get(&BoundarySymbol::from("hs21_intsty_0005"))
        .expect("hs21_intsty_0005 should exist")
        .as_f64();

    let times_h = [4.8667_f64, 17.2667, 19.4333, 21.3667, 23.9833];
    let pptcum_mm = [0.0_f64, 2.01, 4.02, 6.04, 7.35];
    let expected_stmdur = (times_h[4] - times_h[0]) * 3_600.0;
    let expected_timem_2 = (times_h[1] - times_h[0]) * 3_600.0;
    let mut expected_mxint: f64 = 0.0;
    for index in 1..times_h.len() {
        let drain_m = (pptcum_mm[index] - pptcum_mm[index - 1]) * 0.001;
        let delta_time_s = (times_h[index] - times_h[index - 1]) * 3_600.0;
        expected_mxint = expected_mxint.max(drain_m / delta_time_s);
    }

    assert!((stmstr - 4.8667).abs() < 1e-12);
    assert!((prcp - 0.00735).abs() < 1e-12);
    assert!((stmdur - expected_stmdur).abs() < 1e-6);
    assert!((mxint - expected_mxint).abs() < 1e-12);
    assert!(timem_1.abs() < 1e-12);
    assert!((timem_2 - expected_timem_2).abs() < 1e-6);
    assert!(intsty_5.abs() < 1e-12);
}

#[test]
fn breakpoint_runtime_surface_supports_curated_wc1_42_point_event_shape() {
    let climate = parse_climate_from_str(WC1_BREAKPOINT_NBRKPT_42, ClimateParserMode::Strict)
        .expect("42-point wc1 fixture should parse");
    let assignments = BTreeMap::from([(22_u32, climate)]);
    let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("42-point breakpoint assignments should build");

    let nbrkpt = surface
        .state_surface
        .get(&BoundarySymbol::from("hs22_nbrkpt"))
        .expect("hs22_nbrkpt should exist")
        .as_f64();
    let timem_first = surface
        .state_surface
        .get(&BoundarySymbol::from("hs22_timem_0001"))
        .expect("hs22_timem_0001 should exist")
        .as_f64();
    let timem_last = surface
        .state_surface
        .get(&BoundarySymbol::from("hs22_timem_0042"))
        .expect("hs22_timem_0042 should exist")
        .as_f64();
    let intsty_last = surface
        .state_surface
        .get(&BoundarySymbol::from("hs22_intsty_0042"))
        .expect("hs22_intsty_0042 should exist")
        .as_f64();

    assert!((nbrkpt - 42.0).abs() < 1e-12);
    assert!(timem_first.abs() < 1e-12);
    assert!(timem_last > timem_first);
    assert!(intsty_last.abs() < 1e-12);
}

#[test]
fn breakpoint_runtime_surface_accepts_curated_wc1_zero_breakpoint_dry_day() {
    let climate = parse_climate_from_str(
        WC1_UNPALATABLE_RIND_BREAKPOINT_NBRKPT_0,
        ClimateParserMode::Strict,
    )
    .expect("wc1 zero-breakpoint fixture should parse");
    let assignments = BTreeMap::from([(23_u32, climate)]);
    let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("zero-breakpoint dry day should project watershed runtime surface");

    let nbrkpt = surface
        .state_surface
        .get(&BoundarySymbol::from("hs23_nbrkpt"))
        .expect("hs23_nbrkpt should exist")
        .as_f64();
    let prcp = surface
        .state_surface
        .get(&BoundarySymbol::from("hs23_prcp"))
        .expect("hs23_prcp should exist")
        .as_f64();
    let stmdur = surface
        .state_surface
        .get(&BoundarySymbol::from("hs23_stmdur"))
        .expect("hs23_stmdur should exist")
        .as_f64();
    let mxint = surface
        .state_surface
        .get(&BoundarySymbol::from("hs23_mxint"))
        .expect("hs23_mxint should exist")
        .as_f64();
    let stmstr = surface
        .state_surface
        .get(&BoundarySymbol::from("hs23_stmstr"))
        .expect("hs23_stmstr should exist")
        .as_f64();

    assert!(nbrkpt.abs() < 1e-12);
    assert!(prcp.abs() < 1e-12);
    assert!(stmdur.abs() < 1e-12);
    assert!(mxint.abs() < 1e-12);
    assert!(stmstr.abs() < 1e-12);
    assert!(
        !surface
            .state_surface
            .contains_key(&BoundarySymbol::from("hs23_timem_0001"))
    );
    assert!(
        !surface
            .state_surface
            .contains_key(&BoundarySymbol::from("hs23_intsty_0001"))
    );
}

#[test]
fn climate_runtime_surface_accepts_breakpoint_cardinality_at_1500_boundary() {
    let climate =
        parse_climate_from_str(&build_breakpoint_fixture(1_500), ClimateParserMode::Strict)
            .expect("strict parser should accept 1500 breakpoint rows");
    let assignments = BTreeMap::from([(14_u32, climate)]);

    let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("runtime seam should accept 1500 breakpoint rows");
    let nbrkpt = surface
        .state_surface
        .get(&BoundarySymbol::from("hs14_nbrkpt"))
        .expect("hs14_nbrkpt should exist")
        .as_f64();

    assert!((nbrkpt - 1_500.0).abs() < 1e-12);
}

#[test]
fn climate_runtime_surface_rejects_breakpoint_cardinality_over_1500_even_with_parser_override() {
    let climate = parse_climate_from_str(
        &build_breakpoint_fixture(1_501),
        ClimateParserMode::Compatibility(CompatibilityOptions {
            allow_single_storm: false,
            allow_breakpoint_cardinality_override: true,
            allow_legacy_zero_drain_non_positive_dtime: false,
        }),
    )
    .expect("compat parser should allow >1500 breakpoint rows with explicit override");
    let assignments = BTreeMap::from([(16_u32, climate)]);

    let error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect_err("runtime seam must reject >1500 breakpoint rows");
    assert_eq!(error.code(), "CLIM-RUNTIME-E-011");
    assert!(matches!(
        error,
        WatershedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
            hillslope_id: 16,
            value: 1_501,
            max: 1_500
        }
    ));
}

#[test]
fn climate_runtime_surface_rejects_empty_assignment_map() {
    let assignments: BTreeMap<u32, openwepp_input_contract::parsers::climate::ClimateFile> =
        BTreeMap::new();
    let error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect_err("empty assignment map must fail");

    assert_eq!(error.code(), "CLIM-RUNTIME-E-012");
    assert!(matches!(
        error,
        WatershedClimateRuntimeInputError::EmptyClimateAssignments
    ));
}

#[test]
fn climate_runtime_surface_supports_explicit_datver_zero_override() {
    let legacy = parse_climate_from_str(LEGACY_DATVER_CLIMATE, ClimateParserMode::Strict)
        .expect("legacy datver fixture should parse");
    let assignments = BTreeMap::from([(5_u32, legacy)]);

    let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("datver=0.0 override should be accepted");
    let iclig = surface
        .state_surface
        .get(&BoundarySymbol::from("hs5_iclig"))
        .expect("hs5_iclig should exist")
        .as_f64();
    let ip = surface
        .state_surface
        .get(&BoundarySymbol::from("hs5_ip"))
        .expect("hs5_ip should exist")
        .as_f64();
    assert!((iclig - 0.0).abs() < 1e-12);
    assert!((ip - 2.0).abs() < 1e-12);
}

#[test]
fn climate_runtime_surface_applies_timep_floor_for_wet_nonconstant_events() {
    let climate = parse_climate_from_str(WC1_CANOGA_DAY1, ClimateParserMode::Strict)
        .expect("wc1 fixture should parse");
    let assignments = BTreeMap::from([(11_u32, climate)]);

    let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("wc1 runtime assignments should build");
    let timep = surface
        .state_surface
        .get(&BoundarySymbol::from("hs11_timep"))
        .expect("hs11_timep should exist")
        .as_f64();
    let ip = surface
        .state_surface
        .get(&BoundarySymbol::from("hs11_ip"))
        .expect("hs11_ip should exist")
        .as_f64();
    assert!((timep - 0.01).abs() < 1e-12);
    assert!((ip - 2.94).abs() < 1e-12);
}

#[test]
fn climate_runtime_surface_caps_storm_duration_to_23_999_hours() {
    let climate = parse_climate_from_str(WC1_CANOGA_STMDUR_CAP, ClimateParserMode::Strict)
        .expect("wc1 duration-cap fixture should parse");
    let assignments = BTreeMap::from([(12_u32, climate)]);

    let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("duration-cap assignments should build");
    let stmdur = surface
        .state_surface
        .get(&BoundarySymbol::from("hs12_stmdur"))
        .expect("hs12_stmdur should exist")
        .as_f64();
    let ip = surface
        .state_surface
        .get(&BoundarySymbol::from("hs12_ip"))
        .expect("hs12_ip should exist")
        .as_f64();
    assert!((stmdur - (23.999 * 3_600.0)).abs() < 1e-9);
    assert!((ip - 22.589).abs() < 1e-12);
}

#[test]
fn climate_runtime_surface_rejects_pre4_nonzero_datver_branch() {
    let mut climate = parse_climate_from_str(STRICT_VALID_CLIMATE, ClimateParserMode::Strict)
        .expect("strict climate fixture should parse");
    climate.datver = 3.9;
    let assignments = BTreeMap::from([(7_u32, climate)]);

    let error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect_err("pre-4 nonzero datver must fail at watershed seam");
    assert_eq!(error.code(), "CLIM-RUNTIME-E-001");
    assert!(matches!(
        error,
        WatershedClimateRuntimeInputError::UnsupportedDatver { datver } if (datver - 3.9).abs() < 1e-12
    ));
}

#[test]
fn climate_runtime_surface_rejects_duplicate_breakpoint_times() {
    let mut climate = parse_climate_from_str(
        BREAKPOINT_OVERFLOW_CLIMATE,
        ClimateParserMode::Compatibility(CompatibilityOptions {
            allow_single_storm: false,
            allow_breakpoint_cardinality_override: true,
            allow_legacy_zero_drain_non_positive_dtime: false,
        }),
    )
    .expect("breakpoint fixture should parse in compatibility mode");

    let day = climate
        .daily_records
        .first_mut()
        .expect("one breakpoint day expected");
    match day {
        openwepp_input_contract::parsers::climate::ClimateDailyRecord::Breakpoint(record) => {
            let first_timem = record
                .breakpoints
                .first()
                .expect("first breakpoint point should exist")
                .timem;
            record
                .breakpoints
                .get_mut(1)
                .expect("second breakpoint point should exist")
                .timem = first_timem;
        }
        openwepp_input_contract::parsers::climate::ClimateDailyRecord::NoBreakpoint(_) => {
            panic!("expected breakpoint daily record")
        }
    }
    let assignments = BTreeMap::from([(2_u32, climate)]);

    let error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect_err("duplicate breakpoint timem must fail seam guard");
    assert_eq!(error.code(), "CLIM-RUNTIME-E-009");
    assert!(matches!(
        error,
        WatershedClimateRuntimeInputError::NonMonotoneBreakpointTime { .. }
    ));
}

#[test]
fn climate_runtime_surface_rejects_negative_breakpoint_drain() {
    let mut climate = parse_climate_from_str(
        BREAKPOINT_OVERFLOW_CLIMATE,
        ClimateParserMode::Compatibility(CompatibilityOptions {
            allow_single_storm: false,
            allow_breakpoint_cardinality_override: true,
            allow_legacy_zero_drain_non_positive_dtime: false,
        }),
    )
    .expect("breakpoint fixture should parse in compatibility mode");

    let day = climate
        .daily_records
        .first_mut()
        .expect("one breakpoint day expected");
    match day {
        openwepp_input_contract::parsers::climate::ClimateDailyRecord::Breakpoint(record) => {
            record
                .breakpoints
                .first_mut()
                .expect("first breakpoint point should exist")
                .pptcum = 0.02;
            record
                .breakpoints
                .get_mut(1)
                .expect("second breakpoint point should exist")
                .pptcum = 0.01;
        }
        openwepp_input_contract::parsers::climate::ClimateDailyRecord::NoBreakpoint(_) => {
            panic!("expected breakpoint daily record")
        }
    }
    let assignments = BTreeMap::from([(2_u32, climate)]);

    let error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect_err("negative breakpoint drain must fail seam guard");
    assert_eq!(error.code(), "CLIM-RUNTIME-E-006");
    assert!(matches!(
        error,
        WatershedClimateRuntimeInputError::NegativeField {
            field: "drain",
            value
        } if value < 0.0
    ));
}
