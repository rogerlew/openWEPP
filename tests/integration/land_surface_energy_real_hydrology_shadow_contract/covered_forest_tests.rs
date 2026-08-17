use super::*;
use openwepp_biogeochemistry::{BiogeochemistryState, MaterialPool, MineralLayer};
use openwepp_hillslope_orchestrator::DirectSurfaceLiquidParcelKind;
use openwepp_hillslope_orchestrator::land_surface_energy_shadow::{
    BiochemicalConstants, CoveredColumnInputs, CoveredForestShadowResult, CoveredOccupancyInputs,
    LeafBiochemicalInputs, RootHydraulicLayer, RootRuntimeIdentity, UnderCanopyGeometry,
    WaterAmount, execute_covered_forest_shadow, execute_covered_v8_transaction,
    solve_covered_potential_phase,
};
use openwepp_kernel_contract::{
    MaterialReceiverClass, MaximumAuthorization, OccupancyId, StratumId,
};
use openwepp_land_surface_energy::{CoveredColumnShortwaveInputs, CoveredOccupancyShortwaveInputs};
use openwepp_vegetation::carbon_nitrogen::ElementPool;
use openwepp_vegetation::{
    CoupledOwnedState, NitrogenArbiter, NitrogenAuthorization, NitrogenRequest, RootLayer,
    TopologyTile, V8_MODEL_SHA256, V8ComponentOccupancyBinding, V8CoupledOwnedState,
    V8LseComponentId, V8OccupancyState, V8PersistentForcingReceipt, V8TileCanopyAirState,
    VegetationConfiguration, VegetationError,
};
use std::cell::Cell;

const INTERVAL_S: f64 = 1_800.0;
const TILE_FRACTION: f64 = 0.38;

fn biochemical() -> BiochemicalConstants {
    BiochemicalConstants {
        ha_vcmax_j_mol: 65_330.0,
        hd_vcmax_j_mol: 200_000.0,
        entropy_vcmax_j_mol_k: 650.0,
        ha_jmax_j_mol: 43_540.0,
        hd_jmax_j_mol: 200_000.0,
        entropy_jmax_j_mol_k: 650.0,
        kc25_pa: 40.49,
        ha_kc_j_mol: 79_430.0,
        ko25_pa: 27_840.0,
        ha_ko_j_mol: 36_380.0,
        gamma25_pa: 4.275,
        ha_gamma_j_mol: 37_830.0,
        oxygen_partial_pressure_pa: 20_265.0,
        tp_vcmax_ratio: 0.167,
        electron_quantum_yield: 0.85,
        par_photon_umol_per_j: 4.6,
        electron_curvature: 0.7,
        ac_aj_curvature: 0.98,
        ag_ap_curvature: 0.95,
    }
}

fn roots() -> Vec<RootHydraulicLayer> {
    vec![
        RootHydraulicLayer {
            layer_id: "soil-1".into(),
            accessible: true,
            frozen: false,
            root_fraction: 0.62,
            soil_potential_mm: 100.0,
            gravity_head_mm: 120.0,
            z3_m: 0.32,
            dxroot_m: 0.18,
            ksoil_m2_s: 6.0e-11,
        },
        RootHydraulicLayer {
            layer_id: "soil-2".into(),
            accessible: true,
            frozen: false,
            root_fraction: 0.38,
            soil_potential_mm: 100.0,
            gravity_head_mm: 360.0,
            z3_m: 0.55,
            dxroot_m: 0.24,
            ksoil_m2_s: 4.5e-11,
        },
        RootHydraulicLayer {
            layer_id: "soil-dry".into(),
            accessible: false,
            frozen: false,
            root_fraction: 0.0,
            soil_potential_mm: -9_000.0,
            gravity_head_mm: 600.0,
            z3_m: 0.8,
            dxroot_m: 0.31,
            ksoil_m2_s: 2.0e-7,
        },
        RootHydraulicLayer {
            layer_id: "soil-frozen".into(),
            accessible: true,
            frozen: true,
            root_fraction: 0.0,
            soil_potential_mm: -1_100.0,
            gravity_head_mm: 740.0,
            z3_m: 1.1,
            dxroot_m: 0.4,
            ksoil_m2_s: 1.0e-7,
        },
    ]
}

fn upper() -> CoveredOccupancyInputs {
    CoveredOccupancyInputs {
        occupancy_id: "canopy-rank-0".into(),
        medlyn_g1_kpa_sqrt: 3.5,
        g0_umol_m2_s: 25.0,
        sun: LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: 1.110_267_869_704_946_6,
            absorbed_shortwave_w_m2_tile: 219.583_484_232_463_2,
            absorbed_par_w_m2_leaf: 136.097_574_782_013_34,
            vcmax25: 62.0,
            jmax25: 108.0,
            rd25: 1.15,
        },
        shade: LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: 1.598_065_463_628_386_4,
            absorbed_shortwave_w_m2_tile: 297.182_430_346_421_4,
            absorbed_par_w_m2_leaf: 116.714_147_486_897_5,
            vcmax25: 41.0,
            jmax25: 74.0,
            rd25: 0.81,
        },
        biochemical: biochemical(),
        stem_area_m2_m2_tile: 0.72,
        stem_absorbed_shortwave_w_m2_tile: 183.772_038_359_786_73,
        beginning_canopy_liquid_kg_m2_tile: 0.018,
        liquid_interception_fraction: 0.35,
        liquid_capacity_kg_m2_plant: 0.023_328_503_368_824_437,
        stemflow_fraction: 0.08,
        gb_leaf_m_s: 0.035_961_386_715_575_215,
        gb_wet_m_s: 0.019_071_405_305_591_295,
        gb_stem_m_s: 0.013_082_876_106_352_972,
        lai: 2.708_333_333_333_333,
        sai: 0.72,
        clumping_index: 0.82,
        k1_max: 1.2e-6,
        k2_max: 4.2e-6,
        k3_max_m_s: 5.0e-5,
        stem_to_leaf_path_m: 1.0,
        height_m: 12.5,
        root_to_leaf_area: 1.8,
        p50_leaf_mm: -9_800.0,
        p50_xylem_mm: -7_200.0,
        p50_root_mm: -14_000.0,
        vulnerability_exponent: 2.0,
        root_layers: roots(),
    }
}

fn distinct_bands(total: f64, seed: f64) -> BandDirectionalFluxes {
    let direct_vis = total * (0.10 + seed);
    let diffuse_vis = total * (0.17 - seed / 2.0);
    let direct_nir = total * (0.29 + seed / 3.0);
    BandDirectionalFluxes {
        direct_vis,
        diffuse_vis,
        direct_nir,
        diffuse_nir: total - direct_vis - diffuse_vis - direct_nir,
    }
}

fn bound_shortwave(
    occupancies: &[CoveredOccupancyInputs],
    terminal: BandDirectionalFluxes,
    surface_vis_albedo: f64,
    surface_nir_albedo: f64,
) -> CoveredColumnShortwaveInputs {
    let rows = occupancies
        .iter()
        .enumerate()
        .map(|(index, occupancy)| CoveredOccupancyShortwaveInputs {
            occupancy_id: occupancy.occupancy_id.clone(),
            sun_leaf_absorbed_w_m2_tile: distinct_bands(
                occupancy.sun.absorbed_shortwave_w_m2_tile,
                0.01 * index as f64,
            ),
            shade_leaf_absorbed_w_m2_tile: distinct_bands(
                occupancy.shade.absorbed_shortwave_w_m2_tile,
                0.02 + 0.01 * index as f64,
            ),
            stem_absorbed_w_m2_tile: distinct_bands(
                occupancy.stem_absorbed_shortwave_w_m2_tile,
                0.04 + 0.01 * index as f64,
            ),
        })
        .collect::<Vec<_>>();
    let top_reflected = BandDirectionalFluxes {
        direct_vis: 7.0,
        diffuse_vis: 11.0,
        direct_nir: 13.0,
        diffuse_nir: 17.0,
    };
    let values = |bands: BandDirectionalFluxes| {
        [
            bands.direct_vis,
            bands.diffuse_vis,
            bands.direct_nir,
            bands.diffuse_nir,
        ]
    };
    let ground_absorbed = BandDirectionalFluxes {
        direct_vis: terminal.direct_vis * (1.0 - surface_vis_albedo),
        diffuse_vis: terminal.diffuse_vis * (1.0 - surface_vis_albedo),
        direct_nir: terminal.direct_nir * (1.0 - surface_nir_albedo),
        diffuse_nir: terminal.diffuse_nir * (1.0 - surface_nir_albedo),
    };
    let mut incident = values(top_reflected);
    let ground_absorbed_values = values(ground_absorbed);
    for index in 0..4 {
        incident[index] += ground_absorbed_values[index]
            + rows
                .iter()
                .map(|row| {
                    values(row.sun_leaf_absorbed_w_m2_tile)[index]
                        + values(row.shade_leaf_absorbed_w_m2_tile)[index]
                        + values(row.stem_absorbed_w_m2_tile)[index]
                })
                .sum::<f64>();
    }
    CoveredColumnShortwaveInputs {
        incident_w_m2_tile: BandDirectionalFluxes {
            direct_vis: incident[0],
            diffuse_vis: incident[1],
            direct_nir: incident[2],
            diffuse_nir: incident[3],
        },
        top_reflected_w_m2_tile: top_reflected,
        ground_absorbed_by_incident_w_m2_tile: ground_absorbed,
        occupancies: rows,
    }
}

fn column() -> CoveredColumnInputs {
    let mut lower = upper();
    lower.occupancy_id = "canopy-rank-1".into();
    lower.sun.leaf_area_m2_m2_tile = 0.869_597_990_586_524_9;
    lower.sun.absorbed_shortwave_w_m2_tile = 36.606_943_691_269_41;
    lower.sun.absorbed_par_w_m2_leaf = 21.581_281_690_559_077;
    lower.shade.leaf_area_m2_m2_tile = 0.701_235_342_746_808_2;
    lower.shade.absorbed_shortwave_w_m2_tile = 29.370_268_258_774_185;
    lower.shade.absorbed_par_w_m2_leaf = 19.137_976_248_584_64;
    lower.stem_area_m2_m2_tile = 0.417_6;
    lower.stem_absorbed_shortwave_w_m2_tile = 23.961_096_147_421_54;
    lower.lai = 1.570_833_333_333_333;
    lower.sai = 0.417_6;
    lower.liquid_capacity_kg_m2_plant = 0.040_221_557_532_455_925;
    lower.clumping_index = 0.91;
    let occupancies = vec![upper(), lower];
    let terminal = BandDirectionalFluxes {
        direct_vis: 12.572_362_927_904_654,
        diffuse_vis: 2.794_652_935_170_348_4,
        direct_nir: 10.885_826_437_575_982,
        diffuse_nir: 20.063_182_822_663_31,
    };
    CoveredColumnInputs {
        interval_s: INTERVAL_S,
        tile_fraction: TILE_FRACTION,
        pressure_pa: 101_325.0,
        air_temperature_k: 296.0,
        air_specific_humidity_kg_kg: 0.0102,
        reference_wind_m_s: 3.7,
        atmospheric_downward_longwave_w_m2: 395.0,
        ca_pa: 42.0,
        canopy_to_atmosphere_heat_resistance_s_m: 20.992_293_151_292_14,
        canopy_to_atmosphere_vapor_resistance_s_m: 22.734_132_598_127_985,
        latent_heat_j_kg: 2_501_000.0,
        top_rain_kg_m2_tile: 0.0,
        under_canopy_geometry: UnderCanopyGeometry {
            canopy_height_m: 12.5,
            canopy_roughness_m: 1.25,
            reference_height_m: 24.0,
            leaf_area_index: 2.708_333_333_333_333,
        },
        ground: OpenSurfaceProblem {
            interval_s: INTERVAL_S,
            tile_fraction: TILE_FRACTION,
            class: SurfaceClassKind::ForestLitter,
            storage_branch: SurfaceStorageBranch::FiniteCapacity,
            terminal_shortwave_w_m2_tile: terminal,
            surface_vis_albedo: 0.12,
            surface_nir_albedo: 0.24,
            surface_emissivity: 1.0,
            surface_depth_m: 0.04,
            surface_conductivity_w_m_k: 0.103,
            surface_dry_heat_capacity_j_m2_k: 3_235.68,
            litter_capacity_kg_m2_tile: Some(6.0),
            open_geometry: OpenNeutralGeometry {
                reference_height_m: 24.0,
                roughness_momentum_m: 1.25,
                roughness_heat_m: 0.12,
                roughness_vapor_m: 0.08,
            },
            air_temperature_k: 296.0,
            air_specific_humidity_kg_kg: 0.0102,
            air_pressure_pa: 101_325.0,
            reference_wind_m_s: 3.7,
            atmospheric_downward_longwave_w_m2: 395.0,
            surface_liquid_kg_m2_tile: 4.0,
            surface_enthalpy_j_m2_tile: 439_352.808_000_000_5,
            surface_temperature_warm_start_k: 295.0,
            bare_soil: None,
            soil_nodes: vec![
                SoilThermalNodeOperands {
                    layer_id: "thermal-1".into(),
                    depth_m: 0.08,
                    conductivity_w_m_k: 1.1,
                    heat_capacity_j_m2_k: 120_000.0,
                    beginning_temperature_k: 291.5,
                },
                SoilThermalNodeOperands {
                    layer_id: "thermal-2".into(),
                    depth_m: 0.18,
                    conductivity_w_m_k: 1.35,
                    heat_capacity_j_m2_k: 180_000.0,
                    beginning_temperature_k: 289.8,
                },
            ],
        },
        shortwave: bound_shortwave(&occupancies, terminal, 0.12, 0.24),
        occupancies,
    }
}

fn trial() -> Vec<f64> {
    vec![
        -5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66, 296.2, 295.4, 295.6, 295.2, -5_900.0,
        -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66, 295.5, 295.0, 295.6, 295.2, 295.8, 0.011, 295.0,
        291.5, 289.8,
    ]
}

fn root_runtime_identities() -> Vec<RootRuntimeIdentity> {
    ["canopy-rank-0", "canopy-rank-1"]
        .into_iter()
        .flat_map(|occupancy| {
            ["soil-1", "soil-2", "soil-dry", "soil-frozen"].map(move |layer| {
                RootRuntimeIdentity {
                    solver_occupancy_id: occupancy.into(),
                    requesting_owner_id: ResourceOwnerId::try_new("vegetation-v8")
                        .expect("vegetation owner"),
                    occupancy_id: openwepp_hillslope_orchestrator::land_surface_energy_shadow::ComponentId::try_new(occupancy)
                        .expect("occupancy"),
                    layer_id: SoilLayerId::try_new(layer).expect("layer"),
                    source_id: SourceId::try_new(layer).expect("source"),
                }
            })
        })
        .collect()
}

fn identity(snapshot: Sha256Digest, source_id: SourceId) -> RuntimeTileIdentity {
    RuntimeTileIdentity {
        transaction_id: TransactionId(41),
        lse_owner_id: ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology")
            .expect("hydrology owner"),
        soil_thermal_owner_id: ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        configuration_sha256: digest('1'),
        beginning_lse_state_sha256: digest('2'),
        beginning_hydrology_snapshot_sha256: snapshot,
        beginning_soil_thermal_state_sha256: digest('4'),
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        tile_id: TileId::try_new("forest").expect("tile"),
        surface_id: SurfaceId::try_new("surface:ofe-1:forest").expect("surface"),
        surface_class: SurfaceClass::ForestLitter,
        ground_source_type: WaterSourceType::LitterLiquid,
        ground_source_id: source_id,
        ground_source_tile_id: Some(TileId::try_new("forest").expect("source tile")),
        ground_soil_layer_id: None,
        tile_fraction: TILE_FRACTION,
        interval_s: INTERVAL_S,
    }
}

struct CountingFullNitrogen {
    calls: Cell<u32>,
}

impl NitrogenArbiter for CountingFullNitrogen {
    fn beginning_amount(
        &self,
        _key: &openwepp_kernel_contract::MineralNitrogenKey,
    ) -> Result<f64, VegetationError> {
        Ok(1.0)
    }

    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
        self.calls.set(self.calls.get() + 1);
        Ok(requests
            .iter()
            .map(|request| MaximumAuthorization {
                transaction_id: request.transaction_id,
                owner_id: request.owner_id.clone(),
                key: request.key.clone(),
                amount: request.amount,
                basis: request.basis,
            })
            .collect())
    }
}

#[allow(clippy::too_many_lines)]
fn aligned_v8_vegetation_fixture() -> (
    VegetationConfiguration,
    V8CoupledOwnedState,
    Vec<V8ComponentOccupancyBinding>,
    V8PersistentForcingReceipt,
    BiogeochemistryState,
) {
    let mut configuration: VegetationConfiguration = serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v5_diagnostic_configuration.json")
            .expect("committed V5 vegetation configuration fixture"),
    )
    .expect("V5 configuration DTO");
    let source_state: CoupledOwnedState = serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v5_diagnostic_state.json")
            .expect("committed V5 vegetation state fixture"),
    )
    .expect("V5 state DTO");

    let forest = TileId::try_new("forest").expect("forest tile");
    let open = TileId::try_new("open").expect("open tile");
    // Configuration order is physical rank order and deliberately differs
    // from lexical map order, so the owner join cannot rely on incidental
    // request/finalized-use vector ordering.
    let upper_id = StratumId::try_new("stratum-z-upper").expect("upper stratum");
    let lower_id = StratumId::try_new("stratum-a-lower").expect("lower stratum");
    let root_layers = [
        ("soil-1", 0.62),
        ("soil-2", 0.38),
        ("soil-dry", 0.0),
        ("soil-frozen", 0.0),
    ]
    .map(|(layer, fraction)| RootLayer {
        layer_id: SoilLayerId::try_new(layer).expect("root layer"),
        root_fraction: fraction,
        mineral_n_root_fraction: fraction,
        lateral_root_length_m: 0.20,
    })
    .to_vec();
    let mut upper_configuration = configuration.strata[0].clone();
    upper_configuration.stratum_id = upper_id.clone();
    upper_configuration.vertical_rank = 0;
    upper_configuration.tile_ids = vec![forest.clone()];
    upper_configuration.height_m = 12.5;
    upper_configuration.current_growth_fraction = 1.0;
    upper_configuration.root_layers.clone_from(&root_layers);
    let mut lower_configuration = upper_configuration.clone();
    lower_configuration.stratum_id = lower_id.clone();
    lower_configuration.vertical_rank = 1;
    lower_configuration.height_m = 8.0;
    lower_configuration.crown_base_m = 1.5;
    lower_configuration.displacement_m = 4.8;
    configuration.model_definition_sha256 = V8_MODEL_SHA256.into();
    configuration.dt_s = INTERVAL_S;
    configuration.topology_tiles = vec![
        TopologyTile {
            tile_id: forest.clone(),
            fraction: TILE_FRACTION,
        },
        TopologyTile {
            tile_id: open,
            fraction: 1.0 - TILE_FRACTION,
        },
    ];
    configuration.strata = vec![upper_configuration, lower_configuration];
    configuration.configuration_sha256.clear();
    configuration.initial_state_sha256 = "0".repeat(64);
    configuration.configuration_sha256 = configuration
        .canonical_sha256()
        .expect("aligned V8 configuration digest");

    let source_shared = source_state
        .strata
        .values()
        .next()
        .expect("V5 shared stratum")
        .clone();
    let mut upper_shared = source_shared.clone();
    let mut lower_shared = source_shared;
    for shared in [&mut upper_shared, &mut lower_shared] {
        shared.last_transaction_id = 40;
        shared.pending_transfers.clear();
        for pool in shared.tissues.values_mut() {
            pool.storage = ElementPool::default();
            pool.transfer = ElementPool::default();
        }
    }
    let source_lane = source_state
        .occupancies
        .values()
        .next()
        .expect("V5 occupancy lane");
    let occupancy = |canopy_liquid: f64| V8OccupancyState {
        beta_hyd: 0.67,
        canopy_liquid_kg_h2o_m2_tile_ground: canopy_liquid,
        dry_stem_temperature_k: 295.2,
        last_accepted_transaction_id: Some(40),
        root_node_potential_mm: -2_850.0,
        shade_ci_pa: source_lane.shade_ci_pa,
        shade_leaf_potential_mm: -5_450.0,
        shade_leaf_temperature_k: 295.4,
        stem_potential_mm: -4_300.0,
        sun_ci_pa: source_lane.sun_ci_pa,
        sun_leaf_potential_mm: -5_900.0,
        sun_leaf_temperature_k: 296.2,
        wet_surface_temperature_k: 295.6,
    };
    let upper_occupancy = OccupancyId {
        stratum_id: upper_id.clone(),
        tile_id: forest.clone(),
    };
    let lower_occupancy = OccupancyId {
        stratum_id: lower_id.clone(),
        tile_id: forest.clone(),
    };
    let mut state = V8CoupledOwnedState {
        configuration_sha256: configuration.configuration_sha256.clone(),
        last_transaction_id: 40,
        model_definition_sha256: V8_MODEL_SHA256.into(),
        occupancies: BTreeMap::from([
            (upper_occupancy.clone(), occupancy(0.018)),
            (lower_occupancy.clone(), occupancy(0.018)),
        ]),
        state_sha256: String::new(),
        strata: BTreeMap::from([(upper_id, upper_shared), (lower_id, lower_shared)]),
        tile_canopy_air: BTreeMap::from([(
            forest,
            V8TileCanopyAirState {
                canopy_air_specific_humidity_kg_kg: 0.011,
                canopy_air_temperature_k: 295.8,
            },
        )]),
    };
    state.state_sha256 = state.canonical_sha256();
    configuration
        .initial_state_sha256
        .clone_from(&state.state_sha256);
    configuration
        .validate_v8()
        .expect("aligned V8 configuration");
    state.validate(&configuration).expect("aligned V8 state");

    let bindings = vec![
        V8ComponentOccupancyBinding {
            component_id: V8LseComponentId::try_new("canopy-rank-0").expect("upper component"),
            occupancy_id: upper_occupancy,
            vertical_rank: 0,
        },
        V8ComponentOccupancyBinding {
            component_id: V8LseComponentId::try_new("canopy-rank-1").expect("lower component"),
            occupancy_id: lower_occupancy,
            vertical_rank: 1,
        },
    ];
    let soil_temperature_k_by_layer = root_layers
        .iter()
        .map(|root| {
            let temperature = if root.layer_id.as_str() == "soil-frozen" {
                270.0
            } else {
                293.0
            };
            (root.layer_id.clone(), temperature)
        })
        .collect();
    let forcing = V8PersistentForcingReceipt {
        model_definition_sha256: V8_MODEL_SHA256.into(),
        configuration_sha256: configuration.configuration_sha256.clone(),
        transaction_id: TransactionId(41),
        vegetation_beginning_state_sha256: state.state_sha256.clone(),
        air_temperature_k: 296.0,
        gsi: 1.0,
        soil_temperature_k_by_layer,
    };
    let receivers = [
        MaterialReceiverClass::Metabolic,
        MaterialReceiverClass::Cellulose,
        MaterialReceiverClass::Lignin,
        MaterialReceiverClass::CoarseWoodyDebris,
    ]
    .into_iter()
    .map(|receiver| (receiver, MaterialPool::default()))
    .collect();
    let biogeochemistry = BiogeochemistryState {
        layers: root_layers
            .iter()
            .map(|root| {
                (
                    root.layer_id.as_str().to_owned(),
                    MineralLayer {
                        ammonium_n: 1.0,
                        nitrate_n: 1.0,
                    },
                )
            })
            .collect(),
        receivers,
        last_transaction_id: 40,
    };
    (configuration, state, bindings, forcing, biogeochemistry)
}

fn covered_configuration() -> DirectSurfaceLiquidConfiguration {
    let forest = DirectSurfaceLiquidConfigurationRecord {
        key: DirectSurfaceLiquidStoreKey {
            run_id: 83,
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            tile_id: TileId::try_new("forest").expect("tile"),
            surface_id: SurfaceId::try_new("surface:ofe-1:forest").expect("surface"),
            surface_class: SurfaceClass::ForestLitter,
            source_type: WaterSourceType::LitterLiquid,
            source_id: SourceId::try_new("litter-liquid:ofe-1:forest").expect("source"),
        },
        tile_fraction: TILE_FRACTION,
        capacity_kg_m2_tile: 6.0,
        ofe_area_m2: 100.0,
        ground_ingress_mode: DirectGroundIngressMode::CoveredCanopyRelease,
        runon_destination_ofe_id: None,
        runon_destination_tile_id: None,
    };
    let open = DirectSurfaceLiquidConfigurationRecord {
        key: DirectSurfaceLiquidStoreKey {
            run_id: 83,
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            tile_id: TileId::try_new("open").expect("tile"),
            surface_id: SurfaceId::try_new("surface:ofe-1:open").expect("surface"),
            surface_class: SurfaceClass::BareMineralSoil,
            source_type: WaterSourceType::SurfaceLiquid,
            source_id: SourceId::try_new("surface-liquid:ofe-1:open").expect("source"),
        },
        tile_fraction: 1.0 - TILE_FRACTION,
        capacity_kg_m2_tile: 3.0,
        ofe_area_m2: 100.0,
        ground_ingress_mode: DirectGroundIngressMode::OpenRawPrecipitation,
        runon_destination_ofe_id: None,
        runon_destination_tile_id: None,
    };
    DirectSurfaceLiquidConfiguration::new(
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        83,
        vec![OfeId::try_new("ofe-1").expect("OFE")],
        vec![DirectSurfaceLiquidOfeBinding {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            production_lane_index: 0,
            production_lane_id: 1,
            ordered_soil_layer_ids: [
                "thermal-1",
                "thermal-2",
                "soil-1",
                "soil-2",
                "soil-dry",
                "soil-frozen",
            ]
            .map(|layer| SoilLayerId::try_new(layer).expect("layer"))
            .to_vec(),
            infiltration_soil_thermal_layer_id: SoilLayerId::try_new("thermal-1")
                .expect("thermal layer"),
        }],
        vec![forest, open],
    )
    .expect("covered configuration")
}

fn covered_frame_with_forest_liquid(
    configuration: &DirectSurfaceLiquidConfiguration,
    forest_liquid_kg_m2_tile: f64,
) -> DirectRunFrame {
    let identity = DirectRunIdentity::new(83, 11, 1, 1).expect("identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("frame");
    frame.lanes[0].area_m2 = 100.0;
    frame.lanes[0].subsurface_layers = (0..6)
        .map(|_| DirectSubsurfaceLayerState {
            theta_m: 0.02,
            field_capacity_m: 0.02,
            upper_limit_m: 0.2,
            conductivity_m_s: 1.0e-6,
            depth_m: 0.3,
            residual_theta: 0.0,
            frozen_depth_m: 0.0,
            frozen_water_m: 0.0,
            porosity: 0.45,
            field_capacity_theta: 0.25,
            coca: 0.1,
            lateral_conductivity_m_s: 1.0e-7,
        })
        .collect();
    frame.lanes[0].water.soil_water_m = frame.lanes[0]
        .subsurface_layers
        .iter()
        .map(|layer| layer.theta_m)
        .sum();
    let initial = configuration
        .records
        .iter()
        .map(|record| {
            (
                record.key.clone(),
                if record.key.tile_id.as_str() == "forest" {
                    forest_liquid_kg_m2_tile
                } else {
                    0.0
                },
            )
        })
        .collect();
    let state = DirectSurfaceLiquidOwnedState::new_initial(configuration, &initial, 0)
        .expect("surface state");
    frame
        .configure_surface_liquid_shadow(configuration, state)
        .expect("attach surface owner");
    frame
}

fn covered_frame(configuration: &DirectSurfaceLiquidConfiguration) -> DirectRunFrame {
    covered_frame_with_forest_liquid(configuration, 4.0)
}

fn covered_ingress(mass: f64) -> DirectSurfaceLiquidIngressInput {
    let amount = DirectIngressAmount {
        mass_kg_m2_tile_ground: mass,
        temperature_k: 294.0,
        specific_liquid_enthalpy_j_kg: 4_218.0 * (294.0 - 273.15),
        start_s: 0.0,
        end_s: INTERVAL_S,
    };
    DirectSurfaceLiquidIngressInput {
        transaction_id: TransactionId(41),
        day_index: 0,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![
            DirectTileGroundIngress::CoveredCanopyRelease {
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                tile_id: TileId::try_new("forest").expect("tile"),
                surface_id: SurfaceId::try_new("surface:ofe-1:forest").expect("surface"),
                release: DirectCanopyLiquidRelease {
                    throughfall: amount.clone(),
                    initial_drainage: DirectIngressAmount {
                        mass_kg_m2_tile_ground: 0.0,
                        ..amount.clone()
                    },
                    second_drainage: DirectIngressAmount {
                        mass_kg_m2_tile_ground: 0.0,
                        ..amount.clone()
                    },
                    stemflow: DirectIngressAmount {
                        mass_kg_m2_tile_ground: 0.0,
                        ..amount.clone()
                    },
                },
            },
            DirectTileGroundIngress::OpenRawPrecipitation {
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                tile_id: TileId::try_new("open").expect("tile"),
                surface_id: SurfaceId::try_new("surface:ofe-1:open").expect("surface"),
                raw_precipitation: DirectIngressAmount {
                    mass_kg_m2_tile_ground: 0.0,
                    ..amount
                },
            },
        ],
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.04,
        }],
    }
}

fn covered_soil_thermal_with_temperatures(temperatures_k: [f64; 2]) -> SoilThermalSnapshot {
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
                    temperature_k: temperatures_k[0],
                    enthalpy_j_m2_ofe_ground: 1.0e6,
                },
                SoilThermalLayerSnapshot {
                    layer_id: SoilLayerId::try_new("thermal-2").expect("layer"),
                    temperature_k: temperatures_k[1],
                    enthalpy_j_m2_ofe_ground: 2.0e6,
                },
            ],
        }],
    }
}

fn covered_soil_thermal() -> SoilThermalSnapshot {
    covered_soil_thermal_with_temperatures([291.5, 289.8])
}

#[allow(clippy::too_many_lines)]
fn execute_covered_fixture(
    configuration: &DirectSurfaceLiquidConfiguration,
    frame: &DirectRunFrame,
    column: &CoveredColumnInputs,
    solve_trial: Vec<f64>,
    soil_thermal: &SoilThermalSnapshot,
    ingress_mass_kg_m2_tile: f64,
) -> CoveredForestShadowResult {
    let lane = RealHydrologyOfeLaneId {
        lane_index: 0,
        lane_id: frame.lanes[0].lane_id,
    };
    let layer_ids = [
        "thermal-1",
        "thermal-2",
        "soil-1",
        "soil-2",
        "soil-dry",
        "soil-frozen",
    ]
    .map(|layer| SoilLayerId::try_new(layer).expect("layer"))
    .to_vec();
    let owner = RealHydrologyShadowAdapter::try_from_day_start(
        frame,
        0,
        TransactionId(41),
        INTERVAL_S,
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        &[RealHydrologyLaneLayerMap {
            ofe_lane: lane,
            layer_ids,
        }],
    )
    .expect("real owner");
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, configuration)
        .expect("unified snapshot");
    let runtime_identity = identity(
        snapshot.clone(),
        configuration.records[0].key.source_id.clone(),
    );
    let preview = solve_covered_potential_phase(
        runtime_identity.clone(),
        column,
        root_runtime_identities(),
        solve_trial.clone(),
    )
    .expect("preview potential");
    let soil_sources = preview
        .request_batch
        .requests
        .iter()
        .filter_map(|request| {
            request.key.soil_layer_id.as_ref().map(|layer_id| {
                (
                    request.key.clone(),
                    RealHydrologySourceKey {
                        ofe_lane: lane,
                        layer_id: layer_id.clone(),
                    },
                )
            })
        })
        .collect();
    let expectations = UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        digest('2'),
        ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
        snapshot,
        ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        digest('4'),
        ["forest", "open"]
            .map(|tile| {
                (
                    OfeId::try_new("ofe-1").expect("OFE"),
                    TileId::try_new(tile).expect("tile"),
                    vec![
                        SoilLayerId::try_new("thermal-1").expect("layer"),
                        SoilLayerId::try_new("thermal-2").expect("layer"),
                    ],
                )
            })
            .to_vec(),
    )
    .expect("receiver expectations");
    let companion_lse = TileState {
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        tile_id: TileId::try_new("open").expect("tile"),
        surface_enthalpy_j_m2_tile_ground: 0.0,
        surface_temperature_warm_start_k: 291.0,
    };
    let companion_thermal = SoilThermalTileCandidate {
        owner_id: ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        beginning_state_sha256: digest('4'),
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        tile_id: TileId::try_new("open").expect("tile"),
        layers: vec![
            SoilThermalLayerCandidate {
                layer_id: SoilLayerId::try_new("thermal-1").expect("layer"),
                beginning_enthalpy_j_m2_ofe_ground: 1.0e6,
                ground_heat_credit_j_m2_ofe_ground: 0.0,
                infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 1.0e6,
                ending_temperature_k: 291.5,
            },
            SoilThermalLayerCandidate {
                layer_id: SoilLayerId::try_new("thermal-2").expect("layer"),
                beginning_enthalpy_j_m2_ofe_ground: 2.0e6,
                ground_heat_credit_j_m2_ofe_ground: 0.0,
                infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 2.0e6,
                ending_temperature_k: 289.8,
            },
        ],
    };
    let open_key = GroundWaterKey {
        transaction_id: TransactionId(41),
        requesting_owner_id: ResourceOwnerId::try_new("land-surface-energy-v1")
            .expect("LSE owner"),
        requesting_component: openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::GroundSurface,
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        requesting_tile_id: TileId::try_new("open").expect("tile"),
        occupancy_id: None,
        surface_id: Some(SurfaceId::try_new("surface:ofe-1:open").expect("surface")),
        surface_class: Some(SurfaceClass::BareMineralSoil),
        source_type: WaterSourceType::SurfaceLiquid,
        source_id: SourceId::try_new("surface-liquid:ofe-1:open").expect("source"),
        source_tile_id: Some(TileId::try_new("open").expect("source tile")),
        soil_layer_id: None,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
    };
    let companion_request = WaterAmount {
        key: open_key.clone(),
        amount_kg_m2_stand_ground: 0.0,
    };
    let companion_finalized = WaterAmount {
        key: open_key,
        amount_kg_m2_stand_ground: 0.0,
    };
    execute_covered_forest_shadow(
        &adapter,
        configuration,
        &expectations,
        runtime_identity,
        column,
        root_runtime_identities(),
        &soil_sources,
        &covered_ingress(ingress_mass_kg_m2_tile),
        solve_trial.clone(),
        solve_trial,
        soil_thermal,
        &[companion_request],
        &[companion_finalized],
        &[companion_lse],
        &[companion_thermal],
    )
    .expect("covered forest real-owner transaction")
}

#[allow(clippy::too_many_lines)]
fn execute_aligned_v8_fixture(
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    frame: &DirectRunFrame,
) -> (
    openwepp_hillslope_orchestrator::land_surface_energy_shadow::UncommittedCoveredV8OwnerEnvelope,
    u32,
    VegetationConfiguration,
    V8CoupledOwnedState,
) {
    let column = column();
    let lane = RealHydrologyOfeLaneId {
        lane_index: 0,
        lane_id: frame.lanes[0].lane_id,
    };
    let layer_ids = [
        "thermal-1",
        "thermal-2",
        "soil-1",
        "soil-2",
        "soil-dry",
        "soil-frozen",
    ]
    .map(|layer| SoilLayerId::try_new(layer).expect("layer"))
    .to_vec();
    let owner = RealHydrologyShadowAdapter::try_from_day_start(
        frame,
        0,
        TransactionId(41),
        INTERVAL_S,
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        &[RealHydrologyLaneLayerMap {
            ofe_lane: lane,
            layer_ids,
        }],
    )
    .expect("real owner");
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, surface_configuration)
        .expect("unified snapshot");
    let runtime_identity = identity(
        snapshot.clone(),
        surface_configuration.records[0].key.source_id.clone(),
    );
    let roots = root_runtime_identities();
    let soil_sources = roots
        .iter()
        .map(|root| {
            let key = GroundWaterKey {
                transaction_id: TransactionId(41),
                requesting_owner_id: root.requesting_owner_id.clone(),
                requesting_component: openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::VegetationRoot,
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                requesting_tile_id: TileId::try_new("forest").expect("tile"),
                occupancy_id: Some(root.occupancy_id.clone()),
                surface_id: None,
                surface_class: None,
                source_type: WaterSourceType::SoilLayerLiquid,
                source_id: root.source_id.clone(),
                source_tile_id: None,
                soil_layer_id: Some(root.layer_id.clone()),
                amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
            };
            (
                key,
                RealHydrologySourceKey {
                    ofe_lane: lane,
                    layer_id: root.layer_id.clone(),
                },
            )
        })
        .collect();
    let expectations = UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        digest('2'),
        ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
        snapshot,
        ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        digest('4'),
        ["forest", "open"]
            .map(|tile| {
                (
                    OfeId::try_new("ofe-1").expect("OFE"),
                    TileId::try_new(tile).expect("tile"),
                    vec![
                        SoilLayerId::try_new("thermal-1").expect("layer"),
                        SoilLayerId::try_new("thermal-2").expect("layer"),
                    ],
                )
            })
            .to_vec(),
    )
    .expect("receiver expectations");
    let companion_lse = TileState {
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        tile_id: TileId::try_new("open").expect("tile"),
        surface_enthalpy_j_m2_tile_ground: 0.0,
        surface_temperature_warm_start_k: 291.0,
    };
    let companion_thermal = SoilThermalTileCandidate {
        owner_id: ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        beginning_state_sha256: digest('4'),
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        tile_id: TileId::try_new("open").expect("tile"),
        layers: vec![
            SoilThermalLayerCandidate {
                layer_id: SoilLayerId::try_new("thermal-1").expect("layer"),
                beginning_enthalpy_j_m2_ofe_ground: 1.0e6,
                ground_heat_credit_j_m2_ofe_ground: 0.0,
                infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 1.0e6,
                ending_temperature_k: 291.5,
            },
            SoilThermalLayerCandidate {
                layer_id: SoilLayerId::try_new("thermal-2").expect("layer"),
                beginning_enthalpy_j_m2_ofe_ground: 2.0e6,
                ground_heat_credit_j_m2_ofe_ground: 0.0,
                infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 2.0e6,
                ending_temperature_k: 289.8,
            },
        ],
    };
    let open_key = GroundWaterKey {
        transaction_id: TransactionId(41),
        requesting_owner_id: ResourceOwnerId::try_new("land-surface-energy-v1")
            .expect("LSE owner"),
        requesting_component: openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::GroundSurface,
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        requesting_tile_id: TileId::try_new("open").expect("tile"),
        occupancy_id: None,
        surface_id: Some(SurfaceId::try_new("surface:ofe-1:open").expect("surface")),
        surface_class: Some(SurfaceClass::BareMineralSoil),
        source_type: WaterSourceType::SurfaceLiquid,
        source_id: SourceId::try_new("surface-liquid:ofe-1:open").expect("source"),
        source_tile_id: Some(TileId::try_new("open").expect("source tile")),
        soil_layer_id: None,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
    };
    let companion_request = WaterAmount {
        key: open_key.clone(),
        amount_kg_m2_stand_ground: 0.0,
    };
    let companion_finalized = WaterAmount {
        key: open_key,
        amount_kg_m2_stand_ground: 0.0,
    };
    let (vegetation_configuration, vegetation_beginning, bindings, forcing, bgc_beginning) =
        aligned_v8_vegetation_fixture();
    let nitrogen = CountingFullNitrogen {
        calls: Cell::new(0),
    };
    let result = execute_covered_v8_transaction(
        &adapter,
        surface_configuration,
        &expectations,
        runtime_identity,
        &column,
        roots,
        &soil_sources,
        &covered_ingress(0.05),
        &covered_soil_thermal(),
        &[companion_request],
        &[companion_finalized],
        &[companion_lse],
        &[companion_thermal],
        &vegetation_configuration,
        &vegetation_beginning,
        bindings,
        &forcing,
        &nitrogen,
        &bgc_beginning,
    )
    .expect("complete aligned V8 uncommitted transaction");
    (
        result,
        nitrogen.calls.get(),
        vegetation_configuration,
        vegetation_beginning,
    )
}

#[test]
fn covered_root_and_ground_requests_preserve_distinct_owner_identity() {
    let phase = solve_covered_potential_phase(
        identity(digest('3'), SourceId::try_new("litter").expect("source")),
        &column(),
        root_runtime_identities(),
        trial(),
    )
    .expect("covered potential");
    assert_eq!(phase.request_batch.requests.len(), 9);
    assert!(phase.request_batch.requests[..8].iter().all(|request| {
        request.key.requesting_owner_id.as_str() == "vegetation-v8"
            && request.key.requesting_component
                == openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::VegetationRoot
    }));
    assert_eq!(
        phase.request_batch.requests[8]
            .key
            .requesting_owner_id
            .as_str(),
        "land-surface-energy-v1"
    );

    let mut wrong = root_runtime_identities();
    wrong[0].requesting_owner_id =
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("wrong owner");
    let error = solve_covered_potential_phase(
        identity(digest('3'), SourceId::try_new("litter").expect("source")),
        &column(),
        wrong,
        trial(),
    )
    .expect_err("root/LSE owner alias");
    assert_eq!(error.class(), openwepp_hillslope_orchestrator::land_surface_energy_shadow::LandSurfaceEnergyErrorClass::Identity);
}

#[test]
#[allow(clippy::too_many_lines)]
fn covered_forest_wrapper_uses_one_real_authorization_and_post_solve_ingress() {
    let configuration = covered_configuration();
    let frame = covered_frame(&configuration);
    let production_before = frame.clone();
    let lane = RealHydrologyOfeLaneId {
        lane_index: 0,
        lane_id: frame.lanes[0].lane_id,
    };
    let layer_ids = [
        "thermal-1",
        "thermal-2",
        "soil-1",
        "soil-2",
        "soil-dry",
        "soil-frozen",
    ]
    .map(|layer| SoilLayerId::try_new(layer).expect("layer"))
    .to_vec();
    let owner = RealHydrologyShadowAdapter::try_from_day_start(
        &frame,
        0,
        TransactionId(41),
        INTERVAL_S,
        ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        &[RealHydrologyLaneLayerMap {
            ofe_lane: lane,
            layer_ids: layer_ids.clone(),
        }],
    )
    .expect("real owner");
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("unified snapshot");
    let runtime_identity = identity(
        snapshot.clone(),
        configuration.records[0].key.source_id.clone(),
    );
    let preview = solve_covered_potential_phase(
        runtime_identity.clone(),
        &column(),
        root_runtime_identities(),
        trial(),
    )
    .expect("preview potential");
    let soil_sources = preview
        .request_batch
        .requests
        .iter()
        .filter_map(|request| {
            request.key.soil_layer_id.as_ref().map(|layer_id| {
                (
                    request.key.clone(),
                    RealHydrologySourceKey {
                        ofe_lane: lane,
                        layer_id: layer_id.clone(),
                    },
                )
            })
        })
        .collect();
    let expectations = UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        digest('2'),
        ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
        snapshot,
        ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        digest('4'),
        ["forest", "open"]
            .map(|tile| {
                (
                    OfeId::try_new("ofe-1").expect("OFE"),
                    TileId::try_new(tile).expect("tile"),
                    vec![
                        SoilLayerId::try_new("thermal-1").expect("layer"),
                        SoilLayerId::try_new("thermal-2").expect("layer"),
                    ],
                )
            })
            .to_vec(),
    )
    .expect("receiver expectations");
    let companion_lse = TileState {
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        tile_id: TileId::try_new("open").expect("tile"),
        surface_enthalpy_j_m2_tile_ground: 0.0,
        surface_temperature_warm_start_k: 291.0,
    };
    let companion_thermal = SoilThermalTileCandidate {
        owner_id: ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
        beginning_state_sha256: digest('4'),
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        tile_id: TileId::try_new("open").expect("tile"),
        layers: vec![
            SoilThermalLayerCandidate {
                layer_id: SoilLayerId::try_new("thermal-1").expect("layer"),
                beginning_enthalpy_j_m2_ofe_ground: 1.0e6,
                ground_heat_credit_j_m2_ofe_ground: 0.0,
                infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 1.0e6,
                ending_temperature_k: 291.5,
            },
            SoilThermalLayerCandidate {
                layer_id: SoilLayerId::try_new("thermal-2").expect("layer"),
                beginning_enthalpy_j_m2_ofe_ground: 2.0e6,
                ground_heat_credit_j_m2_ofe_ground: 0.0,
                infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 2.0e6,
                ending_temperature_k: 289.8,
            },
        ],
    };
    let open_key = GroundWaterKey {
        transaction_id: TransactionId(41),
        requesting_owner_id: ResourceOwnerId::try_new("land-surface-energy-v1")
            .expect("LSE owner"),
        requesting_component: openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::GroundSurface,
        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
        requesting_tile_id: TileId::try_new("open").expect("tile"),
        occupancy_id: None,
        surface_id: Some(SurfaceId::try_new("surface:ofe-1:open").expect("surface")),
        surface_class: Some(SurfaceClass::BareMineralSoil),
        source_type: WaterSourceType::SurfaceLiquid,
        source_id: SourceId::try_new("surface-liquid:ofe-1:open").expect("source"),
        source_tile_id: Some(TileId::try_new("open").expect("source tile")),
        soil_layer_id: None,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
    };
    let companion_request = WaterAmount {
        key: open_key.clone(),
        amount_kg_m2_stand_ground: 0.0,
    };
    let companion_finalized = WaterAmount {
        key: open_key,
        amount_kg_m2_stand_ground: 0.0,
    };
    let result = execute_covered_forest_shadow(
        &adapter,
        &configuration,
        &expectations,
        runtime_identity,
        &column(),
        root_runtime_identities(),
        &soil_sources,
        &covered_ingress(0.05),
        trial(),
        trial(),
        &covered_soil_thermal(),
        &[companion_request],
        &[companion_finalized],
        &[companion_lse],
        &[companion_thermal],
    )
    .expect("covered forest real-owner transaction");

    assert_eq!(frame, production_before, "production frame mutated");
    assert_eq!(result.potential().request_batch.requests.len(), 9);
    assert_eq!(result.submitted_request_batch().requests.len(), 10);
    assert_eq!(
        result.hydrology_candidate().arbitration().requests,
        result.submitted_request_batch().requests
    );
    for ((request, authorization), finalized) in result
        .hydrology_candidate()
        .arbitration()
        .requests
        .iter()
        .zip(&result.hydrology_candidate().arbitration().authorizations)
        .zip(result.hydrology_candidate().finalized_uses())
    {
        assert_eq!(request.key, authorization.key);
        assert_eq!(request.key, finalized.key);
        assert!(authorization.amount_kg_m2_stand_ground <= request.amount_kg_m2_stand_ground);
        assert!(finalized.amount_kg_m2_stand_ground <= authorization.amount_kg_m2_stand_ground);
    }
    assert_eq!(
        result.final_tile().water_protocol.finalized_uses,
        result.hydrology_candidate().finalized_uses()
    );
    assert!(result.final_tile().energy_operands.validate().is_ok());
    assert!(result.final_tile().diagnostics.accepted);
    let receipts = result.hydrology_candidate().surface_ingress().receipts();
    assert_eq!(receipts.len(), 1);
    assert!(receipts[0].mass_kg_m2_basis_ofe_ground > 0.0);
    assert_eq!(receipts[0].origin_store_key.tile_id.as_str(), "forest");
    assert_eq!(result.hydrology_candidate().rollback_hashes().len(), 3);
}

#[test]
#[allow(clippy::too_many_lines)]
fn covered_forest_condensation_credits_full_litter_store_and_routes_overflow() {
    let configuration = covered_configuration();
    let frame = covered_frame_with_forest_liquid(&configuration, 6.0);
    let production_before = frame.clone();
    let mut condensation_column = column();
    condensation_column.air_specific_humidity_kg_kg = 0.021;
    condensation_column.ground.air_specific_humidity_kg_kg = 0.021;
    condensation_column.ground.surface_liquid_kg_m2_tile = 6.0;
    condensation_column.ground.surface_enthalpy_j_m2_tile = 195_524.208_000_000_65;
    condensation_column.ground.surface_temperature_warm_start_k = 280.0;
    condensation_column.ground.soil_nodes[0].beginning_temperature_k = 278.0;
    condensation_column.ground.soil_nodes[1].beginning_temperature_k = 276.0;
    let mut condensation_trial = trial();
    let common = condensation_trial.len() - 5;
    condensation_trial[common + 2] = 280.0;
    condensation_trial[common + 3] = 278.0;
    condensation_trial[common + 4] = 276.0;
    let soil_thermal = covered_soil_thermal_with_temperatures([278.0, 276.0]);

    let result = execute_covered_fixture(
        &configuration,
        &frame,
        &condensation_column,
        condensation_trial,
        &soil_thermal,
        0.0,
    );

    assert_eq!(frame, production_before, "production frame mutated");
    let ground = &result
        .final_tile()
        .final_solver_candidate
        .evaluation
        .ground_water;
    assert_eq!(
        serde_json::to_value(ground.branch).expect("serialize typed water branch"),
        serde_json::json!("Condensation")
    );
    assert!(ground.law_kg_m2_tile_s < 0.0);
    assert!(ground.final_kg_m2_tile_s < 0.0);
    assert_eq!(
        ground.request_kg_m2_stand_ground.to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        ground
            .authorization_kg_m2_stand_ground
            .expect("fixed zero authorization")
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        ground.finalized_use_kg_m2_stand_ground.to_bits(),
        0.0_f64.to_bits()
    );

    let credits = &result.final_tile().water_protocol.condensation_credits;
    assert_eq!(credits.len(), 1);
    let credit = &credits[0];
    let expected_credit = -ground.final_kg_m2_tile_s * TILE_FRACTION * INTERVAL_S;
    assert_eq!(
        credit.amount_kg_m2_stand_ground.to_bits(),
        expected_credit.to_bits()
    );
    assert!(credit.amount_kg_m2_stand_ground > 0.0);
    assert_eq!(credit.transaction_id, TransactionId(41));
    assert_eq!(credit.hydrology_owner_id.as_str(), "production-hydrology");
    assert_eq!(credit.ofe_id.as_str(), "ofe-1");
    assert_eq!(credit.tile_id.as_str(), "forest");
    assert_eq!(credit.surface_id.as_str(), "surface:ofe-1:forest");
    assert_eq!(
        credit.amount_basis,
        StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval
    );
    assert_eq!(
        credit.specific_liquid_enthalpy_j_kg.to_bits(),
        (4_218.0 * (credit.temperature_k - 273.15)).to_bits()
    );
    assert_eq!(result.hydrology_candidate().condensation_credits(), credits);

    let resource = result.hydrology_candidate().surface_resource();
    assert_eq!(resource.condensation_credits(), credits);
    assert_eq!(resource.condensation_overflow().len(), 1);
    let overflow = &resource.condensation_overflow()[0];
    assert_eq!(overflow.store_key.tile_id.as_str(), "forest");
    let overflow_mass_tolerance = 1.0e-14
        + 64.0
            * f64::EPSILON
            * (overflow.amount_kg_m2_ofe_ground.abs() + credit.amount_kg_m2_stand_ground.abs());
    assert!(
        (overflow.amount_kg_m2_ofe_ground - credit.amount_kg_m2_stand_ground).abs()
            <= overflow_mass_tolerance
    );
    assert_eq!(
        overflow.temperature_k.to_bits(),
        credit.temperature_k.to_bits()
    );
    assert_eq!(
        overflow.specific_liquid_enthalpy_j_kg.to_bits(),
        credit.specific_liquid_enthalpy_j_kg.to_bits()
    );
    let working_forest = resource
        .working_state()
        .records
        .iter()
        .find(|row| row.key.tile_id.as_str() == "forest")
        .expect("working forest store");
    assert_eq!(
        working_forest.liquid_kg_m2_tile.to_bits(),
        6.0_f64.to_bits()
    );

    let ingress = result.hydrology_candidate().surface_ingress();
    assert_eq!(ingress.beginning_state(), resource.working_state());
    let condensation_receipts: Vec<_> = ingress
        .receipts()
        .iter()
        .filter(|row| row.kind == DirectSurfaceLiquidParcelKind::CondensationOverflow)
        .collect();
    assert!(!condensation_receipts.is_empty());
    let receipt_mass: f64 = condensation_receipts
        .iter()
        .map(|row| row.mass_kg_m2_basis_ofe_ground)
        .sum();
    let receipt_enthalpy: f64 = condensation_receipts
        .iter()
        .map(|row| row.enthalpy_j_m2_basis_ofe_ground)
        .sum();
    let receipt_mass_tolerance = 1.0e-14
        + 64.0 * f64::EPSILON * (receipt_mass.abs() + overflow.amount_kg_m2_ofe_ground.abs());
    assert!((receipt_mass - overflow.amount_kg_m2_ofe_ground).abs() <= receipt_mass_tolerance);
    let expected_enthalpy = overflow.amount_kg_m2_ofe_ground * credit.specific_liquid_enthalpy_j_kg;
    let receipt_enthalpy_tolerance =
        1.0e-9 + 64.0 * f64::EPSILON * (receipt_enthalpy.abs() + expected_enthalpy.abs());
    assert!((receipt_enthalpy - expected_enthalpy).abs() <= receipt_enthalpy_tolerance);
    result
        .hydrology_candidate()
        .validate(&configuration)
        .expect("unified condensation candidate");
    assert!(
        result
            .hydrology_candidate()
            .rollback_hashes()
            .iter()
            .all(|row| row.before_sha256 == row.after_sha256)
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn covered_v8_public_wrapper_joins_all_uncommitted_owners_from_one_real_snapshot() {
    let surface_configuration = covered_configuration();
    let frame = covered_frame(&surface_configuration);
    let production_before = frame.clone();

    let (envelope, nitrogen_calls, vegetation_configuration, vegetation_beginning) =
        execute_aligned_v8_fixture(&surface_configuration, &frame);

    assert_eq!(frame, production_before, "production frame mutated");
    assert_eq!(nitrogen_calls, 1, "nitrogen owner called more than once");
    assert_eq!(envelope.transaction_id(), TransactionId(41));
    envelope.validate().expect("complete uncommitted owner set");

    let ending = envelope.vegetation().ending_state();
    assert_eq!(ending.last_transaction_id, 41);
    assert_eq!(ending.model_definition_sha256, V8_MODEL_SHA256);
    assert_eq!(
        ending.configuration_sha256,
        vegetation_configuration.configuration_sha256
    );
    assert_eq!(ending.state_sha256, ending.canonical_sha256());
    assert_ne!(ending.state_sha256, vegetation_beginning.state_sha256);
    ending
        .validate(&vegetation_configuration)
        .expect("strict ending V8 state");
    assert!(
        ending
            .occupancies
            .values()
            .all(|lane| lane.last_accepted_transaction_id == Some(41))
    );

    let arbitration = envelope.hydrology().arbitration();
    let root_requests = arbitration
        .requests
        .iter()
        .filter(|row| {
            row.key.requesting_component
                == openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::VegetationRoot
        })
        .collect::<Vec<_>>();
    let root_authorizations = arbitration
        .authorizations
        .iter()
        .filter(|row| {
            row.key.requesting_component
                == openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::VegetationRoot
        })
        .collect::<Vec<_>>();
    let root_uses = envelope
        .hydrology()
        .finalized_uses()
        .iter()
        .filter(|row| {
            row.key.requesting_component
                == openwepp_hillslope_orchestrator::land_surface_energy_shadow::RequestingComponent::VegetationRoot
        })
        .collect::<Vec<_>>();
    assert_eq!(root_requests.len(), 8);
    assert_eq!(root_authorizations.len(), root_requests.len());
    assert_eq!(root_uses.len(), root_requests.len());
    for ((request, authorization), finalized) in root_requests
        .iter()
        .zip(&root_authorizations)
        .zip(&root_uses)
    {
        assert_eq!(request.key, authorization.key);
        assert_eq!(request.key, finalized.key);
        assert!(authorization.amount_kg_m2_stand_ground <= request.amount_kg_m2_stand_ground);
        assert!(finalized.amount_kg_m2_stand_ground <= authorization.amount_kg_m2_stand_ground);
    }
    let final_root_count: usize = envelope
        .physical()
        .final_tile()
        .vegetation_operands
        .occupancies
        .iter()
        .map(|occupancy| occupancy.root_water.len())
        .sum();
    assert_eq!(final_root_count, root_uses.len());

    let vegetation_n = envelope.vegetation().nitrogen_protocol();
    let bgc_n = envelope.biogeochemistry().protocol();
    assert_eq!(vegetation_n.0, bgc_n.0);
    assert_eq!(vegetation_n.1, bgc_n.1);
    assert_eq!(vegetation_n.2, bgc_n.2);
    assert_eq!(
        envelope.biogeochemistry().receipts().len(),
        envelope.vegetation().material_proposals().len()
    );
    assert_eq!(envelope.biogeochemistry().ending().last_transaction_id, 41);

    assert!(
        envelope
            .physical()
            .final_tile()
            .energy_operands
            .validate()
            .is_ok()
    );
    assert_eq!(
        envelope
            .physical()
            .final_tile()
            .soil_thermal
            .owner_id
            .as_str(),
        "soil-thermal"
    );
    assert!(
        envelope
            .hydrology()
            .rollback_hashes()
            .iter()
            .all(|row| row.before_sha256 == row.after_sha256)
    );

    let owner_source = fs::read_to_string(
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_owner.rs",
    )
    .expect("covered V8 owner source");
    assert!(!owner_source.contains("fn commit"));
}
