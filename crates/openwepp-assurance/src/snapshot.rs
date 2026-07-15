use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::engine::Bundle;
use crate::error::{AssuranceError, Result};
use crate::hash::{sha256_bytes, sha256_file};
use crate::path::{create_dir_all_no_symlinks, safe_output, validate_snapshot_id};

#[derive(Debug)]
pub(crate) struct SnapshotResult {
    pub manifest_path: PathBuf,
    pub manifest_sha256: String,
    pub confirmed_existing: bool,
}

pub(crate) fn create_snapshot(
    root: &Path,
    snapshot_id: &str,
    catalog_sha256: &str,
    contract_version: u32,
    tool_source_sha256: &str,
    bundles: &[Bundle],
    files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<SnapshotResult> {
    validate_snapshot_id(snapshot_id)?;
    create_dir_all_no_symlinks(root, "snapshot root")?;
    let canonical_root = root
        .canonicalize()
        .map_err(|error| AssuranceError::io(root, error))?;
    let target = safe_output(&canonical_root, Path::new(snapshot_id), "snapshot path")?;
    let manifest = snapshot_manifest(
        snapshot_id,
        catalog_sha256,
        contract_version,
        tool_source_sha256,
        bundles,
        files,
    )?;
    if target.exists() {
        return confirm_existing(&target, manifest.as_bytes(), files);
    }
    write_new_snapshot(&canonical_root, &target, files, manifest.as_bytes())
}

fn snapshot_manifest(
    snapshot_id: &str,
    catalog_sha256: &str,
    contract_version: u32,
    tool_source_sha256: &str,
    bundles: &[Bundle],
    files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<String> {
    let dossiers = bundles
        .iter()
        .map(|bundle| SnapshotDossier {
            dossier_id: bundle.dossier.dossier_id.clone(),
            version: bundle.dossier.version.clone(),
            lifecycle: bundle.dossier.lifecycle.label(),
            empirical: bundle.dossier.empirical.label(),
            scientific_root_sha256: bundle.scientific_root.clone(),
            publication_root_sha256: bundle.source_root.clone(),
        })
        .collect();
    let generated_files = files
        .iter()
        .map(|(path, bytes)| SnapshotFile {
            path: path.to_string_lossy().into_owned(),
            sha256: sha256_bytes(bytes),
        })
        .collect();
    let record = SnapshotManifest {
        schema_version: 1,
        snapshot_id,
        tool_version: env!("CARGO_PKG_VERSION"),
        contract_version,
        catalog_sha256,
        tool_source_sha256,
        dossiers,
        public_files: generated_files,
    };
    let mut output = serde_json::to_string_pretty(&record).map_err(|error| {
        AssuranceError::Invalid(format!("failed to serialize snapshot manifest: {error}"))
    })?;
    output.push('\n');
    Ok(output)
}

fn confirm_existing(
    target: &Path,
    manifest: &[u8],
    files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<SnapshotResult> {
    validate_snapshot_layout(target, files)?;
    let manifest_path = target.join("manifest.json");
    if !file_matches(&manifest_path, manifest)? {
        let id = snapshot_name(target)?;
        return Err(AssuranceError::SnapshotConflict(format!(
            "ID '{}' already exists with different content",
            id.to_string_lossy()
        )));
    }
    for (relative, expected) in files {
        let path = target.join("files").join(relative);
        if !file_matches(&path, expected)? {
            let id = snapshot_name(target)?;
            return Err(AssuranceError::SnapshotConflict(format!(
                "ID '{}' has a mutated file '{}'",
                id.to_string_lossy(),
                relative.display()
            )));
        }
    }
    validate_snapshot_layout(target, files)?;
    Ok(SnapshotResult {
        manifest_sha256: sha256_file(&manifest_path)?,
        manifest_path,
        confirmed_existing: true,
    })
}

fn file_matches(path: &Path, expected: &[u8]) -> Result<bool> {
    let file = File::open(path).map_err(|error| AssuranceError::io(path, error))?;
    let maximum = u64::try_from(expected.len())
        .map_err(|_| AssuranceError::Invalid("snapshot file exceeds addressable size".to_owned()))?
        .saturating_add(1);
    let mut observed = Vec::with_capacity(expected.len().saturating_add(1));
    file.take(maximum)
        .read_to_end(&mut observed)
        .map_err(|error| AssuranceError::io(path, error))?;
    Ok(observed == expected)
}

fn validate_snapshot_layout(target: &Path, expected: &BTreeMap<PathBuf, Vec<u8>>) -> Result<()> {
    let mut root_entries = fs::read_dir(target)
        .map_err(|error| AssuranceError::io(target, error))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|error| AssuranceError::io(target, error))?;
    root_entries.sort_by_key(std::fs::DirEntry::file_name);
    let names = root_entries
        .iter()
        .map(std::fs::DirEntry::file_name)
        .collect::<BTreeSet<_>>();
    let expected_names = ["files", "manifest.json"]
        .into_iter()
        .map(std::ffi::OsString::from)
        .collect::<BTreeSet<_>>();
    if names != expected_names {
        return Err(AssuranceError::SnapshotConflict(format!(
            "ID '{}' contains unexpected root entries",
            snapshot_name(target)?.to_string_lossy()
        )));
    }
    for entry in &root_entries {
        let kind = entry
            .file_type()
            .map_err(|error| AssuranceError::io(entry.path(), error))?;
        let valid = match entry.file_name().to_str() {
            Some("files") => kind.is_dir(),
            Some("manifest.json") => kind.is_file(),
            _ => false,
        };
        if !valid {
            return Err(AssuranceError::SnapshotConflict(format!(
                "ID '{}' contains a symlink or invalid root entry",
                snapshot_name(target)?.to_string_lossy()
            )));
        }
    }
    let mut observed = BTreeSet::new();
    collect_snapshot_files(&target.join("files"), Path::new(""), &mut observed)?;
    let expected = expected.keys().cloned().collect::<BTreeSet<_>>();
    if observed != expected {
        return Err(AssuranceError::SnapshotConflict(format!(
            "ID '{}' has an unexpected or missing generated file",
            snapshot_name(target)?.to_string_lossy()
        )));
    }
    Ok(())
}

fn collect_snapshot_files(
    directory: &Path,
    relative: &Path,
    files: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| AssuranceError::io(directory, error))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|error| AssuranceError::io(directory, error))?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let file_type = entry
            .file_type()
            .map_err(|error| AssuranceError::io(entry.path(), error))?;
        let child = relative.join(entry.file_name());
        if file_type.is_dir() {
            collect_snapshot_files(&entry.path(), &child, files)?;
        } else if file_type.is_file() {
            files.insert(child);
        } else {
            return Err(AssuranceError::SnapshotConflict(format!(
                "snapshot contains a non-file, non-directory entry: {}",
                entry.path().display()
            )));
        }
    }
    Ok(())
}

fn write_new_snapshot(
    root: &Path,
    target: &Path,
    files: &BTreeMap<PathBuf, Vec<u8>>,
    manifest: &[u8],
) -> Result<SnapshotResult> {
    let id = snapshot_name(target)?.to_string_lossy();
    let staging = create_staging_directory(root, &id)?;
    let result = (|| {
        for (relative, bytes) in files {
            let output = safe_output(
                &staging,
                &Path::new("files").join(relative),
                "snapshot file",
            )?;
            write_file(&output, bytes)?;
        }
        let staged_manifest = staging.join("manifest.json");
        write_file(&staged_manifest, manifest)?;
        fs::rename(&staging, target).map_err(|error| AssuranceError::io(target, error))?;
        let manifest_path = target.join("manifest.json");
        Ok(SnapshotResult {
            manifest_sha256: sha256_file(&manifest_path)?,
            manifest_path,
            confirmed_existing: false,
        })
    })();
    if result.is_err() && staging.exists() {
        fs::remove_dir_all(&staging).map_err(|error| AssuranceError::io(&staging, error))?;
    }
    result
}

fn create_staging_directory(root: &Path, snapshot_id: &str) -> Result<PathBuf> {
    for attempt in 0..100_u32 {
        let staging = root.join(format!(
            ".{snapshot_id}.tmp-{}-{attempt}",
            std::process::id()
        ));
        match fs::create_dir(&staging) {
            Ok(()) => return Ok(staging),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(AssuranceError::io(&staging, error)),
        }
    }
    Err(AssuranceError::Invalid(format!(
        "could not reserve an exclusive staging directory for snapshot '{snapshot_id}'"
    )))
}

fn snapshot_name(path: &Path) -> Result<&std::ffi::OsStr> {
    path.file_name().ok_or_else(|| {
        AssuranceError::Invalid(format!("snapshot path has no ID: {}", path.display()))
    })
}

fn write_file(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().ok_or_else(|| {
        AssuranceError::Invalid(format!("output has no parent: {}", path.display()))
    })?;
    create_dir_all_no_symlinks(parent, "snapshot output parent")?;
    fs::write(path, bytes).map_err(|error| AssuranceError::io(path, error))
}

#[derive(Serialize)]
struct SnapshotManifest<'a> {
    schema_version: u32,
    snapshot_id: &'a str,
    tool_version: &'static str,
    contract_version: u32,
    catalog_sha256: &'a str,
    tool_source_sha256: &'a str,
    dossiers: Vec<SnapshotDossier>,
    public_files: Vec<SnapshotFile>,
}

#[derive(Serialize)]
struct SnapshotDossier {
    dossier_id: String,
    version: String,
    lifecycle: &'static str,
    empirical: &'static str,
    scientific_root_sha256: String,
    publication_root_sha256: String,
}

#[derive(Serialize)]
struct SnapshotFile {
    path: String,
    sha256: String,
}
