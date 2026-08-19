use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::vegetation_real_hydrology_shadow::{
    RealHydrologyLaneLayerMap, RealHydrologyOfeLaneId, RealHydrologyShadowAdapter,
    execute_v7_real_hydrology_water_shadow,
};
use openwepp_hillslope_orchestrator::{
    DirectRunFrame, DirectRunIdentity, DirectSubsurfaceLayerState,
};
use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId};
use openwepp_vegetation::migration::V5_MODEL_SHA256;
use openwepp_vegetation::{
    CoupledOwnedState, MODEL_SHA256, SnowFreeForcing, SoilLayerForcing, VegetationConfiguration,
    load_model_definition,
};

fn v7_fixture() -> (VegetationConfiguration, CoupledOwnedState) {
    let mut configuration: VegetationConfiguration = serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v5_diagnostic_configuration.json")
            .expect("historical configuration fixture"),
    )
    .expect("historical configuration DTO");
    assert_eq!(configuration.model_definition_sha256, V5_MODEL_SHA256);
    for stratum in &mut configuration.strata {
        if stratum.phenology_type == openwepp_vegetation::PhenologyType::Evergreen {
            stratum.current_growth_fraction = 1.0;
        }
    }
    configuration.model_definition_sha256 = MODEL_SHA256.into();
    configuration.configuration_sha256 = configuration
        .canonical_sha256()
        .expect("V7 configuration digest");

    let mut state: CoupledOwnedState = serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v5_diagnostic_state.json")
            .expect("historical state fixture"),
    )
    .expect("historical state DTO");
    state.model_definition_sha256 = MODEL_SHA256.into();
    state
        .configuration_sha256
        .clone_from(&configuration.configuration_sha256);
    state.state_sha256 = state.canonical_sha256().expect("V7 state digest");
    configuration
        .initial_state_sha256
        .clone_from(&state.state_sha256);
    state.validate(&configuration).expect("V7 state");
    (configuration, state)
}

fn forcing() -> SnowFreeForcing {
    SnowFreeForcing {
        air_temperature_k: 298.15,
        pressure_pa: 101_325.0,
        co2_pa: 42.0,
        vapor_pressure_deficit_kpa: 1.2,
        wind_m_s: 3.7,
        rain_kg_m2: 0.0,
        direct_par_w_m2: 410.0,
        diffuse_par_w_m2: 83.0,
        direct_nir_w_m2: 355.0,
        diffuse_nir_w_m2: 101.0,
        solar_zenith_cosine: 0.67,
        ground_albedo_vis: 0.14,
        ground_albedo_nir: 0.31,
        longwave_down_w_m2: 350.0,
        longwave_up_w_m2: 390.0,
        specific_humidity: 0.01,
        reference_height_m: 20.0,
        soil_layers: vec![SoilLayerForcing {
            layer_id: SoilLayerId::try_new("soil-1").expect("layer"),
            water_beginning_kg_m2: 20.0,
            matric_potential_mm: -1_000.0,
            hydraulic_conductivity_mm_s: 1.0e-5,
            root_path_length_mm: 100.0,
            gravity_root_mm: 500.0,
            temperature_k: 295.0,
            accessible: true,
            frozen: false,
        }],
        root_zone_hydraulics: None,
        gsi: 1.0,
    }
}

fn production_frame() -> DirectRunFrame {
    let identity = DirectRunIdentity::new(71, 9, 1, 1).expect("identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("frame");
    frame.lanes[0].area_m2 = 120.0;
    frame.lanes[0].subsurface_layers = vec![DirectSubsurfaceLayerState {
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
    }];
    frame.lanes[0].water.soil_water_m = 0.02;
    frame
}

fn adapter(
    frame: &DirectRunFrame,
    transaction: u128,
    interval_s: f64,
) -> (RealHydrologyShadowAdapter, RealHydrologyOfeLaneId) {
    let ofe_lane = RealHydrologyOfeLaneId {
        lane_index: 0,
        lane_id: frame.lanes[0].lane_id,
    };
    let adapter = RealHydrologyShadowAdapter::try_from_day_start(
        frame,
        0,
        openwepp_kernel_contract::TransactionId(transaction),
        interval_s,
        ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner"),
        &[RealHydrologyLaneLayerMap {
            ofe_lane,
            layer_ids: vec![SoilLayerId::try_new("soil-1").expect("layer")],
        }],
    )
    .expect("real hydrology adapter");
    (adapter, ofe_lane)
}

#[test]
fn public_v7_two_pass_uses_real_day_start_hydrology_candidate() {
    let (configuration, beginning) = v7_fixture();
    let frame = production_frame();
    let original = frame.clone();
    let (adapter, ofe_lane) = adapter(&frame, 1, configuration.dt_s);

    let result = execute_v7_real_hydrology_water_shadow(
        &load_model_definition().expect("model"),
        &configuration,
        &beginning,
        &forcing(),
        adapter,
        ofe_lane,
    )
    .expect("V7 real-hydrology water shadow");

    let (requests, authorizations, uses) = result.water_phase.protocol();
    assert!(!requests.is_empty());
    assert_eq!(requests.len(), authorizations.len());
    assert_eq!(requests.len(), uses.len());
    assert!(requests.iter().zip(authorizations).zip(uses).all(
        |((request, authorization), use_record)| {
            use_record.amount <= authorization.amount && authorization.amount <= request.amount
        }
    ));
    assert_eq!(frame, original, "production owner changed during shadow");
    assert_eq!(result.real_hydrology_candidate.beginning_frame(), &original);
    assert!(
        result.real_hydrology_candidate.ending_frame().lanes[0].subsurface_layers[0].theta_m
            <= original.lanes[0].subsurface_layers[0].theta_m
    );
}

#[test]
fn public_bridge_rejects_cross_snapshot_water_interval_and_transaction() {
    let (configuration, beginning) = v7_fixture();
    let frame = production_frame();
    let original = frame.clone();
    let model = load_model_definition().expect("model");

    let (wrong_interval, ofe_lane) = adapter(&frame, 1, configuration.dt_s / 2.0);
    assert!(
        execute_v7_real_hydrology_water_shadow(
            &model,
            &configuration,
            &beginning,
            &forcing(),
            wrong_interval,
            ofe_lane,
        )
        .is_err()
    );

    let (wrong_transaction, ofe_lane) = adapter(&frame, 2, configuration.dt_s);
    assert!(
        execute_v7_real_hydrology_water_shadow(
            &model,
            &configuration,
            &beginning,
            &forcing(),
            wrong_transaction,
            ofe_lane,
        )
        .is_err()
    );

    let mut wrong_water = forcing();
    wrong_water.soil_layers[0].water_beginning_kg_m2 = 19.0;
    let (water_adapter, ofe_lane) = adapter(&frame, 1, configuration.dt_s);
    assert!(
        execute_v7_real_hydrology_water_shadow(
            &model,
            &configuration,
            &beginning,
            &wrong_water,
            water_adapter,
            ofe_lane,
        )
        .is_err()
    );
    assert_eq!(frame, original);
}

fn rust_sources_below(path: &Path, sources: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(path).expect("production source directory") {
        let entry = entry.expect("production source entry");
        let path = entry.path();
        if path.is_dir() {
            rust_sources_below(&path, sources);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(path);
        }
    }
}

#[test]
fn real_hydrology_shadow_has_no_runner_or_production_dispatch_path() {
    let mut sources = Vec::new();
    rust_sources_below(Path::new("crates/openwepp-runner/src"), &mut sources);
    rust_sources_below(
        Path::new("crates/openwepp-hillslope-orchestrator/src/direct_runtime"),
        &mut sources,
    );
    assert!(!sources.is_empty());
    for path in sources {
        let source = fs::read_to_string(&path).expect("production source");
        assert!(!source.contains("execute_v7_real_hydrology_water_shadow"));
        assert!(!source.contains("RealHydrologyShadowAdapter"));
        assert!(!source.contains("OPENWEPP_VEGETATION_REAL_HYDROLOGY"));
    }
}
