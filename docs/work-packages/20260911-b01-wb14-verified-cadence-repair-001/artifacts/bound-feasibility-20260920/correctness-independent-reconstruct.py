#!/usr/bin/env python3
"""Independent retained-array arithmetic for the bounded correctness review.

This reads existing traces only.  It makes no evaluator, solver, domain, or
finite-difference call and performs no optimization or constitutive replay.
"""

import hashlib
import json
import math
from pathlib import Path


TRACE = Path(
    "/home/roger/openwepp-experiments/"
    "b01-wb14-grid40-run-evidence-20260919/treatment/trace.json"
)
EXPECTED_TRACE_SHA256 = (
    "93124ec24c18dea0bfba15d1b09a180948dc56e7b62a3e2b76bb94eda194a17e"
)
OUT = Path(__file__).with_name("correctness-independent-reconstruction.json")


def record(records, kind, iteration=None):
    matches = [
        item["value"]
        for item in records
        if item["kind"] == kind
        and (iteration is None or item["value"].get("iteration") == iteration)
    ]
    assert len(matches) == 1, (kind, iteration, len(matches))
    return matches[0]


def explicit_bounds():
    result = [(-math.inf, math.inf)] * 29
    for offset in (0, 10):
        result[offset + 4] = (0.0, 1.0)
        result[offset + 5] = (0.0, 1.0)
        for local in (6, 7, 8):
            result[offset + local] = (273.15, 350.0)
        result[offset + 9] = (200.0, 350.0)
    result[20] = (200.0, 350.0)
    result[21] = (0.0, 0.1)
    result[22] = (200.0, 350.0)
    for index in range(23, 29):
        result[index] = (200.0, 350.0)
    return result


def main():
    trace_bytes = TRACE.read_bytes()
    trace_sha256 = hashlib.sha256(trace_bytes).hexdigest()
    assert trace_sha256 == EXPECTED_TRACE_SHA256
    trace = json.loads(trace_bytes)
    records = trace["records"]
    solver_input = record(records, "solver_input")
    inputs = solver_input["inputs"]
    snow = inputs["stage3_lower_boundary"]

    # Source constants and polynomial coefficients copied for inequality
    # bounds only.  No temperature-dependent model value is evaluated.
    sigma = 5.670_374_419e-8
    tref = 273.15
    pressure = inputs["pressure_pa"]
    qair = inputs["air_specific_humidity_kg_kg"]
    es0_pa = 100.0 * 6.112_134_76
    qsat_at_tref = 0.622 * es0_pa / (pressure - 0.378 * es0_pa)
    ldn_ceiling_at_tref = sigma * tref**4
    assert inputs["atmospheric_downward_longwave_w_m2"] < ldn_ceiling_at_tref
    assert snow["snow_temperature_k"] < tref
    assert qair < qsat_at_tref

    # On t=T-273.15 in [0,76.85], the sole negative derivative term is
    # dominated by its adjacent positive t^5 term.
    tmax = 350.0 - tref
    derivative_group_bracket = (
        6.0 * 8.923_447_72e-11 - 7.0 * 3.732_084_10e-13 * tmax
    )
    assert derivative_group_bracket > 0.0
    coefficients = [
        6.112_134_76,
        4.440_078_56e-1,
        1.430_642_34e-2,
        2.644_614_37e-4,
        3.059_035_58e-6,
        1.962_372_41e-8,
        8.923_447_72e-11,
        -3.732_084_10e-13,
        2.093_399_97e-16,
    ]
    es_positive_term_upper_pa = 100.0 * math.fsum(
        max(coefficient, 0.0) * 77.0**power
        for power, coefficient in enumerate(coefficients)
    )
    denominator_limit_pa = pressure / 0.378
    assert es_positive_term_upper_pa < denominator_limit_pa

    occupancy_checks = []
    for occupancy in inputs["occupancies"]:
        assert inputs["top_rain_kg_m2_tile"] == 0.0
        plant_area = occupancy["lai"] + occupancy["sai"]
        capacity = occupancy["liquid_capacity_kg_m2_plant"] * plant_area
        preliminary_store = occupancy["beginning_canopy_liquid_kg_m2_tile"]
        assert 0.0 < preliminary_store < capacity
        wet_fraction = (preliminary_store / capacity) ** (2.0 / 3.0)
        store_rate = preliminary_store / inputs["interval_s"]
        water_tolerance = 1.0e-12 + 1.0e-9 * store_rate
        assert store_rate > water_tolerance
        assert occupancy["sun"]["leaf_area_m2_m2_tile"] == 0.0
        assert occupancy["shade"]["leaf_area_m2_m2_tile"] > 0.0
        assert occupancy["stem_area_m2_m2_tile"] > 0.0
        assert 0.0 < wet_fraction < 1.0
        occupancy_checks.append(
            {
                "occupancy_id": occupancy["occupancy_id"],
                "capacity_kg_m2_tile": capacity,
                "preliminary_store_kg_m2_tile": preliminary_store,
                "wet_fraction": wet_fraction,
                "store_rate_kg_m2_s": store_rate,
                "water_tolerance_kg_m2_s": water_tolerance,
            }
        )

    units = ([1000.0] * 4 + [1.0] * 6) * 2 + [1.0, 0.001, 1.0] + [1.0] * 6
    bases = []
    for iteration in (0, 6, 14):
        base = record(records, "accepted_base", iteration)
        linear = record(records, "linear_system", iteration)
        x = base["coordinates"]
        residual = base["normalized_residuals"]
        normalizers = base["applied_normalizers"]
        jacobian = linear["jacobian"]
        assert len(x) == len(residual) == len(normalizers) == len(jacobian) == 29
        assert all(len(row) == 29 for row in jacobian)
        assert all(
            raw / scale == normalized
            for raw, scale, normalized in zip(
                base["raw_residuals"], normalizers, residual
            )
        )

        # Predeclared direction: change only represented-snow ground identity
        # coordinate 22 by -1 K.  Since its coordinate scale is 1 K, J_y and
        # J_x have the same column here.
        assert units[22] == 1.0
        direction = [0.0] * 29
        direction[22] = -1.0
        trial = [value + change for value, change in zip(x, direction)]
        feasible = all(
            lower <= value <= upper
            for value, (lower, upper) in zip(trial, explicit_bounds())
        )
        assert feasible
        assert all(jacobian[row][22] == 0.0 for row in range(29) if row != 22)
        linear_prediction = [
            value + jacobian[row][22] * direction[22]
            for row, value in enumerate(residual)
        ]
        identity_prediction = residual.copy()
        identity_prediction[22] = (
            trial[22] - snow["snow_temperature_k"]
        ) / normalizers[22]
        assert all(
            identity_prediction[row] == residual[row]
            for row in range(29)
            if row != 22
        )
        assert max(map(abs, linear_prediction)) < max(map(abs, residual))
        assert max(map(abs, identity_prediction)) < max(map(abs, residual))
        bases.append(
            {
                "iteration": iteration,
                "direction_coordinate": 22,
                "direction_K": -1.0,
                "scaled_radius": 1.0,
                "explicit_bounds_pass": feasible,
                "base_infinity_merit": max(map(abs, residual)),
                "linearized_infinity_merit": max(map(abs, linear_prediction)),
                "source_identity_infinity_merit": max(map(abs, identity_prediction)),
                "matrix_vs_source_prediction_max_difference": max(
                    abs(a - b)
                    for a, b in zip(linear_prediction, identity_prediction)
                ),
                "all_other_rows_unchanged": all(
                    identity_prediction[row] == residual[row]
                    for row in range(29)
                    if row != 22
                ),
                "shade_margin_K": x[7] - tref,
                "captured_newton_shade_direction_K": linear["direction"][7],
            }
        )

    result = {
        "evidence_class": (
            "Ran: finite retained-array/hash arithmetic; Static: source equation "
            "inequalities; no evaluator/domain/solver/FD/optimizer call"
        ),
        "trace_sha256": trace_sha256,
        "qsat_monotonic_derivative_group_lower": derivative_group_bracket,
        "es_positive_term_upper_pa": es_positive_term_upper_pa,
        "es_denominator_limit_pa": denominator_limit_pa,
        "qsat_at_273_15_kg_kg": qsat_at_tref,
        "air_specific_humidity_kg_kg": qair,
        "sigma_tref4_w_m2": ldn_ceiling_at_tref,
        "atmospheric_downward_longwave_w_m2": inputs[
            "atmospheric_downward_longwave_w_m2"
        ],
        "snow_temperature_K": snow["snow_temperature_k"],
        "snow_sensible_to_canopy_w_m2": snow["sensible_to_canopy_air_w_m2"],
        "snow_vapor_to_canopy_kg_m2_s": snow["vapor_to_canopy_air_kg_m2_s"],
        "occupancies": occupancy_checks,
        "bases": bases,
        "scope_limit": (
            "Exact source-identity prediction and equation/domain contradiction; "
            "no tolerance-certified infeasibility and no nonlinear trial result"
        ),
    }
    OUT.write_text(json.dumps(result, indent=2, allow_nan=False) + "\n")


if __name__ == "__main__":
    main()
