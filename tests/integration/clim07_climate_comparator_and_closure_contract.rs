use std::collections::BTreeMap;

use openwepp_climate_runtime_adapter::SharedClimateRuntimeInputError;
use openwepp_comparator_metadata::{
    COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID,
    COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID, COMPMETA_INVESTIGATION_HOURLY_MESSAGE_ID,
    COMPMETA_INVESTIGATION_WATERSHED_MESSAGE_ID, ComparatorConfidenceTier, ComparatorSurfaceClass,
    ComparatorTierRoutingError, ComparatorTierRoutingRequest, route_comparator_tier_metadata,
};
use openwepp_hillslope_orchestrator::runtime_inputs::build_hillslope_runtime_surface_from_climate;
use openwepp_input_contract::parsers::climate::{
    ClimateDailyRecord, ParserMode as ClimateParserMode, parse_climate_from_str,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
use openwepp_watershed_orchestrator::runtime_inputs::{
    WatershedClimateRuntimeInputError, build_watershed_runtime_surface_from_climate_assignments,
};

const STRICT_VALID_CLIMATE: &str = include_str!("../fixtures/infile/climate/strict_valid.cli");
const BREAKPOINT_STMSTR_CLIMATE: &str = include_str!(
    "../fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli",
);
const BREAKPOINT_ZERO_CARDINALITY_CLIMATE: &str =
    include_str!("../fixtures/infile/climate/wc1_unpalatable_rind_breakpoint_nbrkpt_0.cli",);

fn state_value(surface: &BTreeMap<BoundarySymbol, BoundaryValue>, symbol: &str) -> f64 {
    surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing runtime symbol {symbol}"))
        .as_f64()
}

fn symbol_name(prefix: &str, base: &str) -> String {
    if prefix.is_empty() {
        base.to_owned()
    } else {
        format!("{prefix}_{base}")
    }
}

fn series_symbol(prefix: &str, base: &str, index: usize) -> String {
    symbol_name(prefix, &format!("{base}_{index:04}"))
}

fn hyetograph_depth_m(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    prefix: &str,
    count_symbol: &str,
) -> f64 {
    let count_value = state_value(surface, &symbol_name(prefix, count_symbol));
    let count_text = format!("{count_value:.0}");
    let count = count_text
        .parse::<usize>()
        .unwrap_or_else(|error| panic!("invalid series count {count_value}: {error}"));
    if count < 2 {
        return 0.0;
    }

    let mut depth = 0.0;
    for index in 2..=count {
        let previous_time = state_value(surface, &series_symbol(prefix, "timem", index - 1));
        let current_time = state_value(surface, &series_symbol(prefix, "timem", index));
        let intensity = state_value(surface, &series_symbol(prefix, "intsty", index - 1));
        depth += (current_time - previous_time) * intensity;
    }
    depth
}

fn assert_close(actual: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{label} mismatch: expected {expected}, got {actual}, tol {tolerance}"
    );
}

#[test]
fn clim07_continuous_daily_vector_projects_expected_runtime_surface() {
    let climate = parse_climate_from_str(STRICT_VALID_CLIMATE, ClimateParserMode::Strict)
        .expect("strict climate fixture should parse");

    let hillslope_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("continuous-daily hillslope runtime surface should build");

    let hs_state = &hillslope_surface.state_surface;
    assert_close(state_value(hs_state, "datver"), 5.3, 1e-12, "datver");
    assert_close(state_value(hs_state, "iclig"), 1.0, 1e-12, "iclig");
    assert_close(state_value(hs_state, "itemp"), 1.0, 1e-12, "itemp");
    assert_close(state_value(hs_state, "ibrkpt"), 0.0, 1e-12, "ibrkpt");
    assert_close(state_value(hs_state, "iwind"), 0.0, 1e-12, "iwind");
    assert_close(state_value(hs_state, "prcp"), 0.01, 1e-12, "prcp_m");
    assert_close(state_value(hs_state, "stmdur"), 7_200.0, 1e-9, "stmdur_s");
    assert_close(state_value(hs_state, "timep"), 0.25, 1e-12, "timep");
    assert_close(state_value(hs_state, "ip"), 2.1, 1e-12, "ip");
    assert_close(state_value(hs_state, "ninten"), 11.0, 1e-12, "ninten");
    assert_close(
        state_value(hs_state, "timem_0001"),
        0.0,
        1e-12,
        "timem_0001",
    );
    assert_close(
        state_value(hs_state, "timem_0011"),
        7_200.0,
        1e-9,
        "timem_0011",
    );
    assert_close(
        state_value(hs_state, "intsty_0011"),
        0.0,
        1e-12,
        "intsty_0011",
    );

    let hillslope_depth_m = hyetograph_depth_m(hs_state, "", "ninten");
    assert_close(
        hillslope_depth_m,
        0.01,
        1e-9,
        "continuous hyetograph closure",
    );

    let assignments = BTreeMap::from([(1_u32, climate)]);
    let watershed_surface =
        build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("continuous-daily watershed runtime surface should build");

    let ws_state = &watershed_surface.state_surface;
    assert_close(state_value(ws_state, "nclimhs"), 1.0, 1e-12, "nclimhs");
    assert_close(
        state_value(ws_state, "hs1_datver"),
        5.3,
        1e-12,
        "hs1_datver",
    );
    assert_close(
        state_value(ws_state, "hs1_ibrkpt"),
        0.0,
        1e-12,
        "hs1_ibrkpt",
    );
    assert_close(state_value(ws_state, "hs1_prcp"), 0.01, 1e-12, "hs1_prcp");
    assert_close(
        state_value(ws_state, "hs1_ninten"),
        11.0,
        1e-12,
        "hs1_ninten",
    );
    assert_close(
        state_value(ws_state, "hs1_timem_0011"),
        7_200.0,
        1e-9,
        "hs1_timem_0011",
    );

    let watershed_depth_m = hyetograph_depth_m(ws_state, "hs1", "ninten");
    assert_close(
        watershed_depth_m,
        0.01,
        1e-9,
        "continuous watershed closure",
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn clim07_breakpoint_vector_projects_expected_runtime_surface() {
    let climate = parse_climate_from_str(BREAKPOINT_STMSTR_CLIMATE, ClimateParserMode::Strict)
        .expect("breakpoint climate fixture should parse");

    let hillslope_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("breakpoint hillslope runtime surface should build");
    let hs_state = &hillslope_surface.state_surface;

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

    assert_close(state_value(hs_state, "ibrkpt"), 1.0, 1e-12, "ibrkpt");
    assert_close(state_value(hs_state, "nbrkpt"), 5.0, 1e-12, "nbrkpt");
    assert_close(state_value(hs_state, "stmstr"), 4.8667, 1e-12, "stmstr_h");
    assert_close(state_value(hs_state, "prcp"), 0.00735, 1e-12, "prcp_m");
    assert_close(
        state_value(hs_state, "stmdur"),
        expected_stmdur,
        1e-6,
        "stmdur_s",
    );
    assert_close(
        state_value(hs_state, "mxint"),
        expected_mxint,
        1e-12,
        "mxint_m_per_s",
    );
    assert_close(
        state_value(hs_state, "timem_0001"),
        0.0,
        1e-12,
        "timem_0001",
    );
    assert_close(
        state_value(hs_state, "timem_0002"),
        expected_timem_2,
        1e-6,
        "timem_0002",
    );
    assert_close(
        state_value(hs_state, "intsty_0005"),
        0.0,
        1e-12,
        "intsty_0005",
    );

    let hillslope_depth_m = hyetograph_depth_m(hs_state, "", "nbrkpt");
    assert_close(
        hillslope_depth_m,
        0.00735,
        1e-12,
        "breakpoint hyetograph closure",
    );

    let assignments = BTreeMap::from([(21_u32, climate)]);
    let watershed_surface =
        build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("breakpoint watershed runtime surface should build");
    let ws_state = &watershed_surface.state_surface;

    assert_close(
        state_value(ws_state, "hs21_ibrkpt"),
        1.0,
        1e-12,
        "hs21_ibrkpt",
    );
    assert_close(
        state_value(ws_state, "hs21_nbrkpt"),
        5.0,
        1e-12,
        "hs21_nbrkpt",
    );
    assert_close(
        state_value(ws_state, "hs21_stmstr"),
        4.8667,
        1e-12,
        "hs21_stmstr_h",
    );
    assert_close(
        state_value(ws_state, "hs21_prcp"),
        0.00735,
        1e-12,
        "hs21_prcp_m",
    );
    assert_close(
        state_value(ws_state, "hs21_stmdur"),
        expected_stmdur,
        1e-6,
        "hs21_stmdur_s",
    );
    assert_close(
        state_value(ws_state, "hs21_mxint"),
        expected_mxint,
        1e-12,
        "hs21_mxint",
    );

    let watershed_depth_m = hyetograph_depth_m(ws_state, "hs21", "nbrkpt");
    assert_close(
        watershed_depth_m,
        0.00735,
        1e-12,
        "breakpoint watershed closure",
    );
}

#[test]
fn clim07_breakpoint_zero_cardinality_vector_projects_dry_day_surface() {
    let climate = parse_climate_from_str(
        BREAKPOINT_ZERO_CARDINALITY_CLIMATE,
        ClimateParserMode::Strict,
    )
    .expect("zero-cardinality breakpoint fixture should parse");

    let hillslope_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("zero-cardinality breakpoint hillslope runtime surface should build");
    let hs_state = &hillslope_surface.state_surface;

    assert_close(state_value(hs_state, "ibrkpt"), 1.0, 1e-12, "ibrkpt");
    assert_close(state_value(hs_state, "nbrkpt"), 0.0, 1e-12, "nbrkpt");
    assert_close(state_value(hs_state, "stmstr"), 0.0, 1e-12, "stmstr_h");
    assert_close(state_value(hs_state, "prcp"), 0.0, 1e-12, "prcp_m");
    assert_close(state_value(hs_state, "stmdur"), 0.0, 1e-12, "stmdur_s");
    assert_close(state_value(hs_state, "mxint"), 0.0, 1e-12, "mxint_m_per_s");
    assert!(
        !hs_state.contains_key(&BoundarySymbol::from("timem_0001")),
        "zero-cardinality breakpoint dry day must not publish timem_0001"
    );
    assert!(
        !hs_state.contains_key(&BoundarySymbol::from("intsty_0001")),
        "zero-cardinality breakpoint dry day must not publish intsty_0001"
    );

    let hillslope_depth_m = hyetograph_depth_m(hs_state, "", "nbrkpt");
    assert_close(hillslope_depth_m, 0.0, 1e-12, "breakpoint dry-day closure");

    let assignments = BTreeMap::from([(24_u32, climate)]);
    let watershed_surface =
        build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("zero-cardinality breakpoint watershed runtime surface should build");
    let ws_state = &watershed_surface.state_surface;

    assert_close(
        state_value(ws_state, "hs24_ibrkpt"),
        1.0,
        1e-12,
        "hs24_ibrkpt",
    );
    assert_close(
        state_value(ws_state, "hs24_nbrkpt"),
        0.0,
        1e-12,
        "hs24_nbrkpt",
    );
    assert_close(
        state_value(ws_state, "hs24_stmstr"),
        0.0,
        1e-12,
        "hs24_stmstr_h",
    );
    assert_close(
        state_value(ws_state, "hs24_prcp"),
        0.0,
        1e-12,
        "hs24_prcp_m",
    );
    assert_close(
        state_value(ws_state, "hs24_stmdur"),
        0.0,
        1e-12,
        "hs24_stmdur_s",
    );
    assert_close(
        state_value(ws_state, "hs24_mxint"),
        0.0,
        1e-12,
        "hs24_mxint",
    );
    assert!(
        !ws_state.contains_key(&BoundarySymbol::from("hs24_timem_0001")),
        "zero-cardinality breakpoint dry day must not publish hs24_timem_0001"
    );
    assert!(
        !ws_state.contains_key(&BoundarySymbol::from("hs24_intsty_0001")),
        "zero-cardinality breakpoint dry day must not publish hs24_intsty_0001"
    );

    let watershed_depth_m = hyetograph_depth_m(ws_state, "hs24", "nbrkpt");
    assert_close(
        watershed_depth_m,
        0.0,
        1e-12,
        "breakpoint dry-day watershed closure",
    );
}

#[test]
fn clim07_breakpoint_domain_violation_remains_typed_hard_fail() {
    let mut climate = parse_climate_from_str(BREAKPOINT_STMSTR_CLIMATE, ClimateParserMode::Strict)
        .expect("breakpoint climate fixture should parse");

    let day = climate
        .daily_records
        .first_mut()
        .expect("fixture should include one day");
    match day {
        ClimateDailyRecord::Breakpoint(record) => {
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
        ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint climate fixture"),
    }

    let hillslope_error = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect_err("duplicate breakpoint time should fail hillslope seam");
    assert_eq!(hillslope_error.code(), "CLIM-RUNTIME-E-009");
    assert!(matches!(
        hillslope_error,
        SharedClimateRuntimeInputError::NonMonotoneBreakpointTime { .. }
    ));

    let assignments = BTreeMap::from([(2_u32, climate)]);
    let watershed_error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect_err("duplicate breakpoint time should fail watershed seam");
    assert_eq!(watershed_error.code(), "CLIM-RUNTIME-E-009");
    assert!(matches!(
        watershed_error,
        WatershedClimateRuntimeInputError::NonMonotoneBreakpointTime {
            hillslope_id: 2,
            ..
        }
    ));
}

#[test]
fn clim07_confidence_tier_routing_vectors_match_governance_policy() {
    let daily = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
        Some(1),
    ))
    .expect("single OFE daily should route");
    assert_eq!(
        daily.confidence_tier,
        ComparatorConfidenceTier::HigherConfidence
    );
    assert_eq!(
        daily.message_id,
        COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID
    );

    let hourly = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::HourlyWaterBalance,
        Some(2),
    ))
    .expect("hourly routing should succeed");
    assert_eq!(
        hourly.confidence_tier,
        ComparatorConfidenceTier::Investigation
    );
    assert_eq!(hourly.message_id, COMPMETA_INVESTIGATION_HOURLY_MESSAGE_ID);

    let watershed = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::WatershedWaterBalance,
        None,
    ))
    .expect("watershed routing should succeed");
    assert_eq!(
        watershed.confidence_tier,
        ComparatorConfidenceTier::Investigation
    );
    assert_eq!(
        watershed.message_id,
        COMPMETA_INVESTIGATION_WATERSHED_MESSAGE_ID
    );

    let missing = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
        None,
    ))
    .expect_err("missing OFE count must fail");
    assert!(matches!(
        missing,
        ComparatorTierRoutingError::MissingRequiredMetadata {
            field: "contributor_ofe_count",
            message_id: COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID,
        }
    ));
}
