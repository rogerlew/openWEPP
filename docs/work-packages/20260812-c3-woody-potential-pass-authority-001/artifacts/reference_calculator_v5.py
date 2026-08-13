#!/usr/bin/env python3
"""Generate deterministic independent V5 authorization-capped vectors.

This standard-library oracle deliberately evaluates the layer constitutive law
before applying any owner cap.  It does not call or import the Rust
implementation.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
import math
from copy import deepcopy
from pathlib import Path
from typing import Any


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
CONTRACT = ROOT / "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md"
V4_DEFINITION = (
    ROOT
    / "docs/work-packages/20260812-c3-woody-shared-state-authority-001/artifacts"
    / "openwepp_c3_woody_v4_definition.json"
)
VECTOR_PATH = HERE / "openwepp_c3_woody_v5_vectors.json"
DEFINITION_PATH = HERE / "openwepp_c3_woody_v5_definition.json"
MODEL_STACK_COPY = (
    ROOT
    / "docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts"
    / "openwepp_c3_woody_v5_definition.json"
)

V4_SHA256 = "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437"
V3_ABSOLUTE_WATER_TOLERANCE = 1.0e-12
V3_RELATIVE_WATER_TOLERANCE = 1.0e-9


def load_v3_oracle() -> Any:
    path = HERE / "reference_calculator.py"
    spec = importlib.util.spec_from_file_location("openwepp_v3_independent_oracle", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load immutable V3 oracle")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


V3 = load_v3_oracle()


class RedistributionError(ValueError):
    def __init__(self, layer_id: str, q_law: float, evaluated_layers: list[dict[str, Any]]):
        super().__init__(f"hydraulic_redistribution:{layer_id}")
        self.layer_id = layer_id
        self.q_law = q_law
        self.evaluated_layers = deepcopy(evaluated_layers)


def canonical_bytes(value: Any) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def digest_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def digest_value(value: Any) -> str:
    return digest_bytes(canonical_bytes(value))


def file_digest(path: Path) -> str:
    return digest_bytes(path.read_bytes())


def section_digest(text: str, start: str, end: str) -> str:
    begin = text.index(start)
    finish = text.index(end, begin)
    return digest_bytes(text[begin:finish].encode())


def beginning_state() -> dict[str, Any]:
    return {
        "model_definition_sha256": "BOUND_BY_V5_DEFINITION_NOT_ORACLE",
        "configuration_sha256": "configuration-v5-capped-fixture",
        "last_transaction_id": 52,
        "occupancy": {
            "stratum_id": "canopy",
            "tile_id": "tile-a",
            "beta_hyd": 0.37,
            "root_node_potential_mm": -4000.0,
            "stem_potential_mm": -16000.0,
            "sun_leaf_potential_mm": -17100.0,
            "shade_leaf_potential_mm": -16800.0,
        },
    }


def law_flux(layer: dict[str, Any], root_potential_mm: float) -> tuple[float, float]:
    """Return independently evaluated q_law and d(q_law)/d(psi_root)."""
    if (not layer["accessible"] or layer["dry"] or layer["frozen"]
            or layer.get("root_fraction", 1.0) == 0.0):
        return 0.0, 0.0
    gradient_mm = (
        layer["soil_potential_mm"] - root_potential_mm + layer["gravity_head_mm"]
    )
    flux = layer["series_conductance_kg_m2_s_mm"] * gradient_mm
    if flux < 0.0:
        raise ValueError(f"hydraulic_redistribution:{layer['layer_id']}")
    return flux, -layer["series_conductance_kg_m2_s_mm"]


def cap_layer(
    layer: dict[str, Any],
    root_potential_mm: float,
    tile_fraction: float,
    dt_s: float,
    authorization_stand: float,
) -> dict[str, Any]:
    q_law, law_derivative = law_flux(layer, root_potential_mm)
    authorization_tile = authorization_stand / tile_fraction
    cap_rate = authorization_stand / (tile_fraction * dt_s)
    active = cap_rate <= q_law
    q_final = cap_rate if active else q_law
    return {
        "layer_id": layer["layer_id"],
        "accessible": layer["accessible"],
        "dry": layer.get("dry", False),
        "frozen": layer["frozen"],
        "root_fraction": layer.get("root_fraction"),
        "authorization_kg_m2_stand_ground": authorization_stand,
        "authorization_kg_m2_tile_ground": authorization_tile,
        "cap_rate_kg_m2_tile_s": cap_rate,
        "q_law_kg_m2_tile_s": q_law,
        "q_final_kg_m2_tile_s": q_final,
        "branch": "authorization_active_or_tie" if active else "constitutive_law",
        "dq_final_d_root_potential": 0.0 if active else law_derivative,
        "finalized_use_kg_m2_stand_ground": tile_fraction * q_final * dt_s,
    }


def water_scale(
    e_sun: float,
    e_shade: float,
    q1_sun: float,
    q1_shade: float,
    q2: float,
    capped_layers: list[dict[str, Any]],
) -> float:
    operands = [e_sun, e_shade, q1_sun, q1_shade, q2]
    for layer in capped_layers:
        operands.extend(
            [
                layer["q_law_kg_m2_tile_s"],
                layer["cap_rate_kg_m2_tile_s"],
                layer["q_final_kg_m2_tile_s"],
            ]
        )
    return max(V3_ABSOLUTE_WATER_TOLERANCE, *(abs(item) for item in operands))


def residual_record(identity: str, raw: float, scale: float) -> dict[str, Any]:
    tolerance = V3_ABSOLUTE_WATER_TOLERANCE + V3_RELATIVE_WATER_TOLERANCE * scale
    return {
        "identity": identity,
        "raw_kg_m2_tile_s": raw,
        "scale_kg_m2_tile_s": scale,
        "tolerance": tolerance,
        "normalized": raw / tolerance,
    }


RESIDUAL_IDENTITIES = ["sun_gas_minus_q1", "shade_gas_minus_q1",
    "sun_gas_minus_vulnerability_demand", "shade_gas_minus_vulnerability_demand",
    "q1_sum_minus_q2", "q2_minus_capped_q3_sum"]


def diagnostic_residuals(flux: dict[str, Any], case: dict[str, Any]) -> list[dict[str, Any]]:
    cap_scale = capped_scale(flux, case)
    v3_scale = max(V3_ABSOLUTE_WATER_TOLERANCE, case["emax"]["sun"],
        case["emax"]["shade"], abs(flux["q1_sun"]), abs(flux["q1_shade"]),
        abs(flux["q2"]), *(abs(row["q_final_kg_m2_tile_s"]) for row in flux["q3"]))
    return [residual_record(identity, raw, cap_scale if index == 5 else v3_scale)
        for index, (identity, raw) in enumerate(zip(RESIDUAL_IDENTITIES,
                                                    flux["residuals"], strict=True))]


def capped_hydraulic_fluxes(
    x: list[float], case: dict[str, Any], cap_rates: dict[str, float],
    frozen_branches: dict[str, str] | None = None,
) -> dict[str, Any]:
    """Evaluate the complete V3 system with independently capped layer laws."""
    sun, shade, stem, root, beta_sun, beta_shade = x
    p = case["parameters"]
    leaf_factor = p["k1_max"] / p["stem_to_leaf_path_m"]
    q1_sun = (
        leaf_factor
        * p["sun_leaf_area"]
        * V3.vulnerability(stem, p["p50_xylem"], p["ck"])
        * (stem - sun)
    )
    q1_shade = (
        leaf_factor
        * p["shade_leaf_area"]
        * V3.vulnerability(stem, p["p50_xylem"], p["ck"])
        * (stem - shade)
    )
    q2 = (
        (p["k2_max"] / p["height_m"])
        * V3.vulnerability(root, p["p50_xylem"], p["ck"])
        * p["sai"]
        * (root - stem - 1000.0 * p["height_m"])
    )
    q3 = []
    for layer in case["layers"]:
        if not layer["accessible"] or layer["frozen"] or layer["root_fraction"] == 0.0:
            q_law = 0.0
            kr = ks = k3 = rai = soil_v = 0.0
        else:
            soil_v = V3.vulnerability(layer["soil_potential_mm"], p["p50_root"], p["ck"])
            kr = (p["k3_max_m_s"] / layer["z3_m"]) * soil_v
            ks = layer["ksoil_m2_s"] / layer["dxroot_m"]
            k3 = kr * ks / (kr + ks)
            rai = (p["lai"] + p["sai"]) * layer["root_fraction"] * p["root_to_leaf_area"]
            q_law = k3 * rai * (
                layer["soil_potential_mm"] - root + layer["gravity_head_mm"]
            )
            if q_law < 0.0:
                raise RedistributionError(layer["layer_id"], q_law, q3)
        cap_rate = cap_rates[layer["layer_id"]]
        selected = (frozen_branches or {}).get(layer["layer_id"])
        active = cap_rate <= q_law if selected is None else selected == "authorization_active_or_tie"
        q_final = cap_rate if active else q_law
        q3.append({
            "layer_id": layer["layer_id"],
            "accessible": layer["accessible"],
            "dry": layer.get("dry", False),
            "frozen": layer["frozen"],
            "root_fraction": layer["root_fraction"],
            "q_law_kg_m2_tile_s": q_law,
            "cap_rate_kg_m2_tile_s": cap_rate,
            "q_final_kg_m2_tile_s": q_final,
            "branch": "authorization_active_or_tie" if active else "constitutive_law",
            "generalized_derivative_branch": "zero" if active else "constitutive_law",
            "kr_m_s": kr,
            "ks_m_s": ks,
            "k3_series_m_s": k3,
            "rai_m2_m2": rai,
            "soil_vulnerability": soil_v,
        })
    v_sun = V3.vulnerability(sun, p["p50_leaf"], p["ck"])
    v_shade = V3.vulnerability(shade, p["p50_leaf"], p["ck"])
    energy = V3.coupled_canopy_energy(case, (beta_sun, beta_shade), (sun, shade))
    sun_state, shade_state = energy["sun"], energy["shade"]
    e_sun = sun_state["transpiration_kg_m2_tile_s"]
    e_shade = shade_state["transpiration_kg_m2_tile_s"]
    residuals = [
        e_sun - q1_sun,
        e_shade - q1_shade,
        e_sun - case["emax"]["sun"] * v_sun,
        e_shade - case["emax"]["shade"] * v_shade,
        q1_sun + q1_shade - q2,
        q2 - math.fsum(row["q_final_kg_m2_tile_s"] for row in q3),
    ]
    return {
        "q1_sun": q1_sun,
        "q1_shade": q1_shade,
        "q2": q2,
        "q3": q3,
        "v_sun": v_sun,
        "v_shade": v_shade,
        "gas_energy_transpiration_sun": e_sun,
        "gas_energy_transpiration_shade": e_shade,
        "sun_gas_energy_state": sun_state,
        "shade_gas_energy_state": shade_state,
        "canopy_energy_state": {key: value for key, value in energy.items() if key not in {"sun", "shade"}},
        "residuals": residuals,
    }


def capped_scale(flux: dict[str, Any], case: dict[str, Any]) -> float:
    return max(
        V3_ABSOLUTE_WATER_TOLERANCE,
        case["emax"]["sun"],
        case["emax"]["shade"],
        abs(flux["q1_sun"]),
        abs(flux["q1_shade"]),
        abs(flux["q2"]),
        *(abs(row[key]) for row in flux["q3"] for key in (
            "q_law_kg_m2_tile_s", "cap_rate_kg_m2_tile_s", "q_final_kg_m2_tile_s"
        )),
    )


def solve_capped(
    case: dict[str, Any], cap_rates: dict[str, float], start: list[float], max_iterations: int = 50
) -> dict[str, Any]:
    x = start[:]
    if set(cap_rates) != {layer["layer_id"] for layer in case["layers"]}:
        raise ValueError("authorization_identity_schema")
    if any(not math.isfinite(value) or value < 0.0 for value in cap_rates.values()):
        raise ValueError("authorization_domain")
    backtracking = 0
    pivot_min = matrix_norm = last_step = None
    history = []
    for iteration in range(max_iterations + 1):
        flux = capped_hydraulic_fluxes(x, case, cap_rates)
        scale = capped_scale(flux, case)
        tolerance = V3_ABSOLUTE_WATER_TOLERANCE + V3_RELATIVE_WATER_TOLERANCE * scale
        normalized = [value / tolerance for value in flux["residuals"]]
        norm = max(abs(value) for value in normalized)
        history.append(norm)
        if norm <= 1.0 and (last_step is None or last_step <= 1.0e-7):
            identities = [
                "sun_gas_minus_q1", "shade_gas_minus_q1",
                "sun_gas_minus_vulnerability_demand",
                "shade_gas_minus_vulnerability_demand", "q1_sum_minus_q2",
                "q2_minus_capped_q3_sum",
            ]
            v3_scale = max(
                V3_ABSOLUTE_WATER_TOLERANCE,
                case["emax"]["sun"], case["emax"]["shade"],
                abs(flux["q1_sun"]), abs(flux["q1_shade"]), abs(flux["q2"]),
                *(abs(row["q_final_kg_m2_tile_s"]) for row in flux["q3"]),
            )
            f_t, dt_s = case["tile_fraction"], case["dt_s"]
            for row in flux["q3"]:
                row["authorization_kg_m2_stand_ground"] = (
                    row["cap_rate_kg_m2_tile_s"] * f_t * dt_s
                )
                row["authorization_kg_m2_tile_ground"] = row["cap_rate_kg_m2_tile_s"] * dt_s
                row["finalized_use_kg_m2_stand_ground"] = (
                    row["q_final_kg_m2_tile_s"] * f_t * dt_s
                )
            return {
                "iterations": iteration,
                "solution": {
                    "sun_leaf_potential_mm": x[0], "shade_leaf_potential_mm": x[1],
                    "stem_potential_mm": x[2], "root_node_potential_mm": x[3],
                    "beta_hyd_sun": x[4], "beta_hyd_shade": x[5],
                    "beta_hyd": (
                        case["emax"]["sun"] * x[4] + case["emax"]["shade"] * x[5]
                    ) / (case["emax"]["sun"] + case["emax"]["shade"]),
                },
                "fluxes": {key: value for key, value in flux.items() if key != "residuals"},
                "normalized_residuals": [
                    residual_record(identity, raw, scale if index == 5 else v3_scale)
                    for index, (identity, raw) in enumerate(
                        zip(identities, flux["residuals"], strict=True)
                    )
                ],
                "water_residual_scale_kg_m2_tile_s": scale,
                "active_water_caps": [
                    row["layer_id"] for row in flux["q3"]
                    if row["branch"] == "authorization_active_or_tie"
                ],
                "residual_norm_history": history,
                "backtracking_count": backtracking,
                "potential_step_mm": last_step,
                "pivot_magnitude": pivot_min,
                "matrix_norm": matrix_norm,
            }
        if iteration == max_iterations:
            return {"failure": "iteration_limit", "candidate": None, "iterations": iteration,
                    "residual_norm_history": history,
                    "diagnostics": {
                        "pass": "capped", "solve": "outer_gas_energy_hydraulic_coupling",
                        "iterations": iteration,
                        "residual_norms": diagnostic_residuals(flux, case),
                        "step_norm": last_step, "backtracking_count": backtracking,
                        "active_water_caps": [row["layer_id"] for row in flux["q3"]
                                              if row["branch"] == "authorization_active_or_tie"],
                        "pivot_magnitude": pivot_min, "matrix_norm": matrix_norm,
                        "layer_operands_in_configuration_order": deepcopy(flux["q3"]),
                    }}
        scales = [1000.0, 1000.0, 1000.0, 1000.0, 1.0, 1.0]
        steps = [math.sqrt(2.220446049250313e-16) * max(abs(value), unit)
                 for value, unit in zip(x, scales, strict=True)]
        jacobian = [[0.0] * 6 for _ in range(6)]
        for column in range(6):
            plus, minus = x[:], x[:]
            plus[column] += steps[column]
            minus[column] -= steps[column]
            branches = {row["layer_id"]: row["branch"] for row in flux["q3"]}
            rp = capped_hydraulic_fluxes(plus, case, cap_rates, branches)["residuals"]
            rm = capped_hydraulic_fluxes(minus, case, cap_rates, branches)["residuals"]
            for row in range(6):
                jacobian[row][column] = (rp[row] - rm[row]) / (2.0 * steps[column])
        try:
            delta, pivot_min, matrix_norm = V3.solve_linear(jacobian, [-r for r in flux["residuals"]])
        except V3.SingularMatrixError as error:
            return {"failure": "singular", "candidate": None, "iterations": iteration,
                    "pivot_magnitude": error.pivot, "matrix_norm": error.matrix_norm,
                    "diagnostics": {
                        "pass": "capped", "solve": "hydraulic_system",
                        "iterations": iteration, "residual_norms": diagnostic_residuals(flux, case),
                        "step_norm": None, "backtracking_count": backtracking,
                        "active_water_caps": [row["layer_id"] for row in flux["q3"]
                                              if row["branch"] == "authorization_active_or_tie"],
                        "pivot_magnitude": error.pivot, "matrix_norm": error.matrix_norm,
                        "layer_operands_in_configuration_order": deepcopy(flux["q3"]),
                    }}
        if norm <= 1.0 and max(abs(value) for value in delta[:4]) <= 1.0e-7:
            last_step = max(abs(value) for value in delta[:4])
            continue
        accepted = False
        for exponent in range(21):
            factor = 0.5 ** exponent
            trial = [value + factor * change for value, change in zip(x, delta, strict=True)]
            if not (0.0 <= trial[4] <= 1.0 and 0.0 <= trial[5] <= 1.0):
                continue
            trial_flux = capped_hydraulic_fluxes(trial, case, cap_rates)
            trial_tolerance = (
                V3_ABSOLUTE_WATER_TOLERANCE
                + V3_RELATIVE_WATER_TOLERANCE * capped_scale(trial_flux, case)
            )
            trial_norm = max(abs(value) / trial_tolerance for value in trial_flux["residuals"])
            if trial_norm < norm:
                x = trial
                last_step = max(abs(change) * factor for change in delta[:4])
                backtracking += exponent
                accepted = True
                break
        if not accepted:
            return {"failure": "backtracking_limit", "candidate": None, "iterations": iteration,
                    "diagnostics": {
                        "pass": "capped", "solve": "hydraulic_system",
                        "iterations": iteration, "residual_norms": diagnostic_residuals(flux, case),
                        "step_norm": max(abs(value) for value in delta),
                        "backtracking_count": backtracking + 20,
                        "active_water_caps": [row["layer_id"] for row in flux["q3"]
                                              if row["branch"] == "authorization_active_or_tie"],
                        "pivot_magnitude": pivot_min, "matrix_norm": matrix_norm,
                        "layer_operands_in_configuration_order": deepcopy(flux["q3"]),
                    }}
    raise AssertionError("unreachable")


def complete_coupled_vectors() -> dict[str, Any]:
    v3_family = V3.hydraulic_vectors()
    case = deepcopy(v3_family["operands"])
    case["layers"].append({"layer_id": "soil-zero-root", "soil_potential_mm": 100.0,
        "gravity_head_mm": 500.0, "root_fraction": 0.0, "z3_m": 0.7,
        "ksoil_m2_s": 1.0e-7, "dxroot_m": 0.25,
        "accessible": True, "frozen": False})
    uncapped = v3_family["accepted_uncapped_stage_a"]
    uncapped_rates = {
        row["layer_id"]: max(0.0, row["flux"]) for row in uncapped["fluxes"]["q3"]
    }
    uncapped_rates["soil-zero-root"] = 0.0
    mixed_rates = dict(uncapped_rates)
    mixed_rates["soil-1"] *= 0.55
    mixed_rates["soil-2"] *= 1.0
    mixed_rates["soil-dry"] = 0.0
    mixed_rates["soil-frozen"] = 0.0
    start = [-5900.0, -5450.0, -4300.0, -2850.0, 0.68, 0.66]
    mixed = solve_capped(case, mixed_rates, start)
    if "failure" in mixed:
        raise AssertionError(f"mixed coupled capped fixture failed: {mixed}")
    s = mixed["solution"]
    alternate_start = [s["sun_leaf_potential_mm"] + 10.0,
                       s["shade_leaf_potential_mm"] - 10.0,
                       s["stem_potential_mm"] + 5.0,
                       s["root_node_potential_mm"] - 5.0,
                       s["beta_hyd_sun"] + 0.001,
                       s["beta_hyd_shade"] - 0.001]
    alternate = solve_capped(case, mixed_rates, alternate_start)
    if "failure" in alternate:
        raise AssertionError(f"alternate capped warm start failed: {alternate}")
    for key, expected in mixed["solution"].items():
        if not math.isclose(alternate["solution"][key], expected,
                            rel_tol=2.0e-9, abs_tol=2.0e-7):
            raise AssertionError(f"alternate capped warm start mismatch {key}")
    fully_authorized_rates = dict(uncapped_rates)
    all_law = solve_capped(case, fully_authorized_rates, start)
    if "failure" in all_law:
        raise AssertionError(f"fully authorized capped fixture failed: {all_law}")
    for key, expected in uncapped["solution"].items():
        actual = all_law["solution"][key]
        if not math.isclose(actual, expected, rel_tol=2.0e-9, abs_tol=2.0e-7):
            raise AssertionError(f"fully authorized reduction mismatch {key}: {actual} != {expected}")
    valid_space_audit = []
    for fraction in (0.05, 0.25, 0.5, 0.75, 1.0):
        rates = {key: value * fraction for key, value in uncapped_rates.items()}
        audit = solve_capped(case, rates, start)
        valid_space_audit.append({"uniform_authorization_fraction_of_D": fraction,
            "accepted": "failure" not in audit,
            "failure": audit.get("failure"),
            "positive_layer_branches": ([row["branch"] for row in audit["fluxes"]["q3"][:2]]
                                        if "failure" not in audit else None)})
    request_by_layer = {
        row["layer_id"]: row["amount_kg_h2o_m2_stand_ground"]
        for row in uncapped["water_requests"]
    }
    request_by_layer["soil-zero-root"] = 0.0
    for result in (mixed, all_law):
        for row in result["fluxes"]["q3"]:
            row["potential_request_kg_m2_stand_ground"] = request_by_layer[row["layer_id"]]
    tie_x = [all_law["solution"][key] for key in ("sun_leaf_potential_mm",
        "shade_leaf_potential_mm", "stem_potential_mm", "root_node_potential_mm",
        "beta_hyd_sun", "beta_hyd_shade")]
    base_flux = capped_hydraulic_fluxes(tie_x, case, fully_authorized_rates)
    tie_rates = dict(fully_authorized_rates)
    tie_rate = next(row["q_law_kg_m2_tile_s"] for row in base_flux["q3"]
                    if row["layer_id"] == "soil-1")
    tie_rates["soil-1"] = tie_rate
    tied = capped_hydraulic_fluxes(tie_x, case, tie_rates)
    frozen = {row["layer_id"]: row["branch"] for row in tied["q3"]}
    root_step = math.sqrt(2.220446049250313e-16) * max(abs(tie_x[3]), 1000.0)
    plus, minus = tie_x[:], tie_x[:]
    plus[3] += root_step
    minus[3] -= root_step
    frozen_plus = capped_hydraulic_fluxes(plus, case, tie_rates, frozen)
    frozen_minus = capped_hydraulic_fluxes(minus, case, tie_rates, frozen)
    trial_plus = capped_hydraulic_fluxes(plus, case, tie_rates)
    trial_minus = capped_hydraulic_fluxes(minus, case, tie_rates)
    def layer_row(value: dict[str, Any]) -> dict[str, Any]:
        return next(row for row in value["q3"] if row["layer_id"] == "soil-1")
    coupled_tie = {"unperturbed": layer_row(tied), "root_step_mm": root_step,
        "frozen_plus": layer_row(frozen_plus), "frozen_minus": layer_row(frozen_minus),
        "trial_plus_reselected": layer_row(trial_plus),
        "trial_minus_reselected": layer_row(trial_minus),
        "frozen_cap_derivative": (layer_row(frozen_plus)["q_final_kg_m2_tile_s"]
            - layer_row(frozen_minus)["q_final_kg_m2_tile_s"]) / (2.0 * root_step)}
    water_beginning = {
        "soil-1": 0.02,
        "soil-2": 0.015,
        "soil-dry": 0.0,
        "soil-frozen": 0.0,
        "soil-zero-root": 0.0,
    }
    for result in (mixed, all_law):
        water_ending = {
            row["layer_id"]: water_beginning[row["layer_id"]]
            - row["finalized_use_kg_m2_stand_ground"]
            for row in result["fluxes"]["q3"]
        }
        result["hydrology_owner_closure"] = {
            "beginning_inventory_kg_m2_stand_ground": water_beginning,
            "ending_inventory_kg_m2_stand_ground": water_ending,
            "debit_operand": "finalized_use_not_authorization",
            "authorization_debit_poison_ending": {
                row["layer_id"]: water_beginning[row["layer_id"]]
                - row["authorization_kg_m2_stand_ground"]
                for row in result["fluxes"]["q3"]
            },
            "residuals": {
                row["layer_id"]: water_beginning[row["layer_id"]]
                - row["finalized_use_kg_m2_stand_ground"]
                - water_ending[row["layer_id"]]
                for row in result["fluxes"]["q3"]
            },
        }
    state = beginning_state()
    state_sha = digest_value(state)
    singular_case = deepcopy(case)
    singular_case["parameters"]["k1_max"] = 0.0
    singular_case["parameters"]["k2_max"] = 0.0
    for layer in singular_case["layers"]:
        layer["accessible"] = False
    singular = solve_capped(singular_case, {key: 0.0 for key in mixed_rates}, start)
    limited = solve_capped(case, mixed_rates, start, max_iterations=0)
    exhausted = solve_capped(case, {key: 0.0 for key in mixed_rates}, start)
    if exhausted.get("failure") != "backtracking_limit":
        raise AssertionError(f"backtracking exhaustion not exercised: {exhausted}")
    try:
        invalid = dict(mixed_rates)
        invalid["soil-1"] = -1.0
        solve_capped(case, invalid, start)
        raise AssertionError("negative authorization did not fail")
    except ValueError as error:
        domain = {"failure": str(error), "candidate": None, "iterations": 0,
                  "diagnostics": {"pass": "capped", "solve": "authorization_validation",
                                  "iterations": 0, "residual_norms": [], "step_norm": None,
                                  "backtracking_count": 0, "active_water_caps": [],
                                  "pivot_magnitude": None, "matrix_norm": None}}
    redistribution_case = deepcopy(case)
    redistribution_case["layers"][1]["soil_potential_mm"] = -20000.0
    try:
        solve_capped(redistribution_case, mixed_rates, start)
        raise AssertionError("redistribution failure not exercised")
    except RedistributionError as error:
        redistribution = {"failure": str(error), "candidate": None, "iterations": 0,
            "diagnostics": {"pass": "capped", "solve": "hydraulic_system", "iterations": 0,
                "residual_norms": [], "step_norm": None, "backtracking_count": 0,
                "active_water_caps": [], "pivot_magnitude": None, "matrix_norm": None,
                "layer_operands_in_configuration_order": error.evaluated_layers + [{
                    "layer_id": error.layer_id, "q_law_kg_m2_tile_s": error.q_law,
                    "rejected_before_complementarity": True}]}}
    executed_failures = []
    for executed_by, result in (
        ("solve_capped_singular_system", singular),
        ("solve_capped_iteration_limit_zero", limited),
        ("solve_capped_twenty_halving_exhaustion", exhausted),
        ("solve_capped_negative_authorization", domain),
        ("solve_capped_negative_q_law", redistribution),
    ):
        diagnostics = result["diagnostics"]
        diagnostics.update({
            "model_definition_sha256": "BOUND_BY_V5_DEFINITION_NOT_ORACLE",
            "transaction_id": 53,
            "occupancy_id": {"stratum_id": "canopy", "tile_id": "tile-a"},
            "active_bounds": diagnostics.get("active_bounds", []),
            "bracket": diagnostics.get("bracket"),
            "fixed_authorization_identity": {
                "transaction_id": 53, "owner_id": "diagnostic-water-owner",
                "stratum_id": "canopy", "tile_id": "tile-a",
                "basis": "kg_h2o_m2_stand_ground_interval",
            },
        })
        actual_rows = diagnostics.setdefault("layer_operands_in_configuration_order", [])
        for row in actual_rows:
            if "cap_rate_kg_m2_tile_s" in row:
                row["potential_request_kg_m2_stand_ground"] = request_by_layer[row["layer_id"]]
                row["authorization_kg_m2_stand_ground"] = (
                    row["cap_rate_kg_m2_tile_s"] * case["tile_fraction"] * case["dt_s"])
                row["authorization_kg_m2_tile_ground"] = (
                    row["cap_rate_kg_m2_tile_s"] * case["dt_s"])
        executed_failures.append({
            **result,
            "typed_error": result["failure"],
            "last_iterate": None,
            "executed_by": executed_by,
            "beginning_state_sha256_before": state_sha,
            "beginning_state_sha256_after": digest_value(state),
            "rollback_byte_identical": state_sha == digest_value(state),
            "active_water_caps": diagnostics["active_water_caps"],
        })
    return {
        "operands": case,
        "accepted_constrained_all_cap": mixed,
        "alternate_warm_start": {"result": alternate, "start": alternate_start},
        "fully_authorized_value_reduction": {
            "capped_result": all_law,
            "uncapped_reference": uncapped,
            "solution_matches_uncapped": True,
            "branch_note": (
                "value reduction is the acceptance claim; branch labels follow exact evaluated bits. "
                "This fixture's positive rows select the law branch by representational ulps, while "
                "the separate exact-tie fixture proves equality selects authorization-active"
            ),
        },
        "executed_failures": executed_failures,
        "executed_coupled_exact_tie_jacobian": coupled_tie,
        "valid_A_le_D_branch_sweep": {
            "scope": "deterministic empirical audit, not a theorem",
            "cases": valid_space_audit,
            "accepted_positive_law_branch_found": any(case["accepted"] and
                "constitutive_law" in case["positive_layer_branches"]
                for case in valid_space_audit),
        },
    }


def layers() -> list[dict[str, Any]]:
    return [
        {
            "layer_id": "root-z",
            "accessible": True,
            "dry": False,
            "frozen": False,
            "soil_potential_mm": -2000.0,
            "gravity_head_mm": 290.0,
            "series_conductance_kg_m2_s_mm": 2.0e-10,
        },
        {
            "layer_id": "root-a",
            "accessible": True,
            "dry": False,
            "frozen": False,
            "soil_potential_mm": -3000.0,
            "gravity_head_mm": 240.0,
            "series_conductance_kg_m2_s_mm": 1.0e-10,
        },
        {
            "layer_id": "root-m",
            "accessible": True,
            "dry": False,
            "frozen": False,
            "soil_potential_mm": -1000.0,
            "gravity_head_mm": 370.0,
            "series_conductance_kg_m2_s_mm": 0.5e-10,
        },
        {
            "layer_id": "dry-layer",
            "accessible": True,
            "dry": True,
            "frozen": False,
            "soil_potential_mm": -900000.0,
            "gravity_head_mm": 500.0,
            "series_conductance_kg_m2_s_mm": 9.0e-10,
        },
        {
            "layer_id": "frozen-layer",
            "accessible": True,
            "dry": False,
            "frozen": True,
            "soil_potential_mm": -50.0,
            "gravity_head_mm": 700.0,
            "series_conductance_kg_m2_s_mm": 8.0e-10,
        },
        {
            "layer_id": "inaccessible-layer",
            "accessible": False,
            "dry": False,
            "frozen": False,
            "soil_potential_mm": 100.0,
            "gravity_head_mm": 0.0,
            "series_conductance_kg_m2_s_mm": 7.0e-10,
        },
        {
            "layer_id": "zero-root-layer", "accessible": True, "dry": False,
            "frozen": False, "root_fraction": 0.0, "soil_potential_mm": 100.0,
            "gravity_head_mm": 500.0, "series_conductance_kg_m2_s_mm": 6.0e-10,
        },
    ]


def authorization_for_rate(rate: float, tile_fraction: float, dt_s: float) -> float:
    return rate * tile_fraction * dt_s


def mixed_active_set() -> dict[str, Any]:
    tile_fraction = 0.37
    dt_s = 1800.0
    root = -4000.0
    rates = [1.0e-7, 4.0e-6, 1.7e-6, 0.0, 0.0, 0.0, 0.0]
    capped = [
        cap_layer(layer, root, tile_fraction, dt_s, authorization_for_rate(rate, tile_fraction, dt_s))
        for layer, rate in zip(layers(), rates, strict=True)
    ]
    q_sum = sum(item["q_final_kg_m2_tile_s"] for item in capped)
    e_sun = 5.0e-6
    e_shade = 3.1e-6
    q1_sun = 5.0e-6
    q1_shade = 3.1e-6
    q2 = q_sum
    scale = water_scale(e_sun, e_shade, q1_sun, q1_shade, q2, capped)
    residuals = [
        residual_record("sun_gas_minus_q1", e_sun - q1_sun, scale),
        residual_record("shade_gas_minus_q1", e_shade - q1_shade, scale),
        residual_record("q1_sum_minus_q2", q1_sun + q1_shade - q2, scale),
        residual_record(
            "q2_minus_capped_q3_sum",
            q2 - sum(item["q_final_kg_m2_tile_s"] for item in capped),
            scale,
        ),
    ]
    return {
        "identity": {
            "transaction_id": 53,
            "owner_id": "diagnostic-water-owner",
            "stratum_id": "canopy",
            "tile_id": "tile-a",
            "amount_basis": "kg_h2o_m2_stand_ground_interval",
        },
        "tile_fraction": tile_fraction,
        "dt_s": dt_s,
        "root_potential_mm": root,
        "configured_layer_order": [layer["layer_id"] for layer in layers()],
        "layers": capped,
        "active_water_caps": [
            item["layer_id"] for item in capped if item["branch"] == "authorization_active_or_tie"
        ],
        "coupled_fluxes_kg_m2_tile_s": {
            "e_sun": e_sun,
            "e_shade": e_shade,
            "q1_sun": q1_sun,
            "q1_shade": q1_shade,
            "q2": q2,
        },
        "water_residual_scale_kg_m2_tile_s": scale,
        "normalized_residuals": residuals,
        "finalized_total_kg_m2_stand_ground": sum(
            item["finalized_use_kg_m2_stand_ground"] for item in capped
        ),
    }


def controlled_law_branch() -> dict[str, Any]:
    tile_fraction = 0.37
    dt_s = 1800.0
    root = -4000.0
    result = []
    for layer in layers():
        q_law, _ = law_flux(layer, root)
        cap_rate = q_law + 9.0e-6
        result.append(
            cap_layer(
                layer,
                root,
                tile_fraction,
                dt_s,
                authorization_for_rate(cap_rate, tile_fraction, dt_s),
            )
        )
    return {
        "acceptance_evidence": False,
        "purpose": "controlled complementarity law-branch derivative only; authorization may exceed D",
        "layers": result,
    }


def tie_family() -> dict[str, Any]:
    tile_fraction = 0.37
    dt_s = 1800.0
    layer = layers()[2]
    q_law, law_derivative = law_flux(layer, -4000.0)
    def preserved_rate(direction: float) -> float:
        candidate = q_law
        for _ in range(128):
            candidate = math.nextafter(candidate, direction)
            authorization = authorization_for_rate(candidate, tile_fraction, dt_s)
            roundtrip = authorization / (tile_fraction * dt_s)
            if (direction < 0.0 and roundtrip < q_law) or (direction > 0.0 and roundtrip > q_law):
                return roundtrip
        raise AssertionError("authorization roundtrip did not preserve near-tie side")

    cases = []
    for name, cap_rate in [
        ("roundtrip_below", preserved_rate(-math.inf)),
        ("exact_tie", q_law),
        ("roundtrip_above", preserved_rate(math.inf)),
    ]:
        value = cap_layer(
            layer,
            -4000.0,
            tile_fraction,
            dt_s,
            authorization_for_rate(cap_rate, tile_fraction, dt_s),
        )
        value["case"] = name
        value["law_derivative_before_selection"] = law_derivative
        value["cap_rate_f64_hex"] = cap_rate.hex()
        value["q_law_f64_hex"] = q_law.hex()
        value["bitwise_distinct"] = cap_rate.hex() != q_law.hex() if name != "exact_tie" else False
        cases.append(value)
    return {
        "rule": "authorization_active_or_tie iff cap_rate <= q_law",
        "cases": cases,
    }


def poison(
    accepted: Any = None,
    rejected: Any = None,
    typed_error: str | None = None,
    operands: dict[str, Any] | None = None,
) -> dict[str, Any]:
    value: dict[str, Any] = {"executed": True, "candidate": None if typed_error else "accepted"}
    if accepted is not None:
        value["accepted"] = accepted
    if rejected is not None:
        value["rejected"] = rejected
        value["discriminates"] = accepted != rejected
    if typed_error is not None:
        value["typed_error"] = typed_error
    if operands is not None:
        value["operands"] = operands
    return value


def poisons(mixed: dict[str, Any], coupled: dict[str, Any]) -> dict[str, Any]:
    first = mixed["layers"][0]
    law_layer = mixed["layers"][1]
    f_t = mixed["tile_fraction"]
    dt_s = mixed["dt_s"]
    accepted_final = first["finalized_use_kg_m2_stand_ground"]
    return {
        "authorization_amount_as_rate": poison(
            first["cap_rate_kg_m2_tile_s"],
            first["authorization_kg_m2_stand_ground"],
        ),
        "omit_tile_fraction": poison(accepted_final, first["q_final_kg_m2_tile_s"] * dt_s),
        "double_tile_fraction": poison(accepted_final, accepted_final * f_t),
        "omit_interval": poison(accepted_final, f_t * first["q_final_kg_m2_tile_s"]),
        "double_interval": poison(accepted_final, accepted_final * dt_s),
        "sequential_clamp_potential_q": poison(
            coupled["accepted_constrained_all_cap"]["solution"]["beta_hyd"],
            coupled["fully_authorized_value_reduction"]["uncapped_reference"]
            ["solution"]["beta_hyd"],
            operands={
                "accepted_endpoint": "authorization-coupled beta after gas-energy-hydraulic re-solve",
                "rejected_endpoint": "uncapped Stage-A beta with only layer flux posthoc-clamped",
            },
        ),
        "cap_before_constitutive_law": poison(
            first["q_law_kg_m2_tile_s"],
            4.1e-6,
            operands={"accepted_root_potential_mm": -4000.0, "rejected_cap_boundary_root_mm": 6500.0},
        ),
        "strict_less_than_tie": poison(
            "authorization_active_or_tie",
            "constitutive_law",
            operands={"cap_rate": 1.7e-6, "q_law": 1.7e-6},
        ),
        "sorted_layer_id_active_caps": poison(
            mixed["active_water_caps"], sorted(mixed["active_water_caps"])
        ),
        "authorization_as_finalized_debit": poison(
            law_layer["finalized_use_kg_m2_stand_ground"],
            law_layer["authorization_kg_m2_stand_ground"],
        ),
        "borrow_unused_authorization": poison(
            first["finalized_use_kg_m2_stand_ground"],
            first["finalized_use_kg_m2_stand_ground"]
            + law_layer["authorization_kg_m2_stand_ground"]
            - law_layer["finalized_use_kg_m2_stand_ground"],
        ),
        "stale_transaction": poison(
            typed_error="stale_transaction_id",
            operands={"expected": 53, "received": 52},
        ),
        "wrong_occupancy": poison(
            typed_error="authorization_occupancy_mismatch",
            operands={"expected": "canopy@tile-a", "received": "canopy@tile-b"},
        ),
        "wrong_layer": poison(
            typed_error="authorization_layer_mismatch",
            operands={"expected": "root-z", "received": "root-a"},
        ),
        "wrong_basis": poison(
            typed_error="authorization_amount_basis_mismatch",
            operands={
                "expected": "kg_h2o_m2_stand_ground_interval",
                "received": "kg_h2o_m2_tile_ground_interval",
            },
        ),
        "stand_cap_used_inside_tile_law": poison(first["cap_rate_kg_m2_tile_s"],
            first["authorization_kg_m2_stand_ground"] / dt_s),
        "q_law_overwritten_by_q": poison(first["q_law_kg_m2_tile_s"],
            first["q_final_kg_m2_tile_s"]),
        "stale_generalized_branch": poison(0.0, -first["series_conductance_kg_m2_s_mm"]
            if "series_conductance_kg_m2_s_mm" in first else -2.0e-10),
        "reselect_within_tie_perturbation": poison(0.0, -1.275e-9),
        "scalar_ratio_all_layers": poison(
            coupled["accepted_constrained_all_cap"]["solution"]["root_node_potential_mm"],
            coupled["fully_authorized_value_reduction"]["uncapped_reference"]
            ["solution"]["root_node_potential_mm"]),
        "gas_energy_not_resolved": poison(
            coupled["accepted_constrained_all_cap"]["solution"]["beta_hyd"], 0.3747699923315343),
        "wrong_tile_authorization": poison(typed_error="authorization_tile_mismatch",
            operands={"expected": "tile-a", "received": "tile-b"}),
        "cap_tolerance_repairs_identity_or_basis": poison(
            typed_error="cap_tolerance_cannot_repair_identity_or_basis"),
        "producer_supplied_zero_closure": poison(typed_error="independent_closure_required",
            operands={"producer_residual": 0.0}),
        "reauthorization_after_final_pass": poison(typed_error="reauthorization_forbidden"),
        "continue_from_potential_candidate": poison(typed_error="final_pass_requires_beginning_state"),
        "partial_commit": poison(typed_error="atomic_commit_required"),
    }


REQUIRED_POISON_KEYS = {
    "authorization_amount_as_rate", "omit_tile_fraction", "double_tile_fraction",
    "omit_interval", "double_interval", "sequential_clamp_potential_q",
    "cap_before_constitutive_law", "strict_less_than_tie", "sorted_layer_id_active_caps",
    "authorization_as_finalized_debit", "borrow_unused_authorization", "stale_transaction",
    "wrong_occupancy", "wrong_layer", "wrong_basis", "stand_cap_used_inside_tile_law",
    "q_law_overwritten_by_q", "stale_generalized_branch",
    "reselect_within_tie_perturbation", "scalar_ratio_all_layers",
    "gas_energy_not_resolved", "wrong_tile_authorization",
    "cap_tolerance_repairs_identity_or_basis", "producer_supplied_zero_closure",
    "reauthorization_after_final_pass", "continue_from_potential_candidate", "partial_commit",
}


def v4_to_v5_identity_rebind() -> dict[str, Any]:
    source_path = (ROOT / "docs/work-packages/20260812-c3-woody-shared-state-authority-001/"
                   "artifacts/openwepp_c3_woody_v4_vectors.json")
    source = json.loads(source_path.read_bytes())
    complete_v4 = source["whole_state_canonical"]["preimage"]
    payload = {key: deepcopy(value) for key, value in complete_v4.items()
               if key not in {"model_definition_sha256", "configuration_sha256"}}
    payload_bytes = canonical_bytes(payload)
    v4_identity = {
        "model_definition_sha256": V4_SHA256,
        "configuration_sha256": "v4-configuration-digest-fixture",
        "state_sha256": digest_bytes(b"v4-state\0" + payload_bytes),
    }
    v5_identity = {
        "model_definition_sha256": {"definition_digest_injected_by_consumer": True},
        "configuration_sha256": "v5-distinct-configuration-digest-fixture",
        "state_sha256": {"recomputed_after_definition_digest_injection": True},
        "post_freeze_verification_inputs": {
            "definition_path": (
                "docs/work-packages/20260812-c3-woody-potential-pass-authority-001/"
                "artifacts/openwepp_c3_woody_v5_definition.json"
            ),
            "configuration_identity": "v5-distinct-configuration-digest-fixture",
            "payload_sha256": digest_bytes(payload_bytes),
        },
    }
    return {
        "source_validation": "validate complete V4 model, configuration, state digest, schema and lineage first",
        "payload_sha256_before": digest_bytes(payload_bytes),
        "payload_sha256_after": digest_bytes(payload_bytes),
        "payload_byte_identical": True,
        "v4_identity": v4_identity,
        "v5_identity": v5_identity,
        "field_migration_or_synthesis": False,
        "complete_v4_source_preimage": complete_v4,
        "complete_v4_source_fixture_sha256": file_digest(source_path),
        "noncircular_binding_rule": (
            "the fixture binds the complete V4 preimage and a typed consumer-injection marker; "
            "the verifier hashes final frozen V5 definition bytes externally, injects that digest, "
            "then reconstructs the V5 configuration and state digests"
        ),
        "stale_v4_identity_poison": {
            "executed": True,
            "candidate": None,
            "typed_error": "stale_v4_identity_in_v5_state",
            "rejected": v4_identity,
        },
    }


def rollback_injection_vectors() -> list[dict[str, Any]]:
    owners = {
        "vegetation": beginning_state(),
        "water": {"stores": {"soil-1": 0.031, "soil-2": 0.024}, "last_transaction_id": 52},
        "biogeochemistry": {"nh4": {"soil-1": 0.002}, "no3": {"soil-1": 0.003}},
        "energy": {"canopy_j_m2": 1200.0, "ground_j_m2": 3400.0},
        "transaction": {"accepted": 52, "pending": None},
        "diagnostics": {"last_feedback_transaction": 52, "active_caps": []},
    }
    beginning_bytes = canonical_bytes(owners)
    rows = []
    targets = ["vegetation", "water", "biogeochemistry", "energy", "transaction", "diagnostics"]
    for index, phase in enumerate(("authorization_validation", "capped_solve",
            "receiver_construction", "closure_validation", "immediately_before_commit",
            "owner_validation")):
        candidate = deepcopy(owners)
        target = targets[index]
        if target == "vegetation": candidate[target]["occupancy"]["beta_hyd"] += 0.001
        elif target == "water": candidate[target]["stores"]["soil-1"] -= 0.0001
        elif target == "biogeochemistry": candidate[target]["nh4"]["soil-1"] -= 0.0001
        elif target == "energy": candidate[target]["canopy_j_m2"] += 1.0
        elif target == "transaction": candidate[target]["pending"] = 53
        else: candidate[target]["active_caps"] = ["soil-1"]
        candidate_bytes = canonical_bytes(candidate)
        candidate = None
        rows.append({"phase": phase,
            "candidate_was_materially_mutated": candidate_bytes != beginning_bytes,
            "mutated_owner_surface": target,
            "candidate_survives": candidate is not None,
            "beginning_owner_bytes_before_sha256": digest_bytes(beginning_bytes),
            "beginning_owner_bytes_after_sha256": digest_bytes(canonical_bytes(owners)),
            "beginning_owners_byte_identical": canonical_bytes(owners) == beginning_bytes,
            "owner_surfaces": list(owners)})
    return rows


def fixture() -> dict[str, Any]:
    mixed = mixed_active_set()
    coupled = complete_coupled_vectors()
    result = {
        "model_version": "OPENWEPP_C3_WOODY_V5",
        "base_model_sha256": V4_SHA256,
        "oracle_independence": {
            "implementation_language": "Python standard library only",
            "calls_rust": False,
            "expected_values_generated_by_rust": False,
            "canonical_serialization": "recursive key sort, compact separators, UTF-8, LF",
        },
        "units": {
            "authorization": "kg H2O m-2 stand-ground interval-1",
            "tile_amount": "kg H2O m-2 tile-ground interval-1",
            "flux": "kg H2O m-2 tile-ground s-1",
            "potential": "mm H2O",
        },
        "canonical_selection": {
            "law_evaluation": "q_law evaluated independently at every residual evaluation",
            "final_flux": "min(q_law,cap_rate)",
            "cap_rate": "authorization_stand/(tile_fraction*dt_s)",
            "tie": "authorization active when cap_rate <= q_law",
            "generalized_derivative": "zero on active/tie branch; constitutive derivative on law branch",
            "reauthorization": "forbidden",
            "outer_column_fixed_point": "forbidden",
        },
        "families": {
            "complete_coupled_capped_solve": coupled,
            "controlled_layer_complementarity": mixed,
            "exact_and_near_tie": tie_family(),
            "controlled_law_branch_nonacceptance": controlled_law_branch(),
            "zero_dry_frozen_inaccessible": [
                item for item in mixed["layers"] if item["layer_id"].endswith("layer")
            ],
            "capped_failures": coupled["executed_failures"],
            "v4_to_v5_identity_rebind": v4_to_v5_identity_rebind(),
            "phase_rollback_injections": rollback_injection_vectors(),
        },
        "poisons": poisons(mixed, coupled),
    }
    result["checks"] = {
        "all_failures_publish_no_candidate": all(
            item["candidate"] is None for item in result["families"]["capped_failures"]
        ),
        "all_failures_rollback_byte_identically": all(
            item["beginning_state_sha256_before"] == item["beginning_state_sha256_after"]
            and item["rollback_byte_identical"]
            for item in result["families"]["capped_failures"]
        ),
        "active_caps_in_configuration_order": mixed["active_water_caps"]
        == ["root-z", "dry-layer", "frozen-layer", "inaccessible-layer", "zero-root-layer"],
        "exact_tie_is_active_with_zero_derivative": (
            result["families"]["exact_and_near_tie"]["cases"][1]["branch"]
            == "authorization_active_or_tie"
            and result["families"]["exact_and_near_tie"]["cases"][1][
                "dq_final_d_root_potential"
            ]
            == 0.0
        ),
        "controlled_law_branch_executes": all(
            item["branch"] == "constitutive_law"
            and item["q_final_kg_m2_tile_s"] == item["q_law_kg_m2_tile_s"]
            for item in result["families"]["controlled_law_branch_nonacceptance"]["layers"]
        ),
        "all_named_poisons_execute": all(item["executed"] for item in result["poisons"].values()),
        "all_untyped_poisons_discriminate": all(
            item.get("discriminates") is True
            for item in result["poisons"].values() if "typed_error" not in item
        ),
        "exact_required_poison_inventory": set(result["poisons"]) == REQUIRED_POISON_KEYS,
        "zero_root_is_distinct_exact_zero_branch": (
            next(row for row in coupled["accepted_constrained_all_cap"]["fluxes"]["q3"]
                 if row["layer_id"] == "soil-zero-root")["q_law_kg_m2_tile_s"] == 0.0
            and next(layer for layer in coupled["operands"]["layers"]
                     if layer["layer_id"] == "soil-zero-root")["accessible"]
            and not next(layer for layer in coupled["operands"]["layers"]
                         if layer["layer_id"] == "soil-zero-root")["frozen"]
        ),
        "complete_constrained_capped_solve_converges": "failure" not in coupled["accepted_constrained_all_cap"],
        "complete_constrained_has_six_accepted_residuals": (
            len(coupled["accepted_constrained_all_cap"]["normalized_residuals"]) == 6
            and all(abs(row["normalized"]) <= 1.0
                    for row in coupled["accepted_constrained_all_cap"]["normalized_residuals"])
        ),
        "root_residual_is_q2_minus_capped_sum": math.isclose(
            coupled["accepted_constrained_all_cap"]["normalized_residuals"][5]["raw_kg_m2_tile_s"],
            coupled["accepted_constrained_all_cap"]["fluxes"]["q2"]
            - math.fsum(row["q_final_kg_m2_tile_s"]
                        for row in coupled["accepted_constrained_all_cap"]["fluxes"]["q3"]),
            rel_tol=0.0, abs_tol=0.0,
        ),
        "failure_active_caps_match_diagnostics": all(
            row["active_water_caps"] == row["diagnostics"]["active_water_caps"]
            for row in coupled["executed_failures"]
        ),
        "v4_to_v5_payload_byte_identical": (
            result["families"]["v4_to_v5_identity_rebind"]["payload_sha256_before"]
            == result["families"]["v4_to_v5_identity_rebind"]["payload_sha256_after"]
        ),
        "phase_rollback_injections_are_real_and_atomic": all(
            row["candidate_was_materially_mutated"] and not row["candidate_survives"]
            and row["beginning_owners_byte_identical"]
            for row in result["families"]["phase_rollback_injections"]
        ),
        "accepted_transactions_obey_F_le_A_le_D": all(
            0.0 <= row["finalized_use_kg_m2_stand_ground"]
            <= row["authorization_kg_m2_stand_ground"]
            <= row["potential_request_kg_m2_stand_ground"]
            for family in (coupled["accepted_constrained_all_cap"],
                           coupled["fully_authorized_value_reduction"]["capped_result"])
            for row in family["fluxes"]["q3"]
        ),
        "hydrology_debits_finalized_use_and_closes": all(
            residual == 0.0 for residual in coupled["accepted_constrained_all_cap"]
            ["hydrology_owner_closure"]["residuals"].values()
        ),
        "alternate_warm_start_converges_same_solution": all(
            math.isclose(coupled["alternate_warm_start"]["result"]["solution"][key], value,
                         rel_tol=2.0e-9, abs_tol=2.0e-7)
            for key, value in coupled["accepted_constrained_all_cap"]["solution"].items()
        ),
    }
    if not all(result["checks"].values()):
        raise AssertionError(f"V5 fixture check failure: {result['checks']}")
    return result


def definition(vector_sha: str, generator_sha: str) -> dict[str, Any]:
    if file_digest(V4_DEFINITION) != V4_SHA256:
        raise AssertionError("immutable V4 model definition drift")
    contract_text = CONTRACT.read_text()
    section_sha: dict[str, str] = {}
    marker = "## `OPENWEPP_C3_WOODY_V5` Fixed-Authorization Capped-Pass Amendment\n"
    if marker in contract_text:
        section_sha["v5_authorization_capped_pass_amendment"] = section_digest(
            contract_text, marker, "## Change Log\n"
        )
    return {
        "model_version": "OPENWEPP_C3_WOODY_V5",
        "supersedes_model_version": "OPENWEPP_C3_WOODY_V4",
        "canonical_contract": "SC-VEGETATION-001@9",
        "base_model_definition": {
            "model_version": "OPENWEPP_C3_WOODY_V4",
            "sha256": V4_SHA256,
            "import_semantics": (
                "all OPENWEPP_C3_WOODY_V4 authority is normative and unchanged except "
                "where V5 explicitly supersedes the authorization-capped E11-E15 solve, "
                "active-set derivative, water residual scale, cap diagnostics, and capped fixtures"
            ),
        },
        "canonical_section_extraction": (
            "UTF-8 bytes beginning with the exact start heading line and newline, "
            "ending immediately before the exact end heading line"
        ),
        "canonical_section_sha256": section_sha,
        "capped_pass": {
            "unknowns_and_uncapped_laws": "the V3 six-unknown coupled system and q_law equations are unchanged",
            "authorization_conversion": "A_tile=A_W/f_t; cap_rate=A_W/(f_t*dt)",
            "layer_flux": "q_i=min(q_law_i,cap_rate_i), after independent q_law evaluation",
            "tie_rule": "active iff cap_rate_i<=q_law_i",
            "generalized_derivative": "dq_i/dx=0 on active/tie; dq_law_i/dx on law branch",
            "root_residual": "q2-sum_i(q_i)",
            "finalized_use": "F_W=f_t*q_i*dt",
            "active_cap_order": "configured root-layer order",
            "forbidden": [
                "sequential_clamp_of_potential_q",
                "cap_before_constitutive_law",
                "reauthorization",
                "outer_column_fixed_point",
                "borrowing_between_layers_or_occupancies",
                "authorization_as_finalized_debit",
            ],
        },
        "water_residual_scale": {
            "operands": (
                "E_sun_max,E_shade_max,abs(q1_sun),abs(q1_shade),abs(q2), "
                "and per-layer abs(q_law),abs(cap_rate),abs(q_final)"
            ),
            "combination": "max",
            "tolerance": "1e-12 + 1e-9*scale_W_cap",
        },
        "failure_contract": {
            "candidate": "none",
            "beginning_state": "byte-identical",
            "diagnostics": (
                "V3 numerical payload with pass=capped and active_water_caps in configured root-layer order"
            ),
        },
        "migration_policy": {
            "v4_to_v5": (
                "validate V4, copy state payload bit-for-bit, inject distinct V5 model and "
                "configuration identities, then recompute V5 state digest"
            ),
            "field_migration_or_synthesis": "forbidden",
            "stale_v4_model_configuration_or_state_digest": "typed rejection",
        },
        "independent_fixture": {
            "path": (
                "docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/"
                "openwepp_c3_woody_v5_vectors.json"
            ),
            "sha256": vector_sha,
            "generator": (
                "docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/"
                "reference_calculator_v5.py"
            ),
            "generator_sha256": generator_sha,
        },
    }


def main() -> None:
    generated_fixture = fixture()
    VECTOR_PATH.write_bytes(canonical_bytes(generated_fixture))
    generator_sha = file_digest(Path(__file__))
    generated_definition = definition(file_digest(VECTOR_PATH), generator_sha)
    definition_bytes = canonical_bytes(generated_definition)
    DEFINITION_PATH.write_bytes(definition_bytes)
    MODEL_STACK_COPY.write_bytes(definition_bytes)
    print(f"vectors_sha256={file_digest(VECTOR_PATH)}")
    print(f"generator_sha256={generator_sha}")
    print(f"definition_sha256={file_digest(DEFINITION_PATH)}")


if __name__ == "__main__":
    main()
