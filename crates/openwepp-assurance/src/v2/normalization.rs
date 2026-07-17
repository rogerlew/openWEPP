use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde::Serialize;

use super::V2Repository;
use crate::{AssuranceError, Result, sha256_bytes};

const LANGUAGE: &str = "en-US";

/// Whether normalization is a read-only policy check or a source transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V2NormalizationMode {
    Check,
    Apply,
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

/// One exact source identity changed by a normalization transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct V2NormalizationChange {
    pub path: PathBuf,
    pub old_sha256: String,
    pub new_sha256: String,
}

/// Compatibility receipt for the original normalization command.
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
    /// Returns an error if serialization does not produce valid UTF-8.
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

pub(super) fn normalize_report(
    repository: &V2Repository,
    report_id: &str,
    options: &V2NormalizationOptions,
) -> Result<V2NormalizationReceipt> {
    validate_options(options)?;
    let old_root = selected_root(&repository.validate_report(report_id)?, report_id)?;
    let before = collect_file_digests(&repository.root.join("assurance/v2"))?;
    let mode = match options.mode {
        V2NormalizationMode::Check => super::amendment::V2AmendMode::Check,
        V2NormalizationMode::Apply => super::amendment::V2AmendMode::Apply,
    };
    let amendment =
        super::amendment::amend_normalize(&repository.root, report_id, &options.language, mode)?;
    if options.mode == V2NormalizationMode::Check && amendment.changed {
        return Err(AssuranceError::Drift(format!(
            "report '{report_id}' requires American-English normalization; rerun with --apply"
        )));
    }
    let new_root = if amendment.changed && options.mode == V2NormalizationMode::Apply {
        selected_root(
            &V2Repository::open(&repository.root)?.validate_report(report_id)?,
            report_id,
        )?
    } else {
        old_root.clone()
    };
    let changes = if options.mode == V2NormalizationMode::Apply {
        amendment
            .affected_paths
            .into_iter()
            .map(|path| {
                let relative = PathBuf::from(&path);
                let old_sha256 = before
                    .get(&relative)
                    .cloned()
                    .unwrap_or_else(|| "absent".to_owned());
                let bytes = std::fs::read(repository.root.join(&relative))
                    .map_err(|error| AssuranceError::io(&relative, error))?;
                Ok(V2NormalizationChange {
                    path: relative,
                    old_sha256,
                    new_sha256: sha256_bytes(&bytes),
                })
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        Vec::new()
    };
    Ok(V2NormalizationReceipt {
        schema_version: 1,
        operation: options.mode,
        language: options.language.clone(),
        converter: "uk2us".to_owned(),
        report_id: report_id.to_owned(),
        changed: amendment.changed,
        old_source_root_sha256: old_root,
        new_source_root_sha256: new_root,
        changes,
    })
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

fn collect_file_digests(root: &Path) -> Result<BTreeMap<PathBuf, String>> {
    fn visit(root: &Path, directory: &Path, digests: &mut BTreeMap<PathBuf, String>) -> Result<()> {
        for entry in
            std::fs::read_dir(directory).map_err(|error| AssuranceError::io(directory, error))?
        {
            let entry = entry.map_err(|error| AssuranceError::io(directory, error))?;
            let file_type = entry
                .file_type()
                .map_err(|error| AssuranceError::io(entry.path(), error))?;
            if file_type.is_dir() {
                visit(root, &entry.path(), digests)?;
            } else if file_type.is_file() {
                let bytes = std::fs::read(entry.path())
                    .map_err(|error| AssuranceError::io(entry.path(), error))?;
                let relative = Path::new("assurance/v2").join(
                    entry
                        .path()
                        .strip_prefix(root)
                        .map_err(|error| AssuranceError::Invalid(error.to_string()))?,
                );
                digests.insert(relative, sha256_bytes(&bytes));
            }
        }
        Ok(())
    }

    let mut digests = BTreeMap::new();
    visit(root, root, &mut digests)?;
    Ok(digests)
}

pub(super) fn run_converter(executable: &OsString, input: &[u8]) -> Result<Vec<u8>> {
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
    let status = child
        .wait()
        .map_err(|error| AssuranceError::io("uk2us", error))?;
    let stdout = join_converter_pipe(stdout_reader, "uk2us stdout");
    let stderr = join_converter_pipe(stderr_reader, "uk2us stderr");
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
