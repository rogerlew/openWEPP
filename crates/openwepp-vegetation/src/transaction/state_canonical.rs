//! `OPENWEPP_V4_STATE_CANONICAL_V1` whole-state encoding, imported unchanged by V5.

use std::collections::BTreeMap;

use openwepp_kernel_contract::{MaterialDonorClass, MaterialReceiverClass};
use sha2::{Digest, Sha256};

use super::{CoupledOwnedState, PhenologyPhase, StratumSharedState};
use crate::carbon_nitrogen::{ElementPool, MaterialTransfer, Tissue, TissuePool};
use crate::occupancy_state::OccupancyState;
use crate::v8_state::{V8CoupledOwnedState, V8OccupancyState, V8TileCanopyAirState};

enum Node {
    Object(BTreeMap<String, Node>),
    Array(Vec<Node>),
    F64(f64),
    U128(u128),
    String(String),
    Null,
}

pub(super) fn sha256(state: &CoupledOwnedState) -> String {
    format!("{:x}", Sha256::digest(bytes(state)))
}

pub(crate) fn v8_sha256(state: &V8CoupledOwnedState) -> String {
    format!("{:x}", Sha256::digest(v8_bytes(state)))
}

pub(crate) fn v8_bytes(state: &V8CoupledOwnedState) -> Vec<u8> {
    let mut output = Vec::new();
    encode(&v8_node(state), "", &mut output);
    output
}

fn v8_node(state: &V8CoupledOwnedState) -> Node {
    let mut root = BTreeMap::new();
    root.insert(
        "configuration_sha256".into(),
        Node::String(state.configuration_sha256.clone()),
    );
    root.insert(
        "last_transaction_id".into(),
        Node::U128(state.last_transaction_id),
    );
    root.insert(
        "model_definition_sha256".into(),
        Node::String(state.model_definition_sha256.clone()),
    );
    root.insert(
        "occupancies".into(),
        Node::Array(
            state
                .occupancies
                .iter()
                .map(|(id, lane)| {
                    object([
                        (
                            "identity",
                            object([
                                (
                                    "stratum_id",
                                    Node::String(id.stratum_id.as_str().to_owned()),
                                ),
                                ("tile_id", Node::String(id.tile_id.as_str().to_owned())),
                            ]),
                        ),
                        ("state", v8_occupancy(lane)),
                    ])
                })
                .collect(),
        ),
    );
    root.insert("state_sha256".into(), Node::String(String::new()));
    root.insert(
        "strata".into(),
        Node::Object(
            state
                .strata
                .iter()
                .map(|(id, shared)| (id.as_str().to_owned(), stratum(shared)))
                .collect(),
        ),
    );
    root.insert(
        "tile_canopy_air".into(),
        Node::Array(
            state
                .tile_canopy_air
                .iter()
                .map(|(id, lane)| {
                    object([
                        (
                            "identity",
                            object([("tile_id", Node::String(id.as_str().to_owned()))]),
                        ),
                        ("state", v8_tile_air(lane)),
                    ])
                })
                .collect(),
        ),
    );
    Node::Object(root)
}

fn v8_occupancy(value: &V8OccupancyState) -> Node {
    object([
        ("beta_hyd", Node::F64(value.beta_hyd)),
        (
            "canopy_liquid_kg_h2o_m2_tile_ground",
            Node::F64(value.canopy_liquid_kg_h2o_m2_tile_ground),
        ),
        (
            "dry_stem_temperature_k",
            Node::F64(value.dry_stem_temperature_k),
        ),
        (
            "last_accepted_transaction_id",
            value
                .last_accepted_transaction_id
                .map_or(Node::Null, Node::U128),
        ),
        (
            "root_node_potential_mm",
            Node::F64(value.root_node_potential_mm),
        ),
        ("shade_ci_pa", Node::F64(value.shade_ci_pa)),
        (
            "shade_leaf_potential_mm",
            Node::F64(value.shade_leaf_potential_mm),
        ),
        (
            "shade_leaf_temperature_k",
            Node::F64(value.shade_leaf_temperature_k),
        ),
        ("stem_potential_mm", Node::F64(value.stem_potential_mm)),
        ("sun_ci_pa", Node::F64(value.sun_ci_pa)),
        (
            "sun_leaf_potential_mm",
            Node::F64(value.sun_leaf_potential_mm),
        ),
        (
            "sun_leaf_temperature_k",
            Node::F64(value.sun_leaf_temperature_k),
        ),
        (
            "wet_surface_temperature_k",
            Node::F64(value.wet_surface_temperature_k),
        ),
    ])
}

fn v8_tile_air(value: &V8TileCanopyAirState) -> Node {
    object([
        (
            "canopy_air_specific_humidity_kg_kg",
            Node::F64(value.canopy_air_specific_humidity_kg_kg),
        ),
        (
            "canopy_air_temperature_k",
            Node::F64(value.canopy_air_temperature_k),
        ),
    ])
}

fn bytes(state: &CoupledOwnedState) -> Vec<u8> {
    let mut output = Vec::new();
    encode(&node(state), "", &mut output);
    output
}

fn node(state: &CoupledOwnedState) -> Node {
    let mut root = BTreeMap::new();
    root.insert(
        "configuration_sha256".into(),
        Node::String(state.configuration_sha256.clone()),
    );
    root.insert(
        "last_transaction_id".into(),
        Node::U128(state.last_transaction_id),
    );
    root.insert(
        "model_definition_sha256".into(),
        Node::String(state.model_definition_sha256.clone()),
    );
    root.insert(
        "occupancies".into(),
        Node::Array(
            state
                .occupancies
                .iter()
                .map(|(id, lane)| {
                    object([
                        (
                            "identity",
                            object([
                                (
                                    "stratum_id",
                                    Node::String(id.stratum_id.as_str().to_owned()),
                                ),
                                ("tile_id", Node::String(id.tile_id.as_str().to_owned())),
                            ]),
                        ),
                        ("state", occupancy(lane)),
                    ])
                })
                .collect(),
        ),
    );
    root.insert(
        "strata".into(),
        Node::Object(
            state
                .strata
                .iter()
                .map(|(id, shared)| (id.as_str().to_owned(), stratum(shared)))
                .collect(),
        ),
    );
    Node::Object(root)
}

#[cfg(test)]
fn sha256_with_released_scalar_mutation(state: &CoupledOwnedState, path: &str) -> String {
    let mut root = node(state);
    let mut cursor = &mut root;
    for segment in path.replace('[', ".").replace(']', "").split('.') {
        cursor = match cursor {
            Node::Object(values) => values
                .get_mut(segment)
                .unwrap_or_else(|| panic!("released mutation path key {segment}")),
            Node::Array(values) => {
                &mut values[segment
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("released mutation path index {segment}"))]
            }
            _ => panic!("released mutation path traversed a scalar at {segment}"),
        };
    }
    match cursor {
        Node::F64(value) => *value = f64::from_bits(value.to_bits() ^ 1),
        Node::U128(value) => *value += 1,
        Node::String(value) => value.push_str("|mutation"),
        Node::Null | Node::Object(_) | Node::Array(_) => {
            panic!("released mutation path must terminate at a scalar")
        }
    }
    let mut output = Vec::new();
    encode(&root, "", &mut output);
    format!("{:x}", Sha256::digest(output))
}

#[cfg(test)]
fn shared_state_sha256(state: &StratumSharedState) -> String {
    let mut bytes = Vec::new();
    encode(&stratum(state), "", &mut bytes);
    format!("{:x}", Sha256::digest(bytes))
}

fn occupancy(value: &OccupancyState) -> Node {
    object([
        ("beta_hyd", Node::F64(value.beta_hyd)),
        (
            "canopy_air_specific_humidity_kg_kg",
            Node::F64(value.canopy_air_specific_humidity_kg_kg),
        ),
        (
            "canopy_air_temperature_k",
            Node::F64(value.canopy_air_temperature_k),
        ),
        (
            "canopy_liquid_kg_h2o_m2_tile_ground",
            Node::F64(value.canopy_liquid_kg_h2o_m2_tile_ground),
        ),
        (
            "dry_stem_temperature_k",
            Node::F64(value.dry_stem_temperature_k),
        ),
        (
            "last_accepted_transaction_id",
            value
                .last_accepted_transaction_id
                .map_or(Node::Null, Node::U128),
        ),
        (
            "root_node_potential_mm",
            Node::F64(value.root_node_potential_mm),
        ),
        ("shade_ci_pa", Node::F64(value.shade_ci_pa)),
        (
            "shade_leaf_potential_mm",
            Node::F64(value.shade_leaf_potential_mm),
        ),
        (
            "shade_leaf_temperature_k",
            Node::F64(value.shade_leaf_temperature_k),
        ),
        ("stem_potential_mm", Node::F64(value.stem_potential_mm)),
        ("sun_ci_pa", Node::F64(value.sun_ci_pa)),
        (
            "sun_leaf_potential_mm",
            Node::F64(value.sun_leaf_potential_mm),
        ),
        (
            "sun_leaf_temperature_k",
            Node::F64(value.sun_leaf_temperature_k),
        ),
        (
            "wet_surface_temperature_k",
            Node::F64(value.wet_surface_temperature_k),
        ),
    ])
}

fn stratum(value: &StratumSharedState) -> Node {
    object([
        (
            "tissues",
            Node::Object(
                value
                    .tissues
                    .iter()
                    .map(|(id, pool)| (tissue_name(*id).into(), tissue(pool)))
                    .collect(),
            ),
        ),
        ("retranslocation_n", Node::F64(value.retranslocation_n)),
        ("nsc_c", Node::F64(value.nsc_c)),
        ("xs_c", Node::F64(value.xs_c)),
        ("standing_dead", element(&value.standing_dead)),
        ("standing_dead_dm", Node::F64(value.standing_dead_dm)),
        (
            "phase",
            Node::String(
                match value.phase {
                    PhenologyPhase::Dormant => "dormant",
                    PhenologyPhase::Onset => "onset",
                    PhenologyPhase::Active => "active",
                    PhenologyPhase::Offset => "offset",
                }
                .into(),
            ),
        ),
        ("onset_remaining_s", Node::F64(value.onset_remaining_s)),
        ("offset_remaining_s", Node::F64(value.offset_remaining_s)),
        ("previous_gsi", Node::F64(value.previous_gsi)),
        (
            "pending_transfers",
            Node::Array(value.pending_transfers.iter().map(transfer).collect()),
        ),
        ("t10_k", Node::F64(value.t10_k)),
        ("leaf_area", Node::F64(value.leaf_area)),
        ("root_area", Node::F64(value.root_area)),
        ("stem_area", Node::F64(value.stem_area)),
        ("last_transaction_id", Node::U128(value.last_transaction_id)),
    ])
}

fn tissue(value: &TissuePool) -> Node {
    object([
        ("display", element(&value.display)),
        ("storage", element(&value.storage)),
        ("transfer", element(&value.transfer)),
    ])
}

fn element(value: &ElementPool) -> Node {
    object([
        ("carbon", Node::F64(value.carbon)),
        ("nitrogen", Node::F64(value.nitrogen)),
    ])
}

fn transfer(value: &MaterialTransfer) -> Node {
    object([
        ("transaction_id", Node::U128(value.transaction_id)),
        ("owner_id", Node::String(value.owner_id.as_str().to_owned())),
        ("proposal_id", Node::U128(u128::from(value.proposal_id))),
        (
            "donor",
            Node::String(
                match value.donor {
                    MaterialDonorClass::Leaf => "leaf",
                    MaterialDonorClass::FineRoot => "fine_root",
                    MaterialDonorClass::LiveStem => "live_stem",
                    MaterialDonorClass::DeadStem => "dead_stem",
                    MaterialDonorClass::LiveCoarseRoot => "live_coarse_root",
                    MaterialDonorClass::DeadCoarseRoot => "dead_coarse_root",
                }
                .into(),
            ),
        ),
        (
            "receiver",
            Node::String(
                match value.receiver {
                    MaterialReceiverClass::Metabolic => "metabolic",
                    MaterialReceiverClass::Cellulose => "cellulose",
                    MaterialReceiverClass::Lignin => "lignin",
                    MaterialReceiverClass::CoarseWoodyDebris => "coarse_woody_debris",
                }
                .into(),
            ),
        ),
        ("carbon", Node::F64(value.carbon)),
        ("nitrogen", Node::F64(value.nitrogen)),
        ("dry_matter", Node::F64(value.dry_matter)),
    ])
}

fn tissue_name(value: Tissue) -> &'static str {
    match value {
        Tissue::Leaf => "leaf",
        Tissue::FineRoot => "fine_root",
        Tissue::LiveStem => "live_stem",
        Tissue::DeadStem => "dead_stem",
        Tissue::LiveCoarseRoot => "live_coarse_root",
        Tissue::DeadCoarseRoot => "dead_coarse_root",
    }
}

fn object<const N: usize>(entries: [(&str, Node); N]) -> Node {
    Node::Object(
        entries
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
    )
}

fn encode(value: &Node, path: &str, output: &mut Vec<u8>) {
    let mut path = path.as_bytes().to_vec();
    encode_with_path(value, &mut path, output);
}

fn encode_with_path(value: &Node, path: &mut Vec<u8>, output: &mut Vec<u8>) {
    match value {
        Node::Object(object) => {
            emit_prefix(output, path, b"object");
            append_decimal(output, object.len() as u128);
            output.push(b'\n');
            for (key, child) in object {
                let path_len = path.len();
                path.extend_from_slice(b"/k");
                append_decimal(path, key.len() as u128);
                path.push(b':');
                append_hex(path, key.as_bytes());
                encode_with_path(child, path, output);
                path.truncate(path_len);
            }
        }
        Node::Array(array) => {
            emit_prefix(output, path, b"array");
            append_decimal(output, array.len() as u128);
            output.push(b'\n');
            for (index, child) in array.iter().enumerate() {
                let path_len = path.len();
                path.extend_from_slice(b"/i");
                append_decimal(path, index as u128);
                encode_with_path(child, path, output);
                path.truncate(path_len);
            }
        }
        Node::F64(value) => {
            emit_prefix(output, path, b"f64be");
            append_u64_hex_fixed(output, value.to_bits());
            output.push(b'\n');
        }
        Node::U128(value) => {
            emit_prefix(output, path, b"u128");
            append_decimal(output, *value);
            output.push(b'\n');
        }
        Node::String(value) => {
            emit_prefix(output, path, b"string");
            append_decimal(output, value.len() as u128);
            output.push(b':');
            append_hex(output, value.as_bytes());
            output.push(b'\n');
        }
        Node::Null => {
            emit_prefix(output, path, b"null");
            output.push(b'\n');
        }
    }
}

fn emit_prefix(output: &mut Vec<u8>, path: &[u8], kind: &[u8]) {
    output.extend_from_slice(path);
    output.push(b'\t');
    output.extend_from_slice(kind);
    output.push(b'\t');
}

fn append_decimal(output: &mut Vec<u8>, mut value: u128) {
    if value == 0 {
        output.push(b'0');
        return;
    }
    let mut digits = [0_u8; 39];
    let mut cursor = digits.len();
    while value != 0 {
        cursor -= 1;
        digits[cursor] = b'0' + (value % 10) as u8;
        value /= 10;
    }
    output.extend_from_slice(&digits[cursor..]);
}

fn append_u64_hex_fixed(output: &mut Vec<u8>, value: u64) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    for shift in (0..16).rev() {
        output.push(DIGITS[((value >> (shift * 4)) & 0x0f) as usize]);
    }
}

fn append_hex(output: &mut Vec<u8>, bytes: &[u8]) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    output.reserve(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[usize::from(byte >> 4)]);
        output.push(DIGITS[usize::from(byte & 0x0f)]);
    }
}

#[cfg(test)]
fn emit(output: &mut Vec<u8>, path: &str, kind: &str, value: &str) {
    output.extend_from_slice(path.as_bytes());
    output.push(b'\t');
    output.extend_from_slice(kind.as_bytes());
    output.push(b'\t');
    output.extend_from_slice(value.as_bytes());
    output.push(b'\n');
}

#[cfg(test)]
fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_encoder_preserves_full_u128_and_float_bits() {
        let mut value = BTreeMap::new();
        value.insert("integer".into(), Node::U128(u128::MAX));
        value.insert("negative_zero".into(), Node::F64(-0.0));
        let mut bytes = Vec::new();
        encode(&Node::Object(value), "", &mut bytes);
        let text = String::from_utf8(bytes).expect("canonical UTF-8");
        assert!(text.contains("\tu128\t340282366920938463463374607431768211455\n"));
        assert!(text.contains("\tf64be\t8000000000000000\n"));
    }

    #[test]
    fn released_shared_state_vector_matches_independent_digest() {
        let fixture: serde_json::Value = serde_json::from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/work-packages/20260812-c3-woody-shared-state-authority-001/",
            "artifacts/openwepp_c3_woody_v4_vectors.json"
        )))
        .expect("released fixture");
        let expected = fixture["canonical_state_sha256"]
            .as_str()
            .expect("released digest");
        let state = fixture["shared_state"].clone();
        let mut bytes = Vec::new();
        encode_json_fixture(&state, "", &mut bytes);
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), expected);
    }

    #[test]
    fn production_encoder_matches_released_shared_state_and_mutation_digests() {
        let fixture: serde_json::Value = serde_json::from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/work-packages/20260812-c3-woody-shared-state-authority-001/",
            "artifacts/openwepp_c3_woody_v4_vectors.json"
        )))
        .expect("released fixture");
        let state: StratumSharedState = serde_json::from_value(fixture["shared_state"].clone())
            .expect("released shared-state DTO");
        assert_eq!(
            shared_state_sha256(&state),
            fixture["canonical_state_sha256"]
                .as_str()
                .expect("released base digest")
        );

        let mut changed = state.clone();
        changed.leaf_area = f64::from_bits(changed.leaf_area.to_bits() ^ 1);
        assert_eq!(
            shared_state_sha256(&changed),
            fixture["mutation_digests"]["leaf_area"]
                .as_str()
                .expect("released leaf-area mutation digest")
        );

        let mut changed = state;
        changed.pending_transfers[0].proposal_id += 1;
        assert_eq!(
            shared_state_sha256(&changed),
            fixture["mutation_digests"]["pending_transfers[0].proposal_id"]
                .as_str()
                .expect("released proposal-identity mutation digest")
        );
    }

    #[test]
    fn production_whole_state_encoder_matches_released_structural_preimage_and_digest() {
        let fixture: serde_json::Value = serde_json::from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/work-packages/20260812-c3-woody-shared-state-authority-001/",
            "artifacts/openwepp_c3_woody_v4_vectors.json"
        )))
        .expect("released fixture");
        let released = &fixture["whole_state_canonical"];
        let mut value = released["preimage"].clone();
        value["state_sha256"] = serde_json::Value::String(String::new());
        let state: CoupledOwnedState =
            serde_json::from_value(value).expect("released whole-state DTO");
        let production_bytes = bytes(&state);
        assert_eq!(
            hex(&production_bytes),
            released["preimage_utf8_hex"]
                .as_str()
                .expect("released preimage")
        );
        assert_eq!(
            sha256(&state),
            released["sha256"].as_str().expect("released digest")
        );

        let expected_mutations = fixture["whole_state_mutation_digests"]
            .as_object()
            .expect("released whole-state mutation map");
        assert_eq!(expected_mutations.len(), 155);
        for (path, expected) in expected_mutations {
            let mut structural = released["preimage"].clone();
            structural["state_sha256"] = serde_json::Value::String(String::new());
            mutate_released_json_scalar(&mut structural, path);
            match serde_json::from_value::<CoupledOwnedState>(structural) {
                Ok(changed) => assert_eq!(
                    sha256(&changed),
                    expected.as_str().expect("released mutation digest"),
                    "released typed whole-state mutation {path}"
                ),
                Err(_) if schema_invalid_enum_mutation(path) => assert_eq!(
                    sha256_with_released_scalar_mutation(&state, path),
                    expected.as_str().expect("released mutation digest"),
                    "released schema-poison whole-state mutation {path}"
                ),
                Err(error) => panic!("released typed whole-state mutation {path}: {error}"),
            }
        }

        let mut maximum_identity = state;
        maximum_identity.last_transaction_id = u128::MAX;
        for shared in maximum_identity.strata.values_mut() {
            shared.last_transaction_id = u128::MAX;
            for transfer in &mut shared.pending_transfers {
                transfer.transaction_id = u128::MAX;
            }
        }
        for lane in maximum_identity.occupancies.values_mut() {
            lane.last_accepted_transaction_id = Some(u128::MAX);
        }
        let maximum_bytes =
            String::from_utf8(bytes(&maximum_identity)).expect("canonical whole state is UTF-8");
        assert!(maximum_bytes.contains("\tu128\t340282366920938463463374607431768211455\n"));
    }

    fn mutate_released_json_scalar(value: &mut serde_json::Value, path: &str) {
        let normalized = path.replace('[', ".").replace(']', "");
        let mut cursor = value;
        for segment in normalized.split('.') {
            cursor = match cursor {
                serde_json::Value::Object(object) => object
                    .get_mut(segment)
                    .unwrap_or_else(|| panic!("released mutation path key {segment}")),
                serde_json::Value::Array(array) => {
                    &mut array[segment
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("released mutation path index {segment}"))]
                }
                _ => panic!("released mutation path traversed a scalar at {segment}"),
            };
        }
        match cursor {
            serde_json::Value::Number(number) if number.is_f64() => {
                let value = number.as_f64().expect("released f64");
                *cursor = serde_json::json!(f64::from_bits(value.to_bits() ^ 1));
            }
            serde_json::Value::Number(number) => {
                let value = number.as_u64().expect("released unsigned identity");
                *cursor = serde_json::json!(value + 1);
            }
            serde_json::Value::String(value) => value.push_str("|mutation"),
            _ => panic!("released mutation path must terminate at a scalar"),
        }
    }

    fn schema_invalid_enum_mutation(path: &str) -> bool {
        matches!(
            path.rsplit('.').next(),
            Some("phase" | "donor" | "receiver")
        )
    }

    fn encode_json_fixture(value: &serde_json::Value, path: &str, output: &mut Vec<u8>) {
        match value {
            serde_json::Value::Object(object) => {
                emit(output, path, "object", &object.len().to_string());
                let mut entries = object.iter().collect::<Vec<_>>();
                entries.sort_by(|(left, _), (right, _)| left.as_bytes().cmp(right.as_bytes()));
                for (key, child) in entries {
                    encode_json_fixture(
                        child,
                        &format!("{path}/k{}:{}", key.len(), hex(key.as_bytes())),
                        output,
                    );
                }
            }
            serde_json::Value::Array(array) => {
                emit(output, path, "array", &array.len().to_string());
                for (index, child) in array.iter().enumerate() {
                    encode_json_fixture(child, &format!("{path}/i{index}"), output);
                }
            }
            serde_json::Value::Null => emit(output, path, "null", ""),
            serde_json::Value::Bool(value) => {
                emit(output, path, "bool", if *value { "true" } else { "false" });
            }
            serde_json::Value::Number(value) => {
                if let Some(integer) = value.as_u64() {
                    emit(output, path, "u128", &integer.to_string());
                } else {
                    emit(
                        output,
                        path,
                        "f64be",
                        &format!("{:016x}", value.as_f64().expect("fixture f64").to_bits()),
                    );
                }
            }
            serde_json::Value::String(value) => emit(
                output,
                path,
                "string",
                &format!("{}:{}", value.len(), hex(value.as_bytes())),
            ),
        }
    }
}
