//! Exact JSON record shapes for the persisted V4 vegetation state.

use serde_json::{Map, Value};

use crate::VegetationError;

const ROOT: &[&str] = &[
    "model_definition_sha256",
    "configuration_sha256",
    "state_sha256",
    "strata",
    "occupancies",
    "last_transaction_id",
];
const SHARED: &[&str] = &[
    "tissues",
    "retranslocation_n",
    "nsc_c",
    "xs_c",
    "standing_dead",
    "standing_dead_dm",
    "phase",
    "onset_remaining_s",
    "offset_remaining_s",
    "previous_gsi",
    "pending_transfers",
    "t10_k",
    "leaf_area",
    "root_area",
    "stem_area",
    "last_transaction_id",
];
const TISSUES: &[&str] = &[
    "leaf",
    "fine_root",
    "live_stem",
    "dead_stem",
    "live_coarse_root",
    "dead_coarse_root",
];
const TISSUE_POOL: &[&str] = &["display", "storage", "transfer"];
const ELEMENT_POOL: &[&str] = &["carbon", "nitrogen"];
const TRANSFER: &[&str] = &[
    "transaction_id",
    "owner_id",
    "proposal_id",
    "donor",
    "receiver",
    "carbon",
    "nitrogen",
    "dry_matter",
];
const OCCUPANCY_ENTRY: &[&str] = &["identity", "state"];
const OCCUPANCY_IDENTITY: &[&str] = &["stratum_id", "tile_id"];
const OCCUPANCY_STATE: &[&str] = &[
    "beta_hyd",
    "canopy_air_specific_humidity_kg_kg",
    "canopy_air_temperature_k",
    "canopy_liquid_kg_h2o_m2_tile_ground",
    "dry_stem_temperature_k",
    "last_accepted_transaction_id",
    "root_node_potential_mm",
    "shade_ci_pa",
    "shade_leaf_potential_mm",
    "shade_leaf_temperature_k",
    "stem_potential_mm",
    "sun_ci_pa",
    "sun_leaf_potential_mm",
    "sun_leaf_temperature_k",
    "wet_surface_temperature_k",
];

pub(super) fn validate(value: &Value) -> Result<(), VegetationError> {
    let root = exact_object(value, ROOT, "V4 state root")?;
    let strata = object(root, "strata", "V4 strata")?;
    for shared in strata.values() {
        validate_shared(shared)?;
    }
    let occupancies = root
        .get("occupancies")
        .and_then(Value::as_array)
        .ok_or_else(|| schema("V4 occupancies must be an array"))?;
    for entry in occupancies {
        let entry = exact_object(entry, OCCUPANCY_ENTRY, "V4 occupancy entry")?;
        exact_object(
            required(entry, "identity", "V4 occupancy entry")?,
            OCCUPANCY_IDENTITY,
            "V4 occupancy identity",
        )?;
        exact_object(
            required(entry, "state", "V4 occupancy entry")?,
            OCCUPANCY_STATE,
            "V4 occupancy state",
        )?;
    }
    Ok(())
}

fn validate_shared(value: &Value) -> Result<(), VegetationError> {
    let shared = exact_object(value, SHARED, "V4 shared stratum")?;
    exact_object(
        required(shared, "standing_dead", "V4 shared stratum")?,
        ELEMENT_POOL,
        "V4 standing-dead pool",
    )?;
    let tissues = object(shared, "tissues", "V4 tissue map")?;
    if tissues.len() != TISSUES.len() || TISSUES.iter().any(|key| !tissues.contains_key(*key)) {
        return Err(schema(
            "V4 tissue map has missing or unknown tissue identities",
        ));
    }
    for tissue in tissues.values() {
        let tissue = exact_object(tissue, TISSUE_POOL, "V4 tissue pool")?;
        for pool in TISSUE_POOL {
            exact_object(
                required(tissue, pool, "V4 tissue pool")?,
                ELEMENT_POOL,
                "V4 element pool",
            )?;
        }
    }
    let transfers = shared
        .get("pending_transfers")
        .and_then(Value::as_array)
        .ok_or_else(|| schema("V4 pending transfers must be an array"))?;
    for transfer in transfers {
        exact_object(transfer, TRANSFER, "V4 material transfer")?;
    }
    Ok(())
}

fn exact_object<'a>(
    value: &'a Value,
    expected: &[&str],
    label: &str,
) -> Result<&'a Map<String, Value>, VegetationError> {
    let object = value
        .as_object()
        .ok_or_else(|| schema(&format!("{label} must be an object")))?;
    if object.len() != expected.len() || expected.iter().any(|key| !object.contains_key(*key)) {
        return Err(schema(&format!("{label} has missing or unknown fields")));
    }
    Ok(object)
}

fn object<'a>(
    parent: &'a Map<String, Value>,
    key: &str,
    label: &str,
) -> Result<&'a Map<String, Value>, VegetationError> {
    required(parent, key, label)?
        .as_object()
        .ok_or_else(|| schema(&format!("{label} must be an object")))
}

fn required<'a>(
    parent: &'a Map<String, Value>,
    key: &str,
    label: &str,
) -> Result<&'a Value, VegetationError> {
    parent
        .get(key)
        .ok_or_else(|| schema(&format!("{label} is missing {key}")))
}

fn schema(message: &str) -> VegetationError {
    VegetationError::Schema(message.into())
}
