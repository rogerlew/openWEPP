use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

use crate::assurance::{plan_assurance_impacts, reconcile_assurance_impacts};
use crate::canonical::{derived_id, digest, parse_strict, sha256_bytes, validate_schema};
use crate::documentation::{append_lint_paths, changed_markdown_paths};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::execution_context::cargo_configuration_manifest;
pub(crate) use crate::execution_context::environment_record;
use crate::nextest_inventory::collect_testcases;
use crate::policy::{GateDefinition, PolicyBundle, RiskClass};
use crate::repository::{
    CargoGraph, ObservedChange, ObservedSource, Snapshot, host_target_triple,
    neutral_cargo_command, neutral_git_command, observe_committed,
    observe_committed_after_mutation, observe_dirty,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanningStage {
    Intent,
    Terminal,
}

impl PlanningStage {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Intent => "INTENT",
            Self::Terminal => "TERMINAL",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlanRequest {
    pub stage: PlanningStage,
    pub predecessor_intent_plan_id: Option<String>,
    pub boundary: String,
    pub campaign_id: Option<String>,
    pub combined_quality_proof_id: Option<String>,
    pub authorized_paths: Vec<String>,
    pub source: ObservedSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reconciliation {
    pub added_paths: Vec<String>,
    pub removed_paths: Vec<String>,
    pub risk_escalated: bool,
}

/// Reconcile an exact terminal plan to its accepted intent predecessor.
///
/// # Errors
/// Returns a planning error for invalid stages, predecessor identity, paths, risk, or deferral.
pub fn reconcile_intent_terminal(
    repo: &Path,
    intent: &Value,
    terminal: &Value,
) -> Result<Reconciliation> {
    let schema = load_json(&repo.join("gate-policy/v1/schemas/gate-plan.schema.json"))?;
    validate_schema(&schema, intent, "intent gate plan")?;
    validate_schema(&schema, terminal, "terminal gate plan")?;
    verify_plan_identity(intent)?;
    verify_plan_identity(terminal)?;
    if digest(&reconstruct_plan(repo, terminal)?)? != digest(terminal)? {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-TERMINAL-RECONSTRUCTION",
            "terminal plan differs from independent current-policy reconstruction",
        ));
    }
    reconcile_semantics(intent, terminal)
}

fn reconcile_semantics(intent: &Value, terminal: &Value) -> Result<Reconciliation> {
    verify_reconciliation_link(intent, terminal)?;
    let intended = changed_paths(intent)?;
    let actual = changed_paths(terminal)?;
    verify_terminal_authorization(intent, terminal, &actual)?;
    let intent_risk = risk_rank(intent["risk"]["class"].as_str())?;
    let terminal_risk = risk_rank(terminal["risk"]["class"].as_str())?;
    let removed_paths = intended.difference(&actual).cloned().collect();
    verify_monotonic_risk(intent_risk, terminal_risk)?;
    verify_terminal_superset(intent, terminal)?;
    verify_no_terminal_deferral(terminal)?;
    Ok(Reconciliation {
        added_paths: actual.difference(&intended).cloned().collect(),
        removed_paths,
        risk_escalated: terminal_risk > intent_risk,
    })
}

fn verify_reconciliation_link(intent: &Value, terminal: &Value) -> Result<()> {
    require_reconciliation_link(intent["planning_stage"] == "INTENT")?;
    require_reconciliation_link(terminal["planning_stage"] == "TERMINAL")?;
    require_reconciliation_link(terminal["predecessor_intent_plan_id"] == intent["plan_id"])
}

fn require_reconciliation_link(matches: bool) -> Result<()> {
    if matches {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-PLAN-RECONCILIATION",
            "terminal plan does not name the supplied intent plan",
        ))
    }
}

fn changed_paths(plan: &Value) -> Result<BTreeSet<String>> {
    plan["changed_objects"]
        .as_array()
        .ok_or_else(|| {
            GatePolicyError::new(
                ErrorClass::Planning,
                "GATE-PLAN-CHANGES",
                "changed_objects is not an array",
            )
        })?
        .iter()
        .map(|change| {
            change["path"].as_str().map(str::to_owned).ok_or_else(|| {
                GatePolicyError::new(
                    ErrorClass::Planning,
                    "GATE-PLAN-CHANGE-PATH",
                    "changed path is missing",
                )
            })
        })
        .collect()
}

fn verify_terminal_authorization(
    intent: &Value,
    terminal: &Value,
    actual: &BTreeSet<String>,
) -> Result<()> {
    let authorized = string_set(&intent["authorized_paths"], "/authorized_paths")?;
    let terminal_authorized = string_set(&terminal["authorized_paths"], "/authorized_paths")?;
    if authorized != terminal_authorized
        || !actual.is_subset(&authorized)
        || intent["combined_quality"]["requested_proof_id"]
            != terminal["combined_quality"]["requested_proof_id"]
    {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-TERMINAL-UNAUTHORIZED-PATH",
            "terminal changes exceed the exact authorized intent surface",
        ));
    }
    Ok(())
}

fn verify_monotonic_risk(intent_risk: u8, terminal_risk: u8) -> Result<()> {
    if terminal_risk < intent_risk {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-TERMINAL-NONMONOTONIC",
            "terminal planning cannot downgrade risk",
        ));
    }
    Ok(())
}

fn verify_no_terminal_deferral(terminal: &Value) -> Result<()> {
    if !terminal["deferred_obligations"]
        .as_array()
        .is_some_and(Vec::is_empty)
    {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-TERMINAL-RETROACTIVE-DEFERRAL",
            "terminal plans cannot create deferred obligations",
        ));
    }
    Ok(())
}

fn verify_terminal_superset(intent: &Value, terminal: &Value) -> Result<()> {
    for pointer in ["/affected_packages", "/reverse_dependencies"] {
        require_string_superset(intent, terminal, pointer)?;
    }
    require_node_superset(intent, terminal)?;
    require_value_superset(intent, terminal, "/impact_edges")?;
    reconcile_assurance_impacts(intent, terminal, &changed_paths(terminal)?)
}

fn require_string_superset(intent: &Value, terminal: &Value, pointer: &str) -> Result<()> {
    let values = |plan: &Value| -> Result<BTreeSet<String>> {
        plan.pointer(pointer)
            .and_then(Value::as_array)
            .ok_or_else(|| GatePolicyError::new(ErrorClass::Planning, "GATE-PLAN-SHAPE", pointer))?
            .iter()
            .map(|value| {
                value.as_str().map(str::to_owned).ok_or_else(|| {
                    GatePolicyError::new(ErrorClass::Planning, "GATE-PLAN-SHAPE", pointer)
                })
            })
            .collect()
    };
    require_superset(&values(intent)?, &values(terminal)?, pointer)
}

fn require_node_superset(intent: &Value, terminal: &Value) -> Result<()> {
    let intent_nodes = nodes_by_key(intent)?;
    let terminal_nodes = nodes_by_key(terminal)?;
    let intent_ids = node_id_keys(&intent_nodes)?;
    let terminal_ids = node_id_keys(&terminal_nodes)?;
    for (key, intended) in &intent_nodes {
        let Some(actual) = terminal_nodes.get(key) else {
            if node_is_superseded(intended, terminal_nodes.values().copied()) {
                continue;
            }
            return Err(GatePolicyError::new(
                ErrorClass::Planning,
                "GATE-TERMINAL-OBLIGATION-REMOVED",
                key.clone(),
            ));
        };
        require_node_semantics(key, intended, actual)?;
        require_inventory_superset(key, intended, actual)?;
        require_prerequisite_superset(key, intended, actual, &intent_ids, &terminal_ids)?;
    }
    Ok(())
}

fn node_is_superseded<'a>(intended: &Value, mut terminal: impl Iterator<Item = &'a Value>) -> bool {
    let stronger = match intended["gate_definition_id"].as_str() {
        Some("cargo-package-clippy-v1") => "workspace-clippy-v1",
        Some("cargo-package-nextest-v1") => "workspace-full-nextest-v1",
        _ => return false,
    };
    terminal.any(|node| node["gate_definition_id"] == stronger)
}

fn nodes_by_key(plan: &Value) -> Result<BTreeMap<String, &Value>> {
    array_value(plan, "/nodes")?
        .iter()
        .map(|node| Ok((node_key(node)?, node)))
        .collect()
}

fn node_key(node: &Value) -> Result<String> {
    Ok(format!(
        "{}\0{}",
        node["gate_definition_id"]
            .as_str()
            .ok_or_else(|| plan_shape("/nodes/gate_definition_id"))?,
        node["target"]
            .as_str()
            .ok_or_else(|| plan_shape("/nodes/target"))?
    ))
}

fn node_id_keys<'a>(nodes: &'a BTreeMap<String, &'a Value>) -> Result<BTreeMap<&'a str, &'a str>> {
    nodes
        .iter()
        .map(|(key, node)| {
            node["node_id"]
                .as_str()
                .map(|id| (id, key.as_str()))
                .ok_or_else(|| plan_shape("/nodes/node_id"))
        })
        .collect()
}

fn require_node_semantics(key: &str, intent: &Value, terminal: &Value) -> Result<()> {
    let payload = |node: &Value| -> Result<Value> {
        let mut value = node.clone();
        let object = value.as_object_mut().ok_or_else(|| plan_shape("/nodes"))?;
        object.remove("node_id");
        object.remove("prerequisites");
        object.remove("expected_inventory");
        Ok(value)
    };
    if digest(&payload(intent)?)? == digest(&payload(terminal)?)? {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-TERMINAL-NODE-WEAKENED",
            key,
        ))
    }
}

fn require_inventory_superset(key: &str, intent: &Value, terminal: &Value) -> Result<()> {
    let intended = &intent["expected_inventory"];
    let actual = &terminal["expected_inventory"];
    if intended["mode"] != actual["mode"]
        || actual["minimum_count"].as_u64().unwrap_or(0)
            < intended["minimum_count"].as_u64().unwrap_or(u64::MAX)
    {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-TERMINAL-INVENTORY-WEAKENED",
            key,
        ));
    }
    let intended_ids = string_set(&intended["ids"], "/nodes/expected_inventory/ids")?;
    let actual_ids = string_set(&actual["ids"], "/nodes/expected_inventory/ids")?;
    require_superset(&intended_ids, &actual_ids, key)
}

fn require_prerequisite_superset(
    key: &str,
    intent: &Value,
    terminal: &Value,
    intent_ids: &BTreeMap<&str, &str>,
    terminal_ids: &BTreeMap<&str, &str>,
) -> Result<()> {
    let prerequisites = |node: &Value, ids: &BTreeMap<&str, &str>| -> Result<BTreeSet<String>> {
        array_value(node, "/prerequisites")?
            .iter()
            .map(|value| {
                let id = value.as_str().ok_or_else(|| plan_shape("/prerequisites"))?;
                ids.get(id)
                    .map(|key| (*key).to_owned())
                    .ok_or_else(|| plan_shape("/prerequisites"))
            })
            .collect()
    };
    require_superset(
        &prerequisites(intent, intent_ids)?,
        &prerequisites(terminal, terminal_ids)?,
        key,
    )
}

fn string_set(value: &Value, pointer: &str) -> Result<BTreeSet<String>> {
    value
        .as_array()
        .ok_or_else(|| plan_shape(pointer))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| plan_shape(pointer))
        })
        .collect()
}

fn plan_shape(pointer: &str) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Planning, "GATE-PLAN-SHAPE", pointer)
}

fn require_value_superset(intent: &Value, terminal: &Value, pointer: &str) -> Result<()> {
    let values = |plan: &Value| -> Result<BTreeSet<String>> {
        plan.pointer(pointer)
            .and_then(Value::as_array)
            .ok_or_else(|| GatePolicyError::new(ErrorClass::Planning, "GATE-PLAN-SHAPE", pointer))?
            .iter()
            .map(digest)
            .collect()
    };
    require_superset(&values(intent)?, &values(terminal)?, pointer)
}

fn require_superset<T: Ord>(
    intent: &BTreeSet<T>,
    terminal: &BTreeSet<T>,
    label: &str,
) -> Result<()> {
    if intent.is_subset(terminal) {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-TERMINAL-OBLIGATION-REMOVED",
            label,
        ))
    }
}

fn risk_rank(value: Option<&str>) -> Result<u8> {
    match value {
        Some("EDITORIAL") => Ok(0),
        Some("BOUNDED_COMPONENT") => Ok(1),
        Some("INTEGRATED_DOMAIN") => Ok(2),
        Some("CRITICAL") => Ok(3),
        _ => Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-PLAN-RISK",
            "invalid risk class",
        )),
    }
}

pub trait InventoryProvider {
    /// Return the exact stable inventory for one instantiated gate definition.
    ///
    /// # Errors
    ///
    /// Returns a planning error when enumeration is unavailable or malformed.
    fn inventory(
        &self,
        repo: &Path,
        definition: &GateDefinition,
        target: &str,
    ) -> Result<Vec<String>>;
}

#[derive(Debug, Clone, Copy)]
pub struct NextestInventory;

impl InventoryProvider for NextestInventory {
    fn inventory(
        &self,
        repo: &Path,
        definition: &GateDefinition,
        target: &str,
    ) -> Result<Vec<String>> {
        match definition.inventory_source.as_str() {
            "COMMAND" => Ok(vec![digest(&json!({
                "gate_definition_id": definition.gate_definition_id,
                "executor": definition.executor,
                "arguments_template": definition.arguments_template,
                "target": target,
                "acceptance": definition.acceptance,
                "artifact_contract": definition.artifact_contract
            }))?]),
            "NEXTEST_PACKAGE" | "NEXTEST_WORKSPACE" | "NEXTEST_TEST_TARGET" => {
                nextest_inventory(repo, definition, target)
            }
            "DOCTEST_WORKSPACE" => doctest_inventory(repo),
            "AUTHORITY_SUITES" => authority_suite_inventory(repo),
            value => Err(GatePolicyError::new(
                ErrorClass::Planning,
                "GATE-INVENTORY-SOURCE",
                format!("unsupported inventory source: {value}"),
            )),
        }
    }
}

fn authority_suite_inventory(repo: &Path) -> Result<Vec<String>> {
    let registry_path = repo.join("docs/specifications/external-authority/registry.yaml");
    let registry = fs::read_to_string(&registry_path).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-AUTHORITY-INVENTORY-READ",
            format!("{}: {error}", registry_path.display()),
        )
    })?;
    let mut suites = Vec::new();
    let mut fields = BTreeMap::<String, String>::new();
    let flush = |fields: &mut BTreeMap<String, String>, suites: &mut Vec<String>| {
        if fields.get("status").map(String::as_str) == Some("active")
            && fields.get("authority_level").map(String::as_str) == Some("4")
            && fields.get("gate_lane").map(String::as_str) == Some("required")
            && fields.get("failure_class").map(String::as_str) == Some("hard-fail")
            && fields
                .get("integration_test")
                .is_some_and(|value| !value.is_empty())
            && let Some(suite_id) = fields.get("suite_id")
        {
            suites.push(suite_id.clone());
        }
        fields.clear();
    };
    for line in registry.lines() {
        let stripped = line.trim();
        if let Some(suite_id) = stripped.strip_prefix("- suite_id: ") {
            flush(&mut fields, &mut suites);
            fields.insert("suite_id".to_owned(), suite_id.to_owned());
            continue;
        }
        for key in [
            "status",
            "authority_level",
            "gate_lane",
            "failure_class",
            "integration_test",
        ] {
            if let Some(value) = stripped.strip_prefix(&format!("{key}: ")) {
                fields.insert(key.to_owned(), value.to_owned());
            }
        }
    }
    flush(&mut fields, &mut suites);
    suites.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    suites.dedup();
    if suites.is_empty() {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-AUTHORITY-INVENTORY-EMPTY",
            "no active required hard-fail Level-4 authority suites",
        ));
    }
    Ok(suites)
}

#[derive(Debug, Clone)]
struct ConfinedNextestInventory {
    cargo_target: PathBuf,
}

impl InventoryProvider for ConfinedNextestInventory {
    fn inventory(
        &self,
        repo: &Path,
        definition: &GateDefinition,
        target: &str,
    ) -> Result<Vec<String>> {
        match definition.inventory_source.as_str() {
            "COMMAND" => NextestInventory.inventory(repo, definition, target),
            "NEXTEST_PACKAGE" | "NEXTEST_WORKSPACE" | "NEXTEST_TEST_TARGET" => {
                nextest_inventory_at(repo, definition, target, Some(&self.cargo_target))
            }
            "DOCTEST_WORKSPACE" => doctest_inventory_at(repo, Some(&self.cargo_target)),
            "AUTHORITY_SUITES" => authority_suite_inventory(repo),
            value => Err(GatePolicyError::new(
                ErrorClass::Planning,
                "GATE-INVENTORY-SOURCE",
                format!("unsupported inventory source: {value}"),
            )),
        }
    }
}

/// Recompute the current exact inventory for one already-instantiated plan node.
///
/// # Errors
///
/// Returns a policy or planning error when the definition is missing, the node
/// is malformed, or the live inventory cannot be acquired exactly.
pub(crate) fn inventory_for_node(
    repo: &Path,
    node: &Value,
    cargo_target: Option<&Path>,
) -> Result<Vec<String>> {
    let policy = PolicyBundle::load(repo)?;
    let definition_id = node["gate_definition_id"].as_str().ok_or_else(|| {
        GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-NODE-SHAPE",
            "gate_definition_id",
        )
    })?;
    let definition = policy.definition(definition_id).ok_or_else(|| {
        GatePolicyError::new(ErrorClass::Policy, "GATE-DEFINITION-MISSING", definition_id)
    })?;
    let target = node["target"]
        .as_str()
        .ok_or_else(|| GatePolicyError::new(ErrorClass::Planning, "GATE-NODE-SHAPE", "target"))?;
    let mut inventory = inventory_for_definition(repo, node, definition, target, cargo_target)?;
    inventory.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    inventory.dedup();
    Ok(inventory)
}

fn inventory_for_definition(
    repo: &Path,
    node: &Value,
    definition: &GateDefinition,
    target: &str,
    cargo_target: Option<&Path>,
) -> Result<Vec<String>> {
    match definition.inventory_source.as_str() {
        "NEXTEST_PACKAGES" => package_inventories(repo, node, definition, cargo_target),
        "COMMAND" => NextestInventory.inventory(repo, definition, target),
        "NEXTEST_PACKAGE" | "NEXTEST_WORKSPACE" | "NEXTEST_TEST_TARGET" => {
            nextest_inventory_at(repo, definition, target, cargo_target)
        }
        "DOCTEST_WORKSPACE" => doctest_inventory_at(repo, cargo_target),
        "AUTHORITY_SUITES" => authority_suite_inventory(repo),
        value => Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-INVENTORY-SOURCE",
            format!("unsupported inventory source: {value}"),
        )),
    }
}

fn package_inventories(
    repo: &Path,
    node: &Value,
    definition: &GateDefinition,
    cargo_target: Option<&Path>,
) -> Result<Vec<String>> {
    let mut package_definition = definition.clone();
    "NEXTEST_PACKAGE".clone_into(&mut package_definition.inventory_source);
    let mut inventory = Vec::new();
    for package in node_argument_values(node, "--package")? {
        inventory.extend(nextest_inventory_at(
            repo,
            &package_definition,
            &package,
            cargo_target,
        )?);
    }
    Ok(inventory)
}

fn node_argument_values(node: &Value, flag: &str) -> Result<Vec<String>> {
    let arguments = array_value(node, "/arguments")?;
    let arguments = arguments
        .iter()
        .map(|argument| {
            argument
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| plan_shape("/nodes/arguments"))
        })
        .collect::<Result<Vec<_>>>()?;
    argument_values(&arguments, flag)
}

fn argument_values(arguments: &[String], flag: &str) -> Result<Vec<String>> {
    let mut values = Vec::new();
    let mut index = 0;
    while index < arguments.len() {
        if arguments[index] == flag {
            let value = arguments
                .get(index + 1)
                .ok_or_else(|| plan_shape("/nodes/arguments"))?;
            values.push(value.clone());
            index += 2;
        } else {
            index += 1;
        }
    }
    if values.is_empty() {
        Err(plan_shape("/nodes/arguments/packages"))
    } else {
        Ok(values)
    }
}

pub struct Planner<P> {
    inventory: P,
}

impl<P: InventoryProvider> Planner<P> {
    #[must_use]
    pub const fn new(inventory: P) -> Self {
        Self { inventory }
    }

    /// Build and schema-check a deterministic shadow plan.
    ///
    /// # Errors
    ///
    /// Returns a typed error for invalid policy, repository, graph, inventory, or identity input.
    #[allow(
        clippy::too_many_lines,
        reason = "plan assembly mirrors the versioned gate-plan wire contract"
    )]
    pub fn build(&self, repo: &Path, request: &PlanRequest) -> Result<Value> {
        self.build_with_workspace(repo, request, None)
    }

    #[allow(
        clippy::too_many_lines,
        reason = "plan assembly mirrors the versioned gate-plan wire contract"
    )]
    fn build_with_workspace(
        &self,
        repo: &Path,
        request: &PlanRequest,
        workspace: Option<&Path>,
    ) -> Result<Value> {
        self.build_with_workspace_and_context(repo, request, workspace, None)
    }

    #[allow(
        clippy::too_many_lines,
        reason = "plan assembly mirrors the versioned gate-plan wire contract"
    )]
    fn build_with_workspace_and_context(
        &self,
        repo: &Path,
        request: &PlanRequest,
        workspace: Option<&Path>,
        context_override: Option<&Value>,
    ) -> Result<Value> {
        validate_request(request)?;
        let policy = PolicyBundle::load(repo)?;
        let graph = load_source_graph(repo, &request.source, workspace)?;
        let selection_inputs = selection_changes(request);
        let selection = select(&policy, &graph, &selection_inputs);
        let target_head = request
            .source
            .head_commit
            .as_deref()
            .or(request.source.dirty_tree_digest.as_deref())
            .ok_or_else(|| {
                GatePolicyError::new(
                    ErrorClass::Planning,
                    "GATE-ASSURANCE-TARGET-MISSING",
                    "assurance impact requires an exact commit or dirty-tree identity",
                )
            })?;
        let request_campaign_transfer = request.stage == PlanningStage::Terminal
            && request.source.head_commit.is_some()
            && request.campaign_id.is_some();
        let assurance_impacts = plan_assurance_impacts(
            &policy,
            &graph,
            &selection_inputs,
            target_head,
            request.campaign_id.as_deref(),
            request_campaign_transfer,
        )?;
        let root_revision = request.source.head_commit.as_deref().unwrap_or("HEAD");
        let roots = manifest_roots(repo, root_revision, request.source.head_commit.is_none())?;
        let context = context_override
            .cloned()
            .map_or_else(|| current_execution_context(repo), Ok)?;
        let inventory_snapshot = request.source.head_commit.as_deref().map_or_else(
            || Ok(None),
            |head| {
                workspace
                    .map_or_else(
                        || Snapshot::create(repo, head),
                        |root| Snapshot::create_in(repo, head, &root.join("inventory-snapshots")),
                    )
                    .map(Some)
            },
        )?;
        let inventory_repo = inventory_snapshot.as_ref().map_or(repo, Snapshot::path);
        let nodes = self.build_nodes(
            inventory_repo,
            &policy,
            &selection,
            &request.source.base_commit,
        )?;
        let (nodes, combined_quality) = crate::combined_quality::select_and_apply(
            &policy,
            request.combined_quality_proof_id.as_deref(),
            &context,
            &request.source.base_commit,
            nodes,
        )?;
        let quality_scope = quality_scope(&selection, &nodes);

        let changed_objects = request
            .source
            .changes
            .iter()
            .map(|change| {
                let owner = policy
                    .matching_entries(&change.path)
                    .first()
                    .map_or("openwepp-maintainers", |entry| entry.owner.as_str());
                json!({
                    "path": change.path,
                    "change_kind": change.change_kind,
                    "object_kind": change.object_kind,
                    "old_mode": change.old_mode,
                    "new_mode": change.new_mode,
                    "owner": owner
                })
            })
            .collect::<Vec<_>>();
        let source = source_json(&request.source);
        let zero_work = if nodes.is_empty() {
            Some(json!({
                "verified": true,
                "reason_code": "NO_EXECUTABLE_OBLIGATIONS",
                "evidence_sha256": digest(&json!({"changes": changed_objects, "selection": selection.reason_codes}))?
            }))
        } else {
            None
        };
        let mut plan = json!({
            "schema_version": "openwepp-gate-plan-v1",
            "policy": {
                "policy_id": policy.impact_map.policy_id,
                "policy_sha256": policy.impact_map.policy_sha256,
                "impact_map_generation": policy.impact_map.generation,
                "impact_map_sha256": policy.impact_map_sha256,
                "assurance_registry_generation": policy.assurance_registry.generation,
                "assurance_registry_sha256": policy.assurance_registry_sha256
            },
            "planning_stage": request.stage.as_str(),
            "predecessor_intent_plan_id": request.predecessor_intent_plan_id,
            "plan_id": "0000000000000000000000000000000000000000000000000000000000000000",
            "execution_key": "0000000000000000000000000000000000000000000000000000000000000000",
            "boundary": request.boundary,
            "campaign_id": request.campaign_id,
            "combined_quality": combined_quality,
            "authorized_paths": request.authorized_paths,
            "source": source,
            "changed_objects": changed_objects,
            "affected_packages": selection.affected_packages,
            "reverse_dependencies": selection.reverse_dependencies,
            "impact_edges": selection.impact_edges,
            "risk": {
                "class": selection.risk.as_str(),
                "reason_codes": selection.reason_codes,
                "operator_escalation": false
            },
            "planning_controls": {
                "blocking_promotion_rule": "PROSPECTIVE_POLICY_ONLY",
                "investigation_owner_rule": "NAMED_OWNER_REQUIRED",
                "unknown_input_rule": "ESCALATE_CRITICAL"
            },
            "execution_context": context,
            "nodes": nodes,
            "quality_scope": quality_scope,
            "zero_work_disposition": zero_work,
            "environment_roots": roots,
            "deferred_obligations": [],
            "assurance_impacts": assurance_impacts,
            "unmapped_inputs": selection.unmapped,
            "output_root": "target/gate-plan"
        });
        plan["plan_id"] = Value::String(derive_plan_id(&plan)?);
        plan["execution_key"] = Value::String(derive_execution_key(&plan)?);
        verify_node_graph(array_value(&plan, "/nodes")?)?;
        let schema = load_json(&policy.root.join("schemas/gate-plan.schema.json"))?;
        validate_schema(&schema, &plan, "generated gate plan")?;
        Ok(plan)
    }

    #[allow(
        clippy::too_many_lines,
        reason = "node assembly mirrors the versioned terminal-node wire contract"
    )]
    fn build_nodes(
        &self,
        repo: &Path,
        policy: &PolicyBundle,
        selection: &Selection,
        base_commit: &str,
    ) -> Result<Vec<Value>> {
        let mut instances = BTreeMap::<String, (&GateDefinition, String)>::new();
        let mut explicit = selection
            .explicit_definitions
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        for definition in policy.definitions_for_risk(selection.risk) {
            if definition_applies(definition, selection) {
                add_definition_instances(&mut instances, definition, selection);
            }
        }
        explicit.remove("combined-workspace-quality-v1");
        for id in explicit {
            let definition = policy.definition(&id).ok_or_else(|| {
                GatePolicyError::new(ErrorClass::Policy, "GATE-DEFINITION-MISSING", id.clone())
            })?;
            if definition_applies(definition, selection) {
                add_definition_instances(&mut instances, definition, selection);
            }
        }
        add_prerequisite_closure(policy, selection, &mut instances)?;

        let mut built = BTreeMap::<String, String>::new();
        let mut output = Vec::new();
        let target_triple = host_target_triple(repo)?;
        while built.len() < instances.len() {
            let mut progressed = false;
            for (key, (definition, target)) in &instances {
                if built.contains_key(key) {
                    continue;
                }
                let prerequisite_keys = prerequisite_keys(definition, target, &instances);
                if prerequisite_keys
                    .iter()
                    .any(|dependency| !built.contains_key(dependency))
                {
                    continue;
                }
                let prerequisites = prerequisite_keys
                    .iter()
                    .filter_map(|dependency| built.get(dependency).cloned())
                    .collect::<Vec<_>>();
                let arguments = expand_node_arguments(
                    definition,
                    target,
                    base_commit,
                    &selection.affected_packages,
                    &selection.documentation_paths,
                )?;
                let output_paths = expand_arguments(&definition.output_paths, target, base_commit)?;
                let mut inventory = if definition.inventory_source == "NEXTEST_PACKAGES" {
                    let mut package_definition = (*definition).clone();
                    "NEXTEST_PACKAGE".clone_into(&mut package_definition.inventory_source);
                    let mut inventory = Vec::new();
                    for package in argument_values(&arguments, "--package")? {
                        inventory.extend(self.inventory.inventory(
                            repo,
                            &package_definition,
                            &package,
                        )?);
                    }
                    inventory
                } else {
                    self.inventory.inventory(repo, definition, target)?
                };
                inventory.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
                inventory.dedup();
                if inventory.len() < usize::try_from(definition.minimum_count).unwrap_or(usize::MAX)
                {
                    return Err(GatePolicyError::new(
                        ErrorClass::Planning,
                        "GATE-INVENTORY-EMPTY",
                        format!(
                            "{} expected at least {} items",
                            definition.gate_definition_id, definition.minimum_count
                        ),
                    ));
                }
                let mut node = json!({
                    "node_id": "0000000000000000000000000000000000000000000000000000000000000000",
                    "gate_definition_id": definition.gate_definition_id,
                    "gate_family": definition.gate_family,
                    "execution_cost_class": definition.execution_cost_class,
                    "target": target,
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
                    "prerequisites": prerequisites,
                    "expected_inventory": {"mode": definition.inventory_mode, "ids": inventory, "minimum_count": definition.minimum_count},
                    "acceptance": definition.acceptance,
                    "timeout_seconds": definition.timeout_seconds,
                    "retry": {"maximum_attempts": definition.maximum_attempts, "permitted_reasons": definition.permitted_retry_reasons},
                    "artifact_contract": definition.artifact_contract,
                    "output_paths": output_paths,
                    "blocks_transition": definition.blocks_transition,
                    "identity_breakers": definition.identity_breakers,
                    "matrix": {"target": target_triple},
                    "shard": {"index": 0, "total": 1}
                });
                let node_id = derived_id(&node, "node_id")?;
                node["node_id"] = Value::String(node_id.clone());
                built.insert(key.clone(), node_id);
                output.push(node);
                progressed = true;
            }
            if !progressed {
                return Err(GatePolicyError::new(
                    ErrorClass::Policy,
                    "GATE-NODE-DAG-CYCLE",
                    "selected gate definitions could not be topologically ordered",
                ));
            }
        }
        Ok(output)
    }
}

fn load_source_graph(
    repo: &Path,
    source: &ObservedSource,
    workspace: Option<&Path>,
) -> Result<CargoGraph> {
    let base_graph = match workspace {
        Some(root) => {
            CargoGraph::load_at_commit_in(repo, &source.base_commit, &root.join("graph-snapshots"))?
        }
        None => CargoGraph::load_at_commit(repo, &source.base_commit)?,
    };
    let head_graph = match source.head_commit.as_deref() {
        Some(head) => match workspace {
            Some(root) => CargoGraph::load_at_commit_in(repo, head, &root.join("graph-snapshots"))?,
            None => CargoGraph::load_at_commit(repo, head)?,
        },
        None => CargoGraph::load_current(repo)?,
    };
    Ok(base_graph.union(&head_graph))
}

fn quality_scope(selection: &Selection, nodes: &[Value]) -> Value {
    if selection.risk == RiskClass::Editorial {
        return json!({
            "mode": "NOT_APPLICABLE",
            "production_packages": [],
            "covering_node_ids": [],
            "covering_inventory_ids": [],
            "completeness": "COMPLETE",
            "reason_codes": ["NO_PRODUCTION_SURFACE"]
        });
    }
    let affected_mode = matches!(
        selection.risk,
        RiskClass::BoundedComponent | RiskClass::IntegratedDomain
    ) && !selection.affected_packages.is_empty()
        && selection.unmapped.is_empty();
    if !affected_mode {
        return json!({
            "mode": "GLOBAL",
            "production_packages": [],
            "covering_node_ids": [],
            "covering_inventory_ids": [],
            "completeness": "ESCALATED_GLOBAL",
            "reason_codes": ["AFFECTED_CONTRIBUTION_UNBOUNDED"]
        });
    }

    let affected_nodes = nodes
        .iter()
        .filter(|node| node["gate_definition_id"] == "affected-adjudicated-crap-v1")
        .collect::<Vec<_>>();
    if affected_nodes.len() != 1 {
        return json!({
            "mode": "GLOBAL",
            "production_packages": [],
            "covering_node_ids": [],
            "covering_inventory_ids": [],
            "completeness": "ESCALATED_GLOBAL",
            "reason_codes": ["COVERING_TEST_CONTRIBUTION_UNKNOWN"]
        });
    }
    let affected = affected_nodes[0];
    let covering_node_ids = affected["node_id"]
        .as_str()
        .map(|id| vec![id])
        .unwrap_or_default();
    let covering_inventory_ids = affected["expected_inventory"]["ids"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if covering_inventory_ids.is_empty() {
        return json!({
            "mode": "GLOBAL",
            "production_packages": [],
            "covering_node_ids": [],
            "covering_inventory_ids": [],
            "completeness": "ESCALATED_GLOBAL",
            "reason_codes": ["COVERING_TEST_INVENTORY_EMPTY"]
        });
    }
    json!({
        "mode": "AFFECTED",
        "production_packages": selection.affected_packages,
        "covering_node_ids": covering_node_ids,
        "covering_inventory_ids": covering_inventory_ids,
        "completeness": "COMPLETE",
        "reason_codes": ["TERMINAL_PLAN_COVERING_CLOSURE"]
    })
}

pub(crate) fn derive_plan_id(plan: &Value) -> Result<String> {
    let mut payload = plan.clone();
    let object = payload.as_object_mut().ok_or_else(|| {
        GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-PLAN-SHAPE",
            "plan is not an object",
        )
    })?;
    object.remove("plan_id");
    object.remove("execution_key");
    digest(&payload)
}

pub(crate) fn derive_execution_key(plan: &Value) -> Result<String> {
    digest(&json!({
        "plan_id": plan["plan_id"],
        "environment_roots": plan["environment_roots"],
        "execution_context": plan["execution_context"]
    }))
}

pub(crate) fn verify_plan_identity(plan: &Value) -> Result<()> {
    if plan["plan_id"] != derive_plan_id(plan)?
        || plan["execution_key"] != derive_execution_key(plan)?
    {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-PLAN-IDENTITY",
            "plan or execution identity mismatch",
        ));
    }
    verify_node_graph(array_value(plan, "/nodes")?)
}

pub(crate) fn reconstruct_plan(repo: &Path, plan: &Value) -> Result<Value> {
    reconstruct_plan_with_source(repo, plan, false)
}

pub(crate) fn reconstruct_plan_in(
    repo: &Path,
    plan: &Value,
    workspace: &Path,
    after_source_mutation: bool,
) -> Result<Value> {
    let request = reconstruction_request(repo, plan, after_source_mutation)?;
    let workspace = prepare_reconstruction_workspace(workspace)?;
    Planner::new(ConfinedNextestInventory {
        cargo_target: workspace.join("cargo-target"),
    })
    .build_with_workspace(repo, &request, Some(&workspace))
}

pub(crate) fn reconstruct_plan_in_with_bound_context(
    repo: &Path,
    plan: &Value,
    workspace: &Path,
    after_source_mutation: bool,
) -> Result<Value> {
    let request = reconstruction_request(repo, plan, after_source_mutation)?;
    let workspace = prepare_reconstruction_workspace(workspace)?;
    Planner::new(ConfinedNextestInventory {
        cargo_target: workspace.join("cargo-target"),
    })
    .build_with_workspace_and_context(
        repo,
        &request,
        Some(&workspace),
        Some(&plan["execution_context"]),
    )
}

fn prepare_reconstruction_workspace(workspace: &Path) -> Result<PathBuf> {
    let parent = workspace
        .parent()
        .ok_or_else(|| reconstruction_workspace_error("workspace has no parent"))?;
    require_plain_directory(parent)?;
    match fs::symlink_metadata(workspace) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(reconstruction_workspace_error(
                "workspace exists as a symlink or non-directory",
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(workspace)
                .map_err(|error| reconstruction_workspace_error(error.to_string()))?;
        }
        Err(error) => return Err(reconstruction_workspace_error(error.to_string())),
    }
    let canonical_parent = fs::canonicalize(parent)
        .map_err(|error| reconstruction_workspace_error(error.to_string()))?;
    let canonical_workspace = fs::canonicalize(workspace)
        .map_err(|error| reconstruction_workspace_error(error.to_string()))?;
    if canonical_workspace.parent() != Some(canonical_parent.as_path()) {
        return Err(reconstruction_workspace_error(
            "workspace resolves outside its selected parent",
        ));
    }
    for child in ["cargo-target", "graph-snapshots", "inventory-snapshots"] {
        ensure_plain_child(&canonical_workspace, child)?;
    }
    Ok(canonical_workspace)
}

fn require_plain_directory(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| reconstruction_workspace_error(error.to_string()))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        Err(reconstruction_workspace_error(
            "workspace parent is a symlink or non-directory",
        ))
    } else {
        Ok(())
    }
}

fn ensure_plain_child(root: &Path, name: &str) -> Result<()> {
    let child = root.join(name);
    match fs::symlink_metadata(&child) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => Err(
            reconstruction_workspace_error(format!("{name} is a symlink or non-directory")),
        ),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => fs::create_dir(&child)
            .map_err(|error| reconstruction_workspace_error(error.to_string())),
        Err(error) => Err(reconstruction_workspace_error(error.to_string())),
    }
}

fn reconstruction_workspace_error(message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Io, "GATE-RECONSTRUCTION-WORKSPACE", message)
}

fn reconstruct_plan_with_source(
    repo: &Path,
    plan: &Value,
    after_source_mutation: bool,
) -> Result<Value> {
    let request = reconstruction_request(repo, plan, after_source_mutation)?;
    Planner::new(NextestInventory).build(repo, &request)
}

fn reconstruction_request(
    repo: &Path,
    plan: &Value,
    after_source_mutation: bool,
) -> Result<PlanRequest> {
    let base = plan["source"]["base_commit"]
        .as_str()
        .ok_or_else(|| plan_shape("/source/base_commit"))?;
    let source = if let Some(head) = plan["source"]["head_commit"].as_str() {
        if after_source_mutation {
            observe_committed_after_mutation(repo, base, head)?
        } else {
            observe_committed(repo, base, head)?
        }
    } else {
        observe_dirty(repo, base)?
    };
    let stage = match plan["planning_stage"].as_str() {
        Some("INTENT") => PlanningStage::Intent,
        Some("TERMINAL") => PlanningStage::Terminal,
        _ => return Err(plan_shape("/planning_stage")),
    };
    let authorized_paths = string_set(&plan["authorized_paths"], "/authorized_paths")?
        .into_iter()
        .collect();
    Ok(PlanRequest {
        stage,
        predecessor_intent_plan_id: plan["predecessor_intent_plan_id"]
            .as_str()
            .map(str::to_owned),
        boundary: plan["boundary"]
            .as_str()
            .ok_or_else(|| plan_shape("/boundary"))?
            .to_owned(),
        campaign_id: plan["campaign_id"].as_str().map(str::to_owned),
        combined_quality_proof_id: plan["combined_quality"]["requested_proof_id"]
            .as_str()
            .map(str::to_owned),
        authorized_paths,
        source,
    })
}

fn verify_node_graph(nodes: &[Value]) -> Result<()> {
    let mut seen = BTreeSet::new();
    let mut outputs = BTreeSet::new();
    for node in nodes {
        let id = node["node_id"].as_str().ok_or_else(|| {
            GatePolicyError::new(ErrorClass::Planning, "GATE-NODE-SHAPE", "missing node_id")
        })?;
        if node["node_id"] != derived_id(node, "node_id")? || seen.contains(id) {
            return Err(GatePolicyError::new(
                ErrorClass::Planning,
                "GATE-NODE-IDENTITY",
                id,
            ));
        }
        verify_prerequisites(node, &seen, id)?;
        seen.insert(id);
        verify_unique_outputs(node, &mut outputs, id)?;
    }
    Ok(())
}

fn verify_prerequisites(node: &Value, seen: &BTreeSet<&str>, node_id: &str) -> Result<()> {
    for prerequisite in array_value(node, "/prerequisites")? {
        let id = prerequisite.as_str().ok_or_else(|| {
            GatePolicyError::new(ErrorClass::Planning, "GATE-NODE-SHAPE", node_id)
        })?;
        if !seen.contains(id) {
            return Err(GatePolicyError::new(
                ErrorClass::Planning,
                "GATE-NODE-DAG",
                node_id,
            ));
        }
    }
    Ok(())
}

fn verify_unique_outputs<'a>(
    node: &'a Value,
    outputs: &mut BTreeSet<&'a str>,
    node_id: &str,
) -> Result<()> {
    for output in array_value(node, "/output_paths")? {
        let path = output.as_str().ok_or_else(|| {
            GatePolicyError::new(ErrorClass::Planning, "GATE-NODE-SHAPE", node_id)
        })?;
        if !outputs.insert(path) {
            return Err(GatePolicyError::new(
                ErrorClass::Planning,
                "GATE-NODE-OUTPUT-DUPLICATE",
                path,
            ));
        }
    }
    Ok(())
}

fn array_value<'a>(value: &'a Value, pointer: &str) -> Result<&'a [Value]> {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| GatePolicyError::new(ErrorClass::Planning, "GATE-PLAN-SHAPE", pointer))
}

#[derive(Debug)]
struct Selection {
    risk: RiskClass,
    reason_codes: Vec<String>,
    documentation_paths: Vec<String>,
    affected_packages: Vec<String>,
    reverse_dependencies: Vec<String>,
    explicit_definitions: Vec<String>,
    impact_edges: Vec<Value>,
    unmapped: Vec<Value>,
}

fn select(policy: &PolicyBundle, graph: &CargoGraph, changes: &[ObservedChange]) -> Selection {
    let mut risk = RiskClass::Editorial;
    let mut reasons = BTreeSet::new();
    let mut direct_packages = BTreeSet::new();
    let mut explicit = BTreeSet::new();
    let mut impact_edges = Vec::new();
    let mut unmapped = Vec::new();
    for change in changes {
        let mut mapped = false;
        if is_editorial_documentation_path(&change.path) {
            reasons.insert("DOCUMENTATION_ONLY".to_owned());
            mapped = true;
        } else if let Some(package) = graph.package_for_path(&change.path) {
            risk = risk.max(RiskClass::BoundedComponent);
            if Path::new(&change.path)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("rs"))
                && science_sensitive_package(&package)
            {
                risk = RiskClass::Critical;
                reasons.insert("SCIENCE_PACKAGE_WITHOUT_SEMANTIC_EDGE".to_owned());
            }
            direct_packages.insert(package);
            reasons.insert("CARGO_PACKAGE_CHANGED".to_owned());
            mapped = true;
        }
        if change.change_kind == "DELETE" {
            risk = RiskClass::Critical;
            reasons.insert("SOURCE_OR_CONFIGURATION_DELETED".to_owned());
        }
        if matches!(change.path.as_str(), "Cargo.toml" | "Cargo.lock")
            || change.path.ends_with("/Cargo.toml")
            || change.path.ends_with("/build.rs")
        {
            risk = RiskClass::Critical;
            reasons.insert("CARGO_GRAPH_OR_BUILD_INPUT_CHANGED".to_owned());
            mapped = true;
        }
        for entry in policy.matching_entries(&change.path) {
            mapped = true;
            risk = risk.max(entry.risk_floor);
            reasons.extend(entry.reason_codes.iter().cloned());
            direct_packages.extend(entry.affected_packages.iter().cloned());
            explicit.extend(entry.gate_definition_ids.iter().cloned());
            let selected_ids = entry
                .gate_definition_ids
                .iter()
                .chain(&entry.test_targets)
                .chain(&entry.covering_test_targets)
                .chain(&entry.contracts)
                .chain(&entry.authority_suites)
                .chain(&entry.assurance_watches)
                .cloned()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            reasons.insert(format!("SEMANTIC_SURFACE:{}", entry.semantic_surface));
            impact_edges.push(json!({
                "entry_id": entry.entry_id,
                "changed_path": change.path,
                "selected_ids": selected_ids
            }));
        }
        if !mapped {
            risk = RiskClass::Critical;
            reasons.insert("UNKNOWN_INPUT".to_owned());
            unmapped.push(json!({"path": change.path, "reason_code": "UNKNOWN_INPUT", "escalation": "CRITICAL"}));
        }
    }
    let affected = graph.reverse_closure(&direct_packages);
    let reverse_dependencies = affected
        .difference(&direct_packages)
        .cloned()
        .collect::<Vec<_>>();
    Selection {
        risk,
        reason_codes: reasons.into_iter().collect(),
        documentation_paths: changed_markdown_paths(changes),
        affected_packages: affected.into_iter().collect(),
        reverse_dependencies,
        explicit_definitions: explicit.into_iter().collect(),
        impact_edges,
        unmapped,
    }
}

fn is_editorial_documentation_path(path: &str) -> bool {
    const NORMATIVE_PREFIXES: [&str; 9] = [
        "docs/architecture/",
        "docs/contracts/",
        "docs/decisions/",
        "docs/governance/",
        "docs/numerics/",
        "docs/prompt_templates/",
        "docs/specifications/",
        "docs/standards/",
        "docs/work-packages/templates/",
    ];
    const NORMATIVE_EXACT: [&str; 2] = [
        "docs/codex_exec_plans.md",
        "docs/defect_closure_execplans.md",
    ];
    let documentation = path.starts_with("docs/")
        || path == "README.md"
        || path.ends_with("/README.md")
        || Path::new(path)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("md"));
    documentation
        && path != "AGENTS.md"
        && !path.ends_with("/AGENTS.md")
        && !NORMATIVE_EXACT.contains(&path)
        && !NORMATIVE_PREFIXES
            .iter()
            .any(|prefix| path.starts_with(prefix))
}

fn science_sensitive_package(package: &str) -> bool {
    [
        "climate",
        "hillslope",
        "input-contract",
        "kernel",
        "landuse",
        "legacy",
        "management",
        "meteorology",
        "phenology",
        "runner",
        "sim-contract",
        "summary",
        "topology",
        "unit-boundary",
        "watershed",
    ]
    .iter()
    .any(|token| package.contains(token))
}

fn add_definition_instances<'a>(
    instances: &mut BTreeMap<String, (&'a GateDefinition, String)>,
    definition: &'a GateDefinition,
    selection: &Selection,
) {
    match definition.target_template.as_str() {
        "CARGO_PACKAGE" => {
            for package in &selection.affected_packages {
                instances.insert(
                    format!("{}@{package}", definition.gate_definition_id),
                    (definition, package.clone()),
                );
            }
        }
        "STATIC" => {
            let target = static_definition_target(definition);
            instances.insert(
                format!("{}@{target}", definition.gate_definition_id),
                (definition, target.to_owned()),
            );
        }
        _ => {
            instances.insert(
                format!("{}@workspace", definition.gate_definition_id),
                (definition, "workspace".to_owned()),
            );
        }
    }
}

fn definition_applies(definition: &GateDefinition, selection: &Selection) -> bool {
    definition.gate_definition_id != "documentation-lint-v1"
        || !selection.documentation_paths.is_empty()
}

fn static_definition_target(definition: &GateDefinition) -> &str {
    let selector = match definition.inventory_source.as_str() {
        "NEXTEST_TEST_TARGET" => "--test",
        "NEXTEST_PACKAGE" => "-p",
        _ => return definition.gate_definition_id.as_str(),
    };
    definition
        .arguments_template
        .windows(2)
        .find_map(|pair| (pair[0] == selector).then_some(pair[1].as_str()))
        .unwrap_or(definition.gate_definition_id.as_str())
}

fn prerequisite_keys(
    definition: &GateDefinition,
    target: &str,
    instances: &BTreeMap<String, (&GateDefinition, String)>,
) -> Vec<String> {
    let mut keys = Vec::new();
    for prerequisite in &definition.prerequisite_definition_ids {
        let target_key = format!("{prerequisite}@{target}");
        let workspace_key = format!("{prerequisite}@workspace");
        if instances.contains_key(&target_key) {
            keys.push(target_key);
        } else if instances.contains_key(&workspace_key) {
            keys.push(workspace_key);
        }
    }
    keys.sort();
    keys
}

fn add_prerequisite_closure<'a>(
    policy: &'a PolicyBundle,
    selection: &Selection,
    instances: &mut BTreeMap<String, (&'a GateDefinition, String)>,
) -> Result<()> {
    loop {
        let before = instances.len();
        let selected = instances
            .values()
            .map(|(definition, _)| *definition)
            .collect::<Vec<_>>();
        for definition in selected {
            for prerequisite in &definition.prerequisite_definition_ids {
                let dependency = policy.definition(prerequisite).ok_or_else(|| {
                    GatePolicyError::new(
                        ErrorClass::Policy,
                        "GATE-PREREQUISITE-MISSING",
                        prerequisite,
                    )
                })?;
                if !definition_applies(dependency, selection) {
                    return Err(GatePolicyError::new(
                        ErrorClass::Policy,
                        "GATE-CONDITIONAL-PREREQUISITE",
                        prerequisite,
                    ));
                }
                add_definition_instances(instances, dependency, selection);
            }
        }
        if instances.len() == before {
            return Ok(());
        }
    }
}

fn expand_arguments(template: &[String], target: &str, base_commit: &str) -> Result<Vec<String>> {
    let arguments = template
        .iter()
        .map(|argument| {
            argument
                .replace("{package}", target)
                .replace("{base_commit}", base_commit)
        })
        .collect::<Vec<_>>();
    if arguments
        .iter()
        .any(|argument| argument.contains('{') || argument.contains('}'))
    {
        return Err(GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-ARGUMENT-TEMPLATE",
            "unresolved argument placeholder",
        ));
    }
    Ok(arguments)
}

fn expand_node_arguments(
    definition: &GateDefinition,
    target: &str,
    base_commit: &str,
    affected_packages: &[String],
    documentation_paths: &[String],
) -> Result<Vec<String>> {
    let mut arguments = expand_arguments(&definition.arguments_template, target, base_commit)?;
    if definition.gate_definition_id == "affected-adjudicated-crap-v1" {
        for package in affected_packages {
            arguments.push("--package".to_owned());
            arguments.push(package.clone());
        }
    }
    append_lint_paths(
        &mut arguments,
        &definition.gate_definition_id,
        documentation_paths,
    );
    Ok(arguments)
}

fn validate_request(request: &PlanRequest) -> Result<()> {
    let predecessor_valid = match request.stage {
        PlanningStage::Intent => request.predecessor_intent_plan_id.is_none(),
        PlanningStage::Terminal => request
            .predecessor_intent_plan_id
            .as_ref()
            .is_some_and(|id| is_digest(id)),
    };
    if !predecessor_valid {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-PLAN-STAGE-PREDECESSOR",
            "intent plans omit a predecessor; terminal plans require a digest",
        ));
    }
    if request.boundary != "INCREMENT" {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-PLAN-BOUNDARY",
            "this planner supports INCREMENT only; use the conservative lane for broader lifecycle qualification",
        ));
    }
    let authorized = request.authorized_paths.iter().collect::<BTreeSet<_>>();
    if authorized.is_empty()
        || authorized.len() != request.authorized_paths.len()
        || request.authorized_paths.iter().any(|path| {
            path.starts_with('/') || path.split('/').any(|part| matches!(part, "" | "." | ".."))
        })
    {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-PLAN-AUTHORIZED-PATHS",
            "authorized paths must be nonempty, unique, and repository-relative",
        ));
    }
    if request
        .source
        .changes
        .iter()
        .any(|change| !authorized.contains(&change.path))
    {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-PLAN-UNAUTHORIZED-OBSERVED-PATH",
            "observed changes must remain inside the authorized intent surface",
        ));
    }
    Ok(())
}

fn selection_changes(request: &PlanRequest) -> Vec<ObservedChange> {
    request
        .authorized_paths
        .iter()
        .map(|path| {
            request
                .source
                .changes
                .iter()
                .find(|change| change.path == *path)
                .cloned()
                .unwrap_or_else(|| ObservedChange {
                    path: path.clone(),
                    change_kind: "DECLARED".to_owned(),
                    object_kind: "REGULAR".to_owned(),
                    old_mode: None,
                    new_mode: None,
                })
        })
        .collect()
}

fn is_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn source_json(source: &ObservedSource) -> Value {
    json!({
        "base_commit": source.base_commit,
        "head_commit": source.head_commit,
        "dirty_tree_digest": source.dirty_tree_digest,
        "index_digest": source.index_digest,
        "worktree_digest": source.worktree_digest,
        "untracked_digest": source.untracked_digest
    })
}

fn nextest_inventory(
    repo: &Path,
    definition: &GateDefinition,
    target: &str,
) -> Result<Vec<String>> {
    nextest_inventory_at(repo, definition, target, None)
}

fn nextest_inventory_at(
    repo: &Path,
    definition: &GateDefinition,
    target: &str,
    cargo_target: Option<&Path>,
) -> Result<Vec<String>> {
    let mut command = neutral_cargo_command();
    if let Some(cargo_target) = cargo_target {
        command.env("CARGO_TARGET_DIR", cargo_target);
    }
    command.args([
        "nextest",
        "list",
        "--locked",
        "--offline",
        "--message-format",
        "json",
    ]);
    if definition.inventory_source == "NEXTEST_PACKAGE" {
        command.args(["-p", target]);
    } else if definition.inventory_source == "NEXTEST_TEST_TARGET" {
        command.args(["--test", target]);
    } else {
        command.arg("--workspace");
    }
    let output = command.current_dir(repo).output().map_err(|error| {
        GatePolicyError::new(ErrorClass::Io, "GATE-NEXTEST-LIST-SPAWN", error.to_string())
    })?;
    if !output.status.success() {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-NEXTEST-LIST",
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ));
    }
    let listing = parse_strict(&output.stdout)?;
    let mut inventory = Vec::new();
    collect_testcases(&listing, "", &mut inventory);
    inventory.sort();
    inventory.dedup();
    Ok(inventory)
}

fn doctest_inventory(repo: &Path) -> Result<Vec<String>> {
    doctest_inventory_at(repo, None)
}

fn doctest_inventory_at(repo: &Path, cargo_target: Option<&Path>) -> Result<Vec<String>> {
    let mut command = neutral_cargo_command();
    if let Some(cargo_target) = cargo_target {
        command.env("CARGO_TARGET_DIR", cargo_target);
    }
    let output = command
        .args([
            "test",
            "--workspace",
            "--doc",
            "--locked",
            "--offline",
            "--",
            "--list",
        ])
        .current_dir(repo)
        .output()
        .map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-DOCTEST-LIST-SPAWN", error.to_string())
        })?;
    if !output.status.success() {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-DOCTEST-LIST",
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ));
    }
    let text = String::from_utf8(output.stdout).map_err(|error| {
        GatePolicyError::new(ErrorClass::Planning, "GATE-DOCTEST-UTF8", error.to_string())
    })?;
    let mut inventory = text
        .lines()
        .enumerate()
        .map(|(index, line)| (index, line.trim()))
        .filter(|(_, line)| line.ends_with(": test"))
        .map(|(index, line)| sha256_bytes(format!("{index}\0{line}").as_bytes()))
        .collect::<Vec<_>>();
    inventory.push(sha256_bytes(format!("doctest-list-v1\0{text}").as_bytes()));
    inventory.sort();
    inventory.dedup();
    Ok(inventory)
}

pub(crate) fn manifest_roots(repo: &Path, revision: &str, include_dirty: bool) -> Result<Value> {
    let mut roles = tracked_manifest(repo, revision)?;
    for bytes in dirty_manifest_paths(repo, include_dirty)? {
        let (role, object) = dirty_manifest_object(repo, &bytes)?;
        let path = object["path"].as_str().ok_or_else(|| {
            GatePolicyError::new(ErrorClass::Planning, "GATE-MANIFEST-SHAPE", "missing path")
        })?;
        if !manifest_path_included(path) {
            continue;
        }
        for records in roles.values_mut() {
            records.remove(path);
        }
        if object["object_kind"] != "MISSING" {
            roles
                .entry(role)
                .or_default()
                .insert(path.to_owned(), object);
        }
    }
    let root = |role: &str| -> Result<String> {
        digest(&json!({
            "schema_version": "openwepp-manifest-root-v1",
            "role": role,
            "records": roles.get(role).map(|records| records.values().cloned().collect::<Vec<_>>()).unwrap_or_default()
        }))
    };
    Ok(json!({
        "execution_root": root("execution")?,
        "authority_root": root("authority")?,
        "documentation_root": root("documentation")?,
        "assurance_root": root("assurance")?
    }))
}

fn tracked_manifest(
    repo: &Path,
    revision: &str,
) -> Result<BTreeMap<&'static str, BTreeMap<String, Value>>> {
    let listing = git_bytes(
        repo,
        &["ls-tree", "-r", "-z", revision],
        "GATE-MANIFEST-TREE",
    )?;
    let entries = listing
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
        .map(tracked_tree_entry)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let contents = git_blob_batch(repo, entries.iter().map(|entry| entry.oid.as_str()))?;
    let mut roles: BTreeMap<&'static str, BTreeMap<String, Value>> = BTreeMap::new();
    for (entry, content) in entries.into_iter().zip(contents) {
        let (role, path, record) = tracked_manifest_record(&entry, &content)?;
        roles.entry(role).or_default().insert(path, record);
    }
    Ok(roles)
}

#[derive(Debug)]
struct TrackedTreeEntry {
    mode: String,
    oid: String,
    path: String,
}

fn tracked_tree_entry(entry: &[u8]) -> Result<Option<TrackedTreeEntry>> {
    let text = std::str::from_utf8(entry).map_err(|_| {
        GatePolicyError::new(ErrorClass::GitState, "GATE-MANIFEST-NONUTF8", "tree entry")
    })?;
    let (metadata, path) = text
        .split_once('\t')
        .ok_or_else(|| GatePolicyError::new(ErrorClass::GitState, "GATE-MANIFEST-TREE", text))?;
    if !manifest_path_included(path) {
        return Ok(None);
    }
    let mut fields = metadata.split_whitespace();
    let mode = fields.next().unwrap_or_default();
    let kind = fields.next().unwrap_or_default();
    let oid = fields.next().unwrap_or_default();
    if kind != "blob" || oid.is_empty() || fields.next().is_some() {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-MANIFEST-TREE",
            text,
        ));
    }
    Ok(Some(TrackedTreeEntry {
        mode: mode.to_owned(),
        oid: oid.to_owned(),
        path: path.to_owned(),
    }))
}

fn tracked_manifest_record(
    entry: &TrackedTreeEntry,
    content: &[u8],
) -> Result<(&'static str, String, Value)> {
    if entry.mode == "120000" && std::str::from_utf8(content).is_err() {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-MANIFEST-SYMLINK-NONUTF8",
            entry.path.clone(),
        ));
    }
    let role = path_role(&entry.path);
    let record = json!({
        "path": entry.path,
        "object_kind": if entry.mode == "120000" { "SYMLINK" } else { "REGULAR" },
        "mode": entry.mode,
        "content_sha256": sha256_bytes(content),
        "role": role,
        "owner": "openwepp-maintainers"
    });
    Ok((role, entry.path.clone(), record))
}

fn git_blob_batch<'a>(repo: &Path, oids: impl Iterator<Item = &'a str>) -> Result<Vec<Vec<u8>>> {
    let oid_values = oids.collect::<Vec<_>>();
    let count = oid_values.len();
    let oid_lines = if oid_values.is_empty() {
        String::new()
    } else {
        oid_values.join("\n") + "\n"
    };
    let mut child = neutral_git_command()
        .args(["cat-file", "--batch"])
        .current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-MANIFEST-BLOB", error.to_string())
        })?;
    let mut stdin = child.stdin.take().ok_or_else(|| {
        GatePolicyError::new(ErrorClass::Io, "GATE-MANIFEST-BLOB", "missing stdin")
    })?;
    let writer = std::thread::spawn(move || stdin.write_all(oid_lines.as_bytes()));
    let output = child.wait_with_output().map_err(|error| {
        GatePolicyError::new(ErrorClass::Io, "GATE-MANIFEST-BLOB", error.to_string())
    })?;
    writer
        .join()
        .map_err(|_| manifest_batch_error("stdin writer panicked"))?
        .map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-MANIFEST-BLOB", error.to_string())
        })?;
    if output.status.success() {
        parse_blob_batch(&output.stdout, count)
    } else {
        Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-MANIFEST-BLOB",
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

fn parse_blob_batch(bytes: &[u8], count: usize) -> Result<Vec<Vec<u8>>> {
    let mut cursor = 0_usize;
    let mut blobs = Vec::with_capacity(count);
    for _ in 0..count {
        let header_end = bytes[cursor..]
            .iter()
            .position(|byte| *byte == b'\n')
            .map(|offset| cursor + offset)
            .ok_or_else(|| manifest_batch_error("missing header"))?;
        let header = std::str::from_utf8(&bytes[cursor..header_end])
            .map_err(|_| manifest_batch_error("non-UTF8 header"))?;
        let size = header
            .split_whitespace()
            .nth(2)
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or_else(|| manifest_batch_error("invalid header"))?;
        let start = header_end + 1;
        let end = start
            .checked_add(size)
            .filter(|end| *end < bytes.len() && bytes[*end] == b'\n')
            .ok_or_else(|| manifest_batch_error("truncated blob"))?;
        blobs.push(bytes[start..end].to_vec());
        cursor = end + 1;
    }
    if cursor == bytes.len() {
        Ok(blobs)
    } else {
        Err(manifest_batch_error("trailing bytes"))
    }
}

fn manifest_batch_error(message: &str) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::GitState, "GATE-MANIFEST-BLOB", message)
}

fn manifest_path_included(path: &str) -> bool {
    let root_or_runtime = matches!(
        path,
        "Cargo.toml"
            | "Cargo.lock"
            | "AGENTS.md"
            | "README.md"
            | "deny.toml"
            | "clippy.toml"
            | "rust-toolchain.toml"
    ) || [
        "crates/",
        "src/",
        "tests/",
        "tools/",
        ".cargo/",
        ".config/",
        "gate-policy/",
        "assurance/",
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix));
    let selected_documentation = matches!(path, "docs/ROADMAP.md" | "docs/work-packages/README.md")
        || [
            "docs/standards/",
            "docs/decisions/",
            "docs/work-packages/20260717-testgate-plan-shadow-planner-001/",
        ]
        .iter()
        .any(|prefix| path.starts_with(prefix));
    root_or_runtime || selected_documentation
}

fn git_bytes(repo: &Path, arguments: &[&str], code: &'static str) -> Result<Vec<u8>> {
    let output = neutral_git_command()
        .args(arguments)
        .current_dir(repo)
        .output()
        .map_err(|error| GatePolicyError::new(ErrorClass::Io, code, error.to_string()))?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(GatePolicyError::new(
            ErrorClass::GitState,
            code,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

fn dirty_manifest_paths(repo: &Path, include_dirty: bool) -> Result<BTreeSet<Vec<u8>>> {
    if !include_dirty {
        return Ok(BTreeSet::new());
    }
    let mut paths = BTreeSet::new();
    for arguments in [
        &["diff", "--name-only", "-z", "--no-renames", "HEAD", "--"][..],
        &["ls-files", "--others", "--exclude-standard", "-z"][..],
    ] {
        paths.extend(
            git_bytes(repo, arguments, "GATE-MANIFEST-DELTA")?
                .split(|byte| *byte == 0)
                .filter(|part| !part.is_empty())
                .map(<[u8]>::to_vec),
        );
    }
    Ok(paths)
}

fn dirty_manifest_object(repo: &Path, bytes: &[u8]) -> Result<(&'static str, Value)> {
    let path = std::str::from_utf8(bytes).map_err(|_| {
        GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-MANIFEST-NONUTF8",
            "manifest path is not UTF-8",
        )
    })?;
    let (kind, mode, content_sha256) = manifest_object_identity(repo, path)?;
    let role = path_role(path);
    Ok((
        role,
        json!({
            "path": path, "object_kind": kind, "mode": mode,
            "content_sha256": content_sha256, "role": role,
            "owner": "openwepp-maintainers"
        }),
    ))
}

fn manifest_object_identity(
    repo: &Path,
    path: &str,
) -> Result<(&'static str, Option<&'static str>, Option<String>)> {
    let absolute = repo.join(path);
    match fs::symlink_metadata(&absolute) {
        Ok(metadata) => present_manifest_identity(&absolute, path, &metadata),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(("MISSING", None, None)),
        Err(error) => Err(GatePolicyError::new(
            ErrorClass::Io,
            "GATE-MANIFEST-METADATA",
            format!("{path}: {error}"),
        )),
    }
}

fn present_manifest_identity(
    absolute: &Path,
    path: &str,
    metadata: &fs::Metadata,
) -> Result<(&'static str, Option<&'static str>, Option<String>)> {
    if metadata.file_type().is_symlink() {
        symlink_manifest_identity(absolute, path)
    } else if metadata.is_file() {
        regular_manifest_identity(absolute, path, metadata)
    } else {
        Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-MANIFEST-OBJECT",
            path,
        ))
    }
}

fn symlink_manifest_identity(
    absolute: &Path,
    path: &str,
) -> Result<(&'static str, Option<&'static str>, Option<String>)> {
    let target = fs::read_link(absolute).map_err(|error| {
        GatePolicyError::new(ErrorClass::Io, "GATE-MANIFEST-SYMLINK", error.to_string())
    })?;
    let target = target.to_str().ok_or_else(|| {
        GatePolicyError::new(ErrorClass::GitState, "GATE-MANIFEST-SYMLINK-NONUTF8", path)
    })?;
    Ok((
        "SYMLINK",
        Some("120000"),
        Some(sha256_bytes(target.as_bytes())),
    ))
}

fn regular_manifest_identity(
    absolute: &Path,
    path: &str,
    metadata: &fs::Metadata,
) -> Result<(&'static str, Option<&'static str>, Option<String>)> {
    let content = fs::read(absolute).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Io,
            "GATE-MANIFEST-READ",
            format!("{path}: {error}"),
        )
    })?;
    Ok((
        "REGULAR",
        Some(regular_mode(metadata)),
        Some(sha256_bytes(&content)),
    ))
}

#[cfg(unix)]
fn regular_mode(metadata: &fs::Metadata) -> &'static str {
    use std::os::unix::fs::PermissionsExt;
    if metadata.permissions().mode() & 0o111 == 0 {
        "100644"
    } else {
        "100755"
    }
}

#[cfg(not(unix))]
fn regular_mode(_metadata: &fs::Metadata) -> &'static str {
    "100644"
}

fn path_role(path: &str) -> &'static str {
    if path.starts_with("docs/assurance/") || path.starts_with("assurance/") {
        "assurance"
    } else if path == "AGENTS.md"
        || path.ends_with("/AGENTS.md")
        || path.starts_with("gate-policy/")
        || path.starts_with("docs/standards/")
        || path.starts_with("docs/decisions/")
        || path.starts_with("docs/specifications/")
    {
        "authority"
    } else if path.starts_with("docs/") || path.ends_with("README.md") {
        "documentation"
    } else {
        "execution"
    }
}
pub(crate) fn current_execution_context(repo: &Path) -> Result<Value> {
    let policy = PolicyBundle::load(repo)?;
    let target = host_target_triple(repo)?;
    let environment = environment_record(repo, &target)?;
    let tools = tool_records(repo)?;
    let fixtures = hash_path_manifest(repo, "gate-policy/v1/fixtures/")?;
    let configuration = digest(&json!({
        "impact_map": policy.impact_map_value,
        "gate_definitions": policy.registry_value,
        "execution_matrix": policy.execution_matrix_value,
        "assurance_registry": policy.assurance_registry_value,
        "cargo_lock": sha256_bytes(&fs::read(repo.join("Cargo.lock")).map_err(|error| GatePolicyError::new(ErrorClass::Io, "GATE-CARGO-LOCK", error.to_string()))?),
        "cargo_configuration": cargo_configuration_manifest(repo)?
    }))?;
    Ok(json!({
        "environment_manifest_sha256": digest(&environment)?,
        "runner_host_class": std::env::var("RUNNER_NAME").ok(),
        "runner_image_sha256": environment["runner_image_sha256"],
        "fixture_manifest_sha256": fixtures,
        "tool_manifest_sha256": digest(&tools)?,
        "configuration_sha256": configuration
    }))
}

pub(crate) fn command_identity(repo: &Path, program: &str, arguments: &[&str]) -> Result<String> {
    let mut command = if program == "cargo" {
        neutral_cargo_command()
    } else {
        Command::new(program)
    };
    let output = command
        .args(arguments)
        .current_dir(repo)
        .output()
        .map_err(|error| {
            GatePolicyError::new(
                ErrorClass::Io,
                "GATE-TOOL-IDENTITY",
                format!("{program}: {error}"),
            )
        })?;
    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|error| {
            GatePolicyError::new(ErrorClass::Planning, "GATE-TOOL-UTF8", error.to_string())
        })
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-TOOL-IDENTITY",
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

pub(crate) fn tool_records(repo: &Path) -> Result<Value> {
    let specifications: [(&str, &str, &[&str], &str); 21] = [
        ("git", "git", &["--version"], "git"),
        ("cargo-launcher", "cargo", &["--version"], "cargo"),
        ("cargo", "cargo", &["--version"], "@toolchain/cargo"),
        (
            "cargo-fmt-launcher",
            "cargo",
            &["fmt", "--version"],
            "cargo-fmt",
        ),
        (
            "cargo-fmt",
            "cargo",
            &["fmt", "--version"],
            "@toolchain/cargo-fmt",
        ),
        ("rustfmt", "rustfmt", &["--version"], "@toolchain/rustfmt"),
        (
            "cargo-clippy-launcher",
            "cargo",
            &["clippy", "--version"],
            "cargo-clippy",
        ),
        (
            "cargo-clippy",
            "cargo",
            &["clippy", "--version"],
            "@toolchain/cargo-clippy",
        ),
        (
            "clippy-driver",
            "cargo",
            &["clippy", "--version"],
            "@toolchain/clippy-driver",
        ),
        (
            "cargo-nextest",
            "cargo",
            &["nextest", "--version"],
            "cargo-nextest",
        ),
        ("cargo-deny", "cargo", &["deny", "--version"], "cargo-deny"),
        (
            "cargo-llvm-cov",
            "cargo",
            &["llvm-cov", "--version"],
            "cargo-llvm-cov",
        ),
        ("cargo-crap", "cargo", &["crap", "--version"], "cargo-crap"),
        ("rustc-launcher", "rustc", &["-Vv"], "rustc"),
        ("rustc", "rustc", &["-Vv"], "@toolchain/rustc"),
        ("rustdoc-launcher", "rustdoc", &["--version"], "rustdoc"),
        ("rustdoc", "rustdoc", &["--version"], "@toolchain/rustdoc"),
        ("ripgrep", "rg", &["--version"], "rg"),
        (
            "markdown-doc",
            "markdown-doc",
            &["--version"],
            "markdown-doc",
        ),
        ("bash", "bash", &["--version"], "bash"),
        ("tar", "tar", &["--version"], "tar"),
    ];
    specifications
        .iter()
        .map(|(tool_id, program, arguments, byte_program)| {
            let version = command_identity(repo, program, arguments)?;
            let executable = resolve_tool_executable(repo, byte_program)?;
            let bytes = fs::read(&executable).map_err(|error| {
                GatePolicyError::new(
                    ErrorClass::Io,
                    "GATE-TOOL-BYTES",
                    format!("{}: {error}", executable.display()),
                )
            })?;
            Ok(json!({
                "tool_id": tool_id,
                "version": format!("v-{}", &sha256_bytes(version.as_bytes())[..16]),
                "sha256": sha256_bytes(&bytes)
            }))
        })
        .collect::<Result<Vec<_>>>()
        .map(Value::Array)
}

fn resolve_tool_executable(repo: &Path, program: &str) -> Result<std::path::PathBuf> {
    if let Some(component) = program.strip_prefix("@toolchain/") {
        let sysroot = command_identity(repo, "rustc", &["--print", "sysroot"])?;
        let candidate = Path::new(sysroot.trim()).join("bin").join(component);
        return fs::canonicalize(&candidate).map_err(|error| {
            GatePolicyError::new(
                ErrorClass::Io,
                "GATE-TOOL-PATH",
                format!("{}: {error}", candidate.display()),
            )
        });
    }
    let path = std::env::var_os("PATH").ok_or_else(|| {
        GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-TOOL-PATH",
            "PATH is unavailable",
        )
    })?;
    for directory in std::env::split_paths(&path) {
        let candidate = directory.join(program);
        if candidate.is_file() {
            return fs::canonicalize(&candidate).map_err(|error| {
                GatePolicyError::new(ErrorClass::Io, "GATE-TOOL-PATH", error.to_string())
            });
        }
    }
    Err(GatePolicyError::new(
        ErrorClass::Planning,
        "GATE-TOOL-PATH",
        format!("{program} is not on PATH"),
    ))
}

fn hash_path_manifest(repo: &Path, prefix: &str) -> Result<String> {
    let output = neutral_git_command()
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
            "--",
            prefix,
        ])
        .current_dir(repo)
        .output()
        .map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-FIXTURE-LIST", error.to_string())
        })?;
    if !output.status.success() {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-FIXTURE-LIST",
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    let mut manifest = Vec::new();
    for bytes in output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty())
    {
        let path = std::str::from_utf8(bytes).map_err(|_| {
            GatePolicyError::new(
                ErrorClass::GitState,
                "GATE-FIXTURE-PATH",
                "non-UTF8 fixture path",
            )
        })?;
        manifest.push(json!({"path": path, "sha256": sha256_bytes(&fs::read(repo.join(path)).map_err(|error| GatePolicyError::new(ErrorClass::Io, "GATE-FIXTURE-READ", error.to_string()))?)}));
    }
    digest(&Value::Array(manifest))
}

fn load_json(path: &Path) -> Result<Value> {
    let bytes = fs::read(path).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Io,
            "GATE-SCHEMA-READ",
            format!("{}: {error}", path.display()),
        )
    })?;
    parse_strict(&bytes)
}

#[cfg(test)]
mod tests {
    include!("planner_coverage_tests.rs");
}
