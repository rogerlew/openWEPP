//! Identity separation for the independent surface-liquid arithmetic preflight.

use std::collections::BTreeSet;

use super::{
    DirectSurfaceLiquidClosureOperands, DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError,
    DirectSurfaceLiquidErrorCode, contextual_ofe_comparison_failure,
};

pub(super) fn partition_inputs_are_projectable(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
) -> bool {
    let actual = operands
        .partition_inputs
        .iter()
        .map(|row| row.ofe_id.clone())
        .collect::<BTreeSet<_>>();
    let expected = configuration
        .ofe_topology
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    operands.partition_inputs.len() == configuration.ofe_topology.len() && actual == expected
}

pub(super) fn validate_partition_input_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    let actual_ids = operands
        .partition_inputs
        .iter()
        .map(|row| row.ofe_id.clone())
        .collect::<Vec<_>>();
    if actual_ids == configuration.ofe_topology {
        return Ok(());
    }
    let actual_set = actual_ids.iter().cloned().collect::<BTreeSet<_>>();
    let expected_set = configuration
        .ofe_topology
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let identity = configuration
        .ofe_topology
        .iter()
        .find(|expected| !actual_set.contains(*expected))
        .cloned()
        .or_else(|| {
            actual_ids
                .iter()
                .find(|actual| !expected_set.contains(*actual))
                .cloned()
        })
        .or_else(|| {
            actual_ids
                .iter()
                .zip(&configuration.ofe_topology)
                .find(|(actual, expected)| actual != expected)
                .map(|(actual, _)| actual.clone())
        })
        .or_else(|| actual_ids.get(configuration.ofe_topology.len()).cloned())
        .or_else(|| configuration.ofe_topology.first().cloned())
        .ok_or(DirectSurfaceLiquidError::Closure("empty OFE topology"))?;
    Err(contextual_ofe_comparison_failure(
        DirectSurfaceLiquidErrorCode::E009,
        operands.transaction_id,
        &configuration.owner_id,
        &identity,
        "frozen partition-input membership/order",
    ))
}
