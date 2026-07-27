//! Journaled publication of authenticated external transaction outputs.
//!
//! Publication is deliberately separate from execution.  This module never
//! upgrades an execution receipt: it verifies an exact source manifest and an
//! exact destination baseline, records durable intent, then installs staged
//! bytes.  A process failure can therefore leave only an explicit,
//! recoverable `NON_ACCEPTED` transaction.

use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::canonical::{
    canonical_bytes, derived_id, digest, parse_strict, sha256_bytes, validate_schema,
};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::external_outputs::manifest_declared_outputs;

const RECEIPT_SCHEMA: &str = "openwepp-publication-receipt-v1";
const JOURNAL_SCHEMA: &str = "openwepp-publication-journal-v1";

/// One exact external source and repository destination binding.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicationEntry {
    pub relative_path: PathBuf,
    pub source_sha256: String,
    /// Expected destination bytes, or `None` when the destination must be absent.
    pub destination_baseline_sha256: Option<String>,
}

/// Complete authority for one publication transaction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicationPlan {
    pub publication_id: String,
    pub transaction_receipt_id: String,
    pub transaction_receipt_sha256: String,
    pub source_manifest_id: String,
    pub source_manifest_sha256: String,
    pub destination_baseline_sha256: String,
    pub source_root: PathBuf,
    pub destination_root: PathBuf,
    pub transaction_root: PathBuf,
    pub journal_path: PathBuf,
    pub receipt_path: PathBuf,
    pub entries: Vec<PublicationEntry>,
}

/// Durable per-file intent, appended before destination installation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicationJournalRecord {
    pub schema_version: String,
    pub publication_id: String,
    pub sequence: usize,
    pub operation: JournalOperation,
    pub relative_path: PathBuf,
    pub source_sha256: String,
    pub destination_baseline_sha256: Option<String>,
    pub backup_relative_path: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JournalOperation {
    Install,
    Restore,
}

/// Receipt emitted only after every destination byte has been reverified.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicationReceipt {
    pub schema_version: String,
    pub receipt_id: String,
    pub transaction_receipt_id: String,
    pub transaction_receipt_sha256: String,
    pub source_manifest_id: String,
    pub source_manifest_sha256: String,
    pub source_root: PathBuf,
    pub destination_root: PathBuf,
    pub destination_baseline_sha256: String,
    pub journal_path: PathBuf,
    pub journal_sha256: String,
    pub files: Vec<PublishedFile>,
    pub started_at: String,
    pub finished_at: String,
    pub result: PublicationResult,
    pub reason_codes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublishedFile {
    pub source_relative_path: PathBuf,
    pub destination_relative_path: PathBuf,
    pub sha256: String,
    pub size_bytes: u64,
    pub prior_destination_sha256: Option<String>,
    pub journal_sequence: usize,
    pub installed: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PublicationResult {
    Pass,
    NonAccepted,
    Fail,
    Invalid,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PublicationStatus {
    Accepted,
    NonAccepted,
    Restored,
}

/// Observable result. `NonAccepted` is a durable partial transaction, not an
/// accepted receipt and not permission to retry publication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicationOutcome {
    pub status: PublicationStatus,
    pub installed_entries: usize,
    pub failure_code: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryAction {
    Complete,
    Restore,
}

/// Verify, journal, stage, and install all files in the plan.
///
/// # Errors
///
/// Returns a typed error before mutation for malformed plans, source drift,
/// destination collisions, an existing transaction, or filesystem failures.
/// Failures after the first durable journal record return `NON_ACCEPTED`.
pub fn publish(plan: &PublicationPlan) -> Result<PublicationOutcome> {
    let started_at = now()?;
    validate_plan(plan)?;
    require_fresh_transaction(plan)?;
    verify_sources(plan)?;
    verify_destination_baseline(plan)?;
    prepare_transaction_directories(plan)?;

    let mut installed = 0;
    for (index, entry) in plan.entries.iter().enumerate() {
        let sequence = index + 1;
        match install_entry(plan, entry, sequence, false) {
            Ok(()) => installed += 1,
            Err(error) => {
                if plan.journal_path.exists() {
                    return Ok(PublicationOutcome {
                        status: PublicationStatus::NonAccepted,
                        installed_entries: installed,
                        failure_code: Some(error.code),
                    });
                }
                return Err(error);
            }
        }
    }
    finish_accepted(plan, installed, &started_at)
}

/// Deterministically complete or restore one journaled partial transaction.
///
/// # Errors
///
/// Returns a typed error if the journal is absent, malformed, belongs to
/// another plan, or if source, installed, baseline, or backup bytes drifted.
pub fn recover(plan: &PublicationPlan, action: RecoveryAction) -> Result<PublicationOutcome> {
    validate_plan(plan)?;
    let journal = load_journal(plan)?;
    validate_journal(plan, &journal)?;
    if plan.receipt_path.exists() {
        return Err(publication_error(
            ErrorClass::Receipt,
            "GATE-PUBLICATION-ALREADY-ACCEPTED",
            "an accepted publication cannot be recovered",
        ));
    }
    match action {
        RecoveryAction::Complete => recover_complete(plan, &journal),
        RecoveryAction::Restore => recover_restore(plan, &journal),
    }
}

/// Independently verify a receipt and the complete destination manifest.
///
/// # Errors
///
/// Returns a typed receipt or identity error for any mismatch.
pub fn verify_receipt(plan: &PublicationPlan, receipt: &PublicationReceipt) -> Result<()> {
    validate_plan(plan)?;
    validate_receipt_schema(receipt)?;
    if receipt.schema_version != RECEIPT_SCHEMA
        || receipt.transaction_receipt_id != plan.transaction_receipt_id
        || receipt.transaction_receipt_sha256 != plan.transaction_receipt_sha256
        || receipt.source_manifest_id != plan.source_manifest_id
        || receipt.source_manifest_sha256 != plan.source_manifest_sha256
        || receipt.source_root != plan.source_root
        || receipt.destination_root != plan.destination_root
        || receipt.destination_baseline_sha256 != plan.destination_baseline_sha256
        || receipt.journal_path != plan.journal_path
        || receipt.result != PublicationResult::Pass
        || !receipt.reason_codes.is_empty()
    {
        return Err(publication_error(
            ErrorClass::Receipt,
            "GATE-PUBLICATION-RECEIPT-MISMATCH",
            "publication receipt does not bind the exact accepted plan",
        ));
    }
    verify_installed_manifest(plan)?;
    let journal_sha256 =
        sha256_bytes(&fs::read(&plan.journal_path).map_err(|error| {
            io_error("GATE-PUBLICATION-JOURNAL-READ", &plan.journal_path, error)
        })?);
    if receipt.journal_sha256 != journal_sha256
        || receipt.files != published_files(plan)?
        || derived_receipt_id(receipt)? != receipt.receipt_id
    {
        return Err(publication_error(
            ErrorClass::Receipt,
            "GATE-PUBLICATION-RECEIPT-MANIFEST",
            "receipt journal, file manifest, or derived identity mismatches",
        ));
    }
    Ok(())
}

fn recover_complete(
    plan: &PublicationPlan,
    journal: &[PublicationJournalRecord],
) -> Result<PublicationOutcome> {
    verify_sources(plan)?;
    let mut installed = 0;
    for (index, entry) in plan.entries.iter().enumerate() {
        let sequence = index + 1;
        let destination = confined_join(&plan.destination_root, &entry.relative_path)?;
        if regular_file_hash(&destination).ok().as_deref() == Some(entry.source_sha256.as_str()) {
            installed += 1;
            continue;
        }
        let recorded = journal.iter().any(|record| {
            record.operation == JournalOperation::Install
                && record.sequence == sequence
                && record.relative_path == entry.relative_path
        });
        if recorded {
            install_entry(plan, entry, sequence, true)?;
            installed += 1;
        } else {
            verify_one_destination_baseline(plan, entry)?;
            install_entry(plan, entry, sequence, false)?;
            installed += 1;
        }
    }
    let started_at = now()?;
    finish_accepted(plan, installed, &started_at)
}

fn recover_restore(
    plan: &PublicationPlan,
    journal: &[PublicationJournalRecord],
) -> Result<PublicationOutcome> {
    for record in journal
        .iter()
        .filter(|record| record.operation == JournalOperation::Install)
        .rev()
    {
        restore_entry(plan, record)?;
    }
    sync_directory(&plan.destination_root)?;
    Ok(PublicationOutcome {
        status: PublicationStatus::Restored,
        installed_entries: 0,
        failure_code: None,
    })
}

fn install_entry(
    plan: &PublicationPlan,
    entry: &PublicationEntry,
    sequence: usize,
    already_journaled: bool,
) -> Result<()> {
    let source = confined_join(&plan.source_root, &entry.relative_path)?;
    let destination = confined_join(&plan.destination_root, &entry.relative_path)?;
    let staged = confined_join(&plan.transaction_root.join("staged"), &entry.relative_path)?;
    let backup_relative = entry
        .destination_baseline_sha256
        .as_ref()
        .map(|_| PathBuf::from("backups").join(&entry.relative_path));
    if !already_journaled {
        if let Some(relative) = &backup_relative {
            let backup = confined_join(&plan.transaction_root, relative)?;
            copy_verified(
                &destination,
                &backup,
                entry.destination_baseline_sha256.as_deref(),
            )?;
        }
        append_journal(
            plan,
            &PublicationJournalRecord {
                schema_version: JOURNAL_SCHEMA.to_owned(),
                publication_id: plan.publication_id.clone(),
                sequence,
                operation: JournalOperation::Install,
                relative_path: entry.relative_path.clone(),
                source_sha256: entry.source_sha256.clone(),
                destination_baseline_sha256: entry.destination_baseline_sha256.clone(),
                backup_relative_path: backup_relative,
            },
        )?;
    }
    copy_verified(&source, &staged, Some(&entry.source_sha256))?;
    ensure_parent(&destination)?;
    reject_symlink_or_special_if_present(&destination)?;
    fs::rename(&staged, &destination)
        .map_err(|error| io_error("GATE-PUBLICATION-INSTALL-RENAME", &destination, error))?;
    sync_parent(&destination)?;
    require_hash(&destination, &entry.source_sha256)
}

fn restore_entry(plan: &PublicationPlan, record: &PublicationJournalRecord) -> Result<()> {
    let destination = confined_join(&plan.destination_root, &record.relative_path)?;
    reject_symlink_or_special_if_present(&destination)?;
    match (
        &record.destination_baseline_sha256,
        &record.backup_relative_path,
    ) {
        (Some(expected), Some(relative)) => {
            let backup = confined_join(&plan.transaction_root, relative)?;
            require_hash(&backup, expected)?;
            let staged = confined_join(
                &plan.transaction_root.join("restore-staged"),
                &record.relative_path,
            )?;
            copy_verified(&backup, &staged, Some(expected))?;
            append_restore_record(plan, record)?;
            ensure_parent(&destination)?;
            fs::rename(&staged, &destination).map_err(|error| {
                io_error("GATE-PUBLICATION-RESTORE-RENAME", &destination, error)
            })?;
            sync_parent(&destination)?;
            require_hash(&destination, expected)
        }
        (None, None) => {
            append_restore_record(plan, record)?;
            match fs::remove_file(&destination) {
                Ok(()) => sync_parent(&destination),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(error) => Err(io_error(
                    "GATE-PUBLICATION-RESTORE-REMOVE",
                    &destination,
                    error,
                )),
            }
        }
        _ => Err(publication_error(
            ErrorClass::Ledger,
            "GATE-PUBLICATION-BACKUP-BINDING",
            "journal baseline and backup binding disagree",
        )),
    }
}

fn append_restore_record(plan: &PublicationPlan, prior: &PublicationJournalRecord) -> Result<()> {
    append_journal(
        plan,
        &PublicationJournalRecord {
            operation: JournalOperation::Restore,
            ..prior.clone()
        },
    )
}

fn validate_plan(plan: &PublicationPlan) -> Result<()> {
    if plan.publication_id.is_empty()
        || plan.transaction_receipt_id.is_empty()
        || plan.source_manifest_id.is_empty()
    {
        return Err(publication_error(
            ErrorClass::Schema,
            "GATE-PUBLICATION-IDENTITY",
            "publication and terminal receipt identities are required",
        ));
    }
    for digest in [
        &plan.transaction_receipt_id,
        &plan.transaction_receipt_sha256,
        &plan.source_manifest_id,
        &plan.source_manifest_sha256,
        &plan.destination_baseline_sha256,
    ] {
        validate_digest(digest)?;
    }
    if plan.entries.is_empty() {
        return Err(publication_error(
            ErrorClass::Schema,
            "GATE-PUBLICATION-EMPTY",
            "publication manifest is empty",
        ));
    }
    require_absolute_distinct_roots(plan)?;
    let mut paths = BTreeSet::new();
    for entry in &plan.entries {
        validate_relative_path(&entry.relative_path)?;
        validate_digest(&entry.source_sha256)?;
        if let Some(digest) = &entry.destination_baseline_sha256 {
            validate_digest(digest)?;
        }
        if !paths.insert(entry.relative_path.clone()) {
            return Err(publication_error(
                ErrorClass::Schema,
                "GATE-PUBLICATION-DUPLICATE",
                "publication manifest contains a duplicate destination",
            ));
        }
    }
    Ok(())
}

fn require_absolute_distinct_roots(plan: &PublicationPlan) -> Result<()> {
    let roots = [
        &plan.source_root,
        &plan.destination_root,
        &plan.transaction_root,
    ];
    if roots.iter().any(|root| !root.is_absolute()) {
        return Err(publication_error(
            ErrorClass::Schema,
            "GATE-PUBLICATION-ROOT",
            "publication roots must be absolute",
        ));
    }
    if plan.source_root.starts_with(&plan.destination_root)
        || plan.destination_root.starts_with(&plan.source_root)
        || plan.transaction_root.starts_with(&plan.source_root)
        || plan.source_root.starts_with(&plan.transaction_root)
        || plan.transaction_root.starts_with(&plan.destination_root)
        || plan.destination_root.starts_with(&plan.transaction_root)
    {
        return Err(publication_error(
            ErrorClass::Policy,
            "GATE-PUBLICATION-ROOT-OVERLAP",
            "source, destination, and transaction roots must not overlap",
        ));
    }
    if !plan.journal_path.starts_with(&plan.transaction_root)
        || !plan.receipt_path.starts_with(&plan.transaction_root)
    {
        return Err(publication_error(
            ErrorClass::Policy,
            "GATE-PUBLICATION-TRANSACTION-PATH",
            "journal and receipt must be confined below the transaction root",
        ));
    }
    Ok(())
}

fn validate_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(publication_error(
            ErrorClass::Schema,
            "GATE-PUBLICATION-PATH",
            "manifest paths must be confined relative paths",
        ));
    }
    Ok(())
}

fn validate_digest(digest: &str) -> Result<()> {
    if digest.len() == 64
        && digest
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err(publication_error(
            ErrorClass::Schema,
            "GATE-PUBLICATION-DIGEST",
            "manifest digest must be lowercase SHA-256",
        ))
    }
}

fn require_fresh_transaction(plan: &PublicationPlan) -> Result<()> {
    if plan.journal_path.exists() || plan.receipt_path.exists() || plan.transaction_root.exists() {
        return Err(publication_error(
            ErrorClass::Policy,
            "GATE-PUBLICATION-NOT-FRESH",
            "publication transaction already exists; recover it before retry",
        ));
    }
    Ok(())
}

fn verify_sources(plan: &PublicationPlan) -> Result<()> {
    reject_symlink_components(&plan.source_root)?;
    for entry in &plan.entries {
        require_hash(
            &confined_join(&plan.source_root, &entry.relative_path)?,
            &entry.source_sha256,
        )?;
    }
    let declared = plan
        .entries
        .iter()
        .map(|entry| entry.relative_path.clone())
        .collect::<Vec<_>>();
    let manifest = manifest_declared_outputs(&plan.source_root, &declared)?;
    let manifest_value = serde_json::to_value(&manifest).map_err(|error| {
        publication_error(
            ErrorClass::Json,
            "GATE-PUBLICATION-SOURCE-MANIFEST",
            error.to_string(),
        )
    })?;
    if manifest.manifest_id != plan.source_manifest_id
        || digest(&manifest_value)? != plan.source_manifest_sha256
    {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-SOURCE-MANIFEST-DRIFT",
            "source manifest identity differs from the producing transaction",
        ));
    }
    Ok(())
}

fn verify_destination_baseline(plan: &PublicationPlan) -> Result<()> {
    reject_symlink_components(&plan.destination_root)?;
    for entry in &plan.entries {
        verify_one_destination_baseline(plan, entry)?;
    }
    if destination_baseline_digest(&plan.entries)? != plan.destination_baseline_sha256 {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-BASELINE-MANIFEST",
            "destination baseline identity differs from the plan",
        ));
    }
    Ok(())
}

fn destination_baseline_digest(entries: &[PublicationEntry]) -> Result<String> {
    let baseline = entries
        .iter()
        .map(|entry| {
            serde_json::json!({
                "destination_relative_path": entry.relative_path,
                "sha256": entry.destination_baseline_sha256,
            })
        })
        .collect::<Vec<_>>();
    digest(&serde_json::Value::Array(baseline))
}

fn verify_one_destination_baseline(plan: &PublicationPlan, entry: &PublicationEntry) -> Result<()> {
    let destination = confined_join(&plan.destination_root, &entry.relative_path)?;
    match &entry.destination_baseline_sha256 {
        Some(expected) => require_hash(&destination, expected),
        None if destination.exists() => Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-COLLISION",
            format!("destination must be absent: {}", destination.display()),
        )),
        None => Ok(()),
    }
}

fn prepare_transaction_directories(plan: &PublicationPlan) -> Result<()> {
    fs::create_dir_all(plan.transaction_root.join("staged")).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-TRANSACTION-DIR",
            &plan.transaction_root,
            error,
        )
    })?;
    ensure_parent(&plan.journal_path)?;
    ensure_parent(&plan.receipt_path)
}

fn append_journal(plan: &PublicationPlan, record: &PublicationJournalRecord) -> Result<()> {
    let mut bytes = canonical_bytes(&serde_json::to_value(record).map_err(|error| {
        publication_error(
            ErrorClass::Json,
            "GATE-PUBLICATION-JOURNAL-JSON",
            error.to_string(),
        )
    })?)?;
    bytes.push(b'\n');
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&plan.journal_path)
        .map_err(|error| io_error("GATE-PUBLICATION-JOURNAL-OPEN", &plan.journal_path, error))?;
    file.write_all(&bytes)
        .and_then(|()| file.sync_all())
        .map_err(|error| io_error("GATE-PUBLICATION-JOURNAL-WRITE", &plan.journal_path, error))?;
    sync_parent(&plan.journal_path)
}

fn load_journal(plan: &PublicationPlan) -> Result<Vec<PublicationJournalRecord>> {
    let bytes = fs::read(&plan.journal_path)
        .map_err(|error| io_error("GATE-PUBLICATION-JOURNAL-READ", &plan.journal_path, error))?;
    let mut records = Vec::new();
    for line in bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
    {
        records.push(serde_json::from_slice(line).map_err(|error| {
            publication_error(
                ErrorClass::Ledger,
                "GATE-PUBLICATION-JOURNAL-INVALID",
                error.to_string(),
            )
        })?);
    }
    if records.is_empty() {
        return Err(publication_error(
            ErrorClass::Ledger,
            "GATE-PUBLICATION-JOURNAL-EMPTY",
            "publication journal is empty",
        ));
    }
    Ok(records)
}

fn validate_journal(plan: &PublicationPlan, records: &[PublicationJournalRecord]) -> Result<()> {
    for record in records {
        if record.schema_version != JOURNAL_SCHEMA
            || record.publication_id != plan.publication_id
            || record.sequence == 0
            || plan.entries.get(record.sequence - 1).is_none_or(|entry| {
                entry.relative_path != record.relative_path
                    || entry.source_sha256 != record.source_sha256
                    || entry.destination_baseline_sha256 != record.destination_baseline_sha256
            })
        {
            return Err(publication_error(
                ErrorClass::Ledger,
                "GATE-PUBLICATION-JOURNAL-MISMATCH",
                "journal does not bind the exact publication plan",
            ));
        }
    }
    Ok(())
}

fn finish_accepted(
    plan: &PublicationPlan,
    installed: usize,
    started_at: &str,
) -> Result<PublicationOutcome> {
    verify_installed_manifest(plan)?;
    let journal_sha256 =
        sha256_bytes(&fs::read(&plan.journal_path).map_err(|error| {
            io_error("GATE-PUBLICATION-JOURNAL-READ", &plan.journal_path, error)
        })?);
    let mut receipt = PublicationReceipt {
        schema_version: RECEIPT_SCHEMA.to_owned(),
        receipt_id: "0".repeat(64),
        transaction_receipt_id: plan.transaction_receipt_id.clone(),
        transaction_receipt_sha256: plan.transaction_receipt_sha256.clone(),
        source_manifest_id: plan.source_manifest_id.clone(),
        source_manifest_sha256: plan.source_manifest_sha256.clone(),
        source_root: plan.source_root.clone(),
        destination_root: plan.destination_root.clone(),
        destination_baseline_sha256: plan.destination_baseline_sha256.clone(),
        journal_path: plan.journal_path.clone(),
        journal_sha256,
        files: published_files(plan)?,
        started_at: started_at.to_owned(),
        finished_at: now()?,
        result: PublicationResult::Pass,
        reason_codes: Vec::new(),
    };
    receipt.receipt_id = derived_receipt_id(&receipt)?;
    validate_receipt_schema(&receipt)?;
    let bytes = canonical_bytes(&serde_json::to_value(receipt).map_err(|error| {
        publication_error(
            ErrorClass::Json,
            "GATE-PUBLICATION-RECEIPT-JSON",
            error.to_string(),
        )
    })?)?;
    write_new_synced(&plan.receipt_path, &bytes)?;
    Ok(PublicationOutcome {
        status: PublicationStatus::Accepted,
        installed_entries: installed,
        failure_code: None,
    })
}

fn validate_receipt_schema(receipt: &PublicationReceipt) -> Result<()> {
    let schema = parse_strict(include_bytes!(
        "../../../gate-policy/v1/schemas/publication-receipt.schema.json"
    ))?;
    let value = serde_json::to_value(receipt).map_err(|error| {
        publication_error(
            ErrorClass::Json,
            "GATE-PUBLICATION-RECEIPT-JSON",
            error.to_string(),
        )
    })?;
    validate_schema(&schema, &value, "publication-receipt")
}

fn published_files(plan: &PublicationPlan) -> Result<Vec<PublishedFile>> {
    plan.entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let source = confined_join(&plan.source_root, &entry.relative_path)?;
            let size_bytes = fs::metadata(&source)
                .map_err(|error| io_error("GATE-PUBLICATION-METADATA", &source, error))?
                .len();
            Ok(PublishedFile {
                source_relative_path: entry.relative_path.clone(),
                destination_relative_path: entry.relative_path.clone(),
                sha256: entry.source_sha256.clone(),
                size_bytes,
                prior_destination_sha256: entry.destination_baseline_sha256.clone(),
                journal_sequence: index + 1,
                installed: true,
            })
        })
        .collect()
}

fn derived_receipt_id(receipt: &PublicationReceipt) -> Result<String> {
    let value = serde_json::to_value(receipt).map_err(|error| {
        publication_error(
            ErrorClass::Json,
            "GATE-PUBLICATION-RECEIPT-JSON",
            error.to_string(),
        )
    })?;
    derived_id(&value, "receipt_id")
}

fn now() -> Result<String> {
    OffsetDateTime::now_utc().format(&Rfc3339).map_err(|error| {
        publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-TIMESTAMP",
            error.to_string(),
        )
    })
}

fn verify_installed_manifest(plan: &PublicationPlan) -> Result<()> {
    for entry in &plan.entries {
        require_hash(
            &confined_join(&plan.destination_root, &entry.relative_path)?,
            &entry.source_sha256,
        )?;
    }
    Ok(())
}

fn copy_verified(source: &Path, destination: &Path, expected: Option<&str>) -> Result<()> {
    reject_symlink_or_special_if_present(source)?;
    ensure_parent(destination)?;
    let mut input = open_read_nofollow(source)?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination)
        .map_err(|error| io_error("GATE-PUBLICATION-STAGE-CREATE", destination, error))?;
    std::io::copy(&mut input, &mut output)
        .and_then(|_| output.sync_all())
        .map_err(|error| io_error("GATE-PUBLICATION-STAGE-WRITE", destination, error))?;
    sync_parent(destination)?;
    if let Some(digest) = expected {
        require_hash(destination, digest)?;
    }
    Ok(())
}

fn regular_file_hash(path: &Path) -> Result<String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| io_error("GATE-PUBLICATION-METADATA", path, error))?;
    if !metadata.file_type().is_file() {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-NONREGULAR",
            format!("not a regular file: {}", path.display()),
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() != 1 {
            return Err(publication_error(
                ErrorClass::Identity,
                "GATE-PUBLICATION-HARDLINK",
                format!("hardlinked file rejected: {}", path.display()),
            ));
        }
    }
    let mut file = open_read_nofollow(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|error| io_error("GATE-PUBLICATION-FILE-READ", path, error))?;
    let after = file
        .metadata()
        .map_err(|error| io_error("GATE-PUBLICATION-METADATA", path, error))?;
    if metadata.len() != after.len() || metadata.modified().ok() != after.modified().ok() {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-MUTATED",
            format!("file changed while hashing: {}", path.display()),
        ));
    }
    Ok(sha256_bytes(&bytes))
}

fn require_hash(path: &Path, expected: &str) -> Result<()> {
    let actual = regular_file_hash(path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-DRIFT",
            format!("digest drift at {}", path.display()),
        ))
    }
}

fn open_read_nofollow(path: &Path) -> Result<File> {
    #[cfg(unix)]
    {
        use rustix::fs::{Mode, OFlags, open};
        let descriptor = open(
            path,
            OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
            Mode::empty(),
        )
        .map_err(|error| io_error("GATE-PUBLICATION-FILE-OPEN", path, error))?;
        Ok(File::from(descriptor))
    }
    #[cfg(not(unix))]
    {
        File::open(path).map_err(|error| io_error("GATE-PUBLICATION-FILE-OPEN", path, error))
    }
}

fn reject_symlink_or_special_if_present(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-SYMLINK",
            format!("symlink rejected: {}", path.display()),
        )),
        Ok(metadata) if !metadata.file_type().is_file() => Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-NONREGULAR",
            format!("special path rejected: {}", path.display()),
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(io_error("GATE-PUBLICATION-METADATA", path, error)),
    }
}

fn reject_symlink_components(path: &Path) -> Result<()> {
    let mut current = PathBuf::new();
    for component in path.components() {
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(publication_error(
                    ErrorClass::Identity,
                    "GATE-PUBLICATION-ROOT-SYMLINK",
                    format!("symlink component rejected: {}", current.display()),
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
            Err(error) => {
                return Err(io_error("GATE-PUBLICATION-ROOT-METADATA", &current, error));
            }
        }
    }
    Ok(())
}

fn confined_join(root: &Path, relative: &Path) -> Result<PathBuf> {
    validate_relative_path(relative)?;
    reject_symlink_components(root)?;
    let joined = root.join(relative);
    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component);
        if current != joined {
            match fs::symlink_metadata(&current) {
                Ok(metadata) if metadata.file_type().is_symlink() => {
                    return Err(publication_error(
                        ErrorClass::Identity,
                        "GATE-PUBLICATION-PATH-SYMLINK",
                        format!("symlink component rejected: {}", current.display()),
                    ));
                }
                Ok(metadata) if !metadata.file_type().is_dir() => {
                    return Err(publication_error(
                        ErrorClass::Identity,
                        "GATE-PUBLICATION-PATH-COLLISION",
                        format!("non-directory component: {}", current.display()),
                    ));
                }
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(io_error("GATE-PUBLICATION-PATH-METADATA", &current, error));
                }
            }
        }
    }
    Ok(joined)
}

fn ensure_parent(path: &Path) -> Result<()> {
    let parent = path.parent().ok_or_else(|| {
        publication_error(
            ErrorClass::Io,
            "GATE-PUBLICATION-PARENT",
            "path has no parent",
        )
    })?;
    fs::create_dir_all(parent)
        .map_err(|error| io_error("GATE-PUBLICATION-MKDIR", parent, error))?;
    reject_symlink_components(parent)
}

fn write_new_synced(path: &Path, bytes: &[u8]) -> Result<()> {
    ensure_parent(path)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| io_error("GATE-PUBLICATION-RECEIPT-CREATE", path, error))?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|error| io_error("GATE-PUBLICATION-RECEIPT-WRITE", path, error))?;
    sync_parent(path)
}

fn sync_parent(path: &Path) -> Result<()> {
    let parent = path.parent().ok_or_else(|| {
        publication_error(
            ErrorClass::Io,
            "GATE-PUBLICATION-PARENT",
            "path has no parent",
        )
    })?;
    sync_directory(parent)
}

fn sync_directory(path: &Path) -> Result<()> {
    File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| io_error("GATE-PUBLICATION-DIR-SYNC", path, error))
}

fn io_error(code: &'static str, path: &Path, error: impl std::fmt::Display) -> GatePolicyError {
    publication_error(ErrorClass::Io, code, format!("{}: {error}", path.display()))
}

fn publication_error(
    class: ErrorClass,
    code: &'static str,
    message: impl Into<String>,
) -> GatePolicyError {
    GatePolicyError::new(class, code, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "openwepp-publication-{name}-{}",
            std::process::id()
        ))
    }

    fn remove_scratch(path: &Path) {
        if let Err(error) = fs::remove_dir_all(path) {
            assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        }
    }

    fn write(path: &Path, bytes: &[u8]) {
        fs::create_dir_all(path.parent().expect("test path parent")).expect("test parent");
        fs::write(path, bytes).expect("test write");
    }

    fn plan(root: &Path, destination_baseline: Option<&[u8]>) -> PublicationPlan {
        let source = root.join("source");
        let destination = root.join("destination");
        write(&source.join("artifacts/result.json"), b"new-result");
        if let Some(bytes) = destination_baseline {
            write(&destination.join("artifacts/result.json"), bytes);
        } else {
            fs::create_dir_all(&destination).expect("destination root");
        }
        let source_manifest =
            manifest_declared_outputs(&source, &[PathBuf::from("artifacts/result.json")])
                .expect("source manifest");
        let source_manifest_sha256 =
            digest(&serde_json::to_value(&source_manifest).expect("source manifest JSON"))
                .expect("source manifest digest");
        PublicationPlan {
            publication_id: "publication-1".to_owned(),
            transaction_receipt_id: "1".repeat(64),
            transaction_receipt_sha256: "2".repeat(64),
            source_manifest_id: source_manifest.manifest_id,
            source_manifest_sha256,
            destination_baseline_sha256: destination_baseline_digest(&[PublicationEntry {
                relative_path: PathBuf::from("artifacts/result.json"),
                source_sha256: sha256_bytes(b"new-result"),
                destination_baseline_sha256: destination_baseline.map(sha256_bytes),
            }])
            .expect("baseline digest"),
            source_root: source,
            destination_root: destination,
            transaction_root: root.join("transaction"),
            journal_path: root.join("transaction/journal/publication.jsonl"),
            receipt_path: root.join("transaction/receipt/publication.json"),
            entries: vec![PublicationEntry {
                relative_path: PathBuf::from("artifacts/result.json"),
                source_sha256: sha256_bytes(b"new-result"),
                destination_baseline_sha256: destination_baseline.map(sha256_bytes),
            }],
        }
    }

    #[test]
    fn publishes_exact_manifest_and_emits_receipt() {
        let root = scratch("accept");
        remove_scratch(&root);
        let plan = plan(&root, None);
        let outcome = publish(&plan).expect("publication");
        assert_eq!(outcome.status, PublicationStatus::Accepted);
        assert_eq!(
            fs::read(plan.destination_root.join("artifacts/result.json")).expect("installed"),
            b"new-result"
        );
        let receipt: PublicationReceipt =
            serde_json::from_slice(&fs::read(&plan.receipt_path).expect("receipt"))
                .expect("receipt JSON");
        verify_receipt(&plan, &receipt).expect("receipt verification");
        remove_scratch(&root);
    }

    #[test]
    fn rejects_source_drift_and_destination_collision_before_journal() {
        let root = scratch("drift");
        remove_scratch(&root);
        let mut publication_plan = plan(&root, None);
        fs::write(
            publication_plan.source_root.join("artifacts/result.json"),
            b"source drift",
        )
        .expect("source drift");
        assert_eq!(
            publish(&publication_plan).expect_err("drift rejected").code,
            "GATE-PUBLICATION-DRIFT"
        );
        assert!(!publication_plan.journal_path.exists());

        remove_scratch(&root);
        publication_plan = plan(&root, None);
        write(
            &publication_plan
                .destination_root
                .join("artifacts/result.json"),
            b"collision",
        );
        assert_eq!(
            publish(&publication_plan)
                .expect_err("collision rejected")
                .code,
            "GATE-PUBLICATION-COLLISION"
        );
        assert!(!publication_plan.journal_path.exists());
        remove_scratch(&root);
    }

    #[test]
    fn recovery_restores_journal_bound_prior_bytes() {
        let root = scratch("restore");
        remove_scratch(&root);
        let plan = plan(&root, Some(b"old-result"));
        prepare_transaction_directories(&plan).expect("transaction dirs");
        install_entry(&plan, &plan.entries[0], 1, false).expect("partial install");
        assert!(!plan.receipt_path.exists());

        let outcome = recover(&plan, RecoveryAction::Restore).expect("restore");
        assert_eq!(outcome.status, PublicationStatus::Restored);
        assert_eq!(
            fs::read(plan.destination_root.join("artifacts/result.json")).expect("restored"),
            b"old-result"
        );
        remove_scratch(&root);
    }

    #[test]
    fn recovery_completes_a_journaled_partial_transaction() {
        let root = scratch("complete");
        remove_scratch(&root);
        let mut plan = plan(&root, None);
        write(&plan.source_root.join("artifacts/second.json"), b"second");
        plan.entries.push(PublicationEntry {
            relative_path: PathBuf::from("artifacts/second.json"),
            source_sha256: sha256_bytes(b"second"),
            destination_baseline_sha256: None,
        });
        plan.destination_baseline_sha256 =
            destination_baseline_digest(&plan.entries).expect("baseline digest");
        let source_manifest = manifest_declared_outputs(
            &plan.source_root,
            &plan
                .entries
                .iter()
                .map(|entry| entry.relative_path.clone())
                .collect::<Vec<_>>(),
        )
        .expect("source manifest");
        plan.source_manifest_sha256 =
            digest(&serde_json::to_value(&source_manifest).expect("manifest JSON"))
                .expect("manifest digest");
        plan.source_manifest_id = source_manifest.manifest_id;
        prepare_transaction_directories(&plan).expect("transaction dirs");
        install_entry(&plan, &plan.entries[0], 1, false).expect("first install");

        let outcome = recover(&plan, RecoveryAction::Complete).expect("complete");
        assert_eq!(outcome.status, PublicationStatus::Accepted);
        assert!(plan.receipt_path.exists());
        assert_eq!(
            fs::read(plan.destination_root.join("artifacts/second.json")).expect("second"),
            b"second"
        );
        remove_scratch(&root);
    }
}
