use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::amendment_support::{parse_yaml, render_yaml};
use super::identity::{
    IDENTITY_LOCK_PATH, IdentityLock, ReviewLock, calculate_review_lock, collect_regular_sources,
};
use super::read_regular_confined;
use crate::{AssuranceError, Result, sha256_bytes};

const CATALOG_PATH: &str = "assurance/v2/catalog.yaml";

/// Copies the generated-identity read set and reader narratives to a test root.
///
/// # Errors
///
/// Returns an error for a production-shaped target, invalid identity, or I/O
/// failure.
pub fn copy_v2_test_fixture(source: &Path, target: &Path) -> Result<()> {
    reject_git_target(target, "copy")?;
    let lock = IdentityLock::load(source)?;
    lock.verify_files(source)?;
    for path in lock.sources.keys().chain(lock.review_locks.keys()) {
        copy_fixture_member(source, target, Path::new(path))?;
    }
    for path in lock
        .sources
        .keys()
        .filter(|path| path.ends_with("/report.yaml"))
    {
        let report: serde_yaml::Value =
            parse_yaml(path, &read_regular_confined(source, Path::new(path))?)?;
        if let Some(narrative) = report
            .get("reader_metadata")
            .and_then(|value| value.get("related_model_narrative"))
            .and_then(serde_yaml::Value::as_str)
        {
            copy_fixture_member(source, target, &Path::new("usersum").join(narrative))?;
        }
    }
    copy_fixture_member(source, target, Path::new(IDENTITY_LOCK_PATH))
}

/// Recalculates generated identities in a repository-free valid test fixture.
///
/// # Errors
///
/// Returns an error for a production-shaped target or invalid fixture state.
pub fn rebind_v2_test_fixture(root: &Path) -> Result<()> {
    rebind_test_fixture(root, true)
}

/// Rebinds raw fixture bytes without recalculating semantic review locks.
///
/// # Errors
///
/// Returns an error for a production-shaped target or invalid fixture state.
pub fn rebind_invalid_v2_test_fixture(root: &Path) -> Result<()> {
    rebind_test_fixture(root, false)
}

/// Retains one declared report in a repository-free test fixture.
///
/// # Errors
///
/// Returns an error when the report is not uniquely declared or fixture
/// rebinding fails.
pub fn retain_v2_test_report(root: &Path, report_id: &str) -> Result<()> {
    reject_git_target(root, "pruning")?;
    let catalog_path = PathBuf::from(CATALOG_PATH);
    let bytes = read_regular_confined(root, &catalog_path)?;
    let mut catalog: serde_yaml::Value = parse_yaml(&catalog_path, &bytes)?;
    let reports = catalog
        .get_mut("reports")
        .and_then(serde_yaml::Value::as_sequence_mut)
        .ok_or_else(|| AssuranceError::Invalid("catalog reports are missing".to_owned()))?;
    reports
        .retain(|report| report.get("id").and_then(serde_yaml::Value::as_str) == Some(report_id));
    if reports.len() != 1 {
        return Err(AssuranceError::Invalid(format!(
            "fixture report '{report_id}' is not uniquely declared"
        )));
    }
    std::fs::write(root.join(&catalog_path), render_yaml(&catalog)?)
        .map_err(|error| AssuranceError::io(root.join(&catalog_path), error))?;
    let report_root = root.join("assurance/v2/reports");
    for entry in
        std::fs::read_dir(&report_root).map_err(|error| AssuranceError::io(&report_root, error))?
    {
        let entry = entry.map_err(|error| AssuranceError::io(&report_root, error))?;
        if entry.file_name() != std::ffi::OsStr::new(report_id) {
            std::fs::remove_dir_all(entry.path())
                .map_err(|error| AssuranceError::io(entry.path(), error))?;
        }
    }
    rebind_v2_test_fixture(root)
}

fn rebind_test_fixture(root: &Path, recalculate_review_locks: bool) -> Result<()> {
    reject_git_target(root, "rebind")?;
    let previous = IdentityLock::load(root)?;
    if recalculate_review_locks {
        recalculate_fixture_review_locks(root)?;
    }
    let mut external = BTreeMap::new();
    for path in previous.sources.keys() {
        let relative = Path::new(path);
        if !path.starts_with("assurance/v2/") && root.join(relative).is_file() {
            external.insert(
                path.clone(),
                sha256_bytes(&read_regular_confined(root, relative)?),
            );
        }
    }
    let sources = collect_regular_sources(root, &external)?;
    let mut review_locks = BTreeMap::new();
    let reports = root.join("assurance/v2/reports");
    for entry in std::fs::read_dir(&reports).map_err(|error| AssuranceError::io(&reports, error))? {
        let entry = entry.map_err(|error| AssuranceError::io(&reports, error))?;
        let path = entry.path().join("review.lock.json");
        if path.is_file() {
            let relative = path.strip_prefix(root).map_err(|_| {
                AssuranceError::Invalid("fixture review lock escaped root".to_owned())
            })?;
            review_locks.insert(
                relative.to_string_lossy().into_owned(),
                sha256_bytes(
                    &std::fs::read(&path).map_err(|error| AssuranceError::io(&path, error))?,
                ),
            );
        }
    }
    let next = IdentityLock::successor(&previous, sources, review_locks)?;
    std::fs::write(root.join(IDENTITY_LOCK_PATH), next.render()?)
        .map_err(|error| AssuranceError::io(root.join(IDENTITY_LOCK_PATH), error))
}

fn recalculate_fixture_review_locks(root: &Path) -> Result<()> {
    let principals_path = PathBuf::from("assurance/v2/principals.yaml");
    let principals: serde_yaml::Value = parse_yaml(
        &principals_path,
        &read_regular_confined(root, &principals_path)?,
    )?;
    let reports = root.join("assurance/v2/reports");
    for entry in std::fs::read_dir(&reports).map_err(|error| AssuranceError::io(&reports, error))? {
        let entry = entry.map_err(|error| AssuranceError::io(&reports, error))?;
        if !entry
            .file_type()
            .map_err(|error| AssuranceError::io(entry.path(), error))?
            .is_dir()
        {
            continue;
        }
        recalculate_report_review_lock(root, &entry.path(), &principals)?;
    }
    Ok(())
}

fn recalculate_report_review_lock(
    root: &Path,
    report_directory: &Path,
    principals: &serde_yaml::Value,
) -> Result<()> {
    let report_id = report_directory
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            AssuranceError::Invalid("fixture report directory is not UTF-8".to_owned())
        })?;
    let report_path = report_directory.join("report.yaml");
    let lock_path = report_directory.join("review.lock.json");
    if !report_path.is_file() || !lock_path.is_file() {
        return Ok(());
    }
    let relative_report = report_path
        .strip_prefix(root)
        .map_err(|_| AssuranceError::Invalid("fixture report escaped root".to_owned()))?;
    let relative_lock = lock_path
        .strip_prefix(root)
        .map_err(|_| AssuranceError::Invalid("fixture review lock escaped root".to_owned()))?;
    let report: serde_yaml::Value = parse_yaml(
        relative_report,
        &read_regular_confined(root, relative_report)?,
    )?;
    let current = ReviewLock::parse(relative_lock, &read_regular_confined(root, relative_lock)?)?;
    let mut calculated = calculate_review_lock(
        root,
        report_id,
        &report,
        principals,
        &BTreeMap::new(),
        current.legacy_subject_root,
        current.event_ids,
    )?;
    calculated.invalidated_event_ids = current.invalidated_event_ids;
    std::fs::write(&lock_path, calculated.render()?)
        .map_err(|error| AssuranceError::io(&lock_path, error))
}

fn copy_fixture_member(source: &Path, target: &Path, relative: &Path) -> Result<()> {
    let destination = target.join(relative);
    let parent = destination.parent().ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "fixture path has no parent: {}",
            relative.display()
        ))
    })?;
    std::fs::create_dir_all(parent).map_err(|error| AssuranceError::io(parent, error))?;
    let bytes = read_regular_confined(source, relative)?;
    std::fs::write(&destination, bytes).map_err(|error| AssuranceError::io(&destination, error))
}

fn reject_git_target(target: &Path, operation: &str) -> Result<()> {
    if target.join(".git").exists() {
        return Err(AssuranceError::Invalid(format!(
            "fixture {operation} refuses a target containing .git"
        )));
    }
    Ok(())
}
