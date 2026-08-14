#![allow(clippy::missing_errors_doc)]
//! Mineral-nitrogen arbitration and material receiving state for the C3 woody model.

use openwepp_kernel_contract::{
    FinalizedUse, MaterialDonorClass, MaterialReceiverClass, MaximumAuthorization,
    MineralNitrogenKey, MineralNitrogenSpecies, ResourceAmountBasis, ResourceOwnerId,
    ResourceRequest, SoilLayerId, TransactionId,
    authorize_proportionally as authorize_resources_proportionally, validate_request_batch,
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
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialProposal {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransformationsMode {
    Disabled,
    Required,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MineralInventoryOperand {
    pub key: MineralNitrogenKey,
    pub beginning_kg_n_m2: f64,
    pub finalized_use_kg_n_m2: f64,
    pub ending_kg_n_m2: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialReceiverOperand {
    pub receiver: MaterialReceiverClass,
    pub beginning: MaterialPool,
    pub incoming: MaterialPool,
    pub ending: MaterialPool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BiogeochemistryOwnerCandidate {
    transaction_id: TransactionId,
    beginning: BiogeochemistryState,
    ending: BiogeochemistryState,
    finalized_uses: Vec<NitrogenUse>,
    receipts: Vec<MaterialReceipt>,
    mineral_operands: Vec<MineralInventoryOperand>,
    receiver_operands: Vec<MaterialReceiverOperand>,
}

impl BiogeochemistryOwnerCandidate {
    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub fn beginning(&self) -> &BiogeochemistryState {
        &self.beginning
    }

    #[must_use]
    pub fn ending(&self) -> &BiogeochemistryState {
        &self.ending
    }

    #[must_use]
    pub fn finalized_uses(&self) -> &[NitrogenUse] {
        &self.finalized_uses
    }

    #[must_use]
    pub fn receipts(&self) -> &[MaterialReceipt] {
        &self.receipts
    }

    #[must_use]
    pub fn mineral_operands(&self) -> &[MineralInventoryOperand] {
        &self.mineral_operands
    }

    #[must_use]
    pub fn receiver_operands(&self) -> &[MaterialReceiverOperand] {
        &self.receiver_operands
    }

    pub fn validate(&self) -> Result<(), BiogeochemistryError> {
        if self.transaction_id.0 == 0
            || self.ending.last_transaction_id != self.transaction_id.0
            || self.beginning.last_transaction_id >= self.transaction_id.0
        {
            return Err(BiogeochemistryError::InvalidRequest(
                "candidate transaction identity".into(),
            ));
        }
        for operand in &self.mineral_operands {
            if [
                operand.beginning_kg_n_m2,
                operand.finalized_use_kg_n_m2,
                operand.ending_kg_n_m2,
            ]
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
                || (operand.beginning_kg_n_m2 - operand.finalized_use_kg_n_m2).to_bits()
                    != operand.ending_kg_n_m2.to_bits()
            {
                return Err(BiogeochemistryError::InsufficientMineralNitrogen);
            }
        }
        for operand in &self.receiver_operands {
            validate_material_pool(operand.beginning)?;
            validate_material_pool(operand.incoming)?;
            validate_material_pool(operand.ending)?;
            if add_pool(operand.beginning, operand.incoming) != operand.ending {
                return Err(BiogeochemistryError::MaterialClosure);
            }
        }
        Ok(())
    }
}

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

#[allow(clippy::too_many_lines)]
pub fn construct_biogeochemistry_candidate(
    beginning: &BiogeochemistryState,
    transaction_id: TransactionId,
    requests: &[NitrogenRequest],
    authorizations: &[NitrogenAuthorization],
    uses: &[NitrogenUse],
    proposals: &[MaterialProposal],
    transformations: TransformationsMode,
) -> Result<BiogeochemistryOwnerCandidate, BiogeochemistryError> {
    if transformations == TransformationsMode::Required {
        return Err(BiogeochemistryError::TransformationsRequired);
    }
    if requests.len() != authorizations.len() || requests.len() != uses.len() {
        return Err(BiogeochemistryError::InvalidRequest("receipt shape".into()));
    }
    validate_request_batch(requests)
        .map_err(|error| BiogeochemistryError::InvalidRequest(format!("{error:?}")))?;
    let mut candidate = beginning.clone();
    if transaction_id.0 <= beginning.last_transaction_id {
        return Err(BiogeochemistryError::InvalidRequest(
            "stale transaction identity".into(),
        ));
    }
    let available = available_by_key(beginning)?;
    let mut use_by_key = BTreeMap::<MineralNitrogenKey, f64>::new();
    for ((r, a), u) in requests.iter().zip(authorizations).zip(uses) {
        validate_resource_protocol(r, a, u)
            .map_err(|e| BiogeochemistryError::InvalidRequest(format!("{e:?}")))?;
        if r.transaction_id != transaction_id {
            return Err(BiogeochemistryError::InvalidRequest(
                "nitrogen transaction identity".into(),
            ));
        }
        *use_by_key.entry(r.key.clone()).or_default() += u.amount;
    }
    let mut mineral_operands = Vec::with_capacity(available.len());
    for (key, beginning_amount) in available {
        let finalized = use_by_key.remove(&key).unwrap_or(0.0);
        if !finalized.is_finite() || finalized < 0.0 || finalized > beginning_amount {
            return Err(BiogeochemistryError::InsufficientMineralNitrogen);
        }
        let ending_amount = beginning_amount - finalized;
        let layer = candidate
            .layers
            .get_mut(key.layer_id.as_str())
            .ok_or_else(|| BiogeochemistryError::InvalidRequest("unknown layer".into()))?;
        match key.species {
            MineralNitrogenSpecies::Ammonium => layer.ammonium_n = ending_amount,
            MineralNitrogenSpecies::Nitrate => layer.nitrate_n = ending_amount,
        }
        mineral_operands.push(MineralInventoryOperand {
            key,
            beginning_kg_n_m2: beginning_amount,
            finalized_use_kg_n_m2: finalized,
            ending_kg_n_m2: ending_amount,
        });
    }
    if !use_by_key.is_empty() {
        return Err(BiogeochemistryError::InvalidRequest(
            "unknown finalized-use inventory".into(),
        ));
    }
    let receiver_beginning = beginning.receivers.clone();
    let mut receipts = Vec::with_capacity(proposals.len());
    let mut proposal_keys = std::collections::BTreeSet::new();
    for proposal in proposals {
        if proposal.transaction_id != transaction_id.0
            || proposal.proposal_id == 0
            || !proposal_keys.insert((
                proposal.transaction_id,
                proposal.owner_id.clone(),
                proposal.proposal_id,
            ))
        {
            return Err(BiogeochemistryError::MaterialClosure);
        }
        validate_material_pool(proposal.amounts)?;
        let pool = candidate
            .receivers
            .get_mut(&proposal.receiver)
            .ok_or(BiogeochemistryError::MaterialClosure)?;
        *pool = add_pool(*pool, proposal.amounts);
        receipts.push(MaterialReceipt {
            transaction_id: proposal.transaction_id,
            owner_id: proposal.owner_id.clone(),
            donor: proposal.donor,
            receiver: proposal.receiver,
            proposal_id: proposal.proposal_id,
            amounts: proposal.amounts,
        });
    }
    candidate.last_transaction_id = transaction_id.0;
    let receiver_operands = receiver_beginning
        .into_iter()
        .map(|(receiver, beginning_pool)| {
            let incoming = receipts
                .iter()
                .filter(|receipt| receipt.receiver == receiver)
                .fold(MaterialPool::default(), |total, receipt| {
                    add_pool(total, receipt.amounts)
                });
            let ending = candidate.receivers[&receiver];
            MaterialReceiverOperand {
                receiver,
                beginning: beginning_pool,
                incoming,
                ending,
            }
        })
        .collect();
    let owner_candidate = BiogeochemistryOwnerCandidate {
        transaction_id,
        beginning: beginning.clone(),
        ending: candidate,
        finalized_uses: uses.to_vec(),
        receipts,
        mineral_operands,
        receiver_operands,
    };
    owner_candidate.validate()?;
    Ok(owner_candidate)
}

fn validate_material_pool(pool: MaterialPool) -> Result<(), BiogeochemistryError> {
    if [pool.carbon, pool.nitrogen, pool.dry_matter]
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
        || (pool.carbon > 0.0 && pool.dry_matter < pool.carbon)
    {
        return Err(BiogeochemistryError::MaterialClosure);
    }
    Ok(())
}

fn add_pool(left: MaterialPool, right: MaterialPool) -> MaterialPool {
    MaterialPool {
        carbon: left.carbon + right.carbon,
        nitrogen: left.nitrogen + right.nitrogen,
        dry_matter: left.dry_matter + right.dry_matter,
    }
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
        let candidate = construct_biogeochemistry_candidate(
            &beginning,
            tx,
            &requests,
            &authorizations,
            &uses,
            &[],
            TransformationsMode::Disabled,
        )
        .expect("candidate");
        let ending = candidate.ending();
        assert!((ending.layers["l1"].ammonium_n - 0.5).abs() < f64::EPSILON);
        assert!((ending.layers["l1"].nitrate_n - 3.0).abs() < f64::EPSILON);
        assert!((ending.layers["l2"].ammonium_n - 0.25).abs() < f64::EPSILON);
        assert!((ending.layers["l2"].nitrate_n - 9.0).abs() < f64::EPSILON);
    }

    #[test]
    fn duplicate_material_receipt_is_rejected() {
        let proposal = MaterialProposal {
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
            construct_biogeochemistry_candidate(
                &BiogeochemistryState::default(),
                TransactionId(1),
                &[],
                &[],
                &[],
                &[proposal.clone(), proposal],
                TransformationsMode::Disabled,
            ),
            Err(BiogeochemistryError::MaterialClosure)
        );
    }

    #[test]
    fn owner_constructs_exact_material_receipt_and_independent_receiver_operands() {
        let beginning = BiogeochemistryState {
            receivers: BTreeMap::from([(
                MaterialReceiverClass::Metabolic,
                MaterialPool {
                    carbon: 1.0,
                    nitrogen: 2.0,
                    dry_matter: 3.0,
                },
            )]),
            ..BiogeochemistryState::default()
        };
        let proposal = MaterialProposal {
            transaction_id: 1,
            owner_id: ResourceOwnerId::try_new("stratum:canopy").expect("owner"),
            donor: MaterialDonorClass::Leaf,
            receiver: MaterialReceiverClass::Metabolic,
            proposal_id: 7,
            amounts: MaterialPool {
                carbon: 0.4,
                nitrogen: 0.03,
                dry_matter: 0.9,
            },
        };
        let candidate = construct_biogeochemistry_candidate(
            &beginning,
            TransactionId(1),
            &[],
            &[],
            &[],
            std::slice::from_ref(&proposal),
            TransformationsMode::Disabled,
        )
        .expect("receiving owner candidate");
        assert_eq!(
            beginning.receivers[&MaterialReceiverClass::Metabolic]
                .carbon
                .to_bits(),
            1.0_f64.to_bits()
        );
        assert_eq!(candidate.receipts().len(), 1);
        let receipt = &candidate.receipts()[0];
        assert_eq!(receipt.transaction_id, proposal.transaction_id);
        assert_eq!(receipt.owner_id, proposal.owner_id);
        assert_eq!(receipt.donor, proposal.donor);
        assert_eq!(receipt.receiver, proposal.receiver);
        assert_eq!(receipt.proposal_id, proposal.proposal_id);
        assert_eq!(receipt.amounts, proposal.amounts);
        assert_eq!(candidate.receiver_operands().len(), 1);
        assert_eq!(candidate.receiver_operands()[0].incoming, proposal.amounts);
        assert_eq!(
            candidate.ending().receivers[&MaterialReceiverClass::Metabolic],
            MaterialPool {
                carbon: 1.4,
                nitrogen: 2.03,
                dry_matter: 3.9,
            }
        );
        candidate.validate().expect("independent owner ledgers");
    }

    #[test]
    fn transformations_and_wrong_species_finalized_use_fail_closed() {
        assert_eq!(
            construct_biogeochemistry_candidate(
                &BiogeochemistryState::default(),
                TransactionId(1),
                &[],
                &[],
                &[],
                &[],
                TransformationsMode::Required,
            ),
            Err(BiogeochemistryError::TransformationsRequired)
        );

        let tx = TransactionId(1);
        let owner = ResourceOwnerId::try_new("stratum:canopy").expect("owner");
        let ammonium = MineralNitrogenKey {
            layer_id: SoilLayerId::try_new("l1").expect("layer"),
            species: MineralNitrogenSpecies::Ammonium,
        };
        let nitrate = MineralNitrogenKey {
            layer_id: ammonium.layer_id.clone(),
            species: MineralNitrogenSpecies::Nitrate,
        };
        let request = ResourceRequest {
            transaction_id: tx,
            owner_id: owner.clone(),
            key: ammonium.clone(),
            amount: 0.5,
            basis: ResourceAmountBasis::NitrogenKgPerSquareMeterInterval,
        };
        let authorization = MaximumAuthorization {
            transaction_id: tx,
            owner_id: owner.clone(),
            key: ammonium,
            amount: 0.5,
            basis: request.basis,
        };
        let wrong_use = FinalizedUse {
            transaction_id: tx,
            owner_id: owner,
            key: nitrate,
            amount: 0.25,
            basis: request.basis,
        };
        let beginning = BiogeochemistryState {
            layers: BTreeMap::from([(
                "l1".into(),
                MineralLayer {
                    ammonium_n: 1.0,
                    nitrate_n: 1.0,
                },
            )]),
            ..BiogeochemistryState::default()
        };
        assert!(matches!(
            construct_biogeochemistry_candidate(
                &beginning,
                tx,
                &[request],
                &[authorization],
                &[wrong_use],
                &[],
                TransformationsMode::Disabled,
            ),
            Err(BiogeochemistryError::InvalidRequest(_))
        ));
        assert_eq!(
            beginning.layers["l1"].ammonium_n.to_bits(),
            1.0_f64.to_bits()
        );
        assert_eq!(
            beginning.layers["l1"].nitrate_n.to_bits(),
            1.0_f64.to_bits()
        );
    }
}
