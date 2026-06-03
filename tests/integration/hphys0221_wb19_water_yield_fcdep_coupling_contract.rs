use std::collections::BTreeMap;

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest,
};
use openwepp_sim_contract::status::BoundaryClass;

const TOL: f64 = 1.0e-12;

fn lateral_request(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> HillslopeKernelRequest<'_> {
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
        state_surface,
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
#[allow(clippy::too_many_lines)]
fn hphys0221_wb19_hourly_solwpv_branch_uses_hphys0247_bottom_contiguous_saturation_selection() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(2.0));
    state_surface.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(0.3),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(1.4),
    );
    state_surface.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(10.0));
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(1.0e6),
    );

    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.4),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(1.0e-5),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.8));
    state_surface.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));

    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc_0002"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0002"),
        BoundaryValue::scalar(1.0e-5),
    );
    state_surface.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("por_0002"), BoundaryValue::scalar(0.8));
    state_surface.insert(
        BoundarySymbol::from("coca_0002"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(BoundarySymbol::from("cpm_0002"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(24.0),
    );

    let mut kernel = Wb11HydrologyKernel;

    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2005.0),
    );
    let response_2005 = kernel.run_hillslope_phase(&lateral_request(&state_surface));
    assert_eq!(response_2005.status.message_id(), "HKERNEL-WB11-LAT-OK-001");
    let q_2005 = writeback_scalar(&response_2005, "q");
    let fcdep_2005 = writeback_scalar(&response_2005, "wb19_fcdep");
    assert!(
        (q_2005 - 0.3).abs() <= TOL,
        "HPHYS0247 bottom-contiguous selection should allow bottom-layer flow independent of solwpv selector mode"
    );
    assert!(
        (fcdep_2005 - 1.0).abs() <= TOL,
        "hourly substep recomputation should leave threshold-saturated bottom-layer fcdep; fcdep_2005={fcdep_2005}"
    );

    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2006.0),
    );
    let response_2006 = kernel.run_hillslope_phase(&lateral_request(&state_surface));
    assert_eq!(response_2006.status.message_id(), "HKERNEL-WB11-LAT-OK-001");
    let q_2006 = writeback_scalar(&response_2006, "q");
    let fcdep_2006 = writeback_scalar(&response_2006, "wb19_fcdep");
    assert!(
        (q_2006 - 0.3).abs() <= TOL,
        "HPHYS0247 bottom-contiguous selection should allow bottom-layer flow for solwpv=2006"
    );
    assert!(
        (fcdep_2006 - 1.0).abs() <= TOL,
        "solwpv>=2006 must preserve fcdep under equivalent forcing"
    );
}

#[test]
fn hphys0221_wb19_solwpv_lt_2006_updates_fcdep_unsdep_from_watyld() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2005.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(0.3),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(10.0));
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(1.0e6),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(1.0e-5),
    );
    state_surface.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.8));
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&lateral_request(&state_surface));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-LAT-OK-001");

    let watyld = writeback_scalar(&response, "wb19_watyld");
    let fcdep_after = writeback_scalar(&response, "wb19_fcdep");
    let unsdep_after = writeback_scalar(&response, "wb19_unsdep");

    assert!(
        (watyld - 0.1).abs() <= TOL,
        "watyld should follow avpora/avfca/avcoca"
    );
    assert!(
        fcdep_after.abs() <= TOL,
        "non-2006 branch should apply fcdep - q/watyld with lower bound at zero"
    );
    assert!(
        (unsdep_after - 1.0).abs() <= TOL,
        "unsdep should be recomputed from updated fcdep"
    );
}

#[test]
fn hphys0221_wb19_solwpv_ge_2006_does_not_update_fcdep_unsdep_from_watyld() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(9002.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(0.3),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(10.0));
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(1.0e6),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(1.0e-5),
    );
    state_surface.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.8));
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&lateral_request(&state_surface));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-LAT-OK-001");

    let watyld = writeback_scalar(&response, "wb19_watyld");
    let fcdep_after = writeback_scalar(&response, "wb19_fcdep");
    let unsdep_after = writeback_scalar(&response, "wb19_unsdep");

    assert!(
        (watyld - 0.1).abs() <= TOL,
        "watyld should follow avpora/avfca/avcoca"
    );
    assert!(
        (fcdep_after - 1.0).abs() <= TOL,
        "solwpv>=2006 branch must preserve fcdep under equivalent forcing"
    );
    assert!(
        unsdep_after.abs() <= TOL,
        "solwpv>=2006 branch must preserve unsdep under equivalent forcing"
    );
}

#[test]
fn hphys0221_wb19_solwpv_lt_2006_rejects_non_positive_watyld() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2005.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(0.3),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(10.0));
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(1.0e6),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(1.0e-5),
    );
    state_surface.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.6));
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&lateral_request(&state_surface));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-LAT-E-003");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}
