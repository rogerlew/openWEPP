use serde_json::Value;
use std::process::Command;
use std::{fs, path::Path};

use openwepp_persisted_restart_v1::{
    DirectV10RealConsumerCheckpointV1, from_canonical_bytes, to_canonical_bytes,
};

const ARTIFACTS: &str =
    "docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/artifacts";

#[test]
fn production_codec_round_trips_every_released_vector_byte_identically() {
    for name in [
        "checkpoint-vector.json",
        "checkpoint-in-progress-vector.json",
        "checkpoint-cross-midnight-vector.json",
        "checkpoint-multi-destination-vector.json",
    ] {
        let bytes = fs::read(Path::new(ARTIFACTS).join(name)).unwrap();
        let checkpoint: DirectV10RealConsumerCheckpointV1 = from_canonical_bytes(&bytes).unwrap();
        assert_eq!(to_canonical_bytes(&checkpoint).unwrap(), bytes, "{name}");
    }
}

#[test]
fn production_codec_rejects_noncanonical_and_duplicate_bytes() {
    let bytes = fs::read(Path::new(ARTIFACTS).join("checkpoint-vector.json")).unwrap();
    let mut with_newline = bytes.clone();
    with_newline.push(b'\n');
    assert!(from_canonical_bytes::<DirectV10RealConsumerCheckpointV1>(&with_newline).is_err());

    let marker = b"\"version\":1";
    let offset = bytes
        .windows(marker.len())
        .position(|window| window == marker)
        .unwrap();
    let mut duplicate = Vec::with_capacity(bytes.len() + marker.len() + 1);
    duplicate.extend_from_slice(&bytes[..offset + marker.len()]);
    duplicate.extend_from_slice(b",\"version\":1");
    duplicate.extend_from_slice(&bytes[offset + marker.len()..]);
    assert!(from_canonical_bytes::<DirectV10RealConsumerCheckpointV1>(&duplicate).is_err());
}

#[test]
fn v1_rejects_explicit_stage3_null_as_an_extra_member() {
    let bytes = fs::read(Path::new(ARTIFACTS).join("checkpoint-in-progress-vector.json")).unwrap();
    let mut value: Value = serde_json::from_slice(&bytes).unwrap();
    value["phase"]["staged_scientific"]["direct_hydrology"]
        .as_object_mut()
        .unwrap()
        .insert("snow_stage3_shadow".into(), Value::Null);
    let poisoned = serde_json::to_vec(&value).unwrap();
    let parsed: Value = serde_json::from_slice(&poisoned).expect("poison remains valid JSON");
    assert_eq!(
        parsed["phase"]["staged_scientific"]["direct_hydrology"]["snow_stage3_shadow"],
        Value::Null
    );
    assert!(from_canonical_bytes::<DirectV10RealConsumerCheckpointV1>(&poisoned).is_err());
}

#[test]
fn v1_rejects_explicit_v11_attachment_null_as_an_extra_member() {
    let bytes = fs::read(Path::new(ARTIFACTS).join("checkpoint-in-progress-vector.json")).unwrap();
    let mut value: Value = serde_json::from_slice(&bytes).unwrap();
    value["phase"]["staged_scientific"]["direct_hydrology"]
        .as_object_mut()
        .unwrap()
        .insert("snow_stage3_v11_attachment".into(), Value::Null);
    let poisoned = serde_json::to_vec(&value).unwrap();
    let parsed: Value = serde_json::from_slice(&poisoned).expect("poison remains valid JSON");
    assert_eq!(
        parsed["phase"]["staged_scientific"]["direct_hydrology"]["snow_stage3_v11_attachment"],
        Value::Null
    );
    assert!(from_canonical_bytes::<DirectV10RealConsumerCheckpointV1>(&poisoned).is_err());
}

#[test]
fn production_v1_direct_hydrology_shape_matches_frozen_authority_schema() {
    let bytes = fs::read(Path::new(ARTIFACTS).join("checkpoint-in-progress-vector.json")).unwrap();
    let checkpoint: DirectV10RealConsumerCheckpointV1 = from_canonical_bytes(&bytes).unwrap();
    let production = serde_json::to_value(checkpoint).unwrap();
    let production_owner = &production["phase"]["staged_scientific"]["direct_hydrology"];

    let schema: Value = serde_json::from_slice(
        &fs::read(Path::new(ARTIFACTS).join("checkpoint-schema.json")).unwrap(),
    )
    .unwrap();
    let authority_owner = schema
        .pointer(
            "/properties/phase/oneOf/0/properties/staged_scientific/properties/direct_hydrology",
        )
        .unwrap();
    let mut production_members = production_owner
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    let mut authority_members = authority_owner["properties"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    production_members.sort();
    authority_members.sort();
    assert_eq!(production_members, authority_members);
    assert_eq!(authority_owner["additionalProperties"], false);
    assert_eq!(
        authority_owner["required"].as_array().unwrap().len(),
        authority_members.len()
    );
}

#[test]
fn complete_generated_v1_schema_matches_frozen_authority_schema() {
    let output =
        std::env::temp_dir().join(format!("openwepp-restart-v1-schema-{}", std::process::id()));
    if output.exists() {
        fs::remove_dir_all(&output).unwrap();
    }
    fs::create_dir(&output).unwrap();
    let status = Command::new("cargo")
        .args([
            "run",
            "--offline",
            "--quiet",
            "--manifest-path",
            "docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/tools/restart-authority-reference/Cargo.toml",
            "--bin",
            "generate_authority_artifacts",
            "--",
        ])
        .arg(&output)
        .status()
        .expect("execute complete V1 schema generator");
    assert!(status.success());
    assert_eq!(
        fs::read(output.join("checkpoint-schema.json")).unwrap(),
        fs::read(Path::new(ARTIFACTS).join("checkpoint-schema.json")).unwrap()
    );
    fs::remove_dir_all(output).unwrap();
}
