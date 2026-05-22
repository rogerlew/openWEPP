use std::collections::BTreeSet;

use openwepp_hillslope_orchestrator::{
    HillslopePhaseGraph, HillslopePhaseScheduler,
    runtime_inputs::build_hillslope_runtime_surface_from_soil,
};
use openwepp_input_contract::parsers::{
    chaninp::{ChaninpParseOptions, parse_chaninp_from_str},
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
    runtime_inputs::build_watershed_runtime_surface_from_chaninp,
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
const CHANINP_STRICT_VALID: &str = include_str!("../fixtures/infile/chaninp/strict_valid.chaninp");

struct HillslopeSeedProbeKernel {
    invocation_count: usize,
}

impl HillslopeKernel for HillslopeSeedProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "solthk", 0.25);
        assert_state_value(request.state_surface, "dg", 0.1);
        assert_state_value(request.state_surface, "thetdr", 0.05);
        assert_state_value(request.state_surface, "thetfc", 0.31);

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
