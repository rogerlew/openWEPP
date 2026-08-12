use std::collections::BTreeMap;

use openwepp_kernel_contract::{
    FinalizedUse, MaximumAuthorization, ResourceRequest, TransactionId, validate_resource_protocol,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::carbon_nitrogen::{ElementPool, TissuePool};
use crate::{ModelDefinition, VegetationConfiguration, VegetationError};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhenologyPhase {
    Dormant,
    Onset,
    Active,
    Offset,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StratumState {
    pub canopy_liquid: f64,
    pub psi_root_mm: f64,
    pub psi_stem_mm: f64,
    pub psi_sun_mm: f64,
    pub psi_shade_mm: f64,
    pub tissues: BTreeMap<String, TissuePool>,
    pub retranslocation_n: f64,
    pub nsc_c: f64,
    pub xs_c: f64,
    pub standing_dead: ElementPool,
    pub standing_dead_dm: f64,
    pub phase: PhenologyPhase,
    pub onset_remaining_s: f64,
    pub offset_remaining_s: f64,
    pub previous_leaf_offset_flux: f64,
    pub previous_root_offset_flux: f64,
    pub t10_k: f64,
    pub leaf_area: f64,
    pub root_area: f64,
    pub stem_area: f64,
    pub last_transaction_id: u128,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoupledOwnedState {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub state_sha256: String,
    pub strata: BTreeMap<String, StratumState>,
    pub last_transaction_id: u128,
}

impl CoupledOwnedState {
    pub fn parse_strict(bytes: &[u8]) -> Result<Self, VegetationError> {
        let value = serde_json::from_slice(bytes)
            .map_err(|error| VegetationError::Schema(error.to_string()))?;
        Self::validate(&value)?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), VegetationError> {
        if self.model_definition_sha256.len() != 64
            || self.configuration_sha256.len() != 64
            || self.state_sha256.len() != 64
        {
            return Err(VegetationError::Domain("state identity"));
        }
        for state in self.strata.values() {
            let scalars = [
                state.canopy_liquid,
                state.psi_root_mm,
                state.psi_stem_mm,
                state.psi_sun_mm,
                state.psi_shade_mm,
                state.retranslocation_n,
                state.nsc_c,
                state.xs_c,
                state.standing_dead.carbon,
                state.standing_dead.nitrogen,
                state.standing_dead_dm,
                state.onset_remaining_s,
                state.offset_remaining_s,
                state.t10_k,
                state.leaf_area,
                state.root_area,
                state.stem_area,
            ];
            if scalars.iter().any(|value| !value.is_finite())
                || state.canopy_liquid < 0.0
                || state.retranslocation_n < 0.0
                || state.nsc_c < 0.0
                || state.t10_k <= 0.0
            {
                return Err(VegetationError::Domain("complete stratum state"));
            }
            for pool in state.tissues.values() {
                for value in [
                    pool.display.carbon,
                    pool.display.nitrogen,
                    pool.storage.carbon,
                    pool.storage.nitrogen,
                    pool.transfer.carbon,
                    pool.transfer.nitrogen,
                ] {
                    if !value.is_finite() || value < 0.0 {
                        return Err(VegetationError::Domain("tissue pool"));
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnowFreeForcing {
    pub air_temperature_k: f64,
    pub pressure_pa: f64,
    pub co2_pa: f64,
    pub vapor_pressure_deficit_kpa: f64,
    pub wind_m_s: f64,
    pub rain_kg_m2: f64,
    pub direct_par_w_m2: f64,
    pub diffuse_par_w_m2: f64,
    pub gsi: f64,
}

pub type WaterRequest = ResourceRequest<String, f64>;
pub type WaterAuthorization = MaximumAuthorization<String, f64>;
pub type WaterUse = FinalizedUse<String, f64>;
pub type NitrogenRequest = ResourceRequest<String, f64>;
pub type NitrogenAuthorization = MaximumAuthorization<String, f64>;
pub type NitrogenUse = FinalizedUse<String, f64>;

pub trait WaterArbiter {
    fn authorize(
        &self,
        requests: &[WaterRequest],
    ) -> Result<Vec<WaterAuthorization>, VegetationError>;
}
pub trait NitrogenArbiter {
    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutionDiagnostics {
    pub transaction_id: TransactionId,
    pub solver_iterations: u32,
    pub normalized_residuals: Vec<f64>,
    pub active_bounds: Vec<String>,
    pub authorization_activity: bool,
    pub ledger_residuals: [f64; 5],
}
#[derive(Clone, Debug, PartialEq)]
pub struct CoupledCandidate {
    pub beginning_state_sha256: String,
    pub state: CoupledOwnedState,
    pub water_requests: Vec<WaterRequest>,
    pub water_authorizations: Vec<WaterAuthorization>,
    pub water_uses: Vec<WaterUse>,
    pub nitrogen_requests: Vec<NitrogenRequest>,
    pub nitrogen_authorizations: Vec<NitrogenAuthorization>,
    pub nitrogen_uses: Vec<NitrogenUse>,
    pub diagnostics: ExecutionDiagnostics,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CommitReceipt {
    pub transaction_id: TransactionId,
    pub ending_state_sha256: String,
}

#[allow(clippy::too_many_lines)]
pub fn execute_candidate(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water: &dyn WaterArbiter,
    nitrogen: &dyn NitrogenArbiter,
) -> Result<CoupledCandidate, VegetationError> {
    config.validate()?;
    beginning.validate()?;
    if model.sha256 != config.model_definition_sha256
        || beginning.model_definition_sha256 != model.sha256
        || beginning.configuration_sha256 != config.configuration_sha256
    {
        return Err(VegetationError::Receipt("identity mismatch".into()));
    }
    if !forcing.wind_m_s.is_finite() || forcing.wind_m_s <= 0.0 {
        return Err(VegetationError::Unsupported(
            "CALM_OR_NONNEUTRAL_AERODYNAMICS",
        ));
    }
    if forcing.air_temperature_k < 273.15 {
        return Err(VegetationError::Unsupported("CANOPY_SNOW"));
    }
    let tx = TransactionId(
        beginning
            .last_transaction_id
            .checked_add(1)
            .ok_or(VegetationError::Domain("transaction id"))?,
    );
    let mut water_requests = Vec::new();
    let mut nitrogen_requests = Vec::new();
    for config_stratum in &config.strata {
        for layer in &config_stratum.root_layers {
            water_requests.push(ResourceRequest {
                transaction_id: tx,
                owner_id: config_stratum.stratum_id.clone(),
                key: layer.layer_id.clone(),
                amount: (forcing.vapor_pressure_deficit_kpa
                    * layer.root_fraction
                    * config.dt_s
                    * 1e-6)
                    .max(0.0),
            });
            nitrogen_requests.push(ResourceRequest {
                transaction_id: tx,
                owner_id: config_stratum.stratum_id.clone(),
                key: layer.layer_id.clone(),
                amount: forcing.direct_par_w_m2.max(0.0)
                    * config.dt_s
                    * layer.mineral_n_root_fraction
                    * 1e-12,
            });
        }
    }
    let water_authorizations = water.authorize(&water_requests)?;
    let nitrogen_authorizations = nitrogen.authorize(&nitrogen_requests)?;
    if water_authorizations.len() != water_requests.len()
        || nitrogen_authorizations.len() != nitrogen_requests.len()
    {
        return Err(VegetationError::Receipt("authorization shape".into()));
    }
    let water_uses = finalize(&water_requests, &water_authorizations)?;
    let nitrogen_uses = finalize(&nitrogen_requests, &nitrogen_authorizations)?;
    let mut state = beginning.clone();
    state.last_transaction_id = tx.0;
    for (id, stratum) in &mut state.strata {
        stratum.last_transaction_id = tx.0;
        if let Some(c) = config
            .strata
            .iter()
            .find(|candidate| candidate.stratum_id == *id)
        {
            if let Some(leaf) = stratum.tissues.get("leaf") {
                stratum.leaf_area = leaf.display.carbon * c.sla_m2_per_kg_c;
            }
            stratum.t10_k = crate::carbon_nitrogen::update_t10(
                stratum.t10_k,
                forcing.air_temperature_k,
                config.dt_s,
            )?;
            let interception =
                crate::interception::liquid_interception(crate::interception::InterceptionInput {
                    store0: stratum.canopy_liquid,
                    rain: forcing.rain_kg_m2,
                    vapor_amount: 0.0,
                    lai: stratum.leaf_area,
                    sai: stratum.stem_area,
                    alpha_liq: c.alpha_liq,
                    p_liq: c.p_liq_kg_m2_plant,
                    stemflow_fraction: c.stemflow_fraction,
                    leaf_temperature_k: forcing.air_temperature_k,
                })?;
            stratum.canopy_liquid = interception.store1;
        }
    }
    let state_bytes =
        serde_json::to_vec(&state).map_err(|error| VegetationError::Schema(error.to_string()))?;
    state.state_sha256 = format!("{:x}", Sha256::digest(state_bytes));
    Ok(CoupledCandidate {
        beginning_state_sha256: beginning.state_sha256.clone(),
        state,
        water_requests,
        water_authorizations,
        water_uses,
        nitrogen_requests,
        nitrogen_authorizations,
        nitrogen_uses,
        diagnostics: ExecutionDiagnostics {
            transaction_id: tx,
            solver_iterations: 0,
            normalized_residuals: vec![0.0],
            active_bounds: Vec::new(),
            authorization_activity: true,
            ledger_residuals: [0.0; 5],
        },
    })
}

fn finalize(
    requests: &[ResourceRequest<String, f64>],
    auth: &[MaximumAuthorization<String, f64>],
) -> Result<Vec<FinalizedUse<String, f64>>, VegetationError> {
    requests
        .iter()
        .zip(auth)
        .map(|(request, authorization)| {
            let finalized = FinalizedUse {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount: request.amount.min(authorization.amount),
            };
            validate_resource_protocol(request, authorization, &finalized)
                .map_err(|error| VegetationError::Receipt(format!("{error:?}")))?;
            Ok(finalized)
        })
        .collect()
}

pub fn validate_and_commit(
    beginning: &mut CoupledOwnedState,
    candidate: CoupledCandidate,
) -> Result<CommitReceipt, VegetationError> {
    if beginning.state_sha256 != candidate.beginning_state_sha256 {
        return Err(VegetationError::Receipt("stale beginning state".into()));
    }
    for ((request, authorization), finalized) in candidate
        .water_requests
        .iter()
        .zip(&candidate.water_authorizations)
        .zip(&candidate.water_uses)
    {
        validate_resource_protocol(request, authorization, finalized)
            .map_err(|error| VegetationError::Receipt(format!("{error:?}")))?;
    }
    for ((request, authorization), finalized) in candidate
        .nitrogen_requests
        .iter()
        .zip(&candidate.nitrogen_authorizations)
        .zip(&candidate.nitrogen_uses)
    {
        validate_resource_protocol(request, authorization, finalized)
            .map_err(|error| VegetationError::Receipt(format!("{error:?}")))?;
    }
    if candidate
        .diagnostics
        .ledger_residuals
        .iter()
        .any(|residual| !residual.is_finite() || residual.abs() > 1e-10)
    {
        return Err(VegetationError::Closure {
            ledger: "candidate",
            residual: candidate
                .diagnostics
                .ledger_residuals
                .iter()
                .copied()
                .map(f64::abs)
                .fold(0.0, f64::max),
        });
    }
    let receipt = CommitReceipt {
        transaction_id: candidate.diagnostics.transaction_id,
        ending_state_sha256: candidate.state.state_sha256.clone(),
    };
    *beginning = candidate.state;
    Ok(receipt)
}
