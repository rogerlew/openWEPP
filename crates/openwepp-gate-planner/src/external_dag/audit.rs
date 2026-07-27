#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "the ten-check audit keeps each independently evaluated authority and its immutable lifecycle proofs contiguous"
)]
fn construct_audit_report(
    options: &ExternalTransitionOptions,
    plan: &ExternalDagPlan,
    transaction: &ExternalTransaction,
    light: &[ExternalNodeReceipt],
    source_before: &RepositoryIdentity,
    attempt_identity: &str,
    ledger_proof: &AttemptLedgerAdmissionProof,
    started_entry_sha256: &str,
) -> Result<Value> {
    let package_result = (|| {
        require_base_ancestor(&options.repo, &plan.authority.base_commit)?;
        let package_audit = validate_package(
            &options.repo,
            &plan.authority.base_commit,
            Path::new(&plan.authority.package_path),
        )?;
        if package_audit["status"] != "READY" {
            return Err(policy_error(
                "GATE-EXTERNAL-PACKAGE-BLOCKED",
                "canonical package admission did not return READY",
            ));
        }
        Ok(package_audit)
    })();
    let package_audit = package_result
        .as_ref()
        .map_or(Value::Null, |value| (*value).clone());
    let source_result = (|| {
        verify_cheap_evidence(&options.repo, &plan.authority.cheap_gate_evidence)?;
        Ok(json!(source_before))
    })();
    let inventory_result = reconstruct_source_inventory(&options.repo, plan).map(|()| {
        json!({
            "plan_id": plan.plan_id,
            "node_count": transaction.light.len() + transaction.heavy.len(),
        })
    });
    let light_result =
        verify_light_receipts(options, transaction, light).map(|()| json!(light.len()));
    let node_result = collect_node_identities(options, transaction).map(|value| json!(value));
    let node_identities = node_result
        .as_ref()
        .map_or_else(|_| Value::Null, Clone::clone);
    let root_result = directory_identity(&options.attempt_root).and_then(|identity| {
        if identity == attempt_identity {
            Ok(json!(identity))
        } else {
            Err(identity_error(
                "GATE-EXTERNAL-ROOT-REPLACED",
                "attempt root changed during audit construction",
            ))
        }
    });
    let separation_result = verify_root_separation(options).map(|()| json!("VERIFIED"));
    let ledger_result = (|| {
        verify_attempt_ledger_admission_proof(&options.ledger, ledger_proof)?;
        verify_ledger_state_without_admission(&options.ledger)?;
        Ok(json!(ledger_proof))
    })();
    let non_custody_ready = package_result.is_ok()
        && source_result.is_ok()
        && inventory_result.is_ok()
        && light_result.is_ok()
        && node_result.is_ok()
        && root_result.is_ok()
        && separation_result.is_ok()
        && ledger_result.is_ok();
    let custody_result = if non_custody_ready {
        consume_custody_capabilities(options, transaction)
    } else {
        Err(policy_error(
            "GATE-EXTERNAL-CUSTODY-NOT-ADMITTED",
            "non-custody audit checks must pass before capability consumption",
        ))
    };
    let consumed_custody_proof = custody_result
        .as_ref()
        .map_or_else(|_| Value::Null, Clone::clone);
    let node_check = match node_result {
        Ok(evidence) => {
            json!({"check_id": "toolchain_environment", "result": "PASS", "evidence": evidence})
        }
        Err(error) => json!({
            "check_id": "toolchain_environment",
            "result": "FAIL",
            "evidence": null,
            "reason_code": error.code,
            "reason": error.message,
        }),
    };
    let checks = vec![
        evaluated_check("package_authority", package_result),
        evaluated_check("source_identity", source_result),
        evaluated_check_ref("plan_identity", &inventory_result),
        evaluated_check("light_receipts", light_result),
        evaluated_check_ref("inventory_order", &inventory_result),
        node_check,
        evaluated_check("fresh_external_root", root_result),
        evaluated_check("root_separation", separation_result),
        evaluated_check("custody_prerequisites", custody_result),
        evaluated_check("durable_ledger", ledger_result),
    ];
    let ready = checks.iter().all(|check| check["result"] == "PASS");
    let mut audit = json!({
        "schema": EXTERNAL_AUDIT_SCHEMA,
        "audit_id": "",
        "status": if ready { "READY" } else { "BLOCKED" },
        "plan_id": plan.plan_id,
        "transaction_id": transaction.transaction_id,
        "attempt_root": options.attempt_root.display().to_string(),
        "attempt_root_identity": attempt_identity,
        "ledger": options.ledger.display().to_string(),
        "custody_root": options.custody_root.as_ref().map(|path| path.display().to_string()),
        "ledger_head_sha256": ledger_proof.admitted_head_sha256,
        "ledger_admission_proof": ledger_proof,
        "started_entry_sha256": started_entry_sha256,
        "claims": claims_value(&options.claims),
        "source_identity": source_before,
        "package_audit": package_audit,
        "node_identities": node_identities,
        "consumed_custody_proof": consumed_custody_proof,
        "light_receipts": light,
        "checks": checks,
    });
    let audit_id = derived_id(&audit, "audit_id")?;
    audit["audit_id"] = Value::String(audit_id);
    Ok(audit)
}

fn evaluated_check_ref(check_id: &str, result: &Result<Value>) -> Value {
    match result {
        Ok(evidence) => {
            json!({"check_id": check_id, "result": "PASS", "evidence": evidence})
        }
        Err(error) => json!({
            "check_id": check_id,
            "result": "FAIL",
            "evidence": null,
            "reason_code": error.code,
            "reason": error.message,
        }),
    }
}

fn evaluated_check(check_id: &str, result: Result<Value>) -> Value {
    match result {
        Ok(evidence) => {
            json!({"check_id": check_id, "result": "PASS", "evidence": evidence})
        }
        Err(error) => json!({
            "check_id": check_id,
            "result": "FAIL",
            "evidence": null,
            "reason_code": error.code,
            "reason": error.message,
        }),
    }
}

fn require_base_ancestor(repo: &Path, base: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["merge-base", "--is-ancestor", base, "HEAD"])
        .current_dir(repo)
        .status()
        .map_err(|error| external_error("GATE-EXTERNAL-AUTHORITY-BASE", error))?;
    if status.success() {
        Ok(())
    } else {
        Err(policy_error(
            "GATE-EXTERNAL-AUTHORITY-BASE",
            "runtime HEAD does not descend from the authenticated intake anchor",
        ))
    }
}

fn verify_cheap_evidence(repo: &Path, bindings: &[SourceBinding]) -> Result<()> {
    for binding in bindings {
        confined_relative(&binding.path)?;
        let path = repo.join(&binding.path);
        if file_sha256(&path)? != binding.sha256 {
            return Err(policy_error("GATE-EXTERNAL-CHEAP-EVIDENCE", &binding.path));
        }
        let committed = git_bytes(repo, &["show", &format!("HEAD:{}", binding.path)])?;
        if sha256_bytes(&committed) != binding.sha256 {
            return Err(policy_error(
                "GATE-EXTERNAL-CHEAP-EVIDENCE-COMMIT",
                &binding.path,
            ));
        }
    }
    Ok(())
}

fn reconstruct_source_inventory(repo: &Path, plan: &ExternalDagPlan) -> Result<()> {
    #[cfg(test)]
    INVENTORY_RECONSTRUCTION_COUNT.with(|count| count.set(count.get() + 1));
    let repo =
        fs::canonicalize(repo).map_err(|error| external_error("GATE-EXTERNAL-REPO", error))?;
    let mut all_nodes = plan
        .transactions
        .iter()
        .flat_map(|transaction| transaction.light.iter().chain(&transaction.heavy))
        .chain(plan.custody_commands.iter())
        .collect::<Vec<_>>();
    all_nodes.sort_by_key(|node| node.order);
    let mut reader = csv::Reader::from_path(repo.join(&plan.source_plan.path))
        .map_err(|error| external_error("GATE-EXTERNAL-SOURCE-CSV", error))?;
    let expected_headers = csv::StringRecord::from(vec![
        "order",
        "command_id",
        "source_path",
        "argv",
        "environment",
        "working_directory",
        "inputs",
        "outputs",
        "harvard_access",
        "cost_class",
    ]);
    if reader
        .headers()
        .map_err(|error| external_error("GATE-EXTERNAL-SOURCE-CSV-HEADER", error))?
        != &expected_headers
    {
        return Err(schema_error(
            "GATE-EXTERNAL-SOURCE-CSV-HEADER",
            "executor command CSV headers must match the frozen authority exactly",
        ));
    }
    let rows = reader
        .records()
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|error| external_error("GATE-EXTERNAL-SOURCE-CSV", error))?;
    if rows.len() != all_nodes.len() {
        return Err(policy_error(
            "GATE-EXTERNAL-INVENTORY-CARDINALITY",
            "external projection differs from source command inventory",
        ));
    }
    let projected_orders = project_source_orders(&rows)?;
    for (row, node) in rows.iter().zip(&all_nodes) {
        let order_label = row.get(0).unwrap_or("");
        let projected_order = projected_orders
            .get(order_label)
            .copied()
            .ok_or_else(|| policy_error("GATE-EXTERNAL-INVENTORY-ORDER", order_label))?;
        let source_path = row.get(2).unwrap_or("");
        let command = row.get(3).unwrap_or("");
        let environment = row.get(4).unwrap_or("");
        let working_directory = row.get(5).unwrap_or("");
        let inputs = row.get(6).unwrap_or("");
        let outputs = row.get(7).unwrap_or("");
        let access = row.get(8).unwrap_or("");
        let cost = row.get(9).unwrap_or("");
        if projected_order != node.order
            || row.get(1) != Some(node.command_id.as_str())
            || canonical_source_path(&repo, source_path).as_deref()
                != Some(node.source_path.as_str())
            || !command_projection_matches(&repo, command, node)?
            || !environment_projection_matches(command, environment, node)?
            || working_directory != node.source_working_directory
            || canonical_source_working_directory(&repo, working_directory)
                != canonical_plan_working_directory(&repo, &node.cwd)
            || split_source_inventory(inputs) != node.source_inputs
            || !output_projection_matches(outputs, node)
            || matches!(
                access,
                "FORBIDDEN" | "EXPECTED_IDENTITIES_ONLY" | "OPENED_RESULTS_ONLY"
            ) != (node.harvard_access == "NONE")
            || cost != node.cost_class
        {
            return Err(policy_error(
                "GATE-EXTERNAL-INVENTORY-RECONSTRUCTION",
                &node.command_id,
            ));
        }
    }
    verify_contract_inventory(&repo, plan, &all_nodes)
}

#[cfg(test)]
thread_local! {
    static INVENTORY_RECONSTRUCTION_COUNT: std::cell::Cell<usize> = const {
        std::cell::Cell::new(0)
    };
}

#[cfg(test)]
fn reset_inventory_reconstruction_count() {
    INVENTORY_RECONSTRUCTION_COUNT.with(|count| count.set(0));
}

#[cfg(test)]
fn inventory_reconstruction_count() -> usize {
    INVENTORY_RECONSTRUCTION_COUNT.with(std::cell::Cell::get)
}

fn canonical_source_working_directory(repo: &Path, working_directory: &str) -> String {
    let normalized_repo = fs::canonicalize(repo).unwrap_or_else(|_| repo.to_path_buf());
    if Path::new(working_directory) == normalized_repo {
        "${REPO}".to_owned()
    } else {
        working_directory.to_owned()
    }
}

fn canonical_plan_working_directory(repo: &Path, working_directory: &str) -> String {
    let normalized_repo = fs::canonicalize(repo).unwrap_or_else(|_| repo.to_path_buf());
    if Path::new(working_directory) == normalized_repo {
        "${REPO}".to_owned()
    } else {
        working_directory.to_owned()
    }
}

fn split_source_inventory(inputs: &str) -> Vec<String> {
    inputs
        .split(';')
        .map(str::trim)
        .filter(|input| !input.is_empty())
        .map(str::to_owned)
        .collect()
}

fn canonical_source_path(repo: &Path, source_path: &str) -> Option<String> {
    let path = Path::new(source_path);
    if path.is_absolute() {
        let normalized_repo = fs::canonicalize(repo).unwrap_or_else(|_| repo.to_path_buf());
        path.strip_prefix(normalized_repo)
            .ok()
            .and_then(Path::to_str)
            .map(str::to_owned)
    } else {
        confined_relative(source_path)
            .ok()
            .map(|()| source_path.to_owned())
    }
}

fn project_source_orders(rows: &[csv::StringRecord]) -> Result<BTreeMap<String, u64>> {
    let mut parsed = rows
        .iter()
        .map(|row| {
            let label = row.get(0).unwrap_or("");
            let split = label
                .find(|character: char| !character.is_ascii_digit())
                .unwrap_or(label.len());
            let (number, suffix) = label.split_at(split);
            if number.is_empty()
                || number.starts_with('0')
                || !suffix
                    .chars()
                    .all(|character| character.is_ascii_lowercase())
            {
                return Err(policy_error("GATE-EXTERNAL-INVENTORY-ORDER", label));
            }
            let number = number
                .parse::<u64>()
                .map_err(|error| external_error("GATE-EXTERNAL-INVENTORY-ORDER", error))?;
            Ok(((number, suffix.to_owned()), label.to_owned()))
        })
        .collect::<Result<Vec<_>>>()?;
    parsed.sort_by(|left, right| left.0.cmp(&right.0));
    if parsed
        .windows(2)
        .any(|pair| pair[0].0 == pair[1].0 || pair[0].1 == pair[1].1)
    {
        return Err(policy_error(
            "GATE-EXTERNAL-INVENTORY-ORDER",
            "source command order labels must be unique",
        ));
    }
    parsed
        .into_iter()
        .enumerate()
        .map(|(index, (_, label))| {
            u64::try_from(index + 1)
                .map(|order| (label, order))
                .map_err(|error| external_error("GATE-EXTERNAL-INVENTORY-ORDER", error))
        })
        .collect()
}

fn command_projection_matches(repo: &Path, command: &str, node: &ExternalNode) -> Result<bool> {
    let source = shell_words::split(command)
        .map_err(|error| external_error("GATE-EXTERNAL-SOURCE-ARGV", error))?;
    let source_argv = source
        .iter()
        .filter(|token| !token.contains('='))
        .collect::<Vec<_>>();
    let expected = source_argv
        .iter()
        .enumerate()
        .map(|(index, token)| {
            canonical_source_operand(repo, token, index.checked_sub(1).map(|i| source_argv[i]))
        })
        .collect::<Vec<_>>();
    let actual = strip_admitted_injections(&node.argv)
        .iter()
        .map(|token| canonical_plan_operand(token))
        .collect::<Vec<_>>();
    Ok(actual == expected)
}

fn strip_admitted_injections(argv: &[String]) -> Vec<&str> {
    let mut retained = Vec::new();
    let mut index = 0;
    while index < argv.len() {
        if matches!(
            argv[index].as_str(),
            "--execution-root"
                | "--custody-root"
                | "--opening-token"
                | "--component-out"
                | "--object-root"
        ) {
            index += 2;
        } else {
            retained.push(argv[index].as_str());
            index += 1;
        }
    }
    retained
}

fn canonical_source_operand(repo: &Path, token: &str, previous: Option<&String>) -> String {
    if let Some(relative) = token.strip_prefix("/home/workdir/cal04b-objects/") {
        return format!("${{OBJECTS_ROOT}}/{relative}");
    }
    let repo_prefix = format!("{}/", repo.display());
    let relative = token
        .strip_prefix(&repo_prefix)
        .unwrap_or(token)
        .trim_start_matches("./");
    if let Some(binary) = relative.strip_prefix(
        "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/executor/target/",
    ) {
        return format!("${{CARGO_TARGET_DIR}}/{binary}");
    }
    if matches!(
        previous.map(String::as_str),
        Some("--configs" | "--authority-manifest" | "--failures" | "--out" | "--accepted")
    ) && relative.starts_with(
        "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/",
    ) {
        return format!("${{PUBLICATION_ROOT}}/{relative}");
    }
    relative.to_owned()
}

fn canonical_plan_operand(token: &str) -> String {
    token
        .strip_prefix("${REPO}/")
        .unwrap_or(token)
        .trim_start_matches("./")
        .to_owned()
}

fn environment_projection_matches(
    command: &str,
    environment: &str,
    node: &ExternalNode,
) -> Result<bool> {
    let assignments = shell_words::split(command)
        .map_err(|error| external_error("GATE-EXTERNAL-SOURCE-ENV", error))?
        .into_iter()
        .take_while(|token| token.contains('='))
        .filter_map(|token| {
            token
                .split_once('=')
                .map(|(name, value)| (name.to_owned(), value.to_owned()))
        })
        .collect::<BTreeMap<_, _>>();
    if !assignments.is_empty() {
        return Ok(node.env == assignments);
    }
    if matches!(environment, "default" | "none") {
        return Ok(node.env.is_empty()
            || environment == "default"
                && node.command_id.starts_with("build_")
                && node.env
                    == BTreeMap::from([(
                        "CARGO_TARGET_DIR".to_owned(),
                        "${CARGO_TARGET_DIR}".to_owned(),
                    )]));
    }
    let declared = environment
        .split_whitespace()
        .map(|item| {
            item.split_once('=')
                .map(|(name, value)| (name.to_owned(), value.to_owned()))
                .ok_or_else(|| policy_error("GATE-EXTERNAL-SOURCE-ENV", item))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    Ok(node.env == declared)
}

fn output_projection_matches(outputs: &str, node: &ExternalNode) -> bool {
    if node.command_id.starts_with("build_")
        && node
            .declared_outputs
            .iter()
            .any(|path| path == "cargo-target")
    {
        return true;
    }
    let expected = outputs
        .split(';')
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .map(canonical_contract_output)
        .collect::<Vec<_>>();
    expected.iter().all(|path| {
        node.declared_outputs.iter().any(|declaration| {
            path == declaration
                || path.starts_with(&format!("{declaration}/"))
                || declaration.ends_with(&format!("/{path}"))
        })
    })
}

fn verify_contract_inventory(
    repo: &Path,
    plan: &ExternalDagPlan,
    nodes: &[&ExternalNode],
) -> Result<()> {
    let mut reader = csv::Reader::from_path(repo.join(&plan.source_contract.path))
        .map_err(|error| external_error("GATE-EXTERNAL-CONTRACT-CSV", error))?;
    let expected_headers =
        csv::StringRecord::from(vec!["command_id", "prerequisites", "receipt_outputs"]);
    if reader
        .headers()
        .map_err(|error| external_error("GATE-EXTERNAL-CONTRACT-CSV-HEADER", error))?
        != &expected_headers
    {
        return Err(schema_error(
            "GATE-EXTERNAL-CONTRACT-CSV-HEADER",
            "observed command contract headers must match the frozen authority exactly",
        ));
    }
    let rows = reader
        .records()
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|error| external_error("GATE-EXTERNAL-CONTRACT-CSV", error))?;
    if rows.len() != nodes.len() {
        return Err(policy_error(
            "GATE-EXTERNAL-CONTRACT-CARDINALITY",
            "observed command contract cardinality drifted",
        ));
    }
    let by_id = nodes
        .iter()
        .map(|node| (node.command_id.as_str(), *node))
        .collect::<BTreeMap<_, _>>();
    for row in rows {
        let id = row.get(0).unwrap_or("");
        let node = by_id
            .get(id)
            .ok_or_else(|| policy_error("GATE-EXTERNAL-CONTRACT-NODE", id))?;
        let expected = row.get(1).unwrap_or("-");
        let prerequisites = if expected == "-" {
            Vec::new()
        } else {
            expected.split(';').collect::<Vec<_>>()
        };
        if prerequisites
            != node
                .prerequisites
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
        {
            return Err(policy_error("GATE-EXTERNAL-CONTRACT-PREREQUISITES", id));
        }
        let output_cell = row.get(2).unwrap_or("");
        let expected_outputs = if output_cell == "-" {
            Vec::new()
        } else {
            output_cell
                .split(';')
                .filter(|path| !path.is_empty())
                .map(canonical_contract_output)
                .collect::<Vec<_>>()
        };
        if !declared_exactly_covers(&expected_outputs, &node.declared_outputs) {
            return Err(policy_error("GATE-EXTERNAL-CONTRACT-OUTPUTS", id));
        }
    }
    Ok(())
}

fn declared_exactly_covers(expected: &[String], declared: &[String]) -> bool {
    expected.iter().all(|path| {
        declared
            .iter()
            .any(|declaration| path == declaration || path.starts_with(&format!("{declaration}/")))
    }) && declared.iter().all(|declaration| {
        expected
            .iter()
            .any(|path| path == declaration || path.starts_with(&format!("{declaration}/")))
    })
}

fn canonical_contract_output(path: &str) -> String {
    let path = path.trim();
    if path == "every exact trace/stdout/stderr path in native-proof-case-plan.csv" {
        return "objects/native-proof".to_owned();
    }
    if let Some(relative) = path.strip_prefix("/home/workdir/cal04b-objects/") {
        return format!("objects/{relative}");
    }
    if let Some(relative) = path.strip_prefix(
        "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/executor/target/",
    ) {
        return format!("cargo-target/{relative}");
    }
    if let Some(relative) = path.strip_prefix("/home/workdir/openWEPP/target/") {
        return format!("cargo-target/{relative}");
    }
    if let Some(relative) = path.strip_prefix("/home/workdir/openWEPP/docs/") {
        return format!("publication/docs/{relative}");
    }
    if let Some(relative) = path.strip_prefix("artifacts/") {
        return format!(
            "publication/docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/{relative}"
        );
    }
    if path.starts_with("docs/") {
        return format!("publication/{path}");
    }
    path.to_owned()
}

fn verify_light_receipts(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
    receipts: &[ExternalNodeReceipt],
) -> Result<()> {
    if receipts.len() != transaction.light.len() {
        return Err(policy_error(
            "GATE-EXTERNAL-LIGHT-CARDINALITY",
            &transaction.transaction_id,
        ));
    }
    for (node, receipt) in transaction.light.iter().zip(receipts) {
        if receipt.command_id != node.command_id
            || receipt.order != node.order
            || receipt.stage != "LIGHT"
            || receipt.source_sha256 != file_sha256(&options.repo.join(&node.source_path))?
            || receipt.repo_identity != repository_identity(&options.repo)?
        {
            return Err(policy_error(
                "GATE-EXTERNAL-LIGHT-RECEIPT",
                &node.command_id,
            ));
        }
        verify_historical_manifest(&options.attempt_root, &receipt.output_manifest)?;
    }
    Ok(())
}

fn collect_node_identities(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
) -> Result<Vec<Value>> {
    transaction
        .light
        .iter()
        .chain(&transaction.heavy)
        .map(|node| {
            let argv = node
                .argv
                .iter()
                .map(|argument| expand_operand(options, argument))
                .collect::<Result<Vec<_>>>()?;
            let environment = admitted_environment(options, node)?;
            let executable = resolve_executable(&argv[0], &environment)?;
            let executable_sha256 = executable_sha256(&executable)?;
            let source_sha256 = file_sha256(&options.repo.join(&node.source_path))?;
            if Path::new(&node.source_path)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("py"))
                && !argv
                    .iter()
                    .any(|argument| argument.ends_with(&node.source_path))
            {
                return Err(policy_error("GATE-EXTERNAL-SOURCE-ARGV", &node.command_id));
            }
            Ok(json!({
                "command_id": node.command_id,
                "source_path": node.source_path,
                "source_sha256": source_sha256,
                "executable_path": executable,
                "executable_sha256": executable_sha256,
                "executable_version": executable_version(&executable, &executable_sha256)?,
                "argv": argv,
                "environment": environment,
            }))
        })
        .collect()
}

fn verify_root_separation(options: &ExternalTransitionOptions) -> Result<()> {
    let repo = fs::canonicalize(&options.repo)
        .map_err(|error| external_error("GATE-EXTERNAL-REPO", error))?;
    let attempt = fs::canonicalize(&options.attempt_root)
        .map_err(|error| external_error("GATE-EXTERNAL-ROOT", error))?;
    let ledger = fs::canonicalize(&options.ledger)
        .map_err(|error| ledger_error("GATE-EXTERNAL-LEDGER", error))?;
    if attempt.starts_with(&repo)
        || ledger.starts_with(&repo)
        || ledger.starts_with(&attempt)
        || options.receipt_path.starts_with(&repo)
        || options.receipt_path.starts_with(&attempt)
    {
        return Err(policy_error(
            "GATE-EXTERNAL-ROOT-ALIAS",
            "repository, attempt, ledger, and receipt roots must be separated",
        ));
    }
    Ok(())
}

fn verify_ledger_state_without_admission(path: &Path) -> Result<()> {
    let text =
        fs::read_to_string(path).map_err(|error| ledger_error("GATE-EXTERNAL-LEDGER", error))?;
    let mut defect_statuses = BTreeMap::new();
    for line in text.lines().filter(|line| !line.is_empty()) {
        let value = parse_strict(line.as_bytes())?;
        if value["record_type"] == "TOOLING_DEFECT" {
            let defect_id = value["defect_id"]
                .as_str()
                .ok_or_else(|| policy_error("GATE-EXTERNAL-TOOLING-DEFECT-SHAPE", "defect_id"))?;
            let status = value["status"]
                .as_str()
                .ok_or_else(|| policy_error("GATE-EXTERNAL-TOOLING-DEFECT-SHAPE", "status"))?;
            if !matches!(status, "OPEN" | "CLOSED") {
                return Err(policy_error("GATE-EXTERNAL-TOOLING-DEFECT-SHAPE", status));
            }
            defect_statuses.insert(defect_id.to_owned(), status.to_owned());
        }
    }
    for (defect_id, status) in defect_statuses {
        if status == "OPEN" {
            return Err(policy_error("GATE-EXTERNAL-TOOLING-DEFECT", &defect_id));
        }
    }
    Ok(())
}

fn validate_ready(
    options: &ExternalTransitionOptions,
    plan: &ExternalDagPlan,
    transaction: &ExternalTransaction,
    audit: &ConstructedAudit,
    light: &[ExternalNodeReceipt],
    source_before: &RepositoryIdentity,
    started_entry_sha256: &str,
) -> Result<()> {
    let value = audit.as_value();
    if value["plan_id"] != plan.plan_id
        || value["transaction_id"] != transaction.transaction_id
        || value["attempt_root"] != options.attempt_root.display().to_string()
        || value["claims"] != claims_value(&options.claims)
        || value["source_identity"] != json!(source_before)
        || value["light_receipts"]
            != serde_json::to_value(light)
                .map_err(|error| external_error("GATE-EXTERNAL-AUDIT-SERIALIZE", error))?
    {
        return Err(policy_error(
            "GATE-EXTERNAL-AUDIT-CONTEXT",
            "READY context changed before HEAVY admission",
        ));
    }
    verify_consumed_custody_proof(options, transaction, &value["consumed_custody_proof"])?;
    let bytes =
        fs::read(&options.ledger).map_err(|error| ledger_error("GATE-EXTERNAL-LEDGER", error))?;
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
        || !started["audit_id"].is_null()
        || value["started_entry_sha256"] != started_entry_sha256
        || started["admitted_ledger_head_sha256"] != value["ledger_head_sha256"]
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
