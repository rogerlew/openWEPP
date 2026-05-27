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
const VALID_TOPOLOGY_CHANNEL_DEPENDENCY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 2 0 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
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
            BoundarySymbol::from(format!("hs{hillslope_id}_particle_diameter_m_{class:04}")),
            BoundaryValue::scalar(0.000_01 * class_index_scalar(class)),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!(
                "hs{hillslope_id}_particle_flow_fraction_{class:04}"
            )),
            BoundaryValue::scalar(fraction),
        );
    }
}

fn seed_ws17_channel_segment_scaffold(surface: &mut WatershedWritebackSurface, node_id: u32) {
    surface.state_surface.insert(
        BoundarySymbol::from(format!("ws10_channel_{node_id}_nslpts")),
        BoundaryValue::scalar(3.0),
    );
    for (point_number, x, slope) in [(1, 0.0, 0.02), (2, 30.0, 0.08), (3, 60.0, 0.06)] {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("ws10_channel_{node_id}_x_{point_number:04}")),
            BoundaryValue::scalar(x),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("ws10_channel_{node_id}_slope_{point_number:04}")),
            BoundaryValue::scalar(slope),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("ws10_channel_{node_id}_depa_{point_number:04}")),
            BoundaryValue::scalar(2_952.9),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("ws10_channel_{node_id}_depb_{point_number:04}")),
            BoundaryValue::scalar(2_952.9),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("ws10_channel_{node_id}_wida_{point_number:04}")),
            BoundaryValue::scalar(98.43),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("ws10_channel_{node_id}_widb_{point_number:04}")),
            BoundaryValue::scalar(98.43),
        );
    }
}

fn seed_ws22_channel_crfrac(surface: &mut WatershedWritebackSurface, node_id: u32) {
    for (class, fraction) in [(1_u32, 0.2), (2, 0.3), (3, 0.5)] {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("ws10_channel_{node_id}_crfrac_{class:04}")),
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
    seed_ws17_channel_segment_scaffold(&mut runtime_surface, 1);

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
    seed_ws17_channel_segment_scaffold(&mut runtime_surface, 2);

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
    run_ws11_surface_with_topology(surface, VALID_TOPOLOGY)
}

fn run_ws11_surface_with_topology(
    surface: WatershedWritebackSurface,
    topology: &str,
) -> openwepp_watershed_orchestrator::WatershedKernelExecutionReport {
    let graph = parse_topology_fixture_str(topology).expect("fixture should parse");
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
    let dtchr = surface
        .state_surface
        .get(&BoundarySymbol::from("dtchr"))
        .expect("ws11 seeded surface must include dtchr")
        .as_f64();

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshed03 channel sediment vector must succeed; step_reports={:?}",
        report.step_reports
    );

    for symbol in ["ws10_channel_1_qsed", "ws10_channel_1_tc"] {
        assert!(
            has_state_symbol(&report, symbol),
            "missing required channel sediment lineage symbol {symbol}"
        );
    }

    let qsed = state_value(&report, "ws10_channel_1_qsed");
    let tc = state_value(&report, "ws10_channel_1_tc");
    let incoming_mass_kg = (2.0 * 300.0 * 0.01) - (2.0 * 300.0 * 0.0025);
    let expected_qsed = incoming_mass_kg / dtchr.max(300.0);

    assert!((qsed - expected_qsed).abs() <= 1.0e-12);
    assert!(tc.is_finite() && tc >= 0.0);
    assert!(
        (tc - qsed).abs() > 1.0e-9,
        "tc must not collapse to the pre-migration surrogate identity tc=qsed"
    );
}

#[test]
fn wshedimpl18_contract_channel_transport_capacity_responds_to_particle_diameter() {
    let mut baseline_surface = seeded_ws11_surface();
    baseline_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    let baseline_report = run_ws11_surface(baseline_surface);
    assert!(baseline_report.dispatch_report.is_success());

    let baseline_qsed = state_value(&baseline_report, "ws10_channel_1_qsed");
    let baseline_tc = state_value(&baseline_report, "ws10_channel_1_tc");

    let mut perturbed_surface = seeded_ws11_surface();
    perturbed_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    perturbed_surface.state_surface.insert(
        BoundarySymbol::from("hs1_particle_diameter_m_0001"),
        BoundaryValue::scalar(0.0005),
    );
    let perturbed_report = run_ws11_surface(perturbed_surface);
    assert!(perturbed_report.dispatch_report.is_success());

    let perturbed_qsed = state_value(&perturbed_report, "ws10_channel_1_qsed");
    let perturbed_tc = state_value(&perturbed_report, "ws10_channel_1_tc");

    assert!((baseline_qsed - perturbed_qsed).abs() <= 1.0e-12);
    assert!(
        (baseline_tc - perturbed_tc).abs() > 1.0e-9,
        "transport-capacity branch must respond to class-diameter changes"
    );
}

#[test]
fn wshedimpl19_contract_channel_exports_class_payload_family() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl19 class-payload export vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let class_count = state_value(&report, "ws10_channel_1_particle_class_count");
    assert!((class_count - 3.0).abs() <= 1.0e-12);

    let mut fraction_sum = 0.0_f64;
    for class in 1..=3 {
        let fraction = state_value(
            &report,
            &format!("ws10_channel_1_particle_flow_fraction_{class:04}"),
        );
        let diameter = state_value(
            &report,
            &format!("ws10_channel_1_particle_diameter_m_{class:04}"),
        );

        assert!(fraction.is_finite() && (0.0..=1.0).contains(&fraction));
        assert!(diameter.is_finite() && diameter > 0.0);
        fraction_sum += fraction;
    }
    assert!((fraction_sum - 1.0).abs() <= 1.0e-12);
}

#[test]
fn wshedimpl19_contract_channel_ingresses_upstream_channel_payload() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface.state_surface.insert(
        BoundarySymbol::from("hs3_total_detachment_kg"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("hs3_total_deposition_kg"),
        BoundaryValue::scalar(0.0),
    );

    let report = run_ws11_surface_with_topology(surface, VALID_TOPOLOGY_CHANNEL_DEPENDENCY);
    assert!(report.dispatch_report.is_success());

    let upstream_qsed = state_value(&report, "ws10_channel_1_qsed");
    let downstream_qsed = state_value(&report, "ws10_channel_2_qsed");

    assert!(upstream_qsed > 0.0);
    assert!((downstream_qsed - upstream_qsed).abs() <= 1.0e-12);
}

#[test]
fn wshedimpl20_contract_case12_routing_is_opt_in_and_defaults_to_zero_diagnostics() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl20 opt-in vector must succeed; step_reports={:?}",
        report.step_reports
    );

    for symbol in [
        "ws10_channel_1_ws20_case1_segment_count",
        "ws10_channel_1_ws20_case2_segment_count",
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count",
    ] {
        assert!(
            has_state_symbol(&report, symbol),
            "missing required wshedimpl20 diagnostics symbol {symbol}"
        );
    }

    assert!((state_value(&report, "ws10_channel_1_ws20_case1_segment_count") - 0.0).abs() <= 1e-12);
    assert!((state_value(&report, "ws10_channel_1_ws20_case2_segment_count") - 0.0).abs() <= 1e-12);
    assert!(
        (state_value(
            &report,
            "ws10_channel_1_ws20_detachment_unmigrated_segment_count"
        ) - 0.0)
            .abs()
            <= 1e-12
    );
}

#[test]
fn wshedimpl20_contract_case12_opt_in_tracks_detachment_unmigrated_diagnostics() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws20_case12_enable"),
        BoundaryValue::scalar(1.0),
    );

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl20 opt-in vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let case1_segments = state_value(&report, "ws10_channel_1_ws20_case1_segment_count");
    let case2_segments = state_value(&report, "ws10_channel_1_ws20_case2_segment_count");
    let detachment_unmigrated = state_value(
        &report,
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count",
    );

    assert!(case1_segments >= 0.0);
    assert!(case2_segments >= 0.0);
    assert!(
        detachment_unmigrated > 0.0,
        "expected detachment-unmigrated diagnostics to be tracked under ws20 opt-in"
    );
}

#[test]
fn wshedimpl21_contract_case34_routing_is_opt_in_and_defaults_to_zero_diagnostics() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl21 default-off vector must succeed; step_reports={:?}",
        report.step_reports
    );

    for symbol in [
        "ws10_channel_1_ws21_case3_segment_count",
        "ws10_channel_1_ws21_case4_segment_count",
        "ws10_channel_1_ws21_enddet_segment_count",
        "ws10_channel_1_ws21_detach_unmigrated_segment_count",
    ] {
        assert!(
            has_state_symbol(&report, symbol),
            "missing required wshedimpl21 diagnostics symbol {symbol}"
        );
    }

    assert!((state_value(&report, "ws10_channel_1_ws21_case3_segment_count") - 0.0).abs() <= 1e-12);
    assert!((state_value(&report, "ws10_channel_1_ws21_case4_segment_count") - 0.0).abs() <= 1e-12);
    assert!(
        (state_value(&report, "ws10_channel_1_ws21_enddet_segment_count") - 0.0).abs() <= 1e-12
    );
    assert!(
        (state_value(
            &report,
            "ws10_channel_1_ws21_detach_unmigrated_segment_count"
        ) - 0.0)
            .abs()
            <= 1e-12
    );
}

#[test]
fn wshedimpl21_contract_case34_opt_in_tracks_case34_and_unmigrated_diagnostics() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws20_case12_enable"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws21_case34_enable"),
        BoundaryValue::scalar(1.0),
    );
    seed_ws22_channel_crfrac(&mut surface, 1);

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl21 opt-in vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let case3_segments = state_value(&report, "ws10_channel_1_ws21_case3_segment_count");
    let case4_segments = state_value(&report, "ws10_channel_1_ws21_case4_segment_count");
    let enddet_segments = state_value(&report, "ws10_channel_1_ws21_enddet_segment_count");
    let detach_unmigrated = state_value(
        &report,
        "ws10_channel_1_ws21_detach_unmigrated_segment_count",
    );
    let ws20_unmigrated = state_value(
        &report,
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count",
    );

    assert!(
        (case3_segments + case4_segments) > 0.0,
        "expected ws21 case34 diagnostics to register at least one case3/case4 segment"
    );
    assert!(enddet_segments >= 0.0);
    assert!(
        detach_unmigrated > 0.0,
        "expected ws21 detach/dcap unmigrated diagnostics to be tracked under ws21 opt-in"
    );
    assert!(ws20_unmigrated >= detach_unmigrated);
}

#[test]
fn wshedimpl22_contract_ws21_detach_dcap_requires_crfrac_projection() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws20_case12_enable"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws21_case34_enable"),
        BoundaryValue::scalar(1.0),
    );

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
fn wshedimpl22_contract_ws21_opt_in_routes_with_crfrac_projection() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws20_case12_enable"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws21_case34_enable"),
        BoundaryValue::scalar(1.0),
    );
    seed_ws22_channel_crfrac(&mut surface, 1);

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl22 opt-in vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let case3_segments = state_value(&report, "ws10_channel_1_ws21_case3_segment_count");
    let case4_segments = state_value(&report, "ws10_channel_1_ws21_case4_segment_count");
    let enddet_segments = state_value(&report, "ws10_channel_1_ws21_enddet_segment_count");
    let ws21_unmigrated = state_value(
        &report,
        "ws10_channel_1_ws21_detach_unmigrated_segment_count",
    );
    let ws20_unmigrated = state_value(
        &report,
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count",
    );

    assert!((case3_segments + case4_segments) > 0.0);
    assert!(enddet_segments >= 0.0);
    assert!(ws20_unmigrated >= ws21_unmigrated);
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

#[test]
fn wshedimpl16_contract_channel_sediment_requires_particle_diameter_payload() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface
        .state_surface
        .remove(&BoundarySymbol::from("hs1_particle_diameter_m_0003"));

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
fn wshedimpl17_contract_channel_segment_scaffold_requires_ws17_symbols() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface
        .state_surface
        .remove(&BoundarySymbol::from("ws10_channel_1_x_0002"));

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
