#!/usr/bin/env python3
"""Offline fixed-face attribution from the one shifted-face capture.

Reference precisions are declared before calculation: Decimal(100) and
Decimal(200).  The fixed-lambda linear system is formed from exact binary64
rationals, never from rounded normal equations.
"""
import json
import math
import struct
from decimal import Decimal, getcontext
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
CAPTURE = HERE / "shifted-face-observed-capture.json"
OUT = HERE / "shifted-face-attribution.json"

def fbits(word):
    return struct.unpack(">d", int(word, 16).to_bytes(8, "big"))[0]

def rat(word):
    return Fraction.from_float(fbits(word))

def bound(word):
    value = fbits(word)
    return None if math.isinf(value) else Fraction.from_float(value)

def bits(value):
    return f"{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"

def decimal(value, precision):
    getcontext().prec = precision
    return str(Decimal(value.numerator) / Decimal(value.denominator))

def solve(matrix, rhs):
    """Exact-rational Gauss-Jordan solve, with no pivot search beyond first nonzero."""
    n = len(rhs)
    rows = [list(matrix[i]) + [rhs[i]] for i in range(n)]
    for col in range(n):
        pivot = next((row for row in range(col, n) if rows[row][col]), None)
        if pivot is None:
            raise ArithmeticError(f"singular fixed-lambda matrix at {col}")
        rows[col], rows[pivot] = rows[pivot], rows[col]
        scale = rows[col][col]
        rows[col] = [value / scale for value in rows[col]]
        for row in range(n):
            if row == col:
                continue
            scale = rows[row][col]
            if scale:
                rows[row] = [value - scale * pivot_value for value, pivot_value in zip(rows[row], rows[col])]
    return [rows[i][-1] for i in range(n)]

def norm2(values):
    return sum(value * value for value in values)

data = json.loads(CAPTURE.read_text())
face = data["face_capture"]
refusal = face["refusal"]
op = next(item for item in face["operations"] if item["action"] == "optimality-refusal")
free = refusal["free_ids"]
active = [i for i in range(21) if i not in free]
A = [[rat(word) for word in row] for row in op["weighted_matrix"]]
f = [rat(word) for word in op["weighted_residual"]]
p = [rat(word) for word in refusal["p"]]
lam = rat(refusal["lambda"])
radius = rat(refusal["radius"])
y = [f[row] + sum(A[row][col] * p[col] for col in active) for row in range(21)]
B = [[A[row][col] for col in free] for row in range(21)]
q = [p[col] for col in free]
g = [sum(A[row][col] * (sum(A[row][k] * p[k] for k in range(21)) + f[row]) for row in range(21)) for col in range(21)]
h = [g[col] + lam * p[col] for col in range(21)]
recorded_h = [rat(item["h"]) for item in refusal["optimality"]["coordinates"]]
tau = [rat(item["tau"]) for item in refusal["optimality"]["coordinates"]]
classes = []
for col in range(21):
    if col in active:
        cls = "retain_lower" if h[col] >= -tau[col] else "release_lower"
    else:
        cls = "free_pass" if abs(h[col]) <= tau[col] else "free_refuse"
    classes.append(cls)
H = [[sum(B[row][i] * B[row][j] for row in range(21)) + (lam if i == j else 0) for j in range(len(free))] for i in range(len(free))]
rhs = [-sum(B[row][i] * y[row] for row in range(21)) for i in range(len(free))]
qref = solve(H, rhs)
pref = p[:]
for index, col in enumerate(free):
    pref[col] = qref[index]
href = [sum(A[row][col] * (sum(A[row][k] * pref[k] for k in range(21)) + f[row]) for row in range(21)) + lam * pref[col] for col in range(21)]
rounded = [float(value) for value in pref]
rounded_rat = [Fraction.from_float(value) for value in rounded]
rounded_h = [sum(A[row][col] * (sum(A[row][k] * rounded_rat[k] for k in range(21)) + f[row]) for row in range(21)) + lam * rounded_rat[col] for col in range(21)]
lower = [bound(word) for word in refusal["scaled_lower"]]
upper = [bound(word) for word in refusal["scaled_upper"]]
box = [(lower[i] is None or lower[i] <= rounded_rat[i]) and (upper[i] is None or rounded_rat[i] <= upper[i]) for i in range(21)]
norm_gap = radius * radius - norm2(rounded_rat)
recorded_norm = rat(refusal["lambda_trace"]["norm"])
trace = refusal["lambda_trace"]
factor = refusal["factor"]
factor_a = [[rat(word) for word in row] for row in factor["a"]]
factor_a_matches_b = factor_a == B
recorded_norm_exact = norm2(p)
reference_norm_exact = norm2(pref)
step_error = [q[i] - qref[i] for i in range(len(free))]
out = {
  "evidence_class": "Ran: standalone offline exact-rational fixed-face calculation; no runtime invocation",
  "capture": str(CAPTURE),
  "reference_precisions_decimal_digits": [100, 200],
  "method": "exact binary64 rationals; fixed recorded lambda; Gauss-Jordan exact solve",
  "face": {"free_ids": free, "active_ids": active, "lambda_bits": refusal["lambda"], "radius_bits": refusal["radius"], "remaining_radius_bits": refusal["radius"], "initial_radius_bits": face["initial_radius"], "factor_identity_observational": factor["same_invocation_identity"]},
  "recorded": {"p_bits": refusal["p"], "lambda_trace": trace, "factor_sigma_bits": factor["sigma"], "factor_order": factor["order"], "factor_visits": factor["visits"]},
  "exact_h_at_recorded_p": [{"coordinate": i, "exact_100": decimal(h[i],100), "exact_200": decimal(h[i],200), "rounded_bits": bits(float(h[i])), "recorded_h_bits": refusal["optimality"]["coordinates"][i]["h"], "recorded_tau_bits": refusal["optimality"]["coordinates"][i]["tau"], "recorded_match_after_once_round": bits(float(h[i])) == refusal["optimality"]["coordinates"][i]["h"], "classification": classes[i]} for i in range(21)],
  "recorded_face_stationarity": {"free_max_abs_h_100": decimal(max(abs(h[i]) for i in free),100), "free_violations": [i for i in free if abs(h[i]) > tau[i]], "all_coordinate_classes": classes},
  "step_and_factor_fidelity": {"factor_a_matches_recorded_free_B_exactly": factor_a_matches_b, "recorded_vs_fixed_lambda_reference_free_inf_error_100": decimal(max(abs(value) for value in step_error),100), "recorded_vs_fixed_lambda_reference_free_l2_squared_100": decimal(norm2(step_error),100), "recorded_p_norm_squared_100": decimal(recorded_norm_exact,100), "fixed_lambda_reference_norm_squared_100": decimal(reference_norm_exact,100)},
  "fixed_lambda_reference": {"q_exact_100": [decimal(value,100) for value in qref], "q_exact_200": [decimal(value,200) for value in qref], "p_once_rounded_bits": [bits(value) for value in rounded], "free_exact_stationarity_zero": all(href[i] == 0 for i in free), "rounded_free_max_abs_h_100": decimal(max(abs(rounded_h[i]) for i in free),100), "rounded_box_feasible": box, "rounded_all_box_feasible": all(box), "rounded_norm_squared_gap_100": decimal(norm_gap,100), "rounded_ball_feasible": norm_gap >= 0, "fixed_lambda_only": True},
  "trust": {"recorded_lambda_positive": lam > 0, "recorded_norm_100": decimal(recorded_norm,100), "recorded_norm_minus_radius_100": decimal(recorded_norm-radius,100), "bracket_initial": trace["initial_bracket"], "bracket_final": trace["bracket"], "bracket_endpoint_evaluations": trace["bracket_evaluations"], "bisection_evaluation_count": len(trace["bisection_evaluations"]), "recorded_gap_bits": trace["gap"], "recorded_tolerance_bits": trace["tolerance"]},
  "unreached": data["post_refusal_fields"],
  "optional_correction": "not computed"
}
OUT.write_text(json.dumps(out, indent=2) + "\n")
print(json.dumps({"output": str(OUT), "free_violations": out["recorded_face_stationarity"]["free_violations"], "rounded_ball_feasible": out["fixed_lambda_reference"]["rounded_ball_feasible"]}))
