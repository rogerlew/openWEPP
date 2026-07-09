#![allow(clippy::many_single_char_names, clippy::too_many_lines)]

use std::collections::BTreeSet;

use openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload;
use openwepp_input_contract::parsers::{
    chaninp::{ChaninpParseOptions, parse_chaninp_from_str},
    slope::{SlopeParserOptions, parse_slope_str},
    watershed_channel::{WatershedChannelParseOptions, parse_watershed_channel_from_str},
    watershed_impoundment::{
        WatershedImpoundmentParseOptions, parse_watershed_impoundment_from_str,
    },
};
use openwepp_sim_contract::status::{BoundaryClass, StatusClassification};
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};
use openwepp_watershed_orchestrator::{
    HillslopeContribution, WatershedGroundwaterRoutingAuthority, WatershedNetworkFrame,
    execute_watershed_dispatch_with_frame,
};

const TYPED_TOPOLOGY: &str = r"
HILLSLOPES 2
CHANNELS 1
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 2 0 0 C 1 0 0 I 0 0 0
";

const STRICT_VALID_CHANINP: &str = include_str!("../fixtures/infile/chaninp/strict_valid.chaninp");
const STRICT_VALID_SLOPE: &str =
    include_str!("../fixtures/infile/slope/strict_valid_canonical.slp");
const STRICT_VALID_WATERSHED_CHANNEL: &str =
    include_str!("../fixtures/infile/watershed_channel/strict_valid_single_channel.chn");
const STRICT_VALID_WATERSHED_IMPOUNDMENT: &str =
    include_str!("../fixtures/infile/watershed_impoundment/strict_valid_minimal.imp");
const DROP_SPILLWAY_IMPOUNDMENT: &str = r"
95.700
1
drop impoundment desc line 1
drop impoundment desc line 2
drop impoundment desc line 3
1
drop spillway structure
0.60 0.50 3.20 0.60
0.45 0.30 2.00 0.01 0.10
0.50 0.20 0.30
0 0
0 0
0
0
0
0
1.20 1.00 0.80 0.5 0.01
1 5
3
0.80 120.0 24.0
0.85 0.95 1.05
130.0 150.0 175.0
26.0 30.0 35.0
";

fn contribution(
    hillslope_id: u32,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
) -> HillslopeContribution {
    contribution_with_diameters(
        hillslope_id,
        peak_runoff_m3_s,
        duration_seconds,
        vec![0.000_01, 0.000_02, 0.000_03],
    )
}

fn contribution_with_diameters(
    hillslope_id: u32,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    particle_diameter_m: Vec<f64>,
) -> HillslopeContribution {
    HillslopeContribution {
        hillslope_id,
        area_m2: Some(1_800.0),
        peak_runoff_m3_s,
        duration_seconds,
        generated_baseflow_m3: 0.0,
        groundwater_deep_seepage_m3: 0.0,
        total_detachment_kg: peak_runoff_m3_s * duration_seconds * 0.01,
        total_deposition_kg: peak_runoff_m3_s * duration_seconds * 0.0025,
        sediment_concentration_kg_m3: vec![0.35, 0.45, 0.55],
        particle_diameter_m,
        particle_flow_fraction: vec![0.2, 0.3, 0.5],
        hourly_runoff_volume_m3: Vec::new(),
        hourly_sediment_mass_kg: Vec::new(),
    }
}

fn contribution_with_groundwater(
    hillslope_id: u32,
    generated_baseflow_m3: f64,
    groundwater_deep_seepage_m3: f64,
) -> HillslopeContribution {
    HillslopeContribution {
        generated_baseflow_m3,
        groundwater_deep_seepage_m3,
        ..contribution(hillslope_id, 1.0, 300.0)
    }
}

fn build_typed_frame() -> WatershedNetworkFrame {
    build_typed_frame_with_impoundment(STRICT_VALID_WATERSHED_IMPOUNDMENT, true)
}

fn build_typed_frame_with_impoundment(
    impoundment_fixture: &str,
    clamp_impoundment_stage: bool,
) -> WatershedNetworkFrame {
    let graph = parse_topology_fixture_str(TYPED_TOPOLOGY).expect("typed topology should parse");
    let valid_channel_element_ids = BTreeSet::from([4_i32, 5_i32]);
    let chaninp = parse_chaninp_from_str(
        STRICT_VALID_CHANINP,
        ChaninpParseOptions::strict(3, 2),
        &valid_channel_element_ids,
    )
    .expect("strict chan.inp fixture should parse");
    let slope = parse_slope_str(STRICT_VALID_SLOPE, SlopeParserOptions::strict())
        .expect("strict slope fixture should parse");
    let channel = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    let impoundment = parse_watershed_impoundment_from_str(
        impoundment_fixture,
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect("strict watershed impoundment fixture should parse");

    let mut frame = WatershedNetworkFrame::from_parsed_inputs(
        graph,
        Some(chaninp),
        channel,
        slope,
        impoundment,
        3600.0,
        24.0,
    )
    .expect("typed watershed frame should build");
    if clamp_impoundment_stage {
        let impoundment = frame
            .impoundment_controls
            .get_mut(&1)
            .expect("fixture should contain impoundment 1");
        impoundment.h = impoundment.h.min(impoundment.hfull);
    }
    frame
}

fn run_successful_frame(frame: &mut WatershedNetworkFrame) {
    frame.add_hillslope_contribution(contribution(1, 2.0, 300.0));
    frame.add_hillslope_contribution(contribution(2, 1.5, 400.0));
    let topology_report =
        validate_pre_execution_topology(frame.topology()).expect("topology gate should build");
    let report = execute_watershed_dispatch_with_frame(frame, &topology_report)
        .expect("typed watershed dispatch should execute");
    assert!(report.dispatch_report.is_success());
}

#[test]
fn typed_frame_dispatch_records_and_publishes_direct_routed_state() {
    let mut frame = build_typed_frame();
    frame.add_hillslope_contribution(contribution(1, 2.0, 300.0));
    frame.add_hillslope_contribution(contribution(2, 1.5, 400.0));

    let topology_report =
        validate_pre_execution_topology(frame.topology()).expect("topology gate should build");
    assert!(topology_report.is_valid(), "typed topology should be valid");
    let report = execute_watershed_dispatch_with_frame(&mut frame, &topology_report)
        .expect("typed watershed dispatch should execute");

    assert!(report.dispatch_report.is_success());
    assert_eq!(report.step_reports.len(), 2);
    assert!(
        report
            .step_reports
            .iter()
            .all(|step| step.routed_state_applied),
        "typed dispatch should write routed state to the frame"
    );

    let channel = frame
        .routed_channels
        .get(&1)
        .expect("typed dispatch should route channel 1");
    assert!(channel.runoff_volume_m3 > 0.0);
    assert!((channel.runoff_volume_m3 - channel.channel_outflow_m3).abs() <= 1.0e-12);
    assert!(channel.channel_inflow_m3 >= 0.0);
    assert!(channel.channel_storage_m3 >= 0.0);
    assert!(channel.channel_loss_m3 >= 0.0);
    assert!(channel.peak_discharge_m3_s >= 0.0);
    assert_eq!(channel.sediment_state.particle_flow_fraction.len(), 3);
    let channel_inflow_m3 = channel.channel_inflow_m3;
    let channel_outflow_m3 = channel.channel_outflow_m3;
    let channel_storage_m3 = channel.channel_storage_m3;
    let channel_baseflow_m3 = channel.channel_baseflow_m3;
    let channel_loss_m3 = channel.channel_loss_m3;

    let impoundment = frame
        .routed_impoundments
        .get(&1)
        .expect("typed dispatch should route impoundment 1");
    assert!(impoundment.outflow_volume_m3.abs() <= 1.0e-12);
    assert!(impoundment.outflow_rate_m3_s.abs() <= 1.0e-12);
    assert!(impoundment.hnext_m.is_finite());

    let publication = frame
        .publish_typed_routing_report(&report)
        .expect("typed routed state should publish");
    assert_eq!(publication.channel_id, 1);
    assert!(publication.runoff_volume_m3 > 0.0);
    assert_eq!(publication.channel_inflow_m3, Some(channel_inflow_m3));
    assert_eq!(publication.channel_outflow_m3, Some(channel_outflow_m3));
    assert_eq!(publication.channel_storage_m3, Some(channel_storage_m3));
    assert_eq!(publication.channel_baseflow_m3, Some(channel_baseflow_m3));
    assert_eq!(publication.channel_loss_m3, Some(channel_loss_m3));
    assert_eq!(publication.particulate_pollutant_kg, None);
    assert!(publication.total_detachment_kg > 0.0);
    assert!(publication.total_deposition_kg > 0.0);
}

#[test]
fn typed_publication_projects_non_aliased_channel_balance_operands() {
    let mut selected = None;

    for ipeak in [3, 4, 5] {
        let mut frame = build_typed_frame();
        frame.routing_globals.ipeak = ipeak;
        frame.add_hillslope_contribution(contribution(1, 2.0, 300.0));
        frame.add_hillslope_contribution(contribution(2, 1.5, 400.0));

        let topology_report =
            validate_pre_execution_topology(frame.topology()).expect("topology gate should build");
        let report = execute_watershed_dispatch_with_frame(&mut frame, &topology_report)
            .expect("typed watershed dispatch should execute");
        assert!(report.dispatch_report.is_success());

        let channel = frame
            .routed_channels
            .get(&1)
            .unwrap_or_else(|| panic!("ipeak={ipeak} should route channel 1"))
            .clone();
        if (channel.channel_inflow_m3 - channel.channel_outflow_m3).abs() > 1.0e-9 {
            selected = Some((ipeak, frame, report, channel));
            break;
        }
    }

    let (ipeak, mut frame, report, channel) =
        selected.expect("at least one WS11 wave branch should produce non-aliased channel volumes");
    let publication = frame
        .publish_typed_routing_report(&report)
        .unwrap_or_else(|_| panic!("ipeak={ipeak} typed routed state should publish"));

    assert!(
        (publication
            .channel_inflow_m3
            .expect("inflow should publish")
            - channel.channel_inflow_m3)
            .abs()
            <= 1.0e-9
    );
    assert!(
        (publication
            .channel_outflow_m3
            .expect("outflow should publish")
            - channel.channel_outflow_m3)
            .abs()
            <= 1.0e-9
    );
    assert!(
        (publication
            .channel_storage_m3
            .expect("storage should publish")
            - channel.channel_storage_m3)
            .abs()
            <= 1.0e-9
    );
    assert!(
        (publication
            .channel_baseflow_m3
            .expect("baseflow should publish")
            - channel.channel_baseflow_m3)
            .abs()
            <= 1.0e-9
    );
    assert!(
        (publication.channel_loss_m3.expect("loss should publish") - channel.channel_loss_m3).abs()
            <= 1.0e-9
    );
    assert!(
        (publication.runoff_volume_m3 - channel.channel_outflow_m3).abs() <= 1.0e-9,
        "runoff publication remains routed channel outflow, not upstream inflow"
    );
}

#[test]
fn gwbaseflow_lr_bf1_channel_branch_consumes_generated_hbp_not_cbase() {
    let mut with_cbase = build_typed_frame();
    with_cbase.routing_globals.cbase = 99.0;
    with_cbase.configure_groundwater_baseflow_routing(
        WatershedGroundwaterRoutingAuthority::linear_reservoir(0.10)
            .expect("valid threshold should construct"),
    );
    with_cbase.add_hillslope_contribution(contribution_with_groundwater(1, 24.0, 5.0));
    with_cbase.add_hillslope_contribution(contribution(2, 0.5, 300.0));
    let topology_report =
        validate_pre_execution_topology(with_cbase.topology()).expect("topology gate should build");
    let report = execute_watershed_dispatch_with_frame(&mut with_cbase, &topology_report)
        .expect("generated-baseflow dispatch should execute");
    assert!(report.dispatch_report.is_success());
    let channel = with_cbase
        .routed_channels
        .get(&1)
        .expect("channel should route");
    assert!((channel.channel_baseflow_m3 - 24.0).abs() <= 1.0e-9);
    assert!((channel.groundwater_deep_seepage_m3 - 5.0).abs() <= 1.0e-9);

    let mut without_cbase = build_typed_frame();
    without_cbase.routing_globals.cbase = 0.0;
    without_cbase.configure_groundwater_baseflow_routing(
        WatershedGroundwaterRoutingAuthority::linear_reservoir(0.10)
            .expect("valid threshold should construct"),
    );
    without_cbase.add_hillslope_contribution(contribution_with_groundwater(1, 24.0, 5.0));
    without_cbase.add_hillslope_contribution(contribution(2, 0.5, 300.0));
    let topology_report = validate_pre_execution_topology(without_cbase.topology())
        .expect("topology gate should build");
    let report = execute_watershed_dispatch_with_frame(&mut without_cbase, &topology_report)
        .expect("generated-baseflow dispatch should execute");
    assert!(report.dispatch_report.is_success());
    let no_cbase_channel = without_cbase
        .routed_channels
        .get(&1)
        .expect("channel should route");
    assert!((no_cbase_channel.channel_baseflow_m3 - 24.0).abs() <= 1.0e-9);
    assert!(
        (channel.runoff_volume_m3 - no_cbase_channel.runoff_volume_m3).abs() <= 1.0e-9,
        "lr_bf=1 must not substitute cbase into the generated-baseflow branch"
    );
}

#[test]
fn gwbaseflow_bftharea_suppresses_below_threshold_side_baseflow() {
    let mut frame = build_typed_frame();
    frame.configure_groundwater_baseflow_routing(
        WatershedGroundwaterRoutingAuthority::linear_reservoir(1.0)
            .expect("valid threshold should construct"),
    );
    frame.add_hillslope_contribution(contribution_with_groundwater(1, 24.0, 5.0));
    frame.add_hillslope_contribution(contribution(2, 0.5, 300.0));
    let topology_report =
        validate_pre_execution_topology(frame.topology()).expect("topology gate should build");
    let report = execute_watershed_dispatch_with_frame(&mut frame, &topology_report)
        .expect("threshold dispatch should execute");
    assert!(report.dispatch_report.is_success());
    let channel = frame.routed_channels.get(&1).expect("channel should route");
    assert!(channel.channel_baseflow_m3.abs() <= 1.0e-12);
    assert!((channel.groundwater_deep_seepage_m3 - 5.0).abs() <= 1.0e-9);
}

#[test]
fn gwbaseflow_generated_hbp_payload_without_gwcoeff_authority_fails_closed() {
    let mut frame = build_typed_frame();
    frame.add_hillslope_contribution(contribution_with_groundwater(1, 24.0, 5.0));
    frame.add_hillslope_contribution(contribution(2, 0.5, 300.0));
    let topology_report =
        validate_pre_execution_topology(frame.topology()).expect("topology gate should build");
    let report = execute_watershed_dispatch_with_frame(&mut frame, &topology_report)
        .expect("typed dispatch should report guard failure, not panic");
    assert!(
        !report.dispatch_report.is_success(),
        "generated groundwater payloads require gwcoeff authority"
    );
}

#[test]
fn typed_frame_dispatch_executes_ws11_ipeak_branches_with_closure() {
    let mut qpo_by_ipeak = Vec::new();
    for ipeak in [1, 2, 3, 4, 5] {
        let mut frame = build_typed_frame();
        frame.routing_globals.ipeak = ipeak;
        run_successful_frame(&mut frame);

        let channel = frame
            .routed_channels
            .get(&1)
            .unwrap_or_else(|| panic!("ipeak={ipeak} should route channel 1"));
        assert!(
            channel.peak_discharge_m3_s.is_finite() && channel.peak_discharge_m3_s >= 0.0,
            "ipeak={ipeak} invalid qpo"
        );
        assert!(
            channel.duration_seconds.is_finite() && channel.duration_seconds >= 0.0,
            "ipeak={ipeak} invalid durrof"
        );
        assert!(
            channel.runoff_volume_m3.is_finite() && channel.runoff_volume_m3 >= 0.0,
            "ipeak={ipeak} invalid roff"
        );
        if ipeak >= 3 {
            assert!(
                (channel.runoff_volume_m3
                    - (channel.peak_discharge_m3_s * channel.duration_seconds))
                    .abs()
                    <= 1.0e-9,
                "ipeak={ipeak} violates routed closure roff=qpo*durrof"
            );
        }
        qpo_by_ipeak.push((ipeak, channel.peak_discharge_m3_s));
    }

    let qpo_ipeak_1 = qpo_by_ipeak
        .iter()
        .find_map(|(ipeak, qpo)| (*ipeak == 1).then_some(*qpo))
        .expect("ipeak=1 result should exist");
    let qpo_ipeak_4 = qpo_by_ipeak
        .iter()
        .find_map(|(ipeak, qpo)| (*ipeak == 4).then_some(*qpo))
        .expect("ipeak=4 result should exist");
    assert!(
        (qpo_ipeak_1 - qpo_ipeak_4).abs() > 1.0e-9,
        "explicit WS11 ipeak branch outputs should differ"
    );
}

#[test]
fn typed_frame_channel_transport_capacity_responds_to_particle_diameter() {
    let mut baseline = build_typed_frame();
    baseline.add_hillslope_contribution(contribution(1, 2.0, 300.0));
    baseline.add_hillslope_contribution(contribution(2, 1.5, 400.0));
    let topology_report =
        validate_pre_execution_topology(baseline.topology()).expect("topology gate should build");
    let baseline_report = execute_watershed_dispatch_with_frame(&mut baseline, &topology_report)
        .expect("baseline typed watershed dispatch should execute");
    assert!(baseline_report.dispatch_report.is_success());
    let baseline_channel = baseline
        .routed_channels
        .get(&1)
        .expect("baseline should route channel 1");

    let mut perturbed = build_typed_frame();
    perturbed.add_hillslope_contribution(contribution_with_diameters(
        1,
        2.0,
        300.0,
        vec![0.000_50, 0.000_75, 0.001_00],
    ));
    perturbed.add_hillslope_contribution(contribution_with_diameters(
        2,
        1.5,
        400.0,
        vec![0.000_50, 0.000_75, 0.001_00],
    ));
    let topology_report =
        validate_pre_execution_topology(perturbed.topology()).expect("topology gate should build");
    let perturbed_report = execute_watershed_dispatch_with_frame(&mut perturbed, &topology_report)
        .expect("perturbed typed watershed dispatch should execute");
    assert!(perturbed_report.dispatch_report.is_success());
    let perturbed_channel = perturbed
        .routed_channels
        .get(&1)
        .expect("perturbed should route channel 1");

    assert!(
        (baseline_channel.sediment_state.qsed_kg_s - perturbed_channel.sediment_state.qsed_kg_s)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (baseline_channel.sediment_state.transport_capacity_kg_s
            - perturbed_channel.sediment_state.transport_capacity_kg_s)
            .abs()
            > 1.0e-9,
        "transport capacity must not collapse to sediment-load identity"
    );
}

#[test]
fn typed_frame_active_impoundment_matches_drop_spillway_min_controller_composition() {
    let mut frame = build_typed_frame_with_impoundment(DROP_SPILLWAY_IMPOUNDMENT, false);
    let control = frame
        .impoundment_controls
        .get_mut(&1)
        .expect("drop-spillway control should exist");
    control.hfull = 10.0;
    control.source_record.hfull = 10.0;
    run_successful_frame(&mut frame);

    let impoundment = frame
        .routed_impoundments
        .get(&1)
        .expect("drop-spillway impoundment should route");
    let record = &frame
        .impoundment_controls
        .get(&1)
        .expect("drop-spillway control should exist")
        .source_record;
    let expected_qo = expected_drop_spillway_outflow(record, impoundment.hnext_m);

    assert!(impoundment.outflow_rate_m3_s > 0.0);
    assert!(
        (impoundment.outflow_rate_m3_s - expected_qo).abs() <= 1.0e-9,
        "typed WS12 outflow should match 15-family min-controller composition"
    );
}

#[test]
fn typed_frame_impoundment_projection_preserves_non_finite_guard_class() {
    let mut frame = build_typed_frame();
    frame.add_hillslope_contribution(contribution(1, 2.0, 300.0));
    frame.add_hillslope_contribution(contribution(2, 1.5, 400.0));
    frame
        .impoundment_controls
        .get_mut(&1)
        .expect("fixture should contain impoundment 1")
        .source_record
        .stage[0] = f64::NAN;

    let topology_report =
        validate_pre_execution_topology(frame.topology()).expect("topology gate should build");
    let report = execute_watershed_dispatch_with_frame(&mut frame, &topology_report)
        .expect("typed watershed dispatch should return a failure report");

    assert_eq!(
        report.dispatch_report.dispatch_status.classification(),
        StatusClassification::Failure
    );
    let failing_step = report
        .step_reports
        .last()
        .expect("impoundment projection failure should be reported after channel routing");
    assert_eq!(
        failing_step.kernel_status.boundary_class(),
        BoundaryClass::NonFinite
    );
    assert_eq!(
        failing_step.kernel_status.message_id(),
        "WKERNEL-WS10-IMPOUNDMENT-E-002"
    );
}

#[test]
fn typed_frame_impoundment_projection_preserves_domain_guard_class() {
    let mut frame = build_typed_frame();
    frame.add_hillslope_contribution(contribution(1, 2.0, 300.0));
    frame.add_hillslope_contribution(contribution(2, 1.5, 400.0));
    frame
        .impoundment_controls
        .get_mut(&1)
        .expect("fixture should contain impoundment 1")
        .source_record
        .stage[0] = 0.0;

    let topology_report =
        validate_pre_execution_topology(frame.topology()).expect("topology gate should build");
    let report = execute_watershed_dispatch_with_frame(&mut frame, &topology_report)
        .expect("typed watershed dispatch should return a failure report");

    assert_eq!(
        report.dispatch_report.dispatch_status.classification(),
        StatusClassification::Failure
    );
    let failing_step = report
        .step_reports
        .last()
        .expect("impoundment projection failure should be reported after channel routing");
    assert_eq!(
        failing_step.kernel_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
    assert_eq!(
        failing_step.kernel_status.message_id(),
        "WKERNEL-WS10-IMPOUNDMENT-E-003"
    );
}

#[test]
fn typed_frame_dispatch_fails_closed_on_non_finite_hillslope_payload() {
    let mut frame = build_typed_frame();
    frame.add_hillslope_contribution(contribution(1, f64::NAN, 300.0));
    frame.add_hillslope_contribution(contribution(2, 1.5, 400.0));

    let topology_report =
        validate_pre_execution_topology(frame.topology()).expect("topology gate should build");
    let report = execute_watershed_dispatch_with_frame(&mut frame, &topology_report)
        .expect("typed watershed dispatch should return a failure report");

    assert_eq!(
        report.dispatch_report.dispatch_status.classification(),
        StatusClassification::Failure
    );
    let failing_step = report
        .step_reports
        .first()
        .expect("failure should be reported on the first channel step");
    assert!(!failing_step.routed_state_applied);
    assert_eq!(
        failing_step.kernel_status.boundary_class(),
        BoundaryClass::NonFinite
    );
    assert_eq!(
        failing_step.kernel_status.message_id(),
        "WKERNEL-WS10-CHANNEL-E-002"
    );
    assert!(
        frame.routed_channels.is_empty(),
        "non-finite payload should not publish partial channel state"
    );
}

#[test]
fn typed_frame_dispatch_fails_closed_on_impoundment_domain_violation() {
    let mut frame = build_typed_frame();
    frame.add_hillslope_contribution(contribution(1, 2.0, 300.0));
    frame.add_hillslope_contribution(contribution(2, 1.5, 400.0));
    let impoundment = frame
        .impoundment_controls
        .get_mut(&1)
        .expect("fixture should contain impoundment 1");
    impoundment.h = impoundment.hfull + 0.1;

    let topology_report =
        validate_pre_execution_topology(frame.topology()).expect("topology gate should build");
    let report = execute_watershed_dispatch_with_frame(&mut frame, &topology_report)
        .expect("typed watershed dispatch should return a failure report");

    assert_eq!(
        report.dispatch_report.dispatch_status.classification(),
        StatusClassification::Failure
    );
    let failing_step = report
        .step_reports
        .last()
        .expect("impoundment failure should be reported after channel routing");
    assert!(!failing_step.routed_state_applied);
    assert_eq!(
        failing_step.kernel_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
    assert_eq!(
        failing_step.kernel_status.message_id(),
        "WKERNEL-WS10-IMPOUNDMENT-E-003"
    );
    assert!(
        frame.routed_channels.contains_key(&1),
        "upstream channel state is allowed before downstream failure"
    );
    assert!(
        frame.routed_impoundments.is_empty(),
        "domain-invalid impoundment should not publish routed impoundment state"
    );
}

fn expected_drop_spillway_outflow(
    record: &openwepp_input_contract::parsers::watershed_impoundment::ImpoundmentRecord,
    stage: f64,
) -> f64 {
    let mut a = [0.0_f64; 15];
    let mut b = [0.0_f64; 15];
    let mut c = [0.0_f64; 15];
    let mut ha = [record.hfull; 15];

    let DropSpillwayPayload::Ids1 { payload, .. } = &record.drop_spillway else {
        panic!("drop-spillway fixture should use IDS=1 payload");
    };
    let denominator = 1.0 + payload.ke + payload.kb + payload.kc * (payload.lbl + payload.hrh);

    a[0] = 1.0;
    b[0] = payload.coefw * std::f64::consts::PI * payload.diars;
    c[0] = 1.5;
    ha[0] = payload.hrs;

    a[1] = 1.0;
    b[1] = payload.coefo * std::f64::consts::PI * payload.diars.powi(2) / 4.0
        * (2.0_f64 * 9.806_65).sqrt();
    c[1] = 0.5;
    ha[1] = payload.hrs;

    a[2] = payload.hblot + 0.6 * payload.diabl;
    b[2] = std::f64::consts::PI * payload.diabl.powi(2) / 4.0 * (2.0_f64 * 9.806_65).sqrt()
        / denominator.sqrt();
    c[2] = 0.5;
    ha[2] = payload.hrs - (payload.hrh + payload.sbl * payload.lbl - 0.6 * payload.diabl);

    let q1 = if stage > ha[0] {
        b[0] * (stage - ha[0]).powf(c[0])
    } else {
        0.0
    };
    let q2 = if stage > ha[1] {
        b[1] * (stage - ha[1]).powf(c[1])
    } else {
        0.0
    };
    let q3 = if stage > ha[2] {
        let htw = 0.0;
        let head = if htw > a[2] {
            stage - (ha[2] + htw - a[2])
        } else {
            stage - ha[2]
        };
        if head > 0.0 {
            b[2] * head.powf(c[2])
        } else {
            0.0
        }
    } else {
        0.0
    };

    q1.min(q2).min(q3)
}
