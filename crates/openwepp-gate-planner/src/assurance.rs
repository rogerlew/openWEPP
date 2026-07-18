use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};

use crate::canonical::derived_id;
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::policy::{AssuranceReport, AssuranceWatch, PolicyBundle};
use crate::repository::{CargoGraph, ObservedChange};

/// Construct immutable, exact-target assurance impact projections for a plan.
///
/// Every report in the registry is evaluated. Matches coalesce by changed
/// object and report while retaining every matching watch identity.
pub(crate) fn plan_assurance_impacts(
    policy: &PolicyBundle,
    graph: &CargoGraph,
    changes: &[ObservedChange],
    target_head: &str,
    campaign_id: Option<&str>,
    request_campaign_transfer: bool,
) -> Result<Vec<Value>> {
    let mut impacts = Vec::new();
    for change in changes {
        let contexts = semantic_contexts(policy, graph, change);
        let matches = policy
            .assurance_registry
            .reports
            .iter()
            .map(|report| {
                let matched = report
                    .watches
                    .iter()
                    .filter(|watch| watch_matches(watch, change, &contexts))
                    .collect::<Vec<_>>();
                (report, matched)
            })
            .collect::<Vec<_>>();
        for (report, matched) in matches {
            // A change without a governed classification for a report is an
            // impact-map defect, not evidence that the report is unaffected.
            let unknown = matched.is_empty();
            let watch_ids = matched
                .iter()
                .map(|watch| watch.watch_id.clone())
                .collect::<Vec<_>>();
            let lifecycle_boundaries = if unknown {
                vec!["CAMPAIGN_CLOSURE".to_owned()]
            } else {
                matched
                    .iter()
                    .map(|watch| watch.lifecycle_boundary.clone())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect()
            };
            let impact_match = ImpactMatch {
                watch_ids,
                lifecycle_boundaries,
                unknown,
            };
            impacts.push(impact_record(
                policy,
                report,
                change,
                target_head,
                campaign_id,
                request_campaign_transfer,
                &impact_match,
            )?);
        }
    }
    impacts.sort_by(|left, right| {
        (
            left["report_id"].as_str(),
            left["changed_object"]["path"].as_str(),
            left["impact_record_id"].as_str(),
        )
            .cmp(&(
                right["report_id"].as_str(),
                right["changed_object"]["path"].as_str(),
                right["impact_record_id"].as_str(),
            ))
    });
    Ok(impacts)
}

pub(crate) fn reconcile_assurance_impacts(
    intent: &Value,
    terminal: &Value,
    actual_paths: &BTreeSet<String>,
) -> Result<()> {
    let intent_impacts = impacts_by_key(intent)?;
    let terminal_impacts = impacts_by_key(terminal)?;
    for intended in intent_impacts.values() {
        let path = impact_string(intended, "/changed_object/path")?;
        if !actual_paths.contains(path) {
            continue;
        }
        let key = impact_key(intended)?;
        let actual = terminal_impacts
            .get(&key)
            .ok_or_else(|| assurance_error("GATE-TERMINAL-ASSURANCE-REMOVED", &key))?;
        let intended_watches = impact_ids(intended, "/matching_watch_ids")?;
        let actual_watches = impact_ids(actual, "/matching_watch_ids")?;
        if !intended_watches.is_subset(&actual_watches)
            || assurance_binding_weakened(intended, actual)
        {
            return Err(assurance_error("GATE-TERMINAL-ASSURANCE-WEAKENED", key));
        }
    }
    Ok(())
}

fn assurance_binding_weakened(intent: &Value, terminal: &Value) -> bool {
    intent["registry_generation"] != terminal["registry_generation"]
        || intent["registry_sha256"] != terminal["registry_sha256"]
        || intent["report_id"] != terminal["report_id"]
        || intent["source_root"] != terminal["source_root"]
        || intent["assessed_realization_root"] != terminal["assessed_realization_root"]
        || intent["campaign_id"] != terminal["campaign_id"]
        || intent["requested_action"] != terminal["requested_action"]
        || intent["watch_generation"] != terminal["watch_generation"]
        || intent["impact_state"] != terminal["impact_state"]
        || intent["resolution_authority"] != terminal["resolution_authority"]
        || intent["assessed_realization_integrity"] != terminal["assessed_realization_integrity"]
        || intent["campaign_impact_disposition"] != terminal["campaign_impact_disposition"]
        || request_weakened(
            &intent["campaign_transfer_request"],
            &terminal["campaign_transfer_request"],
        )
        || intent["campaign_transfer_currency"] != terminal["campaign_transfer_currency"]
        || request_weakened(
            &intent["release_transfer_request"],
            &terminal["release_transfer_request"],
        )
        || intent["release_transfer_currency"] != terminal["release_transfer_currency"]
        || !string_array_subset(
            &intent["lifecycle_boundaries"],
            &terminal["lifecycle_boundaries"],
        )
        || (intent["changed_object"]["change_kind"] != "DECLARED"
            && intent["changed_object"] != terminal["changed_object"])
        || intent["mapping_complete"] == true && terminal["mapping_complete"] != true
}

fn request_weakened(intent: &Value, terminal: &Value) -> bool {
    intent == "REQUESTED" && terminal != "REQUESTED"
        || intent != "REQUESTED" && terminal != "NOT_REQUESTED" && terminal != "REQUESTED"
}

fn string_array_subset(intent: &Value, terminal: &Value) -> bool {
    let Some(intent) = intent.as_array() else {
        return false;
    };
    let Some(terminal) = terminal.as_array() else {
        return false;
    };
    intent.iter().all(|value| terminal.contains(value))
}

fn impacts(plan: &Value) -> Result<&[Value]> {
    plan.pointer("/assurance_impacts")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| assurance_error("GATE-PLAN-SHAPE", "/assurance_impacts"))
}

fn impacts_by_key(plan: &Value) -> Result<BTreeMap<String, &Value>> {
    let mut output = BTreeMap::new();
    for impact in impacts(plan)? {
        let key = impact_key(impact)?;
        if output.insert(key.clone(), impact).is_some() {
            return Err(assurance_error("GATE-PLAN-ASSURANCE-DUPLICATE", key));
        }
    }
    Ok(output)
}

fn impact_key(impact: &Value) -> Result<String> {
    Ok(format!(
        "{}\0{}",
        impact_string(impact, "/report_id")?,
        impact_string(impact, "/changed_object/path")?
    ))
}

fn impact_string<'a>(impact: &'a Value, pointer: &str) -> Result<&'a str> {
    impact
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| assurance_error("GATE-PLAN-SHAPE", pointer))
}

fn impact_ids(impact: &Value, pointer: &str) -> Result<BTreeSet<String>> {
    impact
        .pointer(pointer)
        .and_then(Value::as_array)
        .ok_or_else(|| assurance_error("GATE-PLAN-SHAPE", pointer))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| assurance_error("GATE-PLAN-SHAPE", pointer))
        })
        .collect()
}

fn assurance_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Planning, code, message)
}

struct SemanticContexts<'a> {
    package: Option<String>,
    contracts: BTreeSet<&'a str>,
    domains: BTreeSet<&'a str>,
    explicit_watches: BTreeSet<&'a str>,
}

fn semantic_contexts<'a>(
    policy: &'a PolicyBundle,
    graph: &CargoGraph,
    change: &ObservedChange,
) -> SemanticContexts<'a> {
    let entries = policy.matching_entries(&change.path);
    SemanticContexts {
        package: graph.package_for_path(&change.path),
        contracts: entries
            .iter()
            .flat_map(|entry| entry.contracts.iter().map(String::as_str))
            .collect(),
        domains: entries
            .iter()
            .map(|entry| entry.semantic_surface.as_str())
            .collect(),
        explicit_watches: entries
            .iter()
            .flat_map(|entry| entry.assurance_watches.iter().map(String::as_str))
            .collect(),
    }
}

fn watch_matches(
    watch: &AssuranceWatch,
    change: &ObservedChange,
    contexts: &SemanticContexts<'_>,
) -> bool {
    if contexts.explicit_watches.contains(watch.watch_id.as_str()) {
        return true;
    }
    match watch.kind.as_str() {
        "exact_path" => change.path == watch.match_value,
        "path_prefix" => component_prefix_matches(&watch.match_value, &change.path),
        "path_glob" => wildmatch(&watch.match_value, &change.path),
        "contract_id" => {
            contexts.contracts.contains(watch.match_value.as_str())
                || contract_path_matches(&change.path, &watch.match_value)
        }
        "cargo_package" => contexts.package.as_deref() == Some(watch.match_value.as_str()),
        "process_domain_tag" => contexts.domains.contains(watch.match_value.as_str()),
        "result_procedure" | "builder_schema" => {
            path_value_matches(&watch.match_value, &change.path)
        }
        _ => false,
    }
}

fn component_prefix_matches(prefix: &str, path: &str) -> bool {
    let prefix = prefix.trim_end_matches('/');
    path == prefix
        || path
            .strip_prefix(prefix)
            .is_some_and(|tail| tail.starts_with('/'))
}

fn path_value_matches(value: &str, path: &str) -> bool {
    if value.ends_with('/') {
        component_prefix_matches(value, path)
    } else {
        value == path
    }
}

fn contract_path_matches(path: &str, contract_id: &str) -> bool {
    path.strip_prefix("docs/specifications/science-contracts/contracts/")
        .and_then(|tail| tail.strip_suffix(".md"))
        == Some(contract_id)
}

struct ImpactMatch {
    watch_ids: Vec<String>,
    lifecycle_boundaries: Vec<String>,
    unknown: bool,
}

fn impact_record(
    policy: &PolicyBundle,
    report: &AssuranceReport,
    change: &ObservedChange,
    target_head: &str,
    campaign_id: Option<&str>,
    request_campaign_transfer: bool,
    impact_match: &ImpactMatch,
) -> Result<Value> {
    let mapping_complete = !impact_match.unknown
        && !impact_match.watch_ids.is_empty()
        && report.resolution_authority.principal_id.is_some()
        && report.resolution_authority.role_record_sha256.is_some();
    let impact_state = if mapping_complete {
        "OPEN_ASSESSMENT"
    } else {
        "OPEN_UNKNOWN"
    };
    let campaign_request = if request_campaign_transfer {
        "REQUESTED"
    } else {
        "NOT_REQUESTED"
    };
    let mut record = json!({
        "impact_record_id": "0".repeat(64),
        "registry_generation": policy.assurance_registry.generation,
        "registry_sha256": policy.assurance_registry_sha256,
        "campaign_id": campaign_id,
        "report_id": report.report_id,
        "source_root": report.source_root,
        "assessed_realization_root": report.assessed_realization_root,
        "target_head": target_head,
        "requested_action": "ASSESS",
        "watch_generation": report.watch_generation,
        "matching_watch_ids": impact_match.watch_ids,
        "lifecycle_boundaries": impact_match.lifecycle_boundaries,
        "changed_object": {
            "path": change.path,
            "change_kind": change.change_kind
        },
        "impact_state": impact_state,
        "mapping_complete": mapping_complete,
        "resolution_authority": {
            "principal_id": report.resolution_authority.principal_id,
            "role_id": report.resolution_authority.role_id,
            "role_record_sha256": report.resolution_authority.role_record_sha256
        },
        "assessed_realization_integrity": "CURRENT",
        "campaign_impact_disposition": "IMPACT_PENDING",
        "campaign_transfer_request": campaign_request,
        "campaign_transfer_currency": "BLOCKED",
        "release_transfer_request": "NOT_REQUESTED",
        "release_transfer_currency": "BLOCKED"
    });
    record["impact_record_id"] = Value::String(derived_id(&record, "impact_record_id")?);
    Ok(record)
}

fn wildmatch(pattern: &str, path: &str) -> bool {
    let pattern = pattern.as_bytes();
    let path = path.as_bytes();
    let mut memo = vec![vec![None; path.len() + 1]; pattern.len() + 1];
    wildmatch_from(pattern, path, 0, 0, &mut memo)
}

/// Validate the subset of Git wildmatch syntax admitted by the registry.
pub(crate) fn validate_wildmatch_pattern(pattern: &str) -> bool {
    let bytes = pattern.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => {
                index += 1;
                if index == bytes.len() {
                    return false;
                }
            }
            b'[' => {
                if class_match(bytes, index, b'x').is_none() {
                    return false;
                }
                let mut closing = index + 1;
                while closing < bytes.len() {
                    if bytes[closing] == b'\\' {
                        closing += 2;
                    } else if bytes[closing] == b']' {
                        break;
                    } else {
                        closing += 1;
                    }
                }
                index = closing;
            }
            _ => {}
        }
        index += 1;
    }
    true
}

fn wildmatch_from(
    pattern: &[u8],
    path: &[u8],
    pattern_index: usize,
    path_index: usize,
    memo: &mut [Vec<Option<bool>>],
) -> bool {
    if let Some(value) = memo[pattern_index][path_index] {
        return value;
    }
    let value = if pattern_index == pattern.len() {
        path_index == path.len()
    } else if pattern[pattern_index] == b'*' {
        let mut next_pattern = pattern_index + 1;
        while pattern.get(next_pattern) == Some(&b'*') {
            next_pattern += 1;
        }
        let recursive = next_pattern >= pattern_index + 2
            && (pattern_index == 0 || pattern[pattern_index - 1] == b'/')
            && (next_pattern == pattern.len() || pattern[next_pattern] == b'/');
        let zero_directories = recursive
            && pattern.get(next_pattern) == Some(&b'/')
            && wildmatch_from(pattern, path, next_pattern + 1, path_index, memo);
        zero_directories
            || wildmatch_from(pattern, path, next_pattern, path_index, memo)
            || path_index < path.len()
                && (recursive || path[path_index] != b'/')
                && wildmatch_from(pattern, path, pattern_index, path_index + 1, memo)
    } else if pattern[pattern_index] == b'?' {
        path_index < path.len()
            && path[path_index] != b'/'
            && wildmatch_from(pattern, path, pattern_index + 1, path_index + 1, memo)
    } else if pattern[pattern_index] == b'[' {
        path_index < path.len()
            && path[path_index] != b'/'
            && class_match(pattern, pattern_index, path[path_index]).is_some_and(
                |(matches, next_pattern)| {
                    matches && wildmatch_from(pattern, path, next_pattern, path_index + 1, memo)
                },
            )
    } else if pattern[pattern_index] == b'\\' {
        pattern.get(pattern_index + 1).is_some_and(|literal| {
            path.get(path_index) == Some(literal)
                && wildmatch_from(pattern, path, pattern_index + 2, path_index + 1, memo)
        })
    } else {
        path_index < path.len()
            && pattern[pattern_index] == path[path_index]
            && wildmatch_from(pattern, path, pattern_index + 1, path_index + 1, memo)
    };
    memo[pattern_index][path_index] = Some(value);
    value
}

fn class_match(pattern: &[u8], start: usize, value: u8) -> Option<(bool, usize)> {
    let mut index = start + 1;
    let negated = matches!(pattern.get(index), Some(b'!' | b'^'));
    index += usize::from(negated);
    let mut matched = false;
    let mut populated = false;
    while index < pattern.len() && pattern[index] != b']' {
        let first = class_literal(pattern, &mut index)?;
        populated = true;
        if pattern.get(index) == Some(&b'-') && pattern.get(index + 1) != Some(&b']') {
            index += 1;
            let last = class_literal(pattern, &mut index)?;
            matched |= first <= value && value <= last;
        } else {
            matched |= first == value;
        }
    }
    if !populated || pattern.get(index) != Some(&b']') {
        return None;
    }
    Some((matched != negated, index + 1))
}

fn class_literal(pattern: &[u8], index: &mut usize) -> Option<u8> {
    if pattern.get(*index) == Some(&b'\\') {
        *index += 1;
    }
    let literal = *pattern.get(*index)?;
    *index += 1;
    Some(literal)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use serde_json::json;

    use super::{
        component_prefix_matches, reconcile_assurance_impacts, validate_wildmatch_pattern,
        wildmatch,
    };

    #[test]
    fn component_prefix_respects_path_boundaries() {
        assert!(component_prefix_matches(
            "crates/openwepp",
            "crates/openwepp/src/lib.rs"
        ));
        assert!(!component_prefix_matches(
            "crates/openwepp",
            "crates/openwepp-extra/lib.rs"
        ));
    }

    #[test]
    fn rooted_wildmatch_distinguishes_single_and_recursive_stars() {
        assert!(wildmatch(
            "tests/fixtures/*/**",
            "tests/fixtures/snow/site/data.csv"
        ));
        assert!(!wildmatch(
            "tests/fixtures/*",
            "tests/fixtures/snow/data.csv"
        ));
        assert!(wildmatch(
            "tests/**/README.?d",
            "tests/fixtures/snow/README.md"
        ));
        assert!(wildmatch("tests/**/README.md", "tests/README.md"));
        assert!(wildmatch("tests/site-[0-9].csv", "tests/site-3.csv"));
        assert!(!wildmatch("tests/site-[!0-9].csv", "tests/site-3.csv"));
        assert!(wildmatch("tests/literal\\?.csv", "tests/literal?.csv"));
        assert!(wildmatch("tests/foo**bar.rs", "tests/fooxbar.rs"));
        assert!(!wildmatch("tests/foo**bar.rs", "tests/foo/nested/bar.rs"));
        assert!(!validate_wildmatch_pattern("tests/[broken"));
        assert!(!validate_wildmatch_pattern("tests/trailing\\"));
    }

    #[test]
    fn terminal_reconciliation_rejects_campaign_or_request_weakening() {
        let impact = json!({
            "report_id": "report", "changed_object": {"path": "src/lib.rs", "change_kind": "MODIFY"},
            "registry_generation": 1, "registry_sha256": "a", "source_root": "b",
            "assessed_realization_root": "c", "campaign_id": "campaign",
            "requested_action": "ASSESS", "watch_generation": 1,
            "matching_watch_ids": ["watch"], "lifecycle_boundaries": ["CAMPAIGN_CLOSURE"],
            "impact_state": "OPEN_ASSESSMENT", "mapping_complete": true,
            "resolution_authority": {"principal_id": "lead", "role_id": "report_lead", "role_record_sha256": "d"},
            "assessed_realization_integrity": "CURRENT", "campaign_impact_disposition": "IMPACT_PENDING",
            "campaign_transfer_request": "REQUESTED", "campaign_transfer_currency": "BLOCKED",
            "release_transfer_request": "NOT_REQUESTED", "release_transfer_currency": "BLOCKED"
        });
        let intent = json!({"assurance_impacts": [impact.clone()]});
        let paths = BTreeSet::from(["src/lib.rs".to_owned()]);
        let mut terminal_impact = impact.clone();
        terminal_impact["campaign_id"] = json!(null);
        terminal_impact["campaign_transfer_request"] = json!("NOT_REQUESTED");
        let terminal = json!({"assurance_impacts": [terminal_impact]});
        let error = reconcile_assurance_impacts(&intent, &terminal, &paths)
            .expect_err("terminal assurance cannot erase campaign transfer intent");
        assert_eq!(error.code, "GATE-TERMINAL-ASSURANCE-WEAKENED");
    }
}
