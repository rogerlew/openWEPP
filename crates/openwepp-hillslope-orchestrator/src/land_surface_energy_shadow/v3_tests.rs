//! Focused frozen-litter V3 coordinator vectors.

use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{
    BeginningLitterPhaseState, FinalizedLitterVapor, LandSurfaceEnergyConfiguration,
    LandSurfaceEnergyV3State, LitterPhaseConfiguration, LitterVaporEnvironment, Sha256Digest,
    SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2, SoilThermalV2MigrationIdentity,
    V2_MODEL_DEFINITION_SHA256, V2_MODEL_VERSION, V2_VEGETATION_MODEL_DEFINITION_SHA256,
    V2_VEGETATION_MODEL_VERSION, V3PhaseFreeSurfaceEnergyLedger, evaluate_raw_litter_vapor,
    finalize_litter_vapor, install_finalized_vapor, migrate_soil_thermal_v1_to_v2,
    migrate_v2_configuration_to_v3, migrate_v2_state_to_v3, project_validated_v1_runtime_to_v2,
    saturation_specific_humidity,
};

use crate::direct_runtime::{
    DirectCanopyLiquidRelease, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectSurfaceLiquidIngressInput, DirectTileGroundIngress, DirectWb14CoupledChildBindingV1,
    SurfaceLiquidConfigurationV2, SurfaceLiquidOwnedStateV2, SurfaceLiquidOwnerEnvelopeV2,
    SurfaceLiquidOwnerModelDefinitionV2,
};

use super::endpoint_fixture;
use super::v3_execution::{FrozenLitterV3RuntimeInput, execute_frozen_litter_v3};
use super::v3_input_projection::{
    FROZEN_LITTER_V3_SUPPORT_FLOOR_NS, FrozenLitterV3PhaseFreeInput, FrozenLitterV3RuntimeError,
    project_frozen_litter_v3_phase,
};
use super::v3_rollback::FrozenLitterV3RollbackSnapshot;

const TRANSACTION: TransactionId = TransactionId(703);
const SUPPORT_START_NS: u128 = 0;
const SUPPORT_END_NS: u128 = 900_000_000_000;
const PARENT_END_NS: u128 = 1_800_000_000_000;

fn digest(byte: char) -> String {
    std::iter::repeat_n(byte, 64).collect()
}

fn typed_digest(byte: char) -> Sha256Digest {
    Sha256Digest::try_new(digest(byte)).expect("test digest")
}

struct RuntimeFixture {
    surface_configuration: SurfaceLiquidConfigurationV2,
    surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    lse_configuration: LandSurfaceEnergyConfiguration,
    lse_state: LandSurfaceEnergyV3State,
    phase_inputs: Vec<FrozenLitterV3PhaseFreeInput>,
    ingress: DirectSurfaceLiquidIngressInput,
    binding: DirectWb14CoupledChildBindingV1,
    soil_owner: SoilThermalOwnerEnvelopeV2,
    soil_restart: SoilThermalOwnerRestartV2,
}

fn lse_v3_fixture() -> (LandSurfaceEnergyConfiguration, LandSurfaceEnergyV3State) {
    let fixture = endpoint_fixture();
    let mut v2_configuration = fixture.lse_configuration.clone();
    v2_configuration.model_version = V2_MODEL_VERSION.into();
    v2_configuration.model_definition_sha256 =
        Sha256Digest::try_new(V2_MODEL_DEFINITION_SHA256).expect("V2 digest");
    v2_configuration.vegetation_configuration.model_version = V2_VEGETATION_MODEL_VERSION.into();
    v2_configuration
        .vegetation_configuration
        .model_definition_sha256 =
        Sha256Digest::try_new(V2_VEGETATION_MODEL_DEFINITION_SHA256).expect("V10 digest");
    v2_configuration.configuration_sha256 = v2_configuration
        .canonical_sha256()
        .expect("V2 config digest");
    v2_configuration.validate_v2().expect("V2 configuration");
    let v2_state = project_validated_v1_runtime_to_v2(
        &fixture.lse_configuration,
        &fixture.lse_state,
        &v2_configuration,
        &v2_configuration
            .vegetation_configuration
            .configuration_sha256,
    )
    .expect("V2 state");
    let v3_configuration =
        migrate_v2_configuration_to_v3(&v2_configuration).expect("V3 configuration");
    let mut v3_state =
        migrate_v2_state_to_v3(&v2_configuration, &v2_state, &v3_configuration).expect("V3 state");
    v3_state.0.last_accepted_transaction_id = None;
    for tile in &mut v3_state.0.tiles {
        if tile.tile_id.as_str() == "forest" {
            tile.surface_temperature_warm_start_k = 273.15;
            tile.surface_enthalpy_j_m2_tile_ground = 0.0;
        }
    }
    v3_state.0.state_sha256 = v3_state.canonical_sha256().expect("V3 state digest");
    v3_state
        .validate(&v3_configuration)
        .expect("adjusted V3 state");
    (v3_configuration, v3_state)
}

fn surface_v2_fixture(
    lse_state: &LandSurfaceEnergyV3State,
) -> (SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerEnvelopeV2) {
    let parent = endpoint_fixture().surface_configuration;
    let depths = parent
        .records
        .iter()
        .filter(|record| record.key.tile_id.as_str() == "forest")
        .map(|record| (record.key.clone(), 0.04))
        .collect();
    let model = SurfaceLiquidOwnerModelDefinitionV2::new(digest('1'), digest('2'), digest('3'))
        .expect("surface V2 model");
    let configuration =
        SurfaceLiquidConfigurationV2::new(parent, model, &depths).expect("surface V2 config");
    let liquid = configuration
        .parent()
        .records
        .iter()
        .map(|record| {
            (
                record.key.clone(),
                if record.key.tile_id.as_str() == "forest" {
                    0.75
                } else {
                    0.0
                },
            )
        })
        .collect();
    let ice = configuration
        .parent()
        .records
        .iter()
        .map(|record| {
            (
                record.key.clone(),
                if record.key.tile_id.as_str() == "forest" {
                    0.25
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
            let value = lse_state
                .0
                .tiles
                .iter()
                .find(|tile| tile.ofe_id == record.key.ofe_id && tile.tile_id == record.key.tile_id)
                .map_or(0.0, |tile| tile.surface_enthalpy_j_m2_tile_ground);
            (record.key.clone(), value)
        })
        .collect();
    let state = SurfaceLiquidOwnedStateV2::new_initial(&configuration, &liquid, &ice, &enthalpy, 0)
        .expect("surface V2 state");
    let owner =
        SurfaceLiquidOwnerEnvelopeV2::wrap_v2(&configuration, state).expect("surface V2 owner");
    (configuration, owner)
}

fn phase_input(
    configuration: &SurfaceLiquidConfigurationV2,
    owner: &SurfaceLiquidOwnerEnvelopeV2,
    accepted_temperature_k: f64,
    evaporating: bool,
) -> FrozenLitterV3PhaseFreeInput {
    let configured = configuration
        .records()
        .iter()
        .find(|record| record.litter_depth_m.is_some())
        .expect("litter configuration");
    let state = owner
        .v2_state()
        .expect("V2 state")
        .records()
        .iter()
        .find(|record| record.key == configured.key)
        .expect("litter state");
    let phase_configuration = LitterPhaseConfiguration {
        litter_depth_m: configured.litter_depth_m.expect("depth"),
        dry_heat_capacity_j_m2_k: 0.04 * 24.0 * 3370.5,
        liquid_capacity_kg_m2_tile: 6.0,
        ice_capacity_kg_m2_tile: configured
            .litter_ice_capacity_kg_m2_tile
            .expect("ice capacity"),
    };
    let beginning = BeginningLitterPhaseState {
        liquid_kg_m2_tile: state.liquid_kg_m2_tile,
        ice_kg_m2_tile: state.litter_ice_kg_m2_tile,
        sensible_energy_j_m2_tile: state.surface_enthalpy_j_m2_tile,
        temperature_k: 273.15,
    };
    let saturation =
        saturation_specific_humidity(accepted_temperature_k, 93_000.0).expect("saturation");
    let environment = LitterVaporEnvironment {
        accepted_phase_free_temperature_k: accepted_temperature_k,
        air_density_kg_m3: 1.1,
        air_pressure_pa: 93_000.0,
        recipient_specific_humidity_kg_kg: if evaporating {
            saturation * 0.999
        } else {
            saturation
        },
        litter_to_canopy_resistance_s_m: 80.0,
    };
    let raw =
        evaluate_raw_litter_vapor(phase_configuration, beginning, environment).expect("raw vapor");
    let interval_s = 900.0;
    let finalized = FinalizedLitterVapor {
        liquid_signed_rate_kg_m2_s: raw
            .raw_liquid_signed_rate_kg_m2_s
            .max(0.0)
            .min(beginning.liquid_kg_m2_tile / interval_s),
        ice_signed_rate_kg_m2_s: raw
            .raw_ice_signed_rate_kg_m2_s
            .max(0.0)
            .min(beginning.ice_kg_m2_tile / interval_s),
    };
    let vapor = finalize_litter_vapor(
        raw,
        finalized,
        beginning,
        accepted_temperature_k,
        interval_s,
    )
    .expect("final vapor");
    let post_vapor = install_finalized_vapor(
        phase_configuration,
        beginning,
        accepted_temperature_k,
        vapor,
    )
    .expect("post vapor");
    let storage =
        (post_vapor.sensible_energy_j_m2_tile - beginning.sensible_energy_j_m2_tile) / interval_s;
    let liquid_vapor = vapor.liquid_signed_energy_j_m2 / interval_s;
    let ice_vapor = vapor.ice_signed_energy_j_m2 / interval_s;
    let surface_energy = V3PhaseFreeSurfaceEnergyLedger {
        beginning_sensible_energy_j_m2: beginning.sensible_energy_j_m2_tile,
        ending_sensible_energy_j_m2: post_vapor.sensible_energy_j_m2_tile,
        absorbed_shortwave_w_m2: storage + liquid_vapor + ice_vapor,
        net_longwave_w_m2: 0.0,
        sensible_to_canopy_air_w_m2: 0.0,
        liquid_vapor_energy_w_m2: liquid_vapor,
        ice_vapor_energy_w_m2: ice_vapor,
        ground_heat_w_m2: 0.0,
        storage_w_m2: storage,
        reconstructed_energy_residual_w_m2: 0.0,
    };
    FrozenLitterV3PhaseFreeInput::from_authority_operands_for_test(
        configured.key.ofe_id.clone(),
        configured.key.tile_id.clone(),
        phase_configuration,
        beginning,
        vapor,
        post_vapor,
        surface_energy,
    )
}

fn amount(mass: f64) -> DirectIngressAmount {
    DirectIngressAmount {
        mass_kg_m2_tile_ground: mass,
        temperature_k: 273.15,
        specific_liquid_enthalpy_j_kg: 0.0,
        start_s: 0.0,
        end_s: 900.0,
    }
}

fn ingress(
    configuration: &SurfaceLiquidConfigurationV2,
    litter_mass: f64,
) -> DirectSurfaceLiquidIngressInput {
    let tile_ingress = configuration
        .parent()
        .records
        .iter()
        .map(|record| {
            if record.key.tile_id.as_str() == "forest" {
                DirectTileGroundIngress::CoveredCanopyRelease {
                    ofe_id: record.key.ofe_id.clone(),
                    tile_id: record.key.tile_id.clone(),
                    surface_id: record.key.surface_id.clone(),
                    release: DirectCanopyLiquidRelease {
                        throughfall: amount(litter_mass),
                        initial_drainage: amount(0.0),
                        second_drainage: amount(0.0),
                        stemflow: amount(0.0),
                    },
                }
            } else {
                DirectTileGroundIngress::OpenRawPrecipitation {
                    ofe_id: record.key.ofe_id.clone(),
                    tile_id: record.key.tile_id.clone(),
                    surface_id: record.key.surface_id.clone(),
                    raw_precipitation: amount(0.0),
                }
            }
        })
        .collect();
    DirectSurfaceLiquidIngressInput {
        transaction_id: TRANSACTION,
        day_index: 0,
        interval_index: 0,
        interval_s: 900.0,
        tile_ingress,
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: configuration.parent().ofe_topology[0].clone(),
            effective_conductivity_m_s: 1.0e-12,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.2,
        }],
    }
}

fn soil_fixture() -> (SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2) {
    let thermal = endpoint_fixture().thermal;
    let owner = migrate_soil_thermal_v1_to_v2(
        &thermal,
        SoilThermalV2MigrationIdentity {
            model_version: "OPENWEPP_SOIL_THERMAL_TEST_V2".into(),
            model_definition_sha256: typed_digest('8'),
            run_id: "83".into(),
            transaction_id: TRANSACTION,
            support_start_ns: SUPPORT_START_NS,
            support_end_ns: SUPPORT_END_NS,
            receipt_chain_sha256: typed_digest('9'),
        },
    )
    .expect("soil V2 owner");
    let restart = SoilThermalOwnerRestartV2 {
        owner_tag: owner.owner_tag.clone(),
        schema_sha256: owner.schema_sha256.clone(),
        exact_carry_definition_sha256: owner.exact_carry_definition_sha256.clone(),
        parent_v1_state_sha256: owner.parent_v1_state_sha256.clone(),
        owner_state_sha256: owner.state.state_sha256.clone(),
        last_accepted_transaction_id: owner.state.last_accepted_transaction_id,
        receipt_chain_sha256: owner.receipt_chain_sha256.clone(),
        restart_sha256: typed_digest('a'),
    };
    (owner, restart)
}

fn runtime_fixture(temperature_k: f64, evaporating: bool, litter_ingress: f64) -> RuntimeFixture {
    let (lse_configuration, lse_state) = lse_v3_fixture();
    let (surface_configuration, surface_owner) = surface_v2_fixture(&lse_state);
    let phase_inputs = vec![phase_input(
        &surface_configuration,
        &surface_owner,
        temperature_k,
        evaporating,
    )];
    let ingress = ingress(&surface_configuration, litter_ingress);
    let binding = DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: 900.0_f64.to_bits(),
        coupled_parent_transaction_sha256: [1; 32],
        accepted_slab_sha256: [2; 32],
        parent_beginning_complete_owner_set_sha256: [3; 32],
        parent_support_start_ns: SUPPORT_START_NS,
        parent_support_end_ns: PARENT_END_NS,
        child_support_start_ns: SUPPORT_START_NS,
        child_support_end_ns: SUPPORT_END_NS,
    };
    let (soil_owner, soil_restart) = soil_fixture();
    RuntimeFixture {
        surface_configuration,
        surface_owner,
        lse_configuration,
        lse_state,
        phase_inputs,
        ingress,
        binding,
        soil_owner,
        soil_restart,
    }
}

fn execute_fixture(
    fixture: &RuntimeFixture,
) -> Result<super::v3_execution::AcceptedFrozenLitterV3RuntimeCandidate, FrozenLitterV3RuntimeError>
{
    execute_frozen_litter_v3(&FrozenLitterV3RuntimeInput {
        transaction_id: TRANSACTION,
        predecessor_transaction_id: None,
        parent_support_start_ns: SUPPORT_START_NS,
        parent_support_end_ns: PARENT_END_NS,
        support_start_ns: SUPPORT_START_NS,
        support_end_ns: SUPPORT_END_NS,
        predecessor_receipt_chain_sha256: digest('b'),
        surface_configuration: &fixture.surface_configuration,
        beginning_surface_owner: &fixture.surface_owner,
        lse_configuration: &fixture.lse_configuration,
        beginning_lse_state: &fixture.lse_state,
        phase_inputs: &fixture.phase_inputs,
        current_ingress: &fixture.ingress,
        wb14_parent: None,
        coupled_binding: fixture.binding,
        soil_thermal_owner: &fixture.soil_owner,
        soil_thermal_restart: &fixture.soil_restart,
    })
}

#[test]
fn frozen_and_thawing_phase_precede_current_ingress_and_wb14() {
    let frozen = runtime_fixture(272.5, false, 0.5);
    let accepted = execute_fixture(&frozen).expect("frozen candidate");
    let beginning = &frozen
        .surface_owner
        .v2_state()
        .expect("beginning V2")
        .records()[0];
    let adjusted = &accepted
        .phase_adjusted_surface_owner
        .v2_state()
        .expect("adjusted V2")
        .records()[0];
    assert!(adjusted.litter_ice_kg_m2_tile > beginning.litter_ice_kg_m2_tile);
    assert!(adjusted.liquid_kg_m2_tile < beginning.liquid_kg_m2_tile);
    assert_eq!(
        accepted.litter_phase_receipts[0].same_support_resolve_count,
        0
    );

    let thawing = runtime_fixture(274.0, false, 0.5);
    let accepted = execute_fixture(&thawing).expect("thawing candidate");
    let beginning = &thawing
        .surface_owner
        .v2_state()
        .expect("beginning V2")
        .records()[0];
    let adjusted = &accepted
        .phase_adjusted_surface_owner
        .v2_state()
        .expect("adjusted V2")
        .records()[0];
    assert!(adjusted.litter_ice_kg_m2_tile < beginning.litter_ice_kg_m2_tile);
    assert!(adjusted.liquid_kg_m2_tile > beginning.liquid_kg_m2_tile);
}

#[test]
fn phase_specific_vapor_has_no_double_debit_and_fusion_closes() {
    let fixture = runtime_fixture(272.5, true, 0.0);
    let projected = project_frozen_litter_v3_phase(
        &fixture.surface_configuration,
        &fixture.surface_owner,
        &fixture.lse_configuration,
        &fixture.lse_state,
        TRANSACTION,
        SUPPORT_START_NS,
        SUPPORT_END_NS,
        &fixture.phase_inputs,
    )
    .expect("phase projection");
    let vapor = fixture.phase_inputs[0].accepted_vapor();
    let row = &projected.closure[0];
    assert!(vapor.liquid_signed_mass_kg_m2 > 0.0);
    assert!(vapor.ice_signed_mass_kg_m2 > 0.0);
    assert!(row.liquid_debit_kg_m2_tile >= vapor.liquid_signed_mass_kg_m2);
    assert!(row.ice_debit_kg_m2_tile >= vapor.ice_signed_mass_kg_m2);
    let accepted = execute_fixture(&fixture).expect("accepted vapor/phase");
    let receipt = &accepted.litter_phase_receipts[0];
    assert!(receipt.closure.total_phase_mass_residual_kg_m2.abs() < 1.0e-12);
    assert!(receipt.closure.fusion_energy_residual_j_m2.abs() < 1.0e-9);
    assert_eq!(
        receipt.transfer.fusion_energy_j_m2.to_bits(),
        (333_700.0 * (receipt.transfer.freeze_kg_m2 - receipt.transfer.melt_kg_m2)).to_bits()
    );
}

#[test]
fn wrong_constant_order_and_identity_poisons_fail_closed() {
    let fixture = runtime_fixture(272.5, false, 0.0);
    let mut wrong_constant = fixture.phase_inputs.clone();
    wrong_constant[0].configuration.dry_heat_capacity_j_m2_k += 1.0;
    assert!(
        project_frozen_litter_v3_phase(
            &fixture.surface_configuration,
            &fixture.surface_owner,
            &fixture.lse_configuration,
            &fixture.lse_state,
            TRANSACTION,
            SUPPORT_START_NS,
            SUPPORT_END_NS,
            &wrong_constant,
        )
        .is_err()
    );

    let mut wrong_order = fixture.phase_inputs.clone();
    wrong_order[0].tile_id = openwepp_kernel_contract::TileId::try_new("open").expect("tile");
    assert!(
        project_frozen_litter_v3_phase(
            &fixture.surface_configuration,
            &fixture.surface_owner,
            &fixture.lse_configuration,
            &fixture.lse_state,
            TRANSACTION,
            SUPPORT_START_NS,
            SUPPORT_END_NS,
            &wrong_order,
        )
        .is_err()
    );

    let mut wrong_identity = runtime_fixture(272.5, false, 0.0);
    wrong_identity.ingress.transaction_id = TransactionId(704);
    assert!(execute_fixture(&wrong_identity).is_err());
}

#[test]
fn wrong_vapor_sign_is_rejected_and_beginning_bytes_roll_back_exactly() {
    let mut fixture = runtime_fixture(272.5, true, 0.0);
    let snapshot = FrozenLitterV3RollbackSnapshot::capture(
        &fixture.surface_configuration,
        &fixture.surface_owner,
        &fixture.lse_state,
        &fixture.soil_owner,
        &fixture.soil_restart,
        None,
    )
    .expect("rollback snapshot");
    let input = &fixture.phase_inputs[0];
    let mut vapor = input.accepted_vapor();
    vapor.finalized.liquid_signed_rate_kg_m2_s = -vapor.finalized.liquid_signed_rate_kg_m2_s;
    fixture.phase_inputs[0] = FrozenLitterV3PhaseFreeInput::from_authority_operands_for_test(
        input.ofe_id.clone(),
        input.tile_id.clone(),
        input.configuration,
        input.beginning,
        vapor,
        input.accepted_post_vapor(),
        input.accepted_surface_energy(),
    );
    assert!(execute_fixture(&fixture).is_err());
    snapshot
        .require_exactly_unchanged(
            &fixture.surface_configuration,
            &fixture.surface_owner,
            &fixture.lse_state,
            &fixture.soil_owner,
            &fixture.soil_restart,
            None,
        )
        .expect("byte-exact rollback");
}

#[test]
fn complete_projection_joins_surface_owner_soil_v2_and_canonical_replay() {
    let fixture = runtime_fixture(273.15, false, 0.0);
    let accepted = execute_fixture(&fixture).expect("complete V3 candidate");
    let bytes = accepted
        .complete_owner_projection
        .canonical_bytes(&fixture.surface_configuration)
        .expect("projection bytes");
    let replay = crate::SurfaceLiquidCompleteOwnerProjectionV3::from_canonical_bytes(
        &fixture.surface_configuration,
        &bytes,
    )
    .expect("projection replay");
    assert_eq!(replay, accepted.complete_owner_projection);
    assert_eq!(
        accepted.ending_lse_state.0.last_accepted_transaction_id,
        Some(TRANSACTION)
    );
    assert_eq!(
        accepted.ingress.ending_owner(),
        &accepted.ending_surface_owner
    );
    let phase_ice = accepted
        .phase_adjusted_surface_owner
        .v2_state()
        .expect("phase owner")
        .records()[0]
        .litter_ice_kg_m2_tile;
    let ending_ice = accepted
        .ending_surface_owner
        .v2_state()
        .expect("ending owner")
        .records()[0]
        .litter_ice_kg_m2_tile;
    assert_eq!(phase_ice.to_bits(), ending_ice.to_bits());
    assert_eq!(
        accepted.rollback,
        FrozenLitterV3RollbackSnapshot::capture(
            &fixture.surface_configuration,
            &fixture.surface_owner,
            &fixture.lse_state,
            &fixture.soil_owner,
            &fixture.soil_restart,
            None,
        )
        .expect("beginning rollback")
    );
}

#[test]
fn exact_floor_and_off_grid_support_are_rejected_without_physics() {
    assert_eq!(FROZEN_LITTER_V3_SUPPORT_FLOOR_NS, 60_000_000_000);
    let fixture = runtime_fixture(273.15, false, 0.0);
    assert!(matches!(
        project_frozen_litter_v3_phase(
            &fixture.surface_configuration,
            &fixture.surface_owner,
            &fixture.lse_configuration,
            &fixture.lse_state,
            TRANSACTION,
            0,
            59_000_000_000,
            &fixture.phase_inputs,
        ),
        Err(FrozenLitterV3RuntimeError::Chronology(_))
    ));
    assert!(
        project_frozen_litter_v3_phase(
            &fixture.surface_configuration,
            &fixture.surface_owner,
            &fixture.lse_configuration,
            &fixture.lse_state,
            TRANSACTION,
            0,
            61_000_000_000,
            &fixture.phase_inputs,
        )
        .is_err()
    );
}

#[test]
fn direct_runtime_exposes_only_the_crate_private_v2_ingress_handoff() {
    let source = include_str!("../direct_runtime.rs");
    assert!(source.contains("pub(crate) use surface_liquid_owner::v2_ingress_adapter"));
    assert!(source.contains("DirectWb14ParentWorkingStateV2"));
    assert!(!source.contains("pub use surface_liquid_owner::v2_ingress_adapter"));
}
