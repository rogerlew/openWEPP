use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SuiteCaseBinding {
    case_id: String,
    soil_file: String,
    expected_threshold_status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SuiteObligation {
    suite_id: String,
    registry_path: String,
    suite_doc: String,
    integration_test: String,
    cohort_fixture: String,
    required_fixture_files: Vec<String>,
    closure_follow_on_package_id: String,
    closure_follow_on_package_path: String,
    closure_follow_on_queue_path: String,
    max_relative_error_threshold_upper_bound: f64,
    min_case_count: usize,
    required_case_bindings: Vec<SuiteCaseBinding>,
    lane_change_control_path: String,
}

#[derive(Debug, Deserialize)]
struct RequiredSuiteObligations {
    schema_version: u32,
    suites: Vec<SuiteObligation>,
}

#[derive(Debug, Deserialize)]
struct CohortCase {
    case_id: String,
    soil_file: String,
    expected_threshold_status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CohortFixture {
    max_relative_error_threshold: f64,
    cases: Vec<CohortCase>,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_repo_file(path: &str) -> String {
    let full_path = repo_root().join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

fn read_repo_json<T: for<'de> Deserialize<'de>>(path: &str) -> T {
    let text = read_repo_file(path);
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("failed to parse JSON fixture {path}: {error}"))
}

fn exists(path: &str) -> bool {
    Path::new(&repo_root().join(path)).exists()
}

fn parse_package_state(package_md: &str) -> Option<String> {
    for line in package_md.lines() {
        let stripped = line.trim();
        if let Some(value) = stripped.strip_prefix("- state:") {
            return Some(value.trim().to_string());
        }
        if let Some(value) = stripped.strip_prefix("state:") {
            return Some(value.trim().to_string());
        }
    }
    None
}

#[test]
fn auth11_obligations_schema_and_anchor_bindings_are_enforced() {
    let obligations: RequiredSuiteObligations =
        read_repo_json("docs/specifications/external-authority/required-suite-obligations.json");
    assert_eq!(obligations.schema_version, 1);

    let suite = obligations
        .suites
        .iter()
        .find(|suite| suite.suite_id == "cas_l4_soil_fc_direct_theta_minus33_cohort_001")
        .unwrap_or_else(|| panic!("required obligations must include direct-theta FC suite entry"));

    assert!(exists(&suite.registry_path));
    assert!(exists(&suite.suite_doc));
    assert!(exists(&suite.integration_test));
    assert!(exists(&suite.cohort_fixture));
    assert!(exists(&suite.lane_change_control_path));
    assert!(exists(&suite.closure_follow_on_package_path));
    assert!(exists(&suite.closure_follow_on_queue_path));
    assert!(
        !suite.closure_follow_on_package_id.trim().is_empty(),
        "closure_follow_on_package_id must be non-empty"
    );
    for fixture_path in &suite.required_fixture_files {
        assert!(
            exists(fixture_path),
            "required fixture must exist: {fixture_path}"
        );
    }

    let fixture: CohortFixture = read_repo_json(&suite.cohort_fixture);
    assert!(
        fixture.max_relative_error_threshold <= suite.max_relative_error_threshold_upper_bound,
        "cohort threshold must not exceed obligations upper bound"
    );
    assert!(
        fixture.cases.len() >= suite.min_case_count,
        "cohort case count must not shrink below obligations minimum"
    );

    let case_map: BTreeMap<&str, &CohortCase> = fixture
        .cases
        .iter()
        .map(|case| (case.case_id.as_str(), case))
        .collect();

    for binding in &suite.required_case_bindings {
        let observed = case_map.get(binding.case_id.as_str()).unwrap_or_else(|| {
            panic!(
                "required case missing from cohort fixture: {}",
                binding.case_id
            )
        });
        assert_eq!(
            observed.soil_file, binding.soil_file,
            "required case {} must keep stable soil_file binding",
            binding.case_id
        );
        if let Some(expected_status) = &binding.expected_threshold_status {
            assert_eq!(
                observed.expected_threshold_status.as_deref().unwrap_or(""),
                expected_status,
                "required case {} must keep stable expected_threshold_status binding",
                binding.case_id
            );
        }
    }

    let package_md = read_repo_file(&suite.closure_follow_on_package_path);
    let package_state = parse_package_state(&package_md).unwrap_or_default();
    assert!(
        package_state == "queued" || package_state == "in_progress",
        "closure follow-on package must stay queued or in_progress while suite is non-blocking"
    );

    let queue_index = read_repo_file(&suite.closure_follow_on_queue_path);
    assert!(
        queue_index.contains(&suite.closure_follow_on_package_id),
        "closure follow-on package id must be present in queue index"
    );
}

#[test]
fn auth11_registry_posture_and_protocol_guard_paths_exist() {
    let registry = read_repo_file("docs/specifications/external-authority/registry.yaml");
    assert!(
        registry.contains("cas_l4_soil_fc_direct_theta_minus33_cohort_001"),
        "registry must contain direct-theta FC suite"
    );
    assert!(
        registry.contains("gate_lane: periodic")
            && registry.contains("failure_class: investigation"),
        "direct-theta FC suite must remain non-blocking until promotion protocol closure"
    );

    let protocol = read_repo_file("docs/specifications/external-authority/promotion-protocol.md");
    assert!(
        protocol.contains("Red-first capture")
            && protocol.contains("Green confirmation")
            && protocol.contains("check_authority_suite_antievasion.sh")
            && protocol.contains("closure_follow_on_package_id"),
        "promotion protocol must codify red/fix/green and anti-evasion execution"
    );
}
