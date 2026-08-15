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
