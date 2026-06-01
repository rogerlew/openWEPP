use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
};
use openwepp_sim_contract::status::BoundaryClass;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WithdrawalCaseExpected {
    status_code: String,
    boundary_class: Option<String>,
    withdrawal_symbol: Option<String>,
    withdrawal_m: Option<f64>,
    soil_water_after_m: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct WithdrawalCase {
    case_id: String,
    phase: String,
    soil_water_before_m: f64,
    q_lateral_input_m: Option<f64>,
    expected: WithdrawalCaseExpected,
}

#[derive(Debug, Deserialize)]
struct WithdrawalFixture {
    suite_id: String,
    units_basis: String,
    tolerance_abs: f64,
    cases: Vec<WithdrawalCase>,
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

fn drainage_request(
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

fn boundary_class_from_fixture(label: &str) -> BoundaryClass {
    match label {
        "MissingRequiredInput" => BoundaryClass::MissingRequiredInput,
        "NonFinite" => BoundaryClass::NonFinite,
        "DomainViolation" => BoundaryClass::DomainViolation,
        other => panic!("unsupported boundary class label in fixture: {other}"),
    }
}

#[test]
fn hphys0224_suite_metadata_and_contract_addendum_are_present() {
    let registry = repo_file("docs/specifications/external-authority/registry.yaml");
    let suite = repo_file(
        "docs/specifications/external-authority/suites/cas_l4_subhyd_withdrawal_soilwater_cap_001.md",
    );
    let subhyd = repo_file("docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md");
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");

    assert!(
        registry.contains("cas_l4_subhyd_withdrawal_soilwater_cap_001")
            && registry.contains("authority_level: 4")
            && registry.contains("gate_lane: required")
            && registry.contains("failure_class: hard-fail")
            && registry
                .contains("tests/integration/hphys0224_wb19_withdrawal_soilwater_cap_contract.rs"),
        "external-authority registry must include HPHYS0224 Level-4 required hard-fail suite"
    );
    assert!(
        suite.contains("SC-SUBHYD-001#INV-SUBHYD-016")
            && suite.contains("SC-WATBAL-001#INV-WATBAL-009")
            && suite.contains("hard-fail")
            && suite.contains("hash:")
            && suite.contains("source_commit:")
            && suite.contains("transform_note:"),
        "suite metadata must include SC linkage, hard-fail posture, and fixture provenance integrity fields"
    );
    assert!(
        subhyd.contains("INV-SUBHYD-016")
            && subhyd.contains("HPHYS0224 WB19 Realized-Withdrawal Soil-Water Cap Addendum")
            && subhyd.contains("must not be silently clamped"),
        "SC-SUBHYD-001 must include HPHYS0224 realized-withdrawal cap authority"
    );
    assert!(
        watbal.contains("HPHYS0224 WB19 Realized-Withdrawal Soil-Water Cap Addendum")
            && watbal.contains("typed hard-fail domain violation")
            && watbal.contains("cas_l4_subhyd_withdrawal_soilwater_cap_001"),
        "SC-WATBAL-001 must include HPHYS0224 non-clamping WB19 cap authority and suite linkage"
    );
}

#[test]
fn hphys0224_withdrawal_cases_enforce_wb19_soilwater_cap() {
    let fixture: WithdrawalFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_subhyd_withdrawal_soilwater_cap_001/withdrawal_soilwater_cap_cases.json",
    );
    assert_eq!(
        fixture.suite_id,
        "cas_l4_subhyd_withdrawal_soilwater_cap_001"
    );
    assert_eq!(fixture.units_basis, "m");

    let mut kernel = Wb11HydrologyKernel;
    for case in &fixture.cases {
        let mut state = seeded_wb19_state_surface();
        state.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(case.soil_water_before_m),
        );

        let response = match case.phase.as_str() {
            "lateral" => kernel.run_hillslope_phase(&lateral_request(&state)),
            "drainage" => kernel.run_hillslope_phase(&drainage_request(
                &state,
                case.q_lateral_input_m.unwrap_or(0.0),
            )),
            other => panic!(
                "unsupported phase in fixture case {}: {other}",
                case.case_id
            ),
        };

        assert_eq!(
            response.status.message_id(),
            case.expected.status_code,
            "{} status mismatch",
            case.case_id
        );

        if let Some(expected_boundary_class) = &case.expected.boundary_class {
            assert_eq!(
                response.status.boundary_class(),
                boundary_class_from_fixture(expected_boundary_class),
                "{} boundary class mismatch",
                case.case_id
            );
        }

        if let (Some(withdrawal_symbol), Some(expected_withdrawal)) =
            (&case.expected.withdrawal_symbol, case.expected.withdrawal_m)
        {
            let observed = writeback_flux_value(&response, withdrawal_symbol);
            assert!(
                (observed - expected_withdrawal).abs() <= fixture.tolerance_abs,
                "{} {} mismatch: expected={} observed={}",
                case.case_id,
                withdrawal_symbol,
                expected_withdrawal,
                observed
            );
        }

        if let Some(expected_soil_water_after) = case.expected.soil_water_after_m {
            let observed_soil_water_after = writeback_state_value(&response, "wb11_soil_water");
            assert!(
                (observed_soil_water_after - expected_soil_water_after).abs()
                    <= fixture.tolerance_abs,
                "{} wb11_soil_water mismatch: expected={} observed={}",
                case.case_id,
                expected_soil_water_after,
                observed_soil_water_after
            );
        }
    }
}
