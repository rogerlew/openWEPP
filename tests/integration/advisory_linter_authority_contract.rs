use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;
use sha2::{Digest, Sha256};

const OLD_POLICY_SHA256: &str = "74203b294dcea4c7f3ecb5fe4110a425d938d2ec75bde60cfc646a54fea3f5e9";
const OLD_POLICY_GIT_BLOB: &str = "ab8fe3e4db61df6691a96a11fa2034b90036bfb2";
const OLD_POLICY_COMMIT: &str = "57f5f6f1f1649022d47124de856108c6a11cc483";
const POLICY_PATH: &str = "docs/standards/testing-and-gate-strategy.md";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path))
        .unwrap_or_else(|error| panic!("expected readable {path}: {error}"))
}

fn json(path: &str) -> Value {
    serde_json::from_str(&text(path))
        .unwrap_or_else(|error| panic!("expected valid JSON {path}: {error}"))
}

#[test]
fn adr0043_is_the_prospective_validation_authority() {
    let adr = text("docs/decisions/0043-gate-planner-is-a-non-authoritative-advisory-linter.md")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for required in [
        "**advisory linter**. Its only product is cited information",
        "deterministic, read-only",
        "never executes validation",
        "no CI role",
        "truthful closure; a linter defect cannot",
        "A0/A1/A3",
        "Harvard custody remains outside the linter",
    ] {
        assert!(adr.contains(required), "ADR-0043 missing: {required}");
    }

    let root_agents = text("AGENTS.md");
    assert!(root_agents.contains("TESTGATE and the gate planner are frozen historical tooling"));
    assert!(root_agents.contains("Run every applicable increment requirement directly"));

    let package_agents = text("docs/work-packages/AGENTS.md");
    assert!(package_agents.contains("## Advisory Validation Planning And Tool Friction"));
    assert!(package_agents.contains("creates no permission, hold, lifecycle"));
}

#[test]
fn canonical_strategy_requires_direct_execution_without_planner_admission() {
    let strategy = text("docs/standards/testing-and-gate-strategy.md")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for required in [
        "Agents apply them directly",
        "advisory finding** is a read-only tool observation",
        "Unknown or ambiguous production impact receives documented conservative",
        "A0/A1/A3 remain non-deferrable",
        "Evidence may be reused only when source, execution and documentation roots",
        "coverage and CRAP",
        "direct assurance-governance duties",
        "Do not dispatch, repair, or extend TESTGATE",
        "does not require a final planner plan",
        "Every suite declares its family, owner, dependencies",
        "semantic or scientific failure can never be retried into accepted execution",
        "A1/A3 | Accepted execution and scientific `CONFORMS`",
        "affected doctests, placeholder/stub scanning",
        "shared numerical primitives, calendars, chronology",
        "deletion, disabling, renaming, filtering, reclassification",
    ] {
        assert!(strategy.contains(required), "strategy missing: {required}");
    }

    for retired in [
        "TESTGATE determines whether an increment is admissible",
        "mechanically generated intent plan",
        "pre-heavy closure audit",
        "authenticated terminal plan",
    ] {
        assert!(
            !strategy.contains(retired),
            "retired prospective authority remains: {retired}"
        );
    }
}

#[test]
fn historical_policy_identity_resolves_the_frozen_object() {
    let registry = json("gate-policy/history/adr0039-generation17.json");
    assert_eq!(registry["schema_version"], "openwepp-historical-policy-v1");
    assert_eq!(registry["policy_sha256"], OLD_POLICY_SHA256);
    assert_eq!(registry["git_blob"], OLD_POLICY_GIT_BLOB);
    assert_eq!(registry["source_commit"], OLD_POLICY_COMMIT);
    assert_eq!(registry["subject_path"], POLICY_PATH);
    assert_eq!(registry["prospective_authority"], false);
    assert_eq!(registry["historical_bytes_immutable"], true);
    assert_eq!(
        registry["verification_rule"],
        "resolve_git_blob_and_sha256_never_live_path"
    );

    let output = Command::new("git")
        .args(["cat-file", "blob", OLD_POLICY_GIT_BLOB])
        .current_dir(root())
        .output()
        .expect("git cat-file historical policy blob");
    assert!(output.status.success(), "historical Git blob must exist");
    assert_eq!(
        format!("{:x}", Sha256::digest(&output.stdout)),
        OLD_POLICY_SHA256
    );

    let commit_path = format!("{OLD_POLICY_COMMIT}:{POLICY_PATH}");
    let resolved = Command::new("git")
        .args(["rev-parse", &commit_path])
        .current_dir(root())
        .output()
        .expect("git rev-parse historical commit:path");
    assert!(
        resolved.status.success(),
        "historical commit:path must resolve"
    );
    assert_eq!(
        String::from_utf8(resolved.stdout)
            .expect("Git object ID must be UTF-8")
            .trim(),
        OLD_POLICY_GIT_BLOB
    );
}

#[test]
fn live_impact_map_has_no_planner_admission_rows() {
    let impact = json("tools/release/authority-policy/impact-map.json");
    let schema = json("tools/release/authority-policy/impact-map.schema.json");
    jsonschema::validator_for(&schema)
        .expect("compile direct impact-map schema")
        .validate(&impact)
        .expect("validate direct impact map");
    // The direct admission map retains its ADR-0039 schema identity for
    // historical continuity. Its live entries carry no planner admission
    // rows and do not belong to the advisory linter.
    assert_eq!(impact["policy_id"], "ADR-0039");
    assert_eq!(impact["enforcement_status"], "SCHEMA_ONLY_NONBLOCKING");
    assert_eq!(
        impact["policy_sha256"],
        format!("{:x}", Sha256::digest(text(POLICY_PATH).as_bytes()))
    );
    let entries = impact["entries"].as_array().expect("impact entries");
    let ids = entries
        .iter()
        .map(|entry| entry["entry_id"].as_str().expect("entry ID"))
        .collect::<Vec<_>>();

    for retired in [
        "gate-policy-authority",
        "gate-lifecycle-authority",
        "gate-planner-authority",
    ] {
        assert!(
            !ids.contains(&retired),
            "retired planner row remains: {retired}"
        );
    }
    assert!(
        ids.iter().all(|entry_id| !entry_id.starts_with("auth11-")),
        "dead anti-evasion routing rows must not survive"
    );

    let policy_readme = text("tools/release/authority-policy/README.md")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for required in [
        "direct science-contract admission",
        "SCHEMA_ONLY_NONBLOCKING",
        "no prospective planner effect",
        "a registry match is information only",
    ] {
        assert!(
            policy_readme.contains(required),
            "policy README missing frozen boundary: {required}"
        );
    }
    assert!(!policy_readme.contains("Status: blocking normal-increment authority"));
}

#[test]
fn direct_authority_definitions_are_compact_and_schema_valid() {
    let definitions = json("tools/release/authority-policy/gate-definitions.json");
    let schema = json("tools/release/authority-policy/gate-definitions.schema.json");
    jsonschema::validator_for(&schema)
        .expect("compile direct authority schema")
        .validate(&definitions)
        .expect("validate direct authority definitions");
    assert_eq!(
        definitions["schema_version"],
        "openwepp-direct-authority-definitions-v1"
    );
    assert_eq!(
        definitions["definitions"]
            .as_array()
            .expect("definitions")
            .len(),
        5
    );
    let encoded = serde_json::to_string(&definitions).expect("encode definitions");
    for forbidden in [
        "blocks_transition",
        "prerequisite_definition_ids",
        "receipt",
        "ledger",
        "artifact_contract",
        "quality_disposition",
    ] {
        assert!(
            !encoded.contains(forbidden),
            "legacy field remains: {forbidden}"
        );
    }
}

#[test]
fn frozen_planner_packages_cannot_resume_from_local_status() {
    for path in [
        "docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/package.md",
        "docs/work-packages/20260727-gate-planner-external-dag-closeout-correction-001/package.md",
        "docs/work-packages/20260727-gate-planner-auth11-terminal-node-selection-001/package.md",
        "docs/work-packages/20260727-gate-planner-auth11-fixed-inventory-test-provider-001/package.md",
    ] {
        let package = text(path);
        assert!(
            package.contains("Status: `FROZEN / SUPERSEDED BY ADR-0043`"),
            "frozen status missing: {path}"
        );
        assert!(
            package.contains("requires explicit user authorization"),
            "resume guard missing: {path}"
        );
    }

    let completed =
        text("docs/work-packages/20260727-testgate-first-attempt-ledger-bootstrap-001/package.md");
    assert!(completed.contains("Status: `COMPLETE`"));

    let catalog = text("docs/work-packages/README.md");
    for package_id in [
        "20260727-gate-planner-external-dag-transaction-adapter-001",
        "20260727-gate-planner-external-dag-closeout-correction-001",
        "20260727-gate-planner-auth11-terminal-node-selection-001",
        "20260727-gate-planner-auth11-fixed-inventory-test-provider-001",
    ] {
        let marker = format!("`{package_id}/` -");
        let entry = catalog
            .split_once(&marker)
            .unwrap_or_else(|| panic!("catalog entry missing: {package_id}"))
            .1
            .split("\n- `")
            .next()
            .expect("catalog entry body");
        assert!(
            entry.contains("FROZEN / SUPERSEDED BY ADR-0043"),
            "catalog frozen status missing: {package_id}"
        );
    }
    let completed_marker = "`20260727-testgate-first-attempt-ledger-bootstrap-001/` -";
    let completed_entry = catalog
        .split_once(completed_marker)
        .expect("completed catalog entry")
        .1
        .split("\n- `")
        .next()
        .expect("completed catalog entry body");
    assert!(completed_entry.contains("COMPLETE / HISTORICAL UNDER ADR-0043"));

    assert!(
        !root()
            .join("tests/integration/testgate_ci_executor_contract.rs")
            .exists()
    );
}
