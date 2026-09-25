#!/usr/bin/env python3
"""Independent binary64 oracle for the retained M1 constrained face.

This is a control-development calculator.  It does not import, call, or
translate the Rust treatment implementation.  It consumes the retained
observation record, applies the frozen BVLS-02 arithmetic literally, and emits
the expected per-coordinate predicates for a future shared-implementation
test.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import struct
from pathlib import Path
from typing import Any


N = 21
UNIT_ROUNDOFF_DENOMINATOR = 1 << 53
GAMMA_OPERATION_COUNT = 87


class IndeterminateArithmetic(ValueError):
    """A frozen BVLS-02 arithmetic guard refused an operation."""


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def from_bits(value: Any) -> float:
    if not isinstance(value, str) or len(value) != 16:
        raise ValueError(f"expected 16-digit binary64 hexadecimal value, got {value!r}")
    return struct.unpack(">d", bytes.fromhex(value))[0]


def bits(value: float) -> str:
    return struct.pack(">d", float(value)).hex()


def exponent_and_fraction(value: float) -> tuple[int, int]:
    raw = int.from_bytes(struct.pack(">d", value), "big")
    return (raw >> 52) & 0x7FF, raw & ((1 << 52) - 1)


def is_subnormal(value: float) -> bool:
    exponent, fraction = exponent_and_fraction(value)
    return exponent == 0 and fraction != 0


def guard_operand(value: float, label: str) -> None:
    if not math.isfinite(value):
        raise IndeterminateArithmetic(f"optimality_nonfinite:{label}")
    if is_subnormal(value):
        raise IndeterminateArithmetic(f"optimality_non_normal_operand:{label}")


def guard_result(value: float, label: str) -> None:
    if not math.isfinite(value):
        raise IndeterminateArithmetic(f"optimality_nonfinite:{label}")
    if is_subnormal(value):
        raise IndeterminateArithmetic(f"optimality_subnormal_intermediate:{label}")


def checked_mul(left: float, right: float, label: str) -> float:
    guard_operand(left, f"{label}:left")
    guard_operand(right, f"{label}:right")
    result = left * right
    guard_result(result, label)
    if result == 0.0 and left != 0.0 and right != 0.0:
        raise IndeterminateArithmetic(f"optimality_underflow_to_zero:{label}")
    return result


def checked_add(left: float, right: float, label: str) -> float:
    guard_operand(left, f"{label}:left")
    guard_operand(right, f"{label}:right")
    result = left + right
    guard_result(result, label)
    if result == 0.0 and not (
        (left == 0.0 and right == 0.0) or left == -right
    ):
        raise IndeterminateArithmetic(f"optimality_zero_addition:{label}")
    return result


def next_up(value: float, label: str) -> float:
    guard_result(value, f"{label}:input")
    if value < 0.0:
        raise IndeterminateArithmetic(f"optimality_upward_enclosure:{label}:negative")
    if value == 0.0:
        return 0.0
    raw = int.from_bytes(struct.pack(">d", value), "big")
    result = struct.unpack(">d", (raw + 1).to_bytes(8, "big"))[0]
    if not math.isfinite(result) or is_subnormal(result):
        raise IndeterminateArithmetic(f"optimality_upward_enclosure:{label}")
    return result


def upward_mul(left: float, right: float, label: str) -> float:
    guard_operand(left, f"{label}:left")
    guard_operand(right, f"{label}:right")
    if left == 0.0 or right == 0.0:
        return 0.0
    return next_up(checked_mul(left, right, label), label)


def upward_add(left: float, right: float, label: str) -> float:
    """Add one written nonzero term and enclose upward.

    A zero explicit right-hand term is skipped as permitted by the frozen text.
    A nonzero term accumulated into a zero running sum remains a written
    addition and therefore receives the required next-up step.
    """
    guard_operand(left, f"{label}:left")
    guard_operand(right, f"{label}:right")
    if right == 0.0:
        return left
    return next_up(checked_add(left, right, label), label)


def decode_vector(values: list[Any], length: int = N) -> list[float]:
    result = [from_bits(value) for value in values]
    if len(result) != length:
        raise ValueError(f"expected {length} values, got {len(result)}")
    return result


def ordered_kkt(
    matrix: list[list[float]], residual: list[float], step: list[float], lam: float
) -> tuple[list[float], list[float], list[float], list[float]]:
    r_hat: list[float] = []
    for row in range(N):
        ap = 0.0
        for col in range(N):
            term = checked_mul(matrix[row][col], step[col], f"r:{row}:mul:{col}")
            ap = checked_add(ap, term, f"r:{row}:add:{col}")
        r_hat.append(checked_add(residual[row], ap, f"r:{row}:f_add"))

    g_hat: list[float] = []
    for col in range(N):
        total = 0.0
        for row in range(N):
            term = checked_mul(matrix[row][col], r_hat[row], f"g:{col}:mul:{row}")
            total = checked_add(total, term, f"g:{col}:add:{row}")
        g_hat.append(total)

    lambda_p: list[float] = []
    h_hat: list[float] = []
    for col in range(N):
        product = checked_mul(lam, step[col], f"lambda_p:{col}")
        lambda_p.append(product)
        h_hat.append(checked_add(g_hat[col], product, f"h:{col}"))
    return r_hat, g_hat, lambda_p, h_hat


def gamma87() -> float:
    denominator = float(UNIT_ROUNDOFF_DENOMINATOR - GAMMA_OPERATION_COUNT)
    raw = float(GAMMA_OPERATION_COUNT) / denominator
    return next_up(raw, "gamma87")


def coordinate_enclosure(
    coordinate: int,
    matrix: list[list[float]],
    residual: list[float],
    step: list[float],
    lam: float,
    coefficient: float,
) -> tuple[float, float]:
    outer = 0.0
    for row in range(N):
        inner = 0.0
        for col in range(N):
            product = upward_mul(
                abs(matrix[row][col]), abs(step[col]), f"c:{coordinate}:r:{row}:j:{col}:mul"
            )
            inner = upward_add(inner, product, f"c:{coordinate}:r:{row}:j:{col}:add")
        row_magnitude = upward_add(
            abs(residual[row]), inner, f"c:{coordinate}:r:{row}:f_add"
        )
        contribution = upward_mul(
            abs(matrix[row][coordinate]),
            row_magnitude,
            f"c:{coordinate}:r:{row}:outer_mul",
        )
        outer = upward_add(outer, contribution, f"c:{coordinate}:r:{row}:outer_add")

    lambda_term = upward_mul(abs(lam), abs(step[coordinate]), f"c:{coordinate}:lambda_mul")
    cbar = upward_add(outer, lambda_term, f"c:{coordinate}:lambda_add")
    tau = upward_mul(coefficient, cbar, f"tau:{coordinate}")
    return cbar, tau


def ordered_square_norm(step: list[float]) -> tuple[float, float]:
    total = 0.0
    for coordinate, value in enumerate(step):
        square = checked_mul(value, value, f"norm:mul:{coordinate}")
        total = checked_add(total, square, f"norm:add:{coordinate}")
    return total, math.sqrt(total)


def classify(
    h_value: float,
    tau: float,
    lower_active: bool,
    upper_active: bool,
    free: bool,
    fixed: bool,
) -> str:
    if fixed:
        return "fixed"
    if lower_active:
        return "release_lower" if h_value < -tau else "retain_lower"
    if upper_active:
        return "release_upper" if h_value > tau else "retain_upper"
    if free:
        return "free_pass" if abs(h_value) <= tau else "free_refuse"
    return "invalid_mask"


def exact_bits(left: list[float], right: list[float]) -> bool:
    return all(bits(a) == bits(b) for a, b in zip(left, right, strict=True))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("capture", type=Path)
    parser.add_argument("--historical-reconstruction", required=True, type=Path)
    parser.add_argument("--method", required=True, type=Path)
    parser.add_argument("--operation", default=1, type=int)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()

    document = json.loads(args.capture.read_text(encoding="utf-8"))
    historical = json.loads(args.historical_reconstruction.read_text(encoding="utf-8"))
    matrix = [decode_vector(row) for row in document["weighted_matrix"]]
    residual = decode_vector(document["weighted_residual"])
    operation = document["operations"][args.operation]
    step = decode_vector(operation["p_candidate"])
    lam = from_bits(operation["lambda"])
    lower_active = list(operation["lower_before"])
    upper_active = list(operation["upper_before"])
    free = list(operation["free_before"])
    base = decode_vector(document["base"])
    lower = decode_vector(document["lower"])
    upper = decode_vector(document["upper"])
    scales = decode_vector(document["scales"])

    if lam < 0.0:
        raise IndeterminateArithmetic("optimality_negative_lambda")
    for label, values in (("f", residual), ("p", step), ("base", base), ("scale", scales)):
        for index, value in enumerate(values):
            guard_operand(value, f"{label}:{index}")
    for row in range(N):
        for col in range(N):
            guard_operand(matrix[row][col], f"A:{row}:{col}")

    r_hat, g_hat, lambda_p, h_hat = ordered_kkt(matrix, residual, step, lam)
    coefficient = gamma87()
    enclosures = [
        coordinate_enclosure(index, matrix, residual, step, lam, coefficient)
        for index in range(N)
    ]

    scaled_lower: list[float] = []
    scaled_upper: list[float] = []
    fixed: list[bool] = []
    coordinate_feasible: list[bool] = []
    for index in range(N):
        scaled_lower.append((lower[index] - base[index]) / scales[index])
        scaled_upper.append((upper[index] - base[index]) / scales[index])
        fixed.append(lower[index] == upper[index])
        coordinate_feasible.append(scaled_lower[index] <= step[index] <= scaled_upper[index])

    norm_square, norm = ordered_square_norm(step)
    radius = from_bits(operation["radius"])
    historical_operation = historical["operations"][args.operation]
    recorded_r = decode_vector(historical_operation["linear_residual"])
    recorded_g = decode_vector(operation["g"])
    recorded_lambda_p = decode_vector(operation["lambda_times_p"])
    recorded_h = decode_vector(operation["h"])

    coordinates = []
    for index in range(N):
        cbar, tau = enclosures[index]
        result = classify(
            h_hat[index], tau, lower_active[index], upper_active[index], free[index], fixed[index]
        )
        ratio = None
        diagnostic_ratio = None
        if tau > 0.0:
            diagnostic_value = abs(h_hat[index]) / tau
            if not math.isfinite(diagnostic_value):
                raise IndeterminateArithmetic(f"optimality_release_ratio:{index}")
            diagnostic_ratio = {
                "bits": bits(diagnostic_value),
                "decimal": repr(diagnostic_value),
            }
        if result in ("release_lower", "release_upper") and tau > 0.0:
            ratio_value = abs(h_hat[index]) / tau
            guard_result(ratio_value, f"release_ratio:{index}")
            if ratio_value == 0.0:
                raise IndeterminateArithmetic(f"optimality_release_ratio:{index}")
            ratio = {"bits": bits(ratio_value), "decimal": repr(ratio_value)}
        coordinates.append(
            {
                "coordinate": index,
                "mask": (
                    "fixed"
                    if fixed[index]
                    else "lower_active"
                    if lower_active[index]
                    else "upper_active"
                    if upper_active[index]
                    else "free"
                    if free[index]
                    else "invalid"
                ),
                "p": {"bits": bits(step[index]), "decimal": repr(step[index])},
                "h_hat": {"bits": bits(h_hat[index]), "decimal": repr(h_hat[index])},
                "cbar": {"bits": bits(cbar), "decimal": repr(cbar)},
                "tau": {"bits": bits(tau), "decimal": repr(tau)},
                "classification": result,
                "release_ratio": ratio,
                "diagnostic_abs_h_over_tau": diagnostic_ratio,
                "scaled_lower": bits(scaled_lower[index]),
                "scaled_upper": bits(scaled_upper[index]),
                "primal_feasible": coordinate_feasible[index],
            }
        )

    classifications: dict[str, list[int]] = {}
    for item in coordinates:
        classifications.setdefault(item["classification"], []).append(item["coordinate"])

    release_records = []
    activation_records = []
    for record_index, record in enumerate(document["operations"]):
        if record["action"].startswith("release"):
            record_step = decode_vector(record["p_candidate"])
            record_lambda = from_bits(record["lambda"])
            _, _, _, record_h = ordered_kkt(matrix, residual, record_step, record_lambda)
            record_enclosures = [
                coordinate_enclosure(index, matrix, residual, record_step, record_lambda, coefficient)
                for index in range(N)
            ]
            record_classes = [
                classify(
                    record_h[index],
                    record_enclosures[index][1],
                    record["lower_before"][index],
                    record["upper_before"][index],
                    record["free_before"][index],
                    lower[index] == upper[index],
                )
                for index in range(N)
            ]
            release_records.append(
                {
                    "record_index": record_index,
                    "selected_coordinate": record["coordinate"],
                    "classification_at_selected_coordinate": record_classes[record["coordinate"]],
                    "complete_return_test_passes": all(
                        value in ("free_pass", "retain_lower", "retain_upper", "fixed")
                        for value in record_classes
                    ),
                    "step_exact_template": exact_bits(record_step, step),
                    "h_exact_template": exact_bits(record_h, h_hat),
                }
            )
        elif record["action"].startswith("activate"):
            candidate = decode_vector(record["p_candidate"])
            activation_records.append(
                {
                    "record_index": record_index,
                    "action": record["action"],
                    "applied": record["applied"],
                    "selected_coordinate": record["coordinate"],
                    "selected_lower": record["lower"],
                    "theta_bits": record["theta"],
                    "candidate_coordinate_bits": record["p_candidate"][record["coordinate"]],
                    "candidate_violates_lower": candidate[record["coordinate"]]
                    < scaled_lower[record["coordinate"]],
                }
            )

    captured_refusal = document["refusal"]
    repeated_release = all(
        item["selected_coordinate"] == 5
        and item["classification_at_selected_coordinate"] == "release_lower"
        and not item["complete_return_test_passes"]
        and item["step_exact_template"]
        and item["h_exact_template"]
        for item in release_records
    )
    repeated_activation = all(
        item["selected_coordinate"] == 5
        and item["selected_lower"]
        and item["theta_bits"] == "8000000000000000"
        and item["candidate_violates_lower"]
        for item in activation_records
    )

    output = {
        "evidence_class": "independent offline binary64 control oracle; no Rust treatment or physical execution",
        "formula_identity": {
            "method_path": str(args.method),
            "method_sha256": sha256(args.method),
            "gamma_operation_count": GAMMA_OPERATION_COUNT,
            "gamma87": {"bits": bits(coefficient), "decimal": repr(coefficient)},
            "zero_term_interpretation": (
                "skip an explicit zero right-hand term; a nonzero term accumulated into a zero running sum "
                "is a written addition and receives next_up"
            ),
        },
        "input_identity": {
            "capture_path": str(args.capture),
            "capture_sha256": sha256(args.capture),
            "historical_reconstruction_path": str(args.historical_reconstruction),
            "historical_reconstruction_sha256": sha256(args.historical_reconstruction),
            "operation_index": args.operation,
            "operation_action": operation["action"],
            "script_path": str(Path(__file__).resolve()),
            "script_sha256": sha256(Path(__file__).resolve()),
        },
        "record_correspondence": {
            "r_hat_exact_bits": all(bits(a) == bits(b) for a, b in zip(r_hat, recorded_r, strict=True)),
            "g_hat_exact_bits": all(bits(a) == bits(b) for a, b in zip(g_hat, recorded_g, strict=True)),
            "lambda_p_exact_bits": all(
                bits(a) == bits(b) for a, b in zip(lambda_p, recorded_lambda_p, strict=True)
            ),
            "h_hat_exact_bits": all(bits(a) == bits(b) for a, b in zip(h_hat, recorded_h, strict=True)),
        },
        "face_checks": {
            "lambda": {"bits": bits(lam), "decimal": repr(lam), "nonnegative": lam >= 0.0},
            "radius": {"bits": bits(radius), "decimal": repr(radius)},
            "ordered_norm_square": {"bits": bits(norm_square), "decimal": repr(norm_square)},
            "ordered_norm": {"bits": bits(norm), "decimal": repr(norm)},
            "ball_feasible": norm <= radius,
            "all_coordinates_primal_feasible": all(coordinate_feasible),
            "mask_partition_valid": all(
                int(fixed[i]) + int(lower_active[i]) + int(upper_active[i]) + int(free[i]) == 1
                for i in range(N)
            ),
        },
        "classifications": classifications,
        "complete_return_test_passes": (
            all(item["classification"] in ("free_pass", "retain_lower", "retain_upper", "fixed") for item in coordinates)
        ),
        "coordinates": coordinates,
        "trajectory_audit": {
            "captured_operation_count": len(document["operations"]),
            "activation_count": len(activation_records),
            "release_count": len(release_records),
            "all_activations_are_coordinate_5_negative_zero_lower_crossings": repeated_activation,
            "all_releases_remain_coordinate_5_resolvable_lower_releases": repeated_release,
            "last_operation": activation_records[-1] if activation_records else None,
            "captured_refusal_kind": captured_refusal["kind"],
            "captured_refusal_stage": captured_refusal["stage"],
            "revised_policy_implication": (
                "The 21 constrained faces still release coordinate 5; the 22 all-free faces still "
                "activate its lower bound at negative-zero theta. The revised predicate therefore "
                "preserves the recorded two-face recurrence and terminal FacePivotLimit rather than "
                "returning a step or tuning tau."
            ),
            "release_records": release_records,
        },
        "r_hat_bits": [bits(value) for value in r_hat],
        "g_hat_bits": [bits(value) for value in g_hat],
        "lambda_p_bits": [bits(value) for value in lambda_p],
        "h_hat_bits": [bits(value) for value in h_hat],
    }
    args.output.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
