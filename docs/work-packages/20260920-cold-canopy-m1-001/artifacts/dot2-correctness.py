#!/usr/bin/env python3
"""Independent reconstruction of M1-DOT2-RESIDUAL-PROTOTYPE-01.

This calculator does not import the author's Dot2 primitive or prototype.  It
reuses only the previously accepted captured arrays and Jacobi factor replay.
"""
from __future__ import annotations

import hashlib
import json
import math
import runpy
import struct
import sys
from decimal import Decimal, localcontext
from fractions import Fraction
from pathlib import Path

N = 21
SPLITTER = 2.0**27 + 1.0
MIN_NORMAL = sys.float_info.min
HERE = Path(__file__).resolve().parent
CASES = (
    ("ordinary_positive", "finite-precision-observed-capture.json", "ec92859cf5d80eb7640e3d01861dc30f61288b49949b5977fb15f192ccf1a642"),
    ("historical_near_root", "face-pivot-observed-capture.json", "57c26d5b9215f9e22cacf2958afb60891acc54c46c540e3dcf0e748dff585a8c"),
)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def decode(raw: str) -> float:
    return struct.unpack(">d", bytes.fromhex(raw))[0]


def bits(value: float) -> str:
    return struct.pack(">d", float(value)).hex()


def vector(values):
    return [decode(value) for value in values]


def frac(value: float) -> Fraction:
    return Fraction.from_float(value)


def admitted(value: float) -> bool:
    return math.isfinite(value) and (value == 0.0 or abs(value) >= MIN_NORMAL)


def guard(value: float) -> float:
    if not admitted(value):
        raise ArithmeticError(f"non-admitted intermediate {bits(value)}")
    return value


def add(left: float, right: float) -> float:
    return guard(left + right)


def sub(left: float, right: float) -> float:
    return guard(left - right)


def mul(left: float, right: float) -> float:
    value = guard(left * right)
    if left != 0.0 and right != 0.0 and value == 0.0:
        raise ArithmeticError("nonzero product underflow")
    return value


def two_sum(left: float, right: float):
    total = add(left, right)
    z = sub(total, left)
    error = add(sub(left, sub(total, z)), sub(right, z))
    assert frac(left) + frac(right) == frac(total) + frac(error)
    return total, error


def split(value: float):
    c = mul(SPLITTER, value)
    high = sub(c, sub(c, value))
    low = sub(value, high)
    assert frac(value) == frac(high) + frac(low)
    return high, low


def two_product(left: float, right: float):
    product = mul(left, right)
    lh, ll = split(left)
    rh, rl = split(right)
    remainder = sub(sub(sub(product, mul(lh, rh)), mul(ll, rh)), mul(lh, rl))
    error = sub(mul(ll, rl), remainder)
    assert frac(left) * frac(right) == frac(product) + frac(error)
    return product, error


def dot2(x, y):
    assert len(x) == len(y) == 22
    assert all(admitted(value) for value in x + y)
    p, s = two_product(x[0], y[0])
    for index in range(1, len(x)):
        high, low = two_product(x[index], y[index])
        p, carry = two_sum(p, high)
        s = add(s, add(carry, low))
    return add(p, s)


def serial_residual(a, f, p):
    values = []
    for row in range(N):
        total = 0.0
        for col in range(N):
            total = total + a[row][col] * p[col]
        values.append(f[row] + total)
    return values


def decimal_residual(a, f, p):
    with localcontext() as context:
        context.prec = 200
        values = []
        for row in range(N):
            total = Decimal(0)
            for col in range(N):
                total += Decimal.from_float(a[row][col]) * Decimal.from_float(p[col])
            values.append(float(Decimal.from_float(f[row]) + total))
        return values


def exact_residual(a, f, p):
    return [frac(f[row]) + sum(frac(a[row][col]) * frac(p[col]) for col in range(N)) for row in range(N)]


def ordered_integer(value: float) -> int:
    raw = int(bits(value), 16)
    return (~raw & ((1 << 64) - 1)) if raw >> 63 else raw | (1 << 63)


def residual_record(value: float, exact: Fraction):
    rounded = float(exact)
    return {
        "bits": bits(value),
        "exact_fraction": str(exact),
        "signed_error_fraction": str(frac(value) - exact),
        "correctly_rounded_bits": bits(rounded),
        "signed_ulp_distance": ordered_integer(value) - ordered_integer(rounded),
        "matches_correct_rounding": bits(value) == bits(rounded),
    }


def up_mul(left: float, right: float) -> float:
    guard(left); guard(right)
    if left == 0.0 or right == 0.0:
        return 0.0
    value = mul(left, right)
    stepped = math.nextafter(value, math.inf)
    return guard(stepped)


def up_add(left: float, right: float) -> float:
    guard(left); guard(right)
    if right == 0.0:
        return left
    value = add(left, right)
    stepped = math.nextafter(value, math.inf)
    return guard(stepped)


def assess(capture, op, a, f, p, lam):
    residual = []
    for row in range(N):
        total = 0.0
        for col in range(N):
            total = add(total, mul(a[row][col], p[col]))
        residual.append(add(f[row], total))
    h = []
    for col in range(N):
        total = 0.0
        for row in range(N):
            total = add(total, mul(a[row][col], residual[row]))
        h.append(add(total, mul(lam, p[col])))
    gamma = math.nextafter(87.0 / float(2**53 - 87), math.inf)
    cbar, tau = [], []
    for coordinate in range(N):
        outer = 0.0
        for row in range(N):
            inner = 0.0
            for col in range(N):
                inner = up_add(inner, up_mul(abs(a[row][col]), abs(p[col])))
            magnitude = up_add(abs(f[row]), inner)
            outer = up_add(outer, up_mul(abs(a[row][coordinate]), magnitude))
        scale = up_add(outer, up_mul(abs(lam), abs(p[coordinate])))
        cbar.append(scale)
        tau.append(up_mul(gamma, scale))
    base, scales = vector(capture["base"]), vector(capture["scales"])
    lower_raw, upper_raw = vector(capture["lower"]), vector(capture["upper"])
    lower = [(lower_raw[i] - base[i]) / scales[i] for i in range(N)]
    upper = [(upper_raw[i] - base[i]) / scales[i] for i in range(N)]
    classes = []
    for i in range(N):
        if lower[i] == upper[i]:
            status = "fixed"
        elif op["lower_before"][i]:
            status = "release_lower" if h[i] < -tau[i] else "retain_lower"
        elif op["upper_before"][i]:
            status = "release_upper" if h[i] > tau[i] else "retain_upper"
        elif op["free_before"][i]:
            status = "free_pass" if abs(h[i]) <= tau[i] else "free_refuse"
        else:
            raise ArithmeticError("unclassified coordinate")
        classes.append(status)
    norm2 = 0.0
    for value in p:
        norm2 = norm2 + value * value
    active = [i for i in range(N) if not op["free_before"][i]]
    active_preserved = all(bits(p[i]) == op["p_candidate"][i] for i in active)
    active_bounds = all(
        (not op["lower_before"][i] or p[i] == lower[i])
        and (not op["upper_before"][i] or p[i] == upper[i]) for i in active
    )
    box = all(lower[i] <= p[i] <= upper[i] for i in range(N))
    ball = norm2 <= decode(op["radius"]) ** 2
    kkt = not any(value in ("release_lower", "release_upper", "free_refuse") for value in classes)
    complete = box and ball and kkt and active_preserved and active_bounds
    objective_sum = 0.0
    for value in residual:
        objective_sum += value * value
    return {
        "coordinates": [
            {"coordinate": i, "p_bits": bits(p[i]), "hhat_bits": bits(h[i]),
             "cbar_bits": bits(cbar[i]), "tau_bits": bits(tau[i]), "classification": classes[i]}
            for i in range(N)
        ],
        "box_feasible": box,
        "box_violations": [i for i in range(N) if not lower[i] <= p[i] <= upper[i]],
        "ball_feasible": ball,
        "active_values_preserved": active_preserved,
        "active_values_equal_declared_bounds": active_bounds,
        "bvls02_pass": kkt,
        "complete_acceptance": complete,
        "norm": repr(math.sqrt(norm2)),
        "residual_inf": repr(max(abs(value) for value in residual)),
        "objective": repr(0.5 * objective_sum),
    }


def named_case(document, name):
    return next(case for case in document["cases"] if case["case"] == name)


def main():
    author_path = HERE / "dot2-results.json"
    assert sha(author_path) == "fd58b06dd50b2312e791f04ba8a906a09d5ffdf863fb261e80c7b1d28957f32b"
    author = json.loads(author_path.read_text())
    accepted = json.loads((HERE / "face-accuracy-author-results.json").read_text())
    replay = runpy.run_path(str(HERE / "face-pivot-correctness-reconstruct.py"))
    cases = []
    for name, filename, expected_hash in CASES:
        capture_path = HERE / filename
        assert sha(capture_path) == expected_hash
        capture = json.loads(capture_path.read_text())
        op = capture["operations"][1]
        a = [vector(row) for row in capture["weighted_matrix"]]
        f, initial, lam = vector(capture["weighted_residual"]), vector(op["p_candidate"]), decode(op["lambda"])
        free = list(op["free_ids"])
        active = [i for i in range(N) if i not in free]
        face_a = [[a[row][i] for i in free] for row in range(N)]
        face_f = []
        for row in range(N):
            total = f[row]
            for i in active:
                total = total + a[row][i] * initial[i]
            face_f.append(total)
        factor = replay["jacobi_factor"](face_a, free, 0)
        candidate = replay["lambda_step"](factor, face_f, lam)
        assert [bits(value) for value in candidate] == [op["p_candidate"][i] for i in free]
        first_delta = replay["lambda_step"](factor, serial_residual(a, f, initial), lam)
        first_p = initial[:]
        for i, value in zip(free, first_delta, strict=True):
            first_p[i] += value
        old_case, author_case = named_case(accepted, name), named_case(author, name)
        accepted_first = old_case["corrections"][0]
        accepted_first_p = [row["p_bits"] for row in accepted_first["assessment"]["coordinates"]]
        assert [bits(value) for value in first_delta] == accepted_first["delta_free_bits"]
        assert [bits(value) for value in first_p] == accepted_first_p
        dot = [dot2([f[row], *a[row]], [1.0, *first_p]) for row in range(N)]
        exact = exact_residual(a, f, first_p)
        historical = decimal_residual(a, f, first_p)
        second_delta = replay["lambda_step"](factor, dot, lam)
        final_p = first_p[:]
        for i, value in zip(free, second_delta, strict=True):
            final_p[i] += value
        reconstructed = assess(capture, op, a, f, final_p, lam)
        residual_rows = []
        author_rows = author_case["dot2_rows"]
        for row in range(N):
            dot_record = residual_record(dot[row], exact[row])
            historical_record = residual_record(historical[row], exact[row])
            residual_rows.append({
                "row": row, "dot2": dot_record, "historical_decimal200": historical_record,
                "author_dot2_bits_match": dot_record["bits"] == author_rows[row]["bits"],
                "author_historical_bits_match": historical_record["bits"] == author_rows[row]["historical_decimal200_bits"],
                "author_exact_fraction_match": str(exact[row]) == author_rows[row]["exact_fraction"],
                "author_dot2_error_and_ulp_match": (
                    dot_record["signed_error_fraction"] == author_rows[row]["dot2"]["signed_error_fraction"]
                    and dot_record["signed_ulp_distance"] == author_rows[row]["dot2"]["signed_ulp_distance_from_exact_rounding"]
                ),
            })
        author_assessment = author_case["second_correction"]["assessment"]
        assessment_bits_match = all(
            left["p_bits"] == right["p_bits"] and left["hhat_bits"] == right["hhat_bits"]
            and left["cbar_bits"] == right["cbar_bits"] and left["tau_bits"] == right["tau_bits"]
            and left["classification"] == right["classification"]
            for left, right in zip(reconstructed["coordinates"], author_assessment["coordinates"], strict=True)
        )
        cases.append({
            "case": name,
            "factor_replay": {"candidate_bits_match": True, "sweeps": factor.sweeps, "rotations": factor.rotations},
            "first_correction": {
                "accepted_delta_bits_match": [bits(value) for value in first_delta] == author_case["first_correction"]["delta_free_bits"],
                "accepted_p_bits_match": [bits(value) for value in first_p] == author_case["first_correction"]["p_bits"],
            },
            "residual_rows": residual_rows,
            "residual_summary": {
                "all_author_bits_errors_ulps_match": all(
                    row["author_dot2_bits_match"] and row["author_historical_bits_match"]
                    and row["author_exact_fraction_match"] and row["author_dot2_error_and_ulp_match"] for row in residual_rows
                ),
                "dot2_correctly_rounded_rows": sum(row["dot2"]["matches_correct_rounding"] for row in residual_rows),
                "dot2_max_abs_ulp_distance": max(abs(row["dot2"]["signed_ulp_distance"]) for row in residual_rows),
                "dot2_non_correctly_rounded_rows": [row["row"] for row in residual_rows if not row["dot2"]["matches_correct_rounding"]],
                "historical_correctly_rounded_rows": sum(row["historical_decimal200"]["matches_correct_rounding"] for row in residual_rows),
                "dot2_max_abs_signed_error_decimal": repr(float(max(abs(frac(dot[row]) - exact[row]) for row in range(N)))),
                "exact_zero_rows_from_fraction_arithmetic": [row for row in range(N) if exact[row] == 0],
                "exact_zero_row_returned_bits": [
                    {"row": row, "dot2_bits": bits(dot[row]), "historical_decimal200_bits": bits(historical[row])}
                    for row in range(N) if exact[row] == 0
                ],
            },
            "second_correction": {
                "delta_bits_match": [bits(value) for value in second_delta] == author_case["second_correction"]["delta_free_bits"],
                "p_bits_match": [bits(value) for value in final_p] == author_case["second_correction"]["p_bits"],
                "all_p_h_cbar_tau_class_bits_match": assessment_bits_match,
                "assessment": reconstructed,
                "author_acceptance_agrees": reconstructed["complete_acceptance"] == author_assessment["prototype_complete_acceptance"],
            },
        })
    output = {
        "schema": "cold-canopy-m1-dot2-correctness/v1",
        "reviewer": "/root/accuracy_correctness (new independent session)",
        "evidence_class": "Ran: independent ORO EFT/Dot2, exact-Fraction residual, Decimal-200 residual, accepted-factor replay, and independent all-coordinate BVLS-02/enclosure/feasibility reconstruction; no Rust or physics",
        "dependencies": {
            "author_results_sha256": sha(author_path),
            "ordinary_capture_sha256": CASES[0][2],
            "historical_capture_sha256": CASES[1][2],
            "accepted_factor_replay_sha256": sha(HERE / "face-pivot-correctness-reconstruct.py"),
            "accepted_accuracy_results_sha256": sha(HERE / "face-accuracy-author-results.json"),
            "review_source_sha256": sha(Path(__file__)),
        },
        "cases": cases,
        "findings": [],
        "fix_verification": {
            "pre_run_import_failure": "PASS: rejected source is preserved; reviewed source imports and ran at its released hash.",
            "guard_and_acceptance": "PASS: all retained EFT operands/intermediates admit, and complete acceptance includes unchanged BVLS-02, box/ball, normal-domain, active-bit preservation, and active-bound equality.",
            "reporting": "PASS: historical grouping/replay, exact errors, ULP distances, cost partitions, and dependency identities are present and independently reproduced."
        },
        "residual_risk": [
            "Dot2 misses correct rounding on 4/21 and 7/21 rows, by as much as 3 and 17 representable steps; this prototype establishes no exact-rounding or general boundary-sign guarantee.",
            "Only two fixed faces with lambda=0 and interior trust balls were evaluated. Nonzero lambda, face changes, arbitrary conditioning, infeasible internal-scratch policy, nonlinear acceptance, and physical behavior remain unestablished.",
            "The calculation is offline Python evidence and does not establish Rust/runtime cost, serialization, or integration behavior."
        ],
        "missing_tests": [
            "No additional test is missing for the authorized two-case offline prototype. Any runtime amendment would require a separately authorized specification and production validation, including policy for the infeasible first-correction scratch point."
        ],
        "verdict": "PASS: both fixed two-case prototype results independently reproduce and satisfy unchanged complete acceptance; Dot2 is not correctly rounded on every row and no general/runtime claim follows.",
    }
    (HERE / "dot2-correctness.json").write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
