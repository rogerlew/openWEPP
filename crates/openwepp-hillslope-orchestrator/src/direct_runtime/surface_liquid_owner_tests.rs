use super::*;

fn raw_state_hash(state: &DirectSurfaceLiquidOwnedState) -> String {
    super::super::surface_liquid_attachment::surface_liquid_raw_state_sha256(state)
}

fn rollback_attempt(error: &DirectSurfaceLiquidError) -> String {
    error
        .failure()
        .expect("canonical public failure")
        .rollback
        .attempted_owner_sha256
        .clone()
        .expect("complete attempted hash")
}

fn owner(value: &str) -> ResourceOwnerId {
    ResourceOwnerId::try_new(value).expect("valid owner")
}

fn tile(value: &str) -> TileId {
    TileId::try_new(value).expect("valid tile")
}

fn ofe(value: &str) -> OfeId {
    OfeId::try_new(value).expect("valid OFE")
}

fn surface(value: &str) -> SurfaceId {
    SurfaceId::try_new(value).expect("valid surface")
}

fn source(value: &str) -> SourceId {
    SourceId::try_new(value).expect("valid source")
}

fn layer(value: &str) -> SoilLayerId {
    SoilLayerId::try_new(value).expect("valid soil layer")
}

fn binding(ofe_id: &str, lane_index: usize) -> DirectSurfaceLiquidOfeBinding {
    let top_layer = layer(&format!("{ofe_id}-soil-1"));
    DirectSurfaceLiquidOfeBinding {
        ofe_id: ofe(ofe_id),
        production_lane_index: lane_index,
        production_lane_id: u32::try_from(lane_index + 1).expect("test lane id"),
        ordered_soil_layer_ids: vec![top_layer.clone(), layer(&format!("{ofe_id}-soil-2"))],
        infiltration_soil_thermal_layer_id: top_layer,
    }
}

fn record(
    ofe_id: &str,
    tile_id: &str,
    tile_fraction: f64,
    surface_class: SurfaceClass,
    source_type: WaterSourceType,
    ingress: DirectGroundIngressMode,
) -> DirectSurfaceLiquidConfigurationRecord {
    DirectSurfaceLiquidConfigurationRecord {
        key: DirectSurfaceLiquidStoreKey {
            run_id: 71,
            ofe_id: ofe(ofe_id),
            tile_id: tile(tile_id),
            surface_id: surface(&format!("surface-{tile_id}")),
            surface_class,
            source_type,
            source_id: source(&format!("source-{tile_id}")),
        },
        tile_fraction,
        capacity_kg_m2_tile: 2.0,
        ofe_area_m2: 100.0,
        ground_ingress_mode: ingress,
        runon_destination_ofe_id: None,
        runon_destination_tile_id: None,
    }
}

fn configuration() -> DirectSurfaceLiquidConfiguration {
    DirectSurfaceLiquidConfiguration::new(
        owner("hydrology"),
        71,
        vec![ofe("ofe-z")],
        vec![binding("ofe-z", 0)],
        vec![
            record(
                "ofe-z",
                "open",
                0.4,
                SurfaceClass::BareMineralSoil,
                WaterSourceType::SurfaceLiquid,
                DirectGroundIngressMode::OpenRawPrecipitation,
            ),
            record(
                "ofe-z",
                "covered",
                0.6,
                SurfaceClass::ForestLitter,
                WaterSourceType::LitterLiquid,
                DirectGroundIngressMode::CoveredCanopyRelease,
            ),
        ],
    )
    .expect("valid configuration")
}

fn state(configuration: &DirectSurfaceLiquidConfiguration) -> DirectSurfaceLiquidOwnedState {
    let liquid = configuration
        .records
        .iter()
        .map(|record| (record.key.clone(), 1.0))
        .collect();
    DirectSurfaceLiquidOwnedState::new_initial(configuration, &liquid, 3)
        .expect("valid initial state")
}

fn record_index(configuration: &DirectSurfaceLiquidConfiguration, tile_id: &str) -> usize {
    configuration
        .records
        .iter()
        .position(|record| record.key.tile_id == tile(tile_id))
        .expect("configured tile")
}

fn request(
    configuration: &DirectSurfaceLiquidConfiguration,
    record_index: usize,
    transaction_id: TransactionId,
    amount: f64,
) -> WaterAmount {
    let record = &configuration.records[record_index];
    WaterAmount {
        key: GroundWaterKey {
            transaction_id,
            requesting_owner_id: owner("land-surface-energy"),
            requesting_component: RequestingComponent::GroundSurface,
            ofe_id: record.key.ofe_id.clone(),
            requesting_tile_id: record.key.tile_id.clone(),
            occupancy_id: None,
            surface_id: Some(record.key.surface_id.clone()),
            surface_class: Some(record.key.surface_class),
            source_type: record.key.source_type,
            source_id: record.key.source_id.clone(),
            source_tile_id: Some(record.key.tile_id.clone()),
            soil_layer_id: None,
            amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        },
        amount_kg_m2_stand_ground: amount,
    }
}

fn attachment_frame() -> crate::DirectRunFrame {
    let identity = crate::DirectRunIdentity::new(71, 1, 1, 1).expect("frame identity");
    let mut frame = crate::DirectRunFrame::skeleton(identity).expect("frame");
    frame.lanes[0].area_m2 = 100.0;
    frame.lanes[0].subsurface_layers = vec![
        crate::DirectSubsurfaceLayerState::neutral(),
        crate::DirectSubsurfaceLayerState::neutral(),
    ];
    frame
}

fn assert_complete_e002(error: &DirectSurfaceLiquidError, tile_id: &TileId) {
    let failure = error.failure().expect("canonical E002 failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.context.tile_id.as_ref(), Some(tile_id));
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
fn complete_configuration_identity_set_precedes_record_domains_in_any_row_position() {
    for domain_index in 0..2 {
        let mut invalid = configuration();
        let identity_index = 1 - domain_index;
        let identity_tile = invalid.records[identity_index].key.tile_id.clone();
        invalid.records[domain_index].tile_fraction = f64::NAN;
        invalid.records[identity_index].key.run_id += 1;
        let error = invalid
            .validate()
            .expect_err("later wrong-run key must precede record domain");
        let failure = error.failure().expect("canonical configuration failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert_eq!(failure.context.tile_id.as_ref(), Some(&identity_tile));
    }
}

#[test]
fn complete_restart_identity_set_precedes_nan_and_overcapacity_in_any_row_position() {
    let configuration = configuration();
    for (domain_index, use_nan) in [(0, true), (1, false)] {
        let mut invalid = state(&configuration);
        let identity_index = 1 - domain_index;
        let identity_tile = invalid.records[identity_index].key.tile_id.clone();
        invalid.records[domain_index].liquid_kg_m2_tile = if use_nan {
            f64::NAN
        } else {
            configuration.records[domain_index].capacity_kg_m2_tile + 1.0
        };
        invalid.records[identity_index].key.run_id += 1;
        let error = invalid
            .validate(&configuration)
            .expect_err("later state key must precede record domain");
        let failure = error.failure().expect("canonical restart failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert_eq!(failure.context.tile_id.as_ref(), Some(&identity_tile));
        assert!(failure.rollback.beginning_owner_sha256.is_some());
    }
}

#[test]
fn attachment_and_authorization_preflight_complete_restart_identity_before_domains() {
    let configuration = configuration();
    for domain_index in 0..2 {
        let mut invalid = state(&configuration);
        let identity_index = 1 - domain_index;
        let identity_tile = invalid.records[identity_index].key.tile_id.clone();
        invalid.records[domain_index].liquid_kg_m2_tile = if domain_index == 0 {
            f64::NAN
        } else {
            configuration.records[domain_index].capacity_kg_m2_tile + 1.0
        };
        invalid.records[identity_index].key.run_id += 1;

        let mut frame = attachment_frame();
        let attachment = frame
            .configure_surface_liquid_shadow(&configuration, invalid.clone())
            .expect_err("attachment identity must precede state domain");
        assert_complete_e002(&attachment, &identity_tile);
        assert!(frame.surface_liquid_shadow.is_none());

        let authorization = authorize_surface_liquid_withdrawals(
            &configuration,
            &invalid,
            TransactionId(9_901),
            None,
            &[],
        )
        .expect_err("authorization identity must precede state domain");
        assert_complete_e002(&authorization, &identity_tile);
        assert!(
            authorization
                .failure()
                .expect("authorization failure")
                .rollback
                .beginning_owner_sha256
                .is_some()
        );
    }
}

#[test]
fn attachment_and_authorization_preflight_complete_configuration_identity_before_domains() {
    let valid = configuration();
    let beginning = state(&valid);
    for domain_index in 0..2 {
        let mut invalid = valid.clone();
        let identity_index = 1 - domain_index;
        let identity_tile = invalid.records[identity_index].key.tile_id.clone();
        invalid.records[domain_index].tile_fraction = f64::NAN;
        invalid.records[identity_index].key.run_id += 1;

        let mut frame = attachment_frame();
        let attachment = frame
            .configure_surface_liquid_shadow(&invalid, beginning.clone())
            .expect_err("attachment configuration identity before domain");
        assert_complete_e002(&attachment, &identity_tile);
        assert!(frame.surface_liquid_shadow.is_none());

        let authorization = authorize_surface_liquid_withdrawals(
            &invalid,
            &beginning,
            TransactionId(9_902),
            None,
            &[],
        )
        .expect_err("authorization configuration identity before domain");
        assert_complete_e002(&authorization, &identity_tile);
        assert!(
            authorization
                .failure()
                .expect("authorization failure")
                .rollback
                .beginning_owner_sha256
                .is_some()
        );
    }
}

#[test]
fn strict_configuration_and_state_round_trip_bind_topology_and_ingress() {
    let configuration = configuration();
    configuration.validate().expect("configuration");
    let state = state(&configuration);
    state.validate(&configuration).expect("state");

    let config_bytes = configuration.canonical_bytes().expect("serialize config");
    let parsed = DirectSurfaceLiquidConfiguration::from_canonical_bytes(&config_bytes)
        .expect("parse config");
    assert_eq!(parsed, configuration);
    let state_bytes = state
        .canonical_bytes(&configuration)
        .expect("serialize state");
    let parsed_state =
        DirectSurfaceLiquidOwnedState::from_canonical_bytes(&configuration, &state_bytes)
            .expect("parse state");
    assert_eq!(parsed_state, state);

    let mut changed = configuration.clone();
    let open = record_index(&changed, "open");
    changed.records[open].ground_ingress_mode = DirectGroundIngressMode::CoveredCanopyRelease;
    let error = changed.validate().expect_err("digest drift");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E002);
    let config_json = String::from_utf8(config_bytes).expect("canonical UTF-8");
    let with_unknown = config_json.replacen('{', r#"{"unknown":1,"#, 1);
    let error = DirectSurfaceLiquidConfiguration::from_canonical_bytes(with_unknown.as_bytes())
        .expect_err("unknown field");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E001);
}

#[test]
fn public_owner_hashes_bind_raw_invalid_state_request_and_resource_bits() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let open = record_index(&configuration, "open");

    let invalid_state_hash = |bits| {
        let mut invalid = beginning.clone();
        invalid.records[open].liquid_kg_m2_tile = f64::from_bits(bits);
        let declared = invalid.state_sha256.clone();
        let error = invalid
            .validate(&configuration)
            .expect_err("raw invalid accepted owner must fail");
        let failure = error.failure().expect("canonical owner failure");
        assert_ne!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(declared.as_str())
        );
        failure
            .rollback
            .beginning_owner_sha256
            .clone()
            .expect("raw beginning owner hash")
    };
    assert_ne!(
        invalid_state_hash(0x7ff8_0000_0000_0201),
        invalid_state_hash(0x7ff8_0000_0000_0202)
    );

    let transaction = TransactionId(920);
    let invalid_request_hash = |bits| {
        let invalid = request(&configuration, open, transaction, f64::from_bits(bits));
        rollback_attempt(
            &authorize_surface_liquid_withdrawals(
                &configuration,
                &beginning,
                transaction,
                None,
                &[invalid],
            )
            .expect_err("raw invalid request must fail"),
        )
    };
    assert_ne!(
        invalid_request_hash(0x7ff8_0000_0000_0211),
        invalid_request_hash(0x7ff8_0000_0000_0212)
    );

    let demand = request(&configuration, open, transaction, 0.2);
    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        std::slice::from_ref(&demand),
    )
    .expect("valid arbitration");
    let invalid_resource_hash = |bits| {
        let finalized = WaterAmount {
            key: demand.key.clone(),
            amount_kg_m2_stand_ground: f64::from_bits(bits),
        };
        rollback_attempt(
            &apply_surface_liquid_resource_phase(&configuration, &arbitration, &[finalized], &[])
                .expect_err("raw invalid finalized use must fail"),
        )
    };
    assert_ne!(
        invalid_resource_hash(0x7ff8_0000_0000_0221),
        invalid_resource_hash(0x7ff8_0000_0000_0222)
    );
}

#[test]
fn configuration_rejects_duplicate_tile_and_lexical_topology_inference() {
    let first = record(
        "ofe-z",
        "tile",
        1.0,
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        DirectGroundIngressMode::OpenRawPrecipitation,
    );
    let mut second = first.clone();
    second.key.surface_id = surface("other-surface");
    second.key.source_id = source("other-source");
    assert!(
        DirectSurfaceLiquidConfiguration::new(
            owner("hydrology"),
            71,
            vec![ofe("ofe-z")],
            vec![binding("ofe-z", 0)],
            vec![first, second],
        )
        .is_err()
    );

    let mut upstream = record(
        "z-upstream",
        "upstream",
        1.0,
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        DirectGroundIngressMode::OpenRawPrecipitation,
    );
    upstream.runon_destination_ofe_id = Some(ofe("a-downstream"));
    upstream.runon_destination_tile_id = Some(tile("downstream"));
    let mut downstream = record(
        "a-downstream",
        "downstream",
        1.0,
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        DirectGroundIngressMode::OpenRawPrecipitation,
    );
    upstream.key.run_id = 71;
    downstream.key.run_id = 71;
    let topology = DirectSurfaceLiquidConfiguration::new(
        owner("hydrology"),
        71,
        vec![ofe("z-upstream"), ofe("a-downstream")],
        vec![binding("z-upstream", 0), binding("a-downstream", 1)],
        vec![downstream, upstream],
    )
    .expect("explicit nonlexical topology");
    assert_eq!(topology.records[0].key.ofe_id, ofe("z-upstream"));
}

#[test]
fn same_store_requests_are_authorized_proportionally_from_one_snapshot() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(101);
    let open = record_index(&configuration, "open");
    let first = request(&configuration, open, transaction, 0.3);
    let mut second = first.clone();
    second.key.requesting_owner_id = owner("second-ground-component");
    second.amount_kg_m2_stand_ground = 0.5;
    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &[first, second],
    )
    .expect("authorize");
    assert!((arbitration.authorizations[0].amount_kg_m2_stand_ground - 0.15).abs() < 1.0e-15);
    assert!((arbitration.authorizations[1].amount_kg_m2_stand_ground - 0.25).abs() < 1.0e-15);
    assert_eq!(arbitration.beginning_state, beginning);
}

#[test]
fn three_equal_demands_use_exact_formula_bits_without_canonical_last_priority() {
    let configuration = DirectSurfaceLiquidConfiguration::new(
        owner("hydrology"),
        71,
        vec![ofe("ofe-z")],
        vec![binding("ofe-z", 0)],
        vec![record(
            "ofe-z",
            "unit-supply",
            1.0,
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            DirectGroundIngressMode::OpenRawPrecipitation,
        )],
    )
    .expect("unit-supply configuration");
    let beginning = state(&configuration);
    let transaction = TransactionId(118);
    let mut request_a = request(&configuration, 0, transaction, 1.0);
    request_a.key.requesting_owner_id = owner("request-a");
    let mut request_b = request_a.clone();
    request_b.key.requesting_owner_id = owner("request-b");
    let mut request_c = request_a.clone();
    request_c.key.requesting_owner_id = owner("request-c");

    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &[request_c.clone(), request_a.clone(), request_b.clone()],
    )
    .expect("three equal proportional authorizations");
    let expected = (1.0_f64 * 1.0_f64) / 3.0_f64;
    let actual_bits = arbitration
        .authorizations()
        .iter()
        .map(|authorization| authorization.amount_kg_m2_stand_ground.to_bits())
        .collect::<Vec<_>>();
    assert_eq!(actual_bits, vec![expected.to_bits(); 3]);
    assert!(arbitration.authorizations().iter().all(|authorization| {
        authorization.reason == WaterAuthorizationReason::ProportionalSupply
    }));

    let reversed = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &[request_b, request_a, request_c],
    )
    .expect("reversed equal-demand authorizations");
    let by_owner = |rows: &[WaterAuthorization]| {
        rows.iter()
            .map(|row| {
                (
                    row.key.requesting_owner_id.clone(),
                    row.amount_kg_m2_stand_ground.to_bits(),
                )
            })
            .collect::<BTreeMap<_, _>>()
    };
    assert_eq!(
        by_owner(arbitration.authorizations()),
        by_owner(reversed.authorizations())
    );
}

#[test]
fn joint_supply_rounding_uses_one_symmetric_scale_and_exact_final_debit() {
    let configuration = DirectSurfaceLiquidConfiguration::new(
        owner("hydrology"),
        71,
        vec![ofe("ofe-z")],
        vec![binding("ofe-z", 0)],
        vec![record(
            "ofe-z",
            "joint-supply",
            1.0,
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            DirectGroundIngressMode::OpenRawPrecipitation,
        )],
    )
    .expect("joint-supply configuration");
    let supply = 0.894_550_366_544_562_1_f64;
    let liquid = BTreeMap::from([(configuration.records[0].key.clone(), supply)]);
    let beginning = DirectSurfaceLiquidOwnedState::new_initial(&configuration, &liquid, 3)
        .expect("joint-supply beginning state");
    let transaction = TransactionId(119);
    let mut first = request(&configuration, 0, transaction, 1.550_567_735_753_300_3);
    first.key.requesting_owner_id = owner("request-a");
    let mut second = request(&configuration, 0, transaction, 0.666_084_441_700_219_5);
    second.key.requesting_owner_id = owner("request-z");

    let raw_sum = first.amount_kg_m2_stand_ground + second.amount_kg_m2_stand_ground;
    let raw = [
        first.amount_kg_m2_stand_ground * supply / raw_sum,
        second.amount_kg_m2_stand_ground * supply / raw_sum,
    ];
    assert!(raw.iter().sum::<f64>() > supply);
    let common_scale = supply / raw.iter().sum::<f64>();
    let expected = [raw[0] * common_scale, raw[1] * common_scale];
    assert!(expected.iter().sum::<f64>() <= supply);

    let forward = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &[first.clone(), second.clone()],
    )
    .expect("jointly safe forward authorization");
    let reverse = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &[second.clone(), first.clone()],
    )
    .expect("jointly safe reverse authorization");
    let by_owner = |rows: &[WaterAuthorization]| {
        rows.iter()
            .map(|row| {
                (
                    row.key.requesting_owner_id.clone(),
                    row.amount_kg_m2_stand_ground.to_bits(),
                )
            })
            .collect::<BTreeMap<_, _>>()
    };
    assert_eq!(
        by_owner(forward.authorizations()),
        by_owner(reverse.authorizations())
    );
    assert_eq!(
        by_owner(forward.authorizations()),
        BTreeMap::from([
            (first.key.requesting_owner_id.clone(), expected[0].to_bits()),
            (
                second.key.requesting_owner_id.clone(),
                expected[1].to_bits()
            ),
        ])
    );
    let authorized_sum = forward
        .authorizations()
        .iter()
        .map(|row| row.amount_kg_m2_stand_ground)
        .sum::<f64>();
    assert!(authorized_sum <= supply);
    let canonical_last_remainder = supply - expected[0];
    assert_ne!(expected[1].to_bits(), canonical_last_remainder.to_bits());

    let finalized = forward
        .authorizations()
        .iter()
        .map(|authorization| WaterAmount {
            key: authorization.key.clone(),
            amount_kg_m2_stand_ground: authorization.amount_kg_m2_stand_ground,
        })
        .collect::<Vec<_>>();
    let candidate = apply_surface_liquid_resource_phase(&configuration, &forward, &finalized, &[])
        .expect("exact F=A candidate");
    assert_eq!(
        candidate.working_state().records[0]
            .liquid_kg_m2_tile
            .to_bits(),
        (supply - authorized_sum).to_bits()
    );
    candidate
        .validate(&configuration)
        .expect("joint-supply candidate closure");
}

#[test]
fn three_distinct_finalized_uses_debit_in_key_order_not_caller_order() {
    let configuration = DirectSurfaceLiquidConfiguration::new(
        owner("hydrology"),
        71,
        vec![ofe("ofe-z")],
        vec![binding("ofe-z", 0)],
        vec![record(
            "ofe-z",
            "ordered-final-use",
            1.0,
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            DirectGroundIngressMode::OpenRawPrecipitation,
        )],
    )
    .expect("ordered-final-use configuration");
    let beginning = state(&configuration);
    let transaction = TransactionId(120);
    let mut first = request(&configuration, 0, transaction, 0.1);
    first.key.requesting_owner_id = owner("request-a");
    let mut second = request(&configuration, 0, transaction, 0.2);
    second.key.requesting_owner_id = owner("request-b");
    let mut third = request(&configuration, 0, transaction, 0.3);
    third.key.requesting_owner_id = owner("request-c");

    let run = |requests: Vec<WaterAmount>| {
        let arbitration = authorize_surface_liquid_withdrawals(
            &configuration,
            &beginning,
            transaction,
            None,
            &requests,
        )
        .expect("full-supply authorization");
        let finalized = arbitration
            .authorizations()
            .iter()
            .map(|authorization| WaterAmount {
                key: authorization.key.clone(),
                amount_kg_m2_stand_ground: authorization.amount_kg_m2_stand_ground,
            })
            .collect::<Vec<_>>();
        let caller_sum = finalized
            .iter()
            .map(|row| row.amount_kg_m2_stand_ground)
            .sum::<f64>();
        let candidate =
            apply_surface_liquid_resource_phase(&configuration, &arbitration, &finalized, &[])
                .expect("canonical final-use debit");
        candidate
            .validate(&configuration)
            .expect("independent canonical reconstruction");
        (caller_sum, candidate)
    };

    let (forward_sum, forward) = run(vec![first.clone(), second.clone(), third.clone()]);
    let (reverse_sum, reverse) = run(vec![third, second, first]);
    assert_ne!(forward_sum.to_bits(), reverse_sum.to_bits());
    assert_eq!(
        forward.working_state().records[0]
            .liquid_kg_m2_tile
            .to_bits(),
        reverse.working_state().records[0]
            .liquid_kg_m2_tile
            .to_bits()
    );
    let canonical_sum = (0.1_f64 + 0.2_f64) + 0.3_f64;
    assert_eq!(
        forward.working_state().records[0]
            .liquid_kg_m2_tile
            .to_bits(),
        (1.0_f64 - canonical_sum).to_bits()
    );
}

#[test]
fn tiny_positive_same_store_oversubscription_fails_closed_without_key_priority() {
    let configuration = configuration();
    let open = record_index(&configuration, "open");
    let liquid = configuration
        .records
        .iter()
        .enumerate()
        .map(|(index, record)| {
            (
                record.key.clone(),
                if index == open { 1.0e-200 } else { 1.0 },
            )
        })
        .collect();
    let beginning = DirectSurfaceLiquidOwnedState::new_initial(&configuration, &liquid, 3)
        .expect("valid tiny-positive initial state");
    let transaction = TransactionId(114);
    let mut first = request(&configuration, open, transaction, 1.0e-200);
    first.key.requesting_owner_id = owner("tiny-first");
    let mut second = first.clone();
    second.key.requesting_owner_id = owner("tiny-second");

    let error = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &[first.clone(), second],
    )
    .expect_err("nonzero proportional numerator underflow must fail closed");
    let failure = error.failure().expect("canonical underflow failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Authorization);
    assert_eq!(failure.context.transaction_id, Some(transaction));
    assert_eq!(
        failure.context.owner_id,
        Some(first.key.requesting_owner_id)
    );
    assert_eq!(failure.context.ofe_id, Some(first.key.ofe_id));
    assert_eq!(failure.context.tile_id, first.key.source_tile_id);
    assert_eq!(failure.context.surface_id, first.key.surface_id);
    assert_eq!(failure.context.source_id, Some(first.key.source_id));
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(raw_state_hash(&beginning).as_str())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
fn canonical_last_tiny_request_is_checked_before_remainder_in_any_caller_order() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(116);
    let open = record_index(&configuration, "open");
    let mut large = request(&configuration, open, transaction, 1.0);
    large.key.requesting_owner_id = owner("aa-large-request");
    let mut tiny = request(&configuration, open, transaction, f64::from_bits(1));
    tiny.key.requesting_owner_id = owner("zz-tiny-request");
    assert!(large.key < tiny.key, "tiny request must be canonical last");

    for requests in [
        vec![large.clone(), tiny.clone()],
        vec![tiny.clone(), large.clone()],
    ] {
        let error = authorize_surface_liquid_withdrawals(
            &configuration,
            &beginning,
            transaction,
            None,
            &requests,
        )
        .expect_err("canonical-last tiny share must be checked before remainder assignment");
        let failure = error.failure().expect("canonical tiny-share failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Authorization);
        assert_eq!(failure.context.transaction_id, Some(transaction));
        assert_eq!(
            failure.context.owner_id,
            Some(tiny.key.requesting_owner_id.clone())
        );
        assert_eq!(failure.context.ofe_id, Some(tiny.key.ofe_id.clone()));
        assert_eq!(failure.context.tile_id, tiny.key.source_tile_id.clone());
        assert_eq!(failure.context.surface_id, tiny.key.surface_id.clone());
        assert_eq!(failure.context.source_id, Some(tiny.key.source_id.clone()));
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(raw_state_hash(&beginning).as_str())
        );
        assert!(failure.rollback.attempted_owner_sha256.is_some());
    }
}

#[test]
fn same_store_finite_demand_overflow_fails_before_authorization_or_candidate() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(111);
    let open = record_index(&configuration, "open");
    let mut first = request(&configuration, open, transaction, f64::MAX * 0.75);
    first.key.requesting_owner_id = owner("overflow-first");
    let mut second = first.clone();
    second.key.requesting_owner_id = owner("overflow-second");
    let requests = vec![first.clone(), second.clone()];

    for caller_order in [requests.clone(), vec![second.clone(), first.clone()]] {
        let error = authorize_surface_liquid_withdrawals(
            &configuration,
            &beginning,
            transaction,
            None,
            &caller_order,
        )
        .expect_err("finite requests with an infinite canonical sum must fail closed");
        let failure = error.failure().expect("canonical overflow failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Authorization);
        assert_eq!(failure.context.transaction_id, Some(transaction));
        assert_eq!(failure.context.owner_id, Some(owner("overflow-first")));
        assert_eq!(failure.context.ofe_id, Some(first.key.ofe_id.clone()));
        assert_eq!(failure.context.tile_id, first.key.source_tile_id.clone());
        assert_eq!(failure.context.surface_id, first.key.surface_id.clone());
        assert_eq!(failure.context.source_id, Some(first.key.source_id.clone()));
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(raw_state_hash(&beginning).as_str())
        );
    }

    let store = configuration
        .store_key_for_water(&first.key)
        .expect("configured store");
    let forged = DirectSurfaceLiquidArbitration {
        transaction_id: transaction,
        expected_predecessor: None,
        beginning_state: beginning,
        requests,
        authorizations: vec![
            WaterAuthorization {
                key: first.key,
                amount_kg_m2_stand_ground: 0.0,
                reason: WaterAuthorizationReason::DrySource,
            },
            WaterAuthorization {
                key: second.key,
                amount_kg_m2_stand_ground: 0.0,
                reason: WaterAuthorizationReason::DrySource,
            },
        ],
        request_store_keys: vec![store.clone(), store],
    };
    let error = apply_surface_liquid_resource_phase(
        &configuration,
        &forged,
        &[
            WaterAmount {
                key: forged.requests[0].key.clone(),
                amount_kg_m2_stand_ground: 0.0,
            },
            WaterAmount {
                key: forged.requests[1].key.clone(),
                amount_kg_m2_stand_ground: 0.0,
            },
        ],
        &[],
    )
    .expect_err("candidate boundary must independently reject overflow");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
}

#[test]
fn adjacent_large_finite_same_store_demands_authorize_proportionally() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(112);
    let open = record_index(&configuration, "open");
    let mut first = request(&configuration, open, transaction, f64::MAX / 4.0);
    first.key.requesting_owner_id = owner("large-first");
    let mut second = first.clone();
    second.key.requesting_owner_id = owner("large-second");
    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &[first, second],
    )
    .expect("large finite nonoverflow total");
    assert!(arbitration.authorizations.iter().all(|row| {
        row.amount_kg_m2_stand_ground.is_finite() && row.amount_kg_m2_stand_ground > 0.0
    }));
    let authorized = arbitration
        .authorizations
        .iter()
        .map(|row| row.amount_kg_m2_stand_ground)
        .sum::<f64>();
    let record = &configuration.records[open];
    let supply = record.tile_fraction * beginning.records[open].liquid_kg_m2_tile;
    assert!((authorized - supply).abs() <= 4.0 * f64::EPSILON * supply.abs());
}

#[test]
fn resource_candidate_debits_finalized_use_and_credits_signed_condensation() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(102);
    let covered = record_index(&configuration, "covered");
    let demand = request(&configuration, covered, transaction, 0.3);
    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        std::slice::from_ref(&demand),
    )
    .expect("authorize");
    let finalized = WaterAmount {
        key: demand.key.clone(),
        amount_kg_m2_stand_ground: 0.12,
    };
    let record = &configuration.records[covered];
    let condensation = CondensationCredit {
        transaction_id: transaction,
        hydrology_owner_id: configuration.owner_id.clone(),
        ofe_id: record.key.ofe_id.clone(),
        tile_id: record.key.tile_id.clone(),
        surface_id: record.key.surface_id.clone(),
        amount_kg_m2_stand_ground: 0.06,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        temperature_k: 285.0,
        specific_liquid_enthalpy_j_kg: openwepp_land_surface_energy::liquid_enthalpy_j_kg(285.0),
    };
    let candidate = apply_surface_liquid_resource_phase(
        &configuration,
        &arbitration,
        &[finalized],
        &[condensation],
    )
    .expect("resource candidate");
    assert_eq!(candidate.beginning_state, beginning);
    assert!((candidate.working_state.records[covered].liquid_kg_m2_tile - 0.9).abs() < 1.0e-15);
    assert!(candidate.condensation_overflow.is_empty());
}

#[test]
fn condensation_rejects_one_bit_temperature_enthalpy_alias() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(105);
    let arbitration =
        authorize_surface_liquid_withdrawals(&configuration, &beginning, transaction, None, &[])
            .expect("authorize empty");
    for record in &configuration.records {
        for poison in 0..2 {
            let exact = openwepp_land_surface_energy::liquid_enthalpy_j_kg(280.0);
            let mut condensation = CondensationCredit {
                transaction_id: transaction,
                hydrology_owner_id: configuration.owner_id.clone(),
                ofe_id: record.key.ofe_id.clone(),
                tile_id: record.key.tile_id.clone(),
                surface_id: record.key.surface_id.clone(),
                amount_kg_m2_stand_ground: 0.1,
                amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
                temperature_k: 280.0,
                specific_liquid_enthalpy_j_kg: exact,
            };
            match poison {
                0 => {
                    condensation.temperature_k =
                        f64::from_bits(condensation.temperature_k.to_bits() + 1);
                }
                1 => {
                    condensation.specific_liquid_enthalpy_j_kg =
                        f64::from_bits(exact.to_bits() + 1);
                }
                _ => unreachable!("bounded condensation poison table"),
            }
            let expected_attempt =
                super::super::surface_liquid_attachment::surface_liquid_raw_resource_attempt_sha256(
                    &configuration,
                    &arbitration,
                    &[],
                    std::slice::from_ref(&condensation),
                );
            let error = apply_surface_liquid_resource_phase(
                &configuration,
                &arbitration,
                &[],
                &[condensation],
            )
            .expect_err("temperature/enthalpy poison");
            let failure = error.failure().expect("canonical condensation failure");
            assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E009);
            assert_eq!(failure.phase, DirectSurfaceLiquidPhase::ResourceCandidate);
            assert_eq!(failure.context.transaction_id, Some(transaction));
            assert_eq!(
                failure.context.owner_id.as_ref(),
                Some(&configuration.owner_id)
            );
            assert_eq!(failure.context.ofe_id.as_ref(), Some(&record.key.ofe_id));
            assert_eq!(failure.context.tile_id.as_ref(), Some(&record.key.tile_id));
            assert_eq!(
                failure.context.surface_id.as_ref(),
                Some(&record.key.surface_id),
            );
            assert_eq!(
                failure.context.source_id.as_ref(),
                Some(&record.key.source_id)
            );
            assert_eq!(
                failure.rollback.beginning_owner_sha256.as_deref(),
                Some(raw_state_hash(&beginning).as_str()),
            );
            assert_eq!(
                failure.rollback.attempted_owner_sha256.as_deref(),
                Some(expected_attempt.as_str()),
            );
        }
    }
    assert_eq!(arbitration.beginning_state, beginning);
}

#[test]
fn resource_candidate_rejects_authorization_as_finalized_use_and_preserves_beginning() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(103);
    let open = record_index(&configuration, "open");
    let demand = request(&configuration, open, transaction, 0.8);
    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        std::slice::from_ref(&demand),
    )
    .expect("authorize");
    let invalid = WaterAmount {
        key: demand.key.clone(),
        amount_kg_m2_stand_ground: 0.5,
    };
    let error = apply_surface_liquid_resource_phase(&configuration, &arbitration, &[invalid], &[])
        .expect_err("authorization is not finalized use");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E006);
    assert_eq!(arbitration.beginning_state, beginning);
}

#[test]
fn condensation_over_capacity_is_a_typed_ingress_parcel_not_clipped() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(104);
    let arbitration =
        authorize_surface_liquid_withdrawals(&configuration, &beginning, transaction, None, &[])
            .expect("authorize empty");
    let open = record_index(&configuration, "open");
    let record = &configuration.records[open];
    let condensation = CondensationCredit {
        transaction_id: transaction,
        hydrology_owner_id: configuration.owner_id.clone(),
        ofe_id: record.key.ofe_id.clone(),
        tile_id: record.key.tile_id.clone(),
        surface_id: record.key.surface_id.clone(),
        amount_kg_m2_stand_ground: 0.8,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        temperature_k: 280.0,
        specific_liquid_enthalpy_j_kg: openwepp_land_surface_energy::liquid_enthalpy_j_kg(280.0),
    };
    let candidate =
        apply_surface_liquid_resource_phase(&configuration, &arbitration, &[], &[condensation])
            .expect("overflow candidate");
    assert!((candidate.working_state.records[open].liquid_kg_m2_tile - 2.0).abs() < f64::EPSILON);
    assert_eq!(candidate.condensation_overflow.len(), 1);
    assert!((candidate.condensation_overflow[0].amount_kg_m2_ofe_ground - 0.4).abs() < 1.0e-15);
}

#[test]
fn finite_condensation_area_conversion_overflow_fails_before_candidate() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(113);
    let arbitration =
        authorize_surface_liquid_withdrawals(&configuration, &beginning, transaction, None, &[])
            .expect("authorize empty");
    let open = record_index(&configuration, "open");
    let record = &configuration.records[open];
    let temperature_k = 280.0;
    let condensation = CondensationCredit {
        transaction_id: transaction,
        hydrology_owner_id: configuration.owner_id.clone(),
        ofe_id: record.key.ofe_id.clone(),
        tile_id: record.key.tile_id.clone(),
        surface_id: record.key.surface_id.clone(),
        amount_kg_m2_stand_ground: f64::MAX / 2.0,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        temperature_k,
        specific_liquid_enthalpy_j_kg: openwepp_land_surface_energy::liquid_enthalpy_j_kg(
            temperature_k,
        ),
    };
    let error =
        apply_surface_liquid_resource_phase(&configuration, &arbitration, &[], &[condensation])
            .expect_err("finite condensation conversion overflow must fail closed");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(arbitration.beginning_state, beginning);
}

#[test]
fn checked_unit_closure_rejects_large_finite_tolerance_overflow_and_underflow() {
    assert_eq!(
        checked_surface_liquid_close(
            f64::MAX,
            f64::MAX / 2.0,
            DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
        ),
        None
    );
    assert_eq!(
        checked_surface_liquid_mul(f64::MIN_POSITIVE, f64::MIN_POSITIVE),
        None
    );
    assert_eq!(
        checked_surface_liquid_div(f64::MIN_POSITIVE, f64::MAX),
        None
    );
}

#[test]
fn every_configuration_and_binding_field_changes_the_digest() {
    let original = configuration();
    let open = record_index(&original, "open");
    let digest = original.recomputed_sha256().expect("digest");
    let mut mutations = Vec::new();
    let mut changed = original.clone();
    changed.owner_id = owner("other-owner");
    mutations.push(changed);
    let mut changed = original.clone();
    changed.run_id += 1;
    mutations.push(changed);
    let mut changed = original.clone();
    changed.ofe_topology[0] = ofe("other-ofe");
    mutations.push(changed);
    let mut changed = original.clone();
    changed.ofe_bindings[0].ofe_id = ofe("other-ofe");
    mutations.push(changed);
    let mut changed = original.clone();
    changed.ofe_bindings[0].production_lane_index += 1;
    mutations.push(changed);
    let mut changed = original.clone();
    changed.ofe_bindings[0].production_lane_id += 1;
    mutations.push(changed);
    let mut changed = original.clone();
    changed.ofe_bindings[0].ordered_soil_layer_ids.swap(0, 1);
    mutations.push(changed);
    let mut changed = original.clone();
    changed.ofe_bindings[0].infiltration_soil_thermal_layer_id = layer("other-layer");
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].key.run_id += 1;
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].key.ofe_id = ofe("other-ofe");
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].key.tile_id = tile("other-tile");
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].key.surface_id = surface("other-surface");
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].key.surface_class = SurfaceClass::ForestLitter;
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].key.source_type = WaterSourceType::LitterLiquid;
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].key.source_id = source("other-source");
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].tile_fraction =
        f64::from_bits(changed.records[open].tile_fraction.to_bits() + 1);
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].capacity_kg_m2_tile =
        f64::from_bits(changed.records[open].capacity_kg_m2_tile.to_bits() + 1);
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].ofe_area_m2 =
        f64::from_bits(changed.records[open].ofe_area_m2.to_bits() + 1);
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].ground_ingress_mode = DirectGroundIngressMode::CoveredCanopyRelease;
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].runon_destination_ofe_id = Some(ofe("destination"));
    mutations.push(changed);
    let mut changed = original.clone();
    changed.records[open].runon_destination_tile_id = Some(tile("destination"));
    mutations.push(changed);

    for changed in mutations {
        assert_ne!(changed.recomputed_sha256().expect("mutated digest"), digest);
    }
}

#[test]
fn canonical_bytes_keep_schema_names_and_exact_bit_hex() {
    let configuration = configuration();
    let bytes = configuration.canonical_bytes().expect("canonical bytes");
    let text = std::str::from_utf8(&bytes).expect("UTF-8 JSON");
    assert!(text.contains(r#""tile_fraction":"3fd999999999999a""#));
    assert!(text.contains(r#""capacity_kg_m2_tile":"4000000000000000""#));
    assert!(text.contains(r#""ofe_bindings""#));
    assert!(!text.contains("_bits"));
    assert_eq!(
        DirectSurfaceLiquidConfiguration::from_canonical_bytes(&bytes)
            .expect("canonical round trip"),
        configuration
    );
}

#[test]
fn valid_noncanonical_bytes_preserve_parsed_public_failure_context() {
    let configuration = configuration();
    let mut configuration_bytes = configuration
        .canonical_bytes()
        .expect("canonical configuration bytes");
    configuration_bytes.push(b' ');
    let failure = DirectSurfaceLiquidConfiguration::from_canonical_bytes(&configuration_bytes)
        .expect_err("trailing whitespace is valid JSON but noncanonical")
        .failure()
        .expect("contextual noncanonical configuration failure")
        .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E001);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Configuration);
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(failure.context.transaction_id, None);
    assert_eq!(failure.rollback.beginning_owner_sha256, None);
    let configuration_attempted_sha256 = failure
        .rollback
        .attempted_owner_sha256
        .expect("raw configuration attempt hash");
    let mut alternate_configuration_bytes = configuration
        .canonical_bytes()
        .expect("canonical configuration bytes");
    alternate_configuration_bytes.push(b'\n');
    let alternate_configuration_hash =
        DirectSurfaceLiquidConfiguration::from_canonical_bytes(&alternate_configuration_bytes)
            .expect_err("alternate trailing whitespace is noncanonical")
            .failure()
            .expect("alternate configuration failure")
            .rollback
            .attempted_owner_sha256
            .clone()
            .expect("alternate raw configuration attempt hash");
    assert_ne!(configuration_attempted_sha256, alternate_configuration_hash);

    let transaction = TransactionId(117);
    let mut accepted = state(&configuration);
    for record in &mut accepted.records {
        record.last_accepted_transaction_id = Some(transaction);
    }
    for continuation in &mut accepted.continuations {
        continuation.next_interval_index = 1;
        continuation.last_accepted_transaction_id = Some(transaction);
    }
    accepted.state_sha256 = accepted.recomputed_sha256().expect("accepted state digest");
    accepted
        .validate(&configuration)
        .expect("valid accepted state");
    let mut state_bytes = accepted
        .canonical_bytes(&configuration)
        .expect("canonical accepted state bytes");
    state_bytes.push(b'\n');
    let failure = DirectSurfaceLiquidOwnedState::from_canonical_bytes(&configuration, &state_bytes)
        .expect_err("trailing whitespace is valid JSON but noncanonical")
        .failure()
        .expect("contextual noncanonical restart failure")
        .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E001);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Restart);
    assert_eq!(failure.context.transaction_id, Some(transaction));
    assert_eq!(failure.context.owner_id, Some(accepted.owner_id.clone()));
    assert_eq!(failure.context.ofe_id, None);
    assert_eq!(failure.context.tile_id, None);
    assert_eq!(failure.context.surface_id, None);
    assert_eq!(failure.context.source_id, None);
    assert_eq!(failure.rollback.beginning_owner_sha256, None);
    let state_attempted_sha256 = failure
        .rollback
        .attempted_owner_sha256
        .expect("raw state attempt hash");
    let mut alternate_state_bytes = accepted
        .canonical_bytes(&configuration)
        .expect("canonical accepted state bytes");
    alternate_state_bytes.push(b' ');
    let alternate_state_hash =
        DirectSurfaceLiquidOwnedState::from_canonical_bytes(&configuration, &alternate_state_bytes)
            .expect_err("alternate trailing whitespace is noncanonical")
            .failure()
            .expect("alternate state failure")
            .rollback
            .attempted_owner_sha256
            .clone()
            .expect("alternate raw state attempt hash");
    assert_ne!(state_attempted_sha256, alternate_state_hash);
}

#[test]
fn state_digest_binds_every_restart_field_and_rejects_invalid_combinations() {
    let configuration = configuration();
    let initial = state(&configuration);
    let digest = initial.recomputed_sha256().expect("state digest");
    let mut mutations = Vec::new();
    let mut changed = initial.clone();
    changed.owner_id = owner("other-owner");
    mutations.push(changed);
    let mut changed = initial.clone();
    changed.configuration_sha256 = "1".repeat(64);
    mutations.push(changed);
    let mut changed = initial.clone();
    changed.records[0].key.tile_id = tile("other-tile");
    mutations.push(changed);
    let mut changed = initial.clone();
    changed.records[0].liquid_kg_m2_tile =
        f64::from_bits(changed.records[0].liquid_kg_m2_tile.to_bits() + 1);
    mutations.push(changed);
    let mut changed = initial.clone();
    changed.records[0].last_accepted_transaction_id = Some(TransactionId(1));
    mutations.push(changed);
    let mut changed = initial.clone();
    changed.continuations[0].ofe_id = ofe("other-ofe");
    mutations.push(changed);
    let mut changed = initial.clone();
    changed.continuations[0].day_index += 1;
    mutations.push(changed);
    let mut changed = initial.clone();
    changed.continuations[0].next_interval_index = 1;
    mutations.push(changed);
    let mut changed = initial.clone();
    changed.continuations[0].cumulative_supply_m = f64::from_bits(1);
    mutations.push(changed);
    let mut changed = initial.clone();
    changed.continuations[0].cumulative_infiltration_m = f64::from_bits(1);
    mutations.push(changed);
    let mut changed = initial.clone();
    changed.continuations[0].last_accepted_transaction_id = Some(TransactionId(1));
    mutations.push(changed);
    for changed in mutations {
        assert_ne!(changed.recomputed_sha256().expect("mutated digest"), digest);
    }

    let mut invalid_initial = initial.clone();
    invalid_initial.continuations[0].next_interval_index = 1;
    invalid_initial.state_sha256 = invalid_initial.recomputed_sha256().expect("digest");
    assert_eq!(
        invalid_initial
            .validate(&configuration)
            .expect_err("invalid initial restart")
            .code(),
        DirectSurfaceLiquidErrorCode::E008
    );

    let mut invalid_accepted = initial;
    for record in &mut invalid_accepted.records {
        record.last_accepted_transaction_id = Some(TransactionId(7));
    }
    for continuation in &mut invalid_accepted.continuations {
        continuation.last_accepted_transaction_id = Some(TransactionId(7));
    }
    invalid_accepted.state_sha256 = invalid_accepted.recomputed_sha256().expect("digest");
    assert_eq!(
        invalid_accepted
            .validate(&configuration)
            .expect_err("accepted interval zero")
            .code(),
        DirectSurfaceLiquidErrorCode::E008
    );
}

#[test]
fn configuration_and_restart_record_failures_preserve_available_identity() {
    let configuration = configuration();
    let open = record_index(&configuration, "open");
    let expected_key = configuration.records[open].key.clone();
    let mut invalid_configuration = configuration.clone();
    invalid_configuration.records[open].capacity_kg_m2_tile = f64::NAN;
    let failure = invalid_configuration
        .validate()
        .expect_err("invalid record capacity")
        .failure()
        .expect("contextual configuration failure")
        .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Configuration);
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(failure.context.ofe_id, Some(expected_key.ofe_id.clone()));
    assert_eq!(failure.context.tile_id, Some(expected_key.tile_id.clone()));
    assert_eq!(
        failure.context.surface_id,
        Some(expected_key.surface_id.clone())
    );
    assert_eq!(
        failure.context.source_id,
        Some(expected_key.source_id.clone())
    );
    assert_eq!(failure.context.transaction_id, None);

    let transaction = TransactionId(115);
    let mut accepted = state(&configuration);
    for record in &mut accepted.records {
        record.last_accepted_transaction_id = Some(transaction);
    }
    for continuation in &mut accepted.continuations {
        continuation.next_interval_index = 1;
        continuation.last_accepted_transaction_id = Some(transaction);
    }
    accepted.state_sha256 = accepted.recomputed_sha256().expect("accepted digest");
    accepted
        .validate(&configuration)
        .expect("valid accepted state");
    accepted.records[open].liquid_kg_m2_tile = f64::NAN;
    let failure = accepted
        .validate(&configuration)
        .expect_err("invalid restart store")
        .failure()
        .expect("contextual restart failure")
        .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Restart);
    assert_eq!(failure.context.transaction_id, Some(transaction));
    assert_eq!(failure.context.owner_id, Some(configuration.owner_id));
    assert_eq!(failure.context.ofe_id, Some(expected_key.ofe_id));
    assert_eq!(failure.context.tile_id, Some(expected_key.tile_id));
    assert_eq!(failure.context.surface_id, Some(expected_key.surface_id));
    assert_eq!(failure.context.source_id, Some(expected_key.source_id));
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(raw_state_hash(&accepted).as_str())
    );
    assert_eq!(failure.rollback.attempted_owner_sha256, None);
}

#[test]
fn sealed_candidate_revalidation_rejects_forged_and_stale_state_with_e009() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        TransactionId(901),
        None,
        &[],
    )
    .expect("arbitration");
    let candidate = apply_surface_liquid_resource_phase(&configuration, &arbitration, &[], &[])
        .expect("candidate");
    let mut forged = candidate.clone();
    forged.working_state.records[0].liquid_kg_m2_tile += 0.25;
    let error = forged
        .validate(&configuration)
        .expect_err("forged working state");
    let failure = error.failure().expect("canonical payload");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E010);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::ResourceCandidate);
    assert_eq!(failure.context.transaction_id, Some(TransactionId(901)));
    assert_eq!(
        failure.rollback.beginning_owner_sha256,
        Some(raw_state_hash(&beginning))
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());

    let mut stale = candidate;
    stale.beginning_state.configuration_sha256 = "1".repeat(64);
    assert_eq!(
        stale
            .validate(&configuration)
            .expect_err("stale candidate")
            .code(),
        DirectSurfaceLiquidErrorCode::E002
    );
}

#[test]
fn canonical_error_codes_cover_e001_through_e011_exactly() {
    let codes = [
        DirectSurfaceLiquidErrorCode::E001,
        DirectSurfaceLiquidErrorCode::E002,
        DirectSurfaceLiquidErrorCode::E003,
        DirectSurfaceLiquidErrorCode::E004,
        DirectSurfaceLiquidErrorCode::E005,
        DirectSurfaceLiquidErrorCode::E006,
        DirectSurfaceLiquidErrorCode::E007,
        DirectSurfaceLiquidErrorCode::E008,
        DirectSurfaceLiquidErrorCode::E009,
        DirectSurfaceLiquidErrorCode::E010,
        DirectSurfaceLiquidErrorCode::E011,
    ];
    for (index, code) in codes.into_iter().enumerate() {
        assert_eq!(code.as_str(), format!("SURFACELIQUID-E-{:03}", index + 1));
    }
}

#[test]
fn public_failure_constructors_carry_identity_and_rollback_context() {
    let context = DirectSurfaceLiquidErrorContext {
        transaction_id: Some(TransactionId(902)),
        owner_id: Some(owner("hydrology-owner")),
        ofe_id: Some(ofe("ofe-a")),
        tile_id: Some(tile("tile-a")),
        surface_id: Some(surface("surface-a")),
        source_id: Some(source("source-a")),
        parcel_id: Some("parcel-a".into()),
    };
    let failures = [
        DirectSurfaceLiquidError::unsupported_domain_failure(
            DirectSurfaceLiquidPhase::IngressCandidate,
            context.clone(),
            Some("a".repeat(64)),
            "snow-present surface",
        ),
        DirectSurfaceLiquidError::exact_one_owner_failure(
            DirectSurfaceLiquidPhase::AtomicEnvelope,
            context.clone(),
            Some("b".repeat(64)),
            Some("c".repeat(64)),
            "legacy depression retention is nonzero",
        ),
        DirectSurfaceLiquidError::atomic_envelope_failure(
            context.clone(),
            Some("d".repeat(64)),
            Some("e".repeat(64)),
            "complete owner mismatch",
        ),
    ];
    for (error, expected) in failures.into_iter().zip([
        DirectSurfaceLiquidErrorCode::E004,
        DirectSurfaceLiquidErrorCode::E007,
        DirectSurfaceLiquidErrorCode::E011,
    ]) {
        let failure = error.failure().expect("canonical runtime payload");
        assert_eq!(failure.code, expected);
        assert_eq!(failure.context, context);
        assert!(failure.rollback.beginning_owner_sha256.is_some());
        let bytes = serde_json::to_vec(failure).expect("serialize typed failure");
        let round_trip: DirectSurfaceLiquidFailure =
            serde_json::from_slice(&bytes).expect("parse typed failure");
        assert_eq!(round_trip, *failure);
        let mut wrong_owner = round_trip;
        wrong_owner.context.owner_id = Some(owner("wrong-owner"));
        assert_ne!(wrong_owner, *failure);
    }
}

#[test]
fn invalid_restart_cannot_emit_canonical_persistence_bytes() {
    let configuration = configuration();
    let mut invalid = state(&configuration);
    invalid.records[0].liquid_kg_m2_tile = f64::NAN;
    let error = invalid
        .canonical_bytes(&configuration)
        .expect_err("invalid state must not serialize");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(
        error.failure().expect("contextual failure").phase,
        DirectSurfaceLiquidPhase::Restart
    );
}

#[test]
fn restart_store_capacity_is_domain_and_identity_precedes_capacity() {
    let configuration = configuration();
    let mut above_capacity = state(&configuration);
    let open = record_index(&configuration, "open");
    above_capacity.records[open].liquid_kg_m2_tile =
        f64::from_bits(configuration.records[open].capacity_kg_m2_tile.to_bits() + 1);
    let failure = above_capacity
        .validate(&configuration)
        .expect_err("finite state liquid above capacity is out of domain")
        .failure()
        .expect("canonical restart domain failure")
        .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Restart);
    assert_eq!(
        failure.context.ofe_id,
        Some(configuration.records[open].key.ofe_id.clone())
    );
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(raw_state_hash(&above_capacity).as_str())
    );

    let mut mixed = above_capacity;
    mixed.records[open].key.tile_id = tile("wrong-tile");
    assert_eq!(
        mixed
            .validate(&configuration)
            .expect_err("restart identity precedes capacity domain")
            .code(),
        DirectSurfaceLiquidErrorCode::E002
    );
}

#[test]
fn authorization_is_reconstructed_before_and_after_resource_construction() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(903);
    let open = record_index(&configuration, "open");
    let demand = request(&configuration, open, transaction, 1.0);
    let mut arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        std::slice::from_ref(&demand),
    )
    .expect("bounded arbitration");
    arbitration.authorizations[0].amount_kg_m2_stand_ground = 0.0;
    let error = apply_surface_liquid_resource_phase(
        &configuration,
        &arbitration,
        &[WaterAmount {
            key: demand.key.clone(),
            amount_kg_m2_stand_ground: 0.0,
        }],
        &[],
    )
    .expect_err("forged authorization");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E009);

    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        std::slice::from_ref(&demand),
    )
    .expect("bounded arbitration");
    let finalized = WaterAmount {
        key: demand.key,
        amount_kg_m2_stand_ground: arbitration.authorizations[0].amount_kg_m2_stand_ground,
    };
    let mut candidate =
        apply_surface_liquid_resource_phase(&configuration, &arbitration, &[finalized], &[])
            .expect("valid candidate");
    candidate.authorizations[0].amount_kg_m2_stand_ground = 0.0;
    assert_eq!(
        candidate
            .validate(&configuration)
            .expect_err("retained authorization poison")
            .code(),
        DirectSurfaceLiquidErrorCode::E006
    );
}

#[test]
fn public_authorization_preserves_identity_domain_protocol_bound_precedence() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(904);
    let open = record_index(&configuration, "open");

    let mut wrong_identity = request(&configuration, open, transaction, f64::NAN);
    wrong_identity.key.transaction_id = TransactionId(905);
    let failure = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &[wrong_identity],
    )
    .expect_err("identity precedes nonfinite domain")
    .failure()
    .expect("canonical identity failure")
    .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Authorization);
    assert_eq!(failure.context.transaction_id, Some(transaction));
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(raw_state_hash(&beginning).as_str())
    );

    let finite = request(&configuration, open, transaction, 0.1);
    let mut nonfinite_duplicate = finite.clone();
    nonfinite_duplicate.amount_kg_m2_stand_ground = f64::NAN;
    let failure = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &[finite, nonfinite_duplicate],
    )
    .expect_err("nonfinite domain precedes duplicate protocol")
    .failure()
    .expect("canonical domain failure")
    .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Authorization);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(raw_state_hash(&beginning).as_str())
    );

    let duplicate_negative = request(&configuration, open, transaction, -f64::MIN_POSITIVE);
    assert_eq!(
        authorize_surface_liquid_withdrawals(
            &configuration,
            &beginning,
            transaction,
            None,
            &[duplicate_negative.clone(), duplicate_negative],
        )
        .expect_err("duplicate protocol precedes finite negative bound")
        .code(),
        DirectSurfaceLiquidErrorCode::E005
    );

    let negative = request(&configuration, open, transaction, -f64::MIN_POSITIVE);
    let failure = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &[negative],
    )
    .expect_err("finite negative request is a bound failure")
    .failure()
    .expect("canonical bound failure")
    .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E006);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Authorization);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(raw_state_hash(&beginning).as_str())
    );
}

#[test]
fn public_resource_phase_classifies_finalized_use_before_later_structure() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(906);
    let open = record_index(&configuration, "open");
    let demand = request(&configuration, open, transaction, 0.2);
    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        std::slice::from_ref(&demand),
    )
    .expect("valid arbitration");

    let mut wrong_identity = WaterAmount {
        key: demand.key.clone(),
        amount_kg_m2_stand_ground: f64::NAN,
    };
    wrong_identity.key.transaction_id = TransactionId(907);
    let failure =
        apply_surface_liquid_resource_phase(&configuration, &arbitration, &[wrong_identity], &[])
            .expect_err("identity precedes nonfinite finalized use")
            .failure()
            .expect("canonical identity failure")
            .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::ResourceCandidate);

    let nonfinite = WaterAmount {
        key: demand.key.clone(),
        amount_kg_m2_stand_ground: f64::NAN,
    };
    let failure = apply_surface_liquid_resource_phase(
        &configuration,
        &arbitration,
        &[nonfinite.clone(), nonfinite],
        &[],
    )
    .expect_err("nonfinite finalized use precedes duplicate/cardinality")
    .failure()
    .expect("canonical domain failure")
    .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(raw_state_hash(&beginning).as_str())
    );

    let negative = WaterAmount {
        key: demand.key.clone(),
        amount_kg_m2_stand_ground: -f64::MIN_POSITIVE,
    };
    let failure =
        apply_surface_liquid_resource_phase(&configuration, &arbitration, &[negative], &[])
            .expect_err("finite negative finalized use is a bound failure")
            .failure()
            .expect("canonical bound failure")
            .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E006);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(raw_state_hash(&beginning).as_str())
    );

    let duplicate_negative = WaterAmount {
        key: demand.key,
        amount_kg_m2_stand_ground: -f64::MIN_POSITIVE,
    };
    assert_eq!(
        apply_surface_liquid_resource_phase(
            &configuration,
            &arbitration,
            &[duplicate_negative.clone(), duplicate_negative],
            &[],
        )
        .expect_err("duplicate finalized protocol precedes finite negative bound")
        .code(),
        DirectSurfaceLiquidErrorCode::E005
    );
}

#[test]
fn retained_protocol_preflight_obeys_identity_domain_cardinality_bound_order() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(911);
    let open = record_index(&configuration, "open");
    let demand = request(&configuration, open, transaction, 0.2);
    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        std::slice::from_ref(&demand),
    )
    .expect("valid arbitration");

    let mut identity_and_nonfinite = arbitration.clone();
    identity_and_nonfinite.requests[0].key.transaction_id = TransactionId(912);
    identity_and_nonfinite.requests[0].amount_kg_m2_stand_ground = f64::NAN;
    assert_eq!(
        apply_surface_liquid_resource_phase(&configuration, &identity_and_nonfinite, &[], &[],)
            .expect_err("retained identity precedes retained nonfinite domain")
            .code(),
        DirectSurfaceLiquidErrorCode::E002
    );

    let mut nonfinite_and_duplicate = arbitration.clone();
    nonfinite_and_duplicate.requests[0].amount_kg_m2_stand_ground = f64::NAN;
    nonfinite_and_duplicate
        .requests
        .push(nonfinite_and_duplicate.requests[0].clone());
    nonfinite_and_duplicate
        .authorizations
        .push(nonfinite_and_duplicate.authorizations[0].clone());
    nonfinite_and_duplicate
        .request_store_keys
        .push(nonfinite_and_duplicate.request_store_keys[0].clone());
    assert_eq!(
        apply_surface_liquid_resource_phase(&configuration, &nonfinite_and_duplicate, &[], &[],)
            .expect_err("retained nonfinite domain precedes duplicate protocol")
            .code(),
        DirectSurfaceLiquidErrorCode::E003
    );

    let mut duplicate_and_negative = arbitration.clone();
    duplicate_and_negative.requests[0].amount_kg_m2_stand_ground = -f64::MIN_POSITIVE;
    duplicate_and_negative.authorizations[0].amount_kg_m2_stand_ground = 0.0;
    duplicate_and_negative
        .requests
        .push(duplicate_and_negative.requests[0].clone());
    duplicate_and_negative
        .authorizations
        .push(duplicate_and_negative.authorizations[0].clone());
    duplicate_and_negative
        .request_store_keys
        .push(duplicate_and_negative.request_store_keys[0].clone());
    assert_eq!(
        apply_surface_liquid_resource_phase(&configuration, &duplicate_and_negative, &[], &[],)
            .expect_err("retained duplicate protocol precedes finite negative bound")
            .code(),
        DirectSurfaceLiquidErrorCode::E005
    );

    let mut negative = arbitration;
    negative.requests[0].amount_kg_m2_stand_ground = -f64::MIN_POSITIVE;
    negative.authorizations[0].amount_kg_m2_stand_ground = 0.0;
    let finalized = WaterAmount {
        key: negative.requests[0].key.clone(),
        amount_kg_m2_stand_ground: 0.0,
    };
    assert_eq!(
        apply_surface_liquid_resource_phase(&configuration, &negative, &[finalized], &[])
            .expect_err("finite negative retained request is a bound failure")
            .code(),
        DirectSurfaceLiquidErrorCode::E006
    );
}

#[test]
fn public_resource_phase_classifies_condensation_domain_and_bound_exactly() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(908);
    let arbitration =
        authorize_surface_liquid_withdrawals(&configuration, &beginning, transaction, None, &[])
            .expect("empty arbitration");
    let record = &configuration.records[record_index(&configuration, "open")];
    let valid = CondensationCredit {
        transaction_id: transaction,
        hydrology_owner_id: configuration.owner_id.clone(),
        ofe_id: record.key.ofe_id.clone(),
        tile_id: record.key.tile_id.clone(),
        surface_id: record.key.surface_id.clone(),
        amount_kg_m2_stand_ground: 0.1,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        temperature_k: 280.0,
        specific_liquid_enthalpy_j_kg: openwepp_land_surface_energy::liquid_enthalpy_j_kg(280.0),
    };

    let mut wrong_identity = valid.clone();
    wrong_identity.transaction_id = TransactionId(909);
    wrong_identity.amount_kg_m2_stand_ground = f64::NAN;
    assert_eq!(
        apply_surface_liquid_resource_phase(&configuration, &arbitration, &[], &[wrong_identity])
            .expect_err("identity precedes nonfinite condensation")
            .code(),
        DirectSurfaceLiquidErrorCode::E002
    );

    let mut nonfinite = valid.clone();
    nonfinite.amount_kg_m2_stand_ground = f64::NAN;
    assert_eq!(
        apply_surface_liquid_resource_phase(
            &configuration,
            &arbitration,
            &[],
            &[valid.clone(), nonfinite],
        )
        .expect_err("nonfinite condensation precedes duplicate protocol")
        .code(),
        DirectSurfaceLiquidErrorCode::E003
    );

    let mut zero_amount = valid.clone();
    zero_amount.amount_kg_m2_stand_ground = 0.0;
    assert_eq!(
        apply_surface_liquid_resource_phase(&configuration, &arbitration, &[], &[zero_amount])
            .expect_err("finite nonpositive condensation is a bound failure")
            .code(),
        DirectSurfaceLiquidErrorCode::E006
    );

    let mut duplicate_zero = valid.clone();
    duplicate_zero.amount_kg_m2_stand_ground = 0.0;
    assert_eq!(
        apply_surface_liquid_resource_phase(
            &configuration,
            &arbitration,
            &[],
            &[duplicate_zero.clone(), duplicate_zero],
        )
        .expect_err("duplicate condensation protocol precedes finite nonpositive bound")
        .code(),
        DirectSurfaceLiquidErrorCode::E005
    );

    let mut invalid_temperature = valid;
    invalid_temperature.amount_kg_m2_stand_ground = 0.0;
    invalid_temperature.temperature_k = 199.0;
    invalid_temperature.specific_liquid_enthalpy_j_kg =
        openwepp_land_surface_energy::liquid_enthalpy_j_kg(199.0);
    assert_eq!(
        apply_surface_liquid_resource_phase(
            &configuration,
            &arbitration,
            &[],
            &[invalid_temperature],
        )
        .expect_err("finite temperature domain precedes amount bound")
        .code(),
        DirectSurfaceLiquidErrorCode::E003
    );
}

#[test]
fn sealed_candidate_revalidation_preserves_identity_and_numeric_codes() {
    let configuration = configuration();
    let beginning = state(&configuration);
    let transaction = TransactionId(910);
    let open = record_index(&configuration, "open");
    let demand = request(&configuration, open, transaction, 0.1);
    let arbitration = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        std::slice::from_ref(&demand),
    )
    .expect("arbitration");
    let finalized = WaterAmount {
        key: demand.key,
        amount_kg_m2_stand_ground: 0.05,
    };
    let candidate =
        apply_surface_liquid_resource_phase(&configuration, &arbitration, &[finalized], &[])
            .expect("candidate");

    let mut wrong_identity = candidate.clone();
    wrong_identity.working_state.records[open].key.tile_id = tile("wrong-tile");
    let failure = wrong_identity
        .validate(&configuration)
        .expect_err("candidate identity poison")
        .failure()
        .expect("canonical candidate identity failure")
        .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::ResourceCandidate);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(raw_state_hash(&beginning).as_str())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());

    let mut nonfinite = candidate.clone();
    nonfinite.finalized_uses[0].amount_kg_m2_stand_ground = f64::NAN;
    assert_eq!(
        nonfinite
            .validate(&configuration)
            .expect_err("candidate nonfinite finalized-use poison")
            .code(),
        DirectSurfaceLiquidErrorCode::E003
    );

    let mut negative = candidate;
    negative.finalized_uses[0].amount_kg_m2_stand_ground = -f64::MIN_POSITIVE;
    assert_eq!(
        negative
            .validate(&configuration)
            .expect_err("candidate negative finalized-use poison")
            .code(),
        DirectSurfaceLiquidErrorCode::E006
    );
}
