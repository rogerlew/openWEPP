//! Ordinary Rust conformance against the committed covered-column authority vectors.

use std::collections::BTreeMap;

use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::physics::{BandDirectionalFluxes, OpenNeutralGeometry, UnderCanopyGeometry};
use crate::solver::{
    BiochemicalConstants, CoveredColumnCandidate, CoveredColumnInputs,
    CoveredColumnShortwaveInputs, CoveredColumnSolveOutcome, CoveredOccupancyInputs,
    CoveredOccupancyShortwaveInputs, CoveredWaterCaps, LeafBiochemicalInputs, NumericalFailure,
    NumericalFailureKind, OpenSurfaceProblem, RootHydraulicLayer, SoilThermalNodeOperands,
    SourceWaterCap, SurfaceClassKind, SurfaceStorageBranch, WaterBranch, solve_covered_column,
};

const VECTOR_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/openwepp_snow_free_lse_v1_vectors.json"
));
const VECTOR_SHA256: &str = "3fb57d7c637abba20659a59e6eb1487f9f4130f909e17b61c8a6f2eb70f4c711";
const RUST_FAILURE_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/openwepp_lse_rust_failure_diagnostics_v1.json"
));
const RUST_FAILURE_SHA256: &str =
    "bdf18078cede6895d25146b6feb9fa7c90aadc436532d4485c43de7ae9b26211";

fn fixture() -> Value {
    assert_eq!(format!("{:x}", Sha256::digest(VECTOR_BYTES)), VECTOR_SHA256);
    serde_json::from_slice(VECTOR_BYTES).expect("covered authority vectors parse")
}

fn rust_failure_fixture() -> Value {
    assert_eq!(
        format!("{:x}", Sha256::digest(RUST_FAILURE_BYTES)),
        RUST_FAILURE_SHA256
    );
    serde_json::from_slice(RUST_FAILURE_BYTES).expect("Rust failure authority parse")
}

#[test]
fn covered_failed_solution_reports_beta_component_shared_and_soil_bounds() {
    let fixture = fixture();
    let expected = &fixture["exact_model_reductions"]["covered_single_rank"]["potential"];
    let (column, start) = column(&fixture, 1, expected);
    let detail = crate::solver::evaluate_covered_column(&column, &start, None, None)
        .expect("bound metadata reference evaluation");
    let mut failed = start;
    for (index, value) in [
        (4, 0.0),
        (5, 1.0),
        (6, 200.0),
        (7, 350.0),
        (8, 200.0),
        (9, 350.0),
        (10, 200.0),
        (11, 0.1),
        (12, 350.0),
        (13, 200.0),
        (14, 350.0),
    ] {
        failed[index] = value;
    }
    let (_, bounds) = crate::solver::covered_failure_metadata(&column, &detail, &failed);
    for expected in [
        "canopy-rank-0:beta_sun:lower",
        "canopy-rank-0:beta_shade:upper",
        "canopy-rank-0:sun_leaf_temperature_k:lower",
        "canopy-rank-0:shade_leaf_temperature_k:upper",
        "canopy-rank-0:wet_surface_temperature_k:lower",
        "canopy-rank-0:dry_stem_temperature_k:upper",
        "shared_canopy_air_temperature_k:lower",
        "shared_canopy_air_specific_humidity_kg_kg:upper",
        "ground_surface_temperature_k:upper",
        "soil_temperature_k:thermal-1:lower",
        "soil_temperature_k:thermal-2:upper",
    ] {
        assert!(
            bounds.iter().any(|bound| bound == expected),
            "missing {expected}"
        );
    }
}

fn number(value: &Value, key: &str) -> f64 {
    value[key]
        .as_f64()
        .unwrap_or_else(|| panic!("missing numeric fixture field {key}"))
}

fn close(actual: f64, expected: f64, identity: &str) {
    let tolerance = 2.0e-9 + 2.0e-9 * expected.abs().max(actual.abs());
    assert!(
        (actual - expected).abs() <= tolerance,
        "{identity}: actual={actual:.17e} expected={expected:.17e} tolerance={tolerance:.3e}"
    );
}

fn close_slice(actual: &[f64], expected: &Value, identity: &str) {
    let expected = expected.as_array().expect("numeric fixture array");
    assert_eq!(actual.len(), expected.len(), "{identity} cardinality");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        close(
            *actual,
            expected.as_f64().expect("numeric fixture element"),
            &format!("{identity}[{index}]"),
        );
    }
}

fn band(value: &Value) -> BandDirectionalFluxes {
    BandDirectionalFluxes {
        direct_vis: number(value, "direct_vis"),
        diffuse_vis: number(value, "diffuse_vis"),
        direct_nir: number(value, "direct_nir"),
        diffuse_nir: number(value, "diffuse_nir"),
    }
}

fn directional_shortwave(detail: &Value) -> CoveredColumnShortwaveInputs {
    let by_band = &detail["whole_column_shortwave"]["by_band"];
    let component = |band_name: &str, direction: &str, occupancy: usize, owner: &str| {
        number(
            &by_band[band_name][direction]["occupancies"][occupancy]["results"],
            owner,
        )
    };
    let flux = |occupancy: usize, owner: &str| BandDirectionalFluxes {
        direct_vis: component("VIS", "direct", occupancy, owner),
        diffuse_vis: component("VIS", "diffuse", occupancy, owner),
        direct_nir: component("NIR", "direct", occupancy, owner),
        diffuse_nir: component("NIR", "diffuse", occupancy, owner),
    };
    let occupancy_count = by_band["VIS"]["direct"]["occupancies"]
        .as_array()
        .expect("shortwave occupancies")
        .len();
    let occupancies = (0..occupancy_count)
        .map(|index| CoveredOccupancyShortwaveInputs {
            occupancy_id: by_band["VIS"]["direct"]["occupancies"][index]["occupancy_id"]
                .as_str()
                .expect("shortwave occupancy identity")
                .to_owned(),
            sun_leaf_absorbed_w_m2_tile: flux(index, "absorbed_leaf_sun"),
            shade_leaf_absorbed_w_m2_tile: flux(index, "absorbed_leaf_shade"),
            stem_absorbed_w_m2_tile: flux(index, "absorbed_stem"),
        })
        .collect::<Vec<_>>();
    CoveredColumnShortwaveInputs {
        incident_w_m2_tile: BandDirectionalFluxes {
            direct_vis: number(&by_band["VIS"]["direct"], "incident_direct"),
            diffuse_vis: number(&by_band["VIS"]["diffuse"], "incident_diffuse"),
            direct_nir: number(&by_band["NIR"]["direct"], "incident_direct"),
            diffuse_nir: number(&by_band["NIR"]["diffuse"], "incident_diffuse"),
        },
        top_reflected_w_m2_tile: BandDirectionalFluxes {
            direct_vis: number(&by_band["VIS"]["direct"], "top_reflected"),
            diffuse_vis: number(&by_band["VIS"]["diffuse"], "top_reflected"),
            direct_nir: number(&by_band["NIR"]["direct"], "top_reflected"),
            diffuse_nir: number(&by_band["NIR"]["diffuse"], "top_reflected"),
        },
        ground_absorbed_by_incident_w_m2_tile: BandDirectionalFluxes {
            direct_vis: number(&by_band["VIS"]["direct"], "ground_absorbed"),
            diffuse_vis: number(&by_band["VIS"]["diffuse"], "ground_absorbed"),
            direct_nir: number(&by_band["NIR"]["direct"], "ground_absorbed"),
            diffuse_nir: number(&by_band["NIR"]["diffuse"], "ground_absorbed"),
        },
        occupancies,
    }
}

fn synthetic_single_shortwave(
    occupancy: &CoveredOccupancyInputs,
    terminal: BandDirectionalFluxes,
    surface_vis_albedo: f64,
    surface_nir_albedo: f64,
) -> CoveredColumnShortwaveInputs {
    let sun = occupancy.sun.absorbed_shortwave_w_m2_tile;
    let shade = occupancy.shade.absorbed_shortwave_w_m2_tile;
    let stem = occupancy.stem_absorbed_shortwave_w_m2_tile;
    let ground_absorbed = BandDirectionalFluxes {
        direct_vis: terminal.direct_vis * (1.0 - surface_vis_albedo),
        diffuse_vis: terminal.diffuse_vis * (1.0 - surface_vis_albedo),
        direct_nir: terminal.direct_nir * (1.0 - surface_nir_albedo),
        diffuse_nir: terminal.diffuse_nir * (1.0 - surface_nir_albedo),
    };
    CoveredColumnShortwaveInputs {
        incident_w_m2_tile: BandDirectionalFluxes {
            direct_vis: ground_absorbed.direct_vis + sun + shade + stem,
            diffuse_vis: ground_absorbed.diffuse_vis,
            direct_nir: ground_absorbed.direct_nir,
            diffuse_nir: ground_absorbed.diffuse_nir,
        },
        top_reflected_w_m2_tile: BandDirectionalFluxes::default(),
        ground_absorbed_by_incident_w_m2_tile: ground_absorbed,
        occupancies: vec![CoveredOccupancyShortwaveInputs {
            occupancy_id: occupancy.occupancy_id.clone(),
            sun_leaf_absorbed_w_m2_tile: BandDirectionalFluxes {
                direct_vis: sun,
                ..Default::default()
            },
            shade_leaf_absorbed_w_m2_tile: BandDirectionalFluxes {
                direct_vis: shade,
                ..Default::default()
            },
            stem_absorbed_w_m2_tile: BandDirectionalFluxes {
                direct_vis: stem,
                ..Default::default()
            },
        }],
    }
}

fn biochemical(value: &Value) -> BiochemicalConstants {
    BiochemicalConstants {
        ha_vcmax_j_mol: number(value, "ha_vcmax_j_mol"),
        hd_vcmax_j_mol: number(value, "hd_vcmax_j_mol"),
        entropy_vcmax_j_mol_k: number(value, "entropy_vcmax_j_mol_k"),
        ha_jmax_j_mol: number(value, "ha_jmax_j_mol"),
        hd_jmax_j_mol: number(value, "hd_jmax_j_mol"),
        entropy_jmax_j_mol_k: number(value, "entropy_jmax_j_mol_k"),
        kc25_pa: number(value, "kc25_pa"),
        ha_kc_j_mol: number(value, "ha_kc_j_mol"),
        ko25_pa: number(value, "ko25_pa"),
        ha_ko_j_mol: number(value, "ha_ko_j_mol"),
        gamma25_pa: number(value, "gamma25_pa"),
        ha_gamma_j_mol: number(value, "ha_gamma_j_mol"),
        oxygen_partial_pressure_pa: number(value, "oxygen_partial_pressure_pa"),
        tp_vcmax_ratio: number(value, "tp_vcmax_ratio"),
        electron_quantum_yield: number(value, "electron_quantum_yield"),
        par_photon_umol_per_j: number(value, "par_photon_umol_per_j"),
        electron_curvature: number(value, "electron_curvature"),
        ac_aj_curvature: number(value, "ac_aj_curvature"),
        ag_ap_curvature: number(value, "ag_ap_curvature"),
    }
}

fn occupancy(value: &Value) -> CoveredOccupancyInputs {
    let case = &value["case"];
    let parameters = &case["parameters"];
    let gas = &case["gas_energy"];
    let class = |name: &str| {
        let value = &case["classes"][name];
        LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: number(value, "leaf_area"),
            absorbed_shortwave_w_m2_tile: number(value, "absorbed_shortwave_w_m2_tile"),
            absorbed_par_w_m2_leaf: number(value, "absorbed_par_w_m2_leaf"),
            vcmax25: number(value, "vcmax25"),
            jmax25: number(value, "jmax25"),
            rd25: number(value, "rd25"),
        }
    };
    let lai = number(parameters, "lai");
    let sai = number(parameters, "sai");
    CoveredOccupancyInputs {
        occupancy_id: value["occupancy_id"]
            .as_str()
            .expect("occupancy id")
            .to_owned(),
        medlyn_g1_kpa_sqrt: number(gas, "medlyn_g1_kpa_sqrt"),
        g0_umol_m2_s: number(gas, "g0_umol_m2_s"),
        sun: class("sun"),
        shade: class("shade"),
        biochemical: biochemical(&case["biochemical_parameters"]),
        stem_area_m2_m2_tile: number(gas, "stem_area"),
        stem_absorbed_shortwave_w_m2_tile: number(gas, "stem_absorbed_shortwave_w_m2_tile"),
        beginning_canopy_liquid_kg_m2_tile: number(gas, "canopy_liquid_kg_m2_tile"),
        liquid_interception_fraction: 0.35,
        liquid_capacity_kg_m2_plant: 0.08 / (lai + sai),
        stemflow_fraction: 0.08,
        gb_leaf_m_s: number(gas, "gb_leaf_m_s"),
        gb_wet_m_s: number(gas, "gb_wet_m_s"),
        gb_stem_m_s: number(gas, "gb_stem_m_s"),
        lai,
        sai,
        clumping_index: number(&case["canopy_longwave"], "clumping_index"),
        k1_sun_max_s1: number(parameters, "k1_max"),
        k1_shade_max_s1: number(parameters, "k1_max"),
        k2_max: number(parameters, "k2_max"),
        k3_max_m_s: number(parameters, "k3_max_m_s"),
        height_m: number(parameters, "height_m"),
        root_to_leaf_area: number(parameters, "root_to_leaf_area"),
        p50_leaf_mm: number(parameters, "p50_leaf"),
        p50_xylem_mm: number(parameters, "p50_xylem"),
        p50_root_mm: number(parameters, "p50_root"),
        vulnerability_exponent: number(parameters, "ck"),
        root_layers: case["layers"]
            .as_array()
            .expect("root layers")
            .iter()
            .map(|layer| RootHydraulicLayer {
                layer_id: layer["layer_id"].as_str().expect("layer id").to_owned(),
                accessible: layer["accessible"].as_bool().expect("accessible"),
                frozen: layer["frozen"].as_bool().expect("frozen"),
                root_fraction: number(layer, "root_fraction"),
                soil_potential_mm: number(layer, "soil_potential_mm"),
                gravity_head_mm: number(layer, "gravity_head_mm"),
                z3_m: number(layer, "z3_m"),
                dxroot_m: number(layer, "dxroot_m"),
                ksoil_m2_s: number(layer, "ksoil_m2_s"),
            })
            .collect(),
    }
}

fn covered_start(
    primitive: &Value,
    rank_count: usize,
    gas: &Value,
    ground_state: &Value,
) -> Vec<f64> {
    (0..rank_count)
        .flat_map(|index| {
            primitive["occupancies"][index]["start"]
                .as_array()
                .expect("occupancy start")
                .iter()
                .map(|value| value.as_f64().expect("start value"))
                .collect::<Vec<_>>()
        })
        .chain([
            number(gas, "canopy_air_temperature_start_k"),
            number(gas, "qcan_start_kg_kg"),
            number(ground_state, "surface_temperature_warm_start_k"),
        ])
        .chain(
            ground_state["soil_temperature_k"]
                .as_array()
                .expect("soil temperatures")
                .iter()
                .map(|value| value.as_f64().expect("soil temperature")),
        )
        .collect()
}

fn prepared_shortwave(
    occupancies: &mut [CoveredOccupancyInputs],
    terminal: BandDirectionalFluxes,
    ground_config: &Value,
    expected: &Value,
) -> CoveredColumnShortwaveInputs {
    if occupancies.len() == 1 {
        return synthetic_single_shortwave(
            &occupancies[0],
            terminal,
            number(ground_config, "ground_surface_albedo_vis"),
            number(ground_config, "ground_surface_albedo_nir"),
        );
    }
    let prepared = directional_shortwave(&expected["detail"]);
    for (index, source) in occupancies.iter_mut().enumerate() {
        let radiation = &prepared.occupancies[index];
        let results = &expected["detail"]["whole_column_shortwave"]["by_band"]["VIS"]["direct"]["occupancies"]
            [index]["results"];
        source.sun.leaf_area_m2_m2_tile = number(results, "leaf_sun_area");
        source.shade.leaf_area_m2_m2_tile = number(results, "leaf_shade_area");
        source.sun.absorbed_shortwave_w_m2_tile = radiation.sun_leaf_absorbed_w_m2_tile.total();
        source.shade.absorbed_shortwave_w_m2_tile = radiation.shade_leaf_absorbed_w_m2_tile.total();
        source.stem_absorbed_shortwave_w_m2_tile = radiation.stem_absorbed_w_m2_tile.total();
        source.sun.absorbed_par_w_m2_leaf = (radiation.sun_leaf_absorbed_w_m2_tile.direct_vis
            + radiation.sun_leaf_absorbed_w_m2_tile.diffuse_vis)
            / source.sun.leaf_area_m2_m2_tile;
        source.shade.absorbed_par_w_m2_leaf = (radiation.shade_leaf_absorbed_w_m2_tile.direct_vis
            + radiation.shade_leaf_absorbed_w_m2_tile.diffuse_vis)
            / source.shade.leaf_area_m2_m2_tile;
    }
    prepared
}

fn covered_primitive(fixture: &Value) -> &Value {
    &fixture["mandatory_exact_scenario_vectors"]["covered_column"]["primitive_input"]
}

fn column(fixture: &Value, rank_count: usize, expected: &Value) -> (CoveredColumnInputs, Vec<f64>) {
    let primitive = covered_primitive(fixture);
    let ground_config = &primitive["ground_config"];
    let ground_state = &primitive["ground_state"];
    let first_case = &primitive["occupancies"][0]["case"];
    let gas = &first_case["gas_energy"];
    let geometry = &ground_config["under_canopy_geometry"];
    let mut occupancies = primitive["occupancies"]
        .as_array()
        .expect("occupancy inputs")[..rank_count]
        .iter()
        .map(occupancy)
        .collect::<Vec<_>>();
    let terminal = band(&ground_config["ground_terminal_shortwave_by_band_direction_w_m2_tile"]);
    let shortwave = prepared_shortwave(&mut occupancies, terminal, ground_config, expected);
    let ground = OpenSurfaceProblem {
        interval_s: number(first_case, "dt_s"),
        tile_fraction: number(first_case, "tile_fraction"),
        class: SurfaceClassKind::ForestLitter,
        storage_branch: SurfaceStorageBranch::FiniteCapacity,
        terminal_shortwave_w_m2_tile: if rank_count == 1 {
            terminal
        } else {
            band(&expected["detail"]["ground_terminal_shortwave_by_band_direction_w_m2_tile"])
        },
        surface_vis_albedo: number(ground_config, "ground_surface_albedo_vis"),
        surface_nir_albedo: number(ground_config, "ground_surface_albedo_nir"),
        surface_emissivity: 1.0,
        surface_depth_m: number(ground_config, "surface_depth_m"),
        surface_conductivity_w_m_k: number(ground_config, "surface_conductivity_w_m_k"),
        surface_dry_heat_capacity_j_m2_k: number(ground_config, "surface_dry_heat_capacity_j_m2_k"),
        litter_capacity_kg_m2_tile: Some(number(ground_config, "litter_capacity_kg_m2_tile")),
        open_geometry: OpenNeutralGeometry {
            reference_height_m: number(&gas["reference_wind_operands"], "z_ref_m"),
            roughness_momentum_m: number(&gas["reference_wind_operands"], "z0m_m"),
            roughness_heat_m: number(&gas["reference_wind_operands"], "z0h_m"),
            roughness_vapor_m: number(&gas["reference_wind_operands"], "z0q_m"),
        },
        air_temperature_k: number(gas, "air_temperature_k"),
        air_specific_humidity_kg_kg: number(gas, "air_specific_humidity_kg_kg"),
        air_pressure_pa: number(gas, "pressure_pa"),
        reference_wind_m_s: number(&gas["reference_wind_operands"], "u_ref_m_s"),
        atmospheric_downward_longwave_w_m2: number(
            &first_case["canopy_longwave"],
            "atmospheric_down_w_m2",
        ),
        surface_liquid_kg_m2_tile: number(ground_state, "surface_liquid_kg_m2_tile"),
        surface_enthalpy_j_m2_tile: number(ground_state, "surface_enthalpy_j_m2_tile"),
        surface_temperature_warm_start_k: number(ground_state, "surface_temperature_warm_start_k"),
        bare_soil: None,
        soil_nodes: ground_config["soil_nodes"]
            .as_array()
            .expect("soil nodes")
            .iter()
            .zip(
                ground_state["soil_temperature_k"]
                    .as_array()
                    .expect("soil temperatures"),
            )
            .map(|(node, temperature)| SoilThermalNodeOperands {
                layer_id: node["layer_id"]
                    .as_str()
                    .expect("thermal layer id")
                    .to_owned(),
                depth_m: number(node, "depth_m"),
                conductivity_w_m_k: number(node, "conductivity_w_m_k"),
                heat_capacity_j_m2_k: number(node, "heat_capacity_j_m2_k"),
                beginning_temperature_k: temperature.as_f64().expect("soil temperature"),
            })
            .collect(),
    };
    let start = covered_start(primitive, rank_count, gas, ground_state);
    (
        CoveredColumnInputs {
            interval_s: number(first_case, "dt_s"),
            tile_fraction: number(first_case, "tile_fraction"),
            pressure_pa: number(gas, "pressure_pa"),
            air_temperature_k: number(gas, "air_temperature_k"),
            air_specific_humidity_kg_kg: number(gas, "air_specific_humidity_kg_kg"),
            reference_wind_m_s: number(&gas["reference_wind_operands"], "u_ref_m_s"),
            atmospheric_downward_longwave_w_m2: number(
                &first_case["canopy_longwave"],
                "atmospheric_down_w_m2",
            ),
            ca_pa: number(gas, "ca_pa"),
            canopy_to_atmosphere_heat_resistance_s_m: number(gas, "rah_s_m"),
            canopy_to_atmosphere_vapor_resistance_s_m: number(gas, "raw_s_m"),
            latent_heat_j_kg: number(gas, "latent_heat_j_kg"),
            top_rain_kg_m2_tile: 0.0,
            under_canopy_geometry: UnderCanopyGeometry {
                canopy_height_m: number(geometry, "z_hv_m"),
                canopy_roughness_m: number(geometry, "z0v_m"),
                reference_height_m: number(geometry, "z_ref_m"),
                leaf_area_index: number(geometry, "lai_m2_m2_tile"),
            },
            ground,
            occupancies,
            shortwave,
        },
        start,
    )
}

fn caps_from_expected(expected: &Value) -> CoveredWaterCaps {
    let components = expected
        .get("components")
        .unwrap_or(&expected["detail"]["ground"]);
    let q3 = if expected.get("components").is_some() {
        components["q3"].as_array().expect("single q3").clone()
    } else {
        expected["detail"]["occupancies"]
            .as_array()
            .expect("multirank occupancies")
            .iter()
            .flat_map(|occupancy| {
                occupancy["hydraulic_and_component"]["q3"]
                    .as_array()
                    .expect("occupancy q3")
                    .iter()
                    .map(move |row| (occupancy["occupancy_id"].clone(), row.clone()))
            })
            .map(|(identity, mut row)| {
                row["occupancy_id"] = identity;
                row
            })
            .collect()
    };
    let mut root = BTreeMap::new();
    for row in q3 {
        let occupancy_id = row["occupancy_id"]
            .as_str()
            .unwrap_or("canopy-rank-0")
            .to_owned();
        root.insert(
            (
                occupancy_id,
                row["layer_id"].as_str().expect("cap layer").to_owned(),
            ),
            SourceWaterCap {
                request_rate_kg_m2_tile_s: number(&row, "request_rate_kg_m2_tile_s"),
                authorization_rate_kg_m2_tile_s: number(&row, "authorization_rate_kg_m2_tile_s"),
            },
        );
    }
    let ground = if expected.get("components").is_some() {
        &expected["components"]["ground_vapor"]
    } else {
        &expected["detail"]["ground"]["ground_vapor"]
    };
    CoveredWaterCaps {
        root,
        ground: SourceWaterCap {
            request_rate_kg_m2_tile_s: number(ground, "request_rate_kg_m2_tile_s"),
            authorization_rate_kg_m2_tile_s: number(ground, "authorization_rate_kg_m2_tile_s"),
        },
    }
}

fn branch_name(branch: WaterBranch) -> &'static str {
    match branch {
        WaterBranch::ConstitutiveLaw => "constitutive_law",
        WaterBranch::AuthorizationActiveOrTie => "authorization_active_or_tie",
        WaterBranch::Condensation => "condensation",
    }
}

fn compare_water(actual: &crate::SourceWaterFlux, expected: &Value, identity: &str) {
    assert_eq!(actual.layer_id, expected["layer_id"]);
    assert_eq!(branch_name(actual.branch), expected["branch"]);
    for (actual, key) in [
        (actual.law_kg_m2_tile_s, "q_law_kg_m2_tile_s"),
        (actual.final_kg_m2_tile_s, "q_final_kg_m2_tile_s"),
        (
            actual.request_kg_m2_stand_ground,
            "request_kg_m2_stand_ground",
        ),
        (
            actual.finalized_use_kg_m2_stand_ground,
            "finalized_use_kg_m2_stand_ground",
        ),
    ] {
        close(actual, number(expected, key), &format!("{identity}.{key}"));
    }
    match (
        actual.authorization_kg_m2_stand_ground,
        expected["authorization_kg_m2_stand_ground"].as_f64(),
    ) {
        (Some(actual), Some(expected)) => {
            close(actual, expected, &format!("{identity}.authorization"));
        }
        (None, None) => {}
        values => panic!("{identity}.authorization optionality mismatch: {values:?}"),
    }
}

fn compare_occupancies(actual: &CoveredColumnCandidate, occupancies: &[Value]) {
    assert_eq!(actual.evaluation.occupancies.len(), occupancies.len());
    let mut source_index = 0;
    for (index, expected_occupancy) in occupancies.iter().enumerate() {
        let expected_component = &expected_occupancy["hydraulic_and_component"];
        let actual_occupancy = &actual.evaluation.occupancies[index];
        close_slice(
            &actual_occupancy.component_temperatures_k,
            &serde_json::json!([
                expected_component["component_temperatures_k"]["sun_leaf"],
                expected_component["component_temperatures_k"]["shade_leaf"],
                expected_component["component_temperatures_k"]["wet_surface"],
                expected_component["component_temperatures_k"]["dry_stem"]
            ]),
            &format!("occupancy[{index}].temperatures"),
        );
        close(
            actual_occupancy.wet_vapor_kg_m2_s,
            number(expected_component, "wet_vapor_kg_m2_tile_s"),
            &format!("occupancy[{index}].wet_vapor"),
        );
        for (class, expected_class, transpiration_key) in [(0, "sun", "sun"), (1, "shade", "shade")]
        {
            close(
                actual_occupancy.gross_assimilation_umol_co2_m2_leaf_s[class],
                number(&expected_component[expected_class], "ag"),
                &format!("occupancy[{index}].{expected_class}.gross_assimilation"),
            );
            close(
                actual_occupancy.net_assimilation_umol_co2_m2_leaf_s[class],
                number(&expected_component[expected_class], "an"),
                &format!("occupancy[{index}].{expected_class}.net_assimilation"),
            );
            close(
                actual_occupancy.dark_respiration_umol_co2_m2_leaf_s[class],
                number(&expected_component[expected_class], "rd"),
                &format!("occupancy[{index}].{expected_class}.respiration"),
            );
            close(
                actual_occupancy.signed_vapor_to_canopy_air_kg_m2_s[class],
                number(
                    &expected_component["component_transpiration_kg_m2_tile_s"],
                    transpiration_key,
                ),
                &format!("occupancy[{index}].{expected_class}.transpiration"),
            );
        }
        for row in expected_component["q3"].as_array().expect("expected q3") {
            compare_water(
                &actual.root_water[source_index],
                row,
                &format!("occupancy[{index}].q3"),
            );
            source_index += 1;
        }
    }
    assert_eq!(source_index, actual.root_water.len());
}

fn compare_longwave(actual: &CoveredColumnCandidate, expected: &Value, multirank: bool) {
    if multirank {
        let receipts = expected["occupancy_receipts"]
            .as_array()
            .expect("longwave receipts");
        assert_eq!(
            actual
                .evaluation
                .whole_column_longwave
                .transmissivities
                .len(),
            receipts.len()
        );
        for (index, receipt) in receipts.iter().enumerate() {
            close(
                actual.evaluation.whole_column_longwave.transmissivities[index],
                number(receipt, "tau"),
                &format!("longwave[{index}].tau"),
            );
            close(
                actual
                    .evaluation
                    .whole_column_longwave
                    .downward_boundaries_w_m2[index],
                number(receipt, "down_top_w_m2"),
                &format!("longwave[{index}].down_top"),
            );
            close(
                actual
                    .evaluation
                    .whole_column_longwave
                    .upward_boundaries_w_m2[index + 1],
                number(receipt, "up_bottom_w_m2"),
                &format!("longwave[{index}].up_bottom"),
            );
            close_slice(
                &actual.evaluation.whole_column_longwave.component_net_w_m2[index],
                &receipt["component_net_w_m2_tile"],
                &format!("longwave[{index}].components"),
            );
        }
        close(
            *actual
                .evaluation
                .whole_column_longwave
                .downward_boundaries_w_m2
                .last()
                .expect("terminal down"),
            number(expected, "terminal_down_w_m2_tile"),
            "longwave.terminal_down",
        );
    } else {
        close(
            actual.evaluation.whole_column_longwave.transmissivities[0],
            number(expected, "tau"),
            "longwave.tau",
        );
        close_slice(
            &actual.evaluation.whole_column_longwave.component_net_w_m2[0],
            &expected["component_net_w_m2_tile"],
            "longwave.components",
        );
    }
    close(
        actual.evaluation.whole_column_longwave.ground_net_w_m2,
        number(expected, "ground_net_w_m2_tile"),
        "longwave.ground_net",
    );
    close(
        actual.evaluation.whole_column_longwave.top_upward_w_m2,
        number(
            expected,
            if multirank {
                "top_up_w_m2_tile"
            } else {
                "top_up_w_m2"
            },
        ),
        "longwave.top_up",
    );
}

fn compare_ground_water(actual: &CoveredColumnCandidate, expected: &Value) {
    close(
        actual.ground_water.law_kg_m2_tile_s,
        number(expected, "q_law_kg_m2_tile_s"),
        "ground.q_law",
    );
    close(
        actual.ground_water.final_kg_m2_tile_s,
        number(expected, "q_final_kg_m2_tile_s"),
        "ground.q_final",
    );
    close(
        actual.ground_water.request_kg_m2_stand_ground,
        number(expected, "request_kg_m2_stand_ground"),
        "ground.request",
    );
    close(
        actual.ground_water.finalized_use_kg_m2_stand_ground,
        number(expected, "finalized_use_kg_m2_stand_ground"),
        "ground.finalized",
    );
    assert_eq!(branch_name(actual.ground_water.branch), expected["branch"]);
}

fn compare_candidate(actual: &CoveredColumnCandidate, expected: &Value, multirank: bool) {
    close_slice(&actual.solution, &expected["solution"], "accepted.solution");
    assert!(actual.iterations <= 50);
    assert!(actual.step_norms.hydraulic_mm <= 1.0e-7);
    assert!(actual.step_norms.beta <= 1.0e-10);
    assert!(actual.step_norms.temperature_k <= 1.0e-8);
    assert!(actual.step_norms.humidity_kg_kg <= 1.0e-12);
    let (residual_detail, ground, occupancies) = if multirank {
        (
            &expected["detail"],
            &expected["detail"]["ground"],
            expected["detail"]["occupancies"]
                .as_array()
                .unwrap()
                .clone(),
        )
    } else {
        (
            &expected["components"],
            &expected["components"],
            vec![serde_json::json!({
                "occupancy_id": "canopy-rank-0",
                "hydraulic_and_component": expected["components"].clone()
            })],
        )
    };
    assert_eq!(
        actual.evaluation.raw_residuals.len(),
        residual_detail["raw_residuals"].as_array().unwrap().len()
    );
    assert!(
        actual
            .evaluation
            .raw_residuals
            .iter()
            .all(|value| value.is_finite())
    );
    assert!(
        actual
            .evaluation
            .normalized_residuals
            .iter()
            .all(|value| value.is_finite() && value.abs() <= 1.0)
    );
    close_slice(
        &actual.evaluation.tolerances,
        &residual_detail["tolerances"],
        "components.tolerances",
    );
    close(
        actual.evaluation.canopy_air_temperature_k,
        number(ground, "canopy_air_temperature_k"),
        "canopy_air_temperature",
    );
    close(
        actual.evaluation.canopy_air_specific_humidity_kg_kg,
        number(ground, "canopy_air_specific_humidity_kg_kg"),
        "canopy_air_humidity",
    );
    close(
        actual.evaluation.ground_temperature_k,
        number(ground, "ground_temperature_k"),
        "ground_temperature",
    );
    close_slice(
        &actual.evaluation.soil_temperature_k,
        &ground["soil_temperature_k"],
        "soil_temperature",
    );
    close_slice(
        &actual.evaluation.ground_heat_cn_w_m2_tile,
        &ground["ground_heat_cn_w_m2_tile"],
        "ground_heat_cn",
    );
    close(
        actual.evaluation.ground_storage_w_m2_tile,
        number(ground, "surface_storage_w_m2_tile"),
        "surface_storage",
    );
    close(
        actual.evaluation.ground_sensible_to_canopy_air_w_m2,
        number(ground, "ground_sensible_w_m2_tile"),
        "ground_sensible",
    );
    close(
        actual.surface_enthalpy_j_m2_tile,
        expected["candidate"]["lse"]["surface_enthalpy_j_m2_tile"]
            .as_f64()
            .unwrap(),
        "candidate.surface_enthalpy",
    );

    compare_ground_water(actual, &ground["ground_vapor"]);

    let expected_longwave = if multirank {
        &expected["detail"]["whole_column_longwave"]
    } else {
        &ground["longwave"]
    };
    compare_longwave(actual, expected_longwave, multirank);

    compare_occupancies(actual, &occupancies);
}

fn accepted(outcome: CoveredColumnSolveOutcome) -> Box<CoveredColumnCandidate> {
    match outcome {
        CoveredColumnSolveOutcome::Accepted(value) => value,
        CoveredColumnSolveOutcome::Rejected(failure) => {
            panic!("covered solve rejected: {failure:?}")
        }
    }
}

#[test]
fn covered_single_rank_potential_fixed_cap_and_alternate_start_match_frozen_oracle() {
    let fixture = fixture();
    let family = &fixture["exact_model_reductions"]["covered_single_rank"];
    let (column, start) = column(&fixture, 1, &family["potential"]);
    let potential =
        accepted(solve_covered_column(&column, None, start.clone()).expect("single potential"));
    compare_candidate(&potential, &family["potential"], false);

    let caps = caps_from_expected(&family["fixed_cap_rebuilt_from_beginning"]);
    let fixed = accepted(
        solve_covered_column(&column, Some(&caps), start.clone()).expect("single fixed cap"),
    );
    compare_candidate(&fixed, &family["fixed_cap_rebuilt_from_beginning"], false);

    let mut alternate = start;
    for value in &mut alternate[..4] {
        *value += 250.0;
    }
    for value in &mut alternate[6..11] {
        *value += 0.5;
    }
    for value in &mut alternate[12..] {
        *value -= 0.5;
    }
    let alternate =
        accepted(solve_covered_column(&column, Some(&caps), alternate).expect("single alternate"));
    compare_candidate(&alternate, &family["alternate_warm_start_fixed_cap"], false);
}

#[test]
fn covered_multirank_potential_fixed_cap_and_alternate_start_match_frozen_oracle() {
    let fixture = fixture();
    let family = &fixture["exact_model_reductions"]["covered_multirank"];
    let (column, start) = column(&fixture, 2, &family["potential"]);
    let potential =
        accepted(solve_covered_column(&column, None, start.clone()).expect("multirank potential"));
    compare_candidate(&potential, &family["potential"], true);

    let caps = caps_from_expected(&family["fixed_cap_rebuilt_from_beginning"]);
    let fixed = accepted(
        solve_covered_column(&column, Some(&caps), start.clone()).expect("multirank fixed cap"),
    );
    compare_candidate(&fixed, &family["fixed_cap_rebuilt_from_beginning"], true);

    let mut alternate = start;
    for occupancy in 0..2 {
        let offset = 10 * occupancy;
        for value in &mut alternate[offset..offset + 4] {
            *value += 150.0;
        }
        for value in &mut alternate[offset + 6..offset + 10] {
            *value += 0.35;
        }
    }
    let alternate = accepted(
        solve_covered_column(&column, Some(&caps), alternate).expect("multirank alternate"),
    );
    compare_candidate(&alternate, &family["alternate_warm_start_fixed_cap"], true);
}

fn compare_exact_failure(actual: &NumericalFailure, record: &Value) {
    assert_eq!(
        actual.iterations,
        u32::try_from(record["iterations"].as_u64().unwrap()).unwrap()
    );
    assert_eq!(
        actual.backtracking_count,
        u32::try_from(record["backtracking_count"].as_u64().unwrap()).unwrap()
    );
    assert_eq!(
        actual.normalized_residuals,
        record["normalized_residuals"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>()
    );
    assert_eq!(actual.pivot_magnitude, record["pivot_magnitude"].as_f64());
    assert_eq!(actual.matrix_norm, record["matrix_norm"].as_f64());
    let expected_rows = record["ordered_residuals"].as_array().unwrap();
    assert_eq!(actual.ordered_residuals.len(), expected_rows.len());
    for (actual, expected) in actual.ordered_residuals.iter().zip(expected_rows) {
        assert_eq!(actual.identity, expected["identity"].as_str().unwrap());
        assert_eq!(
            actual.raw.to_bits(),
            expected["raw"].as_f64().unwrap().to_bits()
        );
        assert_eq!(
            actual.scale.to_bits(),
            expected["scale"].as_f64().unwrap().to_bits()
        );
        assert_eq!(
            actual.tolerance.to_bits(),
            expected["tolerance"].as_f64().unwrap().to_bits()
        );
        assert_eq!(
            actual.normalized.to_bits(),
            expected["normalized"].as_f64().unwrap().to_bits()
        );
        let unit = match actual.unit {
            crate::ResidualUnit::WattsPerSquareMeter => "w_m-2",
            crate::ResidualUnit::KilogramsPerSquareMeterSecond => "kg_m-2_s-1",
            crate::ResidualUnit::Pascal => "pa",
            crate::ResidualUnit::Millimeter => "mm",
            crate::ResidualUnit::KilogramPerKilogram => "kg_kg-1",
            crate::ResidualUnit::Dimensionless => "dimensionless",
        };
        assert_eq!(unit, expected["unit"].as_str().unwrap());
    }
}

fn compare_failure(actual: &NumericalFailure, expected: &Value) {
    let declared = expected
        .get("rust_expected_failure")
        .unwrap_or(&expected["failure"]);
    let declared_kind = match declared.as_str().expect("declared Rust failure kind") {
        "singular" => NumericalFailureKind::SingularPivot,
        "backtracking_limit" => NumericalFailureKind::BacktrackingLimit,
        "iteration_limit" => NumericalFailureKind::IterationLimit,
        other => panic!("unsupported declared numerical failure {other}"),
    };
    assert_eq!(actual.kind, declared_kind);
    assert_eq!(
        actual.occupancy_id.as_deref(),
        expected["diagnostics"]["occupancy_id"].as_str()
    );
    assert_eq!(
        actual.active_bounds,
        expected["diagnostics"]["active_bounds"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>()
    );
    let authority = rust_failure_fixture();
    let record = &authority["records"][declared.as_str().unwrap()];
    compare_exact_failure(actual, record);
    assert_eq!(
        actual.normalized_residuals.len(),
        expected["diagnostics"]["normalized_residuals"]
            .as_array()
            .unwrap()
            .len()
    );
    assert!(
        actual
            .normalized_residuals
            .iter()
            .all(|value| value.is_finite())
    );
    assert_eq!(
        actual.pivot_magnitude.is_some(),
        expected["diagnostics"]["pivot_magnitude"]
            .as_f64()
            .is_some()
    );
    assert_eq!(
        actual.matrix_norm.is_some(),
        expected["diagnostics"]["matrix_norm"].as_f64().is_some()
    );
    assert!(actual.pivot_magnitude.is_none_or(f64::is_finite));
    assert!(actual.matrix_norm.is_none_or(f64::is_finite));
    assert!(expected["candidate"].is_null());
    assert_eq!(expected["beginning_sha256"], expected["rollback_sha256"]);
}

#[test]
fn covered_natural_failures_match_frozen_diagnostics_and_publish_no_candidate() {
    let fixture = fixture();
    let family = &fixture["exact_model_reductions"]["real_numerical_failures"];
    let single = &fixture["exact_model_reductions"]["covered_single_rank"]["potential"];
    let (column, _) = column(&fixture, 1, single);
    let singular_start = vec![
        -1_686.290_413_383_744_2,
        -1_076.375_217_513_848_2,
        -549.022_563_591_395_2,
        -16_498.141_817_800_282,
        0.766_937_989_633_371,
        0.910_948_817_643_815_2,
        311.867_727_810_643_66,
        303.109_059_820_042_55,
        310.862_415_595_048_3,
        302.130_387_949_316_3,
        296.173_269_953_925_74,
        0.010_745_754_067_385_132,
        287.007_378_728_809,
        303.211_718_503_157_95,
        300.884_361_187_077_8,
    ];
    let CoveredColumnSolveOutcome::Rejected(singular) =
        solve_covered_column(&column, None, singular_start).expect("singular execution")
    else {
        panic!("singular vector unexpectedly accepted");
    };
    compare_failure(&singular, &family["singular"]);

    let mut backtracking_column = column.clone();
    backtracking_column.occupancies[0].k1_sun_max_s1 = 8.088_432_468_557_63e-06;
    backtracking_column.occupancies[0].k1_shade_max_s1 = 8.088_432_468_557_63e-06;
    backtracking_column.occupancies[0].k2_max = 1.098_977_274_111_314_2e-05;
    backtracking_column.occupancies[0].k3_max_m_s = 4.518_272_802_798_021e-05;
    let backtracking_start = vec![
        -10_505.902_951_449_543,
        -727.587_785_299_237_3,
        -3_826.381_288_830_032,
        -3_930.312_720_205_574_4,
        0.238_069_469_756_433_98,
        0.703_229_119_903_634_2,
        287.461_165_285_625_2,
        285.336_947_456_111_3,
        309.247_344_139_136_2,
        302.523_914_601_064_9,
        293.624_243_803_588_34,
        0.003_123_110_787_329_362,
        289.612_478_072_562_9,
        297.274_297_172_678_76,
        302.890_128_596_940_6,
    ];
    let CoveredColumnSolveOutcome::Rejected(backtracking) =
        solve_covered_column(&backtracking_column, None, backtracking_start)
            .expect("backtracking execution")
    else {
        panic!("backtracking vector unexpectedly accepted");
    };
    compare_failure(&backtracking, &family["backtracking_limit"]);
}

#[test]
fn covered_natural_iteration_limit_matches_declared_rust_outcome() {
    let fixture = fixture();
    let expected = &fixture["exact_model_reductions"]["real_numerical_failures"]["iteration_limit"];
    let single = &fixture["exact_model_reductions"]["covered_single_rank"]["potential"];
    let (mut candidate, _) = column(&fixture, 1, single);
    candidate.occupancies[0].k1_sun_max_s1 = 0.000_144_429_965_318_365_6;
    candidate.occupancies[0].k1_shade_max_s1 = 0.000_144_429_965_318_365_6;
    candidate.occupancies[0].k2_max = 1.538_924_383_128_636_2e-6;
    candidate.occupancies[0].k3_max_m_s = 1.969_219_977_554_62e-7;
    let start = vec![
        -5_706.990_986_525_235,
        -3_953.815_285_369_903,
        -8_895.703_204_372_228,
        -5_772.288_321_118_055,
        0.665_607_367_950_775_9,
        0.688_411_351_864_297_9,
        294.412_019_297_840_3,
        294.263_277_549_036_33,
        295.165_633_134_614_95,
        285.263_201_819_514_17,
        290.338_956_389_422_1,
        0.010_992_020_878_824_095,
        302.384_969_195_389_3,
        300.912_011_502_635_9,
        296.632_616_843_672_4,
    ];
    let CoveredColumnSolveOutcome::Rejected(failure) =
        solve_covered_column(&candidate, None, start).expect("iteration-limit execution")
    else {
        panic!("iteration-limit vector unexpectedly accepted");
    };
    compare_failure(&failure, expected);
    assert_eq!(
        failure.iterations,
        u32::try_from(expected["rust_expected_iterations"].as_u64().unwrap())
            .expect("bounded expected iteration count")
    );
}
