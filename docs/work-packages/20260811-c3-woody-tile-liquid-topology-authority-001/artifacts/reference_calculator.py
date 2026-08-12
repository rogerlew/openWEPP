#!/usr/bin/env python3
"""Independent OPENWEPP_C3_WOODY_V2 topology/interception oracle."""

from __future__ import annotations

import json
import hashlib
import importlib.util
import math
from dataclasses import asdict, dataclass
from pathlib import Path


@dataclass(frozen=True)
class E04Result:
    store1: float
    intercepted: float
    throughfall: float
    stemflow: float
    initial_drainage: float
    second_drainage: float
    wet_evaporation: float
    condensation: float
    wet_fraction: float


def e04(*, store0: float, rain: float, vapor: float, lai: float, wai: float,
        alpha: float, p_liq: float, f_stem: float) -> E04Result:
    for value in (store0, rain, vapor, lai, wai, alpha, p_liq, f_stem):
        if not math.isfinite(value):
            raise ValueError("nonfinite E04 input")
    if min(store0, rain, lai, wai, alpha, p_liq) < 0.0 or not 0.0 <= f_stem <= 1.0:
        raise ValueError("E04 domain")
    plant_area = lai + wai
    intercepted = alpha * math.tanh(plant_area) * rain
    free = rain - intercepted
    stemflow = f_stem * free
    throughfall = (1.0 - f_stem) * free
    capacity = p_liq * plant_area
    pre = store0 + intercepted
    initial_drainage = max(0.0, pre - capacity)
    store = pre - initial_drainage
    wet_fraction = 0.0 if capacity == 0.0 else (store / capacity) ** (2.0 / 3.0)
    wet_evaporation = min(vapor, store) if vapor > 0.0 else 0.0
    condensation = -vapor if vapor < 0.0 else 0.0
    store = store - wet_evaporation + condensation
    second_drainage = max(0.0, store - capacity)
    store -= second_drainage
    result = E04Result(store, intercepted, throughfall, stemflow,
                       initial_drainage, second_drainage, wet_evaporation,
                       condensation, wet_fraction)
    residual = (store0 + rain + condensation - store - wet_evaporation
                - throughfall - stemflow - initial_drainage - second_drainage)
    if abs(residual) > 2e-15:
        raise AssertionError(f"local closure {residual}")
    return result


def column(rain: float, occupancies: list[dict[str, float | str]]) -> dict:
    incident = rain
    ground_stem = 0.0
    rows = []
    for occupancy in occupancies:
        result = e04(
            store0=float(occupancy["store0"]), rain=incident,
            vapor=float(occupancy["vapor"]), lai=float(occupancy["lai"]),
            wai=float(occupancy["wai"]), alpha=float(occupancy["alpha"]),
            p_liq=float(occupancy["p_liq"]), f_stem=float(occupancy["f_stem"]),
        )
        rows.append({"stratum": occupancy["stratum"],
                     "store0": float(occupancy["store0"]), "incident": incident,
                     **asdict(result)})
        ground_stem += result.stemflow
        incident = result.throughfall + result.initial_drainage + result.second_drainage
    ground = ground_stem + incident
    condensation = sum(float(row["condensation"]) for row in rows)
    evaporation = sum(float(row["wet_evaporation"]) for row in rows)
    stores0 = sum(float(item["store0"]) for item in occupancies)
    stores1 = sum(float(row["store1"]) for row in rows)
    residual = stores0 + rain + condensation - stores1 - evaporation - ground
    if abs(residual) > 4e-15:
        raise AssertionError(f"column closure {residual}")
    return {"occupancies": rows, "ground_liquid": ground, "closure_residual": residual}


def validate_lanes(expected: set[tuple[str, str]], lanes: list[tuple[str, str]]) -> bool:
    return len(lanes) == len(set(lanes)) and set(lanes) == expected


def rollback_after_tile_failure(beginning: dict) -> tuple[str, str]:
    """Build and discard a candidate after an injected tile-local failure."""
    before = json.dumps(beginning, sort_keys=True, separators=(",", ":"))
    candidate = json.loads(before)  # JSON round-trip guarantees no mutable alias.
    try:
        candidate["vegetation"]["lanes"][0]["state"]["canopy_liquid_kg_h2o_m2_tile_ground"] = 999.0
        candidate["vegetation"]["shared_cn"]["leaf_c"] = 0.0
        candidate["water"]["soil_layer_liquid"]["soil-1"] = 0.0
        candidate["biogeochemistry"]["mineral_n"]["soil-1:NH4"] = 0.0
        candidate["energy"]["canopy_j"] = -999.0
        candidate["transaction"] += 1
        raise RuntimeError("injected tile-local solve failure")
    except RuntimeError:
        pass
    after = json.dumps(beginning, sort_keys=True, separators=(",", ":"))
    return before, after


def proportional_authorize(supply: float, requests: dict[tuple, float]) -> dict[tuple, float]:
    if supply < 0.0 or any(amount < 0.0 for amount in requests.values()):
        raise ValueError("negative resource amount")
    total = math.fsum(requests.values())
    scale = 1.0 if total <= supply or total == 0.0 else supply / total
    return {key: amount * scale for key, amount in requests.items()}


def require_identity(expected: tuple, actual: tuple) -> None:
    if expected != actual:
        raise ValueError("typed identity mismatch")


def load_v1_authority_oracle():
    path = (Path(__file__).parents[2]
            / "20260811-coupled-c3-forest-vegetation-model-stack-authority-001"
            / "artifacts/reference_calculator.py")
    spec = importlib.util.spec_from_file_location("openwepp_c3_v1_oracle", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("load digest-bound V1 authority oracle")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def canonical_state_bytes(state: dict) -> str:
    return json.dumps(state, sort_keys=True, separators=(",", ":"))


def vectors() -> dict:
    v1_oracle = load_v1_authority_oracle()
    fractions = {"tile-a": 0.35, "tile-b": 0.65}
    tile_a = column(2.4, [
        {"stratum": "upper", "store0": 0.11, "vapor": 0.015,
         "lai": 2.8, "wai": 0.45, "alpha": 0.72, "p_liq": 0.18,
         "f_stem": 0.12},
        {"stratum": "lower", "store0": 0.04, "vapor": -0.09,
         "lai": 1.7 / 0.35, "wai": 0.22 / 0.35, "alpha": 0.61,
         "p_liq": 0.14, "f_stem": 0.08},
    ])
    tile_b = column(0.7, [
        {"stratum": "upper", "store0": 0.37, "vapor": -0.31,
         "lai": 2.8, "wai": 0.45, "alpha": 0.72, "p_liq": 0.18,
         "f_stem": 0.12},
        {"stratum": "lower", "store0": 0.19, "vapor": 0.04,
         "lai": 1.7 / 0.35, "wai": 0.22 / 0.35, "alpha": 0.61,
         "p_liq": 0.14, "f_stem": 0.08},
    ])
    empty = column(1.3, [])
    stand_ground = sum(fractions[t] * value["ground_liquid"]
                       for t, value in (("tile-a", tile_a), ("tile-b", tile_b)))
    stand_store = (fractions["tile-a"] * sum(r["store1"] for r in tile_a["occupancies"])
                   + fractions["tile-b"] * sum(r["store1"] for r in tile_b["occupancies"]))
    demand_tile = 0.42
    authorization_stand = 0.091
    demand_stand = fractions["tile-a"] * demand_tile
    authorization_tile = authorization_stand / fractions["tile-a"]
    final_tile = 0.24
    final_stand = fractions["tile-a"] * final_tile
    if not 0.0 <= final_stand <= authorization_stand <= demand_stand:
        raise AssertionError("water transaction bounds")

    local_a = e04(store0=0.02, rain=2.2, vapor=0.0, lai=3.0, wai=0.4,
                  alpha=0.7, p_liq=0.2, f_stem=0.1)
    local_b = e04(store0=0.41, rain=0.1, vapor=0.0, lai=3.0, wai=0.4,
                  alpha=0.7, p_liq=0.2, f_stem=0.1)
    weighted_store = 0.35 * local_a.store1 + 0.65 * local_b.store1
    aggregate = e04(store0=0.35 * 0.02 + 0.65 * 0.41,
                    rain=0.35 * 2.2 + 0.65 * 0.1, vapor=0.0,
                    lai=3.0, wai=0.4, alpha=0.7, p_liq=0.2, f_stem=0.1)
    if abs(weighted_store - aggregate.store1) < 1e-6:
        raise AssertionError("nonlinear poison did not distinguish")

    single = column(1.1, [{"stratum": "single", "store0": 0.12, "vapor": 0.03,
                           "lai": 2.0, "wai": 0.3, "alpha": 0.65,
                           "p_liq": 0.16, "f_stem": 0.09}])
    legacy_single = e04(store0=0.12, rain=1.1, vapor=0.03, lai=2.0,
                        wai=0.3, alpha=0.65, p_liq=0.16, f_stem=0.09)
    homogeneous_a = column(1.1, [{"stratum": "shared", "store0": 0.12,
                                  "vapor": 0.03, "lai": 2.0, "wai": 0.3,
                                  "alpha": 0.65, "p_liq": 0.16,
                                  "f_stem": 0.09}])
    homogeneous_b = column(1.1, [{"stratum": "shared", "store0": 0.12,
                                  "vapor": 0.03, "lai": 2.0, "wai": 0.3,
                                  "alpha": 0.65, "p_liq": 0.16,
                                  "f_stem": 0.09}])
    potential_occupancies = [
        {"stratum": "upper", "store0": 0.2, "vapor": 0.12, "lai": 2.4,
         "wai": 0.4, "alpha": 0.7, "p_liq": 0.15, "f_stem": 0.1},
        {"stratum": "lower", "store0": 0.03, "vapor": 0.02, "lai": 1.5,
         "wai": 0.2, "alpha": 0.6, "p_liq": 0.13, "f_stem": 0.07},
    ]
    potential_column = column(1.8, potential_occupancies)
    water_keys = {
        "upper": ("tx-9", "upper", "tile-a", "soil-1", "liquid-water", "stand-ground"),
        "lower": ("tx-9", "lower", "tile-a", "soil-1", "liquid-water", "stand-ground"),
    }
    potential_requests = {water_keys["upper"]: 0.12, water_keys["lower"]: 0.08}
    water_authorizations = proportional_authorize(0.10, potential_requests)
    upper_fraction = water_authorizations[water_keys["upper"]] / potential_requests[water_keys["upper"]]
    controlled_final_occupancies = [dict(item) for item in potential_occupancies]
    controlled_final_occupancies[0]["vapor"] = -0.06
    controlled_final_column = column(1.8, controlled_final_occupancies)
    expected_lanes = {("upper", "tile-a"), ("upper", "tile-b"),
                      ("lower", "tile-a")}
    warm_start = {
        "beta_hyd": 0.73,
        "canopy_air_specific_humidity_kg_kg": 0.009,
        "canopy_air_temperature_k": 295.4,
        "canopy_liquid_kg_h2o_m2_tile_ground": 0.17,
        "dry_stem_temperature_k": 294.8,
        "last_accepted_transaction_id": 4,
        "root_potential_mm_by_layer": [["soil-1", -5100.0], ["soil-2", -6200.0]],
        "shade_ci_pa": 27.1,
        "shade_leaf_potential_mm": -7100.0,
        "shade_leaf_temperature_k": 295.0,
        "stem_potential_mm": -6600.0,
        "sun_ci_pa": 25.9,
        "sun_leaf_potential_mm": -7400.0,
        "sun_leaf_temperature_k": 296.2,
        "wet_surface_temperature_k": 294.9,
    }
    beginning_owners = {
        "vegetation": {"lanes": [{"identity": list(lane), "state": warm_start}
                                  for lane in sorted(expected_lanes)],
                       "shared_cn": {"leaf_c": 0.24, "nsc_c": 0.004}},
        "water": {"soil_layer_liquid": {"soil-1": 1.2, "soil-2": 0.9}},
        "biogeochemistry": {"mineral_n": {"soil-1:NH4": 0.03,
                                             "soil-1:NO3": 0.04,
                                             "soil-2:NH4": 0.02}},
        "energy": {"canopy_j": 82.0, "ground_j": 17.0},
        "transaction": 4,
    }
    initial_bytes, after_failure_bytes = rollback_after_tile_failure(beginning_owners)
    canonical_state = canonical_state_bytes(warm_start)
    local_gpp = {"tile-a": 0.021, "tile-b": 0.006}
    local_resp = {"tile-a": 0.004, "tile-b": 0.0015}
    gpp_stand = sum(fractions[t] * local_gpp[t] for t in fractions)
    resp_stand = sum(fractions[t] * local_resp[t] for t in fractions)
    mineral_requests = {
        ("tx-9", "upper", "soil-1", "NH4"): 0.011,
        ("tx-9", "lower", "soil-1", "NH4"): 0.023,
        ("tx-9", "upper", "soil-1", "NO3"): 0.007,
        ("tx-9", "upper", "soil-2", "NH4"): 0.005,
    }
    n_authorizations = {}
    for layer, species, supply in [("soil-1", "NH4", 0.017),
                                   ("soil-1", "NO3", 0.020),
                                   ("soil-2", "NH4", 0.002)]:
        group = {key: amount for key, amount in mineral_requests.items()
                 if key[2:] == (layer, species)}
        n_authorizations.update(proportional_authorize(supply, group))
    n_finalized = {key: amount * 0.8 for key, amount in n_authorizations.items()}
    n_inventory_ending = {
        ("soil-1", "NH4"): 0.017 - math.fsum(amount for key, amount in n_finalized.items()
                                                if key[2:] == ("soil-1", "NH4")),
        ("soil-1", "NO3"): 0.020 - math.fsum(amount for key, amount in n_finalized.items()
                                                if key[2:] == ("soil-1", "NO3")),
        ("soil-2", "NH4"): 0.002 - math.fsum(amount for key, amount in n_finalized.items()
                                                if key[2:] == ("soil-2", "NH4")),
    }
    n_expected_key = next(key for key in n_authorizations if key[2:] == ("soil-1", "NO3"))
    wrong_n_species = n_expected_key[:3] + ("NH4",)
    wrong_n_layer = n_expected_key[:2] + ("soil-2", n_expected_key[3])
    n_species_rejected = n_layer_rejected = False
    try:
        require_identity(n_expected_key, wrong_n_species)
    except ValueError:
        n_species_rejected = True
    try:
        require_identity(n_expected_key, wrong_n_layer)
    except ValueError:
        n_layer_rejected = True
    authorization_by_occupancy = {("upper", "tile-a"): 0.091,
                                  ("upper", "tile-b"): 0.052}
    expected_water_key = water_keys["upper"]
    swapped_water_key = ("tx-9", "upper", "tile-b", "soil-1",
                         "liquid-water", "stand-ground")
    wrong_authorization_rejected = False
    try:
        require_identity(expected_water_key, swapped_water_key)
    except ValueError:
        wrong_authorization_rejected = True
    accepted_lower_incident = (tile_a["occupancies"][0]["throughfall"]
                               + tile_a["occupancies"][0]["initial_drainage"]
                               + tile_a["occupancies"][0]["second_drainage"])
    wrong_stemflow_incident = accepted_lower_incident + tile_a["occupancies"][0]["stemflow"]
    wrong_tile_route_rejected = False
    try:
        require_identity(("tile-a", "upper", "lower"),
                         ("tile-b", "upper", "lower"))
    except ValueError:
        wrong_tile_route_rejected = True
    wet_case_a = e04(store0=0.02, rain=0.0, vapor=0.0, lai=3.0, wai=0.4,
                     alpha=0.7, p_liq=0.2, f_stem=0.1)
    wet_case_b = e04(store0=0.40, rain=0.0, vapor=0.0, lai=3.0, wai=0.4,
                     alpha=0.7, p_liq=0.2, f_stem=0.1)
    local_wet_a = wet_case_a.wet_fraction
    local_wet_b = wet_case_b.wet_fraction
    wet_energy_args = (240.0, 330.0, 410.0, 296.0, 0.010, 3.2, 0.9)
    wet_energy_a = v1_oracle.wet_canopy_temperature(
        *wet_energy_args, local_wet_a, 1.8, 0.04)
    wet_energy_b = v1_oracle.wet_canopy_temperature(
        *wet_energy_args, local_wet_b, 1.8, 0.04)
    weighted_wet_response = (fractions["tile-a"] * wet_energy_a["evaporation"]
                             + fractions["tile-b"] * wet_energy_b["evaporation"])
    averaged_wet_fraction = (fractions["tile-a"] * local_wet_a
                             + fractions["tile-b"] * local_wet_b)
    averaged_wet_response = v1_oracle.wet_canopy_temperature(
        *wet_energy_args, averaged_wet_fraction, 1.8, 0.04)["evaporation"]
    par_a, par_b = 900.0, 120.0
    fvbc_args = (28.0, 70.0, 120.0, 1.2)
    weighted_fvbc = (fractions["tile-a"] * v1_oracle.fvbc(par_a, *fvbc_args)["an"]
                     + fractions["tile-b"] * v1_oracle.fvbc(par_b, *fvbc_args)["an"])
    aggregated_fvbc = v1_oracle.fvbc(
        fractions["tile-a"] * par_a + fractions["tile-b"] * par_b,
        *fvbc_args)["an"]
    homogeneous_weighted_ground = (fractions["tile-a"] * homogeneous_a["ground_liquid"]
                                   + fractions["tile-b"] * homogeneous_b["ground_liquid"])
    shared_cn_duplicate = 2.0 * (gpp_stand - resp_stand)
    shared_cn_once_value = gpp_stand - resp_stand

    checks = {
        "unequal_tile_fractions": fractions["tile-a"] != fractions["tile-b"],
        "distinct_tile_rain": tile_a["occupancies"][0]["incident"] != tile_b["occupancies"][0]["incident"],
        "distinct_beginning_store": tile_a["occupancies"][0]["store1"] != tile_b["occupancies"][0]["store1"],
        "heterogeneous_upper_columns": (tile_a["occupancies"][1]["stratum"] == "lower"
                                        and tile_b["occupancies"][1]["stratum"] == "lower"
                                        and tile_a["occupancies"][1]["incident"]
                                        != tile_b["occupancies"][1]["incident"]),
        "two_rank_routing": tile_a["occupancies"][1]["incident"] > 0.0,
        "stemflow_bypass": (tile_a["occupancies"][1]["incident"]
                            == tile_a["occupancies"][0]["throughfall"]
                            + tile_a["occupancies"][0]["initial_drainage"]
                            + tile_a["occupancies"][0]["second_drainage"]),
        "condensation_second_drainage": tile_b["occupancies"][0]["second_drainage"] > 0.0,
        "empty_tile": empty["ground_liquid"] == 1.3,
        "single_tile_reduction": (single["occupancies"][0]["store1"] == legacy_single.store1
                                  and single["ground_liquid"] == legacy_single.throughfall
                                  + legacy_single.stemflow + legacy_single.initial_drainage
                                  + legacy_single.second_drainage),
        "homogeneous_two_tile_reduction": (homogeneous_a == homogeneous_b
                                           and homogeneous_weighted_ground
                                           == single["ground_liquid"]),
        "tile_order_permutation": sum(fractions[t] * v["ground_liquid"] for t, v in reversed(list((("tile-a", tile_a), ("tile-b", tile_b))))) == stand_ground,
        "request_weighting": demand_stand == fractions["tile-a"] * demand_tile,
        "authorization_back_conversion": authorization_tile == authorization_stand / fractions["tile-a"],
        "local_and_stand_closure": max(abs(tile_a["closure_residual"]), abs(tile_b["closure_residual"])) < 1e-14,
        "rollback_exact_bytes": initial_bytes == after_failure_bytes,
        "controlled_final_release_changes_lower": (potential_column["occupancies"][1]["incident"]
                                                   != controlled_final_column["occupancies"][1]["incident"]),
        "replicated_store_poison": abs((0.02 + 0.41) - (0.35 * 0.02 + 0.65 * 0.41)) > 1e-6,
        "wrong_area_basis_poison": abs(3.0 - 3.0 / 0.35) > 1e-6,
        "aggregate_incident_poison": abs(weighted_store - aggregate.store1) > 1e-6,
        "wrong_tile_drainage_poison": wrong_tile_route_rejected,
        "stemflow_through_foliage_poison": wrong_stemflow_incident != accepted_lower_incident,
        "omit_second_drainage_poison": tile_b["occupancies"][0]["second_drainage"] != 0.0,
        "double_ft_poison": fractions["tile-a"] * demand_stand != demand_stand,
        "omit_ft_poison": demand_tile != demand_stand,
        "average_wet_fraction_poison": not math.isclose(weighted_wet_response,
                                                         averaged_wet_response),
        "aggregate_par_poison": not math.isclose(weighted_fvbc, aggregated_fvbc),
        "wrong_authorization_poison": wrong_authorization_rejected,
        "duplicate_lane_rejected": not validate_lanes(expected_lanes, [("upper", "tile-a"), ("upper", "tile-a"), ("lower", "tile-a")]),
        "missing_lane_rejected": not validate_lanes(expected_lanes, [("upper", "tile-a"), ("upper", "tile-b")]),
        "shared_cn_once": (shared_cn_once_value != shared_cn_duplicate
                           and gpp_stand == 0.35 * 0.021 + 0.65 * 0.006
                           and resp_stand == 0.35 * 0.004 + 0.65 * 0.0015),
        "mineral_n_after_aggregation": (len(mineral_requests) == 4
                                        and n_species_rejected and n_layer_rejected
                                        and all(amount >= 0.0 for amount in n_inventory_ending.values())),
    }
    if not all(checks.values()):
        raise AssertionError({name: value for name, value in checks.items() if not value})

    return {
        "model_version": "OPENWEPP_C3_WOODY_V2",
        "fractions": fractions,
        "heterogeneous_columns": {"tile-a": tile_a, "tile-b": tile_b},
        "empty_tile": empty,
        "stand": {"ground_liquid": stand_ground, "ending_store": stand_store},
        "water_transaction": {
            "demand_tile": demand_tile, "demand_stand": demand_stand,
            "authorization_stand": authorization_stand,
            "authorization_tile": authorization_tile,
            "final_tile": final_tile, "final_stand": final_stand,
        },
        "nonlinear_poison": {
            "weighted_local_store": weighted_store,
            "aggregate_first_store": aggregate.store1,
            "difference": weighted_store - aggregate.store1,
        },
        "migration": {
            "zero_multitile": [0.0, 0.0],
            "single_tile_coverage_0_35_from_0_07": 0.07 / 0.35,
            "nonzero_multitile": "typed_unresolved_occupancy_lanes",
        },
        "potential_and_controlled_final_columns": {
            "potential": potential_column,
            "controlled_final": controlled_final_column,
        },
        "water_arbitration_and_routing_control": {
            "potential_requests": [{"key": list(key), "amount": amount}
                                   for key, amount in potential_requests.items()],
            "supply": 0.10,
            "authorizations": [{"key": list(key), "amount": amount}
                               for key, amount in water_authorizations.items()],
            "upper_authorization_fraction": upper_fraction,
            "controlled_potential_vapor": 0.12,
            "controlled_final_vapor": controlled_final_occupancies[0]["vapor"],
            "lower_potential_incident": potential_column["occupancies"][1]["incident"],
            "lower_controlled_final_incident": controlled_final_column["occupancies"][1]["incident"],
            "claim_scope": "topology_causality_only_exogenous_vapor_operands",
            "complete_coupled_acceptance_gate": "STAGE_B_E11_E15_EXACT_ORACLE",
        },
        "owner_rollback": {
            "beginning_sha256": hashlib.sha256(initial_bytes.encode()).hexdigest(),
            "after_failure_sha256": hashlib.sha256(after_failure_bytes.encode()).hexdigest(),
            "owner_names": sorted(key for key in beginning_owners if key != "transaction"),
        },
        "canonical_state": {
            "serialized": canonical_state,
            "sha256": hashlib.sha256(canonical_state.encode()).hexdigest(),
            "wrong_unit_poison": "typed_unit_mismatch:MPa",
        },
        "mineral_n_transaction": {
            "requests": [{"key": list(key), "amount": amount}
                         for key, amount in mineral_requests.items()],
            "authorizations": [{"key": list(key), "amount": amount}
                               for key, amount in n_authorizations.items()],
            "finalized": [{"key": list(key), "amount": amount}
                          for key, amount in n_finalized.items()],
            "ending_inventory": [{"key": list(key), "amount": amount}
                                 for key, amount in n_inventory_ending.items()],
            "wrong_species_result": "typed_identity_mismatch",
            "wrong_layer_result": "typed_identity_mismatch",
        },
        "nonlinear_locality": {
            "weighted_fvbc": weighted_fvbc,
            "aggregate_par_fvbc": aggregated_fvbc,
            "weighted_wet_response": weighted_wet_response,
            "averaged_wet_response": averaged_wet_response,
        },
        "shared_carbon_nitrogen": {"gpp_stand": gpp_stand, "leaf_respiration_stand": resp_stand,
                                   "accepted_once": shared_cn_once_value,
                                   "duplicate_transition_poison": shared_cn_duplicate,
                                   "mineral_n_identity": ["soil-1:NH4", "soil-1:NO3", "soil-2:NH4"]},
        "checks": checks,
        "all_pass": all(checks.values()),
    }


if __name__ == "__main__":
    print(json.dumps(vectors(), indent=2, sort_keys=True))
