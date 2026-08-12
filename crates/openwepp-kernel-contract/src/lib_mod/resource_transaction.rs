//! Dependency-neutral resource arbitration transaction types.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct ResourceOwnerId(String);

impl ResourceOwnerId {
    pub fn try_new(value: impl Into<String>) -> Result<Self, ResourceIdentityError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ResourceIdentityError::EmptyOwner);
        }
        Ok(Self(value))
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<'de> Deserialize<'de> for ResourceOwnerId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct SoilLayerId(String);

impl SoilLayerId {
    pub fn try_new(value: impl Into<String>) -> Result<Self, ResourceIdentityError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ResourceIdentityError::EmptyLayer);
        }
        Ok(Self(value))
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<'de> Deserialize<'de> for SoilLayerId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub enum MineralNitrogenSpecies {
    Ammonium,
    Nitrate,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct WaterResourceKey {
    pub layer_id: SoilLayerId,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct MineralNitrogenKey {
    pub layer_id: SoilLayerId,
    pub species: MineralNitrogenSpecies,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ResourceAmountBasis {
    WaterKgPerSquareMeterInterval,
    NitrogenKgPerSquareMeterInterval,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialDonorClass {
    Leaf,
    FineRoot,
    LiveStem,
    DeadStem,
    LiveCoarseRoot,
    DeadCoarseRoot,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialReceiverClass {
    Metabolic,
    Cellulose,
    Lignin,
    CoarseWoodyDebris,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceIdentityError {
    EmptyOwner,
    EmptyLayer,
}
impl fmt::Display for ResourceIdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOwner => formatter.write_str("resource owner identity is empty"),
            Self::EmptyLayer => formatter.write_str("soil layer identity is empty"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ResourceRequest<K, Q> {
    pub transaction_id: TransactionId,
    pub owner_id: ResourceOwnerId,
    pub key: K,
    pub amount: Q,
    pub basis: ResourceAmountBasis,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MaximumAuthorization<K, Q> {
    pub transaction_id: TransactionId,
    pub owner_id: ResourceOwnerId,
    pub key: K,
    pub amount: Q,
    pub basis: ResourceAmountBasis,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FinalizedUse<K, Q> {
    pub transaction_id: TransactionId,
    pub owner_id: ResourceOwnerId,
    pub key: K,
    pub amount: Q,
    pub basis: ResourceAmountBasis,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct TransactionId(pub u128);

impl fmt::Display for TransactionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:032x}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceProtocolViolation {
    TransactionMismatch,
    OwnerMismatch,
    KeyMismatch,
    BasisMismatch,
    NonFinite,
    Negative,
    AuthorizationExceedsRequest,
    FinalizedUseExceedsAuthorization,
}

pub fn authorize_proportionally<K: Clone + Ord>(
    requests: &[ResourceRequest<K, f64>],
    available: &BTreeMap<K, f64>,
    expected_basis: ResourceAmountBasis,
) -> Result<Vec<MaximumAuthorization<K, f64>>, ResourceProtocolViolation> {
    let mut grouped = BTreeMap::<&K, Vec<&ResourceRequest<K, f64>>>::new();
    for request in requests {
        if !request.amount.is_finite() {
            return Err(ResourceProtocolViolation::NonFinite);
        }
        if request.amount < 0.0 {
            return Err(ResourceProtocolViolation::Negative);
        }
        if request.basis != expected_basis {
            return Err(ResourceProtocolViolation::BasisMismatch);
        }
        grouped.entry(&request.key).or_default().push(request);
    }
    let mut totals = BTreeMap::new();
    for (key, values) in &mut grouped {
        values.sort_by(|left, right| {
            left.owner_id
                .cmp(&right.owner_id)
                .then(left.transaction_id.cmp(&right.transaction_id))
        });
        let mut sum = 0.0;
        let mut compensation = 0.0;
        for request in values.iter() {
            let adjusted = request.amount - compensation;
            let next = sum + adjusted;
            compensation = (next - sum) - adjusted;
            sum = next;
        }
        totals.insert(*key, sum);
    }
    requests
        .iter()
        .map(|request| {
            let supply = available.get(&request.key).copied().unwrap_or(0.0);
            if !supply.is_finite() {
                return Err(ResourceProtocolViolation::NonFinite);
            }
            if supply < 0.0 {
                return Err(ResourceProtocolViolation::Negative);
            }
            let total = totals.get(&request.key).copied().unwrap_or(0.0);
            let amount = if total <= supply {
                request.amount
            } else if total == 0.0 {
                0.0
            } else {
                supply * (request.amount / total)
            };
            Ok(MaximumAuthorization {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount,
                basis: request.basis,
            })
        })
        .collect()
}

pub fn validate_resource_protocol<K: PartialEq>(
    request: &ResourceRequest<K, f64>,
    authorization: &MaximumAuthorization<K, f64>,
    finalized: &FinalizedUse<K, f64>,
) -> Result<(), ResourceProtocolViolation> {
    if request.transaction_id != authorization.transaction_id
        || request.transaction_id != finalized.transaction_id
    {
        return Err(ResourceProtocolViolation::TransactionMismatch);
    }
    if request.owner_id != authorization.owner_id || request.owner_id != finalized.owner_id {
        return Err(ResourceProtocolViolation::OwnerMismatch);
    }
    if request.key != authorization.key || request.key != finalized.key {
        return Err(ResourceProtocolViolation::KeyMismatch);
    }
    if request.basis != authorization.basis || request.basis != finalized.basis {
        return Err(ResourceProtocolViolation::BasisMismatch);
    }
    if !request.amount.is_finite()
        || !authorization.amount.is_finite()
        || !finalized.amount.is_finite()
    {
        return Err(ResourceProtocolViolation::NonFinite);
    }
    if request.amount < 0.0 || authorization.amount < 0.0 || finalized.amount < 0.0 {
        return Err(ResourceProtocolViolation::Negative);
    }
    if authorization.amount > request.amount {
        return Err(ResourceProtocolViolation::AuthorizationExceedsRequest);
    }
    if finalized.amount > authorization.amount {
        return Err(ResourceProtocolViolation::FinalizedUseExceedsAuthorization);
    }
    Ok(())
}

pub fn validate_maximum_authorization<K: PartialEq>(
    request: &ResourceRequest<K, f64>,
    authorization: &MaximumAuthorization<K, f64>,
) -> Result<(), ResourceProtocolViolation> {
    if request.transaction_id != authorization.transaction_id {
        return Err(ResourceProtocolViolation::TransactionMismatch);
    }
    if request.owner_id != authorization.owner_id {
        return Err(ResourceProtocolViolation::OwnerMismatch);
    }
    if request.key != authorization.key {
        return Err(ResourceProtocolViolation::KeyMismatch);
    }
    if request.basis != authorization.basis {
        return Err(ResourceProtocolViolation::BasisMismatch);
    }
    if !request.amount.is_finite() || !authorization.amount.is_finite() {
        return Err(ResourceProtocolViolation::NonFinite);
    }
    if request.amount < 0.0 || authorization.amount < 0.0 {
        return Err(ResourceProtocolViolation::Negative);
    }
    if authorization.amount > request.amount {
        return Err(ResourceProtocolViolation::AuthorizationExceedsRequest);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_enforces_ordered_amounts() {
        let request = ResourceRequest {
            transaction_id: TransactionId(1),
            owner_id: ResourceOwnerId::try_new("tree").expect("owner"),
            key: 2_u8,
            amount: 3.0,
            basis: ResourceAmountBasis::WaterKgPerSquareMeterInterval,
        };
        let authorization = MaximumAuthorization {
            transaction_id: TransactionId(1),
            owner_id: ResourceOwnerId::try_new("tree").expect("owner"),
            key: 2_u8,
            amount: 2.0,
            basis: ResourceAmountBasis::WaterKgPerSquareMeterInterval,
        };
        let finalized = FinalizedUse {
            transaction_id: TransactionId(1),
            owner_id: ResourceOwnerId::try_new("tree").expect("owner"),
            key: 2_u8,
            amount: 1.0,
            basis: ResourceAmountBasis::WaterKgPerSquareMeterInterval,
        };
        assert_eq!(
            validate_resource_protocol(&request, &authorization, &finalized),
            Ok(())
        );
    }

    #[test]
    fn typed_id_deserialization_rejects_empty_identity() {
        assert!(serde_json::from_str::<ResourceOwnerId>(r#"""#).is_err());
        assert!(serde_json::from_str::<SoilLayerId>(r#""   ""#).is_err());
    }
}
