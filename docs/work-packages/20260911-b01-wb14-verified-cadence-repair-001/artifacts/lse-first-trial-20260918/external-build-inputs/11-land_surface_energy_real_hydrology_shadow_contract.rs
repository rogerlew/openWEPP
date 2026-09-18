use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::land_surface_energy_shadow::{
    BandDirectionalFluxes, BareSoilParameters, ComponentId, CondensationCredit, GroundWaterKey,
    LandSurfaceEnergyError, LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError,
    MixedRealHydrologyRequest, MixedRealHydrologyUse, OfeId, OpenNeutralGeometry,
    OpenPotentialPhase, OpenSurfaceProblem, OwnerKind, OwnerRollbackHash, RuntimeTileIdentity,
    Sha256Digest, SoilThermalLayerCandidate, SoilThermalLayerSnapshot, SoilThermalNodeOperands,
    SoilThermalOfeSnapshot, SoilThermalSnapshot, SoilThermalTileCandidate, SourceId,
    StandGroundWaterAmountBasis, SurfaceClass, SurfaceClassKind, SurfaceId, SurfaceStorageBranch,
    TileState, UnifiedLseFinalization, UnifiedReceiverExpectations, WaterAuthorization,
    WaterAuthorizationReason, WaterProtocol, WaterSourceType, WaterUseOperands,
    evaluate_open_surface, execute_open_bare_soil_shadow, execute_unified_real_hydrology_shadow,
    finalize_open_phase, solve_open_potential_phase, unified_beginning_hydrology_snapshot_sha256,
    validate_real_receiver_closure, validate_water_use,
};
use openwepp_hillslope_orchestrator::vegetation_real_hydrology_shadow::{
    RealHydrologyLaneLayerMap, RealHydrologyOfeLaneId, RealHydrologyShadowAdapter,
    RealHydrologySourceKey,
};
use openwepp_hillslope_orchestrator::{
    DirectCanopyLiquidRelease, DirectDayConstructorInputs, DirectFrostLaneState,
    DirectFrostRuntimeCarry, DirectGroundIngressMode, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectRunFrame, DirectRunIdentity, DirectSubsurfaceLayerState,
    DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidConfigurationRecord,
    DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidErrorContext,
    DirectSurfaceLiquidIngressInput, DirectSurfaceLiquidOfeBinding, DirectSurfaceLiquidOwnedState,
    DirectSurfaceLiquidPhase, DirectSurfaceLiquidRollbackHashes, DirectSurfaceLiquidStoreKey,
    DirectTileGroundIngress,
};
use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use sha2::{Digest, Sha256};

#[path = "land_surface_energy_real_hydrology_shadow_contract/raw_hash_tests.rs"]
mod raw_hash_tests;

#[path = "land_surface_energy_real_hydrology_shadow_contract/precedence_tests.rs"]
mod precedence_tests;

#[path = "land_surface_energy_real_hydrology_shadow_contract/snow_preflight_tests.rs"]
mod snow_preflight_tests;

#[path = "land_surface_energy_real_hydrology_shadow_contract/finalization_test_support.rs"]
mod finalization_test_support;
#[path = "land_surface_energy_real_hydrology_shadow_contract/sealed_receiver_context_tests.rs"]
mod sealed_receiver_context_tests;
use finalization_test_support::finalization_expectations;
#[path = "land_surface_energy_real_hydrology_shadow_contract/source_binding_tests.rs"]
mod source_binding_tests;
#[path = "land_surface_energy_real_hydrology_shadow_contract/unified_boundary_tests.rs"]
mod unified_boundary_tests;

fn production_frame(supply_m: f64, frozen: bool) -> DirectRunFrame {
    let identity = DirectRunIdentity::new(83, 11, 1, 1).expect("identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("frame");
    frame.lanes[0].area_m2 = 100.0;
    frame.lanes[0].subsurface_layers = vec![DirectSubsurfaceLayerState {
        theta_m: supply_m,
        field_capacity_m: 0.02,
        upper_limit_m: 0.2,
        conductivity_m_s: 1.0e-6,
        depth_m: 0.3,
        residual_theta: 0.0,
        frozen_depth_m: if frozen { 0.3 } else { 0.0 },
        frozen_water_m: if frozen { supply_m } else { 0.0 },
        porosity: 0.45,
        field_capacity_theta: 0.25,
        coca: 0.1,
        lateral_conductivity_m_s: 1.0e-7,
    }];
    frame.lanes[0].water.soil_water_m = supply_m;
    frame
}

fn owner(frame: &DirectRunFrame) -> (RealHydrologyShadowAdapter, RealHydrologySourceKey) {
    let ofe_lane = RealHydrologyOfeLaneId {
        lane_index: 0,
        lane_id: frame.lanes[0].lane_id,
    };
    let layer = SoilLayerId::try_new("thermal-1").expect("layer");
    let adapter = RealHydrologyShadowAdapter::try_from_day_start(
        frame,
        0,
        TransactionId(41),
        1_800.0,
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        &[RealHydrologyLaneLayerMap {
            ofe_lane,
            layer_ids: vec![layer.clone()],
        }],
    )
    .expect("adapter");
    (
        adapter,
        RealHydrologySourceKey {
            ofe_lane,
            layer_id: layer,
        },
    )
}

fn key(component: &str, layer: &str) -> GroundWaterKey {
    let root = component == "root";
    GroundWaterKey {
        transaction_id: TransactionId(41),
        requesting_owner_id: ResourceOwnerId::try_new(if root {
            "vegetation-v8"
        } else {
            "land-surface-energy-v1"
        })
        .expect("request owner"),
        requesting_component: if root {
            openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::VegetationRoot
        } else {
            openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::GroundSurface
        },
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        requesting_tile_id: TileId::try_new("open").expect("tile"),
        occupancy_id: root.then(|| ComponentId::try_new("canopy-rank-0").expect("occupancy")),
        surface_id: (!root).then(|| SurfaceId::try_new("surface:ofe-1:open").expect("surface")),
        surface_class: (!root).then_some(SurfaceClass::BareMineralSoil),
        source_type: WaterSourceType::SoilLayerLiquid,
        source_id: SourceId::try_new(format!("soil:ofe-1:{layer}")).expect("source"),
        source_tile_id: None,
        soil_layer_id: Some(SoilLayerId::try_new(layer).expect("layer")),
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
    }
}

fn request(
    component: &str,
    amount: f64,
    source: &RealHydrologySourceKey,
) -> MixedRealHydrologyRequest {
    MixedRealHydrologyRequest {
        request: openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
            key: key(component, "thermal-1"),
            amount_kg_m2_stand_ground: amount,
        },
        source: source.clone(),
    }
}

fn open_problem() -> OpenSurfaceProblem {
    OpenSurfaceProblem {
        interval_s: 1_800.0,
        tile_fraction: 1.0,
        class: SurfaceClassKind::BareMineralSoil,
        storage_branch: SurfaceStorageBranch::FiniteCapacity,
        terminal_shortwave_w_m2_tile: BandDirectionalFluxes {
            direct_vis: 91.0,
            diffuse_vis: 31.0,
            direct_nir: 117.0,
            diffuse_nir: 39.0,
        },
        surface_vis_albedo: 0.18,
        surface_nir_albedo: 0.31,
        surface_emissivity: 1.0,
        surface_depth_m: 0.02,
        surface_conductivity_w_m_k: 0.75,
        surface_dry_heat_capacity_j_m2_k: 42_000.0,
        litter_capacity_kg_m2_tile: None,
        open_geometry: OpenNeutralGeometry {
            reference_height_m: 20.0,
            roughness_momentum_m: 0.12,
            roughness_heat_m: 0.015,
            roughness_vapor_m: 0.010,
        },
        air_temperature_k: 294.0,
        air_specific_humidity_kg_kg: 0.0095,
        air_pressure_pa: 93_000.0,
        reference_wind_m_s: 2.4,
        atmospheric_downward_longwave_w_m2: 335.0,
        surface_liquid_kg_m2_tile: 0.0,
        surface_enthalpy_j_m2_tile: 42_000.0 * (295.0 - 273.15),
        surface_temperature_warm_start_k: 295.0,
        bare_soil: Some(BareSoilParameters {
            top_layer_liquid_kg_m2: 20.0,
            top_layer_ice_kg_m2: 0.0,
            porosity: 0.46,
            saturated_matric_potential_mm: -120.0,
            clapp_hornberger_b: 4.05,
            theta_initial: 0.22,
        }),
        soil_nodes: (0..4)
            .map(|index| SoilThermalNodeOperands {
                layer_id: format!("thermal-{}", index + 1),
                depth_m: 0.08 + 0.05 * f64::from(index),
                conductivity_w_m_k: 1.1 + 0.12 * f64::from(index),
                heat_capacity_j_m2_k: 120_000.0 + 35_000.0 * f64::from(index),
                beginning_temperature_k: 291.5 - 1.1 * f64::from(index),
            })
            .collect(),
    }
}

fn digest(byte: char) -> Sha256Digest {
    Sha256Digest::try_new(byte.to_string().repeat(64)).expect("digest")
}

fn surface_configuration(
    class: SurfaceClass,
    source_type: WaterSourceType,
) -> DirectSurfaceLiquidConfiguration {
    DirectSurfaceLiquidConfiguration::new(
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        83,
        vec![OfeId::try_new("ofe-1").expect("OFE")],
        vec![DirectSurfaceLiquidOfeBinding {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            production_lane_index: 0,
            production_lane_id: 1,
            ordered_soil_layer_ids: vec![SoilLayerId::try_new("thermal-1").expect("layer")],
            infiltration_soil_thermal_layer_id: SoilLayerId::try_new("thermal-1")
                .expect("thermal layer"),
        }],
        vec![DirectSurfaceLiquidConfigurationRecord {
            key: DirectSurfaceLiquidStoreKey {
                run_id: 83,
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                tile_id: TileId::try_new("open").expect("tile"),
                surface_id: SurfaceId::try_new("surface:ofe-1:open").expect("surface"),
                surface_class: class,
                source_type,
                source_id: SourceId::try_new("surface-store:ofe-1:open").expect("source"),
            },
            tile_fraction: 1.0,
            capacity_kg_m2_tile: 3.0,
            ofe_area_m2: 100.0,
            ground_ingress_mode: DirectGroundIngressMode::OpenRawPrecipitation,
            runon_destination_ofe_id: None,
            runon_destination_tile_id: None,
        }],
    )
    .expect("surface configuration")
}

fn configured_surface_frame(
    class: SurfaceClass,
    source_type: WaterSourceType,
    liquid: f64,
) -> (DirectRunFrame, DirectSurfaceLiquidConfiguration) {
    let configuration = surface_configuration(class, source_type);
    let initial = BTreeMap::from([(configuration.records[0].key.clone(), liquid)]);
    let state = DirectSurfaceLiquidOwnedState::new_initial(&configuration, &initial, 0)
        .expect("surface state");
    let mut frame = production_frame(0.02, false);
    frame
        .configure_surface_liquid_shadow(&configuration, state)
        .expect("configure surface owner");
    (frame, configuration)
}

fn configured_two_tile_surface_frame() -> (DirectRunFrame, DirectSurfaceLiquidConfiguration) {
    let mut open = surface_configuration(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
    )
    .records[0]
        .clone();
    open.tile_fraction = 0.4;
    let mut covered = open.clone();
    covered.key.tile_id = TileId::try_new("covered").expect("covered tile");
    covered.key.surface_id = SurfaceId::try_new("surface:ofe-1:covered").expect("covered surface");
    covered.key.surface_class = SurfaceClass::ForestLitter;
    covered.key.source_type = WaterSourceType::LitterLiquid;
    covered.key.source_id =
        SourceId::try_new("surface-store:ofe-1:covered").expect("covered source");
    covered.tile_fraction = 0.6;
    covered.ground_ingress_mode = DirectGroundIngressMode::CoveredCanopyRelease;
    let configuration = DirectSurfaceLiquidConfiguration::new(
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        83,
        vec![OfeId::try_new("ofe-1").expect("OFE")],
        vec![DirectSurfaceLiquidOfeBinding {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            production_lane_index: 0,
            production_lane_id: 1,
            ordered_soil_layer_ids: vec![SoilLayerId::try_new("thermal-1").expect("layer")],
            infiltration_soil_thermal_layer_id: SoilLayerId::try_new("thermal-1")
                .expect("thermal layer"),
        }],
        vec![open, covered],
    )
    .expect("two-tile surface configuration");
    let initial = configuration
        .records
        .iter()
        .map(|record| (record.key.clone(), 1.0))
        .collect();
    let state = DirectSurfaceLiquidOwnedState::new_initial(&configuration, &initial, 0)
        .expect("two-tile surface state");
    let mut frame = production_frame(0.02, false);
    frame
        .configure_surface_liquid_shadow(&configuration, state)
        .expect("configure two-tile surface owner");
    (frame, configuration)
}

fn open_surface_source_id(configuration: &DirectSurfaceLiquidConfiguration) -> SourceId {
    configuration
        .records
        .iter()
        .find(|record| record.key.tile_id.as_str() == "open")
        .expect("open surface configuration record")
        .key
        .source_id
        .clone()
}

fn surface_potential_batch(
    class: SurfaceClass,
    source_type: WaterSourceType,
    source_id: SourceId,
    liquid: f64,
) -> openwepp_hillslope_orchestrator::land_surface_energy_shadow::PotentialWaterRequestBatch {
    surface_potential_phase(class, source_type, source_id, liquid).request_batch
}

fn surface_potential_phase(
    class: SurfaceClass,
    source_type: WaterSourceType,
    source_id: SourceId,
    liquid: f64,
) -> OpenPotentialPhase {
    surface_potential_phase_with_snapshot(class, source_type, source_id, liquid, digest('3'))
}

fn surface_potential_phase_with_snapshot(
    class: SurfaceClass,
    source_type: WaterSourceType,
    source_id: SourceId,
    liquid: f64,
    beginning_hydrology_snapshot_sha256: Sha256Digest,
) -> OpenPotentialPhase {
    let mut problem = open_problem();
    problem.surface_liquid_kg_m2_tile = liquid;
    problem.class = match class {
        SurfaceClass::BareMineralSoil => SurfaceClassKind::BareMineralSoil,
        SurfaceClass::ForestLitter => SurfaceClassKind::ForestLitter,
    };
    if class == SurfaceClass::ForestLitter {
        problem.litter_capacity_kg_m2_tile = Some(3.0);
        problem.bare_soil = None;
    }
    solve_open_potential_phase(
        RuntimeTileIdentity {
            transaction_id: TransactionId(41),
            soil_thermal_transaction_id: TransactionId(41),
            lse_owner_id: ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
            hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology")
                .expect("hydrology owner"),
            soil_thermal_owner_id: ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
            vegetation_owner_id: ResourceOwnerId::try_new("vegetation-v8")
                .expect("vegetation owner"),
            biogeochemistry_owner_id: ResourceOwnerId::try_new("biogeochemistry")
                .expect("biogeochemistry owner"),
            configuration_sha256: digest('1'),
            beginning_lse_state_sha256: digest('2'),
            beginning_hydrology_snapshot_sha256,
            beginning_soil_thermal_state_sha256: digest('4'),
            beginning_vegetation_state_sha256: digest('5'),
            beginning_biogeochemistry_state_sha256: digest('6'),
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            tile_id: TileId::try_new("open").expect("tile"),
            surface_id: SurfaceId::try_new("surface:ofe-1:open").expect("surface"),
            surface_class: class,
            ground_source_type: source_type,
            ground_source_id: source_id,
            ground_source_tile_id: Some(TileId::try_new("open").expect("source tile")),
            ground_soil_layer_id: None,
            tile_fraction: 1.0,
            interval_s: 1_800.0,
        },
        &problem,
        None,
    )
    .expect("surface potential")
}

fn soil_thermal_snapshot() -> SoilThermalSnapshot {
    SoilThermalSnapshot {
        owner_id: ResourceOwnerId::try_new("soil-thermal").expect("owner"),
        configuration_sha256: digest('5'),
        state_sha256: digest('4'),
        snapshot_sha256: digest('6'),
        last_accepted_transaction_id: None,
        ofes: vec![SoilThermalOfeSnapshot {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            ordered_layers: (0..4)
                .map(|index| SoilThermalLayerSnapshot {
                    layer_id: SoilLayerId::try_new(format!("thermal-{}", index + 1))
                        .expect("layer"),
                    temperature_k: 291.5 - 1.1 * f64::from(index),
                    enthalpy_j_m2_ofe_ground: 1.0e6 * f64::from(index + 1),
                })
                .collect(),
        }],
    }
}

fn unified_finalization(water_protocol: WaterProtocol) -> UnifiedLseFinalization {
    let lse_digest = digest('2');
    let soil_digest = digest('4');
    let owners = [
        (
            OwnerKind::LandSurfaceEnergy,
            "land-surface-energy-v1",
            lse_digest.clone(),
        ),
        (
            OwnerKind::Hydrology,
            "production-hydrology",
            water_protocol.beginning_snapshot_sha256.clone(),
        ),
        (OwnerKind::SoilThermal, "soil-thermal", soil_digest.clone()),
    ];
    let expectations = receiver_expectations(1, water_protocol.beginning_snapshot_sha256.clone());
    UnifiedLseFinalization::try_new(
        &expectations,
        water_protocol,
        vec![TileState {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            tile_id: TileId::try_new("open").expect("tile"),
            surface_enthalpy_j_m2_tile_ground: 10.0,
            surface_temperature_warm_start_k: 291.0,
        }],
        vec![SoilThermalTileCandidate {
            owner_id: ResourceOwnerId::try_new("soil-thermal").expect("owner"),
            beginning_state_sha256: soil_digest,
            beginning_identity:
                openwepp_land_surface_energy::SoilThermalCandidateBeginningIdentity::V1 {
                    configuration_sha256: Sha256Digest::try_new("5".repeat(64))
                        .expect("configuration digest"),
                    last_accepted_transaction_id: None,
                },
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            tile_id: TileId::try_new("open").expect("tile"),
            layers: vec![SoilThermalLayerCandidate {
                layer_id: SoilLayerId::try_new("thermal-1").expect("layer"),
                beginning_enthalpy_j_m2_ofe_ground: 1.0e6,
                beginning_enthalpy_carry: openwepp_land_surface_energy::ExactDyadicEnthalpy::zero(),
                ground_heat_credit_j_m2_ofe_ground: 0.0,
                infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 1.0e6,
                ending_temperature_k: 291.5,
            }],
        }],
        owners
            .into_iter()
            .map(|(owner_kind, owner_id, digest)| OwnerRollbackHash {
                owner_kind,
                owner_id: owner_id.into(),
                before_sha256: digest.clone(),
                after_sha256: digest,
            })
            .collect(),
    )
    .expect("sealed finalization")
}

fn two_tile_finalization(water_protocol: WaterProtocol) -> UnifiedLseFinalization {
    let baseline = unified_finalization(water_protocol);
    let mut tiles = baseline.ending_tile_states_pre_ingress().to_vec();
    let mut covered_tile = tiles[0].clone();
    covered_tile.tile_id = TileId::try_new("covered").expect("covered tile");
    tiles.push(covered_tile);
    let mut thermal = baseline.soil_thermal_candidates().to_vec();
    let mut covered_thermal = thermal[0].clone();
    covered_thermal.tile_id = TileId::try_new("covered").expect("covered thermal tile");
    thermal.push(covered_thermal);
    let expectations = UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        digest('2'),
        ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
        baseline.water_protocol().beginning_snapshot_sha256.clone(),
        ResourceOwnerId::try_new("soil-thermal").expect("thermal owner"),
        digest('4'),
        thermal
            .iter()
            .map(|tile| {
                (
                    tile.ofe_id.clone(),
                    tile.tile_id.clone(),
                    tile.layers
                        .iter()
                        .map(|layer| layer.layer_id.clone())
                        .collect(),
                )
            })
            .collect(),
    )
    .expect("two-tile expectations");
    UnifiedLseFinalization::try_new(
        &expectations,
        baseline.water_protocol().clone(),
        tiles,
        thermal,
        baseline.rollback_hashes().to_vec(),
    )
    .expect("two-tile finalization")
}

fn receiver_expectations(
    layer_count: usize,
    hydrology_snapshot: Sha256Digest,
) -> UnifiedReceiverExpectations {
    UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        digest('2'),
        ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
        hydrology_snapshot,
        ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        digest('4'),
        vec![(
            OfeId::try_new("ofe-1").expect("OFE"),
            TileId::try_new("open").expect("tile"),
            (0..layer_count)
                .map(|index| SoilLayerId::try_new(format!("thermal-{}", index + 1)).expect("layer"))
                .collect(),
        )],
    )
    .expect("receiver expectations")
}

#[test]
fn receiver_expectation_hashes_are_framed_and_invalid_cardinality_is_canonical() {
    let owner = ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner");
    let thermal_owner = ResourceOwnerId::try_new("soil-thermal").expect("thermal owner");
    let make = |ofe: &str, tile: &str, layers: &[&str]| {
        UnifiedReceiverExpectations::try_new(
            owner.clone(),
            digest('2'),
            ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
            digest('3'),
            thermal_owner.clone(),
            digest('4'),
            vec![(
                OfeId::try_new(ofe).expect("OFE"),
                TileId::try_new(tile).expect("tile"),
                layers
                    .iter()
                    .map(|layer| SoilLayerId::try_new(*layer).expect("layer"))
                    .collect(),
            )],
        )
    };
    let split_left = make("a", "bc", &["thermal"]).expect("left split");
    let split_right = make("ab", "c", &["thermal"]).expect("right split");
    assert_ne!(
        split_left.canonical_sha256(),
        split_right.canonical_sha256()
    );
    let one_layer = make("ofe", "tile", &["ab"]).expect("one layer");
    let two_layers = make("ofe", "tile", &["a", "b"]).expect("two layers");
    assert_ne!(one_layer.canonical_sha256(), two_layers.canonical_sha256());

    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) =
        make("ofe", "tile", &["a", "a"]).expect_err("duplicate layers")
    else {
        panic!("invalid expectations must retain canonical failure");
    };
    let failure = error.failure().expect("canonical expectation failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E005);
    assert_eq!(
        failure.phase,
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidPhase::IndependentClosure
    );
    assert_eq!(failure.context.transaction_id, None);
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("soil-thermal")
    );
    assert_eq!(
        failure.context.ofe_id.as_ref().map(OfeId::as_str),
        Some("ofe")
    );
    assert_eq!(
        failure.context.tile_id.as_ref().map(TileId::as_str),
        Some("tile")
    );
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(digest('3').as_str())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

fn ingress_input() -> DirectSurfaceLiquidIngressInput {
    ingress_input_with_mass(0.0)
}

fn ingress_input_with_mass(mass_kg_m2_tile_ground: f64) -> DirectSurfaceLiquidIngressInput {
    let temperature_k = 294.0;
    DirectSurfaceLiquidIngressInput {
        transaction_id: TransactionId(41),
        day_index: 0,
        interval_index: 0,
        interval_s: 1_800.0,
        tile_ingress: vec![DirectTileGroundIngress::OpenRawPrecipitation {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            tile_id: TileId::try_new("open").expect("tile"),
            surface_id: SurfaceId::try_new("surface:ofe-1:open").expect("surface"),
            raw_precipitation: DirectIngressAmount {
                mass_kg_m2_tile_ground,
                temperature_k,
                specific_liquid_enthalpy_j_kg: 4_218.0 * (temperature_k - 273.15),
                start_s: 0.0,
                end_s: 1_800.0,
            },
        }],
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.04,
        }],
    }
}

fn two_tile_ingress_input() -> DirectSurfaceLiquidIngressInput {
    let mut input = ingress_input();
    let zero = DirectIngressAmount {
        mass_kg_m2_tile_ground: 0.0,
        temperature_k: 294.0,
        specific_liquid_enthalpy_j_kg: 4_218.0 * (294.0 - 273.15),
        start_s: 0.0,
        end_s: 1_800.0,
    };
    input
        .tile_ingress
        .push(DirectTileGroundIngress::CoveredCanopyRelease {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            tile_id: TileId::try_new("covered").expect("covered tile"),
            surface_id: SurfaceId::try_new("surface:ofe-1:covered").expect("covered surface"),
            release: DirectCanopyLiquidRelease {
                throughfall: zero.clone(),
                initial_drainage: zero.clone(),
                second_drainage: zero.clone(),
                stemflow: zero,
            },
        });
    input
}

fn accepted_surface_protocol(
    batch: &openwepp_hillslope_orchestrator::land_surface_energy_shadow::PotentialWaterRequestBatch,
    authorizations: &[openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAuthorization],
    snapshot: &Sha256Digest,
) -> WaterProtocol {
    WaterProtocol {
        transaction_id: TransactionId(41),
        hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        beginning_snapshot_sha256: snapshot.clone(),
        requests: batch.requests.clone(),
        authorizations: authorizations.to_vec(),
        finalized_uses: authorizations
            .iter()
            .map(
                |row| openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                    key: row.key.clone(),
                    amount_kg_m2_stand_ground: row.amount_kg_m2_stand_ground,
                },
            )
            .collect(),
        condensation_credits: Vec::new(),
    }
}

#[test]
fn open_surface_is_rebuilt_from_beginning_after_one_mixed_authorization() {
    let frame = production_frame(0.02, false);
    let original = frame.clone();
    let (real_owner, source) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
    let root = request("root", 19.95, &source);
    let result = execute_open_bare_soil_shadow(
        &adapter,
        &open_problem(),
        key("ground", "thermal-1"),
        source,
        &[root],
        |authorizations| {
            Ok(authorizations
                .iter()
                .filter(|row| {
                    row.authorization.key.requesting_component
                        == openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::VegetationRoot
                })
                .map(|row| MixedRealHydrologyUse {
                    finalized_use: openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                        key: row.authorization.key.clone(),
                        amount_kg_m2_stand_ground: row
                            .authorization
                            .amount_kg_m2_stand_ground,
                    },
                    source: row.source.clone(),
                })
                .collect())
        },
    )
    .expect("real owner potential/final transaction");
    assert_eq!(result.arbitration.requests().len(), 2);
    let ground_authorization = result
        .arbitration
        .authorizations()
        .iter()
        .find(|row| {
            row.authorization.key.requesting_component
                == openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::GroundSurface
        })
        .expect("ground authorization");
    let ground_use = result
        .hydrology_candidate
        .finalized_uses()
        .iter()
        .find(|row| {
            row.finalized_use.key.requesting_component
                == openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::GroundSurface
        })
        .expect("ground finalized use");
    assert_eq!(
        ground_use.finalized_use.amount_kg_m2_stand_ground.to_bits(),
        ground_authorization
            .authorization
            .amount_kg_m2_stand_ground
            .to_bits()
    );
    assert_eq!(frame, original);
    assert_eq!(result.hydrology_candidate.beginning_frame(), &original);
}

#[test]
fn root_and_bare_ground_share_one_real_layer_authorization_and_clone_only_debit() {
    let frame = production_frame(0.02, false);
    let original = frame.clone();
    let (real_owner, source) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
    let requests = vec![
        request("root", 15.0, &source),
        request("ground", 15.0, &source),
    ];
    let arbitration = adapter.authorize(&requests).expect("one arbitration");
    assert_eq!(arbitration.authorizations().len(), 2);
    assert!(arbitration.authorizations().iter().all(|row| {
        row.authorization.amount_kg_m2_stand_ground.to_bits() == 10.0_f64.to_bits()
            && row.authorization.amount_kg_m2_stand_ground
                <= requests
                    .iter()
                    .find(|request| request.request.key == row.authorization.key)
                    .expect("matching request")
                    .request
                    .amount_kg_m2_stand_ground
    }));
    let uses = arbitration
        .authorizations()
        .iter()
        .map(|row| MixedRealHydrologyUse {
            finalized_use:
                openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                    key: row.authorization.key.clone(),
                    amount_kg_m2_stand_ground: row.authorization.amount_kg_m2_stand_ground,
                },
            source: row.source.clone(),
        })
        .collect::<Vec<_>>();
    let candidate = adapter
        .candidate_from_finalized_uses(&arbitration, &uses)
        .expect("candidate");
    assert_eq!(frame, original, "production state changed during shadow");
    assert_eq!(candidate.beginning_frame(), &original);
    assert_eq!(
        candidate.ending_frame().lanes[0].subsurface_layers[0]
            .theta_m
            .to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn full_supply_frozen_duplicate_wrong_layer_and_rollback_are_fail_closed() {
    let frame = production_frame(0.02, false);
    let original = frame.clone();
    let (real_owner, source) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
    let full = vec![
        request("root", 4.0, &source),
        request("ground", 5.0, &source),
    ];
    assert!(adapter.authorize(&full).expect("full supply").authorizations().iter().all(
        |row| row.authorization.reason
            == openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAuthorizationReason::FullSupply
    ));
    assert!(matches!(
        adapter.authorize(&[full[0].clone(), full[0].clone()]),
        Err(LandSurfaceEnergyShadowError::Identity(
            "duplicate mixed request"
        ))
    ));
    let mut wrong = full[0].clone();
    wrong.request.key.soil_layer_id = Some(SoilLayerId::try_new("soil-2").expect("wrong layer"));
    assert!(matches!(
        adapter.authorize(&[wrong]),
        Err(LandSurfaceEnergyShadowError::Identity(
            "mixed source identity"
        ))
    ));
    assert_eq!(frame, original);

    let frozen_frame = production_frame(0.02, true);
    let (frozen_owner, frozen_source) = owner(&frozen_frame);
    let frozen_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&frozen_owner);
    let result = frozen_adapter
        .authorize(&[request("ground", 5.0, &frozen_source)])
        .expect("typed frozen exclusion");
    assert_eq!(
        result.authorizations()[0]
            .authorization
            .amount_kg_m2_stand_ground
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        result.authorizations()[0].authorization.reason,
        openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAuthorizationReason::FrozenSource
    );
}

#[test]
fn unified_surface_owner_accepts_open_and_litter_without_rewriting_keys() {
    for (class, source_type) in [
        (
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
        ),
        (SurfaceClass::ForestLitter, WaterSourceType::LitterLiquid),
    ] {
        let (frame, configuration) = configured_surface_frame(class, source_type, 1.0);
        let original = frame.clone();
        let source_id = configuration.records[0].key.source_id.clone();
        let batch = surface_potential_batch(class, source_type, source_id, 1.0);
        let request_keys = batch
            .requests
            .iter()
            .map(|row| row.key.clone())
            .collect::<Vec<_>>();
        let (real_owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
        let hydrology_snapshot =
            unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
                .expect("unified snapshot");
        let candidate = execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &receiver_expectations(1, hydrology_snapshot.clone()),
            &batch,
            &BTreeMap::new(),
            &ingress_input(),
            |authorizations| {
                Ok(unified_finalization(WaterProtocol {
                        transaction_id: TransactionId(41),
                        hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology")
                            .expect("owner"),
                        beginning_snapshot_sha256: hydrology_snapshot.clone(),
                        requests: batch.requests.clone(),
                        authorizations: authorizations.to_vec(),
                        finalized_uses: authorizations
                            .iter()
                            .map(|row| openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                                key: row.key.clone(),
                                amount_kg_m2_stand_ground: row.amount_kg_m2_stand_ground,
                            })
                            .collect(),
                        condensation_credits: Vec::new(),
                    }))
            },
        )
        .expect("unified surface transaction");
        assert_eq!(
            candidate
                .arbitration()
                .requests
                .iter()
                .map(|row| row.key.clone())
                .collect::<Vec<_>>(),
            request_keys
        );
        assert_eq!(frame, original, "production frame changed");
        assert_eq!(candidate.beginning_frame(), &original);
        assert_eq!(
            candidate
                .ending_frame()
                .surface_liquid_shadow
                .as_ref()
                .expect("ending surface state")
                .as_ref(),
            candidate.surface_ingress().ending_state()
        );
    }
}

#[test]
fn unified_snapshot_sha256_frames_the_canonical_soil_snapshot_bytes() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0e-200,
    );
    let surface_state = frame
        .surface_liquid_shadow
        .as_deref()
        .expect("surface state");
    let (real_owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
    let actual = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("unified snapshot");
    let mut sha = Sha256::new();
    for bytes in [
        b"openwepp-unified-hydrology-snapshot-v2".as_slice(),
        b"production-hydrology".as_slice(),
        real_owner.snapshot_bytes(),
        configuration.configuration_sha256.as_bytes(),
        surface_state.state_sha256.as_bytes(),
    ] {
        sha.update((bytes.len() as u64).to_be_bytes());
        sha.update(bytes);
    }
    assert_eq!(actual.as_str(), format!("{:x}", sha.finalize()));
}

#[test]
fn unified_surface_owner_consumes_the_actual_fixed_cap_lse_water_protocol() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let original = frame.clone();
    let (real_owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
    let hydrology_snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("unified snapshot");
    let phase = surface_potential_phase_with_snapshot(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
        hydrology_snapshot.clone(),
    );
    let candidate = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(4, hydrology_snapshot.clone()),
        &phase.request_batch,
        &BTreeMap::new(),
        &ingress_input(),
        |authorizations| {
            let authorization = authorizations
                .iter()
                .find(|row| row.key == phase.request_batch.requests[0].key)
                .ok_or(LandSurfaceEnergyShadowError::Identity(
                    "actual fixed-cap authorization missing",
                ))?;
            let final_candidate = finalize_open_phase(
                &phase,
                &digest('2'),
                authorization,
                None,
                &soil_thermal_snapshot(),
            )?;
            let expectations = finalization_expectations(
                &final_candidate.water_protocol,
                std::slice::from_ref(&final_candidate.soil_thermal),
            );
            UnifiedLseFinalization::try_new(
                &expectations,
                final_candidate.water_protocol,
                vec![final_candidate.ending_tile_state_pre_ingress],
                vec![final_candidate.soil_thermal],
                final_candidate
                    .rollback_hashes
                    .into_iter()
                    .filter(|row| {
                        matches!(
                            row.owner_kind,
                            OwnerKind::LandSurfaceEnergy
                                | OwnerKind::Hydrology
                                | OwnerKind::SoilThermal
                        )
                    })
                    .collect(),
            )
        },
    )
    .expect("actual LSE fixed-cap protocol accepted by real owner");
    assert_eq!(frame, original);
    assert_eq!(
        candidate.arbitration().requests,
        phase.request_batch.requests
    );
    assert_eq!(
        candidate.finalized_uses()[0].key,
        candidate.arbitration().authorizations[0].key
    );
}

#[test]
fn unified_surface_owner_applies_signed_condensation_credit_before_ingress() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::ForestLitter,
        WaterSourceType::LitterLiquid,
        0.2,
    );
    let original = frame.clone();
    let batch = surface_potential_batch(
        SurfaceClass::ForestLitter,
        WaterSourceType::LitterLiquid,
        configuration.records[0].key.source_id.clone(),
        0.2,
    );
    let (real_owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
    let hydrology_snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("unified snapshot");
    let temperature_k = 291.0;
    let candidate = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, hydrology_snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |authorizations| {
            Ok(unified_finalization(WaterProtocol {
                transaction_id: TransactionId(41),
                hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology")
                    .expect("owner"),
                beginning_snapshot_sha256: hydrology_snapshot.clone(),
                requests: batch.requests.clone(),
                authorizations: authorizations.to_vec(),
                finalized_uses: authorizations
                    .iter()
                    .map(|row| {
                        openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                            key: row.key.clone(),
                            amount_kg_m2_stand_ground: 0.0,
                        }
                    })
                    .collect(),
                condensation_credits: vec![CondensationCredit {
                    transaction_id: TransactionId(41),
                    hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology")
                        .expect("owner"),
                    ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                    tile_id: TileId::try_new("open").expect("tile"),
                    surface_id: SurfaceId::try_new("surface:ofe-1:open").expect("surface"),
                    amount_kg_m2_stand_ground: 0.1,
                    amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
                    temperature_k,
                    specific_liquid_enthalpy_j_kg: 4_218.0 * (temperature_k - 273.15),
                }],
            }))
        },
    )
    .expect("condensation resource/ingress transaction");
    assert_eq!(frame, original);
    assert_eq!(candidate.surface_resource().condensation_credits().len(), 1);
    assert_eq!(
        candidate.condensation_credits()[0]
            .amount_kg_m2_stand_ground
            .to_bits(),
        0.1_f64.to_bits()
    );
}

#[test]
fn unified_bridge_rejects_a_valid_but_wrong_lineage_final_protocol() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let original = frame.clone();
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let (real_owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
    let hydrology_snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("unified snapshot");
    let wrong_beginning = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, digest('3')),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |_| panic!("fixed-cap solve must not run for a stale beginning snapshot"),
    );
    assert_public_surface_failure(
        wrong_beginning.expect_err("stale beginning must reject"),
        DirectSurfaceLiquidErrorCode::E002,
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidPhase::Authorization,
        &hydrology_snapshot,
        None,
    );
    assert_eq!(frame, original);
    let result = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, hydrology_snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |authorizations| {
            Ok(unified_finalization(WaterProtocol {
                transaction_id: TransactionId(41),
                hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology")
                    .expect("owner"),
                beginning_snapshot_sha256: digest('9'),
                requests: batch.requests.clone(),
                authorizations: authorizations.to_vec(),
                finalized_uses: authorizations
                    .iter()
                    .map(|row| {
                        openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                            key: row.key.clone(),
                            amount_kg_m2_stand_ground: row.amount_kg_m2_stand_ground,
                        }
                    })
                    .collect(),
                condensation_credits: Vec::new(),
            }))
        },
    );
    assert_public_surface_failure(
        result.expect_err("wrong protocol lineage must reject"),
        DirectSurfaceLiquidErrorCode::E002,
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidPhase::ResourceCandidate,
        &hydrology_snapshot,
        Some("ofe-1"),
    );
    assert_eq!(frame, original);
}

fn assert_public_surface_failure(
    error: LandSurfaceEnergyShadowError,
    code: DirectSurfaceLiquidErrorCode,
    phase: openwepp_hillslope_orchestrator::DirectSurfaceLiquidPhase,
    beginning: &Sha256Digest,
    ofe_id: Option<&str>,
) {
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
        panic!("public bridge must retain canonical surface-liquid failure");
    };
    let failure = error.failure().expect("canonical public failure");
    assert_eq!(failure.code, code);
    assert_eq!(failure.phase, phase);
    assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("production-hydrology")
    );
    assert_eq!(failure.context.ofe_id.as_ref().map(OfeId::as_str), ofe_id);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(beginning.as_str())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
fn request_numeric_preflight_precedes_duplicate_validation() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let mut batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    batch.requests.push(batch.requests[0].clone());
    batch.requests[1].amount_kg_m2_stand_ground = f64::NAN;
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |_| panic!("numeric preflight rejects before finalization"),
    )
    .expect_err("nonfinite duplicate request") else {
        panic!("request preflight must retain canonical failure");
    };
    let failure = error.failure().expect("canonical request failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Authorization);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(snapshot.as_str())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
fn unified_surface_authorization_failure_retains_complete_request_attempt_hash() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        f64::from_bits(1),
    );
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        0.5,
    );

    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |_| panic!("surface authorization arithmetic must reject before finalization"),
    )
    .expect_err("surface authorization underflow must fail closed") else {
        panic!("surface authorization failure must retain canonical context");
    };
    let failure = error
        .failure()
        .expect("canonical surface authorization failure");
    assert_eq!(
        failure.code,
        DirectSurfaceLiquidErrorCode::E003,
        "{failure:?}"
    );
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Authorization);
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(snapshot.as_str()),
        "public unified failure must report the unified beginning snapshot"
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
#[allow(clippy::too_many_lines)]
fn later_request_and_protocol_validation_preserve_exact_offender_context() {
    let (frame, configuration) = configured_two_tile_surface_frame();
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let mut batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        open_surface_source_id(&configuration),
        1.0,
    );
    let covered = configuration
        .records
        .iter()
        .find(|record| record.key.tile_id.as_str() == "covered")
        .expect("covered configuration record");
    let mut later = batch.requests[0].clone();
    later.key.transaction_id = TransactionId(42);
    later.key.requesting_tile_id = covered.key.tile_id.clone();
    later.key.surface_id = Some(covered.key.surface_id.clone());
    later.key.surface_class = Some(covered.key.surface_class);
    later.key.source_type = covered.key.source_type;
    later.key.source_id = covered.key.source_id.clone();
    later.key.source_tile_id = Some(covered.key.tile_id.clone());
    batch.requests.push(later.clone());
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |_| panic!("later malformed request must reject before callback"),
    )
    .expect_err("later request identity poison") else {
        panic!("later request poison must remain canonical");
    };
    let failure = error.failure().expect("canonical later request failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.context.transaction_id, Some(TransactionId(42)));
    assert_eq!(
        failure.context.tile_id.as_ref().map(TileId::as_str),
        Some("covered")
    );
    assert_eq!(
        failure.context.source_id.as_ref().map(SourceId::as_str),
        Some("surface-store:ofe-1:covered")
    );

    let valid_batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let protocol = accepted_surface_protocol(
        &valid_batch,
        &[WaterAuthorization {
            key: valid_batch.requests[0].key.clone(),
            amount_kg_m2_stand_ground: valid_batch.requests[0].amount_kg_m2_stand_ground,
            reason: WaterAuthorizationReason::FullSupply,
        }],
        &snapshot,
    );
    let mut invalid_protocol = protocol.clone();
    let mut later_request = protocol.requests[0].clone();
    later_request.key.requesting_tile_id = covered.key.tile_id.clone();
    later_request.key.surface_id = Some(covered.key.surface_id.clone());
    later_request.key.surface_class = Some(covered.key.surface_class);
    later_request.key.source_type = covered.key.source_type;
    later_request.key.source_id = covered.key.source_id.clone();
    later_request.key.source_tile_id = Some(covered.key.tile_id.clone());
    invalid_protocol.requests.push(later_request.clone());
    let mut later_authorization = WaterAuthorization {
        key: later_request.key.clone(),
        amount_kg_m2_stand_ground: 0.0,
        reason: WaterAuthorizationReason::ZeroSupply,
    };
    later_authorization.key.transaction_id = TransactionId(42);
    invalid_protocol.authorizations.push(later_authorization);
    let baseline = unified_finalization(protocol);
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = UnifiedLseFinalization::try_new(
        &finalization_expectations(
            baseline.water_protocol(),
            baseline.soil_thermal_candidates(),
        ),
        invalid_protocol,
        baseline.ending_tile_states_pre_ingress().to_vec(),
        baseline.soil_thermal_candidates().to_vec(),
        baseline.rollback_hashes().to_vec(),
    )
    .expect_err("later protocol identity poison") else {
        panic!("later protocol poison must remain canonical");
    };
    let failure = error.failure().expect("canonical later protocol failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.context.transaction_id, Some(TransactionId(42)));
    assert_eq!(
        failure.context.tile_id.as_ref().map(TileId::as_str),
        Some("covered")
    );
    assert_eq!(
        failure.context.source_id.as_ref().map(SourceId::as_str),
        Some("surface-store:ofe-1:covered")
    );
}

#[test]
fn final_protocol_cardinality_and_bounds_are_canonical() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let original = frame.clone();
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    for (poison, expected) in [
        (0_usize, DirectSurfaceLiquidErrorCode::E005),
        (1_usize, DirectSurfaceLiquidErrorCode::E006),
        (2_usize, DirectSurfaceLiquidErrorCode::E003),
    ] {
        let result = execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &receiver_expectations(1, snapshot.clone()),
            &batch,
            &BTreeMap::new(),
            &ingress_input(),
            |authorizations| {
                let finalization = unified_finalization(accepted_surface_protocol(
                    &batch,
                    authorizations,
                    &snapshot,
                ));
                let mut protocol = finalization.water_protocol().clone();
                match poison {
                    0 => protocol.requests.push(protocol.requests[0].clone()),
                    1 => {
                        protocol.finalized_uses[0].amount_kg_m2_stand_ground =
                            protocol.authorizations[0].amount_kg_m2_stand_ground + 1.0;
                    }
                    2 => {
                        protocol.requests.push(protocol.requests[0].clone());
                        protocol.finalized_uses[0].amount_kg_m2_stand_ground = f64::NAN;
                    }
                    _ => unreachable!("bounded poison table"),
                }
                UnifiedLseFinalization::try_new(
                    &finalization_expectations(
                        finalization.water_protocol(),
                        finalization.soil_thermal_candidates(),
                    ),
                    protocol,
                    finalization.ending_tile_states_pre_ingress().to_vec(),
                    finalization.soil_thermal_candidates().to_vec(),
                    finalization.rollback_hashes().to_vec(),
                )
            },
        );
        let LandSurfaceEnergyShadowError::SurfaceLiquid(error) =
            result.expect_err("invalid final protocol")
        else {
            panic!("invalid final protocol must retain canonical failure");
        };
        let failure = error.failure().expect("canonical protocol failure");
        assert_eq!(failure.code, expected);
        assert_eq!(
            failure.phase,
            openwepp_hillslope_orchestrator::DirectSurfaceLiquidPhase::ResourceCandidate
        );
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
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(snapshot.as_str())
        );
        assert!(failure.rollback.attempted_owner_sha256.is_some());
        assert_eq!(frame, original, "protocol poison mutated caller owner");
    }
}

#[test]
fn surface_attachment_rejects_wrong_area_and_adapter_layer_map() {
    let base = surface_configuration(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
    );
    let mut wrong_area_records = base.records.clone();
    wrong_area_records[0].ofe_area_m2 = 101.0;
    let wrong_area = DirectSurfaceLiquidConfiguration::new(
        base.owner_id.clone(),
        base.run_id,
        base.ofe_topology.clone(),
        base.ofe_bindings.clone(),
        wrong_area_records,
    )
    .expect("structurally valid wrong-area configuration");
    let wrong_area_state = DirectSurfaceLiquidOwnedState::new_initial(
        &wrong_area,
        &BTreeMap::from([(wrong_area.records[0].key.clone(), 1.0)]),
        0,
    )
    .expect("wrong-area state");
    let mut frame = production_frame(0.02, false);
    assert!(
        frame
            .configure_surface_liquid_shadow(&wrong_area, wrong_area_state)
            .is_err()
    );

    let mut wrong_layer_bindings = base.ofe_bindings.clone();
    wrong_layer_bindings[0].ordered_soil_layer_ids =
        vec![SoilLayerId::try_new("thermal-2").expect("layer")];
    wrong_layer_bindings[0].infiltration_soil_thermal_layer_id =
        SoilLayerId::try_new("thermal-2").expect("layer");
    let wrong_layer = DirectSurfaceLiquidConfiguration::new(
        base.owner_id.clone(),
        base.run_id,
        base.ofe_topology.clone(),
        wrong_layer_bindings,
        base.records.clone(),
    )
    .expect("structurally valid wrong-layer configuration");
    let state = DirectSurfaceLiquidOwnedState::new_initial(
        &wrong_layer,
        &BTreeMap::from([(wrong_layer.records[0].key.clone(), 1.0)]),
        0,
    )
    .expect("wrong-layer state");
    let mut frame = production_frame(0.02, false);
    frame
        .configure_surface_liquid_shadow(&wrong_layer, state)
        .expect("frame validates structural layer count");
    let (wrong_layer_owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&wrong_layer_owner);
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &wrong_layer)
            .expect_err("wrong production binding")
    else {
        panic!("wrong production binding must retain canonical failure");
    };
    let failure = error.failure().expect("canonical binding failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(
        failure.phase,
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidPhase::Restart
    );
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
fn surface_attachment_preserves_invalid_configuration_failure_and_rolls_back() {
    let (mut frame, mut configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let original = frame.clone();
    let attempted_state = DirectSurfaceLiquidOwnedState::new_initial(
        &configuration,
        &BTreeMap::from([(configuration.records[0].key.clone(), 2.0)]),
        0,
    )
    .expect("attempted state");
    let declared_attempted_hash = attempted_state.state_sha256.clone();
    configuration.records[0].capacity_kg_m2_tile = f64::NAN;

    let error = frame
        .configure_surface_liquid_shadow(&configuration, attempted_state)
        .expect_err("invalid configuration must fail closed");
    let failure = error.failure().expect("canonical configuration failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Configuration);
    assert_eq!(failure.context.transaction_id, None);
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(failure.context.ofe_id, None);
    assert_eq!(failure.context.tile_id, None);
    assert_eq!(failure.context.surface_id, None);
    assert_eq!(failure.context.source_id, None);
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());
    assert_ne!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(declared_attempted_hash.as_str()),
        "raw attempt hash must not trust the embedded state digest"
    );
    assert_eq!(frame, original);
}

#[test]
fn first_surface_attachment_preserves_invalid_restart_attempt_hash_without_beginning_hash() {
    let configuration = surface_configuration(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
    );
    let mut attempted_state = DirectSurfaceLiquidOwnedState::new_initial(
        &configuration,
        &BTreeMap::from([(configuration.records[0].key.clone(), 2.0)]),
        0,
    )
    .expect("attempted state");
    let declared_attempted_hash = attempted_state.state_sha256.clone();
    attempted_state.records[0].liquid_kg_m2_tile = f64::NAN;
    let mut frame = production_frame(0.02, false);
    let original = frame.clone();

    let error = frame
        .configure_surface_liquid_shadow(&configuration, attempted_state)
        .expect_err("invalid first restart state must fail closed");
    let failure = error.failure().expect("canonical restart failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Restart);
    assert_eq!(failure.rollback.beginning_owner_sha256, None);
    assert!(failure.rollback.attempted_owner_sha256.is_some());
    assert_ne!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(declared_attempted_hash.as_str())
    );
    assert_eq!(frame, original);
}

#[test]
fn surface_attachment_preserves_invalid_restart_failure_and_rolls_back() {
    let (mut frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let original = frame.clone();
    let mut attempted_state = DirectSurfaceLiquidOwnedState::new_initial(
        &configuration,
        &BTreeMap::from([(configuration.records[0].key.clone(), 2.0)]),
        0,
    )
    .expect("attempted state");
    let declared_attempted_hash = attempted_state.state_sha256.clone();
    let expected_key = attempted_state.records[0].key.clone();
    attempted_state.records[0].liquid_kg_m2_tile = f64::NAN;

    let error = frame
        .configure_surface_liquid_shadow(&configuration, attempted_state)
        .expect_err("invalid restart state must fail closed");
    let failure = error.failure().expect("canonical restart failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::Restart);
    assert_eq!(failure.context.transaction_id, None);
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(failure.context.ofe_id, Some(expected_key.ofe_id));
    assert_eq!(failure.context.tile_id, Some(expected_key.tile_id));
    assert_eq!(failure.context.surface_id, Some(expected_key.surface_id));
    assert_eq!(failure.context.source_id, Some(expected_key.source_id));
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());
    assert_ne!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(declared_attempted_hash.as_str())
    );
    assert_eq!(frame, original);
}

#[test]
fn surface_runtime_rejects_wrong_day_and_lse_ofe_receiver() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let mut stale_day = ingress_input();
    stale_day.day_index = 1;
    let stale = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &stale_day,
        |_| panic!("wrong day rejects before final solve"),
    );
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = stale.expect_err("stale day") else {
        panic!("stale day must retain canonical failure");
    };
    let failure = error.failure().expect("canonical stale-day failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E008);
    assert_eq!(
        failure.phase,
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidPhase::Authorization
    );
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(snapshot.as_str())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());

    let wrong_ofe = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |authorizations| {
            let finalization =
                unified_finalization(accepted_surface_protocol(&batch, authorizations, &snapshot));
            let mut tiles = finalization.ending_tile_states_pre_ingress().to_vec();
            tiles[0].ofe_id = OfeId::try_new("ofe-wrong").expect("wrong OFE");
            UnifiedLseFinalization::try_new(
                &finalization_expectations(
                    finalization.water_protocol(),
                    finalization.soil_thermal_candidates(),
                ),
                finalization.water_protocol().clone(),
                tiles,
                finalization.soil_thermal_candidates().to_vec(),
                finalization.rollback_hashes().to_vec(),
            )
        },
    );
    let error = wrong_ofe.expect_err("wrong OFE finalization must fail");
    let LandSurfaceEnergyShadowError::SurfaceLiquid(surface_error) = error else {
        panic!("sealed finalization must emit canonical envelope failure");
    };
    assert_eq!(
        surface_error.failure().expect("failure payload").code,
        DirectSurfaceLiquidErrorCode::E011
    );
}

fn apply_native_entry_poison(
    frame: &mut DirectRunFrame,
    poison: usize,
) -> DirectSurfaceLiquidErrorCode {
    match poison {
        0 => {
            frame.lanes[0].winter_column.snow.runtime_swe_m = 0.001;
            DirectSurfaceLiquidErrorCode::E004
        }
        1 => {
            frame.lanes[0]
                .day_inputs
                .push(DirectDayConstructorInputs::zero());
            frame.lanes[0].day_inputs[0]
                .infiltration_depression_inputs
                .depression_storage_delta_handoff_m = 0.001;
            DirectSurfaceLiquidErrorCode::E007
        }
        2 => {
            frame.lanes[0].subsurface_layers[0].frozen_depth_m =
                frame.lanes[0].subsurface_layers[0].depth_m;
            frame.lanes[0].subsurface_layers[0].frozen_water_m =
                frame.lanes[0].subsurface_layers[0].theta_m;
            DirectSurfaceLiquidErrorCode::E004
        }
        3 => {
            frame.lanes[0].winter_column.frost.active_frost_coupling = true;
            DirectSurfaceLiquidErrorCode::E004
        }
        4 => {
            frame.lanes[0].frost_runtime_carry =
                Some(DirectFrostRuntimeCarry::from(DirectFrostLaneState::zero()));
            DirectSurfaceLiquidErrorCode::E004
        }
        _ => {
            frame.lanes[0].winter_column.snow.liquid_water_retained_m = 0.001;
            DirectSurfaceLiquidErrorCode::E004
        }
    }
}

fn assert_native_entry_failure(
    error: LandSurfaceEnergyShadowError,
    expected_code: DirectSurfaceLiquidErrorCode,
    snapshot: &Sha256Digest,
) {
    let LandSurfaceEnergyShadowError::SurfaceLiquid(surface_error) = error else {
        panic!("entry poison must retain canonical surface-liquid failure");
    };
    let failure = surface_error.failure().expect("canonical entry failure");
    assert_eq!(failure.code, expected_code);
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
}

#[test]
fn unified_entry_rejects_frozen_snow_and_duplicate_legacy_surface_custody() {
    for poison in 0..6 {
        let (mut frame, configuration) = configured_surface_frame(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            1.0,
        );
        let expected_code = apply_native_entry_poison(&mut frame, poison);
        let original = frame.clone();
        let (owner, _) = owner(&frame);
        let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
        let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
            .expect("poison snapshot remains representable");
        let batch = surface_potential_batch(
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid,
            configuration.records[0].key.source_id.clone(),
            1.0,
        );
        let mut callback_called = false;
        let result = execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &receiver_expectations(1, snapshot.clone()),
            &batch,
            &BTreeMap::new(),
            &ingress_input(),
            |_| {
                callback_called = true;
                Err(LandSurfaceEnergyShadowError::Identity(
                    "unsupported owner state reached callback",
                ))
            },
        );
        assert!(!callback_called, "entry poison {poison} reached callback");
        assert_native_entry_failure(
            result.expect_err("entry poison must fail"),
            expected_code,
            &snapshot,
        );
        assert_eq!(
            frame, original,
            "entry poison {poison} mutated production owner"
        );
    }
}

#[test]
fn unified_ingress_updates_exact_real_receivers_and_preserves_rollback() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let original = frame.clone();
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let candidate = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input_with_mass(50.0),
        |authorizations| {
            Ok(unified_finalization(accepted_surface_protocol(
                &batch,
                authorizations,
                &snapshot,
            )))
        },
    )
    .expect("real receiver candidate");
    let infiltration_enthalpy = candidate
        .surface_ingress()
        .receipts()
        .iter()
        .filter(|receipt| {
            receipt.disposition
                == openwepp_hillslope_orchestrator::DirectSurfaceLiquidReceiptDisposition::Infiltration
        })
        .map(|receipt| receipt.enthalpy_j_m2_basis_ofe_ground)
        .sum::<f64>();
    let retained_enthalpy = candidate
        .surface_ingress()
        .receipts()
        .iter()
        .filter(|receipt| {
            receipt.disposition
                == openwepp_hillslope_orchestrator::DirectSurfaceLiquidReceiptDisposition::RetainedSurface
        })
        .map(|receipt| receipt.enthalpy_j_m2_basis_ofe_ground)
        .sum::<f64>();
    assert!(
        candidate.ending_frame().lanes[0].subsurface_layers[0].theta_m
            > original.lanes[0].subsurface_layers[0].theta_m
    );
    assert_eq!(
        candidate.soil_thermal_candidates()[0].layers[0]
            .infiltration_enthalpy_credit_j_m2_ofe_ground
            .to_bits(),
        infiltration_enthalpy.to_bits()
    );
    assert_eq!(
        candidate.ending_lse_tile_states()[0]
            .surface_enthalpy_j_m2_tile_ground
            .to_bits(),
        (10.0 + retained_enthalpy).to_bits()
    );
    assert_eq!(frame, original, "all receiver work remains clone-only");
    assert!(
        candidate
            .rollback_hashes()
            .iter()
            .all(|hash| hash.before_sha256 == hash.after_sha256)
    );
}

#[test]
fn independent_real_receiver_equations_reject_layer_and_enthalpy_poisons() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let candidate = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input_with_mass(50.0),
        |authorizations| {
            Ok(unified_finalization(accepted_surface_protocol(
                &batch,
                authorizations,
                &snapshot,
            )))
        },
    )
    .expect("receiver candidate");
    validate_real_receiver_closure(candidate.receiver_closure_operands())
        .expect("independent receiver closure");

    let baseline_hash = candidate.receiver_closure_operands().canonical_sha256();
    let mut snapshot_poison = candidate.receiver_closure_operands().clone();
    snapshot_poison.beginning_hydrology_snapshot_sha256 = digest('9');
    assert_ne!(baseline_hash, snapshot_poison.canonical_sha256());

    reject_receiver_layer_distribution_poison(&candidate);
    reject_receiver_enthalpy_poisons(&candidate);
    reject_receiver_nonfinite_arithmetic_poisons(&candidate);
    reject_receiver_topology_poisons(&candidate);
}

fn reject_receiver_topology_poisons(
    candidate: &openwepp_hillslope_orchestrator::land_surface_energy_shadow::UnifiedRealHydrologyCandidate,
) {
    let baseline = candidate.receiver_closure_operands();
    let mut poisons = Vec::new();
    let mut missing_production = baseline.clone();
    missing_production.production_soil.clear();
    poisons.push((
        DirectSurfaceLiquidErrorCode::E011,
        "production-hydrology",
        None,
        missing_production,
    ));
    let mut duplicate_thermal = baseline.clone();
    duplicate_thermal
        .soil_thermal
        .push(duplicate_thermal.soil_thermal[0].clone());
    poisons.push((
        DirectSurfaceLiquidErrorCode::E011,
        "soil-thermal",
        Some("open"),
        duplicate_thermal,
    ));
    let mut rekeyed_lse = baseline.clone();
    rekeyed_lse.lse_tiles[0].tile_id = TileId::try_new("wrong-receiver").expect("poison tile");
    poisons.push((
        DirectSurfaceLiquidErrorCode::E010,
        "land-surface-energy-v1",
        Some("wrong-receiver"),
        rekeyed_lse,
    ));
    for (code, owner, tile, poison) in poisons {
        let attempted = poison.canonical_sha256();
        let error = validate_real_receiver_closure(&poison).expect_err("topology poison");
        let failure = error.failure().expect("canonical topology failure");
        assert_eq!(failure.code, code);
        assert_eq!(
            failure.phase,
            if code == DirectSurfaceLiquidErrorCode::E011 {
                DirectSurfaceLiquidPhase::AtomicEnvelope
            } else {
                DirectSurfaceLiquidPhase::IndependentClosure
            }
        );
        assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
        assert_eq!(
            failure
                .context
                .owner_id
                .as_ref()
                .map(ResourceOwnerId::as_str),
            Some(owner)
        );
        assert_eq!(failure.context.tile_id.as_ref().map(TileId::as_str), tile);
        let expected_beginning = match owner {
            "land-surface-energy-v1" => digest('2'),
            "soil-thermal" => digest('4'),
            _ => baseline.beginning_hydrology_snapshot_sha256.clone(),
        };
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(expected_beginning.as_str())
        );
        assert_eq!(
            failure.rollback.attempted_owner_sha256.as_deref(),
            Some(attempted.as_str())
        );
    }
}

fn reject_receiver_layer_distribution_poison(
    candidate: &openwepp_hillslope_orchestrator::land_surface_energy_shadow::UnifiedRealHydrologyCandidate,
) {
    let mut wrong_distribution = candidate.receiver_closure_operands().clone();
    let duplicate = wrong_distribution.production_soil[0].ordered_layers[0].clone();
    wrong_distribution.production_soil[0]
        .ordered_layers
        .push(duplicate);
    let failure = validate_real_receiver_closure(&wrong_distribution)
        .expect_err("extra production layer must reject");
    assert_eq!(
        failure.failure().expect("canonical failure").code,
        DirectSurfaceLiquidErrorCode::E010
    );
}

fn reject_receiver_enthalpy_poisons(
    candidate: &openwepp_hillslope_orchestrator::land_surface_energy_shadow::UnifiedRealHydrologyCandidate,
) {
    let infiltration_enthalpy =
        candidate.receiver_closure_operands().soil_thermal[0].infiltration_enthalpy_j_m2_ofe_ground;
    assert_ne!(infiltration_enthalpy.to_bits(), 0.0_f64.to_bits());
    for multiplier in [0.0, 2.0] {
        let mut poison = candidate.receiver_closure_operands().clone();
        let thermal = &mut poison.soil_thermal[0];
        thermal.ending_infiltration_credit_j_m2_ofe_ground = thermal
            .beginning_infiltration_credit_j_m2_ofe_ground
            + multiplier * infiltration_enthalpy;
        thermal.ending_enthalpy_j_m2_ofe_ground =
            thermal.beginning_enthalpy_j_m2_ofe_ground + multiplier * infiltration_enthalpy;
        assert_receiver_e010(
            validate_real_receiver_closure(&poison),
            "soil-thermal",
            Some("open"),
        );
    }

    let retained_enthalpy =
        candidate.receiver_closure_operands().lse_tiles[0].retained_enthalpy_j_m2_ofe_ground;
    assert_ne!(retained_enthalpy.to_bits(), 0.0_f64.to_bits());
    for multiplier in [0.0, 2.0] {
        let mut poison = candidate.receiver_closure_operands().clone();
        let tile = &mut poison.lse_tiles[0];
        tile.ending_enthalpy_j_m2_tile_ground = tile.beginning_enthalpy_j_m2_tile_ground
            + multiplier * retained_enthalpy / tile.tile_fraction;
        assert_receiver_e010(
            validate_real_receiver_closure(&poison),
            "land-surface-energy-v1",
            Some("open"),
        );
    }
}

fn reject_receiver_nonfinite_arithmetic_poisons(
    candidate: &openwepp_hillslope_orchestrator::land_surface_energy_shadow::UnifiedRealHydrologyCandidate,
) {
    let mut thermal_overflow = candidate.receiver_closure_operands().clone();
    thermal_overflow.soil_thermal[0].beginning_infiltration_credit_j_m2_ofe_ground = f64::MAX;
    thermal_overflow.soil_thermal[0].beginning_enthalpy_j_m2_ofe_ground = f64::MAX;
    thermal_overflow.soil_thermal[0].infiltration_enthalpy_j_m2_ofe_ground = f64::MAX;
    assert_receiver_e003(
        "thermal-overflow",
        &thermal_overflow,
        &digest('4'),
        validate_real_receiver_closure(&thermal_overflow),
    );

    let mut lse_overflow = candidate.receiver_closure_operands().clone();
    lse_overflow.lse_tiles[0].beginning_enthalpy_j_m2_tile_ground = f64::MAX;
    lse_overflow.lse_tiles[0].retained_enthalpy_j_m2_ofe_ground = f64::MAX;
    assert_receiver_e003(
        "lse-overflow",
        &lse_overflow,
        &digest('2'),
        validate_real_receiver_closure(&lse_overflow),
    );

    let mut lse_underflow = candidate.receiver_closure_operands().clone();
    lse_underflow.lse_tiles[0].tile_fraction = f64::MAX;
    lse_underflow.lse_tiles[0].retained_enthalpy_j_m2_ofe_ground = f64::MIN_POSITIVE;
    assert_receiver_e003(
        "lse-underflow",
        &lse_underflow,
        &digest('2'),
        validate_real_receiver_closure(&lse_underflow),
    );

    let mut precedence = candidate.receiver_closure_operands().clone();
    precedence.production_soil[0].ending_aggregate_soil_water_m += 1.0;
    precedence.lse_tiles[0].tile_fraction = f64::NAN;
    assert_receiver_e003(
        "later-domain-outranks-earlier-equation",
        &precedence,
        &digest('2'),
        validate_real_receiver_closure(&precedence),
    );

    let mut structural_precedence = candidate.receiver_closure_operands().clone();
    structural_precedence.production_soil.clear();
    structural_precedence.lse_tiles[0].beginning_enthalpy_j_m2_tile_ground = f64::MAX;
    structural_precedence.lse_tiles[0].retained_enthalpy_j_m2_ofe_ground = f64::MAX;
    assert_receiver_e003(
        "derived-overflow-outranks-structural-envelope",
        &structural_precedence,
        &digest('2'),
        validate_real_receiver_closure(&structural_precedence),
    );
}

#[test]
fn invalid_expectation_precedes_receiver_construction_overflow() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let original = frame.clone();
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let mismatched_expectations = UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        digest('9'),
        ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
        snapshot.clone(),
        ResourceOwnerId::try_new("soil-thermal").expect("thermal owner"),
        digest('4'),
        vec![(
            OfeId::try_new("ofe-1").expect("OFE"),
            TileId::try_new("open").expect("tile"),
            vec![SoilLayerId::try_new("thermal-1").expect("layer")],
        )],
    )
    .expect("structurally valid mismatched receiver expectations");
    let callback_called = std::cell::Cell::new(false);
    let ingress = ingress_input_with_mass(50.0);
    let result = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &mismatched_expectations,
        &batch,
        &BTreeMap::new(),
        &ingress,
        |authorizations| {
            callback_called.set(true);
            let finalization =
                unified_finalization(accepted_surface_protocol(&batch, authorizations, &snapshot));
            let mut tiles = finalization.ending_tile_states_pre_ingress().to_vec();
            tiles[0].surface_enthalpy_j_m2_tile_ground = f64::MAX;
            let poisoned = UnifiedLseFinalization::try_new(
                &finalization_expectations(
                    finalization.water_protocol(),
                    finalization.soil_thermal_candidates(),
                ),
                finalization.water_protocol().clone(),
                tiles,
                finalization.soil_thermal_candidates().to_vec(),
                finalization.rollback_hashes().to_vec(),
            )?;
            Ok(poisoned)
        },
    );
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = result.expect_err("finite overflow")
    else {
        panic!("receiver overflow must retain canonical failure");
    };
    let failure = error.failure().expect("failure payload");
    assert!(!callback_called.get());
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E011);
    assert_eq!(
        failure.phase,
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidPhase::AtomicEnvelope,
        "{failure:?}"
    );
    assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("land-surface-energy-v1"),
        "{failure:?}"
    );
    assert_eq!(failure.context.ofe_id, None);
    assert_eq!(failure.context.tile_id, None);
    assert_eq!(failure.context.surface_id, None);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(snapshot.as_str())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());
    assert_eq!(failure.context.source_id, None);
    assert_eq!(failure.context.parcel_id, None);
    assert_eq!(frame, original, "overflow mutated caller owner");
}

#[test]
fn production_aggregate_reconstructs_nonzero_residual_unfrozen_water() {
    let (mut frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let layer = &mut frame.lanes[0].subsurface_layers[0];
    layer.residual_theta = 0.05;
    layer.frozen_depth_m = 0.0;
    let residual_water_m = layer.residual_theta * (layer.depth_m - layer.frozen_depth_m);
    frame.lanes[0].water.soil_water_m = layer.theta_m + residual_water_m;
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let candidate = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input_with_mass(50.0),
        |authorizations| {
            Ok(unified_finalization(accepted_surface_protocol(
                &batch,
                authorizations,
                &snapshot,
            )))
        },
    )
    .expect("nonzero residual receiver candidate");
    let operands = candidate.receiver_closure_operands();
    assert_eq!(
        operands.production_soil[0].ordered_layers[0]
            .residual_theta
            .to_bits(),
        0.05_f64.to_bits()
    );
    validate_real_receiver_closure(operands).expect("residual aggregate closes");

    let mut poison = operands.clone();
    poison.production_soil[0].beginning_aggregate_soil_water_m -= residual_water_m;
    poison.production_soil[0].ending_aggregate_soil_water_m -= residual_water_m;
    assert_receiver_e010(
        validate_real_receiver_closure(&poison),
        "production-hydrology",
        None,
    );
}

fn assert_receiver_e010(
    result: Result<(), openwepp_hillslope_orchestrator::DirectSurfaceLiquidError>,
    owner_id: &str,
    tile_id: Option<&str>,
) {
    let error = result.expect_err("receiver poison must fail");
    let failure = error.failure().expect("canonical receiver failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E010);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
    assert!(failure.context.transaction_id.is_some());
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some(owner_id)
    );
    assert!(failure.context.ofe_id.is_some());
    assert_eq!(
        failure.context.tile_id.as_ref().map(TileId::as_str),
        tile_id
    );
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

fn assert_receiver_e003(
    label: &str,
    operands: &openwepp_hillslope_orchestrator::land_surface_energy_shadow::RealReceiverClosureOperands,
    expected_beginning_sha256: &Sha256Digest,
    result: Result<(), openwepp_hillslope_orchestrator::DirectSurfaceLiquidError>,
) {
    let error = result.expect_err("receiver arithmetic poison must fail");
    let failure = error.failure().expect("canonical receiver failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003, "{label}");
    assert!(failure.context.transaction_id.is_some());
    assert!(failure.context.owner_id.is_some());
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(expected_beginning_sha256.as_str()),
        "{label} beginning rollback hash"
    );
    let attempted = operands.canonical_sha256();
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(attempted.as_str()),
        "{label} attempted rollback hash"
    );
}

fn poison_receiver_finalization(
    finalization: &UnifiedLseFinalization,
    poison: usize,
) -> Result<UnifiedLseFinalization, LandSurfaceEnergyShadowError> {
    let mut tiles = finalization.ending_tile_states_pre_ingress().to_vec();
    let mut thermal = finalization.soil_thermal_candidates().to_vec();
    let mut rollback = finalization.rollback_hashes().to_vec();
    match poison {
        0 => thermal.clear(),
        1 => tiles.clear(),
        2 => rollback[0].after_sha256 = digest('9'),
        3 => tiles.push(tiles[0].clone()),
        4 => thermal.push(thermal[0].clone()),
        5 => rollback.push(rollback[0].clone()),
        6 => rollback.push(OwnerRollbackHash {
            owner_kind: OwnerKind::Envelope,
            owner_id: "unexpected-envelope".into(),
            before_sha256: digest('8'),
            after_sha256: digest('8'),
        }),
        7 => {
            rollback.remove(0);
        }
        8 => {
            let mut extra = thermal[0].layers[0].clone();
            extra.layer_id = SoilLayerId::try_new("thermal-extra").expect("extra layer");
            thermal[0].layers.push(extra);
        }
        9 => thermal[0].layers[0].ending_enthalpy_j_m2_ofe_ground = f64::NAN,
        10 => rollback[0].owner_id = "wrong-lse-owner".into(),
        11 => rollback.swap(0, 1),
        12 => rollback[0].before_sha256 = digest('7'),
        _ => rollback[2].owner_id = "wrong-thermal-owner".into(),
    }
    UnifiedLseFinalization::try_new(
        &finalization_expectations(
            finalization.water_protocol(),
            finalization.soil_thermal_candidates(),
        ),
        finalization.water_protocol().clone(),
        tiles,
        thermal,
        rollback,
    )
}

fn assert_receiver_join_failure(
    error: LandSurfaceEnergyShadowError,
    poison: usize,
    snapshot: &Sha256Digest,
) {
    let LandSurfaceEnergyShadowError::SurfaceLiquid(surface_error) = error else {
        panic!("receiver poison {poison} must retain canonical envelope failure");
    };
    let failure = surface_error.failure().expect("failure payload");
    let expected_code = if poison == 9 {
        DirectSurfaceLiquidErrorCode::E003
    } else {
        DirectSurfaceLiquidErrorCode::E011
    };
    assert_eq!(failure.code, expected_code, "receiver poison {poison}");
    let expected_owner = match poison {
        7 => Some("land-surface-energy-v1"),
        10 => Some("wrong-lse-owner"),
        13 => Some("wrong-thermal-owner"),
        _ => None,
    };
    if let Some(expected_owner) = expected_owner {
        assert_eq!(
            failure
                .context
                .owner_id
                .as_ref()
                .map(ResourceOwnerId::as_str),
            Some(expected_owner)
        );
    }
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(snapshot.as_str()),
        "public callback failure uses the actual unified beginning snapshot"
    );
}

#[test]
fn unified_receiver_join_poisons_return_no_partial_candidate() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let original = frame.clone();
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    for poison in 0..14 {
        let result = execute_unified_real_hydrology_shadow(
            &adapter,
            &configuration,
            &receiver_expectations(1, snapshot.clone()),
            &batch,
            &BTreeMap::new(),
            &ingress_input_with_mass(50.0),
            |authorizations| {
                let finalization = unified_finalization(accepted_surface_protocol(
                    &batch,
                    authorizations,
                    &snapshot,
                ));
                poison_receiver_finalization(&finalization, poison)
            },
        );
        assert_receiver_join_failure(
            result.expect_err("receiver poison must reject"),
            poison,
            &snapshot,
        );
        assert_eq!(frame, original, "receiver poison {poison} mutated owner");
    }
}

#[test]
fn sealed_two_tile_finalization_reports_the_actual_second_thermal_receiver() {
    let (frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let result = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |authorizations| {
            let finalization =
                unified_finalization(accepted_surface_protocol(&batch, authorizations, &snapshot));
            let mut tiles = finalization.ending_tile_states_pre_ingress().to_vec();
            let mut second_lse = tiles[0].clone();
            second_lse.tile_id = TileId::try_new("covered").expect("second LSE tile");
            tiles.push(second_lse);
            let mut thermal = finalization.soil_thermal_candidates().to_vec();
            let mut second_thermal = thermal[0].clone();
            second_thermal.tile_id =
                TileId::try_new("wrong-second").expect("offending thermal tile");
            thermal.push(second_thermal);
            UnifiedLseFinalization::try_new(
                &finalization_expectations(
                    finalization.water_protocol(),
                    finalization.soil_thermal_candidates(),
                ),
                finalization.water_protocol().clone(),
                tiles,
                thermal,
                finalization.rollback_hashes().to_vec(),
            )
        },
    );
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) =
        result.expect_err("second-row receiver mismatch must reject")
    else {
        panic!("second-row receiver mismatch must retain E011");
    };
    let failure = error.failure().expect("canonical receiver failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E011);
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("soil-thermal")
    );
    assert_eq!(
        failure.context.ofe_id.as_ref().map(OfeId::as_str),
        Some("ofe-1")
    );
    assert_eq!(
        failure.context.tile_id.as_ref().map(TileId::as_str),
        Some("wrong-second")
    );
}

#[test]
fn public_bridge_rejects_wrong_second_independent_thermal_expectation_before_callback() {
    let (frame, configuration) = configured_two_tile_surface_frame();
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        open_surface_source_id(&configuration),
        1.0,
    );
    let expectations = UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        digest('2'),
        ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
        snapshot.clone(),
        ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        digest('4'),
        vec![
            (
                configuration.records[0].key.ofe_id.clone(),
                configuration.records[0].key.tile_id.clone(),
                vec![SoilLayerId::try_new("thermal-1").expect("layer")],
            ),
            (
                configuration.records[1].key.ofe_id.clone(),
                TileId::try_new("wrong-second").expect("offending tile"),
                vec![SoilLayerId::try_new("thermal-1").expect("layer")],
            ),
        ],
    )
    .expect("structurally valid independent expectations");
    let mut callback_called = false;
    let result = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &expectations,
        &batch,
        &BTreeMap::new(),
        &two_tile_ingress_input(),
        |authorizations| {
            callback_called = true;
            Ok(two_tile_finalization(accepted_surface_protocol(
                &batch,
                authorizations,
                &snapshot,
            )))
        },
    );
    assert!(
        !callback_called,
        "invalid expectations must precede callback"
    );
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) =
        result.expect_err("second expectation mismatch must reject")
    else {
        panic!("expectation mismatch must retain E011");
    };
    let failure = error.failure().expect("canonical receiver failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E011);
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("soil-thermal")
    );
    assert_eq!(
        failure.context.ofe_id.as_ref().map(OfeId::as_str),
        Some("ofe-1")
    );
    assert_eq!(
        failure.context.tile_id.as_ref().map(TileId::as_str),
        Some("wrong-second")
    );
}

#[test]
fn public_bridge_rejects_deleted_first_independent_thermal_expectation_before_callback() {
    let (frame, configuration) = configured_two_tile_surface_frame();
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration).expect("snapshot");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        open_surface_source_id(&configuration),
        1.0,
    );
    let expectations = UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        digest('2'),
        ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
        snapshot.clone(),
        ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        digest('4'),
        vec![(
            configuration.records[1].key.ofe_id.clone(),
            configuration.records[1].key.tile_id.clone(),
            vec![SoilLayerId::try_new("thermal-1").expect("layer")],
        )],
    )
    .expect("structurally valid deletion poison");
    let attempted_hash = expectations.canonical_sha256();
    let mut callback_called = false;
    let result = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &expectations,
        &batch,
        &BTreeMap::new(),
        &two_tile_ingress_input(),
        |authorizations| {
            callback_called = true;
            Ok(two_tile_finalization(accepted_surface_protocol(
                &batch,
                authorizations,
                &snapshot,
            )))
        },
    );
    assert!(
        !callback_called,
        "invalid expectations must precede callback"
    );
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) =
        result.expect_err("deleted first expectation must reject")
    else {
        panic!("expectation deletion must retain E011");
    };
    let failure = error.failure().expect("canonical receiver failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E011);
    assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("soil-thermal")
    );
    assert_eq!(
        failure.context.ofe_id.as_ref().map(OfeId::as_str),
        Some(configuration.records[0].key.ofe_id.as_str())
    );
    assert_eq!(
        failure.context.tile_id.as_ref().map(TileId::as_str),
        Some(configuration.records[0].key.tile_id.as_str())
    );
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(snapshot.as_str())
    );
    assert_ne!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(attempted_hash.as_str()),
        "public attempted hash must frame more than receiver expectations"
    );
}

fn rust_sources_below(path: &Path, sources: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(path).expect("source directory") {
        let entry = entry.expect("source entry");
        let path = entry.path();
        if path.is_dir() {
            rust_sources_below(&path, sources);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(path);
        }
    }
}

#[test]
fn lse_shadow_has_no_runner_or_production_dispatch_path() {
    let mut sources = Vec::new();
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    rust_sources_below(&workspace.join("crates/openwepp-runner/src"), &mut sources);
    rust_sources_below(
        &workspace.join("crates/openwepp-hillslope-orchestrator/src/direct_runtime"),
        &mut sources,
    );
    for path in sources {
        let source = fs::read_to_string(&path).expect("production source");
        assert!(!source.contains("execute_open_bare_soil_shadow"));
        assert!(!source.contains("LandSurfaceEnergyRealHydrologyAdapter"));
    }
}
