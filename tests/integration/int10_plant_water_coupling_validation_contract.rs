use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopePhaseGraph, HillslopePhaseScheduler, HillslopeWritebackSurface,
    runtime_inputs::{
        build_hillslope_runtime_surface_from_slope, build_hillslope_runtime_surface_from_soil,
    },
};
use openwepp_input_contract::parsers::{
    slope::{SlopeParserOptions, parse_slope_str},
    soil::{SoilParserOptions, parse_soil},
};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeAnnualDecompositionAction, HillslopeAnnualGrowthAction,
    HillslopeKernel, HillslopeKernelRequest, KernelRunResponse, KernelWritebackPayload,
    WritebackField,
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

const INT10_DECOMP_MARKER: &str = "int10_decomp_marker";
const INT10_GROWTH_MARKER: &str = "int10_growth_marker";
const ORDERING_FLAG_EPSILON: f64 = 1.0e-12;

#[derive(Default)]
struct Int10ProbeKernel {
    watbal_marker_checks: usize,
}

impl HillslopeKernel for Int10ProbeKernel {
    #[allow(clippy::too_many_lines)]
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        match request.phase_name {
            "decomposition_transition" | "residue_partition_transition" => {
                let context = request
                    .decomposition_context
                    .expect("decomposition phase must carry decomposition context");
                assert!(request.growth_context.is_none());
                assert_unit_ordering_flag(
                    context.order_decomp_before_soil,
                    "order_decomp_before_soil",
                );
                assert_unit_ordering_flag(
                    context.order_growth_after_decomp,
                    "order_growth_after_decomp",
                );

                let payload = context
                    .transition_payload
                    .expect("decomposition phase must carry transition payload");
                assert_eq!(payload.runtime_day_of_year, 200);
                assert!(matches!(
                    payload.control,
                    openwepp_kernel_contract::HillslopeDecompositionTransitionControl::Annual(
                        openwepp_kernel_contract::HillslopeAnnualDecompositionControl {
                            active_action: HillslopeAnnualDecompositionAction::Herbicide,
                            ..
                        }
                    )
                ));

                KernelRunResponse::new(
                    SimulationStatus::ok(SimulationPhase::HillslopeKernel, "INT10-TEST-DECOMP-OK")
                        .expect("status should construct"),
                    KernelWritebackPayload::with_updates(
                        vec![WritebackField::bounded(
                            INT10_DECOMP_MARKER,
                            10.0,
                            Some(0.0),
                            None,
                        )],
                        Vec::new(),
                    ),
                )
            }
            "annual_growth_transition" => {
                let context = request
                    .growth_context
                    .expect("annual growth phase must carry growth context");
                assert!(request.decomposition_context.is_none());
                assert_unit_ordering_flag(
                    context.order_growth_after_decomp,
                    "order_growth_after_decomp",
                );
                assert_unit_ordering_flag(
                    context.order_watbal_after_growth,
                    "order_watbal_after_growth",
                );
                assert!(
                    request
                        .state_surface
                        .contains_key(&BoundarySymbol::from(INT10_DECOMP_MARKER)),
                    "growth phase must observe decomp state marker"
                );

                let payload = context
                    .transition_payload
                    .expect("annual growth phase must carry transition payload");
                assert!(matches!(
                    payload.control,
                    openwepp_kernel_contract::HillslopeGrowthTransitionControl::Annual(
                        openwepp_kernel_contract::HillslopeAnnualGrowthControl {
                            active_action: HillslopeAnnualGrowthAction::None,
                            ..
                        }
                    )
                ));

                KernelRunResponse::new(
                    SimulationStatus::ok(SimulationPhase::HillslopeKernel, "INT10-TEST-GROWTH-OK")
                        .expect("status should construct"),
                    KernelWritebackPayload::with_updates(
                        vec![WritebackField::bounded(
                            INT10_GROWTH_MARKER,
                            20.0,
                            Some(0.0),
                            None,
                        )],
                        Vec::new(),
                    ),
                )
            }
            "perennial_growth_transition" => {
                assert!(
                    request.growth_context.is_none(),
                    "perennial growth context should be skipped for annual branch"
                );
                KernelRunResponse::new(
                    SimulationStatus::ok(
                        SimulationPhase::HillslopeKernel,
                        "INT10-TEST-PERENNIAL-OK",
                    )
                    .expect("status should construct"),
                    KernelWritebackPayload::empty(),
                )
            }
            "evapotranspiration"
            | "percolation_deep_seepage"
            | "lateral_transfer"
            | "drainage"
            | "plant_root_uptake"
            | "runoff_reconciliation"
            | "storage_reconciliation" => {
                assert!(request.decomposition_context.is_none());
                assert!(request.growth_context.is_none());
                assert!(
                    request
                        .state_surface
                        .contains_key(&BoundarySymbol::from(INT10_DECOMP_MARKER)),
                    "watbal phase must observe decomposition marker"
                );
                assert!(
                    request
                        .state_surface
                        .contains_key(&BoundarySymbol::from(INT10_GROWTH_MARKER)),
                    "watbal phase must observe growth marker"
                );
                self.watbal_marker_checks += 1;

                KernelRunResponse::new(
                    SimulationStatus::ok(SimulationPhase::HillslopeKernel, "INT10-TEST-WATBAL-OK")
                        .expect("status should construct"),
                    KernelWritebackPayload::empty(),
                )
            }
            _ => {
                assert!(request.decomposition_context.is_none());
                assert!(request.growth_context.is_none());
                KernelRunResponse::new(
                    SimulationStatus::ok(SimulationPhase::HillslopeKernel, "INT10-TEST-NOP-OK")
                        .expect("status should construct"),
                    KernelWritebackPayload::empty(),
                )
            }
        }
    }
}

#[derive(Default)]
struct NoopKernel;

impl HillslopeKernel for NoopKernel {
    fn run_hillslope_phase(&mut self, _request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "INT10-TEST-NOOP-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

#[test]
fn int10_contract_conformance_validates_coupled_replay_ordering_and_state_transfer() {
    let topology_report = topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Int10ProbeKernel::default();

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, seeded_int10_surface())
        .expect("int10 coupled replay should produce typed report");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        report.scheduler_report.executed_phases(),
        Vec::from(HillslopePhaseGraph::canonical_order())
    );
    assert_eq!(kernel.watbal_marker_checks, 7);
}

#[test]
fn int10_contract_conformance_rejects_missing_growth_to_watbal_ordering_symbol() {
    let topology_report = topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_int10_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("pl_order_watbal_after_growth"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing ordering symbol should return typed phase report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(report.phase_reports.len(), 5);
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-001"
    );
    assert_eq!(
        report.phase_reports[4].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn int10_contract_conformance_rejects_non_finite_coupled_ordering_value() {
    let topology_report = topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_int10_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("pl_order_growth_after_decomp"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("non-finite ordering value should return typed phase report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-002"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

fn seeded_int10_surface() -> HillslopeWritebackSurface {
    let mut surface = combined_slope_soil_surface();
    let state = &mut surface.state_surface;

    insert_scalar(state, "pl_schedule_slot_count", 1.0);
    insert_scalar(state, "pl_schedule_rotation_years", 1.0);
    insert_scalar(state, "pl_schedule_rotation_repeats", 1.0);
    insert_scalar(state, "day", 200.0);
    insert_scalar(state, "year", 1.0);
    insert_scalar(state, "pl_schedule_slot_0001_rotation_index", 1.0);
    insert_scalar(state, "pl_schedule_slot_0001_ofe_index", 1.0);
    insert_scalar(state, "pl_schedule_slot_0001_year_in_rotation", 1.0);
    insert_scalar(state, "pl_schedule_slot_0001_crop_slots", 1.0);
    insert_scalar(state, "pl_schedule_slot_0001_crop_0001_imngmt", 1.0);

    insert_scalar(state, "pl_order_decomp_before_soil", 1.0);
    insert_scalar(state, "pl_order_growth_after_decomp", 1.0);
    insert_scalar(state, "pl_order_watbal_after_growth", 1.0);

    insert_scalar(state, "pl_growth_slot_0001_crop_0001_imngmt", 1.0);
    insert_scalar(state, "pl_growth_slot_0001_crop_0001_jdharv", 240.0);
    insert_scalar(state, "pl_growth_slot_0001_crop_0001_jdplt", 120.0);
    insert_scalar(state, "pl_growth_slot_0001_crop_0001_rw", 1.3);
    for (root, value) in [
        ("btemp", 10.0),
        ("otemp", 25.0),
        ("gddmax", 1700.0),
        ("dlai", 0.85),
        ("dropfc", 0.98),
        ("decfct", 0.65),
        ("spriod", 30.0),
        ("bb", 3.6),
        ("beinp", 35.00196),
        ("extnct", 0.65),
        ("hi", 0.5),
        ("xmxlai", 3.5),
        ("rsr", 0.25),
        ("rtmmax", 3.0),
        ("rdmax", 1.51995),
    ] {
        insert_scalar(
            state,
            &format!("pl_growth_slot_0001_crop_0001_{root}"),
            value,
        );
    }
    insert_scalar(state, "tmax", 25.0);
    insert_scalar(state, "tmin", 13.0);
    insert_scalar(state, "rad", 210.0);
    insert_scalar(state, "prcp", 0.003);
    insert_scalar(state, "Ws", 0.8);

    insert_scalar(state, "sumgdd", 640.0);
    insert_scalar(state, "vdmt", 2.4);
    insert_scalar(state, "cancov", 0.65);
    insert_scalar(state, "lai", 2.1);
    insert_scalar(state, "rtmass", 1.0);
    insert_scalar(state, "rtd", 0.35);
    insert_scalar(state, "hia", 0.45);

    insert_scalar(state, "iresd_seed", 3.0);
    insert_scalar(state, "sumrtm_seed", 2.5);
    insert_scalar(state, "sumsrm_seed", 1.5);

    insert_scalar(state, "pl_decomp_slot_0001_crop_0001_resmgt", 1.0);
    insert_scalar(state, "pl_decomp_slot_0001_crop_0001_jdherb", 200.0);
    insert_scalar(state, "pl_decomp_slot_0001_crop_0001_jdburn", 0.0);
    insert_scalar(state, "pl_decomp_slot_0001_crop_0001_jdslge", 0.0);
    insert_scalar(state, "pl_decomp_slot_0001_crop_0001_jdcut", 0.0);
    insert_scalar(state, "pl_decomp_slot_0001_crop_0001_jdmove", 0.0);
    insert_scalar(state, "pl_decomp_slot_0001_crop_0001_fbrnag", 0.0);
    insert_scalar(state, "pl_decomp_slot_0001_crop_0001_fbrnog", 0.0);
    insert_scalar(state, "pl_decomp_slot_0001_crop_0001_frcut", 0.0);
    insert_scalar(state, "pl_decomp_slot_0001_crop_0001_frmove", 0.0);
    for (root, value) in [("oratea", 0.0065), ("orater", 0.0065)] {
        insert_scalar(
            state,
            &format!("pl_decomp_slot_0001_crop_0001_{root}"),
            value,
        );
        insert_scalar(state, root, value);
    }

    surface
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

fn insert_scalar(
    state_surface: &mut std::collections::BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    value: f64,
) {
    state_surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

fn assert_unit_ordering_flag(value: f64, symbol: &str) {
    assert!(
        (value - 1.0).abs() <= ORDERING_FLAG_EPSILON,
        "{symbol} expected 1.0 but was {value}"
    );
}

fn topology_report() -> openwepp_topology::TopologyValidationReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology fixture should parse");
    validate_pre_execution_topology(&graph).expect("topology validation should succeed")
}
