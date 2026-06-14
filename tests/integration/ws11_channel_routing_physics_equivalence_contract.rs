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

fn seed_zero_sediment_hillslope_payload(
    surface: &mut WatershedWritebackSurface,
    hillslope_id: u32,
) {
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_total_detachment_kg")),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_total_deposition_kg")),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_particle_class_count")),
        BoundaryValue::scalar(EROD15_CLASS_COUNT_SCALAR),
    );

    for class in 1..=3 {
        surface.state_surface.insert(
            BoundarySymbol::from(format!(
                "hs{hillslope_id}_sediment_concentration_kg_m3_{class:04}"
            )),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_particle_diameter_m_{class:04}")),
            BoundaryValue::scalar(0.000_01 * class_index_scalar(class)),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!(
                "hs{hillslope_id}_particle_flow_fraction_{class:04}"
            )),
            BoundaryValue::scalar(0.0),
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

fn seed_ws24_case12_transition_forcing(surface: &mut WatershedWritebackSurface) {
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws20_case12_enable"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws21_case34_enable"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_slope_0002"),
        BoundaryValue::scalar(0.0002),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_slope_0003"),
        BoundaryValue::scalar(0.0002),
    );

    for hillslope_id in [1_u32, 2, 3] {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_total_detachment_kg")),
            BoundaryValue::scalar(10.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_total_deposition_kg")),
            BoundaryValue::scalar(0.0),
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
fn wshedimpl37_contract_wshcqi_runon_lineage_publishes_partitioned_volumes_and_duration_max() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl37 runon-lineage vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let rvolat = state_value(&report, "ws10_channel_1_rvolat");
    let rvotop = state_value(&report, "ws10_channel_1_rvotop");
    let rvolon = state_value(&report, "ws10_channel_1_rvolon");
    let durrunon = state_value(&report, "ws10_channel_1_durrunon");
    let durlat = state_value(&report, "ws10_channel_1_durlat");
    let durtop = state_value(&report, "ws10_channel_1_durtop");
    let durchan = state_value(&report, "ws10_channel_1_durchan");
    let watdur = state_value(&report, "ws10_channel_1_watdur");

    assert!((rvolat - 600.0).abs() <= 1.0e-9);
    assert!((rvotop - 0.0).abs() <= 1.0e-12);
    assert!((rvolon - (rvolat + rvotop)).abs() <= 1.0e-9);
    assert!((durlat - 300.0).abs() <= 1.0e-12);
    assert!((durtop - 0.0).abs() <= 1.0e-12);
    assert!((durrunon - durlat.max(durtop)).abs() <= 1.0e-12);
    assert!((durchan - 600.0).abs() <= 1.0e-12);
    assert!((watdur - durrunon.max(durchan)).abs() <= 1.0e-12);
}

#[test]
fn wshedimpl37_contract_wshirs_threshold_branch_zeroes_ipeak1_outputs_for_tiny_runvol() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(1.0));
    surface
        .flux_surface
        .insert(BoundarySymbol::from("cbase"), BoundaryValue::scalar(0.0));
    for hillslope_id in [1_u32, 2, 3] {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_peakro")),
            BoundaryValue::scalar(1.0e-9),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_watdur")),
            BoundaryValue::scalar(1.0),
        );
    }

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl37 threshold vector must succeed; step_reports={:?}",
        report.step_reports
    );

    assert!((state_value(&report, "ws10_channel_1_ws11_runoff_case") - 4.0).abs() <= 1.0e-12);
    assert!((state_value(&report, "ws10_channel_1_qpo") - 0.0).abs() <= 1.0e-12);
    assert!((state_value(&report, "ws10_channel_1_durrof") - 0.0).abs() <= 1.0e-12);
    assert!((flux_value(&report, "ws10_channel_1_roff") - 0.0).abs() <= 1.0e-12);
}

#[test]
fn wshedimpl37_contract_wshrun_routes_incoming_hydrograph_when_local_runoff_absent_for_ipeak4() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface
        .flux_surface
        .insert(BoundarySymbol::from("cbase"), BoundaryValue::scalar(0.0));

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl37 non-local-runoff wave-routing vector must succeed; step_reports={:?}",
        report.step_reports
    );

    assert!((state_value(&report, "ws10_channel_1_rofc") - 0.0).abs() <= 1.0e-12);
    assert!((state_value(&report, "ws10_channel_1_ws11_runoff_case") - 3.0).abs() <= 1.0e-12);
    assert!(state_value(&report, "ws10_channel_1_rvolon") > 0.001);
    assert!(state_value(&report, "ws10_channel_1_qpo") > 0.0);
    assert!(has_state_symbol(&report, "ws10_channel_1_q1"));
    assert!(has_state_symbol(&report, "ws10_channel_1_qin"));
    assert!(has_state_symbol(&report, "ws10_channel_1_qlat"));
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
fn wshedimpl40_contract_mc_lateral_term_matches_single_segment_baseline_scaling() {
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
        "wshedimpl40 mc lateral-term vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let qlat = state_value(&report, "ws10_channel_1_qlat");
    let c0 = state_value(&report, "ws10_channel_1_c0");
    let c4 = state_value(&report, "ws10_channel_1_c4");
    let expected_c4 = 2.0 * qlat * dtchr * c0;

    assert!(
        (c4 - expected_c4).abs() <= 1.0e-9,
        "wshedimpl40 expected c4=2*qlat*dtchr*c0; observed c4={c4}, expected={expected_c4}, qlat={qlat}, dtchr={dtchr}, c0={c0}"
    );
}

#[test]
fn wshedimpl40_contract_mc_prior_wave_state_memory_changes_branch_output() {
    let mut baseline_surface = seeded_ws11_surface();
    baseline_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    let baseline_report = run_ws11_surface(baseline_surface);
    assert!(
        baseline_report.dispatch_report.is_success(),
        "wshedimpl40 baseline mc vector must succeed; step_reports={:?}",
        baseline_report.step_reports
    );

    let baseline_qpo = state_value(&baseline_report, "ws10_channel_1_qpo");

    let mut prior_seeded_surface = seeded_ws11_surface();
    prior_seeded_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    prior_seeded_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_qin"),
        BoundaryValue::scalar(0.05),
    );
    prior_seeded_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_q1"),
        BoundaryValue::scalar(2.5),
    );

    let prior_seeded_report = run_ws11_surface(prior_seeded_surface);
    assert!(
        prior_seeded_report.dispatch_report.is_success(),
        "wshedimpl40 prior-state mc vector must succeed; step_reports={:?}",
        prior_seeded_report.step_reports
    );

    let prior_seeded_qpo = state_value(&prior_seeded_report, "ws10_channel_1_qpo");
    assert!(
        (prior_seeded_qpo - baseline_qpo).abs() > 1.0e-9,
        "wshedimpl40 prior-wave-state memory must influence MC routing output; baseline_qpo={baseline_qpo}, prior_seeded_qpo={prior_seeded_qpo}"
    );
}

#[test]
fn wshedimpl40_contract_mc_coefficients_allow_signed_publication() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ctlslp"),
        BoundaryValue::scalar(1.2),
    );

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl40 signed-coefficient vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let c1 = state_value(&report, "ws10_channel_1_c1");
    let c2 = state_value(&report, "ws10_channel_1_c2");
    let c3 = state_value(&report, "ws10_channel_1_c3");

    assert!(c1.is_finite() && c2.is_finite() && c3.is_finite());
    assert!(
        (c3 - (1.0 - c1 - c2)).abs() <= 1.0e-12,
        "wshedimpl40 expected MC coefficient closure c3=1-c1-c2; c1={c1}, c2={c2}, c3={c3}"
    );
}

#[test]
fn wshedimpl41_contract_ipeak5_dynamic_refresh_diverges_from_ipeak4_coefficients() {
    let mut ipeak4_surface = seeded_ws11_surface();
    ipeak4_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    ipeak4_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_qin"),
        BoundaryValue::scalar(0.2),
    );
    ipeak4_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_q1"),
        BoundaryValue::scalar(1.2),
    );

    let ipeak4_report = run_ws11_surface(ipeak4_surface);
    assert!(
        ipeak4_report.dispatch_report.is_success(),
        "wshedimpl41 ipeak=4 control vector must succeed; step_reports={:?}",
        ipeak4_report.step_reports
    );

    let mut ipeak5_surface = seeded_ws11_surface();
    ipeak5_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(5.0));
    ipeak5_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_qin"),
        BoundaryValue::scalar(0.2),
    );
    ipeak5_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_q1"),
        BoundaryValue::scalar(1.2),
    );

    let ipeak5_report = run_ws11_surface(ipeak5_surface);
    assert!(
        ipeak5_report.dispatch_report.is_success(),
        "wshedimpl41 ipeak=5 dynamic vector must succeed; step_reports={:?}",
        ipeak5_report.step_reports
    );

    let c0_4 = state_value(&ipeak4_report, "ws10_channel_1_c0");
    let c1_4 = state_value(&ipeak4_report, "ws10_channel_1_c1");
    let c2_4 = state_value(&ipeak4_report, "ws10_channel_1_c2");
    let c3_4 = state_value(&ipeak4_report, "ws10_channel_1_c3");
    let c4_4 = state_value(&ipeak4_report, "ws10_channel_1_c4");
    let c0_5 = state_value(&ipeak5_report, "ws10_channel_1_c0");
    let c1_5 = state_value(&ipeak5_report, "ws10_channel_1_c1");
    let c2_5 = state_value(&ipeak5_report, "ws10_channel_1_c2");
    let c3_5 = state_value(&ipeak5_report, "ws10_channel_1_c3");
    let c4_5 = state_value(&ipeak5_report, "ws10_channel_1_c4");

    let coefficient_delta = (c0_5 - c0_4).abs()
        + (c1_5 - c1_4).abs()
        + (c2_5 - c2_4).abs()
        + (c3_5 - c3_4).abs()
        + (c4_5 - c4_4).abs();
    assert!(
        coefficient_delta > 1.0e-9,
        "wshedimpl41 expected ipeak=5 dynamic refresh to diverge from ipeak=4 coefficients; delta={coefficient_delta}, ipeak4=[{c0_4},{c1_4},{c2_4},{c3_4},{c4_4}], ipeak5=[{c0_5},{c1_5},{c2_5},{c3_5},{c4_5}]"
    );
}

#[test]
fn wshedimpl41_contract_ipeak5_dynamic_coefficients_respond_to_prior_state_seed() {
    let mut baseline_surface = seeded_ws11_surface();
    baseline_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(5.0));
    baseline_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_qin"),
        BoundaryValue::scalar(0.2),
    );
    baseline_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_q1"),
        BoundaryValue::scalar(1.2),
    );
    let baseline_report = run_ws11_surface(baseline_surface);
    assert!(
        baseline_report.dispatch_report.is_success(),
        "wshedimpl41 baseline ipeak=5 vector must succeed; step_reports={:?}",
        baseline_report.step_reports
    );

    let mut perturbed_surface = seeded_ws11_surface();
    perturbed_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(5.0));
    perturbed_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_qin"),
        BoundaryValue::scalar(4.8),
    );
    perturbed_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_q1"),
        BoundaryValue::scalar(0.05),
    );
    let perturbed_report = run_ws11_surface(perturbed_surface);
    assert!(
        perturbed_report.dispatch_report.is_success(),
        "wshedimpl41 perturbed ipeak=5 vector must succeed; step_reports={:?}",
        perturbed_report.step_reports
    );

    let baseline_c0 = state_value(&baseline_report, "ws10_channel_1_c0");
    let baseline_c1 = state_value(&baseline_report, "ws10_channel_1_c1");
    let baseline_c2 = state_value(&baseline_report, "ws10_channel_1_c2");
    let baseline_c3 = state_value(&baseline_report, "ws10_channel_1_c3");
    let baseline_c4 = state_value(&baseline_report, "ws10_channel_1_c4");
    let perturbed_c0 = state_value(&perturbed_report, "ws10_channel_1_c0");
    let perturbed_c1 = state_value(&perturbed_report, "ws10_channel_1_c1");
    let perturbed_c2 = state_value(&perturbed_report, "ws10_channel_1_c2");
    let perturbed_c3 = state_value(&perturbed_report, "ws10_channel_1_c3");
    let perturbed_c4 = state_value(&perturbed_report, "ws10_channel_1_c4");

    let coefficient_delta = (perturbed_c0 - baseline_c0).abs()
        + (perturbed_c1 - baseline_c1).abs()
        + (perturbed_c2 - baseline_c2).abs()
        + (perturbed_c3 - baseline_c3).abs()
        + (perturbed_c4 - baseline_c4).abs();
    assert!(
        coefficient_delta > 1.0e-9,
        "wshedimpl41 expected ipeak=5 dynamic coefficients to respond to prior-state seed perturbation; delta={coefficient_delta}, baseline=[{baseline_c0},{baseline_c1},{baseline_c2},{baseline_c3},{baseline_c4}], perturbed=[{perturbed_c0},{perturbed_c1},{perturbed_c2},{perturbed_c3},{perturbed_c4}]"
    );
}

#[test]
fn wshedimpl41_contract_ipeak5_dynamic_lateral_term_preserves_single_segment_scaling() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(5.0));
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_qin"),
        BoundaryValue::scalar(1.1),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_q1"),
        BoundaryValue::scalar(0.7),
    );

    let dtchr = surface
        .state_surface
        .get(&BoundarySymbol::from("dtchr"))
        .expect("ws11 seeded surface must include dtchr")
        .as_f64();

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl41 ipeak=5 lateral-term vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let qlat = state_value(&report, "ws10_channel_1_qlat");
    let c0 = state_value(&report, "ws10_channel_1_c0");
    let c4 = state_value(&report, "ws10_channel_1_c4");
    let expected_c4 = 2.0 * qlat * dtchr * c0;

    assert!(
        (c4 - expected_c4).abs() <= 1.0e-9,
        "wshedimpl41 expected ipeak=5 c4=2*qlat*dtchr*c0; observed c4={c4}, expected={expected_c4}, qlat={qlat}, dtchr={dtchr}, c0={c0}"
    );
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
fn wshed01_wc_zero_sediment_hillslope_payload_allows_zero_fraction_support() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    for hillslope_id in [1_u32, 2, 3] {
        seed_zero_sediment_hillslope_payload(&mut surface, hillslope_id);
    }

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "complete zero-sediment HBP payloads with zero fractions must route; step_reports={:?}",
        report.step_reports
    );

    assert!(state_value(&report, "ws10_channel_1_qsed").abs() <= 1.0e-12);
    assert!(state_value(&report, "ws10_channel_2_qsed").abs() <= 1.0e-12);
    assert!(state_value(&report, "ws10_channel_1_particle_class_count").abs() <= 1.0e-12);
    assert!(state_value(&report, "ws10_channel_2_particle_class_count").abs() <= 1.0e-12);
}

#[test]
fn wshed01_wc_nchnum_zero_disables_channel_detail_output_without_blocking_routing() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("nchnum"), BoundaryValue::scalar(0.0));

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "nchnum=0 is an output-selection state, not a routing domain violation; step_reports={:?}",
        report.step_reports
    );

    assert!(state_value(&report, "ws10_channel_1_qpo") > 0.0);
    assert!(state_value(&report, "ws10_channel_2_qpo") > 0.0);
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
        "ws10_channel_1_ws24_case2_detach_segment_count",
    ] {
        assert!(
            has_state_symbol(&report, symbol),
            "missing required wshedimpl20 diagnostics symbol {symbol}"
        );
    }

    assert!((state_value(&report, "ws10_channel_1_ws20_case1_segment_count") - 0.0).abs() <= 1e-12);
    assert!((state_value(&report, "ws10_channel_1_ws20_case2_segment_count") - 0.0).abs() <= 1e-12);
    assert!(
        (state_value(&report, "ws10_channel_1_ws24_case2_detach_segment_count") - 0.0).abs()
            <= 1e-12
    );
}

#[test]
fn wshedimpl25_contract_ws20_only_opt_in_requires_crfrac_projection() {
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
        report
            .step_reports
            .iter()
            .any(|step| step.decision_status.message_id() == "WKERNEL-WS10-CHANNEL-E-001"),
        "expected missing crfrac projection failure under ws20-only opt-in; step_reports={:?}",
        report.step_reports
    );
}

#[test]
fn wshedimpl25_contract_ws20_only_opt_in_auto_activates_ws21_with_crfrac_projection() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws20_case12_enable"),
        BoundaryValue::scalar(1.0),
    );
    seed_ws22_channel_crfrac(&mut surface, 1);

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl25 ws20-only opt-in vector must succeed with crfrac projection; step_reports={:?}",
        report.step_reports
    );

    let case3_segments = state_value(&report, "ws10_channel_1_ws21_case3_segment_count");
    let case4_segments = state_value(&report, "ws10_channel_1_ws21_case4_segment_count");

    assert!(
        (case3_segments + case4_segments) > 0.0,
        "expected ws21 case34 activity under ws20-only opt-in when crfrac is projected"
    );
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count"
    ));
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws21_detach_unmigrated_segment_count"
    ));
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
}

#[test]
fn wshedimpl21_contract_case34_opt_in_tracks_case34_diagnostics() {
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

    assert!(
        (case3_segments + case4_segments) > 0.0,
        "expected ws21 case34 diagnostics to register at least one case3/case4 segment"
    );
    assert!(enddet_segments >= 0.0);
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws21_detach_unmigrated_segment_count"
    ));
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count"
    ));
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

    assert!((case3_segments + case4_segments) > 0.0);
    assert!(enddet_segments >= 0.0);
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count"
    ));
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws21_detach_unmigrated_segment_count"
    ));
}

#[test]
fn wshedimpl23_contract_ws21_case4_detach_iterative_closure_retires_unresolved_symbols() {
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
        "wshedimpl23 opt-in vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let case4_segments = state_value(&report, "ws10_channel_1_ws21_case4_segment_count");
    assert!(case4_segments > 0.0);
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws21_detach_unmigrated_segment_count"
    ));
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count"
    ));
}

#[test]
fn wshedimpl26_contract_ws21_case4_iterative_closure_stress_vector_remains_resolved() {
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
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_chnk"),
        BoundaryValue::scalar(0.01),
    );
    seed_ws22_channel_crfrac(&mut surface, 1);

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl26 stress vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let case4_segments = state_value(&report, "ws10_channel_1_ws21_case4_segment_count");

    assert!(case4_segments > 0.0);
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws21_detach_unmigrated_segment_count"
    ));
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count"
    ));
}

#[test]
fn wshedimpl27_contract_ws21_case4_bracket_migration_vector_remains_resolved() {
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
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_chnk"),
        BoundaryValue::scalar(0.01),
    );
    seed_ws22_channel_crfrac(&mut surface, 1);

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl27 bracket migration vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let case4_segments = state_value(&report, "ws10_channel_1_ws21_case4_segment_count");
    let enddet_segments = state_value(&report, "ws10_channel_1_ws21_enddet_segment_count");

    assert!(case4_segments > 0.0);
    assert!(enddet_segments >= 0.0);
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws21_detach_unmigrated_segment_count"
    ));
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count"
    ));
}

#[test]
fn wshedimpl28_contract_ws20_routing_responds_to_wida_lower_boundary_widths() {
    let mut baseline_surface = seeded_ws11_surface();
    baseline_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    baseline_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws20_case12_enable"),
        BoundaryValue::scalar(1.0),
    );
    baseline_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws21_case34_enable"),
        BoundaryValue::scalar(1.0),
    );
    baseline_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ishape"),
        BoundaryValue::scalar(2.0),
    );
    seed_ws22_channel_crfrac(&mut baseline_surface, 1);

    let baseline_report = run_ws11_surface(baseline_surface);
    assert!(
        baseline_report.dispatch_report.is_success(),
        "wshedimpl28 baseline routing vector must succeed; step_reports={:?}",
        baseline_report.step_reports
    );

    let baseline_qsed = state_value(&baseline_report, "ws10_channel_1_qsed");
    let baseline_tc = state_value(&baseline_report, "ws10_channel_1_tc");

    let mut perturbed_surface = seeded_ws11_surface();
    perturbed_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    perturbed_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws20_case12_enable"),
        BoundaryValue::scalar(1.0),
    );
    perturbed_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws21_case34_enable"),
        BoundaryValue::scalar(1.0),
    );
    perturbed_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ishape"),
        BoundaryValue::scalar(2.0),
    );
    perturbed_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_wida_0002"),
        BoundaryValue::scalar(5.0),
    );
    perturbed_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_wida_0003"),
        BoundaryValue::scalar(5.0),
    );
    seed_ws22_channel_crfrac(&mut perturbed_surface, 1);

    let perturbed_report = run_ws11_surface(perturbed_surface);
    assert!(
        perturbed_report.dispatch_report.is_success(),
        "wshedimpl28 perturbed routing vector must succeed; step_reports={:?}",
        perturbed_report.step_reports
    );

    let perturbed_qsed = state_value(&perturbed_report, "ws10_channel_1_qsed");
    let perturbed_tc = state_value(&perturbed_report, "ws10_channel_1_tc");

    let qsed_shift = (baseline_qsed - perturbed_qsed).abs();
    let tc_shift = (baseline_tc - perturbed_tc).abs();
    assert!(
        qsed_shift > 1.0e-9 || tc_shift > 1.0e-9,
        "expected lower-boundary wida perturbation to affect routing outputs; baseline_qsed={baseline_qsed}, perturbed_qsed={perturbed_qsed}, baseline_tc={baseline_tc}, perturbed_tc={perturbed_tc}"
    );
}

#[test]
fn wshedimpl29_contract_ws20_rectangular_widb_mutation_projects_to_state() {
    let channel_only_topology = r"
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
";

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
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ishape"),
        BoundaryValue::scalar(2.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_widb_0001"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_widb_0002"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_widb_0003"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_wida_0002"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_wida_0003"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_chnk"),
        BoundaryValue::scalar(100.0),
    );
    for hillslope_id in [1_u32, 2, 3] {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_peakro")),
            BoundaryValue::scalar(200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_watdur")),
            BoundaryValue::scalar(300.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_total_detachment_kg")),
            BoundaryValue::scalar(0.001),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_total_deposition_kg")),
            BoundaryValue::scalar(0.0),
        );
    }
    seed_ws22_channel_crfrac(&mut surface, 1);

    let report = run_ws11_surface_with_topology(surface, channel_only_topology);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl29 rectangular widb mutation vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let updated_widb_0001 = state_value(&report, "ws10_channel_1_widb_0001");
    let updated_widb_0002 = state_value(&report, "ws10_channel_1_widb_0002");
    assert!(
        updated_widb_0001 > 1.0 + 1.0e-9 || updated_widb_0002 > 1.0 + 1.0e-9,
        "expected ws29 widb mutation to widen at least one rectangular upper-boundary width; widb_0001={updated_widb_0001}, widb_0002={updated_widb_0002}"
    );
}

#[test]
fn wshedimpl30_contract_ws20_ishape3_erodible_lane_vector_executes() {
    let channel_only_topology = r"
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
";

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
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ishape"),
        BoundaryValue::scalar(3.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_chnk"),
        BoundaryValue::scalar(100.0),
    );
    seed_ws22_channel_crfrac(&mut surface, 1);

    let report = run_ws11_surface_with_topology(surface, channel_only_topology);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl30 erodible-lane vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let qsed = state_value(&report, "ws10_channel_1_qsed");
    let tc = state_value(&report, "ws10_channel_1_tc");
    assert!(qsed.is_finite() && qsed >= 0.0);
    assert!(tc.is_finite() && tc >= 0.0);
}

#[test]
#[allow(clippy::too_many_lines)]
fn wshedimpl30_contract_ws20_ishape3_depa_depb_fallback_mapping_affects_outputs() {
    let channel_only_topology = r"
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
";

    let mut no_fallback_surface = seeded_ws11_surface();
    no_fallback_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    no_fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws20_case12_enable"),
        BoundaryValue::scalar(1.0),
    );
    no_fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws21_case34_enable"),
        BoundaryValue::scalar(1.0),
    );
    no_fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ishape"),
        BoundaryValue::scalar(3.0),
    );
    no_fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_chnk"),
        BoundaryValue::scalar(100.0),
    );
    no_fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_widb_0001"),
        BoundaryValue::scalar(1.0),
    );
    no_fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_wida_0002"),
        BoundaryValue::scalar(1.0),
    );
    no_fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_depb_0001"),
        BoundaryValue::scalar(0.00011),
    );
    no_fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_depa_0002"),
        BoundaryValue::scalar(0.00011),
    );
    seed_ws22_channel_crfrac(&mut no_fallback_surface, 1);

    let no_fallback_report =
        run_ws11_surface_with_topology(no_fallback_surface, channel_only_topology);
    assert!(
        no_fallback_report.dispatch_report.is_success(),
        "wshedimpl30 no-fallback vector must succeed; step_reports={:?}",
        no_fallback_report.step_reports
    );

    let no_fallback_qsed = state_value(&no_fallback_report, "ws10_channel_1_qsed");
    let no_fallback_tc = state_value(&no_fallback_report, "ws10_channel_1_tc");

    let mut fallback_surface = seeded_ws11_surface();
    fallback_surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws20_case12_enable"),
        BoundaryValue::scalar(1.0),
    );
    fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ws21_case34_enable"),
        BoundaryValue::scalar(1.0),
    );
    fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ishape"),
        BoundaryValue::scalar(3.0),
    );
    fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_chnk"),
        BoundaryValue::scalar(100.0),
    );
    fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_widb_0001"),
        BoundaryValue::scalar(1.0),
    );
    fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_wida_0002"),
        BoundaryValue::scalar(1.0),
    );
    fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_depb_0001"),
        BoundaryValue::scalar(0.0),
    );
    fallback_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_depa_0002"),
        BoundaryValue::scalar(0.0),
    );
    seed_ws22_channel_crfrac(&mut fallback_surface, 1);

    let fallback_report = run_ws11_surface_with_topology(fallback_surface, channel_only_topology);
    assert!(
        fallback_report.dispatch_report.is_success(),
        "wshedimpl30 fallback vector must succeed; step_reports={:?}",
        fallback_report.step_reports
    );

    let fallback_qsed = state_value(&fallback_report, "ws10_channel_1_qsed");
    let fallback_tc = state_value(&fallback_report, "ws10_channel_1_tc");

    let qsed_shift = (no_fallback_qsed - fallback_qsed).abs();
    let tc_shift = (no_fallback_tc - fallback_tc).abs();
    assert!(
        qsed_shift > 1.0e-9 || tc_shift > 1.0e-9,
        "expected depa/depb-driven rectangular fallback mapping to alter erodible-lane outputs; no_fallback_qsed={no_fallback_qsed}, fallback_qsed={fallback_qsed}, no_fallback_tc={no_fallback_tc}, fallback_tc={fallback_tc}"
    );
}

#[test]
fn wshedimpl31_contract_ws24_rectangular_detach_wida_mutation_projects_to_state() {
    let channel_only_topology = r"
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
";

    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    seed_ws24_case12_transition_forcing(&mut surface);
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ishape"),
        BoundaryValue::scalar(2.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_wida_0002"),
        BoundaryValue::scalar(0.01),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_wida_0003"),
        BoundaryValue::scalar(0.01),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_widb_0001"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_widb_0002"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_widb_0003"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_chnk"),
        BoundaryValue::scalar(1_000_000.0),
    );
    for hillslope_id in [1_u32, 2, 3] {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_peakro")),
            BoundaryValue::scalar(200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_watdur")),
            BoundaryValue::scalar(300.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_total_detachment_kg")),
            BoundaryValue::scalar(100.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_total_deposition_kg")),
            BoundaryValue::scalar(0.0),
        );
    }
    seed_ws22_channel_crfrac(&mut surface, 1);

    let report = run_ws11_surface_with_topology(surface, channel_only_topology);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl31 lower-boundary width mutation vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let ws24_segments = state_value(&report, "ws10_channel_1_ws24_case2_detach_segment_count");
    let updated_wida_0002 = state_value(&report, "ws10_channel_1_wida_0002");
    let updated_wida_0003 = state_value(&report, "ws10_channel_1_wida_0003");

    assert!(
        ws24_segments > 0.0,
        "expected WS24 transition activity for lower-boundary detach mutation vector"
    );
    assert!(
        updated_wida_0002 > 0.01 + 1.0e-9 || updated_wida_0003 > 0.01 + 1.0e-9,
        "expected ws31 lower-boundary detach semantics to widen at least one rectangular wida point; wida_0002={updated_wida_0002}, wida_0003={updated_wida_0003}"
    );
}

#[test]
fn wshedimpl31_contract_non_rectangular_lane_does_not_apply_wida_mutation() {
    let channel_only_topology = r"
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
";

    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    seed_ws24_case12_transition_forcing(&mut surface);
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_ishape"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_wida_0002"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_wida_0003"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_1_chnk"),
        BoundaryValue::scalar(1_000_000.0),
    );
    for hillslope_id in [1_u32, 2, 3] {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_peakro")),
            BoundaryValue::scalar(200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_watdur")),
            BoundaryValue::scalar(300.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_total_detachment_kg")),
            BoundaryValue::scalar(100.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_total_deposition_kg")),
            BoundaryValue::scalar(0.0),
        );
    }
    seed_ws22_channel_crfrac(&mut surface, 1);

    let report = run_ws11_surface_with_topology(surface, channel_only_topology);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl31 non-rectangular control vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let updated_wida_0002 = state_value(&report, "ws10_channel_1_wida_0002");
    let updated_wida_0003 = state_value(&report, "ws10_channel_1_wida_0003");
    assert!((updated_wida_0002 - 1.0).abs() <= 1.0e-9);
    assert!((updated_wida_0003 - 1.0).abs() <= 1.0e-9);
}

#[test]
fn wshedimpl24_contract_case12_transition_requires_crfrac_projection() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    seed_ws24_case12_transition_forcing(&mut surface);

    let report = run_ws11_surface(surface);
    assert!(
        report
            .step_reports
            .iter()
            .any(|step| step.decision_status.message_id() == "WKERNEL-WS10-CHANNEL-E-001"),
        "expected missing crfrac projection failure; step_reports={:?}",
        report.step_reports
    );
}

#[test]
fn wshedimpl24_contract_case12_transition_routes_with_crfrac_projection() {
    let mut surface = seeded_ws11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("ipeak"), BoundaryValue::scalar(4.0));
    seed_ws24_case12_transition_forcing(&mut surface);
    seed_ws22_channel_crfrac(&mut surface, 1);

    let report = run_ws11_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "wshedimpl24 transition vector must succeed; step_reports={:?}",
        report.step_reports
    );

    let case2_segments = state_value(&report, "ws10_channel_1_ws20_case2_segment_count");
    let case1_segments = state_value(&report, "ws10_channel_1_ws20_case1_segment_count");
    let ws24_transition_segments =
        state_value(&report, "ws10_channel_1_ws24_case2_detach_segment_count");
    let ws21_case3_segments = state_value(&report, "ws10_channel_1_ws21_case3_segment_count");
    let ws21_case4_segments = state_value(&report, "ws10_channel_1_ws21_case4_segment_count");

    assert!(
        case2_segments > 0.0,
        "expected case2 activity for ws24 transition forcing vector, got case2={case2_segments}, case1={case1_segments}, ws24={ws24_transition_segments}, ws21_case3={ws21_case3_segments}, ws21_case4={ws21_case4_segments}"
    );
    assert!(
        ws24_transition_segments > 0.0,
        "expected ws24 transition diagnostics activity, got {ws24_transition_segments}"
    );
    assert!(!has_state_symbol(
        &report,
        "ws10_channel_1_ws20_detachment_unmigrated_segment_count"
    ));
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
