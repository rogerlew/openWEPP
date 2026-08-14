//! Dependency-neutral resource arbitration transaction types.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct StratumId(String);

impl StratumId {
    pub fn try_new(value: impl Into<String>) -> Result<Self, ResourceIdentityError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ResourceIdentityError::EmptyStratum);
        }
        Ok(Self(value))
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<'de> Deserialize<'de> for StratumId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct TileId(String);

impl TileId {
    pub fn try_new(value: impl Into<String>) -> Result<Self, ResourceIdentityError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ResourceIdentityError::EmptyTile);
        }
        Ok(Self(value))
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<'de> Deserialize<'de> for TileId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct OccupancyId {
    pub stratum_id: StratumId,
    pub tile_id: TileId,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub enum MineralNitrogenSpecies {
    Ammonium,
    Nitrate,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct WaterResourceKey {
    pub occupancy_id: OccupancyId,
    pub layer_id: SoilLayerId,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct MineralNitrogenKey {
    pub layer_id: SoilLayerId,
    pub species: MineralNitrogenSpecies,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub enum ResourceAmountBasis {
    WaterKgPerSquareMeterInterval,
    WaterKgPerSquareMeterStandGroundInterval,
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
    EmptyStratum,
    EmptyTile,
}
impl fmt::Display for ResourceIdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOwner => formatter.write_str("resource owner identity is empty"),
            Self::EmptyLayer => formatter.write_str("soil layer identity is empty"),
            Self::EmptyStratum => formatter.write_str("stratum identity is empty"),
            Self::EmptyTile => formatter.write_str("tile identity is empty"),
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
    DuplicateRequestIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceProtocolCategory {
    Identity,
    Operand,
    Bound,
}

impl ResourceProtocolViolation {
    #[must_use]
    pub const fn category(self) -> ResourceProtocolCategory {
        match self {
            Self::BasisMismatch | Self::NonFinite | Self::Negative => {
                ResourceProtocolCategory::Operand
            }
            Self::AuthorizationExceedsRequest | Self::FinalizedUseExceedsAuthorization => {
                ResourceProtocolCategory::Bound
            }
            Self::TransactionMismatch
            | Self::OwnerMismatch
            | Self::KeyMismatch
            | Self::DuplicateRequestIdentity => ResourceProtocolCategory::Identity,
        }
    }
}

pub fn validate_unique_request_identities<K: Ord, Q>(
    requests: &[ResourceRequest<K, Q>],
) -> Result<(), ResourceProtocolViolation> {
    let mut identities = BTreeSet::new();
    for request in requests {
        let identity = (
            request.transaction_id,
            &request.owner_id,
            &request.key,
            request.basis,
        );
        if !identities.insert(identity) {
            return Err(ResourceProtocolViolation::DuplicateRequestIdentity);
        }
    }
    Ok(())
}

pub fn validate_request_batch<K: Ord, Q>(
    requests: &[ResourceRequest<K, Q>],
) -> Result<(), ResourceProtocolViolation> {
    if let Some(first) = requests.first() {
        if requests
            .iter()
            .any(|request| request.transaction_id != first.transaction_id)
        {
            return Err(ResourceProtocolViolation::TransactionMismatch);
        }
    }
    validate_unique_request_identities(requests)
}

pub fn authorize_proportionally<K: Clone + Ord>(
    requests: &[ResourceRequest<K, f64>],
    available: &BTreeMap<K, f64>,
    expected_basis: ResourceAmountBasis,
) -> Result<Vec<MaximumAuthorization<K, f64>>, ResourceProtocolViolation> {
    authorize_proportionally_by(requests, available, expected_basis, Clone::clone)
}

/// Proportionally authorize requests grouped by an owner-supply identity while
/// preserving each request's complete resource key in the returned record.
pub fn authorize_proportionally_by<K, S, F>(
    requests: &[ResourceRequest<K, f64>],
    available: &BTreeMap<S, f64>,
    expected_basis: ResourceAmountBasis,
    supply_key: F,
) -> Result<Vec<MaximumAuthorization<K, f64>>, ResourceProtocolViolation>
where
    K: Clone + Ord,
    S: Clone + Ord,
    F: Fn(&K) -> S,
{
    validate_request_batch(requests)?;
    let mut grouped = BTreeMap::<S, Vec<&ResourceRequest<K, f64>>>::new();
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
        grouped
            .entry(supply_key(&request.key))
            .or_default()
            .push(request);
    }
    let mut totals = BTreeMap::new();
    for (key, values) in &mut grouped {
        values.sort_by(|left, right| {
            left.owner_id
                .cmp(&right.owner_id)
                .then(left.transaction_id.cmp(&right.transaction_id))
                .then(left.key.cmp(&right.key))
                .then(left.basis.cmp(&right.basis))
        });
        let mut sum = 0.0;
        let mut compensation = 0.0;
        for request in values.iter() {
            let adjusted = request.amount - compensation;
            let next = sum + adjusted;
            compensation = (next - sum) - adjusted;
            sum = next;
        }
        totals.insert(key.clone(), sum);
    }
    requests
        .iter()
        .map(|request| {
            let key = supply_key(&request.key);
            let supply = available.get(&key).copied().unwrap_or(0.0);
            if !supply.is_finite() {
                return Err(ResourceProtocolViolation::NonFinite);
            }
            if supply < 0.0 {
                return Err(ResourceProtocolViolation::Negative);
            }
            let total = totals.get(&key).copied().unwrap_or(0.0);
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

    fn water_key(stratum: &str, tile: &str, layer: &str) -> WaterResourceKey {
        WaterResourceKey {
            occupancy_id: OccupancyId {
                stratum_id: StratumId::try_new(stratum).expect("stratum"),
                tile_id: TileId::try_new(tile).expect("tile"),
            },
            layer_id: SoilLayerId::try_new(layer).expect("layer"),
        }
    }

    fn water_request(key: WaterResourceKey) -> ResourceRequest<WaterResourceKey, f64> {
        ResourceRequest {
            transaction_id: TransactionId(1),
            owner_id: ResourceOwnerId::try_new("tree").expect("owner"),
            key,
            amount: 3.0,
            basis: ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
        }
    }

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
        assert!(serde_json::from_str::<StratumId>(r#"""#).is_err());
        assert!(serde_json::from_str::<TileId>(r#""   ""#).is_err());
        assert_eq!(
            StratumId::try_new("  "),
            Err(ResourceIdentityError::EmptyStratum)
        );
        assert_eq!(TileId::try_new(""), Err(ResourceIdentityError::EmptyTile));
    }

    #[test]
    fn proportional_authorization_preserves_full_water_resource_identity() {
        let key = water_key("canopy", "north", "soil-1");
        let request = water_request(key.clone());
        let authorization = authorize_proportionally(
            core::slice::from_ref(&request),
            &BTreeMap::from([(key.clone(), 2.0)]),
            ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
        )
        .expect("authorization");

        assert_eq!(authorization[0].key, key);
        assert_eq!(authorization[0].transaction_id, request.transaction_id);
        assert_eq!(authorization[0].owner_id, request.owner_id);
        assert_eq!(authorization[0].basis, request.basis);

        let finalized = FinalizedUse {
            transaction_id: authorization[0].transaction_id,
            owner_id: authorization[0].owner_id.clone(),
            key: authorization[0].key.clone(),
            amount: 1.0,
            basis: authorization[0].basis,
        };
        assert_eq!(
            validate_resource_protocol(&request, &authorization[0], &finalized),
            Ok(())
        );
        assert_eq!(finalized.key, key);
    }

    #[test]
    fn projected_supply_identity_competes_without_erasing_request_identity() {
        let mut upper = water_request(water_key("upper", "tile", "soil-1"));
        upper.amount = 2.0;
        let mut lower = water_request(water_key("lower", "tile", "soil-1"));
        lower.amount = 6.0;
        let requests = [upper.clone(), lower.clone()];
        let available = BTreeMap::from([(SoilLayerId::try_new("soil-1").unwrap(), 4.0)]);
        let authorizations = authorize_proportionally_by(
            &requests,
            &available,
            ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
            |key| key.layer_id.clone(),
        )
        .expect("same-layer authorization");

        assert_eq!(authorizations[0].key, upper.key);
        assert_eq!(authorizations[1].key, lower.key);
        assert_eq!(authorizations[0].amount.to_bits(), 1.0_f64.to_bits());
        assert_eq!(authorizations[1].amount.to_bits(), 3.0_f64.to_bits());
    }

    #[test]
    fn projected_supply_compensation_is_canonical_under_request_reversal() {
        let amounts = [
            2.413_412_142_132_508e-9,
            9_750.505_570_619_86,
            10_575.027_912_829_331,
            1.168_047_125_227_263_8e-7,
        ];
        let requests = ["a", "b", "c", "d"]
            .into_iter()
            .zip(amounts)
            .map(|(stratum, amount)| ResourceRequest {
                amount,
                ..water_request(water_key(stratum, "tile", "soil-1"))
            })
            .collect::<Vec<_>>();
        let available = BTreeMap::from([(SoilLayerId::try_new("soil-1").unwrap(), 10_000.0)]);
        let authorize = |batch: &[ResourceRequest<WaterResourceKey, f64>]| {
            authorize_proportionally_by(
                batch,
                &available,
                ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
                |key| key.layer_id.clone(),
            )
            .unwrap()
            .into_iter()
            .map(|authorization| (authorization.key, authorization.amount.to_bits()))
            .collect::<BTreeMap<_, _>>()
        };
        let mut reversed = requests.clone();
        reversed.reverse();
        assert_eq!(authorize(&requests), authorize(&reversed));
    }

    #[test]
    fn authorization_rejects_wrong_water_resource_identity_component() {
        let request = water_request(water_key("canopy", "north", "soil-1"));

        for wrong_key in [
            water_key("understory", "north", "soil-1"),
            water_key("canopy", "south", "soil-1"),
            water_key("canopy", "north", "soil-2"),
        ] {
            let authorization = MaximumAuthorization {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: wrong_key,
                amount: 2.0,
                basis: request.basis,
            };
            assert_eq!(
                validate_maximum_authorization(&request, &authorization),
                Err(ResourceProtocolViolation::KeyMismatch)
            );
        }
    }

    #[test]
    fn proportional_authorization_rejects_duplicate_request_identity() {
        let request = water_request(water_key("canopy", "north", "soil-1"));
        let duplicate = ResourceRequest {
            amount: 1.0,
            ..request.clone()
        };

        assert_eq!(
            authorize_proportionally(
                &[request, duplicate],
                &BTreeMap::new(),
                ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
            ),
            Err(ResourceProtocolViolation::DuplicateRequestIdentity)
        );
    }

    #[test]
    fn proportional_authorization_rejects_mixed_transaction_ids() {
        let request = water_request(water_key("canopy", "north", "soil-1"));
        let other_transaction = ResourceRequest {
            transaction_id: TransactionId(2),
            owner_id: ResourceOwnerId::try_new("shrub").expect("owner"),
            ..request.clone()
        };

        assert_eq!(
            authorize_proportionally(
                &[request, other_transaction],
                &BTreeMap::new(),
                ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
            ),
            Err(ResourceProtocolViolation::TransactionMismatch)
        );
    }
}
