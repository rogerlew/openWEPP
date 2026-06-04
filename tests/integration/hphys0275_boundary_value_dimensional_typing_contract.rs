use std::collections::BTreeMap;

use openwepp_hillslope_orchestrator::runtime_inputs::{
    build_hillslope_runtime_surface_from_climate,
    build_hillslope_runtime_surface_from_climate_with_context,
};
use openwepp_input_contract::parsers::climate::{
    ParserMode as ClimateParserMode, parse_climate_from_str,
};
use openwepp_kernel_contract::{BoundaryError, BoundarySymbol, BoundaryValue};

const STRICT_VALID_CLIMATE: &str = include_str!("../fixtures/infile/climate/strict_valid.cli");
const BREAKPOINT_STMSTR_CLIMATE: &str = include_str!(
    "../fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli",
);

fn simimpl28_winter_context(rst: f64) -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut context = BTreeMap::new();
    context.insert(
        BoundarySymbol::from("snow.options.snow_file_present"),
        BoundaryValue::scalar(1.0),
    );
    context.insert(
        BoundarySymbol::from("frost.options.frost_file_present"),
        BoundaryValue::scalar(0.0),
    );
    context.insert(
        BoundarySymbol::from("snow.options.rst"),
        BoundaryValue::scalar(rst),
    );
    context.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(0.01),
    );
    context.insert(
        BoundarySymbol::from("frost.runtime_dfrost"),
        BoundaryValue::scalar(0.0),
    );
    context.insert(
        BoundarySymbol::from("frost.runtime_ws_frz"),
        BoundaryValue::scalar(0.0),
    );
    context.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.058));
    context.insert(BoundarySymbol::from("azm"), BoundaryValue::scalar(0.0));
    context
}

fn state_value<'surface>(
    surface: &'surface BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> &'surface BoundaryValue {
    surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing runtime symbol {symbol}"))
}

fn state_count(surface: &BTreeMap<BoundarySymbol, BoundaryValue>, symbol: &str) -> usize {
    let value = state_value(surface, symbol).as_f64();
    assert!(value.is_finite(), "{symbol} count must be finite");
    assert!(value >= 0.0, "{symbol} count must be non-negative");
    let rounded = value.round();
    assert!(
        (value - rounded).abs() <= 1.0e-12,
        "{symbol} count must be integral, got {value}"
    );
    format!("{rounded:.0}")
        .parse::<usize>()
        .unwrap_or_else(|error| panic!("invalid {symbol} count {value}: {error}"))
}

#[test]
fn hphys0275_boundary_constructors_fail_closed_for_invalid_domains() {
    assert!(matches!(
        BoundaryValue::water_depth_meters(-0.001),
        Err(BoundaryError::BelowMinimum {
            boundary: "water_depth_m",
            ..
        })
    ));
    assert!(matches!(
        BoundaryValue::fraction_unit_interval(1.001),
        Err(BoundaryError::AboveMaximum {
            boundary: "fraction_unit_interval",
            ..
        })
    ));
    assert!(matches!(
        BoundaryValue::temperature_celsius(f64::NAN),
        Err(BoundaryError::NonFinite {
            boundary: "temperature_c",
            ..
        })
    ));
}

#[test]
fn hphys0275_daily_climate_surface_publishes_high_risk_symbols_as_typed_values() {
    let climate = parse_climate_from_str(STRICT_VALID_CLIMATE, ClimateParserMode::Strict)
        .expect("strict climate fixture should parse");
    let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("daily climate runtime surface should build");
    let state = &surface.state_surface;

    let typed_symbols = [
        ("prcp", "m"),
        ("stmdur", "s"),
        ("rad", "Ly d^-1"),
        ("tmax", "degC"),
        ("tmin", "degC"),
        ("tdpt", "degC"),
        ("vwind", "m s^-1"),
        ("avrint", "m s^-1"),
        ("mxint", "m s^-1"),
        ("timem_0001", "s"),
        ("intsty_0001", "m s^-1"),
    ];

    for (symbol, expected_unit) in typed_symbols {
        let value = state_value(state, symbol);
        assert_eq!(value.unit_label(), expected_unit, "{symbol} unit label");
        assert_ne!(value.unit_label(), "scalar", "{symbol} must not be scalar");
        assert!(value.as_f64().is_finite(), "{symbol} must remain finite");
    }

    assert_eq!(state_value(state, "wind").unit_label(), "scalar");
    assert!((state_value(state, "wind").as_f64() - 180.0).abs() < 1.0e-12);
    assert!((state_value(state, "prcp").as_f64() - 0.01).abs() < 1.0e-12);
    assert!((state_value(state, "rad").as_f64() - 200.0).abs() < 1.0e-12);
    assert!((state_value(state, "stmdur").as_f64() - 7_200.0).abs() < 1.0e-12);

    let ninten = state_count(state, "ninten");
    for index in 1..=ninten {
        let timem = format!("timem_{index:04}");
        let intsty = format!("intsty_{index:04}");
        assert_eq!(state_value(state, &timem).unit_label(), "s", "{timem}");
        assert_eq!(
            state_value(state, &intsty).unit_label(),
            "m s^-1",
            "{intsty}"
        );
    }

    assert_eq!(state_value(state, "datver").unit_label(), "scalar");
    assert_eq!(state_value(state, "ninten").unit_label(), "scalar");
}

#[test]
fn hphys0275_breakpoint_climate_surface_publishes_selected_symbols_as_typed_values() {
    let climate = parse_climate_from_str(BREAKPOINT_STMSTR_CLIMATE, ClimateParserMode::Strict)
        .expect("breakpoint climate fixture should parse");
    let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("breakpoint climate runtime surface should build");
    let state = &surface.state_surface;

    let typed_symbols = [
        ("stmstr", "h"),
        ("prcp", "m"),
        ("stmdur", "s"),
        ("mxint", "m s^-1"),
        ("tmax", "degC"),
        ("tmin", "degC"),
        ("rad", "Ly d^-1"),
        ("vwind", "m s^-1"),
        ("tdpt", "degC"),
    ];

    for (symbol, expected_unit) in typed_symbols {
        let value = state_value(state, symbol);
        assert_eq!(value.unit_label(), expected_unit, "{symbol} unit label");
        assert_ne!(value.unit_label(), "scalar", "{symbol} must not be scalar");
        assert!(value.as_f64().is_finite(), "{symbol} must remain finite");
    }

    assert_eq!(state_value(state, "wind").unit_label(), "scalar");
    let nbrkpt = state_count(state, "nbrkpt");
    for index in 1..=nbrkpt {
        let timem = format!("timem_{index:04}");
        let intsty = format!("intsty_{index:04}");
        assert_eq!(state_value(state, &timem).unit_label(), "s", "{timem}");
        assert_eq!(
            state_value(state, &intsty).unit_label(),
            "m s^-1",
            "{intsty}"
        );
    }
}

#[test]
fn hphys0275_winter_hourly_surface_publishes_high_risk_symbols_as_typed_values() {
    let climate = parse_climate_from_str(STRICT_VALID_CLIMATE, ClimateParserMode::Strict)
        .expect("strict climate fixture should parse");
    let context = simimpl28_winter_context(0.0);
    let surface = build_hillslope_runtime_surface_from_climate_with_context(&climate, 0, &context)
        .expect("contextual climate runtime surface should build");
    let state = &surface.state_surface;

    for hour in 1..=24 {
        let typed_symbols = [
            (format!("winter.hourly.rad_mj_m2_{hour:04}"), "MJ m^-2 h^-1"),
            (format!("winter.hourly.air_temp_c_{hour:04}"), "degC"),
            (
                format!("winter.hourly.cloud_fraction_{hour:04}"),
                "dimensionless",
            ),
            (format!("snow.hourly.rain_m_{hour:04}"), "m"),
            (format!("snow.hourly.snowfall_m_{hour:04}"), "m"),
        ];

        for (symbol, expected_unit) in typed_symbols {
            let value = state_value(state, &symbol);
            assert_eq!(value.unit_label(), expected_unit, "{symbol} unit label");
            assert_ne!(value.unit_label(), "scalar", "{symbol} must not be scalar");
            assert!(value.as_f64().is_finite(), "{symbol} must remain finite");
        }
    }
}
