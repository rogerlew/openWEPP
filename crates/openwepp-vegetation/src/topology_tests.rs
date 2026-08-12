use crate::VegetationConfiguration;
use crate::transaction::{
    CoupledOwnedState, SnowFreeForcing, SoilLayerForcing, radiation_by_stratum, rain_by_stratum,
};
use openwepp_kernel_contract::SoilLayerId;

fn forcing(rain: f64) -> SnowFreeForcing {
    SnowFreeForcing {
        air_temperature_k: 296.0,
        pressure_pa: 101_325.0,
        co2_pa: 40.0,
        vapor_pressure_deficit_kpa: 1.4,
        wind_m_s: 2.4,
        rain_kg_m2: rain,
        direct_par_w_m2: 620.0,
        diffuse_par_w_m2: 90.0,
        direct_nir_w_m2: 500.0,
        diffuse_nir_w_m2: 80.0,
        solar_zenith_cosine: 0.68,
        ground_albedo_vis: 0.14,
        ground_albedo_nir: 0.18,
        longwave_down_w_m2: 350.0,
        longwave_up_w_m2: 390.0,
        specific_humidity: 0.012,
        reference_height_m: 20.0,
        soil_layers: vec![SoilLayerForcing {
            layer_id: SoilLayerId::try_new("soil-1").expect("layer"),
            water_beginning_kg_m2: 100.0,
            matric_potential_mm: -5_000.0,
            hydraulic_conductivity_mm_s: 0.000_017,
            root_path_length_mm: 1.0,
            gravity_root_mm: 980.0,
            temperature_k: 294.0,
            accessible: true,
            frozen: false,
        }],
        gsi: 0.8,
    }
}

#[test]
fn multi_rank_columns_route_one_rain_and_shortwave_boundary() {
    let mut config: VegetationConfiguration = serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/c3_woody_v1_diagnostic_configuration.json"
    ))
    .expect("configuration");
    let mut state: CoupledOwnedState = serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/c3_woody_v1_diagnostic_state.json"
    ))
    .expect("state");
    let mut lower = config.strata[0].clone();
    lower.stratum_id = "tree-2".into();
    lower.vertical_rank = 1;
    config.strata.push(lower);
    state
        .strata
        .insert("tree-2".into(), state.strata["tree-1"].clone());

    let forcing = forcing(2.0);
    let rain = rain_by_stratum(&config, &state, &forcing).expect("rain routing");
    assert!((rain["tree-1"].0 - forcing.rain_kg_m2).abs() < f64::EPSILON);
    assert!(rain["tree-2"].0 < rain["tree-1"].0);
    assert!(rain["tree-1"].1.abs() < f64::EPSILON);
    assert!((rain["tree-2"].1 - 1.0).abs() < f64::EPSILON);

    let radiation = radiation_by_stratum(&config, &state, &forcing).expect("radiation routing");
    let incident = forcing.direct_par_w_m2
        + forcing.diffuse_par_w_m2
        + forcing.direct_nir_w_m2
        + forcing.diffuse_nir_w_m2;
    let absorbed = radiation
        .values()
        .map(|bands| bands.0.absorbed + bands.1.absorbed)
        .sum::<f64>();
    let reflected = radiation
        .values()
        .map(|bands| bands.0.reflected + bands.1.reflected)
        .sum::<f64>();
    let terminal = radiation
        .values()
        .map(|bands| {
            (1.0 - forcing.ground_albedo_vis)
                * (bands.0.terminal_from_direct + bands.0.terminal_from_diffuse)
                + (1.0 - forcing.ground_albedo_nir)
                    * (bands.1.terminal_from_direct + bands.1.terminal_from_diffuse)
        })
        .sum::<f64>();
    let raw_terminal = {
        let bands = &radiation["tree-2"];
        (1.0 - forcing.ground_albedo_vis)
            * (bands.0.transmitted_direct + bands.0.transmitted_diffuse)
            + (1.0 - forcing.ground_albedo_nir)
                * (bands.1.transmitted_direct + bands.1.transmitted_diffuse)
    };
    let residual = incident - absorbed - reflected - terminal;
    assert!(
        residual.abs() < 1e-8,
        "column residual {residual}; incident={incident} absorbed={absorbed} reflected={reflected} terminal={terminal} raw={raw_terminal}"
    );
}

#[test]
fn stem_optics_absorb_without_creating_photosynthetic_leaf_area() {
    let config: VegetationConfiguration = serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/c3_woody_v1_diagnostic_configuration.json"
    ))
    .expect("configuration");
    let mut state: CoupledOwnedState = serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/c3_woody_v1_diagnostic_state.json"
    ))
    .expect("state");
    state.strata.get_mut("tree-1").expect("stratum").leaf_area = 0.0;
    let radiation = radiation_by_stratum(&config, &state, &forcing(0.0)).expect("stem radiation");
    let (vis, nir) = &radiation["tree-1"];
    assert!((vis.sunlit_lai + vis.shaded_lai).abs() < f64::EPSILON);
    assert!((nir.sunlit_lai + nir.shaded_lai).abs() < f64::EPSILON);
    assert!(vis.absorbed + nir.absorbed > 0.0);
    assert!((vis.sunlit_absorbed + vis.shaded_absorbed).abs() < f64::EPSILON);
    assert!((nir.sunlit_absorbed + nir.shaded_absorbed).abs() < f64::EPSILON);
}

#[test]
fn fractional_tile_preserves_stand_ground_leaf_area_basis() {
    let mut config: VegetationConfiguration = serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/c3_woody_v1_diagnostic_configuration.json"
    ))
    .expect("configuration");
    config.topology_tiles[0].fraction = 0.5;
    config.topology_tiles.push(crate::TopologyTile {
        tile_id: "empty".into(),
        fraction: 0.5,
    });
    let state: CoupledOwnedState = serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/c3_woody_v1_diagnostic_state.json"
    ))
    .expect("state");
    let expected = state.strata["tree-1"].leaf_area;
    let radiation =
        radiation_by_stratum(&config, &state, &forcing(0.0)).expect("fractional-tile radiation");
    let visible = radiation["tree-1"].0;
    assert!((visible.sunlit_lai + visible.shaded_lai - expected).abs() < 1e-12);
}
