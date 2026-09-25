#!/usr/bin/env python3
"""Labelled high-precision diagnostic for the retained face-pivot linear system.

This uses only the recorded binary64 A and f.  It is not a nonlinear replay or
a proposed runtime precision change.  Decimal Gaussian elimination is repeated
at increasing precision so precision-limited residues are visible rather than
being mistaken for exact symbolic results.
"""

from __future__ import annotations

import argparse
import json
import struct
from decimal import Decimal, localcontext
from pathlib import Path


def binary64(value: str) -> float:
    return struct.unpack(">d", bytes.fromhex(value))[0]


def solve(matrix: list[list[Decimal]], rhs: list[Decimal]) -> list[Decimal]:
    size = len(rhs)
    augmented = [row[:] + [rhs[index]] for index, row in enumerate(matrix)]
    for pivot_index in range(size):
        selected = max(range(pivot_index, size), key=lambda row: abs(augmented[row][pivot_index]))
        if augmented[selected][pivot_index] == 0:
            raise ArithmeticError(f"singular pivot {pivot_index}")
        augmented[pivot_index], augmented[selected] = augmented[selected], augmented[pivot_index]
        pivot = augmented[pivot_index][pivot_index]
        for row in range(pivot_index + 1, size):
            multiplier = augmented[row][pivot_index] / pivot
            for column in range(pivot_index + 1, size + 1):
                augmented[row][column] -= multiplier * augmented[pivot_index][column]
            augmented[row][pivot_index] = Decimal(0)
    result = [Decimal(0)] * size
    for row in range(size - 1, -1, -1):
        remainder = sum(augmented[row][column] * result[column] for column in range(row + 1, size))
        result[row] = (augmented[row][size] - remainder) / augmented[row][row]
    return result


def diagnose(capture: dict, precision: int) -> dict[str, str | int]:
    with localcontext() as context:
        context.prec = precision
        a = [[Decimal.from_float(binary64(value)) for value in row] for row in capture["weighted_matrix"]]
        f = [Decimal.from_float(binary64(value)) for value in capture["weighted_residual"]]
        count = len(f)
        all_free = solve(a, [-value for value in f])
        free_ids = [index for index in range(count) if index != 5]
        gram = [
            [sum(a[row][left] * a[row][right] for row in range(count)) for right in free_ids]
            for left in free_ids
        ]
        normal_rhs = [-sum(a[row][column] * f[row] for row in range(count)) for column in free_ids]
        free_solution = solve(gram, normal_rhs)
        constrained = [Decimal(0)] * count
        for column, value in zip(free_ids, free_solution, strict=True):
            constrained[column] = value
        residual = [
            f[row] + sum(a[row][column] * constrained[column] for column in range(count))
            for row in range(count)
        ]
        gradient = [
            sum(a[row][column] * residual[row] for row in range(count))
            for column in range(count)
        ]
        return {
            "decimal_digits": precision,
            "all_free_coordinate_5": str(all_free[5]),
            "lower_face_gradient_5": str(gradient[5]),
            "max_abs_lower_face_free_gradient": str(max(abs(gradient[index]) for index in free_ids)),
            "max_abs_lower_face_residual": str(max(abs(value) for value in residual)),
        }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("capture", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    capture = json.loads(args.capture.read_text(encoding="utf-8"))
    activation = capture["operations"][0]
    release = capture["operations"][1]
    result = {
        "evidence_class": "higher-precision diagnostic of the same retained linear system",
        "runtime_policy_change": False,
        "coordinate": 5,
        "observed_binary64": {
            "all_free_candidate": binary64(activation["p_candidate"][5]),
            "lower_face_g": binary64(release["g"][5]),
            "lower_face_h": binary64(release["h"][5]),
            "lambda": binary64(release["lambda"]),
        },
        "decimal_convergence": [diagnose(capture, precision) for precision in (50, 70, 90, 120, 160, 200)],
        "interpretation_limit": (
            "Convergence toward zero identifies binary64 cancellation sensitivity; "
            "decimal elimination is not an exact symbolic proof or a runtime correction."
        ),
    }
    encoded = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.write_text(encoded, encoding="utf-8")
    else:
        print(encoded, end="")


if __name__ == "__main__":
    main()
