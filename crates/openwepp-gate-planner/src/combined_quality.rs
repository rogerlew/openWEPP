//! Authenticated selection of one combined full-regression and quality node.

use std::collections::BTreeSet;

use serde_json::{Value, json};

use crate::canonical::{derived_id, digest};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::policy::{GateDefinition, PolicyBundle};

const FULL_ID: &str = "workspace-full-nextest-v1";
const CRAP_ID: &str = "adjudicated-crap-v1";
const COMBINED_ID: &str = "combined-workspace-quality-v1";

/// Select a repository-pinned proof and, only when all proof invariants hold,
/// replace the duplicate full-workspace nodes with one instrumented node.
///
/// # Errors
///
/// Returns a typed planning error for an internally inconsistent policy or DAG.
#[allow(
    clippy::too_many_lines,
    reason = "the selector is one ordered fail-closed proof-admission transaction"
)]
pub fn select_and_apply(
    policy: &PolicyBundle,
    requested_proof_id: Option<&str>,
    context: &Value,
    base_commit: &str,
    nodes: Vec<Value>,
) -> Result<(Vec<Value>, Value)> {
    let requested_proof_id =
        requested_proof_id.or(policy.registry.active_combined_quality_proof_id.as_deref());
    let has_full = nodes
        .iter()
        .any(|node| node["gate_definition_id"] == FULL_ID);
    let has_crap = nodes
        .iter()
        .any(|node| node["gate_definition_id"] == CRAP_ID);
    if !has_full || !has_crap {
        return Ok((
            nodes,
            decision(
                "NOT_APPLICABLE",
                "NO_DUPLICATE_FULL_INVENTORY",
                requested_proof_id,
                None,
                None,
                0,
            ),
        ));
    }
    let Some(requested) = requested_proof_id else {
        return Ok((
            nodes,
            decision(
                "SEPARATE",
                "COMBINATION_NOT_ADOPTED_NO_PROOF",
                None,
                None,
                None,
                0,
            ),
        ));
    };
    let Some(proof) = policy
        .registry
        .combined_quality_proofs
        .iter()
        .find(|proof| proof["proof_id"] == requested)
    else {
        return Ok((
            nodes,
            decision(
                "SEPARATE",
                "COMBINATION_NOT_ADOPTED_UNAUTHENTICATED_PROOF",
                Some(requested),
                None,
                None,
                0,
            ),
        ));
    };
    let proof_sha = digest(proof)?;
    let baseline_count = proof["baselines"].as_array().map_or(0, Vec::len);
    if derived_id(proof, "proof_id")? != requested {
        return Ok((
            nodes,
            decision(
                "SEPARATE",
                "COMBINATION_NOT_ADOPTED_UNAUTHENTICATED_PROOF",
                Some(requested),
                None,
                Some(&proof_sha),
                baseline_count,
            ),
        ));
    }
    if let Err(reason) = validate_proof(proof, context) {
        return Ok((
            nodes,
            decision(
                "SEPARATE",
                reason,
                Some(requested),
                None,
                Some(&proof_sha),
                baseline_count,
            ),
        ));
    }
    let definition = policy.definition(COMBINED_ID).ok_or_else(|| {
        planning_error("GATE-COMBINED-DEFINITION", "combined definition is missing")
    })?;
    let combined = combined_node(definition, base_commit, &nodes)?;
    let combined_id = combined["node_id"]
        .as_str()
        .ok_or_else(|| planning_error("GATE-COMBINED-NODE", "node identity"))?
        .to_owned();
    let mut output = Vec::with_capacity(nodes.len().saturating_sub(1));
    let mut inserted = false;
    for mut node in nodes {
        let gate = node["gate_definition_id"].as_str().unwrap_or_default();
        if gate == FULL_ID {
            output.push(combined.clone());
            inserted = true;
            continue;
        }
        if gate == CRAP_ID {
            continue;
        }
        if node["prerequisites"]
            .as_array()
            .is_some_and(|items| items.iter().any(|item| item == &combined_id))
        {
            return Err(planning_error(
                "GATE-COMBINED-DAG",
                "combined node cannot be its own predecessor",
            ));
        }
        output.push(node.take());
    }
    if !inserted {
        return Err(planning_error(
            "GATE-COMBINED-DAG",
            "full node was absent from the selected DAG",
        ));
    }
    Ok((
        output,
        decision(
            "COMBINED",
            "ADOPTED_AUTHENTICATED_PARITY_ECONOMY",
            Some(requested),
            Some(requested),
            Some(&proof_sha),
            baseline_count,
        ),
    ))
}

fn validate_proof(proof: &Value, context: &Value) -> std::result::Result<(), &'static str> {
    if proof["status"] != "PROVEN" {
        return Err("COMBINATION_NOT_ADOPTED_PROOF_FAILED");
    }
    if proof["environment_manifest_sha256"] != context["environment_manifest_sha256"]
        || proof["tool_manifest_sha256"] != context["tool_manifest_sha256"]
        || proof["host_class"] != context["runner_host_class"]
        || proof["runner_image_sha256"] != context["runner_image_sha256"]
    {
        return Err("COMBINATION_NOT_ADOPTED_CONTEXT_DRIFT");
    }
    let baselines = proof["baselines"]
        .as_array()
        .ok_or("COMBINATION_NOT_ADOPTED_INSUFFICIENT_HISTORY")?;
    if baselines.len() != 3 {
        return Err("COMBINATION_NOT_ADOPTED_INSUFFICIENT_HISTORY");
    }
    let mut baseline_ids = BTreeSet::new();
    let mut receipt_ids = BTreeSet::new();
    let mut envelope_ids = BTreeSet::new();
    let mut host: Option<(&Value, &Value)> = None;
    let mut full_times = Vec::new();
    let mut coverage_times = Vec::new();
    let mut combined_times = Vec::new();
    let mut inventory: Option<&Value> = None;
    for baseline in baselines {
        if baseline["trust_class"] != "PROTECTED_CI"
            || !baseline_ids.insert(
                baseline["baseline_id"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            )
        {
            return Err("COMBINATION_NOT_ADOPTED_UNAUTHENTICATED_PROOF");
        }
        let identity = (&baseline["host_class"], &baseline["runner_image_sha256"]);
        if identity != (&proof["host_class"], &proof["runner_image_sha256"]) {
            return Err("COMBINATION_NOT_ADOPTED_CONTEXT_DRIFT");
        }
        if host.is_some_and(|expected| expected != identity) {
            return Err("COMBINATION_NOT_ADOPTED_CONTEXT_DRIFT");
        }
        host = Some(identity);
        for kind in ["full", "coverage", "combined"] {
            let measurement = &baseline[kind];
            if measurement["result"] != "PASS"
                || !receipt_ids.insert(
                    measurement["receipt_id"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
                || !envelope_ids.insert(
                    measurement["envelope_sha256"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
            {
                return Err("COMBINATION_NOT_ADOPTED_PARITY_FAILED");
            }
        }
        if baseline["full"]["inventory_sha256"] != baseline["coverage"]["inventory_sha256"]
            || baseline["full"]["inventory_sha256"] != baseline["combined"]["inventory_sha256"]
            || inventory.is_some_and(|expected| expected != &baseline["full"]["inventory_sha256"])
        {
            return Err("COMBINATION_NOT_ADOPTED_PARITY_FAILED");
        }
        inventory = Some(&baseline["full"]["inventory_sha256"]);
        if ["junit_sha256", "lcov_sha256", "crap_sha256"]
            .iter()
            .any(|field| {
                baseline["combined"][field]
                    .as_str()
                    .is_none_or(|value| value.len() != 64)
            })
        {
            return Err("COMBINATION_NOT_ADOPTED_COVERAGE_INCOMPLETE");
        }
        full_times.push(time(&baseline["full"])?);
        coverage_times.push(time(&baseline["coverage"])?);
        combined_times.push(time(&baseline["combined"])?);
    }
    full_times.sort_unstable();
    coverage_times.sort_unstable();
    combined_times.sort_unstable();
    let full = full_times[1];
    let coverage = coverage_times[1];
    let combined = combined_times[1];
    if combined.saturating_mul(100) > coverage.saturating_mul(120)
        || combined.saturating_mul(100) > full.saturating_add(coverage).saturating_mul(80)
    {
        return Err("COMBINATION_NOT_ADOPTED_ECONOMY_FAILED");
    }
    Ok(())
}

fn time(value: &Value) -> std::result::Result<u64, &'static str> {
    value["wall_time_ms"]
        .as_u64()
        .filter(|value| *value > 0)
        .ok_or("COMBINATION_NOT_ADOPTED_INSUFFICIENT_HISTORY")
}

fn combined_node(definition: &GateDefinition, base_commit: &str, nodes: &[Value]) -> Result<Value> {
    let full = exactly_one(nodes, FULL_ID)?;
    let _coverage = exactly_one(nodes, CRAP_ID)?;
    let arguments = definition
        .arguments_template
        .iter()
        .map(|argument| argument.replace("{base_commit}", base_commit))
        .collect::<Vec<_>>();
    let mut node = json!({
        "node_id": "0".repeat(64),
        "gate_definition_id": definition.gate_definition_id,
        "gate_family": definition.gate_family,
        "execution_cost_class": definition.execution_cost_class,
        "target": "workspace",
        "features": ["default"],
        "authority_class": definition.authority_class,
        "outcome_policy": definition.outcome_policy,
        "failure_classification": definition.failure_classification,
        "owner": definition.owner,
        "investigation_owner": definition.investigation_owner,
        "boundary": definition.boundary,
        "trust_requirement": definition.trust_requirement,
        "reuse_class": definition.reuse_class,
        "executor": definition.executor,
        "arguments": arguments,
        "working_directory": ".",
        "environment_allowlist": definition.environment_allowlist,
        "prerequisites": full["prerequisites"],
        "expected_inventory": full["expected_inventory"],
        "acceptance": definition.acceptance,
        "timeout_seconds": definition.timeout_seconds,
        "retry": {"maximum_attempts": definition.maximum_attempts, "permitted_reasons": definition.permitted_retry_reasons},
        "artifact_contract": definition.artifact_contract,
        "output_paths": definition.output_paths,
        "blocks_transition": definition.blocks_transition,
        "identity_breakers": definition.identity_breakers,
        "matrix": full["matrix"],
        "shard": {"index": 0, "total": 1}
    });
    node["node_id"] = Value::String(derived_id(&node, "node_id")?);
    Ok(node)
}

fn exactly_one<'a>(nodes: &'a [Value], definition: &str) -> Result<&'a Value> {
    let matches = nodes
        .iter()
        .filter(|node| node["gate_definition_id"] == definition)
        .collect::<Vec<_>>();
    if matches.len() == 1 {
        Ok(matches[0])
    } else {
        Err(planning_error(
            "GATE-COMBINED-DAG",
            format!("expected one {definition}, found {}", matches.len()),
        ))
    }
}

fn decision(
    selected: &str,
    reason: &str,
    requested_proof_id: Option<&str>,
    accepted_proof_id: Option<&str>,
    proof_sha256: Option<&str>,
    baseline_count: usize,
) -> Value {
    json!({
        "decision": selected,
        "reason_code": reason,
        "requested_proof_id": requested_proof_id,
        "accepted_proof_id": accepted_proof_id,
        "proof_sha256": proof_sha256,
        "baseline_count": baseline_count,
    })
}

fn planning_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Planning, code, message)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serde_json::{Value, json};

    use super::{select_and_apply, validate_proof};
    use crate::canonical::{derived_id, parse_strict, validate_schema};
    use crate::policy::PolicyBundle;

    fn measurement(id: u8, inventory: &str, wall_time_ms: u64) -> Value {
        json!({
            "receipt_id": format!("{id:064x}"),
            "envelope_sha256": format!("{:064x}", id + 20),
            "inventory_sha256": inventory,
            "result": "PASS",
            "wall_time_ms": wall_time_ms,
        })
    }

    fn proof(full_ms: u64, coverage_ms: u64, combined_ms: u64) -> Value {
        let inventory = "a".repeat(64);
        let baselines = (1..=3)
            .map(|id| {
                let mut combined = measurement(id + 10, &inventory, combined_ms);
                combined["junit_sha256"] = json!("b".repeat(64));
                combined["lcov_sha256"] = json!("c".repeat(64));
                combined["crap_sha256"] = json!("d".repeat(64));
                json!({
                    "baseline_id": format!("baseline-{id}"),
                    "trust_class": "PROTECTED_CI",
                    "host_class": "forest1-x86_64",
                    "runner_image_sha256": "e".repeat(64),
                    "full": measurement(id, &inventory, full_ms),
                    "coverage": measurement(id + 3, &inventory, coverage_ms),
                    "combined": combined,
                })
            })
            .collect::<Vec<_>>();
        let mut proof = json!({
            "schema_version": "openwepp-combined-quality-proof-v1",
            "proof_id": "0".repeat(64),
            "status": "PROVEN",
            "reason_code": "THREE_BASELINE_PARITY_ECONOMY",
            "environment_manifest_sha256": "1".repeat(64),
            "tool_manifest_sha256": "2".repeat(64),
            "host_class": "forest1-x86_64",
            "runner_image_sha256": "e".repeat(64),
            "baselines": baselines,
        });
        proof["proof_id"] = json!(derived_id(&proof, "proof_id").expect("proof identity"));
        proof
    }

    fn context() -> Value {
        json!({
            "environment_manifest_sha256": "1".repeat(64),
            "tool_manifest_sha256": "2".repeat(64),
            "runner_host_class": "forest1-x86_64",
            "runner_image_sha256": "e".repeat(64),
        })
    }

    fn source_nodes() -> Vec<Value> {
        vec![
            json!({
                "node_id": "1".repeat(64),
                "gate_definition_id": "workspace-full-nextest-v1",
                "prerequisites": ["0".repeat(64)],
                "expected_inventory": {"mode": "EXACT", "ids": ["a".repeat(64)], "minimum_count": 1},
                "matrix": {"target": "x86_64-unknown-linux-gnu"}
            }),
            json!({"node_id": "2".repeat(64), "gate_definition_id": "adjudicated-crap-v1"}),
        ]
    }

    #[test]
    fn exact_economy_thresholds_are_recomputed() {
        validate_proof(&proof(100, 60, 72), &context()).expect("120 percent boundary");
        assert_eq!(
            validate_proof(&proof(100, 60, 73), &context()),
            Err("COMBINATION_NOT_ADOPTED_ECONOMY_FAILED")
        );
        validate_proof(&proof(100, 300, 320), &context()).expect("80 percent boundary");
        assert_eq!(
            validate_proof(&proof(100, 300, 321), &context()),
            Err("COMBINATION_NOT_ADOPTED_ECONOMY_FAILED")
        );
    }

    #[test]
    fn parity_context_and_coverage_mutations_are_rejected() {
        let mut value = proof(100, 100, 80);
        value["baselines"][0]["combined"]["inventory_sha256"] = json!("f".repeat(64));
        assert_eq!(
            validate_proof(&value, &context()),
            Err("COMBINATION_NOT_ADOPTED_PARITY_FAILED")
        );
        let mut value = proof(100, 100, 80);
        value["tool_manifest_sha256"] = json!("f".repeat(64));
        assert_eq!(
            validate_proof(&value, &context()),
            Err("COMBINATION_NOT_ADOPTED_CONTEXT_DRIFT")
        );
        let mut value = proof(100, 100, 80);
        value["baselines"][1]["combined"]["lcov_sha256"] = Value::Null;
        assert_eq!(
            validate_proof(&value, &context()),
            Err("COMBINATION_NOT_ADOPTED_COVERAGE_INCOMPLETE")
        );
    }

    #[test]
    fn pinned_proof_replaces_both_duplicate_nodes() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut policy = PolicyBundle::load(&root).expect("policy");
        let proof = proof(100, 100, 80);
        let proof_id = proof["proof_id"].as_str().expect("proof ID").to_owned();
        let schema = parse_strict(
            &std::fs::read(root.join("gate-policy/v1/schemas/gate-definitions.schema.json"))
                .expect("definition schema"),
        )
        .expect("parse definition schema");
        let mut registry = policy.registry_value.clone();
        registry["combined_quality_proofs"]
            .as_array_mut()
            .expect("proof registry")
            .push(proof.clone());
        validate_schema(&schema, &registry, "proof-pinned registry")
            .expect("pinned proof must satisfy policy schema");
        policy.registry.combined_quality_proofs.push(proof);
        policy.registry.active_combined_quality_proof_id = Some(proof_id.clone());
        let (nodes, decision) = select_and_apply(
            &policy,
            None,
            &context(),
            "a".repeat(40).as_str(),
            source_nodes(),
        )
        .expect("policy-owned combined selection");
        assert_eq!(decision["decision"], "COMBINED");
        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0]["gate_definition_id"],
            "combined-workspace-quality-v1"
        );
        assert_eq!(nodes[0]["expected_inventory"]["mode"], "EXACT");
        assert_eq!(nodes[0]["output_paths"].as_array().map(Vec::len), Some(3));
    }
}
