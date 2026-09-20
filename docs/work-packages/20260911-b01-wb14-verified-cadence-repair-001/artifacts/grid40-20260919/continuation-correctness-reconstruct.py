#!/usr/bin/env python3
"""Independent offline Grid40 treatment reconstruction; never runs an evaluator."""

import hashlib
import json
import math
from pathlib import Path
import struct
import sys


TRACE = Path("/home/roger/openwepp-experiments/b01-wb14-grid40-run-evidence-20260919/treatment/trace.json")
SIDECAR = Path("/home/roger/openwepp-experiments/b01-wb14-grid40-run-evidence-20260919/treatment/sidecar.json")
RECEIPT = Path("/home/roger/openwepp-experiments/b01-wb14-grid40-run-evidence-20260919/treatment/receipt.json")
ROW_MAP = Path(__file__).with_name("original-case-row-dimensions.json")
LIQUID_MINIMUM_K = 273.15
MOLAR_GAS_CONSTANT = 8.31446261815324
DRY_AIR_GAS_CONSTANT = 287.05


def require(condition, detail):
    if not condition:
        raise RuntimeError(detail)


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def bits(value):
    return struct.pack(">d", value).hex()


def same_bits(left, right):
    return bits(left) == bits(right)


def vulnerability(potential, p50, exponent):
    return 2.0 ** -((potential / p50) ** exponent)


def violations(trial, occupancy_count, ground_liquid_phase):
    result = []
    if len(trial) < 10 * occupancy_count + 4:
        return [{"reason": "length"}]
    for index, value in enumerate(trial):
        if not math.isfinite(value):
            result.append({"index": index, "reason": "nonfinite"})
    for occupancy in range(occupancy_count):
        offset = 10 * occupancy
        for index in (offset + 4, offset + 5):
            if not 0.0 <= trial[index] <= 1.0:
                result.append({"index": index, "lower": 0.0, "upper": 1.0, "value": trial[index]})
        for index in range(offset + 6, offset + 9):
            if not LIQUID_MINIMUM_K <= trial[index] <= 350.0:
                result.append({"index": index, "lower": LIQUID_MINIMUM_K, "upper": 350.0, "value": trial[index]})
        index = offset + 9
        if not 200.0 <= trial[index] <= 350.0:
            result.append({"index": index, "lower": 200.0, "upper": 350.0, "value": trial[index]})
    common = 10 * occupancy_count
    bounds = [(common, 200.0, 350.0), (common + 1, 0.0, 0.1),
              (common + 2, LIQUID_MINIMUM_K if ground_liquid_phase else 200.0, 350.0)]
    bounds.extend((index, 200.0, 350.0) for index in range(common + 3, len(trial)))
    for index, lower, upper in bounds:
        if not lower <= trial[index] <= upper:
            result.append({"index": index, "lower": lower, "upper": upper, "value": trial[index]})
    return result


def constrained_distances(trial, occupancy_count, ground_liquid_phase):
    bounds = []
    for occupancy in range(occupancy_count):
        offset = 10 * occupancy
        bounds.extend((index, 0.0, 1.0, "beta") for index in (offset + 4, offset + 5))
        bounds.extend((index, LIQUID_MINIMUM_K, 350.0, "liquid-vapor component temperature K")
                      for index in range(offset + 6, offset + 9))
        bounds.append((offset + 9, 200.0, 350.0, "dry-stem temperature K"))
    common = 10 * occupancy_count
    bounds.extend([(common, 200.0, 350.0, "canopy temperature K"),
                   (common + 1, 0.0, 0.1, "canopy humidity kg/kg"),
                   (common + 2, LIQUID_MINIMUM_K if ground_liquid_phase else 200.0,
                    350.0, "ground temperature K")])
    bounds.extend((index, 200.0, 350.0, "soil temperature K")
                  for index in range(common + 3, len(trial)))
    rows = [{"index": index, "kind": kind, "value": trial[index], "lower": lower,
             "upper": upper, "distance_to_lower": trial[index] - lower,
             "distance_to_upper": upper - trial[index]}
            for index, lower, upper, kind in bounds]
    return rows


def reconstruct_hydraulics(inputs, base):
    coordinates = base["coordinates"]
    evaluation = base["evaluation"]
    output = []
    for occupancy_index, (occupancy, observed) in enumerate(
            zip(inputs["occupancies"], evaluation["occupancies"])):
        block = coordinates[10 * occupancy_index:10 * (occupancy_index + 1)]
        psi_sun, psi_shade, psi_stem, psi_root = block[:4]
        shade_temperature = block[7]
        gs = (occupancy["g0_umol_m2_s"] * 1.0e-6 * MOLAR_GAS_CONSTANT
              * shade_temperature / inputs["pressure_pa"])
        rho = inputs["pressure_pa"] / (DRY_AIR_GAS_CONSTANT * coordinates[20])
        shade_area = observed["component_areas_m2_m2_tile"][1]
        shade_surface_q = observed["component_surface_specific_humidity_kg_kg"][1]
        shade_e = (rho * (shade_surface_q - coordinates[21])
                   / (1.0 / occupancy["gb_leaf_m_s"] + 1.0 / gs) * shade_area)
        q1_sun = (occupancy["k1_sun_max_s1"] * occupancy["sun"]["leaf_area_m2_m2_tile"]
                  * vulnerability(psi_stem, occupancy["p50_xylem_mm"], occupancy["vulnerability_exponent"])
                  * (psi_stem - psi_sun))
        q1_shade = (occupancy["k1_shade_max_s1"] * occupancy["shade"]["leaf_area_m2_m2_tile"]
                    * vulnerability(psi_stem, occupancy["p50_xylem_mm"], occupancy["vulnerability_exponent"])
                    * (psi_stem - psi_shade))
        q2 = (occupancy["k2_max"] / occupancy["height_m"]
              * vulnerability(psi_root, occupancy["p50_xylem_mm"], occupancy["vulnerability_exponent"])
              * occupancy["sai"] * (psi_root - psi_stem - 1000.0 * occupancy["height_m"]))
        root_fluxes = []
        for layer in occupancy["root_layers"]:
            if layer["accessible"] and not layer["frozen"] and layer["root_fraction"] > 0.0:
                kr = (occupancy["k3_max_m_s"] / layer["z3_m"]
                      * vulnerability(layer["soil_potential_mm"], occupancy["p50_root_mm"],
                                      occupancy["vulnerability_exponent"]))
                ks = layer["ksoil_m2_s"] / layer["dxroot_m"]
                series = kr * ks / (kr + ks)
                root_area = ((occupancy["lai"] + occupancy["sai"]) * layer["root_fraction"]
                             * occupancy["root_to_leaf_area"])
                flux = series * root_area * (
                    layer["soil_potential_mm"] - psi_root + layer["gravity_head_mm"])
            else:
                kr = ks = series = root_area = 0.0
                flux = 0.0
            root_fluxes.append({"layer_id": layer["layer_id"], "kr_m_s": kr,
                                "ks_m_s": ks, "series_m_s": series,
                                "root_area_m2_m2_tile": root_area,
                                "law_kg_m2_tile_s": flux,
                                "request_kg_m2_stand_ground": max(flux, 0.0)
                                * inputs["tile_fraction"] * inputs["interval_s"]})
        root_sum = math.fsum(row["law_kg_m2_tile_s"] for row in root_fluxes)
        residuals = [psi_sun - psi_stem, shade_e - q1_shade,
                     block[4] - 1.0, block[5] - 1.0,
                     q1_sun + q1_shade - q2, q2 - root_sum]
        observed_residuals = base["raw_residuals"][10 * occupancy_index:10 * occupancy_index + 6]
        require(all(same_bits(left, right) for left, right in zip(residuals, observed_residuals)),
                "hydraulic primitive residual mismatch")
        require(same_bits(shade_e, observed["signed_vapor_to_canopy_air_kg_m2_s"][1]),
                "shade vapor primitive mismatch")
        require(all(same_bits(row["law_kg_m2_tile_s"], captured["law_kg_m2_tile_s"])
                    for row, captured in zip(root_fluxes, observed["source_water"])),
                "root-law primitive mismatch")
        require(all(row["request_kg_m2_stand_ground"] == 0.0 for row in root_fluxes),
                "unexpected positive root request")
        water_scale = max(observed["emax_kg_m2_s"] + [abs(q1_sun), abs(q1_shade), abs(q2), abs(root_sum)])
        water_normalizer = 1.0e-12 + 1.0e-9 * max(1.0e-12, water_scale)
        require(same_bits(water_normalizer, base["applied_normalizers"][10 * occupancy_index + 1]),
                "water normalizer mismatch")
        output.append({
            "occupancy_id": occupancy["occupancy_id"],
            "branches": {"gas": observed["gas_branches"], "wet": observed["wet_branch"],
                         "root": [row["branch"] for row in observed["source_water"]]},
            "rho_kg_m3": rho, "shade_gs_m_s": gs, "shade_area_m2_m2_tile": shade_area,
            "shade_surface_q_kg_kg": shade_surface_q, "canopy_q_kg_kg": coordinates[21],
            "shade_vapor_kg_m2_tile_s": shade_e, "q1_sun_kg_m2_tile_s": q1_sun,
            "q1_shade_kg_m2_tile_s": q1_shade, "q2_kg_m2_tile_s": q2,
            "root_source_sum_kg_m2_tile_s": root_sum, "root_fluxes": root_fluxes,
            "reconstructed_rows_0_to_5": residuals,
            "captured_rows_0_to_5": observed_residuals,
            "all_six_rows_bit_exact": True,
            "water_scale_kg_m2_tile_s": water_scale,
            "water_normalizer_kg_m2_tile_s": water_normalizer,
            "captured_water_normalizer_kg_m2_tile_s": base["applied_normalizers"][10 * occupancy_index + 1],
            "all_root_requests_and_finalized_uses_zero": all(
                captured["request_kg_m2_stand_ground"] == 0.0
                and captured["finalized_use_kg_m2_stand_ground"] == 0.0
                for captured in observed["source_water"]),
        })
    return output


def main():
    require(len(sys.argv) == 2, "usage: continuation-correctness-reconstruct.py OUTPUT")
    output_path = Path(sys.argv[1])
    trace = json.loads(TRACE.read_bytes())
    sidecar = json.loads(SIDECAR.read_bytes())
    receipt = json.loads(RECEIPT.read_bytes())
    row_map = json.loads(ROW_MAP.read_bytes())
    records = trace["records"]
    solver_input = next(row["value"] for row in records if row["kind"] == "solver_input")
    bases = [row["value"] for row in records if row["kind"] == "accepted_base"]
    systems = [row["value"] for row in records if row["kind"] == "linear_system"]
    attempts = [row["value"] for row in records if row["kind"] == "line_search_attempt"]
    terminal = sidecar["terminal"]
    require(receipt["process_exit"] == 0 and receipt["stop_reason"] is None
            and receipt["integrity_unchanged"], "invalid treatment receipt")
    require(terminal["kind"] == "Rejected" and terminal["failure_kind"] == "BacktrackingLimit",
            "unexpected treatment terminal")
    require(len(bases) == len(systems) == 15 and bases[-1]["iteration"] == systems[-1]["iteration"] == 14,
            "unexpected iteration evidence")
    base = bases[-1]
    system = systems[-1]
    coordinates = base["coordinates"]
    direction = system["direction"]
    occupancy_count = len(solver_input["inputs"]["occupancies"])
    ground_liquid_phase = solver_input["inputs"]["stage3_lower_boundary"] is None
    require(not ground_liquid_phase, "expected represented-snow ground domain")
    require(len(row_map["rows"]) == len(base["raw_residuals"]) == 29, "row map mismatch")
    require(all(same_bits(left, right) for left, right in
                zip(terminal["current_base_coordinates"], base["coordinates"])),
            "terminal/base coordinate mismatch")
    require(all(same_bits(left, right) for left, right in
                zip(terminal["normalized_residuals"], base["normalized_residuals"])),
            "terminal/base residual mismatch")

    branch_frames = []
    for frame in bases:
        branch_frames.append({"iteration": frame["iteration"],
                              "gas": [row["gas_branches"] for row in frame["evaluation"]["occupancies"]],
                              "wet": [row["wet_branch"] for row in frame["evaluation"]["occupancies"]],
                              "sun_areas": [row["component_areas_m2_m2_tile"][0]
                                            for row in frame["evaluation"]["occupancies"]],
                              "shade_areas": [row["component_areas_m2_m2_tile"][1]
                                              for row in frame["evaluation"]["occupancies"]]})
    require(all(frame["gas"] == [["Inactive", "ExactZeroPar"]] * 2 for frame in branch_frames),
            "gas branch changed")
    require(all(frame["wet"] == ["ConstitutiveLaw"] * 2 for frame in branch_frames),
            "wet branch changed")
    require(all(frame["sun_areas"] == [0.0, 0.0]
                and all(value > 0.0 for value in frame["shade_areas"])
                for frame in branch_frames), "occupancy activity changed")

    normalization_total = normalization_exact = 0
    for frame in bases:
        for raw, normalizer, normalized in zip(frame["raw_residuals"],
                                                frame["applied_normalizers"],
                                                frame["normalized_residuals"]):
            normalization_total += 1
            normalization_exact += same_bits(raw / normalizer, normalized)
    require(normalization_total == normalization_exact == 435, "normalization mismatch")

    final_attempts = [row for row in attempts if row["iteration"] == 14]
    require([row["exponent"] for row in final_attempts] == list(range(41)),
            "final exponent sequence")
    projection_exact = 0
    for row in final_attempts:
        projected = [value + row["factor"] * change for value, change in zip(coordinates, direction)]
        projection_exact += all(same_bits(left, right) for left, right in zip(projected, row["coordinates"]))
        require(violations(row["coordinates"], occupancy_count, ground_liquid_phase),
                "unexpected domain-valid final attempt")
    require(projection_exact == 41, "captured final trial projection mismatch")
    b40 = final_attempts[-1]
    b40_violations = violations(b40["coordinates"], occupancy_count, ground_liquid_phase)
    require([row["index"] for row in b40_violations] == [7], "unexpected b40 limiter")
    distance = coordinates[7] - LIQUID_MINIMUM_K
    maximum_factor = distance / -direction[7]
    first_lawful_exponent = next(exponent for exponent in range(100)
                                 if 2.0 ** -exponent <= maximum_factor)
    projected_b41 = [value + 2.0 ** -first_lawful_exponent * change
                     for value, change in zip(coordinates, direction)]
    require(first_lawful_exponent == 41 and not violations(
        projected_b41, occupancy_count, ground_liquid_phase), "unexpected projected lawful factor")

    distance_rows = constrained_distances(coordinates, occupancy_count, ground_liquid_phase)
    nearest = min(distance_rows, key=lambda row: min(row["distance_to_lower"], row["distance_to_upper"]))
    require(nearest["index"] == 7, "unexpected nearest bound")
    dominant_index = max(range(29), key=lambda index: abs(base["normalized_residuals"][index]))
    snow_temperature = solver_input["inputs"]["stage3_lower_boundary"]["snow_temperature_k"]
    ground_raw = coordinates[22] - snow_temperature
    require(dominant_index == 22 and same_bits(ground_raw, base["raw_residuals"][22]),
            "represented-snow identity mismatch")
    require(same_bits(ground_raw / base["applied_normalizers"][22],
                      base["normalized_residuals"][22]), "ground normalization mismatch")

    accepted = [row for row in attempts if row.get("accepted")]
    last_accepted = accepted[-1]
    terminal_steps = terminal["step_norms"]
    require(last_accepted["iteration"] == 13 and last_accepted["exponent"] == 36,
            "unexpected prior accepted step")
    require(all(same_bits(terminal_steps[key], last_accepted["step_norms"][key])
                for key in terminal_steps), "terminal step provenance mismatch")
    require([row["exponent"] for row in accepted] == sidecar["counts"]["accepted_strict_exponents"],
            "accepted exponent evidence mismatch")
    require(sum(row["exponent"] for row in accepted) + 40 == terminal["backtracking_count"],
            "backtracking count reconstruction mismatch")

    units = [1000.0] * 4 + [1.0] * 6 + [1000.0] * 4 + [1.0] * 6 \
        + [1.0, 0.001, 1.0] + [1.0] * 6
    scaled_solution = [value / unit for value, unit in zip(direction, units)]
    linear_residuals = [math.fsum(coefficient * value
                                  for coefficient, value in zip(row, scaled_solution)) - rhs
                        for row, rhs in zip(system["jacobian"], system["rhs"])]
    residual_norm = max(map(abs, linear_residuals))
    rhs_norm = max(map(abs, system["rhs"]))
    matrix_norm = max(math.fsum(map(abs, row)) for row in system["jacobian"])
    solution_norm = max(map(abs, scaled_solution))

    result = {
        "evidence_class": "Ran: independent offline arithmetic over retained treatment trace/sidecar only; no evaluator",
        "inputs": {"trace": str(TRACE), "trace_sha256": sha256(TRACE),
                   "sidecar": str(SIDECAR), "sidecar_sha256": sha256(SIDECAR),
                   "receipt": str(RECEIPT), "receipt_sha256": sha256(RECEIPT),
                   "row_map": str(ROW_MAP), "row_map_sha256": sha256(ROW_MAP)},
        "identity": {"source_sha256": receipt["source_sha256"],
                     "binary_sha256": receipt["binary_sha256"],
                     "input_sha256": receipt["input_sha256"],
                     "freeze_sha256": receipt["freeze_sha256"],
                     "arm": receipt["arm"], "elapsed_seconds": receipt["elapsed_seconds"]},
        "terminal": {"kind": terminal["kind"], "failure_kind": terminal["failure_kind"],
                     "iterations": terminal["iterations"],
                     "backtracking_count": terminal["backtracking_count"],
                     "accepted_strict_exponents": sidecar["counts"]["accepted_strict_exponents"]},
        "branch_audit": {"accepted_base_frames": len(branch_frames),
                         "iterations": [row["iteration"] for row in branch_frames],
                         "all_gas_branches": [["Inactive", "ExactZeroPar"]] * 2,
                         "all_wet_branches": ["ConstitutiveLaw"] * 2,
                         "sun_areas_zero_all_frames": True,
                         "shade_areas_positive_all_frames": True,
                         "represented_snow_all_frames": True},
        "normalization": {"row_frames_checked": normalization_total,
                          "bit_exact_raw_div_normalizer": normalization_exact,
                          "normalizers_are_per_frame": True},
        "first_obstruction": {
            "iteration": 14, "coordinate_index": 7,
            "coordinate_role": "upper occupancy active shade-leaf temperature K",
            "base_value_k": coordinates[7], "lower_bound_k": LIQUID_MINIMUM_K,
            "base_distance_to_lower_k": distance, "newton_direction_k": direction[7],
            "maximum_lawful_factor": maximum_factor,
            "attempted_exponents": [row["exponent"] for row in final_attempts],
            "all_41_attempts_domain_invalid": True,
            "all_41_trial_coordinates_match_x_plus_factor_delta_bitwise": True,
            "b40_factor": b40["factor"], "b40_coordinate_k": b40["coordinates"][7],
            "b40_distance_to_lower_k": b40["coordinates"][7] - LIQUID_MINIMUM_K,
            "b40_violating_indices": [row["index"] for row in b40_violations],
            "first_arithmetically_lawful_exponent": first_lawful_exponent,
            "projected_b41_coordinate_k": projected_b41[7],
            "projected_b41_distance_to_lower_k": projected_b41[7] - LIQUID_MINIMUM_K,
            "projected_b41_offline_bound_check": "PASS; no canonical Rust domain-admission call",
            "projected_b41_evaluator_status": "NOT RUN; outside fixed policy; no residual or acceptance inference",
        },
        "endpoint_domain_distances": distance_rows,
        "dominant_residual": {"row_index": dominant_index,
                              "branch_aware_name": row_map["rows"][dominant_index]["name"],
                              "raw_unit": row_map["rows"][dominant_index]["raw_unit"],
                              "ground_temperature_k": coordinates[22],
                              "snow_temperature_k": snow_temperature,
                              "reconstructed_raw_k": ground_raw,
                              "applied_normalizer_k": base["applied_normalizers"][22],
                              "reconstructed_normalized": ground_raw / base["applied_normalizers"][22],
                              "captured_normalized": base["normalized_residuals"][22],
                              "bit_exact": True,
                              "complete_residuals_all_pass": all(abs(value) <= 1.0
                                                                  for value in base["normalized_residuals"])},
        "step_provenance": {"terminal_step_origin_text": terminal["step_origin"],
                            "actual_source_iteration": last_accepted["iteration"],
                            "actual_source_exponent": last_accepted["exponent"],
                            "terminal_equals_prior_accepted_step_bitwise": True,
                            "terminal_steps": terminal_steps,
                            "iteration_14_has_domain_valid_trial": False,
                            "iteration_14_has_rejected_step_norms": False},
        "linear_system": {"iteration": 14, "v10_scaled_potential": system["v10_scaled_potential"],
                          "matrix_infinity_norm_reconstructed": matrix_norm,
                          "matrix_infinity_norm_captured": system["matrix_infinity_norm"],
                          "max_abs_residual": residual_norm,
                          "relative_rhs_residual": residual_norm / rhs_norm,
                          "normwise_backward_error": residual_norm
                          / (matrix_norm * solution_norm + rhs_norm)},
        "hydraulic_primitive_reconstruction": reconstruct_hydraulics(solver_input["inputs"], base),
        "scientific_interpretation": {
            "fixed_policy_result": "valid typed refusal if event/cost integrity independently passes",
            "proximate_cause": "strict b0..40 grid ends before the first domain-valid projected factor at iteration14",
            "root_existence": "UNPROVED",
            "b41_behavior": "UNKNOWN; arithmetic domain projection only; evaluator forbidden",
            "potential_requests_are_final_fluxes": False,
            "global_conservation_claim": False,
        },
    }
    output_path.write_text(json.dumps(result, indent=2, allow_nan=False) + "\n")
    print(json.dumps({"output": str(output_path), "sha256": sha256(output_path),
                      "terminal": result["terminal"],
                      "first_obstruction": result["first_obstruction"]}, indent=2))


if __name__ == "__main__":
    main()
