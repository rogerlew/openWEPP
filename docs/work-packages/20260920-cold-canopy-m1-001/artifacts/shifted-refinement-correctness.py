#!/usr/bin/env python3
"""Independent M1 shifted-refinement reconstruction and exact face oracle.

This file does not import or execute the author candidate.  It reuses only the
previously accepted guarded Dot2 primitive, whose identity is pinned below.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import struct
import sys
from dataclasses import dataclass, field
from fractions import Fraction
from pathlib import Path

CAPTURE_SHA256 = "946d090bbb6ccfcb71eceb6c860fb784041ce6958891358361e44aa1ef136ecb"
DOT2_SHA256 = "78260c2e48572a231f9e1dca7bfaa6ab50d3befa81dbfe5f7f56a41f5abc75f0"
EXPECTED_BASE = "2832bcb686862f654cc1ce121246418777cb0212"
EXPECTED_LAMBDA_BITS = "43352ec96c3eaf00"
EXPECTED_FREE = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20]
EXPECTED_ORDER = [9, 3, 13, 14, 15, 16, 17, 18, 19, 11, 12, 7, 4, 10, 1, 2, 8, 0, 6, 5]
MIN_NORMAL = sys.float_info.min


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def f64(text: str) -> float:
    return struct.unpack(">d", bytes.fromhex(text))[0]


def bits(value: float) -> str:
    return struct.pack(">d", float(value)).hex()


def vec(values):
    return [f64(value) for value in values]


def mat(values):
    return [vec(row) for row in values]


def frac(value: float) -> Fraction:
    return Fraction.from_float(value)


def admitted(value: float) -> bool:
    return math.isfinite(value) and (value == 0.0 or abs(value) >= MIN_NORMAL)


@dataclass
class Ledger:
    arithmetic: int = 0
    guards: int = 0
    branches: int = 0
    exceptional_branches: int = 0
    minimum_nonzero_abs: float | None = None
    maximum_abs: float = 0.0
    signed_zero_values: int = 0

    def guard(self, value: float, label: str) -> float:
        self.guards += 1
        self.branches += 1
        magnitude = abs(value)
        self.maximum_abs = max(self.maximum_abs, magnitude)
        if magnitude and (self.minimum_nonzero_abs is None or magnitude < self.minimum_nonzero_abs):
            self.minimum_nonzero_abs = magnitude
        if value == 0.0:
            self.signed_zero_values += 1
        if not admitted(value):
            self.exceptional_branches += 1
            raise ArithmeticError(f"{label}: unsupported {bits(value)}")
        return value

    def mul(self, left: float, right: float, label: str) -> float:
        self.guard(left, label + ".left")
        self.guard(right, label + ".right")
        value = left * right
        self.arithmetic += 1
        self.guard(value, label + ".result")
        self.branches += 1
        if left != 0.0 and right != 0.0 and value == 0.0:
            self.exceptional_branches += 1
            raise ArithmeticError(label + ": underflow to zero")
        return value

    def add(self, left: float, right: float, label: str) -> float:
        self.guard(left, label + ".left")
        self.guard(right, label + ".right")
        value = left + right
        self.arithmetic += 1
        self.guard(value, label + ".result")
        if value == 0.0 and (left != 0.0 or right != 0.0) and bits(left) != bits(-right):
            self.exceptional_branches += 1
            raise ArithmeticError(label + ": unsupported zero addition")
        return value

    def div(self, left: float, right: float, label: str) -> float:
        self.guard(left, label + ".left")
        self.guard(right, label + ".right")
        self.branches += 1
        if right == 0.0:
            self.exceptional_branches += 1
            raise ArithmeticError(label + ": division by zero")
        value = left / right
        self.arithmetic += 1
        self.guard(value, label + ".result")
        if left != 0.0 and value == 0.0:
            self.exceptional_branches += 1
            raise ArithmeticError(label + ": division underflow")
        return value

    def neg(self, value: float, label: str) -> float:
        self.guard(value, label + ".input")
        result = -value
        self.arithmetic += 1
        return self.guard(result, label + ".result")

    def snapshot(self):
        return {
            "arithmetic": self.arithmetic,
            "guards": self.guards,
            "branches": self.branches,
            "exceptional_branches": self.exceptional_branches,
            "minimum_nonzero_abs": self.minimum_nonzero_abs,
            "maximum_abs": self.maximum_abs,
            "signed_zero_values": self.signed_zero_values,
        }


def ordinary_residual(A, f, p, ledger: Ledger, prefix: str):
    residual = []
    for row in range(len(A)):
        total = 0.0
        for col in range(len(p)):
            total = ledger.add(total, ledger.mul(A[row][col], p[col], f"{prefix}.r[{row},{col}].mul"), f"{prefix}.r[{row},{col}].add")
        residual.append(ledger.add(f[row], total, f"{prefix}.r[{row}].f_add"))
    return residual


def ordinary_h_free(A, residual, p, lam, free_ids, ledger: Ledger, prefix: str):
    h = []
    for coordinate in free_ids:
        gradient = 0.0
        for row in range(len(A)):
            gradient = ledger.add(gradient, ledger.mul(A[row][coordinate], residual[row], f"{prefix}.g[{coordinate},{row}].mul"), f"{prefix}.g[{coordinate},{row}].add")
        lp = ledger.mul(lam, p[coordinate], f"{prefix}.lp[{coordinate}]")
        h.append(ledger.add(gradient, lp, f"{prefix}.h[{coordinate}]"))
    return h


def shifted_inverse(v, sigma, order, h_free, lam, ledger: Ledger, prefix: str):
    """V diag(1/(sigma^2+lambda)) V^T h with captured traversal."""
    size = len(sigma)
    z = [0.0] * size
    for sorted_position in range(size):
        spectral = order[sorted_position]
        total = 0.0
        for free_position in range(size):
            term = ledger.mul(v[free_position][spectral], h_free[free_position], f"{prefix}.t[{spectral},{free_position}].mul")
            total = ledger.add(total, term, f"{prefix}.t[{spectral},{free_position}].add")
        square = ledger.mul(sigma[spectral], sigma[spectral], f"{prefix}.den[{spectral}].square")
        denominator = ledger.add(square, lam, f"{prefix}.den[{spectral}].shift")
        z[spectral] = ledger.div(total, denominator, f"{prefix}.z[{spectral}]")
    delta = []
    for free_position in range(size):
        total = 0.0
        for sorted_position in range(size):
            spectral = order[sorted_position]
            term = ledger.mul(v[free_position][spectral], z[spectral], f"{prefix}.delta[{free_position},{spectral}].mul")
            total = ledger.add(total, term, f"{prefix}.delta[{free_position},{spectral}].add")
        delta.append(ledger.neg(total, f"{prefix}.delta[{free_position}].neg"))
    return delta


def update_free(p, free_ids, delta, ledger: Ledger, prefix: str):
    result = list(p)
    active_before = [bits(p[i]) for i in range(len(p)) if i not in free_ids]
    for position, coordinate in enumerate(free_ids):
        result[coordinate] = ledger.add(p[coordinate], delta[position], f"{prefix}.p[{coordinate}]")
    assert active_before == [bits(result[i]) for i in range(len(p)) if i not in free_ids]
    return result


def load_dot2(path: Path):
    if sha256(path) != DOT2_SHA256:
        raise RuntimeError("accepted Dot2 primitive identity mismatch")
    spec = importlib.util.spec_from_file_location("accepted_dot2_primitive", path)
    module = importlib.util.module_from_spec(spec)
    assert spec and spec.loader
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def dot2_defect(A, f, p, lam, free_ids, dot2):
    aggregates = {"arithmetic": 0, "guards": 0, "branches": 0, "exceptional_branches": 0, "identity_checks": 0, "two_product_calls": 0, "two_sum_calls": 0, "split_calls": 0}
    ranges = []

    def run(left, right):
        result = dot2.dot2(left, right)
        for key in aggregates:
            aggregates[key] += getattr(result.counters, key)
        ranges.append(result.ranges)
        traces.append(result)
        return result.value

    traces = []
    residual = [run([f[row]] + A[row], [1.0] + p) for row in range(len(A))]
    h = [run([A[row][coordinate] for row in range(len(A))] + [lam], residual + [p[coordinate]]) for coordinate in free_ids]
    aggregates["minimum_nonzero_abs"] = min(value.minimum_nonzero_abs for value in ranges if value.minimum_nonzero_abs is not None)
    aggregates["maximum_abs"] = max(value.maximum_abs for value in ranges)
    return residual, h, aggregates, traces


def next_up(value: float, ledger: Ledger, label: str) -> float:
    result = math.nextafter(value, math.inf)
    ledger.arithmetic += 1
    return ledger.guard(result, label)


def upmul(left: float, right: float, ledger: Ledger, label: str) -> float:
    left = abs(ledger.guard(left, label + ".left"))
    right = abs(ledger.guard(right, label + ".right"))
    ledger.branches += 1
    if left == 0.0 or right == 0.0:
        return 0.0
    return next_up(ledger.mul(left, right, label + ".mul"), ledger, label + ".next")


def upadd(left: float, right: float, ledger: Ledger, label: str) -> float:
    left = abs(ledger.guard(left, label + ".left"))
    right = abs(ledger.guard(right, label + ".right"))
    ledger.branches += 1
    if right == 0.0:
        return left
    return next_up(ledger.add(left, right, label + ".add"), ledger, label + ".next")


def ordered_norm(values, ledger: Ledger, prefix: str) -> tuple[float, float]:
    total = 0.0
    for index, value in enumerate(values):
        total = ledger.add(total, ledger.mul(value, value, f"{prefix}[{index}].square"), f"{prefix}[{index}].sum")
    ledger.arithmetic += 1
    return ledger.guard(math.sqrt(total), prefix + ".sqrt"), total


def exact_quantities(A, f, p, lam):
    residual = [frac(f[row]) + sum((frac(A[row][col]) * frac(p[col]) for col in range(len(p))), Fraction()) for row in range(len(A))]
    h = [sum((frac(A[row][col]) * residual[row] for row in range(len(A))), Fraction()) + frac(lam) * frac(p[col]) for col in range(len(p))]
    norm2 = sum((frac(value) ** 2 for value in p), Fraction())
    residual_objective = sum((value * value for value in residual), Fraction()) / 2
    shifted_objective = residual_objective + frac(lam) * norm2 / 2
    return residual, h, norm2, residual_objective, shifted_objective


def rational_record(value: Fraction):
    return {"numerator": str(value.numerator), "denominator": str(value.denominator), "float": float(value)}


def bvls02(A, f, p, lam, lower, upper, scaled_lower, scaled_upper, free_ids, original_radius, reduced_radius, active_reference, radius_tolerance, prefix):
    ledger = Ledger()
    residual = ordinary_residual(A, f, p, ledger, prefix + ".ordered")
    gradient, lp, h = [], [], []
    for coordinate in range(len(p)):
        total = 0.0
        for row in range(len(A)):
            total = ledger.add(total, ledger.mul(A[row][coordinate], residual[row], f"{prefix}.g[{coordinate},{row}].mul"), f"{prefix}.g[{coordinate},{row}].add")
        gradient.append(total)
        lp.append(ledger.mul(lam, p[coordinate], f"{prefix}.lp[{coordinate}]"))
        h.append(ledger.add(total, lp[-1], f"{prefix}.h[{coordinate}]"))
    gamma87 = math.nextafter(87.0 / (2.0**53 - 87.0), math.inf)
    cbar, tau, classes = [], [], []
    free_set = set(free_ids)
    for coordinate in range(len(p)):
        scale_total = 0.0
        for row in range(len(A)):
            inner = 0.0
            for col in range(len(p)):
                inner = upadd(inner, upmul(A[row][col], p[col], ledger, f"{prefix}.c[{coordinate},{row},{col}].term"), ledger, f"{prefix}.c[{coordinate},{row},{col}].inner")
            row_scale = upadd(f[row], inner, ledger, f"{prefix}.c[{coordinate},{row}].rowscale")
            scale_total = upadd(scale_total, upmul(A[row][coordinate], row_scale, ledger, f"{prefix}.c[{coordinate},{row}].outer"), ledger, f"{prefix}.c[{coordinate},{row}].sum")
        scale_total = upadd(scale_total, upmul(lam, p[coordinate], ledger, f"{prefix}.c[{coordinate}].lambda"), ledger, f"{prefix}.c[{coordinate}].final")
        cbar.append(scale_total)
        tau.append(0.0 if scale_total == 0.0 else upmul(gamma87, scale_total, ledger, f"{prefix}.tau[{coordinate}]"))
        if scaled_lower[coordinate] == scaled_upper[coordinate]:
            classification = "Fixed"
        elif lower[coordinate]:
            classification = "ReleaseLower" if h[coordinate] < -tau[coordinate] else "RetainLower"
        elif upper[coordinate]:
            classification = "ReleaseUpper" if h[coordinate] > tau[coordinate] else "RetainUpper"
        elif coordinate in free_set:
            classification = "FreePass" if abs(h[coordinate]) <= tau[coordinate] else "FreeRefuse"
        else:
            raise AssertionError("unclassified coordinate")
        classes.append(classification)
    exact_residual, exact_h, exact_norm2, residual_objective, shifted_objective = exact_quantities(A, f, p, lam)
    free_norm2 = sum((frac(p[i]) ** 2 for i in free_ids), Fraction())
    norm_ledger = Ledger()
    full_norm, full_norm2_ordered = ordered_norm(p, norm_ledger, prefix + ".full_norm")
    reduced_norm, reduced_norm2_ordered = ordered_norm([p[i] for i in free_ids], norm_ledger, prefix + ".reduced_norm")
    radius_gap = reduced_radius - reduced_norm
    box = all(scaled_lower[i] <= p[i] <= scaled_upper[i] for i in range(len(p)))
    active_bits_equal = all(bits(p[i]) == active_reference[str(i)] for i in range(len(p)) if i not in free_set)
    allowed = {"FreePass", "RetainLower", "RetainUpper", "Fixed"}
    return {
        "p_bits": [bits(value) for value in p],
        "residual_bits": [bits(value) for value in residual],
        "gradient_bits": [bits(value) for value in gradient],
        "lambda_p_bits": [bits(value) for value in lp],
        "h_bits": [bits(value) for value in h],
        "cbar_bits": [bits(value) for value in cbar],
        "tau_bits": [bits(value) for value in tau],
        "classes": classes,
        "failed_coordinates": [i for i, value in enumerate(classes) if value not in allowed],
        "largest_abs_h_over_tau": max((abs(h[i]) / tau[i], i) for i in range(len(p)) if tau[i] != 0.0),
        "largest_free_abs_h_over_tau": max((abs(h[i]) / tau[i] if tau[i] != 0.0 else (0.0 if h[i] == 0.0 else math.inf), i) for i in free_ids),
        "largest_raw_free_abs_h": max((abs(h[i]), i) for i in free_ids),
        "exact_h": [rational_record(value) for value in exact_h],
        "max_evaluation_error_abs_over_tau": max(float(abs(frac(h[i]) - exact_h[i]) / frac(tau[i])) for i in range(len(p)) if tau[i] != 0.0),
        "box_feasible": box,
        "active_bits_equal": active_bits_equal,
        "full_ball_exact_squared_feasible": exact_norm2 <= frac(original_radius) ** 2,
        "reduced_ball_exact_squared_feasible": free_norm2 <= frac(reduced_radius) ** 2,
        "full_norm_ordered": full_norm,
        "reduced_norm_ordered": reduced_norm,
        "full_norm2_ordered_bits": bits(full_norm2_ordered),
        "reduced_norm2_ordered_bits": bits(reduced_norm2_ordered),
        "original_radius": original_radius,
        "remaining_face_radius": reduced_radius,
        "radius_gap_ordered": radius_gap,
        "radius_gap_within_tolerance": 0.0 <= radius_gap <= radius_tolerance,
        "radius_tolerance": radius_tolerance,
        "lambda_times_radius_gap_ordered": lam * radius_gap,
        "full_norm2_exact": rational_record(exact_norm2),
        "reduced_norm2_exact": rational_record(free_norm2),
        "reduced_radius2_minus_norm2_exact": rational_record(frac(reduced_radius) ** 2 - free_norm2),
        "lambda_times_reduced_radius2_minus_norm2_exact": rational_record(frac(lam) * (frac(reduced_radius) ** 2 - free_norm2)),
        "half_residual_norm2_objective_exact": rational_record(residual_objective),
        "half_residual_norm2_plus_half_lambda_norm2_exact": rational_record(shifted_objective),
        "all_acceptance_pass": all(value in allowed for value in classes) and box and active_bits_equal and exact_norm2 <= frac(original_radius) ** 2 and free_norm2 <= frac(reduced_radius) ** 2 and 0.0 <= radius_gap <= radius_tolerance,
        "arithmetic_and_guards": {"bvls02": ledger.snapshot(), "norms": norm_ledger.snapshot()},
    }


@dataclass
class ExactWork:
    solves: int = 0
    rational_add_sub: int = 0
    rational_mul: int = 0
    rational_div: int = 0
    pivot_tests: int = 0
    exact_residual_checks: int = 0
    norm_comparisons: int = 0
    bisection_midpoints: int = 0


def exact_solve(matrix, rhs, work: ExactWork):
    work.solves += 1
    augmented = [row[:] + [value] for row, value in zip(matrix, rhs)]
    size = len(matrix)
    for col in range(size):
        pivot = None
        for row in range(col, size):
            work.pivot_tests += 1
            if augmented[row][col] != 0:
                pivot = row
                break
        if pivot is None:
            raise ArithmeticError("exact shifted system singular")
        if pivot != col:
            augmented[col], augmented[pivot] = augmented[pivot], augmented[col]
        divisor = augmented[col][col]
        for row in range(col + 1, size):
            if augmented[row][col] == 0:
                continue
            multiplier = augmented[row][col] / divisor
            work.rational_div += 1
            augmented[row][col] = Fraction()
            for inner in range(col + 1, size + 1):
                augmented[row][inner] -= multiplier * augmented[col][inner]
                work.rational_mul += 1
                work.rational_add_sub += 1
    solution = [Fraction() for _ in range(size)]
    for row in range(size - 1, -1, -1):
        total = Fraction()
        for col in range(row + 1, size):
            total += augmented[row][col] * solution[col]
            work.rational_mul += 1
            work.rational_add_sub += 1
        solution[row] = (augmented[row][size] - total) / augmented[row][row]
        work.rational_add_sub += 1
        work.rational_div += 1
    return solution


def build_exact_face(A, f, p, free_ids, work: ExactWork):
    active = [i for i in range(len(p)) if i not in set(free_ids)]
    y = []
    for row in range(len(A)):
        total = frac(f[row])
        for coordinate in active:
            total += frac(A[row][coordinate]) * frac(p[coordinate])
            work.rational_mul += 1
            work.rational_add_sub += 1
        y.append(total)
    gram = []
    rhs = []
    for left in free_ids:
        gram_row = []
        for right in free_ids:
            total = Fraction()
            for row in range(len(A)):
                total += frac(A[row][left]) * frac(A[row][right])
                work.rational_mul += 1
                work.rational_add_sub += 1
            gram_row.append(total)
        total = Fraction()
        for row in range(len(A)):
            total += frac(A[row][left]) * y[row]
            work.rational_mul += 1
            work.rational_add_sub += 1
        rhs.append(-total)
        gram.append(gram_row)
    return active, y, gram, rhs


def exact_face_oracle(gram, rhs, lam: Fraction, radius2: Fraction, work: ExactWork):
    shifted = [row[:] for row in gram]
    for index in range(len(shifted)):
        shifted[index][index] += lam
        work.rational_add_sub += 1
    solution = exact_solve(shifted, rhs, work)
    for row in range(len(shifted)):
        total = Fraction()
        for col in range(len(shifted)):
            total += shifted[row][col] * solution[col]
            work.rational_mul += 1
            work.rational_add_sub += 1
        work.exact_residual_checks += 1
        if total != rhs[row]:
            raise AssertionError("exact shifted residual is nonzero")
    norm2 = Fraction()
    for value in solution:
        norm2 += value * value
        work.rational_mul += 1
        work.rational_add_sub += 1
    work.norm_comparisons += 1
    sign = 1 if norm2 > radius2 else (-1 if norm2 < radius2 else 0)
    return solution, norm2, sign


def multiplier_reference(A, f, p, free_ids, radius, narrow_low, narrow_high, author_root_endpoints):
    work = ExactWork()
    active, y, gram, rhs = build_exact_face(A, f, p, free_ids, work)
    radius2 = frac(radius) ** 2
    cache = {}

    def evaluate(lam):
        if lam not in cache:
            cache[lam] = exact_face_oracle(gram, rhs, lam, radius2, work)
        return cache[lam]

    zero = Fraction()
    recorded = frac(narrow_high)
    narrow_lo = frac(narrow_low)
    zero_result = evaluate(zero)
    low_result = evaluate(narrow_lo)
    high_result = evaluate(recorded)
    q_zero_requires_positive = zero_result[2] > 0
    broad_valid = q_zero_requires_positive and high_result[2] <= 0
    root_checks = []
    lower = upper = None
    lower_result = upper_result = None
    if author_root_endpoints:
        lower, upper = author_root_endpoints
        lower_result, upper_result = evaluate(lower), evaluate(upper)
        for label, value, evaluated in (("lower", lower, lower_result), ("upper", upper, upper_result)):
            root_checks.append({"label": label, "lambda": rational_record(value), "norm2_minus_radius2": rational_record(evaluated[1] - radius2), "sign": evaluated[2], "exact_system_residual_verified": True})
    contains_narrow = low_result[2] >= 0 and high_result[2] <= 0
    ref_solution = high_result[0]
    return {
        "method": "exact Fraction original-matrix fixed-face normal equations; exact Gaussian elimination and residual verification",
        "active_coordinates": active,
        "q_zero_requires_positive_multiplier": q_zero_requires_positive,
        "broad_bracket_valid": broad_valid,
        "q_zero_norm2_minus_radius2": rational_record(zero_result[1] - radius2),
        "narrow_runtime_bracket": {
            "lower": rational_record(narrow_lo),
            "upper": rational_record(recorded),
            "lower_norm2_minus_radius2": rational_record(low_result[1] - radius2),
            "upper_norm2_minus_radius2": rational_record(high_result[1] - radius2),
            "contains_reference_root": contains_narrow,
        },
        "author_root_endpoint_independent_verification": {
            "endpoints_present": bool(author_root_endpoints),
            "checks": root_checks,
            "ordered": bool(author_root_endpoints) and lower <= upper,
            "opposite_sign_or_exact": bool(author_root_endpoints) and (lower_result[2] == 0 or upper_result[2] == 0 or (lower_result[2] > 0 and upper_result[2] < 0)),
        },
        "recorded_lambda_error_interval": None if not author_root_endpoints else {
            "minimum_recorded_minus_root": rational_record(recorded - upper),
            "maximum_recorded_minus_root": rational_record(recorded - lower),
        },
        "recorded_endpoint_solution_exact": ref_solution,
        "work": work.__dict__,
        "exact_system_residuals_verified": work.exact_residual_checks == work.solves * len(free_ids),
    }


def controls(dot2):
    # Three columns, free coordinates 0 and 2.  Column 1 is fixed/active and
    # contributes to the residual; V is deliberately nonsymmetric.
    # B = diag(2,5) V^T in its first two rows, embedded as free columns
    # 0 and 2.  V is nonsymmetric, so V/V^T confusion changes the answer.
    A = [[1.2, 2.0, 1.6], [-4.0, -3.0, 3.0], [0.0, 5.0, 0.0]]
    f = [-1.0, 4.0, -2.0]
    p0 = [0.25, 0.5, -0.25]
    free = [0, 2]
    v = [[0.6, -0.8], [0.8, 0.6]]
    sigma = [2.0, 5.0]
    order = [1, 0]
    lam = 8.0
    ledger = Ledger()
    residual = ordinary_residual(A, f, p0, ledger, "control")
    h = ordinary_h_free(A, residual, p0, lam, free, ledger, "control")
    delta = shifted_inverse(v, sigma, order, h, lam, ledger, "control")
    p1 = update_free(p0, free, delta, ledger, "control")
    work = ExactWork()
    _, _, gram, rhs = build_exact_face(A, f, p0, free, work)
    exact, _, _ = exact_face_oracle(gram, rhs, frac(lam), Fraction(1000), work)
    orientation_ok = all(abs(p1[coordinate] - float(exact[position])) <= 8 * math.ulp(p1[coordinate]) for position, coordinate in enumerate(free))
    h_without_lambda_q = ordinary_h_free(A, residual, p0, 0.0, free, Ledger(), "control.omitted")
    omission_detected = h != h_without_lambda_q and all(
        h[position] == h_without_lambda_q[position] + lam * p0[coordinate]
        for position, coordinate in enumerate(free)
    )
    active_join = bits(p1[1]) == bits(p0[1]) and residual[2] == 0.5
    bounds = [-1.0, 0.5, -1.0], [1.0, 0.5, 1.0]
    admission_pass = all(bounds[0][i] <= p1[i] <= bounds[1][i] for i in range(3)) and sum(value * value for value in p1) <= 10.0
    rejected_bound = not all(bounds[0][i] <= value <= bounds[1][i] for i, value in enumerate([2.0, 0.5, 0.0]))
    rejected_radius = sum(value * value for value in [0.9, 0.5, 0.9]) > 1.0
    dot = dot2.dot2([1.0, 1.0e16, -1.0e16], [1.0, 1.0, 1.0])
    dot2.verify_identities(dot)
    limitation = False
    try:
        dot2.dot2([sys.float_info.min * 0.5], [1.0])
    except dot2.PrototypeLimitation:
        limitation = True
    checks = {
        "nonzero_lambda_term_detected": omission_detected,
        "v_orientation_and_sigma_column_join": orientation_ok,
        "factor_order_is_nonidentity_and_used": order != list(range(2)),
        "inactive_active_join_and_bit_custody": active_join,
        "unchanged_admission_accepts_valid": admission_pass,
        "unchanged_bound_rejects_invalid": rejected_bound,
        "unchanged_radius_rejects_invalid": rejected_radius,
        "dot2_cancellation_control": dot.value == 1.0,
        "dot2_non_normal_refusal": limitation,
    }
    return {"schema": "openwepp.shifted-refinement-correctness-controls.v1", "checks": checks, "passed": all(checks.values()), "candidate_operation_counts": ledger.snapshot(), "exact_control_work": work.__dict__}


def target(capture_path: Path, dot2_path: Path, author_result_path: Path):
    if sha256(capture_path) != CAPTURE_SHA256:
        raise RuntimeError("capture identity mismatch")
    dot2 = load_dot2(dot2_path)
    data = json.loads(capture_path.read_text())
    capture = data["face_capture"]
    refusal = capture["refusal"]
    factor = refusal["factor"]
    A, f, p0 = mat(capture["weighted_matrix"]), vec(capture["weighted_residual"]), vec(refusal["p"])
    lam = f64(refusal["lambda"])
    free_ids = list(refusal["free_ids"])
    order = list(factor["order"])
    v, sigma = mat(factor["v"]), vec(factor["sigma"])
    if refusal["lambda"] != EXPECTED_LAMBDA_BITS or free_ids != EXPECTED_FREE or order != EXPECTED_ORDER:
        raise RuntimeError("frozen face/lambda/order mismatch")
    if len(v) != len(free_ids) or any(len(row) != len(free_ids) for row in v) or len(sigma) != len(free_ids):
        raise RuntimeError("factor shape mismatch")
    lower, upper = refusal["lower"], refusal["upper"]
    scaled_lower, scaled_upper = vec(refusal["scaled_lower"]), vec(refusal["scaled_upper"])
    original_radius, reduced_radius = f64(capture["initial_radius"]), f64(refusal["radius"])
    tolerance = f64(refusal["lambda_trace"]["tolerance"])
    active_reference = {str(i): bits(p0[i]) for i in range(len(p0)) if i not in set(free_ids)}

    first = Ledger()
    residual0 = ordinary_residual(A, f, p0, first, "correction1")
    h0 = ordinary_h_free(A, residual0, p0, lam, free_ids, first, "correction1")
    delta1 = shifted_inverse(v, sigma, order, h0, lam, first, "correction1")
    p1 = update_free(p0, free_ids, delta1, first, "correction1")

    residual1, h1, dot2_counts, dot2_traces = dot2_defect(A, f, p1, lam, free_ids, dot2)
    second = Ledger()
    delta2 = shifted_inverse(v, sigma, order, h1, lam, second, "correction2")
    p2 = update_free(p1, free_ids, delta2, second, "correction2")
    # Exact EFT identities are a post-candidate oracle and cannot affect p2.
    for trace in dot2_traces:
        dot2.verify_identities(trace)
    dot2_identity_checks = sum(trace.counters.identity_checks for trace in dot2_traces)

    points = {
        "original": bvls02(A, f, p0, lam, lower, upper, scaled_lower, scaled_upper, free_ids, original_radius, reduced_radius, active_reference, tolerance, "p0"),
        "ordinary_correction_1": bvls02(A, f, p1, lam, lower, upper, scaled_lower, scaled_upper, free_ids, original_radius, reduced_radius, active_reference, tolerance, "p1"),
        "compensated_correction_2": bvls02(A, f, p2, lam, lower, upper, scaled_lower, scaled_upper, free_ids, original_radius, reduced_radius, active_reference, tolerance, "p2"),
    }
    captured_operation = capture["operations"][-1]
    captured_optimality = refusal["optimality"]["coordinates"]
    p0_capture_replay = {
        "residual_bits": points["original"]["residual_bits"] == captured_operation["residual"],
        "gradient_bits": points["original"]["gradient_bits"] == captured_operation["g"],
        "lambda_p_bits": points["original"]["lambda_p_bits"] == captured_operation["lambda_times_p"],
        "h_bits": points["original"]["h_bits"] == captured_operation["h"],
        "cbar_bits": points["original"]["cbar_bits"] == [row["cbar"] for row in captured_optimality],
        "tau_bits": points["original"]["tau_bits"] == [row["tau"] for row in captured_optimality],
        "classes": points["original"]["classes"] == [row["class"] for row in captured_optimality],
    }
    narrow = [f64(value) for value in refusal["lambda_trace"]["bracket"]]
    author_result = json.loads(author_result_path.read_text())
    author_point_names = {"original": "p0", "ordinary_correction_1": "p1", "compensated_correction_2": "p2"}
    author_comparison = {}
    for own_name, author_name in author_point_names.items():
        author_point = author_result["points"][author_name]
        own_point = points[own_name]
        author_comparison[author_name] = {
            "p_bits": own_point["p_bits"] == author_result["candidate_bits"][author_name],
            "residual_bits": own_point["residual_bits"] == author_point["residual_bits"],
            "gradient_bits": own_point["gradient_bits"] == author_point["g_bits"],
            "lambda_p_bits": own_point["lambda_p_bits"] == author_point["lambda_p_bits"],
            "h_bits": own_point["h_bits"] == author_point["h_bits"],
            "cbar_bits": own_point["cbar_bits"] == author_point["cbar_bits"],
            "tau_bits": own_point["tau_bits"] == author_point["tau_bits"],
            "classes": own_point["classes"] == author_point["classes"],
        }
    author_intermediates = author_result["correction_intermediates"]
    intermediate_comparison = {
        "h0_free_bits": [bits(value) for value in h0] == author_intermediates["h0_free_bits"],
        "delta1_bits": [bits(value) for value in delta1] == author_intermediates["delta1_bits"],
        "dot2_residual1_bits": [bits(value) for value in residual1] == author_intermediates["dot2_residual1_bits"],
        "h1_free_bits": [bits(value) for value in h1] == author_intermediates["h1_free_bits"],
        "delta2_bits": [bits(value) for value in delta2] == author_intermediates["delta2_bits"],
    }
    author_reference = author_result["reference"]
    author_root_endpoints = None
    if author_reference.get("bracket_established") and "lower_lambda_fraction" in author_reference and "upper_lambda_fraction" in author_reference:
        author_root_endpoints = (Fraction(author_reference["lower_lambda_fraction"]), Fraction(author_reference["upper_lambda_fraction"]))
    reference = multiplier_reference(A, f, p0, free_ids, reduced_radius, narrow[0], narrow[1], author_root_endpoints)
    exact_reference = reference.pop("recorded_endpoint_solution_exact")
    pref = list(p0)
    for position, coordinate in enumerate(free_ids):
        pref[coordinate] = float(exact_reference[position])
    points["fixed_lambda_reference_once_rounded"] = bvls02(A, f, pref, lam, lower, upper, scaled_lower, scaled_upper, free_ids, original_radius, reduced_radius, active_reference, tolerance, "pref")
    for name, point in points.items():
        point["max_abs_step_error_to_once_rounded_reference"] = max(abs(f64(value) - pref[index]) for index, value in enumerate(point["p_bits"]))
    reference["recorded_endpoint_once_rounded_bits"] = [bits(value) for value in pref]
    retained_reference = json.loads((capture_path.parent / "shifted-correctness-reconstruction.json").read_text())["fixed_lambda_exact_reference"]["once_rounded_bits"]
    reference["matches_retained_once_rounded_reference_bits"] = reference["recorded_endpoint_once_rounded_bits"] == retained_reference
    result = {
        "schema": "openwepp.shifted-refinement-correctness-result.v1",
        "evidence_class": "Ran: one independent stdlib binary64 two-correction reconstruction, accepted pinned Dot2 primitive, own unchanged BVLS-02 oracle, and exact-Fraction original-matrix endpoint/root oracle; no author candidate, Rust, physical, or historical prototype execution",
        "identity": {"base_commit": EXPECTED_BASE, "independent_source_sha256": sha256(Path(__file__)), "capture_path": str(capture_path), "capture_sha256": sha256(capture_path), "dot2_path": str(dot2_path), "dot2_sha256": sha256(dot2_path), "author_result_path_used_for_comparison_and_root_endpoints_only": str(author_result_path), "author_result_sha256": sha256(author_result_path)},
        "frozen_protocol": {"candidate": "M1-SHIFTED-DEFECT-CORRECTION-PROTOTYPE-01", "lambda_bits": bits(lam), "free_ids": free_ids, "factor_order": order, "v_rows_are_free_positions": True, "v_sigma_columns_are_raw_spectral_indices": True, "spectral_accumulation_traverses_factor_order": True, "corrections": 2},
        "candidate_intermediates": {"correction1_h_free_bits": [bits(value) for value in h0], "correction1_delta_bits": [bits(value) for value in delta1], "correction2_residual_bits": [bits(value) for value in residual1], "correction2_h_free_bits": [bits(value) for value in h1], "correction2_delta_bits": [bits(value) for value in delta2]},
        "original_capture_replay": {**p0_capture_replay, "all_match": all(p0_capture_replay.values())},
        "author_comparison": {"points": author_comparison, "intermediates": intermediate_comparison, "all_match": all(all(group.values()) for group in author_comparison.values()) and all(intermediate_comparison.values())},
        "points": points,
        "multiplier_reference": reference,
        "work": {"correction1_ordinary_defect_inverse_update": first.snapshot(), "correction2_dot2_defect": dot2_counts, "correction2_post_candidate_exact_eft_identity_checks": dot2_identity_checks, "correction2_inverse_update": second.snapshot(), "expected_shifted_inverse_plus_update_arithmetic_each": 1700, "reference_work_excluded_from_candidate": True, "reviewer_did_not_repeat_root_bisection": True},
        "guards": {"all_candidate_exceptional_branches_zero": first.exceptional_branches == 0 and second.exceptional_branches == 0 and dot2_counts["exceptional_branches"] == 0, "active_bits_preserved_all_points": all(point["active_bits_equal"] for point in points.values())},
    }
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["controls", "target"])
    parser.add_argument("capture", type=Path)
    parser.add_argument("dot2", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--author-result", type=Path)
    args = parser.parse_args()
    dot2 = load_dot2(args.dot2)
    if args.mode == "target" and args.author_result is None:
        parser.error("target mode requires --author-result for endpoint-only verification")
    result = controls(dot2) if args.mode == "controls" else target(args.capture, args.dot2, args.author_result)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True, default=lambda value: rational_record(value) if isinstance(value, Fraction) else str(value)) + "\n")
    print(json.dumps({"mode": args.mode, "passed": result.get("passed"), "schema": result["schema"]}, sort_keys=True))


if __name__ == "__main__":
    main()
