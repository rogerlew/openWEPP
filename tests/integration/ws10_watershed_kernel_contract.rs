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

fn seeded_ws10_surface() -> WatershedWritebackSurface {
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

    // The canonical strict fixture defines one channel record; duplicate it for
    // channel:2 so the three-node WS10 topology has per-channel controls.
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
        STRICT_VALID_WATERSHED_IMPOUNDMENT,
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect("strict watershed impoundment fixture should parse");
    seed_watershed_runtime_surface_from_watershed_impoundment(&mut runtime_surface, &impoundment)
        .expect("watershed impoundment runtime seed should project ws10 symbols");

    runtime_surface.state_surface.insert(
        BoundarySymbol::from("hs1_peakro"),
        BoundaryValue::scalar(2.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("hs1_watdur"),
        BoundaryValue::scalar(300.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("hs2_peakro"),
        BoundaryValue::scalar(1.5),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("hs2_watdur"),
        BoundaryValue::scalar(400.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("hs3_peakro"),
        BoundaryValue::scalar(0.5),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("hs3_watdur"),
        BoundaryValue::scalar(200.0),
    );
    runtime_surface
}

fn run_ws10_surface(
    surface: WatershedWritebackSurface,
) -> openwepp_watershed_orchestrator::WatershedKernelExecutionReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let mut kernel = Ws10ChannelImpoundmentKernel;
    execute_watershed_dispatch_with_kernel(&graph, &topology_report, &mut kernel, surface)
        .expect("ws10 execution should return typed report")
}

#[test]
fn ws10_contract_conformance_executes_channel_impoundment_production_path() {
    let report = run_ws10_surface(seeded_ws10_surface());
    assert!(report.dispatch_report.is_success());

    let ch1_qpo = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_1_qpo"))
        .expect("ws10 channel 1 must publish qpo")
        .as_f64();
    let imp1_qo = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_qo"))
        .expect("ws10 impoundment 1 must publish qo")
        .as_f64();
    let ch2_qpo = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_channel_2_qpo"))
        .expect("ws10 channel 2 must publish qpo")
        .as_f64();

    assert!(ch1_qpo.is_finite() && ch1_qpo > 0.0);
    assert!(imp1_qo.is_finite() && imp1_qo >= 0.0);
    assert!(ch2_qpo.is_finite() && ch2_qpo > 0.0);
}

#[test]
fn ws10_contract_conformance_rejects_missing_required_symbol() {
    let mut surface = seeded_ws10_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("ws10_channel_1_chnn"));

    let report = run_ws10_surface(surface);
    assert_eq!(report.step_reports.len(), 1);
    assert_eq!(
        report.step_reports[0].decision_status.message_id(),
        "WKERNEL-WS10-CHANNEL-E-001"
    );
    assert_eq!(
        report.step_reports[0].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn ws10_contract_conformance_rejects_non_finite_required_symbol() {
    let mut surface = seeded_ws10_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("hs1_peakro"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = run_ws10_surface(surface);
    assert_eq!(report.step_reports.len(), 1);
    assert_eq!(
        report.step_reports[0].decision_status.message_id(),
        "WKERNEL-WS10-CHANNEL-E-002"
    );
    assert_eq!(
        report.step_reports[0].decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn ws10_contract_conformance_rejects_out_of_domain_impoundment_state() {
    let mut surface = seeded_ws10_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_h"),
        BoundaryValue::scalar(0.90),
    );

    let report = run_ws10_surface(surface);
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
