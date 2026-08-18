use super::*;
use openwepp_biogeochemistry::{BiogeochemistryState, MaterialPool, MineralLayer};
use openwepp_hillslope_orchestrator::land_surface_energy_shadow::{
    ExecuteV8LseRuntimeShadowError, LandSurfaceEnergyConfiguration, LandSurfaceEnergyState,
    LandSurfaceForcing, V8CanopyForcingReceipt, execute_v8_lse_runtime_shadow,
};
use openwepp_kernel_contract::{MaterialReceiverClass, OccupancyId, StratumId};
use openwepp_vegetation::carbon_nitrogen::ElementPool;
use openwepp_vegetation::{
    CoupledOwnedState, NitrogenArbiter, NitrogenAuthorization, NitrogenRequest, RootLayer,
    SnowFreeForcing, SoilLayerForcing, TopologyTile, V8_MODEL_SHA256, V8CoupledOwnedState,
    V8OccupancyState, V8TileCanopyAirState, VegetationConfiguration, VegetationError,
};
use std::cell::Cell;

const DT: f64 = 1_800.0;
const COVER: f64 = 0.38;

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

fn configured_vegetation() -> (
    VegetationConfiguration,
    CoupledOwnedState,
    [StratumId; 2],
    TileId,
) {
    let mut cfg: VegetationConfiguration = serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v5_diagnostic_configuration.json")
            .expect("configuration fixture"),
    )
    .expect("configuration");
    let old: CoupledOwnedState = serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v5_diagnostic_state.json").expect("state fixture"),
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
    (cfg, old, ids, forest)
}

fn vegetation() -> (VegetationConfiguration, V8CoupledOwnedState) {
    let (mut cfg, old, ids, forest) = configured_vegetation();
    let mut shared = old.strata.values().next().expect("shared").clone();
    shared.last_transaction_id = 40;
    // The migrated source state is on whole-stand ground. This fixture splits
    // that one stratum evenly across two forest occupancies, so its displayed
    // pools and derived areas must retain that same stand-ground basis.
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
    for p in shared.tissues.values_mut() {
        p.storage = ElementPool::default();
        p.transfer = ElementPool::default();
    }
    let lane = || V8OccupancyState {
        beta_hyd: 0.67,
        canopy_liquid_kg_h2o_m2_tile_ground: 0.018,
        dry_stem_temperature_k: 295.2,
        last_accepted_transaction_id: Some(40),
        root_node_potential_mm: -2850.0,
        shade_ci_pa: 30.0,
        shade_leaf_potential_mm: -5450.0,
        shade_leaf_temperature_k: 295.4,
        stem_potential_mm: -4300.0,
        sun_ci_pa: 30.0,
        sun_leaf_potential_mm: -5900.0,
        sun_leaf_temperature_k: 296.2,
        wet_surface_temperature_k: 295.6,
    };
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
                lane(),
            ),
            (
                OccupancyId {
                    stratum_id: ids[1].clone(),
                    tile_id: forest.clone(),
                },
                lane(),
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

fn surface() -> (DirectSurfaceLiquidConfiguration, DirectRunFrame) {
    surface_for_tiles(&[("forest", COVER, true), ("open", 1.0 - COVER, false)])
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
        precipitation_parcels: Vec::new(),
        runon_parcels: Vec::new(),
    };
    forcing.forcing_sha256 = forcing.canonical_sha256().expect("forcing digest");
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
        .map(|kind| (kind, MaterialPool::default()))
        .collect(),
        last_transaction_id: 40,
    }
}

#[derive(Clone, Copy)]
struct PositiveExpectations<'a> {
    tiles: &'a [&'a str],
    canopy_forcing_ground_albedo: Option<(f64, f64)>,
}

fn execute_positive(
    veg_cfg: &VegetationConfiguration,
    veg_state: &V8CoupledOwnedState,
    surface_cfg: &DirectSurfaceLiquidConfiguration,
    frame: &DirectRunFrame,
    cfg: &LandSurfaceEnergyConfiguration,
    state: &LandSurfaceEnergyState,
    expected: PositiveExpectations<'_>,
) {
    let hyd = RealHydrologyShadowAdapter::try_from_day_start(
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
            .map(|x| SoilLayerId::try_new(x).expect("layer"))
            .to_vec(),
        }],
    )
    .expect("hydrology");
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hyd);
    let forcing = lse_forcing();
    let th = full_thermal();
    let mut snow = snow_forcing(&hyd);
    if let Some((visible, near_infrared)) = expected.canopy_forcing_ground_albedo {
        snow.ground_albedo_vis = visible;
        snow.ground_albedo_nir = near_infrared;
    }
    let receipt = V8CanopyForcingReceipt::try_new(
        veg_cfg.configuration_sha256.clone(),
        veg_state.state_sha256.clone(),
        cfg.configuration_sha256.clone(),
        forcing.forcing_sha256.clone(),
        unified_beginning_hydrology_snapshot_sha256(&adapter, surface_cfg).expect("snapshot"),
        th.snapshot_sha256.clone(),
        TransactionId(41),
        snow,
    )
    .expect("receipt");
    let n = FullNitrogen(Cell::new(0));
    let bgc = biogeochemistry();
    let before = frame.clone();
    let envelope = execute_v8_lse_runtime_shadow(
        veg_cfg,
        veg_state,
        &ResourceOwnerId::try_new("vegetation-v8").expect("owner"),
        &receipt,
        cfg,
        state,
        &forcing,
        &adapter,
        surface_cfg,
        0,
        0,
        &[DirectOfeWb14Parameters {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            effective_conductivity_m_s: 1e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.04,
        }],
        &th,
        &n,
        &bgc,
    )
    .expect("public endpoint");
    assert_eq!(frame, &before);
    assert_eq!(n.0.get(), u32::from(!veg_cfg.strata.is_empty()));
    envelope.validate().expect("sealed envelope");
    let requests = &envelope.hydrology().arbitration().requests;
    for tile in expected.tiles {
        assert!(
            requests
                .iter()
                .any(|row| row.key.requesting_tile_id.as_str() == *tile),
            "missing authorization request for {tile}"
        );
    }
}

fn lse_state(cfg: &LandSurfaceEnergyConfiguration) -> LandSurfaceEnergyState {
    let mut state = LandSurfaceEnergyState {
        model_definition_sha256: cfg.model_definition_sha256.clone(),
        configuration_sha256: cfg.configuration_sha256.clone(),
        state_sha256: digest('0'),
        owner_id: cfg.owner_id.clone(),
        last_accepted_transaction_id: None,
        tiles: cfg
            .ofes
            .iter()
            .flat_map(|ofe| {
                ofe.tiles.iter().map(move |tile| {
                    openwepp_hillslope_orchestrator::land_surface_energy_shadow::TileState {
                        ofe_id: ofe.ofe_id.clone(),
                        tile_id: tile.tile_id.clone(),
                        surface_enthalpy_j_m2_tile_ground: if tile.tile_id.as_str() == "open" {
                            917_700.0
                        } else {
                            439_352.808
                        },
                        surface_temperature_warm_start_k: 295.0,
                    }
                })
            })
            .collect(),
    };
    state.state_sha256 = state.canonical_sha256().expect("state digest");
    state.validate(cfg).expect("LSE state");
    state
}

fn surface_record(
    tile: &str,
    fraction: f64,
    covered: bool,
) -> DirectSurfaceLiquidConfigurationRecord {
    DirectSurfaceLiquidConfigurationRecord {
        key: DirectSurfaceLiquidStoreKey {
            run_id: 83,
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            tile_id: TileId::try_new(tile).expect("tile"),
            surface_id: SurfaceId::try_new(format!("surface:ofe-1:{tile}")).expect("surface"),
            surface_class: if covered {
                SurfaceClass::ForestLitter
            } else {
                SurfaceClass::BareMineralSoil
            },
            source_type: if covered {
                WaterSourceType::LitterLiquid
            } else {
                WaterSourceType::SurfaceLiquid
            },
            source_id: SourceId::try_new(format!("liquid:ofe-1:{tile}")).expect("source"),
        },
        tile_fraction: fraction,
        capacity_kg_m2_tile: if covered { 6.0 } else { 3.0 },
        ofe_area_m2: 100.0,
        ground_ingress_mode: if covered {
            DirectGroundIngressMode::CoveredCanopyRelease
        } else {
            DirectGroundIngressMode::OpenRawPrecipitation
        },
        runon_destination_ofe_id: None,
        runon_destination_tile_id: None,
    }
}

fn surface_for_tiles(
    tiles: &[(&str, f64, bool)],
) -> (DirectSurfaceLiquidConfiguration, DirectRunFrame) {
    let layers = [
        "thermal-1",
        "thermal-2",
        "soil-1",
        "soil-2",
        "soil-dry",
        "soil-frozen",
    ];
    let records = tiles
        .iter()
        .map(|(tile, fraction, covered)| surface_record(tile, *fraction, *covered))
        .collect();
    let cfg = DirectSurfaceLiquidConfiguration::new(
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        83,
        vec![OfeId::try_new("ofe-1").expect("OFE")],
        vec![DirectSurfaceLiquidOfeBinding {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            production_lane_index: 0,
            production_lane_id: 1,
            ordered_soil_layer_ids: layers
                .map(|id| SoilLayerId::try_new(id).expect("layer"))
                .to_vec(),
            infiltration_soil_thermal_layer_id: SoilLayerId::try_new("thermal-1").expect("layer"),
        }],
        records,
    )
    .expect("surface configuration");
    let mut frame =
        DirectRunFrame::skeleton(DirectRunIdentity::new(83, 11, 1, 1).expect("identity"))
            .expect("frame");
    frame.lanes[0].area_m2 = 100.0;
    frame.lanes[0].subsurface_layers = (0..6)
        .map(|_| DirectSubsurfaceLayerState {
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
        })
        .collect();
    frame.lanes[0].water.soil_water_m = frame.lanes[0]
        .subsurface_layers
        .iter()
        .map(|layer| layer.theta_m)
        .sum();
    let initial = cfg
        .records
        .iter()
        .map(|record| {
            (
                record.key.clone(),
                if record.key.surface_class == SurfaceClass::ForestLitter {
                    4.0
                } else {
                    0.0
                },
            )
        })
        .collect();
    let state =
        DirectSurfaceLiquidOwnedState::new_initial(&cfg, &initial, 0).expect("surface state");
    frame
        .configure_surface_liquid_shadow(&cfg, state)
        .expect("surface owner");
    (cfg, frame)
}

fn all_open_vegetation() -> (VegetationConfiguration, V8CoupledOwnedState) {
    let (mut cfg, mut state) = vegetation();
    cfg.topology_tiles = vec![TopologyTile {
        tile_id: TileId::try_new("open").expect("tile"),
        fraction: 1.0,
    }];
    cfg.strata.clear();
    cfg.configuration_sha256.clear();
    cfg.initial_state_sha256 = "0".repeat(64);
    cfg.configuration_sha256 = cfg.canonical_sha256().expect("configuration digest");
    state
        .configuration_sha256
        .clone_from(&cfg.configuration_sha256);
    state.occupancies.clear();
    state.strata.clear();
    state.tile_canopy_air.clear();
    state.state_sha256 = state.canonical_sha256();
    cfg.initial_state_sha256.clone_from(&state.state_sha256);
    cfg.validate_v8()
        .expect("all-open vegetation configuration");
    state.validate(&cfg).expect("all-open vegetation state");
    (cfg, state)
}

fn shared_stratum_two_covered() -> (VegetationConfiguration, V8CoupledOwnedState) {
    let (mut cfg, mut state) = vegetation();
    let tile_a = TileId::try_new("forest-a").expect("tile");
    let tile_b = TileId::try_new("forest-b").expect("tile");
    cfg.topology_tiles = vec![
        TopologyTile {
            tile_id: tile_a.clone(),
            fraction: 0.5,
        },
        TopologyTile {
            tile_id: tile_b.clone(),
            fraction: 0.5,
        },
    ];
    cfg.strata.truncate(1);
    cfg.strata[0].tile_ids = vec![tile_a.clone(), tile_b.clone()];
    let stratum_id = cfg.strata[0].stratum_id.clone();
    let lane = state
        .occupancies
        .values()
        .next()
        .expect("occupancy")
        .clone();
    let shared = state
        .strata
        .get(&stratum_id)
        .expect("shared stratum")
        .clone();
    let canopy_air = state
        .tile_canopy_air
        .values()
        .next()
        .expect("canopy air")
        .clone();
    cfg.configuration_sha256.clear();
    cfg.initial_state_sha256 = "0".repeat(64);
    cfg.configuration_sha256 = cfg.canonical_sha256().expect("configuration digest");
    state
        .configuration_sha256
        .clone_from(&cfg.configuration_sha256);
    state.occupancies = BTreeMap::from([
        (
            OccupancyId {
                stratum_id: stratum_id.clone(),
                tile_id: tile_a.clone(),
            },
            lane.clone(),
        ),
        (
            OccupancyId {
                stratum_id: stratum_id.clone(),
                tile_id: tile_b.clone(),
            },
            lane,
        ),
    ]);
    state.strata = BTreeMap::from([(stratum_id, shared)]);
    state.tile_canopy_air = BTreeMap::from([(tile_a, canopy_air.clone()), (tile_b, canopy_air)]);
    state.state_sha256 = state.canonical_sha256();
    cfg.initial_state_sha256.clone_from(&state.state_sha256);
    cfg.validate_v8().expect("shared-stratum configuration");
    state.validate(&cfg).expect("shared-stratum state");
    (cfg, state)
}

fn select_lse_tiles(
    mut cfg: LandSurfaceEnergyConfiguration,
    tiles: &[(&str, f64, bool)],
) -> LandSurfaceEnergyConfiguration {
    let covered_template = cfg.ofes[0].tiles[0].clone();
    let open_template = cfg.ofes[0].tiles[1].clone();
    cfg.ofes[0].tiles = tiles
        .iter()
        .map(|(id, fraction, covered)| {
            let mut tile = if *covered {
                covered_template.clone()
            } else {
                open_template.clone()
            };
            tile.tile_id = TileId::try_new(*id).expect("tile");
            tile.vegetation_tile_id = tile.tile_id.clone();
            tile.fraction_ofe_ground = *fraction;
            tile
        })
        .collect();
    cfg.configuration_sha256 = cfg.canonical_sha256().expect("LSE configuration digest");
    cfg.validate().expect("selected LSE topology");
    cfg
}

#[test]
fn public_v8_endpoint_closes_mixed_open_and_covered_owners_once() {
    let (veg_cfg, veg_state) = vegetation();
    let (surface_cfg, frame) = surface();
    let cfg = lse_cfg(&veg_cfg);
    let state = lse_state(&cfg);
    execute_positive(
        &veg_cfg,
        &veg_state,
        &surface_cfg,
        &frame,
        &cfg,
        &state,
        PositiveExpectations {
            tiles: &["open", "forest"],
            canopy_forcing_ground_albedo: None,
        },
    );
}

#[test]
fn public_v8_endpoint_closes_an_all_open_ofe() {
    let (veg_cfg, veg_state) = all_open_vegetation();
    let (surface_cfg, frame) = surface_for_tiles(&[("open", 1.0, false)]);
    let cfg = select_lse_tiles(lse_cfg(&veg_cfg), &[("open", 1.0, false)]);
    let state = lse_state(&cfg);
    execute_positive(
        &veg_cfg,
        &veg_state,
        &surface_cfg,
        &frame,
        &cfg,
        &state,
        PositiveExpectations {
            tiles: &["open"],
            canopy_forcing_ground_albedo: None,
        },
    );
}

#[test]
fn public_v8_endpoint_closes_one_shared_stratum_across_two_covered_tiles() {
    let (veg_cfg, veg_state) = shared_stratum_two_covered();
    let tiles = [("forest-a", 0.5, true), ("forest-b", 0.5, true)];
    let (surface_cfg, frame) = surface_for_tiles(&tiles);
    let cfg = select_lse_tiles(lse_cfg(&veg_cfg), &tiles);
    let state = lse_state(&cfg);
    execute_positive(
        &veg_cfg,
        &veg_state,
        &surface_cfg,
        &frame,
        &cfg,
        &state,
        PositiveExpectations {
            tiles: &["forest-a", "forest-b"],
            canopy_forcing_ground_albedo: None,
        },
    );
}

#[test]
fn per_tile_lse_ground_optics_are_the_sole_covered_lower_boundary_owner() {
    let (veg_cfg, veg_state) = shared_stratum_two_covered();
    let tiles = [("forest-a", 0.5, true), ("forest-b", 0.5, true)];
    let (surface_cfg, frame) = surface_for_tiles(&tiles);
    let mut cfg = select_lse_tiles(lse_cfg(&veg_cfg), &tiles);
    cfg.ofes[0].tiles[1].surface_vis_albedo = 0.21;
    cfg.ofes[0].tiles[1].surface_nir_albedo = 0.37;
    cfg.configuration_sha256 = cfg.canonical_sha256().expect("LSE configuration digest");
    cfg.validate().expect("heterogeneous covered optics");
    let state = lse_state(&cfg);

    execute_positive(
        &veg_cfg,
        &veg_state,
        &surface_cfg,
        &frame,
        &cfg,
        &state,
        PositiveExpectations {
            tiles: &["forest-a", "forest-b"],
            canopy_forcing_ground_albedo: Some((0.63, 0.71)),
        },
    );
}

#[derive(Clone, Copy, Debug)]
enum EndpointPoison {
    LseConfigurationBit,
    LseStateBit,
    VegetationConfigurationBit,
    VegetationStateBit,
    HydrologySnapshotBit,
    SoilThermalSnapshotBit,
    DuplicateRank,
    MissingOccupancy,
    ExtraOccupancy,
    MissingTile,
    ExtraTile,
}

#[allow(clippy::too_many_lines)]
fn execute_poison(poison: EndpointPoison) -> ExecuteV8LseRuntimeShadowError {
    let (mut veg_cfg, mut veg_state) = vegetation();
    let (surface_cfg, frame) = surface();
    let mut cfg = lse_cfg(&veg_cfg);
    let mut state = lse_state(&cfg);
    let mut thermal = full_thermal();
    match poison {
        EndpointPoison::LseConfigurationBit => {
            cfg.ofes[0].tiles[0].surface_vis_albedo =
                f64::from_bits(cfg.ofes[0].tiles[0].surface_vis_albedo.to_bits() + 1);
        }
        EndpointPoison::LseStateBit => {
            state.tiles[0].surface_temperature_warm_start_k =
                f64::from_bits(state.tiles[0].surface_temperature_warm_start_k.to_bits() + 1);
        }
        EndpointPoison::VegetationConfigurationBit => {
            veg_cfg.strata[0].height_m = f64::from_bits(veg_cfg.strata[0].height_m.to_bits() + 1);
        }
        EndpointPoison::VegetationStateBit => {
            let lane = veg_state
                .occupancies
                .values_mut()
                .next()
                .expect("occupancy");
            lane.beta_hyd = f64::from_bits(lane.beta_hyd.to_bits() + 1);
        }
        EndpointPoison::DuplicateRank => {
            veg_cfg.strata[1].vertical_rank = veg_cfg.strata[0].vertical_rank;
            veg_cfg.configuration_sha256.clear();
            veg_cfg.configuration_sha256 = veg_cfg.canonical_sha256().expect("poison digest");
            veg_state
                .configuration_sha256
                .clone_from(&veg_cfg.configuration_sha256);
            veg_state.state_sha256 = veg_state.canonical_sha256();
        }
        EndpointPoison::MissingOccupancy => {
            let key = veg_state
                .occupancies
                .keys()
                .next()
                .expect("occupancy")
                .clone();
            veg_state.occupancies.remove(&key);
            veg_state.state_sha256 = veg_state.canonical_sha256();
        }
        EndpointPoison::ExtraOccupancy => {
            let (key, lane) = veg_state.occupancies.iter().next().expect("occupancy");
            let mut extra = key.clone();
            extra.tile_id = TileId::try_new("open").expect("tile");
            veg_state.occupancies.insert(extra, lane.clone());
            veg_state.state_sha256 = veg_state.canonical_sha256();
        }
        EndpointPoison::MissingTile => {
            cfg.ofes[0].tiles.pop();
            cfg.ofes[0].tiles[0].fraction_ofe_ground = 1.0;
            cfg.configuration_sha256 = cfg.canonical_sha256().expect("poison digest");
            state = lse_state(&cfg);
        }
        EndpointPoison::ExtraTile => {
            let mut extra = cfg.ofes[0].tiles[1].clone();
            extra.tile_id = TileId::try_new("extra-open").expect("tile");
            extra.vegetation_tile_id = extra.tile_id.clone();
            extra.fraction_ofe_ground = 0.1;
            cfg.ofes[0].tiles[0].fraction_ofe_ground -= 0.05;
            cfg.ofes[0].tiles[1].fraction_ofe_ground -= 0.05;
            cfg.ofes[0].tiles.push(extra);
            cfg.configuration_sha256 = cfg.canonical_sha256().expect("poison digest");
            state = lse_state(&cfg);
        }
        EndpointPoison::SoilThermalSnapshotBit => {
            thermal.snapshot_sha256 = digest('7');
        }
        EndpointPoison::HydrologySnapshotBit => {}
    }

    let hyd = RealHydrologyShadowAdapter::try_from_day_start(
        &frame,
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
    .expect("hydrology owner");
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hyd);
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
        precipitation_parcels: Vec::new(),
        runon_parcels: Vec::new(),
    };
    forcing.forcing_sha256 = forcing.canonical_sha256().expect("forcing digest");
    let facts = hyd.layer_facts();
    let snow = SnowFreeForcing {
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
                temperature_k: if id == "soil-frozen" { 270.0 } else { 293.0 },
                frozen: fact.frozen,
            }
        })
        .collect(),
        gsi: 1.0,
    };
    let actual_hydrology =
        unified_beginning_hydrology_snapshot_sha256(&adapter, &surface_cfg).expect("snapshot");
    let receipt_hydrology = if matches!(poison, EndpointPoison::HydrologySnapshotBit) {
        digest('b')
    } else {
        actual_hydrology
    };
    let receipt_thermal = if matches!(poison, EndpointPoison::SoilThermalSnapshotBit) {
        digest('6')
    } else {
        thermal.snapshot_sha256.clone()
    };
    let receipt = V8CanopyForcingReceipt::try_new(
        veg_cfg.configuration_sha256.clone(),
        veg_state.state_sha256.clone(),
        cfg.configuration_sha256.clone(),
        forcing.forcing_sha256.clone(),
        receipt_hydrology,
        receipt_thermal,
        TransactionId(41),
        snow,
    )
    .expect("poison receipt");
    let nitrogen = FullNitrogen(Cell::new(0));
    let bgc = BiogeochemistryState {
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
        .map(|kind| (kind, MaterialPool::default()))
        .collect(),
        last_transaction_id: 40,
    };
    execute_v8_lse_runtime_shadow(
        &veg_cfg,
        &veg_state,
        &ResourceOwnerId::try_new("vegetation-v8").expect("owner"),
        &receipt,
        &cfg,
        &state,
        &forcing,
        &adapter,
        &surface_cfg,
        0,
        0,
        &[DirectOfeWb14Parameters {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            effective_conductivity_m_s: 1e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.04,
        }],
        &thermal,
        &nitrogen,
        &bgc,
    )
    .expect_err("public endpoint poison must fail before physics")
}

#[test]
fn public_endpoint_rejects_canonical_seam_poisons_before_physics() {
    for poison in [
        EndpointPoison::LseConfigurationBit,
        EndpointPoison::LseStateBit,
        EndpointPoison::VegetationConfigurationBit,
        EndpointPoison::VegetationStateBit,
        EndpointPoison::HydrologySnapshotBit,
        EndpointPoison::SoilThermalSnapshotBit,
        EndpointPoison::DuplicateRank,
        EndpointPoison::MissingOccupancy,
        EndpointPoison::ExtraOccupancy,
        EndpointPoison::MissingTile,
        EndpointPoison::ExtraTile,
    ] {
        let error = execute_poison(poison);
        assert!(
            matches!(error, ExecuteV8LseRuntimeShadowError::Projection(_)),
            "{poison:?} returned non-prephysics error: {error:?}"
        );
    }
}

#[test]
fn migrated_two_rank_fixture_preserves_stand_ground_leaf_owners() {
    let (configuration, state) = vegetation();
    for occupancy in state.occupancies.keys() {
        let shared = &state.strata[&occupancy.stratum_id];
        assert_eq!(
            shared
                .tissues
                .get(&openwepp_vegetation::carbon_nitrogen::Tissue::Leaf)
                .expect("leaf tissue")
                .display
                .nitrogen
                .to_bits(),
            (0.003 * (COVER / 2.0)).to_bits()
        );
        assert_eq!(
            shared.leaf_area.to_bits(),
            (shared
                .tissues
                .get(&openwepp_vegetation::carbon_nitrogen::Tissue::Leaf)
                .expect("leaf tissue")
                .display
                .carbon
                * configuration.strata[0].sla_m2_per_kg_c)
                .to_bits()
        );
    }
}

#[test]
fn public_endpoint_source_excludes_caller_derived_physics() {
    let source=fs::read_to_string("crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/strict_v8_endpoint.rs").expect("source");
    let signature = source
        .split("pub fn execute_v8_lse_runtime_shadow(")
        .nth(1)
        .and_then(|x| x.split(") -> Result").next())
        .expect("signature");
    for forbidden in [
        "CoveredColumnInputs",
        "OpenSurfaceProblem",
        "beginning_trial",
        "V8ComponentOccupancyBinding",
        "CoveredIngressSchedule",
        "DirectTileGroundIngress",
    ] {
        assert!(
            !signature.contains(forbidden),
            "public endpoint exposes {forbidden}"
        );
    }
}

#[test]
fn canonical_configuration_and_state_bits_fail_closed() {
    let (vegetation_configuration, mut vegetation_state) = vegetation();
    let mut lse = lse_cfg(&vegetation_configuration);
    lse.ofes[0].tiles[0].surface_vis_albedo =
        f64::from_bits(lse.ofes[0].tiles[0].surface_vis_albedo.to_bits() + 1);
    assert!(lse.validate().is_err(), "one-bit LSE configuration poison");

    let occupancy = vegetation_state
        .occupancies
        .values_mut()
        .next()
        .expect("occupancy");
    occupancy.beta_hyd = f64::from_bits(occupancy.beta_hyd.to_bits() + 1);
    assert!(
        vegetation_state
            .validate(&vegetation_configuration)
            .is_err(),
        "one-bit V8 state poison"
    );
}

#[test]
fn rank_and_tile_topology_poisons_are_rejected_by_canonical_owners() {
    let (mut vegetation_configuration, _) = vegetation();
    vegetation_configuration.strata[1].vertical_rank = 0;
    vegetation_configuration.configuration_sha256.clear();
    vegetation_configuration.configuration_sha256 = vegetation_configuration
        .canonical_sha256()
        .expect("poison digest");
    assert!(vegetation_configuration.validate_v8().is_err());

    let (clean, _) = vegetation();
    let mut lse = lse_cfg(&clean);
    lse.ofes[0].tiles[0].fraction_ofe_ground = 0.39;
    lse.configuration_sha256 = lse.canonical_sha256().expect("poison digest");
    assert!(lse.validate().is_err(), "tile-fraction topology poison");
}
