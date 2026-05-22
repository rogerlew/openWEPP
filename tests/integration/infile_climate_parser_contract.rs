use std::fmt::Write as _;
use std::path::PathBuf;

use openwepp_input_contract::parsers::climate::{
    ClimateDailyRecord, ClimateParseError, CompatibilityOptions, ParserMode, parse_climate_file,
    parse_climate_from_str,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/climate").join(name)
}

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
fn strict_mode_accepts_valid_non_breakpoint_fixture() {
    let parsed = parse_climate_file(fixture_path("strict_valid.cli"), ParserMode::Strict).unwrap();

    assert!((parsed.datver - 5.3).abs() < 1e-9);
    assert_eq!(parsed.mode.itemp, 1);
    assert!(!parsed.mode.breakpoint_enabled);
    assert_eq!(parsed.station_id, "TEST STATION 0001");
    assert_eq!(
        parsed.metadata.generator_cmd.as_deref(),
        Some("CLIGEN 5.30 --seed 123")
    );
    assert_eq!(parsed.daily_records.len(), 2);
}

#[test]
fn strict_mode_rejects_itemp2_single_storm() {
    let err = parse_climate_file(fixture_path("single_storm_itemp2.cli"), ParserMode::Strict)
        .unwrap_err();

    assert!(matches!(
        err,
        ClimateParseError::SingleStormUnsupported { .. }
    ));
}

#[test]
fn compat_mode_allows_itemp2_when_enabled() {
    let mode = ParserMode::Compatibility(CompatibilityOptions {
        allow_single_storm: true,
        allow_breakpoint_cardinality_override: false,
        allow_legacy_zero_drain_non_positive_dtime: false,
    });
    let parsed = parse_climate_file(fixture_path("single_storm_itemp2.cli"), mode).unwrap();

    assert_eq!(parsed.mode.itemp, 2);
    assert_eq!(parsed.daily_records.len(), 1);
}

#[test]
fn strict_mode_rejects_unsupported_datver() {
    let err =
        parse_climate_file(fixture_path("unsupported_datver.cli"), ParserMode::Strict).unwrap_err();

    assert!(matches!(err, ClimateParseError::UnsupportedDatver { .. }));
}

#[test]
fn strict_mode_accepts_breakpoint_cardinality_51_under_1500_policy() {
    let parsed = parse_climate_file(
        fixture_path("breakpoint_overflow_51.cli"),
        ParserMode::Strict,
    )
    .unwrap();
    let record = parsed.daily_records.first().expect("one daily record");
    match record {
        ClimateDailyRecord::Breakpoint(day) => {
            assert_eq!(day.nbrkpt, 51);
            assert_eq!(day.breakpoints.len(), 51);
        }
        ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint daily record"),
    }
}

#[test]
fn strict_mode_accepts_curated_wc1_breakpoint_fixture_with_42_points() {
    let parsed = parse_climate_file(
        fixture_path("wc1_major_restlessness_breakpoint_nbrkpt_42.cli"),
        ParserMode::Strict,
    )
    .expect("curated wc1 breakpoint fixture should parse in strict mode");
    let record = parsed.daily_records.first().expect("one daily record");
    match record {
        ClimateDailyRecord::Breakpoint(day) => {
            assert_eq!(day.nbrkpt, 42);
            assert_eq!(day.breakpoints.len(), 42);
        }
        ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint daily record"),
    }
}

#[test]
fn strict_mode_accepts_breakpoint_cardinality_at_1500_boundary() {
    let parsed = parse_climate_from_str(&build_breakpoint_fixture(1_500), ParserMode::Strict)
        .expect("1500 breakpoint rows should parse in strict mode");
    let record = parsed.daily_records.first().expect("one daily record");
    match record {
        ClimateDailyRecord::Breakpoint(day) => {
            assert_eq!(day.nbrkpt, 1_500);
            assert_eq!(day.breakpoints.len(), 1_500);
        }
        ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint daily record"),
    }
}

#[test]
fn strict_mode_rejects_breakpoint_cardinality_over_1500() {
    let err = parse_climate_from_str(&build_breakpoint_fixture(1_501), ParserMode::Strict)
        .expect_err("strict mode must reject >1500 breakpoints");
    assert!(matches!(
        err,
        ClimateParseError::BreakpointCardinality { max: 1_500, .. }
    ));
}

#[test]
fn compat_mode_can_override_breakpoint_cardinality_policy() {
    let mode = ParserMode::Compatibility(CompatibilityOptions {
        allow_single_storm: false,
        allow_breakpoint_cardinality_override: true,
        allow_legacy_zero_drain_non_positive_dtime: false,
    });
    let parsed = parse_climate_from_str(&build_breakpoint_fixture(1_501), mode)
        .expect("compat mode override should allow >1500 breakpoint rows");

    assert_eq!(parsed.daily_records.len(), 1);
    let record = parsed.daily_records.first().expect("one daily record");
    match record {
        ClimateDailyRecord::Breakpoint(day) => {
            assert_eq!(day.nbrkpt, 1_501);
            assert_eq!(day.breakpoints.len(), 1_501);
        }
        ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint daily record"),
    }
}

#[test]
fn strict_mode_rejects_non_monotone_breakpoint_precipitation() {
    let err = parse_climate_file(
        fixture_path("breakpoint_non_monotone.cli"),
        ParserMode::Strict,
    )
    .unwrap_err();

    assert!(matches!(
        err,
        ClimateParseError::BreakpointMonotonicity { .. }
    ));
}

#[test]
fn strict_mode_rejects_duplicate_breakpoint_times() {
    let err = parse_climate_file(
        fixture_path("breakpoint_duplicate_timem.cli"),
        ParserMode::Strict,
    )
    .expect_err("duplicate breakpoint timem must fail strict policy");
    assert!(matches!(
        err,
        ClimateParseError::BreakpointTimeMonotonicity { .. }
    ));
}

#[test]
fn compat_mode_can_enable_legacy_zero_drain_non_positive_dtime() {
    let mode = ParserMode::Compatibility(CompatibilityOptions {
        allow_single_storm: false,
        allow_breakpoint_cardinality_override: false,
        allow_legacy_zero_drain_non_positive_dtime: true,
    });
    let parsed = parse_climate_file(fixture_path("breakpoint_duplicate_timem.cli"), mode)
        .expect("explicit legacy control should allow duplicate timem when drain is zero");
    let record = parsed.daily_records.first().expect("one daily record");
    match record {
        ClimateDailyRecord::Breakpoint(day) => {
            assert_eq!(day.nbrkpt, 3);
            assert_eq!(day.breakpoints.len(), 3);
        }
        ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint daily record"),
    }
}

#[test]
fn strict_mode_rejects_daily_record_arity_mismatch() {
    let err = parse_climate_file(
        fixture_path("malformed_daily_arity.cli"),
        ParserMode::Strict,
    )
    .unwrap_err();

    assert!(matches!(err, ClimateParseError::RecordArity { .. }));
}

#[test]
fn parse_file_reports_io_error_for_missing_path() {
    let err =
        parse_climate_file(fixture_path("does-not-exist.cli"), ParserMode::Strict).unwrap_err();

    assert!(matches!(err, ClimateParseError::Io { .. }));
}
