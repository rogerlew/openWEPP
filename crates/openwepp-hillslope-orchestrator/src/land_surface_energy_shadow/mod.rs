//! Default-off snow-free LSE arbitration against the actual direct hydrology owner.
//!
//! Only production-backed soil-layer liquid is admitted here. Direct runtime's
//! `residue_interception_m` is an ET input rebuilt from growth operands, not a
//! persistent hydrology-owned store, so forest-litter withdrawal is typed
//! unsupported. Direct runtime likewise has no accepted condensation-credit
//! endpoint; condensation remains fail-closed.

#![allow(clippy::missing_errors_doc)]

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{TransactionId, canonical_resource_amount_sum};
pub use openwepp_land_surface_energy::{
    BandDirectionalFluxes, BareSoilParameters, ComponentId, GroundWaterKey, OfeId,
    OpenNeutralGeometry, OpenSurfaceProblem, RequestingComponent, SoilThermalNodeOperands,
    SourceId, StandGroundWaterAmountBasis, SurfaceClass, SurfaceClassKind, SurfaceId,
    SurfaceStorageBranch, WaterAmount, WaterAuthorizationReason, WaterSourceType,
};
use openwepp_land_surface_energy::{
    LandSurfaceEnergyError, OpenSurfaceSolveOutcome, WaterAuthorization, WaterBranch,
    solve_open_surface,
};
use thiserror::Error;

use crate::DirectRunFrame;
use crate::direct_runtime::{
    DirectLayerWithdrawalRequest, aggregate_direct_soil_water,
    apply_direct_finalized_layer_liquid_debit, authorize_direct_layer_withdrawals,
};
use crate::vegetation_real_hydrology_shadow::{
    RealHydrologyShadowAdapter, RealHydrologyShadowError, RealHydrologySourceKey,
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
