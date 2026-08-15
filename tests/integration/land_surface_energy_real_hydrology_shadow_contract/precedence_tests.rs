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
        frame.lanes[index].subsurface_layers = layer_template.clone();
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
                        protocol.requests[0].key.transaction_id = TransactionId(42);
                        protocol.requests[0].amount_kg_m2_stand_ground = f64::NAN;
                    }
                    1 => {
                        protocol.requests.push(protocol.requests[0].clone());
                        protocol.requests[1].amount_kg_m2_stand_ground = -1.0;
                    }
                    _ => unreachable!("bounded poison table"),
                }
                UnifiedLseFinalization::try_new(
                    protocol,
                    baseline.ending_tile_states_pre_ingress().to_vec(),
                    baseline.soil_thermal_candidates().to_vec(),
                    baseline.rollback_hashes().to_vec(),
                )
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
