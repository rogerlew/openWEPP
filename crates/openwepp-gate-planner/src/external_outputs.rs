//! Confined external-output roots and exhaustive regular-file manifests.

use std::collections::BTreeSet;
use std::fs::{self, File, Metadata};
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::canonical::{canonical_bytes, parse_strict, validate_schema};
use crate::error::{ErrorClass, GatePolicyError, Result};

pub const EXTERNAL_OUTPUT_MANIFEST_SCHEMA: &str = "openwepp-external-output-manifest-v1";

/// One hash-bound regular file below an external attempt root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputEntry {
    pub path: String,
    pub file_type: String,
    pub size: u64,
    pub sha256: String,
}

/// Exhaustive, path-sorted inventory of files below an external attempt root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputManifest {
    pub schema: String,
    pub root: String,
    pub entries: Vec<OutputEntry>,
    pub manifest_id: String,
}

/// Exclusively create a fresh, no-follow attempt root.
///
/// # Errors
///
/// Returns an error when the root or one of its ancestors violates the
/// no-follow contract, or when the root cannot be created exclusively.
pub fn prepare_attempt_root(root: &Path) -> Result<()> {
    prepare_attempt_root_with_outputs(root, &[])
}

/// Exclusively create a fresh attempt root and its confined output parents.
///
/// Declared paths are root-relative. Their final components must not exist,
/// declarations may not overlap, and every existing ancestor is checked
/// without following symlinks.
///
/// # Errors
///
/// Returns an error for an unsafe root, invalid or overlapping declarations,
/// an existing output, or an I/O failure.
pub fn prepare_attempt_root_with_outputs(root: &Path, declared: &[PathBuf]) -> Result<()> {
    validate_absolute_root(root)?;
    let parent = root
        .parent()
        .ok_or_else(|| output_error("GATE-EXTERNAL-ROOT", root))?;
    validate_existing_directory_chain(parent)?;
    fs::create_dir(root).map_err(|error| {
        let code = if error.kind() == std::io::ErrorKind::AlreadyExists {
            "GATE-EXTERNAL-ROOT-COLLISION"
        } else {
            "GATE-EXTERNAL-ROOT-CREATE"
        };
        GatePolicyError::new(ErrorClass::Io, code, error.to_string())
    })?;

    let prepared = (|| {
        let declarations = normalized_declarations_allow_empty(declared)?;
        for relative in &declarations {
            let output = root.join(relative);
            if fs::symlink_metadata(&output).is_ok() {
                return Err(output_error("GATE-EXTERNAL-OUTPUT-COLLISION", &output));
            }
            let parent = output
                .parent()
                .ok_or_else(|| output_error("GATE-EXTERNAL-OUTPUT-PATH", &output))?;
            create_confined_parents(root, parent)?;
        }
        Ok(())
    })();
    if prepared.is_err() {
        let _ = fs::remove_dir_all(root);
    }
    prepared
}

/// Build an exhaustive manifest for all and only the declared outputs.
///
/// # Errors
///
/// Returns an error for missing or undeclared outputs, links, special files,
/// path escapes, non-UTF-8 paths, I/O failures, or observed mutation.
pub fn manifest_declared_outputs(root: &Path, declared: &[PathBuf]) -> Result<OutputManifest> {
    let declarations = normalized_declarations(declared)?;
    let root_before = checked_root_metadata(root)?;
    let root_text = utf8_path(root, "GATE-EXTERNAL-ROOT-UTF8")?;
    let mut entries = Vec::new();
    let mut observed_declarations = BTreeSet::new();
    walk_outputs(
        root,
        root,
        &declarations,
        &mut observed_declarations,
        &mut entries,
    )?;
    for declaration in &declarations {
        if !observed_declarations.contains(declaration) {
            return Err(output_error(
                "GATE-EXTERNAL-OUTPUT-MISSING",
                &root.join(declaration),
            ));
        }
    }
    entries.sort_by(|left, right| left.path.cmp(&right.path));
    ensure_root_unchanged(root, &root_before)?;
    let manifest_id = manifest_digest(&root_text, &entries)?;
    let manifest = OutputManifest {
        schema: EXTERNAL_OUTPUT_MANIFEST_SCHEMA.to_owned(),
        root: root_text,
        entries,
        manifest_id,
    };
    validate_manifest_schema(&manifest)?;
    Ok(manifest)
}

/// Independently reconstruct and compare an external-output manifest.
///
/// # Errors
///
/// Returns an error when reconstruction fails or any bound manifest field
/// differs.
pub fn verify_manifest(root: &Path, declared: &[PathBuf], expected: &OutputManifest) -> Result<()> {
    validate_manifest_schema(expected)?;
    if expected.schema != EXTERNAL_OUTPUT_MANIFEST_SCHEMA {
        return Err(GatePolicyError::new(
            ErrorClass::Schema,
            "GATE-EXTERNAL-MANIFEST-SCHEMA",
            expected.schema.clone(),
        ));
    }
    let reconstructed = manifest_declared_outputs(root, declared)?;
    if &reconstructed != expected {
        return Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-EXTERNAL-MANIFEST-MISMATCH",
            "external output bytes or topology differ from the bound manifest",
        ));
    }
    Ok(())
}

fn validate_manifest_schema(manifest: &OutputManifest) -> Result<()> {
    let schema = parse_strict(include_bytes!(
        "../../../gate-policy/v1/schemas/external-output-manifest.schema.json"
    ))?;
    let value = serde_json::to_value(manifest).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Json,
            "GATE-EXTERNAL-MANIFEST-SERIALIZE",
            error.to_string(),
        )
    })?;
    validate_schema(&schema, &value, "external-output-manifest")
}

fn walk_outputs(
    root: &Path,
    directory: &Path,
    declarations: &[PathBuf],
    observed: &mut BTreeSet<PathBuf>,
    entries: &mut Vec<OutputEntry>,
) -> Result<()> {
    let relative_directory = confined_relative(root, directory)?;
    if !directory_allowed(&relative_directory, declarations) {
        return Err(output_error("GATE-EXTERNAL-OUTPUT-UNDECLARED", directory));
    }
    let before = fs::symlink_metadata(directory)
        .map_err(|error| io_error("GATE-EXTERNAL-OUTPUT-METADATA", &error))?;
    reject_non_directory(directory, &before)?;
    mark_observed(&relative_directory, declarations, observed);

    let mut children = fs::read_dir(directory)
        .map_err(|error| io_error("GATE-EXTERNAL-OUTPUT-READDIR", &error))?
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|error| io_error("GATE-EXTERNAL-OUTPUT-READDIR", &error))?;
    children.sort_by_key(fs::DirEntry::file_name);
    for child in children {
        let path = child.path();
        let relative = confined_relative(root, &path)?;
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| io_error("GATE-EXTERNAL-OUTPUT-METADATA", &error))?;
        if metadata.file_type().is_symlink() {
            return Err(output_error("GATE-EXTERNAL-OUTPUT-SYMLINK", &path));
        }
        if metadata.is_dir() {
            walk_outputs(root, &path, declarations, observed, entries)?;
        } else if metadata.is_file() {
            if !file_declared(&relative, declarations) {
                return Err(output_error("GATE-EXTERNAL-OUTPUT-UNDECLARED", &path));
            }
            mark_observed(&relative, declarations, observed);
            entries.push(hash_regular_file(root, &path)?);
        } else {
            return Err(output_error("GATE-EXTERNAL-OUTPUT-SPECIAL", &path));
        }
    }
    ensure_same_identity(directory, &before, "GATE-EXTERNAL-OUTPUT-DIRECTORY-MUTATED")
}

fn hash_regular_file(root: &Path, path: &Path) -> Result<OutputEntry> {
    #[cfg(unix)]
    let mut file = {
        use rustix::fs::{Mode, OFlags, open};
        let descriptor = open(
            path,
            OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
            Mode::empty(),
        )
        .map_err(|error| {
            GatePolicyError::new(
                ErrorClass::Io,
                "GATE-EXTERNAL-OUTPUT-OPEN",
                error.to_string(),
            )
        })?;
        File::from(descriptor)
    };
    #[cfg(not(unix))]
    let mut file =
        File::open(path).map_err(|error| io_error("GATE-EXTERNAL-OUTPUT-OPEN", &error))?;

    let before = file
        .metadata()
        .map_err(|error| io_error("GATE-EXTERNAL-OUTPUT-METADATA", &error))?;
    reject_non_regular_or_linked(path, &before)?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 64 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| io_error("GATE-EXTERNAL-OUTPUT-READ", &error))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let after = file
        .metadata()
        .map_err(|error| io_error("GATE-EXTERNAL-OUTPUT-METADATA", &error))?;
    if !same_identity(&before, &after) {
        return Err(output_error("GATE-EXTERNAL-OUTPUT-MUTATED", path));
    }
    reject_non_regular_or_linked(path, &after)?;
    Ok(OutputEntry {
        path: utf8_path(&confined_relative(root, path)?, "GATE-EXTERNAL-OUTPUT-UTF8")?,
        file_type: "REGULAR".to_owned(),
        size: after.len(),
        sha256: format!("{:x}", hasher.finalize()),
    })
}

fn normalized_declarations(declared: &[PathBuf]) -> Result<Vec<PathBuf>> {
    if declared.is_empty() {
        return Err(GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-EXTERNAL-OUTPUT-EMPTY",
            "at least one external output must be declared",
        ));
    }
    let mut normalized = Vec::with_capacity(declared.len());
    for path in declared {
        validate_relative(path)?;
        if normalized
            .iter()
            .any(|other: &PathBuf| path.starts_with(other) || other.starts_with(path))
        {
            return Err(output_error("GATE-EXTERNAL-OUTPUT-OVERLAP", path));
        }
        normalized.push(path.clone());
    }
    normalized.sort();
    Ok(normalized)
}

fn normalized_declarations_allow_empty(declared: &[PathBuf]) -> Result<Vec<PathBuf>> {
    if declared.is_empty() {
        Ok(Vec::new())
    } else {
        normalized_declarations(declared)
    }
}

fn validate_relative(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(output_error("GATE-EXTERNAL-OUTPUT-PATH", path));
    }
    utf8_path(path, "GATE-EXTERNAL-OUTPUT-UTF8").map(|_| ())
}

fn validate_absolute_root(root: &Path) -> Result<()> {
    if !root.is_absolute()
        || root
            .components()
            .any(|component| matches!(component, Component::ParentDir | Component::CurDir))
    {
        return Err(output_error("GATE-EXTERNAL-ROOT", root));
    }
    Ok(())
}

fn validate_existing_directory_chain(path: &Path) -> Result<()> {
    let mut current = PathBuf::new();
    for component in path.components() {
        current.push(component.as_os_str());
        let metadata = fs::symlink_metadata(&current)
            .map_err(|error| io_error("GATE-EXTERNAL-ROOT-ANCESTOR", &error))?;
        reject_non_directory(&current, &metadata)?;
    }
    Ok(())
}

fn create_confined_parents(root: &Path, directory: &Path) -> Result<()> {
    let relative = confined_relative(root, directory)?;
    let mut current = root.to_owned();
    for component in relative.components() {
        let Component::Normal(name) = component else {
            return Err(output_error("GATE-EXTERNAL-OUTPUT-PATH", directory));
        };
        current.push(name);
        match fs::symlink_metadata(&current) {
            Ok(metadata) => reject_non_directory(&current, &metadata)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                fs::create_dir(&current)
                    .map_err(|error| io_error("GATE-EXTERNAL-OUTPUT-DIRECTORY", &error))?;
            }
            Err(error) => {
                return Err(io_error("GATE-EXTERNAL-OUTPUT-DIRECTORY", &error));
            }
        }
    }
    Ok(())
}

fn checked_root_metadata(root: &Path) -> Result<Metadata> {
    validate_absolute_root(root)?;
    validate_existing_directory_chain(root)?;
    let metadata = fs::symlink_metadata(root)
        .map_err(|error| io_error("GATE-EXTERNAL-ROOT-METADATA", &error))?;
    reject_non_directory(root, &metadata)?;
    Ok(metadata)
}

fn reject_non_directory(path: &Path, metadata: &Metadata) -> Result<()> {
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(output_error("GATE-EXTERNAL-OUTPUT-DIRECTORY-TYPE", path));
    }
    Ok(())
}

fn reject_non_regular_or_linked(path: &Path, metadata: &Metadata) -> Result<()> {
    if !metadata.is_file() {
        return Err(output_error("GATE-EXTERNAL-OUTPUT-FILE-TYPE", path));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() != 1 {
            return Err(output_error("GATE-EXTERNAL-OUTPUT-HARDLINK", path));
        }
    }
    Ok(())
}

fn mark_observed(path: &Path, declarations: &[PathBuf], observed: &mut BTreeSet<PathBuf>) {
    for declaration in declarations {
        if path == declaration || path.starts_with(declaration) {
            observed.insert(declaration.clone());
        }
    }
}

fn directory_allowed(path: &Path, declarations: &[PathBuf]) -> bool {
    path.as_os_str().is_empty()
        || declarations
            .iter()
            .any(|declaration| declaration.starts_with(path) || path.starts_with(declaration))
}

fn file_declared(path: &Path, declarations: &[PathBuf]) -> bool {
    declarations
        .iter()
        .any(|declaration| path == declaration || path.starts_with(declaration))
}

fn confined_relative<'a>(root: &'a Path, path: &'a Path) -> Result<PathBuf> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| output_error("GATE-EXTERNAL-OUTPUT-ESCAPE", path))?;
    if relative
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(output_error("GATE-EXTERNAL-OUTPUT-ESCAPE", path));
    }
    Ok(relative.to_owned())
}

fn manifest_digest(root: &str, entries: &[OutputEntry]) -> Result<String> {
    let value = json!({
        "schema": EXTERNAL_OUTPUT_MANIFEST_SCHEMA,
        "root": root,
        "entries": entries,
    });
    Ok(format!("{:x}", Sha256::digest(canonical_bytes(&value)?)))
}

fn ensure_root_unchanged(root: &Path, before: &Metadata) -> Result<()> {
    ensure_same_identity(root, before, "GATE-EXTERNAL-ROOT-MUTATED")
}

fn ensure_same_identity(path: &Path, before: &Metadata, code: &'static str) -> Result<()> {
    let after = fs::symlink_metadata(path)
        .map_err(|error| io_error("GATE-EXTERNAL-OUTPUT-METADATA", &error))?;
    if !same_identity(before, &after) {
        return Err(output_error(code, path));
    }
    Ok(())
}

#[cfg(unix)]
fn same_identity(left: &Metadata, right: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    left.dev() == right.dev()
        && left.ino() == right.ino()
        && left.len() == right.len()
        && left.mtime() == right.mtime()
        && left.mtime_nsec() == right.mtime_nsec()
        && left.ctime() == right.ctime()
        && left.ctime_nsec() == right.ctime_nsec()
}

#[cfg(not(unix))]
fn same_identity(left: &Metadata, right: &Metadata) -> bool {
    left.len() == right.len() && left.modified().ok() == right.modified().ok()
}

fn utf8_path(path: &Path, code: &'static str) -> Result<String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| output_error(code, path))
}

fn io_error(code: &'static str, error: &std::io::Error) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Io, code, error.to_string())
}

fn output_error(code: &'static str, path: &Path) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Execution, code, path.display().to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{
        EXTERNAL_OUTPUT_MANIFEST_SCHEMA, manifest_declared_outputs,
        prepare_attempt_root_with_outputs, verify_manifest,
    };

    static NEXT: AtomicU64 = AtomicU64::new(0);

    struct Scratch(PathBuf);

    impl Scratch {
        fn new(label: &str) -> Self {
            let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("target")
                .join(format!(
                    "external-output-{label}-{}-{sequence}",
                    std::process::id()
                ));
            fs::create_dir_all(&path).expect("create scratch");
            Self(path.canonicalize().expect("canonical scratch"))
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).expect("remove scratch");
        }
    }

    fn prepared(label: &str, declared: &[PathBuf]) -> (Scratch, PathBuf) {
        let scratch = Scratch::new(label);
        let root = scratch.0.join("attempt");
        prepare_attempt_root_with_outputs(&root, declared).expect("prepare attempt");
        (scratch, root)
    }

    #[test]
    fn manifest_is_exhaustive_sorted_and_verifiable() {
        let declared = vec![
            PathBuf::from("objects/results"),
            PathBuf::from("logs/run.log"),
        ];
        let (_scratch, root) = prepared("happy", &declared);
        fs::create_dir_all(root.join("objects/results/nested")).expect("result directories");
        fs::write(root.join("objects/results/z.csv"), b"z").expect("z");
        fs::write(root.join("objects/results/nested/a.csv"), b"a").expect("a");
        fs::write(root.join("logs/run.log"), b"log").expect("log");

        let manifest = manifest_declared_outputs(&root, &declared).expect("manifest");
        assert_eq!(manifest.schema, EXTERNAL_OUTPUT_MANIFEST_SCHEMA);
        assert_eq!(
            manifest
                .entries
                .iter()
                .map(|entry| entry.path.as_str())
                .collect::<Vec<_>>(),
            vec![
                "logs/run.log",
                "objects/results/nested/a.csv",
                "objects/results/z.csv"
            ]
        );
        assert!(
            manifest
                .entries
                .iter()
                .all(|entry| entry.file_type == "REGULAR")
        );
        assert_eq!(manifest.manifest_id.len(), 64);
        verify_manifest(&root, &declared, &manifest).expect("verify");

        fs::write(root.join("objects/results/z.csv"), b"changed").expect("mutate");
        assert_eq!(
            verify_manifest(&root, &declared, &manifest)
                .expect_err("mutation must fail")
                .code,
            "GATE-EXTERNAL-MANIFEST-MISMATCH"
        );
    }

    #[test]
    fn preparation_rejects_root_collisions_path_escape_and_overlaps() {
        let scratch = Scratch::new("prepare-reject");
        let collision = scratch.0.join("attempt");
        fs::create_dir(&collision).expect("collision");
        assert_eq!(
            prepare_attempt_root_with_outputs(&collision, &[PathBuf::from("out")])
                .expect_err("root collision")
                .code,
            "GATE-EXTERNAL-ROOT-COLLISION"
        );

        let escape = scratch.0.join("escape");
        assert_eq!(
            prepare_attempt_root_with_outputs(&escape, &[PathBuf::from("../outside")])
                .expect_err("path escape")
                .code,
            "GATE-EXTERNAL-OUTPUT-PATH"
        );

        let overlap = scratch.0.join("overlap");
        assert_eq!(
            prepare_attempt_root_with_outputs(
                &overlap,
                &[PathBuf::from("out"), PathBuf::from("out/nested")]
            )
            .expect_err("nested declarations")
            .code,
            "GATE-EXTERNAL-OUTPUT-OVERLAP"
        );
        assert!(!overlap.exists());
    }

    #[test]
    fn manifest_rejects_missing_and_undeclared_outputs() {
        let declared = vec![PathBuf::from("objects/result.csv")];
        let (_scratch, root) = prepared("coverage", &declared);
        assert_eq!(
            manifest_declared_outputs(&root, &declared)
                .expect_err("missing output")
                .code,
            "GATE-EXTERNAL-OUTPUT-MISSING"
        );
        fs::write(root.join("objects/result.csv"), b"result").expect("result");
        fs::write(root.join("surprise"), b"undeclared").expect("surprise");
        assert_eq!(
            manifest_declared_outputs(&root, &declared)
                .expect_err("undeclared output")
                .code,
            "GATE-EXTERNAL-OUTPUT-UNDECLARED"
        );
    }

    #[cfg(unix)]
    #[test]
    fn manifest_rejects_symlinks_hardlinks_and_special_files() {
        use std::os::unix::fs::symlink;
        use std::process::Command;

        let declared = vec![PathBuf::from("objects")];
        let (_scratch, root) = prepared("types", &declared);
        fs::create_dir(root.join("objects")).expect("objects");
        fs::write(root.join("objects/file"), b"bytes").expect("file");
        symlink("file", root.join("objects/link")).expect("symlink");
        assert_eq!(
            manifest_declared_outputs(&root, &declared)
                .expect_err("symlink")
                .code,
            "GATE-EXTERNAL-OUTPUT-SYMLINK"
        );
        fs::remove_file(root.join("objects/link")).expect("remove link");
        fs::hard_link(root.join("objects/file"), root.join("objects/second-name"))
            .expect("hardlink");
        assert_eq!(
            manifest_declared_outputs(&root, &declared)
                .expect_err("hardlink")
                .code,
            "GATE-EXTERNAL-OUTPUT-HARDLINK"
        );
        fs::remove_file(root.join("objects/second-name")).expect("remove hardlink");
        let fifo = root.join("objects/fifo");
        let status = Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .expect("run mkfifo");
        assert!(status.success());
        assert_eq!(
            manifest_declared_outputs(&root, &declared)
                .expect_err("special file")
                .code,
            "GATE-EXTERNAL-OUTPUT-SPECIAL"
        );
    }

    #[cfg(unix)]
    #[test]
    fn preparation_rejects_symlinked_parent() {
        use std::os::unix::fs::symlink;

        let scratch = Scratch::new("parent-link");
        let real = scratch.0.join("real");
        fs::create_dir(&real).expect("real");
        let linked = scratch.0.join("linked");
        symlink(&real, &linked).expect("parent symlink");
        let error =
            prepare_attempt_root_with_outputs(&linked.join("attempt"), &[PathBuf::from("out")])
                .expect_err("symlink parent");
        assert_eq!(error.code, "GATE-EXTERNAL-OUTPUT-DIRECTORY-TYPE");
        assert!(!real.join("attempt").exists());
    }

    #[test]
    fn helper_paths_are_absolute_in_tests() {
        let scratch = Scratch::new("absolute");
        assert!(Path::new(&scratch.0).is_absolute());
    }
}
