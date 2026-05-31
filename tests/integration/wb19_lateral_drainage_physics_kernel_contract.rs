use std::collections::BTreeMap;

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
};
use openwepp_sim_contract::status::BoundaryClass;

const TOL: f64 = 1.0e-9;

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
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(4.0),
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
        (q_lateral - 0.122_188_051_789_035_1).abs() <= TOL,
        "q={q_lateral}"
    );
    assert!(
        (theta1_after_lateral - 4.377_811_948_210_965).abs() <= TOL,
        "theta1_after_lateral={theta1_after_lateral}"
    );
    assert!(
        (theta2_after_lateral - 6.0).abs() <= TOL,
        "theta2_after_lateral={theta2_after_lateral}"
    );
    assert!(
        (drainable_after_lateral - 2.377_811_948_210_965).abs() <= TOL,
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
        (q_subhyd - 0.222_188_051_789_035_1).abs() <= TOL,
        "Qd={q_subhyd}"
    );
    assert!(
        (theta2_after_drainage - 5.9).abs() <= TOL,
        "theta2_after_drainage={theta2_after_drainage}"
    );
    assert!(
        (drainable_after_drainage - 2.277_811_948_210_965).abs() <= TOL,
        "drainable_after_drainage={drainable_after_drainage}"
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
