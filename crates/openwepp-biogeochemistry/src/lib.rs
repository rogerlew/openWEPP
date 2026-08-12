#![allow(clippy::missing_errors_doc)]
//! Mineral-nitrogen arbitration and material receiving state for the C3 woody model.

use openwepp_kernel_contract::{
    FinalizedUse, MaterialDonorClass, MaterialReceiverClass, MaximumAuthorization,
    MineralNitrogenKey, MineralNitrogenSpecies, ResourceAmountBasis, ResourceOwnerId,
    ResourceRequest, SoilLayerId, authorize_proportionally as authorize_resources_proportionally,
    validate_resource_protocol,
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
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialReceipt {
    pub transaction_id: u128,
    pub owner_id: ResourceOwnerId,
    pub donor: MaterialDonorClass,
    pub receiver: MaterialReceiverClass,
    pub proposal_id: u64,
    pub amounts: MaterialPool,
}
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MineralLayer {
    pub ammonium_n: f64,
    pub nitrate_n: f64,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BiogeochemistryState {
    pub layers: BTreeMap<String, MineralLayer>,
    pub receivers: BTreeMap<MaterialReceiverClass, MaterialPool>,
    pub last_transaction_id: u128,
}
pub type NitrogenRequest = ResourceRequest<MineralNitrogenKey, f64>;
pub type NitrogenAuthorization = MaximumAuthorization<MineralNitrogenKey, f64>;
pub type NitrogenUse = FinalizedUse<MineralNitrogenKey, f64>;

pub fn available_by_key(
    state: &BiogeochemistryState,
) -> Result<BTreeMap<MineralNitrogenKey, f64>, BiogeochemistryError> {
    let mut available = BTreeMap::new();
    for (layer_name, layer) in &state.layers {
        if !layer.ammonium_n.is_finite()
            || !layer.nitrate_n.is_finite()
            || layer.ammonium_n < 0.0
            || layer.nitrate_n < 0.0
        {
            return Err(BiogeochemistryError::InvalidRequest(
                "invalid mineral inventory".into(),
            ));
        }
        let layer_id = SoilLayerId::try_new(layer_name.clone())
            .map_err(|error| BiogeochemistryError::InvalidRequest(format!("{error:?}")))?;
        available.insert(
            MineralNitrogenKey {
                layer_id: layer_id.clone(),
                species: MineralNitrogenSpecies::Ammonium,
            },
            layer.ammonium_n,
        );
        available.insert(
            MineralNitrogenKey {
                layer_id,
                species: MineralNitrogenSpecies::Nitrate,
            },
            layer.nitrate_n,
        );
    }
    Ok(available)
}

pub fn authorize_proportionally(
    requests: &[NitrogenRequest],
    available: &BTreeMap<MineralNitrogenKey, f64>,
) -> Result<Vec<NitrogenAuthorization>, BiogeochemistryError> {
    authorize_resources_proportionally(
        requests,
        available,
        ResourceAmountBasis::NitrogenKgPerSquareMeterInterval,
    )
    .map_err(|error| BiogeochemistryError::InvalidRequest(format!("{error:?}")))
}

#[allow(clippy::too_many_arguments)]
pub fn apply_candidate(
    beginning: &BiogeochemistryState,
    transaction_id: u128,
    requests: &[NitrogenRequest],
    authorizations: &[NitrogenAuthorization],
    uses: &[NitrogenUse],
    proposals: &[MaterialReceipt],
    receipts: &[MaterialReceipt],
    require_transformations: bool,
) -> Result<BiogeochemistryState, BiogeochemistryError> {
    if require_transformations {
        return Err(BiogeochemistryError::TransformationsRequired);
    }
    if requests.len() != authorizations.len() || requests.len() != uses.len() {
        return Err(BiogeochemistryError::InvalidRequest("receipt shape".into()));
    }
    let mut candidate = beginning.clone();
    if transaction_id <= beginning.last_transaction_id {
        return Err(BiogeochemistryError::InvalidRequest(
            "stale transaction identity".into(),
        ));
    }
    for ((r, a), u) in requests.iter().zip(authorizations).zip(uses) {
        validate_resource_protocol(r, a, u)
            .map_err(|e| BiogeochemistryError::InvalidRequest(format!("{e:?}")))?;
        if r.transaction_id.0 != transaction_id {
            return Err(BiogeochemistryError::InvalidRequest(
                "nitrogen transaction identity".into(),
            ));
        }
        let layer = candidate
            .layers
            .get_mut(r.key.layer_id.as_str())
            .ok_or_else(|| BiogeochemistryError::InvalidRequest("unknown layer".into()))?;
        let available = match r.key.species {
            MineralNitrogenSpecies::Ammonium => layer.ammonium_n,
            MineralNitrogenSpecies::Nitrate => layer.nitrate_n,
        };
        if u.amount > available {
            return Err(BiogeochemistryError::InsufficientMineralNitrogen);
        }
        match r.key.species {
            MineralNitrogenSpecies::Ammonium => layer.ammonium_n -= u.amount,
            MineralNitrogenSpecies::Nitrate => layer.nitrate_n -= u.amount,
        }
    }
    if proposals.len() != receipts.len() {
        return Err(BiogeochemistryError::MaterialClosure);
    }
    let mut receipt_keys = std::collections::BTreeSet::new();
    for (proposal, receipt) in proposals.iter().zip(receipts) {
        let t = receipt.amounts;
        if proposal != receipt
            || receipt.transaction_id != transaction_id
            || !receipt_keys.insert((
                receipt.transaction_id,
                receipt.owner_id.clone(),
                receipt.proposal_id,
            ))
        {
            return Err(BiogeochemistryError::MaterialClosure);
        }
        if [t.carbon, t.nitrogen, t.dry_matter]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
            || (t.carbon > 0.0 && t.dry_matter < t.carbon)
        {
            return Err(BiogeochemistryError::MaterialClosure);
        }
        let pool = candidate
            .receivers
            .get_mut(&receipt.receiver)
            .ok_or(BiogeochemistryError::MaterialClosure)?;
        pool.carbon += t.carbon;
        pool.nitrogen += t.nitrogen;
        pool.dry_matter += t.dry_matter;
    }
    candidate.last_transaction_id = transaction_id;
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
    use openwepp_kernel_contract::ResourceOwnerId;
    #[test]
    fn competition_is_proportional() {
        let tx = openwepp_kernel_contract::TransactionId(1);
        let owner_a = ResourceOwnerId::try_new("a").expect("owner");
        let owner_b = ResourceOwnerId::try_new("b").expect("owner");
        let key = MineralNitrogenKey {
            layer_id: SoilLayerId::try_new("l").expect("layer"),
            species: MineralNitrogenSpecies::Ammonium,
        };
        let req = vec![
            ResourceRequest {
                transaction_id: tx,
                owner_id: owner_a,
                key: key.clone(),
                amount: 3.0,
                basis: ResourceAmountBasis::NitrogenKgPerSquareMeterInterval,
            },
            ResourceRequest {
                transaction_id: tx,
                owner_id: owner_b,
                key: key.clone(),
                amount: 1.0,
                basis: ResourceAmountBasis::NitrogenKgPerSquareMeterInterval,
            },
        ];
        let auth =
            authorize_proportionally(&req, &BTreeMap::from([(key, 2.0)])).expect("authorization");
        assert!((auth[0].amount - 1.5).abs() < f64::EPSILON);
        assert!((auth[1].amount - 0.5).abs() < f64::EPSILON);
        let mut reversed = req.clone();
        reversed.reverse();
        let reversed_auth =
            authorize_proportionally(&reversed, &BTreeMap::from([(req[0].key.clone(), 2.0)]))
                .expect("order-independent authorization");
        let by_owner = auth
            .iter()
            .chain(&reversed_auth)
            .map(|value| (value.owner_id.as_str(), value.amount))
            .collect::<Vec<_>>();
        assert_eq!(
            by_owner,
            vec![("a", 1.5), ("b", 0.5), ("b", 0.5), ("a", 1.5)]
        );
    }

    #[test]
    fn species_and_layers_are_independent_and_unused_authorization_is_not_debited() {
        let tx = openwepp_kernel_contract::TransactionId(7);
        let owner = ResourceOwnerId::try_new("tree").expect("owner");
        let l1 = SoilLayerId::try_new("l1").expect("layer");
        let l2 = SoilLayerId::try_new("l2").expect("layer");
        let keys = [
            MineralNitrogenKey {
                layer_id: l1.clone(),
                species: MineralNitrogenSpecies::Ammonium,
            },
            MineralNitrogenKey {
                layer_id: l1,
                species: MineralNitrogenSpecies::Nitrate,
            },
            MineralNitrogenKey {
                layer_id: l2,
                species: MineralNitrogenSpecies::Ammonium,
            },
        ];
        let requests = keys
            .iter()
            .cloned()
            .map(|key| ResourceRequest {
                transaction_id: tx,
                owner_id: owner.clone(),
                key,
                amount: 2.0,
                basis: ResourceAmountBasis::NitrogenKgPerSquareMeterInterval,
            })
            .collect::<Vec<_>>();
        let supply = BTreeMap::from([
            (keys[0].clone(), 1.0),
            (keys[1].clone(), 4.0),
            (keys[2].clone(), 0.5),
        ]);
        let authorizations = authorize_proportionally(&requests, &supply).expect("authorization");
        assert_eq!(
            authorizations
                .iter()
                .map(|value| value.amount)
                .collect::<Vec<_>>(),
            vec![1.0, 2.0, 0.5]
        );
        let uses = authorizations
            .iter()
            .map(|authorization| FinalizedUse {
                transaction_id: tx,
                owner_id: owner.clone(),
                key: authorization.key.clone(),
                amount: authorization.amount / 2.0,
                basis: authorization.basis,
            })
            .collect::<Vec<_>>();
        let beginning = BiogeochemistryState {
            layers: BTreeMap::from([
                (
                    "l1".into(),
                    MineralLayer {
                        ammonium_n: 1.0,
                        nitrate_n: 4.0,
                    },
                ),
                (
                    "l2".into(),
                    MineralLayer {
                        ammonium_n: 0.5,
                        nitrate_n: 9.0,
                    },
                ),
            ]),
            ..BiogeochemistryState::default()
        };
        let ending = apply_candidate(
            &beginning,
            tx.0,
            &requests,
            &authorizations,
            &uses,
            &[],
            &[],
            false,
        )
        .expect("candidate");
        assert!((ending.layers["l1"].ammonium_n - 0.5).abs() < f64::EPSILON);
        assert!((ending.layers["l1"].nitrate_n - 3.0).abs() < f64::EPSILON);
        assert!((ending.layers["l2"].ammonium_n - 0.25).abs() < f64::EPSILON);
        assert!((ending.layers["l2"].nitrate_n - 9.0).abs() < f64::EPSILON);
    }

    #[test]
    fn duplicate_material_receipt_is_rejected() {
        let receipt = MaterialReceipt {
            transaction_id: 1,
            owner_id: ResourceOwnerId::try_new("tree").expect("owner"),
            donor: MaterialDonorClass::Leaf,
            receiver: MaterialReceiverClass::Metabolic,
            proposal_id: 0,
            amounts: MaterialPool {
                carbon: 0.0048,
                nitrogen: 0.0001,
                dry_matter: 0.01,
            },
        };
        assert_eq!(
            apply_candidate(
                &BiogeochemistryState::default(),
                1,
                &[],
                &[],
                &[],
                &[receipt.clone(), receipt.clone()],
                &[receipt.clone(), receipt],
                false,
            ),
            Err(BiogeochemistryError::MaterialClosure)
        );
    }
}
