//! Complete receiver-expectation hashing and callback-boundary poisons.

use super::*;

fn protocol_numeric_vector(
    request_amount: f64,
    authorization_amount: f64,
    finalized_amount: f64,
    condensation_amount: Option<f64>,
) -> WaterProtocol {
    let ground_key = key("ground", "thermal-1");
    WaterProtocol {
        transaction_id: TransactionId(41),
        hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        beginning_snapshot_sha256: digest('3'),
        requests: vec![
            openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                key: ground_key.clone(),
                amount_kg_m2_stand_ground: request_amount,
            },
        ],
        authorizations: vec![WaterAuthorization {
            key: ground_key.clone(),
            amount_kg_m2_stand_ground: authorization_amount,
            reason: WaterAuthorizationReason::FullSupply,
        }],
        finalized_uses: vec![
            openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                key: ground_key,
                amount_kg_m2_stand_ground: finalized_amount,
            },
        ],
        condensation_credits: condensation_amount
            .map(|amount| CondensationCredit {
                transaction_id: TransactionId(41),
                hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology")
                    .expect("owner"),
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                tile_id: TileId::try_new("open").expect("tile"),
                surface_id: SurfaceId::try_new("surface:ofe-1:open").expect("surface"),
                amount_kg_m2_stand_ground: amount,
                amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
                temperature_k: 290.0,
                specific_liquid_enthalpy_j_kg: 70_000.0,
            })
            .into_iter()
            .collect(),
    }
}

// The helper mirrors the complete receiver-expectation field set explicitly.
#[allow(clippy::too_many_arguments)]
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
// This exhaustive field-binding matrix is intentionally kept together for auditability.
#[allow(clippy::too_many_lines)]
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
        (3, DirectSurfaceLiquidErrorCode::E004),
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

#[test]
// This exhaustive error-taxonomy matrix is intentionally kept together for auditability.
#[allow(clippy::too_many_lines)]
fn callback_lse_taxonomy_is_exhaustive_and_rebound_to_resource_candidate() {
    let cases = vec![
        (
            LandSurfaceEnergyError::MalformedSerialization("bad".into()),
            DirectSurfaceLiquidErrorCode::E001,
        ),
        (
            LandSurfaceEnergyError::Identity {
                field: "owner",
                expected: "a".into(),
                found: "b".into(),
            },
            DirectSurfaceLiquidErrorCode::E002,
        ),
        (
            LandSurfaceEnergyError::topology_cardinality("duplicate"),
            DirectSurfaceLiquidErrorCode::E005,
        ),
        (
            LandSurfaceEnergyError::topology_domain("open_trial_shape"),
            DirectSurfaceLiquidErrorCode::E003,
        ),
        (
            LandSurfaceEnergyError::NonFinite("operand"),
            DirectSurfaceLiquidErrorCode::E003,
        ),
        (
            LandSurfaceEnergyError::UnsupportedDomain("snow"),
            DirectSurfaceLiquidErrorCode::E004,
        ),
        (
            LandSurfaceEnergyError::ConstitutiveDomain("temperature"),
            DirectSurfaceLiquidErrorCode::E003,
        ),
        (
            LandSurfaceEnergyError::water_identity("identity mismatch"),
            DirectSurfaceLiquidErrorCode::E002,
        ),
        (
            LandSurfaceEnergyError::water_domain("nonfinite authorization"),
            DirectSurfaceLiquidErrorCode::E003,
        ),
        (
            LandSurfaceEnergyError::water_cardinality("missing authorization"),
            DirectSurfaceLiquidErrorCode::E005,
        ),
        (
            LandSurfaceEnergyError::water_bound("D/A/F"),
            DirectSurfaceLiquidErrorCode::E006,
        ),
        (
            LandSurfaceEnergyError::water_closure("pre_ingress_source_mass_closure"),
            DirectSurfaceLiquidErrorCode::E010,
        ),
        (
            LandSurfaceEnergyError::StateLineage("stale"),
            DirectSurfaceLiquidErrorCode::E002,
        ),
        (
            LandSurfaceEnergyError::OwnerEnvelope("wrong owner"),
            DirectSurfaceLiquidErrorCode::E011,
        ),
        (
            LandSurfaceEnergyError::NumericalAcceptedResidual,
            DirectSurfaceLiquidErrorCode::E003,
        ),
        (
            LandSurfaceEnergyError::ComponentClosure("component"),
            DirectSurfaceLiquidErrorCode::E010,
        ),
        (
            LandSurfaceEnergyError::ControlVolumeClosure("volume"),
            DirectSurfaceLiquidErrorCode::E010,
        ),
        (
            LandSurfaceEnergyError::LatentJoin("latent"),
            DirectSurfaceLiquidErrorCode::E010,
        ),
        (
            LandSurfaceEnergyError::GroundHeatJoin("ground"),
            DirectSurfaceLiquidErrorCode::E010,
        ),
    ];
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
    let missing_authorization = WaterProtocol {
        transaction_id: batch.transaction_id,
        hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        beginning_snapshot_sha256: snapshot.clone(),
        requests: batch.requests.clone(),
        authorizations: Vec::new(),
        finalized_uses: Vec::new(),
        condensation_credits: Vec::new(),
    }
    .validate()
    .expect_err("missing authorization");
    let real_cases = vec![
        (
            OfeId::try_new("").expect_err("empty OFE is topology cardinality"),
            DirectSurfaceLiquidErrorCode::E005,
        ),
        (
            evaluate_open_surface(&open_problem(), &[], None, None)
                .expect_err("wrong open trial shape"),
            DirectSurfaceLiquidErrorCode::E003,
        ),
        (
            validate_water_use(WaterUseOperands {
                request_kg_m2: 1.0,
                authorization_kg_m2: 0.5,
                finalized_use_kg_m2: 0.6,
                beginning_store_kg_m2: 1.0,
                condensation_credit_kg_m2: 0.0,
                ending_pre_ingress_store_kg_m2: 0.4,
            })
            .expect_err("D/A/F"),
            DirectSurfaceLiquidErrorCode::E006,
        ),
        (
            validate_water_use(WaterUseOperands {
                request_kg_m2: 1.0,
                authorization_kg_m2: 0.5,
                finalized_use_kg_m2: 0.4,
                beginning_store_kg_m2: 1.0,
                condensation_credit_kg_m2: 0.0,
                ending_pre_ingress_store_kg_m2: 0.5,
            })
            .expect_err("pre-ingress closure"),
            DirectSurfaceLiquidErrorCode::E010,
        ),
        (missing_authorization, DirectSurfaceLiquidErrorCode::E005),
        (
            protocol_numeric_vector(-1.0, 0.0, 0.0, None)
                .validate()
                .expect_err("negative request"),
            DirectSurfaceLiquidErrorCode::E006,
        ),
        (
            protocol_numeric_vector(1.0, -1.0, 0.0, None)
                .validate()
                .expect_err("negative authorization"),
            DirectSurfaceLiquidErrorCode::E006,
        ),
        (
            protocol_numeric_vector(1.0, 1.0, -1.0, None)
                .validate()
                .expect_err("negative finalized use"),
            DirectSurfaceLiquidErrorCode::E006,
        ),
        (
            protocol_numeric_vector(1.0, 1.0, 1.0, Some(-1.0))
                .validate()
                .expect_err("negative condensation"),
            DirectSurfaceLiquidErrorCode::E006,
        ),
        (
            protocol_numeric_vector(1.0, 1.0, 1.0, Some(0.0))
                .validate()
                .expect_err("zero condensation"),
            DirectSurfaceLiquidErrorCode::E006,
        ),
        (
            protocol_numeric_vector(f64::NAN, 0.0, 0.0, None)
                .validate()
                .expect_err("nonfinite request"),
            DirectSurfaceLiquidErrorCode::E003,
        ),
    ];
    for (poison, expected) in cases.into_iter().chain(real_cases) {
        let error = execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &receiver_expectations(1, snapshot.clone()),
            &batch,
            &BTreeMap::new(),
            &ingress_input(),
            |_| Err(LandSurfaceEnergyShadowError::LandSurface(poison)),
        )
        .expect_err("callback poison");
        let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
            panic!("callback error escaped canonical boundary");
        };
        let failure = error.failure().expect("canonical failure");
        assert_eq!(failure.code, expected);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::ResourceCandidate);
        assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(snapshot.as_str())
        );
        assert!(failure.rollback.attempted_owner_sha256.is_some());
        assert_eq!(frame, original);
    }
}

#[test]
// This exhaustive boundary-error matrix is intentionally kept together for auditability.
#[allow(clippy::too_many_lines)]
fn raw_and_canonical_surface_errors_rebind_and_preserve_exact_lower_identity() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
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
    let raw = vec![
        (
            DirectSurfaceLiquidError::Schema("schema"),
            DirectSurfaceLiquidErrorCode::E001,
        ),
        (
            DirectSurfaceLiquidError::Identity("identity"),
            DirectSurfaceLiquidErrorCode::E002,
        ),
        (
            DirectSurfaceLiquidError::Domain("domain"),
            DirectSurfaceLiquidErrorCode::E003,
        ),
        (
            DirectSurfaceLiquidError::Protocol("protocol"),
            DirectSurfaceLiquidErrorCode::E005,
        ),
        (
            DirectSurfaceLiquidError::Bound("bound"),
            DirectSurfaceLiquidErrorCode::E006,
        ),
        (
            DirectSurfaceLiquidError::Closure("closure"),
            DirectSurfaceLiquidErrorCode::E010,
        ),
    ];
    for (poison, expected) in raw {
        let error = execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &receiver_expectations(1, snapshot.clone()),
            &batch,
            &BTreeMap::new(),
            &ingress_input(),
            |_| Err(poison.into()),
        )
        .expect_err("raw surface callback poison");
        let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
            panic!("canonical");
        };
        let failure = error.failure().expect("failure");
        assert_eq!(failure.code, expected);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::ResourceCandidate);
        assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
        assert_eq!(failure.context.owner_id, None);
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(snapshot.as_str())
        );
        assert!(failure.rollback.attempted_owner_sha256.is_some());
    }

    let lower_context = DirectSurfaceLiquidErrorContext {
        transaction_id: Some(TransactionId(999)),
        owner_id: Some(ResourceOwnerId::try_new("exact-lower-owner").expect("owner")),
        ofe_id: Some(OfeId::try_new("ofe-1").expect("OFE")),
        tile_id: Some(TileId::try_new("open").expect("tile")),
        surface_id: Some(SurfaceId::try_new("surface:ofe-1:open").expect("surface")),
        source_id: Some(SourceId::try_new("surface-liquid:ofe-1:open").expect("source")),
        parcel_id: None,
    };
    let poison = DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E011,
        DirectSurfaceLiquidPhase::Authorization,
        lower_context.clone(),
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some("lower-beginning".into()),
            attempted_owner_sha256: Some("lower-attempt".into()),
        },
        "canonical lower failure",
    );
    let error = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |_| Err(poison.into()),
    )
    .expect_err("canonical callback poison");
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
        panic!("canonical");
    };
    let failure = error.failure().expect("failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E011);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::ResourceCandidate);
    assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
    assert_eq!(failure.context.owner_id, lower_context.owner_id);
    assert_eq!(failure.context.ofe_id, lower_context.ofe_id);
    assert_eq!(failure.context.tile_id, lower_context.tile_id);
    assert_eq!(failure.context.surface_id, lower_context.surface_id);
    assert_eq!(failure.context.source_id, lower_context.source_id);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(snapshot.as_str())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
fn configured_infiltration_thermal_layer_must_be_first_before_callback() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
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
    for layers in [
        vec!["thermal-2", "thermal-1", "thermal-3"],
        vec!["thermal-2", "thermal-3"],
        vec!["thermal-other", "thermal-2", "thermal-3"],
    ] {
        let expectations = UnifiedReceiverExpectations::try_new(
            ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
            digest('2'),
            ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
            snapshot.clone(),
            ResourceOwnerId::try_new("soil-thermal").expect("thermal owner"),
            digest('4'),
            vec![(
                OfeId::try_new("ofe-1").expect("OFE"),
                TileId::try_new("open").expect("tile"),
                layers
                    .into_iter()
                    .map(|layer| SoilLayerId::try_new(layer).expect("layer"))
                    .collect(),
            )],
        )
        .expect("structural expectations");
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
                panic!("invalid layer order reached callback")
            },
        )
        .expect_err("wrong infiltration layer order");
        let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
            panic!("canonical");
        };
        let failure = error.failure().expect("failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E011);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::AtomicEnvelope);
        assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
        assert_eq!(
            failure
                .context
                .owner_id
                .as_ref()
                .map(ResourceOwnerId::as_str),
            Some("soil-thermal")
        );
        assert_eq!(
            failure.context.ofe_id.as_ref().map(OfeId::as_str),
            Some("ofe-1")
        );
        assert_eq!(
            failure.context.tile_id.as_ref().map(TileId::as_str),
            Some("open")
        );
        assert_eq!(callback_count.get(), 0);
    }
}
