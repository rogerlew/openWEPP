use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopePhaseGraph, HillslopePhaseScheduler, HillslopeWritebackSurface,
    hillslope_consumer_adapter_for_phase, required_hillslope_consumer_state_symbols,
    runtime_inputs::{
        build_hillslope_runtime_surface_from_slope, build_hillslope_runtime_surface_from_soil,
    },
};
use openwepp_input_contract::parsers::{
    slope::{SlopeParserOptions, parse_slope_str},
    soil::{SoilParserOptions, parse_soil},
};
use openwepp_kernel_contract::{
    BoundarySymbol, HillslopeConsumerAdapter, HillslopeKernel, HillslopeKernelRequest,
    KernelRunResponse, KernelWritebackPayload,
};
use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, SimulationStatus};
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};

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

#[derive(Default)]
struct BoundaryProbeKernel {
    invocation_count: usize,
}

impl HillslopeKernel for BoundaryProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        self.invocation_count += 1;

        let phase = phase_from_name(request.phase_name);
        let expected_adapter = hillslope_consumer_adapter_for_phase(phase);
        assert_eq!(request.consumer_adapter, expected_adapter);

        let required_symbols =
            required_hillslope_consumer_state_symbols(phase, request.state_surface);
        assert!(
            !required_symbols.is_empty(),
            "combined slope+soil runtime should produce required symbols for phase {}",
            request.phase_name
        );
        for symbol in required_symbols {
            assert!(
                request
                    .state_surface
                    .contains_key(&BoundarySymbol::from(symbol)),
                "missing required consumer symbol {} for phase {} ({})",
                symbol,
                request.phase_name,
                request.consumer_adapter.as_str()
            );
        }

        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "SR06-CONSUMER-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

#[derive(Default)]
struct NoopKernel {
    invocation_count: usize,
}

impl HillslopeKernel for NoopKernel {
    fn run_hillslope_phase(&mut self, _request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "SR06-NOOP-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

#[test]
fn consumer_adapter_boundaries_receive_runtime_seam_symbols() {
    let runtime_surface = combined_slope_soil_surface();
    let topology_report = topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = BoundaryProbeKernel::default();

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .expect("combined runtime surface should satisfy consumer boundary wiring");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        report.scheduler_report.executed_phases(),
        Vec::from(HillslopePhaseGraph::canonical_order())
    );
    assert_eq!(
        kernel.invocation_count,
        HillslopePhaseGraph::canonical_order().len()
    );
}

#[test]
fn missing_soil_consumer_symbol_fails_with_typed_missing_input_status() {
    let mut runtime_surface = combined_slope_soil_surface();
    runtime_surface
        .state_surface
        .remove(&BoundarySymbol::from("thetdr"));

    let topology_report = topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .expect("missing required consumer symbol should yield typed phase failure report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::Normalization)
    );
    assert_eq!(kernel.invocation_count, 0);
    assert_eq!(report.phase_reports.len(), 1);
    assert_eq!(
        report.phase_reports[0].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
    assert_eq!(
        report.phase_reports[0].decision_status.message_id(),
        "HS-CONSUMER-E-001"
    );
}

#[test]
fn missing_runoff_slope_symbol_fails_at_runoff_reconciliation_boundary() {
    let mut runtime_surface = combined_slope_soil_surface();
    runtime_surface
        .state_surface
        .remove(&BoundarySymbol::from("avgslp"));

    let topology_report = topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .expect("missing runoff slope symbol should produce typed phase failure report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );
    assert_eq!(kernel.invocation_count, 6);
    assert_eq!(report.phase_reports.len(), 7);
    assert_eq!(
        report.phase_reports[6].phase,
        HillslopePhase::RunoffReconciliation
    );
    assert_eq!(
        report.phase_reports[6].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
    assert_eq!(
        report.phase_reports[6].decision_status.message_id(),
        "HS-CONSUMER-E-001"
    );
}

fn combined_slope_soil_surface() -> HillslopeWritebackSurface {
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    let slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
        .expect("slope fixture should parse");

    let soil_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("soil runtime surface should build");
    let slope_surface = build_hillslope_runtime_surface_from_slope(&slope)
        .expect("slope runtime surface should build");
    merge_hillslope_runtime_surfaces(soil_surface, slope_surface)
}

fn merge_hillslope_runtime_surfaces(
    mut primary: HillslopeWritebackSurface,
    overlay: HillslopeWritebackSurface,
) -> HillslopeWritebackSurface {
    primary.state_surface.extend(overlay.state_surface);
    primary.flux_surface.extend(overlay.flux_surface);
    primary
}

fn topology_report() -> openwepp_topology::TopologyValidationReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology fixture should parse");
    validate_pre_execution_topology(&graph).expect("topology validation should succeed")
}

fn phase_from_name(phase_name: &str) -> HillslopePhase {
    match phase_name {
        "normalization" => HillslopePhase::Normalization,
        "storage_bounds" => HillslopePhase::StorageBounds,
        "evapotranspiration" => HillslopePhase::Evapotranspiration,
        "percolation_deep_seepage" => HillslopePhase::PercolationDeepSeepage,
        "lateral_transfer" => HillslopePhase::LateralTransfer,
        "drainage" => HillslopePhase::Drainage,
        "runoff_reconciliation" => HillslopePhase::RunoffReconciliation,
        "storage_reconciliation" => HillslopePhase::StorageReconciliation,
        "closure_diagnostics" => HillslopePhase::ClosureDiagnostics,
        other => panic!("unknown hillslope phase name {other}"),
    }
}

#[test]
fn phase_to_consumer_adapter_contract_remains_stable() {
    for phase in HillslopePhaseGraph::canonical_order() {
        let adapter = hillslope_consumer_adapter_for_phase(phase);
        assert!(matches!(
            adapter,
            HillslopeConsumerAdapter::Runoff
                | HillslopeConsumerAdapter::Soil
                | HillslopeConsumerAdapter::Watbal
                | HillslopeConsumerAdapter::Perc
        ));
    }
}
