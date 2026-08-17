use super::*;

#[test]
#[allow(clippy::too_many_lines)]
fn multi_ofe_snapshot_binding_failure_reports_the_later_offender() {
    let identity = DirectRunIdentity::new(83, 11, 2, 1).expect("identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("frame");
    let layer_template = production_frame(0.02, false).lanes[0]
        .subsurface_layers
        .clone();
    for (index, lane) in frame.lanes.iter_mut().enumerate() {
        lane.area_m2 = if index == 0 { 100.0 } else { 200.0 };
        lane.subsurface_layers = layer_template.clone();
        lane.water.soil_water_m = 0.02;
    }
    let upper_ofe = OfeId::try_new("ofe-upper").expect("upper OFE");
    let lower_ofe = OfeId::try_new("ofe-lower").expect("lower OFE");
    let upper_tile = TileId::try_new("upper-open").expect("upper tile");
    let lower_tile = TileId::try_new("lower-open").expect("lower tile");
    let layer = SoilLayerId::try_new("thermal-1").expect("layer");
    let bindings = vec![
        DirectSurfaceLiquidOfeBinding {
            ofe_id: upper_ofe.clone(),
            production_lane_index: 0,
            production_lane_id: frame.lanes[0].lane_id,
            ordered_soil_layer_ids: vec![layer.clone()],
            infiltration_soil_thermal_layer_id: layer.clone(),
        },
        DirectSurfaceLiquidOfeBinding {
            ofe_id: lower_ofe.clone(),
            production_lane_index: 1,
            production_lane_id: frame.lanes[1].lane_id,
            ordered_soil_layer_ids: vec![layer.clone()],
            infiltration_soil_thermal_layer_id: layer.clone(),
        },
    ];
    let make_record =
        |ofe_id: OfeId, tile_id: TileId, area: f64, destination: Option<(OfeId, TileId)>| {
            DirectSurfaceLiquidConfigurationRecord {
                key: DirectSurfaceLiquidStoreKey {
                    run_id: 83,
                    surface_id: SurfaceId::try_new(format!(
                        "surface:{}:{}",
                        ofe_id.as_str(),
                        tile_id.as_str()
                    ))
                    .expect("surface"),
                    source_id: SourceId::try_new(format!(
                        "surface-store:{}:{}",
                        ofe_id.as_str(),
                        tile_id.as_str()
                    ))
                    .expect("source"),
                    ofe_id,
                    tile_id,
                    surface_class: SurfaceClass::BareMineralSoil,
                    source_type: WaterSourceType::SurfaceLiquid,
                },
                tile_fraction: 1.0,
                capacity_kg_m2_tile: 3.0,
                ofe_area_m2: area,
                ground_ingress_mode: DirectGroundIngressMode::OpenRawPrecipitation,
                runon_destination_ofe_id: destination.as_ref().map(|row| row.0.clone()),
                runon_destination_tile_id: destination.map(|row| row.1),
            }
        };
    let configuration = DirectSurfaceLiquidConfiguration::new(
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        83,
        vec![upper_ofe.clone(), lower_ofe.clone()],
        bindings.clone(),
        vec![
            make_record(
                upper_ofe,
                upper_tile,
                100.0,
                Some((lower_ofe.clone(), lower_tile.clone())),
            ),
            make_record(lower_ofe.clone(), lower_tile, 200.0, None),
        ],
    )
    .expect("two-OFE configuration");
    let initial = configuration
        .records
        .iter()
        .map(|record| (record.key.clone(), 1.0))
        .collect::<BTreeMap<_, _>>();
    let state = DirectSurfaceLiquidOwnedState::new_initial(&configuration, &initial, 0)
        .expect("initial state");

    let mut one_lane_frame = production_frame(0.02, false);
    let excess_error = one_lane_frame
        .configure_surface_liquid_shadow(&configuration, state.clone())
        .expect_err("second configured OFE exceeds the production frame");
    let excess_failure = excess_error.failure().expect("canonical excess failure");
    let excess_key = &configuration.records[1].key;
    assert_eq!(excess_failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(
        excess_failure.context.ofe_id.as_ref(),
        Some(&excess_key.ofe_id)
    );
    assert_eq!(
        excess_failure.context.tile_id.as_ref(),
        Some(&excess_key.tile_id)
    );
    assert_eq!(
        excess_failure.context.surface_id.as_ref(),
        Some(&excess_key.surface_id)
    );
    assert_eq!(
        excess_failure.context.source_id.as_ref(),
        Some(&excess_key.source_id)
    );

    let short_configuration = surface_configuration(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
    );
    let short_state = DirectSurfaceLiquidOwnedState::new_initial(
        &short_configuration,
        &BTreeMap::from([(short_configuration.records[0].key.clone(), 1.0)]),
        0,
    )
    .expect("short state");
    let short_error = frame
        .configure_surface_liquid_shadow(&short_configuration, short_state)
        .expect_err("missing second configured OFE");
    let short_failure = short_error.failure().expect("canonical short failure");
    assert_eq!(short_failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(
        short_failure.context.owner_id.as_ref(),
        Some(&short_configuration.owner_id)
    );
    assert_eq!(short_failure.context.ofe_id, None);
    assert_eq!(short_failure.context.tile_id, None);
    assert_eq!(short_failure.context.surface_id, None);
    assert_eq!(short_failure.context.source_id, None);

    frame
        .configure_surface_liquid_shadow(&configuration, state)
        .expect("attach exact owner");
    let owner = RealHydrologyShadowAdapter::try_from_day_start(
        &frame,
        0,
        TransactionId(41),
        1_800.0,
        configuration.owner_id.clone(),
        &[
            RealHydrologyLaneLayerMap {
                ofe_lane: RealHydrologyOfeLaneId {
                    lane_index: 0,
                    lane_id: frame.lanes[0].lane_id,
                },
                layer_ids: vec![layer.clone()],
            },
            RealHydrologyLaneLayerMap {
                ofe_lane: RealHydrologyOfeLaneId {
                    lane_index: 1,
                    lane_id: frame.lanes[1].lane_id,
                },
                layer_ids: vec![layer.clone()],
            },
        ],
    )
    .expect("two-OFE adapter");
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);

    // Both OFEs deliberately use the same textual layer ID. A lower-OFE request
    // must still reject a source mapped to the upper production lane.
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let mut batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[1].key.source_id.clone(),
        1.0,
    );
    let lower_record = &configuration.records[1];
    let request_key = &mut batch.requests[0].key;
    request_key.ofe_id = lower_record.key.ofe_id.clone();
    request_key.requesting_tile_id = lower_record.key.tile_id.clone();
    request_key.surface_id = Some(lower_record.key.surface_id.clone());
    request_key.source_type = WaterSourceType::SoilLayerLiquid;
    request_key.source_id = SourceId::try_new("soil:ofe-lower:thermal-1").expect("source");
    request_key.source_tile_id = None;
    request_key.soil_layer_id = Some(layer.clone());
    let request_key = batch.requests[0].key.clone();
    let crossed_source = RealHydrologySourceKey {
        ofe_lane: RealHydrologyOfeLaneId {
            lane_index: 0,
            lane_id: frame.lanes[0].lane_id,
        },
        layer_id: layer.clone(),
    };
    let zero = DirectIngressAmount {
        mass_kg_m2_tile_ground: 0.0,
        temperature_k: 294.0,
        specific_liquid_enthalpy_j_kg: 4_218.0 * (294.0 - 273.15),
        start_s: 0.0,
        end_s: 1_800.0,
    };
    let ingress = DirectSurfaceLiquidIngressInput {
        transaction_id: TransactionId(41),
        day_index: 0,
        interval_index: 0,
        interval_s: 1_800.0,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| DirectTileGroundIngress::OpenRawPrecipitation {
                ofe_id: record.key.ofe_id.clone(),
                tile_id: record.key.tile_id.clone(),
                surface_id: record.key.surface_id.clone(),
                raw_precipitation: zero.clone(),
            })
            .collect(),
        wb14_parameters: configuration
            .ofe_bindings
            .iter()
            .map(|binding| DirectOfeWb14Parameters {
                ofe_id: binding.ofe_id.clone(),
                effective_conductivity_m_s: 1.0e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 0.04,
            })
            .collect(),
    };
    let expectations = UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        digest('2'),
        configuration.owner_id.clone(),
        snapshot.clone(),
        ResourceOwnerId::try_new("soil-thermal").expect("thermal owner"),
        digest('4'),
        configuration
            .records
            .iter()
            .map(|record| {
                (
                    record.key.ofe_id.clone(),
                    record.key.tile_id.clone(),
                    vec![layer.clone()],
                )
            })
            .collect(),
    )
    .expect("expectations");
    let callback_called = std::cell::Cell::new(false);
    let error = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &expectations,
        &batch,
        &BTreeMap::from([(request_key, crossed_source)]),
        &ingress,
        |_| {
            callback_called.set(true);
            panic!("cross-OFE source reached callback")
        },
    )
    .expect_err("same layer ID on the wrong OFE lane must reject");
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
        panic!("cross-OFE source failure must remain canonical");
    };
    let failure = error.failure().expect("canonical failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert!(!callback_called.get());

    let mut wrong_area_records = configuration.records.clone();
    wrong_area_records[1].ofe_area_m2 = 201.0;
    let wrong_area = DirectSurfaceLiquidConfiguration::new(
        configuration.owner_id.clone(),
        configuration.run_id,
        configuration.ofe_topology.clone(),
        bindings.clone(),
        wrong_area_records,
    )
    .expect("structurally valid wrong second area");
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &wrong_area)
            .expect_err("wrong second area")
    else {
        panic!("wrong area must retain canonical failure");
    };
    let failure = error.failure().expect("area failure");
    let lower = &wrong_area.records[1].key;
    assert_eq!(failure.context.ofe_id.as_ref(), Some(&lower.ofe_id));
    assert_eq!(failure.context.tile_id.as_ref(), Some(&lower.tile_id));
    assert_eq!(failure.context.surface_id.as_ref(), Some(&lower.surface_id));
    assert_eq!(failure.context.source_id.as_ref(), Some(&lower.source_id));

    let mut wrong_layer_bindings = bindings;
    let wrong_layer = SoilLayerId::try_new("thermal-wrong").expect("wrong layer");
    wrong_layer_bindings[1].ordered_soil_layer_ids = vec![wrong_layer.clone()];
    wrong_layer_bindings[1].infiltration_soil_thermal_layer_id = wrong_layer;
    let wrong_layer = DirectSurfaceLiquidConfiguration::new(
        configuration.owner_id.clone(),
        configuration.run_id,
        configuration.ofe_topology.clone(),
        wrong_layer_bindings,
        configuration.records.clone(),
    )
    .expect("structurally valid wrong second layer");
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &wrong_layer)
            .expect_err("wrong second layer")
    else {
        panic!("wrong layer must retain canonical failure");
    };
    let failure = error.failure().expect("layer failure");
    assert_eq!(failure.context.ofe_id.as_ref(), Some(&lower_ofe));
    assert_eq!(failure.context.tile_id, None);
    assert_eq!(failure.context.surface_id, None);
    assert_eq!(failure.context.source_id, None);
}
