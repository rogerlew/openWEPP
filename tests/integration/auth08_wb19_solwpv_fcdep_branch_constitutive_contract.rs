use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BranchCaseExpected {
    status_code: String,
    q_m: f64,
    watyld: f64,
    fcdep_m: f64,
    unsdep_m: f64,
}

#[derive(Debug, Deserialize)]
struct BranchCase {
    case_id: String,
    solwpv: i32,
    expected: BranchCaseExpected,
}

#[derive(Debug, Deserialize)]
struct BranchFixture {
    suite_id: String,
    units_basis: String,
    tolerance_abs: f64,
    cases: Vec<BranchCase>,
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

fn seeded_state(solwpv: i32) -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(f64::from(solwpv)),
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
    state_surface
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
fn auth08_suite_registry_and_contract_addendum_are_present() {
    let registry = repo_file("docs/specifications/external-authority/registry.yaml");
    let suite = repo_file(
        "docs/specifications/external-authority/suites/cas_l3_subhyd_solwpv_fcdep_branch_001.md",
    );
    let subhyd = repo_file("docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md");
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");

    assert!(
        registry.contains("cas_l3_subhyd_solwpv_fcdep_branch_001")
            && registry.contains("authority_level: 3")
            && registry.contains("gate_lane: periodic")
            && registry.contains("failure_class: investigation")
            && registry.contains(
                "tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs"
            ),
        "external-authority registry must include AUTH09 Level-3 periodic investigation legacy/sanity suite"
    );
    assert!(
        suite.contains("SC-SUBHYD-001#INV-SUBHYD-015")
            && suite.contains("SC-WATBAL-001#INV-WATBAL-009")
            && suite.contains("Level-3 legacy/sanity evidence")
            && suite.contains("hash:")
            && suite.contains("source_commit:")
            && suite.contains("transform_note:"),
        "AUTH09 suite must link SC invariants, legacy/sanity posture, and fixture provenance metadata"
    );
    assert!(
        subhyd.contains("HPHYS0222 WB19 `solwpv` Branch-Authority Correction Addendum")
            && subhyd.contains("solwpv >= 2006")
            && subhyd.contains("must not apply `fcdep = fcdep - q/watyld`")
            && subhyd.contains("periodic")
            && subhyd.contains("investigation"),
        "SC-SUBHYD-001 must encode HPHYS0222 branch authority closure"
    );
    assert!(
        watbal.contains("HPHYS0222 WB19 `solwpv` Branch-Authority Correction Addendum")
            && watbal.contains("solwpv >= 2006")
            && watbal.contains("must")
            && watbal.contains("`fcdep = fcdep - q/watyld`")
            && watbal.contains("periodic/investigation"),
        "SC-WATBAL-001 must encode HPHYS0222 branch authority closure and AUTH08A re-tiering posture"
    );
}

#[test]
fn auth08_solwpv_branch_fixture_cases_enforce_fcdep_mutation_scope() {
    let fixture: BranchFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l3_subhyd_solwpv_fcdep_branch_001/solwpv_fcdep_branch_cases.json",
    );
    assert_eq!(fixture.suite_id, "cas_l3_subhyd_solwpv_fcdep_branch_001");
    assert_eq!(fixture.units_basis, "m_and_dimensionless");

    let mut kernel = Wb11HydrologyKernel;
    for case in &fixture.cases {
        let response = kernel.run_hillslope_phase(&lateral_request(&seeded_state(case.solwpv)));
        assert_eq!(
            response.status.message_id(),
            case.expected.status_code,
            "{} status mismatch",
            case.case_id
        );

        let q_lateral = writeback_scalar(&response, "q");
        let watyld = writeback_scalar(&response, "wb19_watyld");
        let fcdep = writeback_scalar(&response, "wb19_fcdep");
        let unsdep = writeback_scalar(&response, "wb19_unsdep");

        assert!(
            (q_lateral - case.expected.q_m).abs() <= fixture.tolerance_abs,
            "{} q mismatch: expected={} observed={}",
            case.case_id,
            case.expected.q_m,
            q_lateral
        );
        assert!(
            (watyld - case.expected.watyld).abs() <= fixture.tolerance_abs,
            "{} watyld mismatch: expected={} observed={}",
            case.case_id,
            case.expected.watyld,
            watyld
        );
        assert!(
            (fcdep - case.expected.fcdep_m).abs() <= fixture.tolerance_abs,
            "{} fcdep mismatch: expected={} observed={}",
            case.case_id,
            case.expected.fcdep_m,
            fcdep
        );
        assert!(
            (unsdep - case.expected.unsdep_m).abs() <= fixture.tolerance_abs,
            "{} unsdep mismatch: expected={} observed={}",
            case.case_id,
            case.expected.unsdep_m,
            unsdep
        );
    }
}
