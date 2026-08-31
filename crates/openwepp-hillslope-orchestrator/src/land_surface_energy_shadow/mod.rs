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
    BandDirectionalFluxes, BareSoilParameters, BiochemicalConstants, CanopyReferenceGeometry,
    ComponentId, CondensationCredit, CoveredColumnInputs, CoveredOccupancyInputs,
    CoveredPotentialPhase, FinalCoveredTileCandidate, GroundWaterKey,
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyError, LandSurfaceEnergyErrorClass,
    LandSurfaceEnergyState, LandSurfaceForcing, LeafBiochemicalInputs, LiquidParcel,
    LiquidParcelKind, LiquidTemperatureProvider, NumericalConfiguration, OfeConfiguration, OfeId,
    OpenNeutralGeometry, OpenPotentialPhase, OpenSurfaceProblem, OwnerConfigurationRef, OwnerKind,
    OwnerRollbackHash, PotentialWaterRequestBatch, RequestingComponent, RootHydraulicLayer,
    RootRuntimeIdentity, RuntimeTileIdentity, Sha256Digest, SoilInterfaceLayer,
    SoilThermalLayerCandidate, SoilThermalLayerSnapshot, SoilThermalNodeOperands,
    SoilThermalOfeSnapshot, SoilThermalSnapshot, SoilThermalTileCandidate, SourceId,
    StandGroundWaterAmountBasis, SurfaceClass, SurfaceClassKind, SurfaceConfiguration,
    SurfaceHeatStorageMode, SurfaceId, SurfaceStorageBranch, TileConfiguration, TileState,
    TurbulenceConfiguration, UnderCanopyGeometry, WaterAmount, WaterAuthorization,
    WaterAuthorizationReason, WaterProtocol, WaterProtocolRow, WaterProtocolViolation,
    WaterSourceType, WaterUseOperands, validate_water_use,
};
#[cfg(test)]
use openwepp_land_surface_energy::{
    OpenSurfaceSolveOutcome, WaterBranch, evaluate_open_surface, finalize_open_phase,
    solve_open_potential_phase, solve_open_surface,
};
pub(crate) use openwepp_land_surface_energy::{
    finalize_covered_phase, solve_covered_potential_phase,
};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::direct_runtime::{
    DirectLayerWithdrawalRequest, DirectSurfaceLiquidClosureUnit, aggregate_direct_soil_water,
    apply_direct_finalized_layer_liquid_debit, apply_direct_same_pass_infiltration,
    authorize_direct_layer_withdrawals, checked_surface_liquid_add, checked_surface_liquid_close,
    checked_surface_liquid_div, checked_surface_liquid_mul, checked_surface_liquid_sub,
    checked_surface_liquid_sum,
};
use crate::direct_runtime::{
    DirectWb14ParentWorkingState,
    execute_surface_liquid_ingress_with_parent_state_and_coupled_binding,
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
    DirectSurfaceLiquidStoreKey, apply_surface_liquid_resource_phase,
    authorize_surface_liquid_withdrawals,
};

mod covered_derived_ingress;
mod covered_forest;
mod covered_v8_owner;
mod finalization_sealing;
mod multi_tile_runtime;
#[cfg(test)]
#[path = "../../../../tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs"]
mod raw_boundary_contract_tests;
mod receiver_failure;
mod receiver_preflight;
mod receiver_validation;
pub(crate) mod strict_v8_endpoint;
#[cfg(feature = "restart-authority-evidence")]
pub use strict_v8_endpoint::endpoint_rollback_tests::{
    EndpointFixture, endpoint_fixture, two_ofe_routed_endpoint_fixture,
};
mod unified_entry_preflight;
// The successor coordinator is an admitted candidate-only seam. The active
// V9 consumer adopts it in the following manifest slice; keeping it dormant
// here must not weaken lints on the established V8 runtime.
#[allow(dead_code)]
pub(crate) mod v3_execution;
pub(crate) mod v3_input_projection;
pub(crate) mod v3_rollback;
#[cfg(test)]
mod v3_tests;
mod v8_input_projection;
mod v8_projection;
mod v8_rollback;
pub(super) use covered_derived_ingress::CoveredIngressSchedule;
pub(crate) use covered_forest::CoveredForestShadowResult;
pub(crate) use covered_v8_owner::{
    CoveredCarrierComponentState, CoveredLseIterationState, FixedCapCanopyReleasesByDestination,
    ProvisionalCoveredV8PhysicalEvaluationV1,
};
pub use covered_v8_owner::{CoveredV8OwnerEnvelopeError, UncommittedCoveredV8OwnerEnvelope};
use finalization_sealing::first_sealed_finalization_violation;
use receiver_failure::canonical_receiver_failure;
pub use receiver_validation::validate_soil_thermal_v2_surface_cancellation;
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
#[cfg(all(test, not(feature = "restart-authority-evidence")))]
pub(crate) use strict_v8_endpoint::endpoint_rollback_tests::{EndpointFixture, endpoint_fixture};
pub(crate) use strict_v8_endpoint::execute_v8_lse_runtime_shadow_internal;
pub(crate) use strict_v8_endpoint::execute_v8_lse_runtime_shadow_v11;
pub(crate) use strict_v8_endpoint::execute_v8_lse_runtime_shadow_v11_physical_with_carriers;
pub(crate) use strict_v8_endpoint::execute_v8_lse_runtime_shadow_v11_with_carriers;
pub use strict_v8_endpoint::{ExecuteV8LseRuntimeShadowError, execute_v8_lse_runtime_shadow};
pub use v8_input_projection::{V8CanopyForcingReceipt, V8InputProjectionError};
pub(crate) use v8_input_projection::{
    V10RootZoneLayerReceipt, V10RootZoneReceiptKey, V10RootZoneReceiptSet,
};
pub use v8_projection::V8ProjectionError;
pub use v8_rollback::{
    V8RollbackError, V8RollbackInputs, V8RollbackOwnerBytes, V8RollbackOwnerKind,
    V8RollbackSnapshot,
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
    pre_ingress_soil_thermal_candidates: Vec<SoilThermalTileCandidate>,
    pre_ingress_soil_thermal_sha256: String,
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
    pub fn pre_ingress_soil_thermal_candidates(&self) -> &[SoilThermalTileCandidate] {
        &self.pre_ingress_soil_thermal_candidates
    }

    #[must_use]
    pub fn pre_ingress_soil_thermal_sha256(&self) -> &str {
        &self.pre_ingress_soil_thermal_sha256
    }

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
            || self.pre_ingress_soil_thermal_candidates.len() != self.soil_thermal_candidates.len()
            || self.pre_ingress_soil_thermal_sha256
                != finalization_receiver_sets_sha256(
                    &[],
                    &self.pre_ingress_soil_thermal_candidates,
                    &[],
                )
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
            let pre_ingress = self
                .pre_ingress_soil_thermal_candidates
                .iter()
                .find(|candidate| {
                    candidate.ofe_id == tile.ofe_id && candidate.tile_id == tile.tile_id
                })
                .ok_or_else(|| {
                    self.atomic_failure(
                        OwnerKind::SoilThermal,
                        &self.receiver_closure_operands.soil_thermal_owner_id,
                        "pre-ingress soil-thermal candidate custody",
                    )
                })?;
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
            if pre_ingress.owner_id != tile.owner_id
                || pre_ingress.beginning_state_sha256 != tile.beginning_state_sha256
                || pre_ingress.layers.len() != tile.layers.len()
                || pre_ingress
                    .layers
                    .iter()
                    .zip(&tile.layers)
                    .any(|(before, after)| {
                        before.layer_id != after.layer_id
                            || before.beginning_enthalpy_j_m2_ofe_ground.to_bits()
                                != after.beginning_enthalpy_j_m2_ofe_ground.to_bits()
                            || before.ground_heat_credit_j_m2_ofe_ground.to_bits()
                                != after.ground_heat_credit_j_m2_ofe_ground.to_bits()
                            || before
                                .infiltration_enthalpy_credit_j_m2_ofe_ground
                                .to_bits()
                                != 0.0_f64.to_bits()
                    })
            {
                return Err(self.atomic_failure(
                    OwnerKind::SoilThermal,
                    &self.receiver_closure_operands.soil_thermal_owner_id,
                    "pre-ingress/post-ingress soil-thermal custody",
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

include!("real_hydrology_execution.rs");

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
#[cfg(test)]
pub(crate) fn execute_open_bare_soil_shadow<F>(
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
