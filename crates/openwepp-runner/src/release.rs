use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex, MutexGuard};

use serde::Serialize;
use serde_json::Value;

use crate::api::ReleaseLintReport;
use crate::constants::BINARY_RELEASE_SCHEMA_ID;
use crate::errors::{ReleaseLintError, ReleaseMetadataError};
use crate::role::BinaryRole;
use crate::shared::{
    file_name_string, git_source_commit_or_unknown, path_has_extension_case_insensitive,
    sha256_file_hex, utc_now_rfc3339,
};

static RELEASE_SIDECAR_IO_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[derive(Debug, Serialize)]
struct BinaryReleaseFeatures {
    hbp_supported: bool,
    hbp_schema_major: u32,
    hbp_schema_minor: u32,
    hbp_pass_family: String,
    legacy_ascii_pass_family: String,
    mode2_master_pass_prompt_required: bool,
}

#[derive(Debug, Serialize)]
struct BinaryReleaseValidation {
    schema_valid: bool,
    release_lint_level: String,
    validated_utc: String,
}

#[derive(Debug, Serialize)]
struct BinaryReleaseMetadataDocument {
    schema: String,
    binary_name: String,
    binary_role: String,
    release_tag: String,
    source_repo: String,
    source_commit: String,
    built_utc: String,
    sha256: String,
    features: BinaryReleaseFeatures,
    validation: BinaryReleaseValidation,
}

#[allow(clippy::too_many_lines)]
pub fn lint_release_directory(release_dir: &Path) -> Result<ReleaseLintReport, ReleaseLintError> {
    let entries = fs::read_dir(release_dir).map_err(|source| ReleaseLintError::DirectoryRead {
        path: release_dir.to_path_buf(),
        source,
    })?;

    let mut candidate_binaries = Vec::new();
    for entry_result in entries {
        let entry = entry_result.map_err(|source| ReleaseLintError::DirectoryRead {
            path: release_dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(OsStr::to_str) else {
            continue;
        };

        if !file_name.starts_with("openwepp_") || path_has_extension_case_insensitive(&path, "json")
        {
            continue;
        }

        candidate_binaries.push(path);
    }

    if candidate_binaries.is_empty() {
        return Err(ReleaseLintError::NoReleaseCandidates {
            release_dir: release_dir.to_path_buf(),
        });
    }

    let mut watershed_hbp_supported: Option<bool> = None;
    let mut hillslope_hbp_supported: Option<bool> = None;

    for binary_path in &candidate_binaries {
        let binary_name = file_name_string(binary_path);
        let expected_role = classify_release_binary_role(binary_name.as_str())?;
        if !release_binary_name_is_valid(binary_name.as_str(), expected_role) {
            return Err(ReleaseLintError::InvalidBinaryName { binary_name });
        }

        let sidecar_path = sidecar_path_for_binary(binary_path);
        if !sidecar_path.is_file() {
            return Err(ReleaseLintError::MissingSidecar { sidecar_path });
        }

        let metadata = validate_release_sidecar(&sidecar_path).map_err(|source| {
            ReleaseLintError::SidecarInvalid {
                sidecar_path: sidecar_path.clone(),
                source,
            }
        })?;

        let observed_role = required_str(&metadata, "binary_role").map_err(|source| {
            ReleaseLintError::SidecarInvalid {
                sidecar_path: sidecar_path.clone(),
                source,
            }
        })?;
        if BinaryRole::parse(observed_role).is_none() {
            return Err(ReleaseLintError::SidecarRoleMismatch {
                sidecar_path,
                expected: expected_role,
                observed: observed_role.to_string(),
            });
        }

        if BinaryRole::parse(observed_role) != Some(expected_role) {
            return Err(ReleaseLintError::SidecarRoleMismatch {
                sidecar_path,
                expected: expected_role,
                observed: observed_role.to_string(),
            });
        }

        let observed_binary_name = required_str(&metadata, "binary_name").map_err(|source| {
            ReleaseLintError::SidecarInvalid {
                sidecar_path: sidecar_path.clone(),
                source,
            }
        })?;
        if observed_binary_name != binary_name {
            return Err(ReleaseLintError::SidecarBinaryNameMismatch {
                sidecar_path,
                expected: binary_name,
                observed: observed_binary_name.to_string(),
            });
        }

        let features = required_object(&metadata, "features").map_err(|source| {
            ReleaseLintError::SidecarInvalid {
                sidecar_path: sidecar_path.clone(),
                source,
            }
        })?;
        let hbp_supported = required_bool(features, "hbp_supported").map_err(|source| {
            ReleaseLintError::SidecarInvalid {
                sidecar_path,
                source,
            }
        })?;

        match expected_role {
            BinaryRole::Watershed => watershed_hbp_supported = Some(hbp_supported),
            BinaryRole::Hillslope => hillslope_hbp_supported = Some(hbp_supported),
            BinaryRole::Replay => {}
        }
    }

    if let (Some(watershed), Some(hillslope)) = (watershed_hbp_supported, hillslope_hbp_supported)
        && watershed != hillslope
    {
        return Err(ReleaseLintError::HbpPairMismatch {
            watershed,
            hillslope,
        });
    }

    Ok(ReleaseLintReport {
        checked_binaries: candidate_binaries,
    })
}

pub fn write_release_sidecar_for_binary(
    binary_path: &Path,
    role: BinaryRole,
) -> Result<PathBuf, ReleaseMetadataError> {
    let _io_guard = lock_release_sidecar_io();
    let metadata = build_release_metadata_document(binary_path, role)?;
    let sidecar_path = sidecar_path_for_binary(binary_path);
    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|source| ReleaseMetadataError::JsonSerialize { source })?;
    fs::write(&sidecar_path, json).map_err(|source| ReleaseMetadataError::Io {
        path: sidecar_path.clone(),
        source,
    })?;

    validate_release_sidecar_unlocked(&sidecar_path)?;
    Ok(sidecar_path)
}

pub fn validate_release_sidecar(sidecar_path: &Path) -> Result<Value, ReleaseMetadataError> {
    let _io_guard = lock_release_sidecar_io();
    validate_release_sidecar_unlocked(sidecar_path)
}

fn validate_release_sidecar_unlocked(sidecar_path: &Path) -> Result<Value, ReleaseMetadataError> {
    let content = fs::read_to_string(sidecar_path).map_err(|source| ReleaseMetadataError::Io {
        path: sidecar_path.to_path_buf(),
        source,
    })?;
    let json: Value =
        serde_json::from_str(&content).map_err(|source| ReleaseMetadataError::JsonParse {
            path: sidecar_path.to_path_buf(),
            source,
        })?;

    let schema = required_str(&json, "schema")?;
    if schema != BINARY_RELEASE_SCHEMA_ID {
        return Err(ReleaseMetadataError::InvalidField {
            field: "schema",
            detail: format!("expected {BINARY_RELEASE_SCHEMA_ID}, observed {schema}"),
        });
    }

    let role = required_str(&json, "binary_role")?;
    if BinaryRole::parse(role).is_none() {
        return Err(ReleaseMetadataError::InvalidField {
            field: "binary_role",
            detail: format!("unsupported role {role}"),
        });
    }

    for field in [
        "binary_name",
        "release_tag",
        "source_repo",
        "source_commit",
        "built_utc",
        "sha256",
    ] {
        let _ = required_str(&json, field)?;
    }

    let features = required_object(&json, "features")?;
    let _ = required_bool(features, "hbp_supported")?;
    let _ = required_u64(features, "hbp_schema_major")?;
    let _ = required_u64(features, "hbp_schema_minor")?;
    let _ = required_map_str(features, "hbp_pass_family")?;
    let _ = required_map_str(features, "legacy_ascii_pass_family")?;
    let _ = required_bool(features, "mode2_master_pass_prompt_required")?;

    let validation = required_object(&json, "validation")?;
    let _ = required_bool(validation, "schema_valid")?;
    let _ = required_map_str(validation, "release_lint_level")?;
    let _ = required_map_str(validation, "validated_utc")?;

    Ok(json)
}

fn lock_release_sidecar_io() -> MutexGuard<'static, ()> {
    match RELEASE_SIDECAR_IO_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn build_release_metadata_document(
    binary_path: &Path,
    role: BinaryRole,
) -> Result<BinaryReleaseMetadataDocument, ReleaseMetadataError> {
    let binary_name = file_name_string(binary_path);
    let built_utc = utc_now_rfc3339().map_err(|detail| ReleaseMetadataError::InvalidField {
        field: "built_utc",
        detail,
    })?;
    let source_repo = std::env::var("CARGO_PKG_REPOSITORY")
        .unwrap_or_else(|_| "https://github.com/rogerlew/openWEPP".to_string());

    Ok(BinaryReleaseMetadataDocument {
        schema: BINARY_RELEASE_SCHEMA_ID.to_string(),
        binary_name: binary_name.clone(),
        binary_role: role.as_str().to_string(),
        release_tag: infer_release_tag(binary_name.as_str()),
        source_repo,
        source_commit: git_source_commit_or_unknown(),
        built_utc: built_utc.clone(),
        sha256: sha256_file_hex(binary_path).map_err(|source| ReleaseMetadataError::Io {
            path: binary_path.to_path_buf(),
            source,
        })?,
        features: BinaryReleaseFeatures {
            hbp_supported: true,
            hbp_schema_major: 1,
            hbp_schema_minor: 0,
            hbp_pass_family: "H*.hbp".to_string(),
            legacy_ascii_pass_family: "H*.pass.dat".to_string(),
            mode2_master_pass_prompt_required: true,
        },
        validation: BinaryReleaseValidation {
            schema_valid: true,
            release_lint_level: "contract_v1".to_string(),
            validated_utc: built_utc,
        },
    })
}

fn required_object<'a>(
    json: &'a Value,
    field: &'static str,
) -> Result<&'a serde_json::Map<String, Value>, ReleaseMetadataError> {
    json.get(field)
        .and_then(Value::as_object)
        .ok_or(ReleaseMetadataError::MissingField { field })
}

fn required_str<'a>(json: &'a Value, field: &'static str) -> Result<&'a str, ReleaseMetadataError> {
    json.get(field)
        .and_then(Value::as_str)
        .ok_or(ReleaseMetadataError::MissingField { field })
}

fn required_map_str<'a>(
    json: &'a serde_json::Map<String, Value>,
    field: &'static str,
) -> Result<&'a str, ReleaseMetadataError> {
    json.get(field)
        .and_then(Value::as_str)
        .ok_or(ReleaseMetadataError::MissingField { field })
}

fn required_bool(
    json: &serde_json::Map<String, Value>,
    field: &'static str,
) -> Result<bool, ReleaseMetadataError> {
    json.get(field)
        .and_then(Value::as_bool)
        .ok_or(ReleaseMetadataError::MissingField { field })
}

fn required_u64(
    json: &serde_json::Map<String, Value>,
    field: &'static str,
) -> Result<u64, ReleaseMetadataError> {
    json.get(field)
        .and_then(Value::as_u64)
        .ok_or(ReleaseMetadataError::MissingField { field })
}

fn classify_release_binary_role(binary_name: &str) -> Result<BinaryRole, ReleaseLintError> {
    if binary_name.ends_with("_hill") {
        return Ok(BinaryRole::Hillslope);
    }
    if binary_name.ends_with("_replay") {
        return Ok(BinaryRole::Replay);
    }
    if binary_name.starts_with("openwepp_") {
        return Ok(BinaryRole::Watershed);
    }

    Err(ReleaseLintError::InvalidBinaryName {
        binary_name: binary_name.to_string(),
    })
}

fn release_binary_name_is_valid(binary_name: &str, role: BinaryRole) -> bool {
    if !binary_name.starts_with("openwepp_") {
        return false;
    }

    let stem = &binary_name["openwepp_".len()..];
    if stem.len() < 6 {
        return false;
    }

    let (date_part, suffix_part) = stem.split_at(6);
    if !date_part.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }

    if !suffix_part
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
    {
        return false;
    }

    match role {
        BinaryRole::Watershed => {
            !binary_name.ends_with("_hill") && !binary_name.ends_with("_replay")
        }
        BinaryRole::Hillslope => binary_name.ends_with("_hill"),
        BinaryRole::Replay => binary_name.ends_with("_replay"),
    }
}

fn infer_release_tag(binary_name: &str) -> String {
    if binary_name.starts_with("openwepp_") {
        binary_name.to_string()
    } else {
        "openwepp_dev".to_string()
    }
}

fn sidecar_path_for_binary(binary_path: &Path) -> PathBuf {
    PathBuf::from(format!("{}.json", binary_path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_name_validator_accepts_expected_patterns() {
        assert!(release_binary_name_is_valid(
            "openwepp_260511",
            BinaryRole::Watershed
        ));
        assert!(release_binary_name_is_valid(
            "openwepp_260511_hill",
            BinaryRole::Hillslope
        ));
        assert!(release_binary_name_is_valid(
            "openwepp_260511a_replay",
            BinaryRole::Replay
        ));
    }

    #[test]
    fn release_name_validator_rejects_invalid_patterns() {
        assert!(!release_binary_name_is_valid(
            "openwepp_26051_hill",
            BinaryRole::Hillslope
        ));
        assert!(!release_binary_name_is_valid(
            "openwepp_260511_HILL",
            BinaryRole::Hillslope
        ));
        assert!(!release_binary_name_is_valid(
            "other_260511",
            BinaryRole::Watershed
        ));
    }
}
