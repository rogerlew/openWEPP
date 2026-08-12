//! Offline, provenance-preserving `RHESSys` mapping boundary.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RhessysSource {
    pub source_path: String,
    pub raw_bytes: String,
    pub fields: BTreeMap<String, serde_json::Value>,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct FieldProvenance {
    pub canonical_field: String,
    pub raw_field: Option<String>,
    pub source_sha256: String,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct MigrationReport {
    pub mapping_version: String,
    pub source_path: String,
    pub source_sha256: String,
    pub mapped: BTreeMap<String, serde_json::Value>,
    pub provenance: Vec<FieldProvenance>,
    pub unresolved_required_fields: Vec<String>,
    pub canonical_configuration_sha256: Option<String>,
}

#[must_use]
pub fn migrate(
    source: &RhessysSource,
    supplements: &BTreeMap<String, serde_json::Value>,
    required: &[String],
    mapping: &BTreeMap<String, String>,
) -> MigrationReport {
    let source_sha = format!("{:x}", Sha256::digest(source.raw_bytes.as_bytes()));
    let mut mapped = BTreeMap::new();
    let mut provenance = Vec::new();
    for (raw, canonical) in mapping {
        if let Some(v) = source.fields.get(raw) {
            mapped.insert(canonical.clone(), v.clone());
            provenance.push(FieldProvenance {
                canonical_field: canonical.clone(),
                raw_field: Some(raw.clone()),
                source_sha256: source_sha.clone(),
            });
        }
    }
    for (key, value) in supplements {
        mapped.insert(key.clone(), value.clone());
        provenance.push(FieldProvenance {
            canonical_field: key.clone(),
            raw_field: None,
            source_sha256: source_sha.clone(),
        });
    }
    let unresolved = required
        .iter()
        .filter(|k| !mapped.contains_key(*k))
        .cloned()
        .collect::<Vec<_>>();
    let canonical_configuration_sha256 = if unresolved.is_empty() {
        serde_json::to_vec(&mapped)
            .ok()
            .map(|b| format!("{:x}", Sha256::digest(b)))
    } else {
        None
    };
    MigrationReport {
        mapping_version: "RHESSYS_TO_OPENWEPP_C3_WOODY_V1_V1".into(),
        source_path: source.source_path.clone(),
        source_sha256: source_sha,
        mapped,
        provenance,
        unresolved_required_fields: unresolved,
        canonical_configuration_sha256,
    }
}
