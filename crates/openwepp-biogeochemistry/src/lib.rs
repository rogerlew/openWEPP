#![allow(clippy::missing_errors_doc)]
//! Mineral-nitrogen arbitration and material receiving state for the C3 woody model.

use openwepp_kernel_contract::{
    FinalizedUse, MaximumAuthorization, ResourceRequest, validate_resource_protocol,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum BiogeochemistryError {
    #[error("BGC-E-001: invalid resource request: {0}")]
    InvalidRequest(String),
    #[error("BGC-E-002: finalized use exceeds mineral storage")]
    InsufficientMineralNitrogen,
    #[error("BGC-E-003: material transfer fails C/N/dry-material closure")]
    MaterialClosure,
    #[error("BGC-E-040: soil transformations are required but unsupported by model v1")]
    TransformationsRequired,
}
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MaterialPool {
    pub carbon: f64,
    pub nitrogen: f64,
    pub dry_matter: f64,
}
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MineralLayer {
    pub ammonium_n: f64,
    pub nitrate_n: f64,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BiogeochemistryState {
    pub layers: BTreeMap<String, MineralLayer>,
    pub receivers: BTreeMap<String, MaterialPool>,
    pub last_transaction_id: u128,
}
pub type NitrogenRequest = ResourceRequest<String, f64>;
pub type NitrogenAuthorization = MaximumAuthorization<String, f64>;
pub type NitrogenUse = FinalizedUse<String, f64>;

pub fn authorize_proportionally(
    requests: &[NitrogenRequest],
    available: &BTreeMap<String, f64>,
) -> Result<Vec<NitrogenAuthorization>, BiogeochemistryError> {
    let mut totals: BTreeMap<&str, f64> = BTreeMap::new();
    for r in requests {
        if !r.amount.is_finite() || r.amount < 0.0 {
            return Err(BiogeochemistryError::InvalidRequest(
                "nonfinite or negative amount".into(),
            ));
        }
        *totals.entry(r.key.as_str()).or_default() += r.amount;
    }
    requests
        .iter()
        .map(|r| {
            let total = totals.get(r.key.as_str()).copied().unwrap_or(0.0);
            let supply = available.get(&r.key).copied().unwrap_or(0.0);
            if !supply.is_finite() || supply < 0.0 {
                return Err(BiogeochemistryError::InvalidRequest(
                    "invalid available storage".into(),
                ));
            }
            let amount = if total <= supply {
                r.amount
            } else if total == 0.0 {
                0.0
            } else {
                supply * r.amount / total
            };
            Ok(MaximumAuthorization {
                transaction_id: r.transaction_id,
                owner_id: r.owner_id.clone(),
                key: r.key.clone(),
                amount,
            })
        })
        .collect()
}

pub fn apply_candidate(
    beginning: &BiogeochemistryState,
    requests: &[NitrogenRequest],
    authorizations: &[NitrogenAuthorization],
    uses: &[NitrogenUse],
    transfers: &[(String, MaterialPool)],
    carbon_fraction: f64,
    require_transformations: bool,
) -> Result<BiogeochemistryState, BiogeochemistryError> {
    if require_transformations {
        return Err(BiogeochemistryError::TransformationsRequired);
    }
    if requests.len() != authorizations.len() || requests.len() != uses.len() {
        return Err(BiogeochemistryError::InvalidRequest("receipt shape".into()));
    }
    let mut candidate = beginning.clone();
    for ((r, a), u) in requests.iter().zip(authorizations).zip(uses) {
        validate_resource_protocol(r, a, u)
            .map_err(|e| BiogeochemistryError::InvalidRequest(format!("{e:?}")))?;
        let layer = candidate
            .layers
            .get_mut(&r.key)
            .ok_or_else(|| BiogeochemistryError::InvalidRequest("unknown layer".into()))?;
        let available = layer.ammonium_n + layer.nitrate_n;
        if u.amount > available {
            return Err(BiogeochemistryError::InsufficientMineralNitrogen);
        }
        let nh4 = u.amount.min(layer.ammonium_n);
        layer.ammonium_n -= nh4;
        layer.nitrate_n -= u.amount - nh4;
        candidate.last_transaction_id = u.transaction_id.0;
    }
    if !carbon_fraction.is_finite() || carbon_fraction <= 0.0 || carbon_fraction > 1.0 {
        return Err(BiogeochemistryError::MaterialClosure);
    }
    for (receiver, t) in transfers {
        if [t.carbon, t.nitrogen, t.dry_matter]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
            || (t.dry_matter - t.carbon / carbon_fraction).abs() > 1e-12
        {
            return Err(BiogeochemistryError::MaterialClosure);
        }
        let pool = candidate.receivers.entry(receiver.clone()).or_default();
        pool.carbon += t.carbon;
        pool.nitrogen += t.nitrogen;
        pool.dry_matter += t.dry_matter;
    }
    Ok(candidate)
}

pub fn zero_transformation_flux(required: bool) -> Result<f64, BiogeochemistryError> {
    if required {
        Err(BiogeochemistryError::TransformationsRequired)
    } else {
        Ok(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn competition_is_proportional() {
        let tx = openwepp_kernel_contract::TransactionId(1);
        let req = vec![
            ResourceRequest {
                transaction_id: tx,
                owner_id: "a".into(),
                key: "l".into(),
                amount: 3.0,
            },
            ResourceRequest {
                transaction_id: tx,
                owner_id: "b".into(),
                key: "l".into(),
                amount: 1.0,
            },
        ];
        let auth = authorize_proportionally(&req, &BTreeMap::from([("l".into(), 2.0)]))
            .expect("authorization");
        assert!((auth[0].amount - 1.5).abs() < f64::EPSILON);
        assert!((auth[1].amount - 0.5).abs() < f64::EPSILON);
    }
}
