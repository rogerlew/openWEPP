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
    let Some(mirror) = configured_mirror_root()? else {
        return Ok(());
    };
    let roots = prepare_mirror_roots(artifact_root, &mirror)?;
    mirror_node_outputs(&roots, node)?;
    publish_mirrored_checkpoint(&roots.mirror, node, checkpoint)
}

fn configured_mirror_root() -> Result<Option<PathBuf>> {
    let Some(configured) = std::env::var_os("OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT") else {
        return Ok(None);
    };
    let mirror = PathBuf::from(configured);
    validate_configured_mirror_root(&mirror)?;
    Ok(Some(mirror))
}

fn validate_configured_mirror_root(mirror: &Path) -> Result<()> {
    if !mirror.is_absolute() || mirror.starts_with("/tmp") || mirror.starts_with("/t") {
        return Err(mirror_error(
            "GATE-EXEC-CHECKPOINT-MIRROR-EPHEMERAL",
            mirror.display().to_string(),
        ));
    }
    Ok(())
}

struct MirrorRoots<'a> {
    mirror: PathBuf,
    artifact_root: &'a Path,
}

fn prepare_mirror_roots<'a>(artifact_root: &'a Path, mirror: &Path) -> Result<MirrorRoots<'a>> {
    create_absolute_directories(mirror)?;
    let mirror = mirror
        .canonicalize()
        .map_err(|error| mirror_error("GATE-EXEC-CHECKPOINT-MIRROR", error.to_string()))?;
    let artifact = artifact_root
        .canonicalize()
        .map_err(|error| mirror_error("GATE-EXEC-CHECKPOINT-MIRROR", error.to_string()))?;
    validate_mirror_root_aliases(&mirror, &artifact)?;
    Ok(MirrorRoots {
        mirror,
        artifact_root,
    })
}

fn validate_mirror_root_aliases(mirror: &Path, artifact: &Path) -> Result<()> {
    if mirror.starts_with("/tmp")
        || mirror.starts_with("/t")
        || mirror.starts_with(artifact)
        || artifact.starts_with(mirror)
    {
        return Err(mirror_error(
            "GATE-EXEC-CHECKPOINT-MIRROR-ROOT-ALIAS",
            mirror.display().to_string(),
        ));
    }
    Ok(())
}

fn mirror_node_outputs(roots: &MirrorRoots<'_>, node: &Value) -> Result<()> {
    for relative in string_array(&node["output_paths"], "output_paths")? {
        mirror_node_output(roots, &relative)?;
    }
    Ok(())
}

fn mirror_node_output(roots: &MirrorRoots<'_>, relative: &str) -> Result<()> {
    let source = confined_output_path(roots.artifact_root, relative)?;
    let destination = prepare_mirror_destination(&roots.mirror, relative)?;
    copy_mirror_output(&source, &destination, relative)
}

fn prepare_mirror_destination(mirror: &Path, relative: &str) -> Result<PathBuf> {
    let destination = confined_output_path(mirror, relative)?;
    let parent = destination
        .parent()
        .ok_or_else(|| mirror_error("GATE-EXEC-CHECKPOINT-MIRROR", relative))?;
    create_confined_directories(mirror, parent)?;
    Ok(destination)
}

fn copy_mirror_output(source: &Path, destination: &Path, relative: &str) -> Result<()> {
    let bytes = fs::read(source).map_err(|error| {
        mirror_error(
            "GATE-EXEC-CHECKPOINT-MIRROR",
            format!("{relative}: {error}"),
        )
    })?;
    write_atomic(destination, &bytes)
}

fn publish_mirrored_checkpoint(mirror: &Path, node: &Value, checkpoint: &Value) -> Result<()> {
    let directory = mirror.join(".checkpoints");
    create_confined_directories(mirror, &directory)?;
    let node_id = required_string(node, "node_id")?;
    write_atomic(
        &directory.join(format!("{node_id}.json")),
        &canonical_bytes(checkpoint)?,
    )
}

fn create_absolute_directories(path: &Path) -> Result<()> {
    let mut current = PathBuf::from("/");
    for component in path.components() {
        if push_absolute_component(&mut current, component, path)? {
            ensure_absolute_directory(&current)?;
        }
    }
    Ok(())
}

fn push_absolute_component(
    current: &mut PathBuf,
    component: Component<'_>,
    path: &Path,
) -> Result<bool> {
    match component {
        Component::RootDir => Ok(false),
        Component::Normal(name) => {
            current.push(name);
            Ok(true)
        }
        _ => Err(mirror_error(
            "GATE-EXEC-CHECKPOINT-MIRROR-PATH",
            path.display().to_string(),
        )),
    }
}

fn ensure_absolute_directory(current: &Path) -> Result<()> {
    match fs::symlink_metadata(current) {
        Ok(metadata) => validate_directory_metadata(current, &metadata),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => fs::create_dir(current)
            .map_err(|error| mirror_error("GATE-EXEC-CHECKPOINT-MIRROR", error.to_string())),
        Err(error) => Err(mirror_error(
            "GATE-EXEC-CHECKPOINT-MIRROR",
            error.to_string(),
        )),
    }
}

fn validate_directory_metadata(current: &Path, metadata: &fs::Metadata) -> Result<()> {
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(mirror_error(
            "GATE-EXEC-CHECKPOINT-MIRROR-SYMLINK",
            current.display().to_string(),
        ));
    }
    Ok(())
}

fn mirror_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Execution, code, message)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Output};
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde_json::json;

    use super::{
        create_absolute_directories, mirror_node_checkpoint, prepare_mirror_roots,
        push_absolute_component,
    };
    use crate::canonical::canonical_bytes;

    const CHILD_TEST: &str = "checkpoint_mirror::tests::mirror_scenario_child";
    static NEXT_SCRATCH: AtomicU64 = AtomicU64::new(0);

    struct Scratch(PathBuf);

    impl Scratch {
        fn new(label: &str) -> Self {
            let sequence = NEXT_SCRATCH.fetch_add(1, Ordering::Relaxed);
            let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .expect("canonical repository root");
            let path = repository.join("target").join(format!(
                "checkpoint-mirror-{label}-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("create checkpoint-mirror scratch");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).expect("remove checkpoint-mirror scratch");
        }
    }

    fn run_child(scenario: &str, scratch: &Path, mirror: Option<&Path>) -> Output {
        let mut command = Command::new(std::env::current_exe().expect("current test executable"));
        command
            .args(["--exact", CHILD_TEST, "--ignored", "--nocapture"])
            .env("OPENWEPP_CHECKPOINT_MIRROR_TEST_SCENARIO", scenario)
            .env("OPENWEPP_CHECKPOINT_MIRROR_TEST_SCRATCH", scratch);
        if let Some(mirror) = mirror {
            command.env("OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT", mirror);
        } else {
            command.env_remove("OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT");
        }
        command.output().expect("run isolated mirror scenario")
    }

    fn assert_child_passes(scenario: &str, scratch: &Path, mirror: Option<&Path>) {
        let output = run_child(scenario, scratch, mirror);
        assert!(
            output.status.success(),
            "scenario {scenario} failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn checkpoint_mirror_characterization() {
        let absent = Scratch::new("absent");
        assert_child_passes("absent", absent.path(), None);

        let ephemeral = Scratch::new("ephemeral");
        assert_child_passes(
            "ephemeral",
            ephemeral.path(),
            Some(Path::new("/tmp/openwepp-checkpoint-mirror-rejected")),
        );

        let relative = Scratch::new("relative");
        assert_child_passes(
            "ephemeral",
            relative.path(),
            Some(Path::new("relative-mirror-root")),
        );

        let success = Scratch::new("success");
        assert_child_passes(
            "success",
            success.path(),
            Some(&success.path().join("mirror")),
        );

        let alias = Scratch::new("alias");
        let artifact = alias.path().join("artifact");
        fs::create_dir(&artifact).expect("create alias artifact");
        assert_child_passes("alias", alias.path(), Some(&artifact));

        let lexical = Scratch::new("lexical");
        assert_child_passes(
            "lexical-error",
            lexical.path(),
            Some(&lexical.path().join("mirror")),
        );

        let symlink = Scratch::new("symlink");
        let real = symlink.path().join("real");
        fs::create_dir(&real).expect("create symlink target");
        std::os::unix::fs::symlink(&real, symlink.path().join("link"))
            .expect("create mirror symlink");
        assert_child_passes(
            "symlink",
            symlink.path(),
            Some(&symlink.path().join("link/child")),
        );
    }

    #[test]
    fn absolute_directory_creation_rejects_invalid_components_and_entries() {
        let scratch = Scratch::new("directories");
        let mut root = PathBuf::from("/");
        let root_component = Path::new("/").components().next().expect("root component");
        assert!(
            !push_absolute_component(&mut root, root_component, Path::new("/"))
                .expect("root component is valid"),
            "root component must skip metadata lookup"
        );
        let parent_path = scratch.path().join("created/../escape");
        let error = create_absolute_directories(&parent_path)
            .expect_err("parent component must fail closed");
        assert_eq!(error.code, "GATE-EXEC-CHECKPOINT-MIRROR-PATH");

        let regular = scratch.path().join("regular");
        fs::write(&regular, "not a directory").expect("write regular entry");
        let error = create_absolute_directories(&regular.join("child"))
            .expect_err("regular-file component must fail closed");
        assert_eq!(error.code, "GATE-EXEC-CHECKPOINT-MIRROR-SYMLINK");

        let target = scratch.path().join("target");
        fs::create_dir(&target).expect("create target directory");
        let link = scratch.path().join("link");
        std::os::unix::fs::symlink(&target, &link).expect("create directory symlink");
        let error = create_absolute_directories(&link.join("child"))
            .expect_err("symlink component must fail closed");
        assert_eq!(error.code, "GATE-EXEC-CHECKPOINT-MIRROR-SYMLINK");
    }

    #[test]
    #[ignore = "executed only as an isolated characterization subprocess"]
    fn mirror_scenario_child() {
        let scenario =
            std::env::var("OPENWEPP_CHECKPOINT_MIRROR_TEST_SCENARIO").expect("mirror scenario");
        let scratch = PathBuf::from(
            std::env::var_os("OPENWEPP_CHECKPOINT_MIRROR_TEST_SCRATCH").expect("mirror scratch"),
        );
        let artifact = if scenario == "lexical-error" {
            let target = scratch.join("artifact-target");
            fs::create_dir_all(&target).expect("create lexical artifact target");
            let link = scratch.join("artifact-link");
            std::os::unix::fs::symlink(&target, &link).expect("create lexical artifact link");
            link
        } else {
            let artifact = scratch.join("artifact");
            fs::create_dir_all(&artifact).expect("create artifact root");
            artifact
        };
        let node = json!({
            "node_id": "a".repeat(64),
            "output_paths": ["outputs/one.txt", "nested/two.bin"]
        });
        let checkpoint = json!({"z": 2, "a": [3, 1]});

        match scenario.as_str() {
            "absent" => mirror_node_checkpoint(&artifact, &node, &checkpoint)
                .expect("absent mirror is optional"),
            "ephemeral" => {
                let error = mirror_node_checkpoint(&artifact, &node, &checkpoint)
                    .expect_err("ephemeral or relative root must fail");
                assert_eq!(error.code, "GATE-EXEC-CHECKPOINT-MIRROR-EPHEMERAL");
            }
            "alias" => {
                let error = mirror_node_checkpoint(&artifact, &node, &checkpoint)
                    .expect_err("artifact alias must fail");
                assert_eq!(error.code, "GATE-EXEC-CHECKPOINT-MIRROR-ROOT-ALIAS");
            }
            "lexical-error" => {
                let mirror = scratch.join("mirror");
                let roots =
                    prepare_mirror_roots(&artifact, &mirror).expect("prepare lexical mirror roots");
                assert_eq!(roots.artifact_root, artifact);
                let error = mirror_node_checkpoint(&artifact, &node, &checkpoint)
                    .expect_err("missing source must retain lexical artifact path");
                assert_eq!(error.code, "GATE-EXEC-CHECKPOINT-MIRROR");
                assert!(error.message.starts_with("outputs/one.txt:"));
            }
            "symlink" => {
                let error = mirror_node_checkpoint(&artifact, &node, &checkpoint)
                    .expect_err("symlink root must fail");
                assert_eq!(error.code, "GATE-EXEC-CHECKPOINT-MIRROR-SYMLINK");
            }
            "success" => {
                fs::create_dir_all(artifact.join("outputs")).expect("create output directory");
                fs::create_dir_all(artifact.join("nested")).expect("create nested directory");
                fs::write(artifact.join("outputs/one.txt"), b"first").expect("write first output");
                fs::write(artifact.join("nested/two.bin"), [0_u8, 255, 7])
                    .expect("write binary output");
                mirror_node_checkpoint(&artifact, &node, &checkpoint).expect("mirror checkpoint");
                let mirror = scratch.join("mirror");
                assert_eq!(
                    fs::read(mirror.join("outputs/one.txt")).expect("read mirrored text"),
                    b"first"
                );
                assert_eq!(
                    fs::read(mirror.join("nested/two.bin")).expect("read mirrored binary"),
                    [0_u8, 255, 7]
                );
                assert_eq!(
                    fs::read(mirror.join(format!(".checkpoints/{}.json", "a".repeat(64))))
                        .expect("read mirrored checkpoint"),
                    canonical_bytes(&checkpoint).expect("canonical checkpoint")
                );
            }
            unexpected => panic!("unexpected mirror scenario {unexpected}"),
        }
    }
}
