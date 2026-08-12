//! Default-off coupled-vegetation diagnostic consumer.
//!
//! This module is intentionally absent from every production runtime selector.
use std::collections::BTreeMap;

use openwepp_biogeochemistry::{
    BiogeochemistryError, BiogeochemistryState, NitrogenRequest as BgcRequest, apply_candidate,
    authorize_proportionally,
};
use openwepp_kernel_contract::{MaximumAuthorization, TransactionId};
use openwepp_vegetation::{
    CoupledOwnedState, ModelDefinition, NitrogenArbiter, NitrogenAuthorization, NitrogenRequest,
    SnowFreeForcing, VegetationConfiguration, VegetationError, WaterArbiter, WaterAuthorization,
    WaterRequest, execute_candidate, validate_and_commit,
};

#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticOwnedState {
    pub vegetation: CoupledOwnedState,
    pub biogeochemistry: BiogeochemistryState,
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
    available: &'a BTreeMap<String, f64>,
}
impl WaterArbiter for ProportionalWater<'_> {
    fn authorize(
        &self,
        requests: &[WaterRequest],
    ) -> Result<Vec<WaterAuthorization>, VegetationError> {
        proportional(requests, self.available)
    }
}
struct ProportionalNitrogen<'a> {
    available: &'a BTreeMap<String, f64>,
}
impl NitrogenArbiter for ProportionalNitrogen<'_> {
    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
        let bgc_requests = requests
            .iter()
            .cloned()
            .map(|request| BgcRequest {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id,
                key: request.key,
                amount: request.amount,
            })
            .collect::<Vec<_>>();
        authorize_proportionally(&bgc_requests, self.available)
            .map_err(|error| VegetationError::Receipt(error.to_string()))
    }
}

fn proportional(
    requests: &[WaterRequest],
    available: &BTreeMap<String, f64>,
) -> Result<Vec<WaterAuthorization>, VegetationError> {
    let mut totals: BTreeMap<&str, f64> = BTreeMap::new();
    for request in requests {
        *totals.entry(&request.key).or_default() += request.amount;
    }
    requests
        .iter()
        .map(|request| {
            let supply = available.get(&request.key).copied().unwrap_or(0.0);
            let total = totals.get(request.key.as_str()).copied().unwrap_or(0.0);
            if !supply.is_finite() || supply < 0.0 {
                return Err(VegetationError::Domain("water supply"));
            }
            Ok(MaximumAuthorization {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount: if total <= supply {
                    request.amount
                } else if total == 0.0 {
                    0.0
                } else {
                    supply * request.amount / total
                },
            })
        })
        .collect()
}

pub fn run_default_off_diagnostic(
    owned: &mut DiagnosticOwnedState,
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    forcing: &SnowFreeForcing,
    available_water: &BTreeMap<String, f64>,
    inject_failure: bool,
) -> Result<DiagnosticReceipt, DiagnosticError> {
    let water = ProportionalWater {
        available: available_water,
    };
    let available_n = owned
        .biogeochemistry
        .layers
        .iter()
        .map(|(id, layer)| (id.clone(), layer.ammonium_n + layer.nitrate_n))
        .collect::<BTreeMap<_, _>>();
    let nitrogen = ProportionalNitrogen {
        available: &available_n,
    };
    let vegetation_candidate =
        execute_candidate(model, config, &owned.vegetation, forcing, &water, &nitrogen)?;
    let bgc_candidate = apply_candidate(
        &owned.biogeochemistry,
        &vegetation_candidate.nitrogen_requests,
        &vegetation_candidate.nitrogen_authorizations,
        &vegetation_candidate.nitrogen_uses,
        &[],
        0.5,
        false,
    )?;
    if inject_failure {
        return Err(DiagnosticError::InjectedFailure);
    }
    let water_used = vegetation_candidate
        .water_uses
        .iter()
        .map(|value| value.amount)
        .sum();
    let nitrogen_used = vegetation_candidate
        .nitrogen_uses
        .iter()
        .map(|value| value.amount)
        .sum();
    let transaction_id = vegetation_candidate.diagnostics.transaction_id;
    let mut vegetation_commit = owned.vegetation.clone();
    validate_and_commit(&mut vegetation_commit, vegetation_candidate)?;
    owned.vegetation = vegetation_commit;
    owned.biogeochemistry = bgc_candidate;
    Ok(DiagnosticReceipt {
        transaction_id,
        water_used,
        nitrogen_used,
    })
}
