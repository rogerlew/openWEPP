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
const VECTOR_SHA256: &str = "9f171b0fd0e9a9a2e40d6ea8773d120b961c343e2aad6ad951ae705c8d683f3b";

fn fixture() -> Value {
    assert_eq!(format!("{:x}", Sha256::digest(VECTOR_BYTES)), VECTOR_SHA256);
    serde_json::from_slice(VECTOR_BYTES).expect("covered authority vectors parse")
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

fn column(fixture: &Value, rank_count: usize, expected: &Value) -> (CoveredColumnInputs, Vec<f64>) {
    let primitive =
        &fixture["mandatory_exact_scenario_vectors"]["covered_column"]["primitive_input"];
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
    let shortwave = if rank_count == 1 {
        synthetic_single_shortwave(
            &occupancies[0],
            terminal,
            number(ground_config, "ground_surface_albedo_vis"),
            number(ground_config, "ground_surface_albedo_nir"),
        )
    } else {
        let prepared = directional_shortwave(&expected["detail"]);
        for (index, source) in occupancies.iter_mut().enumerate() {
            let radiation = &prepared.occupancies[index];
            source.sun.leaf_area_m2_m2_tile = number(
                &expected["detail"]["whole_column_shortwave"]["by_band"]["VIS"]["direct"]["occupancies"]
                    [index]["results"],
                "leaf_sun_area",
            );
            source.shade.leaf_area_m2_m2_tile = number(
                &expected["detail"]["whole_column_shortwave"]["by_band"]["VIS"]["direct"]["occupancies"]
                    [index]["results"],
                "leaf_shade_area",
            );
            source.sun.absorbed_shortwave_w_m2_tile = radiation.sun_leaf_absorbed_w_m2_tile.total();
            source.shade.absorbed_shortwave_w_m2_tile =
                radiation.shade_leaf_absorbed_w_m2_tile.total();
            source.stem_absorbed_shortwave_w_m2_tile = radiation.stem_absorbed_w_m2_tile.total();
            source.sun.absorbed_par_w_m2_leaf = (radiation.sun_leaf_absorbed_w_m2_tile.direct_vis
                + radiation.sun_leaf_absorbed_w_m2_tile.diffuse_vis)
                / source.sun.leaf_area_m2_m2_tile;
            source.shade.absorbed_par_w_m2_leaf =
                (radiation.shade_leaf_absorbed_w_m2_tile.direct_vis
                    + radiation.shade_leaf_absorbed_w_m2_tile.diffuse_vis)
                    / source.shade.leaf_area_m2_m2_tile;
        }
        prepared
    };
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
    let start = occupancies
        .iter()
        .enumerate()
        .flat_map(|(index, _)| {
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
        .collect();
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
            close(actual, expected, &format!("{identity}.authorization"))
        }
        (None, None) => {}
        values => panic!("{identity}.authorization optionality mismatch: {values:?}"),
    }
}

fn compare_candidate(actual: &CoveredColumnCandidate, expected: &Value, multirank: bool) {
    close_slice(&actual.solution, &expected["solution"], "accepted.solution");
    assert_eq!(
        actual.iterations as u64,
        expected["iterations"].as_u64().unwrap()
    );
    assert_eq!(
        actual.backtracking_count as u64,
        expected["backtracking_count"].as_u64().unwrap()
    );
    let step = expected.get("step_norms").unwrap_or(&expected["step_norm"]);
    for (actual, key) in [
        (actual.step_norms.hydraulic_mm, "hydraulic_mm"),
        (actual.step_norms.beta, "beta"),
        (actual.step_norms.temperature_k, "temperature_k"),
        (actual.step_norms.humidity_kg_kg, "humidity_kg_kg"),
        (actual.step_norms.ci_pa, "ci_pa"),
    ] {
        close(
            actual,
            number(step, key),
            &format!("diagnostics.step_norms.{key}"),
        );
    }
    let (ground, occupancies) = if multirank {
        (
            &expected["detail"]["ground"],
            expected["detail"]["occupancies"]
                .as_array()
                .unwrap()
                .clone(),
        )
    } else {
        (
            &expected["components"],
            vec![serde_json::json!({
                "occupancy_id": "canopy-rank-0",
                "hydraulic_and_component": expected["components"].clone()
            })],
        )
    };
    close_slice(
        &actual.evaluation.raw_residuals,
        &ground["raw_residuals"],
        "components.raw_residuals",
    );
    close_slice(
        &actual.evaluation.normalized_residuals,
        &ground["normalized_residuals"],
        "components.normalized_residuals",
    );
    close_slice(
        &actual.evaluation.tolerances,
        &ground["tolerances"],
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

    let expected_ground = &ground["ground_vapor"];
    close(
        actual.ground_water.law_kg_m2_tile_s,
        number(expected_ground, "q_law_kg_m2_tile_s"),
        "ground.q_law",
    );
    close(
        actual.ground_water.final_kg_m2_tile_s,
        number(expected_ground, "q_final_kg_m2_tile_s"),
        "ground.q_final",
    );
    close(
        actual.ground_water.request_kg_m2_stand_ground,
        number(expected_ground, "request_kg_m2_stand_ground"),
        "ground.request",
    );
    close(
        actual.ground_water.finalized_use_kg_m2_stand_ground,
        number(expected_ground, "finalized_use_kg_m2_stand_ground"),
        "ground.finalized",
    );
    assert_eq!(
        branch_name(actual.ground_water.branch),
        expected_ground["branch"]
    );

    let expected_longwave = if multirank {
        &expected["detail"]["whole_column_longwave"]
    } else {
        &ground["longwave"]
    };
    if multirank {
        let receipts = expected_longwave["occupancy_receipts"]
            .as_array()
            .expect("longwave occupancy receipts");
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
            number(expected_longwave, "terminal_down_w_m2_tile"),
            "longwave.terminal_down",
        );
    } else {
        close(
            actual.evaluation.whole_column_longwave.transmissivities[0],
            number(expected_longwave, "tau"),
            "longwave.tau",
        );
        close_slice(
            &actual.evaluation.whole_column_longwave.component_net_w_m2[0],
            &expected_longwave["component_net_w_m2_tile"],
            "longwave.components",
        );
    }
    close(
        actual.evaluation.whole_column_longwave.ground_net_w_m2,
        number(expected_longwave, "ground_net_w_m2_tile"),
        "longwave.ground_net",
    );
    close(
        actual.evaluation.whole_column_longwave.top_upward_w_m2,
        number(
            expected_longwave,
            if multirank {
                "top_up_w_m2_tile"
            } else {
                "top_up_w_m2"
            },
        ),
        "longwave.top_up",
    );

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
        let q3 = expected_component["q3"].as_array().expect("expected q3");
        for row in q3 {
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

fn compare_failure(
    actual: NumericalFailure,
    expected: &Value,
    expected_kind: NumericalFailureKind,
) {
    assert_eq!(actual.kind, expected_kind);
    assert_eq!(
        actual.iterations as u64,
        expected["iterations"].as_u64().unwrap()
    );
    close_slice(
        &actual.normalized_residuals,
        &expected["diagnostics"]["normalized_residuals"],
        "failure.normalized_residuals",
    );
    assert_eq!(
        actual.backtracking_count as u64,
        expected["diagnostics"]["backtracking_count"]
            .as_u64()
            .unwrap()
    );
    match (
        actual.pivot_magnitude,
        expected["diagnostics"]["pivot_magnitude"].as_f64(),
    ) {
        (Some(actual), Some(expected)) => close(actual, expected, "failure.pivot"),
        (None, None) => {}
        values => panic!("failure pivot optionality mismatch: {values:?}"),
    }
    match (
        actual.matrix_norm,
        expected["diagnostics"]["matrix_norm"].as_f64(),
    ) {
        (Some(actual), Some(expected)) => close(actual, expected, "failure.matrix_norm"),
        (None, None) => {}
        values => panic!("failure matrix optionality mismatch: {values:?}"),
    }
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
        -5060.058341181588,
        -3223.3606138445093,
        -8678.098409994316,
        -2731.151728213228,
        0.7828034238440497,
        0.41921750290049625,
        294.4386499562707,
        291.8527267957731,
        293.0377484776449,
        292.56945726878206,
        299.21534306450434,
        0.002671465482357275,
        288.94930200326996,
        296.7197862527845,
        292.4442386140941,
    ];
    let CoveredColumnSolveOutcome::Rejected(singular) =
        solve_covered_column(&column, None, singular_start).expect("singular execution")
    else {
        panic!("singular vector unexpectedly accepted");
    };
    compare_failure(
        singular,
        &family["singular"],
        NumericalFailureKind::SingularPivot,
    );

    let backtracking_start = vec![
        -3823.238728569615,
        -8119.418303690043,
        -5920.92689913748,
        -3285.3959407455854,
        0.4205270600405538,
        0.7825045940760162,
        298.01573552618004,
        298.59100122181957,
        293.6906094785682,
        288.14926136113905,
        293.5534210427704,
        0.007584220145071355,
        300.9571667868542,
        295.58463623826225,
        292.8342196334298,
    ];
    let CoveredColumnSolveOutcome::Rejected(backtracking) =
        solve_covered_column(&column, None, backtracking_start).expect("backtracking execution")
    else {
        panic!("backtracking vector unexpectedly accepted");
    };
    compare_failure(
        backtracking,
        &family["backtracking_limit"],
        NumericalFailureKind::BacktrackingLimit,
    );

    // The iteration-limit fixture changes the three hydraulic conductances but
    // retains the same digest-bound physical input surface otherwise.
    let mut limited_column = column;
    limited_column.occupancies[0].k1_sun_max_s1 = 3.71808736481436e-05;
    limited_column.occupancies[0].k1_shade_max_s1 = 3.71808736481436e-05;
    limited_column.occupancies[0].k2_max = 3.952433838191729e-06;
    limited_column.occupancies[0].k3_max_m_s = 0.0002266759889262188;
    let limited_start = vec![
        -1898.4133523366827,
        -7111.481267003401,
        -8204.131337274273,
        -6094.272125330269,
        0.7284837512552559,
        0.66846451641645,
        302.0046249862505,
        289.265493165604,
        294.81734235838894,
        293.6317576401563,
        293.04248607317714,
        0.004348547002765208,
        288.5191447798437,
        287.1176596966161,
        291.94149081359876,
    ];
    let CoveredColumnSolveOutcome::Rejected(limited) =
        solve_covered_column(&limited_column, None, limited_start).expect("iteration execution")
    else {
        panic!("iteration-limit vector unexpectedly accepted");
    };
    compare_failure(
        limited,
        &family["iteration_limit"],
        NumericalFailureKind::IterationLimit,
    );
}
