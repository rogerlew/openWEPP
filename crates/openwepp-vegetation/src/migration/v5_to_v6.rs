//! Exact V5-to-V6 identity transition for rollback snapshots and diagnostics.

use std::collections::BTreeSet;

use openwepp_kernel_contract::TransactionId;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{V5_MODEL_SHA256, identity_only_configuration_payload_matches, valid_historical_state};
use crate::diagnostics::{CoupledSolvePass, NumericalFailureDiagnostics, SolveIdentity};
use crate::error::NumericalFailureCategory;
use crate::{CoupledOwnedState, MODEL_SHA256, VegetationConfiguration};

/// Numerical failure payload plus every external identity needed to prove its
/// configuration, rollback state, and attempted-transaction lineage.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityBoundNumericalFailureDiagnostics {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub beginning_state_sha256: String,
    pub failure_category: NumericalFailureCategory,
    pub diagnostics_sha256: String,
    pub diagnostics: NumericalFailureDiagnostics,
}

impl IdentityBoundNumericalFailureDiagnostics {
    /// Returns the digest of the complete envelope with only its own digest
    /// member excluded.
    pub fn canonical_sha256(&self) -> Result<String, serde_json::Error> {
        let mut canonical = self.clone();
        canonical.diagnostics_sha256.clear();
        serde_json::to_vec(&canonical).map(|bytes| format!("{:x}", Sha256::digest(bytes)))
    }
}

/// Complete candidate emitted only after every V5 receipt has passed.
#[derive(Clone, Debug, PartialEq)]
pub struct V5ToV6Migration {
    pub configuration: VegetationConfiguration,
    pub initial_state: CoupledOwnedState,
    pub state: CoupledOwnedState,
    pub diagnostics: IdentityBoundNumericalFailureDiagnostics,
}

/// Fail-closed precedence for the exact V5-to-V6 snapshot transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum V5ToV6MigrationError {
    InvalidV5ConfigurationIdentity,
    InvalidV5ConfigurationDigest,
    InvalidV5Configuration,
    InvalidV5InitialStateIdentity,
    InvalidV5InitialStateDigest,
    InvalidV5InitialStateLineage,
    InvalidV5StateIdentity,
    InvalidV5StateDigest,
    InvalidV5StateLineage,
    InvalidV5DiagnosticIdentity,
    InvalidV5DiagnosticDigest,
    InvalidV5DiagnosticLineage,
    InvalidV5DiagnosticPayload,
    InvalidV6Configuration,
    ConfigurationPayloadMismatch,
    InvalidV6InitialStateReceipt,
    TargetInitialStateRejected,
    TargetStateRejected,
    TargetDiagnosticRejected,
}

/// Migrates one complete V5 rollback snapshot and rejected-failure diagnostic.
///
/// Validation is completed in the error-variant order before any candidate is
/// assembled. Configuration, initial-state, current-state, and diagnostic
/// scientific payloads are cloned byte-for-byte; only their explicit identity
/// members are rebound and independently re-digested for V6.
pub fn migrate_v5_snapshot(
    source_configuration: &VegetationConfiguration,
    source_initial_state: &CoupledOwnedState,
    source_state: &CoupledOwnedState,
    source_diagnostics: &IdentityBoundNumericalFailureDiagnostics,
    target_configuration: &VegetationConfiguration,
) -> Result<V5ToV6Migration, V5ToV6MigrationError> {
    validate_source_configuration(source_configuration)?;
    validate_source_initial_state(source_configuration, source_initial_state)?;
    validate_source_state(source_configuration, source_initial_state, source_state)?;
    validate_source_diagnostics(source_configuration, source_state, source_diagnostics)?;

    if target_configuration.validate().is_err() {
        return Err(V5ToV6MigrationError::InvalidV6Configuration);
    }
    if !identity_only_configuration_payload_matches(source_configuration, target_configuration) {
        return Err(V5ToV6MigrationError::ConfigurationPayloadMismatch);
    }

    let initial_state = rebind_state(source_initial_state, target_configuration)
        .map_err(|_| V5ToV6MigrationError::TargetInitialStateRejected)?;
    if target_configuration.initial_state_sha256 != initial_state.state_sha256 {
        return Err(V5ToV6MigrationError::InvalidV6InitialStateReceipt);
    }
    let derived_configuration = derive_target_configuration(source_configuration, &initial_state)
        .map_err(|_| V5ToV6MigrationError::InvalidV6Configuration)?;
    if &derived_configuration != target_configuration
        || derived_configuration.configuration_sha256 == source_configuration.configuration_sha256
    {
        return Err(V5ToV6MigrationError::InvalidV6Configuration);
    }
    initial_state
        .validate(target_configuration)
        .map_err(|_| V5ToV6MigrationError::TargetInitialStateRejected)?;

    let state = rebind_state(source_state, target_configuration)
        .map_err(|_| V5ToV6MigrationError::TargetStateRejected)?;
    state
        .validate(target_configuration)
        .map_err(|_| V5ToV6MigrationError::TargetStateRejected)?;

    let diagnostics = rebind_diagnostics(source_diagnostics, &state, target_configuration)
        .map_err(|_| V5ToV6MigrationError::TargetDiagnosticRejected)?;
    validate_v6_diagnostics(target_configuration, &state, &diagnostics)
        .map_err(|()| V5ToV6MigrationError::TargetDiagnosticRejected)?;

    Ok(V5ToV6Migration {
        configuration: derived_configuration,
        initial_state,
        state,
        diagnostics,
    })
}

fn derive_target_configuration(
    source: &VegetationConfiguration,
    initial_state: &CoupledOwnedState,
) -> Result<VegetationConfiguration, crate::VegetationError> {
    let mut target = source.clone();
    target.model_definition_sha256 = MODEL_SHA256.into();
    target.configuration_sha256.clear();
    target.configuration_sha256 = target.canonical_sha256()?;
    target
        .initial_state_sha256
        .clone_from(&initial_state.state_sha256);
    target.validate()?;
    Ok(target)
}

fn validate_source_configuration(
    configuration: &VegetationConfiguration,
) -> Result<(), V5ToV6MigrationError> {
    if configuration.model_definition_sha256 != V5_MODEL_SHA256 {
        return Err(V5ToV6MigrationError::InvalidV5ConfigurationIdentity);
    }
    if configuration.canonical_sha256().ok().as_ref() != Some(&configuration.configuration_sha256) {
        return Err(V5ToV6MigrationError::InvalidV5ConfigurationDigest);
    }
    let mut rebound = configuration.clone();
    rebound.model_definition_sha256 = MODEL_SHA256.into();
    rebound.configuration_sha256.clear();
    rebound.configuration_sha256 = rebound
        .canonical_sha256()
        .map_err(|_| V5ToV6MigrationError::InvalidV5Configuration)?;
    rebound
        .validate()
        .map_err(|_| V5ToV6MigrationError::InvalidV5Configuration)
}

fn validate_source_initial_state(
    configuration: &VegetationConfiguration,
    initial_state: &CoupledOwnedState,
) -> Result<(), V5ToV6MigrationError> {
    if initial_state.model_definition_sha256 != V5_MODEL_SHA256
        || initial_state.configuration_sha256 != configuration.configuration_sha256
    {
        return Err(V5ToV6MigrationError::InvalidV5InitialStateIdentity);
    }
    if initial_state.canonical_sha256().ok().as_ref() != Some(&initial_state.state_sha256) {
        return Err(V5ToV6MigrationError::InvalidV5InitialStateDigest);
    }
    if initial_state.last_transaction_id != 0
        || configuration.initial_state_sha256 != initial_state.state_sha256
        || !valid_historical_state(initial_state, configuration, V5_MODEL_SHA256)
    {
        return Err(V5ToV6MigrationError::InvalidV5InitialStateLineage);
    }
    Ok(())
}

/// Shares exact V5 intake validation with identity-rebound crate tests so
/// historical fixture drift cannot be hidden by direct identity replacement.
#[cfg(test)]
pub(crate) fn validate_v5_initial_fixture(
    configuration: &VegetationConfiguration,
    initial_state: &CoupledOwnedState,
) -> Result<(), V5ToV6MigrationError> {
    validate_source_configuration(configuration)?;
    validate_source_initial_state(configuration, initial_state)
}

fn validate_source_state(
    configuration: &VegetationConfiguration,
    initial_state: &CoupledOwnedState,
    state: &CoupledOwnedState,
) -> Result<(), V5ToV6MigrationError> {
    if state.model_definition_sha256 != V5_MODEL_SHA256
        || state.configuration_sha256 != configuration.configuration_sha256
    {
        return Err(V5ToV6MigrationError::InvalidV5StateIdentity);
    }
    if state.canonical_sha256().ok().as_ref() != Some(&state.state_sha256) {
        return Err(V5ToV6MigrationError::InvalidV5StateDigest);
    }
    let transaction_zero_mismatch = state.last_transaction_id == 0
        && serde_json::to_vec(state).ok() != serde_json::to_vec(initial_state).ok();
    if transaction_zero_mismatch || !valid_historical_state(state, configuration, V5_MODEL_SHA256) {
        return Err(V5ToV6MigrationError::InvalidV5StateLineage);
    }
    Ok(())
}

fn validate_source_diagnostics(
    configuration: &VegetationConfiguration,
    state: &CoupledOwnedState,
    envelope: &IdentityBoundNumericalFailureDiagnostics,
) -> Result<(), V5ToV6MigrationError> {
    if envelope.model_definition_sha256 != V5_MODEL_SHA256
        || envelope.configuration_sha256 != configuration.configuration_sha256
        || envelope.beginning_state_sha256 != state.state_sha256
        || envelope.diagnostics.model_definition_sha256 != V5_MODEL_SHA256
    {
        return Err(V5ToV6MigrationError::InvalidV5DiagnosticIdentity);
    }
    if envelope.canonical_sha256().ok().as_ref() != Some(&envelope.diagnostics_sha256) {
        return Err(V5ToV6MigrationError::InvalidV5DiagnosticDigest);
    }
    validate_diagnostic_lineage(configuration, state, &envelope.diagnostics)
        .map_err(|()| V5ToV6MigrationError::InvalidV5DiagnosticLineage)?;
    validate_failure_category(envelope.failure_category, &envelope.diagnostics)
        .map_err(|()| V5ToV6MigrationError::InvalidV5DiagnosticPayload)?;
    let mut rebound = envelope.diagnostics.clone();
    rebound.model_definition_sha256 = MODEL_SHA256.into();
    rebound
        .validate()
        .map_err(|_| V5ToV6MigrationError::InvalidV5DiagnosticPayload)
}

fn validate_failure_category(
    category: NumericalFailureCategory,
    diagnostics: &NumericalFailureDiagnostics,
) -> Result<(), ()> {
    // V6 admits portability and identity-only migration for exactly one
    // rejected-failure seam. Other numerical failure categories remain typed
    // runtime outcomes, but they have no V5-to-V6 evidence-migration authority.
    if category != NumericalFailureCategory::BacktrackingLimit
        || diagnostics.pass != CoupledSolvePass::Capped
        || diagnostics.solve != SolveIdentity::HydraulicSystem
        || diagnostics.backtracking_count == 0
        || !diagnostics
            .step_norm
            .is_some_and(|value| value.is_finite() && value >= 0.0)
    {
        return Err(());
    }
    Ok(())
}

fn validate_diagnostic_lineage(
    configuration: &VegetationConfiguration,
    state: &CoupledOwnedState,
    diagnostics: &NumericalFailureDiagnostics,
) -> Result<(), ()> {
    let expected_transaction = state.last_transaction_id.checked_add(1).ok_or(())?;
    if diagnostics.transaction_id != TransactionId(expected_transaction)
        || diagnostics.pass != CoupledSolvePass::Capped
        || !state.occupancies.contains_key(&diagnostics.occupancy_id)
    {
        return Err(());
    }
    let fixed = diagnostics
        .fixed_authorization_identity
        .as_ref()
        .ok_or(())?;
    if fixed.transaction_id != diagnostics.transaction_id
        || fixed.occupancy_id != diagnostics.occupancy_id
        || diagnostics.capped_operands.is_none()
    {
        return Err(());
    }
    let stratum = configuration
        .strata
        .iter()
        .find(|item| item.stratum_id == diagnostics.occupancy_id.stratum_id)
        .ok_or(())?;
    let configured_layers = stratum
        .root_layers
        .iter()
        .map(|layer| layer.layer_id.clone())
        .collect::<Vec<_>>();
    let operands = diagnostics.capped_operands.as_ref().ok_or(())?;
    if operands
        .layers
        .iter()
        .map(|layer| &layer.layer_id)
        .ne(configured_layers.iter())
    {
        return Err(());
    }
    let active = diagnostics
        .active_water_caps
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if active.len() != diagnostics.active_water_caps.len()
        || diagnostics
            .active_water_caps
            .iter()
            .any(|id| !configured_layers.contains(id))
        || diagnostics
            .residual_norms
            .iter()
            .any(|residual| residual.identity.is_empty())
    {
        return Err(());
    }
    Ok(())
}

fn rebind_state(
    source: &CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
) -> Result<CoupledOwnedState, serde_json::Error> {
    let mut target = source.clone();
    target.model_definition_sha256 = MODEL_SHA256.into();
    target
        .configuration_sha256
        .clone_from(&target_configuration.configuration_sha256);
    target.state_sha256.clear();
    target.state_sha256 = target
        .canonical_sha256()
        .map_err(|error| serde_json::Error::io(std::io::Error::other(error.to_string())))?;
    Ok(target)
}

fn rebind_diagnostics(
    source: &IdentityBoundNumericalFailureDiagnostics,
    target_state: &CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
) -> Result<IdentityBoundNumericalFailureDiagnostics, serde_json::Error> {
    let mut target = source.clone();
    target.model_definition_sha256 = MODEL_SHA256.into();
    target
        .configuration_sha256
        .clone_from(&target_configuration.configuration_sha256);
    target
        .beginning_state_sha256
        .clone_from(&target_state.state_sha256);
    target.diagnostics.model_definition_sha256 = MODEL_SHA256.into();
    target.diagnostics_sha256.clear();
    target.diagnostics_sha256 = target.canonical_sha256()?;
    Ok(target)
}

fn validate_v6_diagnostics(
    configuration: &VegetationConfiguration,
    state: &CoupledOwnedState,
    diagnostics: &IdentityBoundNumericalFailureDiagnostics,
) -> Result<(), ()> {
    if diagnostics.model_definition_sha256 != MODEL_SHA256
        || diagnostics.configuration_sha256 != configuration.configuration_sha256
        || diagnostics.beginning_state_sha256 != state.state_sha256
        || diagnostics.canonical_sha256().ok().as_ref() != Some(&diagnostics.diagnostics_sha256)
        || diagnostics.diagnostics.validate().is_err()
    {
        return Err(());
    }
    validate_failure_category(diagnostics.failure_category, &diagnostics.diagnostics)?;
    validate_diagnostic_lineage(configuration, state, &diagnostics.diagnostics)
}

#[cfg(test)]
mod tests;
