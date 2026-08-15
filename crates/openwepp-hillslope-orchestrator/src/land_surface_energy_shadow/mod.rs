//! Default-off snow-free LSE arbitration against the actual direct hydrology owner.
//!
//! Soil-layer liquid remains in the production layer owner. Snow-free surface
//! and litter liquid use the digest-bound `DirectSurfaceLiquidOwnedState`.
//! Their separately constructed candidates join only after exact LSE water
//! protocol validation, and timed ingress installs the validated surface ending
//! state into a clone; the production frame is never mutated.

#![allow(clippy::missing_errors_doc)]

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{
    ResourceOwnerId, SoilLayerId, TileId, TransactionId, canonical_resource_amount_sum,
};
pub use openwepp_land_surface_energy::{
    BandDirectionalFluxes, BareSoilParameters, ComponentId, CondensationCredit, GroundWaterKey,
    LandSurfaceEnergyError, LandSurfaceEnergyErrorClass, OfeId, OpenNeutralGeometry,
    OpenPotentialPhase, OpenSurfaceProblem, OwnerKind, OwnerRollbackHash,
    PotentialWaterRequestBatch, RequestingComponent, RuntimeTileIdentity, Sha256Digest,
    SoilThermalLayerCandidate, SoilThermalLayerSnapshot, SoilThermalNodeOperands,
    SoilThermalOfeSnapshot, SoilThermalSnapshot, SoilThermalTileCandidate, SourceId,
    StandGroundWaterAmountBasis, SurfaceClass, SurfaceClassKind, SurfaceId, SurfaceStorageBranch,
    TileState, WaterAmount, WaterAuthorization, WaterAuthorizationReason, WaterProtocol,
    WaterProtocolRow, WaterProtocolViolation, WaterSourceType, WaterUseOperands,
    evaluate_open_surface, finalize_open_phase, solve_open_potential_phase, validate_water_use,
};
use openwepp_land_surface_energy::{OpenSurfaceSolveOutcome, WaterBranch, solve_open_surface};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::direct_runtime::{
    DirectLayerWithdrawalRequest, DirectSurfaceLiquidClosureUnit, aggregate_direct_soil_water,
    apply_direct_finalized_layer_liquid_debit, apply_direct_same_pass_infiltration,
    authorize_direct_layer_withdrawals, checked_surface_liquid_add, checked_surface_liquid_close,
    checked_surface_liquid_div, checked_surface_liquid_mul, checked_surface_liquid_sub,
    checked_surface_liquid_sum,
};
use crate::vegetation_real_hydrology_shadow::{
    RealHydrologyShadowAdapter, RealHydrologyShadowError, RealHydrologySourceKey,
};
use crate::{
    DirectRunFrame, DirectSurfaceLiquidArbitration, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidErrorContext,
    DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidIngressInput,
    DirectSurfaceLiquidOfeBinding, DirectSurfaceLiquidParcelReceipt, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidReceiptDisposition, DirectSurfaceLiquidReceiptRecipient,
    DirectSurfaceLiquidResourceCandidate, DirectSurfaceLiquidRollbackHashes,
    apply_surface_liquid_resource_phase, authorize_surface_liquid_withdrawals,
    execute_surface_liquid_ingress,
};

mod finalization_sealing;
mod receiver_failure;
mod receiver_preflight;
mod receiver_validation;
mod unified_entry_preflight;
use finalization_sealing::first_sealed_finalization_violation;
use receiver_failure::canonical_receiver_failure;
use receiver_validation::{
    FramedSha256, canonicalize_finalized_error, canonicalize_unified_error,
    finalization_receiver_sets_sha256, preflight_protocol_bounds, preflight_protocol_cardinality,
    preflight_protocol_domains, preflight_protocol_identities, preflight_request_bounds,
    preflight_request_cardinality, preflight_request_domains, preflight_request_identities,
    protocol_error_code_and_detail, protocol_failure, receiver_atomic_failure,
    receiver_expectation_fields_sha256, receiver_expectations_sha256, receiver_operands_sha256,
    request_failure, require_receiver_close, shadow_error_code, snapshot_failure,
    validate_surface_production_binding, validate_surface_production_lane_domains,
    water_protocol_sha256, water_request_batch_sha256,
};

const WATER_DENSITY_KG_M3: f64 = 1_000.0;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum LandSurfaceEnergyShadowError {
    #[error("real hydrology identity failure: {0}")]
    Identity(&'static str),
    #[error("real hydrology operand failure: {0}")]
    Operand(&'static str),
    #[error("real hydrology bound failure: {0}")]
    Bound(&'static str),
    #[error("unsupported production custody: {0}")]
    UnsupportedCustody(&'static str),
    #[error(transparent)]
    LandSurface(#[from] LandSurfaceEnergyError),
    #[error(transparent)]
    SurfaceLiquid(#[from] DirectSurfaceLiquidError),
}

impl From<RealHydrologyShadowError> for LandSurfaceEnergyShadowError {
    fn from(value: RealHydrologyShadowError) -> Self {
        let (code, detail) = match value {
            RealHydrologyShadowError::Identity(detail) => {
                (DirectSurfaceLiquidErrorCode::E002, detail)
            }
            RealHydrologyShadowError::Operand(detail) => {
                (DirectSurfaceLiquidErrorCode::E003, detail)
            }
            RealHydrologyShadowError::Bound(detail) => (DirectSurfaceLiquidErrorCode::E006, detail),
            RealHydrologyShadowError::Protocol(_) => {
                (DirectSurfaceLiquidErrorCode::E005, "resource protocol")
            }
        };
        DirectSurfaceLiquidError::canonical_failure(
            code,
            DirectSurfaceLiquidPhase::Authorization,
            DirectSurfaceLiquidErrorContext::default(),
            DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256: None,
                attempted_owner_sha256: None,
            },
            detail,
        )
        .into()
    }
}

/// One neutral LSE/V8 request bound to an actual production layer.
#[derive(Clone, Debug, PartialEq)]
pub struct MixedRealHydrologyRequest {
    pub request: WaterAmount,
    pub source: RealHydrologySourceKey,
}

/// Maximum authorization returned by one immutable production snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct MixedRealHydrologyAuthorization {
    pub authorization: WaterAuthorization,
    pub source: RealHydrologySourceKey,
}

/// Finalized withdrawal produced by the fixed-cap constitutive rebuild.
#[derive(Clone, Debug, PartialEq)]
pub struct MixedRealHydrologyUse {
    pub finalized_use: WaterAmount,
    pub source: RealHydrologySourceKey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixedRealHydrologyArbitration {
    requests: Vec<MixedRealHydrologyRequest>,
    authorizations: Vec<MixedRealHydrologyAuthorization>,
    beginning_frame: DirectRunFrame,
    transaction_id: TransactionId,
}

impl MixedRealHydrologyArbitration {
    #[must_use]
    pub fn requests(&self) -> &[MixedRealHydrologyRequest] {
        &self.requests
    }

    #[must_use]
    pub fn authorizations(&self) -> &[MixedRealHydrologyAuthorization] {
        &self.authorizations
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixedRealHydrologyCandidate {
    beginning_frame: DirectRunFrame,
    ending_frame: DirectRunFrame,
    finalized_uses: Vec<MixedRealHydrologyUse>,
    transaction_id: TransactionId,
}

/// Exact result of the LSE fixed-authorization solve.
#[derive(Clone, Debug, PartialEq)]
pub struct UnifiedLseFinalization {
    water_protocol: WaterProtocol,
    ending_tile_states_pre_ingress: Vec<TileState>,
    soil_thermal_candidates: Vec<SoilThermalTileCandidate>,
    rollback_hashes: Vec<OwnerRollbackHash>,
}

/// Independently frozen LSE and soil-thermal receiver identities.
#[derive(Clone, Debug, PartialEq)]
pub struct UnifiedReceiverExpectations {
    lse_owner_id: ResourceOwnerId,
    beginning_lse_state_sha256: Sha256Digest,
    hydrology_owner_id: ResourceOwnerId,
    beginning_hydrology_snapshot_sha256: Sha256Digest,
    soil_thermal_owner_id: ResourceOwnerId,
    beginning_soil_thermal_state_sha256: Sha256Digest,
    ordered_thermal_layers: Vec<((OfeId, TileId), Vec<SoilLayerId>)>,
}

/// One logical authorization spanning production soil and surface-liquid owners.
#[derive(Clone, Debug, PartialEq)]
pub struct UnifiedRealHydrologyArbitration {
    pub transaction_id: TransactionId,
    pub requests: Vec<WaterAmount>,
    pub authorizations: Vec<WaterAuthorization>,
    soil: MixedRealHydrologyArbitration,
    surface: DirectSurfaceLiquidArbitration,
}

/// Complete default-off water candidate after resource use and timed ingress.
#[derive(Clone, Debug, PartialEq)]
pub struct UnifiedRealHydrologyCandidate {
    transaction_id: TransactionId,
    beginning_frame: DirectRunFrame,
    ending_frame: DirectRunFrame,
    arbitration: UnifiedRealHydrologyArbitration,
    finalized_uses: Vec<WaterAmount>,
    condensation_credits: Vec<CondensationCredit>,
    surface_resource: DirectSurfaceLiquidResourceCandidate,
    surface_ingress: DirectSurfaceLiquidIngressCandidate,
    ending_lse_tile_states: Vec<TileState>,
    soil_thermal_candidates: Vec<SoilThermalTileCandidate>,
    receiver_closure_operands: RealReceiverClosureOperands,
    rollback_hashes: Vec<OwnerRollbackHash>,
}

/// Frozen production-soil layer operands used by the independent receiver validator.
#[derive(Clone, Debug, PartialEq)]
pub struct ProductionSoilLayerReceiverOperands {
    pub layer_id: SoilLayerId,
    pub beginning_liquid_m: f64,
    pub ending_liquid_m: f64,
    pub layer_depth_m: f64,
    pub residual_theta: f64,
    pub frozen_depth_m: f64,
}

/// One bound production lane before and after the shared infiltration transition.
#[derive(Clone, Debug, PartialEq)]
pub struct ProductionSoilReceiverOperands {
    pub ofe_id: OfeId,
    pub production_lane_index: usize,
    pub production_lane_id: u32,
    pub tillage_depth_m: f64,
    pub infiltration_m: f64,
    pub beginning_aggregate_soil_water_m: f64,
    pub ending_aggregate_soil_water_m: f64,
    pub ordered_layers: Vec<ProductionSoilLayerReceiverOperands>,
}

/// Frozen soil-thermal receiver operands for one named infiltration layer.
#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalReceiverOperands {
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub layer_id: SoilLayerId,
    pub beginning_infiltration_credit_j_m2_ofe_ground: f64,
    pub ending_infiltration_credit_j_m2_ofe_ground: f64,
    pub beginning_enthalpy_j_m2_ofe_ground: f64,
    pub infiltration_enthalpy_j_m2_ofe_ground: f64,
    pub ending_enthalpy_j_m2_ofe_ground: f64,
}

/// Frozen LSE tile-state operands for retained post-infiltration enthalpy.
#[derive(Clone, Debug, PartialEq)]
pub struct LseTileReceiverOperands {
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub tile_fraction: f64,
    pub beginning_enthalpy_j_m2_tile_ground: f64,
    pub retained_enthalpy_j_m2_ofe_ground: f64,
    pub ending_enthalpy_j_m2_tile_ground: f64,
}

/// Complete frozen receiver envelope. It contains operands, never residuals.
#[derive(Clone, Debug, PartialEq)]
pub struct RealReceiverClosureOperands {
    pub transaction_id: TransactionId,
    pub hydrology_owner_id: ResourceOwnerId,
    lse_owner_id: ResourceOwnerId,
    soil_thermal_owner_id: ResourceOwnerId,
    pub beginning_hydrology_snapshot_sha256: Sha256Digest,
    beginning_lse_state_sha256: Sha256Digest,
    beginning_soil_thermal_state_sha256: Sha256Digest,
    rollback_hashes: Vec<OwnerRollbackHash>,
    pub production_soil: Vec<ProductionSoilReceiverOperands>,
    pub soil_thermal: Vec<SoilThermalReceiverOperands>,
    pub lse_tiles: Vec<LseTileReceiverOperands>,
    expected_production_soil: Vec<(OfeId, usize, u32, Vec<SoilLayerId>)>,
    expected_soil_thermal: Vec<(OfeId, TileId, SoilLayerId)>,
    expected_lse_tiles: Vec<(OfeId, TileId)>,
    configured_surface_context: Vec<(OfeId, TileId, SurfaceId, SourceId)>,
}

impl RealReceiverClosureOperands {
    /// Canonical attempted-owner hash used by independent receiver failures.
    #[must_use]
    pub fn canonical_sha256(&self) -> String {
        receiver_operands_sha256(self)
    }
}

#[derive(Clone, Debug)]
struct ReceiverEnvelopeViolation {
    code: DirectSurfaceLiquidErrorCode,
    owner_kind: Option<OwnerKind>,
    owner_id: Option<ResourceOwnerId>,
    ofe_id: Option<OfeId>,
    tile_id: Option<TileId>,
    detail: &'static str,
}

impl ReceiverEnvelopeViolation {
    fn for_tile(
        owner_kind: OwnerKind,
        owner_id: Option<ResourceOwnerId>,
        ofe_id: OfeId,
        tile_id: TileId,
        detail: &'static str,
    ) -> Self {
        Self {
            code: DirectSurfaceLiquidErrorCode::E011,
            owner_kind: Some(owner_kind),
            owner_id,
            ofe_id: Some(ofe_id),
            tile_id: Some(tile_id),
            detail,
        }
    }

    fn for_owner(
        owner_kind: OwnerKind,
        owner_id: Option<ResourceOwnerId>,
        detail: &'static str,
    ) -> Self {
        Self {
            code: DirectSurfaceLiquidErrorCode::E011,
            owner_kind: Some(owner_kind),
            owner_id,
            ofe_id: None,
            tile_id: None,
            detail,
        }
    }

    fn cardinality_for_tile(
        owner_kind: OwnerKind,
        owner_id: Option<ResourceOwnerId>,
        ofe_id: OfeId,
        tile_id: TileId,
        detail: &'static str,
    ) -> Self {
        Self {
            code: DirectSurfaceLiquidErrorCode::E005,
            owner_kind: Some(owner_kind),
            owner_id,
            ofe_id: Some(ofe_id),
            tile_id: Some(tile_id),
            detail,
        }
    }
}

impl UnifiedReceiverExpectations {
    /// Canonical attempted-owner hash used by pre-callback receiver failures.
    #[must_use]
    pub fn canonical_sha256(&self) -> String {
        receiver_expectations_sha256(self)
    }

    pub fn try_new(
        lse_owner_id: ResourceOwnerId,
        beginning_lse_state_sha256: Sha256Digest,
        hydrology_owner_id: ResourceOwnerId,
        beginning_hydrology_snapshot_sha256: Sha256Digest,
        soil_thermal_owner_id: ResourceOwnerId,
        beginning_soil_thermal_state_sha256: Sha256Digest,
        ordered_thermal_layers: Vec<(OfeId, TileId, Vec<SoilLayerId>)>,
    ) -> Result<Self, LandSurfaceEnergyShadowError> {
        let attempted_sha256 = receiver_expectation_fields_sha256(
            &lse_owner_id,
            &beginning_lse_state_sha256,
            &hydrology_owner_id,
            &beginning_hydrology_snapshot_sha256,
            &soil_thermal_owner_id,
            &beginning_soil_thermal_state_sha256,
            &ordered_thermal_layers,
        );
        let mut identities = BTreeSet::new();
        let mut layers_by_tile = Vec::new();
        for (ofe_id, tile_id, layers) in ordered_thermal_layers {
            if layers.is_empty()
                || layers.iter().collect::<BTreeSet<_>>().len() != layers.len()
                || !identities.insert((ofe_id.clone(), tile_id.clone()))
            {
                return Err(receiver_expectation_cardinality_failure(
                    &soil_thermal_owner_id,
                    &beginning_hydrology_snapshot_sha256,
                    &attempted_sha256,
                    Some(ofe_id),
                    Some(tile_id),
                    "invalid independent receiver expectations",
                ));
            }
            layers_by_tile.push(((ofe_id, tile_id), layers));
        }
        if layers_by_tile.is_empty() {
            return Err(receiver_expectation_cardinality_failure(
                &soil_thermal_owner_id,
                &beginning_hydrology_snapshot_sha256,
                &attempted_sha256,
                None,
                None,
                "empty independent receiver expectations",
            ));
        }
        Ok(Self {
            lse_owner_id,
            beginning_lse_state_sha256,
            hydrology_owner_id,
            beginning_hydrology_snapshot_sha256,
            soil_thermal_owner_id,
            beginning_soil_thermal_state_sha256,
            ordered_thermal_layers: layers_by_tile,
        })
    }
}

fn receiver_expectation_cardinality_failure(
    owner_id: &ResourceOwnerId,
    beginning_sha256: &Sha256Digest,
    attempted_sha256: &str,
    ofe_id: Option<OfeId>,
    tile_id: Option<TileId>,
    detail: &'static str,
) -> LandSurfaceEnergyShadowError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E005,
        DirectSurfaceLiquidPhase::IndependentClosure,
        DirectSurfaceLiquidErrorContext {
            owner_id: Some(owner_id.clone()),
            ofe_id,
            tile_id,
            ..DirectSurfaceLiquidErrorContext::default()
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(beginning_sha256.to_string()),
            attempted_owner_sha256: Some(attempted_sha256.to_owned()),
        },
        detail,
    )
    .into()
}

impl UnifiedLseFinalization {
    pub fn try_new(
        receiver_expectations: &UnifiedReceiverExpectations,
        water_protocol: WaterProtocol,
        ending_tile_states_pre_ingress: Vec<TileState>,
        soil_thermal_candidates: Vec<SoilThermalTileCandidate>,
        rollback_hashes: Vec<OwnerRollbackHash>,
    ) -> Result<Self, LandSurfaceEnergyShadowError> {
        let beginning = water_protocol.beginning_snapshot_sha256.clone();
        let attempted_protocol = water_protocol_sha256(&water_protocol);
        preflight_protocol_identities(&water_protocol, &beginning, &attempted_protocol)?;
        preflight_protocol_domains(&water_protocol, &beginning, &attempted_protocol)?;
        let attempted_receivers = finalization_receiver_sets_sha256(
            &ending_tile_states_pre_ingress,
            &soil_thermal_candidates,
            &rollback_hashes,
        );
        receiver_validation::preflight_sealed_finalization_numerics(
            &water_protocol,
            &ending_tile_states_pre_ingress,
            &soil_thermal_candidates,
            &rollback_hashes,
            &attempted_receivers,
        )?;
        preflight_protocol_cardinality(&water_protocol, &beginning, &attempted_protocol)?;
        preflight_protocol_bounds(&water_protocol, &beginning, &attempted_protocol)?;
        if let Some(violation) = first_sealed_finalization_violation(
            receiver_expectations,
            &water_protocol,
            &ending_tile_states_pre_ingress,
            &soil_thermal_candidates,
            &rollback_hashes,
        ) {
            let request = violation.ofe_id.as_ref().and_then(|ofe_id| {
                water_protocol.requests.iter().find(|request| {
                    &request.key.ofe_id == ofe_id
                        && violation
                            .tile_id
                            .as_ref()
                            .is_some_and(|tile_id| &request.key.requesting_tile_id == tile_id)
                })
            });
            return Err(canonical_receiver_failure(
                violation.code,
                DirectSurfaceLiquidPhase::AtomicEnvelope,
                water_protocol.transaction_id,
                violation.owner_kind,
                violation.owner_id.as_ref(),
                violation.ofe_id.as_ref(),
                violation.tile_id.as_ref(),
                request.and_then(|row| row.key.surface_id.clone()),
                request.map(|row| row.key.source_id.clone()),
                None,
                &rollback_hashes,
                &attempted_receivers,
                violation.detail,
            )
            .into());
        }
        Ok(Self {
            water_protocol,
            ending_tile_states_pre_ingress,
            soil_thermal_candidates,
            rollback_hashes,
        })
    }

    #[must_use]
    pub const fn water_protocol(&self) -> &WaterProtocol {
        &self.water_protocol
    }

    #[must_use]
    pub fn ending_tile_states_pre_ingress(&self) -> &[TileState] {
        &self.ending_tile_states_pre_ingress
    }

    #[must_use]
    pub fn soil_thermal_candidates(&self) -> &[SoilThermalTileCandidate] {
        &self.soil_thermal_candidates
    }

    #[must_use]
    pub fn rollback_hashes(&self) -> &[OwnerRollbackHash] {
        &self.rollback_hashes
    }

    /// Canonical attempted receiver-set hash used by construction failures.
    #[must_use]
    pub fn receiver_sets_sha256(&self) -> String {
        Self::candidate_receiver_sets_sha256(
            &self.ending_tile_states_pre_ingress,
            &self.soil_thermal_candidates,
            &self.rollback_hashes,
        )
    }

    /// Canonical attempted hash for an unsealed receiver-set candidate.
    #[must_use]
    pub fn candidate_receiver_sets_sha256(
        lse_tiles: &[TileState],
        thermal_tiles: &[SoilThermalTileCandidate],
        rollback_hashes: &[OwnerRollbackHash],
    ) -> String {
        finalization_receiver_sets_sha256(lse_tiles, thermal_tiles, rollback_hashes)
    }
}

impl UnifiedRealHydrologyCandidate {
    #[must_use]
    pub const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub const fn beginning_frame(&self) -> &DirectRunFrame {
        &self.beginning_frame
    }

    #[must_use]
    pub const fn ending_frame(&self) -> &DirectRunFrame {
        &self.ending_frame
    }

    #[must_use]
    pub const fn arbitration(&self) -> &UnifiedRealHydrologyArbitration {
        &self.arbitration
    }

    #[must_use]
    pub fn finalized_uses(&self) -> &[WaterAmount] {
        &self.finalized_uses
    }

    #[must_use]
    pub fn condensation_credits(&self) -> &[CondensationCredit] {
        &self.condensation_credits
    }

    #[must_use]
    pub const fn surface_resource(&self) -> &DirectSurfaceLiquidResourceCandidate {
        &self.surface_resource
    }

    #[must_use]
    pub const fn surface_ingress(&self) -> &DirectSurfaceLiquidIngressCandidate {
        &self.surface_ingress
    }

    #[must_use]
    pub fn ending_lse_tile_states(&self) -> &[TileState] {
        &self.ending_lse_tile_states
    }

    #[must_use]
    pub fn soil_thermal_candidates(&self) -> &[SoilThermalTileCandidate] {
        &self.soil_thermal_candidates
    }

    #[must_use]
    pub const fn receiver_closure_operands(&self) -> &RealReceiverClosureOperands {
        &self.receiver_closure_operands
    }

    #[must_use]
    pub fn rollback_hashes(&self) -> &[OwnerRollbackHash] {
        &self.rollback_hashes
    }

    pub fn validate(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<(), DirectSurfaceLiquidError> {
        validate_real_receiver_closure(&self.receiver_closure_operands)?;
        let ending_surface = self
            .ending_frame
            .surface_liquid_shadow
            .as_deref()
            .ok_or_else(|| {
                self.atomic_failure(
                    OwnerKind::Hydrology,
                    &self.receiver_closure_operands.hydrology_owner_id,
                    "missing ending surface-liquid owner",
                )
            })?;
        if self.transaction_id != self.arbitration.transaction_id
            || self.transaction_id != self.surface_ingress.transaction_id()
            || self.beginning_frame != self.arbitration.soil.beginning_frame
            || ending_surface != self.surface_ingress.ending_state()
            || self.receiver_closure_operands.transaction_id != self.transaction_id
            || self.receiver_closure_operands.production_soil.len()
                != configuration.ofe_bindings.len()
            || self.receiver_closure_operands.lse_tiles.len() != configuration.records.len()
            || self.receiver_closure_operands.soil_thermal.len() != configuration.records.len()
        {
            return Err(self.atomic_failure(
                OwnerKind::Hydrology,
                &self.receiver_closure_operands.hydrology_owner_id,
                "unified candidate owner lineage or ending state",
            ));
        }
        for (tile, operands) in self
            .ending_lse_tile_states
            .iter()
            .zip(&self.receiver_closure_operands.lse_tiles)
        {
            if tile.ofe_id != operands.ofe_id || tile.tile_id != operands.tile_id {
                return Err(self.atomic_failure(
                    OwnerKind::LandSurfaceEnergy,
                    &self.receiver_closure_operands.lse_owner_id,
                    "LSE candidate/receiver closure join",
                ));
            }
            require_receiver_close(
                &self.receiver_closure_operands,
                OwnerKind::LandSurfaceEnergy,
                &self.receiver_closure_operands.lse_owner_id,
                tile.surface_enthalpy_j_m2_tile_ground,
                operands.ending_enthalpy_j_m2_tile_ground,
                DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
                Some(&tile.ofe_id),
                Some(&tile.tile_id),
                "LSE candidate/receiver closure join",
            )?;
        }
        for (tile, operands) in self
            .soil_thermal_candidates
            .iter()
            .zip(&self.receiver_closure_operands.soil_thermal)
        {
            let Some(layer) = tile
                .layers
                .iter()
                .find(|layer| layer.layer_id == operands.layer_id)
            else {
                return Err(self.atomic_failure(
                    OwnerKind::SoilThermal,
                    &self.receiver_closure_operands.soil_thermal_owner_id,
                    "soil-thermal candidate receiver layer",
                ));
            };
            if tile.ofe_id != operands.ofe_id || tile.tile_id != operands.tile_id {
                return Err(self.atomic_failure(
                    OwnerKind::SoilThermal,
                    &self.receiver_closure_operands.soil_thermal_owner_id,
                    "soil-thermal candidate/receiver closure join",
                ));
            }
            require_receiver_close(
                &self.receiver_closure_operands,
                OwnerKind::SoilThermal,
                &self.receiver_closure_operands.soil_thermal_owner_id,
                layer.ending_enthalpy_j_m2_ofe_ground,
                operands.ending_enthalpy_j_m2_ofe_ground,
                DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
                Some(&tile.ofe_id),
                Some(&tile.tile_id),
                "soil-thermal candidate/receiver closure join",
            )?;
        }
        Ok(())
    }

    fn atomic_failure(
        &self,
        owner_kind: OwnerKind,
        owner_id: &ResourceOwnerId,
        detail: &'static str,
    ) -> DirectSurfaceLiquidError {
        receiver_atomic_failure(
            &self.receiver_closure_operands,
            owner_kind,
            owner_id,
            None,
            None,
            detail,
        )
    }
}

/// Digest the complete immutable soil-layer and surface-liquid owner snapshot.
pub fn unified_beginning_hydrology_snapshot_sha256(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<Sha256Digest, LandSurfaceEnergyShadowError> {
    surface_configuration
        .preflight_schema_and_identity_structure()
        .map_err(|error| {
            snapshot_failure(
                error.code(),
                soil_adapter.owner,
                surface_configuration,
                "invalid surface-liquid configuration",
            )
        })?;
    let surface_state = soil_adapter
        .owner
        .beginning_frame()
        .surface_liquid_shadow
        .as_deref()
        .ok_or_else(|| {
            snapshot_failure(
                DirectSurfaceLiquidErrorCode::E002,
                soil_adapter.owner,
                surface_configuration,
                "missing beginning surface-liquid owner",
            )
        })?;
    validate_surface_production_binding(soil_adapter.owner, surface_configuration)?;
    if &surface_configuration.owner_id != soil_adapter.owner.hydrology_owner_id() {
        return Err(snapshot_failure(
            DirectSurfaceLiquidErrorCode::E002,
            soil_adapter.owner,
            surface_configuration,
            "mixed unified hydrology owner",
        ));
    }
    surface_state
        .preflight_schema_and_identity_structure(surface_configuration)
        .map_err(|error| {
            snapshot_failure(
                error.code(),
                soil_adapter.owner,
                surface_configuration,
                "invalid beginning surface-liquid owner",
            )
        })?;
    surface_configuration
        .preflight_declared_digest()
        .map_err(|error| {
            snapshot_failure(
                error.code(),
                soil_adapter.owner,
                surface_configuration,
                "invalid surface-liquid configuration",
            )
        })?;
    surface_state.preflight_declared_digest().map_err(|error| {
        snapshot_failure(
            error.code(),
            soil_adapter.owner,
            surface_configuration,
            "invalid beginning surface-liquid owner",
        )
    })?;
    validate_surface_production_lane_domains(soil_adapter.owner, surface_configuration)?;
    surface_configuration.validate().map_err(|error| {
        snapshot_failure(
            error.code(),
            soil_adapter.owner,
            surface_configuration,
            "invalid surface-liquid configuration",
        )
    })?;
    surface_state
        .validate(surface_configuration)
        .map_err(|error| {
            snapshot_failure(
                error.code(),
                soil_adapter.owner,
                surface_configuration,
                "invalid beginning surface-liquid owner",
            )
        })?;
    compose_unified_beginning_hydrology_snapshot_sha256(
        soil_adapter,
        surface_configuration,
        surface_state,
    )
}

fn compose_unified_beginning_hydrology_snapshot_sha256(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    surface_state: &crate::DirectSurfaceLiquidOwnedState,
) -> Result<Sha256Digest, LandSurfaceEnergyShadowError> {
    let mut digest = Sha256::new();
    for bytes in [
        b"openwepp-unified-hydrology-snapshot-v2".as_slice(),
        soil_adapter.owner.hydrology_owner_id().as_str().as_bytes(),
        soil_adapter.owner.snapshot_bytes(),
        surface_configuration.configuration_sha256.as_bytes(),
        surface_state.state_sha256.as_bytes(),
    ] {
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }
    Sha256Digest::try_new(format!("{:x}", digest.finalize())).map_err(Into::into)
}

/// Join one immutable LSE request batch to both actual water owners.
#[allow(clippy::too_many_lines)]
pub fn execute_unified_real_hydrology_shadow<F>(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress: &DirectSurfaceLiquidIngressInput,
    finalize_fixed_caps: F,
) -> Result<UnifiedRealHydrologyCandidate, LandSurfaceEnergyShadowError>
where
    F: FnOnce(
        &[WaterAuthorization],
    ) -> Result<UnifiedLseFinalization, LandSurfaceEnergyShadowError>,
{
    let expected_beginning_hydrology_snapshot_sha256 =
        &receiver_expectations.beginning_hydrology_snapshot_sha256;
    let unified_entry_preflight::UnifiedEntryPreflight {
        actual_snapshot,
        attempted_sha256,
        soil_requests,
        surface_requests,
    } = unified_entry_preflight::validate_unified_entry(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        request_batch,
        soil_sources,
        ingress,
        expected_beginning_hydrology_snapshot_sha256,
    )?;
    let beginning_surface = soil_adapter
        .owner
        .beginning_frame()
        .surface_liquid_shadow
        .as_deref()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing beginning surface-liquid owner",
        ))?;
    let soil = soil_adapter.authorize(&soil_requests).map_err(|error| {
        unified_entry_preflight::complete_unified_failure(
            canonicalize_unified_error(
                error,
                request_batch,
                expected_beginning_hydrology_snapshot_sha256,
            ),
            &actual_snapshot,
            &attempted_sha256,
        )
    })?;
    let surface = authorize_surface_liquid_withdrawals(
        surface_configuration,
        beginning_surface,
        request_batch.transaction_id,
        beginning_surface
            .records
            .first()
            .and_then(|record| record.last_accepted_transaction_id),
        &surface_requests,
    )
    .map_err(|error| {
        unified_entry_preflight::complete_unified_failure(
            canonicalize_unified_error(
                error.into(),
                request_batch,
                expected_beginning_hydrology_snapshot_sha256,
            ),
            &actual_snapshot,
            &attempted_sha256,
        )
    })?;
    let authorizations = restore_authorization_order(
        request_batch,
        &soil,
        &surface,
        expected_beginning_hydrology_snapshot_sha256,
    )
    .map_err(|error| {
        unified_entry_preflight::complete_unified_failure(
            error,
            &actual_snapshot,
            &attempted_sha256,
        )
    })?;
    let arbitration = UnifiedRealHydrologyArbitration {
        transaction_id: request_batch.transaction_id,
        requests: request_batch.requests.clone(),
        authorizations,
        soil,
        surface,
    };
    let finalized = finalize_fixed_caps(&arbitration.authorizations).map_err(|error| {
        unified_entry_preflight::canonicalize_callback_failure(
            &error,
            request_batch.transaction_id,
            &actual_snapshot,
            &attempted_sha256,
        )
    })?;
    validate_final_protocol(
        &finalized.water_protocol,
        &arbitration,
        expected_beginning_hydrology_snapshot_sha256,
        &surface_configuration.owner_id,
    )?;
    let finalized_protocol = finalized.water_protocol().clone();
    construct_unified_candidate(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        request_batch,
        arbitration,
        finalized,
        ingress,
    )
    .map_err(|error| canonicalize_finalized_error(error, &finalized_protocol))
}

fn validate_native_shadow_supported_domain(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
    attempted_sha256: &str,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(lane_index) = owner.beginning_frame().lanes.iter().position(|lane| {
        crate::direct_runtime::validate_direct_production_winter_lane_domain(lane).is_err()
            || !frost_indices_fit_production_layers(lane)
    }) {
        return Err(DirectSurfaceLiquidError::canonical_failure(
            DirectSurfaceLiquidErrorCode::E003,
            DirectSurfaceLiquidPhase::AtomicEnvelope,
            first_lane_error_context(owner, configuration, lane_index),
            DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256: Some(beginning_hydrology_snapshot_sha256.to_string()),
                attempted_owner_sha256: Some(attempted_sha256.to_owned()),
            },
            "nonfinite or negative production snow lane scalar",
        )
        .into());
    }
    if let Some(lane_index) = owner
        .beginning_frame()
        .lanes
        .iter()
        .position(lane_has_unsupported_frozen_or_snow_state)
    {
        return Err(DirectSurfaceLiquidError::canonical_failure(
            DirectSurfaceLiquidErrorCode::E004,
            DirectSurfaceLiquidPhase::AtomicEnvelope,
            first_lane_error_context(owner, configuration, lane_index),
            DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256: Some(beginning_hydrology_snapshot_sha256.to_string()),
                attempted_owner_sha256: Some(attempted_sha256.to_owned()),
            },
            "snow, terminal snow, frozen, or thawing production frame",
        )
        .into());
    }
    Ok(())
}

fn frost_indices_fit_production_layers(lane: &crate::direct_runtime::DirectLaneFrame) -> bool {
    let layer_count = lane.subsurface_layers.len();
    let winter = &lane.winter_column.frost;
    let winter_fits = winter
        .layer_shadows
        .iter()
        .all(|layer| (1..=layer_count).contains(&layer.layer_index))
        && winter
            .fine_layers
            .iter()
            .all(|layer| (1..=layer_count).contains(&layer.layer_index));
    let carry_fits = lane.frost_runtime_carry.as_ref().is_none_or(|carry| {
        carry
            .layer_shadows
            .iter()
            .all(|layer| (1..=layer_count).contains(&layer.layer_index))
            && carry
                .fine_layers
                .iter()
                .all(|layer| (1..=layer_count).contains(&layer.layer_index))
    });
    winter_fits && carry_fits
}

fn validate_native_shadow_exact_one_custody(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
    attempted_sha256: &str,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let legacy_custody = owner.beginning_day_frames().iter().position(|day| {
        day.infiltration_depression_inputs
            .depression_storage_delta_handoff_m
            .to_bits()
            != 0.0_f64.to_bits()
            || day
                .infiltration_depression_inputs
                .producer_inputs
                .as_ref()
                .is_some_and(|inputs| {
                    inputs.depression_storage_capacity_m.to_bits() != 0.0_f64.to_bits()
                })
            || day
                .infiltration_depression
                .depression_storage_delta_m
                .to_bits()
                != 0.0_f64.to_bits()
    });
    if let Some(lane_index) = legacy_custody {
        return Err(DirectSurfaceLiquidError::exact_one_owner_failure(
            DirectSurfaceLiquidPhase::AtomicEnvelope,
            first_lane_error_context(owner, configuration, lane_index),
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(attempted_sha256.to_owned()),
            "legacy infiltration/depression liquid custody is nonzero",
        )
        .into());
    }
    Ok(())
}

fn lane_has_unsupported_frozen_or_snow_state(
    lane: &crate::direct_runtime::DirectLaneFrame,
) -> bool {
    lane.winter_column.snow.has_runtime_state()
        || lane.winter_column.snow.liquid_water_retained_m > 0.0
        || lane.snow_runtime_carry.is_some()
        || lane.winter_column.frost.has_runtime_state()
        || lane.frost_runtime_carry.is_some()
        || lane
            .subsurface_layers
            .iter()
            .any(|layer| layer.frozen_depth_m > 0.0 || layer.frozen_water_m > 0.0)
}

fn first_lane_error_context(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    lane_index: usize,
) -> DirectSurfaceLiquidErrorContext {
    let binding = configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.production_lane_index == lane_index);
    let record = binding.and_then(|binding| {
        configuration
            .records
            .iter()
            .find(|record| record.key.ofe_id == binding.ofe_id)
    });
    DirectSurfaceLiquidErrorContext {
        transaction_id: Some(owner.transaction_id()),
        owner_id: Some(owner.hydrology_owner_id().clone()),
        ofe_id: binding.map(|binding| binding.ofe_id.clone()),
        tile_id: record.map(|record| record.key.tile_id.clone()),
        surface_id: record.map(|record| record.key.surface_id.clone()),
        source_id: record.map(|record| record.key.source_id.clone()),
        ..DirectSurfaceLiquidErrorContext::default()
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn validate_receiver_expectations(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let lse_owners = request_batch
        .requests
        .iter()
        .filter(|request| request.key.requesting_component == RequestingComponent::GroundSurface)
        .map(|request| request.key.requesting_owner_id.clone())
        .collect::<BTreeSet<_>>();
    let expected_tiles = configuration
        .records
        .iter()
        .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
        .collect::<Vec<_>>();
    let thermal_tiles = expectations
        .ordered_thermal_layers
        .iter()
        .map(|(identity, _)| identity.clone())
        .collect::<Vec<_>>();
    if request_batch.beginning_lse_state_sha256 != expectations.beginning_lse_state_sha256
        || lse_owners.len() != 1
        || !lse_owners.contains(&expectations.lse_owner_id)
    {
        return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(owner.transaction_id()),
                owner_id: Some(expectations.lse_owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(receiver_expectations_sha256(expectations)),
            "independent LSE receiver expectations",
        )
        .into());
    }
    if owner.hydrology_owner_id() != &expectations.hydrology_owner_id
        || beginning_hydrology_snapshot_sha256 != &expectations.beginning_hydrology_snapshot_sha256
    {
        return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(owner.transaction_id()),
                owner_id: Some(expectations.hydrology_owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(receiver_expectations_sha256(expectations)),
            "independent hydrology receiver expectations",
        )
        .into());
    }
    if expectations.soil_thermal_owner_id == expectations.lse_owner_id
        || expectations.soil_thermal_owner_id == expectations.hydrology_owner_id
        || expectations.beginning_soil_thermal_state_sha256
            == expectations.beginning_lse_state_sha256
        || expectations.beginning_soil_thermal_state_sha256
            == expectations.beginning_hydrology_snapshot_sha256
    {
        return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(owner.transaction_id()),
                owner_id: Some(expectations.soil_thermal_owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(receiver_expectations_sha256(expectations)),
            "independent soil-thermal receiver expectation lineage",
        )
        .into());
    }
    if let Some(violation) = first_expected_identity_violation(
        &expected_tiles,
        &thermal_tiles,
        OwnerKind::SoilThermal,
        &expectations.soil_thermal_owner_id,
        "independent soil-thermal receiver expectation topology",
    ) {
        return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(owner.transaction_id()),
                owner_id: violation.owner_id,
                ofe_id: violation.ofe_id,
                tile_id: violation.tile_id,
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(receiver_expectations_sha256(expectations)),
            violation.detail,
        )
        .into());
    }
    for ((ofe_id, tile_id), layers) in &expectations.ordered_thermal_layers {
        let configured_infiltration_layer = configuration
            .ofe_bindings
            .iter()
            .find(|binding| &binding.ofe_id == ofe_id)
            .map(|binding| &binding.infiltration_soil_thermal_layer_id);
        if configured_infiltration_layer != layers.first() {
            return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(owner.transaction_id()),
                    owner_id: Some(expectations.soil_thermal_owner_id.clone()),
                    ofe_id: Some(ofe_id.clone()),
                    tile_id: Some(tile_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                Some(beginning_hydrology_snapshot_sha256.to_string()),
                Some(receiver_expectations_sha256(expectations)),
                "soil-thermal infiltration layer is not the configured first layer",
            )
            .into());
        }
    }
    Ok(())
}

fn validate_final_protocol(
    protocol: &WaterProtocol,
    arbitration: &UnifiedRealHydrologyArbitration,
    expected_snapshot: &Sha256Digest,
    expected_owner: &openwepp_kernel_contract::ResourceOwnerId,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let attempted_sha256 = water_protocol_sha256(protocol);
    if protocol.transaction_id != arbitration.transaction_id
        || &protocol.hydrology_owner_id != expected_owner
        || &protocol.beginning_snapshot_sha256 != expected_snapshot
        || protocol.requests != arbitration.requests
        || protocol.authorizations != arbitration.authorizations
    {
        return Err(protocol_failure(
            DirectSurfaceLiquidErrorCode::E002,
            protocol,
            expected_snapshot,
            &attempted_sha256,
            "final water protocol lineage or D/A identity",
        ));
    }
    preflight_protocol_identities(protocol, expected_snapshot, &attempted_sha256)?;
    preflight_protocol_domains(protocol, expected_snapshot, &attempted_sha256)?;
    preflight_protocol_cardinality(protocol, expected_snapshot, &attempted_sha256)?;
    preflight_protocol_bounds(protocol, expected_snapshot, &attempted_sha256)?;
    if let Err(error) = protocol.validate() {
        let (code, detail) = protocol_error_code_and_detail(&error);
        return Err(protocol_failure(
            code,
            protocol,
            expected_snapshot,
            &attempted_sha256,
            detail,
        ));
    }
    Ok(())
}

pub(super) fn partition_requests(
    batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_sha256: &Sha256Digest,
) -> Result<(Vec<MixedRealHydrologyRequest>, Vec<WaterAmount>), LandSurfaceEnergyShadowError> {
    let mut soil = Vec::new();
    let mut surface = Vec::new();
    let mut consumed_soil_keys = BTreeSet::new();
    for request in &batch.requests {
        match request.key.source_type {
            WaterSourceType::SoilLayerLiquid => {
                let source = soil_sources.get(&request.key).ok_or_else(|| {
                    request_failure(
                        DirectSurfaceLiquidErrorCode::E002,
                        batch,
                        beginning_sha256,
                        Some(&request.key),
                        "missing soil source mapping",
                    )
                })?;
                if request.key.soil_layer_id.as_ref() != Some(&source.layer_id) {
                    return Err(request_failure(
                        DirectSurfaceLiquidErrorCode::E002,
                        batch,
                        beginning_sha256,
                        Some(&request.key),
                        "mixed source identity",
                    ));
                }
                let binding = configuration
                    .ofe_bindings
                    .iter()
                    .find(|binding| binding.ofe_id == request.key.ofe_id);
                if !binding.is_some_and(|binding| {
                    source.ofe_lane.lane_index == binding.production_lane_index
                        && source.ofe_lane.lane_id == binding.production_lane_id
                        && binding.ordered_soil_layer_ids.contains(&source.layer_id)
                }) {
                    return Err(request_failure(
                        DirectSurfaceLiquidErrorCode::E002,
                        batch,
                        beginning_sha256,
                        Some(&request.key),
                        "soil source is not an exact configured OFE/lane/layer member",
                    ));
                }
                consumed_soil_keys.insert(request.key.clone());
                soil.push(MixedRealHydrologyRequest {
                    request: request.clone(),
                    source: source.clone(),
                });
            }
            WaterSourceType::SurfaceLiquid | WaterSourceType::LitterLiquid => {
                if soil_sources.contains_key(&request.key) {
                    return Err(request_failure(
                        DirectSurfaceLiquidErrorCode::E002,
                        batch,
                        beginning_sha256,
                        Some(&request.key),
                        "surface request has soil mapping",
                    ));
                }
                let exact_configured_store = configuration.records.iter().any(|record| {
                    request.key.requesting_tile_id == record.key.tile_id
                        && request.key.ofe_id == record.key.ofe_id
                        && request.key.source_tile_id.as_ref() == Some(&record.key.tile_id)
                        && request.key.surface_id.as_ref() == Some(&record.key.surface_id)
                        && request.key.surface_class == Some(record.key.surface_class)
                        && request.key.source_type == record.key.source_type
                        && request.key.source_id == record.key.source_id
                });
                if !exact_configured_store {
                    return Err(request_failure(
                        DirectSurfaceLiquidErrorCode::E002,
                        batch,
                        beginning_sha256,
                        Some(&request.key),
                        "surface request has no exact configured store",
                    ));
                }
                surface.push(request.clone());
            }
        }
    }
    if consumed_soil_keys.len() != soil_sources.len() {
        return Err(request_failure(
            DirectSurfaceLiquidErrorCode::E002,
            batch,
            beginning_sha256,
            None,
            "unused soil source mapping",
        ));
    }
    Ok((soil, surface))
}

fn restore_authorization_order(
    batch: &PotentialWaterRequestBatch,
    soil: &MixedRealHydrologyArbitration,
    surface: &DirectSurfaceLiquidArbitration,
    beginning_sha256: &Sha256Digest,
) -> Result<Vec<WaterAuthorization>, LandSurfaceEnergyShadowError> {
    let by_key = soil
        .authorizations
        .iter()
        .map(|row| (row.authorization.key.clone(), row.authorization.clone()))
        .chain(
            surface
                .authorizations()
                .iter()
                .map(|row| (row.key.clone(), row.clone())),
        )
        .collect::<BTreeMap<_, _>>();
    if by_key.len() != batch.requests.len() {
        return Err(request_failure(
            DirectSurfaceLiquidErrorCode::E005,
            batch,
            beginning_sha256,
            None,
            "incomplete unified authorization",
        ));
    }
    batch
        .requests
        .iter()
        .map(|request| {
            by_key.get(&request.key).cloned().ok_or_else(|| {
                request_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    batch,
                    beginning_sha256,
                    Some(&request.key),
                    "authorization order identity",
                )
            })
        })
        .collect()
}

fn construct_unified_candidate(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    arbitration: UnifiedRealHydrologyArbitration,
    finalized: UnifiedLseFinalization,
    ingress: &DirectSurfaceLiquidIngressInput,
) -> Result<UnifiedRealHydrologyCandidate, LandSurfaceEnergyShadowError> {
    let UnifiedLseFinalization {
        water_protocol,
        mut ending_tile_states_pre_ingress,
        mut soil_thermal_candidates,
        rollback_hashes,
    } = finalized;
    let (soil_uses, surface_uses) =
        partition_finalized_uses(&arbitration, &water_protocol.finalized_uses)?;
    let soil_candidate =
        soil_adapter.candidate_from_finalized_uses(&arbitration.soil, &soil_uses)?;
    let surface_resource = apply_surface_liquid_resource_phase(
        surface_configuration,
        &arbitration.surface,
        &surface_uses,
        &water_protocol.condensation_credits,
    )?;
    let surface_ingress =
        execute_surface_liquid_ingress(surface_configuration, &surface_resource, ingress)?;
    let mut ending_frame = soil_candidate.ending_frame().clone();
    let receiver_closure_operands = apply_ingress_to_real_receivers(
        soil_adapter.owner,
        surface_configuration,
        receiver_expectations,
        request_batch,
        &surface_ingress,
        &mut ending_frame,
        &mut ending_tile_states_pre_ingress,
        &mut soil_thermal_candidates,
        &rollback_hashes,
        &water_protocol.beginning_snapshot_sha256,
    )?;
    ending_frame.surface_liquid_shadow = Some(Box::new(surface_ingress.ending_state().clone()));
    let candidate = UnifiedRealHydrologyCandidate {
        transaction_id: arbitration.transaction_id,
        beginning_frame: soil_candidate.beginning_frame().clone(),
        ending_frame,
        arbitration,
        finalized_uses: water_protocol.finalized_uses,
        condensation_credits: water_protocol.condensation_credits,
        surface_resource,
        surface_ingress,
        ending_lse_tile_states: ending_tile_states_pre_ingress,
        soil_thermal_candidates,
        receiver_closure_operands,
        rollback_hashes,
    };
    candidate.validate(surface_configuration)?;
    Ok(candidate)
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn apply_ingress_to_real_receivers(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    ingress: &DirectSurfaceLiquidIngressCandidate,
    ending_frame: &mut DirectRunFrame,
    lse_tiles: &mut [TileState],
    soil_thermal: &mut [SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
) -> Result<RealReceiverClosureOperands, LandSurfaceEnergyShadowError> {
    validate_surface_production_binding(owner, configuration)?;
    let receiver_attempt_sha256 =
        finalization_receiver_sets_sha256(lse_tiles, soil_thermal, rollback_hashes);
    receiver_validation::preflight_finalization_receiver_numerics(
        ingress.transaction_id(),
        configuration,
        receiver_expectations,
        lse_tiles,
        soil_thermal,
        rollback_hashes,
        &receiver_attempt_sha256,
    )?;
    receiver_preflight::preflight_receiver_derived_arithmetic(
        owner,
        configuration,
        receiver_expectations,
        ingress,
        ending_frame,
        lse_tiles,
        soil_thermal,
        rollback_hashes,
        &receiver_attempt_sha256,
    )?;
    validate_receiver_expectations(
        owner,
        configuration,
        receiver_expectations,
        request_batch,
        beginning_hydrology_snapshot_sha256,
    )?;
    validate_receiver_sets(
        configuration,
        receiver_expectations,
        lse_tiles,
        soil_thermal,
    )
    .map_err(|violation| {
        receiver_envelope_failure(
            ingress.transaction_id(),
            configuration,
            &violation,
            rollback_hashes,
            &receiver_attempt_sha256,
        )
    })?;
    validate_rollback_joins(
        owner,
        receiver_expectations,
        soil_thermal,
        rollback_hashes,
        beginning_hydrology_snapshot_sha256,
    )
    .map_err(|violation| {
        receiver_envelope_failure(
            ingress.transaction_id(),
            configuration,
            &violation,
            rollback_hashes,
            &receiver_attempt_sha256,
        )
    })?;
    let beginning_frame = ending_frame.clone();
    let beginning_lse_tiles = lse_tiles.to_vec();
    let beginning_soil_thermal = soil_thermal.to_vec();
    let failure_scope = ReceiverFailureScope {
        transaction_id: ingress.transaction_id(),
        configuration,
        expectations: receiver_expectations,
        hydrology_owner_id: owner.hydrology_owner_id(),
        rollback_hashes,
        attempted_sha256: &receiver_attempt_sha256,
    };
    let mut infiltration_m_by_lane =
        BTreeMap::<usize, (f64, &DirectSurfaceLiquidParcelReceipt)>::new();
    for receipt in ingress.receipts() {
        if let Some((lane_index, infiltration_m)) =
            apply_receiver_receipt(&failure_scope, receipt, lse_tiles, soil_thermal)?
        {
            let accumulated = infiltration_m_by_lane
                .entry(lane_index)
                .or_insert((0.0, receipt));
            accumulated.0 =
                checked_surface_liquid_add(accumulated.0, infiltration_m).ok_or_else(|| {
                    receiver_phase_arithmetic_failure(
                        ingress.transaction_id(),
                        Some(OwnerKind::Hydrology),
                        owner.hydrology_owner_id(),
                        receipt,
                        rollback_hashes,
                        &receiver_attempt_sha256,
                        "infiltration lane accumulation is nonfinite or underflowed",
                    )
                })?;
            accumulated.1 = receipt;
        }
    }
    apply_production_infiltration(owner, &failure_scope, ending_frame, infiltration_m_by_lane)?;
    let operands = freeze_real_receiver_closure_operands(
        owner,
        configuration,
        receiver_expectations,
        ingress,
        &beginning_frame,
        ending_frame,
        &beginning_lse_tiles,
        lse_tiles,
        &beginning_soil_thermal,
        soil_thermal,
        rollback_hashes,
        beginning_hydrology_snapshot_sha256,
    )?;
    validate_real_receiver_closure(&operands)?;
    Ok(operands)
}

fn validate_receiver_sets(
    configuration: &DirectSurfaceLiquidConfiguration,
    expectations: &UnifiedReceiverExpectations,
    lse_tiles: &[TileState],
    soil_thermal: &[SoilThermalTileCandidate],
) -> Result<(), ReceiverEnvelopeViolation> {
    let expected_tiles = configuration
        .records
        .iter()
        .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
        .collect::<Vec<_>>();
    validate_receiver_topologies(&expected_tiles, expectations, lse_tiles, soil_thermal)?;
    for candidate in soil_thermal {
        validate_thermal_receiver(configuration, expectations, candidate)?;
    }
    Ok(())
}

fn validate_receiver_topologies(
    expected_tiles: &[(OfeId, TileId)],
    expectations: &UnifiedReceiverExpectations,
    lse_tiles: &[TileState],
    soil_thermal: &[SoilThermalTileCandidate],
) -> Result<(), ReceiverEnvelopeViolation> {
    let expected_expectations = expectations
        .ordered_thermal_layers
        .iter()
        .map(|(identity, _)| identity.clone())
        .collect::<Vec<_>>();
    if let Some(violation) = first_expected_identity_violation(
        expected_tiles,
        &expected_expectations,
        OwnerKind::SoilThermal,
        &expectations.soil_thermal_owner_id,
        "independent soil-thermal expectation topology mismatch",
    ) {
        return Err(violation);
    }
    let actual_lse_tiles = lse_tiles
        .iter()
        .map(|tile| (tile.ofe_id.clone(), tile.tile_id.clone()))
        .collect::<Vec<_>>();
    if let Some(violation) = first_expected_identity_violation(
        expected_tiles,
        &actual_lse_tiles,
        OwnerKind::LandSurfaceEnergy,
        &expectations.lse_owner_id,
        "LSE tile receiver topology mismatch",
    ) {
        return Err(violation);
    }
    let actual_thermal_tiles = soil_thermal
        .iter()
        .map(|tile| (tile.ofe_id.clone(), tile.tile_id.clone()))
        .collect::<Vec<_>>();
    if let Some(index) = first_identity_mismatch(expected_tiles, &actual_thermal_tiles) {
        let missing_expected = if actual_thermal_tiles.len() < expected_tiles.len() {
            let actual_membership = actual_thermal_tiles.iter().collect::<BTreeSet<_>>();
            expected_tiles
                .iter()
                .enumerate()
                .find(|(_, identity)| !actual_membership.contains(identity))
                .map(|(index, _)| index)
        } else {
            None
        };
        let violation = if let Some(missing_index) = missing_expected {
            let (ofe_id, tile_id) = &expected_tiles[missing_index];
            ReceiverEnvelopeViolation::for_tile(
                OwnerKind::SoilThermal,
                Some(expectations.soil_thermal_owner_id.clone()),
                ofe_id.clone(),
                tile_id.clone(),
                "missing soil-thermal tile receiver",
            )
        } else if let Some(candidate) = soil_thermal.get(index) {
            ReceiverEnvelopeViolation::for_tile(
                OwnerKind::SoilThermal,
                Some(candidate.owner_id.clone()),
                candidate.ofe_id.clone(),
                candidate.tile_id.clone(),
                "soil-thermal tile receiver topology mismatch",
            )
        } else {
            let (ofe_id, tile_id) = &expected_tiles[index];
            ReceiverEnvelopeViolation::for_tile(
                OwnerKind::SoilThermal,
                Some(expectations.soil_thermal_owner_id.clone()),
                ofe_id.clone(),
                tile_id.clone(),
                "missing soil-thermal tile receiver",
            )
        };
        return Err(violation);
    }
    Ok(())
}

fn validate_thermal_receiver(
    configuration: &DirectSurfaceLiquidConfiguration,
    expectations: &UnifiedReceiverExpectations,
    candidate: &SoilThermalTileCandidate,
) -> Result<(), ReceiverEnvelopeViolation> {
    let violation = ReceiverEnvelopeViolation::for_tile(
        OwnerKind::SoilThermal,
        Some(candidate.owner_id.clone()),
        candidate.ofe_id.clone(),
        candidate.tile_id.clone(),
        "invalid soil-thermal receiver row",
    );
    let Some(binding) = configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.ofe_id == candidate.ofe_id)
    else {
        return Err(violation);
    };
    let Some(expected_layers) = expectations
        .ordered_thermal_layers
        .iter()
        .find(|(identity, _)| identity == &(candidate.ofe_id.clone(), candidate.tile_id.clone()))
        .map(|(_, layers)| layers)
    else {
        return Err(violation);
    };
    let layers = candidate
        .layers
        .iter()
        .map(|layer| layer.layer_id.clone())
        .collect::<Vec<_>>();
    let has_nonfinite = candidate.layers.iter().any(|layer| {
        !layer.beginning_enthalpy_j_m2_ofe_ground.is_finite()
            || !layer.ground_heat_credit_j_m2_ofe_ground.is_finite()
            || !layer
                .infiltration_enthalpy_credit_j_m2_ofe_ground
                .is_finite()
            || !layer.ending_enthalpy_j_m2_ofe_ground.is_finite()
            || !layer.ending_temperature_k.is_finite()
    });
    let has_duplicate_layer = layers.iter().collect::<BTreeSet<_>>().len() != layers.len();
    if candidate.owner_id != expectations.soil_thermal_owner_id
        || candidate.beginning_state_sha256 != expectations.beginning_soil_thermal_state_sha256
        || layers != *expected_layers
        || has_nonfinite
        || candidate.layers.is_empty()
        || candidate.layers[0].layer_id != binding.infiltration_soil_thermal_layer_id
        || has_duplicate_layer
    {
        return Err(violation);
    }
    Ok(())
}

fn first_identity_mismatch(
    expected: &[(OfeId, TileId)],
    actual: &[(OfeId, TileId)],
) -> Option<usize> {
    (0..expected.len().max(actual.len())).find(|&index| expected.get(index) != actual.get(index))
}

fn first_expected_identity_violation(
    expected: &[(OfeId, TileId)],
    actual: &[(OfeId, TileId)],
    owner_kind: OwnerKind,
    owner_id: &ResourceOwnerId,
    detail: &'static str,
) -> Option<ReceiverEnvelopeViolation> {
    if actual.len() < expected.len() {
        let actual_membership = actual.iter().collect::<BTreeSet<_>>();
        if let Some((ofe_id, tile_id)) = expected
            .iter()
            .find(|identity| !actual_membership.contains(identity))
        {
            return Some(ReceiverEnvelopeViolation::for_tile(
                owner_kind,
                Some(owner_id.clone()),
                ofe_id.clone(),
                tile_id.clone(),
                detail,
            ));
        }
    }
    let index = first_identity_mismatch(expected, actual)?;
    let (ofe_id, tile_id) = actual.get(index).or_else(|| expected.get(index))?;
    Some(ReceiverEnvelopeViolation::for_tile(
        owner_kind,
        Some(owner_id.clone()),
        ofe_id.clone(),
        tile_id.clone(),
        detail,
    ))
}

fn validate_rollback_joins(
    owner: &RealHydrologyShadowAdapter,
    expectations: &UnifiedReceiverExpectations,
    soil_thermal: &[SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
) -> Result<(), ReceiverEnvelopeViolation> {
    let expected = [
        (
            OwnerKind::LandSurfaceEnergy,
            expectations.lse_owner_id.as_str(),
            &expectations.beginning_lse_state_sha256,
        ),
        (
            OwnerKind::Hydrology,
            owner.hydrology_owner_id().as_str(),
            beginning_hydrology_snapshot_sha256,
        ),
        (
            OwnerKind::SoilThermal,
            expectations.soil_thermal_owner_id.as_str(),
            &expectations.beginning_soil_thermal_state_sha256,
        ),
    ];
    if rollback_hashes.len() < expected.len() {
        for (kind, owner_id, _) in expected {
            let expected_present = rollback_hashes
                .iter()
                .any(|actual| actual.owner_kind == kind && actual.owner_id == owner_id);
            if !expected_present {
                return Err(ReceiverEnvelopeViolation::for_owner(
                    kind,
                    ResourceOwnerId::try_new(owner_id.to_owned()).ok(),
                    missing_rollback_detail(kind),
                ));
            }
        }
    }
    for index in 0..rollback_hashes.len().max(expected.len()) {
        let actual = rollback_hashes.get(index);
        let expected_row = expected.get(index);
        match (actual, expected_row) {
            (Some(actual), Some((kind, owner_id, beginning)))
                if actual.owner_kind != *kind
                    || actual.owner_id != *owner_id
                    || &actual.before_sha256 != *beginning
                    || &actual.after_sha256 != *beginning =>
            {
                return Err(rollback_violation(actual, "rollback owner row mismatch"));
            }
            (Some(actual), None) => {
                return Err(rollback_violation(actual, "unexpected rollback owner row"));
            }
            (None, Some((kind, owner_id, _))) => {
                return Err(ReceiverEnvelopeViolation::for_owner(
                    *kind,
                    ResourceOwnerId::try_new((*owner_id).to_owned()).ok(),
                    missing_rollback_detail(*kind),
                ));
            }
            _ => {}
        }
    }
    if let Some(candidate) = soil_thermal.iter().find(|candidate| {
        candidate.owner_id != expectations.soil_thermal_owner_id
            || candidate.beginning_state_sha256 != expectations.beginning_soil_thermal_state_sha256
    }) {
        return Err(ReceiverEnvelopeViolation::for_tile(
            OwnerKind::SoilThermal,
            Some(candidate.owner_id.clone()),
            candidate.ofe_id.clone(),
            candidate.tile_id.clone(),
            "soil-thermal rollback lineage mismatch",
        ));
    }
    Ok(())
}

fn rollback_violation(
    row: &OwnerRollbackHash,
    fallback: &'static str,
) -> ReceiverEnvelopeViolation {
    let detail = match row.owner_kind {
        OwnerKind::LandSurfaceEnergy => "LSE rollback owner row mismatch",
        OwnerKind::SoilThermal => "soil-thermal rollback owner row mismatch",
        OwnerKind::Hydrology => "hydrology rollback owner row mismatch",
        OwnerKind::Vegetation => "vegetation rollback owner row mismatch",
        OwnerKind::Biogeochemistry => "biogeochemistry rollback owner row mismatch",
        OwnerKind::Envelope => fallback,
    };
    ReceiverEnvelopeViolation::for_owner(
        row.owner_kind,
        ResourceOwnerId::try_new(row.owner_id.clone()).ok(),
        detail,
    )
}

const fn missing_rollback_detail(kind: OwnerKind) -> &'static str {
    match kind {
        OwnerKind::LandSurfaceEnergy => "missing LSE rollback owner row",
        OwnerKind::Hydrology => "missing hydrology rollback owner row",
        OwnerKind::SoilThermal => "missing soil-thermal rollback owner row",
        OwnerKind::Vegetation => "missing vegetation rollback owner row",
        OwnerKind::Biogeochemistry => "missing biogeochemistry rollback owner row",
        OwnerKind::Envelope => "missing envelope rollback owner row",
    }
}

fn receiver_envelope_failure(
    transaction_id: TransactionId,
    configuration: &DirectSurfaceLiquidConfiguration,
    violation: &ReceiverEnvelopeViolation,
    rollback_hashes: &[OwnerRollbackHash],
    attempted_sha256: &str,
) -> LandSurfaceEnergyShadowError {
    let record = violation.ofe_id.as_ref().and_then(|ofe_id| {
        configuration.records.iter().find(|record| {
            &record.key.ofe_id == ofe_id
                && violation
                    .tile_id
                    .as_ref()
                    .is_some_and(|tile_id| &record.key.tile_id == tile_id)
        })
    });
    canonical_receiver_failure(
        DirectSurfaceLiquidErrorCode::E011,
        DirectSurfaceLiquidPhase::AtomicEnvelope,
        transaction_id,
        violation.owner_kind,
        violation.owner_id.as_ref(),
        violation.ofe_id.as_ref(),
        violation.tile_id.as_ref(),
        record.map(|record| record.key.surface_id.clone()),
        record.map(|record| record.key.source_id.clone()),
        None,
        rollback_hashes,
        attempted_sha256,
        violation.detail,
    )
    .into()
}

fn receiver_phase_arithmetic_failure(
    transaction_id: TransactionId,
    owner_kind: Option<OwnerKind>,
    owner_id: &ResourceOwnerId,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    rollback_hashes: &[OwnerRollbackHash],
    attempted_sha256: &str,
    detail: &'static str,
) -> LandSurfaceEnergyShadowError {
    canonical_receiver_failure(
        DirectSurfaceLiquidErrorCode::E003,
        DirectSurfaceLiquidPhase::IndependentClosure,
        transaction_id,
        owner_kind,
        Some(owner_id),
        Some(&receipt.recipient_store_key.ofe_id),
        Some(&receipt.recipient_store_key.tile_id),
        Some(receipt.recipient_store_key.surface_id.clone()),
        Some(receipt.recipient_store_key.source_id.clone()),
        Some(receipt.parcel_id.clone()),
        rollback_hashes,
        attempted_sha256,
        detail,
    )
    .into()
}

struct ReceiverFailureScope<'a> {
    transaction_id: TransactionId,
    configuration: &'a DirectSurfaceLiquidConfiguration,
    expectations: &'a UnifiedReceiverExpectations,
    hydrology_owner_id: &'a ResourceOwnerId,
    rollback_hashes: &'a [OwnerRollbackHash],
    attempted_sha256: &'a str,
}

impl ReceiverFailureScope<'_> {
    fn owner_kind(&self, owner_id: &ResourceOwnerId) -> Option<OwnerKind> {
        if owner_id == &self.expectations.lse_owner_id {
            return Some(OwnerKind::LandSurfaceEnergy);
        }
        if owner_id == &self.expectations.soil_thermal_owner_id {
            return Some(OwnerKind::SoilThermal);
        }
        (owner_id == self.hydrology_owner_id).then_some(OwnerKind::Hydrology)
    }

    fn failure(
        &self,
        code: DirectSurfaceLiquidErrorCode,
        owner_id: &ResourceOwnerId,
        receipt: &DirectSurfaceLiquidParcelReceipt,
        detail: &'static str,
    ) -> LandSurfaceEnergyShadowError {
        canonical_receiver_failure(
            code,
            DirectSurfaceLiquidPhase::IndependentClosure,
            self.transaction_id,
            self.owner_kind(owner_id),
            Some(owner_id),
            Some(&receipt.recipient_store_key.ofe_id),
            Some(&receipt.recipient_store_key.tile_id),
            Some(receipt.recipient_store_key.surface_id.clone()),
            Some(receipt.recipient_store_key.source_id.clone()),
            Some(receipt.parcel_id.clone()),
            self.rollback_hashes,
            self.attempted_sha256,
            detail,
        )
        .into()
    }
}

fn apply_receiver_receipt(
    scope: &ReceiverFailureScope<'_>,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    lse_tiles: &mut [TileState],
    soil_thermal: &mut [SoilThermalTileCandidate],
) -> Result<Option<(usize, f64)>, LandSurfaceEnergyShadowError> {
    let binding = scope
        .configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.ofe_id == receipt.recipient_store_key.ofe_id)
        .ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E009,
                &scope.configuration.owner_id,
                receipt,
                "ingress receipt OFE binding",
            )
        })?;
    match (&receipt.disposition, &receipt.recipient) {
        (
            DirectSurfaceLiquidReceiptDisposition::Infiltration,
            DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
                ofe_id,
                production_lane_index,
                production_lane_id,
                ordered_soil_layer_ids,
                soil_thermal_layer_id,
            },
        ) => credit_infiltration_receipt(
            binding,
            receipt,
            soil_thermal,
            ofe_id,
            *production_lane_index,
            *production_lane_id,
            ordered_soil_layer_ids,
            soil_thermal_layer_id,
            scope,
        )
        .map(Some),
        (
            DirectSurfaceLiquidReceiptDisposition::RetainedSurface,
            DirectSurfaceLiquidReceiptRecipient::SurfaceStore { store_key },
        ) => credit_retained_receipt(scope, receipt, store_key, lse_tiles).map(|()| None),
        (
            DirectSurfaceLiquidReceiptDisposition::RoutedRunoff,
            DirectSurfaceLiquidReceiptRecipient::RoutedOfe { .. },
        )
        | (
            DirectSurfaceLiquidReceiptDisposition::OutletRunoff,
            DirectSurfaceLiquidReceiptRecipient::Outlet { .. },
        ) => Ok(None),
        _ => Err(scope.failure(
            DirectSurfaceLiquidErrorCode::E009,
            &scope.configuration.owner_id,
            receipt,
            "receipt disposition/recipient join",
        )),
    }
}

#[allow(clippy::too_many_arguments)]
fn credit_infiltration_receipt(
    binding: &DirectSurfaceLiquidOfeBinding,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    soil_thermal: &mut [SoilThermalTileCandidate],
    ofe_id: &OfeId,
    production_lane_index: usize,
    production_lane_id: u32,
    ordered_soil_layer_ids: &[openwepp_kernel_contract::SoilLayerId],
    soil_thermal_layer_id: &openwepp_kernel_contract::SoilLayerId,
    scope: &ReceiverFailureScope<'_>,
) -> Result<(usize, f64), LandSurfaceEnergyShadowError> {
    if ofe_id != &binding.ofe_id
        || production_lane_index != binding.production_lane_index
        || production_lane_id != binding.production_lane_id
        || ordered_soil_layer_ids != binding.ordered_soil_layer_ids
        || soil_thermal_layer_id != &binding.infiltration_soil_thermal_layer_id
    {
        return Err(scope.failure(
            DirectSurfaceLiquidErrorCode::E009,
            &scope.expectations.soil_thermal_owner_id,
            receipt,
            "infiltration receipt production receiver binding",
        ));
    }
    let thermal = soil_thermal
        .iter_mut()
        .find(|candidate| {
            candidate.ofe_id == receipt.recipient_store_key.ofe_id
                && candidate.tile_id == receipt.recipient_store_key.tile_id
        })
        .ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E010,
                &scope.expectations.soil_thermal_owner_id,
                receipt,
                "missing infiltration soil-thermal tile receiver",
            )
        })?;
    let layer = thermal
        .layers
        .iter_mut()
        .find(|layer| layer.layer_id == binding.infiltration_soil_thermal_layer_id)
        .ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E010,
                &scope.expectations.soil_thermal_owner_id,
                receipt,
                "missing infiltration soil-thermal layer receiver",
            )
        })?;
    layer.infiltration_enthalpy_credit_j_m2_ofe_ground = checked_receiver_credit_add(
        layer.infiltration_enthalpy_credit_j_m2_ofe_ground,
        receipt.enthalpy_j_m2_basis_ofe_ground,
    )
    .ok_or_else(|| {
        scope.failure(
            DirectSurfaceLiquidErrorCode::E003,
            &scope.expectations.soil_thermal_owner_id,
            receipt,
            "soil-thermal infiltration enthalpy arithmetic",
        )
    })?;
    layer.ending_enthalpy_j_m2_ofe_ground = checked_receiver_credit_add(
        layer.ending_enthalpy_j_m2_ofe_ground,
        receipt.enthalpy_j_m2_basis_ofe_ground,
    )
    .ok_or_else(|| {
        scope.failure(
            DirectSurfaceLiquidErrorCode::E003,
            &scope.expectations.soil_thermal_owner_id,
            receipt,
            "soil-thermal ending enthalpy arithmetic",
        )
    })?;
    let infiltration_m =
        checked_surface_liquid_div(receipt.mass_kg_m2_basis_ofe_ground, WATER_DENSITY_KG_M3)
            .ok_or_else(|| {
                scope.failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    &scope.expectations.soil_thermal_owner_id,
                    receipt,
                    "infiltration mass-to-depth arithmetic",
                )
            })?;
    Ok((binding.production_lane_index, infiltration_m))
}

fn credit_retained_receipt(
    scope: &ReceiverFailureScope<'_>,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    store_key: &crate::DirectSurfaceLiquidStoreKey,
    lse_tiles: &mut [TileState],
) -> Result<(), LandSurfaceEnergyShadowError> {
    if store_key != &receipt.recipient_store_key {
        return Err(scope.failure(
            DirectSurfaceLiquidErrorCode::E009,
            &scope.configuration.owner_id,
            receipt,
            "retained receipt typed receiver",
        ));
    }
    let tile = lse_tiles
        .iter_mut()
        .find(|tile| {
            tile.ofe_id == receipt.recipient_store_key.ofe_id
                && tile.tile_id == receipt.recipient_store_key.tile_id
        })
        .ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E010,
                &scope.expectations.lse_owner_id,
                receipt,
                "missing retained LSE tile receiver",
            )
        })?;
    let record = scope
        .configuration
        .records
        .iter()
        .find(|record| record.key == receipt.recipient_store_key)
        .ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E009,
                &scope.configuration.owner_id,
                receipt,
                "retained receipt store receiver",
            )
        })?;
    let retained_tile =
        checked_surface_liquid_div(receipt.enthalpy_j_m2_basis_ofe_ground, record.tile_fraction)
            .ok_or_else(|| {
                scope.failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    &scope.expectations.lse_owner_id,
                    receipt,
                    "retained enthalpy OFE-to-tile arithmetic",
                )
            })?;
    tile.surface_enthalpy_j_m2_tile_ground =
        checked_receiver_credit_add(tile.surface_enthalpy_j_m2_tile_ground, retained_tile)
            .ok_or_else(|| {
                scope.failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    &scope.expectations.lse_owner_id,
                    receipt,
                    "retained surface enthalpy arithmetic",
                )
            })?;
    Ok(())
}

fn checked_receiver_credit_add(beginning: f64, credit: f64) -> Option<f64> {
    let ending = checked_surface_liquid_add(beginning, credit)?;
    (credit == 0.0 || ending.to_bits() != beginning.to_bits()).then_some(ending)
}

fn apply_production_infiltration(
    owner: &RealHydrologyShadowAdapter,
    scope: &ReceiverFailureScope<'_>,
    ending_frame: &mut DirectRunFrame,
    infiltration_m_by_lane: BTreeMap<usize, (f64, &DirectSurfaceLiquidParcelReceipt)>,
) -> Result<(), LandSurfaceEnergyShadowError> {
    for (lane_index, (infiltration_m, receipt)) in infiltration_m_by_lane {
        let failure =
            |code, detail| scope.failure(code, owner.hydrology_owner_id(), receipt, detail);
        let lane = ending_frame.lanes.get_mut(lane_index).ok_or_else(|| {
            failure(
                DirectSurfaceLiquidErrorCode::E010,
                "infiltration production lane receiver",
            )
        })?;
        let day = owner
            .beginning_day_frames()
            .get(lane_index)
            .ok_or_else(|| {
                failure(
                    DirectSurfaceLiquidErrorCode::E010,
                    "infiltration production day receiver",
                )
            })?;
        apply_direct_same_pass_infiltration(
            &mut lane.subsurface_layers,
            infiltration_m,
            day.percolation_inputs.tillage_depth_m,
        )
        .map_err(|_| {
            failure(
                DirectSurfaceLiquidErrorCode::E003,
                "production infiltration receiver",
            )
        })?;
        lane.water.soil_water_m = aggregate_direct_soil_water(
            &lane.subsurface_layers,
            "land_surface_energy_shadow.ingress_soil_water",
        )
        .map_err(|_| {
            failure(
                DirectSurfaceLiquidErrorCode::E003,
                "ingress soil reconstruction",
            )
        })?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn freeze_real_receiver_closure_operands(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    ingress: &DirectSurfaceLiquidIngressCandidate,
    beginning_frame: &DirectRunFrame,
    ending_frame: &DirectRunFrame,
    beginning_lse_tiles: &[TileState],
    ending_lse_tiles: &[TileState],
    beginning_soil_thermal: &[SoilThermalTileCandidate],
    ending_soil_thermal: &[SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
) -> Result<RealReceiverClosureOperands, LandSurfaceEnergyShadowError> {
    let amounts = receiver_preflight::aggregate_receiver_receipts(
        ingress.receipts(),
        |owner, receipt, detail| {
            let owner_id = match owner {
                receiver_preflight::ReceiptAggregationOwner::SurfaceLiquid => {
                    &configuration.owner_id
                }
                receiver_preflight::ReceiptAggregationOwner::LandSurfaceEnergy => {
                    &receiver_expectations.lse_owner_id
                }
                receiver_preflight::ReceiptAggregationOwner::SoilThermal => {
                    &receiver_expectations.soil_thermal_owner_id
                }
            };
            let owner_kind = match owner {
                receiver_preflight::ReceiptAggregationOwner::SurfaceLiquid => None,
                receiver_preflight::ReceiptAggregationOwner::LandSurfaceEnergy => {
                    Some(OwnerKind::LandSurfaceEnergy)
                }
                receiver_preflight::ReceiptAggregationOwner::SoilThermal => {
                    Some(OwnerKind::SoilThermal)
                }
            };
            receiver_phase_arithmetic_failure(
                ingress.transaction_id(),
                owner_kind,
                owner_id,
                receipt,
                rollback_hashes,
                &ingress.ending_state().state_sha256,
                detail,
            )
        },
    )?;
    let production_soil = freeze_production_soil_receivers(
        owner,
        configuration,
        beginning_frame,
        ending_frame,
        &amounts.infiltration_m_by_ofe,
    )?;
    let (soil_thermal, lse_tiles) = freeze_energy_receivers(
        configuration,
        beginning_lse_tiles,
        ending_lse_tiles,
        beginning_soil_thermal,
        ending_soil_thermal,
        &amounts.infiltration_enthalpy_by_tile,
        &amounts.retained_enthalpy_by_tile,
    )?;
    let (expected_production_soil, expected_soil_thermal, expected_lse_tiles) =
        receiver_validation::expected_receiver_identities(configuration);
    let configured_surface_context = configuration
        .records
        .iter()
        .map(|record| {
            (
                record.key.ofe_id.clone(),
                record.key.tile_id.clone(),
                record.key.surface_id.clone(),
                record.key.source_id.clone(),
            )
        })
        .collect();
    Ok(RealReceiverClosureOperands {
        transaction_id: ingress.transaction_id(),
        hydrology_owner_id: owner.hydrology_owner_id().clone(),
        lse_owner_id: receiver_expectations.lse_owner_id.clone(),
        soil_thermal_owner_id: receiver_expectations.soil_thermal_owner_id.clone(),
        beginning_hydrology_snapshot_sha256: beginning_hydrology_snapshot_sha256.clone(),
        beginning_lse_state_sha256: receiver_expectations.beginning_lse_state_sha256.clone(),
        beginning_soil_thermal_state_sha256: receiver_expectations
            .beginning_soil_thermal_state_sha256
            .clone(),
        rollback_hashes: rollback_hashes.to_vec(),
        production_soil,
        soil_thermal,
        lse_tiles,
        expected_production_soil,
        expected_soil_thermal,
        expected_lse_tiles,
        configured_surface_context,
    })
}

fn freeze_production_soil_receivers(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_frame: &DirectRunFrame,
    ending_frame: &DirectRunFrame,
    infiltration_m_by_ofe: &receiver_preflight::OfeAmountMap,
) -> Result<Vec<ProductionSoilReceiverOperands>, LandSurfaceEnergyShadowError> {
    let mut production_soil = Vec::with_capacity(configuration.ofe_bindings.len());
    for binding in &configuration.ofe_bindings {
        let beginning_lane = beginning_frame
            .lanes
            .get(binding.production_lane_index)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure beginning lane",
            ))?;
        let ending_lane = ending_frame
            .lanes
            .get(binding.production_lane_index)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure ending lane",
            ))?;
        let day = owner
            .beginning_day_frames()
            .get(binding.production_lane_index)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure production day",
            ))?;
        if beginning_lane.lane_id != binding.production_lane_id
            || ending_lane.lane_id != binding.production_lane_id
            || beginning_lane.subsurface_layers.len() != binding.ordered_soil_layer_ids.len()
            || ending_lane.subsurface_layers.len() != binding.ordered_soil_layer_ids.len()
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "receiver closure lane/layer identity",
            ));
        }
        let ordered_layers = binding
            .ordered_soil_layer_ids
            .iter()
            .zip(&beginning_lane.subsurface_layers)
            .zip(&ending_lane.subsurface_layers)
            .map(
                |((layer_id, beginning), ending)| ProductionSoilLayerReceiverOperands {
                    layer_id: layer_id.clone(),
                    beginning_liquid_m: beginning.theta_m,
                    ending_liquid_m: ending.theta_m,
                    layer_depth_m: beginning.depth_m,
                    residual_theta: beginning.residual_theta,
                    frozen_depth_m: beginning.frozen_depth_m,
                },
            )
            .collect();
        production_soil.push(ProductionSoilReceiverOperands {
            ofe_id: binding.ofe_id.clone(),
            production_lane_index: binding.production_lane_index,
            production_lane_id: binding.production_lane_id,
            tillage_depth_m: day.percolation_inputs.tillage_depth_m,
            infiltration_m: infiltration_m_by_ofe
                .get(&binding.ofe_id)
                .copied()
                .unwrap_or(0.0),
            beginning_aggregate_soil_water_m: beginning_lane.water.soil_water_m,
            ending_aggregate_soil_water_m: ending_lane.water.soil_water_m,
            ordered_layers,
        });
    }
    Ok(production_soil)
}

#[allow(clippy::too_many_arguments)]
fn freeze_energy_receivers(
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_lse_tiles: &[TileState],
    ending_lse_tiles: &[TileState],
    beginning_soil_thermal: &[SoilThermalTileCandidate],
    ending_soil_thermal: &[SoilThermalTileCandidate],
    infiltration_enthalpy_by_tile: &receiver_preflight::TileAmountMap,
    retained_enthalpy_by_tile: &receiver_preflight::TileAmountMap,
) -> Result<
    (
        Vec<SoilThermalReceiverOperands>,
        Vec<LseTileReceiverOperands>,
    ),
    LandSurfaceEnergyShadowError,
> {
    let mut soil_thermal = Vec::new();
    let mut lse_tiles = Vec::new();
    for record in &configuration.records {
        let tile_key = (record.key.ofe_id.clone(), record.key.tile_id.clone());
        let beginning_thermal = beginning_soil_thermal
            .iter()
            .find(|candidate| {
                candidate.ofe_id == record.key.ofe_id && candidate.tile_id == record.key.tile_id
            })
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure beginning soil thermal",
            ))?;
        let ending_thermal = ending_soil_thermal
            .iter()
            .find(|candidate| {
                candidate.ofe_id == record.key.ofe_id && candidate.tile_id == record.key.tile_id
            })
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure ending soil thermal",
            ))?;
        let binding = configuration
            .ofe_bindings
            .iter()
            .find(|binding| binding.ofe_id == record.key.ofe_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure OFE binding",
            ))?;
        let beginning_layer = beginning_thermal
            .layers
            .iter()
            .find(|layer| layer.layer_id == binding.infiltration_soil_thermal_layer_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure beginning thermal layer",
            ))?;
        let ending_layer = ending_thermal
            .layers
            .iter()
            .find(|layer| layer.layer_id == binding.infiltration_soil_thermal_layer_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure ending thermal layer",
            ))?;
        soil_thermal.push(SoilThermalReceiverOperands {
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            layer_id: binding.infiltration_soil_thermal_layer_id.clone(),
            beginning_infiltration_credit_j_m2_ofe_ground: beginning_layer
                .infiltration_enthalpy_credit_j_m2_ofe_ground,
            ending_infiltration_credit_j_m2_ofe_ground: ending_layer
                .infiltration_enthalpy_credit_j_m2_ofe_ground,
            beginning_enthalpy_j_m2_ofe_ground: beginning_layer.ending_enthalpy_j_m2_ofe_ground,
            infiltration_enthalpy_j_m2_ofe_ground: infiltration_enthalpy_by_tile
                .get(&tile_key)
                .copied()
                .unwrap_or(0.0),
            ending_enthalpy_j_m2_ofe_ground: ending_layer.ending_enthalpy_j_m2_ofe_ground,
        });

        let beginning_lse = beginning_lse_tiles
            .iter()
            .find(|tile| tile.ofe_id == record.key.ofe_id && tile.tile_id == record.key.tile_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure beginning LSE tile",
            ))?;
        let ending_lse = ending_lse_tiles
            .iter()
            .find(|tile| tile.ofe_id == record.key.ofe_id && tile.tile_id == record.key.tile_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure ending LSE tile",
            ))?;
        lse_tiles.push(LseTileReceiverOperands {
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            tile_fraction: record.tile_fraction,
            beginning_enthalpy_j_m2_tile_ground: beginning_lse.surface_enthalpy_j_m2_tile_ground,
            retained_enthalpy_j_m2_ofe_ground: retained_enthalpy_by_tile
                .get(&tile_key)
                .copied()
                .unwrap_or(0.0),
            ending_enthalpy_j_m2_tile_ground: ending_lse.surface_enthalpy_j_m2_tile_ground,
        });
    }
    Ok((soil_thermal, lse_tiles))
}

pub use receiver_validation::validate_real_receiver_closure;
fn partition_finalized_uses(
    arbitration: &UnifiedRealHydrologyArbitration,
    finalized_uses: &[WaterAmount],
) -> Result<(Vec<MixedRealHydrologyUse>, Vec<WaterAmount>), LandSurfaceEnergyShadowError> {
    let soil_sources = arbitration
        .soil
        .requests
        .iter()
        .map(|row| (row.request.key.clone(), row.source.clone()))
        .collect::<BTreeMap<_, _>>();
    let surface_keys = arbitration
        .surface
        .requests()
        .iter()
        .map(|row| row.key.clone())
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    let mut soil = Vec::new();
    let mut surface = Vec::new();
    for row in finalized_uses {
        if !seen.insert(row.key.clone()) {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "duplicate unified finalized use",
            ));
        }
        if let Some(source) = soil_sources.get(&row.key) {
            soil.push(MixedRealHydrologyUse {
                finalized_use: row.clone(),
                source: source.clone(),
            });
        } else if surface_keys.contains(&row.key) {
            surface.push(row.clone());
        } else {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "unknown unified finalized use",
            ));
        }
    }
    if seen.len() != arbitration.requests.len() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "incomplete unified finalized use",
        ));
    }
    Ok((soil, surface))
}

impl MixedRealHydrologyCandidate {
    #[must_use]
    pub fn beginning_frame(&self) -> &DirectRunFrame {
        &self.beginning_frame
    }

    #[must_use]
    pub fn ending_frame(&self) -> &DirectRunFrame {
        &self.ending_frame
    }

    #[must_use]
    pub fn finalized_uses(&self) -> &[MixedRealHydrologyUse] {
        &self.finalized_uses
    }

    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }
}

/// Adapter that reuses the production day-frame authorization and debit kernels.
pub struct LandSurfaceEnergyRealHydrologyAdapter<'a> {
    owner: &'a RealHydrologyShadowAdapter,
}

impl<'a> LandSurfaceEnergyRealHydrologyAdapter<'a> {
    #[must_use]
    pub const fn new(owner: &'a RealHydrologyShadowAdapter) -> Self {
        Self { owner }
    }

    pub fn authorize(
        &self,
        requests: &[MixedRealHydrologyRequest],
    ) -> Result<MixedRealHydrologyArbitration, LandSurfaceEnergyShadowError> {
        let mut seen = BTreeSet::new();
        for request in requests {
            self.validate_request(request)?;
            if !seen.insert((request.source.clone(), request.request.key.clone())) {
                return Err(LandSurfaceEnergyShadowError::Identity(
                    "duplicate mixed request",
                ));
            }
        }
        let mut ranked = requests
            .iter()
            .enumerate()
            .filter(|(_, request)| {
                self.owner
                    .layer_facts()
                    .get(&request.source)
                    .is_some_and(|fact| !fact.frozen)
            })
            .collect::<Vec<_>>();
        ranked.sort_by(|(_, left), (_, right)| {
            (&left.source, &left.request.key).cmp(&(&right.source, &right.request.key))
        });
        let canonical = ranked
            .iter()
            .enumerate()
            .map(|(canonical_rank, (_, request))| {
                Ok(DirectLayerWithdrawalRequest {
                    lane_index: request.source.ofe_lane.lane_index,
                    layer_index: self.owner.layer_index_for_source(&request.source)?,
                    canonical_rank,
                    amount_kg_m2: request.request.amount_kg_m2_stand_ground,
                })
            })
            .collect::<Result<Vec<_>, LandSurfaceEnergyShadowError>>()?;
        let amounts =
            authorize_direct_layer_withdrawals(self.owner.beginning_day_frames(), &canonical)
                .map_err(|_| {
                    LandSurfaceEnergyShadowError::Operand("production authorization failed")
                })?;
        let ranked_amounts = ranked
            .iter()
            .zip(amounts)
            .map(|((original_index, _), amount)| (*original_index, amount))
            .collect::<BTreeMap<_, _>>();
        let authorizations = requests
            .iter()
            .enumerate()
            .map(|(index, request)| {
                let amount = ranked_amounts.get(&index).copied().unwrap_or(0.0);
                let fact = self.owner.layer_facts().get(&request.source).ok_or(
                    LandSurfaceEnergyShadowError::Identity("authorization source disappeared"),
                )?;
                let reason = if request.request.amount_kg_m2_stand_ground == 0.0 {
                    WaterAuthorizationReason::ZeroSupply
                } else if fact.frozen {
                    WaterAuthorizationReason::FrozenSource
                } else if amount.to_bits() == request.request.amount_kg_m2_stand_ground.to_bits() {
                    WaterAuthorizationReason::FullSupply
                } else if fact.liquid_supply_kg_m2 == 0.0 {
                    WaterAuthorizationReason::DrySource
                } else {
                    WaterAuthorizationReason::ProportionalSupply
                };
                Ok(MixedRealHydrologyAuthorization {
                    authorization: WaterAuthorization {
                        key: request.request.key.clone(),
                        amount_kg_m2_stand_ground: amount,
                        reason,
                    },
                    source: request.source.clone(),
                })
            })
            .collect::<Result<Vec<_>, LandSurfaceEnergyShadowError>>()?;
        Ok(MixedRealHydrologyArbitration {
            requests: requests.to_vec(),
            authorizations,
            beginning_frame: self.owner.beginning_frame().clone(),
            transaction_id: self.owner.transaction_id(),
        })
    }

    pub fn candidate_from_finalized_uses(
        &self,
        arbitration: &MixedRealHydrologyArbitration,
        finalized_uses: &[MixedRealHydrologyUse],
    ) -> Result<MixedRealHydrologyCandidate, LandSurfaceEnergyShadowError> {
        if arbitration.transaction_id != self.owner.transaction_id()
            || arbitration.beginning_frame != *self.owner.beginning_frame()
            || finalized_uses.len() != arbitration.requests.len()
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "stale mixed arbitration",
            ));
        }
        let debits = Self::validated_debits(arbitration, finalized_uses)?;
        let mut ending = self.owner.beginning_frame().clone();
        for (source, amounts) in debits {
            let debit = canonical_resource_amount_sum(&amounts)
                .map_err(|_| LandSurfaceEnergyShadowError::Bound("aggregate mixed debit"))?;
            let fact = self.owner.layer_facts().get(&source).ok_or(
                LandSurfaceEnergyShadowError::Identity("candidate source disappeared"),
            )?;
            if debit > fact.liquid_supply_kg_m2 {
                return Err(LandSurfaceEnergyShadowError::Bound(
                    "mixed debit exceeds supply",
                ));
            }
            let layer_index = self.owner.layer_index_for_source(&source)?;
            let lane = ending
                .lanes
                .get_mut(source.ofe_lane.lane_index)
                .ok_or(LandSurfaceEnergyShadowError::Identity("mixed debit lane"))?;
            if lane.lane_id != source.ofe_lane.lane_id {
                return Err(LandSurfaceEnergyShadowError::Identity(
                    "mixed debit lane identity",
                ));
            }
            let layer = lane
                .subsurface_layers
                .get_mut(layer_index)
                .ok_or(LandSurfaceEnergyShadowError::Identity("mixed debit layer"))?;
            let debit_m = if debit.to_bits() == fact.liquid_supply_kg_m2.to_bits() {
                layer.theta_m
            } else {
                checked_surface_liquid_div(debit, WATER_DENSITY_KG_M3).ok_or(
                    LandSurfaceEnergyShadowError::Bound("mixed debit mass-to-depth arithmetic"),
                )?
            };
            apply_direct_finalized_layer_liquid_debit(layer, debit_m)
                .map_err(|_| LandSurfaceEnergyShadowError::Bound("production mixed debit"))?;
        }
        for lane in &mut ending.lanes {
            lane.water.soil_water_m = aggregate_direct_soil_water(
                &lane.subsurface_layers,
                "land_surface_energy_shadow.soil_water",
            )
            .map_err(|_| LandSurfaceEnergyShadowError::Bound("mixed ending reconstruction"))?;
        }
        Ok(MixedRealHydrologyCandidate {
            beginning_frame: self.owner.beginning_frame().clone(),
            ending_frame: ending,
            finalized_uses: finalized_uses.to_vec(),
            transaction_id: self.owner.transaction_id(),
        })
    }

    fn validated_debits(
        arbitration: &MixedRealHydrologyArbitration,
        finalized_uses: &[MixedRealHydrologyUse],
    ) -> Result<
        BTreeMap<RealHydrologySourceKey, BTreeMap<GroundWaterKey, f64>>,
        LandSurfaceEnergyShadowError,
    > {
        let requests = arbitration
            .requests
            .iter()
            .map(|row| ((row.source.clone(), row.request.key.clone()), row))
            .collect::<BTreeMap<_, _>>();
        let authorizations = arbitration
            .authorizations
            .iter()
            .map(|row| ((row.source.clone(), row.authorization.key.clone()), row))
            .collect::<BTreeMap<_, _>>();
        let mut seen = BTreeSet::new();
        let mut debits = BTreeMap::<RealHydrologySourceKey, BTreeMap<GroundWaterKey, f64>>::new();
        for row in finalized_uses {
            let identity = (row.source.clone(), row.finalized_use.key.clone());
            if !seen.insert(identity.clone()) {
                return Err(LandSurfaceEnergyShadowError::Identity(
                    "duplicate mixed finalized use",
                ));
            }
            let request = requests
                .get(&identity)
                .ok_or(LandSurfaceEnergyShadowError::Identity(
                    "unknown mixed finalized use",
                ))?;
            let authorization =
                authorizations
                    .get(&identity)
                    .ok_or(LandSurfaceEnergyShadowError::Identity(
                        "missing mixed authorization",
                    ))?;
            let amount = row.finalized_use.amount_kg_m2_stand_ground;
            if !amount.is_finite()
                || amount < 0.0
                || amount > authorization.authorization.amount_kg_m2_stand_ground
                || authorization.authorization.amount_kg_m2_stand_ground
                    > request.request.amount_kg_m2_stand_ground
            {
                return Err(LandSurfaceEnergyShadowError::Bound("mixed F <= A <= D"));
            }
            debits
                .entry(row.source.clone())
                .or_default()
                .insert(row.finalized_use.key.clone(), amount);
        }
        if seen.len() != requests.len() {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "incomplete mixed finalized uses",
            ));
        }
        Ok(debits)
    }

    #[must_use]
    pub fn reject_condensation_credit(&self) -> LandSurfaceEnergyShadowError {
        LandSurfaceEnergyShadowError::UnsupportedCustody(
            "DirectRunFrame has no production condensation-credit mutation endpoint",
        )
    }

    fn validate_request(
        &self,
        request: &MixedRealHydrologyRequest,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        let key = &request.request.key;
        key.validate(self.owner.transaction_id())?;
        if key.source_type != WaterSourceType::SoilLayerLiquid {
            return Err(LandSurfaceEnergyShadowError::UnsupportedCustody(
                "surface/litter liquid is not a persistent production hydrology store",
            ));
        }
        if key.soil_layer_id.as_ref() != Some(&request.source.layer_id)
            || !self.owner.layer_facts().contains_key(&request.source)
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "mixed source identity",
            ));
        }
        if !request.request.amount_kg_m2_stand_ground.is_finite()
            || request.request.amount_kg_m2_stand_ground < 0.0
        {
            return Err(LandSurfaceEnergyShadowError::Operand(
                "mixed request amount",
            ));
        }
        Ok(())
    }
}

/// Result of the open bare-soil potential/final transaction.
#[derive(Clone, Debug, PartialEq)]
pub struct OpenBareSoilShadowResult {
    pub potential: openwepp_land_surface_energy::AcceptedOpenSurface,
    pub final_surface: openwepp_land_surface_energy::AcceptedOpenSurface,
    pub arbitration: MixedRealHydrologyArbitration,
    pub hydrology_candidate: MixedRealHydrologyCandidate,
}

/// Execute one owner-uncapped solve, one real authorization, and one fixed-cap
/// rebuild. The root finalizer represents the existing V8 capped root solve.
pub fn execute_open_bare_soil_shadow<F>(
    adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    beginning: &OpenSurfaceProblem,
    ground_key: GroundWaterKey,
    ground_source: RealHydrologySourceKey,
    root_requests: &[MixedRealHydrologyRequest],
    finalize_roots: F,
) -> Result<OpenBareSoilShadowResult, LandSurfaceEnergyShadowError>
where
    F: FnOnce(
        &[MixedRealHydrologyAuthorization],
    ) -> Result<Vec<MixedRealHydrologyUse>, LandSurfaceEnergyShadowError>,
{
    let potential = match solve_open_surface(beginning, None, None)? {
        OpenSurfaceSolveOutcome::Accepted(value) => value,
        OpenSurfaceSolveOutcome::Rejected(_) => {
            return Err(LandSurfaceEnergyShadowError::LandSurface(
                LandSurfaceEnergyError::NumericalAcceptedResidual,
            ));
        }
    };
    if potential.evaluation.water.branch == WaterBranch::Condensation {
        return Err(adapter.reject_condensation_credit());
    }
    let mut requests = root_requests.to_vec();
    requests.push(MixedRealHydrologyRequest {
        request: WaterAmount {
            key: ground_key.clone(),
            amount_kg_m2_stand_ground: potential.evaluation.water.request_kg_m2_stand_ground,
        },
        source: ground_source.clone(),
    });
    let arbitration = adapter.authorize(&requests)?;
    let ground_authorization = arbitration
        .authorizations
        .iter()
        .find(|row| row.authorization.key == ground_key && row.source == ground_source)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "ground authorization missing",
        ))?;
    let cap_denominator = checked_surface_liquid_mul(beginning.tile_fraction, beginning.interval_s)
        .ok_or(LandSurfaceEnergyShadowError::Bound(
            "ground authorization interval-basis arithmetic",
        ))?;
    let cap_rate = checked_surface_liquid_div(
        ground_authorization.authorization.amount_kg_m2_stand_ground,
        cap_denominator,
    )
    .ok_or(LandSurfaceEnergyShadowError::Bound(
        "ground authorization rate arithmetic",
    ))?;
    let final_surface = match solve_open_surface(beginning, Some(cap_rate), None)? {
        OpenSurfaceSolveOutcome::Accepted(value) => value,
        OpenSurfaceSolveOutcome::Rejected(_) => {
            return Err(LandSurfaceEnergyShadowError::LandSurface(
                LandSurfaceEnergyError::NumericalAcceptedResidual,
            ));
        }
    };
    let mut uses = finalize_roots(&arbitration.authorizations)?;
    let ground_finalized_use =
        if final_surface.evaluation.water.branch == WaterBranch::AuthorizationActiveOrTie {
            // The accepted active-cap branch is definitionally the exact owner
            // authorization. Preserve that authoritative amount instead of
            // round-tripping it through tile-rate division and multiplication.
            ground_authorization.authorization.amount_kg_m2_stand_ground
        } else {
            final_surface
                .evaluation
                .water
                .finalized_use_kg_m2_stand_ground
        };
    uses.push(MixedRealHydrologyUse {
        finalized_use: WaterAmount {
            key: ground_key,
            amount_kg_m2_stand_ground: ground_finalized_use,
        },
        source: ground_source,
    });
    let hydrology_candidate = adapter.candidate_from_finalized_uses(&arbitration, &uses)?;
    Ok(OpenBareSoilShadowResult {
        potential,
        final_surface,
        arbitration,
        hydrology_candidate,
    })
}
