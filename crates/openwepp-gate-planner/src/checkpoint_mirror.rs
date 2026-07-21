//! Durable per-node checkpoint mirroring outside ephemeral execution roots.

use std::fs;
use std::path::{Component, Path, PathBuf};

use serde_json::Value;

use crate::artifact_contract::create_confined_directories;
use crate::canonical::canonical_bytes;
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::executor::{confined_output_path, required_string, string_array, write_atomic};

pub(crate) fn mirror_node_checkpoint(
    artifact_root: &Path,
    node: &Value,
    checkpoint: &Value,
) -> Result<()> {
    let Some(configured) = std::env::var_os("OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT") else {
        return Ok(());
    };
    let mirror = PathBuf::from(configured);
    if !mirror.is_absolute() || mirror.starts_with("/tmp") || mirror.starts_with("/t") {
        return Err(mirror_error(
            "GATE-EXEC-CHECKPOINT-MIRROR-EPHEMERAL",
            mirror.display().to_string(),
        ));
    }
    create_absolute_directories(&mirror)?;
    let mirror = mirror
        .canonicalize()
        .map_err(|error| mirror_error("GATE-EXEC-CHECKPOINT-MIRROR", error.to_string()))?;
    let artifact = artifact_root
        .canonicalize()
        .map_err(|error| mirror_error("GATE-EXEC-CHECKPOINT-MIRROR", error.to_string()))?;
    if mirror.starts_with("/tmp")
        || mirror.starts_with("/t")
        || mirror.starts_with(&artifact)
        || artifact.starts_with(&mirror)
    {
        return Err(mirror_error(
            "GATE-EXEC-CHECKPOINT-MIRROR-ROOT-ALIAS",
            mirror.display().to_string(),
        ));
    }
    for relative in string_array(&node["output_paths"], "output_paths")? {
        let source = confined_output_path(artifact_root, &relative)?;
        let destination = confined_output_path(&mirror, &relative)?;
        let parent = destination
            .parent()
            .ok_or_else(|| mirror_error("GATE-EXEC-CHECKPOINT-MIRROR", &relative))?;
        create_confined_directories(&mirror, parent)?;
        let bytes = fs::read(&source).map_err(|error| {
            mirror_error(
                "GATE-EXEC-CHECKPOINT-MIRROR",
                format!("{relative}: {error}"),
            )
        })?;
        write_atomic(&destination, &bytes)?;
    }
    let directory = mirror.join(".checkpoints");
    create_confined_directories(&mirror, &directory)?;
    let node_id = required_string(node, "node_id")?;
    write_atomic(
        &directory.join(format!("{node_id}.json")),
        &canonical_bytes(checkpoint)?,
    )
}

fn create_absolute_directories(path: &Path) -> Result<()> {
    let mut current = PathBuf::from("/");
    for component in path.components() {
        match component {
            Component::RootDir => continue,
            Component::Normal(name) => current.push(name),
            _ => {
                return Err(mirror_error(
                    "GATE-EXEC-CHECKPOINT-MIRROR-PATH",
                    path.display().to_string(),
                ));
            }
        }
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(mirror_error(
                    "GATE-EXEC-CHECKPOINT-MIRROR-SYMLINK",
                    current.display().to_string(),
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => fs::create_dir(&current)
                .map_err(|error| mirror_error("GATE-EXEC-CHECKPOINT-MIRROR", error.to_string()))?,
            Err(error) => {
                return Err(mirror_error(
                    "GATE-EXEC-CHECKPOINT-MIRROR",
                    error.to_string(),
                ));
            }
        }
    }
    Ok(())
}

fn mirror_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Execution, code, message)
}
