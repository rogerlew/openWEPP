fn construct_ready_audit(
    options: &ExternalTransitionOptions,
    plan: &ExternalDagPlan,
    transaction: &ExternalTransaction,
    light: &[ExternalNodeReceipt],
    source_before: &RepositoryIdentity,
    attempt_identity: &str,
) -> Result<ConstructedAudit> {
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
    verify_cheap_evidence(&options.repo, &plan.authority.cheap_gate_evidence)?;
    reconstruct_source_inventory(&options.repo, plan)?;
    verify_light_receipts(options, transaction, light)?;
    let node_identities = collect_node_identities(options, transaction)?;
    verify_root_separation(options)?;
    verify_ledger_admission(&options.ledger)?;
    let checks = [
        ("package_authority", package_audit["package_audit_id"].clone()),
        ("source_identity", json!(source_before)),
        ("plan_identity", json!(plan.plan_id)),
        ("light_receipts", json!(light.len())),
        ("inventory_order", json!(transaction.light.len() + transaction.heavy.len())),
        ("toolchain_environment", json!(node_identities)),
        ("fresh_external_root", json!(options.attempt_root)),
        ("root_separation", json!("VERIFIED")),
        ("custody_prerequisites", json!(transaction.custody_prerequisites)),
        ("durable_ledger", ledger_head(&options.ledger)?),
    ]
    .map(|(check_id, evidence)| {
        json!({"check_id": check_id, "result": "PASS", "evidence": evidence})
    });
    let mut audit = json!({
        "schema": EXTERNAL_AUDIT_SCHEMA,
        "audit_id": "",
        "status": "READY",
        "plan_id": plan.plan_id,
        "transaction_id": transaction.transaction_id,
        "attempt_root": options.attempt_root.display().to_string(),
        "attempt_root_identity": attempt_identity,
        "ledger": options.ledger.display().to_string(),
        "ledger_head_sha256": ledger_head(&options.ledger)?,
        "claims": claims_value(&options.claims),
        "source_identity": source_before,
        "package_audit": package_audit,
        "node_identities": node_identities,
        "light_receipts": light,
        "checks": checks,
    });
    let audit_id = derived_id(&audit, "audit_id")?;
    audit["audit_id"] = Value::String(audit_id);
    ConstructedAudit::from_external(audit)
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
    let mut all_nodes = plan
        .transactions
        .iter()
        .flat_map(|transaction| transaction.light.iter().chain(&transaction.heavy))
        .chain(plan.custody_commands.iter())
        .collect::<Vec<_>>();
    all_nodes.sort_by_key(|node| node.order);
    let mut reader = csv::Reader::from_path(repo.join(&plan.source_plan.path))
        .map_err(|error| external_error("GATE-EXTERNAL-SOURCE-CSV", error))?;
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
    let order_labels = rows
        .iter()
        .filter_map(|row| row.get(0))
        .collect::<BTreeSet<_>>();
    if order_labels.len() != rows.len() || order_labels.contains("") {
        return Err(policy_error(
            "GATE-EXTERNAL-INVENTORY-ORDER",
            "source command order labels must be nonempty and unique",
        ));
    }
    for (index, (row, node)) in rows.iter().zip(&all_nodes).enumerate() {
        let projected_order = u64::try_from(index + 1)
            .map_err(|error| external_error("GATE-EXTERNAL-INVENTORY-ORDER", error))?;
        let source_path = row.get(2).unwrap_or("");
        let command = row.get(3).unwrap_or("");
        let environment = row.get(4).unwrap_or("");
        let outputs = row.get(7).unwrap_or("");
        let access = row.get(8).unwrap_or("");
        let cost = row.get(9).unwrap_or("");
        if projected_order != node.order
            || row.get(1) != Some(node.command_id.as_str())
            || !source_path.ends_with(&node.source_path)
            || !command_projection_matches(repo, command, node)?
            || !environment_projection_matches(command, environment, node)?
            || !output_projection_matches(outputs, node)
            || matches!(
                access,
                "FORBIDDEN" | "EXPECTED_IDENTITIES_ONLY" | "OPENED_RESULTS_ONLY"
            )
                != (node.harvard_access == "NONE")
            || cost != node.cost_class
        {
            return Err(policy_error(
                "GATE-EXTERNAL-INVENTORY-RECONSTRUCTION",
                &node.command_id,
            ));
        }
    }
    verify_contract_inventory(repo, plan, &all_nodes)
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
    Ok(environment == "default"
        && (node.env.is_empty()
            || node.command_id.starts_with("build_")
                && node.env
                    == BTreeMap::from([(
                        "CARGO_TARGET_DIR".to_owned(),
                        "${CARGO_TARGET_DIR}".to_owned(),
                    )]))
        || node.env.iter().all(|(name, value)| {
            environment
                .split_whitespace()
                .any(|item| item == format!("{name}={value}"))
        }))
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
        node.declared_outputs
            .iter()
            .any(|declaration| {
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
            let executable_sha256 = file_sha256(&executable)?;
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
        .map_err(|error| external_error("GATE-EXTERNAL-LEDGER", error))?;
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

fn verify_ledger_admission(path: &Path) -> Result<()> {
    let text =
        fs::read_to_string(path).map_err(|error| external_error("GATE-EXTERNAL-LEDGER", error))?;
    for line in text.lines().filter(|line| !line.is_empty()) {
        let value = parse_strict(line.as_bytes())?;
        if value["record_type"] == "TOOLING_DEFECT" && value["status"] == "OPEN" {
            return Err(policy_error(
                "GATE-EXTERNAL-TOOLING-DEFECT",
                value["defect_id"].as_str().unwrap_or("unknown"),
            ));
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
