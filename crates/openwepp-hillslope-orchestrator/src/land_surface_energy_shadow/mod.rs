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
    OfeId, OpenNeutralGeometry, OpenPotentialPhase, OpenSurfaceProblem, OwnerKind,
    OwnerRollbackHash, PotentialWaterRequestBatch, RequestingComponent, RuntimeTileIdentity,
    Sha256Digest, SoilThermalLayerCandidate, SoilThermalLayerSnapshot, SoilThermalNodeOperands,
    SoilThermalOfeSnapshot, SoilThermalSnapshot, SoilThermalTileCandidate, SourceId,
    StandGroundWaterAmountBasis, SurfaceClass, SurfaceClassKind, SurfaceId, SurfaceStorageBranch,
    TileState, WaterAmount, WaterAuthorization, WaterAuthorizationReason, WaterProtocol,
    WaterSourceType, finalize_open_phase, solve_open_potential_phase,
};
use openwepp_land_surface_energy::{
    LandSurfaceEnergyError, OpenSurfaceSolveOutcome, WaterBranch, solve_open_surface,
};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::direct_runtime::{
    DirectLayerWithdrawalRequest, aggregate_direct_soil_water,
    apply_direct_finalized_layer_liquid_debit, apply_direct_same_pass_infiltration,
    authorize_direct_layer_withdrawals,
};
use crate::vegetation_real_hydrology_shadow::{
    RealHydrologyShadowAdapter, RealHydrologyShadowError, RealHydrologySourceKey,
};
use crate::{
    DirectRunFrame, DirectSurfaceLiquidArbitration, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidError, DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidIngressCandidate,
    DirectSurfaceLiquidIngressInput, DirectSurfaceLiquidOfeBinding,
    DirectSurfaceLiquidParcelReceipt, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidReceiptDisposition, DirectSurfaceLiquidReceiptRecipient,
    DirectSurfaceLiquidResourceCandidate, apply_surface_liquid_resource_phase,
    authorize_surface_liquid_withdrawals, execute_surface_liquid_ingress,
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
        match value {
            RealHydrologyShadowError::Identity(detail) => Self::Identity(detail),
            RealHydrologyShadowError::Operand(detail) => Self::Operand(detail),
            RealHydrologyShadowError::Bound(detail) => Self::Bound(detail),
            RealHydrologyShadowError::Protocol(_) => Self::Bound("resource protocol"),
        }
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
    pub beginning_hydrology_snapshot_sha256: Sha256Digest,
    pub production_soil: Vec<ProductionSoilReceiverOperands>,
    pub soil_thermal: Vec<SoilThermalReceiverOperands>,
    pub lse_tiles: Vec<LseTileReceiverOperands>,
}

#[derive(Clone, Debug)]
struct ReceiverEnvelopeViolation {
    owner_id: Option<ResourceOwnerId>,
    ofe_id: Option<OfeId>,
    tile_id: Option<TileId>,
    detail: &'static str,
}

impl ReceiverEnvelopeViolation {
    fn for_tile(
        owner_id: Option<ResourceOwnerId>,
        ofe_id: OfeId,
        tile_id: TileId,
        detail: &'static str,
    ) -> Self {
        Self {
            owner_id,
            ofe_id: Some(ofe_id),
            tile_id: Some(tile_id),
            detail,
        }
    }

    fn for_owner(owner_id: Option<ResourceOwnerId>, detail: &'static str) -> Self {
        Self {
            owner_id,
            ofe_id: None,
            tile_id: None,
            detail,
        }
    }
}

impl UnifiedReceiverExpectations {
    pub fn try_new(
        lse_owner_id: ResourceOwnerId,
        beginning_lse_state_sha256: Sha256Digest,
        beginning_hydrology_snapshot_sha256: Sha256Digest,
        soil_thermal_owner_id: ResourceOwnerId,
        beginning_soil_thermal_state_sha256: Sha256Digest,
        ordered_thermal_layers: Vec<(OfeId, TileId, Vec<SoilLayerId>)>,
    ) -> Result<Self, LandSurfaceEnergyShadowError> {
        let mut identities = BTreeSet::new();
        let mut layers_by_tile = Vec::new();
        for (ofe_id, tile_id, layers) in ordered_thermal_layers {
            if layers.is_empty()
                || layers.iter().collect::<BTreeSet<_>>().len() != layers.len()
                || !identities.insert((ofe_id.clone(), tile_id.clone()))
            {
                return Err(LandSurfaceEnergyShadowError::Identity(
                    "invalid independent receiver expectations",
                ));
            }
            layers_by_tile.push(((ofe_id, tile_id), layers));
        }
        if layers_by_tile.is_empty() {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "empty independent receiver expectations",
            ));
        }
        Ok(Self {
            lse_owner_id,
            beginning_lse_state_sha256,
            beginning_hydrology_snapshot_sha256,
            soil_thermal_owner_id,
            beginning_soil_thermal_state_sha256,
            ordered_thermal_layers: layers_by_tile,
        })
    }
}

impl UnifiedLseFinalization {
    pub fn try_new(
        water_protocol: WaterProtocol,
        ending_tile_states_pre_ingress: Vec<TileState>,
        soil_thermal_candidates: Vec<SoilThermalTileCandidate>,
        rollback_hashes: Vec<OwnerRollbackHash>,
    ) -> Result<Self, LandSurfaceEnergyShadowError> {
        water_protocol.validate()?;
        if let Some(violation) = first_sealed_finalization_violation(
            &water_protocol,
            &ending_tile_states_pre_ingress,
            &soil_thermal_candidates,
            &rollback_hashes,
        ) {
            return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(water_protocol.transaction_id),
                    owner_id: violation.owner_id,
                    ofe_id: violation.ofe_id,
                    tile_id: violation.tile_id,
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                Some(water_protocol.beginning_snapshot_sha256.to_string()),
                Some(finalization_receiver_sets_sha256(
                    &ending_tile_states_pre_ingress,
                    &soil_thermal_candidates,
                    &rollback_hashes,
                )),
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
}

fn first_sealed_finalization_violation(
    protocol: &WaterProtocol,
    lse_tiles: &[TileState],
    thermal_tiles: &[SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
) -> Option<ReceiverEnvelopeViolation> {
    let lse_owner = protocol
        .requests
        .iter()
        .find(|request| request.key.requesting_component == RequestingComponent::GroundSurface)
        .map(|request| request.key.requesting_owner_id.clone());
    if lse_tiles.is_empty() {
        return Some(ReceiverEnvelopeViolation::for_owner(
            lse_owner,
            "missing sealed LSE tile receiver",
        ));
    }
    if let Some(tile) = first_duplicate_lse_tile(lse_tiles) {
        return Some(ReceiverEnvelopeViolation::for_tile(
            lse_owner,
            tile.ofe_id.clone(),
            tile.tile_id.clone(),
            "duplicate sealed LSE tile receiver",
        ));
    }
    if let Some(tile) = first_duplicate_thermal_tile(thermal_tiles) {
        return Some(ReceiverEnvelopeViolation::for_tile(
            Some(tile.owner_id.clone()),
            tile.ofe_id.clone(),
            tile.tile_id.clone(),
            "duplicate sealed soil-thermal tile receiver",
        ));
    }
    for index in 0..lse_tiles.len().max(thermal_tiles.len()) {
        let lse = lse_tiles.get(index);
        let thermal = thermal_tiles.get(index);
        if lse.map(|tile| (&tile.ofe_id, &tile.tile_id))
            != thermal.map(|tile| (&tile.ofe_id, &tile.tile_id))
        {
            return Some(match thermal {
                Some(tile) => ReceiverEnvelopeViolation::for_tile(
                    Some(tile.owner_id.clone()),
                    tile.ofe_id.clone(),
                    tile.tile_id.clone(),
                    "sealed LSE/soil-thermal tile receiver mismatch",
                ),
                None => ReceiverEnvelopeViolation::for_tile(
                    lse_owner,
                    lse?.ofe_id.clone(),
                    lse?.tile_id.clone(),
                    "missing sealed soil-thermal tile receiver",
                ),
            });
        }
    }
    if let Some(candidate) = thermal_tiles.iter().find(|candidate| {
        candidate.layers.is_empty()
            || candidate
                .layers
                .iter()
                .map(|layer| &layer.layer_id)
                .collect::<BTreeSet<_>>()
                .len()
                != candidate.layers.len()
    }) {
        return Some(ReceiverEnvelopeViolation::for_tile(
            Some(candidate.owner_id.clone()),
            candidate.ofe_id.clone(),
            candidate.tile_id.clone(),
            "invalid sealed soil-thermal layer receiver set",
        ));
    }
    first_sealed_rollback_violation(rollback_hashes)
}

fn first_duplicate_lse_tile(tiles: &[TileState]) -> Option<&TileState> {
    let mut seen = BTreeSet::new();
    tiles
        .iter()
        .find(|tile| !seen.insert((tile.ofe_id.clone(), tile.tile_id.clone())))
}

fn first_duplicate_thermal_tile(
    tiles: &[SoilThermalTileCandidate],
) -> Option<&SoilThermalTileCandidate> {
    let mut seen = BTreeSet::new();
    tiles
        .iter()
        .find(|tile| !seen.insert((tile.ofe_id.clone(), tile.tile_id.clone())))
}

fn first_sealed_rollback_violation(
    rows: &[OwnerRollbackHash],
) -> Option<ReceiverEnvelopeViolation> {
    let mut seen = BTreeSet::new();
    rows.iter()
        .find(|row| {
            row.before_sha256 != row.after_sha256
                || !seen.insert((row.owner_kind, row.owner_id.clone()))
        })
        .map(|row| {
            let detail = match row.owner_kind {
                OwnerKind::LandSurfaceEnergy => "invalid sealed LSE rollback row",
                OwnerKind::SoilThermal => "invalid sealed soil-thermal rollback row",
                OwnerKind::Hydrology => "invalid sealed hydrology rollback row",
                OwnerKind::Vegetation => "invalid sealed vegetation rollback row",
                OwnerKind::Biogeochemistry => "invalid sealed biogeochemistry rollback row",
                OwnerKind::Envelope => "invalid sealed envelope rollback row",
            };
            ReceiverEnvelopeViolation::for_owner(
                ResourceOwnerId::try_new(row.owner_id.clone()).ok(),
                detail,
            )
        })
}

fn finalization_receiver_sets_sha256(
    lse_tiles: &[TileState],
    soil_thermal: &[SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
) -> String {
    let mut digest = Sha256::new();
    digest.update(b"openwepp-sealed-lse-finalization-receivers-v1");
    for tile in lse_tiles {
        digest.update(tile.ofe_id.as_str().as_bytes());
        digest.update(tile.tile_id.as_str().as_bytes());
        digest.update(
            tile.surface_enthalpy_j_m2_tile_ground
                .to_bits()
                .to_be_bytes(),
        );
        digest.update(
            tile.surface_temperature_warm_start_k
                .to_bits()
                .to_be_bytes(),
        );
    }
    for tile in soil_thermal {
        digest.update(tile.owner_id.as_str().as_bytes());
        digest.update(tile.beginning_state_sha256.as_str().as_bytes());
        digest.update(tile.ofe_id.as_str().as_bytes());
        digest.update(tile.tile_id.as_str().as_bytes());
        for layer in &tile.layers {
            digest.update(layer.layer_id.as_str().as_bytes());
            digest.update(
                layer
                    .beginning_enthalpy_j_m2_ofe_ground
                    .to_bits()
                    .to_be_bytes(),
            );
            digest.update(
                layer
                    .ending_enthalpy_j_m2_ofe_ground
                    .to_bits()
                    .to_be_bytes(),
            );
        }
    }
    for row in rollback_hashes {
        digest.update([row.owner_kind as u8]);
        digest.update(row.owner_id.as_bytes());
        digest.update(row.before_sha256.as_str().as_bytes());
        digest.update(row.after_sha256.as_str().as_bytes());
    }
    format!("{:x}", digest.finalize())
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
            .ok_or_else(|| self.atomic_failure("missing ending surface-liquid owner"))?;
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
            return Err(self.atomic_failure("unified candidate owner lineage or ending state"));
        }
        for (tile, operands) in self
            .ending_lse_tile_states
            .iter()
            .zip(&self.receiver_closure_operands.lse_tiles)
        {
            if tile.ofe_id != operands.ofe_id
                || tile.tile_id != operands.tile_id
                || !enthalpy_close(
                    tile.surface_enthalpy_j_m2_tile_ground,
                    operands.ending_enthalpy_j_m2_tile_ground,
                )
            {
                return Err(self.atomic_failure("LSE candidate/receiver closure join"));
            }
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
                return Err(self.atomic_failure("soil-thermal candidate receiver layer"));
            };
            if tile.ofe_id != operands.ofe_id
                || tile.tile_id != operands.tile_id
                || !enthalpy_close(
                    layer.ending_enthalpy_j_m2_ofe_ground,
                    operands.ending_enthalpy_j_m2_ofe_ground,
                )
            {
                return Err(self.atomic_failure("soil-thermal candidate/receiver closure join"));
            }
        }
        Ok(())
    }

    fn atomic_failure(&self, detail: &'static str) -> DirectSurfaceLiquidError {
        receiver_atomic_failure(&self.receiver_closure_operands, None, None, detail)
    }
}

/// Digest the complete immutable soil-layer and surface-liquid owner snapshot.
pub fn unified_beginning_hydrology_snapshot_sha256(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<Sha256Digest, LandSurfaceEnergyShadowError> {
    surface_configuration.validate()?;
    validate_surface_production_binding(soil_adapter.owner, surface_configuration)?;
    if &surface_configuration.owner_id != soil_adapter.owner.hydrology_owner_id() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "mixed unified hydrology owner",
        ));
    }
    let surface_state = soil_adapter
        .owner
        .beginning_frame()
        .surface_liquid_shadow
        .as_deref()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing beginning surface-liquid owner",
        ))?;
    surface_state.validate(surface_configuration)?;
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

fn validate_surface_production_binding(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let frame = owner.beginning_frame();
    if configuration.run_id != frame.identity.run_id
        || configuration.ofe_bindings.len() != frame.lanes.len()
        || owner.layer_maps().len() != frame.lanes.len()
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "surface production run or lane count",
        ));
    }
    for ((binding, mapping), lane) in configuration
        .ofe_bindings
        .iter()
        .zip(owner.layer_maps())
        .zip(&frame.lanes)
    {
        if binding.production_lane_index != mapping.ofe_lane.lane_index
            || binding.production_lane_id != mapping.ofe_lane.lane_id
            || binding.production_lane_id != lane.lane_id
            || binding.ordered_soil_layer_ids != mapping.layer_ids
            || binding.ordered_soil_layer_ids.len() != lane.subsurface_layers.len()
            || configuration.records.iter().any(|record| {
                record.key.ofe_id == binding.ofe_id
                    && record.ofe_area_m2.to_bits() != lane.area_m2.to_bits()
            })
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "surface production OFE/lane/area/layer binding",
            ));
        }
    }
    Ok(())
}

/// Join one immutable LSE request batch to both actual water owners.
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
    request_batch.validate()?;
    let actual_snapshot =
        unified_beginning_hydrology_snapshot_sha256(soil_adapter, surface_configuration)?;
    if request_batch.transaction_id != soil_adapter.owner.transaction_id()
        || ingress.transaction_id != request_batch.transaction_id
        || ingress.day_index != soil_adapter.owner.day_index()
        || ingress.interval_s.to_bits() != soil_adapter.owner.interval_s().to_bits()
        || &actual_snapshot != expected_beginning_hydrology_snapshot_sha256
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "unified transaction or beginning snapshot identity",
        ));
    }
    validate_native_shadow_domain(
        soil_adapter.owner,
        surface_configuration,
        expected_beginning_hydrology_snapshot_sha256,
    )?;
    validate_receiver_expectations(
        soil_adapter.owner,
        surface_configuration,
        receiver_expectations,
        request_batch,
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
    let (soil_requests, surface_requests) = partition_requests(request_batch, soil_sources)?;
    let soil = soil_adapter.authorize(&soil_requests)?;
    let surface = authorize_surface_liquid_withdrawals(
        surface_configuration,
        beginning_surface,
        request_batch.transaction_id,
        beginning_surface
            .records
            .first()
            .and_then(|record| record.last_accepted_transaction_id),
        &surface_requests,
    )?;
    let authorizations = restore_authorization_order(request_batch, &soil, &surface)?;
    let arbitration = UnifiedRealHydrologyArbitration {
        transaction_id: request_batch.transaction_id,
        requests: request_batch.requests.clone(),
        authorizations,
        soil,
        surface,
    };
    let finalized = finalize_fixed_caps(&arbitration.authorizations)?;
    validate_final_protocol(
        &finalized.water_protocol,
        &arbitration,
        expected_beginning_hydrology_snapshot_sha256,
        &surface_configuration.owner_id,
    )?;
    construct_unified_candidate(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        arbitration,
        finalized,
        ingress,
    )
}

fn validate_native_shadow_domain(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(lane_index) = owner
        .beginning_frame()
        .lanes
        .iter()
        .position(lane_has_unsupported_frozen_or_snow_state)
    {
        return Err(DirectSurfaceLiquidError::unsupported_domain_failure(
            DirectSurfaceLiquidPhase::AtomicEnvelope,
            first_lane_error_context(owner, configuration, lane_index),
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            "snow, terminal snow, frozen, or thawing production frame",
        )
        .into());
    }
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
            None,
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

fn validate_receiver_expectations(
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
        let first = configuration.records.first();
        return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(owner.transaction_id()),
                owner_id: Some(expectations.lse_owner_id.clone()),
                ofe_id: first.map(|record| record.key.ofe_id.clone()),
                tile_id: first.map(|record| record.key.tile_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(receiver_expectations_sha256(expectations)),
            "independent LSE receiver expectations",
        )
        .into());
    }
    if let Some(violation) = first_expected_identity_violation(
        &expected_tiles,
        &thermal_tiles,
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
    Ok(())
}

fn receiver_expectations_sha256(expectations: &UnifiedReceiverExpectations) -> String {
    let mut digest = Sha256::new();
    digest.update(b"openwepp-unified-receiver-expectations-v1");
    digest.update(expectations.lse_owner_id.as_str().as_bytes());
    digest.update(expectations.beginning_lse_state_sha256.as_str().as_bytes());
    digest.update(
        expectations
            .beginning_hydrology_snapshot_sha256
            .as_str()
            .as_bytes(),
    );
    digest.update(expectations.soil_thermal_owner_id.as_str().as_bytes());
    digest.update(
        expectations
            .beginning_soil_thermal_state_sha256
            .as_str()
            .as_bytes(),
    );
    for ((ofe_id, tile_id), layers) in &expectations.ordered_thermal_layers {
        digest.update(ofe_id.as_str().as_bytes());
        digest.update(tile_id.as_str().as_bytes());
        for layer in layers {
            digest.update(layer.as_str().as_bytes());
        }
    }
    format!("{:x}", digest.finalize())
}

fn validate_final_protocol(
    protocol: &WaterProtocol,
    arbitration: &UnifiedRealHydrologyArbitration,
    expected_snapshot: &Sha256Digest,
    expected_owner: &openwepp_kernel_contract::ResourceOwnerId,
) -> Result<(), LandSurfaceEnergyShadowError> {
    protocol.validate()?;
    if protocol.transaction_id != arbitration.transaction_id
        || &protocol.hydrology_owner_id != expected_owner
        || &protocol.beginning_snapshot_sha256 != expected_snapshot
        || protocol.requests != arbitration.requests
        || protocol.authorizations != arbitration.authorizations
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "final water protocol lineage or D/A identity",
        ));
    }
    Ok(())
}

fn partition_requests(
    batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
) -> Result<(Vec<MixedRealHydrologyRequest>, Vec<WaterAmount>), LandSurfaceEnergyShadowError> {
    let mut soil = Vec::new();
    let mut surface = Vec::new();
    let mut consumed_soil_keys = BTreeSet::new();
    for request in &batch.requests {
        match request.key.source_type {
            WaterSourceType::SoilLayerLiquid => {
                let source = soil_sources.get(&request.key).ok_or(
                    LandSurfaceEnergyShadowError::Identity("missing soil source mapping"),
                )?;
                consumed_soil_keys.insert(request.key.clone());
                soil.push(MixedRealHydrologyRequest {
                    request: request.clone(),
                    source: source.clone(),
                });
            }
            WaterSourceType::SurfaceLiquid | WaterSourceType::LitterLiquid => {
                if soil_sources.contains_key(&request.key) {
                    return Err(LandSurfaceEnergyShadowError::Identity(
                        "surface request has soil mapping",
                    ));
                }
                surface.push(request.clone());
            }
        }
    }
    if consumed_soil_keys.len() != soil_sources.len() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "unused soil source mapping",
        ));
    }
    Ok((soil, surface))
}

fn restore_authorization_order(
    batch: &PotentialWaterRequestBatch,
    soil: &MixedRealHydrologyArbitration,
    surface: &DirectSurfaceLiquidArbitration,
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
        return Err(LandSurfaceEnergyShadowError::Identity(
            "incomplete unified authorization",
        ));
    }
    batch
        .requests
        .iter()
        .map(|request| {
            by_key
                .get(&request.key)
                .cloned()
                .ok_or(LandSurfaceEnergyShadowError::Identity(
                    "authorization order identity",
                ))
        })
        .collect()
}

fn construct_unified_candidate(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
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

#[allow(clippy::too_many_arguments)]
fn apply_ingress_to_real_receivers(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
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
            violation,
            beginning_hydrology_snapshot_sha256,
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
            violation,
            beginning_hydrology_snapshot_sha256,
            &receiver_attempt_sha256,
        )
    })?;
    let beginning_frame = ending_frame.clone();
    let beginning_lse_tiles = lse_tiles.to_vec();
    let beginning_soil_thermal = soil_thermal.to_vec();
    let mut infiltration_m_by_lane = BTreeMap::<usize, f64>::new();
    for receipt in ingress.receipts() {
        if let Some((lane_index, infiltration_m)) =
            apply_receiver_receipt(configuration, receipt, lse_tiles, soil_thermal)?
        {
            *infiltration_m_by_lane.entry(lane_index).or_default() += infiltration_m;
        }
    }
    apply_production_infiltration(owner, ending_frame, infiltration_m_by_lane)?;
    let operands = freeze_real_receiver_closure_operands(
        owner,
        configuration,
        ingress,
        &beginning_frame,
        ending_frame,
        &beginning_lse_tiles,
        lse_tiles,
        &beginning_soil_thermal,
        soil_thermal,
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
        let violation = if let Some(candidate) = soil_thermal.get(index) {
            ReceiverEnvelopeViolation::for_tile(
                Some(candidate.owner_id.clone()),
                candidate.ofe_id.clone(),
                candidate.tile_id.clone(),
                "soil-thermal tile receiver topology mismatch",
            )
        } else {
            let (ofe_id, tile_id) = &expected_tiles[index];
            ReceiverEnvelopeViolation::for_tile(
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
    owner_id: &ResourceOwnerId,
    detail: &'static str,
) -> Option<ReceiverEnvelopeViolation> {
    let index = first_identity_mismatch(expected, actual)?;
    let (ofe_id, tile_id) = actual.get(index).or_else(|| expected.get(index))?;
    Some(ReceiverEnvelopeViolation::for_tile(
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
    violation: ReceiverEnvelopeViolation,
    beginning_sha256: &Sha256Digest,
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
    DirectSurfaceLiquidError::atomic_envelope_failure(
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: violation.owner_id,
            ofe_id: violation.ofe_id,
            tile_id: violation.tile_id,
            surface_id: record.map(|record| record.key.surface_id.clone()),
            source_id: record.map(|record| record.key.source_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        Some(beginning_sha256.to_string()),
        Some(attempted_sha256.to_owned()),
        violation.detail,
    )
    .into()
}

fn apply_receiver_receipt(
    configuration: &DirectSurfaceLiquidConfiguration,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    lse_tiles: &mut [TileState],
    soil_thermal: &mut [SoilThermalTileCandidate],
) -> Result<Option<(usize, f64)>, LandSurfaceEnergyShadowError> {
    let binding = configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.ofe_id == receipt.recipient_store_key.ofe_id)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "ingress receipt OFE binding",
        ))?;
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
        )
        .map(Some),
        (
            DirectSurfaceLiquidReceiptDisposition::RetainedSurface,
            DirectSurfaceLiquidReceiptRecipient::SurfaceStore { store_key },
        ) => credit_retained_receipt(configuration, receipt, store_key, lse_tiles).map(|()| None),
        (
            DirectSurfaceLiquidReceiptDisposition::RoutedRunoff,
            DirectSurfaceLiquidReceiptRecipient::RoutedOfe { .. },
        )
        | (
            DirectSurfaceLiquidReceiptDisposition::OutletRunoff,
            DirectSurfaceLiquidReceiptRecipient::Outlet { .. },
        ) => Ok(None),
        _ => Err(LandSurfaceEnergyShadowError::Identity(
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
) -> Result<(usize, f64), LandSurfaceEnergyShadowError> {
    if ofe_id != &binding.ofe_id
        || production_lane_index != binding.production_lane_index
        || production_lane_id != binding.production_lane_id
        || ordered_soil_layer_ids != binding.ordered_soil_layer_ids
        || soil_thermal_layer_id != &binding.infiltration_soil_thermal_layer_id
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "infiltration receipt production receiver binding",
        ));
    }
    let thermal = soil_thermal
        .iter_mut()
        .find(|candidate| {
            candidate.ofe_id == receipt.recipient_store_key.ofe_id
                && candidate.tile_id == receipt.recipient_store_key.tile_id
        })
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing infiltration soil-thermal tile receiver",
        ))?;
    let layer = thermal
        .layers
        .iter_mut()
        .find(|layer| layer.layer_id == binding.infiltration_soil_thermal_layer_id)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing infiltration soil-thermal layer receiver",
        ))?;
    layer.infiltration_enthalpy_credit_j_m2_ofe_ground += receipt.enthalpy_j_m2_basis_ofe_ground;
    layer.ending_enthalpy_j_m2_ofe_ground += receipt.enthalpy_j_m2_basis_ofe_ground;
    Ok((
        binding.production_lane_index,
        receipt.mass_kg_m2_basis_ofe_ground / WATER_DENSITY_KG_M3,
    ))
}

fn credit_retained_receipt(
    configuration: &DirectSurfaceLiquidConfiguration,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    store_key: &crate::DirectSurfaceLiquidStoreKey,
    lse_tiles: &mut [TileState],
) -> Result<(), LandSurfaceEnergyShadowError> {
    if store_key != &receipt.recipient_store_key {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "retained receipt typed receiver",
        ));
    }
    let tile = lse_tiles
        .iter_mut()
        .find(|tile| {
            tile.ofe_id == receipt.recipient_store_key.ofe_id
                && tile.tile_id == receipt.recipient_store_key.tile_id
        })
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing retained LSE tile receiver",
        ))?;
    let record = configuration
        .records
        .iter()
        .find(|record| record.key == receipt.recipient_store_key)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "retained receipt store receiver",
        ))?;
    tile.surface_enthalpy_j_m2_tile_ground +=
        receipt.enthalpy_j_m2_basis_ofe_ground / record.tile_fraction;
    Ok(())
}

fn apply_production_infiltration(
    owner: &RealHydrologyShadowAdapter,
    ending_frame: &mut DirectRunFrame,
    infiltration_m_by_lane: BTreeMap<usize, f64>,
) -> Result<(), LandSurfaceEnergyShadowError> {
    for (lane_index, infiltration_m) in infiltration_m_by_lane {
        let lane = ending_frame.lanes.get_mut(lane_index).ok_or(
            LandSurfaceEnergyShadowError::Identity("infiltration production lane receiver"),
        )?;
        let day = owner.beginning_day_frames().get(lane_index).ok_or(
            LandSurfaceEnergyShadowError::Identity("infiltration production day receiver"),
        )?;
        apply_direct_same_pass_infiltration(
            &mut lane.subsurface_layers,
            infiltration_m,
            day.percolation_inputs.tillage_depth_m,
        )
        .map_err(|_| LandSurfaceEnergyShadowError::Bound("production infiltration receiver"))?;
        lane.water.soil_water_m = aggregate_direct_soil_water(
            &lane.subsurface_layers,
            "land_surface_energy_shadow.ingress_soil_water",
        )
        .map_err(|_| LandSurfaceEnergyShadowError::Bound("ingress soil reconstruction"))?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn freeze_real_receiver_closure_operands(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    ingress: &DirectSurfaceLiquidIngressCandidate,
    beginning_frame: &DirectRunFrame,
    ending_frame: &DirectRunFrame,
    beginning_lse_tiles: &[TileState],
    ending_lse_tiles: &[TileState],
    beginning_soil_thermal: &[SoilThermalTileCandidate],
    ending_soil_thermal: &[SoilThermalTileCandidate],
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
) -> Result<RealReceiverClosureOperands, LandSurfaceEnergyShadowError> {
    let (infiltration_m_by_ofe, infiltration_enthalpy_by_tile, retained_enthalpy_by_tile) =
        freeze_ingress_receiver_amounts(ingress);
    let production_soil = freeze_production_soil_receivers(
        owner,
        configuration,
        beginning_frame,
        ending_frame,
        &infiltration_m_by_ofe,
    )?;
    let (soil_thermal, lse_tiles) = freeze_energy_receivers(
        configuration,
        beginning_lse_tiles,
        ending_lse_tiles,
        beginning_soil_thermal,
        ending_soil_thermal,
        &infiltration_enthalpy_by_tile,
        &retained_enthalpy_by_tile,
    )?;
    Ok(RealReceiverClosureOperands {
        transaction_id: ingress.transaction_id(),
        hydrology_owner_id: owner.hydrology_owner_id().clone(),
        beginning_hydrology_snapshot_sha256: beginning_hydrology_snapshot_sha256.clone(),
        production_soil,
        soil_thermal,
        lse_tiles,
    })
}

type OfeAmountMap = BTreeMap<OfeId, f64>;
type TileAmountMap = BTreeMap<(OfeId, TileId), f64>;

fn freeze_ingress_receiver_amounts(
    ingress: &DirectSurfaceLiquidIngressCandidate,
) -> (OfeAmountMap, TileAmountMap, TileAmountMap) {
    let mut infiltration_m_by_ofe = BTreeMap::<OfeId, f64>::new();
    let mut infiltration_enthalpy_by_tile = BTreeMap::<(OfeId, TileId), f64>::new();
    let mut retained_enthalpy_by_tile = BTreeMap::<(OfeId, TileId), f64>::new();
    for receipt in ingress.receipts() {
        match receipt.disposition {
            DirectSurfaceLiquidReceiptDisposition::Infiltration => {
                *infiltration_m_by_ofe
                    .entry(receipt.recipient_store_key.ofe_id.clone())
                    .or_default() += receipt.mass_kg_m2_basis_ofe_ground / WATER_DENSITY_KG_M3;
                *infiltration_enthalpy_by_tile
                    .entry((
                        receipt.recipient_store_key.ofe_id.clone(),
                        receipt.recipient_store_key.tile_id.clone(),
                    ))
                    .or_default() += receipt.enthalpy_j_m2_basis_ofe_ground;
            }
            DirectSurfaceLiquidReceiptDisposition::RetainedSurface => {
                *retained_enthalpy_by_tile
                    .entry((
                        receipt.recipient_store_key.ofe_id.clone(),
                        receipt.recipient_store_key.tile_id.clone(),
                    ))
                    .or_default() += receipt.enthalpy_j_m2_basis_ofe_ground;
            }
            DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
            | DirectSurfaceLiquidReceiptDisposition::OutletRunoff => {}
        }
    }
    (
        infiltration_m_by_ofe,
        infiltration_enthalpy_by_tile,
        retained_enthalpy_by_tile,
    )
}

fn freeze_production_soil_receivers(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_frame: &DirectRunFrame,
    ending_frame: &DirectRunFrame,
    infiltration_m_by_ofe: &OfeAmountMap,
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
    infiltration_enthalpy_by_tile: &TileAmountMap,
    retained_enthalpy_by_tile: &TileAmountMap,
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

/// Independently reconstruct all real receiver ending equations from frozen operands.
pub fn validate_real_receiver_closure(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    validate_production_soil_receiver_closure(operands)?;
    for thermal in &operands.soil_thermal {
        let expected_credit = thermal.beginning_infiltration_credit_j_m2_ofe_ground
            + thermal.infiltration_enthalpy_j_m2_ofe_ground;
        let expected_ending = thermal.beginning_enthalpy_j_m2_ofe_ground
            + thermal.infiltration_enthalpy_j_m2_ofe_ground;
        if !enthalpy_close(
            thermal.ending_infiltration_credit_j_m2_ofe_ground,
            expected_credit,
        ) || !enthalpy_close(thermal.ending_enthalpy_j_m2_ofe_ground, expected_ending)
        {
            return Err(receiver_atomic_failure(
                operands,
                Some(&thermal.ofe_id),
                Some(&thermal.tile_id),
                "soil-thermal infiltration enthalpy ending equation",
            ));
        }
    }
    for tile in &operands.lse_tiles {
        if !tile.tile_fraction.is_finite() || tile.tile_fraction <= 0.0 {
            return Err(receiver_atomic_failure(
                operands,
                Some(&tile.ofe_id),
                Some(&tile.tile_id),
                "LSE retained tile fraction",
            ));
        }
        let expected = tile.beginning_enthalpy_j_m2_tile_ground
            + tile.retained_enthalpy_j_m2_ofe_ground / tile.tile_fraction;
        if !enthalpy_close(tile.ending_enthalpy_j_m2_tile_ground, expected) {
            return Err(receiver_atomic_failure(
                operands,
                Some(&tile.ofe_id),
                Some(&tile.tile_id),
                "LSE retained enthalpy ending equation",
            ));
        }
    }
    Ok(())
}

fn validate_production_soil_receiver_closure(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    for lane in &operands.production_soil {
        if lane.ordered_layers.is_empty()
            || !lane.infiltration_m.is_finite()
            || lane.infiltration_m < 0.0
            || !lane.tillage_depth_m.is_finite()
        {
            return Err(receiver_atomic_failure(
                operands,
                Some(&lane.ofe_id),
                None,
                "production soil receiver operand domain",
            ));
        }
        let expected = independently_reconstruct_infiltration(lane).ok_or_else(|| {
            receiver_atomic_failure(
                operands,
                Some(&lane.ofe_id),
                None,
                "production soil receiver reconstruction domain",
            )
        })?;
        for (layer, expected_ending) in lane.ordered_layers.iter().zip(expected) {
            if !mass_m_close(layer.ending_liquid_m, expected_ending) {
                return Err(receiver_atomic_failure(
                    operands,
                    Some(&lane.ofe_id),
                    None,
                    "ordered production soil-layer infiltration equation",
                ));
            }
        }
        let beginning_sum = lane
            .ordered_layers
            .iter()
            .map(|layer| {
                layer.beginning_liquid_m
                    + layer.residual_theta * (layer.layer_depth_m - layer.frozen_depth_m).max(0.0)
            })
            .sum::<f64>();
        let ending_sum = lane
            .ordered_layers
            .iter()
            .map(|layer| {
                layer.ending_liquid_m
                    + layer.residual_theta * (layer.layer_depth_m - layer.frozen_depth_m).max(0.0)
            })
            .sum::<f64>();
        if !mass_m_close(lane.beginning_aggregate_soil_water_m, beginning_sum)
            || !mass_m_close(lane.ending_aggregate_soil_water_m, ending_sum)
            || !mass_m_close(
                lane.ending_aggregate_soil_water_m,
                lane.beginning_aggregate_soil_water_m + lane.infiltration_m,
            )
        {
            return Err(receiver_atomic_failure(
                operands,
                Some(&lane.ofe_id),
                None,
                "aggregate production soil-water ending equation",
            ));
        }
    }
    Ok(())
}

fn independently_reconstruct_infiltration(
    lane: &ProductionSoilReceiverOperands,
) -> Option<Vec<f64>> {
    let first_depth = lane.ordered_layers.first()?.layer_depth_m;
    let resolved_tillage_depth_m = if lane.tillage_depth_m > 1.0e-12 {
        lane.tillage_depth_m
    } else {
        first_depth
    };
    if !resolved_tillage_depth_m.is_finite() || resolved_tillage_depth_m <= 0.0 {
        return None;
    }
    let mut remaining = lane.infiltration_m;
    let mut cumulative_depth_m = 0.0;
    let mut expected = lane
        .ordered_layers
        .iter()
        .map(|layer| layer.beginning_liquid_m)
        .collect::<Vec<_>>();
    for (layer, ending) in lane.ordered_layers.iter().zip(&mut expected) {
        if remaining <= 0.0 {
            break;
        }
        if !layer.layer_depth_m.is_finite()
            || layer.layer_depth_m <= 0.0
            || !layer.residual_theta.is_finite()
            || layer.residual_theta < 0.0
            || !layer.frozen_depth_m.is_finite()
            || layer.frozen_depth_m < 0.0
        {
            return None;
        }
        cumulative_depth_m += layer.layer_depth_m;
        let addition = if cumulative_depth_m < resolved_tillage_depth_m - 1.0e-12 {
            remaining * layer.layer_depth_m / resolved_tillage_depth_m
        } else {
            remaining
        };
        *ending += addition.max(0.0);
        remaining -= addition;
    }
    if remaining > 0.0 {
        *expected.last_mut()? += remaining;
    }
    Some(expected)
}

fn mass_m_close(actual: f64, expected: f64) -> bool {
    let scale = actual.abs() + expected.abs();
    actual.is_finite()
        && expected.is_finite()
        && (actual - expected).abs() <= 1.0e-17 + 64.0 * f64::EPSILON * scale
}

fn enthalpy_close(actual: f64, expected: f64) -> bool {
    let scale = actual.abs() + expected.abs();
    actual.is_finite()
        && expected.is_finite()
        && (actual - expected).abs() <= 1.0e-9 + 64.0 * f64::EPSILON * scale
}

fn receiver_atomic_failure(
    operands: &RealReceiverClosureOperands,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::atomic_envelope_failure(
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(operands.transaction_id),
            owner_id: Some(operands.hydrology_owner_id.clone()),
            ofe_id: ofe_id.cloned(),
            tile_id: tile_id.cloned(),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        Some(operands.beginning_hydrology_snapshot_sha256.to_string()),
        Some(receiver_operands_sha256(operands)),
        detail,
    )
}

fn receiver_operands_sha256(operands: &RealReceiverClosureOperands) -> String {
    let mut digest = Sha256::new();
    digest.update(b"openwepp-real-receiver-closure-operands-v1");
    digest.update(operands.transaction_id.0.to_be_bytes());
    digest.update(operands.hydrology_owner_id.as_str().as_bytes());
    for lane in &operands.production_soil {
        digest.update(lane.ofe_id.as_str().as_bytes());
        digest.update(lane.production_lane_index.to_be_bytes());
        digest.update(lane.production_lane_id.to_be_bytes());
        for value in [
            lane.tillage_depth_m,
            lane.infiltration_m,
            lane.beginning_aggregate_soil_water_m,
            lane.ending_aggregate_soil_water_m,
        ] {
            digest.update(value.to_bits().to_be_bytes());
        }
        for layer in &lane.ordered_layers {
            digest.update(layer.layer_id.as_str().as_bytes());
            digest.update(layer.beginning_liquid_m.to_bits().to_be_bytes());
            digest.update(layer.ending_liquid_m.to_bits().to_be_bytes());
            digest.update(layer.layer_depth_m.to_bits().to_be_bytes());
            digest.update(layer.residual_theta.to_bits().to_be_bytes());
            digest.update(layer.frozen_depth_m.to_bits().to_be_bytes());
        }
    }
    for thermal in &operands.soil_thermal {
        digest.update(thermal.ofe_id.as_str().as_bytes());
        digest.update(thermal.tile_id.as_str().as_bytes());
        digest.update(thermal.layer_id.as_str().as_bytes());
        for value in [
            thermal.beginning_infiltration_credit_j_m2_ofe_ground,
            thermal.ending_infiltration_credit_j_m2_ofe_ground,
            thermal.beginning_enthalpy_j_m2_ofe_ground,
            thermal.infiltration_enthalpy_j_m2_ofe_ground,
            thermal.ending_enthalpy_j_m2_ofe_ground,
        ] {
            digest.update(value.to_bits().to_be_bytes());
        }
    }
    for tile in &operands.lse_tiles {
        digest.update(tile.ofe_id.as_str().as_bytes());
        digest.update(tile.tile_id.as_str().as_bytes());
        for value in [
            tile.tile_fraction,
            tile.beginning_enthalpy_j_m2_tile_ground,
            tile.retained_enthalpy_j_m2_ofe_ground,
            tile.ending_enthalpy_j_m2_tile_ground,
        ] {
            digest.update(value.to_bits().to_be_bytes());
        }
    }
    format!("{:x}", digest.finalize())
}

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
                debit / WATER_DENSITY_KG_M3
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
    let cap_rate = ground_authorization.authorization.amount_kg_m2_stand_ground
        / (beginning.tile_fraction * beginning.interval_s);
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
