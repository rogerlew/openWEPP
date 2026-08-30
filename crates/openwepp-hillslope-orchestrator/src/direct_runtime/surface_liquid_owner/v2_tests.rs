use std::collections::BTreeMap;

use super::tests::{configuration, state};
use super::*;

fn digest(byte: char) -> String {
    std::iter::repeat_n(byte, 64).collect()
}

fn model() -> SurfaceLiquidOwnerModelDefinitionV2 {
    SurfaceLiquidOwnerModelDefinitionV2::new(digest('1'), digest('2'), digest('3'))
        .expect("valid exact successor identity")
}

fn configuration_v2() -> SurfaceLiquidConfigurationV2 {
    let parent = configuration();
    let depths = parent
        .records
        .iter()
        .filter(|record| record.key.surface_class == SurfaceClass::ForestLitter)
        .map(|record| (record.key.clone(), 0.03125))
        .collect();
    SurfaceLiquidConfigurationV2::new(parent, model(), &depths).expect("valid V2 configuration")
}

fn enthalpy_by_key(
    configuration: &DirectSurfaceLiquidConfiguration,
) -> BTreeMap<DirectSurfaceLiquidStoreKey, f64> {
    configuration
        .records
        .iter()
        .enumerate()
        .map(|(index, record)| (record.key.clone(), 1250.0 + index as f64))
        .collect()
}

fn explicit_ice_by_key(
    configuration: &DirectSurfaceLiquidConfiguration,
) -> BTreeMap<DirectSurfaceLiquidStoreKey, f64> {
    configuration
        .records
        .iter()
        .map(|record| {
            let ice = if record.key.surface_class == SurfaceClass::ForestLitter {
                0.375
            } else {
                0.0
            };
            (record.key.clone(), ice)
        })
        .collect()
}

#[test]
fn model_and_configuration_bytes_bind_exact_sources_constants_and_capacity_basis() {
    let model = model();
    let bytes = String::from_utf8(model.canonical_bytes().expect("canonical model bytes"))
        .expect("model bytes are UTF-8 JSON");
    let sources = SurfaceLiquidOwnerSourceIdentityV2::canonical();
    for source in [
        sources.r156_sha256(),
        sources.isba_meb_sha256(),
        sources.isba_fluxes_meb_sha256(),
        sources.ini_csts_sha256(),
        sources.cecill_c_license_sha256(),
    ] {
        assert!(bytes.contains(source));
    }
    for exact_bits in [
        format!("{:016x}", 273.15_f64.to_bits()),
        format!("{:016x}", 920.0_f64.to_bits()),
        format!("{:016x}", 2106.0_f64.to_bits()),
        format!("{:016x}", 333_700.0_f64.to_bits()),
        format!("{:016x}", 3300.0_f64.to_bits()),
    ] {
        assert!(bytes.contains(&exact_bits));
    }

    let configuration = configuration_v2();
    let litter = configuration
        .records()
        .iter()
        .find(|record| record.litter_depth_m.is_some())
        .expect("litter V2 configuration");
    assert_eq!(
        litter
            .litter_ice_capacity_kg_m2_tile
            .expect("explicit ice capacity")
            .to_bits(),
        (0.85_f64 * 1000.0 * 0.03125).to_bits()
    );
}

#[test]
fn v1_migration_preserves_frozen_bytes_and_initializes_positive_zero_ice() {
    let configuration = configuration_v2();
    let v1 = state(configuration.parent());
    let v1_configuration_bytes = configuration
        .parent()
        .canonical_bytes()
        .expect("V1 configuration bytes");
    let v1_state_bytes = v1
        .canonical_bytes(configuration.parent())
        .expect("V1 state bytes");
    let migrated = SurfaceLiquidOwnedStateV2::migrate_from_v1(
        &configuration,
        &v1,
        &enthalpy_by_key(configuration.parent()),
    )
    .expect("checked V1 to V2 migration");

    assert!(
        migrated
            .records()
            .iter()
            .all(|record| record.litter_ice_kg_m2_tile.to_bits() == 0)
    );
    for (migrated, frozen) in migrated.records().iter().zip(&v1.records) {
        assert_eq!(migrated.key, frozen.key);
        assert_eq!(
            migrated.liquid_kg_m2_tile.to_bits(),
            frozen.liquid_kg_m2_tile.to_bits()
        );
        assert_eq!(
            migrated.last_accepted_transaction_id,
            frozen.last_accepted_transaction_id
        );
    }
    assert_eq!(migrated.continuations(), v1.continuations);
    assert_eq!(
        configuration
            .parent()
            .canonical_bytes()
            .expect("unchanged V1 configuration bytes"),
        v1_configuration_bytes
    );
    assert_eq!(
        v1.canonical_bytes(configuration.parent())
            .expect("unchanged V1 state bytes"),
        v1_state_bytes
    );

    let represented = migrated
        .zero_ice_v1_representability_for_test(&configuration)
        .expect("test-only exact-zero-ice proof");
    assert_eq!(
        represented
            .canonical_bytes(configuration.parent())
            .expect("represented V1 bytes"),
        v1_state_bytes
    );
}

#[test]
fn named_v1_to_v2_migration_is_equivalent_and_rejects_poisoned_v1_identity() {
    let configuration = configuration_v2();
    let v1 = state(configuration.parent());
    let enthalpy = enthalpy_by_key(configuration.parent());
    let named =
        migrate_v1_to_v2(&configuration, &v1, &enthalpy).expect("named checked V1 to V2 migration");
    let associated = SurfaceLiquidOwnedStateV2::migrate_from_v1(&configuration, &v1, &enthalpy)
        .expect("associated checked V1 to V2 migration");
    assert_eq!(named, associated);

    let mut poisoned = v1;
    poisoned.state_sha256 = digest('f');
    let error = migrate_v1_to_v2(&configuration, &poisoned, &enthalpy)
        .expect_err("poisoned V1 digest must fail closed");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E002);
}

#[test]
fn explicit_ice_seed_is_finite_bounded_and_never_donates_to_bare_surface() {
    let configuration = configuration_v2();
    let parent_state = state(configuration.parent());
    let liquid = parent_state
        .records
        .iter()
        .map(|record| (record.key.clone(), record.liquid_kg_m2_tile))
        .collect();
    let explicit_ice = explicit_ice_by_key(configuration.parent());
    let state = SurfaceLiquidOwnedStateV2::new_initial(
        &configuration,
        &liquid,
        &explicit_ice,
        &enthalpy_by_key(configuration.parent()),
        3,
    )
    .expect("explicit V2 ice seed");
    assert_eq!(
        state
            .records()
            .iter()
            .find(|record| record.key.surface_class == SurfaceClass::ForestLitter)
            .expect("litter record")
            .litter_ice_kg_m2_tile,
        0.375
    );
    let error = state
        .zero_ice_v1_representability_for_test(&configuration)
        .expect_err("nonzero ice cannot downgrade");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E004);

    let mut invalid_records = state.records().to_vec();
    invalid_records
        .iter_mut()
        .find(|record| record.key.surface_class == SurfaceClass::BareMineralSoil)
        .expect("bare record")
        .litter_ice_kg_m2_tile = f64::from_bits(1);
    let error = SurfaceLiquidOwnedStateV2::try_new(
        &configuration,
        invalid_records,
        state.continuations().to_vec(),
    )
    .expect_err("bare surface ice must fail closed");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
}

#[test]
fn configuration_state_envelope_and_restart_round_trip_canonical_bytes() {
    let configuration = configuration_v2();
    let configuration_bytes = configuration
        .canonical_bytes()
        .expect("canonical V2 configuration");
    let restored_configuration = SurfaceLiquidConfigurationV2::from_canonical_bytes(
        configuration.parent().clone(),
        configuration.model_definition().clone(),
        &configuration_bytes,
    )
    .expect("restore V2 configuration");
    assert_eq!(restored_configuration, configuration);

    let v1 = state(configuration.parent());
    let state = SurfaceLiquidOwnedStateV2::migrate_from_v1(
        &configuration,
        &v1,
        &enthalpy_by_key(configuration.parent()),
    )
    .expect("migrated V2 state");
    let state_bytes = state
        .canonical_bytes(&configuration)
        .expect("canonical V2 state");
    assert_eq!(
        SurfaceLiquidOwnedStateV2::from_canonical_bytes(&configuration, &state_bytes)
            .expect("restore V2 state")
            .canonical_bytes(&configuration)
            .expect("restored state bytes"),
        state_bytes
    );

    let envelope =
        SurfaceLiquidOwnerEnvelopeV2::wrap_v2(&configuration, state).expect("V2 owner envelope");
    let envelope_bytes = envelope
        .canonical_bytes(configuration.parent(), Some(&configuration))
        .expect("canonical envelope bytes");
    let restored_envelope = SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
        configuration.parent(),
        Some(&configuration),
        &envelope_bytes,
    )
    .expect("restore owner envelope");
    assert_eq!(restored_envelope, envelope);

    let restart =
        SurfaceLiquidOwnerRestartV2::new(configuration.parent(), Some(&configuration), envelope)
            .expect("restart frame");
    let restart_bytes = restart
        .canonical_bytes(configuration.parent(), Some(&configuration))
        .expect("canonical restart bytes");
    let restored_restart = SurfaceLiquidOwnerRestartV2::from_canonical_bytes(
        configuration.parent(),
        Some(&configuration),
        &restart_bytes,
    )
    .expect("restore restart frame");
    assert_eq!(restored_restart, restart);
    assert_eq!(
        restored_restart
            .canonical_bytes(configuration.parent(), Some(&configuration))
            .expect("restored restart bytes"),
        restart_bytes
    );
}

#[test]
fn v1_envelope_wrap_does_not_rewrite_frozen_v1_state_bytes() {
    let configuration = configuration();
    let state = state(&configuration);
    let frozen = state
        .canonical_bytes(&configuration)
        .expect("frozen V1 state bytes");
    let envelope = SurfaceLiquidOwnerEnvelopeV2::wrap_v1(&configuration, state, digest('3'))
        .expect("V1 successor envelope variant");
    let bytes = envelope
        .canonical_bytes(&configuration, None)
        .expect("V1 envelope bytes");
    let restored = SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(&configuration, None, &bytes)
        .expect("restore V1 envelope variant");
    let restored_bytes = restored
        .canonical_bytes(&configuration, None)
        .expect("restored V1 envelope bytes");
    assert_eq!(restored_bytes, bytes);
    assert_eq!(
        restored
            .v1_state()
            .expect("V1 payload")
            .canonical_bytes(&configuration)
            .expect("embedded frozen V1 bytes"),
        frozen
    );
}

#[test]
fn failed_candidate_replacement_preserves_exact_beginning_envelope() {
    let configuration = configuration_v2();
    let v1 = state(configuration.parent());
    let state = SurfaceLiquidOwnedStateV2::migrate_from_v1(
        &configuration,
        &v1,
        &enthalpy_by_key(configuration.parent()),
    )
    .expect("migrated V2 state");
    let envelope = SurfaceLiquidOwnerEnvelopeV2::wrap_v2(&configuration, state.clone())
        .expect("beginning envelope");
    let beginning = envelope
        .canonical_bytes(configuration.parent(), Some(&configuration))
        .expect("beginning bytes");
    let mut invalid = state.records().to_vec();
    invalid[0].liquid_kg_m2_tile = f64::NAN;
    let error = envelope
        .try_replace_v2_state(&configuration, invalid, state.continuations().to_vec())
        .expect_err("invalid candidate must fail closed");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
    let failure = error.failure().expect("complete rollback context");
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::AtomicEnvelope);
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());
    assert_eq!(
        envelope
            .canonical_bytes(configuration.parent(), Some(&configuration))
            .expect("unchanged beginning bytes"),
        beginning
    );
}

#[test]
fn generic_phase_separated_owner_mass_closure_is_independently_checked() {
    let configuration = configuration_v2();
    let v1 = state(configuration.parent());
    let beginning = SurfaceLiquidOwnedStateV2::migrate_from_v1(
        &configuration,
        &v1,
        &enthalpy_by_key(configuration.parent()),
    )
    .expect("beginning V2 state");
    let operands = beginning
        .records()
        .iter()
        .map(|record| SurfaceLiquidOwnerClosureRecordV2 {
            key: record.key.clone(),
            liquid_debit_kg_m2_tile: 0.0,
            liquid_credit_kg_m2_tile: 0.0,
            ice_debit_kg_m2_tile: 0.0,
            ice_credit_kg_m2_tile: 0.0,
        })
        .collect::<Vec<_>>();
    validate_surface_liquid_owner_mass_closure_v2(
        &configuration,
        &beginning,
        &beginning,
        &operands,
    )
    .expect("independently reconstructed no-op closure");

    let mut poisoned = operands;
    poisoned[0].liquid_debit_kg_m2_tile = 0.25;
    let error = validate_surface_liquid_owner_mass_closure_v2(
        &configuration,
        &beginning,
        &beginning,
        &poisoned,
    )
    .expect_err("wrong liquid operand must fail closure");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);
}
