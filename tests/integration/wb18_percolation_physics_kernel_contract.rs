use std::collections::BTreeMap;

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest,
};
use openwepp_sim_contract::status::BoundaryClass;

const TOL: f64 = 1.0e-9;

fn seeded_perc_state_surface() -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state = BTreeMap::new();

    // Legacy WB11 symbols are still present for mixed-lane compatibility.
    state.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(10.0),
    );
    state.insert(
        BoundarySymbol::from("wb11_field_capacity"),
        BoundaryValue::scalar(8.0),
    );
    state.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(0.5),
    );

    // WB18 per-layer percolation symbols.
    state.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(5.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(4.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(8.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(2.0e-6),
    );

    state.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(5.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(4.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ul_0002"),
        BoundaryValue::scalar(8.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ssc_0002"),
        BoundaryValue::scalar(1.0e-5),
    );

    state
}

fn build_perc_request(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> HillslopeKernelRequest<'_> {
    let flux_surface = Box::leak(Box::new(BTreeMap::new()));

    HillslopeKernelRequest::with_transition_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        None,
        state_surface,
        flux_surface,
    )
}

fn writeback_state_value(
    response: &openwepp_kernel_contract::KernelRunResponse,
    symbol: &str,
) -> f64 {
    response
        .writeback
        .state_updates
        .iter()
        .find(|field| field.symbol == BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing state writeback symbol {symbol}"))
        .value
        .as_f64()
}

fn writeback_flux_value(
    response: &openwepp_kernel_contract::KernelRunResponse,
    symbol: &str,
) -> f64 {
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
fn wb18_contract_conformance_emits_layerwise_percolation_fluxes() {
    let mut kernel = Wb11HydrologyKernel;
    let state_surface = seeded_perc_state_surface();

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-OK-001");

    let theta_1_after = writeback_state_value(&response, "wb18_perc_theta_0001");
    let theta_2_after = writeback_state_value(&response, "wb18_perc_theta_0002");
    let soil_after = writeback_state_value(&response, "wb11_soil_water");
    let pei_1 = writeback_flux_value(&response, "wb18_perc_pei_0001");
    let pei_2 = writeback_flux_value(&response, "wb18_perc_pei_0002");
    let d_loss = writeback_flux_value(&response, "D");
    let pe_recharge = writeback_flux_value(&response, "Pe");

    assert!(
        (theta_1_after - 4.928_157_672_643_49).abs() <= TOL,
        "theta_1_after={theta_1_after}"
    );
    assert!(
        (theta_2_after - 4.531_842_327_356_51).abs() <= TOL,
        "theta_2_after={theta_2_after}"
    );
    assert!((soil_after - 9.46).abs() <= TOL, "soil_after={soil_after}");
    assert!(
        (pei_1 - 0.071_842_327_356_510_37).abs() <= TOL,
        "pei_1={pei_1}"
    );
    assert!((pei_2 - 0.54).abs() <= TOL, "pei_2={pei_2}");
    assert!((d_loss - 0.54).abs() <= TOL, "d_loss={d_loss}");
    assert!(
        (pe_recharge - 0.54).abs() <= TOL,
        "pe_recharge={pe_recharge}"
    );
}

#[test]
fn wb18_contract_conformance_rejects_missing_layer_symbol() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.remove(&BoundarySymbol::from("wb18_perc_theta_0002"));

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-E-001");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn wb18_contract_conformance_rejects_non_finite_layer_conductivity() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0002"),
        BoundaryValue::scalar(f64::NAN),
    );

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-E-002");
    assert_eq!(response.status.boundary_class(), BoundaryClass::NonFinite);
}

#[test]
fn wb18_contract_conformance_rejects_domain_invalid_layer_upper_limit() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0002"),
        BoundaryValue::scalar(0.0),
    );

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-E-003");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}
