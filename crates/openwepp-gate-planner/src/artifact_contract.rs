//! Shared artifact-contract classification for execution and verification.

use std::fs;
use std::path::{Component, Path, PathBuf};

use serde_json::Value;

use crate::error::{ErrorClass, GatePolicyError, Result};

pub(crate) fn create_confined_directories(root: &Path, directory: &Path) -> Result<()> {
    let relative = directory
        .strip_prefix(root)
        .map_err(|_| path_error(directory))?;
    let mut current = PathBuf::from(root);
    for component in relative.components() {
        let Component::Normal(name) = component else {
            return Err(path_error(directory));
        };
        current.push(name);
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(path_error(&current));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => fs::create_dir(&current)
                .map_err(|error| {
                    GatePolicyError::new(
                        ErrorClass::Io,
                        "GATE-EXEC-OUTPUT-DIRECTORY",
                        error.to_string(),
                    )
                })?,
            Err(error) => {
                return Err(GatePolicyError::new(
                    ErrorClass::Io,
                    "GATE-EXEC-OUTPUT-DIRECTORY",
                    error.to_string(),
                ));
            }
        }
    }
    Ok(())
}

fn path_error(path: &Path) -> GatePolicyError {
    GatePolicyError::new(
        ErrorClass::Execution,
        "GATE-EXEC-OUTPUT-ESCAPE",
        path.display().to_string(),
    )
}

pub(crate) fn has_output_extension(node: &Value, extension: &str) -> bool {
    node["output_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .any(|path| {
            Path::new(path)
                .extension()
                .is_some_and(|value| value == extension)
        })
}

pub(crate) fn node_has_junit_evidence(node: &Value) -> bool {
    node["artifact_contract"] == "nextest-junit-v1" && node["executor"]["kind"] == "NEXTEST_V1"
        || node["artifact_contract"] == "adjudicated-crap-v1" && has_output_extension(node, "xml")
}

pub(crate) fn artifact_kind(contract: &str, path: &str) -> &'static str {
    match (
        contract,
        Path::new(path).extension().and_then(|value| value.to_str()),
    ) {
        ("adjudicated-crap-v1", Some("lcov")) => "LCOV",
        ("adjudicated-crap-v1", Some("xml")) | ("nextest-junit-v1", _) => "JUNIT",
        ("adjudicated-crap-v1", _) => "CRAP",
        ("schema-validation-v1", _) => "SCHEMA",
        _ => "LOG",
    }
}
