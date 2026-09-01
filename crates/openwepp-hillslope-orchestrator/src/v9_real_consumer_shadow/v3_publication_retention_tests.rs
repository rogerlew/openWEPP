//! Focused canonical retention tests for accepted frozen-litter V3 supports.

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use openwepp_land_surface_energy::{
    BeginningLitterPhaseState, FinalizedLitterVapor, LitterPhaseConfiguration, LitterPhaseReceipt,
    LitterPhaseTransactionIdentity, LitterPhaseTransactionInput, OfeId, Sha256Digest,
    SoilThermalLayerSnapshot, SoilThermalOfeSnapshot, SoilThermalOwnerEnvelopeV2,
    SoilThermalOwnerRestartV2, SoilThermalSnapshot, SoilThermalV2MigrationIdentity, SourceId,
    SurfaceClass, SurfaceId, V3PhaseFreeSurfaceEnergyLedger, WaterSourceType,
    evaluate_raw_litter_vapor, execute_litter_phase_v3, migrate_soil_thermal_v1_to_v2,
    saturation_specific_humidity,
};

use crate::direct_runtime::{
    DirectCanopyLiquidRelease, DirectIngressAmount, DirectSurfaceLiquidIngressInput,
    DirectTileGroundIngress,
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

use super::{Digest32, FrozenLitterV3PublicationSupportV1, digest_bytes};

const TRANSACTION: TransactionId = TransactionId(703);
const SUPPORT_END_NS: u128 = 900_000_000_000;

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

fn soil_owner_and_restart() -> (SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2) {
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
            transaction_id: TRANSACTION,
            support_start_ns: 0,
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
