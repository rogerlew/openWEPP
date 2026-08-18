use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};

use super::{
    DirectSurfaceLiquidClosureOperands, DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError,
    DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidOwnedState,
    DirectSurfaceLiquidPhase, DirectSurfaceLiquidRollbackHashes,
};

pub(super) fn ending_aggregate_failure(
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E010,
        DirectSurfaceLiquidPhase::IndependentClosure,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: Some(owner_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

pub(super) fn validate_projected_ending_digest(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    ending: &DirectSurfaceLiquidOwnedState,
) -> Result<(), DirectSurfaceLiquidError> {
    let aggregate_failure =
        |detail| ending_aggregate_failure(operands.transaction_id, &configuration.owner_id, detail);
    let recomputed = ending
        .recomputed_sha256()
        .map_err(|_| aggregate_failure("projected ending-state digest reconstruction"))?;
    if ending.state_sha256 != recomputed {
        return Err(aggregate_failure("projected ending-state digest join"));
    }
    ending
        .validate(configuration)
        .map_err(|_| aggregate_failure("projected ending-state complete validation"))
}

pub(super) fn first_membership_aware_mismatch<T: Clone + Ord>(
    actual: &[T],
    expected: &[T],
) -> Option<T> {
    match actual.len().cmp(&expected.len()) {
        std::cmp::Ordering::Less => expected
            .iter()
            .find(|row| {
                actual.iter().filter(|actual| *actual == *row).count()
                    < expected.iter().filter(|expected| *expected == *row).count()
            })
            .cloned(),
        std::cmp::Ordering::Greater => actual
            .iter()
            .enumerate()
            .find(|(index, row)| {
                actual[..=*index]
                    .iter()
                    .filter(|actual| *actual == *row)
                    .count()
                    > expected.iter().filter(|expected| *expected == *row).count()
            })
            .map(|(_, row)| row.clone()),
        std::cmp::Ordering::Equal => actual
            .iter()
            .zip(expected)
            .find(|(actual, expected)| actual != expected)
            .map(|(actual, _)| actual.clone()),
    }
}
