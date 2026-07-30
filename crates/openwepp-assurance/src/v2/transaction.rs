use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt as _;

use super::V2Repository;
use super::amendment::{MigrationCandidate, NEXT_ROOT, V2_ROOT, V2AmendmentReceipt};
use super::confined::ConfinedDirectory;
use super::identity::{IDENTITY_LOCK_PATH, IdentityLock};
use crate::{AssuranceError, Result, sha256_bytes};

static CANDIDATE_SERIAL: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, PartialEq, Eq)]
struct TreeEntry {
    sha256: String,
    mode: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TreeSnapshot {
    root_mode: u32,
    directories: BTreeMap<PathBuf, u32>,
    files: BTreeMap<PathBuf, TreeEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExternalSnapshot {
    entries: BTreeMap<PathBuf, TreeEntry>,
    directories: BTreeMap<PathBuf, u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TestFault {
    AfterClone,
    BeforeExchange,
    AfterExchange,
    BeforeCleanup,
    #[cfg(test)]
    ExternalDrift,
}

#[cfg(test)]
thread_local! {
    static TEST_FAULT: std::cell::Cell<Option<TestFault>> = const { std::cell::Cell::new(None) };
}

pub(super) fn apply_candidate(
    root: &Path,
    candidate: MigrationCandidate,
) -> Result<V2AmendmentReceipt> {
    let root = root
        .canonicalize()
        .map_err(|error| AssuranceError::io(root, error))?;
    let transaction = ConfinedDirectory::open_ambient(&root, false)?;
    transaction.lock_exclusive(&root)?;
    verify_compare_and_swap(&root, &candidate)?;
    if transaction.directory_exists(Path::new(NEXT_ROOT))? {
        return Err(AssuranceError::Invalid(format!(
            "amendment recovery state requires explicit disposition: {NEXT_ROOT}"
        )));
    }

    let held = capture_tree(&transaction, Path::new(V2_ROOT))?;
    let external = capture_external_read_set(&root, &candidate)?;
    let preparation = (|| {
        clone_v2_tree(&transaction, &held)?;
        inject_fault(TestFault::AfterClone)?;
        apply_replacements(&transaction, &candidate.replacements, &held)?;
        transaction.sync_filesystem()?;
        transaction.sync_tree(Path::new(NEXT_ROOT))?;
        transaction.sync_parent()?;
        let staged = capture_tree(&transaction, Path::new(NEXT_ROOT))?;
        validate_isolated_candidate(&root, &transaction, &candidate)?;
        if capture_tree(&transaction, Path::new(NEXT_ROOT))? != staged {
            return Err(AssuranceError::Drift(
                "staged assurance generation changed during candidate validation".to_owned(),
            ));
        }
        if capture_tree(&transaction, Path::new(V2_ROOT))? != held {
            return Err(AssuranceError::Drift(
                "active assurance generation changed before exchange".to_owned(),
            ));
        }
        inject_external_drift(&root, &external, &candidate.allowed_preexisting_drift)?;
        verify_external_read_set(&root, &external)?;
        verify_compare_and_swap(&root, &candidate)?;
        inject_fault(TestFault::BeforeExchange)
    })();
    if let Err(error) = preparation {
        return Err(combine_recovery(error, discard_next(&transaction)));
    }

    if let Err(error) = transaction.exchange(Path::new(V2_ROOT), Path::new(NEXT_ROOT)) {
        return Err(combine_recovery(error, discard_next(&transaction)));
    }
    if let Err(error) = inject_fault(TestFault::AfterExchange) {
        return Err(combine_recovery(error, restore_previous(&transaction)));
    }
    if let Err(error) = transaction.sync_parent() {
        return Err(combine_recovery(error, restore_previous(&transaction)));
    }
    if let Err(error) = verify_installed(&root, &candidate) {
        return Err(combine_recovery(error, restore_previous(&transaction)));
    }
    let receipt = candidate.receipt;
    if inject_fault(TestFault::BeforeCleanup).is_err() {
        return Ok(receipt);
    }
    if transaction
        .remove_directory_if_exists(Path::new(NEXT_ROOT))
        .is_err()
    {
        return Ok(receipt);
    }
    let _ = transaction.sync_parent();
    Ok(receipt)
}

pub(super) fn check_candidate(
    root: &Path,
    candidate: &MigrationCandidate,
) -> Result<V2AmendmentReceipt> {
    let root = root
        .canonicalize()
        .map_err(|error| AssuranceError::io(root, error))?;
    let transaction = ConfinedDirectory::open_ambient(&root, false)?;
    transaction.lock_exclusive(&root)?;
    verify_compare_and_swap(&root, candidate)?;
    if transaction.directory_exists(Path::new(NEXT_ROOT))? {
        return Err(AssuranceError::Invalid(format!(
            "amendment recovery state requires explicit disposition: {NEXT_ROOT}"
        )));
    }
    let held = capture_tree(&transaction, Path::new(V2_ROOT))?;
    let external = capture_external_read_set(&root, candidate)?;
    let check_root = create_owned_temporary("amend-check", &candidate.receipt.new_generation_id)?;
    let validation = (|| {
        copy_confined_tree(
            &transaction,
            Path::new(V2_ROOT),
            &check_root.join(NEXT_ROOT),
        )?;
        let checked = ConfinedDirectory::open_ambient(&check_root, false)?;
        apply_replacements(&checked, &candidate.replacements, &held)?;
        checked.sync_filesystem()?;
        let staged = capture_tree(&checked, Path::new(NEXT_ROOT))?;
        validate_isolated_candidate(&root, &checked, candidate)?;
        if capture_tree(&checked, Path::new(NEXT_ROOT))? != staged {
            return Err(AssuranceError::Drift(
                "checked assurance generation changed during candidate validation".to_owned(),
            ));
        }
        if capture_tree(&transaction, Path::new(V2_ROOT))? != held {
            return Err(AssuranceError::Drift(
                "active assurance generation changed during candidate check".to_owned(),
            ));
        }
        verify_external_read_set(&root, &external)?;
        verify_compare_and_swap(&root, candidate)
    })();
    let cleanup = remove_temporary(&check_root);
    match (validation, cleanup) {
        (Ok(()), Ok(())) => Ok(candidate.receipt.clone()),
        (Err(error), cleanup) => Err(combine_recovery(error, cleanup)),
        (Ok(()), Err(error)) => Err(error),
    }
}

pub(super) fn verify_generation_tree(root: &Path, tree: &Path) -> Result<String> {
    let root = root
        .canonicalize()
        .map_err(|error| AssuranceError::io(root, error))?;
    let confined = ConfinedDirectory::open_ambient(&root, false)?;
    let lock_path = tree.join("identity.lock.json");
    let bytes = confined.read_regular(&lock_path)?;
    let lock = IdentityLock::parse(&lock_path, &bytes)?;
    for (path, expected) in lock.sources.iter().chain(&lock.review_locks) {
        let declared = Path::new(path);
        let selected = if let Ok(relative) = declared.strip_prefix(V2_ROOT) {
            tree.join(relative)
        } else {
            declared.to_path_buf()
        };
        let observed = sha256_bytes(&confined.read_regular(&selected)?);
        if observed != *expected {
            return Err(AssuranceError::Drift(format!(
                "recovery generation member changed: {path}"
            )));
        }
    }
    Ok(lock.generation_id)
}

fn capture_external_read_set(
    root: &Path,
    candidate: &MigrationCandidate,
) -> Result<ExternalSnapshot> {
    let lock = if let Ok(lock) = IdentityLock::load(root) {
        lock
    } else {
        let bytes = candidate
            .replacements
            .get(Path::new(IDENTITY_LOCK_PATH))
            .ok_or_else(|| {
                AssuranceError::Invalid(
                    "identity migration candidate omitted the generated identity lock".to_owned(),
                )
            })?;
        serde_json::from_slice(bytes).map_err(|error| AssuranceError::Parse {
            path: PathBuf::from(IDENTITY_LOCK_PATH),
            message: error.to_string(),
        })?
    };
    let mut files = lock
        .sources
        .keys()
        .filter(|path| !path.starts_with("assurance/v2/"))
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    files.sort();
    files.dedup();
    let confined = ConfinedDirectory::open_ambient(root, false)?;
    let directories = if confined.directory_exists(Path::new("usersum"))? {
        vec![PathBuf::from("usersum")]
    } else {
        Vec::new()
    };
    capture_confined_paths(&confined, &files, &directories)
}

fn capture_confined_paths(
    root: &ConfinedDirectory,
    files: &[PathBuf],
    directory_roots: &[PathBuf],
) -> Result<ExternalSnapshot> {
    let mut snapshot = ExternalSnapshot {
        entries: BTreeMap::new(),
        directories: BTreeMap::new(),
    };
    for relative in files {
        snapshot.entries.insert(
            relative.clone(),
            TreeEntry {
                sha256: sha256_bytes(&root.read_regular(relative)?),
                mode: root.mode(relative)?,
            },
        );
    }
    for directory_root in directory_roots {
        snapshot
            .directories
            .insert(directory_root.clone(), root.mode(directory_root)?);
        for directory in root.collect_directories(directory_root)? {
            let relative = directory_root.join(directory);
            snapshot
                .directories
                .insert(relative.clone(), root.mode(&relative)?);
        }
        for file in root.collect_regular_files(directory_root)? {
            let relative = directory_root.join(file);
            snapshot.entries.insert(
                relative.clone(),
                TreeEntry {
                    sha256: sha256_bytes(&root.read_regular(&relative)?),
                    mode: root.mode(&relative)?,
                },
            );
        }
    }
    Ok(snapshot)
}

fn verify_external_read_set(root: &Path, expected: &ExternalSnapshot) -> Result<()> {
    let mut selected = expected.directories.keys().cloned().collect::<Vec<_>>();
    selected.extend(expected.entries.keys().cloned());
    selected.retain(|path| {
        path.parent().is_none()
            || !expected
                .directories
                .keys()
                .any(|parent| parent != path && path.starts_with(parent))
    });
    selected.sort();
    selected.dedup();
    let confined = ConfinedDirectory::open_ambient(root, false)?;
    let directory_roots = selected
        .iter()
        .filter(|path| expected.directories.contains_key(*path))
        .cloned()
        .collect::<Vec<_>>();
    let files = selected
        .into_iter()
        .filter(|path| expected.entries.contains_key(path))
        .collect::<Vec<_>>();
    if capture_confined_paths(&confined, &files, &directory_roots)? != *expected {
        return Err(AssuranceError::Drift(
            "external assurance read set changed before exchange".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(test)]
fn inject_fault(point: TestFault) -> Result<()> {
    if TEST_FAULT.get() == Some(point) {
        return Err(AssuranceError::Invalid(format!(
            "injected amendment transaction fault at {point:?}"
        )));
    }
    Ok(())
}

#[cfg(not(test))]
#[allow(clippy::unnecessary_wraps)]
fn inject_fault(_point: TestFault) -> Result<()> {
    Ok(())
}

#[cfg(test)]
fn inject_external_drift(
    root: &Path,
    expected: &ExternalSnapshot,
    allowed_preexisting_drift: &std::collections::BTreeSet<String>,
) -> Result<()> {
    if TEST_FAULT.get() == Some(TestFault::ExternalDrift) {
        let path = allowed_preexisting_drift
            .iter()
            .map(PathBuf::from)
            .find(|path| expected.entries.contains_key(path))
            .or_else(|| expected.entries.keys().next().cloned())
            .ok_or_else(|| {
                AssuranceError::Invalid("external drift fixture has no regular input".to_owned())
            })?;
        fs::OpenOptions::new()
            .append(true)
            .open(root.join(&path))
            .and_then(|mut file| {
                use std::io::Write as _;
                file.write_all(b"\n")
            })
            .map_err(|error| AssuranceError::io(root.join(path), error))?;
    }
    Ok(())
}

#[cfg(not(test))]
#[allow(clippy::unnecessary_wraps)]
fn inject_external_drift(
    _root: &Path,
    _expected: &ExternalSnapshot,
    _allowed_preexisting_drift: &std::collections::BTreeSet<String>,
) -> Result<()> {
    Ok(())
}

fn verify_compare_and_swap(root: &Path, candidate: &MigrationCandidate) -> Result<()> {
    match candidate.receipt.old_generation_id.as_deref() {
        Some(expected) => {
            let current = IdentityLock::load(root)?;
            let exceptions = candidate
                .allowed_preexisting_drift
                .iter()
                .map(String::as_str)
                .collect();
            current.verify_files_except(root, &exceptions)?;
            if current.generation_id != expected {
                return Err(AssuranceError::Drift(format!(
                    "amendment compare-and-swap rejected stale generation '{expected}'"
                )));
            }
        }
        None if root.join(IDENTITY_LOCK_PATH).exists() => {
            return Err(AssuranceError::Drift(
                "identity genesis migration requires an uninitialized generation".to_owned(),
            ));
        }
        None => {}
    }
    Ok(())
}

fn validate_isolated_candidate(
    root: &Path,
    transaction: &ConfinedDirectory,
    candidate: &MigrationCandidate,
) -> Result<()> {
    let candidate_root =
        create_owned_temporary("amend-candidate", &candidate.receipt.new_generation_id)?;
    let staging = create_owned_temporary("amend-staging", &candidate.receipt.new_generation_id)?;
    let result = (|| {
        copy_confined_tree(
            transaction,
            Path::new(NEXT_ROOT),
            &candidate_root.join(V2_ROOT),
        )?;
        let lock = IdentityLock::load(&candidate_root)?;
        let source_root = ConfinedDirectory::open_ambient(root, false)?;
        copy_external_sources(&source_root, &candidate_root, &lock)?;
        if source_root.directory_exists(Path::new("usersum"))? {
            copy_confined_tree(
                &source_root,
                Path::new("usersum"),
                &candidate_root.join("usersum"),
            )?;
        }
        let repository = V2Repository::open(&candidate_root)?;
        for report_id in &candidate.receipt.affected_reports {
            repository.validate_report(report_id)?;
        }
        seed_usersum(&candidate_root, &staging)?;
        for report_id in &candidate.receipt.affected_reports {
            repository.build_report(report_id, &staging)?;
            repository.check_report(report_id, &staging)?;
        }
        Ok(())
    })();
    let cleanup = remove_temporary(&candidate_root).and_then(|()| remove_temporary(&staging));
    match (result, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
        (Err(primary), Err(recovery)) => Err(AssuranceError::Recovery {
            primary: Box::new(primary),
            recovery: Box::new(recovery),
        }),
    }
}

fn remove_temporary(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).map_err(|error| AssuranceError::io(path, error))?;
    }
    Ok(())
}

fn create_owned_temporary(label: &str, generation: &str) -> Result<PathBuf> {
    for _ in 0..1024 {
        let serial = CANDIDATE_SERIAL.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "openwepp-assurance-{label}-{}-{serial}-{}",
            std::process::id(),
            &generation[..16]
        ));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(AssuranceError::io(path, error)),
        }
    }
    Err(AssuranceError::Invalid(format!(
        "could not allocate an owned assurance {label} temporary directory"
    )))
}

fn copy_external_sources(
    root: &ConfinedDirectory,
    candidate_root: &Path,
    lock: &IdentityLock,
) -> Result<()> {
    let target = ConfinedDirectory::open_ambient(candidate_root, false)?;
    for source in lock.sources.keys() {
        if source.starts_with("assurance/v2/") || source.starts_with("usersum/") {
            continue;
        }
        let relative = Path::new(source);
        root.clone_regular_to(relative, &target, relative)?;
        target.set_mode(relative, root.mode(relative)?)?;
    }
    Ok(())
}

fn seed_usersum(candidate_root: &Path, staging: &Path) -> Result<()> {
    let usersum = candidate_root.join("usersum");
    if usersum.is_dir() {
        copy_filesystem_tree(&usersum, &staging.join("usersum"))?;
    }
    Ok(())
}

fn copy_filesystem_tree(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target).map_err(|error| AssuranceError::io(target, error))?;
    let mut entries = fs::read_dir(source)
        .map_err(|error| AssuranceError::io(source, error))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|error| AssuranceError::io(source, error))?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|error| AssuranceError::io(entry.path(), error))?;
        let destination = target.join(entry.file_name());
        if metadata.file_type().is_symlink() {
            return Err(AssuranceError::Invalid(format!(
                "candidate source contains a symlink: {}",
                entry.path().display()
            )));
        }
        if metadata.is_dir() {
            copy_filesystem_tree(&entry.path(), &destination)?;
        } else if metadata.is_file() {
            let bytes =
                fs::read(entry.path()).map_err(|error| AssuranceError::io(entry.path(), error))?;
            write_copied_file(&entry.path(), &destination, &bytes)?;
        }
    }
    Ok(())
}

fn copy_confined_tree(root: &ConfinedDirectory, source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target).map_err(|error| AssuranceError::io(target, error))?;
    let target_root = ConfinedDirectory::open_ambient(target, false)?;
    for directory in root.collect_directories(source)? {
        let destination = target.join(&directory);
        fs::create_dir_all(&destination)
            .map_err(|error| AssuranceError::io(&destination, error))?;
    }
    for file in root.collect_regular_files(source)? {
        let confined = source.join(&file);
        root.clone_regular_to(&confined, &target_root, &file)?;
        target_root.set_mode(&file, root.mode(&confined)?)?;
    }
    Ok(())
}

fn write_copied_file(source: &Path, target: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|error| AssuranceError::io(parent, error))?;
    }
    fs::write(target, bytes).map_err(|error| AssuranceError::io(target, error))?;
    let metadata = fs::metadata(source).map_err(|error| AssuranceError::io(source, error))?;
    #[cfg(unix)]
    set_mode(target, metadata.permissions().mode())?;
    Ok(())
}

fn set_mode(path: &Path, mode: u32) -> Result<()> {
    #[cfg(unix)]
    {
        fs::set_permissions(path, fs::Permissions::from_mode(mode))
            .map_err(|error| AssuranceError::io(path, error))?;
    }
    #[cfg(not(unix))]
    let _ = (path, mode);
    Ok(())
}

fn verify_installed(root: &Path, candidate: &MigrationCandidate) -> Result<()> {
    let lock = IdentityLock::load(root)?;
    lock.verify_files(root)?;
    if lock.generation_id != candidate.receipt.new_generation_id {
        return Err(AssuranceError::Drift(
            "installed generation does not match amendment receipt".to_owned(),
        ));
    }
    let bytes = fs::read(root.join(&candidate.receipt_path))
        .map_err(|error| AssuranceError::io(root.join(&candidate.receipt_path), error))?;
    let observed: V2AmendmentReceipt =
        serde_json::from_slice(&bytes).map_err(|error| AssuranceError::Parse {
            path: candidate.receipt_path.clone(),
            message: error.to_string(),
        })?;
    if observed != candidate.receipt {
        return Err(AssuranceError::Drift(
            "installed amendment receipt differs from committed transition".to_owned(),
        ));
    }
    if !lock.review_locks.is_empty() {
        V2Repository::open(root)?.validate_all()?;
    }
    Ok(())
}

fn capture_tree(root: &ConfinedDirectory, base: &Path) -> Result<TreeSnapshot> {
    let mut directories = BTreeMap::new();
    for path in root.collect_directories(base)? {
        directories.insert(path.clone(), root.mode(&base.join(path))?);
    }
    let mut files = BTreeMap::new();
    for path in root.collect_regular_files(base)? {
        let source = base.join(&path);
        let bytes = root.read_regular(&source)?;
        files.insert(
            path,
            TreeEntry {
                sha256: sha256_bytes(&bytes),
                mode: root.mode(&source)?,
            },
        );
    }
    Ok(TreeSnapshot {
        root_mode: root.mode(base)?,
        directories,
        files,
    })
}

fn clone_v2_tree(root: &ConfinedDirectory, expected: &TreeSnapshot) -> Result<()> {
    root.create_dir_all(Path::new(NEXT_ROOT))?;
    for (directory, mode) in &expected.directories {
        let target = Path::new(NEXT_ROOT).join(directory);
        root.create_dir_all(&target)?;
        root.set_mode(&target, *mode)?;
    }
    for (file, entry) in &expected.files {
        let source = Path::new(V2_ROOT).join(file);
        let target = Path::new(NEXT_ROOT).join(file);
        if root.mode(&source)? != entry.mode {
            return Err(AssuranceError::Drift(format!(
                "assurance source changed while cloning: {}",
                source.display()
            )));
        }
        root.clone_regular_to(&source, root, &target)?;
        root.set_mode(&target, entry.mode)?;
    }
    root.set_mode(Path::new(NEXT_ROOT), expected.root_mode)
}

fn apply_replacements(
    root: &ConfinedDirectory,
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
    expected: &TreeSnapshot,
) -> Result<()> {
    for (path, bytes) in replacements {
        let relative = path.strip_prefix(V2_ROOT).map_err(|_| {
            AssuranceError::Invalid(format!(
                "amendment replacement escapes {V2_ROOT}: {}",
                path.display()
            ))
        })?;
        let target = Path::new(NEXT_ROOT).join(relative);
        let mode = expected
            .files
            .get(relative)
            .map_or(0o644, |entry| entry.mode);
        if expected.files.contains_key(relative) && !root.remove_regular_if_exists(&target)? {
            return Err(AssuranceError::Drift(format!(
                "staged amendment target disappeared: {}",
                target.display()
            )));
        }
        root.write_new(&target, bytes)?;
        root.set_mode(&target, mode)?;
    }
    Ok(())
}

fn restore_previous(root: &ConfinedDirectory) -> Result<()> {
    root.exchange(Path::new(V2_ROOT), Path::new(NEXT_ROOT))?;
    root.sync_parent()?;
    discard_next(root)
}

fn discard_next(root: &ConfinedDirectory) -> Result<()> {
    root.remove_directory_if_exists(Path::new(NEXT_ROOT))?;
    root.sync_parent()
}

fn combine_recovery(primary: AssuranceError, recovery: Result<()>) -> AssuranceError {
    match recovery {
        Ok(()) => primary,
        Err(recovery) => AssuranceError::Recovery {
            primary: Box::new(primary),
            recovery: Box::new(recovery),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{NEXT_ROOT, TEST_FAULT, TestFault, V2_ROOT, verify_generation_tree};
    use crate::v2::amendment::{V2AmendMode, adopt_report_source, amend_attribution};
    use crate::v2::fixture::copy_v2_test_fixture;

    #[test]
    fn every_precommit_fault_preserves_the_active_generation() {
        for fault in [
            TestFault::AfterClone,
            TestFault::BeforeExchange,
            TestFault::AfterExchange,
            TestFault::ExternalDrift,
        ] {
            let fixture = Fixture::new("assurance-amendment-fault");
            copy_v2_test_fixture(&repository_root(), &fixture.path).expect("copy fixture");
            let before = capture_files(&fixture.path.join(V2_ROOT));
            TEST_FAULT.set(Some(fault));
            let result = amend_attribution(
                &fixture.path,
                "roger-lew",
                Some("Roger Lew, transaction fault fixture"),
                None,
                V2AmendMode::Apply,
            );
            TEST_FAULT.set(None);
            assert!(result.is_err(), "fault {fault:?} must fail");
            assert_eq!(before, capture_files(&fixture.path.join(V2_ROOT)));
            assert!(!fixture.path.join(NEXT_ROOT).exists());
        }
    }

    #[test]
    fn adoption_selected_source_race_preserves_the_active_generation() {
        let fixture = Fixture::new("assurance-adoption-external-race");
        copy_v2_test_fixture(&repository_root(), &fixture.path).expect("copy fixture");
        let selected = Path::new("tests/fixtures/cancov_forest/README.md");
        fs::OpenOptions::new()
            .append(true)
            .open(fixture.path.join(selected))
            .and_then(|mut file| {
                use std::io::Write as _;
                file.write_all(b"\nsource adoption initial drift\n")
            })
            .expect("create selected source drift");
        let before = capture_files(&fixture.path.join(V2_ROOT));
        TEST_FAULT.set(Some(TestFault::ExternalDrift));
        let result = adopt_report_source(
            &fixture.path,
            "snow-and-frozen-soil-process-evaluation",
            selected,
            V2AmendMode::Apply,
        );
        TEST_FAULT.set(None);
        assert!(
            result
                .expect_err("selected-source race must fail")
                .to_string()
                .contains("external assurance read set changed before exchange")
        );
        assert_eq!(before, capture_files(&fixture.path.join(V2_ROOT)));
        assert!(!fixture.path.join(NEXT_ROOT).exists());
    }

    #[test]
    fn committed_cleanup_fault_leaves_new_generation_and_typed_recovery_state() {
        let fixture = Fixture::new("assurance-amendment-cleanup-fault");
        copy_v2_test_fixture(&repository_root(), &fixture.path).expect("copy fixture");
        let before = capture_files(&fixture.path.join(V2_ROOT));
        TEST_FAULT.set(Some(TestFault::BeforeCleanup));
        let receipt = amend_attribution(
            &fixture.path,
            "roger-lew",
            Some("Roger Lew, committed cleanup fixture"),
            None,
            V2AmendMode::Apply,
        )
        .expect("committed cleanup fault returns the committed receipt");
        TEST_FAULT.set(None);
        assert!(receipt.changed);
        assert_ne!(before, capture_files(&fixture.path.join(V2_ROOT)));
        assert!(fixture.path.join(NEXT_ROOT).is_dir());
        let repeated = amend_attribution(
            &fixture.path,
            "roger-lew",
            Some("Roger Lew, another revision"),
            None,
            V2AmendMode::Apply,
        )
        .expect_err("pending cleanup must block later amendments");
        assert!(repeated.to_string().contains("recovery state"));
    }

    #[test]
    fn recovery_tree_verifier_checks_the_selected_generation_members() {
        let fixture = Fixture::new("assurance-recovery-tree-verifier");
        copy_v2_test_fixture(&repository_root(), &fixture.path).expect("copy fixture");
        let generation = verify_generation_tree(&fixture.path, Path::new(V2_ROOT))
            .expect("verify copied generation");
        assert!(!generation.is_empty());
        fs::OpenOptions::new()
            .append(true)
            .open(fixture.path.join("assurance/v2/catalog.yaml"))
            .and_then(|mut file| {
                use std::io::Write as _;
                file.write_all(b"\n")
            })
            .expect("change selected generation member");
        assert!(
            verify_generation_tree(&fixture.path, Path::new(V2_ROOT))
                .expect_err("changed member must fail")
                .to_string()
                .contains("generation member changed")
        );
    }

    fn capture_files(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
        fn visit(root: &Path, current: &Path, output: &mut Vec<(PathBuf, Vec<u8>)>) {
            for entry in fs::read_dir(current).expect("read fixture tree") {
                let entry = entry.expect("fixture entry");
                if entry.file_type().expect("fixture metadata").is_dir() {
                    visit(root, &entry.path(), output);
                } else {
                    output.push((
                        entry
                            .path()
                            .strip_prefix(root)
                            .expect("relative fixture path")
                            .to_path_buf(),
                        fs::read(entry.path()).expect("fixture bytes"),
                    ));
                }
            }
        }
        let mut output = Vec::new();
        visit(root, root, &mut output);
        output.sort_by(|left, right| left.0.cmp(&right.0));
        output
    }

    fn repository_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("workspace root")
            .to_path_buf()
    }

    struct Fixture {
        path: PathBuf,
    }

    impl Fixture {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let serial = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("openwepp-{label}-{}-{serial}", std::process::id()));
            if path.exists() {
                fs::remove_dir_all(&path).expect("remove stale fixture");
            }
            fs::create_dir_all(&path).expect("create fixture");
            Self { path }
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            if self.path.exists() {
                fs::remove_dir_all(&self.path).expect("remove fixture");
            }
        }
    }
}
