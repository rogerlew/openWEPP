use super::tests::configuration;
use super::v2_ingress_adapter::{
    DirectWb14ParentWorkingStateV2, apply_ordinary_finalized_uses_to_phase_adjusted_v2,
    execute_surface_liquid_ingress_v2,
    execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding,
    prepare_surface_liquid_resource_candidate_v2,
    prepare_surface_liquid_resource_candidate_v2_with_phase_capacity_spills,
    reset_surface_resource_validation_counters_v2, reset_wb14_parent_v2_handoff_counters,
    surface_resource_validation_counters_v2, wb14_parent_v2_handoff_counters,
};
use super::*;
use crate::direct_runtime::DirectSurfaceLiquidIngressCandidateV2;
use crate::direct_runtime::surface_liquid_ingress::{
    DirectCanopyLiquidRelease, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectSurfaceLiquidIngressInput, DirectSurfaceLiquidParcelKind, DirectTileGroundIngress,
    DirectWb14CoupledChildBindingV1,
};
use openwepp_land_surface_energy::{
    EndingLitterPhaseState, LITTER_ICE_HEAT_CAPACITY_J_KG_K, LitterPhaseCapacitySpillV1,
    LitterPhaseConfiguration, REFERENCE_TEMPERATURE_K, RequestingComponent, Sha256Digest,
    StandGroundWaterAmountBasis, WATER_HEAT_CAPACITY_J_KG_K, WaterAmount, WaterAuthorization,
    WaterAuthorizationReason, retained_litter_phase_ending_v1,
};

fn digest(byte: char) -> String {
    std::iter::repeat_n(byte, 64).collect()
}

fn configuration_v2() -> SurfaceLiquidConfigurationV2 {
    let parent = configuration();
    configuration_v2_from_parent(parent)
}

fn configuration_v2_one_litter() -> SurfaceLiquidConfigurationV2 {
    let base = configuration();
    let mut litter = base
        .records
        .iter()
        .find(|record| record.key.surface_class == SurfaceClass::ForestLitter)
        .expect("litter record")
        .clone();
    litter.tile_fraction = 1.0;
    let parent = DirectSurfaceLiquidConfiguration::new(
        base.owner_id,
        base.run_id,
        base.ofe_topology,
        base.ofe_bindings,
        vec![litter],
    )
    .expect("one-litter parent configuration");
    configuration_v2_from_parent(parent)
}

fn configuration_v2_from_parent(
    parent: DirectSurfaceLiquidConfiguration,
) -> SurfaceLiquidConfigurationV2 {
    let depths = parent
        .records
        .iter()
        .filter(|record| record.key.surface_class == SurfaceClass::ForestLitter)
        .map(|record| (record.key.clone(), 0.03125))
        .collect();
    let model = SurfaceLiquidOwnerModelDefinitionV2::new(digest('1'), digest('2'), digest('3'))
        .expect("V2 model");
    SurfaceLiquidConfigurationV2::new(parent, model, &depths).expect("V2 configuration")
}

fn owner_v2(configuration: &SurfaceLiquidConfigurationV2) -> SurfaceLiquidOwnerEnvelopeV2 {
    let liquid = configuration
        .parent()
        .records
        .iter()
        .map(|record| (record.key.clone(), 0.25))
        .collect();
    let ice = configuration
        .parent()
        .records
        .iter()
        .map(|record| {
            (
                record.key.clone(),
                if record.key.surface_class == SurfaceClass::ForestLitter {
                    0.375
                } else {
                    0.0
                },
            )
        })
        .collect();
    let enthalpy = configuration
        .parent()
        .records
        .iter()
        .map(|record| {
            (
                record.key.clone(),
                if record.key.surface_class == SurfaceClass::ForestLitter {
                    1250.0
                } else {
                    0.0
                },
            )
        })
        .collect();
    let state = SurfaceLiquidOwnedStateV2::new_initial(configuration, &liquid, &ice, &enthalpy, 3)
        .expect("initial V2 state");
    SurfaceLiquidOwnerEnvelopeV2::wrap_v2(configuration, state).expect("V2 envelope")
}

fn zero_closure(owner: &SurfaceLiquidOwnerEnvelopeV2) -> Vec<SurfaceLiquidOwnerClosureRecordV2> {
    owner
        .v2_state()
        .expect("V2 state")
        .records()
        .iter()
        .map(|record| SurfaceLiquidOwnerClosureRecordV2 {
            key: record.key.clone(),
            liquid_debit_kg_m2_tile: 0.0,
            liquid_credit_kg_m2_tile: 0.0,
            ice_debit_kg_m2_tile: 0.0,
            ice_credit_kg_m2_tile: 0.0,
        })
        .collect()
}

fn amount(mass: f64, temperature_k: f64, interval_s: f64) -> DirectIngressAmount {
    DirectIngressAmount {
        mass_kg_m2_tile_ground: mass,
        temperature_k,
        specific_liquid_enthalpy_j_kg: 4218.0 * (temperature_k - 273.15),
        start_s: 0.0,
        end_s: interval_s,
    }
}

fn input(
    configuration: &SurfaceLiquidConfigurationV2,
    transaction_id: TransactionId,
    interval_s: f64,
) -> DirectSurfaceLiquidIngressInput {
    let tile_ingress = configuration
        .parent()
        .records
        .iter()
        .map(|record| match record.ground_ingress_mode {
            DirectGroundIngressMode::OpenRawPrecipitation => {
                DirectTileGroundIngress::OpenRawPrecipitation {
                    ofe_id: record.key.ofe_id.clone(),
                    tile_id: record.key.tile_id.clone(),
                    surface_id: record.key.surface_id.clone(),
                    raw_precipitation: amount(0.20, 274.0, interval_s),
                }
            }
            DirectGroundIngressMode::CoveredCanopyRelease => {
                DirectTileGroundIngress::CoveredCanopyRelease {
                    ofe_id: record.key.ofe_id.clone(),
                    tile_id: record.key.tile_id.clone(),
                    surface_id: record.key.surface_id.clone(),
                    release: DirectCanopyLiquidRelease {
                        throughfall: amount(0.30, 275.0, interval_s),
                        initial_drainage: amount(0.0, 273.15, interval_s),
                        second_drainage: amount(0.0, 273.15, interval_s),
                        stemflow: amount(0.0, 273.15, interval_s),
                    },
                }
            }
        })
        .collect();
    DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s,
        tile_ingress,
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: configuration.parent().ofe_topology[0].clone(),
            effective_conductivity_m_s: 1.0e-12,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.2,
        }],
    }
}

fn coupled_child_binding(
    configuration: &SurfaceLiquidConfigurationV2,
    child_start_ns: u128,
    child_end_ns: u128,
    slab_byte: u8,
) -> DirectWb14CoupledChildBindingV1 {
    let parent_start_ns = 3_u128 * 48 * 1_800_000_000_000;
    assert_eq!(configuration.parent().ofe_topology.len(), 1);
    DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: 1_800.0_f64.to_bits(),
        coupled_parent_transaction_sha256: [11; 32],
        accepted_slab_sha256: [slab_byte; 32],
        parent_beginning_complete_owner_set_sha256: [13; 32],
        parent_support_start_ns: parent_start_ns,
        parent_support_end_ns: parent_start_ns + 1_800_000_000_000,
        child_support_start_ns: child_start_ns,
        child_support_end_ns: child_end_ns,
    }
}

fn state_bytes(
    configuration: &SurfaceLiquidConfigurationV2,
    owner: &SurfaceLiquidOwnerEnvelopeV2,
) -> Vec<u8> {
    owner
        .canonical_bytes(configuration.parent(), Some(configuration))
        .expect("canonical V2 envelope")
}

fn phase_spill_fixture() -> (
    SurfaceLiquidConfigurationV2,
    SurfaceLiquidOwnerEnvelopeV2,
    SurfaceLiquidOwnerEnvelopeV2,
    LitterPhaseCapacitySpillV1,
    Vec<SurfaceLiquidOwnerClosureRecordV2>,
    DirectSurfaceLiquidIngressInput,
) {
    let configuration = configuration_v2_one_litter();
    let seed = owner_v2(&configuration);
    let configured = &configuration.parent().records[0];
    let extension = &configuration.records()[0];
    let transaction = TransactionId(777);
    let temperature_k = 280.0;
    let phase_configuration = LitterPhaseConfiguration {
        litter_depth_m: extension.litter_depth_m.expect("litter depth"),
        dry_heat_capacity_j_m2_k: 3_000.0,
        liquid_capacity_kg_m2_tile: configured.capacity_kg_m2_tile,
        ice_capacity_kg_m2_tile: extension
            .litter_ice_capacity_kg_m2_tile
            .expect("ice capacity"),
    };
    let beginning = seed
        .try_replace_v2_state(
            &configuration,
            vec![SurfaceLiquidStateRecordV2 {
                key: configured.key.clone(),
                liquid_kg_m2_tile: configured.capacity_kg_m2_tile,
                litter_ice_kg_m2_tile: 0.5,
                surface_enthalpy_j_m2_tile: 0.0,
                last_accepted_transaction_id: None,
            }],
            seed.v2_state()
                .expect("seed state")
                .continuations()
                .to_vec(),
        )
        .expect("saturated phase beginning");
    let raw_capacity = phase_configuration.dry_heat_capacity_j_m2_k
        + (configured.capacity_kg_m2_tile + 0.1) * WATER_HEAT_CAPACITY_J_KG_K
        + 0.4 * LITTER_ICE_HEAT_CAPACITY_J_KG_K;
    let raw = EndingLitterPhaseState {
        liquid_kg_m2_tile: configured.capacity_kg_m2_tile + 0.1,
        ice_kg_m2_tile: 0.4,
        sensible_energy_j_m2_tile: raw_capacity * (temperature_k - REFERENCE_TEMPERATURE_K),
        temperature_k,
        heat_capacity_j_m2_k: raw_capacity,
    };
    let retained =
        retained_litter_phase_ending_v1(phase_configuration, raw).expect("retained phase ending");
    let spill_mass = raw.liquid_kg_m2_tile - retained.liquid_kg_m2_tile;
    let spill_specific = WATER_HEAT_CAPACITY_J_KG_K * (temperature_k - REFERENCE_TEMPERATURE_K);
    let spill = LitterPhaseCapacitySpillV1 {
        phase_receipt_sha256: Sha256Digest::try_new(digest('a')).expect("receipt digest"),
        lse_configuration_sha256: Sha256Digest::try_new(digest('b')).expect("LSE digest"),
        transaction_id: transaction,
        ofe_id: configured.key.ofe_id.clone(),
        tile_id: configured.key.tile_id.clone(),
        surface_owner_id: configuration.parent().owner_id.clone(),
        support_start_ns: 0,
        support_end_ns: 1_800_000_000_000,
        liquid_capacity_kg_m2_tile: configured.capacity_kg_m2_tile,
        raw_ending: raw,
        spill_liquid_kg_m2_tile: spill_mass,
        spill_specific_sensible_enthalpy_j_kg: spill_specific,
        spill_sensible_energy_j_m2_tile: spill_mass * spill_specific,
        retained_ending: retained,
    };
    let phase_adjusted = beginning
        .try_replace_v2_state(
            &configuration,
            vec![SurfaceLiquidStateRecordV2 {
                key: configured.key.clone(),
                liquid_kg_m2_tile: retained.liquid_kg_m2_tile,
                litter_ice_kg_m2_tile: retained.ice_kg_m2_tile,
                surface_enthalpy_j_m2_tile: retained.sensible_energy_j_m2_tile,
                last_accepted_transaction_id: None,
            }],
            beginning
                .v2_state()
                .expect("beginning state")
                .continuations()
                .to_vec(),
        )
        .expect("retained phase owner");
    let closure = vec![SurfaceLiquidOwnerClosureRecordV2 {
        key: configured.key.clone(),
        liquid_debit_kg_m2_tile: spill_mass,
        liquid_credit_kg_m2_tile: spill_mass,
        ice_debit_kg_m2_tile: spill_mass,
        ice_credit_kg_m2_tile: 0.0,
    }];
    let mut ingress = input(&configuration, transaction, 1_800.0);
    let DirectTileGroundIngress::CoveredCanopyRelease { release, .. } =
        &mut ingress.tile_ingress[0]
    else {
        panic!("one-litter fixture must use covered ingress");
    };
    for amount in [
        &mut release.throughfall,
        &mut release.initial_drainage,
        &mut release.second_drainage,
        &mut release.stemflow,
    ] {
        amount.mass_kg_m2_tile_ground = 0.0;
    }
    (
        configuration,
        beginning,
        phase_adjusted,
        spill,
        closure,
        ingress,
    )
}

fn execute_phase_spill_fixture() -> (
    DirectSurfaceLiquidIngressCandidateV2,
    LitterPhaseCapacitySpillV1,
) {
    let (configuration, beginning, phase_adjusted, spill, closure, ingress) = phase_spill_fixture();
    let resource = prepare_surface_liquid_resource_candidate_v2_with_phase_capacity_spills(
        &configuration,
        &beginning,
        &phase_adjusted,
        ingress.transaction_id,
        &closure,
        std::slice::from_ref(&spill),
    )
    .expect("typed phase-spill resource");
    let candidate = execute_surface_liquid_ingress_v2(&configuration, &resource, &ingress)
        .expect("phase-spill WB14 candidate");
    (candidate, spill)
}

fn ordinary_surface_protocol(
    configuration: &SurfaceLiquidConfigurationV2,
    transaction_id: TransactionId,
    debit_tile: f64,
) -> (Vec<WaterAmount>, Vec<WaterAuthorization>, Vec<WaterAmount>) {
    let configured = configuration
        .parent()
        .records
        .first()
        .expect("surface configuration row");
    let key = openwepp_land_surface_energy::GroundWaterKey {
        transaction_id,
        requesting_owner_id: configuration.parent().owner_id.clone(),
        requesting_component: RequestingComponent::GroundSurface,
        ofe_id: configured.key.ofe_id.clone(),
        requesting_tile_id: configured.key.tile_id.clone(),
        occupancy_id: None,
        surface_id: Some(configured.key.surface_id.clone()),
        surface_class: Some(configured.key.surface_class),
        source_type: configured.key.source_type,
        source_id: configured.key.source_id.clone(),
        source_tile_id: Some(configured.key.tile_id.clone()),
        soil_layer_id: None,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
    };
    let amount = debit_tile * configured.tile_fraction;
    (
        vec![WaterAmount {
            key: key.clone(),
            amount_kg_m2_stand_ground: amount,
        }],
        vec![WaterAuthorization {
            key: key.clone(),
            amount_kg_m2_stand_ground: amount,
            reason: WaterAuthorizationReason::FullSupply,
        }],
        vec![WaterAmount {
            key,
            amount_kg_m2_stand_ground: amount,
        }],
    )
}

#[test]
fn heterogeneous_v3_resource_join_debits_ordinary_finalized_use_once() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(780);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("native resource");
    let (requests, authorizations, uses) =
        ordinary_surface_protocol(&configuration, transaction, 0.0625);
    let joined = apply_ordinary_finalized_uses_to_phase_adjusted_v2(
        &configuration,
        &resource,
        &requests,
        &authorizations,
        &uses,
        &[],
    )
    .expect("heterogeneous resource join");
    let beginning_liquid = beginning.v2_state().expect("beginning").records()[0].liquid_kg_m2_tile;
    let joined_liquid =
        joined.phase_adjusted_state().expect("joined").records()[0].liquid_kg_m2_tile;
    assert_eq!(
        joined_liquid.to_bits(),
        (beginning_liquid - 0.0625).to_bits()
    );
    assert!(
        apply_ordinary_finalized_uses_to_phase_adjusted_v2(
            &configuration,
            &joined,
            &requests,
            &authorizations,
            &uses,
            &[],
        )
        .is_err(),
        "the typed join cannot debit twice",
    );
}

#[test]
fn heterogeneous_v3_resource_join_accepts_finalized_use_below_authorization() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(784);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("native resource");
    let (mut requests, mut authorizations, uses) =
        ordinary_surface_protocol(&configuration, transaction, 0.03125);
    requests[0].amount_kg_m2_stand_ground *= 2.0;
    authorizations[0].amount_kg_m2_stand_ground *= 1.5;
    let joined = apply_ordinary_finalized_uses_to_phase_adjusted_v2(
        &configuration,
        &resource,
        &requests,
        &authorizations,
        &uses,
        &[],
    )
    .expect("canonical F below A below D");
    assert_eq!(
        joined.phase_adjusted_state().expect("joined").records()[0]
            .liquid_kg_m2_tile
            .to_bits(),
        (0.25_f64 - 0.03125).to_bits(),
    );
}

#[test]
fn heterogeneous_v3_resource_join_rejects_out_of_bound_or_nonfinite_amounts() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(785);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("native resource");
    let (requests, authorizations, uses) =
        ordinary_surface_protocol(&configuration, transaction, 0.03125);
    for invalid in [
        -0.01,
        f64::NAN,
        authorizations[0].amount_kg_m2_stand_ground * 2.0,
    ] {
        let mut poisoned = uses.clone();
        poisoned[0].amount_kg_m2_stand_ground = invalid;
        assert!(
            apply_ordinary_finalized_uses_to_phase_adjusted_v2(
                &configuration,
                &resource,
                &requests,
                &authorizations,
                &poisoned,
                &[],
            )
            .is_err()
        );
    }
}

#[test]
fn heterogeneous_v3_resource_join_retains_native_phase_and_spill_custody() {
    let (configuration, beginning, phase_adjusted, spill, closure, _) = phase_spill_fixture();
    let resource = prepare_surface_liquid_resource_candidate_v2_with_phase_capacity_spills(
        &configuration,
        &beginning,
        &phase_adjusted,
        spill.transaction_id,
        &closure,
        std::slice::from_ref(&spill),
    )
    .expect("native spill resource");
    let joined = apply_ordinary_finalized_uses_to_phase_adjusted_v2(
        &configuration,
        &resource,
        &[],
        &[],
        &[],
        &[],
    )
    .expect("empty ordinary partition");
    assert_eq!(joined.phase_capacity_spills(), std::slice::from_ref(&spill));
    assert_eq!(joined.phase_adjusted_owner(), &phase_adjusted);
}

#[test]
fn heterogeneous_v3_resource_join_rejects_native_vapor_replay_as_ordinary_use() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(781);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("native resource");
    let (requests, authorizations, uses) =
        ordinary_surface_protocol(&configuration, transaction, 0.03125);
    let joined = apply_ordinary_finalized_uses_to_phase_adjusted_v2(
        &configuration,
        &resource,
        &requests,
        &authorizations,
        &uses,
        &[],
    )
    .expect("first exact join");
    assert!(
        apply_ordinary_finalized_uses_to_phase_adjusted_v2(
            &configuration,
            &joined,
            &requests,
            &authorizations,
            &uses,
            &[],
        )
        .is_err()
    );
}

#[test]
fn heterogeneous_v3_resource_join_rejects_foreign_or_duplicate_finalized_use() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(782);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("native resource");
    let (requests, authorizations, mut uses) =
        ordinary_surface_protocol(&configuration, transaction, 0.03125);
    uses.push(uses[0].clone());
    assert!(
        apply_ordinary_finalized_uses_to_phase_adjusted_v2(
            &configuration,
            &resource,
            &requests,
            &authorizations,
            &uses,
            &[],
        )
        .is_err()
    );
}

#[test]
fn heterogeneous_v3_resource_join_executes_one_ingress() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(783);
    let native = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("native resource");
    let (requests, authorizations, uses) =
        ordinary_surface_protocol(&configuration, transaction, 0.03125);
    let joined = apply_ordinary_finalized_uses_to_phase_adjusted_v2(
        &configuration,
        &native,
        &requests,
        &authorizations,
        &uses,
        &[],
    )
    .expect("joined resource");
    let accepted = execute_surface_liquid_ingress_v2(
        &configuration,
        &joined,
        &input(&configuration, transaction, 1800.0),
    )
    .expect("one ingress");
    assert!(
        accepted
            .inner()
            .wb14_calls_by_ofe()
            .values()
            .all(|calls| *calls == 1)
    );
}

#[test]
fn litter_phase_capacity_spill_routes_once_through_wb14() {
    let (candidate, spill) = execute_phase_spill_fixture();
    let receipts = candidate
        .inner()
        .receipts()
        .iter()
        .filter(|receipt| receipt.kind == DirectSurfaceLiquidParcelKind::LitterPhaseOverflow)
        .collect::<Vec<_>>();
    let sources = receipts
        .iter()
        .map(|receipt| receipt.source_parcel_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(sources.len(), 1, "one internally owned source parcel");
    assert_eq!(candidate.inner().wb14_calls_by_ofe().len(), 1);
    assert!(
        candidate
            .inner()
            .wb14_calls_by_ofe()
            .values()
            .all(|calls| *calls == 1),
        "ordinary WB14 must consume the internal spill exactly once",
    );
    assert_eq!(
        receipts
            .iter()
            .map(|receipt| receipt.mass_kg_m2_basis_ofe_ground)
            .sum::<f64>()
            .to_bits(),
        spill.spill_liquid_kg_m2_tile.to_bits(),
    );
}

#[test]
fn litter_phase_capacity_spill_rejects_condensation_alias() {
    let (candidate, _) = execute_phase_spill_fixture();
    assert!(candidate.inner().receipts().iter().any(|receipt| {
        receipt.kind == DirectSurfaceLiquidParcelKind::LitterPhaseOverflow
            && receipt
                .source_parcel_id
                .starts_with("litter-phase-overflow:")
    }));
    assert!(!candidate.inner().receipts().iter().any(|receipt| {
        receipt.kind == DirectSurfaceLiquidParcelKind::CondensationOverflow
            || receipt.source_parcel_id.starts_with("condensation:")
    }));
}

#[test]
fn current_ingress_uses_actual_wb14_with_liquid_only_and_carries_ice_bitwise() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let phase_adjusted = beginning.clone();
    let transaction = TransactionId(701);
    let closure = zero_closure(&beginning);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &phase_adjusted,
        transaction,
        &closure,
    )
    .expect("V2 resource candidate");
    let ingress = input(&configuration, transaction, 1800.0);
    let candidate = execute_surface_liquid_ingress_v2(&configuration, &resource, &ingress)
        .expect("V2 current-ingress candidate");
    candidate
        .validate(&configuration, &resource, &ingress)
        .expect("independent V2 candidate reconstruction");

    let phase_state = phase_adjusted.v2_state().expect("phase state");
    let ending = candidate
        .ending_owner()
        .v2_state()
        .expect("ending V2 state");
    for (phase, ending) in phase_state.records().iter().zip(ending.records()) {
        assert_eq!(
            ending.litter_ice_kg_m2_tile.to_bits(),
            phase.litter_ice_kg_m2_tile.to_bits(),
            "WB14 must not consume or delete litter ice"
        );
        assert!(ending.liquid_kg_m2_tile >= phase.liquid_kg_m2_tile);
    }
    let supplied = candidate
        .inner()
        .ledgers()
        .iter()
        .map(|ledger| ledger.ingress_mass_kg_m2_ofe_ground)
        .sum::<f64>();
    let liquid_only_supply = 0.4_f64 * 0.20 + 0.6 * 0.30;
    assert_eq!(supplied.to_bits(), liquid_only_supply.to_bits());
    assert_ne!(
        supplied.to_bits(),
        (liquid_only_supply + 0.6 * 0.375).to_bits()
    );

    let direct_v1 = crate::direct_runtime::surface_liquid_ingress::execute_surface_liquid_ingress(
        configuration.parent(),
        resource.liquid_arithmetic(),
        &ingress,
    )
    .expect("unchanged V1 WB14 path");
    assert_eq!(candidate.inner().ending_state(), direct_v1.ending_state());
    assert_eq!(candidate.inner().ledgers(), direct_v1.ledgers());
    assert_eq!(candidate.inner().receipts(), direct_v1.receipts());
}

#[test]
fn resource_and_ingress_poisons_fail_closed_without_mutating_v2_bytes() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let before = state_bytes(&configuration, &beginning);
    let transaction = TransactionId(702);
    let mut omitted = zero_closure(&beginning);
    omitted.pop();
    assert!(
        prepare_surface_liquid_resource_candidate_v2(
            &configuration,
            &beginning,
            &beginning,
            transaction,
            &omitted,
        )
        .is_err()
    );

    let mut doubled = zero_closure(&beginning);
    doubled[0].liquid_debit_kg_m2_tile = 0.25;
    assert!(
        prepare_surface_liquid_resource_candidate_v2(
            &configuration,
            &beginning,
            &beginning,
            transaction,
            &doubled,
        )
        .is_err()
    );

    let v1 = super::tests::state(configuration.parent());
    let mixed = SurfaceLiquidOwnerEnvelopeV2::wrap_v1(configuration.parent(), v1, digest('3'))
        .expect("tagged V1 envelope");
    assert!(
        prepare_surface_liquid_resource_candidate_v2(
            &configuration,
            &mixed,
            &beginning,
            transaction,
            &zero_closure(&beginning),
        )
        .is_err()
    );

    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("valid resource");
    let mut poisoned_input = input(&configuration, transaction, 1800.0);
    poisoned_input.wb14_parameters[0].effective_conductivity_m_s = f64::NAN;
    assert!(execute_surface_liquid_ingress_v2(&configuration, &resource, &poisoned_input).is_err());
    assert_eq!(state_bytes(&configuration, &beginning), before);
    assert_eq!(
        state_bytes(&configuration, resource.beginning_owner()),
        before
    );
    assert_eq!(
        state_bytes(&configuration, resource.phase_adjusted_owner()),
        before
    );
}

#[test]
fn v2_parent_restart_and_two_children_preserve_ice_and_finalize_once() {
    let configuration = configuration_v2_one_litter();
    let beginning = owner_v2(&configuration);
    let first_transaction = TransactionId(703);
    let first_resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        first_transaction,
        &zero_closure(&beginning),
    )
    .expect("first child resource");
    let mut first_input = input(&configuration, first_transaction, 900.0);
    zero_ingress_mass(&mut first_input);
    let first = execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding(
        &configuration,
        &first_resource,
        &first_input,
        None,
        false,
        None,
    )
    .expect("first child");
    let parent = first.parent_working_state().expect("open V2 parent");
    let legacy_parent = first
        .inner()
        .parent_working_state()
        .expect("open V1 arithmetic parent");
    reset_wb14_parent_v2_handoff_counters();
    let handoff = parent
        .validated_handoff(&configuration)
        .expect("validated in-process V2 parent handoff");
    assert!(handoff.has_same_liquid_arithmetic(legacy_parent));
    assert_eq!(wb14_parent_v2_handoff_counters(), (1, 0, 2));
    let restart = parent
        .restart_bytes(&configuration)
        .expect("V2 parent restart");
    assert_eq!(wb14_parent_v2_handoff_counters(), (1, 1, 6));
    let restored = DirectWb14ParentWorkingStateV2::from_restart_bytes(&configuration, &restart)
        .expect("V2 parent restart replay");
    assert_eq!(restored, *parent);

    let second_beginning = restored.candidate_owner().clone();
    let second_transaction = TransactionId(704);
    let second_resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &second_beginning,
        &second_beginning,
        second_transaction,
        &zero_closure(&second_beginning),
    )
    .expect("second child resource");
    let mut second_input = input(&configuration, second_transaction, 900.0);
    zero_ingress_mass(&mut second_input);
    let second = execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding(
        &configuration,
        &second_resource,
        &second_input,
        Some(&restored),
        true,
        None,
    )
    .expect("final child");
    assert!(second.parent_working_state().is_none());
    let initial_ice = beginning.v2_state().expect("initial").records()[0].litter_ice_kg_m2_tile;
    let ending_ice =
        second.ending_owner().v2_state().expect("ending").records()[0].litter_ice_kg_m2_tile;
    assert_eq!(ending_ice.to_bits(), initial_ice.to_bits());
}

#[test]
fn staged_v1_parent_advances_native_v2_into_the_real_coupled_child_join() {
    let configuration = configuration_v2_one_litter();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(907);
    let parent_start_ns = 3_u128 * 48 * 1_800_000_000_000;
    let first_end_ns = parent_start_ns + 60_000_000_000;
    let second_end_ns = first_end_ns + 60_000_000_000;
    let parent_end_ns = parent_start_ns + 1_800_000_000_000;
    let mut child_input = input(&configuration, transaction, 60.0);
    zero_ingress_mass(&mut child_input);

    let first_resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("first coupled-child resource");
    let first = execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding(
        &configuration,
        &first_resource,
        &child_input,
        None,
        false,
        Some(coupled_child_binding(
            &configuration,
            parent_start_ns,
            first_end_ns,
            17,
        )),
    )
    .expect("first real coupled child");
    let stale_native = first
        .parent_working_state()
        .expect("first native parent")
        .clone();
    let adopted_native = DirectWb14ParentWorkingStateV2::try_from_validated_liquid_arithmetic(
        &configuration,
        &beginning,
        first
            .inner()
            .parent_working_state()
            .expect("first authoritative V1 parent"),
    )
    .expect("adopt an already-open V1 parent into native V2 custody");
    assert_eq!(adopted_native, stale_native);

    let second_beginning = stale_native.candidate_owner().clone();
    let second_resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &second_beginning,
        &second_beginning,
        transaction,
        &zero_closure(&second_beginning),
    )
    .expect("second coupled-child resource");
    let second = execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding(
        &configuration,
        &second_resource,
        &child_input,
        Some(&stale_native),
        false,
        Some(coupled_child_binding(
            &configuration,
            first_end_ns,
            second_end_ns,
            19,
        )),
    )
    .expect("second real coupled child");
    let authoritative_legacy = second
        .inner()
        .parent_working_state()
        .expect("advanced authoritative V1 parent");
    let expected_native = second
        .parent_working_state()
        .expect("advanced native V2 parent");

    let staged = stale_native
        .try_stage_validated_liquid_arithmetic(&configuration, authoritative_legacy)
        .expect("stage authenticated legacy parent without restart bytes");
    assert_eq!(&staged, expected_native);

    let third_beginning = staged.candidate_owner().clone();
    let third_resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &third_beginning,
        &third_beginning,
        transaction,
        &zero_closure(&third_beginning),
    )
    .expect("third coupled-child resource");
    let mut final_child_input = input(&configuration, transaction, 1_680.0);
    zero_ingress_mass(&mut final_child_input);
    let final_child = execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding(
        &configuration,
        &third_resource,
        &final_child_input,
        Some(&staged),
        true,
        Some(coupled_child_binding(
            &configuration,
            second_end_ns,
            parent_end_ns,
            23,
        )),
    )
    .expect("staged native parent reaches the real child-support join");
    assert!(final_child.parent_working_state().is_none());
    let finalized_owner = staged
        .try_finalize_validated_liquid_owner(&configuration, final_child.inner().ending_state())
        .expect("finalize authenticated V1 child into native V2 owner");
    assert_eq!(&finalized_owner, final_child.ending_owner());
}

#[test]
fn surface_resource_candidate_validates_each_revision_once() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(901);
    reset_surface_resource_validation_counters_v2();
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("validated resource revision");
    assert_eq!(surface_resource_validation_counters_v2(), (1, 3));
    execute_surface_liquid_ingress_v2(
        &configuration,
        &resource,
        &input(&configuration, transaction, 1800.0),
    )
    .expect("trusted ingress consumes validated revision");
    assert_eq!(surface_resource_validation_counters_v2(), (1, 3));
}

#[test]
fn validated_surface_ingress_avoids_owner_reserialization() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(902);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("validated resource");
    reset_surface_resource_validation_counters_v2();
    execute_surface_liquid_ingress_v2(
        &configuration,
        &resource,
        &input(&configuration, transaction, 1800.0),
    )
    .expect("validated ingress");
    assert_eq!(surface_resource_validation_counters_v2(), (0, 0));
}

#[test]
fn surface_resource_mutation_invalidates_validation_proof() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(903);
    reset_surface_resource_validation_counters_v2();
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("validated resource");
    let (requests, authorizations, uses) =
        ordinary_surface_protocol(&configuration, transaction, 0.03125);
    let revised = apply_ordinary_finalized_uses_to_phase_adjusted_v2(
        &configuration,
        &resource,
        &requests,
        &authorizations,
        &uses,
        &[],
    )
    .expect("fully validated revised resource");
    assert_eq!(surface_resource_validation_counters_v2(), (2, 6));
    execute_surface_liquid_ingress_v2(
        &configuration,
        &revised,
        &input(&configuration, transaction, 1800.0),
    )
    .expect("revised proof consumed");
    assert_eq!(surface_resource_validation_counters_v2(), (2, 6));
}

#[test]
fn surface_resource_wrong_configuration_or_nested_owner_rejects() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let transaction = TransactionId(904);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("validated resource");

    let depths = configuration
        .parent()
        .records
        .iter()
        .filter(|record| record.key.surface_class == SurfaceClass::ForestLitter)
        .map(|record| (record.key.clone(), 0.03125))
        .collect();
    let wrong_configuration = SurfaceLiquidConfigurationV2::new(
        configuration.parent().clone(),
        SurfaceLiquidOwnerModelDefinitionV2::new(digest('4'), digest('5'), digest('6'))
            .expect("wrong model"),
        &depths,
    )
    .expect("wrong configuration");
    assert!(
        execute_surface_liquid_ingress_v2(
            &wrong_configuration,
            &resource,
            &input(&configuration, transaction, 1800.0),
        )
        .is_err()
    );

    let mut nested_owner_poison = resource.clone();
    let mut records = beginning
        .v2_state()
        .expect("beginning V2")
        .records()
        .to_vec();
    records[0].surface_enthalpy_j_m2_tile += 1.0;
    let changed_owner = beginning
        .try_replace_v2_state(
            &configuration,
            records,
            beginning
                .v2_state()
                .expect("beginning V2")
                .continuations()
                .to_vec(),
        )
        .expect("changed nested owner");
    nested_owner_poison.replace_phase_adjusted_owner_for_test(changed_owner);
    assert!(
        execute_surface_liquid_ingress_v2(
            &configuration,
            &nested_owner_poison,
            &input(&configuration, transaction, 1800.0),
        )
        .is_err()
    );

    let other = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        TransactionId(905),
        &zero_closure(&beginning),
    )
    .expect("other candidate");
    let mut proof_transfer = resource.clone();
    proof_transfer.transfer_validation_proof_from_for_test(&other);
    assert!(
        execute_surface_liquid_ingress_v2(
            &configuration,
            &proof_transfer,
            &input(&configuration, transaction, 1800.0),
        )
        .is_err()
    );
}

#[test]
fn validated_surface_resource_output_and_rollback_are_unchanged() {
    let configuration = configuration_v2();
    let beginning = owner_v2(&configuration);
    let beginning_bytes = state_bytes(&configuration, &beginning);
    let transaction = TransactionId(906);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        transaction,
        &zero_closure(&beginning),
    )
    .expect("validated resource");
    let ingress = input(&configuration, transaction, 1800.0);
    let first = execute_surface_liquid_ingress_v2(&configuration, &resource, &ingress)
        .expect("first accepted output");
    resource
        .validate(&configuration)
        .expect("fresh boundary validation remains available");
    let second = execute_surface_liquid_ingress_v2(&configuration, &resource, &ingress)
        .expect("second accepted output");
    assert_eq!(first, second);
    let mut poison = resource.clone();
    poison.transfer_validation_proof_from_for_test(
        &prepare_surface_liquid_resource_candidate_v2(
            &configuration,
            &beginning,
            &beginning,
            TransactionId(907),
            &zero_closure(&beginning),
        )
        .expect("foreign proof source"),
    );
    assert!(execute_surface_liquid_ingress_v2(&configuration, &poison, &ingress).is_err());
    assert_eq!(state_bytes(&configuration, &beginning), beginning_bytes);
    assert_eq!(
        state_bytes(&configuration, resource.beginning_owner()),
        beginning_bytes
    );
}

fn zero_ingress_mass(input: &mut DirectSurfaceLiquidIngressInput) {
    for ingress in &mut input.tile_ingress {
        match ingress {
            DirectTileGroundIngress::OpenRawPrecipitation {
                raw_precipitation, ..
            } => raw_precipitation.mass_kg_m2_tile_ground = 0.0,
            DirectTileGroundIngress::CoveredCanopyRelease { release, .. } => {
                release.throughfall.mass_kg_m2_tile_ground = 0.0;
                release.initial_drainage.mass_kg_m2_tile_ground = 0.0;
                release.second_drainage.mass_kg_m2_tile_ground = 0.0;
                release.stemflow.mass_kg_m2_tile_ground = 0.0;
            }
            DirectTileGroundIngress::OpenLiquidParcels { .. }
            | DirectTileGroundIngress::CoveredCanopyReleaseAndRunon { .. } => {
                panic!("unexpected ingress fixture variant")
            }
        }
    }
}
