#!/usr/bin/env python3
"""Independent V8 one-occupancy joint canopy--ground oracle core.

This module reuses only the immutable, checksum-bound V3/V5 Python authority
oracles.  It never imports Rust.  Its trial vector couples the V3 hydraulic and
gas equations to current-trial sun/shade/wet/stem temperatures, one shared
tile canopy-air temperature/humidity, the ground surface, and an ordered soil
thermal column.  Root and ground caps are source-specific rate constraints;
equality selects the cap-active branch and its zero generalized derivative.

The bounded one-occupancy core is intentionally small enough for independent
inspection.  The package's main calculator owns multi-tile routing,
arbitration, ingress partition, and independent owner closure.
"""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import math
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
V3_PATH = (
    HERE.parents[1]
    / "20260812-c3-woody-potential-pass-authority-001"
    / "artifacts/reference_calculator.py"
)
V5_PATH = (
    HERE.parents[1]
    / "20260812-c3-woody-potential-pass-authority-001"
    / "artifacts/reference_calculator_v5.py"
)
V3_SHA256 = "7b137c1aa9ed0912caf4d14c779eca1819014b4217156d36f98619f06daabd1a"
V5_SHA256 = "4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775"
V8_MODEL_DEFINITION_SHA256 = "622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b"

SIGMA = 5.670374419e-8
CW = 4218.0
TREF = 273.15
WATER_ATOL = 1.0e-12
WATER_RTOL = 1.0e-9
ENERGY_ATOL = 1.0e-6
ENERGY_RTOL = 1.0e-10
EPSILON = 2.220446049250313e-16


def _load(name: str, path: Path, expected: str) -> Any:
    actual = hashlib.sha256(path.read_bytes()).hexdigest()
    if actual != expected:
        raise RuntimeError(f"{name} authority checksum mismatch: {actual}")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {name} authority")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


V3 = _load("openwepp_v3_joint_source", V3_PATH, V3_SHA256)
V5 = _load("openwepp_v5_joint_source", V5_PATH, V5_SHA256)


def canonical(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False)


def digest(value: Any) -> str:
    return hashlib.sha256(canonical(value).encode()).hexdigest()


def qsat(temperature_k: float, pressure_pa: float) -> float:
    return V3.saturation_specific_humidity(temperature_k, pressure_pa)


def liquid_enthalpy(temperature_k: float) -> float:
    return CW * (temperature_k - TREF)


def vaporization_enthalpy(temperature_k: float) -> float:
    return 2_501_000.0 - 2369.0 * (temperature_k - TREF)


def build_joint_case() -> dict[str, Any]:
    """Return a deterministic one-occupancy covered-tile authority case."""
    family = V3.hydraulic_vectors()
    case = copy.deepcopy(family["operands"])
    case["canopy_longwave"] = {
        "atmospheric_down_w_m2": case["gas_energy"].pop("longwave_down_w_m2"),
        "clumping_index": 0.82,
        "extinction_m2_plant_m2_ground": 0.8,
    }
    case["gas_energy"].pop("longwave_up_w_m2")
    ground_config = {
        "tile_id": "tile-a",
        "surface_class": "forest_litter",
        "surface_source_id": "surface:tile-a",
        "ground_surface_albedo_vis": 0.12,
        "ground_surface_albedo_nir": 0.24,
        "surface_storage_branch": "finite_capacity",
        "under_canopy_geometry": {"z_hv_m": 12.5, "z0v_m": 1.25,
            "z_ref_m": 24.0, "lai_m2_m2_tile": case["parameters"]["lai"]},
        "surface_dry_heat_capacity_j_m2_k": 3235.68,
        "surface_depth_m": 0.04,
        "surface_conductivity_w_m_k": 0.103,
        "litter_capacity_kg_m2_tile": 6.0,
        "soil_nodes": [
            {"layer_id": "thermal-1", "depth_m": 0.08,
             "conductivity_w_m_k": 1.10, "heat_capacity_j_m2_k": 120_000.0},
            {"layer_id": "thermal-2", "depth_m": 0.18,
             "conductivity_w_m_k": 1.35, "heat_capacity_j_m2_k": 180_000.0},
        ],
    }
    surface_water = 4.0
    surface_start = 295.0
    capacity = ground_config["surface_dry_heat_capacity_j_m2_k"] + surface_water * CW
    ground_state = {
        "surface_liquid_kg_m2_tile": surface_water,
        "surface_enthalpy_j_m2_tile": capacity * (surface_start - TREF),
        "surface_temperature_warm_start_k": surface_start,
        "soil_temperature_k": [291.5, 289.8],
    }
    start = [
        -5900.0, -5450.0, -4300.0, -2850.0, 0.68, 0.66,
        case["classes"]["sun"]["temperature_start_k"],
        case["classes"]["shade"]["temperature_start_k"],
        case["gas_energy"]["wet_temperature_start_k"],
        case["gas_energy"]["stem_temperature_start_k"],
        case["gas_energy"]["canopy_air_temperature_start_k"],
        case["gas_energy"]["qcan_start_kg_kg"], surface_start,
        *ground_state["soil_temperature_k"],
    ]
    seed_column = {"occupancies": [{"occupancy_id": "canopy-rank-0",
        "rank": 0, "case": case}], "ground_config": ground_config,
        "ground_state": ground_state,
        "shortwave": {"zenith_cosine": 0.67,
            "incident_by_band_w_m2_tile": {
                "VIS": {"direct": 410.0, "diffuse": 83.0},
                "NIR": {"direct": 355.0, "diffuse": 101.0}},
            "optics_by_occupancy": {"canopy-rank-0": {
                "VIS": {"leaf": {"rho": 0.09, "tau": 0.06},
                        "stem": {"rho": 0.18, "tau": 0.03}},
                "NIR": {"leaf": {"rho": 0.41, "tau": 0.31},
                        "stem": {"rho": 0.29, "tau": 0.12}}}}}}
    prepared = _prepare_covered_shortwave(seed_column)
    return {"case": prepared["occupancies"][0]["case"],
            "ground_config": prepared["ground_config"],
            "ground_state": ground_state, "start": start,
            "shortwave": prepared["_executed_shortwave"]}


def _default_start(case: dict[str, Any], ground_state: dict[str, Any]) -> list[float]:
    warm = case.get("joint_warm_start", {})
    return [
        warm.get("sun_leaf_potential_mm", -5900.0),
        warm.get("shade_leaf_potential_mm", -5450.0),
        warm.get("stem_potential_mm", -4300.0),
        warm.get("root_potential_mm", -2850.0),
        warm.get("beta_sun", 0.68), warm.get("beta_shade", 0.66),
        case["classes"]["sun"]["temperature_start_k"],
        case["classes"]["shade"]["temperature_start_k"],
        case["gas_energy"]["wet_temperature_start_k"],
        case["gas_energy"]["stem_temperature_start_k"],
        case["gas_energy"]["canopy_air_temperature_start_k"],
        case["gas_energy"]["qcan_start_kg_kg"],
        ground_state["surface_temperature_warm_start_k"],
        *ground_state["soil_temperature_k"],
    ]


def _class_state(case: dict[str, Any], name: str, temperature: float,
                 qcan: float, beta: float) -> dict[str, Any]:
    forcing = case["gas_energy"]
    source = case["classes"][name]
    p = case["biochemical_parameters"]
    pressure, ca = forcing["pressure_pa"], forcing["ca_pa"]
    rb = 1.0 / forcing["gb_leaf_m_s"]
    vcmax_factor = V3.peaked_response(
        temperature, p["ha_vcmax_j_mol"], p["hd_vcmax_j_mol"],
        p["entropy_vcmax_j_mol_k"])
    jmax_factor = V3.peaked_response(
        temperature, p["ha_jmax_j_mol"], p["hd_jmax_j_mol"],
        p["entropy_jmax_j_mol_k"])
    vcmax = source["vcmax25"] * vcmax_factor
    jmax = source["jmax25"] * jmax_factor
    kc = p["kc25_pa"] * V3.arrhenius_response(temperature, p["ha_kc_j_mol"])
    ko = p["ko25_pa"] * V3.arrhenius_response(temperature, p["ha_ko_j_mol"])
    gamma = p["gamma25_pa"] * V3.arrhenius_response(temperature, p["ha_gamma_j_mol"])
    tp = p["tp_vcmax_ratio"] * source["vcmax25"] * vcmax_factor
    rd = V3.peaked_rd(source["rd25"], temperature)
    q_surface = qsat(temperature, pressure)
    es_leaf = q_surface * pressure / (0.622 + 0.378 * q_surface)
    e_can = qcan * pressure / (0.622 + 0.378 * qcan)
    vpd = (es_leaf - e_can) / 1000.0
    if vpd <= 0.0:
        raise ValueError("nonpositive solved surface VPD")

    def ci_residual(ci: float) -> tuple[float, dict[str, Any]]:
        ipsii = (0.5 * p["electron_quantum_yield"] * p["par_photon_umol_per_j"]
                 * source["absorbed_par_w_m2_leaf"])
        electron = (V3.smaller_quadratic_root(
            p["electron_curvature"], -(ipsii + jmax), ipsii * jmax)
            if ipsii > 0.0 else 0.0)
        ac = vcmax * (ci - gamma) / (ci + kc * (1.0 + p["oxygen_partial_pressure_pa"] / ko))
        aj = electron * (ci - gamma) / (4.0 * ci + 8.0 * gamma)
        ap = 3.0 * tp
        ai = V3.smaller_quadratic_root(p["ac_aj_curvature"], -(ac + aj), ac * aj)
        ag = V3.smaller_quadratic_root(p["ag_ap_curvature"], -(ai + ap), ai * ap)
        an = ag - rd
        cs = ca - 1.4 * rb * V3.R_GAS * temperature * an * 1.0e-6
        if cs <= 0.0:
            raise ValueError("nonpositive surface carbon dioxide")
        gs_potential = (forcing["g0_umol_m2_s"] if an <= 0.0 else
            forcing["g0_umol_m2_s"] + 1.6 *
            (1.0 + forcing["medlyn_g1_kpa_sqrt"] / math.sqrt(vpd)) * an / (cs / pressure))
        gs = forcing["g0_umol_m2_s"] + beta * (gs_potential - forcing["g0_umol_m2_s"])
        gs_ms = gs * 1.0e-6 * V3.R_GAS * temperature / pressure
        if gs_ms <= 0.0:
            raise ValueError("nonpositive stomatal conductance")
        rs = 1.0 / gs_ms
        predicted = ca - (1.4 * rb + 1.6 * rs) * V3.R_GAS * temperature * an * 1.0e-6
        return ci - predicted, {"ac": ac, "aj": aj, "ap": ap, "ai": ai,
            "ag": ag, "an": an, "rd": rd, "cs_pa": cs,
            "gs_potential_umol_m2_s": gs_potential, "gs_umol_m2_s": gs,
            "gs_m_s": gs_ms, "rs_s_m": rs, "vpd_kpa": vpd,
            "surface_q_kg_kg": q_surface}

    ci, detail, iterations, bracket = V3.brent_dekker(ci_residual, gamma, ca)
    return {**detail, "ci_pa": ci, "ci_iterations": iterations,
            "ci_bracket_pa": bracket, "temperature_k": temperature}


def _longwave(case: dict[str, Any], temperatures: list[float],
              ground_temperature: float, areas: list[float]) -> dict[str, Any]:
    lai = case["parameters"]["lai"]
    sai = case["parameters"]["sai"]
    config = case["canopy_longwave"]
    tau = math.exp(-config["extinction_m2_plant_m2_ground"]
                   * config["clumping_index"] * (lai + sai))
    area_sum = math.fsum(areas)
    if area_sum == 0.0:
        return {"tau": 1.0, "component_net_w_m2_tile": [0.0] * 4,
                "ground_net_w_m2_tile": config["atmospheric_down_w_m2"]
                - SIGMA * ground_temperature**4, "top_up_w_m2": SIGMA * ground_temperature**4}
    weights = [area / area_sum for area in areas]
    emission = math.fsum(weight * SIGMA * temperature**4
                         for weight, temperature in zip(weights, temperatures, strict=True))
    down_top = config["atmospheric_down_w_m2"]
    down_ground = tau * down_top + (1.0 - tau) * emission
    up_ground = SIGMA * ground_temperature**4
    up_top = tau * up_ground + (1.0 - tau) * emission
    components = [weight * (1.0 - tau) * (down_top + up_ground)
                  - 2.0 * weight * (1.0 - tau) * SIGMA * temperature**4
                  for weight, temperature in zip(weights, temperatures, strict=True)]
    return {"tau": tau, "component_net_w_m2_tile": components,
            "ground_net_w_m2_tile": down_ground - up_ground,
            "down_ground_w_m2": down_ground, "top_up_w_m2": up_top,
            "closure_w_m2": down_top - up_top - (down_ground - up_ground)
            - math.fsum(components)}


def _cap_value(caps: dict[str, Any] | None, family: str, identity: str,
               law: float) -> tuple[float, float, str]:
    if caps is None:
        return law, max(0.0, law), "constitutive_law"
    family_caps = caps.get(family, {})
    if identity not in family_caps:
        raise ValueError(f"authorization_identity_schema:{family}:{identity}")
    supplied = family_caps[identity]
    if isinstance(supplied, dict):
        cap = supplied["cap_rate_kg_m2_tile_s"]
        request = supplied.get("request_rate_kg_m2_tile_s", max(0.0, law))
    else:
        cap, request = supplied, max(0.0, law)
    if not math.isfinite(cap) or cap < 0.0 or not math.isfinite(request) or request < cap:
        raise ValueError(f"authorization_domain:{family}:{identity}")
    if law >= 0.0 and cap <= law:
        return cap, request, "authorization_active_or_tie"
    return law, request, "constitutive_law"


def _supplied_cap_rate(caps: dict[str, Any] | None, family: str,
                       identity: str) -> float | None:
    if caps is None:
        return None
    supplied = caps[family][identity]
    return (supplied["cap_rate_kg_m2_tile_s"]
            if isinstance(supplied, dict) else supplied)


def _validate_cap_identity(case: dict[str, Any], ground_config: dict[str, Any],
                           caps: dict[str, Any] | None) -> None:
    if caps is None:
        return
    if set(caps) != {"root", "ground"}:
        raise ValueError("authorization_family_identity_schema")
    expected_root = {layer["layer_id"] for layer in case["layers"]}
    if set(caps["root"]) != expected_root:
        raise ValueError("authorization_root_identity_schema")
    if set(caps["ground"]) != {ground_config["surface_source_id"]}:
        raise ValueError("authorization_ground_identity_schema")


def _conductance(depth_a: float, conductivity_a: float,
                 depth_b: float, conductivity_b: float) -> float:
    return 2.0 / (depth_a / conductivity_a + depth_b / conductivity_b)


def _absorbed_terminal_shortwave(configuration: dict[str, Any],
                                 field: str) -> dict[str, float]:
    incident = configuration[field]
    vis = configuration["ground_surface_albedo_vis"]
    nir = configuration["ground_surface_albedo_nir"]
    if any(not math.isfinite(value) or not 0.0 <= value <= 1.0 for value in (vis, nir)):
        raise ValueError("ground_shortwave_albedo_domain")
    return {identity: amount * (1.0 - (vis if identity.endswith("vis") else nir))
            for identity, amount in incident.items()}


def under_canopy_neutral_resistance(ground_config: dict[str, Any],
                                    reference_wind_m_s: float) -> dict[str, float]:
    """Exact ISBA-MEB equations 54--63 with the frozen V1 constants."""
    geometry = ground_config["under_canopy_geometry"]
    z_hv, z0v = geometry["z_hv_m"], geometry["z0v_m"]
    z_ref, lai = geometry["z_ref_m"], geometry["lai_m2_m2_tile"]
    kappa, phi_v, z0g = 0.4, 2.0, 0.007
    chi_l, u_l, l_w, nu = 0.12, 1.0, 0.02, 1.5e-5
    values = [z_hv, z0v, z_ref, lai, reference_wind_m_s]
    if any(not math.isfinite(value) or value <= 0.0 for value in values):
        raise ValueError("under_canopy_geometry_domain")
    reynolds = u_l * l_w / nu
    drag = 1.328 * (2.0 / math.sqrt(reynolds)) + 0.45 * (
        (1.0 - chi_l) / math.pi) ** 1.6
    displacement = 1.1 * z_hv * math.log(1.0 + (drag * lai) ** 0.25)
    if not (z_hv > displacement + z0v > z0g
            and z_ref - displacement >= z_hv - displacement > 0.0):
        raise ValueError("under_canopy_geometry_domain")
    u_star = kappa * reference_wind_m_s / math.log((z_hv - displacement) / z0v)
    eddy = kappa * u_star * (z_hv - displacement)
    resistance = z_hv / (phi_v * eddy) * (
        math.exp(phi_v * (1.0 - z0g / z_hv))
        - math.exp(phi_v * (1.0 - (displacement + z0v) / z_hv)))
    if any(not math.isfinite(value) or value <= 0.0
           for value in (drag, u_star, eddy, resistance)):
        raise ValueError("under_canopy_resistance_domain")
    return {"reynolds_number": reynolds, "drag_coefficient": drag,
        "displacement_m": displacement, "u_star_m_s": u_star,
        "eddy_diffusivity_m2_s": eddy, "r_gn_s_m": resistance,
        "heat_resistance_s_m": resistance, "vapor_resistance_s_m": resistance,
        "phi_v": phi_v, "z0g_m": z0g, "chi_l": chi_l,
        "u_l_m_s": u_l, "l_w_m": l_w, "nu_m2_s": nu}


def _raw_residual(case: dict[str, Any], ground_config: dict[str, Any],
                  ground_state: dict[str, Any], trial: list[float],
                  caps: dict[str, Any] | None,
                  frozen_branches: dict[str, str] | None = None) -> tuple[list[float], dict[str, Any]]:
    _validate_cap_identity(case, ground_config, caps)
    node_count = len(ground_config["soil_nodes"])
    if len(trial) != 13 + node_count:
        raise ValueError("joint_trial_identity_schema")
    psi_sun, psi_shade, psi_stem, psi_root, beta_sun, beta_shade = trial[:6]
    tsun, tshade, twet, tstem, tcan, qcan, tsurface = trial[6:13]
    soil_t = trial[13:]
    if not all(math.isfinite(value) for value in trial):
        raise ValueError("joint_nonfinite_trial")
    temperature_values = [*trial[6:11], *trial[12:]]
    if not all(200.0 <= value <= 350.0 for value in temperature_values) or not 0.0 <= qcan <= 0.1:
        raise ValueError("joint_temperature_humidity_bounds")
    if not 0.0 <= beta_sun <= 1.0 or not 0.0 <= beta_shade <= 1.0:
        raise ValueError("joint_beta_bounds")

    forcing, classes, p = case["gas_energy"], case["classes"], case["parameters"]
    ground_aerodynamics = under_canopy_neutral_resistance(
        ground_config, forcing["reference_wind_operands"]["u_ref_m_s"])
    pressure = forcing["pressure_pa"]
    rho = pressure / (forcing["rdry_j_kg_k"] * tcan)
    cp = forcing["cp_air_j_kg_k"]
    wet_fraction = forcing["wet_fraction"]
    dry_sun = classes["sun"]["leaf_area"] * (1.0 - wet_fraction)
    dry_shade = classes["shade"]["leaf_area"] * (1.0 - wet_fraction)
    wet_area = wet_fraction * (classes["sun"]["leaf_area"] + classes["shade"]["leaf_area"]
                               + forcing["stem_area"])
    dry_stem = (1.0 - wet_fraction) * forcing["stem_area"]
    areas = [dry_sun, dry_shade, wet_area, dry_stem]
    sun = _class_state(case, "sun", tsun, qcan, beta_sun)
    shade = _class_state(case, "shade", tshade, qcan, beta_shade)
    rb = 1.0 / forcing["gb_leaf_m_s"]
    transpiration = {
        "sun": rho * (sun["surface_q_kg_kg"] - qcan) / (rb + sun["rs_s_m"]) * dry_sun,
        "shade": rho * (shade["surface_q_kg_kg"] - qcan) / (rb + shade["rs_s_m"]) * dry_shade,
    }
    qsat_wet = qsat(twet, pressure)
    wet_potential = rho * forcing["gb_wet_m_s"] * (qsat_wet - qcan) * wet_area
    wet_cap = forcing["canopy_liquid_kg_m2_tile"] / case["dt_s"]
    wet_branch = ("store_cap_active_or_tie"
        if wet_potential >= 0.0 and wet_cap <= wet_potential else "constitutive_law")
    if frozen_branches and "wet_surface" in frozen_branches:
        wet_branch = frozen_branches["wet_surface"]
    wet_actual = wet_cap if wet_branch == "store_cap_active_or_tie" else wet_potential
    longwave = _longwave(case, [tsun, tshade, twet, tstem], tsurface, areas)
    latent = forcing["latent_heat_j_kg"]
    energy_residuals: list[float] = []
    energy_scales: list[float] = []
    for index, (name, temp, area, state) in enumerate((
            ("sun", tsun, dry_sun, sun), ("shade", tshade, dry_shade, shade))):
        flux = transpiration[name]
        sw = classes[name]["absorbed_shortwave_w_m2_tile"] * (1.0 - wet_fraction)
        sensible = rho * cp * forcing["gb_leaf_m_s"] * area * (temp - tcan)
        lw = longwave["component_net_w_m2_tile"][index]
        energy_residuals.append(sw + lw - sensible - latent * flux)
        energy_scales.append(max(1.0, abs(sw) + abs(lw) + abs(sensible) + abs(latent * flux)))
    wet_sw = wet_fraction * (classes["sun"]["absorbed_shortwave_w_m2_tile"]
        + classes["shade"]["absorbed_shortwave_w_m2_tile"]
        + forcing["stem_absorbed_shortwave_w_m2_tile"])
    wet_h = rho * cp * forcing["gb_wet_m_s"] * wet_area * (twet - tcan)
    wet_lw = longwave["component_net_w_m2_tile"][2]
    energy_residuals.append(wet_sw + wet_lw - wet_h - latent * wet_actual)
    energy_scales.append(max(1.0, abs(wet_sw) + abs(wet_lw) + abs(wet_h) + abs(latent * wet_actual)))
    stem_sw = (1.0 - wet_fraction) * forcing["stem_absorbed_shortwave_w_m2_tile"]
    stem_h = rho * cp * forcing["gb_stem_m_s"] * dry_stem * (tstem - tcan)
    stem_lw = longwave["component_net_w_m2_tile"][3]
    energy_residuals.append(stem_sw + stem_lw - stem_h)
    energy_scales.append(max(1.0, abs(stem_sw) + abs(stem_lw) + abs(stem_h)))

    leaf_factor = p["k1_max"] / p["stem_to_leaf_path_m"]
    q1_sun = leaf_factor * p["sun_leaf_area"] * V3.vulnerability(
        psi_stem, p["p50_xylem"], p["ck"]) * (psi_stem - psi_sun)
    q1_shade = leaf_factor * p["shade_leaf_area"] * V3.vulnerability(
        psi_stem, p["p50_xylem"], p["ck"]) * (psi_stem - psi_shade)
    q2 = ((p["k2_max"] / p["height_m"]) * V3.vulnerability(
        psi_root, p["p50_xylem"], p["ck"]) * p["sai"]
        * (psi_root - psi_stem - 1000.0 * p["height_m"]))
    q3: list[dict[str, Any]] = []
    for layer in case["layers"]:
        if not layer["accessible"] or layer["frozen"] or layer["root_fraction"] == 0.0:
            qlaw = kr = ks = kseries = rai = soil_v = 0.0
        else:
            soil_v = V3.vulnerability(layer["soil_potential_mm"], p["p50_root"], p["ck"])
            kr = p["k3_max_m_s"] / layer["z3_m"] * soil_v
            ks = layer["ksoil_m2_s"] / layer["dxroot_m"]
            kseries = kr * ks / (kr + ks)
            rai = (p["lai"] + p["sai"]) * layer["root_fraction"] * p["root_to_leaf_area"]
            qlaw = kseries * rai * (layer["soil_potential_mm"] - psi_root
                                    + layer["gravity_head_mm"])
            if qlaw < 0.0:
                raise V5.RedistributionError(layer["layer_id"], qlaw, q3)
        qfinal, request, branch = _cap_value(caps, "root", layer["layer_id"], qlaw)
        if frozen_branches and frozen_branches.get(f"root:{layer['layer_id']}"):
            branch = frozen_branches[f"root:{layer['layer_id']}"]
            if branch == "authorization_active_or_tie":
                fixed_cap = _supplied_cap_rate(caps, "root", layer["layer_id"])
                if fixed_cap is None:
                    raise ValueError("frozen_root_cap_without_authorization")
                qfinal = fixed_cap
            else:
                qfinal = qlaw
        q3.append({"layer_id": layer["layer_id"], "q_law_kg_m2_tile_s": qlaw,
            "q_final_kg_m2_tile_s": qfinal, "request_rate_kg_m2_tile_s": request,
            "branch": branch, "kr_m_s": kr, "ks_m_s": ks,
            "k3_series_m_s": kseries, "rai_m2_m2": rai,
            "soil_vulnerability": soil_v})
    hydraulic = [transpiration["sun"] - q1_sun, transpiration["shade"] - q1_shade,
        transpiration["sun"] - case["emax"]["sun"] * V3.vulnerability(psi_sun, p["p50_leaf"], p["ck"]),
        transpiration["shade"] - case["emax"]["shade"] * V3.vulnerability(psi_shade, p["p50_leaf"], p["ck"]),
        q1_sun + q1_shade - q2, q2 - math.fsum(row["q_final_kg_m2_tile_s"] for row in q3)]
    water_scale = max(WATER_ATOL, case["emax"]["sun"], case["emax"]["shade"],
        abs(q1_sun), abs(q1_shade), abs(q2),
        *(abs(row["q_final_kg_m2_tile_s"]) for row in q3))

    surface_water = ground_state["surface_liquid_kg_m2_tile"]
    litter_capacity = ground_config["litter_capacity_kg_m2_tile"]
    humidity = 0.5 * (1.0 - math.cos(math.pi * surface_water / litter_capacity))
    qground = humidity * qsat(tsurface, pressure) + (1.0 - humidity) * qcan
    ground_law = rho * (qground - qcan) / ground_aerodynamics["vapor_resistance_s_m"]
    if ground_law >= 0.0:
        ground_vapor, ground_request, ground_branch = _cap_value(
            caps, "ground", ground_config["surface_source_id"], ground_law)
    else:
        ground_vapor, ground_request, ground_branch = ground_law, 0.0, "condensation"
    if frozen_branches and "ground" in frozen_branches:
        ground_branch = frozen_branches["ground"]
        if ground_branch == "authorization_active_or_tie":
            fixed_cap = _supplied_cap_rate(
                caps, "ground", ground_config["surface_source_id"])
            if fixed_cap is None:
                raise ValueError("frozen_ground_cap_without_authorization")
            ground_vapor = fixed_cap
        elif ground_branch == "condensation":
            ground_vapor = ground_law
        else:
            ground_vapor = ground_law
    ground_h = rho * cp * (tsurface - tcan) / ground_aerodynamics["heat_resistance_s_m"]
    canopy_h = (rho * cp * forcing["gb_leaf_m_s"] * dry_sun * (tsun - tcan)
        + rho * cp * forcing["gb_leaf_m_s"] * dry_shade * (tshade - tcan)
        + wet_h + stem_h)
    canopy_v = transpiration["sun"] + transpiration["shade"] + wet_actual
    reference_h = rho * cp * (tcan - forcing["air_temperature_k"]) / forcing["rah_s_m"]
    reference_v = rho * (qcan - forcing["air_specific_humidity_kg_kg"]) / forcing["raw_s_m"]
    canopy_heat_residual = canopy_h + ground_h - reference_h
    canopy_vapor_residual = canopy_v + ground_vapor - reference_v
    canopy_heat_scale = max(1.0, abs(canopy_h) + abs(ground_h) + abs(reference_h))
    canopy_vapor_scale = max(WATER_ATOL, abs(canopy_v), abs(ground_vapor), abs(reference_v))

    surface_pre_water = surface_water - max(ground_vapor, 0.0) * case["dt_s"] \
        + max(-ground_vapor, 0.0) * case["dt_s"]
    if surface_pre_water < 0.0:
        raise ValueError("surface_water_negative_after_finalized_vapor")
    dry_capacity = ground_config["surface_dry_heat_capacity_j_m2_k"]
    surface_capacity = dry_capacity + surface_pre_water * CW
    storage_branch = ground_config.get("surface_storage_branch", "finite_capacity")
    if storage_branch == "finite_capacity":
        if surface_capacity <= 0.0:
            raise ValueError("finite_surface_capacity_domain")
        surface_storage = (surface_capacity * (tsurface - TREF)
                           - ground_state["surface_enthalpy_j_m2_tile"]) / case["dt_s"]
    elif storage_branch == "equilibrium_zero":
        if not (dry_capacity == 0.0 and surface_pre_water == 0.0
                and ground_state["surface_enthalpy_j_m2_tile"] == 0.0):
            raise ValueError("equilibrium_zero_surface_state")
        surface_storage = 0.0
    else:
        raise ValueError("surface_storage_branch")
    nodes = ground_config["soil_nodes"]
    g_end = [_conductance(ground_config["surface_depth_m"],
        ground_config["surface_conductivity_w_m_k"], nodes[0]["depth_m"],
        nodes[0]["conductivity_w_m_k"]) * (tsurface - soil_t[0])]
    for index in range(len(nodes) - 1):
        g_end.append(_conductance(nodes[index]["depth_m"], nodes[index]["conductivity_w_m_k"],
            nodes[index + 1]["depth_m"], nodes[index + 1]["conductivity_w_m_k"])
            * (soil_t[index] - soil_t[index + 1]))
    beginning_capacity = dry_capacity + surface_water * CW
    surface_begin = (TREF + ground_state["surface_enthalpy_j_m2_tile"]
        / beginning_capacity if storage_branch == "finite_capacity"
        else tsurface)
    begin_soil = ground_state["soil_temperature_k"]
    g_begin = [_conductance(ground_config["surface_depth_m"],
        ground_config["surface_conductivity_w_m_k"], nodes[0]["depth_m"],
        nodes[0]["conductivity_w_m_k"]) * (surface_begin - begin_soil[0])]
    for index in range(len(nodes) - 1):
        g_begin.append(_conductance(nodes[index]["depth_m"], nodes[index]["conductivity_w_m_k"],
            nodes[index + 1]["depth_m"], nodes[index + 1]["conductivity_w_m_k"])
            * (begin_soil[index] - begin_soil[index + 1]))
    g_bar = [0.5 * (old + new) for old, new in zip(g_begin, g_end, strict=True)]
    qv = ground_vapor * (liquid_enthalpy(tsurface) + vaporization_enthalpy(tsurface))
    ground_shortwave_absorbed = _absorbed_terminal_shortwave(
        ground_config, "ground_terminal_shortwave_by_band_direction_w_m2_tile")
    ground_shortwave = math.fsum(ground_shortwave_absorbed.values())
    surface_terms = [ground_shortwave,
        longwave["ground_net_w_m2_tile"], -ground_h, -qv, -g_bar[0], -surface_storage]
    surface_residual = math.fsum(surface_terms)
    surface_scale = max(1.0, math.fsum(abs(value) for value in surface_terms))
    soil_residuals, soil_scales = [], []
    for index, node in enumerate(nodes):
        incoming = g_bar[index]
        outgoing = g_bar[index + 1] if index + 1 < len(g_bar) else 0.0
        storage = node["heat_capacity_j_m2_k"] * (soil_t[index] - begin_soil[index]) / case["dt_s"]
        soil_residuals.append(incoming - outgoing - storage)
        soil_scales.append(max(1.0, abs(incoming) + abs(outgoing) + abs(storage)))

    raw = [*hydraulic, *energy_residuals, canopy_heat_residual,
           canopy_vapor_residual, surface_residual, *soil_residuals]
    identities = ["sun_gas_minus_q1", "shade_gas_minus_q1",
        "sun_gas_minus_vulnerability_demand", "shade_gas_minus_vulnerability_demand",
        "q1_sum_minus_q2", "q2_minus_root_source_sum", "sun_leaf_energy",
        "shade_leaf_energy", "wet_surface_energy", "dry_stem_energy",
        "shared_canopy_air_heat", "shared_canopy_air_vapor", "ground_surface_energy",
        *(f"soil_thermal:{node['layer_id']}" for node in nodes)]
    tolerances = [WATER_ATOL + WATER_RTOL * water_scale] * 6
    tolerances += [ENERGY_ATOL + ENERGY_RTOL * value for value in energy_scales]
    tolerances += [ENERGY_ATOL + ENERGY_RTOL * canopy_heat_scale,
        WATER_ATOL + WATER_RTOL * canopy_vapor_scale,
        ENERGY_ATOL + ENERGY_RTOL * surface_scale]
    tolerances += [ENERGY_ATOL + ENERGY_RTOL * value for value in soil_scales]
    normalized = [value / tolerance for value, tolerance in zip(raw, tolerances, strict=True)]
    f_t, dt = case["tile_fraction"], case["dt_s"]
    for row in q3:
        cap_rate = _supplied_cap_rate(caps, "root", row["layer_id"])
        authorization_amount = (None if cap_rate is None else cap_rate * f_t * dt)
        finalized_amount = (authorization_amount
            if row["branch"] == "authorization_active_or_tie"
            else row["q_final_kg_m2_tile_s"] * f_t * dt)
        row.update({"request_kg_m2_stand_ground":
            row["request_rate_kg_m2_tile_s"] * f_t * dt,
            "authorization_rate_kg_m2_tile_s": cap_rate,
            "authorization_kg_m2_stand_ground": authorization_amount,
            "finalized_use_kg_m2_stand_ground": finalized_amount})
    ground_cap_rate = _supplied_cap_rate(
        caps, "ground", ground_config["surface_source_id"])
    ground_authorization_amount = (None if ground_cap_rate is None else
                                   ground_cap_rate * f_t * dt)
    ground_finalized_amount = (ground_authorization_amount
        if ground_branch == "authorization_active_or_tie"
        else max(ground_vapor, 0.0) * f_t * dt)
    ground_record = {"source_id": ground_config["surface_source_id"],
        "q_law_kg_m2_tile_s": ground_law, "q_final_kg_m2_tile_s": ground_vapor,
        "request_rate_kg_m2_tile_s": ground_request, "branch": ground_branch,
        "request_kg_m2_stand_ground": ground_request * f_t * dt,
        "authorization_rate_kg_m2_tile_s": ground_cap_rate,
        "authorization_kg_m2_stand_ground": ground_authorization_amount,
        "finalized_use_kg_m2_stand_ground": ground_finalized_amount,
        "condensation_credit_kg_m2_stand_ground": f_t * max(-ground_vapor, 0.0) * dt}
    return raw, {"identities": identities, "normalized_residuals": normalized,
        "tolerances": tolerances, "sun": sun, "shade": shade,
        "component_temperatures_k": {"sun_leaf": tsun, "shade_leaf": tshade,
            "wet_surface": twet, "dry_stem": tstem},
        "component_surface_q_kg_kg": {"sun_leaf": sun["surface_q_kg_kg"],
            "shade_leaf": shade["surface_q_kg_kg"], "wet_surface": qsat_wet},
        "component_transpiration_kg_m2_tile_s": transpiration,
        "wet_vapor_kg_m2_tile_s": wet_actual,
        "wet_store_cap_branch": wet_branch,
        "wet_store_cap_active": wet_branch == "store_cap_active_or_tie",
        "canopy_air_temperature_k": tcan, "canopy_air_specific_humidity_kg_kg": qcan,
        "ground_temperature_k": tsurface, "ground_sensible_w_m2_tile": ground_h,
        "ground_aerodynamics": ground_aerodynamics,
        "ground_vapor": ground_record, "surface_storage_w_m2_tile": surface_storage,
        "surface_vapor_energy_w_m2_tile": qv, "soil_temperature_k": soil_t,
        "ground_terminal_shortwave_by_band_direction_w_m2_tile":
            copy.deepcopy(ground_config["ground_terminal_shortwave_by_band_direction_w_m2_tile"]),
        "ground_absorbed_shortwave_by_band_direction_w_m2_tile":
            ground_shortwave_absorbed,
        "ground_heat_end_w_m2_tile": g_end, "ground_heat_cn_w_m2_tile": g_bar,
        "longwave": longwave, "q1_sun": q1_sun, "q1_shade": q1_shade,
        "q2": q2, "q3": q3, "hydraulic_potentials_mm": {
            "sun_leaf": psi_sun, "shade_leaf": psi_shade, "stem": psi_stem, "root": psi_root},
        "beta_hyd": {"sun": beta_sun, "shade": beta_shade},
        "energy_component_scales_w_m2": energy_scales,
        "canopy_heat_scale_w_m2": canopy_heat_scale,
        "canopy_vapor_scale_kg_m2_s": canopy_vapor_scale,
        "surface_energy_scale_w_m2": surface_scale,
        "soil_energy_scales_w_m2": soil_scales,
        "active_water_caps": sorted([*(f"root:{row['layer_id']}" for row in q3
            if row["branch"] == "authorization_active_or_tie"),
            *([f"ground:{ground_config['surface_source_id']}"]
              if ground_branch == "authorization_active_or_tie" else [])])}


def evaluate_joint_residual(case: dict[str, Any], ground_config: dict[str, Any],
                            ground_state: dict[str, Any], trial: list[float],
                            caps: dict[str, Any] | None = None,
                            frozen_branches: dict[str, str] | None = None) -> dict[str, Any]:
    """Expose current-trial ordered component, hydraulic, ground, and soil blocks."""
    raw, detail = _raw_residual(case, ground_config, ground_state, trial, caps,
                                frozen_branches)
    return {"raw_residuals": raw, **detail}


def build_covered_column_case(rank_count: int = 1) -> dict[str, Any]:
    """Build one or two ordered V8 occupancies sharing one tile air/ground node."""
    if rank_count not in (1, 2):
        raise ValueError("covered_column_rank_count")
    base = build_joint_case()
    occupancies = []
    for rank in range(rank_count):
        case = copy.deepcopy(base["case"])
        identity = f"canopy-rank-{rank}"
        if rank:
            # A lower occupancy is not an averaged copy: its conditional area,
            # absorbed radiation, warm starts, and clumping remain distinct.
            area_factor = 0.58
            case["parameters"]["lai"] *= area_factor
            case["parameters"]["sai"] *= area_factor
            case["parameters"]["sun_leaf_area"] *= area_factor
            case["parameters"]["shade_leaf_area"] *= area_factor
            case["gas_energy"]["stem_area"] *= area_factor
            case["classes"]["sun"]["leaf_area"] *= area_factor
            case["classes"]["shade"]["leaf_area"] *= area_factor
            case["canopy_longwave"]["clumping_index"] = 0.91
            case["classes"]["sun"]["temperature_start_k"] -= 0.7
            case["classes"]["shade"]["temperature_start_k"] -= 0.4
        occupancy_start = _default_start(case, base["ground_state"])
        occupancies.append({"occupancy_id": identity, "rank": rank,
                            "case": case, "start": occupancy_start[:10]})
    common_start = [*base["start"][10:]]
    optics = {
        "canopy-rank-0": {
            "VIS": {"leaf": {"rho": 0.09, "tau": 0.06},
                    "stem": {"rho": 0.18, "tau": 0.03}},
            "NIR": {"leaf": {"rho": 0.41, "tau": 0.31},
                    "stem": {"rho": 0.29, "tau": 0.12}}},
        "canopy-rank-1": {
            "VIS": {"leaf": {"rho": 0.12, "tau": 0.04},
                    "stem": {"rho": 0.22, "tau": 0.02}},
            "NIR": {"leaf": {"rho": 0.37, "tau": 0.27},
                    "stem": {"rho": 0.25, "tau": 0.10}}},
    }
    return {"occupancies": occupancies, "ground_config": base["ground_config"],
            "ground_state": base["ground_state"],
            "shortwave": {"zenith_cosine": 0.67,
                "incident_by_band_w_m2_tile": {
                    "VIS": {"direct": 410.0, "diffuse": 83.0},
                    "NIR": {"direct": 355.0, "diffuse": 101.0}},
                "optics_by_occupancy": {item["occupancy_id"]:
                    copy.deepcopy(optics[item["occupancy_id"]]) for item in occupancies}},
            "start": [*(value for item in occupancies for value in item["start"]),
                      *common_start]}


def _prepare_covered_shortwave(column: dict[str, Any]) -> dict[str, Any]:
    """Execute the checksum-bound V3 lower-boundary two-stream column."""
    if "_executed_shortwave" in column:
        return column
    prepared = copy.deepcopy(column)
    spec = prepared["shortwave"]
    layers = []
    for occupancy in prepared["occupancies"]:
        case = occupancy["case"]
        layers.append({"occupancy_id": occupancy["occupancy_id"],
            "leaf_area": case["parameters"]["lai"],
            "stem_area": case["parameters"]["sai"],
            "clumping_index": case["canopy_longwave"]["clumping_index"],
            "leaf_angle_chi": 0.12 if occupancy["rank"] == 0 else -0.08,
            "optics": spec["optics_by_occupancy"][occupancy["occupancy_id"]]})
    executed: dict[str, dict[str, Any]] = {}
    for band, incident in spec["incident_by_band_w_m2_tile"].items():
        albedo = prepared["ground_config"][
            "ground_surface_albedo_vis" if band == "VIS" else
            "ground_surface_albedo_nir"]
        executed[band] = {
            "direct": V3.radiation_component(layers, band, spec["zenith_cosine"],
                incident["direct"], 0.0, albedo),
            "diffuse": V3.radiation_component(layers, band, spec["zenith_cosine"],
                0.0, incident["diffuse"], albedo),
        }
    terminal: dict[str, float] = {}
    for band in ("VIS", "NIR"):
        terminal[f"direct_{band.lower()}"] = executed[band]["direct"]["terminal_direct"]
        terminal[f"diffuse_{band.lower()}"] = (
            executed[band]["direct"]["terminal_diffuse"]
            + executed[band]["diffuse"]["terminal_diffuse"])
    prepared["ground_config"][
        "ground_terminal_shortwave_by_band_direction_w_m2_tile"] = terminal
    for index, occupancy in enumerate(prepared["occupancies"]):
        case = occupancy["case"]
        by_band = {band: {direction: executed[band][direction]["occupancies"][index]["results"]
            for direction in ("direct", "diffuse")} for band in ("VIS", "NIR")}
        sun_sw = math.fsum(by_band[band][direction]["absorbed_leaf_sun"]
            for band in ("VIS", "NIR") for direction in ("direct", "diffuse"))
        shade_sw = math.fsum(by_band[band][direction]["absorbed_leaf_shade"]
            for band in ("VIS", "NIR") for direction in ("direct", "diffuse"))
        stem_sw = math.fsum(by_band[band][direction]["absorbed_stem"]
            for band in ("VIS", "NIR") for direction in ("direct", "diffuse"))
        sun_area = by_band["VIS"]["direct"]["leaf_sun_area"]
        shade_area = by_band["VIS"]["direct"]["leaf_shade_area"]
        case["classes"]["sun"].update({"leaf_area": sun_area,
            "absorbed_shortwave_w_m2_tile": sun_sw,
            "absorbed_par_w_m2_leaf": math.fsum(by_band["VIS"][direction]
                ["absorbed_leaf_sun"] for direction in ("direct", "diffuse")) / sun_area})
        case["classes"]["shade"].update({"leaf_area": shade_area,
            "absorbed_shortwave_w_m2_tile": shade_sw,
            "absorbed_par_w_m2_leaf": math.fsum(by_band["VIS"][direction]
                ["absorbed_leaf_shade"] for direction in ("direct", "diffuse")) / shade_area})
        case["parameters"].update({"sun_leaf_area": sun_area,
                                    "shade_leaf_area": shade_area})
        case["gas_energy"]["stem_absorbed_shortwave_w_m2_tile"] = stem_sw
    prepared["_executed_shortwave"] = {"layers": layers, "by_band": executed,
        "terminal_by_band_direction_w_m2_tile": terminal}
    return prepared


def _column_longwave(occupancies: list[dict[str, Any]], rank_trials: list[list[float]],
                     ground_temperature: float) -> dict[str, Any]:
    down = occupancies[0]["case"]["canopy_longwave"]["atmospheric_down_w_m2"]
    layers = []
    for occupancy, trial in zip(occupancies, rank_trials, strict=True):
        case = occupancy["case"]
        wet = case["gas_energy"]["wet_fraction"]
        areas = [case["classes"]["sun"]["leaf_area"] * (1.0 - wet),
            case["classes"]["shade"]["leaf_area"] * (1.0 - wet),
            wet * (case["classes"]["sun"]["leaf_area"]
                   + case["classes"]["shade"]["leaf_area"]
                   + case["gas_energy"]["stem_area"]),
            (1.0 - wet) * case["gas_energy"]["stem_area"]]
        temperatures = trial[6:10]
        area_sum = math.fsum(areas)
        weights = [value / area_sum for value in areas]
        emission = math.fsum(weight * SIGMA * temperature**4 for weight, temperature
                             in zip(weights, temperatures, strict=True))
        tau = math.exp(-case["canopy_longwave"]["extinction_m2_plant_m2_ground"]
            * case["canopy_longwave"]["clumping_index"]
            * (case["parameters"]["lai"] + case["parameters"]["sai"]))
        layers.append({"tau": tau, "areas": areas, "weights": weights,
                       "temperatures": temperatures, "emission": emission,
                       "down_top": down})
        down = tau * down + (1.0 - tau) * emission
    up = SIGMA * ground_temperature**4
    for layer in reversed(layers):
        layer["up_bottom"] = up
        up = layer["tau"] * up + (1.0 - layer["tau"]) * layer["emission"]
        layer["up_top"] = up
    receipts = []
    for layer in layers:
        component = [weight * (1.0 - layer["tau"])
            * (layer["down_top"] + layer["up_bottom"])
            - 2.0 * weight * (1.0 - layer["tau"]) * SIGMA * temperature**4
            for weight, temperature in zip(layer["weights"], layer["temperatures"], strict=True)]
        receipts.append({"tau": layer["tau"],
            "component_net_w_m2_tile": component,
            "down_top_w_m2": layer["down_top"], "up_bottom_w_m2": layer["up_bottom"]})
    ground_net = down - SIGMA * ground_temperature**4
    top_down = occupancies[0]["case"]["canopy_longwave"]["atmospheric_down_w_m2"]
    closure = top_down - up - ground_net - math.fsum(
        math.fsum(item["component_net_w_m2_tile"]) for item in receipts)
    return {"occupancy_receipts": receipts, "ground_net_w_m2_tile": ground_net,
            "terminal_down_w_m2_tile": down, "top_up_w_m2_tile": up,
            "closure_w_m2_tile": closure}


def evaluate_covered_column_residual(column: dict[str, Any], trial: list[float],
                                     caps: dict[str, Any] | None = None,
                                     frozen_branches: dict[str, str] | None = None) -> dict[str, Any]:
    """Evaluate the exact shared-air, reciprocal-LW ordered covered-column vector."""
    column = _prepare_covered_shortwave(column)
    occupancies = column["occupancies"]
    node_count = len(column["ground_config"]["soil_nodes"])
    expected = 10 * len(occupancies) + 3 + node_count
    if len(trial) != expected:
        raise ValueError("covered_column_trial_identity_schema")
    rank_trials = [trial[index * 10:(index + 1) * 10]
                   for index in range(len(occupancies))]
    common = trial[10 * len(occupancies):]
    tcan, qcan, tground = common[:3]
    soil = common[3:]
    longwave = _column_longwave(occupancies, rank_trials, tground)
    rank_details, raw, tolerances = [], [], []
    shared_heat_rows, shared_vapor_rows = [], []
    for index, (occupancy, rank_trial) in enumerate(zip(occupancies, rank_trials, strict=True)):
        full_trial = [*rank_trial, tcan, qcan, tground, *soil]
        local_caps = None if caps is None else {"root": caps["root"][occupancy["occupancy_id"]],
                                                "ground": caps["ground"]}
        local_frozen = None
        if frozen_branches:
            prefix = occupancy["occupancy_id"] + ":"
            local_frozen = {key.removeprefix(prefix): value for key, value
                            in frozen_branches.items() if key.startswith(prefix)}
            if "ground" in frozen_branches:
                local_frozen["ground"] = frozen_branches["ground"]
        local_raw, detail = _raw_residual(occupancy["case"], column["ground_config"],
            column["ground_state"], full_trial, local_caps, local_frozen)
        new_lw = longwave["occupancy_receipts"][index]["component_net_w_m2_tile"]
        old_lw = detail["longwave"]["component_net_w_m2_tile"]
        adjusted_energy = []
        adjusted_tolerance = []
        for component in range(4):
            value = local_raw[6 + component] + new_lw[component] - old_lw[component]
            old_scale = detail["energy_component_scales_w_m2"][component]
            scale = max(1.0, old_scale - abs(old_lw[component]) + abs(new_lw[component]))
            adjusted_energy.append(value)
            adjusted_tolerance.append(ENERGY_ATOL + ENERGY_RTOL * scale)
        raw.extend([*local_raw[:6], *adjusted_energy])
        tolerances.extend([*detail["tolerances"][:6], *adjusted_tolerance])
        shared_heat_rows.append(local_raw[10])
        shared_vapor_rows.append(local_raw[11])
        rank_details.append({"occupancy_id": occupancy["occupancy_id"],
            "hydraulic_and_component": detail,
            "current_longwave_receipt": longwave["occupancy_receipts"][index]})
    first = rank_details[0]["hydraulic_and_component"]
    forcing = occupancies[0]["case"]["gas_energy"]
    rho = forcing["pressure_pa"] / (forcing["rdry_j_kg_k"] * tcan)
    ground_h = first["ground_sensible_w_m2_tile"]
    ground_v = first["ground_vapor"]["q_final_kg_m2_tile_s"]
    reference_h = rho * forcing["cp_air_j_kg_k"] * (tcan - forcing["air_temperature_k"]) / forcing["rah_s_m"]
    reference_v = rho * (qcan - forcing["air_specific_humidity_kg_kg"]) / forcing["raw_s_m"]
    duplicate_count = len(occupancies) - 1
    shared_heat = math.fsum(shared_heat_rows) - duplicate_count * (ground_h - reference_h)
    shared_vapor = math.fsum(shared_vapor_rows) - duplicate_count * (ground_v - reference_v)
    heat_scale = max(1.0, math.fsum(detail["canopy_heat_scale_w_m2"]
        for detail in (item["hydraulic_and_component"] for item in rank_details))
        - duplicate_count * (abs(ground_h) + abs(reference_h)))
    vapor_scale = max(WATER_ATOL, *(detail["canopy_vapor_scale_kg_m2_s"]
        for detail in (item["hydraulic_and_component"] for item in rank_details)),
        abs(shared_vapor), abs(ground_v), abs(reference_v))
    # Detail does not duplicate producer residuals; use the first evaluated row
    # and replace only its single-layer LW operand with the full-column value.
    first_frozen = None
    if frozen_branches:
        first_prefix = occupancies[0]["occupancy_id"] + ":"
        first_frozen = {key.removeprefix(first_prefix): value for key, value
            in frozen_branches.items() if key.startswith(first_prefix)}
        if "ground" in frozen_branches:
            first_frozen["ground"] = frozen_branches["ground"]
    first_full, _ = _raw_residual(occupancies[0]["case"], column["ground_config"],
        column["ground_state"], [*rank_trials[0], tcan, qcan, tground, *soil],
        None if caps is None else {"root": caps["root"][occupancies[0]["occupancy_id"]],
                                   "ground": caps["ground"]}, first_frozen)
    surface = first_full[12] + longwave["ground_net_w_m2_tile"] \
        - first["longwave"]["ground_net_w_m2_tile"]
    surface_scale = max(1.0, first["surface_energy_scale_w_m2"]
        - abs(first["longwave"]["ground_net_w_m2_tile"])
        + abs(longwave["ground_net_w_m2_tile"]))
    raw.extend([shared_heat, shared_vapor, surface, *first_full[13:]])
    tolerances.extend([ENERGY_ATOL + ENERGY_RTOL * heat_scale,
        WATER_ATOL + WATER_RTOL * vapor_scale,
        ENERGY_ATOL + ENERGY_RTOL * surface_scale, *first["tolerances"][13:]])
    normalized = [value / tolerance for value, tolerance in zip(raw, tolerances, strict=True)]
    identities = [*(f"{item['occupancy_id']}:{identity}" for item in rank_details
        for identity in item["hydraulic_and_component"]["identities"][:10]),
        "shared_canopy_air_heat", "shared_canopy_air_vapor", "ground_surface_energy",
        *(f"soil_thermal:{node['layer_id']}" for node in column["ground_config"]["soil_nodes"])]
    return {"raw_residuals": raw, "normalized_residuals": normalized,
        "tolerances": tolerances, "identities": identities,
        "occupancies": rank_details, "shared_canopy_air": {
            "temperature_k": tcan, "specific_humidity_kg_kg": qcan,
            "heat_residual_w_m2_tile": shared_heat,
            "vapor_residual_kg_m2_tile_s": shared_vapor},
        "ground": first, "whole_column_longwave": longwave,
        "ground_terminal_shortwave_by_band_direction_w_m2_tile":
            copy.deepcopy(column["ground_config"]["ground_terminal_shortwave_by_band_direction_w_m2_tile"]),
        "whole_column_shortwave": copy.deepcopy(column["_executed_shortwave"])}


def _freeze_covered_branches(detail: dict[str, Any]) -> dict[str, str]:
    branches: dict[str, str] = {}
    for occupancy in detail["occupancies"]:
        prefix = occupancy["occupancy_id"] + ":"
        local = occupancy["hydraulic_and_component"]
        for row in local["q3"]:
            branches[prefix + "root:" + row["layer_id"]] = row["branch"]
        branches[prefix + "wet_surface"] = local["wet_store_cap_branch"]
    branches["ground"] = detail["ground"]["ground_vapor"]["branch"]
    return branches


def _solve_normalized_system(evaluator: Any, start: list[float], units: list[float],
                             valid_trial: Any, max_iterations: int,
                             max_halvings: int = 20,
                             frozen_evaluator: Any | None = None,
                             freeze_branches: Any | None = None,
                             step_measure: Any | None = None,
                             step_tolerances: dict[str, float] | None = None) -> dict[str, Any]:
    x = start[:]
    history: list[float] = []
    backtracking = 0
    last_step: float | None = None
    last_steps: dict[str, float] | None = None
    pivot = matrix_norm = None
    for iteration in range(max_iterations + 1):
        detail = evaluator(x)
        normalized = detail["normalized_residuals"]
        norm = max(abs(value) for value in normalized)
        history.append(norm)
        step_ok = (last_steps is not None and step_tolerances is not None
            and all(last_steps[key] <= tolerance
                    for key, tolerance in step_tolerances.items())) \
            if step_measure is not None else (last_step is not None and last_step <= 1.0e-8)
        if norm <= 1.0 and step_ok:
            return {"accepted": True, "iterations": iteration, "solution": x,
                "detail": detail, "residual_norm_history": history,
                "backtracking_count": backtracking, "step_norm": last_step,
                "step_norms": last_steps,
                "pivot_magnitude": pivot, "matrix_norm": matrix_norm}
        if iteration == max_iterations:
            return {"accepted": False, "failure": "iteration_limit", "candidate": None,
                "iterations": iteration, "detail": detail,
                "residual_norm_history": history, "backtracking_count": backtracking,
                "step_norm": last_step, "step_norms": last_steps,
                "pivot_magnitude": pivot, "matrix_norm": matrix_norm}
        frozen = freeze_branches(detail) if freeze_branches is not None else None
        def evaluate_frozen(value: list[float]) -> dict[str, Any]:
            return (frozen_evaluator(value, frozen)
                    if frozen_evaluator is not None else evaluator(value))
        perturb = [math.sqrt(EPSILON) * max(abs(value), unit)
                   for value, unit in zip(x, units, strict=True)]
        jacobian = [[0.0] * len(x) for _ in normalized]
        for column in range(len(x)):
            minus, plus = x[:], x[:]
            minus[column] -= perturb[column]
            plus[column] += perturb[column]
            nminus = evaluate_frozen(minus)["normalized_residuals"]
            nplus = evaluate_frozen(plus)["normalized_residuals"]
            for row in range(len(normalized)):
                jacobian[row][column] = (nplus[row] - nminus[row]) / (2.0 * perturb[column])
        try:
            delta, pivot, matrix_norm = V3.solve_linear(jacobian, [-value for value in normalized])
        except V3.SingularMatrixError as error:
            return {"accepted": False, "failure": "singular", "candidate": None,
                "iterations": iteration, "detail": detail,
                "residual_norm_history": history, "backtracking_count": backtracking,
                "pivot_magnitude": error.pivot, "matrix_norm": error.matrix_norm}
        if norm <= 1.0:
            prospective = [value + change for value, change in zip(x, delta, strict=True)]
            if valid_trial(prospective):
                try:
                    prospective_detail = evaluate_frozen(prospective)
                except (ValueError, ArithmeticError):
                    prospective_detail = None
                if prospective_detail is not None:
                    prospective_steps = (step_measure(delta, detail, prospective_detail)
                        if step_measure is not None else None)
                    prospective_ok = (prospective_steps is not None
                        and step_tolerances is not None
                        and all(prospective_steps[key] <= tolerance
                                for key, tolerance in step_tolerances.items())) \
                        if step_measure is not None else max(abs(value) for value in delta) <= 1.0e-10
                    if prospective_ok:
                        x = prospective
                        last_step = max(abs(value) for value in delta)
                        last_steps = prospective_steps
                        continue
        accepted = False
        for exponent in range(max_halvings + 1):
            factor = 0.5**exponent
            trial = [value + factor * change for value, change
                     in zip(x, delta, strict=True)]
            if not valid_trial(trial):
                continue
            try:
                trial_detail = evaluate_frozen(trial)
            except (ValueError, ArithmeticError):
                continue
            trial_norm = max(abs(value) for value in trial_detail["normalized_residuals"])
            if trial_norm < norm:
                x = trial
                applied = [factor * value for value in delta]
                last_step = max(abs(value) for value in applied)
                last_steps = (step_measure(applied, detail, trial_detail)
                              if step_measure is not None else None)
                backtracking += exponent
                accepted = True
                break
        if not accepted:
            return {"accepted": False, "failure": "backtracking_limit", "candidate": None,
                "iterations": iteration, "detail": detail,
                "residual_norm_history": history,
                "backtracking_count": backtracking + max_halvings,
                "step_norm": max(abs(value) for value in delta),
                "step_norms": last_steps,
                "pivot_magnitude": pivot, "matrix_norm": matrix_norm}
    raise AssertionError("unreachable")


def solve_covered_column(column: dict[str, Any], caps: dict[str, Any] | None = None,
                         max_iterations: int = 50, start: list[float] | None = None,
                         max_halvings: int = 20) -> dict[str, Any]:
    """Solve one/multirank covered column from immutable beginning state."""
    beginning_column = column
    column = _prepare_covered_shortwave(column)
    occupancies = column["occupancies"]
    if caps is not None:
        if set(caps) != {"root", "ground"} or set(caps["root"]) != {
                item["occupancy_id"] for item in occupancies}:
            raise ValueError("covered_column_authorization_identity_schema")
    beginning_hash = digest({"ground": beginning_column["ground_state"],
        "occupancies": [item["case"] for item in beginning_column["occupancies"]]})
    if len(occupancies) == 1:
        local_caps = None if caps is None else {
            "root": caps["root"][occupancies[0]["occupancy_id"]],
            "ground": caps["ground"]}
        selected_start = column["start"] if start is None else start
        local_start = [*selected_start[:10], *selected_start[10:]]
        result = solve_joint(occupancies[0]["case"], column["ground_config"],
            column["ground_state"], caps=local_caps, max_iterations=max_iterations,
            start=local_start)
        result.update({"beginning_sha256": beginning_hash,
            "rollback_sha256": beginning_hash,
            "rebuilt_from_beginning_sha256": beginning_hash})
        if result["accepted"]:
            result["components"]["whole_column_shortwave"] = copy.deepcopy(
                column["_executed_shortwave"])
        return result
    initial = column["start"][:] if start is None else start[:]
    units = [*(value for _ in occupancies for value in
        [1000.0, 1000.0, 1000.0, 1000.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
        1.0, 0.001, 1.0, *([1.0] * len(column["ground_config"]["soil_nodes"]))]
    def valid(value: list[float]) -> bool:
        if not all(math.isfinite(item) for item in value):
            return False
        for index in range(len(occupancies)):
            block = value[index * 10:(index + 1) * 10]
            if not (0.0 <= block[4] <= 1.0 and 0.0 <= block[5] <= 1.0
                    and all(200.0 <= item <= 350.0 for item in block[6:10])):
                return False
        common = value[10 * len(occupancies):]
        return (200.0 <= common[0] <= 350.0 and 0.0 <= common[1] <= 0.1
                and all(200.0 <= item <= 350.0 for item in common[2:]))
    def freeze(detail: dict[str, Any]) -> dict[str, str]:
        return _freeze_covered_branches(detail)
    def measure(applied: list[float], before: dict[str, Any],
                after: dict[str, Any]) -> dict[str, float]:
        hydraulic = beta = temperature = 0.0
        for index in range(len(occupancies)):
            offset = 10 * index
            hydraulic = max(hydraulic, *(abs(value)
                for value in applied[offset:offset + 4]))
            beta = max(beta, *(abs(value) for value in applied[offset + 4:offset + 6]))
            temperature = max(temperature, *(abs(value)
                for value in applied[offset + 6:offset + 10]))
        common_offset = 10 * len(occupancies)
        temperature = max(temperature, abs(applied[common_offset]),
            *(abs(value) for value in applied[common_offset + 2:]))
        ci = 0.0
        for old, new in zip(before["occupancies"], after["occupancies"], strict=True):
            old_local, new_local = (item["hydraulic_and_component"] for item in (old, new))
            ci = max(ci, abs(new_local["sun"]["ci_pa"] - old_local["sun"]["ci_pa"]),
                abs(new_local["shade"]["ci_pa"] - old_local["shade"]["ci_pa"]))
        return {"temperature_k": temperature,
            "humidity_kg_kg": abs(applied[common_offset + 1]),
            "ci_pa": ci, "hydraulic_mm": hydraulic, "beta": beta}
    result = _solve_normalized_system(
        lambda value: evaluate_covered_column_residual(column, value, caps),
        initial, units, valid, max_iterations, max_halvings,
        frozen_evaluator=lambda value, branches:
            evaluate_covered_column_residual(column, value, caps, branches),
        freeze_branches=freeze, step_measure=measure,
        step_tolerances={"temperature_k": 1.0e-8,
            "humidity_kg_kg": 1.0e-12, "ci_pa": 1.0e-8,
            "hydraulic_mm": 1.0e-7, "beta": 1.0e-10})
    result.update({"pass": "capped" if caps else "potential",
        "beginning_sha256": beginning_hash, "rollback_sha256": beginning_hash,
        "rebuilt_from_beginning_sha256": beginning_hash})
    if result["accepted"]:
        detail = result["detail"]
        result["candidate"] = {"vegetation": [
            {"occupancy_id": item["occupancy_id"],
             "component_temperatures_k": item["hydraulic_and_component"]["component_temperatures_k"],
             "hydraulic_potentials_mm": item["hydraulic_and_component"]["hydraulic_potentials_mm"]}
            for item in detail["occupancies"]],
            "shared_canopy_air": detail["shared_canopy_air"],
            "lse": {"surface_enthalpy_j_m2_tile":
                column["ground_state"]["surface_enthalpy_j_m2_tile"]
                + detail["ground"]["surface_storage_w_m2_tile"]
                * occupancies[0]["case"]["dt_s"],
                "surface_temperature_warm_start_k": detail["ground"]["ground_temperature_k"]},
            "soil_thermal": {"temperature_k": detail["ground"]["soil_temperature_k"]},
            "hydrology_mass_mutation": None}
    return result


def build_open_bare_soil_case(layer_count: int = 2) -> dict[str, Any]:
    """Build an exact open neutral CLM-humidity/CN thermal-column case."""
    if layer_count < 1:
        raise ValueError("open_soil_layer_count")
    nodes = [{"layer_id": f"thermal-{index + 1}",
        "depth_m": 0.08 + 0.05 * index,
        "conductivity_w_m_k": 1.1 + 0.12 * index,
        "heat_capacity_j_m2_k": 120_000.0 + 35_000.0 * index}
        for index in range(layer_count)]
    configuration = {"tile_id": "open", "tile_fraction": 1.0,
        "surface_source_id": "soil-layer-1", "surface_class": "bare_mineral_soil",
        "open_geometry": {"z_ref_m": 20.0, "z0m_m": 0.12,
            "z0h_m": 0.015, "z0q_m": 0.010},
        "surface_depth_m": 0.02, "surface_conductivity_w_m_k": 0.75,
        "surface_dry_heat_capacity_j_m2_k": 42_000.0,
        "surface_storage_branch": "finite_capacity",
        "surface_terminal_shortwave_by_band_direction_w_m2_tile": {
            "direct_vis": 91.0, "diffuse_vis": 31.0,
            "direct_nir": 117.0, "diffuse_nir": 39.0},
        "ground_surface_albedo_vis": 0.18,
        "ground_surface_albedo_nir": 0.31,
        "soil_nodes": nodes, "theta_sat": 0.46, "psi_sat_mm": -120.0,
        "clapp_hornberger_b": 4.05, "porosity": 0.46,
        "theta_init": 0.22, "surface_emissivity": 1.0}
    forcing = {"dt_s": 1800.0, "pressure_pa": 93_000.0,
        "air_temperature_k": 294.0, "air_specific_humidity_kg_kg": 0.0095,
        "reference_wind_m_s": 2.4, "atmospheric_longwave_w_m2": 335.0,
        "rdry_j_kg_k": 287.05, "cp_air_j_kg_k": 1004.64}
    surface_start = 295.0
    state = {"surface_liquid_kg_m2_tile": 0.0,
        "surface_enthalpy_j_m2_tile": configuration["surface_dry_heat_capacity_j_m2_k"]
            * (surface_start - TREF),
        "surface_temperature_warm_start_k": surface_start,
        "top_layer_liquid_kg_m2": 26.0, "top_layer_ice_kg_m2": 0.0,
        "soil_temperature_k": [291.5 - 1.1 * index for index in range(layer_count)]}
    return {"configuration": configuration, "forcing": forcing, "state": state,
            "start": [surface_start, *state["soil_temperature_k"]]}


def open_neutral_resistances(configuration: dict[str, Any],
                             reference_wind_m_s: float) -> dict[str, float]:
    geometry = configuration["open_geometry"]
    zref = geometry["z_ref_m"]
    roughness = [geometry[key] for key in ("z0m_m", "z0h_m", "z0q_m")]
    if (not math.isfinite(reference_wind_m_s) or reference_wind_m_s <= 0.0
            or not zref > max(roughness) > 0.0):
        raise ValueError("open_neutral_geometry_domain")
    momentum = math.log(zref / roughness[0])
    heat = momentum * math.log(zref / roughness[1]) / (0.4**2 * reference_wind_m_s)
    vapor = momentum * math.log(zref / roughness[2]) / (0.4**2 * reference_wind_m_s)
    if not heat > 0.0 or not vapor > 0.0:
        raise ValueError("open_neutral_resistance_domain")
    return {"heat_resistance_s_m": heat, "vapor_resistance_s_m": vapor}


def evaluate_open_bare_soil_residual(bundle: dict[str, Any], trial: list[float],
                                     cap: dict[str, float] | float | None = None) -> dict[str, Any]:
    configuration, forcing, state = (bundle[key] for key in
        ("configuration", "forcing", "state"))
    nodes = configuration["soil_nodes"]
    if len(trial) != 1 + len(nodes):
        raise ValueError("open_soil_trial_identity_schema")
    tsurface, soil_t = trial[0], trial[1:]
    if not all(math.isfinite(value) and 200.0 <= value <= 350.0 for value in trial):
        raise ValueError("open_soil_temperature_domain")
    resistance = open_neutral_resistances(configuration, forcing["reference_wind_m_s"])
    top = nodes[0]
    liquid = state["top_layer_liquid_kg_m2"]
    ice = state["top_layer_ice_kg_m2"]
    saturation = min(1.0, max(0.01, (liquid / 1000.0 + ice / 917.0)
        / (top["depth_m"] * configuration["theta_sat"])))
    theta = liquid / (1000.0 * top["depth_m"])
    psi = max(-1.0e8, configuration["psi_sat_mm"]
              * saturation ** (-configuration["clapp_hornberger_b"]))
    alpha = math.exp(psi * 9.80665 / (1000.0 * 461.5 * tsurface))
    theta_air = configuration["porosity"] * (configuration["psi_sat_mm"] / -1.0e7) \
        ** (1.0 / configuration["clapp_hornberger_b"])
    dsl = (0.015 * (configuration["theta_init"] - theta)
        / (configuration["theta_init"] - theta_air)
        if theta < configuration["theta_init"] else 0.0)
    phi_air = configuration["porosity"] - theta_air
    tau_pore = phi_air**2 * (phi_air / configuration["porosity"]) \
        ** (3.0 / configuration["clapp_hornberger_b"])
    diffusivity = 2.12e-5 * (tsurface / TREF) ** 1.75
    rsoil = dsl / (diffusivity * tau_pore)
    q_surface = alpha * qsat(tsurface, forcing["pressure_pa"])
    q_air = forcing["air_specific_humidity_kg_kg"]
    zero_flux_branch = qsat(tsurface, forcing["pressure_pa"]) > q_air > q_surface
    if zero_flux_branch:
        q_surface = q_air
    rho_air = forcing["pressure_pa"] / (
        forcing["rdry_j_kg_k"] * forcing["air_temperature_k"])
    law = rho_air * (q_surface - q_air) \
        / (resistance["vapor_resistance_s_m"] + rsoil)
    caps = None if cap is None else {"ground": {configuration["surface_source_id"]: cap}}
    final, request, branch = (_cap_value(caps, "ground",
        configuration["surface_source_id"], law) if law >= 0.0
        else (law, 0.0, "condensation"))
    sensible = rho_air * forcing["cp_air_j_kg_k"] \
        * (tsurface - forcing["air_temperature_k"]) / resistance["heat_resistance_s_m"]
    longwave = forcing["atmospheric_longwave_w_m2"] - SIGMA * tsurface**4
    absorbed_shortwave = _absorbed_terminal_shortwave(
        configuration, "surface_terminal_shortwave_by_band_direction_w_m2_tile")
    sw = math.fsum(absorbed_shortwave.values())
    capacity = configuration["surface_dry_heat_capacity_j_m2_k"]
    storage_branch = configuration.get("surface_storage_branch", "finite_capacity")
    if storage_branch == "finite_capacity":
        if capacity <= 0.0:
            raise ValueError("finite_surface_capacity_domain")
        storage = (capacity * (tsurface - TREF)
                   - state["surface_enthalpy_j_m2_tile"]) / forcing["dt_s"]
        begin_surface = TREF + state["surface_enthalpy_j_m2_tile"] / capacity
    elif storage_branch == "equilibrium_zero":
        if not (capacity == 0.0 and state["surface_liquid_kg_m2_tile"] == 0.0
                and state["surface_enthalpy_j_m2_tile"] == 0.0):
            raise ValueError("equilibrium_zero_surface_state")
        storage = 0.0
        begin_surface = tsurface
    else:
        raise ValueError("surface_storage_branch")
    g_end = [_conductance(configuration["surface_depth_m"],
        configuration["surface_conductivity_w_m_k"], top["depth_m"],
        top["conductivity_w_m_k"]) * (tsurface - soil_t[0])]
    for index in range(len(nodes) - 1):
        g_end.append(_conductance(nodes[index]["depth_m"], nodes[index]["conductivity_w_m_k"],
            nodes[index + 1]["depth_m"], nodes[index + 1]["conductivity_w_m_k"])
            * (soil_t[index] - soil_t[index + 1]))
    begin_soil = state["soil_temperature_k"]
    g_begin = [_conductance(configuration["surface_depth_m"],
        configuration["surface_conductivity_w_m_k"], top["depth_m"],
        top["conductivity_w_m_k"]) * (begin_surface - begin_soil[0])]
    for index in range(len(nodes) - 1):
        g_begin.append(_conductance(nodes[index]["depth_m"], nodes[index]["conductivity_w_m_k"],
            nodes[index + 1]["depth_m"], nodes[index + 1]["conductivity_w_m_k"])
            * (begin_soil[index] - begin_soil[index + 1]))
    gbar = [0.5 * (old + new) for old, new in zip(g_begin, g_end, strict=True)]
    vapor_energy = final * (liquid_enthalpy(tsurface) + vaporization_enthalpy(tsurface))
    surface_operands = [sw, longwave, -sensible, -vapor_energy, -gbar[0], -storage]
    raw = [math.fsum(surface_operands)]
    scales = [max(1.0, math.fsum(abs(value) for value in surface_operands))]
    soil_records = []
    for index, node in enumerate(nodes):
        incoming = gbar[index]
        outgoing = gbar[index + 1] if index + 1 < len(gbar) else 0.0
        node_storage = node["heat_capacity_j_m2_k"] * (soil_t[index] - begin_soil[index]) / forcing["dt_s"]
        residual = incoming - outgoing - node_storage
        raw.append(residual)
        scales.append(max(1.0, abs(incoming) + abs(outgoing) + abs(node_storage)))
        soil_records.append({"layer_id": node["layer_id"], "temperature_k": soil_t[index],
            "incoming_cn_w_m2": incoming, "outgoing_cn_w_m2": outgoing,
            "storage_w_m2": node_storage, "residual_w_m2": residual})
    tolerances = [ENERGY_ATOL + ENERGY_RTOL * value for value in scales]
    normalized = [value / tolerance for value, tolerance in zip(raw, tolerances, strict=True)]
    f_t, dt = configuration["tile_fraction"], forcing["dt_s"]
    cap_rate = None if cap is None else (cap["cap_rate_kg_m2_tile_s"]
        if isinstance(cap, dict) else cap)
    authorization = None if cap_rate is None else cap_rate * f_t * dt
    finalized = authorization if branch == "authorization_active_or_tie" else max(final, 0.0) * f_t * dt
    return {"raw_residuals": raw, "normalized_residuals": normalized,
        "tolerances": tolerances, "surface_temperature_k": tsurface,
        "soil_temperature_k": soil_t, "soil_records": soil_records,
        "surface_humidity_operands": {"saturation": saturation, "theta": theta,
            "psi_mm": psi, "alpha": alpha, "theta_air": theta_air, "dsl_m": dsl,
            "tau_pore": tau_pore, "diffusivity_m2_s": diffusivity,
            "r_soil_s_m": rsoil, "q_surface_kg_kg": q_surface,
            "zero_flux_branch": zero_flux_branch, "rho_air_kg_m3": rho_air},
        "open_resistances": resistance,
        "ground_vapor": {"source_id": configuration["surface_source_id"],
            "q_law_kg_m2_tile_s": law, "q_final_kg_m2_tile_s": final,
            "branch": branch, "request_kg_m2_stand_ground": request * f_t * dt,
            "authorization_kg_m2_stand_ground": authorization,
            "finalized_use_kg_m2_stand_ground": finalized,
            "condensation_credit_kg_m2_stand_ground": max(-final, 0.0) * f_t * dt},
        "terminal_shortwave_by_band_direction_w_m2_tile":
            copy.deepcopy(configuration["surface_terminal_shortwave_by_band_direction_w_m2_tile"]),
        "absorbed_shortwave_by_band_direction_w_m2_tile": absorbed_shortwave,
        "surface_operands_w_m2": {"shortwave": sw, "longwave": longwave,
            "sensible": sensible, "vapor_energy": vapor_energy,
            "ground_heat_cn": gbar[0], "storage": storage}}


def solve_open_bare_soil(bundle: dict[str, Any], cap: dict[str, float] | float | None = None,
                         max_iterations: int = 50, start: list[float] | None = None,
                         max_halvings: int = 20) -> dict[str, Any]:
    beginning_hash = digest(bundle["state"])
    initial = bundle["start"][:] if start is None else start[:]
    result = _solve_normalized_system(
        lambda value: evaluate_open_bare_soil_residual(bundle, value, cap),
        initial, [1.0] * len(initial),
        lambda value: all(math.isfinite(item) and 200.0 <= item <= 350.0 for item in value),
        max_iterations, max_halvings)
    result.update({"pass": "capped" if cap is not None else "potential",
        "beginning_sha256": beginning_hash, "rollback_sha256": beginning_hash,
        "rebuilt_from_beginning_sha256": beginning_hash})
    if result["accepted"]:
        detail = result["detail"]
        result["candidate"] = {"lse": {"surface_enthalpy_j_m2_tile":
            bundle["state"]["surface_enthalpy_j_m2_tile"]
            + detail["surface_operands_w_m2"]["storage"] * bundle["forcing"]["dt_s"],
            "surface_temperature_warm_start_k": detail["surface_temperature_k"]},
            "soil_thermal": {"temperature_k": detail["soil_temperature_k"]},
            "hydrology_mass_mutation": None}
    return result


def _valid_trial(trial: list[float]) -> bool:
    return (all(math.isfinite(value) for value in trial)
            and all(200.0 <= value <= 350.0 for value in trial[6:11])
            and 0.0 <= trial[11] <= 0.1
            and all(200.0 <= value <= 350.0 for value in trial[12:])
            and 0.0 <= trial[4] <= 1.0 and 0.0 <= trial[5] <= 1.0)


def solve_joint(case: dict[str, Any], ground_config: dict[str, Any],
                ground_state: dict[str, Any], caps: dict[str, Any] | None = None,
                max_iterations: int = 50,
                start: list[float] | None = None) -> dict[str, Any]:
    """Solve the exact ordered one-occupancy V8/LSE residual system."""
    beginning_hash = digest({"ground": ground_state, "case": case})
    if start is None:
        start = _default_start(case, ground_state)
    x = start[:]
    backtracking = 0
    last_steps: dict[str, float] | None = None
    pivot = matrix_norm = None
    history: list[float] = []
    for iteration in range(max_iterations + 1):
        raw, detail = _raw_residual(case, ground_config, ground_state, x, caps)
        detail["raw_residuals"] = raw
        normalized = detail["normalized_residuals"]
        norm = max(abs(value) for value in normalized)
        history.append(norm)
        steps_ok = (last_steps is not None and (last_steps["temperature_k"] <= 1.0e-8
            and last_steps["humidity_kg_kg"] <= 1.0e-12
            and last_steps["hydraulic_mm"] <= 1.0e-7
            and last_steps["beta"] <= 1.0e-10))
        if norm <= 1.0 and steps_ok:
            candidate = {"vegetation": {"component_temperatures_k": detail["component_temperatures_k"],
                "canopy_air_temperature_k": detail["canopy_air_temperature_k"],
                "canopy_air_specific_humidity_kg_kg": detail["canopy_air_specific_humidity_kg_kg"],
                "hydraulic_potentials_mm": detail["hydraulic_potentials_mm"],
                "beta_hyd": detail["beta_hyd"]},
                "lse": {"surface_enthalpy_j_m2_tile": ground_state["surface_enthalpy_j_m2_tile"]
                    + detail["surface_storage_w_m2_tile"] * case["dt_s"],
                    "surface_temperature_warm_start_k": detail["ground_temperature_k"]},
                "soil_thermal": {"temperature_k": detail["soil_temperature_k"]},
                "hydrology_mass_mutation": None}
            return {"accepted": True, "pass": "capped" if caps else "potential",
                "iterations": iteration, "solution": x, "residual_norm_history": history,
                "backtracking_count": backtracking, "step_norms": last_steps,
                "pivot_magnitude": pivot, "matrix_norm": matrix_norm,
                "components": detail, "candidate": candidate,
                "beginning_sha256": beginning_hash, "rollback_sha256": beginning_hash}
        if iteration == max_iterations:
            return {"accepted": False, "failure": "iteration_limit", "candidate": None,
                "iterations": iteration, "beginning_sha256": beginning_hash,
                "rollback_sha256": beginning_hash, "residual_norm_history": history,
                "components": detail,
                "diagnostics": {"normalized_residuals": normalized,
                    "backtracking_count": backtracking, "step_norms": last_steps,
                    "active_water_caps": detail["active_water_caps"],
                    "pivot_magnitude": pivot, "matrix_norm": matrix_norm}}
        units = [1000.0] * 4 + [1.0] * 2 + [1.0] * 5 + [0.001]
        units += [1.0] * (len(x) - 12)
        perturb = [math.sqrt(EPSILON) * max(abs(value), unit)
                   for value, unit in zip(x, units, strict=True)]
        frozen = {f"root:{row['layer_id']}": row["branch"] for row in detail["q3"]}
        frozen["ground"] = detail["ground_vapor"]["branch"]
        # The admitted joint residual vector is row-normalized before the
        # mixed-unit Newton system is formed.  This preserves the frozen pivot
        # rule across water (kg m-2 s-1) and energy (W m-2) rows.
        jacobian = [[0.0] * len(x) for _ in raw]
        for column in range(len(x)):
            minus, plus = x[:], x[:]
            minus[column] -= perturb[column]
            plus[column] += perturb[column]
            _, minus_detail = _raw_residual(
                case, ground_config, ground_state, minus, caps, frozen)
            _, plus_detail = _raw_residual(
                case, ground_config, ground_state, plus, caps, frozen)
            for row in range(len(raw)):
                jacobian[row][column] = (
                    plus_detail["normalized_residuals"][row]
                    - minus_detail["normalized_residuals"][row]
                ) / (2.0 * perturb[column])
        try:
            delta, pivot, matrix_norm = V3.solve_linear(
                jacobian, [-value for value in normalized])
        except V3.SingularMatrixError as error:
            return {"accepted": False, "failure": "singular", "candidate": None,
                "iterations": iteration, "beginning_sha256": beginning_hash,
                "rollback_sha256": beginning_hash,
                "residual_norm_history": history,
                "components": detail,
                "diagnostics": {"normalized_residuals": normalized,
                    "backtracking_count": backtracking, "pivot_magnitude": error.pivot,
                    "matrix_norm": error.matrix_norm, "active_water_caps": detail["active_water_caps"]}}
        prospective_steps = {"hydraulic_mm": max(abs(value) for value in delta[:4]),
            "beta": max(abs(value) for value in delta[4:6]),
            "temperature_k": max(abs(value) for value in [*delta[6:11], *delta[12:]]),
            "humidity_kg_kg": abs(delta[11])}
        if norm <= 1.0 and (prospective_steps["temperature_k"] <= 1.0e-8
                and prospective_steps["humidity_kg_kg"] <= 1.0e-12
                and prospective_steps["hydraulic_mm"] <= 1.0e-7
                and prospective_steps["beta"] <= 1.0e-10):
            last_steps = prospective_steps
            continue
        accepted = False
        for exponent in range(21):
            factor = 0.5**exponent
            trial = [value + factor * change for value, change in zip(x, delta, strict=True)]
            if not _valid_trial(trial):
                continue
            try:
                _, trial_detail = _raw_residual(case, ground_config, ground_state, trial, caps)
            except (ValueError, ArithmeticError):
                continue
            trial_norm = max(abs(value) for value in trial_detail["normalized_residuals"])
            if trial_norm < norm:
                applied = [factor * value for value in delta]
                last_steps = {"hydraulic_mm": max(abs(value) for value in applied[:4]),
                    "beta": max(abs(value) for value in applied[4:6]),
                    "temperature_k": max(abs(value) for value in [*applied[6:11], *applied[12:]]),
                    "humidity_kg_kg": abs(applied[11])}
                x = trial
                backtracking += exponent
                accepted = True
                break
        if not accepted:
            return {"accepted": False, "failure": "backtracking_limit", "candidate": None,
                "iterations": iteration, "beginning_sha256": beginning_hash,
                "rollback_sha256": beginning_hash,
                "residual_norm_history": history,
                "components": detail,
                "diagnostics": {"normalized_residuals": normalized,
                    "backtracking_count": backtracking + 20,
                    "pivot_magnitude": pivot,
                "matrix_norm": matrix_norm, "active_water_caps": detail["active_water_caps"]}}
    raise AssertionError("unreachable")


def _typed_numerical_failure(result: dict[str, Any], bundle: dict[str, Any],
                             solve_identity: str) -> dict[str, Any]:
    """Attach the complete immutable identity and ordered failure payload."""
    if result.get("accepted") or result.get("candidate") is not None:
        raise AssertionError("typed numerical failure requires rejected no-candidate result")
    detail = result["components"]
    beta = detail["beta_hyd"]
    active_bounds = [f"beta_hyd:{name}:{value}" for name, value in beta.items()
                     if value == 0.0 or value == 1.0]
    if detail["wet_store_cap_branch"] == "store_cap_active_or_tie":
        active_bounds.append("canopy_liquid_store_cap")
    configuration_sha256 = digest({"case": bundle["case"],
                                   "ground_config": bundle["ground_config"]})
    state_sha256 = digest(bundle["ground_state"])
    ordered = [{"identity": identity, "residual": residual,
        "tolerance": tolerance, "normalized": normalized}
        for identity, residual, tolerance, normalized in zip(
            detail["identities"], detail["raw_residuals"], detail["tolerances"],
            detail["normalized_residuals"], strict=True)]
    result.update({"model_definition_sha256": V8_MODEL_DEFINITION_SHA256,
        "configuration_sha256": configuration_sha256,
        "state_sha256": state_sha256,
        "transaction_id": 73001, "ofe_id": "ofe-1", "tile_id": "tile-a",
        "occupancy_id": "canopy-rank-0", "pass": "potential",
        "solve": solve_identity, "ordered_residuals": ordered,
        "active_bounds": active_bounds,
        "active_water_caps": copy.deepcopy(detail["active_water_caps"])})
    result["diagnostics"].update({"model_definition_sha256": V8_MODEL_DEFINITION_SHA256,
        "configuration_sha256": configuration_sha256,
        "state_sha256": state_sha256,
        "transaction_id": 73001, "ofe_id": "ofe-1", "tile_id": "tile-a",
        "occupancy_id": "canopy-rank-0", "pass": "potential",
        "solve": solve_identity, "iterations": result["iterations"],
        "ordered_residuals": copy.deepcopy(ordered),
        "active_bounds": copy.deepcopy(active_bounds),
        "active_water_caps": copy.deepcopy(detail["active_water_caps"]),
        "ci_brackets_pa": {"sun": detail["sun"]["ci_bracket_pa"],
                            "shade": detail["shade"]["ci_bracket_pa"]}})
    return result


def build_joint_vectors() -> dict[str, Any]:
    """Execute potential, fixed-cap, alternate-start, and failure vectors."""
    bundle = build_joint_case()
    potential = solve_joint(**{key: bundle[key] for key in ("case", "ground_config", "ground_state")},
                            start=bundle["start"])
    if not potential["accepted"]:
        raise AssertionError(f"joint potential fixture failed: {potential}")
    root_caps = {row["layer_id"]: {"cap_rate_kg_m2_tile_s":
        row["q_final_kg_m2_tile_s"] * (0.99 if row["layer_id"] == "soil-1" else 1.0),
        "request_rate_kg_m2_tile_s": row["request_rate_kg_m2_tile_s"]}
        for row in potential["components"]["q3"]}
    ground = potential["components"]["ground_vapor"]
    ground_cap = max(0.0, ground["q_final_kg_m2_tile_s"]) * 0.7
    caps = {"root": root_caps, "ground": {ground["source_id"]: {
        "cap_rate_kg_m2_tile_s": ground_cap,
        "request_rate_kg_m2_tile_s": ground["request_rate_kg_m2_tile_s"]}}}
    capped = solve_joint(bundle["case"], bundle["ground_config"], bundle["ground_state"],
                         caps=caps, start=bundle["start"])
    alternate = bundle["start"][:]
    alternate[:4] = [value + 250.0 for value in alternate[:4]]
    alternate[6:11] = [value + 0.5 for value in alternate[6:11]]
    alternate[12:] = [value - 0.5 for value in alternate[12:]]
    alternate_result = solve_joint(bundle["case"], bundle["ground_config"],
        bundle["ground_state"], caps=caps, start=alternate)
    # Valid physical warm start whose unmodified normalized Jacobian reaches
    # the frozen relative-pivot singularity threshold after one Newton update.
    singular_start = [-5060.058341181588, -3223.3606138445093,
        -8678.098409994316, -2731.151728213228, 0.7828034238440497,
        0.41921750290049625, 294.4386499562707, 291.8527267957731,
        293.0377484776449, 292.56945726878206, 299.21534306450434,
        0.002671465482357275, 288.94930200326996, 296.7197862527845,
        292.4442386140941]
    singular = solve_joint(bundle["case"], bundle["ground_config"],
        bundle["ground_state"], start=singular_start)
    # Positive, valid parameters and warm start that exercise all 50 admitted
    # updates without reaching the joint acceptance criteria.
    limited_case = copy.deepcopy(bundle["case"])
    limited_case["parameters"].update({
        "k1_max": 3.71808736481436e-05,
        "k2_max": 3.952433838191729e-06,
        "k3_max_m_s": 0.0002266759889262188,
    })
    limited_start = [-1898.4133523366827, -7111.481267003401,
        -8204.131337274273, -6094.272125330269, 0.7284837512552559,
        0.66846451641645, 302.0046249862505, 289.265493165604,
        294.81734235838894, 293.6317576401563, 293.04248607317714,
        0.004348547002765208, 288.5191447798437, 287.1176596966161,
        291.94149081359876]
    limited = solve_joint(limited_case, bundle["ground_config"],
        bundle["ground_state"], start=limited_start, max_iterations=50)
    # This valid physical warm start reaches the canonical b=0..20
    # backtracking guard without changing the Newton direction or line-search
    # limit.  It is deliberately frozen as an independent failure vector.
    backtracking_start = [-3823.238728569615, -8119.418303690043,
        -5920.92689913748, -3285.3959407455854, 0.4205270600405538,
        0.7825045940760162, 298.01573552618004, 298.59100122181957,
        293.6906094785682, 288.14926136113905, 293.5534210427704,
        0.007584220145071355, 300.9571667868542, 295.58463623826225,
        292.8342196334298]
    backtracking = solve_joint(bundle["case"], bundle["ground_config"],
        bundle["ground_state"], start=backtracking_start)
    singular = _typed_numerical_failure(
        singular, bundle, "outer_gas_energy_hydraulic_coupling")
    limited_bundle = {**bundle, "case": limited_case}
    limited = _typed_numerical_failure(
        limited, limited_bundle, "outer_gas_energy_hydraulic_coupling")
    backtracking = _typed_numerical_failure(
        backtracking, bundle, "outer_gas_energy_hydraulic_coupling")
    return {"source_checksums": {"v3": V3_SHA256, "v5": V5_SHA256},
        "unknown_order": ["psi_sun", "psi_shade", "psi_stem", "psi_root",
            "beta_sun", "beta_shade", "T_sun", "T_shade", "T_wet", "T_stem",
            "T_canopy_air", "q_canopy_air", "T_ground", "soil_temperatures..."],
        "potential": potential, "fixed_cap": capped,
        "alternate_start_fixed_cap": alternate_result,
        "singular": singular, "backtracking_limit": backtracking,
        "iteration_limit": limited,
        "beginning_state_sha256": digest(bundle["ground_state"]),
        "beginning_state_unchanged_sha256": digest(bundle["ground_state"])}


def build_expanded_joint_vectors() -> dict[str, Any]:
    """Execute reusable covered multirank and arbitrary-layer open reductions."""
    column = build_covered_column_case(2)
    potential = solve_covered_column(column)
    if not potential["accepted"]:
        raise AssertionError(f"covered multirank potential failed: {potential}")
    root_caps: dict[str, dict[str, Any]] = {}
    for occupancy in potential["detail"]["occupancies"]:
        identity = occupancy["occupancy_id"]
        root_caps[identity] = {}
        for row in occupancy["hydraulic_and_component"]["q3"]:
            factor = 0.99 if identity == "canopy-rank-0" and row["layer_id"] == "soil-1" else 1.0
            root_caps[identity][row["layer_id"]] = {
                "cap_rate_kg_m2_tile_s": row["q_final_kg_m2_tile_s"] * factor,
                "request_rate_kg_m2_tile_s": row["request_rate_kg_m2_tile_s"]}
    ground = potential["detail"]["ground"]["ground_vapor"]
    caps = {"root": root_caps, "ground": {ground["source_id"]: {
        "cap_rate_kg_m2_tile_s": max(0.0, ground["q_final_kg_m2_tile_s"]) * 0.92,
        "request_rate_kg_m2_tile_s": ground["request_rate_kg_m2_tile_s"]}}}
    capped = solve_covered_column(column, caps=caps)
    alternate = column["start"][:]
    for index in range(len(column["occupancies"])):
        offset = index * 10
        alternate[offset:offset + 4] = [value + 150.0
                                        for value in alternate[offset:offset + 4]]
        alternate[offset + 6:offset + 10] = [value + 0.35
                                              for value in alternate[offset + 6:offset + 10]]
    alternate_capped = solve_covered_column(column, caps=caps, start=alternate)
    open_bundle = build_open_bare_soil_case(4)
    open_potential = solve_open_bare_soil(open_bundle)
    if not open_potential["accepted"]:
        raise AssertionError(f"open potential failed: {open_potential}")
    open_ground = open_potential["detail"]["ground_vapor"]
    open_cap = {"cap_rate_kg_m2_tile_s":
        max(0.0, open_ground["q_final_kg_m2_tile_s"]) * 0.85,
        "request_rate_kg_m2_tile_s": open_ground["request_kg_m2_stand_ground"]
            / open_bundle["forcing"]["dt_s"]}
    open_capped = solve_open_bare_soil(open_bundle, cap=open_cap)
    open_alternate = [value + (0.4 if index == 0 else -0.3)
                      for index, value in enumerate(open_bundle["start"])]
    open_alternate_capped = solve_open_bare_soil(
        open_bundle, cap=open_cap, start=open_alternate)
    single_rank = build_joint_vectors()
    return {"source_checksums": {"v3": V3_SHA256, "v5": V5_SHA256},
        "covered_single_rank": {
            "potential": single_rank["potential"],
            "fixed_cap_rebuilt_from_beginning": single_rank["fixed_cap"],
            "alternate_warm_start_fixed_cap": single_rank["alternate_start_fixed_cap"]},
        "covered_multirank": {"potential": potential, "caps": caps,
            "fixed_cap_rebuilt_from_beginning": capped,
            "alternate_warm_start_fixed_cap": alternate_capped,
            "terminal_shortwave_by_band_direction_w_m2_tile":
                copy.deepcopy(column["ground_config"]
                    ["ground_terminal_shortwave_by_band_direction_w_m2_tile"])},
        "open_bare_soil_four_layer": {"potential": open_potential,
            "fixed_cap_rebuilt_from_beginning": open_capped,
            "alternate_warm_start_fixed_cap": open_alternate_capped},
        "real_numerical_failures": {key: single_rank[key] for key in
            ("singular", "backtracking_limit", "iteration_limit")},
        "rollback": {"covered_beginning": digest(column["ground_state"]),
            "open_beginning": digest(open_bundle["state"]),
            "no_hydrology_mass_candidate": True}}


def build_frozen_ground_cap_probe() -> dict[str, Any]:
    """Execute the accepted centered equality-cap review regression."""
    column = build_covered_column_case(2)
    potential = solve_covered_column(column)
    if not potential["accepted"]:
        raise AssertionError(f"frozen-cap probe potential failed: {potential}")
    roots: dict[str, dict[str, dict[str, float]]] = {}
    for occupancy in potential["detail"]["occupancies"]:
        identity = occupancy["occupancy_id"]
        roots[identity] = {row["layer_id"]: {
            "cap_rate_kg_m2_tile_s": row["request_rate_kg_m2_tile_s"],
            "request_rate_kg_m2_tile_s": row["request_rate_kg_m2_tile_s"]}
            for row in occupancy["hydraulic_and_component"]["q3"]}
    ground = potential["detail"]["ground"]["ground_vapor"]
    cap = 0.00015581562596770875
    caps = {"root": roots, "ground": {ground["source_id"]: {
        "cap_rate_kg_m2_tile_s": cap,
        "request_rate_kg_m2_tile_s": ground["request_rate_kg_m2_tile_s"]}}}
    final = solve_covered_column(column, caps=caps)
    if not final["accepted"]:
        raise AssertionError(f"frozen-cap probe final failed: {final}")
    frozen = _freeze_covered_branches(final["detail"])
    trial = final["solution"][:]
    ground_temperature_index = 10 * len(column["occupancies"]) + 2
    perturbation = math.sqrt(EPSILON) * max(
        abs(trial[ground_temperature_index]), 1.0)
    trial[ground_temperature_index] -= perturbation
    unfrozen = evaluate_covered_column_residual(column, trial, caps)
    frozen_multirank = evaluate_covered_column_residual(column, trial, caps, frozen)
    prepared_column = _prepare_covered_shortwave(column)
    first = prepared_column["occupancies"][0]
    common = trial[10 * len(column["occupancies"]):]
    first_trial = [*trial[:10], *common]
    first_caps = {"root": caps["root"][first["occupancy_id"]],
                  "ground": caps["ground"]}
    prefix = first["occupancy_id"] + ":"
    first_frozen = {key.removeprefix(prefix): value for key, value
                    in frozen.items() if key.startswith(prefix)}
    first_frozen["ground"] = frozen["ground"]
    frozen_one_rank = evaluate_joint_residual(first["case"],
        prepared_column["ground_config"], prepared_column["ground_state"],
        first_trial, first_caps, first_frozen)
    if (frozen_multirank["ground"]["ground_vapor"]["q_final_kg_m2_tile_s"].hex()
            != cap.hex()
            or frozen_one_rank["ground_vapor"]["q_final_kg_m2_tile_s"].hex()
            != cap.hex()):
        raise AssertionError("frozen equality cap did not remain byte-exact")
    return {"review_probe": {"cap_rate_kg_m2_tile_s": cap,
            "reported_centered_perturbed_law_kg_m2_tile_s":
                0.00015581538850274556,
            "reported_centered_temperature_perturbation_k":
                -4.4045415711362716e-6},
        "executed": {"primitive_input": column, "authorizations": caps,
            "accepted_center": final, "negative_temperature_perturbation_k": -perturbation,
            "unfrozen": unfrozen, "frozen_multirank": frozen_multirank,
            "frozen_one_rank": frozen_one_rank}}


def build_mandatory_scenario_vectors() -> dict[str, Any]:
    """Run the mandatory exact positive scenario and source-guard matrix.

    Every entry owns newly constructed primitive inputs and a fresh solve.  No
    result is aliased from another scenario.  Potential requests are always
    solved before fixed authorizations, and every capped final solve rebuilds
    from the scenario's immutable beginning state.
    """
    def require(result: dict[str, Any], identity: str) -> dict[str, Any]:
        if not result["accepted"]:
            raise AssertionError(f"mandatory scenario {identity} failed: {result}")
        return result

    def covered_record(column: dict[str, Any], identity: str,
                       caps: dict[str, Any] | None = None,
                       start: list[float] | None = None) -> dict[str, Any]:
        executed_start = column["start"] if start is None else start
        initial_residual = evaluate_covered_column_residual(
            column, executed_start, caps)
        result = require(solve_covered_column(column, caps=caps, start=start), identity)
        return {"primitive_input": copy.deepcopy(column),
                "authorizations": copy.deepcopy(caps),
                "initial_residual": initial_residual, "result": result}

    def open_record(bundle: dict[str, Any], identity: str,
                    cap: dict[str, float] | None = None,
                    start: list[float] | None = None) -> dict[str, Any]:
        result = require(solve_open_bare_soil(bundle, cap=cap, start=start), identity)
        return {"primitive_input": copy.deepcopy(bundle),
                "authorization": copy.deepcopy(cap), "result": result}

    def set_covered_surface_beginning(column: dict[str, Any], liquid: float,
                                      temperature: float) -> None:
        state, config = column["ground_state"], column["ground_config"]
        state["surface_liquid_kg_m2_tile"] = liquid
        state["surface_enthalpy_j_m2_tile"] = (
            config["surface_dry_heat_capacity_j_m2_k"] + liquid * CW
        ) * (temperature - TREF)
        state["surface_temperature_warm_start_k"] = temperature
        common_ground_index = 10 * len(column["occupancies"]) + 2
        column["start"][common_ground_index] = temperature

    def zero_shortwave(field_owner: dict[str, Any], field: str) -> None:
        field_owner[field] = {identity: 0.0 for identity in field_owner[field]}

    def covered_caps(potential: dict[str, Any], surface_factor: float,
                     top_layer_factor: float) -> dict[str, Any]:
        if "detail" in potential:
            occupancy_rows = [(item["occupancy_id"],
                item["hydraulic_and_component"]) for item in potential["detail"]["occupancies"]]
            ground = potential["detail"]["ground"]["ground_vapor"]
        else:
            occupancy_rows = [("canopy-rank-0", potential["components"])]
            ground = potential["components"]["ground_vapor"]
        roots: dict[str, dict[str, dict[str, float]]] = {}
        for occupancy_id, detail in occupancy_rows:
            roots[occupancy_id] = {}
            for row in detail["q3"]:
                factor = top_layer_factor if row["layer_id"] == "soil-1" else 1.0
                roots[occupancy_id][row["layer_id"]] = {
                    "cap_rate_kg_m2_tile_s": row["request_rate_kg_m2_tile_s"] * factor,
                    "request_rate_kg_m2_tile_s": row["request_rate_kg_m2_tile_s"],
                }
        return {"root": roots, "ground": {ground["source_id"]: {
            "cap_rate_kg_m2_tile_s": ground["request_rate_kg_m2_tile_s"]
                * surface_factor,
            "request_rate_kg_m2_tile_s": ground["request_rate_kg_m2_tile_s"],
        }}}

    open_day = build_open_bare_soil_case(4)
    open_day_record = open_record(open_day, "open_bare_day")

    open_night = build_open_bare_soil_case(4)
    zero_shortwave(open_night["configuration"],
                   "surface_terminal_shortwave_by_band_direction_w_m2_tile")
    open_night["forcing"].update({"air_temperature_k": 288.0,
        "air_specific_humidity_kg_kg": 0.006,
        "atmospheric_longwave_w_m2": 285.0})
    open_night_record = open_record(open_night, "open_bare_night")

    covered = build_covered_column_case(2)
    covered_record_value = covered_record(covered, "covered_column")

    dry_litter = build_covered_column_case(1)
    set_covered_surface_beginning(dry_litter, 0.0, 295.0)
    dry_litter["start"] = [-8442.933385788541, -5060.8111475066235,
        -3672.6331414093165, -3929.6275254790544, 0.4622921143511482,
        0.5072731410311941, 296.6432503002497, 301.23352445500245,
        296.85595309942346, 294.6853366143918, 296.32478984785837,
        0.006295749639669071, 288.6088220649914, 294.7372029746422,
        295.8150648904064]
    dry_litter_record = covered_record(dry_litter, "dry_litter_covered")

    wet_litter = build_covered_column_case(1)
    set_covered_surface_beginning(wet_litter, 5.0, 295.0)
    wet_litter_record = covered_record(wet_litter, "wet_litter_covered")

    zero_sw = build_open_bare_soil_case(3)
    zero_shortwave(zero_sw["configuration"],
                   "surface_terminal_shortwave_by_band_direction_w_m2_tile")
    zero_sw_record = open_record(zero_sw, "zero_shortwave")

    longwave_cooling = build_open_bare_soil_case(3)
    zero_shortwave(longwave_cooling["configuration"],
                   "surface_terminal_shortwave_by_band_direction_w_m2_tile")
    longwave_cooling["forcing"]["atmospheric_longwave_w_m2"] = 250.0
    longwave_cooling_record = open_record(longwave_cooling, "longwave_cooling")

    lw_feedback_cool = build_covered_column_case(1)
    set_covered_surface_beginning(lw_feedback_cool, 4.0, 289.0)
    lw_feedback_warm = build_covered_column_case(1)
    set_covered_surface_beginning(lw_feedback_warm, 4.0, 301.0)
    lw_feedback = {
        "cool_ground": covered_record(lw_feedback_cool, "lw_feedback_cool_ground"),
        "warm_ground": covered_record(lw_feedback_warm, "lw_feedback_warm_ground"),
    }

    albedo_low = build_covered_column_case(1)
    albedo_low["ground_config"].update({"ground_surface_albedo_vis": 0.04,
                                         "ground_surface_albedo_nir": 0.08})
    albedo_high = build_covered_column_case(1)
    albedo_high["ground_config"].update({"ground_surface_albedo_vis": 0.62,
                                          "ground_surface_albedo_nir": 0.74})
    albedo_lower_boundary_feedback = {
        "low_albedo": covered_record(albedo_low, "low_ground_albedo_boundary"),
        "high_albedo": covered_record(albedo_high, "high_ground_albedo_boundary"),
    }

    feedback_dry = build_covered_column_case(1)
    set_covered_surface_beginning(feedback_dry, 0.0, 295.0)
    feedback_dry["start"] = dry_litter["start"][:]
    feedback_wet = build_covered_column_case(1)
    set_covered_surface_beginning(feedback_wet, 5.0, 301.0)
    sensible_vapor_feedback = {
        "dry_cool_ground": covered_record(
            feedback_dry, "sensible_vapor_feedback_dry_cool"),
        "wet_warm_ground": covered_record(
            feedback_wet, "sensible_vapor_feedback_wet_warm"),
    }

    wet_evaporation = build_covered_column_case(1)
    wet_evaporation["occupancies"][0]["case"]["gas_energy"][
        "canopy_liquid_kg_m2_tile"] = 8.0
    wet_evaporation_record = covered_record(
        wet_evaporation, "wet_canopy_evaporation")

    condensation = build_open_bare_soil_case(4)
    zero_shortwave(condensation["configuration"],
                   "surface_terminal_shortwave_by_band_direction_w_m2_tile")
    condensation["forcing"].update({"air_specific_humidity_kg_kg": 0.018,
                                     "atmospheric_longwave_w_m2": 300.0})
    condensation_record = open_record(condensation, "supported_condensation")

    def capped_covered_record(identity: str, surface_factor: float,
                              top_factor: float) -> dict[str, Any]:
        column = build_covered_column_case(1)
        potential = require(solve_covered_column(column), identity + ":potential")
        caps = covered_caps(potential, surface_factor, top_factor)
        final = require(solve_covered_column(column, caps=caps), identity + ":final")
        return {"primitive_input": copy.deepcopy(column), "potential": potential,
                "authorizations": caps, "final_rebuilt_from_beginning": final}

    full_caps = capped_covered_record("full_surface_top_layer_caps", 1.0, 1.0)
    partial_surface_cap = capped_covered_record("partial_surface_cap", 0.55, 1.0)
    partial_top_layer_cap = capped_covered_record("partial_top_layer_cap", 1.0, 0.55)

    zero_sources = build_covered_column_case(1)
    zero_source_result = covered_record(zero_sources, "dry_frozen_zero_sources")

    redistribution = build_joint_case()
    redistribution["case"]["layers"][0]["soil_potential_mm"] = -20_000.0
    try:
        solve_joint(redistribution["case"], redistribution["ground_config"],
                    redistribution["ground_state"], start=redistribution["start"])
    except V5.RedistributionError as error:
        redistribution_rejection = {
            "primitive_input": copy.deepcopy(redistribution),
            "error_type": type(error).__name__, "message": str(error),
            "layer_id": error.layer_id, "q_law_kg_m2_tile_s": error.q_law,
            "evaluated_preceding_layers": error.evaluated_layers,
            "beginning_sha256": digest({"ground": redistribution["ground_state"],
                                         "case": redistribution["case"]}),
            "rollback_sha256": digest({"ground": redistribution["ground_state"],
                                        "case": redistribution["case"]}),
        }
    else:
        raise AssertionError("mandatory redistribution rejection did not reject")

    heat_positive = build_open_bare_soil_case(4)
    heat_negative = build_open_bare_soil_case(4)
    heat_negative["state"].update({
        "surface_enthalpy_j_m2_tile": heat_negative["configuration"]
            ["surface_dry_heat_capacity_j_m2_k"] * (288.0 - TREF),
        "surface_temperature_warm_start_k": 288.0,
        "soil_temperature_k": [294.0, 293.0, 292.0, 291.0],
    })
    heat_negative["start"] = [288.0, 294.0, 293.0, 292.0, 291.0]
    zero_shortwave(heat_negative["configuration"],
                   "surface_terminal_shortwave_by_band_direction_w_m2_tile")
    heat_negative["forcing"]["atmospheric_longwave_w_m2"] = 300.0
    heat_reversal = {
        "surface_to_soil": open_record(heat_positive, "ground_heat_surface_to_soil"),
        "soil_to_surface": open_record(heat_negative, "ground_heat_soil_to_surface"),
    }

    equilibrium = build_open_bare_soil_case(4)
    equilibrium_temperature = 290.0
    equilibrium["configuration"].update({
        "surface_dry_heat_capacity_j_m2_k": 0.0,
        "surface_storage_branch": "equilibrium_zero",
    })
    equilibrium["state"].update({
        "surface_liquid_kg_m2_tile": 0.0,
        "surface_enthalpy_j_m2_tile": 0.0,
        "surface_temperature_warm_start_k": equilibrium_temperature,
        "soil_temperature_k": [equilibrium_temperature] * 4,
    })
    equilibrium["start"] = [equilibrium_temperature] * 5
    equilibrium["forcing"]["air_temperature_k"] = equilibrium_temperature
    config, top = equilibrium["configuration"], equilibrium["configuration"]["soil_nodes"][0]
    liquid, ice = (equilibrium["state"][key] for key in
                   ("top_layer_liquid_kg_m2", "top_layer_ice_kg_m2"))
    saturation = min(1.0, max(0.01, (liquid / 1000.0 + ice / 917.0)
        / (top["depth_m"] * config["theta_sat"])))
    psi = max(-1.0e8, config["psi_sat_mm"]
              * saturation ** (-config["clapp_hornberger_b"]))
    alpha = math.exp(psi * 9.80665
        / (1000.0 * 461.5 * equilibrium_temperature))
    equilibrium["forcing"].update({
        "air_specific_humidity_kg_kg": alpha * qsat(
            equilibrium_temperature, equilibrium["forcing"]["pressure_pa"]),
        "atmospheric_longwave_w_m2": SIGMA * equilibrium_temperature**4,
    })
    zero_shortwave(equilibrium["configuration"],
                   "surface_terminal_shortwave_by_band_direction_w_m2_tile")
    equilibrium_alternate = copy.deepcopy(equilibrium)
    equilibrium_alternate["state"]["surface_temperature_warm_start_k"] = 315.0
    equilibrium_alternate_start = equilibrium_alternate["start"][:]
    equilibrium_alternate_start[0] = 295.0
    storage_vectors = {
        "equilibrium_zero": open_record(equilibrium, "equilibrium_zero_storage"),
        "equilibrium_zero_alternate_warm_start": open_record(
            equilibrium_alternate, "equilibrium_zero_alternate_warm_start",
            start=equilibrium_alternate_start),
        "transient_nonzero": open_record(
            build_open_bare_soil_case(4), "transient_nonzero_storage"),
    }

    alternate_covered = build_covered_column_case(1)
    alternate_covered_start = alternate_covered["start"][:]
    for rank in range(len(alternate_covered["occupancies"])):
        offset = rank * 10
        alternate_covered_start[offset:offset + 4] = [
            value + 125.0 for value in alternate_covered_start[offset:offset + 4]]
        alternate_covered_start[offset + 6:offset + 10] = [
            value + 0.25 for value in alternate_covered_start[offset + 6:offset + 10]]
    alternate_open = build_open_bare_soil_case(4)
    alternate_open_start = [value + (0.3 if index == 0 else -0.2)
                            for index, value in enumerate(alternate_open["start"])]
    alternate_starts = {
        "covered": covered_record(alternate_covered, "alternate_start_covered",
                                    start=alternate_covered_start),
        "open": open_record(alternate_open, "alternate_start_open",
                            start=alternate_open_start),
    }
    frozen_ground_cap_probe = build_frozen_ground_cap_probe()

    return {"source_checksums": {"v3": V3_SHA256, "v5": V5_SHA256},
        "open_bare_day": open_day_record, "open_bare_night": open_night_record,
        "covered_column": covered_record_value,
        "dry_litter_covered": dry_litter_record,
        "wet_litter_covered": wet_litter_record,
        "zero_shortwave": zero_sw_record,
        "longwave_cooling": longwave_cooling_record,
        "ground_to_canopy_longwave_feedback": lw_feedback,
        "ground_albedo_lower_boundary_feedback": albedo_lower_boundary_feedback,
        "ground_sensible_vapor_feedback": sensible_vapor_feedback,
        "wet_canopy_evaporation": wet_evaporation_record,
        "supported_condensation": condensation_record,
        "full_surface_top_layer_caps": full_caps,
        "partial_surface_cap": partial_surface_cap,
        "partial_top_layer_cap": partial_top_layer_cap,
        "dry_frozen_zero_sources": zero_source_result,
        "hydraulic_redistribution_rejection": redistribution_rejection,
        "ground_heat_sign_reversal": heat_reversal,
        "storage": storage_vectors, "alternate_starts": alternate_starts,
        "frozen_ground_cap_centered_probe": frozen_ground_cap_probe}


if __name__ == "__main__":
    print(json.dumps(build_expanded_joint_vectors(), sort_keys=True,
                     separators=(",", ":"), allow_nan=False))
