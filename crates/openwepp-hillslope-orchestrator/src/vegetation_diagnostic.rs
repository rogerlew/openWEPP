//! Default-off coupled-vegetation diagnostic consumer.
//!
//! This module is intentionally absent from every production runtime selector.
use std::collections::BTreeMap;

use openwepp_biogeochemistry::{
    BiogeochemistryError, BiogeochemistryState, MaterialPool, MaterialReceipt, apply_candidate,
    authorize_proportionally, available_by_key,
};
use openwepp_kernel_contract::{
    MaximumAuthorization, MineralNitrogenKey, ResourceAmountBasis, SoilLayerId, TransactionId,
    WaterResourceKey, validate_request_batch,
};
use openwepp_vegetation::energy::LATENT_HEAT_VAPORIZATION;
use openwepp_vegetation::{
    CoupledOwnedState, FailurePoint, ModelDefinition, NitrogenArbiter, NitrogenAuthorization,
    NitrogenRequest, SnowFreeForcing, VegetationConfiguration, VegetationError, WaterArbiter,
    WaterAuthorization, WaterRequest, execute_candidate_with_failure,
    validate_and_commit_with_failure,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DiagnosticOwnedState {
    pub vegetation: CoupledOwnedState,
    pub water: DiagnosticWaterState,
    pub biogeochemistry: BiogeochemistryState,
    pub energy: DiagnosticEnergyState,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DiagnosticWaterState {
    pub liquid_kg_m2: BTreeMap<SoilLayerId, f64>,
    pub last_transaction_id: u128,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DiagnosticEnergyState {
    pub last_transaction_id: u128,
    pub last_operands: Option<openwepp_vegetation::ledger::EnergyLedgerOperands>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticReceipt {
    pub transaction_id: TransactionId,
    pub water_used: f64,
    pub nitrogen_used: f64,
}

#[derive(Debug)]
pub enum DiagnosticError {
    Vegetation(VegetationError),
    Biogeochemistry(BiogeochemistryError),
    InjectedFailure,
}

impl From<VegetationError> for DiagnosticError {
    fn from(value: VegetationError) -> Self {
        Self::Vegetation(value)
    }
}
impl From<BiogeochemistryError> for DiagnosticError {
    fn from(value: BiogeochemistryError) -> Self {
        Self::Biogeochemistry(value)
    }
}

struct ProportionalWater<'a> {
    available: &'a BTreeMap<SoilLayerId, f64>,
}
impl WaterArbiter for ProportionalWater<'_> {
    fn beginning_amount(&self, key: &WaterResourceKey) -> Result<f64, VegetationError> {
        self.available
            .get(&key.layer_id)
            .copied()
            .ok_or(VegetationError::Domain("unknown water layer"))
    }
    fn authorize(
        &self,
        requests: &[WaterRequest],
    ) -> Result<Vec<WaterAuthorization>, VegetationError> {
        validate_request_batch(requests)
            .map_err(|error| VegetationError::Receipt(format!("{error:?}")))?;
        if requests.iter().any(|request| {
            request.basis != ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval
                || !request.amount.is_finite()
                || request.amount < 0.0
        }) {
            return Err(VegetationError::Receipt(
                "invalid stand-ground water request".into(),
            ));
        }
        let mut demand = BTreeMap::<SoilLayerId, f64>::new();
        for request in requests {
            *demand.entry(request.key.layer_id.clone()).or_default() += request.amount;
        }
        requests
            .iter()
            .map(|request| {
                let supply = self
                    .available
                    .get(&request.key.layer_id)
                    .copied()
                    .ok_or(VegetationError::Domain("unknown water layer"))?;
                let total = demand[&request.key.layer_id];
                let fraction = if total == 0.0 {
                    0.0
                } else {
                    (supply / total).min(1.0)
                };
                Ok(MaximumAuthorization {
                    transaction_id: request.transaction_id,
                    owner_id: request.owner_id.clone(),
                    key: request.key.clone(),
                    amount: request.amount * fraction,
                    basis: request.basis,
                })
            })
            .collect()
    }
}
struct ProportionalNitrogen<'a> {
    available: &'a BTreeMap<MineralNitrogenKey, f64>,
}
impl NitrogenArbiter for ProportionalNitrogen<'_> {
    fn beginning_amount(&self, key: &MineralNitrogenKey) -> Result<f64, VegetationError> {
        self.available
            .get(key)
            .copied()
            .ok_or(VegetationError::Domain("unknown nitrogen inventory"))
    }
    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
        authorize_proportionally(requests, self.available)
            .map_err(|error| VegetationError::Receipt(error.to_string()))
    }
}

pub fn run_default_off_diagnostic(
    owned: &mut DiagnosticOwnedState,
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    forcing: &SnowFreeForcing,
    available_water: &BTreeMap<SoilLayerId, f64>,
    inject_failure: bool,
) -> Result<DiagnosticReceipt, DiagnosticError> {
    run_default_off_diagnostic_at_phase(
        owned,
        model,
        config,
        forcing,
        available_water,
        inject_failure.then_some(FailurePoint::BeforeCommit),
    )
}

#[allow(clippy::too_many_lines)]
pub fn run_default_off_diagnostic_at_phase(
    owned: &mut DiagnosticOwnedState,
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    forcing: &SnowFreeForcing,
    available_water: &BTreeMap<SoilLayerId, f64>,
    failure: Option<FailurePoint>,
) -> Result<DiagnosticReceipt, DiagnosticError> {
    if available_water != &owned.water.liquid_kg_m2 {
        return Err(DiagnosticError::Vegetation(VegetationError::Receipt(
            "diagnostic water snapshot mismatch".into(),
        )));
    }
    let water = ProportionalWater {
        available: available_water,
    };
    let available_n = available_by_key(&owned.biogeochemistry)?;
    let nitrogen = ProportionalNitrogen {
        available: &available_n,
    };
    let vegetation_candidate = execute_candidate_with_failure(
        model,
        config,
        &owned.vegetation,
        forcing,
        &water,
        &nitrogen,
        failure,
    )?;
    let transfers = vegetation_candidate
        .material_transfers()
        .iter()
        .map(|transfer| {
            Ok(MaterialReceipt {
                transaction_id: transfer.transaction_id,
                owner_id: transfer.owner_id.clone(),
                donor: transfer.donor,
                receiver: transfer.receiver,
                proposal_id: transfer.proposal_id,
                amounts: MaterialPool {
                    carbon: transfer.carbon,
                    nitrogen: transfer.nitrogen,
                    dry_matter: transfer.dry_matter,
                },
            })
        })
        .collect::<Result<Vec<_>, VegetationError>>()?;
    let (nitrogen_requests, nitrogen_authorizations, nitrogen_uses) =
        vegetation_candidate.nitrogen_protocol();
    let bgc_candidate = apply_candidate(
        &owned.biogeochemistry,
        vegetation_candidate.transaction_id().0,
        nitrogen_requests,
        nitrogen_authorizations,
        nitrogen_uses,
        &transfers,
        &transfers,
        false,
    )?;
    let mut water_candidate = owned.water.clone();
    for finalized in vegetation_candidate.water_uses() {
        let store = water_candidate
            .liquid_kg_m2
            .get_mut(&finalized.key.layer_id)
            .ok_or(VegetationError::Domain("unknown diagnostic water layer"))?;
        if finalized.amount > *store {
            return Err(VegetationError::Receipt("diagnostic water overdraft".into()).into());
        }
        *store -= finalized.amount;
    }
    water_candidate.last_transaction_id = vegetation_candidate.transaction_id().0;
    let water_used = vegetation_candidate
        .water_uses()
        .iter()
        .map(|value| value.amount)
        .sum();
    let nitrogen_total_used = vegetation_candidate
        .nitrogen_protocol()
        .2
        .iter()
        .map(|value| value.amount)
        .sum();
    let transaction_id = vegetation_candidate.transaction_id();
    let components = vegetation_candidate.energy_owner_operands();
    let identity = vegetation_candidate
        .ledger_operands()
        .energy
        .identity
        .clone();
    let reconstructed_energy = openwepp_vegetation::ledger::EnergyLedgerOperands {
        identity,
        incident_shortwave_j_m2: components.incident_shortwave_w_m2 * components.interval_s,
        incident_longwave_j_m2: components.incident_longwave_j_m2,
        reflected_shortwave_j_m2: components.reflected_shortwave_w_m2 * components.interval_s,
        terminal_shortwave_j_m2: components.terminal_shortwave_w_m2 * components.interval_s,
        emitted_longwave_j_m2: components.emitted_longwave_j_m2,
        sensible_j_m2: components.sensible_j_m2,
        latent_j_m2: LATENT_HEAT_VAPORIZATION
            * (components.transpiration_kg_m2 + components.wet_phase_change_kg_m2),
        ground_or_storage_j_m2: 0.0,
    };
    let energy_candidate = DiagnosticEnergyState {
        last_transaction_id: transaction_id.0,
        last_operands: Some(reconstructed_energy),
    };
    validate_owner_candidates(
        &vegetation_candidate,
        &water_candidate,
        &bgc_candidate,
        &energy_candidate,
    )?;
    let mut vegetation_commit = owned.vegetation.clone();
    validate_and_commit_with_failure(&mut vegetation_commit, vegetation_candidate, failure)?;
    owned.vegetation = vegetation_commit;
    owned.water = water_candidate;
    owned.biogeochemistry = bgc_candidate;
    owned.energy = energy_candidate;
    Ok(DiagnosticReceipt {
        transaction_id,
        water_used,
        nitrogen_used: nitrogen_total_used,
    })
}

fn validate_owner_candidates(
    vegetation: &openwepp_vegetation::CoupledCandidate,
    water: &DiagnosticWaterState,
    bgc: &BiogeochemistryState,
    energy: &DiagnosticEnergyState,
) -> Result<(), DiagnosticError> {
    let ledgers = vegetation.ledger_operands();
    for store in &ledgers.water.soil {
        if water.liquid_kg_m2.get(&store.layer_id).copied() != Some(store.ending_kg_m2) {
            return Err(VegetationError::Receipt("water owner/ledger mismatch".into()).into());
        }
    }
    for store in &ledgers.nitrogen.mineral {
        let layer = bgc
            .layers
            .get(store.key.layer_id.as_str())
            .ok_or(VegetationError::Receipt(
                "BGC owner/ledger layer mismatch".into(),
            ))?;
        let ending = match store.key.species {
            openwepp_kernel_contract::MineralNitrogenSpecies::Ammonium => layer.ammonium_n,
            openwepp_kernel_contract::MineralNitrogenSpecies::Nitrate => layer.nitrate_n,
        };
        if ending.to_bits() != store.ending_kg_m2.to_bits() {
            return Err(VegetationError::Receipt("BGC owner/ledger mismatch".into()).into());
        }
    }
    if energy.last_transaction_id != vegetation.transaction_id().0
        || energy.last_operands.as_ref() != Some(&ledgers.energy)
    {
        return Err(VegetationError::Receipt("energy owner/ledger mismatch".into()).into());
    }
    Ok(())
}
