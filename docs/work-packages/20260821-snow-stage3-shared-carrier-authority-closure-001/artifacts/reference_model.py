#!/usr/bin/env python3
"""Independent Child 2C carrier and event-boundary contract oracle.

This is a result-independent ledger/checking model.  It is intentionally not
an implementation import and does not call Rust or a production executable.
"""

import json
import hashlib
from pathlib import Path

ROOT = Path(__file__).parent
VECTORS = ROOT / "carrier-boundary-vectors.json"
SIGMA = 5.670374419e-8


def wire(value):
    """Parse a canonical decimal u128 string and return its integer value."""
    if not isinstance(value, str) or value == "" or (len(value) > 1 and value[0] == "0") or any(character < "0" or character > "9" for character in value):
        raise ValueError("noncanonical tick")
    parsed = int(value)
    if parsed > 2**128 - 1:
        raise ValueError("u128 overflow")
    return parsed


def wire_out(value):
    return str(value)


def reject(case, error):
    inputs = case["inputs"]
    owner_digest = inputs.get("before_owner_digest")
    if not owner_digest:
        raise ValueError(f"missing immutable owner digest for {case['id']}")
    result = {"id": case["id"], "status": "rejected", "error": error, "retry": False, "after_sha256": inputs.get("before_sha256", ""), "before_owner_digest": owner_digest, "staged_owner_digest": f"staged-{case['id']}", "after_owner_digest": owner_digest}
    if error == "ERR-CT-021":
        result["retry"] = True
    return result


def carrier(case):
    inputs = case["inputs"]
    if inputs.get("wind_operand") == "raw_10m":
        return reject(case, "SNOWENERGY-E-WIND-001")
    if inputs.get("canopy_air_nodes", 1) != 1:
        return reject(case, "SNOWENERGY-E-CARRIER-001")
    if inputs.get("canopy_intercepted_snow", False):
        return reject(case, "SNOWENERGY-E-SCOPE-001")
    if inputs.get("snow_flux_phase") != inputs.get("segment_phase", inputs.get("snow_flux_phase")):
        return reject(case, "SNOWENERGY-E-REGIME-001")
    exposure = inputs.get("exposure")
    if exposure is None or exposure.get("source") != "sealed-exposure-v1" or exposure.get("provider") != "sealed-stage3-exposure" or exposure.get("provider_digest") == "" or exposure.get("transfer_height_m") != 5.0 or exposure.get("roughness_m") != 0.005 or exposure.get("wind_m_s", 0) <= 0:
        return reject(case, "SNOWENERGY-E-WIND-001")

    participants = inputs["active_participants"]
    receipts = inputs["support_receipts"]
    receipt_ids = [receipt["receipt_id"] for receipt in receipts]
    if (len(participants) != len(set(participants))
            or sorted(participants) != participants
            or {r["participant_id"] for r in receipts} != set(participants)
            or len(receipt_ids) != len(set(receipt_ids))
            or any(not receipt_id for receipt_id in receipt_ids)):
        return reject(case, "SNOWENERGY-E-CARRIER-001")
    support = max(wire(r["minimum_support_ns"]) for r in receipts)

    rho = inputs["rho_air_kg_m3"]
    cp = inputs["cp_air_j_kg_k"]
    ref, canopy, snow = inputs["reference"], inputs["canopy"], inputs["snow"]
    heat_total = ref["heat_conductance_m_s"] + canopy["heat_conductance_m_s"] + snow["heat_conductance_m_s"]
    vapor_total = ref["vapor_conductance_m_s"] + canopy["vapor_conductance_m_s"] + snow["vapor_conductance_m_s"]
    t_ca = (ref["heat_conductance_m_s"] * ref["temperature_k"] + canopy["heat_conductance_m_s"] * canopy["temperature_k"] + snow["heat_conductance_m_s"] * snow["temperature_k"]) / heat_total
    q_ca = (ref["vapor_conductance_m_s"] * ref["specific_humidity"] + canopy["vapor_conductance_m_s"] * canopy["specific_humidity"] + snow["vapor_conductance_m_s"] * snow["specific_humidity"]) / vapor_total
    h_ref = rho * cp * ref["heat_conductance_m_s"] * (ref["temperature_k"] - t_ca)
    h_canopy = -rho * cp * canopy["heat_conductance_m_s"] * (canopy["temperature_k"] - t_ca)
    h_snow = -rho * cp * snow["heat_conductance_m_s"] * (snow["temperature_k"] - t_ca)
    v_ref = rho * ref["vapor_conductance_m_s"] * (ref["specific_humidity"] - q_ca)
    v_canopy = -rho * canopy["vapor_conductance_m_s"] * (canopy["specific_humidity"] - q_ca)
    v_snow = -rho * snow["vapor_conductance_m_s"] * (snow["specific_humidity"] - q_ca)
    f_sky = (1.0 - inputs["effective_canopy_cover"]) ** 1.6
    t_s = snow["temperature_k"]
    components = inputs["canopy_longwave_components"]
    if len(components) < 2 or abs(sum(c["emissive_area_weight"] for c in components) - 1.0) > 1e-12:
        return reject(case, "SNOWENERGY-E-LW-001")
    l_can = sum(c["emissive_area_weight"] * SIGMA * c["temperature_k"] ** 4 for c in components)
    l_down = f_sky * inputs["atmospheric_longwave_w_m2"] + (1.0 - f_sky) * l_can
    l_net = l_down - SIGMA * t_s**4
    l_exchange = (1.0 - f_sky) * (SIGMA * t_s**4 - l_can)

    ledger = inputs["state_ledger"]
    snow_end = ledger["snow_ice_start_kg_m2"] + ledger["solid_precipitation_kg_m2"] - ledger["melt_kg_m2"] - ledger["sublimation_kg_m2"] + ledger["deposition_kg_m2"]
    liquid_end = ledger["liquid_start_kg_m2"] + ledger["rain_kg_m2"] + ledger["melt_kg_m2"] - ledger["refreeze_kg_m2"] - ledger["liquid_runoff_kg_m2"]
    energy_closure = ledger["external_energy_j_m2"] + ledger["canopy_energy_j_m2"] + ledger["snow_energy_j_m2"] - (ledger["energy_end_j_m2"] - ledger["energy_start_j_m2"])
    reciprocal = ledger["canopy_snow_longwave_exchange_j_m2"] + ledger["snow_canopy_longwave_exchange_j_m2"]
    return {"id": case["id"], "status": "accepted", "active_participant_set": participants, "common_minimum_support_ns": wire_out(support), "exposure_receipt_id": exposure["receipt_id"], "shared_air_temperature_k": t_ca, "shared_air_specific_humidity": q_ca, "snow_sensible_into_surface_w_m2": h_snow, "snow_vapor_into_surface_kg_m2_s": v_snow, "canopy_sensible_into_surface_w_m2": h_canopy, "canopy_vapor_into_surface_kg_m2_s": v_canopy, "reference_sensible_into_node_w_m2": h_ref, "reference_vapor_into_node_kg_m2_s": v_ref, "sky_view_fraction": f_sky, "snow_longwave_net_w_m2": l_net, "snow_canopy_longwave_exchange_w_m2": l_exchange, "temperature_residual_w_m2": h_ref - h_snow - h_canopy, "vapor_residual_kg_m2_s": v_ref - v_snow - v_canopy, "snow_ice_end_kg_m2": snow_end, "liquid_end_kg_m2": liquid_end, "vapor_net_kg_m2": ledger["deposition_kg_m2"] - ledger["sublimation_kg_m2"], "energy_closure_j_m2": energy_closure, "longwave_reciprocal_closure_j_m2": reciprocal}


def participant_support(inputs, name):
    values = inputs.get(name, [])
    ids = [item["participant_id"] for item in values]
    receipt_ids = [item["support_receipt_id"] for item in values]
    if len(ids) != len(set(ids)) or ids != sorted(ids) or any(not receipt_id for receipt_id in receipt_ids) or len(receipt_ids) != len(set(receipt_ids)):
        raise ValueError("noncanonical participant set")
    return max((wire(item["minimum_support_ns"]) for item in values), default=0)


def terminal_errors(state, start, tick):
    elapsed_s = (tick - start) / 1_000_000_000.0
    snow = state["snow_start_kg_m2"] + state["snow_rate_kg_m2_s"] * elapsed_s
    liquid = state["liquid_start_kg_m2"] + state["liquid_rate_kg_m2_s"] * elapsed_s
    energy = state["energy_start_j_m2"] + state["energy_rate_j_m2_s"] * elapsed_s
    return abs(snow - state["snow_target_kg_m2"]), abs(liquid - state["liquid_target_kg_m2"]), abs(energy - state["energy_target_j_m2"])


def normalized(error, tolerance):
    if tolerance == 0.0:
        return 0.0 if error == 0.0 else None
    return error / tolerance


def event(case):
    inputs = case["inputs"]
    before = inputs.get("before_sha256", "")
    start, end, proposed = wire(inputs["parent_start_tick"]), wire(inputs["parent_end_tick"]), wire(inputs["proposed_event_tick"])
    pre, post = participant_support(inputs, "pre_active_participants"), participant_support(inputs, "post_active_participants")
    tol_t = wire(inputs["event_time_tolerance_ns"])
    tol_m, tol_l, tol_e = inputs["snow_mass_tolerance_kg_m2"], inputs["liquid_mass_tolerance_kg_m2"], inputs["energy_tolerance_j_m2"]
    candidates = []
    evaluations = []
    for raw_tick in inputs["candidate_ticks"]:
        tick = wire(raw_tick)
        if tick < start or tick > end:
            continue
        time_error = abs(tick - proposed)
        snow_error, liquid_error, energy_error = terminal_errors(inputs["terminal_state"], start, tick)
        pre_duration, post_duration = tick - start, end - tick
        support_ok = (pre_duration == 0 or pre_duration >= pre) and (post_duration == 0 or post_duration >= post)
        time_ok = time_error <= tol_t
        mass_score = normalized(snow_error, tol_m)
        liquid_score = normalized(liquid_error, tol_l)
        energy_score = normalized(energy_error, tol_e)
        tolerance_ok = mass_score is not None and liquid_score is not None and energy_score is not None
        accepted = support_ok and time_ok and snow_error <= tol_m and liquid_error <= tol_l and energy_error <= tol_e and tolerance_ok
        combined = (mass_score or 0.0) + (liquid_score or 0.0) + (energy_score or 0.0) if accepted else None
        evaluations.append({"tick": wire_out(tick), "support_admissible": support_ok, "event_time_error_ns": wire_out(time_error), "snow_mass_error_kg_m2": snow_error, "liquid_mass_error_kg_m2": liquid_error, "energy_error_j_m2": energy_error, "accepted": accepted})
        if accepted:
            candidates.append((time_error, combined, tick, snow_error, liquid_error, energy_error))
    if not candidates:
        result = reject(case, "ERR-CT-021")
        result.update({"pre_common_minimum_support": wire_out(pre), "post_common_minimum_support": wire_out(post), "candidate_evaluations": evaluations})
        return result
    selected = min(candidates, key=lambda row: (row[0], row[1], row[2]))
    return {"id": case["id"], "status": "accepted", "accepted_event_tick": wire_out(selected[2]), "proposed_event_tick": wire_out(proposed), "pre_active_participant_set": [item["participant_id"] for item in inputs.get("pre_active_participants", [])], "post_active_participant_set": [item["participant_id"] for item in inputs.get("post_active_participants", [])], "pre_support_receipts": inputs.get("pre_active_participants", []), "post_support_receipts": inputs.get("post_active_participants", []), "pre_common_minimum_support": wire_out(pre), "post_common_minimum_support": wire_out(post), "candidate_evaluations": evaluations, "event_time_error_ns": wire_out(selected[0]), "snow_mass_error_kg_m2": selected[3], "liquid_mass_error_kg_m2": selected[4], "energy_error_j_m2": selected[5], "combined_normalized_mass_energy_error": selected[1], "tie_break_rank": [wire_out(selected[0]), selected[1], wire_out(selected[2])], "positive_physical_successor": post > 0 and end > selected[2]}


def conservation(case):
    inputs = case["inputs"]
    snow_end = inputs["snow_ice_start_kg_m2"] + inputs["solid_precipitation_kg_m2"] - inputs["melt_kg_m2"] - inputs["sublimation_kg_m2"] + inputs["deposition_kg_m2"]
    liquid_end = inputs["liquid_start_kg_m2"] + inputs["rain_kg_m2"] + inputs["melt_kg_m2"] - inputs["refreeze_kg_m2"] - inputs["liquid_runoff_kg_m2"]
    energy_closure = inputs["external_energy_j_m2"] + inputs["refreeze_energy_j_m2"] - inputs["cold_content_change_j_m2"] - inputs["fusion_energy_j_m2"] - inputs["energy_end_j_m2"]
    diagnostic_alias = inputs["diagnostic_melt_alias_kg_m2"]
    if diagnostic_alias == inputs["melt_kg_m2"]:
        raise ValueError("diagnostic alias unexpectedly authoritative")
    return {"id": case["id"], "status": "accepted", "snow_ice_end_kg_m2": snow_end, "liquid_end_kg_m2": liquid_end, "vapor_net_kg_m2": inputs["deposition_kg_m2"] - inputs["sublimation_kg_m2"], "energy_closure_j_m2": energy_closure, "event_time_error_ns": wire_out(abs(wire(inputs["accepted_event_tick"]) - wire(inputs["proposed_event_tick"]))) }


def digest(value):
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def apply_transition(state, transition):
    owner_state = {
        key: state["owner_state"][key] + transition["owner_delta"].get(key, 0.0)
        for key in state["owner_state"]
    }
    receipt_state = dict(state["receipt_state"])
    receipt_state.update(transition["receipt_update"])
    return {
        "accepted_cursor": transition["accepted_cursor"],
        "owner_state": owner_state,
        "receipt_state": receipt_state,
    }


def restart(case):
    resumed = apply_transition(case["checkpoint"], case["resumed_transition"])
    uninterrupted = apply_transition(case["uninterrupted_start"], case["uninterrupted_transition"])
    return {
        "id": case["id"],
        "status": "accepted",
        "accepted_cursor": resumed["accepted_cursor"],
        "resumed_owner_digest": digest(resumed["owner_state"]),
        "uninterrupted_owner_digest": digest(uninterrupted["owner_state"]),
        "resumed_receipt_digest": digest(resumed["receipt_state"]),
        "uninterrupted_receipt_digest": digest(uninterrupted["receipt_state"]),
        "equivalent": resumed == uninterrupted,
    }


def rollback(case):
    return {
        "id": case["id"],
        "status": "accepted",
        "beginning_owner_digest": digest(case["beginning_owner_map"]),
        "staged_owner_digest": digest(case["staged_candidate_map"]),
        "restored_owner_digest": digest(case["restored_owner_map"]),
        "no_op": case["beginning_owner_map"] == case["restored_owner_map"],
    }


def main():
    vectors = json.loads(VECTORS.read_text())
    results = [carrier(case) for case in vectors["carrier_cases"]]
    results.extend(event(case) for case in vectors["event_cases"])
    results.extend(conservation(case) for case in vectors["conservation_cases"])
    restart_vectors = json.loads((ROOT / "restart-rollback-vectors.json").read_text())
    restart_results = [
        restart(case) if case["id"].startswith("restart_") else rollback(case)
        for case in restart_vectors["cases"]
    ]
    print(json.dumps({"schema": "OPENWEPP_SNOW_STAGE3_SHARED_CARRIER_REFERENCE_RESULTS_V2", "results": results, "restart_results": restart_results}, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
