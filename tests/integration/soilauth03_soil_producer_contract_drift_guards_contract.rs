use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use openwepp_input_contract::parsers::soil::{ParserMode, SoilParserOptions, parse_soil};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct SuiteCaseBinding {
    case_id: String,
    soil_file: String,
    expected_threshold_status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct SoilContractObligations {
    spec_path: String,
    parser_contract_path: String,
    required_symbols: Vec<String>,
    header_order_symbols: Vec<String>,
    header_arity_by_datver: BTreeMap<String, Vec<usize>>,
    policy_row_order_by_datver: BTreeMap<String, Vec<String>>,
    layer_arity_by_datver: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Deserialize)]
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
    soil_contract_obligations: Option<SoilContractObligations>,
}

#[derive(Debug, Deserialize)]
struct RequiredSuiteObligations {
    schema_version: u32,
    suites: Vec<SuiteObligation>,
}

#[derive(Debug, Clone, Deserialize)]
struct GuardCase {
    case_id: String,
    soil_file: String,
    expected_threshold_status: Option<String>,
    expected_datver: String,
    expected_policy_row_tokens: Option<usize>,
    expected_header_arity: usize,
    expected_layer_arity: usize,
    expected_policy_luse: Option<String>,
    expected_policy_stext: Option<String>,
    expected_restrictive_rows: usize,
}

#[derive(Debug, Deserialize)]
struct GuardCasesFixture {
    max_relative_error_threshold: f64,
    cases: Vec<GuardCase>,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_path(path: &str) -> PathBuf {
    repo_root().join(path)
}

fn read_repo_file(path: &str) -> String {
    let full_path = repo_path(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

fn read_repo_json<T: for<'de> Deserialize<'de>>(path: &str) -> T {
    let text = read_repo_file(path);
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("failed to parse JSON file {path}: {error}"))
}

fn tokenize_whitespace_and_quotes(line: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut iter = line.char_indices().peekable();

    while let Some((idx, ch)) = iter.peek().copied() {
        if ch.is_whitespace() {
            let _ = iter.next();
            continue;
        }

        if ch == '\'' || ch == '"' {
            let quote_char = ch;
            let _ = iter.next();
            let mut token = String::new();
            let mut closed = false;
            let mut escaped = false;

            for (_, quote_ch) in iter.by_ref() {
                if quote_char == '"' && escaped {
                    token.push(quote_ch);
                    escaped = false;
                    continue;
                }
                if quote_char == '"' && quote_ch == '\\' {
                    escaped = true;
                    continue;
                }
                if quote_ch == quote_char {
                    closed = true;
                    break;
                }
                token.push(quote_ch);
            }

            if !closed {
                return Err("unterminated quoted token".to_string());
            }
            if quote_char == '"' && escaped {
                return Err("unterminated escape in double-quoted token".to_string());
            }

            tokens.push(token);
            continue;
        }

        let token_start = idx;
        let mut token_end = line.len();
        while let Some((peek_idx, peek_ch)) = iter.peek().copied() {
            if peek_ch.is_whitespace() {
                token_end = peek_idx;
                break;
            }
            let _ = iter.next();
        }
        tokens.push(line[token_start..token_end].to_string());
    }

    Ok(tokens)
}

fn soil_lines(path: &str) -> Vec<String> {
    read_repo_file(path)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(std::string::ToString::to_string)
        .collect()
}

fn validate_required_symbols(
    required_symbols: &[String],
    spec_text: &str,
    parser_contract_text: &str,
) -> Result<(), String> {
    for symbol in required_symbols {
        let symbol_backtick = format!("`{symbol}`");
        if !(spec_text.contains(&symbol_backtick) || spec_text.contains(symbol)) {
            return Err(format!("symbol missing from soil-file spec: {symbol}"));
        }
        if !(parser_contract_text.contains(&symbol_backtick)
            || parser_contract_text.contains(symbol))
        {
            return Err(format!(
                "symbol missing from parser contract SC-INFILE-SOIL-001: {symbol}"
            ));
        }
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn validate_case_structure(
    case: &GuardCase,
    obligations: &SoilContractObligations,
) -> Result<(), String> {
    let fixture_path = format!("tests/fixtures/infile/soil/{}", case.soil_file);
    let lines = soil_lines(&fixture_path);
    if lines.len() < 4 {
        return Err(format!("fixture {} has insufficient rows", case.soil_file));
    }

    let observed_datver = lines[0].trim();
    if observed_datver != case.expected_datver {
        return Err(format!(
            "{} datver mismatch: expected {} observed {}",
            case.case_id, case.expected_datver, observed_datver
        ));
    }

    let topology_tokens = tokenize_whitespace_and_quotes(&lines[2])?;
    if topology_tokens.len() < 2 {
        return Err(format!("{} topology line malformed", case.case_id));
    }
    let ntemp = topology_tokens[0]
        .parse::<usize>()
        .map_err(|error| format!("{} invalid ntemp: {error}", case.case_id))?;

    let allowed_header_arity = obligations
        .header_arity_by_datver
        .get(&case.expected_datver)
        .ok_or_else(|| {
            format!(
                "{} missing header arity obligation for datver {}",
                case.case_id, case.expected_datver
            )
        })?;
    let expected_layer_arity = obligations
        .layer_arity_by_datver
        .get(&case.expected_datver)
        .copied()
        .ok_or_else(|| {
            format!(
                "{} missing layer arity obligation for datver {}",
                case.case_id, case.expected_datver
            )
        })?;

    let policy_shape = obligations
        .policy_row_order_by_datver
        .get(&case.expected_datver)
        .cloned();

    let mut cursor = 3usize;
    let mut restrictive_rows = 0usize;

    for _ in 0..ntemp {
        if let Some(policy_order) = &policy_shape {
            if cursor >= lines.len() {
                return Err(format!("{} missing policy row", case.case_id));
            }
            let policy_tokens = tokenize_whitespace_and_quotes(&lines[cursor])?;
            if policy_tokens.len() != policy_order.len() {
                return Err(format!(
                    "{} policy row arity mismatch: expected {} observed {}",
                    case.case_id,
                    policy_order.len(),
                    policy_tokens.len()
                ));
            }
            if let Some(expected_policy_row_tokens) = case.expected_policy_row_tokens
                && policy_tokens.len() != expected_policy_row_tokens
            {
                return Err(format!(
                    "{} policy row token-count drift: expected {} observed {}",
                    case.case_id,
                    expected_policy_row_tokens,
                    policy_tokens.len()
                ));
            }
            if let Some(expected_luse) = &case.expected_policy_luse
                && policy_tokens
                    .get(1)
                    .is_none_or(|value| value != expected_luse)
            {
                return Err(format!(
                    "{} policy luse drift: expected {} observed {}",
                    case.case_id,
                    expected_luse,
                    policy_tokens.get(1).map_or("", std::string::String::as_str)
                ));
            }
            if let Some(expected_stext) = &case.expected_policy_stext
                && policy_tokens
                    .get(2)
                    .is_none_or(|value| value != expected_stext)
            {
                return Err(format!(
                    "{} policy stext drift: expected {} observed {}",
                    case.case_id,
                    expected_stext,
                    policy_tokens.get(2).map_or("", std::string::String::as_str)
                ));
            }
            cursor += 1;
        }

        if cursor >= lines.len() {
            return Err(format!("{} missing OFE header row", case.case_id));
        }
        let header_tokens = tokenize_whitespace_and_quotes(&lines[cursor])?;
        if !allowed_header_arity.contains(&header_tokens.len()) {
            return Err(format!(
                "{} header arity drift: allowed {:?} observed {}",
                case.case_id,
                allowed_header_arity,
                header_tokens.len()
            ));
        }
        if header_tokens.len() != case.expected_header_arity {
            return Err(format!(
                "{} header arity drift against case fixture: expected {} observed {}",
                case.case_id,
                case.expected_header_arity,
                header_tokens.len()
            ));
        }
        let nsl = header_tokens
            .get(2)
            .ok_or_else(|| format!("{} header missing nsl position", case.case_id))?
            .parse::<usize>()
            .map_err(|error| format!("{} invalid nsl token: {error}", case.case_id))?;
        cursor += 1;

        for _ in 0..nsl {
            if cursor >= lines.len() {
                return Err(format!("{} missing layer row", case.case_id));
            }
            let layer_tokens = tokenize_whitespace_and_quotes(&lines[cursor])?;
            if layer_tokens.len() != expected_layer_arity {
                return Err(format!(
                    "{} layer arity drift: expected {} observed {}",
                    case.case_id,
                    expected_layer_arity,
                    layer_tokens.len()
                ));
            }
            if layer_tokens.len() != case.expected_layer_arity {
                return Err(format!(
                    "{} case layer-arity drift: expected {} observed {}",
                    case.case_id,
                    case.expected_layer_arity,
                    layer_tokens.len()
                ));
            }
            cursor += 1;
        }

        if cursor < lines.len() {
            let probe_tokens = tokenize_whitespace_and_quotes(&lines[cursor])?;
            let looks_like_restrictive = probe_tokens.len() == 3
                && (probe_tokens[0] == "0" || probe_tokens[0] == "1")
                && probe_tokens[1].parse::<f64>().is_ok()
                && probe_tokens[2].parse::<f64>().is_ok();
            if looks_like_restrictive {
                restrictive_rows += 1;
                cursor += 1;
            }
        }
    }

    if cursor != lines.len() {
        return Err(format!(
            "{} has unconsumed rows after structural parse ({cursor} of {})",
            case.case_id,
            lines.len()
        ));
    }

    if restrictive_rows != case.expected_restrictive_rows {
        return Err(format!(
            "{} restrictive-row count drift: expected {} observed {}",
            case.case_id, case.expected_restrictive_rows, restrictive_rows
        ));
    }

    Ok(())
}

fn assert_fixture_lock_integrity(fixture_root: &Path) {
    let status = Command::new("sha256sum")
        .arg("--check")
        .arg("--strict")
        .arg("fixtures.sha256")
        .current_dir(fixture_root)
        .status()
        .unwrap_or_else(|error| {
            panic!(
                "failed to run sha256sum in {}: {error}",
                fixture_root.display()
            )
        });
    assert!(
        status.success(),
        "fixtures.sha256 check failed in {}",
        fixture_root.display()
    );
}

fn load_soilauth03_suite() -> SuiteObligation {
    let obligations: RequiredSuiteObligations =
        read_repo_json("docs/specifications/external-authority/required-suite-obligations.json");
    assert_eq!(obligations.schema_version, 1);

    obligations
        .suites
        .iter()
        .find(|suite| suite.suite_id == "cas_l4_infile_soil_producer_contract_001")
        .unwrap_or_else(|| panic!("SOILAUTH03 suite obligations entry must exist"))
        .clone()
}

fn load_soilauth03_fixture_and_contract_obligations(
    suite: &SuiteObligation,
) -> (GuardCasesFixture, SoilContractObligations) {
    let fixture: GuardCasesFixture = read_repo_json(&suite.cohort_fixture);
    let obligations_map = suite
        .soil_contract_obligations
        .clone()
        .unwrap_or_else(|| panic!("SOILAUTH03 suite must publish soil_contract_obligations"));
    (fixture, obligations_map)
}

fn assert_suite_paths_and_required_files_exist(suite: &SuiteObligation) {
    assert!(
        !suite.closure_follow_on_package_id.trim().is_empty(),
        "closure_follow_on_package_id must be present for traceability"
    );
    for required_path in [
        suite.registry_path.as_str(),
        suite.suite_doc.as_str(),
        suite.integration_test.as_str(),
        suite.cohort_fixture.as_str(),
        suite.closure_follow_on_package_path.as_str(),
        suite.closure_follow_on_queue_path.as_str(),
        suite.lane_change_control_path.as_str(),
    ] {
        assert!(
            repo_path(required_path).exists(),
            "missing path: {required_path}"
        );
    }
    for required_fixture in &suite.required_fixture_files {
        assert!(
            repo_path(required_fixture).exists(),
            "missing required fixture file: {required_fixture}"
        );
    }
}

fn assert_symbol_and_header_obligations(
    obligations_map: &SoilContractObligations,
    spec_text: &str,
    parser_contract_text: &str,
) {
    validate_required_symbols(
        &obligations_map.required_symbols,
        spec_text,
        parser_contract_text,
    )
    .unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(
        obligations_map.header_order_symbols,
        vec![
            "slid", "texid", "nsl", "salb", "sat", "ki", "kr", "shcrit", "avke"
        ],
        "header order obligations must remain canonical"
    );
}

fn assert_fixture_threshold_and_case_floor(suite: &SuiteObligation, fixture: &GuardCasesFixture) {
    assert!(
        fixture.max_relative_error_threshold <= suite.max_relative_error_threshold_upper_bound,
        "guard fixture threshold must not exceed suite upper bound"
    );
    assert!(
        fixture.cases.len() >= suite.min_case_count,
        "guard case count must not shrink below declared minimum"
    );
}

fn assert_required_case_bindings(
    suite: &SuiteObligation,
    fixture: &GuardCasesFixture,
) -> BTreeSet<String> {
    let required_binding_map: BTreeMap<&str, &SuiteCaseBinding> = suite
        .required_case_bindings
        .iter()
        .map(|binding| (binding.case_id.as_str(), binding))
        .collect();
    let fixture_case_map: BTreeMap<&str, &GuardCase> = fixture
        .cases
        .iter()
        .map(|case| (case.case_id.as_str(), case))
        .collect();

    assert_eq!(
        required_binding_map.len(),
        3,
        "SOILAUTH03 must preserve three required canonical fixture anchors"
    );
    let mut observed_files = BTreeSet::new();
    for (case_id, binding) in &required_binding_map {
        let observed = fixture_case_map
            .get(case_id)
            .unwrap_or_else(|| panic!("missing required fixture case {case_id}"));
        assert_eq!(
            observed.soil_file, binding.soil_file,
            "required case {case_id} soil file binding changed"
        );
        if let Some(expected_status) = &binding.expected_threshold_status {
            assert_eq!(
                observed
                    .expected_threshold_status
                    .as_deref()
                    .unwrap_or_default(),
                expected_status,
                "required case {case_id} threshold status changed"
            );
        }
        let _ = observed_files.insert(binding.soil_file.clone());
    }
    observed_files
}

fn assert_case_structure_and_parser_acceptance(
    fixture: &GuardCasesFixture,
    obligations_map: &SoilContractObligations,
) {
    for case in &fixture.cases {
        validate_case_structure(case, obligations_map)
            .unwrap_or_else(|error| panic!("{}: {error}", case.case_id));
        let input = read_repo_file(&format!("tests/fixtures/infile/soil/{}", case.soil_file));
        parse_soil(&input, SoilParserOptions::default()).unwrap_or_else(|error| {
            panic!(
                "strict parser must accept canonical SOILAUTH03 fixture {}: {error:?}",
                case.soil_file
            )
        });
        parse_soil(
            &input,
            SoilParserOptions {
                mode: ParserMode::Compatibility,
                allow_legacy_aliases: true,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .unwrap_or_else(|error| {
            panic!(
                "compat parser must accept canonical SOILAUTH03 fixture {}: {error:?}",
                case.soil_file
            )
        });
    }
}

fn assert_suite_registry_and_doc_posture() {
    let registry = read_repo_file("docs/specifications/external-authority/registry.yaml");
    assert!(
        registry.contains("suite_id: cas_l4_infile_soil_producer_contract_001")
            && registry.contains("gate_lane: required")
            && registry.contains("failure_class: hard-fail")
            && registry.contains(
                "integration_test: tests/integration/soilauth03_soil_producer_contract_drift_guards_contract.rs"
            ),
        "registry must include required/hard-fail SOILAUTH03 suite posture"
    );

    let suite_doc = read_repo_file(
        "docs/specifications/external-authority/suites/cas_l4_infile_soil_producer_contract_001.md",
    );
    assert!(
        suite_doc.contains("required")
            && suite_doc.contains("hard-fail")
            && suite_doc.contains("required-suite-obligations.json")
            && suite_doc.contains("SC-INFILE-SOIL-001"),
        "suite doc must publish required hard-fail posture and contract linkage"
    );
}

fn assert_soilauth03_fixture_lock_and_provenance() {
    let fixture_root = repo_path("tests/fixtures/infile/soil");
    assert_fixture_lock_integrity(&fixture_root);
    let provenance_text = read_repo_file("tests/fixtures/infile/soil/fixtures.provenance.yaml");
    for required_key in [
        "schema_version:",
        "suite_id: cas_l4_infile_soil_producer_contract_001",
        "source_repo:",
        "source_commit:",
        "source_path:",
        "source_sha256:",
        "transform_note:",
    ] {
        assert!(
            provenance_text.contains(required_key),
            "provenance file missing required key: {required_key}"
        );
    }
}

fn assert_tampered_lock_fails() {
    let fixture_root = repo_path("tests/fixtures/infile/soil");
    let lock_path = fixture_root.join("fixtures.sha256");
    let lock_text = fs::read_to_string(&lock_path).unwrap_or_else(|error| {
        panic!(
            "expected readable lock file {}: {error}",
            lock_path.display()
        )
    });
    let mut lines = lock_text.lines();
    let first_line = lines
        .next()
        .unwrap_or_else(|| panic!("fixtures.sha256 must not be empty"));
    let mut first_fields = first_line.split_whitespace();
    let digest = first_fields
        .next()
        .unwrap_or_else(|| panic!("first lock entry must contain digest"));
    let fixture_file = first_fields
        .next()
        .unwrap_or_else(|| panic!("first lock entry must contain fixture path"));
    let replacement = if digest.starts_with('0') { '1' } else { '0' };
    let tampered_digest = format!("{replacement}{}", &digest[1..]);

    let mut tampered = format!("{tampered_digest}  {fixture_file}\n");
    for line in lines {
        tampered.push_str(line);
        tampered.push('\n');
    }
    let temp_name = format!("fixtures.soilauth03.bad.{}.sha256", std::process::id());
    let temp_path = fixture_root.join(&temp_name);
    fs::write(&temp_path, tampered).unwrap_or_else(|error| {
        panic!(
            "failed to write tampered lock file {}: {error}",
            temp_path.display()
        )
    });

    let status = Command::new("sha256sum")
        .arg("--check")
        .arg("--strict")
        .arg(&temp_name)
        .current_dir(&fixture_root)
        .status()
        .unwrap_or_else(|error| {
            panic!(
                "failed to run tamper lock check in {}: {error}",
                fixture_root.display()
            )
        });
    let _ = fs::remove_file(&temp_path);

    assert!(
        !status.success(),
        "tampered fixture lock must fail sha256 verification"
    );
}

#[test]
fn soilauth03_required_suite_obligations_and_contract_guards_hold() {
    let suite = load_soilauth03_suite();
    assert_suite_paths_and_required_files_exist(&suite);

    let (fixture, obligations_map) = load_soilauth03_fixture_and_contract_obligations(&suite);
    let spec_text = read_repo_file(&obligations_map.spec_path);
    let parser_contract_text = read_repo_file(&obligations_map.parser_contract_path);
    assert_symbol_and_header_obligations(&obligations_map, &spec_text, &parser_contract_text);
    assert_fixture_threshold_and_case_floor(&suite, &fixture);
    let observed_files = assert_required_case_bindings(&suite, &fixture);
    assert_case_structure_and_parser_acceptance(&fixture, &obligations_map);
    assert_eq!(
        observed_files.len(),
        3,
        "SOILAUTH03 guard fixture coverage must include the three canonical fixtures"
    );
    assert_suite_registry_and_doc_posture();
    assert_soilauth03_fixture_lock_and_provenance();
}

#[test]
fn soilauth03_injected_drift_vectors_fail_guards() {
    let suite = load_soilauth03_suite();
    let (fixture, mut soil_obligations) = load_soilauth03_fixture_and_contract_obligations(&suite);

    let spec_text = read_repo_file(&soil_obligations.spec_path);
    let parser_contract_text = read_repo_file(&soil_obligations.parser_contract_path);

    soil_obligations
        .required_symbols
        .retain(|symbol| symbol != "kslast");
    let symbol_guard_result = validate_required_symbols(
        &soil_obligations.required_symbols,
        &spec_text,
        &parser_contract_text,
    );
    assert!(
        symbol_guard_result.is_ok(),
        "symbol check with reduced list should still validate listed symbols"
    );
    assert!(
        !soil_obligations
            .required_symbols
            .contains(&"kslast".to_string()),
        "injected drift must remove kslast from required symbol map"
    );

    let mut drifted_obligations = suite
        .soil_contract_obligations
        .clone()
        .unwrap_or_else(|| panic!("SOILAUTH03 suite must publish soil_contract_obligations"));
    drifted_obligations
        .header_arity_by_datver
        .insert("9002".to_string(), vec![9]);
    let policy_case = fixture
        .cases
        .iter()
        .find(|case| case.case_id == "canonical_9002_policy_first")
        .unwrap_or_else(|| panic!("expected canonical_9002_policy_first case in guard fixture"));
    let arity_result = validate_case_structure(policy_case, &drifted_obligations);
    assert!(
        arity_result.is_err(),
        "injecting invalid 9002 header-arity obligations must fail guard validation"
    );
    assert_tampered_lock_fails();
}
