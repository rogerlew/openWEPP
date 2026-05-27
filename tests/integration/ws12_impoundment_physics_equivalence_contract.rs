use openwepp_input_contract::parsers::{
    chaninp::{ChaninpParseOptions, parse_chaninp_from_str},
    watershed_channel::{WatershedChannelParseOptions, parse_watershed_channel_from_str},
    watershed_impoundment::{
        WatershedImpoundmentParseOptions, parse_watershed_impoundment_from_str,
    },
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
use openwepp_sim_contract::status::BoundaryClass;
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};
use openwepp_watershed_orchestrator::{
    WatershedWritebackSurface, Ws10ChannelImpoundmentKernel,
    execute_watershed_dispatch_with_kernel,
    runtime_inputs::{
        build_watershed_runtime_surface_from_chaninp,
        seed_watershed_runtime_surface_from_watershed_channel,
        seed_watershed_runtime_surface_from_watershed_impoundment,
    },
};

const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 2 0 0 C 1 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 0 0 0 I 1 0 0
";

const STRICT_VALID_CHANINP: &str = include_str!("../fixtures/infile/chaninp/strict_valid.chaninp");
const STRICT_VALID_WATERSHED_CHANNEL: &str =
    include_str!("../fixtures/infile/watershed_channel/strict_valid_single_channel.chn");
const STRICT_VALID_WATERSHED_IMPOUNDMENT: &str =
    include_str!("../fixtures/infile/watershed_impoundment/strict_valid_minimal.imp");
const STRICT_VALID_WATERSHED_IMPOUNDMENT_ACTIVE: &str =
    include_str!("../fixtures/infile/watershed_impoundment/strict_valid_active_payloads.imp");
const EROD15_CLASS_COUNT_SCALAR: f64 = 3.0;

fn class_index_scalar(class: usize) -> f64 {
    f64::from(u32::try_from(class).expect("class index should fit within u32 range"))
}

fn seed_erod15_hillslope_payload(
    surface: &mut WatershedWritebackSurface,
    hillslope_id: u32,
    peakro: f64,
    watdur: f64,
) {
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_peakro")),
        BoundaryValue::scalar(peakro),
    );
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_watdur")),
        BoundaryValue::scalar(watdur),
    );
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_total_detachment_kg")),
        BoundaryValue::scalar((peakro * watdur * 0.01).max(0.0)),
    );
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_total_deposition_kg")),
        BoundaryValue::scalar((peakro * watdur * 0.0025).max(0.0)),
    );
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_particle_class_count")),
        BoundaryValue::scalar(EROD15_CLASS_COUNT_SCALAR),
    );

    for (class_index, fraction) in [0.2, 0.3, 0.5].into_iter().enumerate() {
        let class = class_index + 1;
        surface.state_surface.insert(
            BoundarySymbol::from(format!(
                "hs{hillslope_id}_sediment_concentration_kg_m3_{class:04}"
            )),
            BoundaryValue::scalar(0.25 + (class_index_scalar(class) * 0.1)),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!(
                "hs{hillslope_id}_particle_flow_fraction_{class:04}"
            )),
            BoundaryValue::scalar(fraction),
        );
    }
}

fn seeded_ws12_surface_with_impoundment_fixture(
    impoundment_fixture: &str,
) -> WatershedWritebackSurface {
    let valid_channel_element_ids = std::collections::BTreeSet::from([4, 5]);
    let chaninp = parse_chaninp_from_str(
        STRICT_VALID_CHANINP,
        ChaninpParseOptions::strict(3, 2),
        &valid_channel_element_ids,
    )
    .expect("strict chan.inp fixture should parse");
    let mut runtime_surface = build_watershed_runtime_surface_from_chaninp(&chaninp)
        .expect("chan.inp runtime surface should build");

    let watershed_channel = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    seed_watershed_runtime_surface_from_watershed_channel(&mut runtime_surface, &watershed_channel)
        .expect("watershed channel runtime seed should project ws10 symbols");

    // Duplicate channel-1 control symbols for channel:2 so the topology fixture
    // has per-channel controls on both channel nodes.
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chnn"),
        BoundaryValue::scalar(0.05),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_ctlslp"),
        BoundaryValue::scalar(0.03),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chnk"),
        BoundaryValue::scalar(0.000_002),
    );

    let impoundment = parse_watershed_impoundment_from_str(
        impoundment_fixture,
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect("strict watershed impoundment fixture should parse");
    seed_watershed_runtime_surface_from_watershed_impoundment(&mut runtime_surface, &impoundment)
        .expect("watershed impoundment runtime seed should project ws10 symbols");

    seed_erod15_hillslope_payload(&mut runtime_surface, 1, 2.0, 300.0);
    seed_erod15_hillslope_payload(&mut runtime_surface, 2, 1.5, 400.0);
    seed_erod15_hillslope_payload(&mut runtime_surface, 3, 0.5, 200.0);
    runtime_surface
}

fn seeded_ws12_surface() -> WatershedWritebackSurface {
    seeded_ws12_surface_with_impoundment_fixture(STRICT_VALID_WATERSHED_IMPOUNDMENT)
}

fn seeded_ws12_active_surface() -> WatershedWritebackSurface {
    seeded_ws12_surface_with_impoundment_fixture(STRICT_VALID_WATERSHED_IMPOUNDMENT_ACTIVE)
}

fn run_ws12_surface(
    surface: WatershedWritebackSurface,
) -> openwepp_watershed_orchestrator::WatershedKernelExecutionReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let mut kernel = Ws10ChannelImpoundmentKernel;
    execute_watershed_dispatch_with_kernel(&graph, &topology_report, &mut kernel, surface)
        .expect("ws10 execution should return typed report")
}

fn state_value(
    report: &openwepp_watershed_orchestrator::WatershedKernelExecutionReport,
    symbol: &str,
) -> f64 {
    report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing state symbol {symbol}"))
        .as_f64()
}

#[test]
fn ws12_contract_conformance_deauthorizes_surrogate_when_structures_are_inactive() {
    let report = run_ws12_surface(seeded_ws12_surface());
    assert!(report.dispatch_report.is_success());

    let qo = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_qo"))
        .expect("ws12 impoundment must publish qo")
        .as_f64();

    // Contract vector: inactive structures contribute zero to outflow
    // composition under continuity/stage-discharge authority.
    assert!(qo.abs() <= 1.0e-12);
}

#[test]
fn ws12_contract_conformance_rejects_missing_required_coefficient_payload() {
    let mut surface = seeded_ws12_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("ws10_impoundment_1_a0"));

    let report = run_ws12_surface(surface);
    assert_eq!(report.step_reports.len(), 2);
    assert_eq!(
        report.step_reports[1].decision_status.message_id(),
        "WKERNEL-WS10-IMPOUNDMENT-E-001"
    );
    assert_eq!(
        report.step_reports[1].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn ws12_contract_conformance_rejects_non_finite_coefficient_payload() {
    let mut surface = seeded_ws12_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_a1"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = run_ws12_surface(surface);
    assert_eq!(report.step_reports.len(), 2);
    assert_eq!(
        report.step_reports[1].decision_status.message_id(),
        "WKERNEL-WS10-IMPOUNDMENT-E-002"
    );
    assert_eq!(
        report.step_reports[1].decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn ws12_contract_conformance_rejects_invalid_area_denominator() {
    let mut surface = seeded_ws12_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_a0"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_a1"),
        BoundaryValue::scalar(0.0),
    );

    let report = run_ws12_surface(surface);
    assert_eq!(report.step_reports.len(), 2);
    assert_eq!(
        report.step_reports[1].decision_status.message_id(),
        "WKERNEL-WS10-IMPOUNDMENT-E-003"
    );
    assert_eq!(
        report.step_reports[1].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn wshed03_contract_ws12_vector_requires_parser_projected_coefficients_without_manual_seed() {
    let surface = seeded_ws12_surface();

    let report = run_ws12_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "ws12 execution should succeed with parser-projected coefficients and no manual seed; step_reports={:?}",
        report.step_reports
    );

    let qo = state_value(&report, "ws10_impoundment_1_qo");
    let durout = state_value(&report, "ws10_impoundment_1_durout");
    let hnext = state_value(&report, "ws10_impoundment_1_hnext");
    let outflow_volume = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_outflow_volume"))
        .unwrap_or_else(|| panic!("missing flux symbol ws10_impoundment_1_outflow_volume"))
        .as_f64();

    assert!(qo.is_finite() && qo >= 0.0);
    assert!(durout.is_finite() && durout >= 0.0);
    assert!(hnext.is_finite() && hnext >= 0.0);
    assert!(outflow_volume.is_finite() && outflow_volume >= 0.0);
}

#[test]
fn wshed11_contract_ws12_vector_projects_active_structure_payloads() {
    let report = run_ws12_surface(seeded_ws12_active_surface());
    assert!(
        report.dispatch_report.is_success(),
        "ws12 execution should succeed on active payload projection surfaces; step_reports={:?}",
        report.step_reports
    );

    let qo = state_value(&report, "ws10_impoundment_1_qo");
    let durout = state_value(&report, "ws10_impoundment_1_durout");
    let hnext = state_value(&report, "ws10_impoundment_1_hnext");

    assert!(qo.is_finite() && qo >= 0.0);
    assert!(durout.is_finite() && durout >= 0.0);
    assert!(hnext.is_finite() && hnext >= 0.0);
}

#[test]
fn wshed03_contract_ws12_vector_requires_regime_transition_timestep_stability() {
    let mut fine_step = seeded_ws12_surface();
    fine_step.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_deltat"),
        BoundaryValue::scalar(0.1),
    );
    fine_step.state_surface.insert(
        BoundarySymbol::from("hs1_peakro"),
        BoundaryValue::scalar(4.0),
    );
    fine_step.state_surface.insert(
        BoundarySymbol::from("hs2_peakro"),
        BoundaryValue::scalar(3.0),
    );
    fine_step.state_surface.insert(
        BoundarySymbol::from("hs3_peakro"),
        BoundaryValue::scalar(2.0),
    );

    let mut coarse_step = fine_step.clone();
    coarse_step.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_deltat"),
        BoundaryValue::scalar(1.0),
    );

    let fine_report = run_ws12_surface(fine_step);
    let coarse_report = run_ws12_surface(coarse_step);
    assert!(
        fine_report.dispatch_report.is_success(),
        "fine-step ws12 run should succeed for RK4/regime-transition comparison"
    );
    assert!(
        coarse_report.dispatch_report.is_success(),
        "coarse-step ws12 run should succeed for RK4/regime-transition comparison"
    );

    let fine_hnext = state_value(&fine_report, "ws10_impoundment_1_hnext");
    let coarse_hnext = state_value(&coarse_report, "ws10_impoundment_1_hnext");
    assert!(
        (fine_hnext - coarse_hnext).abs() <= 1.0e-3,
        "RK4/adaptive regime-transition lineage requires timestep stability (fine={fine_hnext}, coarse={coarse_hnext})"
    );
}
