use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    Wb11HydrologyKernel,
    runtime_inputs::{HillslopeRuntimeInputError, build_hillslope_runtime_surface_from_soil},
};
use openwepp_input_contract::parsers::soil::{SoilParserOptions, TopologyScope, parse_soil};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest,
};
use openwepp_runner::SidecarPolicy;
use openwepp_sim_contract::status::BoundaryClass;
use serde::Deserialize;

const TOL: f64 = 1.0e-12;
const VALID_9002: &str = include_str!("../fixtures/infile/soil/valid_9002.sol");

#[derive(Debug, Deserialize)]
struct FcWpLayerFixture {
    layer_id: String,
    theta_fc: Option<f64>,
    theta_wp: Option<f64>,
    porosity: Option<f64>,
    dg_m: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct FcWpExpectedFixture {
    profile_fc_store_mm: f64,
    profile_wp_store_mm: f64,
}

#[derive(Debug, Deserialize)]
struct FcWpFixture {
    case_id: String,
    pressure_head_kpa: f64,
    units_basis: String,
    layers: Vec<FcWpLayerFixture>,
    expected: Option<FcWpExpectedFixture>,
}

#[derive(Debug, Deserialize)]
struct RelaxExpectedFixture {
    pei_m: Option<f64>,
    d_m: Option<f64>,
    pei_positive: Option<bool>,
    d_non_negative: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct RelaxFixture {
    case_id: String,
    theta_m: Option<f64>,
    fc_m: f64,
    ul_m: f64,
    ssc_m_s: f64,
    expected: RelaxExpectedFixture,
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

#[test]
fn auth03_package_and_contract_authority_sections_exist() {
    let package = repo_file(
        "docs/work-packages/20260531-auth03-level4-constitutive-gate-bootstrap-001/package.md",
    );
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    let soil = repo_file("docs/specifications/science-contracts/contracts/SC-SOIL-001.md");
    let registry = repo_file("docs/specifications/external-authority/registry.yaml");
    let suite_fc =
        repo_file("docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md");
    let suite_wp =
        repo_file("docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md");
    let suite_relax =
        repo_file("docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md");

    assert!(
        package.contains("Mandatory Contract-First Sequence")
            && package.contains("Level-4 constitutive gates"),
        "AUTH03 package must preserve contract-first sequencing and Level-4 objective scope"
    );
    assert!(
        watbal.contains("### AUTH03 Level-4 Constitutive Gate Bootstrap Addendum"),
        "SC-WATBAL-001 must include AUTH03 constitutive gate addendum"
    );
    assert!(
        soil.contains("## AUTH03 Level-4 FC/WP Constitutive Authority Addendum")
            && soil.contains("INV-SOIL-014"),
        "SC-SOIL-001 must include AUTH03 FC/WP constitutive invariant authority"
    );
    assert!(
        registry.contains("cas_l4_soil_fc_minus33_001")
            && registry.contains("cas_l4_soil_wp_minus1500_001")
            && registry.contains("cas_l4_watbal_relax_to_fc_001"),
        "external-authority registry must list all AUTH03 Level-4 suites"
    );
    assert!(
        suite_fc.contains("gate_lane: required")
            && suite_fc.contains("failure_class: hard-fail")
            && suite_fc.contains("SC-SOIL-001#INV-SOIL-014"),
        "FC suite must be required hard-fail and linked to canonical invariant authority"
    );
    assert!(
        suite_wp.contains("gate_lane: required")
            && suite_wp.contains("failure_class: hard-fail")
            && suite_wp.contains("SC-SOIL-001#INV-SOIL-014"),
        "WP suite must be required hard-fail and linked to canonical invariant authority"
    );
    assert!(
        suite_relax.contains("gate_lane: required")
            && suite_relax.contains("failure_class: hard-fail")
            && suite_relax.contains("SC-WATBAL-001#INV-WATBAL-006"),
        "Relax-to-FC suite must be required hard-fail and linked to percolation threshold authority"
    );
}

#[test]
fn auth03_fc_wp_fixture_vectors_preserve_constitutive_ordering_and_storage_closure() {
    let fc_nominal: FcWpFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/nominal_case.json",
    );
    let fc_boundary: FcWpFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/boundary_case.json",
    );
    let wp_nominal: FcWpFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/nominal_case.json",
    );
    let wp_boundary: FcWpFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/boundary_case.json",
    );
    let wp_invalid: FcWpFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/invalid_wp_gt_fc.json",
    );
    let fc_invalid: FcWpFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/invalid_missing_theta_fc.json",
    );

    for fixture in [&fc_nominal, &fc_boundary, &wp_nominal, &wp_boundary] {
        assert!(
            fixture.pressure_head_kpa.is_finite() && fixture.pressure_head_kpa > 0.0,
            "fixture {} pressure head must be finite and positive",
            fixture.case_id
        );
        assert_eq!(
            fixture.units_basis, "m3_m3",
            "fixture {} must declare m3_m3 units basis",
            fixture.case_id
        );

        let mut fc_store_mm = 0.0_f64;
        let mut wp_store_mm = 0.0_f64;
        for layer in &fixture.layers {
            let theta_fc = layer.theta_fc.expect("expected theta_fc in valid fixture");
            let theta_wp = layer.theta_wp.expect("expected theta_wp in valid fixture");
            let porosity = layer.porosity.expect("expected porosity in valid fixture");
            let dg_m = layer.dg_m.expect("expected dg_m in valid fixture");

            assert!(
                porosity >= theta_fc && theta_fc >= theta_wp && theta_wp >= 0.0,
                "fixture {} layer {} must satisfy porosity >= theta_fc >= theta_wp >= 0, observed porosity={}, theta_fc={}, theta_wp={}",
                fixture.case_id,
                layer.layer_id,
                porosity,
                theta_fc,
                theta_wp
            );
            assert!(
                dg_m > 0.0,
                "fixture {} layer {} must have dg_m > 0",
                fixture.case_id,
                layer.layer_id
            );
            fc_store_mm += theta_fc * dg_m * 1_000.0;
            wp_store_mm += theta_wp * dg_m * 1_000.0;
        }

        if let Some(expected) = &fixture.expected {
            assert!(
                (fc_store_mm - expected.profile_fc_store_mm).abs() <= 1.0e-9,
                "fixture {} expected FC store mismatch: expected={}, observed={}",
                fixture.case_id,
                expected.profile_fc_store_mm,
                fc_store_mm
            );
            assert!(
                (wp_store_mm - expected.profile_wp_store_mm).abs() <= 1.0e-9,
                "fixture {} expected WP store mismatch: expected={}, observed={}",
                fixture.case_id,
                expected.profile_wp_store_mm,
                wp_store_mm
            );
        }
    }

    assert!(
        fc_invalid
            .layers
            .iter()
            .any(|layer| layer.theta_fc.is_none()),
        "invalid FC fixture must include missing theta_fc payload"
    );
    assert!(
        wp_invalid
            .layers
            .iter()
            .any(|layer| match (layer.theta_fc, layer.theta_wp) {
                (Some(theta_fc), Some(theta_wp)) => theta_wp > theta_fc,
                _ => false,
            }),
        "invalid WP fixture must include theta_wp > theta_fc payload"
    );
}

#[test]
fn auth03_relax_to_fc_kernel_vectors_cover_cutoff_and_positive_branch() {
    let near_fc: RelaxFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/near_fc_cutoff.json",
    );
    let above_fc: RelaxFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/above_fc_positive.json",
    );

    let mut kernel = Wb11HydrologyKernel;

    let near_response = kernel.run_hillslope_phase(&build_perc_request(&seeded_perc_state(
        near_fc.theta_m,
        near_fc.fc_m,
        near_fc.ul_m,
        near_fc.ssc_m_s,
    )));
    assert_eq!(
        near_response.status.message_id(),
        "HKERNEL-WB11-PERC-OK-001"
    );
    let near_pei = writeback_flux_value(&near_response, "wb18_perc_pei_0001");
    let near_d = writeback_flux_value(&near_response, "D");
    if let Some(expected_pei) = near_fc.expected.pei_m {
        assert!(
            (near_pei - expected_pei).abs() <= TOL,
            "near-FC pei mismatch: expected={expected_pei}, observed={near_pei}"
        );
    }
    if let Some(expected_d) = near_fc.expected.d_m {
        assert!(
            (near_d - expected_d).abs() <= TOL,
            "near-FC D mismatch: expected={expected_d}, observed={near_d}"
        );
    }

    let above_response = kernel.run_hillslope_phase(&build_perc_request(&seeded_perc_state(
        above_fc.theta_m,
        above_fc.fc_m,
        above_fc.ul_m,
        above_fc.ssc_m_s,
    )));
    assert_eq!(
        above_response.status.message_id(),
        "HKERNEL-WB11-PERC-OK-001"
    );
    let above_pei = writeback_flux_value(&above_response, "wb18_perc_pei_0001");
    let above_d = writeback_flux_value(&above_response, "D");
    if above_fc.expected.pei_positive == Some(true) {
        assert!(
            above_pei > 0.0,
            "above-FC pei must be positive, observed {above_pei}"
        );
    }
    if above_fc.expected.d_non_negative == Some(true) {
        assert!(
            above_d >= 0.0,
            "above-FC D must be non-negative, observed {above_d}"
        );
    }
}

#[test]
fn auth03_constitutive_symbol_missing_and_invalid_payloads_fail_closed() {
    let mut missing_fc_profile = parse_soil(VALID_9002, soil_parser_options_for_fixture())
        .expect("9002 fixture should parse");
    missing_fc_profile.ofes[0].layers[0].fc_rosetta = None;
    missing_fc_profile.ofes[0].layers[0].fc_measured = None;
    let missing_fc_error = build_hillslope_runtime_surface_from_soil(&missing_fc_profile)
        .expect_err("missing theta_fc source must fail closed");
    assert_eq!(missing_fc_error.code(), "HS-RUNTIME-E-004");
    assert!(matches!(
        missing_fc_error,
        HillslopeRuntimeInputError::MissingThetaFieldCapacity
    ));

    let mut nonfinite_fc_profile = parse_soil(VALID_9002, soil_parser_options_for_fixture())
        .expect("9002 fixture should parse");
    nonfinite_fc_profile.ofes[0].layers[0].fc_rosetta = None;
    nonfinite_fc_profile.ofes[0].layers[0].fc_measured = Some(f64::NAN);
    let nonfinite_fc_error = build_hillslope_runtime_surface_from_soil(&nonfinite_fc_profile)
        .expect_err("non-finite theta_fc source must fail closed");
    assert_eq!(nonfinite_fc_error.code(), "HS-RUNTIME-E-010");
    assert!(matches!(
        nonfinite_fc_error,
        HillslopeRuntimeInputError::NonFiniteThetaFieldCapacity { .. }
    ));

    let invalid_relax: RelaxFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/invalid_missing_theta.json",
    );
    assert_eq!(
        invalid_relax.case_id, "relax_invalid_missing_theta",
        "invalid relax fixture id must be stable"
    );
    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&build_perc_request(&seeded_perc_state(
        None,
        invalid_relax.fc_m,
        invalid_relax.ul_m,
        invalid_relax.ssc_m_s,
    )));
    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-E-001");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

fn soil_parser_options_for_fixture() -> SoilParserOptions {
    SoilParserOptions {
        mode: SidecarPolicy::Compat.as_soil_parser_mode(),
        allow_legacy_aliases: true,
        expected_topology_count: Some(1),
        topology_scope: Some(TopologyScope::Hillslope),
    }
}

fn seeded_perc_state(
    theta_m: Option<f64>,
    fc_m: f64,
    ul_m: f64,
    ssc_m_s: f64,
) -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state = BTreeMap::new();
    state.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    if let Some(theta) = theta_m {
        state.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(theta),
        );
        state.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(theta),
        );
    } else {
        state.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(fc_m),
        );
    }
    state.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(fc_m),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(ul_m),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(ssc_m_s),
    );
    state.insert(
        BoundarySymbol::from("wb11_field_capacity"),
        BoundaryValue::scalar(fc_m),
    );
    state.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(1.0),
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
