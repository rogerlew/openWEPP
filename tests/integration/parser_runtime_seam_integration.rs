use std::collections::{BTreeMap, BTreeSet};

use openwepp_hillslope_orchestrator::{
    HillslopePhaseGraph, HillslopePhaseScheduler,
    runtime_inputs::{
        HillslopeRuntimeInputError, build_hillslope_runtime_surface_from_climate,
        build_hillslope_runtime_surface_from_slope, build_hillslope_runtime_surface_from_soil,
    },
};
use openwepp_input_contract::parsers::{
    chaninp::{ChaninpParseOptions, parse_chaninp_from_str},
    climate::{ParserMode as ClimateParserMode, parse_climate_from_str},
    slope::{SlopeParserOptions, parse_slope_str},
    soil::{SoilParserOptions, parse_soil},
};
use openwepp_kernel_contract::{
    BoundarySymbol, HillslopeKernel, HillslopeKernelRequest, KernelRunResponse,
    KernelWritebackPayload, WatershedKernel, WatershedKernelRequest,
};
use openwepp_sim_contract::status::{SimulationPhase, SimulationStatus};
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};
use openwepp_watershed_orchestrator::{
    execute_watershed_dispatch_with_kernel,
    runtime_inputs::{
        build_watershed_runtime_surface_from_chaninp,
        build_watershed_runtime_surface_from_climate_assignments,
    },
};

const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

const SOIL_VALID_9002: &str = include_str!("../fixtures/infile/soil/valid_9002.sol");
const SLOPE_STRICT_VALID_CANONICAL: &str =
    include_str!("../fixtures/infile/slope/strict_valid_canonical.slp");
const CHANINP_STRICT_VALID: &str = include_str!("../fixtures/infile/chaninp/strict_valid.chaninp");
const CLIMATE_STRICT_VALID: &str = include_str!("../fixtures/infile/climate/strict_valid.cli");
const CLIMATE_WC1_DAY1: &str = include_str!("../fixtures/infile/climate/wc1_canoga_day1.cli");
const CLIMATE_WC1_STMDUR_CAP: &str =
    include_str!("../fixtures/infile/climate/wc1_canoga_stmdur_cap.cli");

struct HillslopeSeedProbeKernel {
    invocation_count: usize,
}

impl HillslopeKernel for HillslopeSeedProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "solthk", 0.25);
        assert_state_value(request.state_surface, "dg", 0.1);
        assert_state_value(request.state_surface, "thetdr", 0.05);
        assert_state_value(request.state_surface, "thetfc", 0.31);
        assert_state_value(request.state_surface, "nsl", 2.0);
        assert_state_value(request.state_surface, "ssc", 15.0 / 3.6e6);
        assert_state_value(request.state_surface, "dg_0002", 0.15);
        assert_state_value(request.state_surface, "solthk_0002", 0.25);
        assert_state_value(request.state_surface, "ssc_0002", 8.0 / 3.6e6);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "ARCH17-HS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

struct WatershedSeedProbeKernel {
    invocation_count: usize,
}

impl WatershedKernel for WatershedSeedProbeKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "ipeak", 3.0);
        assert_state_value(request.state_surface, "nchan", 2.0);
        assert_state_value(request.state_surface, "dtchr", 600.0);
        assert_state_value(request.state_surface, "ntchr", 144.0);
        assert_state_value(request.state_surface, "nchnum", 2.0);
        assert_state_value(request.flux_surface, "cbase", 0.000_001);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::WatershedKernel, "ARCH17-WS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

struct HillslopeClimateProbeKernel {
    invocation_count: usize,
}

struct HillslopeSlopeProbeKernel {
    invocation_count: usize,
}

impl HillslopeKernel for HillslopeSlopeProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "nelem", 2.0);
        assert_state_value(request.state_surface, "nwsofe", 2.0);
        assert_state_value(request.state_surface, "nslpts", 3.0);
        assert_state_value(request.state_surface, "slplen", 60.0);
        assert_state_value(request.state_surface, "avgslp", 0.058);
        assert_state_value(request.state_surface, "xinput_0001", 0.0);
        assert_state_value(request.state_surface, "xinput_0002", 0.6);
        assert_state_value(request.state_surface, "slpinp_0002", 0.08);
        assert_state_value(request.state_surface, "ofe2_nslpts", 3.0);
        assert_state_value(request.state_surface, "ofe2_slplen", 40.0);
        assert_state_value(request.state_surface, "ofe2_avgslp", 0.0425);
        assert_state_value(request.state_surface, "ofe2_xinput_0003", 1.0);
        assert_state_value(request.state_surface, "ofe2_slpinp_0003", 0.03);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "ARCH17-HS-SLOPE-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

impl HillslopeKernel for HillslopeClimateProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "datver", 5.3);
        assert_state_value(request.state_surface, "iclig", 1.0);
        assert_state_value(request.state_surface, "itemp", 1.0);
        assert_state_value(request.state_surface, "ibrkpt", 0.0);
        assert_state_value(request.state_surface, "iwind", 0.0);
        assert_state_value(request.state_surface, "prcp", 0.01);
        assert_state_value(request.state_surface, "stmdur", 7_200.0);
        assert_state_value(request.state_surface, "timep", 0.25);
        assert_state_value(request.state_surface, "ip", 2.1);
        assert_state_at_least(request.state_surface, "ninten", 2.0);
        assert_state_value(request.state_surface, "timem_0001", 0.0);
        assert_state_value(request.state_surface, "tmax", 12.0);
        assert_state_value(request.state_surface, "tmin", 2.0);
        assert_state_value(request.state_surface, "rad", 200.0);
        assert_state_value(request.state_surface, "tdpt", -1.0);
        assert_state_value(request.state_surface, "vwind", 3.0);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "CLIM02-HS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

struct WatershedClimateProbeKernel {
    invocation_count: usize,
}

impl WatershedKernel for WatershedClimateProbeKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "nclimhs", 3.0);
        assert_state_value(request.state_surface, "hs1_datver", 5.3);
        assert_state_value(request.state_surface, "hs2_datver", 5.3);
        assert_state_value(request.state_surface, "hs3_datver", 5.3);
        assert_state_value(request.state_surface, "hs1_prcp", 0.01);
        assert_state_value(request.state_surface, "hs2_stmdur", 7_200.0);
        assert_state_value(request.state_surface, "hs3_timep", 0.25);
        assert_state_value(request.state_surface, "hs1_ip", 2.1);
        assert_state_at_least(request.state_surface, "hs2_ninten", 2.0);
        assert_state_value(request.state_surface, "hs3_timem_0001", 0.0);
        assert_state_value(request.state_surface, "hs2_tmax", 12.0);
        assert_state_value(request.state_surface, "hs3_tmin", 2.0);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::WatershedKernel, "CLIM02-WS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

#[test]
fn parser_to_hillslope_runtime_surface_closure() {
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse for seam closure");
    let runtime_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("runtime surface should build from soil parser output");

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = HillslopeSeedProbeKernel {
        invocation_count: 0,
    };
    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .expect("hillslope execution should succeed");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.invocation_count,
        HillslopePhaseGraph::canonical_order().len()
    );
}

#[test]
fn soil_runtime_surface_rejects_missing_saturated_conductivity_projection() {
    let mut soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    soil.ofes[0].layers[0].ksat_mm_h = None;

    let error = build_hillslope_runtime_surface_from_soil(&soil)
        .expect_err("missing ksat must fail with typed seam guard");
    assert_eq!(error.code(), "HS-RUNTIME-E-033");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::MissingSaturatedConductivity {
            ofe_index: 1,
            layer_index: 1
        }
    ));
}

#[test]
fn slope_parser_to_hillslope_runtime_surface_closure() {
    let slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
        .expect("slope fixture should parse for seam closure");
    let runtime_surface = build_hillslope_runtime_surface_from_slope(&slope)
        .expect("runtime surface should build from slope parser output");

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = HillslopeSlopeProbeKernel {
        invocation_count: 0,
    };
    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .expect("hillslope execution should consume slope runtime symbols");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.invocation_count,
        HillslopePhaseGraph::canonical_order().len()
    );
}

#[test]
fn slope_runtime_surface_rejects_non_positive_avgslp_projection() {
    let mut slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
        .expect("slope fixture should parse");
    for point in &mut slope.ofes[0].points {
        point.slpinp = 0.0;
    }

    let error = build_hillslope_runtime_surface_from_slope(&slope)
        .expect_err("non-positive avgslp projection must fail with typed guard");
    assert_eq!(error.code(), "HS-RUNTIME-E-023");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::NonPositiveDerivedAverageSlope {
            ofe_index: 1,
            value
        } if value.abs() < 1e-12
    ));
}

#[test]
fn parser_to_watershed_runtime_surface_closure() {
    let valid_channel_ids = BTreeSet::from([4, 5]);
    let chaninp = parse_chaninp_from_str(
        CHANINP_STRICT_VALID,
        ChaninpParseOptions::strict(3, 2),
        &valid_channel_ids,
    )
    .expect("chan.inp fixture should parse for seam closure");
    let runtime_surface = build_watershed_runtime_surface_from_chaninp(&chaninp)
        .expect("runtime surface should build from chan.inp parser output");

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let mut kernel = WatershedSeedProbeKernel {
        invocation_count: 0,
    };
    let report = execute_watershed_dispatch_with_kernel(
        &graph,
        &topology_report,
        &mut kernel,
        runtime_surface,
    )
    .expect("watershed execution should succeed");

    assert!(report.dispatch_report.is_success());
    assert_eq!(kernel.invocation_count, report.dispatch_report.steps.len());
}

#[test]
fn climate_parser_to_hillslope_runtime_surface_closure() {
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse for hillslope seam closure");
    let runtime_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("hillslope climate runtime surface should build from parser output");

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = HillslopeClimateProbeKernel {
        invocation_count: 0,
    };
    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .expect("hillslope execution should consume climate runtime symbols");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.invocation_count,
        HillslopePhaseGraph::canonical_order().len()
    );
}

#[test]
fn climate_parser_to_watershed_runtime_surface_closure() {
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse for watershed seam closure");
    let assignments = BTreeMap::from([
        (1_u32, climate.clone()),
        (2_u32, climate.clone()),
        (3_u32, climate),
    ]);
    let runtime_surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("watershed climate runtime surface should build from parser outputs");

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let mut kernel = WatershedClimateProbeKernel {
        invocation_count: 0,
    };
    let report = execute_watershed_dispatch_with_kernel(
        &graph,
        &topology_report,
        &mut kernel,
        runtime_surface,
    )
    .expect("watershed execution should consume climate assignment runtime symbols");

    assert!(report.dispatch_report.is_success());
    assert_eq!(kernel.invocation_count, report.dispatch_report.steps.len());
}

#[test]
fn climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path() {
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse for parity check");
    let hillslope_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("hillslope climate runtime surface should build");

    let assignments = BTreeMap::from([(7_u32, climate)]);
    let watershed_surface =
        build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("watershed climate runtime surface should build");
    assert_state_value(&watershed_surface.state_surface, "nclimhs", 1.0);

    for (symbol, value) in &hillslope_surface.state_surface {
        let watershed_symbol = format!("hs7_{}", symbol.as_str());
        let watershed_value = watershed_surface
            .state_surface
            .get(&BoundarySymbol::from(watershed_symbol.as_str()))
            .unwrap_or_else(|| panic!("missing watershed parity symbol {watershed_symbol}"))
            .as_f64();
        let hillslope_value = (*value).as_f64();

        assert!(
            (hillslope_value - watershed_value).abs() < 1e-12,
            "parity mismatch for {} / {}: hillslope={} watershed={}",
            symbol.as_str(),
            watershed_symbol,
            hillslope_value,
            watershed_value
        );
    }
}

#[test]
fn climate_wc1_fixture_applies_timep_floor_and_ip_policy_scaling() {
    let climate = parse_climate_from_str(CLIMATE_WC1_DAY1, ClimateParserMode::Strict)
        .expect("wc1 climate fixture should parse");
    let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("hillslope runtime surface should build from wc1 fixture");
    assert_state_value(&surface.state_surface, "timep", 0.01);
    assert_state_value(&surface.state_surface, "ip", 2.94);
}

#[test]
fn climate_wc1_fixture_caps_storm_duration_before_runtime_projection() {
    let climate = parse_climate_from_str(CLIMATE_WC1_STMDUR_CAP, ClimateParserMode::Strict)
        .expect("wc1 duration-cap fixture should parse");
    let assignments = BTreeMap::from([(9_u32, climate)]);
    let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("watershed runtime surface should build from wc1 duration-cap fixture");
    assert_state_value(&surface.state_surface, "hs9_stmdur", 23.999 * 3_600.0);
    assert_state_value(&surface.state_surface, "hs9_ip", 22.589);
}

fn assert_state_value(
    surface: &std::collections::BTreeMap<BoundarySymbol, openwepp_kernel_contract::BoundaryValue>,
    symbol: &str,
    expected: f64,
) {
    let value = surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing runtime symbol {symbol}"))
        .as_f64();
    assert!(
        (value - expected).abs() < 1e-12,
        "{symbol} mismatch: {value}"
    );
}

fn assert_state_at_least(
    surface: &std::collections::BTreeMap<BoundarySymbol, openwepp_kernel_contract::BoundaryValue>,
    symbol: &str,
    minimum: f64,
) {
    let value = surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing runtime symbol {symbol}"))
        .as_f64();
    assert!(
        value >= minimum,
        "{symbol} expected >= {minimum}, got {value}"
    );
}
