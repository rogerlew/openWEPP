use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use openwepp_gate_planner::Result;
use openwepp_gate_planner::canonical::derived_id;
use openwepp_gate_planner::planner::{InventoryProvider, PlanRequest, Planner, PlanningStage};
use openwepp_gate_planner::policy::{GateDefinition, PolicyBundle};
use openwepp_gate_planner::repository::{ObservedChange, ObservedSource};
use serde_json::Value;

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[derive(Clone, Copy)]
struct FixedInventory;

impl InventoryProvider for FixedInventory {
    fn inventory(
        &self,
        _repo: &Path,
        definition: &GateDefinition,
        target: &str,
    ) -> Result<Vec<String>> {
        Ok(vec![format!("{}::{target}", definition.gate_definition_id)])
    }
}

fn head(root: &Path) -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()
        .expect("run git rev-parse");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .expect("UTF-8 commit")
        .trim()
        .to_owned()
}

fn change(path: &str, kind: &str) -> ObservedChange {
    let (old_mode, new_mode) = match kind {
        "ADD" => (None, Some("100644".to_owned())),
        "DELETE" => (Some("100644".to_owned()), None),
        _ => (Some("100644".to_owned()), Some("100644".to_owned())),
    };
    ObservedChange {
        path: path.to_owned(),
        change_kind: kind.to_owned(),
        object_kind: "REGULAR".to_owned(),
        old_mode,
        new_mode,
    }
}

fn build_plan(root: &Path) -> Value {
    let changes = vec![
        change(
            "crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs",
            "DELETE",
        ),
        change(
            "crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater_v2.rs",
            "ADD",
        ),
        change(
            "docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md",
            "MODIFY",
        ),
        change("crates/openwepp-gate-planner/src/unmapped_new.rs", "ADD"),
        change("crates/openwepp-meteorology/src/phase.rs", "MODIFY"),
        change(
            "assurance/v2/reports/linear-groundwater-reservoir-recurrence/procedures/reproduce_groundwater_report.py",
            "MODIFY",
        ),
        change("assurance/v2/schemas/report.schema.json", "MODIFY"),
        change("tests/fixtures/snow/site/data.csv", "ADD"),
    ];
    let request = PlanRequest {
        stage: PlanningStage::Intent,
        predecessor_intent_plan_id: None,
        boundary: "INCREMENT".to_owned(),
        campaign_id: Some("TESTGATE-ASSURE-01".to_owned()),
        authorized_paths: changes.iter().map(|item| item.path.clone()).collect(),
        source: ObservedSource {
            base_commit: head(root),
            head_commit: None,
            dirty_tree_digest: Some("11".repeat(32)),
            index_digest: Some("22".repeat(32)),
            worktree_digest: Some("33".repeat(32)),
            untracked_digest: Some("44".repeat(32)),
            changes,
        },
    };
    Planner::new(FixedInventory)
        .build(root, &request)
        .expect("build assurance-aware intent plan")
}

fn impact<'a>(plan: &'a Value, report: &str, path: &str) -> &'a Value {
    plan["assurance_impacts"]
        .as_array()
        .expect("assurance impacts")
        .iter()
        .find(|entry| entry["report_id"] == report && entry["changed_object"]["path"] == path)
        .expect("matching impact")
}

fn assert_deterministic_impact_set(plan: &Value) {
    let impacts = plan["assurance_impacts"]
        .as_array()
        .expect("assurance impacts");
    assert_eq!(
        impacts.len(),
        16,
        "every report is assessed for every object"
    );
    for entry in impacts {
        assert_eq!(
            entry["impact_record_id"],
            derived_id(entry, "impact_record_id").expect("reconstruct impact identity")
        );
    }
    let keys = impacts
        .iter()
        .map(|entry| {
            format!(
                "{}\0{}\0{}",
                entry["report_id"].as_str().expect("report ID"),
                entry["changed_object"]["path"]
                    .as_str()
                    .expect("changed path"),
                entry["impact_record_id"].as_str().expect("impact ID")
            )
        })
        .collect::<Vec<_>>();
    assert!(keys.windows(2).all(|pair| pair[0] < pair[1]));
}

fn assert_match_and_currency_bindings(plan: &Value) {
    let groundwater = "linear-groundwater-reservoir-recurrence";
    let old_path = "crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs";
    let old = impact(plan, groundwater, old_path);
    let watches = old["matching_watch_ids"]
        .as_array()
        .expect("matching watches")
        .iter()
        .filter_map(Value::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        watches,
        BTreeSet::from(["gw-contract", "gw-domain", "gw-package", "gw-runtime-path"])
    );
    assert_eq!(old["changed_object"]["change_kind"], "DELETE");
    assert_eq!(old["assessed_realization_integrity"], "CURRENT");
    assert_eq!(
        old["assessed_realization_root"],
        "78231571d51e9d3a95868d3f928b84bc9ce5a58d2deb8cc54e70b18b3784f1bb"
    );
    assert_eq!(old["impact_state"], "OPEN_UNKNOWN");
    assert_eq!(
        old["lifecycle_boundaries"],
        serde_json::json!(["CAMPAIGN_CLOSURE"])
    );

    let snow = impact(plan, "snow-and-frozen-soil-process-evaluation", old_path);
    assert_eq!(snow["impact_state"], "OPEN_ASSESSMENT");
    assert_eq!(snow["campaign_transfer_request"], "NOT_REQUESTED");
    assert_eq!(snow["campaign_transfer_currency"], "BLOCKED");
    assert_eq!(
        snow["target_head"],
        "11".repeat(32),
        "dirty assurance impacts bind the exact dirty-tree identity"
    );

    let unknown_path = "crates/openwepp-gate-planner/src/unmapped_new.rs";
    for report in [groundwater, "snow-and-frozen-soil-process-evaluation"] {
        let unknown = impact(plan, report, unknown_path);
        assert_eq!(unknown["matching_watch_ids"], serde_json::json!([]));
        assert_eq!(unknown["mapping_complete"], false);
        assert_eq!(unknown["impact_state"], "OPEN_UNKNOWN");
    }
}

fn assert_semantic_watch_kinds(plan: &Value) {
    let groundwater = "linear-groundwater-reservoir-recurrence";
    let cases = [
        (
            "snow-and-frozen-soil-process-evaluation",
            "crates/openwepp-meteorology/src/phase.rs",
            "snow-domain",
        ),
        (
            groundwater,
            "assurance/v2/reports/linear-groundwater-reservoir-recurrence/procedures/reproduce_groundwater_report.py",
            "gw-reproduction-procedure",
        ),
        (
            groundwater,
            "assurance/v2/schemas/report.schema.json",
            "gw-assurance-schema",
        ),
        (
            "snow-and-frozen-soil-process-evaluation",
            "tests/fixtures/snow/site/data.csv",
            "snow-fixtures",
        ),
    ];
    for (report, path, watch_id) in cases {
        assert!(
            impact(plan, report, path)["matching_watch_ids"]
                .as_array()
                .expect("matching watches")
                .contains(&serde_json::json!(watch_id))
        );
    }
}

#[test]
fn planner_discovers_registry_wide_exact_semantic_and_unknown_impacts() {
    let root = repo();
    let first = build_plan(&root);
    let second = build_plan(&root);
    assert_eq!(first, second, "identical inputs must yield identical plans");
    assert_deterministic_impact_set(&first);
    assert_match_and_currency_bindings(&first);
    assert_semantic_watch_kinds(&first);
}

#[test]
fn production_registry_is_strict_and_equals_the_canonical_catalog() {
    let root = repo();
    let policy = PolicyBundle::load(&root).expect("load complete assurance policy");
    let registry_ids = policy
        .assurance_registry
        .reports
        .iter()
        .map(|report| report.report_id.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        registry_ids,
        BTreeSet::from([
            "linear-groundwater-reservoir-recurrence",
            "snow-and-frozen-soil-process-evaluation"
        ])
    );

    let schema: Value = serde_json::from_slice(
        &std::fs::read(root.join("gate-policy/v1/schemas/assurance-registry.schema.json"))
            .expect("registry schema"),
    )
    .expect("schema JSON");
    let valid: Value = serde_json::from_slice(
        &std::fs::read(root.join("gate-policy/v1/fixtures/valid/assurance-registry.json"))
            .expect("valid registry fixture"),
    )
    .expect("fixture JSON");
    let validator = jsonschema::draft202012::new(&schema).expect("compile registry schema");
    validator.validate(&valid).expect("valid registry");
    let mutation: Value = serde_json::from_slice(
        &std::fs::read(
            root.join("gate-policy/v1/fixtures/invalid/assurance-registry-incomplete.json"),
        )
        .expect("invalid registry descriptor"),
    )
    .expect("mutation JSON");
    assert_eq!(mutation["base_fixture"], "assurance-registry.json");
    assert_eq!(mutation["instance_pointer"], "/reports/0/watches");
    let mut invalid = valid;
    invalid["reports"][0]["watches"] = mutation["invalid_value"].clone();
    assert!(validator.validate(&invalid).is_err());
}

#[test]
fn only_an_exact_committed_terminal_requests_campaign_transfer() {
    let root = repo();
    let commit = head(&root);
    let changed = change(
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs",
        "MODIFY",
    );
    let request = PlanRequest {
        stage: PlanningStage::Terminal,
        predecessor_intent_plan_id: Some("55".repeat(32)),
        boundary: "CAMPAIGN".to_owned(),
        campaign_id: Some("TESTGATE-ASSURE-01".to_owned()),
        authorized_paths: vec![changed.path.clone()],
        source: ObservedSource {
            base_commit: commit.clone(),
            head_commit: Some(commit.clone()),
            dirty_tree_digest: None,
            index_digest: None,
            worktree_digest: None,
            untracked_digest: None,
            changes: vec![changed],
        },
    };
    let plan = Planner::new(FixedInventory)
        .build(&root, &request)
        .expect("build exact committed terminal plan");
    for entry in plan["assurance_impacts"]
        .as_array()
        .expect("assurance impacts")
    {
        assert_eq!(entry["target_head"], commit);
        assert_eq!(entry["campaign_transfer_request"], "REQUESTED");
        assert_eq!(entry["campaign_transfer_currency"], "BLOCKED");
    }
}
