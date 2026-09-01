//! V16 exact-surface successor retained by the real consumer.
//!
//! The parallel V3 resident remains the immutable physical/high-mirror owner.
//! This resident owns only the mandatory exact companion and accepted V4
//! publication bytes; no carry is projected into process physics.

use openwepp_land_surface_energy::{Sha256Digest, seal_soil_thermal_receipt_free_owner_v2};

use crate::{LseSurfaceEnthalpyOwnerEnvelopeV1, SurfaceLiquidCompleteOwnerProjectionV4};

use super::{DirectV10RealConsumerError, DirectV10RealConsumerShadow, FrozenLitterV3Resident};

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
    accepted_publications: Vec<Vec<u8>>,
}

impl FrozenLitterV4Resident {
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
            accepted_publications: Vec::new(),
        })
    }

    #[must_use]
    pub const fn exact_surface_owner(&self) -> &LseSurfaceEnthalpyOwnerEnvelopeV1 {
        &self.exact_surface_owner
    }

    pub fn accepted_publication_supports_canonical_bytes(&self) -> &[Vec<u8>] {
        &self.accepted_publications
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
        restored.accepted_publications = accepted_projection_bytes.to_vec();
        Ok(restored)
    }

    fn accept_runtime_candidate(
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
        let projection_bytes = candidate
            .complete_owner_projection
            .canonical_bytes(physical.surface_configuration())
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        let replay = SurfaceLiquidCompleteOwnerProjectionV4::from_canonical_bytes(
            physical.surface_configuration(),
            &projection_bytes,
            physical.lse_state().0.state_sha256.as_str(),
        )
        .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        let replay_beginning = replay
            .beginning_exact_surface_owner()
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        let replay_ending = replay
            .exact_surface_owner()
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        let replay_receipt = replay
            .exact_surface_receipt()
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        candidate
            .exact_surface_receipt
            .validate(
                &self.exact_surface_owner,
                &candidate.ending_exact_surface_owner,
            )
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        if replay_beginning != self.exact_surface_owner
            || replay_ending != candidate.ending_exact_surface_owner
            || replay_receipt != candidate.exact_surface_receipt
        {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "V4 publication exact-owner replay",
            )
            .into());
        }
        self.exact_surface_owner = candidate.ending_exact_surface_owner.clone();
        self.accepted_publications.push(projection_bytes);
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
}

impl DirectV10RealConsumerShadow {
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
    ) -> Result<
        crate::land_surface_energy_shadow::v3_execution::AcceptedFrozenLitterV4RuntimeCandidate,
        DirectV10RealConsumerError,
    > {
        let prepared_soil = self
            .inner
            .soil_thermal
            .prepare_next_v2_support(support_start_ns, support_end_ns)?;
        let soil_seals = seal_soil_thermal_receipt_free_owner_v2(&prepared_soil)
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
        let accepted = crate::land_surface_energy_shadow::v3_execution::execute_frozen_litter_v4(
            &crate::land_surface_energy_shadow::v3_execution::FrozenLitterV4RuntimeInput {
                physical:
                    crate::land_surface_energy_shadow::v3_execution::FrozenLitterV3RuntimeInput {
                        transaction_id: fixed.water_protocol.transaction_id,
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
                        soil_thermal_owner: prepared_soil.beginning_owner(),
                        soil_thermal_restart: &soil_seals.restart,
                    },
                beginning_exact_surface_owner: exact.exact_surface_owner(),
            },
        )
        .map_err(|error| {
            super::DirectV9RealConsumerError::Serialization(format!(
                "frozen-litter V4 runtime: {error}"
            ))
        })?;
        let mut next_physical = self.frozen_litter_v3.as_ref().cloned().ok_or(
            super::DirectV9RealConsumerError::Unsupported(
                "missing native frozen-litter V3 physical resident",
            ),
        )?;
        let mut next_exact = self.frozen_litter_v4.as_ref().cloned().ok_or(
            super::DirectV9RealConsumerError::Unsupported(
                "missing native frozen-litter V4 exact resident",
            ),
        )?;
        next_physical.accept_runtime_candidate(&accepted.physical)?;
        next_exact.accept_runtime_candidate(&next_physical, &accepted)?;
        // This pair is one unpublished candidate owner set. No live field is
        // changed until both high-mirror and exact publication replay pass.
        self.frozen_litter_v3 = Some(next_physical);
        self.frozen_litter_v4 = Some(next_exact);
        Ok(accepted)
    }
}
