#!/usr/bin/env python3
"""Count frozen BVLS-02 threshold work from authenticated face operands.

This independent offline oracle implements the written up_mul/up_add order.
It does not import or call the Rust treatment or the earlier oracle.  Its
``reconstruct_threshold_work`` function can be reused for later captured
assessments by supplying their binary64 A, f, p, lambda, and face masks.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import struct
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


N = 21
CAPTURE_SHA256 = "57c26d5b9215f9e22cacf2958afb60891acc54c46c540e3dcf0e748dff585a8c"
METHOD_SHA256 = "7c596afda1331e50c9a33424e82b411d5ec7b40991f0cbcf8a034865dbddfe23"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def from_bits(value: str) -> float:
    if not isinstance(value, str) or len(value) != 16:
        raise ValueError(f"invalid binary64 encoding: {value!r}")
    return struct.unpack(">d", bytes.fromhex(value))[0]


def bits(value: float) -> str:
    return struct.pack(">d", value).hex()


def is_subnormal(value: float) -> bool:
    raw = int.from_bytes(struct.pack(">d", value), "big")
    return ((raw >> 52) & 0x7FF) == 0 and (raw & ((1 << 52) - 1)) != 0


def guard_operand(value: float, label: str) -> None:
    if not math.isfinite(value):
        raise ValueError(f"optimality_nonfinite:{label}")
    if value != 0.0 and is_subnormal(value):
        raise ValueError(f"optimality_non_normal_operand:{label}")


def guard_result(value: float, label: str) -> None:
    if not math.isfinite(value):
        raise ValueError(f"optimality_upward_enclosure:{label}")
    if value != 0.0 and is_subnormal(value):
        raise ValueError(f"optimality_subnormal_intermediate:{label}")


def next_up(value: float, label: str) -> float:
    guard_result(value, f"{label}:next_up_input")
    if value < 0.0:
        raise ValueError(f"optimality_upward_enclosure:{label}")
    if value == 0.0:
        return 0.0
    raw = int.from_bytes(struct.pack(">d", value), "big")
    result = struct.unpack(">d", (raw + 1).to_bytes(8, "big"))[0]
    guard_result(result, f"{label}:next_up_output")
    return result


@dataclass
class Work:
    multiplications: int = 0
    additions: int = 0
    upward_steps: int = 0
    divisions: int = 0
    zero_multiplications_skipped: int = 0
    zero_right_additions_skipped: int = 0
    sections: dict[str, "Work"] = field(default_factory=dict)

    @property
    def bounded_operations(self) -> int:
        return self.multiplications + self.additions + self.upward_steps + self.divisions

    def add(self, other: "Work") -> None:
        self.multiplications += other.multiplications
        self.additions += other.additions
        self.upward_steps += other.upward_steps
        self.divisions += other.divisions
        self.zero_multiplications_skipped += other.zero_multiplications_skipped
        self.zero_right_additions_skipped += other.zero_right_additions_skipped

    def as_dict(self) -> dict[str, Any]:
        return {
            "multiplications": self.multiplications,
            "additions": self.additions,
            "upward_steps": self.upward_steps,
            "divisions": self.divisions,
            "bounded_operations": self.bounded_operations,
            "zero_multiplications_skipped": self.zero_multiplications_skipped,
            "zero_right_additions_skipped": self.zero_right_additions_skipped,
            **({"sections": {key: value.as_dict() for key, value in self.sections.items()}} if self.sections else {}),
        }


def up_mul(left: float, right: float, work: Work, label: str) -> float:
    guard_operand(left, f"{label}:left")
    guard_operand(right, f"{label}:right")
    if left == 0.0 or right == 0.0:
        work.zero_multiplications_skipped += 1
        return 0.0
    work.multiplications += 1
    product = left * right
    if product == 0.0:
        raise ValueError(f"optimality_underflow_to_zero:{label}")
    guard_result(product, label)
    work.upward_steps += 1
    return next_up(product, label)


def up_add(left: float, right: float, work: Work, label: str) -> float:
    guard_operand(left, f"{label}:left")
    guard_operand(right, f"{label}:right")
    if right == 0.0:
        work.zero_right_additions_skipped += 1
        return left
    work.additions += 1
    result = left + right
    if result == 0.0 and not ((left == 0.0 and right == 0.0) or left == -right):
        raise ValueError(f"optimality_zero_addition:{label}")
    guard_result(result, label)
    work.upward_steps += 1
    return next_up(result, label)


def gamma87() -> tuple[float, Work]:
    """Return Gamma87 and its one-time setup work.

    The positive denominator is formed exactly as an integer.  Only the
    binary64 division and its directed step are counted.
    """
    work = Work(divisions=1)
    raw = 87.0 / float((1 << 53) - 87)
    work.upward_steps = 1
    return next_up(raw, "gamma87"), work


def checked_kkt(
    matrix: list[list[float]], residual: list[float], step: list[float], lam: float
) -> list[float]:
    """Reconstruct h in the unchanged ordered KKT path; work is reused, not added."""
    r_hat = []
    for row in range(N):
        inner = 0.0
        for column in range(N):
            inner = inner + matrix[row][column] * step[column]
        r_hat.append(residual[row] + inner)
    g_hat = []
    for coordinate in range(N):
        total = 0.0
        for row in range(N):
            total = total + matrix[row][coordinate] * r_hat[row]
        g_hat.append(total)
    return [g_hat[i] + lam * step[i] for i in range(N)]


def coordinate_threshold(
    coordinate: int,
    matrix: list[list[float]],
    residual: list[float],
    step: list[float],
    lam: float,
    coefficient: float,
) -> tuple[float, float, Work]:
    work = Work()
    inner_work = Work()
    row_scale_work = Work()
    row_contribution_work = Work()
    lambda_work = Work()
    tau_work = Work()
    outer = 0.0
    for row in range(N):
        inner = 0.0
        for column in range(N):
            term = up_mul(
                abs(matrix[row][column]),
                abs(step[column]),
                inner_work,
                f"c:{coordinate}:r:{row}:j:{column}:mul",
            )
            inner = up_add(inner, term, inner_work, f"c:{coordinate}:r:{row}:j:{column}:add")
        row_scale = up_add(
            abs(residual[row]), inner, row_scale_work, f"c:{coordinate}:r:{row}:f_add"
        )
        contribution = up_mul(
            abs(matrix[row][coordinate]),
            row_scale,
            row_contribution_work,
            f"c:{coordinate}:r:{row}:outer_mul",
        )
        outer = up_add(
            outer, contribution, row_contribution_work, f"c:{coordinate}:r:{row}:outer_add"
        )
    lambda_term = up_mul(abs(lam), abs(step[coordinate]), lambda_work, f"c:{coordinate}:lambda_mul")
    cbar = up_add(outer, lambda_term, lambda_work, f"c:{coordinate}:lambda_add")
    tau = up_mul(coefficient, cbar, tau_work, f"tau:{coordinate}")
    for name, section in (
        ("inner_products_and_sums", inner_work),
        ("row_scale_additions", row_scale_work),
        ("row_contributions_and_outer_sums", row_contribution_work),
        ("lambda_term_and_sum", lambda_work),
        ("tau_product", tau_work),
    ):
        work.sections[name] = section
        work.add(section)
    return cbar, tau, work


def reconstruct_threshold_work(
    matrix: list[list[float]],
    residual: list[float],
    step: list[float],
    lam: float,
    lower_active: list[bool],
    upper_active: list[bool],
) -> dict[str, Any]:
    if len(matrix) != N or any(len(row) != N for row in matrix):
        raise ValueError("expected 21 by 21 matrix")
    if any(len(values) != N for values in (residual, step, lower_active, upper_active)):
        raise ValueError("expected 21-coordinate operands and masks")
    if lam < 0.0:
        raise ValueError("optimality_negative_lambda")
    for name, values in (("f", residual), ("p", step)):
        for index, value in enumerate(values):
            guard_operand(value, f"{name}:{index}")
    for row in range(N):
        for column in range(N):
            guard_operand(matrix[row][column], f"A:{row}:{column}")
    guard_operand(lam, "lambda")

    coefficient, coefficient_work = gamma87()
    h_hat = checked_kkt(matrix, residual, step, lam)
    total = Work()
    coordinates = []
    ranking_work = Work()
    release_candidates = []
    nonzero_cbar_count = 0
    for coordinate in range(N):
        cbar, tau, work = coordinate_threshold(
            coordinate, matrix, residual, step, lam, coefficient
        )
        nonzero_cbar_count += int(cbar != 0.0)
        total.add(work)
        if lower_active[coordinate]:
            release = h_hat[coordinate] < -tau
            bound = "lower"
        elif upper_active[coordinate]:
            release = h_hat[coordinate] > tau
            bound = "upper"
        else:
            release = False
            bound = None
        ratio = None
        if release:
            if tau == 0.0:
                rank = "true_sign_tau_zero"
            else:
                ranking_work.divisions += 1
                ratio = abs(h_hat[coordinate]) / tau
                guard_result(ratio, f"release_ratio:{coordinate}")
                if ratio == 0.0:
                    raise ValueError(f"optimality_release_ratio:{coordinate}")
                rank = "finite_ratio"
            release_candidates.append((coordinate, bound, rank, ratio))
        coordinates.append(
            {
                "coordinate": coordinate,
                "cbar_bits": bits(cbar),
                "tau_bits": bits(tau),
                "h_hat_bits": bits(h_hat[coordinate]),
                "release_candidate": release,
                "work": work.as_dict(),
            }
        )
    per_nonzero_cbar_coefficient_work = Work(
        divisions=nonzero_cbar_count, upward_steps=nonzero_cbar_count
    )
    return {
        "coefficient_bits": bits(coefficient),
        "coefficient_single_value_evaluation_work": coefficient_work.as_dict(),
        "nonzero_cbar_coordinates": nonzero_cbar_count,
        "coefficient_setup_work_if_evaluated_inside_each_nonzero_cbar_call": (
            per_nonzero_cbar_coefficient_work.as_dict()
        ),
        "coordinates": coordinates,
        "cbar_tau_work_all_coordinates": total.as_dict(),
        "release_ranking_work": ranking_work.as_dict(),
        "release_candidates": [
            {
                "coordinate": coordinate,
                "bound": bound,
                "rank_kind": rank,
                "ratio_bits": None if ratio is None else bits(ratio),
            }
            for coordinate, bound, rank, ratio in release_candidates
        ],
    }


def decode_vector(values: list[str]) -> list[float]:
    result = [from_bits(value) for value in values]
    if len(result) != N:
        raise ValueError(f"expected {N} values, got {len(result)}")
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("capture", type=Path)
    parser.add_argument("--method", required=True, type=Path)
    parser.add_argument("--operation", type=int, default=1)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    if sha256(args.capture) != CAPTURE_SHA256:
        raise ValueError("capture hash does not match authenticated retained input")
    if sha256(args.method) != METHOD_SHA256:
        raise ValueError("method hash does not match frozen BVLS-02 authority")
    capture = json.loads(args.capture.read_text(encoding="utf-8"))
    operation = capture["operations"][args.operation]
    matrix = [decode_vector(row) for row in capture["weighted_matrix"]]
    residual = decode_vector(capture["weighted_residual"])
    step = decode_vector(operation["p_candidate"])
    lam = from_bits(operation["lambda"])
    result = reconstruct_threshold_work(
        matrix,
        residual,
        step,
        lam,
        list(operation["lower_before"]),
        list(operation["upper_before"]),
    )
    zero_identity = reconstruct_threshold_work(
        [[0.0] * N for _ in range(N)],
        [0.0] * N,
        [0.0] * N,
        0.0,
        [False] * N,
        [False] * N,
    )
    output = {
        "evidence_class": "independent offline binary64 work oracle; no Rust treatment or physical execution",
        "input_identity": {
            "capture_path": str(args.capture),
            "capture_sha256": sha256(args.capture),
            "method_path": str(args.method),
            "method_sha256": sha256(args.method),
            "operation_index": args.operation,
            "operation_action": operation["action"],
            "script_path": str(Path(__file__).resolve()),
            "script_sha256": sha256(Path(__file__).resolve()),
        },
        "counting_scope": {
            "cbar_tau": "executed multiplication, addition, and next_up operations; permitted zero terms count zero",
            "coefficient_setup": "a single Gamma87 value evaluation is one division plus one next_up; the candidate source-level shape evaluates it once for each nonzero Cbar, reported separately; exact integer denominator construction is not counted",
            "ranking": "one division for each genuine active release with nonzero tau; comparisons and ordering are not arithmetic",
            "kkt": "ordered KKT vectors are independently reconstructed for predicates but are unchanged reused work and excluded from added-work counts",
        },
        "zero_identity_check": {
            "cbar_tau_work_all_coordinates": zero_identity[
                "cbar_tau_work_all_coordinates"
            ],
            "passes_exact_zero_work": zero_identity[
                "cbar_tau_work_all_coordinates"
            ]["bounded_operations"]
            == 0,
        },
        **result,
    }
    args.output.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
