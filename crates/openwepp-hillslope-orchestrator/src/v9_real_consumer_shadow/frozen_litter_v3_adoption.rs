//! Native frozen-litter V3 resident carried by the real consumer candidate.

#[cfg(test)]
use std::cell::Cell;
use std::collections::BTreeSet;
use std::sync::Arc;

use crate::land_surface_energy_shadow::{
    LandSurfaceEnergyRealHydrologyAdapter, V8CanopyForcingReceipt,
    execute_frozen_litter_v3_fixed_final_pre_ingress,
};
use crate::vegetation_real_hydrology_shadow::RealHydrologyShadowAdapter;
use crate::{SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerEnvelopeV2};
use openwepp_coupled_time::{Digest32, digest_bytes};
use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyV3State, SurfaceConfiguration,
};

use super::frozen_litter_v3_publication_retention::FrozenLitterV3PublicationSupportV1;
use super::{DirectV10RealConsumerError, DirectV10RealConsumerShadow};

#[cfg(test)]
std::thread_local! {
    static FROZEN_LITTER_V3_FULL_HISTORY_VALIDATIONS: Cell<usize> = const { Cell::new(0) };
    static FROZEN_LITTER_V3_TAIL_VALIDATIONS: Cell<usize> = const { Cell::new(0) };
    static FROZEN_LITTER_V3_HANDOFF_VALIDATIONS: Cell<usize> = const { Cell::new(0) };
}

/// Complete native beginning owner set. It deliberately has no V1/V2 LSE or
/// surface-owner projection API.
#[derive(Clone, Debug, PartialEq)]
pub struct FrozenLitterV3Resident {
    lse_configuration: LandSurfaceEnergyConfiguration,
    lse_state: LandSurfaceEnergyV3State,
    surface_configuration: SurfaceLiquidConfigurationV2,
    surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    accepted_publications: Arc<Vec<FrozenLitterV3PublicationSupportV1>>,
    wb14_parent: Option<crate::direct_runtime::DirectWb14ParentWorkingStateV2>,
    predecessor_receipt_chain_sha256: String,
    validated_revision: ValidatedFrozenLitterV3ResidentRevisionV1,
}

/// Private, immutable and nonserializable proof for one exact resident
/// revision. It cannot be constructed from a digest or transferred to a
/// different resident.
#[derive(Clone, Debug, PartialEq)]
struct ValidatedFrozenLitterV3ResidentRevisionV1 {
    lse_configuration_sha256: String,
    lse_state_sha256: String,
    surface_configuration_sha256: String,
    surface_owner_sha256: String,
    predecessor_receipt_chain_sha256: String,
    publication_count: usize,
    publication_head_sha256: Option<Digest32>,
    publication_tail_sha256: Option<Digest32>,
    publication_chain_sha256: Digest32,
    tail_transaction_id: Option<TransactionId>,
    tail_predecessor_transaction_id: Option<TransactionId>,
    tail_support_start_ns: Option<u128>,
    tail_support_end_ns: Option<u128>,
}

pub(super) struct ValidatedFrozenLitterV3ResidentHandoffV1 {
    resident: FrozenLitterV3Resident,
}

/// Borrowed, non-wire proof that one exact immutable V9 shadow has already
/// been projected and fully validated as its imported V8 physical payload.
pub(crate) struct ValidatedV9ToV8ProjectionV1<'a> {
    source_configuration: &'a openwepp_vegetation::VegetationConfiguration,
    source_state: &'a openwepp_vegetation::V9CoupledOwnedState,
    configuration: openwepp_vegetation::VegetationConfiguration,
    state: openwepp_vegetation::V8CoupledOwnedState,
}

impl ValidatedV9ToV8ProjectionV1<'_> {
    pub(crate) const fn state(&self) -> &openwepp_vegetation::V8CoupledOwnedState {
        &self.state
    }

    pub(crate) fn into_values(
        self,
    ) -> (
        openwepp_vegetation::VegetationConfiguration,
        openwepp_vegetation::V8CoupledOwnedState,
    ) {
        (self.configuration, self.state)
    }

    fn values_for(
        &self,
        shadow: &DirectV10RealConsumerShadow,
    ) -> Result<
        (
            &openwepp_vegetation::VegetationConfiguration,
            &openwepp_vegetation::V8CoupledOwnedState,
        ),
        DirectV10RealConsumerError,
    > {
        if !std::ptr::eq(
            self.source_configuration,
            &shadow.inner.vegetation_configuration,
        ) || !std::ptr::eq(self.source_state, &shadow.inner.vegetation_state)
        {
            return Err(super::DirectV9RealConsumerError::Identity(
                "validated V9-to-V8 projection source",
            )
            .into());
        }
        Ok((&self.configuration, &self.state))
    }
}

impl ValidatedFrozenLitterV3ResidentRevisionV1 {
    fn append_publication_chain(
        chain: Digest32,
        support: &FrozenLitterV3PublicationSupportV1,
    ) -> Digest32 {
        digest_bytes(
            &[
                b"OPENWEPP_FROZEN_LITTER_V3_VALIDATED_HISTORY_V1\0".as_slice(),
                chain.as_bytes(),
                support.publication_sha256().as_bytes(),
            ]
            .concat(),
        )
    }

    fn publication_chain(supports: &[FrozenLitterV3PublicationSupportV1]) -> Digest32 {
        supports
            .iter()
            .fold(Digest32::zero(), Self::append_publication_chain)
    }

    fn from_validated_parts(
        lse_configuration: &LandSurfaceEnergyConfiguration,
        lse_state: &LandSurfaceEnergyV3State,
        surface_configuration: &SurfaceLiquidConfigurationV2,
        surface_owner: &SurfaceLiquidOwnerEnvelopeV2,
        predecessor_receipt_chain_sha256: &str,
        supports: &[FrozenLitterV3PublicationSupportV1],
        tail: Option<&crate::SurfaceLiquidCompleteOwnerProjectionIdentityV3>,
    ) -> Self {
        Self {
            lse_configuration_sha256: lse_configuration.configuration_sha256.to_string(),
            lse_state_sha256: lse_state.0.state_sha256.to_string(),
            surface_configuration_sha256: surface_configuration.configuration_sha256().into(),
            surface_owner_sha256: surface_owner.envelope_sha256().into(),
            predecessor_receipt_chain_sha256: predecessor_receipt_chain_sha256.into(),
            publication_count: supports.len(),
            publication_head_sha256: supports
                .first()
                .map(|support| *support.publication_sha256()),
            publication_tail_sha256: supports.last().map(|support| *support.publication_sha256()),
            publication_chain_sha256: Self::publication_chain(supports),
            tail_transaction_id: tail.map(|identity| identity.transaction_id),
            tail_predecessor_transaction_id: tail
                .and_then(|identity| identity.predecessor_transaction_id),
            tail_support_start_ns: tail.map(|identity| identity.support_start_ns),
            tail_support_end_ns: tail.map(|identity| identity.support_end_ns),
        }
    }

    fn validate_same_revision(
        &self,
        resident: &FrozenLitterV3Resident,
    ) -> Result<(), DirectV10RealConsumerError> {
        if self.lse_configuration_sha256 != resident.lse_configuration.configuration_sha256.as_str()
            || self.lse_state_sha256 != resident.lse_state.0.state_sha256.as_str()
            || self.surface_configuration_sha256
                != resident.surface_configuration.configuration_sha256()
            || self.surface_owner_sha256 != resident.surface_owner.envelope_sha256()
            || self.predecessor_receipt_chain_sha256 != resident.predecessor_receipt_chain_sha256
            || self.publication_count != resident.accepted_publications.len()
            || self.publication_head_sha256
                != resident
                    .accepted_publications
                    .first()
                    .map(|support| *support.publication_sha256())
            || self.publication_tail_sha256
                != resident
                    .accepted_publications
                    .last()
                    .map(|support| *support.publication_sha256())
        {
            return Err(crate::DirectSurfaceLiquidError::Identity(
                "frozen-litter V3 validated resident revision",
            )
            .into());
        }
        Ok(())
    }
}

impl FrozenLitterV3Resident {
    pub fn try_new(
        lse_configuration: LandSurfaceEnergyConfiguration,
        lse_state: LandSurfaceEnergyV3State,
        surface_configuration: SurfaceLiquidConfigurationV2,
        surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    ) -> Result<Self, DirectV10RealConsumerError> {
        lse_state.validate(&lse_configuration)?;
        surface_owner
            .canonical_bytes(surface_configuration.parent(), Some(&surface_configuration))?;
        let surface_state =
            surface_owner
                .v2_state()
                .ok_or(crate::DirectSurfaceLiquidError::Identity(
                    "frozen-litter V3 resident requires a native surface V2 owner",
                ))?;
        if surface_configuration.parent().owner_id.as_str() == lse_configuration.owner_id.as_str() {
            return Err(crate::DirectSurfaceLiquidError::Identity(
                "frozen-litter V3 surface and LSE owners alias",
            )
            .into());
        }
        let configured_litter = lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                ofe.tiles.iter().filter_map(move |tile| {
                    matches!(tile.surface, SurfaceConfiguration::ForestLitter { .. }).then(|| {
                        (
                            ofe.ofe_id.as_str().to_owned(),
                            tile.tile_id.as_str().to_owned(),
                        )
                    })
                })
            })
            .collect::<BTreeSet<_>>();
        let surface_litter = surface_configuration
            .records()
            .iter()
            .filter(|record| record.litter_depth_m.is_some())
            .map(|record| {
                (
                    record.key.ofe_id.as_str().to_owned(),
                    record.key.tile_id.as_str().to_owned(),
                )
            })
            .collect::<BTreeSet<_>>();
        if configured_litter.is_empty() || configured_litter != surface_litter {
            return Err(crate::DirectSurfaceLiquidError::Identity(
                "frozen-litter V3 LSE/surface litter topology",
            )
            .into());
        }
        for record in surface_state.records() {
            let lse_tile = lse_state
                .0
                .tiles
                .iter()
                .find(|tile| {
                    tile.ofe_id.as_str() == record.key.ofe_id.as_str()
                        && tile.tile_id.as_str() == record.key.tile_id.as_str()
                })
                .ok_or(crate::DirectSurfaceLiquidError::Identity(
                    "frozen-litter V3 LSE/surface state topology",
                ))?;
            if lse_tile.surface_enthalpy_j_m2_tile_ground.to_bits()
                != record.surface_enthalpy_j_m2_tile.to_bits()
            {
                return Err(crate::DirectSurfaceLiquidError::Identity(
                    "frozen-litter V3 LSE/surface beginning enthalpy",
                )
                .into());
            }
        }
        let predecessor_receipt_chain_sha256 = lse_state.0.state_sha256.to_string();
        let validated_revision = ValidatedFrozenLitterV3ResidentRevisionV1::from_validated_parts(
            &lse_configuration,
            &lse_state,
            &surface_configuration,
            &surface_owner,
            &predecessor_receipt_chain_sha256,
            &[],
            None,
        );
        Ok(Self {
            lse_configuration,
            lse_state,
            surface_configuration,
            surface_owner,
            accepted_publications: Arc::new(Vec::new()),
            wb14_parent: None,
            predecessor_receipt_chain_sha256,
            validated_revision,
        })
    }

    pub const fn lse_configuration(&self) -> &LandSurfaceEnergyConfiguration {
        &self.lse_configuration
    }

    pub const fn lse_state(&self) -> &LandSurfaceEnergyV3State {
        &self.lse_state
    }

    pub const fn surface_configuration(&self) -> &SurfaceLiquidConfigurationV2 {
        &self.surface_configuration
    }

    pub const fn surface_owner(&self) -> &SurfaceLiquidOwnerEnvelopeV2 {
        &self.surface_owner
    }

    pub(crate) const fn wb14_parent(
        &self,
    ) -> Option<&crate::direct_runtime::DirectWb14ParentWorkingStateV2> {
        self.wb14_parent.as_ref()
    }

    pub(crate) fn predecessor_receipt_chain_sha256(&self) -> &str {
        &self.predecessor_receipt_chain_sha256
    }

    pub fn accepted_publication_supports_canonical_bytes(
        &self,
    ) -> Result<Vec<Vec<u8>>, crate::DirectSurfaceLiquidError> {
        self.accepted_publications
            .iter()
            .map(|support| support.canonical_bytes(&self.surface_configuration))
            .collect()
    }

    pub(crate) fn accepted_complete_owner_projections(
        &self,
    ) -> Result<Vec<crate::SurfaceLiquidCompleteOwnerProjectionV3>, crate::DirectSurfaceLiquidError>
    {
        self.accepted_publications
            .iter()
            .map(|support| support.complete_owner_projection(&self.surface_configuration))
            .collect()
    }

    pub fn restart_wb14_parent_working_state_bytes(
        &self,
    ) -> Result<Option<Vec<u8>>, crate::DirectSurfaceLiquidError> {
        self.wb14_parent
            .as_ref()
            .map(|parent| parent.restart_bytes(&self.surface_configuration))
            .transpose()
    }

    fn stage_envelope_wb14_parent(
        &mut self,
        parent: Option<&crate::direct_runtime::DirectWb14ParentWorkingState>,
        finalized_surface: Option<&crate::direct_runtime::DirectSurfaceLiquidOwnedState>,
    ) -> Result<(), crate::DirectSurfaceLiquidError> {
        match (parent, self.wb14_parent.as_ref()) {
            (None, None) => Ok(()),
            (Some(legacy), Some(native)) => {
                let staged = native
                    .try_stage_validated_liquid_arithmetic(&self.surface_configuration, legacy)?;
                self.wb14_parent = Some(staged);
                Ok(())
            }
            (None, Some(native)) => {
                let finalized_surface =
                    finalized_surface.ok_or(crate::DirectSurfaceLiquidError::Identity(
                        "frozen-litter finalized WB14 parent surface owner",
                    ))?;
                self.validated_revision
                    .validate_same_revision(self)
                    .map_err(|_| {
                        crate::DirectSurfaceLiquidError::Identity(
                            "frozen-litter finalized WB14 resident revision",
                        )
                    })?;
                let finalized_owner = native.try_finalize_validated_liquid_owner(
                    &self.surface_configuration,
                    finalized_surface,
                )?;
                let mut staged = self.clone();
                staged.surface_owner = finalized_owner;
                staged.wb14_parent = None;
                staged.validated_revision.surface_owner_sha256 =
                    staged.surface_owner.envelope_sha256().into();
                staged
                    .validated_revision
                    .validate_same_revision(&staged)
                    .map_err(|_| {
                        crate::DirectSurfaceLiquidError::Identity(
                            "frozen-litter finalized WB14 staged revision",
                        )
                    })?;
                *self = staged;
                Ok(())
            }
            (Some(legacy), None) => {
                self.wb14_parent = Some(
                    crate::direct_runtime::DirectWb14ParentWorkingStateV2::try_from_validated_liquid_arithmetic(
                        &self.surface_configuration,
                        &self.surface_owner,
                        legacy,
                    )?,
                );
                Ok(())
            }
        }
    }

    pub(crate) const fn accepted_publication_count(&self) -> usize {
        self.validated_revision.publication_count
    }

    pub(super) fn has_same_validated_physical_history(
        &self,
        other: &Self,
    ) -> Result<bool, DirectV10RealConsumerError> {
        self.validated_revision.validate_same_revision(self)?;
        other.validated_revision.validate_same_revision(other)?;
        Ok(self.lse_configuration == other.lse_configuration
            && self.lse_state == other.lse_state
            && self.surface_configuration == other.surface_configuration
            && self.surface_owner == other.surface_owner
            && self.accepted_publications == other.accepted_publications
            && self.predecessor_receipt_chain_sha256 == other.predecessor_receipt_chain_sha256
            && self.validated_revision == other.validated_revision)
    }

    pub fn restore_accepted_publication_supports_canonical_bytes(
        &mut self,
        bytes: &[Vec<u8>],
    ) -> Result<(), crate::DirectSurfaceLiquidError> {
        let restored = bytes
            .iter()
            .map(|value| {
                #[cfg(test)]
                FROZEN_LITTER_V3_FULL_HISTORY_VALIDATIONS.with(|count| count.set(count.get() + 1));
                FrozenLitterV3PublicationSupportV1::from_canonical_bytes(
                    &self.surface_configuration,
                    value,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let projections = restored
            .iter()
            .map(|support| support.complete_owner_projection(&self.surface_configuration))
            .collect::<Result<Vec<_>, _>>()?;
        if projections.windows(2).any(|pair| {
            pair[1].identity().predecessor_receipt_chain_sha256
                != pair[0].identity().receipt_chain_sha256
                || pair[1].identity().beginning_surface_owner_sha256 != pair[0].envelope_sha256()
        }) || projections.last().is_some_and(|projection| {
            projection.identity().receipt_chain_sha256 != self.predecessor_receipt_chain_sha256
        }) {
            return Err(crate::DirectSurfaceLiquidError::Identity(
                "frozen-litter V3 publication restoration chain",
            ));
        }
        let validated_revision = ValidatedFrozenLitterV3ResidentRevisionV1::from_validated_parts(
            &self.lse_configuration,
            &self.lse_state,
            &self.surface_configuration,
            &self.surface_owner,
            &self.predecessor_receipt_chain_sha256,
            &restored,
            projections.last().map(|projection| projection.identity()),
        );
        self.accepted_publications = Arc::new(restored);
        self.validated_revision = validated_revision;
        Ok(())
    }

    /// Restore all resident-only restart authority after the immutable V3
    /// physical owners have been reconstructed. Validation completes before
    /// any resident field changes.
    pub fn restore_restart_authority(
        &mut self,
        accepted_publication_bytes: &[Vec<u8>],
        wb14_parent_working_state_bytes: Option<&[u8]>,
        predecessor_receipt_chain_sha256: &str,
    ) -> Result<(), crate::DirectSurfaceLiquidError> {
        if predecessor_receipt_chain_sha256.len() != 64
            || !predecessor_receipt_chain_sha256
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(crate::DirectSurfaceLiquidError::Identity(
                "frozen-litter V3 predecessor receipt-chain digest",
            ));
        }
        let restored_publications = accepted_publication_bytes
            .iter()
            .map(|bytes| {
                #[cfg(test)]
                FROZEN_LITTER_V3_FULL_HISTORY_VALIDATIONS.with(|count| count.set(count.get() + 1));
                FrozenLitterV3PublicationSupportV1::from_canonical_bytes(
                    &self.surface_configuration,
                    bytes,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let restored_projections = restored_publications
            .iter()
            .map(|support| support.complete_owner_projection(&self.surface_configuration))
            .collect::<Result<Vec<_>, _>>()?;
        if restored_projections.windows(2).any(|pair| {
            pair[1].identity().predecessor_receipt_chain_sha256
                != pair[0].identity().receipt_chain_sha256
                || pair[1].identity().beginning_surface_owner_sha256 != pair[0].envelope_sha256()
        }) {
            return Err(crate::DirectSurfaceLiquidError::Identity(
                "frozen-litter V3 publication successor chain",
            ));
        }
        if restored_publications
            .last()
            .map(|support| support.complete_owner_projection(&self.surface_configuration))
            .transpose()?
            .is_some_and(|projection| {
                projection.identity().receipt_chain_sha256 != predecessor_receipt_chain_sha256
            })
        {
            return Err(crate::DirectSurfaceLiquidError::Identity(
                "frozen-litter V3 predecessor receipt-chain publication join",
            ));
        }
        let restored_wb14 = wb14_parent_working_state_bytes
            .map(|bytes| {
                crate::direct_runtime::DirectWb14ParentWorkingStateV2::from_restart_bytes(
                    &self.surface_configuration,
                    bytes,
                )
            })
            .transpose()?;
        let validated_revision = ValidatedFrozenLitterV3ResidentRevisionV1::from_validated_parts(
            &self.lse_configuration,
            &self.lse_state,
            &self.surface_configuration,
            &self.surface_owner,
            predecessor_receipt_chain_sha256,
            &restored_publications,
            restored_projections
                .last()
                .map(|projection| projection.identity()),
        );
        self.accepted_publications = Arc::new(restored_publications);
        self.wb14_parent = restored_wb14;
        self.predecessor_receipt_chain_sha256 = predecessor_receipt_chain_sha256.to_owned();
        self.validated_revision = validated_revision;
        Ok(())
    }

    fn validate_new_publication_tail(
        &self,
        projection: &crate::SurfaceLiquidCompleteOwnerProjectionV3,
        receipts: &[openwepp_land_surface_energy::LitterPhaseReceipt],
    ) -> Result<FrozenLitterV3PublicationSupportV1, crate::DirectSurfaceLiquidError> {
        #[cfg(test)]
        FROZEN_LITTER_V3_TAIL_VALIDATIONS.with(|count| count.set(count.get() + 1));
        self.validated_revision
            .validate_same_revision(self)
            .map_err(|_| {
                crate::DirectSurfaceLiquidError::Identity(
                    "frozen-litter V3 append validation proof",
                )
            })?;
        if projection.identity().predecessor_receipt_chain_sha256
            != self.predecessor_receipt_chain_sha256
            || projection.identity().beginning_surface_owner_sha256
                != self.surface_owner.envelope_sha256()
        {
            return Err(crate::DirectSurfaceLiquidError::Identity(
                "frozen-litter V3 appended publication tail",
            ));
        }
        let support = FrozenLitterV3PublicationSupportV1::try_new(
            &self.surface_configuration,
            projection,
            receipts,
        )?;
        Ok(support)
    }

    pub(super) fn accept_runtime_candidate(
        &mut self,
        candidate: &crate::land_surface_energy_shadow::v3_execution::AcceptedFrozenLitterV3RuntimeCandidate,
    ) -> Result<(), crate::DirectSurfaceLiquidError> {
        candidate
            .ending_lse_state
            .validate(&self.lse_configuration)
            .map_err(|_| crate::DirectSurfaceLiquidError::Identity("accepted V3 LSE ending"))?;
        candidate.ending_surface_owner.canonical_bytes(
            self.surface_configuration.parent(),
            Some(&self.surface_configuration),
        )?;
        let support = self.validate_new_publication_tail(
            &candidate.complete_owner_projection,
            &candidate.litter_phase_receipts,
        )?;
        let identity = candidate.complete_owner_projection.identity();
        let validated_revision = ValidatedFrozenLitterV3ResidentRevisionV1 {
            lse_configuration_sha256: self.lse_configuration.configuration_sha256.to_string(),
            lse_state_sha256: candidate.ending_lse_state.0.state_sha256.to_string(),
            surface_configuration_sha256: self.surface_configuration.configuration_sha256().into(),
            surface_owner_sha256: candidate.ending_surface_owner.envelope_sha256().into(),
            predecessor_receipt_chain_sha256: identity.receipt_chain_sha256.clone(),
            publication_count: self.accepted_publications.len() + 1,
            publication_head_sha256: self
                .accepted_publications
                .first()
                .map(|first| *first.publication_sha256())
                .or(Some(*support.publication_sha256())),
            publication_tail_sha256: Some(*support.publication_sha256()),
            publication_chain_sha256:
                ValidatedFrozenLitterV3ResidentRevisionV1::append_publication_chain(
                    self.validated_revision.publication_chain_sha256,
                    &support,
                ),
            tail_transaction_id: Some(identity.transaction_id),
            tail_predecessor_transaction_id: identity.predecessor_transaction_id,
            tail_support_start_ns: Some(identity.support_start_ns),
            tail_support_end_ns: Some(identity.support_end_ns),
        };
        self.append_accepted_publication(support);
        self.lse_state = candidate.ending_lse_state.clone();
        self.surface_owner = candidate.ending_surface_owner.clone();
        self.wb14_parent = candidate.ingress.parent_working_state().cloned();
        self.predecessor_receipt_chain_sha256 = candidate
            .complete_owner_projection
            .identity()
            .receipt_chain_sha256
            .clone();
        self.validated_revision = validated_revision;
        Ok(())
    }

    fn append_accepted_publication(&mut self, support: FrozenLitterV3PublicationSupportV1) {
        Arc::make_mut(&mut self.accepted_publications).push(support);
    }

    #[cfg(test)]
    pub(super) fn publication_history_shares_allocation_with_for_test(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.accepted_publications, &other.accepted_publications)
    }

    #[cfg(test)]
    pub(super) fn force_deep_clone_publication_history_for_test(&mut self) {
        self.accepted_publications = Arc::new(self.accepted_publications.as_ref().clone());
    }

    pub(super) fn into_validated_handoff(
        self,
    ) -> Result<ValidatedFrozenLitterV3ResidentHandoffV1, DirectV10RealConsumerError> {
        #[cfg(test)]
        FROZEN_LITTER_V3_HANDOFF_VALIDATIONS.with(|count| count.set(count.get() + 1));
        self.validated_revision.validate_same_revision(&self)?;
        Ok(ValidatedFrozenLitterV3ResidentHandoffV1 { resident: self })
    }

    #[cfg(test)]
    pub(super) fn validated_handoff_for_test(self) -> Result<Self, DirectV10RealConsumerError> {
        self.into_validated_handoff()
            .map(ValidatedFrozenLitterV3ResidentHandoffV1::into_resident)
    }

    #[cfg(test)]
    pub(super) fn validate_new_publication_tail_for_test(
        &self,
        projection: &crate::SurfaceLiquidCompleteOwnerProjectionV3,
        receipts: &[openwepp_land_surface_energy::LitterPhaseReceipt],
    ) -> Result<FrozenLitterV3PublicationSupportV1, crate::DirectSurfaceLiquidError> {
        self.validate_new_publication_tail(projection, receipts)
    }

    #[cfg(test)]
    pub(super) fn corrupt_validated_tail_for_test(&mut self) {
        self.validated_revision.publication_count += 1;
    }
}

#[cfg(test)]
pub(super) fn reset_frozen_litter_v3_handoff_counters_for_test() {
    FROZEN_LITTER_V3_FULL_HISTORY_VALIDATIONS.with(|count| count.set(0));
    FROZEN_LITTER_V3_TAIL_VALIDATIONS.with(|count| count.set(0));
    FROZEN_LITTER_V3_HANDOFF_VALIDATIONS.with(|count| count.set(0));
}

#[cfg(test)]
pub(super) fn frozen_litter_v3_handoff_counters_for_test() -> (usize, usize, usize) {
    (
        FROZEN_LITTER_V3_FULL_HISTORY_VALIDATIONS.with(Cell::get),
        FROZEN_LITTER_V3_TAIL_VALIDATIONS.with(Cell::get),
        FROZEN_LITTER_V3_HANDOFF_VALIDATIONS.with(Cell::get),
    )
}

impl ValidatedFrozenLitterV3ResidentHandoffV1 {
    pub(super) fn into_resident(self) -> FrozenLitterV3Resident {
        self.resident
    }
}

impl DirectV10RealConsumerShadow {
    pub(crate) fn validated_v9_to_v8_projection_v1(
        &self,
    ) -> Result<ValidatedV9ToV8ProjectionV1<'_>, DirectV10RealConsumerError> {
        let (configuration, state) = super::project_v9_runtime_to_v8(
            &self.inner.vegetation_configuration,
            &self.inner.vegetation_state,
        )
        .map_err(super::DirectV9RealConsumerError::V9)?;
        Ok(ValidatedV9ToV8ProjectionV1 {
            source_configuration: &self.inner.vegetation_configuration,
            source_state: &self.inner.vegetation_state,
            configuration,
            state,
        })
    }

    pub(crate) fn stage_frozen_litter_wb14_parent_from_inner_v1(
        &mut self,
    ) -> Result<(), DirectV10RealConsumerError> {
        if self.frozen_litter_v4.is_some() && self.frozen_litter_v3.is_none() {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "native V4 WB14 staging requires physical V3 resident",
            )
            .into());
        }
        let parent = self.inner.wb14_parent_working_state.as_ref();
        let finalized_surface = self.inner.hydrology_frame.surface_liquid_shadow.as_deref();
        if let Some(physical) = self.frozen_litter_v3.as_mut() {
            // The native parent was fully validated when its unpublished
            // ingress candidate was constructed. Validate it once more at
            // this carrier handoff, then bind its exact nested arithmetic to
            // the already-validated V1 envelope parent without crossing the
            // durable restart serialization boundary.
            physical.stage_envelope_wb14_parent(parent, finalized_surface)?;
        }
        Ok(())
    }

    pub(crate) fn stage_frozen_litter_wb14_parent_after_native_inactive_prefix_v1(
        &mut self,
        day_index: usize,
        interval_index: usize,
        input: &super::DirectV9ShadowIntervalInput,
        coupled_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        prefix: crate::direct_runtime::ValidatedNativeInactiveWb14PrefixV1,
    ) -> Result<(), DirectV10RealConsumerError> {
        if self.inner.wb14_parent_working_state.is_some() || self.frozen_litter_v4.is_none() {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "native inactive prefix cannot replace an active or half-native WB14 parent",
            )
            .into());
        }
        let physical = self.frozen_litter_v3.as_mut().ok_or(
            super::DirectV9RealConsumerError::OwnerClosure(
                "native inactive prefix requires physical V3 resident",
            ),
        )?;
        if physical.wb14_parent.is_some() {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "native inactive prefix cannot replace native WB14 parent",
            )
            .into());
        }
        physical.wb14_parent = Some(
            crate::direct_runtime::DirectWb14ParentWorkingStateV2::try_begin_after_native_inactive_prefix(
                &physical.surface_configuration,
                &physical.surface_owner,
                openwepp_kernel_contract::TransactionId(
                    self.vegetation_state.0.last_transaction_id,
                ),
                day_index,
                interval_index,
                &input.wb14_parameters,
                coupled_binding,
                prefix,
            )?,
        );
        Ok(())
    }

    pub fn install_frozen_litter_v3_resident(
        &mut self,
        resident: FrozenLitterV3Resident,
    ) -> Result<(), DirectV10RealConsumerError> {
        let validated = resident.into_validated_handoff()?;
        self.frozen_litter_v3 = Some(validated.into_resident());
        Ok(())
    }

    pub const fn frozen_litter_v3_resident(&self) -> Option<&FrozenLitterV3Resident> {
        self.frozen_litter_v3.as_ref()
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(super) fn prepare_frozen_litter_v3_fixed_final(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &super::DirectV9ShadowIntervalInput,
        duration_s_bits: u64,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        unpublished_soil_candidate: Option<&super::DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<
            &super::DirectSoilThermalUnpublishedContinuationResultV2,
        >,
    ) -> Result<
        crate::land_surface_energy_shadow::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
        DirectV10RealConsumerError,
    >{
        self.prepare_frozen_litter_v3_fixed_final_common(
            day_index,
            interval_index,
            input,
            duration_s_bits,
            finalize_wb14_parent_interval,
            wb14_coupled_child_binding,
            None,
            None,
            unpublished_soil_candidate,
            unpublished_soil_continuation,
            None,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn prepare_covered_frozen_litter_v3_fixed_final(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &super::DirectV9ShadowIntervalInput,
        duration_s_bits: u64,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        complete_lower_boundaries: &std::collections::BTreeMap<
            (openwepp_land_surface_energy::OfeId, openwepp_kernel_contract::TileId),
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
    ) -> Result<
        crate::land_surface_energy_shadow::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
        DirectV10RealConsumerError,
    >{
        self.prepare_frozen_litter_v3_fixed_final_common(
            day_index,
            interval_index,
            input,
            duration_s_bits,
            finalize_wb14_parent_interval,
            wb14_coupled_child_binding,
            Some(complete_lower_boundaries),
            Some(covered_destinations),
            unpublished_soil_candidate,
            unpublished_soil_continuation,
            None,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn prepare_covered_frozen_litter_v3_fixed_final_with_projection(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &super::DirectV9ShadowIntervalInput,
        duration_s_bits: u64,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        complete_lower_boundaries: &std::collections::BTreeMap<
            (openwepp_land_surface_energy::OfeId, openwepp_kernel_contract::TileId),
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
        projection: &ValidatedV9ToV8ProjectionV1<'_>,
        validated_soil_read: Option<
            &super::v11_covered::ValidatedCarrierSoilReadV1<'_>,
        >,
    ) -> Result<
        crate::land_surface_energy_shadow::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
        DirectV10RealConsumerError,
    >{
        self.prepare_frozen_litter_v3_fixed_final_common(
            day_index,
            interval_index,
            input,
            duration_s_bits,
            finalize_wb14_parent_interval,
            wb14_coupled_child_binding,
            Some(complete_lower_boundaries),
            Some(covered_destinations),
            unpublished_soil_candidate,
            unpublished_soil_continuation,
            Some(projection),
            validated_soil_read,
        )
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn prepare_frozen_litter_v3_fixed_final_common(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &super::DirectV9ShadowIntervalInput,
        duration_s_bits: u64,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        complete_lower_boundaries: Option<&std::collections::BTreeMap<
            (openwepp_land_surface_energy::OfeId, openwepp_kernel_contract::TileId),
            openwepp_land_surface_energy::Stage3SnowCoveredLowerBoundary,
        >>,
        covered_destinations: Option<&std::collections::BTreeSet<(
            openwepp_land_surface_energy::OfeId,
            openwepp_kernel_contract::TileId,
        )>>,
        unpublished_soil_candidate: Option<&super::DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<
            &super::DirectSoilThermalUnpublishedContinuationResultV2,
        >,
        validated_projection: Option<&ValidatedV9ToV8ProjectionV1<'_>>,
        validated_soil_read: Option<
            &super::v11_covered::ValidatedCarrierSoilReadV1<'_>,
        >,
    ) -> Result<
        crate::land_surface_energy_shadow::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
        DirectV10RealConsumerError,
    >{
        let resident =
            self.frozen_litter_v3
                .as_ref()
                .ok_or(super::DirectV9RealConsumerError::Unsupported(
                    "missing native frozen-litter V3 resident",
                ))?;
        let transaction_id = openwepp_kernel_contract::TransactionId(
            self.inner
                .vegetation_state
                .0
                .last_transaction_id
                .checked_add(1)
                .ok_or(super::DirectV9RealConsumerError::Identity(
                    "V3 vegetation transaction overflow",
                ))?,
        );
        let interval_index = u8::try_from(interval_index).map_err(|_| {
            super::DirectV9RealConsumerError::Identity("V3 interval index overflow")
        })?;
        let interval_s = f64::from_bits(duration_s_bits);
        let snow_free = complete_lower_boundaries.is_none();
        if input.lse_forcing.transaction_id != transaction_id
            || input.lse_forcing.interval_s.to_bits() != duration_s_bits
            || (snow_free
                && (input.lse_forcing.snow_present_at_beginning
                    || input.lse_forcing.snow_present_at_end
                    || input.lse_forcing.snow_terminal_payload_present))
            || (complete_lower_boundaries.is_some() != covered_destinations.is_some())
            || !input.lse_forcing.runon_parcels.is_empty()
        {
            return Err(super::DirectV9RealConsumerError::Unsupported(
                "native V3 forcing identity/domain",
            )
            .into());
        }
        if snow_free {
            input.lse_forcing.validate(transaction_id)?;
        } else {
            // Covered admission is carried by `DirectV11SnowCoveredSegmentInput`
            // and the complete lower-boundary set. Reuse the common scalar,
            // parcel, and meteorology validator on an inspection-only copy;
            // the physical solver below receives the original covered forcing.
            let mut validation_forcing = input.lse_forcing.clone();
            validation_forcing.snow_present_at_beginning = false;
            validation_forcing.snow_present_at_end = false;
            validation_forcing.snow_terminal_payload_present = false;
            validation_forcing.forcing_sha256 = validation_forcing.canonical_sha256()?;
            validation_forcing.validate(transaction_id)?;
        }
        let owned_projection;
        let (v8_configuration, v8_beginning) = if let Some(projection) = validated_projection {
            projection.values_for(self)?
        } else {
            owned_projection = super::project_v9_runtime_to_v8(
                &self.inner.vegetation_configuration,
                &self.inner.vegetation_state,
            )
            .map_err(super::DirectV9RealConsumerError::V9)?;
            (&owned_projection.0, &owned_projection.1)
        };
        let mut effective_hydrology_frame = self.inner.hydrology_frame.clone();
        if let Some(parent) = &self.inner.wb14_parent_working_state {
            effective_hydrology_frame.surface_liquid_shadow =
                Some(Box::new(parent.candidate_state().clone()));
        }
        let hydrology = RealHydrologyShadowAdapter::try_from_day_start(
            &effective_hydrology_frame,
            day_index,
            transaction_id,
            interval_s,
            self.inner.surface_configuration.owner_id.clone(),
            &self.inner.layer_maps,
        )
        .map_err(super::DirectV9RealConsumerError::RealHydrology)?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hydrology);
        let unpublished_physical_beginning = match unpublished_soil_candidate {
            Some(candidate) => {
                let beginning = self
                    .inner
                    .soil_thermal
                    .prepare_unpublished_physical_beginning_v2(
                        &self.inner.lse_configuration,
                        candidate,
                        unpublished_soil_continuation,
                        wb14_coupled_child_binding.child_support_start_ns,
                        wb14_coupled_child_binding.child_support_end_ns,
                    )?;
                Some(beginning)
            }
            None if unpublished_soil_continuation.is_some() => {
                return Err(super::DirectV9RealConsumerError::OwnerClosure(
                    "native covered V3 continuation without soil candidate",
                )
                .into());
            }
            None => None,
        };
        let soil_read = match validated_soil_read {
            Some(validated) => validated.read_view_for(unpublished_soil_candidate)?,
            None => {
                let read_view = unpublished_soil_candidate.map_or_else(
                    || self.inner.soil_thermal.read_view(),
                    super::DirectSoilThermalCandidate::read_view,
                );
                read_view.validate()?;
                read_view
            }
        };
        let soil_snapshot_sha256 = match unpublished_physical_beginning.as_ref() {
            Some(beginning) => beginning
                .predecessor_trial()
                .unpublished_trial_sha256()
                .clone(),
            None => {
                self.inner
                    .soil_thermal
                    .v2()?
                    .owner()
                    .snapshot()
                    .map_err(|_| {
                        super::DirectV9RealConsumerError::OwnerClosure("V3 soil snapshot identity")
                    })?
                    .snapshot_sha256
            }
        };
        let hydrology_snapshot = super::unified_beginning_hydrology_snapshot_sha256(
            &soil_adapter,
            &self.inner.surface_configuration,
        )
        .map_err(super::DirectV9RealConsumerError::LandSurfaceShadow)?;
        let forcing_sha256 = input.lse_forcing.canonical_sha256()?;
        let (vegetation_forcing, root_zone_hydraulics) = super::project_live_vegetation_forcing(
            &input.vegetation_forcing,
            &hydrology,
            soil_read,
            self.inner.root_zone_hydraulic_configuration.as_ref(),
            &self.inner.surface_configuration,
            &self.inner.lse_configuration,
            &self.inner.vegetation_configuration,
            &self.inner.vegetation_state,
            v8_configuration.configuration_sha256.clone(),
            hydrology_snapshot.clone(),
            transaction_id,
            day_index,
            interval_index,
        )?;
        let canopy_forcing = match root_zone_hydraulics {
            Some(receipts) => V8CanopyForcingReceipt::try_new_with_root_zone(
                v8_configuration.configuration_sha256.clone(),
                v8_beginning.state_sha256.clone(),
                self.inner.lse_configuration.configuration_sha256.clone(),
                forcing_sha256,
                hydrology_snapshot,
                soil_snapshot_sha256,
                transaction_id,
                vegetation_forcing,
                receipts,
            )
            .map_err(super::DirectV9RealConsumerError::Projection)?,
            None => V8CanopyForcingReceipt::try_new(
                v8_configuration.configuration_sha256.clone(),
                v8_beginning.state_sha256.clone(),
                self.inner.lse_configuration.configuration_sha256.clone(),
                forcing_sha256,
                hydrology_snapshot,
                soil_snapshot_sha256,
                transaction_id,
                vegetation_forcing,
            )
            .map_err(super::DirectV9RealConsumerError::Projection)?,
        };
        let ordinary_prepared_soil;
        let physical_soil = if let Some(beginning) = unpublished_physical_beginning {
            crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::try_from_v2_unpublished(
                beginning,
            )
            .map_err(super::DirectV9RealConsumerError::Projection)?
        } else {
            ordinary_prepared_soil = self.inner.soil_thermal.prepare_next_v2_support(
                wb14_coupled_child_binding.child_support_start_ns,
                wb14_coupled_child_binding.child_support_end_ns,
            )?;
            crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::try_from_v2(
                &ordinary_prepared_soil,
            )
            .map_err(super::DirectV9RealConsumerError::Projection)?
        };
        execute_frozen_litter_v3_fixed_final_pre_ingress(
            v8_configuration,
            v8_beginning,
            &self.inner.vegetation_owner_id,
            &canopy_forcing,
            &self.inner.lse_configuration,
            &self.inner.lse_state,
            resident.lse_configuration(),
            resident.lse_state(),
            &input.lse_forcing,
            &soil_adapter,
            resident.surface_configuration(),
            resident.surface_owner(),
            day_index,
            interval_index,
            &input.wb14_parameters,
            &physical_soil,
            &self.inner.biogeochemistry,
            self.inner.authority,
            complete_lower_boundaries,
            duration_s_bits,
            covered_destinations,
            finalize_wb14_parent_interval,
            self.inner.wb14_parent_working_state.as_ref(),
            Some(wb14_coupled_child_binding),
        )
        .map_err(|error| super::DirectV9RealConsumerError::Physical(error).into())
    }

    pub(super) fn execute_and_accept_frozen_litter_v3(
        &mut self,
        fixed: &crate::land_surface_energy_shadow::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
        support_start_ns: u128,
        support_end_ns: u128,
        finalize_wb14_parent_interval: bool,
        coupled_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    ) -> Result<
        crate::land_surface_energy_shadow::v3_execution::AcceptedFrozenLitterV3RuntimeCandidate,
        DirectV10RealConsumerError,
    > {
        let prepared_soil = self
            .inner
            .soil_thermal
            .prepare_next_v2_support(support_start_ns, support_end_ns)?;
        let soil_seals = openwepp_land_surface_energy::seal_soil_thermal_receipt_free_owner_v2(
            &prepared_soil,
        )
        .map_err(|_| super::DirectV9RealConsumerError::OwnerClosure("V3 soil restart seal"))?;
        let resident =
            self.frozen_litter_v3
                .as_ref()
                .ok_or(super::DirectV9RealConsumerError::Unsupported(
                    "missing native frozen-litter V3 resident",
                ))?;
        let phase_inputs = fixed
            .frozen_litter_tiles
            .iter()
            .map(|tile| tile.phase_free_input.clone())
            .collect::<Vec<_>>();
        let transaction_id = fixed.water_protocol.transaction_id;
        let accepted = crate::land_surface_energy_shadow::v3_execution::execute_frozen_litter_v3_with_heterogeneous_surface_resource(
            &crate::land_surface_energy_shadow::v3_execution::FrozenLitterV3RuntimeInput {
                transaction_id,
                soil_transaction_authority:
                    crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
                        transaction_id,
                        transaction_id,
                    )
                    .map_err(super::DirectV9RealConsumerError::LandSurfaceShadow)?,
                predecessor_transaction_id: resident.lse_state().0.last_accepted_transaction_id,
                parent_support_start_ns: coupled_binding.parent_support_start_ns,
                parent_support_end_ns: coupled_binding.parent_support_end_ns,
                support_start_ns,
                support_end_ns,
                predecessor_receipt_chain_sha256: resident
                    .predecessor_receipt_chain_sha256()
                    .to_owned(),
                surface_configuration: resident.surface_configuration(),
                beginning_surface_owner: resident.surface_owner(),
                lse_configuration: resident.lse_configuration(),
                beginning_lse_state: resident.lse_state(),
                phase_inputs: &phase_inputs,
                current_ingress: &fixed.derived_current_ingress,
                wb14_parent: resident.wb14_parent(),
                finalize_wb14_parent_interval,
                coupled_binding,
                soil_beginning: crate::land_surface_energy_shadow::v3_execution::FrozenLitterV3SoilBeginningV1::PublishableOwner {
                    owner: prepared_soil.beginning_owner(),
                    restart: &soil_seals.restart,
                },
            },
            &fixed.water_protocol.requests,
            &fixed.water_protocol.authorizations,
            &fixed.water_protocol.finalized_uses,
        )
        .map_err(|error| {
            super::DirectV9RealConsumerError::Serialization(format!(
                "frozen-litter V3 runtime: {error}"
            ))
        })?;
        self.frozen_litter_v3
            .as_mut()
            .ok_or(super::DirectV9RealConsumerError::Unsupported(
                "missing native frozen-litter V3 resident",
            ))?
            .accept_runtime_candidate(&accepted)?;
        Ok(accepted)
    }

    /// Project the already accepted V3 physical transaction into the existing
    /// complete vegetation/BGC envelope without invoking the legacy LSE path.
    pub(super) fn construct_frozen_litter_v3_complete_envelope(
        &self,
        day_index: usize,
        duration_s_bits: u64,
        fixed: &crate::land_surface_energy_shadow::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
        accepted: &crate::land_surface_energy_shadow::v3_execution::AcceptedFrozenLitterV3RuntimeCandidate,
        exact_surface_custody: bool,
    ) -> Result<super::UncommittedCoveredV8OwnerEnvelope, DirectV10RealConsumerError> {
        let transaction_id = fixed.water_protocol.transaction_id;
        let mut effective_hydrology_frame = self.inner.hydrology_frame.clone();
        if let Some(parent) = &self.inner.wb14_parent_working_state {
            effective_hydrology_frame.surface_liquid_shadow =
                Some(Box::new(parent.candidate_state().clone()));
        }
        let hydrology = RealHydrologyShadowAdapter::try_from_day_start(
            &effective_hydrology_frame,
            day_index,
            transaction_id,
            f64::from_bits(duration_s_bits),
            self.inner.surface_configuration.owner_id.clone(),
            &self.inner.layer_maps,
        )
        .map_err(super::DirectV9RealConsumerError::RealHydrology)?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hydrology);
        let unified = crate::land_surface_energy_shadow::construct_v3_unified_hydrology_candidate(
            &soil_adapter,
            &self.inner.surface_configuration,
            fixed,
            accepted,
            exact_surface_custody,
        )
        .map_err(super::DirectV9RealConsumerError::LandSurfaceShadow)?;
        let nitrogen = super::BiogeochemistryNitrogenArbiter::try_new(&self.inner.biogeochemistry)?;
        crate::land_surface_energy_shadow::construct_frozen_litter_v3_owner_envelope_v11(
            fixed,
            unified,
            &fixed.vegetation_configuration,
            &fixed.vegetation_beginning,
            &fixed.persistent_forcing,
            &nitrogen,
            &self.inner.biogeochemistry,
            duration_s_bits,
        )
        .map_err(super::DirectV9RealConsumerError::OwnerEnvelope)
        .map_err(Into::into)
    }

    /// Construct the one represented-snow native envelope from the standard
    /// covered fixed finals. The resident frozen-litter V3/V4 pair is read as
    /// native identity/configuration authority only and remains unchanged.
    pub(super) fn construct_stage3_covered_native_complete_envelope(
        &mut self,
        day_index: usize,
        duration_s_bits: u64,
        finalize_wb14_parent_interval: bool,
        coupled_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        fixed: &crate::land_surface_energy_shadow::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
    ) -> Result<super::UncommittedCoveredV8OwnerEnvelope, DirectV10RealConsumerError> {
        let resident =
            self.frozen_litter_v3
                .as_ref()
                .ok_or(super::DirectV9RealConsumerError::Unsupported(
                    "missing native frozen-litter V3 resident",
                ))?;
        let transaction_id = fixed.water_protocol.transaction_id;
        let mut effective_hydrology_frame = self.inner.hydrology_frame.clone();
        if let Some(parent) = &self.inner.wb14_parent_working_state {
            effective_hydrology_frame.surface_liquid_shadow =
                Some(Box::new(parent.candidate_state().clone()));
        }
        let hydrology = RealHydrologyShadowAdapter::try_from_day_start(
            &effective_hydrology_frame,
            day_index,
            transaction_id,
            f64::from_bits(duration_s_bits),
            self.inner.surface_configuration.owner_id.clone(),
            &self.inner.layer_maps,
        )
        .map_err(super::DirectV9RealConsumerError::RealHydrology)?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hydrology);
        let unified = crate::land_surface_energy_shadow::construct_stage3_covered_native_unified_hydrology_candidate(
            &soil_adapter,
            &self.inner.surface_configuration,
            fixed,
            resident.lse_state(),
            self.inner.wb14_parent_working_state.as_ref(),
            finalize_wb14_parent_interval,
            coupled_binding,
        )
        .map_err(super::DirectV9RealConsumerError::LandSurfaceShadow)?;
        let nitrogen = super::BiogeochemistryNitrogenArbiter::try_new(&self.inner.biogeochemistry)?;
        crate::land_surface_energy_shadow::construct_frozen_litter_v3_owner_envelope_v11(
            fixed,
            unified,
            &fixed.vegetation_configuration,
            &fixed.vegetation_beginning,
            &fixed.persistent_forcing,
            &nitrogen,
            &self.inner.biogeochemistry,
            duration_s_bits,
        )
        .map_err(super::DirectV9RealConsumerError::OwnerEnvelope)
        .map_err(Into::into)
    }

    /// Accept every non-soil owner from the already joined V3 envelope, then
    /// reconstruct and install exactly one native V2 soil successor from the
    /// immutable physical candidates and ingress receipts retained by that
    /// envelope. The complete-owner V3 projection independently seals the
    /// prepared soil beginning and its receipt-free restart identity; both
    /// must join byte-for-byte before installation.
    pub(super) fn accept_frozen_litter_v3_complete_envelope(
        &mut self,
        authoritative_beginning: &Self,
        support_start_ns: u128,
        support_end_ns: u128,
        accepted: &crate::land_surface_energy_shadow::v3_execution::AcceptedFrozenLitterV3RuntimeCandidate,
        envelope: &super::UncommittedCoveredV8OwnerEnvelope,
    ) -> Result<(), DirectV10RealConsumerError> {
        let prepared = authoritative_beginning
            .prepare_next_soil_thermal_support_v2(support_start_ns, support_end_ns)?;
        let receipt_free_seals =
            openwepp_land_surface_energy::seal_soil_thermal_receipt_free_owner_v2(&prepared)
                .map_err(|_| {
                    super::DirectV9RealConsumerError::OwnerClosure(
                        "V3 accepted soil receipt-free reconstruction",
                    )
                })?;
        let prepared_bytes = serde_json::to_vec(prepared.beginning_owner())
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        let restart_bytes = serde_json::to_vec(&receipt_free_seals.restart)
            .map_err(|error| super::DirectV9RealConsumerError::Serialization(error.to_string()))?;
        if Some(prepared_bytes.as_slice())
            != accepted
                .complete_owner_projection
                .soil_thermal_owner_envelope_bytes()
            || Some(restart_bytes.as_slice())
                != accepted
                    .complete_owner_projection
                    .soil_thermal_restart_identity_bytes()
        {
            return Err(super::DirectV9RealConsumerError::OwnerClosure(
                "V3 accepted soil projection/prepared beginning join",
            )
            .into());
        }

        let mut operands = crate::land_surface_energy_shadow::physical_soil_energy_operands_v2(
            crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_from_pre_ingress_candidates(
                envelope.transaction_id(),
                support_start_ns,
                support_end_ns,
                envelope.hydrology().pre_ingress_soil_thermal_candidates(),
            )
            .map_err(super::DirectV9RealConsumerError::LandSurfaceShadow)?,
            support_start_ns,
            support_end_ns,
            &authoritative_beginning.inner.lse_configuration.owner_id,
            &authoritative_beginning.inner.surface_configuration.owner_id,
            envelope.hydrology().pre_ingress_soil_thermal_candidates(),
            envelope.hydrology().surface_ingress(),
        )
        .map_err(super::DirectV9RealConsumerError::LandSurfaceShadow)?;
        super::canonicalize_v2_operand_order(prepared.beginning_owner(), &mut operands)?;
        let expected = super::SoilThermalExpectedAcceptedOperandSetV2::try_new(
            prepared.beginning_owner(),
            &authoritative_beginning.inner.lse_configuration,
            operands,
        )?;
        let soil_accepted = super::aggregate_soil_thermal_ending_v2(
            prepared.beginning_owner(),
            &authoritative_beginning.inner.lse_configuration,
            &expected,
        )?;
        let soil_seals = super::seal_soil_thermal_accepted_candidate_v2(
            prepared.beginning_owner(),
            &soil_accepted,
        )?;

        self.inner
            .accept_envelope_preserving_native_v2_soil(envelope.transaction_id(), envelope)?;
        self.vegetation_state = super::project_v9_runtime_to_v10(
            self.inner.vegetation_state(),
            &self.vegetation_configuration,
        )?;
        self.lse_state = super::project_validated_v1_runtime_to_v2(
            &self.inner.lse_configuration,
            self.inner.lse_state(),
            &self.lse_configuration,
            &openwepp_land_surface_energy::Sha256Digest::try_new(
                self.vegetation_configuration.configuration_sha256.clone(),
            )?,
        )?;
        let authoritative_complete_envelope = self.clone();
        let soil_transaction_authority = self
            .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
                &authoritative_complete_envelope,
                prepared.beginning_owner(),
            )?;
        self.install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
            &authoritative_complete_envelope,
            prepared.beginning_owner(),
            soil_transaction_authority,
            soil_accepted,
            soil_seals,
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod complete_envelope_soil_source_order_guards {
    #[test]
    fn complete_envelope_stages_non_soil_source_before_the_v3_soil_install() {
        let source = include_str!("frozen_litter_v3_adoption.rs");
        let body = source
            .split("pub(super) fn accept_frozen_litter_v3_complete_envelope(")
            .nth(1)
            .expect("complete-envelope acceptance")
            .split("#[cfg(test)]")
            .next()
            .expect("complete-envelope body");
        let accept = body
            .find("accept_envelope_preserving_native_v2_soil")
            .expect("non-soil envelope acceptance");
        let vegetation = body[accept..]
            .find("project_v9_runtime_to_v10")
            .map(|offset| accept + offset)
            .expect("V10 vegetation projection");
        let lse = body[vegetation..]
            .find("project_validated_v1_runtime_to_v2")
            .map(|offset| vegetation + offset)
            .expect("V10 LSE projection");
        let authenticate = body[lse..]
            .find("authenticate_soil_thermal_prepared_beginning_install_authority_v3")
            .map(|offset| lse + offset)
            .expect("three-domain source authentication");
        let install = body[authenticate..]
            .find("install_soil_thermal_accepted_v2_from_authenticated_beginning_v3")
            .map(|offset| authenticate + offset)
            .expect("three-domain soil install");
        assert!(accept < vegetation && vegetation < lse);
        assert!(lse < authenticate && authenticate < install);
        assert!(!body.contains("install_soil_thermal_accepted_v2("));
        assert!(!body.contains("install_soil_thermal_accepted_v2_from_beginning("));
        assert!(!body.contains("install_soil_thermal_accepted_v2_from_authenticated_beginning("));
    }

    #[test]
    fn trusted_wb14_staging_does_not_cross_the_restart_byte_boundary() {
        let source = include_str!("frozen_litter_v3_adoption.rs");
        let join = source
            .split("fn stage_envelope_wb14_parent(")
            .nth(1)
            .expect("native/legacy WB14 join")
            .split("pub(crate) const fn accepted_publication_count")
            .next()
            .expect("native/legacy WB14 join body");
        assert!(join.contains("try_stage_validated_liquid_arithmetic"));
        assert!(join.contains("try_from_validated_liquid_arithmetic"));
        assert!(join.contains("try_finalize_validated_liquid_owner"));
        assert!(!join.contains("restart_bytes"));
        assert!(!join.contains("serde_json"));

        let staging = source
            .split("pub(crate) fn stage_frozen_litter_wb14_parent_from_inner_v1(")
            .nth(1)
            .expect("carrier WB14 staging")
            .split("pub fn install_frozen_litter_v3_resident(")
            .next()
            .expect("carrier WB14 staging body");
        assert_eq!(staging.matches("stage_envelope_wb14_parent").count(), 1);
        assert!(!staging.contains("restart_wb14_parent_working_state_bytes"));
        assert!(!staging.contains("restart_bytes"));
        assert!(!staging.contains("serde_json"));
    }
}
