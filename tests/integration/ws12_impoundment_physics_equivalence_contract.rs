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

const WS12_COEFFICIENT_SURFACE: [(&str, f64); 14] = [
    ("a", 0.0),
    ("b", 0.0),
    ("c", 0.0),
    ("d", 0.0),
    ("e", 0.0),
    ("ha", 0.0),
    ("ht", 0.0),
    ("hlm", 0.0),
    ("a0", 125.0),
    ("a1", 15.0),
    ("a2", 1.0),
    ("l0", 25.0),
    ("l1", 0.0),
    ("l2", 0.0),
];

fn seed_ws12_coefficient_surface(surface: &mut WatershedWritebackSurface) {
    for (suffix, value) in WS12_COEFFICIENT_SURFACE {
        let symbol = format!("ws10_impoundment_1_{suffix}");
        surface
            .state_surface
            .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }
}

fn seeded_ws12_surface() -> WatershedWritebackSurface {
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
        STRICT_VALID_WATERSHED_IMPOUNDMENT,
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect("strict watershed impoundment fixture should parse");
    seed_watershed_runtime_surface_from_watershed_impoundment(&mut runtime_surface, &impoundment)
        .expect("watershed impoundment runtime seed should project ws10 symbols");

    seed_ws12_coefficient_surface(&mut runtime_surface);

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
