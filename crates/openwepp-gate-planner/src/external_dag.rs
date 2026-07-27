//! Authenticated execution of repository-owned external command DAGs.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::canonical::{canonical_bytes, derived_id, digest, parse_strict, validate_schema};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::executor::ExecutionClaims;
use crate::external_outputs::{
    OutputManifest, manifest_declared_outputs, prepare_attempt_root, verify_manifest,
};
use crate::pre_heavy::{ConstructedAudit, append_attempt_record};

pub const EXTERNAL_PLAN_SCHEMA: &str = "openwepp-external-dag-plan-v1";
const EXTERNAL_AUDIT_SCHEMA: &str = "openwepp-external-pre-heavy-audit-v1";
const EXTERNAL_RECEIPT_SCHEMA: &str = "openwepp-external-transaction-receipt-v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalDagPlan {
    pub schema: String,
    pub plan_id: String,
    pub source_plan: SourceBinding,
    pub source_contract: SourceBinding,
    pub transactions: Vec<ExternalTransaction>,
    #[serde(default)]
    pub custody_commands: Vec<ExternalNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceBinding {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalTransaction {
    pub transaction_id: String,
    pub light: Vec<ExternalNode>,
    pub heavy: Vec<ExternalNode>,
    #[serde(default)]
    pub custody_prerequisites: Vec<String>,
    #[serde(default)]
    pub custody_receipts: Vec<CustodyReceiptBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustodyReceiptBinding {
    pub command_id: String,
    pub path: String,
    pub sha256: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalNode {
    pub order: u64,
    pub command_id: String,
    pub argv: Vec<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    pub cwd: String,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    pub cost_class: String,
    pub source_path: String,
    #[serde(default)]
    pub declared_outputs: Vec<String>,
    pub timeout_seconds: u64,
    pub max_attempts: u64,
    pub handoff: String,
    pub harvard_access: String,
}

#[derive(Debug, Clone)]
pub struct ExternalTransitionOptions {
    pub repo: PathBuf,
    pub plan_path: PathBuf,
    pub transaction_id: String,
    pub attempt_root: PathBuf,
    pub ledger: PathBuf,
    pub receipt_path: PathBuf,
    pub custody_root: Option<PathBuf>,
    pub opening_token: Option<PathBuf>,
    pub claims: ExecutionClaims,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalVerifierAttestation {
    pub schema: String,
    pub attestation_id: String,
    pub capability_hash: String,
    pub parent_dispatch_id: String,
    pub agent_task_id: String,
    pub principal: String,
    pub workflow: String,
    pub job: String,
    pub runner: String,
    pub attempt: u64,
    pub script_sha256: String,
    pub argv: Vec<String>,
    pub receipt_sha256: String,
    pub freeze_digest: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalNodeReceipt {
    pub command_id: String,
    pub order: u64,
    pub stage: String,
    pub argv: Vec<String>,
    pub prerequisite_receipt_ids: Vec<String>,
    pub exit_code: i32,
    pub result: String,
    pub output_manifest: OutputManifest,
}

/// Parse and independently validate a committed external-DAG plan.
///
/// # Errors
///
/// Returns a typed error when the document is unreadable, non-canonical,
/// malformed, stale, or violates DAG ordering and execution policy.
pub fn load_plan(path: &Path) -> Result<ExternalDagPlan> {
    let bytes = fs::read(path).map_err(|error| external_error("GATE-EXTERNAL-PLAN-READ", error))?;
    let value = parse_strict(&bytes)?;
    let schema = parse_strict(include_bytes!(
        "../../../gate-policy/v1/schemas/external-dag-plan.schema.json"
    ))?;
    validate_schema(&schema, &value, "external-dag-plan")?;
    let plan: ExternalDagPlan = serde_json::from_value(value.clone())
        .map_err(|error| external_error("GATE-EXTERNAL-PLAN-SHAPE", error))?;
    validate_plan(&plan, &value)?;
    Ok(plan)
}

fn validate_plan(plan: &ExternalDagPlan, value: &Value) -> Result<()> {
    if plan.schema != EXTERNAL_PLAN_SCHEMA {
        return Err(policy_error("GATE-EXTERNAL-PLAN-SCHEMA", &plan.schema));
    }
    if derived_id(value, "plan_id")? != plan.plan_id {
        return Err(policy_error("GATE-EXTERNAL-PLAN-ID", "identity mismatch"));
    }
    let mut transactions = BTreeSet::new();
    let mut global_nodes = BTreeSet::new();
    for transaction in &plan.transactions {
        if transaction.transaction_id.is_empty()
            || !transactions.insert(transaction.transaction_id.as_str())
        {
            return Err(policy_error(
                "GATE-EXTERNAL-TRANSACTION-DUPLICATE",
                &transaction.transaction_id,
            ));
        }
        validate_inventory(transaction, &mut global_nodes)?;
    }
    for node in &plan.custody_commands {
        validate_node(node, "CUSTODY")?;
        if !global_nodes.insert(node.command_id.clone()) {
            return Err(policy_error(
                "GATE-EXTERNAL-NODE-DUPLICATE",
                &node.command_id,
            ));
        }
    }
    validate_global_order(plan)?;
    Ok(())
}

fn validate_inventory(
    transaction: &ExternalTransaction,
    global_nodes: &mut BTreeSet<String>,
) -> Result<()> {
    if transaction.light.is_empty() || transaction.heavy.is_empty() {
        return Err(policy_error(
            "GATE-EXTERNAL-STAGE-EMPTY",
            &transaction.transaction_id,
        ));
    }
    let mut previous_order = None;
    for (stage, nodes) in [("LIGHT", &transaction.light), ("HEAVY", &transaction.heavy)] {
        for node in nodes {
            validate_node(node, stage)?;
            if previous_order.is_some_and(|order| node.order <= order)
                || !global_nodes.insert(node.command_id.clone())
            {
                return Err(policy_error("GATE-EXTERNAL-NODE-ORDER", &node.command_id));
            }
            previous_order = Some(node.order);
        }
    }
    Ok(())
}

fn validate_global_order(plan: &ExternalDagPlan) -> Result<()> {
    let nodes = plan
        .transactions
        .iter()
        .flat_map(|transaction| transaction.light.iter().chain(&transaction.heavy))
        .chain(plan.custody_commands.iter())
        .collect::<Vec<_>>();
    let orders = nodes
        .iter()
        .map(|node| (node.command_id.as_str(), node.order))
        .collect::<BTreeMap<_, _>>();
    if orders.len() != nodes.len() || orders.values().collect::<BTreeSet<_>>().len() != nodes.len()
    {
        return Err(policy_error(
            "GATE-EXTERNAL-GLOBAL-ORDER",
            "command IDs and orders must be globally unique",
        ));
    }
    for node in nodes {
        for prerequisite in &node.prerequisites {
            if orders
                .get(prerequisite.as_str())
                .is_none_or(|order| *order >= node.order)
            {
                return Err(policy_error(
                    "GATE-EXTERNAL-PREREQUISITE-ORDER",
                    &node.command_id,
                ));
            }
        }
    }
    Ok(())
}

fn validate_node(node: &ExternalNode, stage: &str) -> Result<()> {
    if node.argv.is_empty()
        || node.argv.iter().any(|argument| argument.contains('\0'))
        || node.cwd.is_empty()
        || node.timeout_seconds == 0
        || node.max_attempts != 1
        || !matches!(node.cost_class.as_str(), "QUICK" | "DOMAIN" | "HEAVY")
        || !matches!(node.harvard_access.as_str(), "NONE" | "OPENS_HARVARD")
        || stage == "LIGHT" && node.cost_class == "HEAVY"
        || stage == "LIGHT" && node.harvard_access != "NONE"
    {
        return Err(policy_error("GATE-EXTERNAL-NODE-INVALID", &node.command_id));
    }
    for output in &node.declared_outputs {
        confined_relative(output)?;
    }
    Ok(())
}

/// Execute LIGHT, construct READY, durably admit HEAVY, and balance its terminal record.
///
/// # Errors
///
/// Returns a typed error for invalid authority, custody, filesystem state,
/// audit admission, subprocess failure, source mutation, or receipt custody.
pub fn run_external_transition(options: &ExternalTransitionOptions) -> Result<Value> {
    let plan = load_plan(&options.plan_path)?;
    let transaction = plan
        .transactions
        .iter()
        .find(|candidate| candidate.transaction_id == options.transaction_id)
        .ok_or_else(|| {
            policy_error("GATE-EXTERNAL-TRANSACTION-UNKNOWN", &options.transaction_id)
        })?;
    prepare_attempt_root(&options.attempt_root)?;
    require_external_root(&options.repo, &options.attempt_root)?;
    require_regular_ledger(&options.ledger)?;
    verify_source_bindings(&options.repo, &plan)?;
    verify_node_sources(&options.repo, transaction)?;
    verify_custody(transaction)?;
    let custody_receipts = verify_custody_files(options, transaction)?;
    let source_before = source_identity(&options.repo)?;
    let light = execute_stage(
        options,
        transaction,
        "LIGHT",
        &transaction.light,
        Vec::new(),
        &source_before,
        custody_receipts,
    )?;
    let audit = construct_ready_audit(options, &plan, transaction, &light, &source_before)?;
    let started = append_started(options, &plan, &audit)?;
    let outcome = execute_heavy_after_started(
        options,
        &plan,
        transaction,
        &audit,
        &light,
        &source_before,
        &started,
    );
    close_started(options, &plan, &audit, &started, outcome)
}

/// Independently reconstruct an external transaction receipt and its outputs.
///
/// # Errors
///
/// Returns a typed receipt error for identity, inventory, audit, result, or
/// external-output drift.
pub fn verify_external_transaction(plan_path: &Path, receipt: &Value) -> Result<()> {
    let plan = load_plan(plan_path)?;
    if receipt["schema"] != EXTERNAL_RECEIPT_SCHEMA
        || receipt["result"] != "PASS"
        || derived_id(receipt, "receipt_id")? != receipt["receipt_id"]
        || receipt["plan_id"] != plan.plan_id
    {
        return Err(policy_error(
            "GATE-EXTERNAL-RECEIPT-IDENTITY",
            "external receipt identity or result mismatch",
        ));
    }
    let transaction_id = receipt["transaction_id"]
        .as_str()
        .ok_or_else(|| policy_error("GATE-EXTERNAL-RECEIPT-SHAPE", "transaction_id"))?;
    let transaction = plan
        .transactions
        .iter()
        .find(|item| item.transaction_id == transaction_id)
        .ok_or_else(|| policy_error("GATE-EXTERNAL-TRANSACTION-UNKNOWN", transaction_id))?;
    let mut declared = Vec::new();
    verify_receipt_stage(
        &transaction.light,
        &receipt["light"],
        "LIGHT",
        &mut declared,
    )?;
    verify_receipt_stage(
        &transaction.heavy,
        &receipt["heavy"],
        "HEAVY",
        &mut declared,
    )?;
    if receipt["audit"]["schema"] != EXTERNAL_AUDIT_SCHEMA
        || receipt["audit"]["status"] != "READY"
        || derived_id(&receipt["audit"], "audit_id")? != receipt["audit"]["audit_id"]
        || receipt["audit"]["light_receipts"] != receipt["light"]
        || receipt["source_before"] != receipt["source_after"]
    {
        return Err(policy_error(
            "GATE-EXTERNAL-RECEIPT-AUDIT",
            "embedded READY audit or source identity mismatch",
        ));
    }
    Ok(())
}

fn verify_receipt_stage(
    nodes: &[ExternalNode],
    receipts: &Value,
    stage: &str,
    declared_so_far: &mut Vec<PathBuf>,
) -> Result<()> {
    let receipts = receipts
        .as_array()
        .ok_or_else(|| policy_error("GATE-EXTERNAL-RECEIPT-SHAPE", stage))?;
    if receipts.len() != nodes.len() {
        return Err(policy_error("GATE-EXTERNAL-RECEIPT-INVENTORY", stage));
    }
    for (node, value) in nodes.iter().zip(receipts) {
        let receipt: ExternalNodeReceipt = serde_json::from_value(value.clone())
            .map_err(|error| external_error("GATE-EXTERNAL-RECEIPT-SHAPE", error))?;
        if receipt.command_id != node.command_id
            || receipt.order != node.order
            || receipt.stage != stage
            || receipt.result != "PASS"
            || receipt.exit_code != 0
        {
            return Err(policy_error("GATE-EXTERNAL-RECEIPT-NODE", &node.command_id));
        }
        declared_so_far.extend(node.declared_outputs.iter().map(PathBuf::from));
        verify_manifest(
            Path::new(&receipt.output_manifest.root),
            declared_so_far,
            &receipt.output_manifest,
        )?;
    }
    Ok(())
}

fn execute_heavy_after_started(
    options: &ExternalTransitionOptions,
    plan: &ExternalDagPlan,
    transaction: &ExternalTransaction,
    audit: &ConstructedAudit,
    light: &[ExternalNodeReceipt],
    source_before: &str,
    started_entry_sha256: &str,
) -> Result<Value> {
    validate_ready(
        options,
        plan,
        transaction,
        audit,
        light,
        source_before,
        started_entry_sha256,
    )?;
    let prior_outputs = transaction
        .light
        .iter()
        .flat_map(|node| node.declared_outputs.iter().map(PathBuf::from))
        .collect();
    let imported_light = light
        .iter()
        .map(|receipt| {
            let value = serde_json::to_value(receipt)
                .map_err(|error| external_error("GATE-EXTERNAL-RECEIPT-SERIALIZE", error))?;
            Ok((receipt.command_id.clone(), digest(&value)?))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    let heavy = execute_stage(
        options,
        transaction,
        "HEAVY",
        &transaction.heavy,
        prior_outputs,
        source_before,
        imported_light,
    )?;
    if source_identity(&options.repo)? != source_before {
        return Err(policy_error(
            "GATE-EXTERNAL-SOURCE-MUTATION",
            "source checkout changed during transaction",
        ));
    }
    let mut receipt = json!({
        "schema": EXTERNAL_RECEIPT_SCHEMA,
        "receipt_id": "",
        "result": "PASS",
        "plan_id": plan.plan_id,
        "transaction_id": transaction.transaction_id,
        "audit": audit.as_value(),
        "attempt_root": options.attempt_root.display().to_string(),
        "ledger": options.ledger.display().to_string(),
        "claims": claims_value(&options.claims),
        "source_before": source_before,
        "source_after": source_before,
        "light": light,
        "heavy": heavy,
    });
    let receipt_id = derived_id(&receipt, "receipt_id")?;
    receipt["receipt_id"] = Value::String(receipt_id);
    persist_exclusive(&options.receipt_path, &receipt)?;
    Ok(receipt)
}

fn execute_stage(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
    stage: &str,
    nodes: &[ExternalNode],
    mut declared_so_far: Vec<PathBuf>,
    source_before: &str,
    mut receipt_ids: BTreeMap<String, String>,
) -> Result<Vec<ExternalNodeReceipt>> {
    let mut receipts = Vec::new();
    for node in nodes {
        let prerequisites = node
            .prerequisites
            .iter()
            .map(|id| {
                receipt_ids
                    .get(id)
                    .cloned()
                    .ok_or_else(|| policy_error("GATE-EXTERNAL-PREREQUISITE-RECEIPT", id))
            })
            .collect::<Result<Vec<String>>>()?;
        if node.harvard_access == "OPENS_HARVARD" {
            create_opening_token(options, transaction)?;
        }
        declared_so_far.extend(node.declared_outputs.iter().map(PathBuf::from));
        let receipt = execute_node(
            options,
            node,
            stage,
            prerequisites,
            &declared_so_far,
            source_before,
        )?;
        let receipt_value = serde_json::to_value(&receipt)
            .map_err(|error| external_error("GATE-EXTERNAL-RECEIPT-SERIALIZE", error))?;
        receipt_ids.insert(node.command_id.clone(), digest(&receipt_value)?);
        receipts.push(receipt);
    }
    Ok(receipts)
}

fn execute_node(
    options: &ExternalTransitionOptions,
    node: &ExternalNode,
    stage: &str,
    prerequisite_receipt_ids: Vec<String>,
    declared_so_far: &[PathBuf],
    source_before: &str,
) -> Result<ExternalNodeReceipt> {
    let argv = node
        .argv
        .iter()
        .map(|argument| expand_operand(options, argument))
        .collect::<Result<Vec<_>>>()?;
    let executable = &argv[0];
    let cwd = remap_cwd(&options.repo, &options.attempt_root, &node.cwd)?;
    fs::create_dir_all(&cwd).map_err(|error| external_error("GATE-EXTERNAL-CWD-CREATE", error))?;
    let mut command = Command::new(executable);
    command.args(&argv[1..]).current_dir(cwd).env_clear();
    for (name, value) in &node.env {
        command.env(name, expand_operand(options, value)?);
    }
    command.env("OPENWEPP_EXTERNAL_ATTEMPT_ROOT", &options.attempt_root);
    let mut child = command
        .spawn()
        .map_err(|error| external_error("GATE-EXTERNAL-SPAWN", error))?;
    let deadline = Instant::now() + Duration::from_secs(node.timeout_seconds);
    let status = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| external_error("GATE-EXTERNAL-WAIT", error))?
        {
            break status;
        }
        if Instant::now() >= deadline {
            child
                .kill()
                .map_err(|error| external_error("GATE-EXTERNAL-KILL", error))?;
            let _ = child.wait();
            return Err(policy_error("GATE-EXTERNAL-TIMEOUT", &node.command_id));
        }
        std::thread::sleep(Duration::from_millis(25));
    };
    let exit_code = status
        .code()
        .ok_or_else(|| policy_error("GATE-EXTERNAL-SIGNAL", &node.command_id))?;
    if !status.success() {
        return Err(policy_error(
            "GATE-EXTERNAL-COMMAND-FAIL",
            &format!("{} exited {exit_code}", node.command_id),
        ));
    }
    if source_identity(&options.repo)? != source_before {
        return Err(policy_error(
            "GATE-EXTERNAL-SOURCE-MUTATION",
            &node.command_id,
        ));
    }
    let output_manifest = manifest_declared_outputs(&options.attempt_root, declared_so_far)?;
    Ok(ExternalNodeReceipt {
        command_id: node.command_id.clone(),
        order: node.order,
        stage: stage.to_owned(),
        argv,
        prerequisite_receipt_ids,
        exit_code,
        result: "PASS".to_owned(),
        output_manifest,
    })
}

fn expand_operand(options: &ExternalTransitionOptions, operand: &str) -> Result<String> {
    let custody = options
        .custody_root
        .clone()
        .unwrap_or_else(|| PathBuf::from("${CUSTODY_ROOT}"));
    let owned = [
        ("${ATTEMPT_ROOT}", options.attempt_root.clone()),
        ("${OBJECTS_ROOT}", options.attempt_root.join("objects")),
        (
            "${PUBLICATION_ROOT}",
            options.attempt_root.join("publication"),
        ),
        (
            "${CARGO_TARGET_DIR}",
            options.attempt_root.join("cargo-target"),
        ),
        ("${REPO}", options.repo.clone()),
        ("${CUSTODY_ROOT}", custody),
    ];
    let mut expanded = operand.to_owned();
    for (placeholder, value) in &owned {
        expanded = expanded.replace(placeholder, &value.display().to_string());
    }
    if expanded.contains("${") {
        Err(policy_error("GATE-EXTERNAL-PLACEHOLDER-UNKNOWN", operand))
    } else {
        Ok(expanded)
    }
}

fn construct_ready_audit(
    options: &ExternalTransitionOptions,
    plan: &ExternalDagPlan,
    transaction: &ExternalTransaction,
    light: &[ExternalNodeReceipt],
    source_before: &str,
) -> Result<ConstructedAudit> {
    let checks = [
        "package_authority",
        "source_identity",
        "plan_identity",
        "light_receipts",
        "inventory_order",
        "toolchain_environment",
        "fresh_external_root",
        "root_separation",
        "custody_prerequisites",
        "durable_ledger",
    ]
    .map(|check_id| json!({"check_id": check_id, "result": "PASS"}));
    let mut audit = json!({
        "schema": EXTERNAL_AUDIT_SCHEMA,
        "audit_id": "",
        "status": "READY",
        "plan_id": plan.plan_id,
        "transaction_id": transaction.transaction_id,
        "attempt_root": options.attempt_root.display().to_string(),
        "ledger": options.ledger.display().to_string(),
        "ledger_head_sha256": ledger_head(&options.ledger)?,
        "claims": claims_value(&options.claims),
        "source_identity": source_before,
        "light_receipts": light,
        "checks": checks,
    });
    let audit_id = derived_id(&audit, "audit_id")?;
    audit["audit_id"] = Value::String(audit_id);
    ConstructedAudit::from_external(audit)
}

fn validate_ready(
    options: &ExternalTransitionOptions,
    plan: &ExternalDagPlan,
    transaction: &ExternalTransaction,
    audit: &ConstructedAudit,
    light: &[ExternalNodeReceipt],
    source_before: &str,
    started_entry_sha256: &str,
) -> Result<()> {
    let value = audit.as_value();
    if value["plan_id"] != plan.plan_id
        || value["transaction_id"] != transaction.transaction_id
        || value["attempt_root"] != options.attempt_root.display().to_string()
        || value["claims"] != claims_value(&options.claims)
        || value["source_identity"] != source_before
        || value["light_receipts"]
            != serde_json::to_value(light)
                .map_err(|error| external_error("GATE-EXTERNAL-AUDIT-SERIALIZE", error))?
    {
        return Err(policy_error(
            "GATE-EXTERNAL-AUDIT-CONTEXT",
            "READY context changed before HEAVY admission",
        ));
    }
    let bytes =
        fs::read(&options.ledger).map_err(|error| external_error("GATE-EXTERNAL-LEDGER", error))?;
    let last = bytes
        .split(|byte| *byte == b'\n')
        .rev()
        .find(|line| !line.is_empty())
        .ok_or_else(|| policy_error("GATE-EXTERNAL-STARTED-MISSING", "ledger is empty"))?;
    let started = parse_strict(last)?;
    if started["entry_sha256"] != started_entry_sha256
        || started["previous_entry_sha256"] != value["ledger_head_sha256"]
        || started["status"] != "STARTED"
        || started["stage"] != "HEAVY"
        || started["audit_id"] != value["audit_id"]
        || started["plan_id"] != plan.plan_id
        || started["transaction_id"] != transaction.transaction_id
    {
        return Err(policy_error(
            "GATE-EXTERNAL-LEDGER-SUCCESSOR",
            "ledger must equal the audited head plus the exact current STARTED record",
        ));
    }
    Ok(())
}

fn append_started(
    options: &ExternalTransitionOptions,
    plan: &ExternalDagPlan,
    audit: &ConstructedAudit,
) -> Result<String> {
    append_attempt_record(
        &options.ledger,
        json!({
            "record_type": "EXTERNAL_TRANSACTION",
            "status": "STARTED",
            "stage": "HEAVY",
            "phase": "ADMISSION",
            "plan_id": plan.plan_id,
            "transaction_id": options.transaction_id,
            "audit_id": audit.as_value()["audit_id"],
            "attempt_root": options.attempt_root.display().to_string(),
            "claims": claims_value(&options.claims),
        }),
    )
}

fn close_started(
    options: &ExternalTransitionOptions,
    plan: &ExternalDagPlan,
    audit: &ConstructedAudit,
    started: &str,
    outcome: Result<Value>,
) -> Result<Value> {
    let (status, result, reason) = match &outcome {
        Ok(receipt) => ("CLOSED", receipt["result"].clone(), Value::Null),
        Err(error) => (
            "FAILED",
            Value::String("FAIL".to_owned()),
            Value::String(error.code.to_owned()),
        ),
    };
    append_attempt_record(
        &options.ledger,
        json!({
            "record_type": "EXTERNAL_TRANSACTION",
            "status": status,
            "stage": "HEAVY",
            "plan_id": plan.plan_id,
            "transaction_id": options.transaction_id,
            "audit_id": audit.as_value()["audit_id"],
            "started_entry_sha256": started,
            "attempt_root": options.attempt_root.display().to_string(),
            "result": result,
            "reason_code": reason,
        }),
    )?;
    outcome
}

fn create_opening_token(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
) -> Result<()> {
    let token = options.opening_token.as_ref().ok_or_else(|| {
        policy_error(
            "GATE-EXTERNAL-HARVARD-TOKEN-REQUIRED",
            &transaction.transaction_id,
        )
    })?;
    let parent = token.parent().ok_or_else(|| {
        policy_error(
            "GATE-EXTERNAL-HARVARD-TOKEN-PATH",
            &token.display().to_string(),
        )
    })?;
    fs::create_dir_all(parent)
        .map_err(|error| external_error("GATE-EXTERNAL-HARVARD-TOKEN-DIR", error))?;
    let mut stream = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(token)
        .map_err(|error| external_error("GATE-EXTERNAL-HARVARD-TOKEN-EXISTS", error))?;
    stream
        .write_all(b"OPENED_ONCE\n")
        .and_then(|()| stream.sync_all())
        .map_err(|error| external_error("GATE-EXTERNAL-HARVARD-TOKEN-WRITE", error))
}

fn verify_source_bindings(repo: &Path, plan: &ExternalDagPlan) -> Result<()> {
    for binding in [&plan.source_plan, &plan.source_contract] {
        confined_relative(&binding.path)?;
        let bytes = fs::read(repo.join(&binding.path))
            .map_err(|error| external_error("GATE-EXTERNAL-SOURCE-READ", error))?;
        let observed = sha256_bytes(&bytes);
        if observed != binding.sha256 {
            return Err(policy_error("GATE-EXTERNAL-SOURCE-DIGEST", &binding.path));
        }
    }
    Ok(())
}

fn verify_node_sources(repo: &Path, transaction: &ExternalTransaction) -> Result<()> {
    for node in transaction.light.iter().chain(&transaction.heavy) {
        confined_relative(&node.source_path)?;
        let metadata = fs::symlink_metadata(repo.join(&node.source_path))
            .map_err(|error| external_error("GATE-EXTERNAL-NODE-SOURCE", error))?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(policy_error(
                "GATE-EXTERNAL-NODE-SOURCE-TYPE",
                &node.source_path,
            ));
        }
    }
    Ok(())
}

fn verify_custody(transaction: &ExternalTransaction) -> Result<()> {
    let unique = transaction
        .custody_prerequisites
        .iter()
        .collect::<BTreeSet<_>>();
    if unique.len() == transaction.custody_prerequisites.len() {
        Ok(())
    } else {
        Err(policy_error(
            "GATE-EXTERNAL-CUSTODY-DUPLICATE",
            &transaction.transaction_id,
        ))
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "custody admission keeps capability consumption and receipt imports in authority order"
)]
fn verify_custody_files(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
) -> Result<BTreeMap<String, String>> {
    let opens_harvard = transaction
        .heavy
        .iter()
        .any(|node| node.harvard_access == "OPENS_HARVARD");
    if opens_harvard
        && (transaction.custody_prerequisites.len() != 2
            || !transaction
                .custody_receipts
                .iter()
                .any(|binding| binding.kind == "TRANSACTION")
            || !transaction
                .custody_receipts
                .iter()
                .any(|binding| binding.kind == "FREEZE"))
    {
        return Err(policy_error(
            "GATE-EXTERNAL-HARVARD-CUSTODY-INCOMPLETE",
            "Harvard admission requires two attestations plus transaction and freeze receipts",
        ));
    }
    if transaction.custody_prerequisites.is_empty() && transaction.custody_receipts.is_empty() {
        return Ok(BTreeMap::new());
    }
    let root = options.custody_root.as_ref().ok_or_else(|| {
        policy_error(
            "GATE-EXTERNAL-CUSTODY-ROOT-REQUIRED",
            &transaction.transaction_id,
        )
    })?;
    let canonical_root = fs::canonicalize(root)
        .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-ROOT", error))?;
    let mut attestations = Vec::new();
    let mut imported = BTreeMap::new();
    for relative in &transaction.custody_prerequisites {
        confined_relative(relative)?;
        let path = root.join(relative);
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?;
        let canonical = fs::canonicalize(&path)
            .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?;
        if metadata.file_type().is_symlink()
            || !metadata.is_file()
            || !canonical.starts_with(&canonical_root)
        {
            return Err(policy_error("GATE-EXTERNAL-CUSTODY-PATH", relative));
        }
        let value = parse_strict(
            &fs::read(&path)
                .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?,
        )?;
        let schema = parse_strict(include_bytes!(
            "../../../gate-policy/v1/schemas/external-verifier-attestation.schema.json"
        ))?;
        validate_schema(&schema, &value, "external-verifier-attestation")?;
        let attestation: ExternalVerifierAttestation = serde_json::from_value(value.clone())
            .map_err(|error| external_error("GATE-EXTERNAL-ATTESTATION-SHAPE", error))?;
        if attestation.schema != "openwepp-external-verifier-attestation-v1"
            || derived_id(&value, "attestation_id")? != attestation.attestation_id
        {
            return Err(policy_error("GATE-EXTERNAL-ATTESTATION-IDENTITY", relative));
        }
        let command_id = Path::new(relative)
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| policy_error("GATE-EXTERNAL-CUSTODY-PATH", relative))?;
        imported.insert(command_id.to_owned(), attestation.attestation_id.clone());
        attestations.push(attestation);
    }
    if !attestations.is_empty() {
        verify_independent_attestations(&attestations)?;
        consume_capabilities(&canonical_root, &attestations)?;
    }
    for binding in &transaction.custody_receipts {
        confined_relative(&binding.path)?;
        if !matches!(binding.kind.as_str(), "TRANSACTION" | "FREEZE") {
            return Err(policy_error("GATE-EXTERNAL-CUSTODY-KIND", &binding.kind));
        }
        let path = canonical_root.join(&binding.path);
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(policy_error("GATE-EXTERNAL-CUSTODY-PATH", &binding.path));
        }
        let bytes =
            fs::read(&path).map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?;
        if sha256_bytes(&bytes) != binding.sha256 {
            return Err(policy_error("GATE-EXTERNAL-CUSTODY-DIGEST", &binding.path));
        }
        let value = parse_strict(&bytes)?;
        if value["result"] != "PASS" {
            return Err(policy_error("GATE-EXTERNAL-CUSTODY-RESULT", &binding.path));
        }
        let receipt_id = value["receipt_id"]
            .as_str()
            .or_else(|| value["freeze_receipt_id"].as_str())
            .ok_or_else(|| policy_error("GATE-EXTERNAL-CUSTODY-RECEIPT-ID", &binding.path))?;
        if imported
            .insert(binding.command_id.clone(), receipt_id.to_owned())
            .is_some()
        {
            return Err(policy_error(
                "GATE-EXTERNAL-CUSTODY-DUPLICATE",
                &binding.command_id,
            ));
        }
    }
    Ok(imported)
}

fn verify_independent_attestations(attestations: &[ExternalVerifierAttestation]) -> Result<()> {
    if attestations.len() < 2 {
        return Err(policy_error(
            "GATE-EXTERNAL-CUSTODY-CARDINALITY",
            "two independent attestations are required",
        ));
    }
    let first = &attestations[0];
    let same_freeze = attestations
        .iter()
        .all(|item| item.freeze_digest == first.freeze_digest);
    let same_dispatch = attestations
        .iter()
        .all(|item| item.parent_dispatch_id == first.parent_dispatch_id);
    let distinct = [
        attestations
            .iter()
            .map(|item| item.capability_hash.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        attestations
            .iter()
            .map(|item| item.agent_task_id.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        attestations
            .iter()
            .map(|item| item.principal.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        attestations
            .iter()
            .map(|item| {
                (
                    item.workflow.as_str(),
                    item.job.as_str(),
                    item.runner.as_str(),
                    item.attempt,
                )
            })
            .collect::<BTreeSet<_>>()
            .len(),
    ]
    .into_iter()
    .all(|count| count == attestations.len());
    if same_freeze && same_dispatch && distinct {
        Ok(())
    } else {
        Err(policy_error(
            "GATE-EXTERNAL-CUSTODY-INDEPENDENCE",
            "attestations are stale, duplicate, or not independently produced",
        ))
    }
}

fn consume_capabilities(
    custody_root: &Path,
    attestations: &[ExternalVerifierAttestation],
) -> Result<()> {
    let consumed_root = custody_root.join("consumed-capabilities");
    fs::create_dir(&consumed_root)
        .or_else(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                Ok(())
            } else {
                Err(error)
            }
        })
        .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-DIR", error))?;
    let consumed_metadata = fs::symlink_metadata(&consumed_root)
        .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-DIR", error))?;
    if consumed_metadata.file_type().is_symlink() || !consumed_metadata.is_dir() {
        return Err(policy_error(
            "GATE-EXTERNAL-CAPABILITY-DIR-TYPE",
            &consumed_root.display().to_string(),
        ));
    }
    for attestation in attestations {
        let source = custody_root
            .join("capabilities")
            .join(format!("{}.cap", attestation.capability_hash));
        let destination = consumed_root.join(format!("{}.cap", attestation.capability_hash));
        let metadata = fs::symlink_metadata(&source)
            .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-MISSING", error))?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(policy_error(
                "GATE-EXTERNAL-CAPABILITY-TYPE",
                &source.display().to_string(),
            ));
        }
        let preimage = fs::read(&source)
            .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-READ", error))?;
        if sha256_bytes(&preimage) != attestation.capability_hash {
            return Err(policy_error(
                "GATE-EXTERNAL-CAPABILITY-HASH",
                &source.display().to_string(),
            ));
        }
        rustix::fs::renameat_with(
            rustix::fs::CWD,
            &source,
            rustix::fs::CWD,
            &destination,
            rustix::fs::RenameFlags::NOREPLACE,
        )
        .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-CONSUME", error))?;
        FileSync::sync_parent(&destination)?;
    }
    Ok(())
}

struct FileSync;

impl FileSync {
    fn sync_parent(path: &Path) -> Result<()> {
        let parent = path.parent().ok_or_else(|| {
            policy_error(
                "GATE-EXTERNAL-CAPABILITY-PARENT",
                &path.display().to_string(),
            )
        })?;
        fs::File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-SYNC", error))
    }
}

fn require_external_root(repo: &Path, root: &Path) -> Result<()> {
    let repository =
        fs::canonicalize(repo).map_err(|error| external_error("GATE-EXTERNAL-REPO", error))?;
    let attempt =
        fs::canonicalize(root).map_err(|error| external_error("GATE-EXTERNAL-ROOT", error))?;
    if attempt.starts_with(repository) {
        Err(policy_error(
            "GATE-EXTERNAL-ROOT-IN-REPOSITORY",
            &attempt.display().to_string(),
        ))
    } else {
        Ok(())
    }
}

fn require_regular_ledger(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| external_error("GATE-EXTERNAL-LEDGER", error))?;
    if metadata.is_file() && !metadata.file_type().is_symlink() {
        Ok(())
    } else {
        Err(policy_error(
            "GATE-EXTERNAL-LEDGER-TYPE",
            &path.display().to_string(),
        ))
    }
}

fn source_identity(repo: &Path) -> Result<String> {
    let output = Command::new("git")
        .args(["status", "--porcelain=v1", "-z", "--untracked-files=all"])
        .current_dir(repo)
        .output()
        .map_err(|error| external_error("GATE-EXTERNAL-GIT-STATUS", error))?;
    if !output.status.success() {
        return Err(policy_error(
            "GATE-EXTERNAL-GIT-STATUS",
            "git status failed",
        ));
    }
    Ok(sha256_bytes(&output.stdout))
}

fn ledger_head(path: &Path) -> Result<Value> {
    let bytes = fs::read(path).map_err(|error| external_error("GATE-EXTERNAL-LEDGER", error))?;
    let last = bytes
        .split(|byte| *byte == b'\n')
        .rev()
        .find(|line| !line.is_empty());
    match last {
        Some(line) => Ok(parse_strict(line)?["entry_sha256"].clone()),
        None => Ok(Value::Null),
    }
}

fn remap_cwd(repo: &Path, root: &Path, cwd: &str) -> Result<PathBuf> {
    let source = PathBuf::from(cwd);
    if source.is_absolute() {
        if source.starts_with(repo) {
            Ok(repo.to_owned())
        } else {
            Err(policy_error("GATE-EXTERNAL-CWD-ESCAPE", cwd))
        }
    } else {
        confined_relative(cwd)?;
        Ok(root.join("work").join(source))
    }
}

fn confined_relative(value: &str) -> Result<()> {
    let path = Path::new(value);
    if value.is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        Err(policy_error("GATE-EXTERNAL-PATH", value))
    } else {
        Ok(())
    }
}

fn persist_exclusive(path: &Path, value: &Value) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| policy_error("GATE-EXTERNAL-RECEIPT-PATH", &path.display().to_string()))?;
    fs::create_dir_all(parent)
        .map_err(|error| external_error("GATE-EXTERNAL-RECEIPT-DIR", error))?;
    let mut bytes = canonical_bytes(value)?;
    bytes.push(b'\n');
    let mut stream = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|error| external_error("GATE-EXTERNAL-RECEIPT-COLLISION", error))?;
    stream
        .write_all(&bytes)
        .and_then(|()| stream.sync_all())
        .map_err(|error| external_error("GATE-EXTERNAL-RECEIPT-WRITE", error))
}

fn claims_value(claims: &ExecutionClaims) -> Value {
    json!({
        "principal": claims.principal,
        "repository": claims.repository,
        "source_event": claims.source_event,
        "source_ref": claims.source_ref,
        "workflow": claims.workflow,
        "job": claims.job,
        "runner": claims.runner,
        "attempt": claims.attempt,
    })
}

fn sha256_bytes(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}

fn external_error(code: &'static str, error: impl std::fmt::Display) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Execution, code, error.to_string())
}

fn policy_error(code: &'static str, message: &str) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Policy, code, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_heavy_node_in_light_inventory() {
        let node = ExternalNode {
            order: 1,
            command_id: "population".to_owned(),
            argv: vec!["false".to_owned()],
            env: BTreeMap::new(),
            cwd: "work".to_owned(),
            prerequisites: Vec::new(),
            cost_class: "HEAVY".to_owned(),
            source_path: "tool.py".to_owned(),
            declared_outputs: vec!["objects/result".to_owned()],
            timeout_seconds: 1,
            max_attempts: 1,
            handoff: "none".to_owned(),
            harvard_access: "NONE".to_owned(),
        };
        let error = validate_node(&node, "LIGHT").expect_err("must reject");
        assert_eq!(error.code, "GATE-EXTERNAL-NODE-INVALID");
    }

    #[test]
    fn rejects_parent_path() {
        let error = confined_relative("../escape").expect_err("must reject");
        assert_eq!(error.code, "GATE-EXTERNAL-PATH");
    }

    #[test]
    fn placeholder_expansion_is_exact_and_unknowns_fail() {
        let options = ExternalTransitionOptions {
            repo: PathBuf::from("/repo"),
            plan_path: PathBuf::from("/plan"),
            transaction_id: "calibration-v1".to_owned(),
            attempt_root: PathBuf::from("/attempt"),
            ledger: PathBuf::from("/ledger"),
            receipt_path: PathBuf::from("/receipt"),
            custody_root: Some(PathBuf::from("/custody")),
            opening_token: None,
            claims: ExecutionClaims::default(),
        };
        assert_eq!(
            expand_operand(&options, "${OBJECTS_ROOT}/x").expect("known placeholder"),
            "/attempt/objects/x"
        );
        let error = expand_operand(&options, "${CALLER_VALUE}").expect_err("must reject");
        assert_eq!(error.code, "GATE-EXTERNAL-PLACEHOLDER-UNKNOWN");
    }

    #[test]
    fn verifier_labels_cannot_fake_independence() {
        let first = attestation("task-a", "alice", "job-a", "cap-a");
        let mut second = attestation("task-b", "bob", "job-b", "cap-b");
        assert!(verify_independent_attestations(&[first.clone(), second.clone()]).is_ok());
        second.principal.clone_from(&first.principal);
        let error =
            verify_independent_attestations(&[first, second]).expect_err("must reject reuse");
        assert_eq!(error.code, "GATE-EXTERNAL-CUSTODY-INDEPENDENCE");
    }

    fn attestation(
        task: &str,
        principal: &str,
        job: &str,
        capability: &str,
    ) -> ExternalVerifierAttestation {
        ExternalVerifierAttestation {
            schema: "openwepp-external-verifier-attestation-v1".to_owned(),
            attestation_id: "0".repeat(64),
            capability_hash: capability.to_owned(),
            parent_dispatch_id: "dispatch".to_owned(),
            agent_task_id: task.to_owned(),
            principal: principal.to_owned(),
            workflow: "workflow".to_owned(),
            job: job.to_owned(),
            runner: job.to_owned(),
            attempt: 1,
            script_sha256: "1".repeat(64),
            argv: vec!["verify".to_owned()],
            receipt_sha256: "2".repeat(64),
            freeze_digest: "3".repeat(64),
            created_at: "2026-07-27T00:00:00Z".to_owned(),
        }
    }
}
