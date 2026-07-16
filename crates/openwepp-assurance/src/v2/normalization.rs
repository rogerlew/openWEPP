use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde::Serialize;

use super::confined::ConfinedDirectory;
use super::{
    DRAFT, Report, ReportSource, RequiredNullable, V2_CATALOG_PATH, V2Repository, parse_json,
    parse_yaml, read_regular_confined, validate_catalog_binding, validate_report_structure,
};
use crate::{AssuranceError, Result, sha256_bytes};

const LANGUAGE: &str = "en-US";
const V2_ROOT: &str = "assurance/v2";
const NEXT_ROOT: &str = "assurance/.v2.normalize.next";

/// Whether normalization is a read-only policy check or a source transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V2NormalizationMode {
    Check,
    Apply,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum V2NormalizationFault {
    AfterInstall,
    AfterExchangeSync,
    BeforeExchange,
    #[cfg(test)]
    StagedTreeDrift,
    #[cfg(test)]
    BeforeCleanup,
    #[cfg(test)]
    BeforeCandidateContentDrift,
    #[cfg(test)]
    AfterCandidateTreeDrift,
    #[cfg(test)]
    BeforeCatalogReadDrift,
}

/// Explicit options for the DRAFT source normalization operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2NormalizationOptions {
    language: String,
    mode: V2NormalizationMode,
}

impl V2NormalizationOptions {
    #[must_use]
    pub fn new(language: impl Into<String>, mode: V2NormalizationMode) -> Self {
        Self {
            language: language.into(),
            mode,
        }
    }
}

struct NormalizationControls {
    converter: OsString,
    fault_injection: Option<V2NormalizationFault>,
}

impl Default for NormalizationControls {
    fn default() -> Self {
        Self {
            converter: OsString::from("uk2us"),
            fault_injection: None,
        }
    }
}

/// One exact source identity changed by a normalization transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct V2NormalizationChange {
    pub path: PathBuf,
    pub old_sha256: String,
    pub new_sha256: String,
}

/// Deterministic receipt emitted by a check or successful transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct V2NormalizationReceipt {
    pub schema_version: u32,
    pub operation: V2NormalizationMode,
    pub language: String,
    pub converter: String,
    pub report_id: String,
    pub changed: bool,
    pub old_source_root_sha256: String,
    pub new_source_root_sha256: String,
    pub changes: Vec<V2NormalizationChange>,
}

impl V2NormalizationReceipt {
    /// Renders stable pretty JSON suitable for retaining as command evidence.
    ///
    /// # Errors
    ///
    /// Returns a typed serialization error if the receipt cannot be encoded.
    pub fn render_json(&self) -> Result<String> {
        let mut bytes = serde_json::to_vec_pretty(self).map_err(|error| {
            AssuranceError::Invalid(format!(
                "normalization receipt serialization failed: {error}"
            ))
        })?;
        bytes.push(b'\n');
        String::from_utf8(bytes).map_err(|error| {
            AssuranceError::Invalid(format!("normalization receipt was not UTF-8: {error}"))
        })
    }
}

struct Candidate {
    replacements: BTreeMap<PathBuf, Vec<u8>>,
    changes: Vec<V2NormalizationChange>,
}

struct PreparedNormalization {
    tree_before: TreeSnapshot,
    old_root: String,
    candidate: Candidate,
}

type DigestUpdate = (PathBuf, String, String);

struct NormalizedContent {
    replacements: BTreeMap<PathBuf, Vec<u8>>,
    digest_updates: Vec<DigestUpdate>,
}

struct AgentPacket {
    path: PathBuf,
    digest: String,
    bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TreeEntry {
    sha256: String,
    mode: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TreeSnapshot {
    root_mode: u32,
    directories: BTreeMap<PathBuf, u32>,
    files: BTreeMap<PathBuf, TreeEntry>,
}

pub(super) fn normalize_report(
    repository: &V2Repository,
    report_id: &str,
    options: &V2NormalizationOptions,
) -> Result<V2NormalizationReceipt> {
    normalize_report_with_controls(
        repository,
        report_id,
        options,
        &NormalizationControls::default(),
    )
}

fn normalize_report_with_controls(
    repository: &V2Repository,
    report_id: &str,
    options: &V2NormalizationOptions,
    controls: &NormalizationControls,
) -> Result<V2NormalizationReceipt> {
    validate_options(options)?;
    let transaction = ConfinedDirectory::open_ambient(&repository.root, false)?;
    transaction.lock_exclusive(Path::new("assurance normalization"))?;
    ensure_no_recovery_state(&transaction)?;
    let prepared = prepare_normalization(repository, report_id, controls, &transaction)?;
    if prepared.candidate.changes.is_empty() {
        return Ok(receipt(
            options,
            report_id,
            &prepared.old_root,
            prepared.old_root.clone(),
            Vec::new(),
        ));
    }
    if options.mode == V2NormalizationMode::Check {
        return Err(normalization_drift(report_id, &prepared.candidate));
    }
    apply_normalization(
        repository,
        report_id,
        options,
        controls,
        &transaction,
        prepared,
    )
}

fn prepare_normalization(
    repository: &V2Repository,
    report_id: &str,
    controls: &NormalizationControls,
    transaction: &ConfinedDirectory,
) -> Result<PreparedNormalization> {
    let tree_before = capture_v2_tree(transaction)?;
    let source = repository
        .sources
        .get(report_id)
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown v2 report ID '{report_id}'")))?;
    let before = repository.validate_report(report_id)?;
    let old_root = selected_root(&before, report_id)?;
    #[cfg(test)]
    if controls.fault_injection == Some(V2NormalizationFault::BeforeCandidateContentDrift) {
        append_test_drift(
            &repository
                .root
                .join("assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md"),
        )?;
    }
    let candidate = prepare_candidate(repository, source, controls, &tree_before)?;
    #[cfg(test)]
    if controls.fault_injection == Some(V2NormalizationFault::AfterCandidateTreeDrift) {
        append_test_drift(
            &repository
                .root
                .join("assurance/v2/schemas/catalog.schema.json"),
        )?;
    }
    let repeated = repository.validate_report(report_id)?;
    if repeated != before {
        return Err(AssuranceError::Drift(format!(
            "report '{report_id}' changed during normalization"
        )));
    }
    if capture_v2_tree(transaction)? != tree_before {
        return Err(AssuranceError::Drift(
            "assurance/v2 files or modes changed during normalization".to_owned(),
        ));
    }
    Ok(PreparedNormalization {
        tree_before,
        old_root,
        candidate,
    })
}

fn normalization_drift(report_id: &str, candidate: &Candidate) -> AssuranceError {
    let paths = candidate
        .changes
        .iter()
        .take(2)
        .map(|change| change.path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    AssuranceError::Drift(format!(
        "report '{report_id}' is not normalized for {LANGUAGE}: {paths}; rerun with --apply"
    ))
}

fn apply_normalization(
    repository: &V2Repository,
    report_id: &str,
    options: &V2NormalizationOptions,
    controls: &NormalizationControls,
    transaction: &ConfinedDirectory,
    prepared: PreparedNormalization,
) -> Result<V2NormalizationReceipt> {
    let PreparedNormalization {
        tree_before,
        old_root,
        candidate,
    } = prepared;
    repository.verify_inputs()?;
    install_transaction(
        transaction,
        repository,
        &candidate.replacements,
        &tree_before,
        controls.fault_injection,
    )?;
    if controls.fault_injection == Some(V2NormalizationFault::AfterInstall) {
        let error = AssuranceError::Invalid(
            "injected normalization failure after source installation".to_owned(),
        );
        return Err(combine_recovery(error, restore_previous(transaction)));
    }
    let new_root = match validate_installed(repository, report_id) {
        Ok(root) => root,
        Err(error) => {
            return Err(combine_recovery(error, restore_previous(transaction)));
        }
    };
    let committed = receipt(options, report_id, &old_root, new_root, candidate.changes);
    let receipt_json =
        committed
            .render_json()
            .map_err(|source| AssuranceError::CommittedCleanup {
                path: PathBuf::from(NEXT_ROOT),
                receipt_json: "receipt serialization failed".to_owned(),
                source: Box::new(source),
            })?;
    #[cfg(test)]
    if controls.fault_injection == Some(V2NormalizationFault::BeforeCleanup) {
        inject_cleanup_fault(&repository.root)?;
    }
    finish_transaction(transaction).map_err(|source| AssuranceError::CommittedCleanup {
        path: PathBuf::from(NEXT_ROOT),
        receipt_json,
        source: Box::new(source),
    })?;
    Ok(committed)
}

fn ensure_no_recovery_state(root: &ConfinedDirectory) -> Result<()> {
    if root.directory_exists(Path::new(NEXT_ROOT))? {
        Err(AssuranceError::Invalid(format!(
            "normalization recovery state requires explicit disposition before any new operation: {NEXT_ROOT}"
        )))
    } else {
        Ok(())
    }
}

fn validate_installed(repository: &V2Repository, report_id: &str) -> Result<String> {
    let summary = V2Repository::open(&repository.root)?.validate_report(report_id)?;
    selected_root(&summary, report_id)
}

fn validate_options(options: &V2NormalizationOptions) -> Result<()> {
    if options.language == LANGUAGE {
        Ok(())
    } else {
        Err(AssuranceError::Usage(format!(
            "assurance normalization supports only --language {LANGUAGE}"
        )))
    }
}

fn selected_root(summary: &super::V2ValidationSummary, report_id: &str) -> Result<String> {
    summary
        .reports
        .iter()
        .find(|report| report.id == report_id)
        .map(|report| report.source_root_sha256.clone())
        .ok_or_else(|| {
            AssuranceError::Invalid(format!(
                "normalization validation omitted report '{report_id}'"
            ))
        })
}

fn receipt(
    options: &V2NormalizationOptions,
    report_id: &str,
    old_root: &str,
    new_root: String,
    changes: Vec<V2NormalizationChange>,
) -> V2NormalizationReceipt {
    V2NormalizationReceipt {
        schema_version: 1,
        operation: options.mode,
        language: options.language.clone(),
        converter: "uk2us".to_owned(),
        report_id: report_id.to_owned(),
        changed: !changes.is_empty(),
        old_source_root_sha256: old_root.to_owned(),
        new_source_root_sha256: new_root,
        changes,
    }
}

fn prepare_candidate(
    repository: &V2Repository,
    source: &ReportSource,
    controls: &NormalizationControls,
    expected: &TreeSnapshot,
) -> Result<Candidate> {
    let (manifest_bytes, report) = load_normalizable_report(repository, source, expected)?;
    let normalized = normalize_content_sources(repository, &report, controls, expected)?;
    let packet = load_agent_packet(repository, &report, expected)?;
    if normalized.digest_updates.is_empty() {
        return Ok(Candidate {
            replacements: normalized.replacements,
            changes: Vec::new(),
        });
    }
    rebind_candidate(
        repository,
        source,
        controls,
        expected,
        manifest_bytes,
        normalized,
        packet,
    )
}

fn load_normalizable_report(
    repository: &V2Repository,
    source: &ReportSource,
    expected: &TreeSnapshot,
) -> Result<(Vec<u8>, Report)> {
    let manifest_bytes = read_snapshotted(repository, &source.manifest_path, expected)?;
    require_digest(
        &source.manifest_path,
        &manifest_bytes,
        &source.manifest_sha256,
    )?;
    let report: Report = parse_yaml(&source.manifest_path, &manifest_bytes)?;
    validate_catalog_binding(source, &report)?;
    validate_report_structure(&report)?;
    if report.lifecycle != DRAFT {
        return Err(AssuranceError::Invalid(format!(
            "report '{}' is {}; normalization is restricted to DRAFT sources",
            report.id, report.lifecycle
        )));
    }
    if report.agent_assistance.review_entry_authorized {
        return Err(AssuranceError::Invalid(format!(
            "report '{}' has authorized review entry; normalization is restricted to pre-review DRAFT sources",
            report.id
        )));
    }
    Ok((manifest_bytes, report))
}

fn load_agent_packet(
    repository: &V2Repository,
    report: &Report,
    expected: &TreeSnapshot,
) -> Result<AgentPacket> {
    let packet = report
        .dependencies
        .iter()
        .find(|dependency| dependency.id == report.agent_assistance.exact_output_dependency_id)
        .ok_or_else(|| AssuranceError::Invalid("agent output dependency is missing".to_owned()))?;
    let packet_path = required_path(&packet.path, "agent output dependency path")?;
    let packet_digest = required_string(&packet.sha256, "agent output dependency digest")?;
    let packet_bytes = read_snapshotted(repository, packet_path, expected)?;
    require_digest(packet_path, &packet_bytes, packet_digest)?;
    let packet_json: serde_json::Value = parse_json(packet_path, &packet_bytes)?;
    validate_draft_outputs(
        &packet_json,
        [&report.manuscript, &report.supplement]
            .map(|content| (content.path.as_path(), content.sha256.as_str())),
    )?;
    Ok(AgentPacket {
        path: packet_path.to_path_buf(),
        digest: packet_digest.to_owned(),
        bytes: packet_bytes,
    })
}

fn rebind_candidate(
    repository: &V2Repository,
    source: &ReportSource,
    controls: &NormalizationControls,
    expected: &TreeSnapshot,
    manifest_bytes: Vec<u8>,
    normalized: NormalizedContent,
    packet: AgentPacket,
) -> Result<Candidate> {
    #[cfg(not(test))]
    let _ = controls;
    let NormalizedContent {
        mut replacements,
        digest_updates,
    } = normalized;
    let packet_bytes =
        replace_json_draft_output_digests(packet.bytes, &digest_updates, "agent packet")?;
    let new_packet_digest = sha256_bytes(&packet_bytes);
    replacements.insert(packet.path.clone(), packet_bytes);

    let mut manifest_updates = digest_updates.clone();
    manifest_updates.push((packet.path, packet.digest, new_packet_digest));
    let new_manifest = replace_yaml_path_digests(
        manifest_bytes,
        &manifest_updates,
        "report descriptor",
        "path",
        "sha256",
    )?;
    let parsed_report: Report = parse_yaml(&source.manifest_path, &new_manifest)?;
    validate_report_structure(&parsed_report)?;
    let new_manifest_digest = sha256_bytes(&new_manifest);
    replacements.insert(source.manifest_path.clone(), new_manifest);

    let catalog_path = Path::new(V2_CATALOG_PATH);
    #[cfg(test)]
    if controls.fault_injection == Some(V2NormalizationFault::BeforeCatalogReadDrift) {
        append_test_drift(&repository.root.join(catalog_path))?;
    }
    let catalog_bytes = read_snapshotted(repository, catalog_path, expected)?;
    let catalog_updates = [(
        source.manifest_path.clone(),
        source.manifest_sha256.clone(),
        new_manifest_digest,
    )];
    let new_catalog = replace_yaml_path_digests(
        catalog_bytes,
        &catalog_updates,
        "catalog",
        "manifest_path",
        "manifest_sha256",
    )?;
    replacements.insert(catalog_path.to_path_buf(), new_catalog);

    let changes = candidate_changes(expected, &replacements)?;
    Ok(Candidate {
        replacements,
        changes,
    })
}

fn candidate_changes(
    expected: &TreeSnapshot,
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<Vec<V2NormalizationChange>> {
    replacements
        .iter()
        .map(|(path, bytes)| {
            snapshot_entry(expected, path).map(|old| V2NormalizationChange {
                path: path.clone(),
                old_sha256: old.sha256.clone(),
                new_sha256: sha256_bytes(bytes),
            })
        })
        .collect()
}

fn validate_draft_outputs(value: &serde_json::Value, outputs: [(&Path, &str); 2]) -> Result<()> {
    for (path, digest) in outputs {
        if count_draft_output(value, path, digest)? != 1
            || count_path_digest(value, path, digest) != 1
        {
            return Err(AssuranceError::Invalid(format!(
                "agent packet must bind exactly one current draft output: {}",
                path.display()
            )));
        }
    }
    Ok(())
}

fn count_path_digest(value: &serde_json::Value, path: &Path, digest: &str) -> usize {
    match value {
        serde_json::Value::Object(object) => {
            let own = usize::from(
                object.get("path").and_then(serde_json::Value::as_str) == path.to_str()
                    && object.get("sha256").and_then(serde_json::Value::as_str) == Some(digest),
            );
            own + object
                .values()
                .map(|child| count_path_digest(child, path, digest))
                .sum::<usize>()
        }
        serde_json::Value::Array(values) => values
            .iter()
            .map(|child| count_path_digest(child, path, digest))
            .sum(),
        _ => 0,
    }
}

fn normalize_content_sources(
    repository: &V2Repository,
    report: &Report,
    controls: &NormalizationControls,
    expected: &TreeSnapshot,
) -> Result<NormalizedContent> {
    let mut replacements = BTreeMap::new();
    let mut digest_updates = Vec::new();
    for content in [&report.manuscript, &report.supplement] {
        let current = read_snapshotted(repository, &content.path, expected)?;
        require_digest(&content.path, &current, &content.sha256)?;
        let normalized = run_converter(&controls.converter, &current)?;
        if run_converter(&controls.converter, &normalized)? != normalized {
            return Err(AssuranceError::Invalid(
                "uk2us normalization is not idempotent".to_owned(),
            ));
        }
        if normalized != current {
            digest_updates.push((
                content.path.clone(),
                content.sha256.clone(),
                sha256_bytes(&normalized),
            ));
            replacements.insert(content.path.clone(), normalized);
        }
    }
    Ok(NormalizedContent {
        replacements,
        digest_updates,
    })
}

fn required_path<'a>(value: &'a RequiredNullable<PathBuf>, name: &str) -> Result<&'a Path> {
    match value {
        RequiredNullable::Value(path) => Ok(path),
        RequiredNullable::Missing | RequiredNullable::Null => {
            Err(AssuranceError::Invalid(format!("{name} is required")))
        }
    }
}

fn required_string<'a>(value: &'a RequiredNullable<String>, name: &str) -> Result<&'a str> {
    match value {
        RequiredNullable::Value(value) => Ok(value),
        RequiredNullable::Missing | RequiredNullable::Null => {
            Err(AssuranceError::Invalid(format!("{name} is required")))
        }
    }
}

fn require_digest(path: &Path, bytes: &[u8], expected: &str) -> Result<()> {
    if sha256_bytes(bytes) == expected {
        Ok(())
    } else {
        Err(AssuranceError::Drift(format!(
            "normalization input changed before conversion: {}",
            path.display()
        )))
    }
}

fn run_converter(executable: &OsString, input: &[u8]) -> Result<Vec<u8>> {
    let mut child = Command::new(executable)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| AssuranceError::io(PathBuf::from(executable), error))?;
    let (Some(mut stdin), Some(stdout), Some(stderr)) =
        (child.stdin.take(), child.stdout.take(), child.stderr.take())
    else {
        let _ = child.kill();
        let _ = child.wait();
        return Err(AssuranceError::Invalid(
            "uk2us process pipes were unavailable".to_owned(),
        ));
    };
    let stdout_reader = std::thread::spawn(move || read_converter_pipe(stdout));
    let stderr_reader = std::thread::spawn(move || read_converter_pipe(stderr));
    let write_error = stdin.write_all(input).err();
    drop(stdin);
    let status = child.wait();
    let stdout = join_converter_pipe(stdout_reader, "uk2us stdout");
    let stderr = join_converter_pipe(stderr_reader, "uk2us stderr");
    let status = status.map_err(|error| AssuranceError::io("uk2us", error))?;
    if !status.success() {
        let detail = match &stderr {
            Ok(bytes) => String::from_utf8_lossy(bytes).trim().to_owned(),
            Err(error) => format!("stderr capture failed: {error}"),
        };
        return Err(AssuranceError::Invalid(format!(
            "uk2us failed with {status}: {detail}"
        )));
    }
    if let Some(error) = write_error {
        return Err(AssuranceError::io("uk2us stdin", error));
    }
    let stdout = stdout?;
    stderr?;
    std::str::from_utf8(&stdout).map_err(|error| {
        AssuranceError::Invalid(format!("uk2us produced non-UTF-8 output: {error}"))
    })?;
    Ok(stdout)
}

fn read_converter_pipe(mut pipe: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    pipe.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn join_converter_pipe(
    reader: std::thread::JoinHandle<std::io::Result<Vec<u8>>>,
    name: &str,
) -> Result<Vec<u8>> {
    match reader.join() {
        Ok(Ok(bytes)) => Ok(bytes),
        Ok(Err(error)) => Err(AssuranceError::io(name, error)),
        Err(_) => Err(AssuranceError::Invalid(format!(
            "{name} capture thread panicked"
        ))),
    }
}

fn read_snapshotted(
    repository: &V2Repository,
    path: &Path,
    expected: &TreeSnapshot,
) -> Result<Vec<u8>> {
    let bytes = read_regular_confined(&repository.root, path)?;
    let entry = snapshot_entry(expected, path)?;
    if sha256_bytes(&bytes) != entry.sha256 {
        return Err(AssuranceError::Drift(format!(
            "normalization input changed after snapshot: {}",
            path.display()
        )));
    }
    Ok(bytes)
}

fn snapshot_entry<'a>(expected: &'a TreeSnapshot, path: &Path) -> Result<&'a TreeEntry> {
    let relative = path.strip_prefix(V2_ROOT).map_err(|_| {
        AssuranceError::Invalid(format!(
            "normalization input escapes {V2_ROOT}: {}",
            path.display()
        ))
    })?;
    expected.files.get(relative).ok_or_else(|| {
        AssuranceError::Drift(format!(
            "normalization input was not in the held source snapshot: {}",
            path.display()
        ))
    })
}

fn count_draft_output(value: &serde_json::Value, path: &Path, digest: &str) -> Result<usize> {
    let object = value
        .as_object()
        .ok_or_else(|| AssuranceError::Invalid("agent packet must be a JSON object".to_owned()))?;
    if object
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        != Some(1)
    {
        return Err(AssuranceError::Invalid(
            "agent packet schema_version must be 1".to_owned(),
        ));
    }
    let outputs = object
        .get("draft_outputs")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| {
            AssuranceError::Invalid("agent packet draft_outputs must be an array".to_owned())
        })?;
    if outputs.len() != 2 {
        return Err(AssuranceError::Invalid(
            "agent packet must bind exactly the manuscript and supplement draft outputs".to_owned(),
        ));
    }
    let mut count = 0;
    for output in outputs {
        let output = output.as_object().ok_or_else(|| {
            AssuranceError::Invalid("agent packet draft output must be an object".to_owned())
        })?;
        if output.len() != 2 || !output.contains_key("path") || !output.contains_key("sha256") {
            return Err(AssuranceError::Invalid(
                "agent packet draft outputs allow only path and sha256".to_owned(),
            ));
        }
        count += usize::from(
            output.get("path").and_then(serde_json::Value::as_str) == path.to_str()
                && output.get("sha256").and_then(serde_json::Value::as_str) == Some(digest),
        );
    }
    Ok(count)
}

fn replace_yaml_path_digests(
    bytes: Vec<u8>,
    updates: &[(PathBuf, String, String)],
    owner: &str,
    path_key: &str,
    digest_key: &str,
) -> Result<Vec<u8>> {
    replace_path_bound_digests(bytes, updates, owner, |path| {
        (
            format!("{path_key}: {}", path.display()),
            format!("{digest_key}: "),
            String::new(),
        )
    })
}

fn replace_json_draft_output_digests(
    bytes: Vec<u8>,
    updates: &[(PathBuf, String, String)],
    owner: &str,
) -> Result<Vec<u8>> {
    let text = String::from_utf8(bytes)
        .map_err(|error| AssuranceError::Invalid(format!("{owner} is not UTF-8: {error}")))?;
    let (start, end) = unique_json_array_span(&text, "draft_outputs", owner)?;
    let replaced = replace_path_bound_digests(
        text.as_bytes()[start..end].to_vec(),
        updates,
        owner,
        |path| {
            (
                format!("\"path\": \"{}\",", path.display()),
                "\"sha256\": \"".to_owned(),
                "\"".to_owned(),
            )
        },
    )?;
    let mut result = text.into_bytes();
    result.splice(start..end, replaced);
    Ok(result)
}

fn unique_json_array_span(text: &str, key: &str, owner: &str) -> Result<(usize, usize)> {
    let needle = format!("\"{key}\"");
    let matches = text.match_indices(&needle).collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err(AssuranceError::Invalid(format!(
            "{owner} must contain exactly one {needle} key"
        )));
    }
    let key_end = matches[0].0 + needle.len();
    let tail = &text[key_end..];
    let colon = tail
        .find(':')
        .ok_or_else(|| AssuranceError::Invalid(format!("{owner} {needle} key has no value")))?;
    let array = tail[colon + 1..].find('[').ok_or_else(|| {
        AssuranceError::Invalid(format!("{owner} {needle} value is not an array"))
    })?;
    let start = key_end + colon + 1 + array;
    matching_json_array_end(text, start, owner).map(|end| (start, end))
}

fn matching_json_array_end(text: &str, start: usize, owner: &str) -> Result<usize> {
    let mut depth = 0_u32;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, byte) in text.as_bytes()[start..].iter().copied().enumerate() {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'[' => depth += 1,
            b']' if depth == 1 => return Ok(start + offset + 1),
            b']' => depth -= 1,
            _ => {}
        }
    }
    Err(AssuranceError::Invalid(format!(
        "{owner} draft_outputs array is unterminated"
    )))
}

fn replace_path_bound_digests<F>(
    bytes: Vec<u8>,
    updates: &[(PathBuf, String, String)],
    owner: &str,
    syntax: F,
) -> Result<Vec<u8>>
where
    F: Fn(&Path) -> (String, String, String),
{
    let mut text = String::from_utf8(bytes)
        .map_err(|error| AssuranceError::Invalid(format!("{owner} is not UTF-8: {error}")))?;
    for (path, old, new) in updates {
        let (path_line, digest_prefix, digest_suffix) = syntax(path);
        let lines = line_offsets(&text);
        let mut spans = Vec::new();
        for pair in lines.windows(2) {
            let (_, line) = pair[0];
            let (next_start, next_line) = pair[1];
            if line.trim() != path_line {
                continue;
            }
            let trimmed = next_line.trim();
            let expected = format!("{digest_prefix}{old}{digest_suffix}");
            let expected_comma = format!("{expected},");
            if trimmed != expected && trimmed != expected_comma {
                return Err(AssuranceError::Invalid(format!(
                    "{owner} path {} is not followed by its current digest",
                    path.display()
                )));
            }
            let within = next_line.find(old).ok_or_else(|| {
                AssuranceError::Invalid(format!(
                    "{owner} digest span is missing for {}",
                    path.display()
                ))
            })?;
            spans.push(next_start + within..next_start + within + old.len());
        }
        if spans.is_empty() {
            return Err(AssuranceError::Invalid(format!(
                "{owner} does not bind a path-scoped current digest for {}",
                path.display()
            )));
        }
        for span in spans.into_iter().rev() {
            text.replace_range(span, new);
        }
    }
    Ok(text.into_bytes())
}

fn line_offsets(text: &str) -> Vec<(usize, &str)> {
    let mut offset = 0;
    let mut lines = text
        .split_inclusive('\n')
        .map(|line| {
            let current = offset;
            offset += line.len();
            (current, line)
        })
        .collect::<Vec<_>>();
    if !text.ends_with('\n') {
        return lines;
    }
    lines.push((offset, ""));
    lines
}

fn capture_v2_tree(root: &ConfinedDirectory) -> Result<TreeSnapshot> {
    capture_tree(root, Path::new(V2_ROOT))
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

fn install_transaction(
    root: &ConfinedDirectory,
    repository: &V2Repository,
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
    expected: &TreeSnapshot,
    fault_injection: Option<V2NormalizationFault>,
) -> Result<()> {
    if root.directory_exists(Path::new(NEXT_ROOT))? {
        return Err(AssuranceError::Invalid(format!(
            "normalization recovery directory already exists: {NEXT_ROOT}"
        )));
    }
    let candidate_snapshot = expected_candidate_snapshot(expected, replacements)?;
    let preparation = (|| {
        clone_v2_tree(root, expected)?;
        for (path, bytes) in replacements {
            let relative = path.strip_prefix(V2_ROOT).map_err(|_| {
                AssuranceError::Invalid(format!(
                    "normalization replacement escapes {V2_ROOT}: {}",
                    path.display()
                ))
            })?;
            let staged = Path::new(NEXT_ROOT).join(relative);
            let mode = expected
                .files
                .get(relative)
                .ok_or_else(|| {
                    AssuranceError::Drift(format!(
                        "normalization replacement was not in the held source snapshot: {}",
                        path.display()
                    ))
                })?
                .mode;
            if !root.remove_regular_if_exists(&staged)? {
                return Err(AssuranceError::Invalid(format!(
                    "normalization replacement is not an existing regular file: {}",
                    path.display()
                )));
            }
            root.write_new(&staged, bytes)?;
            root.set_mode(&staged, mode)?;
        }
        root.sync_tree(Path::new(NEXT_ROOT))?;
        root.sync_parent()?;
        repository.verify_inputs()?;
        if capture_v2_tree(root)? != *expected {
            return Err(AssuranceError::Drift(
                "assurance/v2 files or modes changed before transaction exchange".to_owned(),
            ));
        }
        #[cfg(test)]
        if fault_injection == Some(V2NormalizationFault::StagedTreeDrift) {
            inject_staged_tree_drift(root)?;
        }
        if capture_tree(root, Path::new(NEXT_ROOT))? != candidate_snapshot {
            return Err(AssuranceError::Drift(
                "staged assurance/v2 files or modes changed before transaction exchange".to_owned(),
            ));
        }
        Ok(())
    })();
    if let Err(error) = preparation {
        return Err(combine_recovery(error, discard_next(root)));
    }
    if fault_injection == Some(V2NormalizationFault::BeforeExchange) {
        return Err(combine_recovery(
            AssuranceError::Invalid("injected normalization exchange failure".to_owned()),
            discard_next(root),
        ));
    }
    if let Err(error) = root.exchange(Path::new(V2_ROOT), Path::new(NEXT_ROOT)) {
        return Err(combine_recovery(error, discard_next(root)));
    }
    let sync = if fault_injection == Some(V2NormalizationFault::AfterExchangeSync) {
        Err(AssuranceError::Invalid(
            "injected normalization parent-sync failure".to_owned(),
        ))
    } else {
        root.sync_parent()
    };
    sync.map_err(|error| combine_recovery(error, restore_previous(root)))
}

fn expected_candidate_snapshot(
    expected: &TreeSnapshot,
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<TreeSnapshot> {
    let mut candidate = expected.clone();
    for (path, bytes) in replacements {
        let relative = path.strip_prefix(V2_ROOT).map_err(|_| {
            AssuranceError::Invalid(format!(
                "normalization replacement escapes {V2_ROOT}: {}",
                path.display()
            ))
        })?;
        let entry = candidate.files.get_mut(relative).ok_or_else(|| {
            AssuranceError::Drift(format!(
                "normalization replacement was not in the held source snapshot: {}",
                path.display()
            ))
        })?;
        entry.sha256 = sha256_bytes(bytes);
    }
    Ok(candidate)
}

fn clone_v2_tree(root: &ConfinedDirectory, expected: &TreeSnapshot) -> Result<()> {
    root.create_dir_all(Path::new(NEXT_ROOT))?;
    for directory in expected.directories.keys() {
        root.create_dir_all(&Path::new(NEXT_ROOT).join(directory))?;
    }
    clone_regular_files(root, expected)?;
    clone_directory_modes(root, expected)?;
    clone_root_mode(root, expected)
}

fn clone_regular_files(root: &ConfinedDirectory, expected: &TreeSnapshot) -> Result<()> {
    for (file, expected_entry) in &expected.files {
        let source = Path::new(V2_ROOT).join(file);
        let target = Path::new(NEXT_ROOT).join(file);
        let bytes = root.read_regular(&source)?;
        let mode = root.mode(&source)?;
        if sha256_bytes(&bytes) != expected_entry.sha256 || mode != expected_entry.mode {
            return Err(AssuranceError::Drift(format!(
                "assurance/v2 source changed while cloning: {}",
                source.display()
            )));
        }
        root.write_new(&target, &bytes)?;
        root.set_mode(&target, mode)?;
    }
    Ok(())
}

fn clone_directory_modes(root: &ConfinedDirectory, expected: &TreeSnapshot) -> Result<()> {
    for (directory, mode) in expected.directories.iter().rev() {
        if root.mode(&Path::new(V2_ROOT).join(directory))? != *mode {
            return Err(AssuranceError::Drift(format!(
                "assurance/v2 directory mode changed while cloning: {}",
                directory.display()
            )));
        }
        root.set_mode(&Path::new(NEXT_ROOT).join(directory), *mode)?;
    }
    Ok(())
}

fn clone_root_mode(root: &ConfinedDirectory, expected: &TreeSnapshot) -> Result<()> {
    if root.mode(Path::new(V2_ROOT))? != expected.root_mode {
        return Err(AssuranceError::Drift(
            "assurance/v2 root mode changed while cloning".to_owned(),
        ));
    }
    root.set_mode(Path::new(NEXT_ROOT), expected.root_mode)
}

fn restore_previous(root: &ConfinedDirectory) -> Result<()> {
    root.exchange(Path::new(V2_ROOT), Path::new(NEXT_ROOT))?;
    root.sync_parent()?;
    discard_next(root)
}

fn finish_transaction(root: &ConfinedDirectory) -> Result<()> {
    discard_next(root)
}

fn discard_next(root: &ConfinedDirectory) -> Result<()> {
    root.remove_directory_if_exists(Path::new(NEXT_ROOT))?;
    root.sync_parent()
}

#[cfg(test)]
fn inject_cleanup_fault(repository_root: &Path) -> Result<()> {
    use std::os::unix::fs::symlink;

    let path = repository_root.join(NEXT_ROOT).join("000-cleanup-fault");
    symlink("/dev/null", &path).map_err(|error| AssuranceError::io(path, error))
}

#[cfg(test)]
fn inject_staged_tree_drift(root: &ConfinedDirectory) -> Result<()> {
    let path = Path::new(NEXT_ROOT).join("schemas/catalog.schema.json");
    let mut bytes = root.read_regular(&path)?;
    bytes.push(b'\n');
    let mode = root.mode(&path)?;
    if !root.remove_regular_if_exists(&path)? {
        return Err(AssuranceError::Invalid(
            "staged drift target disappeared".to_owned(),
        ));
    }
    root.write_new(&path, &bytes)?;
    root.set_mode(&path, mode)
}

#[cfg(test)]
fn append_test_drift(path: &Path) -> Result<()> {
    use std::fs::OpenOptions;

    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .map_err(|error| AssuranceError::io(path, error))?;
    file.write_all(b"\n")
        .map_err(|error| AssuranceError::io(path, error))
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
    use std::ffi::OsString;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{
        NormalizationControls, V2NormalizationFault, V2NormalizationMode, V2NormalizationOptions,
        normalize_report_with_controls, run_converter, validate_draft_outputs,
    };
    use crate::v2::V2Repository;

    const REPORT_ID: &str = "linear-groundwater-reservoir-recurrence";

    #[test]
    fn controlled_converter_errors_are_typed() {
        let missing = OsString::from("openwepp-assurance-missing-normalizer");
        let error = run_converter(&missing, b"input").expect_err("missing converter must fail");
        assert!(matches!(error, crate::AssuranceError::Io { .. }));

        let nonzero = script("normalizer-nonzero", "echo denied >&2\nexit 7\n");
        for _ in 0..32 {
            let error = run_converter(&nonzero.path.join("tool").into_os_string(), b"input")
                .expect_err("nonzero converter must fail");
            assert!(error.to_string().contains("denied"));
        }

        let non_utf8 = script("normalizer-non-utf8", "cat >/dev/null\nprintf '\\377'\n");
        let error = run_converter(&non_utf8.path.join("tool").into_os_string(), b"input")
            .expect_err("non-UTF-8 converter output must fail");
        assert!(error.to_string().contains("non-UTF-8"));

        let streaming = script("normalizer-streaming", "cat\n");
        let input = vec![b'x'; 2 * 1024 * 1024];
        assert_eq!(
            run_converter(&streaming.path.join("tool").into_os_string(), &input)
                .expect("streaming converter must communicate without pipe deadlock"),
            input
        );
    }

    #[test]
    fn non_idempotent_converter_is_rejected_before_mutation() {
        let fixture = repository_fixture("normalizer-non-idempotent");
        let converter = script(
            "normalizer-toggling-tool",
            "input=$(cat)\ncase \"$input\" in *metres*) printf '%s' \"$input\" | sed 's/metres/meters/g';; *) printf '%s' \"$input\" | sed 's/meters/metres/g';; esac\n",
        );
        let before = collect_files(&fixture.path.join("assurance/v2"));
        let repository = V2Repository::open(&fixture.path).expect("open fixture");
        let controls = NormalizationControls {
            converter: converter.path.join("tool").into_os_string(),
            fault_injection: None,
        };
        let error = normalize_report_with_controls(
            &repository,
            REPORT_ID,
            &V2NormalizationOptions::new("en-US", V2NormalizationMode::Apply),
            &controls,
        )
        .expect_err("non-idempotent converter must fail");
        assert!(error.to_string().contains("not idempotent"));
        assert_eq!(before, collect_files(&fixture.path.join("assurance/v2")));
    }

    #[test]
    fn packet_contract_rejects_missing_misplaced_duplicate_and_extra_outputs() {
        let manuscript = Path::new("report/manuscript.md");
        let supplement = Path::new("report/supplement.md");
        let expected = [(manuscript, "a"), (supplement, "b")];
        let output =
            |path: &Path, digest: &str| serde_json::json!({"path": path, "sha256": digest});

        let valid = serde_json::json!({
            "schema_version": 1,
            "draft_outputs": [output(manuscript, "a"), output(supplement, "b")]
        });
        validate_draft_outputs(&valid, expected).expect("exact outputs are valid");

        for invalid in [
            serde_json::json!({
                "schema_version": 1,
                "draft_outputs": [output(manuscript, "a")]
            }),
            serde_json::json!({
                "schema_version": 1,
                "other_outputs": [output(manuscript, "a"), output(supplement, "b")]
            }),
            serde_json::json!({
                "schema_version": 1,
                "draft_outputs": [output(manuscript, "a"), output(manuscript, "a")]
            }),
            serde_json::json!({
                "schema_version": 1,
                "draft_outputs": [
                    output(manuscript, "a"),
                    output(supplement, "b"),
                    output(Path::new("report/extra.md"), "c")
                ]
            }),
            serde_json::json!({
                "schema_version": 1,
                "draft_outputs": [output(manuscript, "a"), output(supplement, "b")],
                "other_outputs": [output(manuscript, "a")]
            }),
        ] {
            validate_draft_outputs(&invalid, expected)
                .expect_err("non-exact packet outputs must fail");
        }
    }

    #[test]
    fn selected_content_and_unrelated_tree_drift_fail_closed() {
        for (label, fault) in [
            (
                "normalizer-selected-content-drift",
                V2NormalizationFault::BeforeCandidateContentDrift,
            ),
            (
                "normalizer-unrelated-tree-drift",
                V2NormalizationFault::AfterCandidateTreeDrift,
            ),
            (
                "normalizer-transient-catalog-drift",
                V2NormalizationFault::BeforeCatalogReadDrift,
            ),
        ] {
            let fixture = repository_fixture(label);
            let converter = script(label, "sed 's/meters/metres/g'\n");
            let repository = V2Repository::open(&fixture.path).expect("open fixture");
            let controls = NormalizationControls {
                converter: converter.path.join("tool").into_os_string(),
                fault_injection: Some(fault),
            };
            let error = normalize_report_with_controls(
                &repository,
                REPORT_ID,
                &V2NormalizationOptions::new("en-US", V2NormalizationMode::Apply),
                &controls,
            )
            .expect_err("injected source drift must fail");
            assert!(matches!(error, crate::AssuranceError::Drift(_)));
            assert!(!fixture.path.join("assurance/.v2.normalize.next").exists());
        }
    }

    #[test]
    fn post_install_fault_restores_exact_source_generation() {
        let fixture = repository_fixture("normalizer-post-install-fault");
        let converter = script("normalizer-changing-tool", "sed 's/meters/metres/g'\n");
        let before = collect_files(&fixture.path.join("assurance/v2"));
        let repository = V2Repository::open(&fixture.path).expect("open fixture");
        let controls = NormalizationControls {
            converter: converter.path.join("tool").into_os_string(),
            fault_injection: Some(V2NormalizationFault::AfterInstall),
        };
        let error = normalize_report_with_controls(
            &repository,
            REPORT_ID,
            &V2NormalizationOptions::new("en-US", V2NormalizationMode::Apply),
            &controls,
        )
        .expect_err("post-install fault must fail");
        assert!(error.to_string().contains("injected normalization failure"));
        assert_eq!(before, collect_files(&fixture.path.join("assurance/v2")));
        assert!(!fixture.path.join("assurance/.v2.normalize.next").exists());
    }

    #[test]
    fn parent_sync_failure_restores_exact_source_generation() {
        let fixture = repository_fixture("normalizer-parent-sync-fault");
        let converter = script("normalizer-sync-tool", "sed 's/meters/metres/g'\n");
        let before = collect_files(&fixture.path.join("assurance/v2"));
        let repository = V2Repository::open(&fixture.path).expect("open fixture");
        let controls = NormalizationControls {
            converter: converter.path.join("tool").into_os_string(),
            fault_injection: Some(V2NormalizationFault::AfterExchangeSync),
        };
        let error = normalize_report_with_controls(
            &repository,
            REPORT_ID,
            &V2NormalizationOptions::new("en-US", V2NormalizationMode::Apply),
            &controls,
        )
        .expect_err("parent-sync fault must fail");
        assert!(error.to_string().contains("parent-sync failure"));
        assert_eq!(before, collect_files(&fixture.path.join("assurance/v2")));
        assert!(!fixture.path.join("assurance/.v2.normalize.next").exists());
    }

    #[test]
    fn staged_drift_and_exchange_failure_remove_candidate_without_source_change() {
        for (label, fault) in [
            (
                "normalizer-staged-tree-drift",
                V2NormalizationFault::StagedTreeDrift,
            ),
            (
                "normalizer-exchange-fault",
                V2NormalizationFault::BeforeExchange,
            ),
        ] {
            let fixture = repository_fixture(label);
            let converter = script(label, "sed 's/meters/metres/g'\n");
            let before = collect_files(&fixture.path.join("assurance/v2"));
            let repository = V2Repository::open(&fixture.path).expect("open fixture");
            let controls = NormalizationControls {
                converter: converter.path.join("tool").into_os_string(),
                fault_injection: Some(fault),
            };
            normalize_report_with_controls(
                &repository,
                REPORT_ID,
                &V2NormalizationOptions::new("en-US", V2NormalizationMode::Apply),
                &controls,
            )
            .expect_err("pre-exchange failure must reject the candidate");
            assert_eq!(before, collect_files(&fixture.path.join("assurance/v2")));
            assert!(!fixture.path.join("assurance/.v2.normalize.next").exists());
        }
    }

    #[test]
    fn post_commit_cleanup_failure_never_restores_partial_old_generation() {
        let fixture = repository_fixture("normalizer-cleanup-fault");
        let converter = script("normalizer-cleanup-tool", "sed 's/meters/metres/g'\n");
        let before = collect_files(&fixture.path.join("assurance/v2"));
        let repository = V2Repository::open(&fixture.path).expect("open fixture");
        let controls = NormalizationControls {
            converter: converter.path.join("tool").into_os_string(),
            fault_injection: Some(V2NormalizationFault::BeforeCleanup),
        };
        let error = normalize_report_with_controls(
            &repository,
            REPORT_ID,
            &V2NormalizationOptions::new("en-US", V2NormalizationMode::Apply),
            &controls,
        )
        .expect_err("cleanup fault must report a committed-cleanup error");
        let receipt_json = match error {
            crate::AssuranceError::CommittedCleanup { receipt_json, .. } => receipt_json,
            other => panic!("expected committed cleanup error, got {other}"),
        };
        let receipt: serde_json::Value =
            serde_json::from_str(&receipt_json).expect("committed receipt is valid JSON");
        assert_eq!(receipt["changed"], true);
        assert_ne!(
            receipt["old_source_root_sha256"],
            receipt["new_source_root_sha256"]
        );
        assert_ne!(before, collect_files(&fixture.path.join("assurance/v2")));
        V2Repository::open(&fixture.path)
            .expect("committed source reopens")
            .validate_report(REPORT_ID)
            .expect("committed source remains valid");
        assert!(fixture.path.join("assurance/.v2.normalize.next").exists());
        let retry = V2Repository::open(&fixture.path)
            .expect("reopen committed source")
            .normalize_report(
                REPORT_ID,
                &V2NormalizationOptions::new("en-US", V2NormalizationMode::Check),
            )
            .expect_err("retained recovery state must block a no-op check");
        assert!(retry.to_string().contains("requires explicit disposition"));
    }

    fn script(label: &str, body: &str) -> Scratch {
        use std::os::unix::fs::PermissionsExt as _;

        let scratch = Scratch::new(label);
        let executable = scratch.path.join("tool");
        fs::write(&executable, format!("#!/bin/sh\n{body}")).expect("write tool");
        let mut permissions = fs::metadata(&executable)
            .expect("tool metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(executable, permissions).expect("make tool executable");
        scratch
    }

    fn repository_fixture(label: &str) -> Scratch {
        let source = repository_root();
        let target = Scratch::new(label);
        copy_tree(
            &source.join("assurance/v2"),
            &target.path.join("assurance/v2"),
        );
        for relative in [
            "assurance/catalog.yaml",
            "assurance/templates/catalog.md",
            "assurance/generated/wepppy-usersum.yaml",
            "usersum/assurance/README.md",
            "usersum/hillslope-hydrology-and-sediment-physics.md",
            "docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md",
            "crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs",
            "docs/work-packages/20260716-assure05-first-production-v2-report-001/artifacts/study-protocol.md",
            "docs/work-packages/20260716-assure05-first-production-v2-report-001/artifacts/realization-freeze.md",
            "docs/work-packages/20260716-assure05-first-production-v2-report-001/prompts/archived/20260716-codex-execute-assure05_prompt.md",
            "docs/work-packages/20260709-laned-active-baseflow-export-closure-001/artifacts/consumer-path-proof.md",
            "docs/work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/artifacts/consumer-path-proof.md",
        ] {
            copy_file(&source, &target.path, relative);
        }
        target
    }

    fn copy_tree(source: &Path, target: &Path) {
        fs::create_dir_all(target).expect("create fixture tree");
        for entry in fs::read_dir(source).expect("read fixture source") {
            let entry = entry.expect("read fixture entry");
            let destination = target.join(entry.file_name());
            if entry.file_type().expect("fixture type").is_dir() {
                copy_tree(&entry.path(), &destination);
            } else {
                fs::copy(entry.path(), destination).expect("copy fixture file");
            }
        }
    }

    fn copy_file(source_root: &Path, target_root: &Path, relative: &str) {
        let target = target_root.join(relative);
        fs::create_dir_all(target.parent().expect("target parent")).expect("create parent");
        fs::copy(source_root.join(relative), target).expect("copy file");
    }

    fn collect_files(root: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
        let mut files = std::collections::BTreeMap::new();
        collect_files_into(root, root, &mut files);
        files
    }

    fn collect_files_into(
        root: &Path,
        directory: &Path,
        files: &mut std::collections::BTreeMap<PathBuf, Vec<u8>>,
    ) {
        for entry in fs::read_dir(directory).expect("read tree") {
            let entry = entry.expect("read entry");
            if entry.file_type().expect("entry type").is_dir() {
                collect_files_into(root, &entry.path(), files);
            } else {
                files.insert(
                    entry
                        .path()
                        .strip_prefix(root)
                        .expect("relative path")
                        .to_path_buf(),
                    fs::read(entry.path()).expect("read file"),
                );
            }
        }
    }

    fn repository_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("canonical root")
    }

    struct Scratch {
        path: PathBuf,
    }

    impl Scratch {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("openwepp-{label}-{}-{counter}", std::process::id()));
            if path.exists() {
                fs::remove_dir_all(&path).expect("remove stale scratch");
            }
            fs::create_dir_all(&path).expect("create scratch");
            Self { path }
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            if self.path.exists() {
                fs::remove_dir_all(&self.path).expect("remove scratch");
            }
        }
    }
}
