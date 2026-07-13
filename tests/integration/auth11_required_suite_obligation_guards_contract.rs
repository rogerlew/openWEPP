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

fn parse_registry_suite_fields(
    registry_yaml: &str,
    suite_id: &str,
) -> Option<BTreeMap<String, String>> {
    let mut in_suite_block = false;
    let mut fields = BTreeMap::new();
    for line in registry_yaml.lines() {
        let stripped = line.trim();
        if let Some(observed_suite_id) = stripped.strip_prefix("- suite_id:") {
            if in_suite_block {
                break;
            }
            in_suite_block = observed_suite_id.trim() == suite_id;
            if in_suite_block {
                fields.insert("suite_id".to_string(), observed_suite_id.trim().to_string());
            }
            continue;
        }
        if !in_suite_block {
            continue;
        }
        if let Some((key, value)) = stripped.split_once(':') {
            fields.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    if fields.is_empty() {
        None
    } else {
        Some(fields)
    }
}

fn parse_registry_suites(registry_yaml: &str) -> Vec<BTreeMap<String, String>> {
    let mut suites = Vec::new();
    let mut current = BTreeMap::new();
    for line in registry_yaml.lines() {
        let stripped = line.trim();
        if let Some(suite_id) = stripped.strip_prefix("- suite_id:") {
            if !current.is_empty() {
                suites.push(current);
                current = BTreeMap::new();
            }
            current.insert("suite_id".to_string(), suite_id.trim().to_string());
            continue;
        }
        if current.is_empty() {
            continue;
        }
        if let Some((key, value)) = stripped.split_once(':') {
            current.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    if !current.is_empty() {
        suites.push(current);
    }
    suites
}

#[test]
fn auth11_all_active_required_suite_targets_exist_and_are_registered() {
    let registry = read_repo_file("docs/specifications/external-authority/registry.yaml");
    let cargo_manifest = read_repo_file("Cargo.toml");
    let mut checked_suite_count = 0_usize;

    for suite in parse_registry_suites(&registry) {
        if suite.get("status").map(String::as_str) != Some("active")
            || suite.get("gate_lane").map(String::as_str) != Some("required")
            || suite.get("failure_class").map(String::as_str) != Some("hard-fail")
        {
            continue;
        }
        checked_suite_count += 1;
        let suite_id = suite.get("suite_id").map_or("<unknown>", String::as_str);
        let integration_test = suite.get("integration_test").unwrap_or_else(|| {
            panic!("active required suite {suite_id} must name integration_test")
        });
        assert!(
            integration_test.starts_with("tests/integration/")
                && Path::new(integration_test)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("rs")),
            "active required suite {suite_id} integration_test must be a Rust integration target: {integration_test}"
        );
        assert!(
            exists(integration_test),
            "active required suite {suite_id} integration target must exist: {integration_test}"
        );
        let target = Path::new(integration_test)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_else(|| {
                panic!("invalid integration_test path for {suite_id}: {integration_test}")
            });
        assert!(
            cargo_manifest.contains(&format!(
                "[[test]]\nname = \"{target}\"\npath = \"{integration_test}\""
            )),
            "active required suite {suite_id} target must be explicitly registered in Cargo.toml: {target}"
        );
    }

    assert!(
        checked_suite_count > 0,
        "registry must expose at least one active required hard-fail suite"
    );
}

#[allow(clippy::too_many_lines)]
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

    let registry = read_repo_file("docs/specifications/external-authority/registry.yaml");
    let suite_registry_fields =
        parse_registry_suite_fields(&registry, "cas_l4_soil_fc_direct_theta_minus33_cohort_001")
            .unwrap_or_else(|| panic!("registry must include direct-theta FC suite block"));
    let gate_lane = suite_registry_fields
        .get("gate_lane")
        .map_or("", std::string::String::as_str);
    let failure_class = suite_registry_fields
        .get("failure_class")
        .map_or("", std::string::String::as_str);

    let package_md = read_repo_file(&suite.closure_follow_on_package_path);
    let package_state = parse_package_state(&package_md).unwrap_or_default();
    match (gate_lane, failure_class) {
        ("periodic", "investigation") => {
            assert!(
                package_state == "queued" || package_state == "in_progress",
                "closure follow-on package must stay queued or in_progress while suite is non-blocking"
            );
        }
        ("required", "hard-fail") => {
            assert_eq!(
                package_state, "complete",
                "closure follow-on package must be complete after direct-theta suite promotion"
            );
        }
        _ => {
            panic!(
                "unexpected direct-theta suite posture in registry: gate_lane={gate_lane}, failure_class={failure_class}"
            );
        }
    }

    let follow_on_package_dir = Path::new(&suite.closure_follow_on_package_path)
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        .unwrap_or_else(|| {
            panic!(
                "invalid closure follow-on package path: {}",
                suite.closure_follow_on_package_path
            )
        });
    assert_eq!(
        follow_on_package_dir, suite.closure_follow_on_package_id,
        "closure follow-on package directory name must match package id"
    );
}

#[test]
fn auth11_registry_posture_and_protocol_guard_paths_exist() {
    let registry = read_repo_file("docs/specifications/external-authority/registry.yaml");
    let suite_registry_fields =
        parse_registry_suite_fields(&registry, "cas_l4_soil_fc_direct_theta_minus33_cohort_001")
            .unwrap_or_else(|| panic!("registry must contain direct-theta FC suite"));
    let gate_lane = suite_registry_fields
        .get("gate_lane")
        .map_or("", std::string::String::as_str);
    let failure_class = suite_registry_fields
        .get("failure_class")
        .map_or("", std::string::String::as_str);
    let recognized_posture = matches!(
        (gate_lane, failure_class),
        ("periodic", "investigation") | ("required", "hard-fail")
    );
    assert!(
        recognized_posture,
        "direct-theta FC suite posture must be either periodic/investigation (pre-closure) or required/hard-fail (post-closure); observed {gate_lane}/{failure_class}"
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
