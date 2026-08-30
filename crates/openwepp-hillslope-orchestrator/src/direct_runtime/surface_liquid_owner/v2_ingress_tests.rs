use super::tests::configuration;
use super::v2_ingress_adapter::{
    DirectWb14ParentWorkingStateV2, execute_surface_liquid_ingress_v2,
    execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding,
    prepare_surface_liquid_resource_candidate_v2,
};
use super::*;
use crate::direct_runtime::surface_liquid_ingress::{
    DirectCanopyLiquidRelease, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectSurfaceLiquidIngressInput, DirectTileGroundIngress,
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

fn state_bytes(
    configuration: &SurfaceLiquidConfigurationV2,
    owner: &SurfaceLiquidOwnerEnvelopeV2,
) -> Vec<u8> {
    owner
        .canonical_bytes(configuration.parent(), Some(configuration))
        .expect("canonical V2 envelope")
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
    let restart = parent
        .restart_bytes(&configuration)
        .expect("V2 parent restart");
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
