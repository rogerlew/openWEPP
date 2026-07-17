use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::amendment::V2AmendmentReceipt;
use crate::{AssuranceError, Result, sha256_bytes};

pub(super) fn render_yaml(value: &serde_yaml::Value) -> Result<Vec<u8>> {
    serde_yaml::to_string(value)
        .map(String::into_bytes)
        .map_err(|error| AssuranceError::Invalid(format!("YAML serialization failed: {error}")))
}

pub(super) fn parse_yaml<T: for<'de> Deserialize<'de>>(
    path: impl Into<PathBuf>,
    bytes: &[u8],
) -> Result<T> {
    serde_yaml::from_slice(bytes).map_err(|error| AssuranceError::Parse {
        path: path.into(),
        message: error.to_string(),
    })
}

pub(super) fn read_regular(root: &Path, path: &Path) -> Result<Vec<u8>> {
    super::confined::read_regular_confined(root, path)
}

pub(super) fn receipt_bytes(receipt: &V2AmendmentReceipt) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(receipt).map_err(|error| {
        AssuranceError::Invalid(format!("amendment receipt serialization failed: {error}"))
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub(super) fn receipt_id(bytes: &[u8]) -> String {
    let mut material = b"openwepp-assurance-amendment-receipt-v1\0".to_vec();
    material.extend_from_slice(bytes);
    sha256_bytes(&material)
}

fn focused_gate_argv() -> Vec<String> {
    [
        "cargo",
        "nextest",
        "run",
        "--workspace",
        "--profile",
        "assurance-amendment",
    ]
    .into_iter()
    .map(ToOwned::to_owned)
    .collect()
}

pub(super) fn gate_argv(impact_class: &str) -> Vec<Vec<String>> {
    match impact_class {
        "metadata-fast" | "editorial-fast" | "governance-focused" => {
            vec![focused_gate_argv()]
        }
        _ => Vec::new(),
    }
}

pub(super) fn require_text(value: &str, label: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(AssuranceError::Usage(format!("{label} cannot be empty")))
    } else {
        Ok(())
    }
}

pub(super) fn require_optional_text(value: Option<&String>, label: &str) -> Result<()> {
    value
        .map(String::as_str)
        .ok_or_else(|| AssuranceError::Usage(format!("lifecycle request requires {label}")))
        .and_then(|value| require_text(value, label))
}

pub(super) fn gate_id(impact_class: &str) -> &'static str {
    match impact_class {
        "metadata-fast" => "assurance-amendment-metadata-v1",
        "editorial-fast" => "assurance-amendment-editorial-v1",
        "governance-focused" => "assurance-amendment-governance-v1",
        "scientific-full" => "assurance-implementation-package-v1",
        "release-full" => "assurance-release-package-v1",
        _ => "assurance-amendment-v1",
    }
}

pub(super) fn set_yaml_string(mapping: &mut serde_yaml::Mapping, field: &str, value: &str) -> bool {
    let key = yaml_key(field);
    if mapping.get(&key).and_then(serde_yaml::Value::as_str) == Some(value) {
        false
    } else {
        mapping.insert(key, serde_yaml::Value::String(value.to_owned()));
        true
    }
}

pub(super) fn yaml_key(value: &str) -> serde_yaml::Value {
    serde_yaml::Value::String(value.to_owned())
}
