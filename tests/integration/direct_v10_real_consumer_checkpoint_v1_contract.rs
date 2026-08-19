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
