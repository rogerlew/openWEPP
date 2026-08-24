use serde_json::Value;
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
    let marker = b"\"surface_liquid_owned_state\":";
    let start = bytes
        .windows(marker.len())
        .position(|window| window == marker)
        .unwrap();
    let value: Value = serde_json::from_slice(&bytes).unwrap();
    let surface = serde_json::to_vec(
        &value["phase"]["staged_scientific"]["direct_hydrology"]["surface_liquid_owned_state"],
    )
    .unwrap();
    let insert = start + marker.len() + surface.len();
    let mut poisoned = bytes;
    poisoned.splice(
        insert..insert,
        b",\"snow_stage3_shadow\":null".iter().copied(),
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
