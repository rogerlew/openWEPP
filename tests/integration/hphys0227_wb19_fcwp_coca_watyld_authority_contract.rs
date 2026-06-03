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
struct FcWpWatyldCaseExpected {
    status_code: String,
    q_m: f64,
    watyld: f64,
    fcdep_m: f64,
    unsdep_m: f64,
}

#[derive(Debug, Deserialize)]
struct FcWpWatyldCase {
    case_id: String,
    phase: String,
    thetfc_0001: f64,
    thetdr_0001: f64,
    expected: FcWpWatyldCaseExpected,
}

#[derive(Debug, Deserialize)]
struct FcWpWatyldFixture {
    suite_id: String,
    units_basis: String,
    tolerance_abs: f64,
    q_consistency_abs_m: f64,
    solwpv: i32,
    theta_0001_m: f64,
    dg_0001_m: f64,
    por_0001: f64,
    coca_0001: f64,
    cpm_0001: f64,
    ssc_0001_m_per_s: f64,
    avgslp: f64,
    slplen_m: f64,
    anisotropy: f64,
    cases: Vec<FcWpWatyldCase>,
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

fn seeded_state(
    fixture: &FcWpWatyldFixture,
    case: &FcWpWatyldCase,
) -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state = BTreeMap::new();
    state.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state.insert(
        BoundarySymbol::from("solthk"),
        BoundaryValue::scalar(fixture.dg_0001_m),
    );
    state.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(f64::from(fixture.solwpv)),
    );
    state.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("avgslp"),
        BoundaryValue::scalar(fixture.avgslp),
    );
    state.insert(
        BoundarySymbol::from("slplen"),
        BoundaryValue::scalar(fixture.slplen_m),
    );
    state.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(fixture.anisotropy),
    );
    state.insert(
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(24.0),
    );
    state.insert(
        BoundarySymbol::from("dg_0001"),
        BoundaryValue::scalar(fixture.dg_0001_m),
    );
    state.insert(
        BoundarySymbol::from("por_0001"),
        BoundaryValue::scalar(fixture.por_0001),
    );
    state.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(fixture.coca_0001),
    );
    state.insert(
        BoundarySymbol::from("cpm_0001"),
        BoundaryValue::scalar(fixture.cpm_0001),
    );
    state.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(case.thetfc_0001),
    );
    state.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(case.thetdr_0001),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(fixture.theta_0001_m),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar((case.thetfc_0001 - case.thetdr_0001) * fixture.dg_0001_m),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(fixture.ssc_0001_m_per_s),
    );
    state
}

#[test]
fn hphys0227_suite_metadata_and_contract_addendum_are_present() {
    let registry = repo_file("docs/specifications/external-authority/registry.yaml");
    let suite = repo_file(
        "docs/specifications/external-authority/suites/cas_l4_subhyd_watyld_fcwp_consistency_001.md",
    );
    let subhyd = repo_file("docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md");
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");

    assert!(
        registry.contains("cas_l4_subhyd_watyld_fcwp_consistency_001")
            && registry.contains("authority_level: 4")
            && registry.contains("gate_lane: required")
            && registry.contains("failure_class: hard-fail")
            && registry.contains(
                "tests/integration/hphys0227_wb19_fcwp_coca_watyld_authority_contract.rs"
            ),
        "registry must include HPHYS0227 Level-4 required hard-fail suite"
    );
    assert!(
        suite.contains("SC-SUBHYD-001#INV-SUBHYD-019")
            && suite.contains("SC-WATBAL-001#INV-WATBAL-009")
            && suite.contains("hash:")
            && suite.contains("source_commit:")
            && suite.contains("transform_note:"),
        "suite metadata must include SC linkage and fixture provenance integrity fields"
    );
    assert!(
        subhyd.contains("INV-SUBHYD-019")
            && subhyd.contains("HPHYS0227 WB19 FC/WP + COCA Water-Yield Coupling Addendum"),
        "SC-SUBHYD-001 must include HPHYS0227 FC/WP + COCA coupling authority"
    );
    assert!(
        watbal.contains("HPHYS0227 WB19 FC/WP + COCA Water-Yield Coupling Addendum")
            && watbal.contains("cas_l4_subhyd_watyld_fcwp_consistency_001"),
        "SC-WATBAL-001 must include HPHYS0227 addendum and suite linkage"
    );
}

#[test]
fn hphys0227_lateral_watyld_uses_fcwp_theta_lineage_and_preserves_coca_threshold_q() {
    let fixture: FcWpWatyldFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/wb19_fcwp_coca_watyld_cases.json",
    );
    assert_eq!(
        fixture.suite_id,
        "cas_l4_subhyd_watyld_fcwp_consistency_001"
    );
    assert_eq!(fixture.units_basis, "m_and_dimensionless");

    let mut kernel = Wb11HydrologyKernel;
    let mut q_by_case: BTreeMap<String, f64> = BTreeMap::new();
    let mut watyld_by_case: BTreeMap<String, f64> = BTreeMap::new();
    let mut fcdep_by_case: BTreeMap<String, f64> = BTreeMap::new();

    for case in &fixture.cases {
        assert_eq!(case.phase, "lateral", "fixture phase must be lateral");
        let response = kernel.run_hillslope_phase(&lateral_request(&seeded_state(&fixture, case)));

        assert_eq!(
            response.status.message_id(),
            case.expected.status_code,
            "{} status mismatch",
            case.case_id
        );

        let q = writeback_flux_value(&response, "q");
        let watyld = writeback_state_value(&response, "wb19_watyld");
        let fcdep = writeback_state_value(&response, "wb19_fcdep");
        let unsdep = writeback_state_value(&response, "wb19_unsdep");

        assert!(
            (q - case.expected.q_m).abs() <= fixture.tolerance_abs,
            "{} q mismatch: observed={} expected={}",
            case.case_id,
            q,
            case.expected.q_m
        );
        assert!(
            (watyld - case.expected.watyld).abs() <= fixture.tolerance_abs,
            "{} watyld mismatch: observed={} expected={}",
            case.case_id,
            watyld,
            case.expected.watyld
        );
        assert!(
            (fcdep - case.expected.fcdep_m).abs() <= fixture.tolerance_abs,
            "{} fcdep mismatch: observed={} expected={}",
            case.case_id,
            fcdep,
            case.expected.fcdep_m
        );
        assert!(
            (unsdep - case.expected.unsdep_m).abs() <= fixture.tolerance_abs,
            "{} unsdep mismatch: observed={} expected={}",
            case.case_id,
            unsdep,
            case.expected.unsdep_m
        );

        q_by_case.insert(case.case_id.clone(), q);
        watyld_by_case.insert(case.case_id.clone(), watyld);
        fcdep_by_case.insert(case.case_id.clone(), fcdep);
    }

    let q_low = q_by_case
        .get("low_fc_theta")
        .expect("missing low_fc_theta case");
    let q_high = q_by_case
        .get("high_fc_theta")
        .expect("missing high_fc_theta case");
    assert!(
        (*q_low - *q_high).abs() <= fixture.q_consistency_abs_m,
        "q must remain coca-threshold controlled when FC/WP theta lineage changes under fixed drfc"
    );

    let watyld_low = watyld_by_case
        .get("low_fc_theta")
        .expect("missing low_fc_theta watyld");
    let watyld_high = watyld_by_case
        .get("high_fc_theta")
        .expect("missing high_fc_theta watyld");
    assert!(
        *watyld_low > *watyld_high,
        "higher FC theta must reduce watyld under fixed porosity/coca"
    );

    let fcdep_low = fcdep_by_case
        .get("low_fc_theta")
        .expect("missing low_fc_theta fcdep");
    let fcdep_high = fcdep_by_case
        .get("high_fc_theta")
        .expect("missing high_fc_theta fcdep");
    assert!(
        *fcdep_low > *fcdep_high,
        "higher FC theta must increase fcdep drawdown under fixed q in solwpv<2006 branch"
    );
}
