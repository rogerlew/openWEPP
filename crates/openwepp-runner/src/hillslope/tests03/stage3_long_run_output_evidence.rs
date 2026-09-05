fn release_scale_normalize_manifest_value(
    value: &mut serde_json::Value,
    run_dir: &str,
) -> Result<(), String> {
    match value {
        serde_json::Value::String(text) => {
            if text.contains(run_dir) {
                *text = text.replace(run_dir, "$RUN_DIR");
            }
        }
        serde_json::Value::Array(values) => {
            for value in values {
                release_scale_normalize_manifest_value(value, run_dir)?;
            }
        }
        serde_json::Value::Object(values) => {
            let original = std::mem::take(values);
            for (key, mut value) in original {
                release_scale_normalize_manifest_value(&mut value, run_dir)?;
                let normalized_key = key.replace(run_dir, "$RUN_DIR");
                if values.insert(normalized_key, value).is_some() {
                    return Err("manifest path normalization produced a duplicate key".to_owned());
                }
            }
        }
        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {}
    }
    Ok(())
}

fn release_scale_normalized_manifest_bytes(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut manifest: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let object = manifest
        .as_object_mut()
        .ok_or_else(|| "qualification manifest root is not an object".to_owned())?;
    let run_dir = object
        .get("run_dir")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "qualification manifest lacks a nonempty run_dir".to_owned())?
        .to_owned();
    let invoked_utc = object
        .get_mut("invoked_utc")
        .ok_or_else(|| "qualification manifest lacks invoked_utc".to_owned())?;
    if !invoked_utc.is_string() {
        return Err("qualification manifest invoked_utc is not a string".to_owned());
    }
    *invoked_utc = serde_json::Value::String("$INVOKED_UTC".to_owned());
    release_scale_normalize_manifest_value(&mut manifest, &run_dir)?;
    serde_json::to_vec(&manifest).map_err(|error| error.to_string())
}

fn release_scale_scientific_output_hashes(
    root: &Path,
) -> (String, std::collections::BTreeMap<String, String>) {
    use sha2::{Digest as _, Sha256};

    fn collect(root: &Path, dir: &Path, paths: &mut Vec<PathBuf>) {
        let mut entries = std::fs::read_dir(dir)
            .expect("read qualification output directory")
            .map(|entry| entry.expect("read qualification output entry").path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                collect(root, &path, paths);
            } else {
                paths.push(
                    path.strip_prefix(root)
                        .expect("output below qualification root")
                        .to_path_buf(),
                );
            }
        }
    }

    let mut paths = Vec::new();
    collect(root, root, &mut paths);
    let mut digest = Sha256::new();
    let mut file_hashes = std::collections::BTreeMap::new();
    for relative in paths {
        let raw = std::fs::read(root.join(&relative)).expect("read qualification output file");
        let (identity, bytes) = if relative == Path::new("openwepp_hillslope_run_manifest.json") {
            (
                "openwepp_hillslope_run_manifest.json#semantic-v1".to_owned(),
                release_scale_normalized_manifest_bytes(&raw)
                    .expect("normalize qualification semantic manifest"),
            )
        } else {
            (relative.to_string_lossy().into_owned(), raw)
        };
        digest.update(identity.as_bytes());
        digest.update([0]);
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(&bytes);
        file_hashes.insert(identity, format!("{:x}", Sha256::digest(bytes)));
    }
    (format!("{:x}", digest.finalize()), file_hashes)
}

#[test]
fn release_scale_semantic_manifest_normalizes_only_invocation_identity() {
    let first = serde_json::json!({
        "schema": "openwepp-hillslope-run-manifest-v1",
        "invoked_utc": "2026-09-03T01:02:03Z",
        "run_dir": "/tmp/first",
        "run_file": "/tmp/first/case.run",
        "argv": ["runner", "--run-dir", "/tmp/first"],
        "output_checksums": {"/tmp/first/output/H83.hbp": "abc"},
        "scientific": {"source_m3": 1.25},
    });
    let mut second = first.clone();
    second["invoked_utc"] = serde_json::json!("2026-09-04T05:06:07Z");
    second["run_dir"] = serde_json::json!("/tmp/second");
    second["run_file"] = serde_json::json!("/tmp/second/case.run");
    second["argv"][2] = serde_json::json!("/tmp/second");
    second["output_checksums"] =
        serde_json::json!({"/tmp/second/output/H83.hbp": "abc"});
    let first = release_scale_normalized_manifest_bytes(
        &serde_json::to_vec(&first).expect("encode first manifest"),
    )
    .expect("normalize first manifest");
    let second = release_scale_normalized_manifest_bytes(
        &serde_json::to_vec(&second).expect("encode second manifest"),
    )
    .expect("normalize second manifest");
    assert_eq!(first, second);

    let mut poisoned: serde_json::Value =
        serde_json::from_slice(&second).expect("parse normalized manifest");
    poisoned["scientific"]["source_m3"] = serde_json::json!(1.5);
    assert_ne!(
        first,
        serde_json::to_vec(&poisoned).expect("encode poisoned manifest")
    );
}
