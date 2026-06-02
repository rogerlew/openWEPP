use std::collections::BTreeMap;

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
};
use openwepp_sim_contract::status::BoundaryClass;

const TOL: f64 = 1.0e-9;

#[allow(clippy::too_many_lines)]
fn seeded_wb19_state_surface() -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state = BTreeMap::new();

    state.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.2));
    state.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2006.0),
    );

    state.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.1));
    state.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.1));
    state.insert(
        BoundarySymbol::from("por_0001"),
        BoundaryValue::scalar(0.95),
    );
    state.insert(
        BoundarySymbol::from("por_0002"),
        BoundaryValue::scalar(0.95),
    );
    state.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));
    state.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(BoundarySymbol::from("cpm_0002"), BoundaryValue::scalar(1.0));
    state.insert(
        BoundarySymbol::from("coca_0002"),
        BoundaryValue::scalar(1.0),
    );

    state.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(4.5),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(6.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(4.0),
    );
    state.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(40.0),
    );
    state.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(4.0),
    );
    state.insert(
        BoundarySymbol::from("thetfc_0002"),
        BoundaryValue::scalar(40.0),
    );
    state.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(8.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ul_0002"),
        BoundaryValue::scalar(8.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(1.0e-5),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ssc_0002"),
        BoundaryValue::scalar(1.0e-5),
    );

    state.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(1.0));
    state.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(1.0));
    state.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(1.0),
    );

    state.insert(
        BoundarySymbol::from("wb19_drain_enabled"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("wb19_drain_depth"),
        BoundaryValue::scalar(0.15),
    );
    state.insert(
        BoundarySymbol::from("wb19_drain_spacing"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("wb19_drain_diameter"),
        BoundaryValue::scalar(0.1),
    );

    // Legacy WB11 compatibility surfaces retained with WB19 authority.
    state.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(2.5),
    );
    state.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(10.5),
    );
    state.insert(
        BoundarySymbol::from("wb11_drainage_coefficient"),
        BoundaryValue::scalar(0.1),
    );

    state
}

fn build_lateral_request(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    pe_recharge: f64,
) -> HillslopeKernelRequest<'_> {
    let flux_surface = Box::leak(Box::new(BTreeMap::from([(
        BoundarySymbol::from("Pe"),
        BoundaryValue::scalar(pe_recharge),
    )])));

    HillslopeKernelRequest::with_transition_context(
        "lateral_transfer",
        HillslopeKernelPhaseClass::HydrologyLateralTransfer,
        HillslopeConsumerAdapter::Watbal,
        None,
        None,
        state_surface,
        flux_surface,
    )
}

fn build_lateral_request_with_qdd(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    pe_recharge: f64,
    q_drainage: f64,
) -> HillslopeKernelRequest<'_> {
    let flux_surface = Box::leak(Box::new(BTreeMap::from([
        (
            BoundarySymbol::from("Pe"),
            BoundaryValue::scalar(pe_recharge),
        ),
        (
            BoundarySymbol::from("Qdd"),
            BoundaryValue::scalar(q_drainage),
        ),
    ])));

    HillslopeKernelRequest::with_transition_context(
        "lateral_transfer",
        HillslopeKernelPhaseClass::HydrologyLateralTransfer,
        HillslopeConsumerAdapter::Watbal,
        None,
        None,
        state_surface,
        flux_surface,
    )
}

fn build_drainage_request(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    q_lateral: f64,
) -> HillslopeKernelRequest<'_> {
    let flux_surface = Box::leak(Box::new(BTreeMap::from([(
        BoundarySymbol::from("q"),
        BoundaryValue::scalar(q_lateral),
    )])));

    HillslopeKernelRequest::with_transition_context(
        "drainage",
        HillslopeKernelPhaseClass::HydrologyDrainage,
        HillslopeConsumerAdapter::Perc,
        None,
        None,
        state_surface,
        flux_surface,
    )
}

fn build_drainage_request_without_q(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> HillslopeKernelRequest<'_> {
    let flux_surface = Box::leak(Box::new(BTreeMap::new()));

    HillslopeKernelRequest::with_transition_context(
        "drainage",
        HillslopeKernelPhaseClass::HydrologyDrainage,
        HillslopeConsumerAdapter::Perc,
        None,
        None,
        state_surface,
        flux_surface,
    )
}

fn writeback_state_value(response: &KernelRunResponse, symbol: &str) -> f64 {
    response
        .writeback
        .state_updates
        .iter()
        .find(|field| field.symbol == BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing state writeback symbol {symbol}"))
        .value
        .as_f64()
}

fn apply_state_writebacks(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    response: &KernelRunResponse,
) {
    for field in &response.writeback.state_updates {
        state_surface.insert(field.symbol.clone(), field.value);
    }
}

fn writeback_flux_value(response: &KernelRunResponse, symbol: &str) -> f64 {
    response
        .writeback
        .flux_updates
        .iter()
        .find(|field| field.symbol == BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing flux writeback symbol {symbol}"))
        .value
        .as_f64()
}

#[test]
fn wb19_contract_conformance_emits_layer_aware_lateral_and_drainage_fluxes() {
    let mut kernel = Wb11HydrologyKernel;
    let state_surface = seeded_wb19_state_surface();

    let lateral_response = kernel.run_hillslope_phase(&build_lateral_request(&state_surface, 0.0));
    assert_eq!(
        lateral_response.status.message_id(),
        "HKERNEL-WB11-LAT-OK-001"
    );

    let q_lateral = writeback_flux_value(&lateral_response, "q");
    let theta1_after_lateral = writeback_state_value(&lateral_response, "wb18_perc_theta_0001");
    let theta2_after_lateral = writeback_state_value(&lateral_response, "wb18_perc_theta_0002");
    let drainable_after_lateral =
        writeback_state_value(&lateral_response, "wb11_drainable_storage");

    assert!(
        (q_lateral - 0.030_547_012_947_258_853).abs() <= TOL,
        "q={q_lateral}"
    );
    assert!(
        (theta1_after_lateral - 4.469_452_987_052_741).abs() <= TOL,
        "theta1_after_lateral={theta1_after_lateral}"
    );
    assert!(
        (theta2_after_lateral - 6.0).abs() <= TOL,
        "theta2_after_lateral={theta2_after_lateral}"
    );
    assert!(
        (drainable_after_lateral - 2.469_452_987_052_741_3).abs() <= TOL,
        "drainable_after_lateral={drainable_after_lateral}"
    );

    let mut drainage_state = state_surface.clone();
    drainage_state.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(theta1_after_lateral),
    );
    drainage_state.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(theta2_after_lateral),
    );
    drainage_state.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(drainable_after_lateral),
    );

    let drainage_response =
        kernel.run_hillslope_phase(&build_drainage_request(&drainage_state, q_lateral));
    assert_eq!(
        drainage_response.status.message_id(),
        "HKERNEL-WB11-DRAIN-OK-001"
    );

    let q_drainage = writeback_flux_value(&drainage_response, "Qdd");
    let q_subhyd = writeback_flux_value(&drainage_response, "Qd");
    let theta2_after_drainage = writeback_state_value(&drainage_response, "wb18_perc_theta_0002");
    let drainable_after_drainage =
        writeback_state_value(&drainage_response, "wb11_drainable_storage");

    assert!((q_drainage - 0.1).abs() <= TOL, "Qdd={q_drainage}");
    assert!(
        (q_subhyd - 0.130_547_012_947_258_84).abs() <= TOL,
        "Qd={q_subhyd}"
    );
    assert!(
        (theta2_after_drainage - 5.9).abs() <= TOL,
        "theta2_after_drainage={theta2_after_drainage}"
    );
    assert!(
        (drainable_after_drainage - 2.369_452_987_052_741).abs() <= TOL,
        "drainable_after_drainage={drainable_after_drainage}"
    );
}

#[test]
fn wb19_contract_conformance_requires_bottom_contiguous_lateral_saturation() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_wb19_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(6.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(3.0),
    );

    let lateral_response = kernel.run_hillslope_phase(&build_lateral_request(&state_surface, 0.0));
    assert_eq!(
        lateral_response.status.message_id(),
        "HKERNEL-WB11-LAT-OK-001"
    );

    let q_lateral = writeback_flux_value(&lateral_response, "q");
    let theta1_after_lateral = writeback_state_value(&lateral_response, "wb18_perc_theta_0001");
    let theta2_after_lateral = writeback_state_value(&lateral_response, "wb18_perc_theta_0002");

    assert!(q_lateral.abs() <= TOL, "q={q_lateral}");
    assert!(
        (theta1_after_lateral - 6.0).abs() <= TOL,
        "theta1_after_lateral={theta1_after_lateral}"
    );
    assert!(
        (theta2_after_lateral - 3.0).abs() <= TOL,
        "theta2_after_lateral={theta2_after_lateral}"
    );
}

#[test]
fn wb19_contract_conformance_applies_fffx_saturation_fraction_to_lateral_conductivity() {
    let mut kernel = Wb11HydrologyKernel;
    let partial_state = seeded_wb19_state_surface();
    let mut full_state = seeded_wb19_state_surface();
    full_state.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(8.0),
    );

    let partial_response = kernel.run_hillslope_phase(&build_lateral_request(&partial_state, 0.0));
    assert_eq!(
        partial_response.status.message_id(),
        "HKERNEL-WB11-LAT-OK-001"
    );
    let full_response = kernel.run_hillslope_phase(&build_lateral_request(&full_state, 0.0));
    assert_eq!(full_response.status.message_id(), "HKERNEL-WB11-LAT-OK-001");

    let q_partial = writeback_flux_value(&partial_response, "q");
    let q_full = writeback_flux_value(&full_response, "q");

    assert!(
        (q_partial - 0.030_547_012_947_258_853).abs() <= TOL,
        "q_partial={q_partial}"
    );
    assert!(
        (q_full - 0.068_730_779_131_332_42).abs() <= TOL,
        "q_full={q_full}"
    );
}

#[test]
fn wb19_contract_conformance_rejects_missing_lateral_symbol() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_wb19_state_surface();
    state_surface.remove(&BoundarySymbol::from("wb19_lateral_anisotropy_ratio"));

    let response = kernel.run_hillslope_phase(&build_lateral_request(&state_surface, 0.0));

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-LAT-E-001");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn wb19_contract_conformance_rejects_non_finite_drainage_symbol() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_wb19_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_spacing"),
        BoundaryValue::scalar(f64::NAN),
    );

    let response = kernel.run_hillslope_phase(&build_drainage_request(&state_surface, 0.0));

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-DRAIN-E-002");
    assert_eq!(response.status.boundary_class(), BoundaryClass::NonFinite);
}

#[test]
fn wb19_contract_conformance_rejects_domain_invalid_drain_enable_flag() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_wb19_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_enabled"),
        BoundaryValue::scalar(2.0),
    );

    let response = kernel.run_hillslope_phase(&build_drainage_request(&state_surface, 0.0));

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-DRAIN-E-003");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn wb19_contract_conformance_hourly_lane_preserves_lateral_flux_on_reference_fixture() {
    let mut kernel = Wb11HydrologyKernel;
    let mut daily_state = seeded_wb19_state_surface();
    daily_state.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(1.0),
    );
    let mut hourly_state = daily_state.clone();
    hourly_state.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(24.0),
    );

    let daily_response = kernel.run_hillslope_phase(&build_lateral_request(&daily_state, 0.0));
    assert_eq!(
        daily_response.status.message_id(),
        "HKERNEL-WB11-LAT-OK-001"
    );
    let hourly_response = kernel.run_hillslope_phase(&build_lateral_request(&hourly_state, 0.0));
    assert_eq!(
        hourly_response.status.message_id(),
        "HKERNEL-WB11-LAT-OK-001"
    );

    let q_daily = writeback_flux_value(&daily_response, "q");
    let q_hourly = writeback_flux_value(&hourly_response, "q");
    assert!(
        (q_daily - q_hourly).abs() <= TOL,
        "hourly and daily lane WB19 lateral execution should remain numerically equivalent on the reference fixture (q_daily={q_daily}, q_hourly={q_hourly})"
    );
}

#[test]
fn wb19_contract_conformance_hourly_lane_preserves_drainage_flux_when_uncapped() {
    let mut kernel = Wb11HydrologyKernel;
    let mut daily_state = seeded_wb19_state_surface();
    daily_state.insert(
        BoundarySymbol::from("wb11_drainage_coefficient"),
        BoundaryValue::scalar(10.0),
    );
    daily_state.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(1.0),
    );
    let mut hourly_state = daily_state.clone();
    hourly_state.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(24.0),
    );

    let daily_lateral = kernel.run_hillslope_phase(&build_lateral_request(&daily_state, 0.0));
    assert_eq!(daily_lateral.status.message_id(), "HKERNEL-WB11-LAT-OK-001");
    let daily_q = writeback_flux_value(&daily_lateral, "q");
    let mut daily_drain_state = daily_state.clone();
    daily_drain_state.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(writeback_state_value(
            &daily_lateral,
            "wb18_perc_theta_0001",
        )),
    );
    daily_drain_state.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(writeback_state_value(
            &daily_lateral,
            "wb18_perc_theta_0002",
        )),
    );
    daily_drain_state.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(writeback_state_value(
            &daily_lateral,
            "wb11_drainable_storage",
        )),
    );
    daily_drain_state.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(writeback_state_value(&daily_lateral, "wb11_soil_water")),
    );
    let daily_drainage =
        kernel.run_hillslope_phase(&build_drainage_request(&daily_drain_state, daily_q));
    assert_eq!(
        daily_drainage.status.message_id(),
        "HKERNEL-WB11-DRAIN-OK-001"
    );
    let qdd_daily = writeback_flux_value(&daily_drainage, "Qdd");

    let hourly_lateral = kernel.run_hillslope_phase(&build_lateral_request(&hourly_state, 0.0));
    assert_eq!(
        hourly_lateral.status.message_id(),
        "HKERNEL-WB11-LAT-OK-001"
    );
    let hourly_q = writeback_flux_value(&hourly_lateral, "q");
    let mut hourly_drain_state = hourly_state.clone();
    hourly_drain_state.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(writeback_state_value(
            &hourly_lateral,
            "wb18_perc_theta_0001",
        )),
    );
    hourly_drain_state.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(writeback_state_value(
            &hourly_lateral,
            "wb18_perc_theta_0002",
        )),
    );
    hourly_drain_state.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(writeback_state_value(
            &hourly_lateral,
            "wb11_drainable_storage",
        )),
    );
    hourly_drain_state.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(writeback_state_value(&hourly_lateral, "wb11_soil_water")),
    );
    let hourly_drainage =
        kernel.run_hillslope_phase(&build_drainage_request(&hourly_drain_state, hourly_q));
    assert_eq!(
        hourly_drainage.status.message_id(),
        "HKERNEL-WB11-DRAIN-OK-001"
    );
    let hourly_qdd = writeback_flux_value(&hourly_drainage, "Qdd");
    let hourly_total_drain = writeback_flux_value(&hourly_drainage, "Qd");

    assert!(
        (qdd_daily - hourly_qdd).abs() <= TOL,
        "hourly and daily lane WB19 drainage execution should remain numerically equivalent on the reference fixture (qdd_daily={qdd_daily}, hourly_qdd={hourly_qdd})"
    );
    assert!(
        (hourly_total_drain - (hourly_q + hourly_qdd)).abs() <= TOL,
        "Qd must remain flux-conservative under hourly lane"
    );
}

#[test]
fn wb19_contract_conformance_rejects_non_integral_lane_substeps() {
    let mut kernel = Wb11HydrologyKernel;
    let mut invalid_state = seeded_wb19_state_surface();
    invalid_state.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(1.5),
    );
    let lateral_response = kernel.run_hillslope_phase(&build_lateral_request(&invalid_state, 0.0));
    assert_eq!(
        lateral_response.status.message_id(),
        "HKERNEL-WB11-LAT-E-003"
    );
    assert_eq!(
        lateral_response.status.boundary_class(),
        BoundaryClass::DomainViolation
    );

    let drainage_response =
        kernel.run_hillslope_phase(&build_drainage_request(&invalid_state, 0.0));
    assert_eq!(
        drainage_response.status.message_id(),
        "HKERNEL-WB11-DRAIN-E-003"
    );
    assert_eq!(
        drainage_response.status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn hphys0242_contract_hourly_tail_runs_drainage_before_lateral_and_publishes_saturation_carry() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_wb19_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(24.0),
    );
    state_surface.insert(
        BoundarySymbol::from("mofe_hourly_carry_arrays_enabled"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_enabled"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(9.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(15.0),
    );

    let drainage_response =
        kernel.run_hillslope_phase(&build_drainage_request_without_q(&state_surface));
    assert_eq!(
        drainage_response.status.message_id(),
        "HKERNEL-WB11-DRAIN-OK-001",
        "hourly baseline tail requires drainage to run before lateral without a preexisting q"
    );
    let q_drainage = writeback_flux_value(&drainage_response, "Qdd");

    let mut lateral_state = state_surface.clone();
    apply_state_writebacks(&mut lateral_state, &drainage_response);
    let lateral_response = kernel.run_hillslope_phase(&build_lateral_request_with_qdd(
        &lateral_state,
        0.0,
        q_drainage,
    ));
    assert_eq!(
        lateral_response.status.message_id(),
        "HKERNEL-WB11-LAT-OK-001"
    );

    let q_lateral = writeback_flux_value(&lateral_response, "q");
    let q_subhyd = writeback_flux_value(&lateral_response, "Qd");
    assert!(
        (q_subhyd - (q_drainage + q_lateral)).abs() <= TOL,
        "lateral phase must publish final same-pass Qd after drainage-first hourly tail"
    );

    let surface_saturation_sum = (1..=24)
        .map(|hour| writeback_state_value(&lateral_response, &format!("ui_SCrunf_{hour:04}")))
        .sum::<f64>();
    assert!(
        surface_saturation_sum > 0.0,
        "positive top-layer saturation excess must be emitted as ui_SCrunf hourly carry"
    );
    let theta_top_after = writeback_state_value(&lateral_response, "wb18_perc_theta_0001");
    assert!(
        theta_top_after <= 8.0 + TOL,
        "top-layer storage must be clipped to the unfrozen upper limit after ui_SCrunf publication"
    );
}
