#!/usr/bin/env python3
"""Independent analytic reconstruction of the attempt12 drainage reduction.

This reads the already-preserved iteration-three witness.  It does not execute
the Rust evaluator or any physical simulation.  It reconstructs the proposed
two-coordinate affine elimination twice: once in binary64-style scalar
arithmetic and once at 100-digit precision over the emitted decimal literals.
"""

from __future__ import annotations

import hashlib
import json
import math
from decimal import Decimal, getcontext
from pathlib import Path


TRACE = Path(__file__).with_name("m1-original60-12.stderr")
UPSTREAM_RECONSTRUCTION = Path(__file__).with_name(
    "m1-original60-12-linear-reconstruction.json"
)
EXPECTED_TRACE_SHA256 = (
    "08ac08c43762bd9ae5e9fd2e4f5ee948056f439cdc76320af29a44c54b70bd1c"
)
EXPECTED_UPSTREAM_RECONSTRUCTION_SHA256 = (
    "f2049668aacf886ff679549ac1b52d638d9439f329c7e92c96afd85b904fb61e"
)
ELIMINATED = (5, 11)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def witness(parse_float):
    selected = None
    for line in TRACE.read_text(encoding="utf-8").splitlines():
        if line.startswith("m1-linear-witness "):
            value = json.loads(line.split(" ", 1)[1], parse_float=parse_float)
            if value["iteration"] == 3:
                selected = value
    if selected is None:
        raise RuntimeError("iteration-three witness is absent")
    return selected


def gaussian_solve(matrix, rhs, zero, abs_fn):
    """Partial-pivot Gaussian elimination, mirroring the existing LU order."""
    a = [row[:] for row in matrix]
    b = rhs[:]
    n = len(b)
    minimum_pivot = None
    for column in range(n):
        pivot_row = max(range(column, n), key=lambda row: abs_fn(a[row][column]))
        pivot = abs_fn(a[pivot_row][column])
        if pivot == zero:
            raise ArithmeticError("singular reduced pivot")
        minimum_pivot = pivot if minimum_pivot is None else min(minimum_pivot, pivot)
        a[column], a[pivot_row] = a[pivot_row], a[column]
        b[column], b[pivot_row] = b[pivot_row], b[column]
        for row in range(column + 1, n):
            factor = a[row][column] / a[column][column]
            a[row][column] = zero
            for entry in range(column + 1, n):
                a[row][entry] -= factor * a[column][entry]
            b[row] -= factor * b[column]
    solution = [zero] * n
    for row in range(n - 1, -1, -1):
        tail = sum(
            (a[row][entry] * solution[entry] for entry in range(row + 1, n)),
            zero,
        )
        solution[row] = (b[row] - tail) / a[row][row]
    return solution, minimum_pivot


def reduce_and_reconstruct(record, zero, abs_fn):
    a = record["normalized_jacobian"]
    b = record["rhs"]
    current = record["coordinates"]
    retained = [index for index in range(21) if index not in ELIMINATED]
    eliminated_direction = {index: -current[index] for index in ELIMINATED}
    reduced_matrix = [[a[row][column] for column in retained] for row in retained]
    reduced_rhs = [
        b[row]
        - sum(
            (a[row][column] * eliminated_direction[column] for column in ELIMINATED),
            zero,
        )
        for row in retained
    ]
    reduced_direction, minimum_pivot = gaussian_solve(
        reduced_matrix, reduced_rhs, zero, abs_fn
    )
    direction = [zero] * 21
    for column in ELIMINATED:
        direction[column] = eliminated_direction[column]
    for column, value in zip(retained, reduced_direction):
        direction[column] = value
    return a, b, current, retained, direction, minimum_pivot


def decimal_string(value: Decimal) -> str:
    return format(value, ".30E")


def main() -> None:
    trace_hash = sha256(TRACE)
    upstream_hash = sha256(UPSTREAM_RECONSTRUCTION)
    if trace_hash != EXPECTED_TRACE_SHA256:
        raise RuntimeError(f"unexpected trace SHA-256: {trace_hash}")
    if upstream_hash != EXPECTED_UPSTREAM_RECONSTRUCTION_SHA256:
        raise RuntimeError(f"unexpected upstream reconstruction SHA-256: {upstream_hash}")

    # Binary64 replay of the serialized witness values.
    binary = witness(float)
    ba, bb, bcurrent, retained, bd, binary_minimum_pivot = reduce_and_reconstruct(
        binary, 0.0, abs
    )
    binary_remainders = [
        math.fsum([*(ba[row][column] * bd[column] for column in range(21)), -bb[row]])
        for row in range(21)
    ]
    binary_original_direction = binary["delta"]

    # Independent high-precision calculation over the exact emitted literals.
    getcontext().prec = 100
    decimal = witness(Decimal)
    da, db, dcurrent, _, dd, decimal_minimum_pivot = reduce_and_reconstruct(
        decimal, Decimal(0), abs
    )
    decimal_remainders = [
        sum((da[row][column] * dd[column] for column in range(21)), Decimal(0))
        - db[row]
        for row in range(21)
    ]

    d_column_nonzeros = [
        {
            "row": row,
            "upper_d_column": repr(ba[row][5]),
            "lower_d_column": repr(ba[row][11]),
        }
        for row in range(21)
        if ba[row][5] != 0.0 or ba[row][11] != 0.0
    ]
    candidate_factors = (1.0, 0.5, 2.0**-20)

    result = {
        "evidence_class": "Static independent analytic reconstruction; no Rust evaluator or physical run",
        "source": {
            "trace": str(TRACE),
            "trace_sha256": trace_hash,
            "upstream_reconstruction": str(UPSTREAM_RECONSTRUCTION),
            "upstream_reconstruction_sha256": upstream_hash,
            "source_tree_sha256": "3b1fab62f385f7ec57bfedc81b734cbdcd8f7020de81f7d48fc3a8d28bac8bc9",
            "iteration": 3,
        },
        "method": {
            "eligible_coordinates": list(ELIMINATED),
            "retained_coordinates": retained,
            "affine_direction": "dD=-D from the selected unscaled dt*D row",
            "reduction": "A_FF*d_F=b_F-A_FE*d_E",
            "solve": "partial-pivot Gaussian elimination in retained coordinate order",
            "binary_residual": "math.fsum([A_ij*d_j..., -b_i]) after binary64 scalar solve",
            "decimal_residual": "100-digit Decimal arithmetic over exact emitted JSON decimal literals",
        },
        "structural_coupling": {
            "nonzero_d_columns": d_column_nonzeros,
            "conclusion": "Rows 3 and 9 retain physical D coupling; A_FE substitution is mandatory.",
        },
        "binary64_replay": {
            "eliminated_direction": {str(i): repr(bd[i]) for i in ELIMINATED},
            "full_step_drainage": {
                str(i): repr((1.0 - 1.0) * bcurrent[i]) for i in ELIMINATED
            },
            "drainage_row_remainders": {
                str(i): repr(binary_remainders[i]) for i in ELIMINATED
            },
            "maximum_absolute_original_system_remainder": repr(
                max(abs(value) for value in binary_remainders)
            ),
            "maximum_remaining_direction_change_from_recorded_full_lu": repr(
                max(abs(bd[i] - binary_original_direction[i]) for i in retained)
            ),
            "reduced_minimum_pivot": repr(binary_minimum_pivot),
            "line_search_drainage": [
                {
                    "alpha": repr(alpha),
                    "coordinates": {
                        str(i): repr((1.0 - alpha) * bcurrent[i])
                        for i in ELIMINATED
                    },
                }
                for alpha in candidate_factors
            ],
        },
        "high_precision_emitted_literals": {
            "eliminated_direction": {
                str(i): decimal_string(dd[i]) for i in ELIMINATED
            },
            "drainage_row_remainders": {
                str(i): decimal_string(decimal_remainders[i]) for i in ELIMINATED
            },
            "maximum_absolute_original_system_remainder": decimal_string(
                max(abs(value) for value in decimal_remainders)
            ),
            "reduced_minimum_pivot": decimal_string(decimal_minimum_pivot),
        },
        "interpretation": [
            "The reconstructed direction enforces the source affine identity dD+D=0 for both selected drainage rows.",
            "The recorded upper and lower D columns affect noneliminated mass/enthalpy rows, so solving A_FF without subtracting A_FE*d_E would solve a different system.",
            "For this witness, the binary64 upper-row product rounds exactly to the recorded normalized RHS, while exact arithmetic over the emitted decimal literals exposes -4.4e-26. This witness supports explicit verification and honest reporting, not an assumption that every normalized row is bit-exact.",
            "The calculation establishes only the linear representation relationship. It is not physical convergence, nonlinear acceptance, or complete Jacobian validation.",
        ],
    }
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
