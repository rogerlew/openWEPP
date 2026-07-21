use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;
use serde_json::Value;

use crate::canonical::{parse_strict, sha256_bytes};
use crate::error::{ErrorClass, GatePolicyError, Result};

static SNAPSHOT_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ObservedChange {
    pub path: String,
    pub change_kind: String,
    pub object_kind: String,
    pub old_mode: Option<String>,
    pub new_mode: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ObservedSource {
    pub base_commit: String,
    pub head_commit: Option<String>,
    pub dirty_tree_digest: Option<String>,
    pub index_digest: Option<String>,
    pub worktree_digest: Option<String>,
    pub untracked_digest: Option<String>,
    pub changes: Vec<ObservedChange>,
}

struct DirtyGitState {
    index: Vec<u8>,
    worktree: Vec<u8>,
    combined: Vec<u8>,
    untracked: Vec<u8>,
}

struct UntrackedState {
    changes: Vec<ObservedChange>,
    manifest: Vec<u8>,
}

struct DirtyIdentities {
    dirty: String,
    index: String,
    worktree: String,
    untracked: String,
}

struct UntrackedObject {
    mode: &'static str,
    kind: &'static str,
    content: Vec<u8>,
}

/// Resolve a revision to its exact commit object ID.
///
/// # Errors
///
/// Returns a Git-state error when the revision is absent or ambiguous.
pub fn resolve_commit(repo: &Path, revision: &str) -> Result<String> {
    let output = git(
        repo,
        &["rev-parse", "--verify", &format!("{revision}^{{commit}}")],
    )?;
    utf8_stdout(&output, "GATE-GIT-REVISION").map(|value| value.trim().to_owned())
}

/// Observe a canonical committed base/head change set.
///
/// # Errors
///
/// Returns a Git-state error for ambiguous state, paths, modes, or commands.
pub fn observe_committed(repo: &Path, base: &str, head: &str) -> Result<ObservedSource> {
    let base_commit = admitted_base(repo, base)?;
    let head_commit = resolve_commit(repo, head)?;
    require_exact_clean_checkout(repo, &head_commit)?;
    observe_committed_commits(repo, base_commit, head_commit)
}

/// Observe a committed base/head change set after execution reported a source mutation.
///
/// This deliberately reconstructs the immutable commit range without requiring the
/// checkout to remain clean. The verifier separately validates the reported dirty
/// post-execution snapshot and INVALID disposition.
pub(crate) fn observe_committed_after_mutation(
    repo: &Path,
    base: &str,
    head: &str,
) -> Result<ObservedSource> {
    let base_commit = admitted_base(repo, base)?;
    let head_commit = resolve_commit(repo, head)?;
    observe_committed_commits(repo, base_commit, head_commit)
}

fn observe_committed_commits(
    repo: &Path,
    base_commit: String,
    head_commit: String,
) -> Result<ObservedSource> {
    let raw = committed_diff(repo, &base_commit, &head_commit)?;
    let changes = parse_raw_changes(&raw)?;
    Ok(ObservedSource {
        base_commit,
        head_commit: Some(head_commit),
        dirty_tree_digest: None,
        index_digest: None,
        worktree_digest: None,
        untracked_digest: None,
        changes,
    })
}

fn require_exact_clean_checkout(repo: &Path, expected_head: &str) -> Result<()> {
    let actual_head = resolve_commit(repo, "HEAD")?;
    let status = git(
        repo,
        &[
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--",
        ],
    )?;
    if actual_head == expected_head && status.stdout.is_empty() {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-COMMITTED-CHECKOUT-NOT-EXACT",
            "committed planning and verification require a clean checkout at the named head",
        ))
    }
}

fn admitted_base(repo: &Path, base: &str) -> Result<String> {
    reject_ambiguous_worktree(repo)?;
    resolve_commit(repo, base)
}

fn committed_diff(repo: &Path, base_commit: &str, head_commit: &str) -> Result<Vec<u8>> {
    git(
        repo,
        &[
            "diff",
            "--raw",
            "-z",
            "--no-renames",
            "--no-ext-diff",
            base_commit,
            head_commit,
            "--",
        ],
    )
    .map(|output| output.stdout)
}

/// Observe separate index, worktree, and untracked manifests from a base commit.
///
/// # Errors
///
/// Returns a Git-state error for ambiguous or unsupported repository state.
pub fn observe_dirty(repo: &Path, base: &str) -> Result<ObservedSource> {
    let base_commit = admitted_base(repo, base)?;
    let git_state = read_dirty_git_state(repo, &base_commit)?;
    let untracked = dirty_changes(repo, &git_state)?;
    let identities = dirty_identities(repo, &git_state, &untracked)?;
    Ok(ObservedSource {
        base_commit,
        head_commit: None,
        dirty_tree_digest: Some(identities.dirty),
        index_digest: Some(identities.index),
        worktree_digest: Some(identities.worktree),
        untracked_digest: Some(identities.untracked),
        changes: untracked.changes,
    })
}

fn read_dirty_git_state(repo: &Path, base_commit: &str) -> Result<DirtyGitState> {
    let index = git(
        repo,
        &[
            "diff",
            "--cached",
            "--raw",
            "-z",
            "--no-renames",
            "--no-ext-diff",
            base_commit,
            "--",
        ],
    )?;
    let worktree = git(
        repo,
        &["diff", "--raw", "-z", "--no-renames", "--no-ext-diff", "--"],
    )?;
    let combined = git(
        repo,
        &[
            "diff",
            "--raw",
            "-z",
            "--no-renames",
            "--no-ext-diff",
            base_commit,
            "--",
        ],
    )?;
    let untracked = git(repo, &["ls-files", "--others", "--exclude-standard", "-z"])?;
    Ok(DirtyGitState {
        index: index.stdout,
        worktree: worktree.stdout,
        combined: combined.stdout,
        untracked: untracked.stdout,
    })
}

fn dirty_changes(repo: &Path, state: &DirtyGitState) -> Result<UntrackedState> {
    let mut changes = parse_raw_changes(&state.combined)?;
    let untracked = parse_untracked(repo, &state.untracked)?;
    changes.extend(untracked.changes);
    changes.sort_by(|left, right| left.path.as_bytes().cmp(right.path.as_bytes()));
    reject_duplicate_changes(&changes)?;
    Ok(UntrackedState {
        changes,
        manifest: untracked.manifest,
    })
}

fn parse_untracked(repo: &Path, raw: &[u8]) -> Result<UntrackedState> {
    let mut changes = Vec::new();
    let mut untracked_manifest = Vec::new();
    for path_bytes in raw.split(|byte| *byte == 0).filter(|part| !part.is_empty()) {
        let (change, manifest_entry) = untracked_entry(repo, path_bytes)?;
        changes.push(change);
        untracked_manifest.extend(manifest_entry);
    }
    Ok(UntrackedState {
        changes,
        manifest: untracked_manifest,
    })
}

fn untracked_entry(repo: &Path, path_bytes: &[u8]) -> Result<(ObservedChange, Vec<u8>)> {
    let path = untracked_path(path_bytes)?;
    validate_repo_path(path)?;
    let object = read_untracked_object(&repo.join(path), path)?;
    let manifest = untracked_manifest_entry(path, &object);
    Ok((
        ObservedChange {
            path: path.to_owned(),
            change_kind: "ADD".to_owned(),
            object_kind: object.kind.to_owned(),
            old_mode: None,
            new_mode: Some(object.mode.to_owned()),
        },
        manifest,
    ))
}

fn untracked_path(path_bytes: &[u8]) -> Result<&str> {
    std::str::from_utf8(path_bytes).map_err(|_| {
        GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-NONUTF8-PATH",
            "untracked path is not UTF-8",
        )
    })
}

fn read_untracked_object(absolute: &Path, path: &str) -> Result<UntrackedObject> {
    let metadata = fs::symlink_metadata(absolute).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Io,
            "GATE-UNTRACKED-METADATA",
            format!("{path}: {error}"),
        )
    })?;
    if metadata.file_type().is_symlink() {
        return read_untracked_symlink(absolute, path);
    }
    if metadata.is_file() {
        return read_untracked_regular(absolute, path, &metadata);
    }
    Err(GatePolicyError::new(
        ErrorClass::GitState,
        "GATE-GIT-UNSUPPORTED-OBJECT",
        format!("unsupported untracked object: {path}"),
    ))
}

fn read_untracked_symlink(absolute: &Path, path: &str) -> Result<UntrackedObject> {
    let target = fs::read_link(absolute).map_err(|error| {
        GatePolicyError::new(ErrorClass::Io, "GATE-SYMLINK-READ", error.to_string())
    })?;
    let target = target.to_str().ok_or_else(|| {
        GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-NONUTF8-SYMLINK",
            format!("symlink target is not UTF-8: {path}"),
        )
    })?;
    Ok(UntrackedObject {
        mode: "120000",
        kind: "SYMLINK",
        content: target.as_bytes().to_vec(),
    })
}

fn read_untracked_regular(
    absolute: &Path,
    path: &str,
    metadata: &fs::Metadata,
) -> Result<UntrackedObject> {
    let content = fs::read(absolute).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Io,
            "GATE-UNTRACKED-READ",
            format!("{path}: {error}"),
        )
    })?;
    Ok(UntrackedObject {
        mode: untracked_regular_mode(metadata)?,
        kind: "REGULAR",
        content,
    })
}

#[cfg(unix)]
// Keep the cross-platform Result contract: non-Unix hosts must fail closed
// because executable mode cannot be reconstructed authoritatively there.
#[allow(clippy::unnecessary_wraps)]
fn untracked_regular_mode(metadata: &fs::Metadata) -> Result<&'static str> {
    use std::os::unix::fs::PermissionsExt;
    if metadata.permissions().mode() & 0o111 == 0 {
        Ok("100644")
    } else {
        Ok("100755")
    }
}

#[cfg(not(unix))]
fn untracked_regular_mode(_metadata: &fs::Metadata) -> Result<&'static str> {
    Err(GatePolicyError::new(
        ErrorClass::GitState,
        "GATE-GIT-MODE-UNAVAILABLE",
        "untracked executable mode is unavailable on this platform",
    ))
}

fn untracked_manifest_entry(path: &str, object: &UntrackedObject) -> Vec<u8> {
    let mut entry = Vec::new();
    entry.extend_from_slice(path.as_bytes());
    entry.push(0);
    entry.extend_from_slice(object.mode.as_bytes());
    entry.push(0);
    entry.extend_from_slice(sha256_bytes(&object.content).as_bytes());
    entry.push(0);
    entry
}

fn reject_duplicate_changes(changes: &[ObservedChange]) -> Result<()> {
    let mut seen = BTreeSet::new();
    if changes
        .iter()
        .any(|change| !seen.insert(change.path.clone()))
    {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-DUPLICATE-PATH",
            "combined dirty state contains a duplicate path",
        ));
    }
    Ok(())
}

fn dirty_identities(
    repo: &Path,
    state: &DirtyGitState,
    untracked: &UntrackedState,
) -> Result<DirtyIdentities> {
    let working_content_manifest = working_content_manifest(repo, &untracked.changes)?;
    let index = sha256_bytes(&state.index);
    let mut worktree_identity = state.worktree.clone();
    worktree_identity.extend_from_slice(&working_content_manifest);
    let worktree = sha256_bytes(&worktree_identity);
    let untracked_digest = sha256_bytes(&untracked.manifest);
    let dirty = dirty_tree_identity(&state.index, &worktree_identity, &untracked.manifest);
    Ok(DirtyIdentities {
        dirty: sha256_bytes(&dirty),
        index,
        worktree,
        untracked: untracked_digest,
    })
}

fn working_content_manifest(repo: &Path, changes: &[ObservedChange]) -> Result<Vec<u8>> {
    let mut working_content_manifest = Vec::new();
    for change in changes {
        working_content_manifest.extend_from_slice(change.path.as_bytes());
        working_content_manifest.push(0);
        working_content_manifest.extend_from_slice(&working_content_identity(repo, change)?);
        working_content_manifest.push(0);
    }
    Ok(working_content_manifest)
}

fn working_content_identity(repo: &Path, change: &ObservedChange) -> Result<Vec<u8>> {
    if change.change_kind == "DELETE" {
        return Ok(b"DELETED".to_vec());
    }
    read_worktree_content(&repo.join(&change.path), &change.path)
        .map(|content| sha256_bytes(&content).into_bytes())
}

fn read_worktree_content(absolute: &Path, path: &str) -> Result<Vec<u8>> {
    let metadata = fs::symlink_metadata(absolute).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Io,
            "GATE-WORKTREE-CONTENT",
            format!("{path}: {error}"),
        )
    })?;
    if metadata.file_type().is_symlink() {
        return read_worktree_symlink(absolute, path);
    }
    fs::read(absolute).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Io,
            "GATE-WORKTREE-CONTENT",
            format!("{path}: {error}"),
        )
    })
}

fn read_worktree_symlink(absolute: &Path, path: &str) -> Result<Vec<u8>> {
    fs::read_link(absolute)
        .and_then(|target| {
            target
                .to_str()
                .map(|value| value.as_bytes().to_vec())
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "non-UTF-8 link target")
                })
        })
        .map_err(|error| {
            GatePolicyError::new(
                ErrorClass::Io,
                "GATE-WORKTREE-SYMLINK",
                format!("{path}: {error}"),
            )
        })
}

fn dirty_tree_identity(index: &[u8], worktree: &[u8], untracked: &[u8]) -> Vec<u8> {
    let mut dirty = Vec::new();
    dirty.extend_from_slice(b"index\0");
    dirty.extend_from_slice(index);
    dirty.extend_from_slice(b"worktree\0");
    dirty.extend_from_slice(worktree);
    dirty.extend_from_slice(b"untracked\0");
    dirty.extend_from_slice(untracked);
    dirty
}

fn reject_ambiguous_worktree(repo: &Path) -> Result<()> {
    reject_sparse_checkout(repo)?;
    reject_unmerged_entries(repo)?;
    reject_intent_to_add(repo)?;
    reject_hidden_index_entries(repo)
}

fn reject_hidden_index_entries(repo: &Path) -> Result<()> {
    let listed = git(repo, &["ls-files", "-v", "-z"])?;
    let hidden = listed
        .stdout
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
        .any(|entry| entry[0] == b'S' || entry[0].is_ascii_lowercase());
    if hidden {
        Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-HIDDEN-INDEX-ENTRY",
            "assume-unchanged and skip-worktree entries block planning",
        ))
    } else {
        Ok(())
    }
}

fn reject_sparse_checkout(repo: &Path) -> Result<()> {
    let output = sparse_checkout_output(repo)?;
    validate_sparse_checkout(&output)
}

fn sparse_checkout_output(repo: &Path) -> Result<Output> {
    neutral_git_command()
        .args(["config", "--bool", "--get", "core.sparseCheckout"])
        .current_dir(repo)
        .output()
        .map_err(|error| GatePolicyError::new(ErrorClass::Io, "GATE-GIT-SPAWN", error.to_string()))
}

fn validate_sparse_checkout(output: &Output) -> Result<()> {
    if output.status.success() {
        return reject_enabled_sparse_checkout(&output.stdout);
    }
    if output.status.code() == Some(1) {
        return Ok(());
    }
    Err(GatePolicyError::new(
        ErrorClass::GitState,
        "GATE-GIT-SPARSE-STATE",
        String::from_utf8_lossy(&output.stderr).trim().to_owned(),
    ))
}

fn reject_enabled_sparse_checkout(stdout: &[u8]) -> Result<()> {
    if stdout == b"true\n" {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-SPARSE",
            "sparse worktrees are not admitted by gate-policy/v1",
        ));
    }
    Ok(())
}

fn reject_unmerged_entries(repo: &Path) -> Result<()> {
    let unmerged = git(repo, &["diff", "--name-only", "--diff-filter=U", "-z"])?;
    if !unmerged.stdout.is_empty() {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-UNMERGED",
            "unmerged index entries block planning",
        ));
    }
    Ok(())
}

fn reject_intent_to_add(repo: &Path) -> Result<()> {
    let staged = git(repo, &["ls-files", "--debug"])?;
    if staged
        .stdout
        .split(|byte| *byte == b'\n')
        .any(is_intent_to_add)
    {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-INTENT-TO-ADD",
            "intent-to-add index entries block planning",
        ));
    }
    Ok(())
}

fn is_intent_to_add(line: &[u8]) -> bool {
    index_flags(line).is_some_and(|flags| flags & 0x2000_0000 != 0)
}

fn index_flags(line: &[u8]) -> Option<u32> {
    let text = std::str::from_utf8(line).ok()?;
    let (_, flags) = text.rsplit_once("flags: ")?;
    u32::from_str_radix(flags.trim(), 16).ok()
}

struct RawHeader<'a> {
    old_mode: &'a str,
    new_mode: &'a str,
    status: &'a str,
}

fn parse_raw_changes(raw: &[u8]) -> Result<Vec<ObservedChange>> {
    let fields = raw
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if fields.len() % 2 != 0 {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-RAW-SHAPE",
            "raw Git diff did not contain header/path pairs",
        ));
    }
    let mut changes = Vec::new();
    for pair in fields.chunks_exact(2) {
        changes.push(parse_raw_change(pair)?);
    }
    changes.sort_by(|left, right| left.path.as_bytes().cmp(right.path.as_bytes()));
    Ok(changes)
}

fn parse_raw_change(pair: &[&[u8]]) -> Result<ObservedChange> {
    let (header, path) = raw_pair_text(pair)?;
    let header = raw_header(header)?;
    build_raw_change(path, &header)
}

fn raw_pair_text<'a>(pair: &'a [&'a [u8]]) -> Result<(&'a str, &'a str)> {
    let header = std::str::from_utf8(pair[0]).map_err(|_| {
        GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-RAW-UTF8",
            "raw diff header is not UTF-8",
        )
    })?;
    let path = std::str::from_utf8(pair[1]).map_err(|_| {
        GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-NONUTF8-PATH",
            "changed path is not UTF-8",
        )
    })?;
    validate_repo_path(path)?;
    Ok((header, path))
}

fn raw_header(header: &str) -> Result<RawHeader<'_>> {
    let parts = header.split_ascii_whitespace().collect::<Vec<_>>();
    if parts.len() != 5 || !parts[0].starts_with(':') {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-RAW-HEADER",
            format!("invalid raw diff header: {header}"),
        ));
    }
    Ok(RawHeader {
        old_mode: &parts[0][1..],
        new_mode: parts[1],
        status: parts[4],
    })
}

fn build_raw_change(path: &str, header: &RawHeader<'_>) -> Result<ObservedChange> {
    reject_submodule(header.old_mode, header.new_mode, path)?;
    let change_kind = raw_change_kind(header.status)?;
    let object_kind = raw_object_kind(effective_mode(header), path)?;
    Ok(ObservedChange {
        path: path.to_owned(),
        change_kind: change_kind.to_owned(),
        object_kind: object_kind.to_owned(),
        old_mode: optional_mode(header.old_mode),
        new_mode: optional_mode(header.new_mode),
    })
}

fn reject_submodule(old_mode: &str, new_mode: &str, path: &str) -> Result<()> {
    if old_mode == "160000" || new_mode == "160000" {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-SUBMODULE",
            format!("submodule change blocks planning: {path}"),
        ));
    }
    Ok(())
}

fn raw_change_kind(status: &str) -> Result<&'static str> {
    status_byte(status).and_then(|byte| classify_status(byte, status))
}

fn status_byte(status: &str) -> Result<u8> {
    status.as_bytes().first().copied().ok_or_else(|| {
        GatePolicyError::new(ErrorClass::GitState, "GATE-GIT-STATUS", "empty Git status")
    })
}

fn classify_status(byte: u8, status: &str) -> Result<&'static str> {
    match byte {
        b'A' => Ok("ADD"),
        b'D' => Ok("DELETE"),
        b'M' => Ok("MODIFY"),
        b'T' => Ok("TYPE_CHANGE"),
        _ => Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-STATUS",
            format!("unsupported Git change status: {status}"),
        )),
    }
}

fn effective_mode<'a>(header: &RawHeader<'a>) -> &'a str {
    if header.new_mode == "000000" {
        header.old_mode
    } else {
        header.new_mode
    }
}

fn raw_object_kind(mode: &str, path: &str) -> Result<&'static str> {
    match mode {
        "100644" | "100755" => Ok("REGULAR"),
        "120000" => Ok("SYMLINK"),
        _ => Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-MODE",
            format!("unsupported Git mode {mode}: {path}"),
        )),
    }
}

fn optional_mode(mode: &str) -> Option<String> {
    (mode != "000000").then(|| mode.to_owned())
}

fn validate_repo_path(path: &str) -> Result<()> {
    if path.is_empty()
        || path.starts_with('/')
        || path
            .split('/')
            .any(|component| component == "." || component == ".." || component.is_empty())
    {
        return Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-PATH",
            format!("invalid repository path: {path}"),
        ));
    }
    Ok(())
}

fn git(repo: &Path, arguments: &[&str]) -> Result<Output> {
    let output = neutral_git_command()
        .args(arguments)
        .current_dir(repo)
        .output()
        .map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-GIT-SPAWN", error.to_string())
        })?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-COMMAND",
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ))
    }
}

pub(crate) fn neutral_git_command() -> Command {
    let mut command = Command::new("git");
    for variable in [
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_INDEX_FILE",
        "GIT_COMMON_DIR",
        "GIT_OBJECT_DIRECTORY",
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
        "GIT_NAMESPACE",
        "GIT_REPLACE_REF_BASE",
        "GIT_CONFIG_COUNT",
        "GIT_CONFIG_PARAMETERS",
        "GIT_ATTR_SOURCE",
    ] {
        command.env_remove(variable);
    }
    command
        .arg("--no-replace-objects")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_NO_REPLACE_OBJECTS", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .args([
            "-c",
            "core.quotepath=false",
            "-c",
            "diff.external=",
            "-c",
            "core.excludesFile=/dev/null",
            "-c",
            "core.attributesFile=/dev/null",
        ]);
    command
}

fn utf8_stdout(output: &Output, code: &'static str) -> Result<String> {
    String::from_utf8(output.stdout.clone())
        .map_err(|error| GatePolicyError::new(ErrorClass::GitState, code, error.to_string()))
}

#[derive(Debug, Clone, Default)]
pub struct CargoGraph {
    package_dirs: BTreeMap<String, String>,
    reverse: BTreeMap<String, BTreeSet<String>>,
}

impl CargoGraph {
    /// Read locked/offline Cargo metadata for the current worktree.
    ///
    /// # Errors
    ///
    /// Returns a metadata error when Cargo or its graph is unavailable.
    pub fn load_current(repo: &Path) -> Result<Self> {
        metadata(repo).and_then(|bytes| Self::from_metadata(&bytes, repo))
    }

    /// Read locked/offline Cargo metadata from an isolated committed snapshot.
    ///
    /// # Errors
    ///
    /// Returns a snapshot or metadata error when reconstruction fails.
    pub fn load_at_commit(repo: &Path, commit: &str) -> Result<Self> {
        let snapshot = Snapshot::create(repo, commit)?;
        metadata(&snapshot.path).and_then(|bytes| Self::from_metadata(&bytes, &snapshot.path))
    }

    pub(crate) fn load_at_commit_in(repo: &Path, commit: &str, parent: &Path) -> Result<Self> {
        let snapshot = Snapshot::create_in(repo, commit, parent)?;
        metadata(&snapshot.path).and_then(|bytes| Self::from_metadata(&bytes, &snapshot.path))
    }

    /// Decode the workspace-only package and dependency graph from Cargo JSON.
    ///
    /// # Errors
    ///
    /// Returns a metadata error for incomplete or out-of-root workspace records.
    pub fn from_metadata(bytes: &[u8], root: &Path) -> Result<Self> {
        let metadata = parse_strict(bytes)?;
        let normalized_root = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
        let packages = metadata["packages"]
            .as_array()
            .ok_or_else(|| metadata_error("packages"))?;
        let workspace_members = metadata["workspace_members"]
            .as_array()
            .ok_or_else(|| metadata_error("workspace_members"))?
            .iter()
            .filter_map(Value::as_str)
            .collect::<BTreeSet<_>>();
        let mut by_id = BTreeMap::new();
        let mut package_dirs = BTreeMap::new();
        for package in packages {
            let id = string_field(package, "id")?;
            if !workspace_members.contains(id) {
                continue;
            }
            reject_undeclared_feature_projection(package)?;
            let name = string_field(package, "name")?;
            let manifest = Path::new(string_field(package, "manifest_path")?);
            let directory = manifest
                .parent()
                .ok_or_else(|| metadata_error("manifest parent"))?;
            let normalized_directory =
                fs::canonicalize(directory).unwrap_or_else(|_| directory.to_path_buf());
            let relative = normalized_directory
                .strip_prefix(&normalized_root)
                .map_err(|_| metadata_error("manifest outside snapshot"))?;
            let relative = relative
                .to_str()
                .ok_or_else(|| metadata_error("non-UTF8 manifest path"))?
                .replace('\\', "/");
            by_id.insert(id.to_owned(), name.to_owned());
            package_dirs.insert(name.to_owned(), relative);
        }
        let mut reverse: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let nodes = metadata
            .pointer("/resolve/nodes")
            .and_then(Value::as_array)
            .ok_or_else(|| metadata_error("resolve.nodes"))?;
        for node in nodes {
            let Some(dependent) = by_id.get(string_field(node, "id")?).cloned() else {
                continue;
            };
            for dependency in node["deps"]
                .as_array()
                .ok_or_else(|| metadata_error("node deps"))?
            {
                if let Some(dependency_name) = by_id.get(string_field(dependency, "pkg")?) {
                    reverse
                        .entry(dependency_name.clone())
                        .or_default()
                        .insert(dependent.clone());
                }
            }
        }
        Ok(Self {
            package_dirs,
            reverse,
        })
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut union = self.clone();
        union.package_dirs.extend(other.package_dirs.clone());
        for (dependency, dependents) in &other.reverse {
            union
                .reverse
                .entry(dependency.clone())
                .or_default()
                .extend(dependents.clone());
        }
        union
    }

    #[must_use]
    pub fn package_for_path(&self, path: &str) -> Option<String> {
        self.package_dirs
            .iter()
            .filter(|(name, directory)| {
                if directory.is_empty() {
                    *name == "openwepp"
                        && (path == "Cargo.toml"
                            || path.starts_with("src/")
                            || path.starts_with("tests/"))
                } else {
                    path == *directory || path.starts_with(&format!("{directory}/"))
                }
            })
            .max_by_key(|(_, directory)| directory.len())
            .map(|(name, _)| name.clone())
    }

    #[must_use]
    pub fn reverse_closure(&self, initial: &BTreeSet<String>) -> BTreeSet<String> {
        let mut result = initial.clone();
        let mut queue = initial.iter().cloned().collect::<VecDeque<_>>();
        while let Some(package) = queue.pop_front() {
            if let Some(dependents) = self.reverse.get(&package) {
                for dependent in dependents {
                    if result.insert(dependent.clone()) {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }
        result
    }

    pub fn packages(&self) -> impl Iterator<Item = &String> {
        self.package_dirs.keys()
    }
}

fn reject_undeclared_feature_projection(package: &Value) -> Result<()> {
    let features = package["features"]
        .as_object()
        .ok_or_else(|| metadata_error("package features"))?;
    if features.keys().all(|feature| feature == "default") {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::CargoMetadata,
            "GATE-CARGO-FEATURE-MATRIX-UNDECLARED",
            "non-default Cargo features require a versioned supported projection matrix",
        ))
    }
}

fn metadata(repo: &Path) -> Result<Vec<u8>> {
    let target = host_target_triple(repo)?;
    let output = neutral_cargo_command()
        .args([
            "metadata",
            "--locked",
            "--offline",
            "--config",
            "net.offline=true",
            "--format-version",
            "1",
            "--filter-platform",
            &target,
        ])
        .current_dir(repo)
        .output()
        .map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-CARGO-SPAWN", error.to_string())
        })?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(GatePolicyError::new(
            ErrorClass::CargoMetadata,
            "GATE-CARGO-METADATA",
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ))
    }
}

pub(crate) fn neutral_cargo_command() -> Command {
    let mut command = Command::new("cargo");
    for variable in [
        "CARGO_LLVM_COV",
        "CARGO_LLVM_COV_BUILD_DIR",
        "CARGO_LLVM_COV_SHOW_ENV",
        "CARGO_LLVM_COV_TARGET_DIR",
        "CARGO_BUILD_RUSTFLAGS",
        "CARGO_ENCODED_RUSTFLAGS",
        "RUSTFLAGS",
        "RUSTDOCFLAGS",
        "RUSTC_WRAPPER",
        "RUSTC_WORKSPACE_WRAPPER",
        "CARGO_BUILD_TARGET",
        "CARGO_TARGET_DIR",
        "LLVM_PROFILE_FILE",
        "NEXTEST_PROFILE",
        "__CARGO_LLVM_COV_RUSTC_WRAPPER",
        "__CARGO_LLVM_COV_RUSTC_WRAPPER_CRATE_NAMES",
        "__CARGO_LLVM_COV_RUSTC_WRAPPER_RUSTFLAGS",
    ] {
        command.env_remove(variable);
    }
    command
}

pub(crate) fn host_target_triple(repo: &Path) -> Result<String> {
    let output = Command::new("rustc")
        .args(["-vV"])
        .current_dir(repo)
        .output()
        .map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-RUSTC-SPAWN", error.to_string())
        })?;
    if !output.status.success() {
        return Err(GatePolicyError::new(
            ErrorClass::CargoMetadata,
            "GATE-RUSTC-HOST",
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ));
    }
    let text = String::from_utf8(output.stdout).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::CargoMetadata,
            "GATE-RUSTC-HOST",
            error.to_string(),
        )
    })?;
    text.lines()
        .find_map(|line| line.strip_prefix("host: "))
        .map(str::to_owned)
        .ok_or_else(|| metadata_error("rustc host triple"))
}

fn string_field<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value[field].as_str().ok_or_else(|| metadata_error(field))
}

fn metadata_error(field: &str) -> GatePolicyError {
    GatePolicyError::new(
        ErrorClass::CargoMetadata,
        "GATE-CARGO-METADATA-SHAPE",
        format!("missing or invalid Cargo metadata field: {field}"),
    )
}

pub(crate) fn remove_reconstruction_workspace(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            Err(GatePolicyError::new(
                ErrorClass::Io,
                "GATE-EXEC-RECONSTRUCTION-CLEANUP",
                path.display().to_string(),
            ))
        }
        Ok(_) => fs::remove_dir_all(path).map_err(|error| {
            GatePolicyError::new(
                ErrorClass::Io,
                "GATE-EXEC-RECONSTRUCTION-CLEANUP",
                error.to_string(),
            )
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(GatePolicyError::new(
            ErrorClass::Io,
            "GATE-EXEC-RECONSTRUCTION-CLEANUP",
            error.to_string(),
        )),
    }
}

pub(crate) struct Snapshot {
    path: PathBuf,
}

impl Snapshot {
    pub(crate) fn create(repo: &Path, commit: &str) -> Result<Self> {
        Self::create_in(repo, commit, &std::env::temp_dir())
    }

    pub(crate) fn create_in(repo: &Path, commit: &str, parent: &Path) -> Result<Self> {
        fs::create_dir_all(parent).map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-SNAPSHOT-CREATE", error.to_string())
        })?;
        let suffix = SNAPSHOT_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!("openwepp-gate-{}-{suffix}", std::process::id()));
        fs::create_dir(&path).map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-SNAPSHOT-CREATE", error.to_string())
        })?;
        let archive = path.join("snapshot.tar");
        let archive_arg = archive.to_str().ok_or_else(|| {
            GatePolicyError::new(
                ErrorClass::Io,
                "GATE-SNAPSHOT-PATH",
                "temporary path is not UTF-8",
            )
        })?;
        if let Err(error) = git(
            repo,
            &[
                "archive",
                "--format=tar",
                &format!("--output={archive_arg}"),
                commit,
            ],
        ) {
            let _cleanup = fs::remove_dir_all(&path);
            return Err(error);
        }
        let output = Command::new("tar")
            .args(["-xf", "snapshot.tar"])
            .current_dir(&path)
            .output()
            .map_err(|error| {
                GatePolicyError::new(ErrorClass::Io, "GATE-SNAPSHOT-EXTRACT", error.to_string())
            })?;
        if !output.status.success() {
            let _cleanup = fs::remove_dir_all(&path);
            return Err(GatePolicyError::new(
                ErrorClass::Io,
                "GATE-SNAPSHOT-EXTRACT",
                String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            ));
        }
        fs::remove_file(&archive).map_err(|error| {
            GatePolicyError::new(
                ErrorClass::Io,
                "GATE-SNAPSHOT-ARCHIVE-CLEANUP",
                error.to_string(),
            )
        })?;
        Ok(Self { path })
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        let _cleanup = fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    use super::{
        CargoGraph, neutral_cargo_command, observe_committed, observe_dirty, parse_raw_changes,
        resolve_commit,
    };

    #[test]
    fn neutral_cargo_command_removes_coverage_instrumentation_environment() {
        let command = neutral_cargo_command();
        let removed = command
            .get_envs()
            .filter_map(|(name, value)| value.is_none().then_some(name))
            .collect::<BTreeSet<_>>();
        for variable in [
            "CARGO_LLVM_COV",
            "CARGO_LLVM_COV_BUILD_DIR",
            "CARGO_LLVM_COV_SHOW_ENV",
            "CARGO_LLVM_COV_TARGET_DIR",
            "LLVM_PROFILE_FILE",
            "RUSTC_WRAPPER",
            "__CARGO_LLVM_COV_RUSTC_WRAPPER",
            "__CARGO_LLVM_COV_RUSTC_WRAPPER_CRATE_NAMES",
            "__CARGO_LLVM_COV_RUSTC_WRAPPER_RUSTFLAGS",
        ] {
            assert!(
                removed.contains(std::ffi::OsStr::new(variable)),
                "{variable}"
            );
        }
    }

    struct TestRepository {
        path: PathBuf,
    }

    impl TestRepository {
        fn create() -> (Self, String) {
            let path = std::env::temp_dir().join(format!(
                "openwepp-gate-repository-test-{}-{}",
                std::process::id(),
                super::SNAPSHOT_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ));
            fs::create_dir(&path).expect("create test repository");
            git(&path, &["init", "--quiet"]);
            git(
                &path,
                &["config", "user.email", "gate-planner@example.invalid"],
            );
            git(&path, &["config", "user.name", "Gate Planner Test"]);
            fs::write(path.join("tracked.txt"), b"base\n").expect("write tracked fixture");
            git(&path, &["add", "tracked.txt"]);
            git(&path, &["commit", "--quiet", "-m", "base"]);
            let base = resolve_commit(&path, "HEAD").expect("resolve base commit");
            (Self { path }, base)
        }
    }

    impl Drop for TestRepository {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.path).expect("remove test repository");
        }
    }

    fn git(repo: &Path, arguments: &[&str]) {
        let output = Command::new("git")
            .args(arguments)
            .current_dir(repo)
            .output()
            .expect("run git fixture command");
        assert!(
            output.status.success(),
            "git {arguments:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn raw_git_changes_preserve_modes_and_disable_rename_semantics() {
        let raw =
            b":100644 100755 aaaa bbbb M\0crates/x/src/lib.rs\0:000000 120000 0000 cccc A\0link\0";
        let changes = parse_raw_changes(raw).expect("parse raw changes");
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].new_mode.as_deref(), Some("100755"));
        assert_eq!(changes[1].object_kind, "SYMLINK");
    }

    #[test]
    fn raw_git_changes_reject_ambiguous_shape_status_mode_and_path() {
        let cases: [(&[u8], &str); 4] = [
            (b":100644 100644 aaaa bbbb M\0", "GATE-GIT-RAW-SHAPE"),
            (
                b":100644 100644 aaaa bbbb R100\0renamed.txt\0",
                "GATE-GIT-STATUS",
            ),
            (b":100644 040000 aaaa bbbb T\0tree\0", "GATE-GIT-MODE"),
            (b":100644 100644 aaaa bbbb M\0../escape\0", "GATE-GIT-PATH"),
        ];
        for (raw, expected_code) in cases {
            let error = parse_raw_changes(raw).expect_err("ambiguous raw change must fail");
            assert_eq!(error.code, expected_code);
        }
    }

    #[test]
    fn committed_and_dirty_observation_bind_exact_repository_content() {
        let (repo, base) = TestRepository::create();
        fs::write(repo.path.join("tracked.txt"), b"committed\n").expect("update tracked file");
        git(&repo.path, &["add", "tracked.txt"]);
        git(&repo.path, &["commit", "--quiet", "-m", "committed change"]);
        let head = resolve_commit(&repo.path, "HEAD").expect("resolve head commit");
        let committed = observe_committed(&repo.path, &base, &head).expect("observe committed");
        assert_eq!(committed.changes.len(), 1);
        assert_eq!(committed.changes[0].path, "tracked.txt");

        fs::write(repo.path.join("tracked.txt"), b"dirty\n").expect("dirty tracked file");
        fs::write(repo.path.join("untracked.txt"), b"first\n").expect("write untracked file");
        let first = observe_dirty(&repo.path, &head).expect("first dirty observation");
        assert_eq!(
            first
                .changes
                .iter()
                .map(|change| change.path.as_str())
                .collect::<Vec<_>>(),
            ["tracked.txt", "untracked.txt"]
        );

        fs::write(repo.path.join("untracked.txt"), b"second\n").expect("change untracked file");
        let second = observe_dirty(&repo.path, &head).expect("second dirty observation");
        assert_ne!(first.untracked_digest, second.untracked_digest);
        assert_ne!(first.worktree_digest, second.worktree_digest);
        assert_ne!(first.dirty_tree_digest, second.dirty_tree_digest);
        assert_eq!(first.index_digest, second.index_digest);

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let path = repo.path.join("untracked.txt");
            let mut permissions = fs::metadata(&path)
                .expect("untracked metadata")
                .permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&path, permissions).expect("make untracked executable");
            let executable = observe_dirty(&repo.path, &head).expect("executable observation");
            let change = executable
                .changes
                .iter()
                .find(|change| change.path == "untracked.txt")
                .expect("untracked change");
            assert_eq!(change.new_mode.as_deref(), Some("100755"));
            assert_ne!(second.untracked_digest, executable.untracked_digest);
        }
    }

    #[test]
    fn dirty_observation_rejects_intent_to_add() {
        let (repo, base) = TestRepository::create();
        fs::write(repo.path.join("tentative.txt"), b"tentative\n").expect("write tentative file");
        git(&repo.path, &["add", "--intent-to-add", "tentative.txt"]);
        let error = observe_dirty(&repo.path, &base).expect_err("intent-to-add must fail");
        assert_eq!(error.code, "GATE-GIT-INTENT-TO-ADD");
    }

    #[test]
    fn cargo_graph_expands_reverse_dependencies() {
        let root = std::path::Path::new("/repo");
        let metadata = br#"{"packages":[{"id":"a 0.1","name":"a","manifest_path":"/repo/crates/a/Cargo.toml","features":{}},{"id":"b 0.1","name":"b","manifest_path":"/repo/crates/b/Cargo.toml","features":{}}],"workspace_members":["a 0.1","b 0.1"],"resolve":{"nodes":[{"id":"a 0.1","deps":[]},{"id":"b 0.1","deps":[{"pkg":"a 0.1","dep_kinds":[{"kind":null,"target":null}]}]}]}}"#;
        let graph = CargoGraph::from_metadata(metadata, root).expect("metadata graph");
        let initial = std::collections::BTreeSet::from(["a".to_owned()]);
        assert_eq!(graph.reverse_closure(&initial).len(), 2);
        assert_eq!(
            graph.package_for_path("crates/a/src/lib.rs").as_deref(),
            Some("a")
        );
    }
}
