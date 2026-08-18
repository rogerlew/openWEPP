#!/usr/bin/env python3
"""Independent joint canopy-ground authority oracle for snow-free LSE V1.

The oracle intentionally shares no implementation with Rust.  It evaluates the
authority equations from primitive configuration, state, and forcing, performs
one immutable-snapshot water arbitration, rebuilds the final solve from the
beginning state under fixed caps, and emits source-resolved owner records.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import math
from pathlib import Path
from typing import Any, Callable

SIGMA = 5.670374419e-8
CW = 4218.0
CP_AIR = 1004.64
TREF = 273.15
DT = 1800.0
ENERGY_TOL = 1.0e-7
MASS_TOL = 1.0e-12
INHERITED_V3_ORACLE_SHA256 = "7b137c1aa9ed0912caf4d14c779eca1819014b4217156d36f98619f06daabd1a"
LSE_V8_JOINT_CORE_SHA256 = "525538f32c91e2377f5d58f72fa4cfff2e81d46d5e12555e79792d92e1e81d6f"
LSE_MODEL_DEFINITION_SHA256 = "e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f"
V8_MODEL_DEFINITION_SHA256 = "622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b"
SCHEMA_SHA256 = {
    "configuration": "6499b98cc1e25f1379bc0ad6052a7536e20c4bfbb9335f9ba5c8de191ae2f009",
    "coupled_transaction": "02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f",
    "diagnostics": "41fb7909d073b4fdf4e59c9fa7da26b9a965ad916688b7867a56525d1bf1460c",
    "forcing": "2138cfbfd69bb7561db6f8e8b995077cd87fa066b49387c18a0252abf820ab70",
    "state": "91243e4087fa2c4775cb3629fe14c64379def4977d3c54a72348ac56d5fa4ee8",
    "water_protocol": "2e5ade752deb0751bb31222da5d8fe3f6a1e5fbee407e20780fa26242a7afd07",
}


def load_joint_core() -> Any:
    """Load the checksum-bound independent joint core once."""
    path = Path(__file__).resolve().parent / "reference_lse_v8_joint_canopy_core.py"
    actual = hashlib.sha256(path.read_bytes()).hexdigest()
    if actual != LSE_V8_JOINT_CORE_SHA256:
        raise RuntimeError("joint canopy-ground authority core checksum mismatch")
    spec = importlib.util.spec_from_file_location("openwepp_joint_canopy_ground_core", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load joint canopy-ground authority core")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


CORE: Any | None = None


def canonical(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False)


def digest(value: Any) -> str:
    return hashlib.sha256(canonical(value).encode()).hexdigest()


def inherited_v3_oracle() -> dict[str, Any]:
    """Execute the immutable independent V3 canopy oracle, never Rust."""
    path = Path(__file__).resolve().parents[2] / "20260812-c3-woody-potential-pass-authority-001" / "artifacts" / "reference_calculator.py"
    if hashlib.sha256(path.read_bytes()).hexdigest() != INHERITED_V3_ORACLE_SHA256:
        raise RuntimeError("inherited V3 authority oracle checksum mismatch")
    spec = importlib.util.spec_from_file_location("openwepp_v3_authority_oracle", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load inherited V3 authority oracle")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    values = module.vectors()
    return {
        "source_path": str(path.relative_to(Path(__file__).resolve().parents[4])),
        "source_sha256": INHERITED_V3_ORACLE_SHA256,
        "radiation": values["families"]["radiation"],
        "aerodynamics": values["families"]["aerodynamics"],
        "hydraulic_potential_pass": values["families"]["hydraulic_potential_pass"],
        "leaf_respiration": values["families"]["leaf_respiration"],
    }


def joint_core_vectors() -> dict[str, Any]:
    path = Path(__file__).resolve().parent / "reference_lse_v8_joint_canopy_core.py"
    actual = hashlib.sha256(path.read_bytes()).hexdigest()
    core = load_joint_core()
    vectors = core.build_joint_vectors()
    vectors["source_path"] = str(path.relative_to(Path(__file__).resolve().parents[4]))
    vectors["source_sha256"] = actual
    return vectors


def latent_vaporization(t_k: float) -> float:
    return 2_501_000.0 - 2369.0 * (t_k - TREF)


def liquid_enthalpy(t_k: float) -> float:
    return CW * (t_k - TREF)


def qsat(t_k: float, pressure_pa: float) -> float:
    tc = t_k - TREF
    es = 611.2 * math.exp(17.67 * tc / (tc + 243.5))
    return 0.622 * es / (pressure_pa - 0.378 * es)


def open_neutral_resistances(tile: dict[str, Any], wind_m_s: float) -> tuple[float, float]:
    kappa = 0.4
    zref = tile["reference_height_m"]
    z0m, z0h, z0q = tile["roughness_momentum_m"], tile["roughness_heat_m"], tile["roughness_vapor_m"]
    if not (wind_m_s > 0.0 and zref > max(z0m, z0h, z0q) > 0.0):
        raise ValueError("open_neutral_geometry_domain")
    common = math.log(zref / z0m) / (kappa * kappa * wind_m_s)
    return common * math.log(zref / z0h), common * math.log(zref / z0q)


def _conductance(depth_a: float, conductivity_a: float,
                 depth_b: float, conductivity_b: float) -> float:
    return 2.0 / (depth_a / conductivity_a + depth_b / conductivity_b)


def _open_trial_valid(trial: list[float]) -> bool:
    return all(math.isfinite(value) and 200.0 <= value <= 350.0 for value in trial)


def _open_raw_residual(
    forcing: dict[str, Any],
    config: dict[str, Any],
    state: dict[str, Any],
    trial: list[float],
    cap_rate: float | None,
    frozen_cap_active: bool | None = None,
) -> tuple[list[float], dict[str, Any]]:
    """Canonical open-surface reduction of the admitted coupled system.

    The only unknowns are the surface temperature and ordered soil-node
    temperatures.  There is no synthetic canopy-air node.  All coefficients,
    CN interface fluxes, water branches, and tolerance scales are recomputed
    from the current trial.
    """
    nodes = config["soil_nodes"]
    if len(trial) != 1 + len(nodes) or not _open_trial_valid(trial):
        raise ValueError("open_trial_identity_or_domain")
    ts, *soil_t = trial
    interval = forcing["interval_s"]
    pressure = forcing["air_pressure_pa"]
    rho = pressure / (287.05 * forcing["air_temperature_k"])
    rh, rv = open_neutral_resistances(config, forcing["reference_wind_m_s"])
    qair = forcing["air_specific_humidity_kg_kg"]
    water0 = state["surface_liquid_kg_m2_tile"]
    if config["surface_class"] == "bare_mineral_soil" and water0 == 0.0:
        law, humidity_detail = bare_soil_vapor(
            config, state, ts, qair, pressure, rv, rho)
    else:
        if config["surface_class"] == "forest_litter":
            capacity = config["litter_capacity_kg_m2_tile"]
            humidity = 0.5 * (1.0 - math.cos(math.pi * min(water0, capacity) / capacity))
        else:
            humidity = 1.0
        qsurface = humidity * qsat(ts, pressure) + (1.0 - humidity) * qair
        law = rho * (qsurface - qair) / rv
        humidity_detail = {"surface_relative_humidity": humidity,
                           "q_surface_kg_kg": qsurface}
    request_rate = max(0.0, law)
    cap_active = law >= 0.0 and cap_rate is not None and cap_rate <= law
    if frozen_cap_active is not None:
        cap_active = frozen_cap_active
    vapor = cap_rate if cap_active and cap_rate is not None else law
    uses_surface_store = not (
        config["surface_class"] == "bare_mineral_soil" and water0 == 0.0)
    if uses_surface_store and vapor >= 0.0 and vapor * interval > water0 + 1.0e-14:
        raise ValueError("surface_source_inventory_domain")
    water1 = (water0 - max(vapor, 0.0) * interval + max(-vapor, 0.0) * interval
              if uses_surface_store else water0)
    sw_by_band = config["terminal_shortwave_w_m2_tile"]
    absorbed_by_band = {
        key: value * (1.0 - (config["surface_vis_albedo"] if key.endswith("vis")
                            else config["surface_nir_albedo"]))
        for key, value in sw_by_band.items()
    }
    reflected_by_band = {key: sw_by_band[key] - absorbed_by_band[key]
                         for key in sw_by_band}
    shortwave = math.fsum(absorbed_by_band.values())
    longwave = forcing["atmospheric_downward_longwave_w_m2"] - (
        config["surface_emissivity"] * SIGMA * ts**4)
    sensible = rho * CP_AIR * (ts - forcing["air_temperature_k"]) / rh
    latent = vapor * (latent_vaporization(ts) + liquid_enthalpy(ts))
    dry_capacity = config["surface_dry_heat_capacity_j_m2_k"]
    if dry_capacity == 0.0 and water0 == 0.0 and state["surface_enthalpy_j_m2_tile"] == 0.0:
        storage = 0.0
        ending_enthalpy = 0.0
        storage_mode = "equilibrium_zero"
    else:
        ending_capacity = dry_capacity + water1 * CW
        ending_enthalpy = ending_capacity * (ts - TREF)
        storage = (ending_enthalpy - state["surface_enthalpy_j_m2_tile"]) / interval
        storage_mode = "enthalpy_prognostic"
    begin_surface = (ts if storage_mode == "equilibrium_zero" else
        TREF + state["surface_enthalpy_j_m2_tile"] / (dry_capacity + water0 * CW))
    g_end = [_conductance(config["surface_depth_m"],
        config["surface_conductivity_w_m_k"], nodes[0]["depth_m"],
        nodes[0]["conductivity_w_m_k"]) * (ts - soil_t[0])]
    g_begin = [_conductance(config["surface_depth_m"],
        config["surface_conductivity_w_m_k"], nodes[0]["depth_m"],
        nodes[0]["conductivity_w_m_k"]) * (begin_surface - state["soil_temperature_k"][0])]
    for index in range(len(nodes) - 1):
        conductance = _conductance(nodes[index]["depth_m"], nodes[index]["conductivity_w_m_k"],
            nodes[index + 1]["depth_m"], nodes[index + 1]["conductivity_w_m_k"])
        g_end.append(conductance * (soil_t[index] - soil_t[index + 1]))
        g_begin.append(conductance * (state["soil_temperature_k"][index]
                                      - state["soil_temperature_k"][index + 1]))
    g_cn = [0.5 * (old + new) for old, new in zip(g_begin, g_end, strict=True)]
    surface_terms = [shortwave, longwave, -sensible, -latent, -g_cn[0], -storage]
    raw = [math.fsum(surface_terms)]
    scales = [max(1.0, math.fsum(abs(value) for value in surface_terms))]
    for index, node in enumerate(nodes):
        incoming = g_cn[index]
        outgoing = g_cn[index + 1] if index + 1 < len(g_cn) else 0.0
        node_storage = node["heat_capacity_j_m2_k"] * (
            soil_t[index] - state["soil_temperature_k"][index]) / interval
        raw.append(incoming - outgoing - node_storage)
        scales.append(max(1.0, abs(incoming) + abs(outgoing) + abs(node_storage)))
    tolerances = [CORE.ENERGY_ATOL + CORE.ENERGY_RTOL * scale for scale in scales]
    return raw, {
        "identities": ["ground_surface_energy", *(
            f"soil_thermal:{node['layer_id']}" for node in nodes)],
        "normalized_residuals": [value / tolerance for value, tolerance in zip(raw, tolerances, strict=True)],
        "tolerances": tolerances,
        "surface_temperature_k": ts,
        "soil_temperature_k": soil_t,
        "surface_vapor_law_kg_m2_tile_s": law,
        "surface_vapor_final_kg_m2_tile_s": vapor,
        "surface_request_rate_kg_m2_tile_s": request_rate,
        "cap_active": cap_active,
        "humidity": humidity_detail,
        "shortwave": {"terminal_by_band_direction_w_m2_tile": sw_by_band,
                      "absorbed_by_band_direction_w_m2_tile": absorbed_by_band,
                      "reflected_to_canopy_by_band_direction_w_m2_tile": reflected_by_band,
                      "closure_w_m2_tile": math.fsum(sw_by_band.values())
                      - math.fsum(absorbed_by_band.values()) - math.fsum(reflected_by_band.values())},
        "longwave_net_w_m2_tile": longwave,
        "sensible_w_m2_tile": sensible,
        "latent_w_m2_tile": latent,
        "ground_heat_cn_w_m2_tile": g_cn,
        "surface_storage_w_m2_tile": storage,
        "surface_storage_mode": storage_mode,
        "ending_surface_enthalpy_j_m2_tile": ending_enthalpy,
        "ending_surface_liquid_before_ingress_kg_m2_tile": water1,
    }


def solve_open_surface(
    forcing: dict[str, Any], config: dict[str, Any], state: dict[str, Any],
    cap_rate: float | None = None, start: list[float] | None = None,
    max_iterations: int = 50,
) -> dict[str, Any]:
    """Frozen scaled Newton solve for the exact open reduction."""
    beginning_hash = digest({"configuration": config, "state": state})
    x = (start[:] if start is not None else
         [state["surface_temperature_warm_start_k"], *state["soil_temperature_k"]])
    last_steps: dict[str, float] | None = None
    backtracking = 0
    pivot = matrix_norm = None
    history: list[float] = []
    for iteration in range(max_iterations + 1):
        _, detail = _open_raw_residual(forcing, config, state, x, cap_rate)
        normalized = detail["normalized_residuals"]
        norm = max(abs(value) for value in normalized)
        history.append(norm)
        if norm <= 1.0 and last_steps is not None \
                and last_steps["temperature_k"] <= 1.0e-8:
            return {"accepted": True, "pass": "capped" if cap_rate is not None else "potential",
                "iterations": iteration, "solution": x, "components": detail,
                "candidate": {"land_surface_energy": {
                    "surface_enthalpy_j_m2_tile": detail["ending_surface_enthalpy_j_m2_tile"],
                    "surface_temperature_warm_start_k": detail["surface_temperature_k"]},
                    "soil_thermal": {"temperature_k": detail["soil_temperature_k"]}},
                "residual_norm_history": history, "backtracking_count": backtracking,
                "step_norms": last_steps, "pivot_magnitude": pivot,
                "matrix_norm": matrix_norm, "beginning_sha256": beginning_hash,
                "rollback_sha256": beginning_hash}
        if iteration == max_iterations:
            return {"accepted": False, "failure": "iteration_limit", "candidate": None,
                "iterations": iteration, "diagnostics": {"normalized_residuals": normalized,
                    "backtracking_count": backtracking, "step_norms": last_steps,
                    "pivot_magnitude": pivot, "matrix_norm": matrix_norm},
                "beginning_sha256": beginning_hash, "rollback_sha256": beginning_hash}
        perturb = [math.sqrt(CORE.EPSILON) * max(abs(value), 1.0) for value in x]
        frozen = detail["cap_active"]
        jacobian = [[0.0] * len(x) for _ in x]
        for column in range(len(x)):
            minus, plus = x[:], x[:]
            minus[column] -= perturb[column]
            plus[column] += perturb[column]
            _, md = _open_raw_residual(forcing, config, state, minus, cap_rate, frozen)
            _, pd = _open_raw_residual(forcing, config, state, plus, cap_rate, frozen)
            for row in range(len(x)):
                jacobian[row][column] = (pd["normalized_residuals"][row]
                    - md["normalized_residuals"][row]) / (2.0 * perturb[column])
        try:
            delta, pivot, matrix_norm = CORE.V3.solve_linear(jacobian, [-value for value in normalized])
        except CORE.V3.SingularMatrixError as error:
            return {"accepted": False, "failure": "singular", "candidate": None,
                "iterations": iteration, "diagnostics": {"normalized_residuals": normalized,
                    "backtracking_count": backtracking, "pivot_magnitude": error.pivot,
                    "matrix_norm": error.matrix_norm}, "beginning_sha256": beginning_hash,
                "rollback_sha256": beginning_hash}
        accepted = False
        for exponent in range(21):
            factor = 0.5**exponent
            trial = [value + factor * change for value, change in zip(x, delta, strict=True)]
            if not _open_trial_valid(trial):
                continue
            try:
                _, td = _open_raw_residual(forcing, config, state, trial, cap_rate)
            except (ValueError, ArithmeticError):
                continue
            if max(abs(value) for value in td["normalized_residuals"]) < norm:
                applied = [factor * value for value in delta]
                last_steps = {"temperature_k": max(abs(value) for value in applied)}
                x = trial
                backtracking += exponent
                accepted = True
                break
        if not accepted:
            return {"accepted": False, "failure": "backtracking_limit", "candidate": None,
                "iterations": iteration, "diagnostics": {"normalized_residuals": normalized,
                    "backtracking_count": backtracking + 20, "pivot_magnitude": pivot,
                    "matrix_norm": matrix_norm}, "beginning_sha256": beginning_hash,
                "rollback_sha256": beginning_hash}
    raise AssertionError("unreachable")


def bare_soil_vapor(tile: dict[str, Any], state: dict[str, Any], surface_t: float,
                    recipient_q: float, pressure_pa: float, aerodynamic_rv: float,
                    moist_air_density_kg_m3: float) -> tuple[float, dict[str, float]]:
    rho_w, rho_i = 1000.0, 917.0
    saturation = min(1.0, max(0.01,
        (state["top_layer_liquid_kg_m2"] / rho_w + state["top_layer_ice_kg_m2"] / rho_i)
        / (tile["top_layer_depth_m"] * tile["soil_porosity"])))
    theta = state["top_layer_liquid_kg_m2"] / (rho_w * tile["top_layer_depth_m"])
    psi = max(-1.0e8, tile["soil_saturated_matric_potential_mm"]
              * saturation ** (-tile["soil_clapp_hornberger_b"]))
    alpha = math.exp(psi * 9.80665 / (1000.0 * 461.5 * surface_t))
    theta_air = tile["soil_porosity"] * (
        tile["soil_saturated_matric_potential_mm"] / -1.0e7
    ) ** (1.0 / tile["soil_clapp_hornberger_b"])
    dry_layer = (0.015 * (tile["soil_theta_initial"] - theta)
                 / (tile["soil_theta_initial"] - theta_air)
                 if theta < tile["soil_theta_initial"] else 0.0)
    pore_air = tile["soil_porosity"] - theta_air
    tortuosity = pore_air**2 * (pore_air / tile["soil_porosity"]) ** (
        3.0 / tile["soil_clapp_hornberger_b"])
    vapor_diffusivity = 2.12e-5 * (surface_t / TREF) ** 1.75
    soil_r = dry_layer / (vapor_diffusivity * tortuosity)
    saturated_q = qsat(surface_t, pressure_pa)
    soil_q = alpha * saturated_q
    if saturated_q > recipient_q > soil_q:
        soil_q = recipient_q
    flux = moist_air_density_kg_m3 * (soil_q - recipient_q) / (aerodynamic_rv + soil_r)
    return flux, {"saturation": saturation, "theta": theta, "psi_mm": psi,
        "alpha": alpha, "theta_air": theta_air, "dry_layer_m": dry_layer,
        "tortuosity": tortuosity, "vapor_diffusivity_m2_s": vapor_diffusivity,
        "soil_resistance_s_m": soil_r, "q_soil_kg_kg": soil_q}


def water_key(
    *, transaction_id: int, owner_id: str, component: str, ofe_id: str,
    requesting_tile_id: str, source_type: str, source_id: str,
    occupancy_id: str | None = None, surface_id: str | None = None,
    surface_class: str | None = None, source_tile_id: str | None = None,
    soil_layer_id: str | None = None,
) -> dict[str, Any]:
    """Construct the complete strict water identity; no string-key shortcut."""
    key = {
        "transaction_id": transaction_id,
        "requesting_owner_id": owner_id,
        "requesting_component": component,
        "ofe_id": ofe_id,
        "requesting_tile_id": requesting_tile_id,
        "occupancy_id": occupancy_id,
        "surface_id": surface_id,
        "surface_class": surface_class,
        "source_type": source_type,
        "source_id": source_id,
        "source_tile_id": source_tile_id,
        "soil_layer_id": soil_layer_id,
        "amount_basis": "kg_h2o_m-2_stand_ground_interval",
    }
    if component == "vegetation_root":
        if not occupancy_id or surface_id is not None or surface_class is not None \
                or source_type != "soil_layer_liquid" or source_tile_id is not None \
                or not soil_layer_id:
            raise ValueError("water_root_key_identity")
    elif component == "ground_surface":
        if occupancy_id is not None or not surface_id or not surface_class:
            raise ValueError("water_ground_key_identity")
        if source_type == "soil_layer_liquid":
            if source_tile_id is not None or not soil_layer_id:
                raise ValueError("water_ground_soil_key_identity")
        elif source_type in {"surface_liquid", "litter_liquid"}:
            if source_tile_id is None or soil_layer_id is not None:
                raise ValueError("water_ground_surface_key_identity")
        else:
            raise ValueError("water_source_type_identity")
    else:
        raise ValueError("water_requesting_component_identity")
    return key


def resource_group_identity(key: dict[str, Any]) -> tuple[Any, ...]:
    """Hydrology groups only exact OFE + physical source/layer inventory."""
    return (key["ofe_id"], key["source_type"], key["source_id"],
            key["source_tile_id"], key["soil_layer_id"])


def arbitrate(requests: list[dict[str, Any]], stores: dict[tuple[Any, ...], float]) -> list[dict[str, Any]]:
    """One immutable beginning-snapshot, exact-key, proportional arbitration."""
    seen: set[str] = set()
    grouped: dict[tuple[Any, ...], list[dict[str, Any]]] = {}
    for request in requests:
        identity = canonical(request["key"])
        if identity in seen:
            raise ValueError("duplicate_request_identity")
        seen.add(identity)
        if request["amount_kg_m2_stand_ground"] < 0.0 or not math.isfinite(
                request["amount_kg_m2_stand_ground"]):
            raise ValueError("water_request_amount_domain")
        grouped.setdefault(resource_group_identity(request["key"]), []).append(request)
    result: list[dict[str, Any]] = []
    for source in sorted(grouped, key=canonical):
        competitors = grouped[source]
        total = math.fsum(item["amount_kg_m2_stand_ground"] for item in competitors)
        supply = stores.get(source, 0.0)
        ratio = 1.0 if total <= supply else (supply / total if total else 0.0)
        reason = "full_supply" if ratio == 1.0 else ("zero_supply" if supply == 0.0 else "proportional_supply")
        for request in competitors:
            result.append({"key": request["key"],
                "amount_kg_m2_stand_ground": request["amount_kg_m2_stand_ground"] * ratio,
                "reason": reason})
    return sorted(result, key=lambda value: canonical(value["key"]))


def validate_water_protocol(protocol: dict[str, Any]) -> dict[str, Any]:
    """Independently validate exact D/A/F identities and bounds."""
    transaction = protocol["transaction_id"]
    indexed: dict[str, dict[str, dict[str, Any]]] = {}
    for family in ("requests", "authorizations", "finalized_uses"):
        current: dict[str, dict[str, Any]] = {}
        for record in protocol[family]:
            if record["key"]["transaction_id"] != transaction:
                raise ValueError("water_mixed_transaction")
            identity = canonical(record["key"])
            if identity in current:
                raise ValueError(f"water_duplicate_identity:{family}")
            amount = record["amount_kg_m2_stand_ground"]
            if not math.isfinite(amount) or amount < 0.0:
                raise ValueError(f"water_amount_domain:{family}")
            current[identity] = record
        indexed[family] = current
    if set(indexed["authorizations"]) != set(indexed["requests"]) \
            or set(indexed["finalized_uses"]) != set(indexed["requests"]):
        raise ValueError("water_daf_identity_set")
    bounds = []
    for identity in sorted(indexed["requests"]):
        request = indexed["requests"][identity]["amount_kg_m2_stand_ground"]
        authorization = indexed["authorizations"][identity]["amount_kg_m2_stand_ground"]
        finalized = indexed["finalized_uses"][identity]["amount_kg_m2_stand_ground"]
        if authorization > request or finalized > authorization:
            raise ValueError(f"water_daf_bound:{identity}:{finalized}:{authorization}:{request}")
        bounds.append({"key": indexed["requests"][identity]["key"],
            "demand_kg_m2_stand_ground": request,
            "authorization_kg_m2_stand_ground": authorization,
            "finalized_use_kg_m2_stand_ground": finalized})
    return {"validated": True, "bounds": bounds,
        "protocol_sha256": digest(protocol)}


def reconstruct_water_stores(protocol: dict[str, Any],
                             beginning_stores: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Independently debit finalized uses and credit typed condensation."""
    validate_water_protocol(protocol)
    stores = {tuple(row["resource_identity"]): row["amount_kg_m2_stand_ground"]
        for row in beginning_stores}
    if len(stores) != len(beginning_stores):
        raise ValueError("water_beginning_store_duplicate")
    uses = {resource: 0.0 for resource in stores}
    credits = {resource: 0.0 for resource in stores}
    for row in protocol["finalized_uses"]:
        resource = resource_group_identity(row["key"])
        if resource not in stores:
            raise ValueError("water_finalized_source_missing")
        uses[resource] += row["amount_kg_m2_stand_ground"]
    seen_credits: set[str] = set()
    for row in protocol.get("condensation_credits", []):
        if row["transaction_id"] != protocol["transaction_id"] \
                or row["hydrology_owner_id"] != protocol["hydrology_owner_id"]:
            raise ValueError("condensation_credit_transaction_owner")
        matching_keys = [request["key"] for request in protocol["requests"]
            if request["key"]["ofe_id"] == row["ofe_id"]
            and request["key"]["requesting_tile_id"] == row["tile_id"]
            and request["key"]["surface_id"] == row["surface_id"]]
        if len(matching_keys) != 1:
            raise ValueError("condensation_credit_key_resolution")
        resolved_key = matching_keys[0]
        identity = canonical({"transaction_id": row["transaction_id"],
            "hydrology_owner_id": row["hydrology_owner_id"],
            "ofe_id": row["ofe_id"], "tile_id": row["tile_id"],
            "surface_id": row["surface_id"]})
        if identity in seen_credits:
            raise ValueError("condensation_credit_duplicate")
        seen_credits.add(identity)
        resource = resource_group_identity(resolved_key)
        if resource not in stores:
            raise ValueError("condensation_credit_source_missing")
        amount = row["amount_kg_m2_stand_ground"]
        if not math.isfinite(amount) or amount <= 0.0:
            raise ValueError("condensation_credit_amount_domain")
        credits[resource] += amount
    ending = []
    for resource in sorted(stores, key=canonical):
        amount = stores[resource] - uses[resource] + credits[resource]
        if amount < -MASS_TOL:
            raise ValueError("water_source_overdraw")
        ending.append({"resource_identity": list(resource),
            "beginning_kg_m2_stand_ground": stores[resource],
            "finalized_use_kg_m2_stand_ground": uses[resource],
            "condensation_credit_kg_m2_stand_ground": credits[resource],
            "ending_kg_m2_stand_ground": amount,
            "mass_residual_kg_m2_stand_ground": stores[resource]
                - uses[resource] + credits[resource] - amount})
    return ending


def apply_post_ingress(
    *, transaction_id: int, ofe_id: str, tile_id: str,
    tile_fraction: float, beginning_mass_tile: float,
    pre_ingress_mass_tile: float, pre_ingress_surface_enthalpy_j_m2_tile: float,
    dry_capacity_j_m2_k: float, parcels: list[dict[str, Any]],
    infiltration_kg_m2_tile: float, runoff_kg_m2_tile: float,
) -> dict[str, Any]:
    """Apply accepted ingress after the fixed-cap solve and retain its energy.

    Hydrology supplies the already accepted infiltration/runoff partition.  The
    oracle does not recreate hydrology science with local min arithmetic.
    """
    if not (0.0 < tile_fraction <= 1.0):
        raise ValueError("tile_fraction_domain")
    ingress_mass = math.fsum(parcel["amount_kg_m2_destination_tile_ground"] for parcel in parcels)
    ingress_energy = math.fsum(parcel["amount_kg_m2_destination_tile_ground"]
        * parcel["specific_liquid_enthalpy_j_kg"] for parcel in parcels)
    if dry_capacity_j_m2_k < 0.0 or pre_ingress_mass_tile < 0.0:
        raise ValueError("surface_thermal_state_domain")
    pre_energy = pre_ingress_surface_enthalpy_j_m2_tile
    mixed_mass = pre_ingress_mass_tile + ingress_mass
    if infiltration_kg_m2_tile < 0.0 or runoff_kg_m2_tile < 0.0 \
            or infiltration_kg_m2_tile + runoff_kg_m2_tile > mixed_mass:
        raise ValueError("accepted_hydrology_partition_bound")
    total_capacity = dry_capacity_j_m2_k + mixed_mass * CW
    if total_capacity <= 0.0:
        raise ValueError("post_ingress_zero_thermal_capacity")
    mixed_temperature = TREF + (pre_energy + ingress_energy) / total_capacity
    mixed_specific = liquid_enthalpy(mixed_temperature)
    retained_mass = mixed_mass - infiltration_kg_m2_tile - runoff_kg_m2_tile
    retained_liquid_energy = retained_mass * mixed_specific
    ending_surface_enthalpy = dry_capacity_j_m2_k * (mixed_temperature - TREF) \
        + retained_liquid_energy
    crossings = [
        {"transaction_id": transaction_id, "ofe_id": ofe_id, "tile_id": tile_id,
         "process": "infiltration", "amount_kg_m2_tile_ground": infiltration_kg_m2_tile,
         "specific_liquid_enthalpy_j_kg": mixed_specific,
         "amount_j_m2_tile_ground": infiltration_kg_m2_tile * mixed_specific,
         "mass_owner": "hydrology", "thermal_receiver": "soil_thermal"},
        {"transaction_id": transaction_id, "ofe_id": ofe_id, "tile_id": tile_id,
         "process": "runoff", "amount_kg_m2_tile_ground": runoff_kg_m2_tile,
         "specific_liquid_enthalpy_j_kg": mixed_specific,
         "amount_j_m2_tile_ground": runoff_kg_m2_tile * mixed_specific,
         "mass_owner": "hydrology", "thermal_receiver": "accepted_downstream_or_outlet"},
    ]
    mass_residual = pre_ingress_mass_tile + ingress_mass - infiltration_kg_m2_tile \
        - runoff_kg_m2_tile - retained_mass
    ending_dry_energy = dry_capacity_j_m2_k * (mixed_temperature - TREF)
    energy_residual = pre_energy + ingress_energy - math.fsum(
        record["amount_j_m2_tile_ground"] for record in crossings) \
        - ending_dry_energy - retained_liquid_energy
    return {"beginning_mass_kg_m2_tile_ground": beginning_mass_tile,
        "pre_ingress_mass_kg_m2_tile_ground": pre_ingress_mass_tile,
        "ingress_mass_kg_m2_tile_ground": ingress_mass,
        "ingress_energy_j_m2_tile_ground": ingress_energy,
        "mixed_temperature_k": mixed_temperature,
        "ending_mass_kg_m2_tile_ground": retained_mass,
        "pre_ingress_surface_enthalpy_j_m2_tile_ground": pre_energy,
        "ending_dry_body_enthalpy_j_m2_tile_ground": ending_dry_energy,
        "ending_liquid_enthalpy_j_m2_tile_ground": retained_liquid_energy,
        "ending_surface_enthalpy_j_m2_tile_ground": ending_surface_enthalpy,
        "crossings": crossings, "mass_residual_kg_m2_tile_ground": mass_residual,
        "energy_residual_j_m2_tile_ground": energy_residual,
        "soil_thermal_infiltration_receipt_j_m2_stand_ground":
            tile_fraction * crossings[0]["amount_j_m2_tile_ground"]}


def typed_owner_beginning(owner_id: str, model_version: str,
                          payload: dict[str, Any]) -> dict[str, Any]:
    model_definition = (V8_MODEL_DEFINITION_SHA256 if model_version == "OPENWEPP_C3_WOODY_V8"
        else LSE_MODEL_DEFINITION_SHA256 if model_version == "OPENWEPP_SNOW_FREE_LSE_V1"
        else digest({"model_version": model_version}))
    configuration = digest({"owner_id": owner_id, "model_version": model_version,
        "configuration": "authority-vector"})
    return {"owner_id": owner_id, "model_version": model_version,
        "model_definition_sha256": model_definition,
        "configuration_sha256": configuration, "state": payload,
        "state_sha256": digest(payload)}


def reconstruct_owner_endings(beginning: dict[str, Any], water: dict[str, Any],
                              joins: dict[str, Any]) -> dict[str, Any]:
    """Reconstruct all five endings from beginning state and primitive operands."""
    forbidden = {"ending_state", "candidate_state_sha256",
        "vegetation_proposals_sha256", "bgc_receipts_sha256"}
    if forbidden.intersection(joins) or any(forbidden.intersection(value)
            for value in joins.values() if isinstance(value, dict)):
        raise ValueError("copied_candidate_or_hash_join_prohibited")
    vegetation_operands = joins["vegetation_component_operands"]
    occupancies = []
    seen = set()
    for operand in vegetation_operands:
        occupancy_id = operand["occupancy_id"]
        if occupancy_id in seen:
            raise ValueError("duplicate_vegetation_occupancy_operand")
        seen.add(occupancy_id)
        occupancies.append({"occupancy_id": occupancy_id,
            "component_temperatures_k": {
                name: operand["component_temperatures_k"][name]
                for name in ("sun_leaf", "shade_leaf", "wet_surface", "dry_stem")},
            "hydraulic_potentials_mm": {
                name: operand["hydraulic_potentials_mm"][name]
                for name in ("sun_leaf", "shade_leaf", "stem", "root")}})
    material_operands = joins["material_operands"]
    proposals = material_operands["vegetation_proposals"]
    if not proposals:
        raise ValueError("empty_material_join_tautology")
    receipts = []
    proposal_ids = set()
    for proposal in proposals:
        if proposal["proposal_id"] in proposal_ids or any(proposal[field] <= 0.0
                for field in ("carbon_kg_m2_stand_ground",
                    "nitrogen_kg_m2_stand_ground", "dry_material_kg_m2_stand_ground")):
            raise ValueError("material_proposal_identity_or_amount")
        proposal_ids.add(proposal["proposal_id"])
        receipts.append({"transaction_id": proposal["transaction_id"],
            "proposal_id": proposal["proposal_id"],
            "receiver": proposal["receiver"],
            "carbon_kg_m2_stand_ground": proposal["carbon_kg_m2_stand_ground"],
            "nitrogen_kg_m2_stand_ground": proposal["nitrogen_kg_m2_stand_ground"],
            "dry_material_kg_m2_stand_ground": proposal[
                "dry_material_kg_m2_stand_ground"]})
    if material_operands.get("bgc_constructed_receipts") != receipts:
        raise ValueError("material_receipt_independent_reconstruction")
    advection = joins["advection_operands"]
    ingress = apply_post_ingress(**copy.deepcopy(advection))
    resource_ledger = reconstruct_water_stores(water,
        beginning["hydrology"]["state"]["resource_stores"])
    soil_operands = joins["soil_thermal_operands"]
    if len(soil_operands["layers"]) != len(
            beginning["soil_thermal"]["state"]["temperatures_k"]):
        raise ValueError("soil_thermal_layer_cardinality")
    infiltration_receipt = soil_operands[
        "infiltration_enthalpy_receipt_j_m2_stand_ground"]
    if soil_operands.get("infiltration_receiver_layer_id") \
            != soil_operands["layers"][0]["layer_id"]:
        raise ValueError("infiltration_enthalpy_wrong_soil_node")
    if soil_operands.get("infiltration_enthalpy_receipt_basis") \
            != "j_m2_stand_ground_interval":
        raise ValueError("infiltration_enthalpy_wrong_area_basis")
    if not math.isclose(infiltration_receipt,
            ingress["soil_thermal_infiltration_receipt_j_m2_stand_ground"],
            rel_tol=0.0, abs_tol=ENERGY_TOL):
        raise ValueError("infiltration_enthalpy_receipt_join")
    soil_temperatures = []
    ground_receipts = []
    for index, (beginning_temperature, layer) in enumerate(zip(
            beginning["soil_thermal"]["state"]["temperatures_k"],
            soil_operands["layers"], strict=True)):
        ground_receipts.append(layer["ground_heat_receipt_j_m2_stand_ground"])
        accepted_energy = layer["ground_heat_receipt_j_m2_stand_ground"] \
            + (infiltration_receipt if index == 0 else 0.0)
        soil_temperatures.append(beginning_temperature
            + accepted_energy
            / (layer["areal_heat_capacity_j_m2_k"] * layer["tile_fraction"]))
    ground_heat_total = math.fsum(ground_receipts)
    if not math.isclose(ground_heat_total,
            joins["ground_heat_operands"]["lse_outgoing_j_m2_stand_ground"],
            rel_tol=0.0, abs_tol=ENERGY_TOL):
        raise ValueError("ground_heat_equal_opposite_join")
    lse_ending = {"surface_enthalpy_j_m2_tile_ground": ingress[
            "ending_surface_enthalpy_j_m2_tile_ground"],
        "surface_temperature_k": ingress["mixed_temperature_k"]}
    condensation_energy = math.fsum(row["amount_kg_m2_stand_ground"]
        * row["specific_liquid_enthalpy_j_kg"]
        for row in water.get("condensation_credits", []))
    if water.get("condensation_credits"):
        lse_ending["condensation_enthalpy_credit_j_m2_stand_ground"] = \
            condensation_energy
    return {
        "vegetation": {"occupancies": occupancies,
            "outgoing_material_proposals": copy.deepcopy(proposals)},
        "hydrology": {"resource_store_ledger": resource_ledger,
            "post_ingress_surface_mass_kg_m2_tile_ground": ingress[
                "ending_mass_kg_m2_tile_ground"]},
        "land_surface_energy": lse_ending,
        "soil_thermal": {"temperatures_k": soil_temperatures,
            "ground_heat_receipt_j_m2_stand_ground": ground_heat_total,
            "infiltration_enthalpy_receipt_j_m2_stand_ground": ingress[
                "soil_thermal_infiltration_receipt_j_m2_stand_ground"]},
        "biogeochemistry": {"mineral_n": copy.deepcopy(
                beginning["biogeochemistry"]["state"]["mineral_n"]),
            "material_receipts": receipts},
    }


def validate_owner_candidates(transaction_id: int, beginning: dict[str, Any],
                              candidates: dict[str, Any], joins: dict[str, Any]) -> dict[str, Any]:
    expected = {
        "vegetation": "vegetation-v8", "hydrology": "hydrology-real-owner",
        "land_surface_energy": "land-surface-energy-v1",
        "soil_thermal": "soil-thermal-owner", "biogeochemistry": "biogeochemistry-owner",
    }
    if set(beginning) != set(expected) or set(candidates) != set(expected):
        raise ValueError("complete_five_owner_set")
    for kind, owner_id in expected.items():
        candidate = candidates[kind]
        owner_beginning = beginning[kind]
        if candidate.get("transaction_id") != transaction_id \
                or candidate.get("owner_id") != owner_id:
            raise ValueError(f"owner_candidate_identity:{kind}")
        for field in ("model_version", "model_definition_sha256", "configuration_sha256"):
            if candidate.get(field) != owner_beginning.get(field):
                raise ValueError(f"owner_model_configuration_identity:{kind}:{field}")
        if candidate.get("beginning_state_sha256") != digest(beginning[kind]):
            raise ValueError(f"owner_beginning_lineage:{kind}")
        if "ending_state" not in candidate:
            raise ValueError(f"owner_ending_state_missing:{kind}")
    water = joins["water"]
    water_validation = validate_water_protocol(water)
    if candidates["hydrology"]["water_protocol_sha256"] != water_validation["protocol_sha256"]:
        raise ValueError("hydrology_water_join")
    expected_endings = reconstruct_owner_endings(beginning, water, joins)
    for kind, expected_ending in expected_endings.items():
        if candidates[kind]["ending_state"] != expected_ending:
            raise ValueError(f"owner_ending_state_reconstruction:{kind}")
    condensation_energy = math.fsum(row["amount_kg_m2_stand_ground"]
        * row["specific_liquid_enthalpy_j_kg"]
        for row in water.get("condensation_credits", []))
    if water.get("condensation_credits") and candidates["land_surface_energy"][
            "ending_state"].get("condensation_enthalpy_credit_j_m2_stand_ground") \
            != condensation_energy:
        raise ValueError("condensation_mass_energy_join")
    return {"water_protocol_sha256": water_validation["protocol_sha256"],
        "ground_heat_join_sha256": digest(joins["ground_heat_operands"]),
        "material_join_sha256": digest(joins["material_operands"]),
        "owner_candidate_set_sha256": digest(candidates)}


def owner_receipts(transaction_id: int, beginning: dict[str, Any],
                   candidates: dict[str, Any], joins: dict[str, Any]) -> dict[str, Any]:
    validate_owner_candidates(transaction_id, beginning, candidates, joins)
    expected = {
        "vegetation": "vegetation-v8", "hydrology": "hydrology-real-owner",
        "land_surface_energy": "land-surface-energy-v1",
        "soil_thermal": "soil-thermal-owner", "biogeochemistry": "biogeochemistry-owner",
    }
    return {kind: {"transaction_id": transaction_id, "owner_kind": kind,
        "owner_id": expected[kind], "beginning_state_sha256": digest(beginning[kind]),
        "candidate_state_sha256": digest(candidates[kind])}
        for kind in expected}


def validate_multi_ofe_route(upstream_crossing: dict[str, Any],
                             downstream_parcel: dict[str, Any]) -> dict[str, Any]:
    if upstream_crossing["process"] != "runoff" \
            or downstream_parcel["parcel_kind"] != "routed_runon":
        raise ValueError("multi_ofe_process_identity")
    joins = {
        "route_id": downstream_parcel["route_id"],
        "source_ofe_id": upstream_crossing["ofe_id"],
        "downstream_declared_source_ofe_id": downstream_parcel["source_ofe_id"],
        "source_tile_id": upstream_crossing["tile_id"],
        "downstream_declared_source_tile_id": downstream_parcel["source_tile_id"],
        "destination_ofe_id": downstream_parcel["destination_ofe_id"],
        "destination_tile_id": downstream_parcel["destination_tile_id"],
        "source_ofe_area_m2": upstream_crossing["source_ofe_area_m2"],
        "destination_ofe_area_m2": downstream_parcel["destination_ofe_area_m2"],
        "interval_s": downstream_parcel["interval_s"],
        "source_transaction_id": upstream_crossing["accepted_transaction_id"],
        "downstream_declared_source_transaction_id": downstream_parcel[
            "source_transaction_id"],
        "destination_transaction_id": downstream_parcel["destination_transaction_id"],
        "amount_basis": downstream_parcel["amount_basis"],
        "upstream_state_sha256": upstream_crossing["source_state_sha256"],
        "downstream_source_state_sha256": downstream_parcel["source_state_sha256"],
        "upstream_mass_kg_m2": upstream_crossing["amount_kg_m2_tile_ground"],
        "downstream_mass_kg_m2": downstream_parcel[
            "amount_kg_m2_destination_tile_ground"],
        "upstream_specific_enthalpy_j_kg": upstream_crossing[
            "specific_liquid_enthalpy_j_kg"],
        "downstream_specific_enthalpy_j_kg": downstream_parcel[
            "specific_liquid_enthalpy_j_kg"],
        "upstream_energy_j_m2": upstream_crossing["amount_j_m2_tile_ground"],
        "downstream_energy_j_m2": downstream_parcel[
            "amount_kg_m2_destination_tile_ground"] * downstream_parcel[
                "specific_liquid_enthalpy_j_kg"]}
    joins.update({"extensive_mass_kg": joins["upstream_mass_kg_m2"]
        * joins["source_ofe_area_m2"],
        "downstream_reconstructed_extensive_mass_kg": joins["downstream_mass_kg_m2"]
            * joins["destination_ofe_area_m2"],
        "extensive_energy_j": joins["upstream_energy_j_m2"]
            * joins["source_ofe_area_m2"],
        "downstream_reconstructed_extensive_energy_j": joins["downstream_energy_j_m2"]
            * joins["destination_ofe_area_m2"]})
    if joins["source_ofe_area_m2"] <= 0.0 or joins["destination_ofe_area_m2"] <= 0.0 \
            or joins["source_ofe_area_m2"] == joins["destination_ofe_area_m2"] \
            or joins["source_ofe_id"] != joins["downstream_declared_source_ofe_id"] \
            or joins["source_tile_id"] != joins["downstream_declared_source_tile_id"] \
            or downstream_parcel["source_ofe_area_m2"] != joins["source_ofe_area_m2"] \
            or joins["source_transaction_id"] \
                != joins["downstream_declared_source_transaction_id"] \
            or joins["source_transaction_id"] == joins["destination_transaction_id"] \
            or not math.isclose(joins["extensive_mass_kg"], joins[
                "downstream_reconstructed_extensive_mass_kg"], rel_tol=0.0,
                abs_tol=MASS_TOL) \
            or not math.isclose(joins["extensive_energy_j"], joins[
                "downstream_reconstructed_extensive_energy_j"], rel_tol=0.0,
                abs_tol=ENERGY_TOL) \
            or not math.isclose(joins["downstream_mass_kg_m2"],
                joins["extensive_mass_kg"] / joins["destination_ofe_area_m2"],
                rel_tol=0.0, abs_tol=MASS_TOL) \
            or not math.isclose(joins["downstream_energy_j_m2"],
                joins["extensive_energy_j"] / joins["destination_ofe_area_m2"],
                rel_tol=0.0, abs_tol=ENERGY_TOL) \
            or joins["upstream_specific_enthalpy_j_kg"] \
                != joins["downstream_specific_enthalpy_j_kg"] \
            or joins["upstream_state_sha256"] != joins["downstream_source_state_sha256"] \
            or upstream_crossing["route_id"] != downstream_parcel["route_id"] \
            or upstream_crossing["interval_s"] != downstream_parcel["interval_s"] \
            or upstream_crossing["amount_basis"] != downstream_parcel["amount_basis"] \
            or joins["amount_basis"] != "kg_h2o_m-2_route_ground_interval":
        raise ValueError("multi_ofe_mass_enthalpy_join")
    return joins


def validate_schema_instances(instances: dict[str, Any]) -> dict[str, Any]:
    """Validate emitted positive records against the checksum-bound schemas."""
    try:
        import jsonschema
        from referencing import Registry, Resource
    except ImportError as error:  # pragma: no cover - required authority tool
        raise RuntimeError("jsonschema is required for authority regeneration") from error
    root = Path(__file__).resolve().parent
    names = {
        "configuration": "lse_v1_configuration_schema.json",
        "coupled_transaction": "lse_v1_coupled_transaction_schema.json",
        "diagnostics": "lse_v1_diagnostics_schema.json",
        "forcing": "lse_v1_forcing_schema.json",
        "state": "lse_v1_state_schema.json",
        "water_protocol": "lse_v1_water_protocol_schema.json",
    }
    schemas: dict[str, Any] = {}
    registry = Registry()
    for key, filename in names.items():
        path = root / filename
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != SCHEMA_SHA256[key]:
            raise RuntimeError(f"schema checksum mismatch:{key}:{actual}")
        schema = json.loads(path.read_text(encoding="utf-8"))
        schemas[key] = schema
        registry = registry.with_resource(schema["$id"], Resource.from_contents(schema))
        registry = registry.with_resource(filename, Resource.from_contents(schema))
        registry = registry.with_resource(
            f"https://openwepp.org/schema/lse/openwepp_snow_free_lse_v1/{filename}",
            Resource.from_contents(schema))
    evidence = {}
    for key, instance in instances.items():
        validator = jsonschema.Draft202012Validator(schemas[key], registry=registry)
        errors = sorted(validator.iter_errors(instance), key=lambda item: list(item.path))
        if errors:
            first = errors[0]
            raise ValueError(f"schema_validation:{key}:{list(first.path)}:{first.message}")
        evidence[key] = {"schema_sha256": SCHEMA_SHA256[key],
            "instance_sha256": digest(instance), "validated": True}
    return evidence


def strict_positive_instances(water_protocol: dict[str, Any]) -> dict[str, Any]:
    """Build one complete strict positive instance for every normative schema."""
    transaction_id = water_protocol["transaction_id"]
    owner_config = lambda owner, model: {"owner_id": owner, "model_version": model,
        "model_definition_sha256": digest({"model": model}),
        "configuration_sha256": digest({"owner": owner, "configuration": 1})}
    vegetation_config = owner_config("vegetation-v8", "OPENWEPP_C3_WOODY_V8")
    vegetation_config["model_definition_sha256"] = V8_MODEL_DEFINITION_SHA256
    configuration = {"model_version": "OPENWEPP_SNOW_FREE_LSE_V1",
        "model_definition_sha256": LSE_MODEL_DEFINITION_SHA256,
        "configuration_sha256": "0" * 64, "owner_id": "land-surface-energy-v1",
        "vegetation_configuration": vegetation_config,
        "hydrology_configuration": owner_config("hydrology-real-owner", "OPENWEPP_WATBAL_OWNER_V1"),
        "soil_thermal_configuration": owner_config("soil-thermal-owner", "OPENWEPP_SOIL_THERMAL_V1"),
        "numerics": {"iteration_limit": 50, "backtracking_exponents": list(range(21)),
            "finite_difference": "centered_sqrt_binary64_epsilon_minus_then_plus",
            "pivot_threshold": "64_times_binary64_epsilon_times_matrix_infinity_norm",
            "equal_pivot_rule": "lowest_row", "temperature_bounds_k": [200, 350],
            "humidity_bounds_kg_kg": [0, 0.1], "temperature_step_tolerance_k": 1e-8,
            "humidity_step_tolerance_kg_kg": 1e-12,
            "hydraulic_step_tolerance_mm": 1e-7, "beta_step_tolerance": 1e-10},
        "ofes": [{"ofe_id": "ofe-1", "area_m2": 100.0,
            "soil_interface_layers": [{"layer_id": "soil-1", "thickness_m": 0.08,
                "thermal_conductivity_w_m_k": 1.1,
                "areal_heat_capacity_j_m2_k": 120000.0}],
            "tiles": [{"tile_id": "open", "fraction_ofe_ground": 1.0,
                "vegetation_tile_id": "open", "surface_vis_albedo": 0.18,
                "surface_nir_albedo": 0.31, "surface_heat_storage_mode": "finite_capacity",
                "turbulence": {"mode": "open_neutral", "reference_height_m": 20.0,
                    "roughness_momentum_m": 0.12, "roughness_heat_m": 0.015,
                    "roughness_vapor_m": 0.01},
                "surface": {"surface_class": "bare_mineral_soil",
                    "dry_areal_heat_capacity_j_m2_k": 42000.0,
                    "mineral_skin_thickness_m": 0.02,
                    "mineral_skin_thermal_conductivity_w_m_k": 0.75,
                    "top_layer_saturated_water_content_m3_m3": 0.46,
                    "top_layer_porosity_m3_m3": 0.46,
                    "top_layer_saturated_matric_potential_mm": -120.0,
                    "top_layer_clapp_hornberger_b": 4.05,
                    "top_layer_initial_water_content_m3_m3": 0.22}}]}]}
    configuration["configuration_sha256"] = digest(
        {**configuration, "configuration_sha256": ""})
    forcing = {"forcing_sha256": "0" * 64, "transaction_id": transaction_id,
        "interval_s": DT, "air_temperature_k": 294.0,
        "air_specific_humidity_kg_kg": 0.0095, "air_pressure_pa": 93000.0,
        "reference_wind_m_s": 2.4, "neutral_stability": True,
        "snow_present_at_beginning": False, "snow_present_at_end": False,
        "snow_terminal_payload_present": False, "direct_vis_w_m2": 170.0,
        "diffuse_vis_w_m2": 55.0, "direct_nir_w_m2": 205.0,
        "diffuse_nir_w_m2": 65.0, "atmospheric_downward_longwave_w_m2": 335.0,
        "precipitation_parcels": [], "runon_parcels": []}
    forcing["forcing_sha256"] = digest({**forcing, "forcing_sha256": ""})
    state = {"model_definition_sha256": LSE_MODEL_DEFINITION_SHA256,
        "configuration_sha256": configuration["configuration_sha256"],
        "state_sha256": "0" * 64, "owner_id": "land-surface-energy-v1",
        "last_accepted_transaction_id": transaction_id - 1,
        "tiles": [{"ofe_id": "ofe-1", "tile_id": "open",
            "surface_enthalpy_j_m2_tile_ground": 917700.0,
            "surface_temperature_warm_start_k": 295.0}]}
    state["state_sha256"] = digest({**state, "state_sha256": ""})
    beginning_payloads = {
        "vegetation": {"occupancies": []},
        "hydrology": {"post_ingress_surface_mass_kg_m2_tile_ground": 0.0,
            "resource_stores": [{"resource_identity": list(resource),
                "amount_kg_m2_stand_ground": math.fsum(row[
                    "amount_kg_m2_stand_ground"] for row in water_protocol["authorizations"]
                    if resource_group_identity(row["key"]) == resource)}
                for resource in sorted({resource_group_identity(row["key"])
                    for row in water_protocol["requests"]}, key=canonical)]},
        "land_surface_energy": {"surface_enthalpy_j_m2_tile_ground":
            state["tiles"][0]["surface_enthalpy_j_m2_tile_ground"]},
        "soil_thermal": {"temperatures_k": [291.5]},
        "biogeochemistry": {"mineral_n": {}, "material_receipts": []}}
    owner_ids = {"vegetation": "vegetation-v8", "hydrology": "hydrology-real-owner",
        "land_surface_energy": "land-surface-energy-v1",
        "soil_thermal": "soil-thermal-owner", "biogeochemistry": "biogeochemistry-owner"}
    owner_models = {"vegetation": "OPENWEPP_C3_WOODY_V8",
        "hydrology": "OPENWEPP_WATBAL_OWNER_V1",
        "land_surface_energy": "OPENWEPP_SNOW_FREE_LSE_V1",
        "soil_thermal": "OPENWEPP_SOIL_THERMAL_V1",
        "biogeochemistry": "OPENWEPP_BGC_OWNER_V1"}
    beginning = {kind: typed_owner_beginning(owner_ids[kind], owner_models[kind], payload)
        for kind, payload in beginning_payloads.items()}
    candidates = {kind: {"transaction_id": transaction_id, "owner_id": owner_ids[kind],
        "model_version": beginning[kind]["model_version"],
        "model_definition_sha256": beginning[kind]["model_definition_sha256"],
        "configuration_sha256": beginning[kind]["configuration_sha256"],
        "beginning_state_sha256": digest(value),
        "ending_state": {**copy.deepcopy(value), "accepted_transaction_id": transaction_id}}
        for kind, value in beginning.items()}
    material_proposal = {"transaction_id": transaction_id,
        "proposal_id": "strict-material-1", "donor": "vegetation-standing-dead",
        "receiver": "bgc-litter", "carbon_kg_m2_stand_ground": 1.0e-3,
        "nitrogen_kg_m2_stand_ground": 5.0e-5,
        "dry_material_kg_m2_stand_ground": 2.2e-3}
    material_receipt = {field: material_proposal[field] for field in (
        "transaction_id", "proposal_id", "receiver", "carbon_kg_m2_stand_ground",
        "nitrogen_kg_m2_stand_ground", "dry_material_kg_m2_stand_ground")}
    joins = {"water": water_protocol, "vegetation_component_operands": [],
        "ground_heat_operands": {"lse_outgoing_j_m2_stand_ground": 0.0},
        "material_operands": {"vegetation_proposals": [material_proposal],
            "bgc_constructed_receipts": [material_receipt]},
        "advection_operands": {"transaction_id": transaction_id, "ofe_id": "ofe-1",
            "tile_id": "open", "tile_fraction": 1.0, "beginning_mass_tile": 0.0,
            "pre_ingress_mass_tile": 0.0,
            "pre_ingress_surface_enthalpy_j_m2_tile": state["tiles"][0][
                "surface_enthalpy_j_m2_tile_ground"],
            "dry_capacity_j_m2_k": 42000.0, "parcels": [],
            "infiltration_kg_m2_tile": 0.0, "runoff_kg_m2_tile": 0.0},
        "soil_thermal_operands": {"layers": [{"layer_id": "soil-1",
            "tile_fraction": 1.0, "areal_heat_capacity_j_m2_k": 120000.0,
            "ground_heat_receipt_j_m2_stand_ground": 0.0}],
            "infiltration_receiver_layer_id": "soil-1",
            "infiltration_enthalpy_receipt_basis": "j_m2_stand_ground_interval",
            "infiltration_enthalpy_receipt_j_m2_stand_ground": 0.0}}
    candidates["hydrology"]["water_protocol_sha256"] = digest(water_protocol)
    reconstructed_endings = reconstruct_owner_endings(beginning, water_protocol, joins)
    for kind, ending_state in reconstructed_endings.items():
        candidates[kind]["ending_state"] = ending_state
    receipts = owner_receipts(transaction_id, beginning, candidates, joins)
    owner_ref = lambda owner, model, definition, config_sha, state_sha: {
        "owner_id": owner, "model_version": model,
        "model_definition_sha256": definition, "configuration_sha256": config_sha,
        "state_sha256": state_sha, "last_accepted_transaction_id": transaction_id - 1}
    vegetation_owner = owner_ref("vegetation-v8", "OPENWEPP_C3_WOODY_V8",
        V8_MODEL_DEFINITION_SHA256, vegetation_config["configuration_sha256"],
        digest(beginning["vegetation"]))
    bgc_owner = owner_ref("biogeochemistry-owner", "OPENWEPP_BGC_OWNER_V1",
        digest({"bgc": "definition"}), digest({"bgc": "configuration"}),
        digest(beginning["biogeochemistry"]))
    hydrology_snapshot = {"owner_id": "hydrology-real-owner",
        "configuration_sha256": configuration["hydrology_configuration"]["configuration_sha256"],
        "state_sha256": digest(beginning["hydrology"]),
        "snapshot_sha256": water_protocol["beginning_snapshot_sha256"],
        "last_accepted_transaction_id": transaction_id - 1,
        "availability_time": "immutable_beginning_before_current_interval_ingress",
        "stores": [{"ofe_id": "ofe-1", "tile_id": None,
            "source_type": "soil_layer_liquid", "source_id": "soil-layer-1",
            "soil_layer_id": "soil-1", "amount_basis": "kg_h2o_m-2_stand_ground",
            "liquid_amount_kg_m2": 1.0, "ice_amount_kg_m2": 0.0,
            "frozen": False, "thawing": False}]}
    soil_snapshot = {"owner_id": "soil-thermal-owner",
        "configuration_sha256": configuration["soil_thermal_configuration"]["configuration_sha256"],
        "state_sha256": digest(beginning["soil_thermal"]),
        "snapshot_sha256": digest({"soil_thermal_snapshot": 1}),
        "last_accepted_transaction_id": transaction_id - 1,
        "ofes": [{"ofe_id": "ofe-1", "ordered_layers": [{"layer_id": "soil-1",
            "temperature_k": 291.5, "enthalpy_j_m2_ofe_ground": 2.202e6}]}]}
    envelope = {"transaction_id": transaction_id,
        "lse_configuration_sha256": configuration["configuration_sha256"],
        "forcing": forcing, "beginning_vegetation_owner": vegetation_owner,
        "beginning_lse_state": state, "beginning_hydrology_snapshot": hydrology_snapshot,
        "beginning_soil_thermal_snapshot": soil_snapshot,
        "beginning_biogeochemistry_owner": bgc_owner,
        "water_protocol": water_protocol, "final_canopy_liquid_releases": [],
        "candidate_owner_bodies": candidates,
        "candidate_owner_receipts": receipts}
    rollback = [{"owner_kind": kind, "owner_id": receipts[kind]["owner_id"],
        "before_sha256": receipts[kind]["beginning_state_sha256"],
        "after_sha256": receipts[kind]["beginning_state_sha256"]}
        for kind in receipts]
    diagnostics = {"model_version": "OPENWEPP_SNOW_FREE_LSE_V1",
        "canonical_contract": "SC-LANDSURFACEENERGY-001@3",
        "model_definition_sha256": LSE_MODEL_DEFINITION_SHA256,
        "configuration_sha256": configuration["configuration_sha256"],
        "beginning_state_sha256": state["state_sha256"], "transaction_id": transaction_id,
        "ofe_id": "ofe-1", "tile_id": "open", "occupancy_id": None,
        "pass": "potential", "solve": "surface_energy", "accepted": True,
        "failure_code": None, "failure_kind": None, "iterations": 4,
        "backtracking_count": 0, "ordered_residuals": [],
        "step_norms": {"temperature_k": 0.0, "humidity_kg_kg": None,
            "ci_pa": None, "hydraulic_mm": None, "beta": None},
        "active_bounds": [], "active_water_caps": [], "bracket": None,
        "pivot_magnitude": 1.0, "matrix_infinity_norm": 1.0,
        "owner_rollback_hashes": rollback}
    return {"configuration": configuration, "forcing": forcing, "state": state,
        "water_protocol": water_protocol, "diagnostics": diagnostics,
        "coupled_transaction": envelope}


def execute_covered_water_transaction(
    core: Any, *, rank_count: int, ofe_id: str, tile_id: str,
    tile_fraction: float, transaction_id: int, supply_fraction: float,
) -> dict[str, Any]:
    """Execute potential -> one arbitration -> rebuilt fixed-cap covered solve."""
    column = core.build_covered_column_case(rank_count)
    for occupancy in column["occupancies"]:
        occupancy["case"]["tile_fraction"] = tile_fraction
    column["ground_config"]["tile_id"] = tile_id
    column["ground_config"]["surface_source_id"] = f"litter-liquid:{ofe_id}:{tile_id}"
    potential = core.solve_covered_column(column)
    if not potential["accepted"]:
        raise RuntimeError("covered_potential_rejected")
    dt = column["occupancies"][0]["case"]["dt_s"]
    requests = []
    for occupancy in potential["detail"]["occupancies"]:
        occupancy_id = occupancy["occupancy_id"]
        for row in occupancy["hydraulic_and_component"]["q3"]:
            key = water_key(transaction_id=transaction_id, owner_id="vegetation-v8",
                component="vegetation_root", ofe_id=ofe_id,
                requesting_tile_id=tile_id, occupancy_id=occupancy_id,
                source_type="soil_layer_liquid", source_id=f"soil:{ofe_id}:{row['layer_id']}",
                soil_layer_id=row["layer_id"])
            requests.append({"key": key, "amount_kg_m2_stand_ground":
                row["request_rate_kg_m2_tile_s"] * tile_fraction * dt})
    ground_row = potential["detail"]["ground"]["ground_vapor"]
    ground_key = water_key(transaction_id=transaction_id,
        owner_id="land-surface-energy-v1", component="ground_surface", ofe_id=ofe_id,
        requesting_tile_id=tile_id, surface_id=f"surface:{ofe_id}:{tile_id}",
        surface_class="forest_litter", source_type="litter_liquid",
        source_id=column["ground_config"]["surface_source_id"], source_tile_id=tile_id)
    requests.append({"key": ground_key, "amount_kg_m2_stand_ground":
        ground_row["request_rate_kg_m2_tile_s"] * tile_fraction * dt})
    total_by_resource: dict[tuple[Any, ...], float] = {}
    for request in requests:
        identity = resource_group_identity(request["key"])
        total_by_resource[identity] = total_by_resource.get(identity, 0.0) \
            + request["amount_kg_m2_stand_ground"]
    stores = {identity: (amount if identity[-1] == "soil-2" else supply_fraction * amount)
              for identity, amount in total_by_resource.items()}
    authorizations = arbitrate(requests, stores)
    auth_index = {canonical(item["key"]): item for item in authorizations}
    root_caps: dict[str, dict[str, Any]] = {}
    for occupancy in potential["detail"]["occupancies"]:
        occupancy_id = occupancy["occupancy_id"]
        root_caps[occupancy_id] = {}
        for row in occupancy["hydraulic_and_component"]["q3"]:
            key = next(request["key"] for request in requests
                if request["key"]["occupancy_id"] == occupancy_id
                and request["key"]["soil_layer_id"] == row["layer_id"])
            authorization = auth_index[canonical(key)]["amount_kg_m2_stand_ground"]
            root_caps[occupancy_id][row["layer_id"]] = {
                "cap_rate_kg_m2_tile_s": authorization / (tile_fraction * dt),
                "request_rate_kg_m2_tile_s": next(request["amount_kg_m2_stand_ground"]
                    for request in requests if request["key"] == key) / (tile_fraction * dt)}
    ground_authorization = auth_index[canonical(ground_key)]["amount_kg_m2_stand_ground"]
    caps = {"root": root_caps, "ground": {column["ground_config"]["surface_source_id"]: {
        "cap_rate_kg_m2_tile_s": ground_authorization / (tile_fraction * dt),
        "request_rate_kg_m2_tile_s": next(request["amount_kg_m2_stand_ground"]
            for request in requests if request["key"] == ground_key) / (tile_fraction * dt)}}}
    final = core.solve_covered_column(column, caps=caps, start=potential["solution"])
    if not final["accepted"]:
        raise RuntimeError("covered_fixed_cap_rejected")
    finalized_uses = []
    for occupancy in final["detail"]["occupancies"]:
        occupancy_id = occupancy["occupancy_id"]
        for row in occupancy["hydraulic_and_component"]["q3"]:
            key = next(request["key"] for request in requests
                if request["key"]["occupancy_id"] == occupancy_id
                and request["key"]["soil_layer_id"] == row["layer_id"])
            solved_amount = (auth_index[canonical(key)]["amount_kg_m2_stand_ground"]
                if row["branch"] == "authorization_active_or_tie" else
                row["q_final_kg_m2_tile_s"] * tile_fraction * dt)
            finalized_uses.append({"key": key, "amount_kg_m2_stand_ground":
                solved_amount})
    final_ground = final["detail"]["ground"]["ground_vapor"]
    solved_ground_amount = (ground_authorization
        if final_ground["branch"] == "authorization_active_or_tie" else
        max(0.0, final_ground["q_final_kg_m2_tile_s"]) * tile_fraction * dt)
    finalized_uses.append({"key": ground_key,
        "amount_kg_m2_stand_ground": solved_ground_amount})
    credits = []
    if final_ground["q_final_kg_m2_tile_s"] < 0.0:
        amount = -final_ground["q_final_kg_m2_tile_s"] * tile_fraction * dt
        temperature = final["detail"]["ground"]["ground_temperature_k"]
        credits.append({"transaction_id": transaction_id,
            "hydrology_owner_id": "hydrology-real-owner", "ofe_id": ofe_id,
            "tile_id": tile_id, "surface_id": f"surface:{ofe_id}:{tile_id}",
            "amount_kg_m2_stand_ground": amount,
            "amount_basis": "kg_h2o_m-2_stand_ground_interval",
            "temperature_k": temperature,
            "specific_liquid_enthalpy_j_kg": liquid_enthalpy(temperature)})
    snapshot_stores = [{"resource_identity": list(identity),
        "amount_kg_m2_stand_ground": stores[identity]}
        for identity in sorted(stores, key=canonical)]
    protocol = {"transaction_id": transaction_id,
        "hydrology_owner_id": "hydrology-real-owner",
        "beginning_snapshot_sha256": digest({"stores": snapshot_stores, "time": "beginning"}),
        "requests": sorted(requests, key=lambda item: canonical(item["key"])),
        "authorizations": authorizations,
        "finalized_uses": sorted(finalized_uses, key=lambda item: canonical(item["key"])),
        "condensation_credits": credits}
    validation = validate_water_protocol(protocol)
    arbitration_hash = digest({"snapshot": protocol["beginning_snapshot_sha256"],
        "requests": protocol["requests"], "authorizations": protocol["authorizations"]})
    expected_caps = {"root": {}, "ground": {}}
    for authorization in authorizations:
        key = authorization["key"]
        request = next(item for item in requests if item["key"] == key)
        cap = {"cap_rate_kg_m2_tile_s":
            authorization["amount_kg_m2_stand_ground"] / (tile_fraction * dt),
            "request_rate_kg_m2_tile_s":
            request["amount_kg_m2_stand_ground"] / (tile_fraction * dt)}
        if key["requesting_component"] == "vegetation_root":
            expected_caps["root"].setdefault(key["occupancy_id"], {})[
                key["soil_layer_id"]] = cap
        else:
            expected_caps["ground"][key["source_id"]] = cap
    cap_binding_proved = caps == expected_caps
    return {"authority_inputs": {"ground_config": column["ground_config"],
            "ground_state": column["ground_state"],
            "occupancy_cases_sha256": digest([item["case"] for item in column["occupancies"]])},
        "potential": potential, "beginning_column_sha256": potential["beginning_sha256"],
        "immutable_arbitration": {"call_count": 1, "input_sha256": arbitration_hash,
            "output_sha256": digest(authorizations)}, "water_protocol": protocol,
        "beginning_resource_stores": snapshot_stores,
        "water_protocol_validation": validation, "fixed_caps": caps, "final": final,
        "final_rebuilt_from_beginning": {
            "potential_beginning_sha256": potential["beginning_sha256"],
            "final_beginning_sha256": final["beginning_sha256"],
            "final_rebuilt_from_beginning_sha256": final["rebuilt_from_beginning_sha256"],
            "proved": potential["beginning_sha256"] == final["beginning_sha256"]
                == final["rebuilt_from_beginning_sha256"]},
        "single_immutable_authorization": {
            "call_count": 1, "arbitration_input_sha256": arbitration_hash,
            "fixed_cap_sha256": digest(caps), "proved": cap_binding_proved}}


def execute_shared_layer_root_ground_competition(core: Any) -> dict[str, Any]:
    """One OFE snapshot arbitrates covered roots and open bare-soil demand."""
    transaction_id, ofe_id, dt = 20260814003, "ofe-shared", 1800.0
    covered_fraction, second_covered_fraction, open_fraction = 0.40, 0.35, 0.25
    covered = core.build_covered_column_case(2)
    for occupancy in covered["occupancies"]:
        occupancy["case"]["tile_fraction"] = covered_fraction
    covered["ground_config"]["tile_id"] = "forest"
    covered["ground_config"]["surface_source_id"] = "litter:ofe-shared:forest"
    covered_second = core.build_covered_column_case(2)
    renamed_optics = {}
    for occupancy in covered_second["occupancies"]:
        old_occupancy_id = occupancy["occupancy_id"]
        occupancy["case"]["tile_fraction"] = second_covered_fraction
        occupancy["case"]["classes"]["sun"]["absorbed_shortwave_w_m2_tile"] *= 0.72
        occupancy["case"]["classes"]["shade"]["absorbed_shortwave_w_m2_tile"] *= 0.83
        occupancy["case"]["classes"]["sun"]["absorbed_par_w_m2_leaf"] *= 0.72
        occupancy["case"]["classes"]["shade"]["absorbed_par_w_m2_leaf"] *= 0.83
        occupancy["occupancy_id"] += "-south"
        renamed_optics[occupancy["occupancy_id"]] = covered_second["shortwave"] \
            ["optics_by_occupancy"][old_occupancy_id]
    covered_second["shortwave"]["optics_by_occupancy"] = renamed_optics
    for incident in covered_second["shortwave"]["incident_by_band_w_m2_tile"].values():
        incident["direct"] *= 0.72
        incident["diffuse"] *= 0.83
    covered_second["ground_config"]["tile_id"] = "forest-south"
    covered_second["ground_config"]["surface_source_id"] = "litter:ofe-shared:forest-south"
    covered_second["ground_config"]["ground_terminal_shortwave_by_band_direction_w_m2_tile"] = {
        key: 0.68 * value for key, value in covered_second["ground_config"]
            ["ground_terminal_shortwave_by_band_direction_w_m2_tile"].items()}
    open_bundle = core.build_open_bare_soil_case(4)
    open_bundle["configuration"]["tile_fraction"] = open_fraction
    open_bundle["configuration"]["tile_id"] = "open"
    open_bundle["configuration"]["surface_source_id"] = "soil:ofe-shared:soil-1"
    covered_potential = core.solve_covered_column(covered)
    covered_second_potential = core.solve_covered_column(covered_second)
    open_potential = core.solve_open_bare_soil(open_bundle)
    if not covered_potential["accepted"] or not covered_second_potential["accepted"] \
            or not open_potential["accepted"]:
        raise RuntimeError("shared_layer_potential_rejected")
    requests = []
    for occupancy in covered_potential["detail"]["occupancies"]:
        for row in occupancy["hydraulic_and_component"]["q3"]:
            key = water_key(transaction_id=transaction_id, owner_id="vegetation-v8",
                component="vegetation_root", ofe_id=ofe_id,
                requesting_tile_id="forest", occupancy_id=occupancy["occupancy_id"],
                source_type="soil_layer_liquid", source_id=f"soil:{ofe_id}:{row['layer_id']}",
                soil_layer_id=row["layer_id"])
            requests.append({"key": key, "amount_kg_m2_stand_ground":
                row["request_rate_kg_m2_tile_s"] * covered_fraction * dt})
    for occupancy in covered_second_potential["detail"]["occupancies"]:
        for row in occupancy["hydraulic_and_component"]["q3"]:
            key = water_key(transaction_id=transaction_id, owner_id="vegetation-v8",
                component="vegetation_root", ofe_id=ofe_id,
                requesting_tile_id="forest-south", occupancy_id=occupancy["occupancy_id"],
                source_type="soil_layer_liquid", source_id=f"soil:{ofe_id}:{row['layer_id']}",
                soil_layer_id=row["layer_id"])
            requests.append({"key": key, "amount_kg_m2_stand_ground":
                row["request_rate_kg_m2_tile_s"] * second_covered_fraction * dt})
    litter = covered_potential["detail"]["ground"]["ground_vapor"]
    litter_key = water_key(transaction_id=transaction_id,
        owner_id="land-surface-energy-v1", component="ground_surface", ofe_id=ofe_id,
        requesting_tile_id="forest", surface_id="surface:ofe-shared:forest",
        surface_class="forest_litter", source_type="litter_liquid",
        source_id=covered["ground_config"]["surface_source_id"], source_tile_id="forest")
    requests.append({"key": litter_key, "amount_kg_m2_stand_ground":
        litter["request_rate_kg_m2_tile_s"] * covered_fraction * dt})
    litter_second = covered_second_potential["detail"]["ground"]["ground_vapor"]
    litter_second_key = water_key(transaction_id=transaction_id,
        owner_id="land-surface-energy-v1", component="ground_surface", ofe_id=ofe_id,
        requesting_tile_id="forest-south", surface_id="surface:ofe-shared:forest-south",
        surface_class="forest_litter", source_type="litter_liquid",
        source_id=covered_second["ground_config"]["surface_source_id"],
        source_tile_id="forest-south")
    requests.append({"key": litter_second_key, "amount_kg_m2_stand_ground":
        litter_second["request_rate_kg_m2_tile_s"] * second_covered_fraction * dt})
    open_row = open_potential["detail"]["ground_vapor"]
    open_key = water_key(transaction_id=transaction_id,
        owner_id="land-surface-energy-v1", component="ground_surface", ofe_id=ofe_id,
        requesting_tile_id="open", surface_id="surface:ofe-shared:open",
        surface_class="bare_mineral_soil", source_type="soil_layer_liquid",
        source_id="soil:ofe-shared:soil-1", soil_layer_id="soil-1")
    requests.append({"key": open_key, "amount_kg_m2_stand_ground":
        open_row["request_kg_m2_stand_ground"]})
    totals: dict[tuple[Any, ...], float] = {}
    for request in requests:
        resource = resource_group_identity(request["key"])
        totals[resource] = totals.get(resource, 0.0) + request["amount_kg_m2_stand_ground"]
    stores = {resource: (0.55 * amount if resource[-1] == "soil-1" else amount)
              for resource, amount in totals.items()}
    authorizations = arbitrate(requests, stores)
    auth = {canonical(item["key"]): item for item in authorizations}
    root_caps: dict[str, dict[str, Any]] = {}
    for occupancy in covered_potential["detail"]["occupancies"]:
        root_caps[occupancy["occupancy_id"]] = {}
        for row in occupancy["hydraulic_and_component"]["q3"]:
            request = next(item for item in requests if item["key"]["occupancy_id"]
                == occupancy["occupancy_id"] and item["key"]["soil_layer_id"] == row["layer_id"])
            amount = auth[canonical(request["key"])]["amount_kg_m2_stand_ground"]
            root_caps[occupancy["occupancy_id"]][row["layer_id"]] = {
                "cap_rate_kg_m2_tile_s": amount / (covered_fraction * dt),
                "request_rate_kg_m2_tile_s": request["amount_kg_m2_stand_ground"]
                    / (covered_fraction * dt)}
    litter_auth = auth[canonical(litter_key)]["amount_kg_m2_stand_ground"]
    covered_caps = {"root": root_caps, "ground": {
        covered["ground_config"]["surface_source_id"]: {
            "cap_rate_kg_m2_tile_s": litter_auth / (covered_fraction * dt),
            "request_rate_kg_m2_tile_s": next(item["amount_kg_m2_stand_ground"]
                for item in requests if item["key"] == litter_key) / (covered_fraction * dt)}}}
    second_root_caps: dict[str, dict[str, Any]] = {}
    for occupancy in covered_second_potential["detail"]["occupancies"]:
        second_root_caps[occupancy["occupancy_id"]] = {}
        for row in occupancy["hydraulic_and_component"]["q3"]:
            request = next(item for item in requests if item["key"]["occupancy_id"]
                == occupancy["occupancy_id"] and item["key"]["soil_layer_id"] == row["layer_id"])
            amount = auth[canonical(request["key"])]["amount_kg_m2_stand_ground"]
            second_root_caps[occupancy["occupancy_id"]][row["layer_id"]] = {
                "cap_rate_kg_m2_tile_s": amount / (second_covered_fraction * dt),
                "request_rate_kg_m2_tile_s": request["amount_kg_m2_stand_ground"]
                    / (second_covered_fraction * dt)}
    second_litter_auth = auth[canonical(litter_second_key)]["amount_kg_m2_stand_ground"]
    second_caps = {"root": second_root_caps, "ground": {
        covered_second["ground_config"]["surface_source_id"]: {
            "cap_rate_kg_m2_tile_s": second_litter_auth / (second_covered_fraction * dt),
            "request_rate_kg_m2_tile_s": next(item["amount_kg_m2_stand_ground"]
                for item in requests if item["key"] == litter_second_key)
                / (second_covered_fraction * dt)}}}
    open_auth = auth[canonical(open_key)]["amount_kg_m2_stand_ground"]
    open_cap = {"cap_rate_kg_m2_tile_s": open_auth / (open_fraction * dt),
        "request_rate_kg_m2_tile_s": open_row["request_kg_m2_stand_ground"]
            / (open_fraction * dt)}
    covered_final = core.solve_covered_column(
        covered, caps=covered_caps, start=covered_potential["solution"])
    covered_second_final = core.solve_covered_column(
        covered_second, caps=second_caps, start=covered_second_potential["solution"])
    open_final = core.solve_open_bare_soil(open_bundle, cap=open_cap)
    if not covered_final["accepted"] or not covered_second_final["accepted"] \
            or not open_final["accepted"]:
        raise RuntimeError("shared_layer_fixed_cap_rejected")
    finalized_uses: list[dict[str, Any]] = []
    def append_covered_uses(result: dict[str, Any], tile: str, fraction: float) -> None:
        for occupancy in result["detail"]["occupancies"]:
            for row in occupancy["hydraulic_and_component"]["q3"]:
                request = next(item for item in requests
                    if item["key"]["requesting_tile_id"] == tile
                    and item["key"]["occupancy_id"] == occupancy["occupancy_id"]
                    and item["key"]["soil_layer_id"] == row["layer_id"])
                authorization = auth[canonical(request["key"])]["amount_kg_m2_stand_ground"]
                amount = (authorization if row["branch"] == "authorization_active_or_tie"
                    else row["q_final_kg_m2_tile_s"] * fraction * dt)
                finalized_uses.append({"key": request["key"],
                    "amount_kg_m2_stand_ground": amount})
        vapor = result["detail"]["ground"]["ground_vapor"]
        request = next(item for item in requests
            if item["key"]["requesting_tile_id"] == tile
            and item["key"]["requesting_component"] == "ground_surface")
        authorization = auth[canonical(request["key"])]["amount_kg_m2_stand_ground"]
        amount = (authorization if vapor["branch"] == "authorization_active_or_tie"
            else max(vapor["q_final_kg_m2_tile_s"], 0.0) * fraction * dt)
        finalized_uses.append({"key": request["key"],
            "amount_kg_m2_stand_ground": amount})
    append_covered_uses(covered_final, "forest", covered_fraction)
    append_covered_uses(covered_second_final, "forest-south", second_covered_fraction)
    open_final_row = open_final["detail"]["ground_vapor"]
    open_final_amount = (open_auth if open_final_row["branch"] == "authorization_active_or_tie"
        else max(open_final_row["q_final_kg_m2_tile_s"], 0.0) * open_fraction * dt)
    finalized_uses.append({"key": open_key,
        "amount_kg_m2_stand_ground": open_final_amount})
    protocol = {"transaction_id": transaction_id,
        "hydrology_owner_id": "hydrology-real-owner",
        "beginning_snapshot_sha256": digest({"stores": [
            {"resource_identity": list(resource),
             "amount_kg_m2_stand_ground": stores[resource]}
            for resource in sorted(stores, key=canonical)]}),
        "requests": sorted(requests, key=lambda item: canonical(item["key"])),
        "authorizations": authorizations,
        "finalized_uses": sorted(finalized_uses, key=lambda item: canonical(item["key"])),
        "condensation_credits": []}
    protocol_validation = validate_water_protocol(protocol)
    source_ledger = []
    for resource in sorted(stores, key=canonical):
        use = math.fsum(item["amount_kg_m2_stand_ground"] for item in finalized_uses
            if resource_group_identity(item["key"]) == resource)
        authorization = math.fsum(item["amount_kg_m2_stand_ground"] for item in authorizations
            if resource_group_identity(item["key"]) == resource)
        ending = stores[resource] - use
        if ending < -MASS_TOL:
            raise ValueError("shared_source_overdraw")
        source_ledger.append({"resource_identity": list(resource),
            "beginning_kg_m2_stand_ground": stores[resource],
            "finalized_use_kg_m2_stand_ground": use,
            "ending_kg_m2_stand_ground": ending,
            "unused_authorization_kg_m2_stand_ground": authorization - use,
            "mass_residual_kg_m2_stand_ground": stores[resource] - use - ending})
    soil1_competitors = [item for item in authorizations
        if item["key"]["soil_layer_id"] == "soil-1"]
    owners = {item["key"]["requesting_component"] for item in soil1_competitors}
    if owners != {"vegetation_root", "ground_surface"}:
        raise AssertionError("shared_layer_competitor_identity")
    return {"arbitration_call_count": 1, "beginning_snapshot_sha256": digest(
            [{"resource": list(key), "amount": stores[key]} for key in sorted(stores, key=canonical)]),
        "requests": sorted(requests, key=lambda item: canonical(item["key"])),
        "authorizations": authorizations, "finalized_uses": protocol["finalized_uses"],
        "water_protocol": protocol, "water_protocol_validation": protocol_validation,
        "source_ending_store_ledger": source_ledger,
        "soil_1_competitors": soil1_competitors,
        "covered_fixed_cap_rebuilt": covered_final,
        "second_heterogeneous_covered_fixed_cap_rebuilt": covered_second_final,
        "open_fixed_cap_rebuilt": open_final,
        "both_rebuilt_from_beginning": (covered_final["beginning_sha256"]
            == covered_final["rebuilt_from_beginning_sha256"]
            and covered_second_final["beginning_sha256"]
                == covered_second_final["rebuilt_from_beginning_sha256"]
            and open_final["beginning_sha256"] == open_final["rebuilt_from_beginning_sha256"])}


def build_condensation_owner_transaction(vector: dict[str, Any]) -> dict[str, Any]:
    """Credit accepted dew mass and energy through the complete owner envelope."""
    transaction_id = 20260814004
    primitive, result = vector["primitive_input"], vector["result"]
    config, state = primitive["configuration"], primitive["state"]
    vapor = result["detail"]["ground_vapor"]
    key = water_key(transaction_id=transaction_id,
        owner_id="land-surface-energy-v1", component="ground_surface",
        ofe_id="ofe-condensation", requesting_tile_id=config["tile_id"],
        surface_id="surface:ofe-condensation:open",
        surface_class=config["surface_class"], source_type="soil_layer_liquid",
        source_id=config["surface_source_id"], soil_layer_id="soil-1")
    zero = {"key": key, "amount_kg_m2_stand_ground": 0.0}
    authorization = {**copy.deepcopy(zero), "reason": "full_supply"}
    credit = {"transaction_id": transaction_id,
        "hydrology_owner_id": "hydrology-real-owner",
        "ofe_id": key["ofe_id"], "tile_id": key["requesting_tile_id"],
        "surface_id": key["surface_id"],
        "amount_kg_m2_stand_ground": vapor["condensation_credit_kg_m2_stand_ground"],
        "amount_basis": "kg_h2o_m-2_stand_ground_interval",
        "temperature_k": result["detail"]["surface_temperature_k"],
        "specific_liquid_enthalpy_j_kg": liquid_enthalpy(
            result["detail"]["surface_temperature_k"])}
    protocol = {"transaction_id": transaction_id,
        "hydrology_owner_id": "hydrology-real-owner",
        "beginning_snapshot_sha256": digest(state), "requests": [copy.deepcopy(zero)],
        "authorizations": [authorization], "finalized_uses": [copy.deepcopy(zero)],
        "condensation_credits": [credit]}
    beginning_stores = [{"resource_identity": list(resource_group_identity(key)),
        "amount_kg_m2_stand_ground": state["top_layer_liquid_kg_m2"]}]
    beginning_payloads = {"vegetation": {"occupancies": []},
        "hydrology": {"resource_stores": beginning_stores,
            "post_ingress_surface_mass_kg_m2_tile_ground":
                state["surface_liquid_kg_m2_tile"]},
        "land_surface_energy": {"surface_enthalpy_j_m2_tile_ground":
            state["surface_enthalpy_j_m2_tile"]},
        "soil_thermal": {"temperatures_k": state["soil_temperature_k"]},
        "biogeochemistry": {"mineral_n": {}, "material_receipts": []}}
    ground_heat = result["detail"]["soil_records"][0]["incoming_cn_w_m2"] * \
        primitive["forcing"]["dt_s"] * config["tile_fraction"]
    owner_ids = {"vegetation": "vegetation-v8", "hydrology": "hydrology-real-owner",
        "land_surface_energy": "land-surface-energy-v1", "soil_thermal": "soil-thermal-owner",
        "biogeochemistry": "biogeochemistry-owner"}
    owner_models = {"vegetation": "OPENWEPP_C3_WOODY_V8",
        "hydrology": "OPENWEPP_WATBAL_OWNER_V1",
        "land_surface_energy": "OPENWEPP_SNOW_FREE_LSE_V1",
        "soil_thermal": "OPENWEPP_SOIL_THERMAL_V1",
        "biogeochemistry": "OPENWEPP_BGC_OWNER_V1"}
    beginning = {kind: typed_owner_beginning(owner_ids[kind], owner_models[kind], payload)
        for kind, payload in beginning_payloads.items()}
    candidates = {kind: {"transaction_id": transaction_id, "owner_id": owner_ids[kind],
        "model_version": beginning[kind]["model_version"],
        "model_definition_sha256": beginning[kind]["model_definition_sha256"],
        "configuration_sha256": beginning[kind]["configuration_sha256"],
        "beginning_state_sha256": digest(beginning[kind]), "ending_state": {"pending": True}}
        for kind in owner_ids}
    candidates["hydrology"]["water_protocol_sha256"] = digest(protocol)
    proposal = {"transaction_id": transaction_id, "proposal_id": "dew-material-1",
        "donor": "vegetation-standing-dead", "receiver": "bgc-litter",
        "carbon_kg_m2_stand_ground": 8.0e-4,
        "nitrogen_kg_m2_stand_ground": 4.0e-5,
        "dry_material_kg_m2_stand_ground": 1.7e-3}
    receipt = {field: proposal[field] for field in ("transaction_id", "proposal_id",
        "receiver", "carbon_kg_m2_stand_ground", "nitrogen_kg_m2_stand_ground",
        "dry_material_kg_m2_stand_ground")}
    fraction = config["tile_fraction"]
    local_condensation = credit["amount_kg_m2_stand_ground"] / fraction
    configured_nodes = config["soil_nodes"]
    expected_node_ids = [f"thermal-{index + 1}"
        for index in range(len(configured_nodes))]
    if len(configured_nodes) != len(state["soil_temperature_k"]) \
            or len(configured_nodes) != len(
                result["candidate"]["soil_thermal"]["temperature_k"]) \
            or [node["layer_id"] for node in configured_nodes] != expected_node_ids:
        raise ValueError("condensation_soil_node_identity_or_order")
    layers = [{"layer_id": node["layer_id"], "tile_fraction": fraction,
        "areal_heat_capacity_j_m2_k": node["heat_capacity_j_m2_k"],
        "ground_heat_receipt_j_m2_stand_ground":
            (ending_temperature - beginning_temperature)
                * node["heat_capacity_j_m2_k"] * fraction}
        for beginning_temperature, ending_temperature, node in zip(
            state["soil_temperature_k"], result["candidate"]["soil_thermal"]["temperature_k"],
            configured_nodes, strict=True)]
    joins = {"water": protocol, "vegetation_component_operands": [],
        "ground_heat_operands": {"lse_outgoing_j_m2_stand_ground":
            math.fsum(layer["ground_heat_receipt_j_m2_stand_ground"] for layer in layers)},
        "material_operands": {"vegetation_proposals": [proposal],
            "bgc_constructed_receipts": [receipt]},
        "advection_operands": {"transaction_id": transaction_id,
            "ofe_id": "ofe-condensation", "tile_id": config["tile_id"],
            "tile_fraction": fraction,
            "beginning_mass_tile": state["surface_liquid_kg_m2_tile"],
            "pre_ingress_mass_tile": state["surface_liquid_kg_m2_tile"]
                + local_condensation,
            "pre_ingress_surface_enthalpy_j_m2_tile": result["candidate"]["lse"]
                ["surface_enthalpy_j_m2_tile"],
            "dry_capacity_j_m2_k": config["surface_dry_heat_capacity_j_m2_k"],
            "parcels": [], "infiltration_kg_m2_tile": 0.0,
            "runoff_kg_m2_tile": 0.0},
        "soil_thermal_operands": {"layers": layers,
            "infiltration_receiver_layer_id": layers[0]["layer_id"],
            "infiltration_enthalpy_receipt_basis": "j_m2_stand_ground_interval",
            "infiltration_enthalpy_receipt_j_m2_stand_ground": 0.0}}
    reconstructed = reconstruct_owner_endings(beginning, protocol, joins)
    for kind, ending_state in reconstructed.items():
        candidates[kind]["ending_state"] = ending_state
    validation = validate_owner_candidates(transaction_id, beginning, candidates, joins)
    return {"transaction_id": transaction_id, "beginning": beginning,
        "water_protocol": protocol, "condensation_energy_credit": credit,
        "candidates": candidates, "joins": joins, "validation": validation,
        "owner_receipts": owner_receipts(transaction_id, beginning, candidates, joins)}


def validate_supported_domain(forcing: dict[str, Any], surface_classes: list[str],
                              *, soil_frozen: bool = False,
                              soil_thawing: bool = False,
                              surface_class_count_per_tile: int = 1) -> None:
    if forcing["snow_present_at_beginning"] or forcing["snow_present_at_end"]:
        raise ValueError("LSEB-E-030:snow_present")
    if forcing["snow_terminal_payload_present"]:
        raise ValueError("LSEB-E-030:snow_terminal_payload")
    if not forcing["neutral_stability"]:
        raise ValueError("LSEB-E-030:nonneutral_stability")
    if not math.isfinite(forcing["reference_wind_m_s"]) or forcing["reference_wind_m_s"] <= 0.0:
        raise ValueError("LSEB-E-030:calm_or_nonfinite_wind")
    if any(value not in {"bare_mineral_soil", "forest_litter"} for value in surface_classes):
        raise ValueError("LSEB-E-030:unsupported_surface_class")
    if soil_frozen:
        raise ValueError("LSEB-E-030:frozen_soil")
    if soil_thawing:
        raise ValueError("LSEB-E-030:thawing_soil")
    if surface_class_count_per_tile != 1:
        raise ValueError("LSEB-E-030:multiple_surface_classes_per_tile")
    for parcel in [*forcing["precipitation_parcels"], *forcing["runon_parcels"]]:
        if parcel["amount_kg_m2_destination_tile_ground"] > 0.0 and (
                "temperature_k" not in parcel or "specific_liquid_enthalpy_j_kg" not in parcel):
            raise ValueError("LSEB-E-030:missing_liquid_enthalpy")


def executed_validation_failures(base_forcing: dict[str, Any]) -> dict[str, Any]:
    """Execute each domain mutation through the typed validator."""
    mutations: dict[str, tuple[Callable[[dict[str, Any]], None], dict[str, Any]]] = {
        "snow_present": (lambda value: value.__setitem__("snow_present_at_beginning", True), {}),
        "snow_terminal": (lambda value: value.__setitem__("snow_terminal_payload_present", True), {}),
        "calm_wind": (lambda value: value.__setitem__("reference_wind_m_s", 0.0), {}),
        "nonneutral": (lambda value: value.__setitem__("neutral_stability", False), {}),
        "frozen_soil": (lambda value: None, {"soil_frozen": True}),
        "thawing_soil": (lambda value: None, {"soil_thawing": True}),
        "multiple_surface_classes_per_tile": (lambda value: None,
            {"surface_class_count_per_tile": 2}),
        "missing_ingress_enthalpy": (lambda value: value["precipitation_parcels"].append({
            "parcel_kind": "precipitation", "parcel_id": "bad-rain",
            "source_owner_id": "meteorology", "source_ofe_id": "atmosphere",
            "source_tile_id": "atmosphere", "destination_ofe_id": "ofe-1",
            "destination_tile_id": "open", "start_s": 0.0, "end_s": 1800.0,
            "amount_kg_m2_destination_tile_ground": 1.0,
            "temperature_provider": "harder_pomeroy_hourly"}), {}),
    }
    beginning = {"vegetation": {"state": 1}, "hydrology": {"state": 1},
        "land_surface_energy": {"state": 1}, "soil_thermal": {"state": 1},
        "biogeochemistry": {"state": 1}, "envelope": {"pending": []}}
    beginning_hashes = {key: digest(value) for key, value in beginning.items()}
    results = {}
    for name, (mutation, domain) in mutations.items():
        attempted = copy.deepcopy(base_forcing)
        mutation(attempted)
        try:
            validate_supported_domain(attempted, ["bare_mineral_soil"], **domain)
        except ValueError as error:
            after = {key: digest(value) for key, value in beginning.items()}
            results[name] = {"accepted": False, "typed_failure": str(error),
                "candidate": None, "rollback_before": beginning_hashes,
                "rollback_after": after, "rollback_exact": after == beginning_hashes}
        else:
            raise AssertionError(f"failure mutation accepted:{name}")
    return results


def executed_protocol_poisons(authoritative: dict[str, Any]) -> dict[str, Any]:
    """Execute counterfactual D/A/F branches through independent validation."""
    expected_requests = digest(authoritative["requests"])
    expected_authorizations = digest(authoritative["authorizations"])
    expected_final = digest(authoritative["finalized_uses"])
    def strict(record: dict[str, Any]) -> None:
        validate_water_protocol(record)
        if digest(record["requests"]) != expected_requests:
            raise ValueError("immutable_potential_request_mismatch")
        if digest(record["authorizations"]) != expected_authorizations:
            raise ValueError("single_authorization_mismatch")
        if digest(record["finalized_uses"]) != expected_final:
            raise ValueError("exact_joint_finalized_use_mismatch")
    mutations: dict[str, Callable[[dict[str, Any]], None]] = {
        "agricultural_pmet_canopy_shortfall_donation": lambda value:
            value["requests"][-1].__setitem__("amount_kg_m2_stand_ground",
                value["requests"][-1]["amount_kg_m2_stand_ground"] + 0.01),
        "request_inflation_after_final": lambda value: value["requests"][0].__setitem__(
            "amount_kg_m2_stand_ground", value["requests"][0]["amount_kg_m2_stand_ground"] + 0.01),
        "second_authorization": lambda value: value["authorizations"].append(
            copy.deepcopy(value["authorizations"][0])),
        "wrong_layer": lambda value: value["finalized_uses"][0]["key"].__setitem__(
            "soil_layer_id", "wrong-layer"),
        "wrong_occupancy": lambda value: value["finalized_uses"][0]["key"].__setitem__(
            "occupancy_id", "wrong-occupancy"),
        "tile_amount_as_stand_amount": lambda value: value["finalized_uses"][0].__setitem__(
            "amount_kg_m2_stand_ground", 2.0 * value["finalized_uses"][0]["amount_kg_m2_stand_ground"]),
        "tile_fraction_omitted": lambda value: value["finalized_uses"][0].__setitem__(
            "amount_kg_m2_stand_ground", value["finalized_uses"][0]["amount_kg_m2_stand_ground"] / 0.62),
        "tile_fraction_applied_twice": lambda value: value["finalized_uses"][0].__setitem__(
            "amount_kg_m2_stand_ground", value["finalized_uses"][0]["amount_kg_m2_stand_ground"] * 0.62),
    }
    results = {}
    for name, mutation in mutations.items():
        attempted = copy.deepcopy(authoritative)
        mutation(attempted)
        try:
            strict(attempted)
        except ValueError as error:
            results[name] = {"accepted": False, "typed_failure": str(error),
                "candidate": None, "authoritative_sha256": digest(authoritative),
                "attempted_sha256": digest(attempted)}
        else:
            raise AssertionError(f"poison accepted:{name}")
    # Exercise the F<A branch explicitly.  This protocol-only vector is not a
    # substitute for a constitutive solve; it proves that an unused maximum
    # authorization cannot be silently substituted for finalized use.
    lower_final = copy.deepcopy(authoritative)
    lower_index = next(index for index, item in enumerate(lower_final["authorizations"])
                       if item["amount_kg_m2_stand_ground"] > 0.0)
    lower_key = canonical(lower_final["authorizations"][lower_index]["key"])
    finalized_index = next(index for index, item in enumerate(lower_final["finalized_uses"])
                           if canonical(item["key"]) == lower_key)
    authorization_amount = lower_final["authorizations"][lower_index]["amount_kg_m2_stand_ground"]
    lower_final["finalized_uses"][finalized_index]["amount_kg_m2_stand_ground"] = 0.5 * authorization_amount
    validate_water_protocol(lower_final)
    expected_lower_sha = digest(lower_final["finalized_uses"])
    attempted = copy.deepcopy(lower_final)
    attempted["finalized_uses"][finalized_index]["amount_kg_m2_stand_ground"] = authorization_amount
    try:
        validate_water_protocol(attempted)
        if digest(attempted["finalized_uses"]) != expected_lower_sha:
            raise ValueError("authorization_substituted_for_exact_finalized_use")
    except ValueError as error:
        results["authorization_as_finalized_use_when_f_less_than_a"] = {
            "accepted": False, "typed_failure": str(error), "candidate": None,
            "valid_lower_final_protocol_sha256": digest(lower_final),
            "attempted_sha256": digest(attempted)}
    else:
        raise AssertionError("authorization_as_finalized_use_poison accepted")
    return results


def executed_component_poisons(mandatory: dict[str, Any], ingress: dict[str, Any],
                               owner_transaction: dict[str, Any]) -> dict[str, Any]:
    """Execute physical counterfactual records through exact operand validation."""
    covered = mandatory["covered_column"]
    open_day = mandatory["open_bare_day"]
    condensation = mandatory["supported_condensation"]
    records = {
        "radiation": copy.deepcopy(covered), "open_energy": copy.deepcopy(open_day),
        "condensation": copy.deepcopy(condensation), "ingress": copy.deepcopy(ingress),
        "candidates": copy.deepcopy(owner_transaction)}

    def validate_radiation(record: dict[str, Any]) -> None:
        primitive, detail = record["primitive_input"], record["result"]["detail"]
        terminal = detail["ground_terminal_shortwave_by_band_direction_w_m2_tile"]
        shortwave = detail["whole_column_shortwave"]
        expected_terminal = shortwave["terminal_by_band_direction_w_m2_tile"]
        if set(terminal) != {"direct_vis", "diffuse_vis", "direct_nir", "diffuse_nir"} \
                or terminal != expected_terminal:
            raise ValueError("shortwave_band_direction_identity")
        expected_layer_ids = [row["occupancy_id"] for row in primitive["occupancies"]]
        if [row["occupancy_id"] for row in shortwave["layers"]] != expected_layer_ids:
            raise ValueError("shortwave_occupancy_identity")
        reconstructed_terminal = {}
        for band, directions in shortwave["by_band"].items():
            albedo = primitive["ground_config"][
                "ground_surface_albedo_vis" if band == "VIS" else
                "ground_surface_albedo_nir"]
            for direction, flow in directions.items():
                for occupancy in flow["occupancies"]:
                    component = occupancy["results"]
                    if not math.isclose(component["absorbed_plant"], math.fsum([
                            component["absorbed_leaf_sun"], component["absorbed_leaf_shade"],
                            component["absorbed_stem"]]), rel_tol=0.0, abs_tol=ENERGY_TOL):
                        raise ValueError("leaf_stem_shortwave_owner_closure")
                plant = math.fsum(occupancy["results"]["absorbed_plant"]
                    for occupancy in flow["occupancies"])
                incident = flow["incident_direct"] + flow["incident_diffuse"]
                closure = incident - flow["top_reflected"] - plant - flow["ground_absorbed"]
                expected_ground = (flow["terminal_direct"] + flow["terminal_diffuse"]) \
                    * (1.0 - albedo)
                if abs(closure) > ENERGY_TOL or not math.isclose(
                        flow["ground_absorbed"], expected_ground,
                        rel_tol=0.0, abs_tol=ENERGY_TOL):
                    raise ValueError("shortwave_directional_closure")
            lower = band.lower()
            reconstructed_terminal[f"direct_{lower}"] = directions["direct"]["terminal_direct"]
            reconstructed_terminal[f"diffuse_{lower}"] = directions["direct"][
                "terminal_diffuse"] + directions["diffuse"]["terminal_diffuse"]
        if terminal != reconstructed_terminal:
            raise ValueError("shortwave_terminal_reconstruction")
        occupancies = primitive["occupancies"]
        solved = detail["occupancies"]
        if len(solved) != len(occupancies):
            raise ValueError("longwave_occupancy_cardinality")
        down = occupancies[0]["case"]["canopy_longwave"]["atmospheric_down_w_m2"]
        layers = []
        executed_leaf_areas = {row["occupancy_id"]: (row["results"]["leaf_sun_area"],
            row["results"]["leaf_shade_area"])
            for row in shortwave["by_band"]["VIS"]["direct"]["occupancies"]}
        for source, result in zip(occupancies, solved, strict=True):
            case = source["case"]
            wet = (min(case["gas_energy"]["canopy_liquid_kg_m2_tile"], 0.08) / 0.08) \
                ** (2.0 / 3.0)
            sun_area, shade_area = executed_leaf_areas[source["occupancy_id"]]
            areas = [sun_area * (1.0 - wet), shade_area * (1.0 - wet),
                wet * (sun_area + shade_area
                    + case["gas_energy"]["stem_area"]),
                (1.0 - wet) * case["gas_energy"]["stem_area"]]
            weights = [area / math.fsum(areas) for area in areas]
            temps_map = result["hydraulic_and_component"]["component_temperatures_k"]
            temps = [temps_map[name] for name in
                ("sun_leaf", "shade_leaf", "wet_surface", "dry_stem")]
            tau = math.exp(-case["canopy_longwave"]["extinction_m2_plant_m2_ground"]
                * case["canopy_longwave"]["clumping_index"]
                * (case["parameters"]["lai"] + case["parameters"]["sai"]))
            emission = math.fsum(weight * SIGMA * temperature**4
                for weight, temperature in zip(weights, temps, strict=True))
            layers.append({"tau": tau, "weights": weights, "temps": temps,
                "emission": emission, "down": down})
            down = tau * down + (1.0 - tau) * emission
        ground_temperature = detail["ground"]["ground_temperature_k"]
        up = SIGMA * ground_temperature**4
        for layer in reversed(layers):
            layer["up_bottom"] = up
            up = layer["tau"] * up + (1.0 - layer["tau"]) * layer["emission"]
        expected_receipts = []
        for layer in layers:
            components = [weight * (1.0 - layer["tau"])
                * (layer["down"] + layer["up_bottom"])
                - 2.0 * weight * (1.0 - layer["tau"]) * SIGMA * temperature**4
                for weight, temperature in zip(layer["weights"], layer["temps"], strict=True)]
            expected_receipts.append({"tau": layer["tau"],
                "component_net_w_m2_tile": components,
                "down_top_w_m2": layer["down"], "up_bottom_w_m2": layer["up_bottom"]})
        actual = detail["whole_column_longwave"]
        receipts_close = len(actual["occupancy_receipts"]) == len(expected_receipts)
        if receipts_close:
            for observed, expected in zip(actual["occupancy_receipts"],
                    expected_receipts, strict=True):
                receipts_close = receipts_close and all(math.isclose(observed[field],
                    expected[field], rel_tol=0.0, abs_tol=ENERGY_TOL)
                    for field in ("tau", "down_top_w_m2", "up_bottom_w_m2")) \
                    and len(observed["component_net_w_m2_tile"]) == len(
                        expected["component_net_w_m2_tile"]) \
                    and all(math.isclose(left, right, rel_tol=0.0, abs_tol=ENERGY_TOL)
                        for left, right in zip(observed["component_net_w_m2_tile"],
                            expected["component_net_w_m2_tile"], strict=True))
        if not receipts_close or not math.isclose(actual["terminal_down_w_m2_tile"], down,
                rel_tol=0.0, abs_tol=ENERGY_TOL) or not math.isclose(
                    actual["top_up_w_m2_tile"], up, rel_tol=0.0, abs_tol=ENERGY_TOL):
            raise ValueError("canopy_ground_longwave_reconstruction")
        ground_net = down - SIGMA * ground_temperature**4
        closure = occupancies[0]["case"]["canopy_longwave"]["atmospheric_down_w_m2"] \
            - up - ground_net - math.fsum(math.fsum(row["component_net_w_m2_tile"])
                for row in expected_receipts)
        if not math.isclose(actual["ground_net_w_m2_tile"], ground_net,
                rel_tol=0.0, abs_tol=ENERGY_TOL) or abs(closure) > ENERGY_TOL:
            raise ValueError("longwave_component_closure")

    def validate_open_energy(record: dict[str, Any]) -> None:
        detail = record["result"]["detail"]
        operands = detail["surface_operands_w_m2"]
        if set(operands) != {"shortwave", "longwave", "sensible", "vapor_energy",
                "ground_heat_cn", "storage"}:
            raise ValueError("surface_energy_operand_set")
        residual = math.fsum([operands["shortwave"], operands["longwave"],
            -operands["sensible"], -operands["vapor_energy"],
            -operands["ground_heat_cn"], -operands["storage"]])
        if residual != detail["raw_residuals"][0] \
                or abs(residual) > detail["tolerances"][0]:
            raise ValueError("surface_energy_closure")

    def validate_condensation(record: dict[str, Any]) -> None:
        primitive, detail = record["primitive_input"], record["result"]["detail"]
        vapor = detail["ground_vapor"]
        fraction = primitive["configuration"]["tile_fraction"]
        interval = primitive["forcing"]["dt_s"]
        expected_credit = -vapor["q_final_kg_m2_tile_s"] * fraction * interval
        if vapor["branch"] != "condensation" or vapor["q_final_kg_m2_tile_s"] >= 0.0 \
                or vapor["q_law_kg_m2_tile_s"] != vapor["q_final_kg_m2_tile_s"] \
                or vapor["request_kg_m2_stand_ground"] != 0.0 \
                or vapor["finalized_use_kg_m2_stand_ground"] != 0.0 \
                or vapor["condensation_credit_kg_m2_stand_ground"] != expected_credit:
            raise ValueError("condensation_mass_sign_identity")

    def validate_ingress(record: dict[str, Any]) -> None:
        if [row["process"] for row in record["crossings"]] != ["infiltration", "runoff"] \
                or [row["thermal_receiver"] for row in record["crossings"]] \
                    != ["soil_thermal", "accepted_downstream_or_outlet"]:
            raise ValueError("advected_receiver_identity")
        mass = record["pre_ingress_mass_kg_m2_tile_ground"] \
            + record["ingress_mass_kg_m2_tile_ground"] \
            - math.fsum(row["amount_kg_m2_tile_ground"] for row in record["crossings"]) \
            - record["ending_mass_kg_m2_tile_ground"]
        energy = record["pre_ingress_surface_enthalpy_j_m2_tile_ground"] \
            + record["ingress_energy_j_m2_tile_ground"] \
            - math.fsum(row["amount_j_m2_tile_ground"] for row in record["crossings"]) \
            - record["ending_dry_body_enthalpy_j_m2_tile_ground"] \
            - record["ending_liquid_enthalpy_j_m2_tile_ground"]
        if abs(mass) > MASS_TOL or abs(energy) > ENERGY_TOL:
            raise ValueError("post_ingress_mass_energy_closure")

    validators: dict[str, Callable[[Any], None]] = {"radiation": validate_radiation,
        "open_energy": validate_open_energy, "condensation": validate_condensation,
        "ingress": validate_ingress,
        "candidates": lambda record: validate_owner_candidates(record["transaction_id"],
            record["beginning"], record["candidates"], record["joins"])}
    for kind, record in records.items():
        validators[kind](record)
    mutations: dict[str, tuple[str, Callable[[Any], None], str]] = {
        "prescribed_stale_upward_longwave": ("radiation",
            lambda value: value["result"]["detail"]["whole_column_longwave"].__setitem__("top_up_w_m2_tile",
                value["result"]["detail"]["whole_column_longwave"]["top_up_w_m2_tile"] + 1.0), "stale_longwave_operand"),
        "omitted_canopy_ground_longwave": ("radiation",
            lambda value: value["result"]["detail"]["whole_column_longwave"]
                ["occupancy_receipts"].pop(),
            "missing_longwave_receipt"),
        "duplicated_canopy_ground_longwave": ("radiation",
            lambda value: value["result"]["detail"]["whole_column_longwave"]
                ["occupancy_receipts"].append(copy.deepcopy(value["result"]["detail"]
                    ["whole_column_longwave"]["occupancy_receipts"][0])),
            "duplicate_longwave_receipt"),
        "direct_diffuse_swap": ("radiation", lambda value: (
            value["result"]["detail"]["ground_terminal_shortwave_by_band_direction_w_m2_tile"]
                .__setitem__("direct_vis", value["result"]["detail"]
                    ["ground_terminal_shortwave_by_band_direction_w_m2_tile"]["diffuse_vis"]),
            value["result"]["detail"]["ground_terminal_shortwave_by_band_direction_w_m2_tile"]
                .__setitem__("diffuse_vis", records["radiation"]["primitive_input"]
                    ["ground_config"]["ground_terminal_shortwave_by_band_direction_w_m2_tile"]
                    ["direct_vis"])),
            "direct_diffuse_identity"),
        "vis_nir_swap": ("radiation", lambda value: (
            value["result"]["detail"]["ground_terminal_shortwave_by_band_direction_w_m2_tile"]
                .__setitem__("direct_vis", value["result"]["detail"]
                    ["ground_terminal_shortwave_by_band_direction_w_m2_tile"]["direct_nir"]),
            value["result"]["detail"]["ground_terminal_shortwave_by_band_direction_w_m2_tile"]
                .__setitem__("direct_nir", records["radiation"]["primitive_input"]
                    ["ground_config"]["ground_terminal_shortwave_by_band_direction_w_m2_tile"]
                    ["direct_vis"])),
            "vis_nir_identity"),
        "leaf_stem_ground_radiation_swap": ("radiation",
            lambda value: value["result"]["detail"]["whole_column_longwave"]
                ["occupancy_receipts"][0]
                ["component_net_w_m2_tile"].reverse(), "component_radiation_owner_identity"),
        "latent_heat_applied_twice": ("open_energy",
            lambda value: value["result"]["detail"]["surface_operands_w_m2"].__setitem__(
                "vapor_energy", 2.0 * value["result"]["detail"]
                    ["surface_operands_w_m2"]["vapor_energy"]),
            "latent_mass_energy_join"),
        "condensation_sign_reversal": ("condensation",
            lambda value: value["result"]["detail"]["ground_vapor"].__setitem__(
                "q_final_kg_m2_tile_s", abs(value["result"]["detail"]
                    ["ground_vapor"]["q_final_kg_m2_tile_s"])), "condensation_sign_identity"),
        "negative_flux_zero_clipping": ("condensation",
            lambda value: value["result"]["detail"]["ground_vapor"].__setitem__(
                "q_final_kg_m2_tile_s", 0.0),
            "signed_vapor_flux_identity"),
        "precipitation_enthalpy_omitted": ("ingress",
            lambda value: value["crossings"][0].__setitem__("amount_j_m2_tile_ground", 0.0),
            "advected_energy_identity"),
        "runoff_infiltration_advection_swap": ("ingress",
            lambda value: value["crossings"].reverse(), "advected_receiver_identity"),
        "ground_heat_consumed_twice": ("candidates",
            lambda value: value["joins"]["ground_heat_operands"].__setitem__(
                "lse_outgoing_j_m2_stand_ground", 2.0 * value["joins"]
                    ["ground_heat_operands"]["lse_outgoing_j_m2_stand_ground"]),
            "equal_opposite_ground_heat_identity"),
        "legacy_surtmp_as_persistent_state": ("candidates",
            lambda value: value["candidates"]["land_surface_energy"]["ending_state"].__setitem__(
                "surface_enthalpy_j_m2_tile_ground", 295.0),
            "enthalpy_state_identity"),
        "producer_supplied_residual": ("open_energy",
            lambda value: value["result"]["detail"]["surface_operands_w_m2"].__setitem__(
                "producer_residual", 0.0),
            "producer_residual_prohibited"),
        "vegetation_candidate_body_mutation": ("candidates",
            lambda value: value["candidates"]["vegetation"]["ending_state"]
                .__setitem__("mutated", True), "vegetation_candidate_body_reconstruction"),
        "hydrology_candidate_body_mutation": ("candidates",
            lambda value: value["candidates"]["hydrology"]["ending_state"]
                .__setitem__("mutated", True), "hydrology_candidate_body_reconstruction"),
        "lse_candidate_body_mutation": ("candidates",
            lambda value: value["candidates"]["land_surface_energy"]["ending_state"]
                .__setitem__("mutated", True), "lse_candidate_body_reconstruction"),
        "soil_thermal_candidate_body_mutation": ("candidates",
            lambda value: value["candidates"]["soil_thermal"]["ending_state"]
                .__setitem__("mutated", True), "soil_candidate_body_reconstruction"),
        "bgc_candidate_body_mutation": ("candidates",
            lambda value: value["candidates"]["biogeochemistry"]["ending_state"]
                .__setitem__("mutated", True), "bgc_candidate_body_reconstruction"),
        "copied_candidate_join": ("candidates",
            lambda value: value["joins"].__setitem__("vegetation", {
                "ending_state": copy.deepcopy(value["candidates"]["vegetation"]
                    ["ending_state"])}), "copied_candidate_join_prohibited"),
        "empty_material_tautology": ("candidates", lambda value: (
            value["joins"]["material_operands"].__setitem__("vegetation_proposals", []),
            value["joins"]["material_operands"].__setitem__(
                "bgc_constructed_receipts", [])), "empty_material_join_prohibited"),
        "empty_material_hash_equality": ("candidates",
            lambda value: value["joins"].__setitem__("material", {
                "vegetation_proposals_sha256": digest([]),
                "bgc_receipts_sha256": digest([])}), "material_hash_tautology_prohibited"),
        "soil_thermal_node_truncation": ("candidates",
            lambda value: value["joins"]["soil_thermal_operands"]["layers"].pop(),
            "soil_thermal_node_cardinality"),
        "infiltration_enthalpy_omitted_from_soil_node": ("candidates",
            lambda value: value["joins"]["soil_thermal_operands"].__setitem__(
                "infiltration_enthalpy_receipt_j_m2_stand_ground", 0.0),
            "infiltration_enthalpy_omission"),
        "infiltration_enthalpy_duplicated_in_soil_node": ("candidates",
            lambda value: value["joins"]["soil_thermal_operands"].__setitem__(
                "infiltration_enthalpy_receipt_j_m2_stand_ground", 2.0 * value["joins"]
                    ["soil_thermal_operands"]
                    ["infiltration_enthalpy_receipt_j_m2_stand_ground"]),
            "infiltration_enthalpy_duplication"),
        "infiltration_enthalpy_wrong_soil_node": ("candidates",
            lambda value: value["joins"]["soil_thermal_operands"].__setitem__(
                "infiltration_receiver_layer_id", value["joins"]
                    ["soil_thermal_operands"]["layers"][1]["layer_id"]),
            "infiltration_enthalpy_wrong_node"),
        "infiltration_enthalpy_wrong_area_basis": ("candidates",
            lambda value: value["joins"]["soil_thermal_operands"].__setitem__(
                "infiltration_enthalpy_receipt_basis", "j_m2_tile_ground_interval"),
            "infiltration_enthalpy_wrong_basis"),
    }
    results = {}
    for name, (kind, mutation, error_identity) in mutations.items():
        attempted = copy.deepcopy(records[kind])
        mutation(attempted)
        try:
            validators[kind](attempted)
        except ValueError as error:
            results[name] = {"accepted": False, "typed_failure": str(error),
                "candidate": None, "authoritative_sha256": digest(records[kind]),
                "attempted_sha256": digest(attempted)}
        else:
            raise AssertionError(f"component poison accepted:{name}")
    return results


def wrap_natural_failures(raw: dict[str, Any], owner_transaction: dict[str, Any]) -> dict[str, Any]:
    """Bind natural numerical failures to the complete five-owner envelope."""
    before = {kind: digest(state) for kind, state in owner_transaction["beginning"].items()}
    envelope = {"transaction_id": owner_transaction["transaction_id"],
        "owner_set": sorted(owner_transaction["beginning"]), "pending_requests": [],
        "pending_authorizations": [], "pending_finalized_uses": [],
        "pending_material_proposals": [], "pending_material_receipts": [],
        "diagnostics": {"accepted": False}}
    before["transaction_envelope"] = digest(envelope)
    wrapped = {}
    for name, record in raw.items():
        if record.get("accepted") is not False or record.get("candidate") is not None:
            raise ValueError(f"natural_failure_candidate_present:{name}")
        wrapped[name] = {**copy.deepcopy(record), "typed_failure": record["failure"],
            "owner_and_envelope_rollback_before": copy.deepcopy(before),
            "owner_and_envelope_rollback_after": copy.deepcopy(before),
            "rollback_exact": True}
    return wrapped


def bind_failure_diagnostics(failures: dict[str, Any], owner_transaction: dict[str, Any],
                             configuration_sha256: str,
                             beginning_state_sha256: str) -> dict[str, Any]:
    """Construct and schema-validate one complete diagnostics DTO per failure."""
    transaction_id = owner_transaction["transaction_id"]
    owner_ids = {kind: state["owner_id"]
        for kind, state in owner_transaction["beginning"].items()}
    owner_ids["envelope"] = "coupled-transaction-envelope"
    before = {kind: digest(state) for kind, state in owner_transaction["beginning"].items()}
    before["envelope"] = digest({"transaction_id": transaction_id,
        "owners": sorted(owner_transaction["beginning"]), "candidate": None})
    bound = {}
    for name, source in failures.items():
        record = copy.deepcopy(source)
        declared_failure = record.get("failure")
        natural = declared_failure in {"singular", "backtracking_limit", "iteration_limit"}
        raw_diag = record.get("diagnostics", {}) if natural else {}
        if natural:
            failure_code, failure_kind = {
                "singular": ("LSEB-E-034", "singular_pivot"),
                "backtracking_limit": ("LSEB-E-034", "backtracking_limit"),
                "iteration_limit": ("LSEB-E-034", "iteration_limit")}[declared_failure]
        else:
            failure_code, failure_kind = "LSEB-E-030", "unsupported_domain"
        residuals = []
        for index, row in enumerate(raw_diag.get("ordered_residuals", [])):
            raw_value = row.get("raw", row.get("residual"))
            tolerance = row["tolerance"]
            identity = row["identity"]
            unit = ("kg_m-2_s-1" if index < 6 or identity == "shared_canopy_air_vapor"
                else "w_m-2")
            residuals.append({"identity": identity, "raw": raw_value,
                "scale": max(1.0, abs(raw_value)), "tolerance": tolerance,
                "normalized": row["normalized"], "unit": unit})
        raw_steps = raw_diag.get("step_norms") or record.get("step_norms") or {}
        diagnostics = {"model_version": "OPENWEPP_SNOW_FREE_LSE_V1",
            "canonical_contract": "SC-LANDSURFACEENERGY-001@3",
            "model_definition_sha256": LSE_MODEL_DEFINITION_SHA256,
            "configuration_sha256": configuration_sha256,
            "beginning_state_sha256": beginning_state_sha256,
            "transaction_id": transaction_id,
            "ofe_id": raw_diag.get("ofe_id", "ofe-1"),
            "tile_id": raw_diag.get("tile_id", "forest"),
            "occupancy_id": raw_diag.get("occupancy_id") if natural else None,
            "pass": ("final_fixed_cap" if raw_diag.get("pass") == "capped"
                else raw_diag.get("pass", "potential")),
            "solve": ("joint_canopy_ground" if natural else "independent_closure"),
            "accepted": False, "failure_code": failure_code,
            "failure_kind": failure_kind,
            "iterations": int(raw_diag.get("iterations", record.get("iterations", 0))),
            "backtracking_count": int(raw_diag.get("backtracking_count",
                record.get("backtracking_count") or 0)),
            "ordered_residuals": residuals,
            "step_norms": {"temperature_k": raw_steps.get("temperature_k"),
                "humidity_kg_kg": raw_steps.get("humidity_kg_kg"),
                "ci_pa": raw_steps.get("ci_pa"),
                "hydraulic_mm": raw_steps.get("hydraulic_mm"),
                "beta": raw_steps.get("beta")},
            "active_bounds": sorted(raw_diag.get("active_bounds",
                record.get("active_bounds", []))),
            "active_water_caps": [value for value in raw_diag.get("active_water_caps",
                record.get("active_water_caps", [])) if isinstance(value, dict)],
            "bracket": None,
            "pivot_magnitude": raw_diag.get("pivot_magnitude"),
            "matrix_infinity_norm": raw_diag.get("matrix_norm"),
            "owner_rollback_hashes": [{"owner_kind": kind, "owner_id": owner_ids[kind],
                "before_sha256": before[kind], "after_sha256": before[kind]}
                for kind in ("vegetation", "hydrology", "land_surface_energy",
                    "biogeochemistry", "soil_thermal", "envelope")]}
        validation = validate_schema_instances({"diagnostics": diagnostics})["diagnostics"]
        record.update({"attempted_transaction_id": transaction_id,
            "attempted_transaction_lineage": {kind: transaction_id for kind in
                ("vegetation", "hydrology", "land_surface_energy",
                 "biogeochemistry", "soil_thermal", "envelope")},
            "transaction_id": transaction_id,
            "configuration_sha256": configuration_sha256,
            "state_sha256": beginning_state_sha256,
            "candidate": None, "diagnostics": diagnostics,
            "diagnostics_schema_validation": validation,
            "owner_and_envelope_rollback_before": copy.deepcopy(before),
            "owner_and_envelope_rollback_after": copy.deepcopy(before),
            "rollback_exact": True})
        bound[name] = record
    return bound


def executed_diagnostic_pairing_poisons(base: dict[str, Any]) -> dict[str, Any]:
    """Reject every incompatible code pairing for the emitted failure categories."""
    all_codes = [f"LSEB-E-{number:03d}" for number in range(30, 41)]
    allowed = {"unsupported_domain": "LSEB-E-030",
        "singular_pivot": "LSEB-E-034", "backtracking_limit": "LSEB-E-034",
        "iteration_limit": "LSEB-E-034"}
    results = {}
    for failure_kind, allowed_code in allowed.items():
        for incompatible_code in all_codes:
            if incompatible_code == allowed_code:
                continue
            attempted = copy.deepcopy(base)
            attempted.update({"accepted": False, "failure_kind": failure_kind,
                "failure_code": incompatible_code})
            name = f"{failure_kind}_with_{incompatible_code.lower().replace('-', '_')}"
            try:
                validate_schema_instances({"diagnostics": attempted})
            except ValueError as error:
                results[name] = {"accepted": False, "candidate": None,
                    "typed_failure": str(error), "failure_kind": failure_kind,
                    "attempted_failure_code": incompatible_code,
                    "required_failure_code": allowed_code}
            else:
                raise AssertionError(f"diagnostic pairing poison accepted:{name}")
    if len(results) != 40:
        raise AssertionError(f"diagnostic pairing poison cardinality:{len(results)}")
    return results


def build_authority_vectors() -> dict[str, Any]:
    """Execute the exact model reductions and complete owner transaction."""
    global CORE
    CORE = load_joint_core()
    expanded = CORE.build_expanded_joint_vectors()
    mandatory = CORE.build_mandatory_scenario_vectors()
    required_mandatory = {"open_bare_day", "open_bare_night", "covered_column",
        "dry_litter_covered", "wet_litter_covered", "zero_shortwave",
        "longwave_cooling", "ground_to_canopy_longwave_feedback",
        "ground_sensible_vapor_feedback", "wet_canopy_evaporation",
        "supported_condensation", "full_surface_top_layer_caps",
        "partial_surface_cap", "partial_top_layer_cap", "dry_frozen_zero_sources",
        "hydraulic_redistribution_rejection", "ground_heat_sign_reversal",
        "storage", "alternate_starts"}
    required_mandatory.add("ground_albedo_lower_boundary_feedback")
    required_mandatory.add("frozen_ground_cap_centered_probe")
    if not required_mandatory.issubset(mandatory):
        raise AssertionError(f"mandatory scenario family missing:{required_mandatory - set(mandatory)}")
    transaction = execute_covered_water_transaction(CORE, rank_count=2,
        ofe_id="ofe-1", tile_id="forest", tile_fraction=0.62,
        transaction_id=20260814001, supply_fraction=0.78)
    shared_layer_competition = execute_shared_layer_root_ground_competition(CORE)
    condensation_owner = build_condensation_owner_transaction(
        mandatory["supported_condensation"])
    protocol = transaction["water_protocol"]
    schema_instances = strict_positive_instances(protocol)
    forcing = schema_instances["forcing"]
    validate_supported_domain(forcing, ["bare_mineral_soil", "forest_litter"])

    # Exact equilibrium reduction: Cdry=W=U=0, with no fabricated stored energy.
    equilibrium_bundle = CORE.build_open_bare_soil_case(2)
    equilibrium_config = {
        "surface_class": "bare_mineral_soil", "reference_height_m": 20.0,
        "roughness_momentum_m": 0.12, "roughness_heat_m": 0.015,
        "roughness_vapor_m": 0.01, "soil_nodes": equilibrium_bundle["configuration"]["soil_nodes"],
        "terminal_shortwave_w_m2_tile": equilibrium_bundle["configuration"]
            ["surface_terminal_shortwave_by_band_direction_w_m2_tile"],
        "surface_vis_albedo": 0.0, "surface_nir_albedo": 0.0,
        "surface_emissivity": 1.0, "surface_dry_heat_capacity_j_m2_k": 0.0,
        "surface_depth_m": equilibrium_bundle["configuration"]["surface_depth_m"],
        "surface_conductivity_w_m_k": equilibrium_bundle["configuration"]
            ["surface_conductivity_w_m_k"], "litter_capacity_kg_m2_tile": 6.0,
        "top_layer_depth_m": equilibrium_bundle["configuration"]["soil_nodes"][0]["depth_m"],
        "soil_thermal_conductivity_w_m_k": equilibrium_bundle["configuration"]
            ["soil_nodes"][0]["conductivity_w_m_k"], "soil_porosity": 0.46,
        "soil_saturated_matric_potential_mm": -120.0,
        "soil_clapp_hornberger_b": 4.05, "soil_theta_initial": 0.22}
    equilibrium_forcing = {"interval_s": 1800.0, "air_temperature_k": 294.0,
        "air_specific_humidity_kg_kg": 0.0095, "air_pressure_pa": 93000.0,
        "reference_wind_m_s": 2.4, "atmospheric_downward_longwave_w_m2": 335.0}
    equilibrium_state = {"surface_liquid_kg_m2_tile": 0.0,
        "surface_enthalpy_j_m2_tile": 0.0, "surface_temperature_warm_start_k": 295.0,
        "soil_temperature_k": equilibrium_bundle["state"]["soil_temperature_k"],
        "top_layer_liquid_kg_m2": equilibrium_bundle["state"]["top_layer_liquid_kg_m2"],
        "top_layer_ice_kg_m2": 0.0}
    equilibrium = solve_open_surface(equilibrium_forcing, equilibrium_config,
        equilibrium_state, cap_rate=0.0)
    if not equilibrium["accepted"] or equilibrium["components"]["surface_storage_mode"] \
            != "equilibrium_zero" or equilibrium["candidate"]["land_surface_energy"] \
            ["surface_enthalpy_j_m2_tile"] != 0.0:
        raise AssertionError("equilibrium_zero_branch_not_executed")

    # Post-final ingress uses an accepted hydrology partition and updates both
    # retained LSE enthalpy and the soil-thermal infiltration receipt.
    final_ground = transaction["final"]["detail"]["ground"]
    beginning_surface_mass = transaction["authority_inputs"]["ground_state"] \
        ["surface_liquid_kg_m2_tile"]
    final_ground_vapor = final_ground["ground_vapor"]["q_final_kg_m2_tile_s"]
    pre_mass = beginning_surface_mass - max(final_ground_vapor, 0.0) * 1800.0 \
        + max(-final_ground_vapor, 0.0) * 1800.0
    rain = {"parcel_kind": "precipitation", "parcel_id": "rain-1",
        "source_owner_id": "meteorology", "source_ofe_id": "atmosphere",
        "source_tile_id": "atmosphere", "destination_ofe_id": "ofe-1",
        "destination_tile_id": "forest", "start_s": 0.0, "end_s": 1800.0,
        "amount_kg_m2_destination_tile_ground": 1.2,
        "temperature_k": 281.2, "specific_liquid_enthalpy_j_kg": liquid_enthalpy(281.2),
        "temperature_provider": "harder_pomeroy_hourly",
        "source_state_sha256": digest({"meteorology": "rain-1"})}
    runon = {"parcel_kind": "routed_runon", "parcel_id": "runon-1",
        "source_owner_id": "hydrology-real-owner", "source_ofe_id": "ofe-0",
        "source_tile_id": "forest-upstream", "destination_ofe_id": "ofe-1",
        "destination_tile_id": "forest", "start_s": 240.0, "end_s": 960.0,
        "amount_kg_m2_destination_tile_ground": 0.6,
        "temperature_k": 289.6, "specific_liquid_enthalpy_j_kg": liquid_enthalpy(289.6),
        "temperature_provider": "accepted_upstream_outlet_parcel",
        "source_state_sha256": digest({"upstream": "accepted"})}
    ingress = apply_post_ingress(transaction_id=20260814001, ofe_id="ofe-1",
        tile_id="forest", tile_fraction=0.62,
        beginning_mass_tile=beginning_surface_mass,
        pre_ingress_mass_tile=pre_mass,
        pre_ingress_surface_enthalpy_j_m2_tile=transaction["final"]["candidate"]
            ["lse"]["surface_enthalpy_j_m2_tile"],
        dry_capacity_j_m2_k=transaction["authority_inputs"]["ground_config"]
            ["surface_dry_heat_capacity_j_m2_k"],
        parcels=[rain, runon], infiltration_kg_m2_tile=1.0, runoff_kg_m2_tile=0.25)
    beginning_stores = [{"resource_identity": row["resource_identity"],
        "amount_kg_m2_stand_ground": row["amount_kg_m2_stand_ground"]}
        for row in transaction["water_protocol"].get("beginning_stores", [])]
    if not beginning_stores:
        # The immutable arbitration snapshot is the source of supply; expose its
        # typed values rather than reconstructing them from authorization.
        beginning_stores = copy.deepcopy(transaction["beginning_resource_stores"])
    beginning_payloads = {"vegetation": {"model": "OPENWEPP_C3_WOODY_V8",
            "column_sha256": transaction["beginning_column_sha256"]},
        "hydrology": {"snapshot": protocol["beginning_snapshot_sha256"],
            "resource_stores": beginning_stores,
            "post_ingress_surface_mass_kg_m2_tile_ground": beginning_surface_mass},
        "land_surface_energy": {"model": "OPENWEPP_SNOW_FREE_LSE_V1",
            "surface_enthalpy_j_m2_tile_ground": transaction["authority_inputs"]
                ["ground_state"]["surface_enthalpy_j_m2_tile"]},
        "soil_thermal": {"temperatures_k": transaction["authority_inputs"]
            ["ground_state"]["soil_temperature_k"]},
        "biogeochemistry": {"mineral_n": {}, "material_receipts": []}}
    owner_ids = {"vegetation": "vegetation-v8", "hydrology": "hydrology-real-owner",
        "land_surface_energy": "land-surface-energy-v1", "soil_thermal": "soil-thermal-owner",
        "biogeochemistry": "biogeochemistry-owner"}
    owner_models = {"vegetation": "OPENWEPP_C3_WOODY_V8",
        "hydrology": "OPENWEPP_WATBAL_OWNER_V1",
        "land_surface_energy": "OPENWEPP_SNOW_FREE_LSE_V1",
        "soil_thermal": "OPENWEPP_SOIL_THERMAL_V1",
        "biogeochemistry": "OPENWEPP_BGC_OWNER_V1"}
    beginning = {kind: typed_owner_beginning(owner_ids[kind], owner_models[kind], payload)
        for kind, payload in beginning_payloads.items()}
    ground_heat_stand = final_ground["ground_heat_cn_w_m2_tile"][0] * 1800.0 * 0.62
    owner_ids = {"vegetation": "vegetation-v8", "hydrology": "hydrology-real-owner",
        "land_surface_energy": "land-surface-energy-v1", "soil_thermal": "soil-thermal-owner",
        "biogeochemistry": "biogeochemistry-owner"}
    candidates = {kind: {"transaction_id": 20260814001, "owner_id": owner_ids[kind],
        "model_version": beginning[kind]["model_version"],
        "model_definition_sha256": beginning[kind]["model_definition_sha256"],
        "configuration_sha256": beginning[kind]["configuration_sha256"],
        "beginning_state_sha256": digest(beginning[kind]), "ending_state": {"pending": True}}
        for kind in owner_ids}
    candidates["hydrology"]["water_protocol_sha256"] = digest(protocol)
    proposal = {"transaction_id": 20260814001, "proposal_id": "main-material-1",
        "donor": "vegetation-standing-dead", "receiver": "bgc-litter",
        "carbon_kg_m2_stand_ground": 1.2e-3,
        "nitrogen_kg_m2_stand_ground": 6.0e-5,
        "dry_material_kg_m2_stand_ground": 2.6e-3}
    receipt = {field: proposal[field] for field in ("transaction_id", "proposal_id",
        "receiver", "carbon_kg_m2_stand_ground", "nitrogen_kg_m2_stand_ground",
        "dry_material_kg_m2_stand_ground")}
    soil_layers = [{"layer_id": f"soil-{index + 1}", "tile_fraction": 0.62,
        "areal_heat_capacity_j_m2_k": capacity,
        "ground_heat_receipt_j_m2_stand_ground":
            (ending_temperature - beginning_temperature) * capacity * 0.62}
        for index, (beginning_temperature, ending_temperature, capacity) in enumerate(zip(
            beginning["soil_thermal"]["state"]["temperatures_k"],
            transaction["final"]["candidate"]["soil_thermal"]["temperature_k"],
            (120000.0, 180000.0), strict=True))]
    joins = {"water": protocol,
        "vegetation_component_operands": copy.deepcopy(
            transaction["final"]["candidate"]["vegetation"]),
        "ground_heat_operands": {"lse_outgoing_j_m2_stand_ground": ground_heat_stand},
        "material_operands": {"vegetation_proposals": [proposal],
            "bgc_constructed_receipts": [receipt]},
        "advection_operands": {"transaction_id": 20260814001, "ofe_id": "ofe-1",
            "tile_id": "forest", "tile_fraction": 0.62,
            "beginning_mass_tile": beginning_surface_mass,
            "pre_ingress_mass_tile": pre_mass,
            "pre_ingress_surface_enthalpy_j_m2_tile": transaction["final"]["candidate"]
                ["lse"]["surface_enthalpy_j_m2_tile"],
            "dry_capacity_j_m2_k": transaction["authority_inputs"]["ground_config"]
                ["surface_dry_heat_capacity_j_m2_k"],
            "parcels": [copy.deepcopy(rain), copy.deepcopy(runon)],
            "infiltration_kg_m2_tile": 1.0, "runoff_kg_m2_tile": 0.25},
        "soil_thermal_operands": {"layers": soil_layers,
            "infiltration_receiver_layer_id": soil_layers[0]["layer_id"],
            "infiltration_enthalpy_receipt_basis": "j_m2_stand_ground_interval",
            "infiltration_enthalpy_receipt_j_m2_stand_ground": ingress[
                "soil_thermal_infiltration_receipt_j_m2_stand_ground"]}}
    reconstructed_endings = reconstruct_owner_endings(beginning, protocol, joins)
    for kind, ending_state in reconstructed_endings.items():
        candidates[kind]["ending_state"] = ending_state
    owner_transaction = {"transaction_id": 20260814001, "beginning": beginning,
        "candidates": candidates, "joins": joins}
    owner_validation = validate_owner_candidates(20260814001, beginning, candidates, joins)
    receipts = owner_receipts(20260814001, beginning, candidates, joins)
    upstream = execute_covered_water_transaction(CORE, rank_count=2,
        ofe_id="ofe-0", tile_id="upstream-forest", tile_fraction=1.0,
        transaction_id=20260814002, supply_fraction=1.0)
    upstream_ground = upstream["final"]["detail"]["ground"]
    upstream_beginning_mass = upstream["authority_inputs"]["ground_state"] \
        ["surface_liquid_kg_m2_tile"]
    upstream_q = upstream_ground["ground_vapor"]["q_final_kg_m2_tile_s"]
    upstream_pre_mass = upstream_beginning_mass - max(upstream_q, 0.0) * DT \
        + max(-upstream_q, 0.0) * DT
    upstream_rain = {**rain, "parcel_id": "rain-upstream",
        "destination_ofe_id": "ofe-0", "destination_tile_id": "upstream-forest",
        "amount_kg_m2_destination_tile_ground": 1.1}
    upstream_ingress = apply_post_ingress(transaction_id=20260814002, ofe_id="ofe-0",
        tile_id="upstream-forest", tile_fraction=1.0,
        beginning_mass_tile=upstream_beginning_mass, pre_ingress_mass_tile=upstream_pre_mass,
        pre_ingress_surface_enthalpy_j_m2_tile=upstream["final"]["candidate"]
            ["lse"]["surface_enthalpy_j_m2_tile"],
        dry_capacity_j_m2_k=upstream["authority_inputs"]["ground_config"]
            ["surface_dry_heat_capacity_j_m2_k"], parcels=[upstream_rain],
        infiltration_kg_m2_tile=0.4, runoff_kg_m2_tile=0.6)
    upstream_outlet = copy.deepcopy(upstream_ingress["crossings"][1])
    upstream_outlet.update({"route_id": "ofe-0:upstream-forest->ofe-1:forest",
        "interval_s": DT, "amount_basis": "kg_h2o_m-2_route_ground_interval",
        "source_ofe_area_m2": 120.0,
        "accepted_transaction_id": 20260814002,
        "source_state_sha256": digest(upstream["final"]["candidate"])})
    routed_extensive_mass = upstream_outlet["amount_kg_m2_tile_ground"] \
        * upstream_outlet["source_ofe_area_m2"]
    destination_ofe_area = 200.0
    routed_runon = {"parcel_kind": "routed_runon", "parcel_id": "route-0-to-1",
        "route_id": "ofe-0:upstream-forest->ofe-1:forest",
        "source_owner_id": "hydrology-real-owner", "source_ofe_id": "ofe-0",
        "source_tile_id": "upstream-forest", "destination_ofe_id": "ofe-1",
        "destination_tile_id": "forest", "interval_s": DT,
        "start_s": 0.0, "end_s": DT,
        "source_transaction_id": upstream_outlet["accepted_transaction_id"],
        "destination_transaction_id": 20260814001,
        "source_ofe_area_m2": upstream_outlet["source_ofe_area_m2"],
        "destination_ofe_area_m2": destination_ofe_area,
        "amount_basis": "kg_h2o_m-2_route_ground_interval",
        "amount_kg_m2_destination_tile_ground": routed_extensive_mass
            / destination_ofe_area,
        "temperature_k": TREF + upstream_outlet["specific_liquid_enthalpy_j_kg"] / CW,
        "specific_liquid_enthalpy_j_kg": upstream_outlet[
            "specific_liquid_enthalpy_j_kg"],
        "temperature_provider": "accepted_upstream_outlet_parcel",
        "source_state_sha256": upstream_outlet["source_state_sha256"]}
    route_join = validate_multi_ofe_route(upstream_outlet, routed_runon)
    routed_ingress = apply_post_ingress(transaction_id=20260814001, ofe_id="ofe-1",
        tile_id="forest", tile_fraction=0.62, beginning_mass_tile=beginning_surface_mass,
        pre_ingress_mass_tile=pre_mass,
        pre_ingress_surface_enthalpy_j_m2_tile=transaction["final"]["candidate"]
            ["lse"]["surface_enthalpy_j_m2_tile"],
        dry_capacity_j_m2_k=transaction["authority_inputs"]["ground_config"]
            ["surface_dry_heat_capacity_j_m2_k"], parcels=[rain, routed_runon],
        infiltration_kg_m2_tile=1.0, runoff_kg_m2_tile=0.25)
    multi_ofe = {"upstream": upstream, "upstream_post_ingress": upstream_ingress,
        "upstream_typed_runoff_crossing": upstream_outlet,
        "downstream_runon_parcel": routed_runon,
        "route_join": route_join, "downstream_ingress": routed_ingress}
    routed_forcing_parcel_fields = (
        "parcel_kind", "parcel_id", "source_owner_id", "source_ofe_id",
        "source_tile_id", "destination_ofe_id", "destination_tile_id",
        "start_s", "end_s",
        "amount_kg_m2_destination_tile_ground", "temperature_k",
        "specific_liquid_enthalpy_j_kg", "temperature_provider",
        "source_state_sha256")
    routed_forcing_parcel = {field: routed_runon[field]
        for field in routed_forcing_parcel_fields}
    routed_forcing = copy.deepcopy(schema_instances["forcing"])
    routed_forcing.update({"transaction_id": 20260814001,
        "precipitation_parcels": [], "runon_parcels": [routed_forcing_parcel]})
    routed_forcing["forcing_sha256"] = digest(
        {**routed_forcing, "forcing_sha256": ""})
    routed_forcing_validation = validate_schema_instances(
        {"forcing": routed_forcing})["forcing"]
    condensation_protocol_validation = validate_schema_instances(
        {"water_protocol": condensation_owner["water_protocol"]})["water_protocol"]
    condensation_owner["water_protocol_schema_validation"] = \
        condensation_protocol_validation
    multi_ofe["routed_forcing"] = routed_forcing
    multi_ofe["routed_forcing_schema_validation"] = routed_forcing_validation

    # Replace the scaffold envelope with the exact physically constructed
    # five-owner candidate transaction. Candidate bodies and primitive joins
    # were validated above before these receipts were issued.
    coupled = copy.deepcopy(schema_instances["coupled_transaction"])
    coupled["transaction_id"] = owner_transaction["transaction_id"]
    coupled["lse_configuration_sha256"] = beginning["land_surface_energy"][
        "configuration_sha256"]
    coupled["water_protocol"] = protocol
    coupled["candidate_owner_bodies"] = copy.deepcopy(candidates)
    coupled["candidate_owner_receipts"] = receipts
    coupled["beginning_vegetation_owner"].update({
        "model_definition_sha256": beginning["vegetation"]["model_definition_sha256"],
        "configuration_sha256": beginning["vegetation"]["configuration_sha256"],
        "state_sha256": beginning["vegetation"]["state_sha256"],
        "last_accepted_transaction_id": owner_transaction["transaction_id"] - 1})
    actual_lse_state = copy.deepcopy(schema_instances["state"])
    actual_lse_state["configuration_sha256"] = beginning["land_surface_energy"][
        "configuration_sha256"]
    actual_lse_state["tiles"][0].update({"ofe_id": "ofe-1", "tile_id": "forest",
        "surface_enthalpy_j_m2_tile_ground": beginning["land_surface_energy"]["state"]
            ["surface_enthalpy_j_m2_tile_ground"],
        "surface_temperature_warm_start_k": transaction["authority_inputs"]
            ["ground_state"]["surface_temperature_warm_start_k"]})
    # The strict positive state is a distinct, fully projected forest-tile
    # instance. Bind its digest only after every identity and state field has
    # reached final bytes; reusing the owner-envelope beginning digest here
    # left a schema-valid but cryptographically stale positive fixture.
    actual_lse_state["state_sha256"] = digest(
        {**actual_lse_state, "state_sha256": ""})
    coupled["beginning_lse_state"] = actual_lse_state
    hydrology_schema_stores = []
    for row in beginning_stores:
        ofe, source_type, source_id, source_tile, soil_layer = row["resource_identity"]
        amount = row["amount_kg_m2_stand_ground"]
        tile_source = source_type != "soil_layer_liquid"
        hydrology_schema_stores.append({"ofe_id": ofe,
            "tile_id": source_tile if tile_source else None,
            "source_type": source_type, "source_id": source_id,
            "soil_layer_id": soil_layer if not tile_source else None,
            "amount_basis": ("kg_h2o_m-2_tile_ground" if tile_source
                else "kg_h2o_m-2_stand_ground"),
            "liquid_amount_kg_m2": amount / 0.62 if tile_source else amount,
            "ice_amount_kg_m2": 0.0, "frozen": False, "thawing": False})
    coupled["beginning_hydrology_snapshot"].update({
        "configuration_sha256": beginning["hydrology"]["configuration_sha256"],
        "state_sha256": beginning["hydrology"]["state_sha256"],
        "snapshot_sha256": protocol["beginning_snapshot_sha256"],
        "last_accepted_transaction_id": owner_transaction["transaction_id"] - 1,
        "stores": hydrology_schema_stores})
    coupled["beginning_soil_thermal_snapshot"].update({
        "configuration_sha256": beginning["soil_thermal"]["configuration_sha256"],
        "state_sha256": beginning["soil_thermal"]["state_sha256"],
        "snapshot_sha256": digest(beginning["soil_thermal"]),
        "last_accepted_transaction_id": owner_transaction["transaction_id"] - 1,
        "ofes": [{"ofe_id": "ofe-1", "ordered_layers": [
            {"layer_id": f"thermal-{index + 1}", "temperature_k": temperature,
             "enthalpy_j_m2_ofe_ground": temperature * capacity}
            for index, (temperature, capacity) in enumerate(zip(
                beginning["soil_thermal"]["state"]["temperatures_k"],
                (120000.0, 180000.0), strict=True))]}]})
    coupled["beginning_biogeochemistry_owner"].update({
        "model_definition_sha256": beginning["biogeochemistry"]["model_definition_sha256"],
        "configuration_sha256": beginning["biogeochemistry"]["configuration_sha256"],
        "state_sha256": beginning["biogeochemistry"]["state_sha256"],
        "last_accepted_transaction_id": owner_transaction["transaction_id"] - 1})
    schema_instances["coupled_transaction"] = coupled
    schema_instances["state"] = actual_lse_state
    schema_evidence = validate_schema_instances(schema_instances)
    failures = executed_validation_failures(forcing)
    poisons = executed_protocol_poisons(protocol)
    poisons.update(executed_component_poisons(mandatory, ingress, owner_transaction))
    poisons.update(executed_diagnostic_pairing_poisons(
        schema_instances["diagnostics"]))
    numerical_failures = wrap_natural_failures(
        expanded["real_numerical_failures"], owner_transaction)
    failures.update(numerical_failures)
    failures = bind_failure_diagnostics(failures, owner_transaction,
        beginning["land_surface_energy"]["configuration_sha256"],
        beginning["land_surface_energy"]["state_sha256"])
    reconstructed = {
        "final_rebuilt_from_beginning": transaction["final_rebuilt_from_beginning"],
        "single_immutable_authorization": transaction["single_immutable_authorization"],
        "post_ingress_mass_closure": ingress["mass_residual_kg_m2_tile_ground"],
        "post_ingress_energy_closure": ingress["energy_residual_j_m2_tile_ground"],
        "all_schema_instances_validated": all(value["validated"]
            for value in schema_evidence.values()),
        "all_validation_failures_rollback_exact": all(value["rollback_exact"]
            and value["candidate"] is None for value in failures.values()),
        "all_poisons_rejected": all(not value["accepted"] and value["candidate"] is None
            for value in poisons.values())}
    if not (reconstructed["final_rebuilt_from_beginning"]["proved"]
            and reconstructed["single_immutable_authorization"]["proved"]
            and abs(reconstructed["post_ingress_mass_closure"]) <= MASS_TOL
            and abs(reconstructed["post_ingress_energy_closure"]) <= 1.0e-9
            and reconstructed["all_schema_instances_validated"]
            and reconstructed["all_validation_failures_rollback_exact"]
            and reconstructed["all_poisons_rejected"]
            and shared_layer_competition["arbitration_call_count"] == 1
            and shared_layer_competition["both_rebuilt_from_beginning"]
            and all(abs(row["mass_residual_kg_m2_stand_ground"]) <= MASS_TOL
                and row["unused_authorization_kg_m2_stand_ground"] >= -MASS_TOL
                for row in shared_layer_competition["source_ending_store_ledger"])
            and validate_multi_ofe_route(upstream_outlet, routed_runon) == route_join
            and condensation_owner["candidates"]["hydrology"]["ending_state"]
                ["resource_store_ledger"][0]["condensation_credit_kg_m2_stand_ground"] > 0.0):
        raise AssertionError(f"reconstructed authority invariant failure:{reconstructed}")
    return {"schema": "openwepp-snow-free-lse-v1-joint-authority-vectors-3",
        "model": "OPENWEPP_SNOW_FREE_LSE_V1",
        "model_definition_sha256": LSE_MODEL_DEFINITION_SHA256,
        "source_checksums": {"joint_core_sha256": LSE_V8_JOINT_CORE_SHA256,
            "inherited_v3_oracle_sha256": INHERITED_V3_ORACLE_SHA256,
            "schemas": SCHEMA_SHA256},
        "inherited_v8_canopy_authority_execution": inherited_v3_oracle(),
        "exact_model_reductions": expanded,
        "mandatory_exact_scenario_vectors": mandatory,
        "complete_water_transaction": transaction,
        "shared_layer_root_ground_competition": shared_layer_competition,
        "positive_condensation_owner_transaction": condensation_owner,
        "equilibrium_zero_storage_branch": equilibrium,
        "post_ingress_owner_candidates": {"ingress": ingress,
            "beginning": beginning, "candidates": candidates, "joins": joins,
            "owner_validation": owner_validation, "owner_receipts": receipts},
        "multi_ofe_routed_owner_vector": multi_ofe,
        "strict_schema_instances": schema_instances,
        "strict_schema_validation": schema_evidence,
        "executed_failure_vectors": failures,
        "executed_poison_vectors": poisons,
        "reconstructed_invariants": reconstructed}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", type=Path)
    args = parser.parse_args()
    payload = canonical(build_authority_vectors()) + "\n"
    if args.write:
        args.write.write_text(payload, encoding="utf-8")
        print(hashlib.sha256(payload.encode()).hexdigest())
    else:
        print(payload, end="")


if __name__ == "__main__":
    main()
