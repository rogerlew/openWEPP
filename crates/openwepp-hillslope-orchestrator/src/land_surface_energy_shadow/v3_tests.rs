//! Focused frozen-litter V3 coordinator vectors.

use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::{
    BeginningLitterPhaseState, ExactDyadicEnthalpy, FinalizedLitterVapor,
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyV3State, LitterPhaseConfiguration,
    LitterVaporEnvironment, OfeId, Sha256Digest, SoilThermalOwnerEnvelopeV2,
    SoilThermalOwnerRestartV2, SoilThermalV2MigrationIdentity, SurfaceId,
    V2_MODEL_DEFINITION_SHA256, V2_MODEL_VERSION, V2_VEGETATION_MODEL_DEFINITION_SHA256,
    V2_VEGETATION_MODEL_VERSION, V3PhaseFreeSurfaceEnergyLedger, evaluate_raw_litter_vapor,
    finalize_litter_vapor, install_finalized_vapor, migrate_soil_thermal_v1_to_v2,
    migrate_v2_configuration_to_v3, migrate_v2_state_to_v3, project_validated_v1_runtime_to_v2,
    saturation_specific_humidity,
};

use crate::direct_runtime::{
    DirectCanopyLiquidRelease, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectSurfaceLiquidIngressInput, DirectSurfaceLiquidReceiptDisposition,
    DirectTileGroundIngress, DirectWb14CoupledChildBindingV1,
    LseSurfaceEnthalpyAcceptedEnergyOperandV1, LseSurfaceEnthalpyEnergyOperandKindV1,
    LseSurfaceEnthalpyOwnerEnvelopeV1, SurfaceLiquidConfigurationV2, SurfaceLiquidOwnedStateV2,
    SurfaceLiquidOwnerEnvelopeV2, SurfaceLiquidOwnerModelDefinitionV2,
};

use super::endpoint_fixture;
use super::v3_execution::{
    FrozenLitterV3RuntimeInput, FrozenLitterV4RuntimeInput, execute_frozen_litter_v3,
    execute_frozen_litter_v4,
};
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
        finalize_wb14_parent_interval: false,
        coupled_binding: fixture.binding,
        soil_thermal_owner: &fixture.soil_owner,
        soil_thermal_restart: &fixture.soil_restart,
    })
}

fn physical_input(fixture: &RuntimeFixture) -> FrozenLitterV3RuntimeInput<'_> {
    FrozenLitterV3RuntimeInput {
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
        finalize_wb14_parent_interval: false,
        coupled_binding: fixture.binding,
        soil_thermal_owner: &fixture.soil_owner,
        soil_thermal_restart: &fixture.soil_restart,
    }
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
fn exact_surface_successor_joins_v3_mirrors_receipt_and_projection_v4() {
    let fixture = runtime_fixture(273.15, false, 0.0);
    let beginning_exact = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
        &fixture.lse_configuration,
        &fixture.lse_state,
        &fixture.surface_configuration,
        &fixture.surface_owner,
    )
    .expect("exact owner adoption");
    let accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: physical_input(&fixture),
        beginning_exact_surface_owner: &beginning_exact,
    })
    .expect("accepted exact-surface candidate");
    accepted
        .exact_surface_receipt
        .validate(&beginning_exact, &accepted.ending_exact_surface_owner)
        .expect("exact receipt replay");
    let replay = crate::SurfaceLiquidCompleteOwnerProjectionV4::from_canonical_bytes(
        &fixture.surface_configuration,
        &accepted
            .complete_owner_projection
            .canonical_bytes(&fixture.surface_configuration)
            .expect("projection V4 bytes"),
        fixture.lse_state.0.state_sha256.as_str(),
    )
    .expect("projection V4 replay");
    assert_eq!(replay, accepted.complete_owner_projection);
    for exact in accepted.ending_exact_surface_owner.records() {
        let surface = accepted
            .physical
            .ending_surface_owner
            .v2_state()
            .expect("surface V2")
            .records()
            .iter()
            .find(|record| record.key == exact.surface_key)
            .expect("surface high mirror");
        let lse = accepted
            .physical
            .ending_lse_state
            .0
            .tiles
            .iter()
            .find(|tile| {
                tile.ofe_id == exact.surface_key.ofe_id && tile.tile_id == exact.surface_key.tile_id
            })
            .expect("LSE high mirror");
        assert_eq!(
            exact.enthalpy_hi_j_m2_tile.to_bits(),
            surface.surface_enthalpy_j_m2_tile.to_bits()
        );
        assert_eq!(
            exact.enthalpy_hi_j_m2_tile.to_bits(),
            lse.surface_enthalpy_j_m2_tile_ground.to_bits()
        );
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn exact_surface_operands_refuse_reorder_omission_owner_and_source_substitution() {
    let fixture = runtime_fixture(272.5, false, 0.0);
    let beginning = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
        &fixture.lse_configuration,
        &fixture.lse_state,
        &fixture.surface_configuration,
        &fixture.surface_owner,
    )
    .expect("exact owner adoption");
    let accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: physical_input(&fixture),
        beginning_exact_surface_owner: &beginning,
    })
    .expect("accepted exact-surface candidate");
    let original = accepted.exact_surface_receipt.accepted_operands.clone();
    let beginning_bytes = beginning.canonical_bytes().expect("beginning exact bytes");
    let surface_before = fixture.surface_owner.clone();
    let lse_before = fixture.lse_state.clone();
    let attempt = |operands| {
        beginning.advance_exact(
            &accepted.physical.ending_lse_state,
            &fixture.surface_configuration,
            &accepted.physical.ending_surface_owner,
            TRANSACTION,
            None,
            SUPPORT_START_NS,
            SUPPORT_END_NS,
            &original,
            operands,
        )
    };
    let assert_operand_poison = |label: &str, operands| {
        assert!(
            attempt(operands).is_err(),
            "accepted operand poison: {label}"
        );
        assert_eq!(
            beginning.canonical_bytes().expect("rollback exact bytes"),
            beginning_bytes,
            "exact owner changed after {label}",
        );
        assert_eq!(
            fixture.surface_owner, surface_before,
            "surface owner changed after {label}"
        );
        assert_eq!(
            fixture.lse_state, lse_before,
            "LSE owner changed after {label}"
        );
    };

    let mut reordered = original.clone();
    reordered.swap(0, 1);
    assert_operand_poison("reorder", reordered);

    let mut omitted = original.clone();
    let phase_index = omitted
        .iter()
        .position(|operand| {
            operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::PhaseFreeSurfaceEnergy
        })
        .expect("phase operand");
    omitted.remove(phase_index);
    assert_operand_poison("omission", omitted);

    let mut duplicated = original.clone();
    duplicated.insert(0, duplicated[0].clone());
    assert_operand_poison("duplication", duplicated);

    let mut kind_poison = original.clone();
    kind_poison[0].kind = LseSurfaceEnthalpyEnergyOperandKindV1::LitterFusionEnergy;
    assert_operand_poison("kind", kind_poison);

    let mut ordinal_poison = original.clone();
    ordinal_poison[0].ordinal = 17;
    assert_operand_poison("ordinal", ordinal_poison);

    let mut owner_poison = original.clone();
    owner_poison[0].source_owner_id =
        ResourceOwnerId::try_new("foreign-surface-owner").expect("foreign owner");
    assert_operand_poison("source owner", owner_poison);

    let mut receipt_poison = original.clone();
    let litter_key = receipt_poison
        .iter()
        .find(|operand| {
            operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::PhaseFreeSurfaceEnergy
        })
        .expect("litter operand")
        .surface_key
        .clone();
    for operand in receipt_poison.iter_mut().filter(|operand| {
        operand.surface_key == litter_key
            && operand.kind != LseSurfaceEnthalpyEnergyOperandKindV1::RetainedIngressTileCredit
    }) {
        operand.source_receipt_sha256 = typed_digest('e');
    }
    assert_operand_poison("source receipt", receipt_poison);

    let mut ofe_poison = original.clone();
    ofe_poison[0].surface_key.ofe_id = OfeId::try_new("foreign-ofe").expect("foreign OFE");
    assert_operand_poison("OFE", ofe_poison);

    let mut tile_poison = original.clone();
    tile_poison[0].surface_key.tile_id =
        openwepp_kernel_contract::TileId::try_new("foreign-tile").expect("foreign tile");
    assert_operand_poison("tile", tile_poison);

    let mut surface_poison = original.clone();
    surface_poison[0].surface_key.surface_id =
        SurfaceId::try_new("foreign-surface").expect("foreign surface");
    assert_operand_poison("surface", surface_poison);

    let mut transaction_poison = original.clone();
    transaction_poison[0].transaction_id = TransactionId(TRANSACTION.0 + 1);
    assert_operand_poison("transaction", transaction_poison);

    let mut predecessor_poison = original.clone();
    predecessor_poison[0].predecessor_transaction_id = Some(TransactionId(1));
    assert_operand_poison("predecessor", predecessor_poison);

    let mut support_start_poison = original.clone();
    support_start_poison[0].support_start_ns += 1;
    assert_operand_poison("support start", support_start_poison);

    let mut support_end_poison = original.clone();
    support_end_poison[0].support_end_ns -= 1;
    assert_operand_poison("support end", support_end_poison);

    let mut units_poison = original.clone();
    units_poison[0].units = "producer residual".to_owned();
    assert_operand_poison("units/producer residual", units_poison);

    let mut basis_poison = original.clone();
    basis_poison[0].basis = "OFE_ground".to_owned();
    assert_operand_poison("basis/tolerance repair", basis_poison);

    let nonzero_index = original
        .iter()
        .position(|operand| operand.energy_j_m2_tile_ground != 0.0)
        .expect("nonzero accepted phase/fusion operand");
    let nonzero = original[nonzero_index].energy_j_m2_tile_ground;
    for (label, replacement) in [
        ("discarded credit", 0.0),
        ("forced ULP", f64::from_bits(nonzero.to_bits() ^ 1)),
        ("zero snap", -0.0_f64),
    ] {
        let mut amount_poison = original.clone();
        amount_poison[nonzero_index].energy_j_m2_tile_ground = replacement;
        assert_operand_poison(label, amount_poison);
    }
    assert_eq!(
        beginning,
        LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
            ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
            &fixture.lse_configuration,
            &fixture.lse_state,
            &fixture.surface_configuration,
            &fixture.surface_owner,
        )
        .expect("rollback replay"),
        "failed trials must leave the exact beginning bit-identical",
    );
}

#[test]
fn exact_surface_refuses_retained_credit_omission_before_candidate_seal() {
    let fixture = runtime_fixture(273.15, false, 0.01);
    let beginning = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
        &fixture.lse_configuration,
        &fixture.lse_state,
        &fixture.surface_configuration,
        &fixture.surface_owner,
    )
    .expect("exact owner adoption");
    let accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: physical_input(&fixture),
        beginning_exact_surface_owner: &beginning,
    })
    .expect("accepted retained-credit candidate");
    let mut expected = accepted.exact_surface_receipt.accepted_operands.clone();
    let mut retained = expected
        .iter()
        .find(|operand| {
            operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::PhaseFreeSurfaceEnergy
        })
        .expect("litter phase operand")
        .clone();
    retained.kind = LseSurfaceEnthalpyEnergyOperandKindV1::RetainedIngressTileCredit;
    retained.ordinal = 0;
    retained.energy_j_m2_tile_ground = 0.0;
    expected.push(retained);
    expected.sort_by(|left, right| {
        (&left.surface_key, left.kind, left.ordinal).cmp(&(
            &right.surface_key,
            right.kind,
            right.ordinal,
        ))
    });
    let mut omitted = expected.clone();
    omitted.remove(
        omitted
            .iter()
            .position(|operand| {
                operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::RetainedIngressTileCredit
            })
            .expect("retained operand"),
    );
    assert!(
        beginning
            .advance_exact(
                &accepted.physical.ending_lse_state,
                &fixture.surface_configuration,
                &accepted.physical.ending_surface_owner,
                TRANSACTION,
                None,
                SUPPORT_START_NS,
                SUPPORT_END_NS,
                &expected,
                omitted,
            )
            .is_err()
    );
    assert_eq!(beginning.receipt_chain_sha256.as_str(), digest('0'));
}

#[test]
#[allow(clippy::too_many_lines)]
fn exact_surface_groups_multiple_parcels_and_distinct_tile_credits_with_fusion() {
    let mut fixture = runtime_fixture(272.5, false, 0.01);
    let energetic_amount = |mass, temperature_k| {
        let mut value = amount(mass);
        value.temperature_k = temperature_k;
        value.specific_liquid_enthalpy_j_kg =
            openwepp_land_surface_energy::liquid_enthalpy_j_kg(temperature_k);
        value
    };
    for tile in &mut fixture.ingress.tile_ingress {
        match tile {
            DirectTileGroundIngress::CoveredCanopyRelease { release, .. } => {
                release.throughfall = energetic_amount(0.01, 274.0);
                release.initial_drainage = energetic_amount(0.02, 275.0);
                release.second_drainage = energetic_amount(0.03, 276.0);
                release.stemflow = energetic_amount(0.04, 277.0);
            }
            DirectTileGroundIngress::OpenRawPrecipitation {
                raw_precipitation, ..
            } => *raw_precipitation = energetic_amount(0.05, 278.0),
            DirectTileGroundIngress::OpenLiquidParcels { .. }
            | DirectTileGroundIngress::CoveredCanopyReleaseAndRunon { .. } => {
                panic!("fixture ingress posture")
            }
        }
    }
    let beginning = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
        &fixture.lse_configuration,
        &fixture.lse_state,
        &fixture.surface_configuration,
        &fixture.surface_owner,
    )
    .expect("exact owner adoption");
    let accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: physical_input(&fixture),
        beginning_exact_surface_owner: &beginning,
    })
    .expect("accepted multi-parcel/multitile exact candidate");

    let retained_receipts = accepted
        .physical
        .ingress
        .inner()
        .receipts()
        .iter()
        .filter(|receipt| {
            receipt.disposition == DirectSurfaceLiquidReceiptDisposition::RetainedSurface
        })
        .collect::<Vec<_>>();
    let forest_receipt_count = retained_receipts
        .iter()
        .filter(|receipt| receipt.recipient_store_key.tile_id.as_str() == "forest")
        .count();
    assert!(
        forest_receipt_count >= 2,
        "multiple same-tile parcels must survive as receipts"
    );

    let replayed_credits = super::retained_surface_tile_credits_from_receipts_v1(
        &fixture.surface_configuration,
        TRANSACTION,
        accepted.physical.ingress.inner().receipts(),
    )
    .expect("independent retained-credit replay");
    assert_eq!(
        replayed_credits.len(),
        2,
        "both distinct tiles retain a credit"
    );
    assert_ne!(
        replayed_credits[0].tile_fraction.to_bits(),
        replayed_credits[1].tile_fraction.to_bits(),
        "the two exact credits exercise distinct OFE-to-tile fractions",
    );
    assert_ne!(
        replayed_credits[0].energy_j_m2_tile_ground.to_bits(),
        replayed_credits[1].energy_j_m2_tile_ground.to_bits(),
        "the two exact credits must be numerically distinct",
    );
    let retained_operands = accepted
        .exact_surface_receipt
        .accepted_operands
        .iter()
        .filter(|operand| {
            operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::RetainedIngressTileCredit
        })
        .collect::<Vec<_>>();
    assert_eq!(retained_operands.len(), replayed_credits.len());
    for (operand, replayed) in retained_operands.iter().zip(&replayed_credits) {
        assert_eq!(operand.surface_key, replayed.store_key);
        assert_eq!(operand.ordinal, replayed.ordinal);
        assert_eq!(
            operand.source_receipt_sha256,
            replayed.source_receipt_sha256
        );
        assert_eq!(
            operand.energy_j_m2_tile_ground.to_bits(),
            replayed.energy_j_m2_tile_ground.to_bits(),
        );
    }
    assert!(
        accepted
            .exact_surface_receipt
            .accepted_operands
            .iter()
            .any(|operand| {
                operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::LitterFusionEnergy
                    && operand.energy_j_m2_tile_ground != 0.0
            })
    );
    accepted
        .exact_surface_receipt
        .validate_independent(
            &beginning,
            &accepted.ending_exact_surface_owner,
            &accepted.exact_surface_receipt.accepted_operands,
        )
        .expect("multi-parcel/multitile/fusion exact replay");
}

#[test]
#[allow(clippy::too_many_lines)]
fn exact_surface_authentic_retained_sub_ulp_credits_and_poisons_are_fail_closed() {
    let freezing = 273.15_f64;
    let warm_temperature_k = 274.0_f64;
    let symmetric_cold_bits = (2.0 * freezing - warm_temperature_k).to_bits();
    for expect_positive in [true, false] {
        let cold_temperature_k = (-128_i64..=128)
            .map(|offset| {
                if offset.is_negative() {
                    f64::from_bits(symmetric_cold_bits - offset.unsigned_abs())
                } else {
                    f64::from_bits(symmetric_cold_bits + offset.unsigned_abs())
                }
            })
            .filter(|candidate| {
                let sum = openwepp_land_surface_energy::liquid_enthalpy_j_kg(warm_temperature_k)
                    + openwepp_land_surface_energy::liquid_enthalpy_j_kg(*candidate);
                sum != 0.0 && sum.is_sign_positive() == expect_positive
            })
            .min_by(|left, right| {
                let left_sum =
                    (openwepp_land_surface_energy::liquid_enthalpy_j_kg(warm_temperature_k)
                        + openwepp_land_surface_energy::liquid_enthalpy_j_kg(*left))
                    .abs();
                let right_sum =
                    (openwepp_land_surface_energy::liquid_enthalpy_j_kg(warm_temperature_k)
                        + openwepp_land_surface_energy::liquid_enthalpy_j_kg(*right))
                    .abs();
                left_sum.total_cmp(&right_sum)
            })
            .expect("near-cancelling retained temperature pair");
        let mut fixture = runtime_fixture(315.0, false, 0.0);
        for tile in &mut fixture.ingress.tile_ingress {
            if let DirectTileGroundIngress::CoveredCanopyRelease { release, .. } = tile {
                release.throughfall = DirectIngressAmount {
                    mass_kg_m2_tile_ground: 0.05,
                    temperature_k: warm_temperature_k,
                    specific_liquid_enthalpy_j_kg:
                        openwepp_land_surface_energy::liquid_enthalpy_j_kg(warm_temperature_k),
                    start_s: 0.0,
                    end_s: 900.0,
                };
                release.initial_drainage = DirectIngressAmount {
                    mass_kg_m2_tile_ground: 0.05,
                    temperature_k: cold_temperature_k,
                    specific_liquid_enthalpy_j_kg:
                        openwepp_land_surface_energy::liquid_enthalpy_j_kg(cold_temperature_k),
                    start_s: 0.0,
                    end_s: 900.0,
                };
            }
        }
        let beginning = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
            ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
            &fixture.lse_configuration,
            &fixture.lse_state,
            &fixture.surface_configuration,
            &fixture.surface_owner,
        )
        .expect("authentic retained sub-ULP adoption");
        let beginning_bytes = beginning.canonical_bytes().expect("beginning exact bytes");
        let accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
            physical: physical_input(&fixture),
            beginning_exact_surface_owner: &beginning,
        })
        .expect("authentic retained sub-ULP V4 support");
        let expected = accepted.exact_surface_receipt.accepted_operands.clone();
        let retained_index = expected
            .iter()
            .position(|operand| {
                operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::RetainedIngressTileCredit
                    && operand.surface_key.tile_id.as_str() == "forest"
            })
            .expect("authentic retained forest operand");
        let retained = &expected[retained_index];
        assert_ne!(retained.energy_j_m2_tile_ground, 0.0);
        assert_eq!(
            retained.energy_j_m2_tile_ground.is_sign_positive(),
            expect_positive
        );
        let beginning_high = beginning
            .records()
            .iter()
            .find(|record| record.surface_key == retained.surface_key)
            .expect("beginning retained exact record")
            .enthalpy_hi_j_m2_tile;
        let without_retained = expected
            .iter()
            .enumerate()
            .filter(|(index, operand)| {
                *index != retained_index && operand.surface_key == retained.surface_key
            })
            .map(|(_, operand)| operand.energy_j_m2_tile_ground)
            .collect::<Vec<_>>();
        let (high_without_retained, carry_without_retained) =
            ExactDyadicEnthalpy::exact_sum_binary64(
                beginning_high,
                &ExactDyadicEnthalpy::zero(),
                &without_retained,
            )
            .expect("exact total without retained credit")
            .rounded_high_and_remainder()
            .expect("rounded total without retained credit");
        let ending = accepted
            .ending_exact_surface_owner
            .records()
            .iter()
            .find(|record| record.surface_key == retained.surface_key)
            .expect("ending retained exact record");
        assert_eq!(
            ending.enthalpy_hi_j_m2_tile.to_bits(),
            high_without_retained.to_bits(),
            "authentic retained credit must be sub-ULP of the accepted high term",
        );
        assert_ne!(ending.enthalpy_carry, carry_without_retained);

        let assert_poison = |accepted_operands: Vec<LseSurfaceEnthalpyAcceptedEnergyOperandV1>| {
            assert!(
                beginning
                    .advance_exact(
                        &accepted.physical.ending_lse_state,
                        &fixture.surface_configuration,
                        &accepted.physical.ending_surface_owner,
                        TRANSACTION,
                        None,
                        SUPPORT_START_NS,
                        SUPPORT_END_NS,
                        &expected,
                        accepted_operands,
                    )
                    .is_err(),
                "authentic retained operand poison must fail closed",
            );
        };
        let mut omitted = expected.clone();
        omitted.remove(retained_index);
        assert_poison(omitted);
        let mut source_substitution = expected.clone();
        source_substitution[retained_index].source_receipt_sha256 = typed_digest('f');
        assert_poison(source_substitution);
        let mut reordered = expected.clone();
        let swap_index = retained_index.checked_sub(1).unwrap_or(retained_index + 1);
        reordered.swap(retained_index, swap_index);
        assert_poison(reordered);
        let replayed = super::retained_surface_tile_credits_from_receipts_v1(
            &fixture.surface_configuration,
            TRANSACTION,
            accepted.physical.ingress.inner().receipts(),
        )
        .expect("authentic retained replay");
        let credit = replayed
            .iter()
            .find(|credit| credit.store_key == retained.surface_key)
            .expect("replayed retained forest credit");
        assert_ne!(
            credit.energy_j_m2_ofe_ground.to_bits(),
            credit.energy_j_m2_tile_ground.to_bits(),
        );
        let mut wrong_basis_formula = expected.clone();
        wrong_basis_formula[retained_index].energy_j_m2_tile_ground = credit.energy_j_m2_ofe_ground;
        assert_poison(wrong_basis_formula);
        assert_eq!(
            beginning.canonical_bytes().expect("rollback exact bytes"),
            beginning_bytes,
        );
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn exact_surface_v2_v3_high_mirror_poisons_roll_back_all_beginning_bytes() {
    let fixture = runtime_fixture(272.5, false, 0.0);
    let beginning = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
        &fixture.lse_configuration,
        &fixture.lse_state,
        &fixture.surface_configuration,
        &fixture.surface_owner,
    )
    .expect("exact owner adoption");
    let accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: physical_input(&fixture),
        beginning_exact_surface_owner: &beginning,
    })
    .expect("accepted exact-surface candidate");
    let expected = &accepted.exact_surface_receipt.accepted_operands;
    let beginning_bytes = beginning.canonical_bytes().expect("beginning exact bytes");
    let surface_beginning_bytes = fixture
        .surface_owner
        .canonical_bytes(
            fixture.surface_configuration.parent(),
            Some(&fixture.surface_configuration),
        )
        .expect("beginning surface bytes");
    let lse_beginning_bytes = serde_json::to_vec(&fixture.lse_state).expect("beginning LSE bytes");

    let mut lse_poison = accepted.physical.ending_lse_state.clone();
    let poisoned_tile = lse_poison
        .0
        .tiles
        .iter_mut()
        .find(|tile| tile.tile_id.as_str() == "forest")
        .expect("forest LSE tile");
    poisoned_tile.surface_enthalpy_j_m2_tile_ground =
        f64::from_bits(poisoned_tile.surface_enthalpy_j_m2_tile_ground.to_bits() ^ 1);
    lse_poison.0.state_sha256 = lse_poison.canonical_sha256().expect("poisoned LSE digest");
    assert!(
        beginning
            .advance_exact(
                &lse_poison,
                &fixture.surface_configuration,
                &accepted.physical.ending_surface_owner,
                TRANSACTION,
                None,
                SUPPORT_START_NS,
                SUPPORT_END_NS,
                expected,
                expected.clone(),
            )
            .is_err(),
        "V3 high-mirror mismatch must refuse before owner replacement",
    );

    let surface_state = accepted
        .physical
        .ending_surface_owner
        .v2_state()
        .expect("ending V2 state");
    let surface_poison = accepted
        .physical
        .ending_surface_owner
        .try_replace_v2_state(
            &fixture.surface_configuration,
            surface_state
                .records()
                .iter()
                .cloned()
                .map(|mut record| {
                    if record.key.tile_id.as_str() == "forest" {
                        record.surface_enthalpy_j_m2_tile =
                            f64::from_bits(record.surface_enthalpy_j_m2_tile.to_bits() ^ 1);
                    }
                    record
                })
                .collect(),
            surface_state.continuations().to_vec(),
        )
        .expect("sealed V2 high poison");
    assert!(
        beginning
            .advance_exact(
                &accepted.physical.ending_lse_state,
                &fixture.surface_configuration,
                &surface_poison,
                TRANSACTION,
                None,
                SUPPORT_START_NS,
                SUPPORT_END_NS,
                expected,
                expected.clone(),
            )
            .is_err(),
        "V2 high-mirror mismatch must refuse before owner replacement",
    );
    assert_eq!(
        beginning.canonical_bytes().expect("rollback bytes"),
        beginning_bytes
    );
    assert_eq!(
        fixture
            .surface_owner
            .canonical_bytes(
                fixture.surface_configuration.parent(),
                Some(&fixture.surface_configuration),
            )
            .expect("rollback surface bytes"),
        surface_beginning_bytes,
    );
    assert_eq!(
        serde_json::to_vec(&fixture.lse_state).expect("rollback LSE bytes"),
        lse_beginning_bytes,
    );
}

#[test]
fn exact_surface_noncanonical_carry_and_stale_restart_checkpoint_refuse() {
    let fixture = runtime_fixture(272.5, false, 0.0);
    let beginning = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
        &fixture.lse_configuration,
        &fixture.lse_state,
        &fixture.surface_configuration,
        &fixture.surface_owner,
    )
    .expect("exact owner adoption");
    let accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: physical_input(&fixture),
        beginning_exact_surface_owner: &beginning,
    })
    .expect("accepted exact-surface candidate");
    let beginning_bytes = beginning.canonical_bytes().expect("beginning exact bytes");

    let mut noncanonical_carry = accepted.ending_exact_surface_owner.clone();
    noncanonical_carry.records[0].enthalpy_carry = ExactDyadicEnthalpy {
        sign: 0,
        coefficient_hex: "1".to_owned(),
        exponent2: 0,
    };
    assert!(noncanonical_carry.validate().is_err());

    let mut stale_restart = accepted
        .ending_exact_surface_owner
        .restart()
        .expect("ending restart");
    stale_restart.owner = beginning.clone();
    assert!(stale_restart.validate().is_err());

    let mut stale_checkpoint = accepted
        .ending_exact_surface_owner
        .checkpoint(Some(accepted.exact_surface_receipt.clone()))
        .expect("ending checkpoint");
    stale_checkpoint.owner = beginning.clone();
    assert!(stale_checkpoint.validate().is_err());

    assert_eq!(
        beginning.canonical_bytes().expect("rollback bytes"),
        beginning_bytes
    );
}

#[test]
fn exact_surface_true_noop_preserves_negative_zero_high_bits() {
    let mut fixture = runtime_fixture(273.15, false, 0.0);
    for tile in &mut fixture.lse_state.0.tiles {
        if tile.tile_id.as_str() == "forest" {
            tile.surface_enthalpy_j_m2_tile_ground = -0.0;
        }
    }
    fixture.lse_state.0.state_sha256 = fixture
        .lse_state
        .canonical_sha256()
        .expect("negative-zero V3 state digest");
    let (surface_configuration, surface_owner) = surface_v2_fixture(&fixture.lse_state);
    fixture.surface_configuration = surface_configuration;
    fixture.surface_owner = surface_owner;
    fixture.phase_inputs = vec![phase_input(
        &fixture.surface_configuration,
        &fixture.surface_owner,
        273.15,
        false,
    )];
    fixture.ingress = ingress(&fixture.surface_configuration, 0.0);
    let beginning = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
        &fixture.lse_configuration,
        &fixture.lse_state,
        &fixture.surface_configuration,
        &fixture.surface_owner,
    )
    .expect("negative-zero exact owner adoption");
    let accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: physical_input(&fixture),
        beginning_exact_surface_owner: &beginning,
    })
    .expect("accepted negative-zero no-op");
    let forest = accepted
        .ending_exact_surface_owner
        .records()
        .iter()
        .find(|record| record.surface_key.tile_id.as_str() == "forest")
        .expect("forest exact record");
    assert_eq!(forest.enthalpy_hi_j_m2_tile.to_bits(), (-0.0_f64).to_bits());
    assert_eq!(
        accepted
            .physical
            .ending_surface_owner
            .v2_state()
            .expect("ending V2")
            .records()
            .iter()
            .find(|record| record.key.tile_id.as_str() == "forest")
            .expect("forest surface record")
            .surface_enthalpy_j_m2_tile
            .to_bits(),
        (-0.0_f64).to_bits(),
    );
}

#[allow(clippy::too_many_lines)]
fn exact_surface_named_operand_trial(
    beginning_high: f64,
    nonzero_values: &[f64],
) -> Result<(f64, ExactDyadicEnthalpy), crate::LseSurfaceEnthalpyErrorV1> {
    let mut fixture = runtime_fixture(273.15, false, 0.0);
    for tile in &mut fixture.lse_state.0.tiles {
        if tile.tile_id.as_str() == "forest" {
            tile.surface_enthalpy_j_m2_tile_ground = beginning_high;
        }
    }
    fixture.lse_state.0.state_sha256 = fixture.lse_state.canonical_sha256().expect("V3 digest");
    let (surface_configuration, surface_owner) = surface_v2_fixture(&fixture.lse_state);
    fixture.surface_configuration = surface_configuration;
    fixture.surface_owner = surface_owner;
    let beginning = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("lse-surface-enthalpy-vector").expect("exact owner ID"),
        &fixture.lse_configuration,
        &fixture.lse_state,
        &fixture.surface_configuration,
        &fixture.surface_owner,
    )
    .expect("vector adoption");
    let key = beginning
        .records()
        .iter()
        .find(|record| record.surface_key.tile_id.as_str() == "forest")
        .expect("forest exact key")
        .surface_key
        .clone();
    let mut values = [0.0; 7];
    for (target, source) in values.iter_mut().zip(nonzero_values) {
        *target = *source;
    }
    let operands = values
        .iter()
        .enumerate()
        .map(
            |(ordinal, value)| LseSurfaceEnthalpyAcceptedEnergyOperandV1 {
                surface_key: key.clone(),
                kind: if ordinal < 6 {
                    LseSurfaceEnthalpyEnergyOperandKindV1::PhaseFreeSurfaceEnergy
                } else {
                    LseSurfaceEnthalpyEnergyOperandKindV1::LitterFusionEnergy
                },
                ordinal: if ordinal < 6 {
                    u32::try_from(ordinal).expect("phase ordinal")
                } else {
                    0
                },
                source_owner_id: fixture.surface_configuration.parent().owner_id.clone(),
                source_receipt_sha256: typed_digest('a'),
                transaction_id: TRANSACTION,
                predecessor_transaction_id: None,
                support_start_ns: SUPPORT_START_NS,
                support_end_ns: SUPPORT_END_NS,
                units: "J m^-2 tile-ground".to_owned(),
                basis: "tile_ground".to_owned(),
                energy_j_m2_tile_ground: *value,
            },
        )
        .collect::<Vec<_>>();
    let (expected_high, expected_carry) = ExactDyadicEnthalpy::exact_sum_binary64(
        beginning_high,
        &ExactDyadicEnthalpy::zero(),
        &values,
    )?
    .rounded_high_and_remainder()?;
    let mut ending_lse = fixture.lse_state.clone();
    for tile in &mut ending_lse.0.tiles {
        if tile.tile_id.as_str() == "forest" {
            tile.surface_enthalpy_j_m2_tile_ground = expected_high;
        }
    }
    ending_lse.0.state_sha256 = ending_lse.canonical_sha256().expect("ending V3 digest");
    let surface_state = fixture.surface_owner.v2_state().expect("surface V2");
    let ending_surface = fixture
        .surface_owner
        .try_replace_v2_state(
            &fixture.surface_configuration,
            surface_state
                .records()
                .iter()
                .cloned()
                .map(|mut record| {
                    if record.key == key {
                        record.surface_enthalpy_j_m2_tile = expected_high;
                    }
                    record
                })
                .collect(),
            surface_state.continuations().to_vec(),
        )
        .expect("ending surface V2");
    let accepted = beginning.advance_exact(
        &ending_lse,
        &fixture.surface_configuration,
        &ending_surface,
        TRANSACTION,
        None,
        SUPPORT_START_NS,
        SUPPORT_END_NS,
        &operands,
        operands.clone(),
    )?;
    let ending = accepted
        .ending_owner
        .records()
        .iter()
        .find(|record| record.surface_key == key)
        .expect("ending forest exact record");
    assert_eq!(
        ending.enthalpy_hi_j_m2_tile.to_bits(),
        expected_high.to_bits()
    );
    assert_eq!(ending.enthalpy_carry, expected_carry);
    Ok((ending.enthalpy_hi_j_m2_tile, ending.enthalpy_carry.clone()))
}

#[test]
fn exact_surface_rounding_vectors_cover_sub_ulp_ties_crossing_cancellation_and_subnormal() {
    let (_, positive_sub_ulp) =
        exact_surface_named_operand_trial(1.0, &[2.0_f64.powi(-54)]).expect("positive sub-ULP");
    let (_, negative_sub_ulp) =
        exact_surface_named_operand_trial(1.0, &[-2.0_f64.powi(-54)]).expect("negative sub-ULP");
    assert_ne!(positive_sub_ulp, ExactDyadicEnthalpy::zero());
    assert_ne!(negative_sub_ulp, ExactDyadicEnthalpy::zero());

    let (even_tie, _) =
        exact_surface_named_operand_trial(1.0, &[2.0_f64.powi(-53)]).expect("even tie");
    assert_eq!(even_tie.to_bits(), 1.0_f64.to_bits());
    let odd = f64::from_bits(1.0_f64.to_bits() + 1);
    let (odd_tie, _) =
        exact_surface_named_operand_trial(odd, &[2.0_f64.powi(-53)]).expect("odd tie");
    assert_eq!(odd_tie.to_bits(), 1.0_f64.to_bits() + 2);

    let (crossing, _) =
        exact_surface_named_operand_trial(1.0, &[2.0_f64.powi(-53), f64::from_bits(1)])
            .expect("ULP crossing");
    assert_eq!(crossing.to_bits(), 1.0_f64.to_bits() + 1);
    let (cancelled, cancelled_carry) =
        exact_surface_named_operand_trial(1.0, &[2.0_f64.powi(100), -2.0_f64.powi(100)])
            .expect("cancellation");
    assert_eq!(cancelled.to_bits(), 1.0_f64.to_bits());
    assert_eq!(cancelled_carry, ExactDyadicEnthalpy::zero());
    let (positive_subnormal, carry) =
        exact_surface_named_operand_trial(0.0, &[f64::from_bits(1)]).expect("subnormal");
    assert_eq!(positive_subnormal.to_bits(), 1);
    assert_eq!(carry, ExactDyadicEnthalpy::zero());
    let (negative_subnormal, carry) =
        exact_surface_named_operand_trial(0.0, &[f64::from_bits((1_u64 << 63) | 1)])
            .expect("negative subnormal");
    assert_eq!(negative_subnormal.to_bits(), (1_u64 << 63) | 1);
    assert_eq!(carry, ExactDyadicEnthalpy::zero());

    let (largest_finite, carry) =
        exact_surface_named_operand_trial(f64::MAX, &[-f64::from_bits(1)])
            .expect("largest-finite boundary");
    assert_eq!(largest_finite.to_bits(), f64::MAX.to_bits());
    assert_ne!(carry, ExactDyadicEnthalpy::zero());

    assert!(exact_surface_named_operand_trial(f64::MAX, &[f64::MAX]).is_err());
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
