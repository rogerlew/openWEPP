//! Typed E19 mineral-nitrogen request, authorization, and final-use boundary.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{
    FinalizedUse, MaximumAuthorization, MineralNitrogenKey, MineralNitrogenSpecies,
    ResourceAmountBasis, ResourceOwnerId, ResourceProtocolViolation, ResourceRequest,
    TransactionId, validate_maximum_authorization, validate_resource_protocol,
};
use thiserror::Error;

use crate::config::StratumConfiguration;

const NITROGEN_BASIS: ResourceAmountBasis = ResourceAmountBasis::NitrogenKgPerSquareMeterInterval;
const CONFIGURATION_FRACTION_TOLERANCE: f64 = 1.0e-12;

pub type PotentialMineralNitrogenRequest = ResourceRequest<MineralNitrogenKey, f64>;
pub type MineralNitrogenMaximumAuthorization = MaximumAuthorization<MineralNitrogenKey, f64>;
pub type MineralNitrogenFinalizedUse = FinalizedUse<MineralNitrogenKey, f64>;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum NitrogenProtocolError {
    #[error("configured mineral-nitrogen layer set is empty")]
    EmptyConfiguredLayers,
    #[error("configured mineral-nitrogen layer identity is duplicated")]
    DuplicateConfiguredLayer,
    #[error("configured mineral-nitrogen root fraction is invalid")]
    InvalidConfiguredFraction,
    #[error("configured mineral-nitrogen root fractions do not sum to one")]
    ConfiguredFractionsDoNotClose,
    #[error("mineral-nitrogen amount is nonfinite")]
    NonFiniteAmount,
    #[error("mineral-nitrogen amount is negative")]
    NegativeAmount,
    #[error("mineral-nitrogen request identity is duplicated")]
    DuplicateRequestIdentity,
    #[error("mineral-nitrogen authorization identity is duplicated")]
    DuplicateAuthorizationIdentity,
    #[error("mineral-nitrogen resource key set differs from configured layers and species")]
    ResourceKeySetMismatch,
    #[error("mineral-nitrogen authorization does not correspond exactly to its request")]
    AuthorizationCorrespondence,
    #[error("mineral-nitrogen transaction identity mismatch")]
    TransactionMismatch,
    #[error("mineral-nitrogen owner identity mismatch")]
    OwnerMismatch,
    #[error("mineral-nitrogen owner is not the exact requesting stratum identity")]
    StratumOwnerMismatch,
    #[error("mineral-nitrogen amount basis mismatch")]
    BasisMismatch,
    #[error("mineral-nitrogen authorization exceeds its potential request")]
    AuthorizationExceedsRequest,
    #[error("finalized mineral-nitrogen use exceeds its maximum authorization")]
    FinalizedUseExceedsAuthorization,
}

impl From<NitrogenProtocolError> for crate::VegetationError {
    fn from(error: NitrogenProtocolError) -> Self {
        use NitrogenProtocolError as E;
        let message = error.to_string();
        match error {
            E::InvalidConfiguredFraction
            | E::ConfiguredFractionsDoNotClose
            | E::NonFiniteAmount
            | E::NegativeAmount
            | E::BasisMismatch => Self::ResourceOperand(message),
            E::AuthorizationExceedsRequest | E::FinalizedUseExceedsAuthorization => {
                Self::ResourceBound(message)
            }
            E::EmptyConfiguredLayers
            | E::DuplicateConfiguredLayer
            | E::DuplicateRequestIdentity
            | E::DuplicateAuthorizationIdentity
            | E::ResourceKeySetMismatch
            | E::AuthorizationCorrespondence
            | E::TransactionMismatch
            | E::OwnerMismatch
            | E::StratumOwnerMismatch => Self::ResourceIdentity(message),
        }
    }
}

/// Complete potential E19 requests for one stratum and transaction.
///
/// All scalar nitrogen amounts accepted or returned by this module are
/// interval `kg N m^-2` on the transaction stand-ground area basis.
/// Internal retranslocated nitrogen is applied before the external shortfall is
/// partitioned across configured layers and chemical species. This object does
/// not debit the internal pool; it records the potential calculation only.
#[derive(Clone, Debug, PartialEq)]
pub struct PotentialMineralNitrogenRequestBatch {
    transaction_id: TransactionId,
    owner_id: ResourceOwnerId,
    potential_total_demand: f64,
    internal_offer: f64,
    potential_internal_use: f64,
    potential_external_shortfall: f64,
    requests: Vec<PotentialMineralNitrogenRequest>,
}

impl PotentialMineralNitrogenRequestBatch {
    pub fn try_from_stratum_configuration(
        transaction_id: TransactionId,
        owner_id: ResourceOwnerId,
        stratum: &StratumConfiguration,
        potential_total_demand: f64,
        internal_retranslocated_offer: f64,
    ) -> Result<Self, NitrogenProtocolError> {
        if owner_id.as_str() != stratum.stratum_id.as_str() {
            return Err(NitrogenProtocolError::StratumOwnerMismatch);
        }
        validate_amount(potential_total_demand)?;
        validate_amount(internal_retranslocated_offer)?;
        if !stratum.nh4_request_fraction.is_finite()
            || !(0.0..=1.0).contains(&stratum.nh4_request_fraction)
        {
            return Err(NitrogenProtocolError::InvalidConfiguredFraction);
        }

        let configured = validate_configured_layers(stratum)?;
        let potential_internal_use = internal_retranslocated_offer.min(potential_total_demand);
        let potential_external_shortfall = potential_total_demand - potential_internal_use;
        let ammonium_fraction = stratum.nh4_request_fraction;
        let nitrate_fraction = 1.0 - ammonium_fraction;

        let mut requests = Vec::with_capacity(stratum.root_layers.len() * 2);
        for root in &stratum.root_layers {
            for (species, species_fraction) in [
                (MineralNitrogenSpecies::Ammonium, ammonium_fraction),
                (MineralNitrogenSpecies::Nitrate, nitrate_fraction),
            ] {
                requests.push(PotentialMineralNitrogenRequest {
                    transaction_id,
                    owner_id: owner_id.clone(),
                    key: MineralNitrogenKey {
                        layer_id: root.layer_id.clone(),
                        species,
                    },
                    amount: potential_external_shortfall
                        * root.mineral_n_root_fraction
                        * species_fraction,
                    basis: NITROGEN_BASIS,
                });
            }
        }

        Self::try_from_requests(
            transaction_id,
            owner_id,
            potential_total_demand,
            internal_retranslocated_offer,
            requests,
            &configured,
        )
    }

    fn try_from_requests(
        transaction_id: TransactionId,
        owner_id: ResourceOwnerId,
        potential_total_demand: f64,
        internal_offer: f64,
        requests: Vec<PotentialMineralNitrogenRequest>,
        expected_keys: &BTreeSet<MineralNitrogenKey>,
    ) -> Result<Self, NitrogenProtocolError> {
        let mut requests_by_key = BTreeMap::new();
        for request in requests {
            validate_request_identity(&request, transaction_id, &owner_id)?;
            if requests_by_key
                .insert(request.key.clone(), request)
                .is_some()
            {
                return Err(NitrogenProtocolError::DuplicateRequestIdentity);
            }
        }
        let actual_keys = requests_by_key.keys().cloned().collect::<BTreeSet<_>>();
        if &actual_keys != expected_keys {
            return Err(NitrogenProtocolError::ResourceKeySetMismatch);
        }

        let requests = expected_keys
            .iter()
            .map(|key| {
                requests_by_key
                    .remove(key)
                    .ok_or(NitrogenProtocolError::ResourceKeySetMismatch)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let potential_internal_use = internal_offer.min(potential_total_demand);
        Ok(Self {
            transaction_id,
            owner_id,
            potential_total_demand,
            internal_offer,
            potential_internal_use,
            potential_external_shortfall: potential_total_demand - potential_internal_use,
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
    pub fn potential_total_demand(&self) -> f64 {
        self.potential_total_demand
    }

    #[must_use]
    pub fn internal_offer(&self) -> f64 {
        self.internal_offer
    }

    #[must_use]
    pub fn potential_internal_use(&self) -> f64 {
        self.potential_internal_use
    }

    #[must_use]
    pub fn potential_external_shortfall(&self) -> f64 {
        self.potential_external_shortfall
    }

    #[must_use]
    pub fn requests(&self) -> &[PotentialMineralNitrogenRequest] {
        &self.requests
    }
}

/// Complete maximum authorizations proven against their exact potential batch.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidatedMineralNitrogenAuthorizations {
    request_batch: PotentialMineralNitrogenRequestBatch,
    authorizations: BTreeMap<MineralNitrogenKey, MineralNitrogenMaximumAuthorization>,
}

impl ValidatedMineralNitrogenAuthorizations {
    pub fn try_new(
        request_batch: &PotentialMineralNitrogenRequestBatch,
        authorizations: Vec<MineralNitrogenMaximumAuthorization>,
    ) -> Result<Self, NitrogenProtocolError> {
        let requests = request_batch
            .requests
            .iter()
            .map(|request| (request.key.clone(), request))
            .collect::<BTreeMap<_, _>>();
        let mut validated = BTreeMap::new();
        for authorization in authorizations {
            validate_authorization_identity(
                &authorization,
                request_batch.transaction_id,
                &request_batch.owner_id,
            )?;
            if validated.contains_key(&authorization.key) {
                return Err(NitrogenProtocolError::DuplicateAuthorizationIdentity);
            }
            let request = requests
                .get(&authorization.key)
                .ok_or(NitrogenProtocolError::AuthorizationCorrespondence)?;
            validate_maximum_authorization(request, &authorization)
                .map_err(map_protocol_violation)?;
            validated.insert(authorization.key.clone(), authorization);
        }
        if validated.len() != requests.len() {
            return Err(NitrogenProtocolError::AuthorizationCorrespondence);
        }
        Ok(Self {
            request_batch: request_batch.clone(),
            authorizations: validated,
        })
    }

    #[must_use]
    pub fn request_batch(&self) -> &PotentialMineralNitrogenRequestBatch {
        &self.request_batch
    }

    #[must_use]
    pub fn authorizations(
        &self,
    ) -> &BTreeMap<MineralNitrogenKey, MineralNitrogenMaximumAuthorization> {
        &self.authorizations
    }

    /// Finalize external uptake after final water-limited C/N demand is known.
    ///
    /// Unused authorization remains unused. When final external demand is less
    /// than the authorization sum, each exact layer/species authorization is
    /// reduced by the common proportional factor required by E19.
    pub fn finalize(
        &self,
        final_total_demand: f64,
    ) -> Result<MineralNitrogenFinalization, NitrogenProtocolError> {
        validate_amount(final_total_demand)?;
        let internal_use = self.request_batch.internal_offer.min(final_total_demand);
        let final_external_demand = final_total_demand - internal_use;
        let ordered_authorizations = self
            .request_batch
            .requests
            .iter()
            .map(|request| {
                self.authorizations
                    .get(&request.key)
                    .ok_or(NitrogenProtocolError::AuthorizationCorrespondence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let authorization_sum = compensated_sum(
            self.request_batch
                .requests
                .iter()
                .zip(&ordered_authorizations)
                .map(|(_, authorization)| authorization.amount),
        );
        validate_amount(authorization_sum)?;
        let external_use = final_external_demand.min(authorization_sum);

        let mut finalized_uses = Vec::with_capacity(self.request_batch.requests.len());
        for (request, authorization) in self
            .request_batch
            .requests
            .iter()
            .zip(ordered_authorizations)
        {
            let amount = if authorization_sum == 0.0 {
                0.0
            } else if final_external_demand >= authorization_sum {
                authorization.amount
            } else {
                external_use * authorization.amount / authorization_sum
            };
            let finalized = MineralNitrogenFinalizedUse {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount,
                basis: request.basis,
            };
            validate_resource_protocol(request, authorization, &finalized)
                .map_err(map_protocol_violation)?;
            finalized_uses.push(finalized);
        }

        Ok(MineralNitrogenFinalization {
            final_total_demand,
            internal_use,
            internal_remaining: self.request_batch.internal_offer - internal_use,
            final_external_demand,
            authorization_sum,
            external_use,
            finalized_uses,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MineralNitrogenFinalization {
    pub final_total_demand: f64,
    pub internal_use: f64,
    pub internal_remaining: f64,
    pub final_external_demand: f64,
    pub authorization_sum: f64,
    pub external_use: f64,
    pub finalized_uses: Vec<MineralNitrogenFinalizedUse>,
}

fn validate_configured_layers(
    stratum: &StratumConfiguration,
) -> Result<BTreeSet<MineralNitrogenKey>, NitrogenProtocolError> {
    if stratum.root_layers.is_empty() {
        return Err(NitrogenProtocolError::EmptyConfiguredLayers);
    }
    let mut layer_ids = BTreeSet::new();
    let mut fraction_sum = 0.0;
    let mut compensation = 0.0;
    for root in &stratum.root_layers {
        if !layer_ids.insert(root.layer_id.clone()) {
            return Err(NitrogenProtocolError::DuplicateConfiguredLayer);
        }
        if !root.mineral_n_root_fraction.is_finite() || root.mineral_n_root_fraction < 0.0 {
            return Err(NitrogenProtocolError::InvalidConfiguredFraction);
        }
        let adjusted = root.mineral_n_root_fraction - compensation;
        let next = fraction_sum + adjusted;
        compensation = (next - fraction_sum) - adjusted;
        fraction_sum = next;
    }
    if (fraction_sum - 1.0).abs() > CONFIGURATION_FRACTION_TOLERANCE {
        return Err(NitrogenProtocolError::ConfiguredFractionsDoNotClose);
    }

    Ok(layer_ids
        .into_iter()
        .flat_map(|layer_id| {
            [
                MineralNitrogenSpecies::Ammonium,
                MineralNitrogenSpecies::Nitrate,
            ]
            .into_iter()
            .map(move |species| MineralNitrogenKey {
                layer_id: layer_id.clone(),
                species,
            })
        })
        .collect())
}

fn validate_request_identity(
    request: &PotentialMineralNitrogenRequest,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
) -> Result<(), NitrogenProtocolError> {
    if request.transaction_id != transaction_id {
        return Err(NitrogenProtocolError::TransactionMismatch);
    }
    if &request.owner_id != owner_id {
        return Err(NitrogenProtocolError::OwnerMismatch);
    }
    if request.basis != NITROGEN_BASIS {
        return Err(NitrogenProtocolError::BasisMismatch);
    }
    validate_amount(request.amount)
}

fn validate_authorization_identity(
    authorization: &MineralNitrogenMaximumAuthorization,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
) -> Result<(), NitrogenProtocolError> {
    if authorization.transaction_id != transaction_id {
        return Err(NitrogenProtocolError::TransactionMismatch);
    }
    if &authorization.owner_id != owner_id {
        return Err(NitrogenProtocolError::OwnerMismatch);
    }
    if authorization.basis != NITROGEN_BASIS {
        return Err(NitrogenProtocolError::BasisMismatch);
    }
    validate_amount(authorization.amount)
}

fn validate_amount(amount: f64) -> Result<(), NitrogenProtocolError> {
    if !amount.is_finite() {
        return Err(NitrogenProtocolError::NonFiniteAmount);
    }
    if amount < 0.0 {
        return Err(NitrogenProtocolError::NegativeAmount);
    }
    Ok(())
}

fn compensated_sum(values: impl IntoIterator<Item = f64>) -> f64 {
    let mut sum = 0.0;
    let mut compensation = 0.0;
    for value in values {
        let adjusted = value - compensation;
        let next = sum + adjusted;
        compensation = (next - sum) - adjusted;
        sum = next;
    }
    sum
}

fn map_protocol_violation(violation: ResourceProtocolViolation) -> NitrogenProtocolError {
    match violation {
        ResourceProtocolViolation::TransactionMismatch => {
            NitrogenProtocolError::TransactionMismatch
        }
        ResourceProtocolViolation::OwnerMismatch => NitrogenProtocolError::OwnerMismatch,
        ResourceProtocolViolation::KeyMismatch => {
            NitrogenProtocolError::AuthorizationCorrespondence
        }
        ResourceProtocolViolation::BasisMismatch => NitrogenProtocolError::BasisMismatch,
        ResourceProtocolViolation::NonFinite => NitrogenProtocolError::NonFiniteAmount,
        ResourceProtocolViolation::Negative => NitrogenProtocolError::NegativeAmount,
        ResourceProtocolViolation::AuthorizationExceedsRequest => {
            NitrogenProtocolError::AuthorizationExceedsRequest
        }
        ResourceProtocolViolation::FinalizedUseExceedsAuthorization => {
            NitrogenProtocolError::FinalizedUseExceedsAuthorization
        }
        ResourceProtocolViolation::DuplicateRequestIdentity => {
            NitrogenProtocolError::DuplicateRequestIdentity
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;
    use crate::config::RootLayer;
    use openwepp_kernel_contract::SoilLayerId;

    fn owner() -> ResourceOwnerId {
        ResourceOwnerId::try_new("tree-1").expect("valid owner")
    }

    #[test]
    fn owner_identity_must_equal_exact_requesting_stratum() {
        let stratum = stratum();
        let wrong = ResourceOwnerId::try_new("another-stratum").expect("owner");
        assert_eq!(
            PotentialMineralNitrogenRequestBatch::try_from_stratum_configuration(
                TransactionId(1),
                wrong,
                &stratum,
                0.1,
                0.0,
            ),
            Err(NitrogenProtocolError::StratumOwnerMismatch)
        );
    }

    fn stratum() -> StratumConfiguration {
        let (configuration, _) = crate::transaction::v7_identity_rebound_fixture();
        configuration.strata[0].clone()
    }

    fn two_layer_stratum() -> StratumConfiguration {
        let mut value = stratum();
        value.nh4_request_fraction = 0.4;
        value.root_layers = vec![
            RootLayer {
                layer_id: SoilLayerId::try_new("soil-1").expect("layer"),
                root_fraction: 0.25,
                mineral_n_root_fraction: 0.25,
                lateral_root_length_m: 0.001,
            },
            RootLayer {
                layer_id: SoilLayerId::try_new("soil-2").expect("layer"),
                root_fraction: 0.75,
                mineral_n_root_fraction: 0.75,
                lateral_root_length_m: 0.002,
            },
        ];
        value
    }

    fn batch(potential_demand: f64, internal_offer: f64) -> PotentialMineralNitrogenRequestBatch {
        PotentialMineralNitrogenRequestBatch::try_from_stratum_configuration(
            TransactionId(11),
            owner(),
            &two_layer_stratum(),
            potential_demand,
            internal_offer,
        )
        .expect("request batch")
    }

    fn full_authorizations(
        batch: &PotentialMineralNitrogenRequestBatch,
    ) -> Vec<MineralNitrogenMaximumAuthorization> {
        batch
            .requests()
            .iter()
            .map(|request| MineralNitrogenMaximumAuthorization {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount: request.amount,
                basis: request.basis,
            })
            .collect()
    }

    fn assert_identity_and_bounds(
        batch: &PotentialMineralNitrogenRequestBatch,
        validated: &ValidatedMineralNitrogenAuthorizations,
        finalization: &MineralNitrogenFinalization,
    ) {
        assert_eq!(finalization.finalized_uses.len(), batch.requests().len());
        for ((request, finalized), authorization) in batch
            .requests()
            .iter()
            .zip(&finalization.finalized_uses)
            .map(|pair| {
                let authorization = &validated.authorizations()[&pair.0.key];
                (pair, authorization)
            })
        {
            assert_eq!(finalized.transaction_id, request.transaction_id);
            assert_eq!(finalized.owner_id, request.owner_id);
            assert_eq!(finalized.key, request.key);
            assert_eq!(finalized.basis, request.basis);
            assert!(finalized.amount <= authorization.amount);
            assert!(authorization.amount <= request.amount);
        }
    }

    fn finalize_full(
        potential_demand: f64,
        final_demand: f64,
        internal_offer: f64,
    ) -> (
        PotentialMineralNitrogenRequestBatch,
        ValidatedMineralNitrogenAuthorizations,
        MineralNitrogenFinalization,
    ) {
        let request_batch = batch(potential_demand, internal_offer);
        let request_snapshot = request_batch.requests().to_vec();
        let validated = ValidatedMineralNitrogenAuthorizations::try_new(
            &request_batch,
            full_authorizations(&request_batch),
        )
        .expect("full authorizations");
        let finalization = validated
            .finalize(final_demand)
            .expect("canonical finalization");
        assert_eq!(request_batch.requests(), request_snapshot);
        assert_eq!(
            finalization.final_total_demand.to_bits(),
            final_demand.to_bits()
        );
        assert_identity_and_bounds(&request_batch, &validated, &finalization);
        (request_batch, validated, finalization)
    }

    #[test]
    fn potential_requests_apply_internal_retranslocation_then_split_layer_and_species() {
        let request_batch = batch(8.0, 2.0);
        approx::assert_abs_diff_eq!(request_batch.potential_internal_use(), 2.0);
        approx::assert_abs_diff_eq!(request_batch.potential_external_shortfall(), 6.0);
        let amounts = request_batch
            .requests()
            .iter()
            .map(|request| {
                (
                    (request.key.layer_id.as_str(), request.key.species),
                    request.amount,
                )
            })
            .collect::<BTreeMap<_, _>>();
        approx::assert_abs_diff_eq!(
            amounts[&("soil-1", MineralNitrogenSpecies::Ammonium)],
            0.6,
            epsilon = 1.0e-15
        );
        approx::assert_abs_diff_eq!(
            amounts[&("soil-1", MineralNitrogenSpecies::Nitrate)],
            0.9,
            epsilon = 1.0e-15
        );
        approx::assert_abs_diff_eq!(
            amounts[&("soil-2", MineralNitrogenSpecies::Ammonium)],
            1.8,
            epsilon = 1.0e-15
        );
        approx::assert_abs_diff_eq!(
            amounts[&("soil-2", MineralNitrogenSpecies::Nitrate)],
            2.7,
            epsilon = 1.0e-15
        );
        assert!(request_batch.requests().iter().all(|request| {
            request.transaction_id == TransactionId(11)
                && request.owner_id == owner()
                && request.basis == NITROGEN_BASIS
        }));
    }

    #[test]
    fn internal_offer_satisfying_demand_still_emits_complete_zero_request_set() {
        let request_batch = batch(1.0, 2.0);
        assert_eq!(request_batch.requests().len(), 4);
        assert!(
            request_batch
                .requests()
                .iter()
                .all(|request| request.amount == 0.0)
        );
        let validated = ValidatedMineralNitrogenAuthorizations::try_new(
            &request_batch,
            full_authorizations(&request_batch),
        )
        .expect("zero authorizations");
        let finalization = validated.finalize(0.5).expect("zero external use");
        approx::assert_abs_diff_eq!(finalization.internal_use, 0.5);
        assert_eq!(finalization.external_use.to_bits(), 0.0_f64.to_bits());
        assert!(
            finalization
                .finalized_uses
                .iter()
                .all(|value| value.amount == 0.0)
        );
    }

    #[test]
    fn final_use_is_proportional_to_authorization_and_unused_authorization_is_not_use() {
        let request_batch = batch(10.0, 2.0);
        let mut authorizations = full_authorizations(&request_batch);
        let scales = [0.5, 0.5, 0.5, 0.5];
        for (authorization, scale) in authorizations.iter_mut().zip(scales) {
            authorization.amount *= scale;
        }
        let validated =
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, authorizations)
                .expect("partial authorization");
        let finalization = validated.finalize(4.0).expect("finalization");
        approx::assert_abs_diff_eq!(finalization.internal_use, 2.0);
        approx::assert_abs_diff_eq!(finalization.final_external_demand, 2.0);
        approx::assert_abs_diff_eq!(finalization.authorization_sum, 4.0);
        approx::assert_abs_diff_eq!(finalization.external_use, 2.0);
        for finalized in &finalization.finalized_uses {
            let authorization = &validated.authorizations()[&finalized.key];
            approx::assert_abs_diff_eq!(finalized.amount, authorization.amount * 0.5);
            assert!(finalized.amount < authorization.amount);
            assert_eq!(finalized.transaction_id, TransactionId(11));
            assert_eq!(finalized.owner_id, owner());
            assert_eq!(finalized.basis, NITROGEN_BASIS);
        }
    }

    #[test]
    fn full_final_demand_uses_exact_partial_authorizations() {
        let request_batch = batch(10.0, 2.0);
        let mut authorizations = full_authorizations(&request_batch);
        for authorization in &mut authorizations {
            authorization.amount *= 0.25;
        }
        let validated =
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, authorizations)
                .expect("partial authorization");
        let finalization = validated.finalize(10.0).expect("finalization");
        approx::assert_abs_diff_eq!(finalization.external_use, 2.0);
        for finalized in &finalization.finalized_uses {
            approx::assert_abs_diff_eq!(
                finalized.amount,
                validated.authorizations()[&finalized.key].amount
            );
        }
    }

    #[test]
    fn final_demand_equal_below_and_one_ulp_above_potential_preserve_requests() {
        let potential = 0.125_f64;
        for final_demand in [potential, 0.0625, f64::from_bits(potential.to_bits() + 1)] {
            let (batch, validated, finalization) = finalize_full(potential, final_demand, 0.0);
            let expected_external = final_demand.min(
                validated
                    .authorizations()
                    .values()
                    .map(|authorization| authorization.amount)
                    .sum(),
            );
            assert_eq!(
                finalization.external_use.to_bits(),
                expected_external.to_bits()
            );
            assert_eq!(
                batch.potential_total_demand().to_bits(),
                potential.to_bits()
            );
        }
    }

    #[test]
    fn observed_two_ulp_final_demand_uses_immutable_potential_request_batch() {
        let potential = f64::from_bits(4_546_826_747_422_758_608);
        let final_demand = f64::from_bits(4_546_826_747_422_758_610);
        assert_eq!(potential, 0.000_097_555_468_386_935_4);
        assert_eq!(final_demand, 0.000_097_555_468_386_935_42);
        let (batch, validated, finalization) = finalize_full(potential, final_demand, 0.0);
        assert_eq!(
            batch.potential_total_demand().to_bits(),
            potential.to_bits()
        );
        assert_eq!(
            finalization.final_total_demand.to_bits(),
            final_demand.to_bits()
        );
        assert_eq!(finalization.external_use.to_bits(), potential.to_bits());
        assert!(finalization.external_use < finalization.final_total_demand);
        assert_eq!(
            finalization.authorization_sum.to_bits(),
            potential.to_bits()
        );
        assert_identity_and_bounds(&batch, &validated, &finalization);
    }

    #[test]
    fn materially_greater_final_demand_is_bounded_by_potential_authorization() {
        let (batch, validated, finalization) = finalize_full(1.0, 3.0, 0.0);
        assert_eq!(finalization.final_total_demand, 3.0);
        assert_eq!(finalization.authorization_sum, 1.0);
        assert_eq!(finalization.external_use, 1.0);
        assert_eq!(finalization.final_external_demand, 3.0);
        assert_identity_and_bounds(&batch, &validated, &finalization);
    }

    #[test]
    fn zero_authorization_finalizes_zero_for_every_layer_and_species() {
        let request_batch = batch(8.0, 2.0);
        let zero = request_batch
            .requests()
            .iter()
            .map(|request| MineralNitrogenMaximumAuthorization {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount: 0.0,
                basis: request.basis,
            })
            .collect();
        let validated = ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, zero)
            .expect("zero authorizations");
        let finalization = validated.finalize(9.0).expect("zero finalization");
        assert_eq!(finalization.internal_use, 2.0);
        assert_eq!(finalization.external_use, 0.0);
        assert!(
            finalization
                .finalized_uses
                .iter()
                .all(|use_| use_.amount == 0.0)
        );
        assert_identity_and_bounds(&request_batch, &validated, &finalization);
    }

    #[test]
    fn internal_n_full_and_partial_branches_preserve_external_finalization_identity() {
        let (_, _, full_internal) = finalize_full(1.0, 0.5, 2.0);
        assert_eq!(full_internal.internal_use, 0.5);
        assert_eq!(full_internal.internal_remaining, 1.5);
        assert_eq!(full_internal.external_use, 0.0);

        let (_, _, partial_internal) = finalize_full(8.0, 5.0, 2.0);
        assert_eq!(partial_internal.internal_use, 2.0);
        assert_eq!(partial_internal.internal_remaining, 0.0);
        assert_eq!(partial_internal.final_external_demand, 3.0);
        assert_eq!(partial_internal.external_use, 3.0);
    }

    #[test]
    fn authorization_rejects_wrong_layer_species_missing_duplicate_and_basis() {
        let request_batch = batch(8.0, 2.0);
        let complete = full_authorizations(&request_batch);

        let mut wrong_layer = complete.clone();
        wrong_layer[0].key.layer_id = SoilLayerId::try_new("soil-x").expect("layer");
        assert_eq!(
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, wrong_layer),
            Err(NitrogenProtocolError::AuthorizationCorrespondence)
        );

        let mut wrong_species = complete.clone();
        wrong_species[0].key.species = MineralNitrogenSpecies::Nitrate;
        assert_eq!(
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, wrong_species),
            Err(NitrogenProtocolError::DuplicateAuthorizationIdentity)
        );

        let mut missing = complete.clone();
        missing.pop();
        assert_eq!(
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, missing),
            Err(NitrogenProtocolError::AuthorizationCorrespondence)
        );

        let mut duplicate = complete.clone();
        duplicate.push(complete[0].clone());
        assert_eq!(
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, duplicate),
            Err(NitrogenProtocolError::DuplicateAuthorizationIdentity)
        );

        let mut wrong_basis = complete;
        wrong_basis[0].basis = ResourceAmountBasis::WaterKgPerSquareMeterInterval;
        assert_eq!(
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, wrong_basis),
            Err(NitrogenProtocolError::BasisMismatch)
        );
    }

    #[test]
    fn authorization_rejects_wrong_transaction_and_owner_identity() {
        let request_batch = batch(8.0, 2.0);

        let mut wrong_transaction = full_authorizations(&request_batch);
        wrong_transaction[0].transaction_id = TransactionId(12);
        assert_eq!(
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, wrong_transaction),
            Err(NitrogenProtocolError::TransactionMismatch)
        );

        let mut wrong_owner = full_authorizations(&request_batch);
        wrong_owner[0].owner_id = ResourceOwnerId::try_new("tree-2").expect("owner");
        assert_eq!(
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, wrong_owner),
            Err(NitrogenProtocolError::OwnerMismatch)
        );
    }

    #[test]
    fn request_and_authorization_domain_guards_are_fail_closed() {
        assert_eq!(
            PotentialMineralNitrogenRequestBatch::try_from_stratum_configuration(
                TransactionId(11),
                owner(),
                &two_layer_stratum(),
                f64::NAN,
                0.0,
            ),
            Err(NitrogenProtocolError::NonFiniteAmount)
        );

        let request_batch = batch(8.0, 2.0);
        let mut excessive = full_authorizations(&request_batch);
        excessive[0].amount += 1.0;
        assert_eq!(
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, excessive),
            Err(NitrogenProtocolError::AuthorizationExceedsRequest)
        );

        let validated = ValidatedMineralNitrogenAuthorizations::try_new(
            &request_batch,
            full_authorizations(&request_batch),
        )
        .expect("authorization");
        assert_eq!(
            validated.finalize(f64::NAN),
            Err(NitrogenProtocolError::NonFiniteAmount)
        );
        assert_eq!(
            validated.finalize(-1.0),
            Err(NitrogenProtocolError::NegativeAmount)
        );
        let finalization = validated.finalize(9.0).expect("canonical finalization");
        assert_eq!(finalization.final_total_demand, 9.0);
        assert_eq!(finalization.external_use, 6.0);
    }

    #[test]
    fn nitrogen_boundary_categories_retain_canonical_transaction_codes() {
        let request_batch = batch(8.0, 2.0);
        let mut wrong_owner = full_authorizations(&request_batch);
        wrong_owner[0].owner_id = ResourceOwnerId::try_new("wrong").unwrap();
        let identity = ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, wrong_owner)
            .unwrap_err();
        assert!(matches!(
            crate::VegetationError::from(identity),
            crate::VegetationError::ResourceIdentity(_)
        ));

        let operand = PotentialMineralNitrogenRequestBatch::try_from_stratum_configuration(
            TransactionId(11),
            owner(),
            &two_layer_stratum(),
            f64::NAN,
            0.0,
        )
        .unwrap_err();
        assert!(matches!(
            crate::VegetationError::from(operand),
            crate::VegetationError::ResourceOperand(_)
        ));

        let mut excessive = full_authorizations(&request_batch);
        excessive[0].amount = request_batch.requests()[0].amount + 1.0;
        let authorization_bound =
            ValidatedMineralNitrogenAuthorizations::try_new(&request_batch, excessive).unwrap_err();
        assert!(matches!(
            crate::VegetationError::from(authorization_bound),
            crate::VegetationError::ResourceBound(_)
        ));

        let request = &request_batch.requests()[0];
        let authorization = &full_authorizations(&request_batch)[0];
        let finalized = MineralNitrogenFinalizedUse {
            transaction_id: request.transaction_id,
            owner_id: request.owner_id.clone(),
            key: request.key.clone(),
            amount: f64::from_bits(authorization.amount.to_bits() + 1),
            basis: request.basis,
        };
        let finalized_bound = validate_resource_protocol(request, authorization, &finalized)
            .map_err(map_protocol_violation)
            .unwrap_err();
        assert!(matches!(
            crate::VegetationError::from(finalized_bound),
            crate::VegetationError::ResourceBound(_)
        ));
    }
}
