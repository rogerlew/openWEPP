use super::*;

#[test]
fn surface_attachment_attempt_hashes_bind_raw_invalid_bits_not_stale_declared_digests() {
    let configuration = surface_configuration(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
    );
    let initial = DirectSurfaceLiquidOwnedState::new_initial(
        &configuration,
        &BTreeMap::from([(configuration.records[0].key.clone(), 2.0)]),
        0,
    )
    .expect("initial state");

    let state_failure_hash = |raw_bits| {
        let mut attempted = initial.clone();
        attempted.records[0].liquid_kg_m2_tile = f64::from_bits(raw_bits);
        let mut frame = production_frame(0.02, false);
        let error = frame
            .configure_surface_liquid_shadow(&configuration, attempted)
            .expect_err("raw nonfinite state must fail closed");
        let failure = error.failure().expect("canonical state failure");
        assert_eq!(failure.rollback.beginning_owner_sha256, None);
        failure
            .rollback
            .attempted_owner_sha256
            .clone()
            .expect("direct wrapper attempted hash")
    };
    let state_a = state_failure_hash(0x7ff8_0000_0000_0001);
    let state_b = state_failure_hash(0x7ff8_0000_0000_0002);
    assert_ne!(state_a, state_b, "raw NaN payload bits must enter the hash");

    let configuration_failure_hash = |raw_bits| {
        let mut attempted_configuration = configuration.clone();
        attempted_configuration.records[0].capacity_kg_m2_tile = f64::from_bits(raw_bits);
        assert_eq!(
            attempted_configuration.configuration_sha256, configuration.configuration_sha256,
            "poisons retain one stale declared digest"
        );
        let mut frame = production_frame(0.02, false);
        let error = frame
            .configure_surface_liquid_shadow(&attempted_configuration, initial.clone())
            .expect_err("raw nonfinite configuration must fail closed");
        let failure = error.failure().expect("canonical configuration failure");
        assert_eq!(failure.rollback.beginning_owner_sha256, None);
        failure
            .rollback
            .attempted_owner_sha256
            .clone()
            .expect("direct wrapper attempted hash")
    };
    let configuration_a = configuration_failure_hash(0x7ff8_0000_0000_0011);
    let configuration_b = configuration_failure_hash(0x7ff8_0000_0000_0012);
    assert_ne!(
        configuration_a, configuration_b,
        "raw invalid configuration bits must enter the hash"
    );
}

#[test]
fn unified_snapshot_failure_hashes_bind_raw_invalid_configuration_bits() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot_failure = |raw_bits| {
        let mut attempted = configuration.clone();
        attempted.records[0].ofe_area_m2 = f64::from_bits(raw_bits);
        assert_eq!(
            attempted.configuration_sha256, configuration.configuration_sha256,
            "both attempts retain the same stale declared digest"
        );
        let LandSurfaceEnergyShadowError::SurfaceLiquid(error) =
            unified_beginning_hydrology_snapshot_sha256(&adapter, &attempted)
                .expect_err("invalid raw snapshot input must fail closed")
        else {
            panic!("snapshot failure must retain canonical surface-liquid context");
        };
        let failure = error.failure().expect("canonical snapshot failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
        assert!(failure.rollback.beginning_owner_sha256.is_some());
        failure
            .rollback
            .attempted_owner_sha256
            .clone()
            .expect("snapshot attempted hash")
    };

    let first = snapshot_failure(0x7ff8_0000_0000_0101);
    let second = snapshot_failure(0x7ff8_0000_0000_0102);
    assert_ne!(
        first, second,
        "raw invalid snapshot attempts must not collide"
    );
}
