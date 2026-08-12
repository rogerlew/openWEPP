//! Dependency-neutral resource arbitration transaction types.

use core::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceRequest<K, Q> {
    pub transaction_id: TransactionId,
    pub owner_id: String,
    pub key: K,
    pub amount: Q,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaximumAuthorization<K, Q> {
    pub transaction_id: TransactionId,
    pub owner_id: String,
    pub key: K,
    pub amount: Q,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalizedUse<K, Q> {
    pub transaction_id: TransactionId,
    pub owner_id: String,
    pub key: K,
    pub amount: Q,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
    NonFinite,
    Negative,
    AuthorizationExceedsRequest,
    FinalizedUseExceedsAuthorization,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_enforces_ordered_amounts() {
        let request = ResourceRequest {
            transaction_id: TransactionId(1),
            owner_id: "tree".into(),
            key: 2_u8,
            amount: 3.0,
        };
        let authorization = MaximumAuthorization {
            transaction_id: TransactionId(1),
            owner_id: "tree".into(),
            key: 2_u8,
            amount: 2.0,
        };
        let finalized = FinalizedUse {
            transaction_id: TransactionId(1),
            owner_id: "tree".into(),
            key: 2_u8,
            amount: 1.0,
        };
        assert_eq!(
            validate_resource_protocol(&request, &authorization, &finalized),
            Ok(())
        );
    }
}
