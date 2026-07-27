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
pub(super) fn verify_custody_files(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
    _consume_capabilities: bool,
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
            || attestation.transaction_id != transaction.transaction_id
        {
            return Err(custody_error(
                "GATE-EXTERNAL-ATTESTATION-FRESHNESS",
                relative,
            ));
        }
        let command_id = Path::new(relative)
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| policy_error("GATE-EXTERNAL-CUSTODY-PATH", relative))?;
        imported.insert(command_id.to_owned(), attestation.attestation_id.clone());
        attestations.push((relative.clone(), attestation));
    }
    let mut transaction_receipt = None;
    let mut freeze_receipt = None;
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
        match binding.kind.as_str() {
            "TRANSACTION" => transaction_receipt = Some((binding, value.clone())),
            "FREEZE" => freeze_receipt = Some((binding, value.clone())),
            _ => unreachable!("custody kind checked above"),
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
    if opens_harvard {
        authenticate_cross_transition_custody(
            options,
            &attestations,
            transaction_receipt.ok_or_else(|| {
                policy_error(
                    "GATE-EXTERNAL-HARVARD-CUSTODY-INCOMPLETE",
                    "transaction receipt missing",
                )
            })?,
            freeze_receipt.ok_or_else(|| {
                policy_error(
                    "GATE-EXTERNAL-HARVARD-CUSTODY-INCOMPLETE",
                    "freeze receipt missing",
                )
            })?,
        )?;
    }
    if !attestations.is_empty() {
        verify_attestation_freshness(
            &attestations
                .iter()
                .map(|(_, attestation)| attestation.clone())
                .collect::<Vec<_>>(),
        )?;
        verify_independent_attestations(
            &attestations
                .iter()
                .map(|(_, attestation)| attestation.clone())
                .collect::<Vec<_>>(),
        )?;
        authenticate_verifier_custody(options, &canonical_root, &attestations)?;
    }
    Ok(imported)
}

fn verify_attestation_freshness(attestations: &[ExternalVerifierAttestation]) -> Result<()> {
    let now = time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|error| custody_error("GATE-EXTERNAL-ATTESTATION-FRESHNESS", error))?;
    let today = now.get(..10).ok_or_else(|| {
        custody_error(
            "GATE-EXTERNAL-ATTESTATION-FRESHNESS",
            "current UTC date",
        )
    })?;
    let current_utc_day = attestations.iter().all(|item| {
        item.created_at.len() >= 20
            && item.created_at.get(..10) == Some(today)
            && item.created_at.as_bytes().get(10) == Some(&b'T')
            && (item.created_at.ends_with('Z') || item.created_at.ends_with("+00:00"))
    });
    if current_utc_day {
        Ok(())
    } else {
        Err(custody_error(
            "GATE-EXTERNAL-ATTESTATION-FRESHNESS",
            "attestations must be created on the current UTC dispatch day",
        ))
    }
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
    for (attestation_path, attestation) in attestations {
        let verifier_id = verifier_id_from_attestation_path(attestation_path)?;
        let capability = custody_root
            .join("capabilities")
            .join(format!("{}.cap", attestation.capability_hash));
        let metadata = fs::symlink_metadata(&capability)
            .map_err(|error| custody_error("GATE-EXTERNAL-CAPABILITY-MISSING", error))?;
        if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.nlink() != 1 {
            return Err(custody_error(
                "GATE-EXTERNAL-CAPABILITY-TYPE",
                capability.display(),
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
    }
    Ok(())
}

/// Consume an already authenticated transaction's capabilities exactly once.
///
/// This is the only custody operation which mutates either capability directory.
/// The returned proof is suitable for embedding in the READY audit and for
/// mutation-free verification by HEAVY execution and independent verification.
#[allow(
    clippy::too_many_lines,
    reason = "the atomic dispatch-scoped custody transition keeps preflight, rename, and immutable proof construction in one auditable sequence"
)]
pub(super) fn consume_custody_capabilities(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
) -> Result<Value> {
    let imported = verify_custody_files(options, transaction, false)?;
    if transaction.custody_prerequisites.is_empty() {
        return Ok(Value::Null);
    }
    let custody_root = options.custody_root.as_ref().ok_or_else(|| {
        policy_error(
            "GATE-EXTERNAL-CUSTODY-ROOT-REQUIRED",
            &transaction.transaction_id,
        )
    })?;
    let custody_root = fs::canonicalize(custody_root)
        .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-ROOT", error))?;
    let attestations = read_transaction_attestations(&custody_root, transaction)?;
    let dispatch_id = attestations
        .first()
        .map(|item| item.parent_dispatch_id.clone())
        .ok_or_else(|| {
            policy_error(
                "GATE-EXTERNAL-CUSTODY-CARDINALITY",
                &transaction.transaction_id,
            )
        })?;
    let consumed_parent = custody_root.join("consumed-capabilities");
    match fs::create_dir(&consumed_parent) {
        Ok(()) => FileSync::sync_parent(&consumed_parent)?,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(external_error("GATE-EXTERNAL-CAPABILITY-DIR", error)),
    }
    require_capability_directory(&consumed_parent)?;
    let scope_id = sha256_bytes(
        format!("{dispatch_id}\0{}", transaction.transaction_id).as_bytes(),
    );
    let consumed_root = consumed_parent.join(scope_id);

    for (relative, attestation) in transaction.custody_prerequisites.iter().zip(&attestations) {
        let command_id = Path::new(relative)
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| policy_error("GATE-EXTERNAL-CUSTODY-PATH", relative))?;
        if imported.get(command_id) != Some(&attestation.attestation_id)
            || attestation.transaction_id != transaction.transaction_id
        {
            return Err(policy_error("GATE-EXTERNAL-ATTESTATION-CHANGED", relative));
        }
    }
    verify_independent_attestations(&attestations)?;
    if consumed_root
        .try_exists()
        .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-CONSUME-PREFLIGHT", error))?
    {
        return Err(policy_error(
            "GATE-EXTERNAL-CAPABILITY-ALREADY-CONSUMED",
            &dispatch_id,
        ));
    }
    let active_root = custody_root.join("capabilities");
    let expected_names = attestations
        .iter()
        .map(|attestation| format!("{}.cap", attestation.capability_hash))
        .collect::<BTreeSet<_>>();
    let active_names = fs::read_dir(&active_root)
        .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-DIR", error))?
        .map(|entry| {
            entry
                .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-DIR", error))
                .and_then(|entry| {
                    entry.file_name().into_string().map_err(|_| {
                        policy_error("GATE-EXTERNAL-CAPABILITY-PATH", "non-UTF-8 entry")
                    })
                })
        })
        .collect::<Result<BTreeSet<_>>>()?;
    if active_names != expected_names {
        return Err(policy_error(
            "GATE-EXTERNAL-CAPABILITY-INVENTORY",
            &transaction.transaction_id,
        ));
    }
    rustix::fs::renameat_with(
        rustix::fs::CWD,
        &active_root,
        rustix::fs::CWD,
        &consumed_root,
        rustix::fs::RenameFlags::NOREPLACE,
    )
    .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-CONSUME", error))?;
    FileSync::sync_parent(&consumed_root)?;
    let entries = attestations
        .iter()
        .map(|attestation| {
            json!({
                "name": format!("{}.cap", attestation.capability_hash),
                "capability_hash": attestation.capability_hash,
                "attestation_id": attestation.attestation_id,
            })
        })
        .collect::<Vec<_>>();
    let mut proof = json!({
        "schema": "openwepp-consumed-capability-root-proof-v1",
        "proof_id": "",
        "transaction_id": transaction.transaction_id,
        "parent_dispatch_id": dispatch_id,
        "custody_root": custody_root.display().to_string(),
        "consumed_root": consumed_root.display().to_string(),
        "consumed_root_identity": directory_identity(&consumed_root)?,
        "entries": entries,
        "imported_receipts": imported,
    });
    proof["proof_id"] = Value::String(derived_id(&proof, "proof_id")?);
    verify_consumed_custody_proof(options, transaction, &proof)?;
    Ok(proof)
}

/// Verify the audit's immutable consumed-root proof without changing custody.
#[allow(
    clippy::too_many_lines,
    reason = "proof verification keeps transaction, dispatch, root, inventory, and receipt bindings in one mutation-free check"
)]
pub(super) fn verify_consumed_custody_proof(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
    proof: &Value,
) -> Result<BTreeMap<String, String>> {
    if transaction.custody_prerequisites.is_empty() {
        return if proof.is_null() {
            Ok(BTreeMap::new())
        } else {
            Err(custody_error(
                "GATE-EXTERNAL-CONSUMED-PROOF",
                "unexpected proof for transaction without custody",
            ))
        };
    }
    if proof["schema"] != "openwepp-consumed-capability-root-proof-v1"
        || proof["transaction_id"] != transaction.transaction_id
        || derived_id(proof, "proof_id")? != proof["proof_id"]
    {
        return Err(custody_error(
            "GATE-EXTERNAL-CONSUMED-PROOF",
            &transaction.transaction_id,
        ));
    }
    let custody_root = options.custody_root.as_ref().ok_or_else(|| {
        policy_error(
            "GATE-EXTERNAL-CUSTODY-ROOT-REQUIRED",
            &transaction.transaction_id,
        )
    })?;
    let custody_root = fs::canonicalize(custody_root)
        .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-ROOT", error))?;
    let proof_dispatch_id = proof["parent_dispatch_id"]
        .as_str()
        .ok_or_else(|| {
            custody_error(
                "GATE-EXTERNAL-CONSUMED-PROOF",
                "parent dispatch identity",
            )
        })?;
    let attestations = read_transaction_attestations(&custody_root, transaction)?;
    let dispatch_id = attestations
        .first()
        .map(|item| item.parent_dispatch_id.as_str())
        .ok_or_else(|| {
            policy_error(
                "GATE-EXTERNAL-CUSTODY-CARDINALITY",
                &transaction.transaction_id,
            )
        })?;
    if proof_dispatch_id != dispatch_id {
        return Err(custody_error(
            "GATE-EXTERNAL-ATTESTATION-FRESHNESS",
            dispatch_id,
        ));
    }
    let scope_id =
        sha256_bytes(format!("{dispatch_id}\0{}", transaction.transaction_id).as_bytes());
    let consumed_root = custody_root.join("consumed-capabilities").join(scope_id);
    require_capability_directory(&consumed_root)?;
    if proof["custody_root"] != custody_root.display().to_string()
        || proof["consumed_root"] != consumed_root.display().to_string()
        || proof["consumed_root_identity"] != directory_identity(&consumed_root)?
    {
        return Err(custody_error(
            "GATE-EXTERNAL-CONSUMED-PROOF-ROOT",
            &transaction.transaction_id,
        ));
    }
    let expected = attestations
        .iter()
        .map(|attestation| {
            (
                format!("{}.cap", attestation.capability_hash),
                attestation.capability_hash.clone(),
                attestation.attestation_id.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    let observed_entries = proof["entries"]
        .as_array()
        .ok_or_else(|| custody_error("GATE-EXTERNAL-CONSUMED-PROOF", &transaction.transaction_id))?;
    let observed = observed_entries
        .iter()
        .map(|entry| {
            let name = entry["name"]
                .as_str()
                .ok_or_else(|| custody_error("GATE-EXTERNAL-CONSUMED-PROOF", "entry name"))?;
            let hash = entry["capability_hash"]
                .as_str()
                .ok_or_else(|| custody_error("GATE-EXTERNAL-CONSUMED-PROOF", "entry hash"))?;
            let attestation_id = entry["attestation_id"].as_str().ok_or_else(|| {
                custody_error("GATE-EXTERNAL-CONSUMED-PROOF", "attestation identity")
            })?;
            Ok((name.to_owned(), hash.to_owned(), attestation_id.to_owned()))
        })
        .collect::<Result<BTreeSet<_>>>()?;
    let disk_names = fs::read_dir(&consumed_root)
        .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-DIR", error))?
        .map(|entry| {
            entry
                .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-DIR", error))
                .and_then(|entry| {
                    entry.file_name().into_string().map_err(|_| {
                        custody_error("GATE-EXTERNAL-CONSUMED-PROOF", "non-UTF-8 entry")
                    })
                })
        })
        .collect::<Result<BTreeSet<_>>>()?;
    if observed != expected
        || disk_names != expected.iter().map(|(name, _, _)| name.clone()).collect()
    {
        return Err(custody_error(
            "GATE-EXTERNAL-CONSUMED-PROOF-INVENTORY",
            &transaction.transaction_id,
        ));
    }
    for (name, hash, _) in &expected {
        require_capability_file(&consumed_root.join(name), hash)?;
    }
    serde_json::from_value(proof["imported_receipts"].clone())
        .map_err(|error| custody_error("GATE-EXTERNAL-CONSUMED-PROOF", error))
}

fn read_transaction_attestations(
    custody_root: &Path,
    transaction: &ExternalTransaction,
) -> Result<Vec<ExternalVerifierAttestation>> {
    transaction
        .custody_prerequisites
        .iter()
        .map(|relative| {
            serde_json::from_value(parse_strict(
                &fs::read(custody_root.join(relative))
                    .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?,
            )?)
            .map_err(|error| external_error("GATE-EXTERNAL-ATTESTATION-SHAPE", error))
        })
        .collect()
}

fn require_capability_directory(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-DIR", error))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(policy_error(
            "GATE-EXTERNAL-CAPABILITY-DIR-TYPE",
            &path.display().to_string(),
        ));
    }
    Ok(())
}

fn require_capability_file(path: &Path, expected_hash: &str) -> Result<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| custody_error("GATE-EXTERNAL-CAPABILITY-MISSING", error))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.nlink() != 1 {
        return Err(custody_error(
            "GATE-EXTERNAL-CAPABILITY-TYPE",
            path.display(),
        ));
    }
    let bytes =
        fs::read(path).map_err(|error| external_error("GATE-EXTERNAL-CAPABILITY-READ", error))?;
    if sha256_bytes(&bytes) != expected_hash {
        return Err(policy_error(
            "GATE-EXTERNAL-CAPABILITY-HASH",
            &path.display().to_string(),
        ));
    }
    Ok(())
}

fn authenticate_cross_transition_custody(
    options: &ExternalTransitionOptions,
    attestations: &[(String, ExternalVerifierAttestation)],
    transaction: (&CustodyReceiptBinding, Value),
    freeze: (&CustodyReceiptBinding, Value),
) -> Result<()> {
    let (transaction_binding, transaction_value) = transaction;
    let (freeze_binding, freeze_value) = freeze;
    if transaction_value["schema"] != EXTERNAL_RECEIPT_SCHEMA
        || transaction_value["transaction_id"] != "calibration-v1"
        || derived_id(&transaction_value, "receipt_id")? != transaction_value["receipt_id"]
        || freeze_value["schema"] != "cal04b-freeze-receipt-v1"
        || derived_id(&freeze_value, "freeze_receipt_id")? != freeze_value["freeze_receipt_id"]
        || freeze_value["calibration_receipt_sha256"] != transaction_binding.sha256
        || freeze_value["manifest_sha256"] != freeze_value["freeze_digest"]
        || attestations
            .iter()
            .any(|(_, attestation)| attestation.freeze_digest != freeze_value["freeze_digest"])
    {
        return Err(policy_error(
            "GATE-EXTERNAL-CUSTODY-CROSS-BINDING",
            &freeze_binding.path,
        ));
    }
    let plan = load_plan(&options.plan_path)?;
    let parent = plan.parent_plan.as_ref().ok_or_else(|| {
        policy_error(
            "GATE-EXTERNAL-PARENT-PLAN",
            "holdout custody requires a Generation-A parent plan",
        )
    })?;
    verify_external_transaction(Path::new(&parent.path), &transaction_value)
}

fn create_opening_token_custody_receipt(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
) -> Result<Value> {
    if !transaction
        .heavy
        .iter()
        .any(|node| node.harvard_access == "OPENS_HARVARD")
    {
        return Ok(Value::Null);
    }
    let custody_root = options.custody_root.as_ref().ok_or_else(|| {
        policy_error(
            "GATE-EXTERNAL-CUSTODY-ROOT-REQUIRED",
            &transaction.transaction_id,
        )
    })?;
    let token = options.opening_token.as_ref().ok_or_else(|| {
        policy_error(
            "GATE-EXTERNAL-HARVARD-TOKEN-REQUIRED",
            &transaction.transaction_id,
        )
    })?;
    let token_canonical = fs::canonicalize(token)
        .map_err(|error| external_error("GATE-EXTERNAL-HARVARD-TOKEN", error))?;
    let custody_canonical = fs::canonicalize(custody_root)
        .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-ROOT", error))?;
    let token_metadata = fs::symlink_metadata(token)
        .map_err(|error| external_error("GATE-EXTERNAL-HARVARD-TOKEN", error))?;
    if token_metadata.file_type().is_symlink()
        || !token_metadata.is_file()
        || token_canonical != custody_canonical.join("holdout-opened-once.lock")
    {
        return Err(policy_error(
            "GATE-EXTERNAL-HARVARD-TOKEN-PATH",
            &token.display().to_string(),
        ));
    }
    let holdout_receipt = holdout_execution_receipt_path(options, transaction)?;
    let holdout_bytes = fs::read(&holdout_receipt)
        .map_err(|error| external_error("GATE-EXTERNAL-HOLDOUT-RECEIPT", error))?;
    let fields = holdout_execution_fields(&holdout_bytes)?;
    let freeze = parse_strict(
        &fs::read(custody_root.join("freeze.receipt.json"))
            .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?,
    )?;
    let token_sha256 = file_sha256(token)?;
    if fields.get("state").map(String::as_str) != Some("PASS_SCORED_NO_REFIT")
        || fields.get("token_sha256") != Some(&token_sha256)
        || fields.get("freeze_digest").map(String::as_str) != freeze["freeze_digest"].as_str()
    {
        return Err(policy_error(
            "GATE-EXTERNAL-OPENING-CUSTODY",
            "holdout receipt does not bind the opening token and freeze",
        ));
    }
    let mut value = json!({
        "schema": "openwepp-holdout-opening-token-receipt-v1",
        "opening_token_receipt_id": "",
        "result": "PASS",
        "token_path": token.display().to_string(),
        "token_sha256": token_sha256,
        "freeze_digest": freeze["freeze_digest"],
        "holdout_execution_receipt_path": holdout_receipt.display().to_string(),
        "holdout_execution_receipt_sha256": sha256_bytes(&holdout_bytes),
    });
    value["opening_token_receipt_id"] =
        Value::String(derived_id(&value, "opening_token_receipt_id")?);
    persist_exclusive(
        &custody_root.join("holdout-opening-token.receipt.json"),
        &value,
    )?;
    Ok(value)
}

fn verify_opening_token_custody_receipt(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
    embedded: &Value,
) -> Result<()> {
    let opens = transaction
        .heavy
        .iter()
        .any(|node| node.harvard_access == "OPENS_HARVARD");
    if !opens {
        return if embedded.is_null() {
            Ok(())
        } else {
            Err(policy_error(
                "GATE-EXTERNAL-OPENING-CUSTODY",
                "non-Harvard transaction carried opening custody",
            ))
        };
    }
    let custody_root = options
        .custody_root
        .as_ref()
        .ok_or_else(|| policy_error("GATE-EXTERNAL-CUSTODY-ROOT-REQUIRED", "opening custody"))?;
    let path = custody_root.join("holdout-opening-token.receipt.json");
    let persisted = parse_strict(
        &fs::read(&path).map_err(|error| external_error("GATE-EXTERNAL-OPENING-CUSTODY", error))?,
    )?;
    let schema = parse_strict(include_bytes!(
        "../../../../gate-policy/v1/schemas/holdout-opening-token-receipt.schema.json"
    ))?;
    validate_schema(&schema, &persisted, "holdout-opening-token-receipt")?;
    if &persisted != embedded
        || embedded["schema"] != "openwepp-holdout-opening-token-receipt-v1"
        || embedded["result"] != "PASS"
        || derived_id(embedded, "opening_token_receipt_id")? != embedded["opening_token_receipt_id"]
    {
        return Err(policy_error(
            "GATE-EXTERNAL-OPENING-CUSTODY",
            &path.display().to_string(),
        ));
    }
    let token = options
        .opening_token
        .as_ref()
        .ok_or_else(|| policy_error("GATE-EXTERNAL-HARVARD-TOKEN-REQUIRED", "opening custody"))?;
    if token != &custody_root.join("holdout-opened-once.lock") {
        return Err(policy_error(
            "GATE-EXTERNAL-HARVARD-TOKEN-PATH",
            &token.display().to_string(),
        ));
    }
    let holdout_receipt = holdout_execution_receipt_path(options, transaction)?;
    let holdout_bytes = fs::read(&holdout_receipt)
        .map_err(|error| external_error("GATE-EXTERNAL-HOLDOUT-RECEIPT", error))?;
    let fields = holdout_execution_fields(&holdout_bytes)?;
    let freeze = parse_strict(
        &fs::read(custody_root.join("freeze.receipt.json"))
            .map_err(|error| external_error("GATE-EXTERNAL-CUSTODY-READ", error))?,
    )?;
    if embedded["token_path"] != token.display().to_string()
        || embedded["token_sha256"] != file_sha256(token)?
        || embedded["holdout_execution_receipt_path"] != holdout_receipt.display().to_string()
        || embedded["holdout_execution_receipt_sha256"] != sha256_bytes(&holdout_bytes)
        || fields.get("token_sha256").map(String::as_str) != embedded["token_sha256"].as_str()
        || fields.get("freeze_digest").map(String::as_str) != embedded["freeze_digest"].as_str()
        || embedded["freeze_digest"] != freeze["freeze_digest"]
        || fields.get("state").map(String::as_str) != Some("PASS_SCORED_NO_REFIT")
    {
        return Err(policy_error(
            "GATE-EXTERNAL-OPENING-CUSTODY",
            "opening token, freeze, or holdout receipt drifted",
        ));
    }
    Ok(())
}

fn holdout_execution_receipt_path(
    options: &ExternalTransitionOptions,
    transaction: &ExternalTransaction,
) -> Result<PathBuf> {
    let relative = transaction
        .heavy
        .iter()
        .flat_map(|node| &node.declared_outputs)
        .find(|path| path.ends_with("/holdout-execution-receipt.csv"))
        .ok_or_else(|| {
            policy_error(
                "GATE-EXTERNAL-OPENING-CUSTODY",
                "holdout execution receipt is undeclared",
            )
        })?;
    Ok(options.attempt_root.join(relative))
}

fn holdout_execution_fields(bytes: &[u8]) -> Result<BTreeMap<String, String>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bytes);
    let headers = reader
        .headers()
        .map_err(|error| external_error("GATE-EXTERNAL-HOLDOUT-RECEIPT", error))?
        .clone();
    let rows = reader
        .records()
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|error| external_error("GATE-EXTERNAL-HOLDOUT-RECEIPT", error))?;
    if rows.len() != 1 || headers.len() != rows[0].len() {
        return Err(policy_error(
            "GATE-EXTERNAL-HOLDOUT-RECEIPT",
            "expected exactly one complete row",
        ));
    }
    Ok(headers
        .iter()
        .zip(rows[0].iter())
        .map(|(name, value)| (name.to_owned(), value.to_owned()))
        .collect())
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
    let operands = attestation
        .argv
        .get(1..)
        .ok_or_else(|| policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id))?;
    if operands.len() % 2 != 0 {
        return Err(policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id));
    }
    let mut supplied = BTreeMap::new();
    for pair in operands.chunks_exact(2) {
        if supplied
            .insert(pair[0].as_str(), pair[1].as_str())
            .is_some()
        {
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
    let verifier_suffix = verifier_id
        .strip_prefix("verifier_")
        .ok_or_else(|| policy_error("GATE-EXTERNAL-CUSTODY-ARGV", verifier_id))?;
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
        ("--transaction-id", attestation.transaction_id.clone()),
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
