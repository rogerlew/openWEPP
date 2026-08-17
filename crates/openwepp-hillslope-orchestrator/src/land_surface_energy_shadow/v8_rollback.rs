//! Exact-byte rollback evidence for the heterogeneous covered V8 transaction.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_biogeochemistry::BiogeochemistryState;
use openwepp_kernel_contract::ResourceOwnerId;
use openwepp_land_surface_energy::{LandSurfaceEnergyState, SoilThermalSnapshot, WaterProtocol};
use openwepp_vegetation::V8CoupledOwnedState;
use serde::Serialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

use super::{DirectSurfaceLiquidIngressInput, LandSurfaceEnergyRealHydrologyAdapter};

const SNAPSHOT_DOMAIN: &[u8] = b"openwepp-covered-v8-rollback-v1";

/// Exact owner classes that must remain byte-identical after any failed phase.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum V8RollbackOwnerKind {
    Vegetation,
    UnifiedHydrology,
    LandSurfaceEnergy,
    SoilThermal,
    Biogeochemistry,
    PendingEnvelope,
}

/// One actual owner identity, its exact serialized bytes, and an independent hash.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct V8RollbackOwnerBytes {
    pub kind: V8RollbackOwnerKind,
    pub owner_id: ResourceOwnerId,
    pub bytes: Vec<u8>,
    pub sha256: String,
}

/// Typed actual-owner inputs captured before execution or after a failed phase.
pub struct V8RollbackInputs<'a> {
    pub vegetation_owner_id: &'a ResourceOwnerId,
    pub vegetation: &'a V8CoupledOwnedState,
    pub hydrology: &'a LandSurfaceEnergyRealHydrologyAdapter<'a>,
    pub lse: &'a LandSurfaceEnergyState,
    pub soil_thermal: &'a SoilThermalSnapshot,
    pub biogeochemistry_owner_id: &'a ResourceOwnerId,
    pub biogeochemistry: &'a BiogeochemistryState,
    pub pending_envelope_owner_id: &'a ResourceOwnerId,
    pub pending_water_protocol: &'a WaterProtocol,
    pub pending_ingress: &'a DirectSurfaceLiquidIngressInput,
    /// Exact pending diagnostic envelope bytes. The rollback layer never
    /// substitutes a declared digest for this payload.
    pub pending_diagnostic_bytes: &'a [u8],
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum V8RollbackError {
    #[error("V8 rollback serialization failed for {owner}: {detail}")]
    Serialization { owner: &'static str, detail: String },
    #[error("V8 rollback owner identities are not distinct")]
    DuplicateOwnerIdentity,
    #[error("V8 rollback owner hash aliases another owner")]
    DuplicateOwnerHash,
    #[error("V8 rollback owner set changed")]
    OwnerSet,
    #[error("V8 rollback bytes changed for owner {owner_id:?}")]
    OwnerMutation { owner_id: ResourceOwnerId },
    #[error("V8 rollback record hash is not derived from its exact bytes for owner {owner_id:?}")]
    RecordHash { owner_id: ResourceOwnerId },
    #[error("V8 rollback hydrology owner has no attached surface-liquid state")]
    MissingSurfaceOwner,
}

/// Exact-byte snapshot used as the beginning receipt and at every failure edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct V8RollbackSnapshot {
    owners: BTreeMap<ResourceOwnerId, V8RollbackOwnerBytes>,
}

impl V8RollbackSnapshot {
    /// Capture actual state rather than trusting any embedded state digest.
    pub fn capture(inputs: &V8RollbackInputs<'_>) -> Result<Self, V8RollbackError> {
        let vegetation = json_bytes("vegetation", inputs.vegetation)?;
        let hydrology_owner_id = inputs.hydrology.owner.hydrology_owner_id().clone();
        let surface = inputs
            .hydrology
            .owner
            .beginning_frame()
            .surface_liquid_shadow
            .as_deref()
            .ok_or(V8RollbackError::MissingSurfaceOwner)?;
        let surface_bytes = surface_owner_bytes(surface)?;
        let hydrology = framed_bytes(&[
            ("production_frame", inputs.hydrology.owner.snapshot_bytes()),
            ("surface_owner", &surface_bytes),
        ]);
        let lse = json_bytes("land surface energy", inputs.lse)?;
        let soil_thermal = json_bytes("soil thermal", inputs.soil_thermal)?;
        let biogeochemistry = json_bytes("biogeochemistry", inputs.biogeochemistry)?;
        let protocol = json_bytes("pending water protocol", inputs.pending_water_protocol)?;
        let ingress = json_bytes("pending ingress", inputs.pending_ingress)?;
        let pending = framed_bytes(&[
            ("water_protocol", &protocol),
            ("ingress", &ingress),
            ("diagnostics", inputs.pending_diagnostic_bytes),
        ]);

        Self::from_components([
            (
                V8RollbackOwnerKind::Vegetation,
                inputs.vegetation_owner_id.clone(),
                vegetation,
            ),
            (
                V8RollbackOwnerKind::UnifiedHydrology,
                hydrology_owner_id,
                hydrology,
            ),
            (
                V8RollbackOwnerKind::LandSurfaceEnergy,
                inputs.lse.owner_id.clone(),
                lse,
            ),
            (
                V8RollbackOwnerKind::SoilThermal,
                inputs.soil_thermal.owner_id.clone(),
                soil_thermal,
            ),
            (
                V8RollbackOwnerKind::Biogeochemistry,
                inputs.biogeochemistry_owner_id.clone(),
                biogeochemistry,
            ),
            (
                V8RollbackOwnerKind::PendingEnvelope,
                inputs.pending_envelope_owner_id.clone(),
                pending,
            ),
        ])
    }

    /// Recapture actual post-failure bytes and require byte identity for every owner.
    pub fn check_post_failure(&self, actual: &V8RollbackInputs<'_>) -> Result<(), V8RollbackError> {
        let after = Self::capture(actual)?;
        self.check_snapshot(&after)
    }

    /// Compare two already captured snapshots without reducing them to hashes.
    pub fn check_snapshot(&self, after: &Self) -> Result<(), V8RollbackError> {
        self.validate_hashes()?;
        after.validate_hashes()?;
        if self.owners.keys().collect::<BTreeSet<_>>()
            != after.owners.keys().collect::<BTreeSet<_>>()
        {
            return Err(V8RollbackError::OwnerSet);
        }
        for (owner_id, beginning) in &self.owners {
            let ending = after
                .owners
                .get(owner_id)
                .ok_or(V8RollbackError::OwnerSet)?;
            if beginning.kind != ending.kind || beginning.bytes != ending.bytes {
                return Err(V8RollbackError::OwnerMutation {
                    owner_id: owner_id.clone(),
                });
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn owners(&self) -> &BTreeMap<ResourceOwnerId, V8RollbackOwnerBytes> {
        &self.owners
    }

    fn from_components<const N: usize>(
        components: [(V8RollbackOwnerKind, ResourceOwnerId, Vec<u8>); N],
    ) -> Result<Self, V8RollbackError> {
        let mut owners = BTreeMap::new();
        let mut hashes = BTreeSet::new();
        for (kind, owner_id, bytes) in components {
            let sha256 = owner_sha256(kind, &owner_id, &bytes);
            if !hashes.insert(sha256.clone()) {
                return Err(V8RollbackError::DuplicateOwnerHash);
            }
            let record = V8RollbackOwnerBytes {
                kind,
                owner_id: owner_id.clone(),
                bytes,
                sha256,
            };
            if owners.insert(owner_id, record).is_some() {
                return Err(V8RollbackError::DuplicateOwnerIdentity);
            }
        }
        Ok(Self { owners })
    }

    fn validate_hashes(&self) -> Result<(), V8RollbackError> {
        for (owner_id, record) in &self.owners {
            if record.owner_id != *owner_id
                || record.sha256 != owner_sha256(record.kind, owner_id, &record.bytes)
            {
                return Err(V8RollbackError::RecordHash {
                    owner_id: owner_id.clone(),
                });
            }
        }
        Ok(())
    }
}

fn json_bytes<T: Serialize>(owner: &'static str, value: &T) -> Result<Vec<u8>, V8RollbackError> {
    serde_json::to_vec(value).map_err(|error| V8RollbackError::Serialization {
        owner,
        detail: error.to_string(),
    })
}

#[derive(Serialize)]
struct RawSurfaceOwner<'a> {
    owner_id: &'a ResourceOwnerId,
    configuration_sha256: &'a str,
    state_sha256: &'a str,
    records: Vec<RawSurfaceRecord<'a>>,
    continuations: Vec<RawSurfaceContinuation<'a>>,
}

#[derive(Serialize)]
struct RawSurfaceRecord<'a> {
    key: &'a crate::DirectSurfaceLiquidStoreKey,
    liquid_bits: String,
    last_accepted_transaction_id: Option<openwepp_kernel_contract::TransactionId>,
}

#[derive(Serialize)]
struct RawSurfaceContinuation<'a> {
    ofe_id: &'a openwepp_land_surface_energy::OfeId,
    day_index: usize,
    next_interval_index: u8,
    cumulative_supply_bits: String,
    cumulative_infiltration_bits: String,
    last_accepted_transaction_id: Option<openwepp_kernel_contract::TransactionId>,
}

fn surface_owner_bytes(
    state: &crate::DirectSurfaceLiquidOwnedState,
) -> Result<Vec<u8>, V8RollbackError> {
    let records = state
        .records
        .iter()
        .map(|record| RawSurfaceRecord {
            key: &record.key,
            liquid_bits: format!("{:016x}", record.liquid_kg_m2_tile.to_bits()),
            last_accepted_transaction_id: record.last_accepted_transaction_id,
        })
        .collect();
    let continuations = state
        .continuations
        .iter()
        .map(|value| RawSurfaceContinuation {
            ofe_id: &value.ofe_id,
            day_index: value.day_index,
            next_interval_index: value.next_interval_index,
            cumulative_supply_bits: format!("{:016x}", value.cumulative_supply_m.to_bits()),
            cumulative_infiltration_bits: format!(
                "{:016x}",
                value.cumulative_infiltration_m.to_bits()
            ),
            last_accepted_transaction_id: value.last_accepted_transaction_id,
        })
        .collect();
    json_bytes(
        "unified hydrology surface owner",
        &RawSurfaceOwner {
            owner_id: &state.owner_id,
            configuration_sha256: &state.configuration_sha256,
            state_sha256: &state.state_sha256,
            records,
            continuations,
        },
    )
}

fn framed_bytes(parts: &[(&str, &[u8])]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for (name, payload) in parts {
        bytes.extend_from_slice(&(name.len() as u64).to_be_bytes());
        bytes.extend_from_slice(name.as_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_be_bytes());
        bytes.extend_from_slice(payload);
    }
    bytes
}

fn owner_sha256(kind: V8RollbackOwnerKind, owner_id: &ResourceOwnerId, bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    for part in [
        SNAPSHOT_DOMAIN,
        owner_kind_name(kind).as_bytes(),
        owner_id.as_str().as_bytes(),
        bytes,
    ] {
        digest.update((part.len() as u64).to_be_bytes());
        digest.update(part);
    }
    format!("{:x}", digest.finalize())
}

const fn owner_kind_name(kind: V8RollbackOwnerKind) -> &'static str {
    match kind {
        V8RollbackOwnerKind::Vegetation => "vegetation",
        V8RollbackOwnerKind::UnifiedHydrology => "unified_hydrology",
        V8RollbackOwnerKind::LandSurfaceEnergy => "land_surface_energy",
        V8RollbackOwnerKind::SoilThermal => "soil_thermal",
        V8RollbackOwnerKind::Biogeochemistry => "biogeochemistry",
        V8RollbackOwnerKind::PendingEnvelope => "pending_envelope",
    }
}

#[cfg(test)]
mod tests {
    use openwepp_vegetation::V8_MODEL_SHA256;

    use super::*;

    fn owner(value: &str) -> ResourceOwnerId {
        ResourceOwnerId::try_new(value).expect("owner ID")
    }

    fn snapshot() -> V8RollbackSnapshot {
        V8RollbackSnapshot::from_components([
            (V8RollbackOwnerKind::Vegetation, owner("veg"), vec![1, 2]),
            (
                V8RollbackOwnerKind::UnifiedHydrology,
                owner("hydrology"),
                vec![3, 4],
            ),
            (
                V8RollbackOwnerKind::LandSurfaceEnergy,
                owner("lse"),
                vec![5, 6],
            ),
            (
                V8RollbackOwnerKind::SoilThermal,
                owner("thermal"),
                vec![7, 8],
            ),
            (
                V8RollbackOwnerKind::Biogeochemistry,
                owner("bgc"),
                vec![1, 2],
            ),
            (
                V8RollbackOwnerKind::PendingEnvelope,
                owner("pending"),
                vec![9, 10],
            ),
        ])
        .expect("rollback snapshot")
    }

    #[test]
    fn exact_bytes_pass_and_owner_hashes_do_not_alias() {
        let beginning = snapshot();
        let after = snapshot();
        beginning
            .check_snapshot(&after)
            .expect("byte-identical rollback");
        let hashes = beginning
            .owners()
            .values()
            .map(|record| record.sha256.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(hashes.len(), beginning.owners().len());
        assert_ne!(
            beginning.owners()[&owner("veg")].sha256,
            beginning.owners()[&owner("bgc")].sha256
        );
    }

    #[test]
    fn one_byte_owner_mutation_fails_even_with_a_copied_hash() {
        let beginning = snapshot();
        let mut after = beginning.clone();
        let record = after.owners.get_mut(&owner("veg")).expect("vegetation");
        record.bytes[0] ^= 1;
        assert_eq!(
            beginning.check_snapshot(&after),
            Err(V8RollbackError::RecordHash {
                owner_id: owner("veg")
            })
        );
    }

    #[test]
    fn rehashed_post_failure_mutation_reports_exact_owner() {
        let beginning = snapshot();
        let mut after = beginning.clone();
        let owner_id = owner("pending");
        let record = after.owners.get_mut(&owner_id).expect("pending envelope");
        record.bytes.push(11);
        record.sha256 = owner_sha256(record.kind, &owner_id, &record.bytes);
        assert_eq!(
            beginning.check_snapshot(&after),
            Err(V8RollbackError::OwnerMutation { owner_id })
        );
    }

    #[test]
    fn framing_distinguishes_boundaries_and_preserves_payload_bytes() {
        let left = framed_bytes(&[("a", b"bc"), ("d", b"e")]);
        let right = framed_bytes(&[("a", b"b"), ("c", b"de")]);
        assert_ne!(left, right);
        assert!(left.windows(2).any(|window| window == [1, 2]));
    }

    #[test]
    fn typed_owner_serialization_hashes_actual_fields_not_declared_digest_aliases() {
        let mut vegetation = V8CoupledOwnedState {
            configuration_sha256: "c".repeat(64),
            last_transaction_id: 4,
            model_definition_sha256: V8_MODEL_SHA256.into(),
            occupancies: BTreeMap::new(),
            state_sha256: "d".repeat(64),
            strata: BTreeMap::new(),
            tile_canopy_air: BTreeMap::new(),
        };
        let before_vegetation = json_bytes("vegetation", &vegetation).expect("vegetation bytes");
        vegetation.last_transaction_id = 5;
        let after_vegetation = json_bytes("vegetation", &vegetation).expect("vegetation bytes");
        assert_ne!(before_vegetation, after_vegetation);

        let mut bgc = BiogeochemistryState::default();
        let before_bgc = json_bytes("biogeochemistry", &bgc).expect("BGC bytes");
        bgc.last_transaction_id = 1;
        let after_bgc = json_bytes("biogeochemistry", &bgc).expect("BGC bytes");
        assert_ne!(before_bgc, after_bgc);
        assert_ne!(
            owner_sha256(
                V8RollbackOwnerKind::Vegetation,
                &owner("veg"),
                &after_vegetation
            ),
            owner_sha256(
                V8RollbackOwnerKind::Biogeochemistry,
                &owner("bgc"),
                &after_bgc
            )
        );
    }

    #[test]
    fn duplicate_real_owner_identity_is_rejected() {
        assert_eq!(
            V8RollbackSnapshot::from_components([
                (V8RollbackOwnerKind::Vegetation, owner("shared"), vec![1]),
                (
                    V8RollbackOwnerKind::Biogeochemistry,
                    owner("shared"),
                    vec![2]
                ),
            ]),
            Err(V8RollbackError::DuplicateOwnerIdentity)
        );
    }
}
