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
use crate::external_dag::verify_external_transaction;
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
    pub external_plan_path: PathBuf,
    pub transaction_receipt_path: PathBuf,
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
    pub external_plan_path: PathBuf,
    pub transaction_receipt_path: PathBuf,
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
    verify_producing_transaction(plan)?;
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
    verify_producing_transaction(plan)?;
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
    verify_producing_transaction(plan)?;
    validate_receipt_schema(receipt)?;
    if receipt.schema_version != RECEIPT_SCHEMA
        || receipt.external_plan_path != plan.external_plan_path
        || receipt.transaction_receipt_path != plan.transaction_receipt_path
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
    let destination = confined_join(&plan.destination_root, &entry.relative_path)?;
    let staged = confined_join(&plan.transaction_root.join("staged"), &entry.relative_path)?;
    let backup_relative = entry
        .destination_baseline_sha256
        .as_ref()
        .map(|_| PathBuf::from("backups").join(&entry.relative_path));
    if !already_journaled {
        if let Some(relative) = &backup_relative {
            let backup = confined_join(&plan.transaction_root, relative)?;
            copy_destination_backup(
                plan,
                entry,
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
    copy_source_to_stage(plan, entry, &staged, || {})?;
    ensure_parent(&destination)?;
    reject_symlink_or_special_if_present(&destination)?;
    verify_one_destination_baseline(plan, entry)?;
    install_staged_destination(plan, entry, &staged, || {})?;
    sync_parent(&destination)?;
    require_hash(&destination, &entry.source_sha256)
}

#[cfg(target_os = "linux")]
#[allow(
    clippy::too_many_lines,
    reason = "backup custody keeps descriptor acquisition, byte binding, fsync, and root rechecks contiguous"
)]
fn copy_destination_backup(
    plan: &PublicationPlan,
    entry: &PublicationEntry,
    backup: &Path,
    expected: Option<&str>,
) -> Result<()> {
    use rustix::fs::{Mode, OFlags, ResolveFlags, openat2};

    let destination_root = openat2(
        rustix::fs::CWD,
        &plan.destination_root,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-BACKUP-SOURCE-ROOT",
            &plan.destination_root,
            error,
        )
    })?;
    let transaction_root = openat2(
        rustix::fs::CWD,
        &plan.transaction_root,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-BACKUP-ROOT",
            &plan.transaction_root,
            error,
        )
    })?;
    let destination_root_identity = rustix::fs::fstat(&destination_root).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-BACKUP-SOURCE-ROOT-STAT",
            &plan.destination_root,
            error,
        )
    })?;
    let transaction_root_identity = rustix::fs::fstat(&transaction_root).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-BACKUP-ROOT-STAT",
            &plan.transaction_root,
            error,
        )
    })?;
    let source_descriptor = openat2(
        &destination_root,
        &entry.relative_path,
        OFlags::RDONLY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-BACKUP-SOURCE-OPEN",
            &plan.destination_root.join(&entry.relative_path),
            error,
        )
    })?;
    let mut source = File::from(source_descriptor);
    let mut bytes = Vec::new();
    source.read_to_end(&mut bytes).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-BACKUP-SOURCE-READ",
            &plan.destination_root.join(&entry.relative_path),
            error,
        )
    })?;
    if expected.is_none() || expected.is_some_and(|digest| sha256_bytes(&bytes) != digest) {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-BACKUP-BASELINE-DRIFT",
            "destination bytes changed before backup",
        ));
    }
    let relative = backup.strip_prefix(&plan.transaction_root).map_err(|_| {
        publication_error(
            ErrorClass::Policy,
            "GATE-PUBLICATION-BACKUP-CONFINEMENT",
            "backup is outside the transaction root",
        )
    })?;
    ensure_relative_parent_at(&transaction_root, relative, backup)?;
    let output_descriptor = openat2(
        &transaction_root,
        relative,
        OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::CLOEXEC,
        Mode::RUSR | Mode::WUSR,
        ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| io_error("GATE-PUBLICATION-BACKUP-CREATE", backup, error))?;
    let mut output = File::from(output_descriptor);
    output
        .write_all(&bytes)
        .and_then(|()| output.sync_all())
        .map_err(|error| io_error("GATE-PUBLICATION-BACKUP-WRITE", backup, error))?;
    require_unchanged_root(
        &plan.destination_root,
        destination_root_identity.st_dev,
        destination_root_identity.st_ino,
        "GATE-PUBLICATION-BACKUP-SOURCE-ROOT-RACE",
    )?;
    require_unchanged_root(
        &plan.transaction_root,
        transaction_root_identity.st_dev,
        transaction_root_identity.st_ino,
        "GATE-PUBLICATION-BACKUP-ROOT-RACE",
    )
}

#[cfg(not(target_os = "linux"))]
fn copy_destination_backup(
    _plan: &PublicationPlan,
    _entry: &PublicationEntry,
    _backup: &Path,
    _expected: Option<&str>,
) -> Result<()> {
    Err(publication_error(
        ErrorClass::Policy,
        "GATE-PUBLICATION-DESCRIPTOR-UNAVAILABLE",
        "descriptor-relative publication backups are required",
    ))
}

#[cfg(target_os = "linux")]
#[allow(
    clippy::too_many_lines,
    reason = "source descriptor acquisition, byte verification, root recheck, and staging form one auditable custody boundary"
)]
fn copy_source_to_stage(
    plan: &PublicationPlan,
    entry: &PublicationEntry,
    staged: &Path,
    before_root_recheck: impl FnOnce(),
) -> Result<()> {
    use rustix::fs::{Mode, OFlags, ResolveFlags, openat2};
    use std::os::unix::fs::MetadataExt;

    let source_root = openat2(
        rustix::fs::CWD,
        &plan.source_root,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-SOURCE-ROOT-OPEN",
            &plan.source_root,
            error,
        )
    })?;
    let source_root_identity = rustix::fs::fstat(&source_root).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-SOURCE-ROOT-STAT",
            &plan.source_root,
            error,
        )
    })?;
    let source_descriptor = openat2(
        &source_root,
        &entry.relative_path,
        OFlags::RDONLY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-SOURCE-OPEN",
            &plan.source_root.join(&entry.relative_path),
            error,
        )
    })?;
    let mut source = File::from(source_descriptor);
    let source_metadata = source.metadata().map_err(|error| {
        io_error(
            "GATE-PUBLICATION-SOURCE-STAT",
            &plan.source_root.join(&entry.relative_path),
            error,
        )
    })?;
    if !source_metadata.is_file() || source_metadata.nlink() != 1 {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-SOURCE-NONREGULAR",
            "publication source must be a singly linked regular file",
        ));
    }
    let mut bytes = Vec::new();
    source.read_to_end(&mut bytes).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-SOURCE-READ",
            &plan.source_root.join(&entry.relative_path),
            error,
        )
    })?;
    let after = source.metadata().map_err(|error| {
        io_error(
            "GATE-PUBLICATION-SOURCE-STAT",
            &plan.source_root.join(&entry.relative_path),
            error,
        )
    })?;
    if (
        source_metadata.dev(),
        source_metadata.ino(),
        source_metadata.len(),
    ) != (after.dev(), after.ino(), after.len())
        || source_metadata.modified().ok() != after.modified().ok()
        || sha256_bytes(&bytes) != entry.source_sha256
    {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-SOURCE-DRIFT",
            "publication source identity or bytes changed while reading",
        ));
    }

    before_root_recheck();

    let reopened_source_root = openat2(
        rustix::fs::CWD,
        &plan.source_root,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-SOURCE-ROOT-OPEN",
            &plan.source_root,
            error,
        )
    })?;
    let current_source_root = rustix::fs::fstat(&reopened_source_root).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-SOURCE-ROOT-STAT",
            &plan.source_root,
            error,
        )
    })?;
    if (source_root_identity.st_dev, source_root_identity.st_ino)
        != (current_source_root.st_dev, current_source_root.st_ino)
    {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-SOURCE-ROOT-RACE",
            "publication source root identity changed while reading",
        ));
    }

    let staged_relative = staged.strip_prefix(&plan.transaction_root).map_err(|_| {
        publication_error(
            ErrorClass::Policy,
            "GATE-PUBLICATION-STAGE-CONFINEMENT",
            "staged path is outside the transaction root",
        )
    })?;
    let transaction_root = openat2(
        rustix::fs::CWD,
        &plan.transaction_root,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-STAGE-ROOT-OPEN",
            &plan.transaction_root,
            error,
        )
    })?;
    let transaction_root_identity = rustix::fs::fstat(&transaction_root).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-STAGE-ROOT-STAT",
            &plan.transaction_root,
            error,
        )
    })?;
    ensure_relative_parent_at(&transaction_root, staged_relative, staged)?;
    let mut output = File::from(
        openat2(
            &transaction_root,
            staged_relative,
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::CLOEXEC,
            Mode::RUSR | Mode::WUSR,
            ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
        )
        .map_err(|error| io_error("GATE-PUBLICATION-STAGE-CREATE", staged, error))?,
    );
    output
        .write_all(&bytes)
        .and_then(|()| output.sync_all())
        .map_err(|error| io_error("GATE-PUBLICATION-STAGE-WRITE", staged, error))?;
    require_unchanged_root(
        &plan.transaction_root,
        transaction_root_identity.st_dev,
        transaction_root_identity.st_ino,
        "GATE-PUBLICATION-STAGE-ROOT-RACE",
    )
}

#[cfg(target_os = "linux")]
fn require_unchanged_root(
    path: &Path,
    expected_device: u64,
    expected_inode: u64,
    code: &'static str,
) -> Result<()> {
    use rustix::fs::{Mode, OFlags, ResolveFlags, openat2};

    let reopened = openat2(
        rustix::fs::CWD,
        path,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| io_error(code, path, error))?;
    let current = rustix::fs::fstat(&reopened).map_err(|error| io_error(code, path, error))?;
    if (expected_device, expected_inode) == (current.st_dev, current.st_ino) {
        Ok(())
    } else {
        Err(publication_error(
            ErrorClass::Identity,
            code,
            format!("root identity changed: {}", path.display()),
        ))
    }
}

#[cfg(target_os = "linux")]
fn ensure_relative_parent_at(
    root: &rustix::fd::OwnedFd,
    relative: &Path,
    display_path: &Path,
) -> Result<()> {
    use rustix::fs::{Mode, OFlags, ResolveFlags, mkdirat, openat2};

    let parent = relative.parent().ok_or_else(|| {
        publication_error(
            ErrorClass::Schema,
            "GATE-PUBLICATION-RELATIVE-PARENT",
            "transaction-relative path has no parent",
        )
    })?;
    let mut cumulative = PathBuf::new();
    for component in parent.components() {
        cumulative.push(component);
        match mkdirat(root, &cumulative, Mode::RUSR | Mode::WUSR | Mode::XUSR) {
            Ok(()) => {}
            Err(error) if error == rustix::io::Errno::EXIST => {}
            Err(error) => {
                return Err(io_error(
                    "GATE-PUBLICATION-TRANSACTION-MKDIR",
                    display_path,
                    error,
                ));
            }
        }
        openat2(
            root,
            &cumulative,
            OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
            ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
        )
        .map_err(|error| io_error("GATE-PUBLICATION-TRANSACTION-DIR-OPEN", display_path, error))?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn open_parent_beneath(root: &rustix::fd::OwnedFd, relative: &Path) -> Result<rustix::fd::OwnedFd> {
    use rustix::fs::{Mode, OFlags, ResolveFlags, openat2};

    let parent = relative.parent().unwrap_or_else(|| Path::new("."));
    openat2(
        root,
        parent,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        publication_error(
            ErrorClass::Io,
            "GATE-PUBLICATION-RECOVERY-PARENT-OPEN",
            format!("{}: {error}", relative.display()),
        )
    })
}

#[cfg(not(target_os = "linux"))]
fn copy_source_to_stage(
    _plan: &PublicationPlan,
    _entry: &PublicationEntry,
    _staged: &Path,
    _before_root_recheck: impl FnOnce(),
) -> Result<()> {
    Err(publication_error(
        ErrorClass::Policy,
        "GATE-PUBLICATION-DESCRIPTOR-UNAVAILABLE",
        "descriptor-relative publication source reads are required",
    ))
}

#[cfg(target_os = "linux")]
#[allow(
    clippy::too_many_lines,
    reason = "the install boundary intentionally keeps descriptor acquisition, identity rechecks, and the single rename in one auditable scope"
)]
fn install_staged_destination(
    plan: &PublicationPlan,
    entry: &PublicationEntry,
    staged: &Path,
    before_install: impl FnOnce(),
) -> Result<()> {
    use rustix::fd::OwnedFd;
    use rustix::fs::{
        AtFlags, Mode, OFlags, RenameFlags, ResolveFlags, openat2, renameat, renameat_with, statat,
    };
    use std::os::unix::fs::MetadataExt;

    fn open_root(path: &Path) -> Result<OwnedFd> {
        openat2(
            rustix::fs::CWD,
            path,
            OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
            ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
        )
        .map_err(|error| io_error("GATE-PUBLICATION-ROOT-OPEN", path, error))
    }

    fn open_parent(root: &OwnedFd, relative: &Path) -> Result<OwnedFd> {
        let parent = relative.parent().unwrap_or_else(|| Path::new("."));
        openat2(
            root,
            parent,
            OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
            ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
        )
        .map_err(|error| {
            publication_error(
                ErrorClass::Io,
                "GATE-PUBLICATION-PARENT-OPEN",
                format!("{}: {error}", relative.display()),
            )
        })
    }

    let destination_root = open_root(&plan.destination_root)?;
    let transaction_root = open_root(&plan.transaction_root)?;
    let destination_parent = open_parent(&destination_root, &entry.relative_path)?;
    let staged_relative = staged.strip_prefix(&plan.transaction_root).map_err(|_| {
        publication_error(
            ErrorClass::Policy,
            "GATE-PUBLICATION-STAGE-CONFINEMENT",
            "staged path is outside the transaction root",
        )
    })?;
    let staged_parent = open_parent(&transaction_root, staged_relative)?;
    let destination_name = entry.relative_path.file_name().ok_or_else(|| {
        publication_error(
            ErrorClass::Schema,
            "GATE-PUBLICATION-PATH",
            "publication destination has no file name",
        )
    })?;
    let staged_name = staged_relative.file_name().ok_or_else(|| {
        publication_error(
            ErrorClass::Schema,
            "GATE-PUBLICATION-PATH",
            "publication staged path has no file name",
        )
    })?;
    let staged_descriptor = openat2(
        &staged_parent,
        staged_name,
        OFlags::RDONLY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| io_error("GATE-PUBLICATION-STAGE-OPEN", staged, error))?;
    let mut staged_file = File::from(staged_descriptor);
    let staged_metadata = staged_file
        .metadata()
        .map_err(|error| io_error("GATE-PUBLICATION-STAGE-STAT", staged, error))?;
    if !staged_metadata.is_file() || staged_metadata.nlink() != 1 {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-STAGE-NONREGULAR",
            "staged publication source must be a singly linked regular file",
        ));
    }
    let mut staged_bytes = Vec::new();
    staged_file
        .read_to_end(&mut staged_bytes)
        .map_err(|error| io_error("GATE-PUBLICATION-STAGE-READ", staged, error))?;
    if sha256_bytes(&staged_bytes) != entry.source_sha256 {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-STAGE-DRIFT",
            "staged publication bytes differ from the source authority",
        ));
    }

    // Capture the exact directory objects and destination inode immediately
    // before the only mutating syscall. The pathname is reopened after the
    // test hook so replacement of either root cannot redirect or detach the
    // descriptor-relative rename.
    let destination_root_identity = rustix::fs::fstat(&destination_root)
        .map_err(|error| io_error("GATE-PUBLICATION-ROOT-STAT", &plan.destination_root, error))?;
    let transaction_root_identity = rustix::fs::fstat(&transaction_root)
        .map_err(|error| io_error("GATE-PUBLICATION-ROOT-STAT", &plan.transaction_root, error))?;
    let baseline_identity = match &entry.destination_baseline_sha256 {
        Some(expected) => {
            let descriptor = openat2(
                &destination_parent,
                destination_name,
                OFlags::RDONLY | OFlags::CLOEXEC,
                Mode::empty(),
                ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
            )
            .map_err(|error| {
                io_error(
                    "GATE-PUBLICATION-BASELINE-OPEN",
                    &plan.destination_root.join(&entry.relative_path),
                    error,
                )
            })?;
            let mut file = File::from(descriptor);
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).map_err(|error| {
                io_error(
                    "GATE-PUBLICATION-BASELINE-READ",
                    &plan.destination_root.join(&entry.relative_path),
                    error,
                )
            })?;
            if sha256_bytes(&bytes) != *expected {
                return Err(publication_error(
                    ErrorClass::Identity,
                    "GATE-PUBLICATION-BASELINE-RACE",
                    "destination baseline changed before installation",
                ));
            }
            Some(file.metadata().map_err(|error| {
                io_error(
                    "GATE-PUBLICATION-BASELINE-STAT",
                    &plan.destination_root.join(&entry.relative_path),
                    error,
                )
            })?)
        }
        None => None,
    };

    before_install();

    let reopened_destination_root = open_root(&plan.destination_root)?;
    let reopened_transaction_root = open_root(&plan.transaction_root)?;
    let current_destination_root = rustix::fs::fstat(&reopened_destination_root)
        .map_err(|error| io_error("GATE-PUBLICATION-ROOT-STAT", &plan.destination_root, error))?;
    let current_transaction_root = rustix::fs::fstat(&reopened_transaction_root)
        .map_err(|error| io_error("GATE-PUBLICATION-ROOT-STAT", &plan.transaction_root, error))?;
    if (
        destination_root_identity.st_dev,
        destination_root_identity.st_ino,
    ) != (
        current_destination_root.st_dev,
        current_destination_root.st_ino,
    ) || (
        transaction_root_identity.st_dev,
        transaction_root_identity.st_ino,
    ) != (
        current_transaction_root.st_dev,
        current_transaction_root.st_ino,
    ) {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-ROOT-RACE",
            "publication root identity changed before installation",
        ));
    }

    if let Some(baseline) = baseline_identity {
        let current = statat(
            &destination_parent,
            destination_name,
            AtFlags::SYMLINK_NOFOLLOW,
        )
        .map_err(|error| {
            io_error(
                "GATE-PUBLICATION-BASELINE-RECHECK",
                &plan.destination_root.join(&entry.relative_path),
                error,
            )
        })?;
        if (baseline.dev(), baseline.ino()) != (current.st_dev, current.st_ino) {
            return Err(publication_error(
                ErrorClass::Identity,
                "GATE-PUBLICATION-BASELINE-RACE",
                "destination inode changed before installation",
            ));
        }
        let current_staged = statat(&staged_parent, staged_name, AtFlags::SYMLINK_NOFOLLOW)
            .map_err(|error| io_error("GATE-PUBLICATION-STAGE-RECHECK", staged, error))?;
        if (staged_metadata.dev(), staged_metadata.ino())
            != (current_staged.st_dev, current_staged.st_ino)
        {
            return Err(publication_error(
                ErrorClass::Identity,
                "GATE-PUBLICATION-STAGE-RACE",
                "staged publication inode changed before installation",
            ));
        }
        renameat(
            &staged_parent,
            staged_name,
            &destination_parent,
            destination_name,
        )
        .map_err(|error| {
            io_error(
                "GATE-PUBLICATION-INSTALL-RENAME",
                &plan.destination_root.join(&entry.relative_path),
                error,
            )
        })
    } else {
        let current_staged = statat(&staged_parent, staged_name, AtFlags::SYMLINK_NOFOLLOW)
            .map_err(|error| io_error("GATE-PUBLICATION-STAGE-RECHECK", staged, error))?;
        if (staged_metadata.dev(), staged_metadata.ino())
            != (current_staged.st_dev, current_staged.st_ino)
        {
            return Err(publication_error(
                ErrorClass::Identity,
                "GATE-PUBLICATION-STAGE-RACE",
                "staged publication inode changed before installation",
            ));
        }
        renameat_with(
            &staged_parent,
            staged_name,
            &destination_parent,
            destination_name,
            RenameFlags::NOREPLACE,
        )
        .map_err(|error| {
            io_error(
                "GATE-PUBLICATION-INSTALL-NOREPLACE",
                &plan.destination_root.join(&entry.relative_path),
                error,
            )
        })
    }
}

#[cfg(not(target_os = "linux"))]
fn install_staged_destination(
    _plan: &PublicationPlan,
    _entry: &PublicationEntry,
    _staged: &Path,
    _before_install: impl FnOnce(),
) -> Result<()> {
    Err(publication_error(
        ErrorClass::Policy,
        "GATE-PUBLICATION-DESCRIPTOR-UNAVAILABLE",
        "descriptor-relative publication is required",
    ))
}

fn restore_entry(plan: &PublicationPlan, record: &PublicationJournalRecord) -> Result<()> {
    let destination = confined_join(&plan.destination_root, &record.relative_path)?;
    reject_symlink_or_special_if_present(&destination)?;
    match (
        &record.destination_baseline_sha256,
        &record.backup_relative_path,
    ) {
        (Some(expected), Some(relative)) => {
            let staged = confined_join(
                &plan.transaction_root.join("restore-staged"),
                &record.relative_path,
            )?;
            copy_transaction_file_to_stage(plan, relative, &staged, expected)?;
            append_restore_record(plan, record)?;
            let restore_entry = PublicationEntry {
                relative_path: record.relative_path.clone(),
                source_sha256: expected.clone(),
                destination_baseline_sha256: Some(record.source_sha256.clone()),
            };
            install_staged_destination(plan, &restore_entry, &staged, || {})?;
            sync_parent(&destination)?;
            require_hash(&destination, expected)
        }
        (None, None) => {
            append_restore_record(plan, record)?;
            remove_installed_descriptor_relative(plan, record)
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

#[cfg(target_os = "linux")]
fn copy_transaction_file_to_stage(
    plan: &PublicationPlan,
    source_relative: &Path,
    staged: &Path,
    expected: &str,
) -> Result<()> {
    use rustix::fs::{Mode, OFlags, ResolveFlags, openat2};

    let root = openat2(
        rustix::fs::CWD,
        &plan.transaction_root,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-RECOVERY-ROOT-OPEN",
            &plan.transaction_root,
            error,
        )
    })?;
    let identity = rustix::fs::fstat(&root).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-RECOVERY-ROOT-STAT",
            &plan.transaction_root,
            error,
        )
    })?;
    let mut source = File::from(
        openat2(
            &root,
            source_relative,
            OFlags::RDONLY | OFlags::CLOEXEC,
            Mode::empty(),
            ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
        )
        .map_err(|error| {
            io_error(
                "GATE-PUBLICATION-RECOVERY-BACKUP-OPEN",
                &plan.transaction_root.join(source_relative),
                error,
            )
        })?,
    );
    let metadata = source.metadata().map_err(|error| {
        io_error(
            "GATE-PUBLICATION-RECOVERY-BACKUP-STAT",
            &plan.transaction_root.join(source_relative),
            error,
        )
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if !metadata.is_file() || metadata.nlink() != 1 {
            return Err(publication_error(
                ErrorClass::Identity,
                "GATE-PUBLICATION-RECOVERY-BACKUP-TYPE",
                "recovery backup must be a singly linked regular file",
            ));
        }
    }
    let mut bytes = Vec::new();
    source.read_to_end(&mut bytes).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-RECOVERY-BACKUP-READ",
            &plan.transaction_root.join(source_relative),
            error,
        )
    })?;
    if sha256_bytes(&bytes) != expected {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-RECOVERY-BACKUP-DRIFT",
            "recovery backup bytes differ from the journal baseline",
        ));
    }
    let staged_relative = staged.strip_prefix(&plan.transaction_root).map_err(|_| {
        publication_error(
            ErrorClass::Policy,
            "GATE-PUBLICATION-RECOVERY-STAGE-CONFINEMENT",
            "recovery stage is outside the transaction root",
        )
    })?;
    ensure_relative_parent_at(&root, staged_relative, staged)?;
    let mut output = File::from(
        openat2(
            &root,
            staged_relative,
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::CLOEXEC,
            Mode::RUSR | Mode::WUSR,
            ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
        )
        .map_err(|error| io_error("GATE-PUBLICATION-RECOVERY-STAGE-CREATE", staged, error))?,
    );
    output
        .write_all(&bytes)
        .and_then(|()| output.sync_all())
        .map_err(|error| io_error("GATE-PUBLICATION-RECOVERY-STAGE-WRITE", staged, error))?;
    require_unchanged_root(
        &plan.transaction_root,
        identity.st_dev,
        identity.st_ino,
        "GATE-PUBLICATION-RECOVERY-ROOT-RACE",
    )
}

#[cfg(not(target_os = "linux"))]
fn copy_transaction_file_to_stage(
    _plan: &PublicationPlan,
    _source_relative: &Path,
    _staged: &Path,
    _expected: &str,
) -> Result<()> {
    Err(publication_error(
        ErrorClass::Policy,
        "GATE-PUBLICATION-DESCRIPTOR-UNAVAILABLE",
        "descriptor-relative recovery is required",
    ))
}

#[cfg(target_os = "linux")]
fn remove_installed_descriptor_relative(
    plan: &PublicationPlan,
    record: &PublicationJournalRecord,
) -> Result<()> {
    remove_installed_descriptor_relative_with_hook(plan, record, || {})
}

#[cfg(target_os = "linux")]
fn remove_installed_descriptor_relative_with_hook(
    plan: &PublicationPlan,
    record: &PublicationJournalRecord,
    before_recheck: impl FnOnce(),
) -> Result<()> {
    use rustix::fs::{AtFlags, Mode, OFlags, ResolveFlags, openat2, statat, unlinkat};
    use std::os::unix::fs::MetadataExt;

    let root = openat2(
        rustix::fs::CWD,
        &plan.destination_root,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-RECOVERY-DESTINATION-OPEN",
            &plan.destination_root,
            error,
        )
    })?;
    let identity = rustix::fs::fstat(&root).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-RECOVERY-DESTINATION-STAT",
            &plan.destination_root,
            error,
        )
    })?;
    let parent = open_parent_beneath(&root, &record.relative_path)?;
    let name = record.relative_path.file_name().ok_or_else(|| {
        publication_error(
            ErrorClass::Schema,
            "GATE-PUBLICATION-PATH",
            "missing recovery file name",
        )
    })?;
    let mut file = File::from(
        openat2(
            &parent,
            name,
            OFlags::RDONLY | OFlags::CLOEXEC,
            Mode::empty(),
            ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
        )
        .map_err(|error| {
            io_error(
                "GATE-PUBLICATION-RECOVERY-DESTINATION-FILE",
                &plan.destination_root.join(&record.relative_path),
                error,
            )
        })?,
    );
    let metadata = file.metadata().map_err(|error| {
        io_error(
            "GATE-PUBLICATION-RECOVERY-DESTINATION-STAT",
            &plan.destination_root.join(&record.relative_path),
            error,
        )
    })?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-RECOVERY-DESTINATION-READ",
            &plan.destination_root.join(&record.relative_path),
            error,
        )
    })?;
    if !metadata.is_file() || metadata.nlink() != 1 || sha256_bytes(&bytes) != record.source_sha256
    {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-RECOVERY-DESTINATION-DRIFT",
            "installed destination differs from the journaled source",
        ));
    }
    before_recheck();
    let current = statat(&parent, name, AtFlags::SYMLINK_NOFOLLOW).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-RECOVERY-DESTINATION-RECHECK",
            &plan.destination_root.join(&record.relative_path),
            error,
        )
    })?;
    if (metadata.dev(), metadata.ino()) != (current.st_dev, current.st_ino) {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-RECOVERY-DESTINATION-RACE",
            "installed destination inode changed before recovery delete",
        ));
    }
    require_unchanged_root(
        &plan.destination_root,
        identity.st_dev,
        identity.st_ino,
        "GATE-PUBLICATION-RECOVERY-DESTINATION-ROOT-RACE",
    )?;
    unlinkat(&parent, name, AtFlags::empty()).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-RESTORE-REMOVE",
            &plan.destination_root.join(&record.relative_path),
            error,
        )
    })
}

#[cfg(not(target_os = "linux"))]
fn remove_installed_descriptor_relative(
    _plan: &PublicationPlan,
    _record: &PublicationJournalRecord,
) -> Result<()> {
    Err(publication_error(
        ErrorClass::Policy,
        "GATE-PUBLICATION-DESCRIPTOR-UNAVAILABLE",
        "descriptor-relative recovery is required",
    ))
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
    for path in [&plan.external_plan_path, &plan.transaction_receipt_path] {
        if !path.is_absolute() {
            return Err(publication_error(
                ErrorClass::Schema,
                "GATE-PUBLICATION-AUTHORITY-PATH",
                "external plan and transaction receipt paths must be absolute",
            ));
        }
    }
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

fn verify_producing_transaction(plan: &PublicationPlan) -> Result<()> {
    reject_symlink_or_special_if_present(&plan.external_plan_path)?;
    reject_symlink_or_special_if_present(&plan.transaction_receipt_path)?;
    let receipt_bytes = fs::read(&plan.transaction_receipt_path).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-TRANSACTION-RECEIPT-READ",
            &plan.transaction_receipt_path,
            error,
        )
    })?;
    if sha256_bytes(&receipt_bytes) != plan.transaction_receipt_sha256 {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-TRANSACTION-RECEIPT-SHA",
            "producing transaction receipt bytes differ from the publication plan",
        ));
    }
    let receipt = parse_strict(&receipt_bytes)?;
    verify_external_transaction(&plan.external_plan_path, &receipt)?;
    if receipt["receipt_id"].as_str() != Some(plan.transaction_receipt_id.as_str()) {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-TRANSACTION-RECEIPT-ID",
            "producing transaction receipt identity differs from the publication plan",
        ));
    }
    let final_manifest = receipt["heavy"]
        .as_array()
        .and_then(|receipts| receipts.last())
        .and_then(|node| node.get("output_manifest"))
        .ok_or_else(|| {
            publication_error(
                ErrorClass::Receipt,
                "GATE-PUBLICATION-TRANSACTION-MANIFEST",
                "producing transaction has no terminal output manifest",
            )
        })?;
    if final_manifest["root"].as_str() != Some(plan.source_root.to_string_lossy().as_ref())
        || final_manifest["manifest_id"].as_str() != Some(plan.source_manifest_id.as_str())
        || digest(final_manifest)? != plan.source_manifest_sha256
    {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-TRANSACTION-MANIFEST",
            "publication source does not correspond to the verified producing manifest",
        ));
    }
    Ok(())
}

fn require_absolute_distinct_roots(plan: &PublicationPlan) -> Result<()> {
    let roots = [
        &plan.source_root,
        &plan.destination_root,
        &plan.transaction_root,
    ];
    if roots.iter().any(|root| {
        !root.is_absolute()
            || root
                .components()
                .any(|component| matches!(component, Component::ParentDir | Component::CurDir))
    }) {
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
    for path in [
        &plan.external_plan_path,
        &plan.transaction_receipt_path,
        &plan.journal_path,
        &plan.receipt_path,
    ] {
        if !path.is_absolute()
            || path
                .components()
                .any(|component| matches!(component, Component::ParentDir | Component::CurDir))
        {
            return Err(publication_error(
                ErrorClass::Schema,
                "GATE-PUBLICATION-LEXICAL-PATH",
                "authority, journal, and receipt paths must be normalized absolute paths",
            ));
        }
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
    #[cfg(target_os = "linux")]
    {
        append_journal_descriptor_relative(plan, record)
    }
    #[cfg(not(target_os = "linux"))]
    {
        append_journal_portable(plan, record)
    }
}

#[cfg(target_os = "linux")]
fn append_journal_descriptor_relative(
    plan: &PublicationPlan,
    record: &PublicationJournalRecord,
) -> Result<()> {
    append_journal_descriptor_relative_with_hook(plan, record, || {})
}

#[cfg(target_os = "linux")]
fn append_journal_descriptor_relative_with_hook(
    plan: &PublicationPlan,
    record: &PublicationJournalRecord,
    before_root_recheck: impl FnOnce(),
) -> Result<()> {
    use rustix::fs::{Mode, OFlags, ResolveFlags, openat2};

    let mut bytes = canonical_bytes(&serde_json::to_value(record).map_err(|error| {
        publication_error(
            ErrorClass::Json,
            "GATE-PUBLICATION-JOURNAL-JSON",
            error.to_string(),
        )
    })?)?;
    bytes.push(b'\n');
    let relative = plan
        .journal_path
        .strip_prefix(&plan.transaction_root)
        .map_err(|_| {
            publication_error(
                ErrorClass::Policy,
                "GATE-PUBLICATION-JOURNAL-CONFINEMENT",
                "journal is outside the transaction root",
            )
        })?;
    let root = openat2(
        rustix::fs::CWD,
        &plan.transaction_root,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-TRANSACTION-ROOT-OPEN",
            &plan.transaction_root,
            error,
        )
    })?;
    let root_identity = rustix::fs::fstat(&root).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-TRANSACTION-ROOT-STAT",
            &plan.transaction_root,
            error,
        )
    })?;
    let descriptor = openat2(
        &root,
        relative,
        OFlags::WRONLY | OFlags::APPEND | OFlags::CREATE | OFlags::CLOEXEC,
        Mode::RUSR | Mode::WUSR,
        ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| io_error("GATE-PUBLICATION-JOURNAL-OPEN", &plan.journal_path, error))?;
    let mut file = File::from(descriptor);
    file.write_all(&bytes)
        .and_then(|()| file.sync_all())
        .map_err(|error| io_error("GATE-PUBLICATION-JOURNAL-WRITE", &plan.journal_path, error))?;
    before_root_recheck();
    let reopened = openat2(
        rustix::fs::CWD,
        &plan.transaction_root,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        io_error(
            "GATE-PUBLICATION-TRANSACTION-ROOT-REOPEN",
            &plan.transaction_root,
            error,
        )
    })?;
    let current = rustix::fs::fstat(&reopened).map_err(|error| {
        io_error(
            "GATE-PUBLICATION-TRANSACTION-ROOT-STAT",
            &plan.transaction_root,
            error,
        )
    })?;
    if (root_identity.st_dev, root_identity.st_ino) != (current.st_dev, current.st_ino) {
        return Err(publication_error(
            ErrorClass::Identity,
            "GATE-PUBLICATION-TRANSACTION-ROOT-RACE",
            "transaction root changed during journal append",
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn append_journal_portable(
    plan: &PublicationPlan,
    record: &PublicationJournalRecord,
) -> Result<()> {
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
        external_plan_path: plan.external_plan_path.clone(),
        transaction_receipt_path: plan.transaction_receipt_path.clone(),
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

#[cfg(test)]
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
    use crate::executor::ExecutionClaims;
    use crate::external_dag::{ExternalTransitionOptions, run_external_transition};

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

    fn git(repo: &Path, arguments: &[&str]) -> String {
        let output = std::process::Command::new("git")
            .args(arguments)
            .current_dir(repo)
            .output()
            .expect("test git command");
        assert!(
            output.status.success(),
            "git {arguments:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout)
            .expect("git UTF-8")
            .trim()
            .to_owned()
    }

    #[allow(
        clippy::too_many_lines,
        reason = "fixture constructs a complete authenticated publication authority"
    )]
    fn plan(root: &Path, destination_baseline: Option<&[u8]>) -> PublicationPlan {
        let source = root.join("source");
        let destination = root.join("destination");
        if source.exists() {
            fs::remove_dir_all(&source).expect("replace publication source root");
        }
        let transaction_receipt_path = root.join("transaction-receipt.json");
        for receipt in [
            transaction_receipt_path.clone(),
            transaction_receipt_path.with_extension("light.json"),
            transaction_receipt_path.with_extension("audit.json"),
        ] {
            if let Err(error) = fs::remove_file(receipt) {
                assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
            }
        }
        if let Some(bytes) = destination_baseline {
            write(&destination.join("objects/artifacts/result.json"), bytes);
        } else {
            fs::create_dir_all(&destination).expect("destination root");
        }
        let authority_repo = root.join("authority-repo");
        if authority_repo.exists() {
            fs::remove_dir_all(&authority_repo).expect("replace authority repository");
        }
        fs::create_dir_all(&authority_repo).expect("authority repository");
        git(&authority_repo, &["init", "--quiet"]);
        git(&authority_repo, &["config", "user.name", "openWEPP test"]);
        git(
            &authority_repo,
            &["config", "user.email", "openwepp-test@example.invalid"],
        );
        let package_path = "docs/work-packages/publication-fixture/package.md";
        write(
            &authority_repo.join(package_path),
            b"# Publication fixture\n\nStatus: `ACTIVE`\n\n## Intended Write Set\n\n- `evidence/**`\n- `plan/**`\n- `tools/**`\n",
        );
        write(
            &authority_repo.join("gate-policy/v1/schemas/package-audit.schema.json"),
            include_bytes!("../../../gate-policy/v1/schemas/package-audit.schema.json"),
        );
        git(&authority_repo, &["add", "."]);
        git(
            &authority_repo,
            &["commit", "--quiet", "-m", "publication fixture scaffold"],
        );
        let base_commit = git(&authority_repo, &["rev-parse", "HEAD"]);
        write(
            &authority_repo.join("evidence/cheap.txt"),
            b"focused gates passed\n",
        );
        write(
            &authority_repo.join("tools/light.sh"),
            b"#!/bin/sh\nset -eu\nmkdir -p \"$(dirname \"$1\")\"\nprintf 'new-result' > \"$1\"\n",
        );
        write(
            &authority_repo.join("tools/heavy.sh"),
            b"#!/bin/sh\nset -eu\ntest \"$(cat \"$1\")\" = new-result\n",
        );
        #[cfg(unix)]
        for script in ["tools/light.sh", "tools/heavy.sh"] {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(
                authority_repo.join(script),
                fs::Permissions::from_mode(0o755),
            )
            .expect("executable fixture script");
        }
        let source_csv = format!(
            "order,command_id,source_path,argv,environment,working_directory,inputs,outputs,harvard_access,cost_class\n\
             1,light,tools/light.sh,tools/light.sh ${{OBJECTS_ROOT}}/artifacts/result.json,default,{},seed,objects/artifacts/result.json,FORBIDDEN,QUICK\n\
             2,heavy,tools/heavy.sh,tools/heavy.sh ${{OBJECTS_ROOT}}/artifacts/result.json,default,{},light output,,FORBIDDEN,HEAVY\n",
            authority_repo.display(),
            authority_repo.display(),
        );
        write(
            &authority_repo.join("plan/source.csv"),
            source_csv.as_bytes(),
        );
        write(
            &authority_repo.join("plan/contract.csv"),
            b"command_id,prerequisites,receipt_outputs\nlight,-,objects/artifacts/result.json\nheavy,light,-\n",
        );
        let binding = |path: &str| {
            serde_json::json!({
                "path": path,
                "sha256": sha256_bytes(&fs::read(authority_repo.join(path)).expect("binding bytes"))
            })
        };
        let external_plan_path = authority_repo.join("plan/external.json");
        let mut external_plan = serde_json::json!({
            "schema": "openwepp-external-dag-plan-v1",
            "plan_id": "0".repeat(64),
            "generation": "A",
            "parent_plan": null,
            "source_identity": null,
            "source_plan": binding("plan/source.csv"),
            "source_contract": binding("plan/contract.csv"),
            "authority": {
                "package_path": package_path,
                "base_commit": base_commit,
                "cheap_gate_evidence": [binding("evidence/cheap.txt")]
            },
            "transactions": [{
                "transaction_id": "publication-source",
                "light": [{
                    "order": 1,
                    "command_id": "light",
                    "argv": ["${REPO}/tools/light.sh", "${OBJECTS_ROOT}/artifacts/result.json"],
                    "env": {},
                    "cwd": authority_repo.display().to_string(),
                    "source_working_directory": authority_repo.display().to_string(),
                    "source_inputs": ["seed"],
                    "prerequisites": [],
                    "cost_class": "QUICK",
                    "source_path": "tools/light.sh",
                    "declared_outputs": ["objects/artifacts/result.json"],
                    "timeout_seconds": 10,
                    "max_attempts": 1,
                    "handoff": "READY audit",
                    "harvard_access": "NONE"
                }],
                "heavy": [{
                    "order": 2,
                    "command_id": "heavy",
                    "argv": ["${REPO}/tools/heavy.sh", "${OBJECTS_ROOT}/artifacts/result.json"],
                    "env": {},
                    "cwd": authority_repo.display().to_string(),
                    "source_working_directory": authority_repo.display().to_string(),
                    "source_inputs": ["light output"],
                    "prerequisites": ["light"],
                    "cost_class": "HEAVY",
                    "source_path": "tools/heavy.sh",
                    "declared_outputs": [],
                    "timeout_seconds": 10,
                    "max_attempts": 1,
                    "handoff": "terminal receipt",
                    "harvard_access": "NONE"
                }],
                "custody_prerequisites": [],
                "custody_receipts": []
            }],
            "custody_commands": []
        });
        external_plan["plan_id"] =
            serde_json::Value::String(derived_id(&external_plan, "plan_id").expect("plan ID"));
        write(
            &external_plan_path,
            &canonical_bytes(&external_plan).expect("canonical external plan"),
        );
        git(&authority_repo, &["add", "."]);
        git(
            &authority_repo,
            &["commit", "--quiet", "-m", "bind publication transaction"],
        );
        let ledger = std::env::current_dir()
            .expect("publication fixture cwd")
            .join("target/publication-fixture-ledgers")
            .join(root.file_name().expect("publication fixture root name"))
            .join("execution-ledger.jsonl");
        write(&ledger, b"");
        let transaction_receipt = run_external_transition(&ExternalTransitionOptions {
            repo: authority_repo.clone(),
            plan_path: external_plan_path.clone(),
            transaction_id: "publication-source".to_owned(),
            attempt_root: source.clone(),
            ledger,
            receipt_path: transaction_receipt_path.clone(),
            custody_root: None,
            opening_token: None,
            claims: ExecutionClaims {
                principal: "publication-test".to_owned(),
                repository: "local/publication".to_owned(),
                source_event: "test".to_owned(),
                source_ref: "refs/heads/main".to_owned(),
                workflow: "publication-fixture".to_owned(),
                job: "light-ready-heavy".to_owned(),
                runner: "local-test".to_owned(),
                attempt: 1,
            },
        })
        .expect("execute authenticated publication source transaction");
        let transaction_receipt_bytes =
            fs::read(&transaction_receipt_path).expect("persisted transaction receipt");
        let source_manifest =
            manifest_declared_outputs(&source, &[PathBuf::from("objects/artifacts/result.json")])
                .expect("source manifest");
        let source_manifest_sha256 =
            digest(&serde_json::to_value(&source_manifest).expect("source manifest JSON"))
                .expect("source manifest digest");
        PublicationPlan {
            publication_id: "publication-1".to_owned(),
            external_plan_path,
            transaction_receipt_path,
            transaction_receipt_id: transaction_receipt["receipt_id"]
                .as_str()
                .expect("receipt ID string")
                .to_owned(),
            transaction_receipt_sha256: sha256_bytes(&transaction_receipt_bytes),
            source_manifest_id: source_manifest.manifest_id,
            source_manifest_sha256,
            destination_baseline_sha256: destination_baseline_digest(&[PublicationEntry {
                relative_path: PathBuf::from("objects/artifacts/result.json"),
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
                relative_path: PathBuf::from("objects/artifacts/result.json"),
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
            fs::read(plan.destination_root.join("objects/artifacts/result.json"),)
                .expect("installed"),
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
            publication_plan
                .source_root
                .join("objects/artifacts/result.json"),
            b"source drift",
        )
        .expect("source drift");
        assert_eq!(
            publish(&publication_plan).expect_err("drift rejected").code,
            "GATE-EXTERNAL-MANIFEST-HISTORICAL-DRIFT"
        );
        assert!(!publication_plan.journal_path.exists());

        remove_scratch(&root);
        publication_plan = plan(&root, None);
        write(
            &publication_plan
                .destination_root
                .join("objects/artifacts/result.json"),
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
    fn rejects_forged_transaction_claims_before_publication_mutation() {
        let root = scratch("forged-transaction");
        remove_scratch(&root);
        let mut publication_plan = plan(&root, None);
        publication_plan.transaction_receipt_id = "a".repeat(64);
        assert_eq!(
            publish(&publication_plan)
                .expect_err("caller-supplied transaction identity must not be trusted")
                .code,
            "GATE-PUBLICATION-TRANSACTION-RECEIPT-ID"
        );
        assert!(!publication_plan.journal_path.exists());

        publication_plan = plan(&root, None);
        fs::write(&publication_plan.transaction_receipt_path, b"{}\n")
            .expect("replace receipt bytes");
        assert_eq!(
            publish(&publication_plan)
                .expect_err("exact producing receipt bytes must be bound")
                .code,
            "GATE-PUBLICATION-TRANSACTION-RECEIPT-SHA"
        );
        assert!(!publication_plan.journal_path.exists());
        remove_scratch(&root);
    }

    #[test]
    fn rejects_valid_receipt_bound_to_a_different_source_manifest() {
        let root = scratch("foreign-manifest");
        remove_scratch(&root);
        let mut publication_plan = plan(&root, None);
        publication_plan.source_manifest_id = "b".repeat(64);
        assert_eq!(
            publish(&publication_plan)
                .expect_err("publication must consume the producer's terminal manifest")
                .code,
            "GATE-PUBLICATION-TRANSACTION-MANIFEST"
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
            fs::read(plan.destination_root.join("objects/artifacts/result.json"),)
                .expect("restored"),
            b"old-result"
        );
        remove_scratch(&root);
    }

    #[test]
    fn recovery_completes_a_journaled_partial_transaction() {
        let root = scratch("complete");
        remove_scratch(&root);
        let plan = plan(&root, None);
        prepare_transaction_directories(&plan).expect("transaction dirs");
        install_entry(&plan, &plan.entries[0], 1, false).expect("first install");

        let outcome = recover(&plan, RecoveryAction::Complete).expect("complete");
        assert_eq!(outcome.status, PublicationStatus::Accepted);
        assert!(plan.receipt_path.exists());
        assert_eq!(
            fs::read(plan.destination_root.join("objects/artifacts/result.json"),).expect("result"),
            b"new-result"
        );
        remove_scratch(&root);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn rejects_destination_root_replacement_at_install_boundary_without_overwrite() {
        let root = scratch("root-install-race");
        remove_scratch(&root);
        let publication_plan = plan(&root, None);
        prepare_transaction_directories(&publication_plan).expect("transaction dirs");
        let entry = &publication_plan.entries[0];
        let source = publication_plan.source_root.join(&entry.relative_path);
        let staged = publication_plan
            .transaction_root
            .join("staged")
            .join(&entry.relative_path);
        copy_verified(&source, &staged, Some(&entry.source_sha256)).expect("stage source");
        ensure_parent(&publication_plan.destination_root.join(&entry.relative_path))
            .expect("destination parent");

        let displaced = root.join("destination-displaced");
        let replacement_file = publication_plan.destination_root.join(&entry.relative_path);
        let error = install_staged_destination(&publication_plan, entry, &staged, || {
            fs::rename(&publication_plan.destination_root, &displaced)
                .expect("displace destination root");
            write(&replacement_file, b"attacker-owned");
        })
        .expect_err("root replacement must reject");

        assert_eq!(error.code, "GATE-PUBLICATION-ROOT-RACE");
        assert_eq!(
            fs::read(&replacement_file).expect("replacement preserved"),
            b"attacker-owned"
        );
        assert!(staged.exists(), "staged source remains recoverable");
        assert!(
            !displaced.join(&entry.relative_path).exists(),
            "detached verified root was not mutated"
        );
        remove_scratch(&root);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn rejects_destination_inode_replacement_at_install_boundary_without_overwrite() {
        let root = scratch("destination-install-race");
        remove_scratch(&root);
        let publication_plan = plan(&root, Some(b"old-result"));
        prepare_transaction_directories(&publication_plan).expect("transaction dirs");
        let entry = &publication_plan.entries[0];
        let source = publication_plan.source_root.join(&entry.relative_path);
        let staged = publication_plan
            .transaction_root
            .join("staged")
            .join(&entry.relative_path);
        copy_verified(&source, &staged, Some(&entry.source_sha256)).expect("stage source");
        let destination = publication_plan.destination_root.join(&entry.relative_path);
        let displaced = root.join("displaced-baseline");

        let error = install_staged_destination(&publication_plan, entry, &staged, || {
            fs::rename(&destination, &displaced).expect("displace verified baseline");
            fs::write(&destination, b"attacker-owned").expect("replace destination inode");
        })
        .expect_err("destination replacement must reject");

        assert_eq!(error.code, "GATE-PUBLICATION-BASELINE-RACE");
        assert_eq!(
            fs::read(&destination).expect("replacement preserved"),
            b"attacker-owned"
        );
        assert_eq!(
            fs::read(&displaced).expect("baseline preserved"),
            b"old-result"
        );
        assert!(staged.exists(), "staged source remains recoverable");
        remove_scratch(&root);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn rejects_source_root_replacement_after_descriptor_read_without_staging() {
        let root = scratch("source-read-race");
        remove_scratch(&root);
        let publication_plan = plan(&root, None);
        prepare_transaction_directories(&publication_plan).expect("transaction dirs");
        let entry = &publication_plan.entries[0];
        let staged = publication_plan
            .transaction_root
            .join("staged")
            .join(&entry.relative_path);
        let displaced = root.join("source-displaced");
        let replacement_source = publication_plan.source_root.join(&entry.relative_path);

        let error = copy_source_to_stage(&publication_plan, entry, &staged, || {
            fs::rename(&publication_plan.source_root, &displaced).expect("displace source root");
            write(&replacement_source, b"attacker-owned");
        })
        .expect_err("source root replacement must reject");

        assert_eq!(error.code, "GATE-PUBLICATION-SOURCE-ROOT-RACE");
        assert!(!staged.exists(), "unaccepted bytes were not staged");
        assert_eq!(
            fs::read(&replacement_source).expect("replacement preserved"),
            b"attacker-owned"
        );
        assert_eq!(
            fs::read(displaced.join(&entry.relative_path)).expect("verified source preserved"),
            b"new-result"
        );
        remove_scratch(&root);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn rejects_transaction_root_replacement_before_staged_install() {
        let root = scratch("transaction-install-race");
        remove_scratch(&root);
        let publication_plan = plan(&root, None);
        prepare_transaction_directories(&publication_plan).expect("transaction dirs");
        let entry = &publication_plan.entries[0];
        let source = publication_plan.source_root.join(&entry.relative_path);
        let staged = publication_plan
            .transaction_root
            .join("staged")
            .join(&entry.relative_path);
        copy_verified(&source, &staged, Some(&entry.source_sha256)).expect("stage source");
        ensure_parent(&publication_plan.destination_root.join(&entry.relative_path))
            .expect("destination parent");
        let displaced = root.join("transaction-displaced");
        let replacement_stage = publication_plan
            .transaction_root
            .join("staged")
            .join(&entry.relative_path);

        let error = install_staged_destination(&publication_plan, entry, &staged, || {
            fs::rename(&publication_plan.transaction_root, &displaced)
                .expect("displace transaction root");
            write(&replacement_stage, b"attacker-owned");
        })
        .expect_err("transaction root replacement must reject");

        assert_eq!(error.code, "GATE-PUBLICATION-ROOT-RACE");
        assert_eq!(
            fs::read(&replacement_stage).expect("replacement stage preserved"),
            b"attacker-owned"
        );
        assert!(
            !publication_plan
                .destination_root
                .join(&entry.relative_path)
                .exists(),
            "no destination bytes installed"
        );
        assert!(
            displaced.join("staged").join(&entry.relative_path).exists(),
            "verified staged bytes remain recoverable"
        );
        remove_scratch(&root);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn rejects_transaction_root_replacement_during_journal_append() {
        let root = scratch("journal-root-race");
        remove_scratch(&root);
        let publication_plan = plan(&root, None);
        prepare_transaction_directories(&publication_plan).expect("transaction dirs");
        let entry = &publication_plan.entries[0];
        let record = PublicationJournalRecord {
            schema_version: JOURNAL_SCHEMA.to_owned(),
            publication_id: publication_plan.publication_id.clone(),
            sequence: 1,
            operation: JournalOperation::Install,
            relative_path: entry.relative_path.clone(),
            source_sha256: entry.source_sha256.clone(),
            destination_baseline_sha256: None,
            backup_relative_path: None,
        };
        let displaced = root.join("journal-transaction-displaced");

        let error =
            append_journal_descriptor_relative_with_hook(&publication_plan, &record, || {
                fs::rename(&publication_plan.transaction_root, &displaced)
                    .expect("displace journal transaction root");
                fs::create_dir_all(
                    publication_plan
                        .journal_path
                        .parent()
                        .expect("journal parent"),
                )
                .expect("replacement journal tree");
            })
            .expect_err("journal root replacement must reject");

        assert_eq!(error.code, "GATE-PUBLICATION-TRANSACTION-ROOT-RACE");
        assert!(
            !publication_plan.journal_path.exists(),
            "replacement transaction root received no journal bytes"
        );
        assert!(
            displaced.join("journal/publication.jsonl").exists(),
            "durable journal remains bound to the verified root"
        );
        remove_scratch(&root);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn rejects_symlinked_staging_subtree_without_external_write() {
        use std::os::unix::fs::symlink;

        let root = scratch("staging-subtree-race");
        remove_scratch(&root);
        let publication_plan = plan(&root, None);
        prepare_transaction_directories(&publication_plan).expect("transaction dirs");
        fs::remove_dir_all(publication_plan.transaction_root.join("staged"))
            .expect("remove staging directory");
        let external = root.join("external-staging");
        fs::create_dir_all(&external).expect("external staging");
        symlink(&external, publication_plan.transaction_root.join("staged"))
            .expect("replace staging subtree");
        let entry = &publication_plan.entries[0];
        let staged = publication_plan
            .transaction_root
            .join("staged")
            .join(&entry.relative_path);

        let error = copy_source_to_stage(&publication_plan, entry, &staged, || {})
            .expect_err("symlinked staging subtree must reject");

        assert_eq!(error.code, "GATE-PUBLICATION-TRANSACTION-DIR-OPEN");
        assert!(
            fs::read_dir(&external)
                .expect("external directory")
                .next()
                .is_none(),
            "external staging target remains untouched"
        );
        remove_scratch(&root);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn rejects_symlinked_backup_subtree_without_external_write() {
        use std::os::unix::fs::symlink;

        let root = scratch("backup-subtree-race");
        remove_scratch(&root);
        let publication_plan = plan(&root, Some(b"old-result"));
        prepare_transaction_directories(&publication_plan).expect("transaction dirs");
        let external = root.join("external-backups");
        fs::create_dir_all(&external).expect("external backups");
        symlink(&external, publication_plan.transaction_root.join("backups"))
            .expect("replace backup subtree");
        let entry = &publication_plan.entries[0];
        let backup = publication_plan
            .transaction_root
            .join("backups")
            .join(&entry.relative_path);

        let error = copy_destination_backup(
            &publication_plan,
            entry,
            &backup,
            entry.destination_baseline_sha256.as_deref(),
        )
        .expect_err("symlinked backup subtree must reject");

        assert_eq!(error.code, "GATE-PUBLICATION-TRANSACTION-DIR-OPEN");
        assert!(
            fs::read_dir(&external)
                .expect("external directory")
                .next()
                .is_none(),
            "external backup target remains untouched"
        );
        remove_scratch(&root);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn recovery_delete_rejects_destination_root_swap_without_redirection() {
        let root = scratch("recovery-destination-root-swap");
        remove_scratch(&root);
        let destination_root = root.join("destination");
        let relative_path = PathBuf::from("objects/artifacts/result.json");
        write(&destination_root.join(&relative_path), b"new-result");
        let plan = PublicationPlan {
            publication_id: "recovery-test".to_owned(),
            external_plan_path: root.join("plan"),
            transaction_receipt_path: root.join("receipt"),
            transaction_receipt_id: "0".repeat(64),
            transaction_receipt_sha256: "0".repeat(64),
            source_manifest_id: "0".repeat(64),
            source_manifest_sha256: "0".repeat(64),
            destination_baseline_sha256: "0".repeat(64),
            source_root: root.join("source"),
            destination_root: destination_root.clone(),
            transaction_root: root.join("transaction"),
            journal_path: root.join("transaction/journal/publication.jsonl"),
            receipt_path: root.join("transaction/receipt/publication.json"),
            entries: Vec::new(),
        };
        let record = PublicationJournalRecord {
            schema_version: JOURNAL_SCHEMA.to_owned(),
            publication_id: plan.publication_id.clone(),
            sequence: 1,
            operation: JournalOperation::Install,
            relative_path: relative_path.clone(),
            source_sha256: sha256_bytes(b"new-result"),
            destination_baseline_sha256: None,
            backup_relative_path: None,
        };
        let displaced = root.join("destination-displaced");
        let attacker = destination_root.join(&relative_path);
        let error = remove_installed_descriptor_relative_with_hook(&plan, &record, || {
            fs::rename(&destination_root, &displaced).expect("swap destination root");
            write(&attacker, b"new-result");
        })
        .expect_err("root swap must reject");
        assert_eq!(
            error.code,
            "GATE-PUBLICATION-RECOVERY-DESTINATION-ROOT-RACE"
        );
        assert_eq!(fs::read(&attacker).expect("attacker file"), b"new-result");
        assert!(displaced.join(&relative_path).exists());
        remove_scratch(&root);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn recovery_backup_read_rejects_swapped_ancestor_without_external_read() {
        use std::os::unix::fs::symlink;

        let root = scratch("recovery-backup-ancestor-swap");
        remove_scratch(&root);
        let transaction_root = root.join("transaction");
        fs::create_dir_all(&transaction_root).expect("transaction root");
        let external = root.join("external-backups");
        write(
            &external.join("objects/artifacts/result.json"),
            b"old-result",
        );
        symlink(&external, transaction_root.join("backups")).expect("swap backup ancestor");
        let plan = PublicationPlan {
            publication_id: "recovery-test".to_owned(),
            external_plan_path: root.join("plan"),
            transaction_receipt_path: root.join("receipt"),
            transaction_receipt_id: "0".repeat(64),
            transaction_receipt_sha256: "0".repeat(64),
            source_manifest_id: "0".repeat(64),
            source_manifest_sha256: "0".repeat(64),
            destination_baseline_sha256: "0".repeat(64),
            source_root: root.join("source"),
            destination_root: root.join("destination"),
            transaction_root: transaction_root.clone(),
            journal_path: transaction_root.join("journal/publication.jsonl"),
            receipt_path: transaction_root.join("receipt/publication.json"),
            entries: Vec::new(),
        };
        let staged = transaction_root.join("restore-staged/objects/artifacts/result.json");
        let error = copy_transaction_file_to_stage(
            &plan,
            Path::new("backups/objects/artifacts/result.json"),
            &staged,
            &sha256_bytes(b"old-result"),
        )
        .expect_err("swapped backup ancestor must reject");
        assert_eq!(error.code, "GATE-PUBLICATION-RECOVERY-BACKUP-OPEN");
        assert!(!staged.exists());
        assert_eq!(
            fs::read(external.join("objects/artifacts/result.json")).expect("external backup"),
            b"old-result"
        );
        remove_scratch(&root);
    }
}
