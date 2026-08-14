use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::land_surface_energy_shadow::{
    BandDirectionalFluxes, BareSoilParameters, ComponentId, GroundWaterKey,
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError, MixedRealHydrologyRequest,
    MixedRealHydrologyUse, OfeId, OpenNeutralGeometry, OpenSurfaceProblem, SoilThermalNodeOperands,
    SourceId, StandGroundWaterAmountBasis, SurfaceClass, SurfaceClassKind, SurfaceId,
    SurfaceStorageBranch, WaterSourceType, execute_open_bare_soil_shadow,
};
use openwepp_hillslope_orchestrator::vegetation_real_hydrology_shadow::{
    RealHydrologyLaneLayerMap, RealHydrologyOfeLaneId, RealHydrologyShadowAdapter,
    RealHydrologySourceKey,
};
use openwepp_hillslope_orchestrator::{
    DirectRunFrame, DirectRunIdentity, DirectSubsurfaceLayerState,
};
use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};

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
    let layer = SoilLayerId::try_new("soil-1").expect("layer");
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
            key: key(component, "soil-1"),
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
        key("ground", "soil-1"),
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
        row.authorization.amount_kg_m2_stand_ground == 10.0
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
        candidate.ending_frame().lanes[0].subsurface_layers[0].theta_m,
        0.0
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
            .amount_kg_m2_stand_ground,
        0.0
    );
    assert_eq!(
        result.authorizations()[0].authorization.reason,
        openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAuthorizationReason::FrozenSource
    );
}

#[test]
fn litter_and_condensation_remain_typed_unsupported_without_production_store() {
    let frame = production_frame(0.02, false);
    let (real_owner, source) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&real_owner);
    let mut litter = request("ground", 1.0, &source);
    litter.request.key.source_type = WaterSourceType::LitterLiquid;
    litter.request.key.source_tile_id = Some(TileId::try_new("open").expect("tile"));
    litter.request.key.soil_layer_id = None;
    assert!(matches!(
        adapter.authorize(&[litter]),
        Err(LandSurfaceEnergyShadowError::UnsupportedCustody(_))
    ));
    assert!(matches!(
        adapter.reject_condensation_credit(),
        LandSurfaceEnergyShadowError::UnsupportedCustody(_)
    ));
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
    rust_sources_below(Path::new("crates/openwepp-runner/src"), &mut sources);
    rust_sources_below(
        Path::new("crates/openwepp-hillslope-orchestrator/src/direct_runtime"),
        &mut sources,
    );
    for path in sources {
        let source = fs::read_to_string(&path).expect("production source");
        assert!(!source.contains("execute_open_bare_soil_shadow"));
        assert!(!source.contains("LandSurfaceEnergyRealHydrologyAdapter"));
    }
}
