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
            "../../../../gate-policy/v1/schemas/external-verifier-attestation.schema.json"
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
        attestations.push((relative.clone(), attestation));
    }
    if !attestations.is_empty() {
        verify_independent_attestations(
            &attestations
                .iter()
                .map(|(_, attestation)| attestation.clone())
                .collect::<Vec<_>>(),
        )?;
        authenticate_verifier_custody(options, &canonical_root, &attestations)?;
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

fn authenticate_verifier_custody(
    options: &ExternalTransitionOptions,
    custody_root: &Path,
    attestations: &[(String, ExternalVerifierAttestation)],
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
    for (attestation_path, attestation) in attestations {
        let verifier_id = verifier_id_from_attestation_path(attestation_path)?;
        let capability = custody_root
            .join("capabilities")
            .join(format!("{}.cap", attestation.capability_hash));
        let consumed = consumed_root.join(format!("{}.cap", attestation.capability_hash));
        let metadata = fs::symlink_metadata(&capability)
            .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-MISSING", error))?;
        if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.nlink() != 1 {
            return Err(policy_error(
                "GATE-EXTERNAL-CAPABILITY-TYPE",
                &capability.display().to_string(),
            ));
        }
        let preimage = fs::read(&capability)
            .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-READ", error))?;
        if sha256_bytes(&preimage) != attestation.capability_hash {
            return Err(policy_error(
                "GATE-EXTERNAL-CAPABILITY-HASH",
                &capability.display().to_string(),
            ));
        }
        authenticate_verifier_receipt(options, custody_root, verifier_id, attestation)?;
        rustix::fs::renameat_with(
            rustix::fs::CWD,
            &capability,
            rustix::fs::CWD,
            &consumed,
            rustix::fs::RenameFlags::NOREPLACE,
        )
        .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-CONSUME", error))?;
        FileSync::sync_parent(&consumed)?;
    }
    Ok(())
}

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

fn verifier_id_from_attestation_path(path: &str) -> Result<&'static str> {
    match Path::new(path).file_name().and_then(|name| name.to_str()) {
        Some("freeze_verify_a.json") => Ok("verifier_a"),
        Some("freeze_verify_b.json") => Ok("verifier_b"),
        _ => Err(policy_error("GATE-EXTERNAL-CUSTODY-PATH", path)),
    }
}

fn authenticate_verifier_receipt(
    options: &ExternalTransitionOptions,
    custody_root: &Path,
    verifier_id: &str,
    attestation: &ExternalVerifierAttestation,
) -> Result<()> {
    let receipt = custody_root
        .join("freeze-receipts")
        .join(format!("{verifier_id}.csv"));
    let metadata = fs::symlink_metadata(&receipt)
        .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(policy_error(
            "GATE-EXTERNAL-CUSTODY-PATH",
            &receipt.display().to_string(),
        ));
    }
    let receipt_bytes =
        fs::read(&receipt).map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?;
    if sha256_bytes(&receipt_bytes) != attestation.receipt_sha256 {
        return Err(policy_error(
            "GATE-EXTERNAL-CUSTODY-DIGEST",
            &receipt.display().to_string(),
        ));
    }
    let script = options.repo.join(
        "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py",
    );
    if sha256_bytes(
        &fs::read(&script).map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?,
    ) != attestation.script_sha256
    {
        return Err(policy_error(
            "GATE-EXTERNAL-CUSTODY-SCRIPT",
            &script.display().to_string(),
        ));
    }
    let verifier_root =
        authenticate_attestation_argv(custody_root, verifier_id, &script, attestation)?;
    authenticate_receipt_row(
        options,
        verifier_id,
        &script,
        &verifier_root,
        attestation,
        &receipt_bytes,
    )
}

fn authenticate_attestation_argv(
    custody_root: &Path,
    verifier_id: &str,
    script: &Path,
    attestation: &ExternalVerifierAttestation,
) -> Result<PathBuf> {
    if attestation.argv.first().map(String::as_str) != script.to_str() {
        return Err(policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id));
    }
    let operands = attestation.argv.get(1..).ok_or_else(|| {
        policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id)
    })?;
    if operands.len() % 2 != 0 {
        return Err(policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id));
    }
    let mut supplied = BTreeMap::new();
    for pair in operands.chunks_exact(2) {
        if supplied.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
            return Err(policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id));
        }
    }
    let execution_root = PathBuf::from(
        supplied
            .get("--execution-root")
            .ok_or_else(|| policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id))?,
    );
    if !execution_root.is_absolute()
        || execution_root
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id));
    }
    let verifier_suffix = verifier_id.strip_prefix("verifier_").ok_or_else(|| {
        policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id)
    })?;
    let expected = BTreeMap::from([
        ("--execution-root", execution_root.display().to_string()),
        ("--verifier-id", verifier_id.to_owned()),
        ("--custody-root", custody_root.display().to_string()),
        (
            "--capability",
            custody_root
                .join("capabilities")
                .join(format!("{}.cap", attestation.capability_hash))
                .display()
                .to_string(),
        ),
        (
            "--attestation-out",
            custody_root
                .join(format!("freeze_verify_{verifier_suffix}.json"))
                .display()
                .to_string(),
        ),
        (
            "--parent-dispatch-id",
            attestation.parent_dispatch_id.clone(),
        ),
        ("--agent-task-id", attestation.agent_task_id.clone()),
        ("--principal", attestation.principal.clone()),
        ("--workflow", attestation.workflow.clone()),
        ("--job", attestation.job.clone()),
        ("--runner", attestation.runner.clone()),
        ("--attempt", attestation.attempt.to_string()),
    ]);
    if supplied.len() != expected.len() {
        return Err(policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id));
    }
    for (flag, value) in expected {
        if supplied.get(flag).copied() != Some(value.as_str()) {
            return Err(policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id));
        }
    }
    Ok(execution_root)
}

fn authenticate_receipt_row(
    options: &ExternalTransitionOptions,
    verifier_id: &str,
    script: &Path,
    verifier_root: &Path,
    attestation: &ExternalVerifierAttestation,
    bytes: &[u8],
) -> Result<()> {
    const HEADER: &str =
        "verifier_id,freeze_digest,verifier_script_sha256,command,command_sha256,timestamp,state";
    let text = std::str::from_utf8(bytes)
        .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-RECEIPT", error))?;
    let lines = text.lines().collect::<Vec<_>>();
    if lines.len() != 2 || lines[0] != HEADER {
        return Err(policy_error("GATE-EXTERNAL-CUSTODY-RECEIPT", verifier_id));
    }
    let fields = lines[1].split(',').collect::<Vec<_>>();
    let script_relative = script.strip_prefix(&options.repo).map_err(|_| {
        policy_error(
            "GATE-EXTERNAL-CUSTODY-SCRIPT",
            &script.display().to_string(),
        )
    })?;
    let expected_command = format!(
        "PYTHONDONTWRITEBYTECODE=1 .venv/bin/python {} --execution-root {} --verifier-id {verifier_id}",
        script_relative.display(),
        verifier_root.display()
    );
    if fields.len() != 7
        || fields[0] != verifier_id
        || fields[1] != attestation.freeze_digest
        || fields[2] != attestation.script_sha256
        || fields[3] != expected_command
        || fields[4] != sha256_bytes(expected_command.as_bytes())
        || fields[6] != "PASS"
    {
        return Err(policy_error("GATE-EXTERNAL-CUSTODY-RECEIPT", verifier_id));
    }
    Ok(())
}
