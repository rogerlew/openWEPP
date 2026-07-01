use std::error::Error as _;
use std::fmt::Write as _;
use std::path::PathBuf;

use openwepp_hillslope_orchestrator::runtime_inputs::build_hillslope_climate_runtime_request;
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

fn assert_close(observed: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(
        (observed - expected).abs() <= tolerance,
        "{label}: observed {observed}, expected {expected}"
    );
}

fn reconstruct_precip_m(timem_s: &[f64], intsty_m_s: &[f64]) -> f64 {
    timem_s
        .windows(2)
        .zip(intsty_m_s.iter())
        .map(|(window, intensity)| (window[1] - window[0]) * *intensity)
        .sum()
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
fn strict_climate_parser_projects_non_breakpoint_direct_runtime_request() {
    let parsed = parse_climate_file(fixture_path("strict_valid.cli"), ParserMode::Strict)
        .expect("strict valid climate should parse");

    let request = build_hillslope_climate_runtime_request(&parsed)
        .expect("strict climate should project to hillslope runtime request");

    assert_close(
        request.direct_latitude_degrees(),
        45.0,
        1.0e-12,
        "direct latitude",
    );
    assert_close(
        request.direct_elevation_m(),
        1000.0,
        1.0e-12,
        "direct elevation",
    );
    assert_close(
        request.direct_monthly_max_c()[0],
        1.0,
        1.0e-12,
        "monthly maximum temperature",
    );
    assert_close(
        request.direct_monthly_min_c()[11],
        6.0,
        1.0e-12,
        "monthly minimum temperature",
    );

    let day1 = request
        .direct_day_forcing(0)
        .expect("day 1 direct forcing should project");
    assert_close(day1.prcp_m, 0.010, 1.0e-12, "day 1 precipitation");
    assert_close(day1.tmax_c, 12.0, 1.0e-12, "day 1 tmax");
    assert_close(day1.tmin_c, 2.0, 1.0e-12, "day 1 tmin");
    assert_close(day1.rad_ly, 200.0, 1.0e-12, "day 1 radiation");
    assert_close(day1.vwind_m_s, 3.0, 1.0e-12, "day 1 wind speed");
    assert_close(day1.wind_deg, 180.0, 1.0e-12, "day 1 wind direction");
    assert_close(day1.tdpt_c, -1.0, 1.0e-12, "day 1 dew point");
    assert_eq!(day1.timem_s.len(), day1.intsty_m_s.len());
    assert_eq!(day1.timem_s.len(), 11);
    assert_close(
        reconstruct_precip_m(&day1.timem_s, &day1.intsty_m_s),
        day1.prcp_m,
        1.0e-12,
        "day 1 disaggregated precipitation closure",
    );

    let day2 = request
        .direct_day_forcing(1)
        .expect("day 2 dry direct forcing should project");
    assert_close(day2.prcp_m, 0.0, 1.0e-12, "day 2 precipitation");
    assert!(day2.timem_s.is_empty());
    assert!(day2.intsty_m_s.is_empty());
}

#[test]
fn strict_mode_accepts_datver_5_323_and_canonicalizes_to_5_3() {
    let parsed = parse_climate_file(fixture_path("datver_5_323.cli"), ParserMode::Strict)
        .expect("5.323 should be accepted as CLIGEN 5.3-family datver");

    assert!((parsed.datver - 5.3).abs() < 1e-9);
    assert_eq!(
        parsed.metadata.generator_cmd.as_deref(),
        Some("CLIGEN 5.323 --seed 123")
    );
    assert_eq!(parsed.daily_records.len(), 2);
}

#[test]
fn legacy_datver_zero_projects_direct_runtime_without_v4_intensity_policy() {
    let legacy = parse_climate_file(fixture_path("legacy_datver_0.cli"), ParserMode::Strict)
        .expect("legacy datver 0 climate should parse");
    let v4_source =
        include_str!("../fixtures/infile/climate/legacy_datver_0.cli").replacen("0.0", "5.30", 1);
    let v4 = parse_climate_from_str(&v4_source, ParserMode::Strict)
        .expect("v4 policy comparison climate should parse");

    let legacy_request = build_hillslope_climate_runtime_request(&legacy)
        .expect("datver 0 climate should project with legacy override");
    let v4_request = build_hillslope_climate_runtime_request(&v4)
        .expect("datver 5.30 climate should project with v4 policy");

    let legacy_day = legacy_request
        .direct_day_forcing(0)
        .expect("legacy direct forcing should project");
    let v4_day = v4_request
        .direct_day_forcing(0)
        .expect("v4 direct forcing should project");

    assert_close(legacy_day.prcp_m, 0.005, 1.0e-12, "legacy precipitation");
    assert_close(
        v4_day.prcp_m,
        legacy_day.prcp_m,
        1.0e-12,
        "v4 precipitation",
    );
    assert_close(
        reconstruct_precip_m(&legacy_day.timem_s, &legacy_day.intsty_m_s),
        legacy_day.prcp_m,
        1.0e-12,
        "legacy disaggregated precipitation closure",
    );
    assert_close(
        reconstruct_precip_m(&v4_day.timem_s, &v4_day.intsty_m_s),
        v4_day.prcp_m,
        1.0e-12,
        "v4 disaggregated precipitation closure",
    );

    let legacy_peak = legacy_day.intsty_m_s.iter().copied().fold(0.0, f64::max);
    let v4_peak = v4_day.intsty_m_s.iter().copied().fold(0.0, f64::max);
    assert!(
        legacy_peak > v4_peak,
        "datver 0 override must not apply the datver>=4.0 intensity reduction"
    );
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
fn compatibility_itemp2_parse_is_rejected_by_hillslope_runtime_request() {
    let mode = ParserMode::Compatibility(CompatibilityOptions {
        allow_single_storm: true,
        allow_breakpoint_cardinality_override: false,
        allow_legacy_zero_drain_non_positive_dtime: false,
    });
    let parsed = parse_climate_file(fixture_path("single_storm_itemp2.cli"), mode)
        .expect("compat parser should allow itemp=2");

    let error = build_hillslope_climate_runtime_request(&parsed)
        .expect_err("hillslope runtime request must reject itemp=2");

    assert_eq!(error.code(), "CLIM-RUNTIME-E-002");
}

#[test]
fn strict_mode_rejects_unsupported_datver() {
    let err =
        parse_climate_file(fixture_path("unsupported_datver.cli"), ParserMode::Strict).unwrap_err();

    assert!(matches!(err, ClimateParseError::UnsupportedDatver { .. }));
}

#[test]
fn strict_mode_rejects_datver_5_4_boundary() {
    let src =
        include_str!("../fixtures/infile/climate/strict_valid.cli").replacen("5.30", "5.4", 1);
    let err = parse_climate_from_str(&src, ParserMode::Strict)
        .expect_err("5.4 must remain outside the accepted 5.3-family domain");

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
fn strict_mode_accepts_curated_wc1_breakpoint_fixture_with_zero_points() {
    let parsed = parse_climate_file(
        fixture_path("wc1_unpalatable_rind_breakpoint_nbrkpt_0.cli"),
        ParserMode::Strict,
    )
    .expect("curated wc1 zero-breakpoint fixture should parse in strict mode");
    let record = parsed.daily_records.first().expect("one daily record");
    match record {
        ClimateDailyRecord::Breakpoint(day) => {
            assert_eq!(day.nbrkpt, 0);
            assert!(day.breakpoints.is_empty());
        }
        ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint daily record"),
    }
}

#[test]
fn strict_climate_parser_projects_breakpoint_direct_runtime_request() {
    let parsed = parse_climate_file(
        fixture_path("wc1_major_restlessness_breakpoint_nbrkpt_42.cli"),
        ParserMode::Strict,
    )
    .expect("curated breakpoint climate should parse");

    let request = build_hillslope_climate_runtime_request(&parsed)
        .expect("breakpoint climate should project to hillslope runtime request");

    assert_close(
        request.direct_latitude_degrees(),
        40.66,
        1.0e-12,
        "breakpoint latitude",
    );
    assert_close(
        request.direct_elevation_m(),
        3253.0,
        1.0e-12,
        "breakpoint elevation",
    );

    let day = request
        .direct_day_forcing(0)
        .expect("breakpoint direct forcing should project");

    assert_close(day.prcp_m, 0.0888, 1.0e-12, "breakpoint precipitation");
    assert_close(day.tmax_c, 13.5, 1.0e-12, "breakpoint tmax");
    assert_close(day.tmin_c, 8.5, 1.0e-12, "breakpoint tmin");
    assert_close(day.rad_ly, 193.0, 1.0e-12, "breakpoint radiation");
    assert_close(day.vwind_m_s, 3.7, 1.0e-12, "breakpoint wind speed");
    assert_close(day.wind_deg, 0.0, 1.0e-12, "breakpoint wind direction");
    assert_close(day.tdpt_c, 9.5, 1.0e-12, "breakpoint dew point");
    assert_eq!(day.timem_s.len(), 42);
    assert_eq!(day.intsty_m_s.len(), 42);
    assert_close(day.timem_s[0], 0.0, 1.0e-12, "first breakpoint time");
    assert_close(
        day.timem_s[1],
        0.3 * 3600.0,
        1.0e-9,
        "second breakpoint time",
    );
    assert_close(
        day.intsty_m_s[0],
        0.00229 / (0.3 * 3600.0),
        1.0e-15,
        "first breakpoint intensity",
    );
    assert_close(
        reconstruct_precip_m(&day.timem_s, &day.intsty_m_s),
        day.prcp_m,
        1.0e-12,
        "breakpoint precipitation closure",
    );
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
fn direct_day_forcing_reports_out_of_range_runtime_error() {
    let parsed = parse_climate_file(fixture_path("strict_valid.cli"), ParserMode::Strict)
        .expect("strict valid climate should parse");
    let request = build_hillslope_climate_runtime_request(&parsed)
        .expect("strict climate should project to runtime request");

    let error = request
        .direct_day_forcing(2)
        .expect_err("day index beyond parsed forcing span must fail");

    assert_eq!(error.code(), "CLIM-RUNTIME-E-004");
    assert_eq!(
        error.to_string(),
        "CLIM-RUNTIME-E-004: requested day index 2 exceeds available climate records 2"
    );
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

fn assert_climate_error_display(error: &ClimateParseError, expected: &str) {
    assert_eq!(error.to_string(), expected);
}

#[test]
fn climate_parse_error_display_strings_are_stable_for_io_and_record_shape() {
    assert_climate_error_display(
        &ClimateParseError::Io {
            path: PathBuf::from("missing.cli"),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "boom"),
        },
        "failed to read climate file 'missing.cli': boom",
    );
    assert_climate_error_display(
        &ClimateParseError::UnexpectedEof {
            context: "daily record",
        },
        "unexpected end of file while parsing daily record",
    );
    assert_climate_error_display(
        &ClimateParseError::RecordArity {
            line: 7,
            context: "metadata line",
            expected: 6,
            found: 5,
        },
        "line 7: metadata line expects 6 token(s), found 5",
    );
    assert_climate_error_display(
        &ClimateParseError::TokenParse {
            line: 8,
            field: "datver",
            token: "bad".to_string(),
        },
        "line 8: failed to parse field 'datver' from token 'bad'",
    );
}

#[test]
fn climate_parse_error_display_strings_are_stable_for_domain_errors() {
    assert_climate_error_display(
        &ClimateParseError::UnsupportedDatver {
            line: 1,
            value: 5.4,
        },
        "line 1: unsupported datver '5.4'",
    );
    assert_climate_error_display(
        &ClimateParseError::EnumDomain {
            line: 2,
            field: "iwind",
            value: 3,
        },
        "line 2: value '3' is out of domain for 'iwind'",
    );
    assert_climate_error_display(
        &ClimateParseError::SingleStormUnsupported { line: 2 },
        "line 2: single-storm mode (itemp=2) is unsupported",
    );
    assert_climate_error_display(
        &ClimateParseError::FieldRange {
            line: 12,
            field: "prcp",
            value: -0.1,
        },
        "line 12: value '-0.1' violates range for 'prcp'",
    );
    assert_climate_error_display(
        &ClimateParseError::DateDomain {
            line: 13,
            day: 31,
            month: 2,
            year: 2001,
        },
        "line 13: invalid date tuple (31, 2, 2001)",
    );
}

#[test]
fn climate_parse_error_display_strings_are_stable_for_breakpoints_and_counts() {
    assert_climate_error_display(
        &ClimateParseError::BreakpointCardinality {
            line: 14,
            nbrkpt: 1501,
            max: 1500,
        },
        "line 14: breakpoint count '1501' exceeds policy max '1500'",
    );
    assert_climate_error_display(
        &ClimateParseError::BreakpointMonotonicity {
            line: 15,
            previous: 2.5,
            current: 2.4,
        },
        "line 15: cumulative breakpoint precipitation must be monotone: previous=2.5, current=2.4",
    );
    assert_climate_error_display(
        &ClimateParseError::BreakpointTimeMonotonicity {
            line: 16,
            previous: 1.0,
            current: 1.0,
        },
        "line 16: breakpoint timem must be strictly increasing: previous=1, current=1",
    );
    assert_climate_error_display(
        &ClimateParseError::RecordCount {
            context: "daily records",
            expected: 2,
            found: 1,
        },
        "daily records: expected 2, found 1",
    );
    assert_climate_error_display(
        &ClimateParseError::InvariantViolation {
            line: 17,
            context: "daily sequence",
        },
        "line 17: invariant violation for daily sequence",
    );
}

#[test]
fn climate_parse_error_source_is_only_io_source() {
    let io_error = ClimateParseError::Io {
        path: PathBuf::from("missing.cli"),
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "boom"),
    };
    assert!(io_error.source().is_some());

    let non_io_error = ClimateParseError::UnexpectedEof {
        context: "daily record",
    };
    assert!(non_io_error.source().is_none());
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
