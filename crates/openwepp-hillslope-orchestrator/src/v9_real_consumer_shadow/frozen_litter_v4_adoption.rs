//! V16 exact-surface successor retained by the real consumer.
//!
//! The parallel V3 resident remains the immutable physical/high-mirror owner.
//! This resident owns only the mandatory exact companion and accepted V4
//! publication bytes; no carry is projected into process physics.

use std::sync::Arc;

use openwepp_land_surface_energy::{Sha256Digest, seal_soil_thermal_receipt_free_owner_v2};

use crate::land_surface_energy_shadow::LandSurfaceEnergyRealHydrologyAdapter;
use crate::vegetation_real_hydrology_shadow::RealHydrologyShadowAdapter;
use crate::{LseSurfaceEnthalpyOwnerEnvelopeV1, SurfaceLiquidCompleteOwnerProjectionV4};

use super::{DirectV10RealConsumerError, DirectV10RealConsumerShadow, FrozenLitterV3Resident};

#[cfg(test)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct CoveredNativePhysicalPathAuditV1 {
    pub represented_snow_retention_validation_count: u32,
    pub represented_snow_retention_by_map: Vec<CoveredNativeRetentionMapAuditV1>,
    pub snow_free_litter_physics_call_count: u32,
    pub snow_free_surface_physics_call_count: u32,
    pub snow_free_wb14_physics_call_count: u32,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoveredNativeRetentionMapAuditV1 {
    pub map: super::v11_covered::CanonicalCoveredPoisonTargetV1,
    pub beginning_v3_sha256: openwepp_coupled_time::Digest32,
    pub ending_v3_sha256: openwepp_coupled_time::Digest32,
    pub beginning_v4_sha256: openwepp_coupled_time::Digest32,
    pub ending_v4_sha256: openwepp_coupled_time::Digest32,
}

#[cfg(test)]
std::thread_local! {
    static COVERED_NATIVE_PHYSICAL_PATH_AUDIT: std::cell::RefCell<Option<CoveredNativePhysicalPathAuditV1>> = const { std::cell::RefCell::new(None) };
    static COVERED_REPRESENTED_SNOW_PHYSICAL_SCOPE: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(crate) struct CoveredNativePhysicalPathAuditGuardV1;

#[cfg(test)]
impl Drop for CoveredNativePhysicalPathAuditGuardV1 {
    fn drop(&mut self) {
        COVERED_NATIVE_PHYSICAL_PATH_AUDIT.with(|audit| *audit.borrow_mut() = None);
    }
}

#[cfg(test)]
pub(crate) fn begin_covered_native_physical_path_audit_v1() -> CoveredNativePhysicalPathAuditGuardV1
{
    COVERED_NATIVE_PHYSICAL_PATH_AUDIT.with(|audit| {
        *audit.borrow_mut() = Some(CoveredNativePhysicalPathAuditV1::default());
    });
    CoveredNativePhysicalPathAuditGuardV1
}

#[cfg(test)]
pub(crate) fn take_covered_native_physical_path_audit_v1() -> CoveredNativePhysicalPathAuditV1 {
    COVERED_NATIVE_PHYSICAL_PATH_AUDIT.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
fn record_represented_snow_retention_validation_v1() {
    COVERED_NATIVE_PHYSICAL_PATH_AUDIT.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit.represented_snow_retention_validation_count = audit
                .represented_snow_retention_validation_count
                .saturating_add(1);
        }
    });
}

#[cfg(test)]
#[derive(Clone, Copy)]
pub(crate) struct CoveredNativeInactiveProjectionSnapshotV1 {
    pub(crate) v3_sha256: openwepp_coupled_time::Digest32,
    pub(crate) v4_sha256: openwepp_coupled_time::Digest32,
}

#[cfg(test)]
impl CoveredNativeInactiveProjectionSnapshotV1 {
    pub(crate) const fn digests(
        self,
    ) -> (
        openwepp_coupled_time::Digest32,
        openwepp_coupled_time::Digest32,
    ) {
        (self.v3_sha256, self.v4_sha256)
    }
}

#[cfg(test)]
fn inactive_v3_projection_v1(
    shadow: &DirectV10RealConsumerShadow,
    physical: &FrozenLitterV3Resident,
) -> Result<Vec<u8>, DirectV10RealConsumerError> {
    fn push_bytes(out: &mut Vec<u8>, value: &[u8]) {
        out.extend_from_slice(&(value.len() as u64).to_be_bytes());
        out.extend_from_slice(value);
    }

    let mut out = b"OPENWEPP_INACTIVE_FROZEN_LITTER_V3_PROJECTION_V1\0".to_vec();
    push_bytes(&mut out, &shadow.canonical_v11_lse_owner_bytes()?);
    push_bytes(
        &mut out,
        &physical
            .surface_owner()
            .canonical_bytes(
                physical.surface_configuration().parent(),
                Some(physical.surface_configuration()),
            )
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?,
    );
    push_bytes(
        &mut out,
        physical.predecessor_receipt_chain_sha256().as_bytes(),
    );
    for support in physical.accepted_publication_supports_canonical_bytes()? {
        push_bytes(&mut out, &support);
    }
    if let Some(wb14) = physical.restart_wb14_parent_working_state_bytes()? {
        out.push(1);
        push_bytes(&mut out, &wb14);
    } else {
        out.push(0);
    }
    Ok(out)
}

#[cfg(test)]
pub(crate) fn capture_represented_snow_inactive_projection_v1(
    shadow: &DirectV10RealConsumerShadow,
) -> Result<Option<CoveredNativeInactiveProjectionSnapshotV1>, DirectV10RealConsumerError> {
    let (Some(v3), Some(v4)) = (&shadow.frozen_litter_v3, &shadow.frozen_litter_v4) else {
        if shadow.frozen_litter_v3.is_some() || shadow.frozen_litter_v4.is_some() {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "represented-snow inactive projection requires paired V3/V4 residents",
            )
            .into());
        }
        return Ok(None);
    };
    Ok(Some(CoveredNativeInactiveProjectionSnapshotV1 {
        v3_sha256: openwepp_coupled_time::digest_bytes(&inactive_v3_projection_v1(shadow, v3)?),
        v4_sha256: openwepp_coupled_time::digest_bytes(&v4.canonical_inactive_projection_v1()?),
    }))
}

#[cfg(test)]
pub(crate) fn record_represented_snow_map_retention_v1(
    beginning: Option<CoveredNativeInactiveProjectionSnapshotV1>,
    ending: Option<CoveredNativeInactiveProjectionSnapshotV1>,
    represented_snow_native: bool,
) -> Result<(), DirectV10RealConsumerError> {
    if !represented_snow_native {
        return Ok(());
    }
    let beginning = beginning.ok_or(super::DirectV9RealConsumerError::OwnerClosure(
        "represented-snow map retention missing beginning inactive projection",
    ))?;
    let ending = ending.ok_or(super::DirectV9RealConsumerError::OwnerClosure(
        "represented-snow map retention missing returned-ending inactive projection",
    ))?;
    if beginning.v3_sha256 != ending.v3_sha256 || beginning.v4_sha256 != ending.v4_sha256 {
        return Err(super::DirectV9RealConsumerError::OwnerClosure(
            "represented-snow map changed inactive V3/V4 projection",
        )
        .into());
    }
    record_represented_snow_retention_validation_v1();
    if let Some(map) = super::v11_covered::canonical_covered_current_map_for_test() {
        COVERED_NATIVE_PHYSICAL_PATH_AUDIT.with(|audit| {
            if let Some(audit) = audit.borrow_mut().as_mut() {
                audit
                    .represented_snow_retention_by_map
                    .push(CoveredNativeRetentionMapAuditV1 {
                        map,
                        beginning_v3_sha256: beginning.v3_sha256,
                        ending_v3_sha256: ending.v3_sha256,
                        beginning_v4_sha256: beginning.v4_sha256,
                        ending_v4_sha256: ending.v4_sha256,
                    });
            }
        });
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn record_represented_snow_endpoint_retention_v1(
    beginning: &DirectV10RealConsumerShadow,
    ending: &DirectV10RealConsumerShadow,
) -> Result<(), DirectV10RealConsumerError> {
    let beginning_physical = beginning.frozen_litter_v3.as_ref().ok_or(
        super::DirectV9RealConsumerError::OwnerClosure(
            "represented-snow retention missing beginning V3 resident",
        ),
    )?;
    let ending_physical =
        ending
            .frozen_litter_v3
            .as_ref()
            .ok_or(super::DirectV9RealConsumerError::OwnerClosure(
                "represented-snow retention missing ending V3 resident",
            ))?;
    let canonical_beginning = beginning.canonical_v11_lse_owner_bytes()?;
    let canonical_ending = ending.canonical_v11_lse_owner_bytes()?;
    let beginning_publication_history =
        beginning_physical.accepted_publication_supports_canonical_bytes()?;
    let ending_publication_history =
        ending_physical.accepted_publication_supports_canonical_bytes()?;
    let beginning_wb14_restart = beginning_physical.restart_wb14_parent_working_state_bytes()?;
    let ending_wb14_restart = ending_physical.restart_wb14_parent_working_state_bytes()?;
    let beginning_v3_projection = inactive_v3_projection_v1(beginning, beginning_physical)?;
    let ending_v3_projection = inactive_v3_projection_v1(ending, ending_physical)?;
    let beginning_v4_projection = beginning
        .frozen_litter_v4
        .as_ref()
        .ok_or(super::DirectV9RealConsumerError::OwnerClosure(
            "represented-snow retention missing beginning V4 resident",
        ))?
        .canonical_inactive_projection_v1()?;
    let ending_v4_projection = ending
        .frozen_litter_v4
        .as_ref()
        .ok_or(super::DirectV9RealConsumerError::OwnerClosure(
            "represented-snow retention missing ending V4 resident",
        ))?
        .canonical_inactive_projection_v1()?;
    if !ending_physical.has_same_validated_physical_history(beginning_physical)?
        || ending.frozen_litter_v4 != beginning.frozen_litter_v4
        || canonical_ending != canonical_beginning
        || ending_publication_history != beginning_publication_history
        || ending_wb14_restart != beginning_wb14_restart
        || ending_v3_projection != beginning_v3_projection
        || ending_v4_projection != beginning_v4_projection
    {
        return Err(super::DirectV9RealConsumerError::OwnerClosure(
            "represented-snow ending resident revision changed",
        )
        .into());
    }
    record_represented_snow_retention_validation_v1();
    Ok(())
}

#[cfg(test)]
fn record_native_physical_boundary_v1(select: impl FnOnce(&mut CoveredNativePhysicalPathAuditV1)) {
    let active = COVERED_REPRESENTED_SNOW_PHYSICAL_SCOPE.with(|scope| scope.get() > 0);
    if !active {
        return;
    }
    COVERED_NATIVE_PHYSICAL_PATH_AUDIT.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            select(audit);
        }
    });
}

#[cfg(test)]
pub(crate) fn record_native_litter_physics_entry_v1() {
    record_native_physical_boundary_v1(|audit| {
        audit.snow_free_litter_physics_call_count =
            audit.snow_free_litter_physics_call_count.saturating_add(1);
    });
}

#[cfg(test)]
pub(crate) fn record_native_surface_resource_entry_v1() {
    record_native_physical_boundary_v1(|audit| {
        audit.snow_free_surface_physics_call_count =
            audit.snow_free_surface_physics_call_count.saturating_add(1);
    });
}

#[cfg(test)]
pub(crate) fn record_native_surface_ingress_entry_v1() {
    record_native_physical_boundary_v1(|audit| {
        audit.snow_free_surface_physics_call_count =
            audit.snow_free_surface_physics_call_count.saturating_add(1);
    });
}

#[cfg(test)]
pub(crate) fn record_native_wb14_physics_entry_v1() {
    record_native_physical_boundary_v1(|audit| {
        audit.snow_free_wb14_physics_call_count =
            audit.snow_free_wb14_physics_call_count.saturating_add(1);
    });
}

#[cfg(test)]
struct CoveredRepresentedSnowPhysicalScopeV1;

#[cfg(test)]
impl CoveredRepresentedSnowPhysicalScopeV1 {
    fn enter() -> Self {
        COVERED_REPRESENTED_SNOW_PHYSICAL_SCOPE.with(|scope| {
            scope.set(scope.get().saturating_add(1));
        });
        Self
    }
}

#[cfg(test)]
impl Drop for CoveredRepresentedSnowPhysicalScopeV1 {
    fn drop(&mut self) {
        COVERED_REPRESENTED_SNOW_PHYSICAL_SCOPE.with(|scope| {
            scope.set(scope.get().saturating_sub(1));
        });
    }
}

fn augment_v3_lse_owner_bytes(
    v3_state_bytes: &[u8],
    exact_owner_bytes: &[u8],
) -> Result<Vec<u8>, DirectV10RealConsumerError> {
    let mut state: serde_json::Value = serde_json::from_slice(v3_state_bytes)
        .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
    let object = state
        .as_object_mut()
        .ok_or(super::DirectV9RealConsumerError::OwnerClosure(
            "V4 complete LSE owner requires object-shaped V3 state",
        ))?;
    const EXACT_KEY: &str = "exact_surface_enthalpy_owner_v1";
    if object.contains_key(EXACT_KEY) {
        return Err(super::DirectV9RealConsumerError::OwnerClosure(
            "V4 complete LSE owner exact-state key collision",
        )
        .into());
    }
    let exact: serde_json::Value = serde_json::from_slice(exact_owner_bytes)
        .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
    object.insert(EXACT_KEY.to_owned(), exact);
    serde_json::to_vec(&state)
        .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()).into())
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrozenLitterV4Resident {
    exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    publication_history_beginning_lse_v3_state_sha256: Sha256Digest,
    accepted_publications: Arc<Vec<Vec<u8>>>,
}

impl FrozenLitterV4Resident {
    #[cfg(test)]
    pub(super) fn canonical_inactive_projection_v1(
        &self,
    ) -> Result<Vec<u8>, DirectV10RealConsumerError> {
        fn push_bytes(out: &mut Vec<u8>, value: &[u8]) {
            out.extend_from_slice(&(value.len() as u64).to_be_bytes());
            out.extend_from_slice(value);
        }
        let mut out = b"OPENWEPP_INACTIVE_FROZEN_LITTER_V4_PROJECTION_V1\0".to_vec();
        push_bytes(
            &mut out,
            &self
                .exact_surface_owner
                .canonical_bytes()
                .map_err(|error| {
                    super::DirectV9RealConsumerError::Serialization(error.to_string())
                })?,
        );
        push_bytes(
            &mut out,
            self.publication_history_beginning_lse_v3_state_sha256
                .as_str()
                .as_bytes(),
        );
        for publication in self.accepted_publications.iter() {
            push_bytes(&mut out, publication);
        }
        Ok(out)
    }

    pub fn try_new(
        physical: &FrozenLitterV3Resident,
        exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    ) -> Result<Self, DirectV10RealConsumerError> {
        exact_surface_owner
            .validate_frozen_parent_join(
                physical.lse_configuration(),
                physical.lse_state(),
                physical.surface_configuration(),
                physical.surface_owner(),
            )
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        Ok(Self {
            exact_surface_owner,
            publication_history_beginning_lse_v3_state_sha256: physical
                .lse_state()
                .0
                .state_sha256
                .clone(),
            accepted_publications: Arc::new(Vec::new()),
        })
    }

    #[must_use]
    pub const fn exact_surface_owner(&self) -> &LseSurfaceEnthalpyOwnerEnvelopeV1 {
        &self.exact_surface_owner
    }

    pub fn accepted_publication_supports_canonical_bytes(&self) -> &[Vec<u8>] {
        self.accepted_publications.as_slice()
    }

    /// Canonical V11 LSE owner bytes. The V3 high-state remains unchanged,
    /// while the authoritative exact companion is nested into the same
    /// manifest owner so the seven-owner V11 topology does not change.
    pub(super) fn v11_complete_lse_owner_bytes(
        &self,
        physical: &FrozenLitterV3Resident,
    ) -> Result<Vec<u8>, DirectV10RealConsumerError> {
        self.exact_surface_owner
            .validate_frozen_parent_join(
                physical.lse_configuration(),
                physical.lse_state(),
                physical.surface_configuration(),
                physical.surface_owner(),
            )
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        let v3_state_bytes = serde_json::to_vec(physical.lse_state())
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        let exact_owner_bytes = self
            .exact_surface_owner
            .canonical_bytes()
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        augment_v3_lse_owner_bytes(&v3_state_bytes, &exact_owner_bytes)
    }

    pub fn try_restore(
        physical: &FrozenLitterV3Resident,
        exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
        accepted_projection_bytes: &[Vec<u8>],
        publication_history_beginning_lse_v3_state_sha256: &Sha256Digest,
    ) -> Result<Self, DirectV10RealConsumerError> {
        let mut restored = Self::try_new(physical, exact_surface_owner)?;
        let physical_projections = physical.accepted_complete_owner_projections()?;
        if physical_projections.len() != accepted_projection_bytes.len() {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "V4 restart physical/exact publication cardinality",
            )
            .into());
        }
        let mut predecessor = None;
        let mut expected_beginning_lse_v3_state_sha256 =
            publication_history_beginning_lse_v3_state_sha256.clone();
        for (ordinal, (bytes, physical_projection)) in accepted_projection_bytes
            .iter()
            .zip(&physical_projections)
            .enumerate()
        {
            let projection = SurfaceLiquidCompleteOwnerProjectionV4::from_canonical_bytes(
                physical.surface_configuration(),
                bytes,
                expected_beginning_lse_v3_state_sha256.as_str(),
            )
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
            let beginning = projection
                .beginning_exact_surface_owner()
                .map_err(|error| {
                    super::DirectV9RealConsumerError::Serialization(error.to_string())
                })?;
            let ending = projection.exact_surface_owner().map_err(|error| {
                super::DirectV9RealConsumerError::Serialization(error.to_string())
            })?;
            if ordinal == 0
                && beginning.receipt_chain_sha256.as_str()
                    != "0000000000000000000000000000000000000000000000000000000000000000"
            {
                return Err(super::DirectV9RealConsumerError::OwnerClosure(
                    "V4 restart publication history adoption anchor",
                )
                .into());
            }
            if projection
                .projection_v3(physical.surface_configuration())
                .map_err(|error| {
                    super::DirectV9RealConsumerError::Serialization(error.to_string())
                })?
                != *physical_projection
            {
                return Err(super::DirectV9RealConsumerError::OwnerClosure(
                    "V4 restart nested physical publication substitution",
                )
                .into());
            }
            if predecessor
                .as_ref()
                .is_some_and(|prior: &LseSurfaceEnthalpyOwnerEnvelopeV1| prior != &beginning)
            {
                return Err(super::DirectV9RealConsumerError::OwnerClosure(
                    "V4 restart publication successor chain",
                )
                .into());
            }
            expected_beginning_lse_v3_state_sha256 = ending.frozen_lse_v3_state_sha256.clone();
            predecessor = Some(ending);
        }
        if let Some(ending) = predecessor {
            if ending != restored.exact_surface_owner {
                return Err(super::DirectV9RealConsumerError::OwnerClosure(
                    "V4 restart final publication owner",
                )
                .into());
            }
        } else if restored.exact_surface_owner.receipt_chain_sha256.as_str()
            != "0000000000000000000000000000000000000000000000000000000000000000"
            || restored
                .exact_surface_owner
                .records()
                .iter()
                .any(|record| record.last_accepted_transaction_id.is_some())
        {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "V4 restart omitted accepted publication history",
            )
            .into());
        }
        restored.publication_history_beginning_lse_v3_state_sha256 =
            publication_history_beginning_lse_v3_state_sha256.clone();
        restored.accepted_publications = Arc::new(accepted_projection_bytes.to_vec());
        Ok(restored)
    }

    fn append_accepted_publication(&mut self, publication: Vec<u8>) {
        Arc::make_mut(&mut self.accepted_publications).push(publication);
    }

    #[cfg(test)]
    pub(super) fn publication_history_shares_allocation_with_for_test(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.accepted_publications, &other.accepted_publications)
    }

    #[cfg(test)]
    pub(super) fn force_deep_clone_publication_history_for_test(&mut self) {
        self.accepted_publications = Arc::new(self.accepted_publications.as_ref().clone());
    }

    pub(super) fn accept_runtime_candidate(
        &mut self,
        physical: &FrozenLitterV3Resident,
        candidate: &crate::land_surface_energy_shadow::v3_execution::AcceptedFrozenLitterV4RuntimeCandidate,
    ) -> Result<(), DirectV10RealConsumerError> {
        candidate
            .ending_exact_surface_owner
            .validate_frozen_parent_join(
                physical.lse_configuration(),
                &candidate.physical.ending_lse_state,
                physical.surface_configuration(),
                &candidate.physical.ending_surface_owner,
            )
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        candidate
            .exact_surface_receipt
            .validate(
                &self.exact_surface_owner,
                &candidate.ending_exact_surface_owner,
            )
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        self.exact_surface_owner = candidate.ending_exact_surface_owner.clone();
        self.append_accepted_publication(
            candidate.complete_owner_projection_canonical_bytes.clone(),
        );
        Ok(())
    }
}

#[cfg(test)]
mod complete_lse_owner_bytes_tests {
    use std::collections::BTreeMap;

    use openwepp_coupled_time::complete_owner_set_digest;
    use openwepp_vegetation::V11OwnerEnvelope;

    use super::augment_v3_lse_owner_bytes;

    fn complete_digest(lse_bytes: Vec<u8>) -> openwepp_coupled_time::Digest32 {
        let lse = V11OwnerEnvelope::try_new("land_surface_energy".to_owned(), lse_bytes)
            .expect("LSE owner");
        let vegetation = V11OwnerEnvelope::try_new("vegetation".to_owned(), b"fixed".to_vec())
            .expect("vegetation owner");
        let owners = BTreeMap::from([
            ("land_surface_energy".to_owned(), lse),
            ("vegetation".to_owned(), vegetation),
        ]);
        complete_owner_set_digest(
            &owners
                .values()
                .map(V11OwnerEnvelope::to_owner_state)
                .collect::<Result<Vec<_>, _>>()
                .expect("owner states"),
        )
        .expect("complete owner digest")
    }

    #[test]
    fn changed_exact_carry_changes_complete_owner_identity() {
        let first = augment_v3_lse_owner_bytes(
            br#"{"configuration_sha256":"c","state_sha256":"s"}"#,
            br#"{"carry":"1"}"#,
        )
        .expect("first augmented owner");
        let second = augment_v3_lse_owner_bytes(
            br#"{"configuration_sha256":"c","state_sha256":"s"}"#,
            br#"{"carry":"2"}"#,
        )
        .expect("second augmented owner");
        assert_ne!(complete_digest(first), complete_digest(second));
    }

    #[test]
    fn exact_owner_key_collision_and_malformed_nested_owner_fail_closed() {
        assert!(
            augment_v3_lse_owner_bytes(br#"{"exact_surface_enthalpy_owner_v1":{}}"#, br"{}",)
                .is_err(),
        );
        assert!(augment_v3_lse_owner_bytes(br"{}", b"not-json").is_err());
    }

    #[test]
    fn covered_native_map_retains_one_physical_exact_pair_without_final_replay() {
        let source = include_str!("frozen_litter_v4_adoption.rs");
        let body = source
            .rsplit_once("pub(crate) fn evaluate_covered_frozen_litter_v4_candidate_v1")
            .map(|(_, body)| body)
            .expect("covered native V4 evaluator")
            .split("pub fn install_frozen_litter_v4_resident")
            .next()
            .expect("covered native V4 evaluator body");
        assert_eq!(
            body.matches("prepare_covered_frozen_litter_v3_fixed_final")
                .count(),
            1
        );
        assert_eq!(
            body.matches("execute_and_accept_frozen_litter_v4").count(),
            1
        );
        assert_eq!(
            body.matches("construct_frozen_litter_v3_complete_envelope")
                .count(),
            1
        );
        assert!(!body.contains("install_frozen_litter_v4_resident"));

        let acceptance = source
            .rsplit_once("pub(super) fn execute_and_accept_frozen_litter_v4")
            .map(|(_, body)| body)
            .expect("native V4 acceptance")
            .split("\n}\n")
            .next()
            .expect("native V4 acceptance body");
        assert!(acceptance.contains("next_exact.accept_runtime_candidate(&beginning_physical"));
        assert!(!acceptance.contains("next_exact.accept_runtime_candidate(&next_physical"));
    }
}

impl DirectV10RealConsumerShadow {
    pub(crate) fn validate_frozen_litter_v4_resident_pair_v1(
        &self,
    ) -> Result<(), DirectV10RealConsumerError> {
        match (&self.frozen_litter_v3, &self.frozen_litter_v4) {
            (None, None) => Ok(()),
            (Some(physical), Some(exact)) => exact
                .exact_surface_owner
                .validate_frozen_parent_join(
                    physical.lse_configuration(),
                    physical.lse_state(),
                    physical.surface_configuration(),
                    physical.surface_owner(),
                )
                .map_err(|error| {
                    super::DirectV9RealConsumerError::Serialization(error.to_string()).into()
                }),
            _ => Err(super::DirectV9RealConsumerError::OwnerClosure(
                "native covered V3/V4 resident pair",
            )
            .into()),
        }
    }

    #[cfg(test)]
    pub(crate) fn poison_half_native_frozen_litter_custody_for_test(&mut self) {
        self.frozen_litter_v4 = None;
    }

    #[cfg(test)]
    pub(crate) fn poison_substitute_native_frozen_litter_custody_for_test(&mut self) {
        if let Some(exact) = self.frozen_litter_v4.as_mut() {
            exact.exact_surface_owner.frozen_lse_v3_state_sha256 =
                Sha256Digest::try_new("0".repeat(64)).expect("valid substituted digest");
        }
    }

    pub(crate) fn canonical_v11_lse_owner_bytes(
        &self,
    ) -> Result<Vec<u8>, DirectV10RealConsumerError> {
        if let Some(exact) = self.frozen_litter_v4.as_ref() {
            let physical = self.frozen_litter_v3.as_ref().ok_or(
                super::DirectV9RealConsumerError::OwnerClosure(
                    "native V4 LSE publication requires physical V3 resident",
                ),
            )?;
            exact.v11_complete_lse_owner_bytes(physical)
        } else if let Some(physical) = self.frozen_litter_v3.as_ref() {
            serde_json::to_vec(physical.lse_state()).map_err(|error| {
                super::DirectV9RealConsumerError::Serialization(error.to_string()).into()
            })
        } else {
            serde_json::to_vec(&self.inner.lse_state).map_err(|error| {
                super::DirectV9RealConsumerError::Serialization(error.to_string()).into()
            })
        }
    }

    /// Install a hydrology frame reconstructed by the exact V2 restart only
    /// when every V1-owned field is bit-identical and the sole difference is
    /// the Stage-3 attachment containing authoritative V4 custody.
    #[cfg(feature = "persisted-restart-v1")]
    pub fn restart_authority_install_exact_hydrology_frame_v2(
        &mut self,
        restored: crate::DirectRunFrame,
    ) -> Result<(), DirectV10RealConsumerError> {
        let restored_attachment = restored
            .snow_stage3_v11_attachment
            .as_deref()
            .filter(|attachment| attachment.restart_authority_contains_frozen_litter_v4())
            .ok_or(super::DirectV9RealConsumerError::OwnerClosure(
                "exact hydrology V2 restore requires Stage-3 V4 custody",
            ))?;
        let mut expected = self.inner.hydrology_frame.clone();
        expected.snow_stage3_v11_attachment = Some(Box::new(restored_attachment.clone()));
        if expected != restored {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "exact hydrology V2 restore changed V1-owned frame state",
            )
            .into());
        }
        let mut candidate = self.clone();
        candidate.inner.hydrology_frame = restored;
        candidate.restart_authority_validate_v9_complete_owner_set_exact()?;
        *self = candidate;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn evaluate_covered_frozen_litter_v4_candidate_v1(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &super::DirectV11SnowCoveredSegmentInput,
        duration_s_bits: u64,
        support_start_ns: u128,
        support_end_ns: u128,
        finalize_wb14_parent_interval: bool,
        coupled_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        complete_lower_boundaries: &std::collections::BTreeMap<
            (
                openwepp_land_surface_energy::OfeId,
                openwepp_kernel_contract::TileId,
            ),
            openwepp_land_surface_energy::Stage3SnowCoveredLowerBoundary,
        >,
        covered_destinations: &std::collections::BTreeSet<(
            openwepp_land_surface_energy::OfeId,
            openwepp_kernel_contract::TileId,
        )>,
        unpublished_soil_candidate: Option<&super::DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<
            &super::DirectSoilThermalUnpublishedContinuationResultV2,
        >,
    ) -> Result<Option<(Self, super::UncommittedCoveredV8OwnerEnvelope)>, DirectV10RealConsumerError>
    {
        if self.frozen_litter_v4.is_none() {
            return Ok(None);
        }
        if self.frozen_litter_v3.is_none() {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "native covered V4 candidate requires physical V3 resident",
            )
            .into());
        }
        let mut candidate = self.clone();
        // The complete Stage-3 lower-boundary set owns the snow surface.
        // Native V3/V4 receives the same canopy/soil forcing projection as
        // the legacy covered endpoint, with snow posture carried exclusively
        // by the typed lower boundaries below.
        let mut canopy_soil_forcing = input.lse_forcing.clone();
        canopy_soil_forcing.snow_present_at_beginning = false;
        canopy_soil_forcing.snow_present_at_end = false;
        canopy_soil_forcing.forcing_sha256 = canopy_soil_forcing.canonical_sha256()?;
        let physical_input = super::DirectV9ShadowIntervalInput {
            lse_forcing: canopy_soil_forcing,
            vegetation_forcing: input.vegetation_forcing.clone(),
            wb14_parameters: input.wb14_parameters.clone(),
        };
        let fixed = candidate.prepare_covered_frozen_litter_v3_fixed_final(
            day_index,
            interval_index,
            &physical_input,
            duration_s_bits,
            finalize_wb14_parent_interval,
            coupled_binding,
            complete_lower_boundaries,
            covered_destinations,
            unpublished_soil_candidate,
            unpublished_soil_continuation,
        )?;
        if !fixed.stage3_covered_native_tiles.is_empty() {
            if !fixed.frozen_litter_tiles.is_empty() {
                return Err(super::DirectV9RealConsumerError::OwnerClosure(
                    "mixed represented-snow and active frozen-litter V3 posture",
                )
                .into());
            }
            let represented_snow_native_column_skips_frozen_litter_v3_v4_and_wb14 = true;
            let envelope = candidate.construct_stage3_covered_native_complete_envelope(
                day_index,
                duration_s_bits,
                finalize_wb14_parent_interval,
                coupled_binding,
                &fixed,
            )?;
            let beginning_physical = self.frozen_litter_v3.as_ref().ok_or(
                super::DirectV9RealConsumerError::OwnerClosure(
                    "represented-snow missing beginning V3 resident",
                ),
            )?;
            let ending_physical = candidate.frozen_litter_v3.as_ref().ok_or(
                super::DirectV9RealConsumerError::OwnerClosure(
                    "represented-snow missing ending V3 resident",
                ),
            )?;
            // The candidate legitimately restages its WB14 parent before the
            // represented-snow branch. Compare exactly the unchanged physical
            // and retained-history revision, without serializing that history.
            let represented_snow_native_column_retains_frozen_litter_v3_v4_bytes = ending_physical
                .has_same_validated_physical_history(beginning_physical)?
                && candidate.frozen_litter_v4 == self.frozen_litter_v4;
            let represented_snow_native_column_does_not_construct_second_inner_envelope =
                candidate.inner == self.inner;
            let represented_snow_native_column_rolls_back_complete_owner_on_failure =
                represented_snow_native_column_does_not_construct_second_inner_envelope
                    && represented_snow_native_column_retains_frozen_litter_v3_v4_bytes;
            if !represented_snow_native_column_skips_frozen_litter_v3_v4_and_wb14
                || !represented_snow_native_column_does_not_construct_second_inner_envelope
                || !represented_snow_native_column_retains_frozen_litter_v3_v4_bytes
                || !represented_snow_native_column_rolls_back_complete_owner_on_failure
            {
                return Err(super::DirectV9RealConsumerError::OwnerClosure(
                    "represented-snow native inactive litter custody",
                )
                .into());
            }
            return Ok(Some((candidate, envelope)));
        }
        let accepted = candidate.execute_and_accept_frozen_litter_v4(
            &fixed,
            support_start_ns,
            support_end_ns,
            finalize_wb14_parent_interval,
            coupled_binding,
            unpublished_soil_continuation,
        )?;
        let envelope = candidate.construct_frozen_litter_v3_complete_envelope(
            day_index,
            duration_s_bits,
            &fixed,
            &accepted.physical,
            true,
        )?;
        let physical = candidate.frozen_litter_v3.as_ref().ok_or(
            super::DirectV9RealConsumerError::OwnerClosure(
                "native covered V4 ending physical resident",
            ),
        )?;
        accepted
            .ending_exact_surface_owner
            .validate_frozen_parent_join(
                physical.lse_configuration(),
                &accepted.physical.ending_lse_state,
                physical.surface_configuration(),
                &accepted.physical.ending_surface_owner,
            )
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        Ok(Some((candidate, envelope)))
    }

    /// Evaluate the represented-snow native physical prefix without creating
    /// any vegetation, biogeochemistry, joint-owner, or restart envelope.
    /// The installed V3/V4 pair is immutable identity authority only.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn evaluate_covered_frozen_litter_v4_physical_v1(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &super::DirectV11SnowCoveredSegmentInput,
        duration_s_bits: u64,
        finalize_wb14_parent_interval: bool,
        coupled_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        complete_lower_boundaries: &std::collections::BTreeMap<
            (
                openwepp_land_surface_energy::OfeId,
                openwepp_kernel_contract::TileId,
            ),
            openwepp_land_surface_energy::Stage3SnowCoveredLowerBoundary,
        >,
        covered_destinations: &std::collections::BTreeSet<(
            openwepp_land_surface_energy::OfeId,
            openwepp_kernel_contract::TileId,
        )>,
        validated_v8_projection: &super::ValidatedV9ToV8ProjectionV1<'_>,
        unpublished_soil_candidate: Option<&super::DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<
            &super::DirectSoilThermalUnpublishedContinuationResultV2,
        >,
        validated_soil_read: Option<&super::v11_covered::ValidatedCarrierSoilReadV1<'_>>,
    ) -> Result<
        Option<crate::land_surface_energy_shadow::ProvisionalCoveredV8PhysicalEvaluationV1>,
        DirectV10RealConsumerError,
    > {
        let (Some(physical_resident), Some(_exact_resident)) =
            (&self.frozen_litter_v3, &self.frozen_litter_v4)
        else {
            if self.frozen_litter_v3.is_some() || self.frozen_litter_v4.is_some() {
                return Err(super::DirectV9RealConsumerError::OwnerClosure(
                    "half-native covered physical-only posture",
                )
                .into());
            }
            return Ok(None);
        };

        let mut canopy_soil_forcing = input.lse_forcing.clone();
        canopy_soil_forcing.snow_present_at_beginning = false;
        canopy_soil_forcing.snow_present_at_end = false;
        canopy_soil_forcing.forcing_sha256 = canopy_soil_forcing.canonical_sha256()?;
        let physical_input = super::DirectV9ShadowIntervalInput {
            lse_forcing: canopy_soil_forcing,
            vegetation_forcing: input.vegetation_forcing.clone(),
            wb14_parameters: input.wb14_parameters.clone(),
        };
        let fixed = self.prepare_covered_frozen_litter_v3_fixed_final_with_projection(
            day_index,
            interval_index,
            &physical_input,
            duration_s_bits,
            finalize_wb14_parent_interval,
            coupled_binding,
            complete_lower_boundaries,
            covered_destinations,
            unpublished_soil_candidate,
            unpublished_soil_continuation,
            validated_v8_projection,
            validated_soil_read,
        )?;
        if fixed.stage3_covered_native_tiles.is_empty() || !fixed.frozen_litter_tiles.is_empty() {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "represented-snow physical-only native tile posture",
            )
            .into());
        }

        #[cfg(test)]
        let _represented_snow_physical_scope = CoveredRepresentedSnowPhysicalScopeV1::enter();

        let transaction_id = fixed.water_protocol.transaction_id;
        let hydrology = RealHydrologyShadowAdapter::try_from_day_start(
            &self.inner.hydrology_frame,
            day_index,
            transaction_id,
            f64::from_bits(duration_s_bits),
            self.inner.surface_configuration.owner_id.clone(),
            &self.inner.layer_maps,
        )
        .map_err(super::DirectV9RealConsumerError::RealHydrology)?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hydrology);
        let native_parent_handoff = physical_resident
            .wb14_parent()
            .map(|parent| parent.validated_handoff(physical_resident.surface_configuration()))
            .transpose()?;
        let active_parent = match (
            self.inner.wb14_parent_working_state.as_ref(),
            native_parent_handoff.as_ref(),
        ) {
            (Some(inner), Some(native)) => {
                if !native.has_same_liquid_arithmetic(inner) {
                    return Err(super::DirectV9RealConsumerError::OwnerClosure(
                        "represented-snow V1/V2 WB14 parent arithmetic join",
                    )
                    .into());
                }
                Some(inner)
            }
            (Some(inner), None) => Some(inner),
            (None, Some(native)) => Some(native.liquid_arithmetic()),
            (None, None) => None,
        };
        let unified = crate::land_surface_energy_shadow::
            construct_stage3_covered_native_unified_hydrology_candidate(
                &soil_adapter,
                &self.inner.surface_configuration,
                &fixed,
                physical_resident.lse_state(),
                active_parent,
                finalize_wb14_parent_interval,
                coupled_binding,
            )
            .map_err(super::DirectV9RealConsumerError::LandSurfaceShadow)?;
        let physical = crate::land_surface_energy_shadow::
            ProvisionalCoveredV8PhysicalEvaluationV1::try_new_stage3_covered_native(fixed, unified)
            .map_err(super::DirectV9RealConsumerError::OwnerEnvelope)?;
        #[cfg(test)]
        let physical = {
            let mut physical = physical;
            // This is intentionally captured only after the returned physical
            // endpoint exists. The immutable resident is the native ending
            // custody at this owning boundary; the caller independently
            // captured the beginning projection before entering the map.
            let inactive_projection = capture_represented_snow_inactive_projection_v1(self)?
                .ok_or(super::DirectV9RealConsumerError::OwnerClosure(
                    "represented-snow physical result missing ending inactive projection",
                ))?;
            let (v3_sha256, v4_sha256) = inactive_projection.digests();
            physical.bind_native_inactive_projection_for_test(v3_sha256, v4_sha256);
            physical
        };
        Ok(Some(physical))
    }

    pub fn install_frozen_litter_v4_resident(
        &mut self,
        physical: FrozenLitterV3Resident,
        exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    ) -> Result<(), DirectV10RealConsumerError> {
        let exact = FrozenLitterV4Resident::try_new(&physical, exact_surface_owner)?;
        self.install_frozen_litter_v3_resident(physical)?;
        self.frozen_litter_v4 = Some(exact);
        Ok(())
    }

    /// Install an unchanged pair from a trusted in-process charged candidate.
    /// The V3 handoff binds its complete immutable revision; the private V4
    /// value is checked against that exact current parent without replaying
    /// retained restart bytes.
    pub(crate) fn install_validated_frozen_litter_v4_residents(
        &mut self,
        physical: FrozenLitterV3Resident,
        exact: FrozenLitterV4Resident,
    ) -> Result<(), DirectV10RealConsumerError> {
        exact
            .exact_surface_owner
            .validate_frozen_parent_join(
                physical.lse_configuration(),
                physical.lse_state(),
                physical.surface_configuration(),
                physical.surface_owner(),
            )
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        if exact.accepted_publications.len() != physical.accepted_publication_count() {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "validated frozen-litter V3/V4 publication cardinality",
            )
            .into());
        }
        let physical = physical.into_validated_handoff()?.into_resident();
        self.frozen_litter_v3 = Some(physical);
        self.frozen_litter_v4 = Some(exact);
        Ok(())
    }

    /// Atomically install an independently restored physical/exact resident
    /// pair after their histories and final owner join have been validated.
    pub fn install_restored_frozen_litter_v4_residents(
        &mut self,
        physical: FrozenLitterV3Resident,
        exact: FrozenLitterV4Resident,
    ) -> Result<(), DirectV10RealConsumerError> {
        let physical_publications = physical.accepted_publication_supports_canonical_bytes()?;
        let physical_wb14 = physical.restart_wb14_parent_working_state_bytes()?;
        let predecessor_receipt_chain = physical.predecessor_receipt_chain_sha256().to_owned();
        let mut checked_physical = FrozenLitterV3Resident::try_new(
            physical.lse_configuration().clone(),
            physical.lse_state().clone(),
            physical.surface_configuration().clone(),
            physical.surface_owner().clone(),
        )?;
        checked_physical.restore_restart_authority(
            &physical_publications,
            physical_wb14.as_deref(),
            &predecessor_receipt_chain,
        )?;
        let checked_exact = FrozenLitterV4Resident::try_restore(
            &checked_physical,
            exact.exact_surface_owner.clone(),
            &exact.accepted_publications,
            &exact.publication_history_beginning_lse_v3_state_sha256,
        )?;
        self.frozen_litter_v3 = Some(checked_physical);
        self.frozen_litter_v4 = Some(checked_exact);
        Ok(())
    }

    #[must_use]
    pub const fn frozen_litter_v4_resident(&self) -> Option<&FrozenLitterV4Resident> {
        self.frozen_litter_v4.as_ref()
    }

    pub(super) fn execute_and_accept_frozen_litter_v4(
        &mut self,
        fixed: &crate::land_surface_energy_shadow::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
        support_start_ns: u128,
        support_end_ns: u128,
        finalize_wb14_parent_interval: bool,
        coupled_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        authenticated_soil_continuation: Option<
            &super::DirectSoilThermalUnpublishedContinuationResultV2,
        >,
    ) -> Result<
        crate::land_surface_energy_shadow::v3_execution::AcceptedFrozenLitterV4RuntimeCandidate,
        DirectV10RealConsumerError,
    > {
        #[cfg(test)]
        record_native_litter_physics_entry_v1();
        let execution_setup_profile =
            super::ImportedStackProfileScopeV1::begin("imported frozen execution setup");
        let fixed_unpublished_soil =
            fixed.frozen_litter_tiles.first().and_then(|tile| {
                match &tile.soil_thermal {
                crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::V2Unpublished(
                    beginning,
                ) => Some(beginning.as_ref()),
                _ => None,
            }
            });
        if let Some(beginning) = fixed_unpublished_soil
            && fixed.frozen_litter_tiles.iter().any(|tile| {
                tile.soil_thermal
                    != crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::V2Unpublished(
                        Box::new(beginning.clone()),
                    )
            })
        {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "V4 fixed unpublished soil beginning equality",
            )
            .into());
        }
        let ordinary_prepared_soil;
        let publishable_prepared_soil = if let Some(beginning) = fixed_unpublished_soil {
            let resident = self.inner.soil_thermal.v2()?;
            resident.validate()?;
            resident.validate_unpublished_physical_beginning(beginning)?;
            if authenticated_soil_continuation.is_some_and(|continuation| {
                continuation.physical_trial() != beginning.predecessor_trial()
            }) || beginning.support_start_ns() != support_start_ns
                || beginning.support_end_ns() != support_end_ns
                || beginning.authority().beginning_owner().transaction_id
                    != beginning.transaction_id()
            {
                return Err(super::DirectV9RealConsumerError::OwnerClosure(
                    "V4 fixed unpublished soil transaction/support join",
                )
                .into());
            }
            None
        } else if let Some(prepared) = authenticated_soil_continuation
            .map(super::DirectSoilThermalUnpublishedContinuationResultV2::original_prepared)
        {
            let resident = self.inner.soil_thermal.v2()?;
            resident.validate()?;
            resident.validate_prepared_beginning(prepared.beginning_owner())?;
            if prepared.beginning_owner().transaction_id != fixed.water_protocol.transaction_id
                || prepared.beginning_owner().support_start_ns != support_start_ns
                || prepared.beginning_owner().support_end_ns != support_end_ns
            {
                return Err(super::DirectV9RealConsumerError::OwnerClosure(
                    "V4 deferred prepared-soil transaction/support join",
                )
                .into());
            }
            Some(prepared)
        } else {
            ordinary_prepared_soil = self
                .inner
                .soil_thermal
                .prepare_next_v2_support(support_start_ns, support_end_ns)?;
            Some(&ordinary_prepared_soil)
        };
        let soil_seals = publishable_prepared_soil
            .map(seal_soil_thermal_receipt_free_owner_v2)
            .transpose()
            .map_err(|_| super::DirectV9RealConsumerError::OwnerClosure("V4 soil restart seal"))?;
        let physical =
            self.frozen_litter_v3
                .as_ref()
                .ok_or(super::DirectV9RealConsumerError::Unsupported(
                    "missing native frozen-litter V3 physical resident",
                ))?;
        let exact =
            self.frozen_litter_v4
                .as_ref()
                .ok_or(super::DirectV9RealConsumerError::Unsupported(
                    "missing native frozen-litter V4 exact resident",
                ))?;
        let phase_inputs = fixed
            .frozen_litter_tiles
            .iter()
            .map(|tile| tile.phase_free_input.clone())
            .collect::<Vec<_>>();
        drop(execution_setup_profile);
        let runtime_profile = super::ImportedStackProfileScopeV1::begin("imported frozen runtime");
        let accepted = crate::land_surface_energy_shadow::v3_execution::execute_frozen_litter_v4_with_heterogeneous_surface_resource(
            &crate::land_surface_energy_shadow::v3_execution::FrozenLitterV4RuntimeInput {
                physical:
                    crate::land_surface_energy_shadow::v3_execution::FrozenLitterV3RuntimeInput {
                        transaction_id: fixed.water_protocol.transaction_id,
                        soil_transaction_authority:
                            crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
                                fixed.water_protocol.transaction_id,
                                publishable_prepared_soil.map_or_else(
                                    || fixed_unpublished_soil.map(openwepp_land_surface_energy::SoilThermalUnpublishedPhysicalBeginningV2::transaction_id).unwrap_or(fixed.water_protocol.transaction_id),
                                    |prepared| prepared.beginning_owner().transaction_id,
                                ),
                            )
                            .map_err(super::DirectV9RealConsumerError::LandSurfaceShadow)?,
                        predecessor_transaction_id: physical
                            .lse_state()
                            .0
                            .last_accepted_transaction_id,
                        parent_support_start_ns: coupled_binding.parent_support_start_ns,
                        parent_support_end_ns: coupled_binding.parent_support_end_ns,
                        support_start_ns,
                        support_end_ns,
                        predecessor_receipt_chain_sha256: physical
                            .predecessor_receipt_chain_sha256()
                            .to_owned(),
                        surface_configuration: physical.surface_configuration(),
                        beginning_surface_owner: physical.surface_owner(),
                        lse_configuration: physical.lse_configuration(),
                        beginning_lse_state: physical.lse_state(),
                        phase_inputs: &phase_inputs,
                        current_ingress: &fixed.derived_current_ingress,
                        wb14_parent: physical.wb14_parent(),
                        finalize_wb14_parent_interval,
                        coupled_binding,
                        soil_beginning: match (publishable_prepared_soil, soil_seals.as_ref(), fixed_unpublished_soil) {
                            (Some(prepared), Some(seals), _) => crate::land_surface_energy_shadow::v3_execution::FrozenLitterV3SoilBeginningV1::PublishableOwner {
                                owner: prepared.beginning_owner(),
                                restart: &seals.restart,
                            },
                            (None, None, Some(beginning)) => crate::land_surface_energy_shadow::v3_execution::FrozenLitterV3SoilBeginningV1::CandidateOnlyUnpublishedSoil(beginning),
                            _ => return Err(super::DirectV9RealConsumerError::OwnerClosure(
                                "V4 soil beginning discriminator",
                            ).into()),
                        },
                    },
                beginning_exact_surface_owner: exact.exact_surface_owner(),
            },
            &fixed.water_protocol.requests,
            &fixed.water_protocol.authorizations,
            &fixed.water_protocol.finalized_uses,
        )
        .map_err(|error| {
            super::DirectV9RealConsumerError::Serialization(format!(
                "frozen-litter V4 runtime: {error}"
            ))
        })?;
        drop(runtime_profile);
        let acceptance_profile =
            super::ImportedStackProfileScopeV1::begin("imported frozen acceptance");
        if accepted
            .physical
            .complete_owner_projection
            .is_candidate_only_unpublished_soil()
        {
            return Ok(accepted);
        }
        let beginning_physical = self.frozen_litter_v3.as_ref().cloned().ok_or(
            super::DirectV9RealConsumerError::Unsupported(
                "missing native frozen-litter V3 physical resident",
            ),
        )?;
        let mut next_physical = beginning_physical.clone();
        let mut next_exact = self.frozen_litter_v4.as_ref().cloned().ok_or(
            super::DirectV9RealConsumerError::Unsupported(
                "missing native frozen-litter V4 exact resident",
            ),
        )?;
        next_physical.accept_runtime_candidate(&accepted.physical)?;
        next_exact.accept_runtime_candidate(&beginning_physical, &accepted)?;
        // This pair is one unpublished candidate owner set. No live field is
        // changed until both high-mirror and exact publication replay pass.
        self.frozen_litter_v3 = Some(next_physical);
        self.frozen_litter_v4 = Some(next_exact);
        drop(acceptance_profile);
        Ok(accepted)
    }

    pub(super) fn accept_promoted_candidate_only_frozen_litter_v4(
        &mut self,
        authoritative_beginning: &Self,
        original_prepared_owner: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        accepted: &mut crate::land_surface_energy_shadow::v3_execution::AcceptedFrozenLitterV4RuntimeCandidate,
    ) -> Result<(), DirectV10RealConsumerError> {
        if !accepted
            .physical
            .complete_owner_projection
            .is_candidate_only_unpublished_soil()
        {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "V4 deferred promotion requires candidate-only soil custody",
            )
            .into());
        }
        let beginning_physical = authoritative_beginning
            .frozen_litter_v3
            .as_ref()
            .cloned()
            .ok_or(super::DirectV9RealConsumerError::Unsupported(
                "missing beginning native frozen-litter V3 physical resident",
            ))?;
        let beginning_exact = authoritative_beginning
            .frozen_litter_v4
            .as_ref()
            .cloned()
            .ok_or(super::DirectV9RealConsumerError::Unsupported(
                "missing beginning native frozen-litter V4 exact resident",
            ))?;
        let resident = self.inner.soil_thermal.v2()?;
        let accepted_custody =
            resident
                .latest_accepted()
                .ok_or(super::DirectV9RealConsumerError::OwnerClosure(
                    "V4 deferred promotion requires one accepted soil replay",
                ))?;
        let accepted_owner = resident.owner().clone();
        let accepted_restart = accepted_custody.seals().restart.clone();
        accepted
            .promote_candidate_only_soil_after_final_replay(
                beginning_physical.surface_configuration(),
                beginning_physical.lse_state(),
                beginning_exact.exact_surface_owner(),
                original_prepared_owner,
                &accepted_owner,
                &accepted_restart,
            )
            .map_err(|error| {
                super::DirectV9RealConsumerError::Serialization(format!(
                    "frozen-litter V4 deferred projection promotion: {error}"
                ))
            })?;
        let mut next_physical = beginning_physical.clone();
        let mut next_exact = beginning_exact;
        next_physical.accept_runtime_candidate(&accepted.physical)?;
        next_exact.accept_runtime_candidate(&beginning_physical, accepted)?;
        self.frozen_litter_v3 = Some(next_physical);
        self.frozen_litter_v4 = Some(next_exact);
        Ok(())
    }
}
