use std::collections::BTreeMap;

use openwepp_hillslope_orchestrator::{HillslopePhase, HillslopePhaseGraph, Wb11HydrologyKernel};
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
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));

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
    state.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(1.0));

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

fn state_scalar(state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>, symbol: &str) -> f64 {
    state_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing state symbol {symbol}"))
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
        (theta_1_after - 4.998_320_164_567_006).abs() <= TOL,
        "theta_1_after={theta_1_after}"
    );
    assert!(
        (theta_2_after - 4.987_995_212_529_218).abs() <= TOL,
        "theta_2_after={theta_2_after}"
    );
    assert!(
        (soil_after - 9.986_315_377_096_224).abs() <= TOL,
        "soil_after={soil_after}"
    );
    assert!(
        (pei_1 - 0.001_679_835_432_994_461_2).abs() <= TOL,
        "pei_1={pei_1}"
    );
    assert!(
        (pei_2 - 0.013_684_622_903_775_864).abs() <= TOL,
        "pei_2={pei_2}"
    );
    assert!(
        (d_loss - 0.013_684_622_903_775_864).abs() <= TOL,
        "d_loss={d_loss}"
    );
    assert!(
        (pe_recharge - 0.013_684_622_903_775_864).abs() <= TOL,
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

#[test]
fn wb18_contract_conformance_allows_non_positive_fc_ul_ratio_with_legacy_bi_zero_branch() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(5.0),
    );

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-OK-001");

    let pei = writeback_flux_value(&response, "wb18_perc_pei_0001");
    let theta_after = writeback_state_value(&response, "wb18_perc_theta_0001");
    assert!(
        (pei - 0.1728).abs() <= TOL,
        "legacy Bi=0 branch should yield fx=1 and pei=0.1728, observed {pei}"
    );
    assert!(
        (theta_after - 4.8272).abs() <= TOL,
        "theta_after={theta_after}"
    );
}

#[test]
fn wb18_contract_conformance_saturated_branch_bypasses_fc_ul_ratio_guard() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(9.0),
    );

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-OK-001");
    let pei = writeback_flux_value(&response, "wb18_perc_pei_0001");
    assert!(
        pei > 0.0,
        "saturated-branch bypass must not raise ratio-domain hard-fail; pei={pei}"
    );
}

#[test]
fn hphys0254_wb18_lower_layer_over_ul_uses_legacy_stu_cap() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(9.0),
    );

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-OK-001");

    let bottom_flux = 86_400.0 * state_scalar(&state_surface, "wb18_perc_ssc_0002");
    let lower_theta_after_bottom = 9.0 - bottom_flux;
    assert!(
        lower_theta_after_bottom / state_scalar(&state_surface, "wb18_perc_ul_0002") >= 0.95,
        "test vector must exercise legacy lower-layer stu cap"
    );

    let stz = state_scalar(&state_surface, "wb18_perc_theta_0001")
        / state_scalar(&state_surface, "wb18_perc_ul_0001");
    let ratio = state_scalar(&state_surface, "wb18_perc_fc_0001")
        / state_scalar(&state_surface, "wb18_perc_ul_0001");
    let bi = -2.655_f64 / ratio.log10();
    let fx = stz.powf(bi).max(0.002);
    let pei_pre = 86_400.0 * state_scalar(&state_surface, "wb18_perc_ssc_0001") * fx;
    let expected_top_pei = pei_pre * (1.0_f64 - 0.95).sqrt();

    let pei_1 = writeback_flux_value(&response, "wb18_perc_pei_0001");
    assert!(
        (pei_1 - expected_top_pei).abs() <= TOL,
        "baseline perc.for caps downstream stu to 0.95 before attenuation; expected={expected_top_pei}, observed={pei_1}"
    );
}

#[test]
fn wb18_contract_conformance_hourly_lane_substeps_execute_iterative_recompute() {
    let mut kernel = Wb11HydrologyKernel;
    let mut daily_state_surface = seeded_perc_state_surface();
    daily_state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));

    let daily_response = kernel.run_hillslope_phase(&build_perc_request(&daily_state_surface));
    assert_eq!(
        daily_response.status.message_id(),
        "HKERNEL-WB11-PERC-OK-001"
    );
    let pei_daily = writeback_flux_value(&daily_response, "wb18_perc_pei_0001");

    let mut hourly_state_surface = daily_state_surface.clone();
    hourly_state_surface.insert(
        BoundarySymbol::from("wb18_perc_lane_substeps"),
        BoundaryValue::scalar(24.0),
    );

    let hourly_response = kernel.run_hillslope_phase(&build_perc_request(&hourly_state_surface));
    assert_eq!(
        hourly_response.status.message_id(),
        "HKERNEL-WB11-PERC-OK-001"
    );
    let pei_hourly = writeback_flux_value(&hourly_response, "wb18_perc_pei_0001");
    let theta_hourly = writeback_state_value(&hourly_response, "wb18_perc_theta_0001");

    assert!(
        (pei_hourly - 0.172_800_000_000_000_06).abs() <= TOL,
        "hourly lane must force baseline bottom-layer fx=1 and accumulate 24 attenuated bottom seepage steps (observed={pei_hourly})"
    );
    assert!(
        (theta_hourly - 4.827_2).abs() <= TOL,
        "hourly lane must mutate bottom layer by accumulated sep/ui_LFtstp (observed_theta={theta_hourly})"
    );
    assert!(
        (pei_hourly - (pei_daily / 24.0)).abs() > 1.0e-8,
        "hourly lane must not regress to divisor-only single-pass attenuation (daily={pei_daily}, hourly={pei_hourly})"
    );
}

#[test]
fn wb18_contract_conformance_hourly_restrictive_bottom_uses_bedrock_thickness_weighting() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_lane_substeps"),
        BoundaryValue::scalar(24.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(5.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(4.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(BoundarySymbol::from("slflag"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("ui_bdrkth"),
        BoundaryValue::scalar(10.0),
    );
    state_surface.insert(
        BoundarySymbol::from("kslast"),
        BoundaryValue::scalar(0.01 / 3.6e6),
    );

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-OK-001");

    let layer_conductivity = state_scalar(&state_surface, "wb18_perc_ssc_0001");
    let layer_depth = state_scalar(&state_surface, "dg_0001");
    let restrictive_thickness = state_scalar(&state_surface, "ui_bdrkth");
    let restrictive_conductivity = state_scalar(&state_surface, "kslast");
    let expected_effective_conductivity = (layer_depth + restrictive_thickness)
        / ((layer_depth / layer_conductivity) + (restrictive_thickness / restrictive_conductivity));
    let expected_loss = 86_400.0 * expected_effective_conductivity;
    let unrestricted_loss = 86_400.0 * layer_conductivity;

    let d_loss = writeback_flux_value(&response, "D");
    let pe_recharge = writeback_flux_value(&response, "Pe");
    let pei_1 = writeback_flux_value(&response, "wb18_perc_pei_0001");
    let theta_after = writeback_state_value(&response, "wb18_perc_theta_0001");
    let soil_after = writeback_state_value(&response, "wb11_soil_water");

    assert!(
        (d_loss - expected_loss).abs() <= TOL,
        "hourly bottom restrictive branch must force fx=1 and use ui_bdrkth/kslast thickness-weighted K (expected={expected_loss}, observed={d_loss})"
    );
    assert!(
        (pe_recharge - expected_loss).abs() <= TOL,
        "Pe must publish the same accumulated bottom seepage as D (expected={expected_loss}, observed={pe_recharge})"
    );
    assert!(
        (pei_1 - expected_loss).abs() <= TOL,
        "bottom-layer pei must equal accumulated bottom seepage (expected={expected_loss}, observed={pei_1})"
    );
    assert!(
        (theta_after - (5.0 - expected_loss)).abs() <= TOL,
        "bottom layer state must mutate by accumulated sep/ui_LFtstp (expected={}, observed={theta_after})",
        5.0 - expected_loss
    );
    assert!(
        (soil_after - (5.0 - expected_loss)).abs() <= TOL,
        "aggregate soil water must track single-layer post-percolation state (expected={}, observed={soil_after})",
        5.0 - expected_loss
    );
    assert!(
        d_loss < unrestricted_loss / 100.0,
        "restrictive layer should strongly attenuate hourly bottom seepage (restricted={d_loss}, unrestricted={unrestricted_loss})"
    );
}

#[test]
fn wb18_contract_conformance_hourly_restrictive_bottom_requires_ui_bdrkth() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_lane_substeps"),
        BoundaryValue::scalar(24.0),
    );
    state_surface.insert(BoundarySymbol::from("slflag"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("kslast"),
        BoundaryValue::scalar(0.01 / 3.6e6),
    );

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-E-001");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn wb18_contract_conformance_hourly_restrictive_bottom_rejects_non_finite_ui_bdrkth() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_lane_substeps"),
        BoundaryValue::scalar(24.0),
    );
    state_surface.insert(BoundarySymbol::from("slflag"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("kslast"),
        BoundaryValue::scalar(0.01 / 3.6e6),
    );
    state_surface.insert(
        BoundarySymbol::from("ui_bdrkth"),
        BoundaryValue::scalar(f64::NAN),
    );

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-E-002");
    assert_eq!(response.status.boundary_class(), BoundaryClass::NonFinite);
}

#[test]
fn wb18_contract_conformance_hourly_restrictive_bottom_rejects_non_positive_ui_bdrkth() {
    let mut kernel = Wb11HydrologyKernel;
    for invalid_thickness in [0.0, -1.0] {
        let mut state_surface = seeded_perc_state_surface();
        state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_lane_substeps"),
            BoundaryValue::scalar(24.0),
        );
        state_surface.insert(BoundarySymbol::from("slflag"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("kslast"),
            BoundaryValue::scalar(0.01 / 3.6e6),
        );
        state_surface.insert(
            BoundarySymbol::from("ui_bdrkth"),
            BoundaryValue::scalar(invalid_thickness),
        );

        let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));
        assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-E-003");
        assert_eq!(
            response.status.boundary_class(),
            BoundaryClass::DomainViolation
        );
    }
}

#[test]
fn wb18_contract_conformance_rejects_non_positive_lane_substeps() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_lane_substeps"),
        BoundaryValue::scalar(0.0),
    );

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-E-003");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn wb18_contract_conformance_daily_restrictive_layer_harmonic_conductivity_reduces_bottom_flux() {
    let mut kernel = Wb11HydrologyKernel;
    let mut unrestricted_state = seeded_perc_state_surface();
    unrestricted_state.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));

    let unrestricted_response =
        kernel.run_hillslope_phase(&build_perc_request(&unrestricted_state));
    assert_eq!(
        unrestricted_response.status.message_id(),
        "HKERNEL-WB11-PERC-OK-001"
    );
    let pei_unrestricted = writeback_flux_value(&unrestricted_response, "wb18_perc_pei_0001");

    let mut restrictive_state = unrestricted_state.clone();
    restrictive_state.insert(BoundarySymbol::from("slflag"), BoundaryValue::scalar(1.0));
    restrictive_state.insert(
        BoundarySymbol::from("kslast"),
        BoundaryValue::scalar(1.0e-8),
    );

    let restrictive_response = kernel.run_hillslope_phase(&build_perc_request(&restrictive_state));
    assert_eq!(
        restrictive_response.status.message_id(),
        "HKERNEL-WB11-PERC-OK-001"
    );
    let pei_restrictive = writeback_flux_value(&restrictive_response, "wb18_perc_pei_0001");

    let stz = 5.0_f64 / 8.0_f64;
    let ratio = 4.0_f64 / 8.0_f64;
    let bi = -2.655_f64 / ratio.log10();
    let fx = stz.powf(bi).max(0.002);
    let k_eff = (2.0 * 1.0e-5 * 1.0e-8) / (1.0e-5 + 1.0e-8);
    let expected_restrictive = 86_400.0 * k_eff * fx;

    assert!(
        (pei_restrictive - expected_restrictive).abs() <= 1.0e-6,
        "restrictive bottom-layer branch must use harmonic Ksi_eff (expected={expected_restrictive}, observed={pei_restrictive})"
    );
    assert!(
        pei_restrictive < pei_unrestricted,
        "restrictive bottom-layer conductivity must reduce daily seepage (restricted={pei_restrictive}, unrestricted={pei_unrestricted})"
    );
}

#[test]
fn wb18_contract_conformance_rejects_non_positive_kslast_when_slflag_enabled() {
    let mut kernel = Wb11HydrologyKernel;
    let mut state_surface = seeded_perc_state_surface();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("slflag"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("kslast"), BoundaryValue::scalar(0.0));

    let response = kernel.run_hillslope_phase(&build_perc_request(&state_surface));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-E-003");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn hphys0242_contract_wb18_hourly_percolation_precedes_final_hour_et() {
    let ordered = HillslopePhaseGraph::canonical_order();
    let percolation_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::PercolationDeepSeepage)
        .expect("percolation phase must exist");
    let et_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::Evapotranspiration)
        .expect("evapotranspiration phase must exist");

    assert!(
        percolation_index < et_index,
        "HPHYS0242 requires WB18 hourly percolation lineage before final-hour ET"
    );
}
