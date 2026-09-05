//! Focused canonical retention tests for accepted frozen-litter V3 supports.

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use openwepp_land_surface_energy::{
    BeginningLitterPhaseState, FinalizedLitterVapor, LitterPhaseConfiguration, LitterPhaseReceipt,
    LitterPhaseTransactionIdentity, LitterPhaseTransactionInput, LitterVaporEnvironment, OfeId,
    Sha256Digest, SoilThermalLayerSnapshot, SoilThermalOfeSnapshot, SoilThermalOwnerEnvelopeV2,
    SoilThermalOwnerRestartV2, SoilThermalSnapshot, SoilThermalV2MigrationIdentity, SourceId,
    SurfaceClass, SurfaceConfiguration, SurfaceId, V2_MODEL_DEFINITION_SHA256, V2_MODEL_VERSION,
    V2_VEGETATION_MODEL_DEFINITION_SHA256, V2_VEGETATION_MODEL_VERSION,
    V3PhaseFreeSurfaceEnergyLedger, WaterSourceType, evaluate_raw_litter_vapor,
    execute_litter_phase_v3, finalize_litter_vapor, install_finalized_vapor,
    migrate_soil_thermal_v1_to_v2, migrate_v2_configuration_to_v3, migrate_v2_state_to_v3,
    project_validated_v1_runtime_to_v2, saturation_specific_humidity,
};

use crate::direct_runtime::{
    DirectCanopyLiquidRelease, DirectIngressAmount, DirectSurfaceLiquidIngressInput,
    DirectTileGroundIngress, DirectWb14CoupledChildBindingV1,
    execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding,
    prepare_surface_liquid_resource_candidate_v2,
};
use crate::{
    DirectGroundIngressMode, DirectOfeWb14Parameters, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidConfigurationRecord, DirectSurfaceLiquidOfeBinding,
    DirectSurfaceLiquidStoreKey, SurfaceLiquidCompleteOwnerProjectionIdentityV3,
    SurfaceLiquidCompleteOwnerProjectionV3, SurfaceLiquidConfigurationV2,
    SurfaceLiquidOwnedStateV2, SurfaceLiquidOwnerClosureRecordV2, SurfaceLiquidOwnerEnvelopeV2,
    SurfaceLiquidOwnerModelDefinitionV2,
};

use serde::{Deserialize, Serialize};

use super::frozen_litter_v3_adoption::{
    frozen_litter_v3_handoff_counters_for_test, reset_frozen_litter_v3_handoff_counters_for_test,
};
use super::{
    Digest32, FrozenLitterV3PublicationSupportV1, FrozenLitterV3Resident, FrozenLitterV4Resident,
    digest_bytes,
};
use crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2;
use crate::land_surface_energy_shadow::v3_execution::{
    AcceptedFrozenLitterV3RuntimeCandidate, AcceptedFrozenLitterV4RuntimeCandidate,
    FrozenLitterV3RuntimeInput, FrozenLitterV3SoilBeginningV1, FrozenLitterV4RuntimeInput,
    execute_frozen_litter_v3, execute_frozen_litter_v4,
};
use crate::land_surface_energy_shadow::v3_input_projection::FrozenLitterV3PhaseFreeInput;

const TRANSACTION: TransactionId = TransactionId(703);
const SUPPORT_END_NS: u128 = 900_000_000_000;
const RUNTIME_SUPPORT_NS: u128 = 1_800_000_000_000;

fn digest(byte: char) -> String {
    std::iter::repeat_n(byte, 64).collect()
}

fn typed_digest(byte: char) -> Sha256Digest {
    Sha256Digest::try_new(digest(byte)).expect("test digest")
}

fn ofe() -> OfeId {
    OfeId::try_new("ofe-z").expect("OFE")
}

fn configuration() -> SurfaceLiquidConfigurationV2 {
    let ofe_id = ofe();
    let layer = SoilLayerId::try_new("soil-1").expect("layer");
    let records = ["forest-a", "forest-b"]
        .into_iter()
        .map(|tile| DirectSurfaceLiquidConfigurationRecord {
            key: DirectSurfaceLiquidStoreKey {
                run_id: 71,
                ofe_id: ofe_id.clone(),
                tile_id: TileId::try_new(tile).expect("tile"),
                surface_id: SurfaceId::try_new(format!("surface-{tile}")).expect("surface"),
                surface_class: SurfaceClass::ForestLitter,
                source_type: WaterSourceType::LitterLiquid,
                source_id: SourceId::try_new(format!("source-{tile}")).expect("source"),
            },
            tile_fraction: 0.5,
            capacity_kg_m2_tile: 2.0,
            ofe_area_m2: 100.0,
            ground_ingress_mode: DirectGroundIngressMode::CoveredCanopyRelease,
            runon_destination_ofe_id: None,
            runon_destination_tile_id: None,
        })
        .collect();
    let parent = DirectSurfaceLiquidConfiguration::new(
        ResourceOwnerId::try_new("surface-owner").expect("owner"),
        71,
        vec![ofe_id.clone()],
        vec![DirectSurfaceLiquidOfeBinding {
            ofe_id,
            production_lane_index: 0,
            production_lane_id: 1,
            ordered_soil_layer_ids: vec![layer.clone()],
            infiltration_soil_thermal_layer_id: layer,
        }],
        records,
    )
    .expect("parent configuration");
    let depths = parent
        .records
        .iter()
        .map(|record| (record.key.clone(), 0.03125))
        .collect();
    SurfaceLiquidConfigurationV2::new(
        parent,
        SurfaceLiquidOwnerModelDefinitionV2::new(digest('1'), digest('2'), digest('3'))
            .expect("surface V2 model"),
        &depths,
    )
    .expect("surface V2 configuration")
}

fn beginning_owner(configuration: &SurfaceLiquidConfigurationV2) -> SurfaceLiquidOwnerEnvelopeV2 {
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
        .map(|record| (record.key.clone(), 0.375))
        .collect();
    let enthalpy = configuration
        .parent()
        .records
        .iter()
        .map(|record| (record.key.clone(), 0.0))
        .collect();
    let state = SurfaceLiquidOwnedStateV2::new_initial(configuration, &liquid, &ice, &enthalpy, 0)
        .expect("surface V2 state");
    SurfaceLiquidOwnerEnvelopeV2::wrap_v2(configuration, state).expect("surface V2 owner")
}

fn zero_closure(owner: &SurfaceLiquidOwnerEnvelopeV2) -> Vec<SurfaceLiquidOwnerClosureRecordV2> {
    owner
        .v2_state()
        .expect("surface V2 state")
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

fn amount(mass: f64) -> DirectIngressAmount {
    DirectIngressAmount {
        mass_kg_m2_tile_ground: mass,
        temperature_k: 273.15,
        specific_liquid_enthalpy_j_kg: 0.0,
        start_s: 0.0,
        end_s: 900.0,
    }
}

fn ingress(configuration: &SurfaceLiquidConfigurationV2) -> DirectSurfaceLiquidIngressInput {
    DirectSurfaceLiquidIngressInput {
        transaction_id: TRANSACTION,
        day_index: 0,
        interval_index: 0,
        interval_s: 900.0,
        tile_ingress: configuration
            .parent()
            .records
            .iter()
            .map(|record| DirectTileGroundIngress::CoveredCanopyRelease {
                ofe_id: record.key.ofe_id.clone(),
                tile_id: record.key.tile_id.clone(),
                surface_id: record.key.surface_id.clone(),
                release: DirectCanopyLiquidRelease {
                    throughfall: amount(0.02),
                    initial_drainage: amount(0.0),
                    second_drainage: amount(0.0),
                    stemflow: amount(0.0),
                },
            })
            .collect(),
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: ofe(),
            effective_conductivity_m_s: 1.0e-12,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.2,
        }],
    }
}

fn phase_receipts(
    configuration: &SurfaceLiquidConfigurationV2,
    owner: &SurfaceLiquidOwnerEnvelopeV2,
) -> Vec<LitterPhaseReceipt> {
    configuration
        .records()
        .iter()
        .map(|extension| {
            let record = configuration
                .parent()
                .records
                .iter()
                .find(|record| record.key == extension.key)
                .expect("parent record");
            let phase_configuration = LitterPhaseConfiguration {
                litter_depth_m: extension.litter_depth_m.expect("depth"),
                dry_heat_capacity_j_m2_k: 3_235.68,
                liquid_capacity_kg_m2_tile: record.capacity_kg_m2_tile,
                ice_capacity_kg_m2_tile: extension
                    .litter_ice_capacity_kg_m2_tile
                    .expect("ice capacity"),
            };
            let beginning = BeginningLitterPhaseState {
                liquid_kg_m2_tile: 0.25,
                ice_kg_m2_tile: 0.375,
                sensible_energy_j_m2_tile: 0.0,
                temperature_k: 273.15,
            };
            let humidity = saturation_specific_humidity(273.15, 93_000.0).expect("humidity");
            let environment = openwepp_land_surface_energy::LitterVaporEnvironment {
                accepted_phase_free_temperature_k: 273.15,
                air_density_kg_m3: 1.1,
                air_pressure_pa: 93_000.0,
                recipient_specific_humidity_kg_kg: humidity,
                litter_to_canopy_resistance_s_m: 80.0,
            };
            let raw = evaluate_raw_litter_vapor(phase_configuration, beginning, environment)
                .expect("raw vapor");
            assert_eq!(raw.raw_liquid_signed_rate_kg_m2_s.to_bits(), 0);
            assert_eq!(raw.raw_ice_signed_rate_kg_m2_s.to_bits(), 0);
            let owner_sha = Sha256Digest::try_new(owner.envelope_sha256()).expect("owner digest");
            execute_litter_phase_v3(&LitterPhaseTransactionInput {
                identity: LitterPhaseTransactionIdentity {
                    lse_configuration_sha256: typed_digest('4'),
                    transaction_id: TRANSACTION,
                    ofe_id: record.key.ofe_id.clone(),
                    tile_id: record.key.tile_id.clone(),
                    surface_owner_id: configuration.parent().owner_id.clone(),
                    beginning_surface_owner_sha256: owner_sha.clone(),
                    candidate_surface_owner_sha256: owner_sha,
                    support_start_ns: 0,
                    support_end_ns: SUPPORT_END_NS,
                },
                configuration: phase_configuration,
                beginning,
                vapor_environment: environment,
                finalized_vapor: FinalizedLitterVapor {
                    liquid_signed_rate_kg_m2_s: 0.0,
                    ice_signed_rate_kg_m2_s: 0.0,
                },
                phase_free_surface_energy: V3PhaseFreeSurfaceEnergyLedger {
                    beginning_sensible_energy_j_m2: 0.0,
                    ending_sensible_energy_j_m2: 0.0,
                    absorbed_shortwave_w_m2: 0.0,
                    net_longwave_w_m2: 0.0,
                    sensible_to_canopy_air_w_m2: 0.0,
                    liquid_vapor_energy_w_m2: 0.0,
                    ice_vapor_energy_w_m2: 0.0,
                    ground_heat_w_m2: 0.0,
                    storage_w_m2: 0.0,
                    reconstructed_energy_residual_w_m2: 0.0,
                },
            })
            .expect("phase receipt")
            .receipt
        })
        .collect()
}

fn soil_owner_and_restart_at(
    transaction_id: TransactionId,
    support_start_ns: u128,
    support_end_ns: u128,
) -> (SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2) {
    let snapshot = SoilThermalSnapshot {
        owner_id: ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        configuration_sha256: typed_digest('5'),
        state_sha256: typed_digest('6'),
        snapshot_sha256: typed_digest('7'),
        last_accepted_transaction_id: None,
        ofes: vec![SoilThermalOfeSnapshot {
            ofe_id: ofe(),
            ordered_layers: vec![SoilThermalLayerSnapshot {
                layer_id: SoilLayerId::try_new("soil-1").expect("layer"),
                temperature_k: 273.15,
                enthalpy_j_m2_ofe_ground: 0.0,
            }],
        }],
    };
    let owner = migrate_soil_thermal_v1_to_v2(
        &snapshot,
        SoilThermalV2MigrationIdentity {
            model_version: "OPENWEPP_SOIL_THERMAL_TEST_V2".into(),
            model_definition_sha256: typed_digest('8'),
            run_id: "71".into(),
            transaction_id,
            support_start_ns,
            support_end_ns,
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

fn soil_owner_and_restart() -> (SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2) {
    soil_owner_and_restart_at(TRANSACTION, 0, SUPPORT_END_NS)
}

struct Fixture {
    configuration: SurfaceLiquidConfigurationV2,
    receipts: Vec<LitterPhaseReceipt>,
    retained: FrozenLitterV3PublicationSupportV1,
}

#[derive(Clone, Deserialize, Serialize)]
struct TestReceiptFrame {
    ordinal: u32,
    model_version: String,
    model_definition_sha256: String,
    lse_configuration_sha256: String,
    receipt_sha256: String,
    canonical_receipt_bytes: Vec<u8>,
}

#[derive(Clone, Deserialize, Serialize)]
struct TestPublicationWire {
    schema: String,
    surface_configuration_sha256: String,
    projection_sha256: String,
    projection_receipt_chain_sha256: String,
    canonical_projection_bytes: Vec<u8>,
    ordered_litter_phase_receipts: Vec<TestReceiptFrame>,
    publication_sha256: Digest32,
}

fn reseal(mut wire: TestPublicationWire) -> Vec<u8> {
    wire.publication_sha256 = Digest32::zero();
    let zeroed = serde_json::to_vec(&wire).expect("zeroed publication wire");
    wire.publication_sha256 = digest_bytes(
        &[
            b"OPENWEPP_FROZEN_LITTER_V3_PUBLICATION_SUPPORT_V1\0".as_slice(),
            zeroed.as_slice(),
        ]
        .concat(),
    );
    serde_json::to_vec(&wire).expect("resealed publication wire")
}

fn wire(fixture: &Fixture) -> TestPublicationWire {
    let bytes = fixture
        .retained
        .canonical_bytes(&fixture.configuration)
        .expect("canonical support");
    serde_json::from_slice(&bytes).expect("publication wire")
}

fn fixture() -> Fixture {
    let configuration = configuration();
    let beginning = beginning_owner(&configuration);
    let receipts = phase_receipts(&configuration, &beginning);
    let resource = prepare_surface_liquid_resource_candidate_v2(
        &configuration,
        &beginning,
        &beginning,
        TRANSACTION,
        &zero_closure(&beginning),
    )
    .expect("resource candidate");
    let ingress = ingress(&configuration);
    let candidate = execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding(
        &configuration,
        &resource,
        &ingress,
        None,
        false,
        None,
    )
    .expect("accepted ingress");
    let parent_bytes = candidate
        .parent_working_state()
        .expect("open parent")
        .restart_bytes(&configuration)
        .expect("WB14 parent bytes");
    let parent_value: serde_json::Value =
        serde_json::from_slice(&parent_bytes).expect("WB14 parent frame");
    let liquid_bytes: Vec<u8> =
        serde_json::from_value(parent_value["liquid_arithmetic_bytes"].clone())
            .expect("liquid bytes");
    let liquid: serde_json::Value = serde_json::from_slice(&liquid_bytes).expect("liquid frame");
    let parent_end_ns = u128::from(
        liquid["parent_support_end_ns"]
            .as_u64()
            .expect("parent end"),
    );
    let (soil_owner, soil_restart) = soil_owner_and_restart();
    let projection = SurfaceLiquidCompleteOwnerProjectionV3::new(
        &configuration,
        SurfaceLiquidCompleteOwnerProjectionIdentityV3 {
            run_id: 71,
            transaction_id: TRANSACTION,
            soil_thermal_run_id: soil_owner.run_id.clone(),
            soil_thermal_transaction_id: soil_owner.transaction_id,
            predecessor_transaction_id: None,
            soil_thermal_predecessor_transaction_id: soil_owner.expected_predecessor_transaction_id,
            parent_support_start_ns: 0,
            parent_support_end_ns: parent_end_ns,
            support_start_ns: 0,
            support_end_ns: SUPPORT_END_NS,
            beginning_surface_owner_sha256: beginning.envelope_sha256().into(),
            phase_adjusted_surface_owner_sha256: beginning.envelope_sha256().into(),
            predecessor_receipt_chain_sha256: digest('b'),
            receipt_chain_sha256: digest('0'),
        },
        candidate.ending_owner(),
        &beginning,
        Some(&parent_bytes),
        &receipts,
        candidate.inner().receipts(),
        &soil_owner,
        &soil_restart,
    )
    .expect("complete owner projection");
    let retained =
        FrozenLitterV3PublicationSupportV1::try_new(&configuration, &projection, &receipts)
            .expect("retained publication support");
    Fixture {
        configuration,
        receipts,
        retained,
    }
}

fn resident_fixture_with_runtime_surface_depths(
    use_runtime_surface_depths: bool,
) -> FrozenLitterV3Resident {
    let endpoint = crate::land_surface_energy_shadow::endpoint_fixture();
    let mut v2_configuration = endpoint.lse_configuration.clone();
    v2_configuration.model_version = V2_MODEL_VERSION.into();
    v2_configuration.model_definition_sha256 =
        Sha256Digest::try_new(V2_MODEL_DEFINITION_SHA256).expect("V2 digest");
    v2_configuration.vegetation_configuration.model_version = V2_VEGETATION_MODEL_VERSION.into();
    v2_configuration
        .vegetation_configuration
        .model_definition_sha256 =
        Sha256Digest::try_new(V2_VEGETATION_MODEL_DEFINITION_SHA256).expect("V2 vegetation digest");
    v2_configuration.configuration_sha256 = v2_configuration
        .canonical_sha256()
        .expect("V2 configuration digest");
    v2_configuration.validate_v2().expect("V2 configuration");
    let v2_state = project_validated_v1_runtime_to_v2(
        &endpoint.lse_configuration,
        &endpoint.lse_state,
        &v2_configuration,
        &v2_configuration
            .vegetation_configuration
            .configuration_sha256,
    )
    .expect("V2 state");
    let base_configuration =
        migrate_v2_configuration_to_v3(&v2_configuration).expect("V3 configuration");
    let base_state = migrate_v2_state_to_v3(&v2_configuration, &v2_state, &base_configuration)
        .expect("V3 state");

    let base_ofe = base_configuration.ofes[0].clone();
    let base_tile = base_ofe
        .tiles
        .iter()
        .find(|tile| matches!(tile.surface, SurfaceConfiguration::ForestLitter { .. }))
        .expect("forest tile")
        .clone();
    let mut lse_configuration = base_configuration.clone();
    let mut ofe_configuration = base_ofe;
    ofe_configuration.ofe_id = ofe();
    ofe_configuration.tiles = ["forest-a", "forest-b"]
        .into_iter()
        .map(|name| {
            let mut tile = base_tile.clone();
            tile.tile_id = TileId::try_new(name).expect("tile");
            tile.fraction_ofe_ground = 0.5;
            tile
        })
        .collect();
    lse_configuration.ofes = vec![ofe_configuration];
    lse_configuration.configuration_sha256 = lse_configuration
        .canonical_sha256()
        .expect("custom V3 configuration digest");
    lse_configuration
        .validate_v3()
        .expect("custom V3 configuration");

    let base_tile_state = base_state
        .0
        .tiles
        .iter()
        .find(|tile| tile.tile_id.as_str() == base_tile.tile_id.as_str())
        .expect("forest tile state")
        .clone();
    let mut lse_state = base_state;
    lse_state.0.configuration_sha256 = lse_configuration.configuration_sha256.clone();
    lse_state.0.last_accepted_transaction_id = None;
    lse_state.0.tiles = ["forest-a", "forest-b"]
        .into_iter()
        .map(|name| {
            let mut tile = base_tile_state.clone();
            tile.ofe_id = ofe();
            tile.tile_id = TileId::try_new(name).expect("tile");
            tile.surface_temperature_warm_start_k = 273.15;
            tile.surface_enthalpy_j_m2_tile_ground = 0.0;
            tile
        })
        .collect();
    lse_state.0.state_sha256 = lse_state
        .canonical_sha256()
        .expect("custom V3 state digest");
    lse_state
        .validate(&lse_configuration)
        .expect("custom V3 state");

    let surface_configuration = if use_runtime_surface_depths {
        let parent = configuration().parent().clone();
        let litter_depths = parent
            .records
            .iter()
            .map(|record| {
                let thickness_m = lse_configuration
                    .ofes
                    .iter()
                    .find(|ofe| ofe.ofe_id == record.key.ofe_id)
                    .and_then(|ofe| {
                        ofe.tiles
                            .iter()
                            .find(|tile| tile.tile_id == record.key.tile_id)
                    })
                    .and_then(|tile| match tile.surface {
                        SurfaceConfiguration::ForestLitter { thickness_m, .. } => Some(thickness_m),
                        SurfaceConfiguration::BareMineralSoil { .. } => None,
                    })
                    .expect("runtime litter depth");
                (record.key.clone(), thickness_m)
            })
            .collect();
        SurfaceLiquidConfigurationV2::new(
            parent,
            SurfaceLiquidOwnerModelDefinitionV2::new(digest('1'), digest('2'), digest('3'))
                .expect("surface V2 model"),
            &litter_depths,
        )
        .expect("runtime surface V2 configuration")
    } else {
        configuration()
    };
    let surface_owner = beginning_owner(&surface_configuration);
    FrozenLitterV3Resident::try_new(
        lse_configuration,
        lse_state,
        surface_configuration,
        surface_owner,
    )
    .expect("frozen-litter V3 resident")
}

fn resident_fixture() -> FrozenLitterV3Resident {
    resident_fixture_with_runtime_surface_depths(false)
}

fn runtime_resident_fixture() -> FrozenLitterV3Resident {
    resident_fixture_with_runtime_surface_depths(true)
}

fn v4_resident_fixture() -> (FrozenLitterV3Resident, FrozenLitterV4Resident) {
    let physical = runtime_resident_fixture();
    let exact = crate::LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("frozen-litter-v4-cow-test").expect("exact owner"),
        physical.lse_configuration(),
        physical.lse_state(),
        physical.surface_configuration(),
        physical.surface_owner(),
    )
    .expect("exact V4 owner");
    let exact = FrozenLitterV4Resident::try_new(&physical, exact).expect("V4 resident");
    (physical, exact)
}

fn runtime_phase_inputs(
    configuration: &SurfaceLiquidConfigurationV2,
    owner: &SurfaceLiquidOwnerEnvelopeV2,
    lse_configuration: &openwepp_land_surface_energy::LandSurfaceEnergyConfiguration,
    lse_state: &openwepp_land_surface_energy::LandSurfaceEnergyV3State,
    support_s: f64,
) -> Vec<FrozenLitterV3PhaseFreeInput> {
    configuration
        .records()
        .iter()
        .map(|configured| {
            let state = owner
                .v2_state()
                .expect("V2 state")
                .records()
                .iter()
                .find(|record| record.key == configured.key)
                .expect("configured state");
            let lse_configured = lse_configuration
                .ofes
                .iter()
                .find(|ofe| ofe.ofe_id == configured.key.ofe_id)
                .and_then(|ofe| {
                    ofe.tiles
                        .iter()
                        .find(|tile| tile.tile_id == configured.key.tile_id)
                })
                .expect("LSE configured tile");
            let SurfaceConfiguration::ForestLitter {
                liquid_capacity_kg_m2_tile_ground,
                thickness_m,
                dry_density_kg_m3,
                dry_specific_heat_j_kg_k,
            } = &lse_configured.surface
            else {
                panic!("LSE forest-litter tile");
            };
            let lse_tile = lse_state
                .0
                .tiles
                .iter()
                .find(|tile| {
                    tile.ofe_id == configured.key.ofe_id && tile.tile_id == configured.key.tile_id
                })
                .expect("LSE state tile");
            let phase_configuration = LitterPhaseConfiguration {
                litter_depth_m: *thickness_m,
                dry_heat_capacity_j_m2_k: *thickness_m
                    * *dry_density_kg_m3
                    * *dry_specific_heat_j_kg_k,
                liquid_capacity_kg_m2_tile: *liquid_capacity_kg_m2_tile_ground,
                ice_capacity_kg_m2_tile: configured
                    .litter_ice_capacity_kg_m2_tile
                    .expect("ice capacity"),
            };
            let beginning = BeginningLitterPhaseState {
                liquid_kg_m2_tile: state.liquid_kg_m2_tile,
                ice_kg_m2_tile: state.litter_ice_kg_m2_tile,
                sensible_energy_j_m2_tile: state.surface_enthalpy_j_m2_tile,
                temperature_k: lse_tile.surface_temperature_warm_start_k,
            };
            let accepted_temperature_k = lse_tile.surface_temperature_warm_start_k;
            let humidity =
                saturation_specific_humidity(accepted_temperature_k, 93_000.0).expect("humidity");
            let environment = LitterVaporEnvironment {
                accepted_phase_free_temperature_k: accepted_temperature_k,
                air_density_kg_m3: 1.1,
                air_pressure_pa: 93_000.0,
                recipient_specific_humidity_kg_kg: humidity,
                litter_to_canopy_resistance_s_m: 80.0,
            };
            let raw = evaluate_raw_litter_vapor(phase_configuration, beginning, environment)
                .expect("raw vapor");
            let finalized = FinalizedLitterVapor {
                liquid_signed_rate_kg_m2_s: 0.0,
                ice_signed_rate_kg_m2_s: 0.0,
            };
            let vapor =
                finalize_litter_vapor(raw, finalized, beginning, accepted_temperature_k, support_s)
                    .expect("final vapor");
            let post_vapor = install_finalized_vapor(
                phase_configuration,
                beginning,
                accepted_temperature_k,
                vapor,
            )
            .expect("post vapor");
            FrozenLitterV3PhaseFreeInput::from_authority_operands_for_test(
                configured.key.ofe_id.clone(),
                configured.key.tile_id.clone(),
                phase_configuration,
                beginning,
                vapor,
                post_vapor,
                V3PhaseFreeSurfaceEnergyLedger {
                    beginning_sensible_energy_j_m2: beginning.sensible_energy_j_m2_tile,
                    ending_sensible_energy_j_m2: post_vapor.sensible_energy_j_m2_tile,
                    absorbed_shortwave_w_m2: 0.0,
                    net_longwave_w_m2: 0.0,
                    sensible_to_canopy_air_w_m2: 0.0,
                    liquid_vapor_energy_w_m2: 0.0,
                    ice_vapor_energy_w_m2: 0.0,
                    ground_heat_w_m2: 0.0,
                    storage_w_m2: 0.0,
                    reconstructed_energy_residual_w_m2: 0.0,
                },
            )
        })
        .collect()
}

fn runtime_ingress(
    configuration: &SurfaceLiquidConfigurationV2,
    transaction_id: TransactionId,
    support_start_ns: u128,
    support_end_ns: u128,
    interval_index: u8,
) -> DirectSurfaceLiquidIngressInput {
    let start_s = 0.0;
    let end_s = (support_end_ns - support_start_ns) as f64 / 1_000_000_000.0;
    let amount = |mass| DirectIngressAmount {
        mass_kg_m2_tile_ground: mass,
        temperature_k: 273.15,
        specific_liquid_enthalpy_j_kg: 0.0,
        start_s,
        end_s,
    };
    DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 0,
        interval_index,
        interval_s: end_s - start_s,
        tile_ingress: configuration
            .parent()
            .records
            .iter()
            .map(|record| DirectTileGroundIngress::CoveredCanopyRelease {
                ofe_id: record.key.ofe_id.clone(),
                tile_id: record.key.tile_id.clone(),
                surface_id: record.key.surface_id.clone(),
                release: DirectCanopyLiquidRelease {
                    throughfall: amount(0.02),
                    initial_drainage: amount(0.0),
                    second_drainage: amount(0.0),
                    stemflow: amount(0.0),
                },
            })
            .collect(),
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: configuration.parent().ofe_topology[0].clone(),
            effective_conductivity_m_s: 1.0e-12,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.2,
        }],
    }
}

fn runtime_binding(
    support_start_ns: u128,
    support_end_ns: u128,
) -> DirectWb14CoupledChildBindingV1 {
    DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: ((support_end_ns - support_start_ns) as f64 / 1_000_000_000.0)
            .to_bits(),
        coupled_parent_transaction_sha256: [1; 32],
        accepted_slab_sha256: [2; 32],
        parent_beginning_complete_owner_set_sha256: [3; 32],
        parent_support_start_ns: support_start_ns,
        parent_support_end_ns: support_end_ns,
        child_support_start_ns: support_start_ns,
        child_support_end_ns: support_end_ns,
    }
}

fn execute_runtime_v3_candidate(
    resident: &FrozenLitterV3Resident,
    transaction_id: TransactionId,
    support_start_ns: u128,
    support_end_ns: u128,
) -> AcceptedFrozenLitterV3RuntimeCandidate {
    let support_s = (support_end_ns - support_start_ns) as f64 / 1_000_000_000.0;
    let phase_inputs = runtime_phase_inputs(
        resident.surface_configuration(),
        resident.surface_owner(),
        resident.lse_configuration(),
        resident.lse_state(),
        support_s,
    );
    let ingress = runtime_ingress(
        resident.surface_configuration(),
        transaction_id,
        support_start_ns,
        support_end_ns,
        u8::try_from(transaction_id.0 - TRANSACTION.0).expect("interval index"),
    );
    let (soil_owner, soil_restart) =
        soil_owner_and_restart_at(transaction_id, support_start_ns, support_end_ns);
    execute_frozen_litter_v3(&FrozenLitterV3RuntimeInput {
        transaction_id,
        soil_transaction_authority: PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            transaction_id,
            soil_owner.transaction_id,
        )
        .expect("soil transaction authority"),
        predecessor_transaction_id: resident.lse_state().0.last_accepted_transaction_id,
        parent_support_start_ns: support_start_ns,
        parent_support_end_ns: support_end_ns,
        support_start_ns,
        support_end_ns,
        predecessor_receipt_chain_sha256: resident.predecessor_receipt_chain_sha256().to_owned(),
        surface_configuration: resident.surface_configuration(),
        beginning_surface_owner: resident.surface_owner(),
        lse_configuration: resident.lse_configuration(),
        beginning_lse_state: resident.lse_state(),
        phase_inputs: &phase_inputs,
        current_ingress: &ingress,
        wb14_parent: resident.wb14_parent(),
        finalize_wb14_parent_interval: true,
        coupled_binding: runtime_binding(support_start_ns, support_end_ns),
        soil_beginning: FrozenLitterV3SoilBeginningV1::PublishableOwner {
            owner: &soil_owner,
            restart: &soil_restart,
        },
    })
    .expect("accepted V3 runtime candidate")
}

fn execute_runtime_v4_candidate(
    physical: &FrozenLitterV3Resident,
    exact: &FrozenLitterV4Resident,
    transaction_id: TransactionId,
    support_start_ns: u128,
    support_end_ns: u128,
) -> AcceptedFrozenLitterV4RuntimeCandidate {
    let support_s = (support_end_ns - support_start_ns) as f64 / 1_000_000_000.0;
    let phase_inputs = runtime_phase_inputs(
        physical.surface_configuration(),
        physical.surface_owner(),
        physical.lse_configuration(),
        physical.lse_state(),
        support_s,
    );
    let ingress = runtime_ingress(
        physical.surface_configuration(),
        transaction_id,
        support_start_ns,
        support_end_ns,
        u8::try_from(transaction_id.0 - TRANSACTION.0).expect("interval index"),
    );
    let (soil_owner, soil_restart) =
        soil_owner_and_restart_at(transaction_id, support_start_ns, support_end_ns);
    execute_frozen_litter_v4(&FrozenLitterV4RuntimeInput {
        physical: FrozenLitterV3RuntimeInput {
            transaction_id,
            soil_transaction_authority: PhysicalSoilEnergyTransactionAuthorityV2::try_new(
                transaction_id,
                soil_owner.transaction_id,
            )
            .expect("soil transaction authority"),
            predecessor_transaction_id: physical.lse_state().0.last_accepted_transaction_id,
            parent_support_start_ns: support_start_ns,
            parent_support_end_ns: support_end_ns,
            support_start_ns,
            support_end_ns,
            predecessor_receipt_chain_sha256: physical
                .predecessor_receipt_chain_sha256()
                .to_owned(),
            surface_configuration: physical.surface_configuration(),
            beginning_surface_owner: physical.surface_owner(),
            lse_configuration: physical.lse_configuration(),
            beginning_lse_state: physical.lse_state(),
            phase_inputs: &phase_inputs,
            current_ingress: &ingress,
            wb14_parent: physical.wb14_parent(),
            finalize_wb14_parent_interval: true,
            coupled_binding: runtime_binding(support_start_ns, support_end_ns),
            soil_beginning: FrozenLitterV3SoilBeginningV1::PublishableOwner {
                owner: &soil_owner,
                restart: &soil_restart,
            },
        },
        beginning_exact_surface_owner: exact.exact_surface_owner(),
    })
    .expect("accepted V4 runtime candidate")
}

fn v3_resident_canonical_bytes(resident: &FrozenLitterV3Resident) -> Vec<u8> {
    fn push(out: &mut Vec<u8>, bytes: &[u8]) {
        out.extend_from_slice(&(bytes.len() as u64).to_be_bytes());
        out.extend_from_slice(bytes);
    }
    let mut bytes = Vec::new();
    push(
        &mut bytes,
        &serde_json::to_vec(resident.lse_state()).expect("LSE bytes"),
    );
    push(
        &mut bytes,
        &resident
            .surface_owner()
            .canonical_bytes(
                resident.surface_configuration().parent(),
                Some(resident.surface_configuration()),
            )
            .expect("surface bytes"),
    );
    push(
        &mut bytes,
        resident.predecessor_receipt_chain_sha256().as_bytes(),
    );
    for publication in resident
        .accepted_publication_supports_canonical_bytes()
        .expect("publication bytes")
    {
        push(&mut bytes, &publication);
    }
    if let Some(parent) = resident
        .restart_wb14_parent_working_state_bytes()
        .expect("WB14 bytes")
    {
        push(&mut bytes, &parent);
    }
    bytes
}

pub(super) fn native_v4_resident_pair_fixture(
    shadow: &super::DirectV10RealConsumerShadow,
) -> (
    FrozenLitterV3Resident,
    crate::LseSurfaceEnthalpyOwnerEnvelopeV1,
) {
    let lse_configuration =
        migrate_v2_configuration_to_v3(&shadow.lse_configuration).expect("native V3 configuration");
    let lse_state = migrate_v2_state_to_v3(
        &shadow.lse_configuration,
        &shadow.lse_state,
        &lse_configuration,
    )
    .expect("native V3 state");

    let litter_depths = shadow
        .inner
        .surface_configuration
        .records
        .iter()
        .filter(|record| {
            record.key.surface_class == SurfaceClass::ForestLitter
                && record.key.source_type == WaterSourceType::LitterLiquid
        })
        .map(|record| {
            let thickness_m = lse_configuration
                .ofes
                .iter()
                .find(|ofe| ofe.ofe_id == record.key.ofe_id)
                .and_then(|ofe| {
                    ofe.tiles
                        .iter()
                        .find(|tile| tile.tile_id == record.key.tile_id)
                })
                .and_then(|tile| match tile.surface {
                    SurfaceConfiguration::ForestLitter { thickness_m, .. } => Some(thickness_m),
                    SurfaceConfiguration::BareMineralSoil { .. } => None,
                })
                .expect("native litter depth topology");
            (record.key.clone(), thickness_m)
        })
        .collect();
    let surface_configuration = SurfaceLiquidConfigurationV2::new(
        shadow.inner.surface_configuration.clone(),
        SurfaceLiquidOwnerModelDefinitionV2::new(
            typed_digest('1').to_string(),
            typed_digest('2').to_string(),
            shadow
                .inner
                .surface_configuration
                .configuration_sha256
                .clone(),
        )
        .expect("native surface V2 model"),
        &litter_depths,
    )
    .expect("native surface V2 configuration");
    let surface_enthalpy = surface_configuration
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
    let surface_state = crate::migrate_v1_to_v2(
        &surface_configuration,
        shadow
            .inner
            .hydrology_frame
            .surface_liquid_shadow
            .as_deref()
            .expect("native surface V1 owner"),
        &surface_enthalpy,
    )
    .expect("native surface V2 state");
    let surface_owner =
        SurfaceLiquidOwnerEnvelopeV2::wrap_v2(&surface_configuration, surface_state)
            .expect("native surface V2 owner");
    let physical = FrozenLitterV3Resident::try_new(
        lse_configuration,
        lse_state,
        surface_configuration,
        surface_owner,
    )
    .expect("topology-driven native V3 resident");
    let exact = crate::LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        ResourceOwnerId::try_new("canonical-covered-native-exact").expect("exact owner"),
        physical.lse_configuration(),
        physical.lse_state(),
        physical.surface_configuration(),
        physical.surface_owner(),
    )
    .expect("native V4 exact resident");
    (physical, exact)
}

pub(super) fn migrate_shadow_to_native_v2_soil(
    v1_shadow: &super::DirectV10RealConsumerShadow,
    support_end_ns: u128,
) -> super::DirectV10RealConsumerShadow {
    let current_transaction = TransactionId(v1_shadow.vegetation_state.0.last_transaction_id);
    let support_transaction = TransactionId(current_transaction.0 + 1);
    let migrated = migrate_soil_thermal_v1_to_v2(
        v1_shadow.inner.soil_thermal.v1().expect("V1 soil resident"),
        SoilThermalV2MigrationIdentity {
            model_version: v1_shadow
                .inner
                .lse_configuration
                .soil_thermal_configuration
                .model_version
                .clone(),
            model_definition_sha256: v1_shadow
                .inner
                .lse_configuration
                .soil_thermal_configuration
                .model_definition_sha256
                .clone(),
            run_id: "canonical-covered-native-v2".to_owned(),
            transaction_id: support_transaction,
            support_start_ns: 0,
            support_end_ns,
            receipt_chain_sha256: typed_digest('c'),
        },
    )
    .expect("migrate canonical covered native V2 soil");
    let prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
        &migrated,
        support_transaction,
        0,
        support_end_ns,
    )
    .expect("prepare canonical covered native V2 soil");
    let seals = openwepp_land_surface_energy::seal_soil_thermal_receipt_free_owner_v2(&prepared)
        .expect("seal canonical covered native V2 soil");
    super::DirectV10RealConsumerShadow::try_new_v2(
        v1_shadow.vegetation_configuration.clone(),
        v1_shadow.vegetation_state.clone(),
        v1_shadow.inner.vegetation_owner_id.clone(),
        v1_shadow.lse_configuration.clone(),
        v1_shadow.lse_state.clone(),
        v1_shadow.inner.surface_configuration.clone(),
        v1_shadow.inner.layer_maps.clone(),
        prepared,
        seals,
        v1_shadow.inner.biogeochemistry.clone(),
        v1_shadow.inner.hydrology_frame.clone(),
        v1_shadow.inner.next_day_index,
        v1_shadow.gsi_owner_configuration.clone(),
        v1_shadow.gsi_state.clone(),
        v1_shadow.provider_static_configuration.clone(),
        v1_shadow.provider_cursor.clone(),
        v1_shadow.root_zone_hydraulic_configuration.clone(),
    )
    .expect("construct canonical covered native V2 shadow")
}

#[test]
fn canonical_roundtrip_restores_projection_and_ordered_model_receipts() {
    let fixture = fixture();
    let bytes = fixture
        .retained
        .canonical_bytes(&fixture.configuration)
        .expect("canonical support");
    let replay =
        FrozenLitterV3PublicationSupportV1::from_canonical_bytes(&fixture.configuration, &bytes)
            .expect("support replay");
    let wire_text = std::str::from_utf8(&bytes).expect("UTF-8 canonical wire");
    assert!(!wire_text.contains("diagnostic"));
    assert!(!wire_text.contains("rollback"));
    assert_eq!(replay, fixture.retained);
    assert_eq!(
        replay
            .canonical_bytes(&fixture.configuration)
            .expect("canonical replay"),
        bytes
    );
    assert_ne!(replay.publication_sha256(), &Digest32::zero());
    assert_eq!(
        replay
            .ordered_litter_phase_receipts(&fixture.configuration)
            .expect("receipt replay"),
        fixture.receipts
    );
    let projection = replay
        .complete_owner_projection(&fixture.configuration)
        .expect("projection replay");
    let wire = wire(&fixture);
    assert_eq!(projection.projection_sha256(), wire.projection_sha256);
    assert_eq!(
        projection.identity().receipt_chain_sha256,
        wire.projection_receipt_chain_sha256
    );
}

#[test]
fn omission_reorder_and_model_substitution_fail_even_after_reseal() {
    let fixture = fixture();

    let mut omitted = wire(&fixture);
    omitted.ordered_litter_phase_receipts.pop();
    assert!(
        FrozenLitterV3PublicationSupportV1::from_canonical_bytes(
            &fixture.configuration,
            &reseal(omitted),
        )
        .is_err()
    );

    let mut reordered = wire(&fixture);
    reordered.ordered_litter_phase_receipts.swap(0, 1);
    for (ordinal, frame) in reordered
        .ordered_litter_phase_receipts
        .iter_mut()
        .enumerate()
    {
        frame.ordinal = u32::try_from(ordinal).expect("ordinal");
    }
    assert!(
        FrozenLitterV3PublicationSupportV1::from_canonical_bytes(
            &fixture.configuration,
            &reseal(reordered),
        )
        .is_err()
    );

    let mut substituted = wire(&fixture);
    substituted.ordered_litter_phase_receipts[0].model_definition_sha256 = digest('f');
    assert!(
        FrozenLitterV3PublicationSupportV1::from_canonical_bytes(
            &fixture.configuration,
            &reseal(substituted),
        )
        .is_err()
    );
}

#[test]
fn nested_projection_and_receipt_chain_substitution_fail_closed() {
    let fixture = fixture();
    let mut projection = wire(&fixture);
    projection.canonical_projection_bytes[0] ^= 1;
    assert!(
        FrozenLitterV3PublicationSupportV1::from_canonical_bytes(
            &fixture.configuration,
            &reseal(projection),
        )
        .is_err()
    );

    let mut chain = wire(&fixture);
    chain.projection_receipt_chain_sha256 = digest('e');
    assert!(
        FrozenLitterV3PublicationSupportV1::from_canonical_bytes(
            &fixture.configuration,
            &reseal(chain),
        )
        .is_err()
    );
}

#[test]
fn install_frozen_litter_v3_resident_moves_validated_history_without_roundtrip() {
    let fixture = fixture();
    let bytes = fixture
        .retained
        .canonical_bytes(&fixture.configuration)
        .expect("accepted publication bytes");
    let receipt_chain = fixture
        .retained
        .complete_owner_projection(&fixture.configuration)
        .expect("accepted projection")
        .identity()
        .receipt_chain_sha256
        .clone();
    let mut resident = resident_fixture();
    resident
        .restore_restart_authority(&[bytes.clone()], None, &receipt_chain)
        .expect("full restart replay before in-memory handoff");
    reset_frozen_litter_v3_handoff_counters_for_test();
    let installed = resident
        .validated_handoff_for_test()
        .expect("validated in-memory handoff");
    assert_eq!(frozen_litter_v3_handoff_counters_for_test(), (0, 0, 1));
    assert_eq!(
        installed
            .accepted_publication_supports_canonical_bytes()
            .expect("installed publication bytes"),
        vec![bytes],
    );
}

#[test]
fn validated_resident_append_checks_only_new_tail_and_preserves_prefix() {
    let fixture = fixture();
    let projection = fixture
        .retained
        .complete_owner_projection(&fixture.configuration)
        .expect("accepted projection");
    let mut resident = resident_fixture();
    resident
        .restore_restart_authority(&[], None, &digest('b'))
        .expect("align predecessor authority");
    let prefix = resident
        .accepted_publication_supports_canonical_bytes()
        .expect("prefix bytes");
    reset_frozen_litter_v3_handoff_counters_for_test();
    resident
        .validate_new_publication_tail_for_test(&projection, &fixture.receipts)
        .expect("new tail validation");
    assert_eq!(frozen_litter_v3_handoff_counters_for_test(), (0, 1, 0));
    assert_eq!(
        resident
            .accepted_publication_supports_canonical_bytes()
            .expect("unchanged prefix bytes"),
        prefix,
    );
}

#[test]
fn frozen_litter_restart_revalidates_complete_publication_history() {
    let fixture = fixture();
    let bytes = fixture
        .retained
        .canonical_bytes(&fixture.configuration)
        .expect("accepted publication bytes");
    let receipt_chain = fixture
        .retained
        .complete_owner_projection(&fixture.configuration)
        .expect("accepted projection")
        .identity()
        .receipt_chain_sha256
        .clone();
    let mut resident = resident_fixture();
    reset_frozen_litter_v3_handoff_counters_for_test();
    resident
        .restore_restart_authority(&[bytes.clone()], None, &receipt_chain)
        .expect("full restart replay");
    assert_eq!(frozen_litter_v3_handoff_counters_for_test(), (1, 0, 0));
    assert_eq!(
        resident
            .accepted_publication_supports_canonical_bytes()
            .expect("restored publication bytes"),
        vec![bytes],
    );
}

#[test]
fn validated_resident_tail_poison_rolls_back() {
    let fixture = fixture();
    let projection = fixture
        .retained
        .complete_owner_projection(&fixture.configuration)
        .expect("accepted projection");
    let mut resident = resident_fixture();
    resident
        .restore_restart_authority(&[], None, &digest('b'))
        .expect("align predecessor authority");
    let beginning_lse = resident.lse_state().clone();
    let beginning_surface = resident.surface_owner().clone();
    let beginning_history = resident
        .accepted_publication_supports_canonical_bytes()
        .expect("beginning history");
    resident.corrupt_validated_tail_for_test();
    assert!(
        resident
            .validate_new_publication_tail_for_test(&projection, &fixture.receipts)
            .is_err()
    );
    assert_eq!(resident.lse_state(), &beginning_lse);
    assert_eq!(resident.surface_owner(), &beginning_surface);
    assert_eq!(
        resident
            .accepted_publication_supports_canonical_bytes()
            .expect("rolled-back history"),
        beginning_history,
    );
}

#[test]
fn validated_resident_install_cost_is_history_length_independent() {
    let empty = resident_fixture();
    reset_frozen_litter_v3_handoff_counters_for_test();
    empty
        .validated_handoff_for_test()
        .expect("empty-history handoff");
    assert_eq!(frozen_litter_v3_handoff_counters_for_test(), (0, 0, 1));

    let fixture = fixture();
    let bytes = fixture
        .retained
        .canonical_bytes(&fixture.configuration)
        .expect("accepted publication bytes");
    let receipt_chain = fixture
        .retained
        .complete_owner_projection(&fixture.configuration)
        .expect("accepted projection")
        .identity()
        .receipt_chain_sha256
        .clone();
    let mut one = resident_fixture();
    one.restore_restart_authority(&[bytes], None, &receipt_chain)
        .expect("one-history restart replay");
    reset_frozen_litter_v3_handoff_counters_for_test();
    one.validated_handoff_for_test()
        .expect("one-history handoff");
    assert_eq!(frozen_litter_v3_handoff_counters_for_test(), (0, 0, 1));
}

#[test]
fn v3_shared_and_forced_deep_histories_match_production_acceptance_and_rejection() {
    let mut beginning = runtime_resident_fixture();
    beginning
        .restore_restart_authority(&[], None, &digest('b'))
        .expect("aligned V3 predecessor");
    let shared_anchor = beginning.clone();
    let mut shared = beginning.clone();
    let mut forced_deep = beginning.clone();
    forced_deep.force_deep_clone_publication_history_for_test();
    assert!(shared_anchor.publication_history_shares_allocation_with_for_test(&shared));
    assert!(!shared.publication_history_shares_allocation_with_for_test(&forced_deep));

    let shared_candidate =
        execute_runtime_v3_candidate(&shared, TRANSACTION, 0, RUNTIME_SUPPORT_NS);
    let forced_candidate =
        execute_runtime_v3_candidate(&forced_deep, TRANSACTION, 0, RUNTIME_SUPPORT_NS);
    shared
        .accept_runtime_candidate(&shared_candidate)
        .expect("shared production acceptance");
    forced_deep
        .accept_runtime_candidate(&forced_candidate)
        .expect("forced-deep production acceptance");

    assert!(!shared_anchor.publication_history_shares_allocation_with_for_test(&shared));
    assert!(
        shared
            .has_same_validated_physical_history(&forced_deep)
            .expect("accepted revision parity")
    );
    assert_eq!(
        v3_resident_canonical_bytes(&shared),
        v3_resident_canonical_bytes(&forced_deep),
    );
    shared
        .clone()
        .validated_handoff_for_test()
        .expect("shared accepted revision");
    forced_deep
        .clone()
        .validated_handoff_for_test()
        .expect("forced accepted revision");

    let shared_before_rejection = v3_resident_canonical_bytes(&shared);
    let forced_before_rejection = v3_resident_canonical_bytes(&forced_deep);
    let mut rejected_shared = execute_runtime_v3_candidate(
        &shared,
        TransactionId(TRANSACTION.0 + 1),
        RUNTIME_SUPPORT_NS,
        RUNTIME_SUPPORT_NS * 2,
    );
    let mut rejected_forced = execute_runtime_v3_candidate(
        &forced_deep,
        TransactionId(TRANSACTION.0 + 1),
        RUNTIME_SUPPORT_NS,
        RUNTIME_SUPPORT_NS * 2,
    );
    rejected_shared.ending_lse_state.0.configuration_sha256 = typed_digest('f');
    rejected_forced.ending_lse_state.0.configuration_sha256 = typed_digest('f');
    assert!(shared.accept_runtime_candidate(&rejected_shared).is_err());
    assert!(
        forced_deep
            .accept_runtime_candidate(&rejected_forced)
            .is_err()
    );
    assert_eq!(
        v3_resident_canonical_bytes(&shared),
        shared_before_rejection
    );
    assert_eq!(
        v3_resident_canonical_bytes(&forced_deep),
        forced_before_rejection,
    );
    assert!(
        shared
            .has_same_validated_physical_history(&forced_deep)
            .expect("rejected revision parity")
    );
    shared
        .validated_handoff_for_test()
        .expect("shared rejected-flow revision");
    forced_deep
        .validated_handoff_for_test()
        .expect("forced rejected-flow revision");
}

#[test]
fn v4_nonempty_restore_then_shared_and_forced_deep_production_flows_match() {
    let (mut physical, mut exact) = v4_resident_fixture();
    physical
        .restore_restart_authority(&[], None, &digest('b'))
        .expect("aligned V4 physical predecessor");
    let history_beginning_lse_sha = physical.lse_state().0.state_sha256.clone();
    let first = execute_runtime_v4_candidate(&physical, &exact, TRANSACTION, 0, RUNTIME_SUPPORT_NS);
    let beginning_physical = physical.clone();
    physical
        .accept_runtime_candidate(&first.physical)
        .expect("first V3 acceptance");
    exact
        .accept_runtime_candidate(&beginning_physical, &first)
        .expect("first V4 acceptance");
    assert_eq!(
        exact.accepted_publication_supports_canonical_bytes().len(),
        1
    );

    let restored = FrozenLitterV4Resident::try_restore(
        &physical,
        exact.exact_surface_owner().clone(),
        exact.accepted_publication_supports_canonical_bytes(),
        &history_beginning_lse_sha,
    )
    .expect("nonempty validated V4 restore");
    assert_eq!(
        restored.accepted_publication_supports_canonical_bytes(),
        exact.accepted_publication_supports_canonical_bytes(),
    );
    let restored_bytes = restored
        .canonical_inactive_projection_v1()
        .expect("restored V4 bytes");

    let mut rejected_shared = restored.clone();
    let mut rejected_forced = restored.clone();
    rejected_forced.force_deep_clone_publication_history_for_test();
    let mut rejected_shared_candidate = execute_runtime_v4_candidate(
        &physical,
        &rejected_shared,
        TransactionId(TRANSACTION.0 + 1),
        RUNTIME_SUPPORT_NS,
        RUNTIME_SUPPORT_NS * 2,
    );
    let mut rejected_forced_candidate = execute_runtime_v4_candidate(
        &physical,
        &rejected_forced,
        TransactionId(TRANSACTION.0 + 1),
        RUNTIME_SUPPORT_NS,
        RUNTIME_SUPPORT_NS * 2,
    );
    rejected_shared_candidate
        .ending_exact_surface_owner
        .state_sha256 = typed_digest('f');
    rejected_forced_candidate
        .ending_exact_surface_owner
        .state_sha256 = typed_digest('f');
    assert!(
        rejected_shared
            .accept_runtime_candidate(&physical, &rejected_shared_candidate)
            .is_err()
    );
    assert!(
        rejected_forced
            .accept_runtime_candidate(&physical, &rejected_forced_candidate)
            .is_err()
    );
    assert_eq!(
        rejected_shared
            .canonical_inactive_projection_v1()
            .expect("shared rejected bytes"),
        restored_bytes,
    );
    assert_eq!(
        rejected_forced
            .canonical_inactive_projection_v1()
            .expect("forced rejected bytes"),
        restored_bytes,
    );

    let restored_anchor = restored.clone();
    let mut shared = restored.clone();
    let mut forced_deep = restored.clone();
    forced_deep.force_deep_clone_publication_history_for_test();
    assert!(restored_anchor.publication_history_shares_allocation_with_for_test(&shared));
    assert!(!shared.publication_history_shares_allocation_with_for_test(&forced_deep));
    let shared_candidate = execute_runtime_v4_candidate(
        &physical,
        &shared,
        TransactionId(TRANSACTION.0 + 1),
        RUNTIME_SUPPORT_NS,
        RUNTIME_SUPPORT_NS * 2,
    );
    let forced_candidate = execute_runtime_v4_candidate(
        &physical,
        &forced_deep,
        TransactionId(TRANSACTION.0 + 1),
        RUNTIME_SUPPORT_NS,
        RUNTIME_SUPPORT_NS * 2,
    );
    shared
        .accept_runtime_candidate(&physical, &shared_candidate)
        .expect("shared V4 acceptance");
    forced_deep
        .accept_runtime_candidate(&physical, &forced_candidate)
        .expect("forced V4 acceptance");
    assert!(!restored_anchor.publication_history_shares_allocation_with_for_test(&shared));
    assert_eq!(shared, forced_deep);
    assert_eq!(
        shared.accepted_publication_supports_canonical_bytes().len(),
        2
    );
    assert_eq!(
        shared
            .canonical_inactive_projection_v1()
            .expect("shared accepted bytes"),
        forced_deep
            .canonical_inactive_projection_v1()
            .expect("forced accepted bytes"),
    );

    let mut ending_physical = physical.clone();
    ending_physical
        .accept_runtime_candidate(&shared_candidate.physical)
        .expect("second V3 acceptance");
    let replay = FrozenLitterV4Resident::try_restore(
        &ending_physical,
        shared.exact_surface_owner().clone(),
        shared.accepted_publication_supports_canonical_bytes(),
        &history_beginning_lse_sha,
    )
    .expect("two-publication V4 replay");
    assert_eq!(replay, shared);
}
