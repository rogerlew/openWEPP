//! Exact V6-to-V7 identity transition for the storage-transfer amendment.

use crate::carbon_nitrogen::Tissue;
use crate::config::PhenologyType;
use crate::{CoupledOwnedState, MODEL_SHA256, VegetationConfiguration};
use serde::Serialize;

use super::{V6_MODEL_SHA256, identity_only_configuration_payload_matches};

const TISSUES: [Tissue; 6] = [
    Tissue::Leaf,
    Tissue::FineRoot,
    Tissue::LiveStem,
    Tissue::DeadStem,
    Tissue::LiveCoarseRoot,
    Tissue::DeadCoarseRoot,
];

/// Storage/transfer subpool identity used by an exhaustive evergreen report.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V6ToV7PoolIdentity {
    Storage,
    Transfer,
}

/// Element identity used by an exhaustive evergreen report.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V6ToV7ElementIdentity {
    Carbon,
    Nitrogen,
}

/// One exact V7 semantic field that cannot be normalized during migration.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "field")]
pub enum V6ToV7MigrationField {
    CurrentGrowthFraction,
    TissuePool {
        tissue: Tissue,
        pool: V6ToV7PoolIdentity,
        element: V6ToV7ElementIdentity,
    },
}

/// One deterministic unresolved field for one evergreen stratum.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct V6ToV7UnresolvedField {
    pub stratum_id: openwepp_kernel_contract::StratumId,
    pub field: V6ToV7MigrationField,
}

/// Exhaustive semantic report. No partial V7 candidate accompanies it.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct V6ToV7MigrationReport {
    pub source_model_definition_sha256: String,
    pub target_model_definition_sha256: String,
    pub unresolved: Vec<V6ToV7UnresolvedField>,
}

/// Complete identity-rebound configuration and rollback state pair.
#[derive(Clone, Debug, PartialEq)]
pub struct V6ToV7Migration {
    pub configuration: VegetationConfiguration,
    pub initial_state: CoupledOwnedState,
    pub state: CoupledOwnedState,
}

/// V7 semantic violations are data, not a partially successful migration.
#[derive(Clone, Debug, PartialEq)]
pub enum V6ToV7MigrationResult {
    Complete(Box<V6ToV7Migration>),
    Incomplete(V6ToV7MigrationReport),
}

/// Fail-closed validation precedence for the exact V6-to-V7 transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum V6ToV7MigrationError {
    InvalidV6Configuration,
    InvalidV6InitialStateIdentity,
    InvalidV6InitialStateDigest,
    InvalidV6InitialStateLineage,
    InvalidV6StateIdentity,
    InvalidV6StateDigest,
    InvalidV6StateLineage,
    InvalidV7Configuration,
    ConfigurationPayloadMismatch,
    InvalidV7InitialStateReceipt,
    TargetInitialStateRejected,
    TargetStateRejected,
}

/// Migrates a complete V6 configuration, initial state, and current state.
///
/// The operation validates all V6 identity, digest, and transaction-lineage
/// receipts before inspecting the V7 evergreen posture. Seasonal-deciduous
/// scientific/state payload is copied unchanged. Migration never executes an
/// onset preparation or deployment. If an evergreen field violates V7, every
/// distinct violation across the initial/current pair is returned and no V7
/// candidate exists.
pub fn migrate_v6_snapshot(
    source_configuration: &VegetationConfiguration,
    source_initial_state: &CoupledOwnedState,
    source_state: &CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
) -> Result<V6ToV7MigrationResult, V6ToV7MigrationError> {
    validate_source_configuration(source_configuration)?;
    validate_source_initial_state(source_configuration, source_initial_state)?;
    validate_source_state(source_configuration, source_initial_state, source_state)?;
    validate_target_configuration(target_configuration)?;
    if !identity_only_configuration_payload_matches(source_configuration, target_configuration) {
        return Err(V6ToV7MigrationError::ConfigurationPayloadMismatch);
    }
    let initial_state = rebind_state(source_initial_state, target_configuration)
        .map_err(|_| V6ToV7MigrationError::TargetInitialStateRejected)?;
    if target_configuration.initial_state_sha256 != initial_state.state_sha256 {
        return Err(V6ToV7MigrationError::InvalidV7InitialStateReceipt);
    }

    let unresolved = evergreen_unresolved(source_configuration, source_initial_state, source_state);
    if !unresolved.is_empty() {
        return Ok(V6ToV7MigrationResult::Incomplete(V6ToV7MigrationReport {
            source_model_definition_sha256: V6_MODEL_SHA256.into(),
            target_model_definition_sha256: MODEL_SHA256.into(),
            unresolved,
        }));
    }

    target_configuration
        .validate()
        .map_err(|_| V6ToV7MigrationError::InvalidV7Configuration)?;
    initial_state
        .validate(target_configuration)
        .map_err(|_| V6ToV7MigrationError::TargetInitialStateRejected)?;

    let derived_configuration = derive_target_configuration(source_configuration, &initial_state)
        .map_err(|_| V6ToV7MigrationError::InvalidV7Configuration)?;
    if &derived_configuration != target_configuration
        || derived_configuration.configuration_sha256 == source_configuration.configuration_sha256
    {
        return Err(V6ToV7MigrationError::InvalidV7Configuration);
    }

    let state = rebind_state(source_state, target_configuration)
        .map_err(|_| V6ToV7MigrationError::TargetStateRejected)?;
    state
        .validate(target_configuration)
        .map_err(|_| V6ToV7MigrationError::TargetStateRejected)?;

    Ok(V6ToV7MigrationResult::Complete(Box::new(V6ToV7Migration {
        configuration: derived_configuration,
        initial_state,
        state,
    })))
}

fn validate_source_configuration(
    configuration: &VegetationConfiguration,
) -> Result<(), V6ToV7MigrationError> {
    configuration
        .validate_historical(V6_MODEL_SHA256)
        .map_err(|_| V6ToV7MigrationError::InvalidV6Configuration)
}

fn validate_source_initial_state(
    configuration: &VegetationConfiguration,
    initial_state: &CoupledOwnedState,
) -> Result<(), V6ToV7MigrationError> {
    if initial_state.model_definition_sha256 != V6_MODEL_SHA256
        || initial_state.configuration_sha256 != configuration.configuration_sha256
    {
        return Err(V6ToV7MigrationError::InvalidV6InitialStateIdentity);
    }
    if initial_state.canonical_sha256().ok().as_ref() != Some(&initial_state.state_sha256) {
        return Err(V6ToV7MigrationError::InvalidV6InitialStateDigest);
    }
    if initial_state.last_transaction_id != 0
        || configuration.initial_state_sha256 != initial_state.state_sha256
        || !valid_historical_state(initial_state, configuration)
    {
        return Err(V6ToV7MigrationError::InvalidV6InitialStateLineage);
    }
    Ok(())
}

fn validate_source_state(
    configuration: &VegetationConfiguration,
    initial_state: &CoupledOwnedState,
    state: &CoupledOwnedState,
) -> Result<(), V6ToV7MigrationError> {
    if state.model_definition_sha256 != V6_MODEL_SHA256
        || state.configuration_sha256 != configuration.configuration_sha256
    {
        return Err(V6ToV7MigrationError::InvalidV6StateIdentity);
    }
    if state.canonical_sha256().ok().as_ref() != Some(&state.state_sha256) {
        return Err(V6ToV7MigrationError::InvalidV6StateDigest);
    }
    let transaction_zero_mismatch = state.last_transaction_id == 0
        && serde_json::to_vec(state).ok() != serde_json::to_vec(initial_state).ok();
    if transaction_zero_mismatch || !valid_historical_state(state, configuration) {
        return Err(V6ToV7MigrationError::InvalidV6StateLineage);
    }
    Ok(())
}

fn validate_target_configuration(
    configuration: &VegetationConfiguration,
) -> Result<(), V6ToV7MigrationError> {
    // Historical-mode validation intentionally defers only V7's evergreen
    // semantic rule so that it can be returned exhaustively below.
    configuration
        .validate_historical(MODEL_SHA256)
        .map_err(|_| V6ToV7MigrationError::InvalidV7Configuration)
}

fn valid_historical_state(
    state: &CoupledOwnedState,
    configuration: &VegetationConfiguration,
) -> bool {
    state
        .validate_historical(configuration, V6_MODEL_SHA256)
        .is_ok()
}

fn evergreen_unresolved(
    configuration: &VegetationConfiguration,
    initial_state: &CoupledOwnedState,
    state: &CoupledOwnedState,
) -> Vec<V6ToV7UnresolvedField> {
    let mut unresolved = Vec::new();
    for stratum in &configuration.strata {
        if stratum.phenology_type != PhenologyType::Evergreen {
            continue;
        }
        if stratum.current_growth_fraction.to_bits() != 1.0_f64.to_bits() {
            unresolved.push(V6ToV7UnresolvedField {
                stratum_id: stratum.stratum_id.clone(),
                field: V6ToV7MigrationField::CurrentGrowthFraction,
            });
        }
        for tissue in TISSUES {
            for pool in [V6ToV7PoolIdentity::Storage, V6ToV7PoolIdentity::Transfer] {
                for element in [
                    V6ToV7ElementIdentity::Carbon,
                    V6ToV7ElementIdentity::Nitrogen,
                ] {
                    if pool_element(initial_state, &stratum.stratum_id, tissue, pool, element)
                        .is_some_and(|value| value != 0.0)
                        || pool_element(state, &stratum.stratum_id, tissue, pool, element)
                            .is_some_and(|value| value != 0.0)
                    {
                        unresolved.push(V6ToV7UnresolvedField {
                            stratum_id: stratum.stratum_id.clone(),
                            field: V6ToV7MigrationField::TissuePool {
                                tissue,
                                pool,
                                element,
                            },
                        });
                    }
                }
            }
        }
    }
    unresolved.sort();
    unresolved.dedup();
    unresolved
}

fn pool_element(
    state: &CoupledOwnedState,
    stratum_id: &openwepp_kernel_contract::StratumId,
    tissue: Tissue,
    pool: V6ToV7PoolIdentity,
    element: V6ToV7ElementIdentity,
) -> Option<f64> {
    let tissue_pool = state.strata.get(stratum_id)?.tissues.get(&tissue)?;
    let element_pool = match pool {
        V6ToV7PoolIdentity::Storage => tissue_pool.storage,
        V6ToV7PoolIdentity::Transfer => tissue_pool.transfer,
    };
    Some(match element {
        V6ToV7ElementIdentity::Carbon => element_pool.carbon,
        V6ToV7ElementIdentity::Nitrogen => element_pool.nitrogen,
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

fn rebind_state(
    source: &CoupledOwnedState,
    target_configuration: &VegetationConfiguration,
) -> Result<CoupledOwnedState, crate::VegetationError> {
    let mut target = source.clone();
    target.model_definition_sha256 = MODEL_SHA256.into();
    target
        .configuration_sha256
        .clone_from(&target_configuration.configuration_sha256);
    target.state_sha256.clear();
    target.state_sha256 = target.canonical_sha256()?;
    Ok(target)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::cast_precision_loss)]
    use super::*;

    const V5_CONFIGURATION_BYTES: &[u8] =
        include_bytes!("../../../../tests/fixtures/c3_woody_v5_diagnostic_configuration.json");
    const V5_STATE_BYTES: &[u8] =
        include_bytes!("../../../../tests/fixtures/c3_woody_v5_diagnostic_state.json");

    struct Fixture {
        source_configuration: VegetationConfiguration,
        source_initial_state: CoupledOwnedState,
        source_state: CoupledOwnedState,
        target_configuration: VegetationConfiguration,
    }

    fn fixture(phenology_type: PhenologyType) -> Fixture {
        let mut source_configuration: VegetationConfiguration =
            serde_json::from_slice(V5_CONFIGURATION_BYTES).expect("historical configuration");
        let mut source_initial_state: CoupledOwnedState =
            serde_json::from_slice(V5_STATE_BYTES).expect("historical state");
        source_configuration.model_definition_sha256 = V6_MODEL_SHA256.into();
        for stratum in &mut source_configuration.strata {
            stratum.phenology_type = phenology_type;
            if phenology_type == PhenologyType::SeasonalDeciduous {
                stratum.onset_duration_s = Some(259_200.0);
                stratum.offset_duration_s = Some(259_200.0);
                stratum.gsi_on_threshold = Some(0.55);
                stratum.gsi_off_threshold = Some(0.35);
                stratum.gsi_hysteresis = Some(0.05);
            }
        }
        source_configuration.configuration_sha256.clear();
        source_configuration.configuration_sha256 = source_configuration
            .canonical_sha256()
            .expect("V6 configuration digest");
        source_initial_state.model_definition_sha256 = V6_MODEL_SHA256.into();
        source_initial_state
            .configuration_sha256
            .clone_from(&source_configuration.configuration_sha256);
        source_initial_state.state_sha256 = source_initial_state
            .canonical_sha256()
            .expect("V6 state digest");
        source_configuration
            .initial_state_sha256
            .clone_from(&source_initial_state.state_sha256);
        source_configuration
            .validate_historical(V6_MODEL_SHA256)
            .expect("valid V6 configuration");

        let source_state = source_initial_state.clone();
        let target_configuration =
            target_configuration(&source_configuration, &source_initial_state);
        Fixture {
            source_configuration,
            source_initial_state,
            source_state,
            target_configuration,
        }
    }

    fn target_configuration(
        source: &VegetationConfiguration,
        initial_state: &CoupledOwnedState,
    ) -> VegetationConfiguration {
        let mut target = source.clone();
        target.model_definition_sha256 = MODEL_SHA256.into();
        target.configuration_sha256.clear();
        target.configuration_sha256 = target.canonical_sha256().expect("V7 digest");
        let rebound = rebind_state(initial_state, &target).expect("rebound initial state");
        target.initial_state_sha256 = rebound.state_sha256;
        target
    }

    fn source_bytes(fixture: &Fixture) -> Vec<Vec<u8>> {
        [
            serde_json::to_vec(&fixture.source_configuration).expect("configuration bytes"),
            serde_json::to_vec(&fixture.source_initial_state).expect("initial bytes"),
            serde_json::to_vec(&fixture.source_state).expect("state bytes"),
        ]
        .to_vec()
    }

    #[test]
    fn seasonal_migration_preserves_every_nonidentity_byte_and_does_not_run_onset() {
        let mut fixture = fixture(PhenologyType::SeasonalDeciduous);
        let shared = fixture
            .source_initial_state
            .strata
            .values_mut()
            .next()
            .expect("shared state");
        shared.phase = crate::PhenologyPhase::Dormant;
        shared.previous_gsi = 0.2;
        for (index, tissue) in TISSUES.into_iter().enumerate() {
            let pool = shared.tissues.get_mut(&tissue).expect("six tissues");
            pool.storage.carbon = 0.01 + index as f64 * 0.001;
            pool.storage.nitrogen = 0.001 + index as f64 * 0.0001;
            pool.transfer.carbon = 0.02 + index as f64 * 0.001;
            pool.transfer.nitrogen = 0.002 + index as f64 * 0.0001;
        }
        fixture.source_initial_state.state_sha256 = fixture
            .source_initial_state
            .canonical_sha256()
            .expect("mutated initial digest");
        fixture.source_configuration.initial_state_sha256 =
            fixture.source_initial_state.state_sha256.clone();
        fixture.source_state = fixture.source_initial_state.clone();
        fixture.source_state.last_transaction_id = 7;
        for shared in fixture.source_state.strata.values_mut() {
            shared.last_transaction_id = 7;
        }
        for occupancy in fixture.source_state.occupancies.values_mut() {
            occupancy.last_accepted_transaction_id = Some(7);
        }
        fixture.source_state.state_sha256 = fixture
            .source_state
            .canonical_sha256()
            .expect("accepted V6 state digest");
        fixture.target_configuration =
            target_configuration(&fixture.source_configuration, &fixture.source_initial_state);

        let result = migrate_v6_snapshot(
            &fixture.source_configuration,
            &fixture.source_initial_state,
            &fixture.source_state,
            &fixture.target_configuration,
        )
        .expect("migration result");
        let V6ToV7MigrationResult::Complete(migrated) = result else {
            panic!("seasonal state must migrate completely")
        };
        assert_eq!(migrated.configuration, fixture.target_configuration);

        let mut expected = fixture.source_state.clone();
        expected.model_definition_sha256 = MODEL_SHA256.into();
        expected.configuration_sha256 = fixture.target_configuration.configuration_sha256.clone();
        expected.state_sha256.clear();
        let mut actual = migrated.state;
        actual.state_sha256.clear();
        assert_eq!(actual, expected);
    }

    #[test]
    fn evergreen_reports_all_twenty_five_fields_once_and_preserves_sources() {
        let mut fixture = fixture(PhenologyType::Evergreen);
        for state in [&mut fixture.source_initial_state, &mut fixture.source_state] {
            for shared in state.strata.values_mut() {
                for (index, tissue) in TISSUES.into_iter().enumerate() {
                    let pool = shared.tissues.get_mut(&tissue).expect("six tissues");
                    pool.storage.carbon = 1.0 + index as f64;
                    pool.storage.nitrogen = 2.0 + index as f64;
                    pool.transfer.carbon = 3.0 + index as f64;
                    pool.transfer.nitrogen = 4.0 + index as f64;
                }
            }
            state.state_sha256 = state.canonical_sha256().expect("poison digest");
        }
        fixture.source_configuration.initial_state_sha256 =
            fixture.source_initial_state.state_sha256.clone();
        fixture.target_configuration =
            target_configuration(&fixture.source_configuration, &fixture.source_initial_state);
        let before = source_bytes(&fixture);

        let result = migrate_v6_snapshot(
            &fixture.source_configuration,
            &fixture.source_initial_state,
            &fixture.source_state,
            &fixture.target_configuration,
        )
        .expect("semantic report");
        let V6ToV7MigrationResult::Incomplete(report) = result else {
            panic!("invalid evergreen state must not migrate")
        };
        assert_eq!(report.unresolved.len(), 25);
        assert_eq!(
            report.unresolved[0].field,
            V6ToV7MigrationField::CurrentGrowthFraction
        );
        assert_eq!(source_bytes(&fixture), before);
    }

    #[test]
    fn valid_evergreen_accepts_both_zero_signs() {
        let mut fixture = fixture(PhenologyType::Evergreen);
        for stratum in &mut fixture.source_configuration.strata {
            stratum.current_growth_fraction = 1.0;
        }
        fixture.source_configuration.configuration_sha256.clear();
        fixture.source_configuration.configuration_sha256 = fixture
            .source_configuration
            .canonical_sha256()
            .expect("V6 digest");
        for state in [&mut fixture.source_initial_state, &mut fixture.source_state] {
            state.configuration_sha256 = fixture.source_configuration.configuration_sha256.clone();
            let pool = &mut state
                .strata
                .values_mut()
                .next()
                .expect("shared")
                .tissues
                .get_mut(&Tissue::Leaf)
                .expect("leaf")
                .storage;
            pool.carbon = -0.0;
            pool.nitrogen = 0.0;
            state.state_sha256 = state.canonical_sha256().expect("state digest");
        }
        fixture.source_configuration.initial_state_sha256 =
            fixture.source_initial_state.state_sha256.clone();
        fixture.target_configuration =
            target_configuration(&fixture.source_configuration, &fixture.source_initial_state);

        assert!(matches!(
            migrate_v6_snapshot(
                &fixture.source_configuration,
                &fixture.source_initial_state,
                &fixture.source_state,
                &fixture.target_configuration,
            ),
            Ok(V6ToV7MigrationResult::Complete(_))
        ));
    }

    #[test]
    fn invalid_source_digest_fails_without_mutation() {
        let mut fixture = fixture(PhenologyType::SeasonalDeciduous);
        fixture.source_state.state_sha256 = "0".repeat(64);
        let before = source_bytes(&fixture);
        assert_eq!(
            migrate_v6_snapshot(
                &fixture.source_configuration,
                &fixture.source_initial_state,
                &fixture.source_state,
                &fixture.target_configuration,
            ),
            Err(V6ToV7MigrationError::InvalidV6StateDigest)
        );
        assert_eq!(source_bytes(&fixture), before);
    }

    #[test]
    fn invalid_transaction_lineage_fails_without_mutation() {
        let mut fixture = fixture(PhenologyType::SeasonalDeciduous);
        fixture.source_state.last_transaction_id = 7;
        fixture.source_state.state_sha256 = fixture
            .source_state
            .canonical_sha256()
            .expect("poison digest");
        let before = source_bytes(&fixture);
        assert_eq!(
            migrate_v6_snapshot(
                &fixture.source_configuration,
                &fixture.source_initial_state,
                &fixture.source_state,
                &fixture.target_configuration,
            ),
            Err(V6ToV7MigrationError::InvalidV6StateLineage)
        );
        assert_eq!(source_bytes(&fixture), before);
    }

    #[test]
    fn target_initial_receipt_is_strict_even_when_evergreen_is_unresolved() {
        let mut fixture = fixture(PhenologyType::Evergreen);
        fixture.target_configuration.initial_state_sha256 = "0".repeat(64);
        let before = source_bytes(&fixture);
        assert_eq!(
            migrate_v6_snapshot(
                &fixture.source_configuration,
                &fixture.source_initial_state,
                &fixture.source_state,
                &fixture.target_configuration,
            ),
            Err(V6ToV7MigrationError::InvalidV7InitialStateReceipt)
        );
        assert_eq!(source_bytes(&fixture), before);
    }
}
