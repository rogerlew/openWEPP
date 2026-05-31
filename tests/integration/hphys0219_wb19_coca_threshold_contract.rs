use std::collections::BTreeMap;

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest,
};
use openwepp_sim_contract::status::BoundaryClass;

const TOL: f64 = 1.0e-12;

fn seeded_lateral_state(coca: f64, cpm: f64) -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state = BTreeMap::new();
    state.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(0.8),
    );
    state.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
    state.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(10.0));
    state.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(1.0e6),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(0.2),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(1.0e-5),
    );
    state.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
    state.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(coca),
    );
    state.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(cpm));
    state
}

fn lateral_request(state: &BTreeMap<BoundarySymbol, BoundaryValue>) -> HillslopeKernelRequest<'_> {
    let flux_surface = Box::leak(Box::new(BTreeMap::from([(
        BoundarySymbol::from("Pe"),
        BoundaryValue::scalar(0.0),
    )])));
    HillslopeKernelRequest::with_transition_context(
        "lateral_transfer",
        HillslopeKernelPhaseClass::HydrologyLateralTransfer,
        HillslopeConsumerAdapter::Watbal,
        None,
        None,
        state,
        flux_surface,
    )
}

fn writeback_scalar(response: &openwepp_kernel_contract::KernelRunResponse, symbol: &str) -> f64 {
    response
        .writeback
        .state_updates
        .iter()
        .chain(response.writeback.flux_updates.iter())
        .find(|field| field.symbol == BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing writeback symbol {symbol}"))
        .value
        .as_f64()
}

#[test]
fn hphys0219_wb19_lateral_withdrawal_uses_coca_adjusted_threshold() {
    let state = seeded_lateral_state(0.5, 1.0);
    let mut kernel = Wb11HydrologyKernel;

    let response = kernel.run_hillslope_phase(&lateral_request(&state));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-LAT-OK-001");

    let q_lateral = writeback_scalar(&response, "q");
    let theta_after = writeback_scalar(&response, "wb18_perc_theta_0001");
    let soil_water_after = writeback_scalar(&response, "wb11_soil_water");
    let drainable_after = writeback_scalar(&response, "wb11_drainable_storage");

    // drfc-equivalent threshold: fc + (1-coca)*dg = 0.2 + 0.5*1.0 = 0.7
    // available above threshold: 1.0 - 0.7 = 0.3 (not legacy 0.8 from fc-only threshold)
    assert!((q_lateral - 0.3).abs() <= TOL, "q={q_lateral}");
    assert!(
        (theta_after - 0.7).abs() <= TOL,
        "theta_after={theta_after}"
    );
    assert!(
        (soil_water_after - 0.7).abs() <= TOL,
        "soil_water_after={soil_water_after}"
    );
    assert!(
        drainable_after.abs() <= TOL,
        "drainable_after={drainable_after}"
    );
}

#[test]
fn hphys0219_wb19_lateral_rejects_domain_invalid_coca() {
    let state = seeded_lateral_state(1.2, 1.0);
    let mut kernel = Wb11HydrologyKernel;

    let response = kernel.run_hillslope_phase(&lateral_request(&state));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-LAT-E-003");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn hphys0219_wb19_threshold_is_independent_of_cpm_when_coca_is_fixed() {
    let mut kernel = Wb11HydrologyKernel;
    let response_cpm_low =
        kernel.run_hillslope_phase(&lateral_request(&seeded_lateral_state(0.5, 0.5)));
    let q_low = writeback_scalar(&response_cpm_low, "q");

    let response_cpm_high =
        kernel.run_hillslope_phase(&lateral_request(&seeded_lateral_state(0.5, 1.0)));
    let q_high = writeback_scalar(&response_cpm_high, "q");

    assert!(
        (q_low - q_high).abs() <= TOL,
        "q should follow coca-threshold lineage, not cpm lineage (q_low={q_low}, q_high={q_high})"
    );
}
