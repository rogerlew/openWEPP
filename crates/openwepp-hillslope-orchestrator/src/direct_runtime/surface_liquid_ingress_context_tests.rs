//! Public canonical-context poisons for the surface-liquid ingress boundary.

use openwepp_kernel_contract::TransactionId;

use super::tests::{
    initial_state, one_tile_configuration, open_ingress, parameters, resource_candidate,
};
use super::{
    DirectGroundIngressMode, DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidIngressInput,
    DirectSurfaceLiquidPhase, DirectTileGroundIngress, INTERVAL_S, execute_surface_liquid_ingress,
};

#[test]
fn cadence_failure_is_e008_with_exact_transaction_and_attempt_hash() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(411);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S + 1.0,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("wrong cadence");
    let failure = error.failure().expect("canonical failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E008);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
fn public_ingress_failures_retain_available_owner_and_exact_offender_context() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let resource_transaction = TransactionId(4_120);
    let resource = resource_candidate(&configuration, &beginning, resource_transaction, None, &[]);
    let mut input = DirectSurfaceLiquidIngressInput {
        transaction_id: TransactionId(4_121),
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };

    let mismatch = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("transaction mismatch");
    let failure = mismatch.failure().expect("canonical transaction failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.context.transaction_id, Some(input.transaction_id));
    assert_eq!(
        failure.context.owner_id.as_ref(),
        Some(&configuration.owner_id)
    );
    assert_eq!(failure.context.ofe_id, None);
    assert_eq!(failure.context.tile_id, None);

    input.transaction_id = resource_transaction;
    let DirectTileGroundIngress::OpenRawPrecipitation {
        raw_precipitation, ..
    } = &mut input.tile_ingress[0]
    else {
        panic!("open ingress fixture");
    };
    raw_precipitation.mass_kg_m2_tile_ground = f64::NAN;
    let invalid = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("invalid direct ingress amount");
    let failure = invalid.failure().expect("canonical direct-ingress failure");
    let key = &configuration.records[0].key;
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.context.transaction_id, Some(resource_transaction));
    assert_eq!(
        failure.context.owner_id.as_ref(),
        Some(&configuration.owner_id)
    );
    assert_eq!(failure.context.ofe_id.as_ref(), Some(&key.ofe_id));
    assert_eq!(failure.context.tile_id.as_ref(), Some(&key.tile_id));
    assert_eq!(failure.context.surface_id.as_ref(), Some(&key.surface_id));
    assert_eq!(failure.context.source_id.as_ref(), Some(&key.source_id));
}
