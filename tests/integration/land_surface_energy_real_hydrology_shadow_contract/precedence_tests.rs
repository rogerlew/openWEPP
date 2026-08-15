//! Mixed-poison checks for canonical public E002-E008 precedence.

use super::*;

fn canonical_failure(
    error: LandSurfaceEnergyShadowError,
) -> openwepp_hillslope_orchestrator::DirectSurfaceLiquidFailure {
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
        panic!("mixed poison must retain canonical surface-liquid failure");
    };
    error
        .failure()
        .expect("canonical mixed-poison failure")
        .clone()
}

fn assert_hashes(failure: &openwepp_hillslope_orchestrator::DirectSurfaceLiquidFailure) {
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

fn raw_unified_snapshot(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
) -> Sha256Digest {
    let state = owner
        .beginning_frame()
        .surface_liquid_shadow
        .as_deref()
        .expect("surface owner");
    let mut digest = Sha256::new();
    for bytes in [
        b"openwepp-unified-hydrology-snapshot-v2".as_slice(),
        configuration.owner_id.as_str().as_bytes(),
        owner.snapshot_bytes(),
        configuration.configuration_sha256.as_bytes(),
        state.state_sha256.as_bytes(),
    ] {
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }
    Sha256Digest::try_new(format!("{:x}", digest.finalize())).expect("raw unified snapshot")
}

fn two_ofe_attachment_fixture() -> (
    DirectRunFrame,
    DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidOwnedState,
) {
    let identity = DirectRunIdentity::new(83, 11, 2, 1).expect("identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("frame");
    let layer_template = production_frame(0.02, false).lanes[0]
        .subsurface_layers
        .clone();
    let ofes = [
        OfeId::try_new("ofe-upper").expect("upper OFE"),
        OfeId::try_new("ofe-lower").expect("lower OFE"),
    ];
    let tiles = [
        TileId::try_new("upper-open").expect("upper tile"),
        TileId::try_new("lower-open").expect("lower tile"),
    ];
    let layer = SoilLayerId::try_new("thermal-1").expect("layer");
    let mut bindings = Vec::new();
    let mut records = Vec::new();
    for index in 0..2 {
        frame.lanes[index].area_m2 = [100.0, 200.0][index];
        frame.lanes[index]
            .subsurface_layers
            .clone_from(&layer_template);
        frame.lanes[index].water.soil_water_m = 0.02;
        bindings.push(DirectSurfaceLiquidOfeBinding {
            ofe_id: ofes[index].clone(),
            production_lane_index: index,
            production_lane_id: frame.lanes[index].lane_id,
            ordered_soil_layer_ids: vec![layer.clone()],
            infiltration_soil_thermal_layer_id: layer.clone(),
        });
        records.push(DirectSurfaceLiquidConfigurationRecord {
            key: DirectSurfaceLiquidStoreKey {
                run_id: 83,
                ofe_id: ofes[index].clone(),
                tile_id: tiles[index].clone(),
                surface_id: SurfaceId::try_new(format!("surface-{index}")).expect("surface"),
                surface_class: SurfaceClass::BareMineralSoil,
                source_type: WaterSourceType::SurfaceLiquid,
                source_id: SourceId::try_new(format!("surface-store-{index}")).expect("source"),
            },
            tile_fraction: 1.0,
            capacity_kg_m2_tile: 3.0,
            ofe_area_m2: frame.lanes[index].area_m2,
            ground_ingress_mode: DirectGroundIngressMode::OpenRawPrecipitation,
            runon_destination_ofe_id: (index == 0).then(|| ofes[1].clone()),
            runon_destination_tile_id: (index == 0).then(|| tiles[1].clone()),
        });
    }
    let configuration = DirectSurfaceLiquidConfiguration::new(
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        83,
        ofes.into_iter().collect(),
        bindings,
        records,
    )
    .expect("two-OFE configuration");
    let initial = configuration
        .records
        .iter()
        .map(|record| (record.key.clone(), 1.0))
        .collect();
    let state = DirectSurfaceLiquidOwnedState::new_initial(&configuration, &initial, 0)
        .expect("two-OFE state");
    (frame, configuration, state)
}

#[test]
fn request_identity_native_domain_cardinality_and_bound_precedence_is_canonical() {
    for poison in 0..3 {
        let (mut frame, configuration) = configured_surface_frame(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            1.0,
        );
        if poison == 1 {
            frame.lanes[0].winter_column.snow.runtime_swe_m = 0.001;
        }
        let original = frame.clone();
        let (owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
        let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
            .expect("mixed-poison snapshot");
        let mut batch = surface_potential_batch(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            configuration.records[0].key.source_id.clone(),
            1.0,
        );
        match poison {
            0 => {
                batch.requests[0].key.transaction_id = TransactionId(42);
                batch.requests[0].amount_kg_m2_stand_ground = f64::NAN;
            }
            1 => batch.requests.push(batch.requests[0].clone()),
            2 => {
                batch.requests.push(batch.requests[0].clone());
                batch.requests[1].amount_kg_m2_stand_ground = -1.0;
            }
            _ => unreachable!("bounded poison table"),
        }
        let failure = canonical_failure(
            execute_unified_real_hydrology_shadow(
                &adapter,
                &configuration,
                &receiver_expectations(1, snapshot),
                &batch,
                &BTreeMap::new(),
                &ingress_input(),
                |_| panic!("mixed request poison reached finalization"),
            )
            .expect_err("mixed request poison"),
        );
        assert_eq!(
            failure.code,
            [
                DirectSurfaceLiquidErrorCode::E002,
                DirectSurfaceLiquidErrorCode::E004,
                DirectSurfaceLiquidErrorCode::E005,
            ][poison]
        );
        assert_hashes(&failure);
        assert_eq!(frame, original, "mixed request poison mutated owner");
    }
}

#[test]
fn ingress_identity_precedes_request_and_winter_e003_without_callback() {
    for poison in 0..2 {
        let (mut frame, configuration) = configured_surface_frame(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            1.0,
        );
        let mut batch = surface_potential_batch(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            configuration.records[0].key.source_id.clone(),
            1.0,
        );
        let mut ingress = ingress_input();
        match poison {
            0 => {
                batch.requests[0].amount_kg_m2_stand_ground = f64::NAN;
                match &mut ingress.tile_ingress[0] {
                    DirectTileGroundIngress::OpenRawPrecipitation { tile_id, .. }
                    | DirectTileGroundIngress::CoveredCanopyRelease { tile_id, .. } => {
                        *tile_id = TileId::try_new("unknown-tile").expect("unknown tile");
                    }
                }
            }
            1 => {
                frame.lanes[0].winter_column.frost.total_fine_layer_count = 0.5;
                ingress.wb14_parameters[0].ofe_id =
                    OfeId::try_new("unknown-ofe").expect("unknown OFE");
            }
            _ => unreachable!("bounded ingress precedence poison"),
        }
        let original = frame.clone();
        let (owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
        let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
            .expect("identity poison snapshot remains representable");
        let callback_called = std::cell::Cell::new(false);
        let failure = canonical_failure(
            execute_unified_real_hydrology_shadow(
                &adapter,
                &configuration,
                &receiver_expectations(1, snapshot),
                &batch,
                &BTreeMap::new(),
                &ingress,
                |_| {
                    callback_called.set(true);
                    panic!("invalid ingress identity reached finalization callback")
                },
            )
            .expect_err("ingress identity poison must fail"),
        );
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert!(!callback_called.get());
        assert_hashes(&failure);
        assert_eq!(frame, original, "ingress identity poison mutated owner");
    }
}

#[test]
fn configured_surface_source_identity_precedes_request_and_winter_e003() {
    for poison in 0..2 {
        let (mut frame, configuration) = configured_surface_frame(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            1.0,
        );
        let mut batch = surface_potential_batch(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            configuration.records[0].key.source_id.clone(),
            1.0,
        );
        match poison {
            0 => {
                batch.requests[0].key.source_id =
                    SourceId::try_new("syntactically-valid-wrong-store").expect("wrong source");
                batch.requests[0].amount_kg_m2_stand_ground = f64::NAN;
            }
            1 => {
                batch.requests[0].key.source_tile_id =
                    Some(TileId::try_new("syntactically-valid-wrong-tile").expect("wrong tile"));
                frame.lanes[0].winter_column.frost.total_fine_layer_count = 0.5;
            }
            _ => unreachable!("bounded configured-source poison"),
        }
        let original = frame.clone();
        let (owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
        let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
            .expect("configured-source snapshot");
        let callback_called = std::cell::Cell::new(false);
        let failure = canonical_failure(
            execute_unified_real_hydrology_shadow(
                &adapter,
                &configuration,
                &receiver_expectations(1, snapshot.clone()),
                &batch,
                &BTreeMap::new(),
                &ingress_input(),
                |_| {
                    callback_called.set(true);
                    panic!("invalid source mapping reached finalization callback")
                },
            )
            .expect_err("configured source identity must precede E003"),
        );
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(snapshot.as_str())
        );
        assert!(!callback_called.get());
        assert_hashes(&failure);
        assert_eq!(frame, original, "configured-source poison mutated owner");
    }
}

#[test]
fn source_mapping_e002_attempt_hash_binds_wb14_inputs() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("source-map snapshot");
    let mut batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    batch.requests[0].key.source_id =
        SourceId::try_new("syntactically-valid-wrong-store").expect("wrong source");
    let attempted = |ingress: DirectSurfaceLiquidIngressInput| {
        let failure = canonical_failure(
            execute_unified_real_hydrology_shadow(
                &adapter,
                &configuration,
                &receiver_expectations(1, snapshot.clone()),
                &batch,
                &BTreeMap::new(),
                &ingress,
                |_| panic!("source-map poison reached callback"),
            )
            .expect_err("source-map poison"),
        );
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        failure
            .rollback
            .attempted_owner_sha256
            .expect("complete attempted hash")
    };
    let baseline = attempted(ingress_input());
    let mut changed = ingress_input();
    changed.wb14_parameters[0].matric_potential_m = 0.2;
    assert_ne!(baseline, attempted(changed));
}

#[test]
fn unified_attempt_hash_distinguishes_ingress_component_and_wb14_operands() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("attempt-hash snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let attempted = |request_batch: &openwepp_hillslope_orchestrator::land_surface_energy_shadow::PotentialWaterRequestBatch,
                     mut ingress: DirectSurfaceLiquidIngressInput| {
        ingress.interval_s = f64::NAN;
        let failure = canonical_failure(
            execute_unified_real_hydrology_shadow(
                &adapter,
                &configuration,
                &receiver_expectations(1, snapshot.clone()),
                request_batch,
                &BTreeMap::new(),
                &ingress,
                |_| panic!("nonfinite cadence reached callback"),
            )
            .expect_err("nonfinite cadence"),
        );
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
        failure
            .rollback
            .attempted_owner_sha256
            .expect("attempted hash")
    };
    let baseline = attempted(&batch, ingress_input());
    let mut mass = ingress_input();
    let DirectTileGroundIngress::OpenRawPrecipitation {
        raw_precipitation, ..
    } = &mut mass.tile_ingress[0]
    else {
        panic!("open ingress fixture")
    };
    raw_precipitation.mass_kg_m2_tile_ground = 0.25;
    let mut wb14 = ingress_input();
    wb14.wb14_parameters[0].effective_conductivity_m_s = 2.0e-6;
    let mut interval_index = ingress_input();
    interval_index.interval_index = 1;
    let mut changed_request = batch.clone();
    changed_request.requests[0].amount_kg_m2_stand_ground = 0.5;
    assert_ne!(baseline, attempted(&batch, mass));
    assert_ne!(baseline, attempted(&batch, wb14));
    assert_ne!(baseline, attempted(&batch, interval_index));
    assert_ne!(baseline, attempted(&changed_request, ingress_input()));
}

#[test]
fn unified_snapshot_mismatch_reports_computed_beginning_hash() {
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
    let expected = digest('f');
    let failure = canonical_failure(
        execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &receiver_expectations(1, expected.clone()),
            &batch,
            &BTreeMap::new(),
            &ingress_input(),
            |_| panic!("snapshot mismatch reached callback"),
        )
        .expect_err("snapshot mismatch"),
    );
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(actual.as_str())
    );
    assert_ne!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(expected.as_str())
    );
}

#[test]
fn public_attachment_rejects_each_nonfinite_production_lane_area_as_e003() {
    for lane_index in 0..2 {
        for nonfinite_area in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let (mut frame, configuration, state) = two_ofe_attachment_fixture();
            frame
                .configure_surface_liquid_shadow(&configuration, state.clone())
                .expect("initial attachment");
            let beginning_surface = frame
                .surface_liquid_shadow
                .as_deref()
                .expect("beginning surface owner")
                .clone();
            frame.lanes[lane_index].area_m2 = nonfinite_area;
            let error = frame
                .configure_surface_liquid_shadow(&configuration, state)
                .expect_err("nonfinite lane area must reject at attachment");
            let failure = canonical_failure(LandSurfaceEnergyShadowError::SurfaceLiquid(error));
            assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
            assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Configuration);
            assert_eq!(
                failure.context.owner_id.as_ref(),
                Some(&configuration.owner_id)
            );
            assert_eq!(
                failure.context.ofe_id.as_ref(),
                Some(&configuration.ofe_topology[lane_index]),
            );
            assert_eq!(failure.context.tile_id, None);
            assert_eq!(failure.context.surface_id, None);
            assert_eq!(failure.context.source_id, None);
            assert_hashes(&failure);
            assert_eq!(
                frame.surface_liquid_shadow.as_deref(),
                Some(&beginning_surface),
                "failed reattachment replaced the accepted owner",
            );
        }
    }
}

#[test]
fn attachment_lane_identity_precedes_nonfinite_area_in_any_position() {
    for domain_lane_index in 0..2 {
        let (mut frame, configuration, state) = two_ofe_attachment_fixture();
        frame
            .configure_surface_liquid_shadow(&configuration, state.clone())
            .expect("initial attachment");
        frame.lanes[domain_lane_index].area_m2 = f64::NAN;
        let identity_lane_index = 1 - domain_lane_index;
        let mut wrong_identity = configuration.clone();
        wrong_identity.ofe_bindings[identity_lane_index].production_lane_id += 100;
        let error = frame
            .configure_surface_liquid_shadow(&wrong_identity, state)
            .expect_err("lane identity must precede another lane's domain");
        let failure = canonical_failure(LandSurfaceEnergyShadowError::SurfaceLiquid(error));
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert_eq!(
            failure.context.ofe_id.as_ref(),
            Some(&configuration.ofe_topology[identity_lane_index]),
        );
        assert_hashes(&failure);
    }
}

#[test]
fn attachment_state_identity_precedes_nonfinite_lane_area_without_mutation() {
    for state_poison in 0..2 {
        let (mut frame, configuration, state) = two_ofe_attachment_fixture();
        frame
            .configure_surface_liquid_shadow(&configuration, state.clone())
            .expect("initial attachment");
        frame.lanes[0].area_m2 = f64::NAN;
        let beginning_surface = frame
            .surface_liquid_shadow
            .as_deref()
            .expect("beginning surface owner")
            .clone();
        let beginning_lane_area_bits = frame
            .lanes
            .iter()
            .map(|lane| lane.area_m2.to_bits())
            .collect::<Vec<_>>();
        let mut attempted_state = state.clone();
        match state_poison {
            0 => {
                attempted_state.records[1].key.tile_id =
                    TileId::try_new("wrong-state-tile").expect("wrong tile");
            }
            1 => attempted_state.records[1].liquid_kg_m2_tile = 1.25,
            _ => unreachable!("bounded state poison table"),
        }

        let error = frame
            .configure_surface_liquid_shadow(&configuration, attempted_state)
            .expect_err("state identity must precede lane numeric domain");
        let failure = canonical_failure(LandSurfaceEnergyShadowError::SurfaceLiquid(error));
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Restart);
        assert_hashes(&failure);
        assert_eq!(
            frame.surface_liquid_shadow.as_deref(),
            Some(&beginning_surface),
            "failed attachment replaced the accepted owner",
        );
        assert_eq!(
            frame
                .lanes
                .iter()
                .map(|lane| lane.area_m2.to_bits())
                .collect::<Vec<_>>(),
            beginning_lane_area_bits,
            "failed attachment mutated production lanes",
        );
    }
}

#[test]
fn request_identity_precedes_nonfinite_beginning_surface_state() {
    for state_index in 0..2 {
        let (mut frame, configuration) = configured_two_tile_surface_frame();
        let snapshot = {
            let (owner, _) = owner(&frame);
            unified_beginning_hydrology_snapshot_sha256(
                &LandSurfaceEnergyRealHydrologyAdapter::new(&owner),
                &configuration,
            )
            .expect("clean mixed-input snapshot")
        };
        frame
            .surface_liquid_shadow
            .as_deref_mut()
            .expect("surface owner")
            .records[state_index]
            .liquid_kg_m2_tile = f64::NAN;
        let beginning_liquid_bits = frame
            .surface_liquid_shadow
            .as_deref()
            .expect("surface owner")
            .records
            .iter()
            .map(|record| record.liquid_kg_m2_tile.to_bits())
            .collect::<Vec<_>>();
        let (owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
        let mut batch = surface_potential_batch(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            open_surface_source_id(&configuration),
            1.0,
        );
        batch.requests[0].key.transaction_id = TransactionId(42);
        let failure = canonical_failure(
            execute_unified_real_hydrology_shadow(
                &adapter,
                &configuration,
                &receiver_expectations(1, snapshot),
                &batch,
                &BTreeMap::new(),
                &ingress_input(),
                |_| panic!("mixed state/request poison reached finalization"),
            )
            .expect_err("request identity must precede nonfinite state"),
        );
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert_eq!(failure.context.transaction_id, Some(TransactionId(42)));
        assert_hashes(&failure);
        assert_eq!(
            frame
                .surface_liquid_shadow
                .as_deref()
                .expect("surface owner")
                .records
                .iter()
                .map(|record| record.liquid_kg_m2_tile.to_bits())
                .collect::<Vec<_>>(),
            beginning_liquid_bits,
            "mixed-input failure mutated owner",
        );
    }
}

#[test]
fn request_identity_precedes_stale_digest_nonfinite_configuration_and_binds_raw_bits() {
    let mut attempted_hashes = Vec::new();
    for raw_bits in [0x7ff8_0000_0000_0601, 0x7ff8_0000_0000_0602] {
        let (frame, mut configuration) = configured_surface_frame(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            1.0,
        );
        let (owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
        let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
            .expect("clean mixed-input snapshot");
        let stale_digest = configuration.configuration_sha256.clone();
        configuration.records[0].capacity_kg_m2_tile = f64::from_bits(raw_bits);
        assert_eq!(configuration.configuration_sha256, stale_digest);
        let mut batch = surface_potential_batch(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            open_surface_source_id(&configuration),
            1.0,
        );
        batch.requests[0].key.transaction_id = TransactionId(42);
        let failure = canonical_failure(
            execute_unified_real_hydrology_shadow(
                &adapter,
                &configuration,
                &receiver_expectations(1, snapshot),
                &batch,
                &BTreeMap::new(),
                &ingress_input(),
                |_| panic!("mixed configuration/request poison reached finalization"),
            )
            .expect_err("request identity must precede nonfinite configuration"),
        );
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert_eq!(failure.context.transaction_id, Some(TransactionId(42)));
        assert_hashes(&failure);
        attempted_hashes.push(
            failure
                .rollback
                .attempted_owner_sha256
                .expect("raw-bound unified attempt"),
        );
    }
    assert_ne!(
        attempted_hashes[0], attempted_hashes[1],
        "request E002 attempt must bind stale-digest NaN payload bits",
    );
}

#[test]
fn every_cross_input_e002_precedes_config_or_state_e003_without_callback() {
    for domain_owner in 0..2 {
        for identity_poison in 0..4 {
            let (mut frame, mut configuration) = configured_surface_frame(
                SurfaceClass::BareMineralSoil,
                WaterSourceType::SurfaceLiquid,
                1.0,
            );
            let clean_snapshot = {
                let (clean_owner, _) = owner(&frame);
                unified_beginning_hydrology_snapshot_sha256(
                    &LandSurfaceEnergyRealHydrologyAdapter::new(&clean_owner),
                    &configuration,
                )
                .expect("clean snapshot")
            };
            if domain_owner == 0 {
                configuration.records[0].capacity_kg_m2_tile = f64::NAN;
            } else {
                frame
                    .surface_liquid_shadow
                    .as_deref_mut()
                    .expect("surface state")
                    .records[0]
                    .liquid_kg_m2_tile = f64::NAN;
            }
            let (real_owner, _) = owner(&frame);
            let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
            let mut batch = surface_potential_batch(
                SurfaceClass::BareMineralSoil,
                WaterSourceType::SurfaceLiquid,
                configuration.records[0].key.source_id.clone(),
                1.0,
            );
            let mut ingress = ingress_input();
            let mut expected_snapshot = clean_snapshot;
            match identity_poison {
                0 => match &mut ingress.tile_ingress[0] {
                    DirectTileGroundIngress::OpenRawPrecipitation { tile_id, .. }
                    | DirectTileGroundIngress::CoveredCanopyRelease { tile_id, .. } => {
                        *tile_id = TileId::try_new("unknown-tile").expect("tile");
                    }
                },
                1 => {
                    batch.requests[0].key.source_id =
                        SourceId::try_new("wrong-configured-source").expect("source");
                }
                2 => {
                    batch.transaction_id = TransactionId(42);
                    for request in &mut batch.requests {
                        request.key.transaction_id = TransactionId(42);
                    }
                    ingress.transaction_id = TransactionId(42);
                }
                3 => expected_snapshot = digest('9'),
                _ => unreachable!("bounded identity poison"),
            }
            let callback_count = std::cell::Cell::new(0);
            let failure = canonical_failure(
                execute_unified_real_hydrology_shadow(
                    &adapter,
                    &configuration,
                    &receiver_expectations(1, expected_snapshot),
                    &batch,
                    &BTreeMap::new(),
                    &ingress,
                    |_| {
                        callback_count.set(callback_count.get() + 1);
                        panic!("identity/domain cross-poison reached callback")
                    },
                )
                .expect_err("identity must precede config/state domain"),
            );
            assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
            assert_hashes(&failure);
            assert_eq!(callback_count.get(), 0);
        }
    }
}

#[test]
fn stale_config_or_state_digest_with_nonfinite_bits_is_identity_failure_without_callback() {
    for poisoned_owner in 0..2 {
        let (mut frame, mut configuration) = configured_surface_frame(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            1.0,
        );
        let clean_snapshot = {
            let (clean_owner, _) = owner(&frame);
            unified_beginning_hydrology_snapshot_sha256(
                &LandSurfaceEnergyRealHydrologyAdapter::new(&clean_owner),
                &configuration,
            )
            .expect("clean snapshot")
        };
        if poisoned_owner == 0 {
            configuration.records[0].capacity_kg_m2_tile = f64::NAN;
        } else {
            frame
                .surface_liquid_shadow
                .as_deref_mut()
                .expect("surface state")
                .records[0]
                .liquid_kg_m2_tile = f64::NAN;
        }
        let before = owner(&frame).0.snapshot_bytes().to_vec();
        let (real_owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
        let batch = surface_potential_batch(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            configuration.records[0].key.source_id.clone(),
            1.0,
        );
        let callback_count = std::cell::Cell::new(0);
        let failure = canonical_failure(
            execute_unified_real_hydrology_shadow(
                &adapter,
                &configuration,
                &receiver_expectations(1, clean_snapshot),
                &batch,
                &BTreeMap::new(),
                &ingress_input(),
                |_| {
                    callback_count.set(callback_count.get() + 1);
                    panic!("stale digest/nonfinite poison reached callback")
                },
            )
            .expect_err("stale self-digest must fail as identity"),
        );
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert_hashes(&failure);
        assert_eq!(callback_count.get(), 0);
        assert_eq!(
            owner(&frame).0.snapshot_bytes(),
            before,
            "identity failure mutated owner bytes"
        );
    }
}

#[test]
fn receiver_rejects_every_invalid_lane_area_as_e003_without_mutation_or_callback() {
    for area in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -1.0] {
        let (mut frame, configuration) = configured_surface_frame(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            1.0,
        );
        frame.lanes[0].area_m2 = area;
        let lane_bits = frame.lanes[0].area_m2.to_bits();
        let beginning_surface = frame
            .surface_liquid_shadow
            .as_deref()
            .expect("surface owner")
            .clone();
        let (real_owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
        let snapshot_failure = canonical_failure(
            unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
                .expect_err("invalid receiver lane area"),
        );
        assert_eq!(snapshot_failure.code, DirectSurfaceLiquidErrorCode::E003);
        assert_hashes(&snapshot_failure);

        let snapshot = raw_unified_snapshot(&real_owner, &configuration);
        let batch = surface_potential_batch(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            configuration.records[0].key.source_id.clone(),
            1.0,
        );
        let callback_count = std::cell::Cell::new(0);
        let failure = canonical_failure(
            execute_unified_real_hydrology_shadow(
                &adapter,
                &configuration,
                &receiver_expectations(1, snapshot),
                &batch,
                &BTreeMap::new(),
                &ingress_input(),
                |_| {
                    callback_count.set(callback_count.get() + 1);
                    panic!("invalid lane domain reached callback")
                },
            )
            .expect_err("invalid receiver lane area"),
        );
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
        assert_hashes(&failure);
        assert_eq!(callback_count.get(), 0);
        assert_eq!(
            real_owner.beginning_frame().lanes[0].area_m2.to_bits(),
            lane_bits
        );
        assert_eq!(
            real_owner
                .beginning_frame()
                .surface_liquid_shadow
                .as_deref(),
            Some(&beginning_surface),
        );
    }
}

#[test]
fn receiver_state_identity_precedes_invalid_lane_domain_without_mutation_or_callback() {
    for state_poison in 0..2 {
        let (mut frame, configuration) = configured_surface_frame(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            1.0,
        );
        frame.lanes[0].area_m2 = f64::NAN;
        let state = frame
            .surface_liquid_shadow
            .as_deref_mut()
            .expect("surface owner");
        match state_poison {
            0 => {
                state.records[0].key.tile_id =
                    TileId::try_new("wrong-state-tile").expect("wrong tile");
            }
            1 => state.records[0].liquid_kg_m2_tile = 1.25,
            _ => unreachable!("bounded state poison table"),
        }
        let lane_bits = frame.lanes[0].area_m2.to_bits();
        let beginning_surface = frame
            .surface_liquid_shadow
            .as_deref()
            .expect("surface owner")
            .clone();
        let (real_owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
        let snapshot = raw_unified_snapshot(&real_owner, &configuration);
        let batch = surface_potential_batch(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            configuration.records[0].key.source_id.clone(),
            1.0,
        );
        let callback_count = std::cell::Cell::new(0);
        let failure = canonical_failure(
            execute_unified_real_hydrology_shadow(
                &adapter,
                &configuration,
                &receiver_expectations(1, snapshot),
                &batch,
                &BTreeMap::new(),
                &ingress_input(),
                |_| {
                    callback_count.set(callback_count.get() + 1);
                    panic!("state identity/lane-domain poison reached callback")
                },
            )
            .expect_err("state identity must precede lane domain"),
        );
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert_hashes(&failure);
        assert_eq!(callback_count.get(), 0);
        assert_eq!(
            real_owner.beginning_frame().lanes[0].area_m2.to_bits(),
            lane_bits
        );
        assert_eq!(
            real_owner
                .beginning_frame()
                .surface_liquid_shadow
                .as_deref(),
            Some(&beginning_surface),
        );
    }
}

#[test]
fn exact_one_custody_precedes_finite_cadence_failure() {
    let (mut frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    frame.lanes[0]
        .day_inputs
        .push(DirectDayConstructorInputs::zero());
    frame.lanes[0].day_inputs[0]
        .infiltration_depression_inputs
        .depression_storage_delta_handoff_m = 0.001;
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("custody snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let mut ingress = ingress_input();
    ingress.interval_s += 1.0;
    let failure = canonical_failure(
        execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &receiver_expectations(1, snapshot),
            &batch,
            &BTreeMap::new(),
            &ingress,
            |_| panic!("duplicate custody reached finalization"),
        )
        .expect_err("duplicate custody and cadence"),
    );
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E007);
    assert_hashes(&failure);
}

#[test]
fn final_protocol_identity_and_cardinality_precede_nonfinite_and_negative_amounts() {
    for poison in 0..2 {
        let (frame, configuration) = configured_surface_frame(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            1.0,
        );
        let (owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
        let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
            .expect("protocol snapshot");
        let batch = surface_potential_batch(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            configuration.records[0].key.source_id.clone(),
            1.0,
        );
        let result = execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &receiver_expectations(1, snapshot.clone()),
            &batch,
            &BTreeMap::new(),
            &ingress_input(),
            |authorizations| {
                let baseline = unified_finalization(accepted_surface_protocol(
                    &batch,
                    authorizations,
                    &snapshot,
                ));
                let mut protocol = baseline.water_protocol().clone();
                match poison {
                    0 => {
                        protocol.requests[0].amount_kg_m2_stand_ground = f64::NAN;
                        protocol.authorizations[0].key.transaction_id = TransactionId(42);
                    }
                    1 => {
                        protocol.requests.push(protocol.requests[0].clone());
                        protocol.requests[1].amount_kg_m2_stand_ground = -1.0;
                    }
                    _ => unreachable!("bounded poison table"),
                }
                let standalone = UnifiedLseFinalization::try_new(
                    &finalization_expectations(
                        baseline.water_protocol(),
                        baseline.soil_thermal_candidates(),
                    ),
                    protocol,
                    baseline.ending_tile_states_pre_ingress().to_vec(),
                    baseline.soil_thermal_candidates().to_vec(),
                    baseline.rollback_hashes().to_vec(),
                );
                let standalone_failure = canonical_failure(
                    standalone
                        .as_ref()
                        .expect_err("standalone mixed protocol poison")
                        .clone(),
                );
                assert_eq!(
                    standalone_failure.code,
                    [
                        DirectSurfaceLiquidErrorCode::E002,
                        DirectSurfaceLiquidErrorCode::E005,
                    ][poison]
                );
                assert_hashes(&standalone_failure);
                standalone
            },
        );
        let failure = canonical_failure(result.expect_err("mixed protocol poison"));
        assert_eq!(
            failure.code,
            [
                DirectSurfaceLiquidErrorCode::E002,
                DirectSurfaceLiquidErrorCode::E005,
            ][poison]
        );
        assert_hashes(&failure);
    }
}

#[test]
fn finalization_applies_precedence_across_protocol_and_all_receiver_sets() {
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        SourceId::try_new("surface-store:ofe-1:open").expect("source"),
        1.0,
    );
    let authorization = WaterAuthorization {
        key: batch.requests[0].key.clone(),
        amount_kg_m2_stand_ground: batch.requests[0].amount_kg_m2_stand_ground,
        reason: WaterAuthorizationReason::FullSupply,
    };
    let baseline = unified_finalization(accepted_surface_protocol(
        &batch,
        &[authorization],
        &digest('3'),
    ));

    for receiver_set in 0..2 {
        for protocol_poison in 0..3 {
            let mut protocol = baseline.water_protocol().clone();
            match protocol_poison {
                0 => protocol.requests.push(protocol.requests[0].clone()),
                1 => protocol.requests[0].amount_kg_m2_stand_ground = -1.0,
                2 => protocol.requests[0].key.transaction_id = TransactionId(42),
                _ => unreachable!("bounded protocol poison table"),
            }
            let mut lse = baseline.ending_tile_states_pre_ingress().to_vec();
            let mut thermal = baseline.soil_thermal_candidates().to_vec();
            match receiver_set {
                0 => lse[0].surface_enthalpy_j_m2_tile_ground = f64::NAN,
                1 => {
                    thermal[0].layers[0].infiltration_enthalpy_credit_j_m2_ofe_ground = f64::NAN;
                }
                _ => unreachable!("bounded receiver-set table"),
            }
            let failure = canonical_failure(
                UnifiedLseFinalization::try_new(
                    &finalization_expectations(
                        baseline.water_protocol(),
                        baseline.soil_thermal_candidates(),
                    ),
                    protocol,
                    lse,
                    thermal,
                    baseline.rollback_hashes().to_vec(),
                )
                .expect_err("mixed protocol/receiver poison"),
            );
            assert_eq!(
                failure.code,
                if protocol_poison == 2 {
                    DirectSurfaceLiquidErrorCode::E002
                } else {
                    DirectSurfaceLiquidErrorCode::E003
                },
                "protocol poison {protocol_poison}, receiver set {receiver_set}",
            );
            assert_hashes(&failure);
        }
    }
}
