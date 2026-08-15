use super::*;

fn set_snow_scalar(frame: &mut DirectRunFrame, field: usize, value: f64) {
    let snow = &mut frame.lanes[0].winter_column.snow;
    match field {
        0 => snow.runtime_swe_m = value,
        1 => snow.runtime_depth_m = value,
        2 => snow.runtime_density_kg_m3 = value,
        3 => snow.runtime_settle_day_count = value,
        4 => snow.coe_boundary_depth_m = value,
        5 => snow.coe_boundary_density_kg_m3 = value,
        6 => snow.coe_boundary_settle_day_count = value,
        7 => snow.liquid_water_retained_m = value,
        _ => unreachable!("complete DirectSnowLaneState scalar table"),
    }
}

fn set_snow_carry_scalar(frame: &mut DirectRunFrame, field: usize, value: f64) {
    let mut carry = openwepp_hillslope_orchestrator::DirectSnowRuntimeCarry::from(
        openwepp_hillslope_orchestrator::DirectSnowLaneState::zero(),
    );
    match field {
        0 => carry.runtime_swe_m = value,
        1 => carry.runtime_depth_m = value,
        2 => carry.runtime_density_kg_m3 = value,
        3 => carry.runtime_settle_day_count = value,
        4 => carry.coe_boundary_depth_m = value,
        5 => carry.coe_boundary_density_kg_m3 = value,
        6 => carry.coe_boundary_settle_day_count = value,
        7 => carry.liquid_water_retained_m = value,
        _ => unreachable!("complete DirectSnowRuntimeCarry scalar table"),
    }
    frame.lanes[0].snow_runtime_carry = Some(Box::new(carry));
}

fn execute_snow_poison(
    field: usize,
    value: f64,
    runtime_carry: bool,
) -> (LandSurfaceEnergyShadowError, Sha256Digest, bool) {
    let (mut frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    if runtime_carry {
        set_snow_carry_scalar(&mut frame, field, value);
    } else {
        set_snow_scalar(&mut frame, field, value);
    }
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("snow scalar snapshot remains representable");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let mut callback_called = false;
    let error = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |_| {
            callback_called = true;
            Err(LandSurfaceEnergyShadowError::Identity(
                "snow scalar poison reached callback",
            ))
        },
    )
    .expect_err("snow scalar poison must fail at entry");
    (error, snapshot, callback_called)
}

fn assert_snow_failure(
    error: LandSurfaceEnergyShadowError,
    snapshot: &Sha256Digest,
    expected_code: DirectSurfaceLiquidErrorCode,
) -> String {
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
        panic!("snow scalar poison must be canonical");
    };
    let failure = error.failure().expect("canonical snow scalar failure");
    assert_eq!(failure.code, expected_code);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::AtomicEnvelope);
    assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("production-hydrology")
    );
    assert_eq!(
        failure.context.ofe_id.as_ref().map(OfeId::as_str),
        Some("ofe-1")
    );
    assert_eq!(
        failure.context.tile_id.as_ref().map(TileId::as_str),
        Some("open")
    );
    assert_eq!(
        failure.context.surface_id.as_ref().map(SurfaceId::as_str),
        Some("surface:ofe-1:open")
    );
    assert_eq!(
        failure.context.source_id.as_ref().map(SourceId::as_str),
        Some("surface-store:ofe-1:open")
    );
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(snapshot.as_str())
    );
    failure
        .rollback
        .attempted_owner_sha256
        .clone()
        .expect("attempted request hash")
}

#[test]
fn every_snow_lane_scalar_rejects_invalid_before_unsupported_snow() {
    let mut attempted_sha256 = None;
    for runtime_carry in [false, true] {
        for field in 0..8 {
            for value in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let (error, snapshot, callback_called) =
                    execute_snow_poison(field, value, runtime_carry);
                assert!(!callback_called, "field {field} reached callback");
                let attempted =
                    assert_snow_failure(error, &snapshot, DirectSurfaceLiquidErrorCode::E003);
                if let Some(expected) = &attempted_sha256 {
                    assert_eq!(&attempted, expected, "field {field} request hash");
                } else {
                    attempted_sha256 = Some(attempted);
                }
            }

            let (error, snapshot, callback_called) =
                execute_snow_poison(field, 0.001, runtime_carry);
            assert!(!callback_called, "positive field {field} reached callback");
            let attempted =
                assert_snow_failure(error, &snapshot, DirectSurfaceLiquidErrorCode::E004);
            assert_eq!(attempted_sha256.as_ref(), Some(&attempted));
        }
    }
}
