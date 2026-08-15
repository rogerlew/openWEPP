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
