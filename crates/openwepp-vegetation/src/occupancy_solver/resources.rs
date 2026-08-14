//! Typed V3 occupancy water request and authorization boundaries.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{
    MaximumAuthorization, OccupancyId, ResourceAmountBasis, ResourceOwnerId, ResourceRequest,
    SoilLayerId, TileId, TransactionId, WaterResourceKey, validate_maximum_authorization,
};
use thiserror::Error;

const WATER_STAND_BASIS: ResourceAmountBasis =
    ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval;

pub type PotentialWaterRequest = ResourceRequest<WaterResourceKey, f64>;
pub type WaterMaximumAuthorization = MaximumAuthorization<WaterResourceKey, f64>;

#[derive(Clone, Debug, PartialEq)]
pub struct OccupancyRootLayers {
    pub occupancy_id: OccupancyId,
    pub layer_ids: Vec<SoilLayerId>,
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum WaterResourceBoundaryError {
    #[error("duplicate configured occupancy identity")]
    DuplicateConfiguredOccupancy,
    #[error("configured occupancy has no root layers")]
    MissingConfiguredRootLayer,
    #[error("configured occupancy repeats a root-layer identity")]
    DuplicateConfiguredRootLayer,
    #[error("water request identity is duplicated")]
    DuplicateRequestIdentity,
    #[error("water authorization identity is duplicated")]
    DuplicateAuthorizationIdentity,
    #[error("water resource key set differs from configured occupancy root layers")]
    ResourceKeySetMismatch,
    #[error("water resource transaction identity differs from the candidate transaction")]
    TransactionMismatch,
    #[error("water resource owner identity differs from the vegetation owner")]
    OwnerMismatch,
    #[error("water resource amount basis is not stand-ground interval mass")]
    BasisMismatch,
    #[error("water resource amount is nonfinite")]
    NonFiniteAmount,
    #[error("water resource amount is negative")]
    NegativeAmount,
    #[error("water authorization does not correspond exactly to its request")]
    AuthorizationCorrespondence,
    #[error("water authorization exceeds its potential request")]
    AuthorizationExceedsRequest,
    #[error("tile fraction is nonfinite or not positive")]
    InvalidTileFraction,
    #[error("interval duration is nonfinite or not positive")]
    InvalidInterval,
    #[error("water authorization is missing for the requested occupancy and layer")]
    MissingAuthorization,
    #[error("finalized stand-ground use exceeds authorization")]
    FinalizedUseExceedsAuthorization,
}

impl From<WaterResourceBoundaryError> for crate::VegetationError {
    fn from(error: WaterResourceBoundaryError) -> Self {
        use WaterResourceBoundaryError as E;
        let message = error.to_string();
        match error {
            E::BasisMismatch
            | E::NonFiniteAmount
            | E::NegativeAmount
            | E::InvalidTileFraction
            | E::InvalidInterval => Self::ResourceOperand(message),
            E::AuthorizationExceedsRequest | E::FinalizedUseExceedsAuthorization => {
                Self::ResourceBound(message)
            }
            E::DuplicateConfiguredOccupancy
            | E::MissingConfiguredRootLayer
            | E::DuplicateConfiguredRootLayer
            | E::DuplicateRequestIdentity
            | E::DuplicateAuthorizationIdentity
            | E::ResourceKeySetMismatch
            | E::TransactionMismatch
            | E::OwnerMismatch
            | E::AuthorizationCorrespondence
            | E::MissingAuthorization => Self::ResourceIdentity(message),
        }
    }
}

/// A complete, single-transaction batch of potential occupancy water requests.
#[derive(Clone, Debug, PartialEq)]
pub struct PotentialWaterRequestBatch {
    transaction_id: TransactionId,
    owner_id: ResourceOwnerId,
    requests: Vec<PotentialWaterRequest>,
}

impl PotentialWaterRequestBatch {
    pub fn try_from_stand_amounts(
        transaction_id: TransactionId,
        owner_id: ResourceOwnerId,
        configured_root_layers: &[OccupancyRootLayers],
        amounts_kg_m2_stand_ground: &BTreeMap<WaterResourceKey, f64>,
    ) -> Result<Self, WaterResourceBoundaryError> {
        validate_configured_root_layers(configured_root_layers)?;
        let requests = configured_root_layers
            .iter()
            .flat_map(|occupancy| {
                occupancy.layer_ids.iter().map(|layer_id| {
                    let key = WaterResourceKey {
                        occupancy_id: occupancy.occupancy_id.clone(),
                        layer_id: layer_id.clone(),
                    };
                    let amount = amounts_kg_m2_stand_ground.get(&key).copied();
                    (key, amount)
                })
            })
            .map(|(key, amount)| {
                amount
                    .ok_or(WaterResourceBoundaryError::ResourceKeySetMismatch)
                    .map(|amount| PotentialWaterRequest {
                        transaction_id,
                        owner_id: owner_id.clone(),
                        key,
                        amount,
                        basis: WATER_STAND_BASIS,
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        if requests.len() != amounts_kg_m2_stand_ground.len() {
            return Err(WaterResourceBoundaryError::ResourceKeySetMismatch);
        }
        Self::try_from_requests(transaction_id, owner_id, configured_root_layers, requests)
    }

    pub fn try_from_requests(
        transaction_id: TransactionId,
        owner_id: ResourceOwnerId,
        configured_root_layers: &[OccupancyRootLayers],
        requests: Vec<PotentialWaterRequest>,
    ) -> Result<Self, WaterResourceBoundaryError> {
        let expected_keys = validate_configured_root_layers(configured_root_layers)?;
        let mut requests_by_key = BTreeMap::new();
        for request in requests {
            validate_request_identity(&request, transaction_id, &owner_id)?;
            if requests_by_key
                .insert(request.key.clone(), request)
                .is_some()
            {
                return Err(WaterResourceBoundaryError::DuplicateRequestIdentity);
            }
        }
        let actual_keys = requests_by_key.keys().cloned().collect::<BTreeSet<_>>();
        if actual_keys != expected_keys {
            return Err(WaterResourceBoundaryError::ResourceKeySetMismatch);
        }
        let requests = configured_root_layers
            .iter()
            .flat_map(|occupancy| {
                occupancy.layer_ids.iter().map(|layer_id| WaterResourceKey {
                    occupancy_id: occupancy.occupancy_id.clone(),
                    layer_id: layer_id.clone(),
                })
            })
            .map(|key| {
                requests_by_key
                    .remove(&key)
                    .ok_or(WaterResourceBoundaryError::ResourceKeySetMismatch)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            transaction_id,
            owner_id,
            requests,
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
    pub fn requests(&self) -> &[PotentialWaterRequest] {
        &self.requests
    }

    #[must_use]
    pub fn into_requests(self) -> Vec<PotentialWaterRequest> {
        self.requests
    }
}

/// Maximum authorizations proven to correspond one-for-one with a request batch.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidatedWaterAuthorizations {
    transaction_id: TransactionId,
    owner_id: ResourceOwnerId,
    requests: BTreeMap<WaterResourceKey, PotentialWaterRequest>,
    authorizations: BTreeMap<WaterResourceKey, WaterMaximumAuthorization>,
}

impl ValidatedWaterAuthorizations {
    pub fn try_new(
        request_batch: &PotentialWaterRequestBatch,
        authorizations: Vec<WaterMaximumAuthorization>,
    ) -> Result<Self, WaterResourceBoundaryError> {
        let requests = request_batch
            .requests
            .iter()
            .cloned()
            .map(|request| (request.key.clone(), request))
            .collect::<BTreeMap<_, _>>();
        let mut validated = BTreeMap::new();
        for authorization in authorizations {
            if validated.contains_key(&authorization.key) {
                return Err(WaterResourceBoundaryError::DuplicateAuthorizationIdentity);
            }
            let request = requests
                .get(&authorization.key)
                .ok_or(WaterResourceBoundaryError::AuthorizationCorrespondence)?;
            validate_authorization(request, &authorization)?;
            validated.insert(authorization.key.clone(), authorization);
        }
        if validated.len() != requests.len() {
            return Err(WaterResourceBoundaryError::AuthorizationCorrespondence);
        }
        Ok(Self {
            transaction_id: request_batch.transaction_id,
            owner_id: request_batch.owner_id.clone(),
            requests,
            authorizations: validated,
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
    pub fn requests(&self) -> &BTreeMap<WaterResourceKey, PotentialWaterRequest> {
        &self.requests
    }

    #[must_use]
    pub fn authorizations(&self) -> &BTreeMap<WaterResourceKey, WaterMaximumAuthorization> {
        &self.authorizations
    }

    /// Convert validated stand-ground authorizations to occupancy tile-ground caps once.
    pub fn to_local_cap_map(
        &self,
        tile_fractions: &BTreeMap<TileId, f64>,
    ) -> Result<BTreeMap<WaterResourceKey, f64>, WaterResourceBoundaryError> {
        self.authorizations
            .iter()
            .map(|(key, authorization)| {
                let tile_fraction = tile_fractions
                    .get(&key.occupancy_id.tile_id)
                    .copied()
                    .ok_or(WaterResourceBoundaryError::InvalidTileFraction)?;
                if !tile_fraction.is_finite() || tile_fraction <= 0.0 {
                    return Err(WaterResourceBoundaryError::InvalidTileFraction);
                }
                let local_cap = authorization.amount / tile_fraction;
                if !local_cap.is_finite() {
                    return Err(WaterResourceBoundaryError::NonFiniteAmount);
                }
                Ok((key.clone(), local_cap))
            })
            .collect()
    }

    /// Validate a finalized stand-ground use against its exact authorization.
    ///
    /// V5 admits no representational normalization at this ownership boundary:
    /// any finite value above the immutable authorization rejects.
    pub fn validate_finalized_stand_amount(
        &self,
        key: &WaterResourceKey,
        finalized_kg_m2_stand_ground: f64,
        tile_fractions: &BTreeMap<TileId, f64>,
        interval_s: f64,
    ) -> Result<f64, WaterResourceBoundaryError> {
        validate_amount(finalized_kg_m2_stand_ground)?;
        let authorization = self
            .authorizations
            .get(key)
            .ok_or(WaterResourceBoundaryError::MissingAuthorization)?;
        let tile_fraction = tile_fractions
            .get(&key.occupancy_id.tile_id)
            .copied()
            .ok_or(WaterResourceBoundaryError::InvalidTileFraction)?;
        if !tile_fraction.is_finite() || tile_fraction <= 0.0 {
            return Err(WaterResourceBoundaryError::InvalidTileFraction);
        }
        if !interval_s.is_finite() || interval_s <= 0.0 {
            return Err(WaterResourceBoundaryError::InvalidInterval);
        }
        if finalized_kg_m2_stand_ground > authorization.amount {
            Err(WaterResourceBoundaryError::FinalizedUseExceedsAuthorization)
        } else {
            Ok(finalized_kg_m2_stand_ground)
        }
    }
}

fn validate_configured_root_layers(
    configured: &[OccupancyRootLayers],
) -> Result<BTreeSet<WaterResourceKey>, WaterResourceBoundaryError> {
    let mut occupancies = BTreeSet::new();
    let mut keys = BTreeSet::new();
    for occupancy in configured {
        if !occupancies.insert(occupancy.occupancy_id.clone()) {
            return Err(WaterResourceBoundaryError::DuplicateConfiguredOccupancy);
        }
        if occupancy.layer_ids.is_empty() {
            return Err(WaterResourceBoundaryError::MissingConfiguredRootLayer);
        }
        let mut layers = BTreeSet::new();
        for layer_id in &occupancy.layer_ids {
            if !layers.insert(layer_id.clone()) {
                return Err(WaterResourceBoundaryError::DuplicateConfiguredRootLayer);
            }
            keys.insert(WaterResourceKey {
                occupancy_id: occupancy.occupancy_id.clone(),
                layer_id: layer_id.clone(),
            });
        }
    }
    Ok(keys)
}

fn validate_request_identity(
    request: &PotentialWaterRequest,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
) -> Result<(), WaterResourceBoundaryError> {
    if request.transaction_id != transaction_id {
        return Err(WaterResourceBoundaryError::TransactionMismatch);
    }
    if &request.owner_id != owner_id {
        return Err(WaterResourceBoundaryError::OwnerMismatch);
    }
    if request.basis != WATER_STAND_BASIS {
        return Err(WaterResourceBoundaryError::BasisMismatch);
    }
    validate_amount(request.amount)
}

fn validate_authorization(
    request: &PotentialWaterRequest,
    authorization: &WaterMaximumAuthorization,
) -> Result<(), WaterResourceBoundaryError> {
    validate_maximum_authorization(request, authorization).map_err(|violation| {
        use openwepp_kernel_contract::ResourceProtocolViolation;
        match violation {
            ResourceProtocolViolation::TransactionMismatch => {
                WaterResourceBoundaryError::TransactionMismatch
            }
            ResourceProtocolViolation::OwnerMismatch => WaterResourceBoundaryError::OwnerMismatch,
            ResourceProtocolViolation::BasisMismatch => WaterResourceBoundaryError::BasisMismatch,
            ResourceProtocolViolation::NonFinite => WaterResourceBoundaryError::NonFiniteAmount,
            ResourceProtocolViolation::Negative => WaterResourceBoundaryError::NegativeAmount,
            ResourceProtocolViolation::AuthorizationExceedsRequest => {
                WaterResourceBoundaryError::AuthorizationExceedsRequest
            }
            ResourceProtocolViolation::KeyMismatch
            | ResourceProtocolViolation::FinalizedUseExceedsAuthorization
            | ResourceProtocolViolation::DuplicateRequestIdentity => {
                WaterResourceBoundaryError::AuthorizationCorrespondence
            }
        }
    })
}

fn validate_amount(amount: f64) -> Result<(), WaterResourceBoundaryError> {
    if !amount.is_finite() {
        return Err(WaterResourceBoundaryError::NonFiniteAmount);
    }
    if amount < 0.0 {
        return Err(WaterResourceBoundaryError::NegativeAmount);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_kernel_contract::{ResourceIdentityError, StratumId};

    fn owner(value: &str) -> ResourceOwnerId {
        ResourceOwnerId::try_new(value).expect("owner should be valid")
    }

    fn layer(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("layer should be valid")
    }

    fn occupancy(stratum: &str, tile: &str) -> OccupancyId {
        OccupancyId {
            stratum_id: StratumId::try_new(stratum).expect("stratum should be valid"),
            tile_id: TileId::try_new(tile).expect("tile should be valid"),
        }
    }

    fn configured() -> Vec<OccupancyRootLayers> {
        vec![
            OccupancyRootLayers {
                occupancy_id: occupancy("upper", "tile-a"),
                layer_ids: vec![layer("surface"), layer("deep")],
            },
            OccupancyRootLayers {
                occupancy_id: occupancy("upper", "tile-b"),
                layer_ids: vec![layer("surface"), layer("deep")],
            },
        ]
    }

    fn amount_map() -> BTreeMap<WaterResourceKey, f64> {
        configured()
            .into_iter()
            .flat_map(|occupancy| {
                occupancy
                    .layer_ids
                    .into_iter()
                    .enumerate()
                    .map(move |(index, layer_id)| {
                        let amount = if index == 0 { 0.1 } else { 0.15 };
                        (
                            WaterResourceKey {
                                occupancy_id: occupancy.occupancy_id.clone(),
                                layer_id,
                            },
                            amount,
                        )
                    })
            })
            .collect()
    }

    fn request_batch() -> PotentialWaterRequestBatch {
        PotentialWaterRequestBatch::try_from_stand_amounts(
            TransactionId(41),
            owner("vegetation"),
            &configured(),
            &amount_map(),
        )
        .expect("request batch should validate")
    }

    fn authorizations(batch: &PotentialWaterRequestBatch) -> Vec<WaterMaximumAuthorization> {
        batch
            .requests()
            .iter()
            .map(|request| WaterMaximumAuthorization {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount: request.amount * 0.5,
                basis: request.basis,
            })
            .collect()
    }

    #[test]
    fn builds_complete_stand_ground_requests_in_configured_order() {
        let batch = request_batch();
        assert_eq!(batch.transaction_id(), TransactionId(41));
        assert_eq!(batch.owner_id().as_str(), "vegetation");
        assert_eq!(batch.requests().len(), 4);
        assert_eq!(
            batch.requests()[0].key.occupancy_id,
            occupancy("upper", "tile-a")
        );
        assert_eq!(batch.requests()[0].key.layer_id, layer("surface"));
        assert_eq!(batch.requests()[1].key.layer_id, layer("deep"));
        assert!(
            batch
                .requests()
                .iter()
                .all(|request| request.basis == WATER_STAND_BASIS)
        );

        let mut reversed = batch.requests().to_vec();
        reversed.reverse();
        let canonical = PotentialWaterRequestBatch::try_from_requests(
            TransactionId(41),
            owner("vegetation"),
            &configured(),
            reversed,
        )
        .expect("request ordering should canonicalize");
        assert_eq!(canonical.requests(), batch.requests());
    }

    #[test]
    fn rejects_missing_extra_duplicate_and_wrong_identity_requests() {
        let mut missing = amount_map();
        let removed = missing.keys().next().cloned().expect("key should exist");
        missing.remove(&removed);
        assert_eq!(
            PotentialWaterRequestBatch::try_from_stand_amounts(
                TransactionId(41),
                owner("vegetation"),
                &configured(),
                &missing,
            ),
            Err(WaterResourceBoundaryError::ResourceKeySetMismatch)
        );

        let mut extra = amount_map();
        extra.insert(
            WaterResourceKey {
                occupancy_id: occupancy("wrong", "tile-a"),
                layer_id: layer("surface"),
            },
            0.1,
        );
        assert_eq!(
            PotentialWaterRequestBatch::try_from_stand_amounts(
                TransactionId(41),
                owner("vegetation"),
                &configured(),
                &extra,
            ),
            Err(WaterResourceBoundaryError::ResourceKeySetMismatch)
        );

        let batch = request_batch();
        let mut duplicate = batch.requests().to_vec();
        duplicate.push(duplicate[0].clone());
        assert_eq!(
            PotentialWaterRequestBatch::try_from_requests(
                TransactionId(41),
                owner("vegetation"),
                &configured(),
                duplicate,
            ),
            Err(WaterResourceBoundaryError::DuplicateRequestIdentity)
        );

        for (requests, expected) in [
            {
                let mut requests = batch.requests().to_vec();
                requests[0].transaction_id = TransactionId(40);
                (requests, WaterResourceBoundaryError::TransactionMismatch)
            },
            {
                let mut requests = batch.requests().to_vec();
                requests[0].owner_id = owner("other");
                (requests, WaterResourceBoundaryError::OwnerMismatch)
            },
            {
                let mut requests = batch.requests().to_vec();
                requests[0].basis = ResourceAmountBasis::WaterKgPerSquareMeterInterval;
                (requests, WaterResourceBoundaryError::BasisMismatch)
            },
        ] {
            assert_eq!(
                PotentialWaterRequestBatch::try_from_requests(
                    TransactionId(41),
                    owner("vegetation"),
                    &configured(),
                    requests,
                ),
                Err(expected)
            );
        }
    }

    #[test]
    fn rejects_invalid_amounts_and_configured_layer_shapes() {
        for invalid in [f64::NAN, f64::INFINITY, -0.1] {
            let mut amounts = amount_map();
            let key = amounts.keys().next().cloned().expect("key should exist");
            amounts.insert(key, invalid);
            assert!(matches!(
                PotentialWaterRequestBatch::try_from_stand_amounts(
                    TransactionId(41),
                    owner("vegetation"),
                    &configured(),
                    &amounts,
                ),
                Err(WaterResourceBoundaryError::NonFiniteAmount
                    | WaterResourceBoundaryError::NegativeAmount)
            ));
        }

        let mut duplicate_occupancy = configured();
        duplicate_occupancy.push(duplicate_occupancy[0].clone());
        assert_eq!(
            validate_configured_root_layers(&duplicate_occupancy),
            Err(WaterResourceBoundaryError::DuplicateConfiguredOccupancy)
        );
        let mut duplicate_layer = configured();
        duplicate_layer[0].layer_ids.push(layer("surface"));
        assert_eq!(
            validate_configured_root_layers(&duplicate_layer),
            Err(WaterResourceBoundaryError::DuplicateConfiguredRootLayer)
        );
        let mut missing_layer = configured();
        missing_layer[0].layer_ids.clear();
        assert_eq!(
            validate_configured_root_layers(&missing_layer),
            Err(WaterResourceBoundaryError::MissingConfiguredRootLayer)
        );
    }

    #[test]
    fn validates_authorizations_one_for_one_without_identity_normalization() {
        let batch = request_batch();
        let accepted = ValidatedWaterAuthorizations::try_new(&batch, authorizations(&batch))
            .expect("authorization batch should validate");
        assert_eq!(accepted.authorizations().len(), batch.requests().len());

        let mut missing = authorizations(&batch);
        missing.pop();
        assert_eq!(
            ValidatedWaterAuthorizations::try_new(&batch, missing),
            Err(WaterResourceBoundaryError::AuthorizationCorrespondence)
        );

        let mut duplicate = authorizations(&batch);
        duplicate.push(duplicate[0].clone());
        assert_eq!(
            ValidatedWaterAuthorizations::try_new(&batch, duplicate),
            Err(WaterResourceBoundaryError::DuplicateAuthorizationIdentity)
        );

        let mut wrong_owner = authorizations(&batch);
        wrong_owner[0].owner_id = owner("other");
        assert_eq!(
            ValidatedWaterAuthorizations::try_new(&batch, wrong_owner),
            Err(WaterResourceBoundaryError::OwnerMismatch)
        );

        let mut wrong_basis = authorizations(&batch);
        wrong_basis[0].basis = ResourceAmountBasis::WaterKgPerSquareMeterInterval;
        assert_eq!(
            ValidatedWaterAuthorizations::try_new(&batch, wrong_basis),
            Err(WaterResourceBoundaryError::BasisMismatch)
        );

        let mut excess = authorizations(&batch);
        excess[0].amount = batch.requests()[0].amount + f64::EPSILON;
        assert_eq!(
            ValidatedWaterAuthorizations::try_new(&batch, excess),
            Err(WaterResourceBoundaryError::AuthorizationExceedsRequest)
        );
    }

    #[test]
    fn converts_authorizations_to_tile_basis_exactly_once() {
        let batch = request_batch();
        let validated = ValidatedWaterAuthorizations::try_new(&batch, authorizations(&batch))
            .expect("authorization batch should validate");
        let fractions = BTreeMap::from([
            (
                TileId::try_new("tile-a").expect("tile should be valid"),
                0.25,
            ),
            (
                TileId::try_new("tile-b").expect("tile should be valid"),
                0.75,
            ),
        ]);
        let caps = validated
            .to_local_cap_map(&fractions)
            .expect("tile caps should construct");
        let request = &batch.requests()[0];
        assert_eq!(
            caps.get(&request.key).copied(),
            Some(request.amount * 0.5 / 0.25)
        );
    }

    #[test]
    fn finalized_use_rejects_even_one_bit_above_authorization() {
        let batch = request_batch();
        let validated = ValidatedWaterAuthorizations::try_new(&batch, authorizations(&batch))
            .expect("authorization batch should validate");
        let key = &batch.requests()[0].key;
        let authorization = validated.authorizations()[key].amount;
        let tile_fraction = 0.25;
        let tile_fractions = BTreeMap::from([
            (
                TileId::try_new("tile-a").expect("tile should be valid"),
                tile_fraction,
            ),
            (
                TileId::try_new("tile-b").expect("tile should be valid"),
                0.75,
            ),
        ]);
        let interval_s = 1_800.0;
        let one_bit_above = f64::from_bits(authorization.to_bits() + 1);
        let one_bit_below = f64::from_bits(authorization.to_bits() - 1);

        assert_eq!(
            validated.validate_finalized_stand_amount(
                key,
                one_bit_above,
                &tile_fractions,
                interval_s,
            ),
            Err(WaterResourceBoundaryError::FinalizedUseExceedsAuthorization)
        );
        assert_eq!(
            validated.validate_finalized_stand_amount(
                key,
                authorization,
                &tile_fractions,
                interval_s,
            ),
            Ok(authorization)
        );
        assert_eq!(
            validated.validate_finalized_stand_amount(
                key,
                one_bit_below,
                &tile_fractions,
                interval_s,
            ),
            Ok(one_bit_below)
        );

        let wrong_key = WaterResourceKey {
            occupancy_id: occupancy("upper", "wrong-tile"),
            layer_id: layer("surface"),
        };
        assert_eq!(
            validated.validate_finalized_stand_amount(&wrong_key, 0.0, &tile_fractions, interval_s),
            Err(WaterResourceBoundaryError::MissingAuthorization)
        );
    }

    #[test]
    fn identity_constructor_rejects_empty_values_upstream() {
        assert_eq!(
            ResourceOwnerId::try_new(""),
            Err(ResourceIdentityError::EmptyOwner)
        );
    }

    #[test]
    fn water_boundary_categories_retain_canonical_transaction_codes() {
        let batch = request_batch();
        let mut duplicate = batch.requests().to_vec();
        duplicate.push(duplicate[0].clone());
        let identity = PotentialWaterRequestBatch::try_from_requests(
            batch.transaction_id(),
            batch.owner_id().clone(),
            &configured(),
            duplicate,
        )
        .unwrap_err();
        assert!(matches!(
            crate::VegetationError::from(identity),
            crate::VegetationError::ResourceIdentity(_)
        ));

        let mut nonfinite = amount_map();
        *nonfinite.values_mut().next().unwrap() = f64::NAN;
        let operand = PotentialWaterRequestBatch::try_from_stand_amounts(
            batch.transaction_id(),
            batch.owner_id().clone(),
            &configured(),
            &nonfinite,
        )
        .unwrap_err();
        assert!(matches!(
            crate::VegetationError::from(operand),
            crate::VegetationError::ResourceOperand(_)
        ));

        let mut excessive = authorizations(&batch);
        excessive[0].amount = batch.requests()[0].amount + 1.0;
        let bound = ValidatedWaterAuthorizations::try_new(&batch, excessive).unwrap_err();
        assert!(matches!(
            crate::VegetationError::from(bound),
            crate::VegetationError::ResourceBound(_)
        ));

        let validated = ValidatedWaterAuthorizations::try_new(&batch, authorizations(&batch))
            .expect("valid authorizations");
        let key = &batch.requests()[0].key;
        let authorization = validated.authorizations()[key].amount;
        let finalized_bound = validated
            .validate_finalized_stand_amount(
                key,
                f64::from_bits(authorization.to_bits() + 1),
                &BTreeMap::from([
                    (TileId::try_new("tile-a").unwrap(), 0.5),
                    (TileId::try_new("tile-b").unwrap(), 0.5),
                ]),
                1.0,
            )
            .unwrap_err();
        assert!(matches!(
            crate::VegetationError::from(finalized_bound),
            crate::VegetationError::ResourceBound(_)
        ));
    }
}
