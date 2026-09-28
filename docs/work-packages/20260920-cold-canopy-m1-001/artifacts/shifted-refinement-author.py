#!/usr/bin/env python3
"""Frozen offline author batch for M1-SHIFTED-DEFECT-CORRECTION-PROTOTYPE-01.

The candidate path is scalar IEEE binary64.  Exact Fraction work is confined to
the separately reported reference and post-candidate oracles.  This module exposes
small operations for the non-result-bearing control program; its ``main`` is
the sole result-bearing author batch.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import struct
import sys
import time
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
CAPTURE = HERE / "shifted-face-observed-capture.json"
DOT2 = HERE / "dot2-primitive.py"
CAPTURE_SHA256 = "946d090bbb6ccfcb71eceb6c860fb784041ce6958891358361e44aa1ef136ecb"
DOT2_SHA256 = "78260c2e48572a231f9e1dca7bfaa6ab50d3befa81dbfe5f7f56a41f5abc75f0"
RETAINED_REFERENCE = HERE / "shifted-correctness-reconstruction.json"
RETAINED_REFERENCE_SHA256 = "79d4669f3ca3f7e23bf1222d0f6b07883bd14d4958ac4bd93cfdf895b1b08410"
N = 21
COUNTS = {"scalar": 0, "guards": 0, "exceptions": 0}


class PrototypeLimitation(RuntimeError):
    pass


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def f64(word: str) -> float:
    return struct.unpack(">d", int(word, 16).to_bytes(8, "big"))[0]


def bits(value: float) -> str:
    return struct.pack(">d", float(value)).hex()


def q(value: float) -> Fraction:
    return Fraction.from_float(value)


def normal_or_zero(value: float) -> bool:
    return value == 0.0 or abs(value) >= sys.float_info.min


def guard(value: float, label: str) -> float:
    COUNTS["guards"] += 1
    if not math.isfinite(value) or not normal_or_zero(value):
        COUNTS["exceptions"] += 1
        raise PrototypeLimitation(f"{label}: nonfinite or nonzero subnormal {bits(value)}")
    return value


def snapshot() -> dict:
    return dict(COUNTS)


def count_delta(before: dict) -> dict:
    return {key: COUNTS[key] - before[key] for key in COUNTS}


def mul(left: float, right: float, label: str) -> float:
    COUNTS["scalar"] += 1
    guard(left, label + ".left"); guard(right, label + ".right")
    value = left * right
    if value == 0.0 and left != 0.0 and right != 0.0:
        COUNTS["exceptions"] += 1
        raise PrototypeLimitation(label + ": nonzero-product underflow")
    return guard(value, label + ".result")


def add(left: float, right: float, label: str) -> float:
    COUNTS["scalar"] += 1
    guard(left, label + ".left"); guard(right, label + ".right")
    value = left + right
    if value == 0.0 and (left != 0.0 or right != 0.0) and bits(left) != bits(-right):
        COUNTS["exceptions"] += 1
        raise PrototypeLimitation(label + ": unexpected zero-add")
    return guard(value, label + ".result")


def ordered_residual(A: list[list[float]], f: list[float], p: list[float]) -> list[float]:
    out = []
    for row in range(N):
        total = 0.0
        for col in range(N):
            total = add(total, mul(A[row][col], p[col], f"r[{row},{col}]"), f"r[{row}]")
        out.append(add(f[row], total, f"r[{row}].f"))
    return out


def ordinary_defect(A: list[list[float]], f: list[float], p: list[float], lam: float, columns: list[int] | None = None) -> list[float]:
    residual = ordered_residual(A, f, p)
    cols = list(range(N)) if columns is None else columns
    out = []
    for col in cols:
        total = 0.0
        for row in range(N):
            total = add(total, mul(A[row][col], residual[row], f"g[{col},{row}]"), f"g[{col}]")
        out.append(add(total, mul(lam, p[col], f"lp[{col}]"), f"h[{col}]"))
    return out


def load_dot2():
    if sha256(DOT2) != DOT2_SHA256:
        raise PrototypeLimitation("accepted Dot2 primitive identity mismatch")
    spec = importlib.util.spec_from_file_location("m1_accepted_dot2", DOT2)
    if spec is None or spec.loader is None:
        raise PrototypeLimitation("cannot load accepted Dot2 primitive")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def dot2_defect(A: list[list[float]], f: list[float], p: list[float], lam: float, columns: list[int], dot2_module) -> tuple[list[float], list[float], dict, list]:
    residual, traces = [], []
    cost = {"dot2_rows": 0, "dot2_gradients": 0, "primitive_arithmetic": 0, "primitive_guards": 0, "primitive_identity_checks": 0}
    for row in range(N):
        d = dot2_module.dot2([f[row], *A[row]], [1.0, *p])
        traces.append(d)
        residual.append(d.value); cost["dot2_rows"] += 1
        cost["primitive_arithmetic"] += d.counters.arithmetic
        cost["primitive_guards"] += d.counters.guards
        cost["primitive_identity_checks"] += d.counters.identity_checks
    out = []
    for col in columns:
        d = dot2_module.dot2([A[row][col] for row in range(N)] + [lam], residual + [p[col]])
        traces.append(d)
        out.append(d.value); cost["dot2_gradients"] += 1
        cost["primitive_arithmetic"] += d.counters.arithmetic
        cost["primitive_guards"] += d.counters.guards
        cost["primitive_identity_checks"] += d.counters.identity_checks
    return residual, out, cost, traces


def shifted_inverse(v: list[list[float]], sigma: list[float], order: list[int], h: list[float], lam: float) -> list[float]:
    """Apply -V diag(1/(sigma²+lambda)) Vᵀ h in stored index order."""
    m = len(h)
    z = [0.0] * m
    for j in order:
        total = 0.0
        for i in range(m):
            total = add(total, mul(v[i][j], h[i], f"vt[{j},{i}]"), f"vt[{j}]")
        denominator = add(mul(sigma[j], sigma[j], f"den.square[{j}]"), lam, f"den[{j}]")
        if denominator == 0.0:
            raise PrototypeLimitation(f"singular shifted denominator {j}")
        COUNTS["scalar"] += 1
        quotient = total / denominator
        if quotient == 0.0 and total != 0.0: raise PrototypeLimitation(f"z[{j}]: division underflow")
        z[j] = guard(quotient, f"z[{j}]")
    delta = []
    for i in range(m):
        total = 0.0
        for j in order:
            total = add(total, mul(v[i][j], z[j], f"vz[{i},{j}]"), f"vz[{i}]")
        COUNTS["scalar"] += 1
        delta.append(guard(-total, f"delta[{i}]"))
    return delta


def update_free(p: list[float], free: list[int], delta: list[float]) -> list[float]:
    out = p[:]
    for slot, col in enumerate(free):
        out[col] = add(out[col], delta[slot], f"update[{col}]")
    return out


def upmul(left: float, right: float, label: str) -> float:
    left, right = abs(guard(left, label + ".left")), abs(guard(right, label + ".right"))
    if left == 0.0 or right == 0.0: return 0.0
    COUNTS["scalar"] += 2
    product = left * right
    if product == 0.0:
        COUNTS["exceptions"] += 1
        raise PrototypeLimitation(label + ": enclosure product underflow")
    return guard(math.nextafter(guard(product, label + ".product"), math.inf), label + ".next")


def upadd(left: float, right: float, label: str) -> float:
    left, right = abs(guard(left, label + ".left")), abs(guard(right, label + ".right"))
    if right == 0.0: return left
    COUNTS["scalar"] += 2
    return guard(math.nextafter(guard(left + right, label + ".sum"), math.inf), label + ".next")


def bvls02(A: list[list[float]], f: list[float], p: list[float], lam: float, lo: list[float], hi: list[float], lower: list[bool], upper: list[bool], radius: float) -> dict:
    if guard(lam, "lambda") < 0:
        raise PrototypeLimitation("negative lambda")
    start = snapshot()
    residual = ordered_residual(A, f, p)
    g, lp, h = [], [], []
    for col in range(N):
        total = 0.0
        for row in range(N): total = add(total, mul(A[row][col], residual[row], f"g[{col},{row}]"), f"g[{col}]")
        g.append(total); lp.append(mul(lam, p[col], f"lp[{col}]")); h.append(add(total, lp[-1], f"h[{col}]"))
    kkt_count = count_delta(start)
    # Coefficient preparation is separately counted, outside the enclosure bound.
    gamma = math.nextafter(87.0 / (2.0**53 - 87.0), math.inf)
    start = snapshot()
    cbar, tau, classes = [], [], []
    for col in range(N):
        cb = 0.0
        for row in range(N):
            inner = 0.0
            for k in range(N): inner = upadd(inner, upmul(A[row][k], p[k], f"cbar[{col},{row},{k}]"), f"cbar.inner[{col},{row}]")
            cb = upadd(cb, upmul(A[row][col], upadd(f[row], inner, f"cbar.scale[{col},{row}]"), f"cbar.term[{col},{row}]"), f"cbar.sum[{col}]")
        cb = upadd(cb, upmul(lam, p[col], f"cbar.lambda.product[{col}]"), f"cbar.lambda.sum[{col}]")
        cbar.append(cb); t = 0.0 if cb == 0.0 else upmul(gamma, cb, f"tau[{col}]")
        tau.append(t)
        if lo[col] == hi[col]: cls = "Fixed"
        elif lower[col]: cls = "ReleaseLower" if h[col] < -t else "RetainLower"
        elif upper[col]: cls = "ReleaseUpper" if h[col] > t else "RetainUpper"
        else: cls = "FreePass" if abs(h[col]) <= t else "FreeRefuse"
        classes.append(cls)
    enclosure_count = count_delta(start)
    n2 = sum((q(x) * q(x) for x in p), Fraction())
    return {"residual_bits": [bits(x) for x in residual], "g_bits": [bits(x) for x in g], "lambda_p_bits": [bits(x) for x in lp], "h_bits": [bits(x) for x in h], "cbar_bits": [bits(x) for x in cbar], "tau_bits": [bits(x) for x in tau], "classes": classes,
            "box_feasible": all(lo[i] <= p[i] <= hi[i] for i in range(N)), "norm_squared_exact": str(n2), "radius_feasible_exact": n2 <= q(radius)*q(radius),
            "work": {"kkt": kkt_count, "enclosure": enclosure_count, "gamma_preparation_scalar": 3}}


class ExactWork:
    """Offline rational operation counts; bit complexity is not implied."""
    def __init__(self):
        self.counts = {"add_sub": 0, "multiply": 0, "divide": 0,
                       "solves": 0, "residual_components_verified": 0,
                       "norm_comparisons": 0, "face_matrix_builds": 0}

    def add(self, a, b):
        self.counts["add_sub"] += 1
        return a + b

    def sub(self, a, b):
        self.counts["add_sub"] += 1
        return a - b

    def mul(self, a, b):
        self.counts["multiply"] += 1
        return a * b

    def div(self, a, b):
        self.counts["divide"] += 1
        return a / b

    def dot(self, a, b):
        result = Fraction()
        for left, right in zip(a, b):
            result = self.add(result, self.mul(left, right))
        return result


def exact_solve(matrix, rhs, work):
    work.counts["solves"] += 1
    rows = [row[:] + [b] for row, b in zip(matrix, rhs)]
    for col in range(len(rhs)):
        pivot = next((r for r in range(col, len(rhs)) if rows[r][col]), None)
        if pivot is None:
            raise PrototypeLimitation(f"exact shifted system singular at {col}")
        rows[col], rows[pivot] = rows[pivot], rows[col]
        divisor = rows[col][col]
        rows[col] = [work.div(x, divisor) for x in rows[col]]
        for row in range(len(rhs)):
            if row != col and rows[row][col]:
                factor = rows[row][col]
                rows[row] = [work.sub(x, work.mul(factor, y)) for x, y in zip(rows[row], rows[col])]
    solution = [rows[i][-1] for i in range(len(rhs))]
    for row, expected in zip(matrix, rhs):
        if work.dot(row, solution) != expected:
            raise PrototypeLimitation("exact linear-system residual failed")
        work.counts["residual_components_verified"] += 1
    return solution


def exact_point_metrics(A, f, p, lam: float) -> dict:
    """Post-candidate exact-binary64 operand reconstruction; never candidate input."""
    residual = [q(f[row]) + sum((q(A[row][col])*q(p[col]) for col in range(N)), Fraction()) for row in range(N)]
    h = [sum((q(A[row][col])*residual[row] for row in range(N)), Fraction()) + q(lam)*q(p[col]) for col in range(N)]
    ordered_h = ordinary_defect(A, f, p, lam)
    return {"exact_residual": [str(x) for x in residual], "exact_h": [str(x) for x in h],
            "ordered_h_evaluation_error": [str(q(ordered_h[i])-h[i]) for i in range(N)],
            "objective_half_residual_norm_squared": str(sum((x*x for x in residual), Fraction()) / 2)}


def norm_and_disposition(assessment, p, p0, free, active, initial_radius, face_radius, tolerance, lam):
    start = snapshot()
    full = reduced = 0.0
    for i in range(N):
        full = add(full, mul(p[i], p[i], f"full_norm[{i}]"), "full_norm")
    for i in free:
        reduced = add(reduced, mul(p[i], p[i], f"reduced_norm[{i}]"), "reduced_norm")
    COUNTS["scalar"] += 2
    full_norm = guard(math.sqrt(full), "full_norm.sqrt")
    reduced_norm = guard(math.sqrt(reduced), "reduced_norm.sqrt")
    norm_counts = count_delta(start)
    start = snapshot()
    full_gap = add(initial_radius, -full_norm, "full.gap")
    gap = add(face_radius, -reduced_norm, "reduced.gap")
    complementarity = mul(lam, gap, "lambda.gap")
    gap_counts = count_delta(start)
    h, tau = [f64(x) for x in assessment["h_bits"]], [f64(x) for x in assessment["tau_bits"]]
    failed = [i for i, cls in enumerate(assessment["classes"]) if cls in ("FreeRefuse", "ReleaseLower", "ReleaseUpper")]
    ratios = [(abs(h[i])/tau[i] if tau[i] else (0.0 if h[i] == 0.0 else math.inf), i) for i in free]
    full_exact = sum((q(x)*q(x) for x in p), Fraction())
    reduced_exact = sum((q(p[i])*q(p[i]) for i in free), Fraction())
    active_equal = all(bits(p[i]) == bits(p0[i]) for i in active)
    full_feasible = full_norm <= initial_radius
    reduced_feasible = reduced_norm <= face_radius
    gap_pass = 0.0 <= gap <= tolerance
    result = {"active_bits_equal_p0": active_equal, "failed_coordinates": failed,
      "stationarity_pass": not failed, "box_feasible": assessment["box_feasible"],
      "max_free_abs_h_over_tau": max(ratios), "full_norm_squared_ordered_bits": bits(full), "reduced_norm_squared_ordered_bits": bits(reduced),
      "full_norm_ordered_bits": bits(full_norm), "reduced_norm_ordered_bits": bits(reduced_norm),
      "largest_raw_free_h": max((abs(h[i]), i) for i in free),
      "full_norm_squared_exact": str(full_exact), "reduced_norm_squared_exact": str(reduced_exact),
      "full_radius_gap_ordered": full_gap, "reduced_radius_gap_ordered": gap, "radius_tolerance": tolerance,
      "full_feasible_ordered": full_feasible, "reduced_feasible_ordered": reduced_feasible,
      "full_feasible_exact_squared": full_exact <= q(initial_radius)*q(initial_radius),
      "reduced_feasible_exact_squared": reduced_exact <= q(face_radius)*q(face_radius),
      "reduced_gap_within_tolerance": gap_pass, "lambda_positive": lam > 0,
      "positive_lambda_complementarity_approx": complementarity,
      "squared_complementarity_exact": str(q(lam)*(q(face_radius)*q(face_radius)-reduced_exact)),
      "all_original_predicates_pass": not failed and active_equal and assessment["box_feasible"] and full_feasible and reduced_feasible and gap_pass,
      "norm_work": norm_counts, "gap_complementarity_work": gap_counts,
      "seam_comparisons": {"active_bits": len(active), "box": 2*N, "full_reduced_radius": 2, "gap": 2},
      "units": "p and radius in dimensionless scaled trust coordinates; h in weighted objective gradient units; lambda in weighted objective units per scaled-coordinate squared; lambda*gap is diagnostic, not exact complementarity"}
    return result


def fixed_lambda_reference(A, f, p0, free, active, lam, radius, lo, hi, lower, upper, initial_radius, tolerance) -> dict:
    if sha256(RETAINED_REFERENCE) != RETAINED_REFERENCE_SHA256: raise PrototypeLimitation("retained fixed-lambda reference identity mismatch")
    retained = json.loads(RETAINED_REFERENCE.read_text())["fixed_lambda_exact_reference"]
    pref = [f64(x) for x in retained["once_rounded_bits"]]
    assess = bvls02(A,f,pref,lam,lo,hi,lower,upper,radius)
    return {"source_sha256": RETAINED_REFERENCE_SHA256, "once_rounded_bits": [bits(x) for x in pref], "assessment": assess,
      "disposition": norm_and_disposition(assess,pref,p0,free,active,initial_radius,radius,tolerance,lam), "step_error_from_p0": [p0[i]-pref[i] for i in range(N)], "exact_metrics": exact_point_metrics(A,f,pref,lam)}


def objectives(A, f, p, lam: float) -> dict:
    residual = [q(f[row]) + sum((q(A[row][col])*q(p[col]) for col in range(N)), Fraction()) for row in range(N)]
    original = sum((x*x for x in residual), Fraction()) / 2
    shifted = original + q(lam)*sum((q(x)*q(x) for x in p), Fraction())/2
    return {"original_half_residual_norm_squared": str(original), "shifted_half_residual_norm_squared": str(shifted)}


def reference(A, f, p, free, active, lam, radius, deadline, runtime_low=5962417150275280.0):
    work = ExactWork()
    exact_A = [[q(x) for x in row] for row in A]
    exact_p = [q(x) for x in p]
    y = [work.add(q(f[r]), work.dot([exact_A[r][i] for i in active], [exact_p[i] for i in active])) for r in range(len(A))]
    columns = [[row[i] for row in exact_A] for i in free]
    gram = [[work.dot(ci, cj) for cj in columns] for ci in columns]
    rhs = [work.sub(Fraction(), work.dot(column, y)) for column in columns]
    work.counts["face_matrix_builds"] += 1
    exact_lam, exact_radius = q(lam), q(radius)
    radius2 = work.mul(exact_radius, exact_radius)
    cache = {}

    def evaluate(value):
        if value not in cache:
            matrix = [[work.add(gram[i][j], value) if i == j else gram[i][j] for j in range(len(free))] for i in range(len(free))]
            solution = exact_solve(matrix, rhs, work)
            norm2 = work.dot(solution, solution)
            difference = work.sub(norm2, radius2)
            work.counts["norm_comparisons"] += 1
            cache[value] = (solution, norm2, difference, True)
        return cache[value]

    narrow_low, high, low = q(runtime_low), exact_lam, Fraction()
    low_narrow_result = evaluate(narrow_low)
    high_result = evaluate(high)
    zero_result = evaluate(low)
    def endpoint(value, result):
        return {"lambda_fraction": str(value), "norm_squared_minus_radius_squared": str(result[2]),
                "sign": (result[2] > 0) - (result[2] < 0), "linear_residual_exact_zero": result[3]}
    out = {"endpoint_evaluations": [endpoint(narrow_low, low_narrow_result), endpoint(high, high_result)],
           "q0": endpoint(low, zero_result),
           "recorded_runtime_bracket_contains_reference": low_narrow_result[2] >= 0 and high_result[2] <= 0,
           "recorded_squared_complementarity_exact": str(work.mul(exact_lam, work.sub(radius2, high_result[1])))}
    if zero_result[2] <= 0 or high_result[2] > 0:
        out.update(bracket_established=False, reason="q(0) does not require positive multiplier or recorded upper is outside radius", exact_reference_operations=work.counts)
        return out
    if high_result[2] == 0:
        low = high
    completed = 0
    trace = []
    while low != high and completed < 80 and time.monotonic() < deadline:
        middle = work.div(work.add(low, high), Fraction(2))
        result = evaluate(middle)
        trace.append(endpoint(middle, result))
        completed += 1
        if result[2] == 0:
            low = high = middle
        elif result[2] > 0:
            low = middle
        else:
            high = middle
    final_low, final_high = evaluate(low), evaluate(high)
    out.update({"bracket_established": True, "bisections_completed": completed,
        "complete_80_or_exact_zero": completed == 80 or low == high,
        "lower_lambda_fraction": str(low), "upper_lambda_fraction": str(high),
        "bracket_width": str(work.sub(high, low)),
        "lambda_error_interval_against_recorded": [str(work.sub(exact_lam, high)), str(work.sub(exact_lam, low))],
        "lambda_error_definition": "recorded multiplier minus true root",
        "final_lower_norm_squared_minus_radius_squared": str(final_low[2]),
        "final_upper_norm_squared_minus_radius_squared": str(final_high[2]),
        "final_endpoint_residuals_exact_zero": [final_low[3], final_high[3]],
        "final_opposite_signs_or_exact_zero": low == high and final_low[2] == 0 or final_low[2] > 0 and final_high[2] < 0,
        "all_linear_residuals_exact_zero": all(value[3] for value in cache.values()),
        "exact_reference_operations": work.counts, "bisection_trace": trace,
        "recorded_endpoint_once_rounded_bits": [bits(float(x)) for x in high_result[0]],
        "recorded_norm_squared_exact": str(high_result[1])})
    return out


def main() -> None:
    parser = argparse.ArgumentParser(); parser.add_argument("--output", required=True); parser.add_argument("--reference-seconds", type=float, default=240.0)
    args = parser.parse_args()
    if sha256(CAPTURE) != CAPTURE_SHA256: raise PrototypeLimitation("capture identity mismatch")
    data = json.loads(CAPTURE.read_text())["face_capture"]; refusal = data["refusal"]; factor = refusal["factor"]
    if not all(k in factor for k in ("v", "sigma", "order", "visits")): raise PrototypeLimitation("captured factor array absent; reconstruction requires separate authorization")
    A = [[f64(x) for x in row] for row in data["weighted_matrix"]]; f = [f64(x) for x in data["weighted_residual"]]
    p0 = [f64(x) for x in refusal["p"]]; lam = f64(refusal["lambda"]); free = list(refusal["free_ids"]); active = [i for i in range(N) if i not in free]
    lo = [f64(x) for x in refusal["scaled_lower"]]; hi = [f64(x) for x in refusal["scaled_upper"]]
    v = [[f64(x) for x in row] for row in factor["v"]]; sigma = [f64(x) for x in factor["sigma"]]
    if factor["a"] != [[bits(A[row][col]) for col in free] for row in range(N)]: raise PrototypeLimitation("factor A does not equal captured free B")
    order = list(factor["order"])
    if len(order) != len(free) or set(order) != set(range(len(free))): raise PrototypeLimitation("captured factor order is not a permutation")
    lower = refusal["lower"]; upper = refusal["upper"]
    if lam != 5962417150275328.0 or lam <= 0: raise PrototypeLimitation("frozen positive shift changed")
    p0_assessment = bvls02(A,f,p0,lam,lo,hi,lower,upper,f64(refusal["radius"]))
    expected = refusal["optimality"]["coordinates"]; observed = data["operations"][-1]
    if p0_assessment["residual_bits"] != observed["residual"] or p0_assessment["g_bits"] != observed["g"] or p0_assessment["lambda_p_bits"] != observed["lambda_times_p"] or p0_assessment["h_bits"] != observed["h"] or p0_assessment["cbar_bits"] != [x["cbar"] for x in expected] or p0_assessment["tau_bits"] != [x["tau"] for x in expected] or p0_assessment["classes"] != [x["class"] for x in expected]: raise PrototypeLimitation("unchanged BVLS-02 p0 replay mismatch")
    start = snapshot(); h0_free = ordinary_defect(A, f, p0, lam, free); defect1_count = count_delta(start)
    start = snapshot(); delta1 = shifted_inverse(v, sigma, order, h0_free, lam); inverse1_count = count_delta(start)
    start = snapshot(); p1 = update_free(p0, free, delta1); update1_count = count_delta(start)
    dot2 = load_dot2(); start = snapshot(); residual1, h1_free, dot_cost, dot_traces = dot2_defect(A, f, p1, lam, free, dot2); dot2_count = count_delta(start)
    start = snapshot(); delta2 = shifted_inverse(v, sigma, order, h1_free, lam); inverse2_count = count_delta(start)
    start = snapshot(); p2 = update_free(p1, free, delta2); update2_count = count_delta(start)
    for trace in dot_traces: dot2.verify_identities(trace)
    dot_cost["primitive_identity_checks"] = sum(trace.counters.identity_checks for trace in dot_traces)
    dot_cost["primitive_exceptional_branches"] = sum(trace.counters.exceptional_branches for trace in dot_traces)
    dot_cost["minimum_nonzero_abs"] = min(trace.ranges.minimum_nonzero_abs for trace in dot_traces if trace.ranges.minimum_nonzero_abs is not None)
    dot_cost["maximum_abs"] = max(trace.ranges.maximum_abs for trace in dot_traces)
    tolerance = f64(refusal["lambda_trace"]["tolerance"]); radius = f64(refusal["radius"]); initial_radius = f64(data["initial_radius"])
    cutoff = time.monotonic() + args.reference_seconds
    start = snapshot(); p1_assessment = bvls02(A,f,p1,lam,lo,hi,lower,upper,radius); p1_kkt_count = count_delta(start)
    start = snapshot(); p2_assessment = bvls02(A,f,p2,lam,lo,hi,lower,upper,radius); p2_kkt_count = count_delta(start)
    result = {"schema": "cold-canopy-m1-shifted-refinement-author-result/v1", "candidate": "M1-SHIFTED-DEFECT-CORRECTION-PROTOTYPE-01",
      "identity": {"capture_sha256": CAPTURE_SHA256, "dot2_sha256": DOT2_SHA256, "factor_identity": factor["same_invocation_identity"], "lambda_bits": refusal["lambda"], "factor_order": order},
      "factor_reconstruction": "not needed: captured V/sigma/order/visits present and captured factor A equals B bitwise",
      "points": {"p0": p0_assessment, "p1": p1_assessment, "p2": p2_assessment},
      "point_dispositions": {"p0": norm_and_disposition(p0_assessment,p0,p0,free,active,initial_radius,radius,tolerance,lam), "p1": norm_and_disposition(p1_assessment,p1,p0,free,active,initial_radius,radius,tolerance,lam), "p2": norm_and_disposition(p2_assessment,p2,p0,free,active,initial_radius,radius,tolerance,lam)},
      "objectives": {"p0": objectives(A,f,p0,lam), "p1": objectives(A,f,p1,lam), "p2": objectives(A,f,p2,lam)},
      "active_bits_preserved": {"p1": all(bits(p1[i]) == bits(p0[i]) for i in active), "p2": all(bits(p2[i]) == bits(p0[i]) for i in active)},
      "candidate_bits": {"p0": [bits(x) for x in p0], "p1": [bits(x) for x in p1], "p2": [bits(x) for x in p2]},
      "correction_intermediates": {"h0_free_bits": [bits(x) for x in h0_free], "delta1_bits": [bits(x) for x in delta1], "dot2_residual1_bits": [bits(x) for x in residual1], "h1_free_bits": [bits(x) for x in h1_free], "delta2_bits": [bits(x) for x in delta2]},
      "post_candidate_exact_metrics": {"p0": exact_point_metrics(A,f,p0,lam), "p1": exact_point_metrics(A,f,p1,lam), "p2": exact_point_metrics(A,f,p2,lam)},
      "additional_work": {"shifted_inverse_applications": 2, "ordinary_full_defect_free_gradients": len(free), "dot2_full_defect_free_gradients": len(free), "free_coordinate_updates": 2*len(free), "shifted_inverse_arithmetic_including_negation": 2*(4*len(free)*len(free)+4*len(free)), "ordinary_defect_arithmetic": N*(2*N+1) + len(free)*(2*N+2), "dot2_defect_arithmetic": (N+len(free))*(25*(N+1)-7), **dot_cost},
      "work_partition": {"defect1": defect1_count, "inverse1": inverse1_count, "update1": update1_count, "dot2": {**dot2_count, **dot_cost}, "inverse2": inverse2_count, "update2": update2_count, "p0_kkt_enclosure": p0_assessment["work"], "p1_kkt_enclosure": p1_kkt_count, "p2_kkt_enclosure": p2_kkt_count, "candidate_scalar_arithmetic": defect1_count["scalar"]+inverse1_count["scalar"]+update1_count["scalar"]+dot_cost["primitive_arithmetic"]+inverse2_count["scalar"]+update2_count["scalar"], "exact_reference_work": "excluded from prospective candidate total"},
      "fixed_lambda_reference": fixed_lambda_reference(A,f,p0,free,active,lam,radius,lo,hi,lower,upper,initial_radius,tolerance),
      "reference": reference(A, f, p0, free, active, lam, radius, cutoff)}
    if result["work_partition"]["candidate_scalar_arithmetic"] != 27446:
        raise PrototypeLimitation("candidate operation count mismatch")
    pref = [f64(x) for x in result["fixed_lambda_reference"]["once_rounded_bits"]]
    result["fixed_lambda_reference"]["objectives"] = objectives(A,f,pref,lam)
    for name, point in (("p0", p0), ("p1", p1), ("p2", p2)):
        result["point_dispositions"][name]["step_error_to_once_rounded_reference"] = [str(q(point[i])-q(pref[i])) for i in range(N)]
        result["point_dispositions"][name]["max_abs_step_error_to_once_rounded_reference"] = float(max(abs(q(point[i])-q(pref[i])) for i in range(N)))
        tau = [f64(x) for x in result["points"][name]["tau_bits"]]
        metrics = result["post_candidate_exact_metrics"][name]
        error = [Fraction(x) for x in metrics["ordered_h_evaluation_error"]]
        metrics["max_abs_evaluation_error_over_tau"] = max(float(abs(error[i])/q(tau[i])) if tau[i] else 0.0 for i in range(N))
    ref_endpoint = result["reference"].get("recorded_endpoint_once_rounded_bits")
    if ref_endpoint is not None:
        result["reference"]["matches_retained_fixed_lambda_bits"] = ref_endpoint == [bits(pref[i]) for i in free]
        if not result["reference"]["matches_retained_fixed_lambda_bits"]:
            raise PrototypeLimitation("new required endpoint solve disagrees with retained reference")
    result["work_partition"]["norms_and_gaps_by_point"] = {name: {"norm": d["norm_work"], "gap": d["gap_complementarity_work"], "seam_comparisons": d["seam_comparisons"]} for name, d in result["point_dispositions"].items()}
    result["work_partition"]["post_candidate_exact_oracle"] = {"points": 4, "eft_identity_checks": dot_cost["primitive_identity_checks"], "classification": "offline diagnostic work, excluded from candidate; exact residual/h, objective and step-error reconstructions"}
    result["identity"]["author_sha256"] = sha256(Path(__file__))
    Path(args.output).write_text(json.dumps(result, indent=2, sort_keys=True, allow_nan=False) + "\n")


if __name__ == "__main__": main()
