use std::path::PathBuf;

use openwepp_input_contract::parsers::climate::{
    ClimateDailyRecord, ClimateParseError, CompatibilityOptions, ParserMode, parse_climate_file,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/climate").join(name)
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
fn strict_mode_rejects_breakpoint_cardinality_over_50() {
    let err = parse_climate_file(
        fixture_path("breakpoint_overflow_51.cli"),
        ParserMode::Strict,
    )
    .unwrap_err();

    assert!(matches!(
        err,
        ClimateParseError::BreakpointCardinality { .. }
    ));
}

#[test]
fn compat_mode_can_override_breakpoint_cardinality_policy() {
    let mode = ParserMode::Compatibility(CompatibilityOptions {
        allow_single_storm: false,
        allow_breakpoint_cardinality_override: true,
    });
    let parsed = parse_climate_file(fixture_path("breakpoint_overflow_51.cli"), mode).unwrap();

    assert_eq!(parsed.daily_records.len(), 1);
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
