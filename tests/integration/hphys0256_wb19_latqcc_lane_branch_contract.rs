use std::{collections::BTreeMap, fs};

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
};

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

fn writeback_scalar(response: &KernelRunResponse, symbol: &str) -> f64 {
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

fn base_daily_hourly_divergence_state() -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state = BTreeMap::new();

    state.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.4));
    state.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(9002.0),
    );
    state.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
    state.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(10.0));
    state.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(0.02),
    );
    state.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.09),
    );

    for layer_index in 1..=2 {
        let suffix = format!("{layer_index:04}");
        state.insert(
            BoundarySymbol::from(format!("dg_{suffix}")),
            BoundaryValue::scalar(0.2),
        );
        state.insert(
            BoundarySymbol::from(format!("por_{suffix}")),
            BoundaryValue::scalar(0.5),
        );
        state.insert(
            BoundarySymbol::from(format!("cpm_{suffix}")),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from(format!("coca_{suffix}")),
            BoundaryValue::scalar(0.95),
        );
        state.insert(
            BoundarySymbol::from(format!("thetfc_{suffix}")),
            BoundaryValue::scalar(0.3),
        );
        state.insert(
            BoundarySymbol::from(format!("thetdr_{suffix}")),
            BoundaryValue::scalar(0.1),
        );
        state.insert(
            BoundarySymbol::from(format!("wb18_perc_fc_{suffix}")),
            BoundaryValue::scalar(0.04),
        );
        state.insert(
            BoundarySymbol::from(format!("wb18_perc_ul_{suffix}")),
            BoundaryValue::scalar(0.08),
        );
        state.insert(
            BoundarySymbol::from(format!("wb18_perc_ssc_{suffix}")),
            BoundaryValue::scalar(1.0e-6),
        );
        state.insert(
            BoundarySymbol::from(format!("wb19_lateral_ssh_{suffix}")),
            BoundaryValue::scalar(1.0e-6),
        );
    }

    state.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.07),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.02),
    );

    state
}

fn expected_daily_solwpv_ge_2006_q() -> f64 {
    let layer_storage = 0.07_f64;
    let field_capacity_store = 0.04_f64;
    let upper_limit = 0.08_f64;
    let layer_thickness = 0.2_f64;
    let conductivity = 1.0e-6_f64;
    let avgslp = 0.1_f64;
    let slplen = 10.0_f64;
    let hk = -2.655 / (field_capacity_store / upper_limit).log10();
    let saturation_ratio = layer_storage / upper_limit;
    let conductivity_fraction = if saturation_ratio < 0.95 {
        saturation_ratio.powf(hk).max(0.002)
    } else {
        1.0
    };
    let latk =
        86_400.0 * ((conductivity * conductivity_fraction * layer_thickness) / layer_thickness);

    (layer_thickness * latk * avgslp.atan().sin()) / slplen
}

#[test]
fn hphys0256_daily_solwpv_ge_2006_uses_daily_fzdrfc_branch_without_meblfc_gate() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = base_daily_hourly_divergence_state();
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(1.0),
    );

    let response = kernel.run_hillslope_phase(&lateral_request(&state_surface));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-LAT-OK-001");

    let q_lateral = writeback_scalar(&response, "q");
    let expected = expected_daily_solwpv_ge_2006_q();
    assert!(
        (q_lateral - expected).abs() <= TOL,
        "q_lateral={q_lateral}, expected_daily_q={expected}"
    );
    assert!(
        q_lateral > 0.0,
        "daily branch must not be blocked by hourly meblfc"
    );
}

#[test]
fn hphys0256_hourly_branch_retains_meblfc_gate_for_same_state() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = base_daily_hourly_divergence_state();
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(24.0),
    );

    let response = kernel.run_hillslope_phase(&lateral_request(&state_surface));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-LAT-OK-001");

    let q_lateral = writeback_scalar(&response, "q");
    assert!(
        q_lateral.abs() <= TOL,
        "hourly branch must retain meblfc gate; q_lateral={q_lateral}"
    );
}

#[test]
fn hphys0256_contract_authority_sections_exist() {
    let subhyd_contract =
        fs::read_to_string("docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md")
            .expect("read SC-SUBHYD-001");
    let watbal_contract =
        fs::read_to_string("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md")
            .expect("read SC-WATBAL-001");

    assert!(subhyd_contract.contains("INV-SUBHYD-026"));
    assert!(subhyd_contract.contains("HPHYS0256 WB19 Daily Lateral Lane-Branch Addendum"));
    assert!(watbal_contract.contains("INV-WATBAL-043"));
    assert!(watbal_contract.contains("HPHYS0256 WB19 Latqcc Lane-Branch Addendum"));
}
