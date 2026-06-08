use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LayerPoolCaseExpected {
    status_code: String,
    withdrawal_symbol: String,
    withdrawal_m: f64,
    soil_water_after_m: f64,
}

#[derive(Debug, Deserialize)]
struct LayerPoolCase {
    case_id: String,
    phase: String,
    legacy_drainable_storage_before_m: f64,
    q_lateral_input_m: Option<f64>,
    expected: LayerPoolCaseExpected,
}

#[derive(Debug, Deserialize)]
struct LayerPoolFixture {
    suite_id: String,
    units_basis: String,
    tolerance_abs: f64,
    cases: Vec<LayerPoolCase>,
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

fn collect_rust_sources(dir: &Path) -> Vec<PathBuf> {
    let mut entries = Vec::new();
    for item in fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("failed to read directory {}: {error}", dir.display()))
    {
        let item = item.unwrap_or_else(|error| {
            panic!(
                "failed to read directory entry under {}: {error}",
                dir.display()
            )
        });
        let path = item.path();
        if path.is_dir() {
            entries.extend(collect_rust_sources(&path));
            continue;
        }
        if path.extension().is_some_and(|extension| extension == "rs") {
            entries.push(path);
        }
    }
    entries
}

fn repo_hydrology_sources() -> Vec<PathBuf> {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let hydrology_root = repo_root.join("crates/openwepp-hillslope-orchestrator/src/hydrology");
    collect_rust_sources(&hydrology_root)
}

fn repo_file_contains(path: &Path, pattern: &str) -> bool {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", path.display()));
    text.contains(pattern)
}

#[allow(clippy::too_many_lines)]
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
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(40.0),
    );
    state.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(4.0),
    );
    state.insert(
        BoundarySymbol::from("thetfc_0002"),
        BoundaryValue::scalar(40.0),
    );
    state.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.0),
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
        BoundarySymbol::from("wb19_lateral_drain_lane_substeps"),
        BoundaryValue::scalar(24.0),
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

#[test]
fn hphys0225_suite_metadata_and_contract_addendum_are_present() {
    let registry = repo_file("docs/specifications/external-authority/registry.yaml");
    let suite = repo_file(
        "docs/specifications/external-authority/suites/cas_l4_subhyd_layer_pool_withdrawal_cap_001.md",
    );
    let subhyd = repo_file("docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md");
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");

    assert!(
        registry.contains("cas_l4_subhyd_layer_pool_withdrawal_cap_001")
            && registry.contains("authority_level: 4")
            && registry.contains("gate_lane: required")
            && registry.contains("failure_class: hard-fail")
            && registry
                .contains("tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs"),
        "registry must include HPHYS0225 Level-4 required hard-fail suite"
    );
    assert!(
        suite.contains("SC-SUBHYD-001#INV-SUBHYD-017")
            && suite.contains("SC-WATBAL-001#INV-WATBAL-009")
            && suite.contains("hash:")
            && suite.contains("source_commit:")
            && suite.contains("transform_note:"),
        "suite metadata must include SC linkage and fixture provenance integrity fields"
    );
    assert!(
        subhyd.contains("INV-SUBHYD-017")
            && subhyd.contains("HPHYS0225 WB19 Layer-Pool Available-Cap Authority Addendum"),
        "SC-SUBHYD-001 must include HPHYS0225 layer-pool cap authority"
    );
    assert!(
        watbal.contains("HPHYS0225 WB19 Layer-Pool Available-Cap Authority Addendum")
            && watbal.contains("cas_l4_subhyd_layer_pool_withdrawal_cap_001"),
        "SC-WATBAL-001 must include HPHYS0225 addendum and suite linkage"
    );
}

#[test]
fn hphys0225_cases_hold_withdrawal_behavior_constant_across_legacy_pool_perturbations() {
    let fixture: LayerPoolFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/layer_pool_withdrawal_cap_cases.json",
    );
    assert_eq!(
        fixture.suite_id,
        "cas_l4_subhyd_layer_pool_withdrawal_cap_001"
    );
    assert_eq!(fixture.units_basis, "m");

    let mut kernel = Wb11HydrologyKernel;
    let mut observed_withdrawals: BTreeMap<String, f64> = BTreeMap::new();

    for case in &fixture.cases {
        let mut state = seeded_wb19_state_surface();
        state.insert(
            BoundarySymbol::from("wb11_drainable_storage"),
            BoundaryValue::scalar(case.legacy_drainable_storage_before_m),
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

        let observed_withdrawal = writeback_flux_value(&response, &case.expected.withdrawal_symbol);
        assert!(
            (observed_withdrawal - case.expected.withdrawal_m).abs() <= fixture.tolerance_abs,
            "{} {} mismatch: expected={} observed={}",
            case.case_id,
            case.expected.withdrawal_symbol,
            case.expected.withdrawal_m,
            observed_withdrawal
        );
        observed_withdrawals.insert(case.case_id.clone(), observed_withdrawal);

        let observed_soil_water_after = writeback_state_value(&response, "wb11_soil_water");
        assert!(
            (observed_soil_water_after - case.expected.soil_water_after_m).abs()
                <= fixture.tolerance_abs,
            "{} wb11_soil_water mismatch: expected={} observed={}",
            case.case_id,
            case.expected.soil_water_after_m,
            observed_soil_water_after
        );
    }

    let lateral_low = observed_withdrawals
        .get("lateral_low_legacy_pool")
        .expect("missing lateral low-legacy case");
    let lateral_high = observed_withdrawals
        .get("lateral_high_legacy_pool")
        .expect("missing lateral high-legacy case");
    assert!(
        (*lateral_low - *lateral_high).abs() <= fixture.tolerance_abs,
        "legacy pool perturbation must not change lateral realized withdrawal"
    );

    let drainage_low = observed_withdrawals
        .get("drainage_low_legacy_pool")
        .expect("missing drainage low-legacy case");
    let drainage_high = observed_withdrawals
        .get("drainage_high_legacy_pool")
        .expect("missing drainage high-legacy case");
    assert!(
        (*drainage_low - *drainage_high).abs() <= fixture.tolerance_abs,
        "legacy pool perturbation must not change drainage realized withdrawal"
    );
}

#[test]
fn hphys0225_runtime_source_forbids_legacy_max_reconciliation() {
    let forbidden_lateral = "layer_pool.max(drainable_storage_legacy + recharge_pe)";
    let forbidden_drainage = "layer_pool.max(drainable_storage_legacy)";

    let findings = repo_hydrology_sources()
        .into_iter()
        .filter(|path| {
            repo_file_contains(path, forbidden_lateral)
                || repo_file_contains(path, forbidden_drainage)
        })
        .map(|path| {
            path.strip_prefix(env!("CARGO_MANIFEST_DIR"))
                .unwrap_or(path.as_path())
                .to_string_lossy()
                .to_string()
        })
        .collect::<Vec<_>>();

    assert!(
        findings.is_empty(),
        "HPHYS0225 must remove legacy max-reconciliation expressions from hydrology runtime sources; found {findings:?}",
    );

    let allowed_state_sources = repo_hydrology_sources();
    let has_layer_cap = allowed_state_sources
        .iter()
        .any(|path| repo_file_contains(path, "let available_pool = layer_pool;"));
    assert!(
        has_layer_cap,
        "HPHYS0225 must preserve available-pool cap assignment from layer-derived state"
    );
}
