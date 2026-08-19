//! Exact-owner rollback tests for `SC-LANDSURFACEENERGY-001` and
//! `SC-SURFACELIQUID-001`.
#![cfg_attr(not(test), allow(dead_code, unused_imports))]

use std::collections::BTreeMap;
use std::fs;

use crate::land_surface_energy_shadow::{
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError, OfeId, Sha256Digest,
    SoilThermalLayerSnapshot, SoilThermalOfeSnapshot, SoilThermalSnapshot, SourceId, SurfaceClass,
    SurfaceId, V8RollbackSnapshot, WaterSourceType, unified_beginning_hydrology_snapshot_sha256,
};
use crate::vegetation_real_hydrology_shadow::{
    RealHydrologyLaneLayerMap, RealHydrologyOfeLaneId, RealHydrologyShadowAdapter,
    RealHydrologySourceKey,
};
use crate::{
    DirectGroundIngressMode, DirectOfeWb14Parameters, DirectRunFrame, DirectRunIdentity,
    DirectSubsurfaceLayerState, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidConfigurationRecord, DirectSurfaceLiquidOfeBinding,
    DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidStoreKey,
};
use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};

use super::*;
use crate::land_surface_energy_shadow::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyState, LandSurfaceForcing,
    V8CanopyForcingReceipt,
};
use openwepp_biogeochemistry::{BiogeochemistryState, MaterialPool, MineralLayer};
use openwepp_kernel_contract::{MaterialReceiverClass, OccupancyId, StratumId};
use openwepp_vegetation::carbon_nitrogen::ElementPool;
use openwepp_vegetation::{
    CoupledOwnedState, NitrogenArbiter, NitrogenAuthorization, NitrogenRequest, RootLayer,
    SnowFreeForcing, SoilLayerForcing, StratumSharedState, TopologyTile, V8_MODEL_SHA256,
    V8CoupledOwnedState, V8OccupancyState, V8TileCanopyAirState, VegetationConfiguration,
    VegetationError,
};
use std::cell::Cell;

const DT: f64 = 1_800.0;
const COVER: f64 = 0.38;

fn digest(character: char) -> Sha256Digest {
    Sha256Digest::try_new(character.to_string().repeat(64)).expect("digest")
}

fn occupancy_lane() -> V8OccupancyState {
    V8OccupancyState {
        beta_hyd: 0.67,
        canopy_liquid_kg_h2o_m2_tile_ground: 0.018,
        dry_stem_temperature_k: 295.2,
        last_accepted_transaction_id: Some(40),
        root_node_potential_mm: -2_850.0,
        shade_ci_pa: 30.0,
        shade_leaf_potential_mm: -5_450.0,
        shade_leaf_temperature_k: 295.4,
        stem_potential_mm: -4_300.0,
        sun_ci_pa: 30.0,
        sun_leaf_potential_mm: -5_900.0,
        sun_leaf_temperature_k: 296.2,
        wet_surface_temperature_k: 295.6,
    }
}

fn migrate_shared_stratum(
    old: &CoupledOwnedState,
    cfg: &VegetationConfiguration,
) -> StratumSharedState {
    let mut shared = old.strata.values().next().expect("shared").clone();
    shared.last_transaction_id = 40;
    let occupancy_share = COVER / 2.0;
    for pool in shared.tissues.values_mut() {
        pool.display.carbon *= occupancy_share;
        pool.display.nitrogen *= occupancy_share;
    }
    let displayed_leaf_carbon = shared
        .tissues
        .get(&openwepp_vegetation::carbon_nitrogen::Tissue::Leaf)
        .expect("leaf tissue")
        .display
        .carbon;
    shared.leaf_area = displayed_leaf_carbon * cfg.strata[0].sla_m2_per_kg_c;
    shared.stem_area = shared.leaf_area * cfg.strata[0].sai_relation;
    shared.root_area = (shared.leaf_area + shared.stem_area) * cfg.strata[0].root_to_leaf_area;
    shared.pending_transfers.clear();
    for pool in shared.tissues.values_mut() {
        pool.storage = ElementPool::default();
        pool.transfer = ElementPool::default();
    }
    shared
}

struct FullNitrogen(Cell<u32>);
impl NitrogenArbiter for FullNitrogen {
    fn beginning_amount(
        &self,
        _: &openwepp_kernel_contract::MineralNitrogenKey,
    ) -> Result<f64, VegetationError> {
        Ok(1.0)
    }
    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
        self.0.set(self.0.get() + 1);
        Ok(requests
            .iter()
            .map(|r| openwepp_kernel_contract::MaximumAuthorization {
                transaction_id: r.transaction_id,
                owner_id: r.owner_id.clone(),
                key: r.key.clone(),
                amount: r.amount,
                basis: r.basis,
            })
            .collect())
    }
}

fn vegetation() -> (VegetationConfiguration, V8CoupledOwnedState) {
    let mut cfg: VegetationConfiguration = serde_json::from_slice(
        &fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/c3_woody_v5_diagnostic_configuration.json"
        ))
        .expect("configuration fixture"),
    )
    .expect("configuration");
    let old: CoupledOwnedState = serde_json::from_slice(
        &fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/c3_woody_v5_diagnostic_state.json"
        ))
        .expect("state fixture"),
    )
    .expect("state");
    let forest = TileId::try_new("forest").expect("tile");
    let ids = [
        StratumId::try_new("stratum-z-upper").expect("stratum"),
        StratumId::try_new("stratum-a-lower").expect("stratum"),
    ];
    let roots = [
        ("soil-1", 0.62),
        ("soil-2", 0.38),
        ("soil-dry", 0.0),
        ("soil-frozen", 0.0),
    ]
    .map(|(id, f)| RootLayer {
        layer_id: SoilLayerId::try_new(id).expect("layer"),
        root_fraction: f,
        mineral_n_root_fraction: f,
        lateral_root_length_m: 0.2,
    })
    .to_vec();
    let mut upper = cfg.strata[0].clone();
    upper.stratum_id = ids[0].clone();
    upper.vertical_rank = 0;
    upper.tile_ids = vec![forest.clone()];
    upper.height_m = 12.5;
    upper.current_growth_fraction = 1.0;
    upper.root_layers.clone_from(&roots);
    let mut lower = upper.clone();
    lower.stratum_id = ids[1].clone();
    lower.vertical_rank = 1;
    lower.height_m = 8.0;
    lower.crown_base_m = 1.5;
    lower.displacement_m = 4.8;
    cfg.model_definition_sha256 = V8_MODEL_SHA256.into();
    cfg.dt_s = DT;
    cfg.topology_tiles = vec![
        TopologyTile {
            tile_id: forest.clone(),
            fraction: COVER,
        },
        TopologyTile {
            tile_id: TileId::try_new("open").expect("tile"),
            fraction: 1.0 - COVER,
        },
    ];
    cfg.strata = vec![upper, lower];
    cfg.configuration_sha256.clear();
    cfg.initial_state_sha256 = "0".repeat(64);
    cfg.configuration_sha256 = cfg.canonical_sha256().expect("digest");
    // Preserve the migrated whole-stand-ground basis across both occupancies.
    let shared = migrate_shared_stratum(&old, &cfg);
    let mut state = V8CoupledOwnedState {
        configuration_sha256: cfg.configuration_sha256.clone(),
        last_transaction_id: 40,
        model_definition_sha256: V8_MODEL_SHA256.into(),
        occupancies: BTreeMap::from([
            (
                OccupancyId {
                    stratum_id: ids[0].clone(),
                    tile_id: forest.clone(),
                },
                occupancy_lane(),
            ),
            (
                OccupancyId {
                    stratum_id: ids[1].clone(),
                    tile_id: forest.clone(),
                },
                occupancy_lane(),
            ),
        ]),
        state_sha256: String::new(),
        strata: BTreeMap::from([(ids[0].clone(), shared.clone()), (ids[1].clone(), shared)]),
        tile_canopy_air: BTreeMap::from([(
            forest,
            V8TileCanopyAirState {
                canopy_air_specific_humidity_kg_kg: 0.011,
                canopy_air_temperature_k: 295.8,
            },
        )]),
    };
    state.state_sha256 = state.canonical_sha256();
    cfg.initial_state_sha256.clone_from(&state.state_sha256);
    cfg.validate_v8().expect("V8 config");
    state.validate(&cfg).expect("V8 state");
    (cfg, state)
}

fn subsurface_layer() -> DirectSubsurfaceLayerState {
    DirectSubsurfaceLayerState {
        theta_m: 0.02,
        field_capacity_m: 0.02,
        upper_limit_m: 0.2,
        conductivity_m_s: 1e-6,
        depth_m: 0.3,
        residual_theta: 0.0,
        frozen_depth_m: 0.0,
        frozen_water_m: 0.0,
        porosity: 0.45,
        field_capacity_theta: 0.25,
        coca: 0.1,
        lateral_conductivity_m_s: 1e-7,
    }
}

fn surface() -> (DirectSurfaceLiquidConfiguration, DirectRunFrame) {
    let row =
        |tile: &str, fraction, class, source_type, mode| DirectSurfaceLiquidConfigurationRecord {
            key: DirectSurfaceLiquidStoreKey {
                run_id: 83,
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                tile_id: TileId::try_new(tile).expect("tile"),
                surface_id: SurfaceId::try_new(format!("surface:ofe-1:{tile}")).expect("surface"),
                surface_class: class,
                source_type,
                source_id: SourceId::try_new(format!("liquid:ofe-1:{tile}")).expect("source"),
            },
            tile_fraction: fraction,
            capacity_kg_m2_tile: if tile == "forest" { 6.0 } else { 3.0 },
            ofe_area_m2: 100.0,
            ground_ingress_mode: mode,
            runon_destination_ofe_id: None,
            runon_destination_tile_id: None,
        };
    let layers = [
        "thermal-1",
        "thermal-2",
        "soil-1",
        "soil-2",
        "soil-dry",
        "soil-frozen",
    ];
    let cfg = DirectSurfaceLiquidConfiguration::new(
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        83,
        vec![OfeId::try_new("ofe-1").expect("OFE")],
        vec![DirectSurfaceLiquidOfeBinding {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            production_lane_index: 0,
            production_lane_id: 1,
            ordered_soil_layer_ids: layers
                .map(|x| SoilLayerId::try_new(x).expect("layer"))
                .to_vec(),
            infiltration_soil_thermal_layer_id: SoilLayerId::try_new("thermal-1").expect("layer"),
        }],
        vec![
            row(
                "forest",
                COVER,
                SurfaceClass::ForestLitter,
                WaterSourceType::LitterLiquid,
                DirectGroundIngressMode::CoveredCanopyRelease,
            ),
            row(
                "open",
                1.0 - COVER,
                SurfaceClass::BareMineralSoil,
                WaterSourceType::SurfaceLiquid,
                DirectGroundIngressMode::OpenRawPrecipitation,
            ),
        ],
    )
    .expect("surface config");
    let mut frame =
        DirectRunFrame::skeleton(DirectRunIdentity::new(83, 11, 1, 1).expect("identity"))
            .expect("frame");
    frame.lanes[0].area_m2 = 100.0;
    frame.lanes[0].subsurface_layers = (0..6).map(|_| subsurface_layer()).collect();
    frame.lanes[0].water.soil_water_m = frame.lanes[0]
        .subsurface_layers
        .iter()
        .map(|layer| layer.theta_m)
        .sum();
    let initial = cfg
        .records
        .iter()
        .map(|r| {
            (
                r.key.clone(),
                if r.key.tile_id.as_str() == "forest" {
                    4.0
                } else {
                    0.0
                },
            )
        })
        .collect();
    let state = DirectSurfaceLiquidOwnedState::new_initial(&cfg, &initial, 0).expect("state");
    frame
        .configure_surface_liquid_shadow(&cfg, state)
        .expect("owner");
    (cfg, frame)
}

fn lse_cfg(veg: &VegetationConfiguration) -> LandSurfaceEnergyConfiguration {
    let tile = |id: &str,
                fraction: f64,
                vegetation: &str,
                turbulence: serde_json::Value,
                surface: serde_json::Value,
                vis: f64,
                nir: f64| serde_json::json!({"tile_id":id,"fraction_ofe_ground":fraction,"vegetation_tile_id":vegetation,"surface_vis_albedo":vis,"surface_nir_albedo":nir,"surface_heat_storage_mode":"finite_capacity","turbulence":turbulence,"surface":surface});
    let mut cfg:LandSurfaceEnergyConfiguration=serde_json::from_value(serde_json::json!({"model_version":"OPENWEPP_SNOW_FREE_LSE_V1","model_definition_sha256":"e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f","configuration_sha256":digest('0'),"owner_id":"land-surface-energy-v1","vegetation_configuration":{"owner_id":"vegetation-v8","model_version":"OPENWEPP_C3_WOODY_V8","model_definition_sha256":V8_MODEL_SHA256,"configuration_sha256":veg.configuration_sha256},"hydrology_configuration":{"owner_id":"production-hydrology","model_version":"direct-v1","model_definition_sha256":digest('8'),"configuration_sha256":digest('9')},"soil_thermal_configuration":{"owner_id":"soil-thermal","model_version":"thermal-v1","model_definition_sha256":digest('a'),"configuration_sha256":digest('5')},"numerics":{"iteration_limit":50,"backtracking_exponents":(0..=20).collect::<Vec<_>>(),"finite_difference":"centered_sqrt_binary64_epsilon_minus_then_plus","pivot_threshold":"64_times_binary64_epsilon_times_matrix_infinity_norm","equal_pivot_rule":"lowest_row","temperature_bounds_k":[200,350],"humidity_bounds_kg_kg":[0.0,0.1],"temperature_step_tolerance_k":1e-8,"humidity_step_tolerance_kg_kg":1e-12,"hydraulic_step_tolerance_mm":1e-7,"beta_step_tolerance":1e-10},"ofes":[{"ofe_id":"ofe-1","area_m2":100.0,"soil_interface_layers":[{"layer_id":"thermal-1","thickness_m":0.08,"thermal_conductivity_w_m_k":1.1,"areal_heat_capacity_j_m2_k":120_000.0},{"layer_id":"thermal-2","thickness_m":0.18,"thermal_conductivity_w_m_k":1.35,"areal_heat_capacity_j_m2_k":180_000.0}],"tiles":[tile("forest",COVER,"forest",serde_json::json!({"mode":"covered_neutral","canopy_height_m":12.5,"ground_exchange_roughness_m":0.08,"leaf_area_index_m2_m2_tile_ground":2.0,"canopy_to_reference":{"reference_height_m":24.0,"displacement_m":4.8,"roughness_momentum_m":1.25,"roughness_heat_m":0.12,"roughness_vapor_m":0.08}}),serde_json::json!({"surface_class":"forest_litter","liquid_capacity_kg_m2_tile_ground":6.0,"thickness_m":0.04,"dry_density_kg_m3":24.0,"dry_specific_heat_j_kg_k":3370.5}),0.12,0.24),tile("open",1.0-COVER,"open",serde_json::json!({"mode":"open_neutral","reference_height_m":24.0,"roughness_momentum_m":0.12,"roughness_heat_m":0.015,"roughness_vapor_m":0.01}),serde_json::json!({"surface_class":"bare_mineral_soil","dry_areal_heat_capacity_j_m2_k":42000.0,"mineral_skin_thickness_m":0.02,"mineral_skin_thermal_conductivity_w_m_k":0.75,"top_layer_saturated_water_content_m3_m3":0.46,"top_layer_porosity_m3_m3":0.46,"top_layer_saturated_matric_potential_mm":-120.0,"top_layer_clapp_hornberger_b":4.05,"top_layer_initial_water_content_m3_m3":0.22}),0.18,0.31)]}]})).expect("LSE config");
    for id in ["soil-1", "soil-2", "soil-dry", "soil-frozen"] {
        let mut layer = cfg.ofes[0].soil_interface_layers[1].clone();
        layer.layer_id = SoilLayerId::try_new(id).expect("layer");
        cfg.ofes[0].soil_interface_layers.push(layer);
    }
    cfg.configuration_sha256 = cfg.canonical_sha256().expect("digest");
    cfg.validate().expect("valid config");
    cfg
}

fn thermal() -> SoilThermalSnapshot {
    SoilThermalSnapshot {
        owner_id: ResourceOwnerId::try_new("soil-thermal").expect("owner"),
        configuration_sha256: digest('5'),
        state_sha256: digest('4'),
        snapshot_sha256: digest('6'),
        last_accepted_transaction_id: None,
        ofes: vec![SoilThermalOfeSnapshot {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            ordered_layers: vec![
                SoilThermalLayerSnapshot {
                    layer_id: SoilLayerId::try_new("thermal-1").expect("layer"),
                    temperature_k: 291.5,
                    enthalpy_j_m2_ofe_ground: 1e6,
                },
                SoilThermalLayerSnapshot {
                    layer_id: SoilLayerId::try_new("thermal-2").expect("layer"),
                    temperature_k: 289.8,
                    enthalpy_j_m2_ofe_ground: 2e6,
                },
            ],
        }],
    }
}

fn full_thermal() -> SoilThermalSnapshot {
    let mut snapshot = thermal();
    for (id, temperature_k) in [
        ("soil-1", 293.0),
        ("soil-2", 293.0),
        ("soil-dry", 293.0),
        ("soil-frozen", 270.0),
    ] {
        snapshot.ofes[0]
            .ordered_layers
            .push(SoilThermalLayerSnapshot {
                layer_id: SoilLayerId::try_new(id).expect("layer"),
                temperature_k,
                enthalpy_j_m2_ofe_ground: 2.0e6,
            });
    }
    snapshot
}

fn hydrology(frame: &DirectRunFrame) -> RealHydrologyShadowAdapter {
    RealHydrologyShadowAdapter::try_from_day_start(
        frame,
        0,
        TransactionId(41),
        DT,
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        &[RealHydrologyLaneLayerMap {
            ofe_lane: RealHydrologyOfeLaneId {
                lane_index: 0,
                lane_id: 1,
            },
            layer_ids: [
                "thermal-1",
                "thermal-2",
                "soil-1",
                "soil-2",
                "soil-dry",
                "soil-frozen",
            ]
            .map(|id| SoilLayerId::try_new(id).expect("layer"))
            .to_vec(),
        }],
    )
    .expect("hydrology")
}

fn lse_state(cfg: &LandSurfaceEnergyConfiguration) -> LandSurfaceEnergyState {
    let mut state: LandSurfaceEnergyState = serde_json::from_value(serde_json::json!({
        "model_definition_sha256": cfg.model_definition_sha256,
        "configuration_sha256": cfg.configuration_sha256,
        "state_sha256": digest('0'),
        "owner_id": cfg.owner_id,
        "last_accepted_transaction_id": null,
        "tiles": [
            {"ofe_id":"ofe-1","tile_id":"forest","surface_enthalpy_j_m2_tile_ground":439_352.808,"surface_temperature_warm_start_k":295.0},
            {"ofe_id":"ofe-1","tile_id":"open","surface_enthalpy_j_m2_tile_ground":917_700.0,"surface_temperature_warm_start_k":295.0}
        ]
    }))
    .expect("state");
    state.state_sha256 = state.canonical_sha256().expect("digest");
    state
}

fn lse_forcing() -> LandSurfaceForcing {
    let mut forcing = LandSurfaceForcing {
        forcing_sha256: digest('0'),
        transaction_id: TransactionId(41),
        interval_s: DT,
        air_temperature_k: 296.0,
        air_specific_humidity_kg_kg: 0.0102,
        air_pressure_pa: 101_325.0,
        reference_wind_m_s: 3.7,
        neutral_stability: true,
        snow_present_at_beginning: false,
        snow_present_at_end: false,
        snow_terminal_payload_present: false,
        direct_vis_w_m2: 410.0,
        diffuse_vis_w_m2: 83.0,
        direct_nir_w_m2: 355.0,
        diffuse_nir_w_m2: 101.0,
        atmospheric_downward_longwave_w_m2: 395.0,
        precipitation_parcels: vec![],
        runon_parcels: vec![],
    };
    forcing.forcing_sha256 = forcing.canonical_sha256().expect("digest");
    forcing
}

fn snow_forcing(hydrology: &RealHydrologyShadowAdapter) -> SnowFreeForcing {
    let facts = hydrology.layer_facts();
    SnowFreeForcing {
        air_temperature_k: 296.0,
        pressure_pa: 101_325.0,
        co2_pa: 42.0,
        vapor_pressure_deficit_kpa: 1.0,
        wind_m_s: 3.7,
        rain_kg_m2: 0.0,
        direct_par_w_m2: 410.0,
        diffuse_par_w_m2: 83.0,
        direct_nir_w_m2: 355.0,
        diffuse_nir_w_m2: 101.0,
        solar_zenith_cosine: 0.67,
        ground_albedo_vis: 0.12,
        ground_albedo_nir: 0.24,
        longwave_down_w_m2: 395.0,
        longwave_up_w_m2: 0.0,
        specific_humidity: 0.0102,
        reference_height_m: 24.0,
        soil_layers: [
            "thermal-1",
            "thermal-2",
            "soil-1",
            "soil-2",
            "soil-dry",
            "soil-frozen",
        ]
        .into_iter()
        .map(|id| {
            let key = RealHydrologySourceKey {
                ofe_lane: RealHydrologyOfeLaneId {
                    lane_index: 0,
                    lane_id: 1,
                },
                layer_id: SoilLayerId::try_new(id).expect("layer"),
            };
            let fact = &facts[&key];
            SoilLayerForcing {
                layer_id: key.layer_id,
                accessible: id != "soil-dry",
                water_beginning_kg_m2: fact.liquid_supply_kg_m2,
                matric_potential_mm: 100.0,
                hydraulic_conductivity_mm_s: 0.006,
                root_path_length_mm: 200.0,
                gravity_root_mm: 120.0,
                temperature_k: match id {
                    "thermal-1" => 291.5,
                    "thermal-2" => 289.8,
                    "soil-frozen" => 270.0,
                    _ => 293.0,
                },
                frozen: fact.frozen,
            }
        })
        .collect(),
        gsi: 1.0,
    }
}

fn biogeochemistry() -> BiogeochemistryState {
    BiogeochemistryState {
        layers: ["soil-1", "soil-2", "soil-dry", "soil-frozen"]
            .into_iter()
            .map(|id| {
                (
                    id.into(),
                    MineralLayer {
                        ammonium_n: 1.0,
                        nitrate_n: 1.0,
                    },
                )
            })
            .collect(),
        receivers: [
            MaterialReceiverClass::Metabolic,
            MaterialReceiverClass::Cellulose,
            MaterialReceiverClass::Lignin,
            MaterialReceiverClass::CoarseWoodyDebris,
        ]
        .into_iter()
        .map(|class| (class, MaterialPool::default()))
        .collect(),
        last_transaction_id: 40,
    }
}

pub struct EndpointFixture {
    pub vegetation_configuration: VegetationConfiguration,
    pub vegetation_state: V8CoupledOwnedState,
    pub surface_configuration: DirectSurfaceLiquidConfiguration,
    pub hydrology: RealHydrologyShadowAdapter,
    pub lse_configuration: LandSurfaceEnergyConfiguration,
    pub lse_state: LandSurfaceEnergyState,
    pub forcing: LandSurfaceForcing,
    pub thermal: SoilThermalSnapshot,
    pub receipt: V8CanopyForcingReceipt,
    pub biogeochemistry: BiogeochemistryState,
}

pub fn endpoint_fixture() -> EndpointFixture {
    let (vegetation_configuration, vegetation_state) = vegetation();
    let (surface_configuration, frame) = surface();
    let hydrology = hydrology(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hydrology);
    let lse_configuration = lse_cfg(&vegetation_configuration);
    let lse_state = lse_state(&lse_configuration);
    let forcing = lse_forcing();
    let thermal = full_thermal();
    let receipt = V8CanopyForcingReceipt::try_new(
        vegetation_configuration.configuration_sha256.clone(),
        vegetation_state.state_sha256.clone(),
        lse_configuration.configuration_sha256.clone(),
        forcing.forcing_sha256.clone(),
        unified_beginning_hydrology_snapshot_sha256(&adapter, &surface_configuration)
            .expect("snapshot"),
        thermal.snapshot_sha256.clone(),
        TransactionId(41),
        snow_forcing(&hydrology),
    )
    .expect("receipt");
    EndpointFixture {
        vegetation_configuration,
        vegetation_state,
        surface_configuration,
        hydrology,
        lse_configuration,
        lse_state,
        forcing,
        thermal,
        receipt,
        biogeochemistry: biogeochemistry(),
    }
}

const fn failure_phases() -> [V8EndpointFailureInjection; 19] {
    [
        V8EndpointFailureInjection::AfterProjection,
        V8EndpointFailureInjection::AfterSolverReady,
        V8EndpointFailureInjection::AfterPotentialTile(0),
        V8EndpointFailureInjection::AfterPotentialTile(1),
        V8EndpointFailureInjection::AfterCombinedRequests,
        V8EndpointFailureInjection::AfterAuthorization,
        V8EndpointFailureInjection::AfterFinalTile(0),
        V8EndpointFailureInjection::AfterFinalTile(1),
        V8EndpointFailureInjection::AfterE04Ingress,
        V8EndpointFailureInjection::AfterOpenIngress,
        V8EndpointFailureInjection::AfterUnifiedHydrology,
        V8EndpointFailureInjection::AfterLocalEnergy,
        V8EndpointFailureInjection::AfterOfeEnergy,
        V8EndpointFailureInjection::AfterV8Receipts,
        V8EndpointFailureInjection::AfterPersistentPhase,
        V8EndpointFailureInjection::AfterVegetationCandidate,
        V8EndpointFailureInjection::AfterBiogeochemistryCandidate,
        V8EndpointFailureInjection::AfterEnvelopeValidation,
        V8EndpointFailureInjection::BeforeReturn,
    ]
}

#[test]
fn every_injected_phase_preserves_all_six_actual_owner_byte_records() {
    let fixture = endpoint_fixture();
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&fixture.hydrology);
    let n = FullNitrogen(Cell::new(0));
    let vegetation_owner_id = ResourceOwnerId::try_new("vegetation-v8").expect("owner");
    let biogeochemistry_owner_id = ResourceOwnerId::try_new("biogeochemistry").expect("owner");
    let pending_owner_id = ResourceOwnerId::try_new("strict-v8-pending-envelopes").expect("owner");
    for phase in failure_phases() {
        let beginning = V8RollbackSnapshot::capture_endpoint_beginning(
            &vegetation_owner_id,
            &fixture.vegetation_state,
            &adapter,
            &fixture.lse_state,
            &fixture.thermal,
            &biogeochemistry_owner_id,
            &fixture.biogeochemistry,
            &pending_owner_id,
            &[],
            &[],
            &[],
        )
        .expect("beginning rollback snapshot");
        let error = execute_v8_lse_runtime_shadow_internal(
            &fixture.vegetation_configuration,
            &fixture.vegetation_state,
            &vegetation_owner_id,
            &fixture.receipt,
            &fixture.lse_configuration,
            &fixture.lse_state,
            &fixture.forcing,
            &adapter,
            &fixture.surface_configuration,
            0,
            0,
            &[DirectOfeWb14Parameters {
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                effective_conductivity_m_s: 1e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 0.04,
            }],
            &fixture.thermal,
            &n,
            &fixture.biogeochemistry,
            Some(phase),
            openwepp_land_surface_energy::CoveredColumnAuthority::HistoricalV8,
        )
        .expect_err("injected endpoint failure");
        match phase {
            V8EndpointFailureInjection::AfterProjection
            | V8EndpointFailureInjection::AfterSolverReady
            | V8EndpointFailureInjection::AfterV8Receipts
            | V8EndpointFailureInjection::BeforeReturn => assert!(matches!(
                error,
                ExecuteV8LseRuntimeShadowError::Identity(
                    "test-injected strict V8 endpoint phase failure"
                )
            )),
            V8EndpointFailureInjection::AfterPersistentPhase
            | V8EndpointFailureInjection::AfterVegetationCandidate
            | V8EndpointFailureInjection::AfterBiogeochemistryCandidate
            | V8EndpointFailureInjection::AfterEnvelopeValidation => assert!(matches!(
                error,
                ExecuteV8LseRuntimeShadowError::Owner(CoveredV8OwnerEnvelopeError::Identity(
                    "test-injected strict V8 owner phase failure"
                ))
            )),
            _ => assert!(matches!(
                error,
                ExecuteV8LseRuntimeShadowError::Physical(LandSurfaceEnergyShadowError::Identity(
                    "test-injected strict V8 multi-tile phase failure"
                ))
            )),
        }
        let after = V8RollbackSnapshot::capture_endpoint_beginning(
            &vegetation_owner_id,
            &fixture.vegetation_state,
            &adapter,
            &fixture.lse_state,
            &fixture.thermal,
            &biogeochemistry_owner_id,
            &fixture.biogeochemistry,
            &pending_owner_id,
            &[],
            &[],
            &[],
        )
        .expect("post-failure rollback snapshot");
        assert_eq!(beginning.owners().len(), 6);
        assert_eq!(beginning.owners(), after.owners(), "phase {phase:?}");
        beginning
            .check_snapshot(&after)
            .unwrap_or_else(|error| panic!("phase {phase:?} rollback: {error}"));
    }
}
