#!/usr/bin/env python3
"""Generate independent OPENWEPP_C3_WOODY_V6 portability vectors."""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import math
from pathlib import Path

RTOL = 3.0e-7
EXACT_FIELDS = (
    "model_definition_sha256", "configuration_sha256", "transaction_id",
    "occupancy_id", "pass", "solve", "field", "typed_failure", "candidate",
    "unit", "basis", "present", "iterations", "backtracking_count",
    "residual_cardinality", "active_bounds", "active_water_caps", "branches",
    "rollback_sha256_before", "rollback_sha256_after", "accepted_value",
)


def finite_value(encoded: dict[str, object]) -> float | None:
    if encoded["class"] != "finite":
        return None
    return float(encoded["value"])


def portable_equal(reference: dict[str, object], actual: dict[str, object]) -> bool:
    """Evaluate exact eligibility, then the V6 rtol-only scalar rule."""
    if any(reference[field] != actual[field] for field in EXACT_FIELDS):
        return False
    if reference["model_definition_sha256"] != "BOUND_BY_V6_DEFINITION_NOT_ORACLE":
        return False
    if (reference["pass"], reference["solve"], reference["field"], reference["typed_failure"]) != (
        "capped", "hydraulic_system", "step_norm", "backtracking_limit"
    ):
        return False
    if not reference["present"] or reference["candidate"] is not None:
        return False
    if reference["accepted_value"] or not reference["typed_failure"]:
        return False
    if reference["rollback_sha256_before"] != reference["rollback_sha256_after"]:
        return False
    if actual["rollback_sha256_before"] != actual["rollback_sha256_after"]:
        return False
    a = finite_value(reference["scalar"])
    b = finite_value(actual["scalar"])
    if a is None or b is None:
        return False
    if a < 0.0 or b < 0.0:
        return False
    if (a == 0.0) != (b == 0.0):
        return False
    if a != 0.0 and math.copysign(1.0, a) != math.copysign(1.0, b):
        return False
    return abs(a - b) <= RTOL * max(abs(a), abs(b))


def record(value: float) -> dict[str, object]:
    return {
        "accepted_value": False,
        "active_bounds": [],
        "active_water_caps": ["soil-1", "soil-2"],
        "backtracking_count": 94,
        "basis": "unscaled_six_unknown_newton_correction",
        "branches": ["authorization_active_or_tie", "authorization_active_or_tie"],
        "candidate": None,
        "configuration_sha256": "v6-fixture-configuration",
        "field": "step_norm",
        "iterations": 7,
        "model_definition_sha256": "BOUND_BY_V6_DEFINITION_NOT_ORACLE",
        "occupancy_id": {"stratum_id": "canopy", "tile_id": "tile-a"},
        "pass": "capped",
        "present": True,
        "residual_cardinality": 6,
        "rollback_sha256_after": "rollback-snapshot",
        "rollback_sha256_before": "rollback-snapshot",
        "scalar": {"class": "finite", "value": value},
        "solve": "hydraulic_system",
        "transaction_id": 53,
        "typed_failure": "backtracking_limit",
        "unit": "mixed_native_unknown_units",
    }


def largest_passing_above(reference_record: dict[str, object]) -> float:
    a = finite_value(reference_record["scalar"])
    assert a is not None and a > 0.0
    low = a
    high = a * (1.0 + 2.0 * RTOL)
    while True:
        trial = copy.deepcopy(reference_record)
        trial["scalar"]["value"] = high
        if not portable_equal(reference_record, trial):
            break
        high *= 1.0 + RTOL
    for _ in range(1075):
        middle = (low + high) / 2.0
        if middle == low or middle == high:
            break
        trial = copy.deepcopy(reference_record)
        trial["scalar"]["value"] = middle
        if portable_equal(reference_record, trial):
            low = middle
        else:
            high = middle
    while True:
        adjacent = math.nextafter(low, math.inf)
        trial = copy.deepcopy(reference_record)
        trial["scalar"]["value"] = adjacent
        if not portable_equal(reference_record, trial):
            return low
        low = adjacent


def comparison_case(name: str, reference: dict[str, object], actual: dict[str, object], expected: bool) -> dict[str, object]:
    return {"actual": actual, "case": name, "expected_equal": expected,
            "observed_equal": portable_equal(reference, actual), "reference": reference}


def sha256_value(value: object) -> str:
    return hashlib.sha256(canonical_bytes(value)).hexdigest()


def identity_transition() -> dict[str, object]:
    scientific_payload = {
        "configuration": {"tile_fraction": 0.38, "root_layers": ["soil-1", "soil-2"]},
        "diagnostic": {"backtracking_count": 94, "step_norm": 3925.8532969524972},
        "state": {"root_node_potential_mm": -812.5, "sun_ci_pa": 28.25},
    }
    payload_bytes = canonical_bytes(scientific_payload)
    payload_sha = hashlib.sha256(payload_bytes).hexdigest()
    v5_config = sha256_value({"model": "OPENWEPP_C3_WOODY_V5", "payload_sha256": payload_sha})
    v6_config = sha256_value({"model": "OPENWEPP_C3_WOODY_V6", "payload_sha256": payload_sha})
    v5_state = sha256_value({"configuration_sha256": v5_config, "model": "OPENWEPP_C3_WOODY_V5", "payload_sha256": payload_sha})
    v6_state = sha256_value({"configuration_sha256": v6_config, "model": "OPENWEPP_C3_WOODY_V6", "payload_sha256": payload_sha})
    v5_diagnostic = sha256_value({"model": "OPENWEPP_C3_WOODY_V5", "state_sha256": v5_state, "payload_sha256": payload_sha})
    v6_diagnostic = sha256_value({"model": "OPENWEPP_C3_WOODY_V6", "state_sha256": v6_state, "payload_sha256": payload_sha})
    if len({v5_config, v6_config, v5_state, v6_state, v5_diagnostic, v6_diagnostic}) != 6:
        raise RuntimeError("identity transition did not derive distinct identities")
    return {
        "non_identity_scientific_payload": scientific_payload,
        "non_identity_payload_bytes_sha256_after": payload_sha,
        "non_identity_payload_bytes_sha256_before": payload_sha,
        "source": {"configuration_sha256": v5_config, "diagnostic_sha256": v5_diagnostic,
                   "model_version": "OPENWEPP_C3_WOODY_V5", "state_sha256": v5_state},
        "target": {"configuration_sha256": v6_config, "diagnostic_sha256": v6_diagnostic,
                   "model_version": "OPENWEPP_C3_WOODY_V6", "state_sha256": v6_state},
    }


def poison(name: str, field: str, value: object) -> dict[str, object]:
    reference = record(1.0)
    actual = copy.deepcopy(reference)
    actual[field] = value
    return comparison_case(name, reference, actual, False)


def generate() -> dict[str, object]:
    one = record(1.0)
    boundary = largest_passing_above(one)
    numeric: list[dict[str, object]] = []
    for name, a, b, expected in (
        ("observed_cpython_rust_step_norm", 3925.8532969524972, 3925.8544224384018, True),
        ("exact_largest_representable_boundary", 1.0, boundary, True),
        ("one_representable_value_inside", 1.0, math.nextafter(boundary, 1.0), True),
        ("first_representable_value_outside", 1.0, math.nextafter(boundary, math.inf), False),
        ("positive_zero_vs_negative_zero", 0.0, -0.0, True),
        ("zero_vs_minimum_positive_subnormal", 0.0, math.ulp(0.0), False),
        ("sign_mismatch", 1.0, -1.0, False),
        ("reversed_observed_operands", 3925.8544224384018, 3925.8532969524972, True),
        ("lower_side_boundary", 1.0, 1.0 / (1.0 + RTOL), True),
        ("negative_step_norm", -3925.8532969524972, -3925.8544224384018, False),
    ):
        numeric.append(comparison_case(name, record(a), record(b), expected))

    nonfinite = []
    for name, side, token in (
        ("reference_nan", "reference", "nan"),
        ("actual_nan", "actual", "nan"),
        ("reference_positive_infinity", "reference", "+infinity"),
        ("actual_negative_infinity", "actual", "-infinity"),
    ):
        reference, actual = record(1.0), record(1.0)
        target = reference if side == "reference" else actual
        target["scalar"] = {"class": token}
        nonfinite.append(comparison_case(name, reference, actual, False))

    poisons = [
        poison("wrong_model", "model_definition_sha256", "wrong-model"),
        poison("wrong_configuration", "configuration_sha256", "wrong-configuration"),
        poison("wrong_transaction", "transaction_id", 54),
        poison("wrong_occupancy", "occupancy_id", {"stratum_id": "canopy", "tile_id": "tile-b"}),
        poison("wrong_pass", "pass", "potential"), poison("wrong_solve", "solve", "canopy_energy"),
        poison("wrong_field", "field", "matrix_norm"), poison("wrong_failure", "typed_failure", "iteration_limit"),
        poison("candidate_present", "candidate", {"state": "forbidden"}),
        poison("wrong_unit", "unit", "kelvin"), poison("wrong_basis", "basis", "normalized_residual"),
        poison("presence_mismatch", "present", False), poison("wrong_iteration_count", "iterations", 8),
        poison("wrong_backtracking_count", "backtracking_count", 93),
        poison("wrong_cardinality", "residual_cardinality", 5),
        poison("wrong_bound_order", "active_bounds", ["upper", "lower"]),
        poison("wrong_cap_order", "active_water_caps", ["soil-2", "soil-1"]),
        poison("wrong_branch", "branches", ["authorization_active_or_tie", "constitutive_law"]),
        poison("accepted_value_firewall", "accepted_value", True),
    ]
    rollback_reference, rollback_actual = record(1.0), record(1.0)
    rollback_actual["rollback_sha256_after"] = "mutated-snapshot"
    poisons.append(comparison_case("rollback_mutation", rollback_reference, rollback_actual, False))
    all_cases = numeric + nonfinite + poisons
    if any(row["expected_equal"] != row["observed_equal"] for row in all_cases):
        raise RuntimeError("case expectation mismatch")
    transition = identity_transition()
    return {
        "base_model_sha256": "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3",
        "checks": {"all_cases_execute": True, "all_expected_results_match": True,
                   "boundary_is_inclusive": numeric[1]["observed_equal"],
                   "first_outside_rejected": not numeric[3]["observed_equal"],
                   "observed_pair_accepted": numeric[0]["observed_equal"],
                   "signed_zeros_equal": numeric[4]["observed_equal"],
                   "symmetric_reversed_operands": numeric[7]["observed_equal"],
                   "negative_step_norm_rejected": not numeric[9]["observed_equal"],
                   "v5_to_v6_distinct_digests": len({*transition["source"].values(), *transition["target"].values()}) > 2,
                   "v5_to_v6_payload_byte_identical": transition["non_identity_payload_bytes_sha256_before"] == transition["non_identity_payload_bytes_sha256_after"],
                   "zero_nonzero_rejected": not numeric[5]["observed_equal"]},
        "comparison": {"formula": "abs(a-b) <= rtol*max(abs(a),abs(b))",
                       "relative_tolerance": RTOL,
                       "signed_zero_semantics": "+0.0 and -0.0 are one exact zero class",
                       "scope": "finite backtracking_limit.step_norm from same rejected cross-runtime nonlinear trajectory only"},
        "families": {"eligibility_and_firewall_poisons": poisons,
                     "nonfinite_rejections": nonfinite, "numeric_boundary_cases": numeric},
        "identity_transition": transition,
        "model_version": "OPENWEPP_C3_WOODY_V6",
    }


def canonical_bytes(value: object) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify", type=Path)
    args = parser.parse_args()
    generated = canonical_bytes(generate())
    if args.verify:
        if args.verify.read_bytes() != generated:
            raise SystemExit("FAIL: committed V6 vectors differ from independent regeneration")
        print("PASS: committed V6 vectors match independent regeneration")
    else:
        print(generated.decode(), end="")


if __name__ == "__main__":
    main()
