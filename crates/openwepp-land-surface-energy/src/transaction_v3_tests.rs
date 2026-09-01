use super::*;
use crate::{
    BandDirectionalFluxes, BiochemicalConstants, ComponentId, CoveredColumnAuthority,
    CoveredColumnShortwaveInputs, CoveredOccupancyInputs, CoveredOccupancyShortwaveInputs,
    LeafBiochemicalInputs, OpenNeutralGeometry, OpenSurfaceProblem, RootHydraulicLayer,
    SoilThermalLayerSnapshot, SoilThermalNodeOperands, SoilThermalOfeSnapshot, SoilThermalSnapshot,
    SurfaceId, SurfaceStorageBranch, UnderCanopyGeometry, WaterAuthorizationReason,
    evaluate_v3_phase_free_covered_column,
};
use openwepp_kernel_contract::SoilLayerId;

fn digest(byte: char) -> Sha256Digest {
    Sha256Digest::try_new(byte.to_string().repeat(64)).expect("test digest")
}

fn owner(value: &str) -> ResourceOwnerId {
    ResourceOwnerId::try_new(value).expect("test owner")
}

fn configuration() -> LitterPhaseConfiguration {
    LitterPhaseConfiguration {
        litter_depth_m: 0.04,
        dry_heat_capacity_j_m2_k: 3_235.68,
        liquid_capacity_kg_m2_tile: 6.0,
        ice_capacity_kg_m2_tile: 34.0,
    }
}

fn litter_beginning() -> BeginningLitterPhaseState {
    let liquid = 4.0;
    let ice = 0.5;
    let temperature = 295.0;
    let capacity = configuration().dry_heat_capacity_j_m2_k
        + liquid * crate::WATER_HEAT_CAPACITY_J_KG_K
        + ice * crate::LITTER_ICE_HEAT_CAPACITY_J_KG_K;
    BeginningLitterPhaseState {
        liquid_kg_m2_tile: liquid,
        ice_kg_m2_tile: ice,
        sensible_energy_j_m2_tile: capacity * (temperature - crate::REFERENCE_TEMPERATURE_K),
        temperature_k: temperature,
    }
}

fn column() -> CoveredColumnInputs {
    let litter = litter_beginning();
    let ground = OpenSurfaceProblem {
        interval_s: 1_800.0,
        tile_fraction: 0.38,
        class: SurfaceClassKind::ForestLitter,
        storage_branch: SurfaceStorageBranch::FiniteCapacity,
        terminal_shortwave_w_m2_tile: BandDirectionalFluxes {
            direct_vis: 47.4,
            diffuse_vis: 8.7,
            direct_nir: 41.1,
            diffuse_nir: 52.1,
        },
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
        surface_liquid_kg_m2_tile: litter.liquid_kg_m2_tile,
        surface_enthalpy_j_m2_tile: litter.sensible_energy_j_m2_tile,
        surface_temperature_warm_start_k: litter.temperature_k,
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
    };
    let occupancy = CoveredOccupancyInputs {
        occupancy_id: "canopy-rank-0".into(),
        medlyn_g1_kpa_sqrt: 3.5,
        g0_umol_m2_s: 25.0,
        sun: LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: 1.11,
            absorbed_shortwave_w_m2_tile: 220.67,
            absorbed_par_w_m2_leaf: 136.73,
            vcmax25: 62.0,
            jmax25: 108.0,
            rd25: 1.15,
        },
        shade: LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: 1.60,
            absorbed_shortwave_w_m2_tile: 300.71,
            absorbed_par_w_m2_leaf: 118.23,
            vcmax25: 41.0,
            jmax25: 74.0,
            rd25: 0.81,
        },
        biochemical: BiochemicalConstants {
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
        },
        stem_area_m2_m2_tile: 0.72,
        stem_absorbed_shortwave_w_m2_tile: 185.38,
        beginning_canopy_liquid_kg_m2_tile: 0.018,
        liquid_interception_fraction: 0.35,
        liquid_capacity_kg_m2_plant: 0.023_328_503_368_824_437,
        stemflow_fraction: 0.08,
        gb_leaf_m_s: 0.035_961_386_715_575_215,
        gb_wet_m_s: 0.019_071_405_305_591_295,
        gb_stem_m_s: 0.013_082_876_106_352_972,
        lai: 2.71,
        sai: 0.72,
        clumping_index: 0.82,
        k1_sun_max_s1: 1.2e-6,
        k1_shade_max_s1: 1.2e-6,
        k2_max: 4.2e-6,
        k3_max_m_s: 5.0e-5,
        height_m: 12.5,
        root_to_leaf_area: 1.8,
        p50_leaf_mm: -9_800.0,
        p50_xylem_mm: -7_200.0,
        p50_root_mm: -14_000.0,
        vulnerability_exponent: 2.0,
        root_layers: vec![RootHydraulicLayer {
            layer_id: "soil-1".into(),
            accessible: true,
            frozen: false,
            root_fraction: 1.0,
            soil_potential_mm: 100.0,
            gravity_head_mm: 120.0,
            z3_m: 0.32,
            dxroot_m: 0.18,
            ksoil_m2_s: 6.0e-11,
        }],
    };
    let ground_absorbed = crate::partition_ground_shortwave(
        ground.terminal_shortwave_w_m2_tile,
        ground.surface_vis_albedo,
        ground.surface_nir_albedo,
    )
    .expect("ground shortwave")
    .absorbed;
    let sun = BandDirectionalFluxes {
        direct_vis: occupancy.sun.absorbed_shortwave_w_m2_tile,
        ..BandDirectionalFluxes::default()
    };
    let shade = BandDirectionalFluxes {
        direct_vis: occupancy.shade.absorbed_shortwave_w_m2_tile,
        ..BandDirectionalFluxes::default()
    };
    let stem = BandDirectionalFluxes {
        direct_vis: occupancy.stem_absorbed_shortwave_w_m2_tile,
        ..BandDirectionalFluxes::default()
    };
    let incident = BandDirectionalFluxes {
        direct_vis: ground_absorbed.direct_vis
            + sun.direct_vis
            + shade.direct_vis
            + stem.direct_vis,
        diffuse_vis: ground_absorbed.diffuse_vis,
        direct_nir: ground_absorbed.direct_nir,
        diffuse_nir: ground_absorbed.diffuse_nir,
    };
    CoveredColumnInputs {
        authority: CoveredColumnAuthority::HistoricalV8,
        interval_s: ground.interval_s,
        tile_fraction: ground.tile_fraction,
        pressure_pa: ground.air_pressure_pa,
        air_temperature_k: ground.air_temperature_k,
        air_specific_humidity_kg_kg: ground.air_specific_humidity_kg_kg,
        reference_wind_m_s: ground.reference_wind_m_s,
        atmospheric_downward_longwave_w_m2: ground.atmospheric_downward_longwave_w_m2,
        ca_pa: 42.0,
        canopy_to_atmosphere_heat_resistance_s_m: 20.99,
        canopy_to_atmosphere_vapor_resistance_s_m: 22.73,
        latent_heat_j_kg: 2_501_000.0,
        top_rain_kg_m2_tile: 0.0,
        under_canopy_geometry: UnderCanopyGeometry {
            canopy_height_m: 12.5,
            canopy_roughness_m: 1.25,
            reference_height_m: 24.0,
            leaf_area_index: 2.7,
        },
        ground,
        occupancies: vec![occupancy],
        shortwave: CoveredColumnShortwaveInputs {
            incident_w_m2_tile: incident,
            top_reflected_w_m2_tile: BandDirectionalFluxes::default(),
            ground_absorbed_by_incident_w_m2_tile: ground_absorbed,
            occupancies: vec![CoveredOccupancyShortwaveInputs {
                occupancy_id: "canopy-rank-0".into(),
                sun_leaf_absorbed_w_m2_tile: sun,
                shade_leaf_absorbed_w_m2_tile: shade,
                stem_absorbed_w_m2_tile: stem,
            }],
        },
        stage3_lower_boundary: None,
        stage3_optical: None,
    }
}

fn identity(interval_s: f64) -> RuntimeTileIdentity {
    RuntimeTileIdentity {
        transaction_id: TransactionId(91),
        soil_thermal_transaction_id: TransactionId(91),
        lse_owner_id: owner("lse-v3"),
        hydrology_owner_id: owner("hydrology"),
        soil_thermal_owner_id: owner("soil"),
        vegetation_owner_id: owner("vegetation"),
        biogeochemistry_owner_id: owner("bgc"),
        configuration_sha256: digest('a'),
        beginning_lse_state_sha256: digest('b'),
        beginning_hydrology_snapshot_sha256: digest('c'),
        beginning_soil_thermal_state_sha256: digest('d'),
        beginning_vegetation_state_sha256: digest('e'),
        beginning_biogeochemistry_state_sha256: digest('f'),
        ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
        tile_id: TileId::try_new("forest").expect("tile"),
        surface_id: SurfaceId::try_new("litter").expect("surface"),
        surface_class: SurfaceClass::ForestLitter,
        ground_source_type: WaterSourceType::LitterLiquid,
        ground_source_id: crate::SourceId::try_new("litter").expect("source"),
        ground_source_tile_id: Some(TileId::try_new("forest").expect("tile")),
        ground_soil_layer_id: None,
        tile_fraction: 0.38,
        interval_s,
    }
}

fn initial_trial() -> Vec<f64> {
    vec![
        -17_238.6, -17_192.4, -17_085.3, -4_540.2, 0.413, 0.415, 303.6, 303.5, 306.4, 312.5, 300.1,
        0.01393, 296.0, 291.5, 289.8,
    ]
}

fn roots() -> Vec<RootRuntimeIdentity> {
    vec![RootRuntimeIdentity {
        solver_occupancy_id: "canopy-rank-0".into(),
        requesting_owner_id: owner("vegetation"),
        occupancy_id: ComponentId::try_new("canopy-rank-0").expect("occupancy"),
        layer_id: SoilLayerId::try_new("soil-1").expect("layer"),
        source_id: crate::SourceId::try_new("soil-1").expect("source"),
    }]
}

fn soil() -> SoilThermalSnapshot {
    SoilThermalSnapshot {
        owner_id: owner("soil"),
        configuration_sha256: digest('a'),
        state_sha256: digest('d'),
        snapshot_sha256: digest('9'),
        last_accepted_transaction_id: Some(TransactionId(90)),
        ofes: vec![SoilThermalOfeSnapshot {
            ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
            ordered_layers: vec![
                SoilThermalLayerSnapshot {
                    layer_id: SoilLayerId::try_new("thermal-1").expect("layer"),
                    temperature_k: 291.5,
                    enthalpy_j_m2_ofe_ground: 1.0e6,
                },
                SoilThermalLayerSnapshot {
                    layer_id: SoilLayerId::try_new("thermal-2").expect("layer"),
                    temperature_k: 289.8,
                    enthalpy_j_m2_ofe_ground: 2.0e6,
                },
            ],
        }],
    }
}

#[test]
fn potential_and_fixed_final_use_v3_and_return_the_accepted_evaluation() {
    let beginning = column();
    let beginning_bytes = serde_json::to_vec(&format!("{beginning:?}")).expect("beginning bytes");
    let potential = solve_v3_covered_potential_phase(
        identity(1_800.0),
        &beginning,
        roots(),
        &initial_trial(),
        configuration(),
        litter_beginning(),
    )
    .expect("V3 potential");
    let vapor = potential.accepted().evaluation.vapor.finalized;
    let phase_authorization = V3PhaseSpecificVaporAuthorization {
        liquid_outbound_rate_kg_m2_s: vapor.liquid_signed_rate_kg_m2_s.max(0.0),
        ice_outbound_rate_kg_m2_s: vapor.ice_signed_rate_kg_m2_s.max(0.0),
    };
    let amount = phase_authorization
        .aggregate_outbound_kg_m2_stand_ground(beginning.tile_fraction, beginning.interval_s)
        .expect("aggregate authorization");
    let authorizations = potential
        .request_batch()
        .requests
        .iter()
        .map(|request| WaterAuthorization {
            key: request.key.clone(),
            amount_kg_m2_stand_ground: if request.key.requesting_component
                == RequestingComponent::GroundSurface
            {
                amount
            } else {
                request.amount_kg_m2_stand_ground
            },
            reason: WaterAuthorizationReason::FullSupply,
        })
        .collect();
    let fixed = finalize_v3_covered_phase(
        &potential,
        &digest('b'),
        authorizations,
        phase_authorization,
        &potential.accepted().solution,
        SoilThermalFinalizationBeginning::V1(&soil()),
    )
    .expect("V3 fixed final");
    let accepted = &fixed.accepted_fixed_final.evaluation;
    assert_eq!(
        (accepted.vapor.finalized.liquid_signed_rate_kg_m2_s
            + accepted.vapor.finalized.ice_signed_rate_kg_m2_s)
            .to_bits(),
        accepted
            .predecessor
            .ground_water
            .final_kg_m2_tile_s
            .to_bits()
    );
    assert_eq!(
        fixed.water_protocol.requests,
        potential.request_batch().requests
    );
    assert_eq!(fixed.water_protocol.authorizations.len(), 2);
    assert_eq!(fixed.water_protocol.finalized_uses.len(), 2);
    let finalized_surface_use = fixed
        .water_protocol
        .finalized_uses
        .iter()
        .find(|row| row.key.requesting_component == RequestingComponent::GroundSurface)
        .expect("named V3 surface finalized use");
    assert_eq!(
        finalized_surface_use.amount_kg_m2_stand_ground.to_bits(),
        amount.to_bits(),
        "the publishable debit is the authorized aggregate of accepted liquid and ice vapor"
    );
    assert!(fixed.water_protocol.condensation_credits.is_empty());
    assert_eq!(
        serde_json::to_vec(&format!("{beginning:?}")).expect("ending bytes"),
        beginning_bytes
    );
}

#[test]
fn phase_free_v3_evaluation_admits_subfreezing_litter_without_legacy_liquid_domain() {
    let mut beginning = column();
    let mut litter = litter_beginning();
    let temperature_k = 270.0;
    let capacity = configuration().dry_heat_capacity_j_m2_k
        + litter.liquid_kg_m2_tile * crate::WATER_HEAT_CAPACITY_J_KG_K
        + litter.ice_kg_m2_tile * crate::LITTER_ICE_HEAT_CAPACITY_J_KG_K;
    litter.temperature_k = temperature_k;
    litter.sensible_energy_j_m2_tile = capacity * (temperature_k - crate::REFERENCE_TEMPERATURE_K);
    beginning.ground.surface_temperature_warm_start_k = temperature_k;
    beginning.ground.surface_enthalpy_j_m2_tile = litter.sensible_energy_j_m2_tile;
    let mut trial = initial_trial();
    trial[12] = temperature_k;

    let evaluation = evaluate_v3_phase_free_covered_column(
        &beginning,
        &trial,
        None,
        None,
        V3LitterResidualContext {
            configuration: configuration(),
            beginning: litter,
            finalized_vapor: None,
        },
    )
    .expect("subfreezing V3 evaluation");

    assert_eq!(
        evaluation
            .vapor
            .raw
            .environment
            .accepted_phase_free_temperature_k
            .to_bits(),
        temperature_k.to_bits()
    );
    assert!(evaluation.vapor.raw.raw_ice_signed_rate_kg_m2_s.is_finite());
}

#[test]
fn off_grid_support_rejects_before_any_v3_solve() {
    let beginning = column();
    let error = solve_v3_covered_potential_phase(
        identity(90.0),
        &beginning,
        roots(),
        &initial_trial(),
        configuration(),
        litter_beginning(),
    )
    .expect_err("off-grid support");
    assert!(matches!(
        error,
        LandSurfaceEnergyError::FrozenLitterTransaction(_)
    ));
}

#[test]
fn aggregate_and_named_authorization_mismatch_fails_closed() {
    let beginning = column();
    let potential = solve_v3_covered_potential_phase(
        identity(1_800.0),
        &beginning,
        roots(),
        &initial_trial(),
        configuration(),
        litter_beginning(),
    )
    .expect("V3 potential");
    let authorizations = potential
        .request_batch()
        .requests
        .iter()
        .map(|request| WaterAuthorization {
            key: request.key.clone(),
            amount_kg_m2_stand_ground: if request.key.requesting_component
                == RequestingComponent::GroundSurface
            {
                0.0
            } else {
                request.amount_kg_m2_stand_ground
            },
            reason: WaterAuthorizationReason::ZeroSupply,
        })
        .collect();
    let error = finalize_v3_covered_phase(
        &potential,
        &digest('b'),
        authorizations,
        V3PhaseSpecificVaporAuthorization {
            liquid_outbound_rate_kg_m2_s: 1.0e-9,
            ice_outbound_rate_kg_m2_s: 0.0,
        },
        &potential.accepted().solution,
        SoilThermalFinalizationBeginning::V1(&soil()),
    )
    .expect_err("mismatched aggregate authorization");
    assert!(matches!(
        error,
        LandSurfaceEnergyError::FrozenLitterVapor(_)
    ));
}

#[test]
fn successor_producer_binds_only_the_v3_covered_solver() {
    let source = include_str!("transaction_v3.rs");
    assert!(
        source
            .matches("solve_v3_phase_free_covered_column(")
            .count()
            >= 2
    );
    assert!(!source.contains("solve_covered_potential_phase("));
    assert!(!source.contains("finalize_covered_phase("));
    assert!(!source.contains("solve_covered_column("));
}
