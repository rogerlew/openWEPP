use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::canonical::{digest, sha256_bytes};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::planner::command_identity;
use crate::repository::neutral_git_command;

pub(crate) fn environment_record(repo: &Path, target: &str) -> Result<Value> {
    let compiler = command_identity(repo, "rustc", &["-Vv"])?;
    let mut variables = BTreeMap::new();
    for (key, value) in std::env::vars_os() {
        let Some(key) = key.to_str() else {
            return Err(GatePolicyError::new(
                ErrorClass::Planning,
                "GATE-ENVIRONMENT-NONUTF8",
                "environment key",
            ));
        };
        let value = value.to_str().ok_or_else(|| {
            GatePolicyError::new(ErrorClass::Planning, "GATE-ENVIRONMENT-NONUTF8", key)
        })?;
        variables.insert(key.to_owned(), value.to_owned());
    }
    let variables = json!({
        "variables": variables,
        "cargo_configuration": cargo_configuration_manifest(repo)?,
        "git_local_configuration_sha256": git_local_configuration_digest(repo)?
    });
    let runner_image_sha256 = match std::env::var("OPENWEPP_RUNNER_IMAGE_ID") {
        Ok(image) => {
            let digest = image.strip_prefix("sha256:").ok_or_else(|| {
                GatePolicyError::new(
                    ErrorClass::Planning,
                    "GATE-RUNNER-IMAGE-IDENTITY",
                    "OPENWEPP_RUNNER_IMAGE_ID must use sha256:<digest>",
                )
            })?;
            if digest.len() != 64
                || !digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            {
                return Err(GatePolicyError::new(
                    ErrorClass::Planning,
                    "GATE-RUNNER-IMAGE-IDENTITY",
                    "OPENWEPP_RUNNER_IMAGE_ID must contain a lowercase SHA-256 digest",
                ));
            }
            Some(digest.to_owned())
        }
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(_)) => {
            return Err(GatePolicyError::new(
                ErrorClass::Planning,
                "GATE-RUNNER-IMAGE-IDENTITY",
                "OPENWEPP_RUNNER_IMAGE_ID is not UTF-8",
            ));
        }
    };
    Ok(json!({
        "platform": format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        "target_triple": target,
        "compiler": format!("rustc-{}", &sha256_bytes(compiler.as_bytes())[..16]),
        "features": ["default"],
        "variables_sha256": digest(&variables)?,
        "runner_image_sha256": runner_image_sha256
    }))
}

fn git_local_configuration_digest(repo: &Path) -> Result<String> {
    let output = neutral_git_command()
        .args(["config", "--local", "--null", "--list"])
        .current_dir(repo)
        .output()
        .map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-GIT-CONFIG", error.to_string())
        })?;
    if output.status.success() {
        Ok(sha256_bytes(&output.stdout))
    } else {
        Err(GatePolicyError::new(
            ErrorClass::GitState,
            "GATE-GIT-CONFIG",
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

pub(crate) fn cargo_configuration_manifest(repo: &Path) -> Result<Value> {
    let candidates = cargo_configuration_candidates(repo);
    let repository = fs::canonicalize(repo).map_err(|error| {
        GatePolicyError::new(ErrorClass::Io, "GATE-CARGO-CONFIG", error.to_string())
    })?;
    let mut records = Vec::new();
    for path in candidates {
        if let Some(record) = cargo_configuration_record(&repository, &path)? {
            records.push(record);
        }
    }
    Ok(Value::Array(records))
}

fn cargo_configuration_candidates(repo: &Path) -> std::collections::BTreeSet<PathBuf> {
    let mut candidates = std::collections::BTreeSet::new();
    for ancestor in repo.ancestors() {
        candidates.insert(ancestor.join(".cargo/config"));
        candidates.insert(ancestor.join(".cargo/config.toml"));
    }
    if let Some(home) = std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cargo")))
    {
        candidates.insert(home.join("config"));
        candidates.insert(home.join("config.toml"));
    }
    candidates
}

fn cargo_configuration_record(repository: &Path, path: &Path) -> Result<Option<Value>> {
    match fs::read(path) {
        Ok(bytes) => active_cargo_configuration_record(repository, path, &bytes).map(Some),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(GatePolicyError::new(
            ErrorClass::Io,
            "GATE-CARGO-CONFIG",
            format!("{}: {error}", path.display()),
        )),
    }
}

fn active_cargo_configuration_record(
    repository: &Path,
    path: &Path,
    bytes: &[u8],
) -> Result<Value> {
    let canonical = fs::canonicalize(path).map_err(|error| {
        GatePolicyError::new(ErrorClass::Io, "GATE-CARGO-CONFIG", error.to_string())
    })?;
    if !canonical.starts_with(repository) {
        return Err(GatePolicyError::new(
            ErrorClass::CargoMetadata,
            "GATE-CARGO-EXTERNAL-CONFIG",
            format!(
                "external Cargo configuration is unsupported: {}",
                canonical.display()
            ),
        ));
    }
    let path = canonical.to_str().ok_or_else(|| {
        GatePolicyError::new(
            ErrorClass::Planning,
            "GATE-CARGO-CONFIG-NONUTF8",
            canonical.display().to_string(),
        )
    })?;
    Ok(json!({
        "path": path,
        "sha256": sha256_bytes(bytes)
    }))
}
