//! BGC receiving-owner construction and the heterogeneous covered V8 envelope.
//!
//! Projection and vegetation persistence are completed before this boundary.
//! This module validates the already sealed candidates, joins root D/A/F to the
//! actual real-hydrology owner, and lets BGC construct its own receipts. It
//! exposes no partial or whole-owner commit API.

#![allow(dead_code)]

use std::collections::BTreeMap;

use openwepp_biogeochemistry::{
    BiogeochemistryError, BiogeochemistryOwnerCandidate, BiogeochemistryState, MaterialPool,
    MaterialProposal, MaterialReceipt, TransformationsMode, construct_biogeochemistry_candidate,
};
use openwepp_kernel_contract::{TileId, TransactionId};
use openwepp_land_surface_energy::{
    AcceptedCoveredVegetationOperands, CoveredCanopyAirEnergyOperands,
    CoveredLowerBoundaryEnergyOperands, CoveredOccupancyEnergyOperands, GroundWaterKey, OfeId,
    RequestingComponent, Stage3SnowCoveredLowerBoundary, Stage3SnowOpticalBoundaryReceiptV1,
    WaterAmount,
};
use openwepp_vegetation::{
    NitrogenArbiter, UncommittedV8VegetationCandidate, V8ComponentOccupancyBinding,
    V8CoupledOwnedState, V8PersistentForcingReceipt, VegetationConfiguration, VegetationError,
    carbon_nitrogen::MaterialTransfer, construct_uncommitted_v8_vegetation_candidate,
    execute_uncommitted_v8_persistent_phase, execute_uncommitted_v8_persistent_phase_v11,
};
use thiserror::Error;

use super::{
    CoveredForestShadowResult, UnifiedRealHydrologyCandidate,
    multi_tile_runtime::MultiTileRuntimeResult,
    v8_projection::{
        V8CoveredProjection, project_covered_forest_v8_passes, project_multi_tile_v8_passes_v11,
    },
};

pub(crate) type FixedCapCanopyReleasesByDestination = BTreeMap<
    (OfeId, TileId),
    (
        crate::DirectCanopyLiquidRelease,
        openwepp_coupled_time::Digest32,
    ),
>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum V8OwnerFailurePhase {
    Persistent,
    VegetationCandidate,
    BiogeochemistryCandidate,
    EnvelopeValidation,
}

type OwnerFailureHook<'a> =
    Option<&'a dyn Fn(V8OwnerFailurePhase) -> Result<(), CoveredV8OwnerEnvelopeError>>;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum CoveredV8OwnerEnvelopeError {
    #[error("V8 covered-owner identity failure: {0}")]
    Identity(&'static str),
    #[error(transparent)]
    LandSurface(#[from] openwepp_land_surface_energy::LandSurfaceEnergyError),
    #[error(transparent)]
    Vegetation(#[from] VegetationError),
    #[error(transparent)]
    Biogeochemistry(#[from] BiogeochemistryError),
    #[error(transparent)]
    Projection(#[from] super::V8ProjectionError),
}

/// Complete uncommitted covered owner set. LSE and soil-thermal candidates are
/// retained inside `physical`; the hydrology candidate is the actual unified
/// owner result rather than a reconstructed inventory.
#[derive(Clone, Debug, PartialEq)]
pub struct UncommittedCoveredV8OwnerEnvelope {
    transaction_id: TransactionId,
    vegetation: UncommittedV8VegetationCandidate,
    physical: CoveredV8PhysicalOwner,
    biogeochemistry: BiogeochemistryOwnerCandidate,
}

impl UncommittedCoveredV8OwnerEnvelope {
    pub(crate) fn into_provisional_physical(self) -> ProvisionalCoveredV8PhysicalEvaluationV1 {
        ProvisionalCoveredV8PhysicalEvaluationV1 {
            transaction_id: self.transaction_id,
            physical: self.physical,
            #[cfg(test)]
            native_inactive_projection_sha256: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CoveredLseIterationState {
    pub canopy_air_temperature_k: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub snow_temperature_k: f64,
    pub snow_sensible_w_m2: f64,
    pub snow_vapor_kg_m2_s: f64,
    pub snow_latent_w_m2: f64,
    pub snow_net_longwave_w_m2: f64,
    pub component_temperatures_k: Vec<(String, [f64; 4])>,
    pub component_carrier_surfaces: Vec<CoveredCarrierComponentState>,
    pub canopy_sensible_w_m2: f64,
    pub canopy_vapor_kg_m2_s: f64,
    pub sensible_to_reference_air_w_m2: f64,
    pub vapor_to_reference_air_kg_m2_s: f64,
    pub shared_heat_residual_w_m2: f64,
    pub shared_heat_tolerance_w_m2: f64,
    pub shared_vapor_residual_kg_m2_s: f64,
    pub shared_vapor_tolerance_kg_m2_s: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CoveredCarrierComponentState {
    pub vertical_occupancy_ordinal: u32,
    pub occupancy_id: String,
    pub component_ordinal: u8,
    pub surface_area_m2_m2_tile: f64,
    pub emissive_area_m2_m2_tile: f64,
    pub heat_conductance_m_s_tile: f64,
    pub vapor_conductance_m_s_tile: f64,
    pub vapor_authorization_kg_m2_tile_s: Option<f64>,
    pub temperature_k: f64,
    pub specific_humidity_kg_kg: f64,
    pub sensible_to_canopy_air_w_m2: f64,
    pub vapor_to_canopy_air_kg_m2_s: f64,
}

#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
enum CoveredV8PhysicalOwner {
    Legacy(CoveredForestShadowResult),
    MultiTile(MultiTileRuntimeResult),
    FrozenLitterV3 {
        hydrology: UnifiedRealHydrologyCandidate,
        fixed: Box<super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate>,
    },
}

/// Unpublished physical result used only while solving the covered Stage-3
/// fixed point. It contains no vegetation/BGC candidate, V8 projection
/// receipt, accepted publication, or complete-owner envelope.
pub(crate) struct ProvisionalCoveredV8PhysicalEvaluationV1 {
    transaction_id: TransactionId,
    physical: CoveredV8PhysicalOwner,
    #[cfg(test)]
    native_inactive_projection_sha256: Option<(
        openwepp_coupled_time::Digest32,
        openwepp_coupled_time::Digest32,
    )>,
}

impl ProvisionalCoveredV8PhysicalEvaluationV1 {
    pub(crate) fn try_new(
        physical: MultiTileRuntimeResult,
    ) -> Result<Self, CoveredV8OwnerEnvelopeError> {
        let transaction_id = physical.hydrology_candidate().transaction_id();
        if physical.finalized_tiles().is_empty() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "empty provisional covered physical evaluation",
            ));
        }
        Ok(Self {
            transaction_id,
            physical: CoveredV8PhysicalOwner::MultiTile(physical),
            #[cfg(test)]
            native_inactive_projection_sha256: None,
        })
    }

    pub(crate) fn try_new_stage3_covered_native(
        fixed: super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
        hydrology: UnifiedRealHydrologyCandidate,
    ) -> Result<Self, CoveredV8OwnerEnvelopeError> {
        validate_frozen_litter_v3_fixed_identity(&fixed, &hydrology)?;
        if fixed.stage3_covered_native_tiles.is_empty() || !fixed.frozen_litter_tiles.is_empty() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "Stage3CoveredNative physical-only posture",
            ));
        }
        let transaction_id = hydrology.transaction_id();
        Ok(Self {
            transaction_id,
            physical: CoveredV8PhysicalOwner::FrozenLitterV3 {
                hydrology,
                fixed: Box::new(fixed),
            },
            #[cfg(test)]
            native_inactive_projection_sha256: None,
        })
    }

    #[cfg(test)]
    pub(crate) fn bind_native_inactive_projection_for_test(
        &mut self,
        v3_sha256: openwepp_coupled_time::Digest32,
        v4_sha256: openwepp_coupled_time::Digest32,
    ) {
        self.native_inactive_projection_sha256 = Some((v3_sha256, v4_sha256));
    }

    #[cfg(test)]
    pub(crate) const fn native_inactive_projection_for_test(
        &self,
    ) -> Option<(
        openwepp_coupled_time::Digest32,
        openwepp_coupled_time::Digest32,
    )> {
        self.native_inactive_projection_sha256
    }

    #[cfg(test)]
    pub(crate) fn validate_private_arbitration_projection_sensitivity_v1(
        &self,
    ) -> Result<(), CoveredV8OwnerEnvelopeError> {
        if !matches!(self.physical, CoveredV8PhysicalOwner::FrozenLitterV3 { .. }) {
            return Ok(());
        }
        let baseline = self.canonical_private_projection_v1()?.sha256;
        let mut soil_poison = Self {
            transaction_id: self.transaction_id,
            physical: self.physical.clone(),
            native_inactive_projection_sha256: self.native_inactive_projection_sha256,
        };
        let CoveredV8PhysicalOwner::FrozenLitterV3 { hydrology, .. } = &mut soil_poison.physical
        else {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "native arbitration projection posture",
            ));
        };
        let source = hydrology
            .arbitration
            .soil
            .requests
            .first_mut()
            .map(|request| &mut request.source)
            .or_else(|| {
                hydrology
                    .arbitration
                    .soil
                    .authorizations
                    .first_mut()
                    .map(|authorization| &mut authorization.source)
            })
            .ok_or(CoveredV8OwnerEnvelopeError::Identity(
                "native soil arbitration projection source",
            ))?;
        source.ofe_lane.lane_id ^= 1_u32 << 31;
        if soil_poison.canonical_private_projection_v1()?.sha256 == baseline {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "soil arbitration source omitted from private projection",
            ));
        }

        let mut surface_poison = Self {
            transaction_id: self.transaction_id,
            physical: self.physical.clone(),
            native_inactive_projection_sha256: self.native_inactive_projection_sha256,
        };
        let CoveredV8PhysicalOwner::FrozenLitterV3 { hydrology, .. } = &mut surface_poison.physical
        else {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "native surface arbitration projection posture",
            ));
        };
        hydrology.arbitration.surface = None;
        if surface_poison.canonical_private_projection_v1()?.sha256 == baseline {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "surface arbitration omitted from private projection",
            ));
        }

        let mut soil_candidate_poison = Self {
            transaction_id: self.transaction_id,
            physical: self.physical.clone(),
            native_inactive_projection_sha256: self.native_inactive_projection_sha256,
        };
        let CoveredV8PhysicalOwner::FrozenLitterV3 { hydrology, .. } =
            &mut soil_candidate_poison.physical
        else {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "native soil-candidate projection posture",
            ));
        };
        let carry = hydrology
            .pre_ingress_soil_thermal_candidates
            .first_mut()
            .and_then(|candidate| candidate.layers.first_mut())
            .map(|layer| &mut layer.beginning_enthalpy_carry)
            .ok_or(CoveredV8OwnerEnvelopeError::Identity(
                "native soil-candidate projection carry",
            ))?;
        *carry = openwepp_land_surface_energy::ExactDyadicEnthalpy::from_f64(f64::from_bits(1))
            .map_err(|_| {
                CoveredV8OwnerEnvelopeError::Identity("native soil-candidate projection poison")
            })?;
        if soil_candidate_poison
            .canonical_private_projection_v1()?
            .sha256
            == baseline
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "soil candidate carry omitted from private projection",
            ));
        }

        let mut fixed_beginning_poison = Self {
            transaction_id: self.transaction_id,
            physical: self.physical.clone(),
            native_inactive_projection_sha256: self.native_inactive_projection_sha256,
        };
        let CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } =
            &mut fixed_beginning_poison.physical
        else {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "native fixed arbitration projection posture",
            ));
        };
        let lane = fixed
            .soil_arbitration
            .beginning_frame
            .lanes
            .first_mut()
            .ok_or(CoveredV8OwnerEnvelopeError::Identity(
                "native fixed arbitration beginning lane",
            ))?;
        lane.water.infiltration_m = f64::from_bits(lane.water.infiltration_m.to_bits() ^ 1);
        if fixed_beginning_poison
            .canonical_private_projection_v1()?
            .sha256
            == baseline
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "native fixed arbitration beginning omitted from private projection",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub(crate) const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    /// Canonical, read-only test projection of the complete private physical
    /// owner.  The projection is made at the production boundary while the
    /// move-only value is still intact; it exposes only a digest and explicit
    /// cardinalities, never a clone or a publishable owner representation.
    #[cfg(test)]
    pub(crate) fn canonical_private_projection_v1(
        &self,
    ) -> Result<CanonicalCoveredPrivatePhysicalProjectionV1, CoveredV8OwnerEnvelopeError> {
        fn push_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
            bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
            bytes.extend_from_slice(value);
        }
        fn push_str(bytes: &mut Vec<u8>, value: &str) {
            push_bytes(bytes, value.as_bytes());
        }
        fn push_f64(bytes: &mut Vec<u8>, value: f64) {
            bytes.extend_from_slice(&value.to_bits().to_be_bytes());
        }
        fn push_json<T: serde::Serialize>(
            bytes: &mut Vec<u8>,
            value: &T,
            detail: &'static str,
        ) -> Result<(), CoveredV8OwnerEnvelopeError> {
            let encoded = serde_json::to_vec(value)
                .map_err(|_| CoveredV8OwnerEnvelopeError::Identity(detail))?;
            push_bytes(bytes, &encoded);
            Ok(())
        }
        fn push_source(
            bytes: &mut Vec<u8>,
            source: &crate::vegetation_real_hydrology_shadow::RealHydrologySourceKey,
        ) {
            bytes.extend_from_slice(&(source.ofe_lane.lane_index as u64).to_be_bytes());
            bytes.extend_from_slice(&source.ofe_lane.lane_id.to_be_bytes());
            push_str(bytes, source.layer_id.as_str());
        }

        let hydrology = self.hydrology();
        let ingress = hydrology.surface_ingress();
        let lse_states = self.covered_lse_iteration_state_by_destination()?;
        let releases = self.fixed_cap_canopy_releases_by_destination(1.0)?;
        let mut bytes = b"OPENWEPP_CANONICAL_COVERED_PRIVATE_PHYSICAL_PROJECTION_V1\0".to_vec();
        bytes.extend_from_slice(&self.transaction_id.0.to_be_bytes());
        bytes.push(match &self.physical {
            CoveredV8PhysicalOwner::Legacy(_) => 0,
            CoveredV8PhysicalOwner::MultiTile(_) => 1,
            CoveredV8PhysicalOwner::FrozenLitterV3 { .. } => 2,
        });
        push_str(
            &mut bytes,
            &hydrology.receiver_closure_operands().canonical_sha256(),
        );
        let beginning_snapshot = hydrology
            .receiver_closure_operands()
            .beginning_hydrology_snapshot_sha256
            .as_str();
        push_bytes(
            &mut bytes,
            &hydrology
                .beginning_frame()
                .canonical_hydrology_physical_projection_v1(beginning_snapshot)
                .map_err(|_| {
                    CoveredV8OwnerEnvelopeError::Identity(
                        "beginning hydrology frame private projection",
                    )
                })?,
        );
        push_bytes(
            &mut bytes,
            &hydrology
                .ending_frame()
                .canonical_hydrology_physical_projection_v1(beginning_snapshot)
                .map_err(|_| {
                    CoveredV8OwnerEnvelopeError::Identity(
                        "ending hydrology frame private projection",
                    )
                })?,
        );
        push_bytes(
            &mut bytes,
            &hydrology
                .surface_resource()
                .canonical_private_projection_v1()
                .map_err(|_| {
                    CoveredV8OwnerEnvelopeError::Identity(
                        "surface resource private physical projection",
                    )
                })?,
        );
        push_bytes(
            &mut bytes,
            &ingress.canonical_private_projection_v1().map_err(|_| {
                CoveredV8OwnerEnvelopeError::Identity("surface ingress private projection")
            })?,
        );
        push_str(&mut bytes, hydrology.pre_ingress_soil_thermal_sha256());
        push_str(&mut bytes, ingress.wb14_child_receipt_set_sha256().as_str());
        push_bytes(&mut bytes, ingress.wb14_child_replay_bytes());
        match ingress.wb14_parent_receipt_set_sha256() {
            Some(value) => {
                bytes.push(1);
                push_str(&mut bytes, value.as_str());
            }
            None => bytes.push(0),
        }
        match ingress.wb14_parent_replay_bytes() {
            Some(value) => {
                bytes.push(1);
                push_bytes(&mut bytes, value);
            }
            None => bytes.push(0),
        }
        let arbitration = hydrology.arbitration();
        bytes.extend_from_slice(&arbitration.transaction_id.0.to_be_bytes());
        bytes.extend_from_slice(&arbitration.soil.transaction_id.0.to_be_bytes());
        push_bytes(
            &mut bytes,
            &arbitration
                .soil
                .beginning_frame
                .canonical_hydrology_physical_projection_v1(beginning_snapshot)
                .map_err(|_| {
                    CoveredV8OwnerEnvelopeError::Identity(
                        "soil arbitration beginning-frame private projection",
                    )
                })?,
        );
        bytes.extend_from_slice(&(arbitration.soil.requests.len() as u64).to_be_bytes());
        for request in &arbitration.soil.requests {
            push_json(
                &mut bytes,
                &request.request,
                "soil arbitration request private projection",
            )?;
            push_source(&mut bytes, &request.source);
        }
        bytes.extend_from_slice(&(arbitration.soil.authorizations.len() as u64).to_be_bytes());
        for authorization in &arbitration.soil.authorizations {
            push_json(
                &mut bytes,
                &authorization.authorization,
                "soil arbitration authorization private projection",
            )?;
            push_source(&mut bytes, &authorization.source);
        }
        match arbitration.surface.as_ref() {
            Some(surface) => {
                bytes.push(1);
                bytes.extend_from_slice(&surface.transaction_id().0.to_be_bytes());
                match surface.expected_predecessor() {
                    Some(transaction) => {
                        bytes.push(1);
                        bytes.extend_from_slice(&transaction.0.to_be_bytes());
                    }
                    None => bytes.push(0),
                }
                push_json(
                    &mut bytes,
                    surface.beginning_state(),
                    "surface arbitration beginning-state private projection",
                )?;
                push_json(
                    &mut bytes,
                    &surface.requests(),
                    "surface arbitration requests private projection",
                )?;
                push_json(
                    &mut bytes,
                    &surface.authorizations(),
                    "surface arbitration authorizations private projection",
                )?;
                push_json(
                    &mut bytes,
                    &surface.request_store_keys(),
                    "surface arbitration store-key private projection",
                )?;
            }
            None => bytes.push(0),
        }
        for value in [
            hydrology.arbitration().requests.len(),
            hydrology.arbitration().authorizations.len(),
            hydrology.finalized_uses().len(),
            hydrology.condensation_credits().len(),
            ingress.receipts().len(),
            ingress.ledgers().len(),
            hydrology.ending_lse_tile_states().len(),
            hydrology.pre_ingress_soil_thermal_candidates().len(),
            hydrology.soil_thermal_candidates().len(),
            hydrology.rollback_hashes().len(),
        ] {
            bytes.extend_from_slice(&(value as u64).to_be_bytes());
        }
        push_json(
            &mut bytes,
            &hydrology.arbitration().requests,
            "canonical private physical requests projection",
        )?;
        push_json(
            &mut bytes,
            &hydrology.arbitration().authorizations,
            "canonical private physical authorizations projection",
        )?;
        push_json(
            &mut bytes,
            &hydrology.finalized_uses(),
            "canonical private physical uses projection",
        )?;
        push_json(
            &mut bytes,
            &hydrology.condensation_credits(),
            "canonical private physical condensation projection",
        )?;
        push_json(
            &mut bytes,
            &ingress.receipts(),
            "canonical private physical ingress receipts projection",
        )?;
        push_json(
            &mut bytes,
            &ingress.ledgers(),
            "canonical private physical ingress ledgers projection",
        )?;
        push_json(
            &mut bytes,
            &hydrology.ending_lse_tile_states(),
            "canonical private physical LSE tiles projection",
        )?;
        push_json(
            &mut bytes,
            &hydrology.rollback_hashes(),
            "canonical private physical rollback projection",
        )?;
        for ((ofe, tile), (release, source_sha256)) in &releases {
            push_str(&mut bytes, ofe.as_str());
            push_str(&mut bytes, tile.as_str());
            bytes.extend_from_slice(source_sha256.as_bytes());
            for amount in [
                &release.throughfall,
                &release.initial_drainage,
                &release.second_drainage,
                &release.stemflow,
            ] {
                for value in [
                    amount.mass_kg_m2_tile_ground,
                    amount.temperature_k,
                    amount.specific_liquid_enthalpy_j_kg,
                    amount.start_s,
                    amount.end_s,
                ] {
                    push_f64(&mut bytes, value);
                }
            }
        }
        push_bytes(
            &mut bytes,
            &super::multi_tile_runtime::canonical_soil_thermal_candidate_set_projection_v1(
                b"OPENWEPP_PRE_INGRESS_SOIL_THERMAL_CANDIDATES_V1\0",
                hydrology.pre_ingress_soil_thermal_candidates(),
            )
            .map_err(|_| {
                CoveredV8OwnerEnvelopeError::Identity(
                    "pre-ingress soil candidate private projection",
                )
            })?,
        );
        push_bytes(
            &mut bytes,
            &super::multi_tile_runtime::canonical_soil_thermal_candidate_set_projection_v1(
                b"OPENWEPP_ENDING_SOIL_THERMAL_CANDIDATES_V1\0",
                hydrology.soil_thermal_candidates(),
            )
            .map_err(|_| {
                CoveredV8OwnerEnvelopeError::Identity("ending soil candidate private projection")
            })?,
        );
        for ((ofe, tile), state) in &lse_states {
            push_str(&mut bytes, ofe.as_str());
            push_str(&mut bytes, tile.as_str());
            for value in [
                state.canopy_air_temperature_k,
                state.canopy_air_specific_humidity_kg_kg,
                state.snow_temperature_k,
                state.snow_sensible_w_m2,
                state.snow_vapor_kg_m2_s,
                state.snow_latent_w_m2,
                state.snow_net_longwave_w_m2,
                state.canopy_sensible_w_m2,
                state.canopy_vapor_kg_m2_s,
                state.sensible_to_reference_air_w_m2,
                state.vapor_to_reference_air_kg_m2_s,
                state.shared_heat_residual_w_m2,
                state.shared_heat_tolerance_w_m2,
                state.shared_vapor_residual_kg_m2_s,
                state.shared_vapor_tolerance_kg_m2_s,
            ] {
                push_f64(&mut bytes, value);
            }
            bytes.extend_from_slice(&(state.component_temperatures_k.len() as u64).to_be_bytes());
            for (id, temperatures) in &state.component_temperatures_k {
                push_str(&mut bytes, id);
                for value in temperatures {
                    push_f64(&mut bytes, *value);
                }
            }
            bytes.extend_from_slice(&(state.component_carrier_surfaces.len() as u64).to_be_bytes());
            for component in &state.component_carrier_surfaces {
                bytes.extend_from_slice(&component.vertical_occupancy_ordinal.to_be_bytes());
                push_str(&mut bytes, &component.occupancy_id);
                bytes.push(component.component_ordinal);
                for value in [
                    component.surface_area_m2_m2_tile,
                    component.emissive_area_m2_m2_tile,
                    component.heat_conductance_m_s_tile,
                    component.vapor_conductance_m_s_tile,
                    component.temperature_k,
                    component.specific_humidity_kg_kg,
                    component.sensible_to_canopy_air_w_m2,
                    component.vapor_to_canopy_air_kg_m2_s,
                ] {
                    push_f64(&mut bytes, value);
                }
                match component.vapor_authorization_kg_m2_tile_s {
                    Some(value) => {
                        bytes.push(1);
                        push_f64(&mut bytes, value);
                    }
                    None => bytes.push(0),
                }
            }
        }
        let (potential_tile_count, finalized_tile_count, weighted_ofe_count, native_tile_count) =
            match &self.physical {
                CoveredV8PhysicalOwner::Legacy(_) => (0, 0, 0, 0),
                CoveredV8PhysicalOwner::MultiTile(value) => {
                    push_bytes(
                        &mut bytes,
                        &value
                            .canonical_covered_physical_projection_v1()
                            .map_err(|_| {
                                CoveredV8OwnerEnvelopeError::Identity(
                                    "multi-tile covered physical projection",
                                )
                            })?,
                    );
                    (
                        value.potential_tiles().len(),
                        value.finalized_tiles().len(),
                        value.weighted_ofe_energy().len(),
                        0,
                    )
                }
                CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } => {
                    push_bytes(
                        &mut bytes,
                        &fixed
                            .canonical_stage3_native_physical_projection_v1()
                            .map_err(|error| {
                                CoveredV8OwnerEnvelopeError::Identity(match error {
                                    super::LandSurfaceEnergyShadowError::Identity(detail) => detail,
                                    _ => "native covered physical projection",
                                })
                            })?,
                    );
                    (
                        fixed.potential_vegetation_operands.len(),
                        fixed.legacy_tiles.len(),
                        0,
                        fixed.stage3_covered_native_tiles.len(),
                    )
                }
            };
        for value in [
            potential_tile_count,
            finalized_tile_count,
            weighted_ofe_count,
            native_tile_count,
            lse_states.len(),
            releases.len(),
        ] {
            bytes.extend_from_slice(&(value as u64).to_be_bytes());
        }
        Ok(CanonicalCoveredPrivatePhysicalProjectionV1 {
            sha256: openwepp_coupled_time::digest_bytes(&bytes),
            potential_tile_count,
            finalized_tile_count,
            weighted_ofe_count,
            native_tile_count,
            lse_destination_count: lse_states.len(),
            release_destination_count: releases.len(),
        })
    }

    #[must_use]
    pub(crate) fn hydrology(&self) -> &UnifiedRealHydrologyCandidate {
        self.physical.hydrology()
    }

    #[must_use]
    pub(crate) const fn is_stage3_covered_native(&self) -> bool {
        matches!(
            &self.physical,
            CoveredV8PhysicalOwner::FrozenLitterV3 { .. }
        )
    }

    pub(crate) fn fixed_cap_canopy_releases_by_destination(
        &self,
        interval_s: f64,
    ) -> Result<FixedCapCanopyReleasesByDestination, CoveredV8OwnerEnvelopeError> {
        match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => {
                fixed_cap_canopy_releases_from_multi_tile(value, interval_s)
            }
            CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } => {
                fixed_cap_canopy_releases_from_frozen_litter_v3(fixed, interval_s)
            }
            CoveredV8PhysicalOwner::Legacy(_) => Err(CoveredV8OwnerEnvelopeError::Identity(
                "physical-only fixed-cap release requires multi-tile owner",
            )),
        }
    }

    pub(crate) fn covered_lse_iteration_state_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), CoveredLseIterationState>, CoveredV8OwnerEnvelopeError>
    {
        match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => {
                covered_lse_iteration_states_from_multi_tile(value)
            }
            CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } => {
                covered_lse_iteration_states_from_frozen_litter_v3(fixed)
            }
            CoveredV8PhysicalOwner::Legacy(_) => Err(CoveredV8OwnerEnvelopeError::Identity(
                "physical-only iteration state requires multi-tile owner",
            )),
        }
    }

    pub(crate) fn covered_snow_longwave_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
        match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => {
                covered_snow_longwave_from_multi_tile(value)
            }
            CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } => {
                covered_snow_longwave_from_frozen_litter_v3(fixed)
            }
            CoveredV8PhysicalOwner::Legacy(_) => Err(CoveredV8OwnerEnvelopeError::Identity(
                "physical-only longwave requires multi-tile owner",
            )),
        }
    }

    pub(crate) fn covered_snow_shortwave_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
        match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => {
                covered_snow_shortwave_from_multi_tile(value)
            }
            CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } => {
                covered_snow_shortwave_from_frozen_litter_v3(fixed)
            }
            CoveredV8PhysicalOwner::Legacy(_) => Err(CoveredV8OwnerEnvelopeError::Identity(
                "physical-only shortwave requires multi-tile owner",
            )),
        }
    }

    /// Consume the final map's physical prefix and construct its one complete
    /// vegetation/BGC envelope.  Iteration maps retain this value behind a
    /// private move-only carrier result and therefore have no route to this
    /// continuation.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn into_complete_owner_envelope_v11(
        self,
        bindings: &[V8ComponentOccupancyBinding],
        vegetation_configuration: &VegetationConfiguration,
        vegetation_beginning: &V8CoupledOwnedState,
        persistent_forcing: &V8PersistentForcingReceipt,
        nitrogen: &dyn NitrogenArbiter,
        biogeochemistry_beginning: &BiogeochemistryState,
        duration_s_bits: u64,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
        canonical_covered_final_v8_receipt_boundary_v1();
        let final_constructor_hook = |phase| canonical_covered_final_owner_boundary_v1(phase);
        match self.physical {
            CoveredV8PhysicalOwner::MultiTile(physical) => {
                let potentials = physical
                    .potential_tiles()
                    .iter()
                    .filter_map(|value| value.covered())
                    .map(|value| &value.potential_vegetation_operands)
                    .collect::<Vec<_>>();
                let finals = physical
                    .finalized_tiles()
                    .iter()
                    .filter_map(|value| value.covered())
                    .map(|value| &value.vegetation_operands)
                    .collect::<Vec<_>>();
                let projected = project_multi_tile_v8_passes_v11(
                    &potentials,
                    &finals,
                    bindings,
                    physical.hydrology_candidate(),
                    vegetation_configuration,
                    vegetation_beginning,
                    duration_s_bits,
                )?;
                construct_multi_tile_v8_owner_envelope_v11(
                    physical,
                    &projected,
                    vegetation_configuration,
                    vegetation_beginning,
                    persistent_forcing,
                    nitrogen,
                    biogeochemistry_beginning,
                    Some(&final_constructor_hook),
                    duration_s_bits,
                )
            }
            CoveredV8PhysicalOwner::FrozenLitterV3 { hydrology, fixed } => {
                construct_frozen_litter_v3_owner_envelope_v11_with_failure_hook(
                    &fixed,
                    hydrology,
                    vegetation_configuration,
                    vegetation_beginning,
                    persistent_forcing,
                    nitrogen,
                    biogeochemistry_beginning,
                    Some(&final_constructor_hook),
                    duration_s_bits,
                )
            }
            CoveredV8PhysicalOwner::Legacy(_) => Err(CoveredV8OwnerEnvelopeError::Identity(
                "final covered physical prefix requires multi-tile owner",
            )),
        }
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CanonicalCoveredPrivatePhysicalProjectionV1 {
    pub(crate) sha256: openwepp_coupled_time::Digest32,
    pub(crate) potential_tile_count: usize,
    pub(crate) finalized_tile_count: usize,
    pub(crate) weighted_ofe_count: usize,
    pub(crate) native_tile_count: usize,
    pub(crate) lse_destination_count: usize,
    pub(crate) release_destination_count: usize,
}

fn fixed_cap_canopy_releases_from_multi_tile(
    physical: &MultiTileRuntimeResult,
    interval_s: f64,
) -> Result<FixedCapCanopyReleasesByDestination, CoveredV8OwnerEnvelopeError> {
    fixed_cap_canopy_releases_from_finalized_tiles(physical.finalized_tiles(), interval_s)
}

fn fixed_cap_canopy_releases_from_finalized_tiles(
    finalized_tiles: &[super::multi_tile_runtime::FinalizedRuntimeTile],
    interval_s: f64,
) -> Result<FixedCapCanopyReleasesByDestination, CoveredV8OwnerEnvelopeError> {
    let mut releases = BTreeMap::new();
    for tile in finalized_tiles {
        let Some(covered) = tile.covered() else {
            continue;
        };
        covered.vegetation_operands.validate()?;
        let release = super::covered_derived_ingress::derive_release_from_ledgers(
            covered
                .vegetation_operands
                .occupancies
                .iter()
                .map(|row| (row.occupancy_id.as_str(), &row.liquid)),
            covered
                .vegetation_operands
                .ground_canopy_release_kg_m2_tile_ground,
            covered
                .vegetation_operands
                .ground_stemflow_kg_m2_tile_ground,
            interval_s,
        )
        .map_err(|_| CoveredV8OwnerEnvelopeError::Identity("fixed-cap release reconstruction"))?;
        let source_identity = openwepp_coupled_time::digest_bytes(
            &serde_json::to_vec(&covered.vegetation_operands).map_err(|_| {
                CoveredV8OwnerEnvelopeError::Identity("fixed-cap release source framing")
            })?,
        );
        let destination = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        if releases
            .insert(destination, (release, source_identity))
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate fixed-cap release destination",
            ));
        }
    }
    Ok(releases)
}

fn fixed_cap_canopy_releases_from_frozen_litter_v3(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
    interval_s: f64,
) -> Result<FixedCapCanopyReleasesByDestination, CoveredV8OwnerEnvelopeError> {
    let mut releases =
        fixed_cap_canopy_releases_from_finalized_tiles(&fixed.legacy_tiles, interval_s)?;
    for tile in &fixed.frozen_litter_tiles {
        validate_frozen_litter_v3_tile_identity(fixed, tile)?;
        let covered = &tile.fixed_final.complete_physical_candidate;
        covered.vegetation_operands.validate()?;
        let release = super::covered_derived_ingress::derive_release_from_ledgers(
            covered
                .vegetation_operands
                .occupancies
                .iter()
                .map(|row| (row.occupancy_id.as_str(), &row.liquid)),
            covered
                .vegetation_operands
                .ground_canopy_release_kg_m2_tile_ground,
            covered
                .vegetation_operands
                .ground_stemflow_kg_m2_tile_ground,
            interval_s,
        )
        .map_err(|_| CoveredV8OwnerEnvelopeError::Identity("fixed-cap release reconstruction"))?;
        let source_identity = openwepp_coupled_time::digest_bytes(
            &serde_json::to_vec(&covered.vegetation_operands).map_err(|_| {
                CoveredV8OwnerEnvelopeError::Identity("fixed-cap release source framing")
            })?,
        );
        let destination = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        if releases
            .insert(destination, (release, source_identity))
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate fixed-cap release destination",
            ));
        }
    }
    Ok(releases)
}

fn covered_lse_iteration_states_from_multi_tile(
    physical: &MultiTileRuntimeResult,
) -> Result<BTreeMap<(OfeId, TileId), CoveredLseIterationState>, CoveredV8OwnerEnvelopeError> {
    covered_lse_iteration_states_from_finalized_tiles(physical.finalized_tiles())
}

fn covered_lse_iteration_states_from_finalized_tiles(
    finalized_tiles: &[super::multi_tile_runtime::FinalizedRuntimeTile],
) -> Result<BTreeMap<(OfeId, TileId), CoveredLseIterationState>, CoveredV8OwnerEnvelopeError> {
    let mut states = BTreeMap::new();
    for tile in finalized_tiles {
        let Some(covered) = tile.covered() else {
            continue;
        };
        let lower = match &covered.energy_operands.lower_boundary {
            CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(value) => value,
            CoveredLowerBoundaryEnergyOperands::SnowFree(_) => continue,
        };
        let column = &covered.energy_operands.column;
        let component_temperatures_k = column
            .occupancies
            .iter()
            .map(|occupancy| {
                (
                    occupancy.occupancy_id.clone(),
                    [
                        occupancy.sun_leaf.surface_temperature_k,
                        occupancy.shade_leaf.surface_temperature_k,
                        occupancy.wet_surface.surface_temperature_k,
                        occupancy.dry_stem.surface_temperature_k,
                    ],
                )
            })
            .collect();
        let component_carrier_surfaces = column
            .occupancies
            .iter()
            .enumerate()
            .flat_map(|(vertical_ordinal, occupancy)| {
                [
                    occupancy.sun_leaf,
                    occupancy.shade_leaf,
                    occupancy.wet_surface,
                    occupancy.dry_stem,
                ]
                .into_iter()
                .enumerate()
                .map(|(ordinal, surface)| CoveredCarrierComponentState {
                    vertical_occupancy_ordinal: vertical_ordinal as u32,
                    occupancy_id: occupancy.occupancy_id.clone(),
                    component_ordinal: ordinal as u8,
                    surface_area_m2_m2_tile: surface.surface_area_m2_m2_tile,
                    emissive_area_m2_m2_tile: surface.emissive_area_m2_m2_tile,
                    heat_conductance_m_s_tile: surface.heat_conductance_m_s_tile,
                    vapor_conductance_m_s_tile: surface.vapor_conductance_m_s_tile,
                    vapor_authorization_kg_m2_tile_s: surface.vapor_authorization_kg_m2_tile_s,
                    temperature_k: surface.surface_temperature_k,
                    specific_humidity_kg_kg: surface.surface_specific_humidity_kg_kg,
                    sensible_to_canopy_air_w_m2: surface.sensible_to_canopy_air_w_m2_tile,
                    vapor_to_canopy_air_kg_m2_s: surface.signed_vapor_to_canopy_air_kg_m2_tile_s,
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let key = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        if states
            .insert(
                key,
                CoveredLseIterationState {
                    canopy_air_temperature_k: column.canopy_air.canopy_air_temperature_k,
                    canopy_air_specific_humidity_kg_kg: column
                        .canopy_air
                        .canopy_air_specific_humidity_kg_kg,
                    snow_temperature_k: lower.snow_temperature_k,
                    snow_sensible_w_m2: column.canopy_air.ground_sensible_to_canopy_air_w_m2_tile,
                    snow_vapor_kg_m2_s: column.canopy_air.ground_vapor_to_canopy_air_kg_m2_tile_s,
                    snow_latent_w_m2: column.canopy_air.ground_vapor_to_canopy_air_kg_m2_tile_s
                        * lower.latent_heat_j_kg,
                    snow_net_longwave_w_m2: column.longwave.ground_net_w_m2_tile,
                    component_temperatures_k,
                    component_carrier_surfaces,
                    canopy_sensible_w_m2: column.canopy_air.canopy_sensible_w_m2_tile,
                    canopy_vapor_kg_m2_s: column.canopy_air.canopy_vapor_kg_m2_tile_s,
                    sensible_to_reference_air_w_m2: column
                        .canopy_air
                        .sensible_to_reference_air_w_m2_tile,
                    vapor_to_reference_air_kg_m2_s: column
                        .canopy_air
                        .vapor_to_reference_air_kg_m2_tile_s,
                    shared_heat_residual_w_m2: column.canopy_air.shared_heat_residual_w_m2_tile,
                    shared_heat_tolerance_w_m2: column.canopy_air.shared_heat_tolerance_w_m2_tile,
                    shared_vapor_residual_kg_m2_s: column
                        .canopy_air
                        .shared_vapor_residual_kg_m2_tile_s,
                    shared_vapor_tolerance_kg_m2_s: column
                        .canopy_air
                        .shared_vapor_tolerance_kg_m2_tile_s,
                },
            )
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered iteration destination",
            ));
        }
    }
    Ok(states)
}

fn validate_frozen_litter_v3_tile_identity(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
    tile: &super::v3_multitile_adoption::AcceptedV3ForestLitterTile,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    let candidate = &tile.fixed_final;
    let complete = &candidate.complete_physical_candidate;
    let transaction_id = fixed.water_protocol.transaction_id;
    if candidate.transaction_id != transaction_id
        || candidate.identity.transaction_id != transaction_id
        || candidate.water_protocol.transaction_id != transaction_id
        || complete.transaction_id != transaction_id
        || complete.identity.transaction_id != transaction_id
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "frozen-litter V3 retained physical transaction",
        ));
    }
    if candidate.identity != complete.identity {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "frozen-litter V3 fixed/complete runtime identity",
        ));
    }
    if candidate.identity.ofe_id != tile.phase_free_input.ofe_id
        || candidate.identity.tile_id != tile.phase_free_input.tile_id
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "frozen-litter V3 phase-free destination identity",
        ));
    }
    Ok(())
}

fn validate_frozen_litter_v3_fixed_identity(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
    hydrology: &UnifiedRealHydrologyCandidate,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    let transaction_id = fixed.water_protocol.transaction_id;
    if transaction_id != hydrology.transaction_id()
        || (fixed.frozen_litter_tiles.is_empty() && fixed.stage3_covered_native_tiles.is_empty())
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "frozen-litter V3 retained batch transaction",
        ));
    }
    for tile in &fixed.legacy_tiles {
        let transaction_id = match tile {
            super::multi_tile_runtime::FinalizedRuntimeTile::Open(value) => {
                value.identity.transaction_id
            }
            super::multi_tile_runtime::FinalizedRuntimeTile::Stage3OpenSnow {
                identity, ..
            } => identity.transaction_id,
            super::multi_tile_runtime::FinalizedRuntimeTile::Covered(value) => {
                value.identity.transaction_id
            }
        };
        if transaction_id != fixed.water_protocol.transaction_id
            || tile.water_protocol().is_some_and(|protocol| {
                protocol.transaction_id != fixed.water_protocol.transaction_id
            })
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "frozen-litter V3 retained legacy transaction",
            ));
        }
    }
    for tile in &fixed.frozen_litter_tiles {
        validate_frozen_litter_v3_tile_identity(fixed, tile)?;
    }
    for native in &fixed.stage3_covered_native_tiles {
        let matching = fixed.legacy_tiles.iter().filter_map(|tile| {
            let super::multi_tile_runtime::FinalizedRuntimeTile::Covered(covered) = tile else {
                return None;
            };
            (covered.identity == native.identity).then_some(covered)
        });
        let rows = matching.collect::<Vec<_>>();
        let expected_optical = native.covered_beginning.stage3_optical.as_ref().ok_or(
            CoveredV8OwnerEnvelopeError::Identity(
                "Stage3CoveredNative missing retained optical receipt",
            ),
        )?;
        let expected_lower = native
            .covered_beginning
            .stage3_lower_boundary
            .as_ref()
            .ok_or(CoveredV8OwnerEnvelopeError::Identity(
                "Stage3CoveredNative missing retained lower boundary",
            ))?;
        let Some(covered) = rows.first() else {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "Stage3CoveredNative standard covered solve cardinality",
            ));
        };
        let CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(accepted_lower) =
            &covered.energy_operands.lower_boundary
        else {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "Stage3CoveredNative accepted lower-boundary posture",
            ));
        };
        if rows.len() != 1
            || accepted_lower.optical != *expected_optical
            || accepted_lower.optical_receipt_sha256 != expected_lower.optical_receipt_sha256
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "Stage3CoveredNative exact standard physical receipt join",
            ));
        }
    }
    Ok(())
}

fn frozen_litter_v3_stage3_boundary<'a>(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
    tile: &'a super::v3_multitile_adoption::AcceptedV3ForestLitterTile,
) -> Result<
    (
        &'a Stage3SnowCoveredLowerBoundary,
        &'a Stage3SnowOpticalBoundaryReceiptV1,
    ),
    CoveredV8OwnerEnvelopeError,
> {
    validate_frozen_litter_v3_tile_identity(fixed, tile)?;
    let complete = &tile.fixed_final.complete_physical_candidate;
    let lower = tile
        .covered_beginning
        .stage3_lower_boundary
        .as_ref()
        .ok_or(CoveredV8OwnerEnvelopeError::Identity(
            "frozen-litter V3 missing Stage-3 lower boundary",
        ))?;
    let optical = tile.covered_beginning.stage3_optical.as_ref().ok_or(
        CoveredV8OwnerEnvelopeError::Identity("frozen-litter V3 missing Stage-3 optical receipt"),
    )?;
    optical.validate()?;
    if optical.ofe_id != complete.identity.ofe_id
        || optical.tile_id != complete.identity.tile_id
        || lower.optical_receipt_sha256.as_ref() != Some(&optical.receipt_sha256)
        || lower.snow_vis_albedo.to_bits() != optical.snow_vis_albedo.to_bits()
        || lower.snow_nir_albedo.to_bits() != optical.snow_nir_albedo.to_bits()
        || lower.stage3_albedo_state_sha256 != optical.stage3_albedo_state_sha256
        || lower.forcing_receipt_sha256 != optical.forcing_receipt_sha256
        || lower.shortwave_absorbed_w_m2.to_bits() != optical.absorbed_w_m2_tile.total().to_bits()
        || complete.energy_operands.shortwave.ground_terminal_w_m2_tile
            != optical.terminal_w_m2_tile
        || complete.energy_operands.shortwave.ground_absorbed_w_m2_tile
            != optical.absorbed_w_m2_tile
        || complete
            .energy_operands
            .shortwave
            .ground_reflected_w_m2_tile
            != optical.reflected_w_m2_tile
        || complete
            .energy_operands
            .longwave
            .ground_net_w_m2_tile
            .to_bits()
            != lower.net_longwave_w_m2.to_bits()
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "frozen-litter V3 Stage-3 boundary/physical receipt join",
        ));
    }
    Ok((lower, optical))
}

fn frozen_litter_v3_iteration_state(
    occupancies: &[CoveredOccupancyEnergyOperands],
    canopy_air: CoveredCanopyAirEnergyOperands,
    lower: &Stage3SnowCoveredLowerBoundary,
    snow_net_longwave_w_m2: f64,
) -> CoveredLseIterationState {
    let component_temperatures_k = occupancies
        .iter()
        .map(|occupancy| {
            (
                occupancy.occupancy_id.clone(),
                [
                    occupancy.sun_leaf.surface_temperature_k,
                    occupancy.shade_leaf.surface_temperature_k,
                    occupancy.wet_surface.surface_temperature_k,
                    occupancy.dry_stem.surface_temperature_k,
                ],
            )
        })
        .collect();
    let component_carrier_surfaces = occupancies
        .iter()
        .enumerate()
        .flat_map(|(vertical_ordinal, occupancy)| {
            [
                occupancy.sun_leaf,
                occupancy.shade_leaf,
                occupancy.wet_surface,
                occupancy.dry_stem,
            ]
            .into_iter()
            .enumerate()
            .map(move |(ordinal, surface)| CoveredCarrierComponentState {
                vertical_occupancy_ordinal: vertical_ordinal as u32,
                occupancy_id: occupancy.occupancy_id.clone(),
                component_ordinal: ordinal as u8,
                surface_area_m2_m2_tile: surface.surface_area_m2_m2_tile,
                emissive_area_m2_m2_tile: surface.emissive_area_m2_m2_tile,
                heat_conductance_m_s_tile: surface.heat_conductance_m_s_tile,
                vapor_conductance_m_s_tile: surface.vapor_conductance_m_s_tile,
                vapor_authorization_kg_m2_tile_s: surface.vapor_authorization_kg_m2_tile_s,
                temperature_k: surface.surface_temperature_k,
                specific_humidity_kg_kg: surface.surface_specific_humidity_kg_kg,
                sensible_to_canopy_air_w_m2: surface.sensible_to_canopy_air_w_m2_tile,
                vapor_to_canopy_air_kg_m2_s: surface.signed_vapor_to_canopy_air_kg_m2_tile_s,
            })
            .collect::<Vec<_>>()
        })
        .collect();
    CoveredLseIterationState {
        canopy_air_temperature_k: canopy_air.canopy_air_temperature_k,
        canopy_air_specific_humidity_kg_kg: canopy_air.canopy_air_specific_humidity_kg_kg,
        snow_temperature_k: lower.snow_temperature_k,
        snow_sensible_w_m2: canopy_air.ground_sensible_to_canopy_air_w_m2_tile,
        snow_vapor_kg_m2_s: canopy_air.ground_vapor_to_canopy_air_kg_m2_tile_s,
        snow_latent_w_m2: canopy_air.ground_vapor_to_canopy_air_kg_m2_tile_s
            * lower.latent_heat_j_kg,
        snow_net_longwave_w_m2,
        component_temperatures_k,
        component_carrier_surfaces,
        canopy_sensible_w_m2: canopy_air.canopy_sensible_w_m2_tile,
        canopy_vapor_kg_m2_s: canopy_air.canopy_vapor_kg_m2_tile_s,
        sensible_to_reference_air_w_m2: canopy_air.sensible_to_reference_air_w_m2_tile,
        vapor_to_reference_air_kg_m2_s: canopy_air.vapor_to_reference_air_kg_m2_tile_s,
        shared_heat_residual_w_m2: canopy_air.shared_heat_residual_w_m2_tile,
        shared_heat_tolerance_w_m2: canopy_air.shared_heat_tolerance_w_m2_tile,
        shared_vapor_residual_kg_m2_s: canopy_air.shared_vapor_residual_kg_m2_tile_s,
        shared_vapor_tolerance_kg_m2_s: canopy_air.shared_vapor_tolerance_kg_m2_tile_s,
    }
}

fn covered_lse_iteration_states_from_frozen_litter_v3(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
) -> Result<BTreeMap<(OfeId, TileId), CoveredLseIterationState>, CoveredV8OwnerEnvelopeError> {
    let mut states = covered_lse_iteration_states_from_finalized_tiles(&fixed.legacy_tiles)?;
    for tile in &fixed.frozen_litter_tiles {
        let (lower, _) = frozen_litter_v3_stage3_boundary(fixed, tile)?;
        let complete = &tile.fixed_final.complete_physical_candidate;
        let key = (
            complete.identity.ofe_id.clone(),
            complete.identity.tile_id.clone(),
        );
        let state = frozen_litter_v3_iteration_state(
            &complete.energy_operands.occupancies,
            complete.energy_operands.canopy_air,
            lower,
            complete.energy_operands.longwave.ground_net_w_m2_tile,
        );
        if states.insert(key, state).is_some() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered iteration destination",
            ));
        }
    }
    Ok(states)
}

fn covered_snow_longwave_from_multi_tile(
    physical: &MultiTileRuntimeResult,
) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
    covered_snow_longwave_from_finalized_tiles(physical.finalized_tiles())
}

fn covered_snow_longwave_from_finalized_tiles(
    finalized_tiles: &[super::multi_tile_runtime::FinalizedRuntimeTile],
) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
    let mut receipts = BTreeMap::new();
    for tile in finalized_tiles {
        let Some(covered) = tile.covered() else {
            continue;
        };
        let key = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        let value = covered.energy_operands.column.longwave.ground_net_w_m2_tile;
        if receipts.insert(key, value).is_some() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered longwave destination",
            ));
        }
    }
    if receipts.is_empty() {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "empty covered longwave destination set",
        ));
    }
    Ok(receipts)
}

fn covered_snow_shortwave_from_multi_tile(
    physical: &MultiTileRuntimeResult,
) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
    covered_snow_shortwave_from_finalized_tiles(physical.finalized_tiles())
}

fn covered_snow_shortwave_from_finalized_tiles(
    finalized_tiles: &[super::multi_tile_runtime::FinalizedRuntimeTile],
) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
    let mut receipts = BTreeMap::new();
    for tile in finalized_tiles {
        let Some(covered) = tile.covered() else {
            continue;
        };
        let key = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        let value = match &covered.energy_operands.lower_boundary {
            CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) => {
                stage3.optical.absorbed_w_m2_tile.total()
            }
            CoveredLowerBoundaryEnergyOperands::SnowFree(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered Stage-3 optical receipt for shortwave",
                ));
            }
        };
        if receipts.insert(key, value).is_some() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered shortwave destination",
            ));
        }
    }
    if receipts.is_empty() {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "empty covered shortwave destination set",
        ));
    }
    Ok(receipts)
}

fn covered_snow_longwave_from_frozen_litter_v3(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
    let mut receipts = BTreeMap::new();
    for tile in &fixed.legacy_tiles {
        let Some(covered) = tile.covered() else {
            continue;
        };
        let key = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        let value = covered.energy_operands.column.longwave.ground_net_w_m2_tile;
        if receipts.insert(key, value).is_some() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered longwave destination",
            ));
        }
    }
    for tile in &fixed.frozen_litter_tiles {
        frozen_litter_v3_stage3_boundary(fixed, tile)?;
        let complete = &tile.fixed_final.complete_physical_candidate;
        let key = (
            complete.identity.ofe_id.clone(),
            complete.identity.tile_id.clone(),
        );
        let value = complete.energy_operands.longwave.ground_net_w_m2_tile;
        if receipts.insert(key, value).is_some() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered longwave destination",
            ));
        }
    }
    if receipts.is_empty() {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "empty covered longwave destination set",
        ));
    }
    Ok(receipts)
}

fn covered_snow_shortwave_from_frozen_litter_v3(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
    let mut receipts = BTreeMap::new();
    for tile in &fixed.legacy_tiles {
        let Some(covered) = tile.covered() else {
            continue;
        };
        let optical = match &covered.energy_operands.lower_boundary {
            CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) => &stage3.optical,
            CoveredLowerBoundaryEnergyOperands::SnowFree(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered Stage-3 optical receipt for shortwave",
                ));
            }
        };
        optical.validate()?;
        let key = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        if receipts
            .insert(key, optical.absorbed_w_m2_tile.total())
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered shortwave destination",
            ));
        }
    }
    for tile in &fixed.frozen_litter_tiles {
        let (_, optical) = frozen_litter_v3_stage3_boundary(fixed, tile)?;
        let key = (optical.ofe_id.clone(), optical.tile_id.clone());
        if receipts
            .insert(key, optical.absorbed_w_m2_tile.total())
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered shortwave destination",
            ));
        }
    }
    if receipts.is_empty() {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "empty covered shortwave destination set",
        ));
    }
    Ok(receipts)
}

fn covered_snow_optical_from_frozen_litter_v3(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
) -> Result<
    BTreeMap<(OfeId, TileId), Stage3SnowOpticalBoundaryReceiptV1>,
    CoveredV8OwnerEnvelopeError,
> {
    let mut receipts = BTreeMap::new();
    for tile in &fixed.legacy_tiles {
        let Some(covered) = tile.covered() else {
            continue;
        };
        let optical = match &covered.energy_operands.lower_boundary {
            CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) => stage3.optical.clone(),
            CoveredLowerBoundaryEnergyOperands::SnowFree(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered optical receipt for snow-free payload",
                ));
            }
        };
        optical.validate()?;
        let key = (
            covered.identity.ofe_id.clone(),
            covered.identity.tile_id.clone(),
        );
        if receipts.insert(key, optical).is_some() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered optical receipt",
            ));
        }
    }
    for tile in &fixed.frozen_litter_tiles {
        let (_, optical) = frozen_litter_v3_stage3_boundary(fixed, tile)?;
        let key = (optical.ofe_id.clone(), optical.tile_id.clone());
        if receipts.insert(key, optical.clone()).is_some() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate covered optical receipt",
            ));
        }
    }
    if receipts.is_empty() {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "empty covered optical receipt set",
        ));
    }
    Ok(receipts)
}

impl CoveredV8PhysicalOwner {
    fn hydrology(&self) -> &UnifiedRealHydrologyCandidate {
        match self {
            Self::Legacy(value) => value.hydrology_candidate(),
            Self::MultiTile(value) => value.hydrology_candidate(),
            Self::FrozenLitterV3 { hydrology, .. } => hydrology,
        }
    }
}

impl UncommittedCoveredV8OwnerEnvelope {
    #[must_use]
    pub const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub const fn vegetation(&self) -> &UncommittedV8VegetationCandidate {
        &self.vegetation
    }

    #[must_use]
    pub fn hydrology(&self) -> &UnifiedRealHydrologyCandidate {
        self.physical.hydrology()
    }

    pub(crate) fn fixed_cap_canopy_releases_by_destination(
        &self,
        interval_s: f64,
    ) -> Result<FixedCapCanopyReleasesByDestination, CoveredV8OwnerEnvelopeError> {
        match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => {
                fixed_cap_canopy_releases_from_multi_tile(value, interval_s)
            }
            CoveredV8PhysicalOwner::Legacy(_) => Err(CoveredV8OwnerEnvelopeError::Identity(
                "fixed-cap release requires multi-tile physical owner",
            )),
            CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } => {
                fixed_cap_canopy_releases_from_frozen_litter_v3(fixed, interval_s)
            }
        }
    }

    pub(crate) fn covered_lse_iteration_state_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), CoveredLseIterationState>, CoveredV8OwnerEnvelopeError>
    {
        if let CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } = &self.physical {
            return covered_lse_iteration_states_from_frozen_litter_v3(fixed);
        }
        let physical = match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => value,
            CoveredV8PhysicalOwner::Legacy(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered iteration state requires multi-tile physical owner",
                ));
            }
            CoveredV8PhysicalOwner::FrozenLitterV3 { .. } => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered iteration state is unavailable after V3 finalization",
                ));
            }
        };
        let mut states = BTreeMap::new();
        for tile in physical.finalized_tiles() {
            let Some(covered) = tile.covered() else {
                continue;
            };
            let lower = match &covered.energy_operands.lower_boundary {
                CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(value) => value,
                CoveredLowerBoundaryEnergyOperands::SnowFree(_) => continue,
            };
            let column = &covered.energy_operands.column;
            let component_temperatures_k = column
                .occupancies
                .iter()
                .map(|occupancy| {
                    (
                        occupancy.occupancy_id.clone(),
                        [
                            occupancy.sun_leaf.surface_temperature_k,
                            occupancy.shade_leaf.surface_temperature_k,
                            occupancy.wet_surface.surface_temperature_k,
                            occupancy.dry_stem.surface_temperature_k,
                        ],
                    )
                })
                .collect();
            let component_carrier_surfaces = column
                .occupancies
                .iter()
                .enumerate()
                .flat_map(|(vertical_ordinal, occupancy)| {
                    [
                        occupancy.sun_leaf,
                        occupancy.shade_leaf,
                        occupancy.wet_surface,
                        occupancy.dry_stem,
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(ordinal, surface)| CoveredCarrierComponentState {
                        vertical_occupancy_ordinal: vertical_ordinal as u32,
                        occupancy_id: occupancy.occupancy_id.clone(),
                        component_ordinal: ordinal as u8,
                        surface_area_m2_m2_tile: surface.surface_area_m2_m2_tile,
                        emissive_area_m2_m2_tile: surface.emissive_area_m2_m2_tile,
                        heat_conductance_m_s_tile: surface.heat_conductance_m_s_tile,
                        vapor_conductance_m_s_tile: surface.vapor_conductance_m_s_tile,
                        vapor_authorization_kg_m2_tile_s: surface.vapor_authorization_kg_m2_tile_s,
                        temperature_k: surface.surface_temperature_k,
                        specific_humidity_kg_kg: surface.surface_specific_humidity_kg_kg,
                        sensible_to_canopy_air_w_m2: surface.sensible_to_canopy_air_w_m2_tile,
                        vapor_to_canopy_air_kg_m2_s: surface
                            .signed_vapor_to_canopy_air_kg_m2_tile_s,
                    })
                    .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let key = (
                covered.identity.ofe_id.clone(),
                covered.identity.tile_id.clone(),
            );
            if states
                .insert(
                    key,
                    CoveredLseIterationState {
                        canopy_air_temperature_k: column.canopy_air.canopy_air_temperature_k,
                        canopy_air_specific_humidity_kg_kg: column
                            .canopy_air
                            .canopy_air_specific_humidity_kg_kg,
                        snow_temperature_k: lower.snow_temperature_k,
                        snow_sensible_w_m2: column
                            .canopy_air
                            .ground_sensible_to_canopy_air_w_m2_tile,
                        snow_vapor_kg_m2_s: column
                            .canopy_air
                            .ground_vapor_to_canopy_air_kg_m2_tile_s,
                        snow_latent_w_m2: column.canopy_air.ground_vapor_to_canopy_air_kg_m2_tile_s
                            * lower.latent_heat_j_kg,
                        snow_net_longwave_w_m2: column.longwave.ground_net_w_m2_tile,
                        component_temperatures_k,
                        component_carrier_surfaces,
                        canopy_sensible_w_m2: column.canopy_air.canopy_sensible_w_m2_tile,
                        canopy_vapor_kg_m2_s: column.canopy_air.canopy_vapor_kg_m2_tile_s,
                        sensible_to_reference_air_w_m2: column
                            .canopy_air
                            .sensible_to_reference_air_w_m2_tile,
                        vapor_to_reference_air_kg_m2_s: column
                            .canopy_air
                            .vapor_to_reference_air_kg_m2_tile_s,
                        shared_heat_residual_w_m2: column.canopy_air.shared_heat_residual_w_m2_tile,
                        shared_heat_tolerance_w_m2: column
                            .canopy_air
                            .shared_heat_tolerance_w_m2_tile,
                        shared_vapor_residual_kg_m2_s: column
                            .canopy_air
                            .shared_vapor_residual_kg_m2_tile_s,
                        shared_vapor_tolerance_kg_m2_s: column
                            .canopy_air
                            .shared_vapor_tolerance_kg_m2_tile_s,
                    },
                )
                .is_some()
            {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "duplicate covered iteration destination",
                ));
            }
        }
        Ok(states)
    }

    #[must_use]
    pub const fn biogeochemistry(&self) -> &BiogeochemistryOwnerCandidate {
        &self.biogeochemistry
    }

    pub(crate) fn covered_snow_longwave_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
        if let CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } = &self.physical {
            return covered_snow_longwave_from_frozen_litter_v3(fixed);
        }
        let physical = match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => value,
            CoveredV8PhysicalOwner::Legacy(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered longwave requires multi-tile physical owner",
                ));
            }
            CoveredV8PhysicalOwner::FrozenLitterV3 { .. } => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered longwave is unavailable after V3 finalization",
                ));
            }
        };
        let mut receipts = BTreeMap::new();
        for tile in physical.finalized_tiles() {
            let Some(covered) = tile.covered() else {
                continue;
            };
            let key = (
                covered.identity.ofe_id.clone(),
                covered.identity.tile_id.clone(),
            );
            let value = covered.energy_operands.column.longwave.ground_net_w_m2_tile;
            if receipts.insert(key, value).is_some() {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "duplicate covered longwave destination",
                ));
            }
        }
        if receipts.is_empty() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "empty covered longwave destination set",
            ));
        }
        Ok(receipts)
    }

    pub(crate) fn covered_snow_shortwave_by_destination(
        &self,
    ) -> Result<BTreeMap<(OfeId, TileId), f64>, CoveredV8OwnerEnvelopeError> {
        if let CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } = &self.physical {
            return covered_snow_shortwave_from_frozen_litter_v3(fixed);
        }
        let physical = match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => value,
            CoveredV8PhysicalOwner::Legacy(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered shortwave requires multi-tile physical owner",
                ));
            }
            CoveredV8PhysicalOwner::FrozenLitterV3 { .. } => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered shortwave is unavailable after V3 finalization",
                ));
            }
        };
        let mut receipts = BTreeMap::new();
        for tile in physical.finalized_tiles() {
            let Some(covered) = tile.covered() else {
                continue;
            };
            let key = (
                covered.identity.ofe_id.clone(),
                covered.identity.tile_id.clone(),
            );
            let value = match &covered.energy_operands.lower_boundary {
                CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) => {
                    stage3.optical.absorbed_w_m2_tile.total()
                }
                CoveredLowerBoundaryEnergyOperands::SnowFree(_) => {
                    return Err(CoveredV8OwnerEnvelopeError::Identity(
                        "covered Stage-3 optical receipt for shortwave",
                    ));
                }
            };
            if receipts.insert(key, value).is_some() {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "duplicate covered shortwave destination",
                ));
            }
        }
        if receipts.is_empty() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "empty covered shortwave destination set",
            ));
        }
        Ok(receipts)
    }

    pub(crate) fn covered_snow_optical_by_destination(
        &self,
    ) -> Result<
        BTreeMap<(OfeId, TileId), Stage3SnowOpticalBoundaryReceiptV1>,
        CoveredV8OwnerEnvelopeError,
    > {
        if let CoveredV8PhysicalOwner::FrozenLitterV3 { fixed, .. } = &self.physical {
            return covered_snow_optical_from_frozen_litter_v3(fixed);
        }
        let physical = match &self.physical {
            CoveredV8PhysicalOwner::MultiTile(value) => value,
            CoveredV8PhysicalOwner::Legacy(_) => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered optical receipt requires multi-tile physical owner",
                ));
            }
            CoveredV8PhysicalOwner::FrozenLitterV3 { .. } => {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "covered optical receipt is unavailable after V3 finalization",
                ));
            }
        };
        let mut receipts = BTreeMap::new();
        for tile in physical.finalized_tiles() {
            let Some(covered) = tile.covered() else {
                continue;
            };
            let optical = match &covered.energy_operands.lower_boundary {
                CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) => {
                    stage3.optical.clone()
                }
                CoveredLowerBoundaryEnergyOperands::SnowFree(_) => {
                    return Err(CoveredV8OwnerEnvelopeError::Identity(
                        "covered optical receipt for snow-free payload",
                    ));
                }
            };
            let key = (
                covered.identity.ofe_id.clone(),
                covered.identity.tile_id.clone(),
            );
            if receipts.insert(key, optical).is_some() {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "duplicate covered optical receipt",
                ));
            }
        }
        if receipts.is_empty() {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "empty covered optical receipt set",
            ));
        }
        Ok(receipts)
    }

    pub fn validate(&self) -> Result<(), CoveredV8OwnerEnvelopeError> {
        self.vegetation.validate_sealed()?;
        self.biogeochemistry.validate()?;
        validate_owner_transaction_identity(
            self.transaction_id,
            self.vegetation.transaction_id(),
            self.physical.hydrology().transaction_id(),
            self.biogeochemistry.transaction_id(),
        )?;
        compare_material_receipts(&self.vegetation, &self.biogeochemistry)
    }
}

fn validate_owner_transaction_identity(
    envelope: TransactionId,
    vegetation: TransactionId,
    physical: TransactionId,
    biogeochemistry: TransactionId,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    if envelope != vegetation || envelope != physical || envelope != biogeochemistry {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "heterogeneous transaction identity",
        ));
    }
    Ok(())
}

/// Construct BGC independently from the sealed vegetation protocol and join it
/// to the already executed physical owners. No water authorization, physical
/// solve, vegetation persistence, or owner mutation is reachable here.
#[allow(clippy::too_many_arguments)]
pub(crate) fn construct_covered_v8_owner_envelope(
    physical: CoveredForestShadowResult,
    bindings: &[V8ComponentOccupancyBinding],
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    let projected = project_covered_forest_v8_passes(
        &physical,
        bindings,
        vegetation_configuration,
        vegetation_beginning,
    )?;
    let persistent = execute_uncommitted_v8_persistent_phase(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        persistent_forcing,
        nitrogen,
    )?;
    let vegetation = construct_uncommitted_v8_vegetation_candidate(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        projected.final_state(),
        &persistent,
    )?;
    validate_actual_root_protocol(
        &physical.final_tile().vegetation_operands,
        physical.hydrology_candidate(),
    )?;
    join_covered_v8_owner_envelope(
        CoveredV8PhysicalOwner::Legacy(physical),
        vegetation,
        biogeochemistry_beginning,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn construct_multi_tile_v8_owner_envelope(
    physical: MultiTileRuntimeResult,
    projected: &V8CoveredProjection,
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
    failure_hook: OwnerFailureHook<'_>,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::Persistent)?;
    let persistent = execute_uncommitted_v8_persistent_phase(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        persistent_forcing,
        nitrogen,
    )?;
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::VegetationCandidate)?;
    let vegetation = construct_uncommitted_v8_vegetation_candidate(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        projected.final_state(),
        &persistent,
    )?;
    join_covered_v8_owner_envelope(
        CoveredV8PhysicalOwner::MultiTile(physical),
        vegetation,
        biogeochemistry_beginning,
        failure_hook,
    )
}

/// V11-only owner construction over the authenticated common-slab duration.
#[allow(clippy::too_many_arguments)]
pub(crate) fn construct_multi_tile_v8_owner_envelope_v11(
    physical: MultiTileRuntimeResult,
    projected: &V8CoveredProjection,
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
    failure_hook: OwnerFailureHook<'_>,
    duration_s_bits: u64,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::Persistent)?;
    #[cfg(test)]
    let persistent_forcing = canonical_covered_poison_persistent_forcing_v1(persistent_forcing);
    #[cfg(test)]
    let persistent_forcing = &persistent_forcing;
    let persistent = execute_uncommitted_v8_persistent_phase_v11(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        persistent_forcing,
        nitrogen,
        duration_s_bits,
    )?;
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::VegetationCandidate)?;
    #[cfg(test)]
    let vegetation_beginning =
        canonical_covered_poison_vegetation_beginning_v1(vegetation_beginning);
    #[cfg(test)]
    let vegetation_beginning = &vegetation_beginning;
    let vegetation = construct_uncommitted_v8_vegetation_candidate(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        projected.final_state(),
        &persistent,
    )?;
    join_covered_v8_owner_envelope(
        CoveredV8PhysicalOwner::MultiTile(physical),
        vegetation,
        biogeochemistry_beginning,
        failure_hook,
    )
}

/// Complete the vegetation/BGC owner join from an already accepted native V3
/// physical transaction. This is projection-only: it cannot invoke an LSE
/// solve, water authorization, surface mutation, or WB14 execution.
#[allow(clippy::too_many_arguments)]
pub(crate) fn construct_frozen_litter_v3_owner_envelope_v11(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
    hydrology: UnifiedRealHydrologyCandidate,
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
    duration_s_bits: u64,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    construct_frozen_litter_v3_owner_envelope_v11_with_failure_hook(
        fixed,
        hydrology,
        vegetation_configuration,
        vegetation_beginning,
        persistent_forcing,
        nitrogen,
        biogeochemistry_beginning,
        None,
        duration_s_bits,
    )
}

#[allow(clippy::too_many_arguments)]
fn construct_frozen_litter_v3_owner_envelope_v11_with_failure_hook(
    fixed: &super::v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
    hydrology: UnifiedRealHydrologyCandidate,
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
    failure_hook: OwnerFailureHook<'_>,
    duration_s_bits: u64,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    validate_frozen_litter_v3_fixed_identity(fixed, &hydrology)?;
    let potentials = fixed
        .potential_vegetation_operands
        .iter()
        .collect::<Vec<_>>();
    let mut finals = fixed
        .legacy_tiles
        .iter()
        .filter_map(|tile| match tile {
            super::multi_tile_runtime::FinalizedRuntimeTile::Covered(value) => {
                Some(&value.vegetation_operands)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    finals.extend(fixed.frozen_litter_tiles.iter().map(|tile| {
        &tile
            .fixed_final
            .complete_physical_candidate
            .vegetation_operands
    }));
    let projected = project_multi_tile_v8_passes_v11(
        &potentials,
        &finals,
        &fixed.vegetation_bindings,
        &hydrology,
        vegetation_configuration,
        vegetation_beginning,
        duration_s_bits,
    )?;
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::Persistent)?;
    #[cfg(test)]
    let persistent_forcing = canonical_covered_poison_persistent_forcing_v1(persistent_forcing);
    #[cfg(test)]
    let persistent_forcing = &persistent_forcing;
    let persistent = execute_uncommitted_v8_persistent_phase_v11(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        persistent_forcing,
        nitrogen,
        duration_s_bits,
    )?;
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::VegetationCandidate)?;
    #[cfg(test)]
    let vegetation_beginning =
        canonical_covered_poison_vegetation_beginning_v1(vegetation_beginning);
    #[cfg(test)]
    let vegetation_beginning = &vegetation_beginning;
    let vegetation = construct_uncommitted_v8_vegetation_candidate(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        projected.final_state(),
        &persistent,
    )?;
    join_covered_v8_owner_envelope(
        CoveredV8PhysicalOwner::FrozenLitterV3 {
            hydrology,
            fixed: Box::new(fixed.clone()),
        },
        vegetation,
        biogeochemistry_beginning,
        failure_hook,
    )
}

fn join_covered_v8_owner_envelope(
    physical: CoveredV8PhysicalOwner,
    vegetation: UncommittedV8VegetationCandidate,
    biogeochemistry_beginning: &BiogeochemistryState,
    failure_hook: OwnerFailureHook<'_>,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    vegetation.validate_sealed()?;
    if vegetation.transaction_id() != physical.hydrology().transaction_id()
        || biogeochemistry_beginning.last_transaction_id.checked_add(1)
            != Some(vegetation.transaction_id().0)
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "beginning owner lineage",
        ));
    }
    let proposals = vegetation
        .material_proposals()
        .iter()
        .map(|value| MaterialProposal {
            transaction_id: value.transaction_id,
            owner_id: value.owner_id.clone(),
            donor: value.donor,
            receiver: value.receiver,
            proposal_id: value.proposal_id,
            amounts: MaterialPool {
                carbon: value.carbon,
                nitrogen: value.nitrogen,
                dry_matter: value.dry_matter,
            },
        })
        .collect::<Vec<_>>();
    let (requests, authorizations, uses) = vegetation.nitrogen_protocol();
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::BiogeochemistryCandidate)?;
    #[cfg(test)]
    let biogeochemistry_beginning =
        canonical_covered_poison_biogeochemistry_beginning_v1(biogeochemistry_beginning);
    #[cfg(test)]
    let biogeochemistry_beginning = &biogeochemistry_beginning;
    let biogeochemistry = construct_biogeochemistry_candidate(
        biogeochemistry_beginning,
        vegetation.transaction_id(),
        requests,
        authorizations,
        uses,
        &proposals,
        TransformationsMode::Disabled,
    )?;
    compare_material_receipts(&vegetation, &biogeochemistry)?;
    #[allow(unused_mut)] // Mutated only by the cfg(test) real-validator poison.
    let mut envelope = UncommittedCoveredV8OwnerEnvelope {
        transaction_id: vegetation.transaction_id(),
        vegetation,
        physical,
        biogeochemistry,
    };
    #[cfg(test)]
    if matches!(
        crate::v9_real_consumer_shadow::canonical_covered_parity_poison_v1(),
        Some(
            crate::v9_real_consumer_shadow::CanonicalCoveredPhysicalParityPoisonV1::V8EnvelopeValidation
        )
    ) {
        envelope.transaction_id = TransactionId(envelope.transaction_id.0.saturating_add(1));
    }
    run_owner_failure_hook(failure_hook, V8OwnerFailurePhase::EnvelopeValidation)?;
    #[cfg(test)]
    crate::v9_real_consumer_shadow::canonical_covered_final_validation_boundary_v1(
        crate::v9_real_consumer_shadow::CanonicalCoveredFinalConstructorStageV1::V8Receipt,
    );
    envelope.validate()?;
    Ok(envelope)
}

fn run_owner_failure_hook(
    hook: OwnerFailureHook<'_>,
    phase: V8OwnerFailurePhase,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    if let Some(hook) = hook {
        hook(phase)?;
    }
    Ok(())
}

#[cfg(test)]
fn canonical_covered_poison_persistent_forcing_v1(
    forcing: &V8PersistentForcingReceipt,
) -> V8PersistentForcingReceipt {
    let mut forcing = forcing.clone();
    if matches!(
        crate::v9_real_consumer_shadow::canonical_covered_parity_poison_v1(),
        Some(
            crate::v9_real_consumer_shadow::CanonicalCoveredPhysicalParityPoisonV1::V8Persistent
                | crate::v9_real_consumer_shadow::CanonicalCoveredPhysicalParityPoisonV1::LowerBoundaryAndV8Persistent
        )
    ) {
        forcing.transaction_id = TransactionId(forcing.transaction_id.0.saturating_add(1));
    }
    forcing
}

#[cfg(test)]
fn canonical_covered_poison_vegetation_beginning_v1(
    beginning: &V8CoupledOwnedState,
) -> V8CoupledOwnedState {
    let mut beginning = beginning.clone();
    if matches!(
        crate::v9_real_consumer_shadow::canonical_covered_parity_poison_v1(),
        Some(
            crate::v9_real_consumer_shadow::CanonicalCoveredPhysicalParityPoisonV1::V8VegetationCandidate
        )
    ) {
        beginning.state_sha256.push('0');
    }
    beginning
}

#[cfg(test)]
fn canonical_covered_poison_biogeochemistry_beginning_v1(
    beginning: &BiogeochemistryState,
) -> BiogeochemistryState {
    let mut beginning = beginning.clone();
    if matches!(
        crate::v9_real_consumer_shadow::canonical_covered_parity_poison_v1(),
        Some(
            crate::v9_real_consumer_shadow::CanonicalCoveredPhysicalParityPoisonV1::V8Biogeochemistry
        )
    ) {
        beginning.last_transaction_id = beginning.last_transaction_id.saturating_add(1);
    }
    beginning
}

#[cfg(test)]
fn canonical_covered_final_v8_receipt_boundary_v1() {
    crate::v9_real_consumer_shadow::canonical_covered_final_constructor_boundary_v1(
        crate::v9_real_consumer_shadow::CanonicalCoveredFinalConstructorStageV1::V8Receipt,
    );
}

#[cfg(not(test))]
const fn canonical_covered_final_v8_receipt_boundary_v1() {}

#[cfg(test)]
fn canonical_covered_final_owner_boundary_v1(
    phase: V8OwnerFailurePhase,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    use crate::v9_real_consumer_shadow::CanonicalCoveredFinalConstructorStageV1 as Stage;
    let stage = match phase {
        V8OwnerFailurePhase::Persistent => Stage::VegetationPersistent,
        V8OwnerFailurePhase::VegetationCandidate => Stage::VegetationMaterial,
        V8OwnerFailurePhase::BiogeochemistryCandidate => Stage::Biogeochemistry,
        V8OwnerFailurePhase::EnvelopeValidation => return Ok(()),
    };
    crate::v9_real_consumer_shadow::canonical_covered_final_constructor_boundary_v1(stage);
    Ok(())
}

#[cfg(not(test))]
fn canonical_covered_final_owner_boundary_v1(
    _: V8OwnerFailurePhase,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    Ok(())
}

fn validate_actual_root_protocol(
    source: &AcceptedCoveredVegetationOperands,
    hydrology: &UnifiedRealHydrologyCandidate,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    let requests = rows_by_key(&hydrology.arbitration().requests)?;
    let authorizations = authorization_rows_by_key(&hydrology.arbitration().authorizations)?;
    let uses = rows_by_key(hydrology.finalized_uses())?;
    let mut expected_keys = std::collections::BTreeSet::new();
    for occupancy in &source.occupancies {
        for root in &occupancy.root_water {
            let key = &root.key;
            expected_keys.insert(key.clone());
            if key.transaction_id != source.transaction_id
                || key.requesting_owner_id != source.vegetation_owner_id
                || key.requesting_component != RequestingComponent::VegetationRoot
                || key.ofe_id != source.ofe_id
                || key.requesting_tile_id != source.tile_id
                || key.occupancy_id.as_ref() != Some(&occupancy.occupancy_id)
                || key.soil_layer_id.is_none()
                || requests.get(key).map(|value| value.to_bits())
                    != Some(root.request_kg_m2_stand_ground.to_bits())
                || authorizations.get(key).map(|value| value.to_bits())
                    != Some(root.authorization_kg_m2_stand_ground.to_bits())
                || uses.get(key).map(|value| value.to_bits())
                    != Some(root.finalized_use_kg_m2_stand_ground.to_bits())
            {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "root D/A/F actual-owner correspondence",
                ));
            }
        }
    }
    let actual_root_keys = requests
        .keys()
        .filter(|key| {
            key.requesting_owner_id == source.vegetation_owner_id
                && key.requesting_component == RequestingComponent::VegetationRoot
                && key.ofe_id == source.ofe_id
                && key.requesting_tile_id == source.tile_id
        })
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let actual_authorization_keys = authorizations
        .keys()
        .filter(|key| actual_root_keys.contains(*key))
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let actual_use_keys = uses
        .keys()
        .filter(|key| actual_root_keys.contains(*key))
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    if expected_keys != actual_root_keys
        || expected_keys != actual_authorization_keys
        || expected_keys != actual_use_keys
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "root D/A/F actual-owner key set",
        ));
    }
    Ok(())
}

fn rows_by_key(
    rows: &[WaterAmount],
) -> Result<BTreeMap<GroundWaterKey, f64>, CoveredV8OwnerEnvelopeError> {
    let mut values = BTreeMap::new();
    for row in rows {
        if values
            .insert(row.key.clone(), row.amount_kg_m2_stand_ground)
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate actual hydrology water row",
            ));
        }
    }
    Ok(values)
}

fn authorization_rows_by_key(
    rows: &[openwepp_land_surface_energy::WaterAuthorization],
) -> Result<BTreeMap<GroundWaterKey, f64>, CoveredV8OwnerEnvelopeError> {
    let mut values = BTreeMap::new();
    for row in rows {
        if values
            .insert(row.key.clone(), row.amount_kg_m2_stand_ground)
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate actual hydrology authorization row",
            ));
        }
    }
    Ok(values)
}

fn compare_material_receipts(
    vegetation: &UncommittedV8VegetationCandidate,
    biogeochemistry: &BiogeochemistryOwnerCandidate,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    compare_material_receipt_rows(vegetation.material_proposals(), biogeochemistry.receipts())
}

fn compare_material_receipt_rows(
    proposals: &[MaterialTransfer],
    receipts: &[MaterialReceipt],
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    if proposals.len() != receipts.len()
        || proposals.iter().zip(receipts).any(|(proposal, receipt)| {
            proposal.transaction_id != receipt.transaction_id
                || proposal.owner_id != receipt.owner_id
                || proposal.donor != receipt.donor
                || proposal.receiver != receipt.receiver
                || proposal.proposal_id != receipt.proposal_id
                || proposal.carbon.to_bits() != receipt.amounts.carbon.to_bits()
                || proposal.nitrogen.to_bits() != receipt.amounts.nitrogen.to_bits()
                || proposal.dry_matter.to_bits() != receipt.amounts.dry_matter.to_bits()
        })
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "vegetation proposal/BGC receipt correspondence",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use openwepp_kernel_contract::{
        MaterialDonorClass, MaterialReceiverClass, ResourceOwnerId, SoilLayerId, TileId,
        TransactionId,
    };
    use openwepp_land_surface_energy::{
        ComponentId, OfeId, RequestingComponent, SourceId, StandGroundWaterAmountBasis,
        WaterSourceType,
    };

    use super::*;

    fn root_key() -> GroundWaterKey {
        GroundWaterKey {
            transaction_id: TransactionId(9),
            requesting_owner_id: ResourceOwnerId::try_new("vegetation-v8").expect("owner"),
            requesting_component: RequestingComponent::VegetationRoot,
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            requesting_tile_id: TileId::try_new("tile-1").expect("tile"),
            occupancy_id: Some(ComponentId::try_new("component-1").expect("component")),
            surface_id: None,
            surface_class: None,
            source_type: WaterSourceType::SoilLayerLiquid,
            source_id: SourceId::try_new("soil-1").expect("source"),
            source_tile_id: None,
            soil_layer_id: Some(SoilLayerId::try_new("soil-1").expect("layer")),
            amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        }
    }

    #[test]
    fn actual_owner_rows_preserve_complete_key_and_exact_amount() {
        let row = WaterAmount {
            key: root_key(),
            amount_kg_m2_stand_ground: 0.125,
        };
        let mapped = rows_by_key(std::slice::from_ref(&row)).expect("unique row");
        assert_eq!(mapped.len(), 1);
        assert_eq!(
            mapped[&row.key].to_bits(),
            row.amount_kg_m2_stand_ground.to_bits()
        );
    }

    #[test]
    fn duplicate_actual_owner_key_is_rejected_before_join() {
        let row = WaterAmount {
            key: root_key(),
            amount_kg_m2_stand_ground: 0.125,
        };
        assert_eq!(
            rows_by_key(&[row.clone(), row]),
            Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate actual hydrology water row"
            ))
        );
    }

    #[test]
    fn v50_envelope_transaction_join_refuses_each_owner_substitution() {
        let exact = TransactionId(42);
        validate_owner_transaction_identity(exact, exact, exact, exact)
            .expect("exact V50 envelope transaction join");
        for (label, envelope, vegetation, physical, biogeochemistry) in [
            ("envelope", TransactionId(41), exact, exact, exact),
            ("vegetation", exact, TransactionId(41), exact, exact),
            ("physical", exact, exact, TransactionId(41), exact),
            ("biogeochemistry", exact, exact, exact, TransactionId(41)),
        ] {
            assert_eq!(
                validate_owner_transaction_identity(
                    envelope,
                    vegetation,
                    physical,
                    biogeochemistry,
                ),
                Err(CoveredV8OwnerEnvelopeError::Identity(
                    "heterogeneous transaction identity"
                )),
                "{label} transaction substitution must refuse",
            );
        }
    }

    #[test]
    fn v50_envelope_material_receipt_substitution_refuses() {
        let owner = ResourceOwnerId::try_new("v50-envelope-owner").expect("owner");
        let proposal = MaterialTransfer {
            transaction_id: 42,
            owner_id: owner.clone(),
            proposal_id: 7,
            donor: MaterialDonorClass::Leaf,
            receiver: MaterialReceiverClass::Metabolic,
            carbon: 0.0048,
            nitrogen: 0.0001,
            dry_matter: 0.01,
        };
        let receipt = MaterialReceipt {
            transaction_id: proposal.transaction_id,
            owner_id: owner,
            donor: proposal.donor,
            receiver: proposal.receiver,
            proposal_id: proposal.proposal_id,
            amounts: MaterialPool {
                carbon: proposal.carbon,
                nitrogen: proposal.nitrogen,
                dry_matter: proposal.dry_matter,
            },
        };
        compare_material_receipt_rows(
            std::slice::from_ref(&proposal),
            std::slice::from_ref(&receipt),
        )
        .expect("exact V50 material receipt join");
        for (label, mut poison) in [
            ("transaction", receipt.clone()),
            ("owner", receipt.clone()),
            ("proposal", receipt.clone()),
            ("carbon", receipt.clone()),
        ] {
            match label {
                "transaction" => poison.transaction_id = 41,
                "owner" => {
                    poison.owner_id = ResourceOwnerId::try_new("foreign-owner").expect("owner")
                }
                "proposal" => poison.proposal_id = 8,
                _ => poison.amounts.carbon = f64::from_bits(poison.amounts.carbon.to_bits() + 1),
            }
            assert_eq!(
                compare_material_receipt_rows(
                    std::slice::from_ref(&proposal),
                    std::slice::from_ref(&poison),
                ),
                Err(CoveredV8OwnerEnvelopeError::Identity(
                    "vegetation proposal/BGC receipt correspondence"
                )),
                "{label} material receipt substitution must refuse",
            );
        }
    }
}
