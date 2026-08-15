//! Complete receiver-expectation hashing and callback-boundary poisons.

use super::*;

fn expectation(
    snapshot: Sha256Digest,
    lse_owner: &str,
    lse_beginning: Sha256Digest,
    hydrology_owner: &str,
    thermal_owner: &str,
    thermal_beginning: Sha256Digest,
    tile: &str,
    layer: &str,
) -> UnifiedReceiverExpectations {
    UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new(lse_owner).expect("LSE owner"),
        lse_beginning,
        ResourceOwnerId::try_new(hydrology_owner).expect("hydrology owner"),
        snapshot,
        ResourceOwnerId::try_new(thermal_owner).expect("thermal owner"),
        thermal_beginning,
        vec![(
            OfeId::try_new("ofe-1").expect("OFE"),
            TileId::try_new(tile).expect("tile"),
            vec![SoilLayerId::try_new(layer).expect("layer")],
        )],
    )
    .expect("structurally valid expectations")
}

#[test]
fn attempted_hash_binds_every_receiver_expectation_field_and_expected_snapshot() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let actual = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("actual snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let attempted = |expectations: UnifiedReceiverExpectations| {
        let mut ingress = ingress_input();
        ingress.interval_s = f64::NAN;
        let error = execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &expectations,
            &batch,
            &BTreeMap::new(),
            &ingress,
            |_| panic!("nonfinite ingress reached callback"),
        )
        .expect_err("nonfinite ingress");
        let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
            panic!("expectation hash poison must remain canonical");
        };
        let failure = error.failure().expect("canonical failure");
        assert!(matches!(
            failure.code,
            DirectSurfaceLiquidErrorCode::E002 | DirectSurfaceLiquidErrorCode::E003
        ));
        failure
            .rollback
            .attempted_owner_sha256
            .clone()
            .expect("complete attempted hash")
    };
    let make = |snapshot,
                lse_owner,
                lse_digest,
                hydrology_owner,
                thermal_owner,
                thermal_digest,
                tile,
                layer| {
        expectation(
            snapshot,
            lse_owner,
            lse_digest,
            hydrology_owner,
            thermal_owner,
            thermal_digest,
            tile,
            layer,
        )
    };
    let baseline = attempted(make(
        actual.clone(),
        "land-surface-energy-v1",
        digest('2'),
        "production-hydrology",
        "soil-thermal",
        digest('4'),
        "open",
        "thermal-1",
    ));
    let variants = [
        make(
            actual.clone(),
            "lse-other",
            digest('2'),
            "production-hydrology",
            "soil-thermal",
            digest('4'),
            "open",
            "thermal-1",
        ),
        make(
            actual.clone(),
            "land-surface-energy-v1",
            digest('7'),
            "production-hydrology",
            "soil-thermal",
            digest('4'),
            "open",
            "thermal-1",
        ),
        make(
            actual.clone(),
            "land-surface-energy-v1",
            digest('2'),
            "hydrology-other",
            "soil-thermal",
            digest('4'),
            "open",
            "thermal-1",
        ),
        make(
            digest('8'),
            "land-surface-energy-v1",
            digest('2'),
            "production-hydrology",
            "soil-thermal",
            digest('4'),
            "open",
            "thermal-1",
        ),
        make(
            actual.clone(),
            "land-surface-energy-v1",
            digest('2'),
            "production-hydrology",
            "thermal-other",
            digest('4'),
            "open",
            "thermal-1",
        ),
        make(
            actual.clone(),
            "land-surface-energy-v1",
            digest('2'),
            "production-hydrology",
            "soil-thermal",
            digest('6'),
            "open",
            "thermal-1",
        ),
        make(
            actual.clone(),
            "land-surface-energy-v1",
            digest('2'),
            "production-hydrology",
            "soil-thermal",
            digest('4'),
            "wrong-tile",
            "thermal-1",
        ),
        make(
            actual,
            "land-surface-energy-v1",
            digest('2'),
            "production-hydrology",
            "soil-thermal",
            digest('4'),
            "open",
            "thermal-other",
        ),
    ];
    for (index, variant) in variants.into_iter().enumerate() {
        assert_ne!(baseline, attempted(variant), "expectation field {index}");
    }
}

#[test]
fn invalid_expectation_lineage_and_topology_reject_before_callback() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let original = frame.clone();
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let invalid = [
        expectation(
            snapshot.clone(),
            "lse-other",
            digest('2'),
            "production-hydrology",
            "soil-thermal",
            digest('4'),
            "open",
            "thermal-1",
        ),
        expectation(
            snapshot.clone(),
            "land-surface-energy-v1",
            digest('2'),
            "hydrology-other",
            "soil-thermal",
            digest('4'),
            "open",
            "thermal-1",
        ),
        expectation(
            snapshot.clone(),
            "land-surface-energy-v1",
            digest('2'),
            "production-hydrology",
            "production-hydrology",
            digest('4'),
            "open",
            "thermal-1",
        ),
        expectation(
            snapshot.clone(),
            "land-surface-energy-v1",
            digest('2'),
            "production-hydrology",
            "soil-thermal",
            digest('2'),
            "open",
            "thermal-1",
        ),
        expectation(
            snapshot.clone(),
            "land-surface-energy-v1",
            digest('2'),
            "production-hydrology",
            "soil-thermal",
            digest('4'),
            "wrong-tile",
            "thermal-1",
        ),
    ];
    for (index, expectations) in invalid.into_iter().enumerate() {
        let callback_count = std::cell::Cell::new(0);
        let error = execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &expectations,
            &batch,
            &BTreeMap::new(),
            &ingress_input(),
            |_| {
                callback_count.set(callback_count.get() + 1);
                panic!("invalid expectation reached callback")
            },
        )
        .expect_err("invalid expectations");
        let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
            panic!("expectation failure must remain canonical");
        };
        let failure = error.failure().expect("canonical expectation failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E011, "{index}");
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::AtomicEnvelope);
        assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(snapshot.as_str())
        );
        assert!(failure.rollback.attempted_owner_sha256.is_some());
        assert_eq!(callback_count.get(), 0);
        assert_eq!(frame, original);
    }
}

#[test]
fn raw_callback_errors_are_canonicalized_without_fabricated_row_identity() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let original = frame.clone();
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    for (poison, expected) in [
        (0, DirectSurfaceLiquidErrorCode::E002),
        (1, DirectSurfaceLiquidErrorCode::E003),
        (2, DirectSurfaceLiquidErrorCode::E006),
        (3, DirectSurfaceLiquidErrorCode::E002),
        (4, DirectSurfaceLiquidErrorCode::E003),
    ] {
        let callback_count = std::cell::Cell::new(0);
        let error = execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &receiver_expectations(1, snapshot.clone()),
            &batch,
            &BTreeMap::new(),
            &ingress_input(),
            |_| {
                callback_count.set(callback_count.get() + 1);
                Err(match poison {
                    0 => LandSurfaceEnergyShadowError::Identity("raw callback identity"),
                    1 => LandSurfaceEnergyShadowError::Operand("raw callback operand"),
                    2 => LandSurfaceEnergyShadowError::Bound("raw callback bound"),
                    3 => LandSurfaceEnergyShadowError::UnsupportedCustody("raw callback custody"),
                    _ => LandSurfaceEnergyShadowError::LandSurface(
                        LandSurfaceEnergyError::NonFinite("raw callback LSE"),
                    ),
                })
            },
        )
        .expect_err("raw callback error");
        let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
            panic!("raw callback error escaped public boundary");
        };
        let failure = error.failure().expect("canonical callback failure");
        assert_eq!(failure.code, expected, "{poison}");
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::ResourceCandidate);
        assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
        assert_eq!(failure.context.owner_id, None);
        assert_eq!(failure.context.ofe_id, None);
        assert_eq!(failure.context.tile_id, None);
        assert_eq!(failure.context.surface_id, None);
        assert_eq!(failure.context.source_id, None);
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(snapshot.as_str())
        );
        assert!(failure.rollback.attempted_owner_sha256.is_some());
        assert_eq!(callback_count.get(), 1);
        assert_eq!(frame, original);
    }
}
