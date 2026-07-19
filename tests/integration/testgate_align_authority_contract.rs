use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use sha2::{Digest, Sha256};

const CONTRACTS: &[(&str, &str, &str, &str, &str)] = &[
    (
        "execution-matrix",
        "execution-matrix.schema.json",
        "execution-matrix-unsupported-target.json",
        "openwepp:gate-policy:execution-matrix:1",
        "openwepp-gate-execution-matrix-v1",
    ),
    (
        "gate-definitions",
        "gate-definitions.schema.json",
        "gate-definitions-shell-template.json",
        "openwepp:gate-policy:gate-definitions:1",
        "openwepp-gate-definitions-v1",
    ),
    (
        "impact-map",
        "impact-map.schema.json",
        "impact-map-unknown-downgrade.json",
        "openwepp:gate-policy:impact-map:1",
        "openwepp-gate-impact-map-v1",
    ),
    (
        "gate-plan",
        "gate-plan.schema.json",
        "gate-plan-shell-injection.json",
        "openwepp:gate-policy:gate-plan:1",
        "openwepp-gate-plan-v1",
    ),
    (
        "gate-receipt",
        "gate-receipt.schema.json",
        "gate-receipt-recursive-envelope.json",
        "openwepp:gate-policy:gate-receipt:1",
        "openwepp-gate-receipt-v1",
    ),
    (
        "attestation-envelope",
        "attestation-envelope.schema.json",
        "attestation-envelope-local-trust.json",
        "openwepp:gate-policy:attestation-envelope:1",
        "openwepp-gate-attestation-envelope-v1",
    ),
    (
        "campaign-ledger",
        "campaign-ledger.schema.json",
        "campaign-ledger-waived-state.json",
        "openwepp:gate-policy:campaign-ledger:1",
        "openwepp-campaign-ledger-v1",
    ),
    (
        "assurance-impact",
        "assurance-impact.schema.json",
        "assurance-impact-bare-current.json",
        "openwepp:gate-policy:assurance-impact:1",
        "openwepp-assurance-impact-v1",
    ),
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn load_json(path: &Path) -> Value {
    let bytes = fs::read(path)
        .unwrap_or_else(|error| panic!("expected readable JSON {}: {error}", path.display()));
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|error| panic!("expected valid JSON {}: {error}", path.display()))
}

fn load_text(path: &str) -> String {
    let full_path = repo_root().join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

#[test]
fn schemas_accept_positive_and_reject_negative_fixtures() {
    let policy = repo_root().join("gate-policy/v1");
    for (fixture_stem, schema_name, invalid_name, _, _) in CONTRACTS {
        let schema_path = policy.join("schemas").join(schema_name);
        let schema = load_json(&schema_path);
        let validator = jsonschema::draft202012::new(&schema).unwrap_or_else(|error| {
            panic!("schema must compile {}: {error}", schema_path.display())
        });

        let valid_path = policy
            .join("fixtures/valid")
            .join(format!("{fixture_stem}.json"));
        let valid = load_json(&valid_path);
        if let Err(error) = validator.validate(&valid) {
            panic!(
                "positive fixture must validate {}: {error}",
                valid_path.display()
            );
        }

        let invalid_path = policy.join("fixtures/invalid").join(invalid_name);
        let mutation = load_json(&invalid_path);
        assert_eq!(mutation["base_fixture"], format!("{fixture_stem}.json"));
        let mut invalid = valid.clone();
        apply_single_mutation(&mut invalid, &mutation, &invalid_path);
        let errors = validator
            .iter_errors(&invalid)
            .map(|error| {
                (
                    error.instance_path().to_string(),
                    error.schema_path().to_string(),
                )
            })
            .collect::<Vec<_>>();
        assert!(
            !errors.is_empty(),
            "one-mutation negative fixture must be rejected: {}",
            invalid_path.display()
        );
        let expected_instance = mutation["expected_instance_path"]
            .as_str()
            .expect("expected instance path");
        let expected_schema = mutation["expected_schema_path_contains"]
            .as_str()
            .expect("expected schema path fragment");
        assert!(
            errors.iter().any(|(instance, schema)| {
                instance == expected_instance && schema.contains(expected_schema)
            }),
            "negative fixture {} missed intended rejection; errors: {errors:?}",
            invalid_path.display()
        );
    }
}

#[test]
fn production_impact_map_is_schema_valid_and_fail_closed() {
    let policy = repo_root().join("gate-policy/v1");
    let schema = load_json(&policy.join("schemas/impact-map.schema.json"));
    let impact_map = load_json(&policy.join("impact-map.json"));
    let validator = jsonschema::draft202012::new(&schema).expect("compile impact-map schema");
    validator
        .validate(&impact_map)
        .expect("production impact map must validate");

    assert_eq!(
        impact_map["unknown_path_action"], "ESCALATE_CRITICAL",
        "unknown paths must never silently narrow selection"
    );
    assert_eq!(
        impact_map["enforcement_status"], "BLOCKING",
        "TESTGATE must be normal increment authority"
    );
    let policy_bytes = fs::read(repo_root().join("docs/standards/testing-and-gate-strategy.md"))
        .expect("read canonical gate strategy");
    let policy_sha256 = format!("{:x}", Sha256::digest(policy_bytes));
    assert_eq!(
        impact_map["policy_sha256"], policy_sha256,
        "impact map must bind the exact canonical gate strategy"
    );
    let entries = impact_map["entries"]
        .as_array()
        .expect("impact-map entries");
    assert!(
        entries.iter().any(|entry| {
            entry["matcher"]["value"] == "gate-policy/" && entry["risk_floor"] == "CRITICAL"
        }),
        "the gate-policy authority must map to critical risk"
    );
}

#[test]
fn production_gate_definitions_are_schema_valid_and_registered() {
    let policy = repo_root().join("gate-policy/v1");
    let schema = load_json(&policy.join("schemas/gate-definitions.schema.json"));
    let registry = load_json(&policy.join("gate-definitions.json"));
    let validator = jsonschema::draft202012::new(&schema).expect("compile definitions schema");
    validator
        .validate(&registry)
        .expect("production gate definitions must validate");
    assert_eq!(registry["enforcement_status"], "BLOCKING");

    let definitions = registry["definitions"].as_array().expect("definitions");
    let ids = definitions
        .iter()
        .map(|definition| {
            definition["gate_definition_id"]
                .as_str()
                .expect("definition id")
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        ids.len(),
        definitions.len(),
        "definition IDs must be unique"
    );
    for definition in definitions {
        let environment = definition["environment_allowlist"]
            .as_array()
            .expect("environment allowlist");
        if environment.iter().any(|key| key == "RUSTUP_HOME") {
            assert!(
                environment.iter().any(|key| key == "RUSTUP_TOOLCHAIN"),
                "{} must retain the immutable image toolchain selection",
                definition["gate_definition_id"]
            );
        }
    }

    let impact_map = load_json(&policy.join("impact-map.json"));
    for entry in impact_map["entries"].as_array().expect("impact entries") {
        for id in entry["gate_definition_ids"]
            .as_array()
            .expect("definition IDs")
        {
            assert!(
                ids.contains(id.as_str().expect("definition ID")),
                "unregistered gate definition: {id}"
            );
        }
    }

    let groundwater = definitions
        .iter()
        .find(|definition| definition["gate_definition_id"] == "hard-invariant-groundwater-v1")
        .expect("groundwater A1 definition");
    assert_eq!(groundwater["authority_class"], "A1");
    assert_eq!(groundwater["inventory_source"], "NEXTEST_PACKAGE");
    assert!(
        groundwater["arguments_template"]
            .as_array()
            .expect("groundwater arguments")
            .iter()
            .any(|argument| argument == "openwepp-hillslope-orchestrator")
    );
    let groundwater_tests = fs::read_to_string(
        repo_root()
            .join("crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs"),
    )
    .expect("groundwater producer tests");
    for invariant in [
        "gwbaseflow_linear_reservoir_recurrence_uses_prior_day_exports",
        "gwbaseflow_mofe_recharge_aggregates_lane_deep_percolation",
        "gwbaseflow_exports_over_accepted_storage_fail_closed",
    ] {
        assert!(groundwater_tests.contains(invariant), "missing {invariant}");
    }

    let admission =
        fs::read_to_string(repo_root().join("tools/release/check_science_contract_admission.sh"))
            .expect("science admission gate");
    assert!(admission.contains("applicable_a3 = sorted("));
    assert!(admission.contains("sorted(suites) != applicable_a3"));
}

#[test]
fn schemas_preserve_closed_identity_and_state_vocabularies() {
    let schemas = repo_root().join("gate-policy/v1/schemas");
    for (_, schema_name, _, expected_id, expected_version) in CONTRACTS {
        let schema = load_json(&schemas.join(schema_name));
        assert_eq!(
            schema["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
        assert_eq!(schema["additionalProperties"], false);
        assert_eq!(schema["$id"], *expected_id, "schema identity changed");
        assert_eq!(
            schema["properties"]["schema_version"]["const"], *expected_version,
            "schema version changed"
        );
        assert_object_schemas_are_closed(&schema, schema_name);
    }

    let plan = load_json(&schemas.join("gate-plan.schema.json"));
    let node_properties = &plan["$defs"]["node"]["properties"];
    assert!(node_properties.get("arguments").is_some());
    assert!(node_properties.get("shell").is_none());
    assert_eq!(plan["$defs"]["node"]["additionalProperties"], false);
    for required in [
        "target",
        "features",
        "failure_classification",
        "investigation_owner",
    ] {
        assert!(
            node_properties.get(required).is_some(),
            "plan node lost {required}"
        );
    }
    for required in [
        "planning_controls",
        "execution_context",
        "zero_work_disposition",
    ] {
        assert!(
            plan["properties"].get(required).is_some(),
            "plan lost {required}"
        );
    }

    let receipt = load_json(&schemas.join("gate-receipt.schema.json"));
    assert!(receipt["properties"].get("envelope_id").is_none());
    assert!(
        receipt["$defs"]["dagNode"]["properties"]
            .get("plan_node_sha256")
            .is_some()
    );

    let ledger = load_json(&schemas.join("campaign-ledger.schema.json"));
    let states = ledger["$defs"]["obligation"]["properties"]["state"]["enum"]
        .as_array()
        .expect("closed obligation states");
    assert!(!states.iter().any(|state| state == "WAIVED"));
    assert!(states.iter().any(|state| state == "LEGACY_UNVERIFIED"));

    let assurance = load_json(&schemas.join("assurance-impact.schema.json"));
    let aggregate = assurance["properties"]["aggregate_impact"]["enum"]
        .as_array()
        .expect("closed assurance aggregate states");
    assert!(!aggregate.iter().any(|state| state == "CURRENT"));

    let plan_assurance = &plan["$defs"]["assuranceImpact"]["properties"];
    for required in ["campaign_transfer_request", "release_transfer_request"] {
        assert!(
            plan_assurance.get(required).is_some(),
            "plan assurance impact lost {required}"
        );
    }
}

#[test]
fn contradictory_pass_receipts_fail_closed() {
    let policy = repo_root().join("gate-policy/v1");

    let receipt_schema = load_json(&policy.join("schemas/gate-receipt.schema.json"));
    let receipt_validator =
        jsonschema::draft202012::new(&receipt_schema).expect("compile receipt schema");
    let receipt = load_json(&policy.join("fixtures/valid/gate-receipt.json"));
    for (pointer, value) in [
        ("/counts/failed", Value::from(1)),
        ("/source_mutation_check/unchanged", Value::from(false)),
        ("/executed_inventory", Value::Array(Vec::new())),
    ] {
        let mut contradictory = receipt.clone();
        *contradictory.pointer_mut(pointer).expect("receipt pointer") = value;
        assert!(
            !receipt_validator.is_valid(&contradictory),
            "PASS receipt contradiction must fail at {pointer}"
        );
    }

    for contradictory in contradictory_pass_receipts(&receipt) {
        assert!(
            receipt_validator.is_valid(&contradictory),
            "cross-field contradiction should remain structurally valid"
        );
        assert!(
            !receipt_semantics_are_consistent(&contradictory),
            "semantic receipt guard must reject contradictory PASS evidence"
        );
    }
}

#[test]
fn contradictory_campaign_states_fail_closed() {
    let policy = repo_root().join("gate-policy/v1");

    let ledger_schema = load_json(&policy.join("schemas/campaign-ledger.schema.json"));
    let ledger_validator =
        jsonschema::draft202012::new(&ledger_schema).expect("compile ledger schema");
    let ledger = load_json(&policy.join("fixtures/valid/campaign-ledger.json"));
    let mut unanchored = ledger.clone();
    unanchored["backstop"]["state"] = Value::String("CURRENT".to_owned());
    assert!(!ledger_validator.is_valid(&unanchored));
    let mut empty_certified = ledger;
    empty_certified["lifecycle"] = Value::String("CERTIFIED".to_owned());
    assert!(!ledger_validator.is_valid(&empty_certified));

    let ledger = load_json(&policy.join("fixtures/valid/campaign-ledger.json"));
    let certified = make_certified_ledger(ledger);
    ledger_validator
        .validate(&certified)
        .expect("complete certified fixture must be structurally valid");
    assert!(campaign_semantics_are_consistent(&certified));
    let mut wrong_certified_head = certified.clone();
    wrong_certified_head["certification"]["certified_head"] = Value::String("9".repeat(64));
    assert!(ledger_validator.is_valid(&wrong_certified_head));
    assert!(!campaign_semantics_are_consistent(&wrong_certified_head));
    let mut stale_current_backstop = certified;
    stale_current_backstop["backstop"]["head_advances_since_anchor"] = Value::from(99);
    assert!(!ledger_validator.is_valid(&stale_current_backstop));

    let mut pass_without_receipt = load_json(&policy.join("fixtures/valid/campaign-ledger.json"));
    pass_without_receipt["events"]
        .as_array_mut()
        .expect("ledger events")
        .push(serde_json::json!({
            "event_id": "7878787878787878787878787878787878787878787878787878787878787878",
            "predecessor_event_id": "4444444444444444444444444444444444444444444444444444444444444444",
            "event_type": "OBLIGATION_TRANSITION",
            "target_head": "2222222222222222222222222222222222222222222222222222222222222222",
            "authorized_by": "openwepp-maintainers",
            "recorded_at": "2026-07-17T12:03:00Z",
            "payload": {
                "kind": "OBLIGATION_TRANSITION",
                "obligation_id": "full-regression",
                "from_state": "PENDING",
                "to_state": "PASS",
                "reason_code": "RECEIPT_ACCEPTED",
                "receipt_id": null,
                "replacement_obligation_id": null
            }
        }));
    assert!(!ledger_validator.is_valid(&pass_without_receipt));
}

#[test]
fn contradictory_assurance_currency_and_impact_matchers_fail_closed() {
    let policy = repo_root().join("gate-policy/v1");

    let assurance_schema = load_json(&policy.join("schemas/assurance-impact.schema.json"));
    let assurance_validator =
        jsonschema::draft202012::new(&assurance_schema).expect("compile assurance schema");
    let mut assurance = load_json(&policy.join("fixtures/valid/assurance-impact.json"));
    assurance["aggregate_impact"] = Value::String("REFRESH_COMPLETE".to_owned());
    assurance["axes"]["campaign_transfer_currency"] = Value::String("CURRENT".to_owned());
    assert!(!assurance_validator.is_valid(&assurance));

    let mut empty_fold = load_json(&policy.join("fixtures/valid/assurance-impact.json"));
    empty_fold["entries"] = Value::Array(Vec::new());
    empty_fold["aggregate_impact"] = Value::String("NO_IMPACT_DETECTED".to_owned());
    empty_fold["axes"]["campaign_impact_disposition"] =
        Value::String("NO_IMPACT_DETECTED".to_owned());
    assurance_validator
        .validate(&empty_fold)
        .expect("canonical empty assurance fold must be representable");
    let mut bare_current = empty_fold.clone();
    bare_current["axes"]["campaign_transfer_request"] = Value::String("REQUESTED".to_owned());
    bare_current["axes"]["campaign_transfer_currency"] = Value::String("CURRENT".to_owned());
    assert!(!assurance_validator.is_valid(&bare_current));
    bare_current["events"] = serde_json::json!([{
        "event_id": "8181818181818181818181818181818181818181818181818181818181818181",
        "predecessor_event_id": null,
        "event_type": "CAMPAIGN_TRANSFER",
        "target_head": "1111111111111111111111111111111111111111111111111111111111111111",
        "request_id": "8282828282828282828282828282828282828282828282828282828282828282",
        "receipt_id": "8383838383838383838383838383838383838383838383838383838383838383",
        "envelope_id": "8484848484848484848484848484848484848484848484848484848484848484",
        "principal_id": "report-lead",
        "role_record_sha256": "8585858585858585858585858585858585858585858585858585858585858585",
        "recorded_at": "2026-07-17T12:04:00Z"
    }]);
    assurance_validator
        .validate(&bare_current)
        .expect("current transfer with a typed event is representable");

    let impact_schema = load_json(&policy.join("schemas/impact-map.schema.json"));
    let impact_validator =
        jsonschema::draft202012::new(&impact_schema).expect("compile impact map schema");
    let mut traversal = load_json(&policy.join("fixtures/valid/impact-map.json"));
    traversal["entries"][0]["matcher"]["value"] = Value::String("../../".to_owned());
    assert!(!impact_validator.is_valid(&traversal));
}

fn contradictory_pass_receipts(receipt: &Value) -> Vec<Value> {
    let mut inventory = receipt.clone();
    inventory["executed_inventory"] = serde_json::json!(["different_nonempty_executed_inventory"]);

    let mut authority = receipt.clone();
    authority["authority_outcomes"][0]["execution_integrity"] = Value::String("FAIL".to_owned());

    let mut attempt = receipt.clone();
    attempt["attempts"][0]["result"] = Value::String("FAIL".to_owned());
    attempt["attempts"][0]["exit_code"] = Value::from(1);

    let mut mutation = receipt.clone();
    mutation["source_mutation_check"]["after_sha256"] = Value::String("9".repeat(64));

    let mut zero_work = receipt.clone();
    zero_work["zero_work"] = Value::from(true);
    zero_work["dag_nodes"] = Value::Array(Vec::new());
    zero_work["attempts"] = Value::Array(Vec::new());
    zero_work["planned_inventory"] = Value::Array(Vec::new());
    zero_work["executed_inventory"] = Value::Array(Vec::new());

    vec![inventory, authority, attempt, mutation, zero_work]
}

fn receipt_semantics_are_consistent(receipt: &Value) -> bool {
    let result = receipt["result"].as_str();
    if !matches!(result, Some("PASS" | "PASS_WITH_RETRY")) {
        return true;
    }
    if receipt["planned_inventory"] != receipt["executed_inventory"] {
        return false;
    }
    let counts = &receipt["counts"];
    if counts["failed"] != 0 || counts["blocked"] != 0 {
        return false;
    }
    let mutation = &receipt["source_mutation_check"];
    if mutation["required"] == true
        && (mutation["unchanged"] != true || mutation["before_sha256"] != mutation["after_sha256"])
    {
        return false;
    }
    if receipt["zero_work"] == true {
        return receipt["dag_nodes"].as_array().is_some_and(Vec::is_empty)
            && receipt["attempts"].as_array().is_some_and(Vec::is_empty)
            && receipt["planned_inventory"]
                .as_array()
                .is_some_and(Vec::is_empty)
            && counts["passed"] == 0
            && counts["skipped"] == 0
            && counts["retried"] == 0;
    }
    let attempts_pass = receipt["attempts"].as_array().is_some_and(|attempts| {
        !attempts.is_empty()
            && attempts.iter().all(|attempt| {
                attempt["result"] == "PASS"
                    && attempt["exit_code"] == 0
                    && attempt["retry_reason"].is_null()
            })
    });
    let outcomes_pass = receipt["authority_outcomes"]
        .as_array()
        .is_some_and(|outcomes| {
            outcomes.iter().all(|outcome| {
                matches!(
                    outcome["execution_integrity"].as_str(),
                    Some("PASS" | "PASS_WITH_RETRY")
                )
            })
        });
    attempts_pass && outcomes_pass
}

fn make_certified_ledger(mut ledger: Value) -> Value {
    let receipt_id = "5151515151515151515151515151515151515151515151515151515151515151";
    let envelope_id = "5252525252525252525252525252525252525252525252525252525252525252";
    let authorization_id = "5353535353535353535353535353535353535353535353535353535353535353";
    let current_head = ledger["current_head"].clone();
    ledger["lifecycle"] = Value::String("CERTIFIED".to_owned());
    ledger["obligations"][0]["state"] = Value::String("PASS".to_owned());
    ledger["obligations"][0]["receipt_id"] = Value::String(receipt_id.to_owned());
    ledger["receipts"] = serde_json::json!([{
        "receipt_id": receipt_id,
        "envelope_id": envelope_id,
        "obligation_ids": ["full-regression"]
    }]);
    ledger["authorization_events"] = serde_json::json!([{
        "authorization_id": authorization_id,
        "principal_id": "release-owner",
        "role_id": "campaign-certifier",
        "predecessor_ledger_id": null,
        "transition_sha256": "5454545454545454545454545454545454545454545454545454545454545454",
        "repository": "rogerlew/openWEPP",
        "campaign_id": "TESTGATE-ALIGN-01",
        "target_head": current_head,
        "envelope_id": envelope_id
    }]);
    ledger["backstop"] = serde_json::json!({
        "state": "CURRENT",
        "anchor_receipt_id": receipt_id,
        "anchor_envelope_id": envelope_id,
        "anchor_head": current_head,
        "authenticated_completed_at": "2026-07-17T12:05:00Z",
        "head_advances_since_anchor": 0
    });
    ledger["certification"] = serde_json::json!({
        "receipt_id": receipt_id,
        "envelope_id": envelope_id,
        "certified_head": current_head,
        "authenticated_completed_at": "2026-07-17T12:05:00Z",
        "authorization_id": authorization_id
    });
    ledger
}

fn campaign_semantics_are_consistent(ledger: &Value) -> bool {
    if ledger["lifecycle"] != "CERTIFIED" {
        return true;
    }
    let certification = &ledger["certification"];
    if certification["certified_head"] != ledger["current_head"] {
        return false;
    }
    let receipt_bound = ledger["receipts"].as_array().is_some_and(|receipts| {
        receipts.iter().any(|receipt| {
            receipt["receipt_id"] == certification["receipt_id"]
                && receipt["envelope_id"] == certification["envelope_id"]
        })
    });
    let authorization_bound = ledger["authorization_events"]
        .as_array()
        .is_some_and(|events| {
            events.iter().any(|event| {
                event["authorization_id"] == certification["authorization_id"]
                    && event["target_head"] == ledger["current_head"]
            })
        });
    receipt_bound && authorization_bound
}

fn apply_single_mutation(instance: &mut Value, mutation: &Value, fixture_path: &Path) {
    let pointer = mutation["instance_pointer"]
        .as_str()
        .expect("mutation instance pointer");
    let value = mutation["invalid_value"].clone();
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" => {
            let target = instance.pointer_mut(pointer).unwrap_or_else(|| {
                panic!(
                    "replace pointer absent in {}: {pointer}",
                    fixture_path.display()
                )
            });
            assert_ne!(*target, value, "replacement must change exactly one value");
            *target = value;
        }
        "add" => {
            let (parent_pointer, encoded_key) = pointer
                .rsplit_once('/')
                .expect("add pointer must include an object key");
            let key = encoded_key.replace("~1", "/").replace("~0", "~");
            let parent = instance
                .pointer_mut(parent_pointer)
                .and_then(Value::as_object_mut)
                .unwrap_or_else(|| {
                    panic!(
                        "add parent absent in {}: {parent_pointer}",
                        fixture_path.display()
                    )
                });
            assert!(
                !parent.contains_key(&key),
                "add mutation must introduce one key"
            );
            parent.insert(key, value);
        }
        operation => panic!("unsupported mutation operation {operation}"),
    }
}

#[test]
fn plan_accepts_git_sha1_and_requires_terminal_intent_lineage() {
    let policy = repo_root().join("gate-policy/v1");
    let schema = load_json(&policy.join("schemas/gate-plan.schema.json"));
    let validator = jsonschema::draft202012::new(&schema).expect("compile gate-plan schema");
    let mut plan = load_json(&policy.join("fixtures/valid/gate-plan.json"));
    plan["source"]["base_commit"] = Value::String("1".repeat(40));
    validator
        .validate(&plan)
        .expect("Git SHA-1 object IDs remain supported");

    plan["predecessor_intent_plan_id"] = Value::Null;
    assert!(
        !validator.is_valid(&plan),
        "a terminal plan must bind its predecessor intent plan"
    );

    let mut zero_work = load_json(&policy.join("fixtures/valid/gate-plan.json"));
    zero_work["nodes"] = Value::Array(Vec::new());
    zero_work["zero_work_disposition"] = serde_json::json!({
        "verified": true,
        "reason_code": "NO_AFFECTED_TARGETS",
        "evidence_sha256": "abababababababababababababababababababababababababababababababab"
    });
    validator
        .validate(&zero_work)
        .expect("a governed verified empty DAG is representable");

    let mut legacy_without_adapter = load_json(&policy.join("fixtures/valid/gate-plan.json"));
    legacy_without_adapter["nodes"][0]["executor"]["kind"] =
        Value::String("LEGACY_ADAPTER_V1".to_owned());
    assert!(
        !validator.is_valid(&legacy_without_adapter),
        "legacy adapters must bind an exact adapter digest"
    );

    let mut undefined_predicate = load_json(&policy.join("fixtures/valid/gate-plan.json"));
    undefined_predicate["nodes"][0]["acceptance"] = serde_json::json!({
        "kind": "ARTIFACT",
        "operator": "VALID",
        "expected": "junit",
        "children": []
    });
    assert!(
        !validator.is_valid(&undefined_predicate),
        "undefined predicate kind/operator combinations must fail"
    );
}

#[test]
fn primary_governance_surfaces_delegate_lifecycle_to_adr0039() {
    let pointer_surfaces = [
        "AGENTS.md",
        "crates/AGENTS.md",
        "tests/AGENTS.md",
        "docs/work-packages/AGENTS.md",
        "docs/codex_exec_plans.md",
        "docs/standards/prompt-wording-guidance.md",
        "docs/standards/kernel-work-package-preparation.md",
        "docs/standards/mechanical-refactor-authoring-guide.md",
        "docs/standards/module-test-enhancement-authoring-guide.md",
        "docs/standards/code-quality-refactor-authoring-guide.md",
        "docs/standards/local-ci-gate-selection.md",
        "docs/standards/rust-scientific-coding-standard.md",
        "docs/work-packages/cqr-nightly-burndown-execplan.md",
        "docs/work-packages/templates/cqr-nightly-package.md",
        "docs/work-packages/templates/cqr-nightly-kickoff-prompt.md",
        "docs/prompt_templates/mechanical-refactor-kickoff-template.md",
        "docs/dev-guide/01-orientation.md",
        "docs/dev-guide/07-contributing.md",
        "docs/architecture/watershed-runtime-architecture-specification.md",
        "docs/work-packages/20260613-refactor022-mofe-scheduler-runner-watershed-line-count-split-001/package.md",
    ];
    for path in pointer_surfaces {
        let text = load_text(path);
        assert!(
            text.contains("testing-and-gate-strategy.md"),
            "{path} must point to the canonical lifecycle authority"
        );
    }

    let work_packages = load_text("docs/work-packages/AGENTS.md");
    let work_packages_normalized = work_packages
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(work_packages_normalized.contains("accepted pre-implementation intent plan"));
    assert!(work_packages_normalized.contains("Deferred is not"));
    assert!(work_packages_normalized.contains("Before the mechanical planner/ledger is cut over"));

    let root = load_text("AGENTS.md");
    let root_normalized = root.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(root_normalized.contains("Execute every increment gate selected"));
    assert!(root_normalized.contains("Critical changes still require"));

    for (path, forbidden) in [
        (
            "docs/dev-guide/07-contributing.md",
            "Every change must pass",
        ),
        (
            "docs/architecture/watershed-runtime-architecture-specification.md",
            "inherits the root closure loop unless",
        ),
        (
            "docs/architecture/watershed-runtime-architecture-specification.md",
            "final closure must either run these gates",
        ),
        (
            "docs/architecture/watershed-runtime-architecture-specification.md",
            "run or explicitly hold on the required Rust closure loop",
        ),
        (
            "docs/standards/mechanical-refactor-authoring-guide.md",
            "code movement, full closure gates",
        ),
        (
            "docs/specifications/correctness-authority-model.md",
            "risk-accepted by governance",
        ),
    ] {
        assert!(
            !load_text(path).contains(forbidden),
            "{path} retained contradictory lifecycle language: {forbidden}"
        );
    }
}

#[test]
fn adr0021_thresholds_and_correctness_authority_remain_protected() {
    let adr = load_text("docs/decisions/0021-module-coverage-closure-thresholds.md");
    for protected in [
        "**≥ 90% region AND ≥ 90% line.**",
        "**≥ 85% region AND ≥ 85% line.**",
        "no eligible function below **75% region**",
        "**Per-function complexity-risk bound (CRAP ≤ 30).**",
        "empty actionable production set",
    ] {
        assert!(adr.contains(protected), "ADR-0021 lost: {protected}");
    }
    assert!(adr.contains("Execution cadence aligned to ADR-0039"));
    assert!(adr.contains("not the default for an ordinary bounded"));
    assert!(adr.contains("critical change, campaign closure, and release close against the whole"));

    let correctness = load_text("docs/specifications/correctness-authority-model.md");
    assert!(correctness.contains("affected A0 admission, A1 hard-invariant, and A3"));
    assert!(correctness.contains("execution integrity and"));
    assert!(correctness.contains("investigation outcomes remain separate axes"));
}

fn assert_object_schemas_are_closed(value: &Value, location: &str) {
    match value {
        Value::Object(object) => {
            if object.get("type") == Some(&Value::String("object".to_owned())) {
                assert!(
                    object.contains_key("additionalProperties")
                        || object.contains_key("unevaluatedProperties"),
                    "object schema must declare a closed or typed map boundary at {location}"
                );
                assert_ne!(
                    object.get("additionalProperties"),
                    Some(&Value::Bool(true)),
                    "object schema cannot be open at {location}"
                );
            }
            for (key, child) in object {
                assert_object_schemas_are_closed(child, &format!("{location}/{key}"));
            }
        }
        Value::Array(array) => {
            for (index, child) in array.iter().enumerate() {
                assert_object_schemas_are_closed(child, &format!("{location}/{index}"));
            }
        }
        _ => {}
    }
}
