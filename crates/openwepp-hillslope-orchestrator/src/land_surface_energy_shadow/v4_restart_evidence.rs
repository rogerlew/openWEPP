//! Feature-gated accepted V4 evidence for persisted-restart verification.
//!
//! This module deliberately executes the same production V4 coordinator used
//! by the frozen-litter runtime tests. It is not compiled into default builds
//! and does not add a production execution path.

use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::{
    BeginningLitterPhaseState, ExactDyadicEnthalpy, FinalizedLitterVapor,
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyV3State, LitterPhaseConfiguration,
    LitterVaporEnvironment, Sha256Digest, SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2,
    SoilThermalV2MigrationIdentity, V2_MODEL_DEFINITION_SHA256, V2_MODEL_VERSION,
    V2_VEGETATION_MODEL_DEFINITION_SHA256, V2_VEGETATION_MODEL_VERSION,
    V3PhaseFreeSurfaceEnergyLedger, evaluate_raw_litter_vapor, finalize_litter_vapor,
    install_finalized_vapor, liquid_enthalpy_j_kg, migrate_soil_thermal_v1_to_v2,
    migrate_v2_configuration_to_v3, migrate_v2_state_to_v3, prepare_soil_thermal_support_v2,
    project_validated_v1_runtime_to_v2, saturation_specific_humidity,
    seal_soil_thermal_receipt_free_owner_v2,
};

use crate::direct_runtime::{
    DirectCanopyLiquidRelease, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectSurfaceLiquidIngressInput, DirectTileGroundIngress, DirectWb14CoupledChildBindingV1,
    LseSurfaceEnthalpyEnergyCreditReceiptV1, LseSurfaceEnthalpyOwnerCheckpointV1,
    LseSurfaceEnthalpyOwnerEnvelopeV1, LseSurfaceEnthalpyOwnerRestartV1,
    SurfaceLiquidCompleteOwnerProjectionV3, SurfaceLiquidCompleteOwnerProjectionV4,
    SurfaceLiquidConfigurationV2, SurfaceLiquidOwnedStateV2, SurfaceLiquidOwnerEnvelopeV2,
    SurfaceLiquidOwnerModelDefinitionV2,
};

use super::endpoint_fixture;
use super::v3_execution::{
    FrozenLitterV3RuntimeInput, FrozenLitterV4RuntimeInput, execute_frozen_litter_v4,
};
use super::v3_input_projection::FrozenLitterV3PhaseFreeInput;
use crate::v9_real_consumer_shadow::FrozenLitterV3PublicationSupportV1;

const TRANSACTION: TransactionId = TransactionId(703);
const SUPPORT_START_NS: u128 = 0;
const SUPPORT_END_NS: u128 = 1_800_000_000_000;
const PARENT_END_NS: u128 = SUPPORT_END_NS;
const SUCCESSOR_TRANSACTION: TransactionId = TransactionId(704);
const SUCCESSOR_SUPPORT_END_NS: u128 = 3_600_000_000_000;

/// Public, accepted production-coordinator constituents used to prove that a
/// persisted V4 reload preserves exact enthalpy, including signed zero.
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptedNegativeZeroV4EvidenceV1 {
    pub transaction_id: TransactionId,
    pub predecessor_transaction_id: Option<TransactionId>,
    pub parent_support_start_ns: u128,
    pub parent_support_end_ns: u128,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub predecessor_receipt_chain_sha256: String,
    pub lse_configuration: LandSurfaceEnergyConfiguration,
    pub beginning_lse_state: LandSurfaceEnergyV3State,
    pub ending_lse_state: LandSurfaceEnergyV3State,
    pub surface_configuration: SurfaceLiquidConfigurationV2,
    pub beginning_surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    pub ending_surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    pub beginning_exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    pub ending_exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    pub ending_exact_surface_restart: LseSurfaceEnthalpyOwnerRestartV1,
    pub ending_exact_surface_checkpoint: LseSurfaceEnthalpyOwnerCheckpointV1,
    pub exact_surface_receipt: LseSurfaceEnthalpyEnergyCreditReceiptV1,
    pub projection_v3: SurfaceLiquidCompleteOwnerProjectionV3,
    pub projection_v4: SurfaceLiquidCompleteOwnerProjectionV4,
    pub physical_v3_publication_bytes: Vec<u8>,
    pub soil_thermal_owner: SoilThermalOwnerEnvelopeV2,
    pub soil_thermal_restart: SoilThermalOwnerRestartV2,
}

/// One accepted production V4 support in a two-support continuity vector.
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptedNonzeroCarryV4SupportEvidenceV1 {
    pub transaction_id: TransactionId,
    pub predecessor_transaction_id: Option<TransactionId>,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub predecessor_receipt_chain_sha256: String,
    pub beginning_lse_state: LandSurfaceEnergyV3State,
    pub ending_lse_state: LandSurfaceEnergyV3State,
    pub beginning_surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    pub ending_surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    pub beginning_exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    pub ending_exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    pub ending_exact_surface_restart: LseSurfaceEnthalpyOwnerRestartV1,
    pub ending_exact_surface_checkpoint: LseSurfaceEnthalpyOwnerCheckpointV1,
    pub exact_surface_receipt: LseSurfaceEnthalpyEnergyCreditReceiptV1,
    pub projection_v3: SurfaceLiquidCompleteOwnerProjectionV3,
    pub projection_v4: SurfaceLiquidCompleteOwnerProjectionV4,
    pub physical_v3_publication_bytes: Vec<u8>,
    pub soil_thermal_owner: SoilThermalOwnerEnvelopeV2,
    pub soil_thermal_restart: SoilThermalOwnerRestartV2,
}

/// Authentic uninterrupted two-support vector used to compare a persisted
/// split/reload with the production executor's direct continuation.
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptedNonzeroCarrySplitV4EvidenceV1 {
    pub lse_configuration: LandSurfaceEnergyConfiguration,
    pub surface_configuration: SurfaceLiquidConfigurationV2,
    pub first: AcceptedNonzeroCarryV4SupportEvidenceV1,
    pub second: AcceptedNonzeroCarryV4SupportEvidenceV1,
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

fn digest(byte: char) -> String {
    std::iter::repeat_n(byte, 64).collect()
}

fn typed_digest(byte: char) -> Sha256Digest {
    Sha256Digest::try_new(digest(byte)).expect("evidence digest")
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
        .expect("V2 configuration digest");
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
            tile.surface_enthalpy_j_m2_tile_ground = -0.0;
        }
    }
    v3_state.0.state_sha256 = v3_state.canonical_sha256().expect("V3 state digest");
    v3_state
        .validate(&v3_configuration)
        .expect("negative-zero V3 state");
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
) -> FrozenLitterV3PhaseFreeInput {
    phase_input_for_support(configuration, owner, 273.15, 1_800.0, false)
}

fn phase_input_for_support(
    configuration: &SurfaceLiquidConfigurationV2,
    owner: &SurfaceLiquidOwnerEnvelopeV2,
    beginning_temperature_k: f64,
    interval_s: f64,
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
        temperature_k: beginning_temperature_k,
    };
    let saturation =
        saturation_specific_humidity(beginning_temperature_k, 93_000.0).expect("saturation");
    let environment = LitterVaporEnvironment {
        accepted_phase_free_temperature_k: beginning_temperature_k,
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
        beginning_temperature_k,
        interval_s,
    )
    .expect("final vapor");
    let post_vapor = install_finalized_vapor(
        phase_configuration,
        beginning,
        beginning_temperature_k,
        vapor,
    )
    .expect("post vapor");
    let storage =
        (post_vapor.sensible_energy_j_m2_tile - beginning.sensible_energy_j_m2_tile) / interval_s;
    let liquid_vapor = vapor.liquid_signed_energy_j_m2 / interval_s;
    let ice_vapor = vapor.ice_signed_energy_j_m2 / interval_s;
    let exact_carry_probe_w_m2 = if evaporating {
        2.0_f64.powi(-54) / interval_s
    } else {
        0.0
    };
    let surface_energy = V3PhaseFreeSurfaceEnergyLedger {
        beginning_sensible_energy_j_m2: beginning.sensible_energy_j_m2_tile,
        ending_sensible_energy_j_m2: post_vapor.sensible_energy_j_m2_tile,
        absorbed_shortwave_w_m2: storage + liquid_vapor + ice_vapor,
        net_longwave_w_m2: exact_carry_probe_w_m2,
        sensible_to_canopy_air_w_m2: 0.0,
        liquid_vapor_energy_w_m2: liquid_vapor,
        ice_vapor_energy_w_m2: ice_vapor,
        ground_heat_w_m2: 0.0,
        storage_w_m2: storage,
        reconstructed_energy_residual_w_m2: exact_carry_probe_w_m2,
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
        end_s: 1_800.0,
    }
}

fn ingress(configuration: &SurfaceLiquidConfigurationV2) -> DirectSurfaceLiquidIngressInput {
    ingress_for_support(configuration, TRANSACTION, 1_800.0)
}

fn ingress_for_support(
    configuration: &SurfaceLiquidConfigurationV2,
    transaction_id: TransactionId,
    interval_s: f64,
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
                        throughfall: amount(0.0),
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
        transaction_id,
        day_index: 0,
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

#[allow(clippy::too_many_arguments)]
fn accepted_support_evidence(
    configuration: &SurfaceLiquidConfigurationV2,
    accepted: super::v3_execution::AcceptedFrozenLitterV4RuntimeCandidate,
    transaction_id: TransactionId,
    predecessor_transaction_id: Option<TransactionId>,
    support_start_ns: u128,
    support_end_ns: u128,
    predecessor_receipt_chain_sha256: String,
    beginning_lse_state: LandSurfaceEnergyV3State,
    beginning_surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    beginning_exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    soil_thermal_owner: SoilThermalOwnerEnvelopeV2,
    soil_thermal_restart: SoilThermalOwnerRestartV2,
) -> AcceptedNonzeroCarryV4SupportEvidenceV1 {
    let physical_v3_publication_bytes = FrozenLitterV3PublicationSupportV1::try_new(
        configuration,
        &accepted.physical.complete_owner_projection,
        &accepted.physical.litter_phase_receipts,
    )
    .and_then(|publication| publication.canonical_bytes(configuration))
    .expect("accepted physical V3 publication bytes");
    let ending_exact_surface_restart = accepted
        .ending_exact_surface_owner
        .restart()
        .expect("ending exact restart");
    let ending_exact_surface_checkpoint = accepted
        .ending_exact_surface_owner
        .checkpoint(Some(accepted.exact_surface_receipt.clone()))
        .expect("ending exact checkpoint");
    AcceptedNonzeroCarryV4SupportEvidenceV1 {
        transaction_id,
        predecessor_transaction_id,
        support_start_ns,
        support_end_ns,
        predecessor_receipt_chain_sha256,
        beginning_lse_state,
        ending_lse_state: accepted.physical.ending_lse_state,
        beginning_surface_owner,
        ending_surface_owner: accepted.physical.ending_surface_owner,
        beginning_exact_surface_owner,
        ending_exact_surface_owner: accepted.ending_exact_surface_owner,
        ending_exact_surface_restart,
        ending_exact_surface_checkpoint,
        exact_surface_receipt: accepted.exact_surface_receipt,
        projection_v3: accepted.physical.complete_owner_projection,
        projection_v4: accepted.complete_owner_projection,
        physical_v3_publication_bytes,
        soil_thermal_owner,
        soil_thermal_restart,
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

fn runtime_fixture() -> RuntimeFixture {
    let (lse_configuration, lse_state) = lse_v3_fixture();
    let (surface_configuration, surface_owner) = surface_v2_fixture(&lse_state);
    let phase_inputs = vec![phase_input(&surface_configuration, &surface_owner)];
    let ingress = ingress(&surface_configuration);
    let binding = DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: 1_800.0_f64.to_bits(),
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

/// Execute an authentic accepted V4 transaction whose forest exact high mirror
/// begins at negative zero and whose accepted energy operands are all zero.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn accepted_negative_zero_v4_evidence_v1() -> AcceptedNegativeZeroV4EvidenceV1 {
    let fixture = runtime_fixture();
    let beginning_exact_surface_owner = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
        &fixture.lse_configuration,
        &fixture.lse_state,
        &fixture.surface_configuration,
        &fixture.surface_owner,
    )
    .expect("negative-zero exact owner adoption");
    let predecessor_receipt_chain_sha256 = digest('b');
    let accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: FrozenLitterV3RuntimeInput {
            transaction_id: TRANSACTION,
            predecessor_transaction_id: None,
            parent_support_start_ns: SUPPORT_START_NS,
            parent_support_end_ns: PARENT_END_NS,
            support_start_ns: SUPPORT_START_NS,
            support_end_ns: SUPPORT_END_NS,
            predecessor_receipt_chain_sha256: predecessor_receipt_chain_sha256.clone(),
            surface_configuration: &fixture.surface_configuration,
            beginning_surface_owner: &fixture.surface_owner,
            lse_configuration: &fixture.lse_configuration,
            beginning_lse_state: &fixture.lse_state,
            phase_inputs: &fixture.phase_inputs,
            current_ingress: &fixture.ingress,
            wb14_parent: None,
            finalize_wb14_parent_interval: true,
            coupled_binding: fixture.binding,
            soil_thermal_owner: &fixture.soil_owner,
            soil_thermal_restart: &fixture.soil_restart,
        },
        beginning_exact_surface_owner: &beginning_exact_surface_owner,
    })
    .expect("accepted negative-zero V4 transaction");
    let physical_v3_publication_bytes = FrozenLitterV3PublicationSupportV1::try_new(
        &fixture.surface_configuration,
        &accepted.physical.complete_owner_projection,
        &accepted.physical.litter_phase_receipts,
    )
    .and_then(|publication| publication.canonical_bytes(&fixture.surface_configuration))
    .expect("accepted physical V3 publication bytes");
    let ending_exact_surface_restart = accepted
        .ending_exact_surface_owner
        .restart()
        .expect("ending exact restart");
    let ending_exact_surface_checkpoint = accepted
        .ending_exact_surface_owner
        .checkpoint(Some(accepted.exact_surface_receipt.clone()))
        .expect("ending exact checkpoint");

    AcceptedNegativeZeroV4EvidenceV1 {
        transaction_id: TRANSACTION,
        predecessor_transaction_id: None,
        parent_support_start_ns: SUPPORT_START_NS,
        parent_support_end_ns: PARENT_END_NS,
        support_start_ns: SUPPORT_START_NS,
        support_end_ns: SUPPORT_END_NS,
        predecessor_receipt_chain_sha256,
        lse_configuration: fixture.lse_configuration,
        beginning_lse_state: fixture.lse_state,
        ending_lse_state: accepted.physical.ending_lse_state,
        surface_configuration: fixture.surface_configuration,
        beginning_surface_owner: fixture.surface_owner,
        ending_surface_owner: accepted.physical.ending_surface_owner,
        beginning_exact_surface_owner,
        ending_exact_surface_owner: accepted.ending_exact_surface_owner,
        ending_exact_surface_restart,
        ending_exact_surface_checkpoint,
        exact_surface_receipt: accepted.exact_surface_receipt,
        projection_v3: accepted.physical.complete_owner_projection,
        projection_v4: accepted.complete_owner_projection,
        physical_v3_publication_bytes,
        soil_thermal_owner: fixture.soil_owner,
        soil_thermal_restart: fixture.soil_restart,
    }
}

/// Execute the second authentic support from constituents recovered at a
/// checkpoint boundary. Callers must pass the admitted physical V3 and exact
/// V4 beginning owners; the production coordinator performs the next support.
#[must_use]
#[allow(
    clippy::missing_panics_doc,
    clippy::too_many_arguments,
    clippy::too_many_lines
)]
pub fn execute_nonzero_carry_successor_after_reload_v1(
    lse_configuration: &LandSurfaceEnergyConfiguration,
    surface_configuration: &SurfaceLiquidConfigurationV2,
    beginning_lse_state: &LandSurfaceEnergyV3State,
    beginning_surface_owner: &SurfaceLiquidOwnerEnvelopeV2,
    beginning_exact_surface_owner: &LseSurfaceEnthalpyOwnerEnvelopeV1,
    predecessor_projection_v3: &SurfaceLiquidCompleteOwnerProjectionV3,
    beginning_soil_owner: &SoilThermalOwnerEnvelopeV2,
) -> AcceptedNonzeroCarryV4SupportEvidenceV1 {
    let prepared_second_soil = prepare_soil_thermal_support_v2(
        beginning_soil_owner,
        SUCCESSOR_TRANSACTION,
        SUPPORT_END_NS,
        SUCCESSOR_SUPPORT_END_NS,
    )
    .expect("prepare successor soil support");
    let second_soil_owner = prepared_second_soil.beginning_owner().clone();
    let second_soil_restart = seal_soil_thermal_receipt_free_owner_v2(&prepared_second_soil)
        .expect("seal successor soil support")
        .restart;
    let second_temperature_k = beginning_lse_state
        .0
        .tiles
        .iter()
        .find(|tile| tile.tile_id.as_str() == "forest")
        .expect("reloaded beginning forest LSE tile")
        .surface_temperature_warm_start_k;
    let second_phase_inputs = vec![phase_input_for_support(
        surface_configuration,
        beginning_surface_owner,
        second_temperature_k,
        1_800.0,
        false,
    )];
    let second_ingress = ingress_for_support(surface_configuration, SUCCESSOR_TRANSACTION, 1_800.0);
    let second_predecessor_chain = predecessor_projection_v3
        .identity()
        .receipt_chain_sha256
        .clone();
    let second_accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: FrozenLitterV3RuntimeInput {
            transaction_id: SUCCESSOR_TRANSACTION,
            predecessor_transaction_id: Some(TRANSACTION),
            parent_support_start_ns: SUPPORT_END_NS,
            parent_support_end_ns: SUCCESSOR_SUPPORT_END_NS,
            support_start_ns: SUPPORT_END_NS,
            support_end_ns: SUCCESSOR_SUPPORT_END_NS,
            predecessor_receipt_chain_sha256: second_predecessor_chain.clone(),
            surface_configuration,
            beginning_surface_owner,
            lse_configuration,
            beginning_lse_state,
            phase_inputs: &second_phase_inputs,
            current_ingress: &second_ingress,
            wb14_parent: None,
            finalize_wb14_parent_interval: true,
            coupled_binding: DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: 1_800.0_f64.to_bits(),
                coupled_parent_transaction_sha256: [21; 32],
                accepted_slab_sha256: [22; 32],
                parent_beginning_complete_owner_set_sha256: [23; 32],
                parent_support_start_ns: SUPPORT_END_NS,
                parent_support_end_ns: SUCCESSOR_SUPPORT_END_NS,
                child_support_start_ns: SUPPORT_END_NS,
                child_support_end_ns: SUCCESSOR_SUPPORT_END_NS,
            },
            soil_thermal_owner: &second_soil_owner,
            soil_thermal_restart: &second_soil_restart,
        },
        beginning_exact_surface_owner,
    })
    .expect("accepted successor exact support from reloaded owners");
    accepted_support_evidence(
        surface_configuration,
        second_accepted,
        SUCCESSOR_TRANSACTION,
        Some(TRANSACTION),
        SUPPORT_END_NS,
        SUCCESSOR_SUPPORT_END_NS,
        second_predecessor_chain,
        beginning_lse_state.clone(),
        beginning_surface_owner.clone(),
        beginning_exact_surface_owner.clone(),
        second_soil_owner,
        second_soil_restart,
    )
}

/// Execute two lawful, parent-final V4 supports without a checkpoint boundary.
/// The first support exercises real vapor-energy operands that leave a
/// nonzero exact remainder; the second consumes that exact owner as its only
/// beginning authority.
#[must_use]
#[allow(clippy::missing_panics_doc, clippy::too_many_lines)]
pub fn accepted_nonzero_carry_split_v4_evidence_v1() -> AcceptedNonzeroCarrySplitV4EvidenceV1 {
    let fixture = runtime_fixture();
    let first_beginning_lse = fixture.lse_state.clone();
    let first_beginning_surface = fixture.surface_owner.clone();
    let first_beginning_exact = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID"),
        &fixture.lse_configuration,
        &first_beginning_lse,
        &fixture.surface_configuration,
        &first_beginning_surface,
    )
    .expect("exact owner adoption");
    let first_phase_inputs = vec![phase_input_for_support(
        &fixture.surface_configuration,
        &first_beginning_surface,
        273.15,
        1_800.0,
        true,
    )];
    let mut first_ingress =
        ingress_for_support(&fixture.surface_configuration, TRANSACTION, 1_800.0);
    for tile in &mut first_ingress.tile_ingress {
        if let DirectTileGroundIngress::CoveredCanopyRelease { release, .. } = tile {
            release.throughfall = DirectIngressAmount {
                mass_kg_m2_tile_ground: 1.0,
                temperature_k: 274.0,
                specific_liquid_enthalpy_j_kg: liquid_enthalpy_j_kg(274.0),
                start_s: 0.0,
                end_s: 1_800.0,
            };
        }
    }
    let first_predecessor_chain = digest('b');
    let first_accepted = execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: FrozenLitterV3RuntimeInput {
            transaction_id: TRANSACTION,
            predecessor_transaction_id: None,
            parent_support_start_ns: SUPPORT_START_NS,
            parent_support_end_ns: SUPPORT_END_NS,
            support_start_ns: SUPPORT_START_NS,
            support_end_ns: SUPPORT_END_NS,
            predecessor_receipt_chain_sha256: first_predecessor_chain.clone(),
            surface_configuration: &fixture.surface_configuration,
            beginning_surface_owner: &first_beginning_surface,
            lse_configuration: &fixture.lse_configuration,
            beginning_lse_state: &first_beginning_lse,
            phase_inputs: &first_phase_inputs,
            current_ingress: &first_ingress,
            wb14_parent: None,
            finalize_wb14_parent_interval: true,
            coupled_binding: DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: 1_800.0_f64.to_bits(),
                coupled_parent_transaction_sha256: [11; 32],
                accepted_slab_sha256: [12; 32],
                parent_beginning_complete_owner_set_sha256: [13; 32],
                parent_support_start_ns: SUPPORT_START_NS,
                parent_support_end_ns: SUPPORT_END_NS,
                child_support_start_ns: SUPPORT_START_NS,
                child_support_end_ns: SUPPORT_END_NS,
            },
            soil_thermal_owner: &fixture.soil_owner,
            soil_thermal_restart: &fixture.soil_restart,
        },
        beginning_exact_surface_owner: &first_beginning_exact,
    })
    .expect("accepted first nonzero-carry support");
    assert!(
        first_accepted
            .ending_exact_surface_owner
            .records()
            .iter()
            .any(|record| record.enthalpy_carry != ExactDyadicEnthalpy::zero()),
        "the first authentic support must retain a nonzero exact carry",
    );
    let first = accepted_support_evidence(
        &fixture.surface_configuration,
        first_accepted,
        TRANSACTION,
        None,
        SUPPORT_START_NS,
        SUPPORT_END_NS,
        first_predecessor_chain,
        first_beginning_lse,
        first_beginning_surface,
        first_beginning_exact,
        fixture.soil_owner.clone(),
        fixture.soil_restart.clone(),
    );

    let second = execute_nonzero_carry_successor_after_reload_v1(
        &fixture.lse_configuration,
        &fixture.surface_configuration,
        &first.ending_lse_state,
        &first.ending_surface_owner,
        &first.ending_exact_surface_owner,
        &first.projection_v3,
        &first.soil_thermal_owner,
    );
    AcceptedNonzeroCarrySplitV4EvidenceV1 {
        lse_configuration: fixture.lse_configuration,
        surface_configuration: fixture.surface_configuration,
        first,
        second,
    }
}

#[cfg(test)]
mod tests {
    use openwepp_land_surface_energy::ExactDyadicEnthalpy;

    use super::{
        accepted_negative_zero_v4_evidence_v1, accepted_nonzero_carry_split_v4_evidence_v1,
    };

    #[test]
    fn accepted_evidence_preserves_negative_zero_and_replays_projection() {
        let evidence = accepted_negative_zero_v4_evidence_v1();
        let forest = evidence
            .ending_exact_surface_owner
            .records()
            .iter()
            .find(|record| record.surface_key.tile_id.as_str() == "forest")
            .expect("forest exact record");
        assert_eq!(forest.enthalpy_hi_j_m2_tile.to_bits(), (-0.0_f64).to_bits());
        assert!(
            evidence
                .exact_surface_receipt
                .accepted_operands
                .iter()
                .all(|operand| operand.energy_j_m2_tile_ground == 0.0)
        );
        evidence
            .projection_v4
            .validate(
                &evidence.surface_configuration,
                evidence.beginning_lse_state.0.state_sha256.as_str(),
            )
            .expect("V4 projection replay");
    }

    #[test]
    fn accepted_nonzero_carry_support_advances_with_exact_continuity() {
        let evidence = accepted_nonzero_carry_split_v4_evidence_v1();
        assert_eq!(
            evidence.second.beginning_exact_surface_owner,
            evidence.first.ending_exact_surface_owner,
        );
        assert_eq!(
            evidence.second.beginning_surface_owner,
            evidence.first.ending_surface_owner,
        );
        assert_eq!(
            evidence.second.beginning_lse_state,
            evidence.first.ending_lse_state,
        );
        assert!(
            evidence
                .first
                .ending_exact_surface_owner
                .records()
                .iter()
                .any(|record| record.enthalpy_carry != ExactDyadicEnthalpy::zero())
        );
        evidence
            .second
            .exact_surface_receipt
            .validate(
                &evidence.first.ending_exact_surface_owner,
                &evidence.second.ending_exact_surface_owner,
            )
            .expect("successor receipt continuity");
    }
}
