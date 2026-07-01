use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::runtime_inputs::project_typed_soil_profile_publication;
use openwepp_input_contract::parsers::soil::{ParserMode, SoilParserOptions, parse_soil};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BucketThresholds {
    low_max: f64,
    medium_max: f64,
}

#[derive(Debug, Deserialize)]
struct CohortCase {
    case_id: String,
    soil_file: String,
    expected_rock_bucket: String,
    expected_threshold_status: String,
}

#[derive(Debug, Deserialize)]
struct CohortFixture {
    suite_id: String,
    units_basis: String,
    max_relative_error_threshold: f64,
    rock_fragment_bucket_thresholds_pct: BucketThresholds,
    cases: Vec<CohortCase>,
}

#[derive(Debug, Clone)]
struct CaseResult {
    case_id: String,
    rock_bucket: String,
    weighted_rock_pct: f64,
    authority_fc_store_mm: f64,
    model_fc_store_mm: f64,
    relative_error: f64,
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

fn strict_soil_parser_options() -> SoilParserOptions {
    SoilParserOptions {
        mode: ParserMode::Strict,
        allow_legacy_aliases: false,
        expected_topology_count: None,
        topology_scope: None,
    }
}

fn rock_bucket(rock_pct: f64, thresholds: &BucketThresholds) -> String {
    if rock_pct <= thresholds.low_max {
        "low".to_string()
    } else if rock_pct <= thresholds.medium_max {
        "medium".to_string()
    } else {
        "high".to_string()
    }
}

fn evaluate_case(case: &CohortCase, fixture: &CohortFixture) -> CaseResult {
    let root = "tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001";
    let soil_text = repo_file(&format!("{root}/{}", case.soil_file));
    let soil = parse_soil(&soil_text, strict_soil_parser_options())
        .unwrap_or_else(|error| panic!("{} should parse: {error}", case.case_id));
    let projection = project_typed_soil_profile_publication(&soil).unwrap_or_else(|error| {
        panic!(
            "{} typed soil profile publication projection should build: {error}",
            case.case_id
        )
    });

    let ofe = soil
        .ofes
        .first()
        .unwrap_or_else(|| panic!("{} missing first OFE", case.case_id));
    let mut previous_depth_mm = 0.0_f64;
    let mut authority_fc_store_mm = 0.0_f64;
    let mut weighted_rock_sum = 0.0_f64;
    let mut total_thickness_mm = 0.0_f64;
    for (layer_position, layer) in ofe.layers.iter().enumerate() {
        let layer_index = layer_position + 1;
        let thickness_mm = layer.depth_mm - previous_depth_mm;
        previous_depth_mm = layer.depth_mm;
        assert!(
            thickness_mm > 0.0,
            "{} layer {} non-positive thickness",
            case.case_id,
            layer_index
        );
        let theta_fc = layer.fc_measured.unwrap_or_else(|| {
            panic!("{} layer {} missing fc_measured", case.case_id, layer_index)
        });
        authority_fc_store_mm += theta_fc * thickness_mm;
        weighted_rock_sum += layer.rock_frag_pct * thickness_mm;
        total_thickness_mm += thickness_mm;
    }

    let weighted_rock_pct = if total_thickness_mm > 0.0 {
        weighted_rock_sum / total_thickness_mm
    } else {
        0.0
    };
    let mut model_fc_store_mm = 0.0_f64;
    assert_eq!(
        projection.nsl,
        projection.layers.len(),
        "{} typed nsl should match typed layer count",
        case.case_id
    );
    for layer in &projection.layers {
        model_fc_store_mm += layer.thetfc * layer.dg_m * 1_000.0;
    }
    let relative_error =
        (model_fc_store_mm - authority_fc_store_mm).abs() / authority_fc_store_mm.max(f64::EPSILON);
    CaseResult {
        case_id: case.case_id.clone(),
        rock_bucket: rock_bucket(
            weighted_rock_pct,
            &fixture.rock_fragment_bucket_thresholds_pct,
        ),
        weighted_rock_pct,
        authority_fc_store_mm,
        model_fc_store_mm,
        relative_error,
    }
}

fn assert_auth11_anchor_bindings(fixture: &CohortFixture) {
    assert!(
        fixture.cases.iter().any(
            |case| case.case_id == "valid_9002_reference" && case.soil_file == "valid_9002.sol"
        ),
        "AUTH11 anchor guard: valid_9002_reference fixture binding must remain present"
    );
    assert!(
        fixture.cases.iter().any(
            |case| case.case_id == "valid_7778_reference" && case.soil_file == "valid_7778.sol"
        ),
        "AUTH11 anchor guard: valid_7778_reference fixture binding must remain present"
    );
    assert!(
        fixture.cases.iter().any(|case| {
            case.case_id == "h1_synthetic_low_rock_authority"
                && case.soil_file == "h1_high_rock_fc_authority.sol"
        }),
        "AUTH11 anchor guard: synthetic H1 fixture binding must remain present"
    );
    assert!(
        fixture.cases.iter().any(|case| {
            case.case_id == "h1_real_rocky_authority"
                && case.soil_file == "h1_real_rocky_p1_authority.sol"
        }),
        "AUTH12 anchor guard: rocky H1 fixture binding must remain present"
    );
}

fn collect_case_results(fixture: &CohortFixture) -> Vec<CaseResult> {
    fixture
        .cases
        .iter()
        .map(|case| evaluate_case(case, fixture))
        .collect()
}

fn collect_case_mismatches(fixture: &CohortFixture, results: &[CaseResult]) -> Vec<String> {
    let mut mismatches = Vec::new();
    for case in &fixture.cases {
        let result = results
            .iter()
            .find(|result| result.case_id == case.case_id)
            .unwrap_or_else(|| panic!("missing evaluated result for {}", case.case_id));
        if result.rock_bucket != case.expected_rock_bucket {
            mismatches.push(format!(
                "{} bucket mismatch: expected={} observed={} (weighted rock={}%)",
                result.case_id,
                case.expected_rock_bucket,
                result.rock_bucket,
                result.weighted_rock_pct
            ));
        }
        let observed_threshold_status =
            if result.relative_error > fixture.max_relative_error_threshold {
                "exceeds"
            } else {
                "within"
            };
        if observed_threshold_status != case.expected_threshold_status {
            mismatches.push(format!(
                "{} threshold status mismatch: expected={} observed={} (authority_fc_store_mm={} model_fc_store_mm={} rel_err={} threshold={})",
                result.case_id,
                case.expected_threshold_status,
                observed_threshold_status,
                result.authority_fc_store_mm,
                result.model_fc_store_mm,
                result.relative_error,
                fixture.max_relative_error_threshold,
            ));
        }
    }
    mismatches
}

fn assert_bucket_metrics(results: Vec<CaseResult>) {
    let mut buckets: BTreeMap<String, Vec<CaseResult>> = BTreeMap::new();
    for result in results {
        buckets
            .entry(result.rock_bucket.clone())
            .or_default()
            .push(result);
    }
    assert!(
        !buckets.is_empty(),
        "cohort must emit at least one rock bucket"
    );

    for (bucket, entries) in buckets {
        let entry_count_u32 = u32::try_from(entries.len())
            .unwrap_or_else(|_| panic!("bucket {bucket} entry count overflow"));
        let mean_rel_err = entries
            .iter()
            .map(|entry| entry.relative_error)
            .sum::<f64>()
            / f64::from(entry_count_u32);
        assert!(
            mean_rel_err.is_finite(),
            "bucket {bucket} mean relative error must be finite"
        );
        for entry in entries {
            assert!(
                entry.model_fc_store_mm.is_finite()
                    && entry.authority_fc_store_mm.is_finite()
                    && entry.weighted_rock_pct.is_finite(),
                "case {} must publish finite FC authority/model/rock metrics",
                entry.case_id
            );
        }
    }
}

#[test]
fn auth07_package_and_suite_authority_sections_exist() {
    let package = repo_file(
        "docs/work-packages/20260531-auth07-fc-authority-cohort-suite-bootstrap-001/package.md",
    );
    let registry = repo_file("docs/specifications/external-authority/registry.yaml");
    let suite = repo_file(
        "docs/specifications/external-authority/suites/cas_l4_soil_fc_direct_theta_minus33_cohort_001.md",
    );
    let soil_contract = repo_file("docs/specifications/science-contracts/contracts/SC-SOIL-001.md");

    assert!(
        package.contains("Objective")
            && package.contains("independent")
            && package.contains("rock-fragment stratified reporting"),
        "AUTH07 package must capture independent FC authority cohort scope"
    );
    assert!(
        registry.contains("cas_l4_soil_fc_direct_theta_minus33_cohort_001")
            && registry.contains("gate_lane: required")
            && registry.contains("failure_class: hard-fail"),
        "registry must include AUTH12 cohort suite in required hard-fail lane"
    );
    assert!(
        suite.contains("authority_level: 4")
            && suite.contains("hash:")
            && suite.contains("source_commit:")
            && suite.contains("transform_note:"),
        "AUTH11 cohort suite must include level-4 + fixture provenance metadata"
    );
    assert!(
        soil_contract.contains("AUTH12 FC Rocky-Soil Closure and Promotion Addendum"),
        "SC-SOIL-001 must include AUTH12 rocky-soil FC closure addendum"
    );
}

#[test]
fn auth07_profile_fc_authority_cohort_threshold_and_rock_bucket_classification() {
    let fixture: CohortFixture = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/cohort_case.json",
    );
    assert_eq!(
        fixture.suite_id,
        "cas_l4_soil_fc_direct_theta_minus33_cohort_001"
    );
    assert_eq!(fixture.units_basis, "m3_m3_and_mm");
    assert_auth11_anchor_bindings(&fixture);

    let results = collect_case_results(&fixture);
    let mismatches = collect_case_mismatches(&fixture, &results);
    assert!(
        mismatches.is_empty(),
        "AUTH07 cohort expectations mismatched:\n{}",
        mismatches.join("\n")
    );
    assert_bucket_metrics(results);
}
