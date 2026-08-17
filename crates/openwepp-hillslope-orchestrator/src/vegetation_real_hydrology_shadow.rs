//! Default-off V7 root-water arbitration against real direct hydrology state.
//!
//! This adapter is deliberately not reachable from a runtime selector. It
//! freezes and owns a full clone of a day-start [`DirectRunFrame`], while the
//! V7 vegetation resource DTO remains unchanged inside a V8-precursor root/OFE
//! source envelope. The complete V8 ground/surface-class identity is not
//! claimed here.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use openwepp_kernel_contract::{
    MaximumAuthorization, OccupancyId, ResourceAmountBasis, ResourceOwnerId,
    ResourceProtocolViolation, SoilLayerId, TransactionId, WaterResourceKey,
    canonical_resource_amount_sum, validate_resource_protocol,
};
use openwepp_vegetation::{
    CoupledOwnedState, ModelDefinition, SnowFreeForcing, UncommittedWaterPhase,
    VegetationConfiguration, VegetationError, WaterArbiter, WaterArbitration, WaterAuthorization,
    WaterAuthorizationReason, WaterOwnerCandidate, WaterOwnerSnapshot, WaterRequest, WaterUse,
    execute_uncommitted_water_phase, reconstruct_water_ending,
};

use crate::direct_runtime::{
    DirectLayerWithdrawalRequest, aggregate_direct_soil_water,
    apply_direct_finalized_layer_liquid_debit, authorize_direct_layer_withdrawals,
};
use crate::{DirectDayFrame, DirectRunFrame, DirectSubsurfaceLayerState};

const WATER_DENSITY_KG_M3: f64 = 1_000.0;
type LayerFacts = BTreeMap<RealHydrologySourceKey, RealHydrologyLayerFact>;
type LayerIndexes = BTreeMap<RealHydrologySourceKey, usize>;

/// The only amount basis accepted by the real-hydrology shadow boundary.
pub const REAL_HYDROLOGY_WATER_BASIS: ResourceAmountBasis =
    ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval;

/// Exact typed identity of one production OFE/lane.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RealHydrologyOfeLaneId {
    pub lane_index: usize,
    pub lane_id: u32,
}

/// Exact production supply identity used for arbitration and debit.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RealHydrologySourceKey {
    pub ofe_lane: RealHydrologyOfeLaneId,
    pub layer_id: SoilLayerId,
}

/// Ordered configured layer mapping for one production OFE/lane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealHydrologyLaneLayerMap {
    pub ofe_lane: RealHydrologyOfeLaneId,
    pub layer_ids: Vec<SoilLayerId>,
}

/// V8-precursor root/OFE envelope around the unchanged V7 [`WaterRequest`].
#[derive(Clone, Debug, PartialEq)]
pub struct RealHydrologyWaterRequest {
    pub transaction_id: TransactionId,
    pub interval_s: f64,
    pub requester: OccupancyId,
    pub source: RealHydrologySourceKey,
    pub basis: ResourceAmountBasis,
    pub rooted: bool,
    pub request: WaterRequest,
}

/// Source-key envelope for a V7 finalized use returned by the capped solve.
#[derive(Clone, Debug, PartialEq)]
pub struct RealHydrologyWaterUse {
    pub transaction_id: TransactionId,
    pub interval_s: f64,
    pub requester: OccupancyId,
    pub source: RealHydrologySourceKey,
    pub basis: ResourceAmountBasis,
    pub finalized_use: WaterUse,
}

/// Immutable production fact associated with one configured source layer.
#[derive(Clone, Debug, PartialEq)]
pub struct RealHydrologyLayerFact {
    pub source: RealHydrologySourceKey,
    pub liquid_supply_kg_m2: f64,
    pub frozen: bool,
}

/// Maximum authorization and reason with the exact real-hydrology source key.
#[derive(Clone, Debug, PartialEq)]
pub struct RealHydrologyAuthorization {
    pub transaction_id: TransactionId,
    pub interval_s: f64,
    pub requester: OccupancyId,
    pub source: RealHydrologySourceKey,
    pub basis: ResourceAmountBasis,
    pub authorization: WaterAuthorization,
    pub reason: WaterAuthorizationReason,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ArbitrationKey {
    requester: WaterResourceKey,
    source: RealHydrologySourceKey,
}

/// Validated immutable authorization result bound to its full beginning frame.
#[derive(Clone, Debug, PartialEq)]
pub struct RealHydrologyArbitration {
    beginning_frame: DirectRunFrame,
    run_id: u64,
    hillslope_id: u32,
    day_index: usize,
    transaction_id: TransactionId,
    interval_s: f64,
    hydrology_owner_id: ResourceOwnerId,
    snapshot_bytes: Vec<u8>,
    snapshot_fingerprint: String,
    requests: Vec<RealHydrologyWaterRequest>,
    authorizations: Vec<RealHydrologyAuthorization>,
}

impl RealHydrologyArbitration {
    #[must_use]
    pub fn authorizations(&self) -> &[RealHydrologyAuthorization] {
        &self.authorizations
    }

    #[must_use]
    pub fn requests(&self) -> &[RealHydrologyWaterRequest] {
        &self.requests
    }

    #[must_use]
    pub fn snapshot_bytes(&self) -> &[u8] {
        &self.snapshot_bytes
    }

    #[must_use]
    pub fn snapshot_fingerprint(&self) -> &str {
        &self.snapshot_fingerprint
    }
}

/// Full real-hydrology shadow candidate. No production frame is mutated.
#[derive(Clone, Debug, PartialEq)]
pub struct RealHydrologyShadowCandidate {
    beginning_frame: DirectRunFrame,
    ending_frame: DirectRunFrame,
    transaction_id: TransactionId,
    interval_s: f64,
    finalized_uses: Vec<RealHydrologyWaterUse>,
    beginning_snapshot_bytes: Vec<u8>,
    beginning_snapshot_fingerprint: String,
}

impl RealHydrologyShadowCandidate {
    #[must_use]
    pub fn beginning_frame(&self) -> &DirectRunFrame {
        &self.beginning_frame
    }

    #[must_use]
    pub fn ending_frame(&self) -> &DirectRunFrame {
        &self.ending_frame
    }

    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub fn finalized_uses(&self) -> &[RealHydrologyWaterUse] {
        &self.finalized_uses
    }

    #[must_use]
    pub fn beginning_snapshot_bytes(&self) -> &[u8] {
        &self.beginning_snapshot_bytes
    }

    #[must_use]
    pub fn beginning_snapshot_fingerprint(&self) -> &str {
        &self.beginning_snapshot_fingerprint
    }
}

/// Typed fail-closed errors for the real-hydrology shadow boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RealHydrologyShadowError {
    Identity(&'static str),
    Operand(&'static str),
    Bound(&'static str),
    Protocol(ResourceProtocolViolation),
}

impl fmt::Display for RealHydrologyShadowError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identity(detail) => write!(formatter, "VEGTXN-E-001: {detail}"),
            Self::Operand(detail) => write!(formatter, "VEGTXN-E-002: {detail}"),
            Self::Bound(detail) => write!(formatter, "VEGTXN-E-003: {detail}"),
            Self::Protocol(violation) => {
                let code = match violation.category() {
                    openwepp_kernel_contract::ResourceProtocolCategory::Identity => "VEGTXN-E-001",
                    openwepp_kernel_contract::ResourceProtocolCategory::Operand => "VEGTXN-E-002",
                    openwepp_kernel_contract::ResourceProtocolCategory::Bound => "VEGTXN-E-003",
                };
                write!(formatter, "{code}: {violation:?}")
            }
        }
    }
}

impl std::error::Error for RealHydrologyShadowError {}

fn vegetation_boundary_error(error: RealHydrologyShadowError) -> VegetationError {
    match error {
        RealHydrologyShadowError::Identity(detail) => {
            VegetationError::ResourceIdentity(detail.into())
        }
        RealHydrologyShadowError::Operand(detail) => {
            VegetationError::ResourceOperand(detail.into())
        }
        RealHydrologyShadowError::Bound(detail) => VegetationError::ResourceBound(detail.into()),
        RealHydrologyShadowError::Protocol(violation) => VegetationError::from(violation),
    }
}

impl From<ResourceProtocolViolation> for RealHydrologyShadowError {
    fn from(value: ResourceProtocolViolation) -> Self {
        Self::Protocol(value)
    }
}

/// Immutable day-start adapter over a complete clone of production state.
#[derive(Clone, Debug, PartialEq)]
pub struct RealHydrologyShadowAdapter {
    beginning_frame: DirectRunFrame,
    beginning_day_frames: Vec<DirectDayFrame>,
    day_index: usize,
    transaction_id: TransactionId,
    interval_s: f64,
    hydrology_owner_id: ResourceOwnerId,
    layer_facts: BTreeMap<RealHydrologySourceKey, RealHydrologyLayerFact>,
    layer_indexes: BTreeMap<RealHydrologySourceKey, usize>,
    layer_maps: Vec<RealHydrologyLaneLayerMap>,
    snapshot_bytes: Vec<u8>,
    snapshot_fingerprint: String,
}

impl RealHydrologyShadowAdapter {
    /// Freeze the real day-start owner. `layer_maps` must exactly cover every
    /// production lane and its configured layer order.
    pub fn try_from_day_start(
        frame: &DirectRunFrame,
        day_index: usize,
        transaction_id: TransactionId,
        interval_s: f64,
        hydrology_owner_id: ResourceOwnerId,
        layer_maps: &[RealHydrologyLaneLayerMap],
    ) -> Result<Self, RealHydrologyShadowError> {
        if day_index >= frame.identity.day_count {
            return Err(RealHydrologyShadowError::Identity(
                "day index outside run identity",
            ));
        }
        if !interval_s.is_finite() || interval_s <= 0.0 {
            return Err(RealHydrologyShadowError::Operand(
                "invalid arbitration interval",
            ));
        }
        if frame.lanes.len() != frame.identity.lane_count || layer_maps.len() != frame.lanes.len() {
            return Err(RealHydrologyShadowError::Identity(
                "OFE/lane count mismatch",
            ));
        }

        let beginning_day_frames = (0..frame.lanes.len())
            .map(|lane_index| frame.seed_day_frame(lane_index, day_index))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| RealHydrologyShadowError::Identity("production day-frame seed failed"))?;

        let (layer_facts, layer_indexes) =
            extract_layer_facts(frame, &beginning_day_frames, layer_maps)?;

        let snapshot_bytes = canonical_snapshot_bytes(
            frame,
            &beginning_day_frames,
            day_index,
            interval_s,
            transaction_id,
            &hydrology_owner_id,
            layer_maps,
        );
        let snapshot_fingerprint = deterministic_fingerprint(&snapshot_bytes);
        Ok(Self {
            beginning_frame: frame.clone(),
            beginning_day_frames,
            day_index,
            transaction_id,
            interval_s,
            hydrology_owner_id,
            layer_facts,
            layer_indexes,
            layer_maps: layer_maps.to_vec(),
            snapshot_bytes,
            snapshot_fingerprint,
        })
    }

    #[must_use]
    pub fn beginning_frame(&self) -> &DirectRunFrame {
        &self.beginning_frame
    }

    #[must_use]
    pub fn beginning_day_frames(&self) -> &[DirectDayFrame] {
        &self.beginning_day_frames
    }

    #[must_use]
    pub fn layer_facts(&self) -> &BTreeMap<RealHydrologySourceKey, RealHydrologyLayerFact> {
        &self.layer_facts
    }

    pub(crate) fn layer_index_for_source(
        &self,
        source: &RealHydrologySourceKey,
    ) -> Result<usize, RealHydrologyShadowError> {
        self.layer_index(source)
    }

    #[must_use]
    pub(crate) fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub(crate) fn hydrology_owner_id(&self) -> &ResourceOwnerId {
        &self.hydrology_owner_id
    }

    #[must_use]
    pub(crate) fn day_index(&self) -> usize {
        self.day_index
    }

    #[must_use]
    pub(crate) fn interval_s(&self) -> f64 {
        self.interval_s
    }

    #[must_use]
    pub(crate) fn layer_maps(&self) -> &[RealHydrologyLaneLayerMap] {
        &self.layer_maps
    }

    #[must_use]
    pub fn snapshot_bytes(&self) -> &[u8] {
        &self.snapshot_bytes
    }

    pub fn authorize(
        &self,
        requests: &[RealHydrologyWaterRequest],
    ) -> Result<RealHydrologyArbitration, RealHydrologyShadowError> {
        let mut eligible = Vec::new();
        let mut positive_eligible_count_by_source =
            BTreeMap::<RealHydrologySourceKey, usize>::new();
        let mut seen = BTreeSet::new();
        for (index, envelope) in requests.iter().enumerate() {
            self.validate_request(envelope)?;
            let identity = ArbitrationKey {
                requester: envelope.request.key.clone(),
                source: envelope.source.clone(),
            };
            if !seen.insert(identity.clone()) {
                return Err(RealHydrologyShadowError::Identity(
                    "duplicate source-keyed request identity",
                ));
            }
            let fact = &self.layer_facts[&envelope.source];
            if envelope.rooted && !fact.frozen {
                if envelope.request.amount > 0.0 {
                    *positive_eligible_count_by_source
                        .entry(envelope.source.clone())
                        .or_default() += 1;
                }
                eligible.push((identity, index));
            }
        }
        eligible.sort_by(|(left, _), (right, _)| left.cmp(right));
        let direct_requests = eligible
            .iter()
            .enumerate()
            .map(|(canonical_rank, (_, index))| {
                let envelope = &requests[*index];
                Ok(DirectLayerWithdrawalRequest {
                    lane_index: envelope.source.ofe_lane.lane_index,
                    layer_index: self.layer_index(&envelope.source)?,
                    canonical_rank,
                    amount_kg_m2: envelope.request.amount,
                })
            })
            .collect::<Result<Vec<_>, RealHydrologyShadowError>>()?;
        let direct_authorizations =
            authorize_direct_layer_withdrawals(&self.beginning_day_frames, &direct_requests)
                .map_err(|_| {
                    RealHydrologyShadowError::Operand(
                        "production hydrology withdrawal authorization rejected",
                    )
                })?;
        let proportional_by_index = eligible
            .iter()
            .map(|(_, index)| *index)
            .zip(direct_authorizations)
            .collect::<BTreeMap<_, _>>();

        let mut authorizations = Vec::with_capacity(requests.len());
        for (index, envelope) in requests.iter().enumerate() {
            let fact = &self.layer_facts[&envelope.source];
            let (amount, reason) = classify_authorization(
                envelope,
                fact,
                proportional_by_index.get(&index).copied(),
                positive_eligible_count_by_source
                    .get(&envelope.source)
                    .copied()
                    .unwrap_or(0),
            )?;
            authorizations.push(RealHydrologyAuthorization {
                transaction_id: envelope.transaction_id,
                interval_s: envelope.interval_s,
                requester: envelope.requester.clone(),
                source: envelope.source.clone(),
                basis: envelope.basis,
                authorization: MaximumAuthorization {
                    transaction_id: envelope.request.transaction_id,
                    owner_id: envelope.request.owner_id.clone(),
                    key: envelope.request.key.clone(),
                    amount,
                    basis: envelope.request.basis,
                },
                reason,
            });
        }

        Ok(RealHydrologyArbitration {
            beginning_frame: self.beginning_frame.clone(),
            run_id: self.beginning_frame.identity.run_id,
            hillslope_id: self.beginning_frame.identity.hillslope_id,
            day_index: self.day_index,
            transaction_id: self.transaction_id,
            interval_s: self.interval_s,
            hydrology_owner_id: self.hydrology_owner_id.clone(),
            snapshot_bytes: self.snapshot_bytes.clone(),
            snapshot_fingerprint: self.snapshot_fingerprint.clone(),
            requests: requests.to_vec(),
            authorizations,
        })
    }

    #[allow(clippy::too_many_lines)]
    pub fn candidate_from_finalized_uses(
        &self,
        arbitration: &RealHydrologyArbitration,
        finalized_uses: &[RealHydrologyWaterUse],
    ) -> Result<RealHydrologyShadowCandidate, RealHydrologyShadowError> {
        self.validate_arbitration(arbitration)?;
        if finalized_uses.len() != arbitration.requests.len() {
            return Err(RealHydrologyShadowError::Identity(
                "finalized-use key set mismatch",
            ));
        }
        let authorization_by_key = arbitration
            .authorizations
            .iter()
            .map(|value| {
                (
                    ArbitrationKey {
                        requester: value.authorization.key.clone(),
                        source: value.source.clone(),
                    },
                    value,
                )
            })
            .collect::<BTreeMap<_, _>>();
        let request_by_key = arbitration
            .requests
            .iter()
            .map(|value| {
                (
                    ArbitrationKey {
                        requester: value.request.key.clone(),
                        source: value.source.clone(),
                    },
                    value,
                )
            })
            .collect::<BTreeMap<_, _>>();
        let mut seen = BTreeSet::new();
        let mut debit_amounts_by_source =
            BTreeMap::<RealHydrologySourceKey, BTreeMap<ArbitrationKey, f64>>::new();
        for envelope in finalized_uses {
            self.validate_use_envelope(envelope)?;
            let key = ArbitrationKey {
                requester: envelope.finalized_use.key.clone(),
                source: envelope.source.clone(),
            };
            if !seen.insert(key.clone()) {
                return Err(RealHydrologyShadowError::Identity(
                    "duplicate finalized-use identity",
                ));
            }
            let request = request_by_key
                .get(&key)
                .ok_or(RealHydrologyShadowError::Identity(
                    "unknown finalized-use source identity",
                ))?;
            let authorization =
                authorization_by_key
                    .get(&key)
                    .ok_or(RealHydrologyShadowError::Identity(
                        "missing source authorization",
                    ))?;
            validate_resource_protocol(
                &request.request,
                &authorization.authorization,
                &envelope.finalized_use,
            )?;
            debit_amounts_by_source
                .entry(envelope.source.clone())
                .or_default()
                .insert(key, envelope.finalized_use.amount);
        }
        if seen.len() != request_by_key.len() {
            return Err(RealHydrologyShadowError::Identity(
                "finalized-use key set mismatch",
            ));
        }

        let debit_by_source = debit_amounts_by_source
            .into_iter()
            .map(|(source, amounts)| {
                canonical_resource_amount_sum(&amounts)
                    .map(|amount| (source, amount))
                    .map_err(RealHydrologyShadowError::Protocol)
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;

        let mut ending_frame = self.beginning_frame.clone();
        for (source, debit_kg_m2) in debit_by_source {
            if !debit_kg_m2.is_finite() || debit_kg_m2 < 0.0 {
                return Err(RealHydrologyShadowError::Operand(
                    "invalid aggregate finalized debit",
                ));
            }
            let fact = self
                .layer_facts
                .get(&source)
                .ok_or(RealHydrologyShadowError::Identity("unknown debit source"))?;
            if debit_kg_m2 > fact.liquid_supply_kg_m2 {
                return Err(RealHydrologyShadowError::Bound(
                    "finalized debit exceeds beginning liquid",
                ));
            }
            let lane = ending_frame
                .lanes
                .get_mut(source.ofe_lane.lane_index)
                .ok_or(RealHydrologyShadowError::Identity("debit OFE/lane missing"))?;
            if lane.lane_id != source.ofe_lane.lane_id {
                return Err(RealHydrologyShadowError::Identity(
                    "debit OFE/lane identity mismatch",
                ));
            }
            let mapping_index = self.layer_index(&source)?;
            let layer = lane
                .subsurface_layers
                .get_mut(mapping_index)
                .ok_or(RealHydrologyShadowError::Identity("debit layer missing"))?;
            let debit_m = if debit_kg_m2.to_bits() == fact.liquid_supply_kg_m2.to_bits() {
                layer.theta_m
            } else {
                debit_kg_m2 / WATER_DENSITY_KG_M3
            };
            apply_direct_finalized_layer_liquid_debit(layer, debit_m).map_err(|_| {
                RealHydrologyShadowError::Bound("production finalized layer debit rejected")
            })?;
        }
        for lane in &mut ending_frame.lanes {
            lane.water.soil_water_m = aggregate_soil_water_m(&lane.subsurface_layers)?;
        }

        Ok(RealHydrologyShadowCandidate {
            beginning_frame: self.beginning_frame.clone(),
            ending_frame,
            transaction_id: self.transaction_id,
            interval_s: self.interval_s,
            finalized_uses: finalized_uses.to_vec(),
            beginning_snapshot_bytes: self.snapshot_bytes.clone(),
            beginning_snapshot_fingerprint: self.snapshot_fingerprint.clone(),
        })
    }

    fn validate_request(
        &self,
        envelope: &RealHydrologyWaterRequest,
    ) -> Result<(), RealHydrologyShadowError> {
        if envelope.transaction_id != self.transaction_id
            || envelope.transaction_id != envelope.request.transaction_id
            || envelope.interval_s.to_bits() != self.interval_s.to_bits()
            || envelope.requester != envelope.request.key.occupancy_id
            || envelope.source.layer_id != envelope.request.key.layer_id
            || !self.layer_facts.contains_key(&envelope.source)
        {
            return Err(RealHydrologyShadowError::Identity(
                "request envelope identity or basis mismatch",
            ));
        }
        if envelope.basis != REAL_HYDROLOGY_WATER_BASIS || envelope.basis != envelope.request.basis
        {
            return Err(RealHydrologyShadowError::Operand(
                "request envelope amount basis mismatch",
            ));
        }
        if !envelope.request.amount.is_finite() || envelope.request.amount < 0.0 {
            return Err(RealHydrologyShadowError::Operand(
                "invalid water request amount",
            ));
        }
        Ok(())
    }

    fn validate_use_envelope(
        &self,
        envelope: &RealHydrologyWaterUse,
    ) -> Result<(), RealHydrologyShadowError> {
        if envelope.transaction_id != self.transaction_id
            || envelope.transaction_id != envelope.finalized_use.transaction_id
            || envelope.interval_s.to_bits() != self.interval_s.to_bits()
            || envelope.requester != envelope.finalized_use.key.occupancy_id
            || envelope.source.layer_id != envelope.finalized_use.key.layer_id
            || !self.layer_facts.contains_key(&envelope.source)
        {
            return Err(RealHydrologyShadowError::Identity(
                "finalized-use envelope identity or basis mismatch",
            ));
        }
        if envelope.basis != REAL_HYDROLOGY_WATER_BASIS
            || envelope.basis != envelope.finalized_use.basis
        {
            return Err(RealHydrologyShadowError::Operand(
                "finalized-use envelope amount basis mismatch",
            ));
        }
        Ok(())
    }

    fn validate_arbitration(
        &self,
        arbitration: &RealHydrologyArbitration,
    ) -> Result<(), RealHydrologyShadowError> {
        if arbitration.beginning_frame != self.beginning_frame
            || arbitration.run_id != self.beginning_frame.identity.run_id
            || arbitration.hillslope_id != self.beginning_frame.identity.hillslope_id
            || arbitration.day_index != self.day_index
            || arbitration.transaction_id != self.transaction_id
            || arbitration.interval_s.to_bits() != self.interval_s.to_bits()
            || arbitration.hydrology_owner_id != self.hydrology_owner_id
            || arbitration.snapshot_bytes != self.snapshot_bytes
            || arbitration.snapshot_fingerprint != self.snapshot_fingerprint
            || arbitration.requests.len() != arbitration.authorizations.len()
        {
            return Err(RealHydrologyShadowError::Identity(
                "stale or foreign arbitration envelope",
            ));
        }
        Ok(())
    }

    fn layer_index(
        &self,
        source: &RealHydrologySourceKey,
    ) -> Result<usize, RealHydrologyShadowError> {
        self.layer_indexes
            .get(source)
            .copied()
            .ok_or(RealHydrologyShadowError::Identity(
                "configured layer index missing",
            ))
    }
}

fn classify_authorization(
    envelope: &RealHydrologyWaterRequest,
    fact: &RealHydrologyLayerFact,
    eligible_authorization: Option<f64>,
    positive_eligible_count: usize,
) -> Result<(f64, WaterAuthorizationReason), RealHydrologyShadowError> {
    if envelope.request.amount == 0.0 {
        return Ok((0.0, WaterAuthorizationReason::ZeroDemand));
    }
    if !envelope.rooted {
        return Ok((0.0, WaterAuthorizationReason::RootingExclusion));
    }
    if fact.frozen {
        return Ok((0.0, WaterAuthorizationReason::FrozenExclusion));
    }
    let authorization = eligible_authorization.ok_or(RealHydrologyShadowError::Identity(
        "eligible request lacks production authorization",
    ))?;
    let reason = if authorization.to_bits() == envelope.request.amount.to_bits() {
        WaterAuthorizationReason::FullySupplied
    } else if fact.liquid_supply_kg_m2 == 0.0 {
        WaterAuthorizationReason::LiquidStorageLimit
    } else if positive_eligible_count > 1 {
        WaterAuthorizationReason::CompetingDemand
    } else {
        WaterAuthorizationReason::LiquidStorageLimit
    };
    Ok((authorization, reason))
}

/// One explicit V7 vegetation-to-real-hydrology bridge for a selected OFE.
///
/// The mature V7 constitutive solve remains unchanged. This bridge adds the
/// V8-precursor root/OFE owner envelope at the orchestrator boundary and records the
/// complete real owner candidate produced from the finalized V7 uses.
pub struct SingleOfeRealHydrologyWaterArbiter {
    adapter: RealHydrologyShadowAdapter,
    ofe_lane: RealHydrologyOfeLaneId,
    rooted_layers: BTreeSet<SoilLayerId>,
    real_arbitration: RefCell<Option<RealHydrologyArbitration>>,
    real_candidate: RefCell<Option<RealHydrologyShadowCandidate>>,
}

impl SingleOfeRealHydrologyWaterArbiter {
    pub fn try_new(
        adapter: RealHydrologyShadowAdapter,
        ofe_lane: RealHydrologyOfeLaneId,
        rooted_layers: BTreeSet<SoilLayerId>,
    ) -> Result<Self, RealHydrologyShadowError> {
        let available_layers = adapter
            .layer_facts
            .keys()
            .filter(|source| source.ofe_lane == ofe_lane)
            .map(|source| source.layer_id.clone())
            .collect::<BTreeSet<_>>();
        if available_layers.is_empty() || !rooted_layers.is_subset(&available_layers) {
            return Err(RealHydrologyShadowError::Identity(
                "selected OFE or rooted-layer identity mismatch",
            ));
        }
        Ok(Self {
            adapter,
            ofe_lane,
            rooted_layers,
            real_arbitration: RefCell::new(None),
            real_candidate: RefCell::new(None),
        })
    }

    #[must_use]
    pub fn real_candidate(&self) -> Option<RealHydrologyShadowCandidate> {
        self.real_candidate.borrow().clone()
    }

    fn source_for_layer(
        &self,
        layer_id: &SoilLayerId,
    ) -> Result<RealHydrologySourceKey, VegetationError> {
        let source = RealHydrologySourceKey {
            ofe_lane: self.ofe_lane,
            layer_id: layer_id.clone(),
        };
        if !self.adapter.layer_facts.contains_key(&source) {
            return Err(VegetationError::ResourceIdentity(
                "real hydrology source layer unavailable".into(),
            ));
        }
        Ok(source)
    }

    fn project_arbitration(
        &self,
        real: &RealHydrologyArbitration,
        transaction_id: TransactionId,
        owner_id: ResourceOwnerId,
    ) -> Result<WaterArbitration, VegetationError> {
        let beginning = real
            .requests
            .iter()
            .map(|request| {
                let fact = self
                    .adapter
                    .layer_facts
                    .get(&request.source)
                    .ok_or_else(|| {
                        VegetationError::ResourceIdentity(
                            "real hydrology snapshot source missing".into(),
                        )
                    })?;
                Ok((request.source.layer_id.clone(), fact.liquid_supply_kg_m2))
            })
            .collect::<Result<BTreeMap<_, _>, VegetationError>>()?;
        let authorizations = real
            .authorizations
            .iter()
            .map(|value| value.authorization.clone())
            .collect::<Vec<_>>();
        let reasons = real
            .authorizations
            .iter()
            .map(|value| (value.authorization.key.clone(), value.reason))
            .collect::<BTreeMap<_, _>>();
        let snapshot =
            WaterOwnerSnapshot::try_new(transaction_id, owner_id, beginning, reasons.clone())?;
        WaterArbitration::try_new(snapshot, authorizations, reasons)
    }
}

impl WaterArbiter for SingleOfeRealHydrologyWaterArbiter {
    fn authorize(&self, requests: &[WaterRequest]) -> Result<WaterArbitration, VegetationError> {
        if self.real_arbitration.borrow().is_some() {
            return Err(VegetationError::ResourceIdentity(
                "real hydrology authorization already issued".into(),
            ));
        }
        let first = requests.first().ok_or_else(|| {
            VegetationError::ResourceIdentity("empty requests require zero-demand path".into())
        })?;
        let transaction_id = first.transaction_id;
        let owner_id = first.owner_id.clone();
        let envelopes = requests
            .iter()
            .map(|request| {
                let source = self.source_for_layer(&request.key.layer_id)?;
                Ok(RealHydrologyWaterRequest {
                    transaction_id: request.transaction_id,
                    interval_s: self.adapter.interval_s,
                    requester: request.key.occupancy_id.clone(),
                    source,
                    basis: request.basis,
                    rooted: self.rooted_layers.contains(&request.key.layer_id),
                    request: request.clone(),
                })
            })
            .collect::<Result<Vec<_>, VegetationError>>()?;
        let real = self
            .adapter
            .authorize(&envelopes)
            .map_err(vegetation_boundary_error)?;
        let projected = self.project_arbitration(&real, transaction_id, owner_id)?;
        self.real_arbitration.replace(Some(real));
        Ok(projected)
    }

    fn authorize_zero_demand(
        &self,
        transaction_id: TransactionId,
        owner_id: &ResourceOwnerId,
    ) -> Result<WaterArbitration, VegetationError> {
        if self.real_arbitration.borrow().is_some() {
            return Err(VegetationError::ResourceIdentity(
                "real hydrology authorization already issued".into(),
            ));
        }
        if transaction_id != self.adapter.transaction_id {
            return Err(VegetationError::ResourceIdentity(
                "zero-demand transaction differs from real hydrology snapshot".into(),
            ));
        }
        let real = self
            .adapter
            .authorize(&[])
            .map_err(vegetation_boundary_error)?;
        let beginning = self
            .adapter
            .layer_facts
            .iter()
            .filter(|(source, _)| source.ofe_lane == self.ofe_lane)
            .map(|(source, fact)| (source.layer_id.clone(), fact.liquid_supply_kg_m2))
            .collect::<BTreeMap<_, _>>();
        let snapshot = WaterOwnerSnapshot::try_new(
            transaction_id,
            owner_id.clone(),
            beginning,
            BTreeMap::new(),
        )?;
        let projected = WaterArbitration::try_new(snapshot, Vec::new(), BTreeMap::new())?;
        self.real_arbitration.replace(Some(real));
        Ok(projected)
    }

    fn candidate_from_finalized_use(
        &self,
        transaction_id: TransactionId,
        arbitration: &WaterArbitration,
        finalized_uses: &[WaterUse],
    ) -> Result<WaterOwnerCandidate, VegetationError> {
        if self.real_candidate.borrow().is_some() {
            return Err(VegetationError::ResourceIdentity(
                "real hydrology candidate already constructed".into(),
            ));
        }
        let real_arbitration = self.real_arbitration.borrow().clone().ok_or_else(|| {
            VegetationError::ResourceIdentity(
                "real hydrology candidate before authorization".into(),
            )
        })?;
        let expected_arbitration = self.project_arbitration(
            &real_arbitration,
            transaction_id,
            arbitration.snapshot().owner_id().clone(),
        )?;
        if arbitration != &expected_arbitration {
            return Err(VegetationError::ResourceIdentity(
                "vegetation and real-hydrology authorization mismatch".into(),
            ));
        }
        let envelopes = finalized_uses
            .iter()
            .map(|use_record| {
                let source = self.source_for_layer(&use_record.key.layer_id)?;
                Ok(RealHydrologyWaterUse {
                    transaction_id: use_record.transaction_id,
                    interval_s: self.adapter.interval_s,
                    requester: use_record.key.occupancy_id.clone(),
                    source,
                    basis: use_record.basis,
                    finalized_use: use_record.clone(),
                })
            })
            .collect::<Result<Vec<_>, VegetationError>>()?;
        let real_candidate = self
            .adapter
            .candidate_from_finalized_uses(&real_arbitration, &envelopes)
            .map_err(vegetation_boundary_error)?;
        let ending = reconstruct_water_ending(arbitration.snapshot(), finalized_uses)?;
        let projected = WaterOwnerCandidate::try_new(
            transaction_id,
            arbitration.snapshot().owner_id().clone(),
            arbitration.snapshot().clone(),
            ending,
            finalized_uses.to_vec(),
        )?;
        self.real_candidate.replace(Some(real_candidate));
        Ok(projected)
    }
}

/// Complete bounded Child-2 result: exact V7 two-pass water phase plus the
/// actual cloned direct-hydrology owner candidate.
pub struct V7RealHydrologyWaterShadow {
    pub water_phase: UncommittedWaterPhase,
    pub real_hydrology_candidate: RealHydrologyShadowCandidate,
}

pub fn execute_v7_real_hydrology_water_shadow(
    model: &ModelDefinition,
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    adapter: RealHydrologyShadowAdapter,
    ofe_lane: RealHydrologyOfeLaneId,
) -> Result<V7RealHydrologyWaterShadow, VegetationError> {
    if adapter.beginning_frame.lanes.len() != 1 || ofe_lane.lane_index != 0 {
        return Err(VegetationError::Unsupported(
            "Child-2 executable vegetation bridge is single-OFE; routed multi-OFE coordination belongs to the real-consumer shadow",
        ));
    }
    if adapter.interval_s.to_bits() != configuration.dt_s.to_bits() {
        return Err(VegetationError::ResourceIdentity(
            "real-hydrology interval differs from vegetation interval".into(),
        ));
    }
    let expected_transaction = beginning
        .last_transaction_id
        .checked_add(1)
        .ok_or_else(|| {
            VegetationError::ResourceIdentity("vegetation transaction identity overflow".into())
        })?;
    if adapter.transaction_id.0 != expected_transaction {
        return Err(VegetationError::ResourceIdentity(
            "real-hydrology transaction differs from vegetation candidate".into(),
        ));
    }
    let mut source_layers = adapter
        .layer_indexes
        .iter()
        .filter(|(source, _)| source.ofe_lane == ofe_lane)
        .map(|(source, index)| (*index, source))
        .collect::<Vec<_>>();
    source_layers.sort_by_key(|(index, _)| *index);
    if source_layers.len() != forcing.soil_layers.len() {
        return Err(VegetationError::ResourceIdentity(
            "real-hydrology and vegetation layer cardinality differ".into(),
        ));
    }
    let mut rooted_layers = BTreeSet::new();
    for ((expected_index, source), forcing_layer) in source_layers.iter().zip(&forcing.soil_layers)
    {
        if *expected_index >= forcing.soil_layers.len() || source.layer_id != forcing_layer.layer_id
        {
            return Err(VegetationError::ResourceIdentity(
                "real-hydrology and vegetation layer order differ".into(),
            ));
        }
        let fact = &adapter.layer_facts[*source];
        if fact.liquid_supply_kg_m2.to_bits() != forcing_layer.water_beginning_kg_m2.to_bits()
            || fact.frozen != forcing_layer.frozen
        {
            return Err(VegetationError::ResourceIdentity(
                "real-hydrology and vegetation layer snapshot differ".into(),
            ));
        }
        if forcing_layer.accessible {
            rooted_layers.insert(source.layer_id.clone());
        }
    }
    let arbiter = SingleOfeRealHydrologyWaterArbiter::try_new(adapter, ofe_lane, rooted_layers)
        .map_err(vegetation_boundary_error)?;
    let water_phase =
        execute_uncommitted_water_phase(model, configuration, beginning, forcing, &arbiter)?;
    let real_hydrology_candidate = arbiter.real_candidate().ok_or_else(|| {
        VegetationError::Receipt("real hydrology candidate was not constructed".into())
    })?;
    Ok(V7RealHydrologyWaterShadow {
        water_phase,
        real_hydrology_candidate,
    })
}

fn extract_layer_facts(
    frame: &DirectRunFrame,
    day_frames: &[DirectDayFrame],
    layer_maps: &[RealHydrologyLaneLayerMap],
) -> Result<(LayerFacts, LayerIndexes), RealHydrologyShadowError> {
    let mut layer_facts = BTreeMap::new();
    let mut layer_indexes = BTreeMap::new();
    for (lane_index, ((lane, day_frame), mapping)) in frame
        .lanes
        .iter()
        .zip(day_frames)
        .zip(layer_maps)
        .enumerate()
    {
        let expected_lane = RealHydrologyOfeLaneId {
            lane_index,
            lane_id: lane.lane_id,
        };
        if mapping.ofe_lane != expected_lane
            || mapping.layer_ids.len() != day_frame.percolation_inputs.layers.len()
        {
            return Err(RealHydrologyShadowError::Identity(
                "configured OFE/layer mapping mismatch",
            ));
        }
        let mut unique_layers = BTreeSet::new();
        for (layer_index, (layer_id, layer)) in mapping
            .layer_ids
            .iter()
            .zip(&day_frame.percolation_inputs.layers)
            .enumerate()
        {
            if !unique_layers.insert(layer_id.clone()) {
                return Err(RealHydrologyShadowError::Identity(
                    "duplicate configured layer identity",
                ));
            }
            validate_layer(layer)?;
            let source = RealHydrologySourceKey {
                ofe_lane: expected_lane,
                layer_id: layer_id.clone(),
            };
            let fully_frozen = layer.frozen_depth_m.to_bits() == layer.depth_m.to_bits();
            let unfrozen = layer.frozen_depth_m == 0.0 && layer.frozen_water_m == 0.0;
            if !fully_frozen && !unfrozen {
                return Err(RealHydrologyShadowError::Operand(
                    "partially frozen layer requires a future typed forcing surface",
                ));
            }
            let fact = RealHydrologyLayerFact {
                source: source.clone(),
                liquid_supply_kg_m2: layer.theta_m * WATER_DENSITY_KG_M3,
                frozen: fully_frozen,
            };
            if layer_facts.insert(source.clone(), fact).is_some()
                || layer_indexes.insert(source, layer_index).is_some()
            {
                return Err(RealHydrologyShadowError::Identity(
                    "duplicate production source identity",
                ));
            }
        }
        let aggregate = aggregate_soil_water_m(&day_frame.percolation_inputs.layers)?;
        if aggregate.to_bits() != day_frame.water.soil_water_m.to_bits() {
            return Err(RealHydrologyShadowError::Operand(
                "day-start aggregate soil-water identity mismatch",
            ));
        }
    }
    Ok((layer_facts, layer_indexes))
}

fn validate_layer(layer: &DirectSubsurfaceLayerState) -> Result<(), RealHydrologyShadowError> {
    let values = [
        layer.theta_m,
        layer.field_capacity_m,
        layer.upper_limit_m,
        layer.conductivity_m_s,
        layer.depth_m,
        layer.residual_theta,
        layer.frozen_depth_m,
        layer.frozen_water_m,
        layer.porosity,
        layer.field_capacity_theta,
        layer.coca,
        layer.lateral_conductivity_m_s,
    ];
    if values.iter().any(|value| !value.is_finite())
        || layer.theta_m < 0.0
        || layer.residual_theta < 0.0
        || layer.depth_m < 0.0
        || layer.frozen_depth_m < 0.0
        || layer.frozen_depth_m > layer.depth_m
        || layer.frozen_water_m < 0.0
    {
        return Err(RealHydrologyShadowError::Operand(
            "invalid production layer state",
        ));
    }
    Ok(())
}

fn canonical_snapshot_bytes(
    frame: &DirectRunFrame,
    day_frames: &[DirectDayFrame],
    day_index: usize,
    interval_s: f64,
    transaction_id: TransactionId,
    hydrology_owner_id: &ResourceOwnerId,
    layer_maps: &[RealHydrologyLaneLayerMap],
) -> Vec<u8> {
    fn push_u64(bytes: &mut Vec<u8>, value: u64) {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    fn push_f64(bytes: &mut Vec<u8>, value: f64) {
        bytes.extend_from_slice(&value.to_bits().to_le_bytes());
    }
    fn push_text(bytes: &mut Vec<u8>, value: &str) {
        push_u64(bytes, value.len() as u64);
        bytes.extend_from_slice(value.as_bytes());
    }

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"openwepp-real-hydrology-shadow-v1\0");
    push_u64(&mut bytes, frame.identity.run_id);
    push_u64(&mut bytes, u64::from(frame.identity.hillslope_id));
    push_u64(&mut bytes, frame.identity.lane_count as u64);
    push_u64(&mut bytes, frame.identity.day_count as u64);
    push_u64(&mut bytes, day_index as u64);
    push_f64(&mut bytes, interval_s);
    bytes.extend_from_slice(&transaction_id.0.to_le_bytes());
    push_text(&mut bytes, hydrology_owner_id.as_str());
    for ((lane, day_frame), mapping) in frame.lanes.iter().zip(day_frames).zip(layer_maps) {
        push_u64(&mut bytes, u64::from(lane.lane_id));
        push_u64(&mut bytes, u64::from(lane.upstream_lane_id));
        push_u64(&mut bytes, u64::from(lane.downstream_lane_id));
        for value in [
            lane.upstream_area_ratio,
            lane.area_m2,
            day_frame.water.soil_water_m,
            day_frame.water.infiltration_m,
            day_frame.water.runoff_m,
            day_frame.water.evapotranspiration_m,
            day_frame.water.drainage_m,
            day_frame.water.lateral_flow_m,
            day_frame.transfer.upstream_flow_m,
            day_frame.transfer.subsurface_input_m,
        ] {
            push_f64(&mut bytes, value);
        }
        for values in [
            &day_frame.transfer.surface_carry_m,
            &day_frame.transfer.surface_hourly_weights,
            &day_frame.transfer.lateral_carry_m,
        ] {
            for value in values {
                push_f64(&mut bytes, *value);
            }
        }
        push_u64(&mut bytes, mapping.layer_ids.len() as u64);
        for (layer_id, layer) in mapping
            .layer_ids
            .iter()
            .zip(&day_frame.percolation_inputs.layers)
        {
            push_text(&mut bytes, layer_id.as_str());
            for value in [
                layer.theta_m,
                layer.field_capacity_m,
                layer.upper_limit_m,
                layer.conductivity_m_s,
                layer.depth_m,
                layer.residual_theta,
                layer.frozen_depth_m,
                layer.frozen_water_m,
                layer.porosity,
                layer.field_capacity_theta,
                layer.coca,
                layer.lateral_conductivity_m_s,
            ] {
                push_f64(&mut bytes, value);
            }
        }
    }
    bytes
}

fn deterministic_fingerprint(bytes: &[u8]) -> String {
    // Stable FNV-1a join token. The canonical bytes remain exposed for exact
    // collision-free comparison; this compact token is not a security digest.
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

fn aggregate_soil_water_m(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, RealHydrologyShadowError> {
    aggregate_direct_soil_water(layers, "real_hydrology_shadow.aggregate_soil_water_m")
        .map_err(|_| RealHydrologyShadowError::Operand("invalid aggregate soil water"))
}

#[cfg(test)]
#[allow(clippy::cast_precision_loss, clippy::float_cmp)]
mod tests_extra {
    use super::*;
    use openwepp_kernel_contract::{FinalizedUse, StratumId, TileId};
    use openwepp_vegetation::WaterRequest;

    fn layer(theta_m: f64, frozen: bool) -> DirectSubsurfaceLayerState {
        DirectSubsurfaceLayerState {
            theta_m,
            field_capacity_m: 0.02,
            upper_limit_m: 0.2,
            conductivity_m_s: 1.0e-6,
            depth_m: 0.3,
            residual_theta: 0.01,
            frozen_depth_m: if frozen { 0.3 } else { 0.0 },
            frozen_water_m: if frozen { 0.01 } else { 0.0 },
            porosity: 0.45,
            field_capacity_theta: 0.25,
            coca: 0.1,
            lateral_conductivity_m_s: 1.0e-7,
        }
    }

    fn frame(lane_count: usize) -> DirectRunFrame {
        let identity = crate::DirectRunIdentity::new(17, 23, lane_count, 2)
            .expect("valid direct-run identity");
        let mut frame = DirectRunFrame::skeleton(identity).expect("valid direct-run frame");
        for (index, lane) in frame.lanes.iter_mut().enumerate() {
            lane.area_m2 = 100.0 + index as f64;
            lane.subsurface_layers =
                vec![layer(0.03 + index as f64 * 0.01, false), layer(0.07, false)];
            lane.water.soil_water_m =
                aggregate_soil_water_m(&lane.subsurface_layers).expect("finite aggregate");
        }
        frame
    }

    fn layer_maps(frame: &DirectRunFrame) -> Vec<RealHydrologyLaneLayerMap> {
        frame
            .lanes
            .iter()
            .enumerate()
            .map(|(lane_index, lane)| RealHydrologyLaneLayerMap {
                ofe_lane: RealHydrologyOfeLaneId {
                    lane_index,
                    lane_id: lane.lane_id,
                },
                // Deliberately reverse lexical order. Candidate debit must use
                // configured vector position, never BTreeMap key position.
                layer_ids: vec![
                    SoilLayerId::try_new("z-layer").expect("layer identity"),
                    SoilLayerId::try_new("a-layer").expect("layer identity"),
                ],
            })
            .collect()
    }

    fn occupancy(name: &str) -> OccupancyId {
        OccupancyId {
            stratum_id: StratumId::try_new(name).expect("stratum identity"),
            tile_id: TileId::try_new("tile-1").expect("tile identity"),
        }
    }

    fn request(
        transaction_id: TransactionId,
        source: RealHydrologySourceKey,
        requester: OccupancyId,
        amount: f64,
        rooted: bool,
    ) -> RealHydrologyWaterRequest {
        let vegetation_owner = ResourceOwnerId::try_new("vegetation").expect("owner identity");
        RealHydrologyWaterRequest {
            transaction_id,
            interval_s: 86_400.0,
            requester: requester.clone(),
            source: source.clone(),
            basis: REAL_HYDROLOGY_WATER_BASIS,
            rooted,
            request: WaterRequest {
                transaction_id,
                owner_id: vegetation_owner,
                key: WaterResourceKey {
                    occupancy_id: requester,
                    layer_id: source.layer_id,
                },
                amount,
                basis: REAL_HYDROLOGY_WATER_BASIS,
            },
        }
    }

    fn finalized(request: &RealHydrologyWaterRequest, amount: f64) -> RealHydrologyWaterUse {
        RealHydrologyWaterUse {
            transaction_id: request.transaction_id,
            interval_s: request.interval_s,
            requester: request.requester.clone(),
            source: request.source.clone(),
            basis: request.basis,
            finalized_use: FinalizedUse {
                transaction_id: request.request.transaction_id,
                owner_id: request.request.owner_id.clone(),
                key: request.request.key.clone(),
                amount,
                basis: request.request.basis,
            },
        }
    }

    #[test]
    fn real_owner_preserves_requester_identity_and_configured_layer_order() {
        let frame = frame(1);
        let before = frame.clone();
        let maps = layer_maps(&frame);
        let transaction_id = TransactionId(41);
        let hydrology_owner =
            ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner identity");
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &frame,
            0,
            transaction_id,
            86_400.0,
            hydrology_owner,
            &maps,
        )
        .expect("valid adapter");
        let source = RealHydrologySourceKey {
            ofe_lane: maps[0].ofe_lane,
            layer_id: maps[0].layer_ids[0].clone(),
        };
        let request = request(transaction_id, source, occupancy("upper"), 20.0, true);
        let arbitration = adapter
            .authorize(std::slice::from_ref(&request))
            .expect("authorization");
        assert_eq!(arbitration.authorizations()[0].authorization.amount, 20.0);
        let candidate = adapter
            .candidate_from_finalized_uses(&arbitration, &[finalized(&request, 7.0)])
            .expect("real candidate");
        assert_eq!(
            candidate.ending_frame().lanes[0].subsurface_layers[0].theta_m,
            0.023
        );
        assert_eq!(
            candidate.ending_frame().lanes[0].subsurface_layers[1].theta_m,
            0.07
        );
        assert_eq!(frame, before);
        assert_eq!(
            candidate.beginning_snapshot_bytes(),
            arbitration.snapshot_bytes()
        );
    }

    #[test]
    fn real_owner_competes_equal_status_and_debits_finalized_use_only() {
        let frame = frame(2);
        let before = frame.clone();
        let maps = layer_maps(&frame);
        let transaction_id = TransactionId(42);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &frame,
            1,
            transaction_id,
            86_400.0,
            ResourceOwnerId::try_new("production-hydrology").expect("owner"),
            &maps,
        )
        .expect("valid adapter");
        let source = RealHydrologySourceKey {
            ofe_lane: maps[1].ofe_lane,
            layer_id: maps[1].layer_ids[0].clone(),
        };
        let left = request(
            transaction_id,
            source.clone(),
            occupancy("left"),
            30.0,
            true,
        );
        let right = request(transaction_id, source, occupancy("right"), 30.0, true);
        let arbitration = adapter
            .authorize(&[left.clone(), right.clone()])
            .expect("authorization");
        assert_eq!(arbitration.authorizations()[0].authorization.amount, 20.0);
        assert_eq!(arbitration.authorizations()[1].authorization.amount, 20.0);
        assert!(
            arbitration
                .authorizations()
                .iter()
                .all(|value| value.reason == WaterAuthorizationReason::CompetingDemand)
        );
        let candidate = adapter
            .candidate_from_finalized_uses(
                &arbitration,
                &[finalized(&left, 10.0), finalized(&right, 5.0)],
            )
            .expect("real candidate");
        assert_eq!(
            candidate.ending_frame().lanes[1].subsurface_layers[0].theta_m,
            0.025
        );
        assert_eq!(candidate.ending_frame().lanes[0], frame.lanes[0]);
        assert_eq!(frame, before);
    }

    #[test]
    fn real_owner_retains_zero_frozen_and_rooting_facts() {
        let mut frame = frame(1);
        frame.lanes[0].subsurface_layers[1] = layer(0.07, true);
        frame.lanes[0].water.soil_water_m =
            aggregate_soil_water_m(&frame.lanes[0].subsurface_layers).expect("aggregate");
        let maps = layer_maps(&frame);
        let transaction_id = TransactionId(43);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &frame,
            0,
            transaction_id,
            86_400.0,
            ResourceOwnerId::try_new("production-hydrology").expect("owner"),
            &maps,
        )
        .expect("valid adapter");
        let frozen_source = RealHydrologySourceKey {
            ofe_lane: maps[0].ofe_lane,
            layer_id: maps[0].layer_ids[1].clone(),
        };
        let frozen = request(
            transaction_id,
            frozen_source,
            occupancy("frozen"),
            3.0,
            true,
        );
        let unrooted = request(
            transaction_id,
            RealHydrologySourceKey {
                ofe_lane: maps[0].ofe_lane,
                layer_id: maps[0].layer_ids[0].clone(),
            },
            occupancy("unrooted"),
            3.0,
            false,
        );
        let zero = request(
            transaction_id,
            unrooted.source.clone(),
            occupancy("zero"),
            0.0,
            true,
        );
        let arbitration = adapter
            .authorize(&[frozen, unrooted, zero])
            .expect("authorization");
        assert_eq!(
            arbitration.authorizations()[0].reason,
            WaterAuthorizationReason::FrozenExclusion
        );
        assert_eq!(
            arbitration.authorizations()[1].reason,
            WaterAuthorizationReason::RootingExclusion
        );
        assert_eq!(
            arbitration.authorizations()[2].reason,
            WaterAuthorizationReason::ZeroDemand
        );
        assert!(adapter.layer_facts().values().any(|fact| fact.frozen));
    }

    #[test]
    fn real_owner_rejects_wrong_source_and_overuse_without_mutation() {
        let frame = frame(1);
        let before = frame.clone();
        let maps = layer_maps(&frame);
        let transaction_id = TransactionId(44);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &frame,
            0,
            transaction_id,
            86_400.0,
            ResourceOwnerId::try_new("production-hydrology").expect("owner"),
            &maps,
        )
        .expect("valid adapter");
        let source = RealHydrologySourceKey {
            ofe_lane: maps[0].ofe_lane,
            layer_id: maps[0].layer_ids[0].clone(),
        };
        let valid = request(transaction_id, source, occupancy("owner"), 10.0, true);
        let mut wrong = valid.clone();
        wrong.source.ofe_lane.lane_id = 99;
        assert!(matches!(
            adapter.authorize(&[wrong]),
            Err(RealHydrologyShadowError::Identity(_))
        ));

        let arbitration = adapter
            .authorize(std::slice::from_ref(&valid))
            .expect("authorization");
        assert!(matches!(
            adapter.candidate_from_finalized_uses(&arbitration, &[finalized(&valid, 11.0)]),
            Err(RealHydrologyShadowError::Protocol(
                ResourceProtocolViolation::FinalizedUseExceedsAuthorization
            ))
        ));
        assert_eq!(frame, before);
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::too_many_lines)]
mod tests {
    use super::*;
    use crate::{DirectRunIdentity, DirectWaterState};
    use openwepp_kernel_contract::{FinalizedUse, ResourceRequest, StratumId, TileId};

    fn layer(theta_m: f64, frozen_depth_m: f64) -> DirectSubsurfaceLayerState {
        DirectSubsurfaceLayerState {
            theta_m,
            field_capacity_m: 0.02,
            upper_limit_m: 0.2,
            conductivity_m_s: 0.000_001,
            depth_m: 0.5,
            residual_theta: 0.1,
            frozen_depth_m,
            frozen_water_m: if frozen_depth_m > 0.0 { 0.001 } else { 0.0 },
            porosity: 0.45,
            field_capacity_theta: 0.25,
            coca: 0.2,
            lateral_conductivity_m_s: 0.000_002,
        }
    }

    fn frame(theta_by_lane: &[Vec<(f64, f64)>]) -> DirectRunFrame {
        let identity = DirectRunIdentity::new(41, 7, theta_by_lane.len(), 2).expect("identity");
        let mut frame = DirectRunFrame::skeleton(identity).expect("frame");
        for (lane, layers) in frame.lanes.iter_mut().zip(theta_by_lane) {
            lane.area_m2 = 10.0 * f64::from(lane.lane_id);
            lane.water = DirectWaterState {
                soil_water_m: 0.0,
                infiltration_m: 0.003,
                runoff_m: 0.004,
                evapotranspiration_m: 0.005,
                drainage_m: 0.006,
                lateral_flow_m: 0.007,
            };
            lane.transfer.upstream_flow_m = 0.008;
            lane.transfer.subsurface_input_m = 0.009;
            lane.subsurface_layers = layers
                .iter()
                .map(|(theta, frozen)| layer(*theta, *frozen))
                .collect();
            lane.water.soil_water_m =
                aggregate_soil_water_m(&lane.subsurface_layers).expect("aggregate");
        }
        frame
    }

    fn layer_id(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("layer id")
    }

    fn owner(value: &str) -> ResourceOwnerId {
        ResourceOwnerId::try_new(value).expect("owner id")
    }

    fn occupancy(value: &str) -> OccupancyId {
        OccupancyId {
            stratum_id: StratumId::try_new(format!("stratum-{value}")).expect("stratum"),
            tile_id: TileId::try_new(format!("tile-{value}")).expect("tile"),
        }
    }

    fn maps(frame: &DirectRunFrame, names: &[Vec<&str>]) -> Vec<RealHydrologyLaneLayerMap> {
        frame
            .lanes
            .iter()
            .enumerate()
            .zip(names)
            .map(|((lane_index, lane), names)| RealHydrologyLaneLayerMap {
                ofe_lane: RealHydrologyOfeLaneId {
                    lane_index,
                    lane_id: lane.lane_id,
                },
                layer_ids: names.iter().map(|name| layer_id(name)).collect(),
            })
            .collect()
    }

    fn request(
        transaction_id: TransactionId,
        request_owner: &ResourceOwnerId,
        requester: OccupancyId,
        source: RealHydrologySourceKey,
        amount: f64,
        rooted: bool,
    ) -> RealHydrologyWaterRequest {
        RealHydrologyWaterRequest {
            transaction_id,
            interval_s: 86_400.0,
            requester: requester.clone(),
            source: source.clone(),
            basis: REAL_HYDROLOGY_WATER_BASIS,
            rooted,
            request: ResourceRequest {
                transaction_id,
                owner_id: request_owner.clone(),
                key: WaterResourceKey {
                    occupancy_id: requester,
                    layer_id: source.layer_id,
                },
                amount,
                basis: REAL_HYDROLOGY_WATER_BASIS,
            },
        }
    }

    fn finalized(request: &RealHydrologyWaterRequest, amount: f64) -> RealHydrologyWaterUse {
        RealHydrologyWaterUse {
            transaction_id: request.transaction_id,
            interval_s: request.interval_s,
            requester: request.requester.clone(),
            source: request.source.clone(),
            basis: request.basis,
            finalized_use: FinalizedUse {
                transaction_id: request.request.transaction_id,
                owner_id: request.request.owner_id.clone(),
                key: request.request.key.clone(),
                amount,
                basis: request.request.basis,
            },
        }
    }

    #[test]
    fn single_lane_reasons_cover_full_partial_zero_frozen_and_rooting() {
        let original = frame(&[vec![(0.010, 0.0), (0.004, 0.0), (0.0, 0.0), (0.005, 0.5)]]);
        let mapping = maps(&original, &[vec!["full", "partial", "zero", "frozen"]]);
        let transaction = TransactionId(9);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &original,
            0,
            transaction,
            86_400.0,
            owner("real-hydrology"),
            &mapping,
        )
        .expect("adapter");
        let lane = mapping[0].ofe_lane;
        let requester_owner = owner("vegetation-requester");
        let sources = mapping[0]
            .layer_ids
            .iter()
            .map(|layer_id| RealHydrologySourceKey {
                ofe_lane: lane,
                layer_id: layer_id.clone(),
            })
            .collect::<Vec<_>>();
        let requests = vec![
            request(
                transaction,
                &requester_owner,
                occupancy("full"),
                sources[0].clone(),
                5.0,
                true,
            ),
            request(
                transaction,
                &requester_owner,
                occupancy("p1"),
                sources[1].clone(),
                4.0,
                true,
            ),
            request(
                transaction,
                &requester_owner,
                occupancy("p2"),
                sources[1].clone(),
                4.0,
                true,
            ),
            request(
                transaction,
                &requester_owner,
                occupancy("zero"),
                sources[2].clone(),
                2.0,
                true,
            ),
            request(
                transaction,
                &requester_owner,
                occupancy("frozen"),
                sources[3].clone(),
                2.0,
                true,
            ),
            request(
                transaction,
                &requester_owner,
                occupancy("root"),
                sources[0].clone(),
                2.0,
                false,
            ),
        ];
        let arbitration = adapter.authorize(&requests).expect("authorization");
        let amounts = arbitration
            .authorizations()
            .iter()
            .map(|value| value.authorization.amount)
            .collect::<Vec<_>>();
        assert_eq!(amounts, vec![5.0, 2.0, 2.0, 0.0, 0.0, 0.0]);
        assert_eq!(
            arbitration
                .authorizations()
                .iter()
                .map(|value| value.reason)
                .collect::<Vec<_>>(),
            vec![
                WaterAuthorizationReason::FullySupplied,
                WaterAuthorizationReason::CompetingDemand,
                WaterAuthorizationReason::CompetingDemand,
                WaterAuthorizationReason::LiquidStorageLimit,
                WaterAuthorizationReason::FrozenExclusion,
                WaterAuthorizationReason::RootingExclusion,
            ]
        );
        assert_eq!(adapter.beginning_frame(), &original);
        assert_eq!(
            arbitration.snapshot_bytes(),
            adapter.snapshot_bytes.as_slice()
        );
        assert_eq!(
            arbitration.snapshot_fingerprint(),
            adapter.snapshot_fingerprint
        );
    }

    #[test]
    fn finalized_use_only_debits_configured_vector_index_and_preserves_full_clone() {
        let original = frame(&[vec![(0.008, 0.0), (0.012, 0.0)]]);
        let original_copy = original.clone();
        // Deliberately reverse lexical order to prove vector index is explicit.
        let mapping = maps(&original, &[vec!["z-layer", "a-layer"]]);
        let transaction = TransactionId(10);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &original,
            1,
            transaction,
            86_400.0,
            owner("hydrology"),
            &mapping,
        )
        .expect("adapter");
        let source = RealHydrologySourceKey {
            ofe_lane: mapping[0].ofe_lane,
            layer_id: mapping[0].layer_ids[1].clone(),
        };
        let request = request(
            transaction,
            &owner("vegetation"),
            occupancy("one"),
            source,
            10.0,
            true,
        );
        let arbitration = adapter
            .authorize(std::slice::from_ref(&request))
            .expect("authorize");
        assert_eq!(arbitration.authorizations()[0].authorization.amount, 10.0);
        let candidate = adapter
            .candidate_from_finalized_uses(&arbitration, &[finalized(&request, 3.0)])
            .expect("candidate");

        assert_eq!(
            original, original_copy,
            "immutable production input changed"
        );
        let mut expected = original_copy.clone();
        expected.lanes[0].subsurface_layers[1].theta_m -= 0.003;
        expected.lanes[0].water.soil_water_m =
            aggregate_soil_water_m(&expected.lanes[0].subsurface_layers).expect("aggregate");
        assert_eq!(candidate.beginning_frame(), &original_copy);
        assert_eq!(candidate.ending_frame(), &expected);
        assert_eq!(candidate.finalized_uses()[0].finalized_use.amount, 3.0);
        assert_eq!(
            candidate.beginning_snapshot_bytes(),
            arbitration.snapshot_bytes()
        );
        assert_eq!(
            candidate.beginning_snapshot_fingerprint(),
            arbitration.snapshot_fingerprint()
        );
    }

    #[test]
    fn multi_lane_same_v7_key_uses_distinct_real_sources() {
        let original = frame(&[vec![(0.002, 0.0)], vec![(0.006, 0.0)]]);
        let mapping = maps(&original, &[vec!["shared"], vec!["shared"]]);
        let transaction = TransactionId(11);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &original,
            0,
            transaction,
            86_400.0,
            owner("hydrology"),
            &mapping,
        )
        .expect("adapter");
        let common_occupancy = occupancy("same-v7-key");
        let requester_owner = owner("vegetation");
        let requests = mapping
            .iter()
            .map(|map| {
                request(
                    transaction,
                    &requester_owner,
                    common_occupancy.clone(),
                    RealHydrologySourceKey {
                        ofe_lane: map.ofe_lane,
                        layer_id: map.layer_ids[0].clone(),
                    },
                    4.0,
                    true,
                )
            })
            .collect::<Vec<_>>();
        let arbitration = adapter.authorize(&requests).expect("authorize");
        assert_eq!(
            arbitration
                .authorizations()
                .iter()
                .map(|value| value.authorization.amount)
                .collect::<Vec<_>>(),
            vec![2.0, 4.0]
        );
        let uses = vec![finalized(&requests[0], 1.0), finalized(&requests[1], 3.0)];
        let candidate = adapter
            .candidate_from_finalized_uses(&arbitration, &uses)
            .expect("candidate");
        assert_eq!(
            candidate.ending_frame().lanes[0].subsurface_layers[0].theta_m,
            0.001
        );
        assert_eq!(
            candidate.ending_frame().lanes[1].subsurface_layers[0].theta_m,
            0.003
        );
        assert_eq!(original.lanes[0].subsurface_layers[0].theta_m, 0.002);
        assert_eq!(original.lanes[1].subsurface_layers[0].theta_m, 0.006);
    }

    #[test]
    fn wrong_identity_basis_layer_and_failed_finalization_roll_back_exactly() {
        let original = frame(&[vec![(0.005, 0.0)]]);
        let original_copy = original.clone();
        let mapping = maps(&original, &[vec!["only"]]);
        let transaction = TransactionId(12);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &original,
            0,
            transaction,
            86_400.0,
            owner("hydrology"),
            &mapping,
        )
        .expect("adapter");
        let valid = request(
            transaction,
            &owner("vegetation"),
            occupancy("valid"),
            RealHydrologySourceKey {
                ofe_lane: mapping[0].ofe_lane,
                layer_id: mapping[0].layer_ids[0].clone(),
            },
            4.0,
            true,
        );

        let mut wrong_transaction = valid.clone();
        wrong_transaction.transaction_id = TransactionId(13);
        assert!(adapter.authorize(&[wrong_transaction]).is_err());
        let mut wrong_basis = valid.clone();
        wrong_basis.basis = ResourceAmountBasis::NitrogenKgPerSquareMeterInterval;
        wrong_basis.request.basis = wrong_basis.basis;
        assert!(adapter.authorize(&[wrong_basis]).is_err());
        let mut wrong_layer = valid.clone();
        wrong_layer.request.key.layer_id = layer_id("other");
        assert!(adapter.authorize(&[wrong_layer]).is_err());

        let arbitration = adapter
            .authorize(std::slice::from_ref(&valid))
            .expect("authorize");
        let excessive = finalized(&valid, 4.5);
        assert!(
            adapter
                .candidate_from_finalized_uses(&arbitration, &[excessive])
                .is_err()
        );
        let mut foreign_source = finalized(&valid, 1.0);
        foreign_source.source.layer_id = layer_id("other");
        assert!(
            adapter
                .candidate_from_finalized_uses(&arbitration, &[foreign_source])
                .is_err()
        );
        assert_eq!(original, original_copy);
        assert_eq!(adapter.beginning_frame(), &original_copy);
    }

    #[test]
    fn partial_frost_is_typed_unsupported_instead_of_whole_layer_exclusion() {
        let original = frame(&[vec![(0.005, 0.1)]]);
        let mapping = maps(&original, &[vec!["partial-frost"]]);
        assert!(matches!(
            RealHydrologyShadowAdapter::try_from_day_start(
                &original,
                0,
                TransactionId(13),
                86_400.0,
                owner("hydrology"),
                &mapping,
            ),
            Err(RealHydrologyShadowError::Operand(
                "partially frozen layer requires a future typed forcing surface"
            ))
        ));
    }

    #[test]
    fn single_request_nonzero_shortfall_is_a_liquid_storage_limit() {
        let original = frame(&[vec![(0.004, 0.0)]]);
        let mapping = maps(&original, &[vec!["limited"]]);
        let transaction = TransactionId(14);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &original,
            0,
            transaction,
            86_400.0,
            owner("hydrology"),
            &mapping,
        )
        .expect("adapter");
        let request = request(
            transaction,
            &owner("vegetation"),
            occupancy("single"),
            RealHydrologySourceKey {
                ofe_lane: mapping[0].ofe_lane,
                layer_id: mapping[0].layer_ids[0].clone(),
            },
            5.0,
            true,
        );
        let arbitration = adapter.authorize(&[request]).expect("authorization");
        assert_eq!(arbitration.authorizations()[0].authorization.amount, 4.0);
        assert_eq!(
            arbitration.authorizations()[0].reason,
            WaterAuthorizationReason::LiquidStorageLimit
        );
    }

    #[test]
    fn exact_full_depletion_survives_mass_depth_round_trip() {
        let theta_m = f64::from_bits(0x3fc4_6418_8ecb_23c0);
        let original = frame(&[vec![(theta_m, 0.0)]]);
        let mapping = maps(&original, &[vec!["deplete"]]);
        let transaction = TransactionId(15);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &original,
            0,
            transaction,
            86_400.0,
            owner("hydrology"),
            &mapping,
        )
        .expect("adapter");
        let amount = theta_m * WATER_DENSITY_KG_M3;
        let request = request(
            transaction,
            &owner("vegetation"),
            occupancy("deplete"),
            RealHydrologySourceKey {
                ofe_lane: mapping[0].ofe_lane,
                layer_id: mapping[0].layer_ids[0].clone(),
            },
            amount,
            true,
        );
        let arbitration = adapter
            .authorize(std::slice::from_ref(&request))
            .expect("authorization");
        let candidate = adapter
            .candidate_from_finalized_uses(&arbitration, &[finalized(&request, amount)])
            .expect("candidate");
        assert_eq!(
            candidate.ending_frame().lanes[0].subsurface_layers[0]
                .theta_m
                .to_bits(),
            0.0_f64.to_bits()
        );
    }

    #[test]
    fn every_rounded_maximum_can_be_finalized_without_source_overdraw() {
        let supply_kg_m2 = 15.653_309_008_252_922;
        let original = frame(&[vec![(supply_kg_m2 / WATER_DENSITY_KG_M3, 0.0)]]);
        let mapping = maps(&original, &[vec!["shared"]]);
        let transaction = TransactionId(16);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &original,
            0,
            transaction,
            86_400.0,
            owner("hydrology"),
            &mapping,
        )
        .expect("adapter");
        let requester_owner = owner("vegetation");
        let source = RealHydrologySourceKey {
            ofe_lane: mapping[0].ofe_lane,
            layer_id: mapping[0].layer_ids[0].clone(),
        };
        let requests = [
            request(
                transaction,
                &requester_owner,
                occupancy("a"),
                source.clone(),
                8.485_679_527_629_57,
                true,
            ),
            request(
                transaction,
                &requester_owner,
                occupancy("b"),
                source,
                9.282_475_483_155_647,
                true,
            ),
        ];
        let arbitration = adapter.authorize(&requests).expect("authorization");
        let uses = requests
            .iter()
            .zip(arbitration.authorizations())
            .map(|(request, authorization)| finalized(request, authorization.authorization.amount))
            .collect::<Vec<_>>();
        adapter
            .candidate_from_finalized_uses(&arbitration, &uses)
            .expect("all maxima form a valid candidate");
    }

    #[test]
    fn signed_zero_request_supply_and_frost_share_the_exact_zero_class() {
        let original = frame(&[vec![(-0.0, -0.0)]]);
        let mapping = maps(&original, &[vec!["zero"]]);
        let transaction = TransactionId(17);
        let adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &original,
            0,
            transaction,
            86_400.0,
            owner("hydrology"),
            &mapping,
        )
        .expect("signed-zero adapter");
        let requester_owner = owner("vegetation");
        let source = RealHydrologySourceKey {
            ofe_lane: mapping[0].ofe_lane,
            layer_id: mapping[0].layer_ids[0].clone(),
        };
        let requests = [
            request(
                transaction,
                &requester_owner,
                occupancy("zero-demand"),
                source.clone(),
                -0.0,
                true,
            ),
            request(
                transaction,
                &requester_owner,
                occupancy("positive-a"),
                source.clone(),
                1.0,
                true,
            ),
            request(
                transaction,
                &requester_owner,
                occupancy("positive-b"),
                source,
                2.0,
                true,
            ),
        ];
        let arbitration = adapter.authorize(&requests).expect("authorization");
        assert_eq!(
            arbitration.authorizations()[0].reason,
            WaterAuthorizationReason::ZeroDemand
        );
        assert!(
            arbitration.authorizations()[1..]
                .iter()
                .all(|authorization| {
                    authorization.authorization.amount == 0.0
                        && authorization.reason == WaterAuthorizationReason::LiquidStorageLimit
                })
        );
    }
}
