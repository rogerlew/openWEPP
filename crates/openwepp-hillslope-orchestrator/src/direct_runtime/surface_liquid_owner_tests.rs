use super::*;

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
        Some(beginning.state_sha256.as_str())
    );
    assert_eq!(failure.rollback.attempted_owner_sha256, None);
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
            Some(beginning.state_sha256.as_str())
        );
        assert_eq!(failure.rollback.attempted_owner_sha256, None);
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

    let error = authorize_surface_liquid_withdrawals(
        &configuration,
        &beginning,
        transaction,
        None,
        &requests,
    )
    .expect_err("finite requests with an infinite sum must fail closed");
    let failure = error.failure().expect("canonical overflow failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Authorization);
    assert_eq!(failure.context.transaction_id, Some(transaction));
    assert_eq!(failure.context.owner_id, Some(owner("overflow-second")));
    assert_eq!(failure.context.ofe_id, Some(second.key.ofe_id.clone()));
    assert_eq!(failure.context.tile_id, second.key.source_tile_id.clone());
    assert_eq!(failure.context.surface_id, second.key.surface_id.clone());
    assert_eq!(
        failure.context.source_id,
        Some(second.key.source_id.clone())
    );
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(beginning.state_sha256.as_str())
    );

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
    let open = record_index(&configuration, "open");
    let record = &configuration.records[open];
    let exact = openwepp_land_surface_energy::liquid_enthalpy_j_kg(280.0);
    let condensation = CondensationCredit {
        transaction_id: transaction,
        hydrology_owner_id: configuration.owner_id.clone(),
        ofe_id: record.key.ofe_id.clone(),
        tile_id: record.key.tile_id.clone(),
        surface_id: record.key.surface_id.clone(),
        amount_kg_m2_stand_ground: 0.1,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        temperature_k: 280.0,
        specific_liquid_enthalpy_j_kg: f64::from_bits(exact.to_bits() + 1),
    };
    let error =
        apply_surface_liquid_resource_phase(&configuration, &arbitration, &[], &[condensation])
            .expect_err("enthalpy poison");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E009);
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
    assert_eq!(failure.rollback.attempted_owner_sha256, None);

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
    assert_eq!(
        failure.rollback.beginning_owner_sha256,
        Some(accepted.state_sha256)
    );
    assert_eq!(failure.rollback.attempted_owner_sha256, None);
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
        Some(accepted.state_sha256.as_str())
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
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E009);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::ResourceCandidate);
    assert_eq!(failure.context.transaction_id, Some(TransactionId(901)));
    assert_eq!(
        failure.rollback.beginning_owner_sha256,
        Some(beginning.state_sha256.clone())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());

    let mut stale = candidate;
    stale.beginning_state.configuration_sha256 = "1".repeat(64);
    assert_eq!(
        stale
            .validate(&configuration)
            .expect_err("stale candidate")
            .code(),
        DirectSurfaceLiquidErrorCode::E009
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
        DirectSurfaceLiquidErrorCode::E009
    );
}
