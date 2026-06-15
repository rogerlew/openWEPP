use openwepp_input_contract::parsers::irrigation_fixeddate::{
    FixedDateEvent, ParseMode as FixedDateParseMode, parse_fixeddate_str,
};

const FIXEDDATE_STRICT_VALID_SPRINKLER: &str =
    include_str!("../../../../../tests/fixtures/infile/irrigation_fixeddate/strict_valid_sprinkler.ifd");
const FIXEDDATE_STRICT_VALID_FURROW: &str =
    include_str!("../../../../../tests/fixtures/infile/irrigation_fixeddate/strict_valid_furrow.ifd");

fn fixeddate_runtime_scalar(surface: &crate::HillslopeWritebackSurface, symbol: &str) -> f64 {
    surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("{symbol} should be present"))
        .as_f64()
}

#[test]
fn fixeddate_irrigation_runtime_projects_sprinkler_events() {
    let fixeddate =
        parse_fixeddate_str(FIXEDDATE_STRICT_VALID_SPRINKLER, FixedDateParseMode::Strict)
            .expect("strict sprinkler fixed-date fixture should parse");

    let surface = super::build_hillslope_runtime_surface_from_irrigation_fixeddate(&fixeddate)
        .expect("fixed-date sprinkler runtime projection should build");

    assert!((fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.enabled") - 1.0).abs() < 1e-12);
    assert!((fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.datver") - 95.7).abs() < 1e-12);
    assert!((fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.ofe_count") - 2.0).abs() < 1e-12);
    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.system_type") - 1.0).abs()
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.schedule_type") - 2.0).abs()
            < 1e-12
    );
    assert!((fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.event_count") - 2.0).abs() < 1e-12);

    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.event_0001.ofe_id") - 1.0)
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.event_0001.day") - 120.0)
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.event_0001.year") - 1.0)
            < 1e-12
    );
    assert!(
        fixeddate_runtime_scalar(
            &surface,
            "irrigation.fixeddate.event_0001.schedule_termination_flag"
        )
        .abs()
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(
            &surface,
            "irrigation.fixeddate.event_0001.sprinkler_rate_m_per_s"
        ) - 0.000_020)
            .abs()
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(
            &surface,
            "irrigation.fixeddate.event_0001.sprinkler_depth_m"
        ) - 0.0120)
            .abs()
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(
            &surface,
            "irrigation.fixeddate.event_0001.sprinkler_nozzle_factor"
        ) - 1.0)
            .abs()
            < 1e-12
    );

    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.event_0002.ofe_id") - 2.0)
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.event_0002.day") - 130.0)
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(
            &surface,
            "irrigation.fixeddate.event_0002.sprinkler_nozzle_factor"
        ) - 1.10)
            .abs()
            < 1e-12
    );
}

#[test]
fn fixeddate_irrigation_runtime_projects_furrow_totals() {
    let fixeddate = parse_fixeddate_str(FIXEDDATE_STRICT_VALID_FURROW, FixedDateParseMode::Strict)
        .expect("strict furrow fixed-date fixture should parse");

    let surface = super::build_hillslope_runtime_surface_from_irrigation_fixeddate(&fixeddate)
        .expect("fixed-date furrow runtime projection should build");

    assert!((fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.enabled") - 1.0).abs() < 1e-12);
    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.system_type") - 2.0).abs()
            < 1e-12
    );
    assert!((fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.event_count") - 1.0).abs() < 1e-12);
    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.event_0001.ofe_id") - 1.0)
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.event_0001.day") - 100.0)
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(&surface, "irrigation.fixeddate.event_0001.furrow_surges") - 2.0)
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(
            &surface,
            "irrigation.fixeddate.event_0001.furrow_total_duration_s"
        ) - 4500.0)
            .abs()
            < 1e-12
    );
    assert!(
        (fixeddate_runtime_scalar(
            &surface,
            "irrigation.fixeddate.event_0001.furrow_total_supply_volume_m3"
        ) - 0.81)
            .abs()
            < 1e-12
    );
}

#[test]
fn fixeddate_irrigation_runtime_rejects_invalid_header_surfaces() {
    let mut fixeddate =
        parse_fixeddate_str(FIXEDDATE_STRICT_VALID_SPRINKLER, FixedDateParseMode::Strict)
            .expect("strict sprinkler fixed-date fixture should parse");
    fixeddate.datver = 0.0;

    let error = super::build_hillslope_runtime_surface_from_irrigation_fixeddate(&fixeddate)
        .expect_err("non-positive datver must fail closed");

    assert!(matches!(
        error,
        super::HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
            field: "irrigation.fixeddate.datver",
            value,
            allowed: "> 0.0",
        } if value == 0.0
    ));
}

#[test]
fn fixeddate_irrigation_runtime_rejects_initial_record_count_mismatch() {
    let mut fixeddate =
        parse_fixeddate_str(FIXEDDATE_STRICT_VALID_SPRINKLER, FixedDateParseMode::Strict)
            .expect("strict sprinkler fixed-date fixture should parse");
    fixeddate.initial_records.pop();

    let error = super::build_hillslope_runtime_surface_from_irrigation_fixeddate(&fixeddate)
        .expect_err("initial record count mismatch must fail closed");

    assert!(matches!(
        error,
        super::HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
            field: "irrigation.fixeddate.initial_records",
            value,
            allowed: "== irrigation.fixeddate.ofe_count",
        } if (value - 1.0).abs() < 1e-12
    ));
}

#[test]
fn fixeddate_irrigation_runtime_rejects_invalid_sprinkler_rate() {
    let mut fixeddate =
        parse_fixeddate_str(FIXEDDATE_STRICT_VALID_SPRINKLER, FixedDateParseMode::Strict)
            .expect("strict sprinkler fixed-date fixture should parse");
    let FixedDateEvent::Sprinkler(event) = &mut fixeddate.events[0] else {
        panic!("fixture event should be sprinkler");
    };
    event.irint = 0.0;

    let error = super::build_hillslope_runtime_surface_from_irrigation_fixeddate(&fixeddate)
        .expect_err("zero sprinkler rate must fail closed");

    assert!(matches!(
        error,
        super::HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
            field: "irrigation.fixeddate.event_####.sprinkler_rate_m_per_s",
            value,
            allowed: "> 0.0",
        } if value == 0.0
    ));
}

#[test]
fn fixeddate_irrigation_runtime_rejects_invalid_furrow_window() {
    let mut fixeddate = parse_fixeddate_str(FIXEDDATE_STRICT_VALID_FURROW, FixedDateParseMode::Strict)
        .expect("strict furrow fixed-date fixture should parse");
    let FixedDateEvent::Furrow(event) = &mut fixeddate.events[0] else {
        panic!("fixture event should be furrow");
    };
    event.rows[0].tend = -1.0;

    let error = super::build_hillslope_runtime_surface_from_irrigation_fixeddate(&fixeddate)
        .expect_err("furrow end before start must fail closed");

    assert!(matches!(
        error,
        super::HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
            field: "irrigation.fixeddate.event_####.furrow_end_s",
            value,
            allowed: ">= irrigation.fixeddate.event_####.furrow_start_s",
        } if (value + 1.0).abs() < 1e-12
    ));
}
