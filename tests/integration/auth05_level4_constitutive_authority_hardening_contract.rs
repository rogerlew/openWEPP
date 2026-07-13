use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::runtime_inputs::project_typed_soil_wb11_runtime;
use openwepp_hillslope_orchestrator::{
    DirectDayFrame, DirectPercolationInputs, DirectRunIdentity, DirectRuntimeError,
    DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
};
use openwepp_input_contract::parsers::soil::{SoilParserOptions, parse_soil};
use serde::Deserialize;

const TOL: f64 = 1.0e-9;
const VALID_9002: &str = include_str!("../fixtures/infile/soil/valid_9002.sol");
const VALID_7778: &str = include_str!("../fixtures/infile/soil/valid_7778.sol");

#[derive(Debug, Deserialize)]
struct LayerFixture {
    theta_fc: Option<f64>,
    theta_wp: f64,
    porosity: f64,
    dg_m: f64,
}

#[derive(Debug, Deserialize)]
struct StorageExpected {
    profile_fc_store_mm: f64,
    profile_wp_store_mm: f64,
}

#[derive(Debug, Deserialize)]
struct ConstitutiveFixture {
    layers: Vec<LayerFixture>,
    expected: Option<StorageExpected>,
}

#[derive(Debug, Deserialize)]
struct RelaxFixture {
    theta_m: Option<f64>,
    fc_m: f64,
    ul_m: f64,
    ssc_m_s: f64,
}

fn repo_json<T: for<'de> Deserialize<'de>>(path: &str) -> T {
    let text = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("failed to parse {path}: {error}"))
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= TOL,
        "observed {actual}, expected {expected}"
    );
}

#[test]
fn auth05_fc_wp_fixtures_and_typed_runtime_preserve_constitutive_authority() {
    for path in [
        "tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/nominal_case.json",
        "tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/boundary_case.json",
        "tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/nominal_case.json",
        "tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/boundary_case.json",
    ] {
        let fixture: ConstitutiveFixture = repo_json(path);
        let expected = fixture
            .expected
            .expect("valid fixture must provide aggregate authority");
        let mut fc_mm = 0.0;
        let mut wp_mm = 0.0;
        for layer in fixture.layers {
            let fc = layer
                .theta_fc
                .expect("valid FC fixture must provide theta_fc");
            assert!(
                layer.porosity + TOL >= fc && fc + TOL >= layer.theta_wp && layer.theta_wp >= 0.0
            );
            fc_mm += fc * layer.dg_m * 1_000.0;
            wp_mm += layer.theta_wp * layer.dg_m * 1_000.0;
        }
        assert_close(fc_mm, expected.profile_fc_store_mm);
        assert_close(wp_mm, expected.profile_wp_store_mm);
    }

    for soil_text in [VALID_9002, VALID_7778] {
        let soil = parse_soil(soil_text, SoilParserOptions::default())
            .expect("authority soil fixture must parse");
        let projection = project_typed_soil_wb11_runtime(&soil)
            .expect("authority soil must project into current typed runtime");
        assert!(!projection.layers.is_empty());
        for layer in projection.layers {
            assert!(layer.porosity + TOL >= layer.thetfc);
            assert!(layer.thetfc + TOL >= layer.thetdr);
            assert!(layer.thetdr >= 0.0);
        }
    }
}

#[test]
fn auth05_invalid_fc_wp_and_missing_theta_are_fail_closed() {
    let missing: ConstitutiveFixture = repo_json(
        "tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/invalid_missing_theta_fc.json",
    );
    assert!(missing.layers.iter().any(|layer| layer.theta_fc.is_none()));

    let invalid: ConstitutiveFixture =
        repo_json("tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/invalid_wp_gt_fc.json");
    assert!(
        invalid
            .layers
            .iter()
            .any(|layer| layer.theta_fc.is_some_and(|fc| layer.theta_wp > fc))
    );

    let invalid_relax: RelaxFixture = repo_json(
        "tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/invalid_missing_theta.json",
    );
    assert!(invalid_relax.theta_m.is_none());
}

fn run_percolation(fixture: &RelaxFixture) -> Result<DirectDayFrame, DirectRuntimeError> {
    let theta = fixture
        .theta_m
        .ok_or(DirectRuntimeError::DirectDomainViolation {
            field: "percolation.theta_m",
        })?;
    let layer = DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
        theta_m: theta,
        field_capacity_m: fixture.fc_m,
        upper_limit_m: fixture.ul_m,
        conductivity_m_s: fixture.ssc_m_s,
        depth_m: 1.0,
        ..DirectSubsurfaceLayerInputs::neutral()
    });
    let identity = DirectRunIdentity::new(1, 1, 1, 1)?;
    let mut day = DirectDayFrame::seed(identity, 0, 0)?;
    day.percolation_inputs = DirectPercolationInputs {
        soil_water_initial_m: theta,
        layers: vec![layer],
        ..DirectPercolationInputs::neutral()
    };
    day.run_r4m_percolation_span()?;
    Ok(day)
}

#[test]
fn auth05_direct_percolation_obeys_fc_cutoff_and_positive_above_fc_response() {
    let cutoff: RelaxFixture =
        repo_json("tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/near_fc_cutoff.json");
    let cutoff_day = run_percolation(&cutoff).expect("near-FC case must execute");
    assert_close(cutoff_day.percolation.deep_seepage_m, 0.0);
    assert_close(cutoff_day.percolation.per_layer_flux_m[0], 0.0);

    let above: RelaxFixture = repo_json(
        "tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/above_fc_positive.json",
    );
    let above_day = run_percolation(&above).expect("above-FC case must execute");
    assert!(above_day.percolation.deep_seepage_m > 0.0);
    assert!(above_day.percolation.soil_water_after_m >= 0.0);
}
