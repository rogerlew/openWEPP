//! Uncommitted V6 water-phase orchestration.
//!
//! This stage executes the exact potential and fixed-authorization column
//! passes and asks the water owner to construct its debit candidate. It cannot
//! be committed: shared C/N, energy, biogeochemistry, and the atomic owner
//! transaction remain downstream obligations.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{
    ResourceOwnerId, SoilLayerId, TileId, TransactionId, validate_resource_protocol,
};

use crate::column::{OccupancyDiagnostics, TileColumnsResult};
use crate::occupancy_solver::capped_pass::{CappedWaterLayerOperands, execute_capped_column_pass};
use crate::occupancy_solver::evaluator::{
    ProductionCappedOccupancyEvaluator, ProductionPotentialOccupancyEvaluator,
};
use crate::occupancy_solver::radiation::PreparedRadiation;
use crate::occupancy_solver::request_pass::execute_potential_column_pass;
use crate::occupancy_solver::resources::ValidatedWaterAuthorizations;
use crate::transaction::{
    CoupledOwnedState, FailurePoint, SnowFreeForcing, WaterArbiter, WaterAuthorization,
    WaterRequest, WaterUse, validate_execution,
};
use crate::{ModelDefinition, VegetationConfiguration, VegetationError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaterAuthorizationReason {
    FullySupplied,
    ZeroDemand,
    LiquidStorageLimit,
    FrozenExclusion,
    RootingExclusion,
    CompetingDemand,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WaterOwnerSnapshot {
    transaction_id: TransactionId,
    owner_id: ResourceOwnerId,
    beginning_kg_m2_by_layer: BTreeMap<SoilLayerId, f64>,
    authorization_facts:
        BTreeMap<openwepp_kernel_contract::WaterResourceKey, WaterAuthorizationReason>,
}

impl WaterOwnerSnapshot {
    pub fn try_new(
        transaction_id: TransactionId,
        owner_id: ResourceOwnerId,
        beginning_kg_m2_by_layer: BTreeMap<SoilLayerId, f64>,
        authorization_facts: BTreeMap<
            openwepp_kernel_contract::WaterResourceKey,
            WaterAuthorizationReason,
        >,
    ) -> Result<Self, VegetationError> {
        if beginning_kg_m2_by_layer.is_empty()
            || beginning_kg_m2_by_layer
                .values()
                .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(VegetationError::Receipt(
                "invalid water-owner snapshot".into(),
            ));
        }
        Ok(Self {
            transaction_id,
            owner_id,
            beginning_kg_m2_by_layer,
            authorization_facts,
        })
    }

    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }
    #[must_use]
    pub fn owner_id(&self) -> &ResourceOwnerId {
        &self.owner_id
    }
    #[must_use]
    pub fn beginning_kg_m2_by_layer(&self) -> &BTreeMap<SoilLayerId, f64> {
        &self.beginning_kg_m2_by_layer
    }
    #[must_use]
    pub fn authorization_facts(
        &self,
    ) -> &BTreeMap<openwepp_kernel_contract::WaterResourceKey, WaterAuthorizationReason> {
        &self.authorization_facts
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WaterArbitration {
    snapshot: WaterOwnerSnapshot,
    authorizations: Vec<WaterAuthorization>,
    reasons: BTreeMap<openwepp_kernel_contract::WaterResourceKey, WaterAuthorizationReason>,
}

impl WaterArbitration {
    pub fn try_new(
        snapshot: WaterOwnerSnapshot,
        authorizations: Vec<WaterAuthorization>,
        reasons: BTreeMap<openwepp_kernel_contract::WaterResourceKey, WaterAuthorizationReason>,
    ) -> Result<Self, VegetationError> {
        let keys = authorizations
            .iter()
            .map(|value| value.key.clone())
            .collect::<BTreeSet<_>>();
        if keys.len() != authorizations.len()
            || keys != reasons.keys().cloned().collect()
            || reasons != snapshot.authorization_facts
            || authorizations.iter().any(|value| {
                value.transaction_id != snapshot.transaction_id
                    || value.owner_id != snapshot.owner_id
            })
        {
            return Err(VegetationError::Receipt(
                "invalid water arbitration identity".into(),
            ));
        }
        Ok(Self {
            snapshot,
            authorizations,
            reasons,
        })
    }

    #[must_use]
    pub fn snapshot(&self) -> &WaterOwnerSnapshot {
        &self.snapshot
    }
    #[must_use]
    pub fn authorizations(&self) -> &[WaterAuthorization] {
        &self.authorizations
    }
    #[must_use]
    pub fn reasons(
        &self,
    ) -> &BTreeMap<openwepp_kernel_contract::WaterResourceKey, WaterAuthorizationReason> {
        &self.reasons
    }
}

/// Receiving-owner state proposal. The owner supplies beginning and ending
/// stores; the vegetation boundary independently reconstructs and validates
/// every debit before exposing the enclosing water phase.
#[derive(Clone, Debug, PartialEq)]
pub struct WaterOwnerCandidate {
    transaction_id: TransactionId,
    owner_id: ResourceOwnerId,
    snapshot: WaterOwnerSnapshot,
    ending_kg_m2_by_layer: BTreeMap<SoilLayerId, f64>,
    finalized_uses: Vec<WaterUse>,
}

impl WaterOwnerCandidate {
    pub fn try_new(
        transaction_id: TransactionId,
        owner_id: ResourceOwnerId,
        snapshot: WaterOwnerSnapshot,
        ending_kg_m2_by_layer: BTreeMap<SoilLayerId, f64>,
        finalized_uses: Vec<WaterUse>,
    ) -> Result<Self, VegetationError> {
        if snapshot.transaction_id != transaction_id
            || snapshot.owner_id != owner_id
            || snapshot
                .beginning_kg_m2_by_layer
                .keys()
                .ne(ending_kg_m2_by_layer.keys())
            || snapshot
                .beginning_kg_m2_by_layer
                .values()
                .chain(ending_kg_m2_by_layer.values())
                .any(|value| !value.is_finite() || *value < 0.0)
            || finalized_uses.iter().any(|value| {
                value.transaction_id != transaction_id
                    || value.owner_id != owner_id
                    || !value.amount.is_finite()
                    || value.amount < 0.0
            })
        {
            return Err(VegetationError::Receipt(
                "invalid water-owner candidate surface".into(),
            ));
        }
        Ok(Self {
            transaction_id,
            owner_id,
            snapshot,
            ending_kg_m2_by_layer,
            finalized_uses,
        })
    }

    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub fn owner_id(&self) -> &ResourceOwnerId {
        &self.owner_id
    }

    #[must_use]
    pub fn beginning_kg_m2_by_layer(&self) -> &BTreeMap<SoilLayerId, f64> {
        &self.snapshot.beginning_kg_m2_by_layer
    }

    #[must_use]
    pub fn ending_kg_m2_by_layer(&self) -> &BTreeMap<SoilLayerId, f64> {
        &self.ending_kg_m2_by_layer
    }

    #[must_use]
    pub fn finalized_uses(&self) -> &[WaterUse] {
        &self.finalized_uses
    }
}

/// Canonical receiving-owner debit reconstruction. Exact resource-key order
/// is fixed by `BTreeMap`; every owner and validator uses the same aggregation
/// and one subtraction per layer.
pub fn reconstruct_water_ending(
    snapshot: &WaterOwnerSnapshot,
    finalized_uses: &[WaterUse],
) -> Result<BTreeMap<SoilLayerId, f64>, VegetationError> {
    let by_key = finalized_uses
        .iter()
        .map(|value| (value.key.clone(), value.amount))
        .collect::<BTreeMap<_, _>>();
    if by_key.len() != finalized_uses.len() {
        return Err(VegetationError::Receipt(
            "duplicate finalized water identity".into(),
        ));
    }
    let mut debits = BTreeMap::<SoilLayerId, f64>::new();
    for (key, amount) in by_key {
        if !amount.is_finite() || amount < 0.0 {
            return Err(VegetationError::Receipt(
                "invalid finalized water amount".into(),
            ));
        }
        *debits.entry(key.layer_id).or_default() += amount;
    }
    let mut ending = snapshot.beginning_kg_m2_by_layer.clone();
    if debits.is_empty() {
        return Ok(ending);
    }
    if ending.keys().ne(debits.keys()) {
        return Err(VegetationError::Receipt(
            "finalized water layer identity".into(),
        ));
    }
    for (layer, debit) in debits {
        let value = ending
            .get_mut(&layer)
            .ok_or(VegetationError::Domain("unknown water layer"))?;
        *value -= debit;
        if !value.is_finite() || *value < 0.0 {
            return Err(VegetationError::Receipt("water owner overdraft".into()));
        }
    }
    Ok(ending)
}

/// Complete physical water stage. This type deliberately has no conversion to
/// accepted state and has no commit method. The later sealed vegetation
/// proposal retains this phase for receiving-owner validation.
#[derive(Clone, Debug, PartialEq)]
pub struct UncommittedWaterPhase {
    beginning_state_sha256: String,
    transaction_id: TransactionId,
    interval_s: f64,
    requests: Vec<WaterRequest>,
    authorizations: Vec<WaterAuthorization>,
    finalized_uses: Vec<WaterUse>,
    potential_columns: TileColumnsResult,
    final_columns: TileColumnsResult,
    potential_radiation: PreparedRadiation,
    final_radiation: PreparedRadiation,
    potential_diagnostics: BTreeMap<openwepp_kernel_contract::OccupancyId, OccupancyDiagnostics>,
    final_diagnostics: BTreeMap<openwepp_kernel_contract::OccupancyId, OccupancyDiagnostics>,
    water_operands: Vec<CappedWaterLayerOperands>,
    water_owner_candidate: WaterOwnerCandidate,
}

impl UncommittedWaterPhase {
    #[must_use]
    pub fn beginning_state_sha256(&self) -> &str {
        &self.beginning_state_sha256
    }
    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }
    #[must_use]
    pub fn interval_s(&self) -> f64 {
        self.interval_s
    }
    #[must_use]
    pub fn protocol(&self) -> (&[WaterRequest], &[WaterAuthorization], &[WaterUse]) {
        (&self.requests, &self.authorizations, &self.finalized_uses)
    }
    #[must_use]
    pub fn potential_columns(&self) -> &TileColumnsResult {
        &self.potential_columns
    }
    #[must_use]
    pub fn final_columns(&self) -> &TileColumnsResult {
        &self.final_columns
    }
    #[must_use]
    pub fn potential_radiation(&self) -> &PreparedRadiation {
        &self.potential_radiation
    }
    #[must_use]
    pub fn final_radiation(&self) -> &PreparedRadiation {
        &self.final_radiation
    }
    #[must_use]
    pub fn potential_diagnostics(
        &self,
    ) -> &BTreeMap<openwepp_kernel_contract::OccupancyId, OccupancyDiagnostics> {
        &self.potential_diagnostics
    }
    #[must_use]
    pub fn final_diagnostics(
        &self,
    ) -> &BTreeMap<openwepp_kernel_contract::OccupancyId, OccupancyDiagnostics> {
        &self.final_diagnostics
    }
    #[must_use]
    pub fn water_operands(&self) -> &[CappedWaterLayerOperands] {
        &self.water_operands
    }
    #[must_use]
    pub fn water_owner_candidate(&self) -> &WaterOwnerCandidate {
        &self.water_owner_candidate
    }
    /// Reconstruct accepted E16/E17 operands exclusively from the sealed final
    /// capped pass. Potential columns cannot enter this boundary.
    pub fn final_stratum_carbon_operands(
        &self,
    ) -> Result<
        BTreeMap<openwepp_kernel_contract::StratumId, crate::carbon_phase::StratumCarbonOperands>,
        VegetationError,
    > {
        crate::carbon_phase::aggregate_stratum_carbon(&self.final_columns, self.interval_s)
    }

    /// Reconstruct potential E18/E19 operands exclusively from the sealed
    /// owner-uncapped pass. These operands form potential mineral-N demand;
    /// they cannot be substituted for the accepted capped allocation input.
    pub fn potential_stratum_carbon_operands(
        &self,
    ) -> Result<
        BTreeMap<openwepp_kernel_contract::StratumId, crate::carbon_phase::StratumCarbonOperands>,
        VegetationError,
    > {
        crate::carbon_phase::aggregate_potential_stratum_carbon(
            &self.potential_columns,
            self.interval_s,
        )
    }
}

pub fn execute_uncommitted_water_phase(
    model: &ModelDefinition,
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water: &dyn WaterArbiter,
) -> Result<UncommittedWaterPhase, VegetationError> {
    execute_uncommitted_water_phase_with_failure(
        model,
        configuration,
        beginning,
        forcing,
        water,
        None,
    )
}

#[allow(clippy::too_many_lines)]
pub fn execute_uncommitted_water_phase_with_failure(
    model: &ModelDefinition,
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water: &dyn WaterArbiter,
    failure: Option<FailurePoint>,
) -> Result<UncommittedWaterPhase, VegetationError> {
    if failure == Some(FailurePoint::Validation) {
        return Err(VegetationError::InjectedFailure("validation"));
    }
    validate_execution(model, configuration, beginning, forcing)?;
    let transaction_id = TransactionId(
        beginning
            .last_transaction_id
            .checked_add(1)
            .ok_or_else(|| VegetationError::Receipt("V6 transaction identity overflow".into()))?,
    );
    let owner_id = ResourceOwnerId::try_new("vegetation")
        .map_err(|error| VegetationError::Receipt(error.to_string()))?;
    let top_rain = configuration
        .topology_tiles
        .iter()
        .map(|tile| (tile.tile_id.clone(), forcing.rain_kg_m2))
        .collect::<BTreeMap<TileId, f64>>();

    if failure == Some(FailurePoint::Radiation) {
        return Err(VegetationError::InjectedFailure("radiation"));
    }
    if failure == Some(FailurePoint::Interception) {
        return Err(VegetationError::InjectedFailure("interception"));
    }
    if failure == Some(FailurePoint::PotentialCoupledSolve) {
        return Err(VegetationError::InjectedFailure("potential coupled solve"));
    }
    let potential_evaluator =
        ProductionPotentialOccupancyEvaluator::from_configuration(configuration)?;
    let potential = execute_potential_column_pass(
        configuration,
        beginning,
        forcing,
        transaction_id,
        owner_id.clone(),
        &top_rain,
        &potential_evaluator,
    )?;
    let requests = potential.water_requests.requests().to_vec();

    if failure == Some(FailurePoint::WaterAuthorization) {
        return Err(VegetationError::InjectedFailure("water authorization"));
    }
    let arbitration = if requests.is_empty() {
        water.authorize_zero_demand(transaction_id, &owner_id)?
    } else {
        water.authorize(&requests)?
    };
    validate_arbitration(transaction_id, &owner_id, &requests, &arbitration)?;
    let validated = ValidatedWaterAuthorizations::try_new(
        &potential.water_requests,
        arbitration.authorizations.clone(),
    )
    .map_err(VegetationError::from)?;
    let authorizations = requests
        .iter()
        .map(|request| {
            validated
                .authorizations()
                .get(&request.key)
                .cloned()
                .ok_or_else(|| {
                    VegetationError::Receipt("missing validated V6 water authorization".into())
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    if failure == Some(FailurePoint::CappedResolve) {
        return Err(VegetationError::InjectedFailure("capped resolve"));
    }
    let capped_evaluator = ProductionCappedOccupancyEvaluator::from_configuration(configuration)?;
    let capped = execute_capped_column_pass(
        configuration,
        beginning,
        forcing,
        transaction_id,
        configuration.dt_s,
        &top_rain,
        &validated,
        &capped_evaluator,
    )?;
    let water_owner_candidate = water.candidate_from_finalized_use(
        transaction_id,
        &arbitration,
        &capped.finalized_water_uses,
    )?;
    if failure == Some(FailurePoint::OwnerValidation) {
        return Err(VegetationError::InjectedFailure("water owner validation"));
    }
    validate_water_owner_candidate(
        transaction_id,
        &requests,
        &authorizations,
        &arbitration,
        &capped.finalized_water_uses,
        &water_owner_candidate,
    )?;

    Ok(UncommittedWaterPhase {
        beginning_state_sha256: beginning.state_sha256.clone(),
        transaction_id,
        interval_s: configuration.dt_s,
        requests,
        authorizations,
        finalized_uses: capped.finalized_water_uses,
        potential_columns: potential.columns,
        final_columns: capped.columns,
        potential_radiation: potential.radiation,
        final_radiation: capped.radiation,
        potential_diagnostics: potential.diagnostics,
        final_diagnostics: capped.diagnostics,
        water_operands: capped.water_operands,
        water_owner_candidate,
    })
}

fn validate_arbitration(
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
    requests: &[WaterRequest],
    arbitration: &WaterArbitration,
) -> Result<(), VegetationError> {
    let layers = requests
        .iter()
        .map(|request| request.key.layer_id.clone())
        .collect::<BTreeSet<_>>();
    if arbitration.snapshot.transaction_id != transaction_id
        || &arbitration.snapshot.owner_id != owner_id
        || (!requests.is_empty()
            && arbitration
                .snapshot
                .beginning_kg_m2_by_layer
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>()
                != layers)
        || arbitration.reasons != arbitration.snapshot.authorization_facts
        || arbitration.authorizations.len() != requests.len()
        || arbitration.reasons.len() != requests.len()
    {
        return Err(VegetationError::ResourceIdentity(
            "V6 immutable water snapshot identity".into(),
        ));
    }
    let requests_by_key = requests
        .iter()
        .map(|request| (request.key.clone(), request))
        .collect::<BTreeMap<_, _>>();
    let mut authorized_by_layer = BTreeMap::<SoilLayerId, f64>::new();
    for authorization in &arbitration.authorizations {
        let request = requests_by_key.get(&authorization.key).ok_or_else(|| {
            VegetationError::ResourceIdentity("V6 water arbitration request identity".into())
        })?;
        let reason = arbitration.reasons.get(&authorization.key).ok_or_else(|| {
            VegetationError::ResourceIdentity("V6 water authorization reason missing".into())
        })?;
        let valid_reason = match reason {
            WaterAuthorizationReason::ZeroDemand => {
                request.amount.to_bits() == 0.0_f64.to_bits()
                    && authorization.amount.to_bits() == 0.0_f64.to_bits()
            }
            WaterAuthorizationReason::FullySupplied => {
                request.amount > 0.0 && authorization.amount.to_bits() == request.amount.to_bits()
            }
            WaterAuthorizationReason::LiquidStorageLimit => {
                request.amount > 0.0
                    && authorization.amount < request.amount
                    && arbitration.snapshot.beginning_kg_m2_by_layer[&request.key.layer_id]
                        .to_bits()
                        == 0.0_f64.to_bits()
            }
            WaterAuthorizationReason::CompetingDemand => {
                request.amount > 0.0
                    && authorization.amount >= 0.0
                    && authorization.amount < request.amount
            }
            WaterAuthorizationReason::FrozenExclusion
            | WaterAuthorizationReason::RootingExclusion => {
                request.amount > 0.0 && authorization.amount.to_bits() == 0.0_f64.to_bits()
            }
        };
        if !valid_reason {
            return Err(VegetationError::ResourceIdentity(
                "V6 water authorization reason mismatch".into(),
            ));
        }
        *authorized_by_layer
            .entry(request.key.layer_id.clone())
            .or_default() += authorization.amount;
    }
    if authorized_by_layer.iter().any(|(layer, authorized)| {
        !authorized.is_finite()
            || *authorized > arbitration.snapshot.beginning_kg_m2_by_layer[layer]
    }) {
        return Err(VegetationError::ResourceBound(
            "V6 water snapshot overbooking".into(),
        ));
    }
    Ok(())
}

fn validate_water_owner_candidate(
    transaction_id: TransactionId,
    requests: &[WaterRequest],
    authorizations: &[WaterAuthorization],
    arbitration: &WaterArbitration,
    finalized_uses: &[WaterUse],
    candidate: &WaterOwnerCandidate,
) -> Result<(), VegetationError> {
    if requests.len() != authorizations.len()
        || requests.len() != finalized_uses.len()
        || candidate.transaction_id != transaction_id
        || candidate.finalized_uses != finalized_uses
    {
        return Err(VegetationError::ResourceIdentity(
            "V6 water-owner candidate protocol identity".into(),
        ));
    }
    let owner_id = requests
        .first()
        .map_or(&arbitration.snapshot.owner_id, |request| &request.owner_id);
    if &candidate.owner_id != owner_id {
        return Err(VegetationError::ResourceIdentity(
            "V6 water-owner candidate owner identity".into(),
        ));
    }

    let mut keys = BTreeSet::new();
    let mut maximum_authorizations = BTreeMap::<SoilLayerId, f64>::new();
    for ((request, authorization), finalized) in
        requests.iter().zip(authorizations).zip(finalized_uses)
    {
        validate_resource_protocol(request, authorization, finalized)
            .map_err(VegetationError::from)?;
        if !keys.insert(request.key.clone()) {
            return Err(VegetationError::ResourceIdentity(
                "duplicate V6 finalized water identity".into(),
            ));
        }
        *maximum_authorizations
            .entry(request.key.layer_id.clone())
            .or_default() += authorization.amount;
    }
    let expected_ending = reconstruct_water_ending(&arbitration.snapshot, finalized_uses)?;
    if candidate.snapshot != arbitration.snapshot
        || candidate
            .ending_kg_m2_by_layer
            .keys()
            .ne(expected_ending.keys())
    {
        return Err(VegetationError::ResourceIdentity(
            "V6 water-owner candidate layer identity".into(),
        ));
    }
    for (layer_id, ending) in expected_ending {
        let beginning = arbitration.snapshot.beginning_kg_m2_by_layer[&layer_id];
        if !beginning.is_finite()
            || beginning < 0.0
            || !ending.is_finite()
            || ending < 0.0
            || maximum_authorizations
                .get(&layer_id)
                .copied()
                .unwrap_or(0.0)
                > beginning
            || candidate.snapshot.beginning_kg_m2_by_layer[&layer_id].to_bits()
                != beginning.to_bits()
            || candidate.ending_kg_m2_by_layer[&layer_id].to_bits() != ending.to_bits()
        {
            return Err(VegetationError::ResourceBound(
                "V6 independently reconstructed water-owner debit".into(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use openwepp_kernel_contract::{
        MaximumAuthorization, ResourceAmountBasis, SoilLayerId, TileId, validate_resource_protocol,
    };

    use super::*;
    use crate::transaction::{SoilLayerForcing, v7_identity_rebound_fixture};

    struct FullWater {
        stores: BTreeMap<SoilLayerId, f64>,
        authorization_calls: Cell<u32>,
    }

    impl WaterArbiter for FullWater {
        fn authorize(
            &self,
            requests: &[WaterRequest],
        ) -> Result<WaterArbitration, VegetationError> {
            self.authorization_calls
                .set(self.authorization_calls.get() + 1);
            let authorizations = requests
                .iter()
                .map(|request| MaximumAuthorization {
                    transaction_id: request.transaction_id,
                    owner_id: request.owner_id.clone(),
                    key: request.key.clone(),
                    amount: request.amount,
                    basis: request.basis,
                })
                .collect::<Vec<_>>();
            let reasons: BTreeMap<_, _> = requests
                .iter()
                .map(|request| {
                    (
                        request.key.clone(),
                        if request.amount == 0.0 {
                            WaterAuthorizationReason::ZeroDemand
                        } else {
                            WaterAuthorizationReason::FullySupplied
                        },
                    )
                })
                .collect();
            let snapshot = WaterOwnerSnapshot::try_new(
                requests[0].transaction_id,
                requests[0].owner_id.clone(),
                self.stores.clone(),
                reasons.clone(),
            )?;
            WaterArbitration::try_new(snapshot, authorizations, reasons)
        }

        fn candidate_from_finalized_use(
            &self,
            transaction_id: TransactionId,
            arbitration: &WaterArbitration,
            finalized_uses: &[WaterUse],
        ) -> Result<WaterOwnerCandidate, VegetationError> {
            let ending = reconstruct_water_ending(arbitration.snapshot(), finalized_uses)?;
            WaterOwnerCandidate::try_new(
                transaction_id,
                arbitration.snapshot().owner_id().clone(),
                arbitration.snapshot().clone(),
                ending,
                finalized_uses.to_vec(),
            )
        }
    }

    fn forcing() -> SnowFreeForcing {
        SnowFreeForcing {
            air_temperature_k: 298.15,
            pressure_pa: 101_325.0,
            co2_pa: 42.0,
            vapor_pressure_deficit_kpa: 1.2,
            wind_m_s: 3.7,
            rain_kg_m2: 0.0,
            direct_par_w_m2: 410.0,
            diffuse_par_w_m2: 83.0,
            direct_nir_w_m2: 355.0,
            diffuse_nir_w_m2: 101.0,
            solar_zenith_cosine: 0.67,
            ground_albedo_vis: 0.14,
            ground_albedo_nir: 0.31,
            longwave_down_w_m2: 350.0,
            longwave_up_w_m2: 390.0,
            specific_humidity: 0.01,
            reference_height_m: 20.0,
            soil_layers: vec![SoilLayerForcing {
                layer_id: SoilLayerId::try_new("soil-1").expect("layer"),
                water_beginning_kg_m2: 20.0,
                matric_potential_mm: -1_000.0,
                hydraulic_conductivity_mm_s: 1.0e-5,
                root_path_length_mm: 100.0,
                gravity_root_mm: 500.0,
                temperature_k: 295.0,
                accessible: true,
                frozen: false,
            }],
            gsi: 1.0,
        }
    }

    fn water() -> FullWater {
        FullWater {
            stores: BTreeMap::from([(SoilLayerId::try_new("soil-1").expect("layer"), 20.0)]),
            authorization_calls: Cell::new(0),
        }
    }

    fn assert_capped_carbon_boundary(
        phase: &UncommittedWaterPhase,
        configuration: &VegetationConfiguration,
    ) {
        let final_carbon = phase
            .final_stratum_carbon_operands()
            .expect("final capped carbon operands");
        assert!(
            crate::carbon_phase::aggregate_stratum_carbon(
                phase.potential_columns(),
                configuration.dt_s,
            )
            .is_err()
        );
        let mut absent = phase.final_columns().clone();
        absent.columns[0].occupancy_results[0].carbon_operands = None;
        assert!(
            crate::carbon_phase::aggregate_stratum_carbon(&absent, configuration.dt_s).is_err()
        );
        let mut duplicate = phase.final_columns().clone();
        let duplicate_occupancy = duplicate.columns[0].occupancy_results[0].clone();
        duplicate.columns[0]
            .occupancy_results
            .push(duplicate_occupancy);
        assert!(
            crate::carbon_phase::aggregate_stratum_carbon(&duplicate, configuration.dt_s).is_err()
        );
        let mut wrong_identity = phase.final_columns().clone();
        wrong_identity.columns[0].occupancy_results[0]
            .occupancy_id
            .tile_id = TileId::try_new("wrong-tile").expect("tile");
        assert!(
            crate::carbon_phase::aggregate_stratum_carbon(&wrong_identity, configuration.dt_s)
                .is_err()
        );
        let mut invalid_class = phase.final_columns().clone();
        invalid_class.columns[0].occupancy_results[0]
            .carbon_operands
            .as_mut()
            .expect("production operands")
            .sun_gross_assimilation_umol_co2_m2_leaf_s = -1.0;
        assert!(
            crate::carbon_phase::aggregate_stratum_carbon(&invalid_class, configuration.dt_s)
                .is_err()
        );
        let mut inconsistent_t10 = phase.final_columns().clone();
        let mut second_column = inconsistent_t10.columns[0].clone();
        let second_tile = TileId::try_new("second-carbon-tile").expect("tile");
        second_column.tile_id = second_tile.clone();
        second_column.ledger.tile_id = second_tile.clone();
        second_column.occupancy_results[0].occupancy_id.tile_id = second_tile;
        second_column.occupancy_results[0]
            .carbon_operands
            .as_mut()
            .expect("production operands")
            .advanced_t10_k += 1.0;
        inconsistent_t10.columns.push(second_column);
        assert!(
            crate::carbon_phase::aggregate_stratum_carbon(&inconsistent_t10, configuration.dt_s)
                .is_err()
        );
        assert_eq!(final_carbon.len(), configuration.strata.len());
        assert!(final_carbon.values().all(|operands| {
            operands.gross_primary_production_kg_c_m2 > 0.0
                && operands.accepted_leaf_respiration_kg_c_m2 > 0.0
                && operands.advanced_t10_k > 0.0
        }));
    }

    #[test]
    fn public_water_phase_runs_exact_two_pass_protocol_without_commit() {
        let (configuration, beginning) = v7_identity_rebound_fixture();
        let beginning_bytes = serde_json::to_vec(&beginning).expect("beginning bytes");
        let owner = water();
        let phase = execute_uncommitted_water_phase(
            &crate::load_model_definition().expect("model"),
            &configuration,
            &beginning,
            &forcing(),
            &owner,
        )
        .expect("uncommitted water phase");

        assert_eq!(owner.authorization_calls.get(), 1);
        assert_eq!(phase.transaction_id(), TransactionId(1));
        assert_eq!(phase.beginning_state_sha256(), beginning.state_sha256);
        let (requests, authorizations, uses) = phase.protocol();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests.len(), authorizations.len());
        assert_eq!(requests.len(), uses.len());
        assert_eq!(phase.water_operands().len(), uses.len());
        assert_capped_carbon_boundary(&phase, &configuration);
        for ((request, authorization), finalized) in requests.iter().zip(authorizations).zip(uses) {
            validate_resource_protocol(request, authorization, finalized)
                .expect("typed D/A/F protocol");
            assert_eq!(
                request.basis,
                ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval
            );
            assert!(finalized.amount <= authorization.amount);
            assert!(authorization.amount <= request.amount);
        }
        let operand = &phase.water_operands()[0];
        assert_eq!(operand.beginning_state_sha256, beginning.state_sha256);
        assert_eq!(
            operand.beginning_occupancy_state,
            beginning.occupancies[&operand.key.occupancy_id]
        );
        assert_eq!(
            operand
                .candidate_occupancy_state
                .last_accepted_transaction_id,
            operand
                .beginning_occupancy_state
                .last_accepted_transaction_id
        );
        assert_eq!(
            serde_json::to_vec(&beginning).expect("after bytes"),
            beginning_bytes
        );
    }

    #[test]
    fn injected_public_water_failures_return_no_candidate_and_preserve_state() {
        let points = [
            FailurePoint::Validation,
            FailurePoint::Radiation,
            FailurePoint::Interception,
            FailurePoint::PotentialCoupledSolve,
            FailurePoint::WaterAuthorization,
            FailurePoint::CappedResolve,
            FailurePoint::OwnerValidation,
        ];
        for point in points {
            let (configuration, beginning) = v7_identity_rebound_fixture();
            let bytes = serde_json::to_vec(&beginning).expect("before bytes");
            let owner = water();
            let water_bytes = serde_json::to_vec(&owner.stores).expect("water bytes");
            let result = execute_uncommitted_water_phase_with_failure(
                &crate::load_model_definition().expect("model"),
                &configuration,
                &beginning,
                &forcing(),
                &owner,
                Some(point),
            );
            assert!(matches!(result, Err(VegetationError::InjectedFailure(_))));
            assert_eq!(serde_json::to_vec(&beginning).expect("after bytes"), bytes);
            assert_eq!(
                serde_json::to_vec(&owner.stores).expect("water after"),
                water_bytes
            );
        }
    }

    #[test]
    fn independently_rejects_owner_candidate_with_wrong_debit() {
        struct AuthorizationDebitOwner(FullWater);
        impl WaterArbiter for AuthorizationDebitOwner {
            fn authorize(
                &self,
                requests: &[WaterRequest],
            ) -> Result<WaterArbitration, VegetationError> {
                self.0.authorize(requests)
            }
            fn candidate_from_finalized_use(
                &self,
                transaction_id: TransactionId,
                arbitration: &WaterArbitration,
                finalized_uses: &[WaterUse],
            ) -> Result<WaterOwnerCandidate, VegetationError> {
                let layer = finalized_uses[0].key.layer_id.clone();
                let beginning = arbitration.snapshot().beginning_kg_m2_by_layer()[&layer];
                WaterOwnerCandidate::try_new(
                    transaction_id,
                    arbitration.snapshot().owner_id().clone(),
                    arbitration.snapshot().clone(),
                    BTreeMap::from([(
                        layer,
                        beginning - arbitration.authorizations()[0].amount - 1.0e-6,
                    )]),
                    finalized_uses.to_vec(),
                )
            }
        }

        let (configuration, beginning) = v7_identity_rebound_fixture();
        let mut limited = water();
        limited.stores.values_mut().for_each(|value| *value = 0.05);
        let owner = AuthorizationDebitOwner(limited);
        let bytes = serde_json::to_vec(&beginning).expect("before bytes");
        assert!(matches!(
            execute_uncommitted_water_phase(
                &crate::load_model_definition().expect("model"),
                &configuration,
                &beginning,
                &forcing(),
                &owner,
            ),
            Err(VegetationError::ResourceBound(_))
        ));
        assert_eq!(serde_json::to_vec(&beginning).expect("after bytes"), bytes);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn snapshot_reasons_shared_layer_and_authorization_as_debit_are_independent() {
        use openwepp_kernel_contract::{
            FinalizedUse, OccupancyId, ResourceOwnerId, ResourceRequest, StratumId, TileId,
            WaterResourceKey,
        };

        let transaction_id = TransactionId(9);
        let owner_id = ResourceOwnerId::try_new("vegetation").expect("owner");
        let layer_id = SoilLayerId::try_new("soil-shared").expect("layer");
        let key = |stratum: &str, tile: &str| WaterResourceKey {
            occupancy_id: OccupancyId {
                stratum_id: StratumId::try_new(stratum).expect("stratum"),
                tile_id: TileId::try_new(tile).expect("tile"),
            },
            layer_id: layer_id.clone(),
        };
        let keys = [key("upper", "a"), key("lower", "a")];
        let requests = keys
            .iter()
            .cloned()
            .zip([0.3, 0.4])
            .map(|(key, amount)| ResourceRequest {
                transaction_id,
                owner_id: owner_id.clone(),
                key,
                amount,
                basis: ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
            })
            .collect::<Vec<_>>();
        let authorizations = requests
            .iter()
            .zip([0.2, 0.3])
            .map(|(request, amount)| MaximumAuthorization {
                transaction_id,
                owner_id: owner_id.clone(),
                key: request.key.clone(),
                amount,
                basis: request.basis,
            })
            .collect::<Vec<_>>();
        let uses = requests
            .iter()
            .zip([0.01, 0.07])
            .map(|(request, amount)| FinalizedUse {
                transaction_id,
                owner_id: owner_id.clone(),
                key: request.key.clone(),
                amount,
                basis: request.basis,
            })
            .collect::<Vec<_>>();
        let reasons = keys
            .iter()
            .cloned()
            .map(|key| (key, WaterAuthorizationReason::CompetingDemand))
            .collect::<BTreeMap<_, _>>();
        let snapshot = WaterOwnerSnapshot::try_new(
            transaction_id,
            owner_id.clone(),
            BTreeMap::from([(layer_id.clone(), 1.0)]),
            reasons.clone(),
        )
        .expect("snapshot");
        let arbitration =
            WaterArbitration::try_new(snapshot.clone(), authorizations.clone(), reasons.clone())
                .expect("arbitration");
        validate_arbitration(transaction_id, &owner_id, &requests, &arbitration)
            .expect("reason and snapshot bound");
        let ending = reconstruct_water_ending(&snapshot, &uses).expect("canonical shared debit");
        assert_eq!(
            ending[&layer_id].to_bits(),
            (1.0_f64 - (0.01 + 0.07)).to_bits()
        );
        assert_ne!(
            ending[&layer_id].to_bits(),
            ((1.0_f64 - 0.01) - 0.07).to_bits()
        );
        let valid = WaterOwnerCandidate::try_new(
            transaction_id,
            owner_id.clone(),
            snapshot.clone(),
            ending,
            uses.clone(),
        )
        .expect("valid candidate");
        validate_water_owner_candidate(
            transaction_id,
            &requests,
            &authorizations,
            &arbitration,
            &uses,
            &valid,
        )
        .expect("shared layer candidate");

        let authorization_debit = WaterOwnerCandidate::try_new(
            transaction_id,
            owner_id.clone(),
            snapshot.clone(),
            BTreeMap::from([(layer_id.clone(), 1.0 - (0.2 + 0.3))]),
            uses.clone(),
        )
        .expect("structurally valid poison");
        assert!(
            validate_water_owner_candidate(
                transaction_id,
                &requests,
                &authorizations,
                &arbitration,
                &uses,
                &authorization_debit,
            )
            .is_err()
        );

        let stale_snapshot = WaterOwnerSnapshot::try_new(
            transaction_id,
            owner_id.clone(),
            BTreeMap::from([(layer_id, 2.0)]),
            reasons.clone(),
        )
        .expect("stale snapshot");
        let stale = WaterOwnerCandidate::try_new(
            transaction_id,
            stale_snapshot.owner_id().clone(),
            stale_snapshot.clone(),
            reconstruct_water_ending(&stale_snapshot, &uses).expect("stale ending"),
            uses.clone(),
        )
        .expect("stale candidate");
        assert!(
            validate_water_owner_candidate(
                transaction_id,
                &requests,
                &authorizations,
                &arbitration,
                &uses,
                &stale,
            )
            .is_err()
        );

        let mut wrong_reason = arbitration.clone();
        *wrong_reason.reasons.get_mut(&keys[0]).expect("reason") =
            WaterAuthorizationReason::FrozenExclusion;
        assert!(validate_arbitration(transaction_id, &owner_id, &requests, &wrong_reason).is_err());
        *wrong_reason.reasons.get_mut(&keys[0]).expect("reason") =
            WaterAuthorizationReason::RootingExclusion;
        assert!(validate_arbitration(transaction_id, &owner_id, &requests, &wrong_reason).is_err());
    }
}
