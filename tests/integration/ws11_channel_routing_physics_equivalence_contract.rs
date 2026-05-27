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

fn seeded_ws11_surface() -> WatershedWritebackSurface {
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
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_ishape"),
        BoundaryValue::scalar(1.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_ienslp"),
        BoundaryValue::scalar(1.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chnz"),
        BoundaryValue::scalar(19.99),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chnnbr"),
        BoundaryValue::scalar(0.03),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chntcr"),
        BoundaryValue::scalar(19.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chnedm"),
        BoundaryValue::scalar(900.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chneds"),
        BoundaryValue::scalar(0.0001),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_ctlz"),
        BoundaryValue::scalar(4.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_ctln"),
        BoundaryValue::scalar(0.04),
    );

    let impoundment = parse_watershed_impoundment_from_str(
        STRICT_VALID_WATERSHED_IMPOUNDMENT,
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

fn run_ws11_surface(
    surface: WatershedWritebackSurface,
) -> openwepp_watershed_orchestrator::WatershedKernelExecutionReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let mut kernel = Ws10ChannelImpoundmentKernel;
    execute_watershed_dispatch_with_kernel(&graph, &topology_report, &mut kernel, surface)
        .expect("ws11 execution should return typed report")
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

fn flux_value(
    report: &openwepp_watershed_orchestrator::WatershedKernelExecutionReport,
    symbol: &str,
) -> f64 {
    report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing flux symbol {symbol}"))
        .as_f64()
}

fn has_state_symbol(
    report: &openwepp_watershed_orchestrator::WatershedKernelExecutionReport,
    symbol: &str,
) -> bool {
    report
        .writeback_surface
        .state_surface
        .contains_key(&BoundarySymbol::from(symbol))
}

#[test]
fn ws11_contract_conformance_executes_ipeak_1_and_2_with_finite_outputs() {
    for ipeak in [1.0, 2.0] {
        let mut surface = seeded_ws11_surface();
        surface
            .state_surface
            .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(ipeak));

        let report = run_ws11_surface(surface);
        assert!(
            report.dispatch_report.is_success(),
            "ipeak={ipeak} must succeed; step_reports={:?}",
            report.step_reports
        );

        let qpo = state_value(&report, "ws10_channel_1_qpo");
        let durrof = state_value(&report, "ws10_channel_1_durrof");
        let roff = flux_value(&report, "ws10_channel_1_roff");

        assert!(qpo.is_finite() && qpo >= 0.0, "ipeak={ipeak} invalid qpo");
        assert!(
            durrof.is_finite() && durrof >= 0.0,
            "ipeak={ipeak} invalid durrof"
        );
        assert!(
            roff.is_finite() && roff >= 0.0,
            "ipeak={ipeak} invalid roff"
        );
    }
}

#[test]
fn ws11_contract_conformance_executes_ipeak_3_and_4_with_routed_closure() {
    for ipeak in [3.0, 4.0] {
        let mut surface = seeded_ws11_surface();
        surface
            .state_surface
            .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(ipeak));

        let report = run_ws11_surface(surface);
        assert!(
            report.dispatch_report.is_success(),
            "ipeak={ipeak} must succeed; step_reports={:?}",
            report.step_reports
        );

        let qpo = state_value(&report, "ws10_channel_1_qpo");
        let durrof = state_value(&report, "ws10_channel_1_durrof");
        let roff = flux_value(&report, "ws10_channel_1_roff");

        assert!(qpo.is_finite() && qpo >= 0.0, "ipeak={ipeak} invalid qpo");
        assert!(
            durrof.is_finite() && durrof >= 0.0,
            "ipeak={ipeak} invalid durrof"
        );
        assert!(
            roff.is_finite() && roff >= 0.0,
            "ipeak={ipeak} invalid roff"
        );
        assert!(
            (roff - (qpo * durrof)).abs() <= 1.0e-9,
            "ipeak={ipeak} violates closure roff=qpo*durrof"
        );
    }
}

#[test]
fn ws11_contract_conformance_requires_ipeak_symbol() {
    let mut surface = seeded_ws11_surface();
    surface.state_surface.remove(&BoundarySymbol::from("ipeak"));

    let report = run_ws11_surface(surface);
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
fn ws11_contract_conformance_rejects_non_finite_ipeak() {
    let mut surface = seeded_ws11_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("ipeak"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = run_ws11_surface(surface);
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
fn ws11_contract_conformance_rejects_out_of_domain_ipeak() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(0.0));

    let report = run_ws11_surface(surface);
    assert_eq!(report.step_reports.len(), 1);
    assert_eq!(
        report.step_reports[0].decision_status.message_id(),
        "WKERNEL-WS10-CHANNEL-E-003"
    );
    assert_eq!(
        report.step_reports[0].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn ws11_contract_conformance_distinguishes_ipeak_branches() {
    let mut ipeak_1_surface = seeded_ws11_surface();
    ipeak_1_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(1.0));
    let ipeak_1_report = run_ws11_surface(ipeak_1_surface);
    assert!(
        ipeak_1_report.dispatch_report.is_success(),
        "ipeak=1 should succeed; step_reports={:?}",
        ipeak_1_report.step_reports
    );

    let mut ipeak_4_surface = seeded_ws11_surface();
    ipeak_4_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    let ipeak_4_report = run_ws11_surface(ipeak_4_surface);
    assert!(
        ipeak_4_report.dispatch_report.is_success(),
        "ipeak=4 should succeed; step_reports={:?}",
        ipeak_4_report.step_reports
    );

    let qpo_ipeak_1 = state_value(&ipeak_1_report, "ws10_channel_1_qpo");
    let qpo_ipeak_4 = state_value(&ipeak_4_report, "ws10_channel_1_qpo");

    assert!(
        (qpo_ipeak_1 - qpo_ipeak_4).abs() > 1.0e-9,
        "ipeak branch outputs are identical ({qpo_ipeak_1}); expected explicit branch-dependent routing behavior"
    );
}

#[test]
fn wshed03_contract_kw_mc_vector_requires_wave_routing_state_family_publication() {
    for ipeak in [3.0, 4.0] {
        let mut surface = seeded_ws11_surface();
        surface
            .state_surface
            .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(ipeak));

        let report = run_ws11_surface(surface);
        assert!(
            report.dispatch_report.is_success(),
            "ipeak={ipeak} execution must succeed before state-family assertions"
        );

        for symbol in [
            "ws10_channel_1_q1",
            "ws10_channel_1_qin",
            "ws10_channel_1_qlat",
            "ws10_channel_1_c0",
            "ws10_channel_1_c1",
            "ws10_channel_1_c2",
            "ws10_channel_1_c3",
            "ws10_channel_1_c4",
        ] {
            assert!(
                has_state_symbol(&report, symbol),
                "ipeak={ipeak} missing required KW/MC lineage state symbol {symbol}"
            );
        }
    }
}

#[test]
fn wshed03_contract_channel_sediment_vector_requires_channel_sediment_publication_family() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));

    let report = run_ws11_surface(surface);
    assert!(report.dispatch_report.is_success());

    for symbol in ["ws10_channel_1_qsed", "ws10_channel_1_tc"] {
        assert!(
            has_state_symbol(&report, symbol),
            "missing required channel sediment lineage symbol {symbol}"
        );
    }
}

#[test]
#[allow(clippy::similar_names)]
fn wshedimpl15_contract_channel_sediment_scaffold_publishes_baseline_conversions() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));

    let report = run_ws11_surface(surface);
    assert!(report.dispatch_report.is_success());

    for symbol in [
        "ws10_channel_1_chz",
        "ws10_channel_1_nbarch",
        "ws10_channel_1_crsh",
        "ws10_channel_1_depmid",
        "ws10_channel_1_depsid",
    ] {
        assert!(
            has_state_symbol(&report, symbol),
            "missing required WS15 scaffold symbol {symbol}"
        );
    }

    let crsh = state_value(&report, "ws10_channel_1_crsh");
    let depmid = state_value(&report, "ws10_channel_1_depmid");
    let depsid = state_value(&report, "ws10_channel_1_depsid");

    assert!((crsh - (19.0 * 0.021)).abs() <= 1.0e-12);
    assert!((depmid - (900.0 * 3.281)).abs() <= 1.0e-9);
    assert!((depsid - (0.0001 * 3.281)).abs() <= 1.0e-12);
}

#[test]
fn wshedimpl15_contract_channel_sediment_scaffold_requires_projected_controls() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface
        .state_surface
        .remove(&BoundarySymbol::from("ws10_channel_1_chntcr"));

    let report = run_ws11_surface(surface);
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
