use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

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

pub fn lint_release_directory(release_dir: &Path) -> Result<ReleaseLintReport, ReleaseLintError> {
    let candidate_binaries = collect_release_candidate_binaries(release_dir)?;
    let hbp_pair = lint_release_binaries(&candidate_binaries)?;
    validate_release_hbp_pair(&hbp_pair)?;

    Ok(ReleaseLintReport {
        checked_binaries: candidate_binaries,
    })
}

#[derive(Debug, Default)]
struct ReleaseHbpPair {
    watershed: Option<bool>,
    hillslope: Option<bool>,
}

impl ReleaseHbpPair {
    fn record(&mut self, role: BinaryRole, hbp_supported: bool) {
        match role {
            BinaryRole::Watershed => self.watershed = Some(hbp_supported),
            BinaryRole::Hillslope => self.hillslope = Some(hbp_supported),
            BinaryRole::Replay => {}
        }
    }
}

#[derive(Debug)]
struct ReleaseBinaryLintOutcome {
    role: BinaryRole,
    hbp_supported: bool,
}

fn collect_release_candidate_binaries(
    release_dir: &Path,
) -> Result<Vec<PathBuf>, ReleaseLintError> {
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

    Ok(candidate_binaries)
}

fn lint_release_binaries(
    candidate_binaries: &[PathBuf],
) -> Result<ReleaseHbpPair, ReleaseLintError> {
    let mut hbp_pair = ReleaseHbpPair::default();
    for binary_path in candidate_binaries {
        let outcome = lint_release_binary(binary_path)?;
        hbp_pair.record(outcome.role, outcome.hbp_supported);
    }

    Ok(hbp_pair)
}

fn lint_release_binary(binary_path: &Path) -> Result<ReleaseBinaryLintOutcome, ReleaseLintError> {
    let binary_name = file_name_string(binary_path);
    let expected_role = classify_release_binary_role(binary_name.as_str())?;
    if !release_binary_name_is_valid(binary_name.as_str(), expected_role) {
        return Err(ReleaseLintError::InvalidBinaryName { binary_name });
    }

    let sidecar_path = sidecar_path_for_binary(binary_path);
    if !sidecar_path.is_file() {
        return Err(ReleaseLintError::MissingSidecar { sidecar_path });
    }

    let metadata = validate_lint_sidecar(&sidecar_path)?;
    validate_lint_sidecar_role(&metadata, &sidecar_path, expected_role)?;
    validate_lint_sidecar_binary_name(&metadata, &sidecar_path, binary_name.as_str())?;
    let hbp_supported = lint_sidecar_hbp_supported(&metadata, &sidecar_path)?;

    Ok(ReleaseBinaryLintOutcome {
        role: expected_role,
        hbp_supported,
    })
}

fn validate_lint_sidecar(sidecar_path: &Path) -> Result<Value, ReleaseLintError> {
    validate_release_sidecar(sidecar_path).map_err(|source| ReleaseLintError::SidecarInvalid {
        sidecar_path: sidecar_path.to_path_buf(),
        source,
    })
}

fn validate_lint_sidecar_role(
    metadata: &Value,
    sidecar_path: &Path,
    expected_role: BinaryRole,
) -> Result<(), ReleaseLintError> {
    let observed_role = required_lint_sidecar_str(metadata, "binary_role", sidecar_path)?;
    if BinaryRole::parse(observed_role) != Some(expected_role) {
        return Err(ReleaseLintError::SidecarRoleMismatch {
            sidecar_path: sidecar_path.to_path_buf(),
            expected: expected_role,
            observed: observed_role.to_string(),
        });
    }

    Ok(())
}

fn validate_lint_sidecar_binary_name(
    metadata: &Value,
    sidecar_path: &Path,
    expected_binary_name: &str,
) -> Result<(), ReleaseLintError> {
    let observed_binary_name = required_lint_sidecar_str(metadata, "binary_name", sidecar_path)?;
    if observed_binary_name != expected_binary_name {
        return Err(ReleaseLintError::SidecarBinaryNameMismatch {
            sidecar_path: sidecar_path.to_path_buf(),
            expected: expected_binary_name.to_string(),
            observed: observed_binary_name.to_string(),
        });
    }

    Ok(())
}

fn lint_sidecar_hbp_supported(
    metadata: &Value,
    sidecar_path: &Path,
) -> Result<bool, ReleaseLintError> {
    let features = required_object(metadata, "features").map_err(|source| {
        ReleaseLintError::SidecarInvalid {
            sidecar_path: sidecar_path.to_path_buf(),
            source,
        }
    })?;
    required_bool(features, "hbp_supported").map_err(|source| ReleaseLintError::SidecarInvalid {
        sidecar_path: sidecar_path.to_path_buf(),
        source,
    })
}

fn required_lint_sidecar_str<'a>(
    metadata: &'a Value,
    field: &'static str,
    sidecar_path: &Path,
) -> Result<&'a str, ReleaseLintError> {
    required_str(metadata, field).map_err(|source| ReleaseLintError::SidecarInvalid {
        sidecar_path: sidecar_path.to_path_buf(),
        source,
    })
}

fn validate_release_hbp_pair(hbp_pair: &ReleaseHbpPair) -> Result<(), ReleaseLintError> {
    if let (Some(watershed), Some(hillslope)) = (hbp_pair.watershed, hbp_pair.hillslope)
        && watershed != hillslope
    {
        return Err(ReleaseLintError::HbpPairMismatch {
            watershed,
            hillslope,
        });
    }

    Ok(())
}

pub fn write_release_sidecar_for_binary(
    binary_path: &Path,
    role: BinaryRole,
) -> Result<PathBuf, ReleaseMetadataError> {
    let _io_guard = lock_release_sidecar_io();
    let sidecar_path = sidecar_path_for_binary(binary_path);
    if sidecar_is_fresh_for_binary_unlocked(&sidecar_path, binary_path, role) {
        return Ok(sidecar_path);
    }

    let metadata = build_release_metadata_document(binary_path, role)?;
    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|source| ReleaseMetadataError::JsonSerialize { source })?;
    write_release_sidecar_atomically(&sidecar_path, &json)?;

    validate_release_sidecar_unlocked(&sidecar_path)?;
    Ok(sidecar_path)
}

fn write_release_sidecar_atomically(
    sidecar_path: &Path,
    json: &str,
) -> Result<(), ReleaseMetadataError> {
    let temp_path = release_sidecar_temp_path(sidecar_path);
    fs::write(&temp_path, json).map_err(|source| ReleaseMetadataError::Io {
        path: temp_path.clone(),
        source,
    })?;
    fs::rename(&temp_path, sidecar_path).map_err(|source| {
        let _ = fs::remove_file(&temp_path);
        ReleaseMetadataError::Io {
            path: sidecar_path.to_path_buf(),
            source,
        }
    })?;
    Ok(())
}

fn release_sidecar_temp_path(sidecar_path: &Path) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let file_name = sidecar_path
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("openwepp-release-sidecar.json");
    sidecar_path.with_file_name(format!(
        "{}.{}.{}.tmp",
        file_name,
        std::process::id(),
        nanos
    ))
}

fn sidecar_is_fresh_for_binary_unlocked(
    sidecar_path: &Path,
    binary_path: &Path,
    role: BinaryRole,
) -> bool {
    if !sidecar_path.is_file() {
        return false;
    }

    let Ok(metadata) = validate_release_sidecar_unlocked(sidecar_path) else {
        return false;
    };

    let Ok(observed_role) = required_str(&metadata, "binary_role") else {
        return false;
    };
    if BinaryRole::parse(observed_role) != Some(role) {
        return false;
    }

    let Ok(observed_binary_name) = required_str(&metadata, "binary_name") else {
        return false;
    };
    if observed_binary_name != file_name_string(binary_path) {
        return false;
    }

    let binary_mtime = match fs::metadata(binary_path) {
        Ok(meta) => match meta.modified() {
            Ok(modified) => modified,
            Err(_) => return false,
        },
        Err(_) => return false,
    };
    let sidecar_mtime = match fs::metadata(sidecar_path) {
        Ok(meta) => match meta.modified() {
            Ok(modified) => modified,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    sidecar_mtime >= binary_mtime
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
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let token = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("{prefix}_{token}"));
        fs::create_dir_all(&dir).expect("temp directory should be creatable");
        dir
    }

    fn write_fixture_binary(dir: &Path, name: &str) -> PathBuf {
        let binary_path = dir.join(name);
        fs::write(&binary_path, format!("fixture-binary-{name}"))
            .expect("binary fixture should be writable");
        binary_path
    }

    fn write_valid_fixture_sidecar(binary_path: &Path, role: BinaryRole) -> PathBuf {
        write_release_sidecar_for_binary(binary_path, role)
            .expect("fixture sidecar should be writable")
    }

    fn rewrite_sidecar_value(sidecar_path: &Path, update: impl FnOnce(&mut Value)) {
        let payload = fs::read_to_string(sidecar_path).expect("sidecar fixture should be readable");
        let mut json: Value =
            serde_json::from_str(&payload).expect("sidecar fixture should parse as json");
        update(&mut json);
        let rewritten = serde_json::to_string_pretty(&json)
            .expect("rewritten sidecar fixture should serialize");
        fs::write(sidecar_path, rewritten).expect("sidecar fixture should be rewritable");
    }

    #[test]
    fn m09_public_sidecar_validation_preserves_fail_closed_priority() {
        let dir = unique_temp_dir("m09_sidecar_priority");
        let missing = dir.join("missing.json");
        assert!(matches!(
            validate_release_sidecar(&missing),
            Err(ReleaseMetadataError::Io { path, .. }) if path == missing
        ));

        let sidecar = dir.join("release.json");
        fs::write(&sidecar, "{not json").expect("malformed fixture must be writable");
        assert!(matches!(
            validate_release_sidecar(&sidecar),
            Err(ReleaseMetadataError::JsonParse { path, .. }) if path == sidecar
        ));

        fs::write(&sidecar, r#"{"binary_role":"not-a-role"}"#)
            .expect("missing-schema fixture must be writable");
        assert!(matches!(
            validate_release_sidecar(&sidecar),
            Err(ReleaseMetadataError::MissingField { field: "schema" })
        ));

        fs::write(&sidecar, r#"{"schema":"wrong","binary_role":"not-a-role"}"#)
            .expect("schema-priority fixture must be writable");
        assert!(matches!(
            validate_release_sidecar(&sidecar),
            Err(ReleaseMetadataError::InvalidField { field: "schema", detail })
                if detail.ends_with("observed wrong")
        ));

        fs::write(
            &sidecar,
            format!(r#"{{"schema":"{BINARY_RELEASE_SCHEMA_ID}","binary_role":"not-a-role"}}"#),
        )
        .expect("role-priority fixture must be writable");
        assert!(matches!(
            validate_release_sidecar(&sidecar),
            Err(ReleaseMetadataError::InvalidField { field: "binary_role", detail })
                if detail == "unsupported role not-a-role"
        ));

        fs::write(
            &sidecar,
            format!(r#"{{"schema":"{BINARY_RELEASE_SCHEMA_ID}","binary_role":"watershed"}}"#),
        )
        .expect("missing-field fixture must be writable");
        assert!(matches!(
            validate_release_sidecar(&sidecar),
            Err(ReleaseMetadataError::MissingField {
                field: "binary_name"
            })
        ));

        fs::remove_dir_all(dir).ok();
    }

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

    #[test]
    fn write_release_sidecar_reuses_fresh_sidecar_without_rewrite() {
        let dir = unique_temp_dir("release_sidecar_reuse");
        let binary_path = dir.join("openwepp_260528_hill");
        fs::write(&binary_path, b"fixture-binary-v1").expect("binary fixture should be writable");

        let sidecar_path = write_release_sidecar_for_binary(&binary_path, BinaryRole::Hillslope)
            .expect("first sidecar write should succeed");
        let first_payload =
            fs::read_to_string(&sidecar_path).expect("first sidecar payload should be readable");
        let first_mtime = fs::metadata(&sidecar_path)
            .expect("first sidecar metadata should be readable")
            .modified()
            .expect("first sidecar mtime should be readable");

        std::thread::sleep(Duration::from_millis(5));
        let second_sidecar_path =
            write_release_sidecar_for_binary(&binary_path, BinaryRole::Hillslope)
                .expect("second sidecar write should succeed");
        let second_payload = fs::read_to_string(&second_sidecar_path)
            .expect("second sidecar payload should be readable");
        let second_mtime = fs::metadata(&second_sidecar_path)
            .expect("second sidecar metadata should be readable")
            .modified()
            .expect("second sidecar mtime should be readable");

        assert_eq!(sidecar_path, second_sidecar_path);
        assert_eq!(first_payload, second_payload);
        assert_eq!(
            first_mtime, second_mtime,
            "fresh sidecar should be reused without rewrite"
        );

        fs::remove_dir_all(dir).expect("temp directory cleanup should succeed");
    }

    #[test]
    fn lint_release_directory_accepts_valid_candidates_and_ignores_non_candidates() {
        let dir = unique_temp_dir("release_lint_success");
        let watershed_path = write_fixture_binary(&dir, "openwepp_260528");
        let hillslope_path = write_fixture_binary(&dir, "openwepp_260528_hill");
        let _ignored_json = write_fixture_binary(&dir, "openwepp_260528_replay.json");
        let nested_dir = dir.join("openwepp_260528_replay");
        fs::create_dir_all(&nested_dir).expect("nested fixture directory should be creatable");

        write_valid_fixture_sidecar(&watershed_path, BinaryRole::Watershed);
        write_valid_fixture_sidecar(&hillslope_path, BinaryRole::Hillslope);

        let report =
            lint_release_directory(&dir).expect("valid release candidates should lint cleanly");
        let mut checked_names = report
            .checked_binaries
            .iter()
            .map(|path| file_name_string(path))
            .collect::<Vec<_>>();
        checked_names.sort();

        assert_eq!(checked_names, ["openwepp_260528", "openwepp_260528_hill"]);

        fs::remove_dir_all(dir).expect("temp directory cleanup should succeed");
    }

    #[test]
    fn lint_release_directory_reports_no_release_candidates_after_filtering() {
        let dir = unique_temp_dir("release_lint_no_candidates");
        fs::write(dir.join("notes.txt"), b"not a candidate")
            .expect("non-candidate fixture should be writable");
        fs::write(dir.join("openwepp_260528.json"), b"{}")
            .expect("json sidecar fixture should be writable");

        let error = lint_release_directory(&dir).expect_err("no candidates should fail lint");
        match error {
            ReleaseLintError::NoReleaseCandidates { release_dir } => assert_eq!(release_dir, dir),
            other => panic!("expected no release candidates error, observed {other:?}"),
        }

        fs::remove_dir_all(dir).expect("temp directory cleanup should succeed");
    }

    #[test]
    fn lint_release_directory_reports_missing_sidecar_for_candidate() {
        let dir = unique_temp_dir("release_lint_missing_sidecar");
        let binary_path = write_fixture_binary(&dir, "openwepp_260528_hill");
        let expected_sidecar = sidecar_path_for_binary(&binary_path);

        let error = lint_release_directory(&dir).expect_err("missing sidecar should fail lint");
        match error {
            ReleaseLintError::MissingSidecar { sidecar_path } => {
                assert_eq!(sidecar_path, expected_sidecar);
            }
            other => panic!("expected missing sidecar error, observed {other:?}"),
        }

        fs::remove_dir_all(dir).expect("temp directory cleanup should succeed");
    }

    #[test]
    fn lint_release_directory_reports_invalid_binary_name_before_sidecar_read() {
        let dir = unique_temp_dir("release_lint_invalid_name");
        let binary_path = write_fixture_binary(&dir, "openwepp_26052_hill");
        let sidecar_path = sidecar_path_for_binary(&binary_path);
        fs::write(&sidecar_path, b"{not json").expect("malformed sidecar fixture is writable");

        let error = lint_release_directory(&dir).expect_err("invalid binary name should fail lint");
        match error {
            ReleaseLintError::InvalidBinaryName { binary_name } => {
                assert_eq!(binary_name, "openwepp_26052_hill");
            }
            other => panic!("expected invalid binary name error, observed {other:?}"),
        }

        fs::remove_dir_all(dir).expect("temp directory cleanup should succeed");
    }

    #[test]
    fn lint_release_directory_reports_sidecar_binary_name_mismatch() {
        let dir = unique_temp_dir("release_lint_binary_name_mismatch");
        let binary_path = write_fixture_binary(&dir, "openwepp_260528_hill");
        let sidecar_path = write_valid_fixture_sidecar(&binary_path, BinaryRole::Hillslope);
        rewrite_sidecar_value(&sidecar_path, |json| {
            json["binary_name"] = Value::String("openwepp_260528_other_hill".to_string());
        });

        let error =
            lint_release_directory(&dir).expect_err("binary_name mismatch should fail lint");
        match error {
            ReleaseLintError::SidecarBinaryNameMismatch {
                sidecar_path: observed_sidecar,
                expected,
                observed,
            } => {
                assert_eq!(observed_sidecar, sidecar_path);
                assert_eq!(expected, "openwepp_260528_hill");
                assert_eq!(observed, "openwepp_260528_other_hill");
            }
            other => panic!("expected sidecar binary name mismatch, observed {other:?}"),
        }

        fs::remove_dir_all(dir).expect("temp directory cleanup should succeed");
    }

    #[test]
    fn lint_release_directory_reports_hbp_pair_mismatch_after_candidate_scan() {
        let dir = unique_temp_dir("release_lint_hbp_pair_mismatch");
        let watershed_path = write_fixture_binary(&dir, "openwepp_260528");
        let hillslope_path = write_fixture_binary(&dir, "openwepp_260528_hill");
        write_valid_fixture_sidecar(&watershed_path, BinaryRole::Watershed);
        let hillslope_sidecar = write_valid_fixture_sidecar(&hillslope_path, BinaryRole::Hillslope);
        rewrite_sidecar_value(&hillslope_sidecar, |json| {
            json["features"]["hbp_supported"] = Value::Bool(false);
        });

        let error = lint_release_directory(&dir).expect_err("hbp mismatch should fail lint");
        match error {
            ReleaseLintError::HbpPairMismatch {
                watershed,
                hillslope,
            } => {
                assert!(watershed);
                assert!(!hillslope);
            }
            other => panic!("expected hbp pair mismatch, observed {other:?}"),
        }

        fs::remove_dir_all(dir).expect("temp directory cleanup should succeed");
    }

    #[test]
    fn write_release_sidecar_rewrites_when_binary_is_newer() {
        let dir = unique_temp_dir("release_sidecar_refresh");
        let binary_path = dir.join("openwepp_260528_hill");
        fs::write(&binary_path, b"fixture-binary-v1").expect("binary fixture should be writable");

        let sidecar_path = write_release_sidecar_for_binary(&binary_path, BinaryRole::Hillslope)
            .expect("initial sidecar write should succeed");
        let first_payload =
            fs::read_to_string(&sidecar_path).expect("first sidecar payload should be readable");

        std::thread::sleep(Duration::from_millis(5));
        fs::write(&binary_path, b"fixture-binary-v2")
            .expect("binary fixture update should be writable");
        std::thread::sleep(Duration::from_millis(5));
        let _ = write_release_sidecar_for_binary(&binary_path, BinaryRole::Hillslope)
            .expect("refreshed sidecar write should succeed");
        let second_payload =
            fs::read_to_string(&sidecar_path).expect("second sidecar payload should be readable");

        assert_ne!(
            first_payload, second_payload,
            "newer binary should force sidecar refresh"
        );

        fs::remove_dir_all(dir).expect("temp directory cleanup should succeed");
    }
}
