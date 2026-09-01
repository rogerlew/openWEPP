//! Native frozen-litter V3 resident carried by the real consumer candidate.

use std::collections::BTreeSet;

use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyV3State, SurfaceConfiguration,
};

use crate::land_surface_energy_shadow::{
    LandSurfaceEnergyRealHydrologyAdapter, V8CanopyForcingReceipt,
    execute_frozen_litter_v3_fixed_final_pre_ingress,
};
use crate::vegetation_real_hydrology_shadow::RealHydrologyShadowAdapter;
use crate::{SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerEnvelopeV2};

use super::frozen_litter_v3_publication_retention::FrozenLitterV3PublicationSupportV1;
use super::{DirectV10RealConsumerError, DirectV10RealConsumerShadow};

/// Complete native beginning owner set. It deliberately has no V1/V2 LSE or
/// surface-owner projection API.
#[derive(Clone, Debug, PartialEq)]
pub struct FrozenLitterV3Resident {
    lse_configuration: LandSurfaceEnergyConfiguration,
    lse_state: LandSurfaceEnergyV3State,
    surface_configuration: SurfaceLiquidConfigurationV2,
    surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    accepted_publications: Vec<FrozenLitterV3PublicationSupportV1>,
    wb14_parent: Option<crate::direct_runtime::DirectWb14ParentWorkingStateV2>,
    predecessor_receipt_chain_sha256: String,
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
        Ok(Self {
            lse_configuration,
            lse_state,
            surface_configuration,
            surface_owner,
            accepted_publications: Vec::new(),
            wb14_parent: None,
            predecessor_receipt_chain_sha256,
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

    pub(crate) const fn accepted_publication_count(&self) -> usize {
        self.accepted_publications.len()
    }

    pub fn restore_accepted_publication_supports_canonical_bytes(
        &mut self,
        bytes: &[Vec<u8>],
    ) -> Result<(), crate::DirectSurfaceLiquidError> {
        let restored = bytes
            .iter()
            .map(|value| {
                FrozenLitterV3PublicationSupportV1::from_canonical_bytes(
                    &self.surface_configuration,
                    value,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.accepted_publications = restored;
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
        self.accepted_publications = restored_publications;
        self.wb14_parent = restored_wb14;
        self.predecessor_receipt_chain_sha256 = predecessor_receipt_chain_sha256.to_owned();
        Ok(())
    }

    pub(super) fn retain_accepted_publication(
        &mut self,
        projection: &crate::SurfaceLiquidCompleteOwnerProjectionV3,
        receipts: &[openwepp_land_surface_energy::LitterPhaseReceipt],
    ) -> Result<(), crate::DirectSurfaceLiquidError> {
        let support = FrozenLitterV3PublicationSupportV1::try_new(
            &self.surface_configuration,
            projection,
            receipts,
        )?;
        self.accepted_publications.push(support);
        Ok(())
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
        self.retain_accepted_publication(
            &candidate.complete_owner_projection,
            &candidate.litter_phase_receipts,
        )?;
        self.lse_state = candidate.ending_lse_state.clone();
        self.surface_owner = candidate.ending_surface_owner.clone();
        self.wb14_parent = candidate.ingress.parent_working_state().cloned();
        self.predecessor_receipt_chain_sha256 = candidate
            .complete_owner_projection
            .identity()
            .receipt_chain_sha256
            .clone();
        Ok(())
    }
}

impl DirectV10RealConsumerShadow {
    pub fn install_frozen_litter_v3_resident(
        &mut self,
        resident: FrozenLitterV3Resident,
    ) -> Result<(), DirectV10RealConsumerError> {
        let checked = FrozenLitterV3Resident::try_new(
            resident.lse_configuration.clone(),
            resident.lse_state.clone(),
            resident.surface_configuration.clone(),
            resident.surface_owner.clone(),
        )?;
        let mut checked = checked;
        let bytes = resident.accepted_publication_supports_canonical_bytes()?;
        checked.restore_accepted_publication_supports_canonical_bytes(&bytes)?;
        checked.wb14_parent = resident.wb14_parent.clone();
        checked
            .predecessor_receipt_chain_sha256
            .clone_from(&resident.predecessor_receipt_chain_sha256);
        self.frozen_litter_v3 = Some(checked);
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
        let transaction_id = TransactionId(
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
        if input.lse_forcing.transaction_id != transaction_id
            || input.lse_forcing.interval_s.to_bits() != duration_s_bits
            || input.lse_forcing.snow_present_at_beginning
            || input.lse_forcing.snow_present_at_end
            || input.lse_forcing.snow_terminal_payload_present
            || !input.lse_forcing.runon_parcels.is_empty()
        {
            return Err(super::DirectV9RealConsumerError::Unsupported(
                "native V3 forcing identity/domain",
            )
            .into());
        }
        input.lse_forcing.validate(transaction_id)?;
        let (v8_configuration, v8_beginning) = super::project_v9_runtime_to_v8(
            &self.inner.vegetation_configuration,
            &self.inner.vegetation_state,
        )
        .map_err(super::DirectV9RealConsumerError::V9)?;
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
        let soil_read = self.inner.soil_thermal.read_view();
        soil_read.validate()?;
        let soil_snapshot_sha256 = self
            .inner
            .soil_thermal
            .v2()?
            .owner()
            .snapshot()
            .map_err(|_| {
                super::DirectV9RealConsumerError::OwnerClosure("V3 soil snapshot identity")
            })?
            .snapshot_sha256;
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
        let prepared_soil = self.inner.soil_thermal.prepare_next_v2_support(
            wb14_coupled_child_binding.child_support_start_ns,
            wb14_coupled_child_binding.child_support_end_ns,
        )?;
        let physical_soil =
            crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::try_from_v2(
                &prepared_soil,
            )
            .map_err(super::DirectV9RealConsumerError::Projection)?;
        execute_frozen_litter_v3_fixed_final_pre_ingress(
            &v8_configuration,
            &v8_beginning,
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
            None,
            duration_s_bits,
            None,
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
        let accepted = crate::land_surface_energy_shadow::v3_execution::execute_frozen_litter_v3(
            &crate::land_surface_energy_shadow::v3_execution::FrozenLitterV3RuntimeInput {
                transaction_id,
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
                soil_thermal_owner: prepared_soil.beginning_owner(),
                soil_thermal_restart: &soil_seals.restart,
            },
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
        if prepared_bytes
            != accepted
                .complete_owner_projection
                .soil_thermal_owner_envelope_bytes()
            || restart_bytes
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
        self.install_soil_thermal_accepted_v2_from_beginning(
            authoritative_beginning,
            prepared.beginning_owner(),
            soil_accepted,
            soil_seals,
        )?;
        Ok(())
    }
}
