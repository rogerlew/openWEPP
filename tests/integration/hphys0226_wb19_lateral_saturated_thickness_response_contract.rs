use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LateralResponseCaseExpected {
    status_code: String,
}

#[derive(Debug, Deserialize)]
struct LateralResponseCase {
    case_id: String,
    phase: String,
    theta_0001_m: f64,
    fc_0001_m: f64,
    expected: LateralResponseCaseExpected,
}

#[derive(Debug, Deserialize)]
struct LateralResponseFixture {
    suite_id: String,
    units_basis: String,
    tolerance_abs: f64,
    minimum_q_delta_m: f64,
    cases: Vec<LateralResponseCase>,
}

fn repo_file(path: &str) -> String {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let full_path = Path::new(repo_root).join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

fn repo_json_fixture<T: for<'de> Deserialize<'de>>(path: &str) -> T {
    let text = repo_file(path);
    serde_json::from_str::<T>(&text)
        .unwrap_or_else(|error| panic!("failed to parse fixture {path} as JSON: {error}"))
}

fn seeded_state(theta: f64, fc: f64) -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state = BTreeMap::new();
    state.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(1.0));
    state.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2006.0),
    );
    state.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
    state.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(10.0));
    state.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(1.0e6),
    );

    state.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
    state.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.8));
    state.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));
    state.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(theta),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(fc),
    );
    state.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(fc),
    );
    state.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(1.0e-5),
    );
    state
}

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
fn hphys0226_suite_metadata_and_contract_addendum_are_present() {
    let registry = repo_file("docs/specifications/external-authority/registry.yaml");
    let suite = repo_file(
        "docs/specifications/external-authority/suites/cas_l4_subhyd_lateral_saturated_thickness_response_001.md",
    );
    let subhyd = repo_file("docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md");
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");

    assert!(
        registry.contains("cas_l4_subhyd_lateral_saturated_thickness_response_001")
            && registry.contains("authority_level: 4")
            && registry.contains("gate_lane: required")
            && registry.contains("failure_class: hard-fail")
            && registry.contains(
                "tests/integration/hphys0226_wb19_lateral_saturated_thickness_response_contract.rs"
            ),
        "registry must include HPHYS0226 Level-4 required hard-fail suite"
    );
    assert!(
        suite.contains("SC-SUBHYD-001#INV-SUBHYD-018")
            && suite.contains("SC-WATBAL-001#INV-WATBAL-009")
            && suite.contains("hash:")
            && suite.contains("source_commit:")
            && suite.contains("transform_note:"),
        "suite metadata must include SC linkage and fixture provenance integrity fields"
    );
    assert!(
        subhyd.contains("INV-SUBHYD-018")
            && subhyd.contains("HPHYS0226 WB19 Lateral Saturated-Thickness Response Addendum"),
        "SC-SUBHYD-001 must include HPHYS0226 lateral response authority"
    );
    assert!(
        watbal.contains("HPHYS0226 WB19 Lateral Saturated-Thickness Response Addendum")
            && watbal.contains("cas_l4_subhyd_lateral_saturated_thickness_response_001"),
        "SC-WATBAL-001 must include HPHYS0226 addendum and suite linkage"
    );
}

#[test]
fn hphys0226_lateral_flux_increases_with_saturated_thickness_under_fixed_drivers() {
    let fixture: LateralResponseFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_subhyd_lateral_saturated_thickness_response_001/lateral_saturated_thickness_response_cases.json",
    );
    assert_eq!(
        fixture.suite_id,
        "cas_l4_subhyd_lateral_saturated_thickness_response_001"
    );
    assert_eq!(fixture.units_basis, "m");

    let mut kernel = Wb11HydrologyKernel;
    let mut q_by_case: BTreeMap<String, f64> = BTreeMap::new();
    let mut layer_pool_by_case: BTreeMap<String, f64> = BTreeMap::new();

    for case in &fixture.cases {
        assert_eq!(case.phase, "lateral", "fixture phase must be lateral");
        let state = seeded_state(case.theta_0001_m, case.fc_0001_m);
        let response = kernel.run_hillslope_phase(&lateral_request(&state));

        assert_eq!(
            response.status.message_id(),
            case.expected.status_code,
            "{} status mismatch",
            case.case_id
        );

        let q = writeback_flux_value(&response, "q");
        let layer_pool = (case.theta_0001_m - case.fc_0001_m).max(0.0);
        assert!(
            q <= layer_pool + fixture.tolerance_abs,
            "{} q exceeds layer-derived available pool: q={} layer_pool={}",
            case.case_id,
            q,
            layer_pool
        );
        q_by_case.insert(case.case_id.clone(), q);
        layer_pool_by_case.insert(case.case_id.clone(), layer_pool);
    }

    let q_low = q_by_case
        .get("low_saturated_thickness")
        .expect("missing low_saturated_thickness case");
    let q_high = q_by_case
        .get("high_saturated_thickness")
        .expect("missing high_saturated_thickness case");
    let pool_low = layer_pool_by_case
        .get("low_saturated_thickness")
        .expect("missing low layer pool");
    let pool_high = layer_pool_by_case
        .get("high_saturated_thickness")
        .expect("missing high layer pool");

    assert!(
        (*q_low - *pool_low).abs() <= fixture.tolerance_abs,
        "low saturated thickness case should be pool-limited"
    );
    assert!(
        (*q_high - *pool_high).abs() <= fixture.tolerance_abs,
        "high saturated thickness case should be pool-limited"
    );
    assert!(
        *q_high > *q_low,
        "lateral flux must increase with saturated thickness under fixed drivers"
    );
    assert!(
        (*q_high - *q_low) >= fixture.minimum_q_delta_m,
        "lateral flux delta must exceed minimum threshold"
    );
}
