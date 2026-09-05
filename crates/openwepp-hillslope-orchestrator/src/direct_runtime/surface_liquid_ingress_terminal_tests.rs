//! Terminal ingress regressions for `SC-SURFACELIQUID-001`.

use std::collections::BTreeMap;

use openwepp_kernel_contract::TransactionId;

use super::{
    DirectGroundIngressMode, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidIngressInput,
    DirectSurfaceLiquidParcelKind, DirectSurfaceLiquidPhase, DirectTileGroundIngress,
    DirectWb14CoupledChildBindingV1, INTERVAL_S, TimedParcel,
    execute_surface_liquid_ingress,
    execute_surface_liquid_ingress_with_parent_state_and_coupled_binding, initial_state,
    liquid_specific_enthalpy, one_tile_configuration, open_ingress, parameters,
    resource_candidate, route_runoff, routed_configuration,
};

fn exact_child_ingress(
    record: &super::DirectSurfaceLiquidConfigurationRecord,
    interval_s: f64,
) -> DirectTileGroundIngress {
    let temperature_k = 285.0;
    DirectTileGroundIngress::OpenRawPrecipitation {
        ofe_id: record.key.ofe_id.clone(),
        tile_id: record.key.tile_id.clone(),
        surface_id: record.key.surface_id.clone(),
        raw_precipitation: DirectIngressAmount {
            mass_kg_m2_tile_ground: 0.0,
            temperature_k,
            specific_liquid_enthalpy_j_kg: liquid_specific_enthalpy(temperature_k),
            start_s: 0.0,
            end_s: interval_s,
        },
    }
}

fn child_binding(
    parent_start_ns: u128,
    child_start_ns: u128,
    child_end_ns: u128,
    slab_byte: u8,
) -> DirectWb14CoupledChildBindingV1 {
    DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: 60.0_f64.to_bits(),
        coupled_parent_transaction_sha256: [11; 32],
        accepted_slab_sha256: [slab_byte; 32],
        parent_beginning_complete_owner_set_sha256: [13; 32],
        parent_support_start_ns: parent_start_ns,
        parent_support_end_ns: parent_start_ns + 1_800_000_000_000,
        child_support_start_ns: child_start_ns,
        child_support_end_ns: child_end_ns,
    }
}

#[test]
fn coupled_child_requires_and_advances_exact_predecessor_parent_cursor() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let persistent = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(9_201);
    let parent_start_ns = 3_u128 * 48 * 1_800_000_000_000;
    let first_end_ns = parent_start_ns + 60_000_000_000;
    let second_end_ns = first_end_ns + 60_000_000_000;
    let input = || DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: 60.0,
        tile_ingress: vec![exact_child_ingress(&configuration.records[0], 60.0)],
        wb14_parameters: parameters(&configuration),
    };
    let first_resource =
        resource_candidate(&configuration, &persistent, transaction_id, None, &[]);
    let first_binding = child_binding(parent_start_ns, parent_start_ns, first_end_ns, 17);
    let first = execute_surface_liquid_ingress_with_parent_state_and_coupled_binding(
        &configuration,
        &first_resource,
        &input(),
        None,
        false,
        Some(first_binding),
    )
    .expect("first exact coupled child");
    let first_parent = first.parent_working_state().expect("open parent custody");
    let second_resource = resource_candidate(
        &configuration,
        first_parent.candidate_state(),
        transaction_id,
        None,
        &[],
    );
    let second_binding = child_binding(parent_start_ns, first_end_ns, second_end_ns, 19);
    let second = execute_surface_liquid_ingress_with_parent_state_and_coupled_binding(
        &configuration,
        &second_resource,
        &input(),
        Some(first_parent),
        false,
        Some(second_binding),
    )
    .expect("adjacent successor coupled child");
    let second_parent = second.parent_working_state().expect("successor parent custody");
    let restart: serde_json::Value = serde_json::from_slice(
        &second_parent
            .restart_bytes(&configuration)
            .expect("canonical successor parent"),
    )
    .expect("successor parent JSON");
    assert_eq!(restart["accepted_until_ns"], serde_json::json!(second_end_ns));

    let before = second_resource.clone();
    let error = execute_surface_liquid_ingress_with_parent_state_and_coupled_binding(
        &configuration,
        &second_resource,
        &input(),
        None,
        false,
        Some(second_binding),
    )
    .expect_err("successor without predecessor parent custody must fail closed");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E008);
    assert_eq!(second_resource, before);
}

#[test]
fn finite_ingress_enthalpy_overflow_fails_before_candidate() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(393);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], f64::MAX / 2.0)],
        wb14_parameters: parameters(&configuration),
    };
    let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("finite parcel enthalpy overflow must fail closed");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
}

#[test]
fn finite_routing_area_underflow_fails_before_receipt() {
    let mut configuration = routed_configuration();
    configuration.records[0].ofe_area_m2 = f64::MIN_POSITIVE;
    configuration.records[1].ofe_area_m2 = f64::MAX;
    configuration.configuration_sha256 = configuration.recomputed_sha256().expect("digest");
    configuration.validate().expect("finite extreme areas");
    let source = &configuration.records[0];
    let parcel = TimedParcel {
        parcel_id: "underflow-route".to_owned(),
        origin_store_key: source.key.clone(),
        recipient_store_key: source.key.clone(),
        basis_ofe_id: source.key.ofe_id.clone(),
        kind: DirectSurfaceLiquidParcelKind::RawPrecipitation,
        start_s: 0.0,
        end_s: INTERVAL_S,
        mass_kg_m2_basis_ofe_ground: 1.0,
        temperature_k: 273.15,
        enthalpy_j_m2_basis_ofe_ground: 1.0,
    };
    let mut pending = BTreeMap::new();
    let mut receipts = Vec::new();
    let error = route_runoff(
        &configuration,
        &source.key.ofe_id,
        vec![parcel],
        &mut pending,
        &mut receipts,
        TransactionId(394),
    )
    .expect_err("finite area-ratio underflow must fail closed");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
    assert!(pending.is_empty());
    assert!(receipts.is_empty());
}

#[test]
fn wb14_failure_preserves_every_resource_candidate_byte() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let first_transaction_id = TransactionId(401);
    let first_resource =
        resource_candidate(&configuration, &beginning, first_transaction_id, None, &[]);
    let first = execute_surface_liquid_ingress(
        &configuration,
        &first_resource,
        &DirectSurfaceLiquidIngressInput {
            transaction_id: first_transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
            wb14_parameters: vec![DirectOfeWb14Parameters {
                ofe_id: configuration.ofe_topology[0].clone(),
                effective_conductivity_m_s: 1.0e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 1.0,
            }],
        },
    )
    .expect("first continuation");
    assert!(first.ending_state.continuations[0].cumulative_infiltration_m > 0.0);
    let transaction_id = TransactionId(402);
    let resource = resource_candidate(
        &configuration,
        &first.ending_state,
        transaction_id,
        Some(first_transaction_id),
        &[],
    );
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 1,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: configuration.ofe_topology[0].clone(),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.0,
        }],
    };
    let before_candidate = resource.clone();
    let before = (
        resource
            .beginning_state()
            .canonical_bytes(&configuration)
            .expect("beginning bytes before"),
        resource
            .working_state()
            .canonical_bytes(&configuration)
            .expect("working bytes before"),
    );
    let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("invalid continuation bound");
    let failure = error.failure().expect("canonical failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E008);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());
    let after = (
        resource
            .beginning_state()
            .canonical_bytes(&configuration)
            .expect("beginning bytes after"),
        resource
            .working_state()
            .canonical_bytes(&configuration)
            .expect("working bytes after"),
    );
    assert_eq!(after, before);
    assert_eq!(resource, before_candidate);
}

#[test]
fn sealed_ingress_candidate_reconstructs_and_rejects_forgery() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(412);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let mut candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("valid candidate");
    candidate
        .validate(&configuration, &resource, &input)
        .expect("candidate reconstruction");
    candidate.ending_state.records[0].liquid_kg_m2_tile += 0.25;
    let error = candidate
        .validate(&configuration, &resource, &input)
        .expect_err("forged ending state");
    let failure = error.failure().expect("canonical failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E009);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}
