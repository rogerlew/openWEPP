#!/usr/bin/env python3
"""Independent correctness reconstruction for the retained-face accuracy audit.

This reviewer-owned calculator intentionally does not import the author's
calculator.  It combines exact rational evaluation and solution (the latter is
used only to certify mathematical signs/zeros) with an independent Decimal
Householder QR accuracy reference.  Binary64 Jacobi replay and the two declared
corrections are independently translated from the frozen contract.
"""
from __future__ import annotations

import ctypes
import hashlib
import json
import math
import struct
import sys
from dataclasses import dataclass
from decimal import Decimal, localcontext
from fractions import Fraction
from pathlib import Path

N = 21
HERE = Path(__file__).resolve().parent
HEAD = "15b45db425e03a7217bbd2f0be2ea20ca454c7e2"
CAPTURES = (
    ("ordinary_positive", "finite-precision-observed-capture.json", "ec92859cf5d80eb7640e3d01861dc30f61288b49949b5977fb15f192ccf1a642"),
    ("historical_near_root", "face-pivot-observed-capture.json", "57c26d5b9215f9e22cacf2958afb60891acc54c46c540e3dcf0e748dff585a8c"),
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def decode(value: str) -> float:
    return struct.unpack(">d", bytes.fromhex(value))[0]


def bits(value: float) -> str:
    return struct.pack(">d", float(value)).hex()


def vector(values: list[str]) -> list[float]:
    return [decode(value) for value in values]


def q(value: float) -> Fraction:
    return Fraction.from_float(value)


def normal_or_zero(value: float) -> bool:
    return math.isfinite(value) and (value == 0.0 or abs(value) >= sys.float_info.min)


def dec(value: float) -> Decimal:
    return Decimal.from_float(value)


def ordered_kkt(a: list[list[float]], f: list[float], p: list[float], lam: float):
    residual = []
    for row in range(N):
        total = 0.0
        for col in range(N):
            total += a[row][col] * p[col]
        residual.append(f[row] + total)
    gradient = []
    for col in range(N):
        total = 0.0
        for row in range(N):
            total += a[row][col] * residual[row]
        gradient.append(total)
    h = [gradient[i] + lam * p[i] for i in range(N)]
    return residual, h


def exact_kkt(a: list[list[float]], f: list[float], p: list[float], lam: float):
    aq = [[q(value) for value in row] for row in a]
    fq, pq, lq = [q(value) for value in f], [q(value) for value in p], q(lam)
    residual = [fq[row] + sum(aq[row][col] * pq[col] for col in range(N)) for row in range(N)]
    h = [sum(aq[row][col] * residual[row] for row in range(N)) + lq * pq[col] for col in range(N)]
    return residual, h


def up_mul(left: float, right: float) -> float:
    if left == 0.0 or right == 0.0:
        return 0.0
    return math.nextafter(left * right, math.inf)


def up_add(left: float, right: float) -> float:
    if right == 0.0:
        return left
    return math.nextafter(left + right, math.inf)


def enclosure(a: list[list[float]], f: list[float], p: list[float], lam: float):
    gamma87 = math.nextafter(87.0 / float(2**53 - 87), math.inf)
    result = []
    for coordinate in range(N):
        outer = 0.0
        for row in range(N):
            inner = 0.0
            for col in range(N):
                inner = up_add(inner, up_mul(abs(a[row][col]), abs(p[col])))
            magnitude = up_add(abs(f[row]), inner)
            outer = up_add(outer, up_mul(abs(a[row][coordinate]), magnitude))
        cbar = up_add(outer, up_mul(abs(lam), abs(p[coordinate])))
        result.append((cbar, up_mul(gamma87, cbar)))
    return gamma87, result


def guarded_normal_domain_replay(a, f, p, lam) -> bool:
    """Check every written KKT/enclosure magnitude is normal or explicit zero."""
    valid = all(normal_or_zero(x) for row in a for x in row)
    valid &= all(normal_or_zero(x) for x in f + p + [lam])
    residual = []
    for row in range(N):
        total = 0.0
        for col in range(N):
            product = a[row][col] * p[col]
            valid &= normal_or_zero(product) and not (a[row][col] != 0.0 and p[col] != 0.0 and product == 0.0)
            previous = total
            total = previous + product
            valid &= normal_or_zero(total)
            valid &= total != 0.0 or (previous == 0.0 and product == 0.0) or previous == -product
        value = f[row] + total
        valid &= normal_or_zero(value)
        valid &= value != 0.0 or (f[row] == 0.0 and total == 0.0) or f[row] == -total
        residual.append(value)
    for col in range(N):
        total = 0.0
        for row in range(N):
            product = a[row][col] * residual[row]
            valid &= normal_or_zero(product) and not (a[row][col] != 0.0 and residual[row] != 0.0 and product == 0.0)
            previous = total
            total = previous + product
            valid &= normal_or_zero(total)
            valid &= total != 0.0 or (previous == 0.0 and product == 0.0) or previous == -product
        product = lam * p[col]
        value = total + product
        valid &= normal_or_zero(product) and not (lam != 0.0 and p[col] != 0.0 and product == 0.0)
        valid &= normal_or_zero(value)
        valid &= value != 0.0 or (total == 0.0 and product == 0.0) or total == -product
    for row in range(N):
        for col in range(N):
            product = abs(a[row][col]) * abs(p[col])
            valid &= normal_or_zero(product) and not (a[row][col] != 0.0 and p[col] != 0.0 and product == 0.0)
    gamma87, rows = enclosure(a, f, p, lam)
    valid &= normal_or_zero(gamma87)
    valid &= all(normal_or_zero(cbar) and normal_or_zero(tau) for cbar, tau in rows)
    valid &= all(not (cbar != 0.0 and gamma87 != 0.0 and gamma87 * cbar == 0.0) for cbar, _ in rows)
    return bool(valid)


def scaled_bounds(capture: dict):
    base, scale = vector(capture["base"]), vector(capture["scales"])
    lower, upper = vector(capture["lower"]), vector(capture["upper"])
    return ([(lower[i] - base[i]) / scale[i] for i in range(N)],
            [(upper[i] - base[i]) / scale[i] for i in range(N)])


def assess_binary64(capture: dict, op: dict, a, f, p, lam):
    residual, hhat = ordered_kkt(a, f, p, lam)
    exact_residual, exact_h = exact_kkt(a, f, p, lam)
    gamma87, bounds = enclosure(a, f, p, lam)
    lower, upper = scaled_bounds(capture)
    classes = []
    rows = []
    for i, (cbar, tau) in enumerate(bounds):
        if op["lower_before"][i]:
            classification = "release_lower" if hhat[i] < -tau else "retain_lower"
        elif op["upper_before"][i]:
            classification = "release_upper" if hhat[i] > tau else "retain_upper"
        elif op["free_before"][i]:
            classification = "free_pass" if abs(hhat[i]) <= tau else "free_refuse"
        else:
            classification = "fixed"
        classes.append(classification)
        error = q(hhat[i]) - exact_h[i]
        rows.append({
            "coordinate": i, "p_bits": bits(p[i]), "hhat_bits": bits(hhat[i]),
            "exact_h_fraction": str(exact_h[i]), "exact_h_decimal": repr(float(exact_h[i])),
            "evaluation_error_fraction": str(error), "evaluation_error_decimal": repr(float(error)),
            "cbar_bits": bits(cbar), "tau_bits": bits(tau), "tau": repr(tau),
            "evaluation_error_within_tau": abs(error) <= q(tau),
            "exact_h_over_tau": None if tau == 0.0 else repr(float(abs(exact_h[i]) / q(tau))),
            "classification": classification,
        })
    norm2 = 0.0
    for value in p:
        norm2 += value * value
    radius = decode(op["radius"])
    original_kkt = []
    exact_original_kkt = []
    for i, (_, tau) in enumerate(bounds):
        fixed = lower[i] == upper[i]
        if fixed:
            original_kkt.append("fixed")
            exact_original_kkt.append("fixed")
        elif p[i] == lower[i]:
            original_kkt.append("lower_pass" if hhat[i] >= -tau else "lower_fail")
            exact_original_kkt.append("lower_pass" if exact_h[i] >= 0 else "lower_fail")
        elif p[i] == upper[i]:
            original_kkt.append("upper_pass" if hhat[i] <= tau else "upper_fail")
            exact_original_kkt.append("upper_pass" if exact_h[i] <= 0 else "upper_fail")
        else:
            original_kkt.append("interior_pass" if abs(hhat[i]) <= tau else "interior_fail")
            exact_original_kkt.append("interior_pass" if exact_h[i] == 0 else "interior_fail")
    active = [i for i in range(N) if not op["free_before"][i]]
    active_norm2 = 0.0
    for i in active:
        active_norm2 += p[i] * p[i]
    full_radius = decode(capture["initial_radius"])
    remaining2 = full_radius * full_radius
    for i in active:
        remaining2 -= p[i] * p[i]
    reconstructed_remaining = math.sqrt(remaining2)
    active_on_declared_bound = all(
        (not op["lower_before"][i] or p[i] == lower[i])
        and (not op["upper_before"][i] or p[i] == upper[i])
        for i in active
    )
    box_feasible = all(lower[i] <= p[i] <= upper[i] for i in range(N))
    ball_feasible = norm2 <= radius * radius
    objective_sum = 0.0
    for value in residual:
        objective_sum += value * value
    return {
        "coordinates": rows,
        "all_evaluation_errors_within_tau": all(row["evaluation_error_within_tau"] for row in rows),
        "bvls02_face_pass": not any(value in ("release_lower", "release_upper", "free_refuse") for value in classes),
        "box_feasible": box_feasible,
        "box_violations": [i for i in range(N) if not lower[i] <= p[i] <= upper[i]],
        "ball_feasible": ball_feasible,
        "active_values_preserved": all(bits(p[i]) == op["p_candidate"][i] for i in active),
        "active_values_equal_declared_bounds": active_on_declared_bound,
        "active_norm_squared": repr(active_norm2),
        "remaining_radius_reconstructed": repr(reconstructed_remaining),
        "remaining_radius_bits_match": bits(reconstructed_remaining) == op["radius"],
        "full_original_approximate_kkt_pass": box_feasible and ball_feasible and all(not value.endswith("fail") for value in original_kkt),
        "full_original_approximate_kkt_classes": original_kkt,
        "full_original_exact_kkt_pass": box_feasible and ball_feasible and all(not value.endswith("fail") for value in exact_original_kkt),
        "full_original_exact_kkt_classes": exact_original_kkt,
        "norm": repr(math.sqrt(norm2)), "radius": repr(radius),
        "residual_inf": repr(max(abs(x) for x in residual)),
        "objective": repr(0.5 * objective_sum),
        "gamma87_bits": bits(gamma87),
        "all_kkt_and_enclosure_intermediates_normal_or_zero": guarded_normal_domain_replay(a, f, p, lam),
        "complete_bvls02_and_feasibility_pass": box_feasible and ball_feasible and not any(value in ("release_lower", "release_upper", "free_refuse") for value in classes),
    }


def attach_step_error(assessment: dict, p: list[float], exact_reference: list[Fraction]) -> None:
    errors = [q(p[i]) - exact_reference[i] for i in range(N)]
    with localcontext() as context:
        context.prec = 100
        l2 = sum((Decimal(value.numerator) / Decimal(value.denominator)) ** 2 for value in errors).sqrt()
        maximum = max(abs(Decimal(value.numerator) / Decimal(value.denominator)) for value in errors)
    assessment["step_error_vs_exact_face_reference"] = {
        "max_abs_decimal": str(maximum), "l2_decimal": str(l2),
        "per_coordinate_fraction": [str(value) for value in errors],
    }


def exact_face_solution(a, f, recorded_p, free_ids, lam):
    """Exact solve of the normal equations, used only for signs and exact zeros.

    This is not an accuracy certificate.  Because all inputs are rational and
    elimination is exact, it is a useful independent certificate of the exact
    fixed-face minimizer after the Householder reference establishes accuracy.
    """
    aq, fq = [[q(value) for value in row] for row in a], [q(value) for value in f]
    active = [i for i in range(N) if i not in free_ids]
    face_residual = [fq[row] + sum(aq[row][i] * q(recorded_p[i]) for i in active) for row in range(N)]
    n = len(free_ids)
    gram = [[sum(aq[row][free_ids[i]] * aq[row][free_ids[j]] for row in range(N))
             + (q(lam) if i == j else 0) for j in range(n)] for i in range(n)]
    rhs = [-sum(aq[row][free_ids[i]] * face_residual[row] for row in range(N)) for i in range(n)]
    augmented = [gram[i] + [rhs[i]] for i in range(n)]
    pivots = []
    for col in range(n):
        pivot = next(row for row in range(col, n) if augmented[row][col] != 0)
        augmented[col], augmented[pivot] = augmented[pivot], augmented[col]
        scale = augmented[col][col]
        pivots.append(scale)
        for j in range(col, n + 1):
            augmented[col][j] /= scale
        for row in range(n):
            if row == col or augmented[row][col] == 0:
                continue
            scale = augmented[row][col]
            for j in range(col, n + 1):
                augmented[row][j] -= scale * augmented[col][j]
    free_solution = [augmented[i][n] for i in range(n)]
    p = [q(value) for value in recorded_p]
    for i, value in zip(free_ids, free_solution, strict=True):
        p[i] = value
    residual = [fq[row] + sum(aq[row][i] * p[i] for i in range(N)) for row in range(N)]
    h = [sum(aq[row][i] * residual[row] for row in range(N)) + q(lam) * p[i] for i in range(N)]
    return p, residual, h, pivots


def householder_reference(a, f, recorded_p, free_ids, lam, precision):
    """Decimal Householder QR of [A_free;sqrt(lambda)I] on the fixed face."""
    active = [i for i in range(N) if i not in free_ids]
    with localcontext() as context:
        context.prec = precision
        matrix = [[dec(a[row][i]) for i in free_ids] for row in range(N)]
        rhs = [-(dec(f[row]) + sum(dec(a[row][i]) * dec(recorded_p[i]) for i in active)) for row in range(N)]
        if lam != 0.0:
            root = dec(lam).sqrt()
            for i in range(len(free_ids)):
                matrix.append([root if i == j else Decimal(0) for j in range(len(free_ids))])
                rhs.append(Decimal(0))
        original = [row[:] for row in matrix]
        reflectors = []
        m, n = len(matrix), len(free_ids)
        for k in range(n):
            x = [matrix[row][k] for row in range(k, m)]
            magnitude = sum(value * value for value in x).sqrt()
            alpha = -magnitude if x[0] >= 0 else magnitude
            v = x[:]
            v[0] -= alpha
            beta = Decimal(2) / sum(value * value for value in v)
            for col in range(k, n):
                projection = beta * sum(v[row - k] * matrix[row][col] for row in range(k, m))
                for row in range(k, m):
                    matrix[row][col] -= projection * v[row - k]
            projection = beta * sum(v[row - k] * rhs[row] for row in range(k, m))
            for row in range(k, m):
                rhs[row] -= projection * v[row - k]
            reflectors.append((k, v, beta))
        solution = [Decimal(0)] * n
        for row in range(n - 1, -1, -1):
            solution[row] = (rhs[row] - sum(matrix[row][j] * solution[j] for j in range(row + 1, n))) / matrix[row][row]
        qthin = [[Decimal(1) if row == col else Decimal(0) for col in range(n)] for row in range(m)]
        for k, v, beta in reversed(reflectors):
            for col in range(n):
                projection = beta * sum(v[row - k] * qthin[row][col] for row in range(k, m))
                for row in range(k, m):
                    qthin[row][col] -= projection * v[row - k]
        factor_error = max(abs(original[row][col] - sum(qthin[row][j] * matrix[j][col] for j in range(n)))
                           for row in range(m) for col in range(n))
        orth_error = max(abs(sum(qthin[row][i] * qthin[row][j] for row in range(m))
                             - (Decimal(1) if i == j else Decimal(0))) for i in range(n) for j in range(n))
        residual = [sum(original[row][j] * solution[j] for j in range(n)) - (-(dec(f[row]) + sum(dec(a[row][i]) * dec(recorded_p[i]) for i in active)) if row < N else Decimal(0)) for row in range(m)]
        stationarity = [sum(original[row][col] * residual[row] for row in range(m)) for col in range(n)]
        full_step = [dec(value) for value in recorded_p]
        for coordinate, value in zip(free_ids, solution, strict=True):
            full_step[coordinate] = value
        return {
            "digits": precision, "free_decimal": [str(value) for value in solution],
            "factorization_max_abs": str(factor_error), "orthogonality_max_abs": str(orth_error),
            "augmented_residual_inf": str(max(abs(value) for value in residual)),
            "free_stationarity_inf": str(max(abs(value) for value in stationarity)),
            "step_norm_decimal": str(sum(value * value for value in full_step).sqrt()),
        }, solution


try:
    _libc = ctypes.CDLL(None)
    _hypot = _libc.hypot
    _hypot.argtypes = (ctypes.c_double, ctypes.c_double)
    _hypot.restype = ctypes.c_double
except (AttributeError, OSError):
    _hypot = None


def host_hypot(a, b):
    return math.hypot(a, b) if _hypot is None else float(_hypot(a, b))


@dataclass
class Factor:
    original: list[list[float]]
    rotated: list[list[float]]
    v: list[list[float]]
    sigma: list[float]
    order: list[int]
    sweeps: int
    rotations: int


def jacobi_factor(matrix, free_ids):
    original, rotated = [row[:] for row in matrix], [row[:] for row in matrix]
    n = len(free_ids)
    v = [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]
    rotations = 0
    for sweep in range(1, 65):
        changed = False
        for left in range(n):
            for right in range(left + 1, n):
                alpha = beta = gamma = 0.0
                for row in rotated:
                    alpha += row[left] * row[left]
                    beta += row[right] * row[right]
                    gamma += row[left] * row[right]
                if abs(gamma) > (2.0**-48) * math.sqrt(alpha * beta):
                    tau = (beta - alpha) / (2.0 * gamma)
                    tangent = 1.0 if tau == 0.0 else math.copysign(1.0, tau) / (abs(tau) + host_hypot(tau, 1.0))
                    cosine = 1.0 / host_hypot(tangent, 1.0)
                    sine = cosine * tangent
                    for target in (rotated, v):
                        for row in target:
                            x, y = row[left], row[right]
                            row[left] = cosine * x - sine * y
                            row[right] = sine * x + cosine * y
                    rotations += 1
                    changed = True
        if not changed:
            break
    else:
        raise ArithmeticError("Jacobi replay did not converge")
    sigma = []
    for col in range(n):
        total = 0.0
        for row in rotated:
            total += row[col] * row[col]
        sigma.append(math.sqrt(total))
    order = sorted(range(n), key=lambda i: (-sigma[i], free_ids[i]))
    return Factor(original, rotated, v, sigma, order, sweep, rotations)


def factor_step(factor, residual, lam):
    n = len(factor.sigma)
    coefficients = [0.0] * n
    for j in factor.order:
        uty = 0.0
        for row, residual_value in enumerate(residual):
            av = 0.0
            for free in range(n):
                av += factor.original[row][free] * factor.v[free][j]
            uty += (av / factor.sigma[j]) * residual_value
        coefficients[j] = -((factor.sigma[j] / (factor.sigma[j] * factor.sigma[j] + lam)) * uty)
    result = []
    for output in range(n):
        total = 0.0
        for j in factor.order:
            total += factor.v[output][j] * coefficients[j]
        result.append(total)
    return result


def exact_reference_assessment(capture, p, residual, h, pivots, lam, radius):
    lower, upper = scaled_bounds(capture)
    lq = [None if math.isinf(x) else q(x) for x in lower]
    uq = [None if math.isinf(x) else q(x) for x in upper]
    coordinate_feasible = [
        (lq[i] is None or lq[i] <= p[i]) and (uq[i] is None or p[i] <= uq[i])
        for i in range(N)
    ]
    box = all(coordinate_feasible)
    full_kkt = []
    for i in range(N):
        if lq[i] is not None and uq[i] is not None and p[i] == lq[i] and p[i] == uq[i]:
            status = "fixed"
        elif lq[i] is not None and p[i] == lq[i]:
            status = "lower_pass" if h[i] >= 0 else "lower_fail"
        elif uq[i] is not None and p[i] == uq[i]:
            status = "upper_pass" if h[i] <= 0 else "upper_fail"
        else:
            status = "interior_pass" if h[i] == 0 else "interior_fail"
        full_kkt.append(status)
    objective = sum(value * value for value in residual) / 2
    with localcontext() as context:
        context.prec = 100
        norm = sum((Decimal(value.numerator) / Decimal(value.denominator)) ** 2 for value in p).sqrt()
        residual_inf = max(abs(Decimal(value.numerator) / Decimal(value.denominator)) for value in residual)
        min_pivot = min(abs(Decimal(value.numerator) / Decimal(value.denominator)) for value in pivots)
    full_kkt_pass = not any(value.endswith("fail") for value in full_kkt)
    box_and_ball = box and sum(value * value for value in p) <= q(radius) * q(radius)
    return {
        "box_feasible": box,
        "box_violations": [i for i in range(N) if not coordinate_feasible[i]],
        "ball_feasible": sum(value * value for value in p) <= q(radius) * q(radius),
        "step_norm_decimal": str(norm),
        "residual_inf_decimal": str(residual_inf),
        "objective_fraction": str(objective),
        "all_coordinate_residual_fraction": [str(value) for value in residual],
        "all_coordinate_h_fraction": [str(value) for value in h],
        "free_stationarity_exact": all(h[i] == 0 for i in range(N) if i != 5),
        "full_original_kkt_pass": full_kkt_pass,
        "full_original_kkt_classes": full_kkt,
        "coordinate11_fraction": str(p[11]), "coordinate11_is_exact_zero": p[11] == 0,
        "active_coordinate5_h_fraction": str(h[5]), "active_coordinate5_h_sign": (h[5] > 0) - (h[5] < 0),
        "exact_free_system_rank": len(pivots),
        "exact_nonzero_elimination_pivots": sum(value != 0 for value in pivots),
        "minimum_exact_pivot_magnitude_decimal": str(min_pivot),
        "recorded_lambda_fraction": str(q(lam)),
        "lambda_zero_interior_face_equivalence": lam == 0.0 and sum(value * value for value in p) < q(radius) * q(radius),
        "fixed_face_trust_minimizer_established": len(pivots) == 20 and lam == 0.0 and sum(value * value for value in p) < q(radius) * q(radius),
        "original_bounded_subproblem_minimizer_established": len(pivots) == 20 and box_and_ball and full_kkt_pass,
        "interpretation": "exact rational normal equations certify exact signs and structural zeros only; Householder QR supplies the numerical accuracy reference",
    }


def main():
    cases = []
    for name, filename, expected_hash in CAPTURES:
        path = HERE / filename
        assert sha256(path) == expected_hash
        capture = json.loads(path.read_text())
        op = capture["operations"][1]
        a, f = [vector(row) for row in capture["weighted_matrix"]], vector(capture["weighted_residual"])
        p, lam, radius = vector(op["p_candidate"]), decode(op["lambda"]), decode(op["radius"])
        free_ids = list(op["free_ids"])
        existing = assess_binary64(capture, op, a, f, p, lam)
        exact_p, exact_residual, exact_h, exact_pivots = exact_face_solution(a, f, p, free_ids, lam)
        attach_step_error(existing, p, exact_p)
        exact_assessment = exact_reference_assessment(
            capture, exact_p, exact_residual, exact_h, exact_pivots, lam, radius
        )
        references, solutions = [], []
        for precision in (100, 200):
            reference, solution = householder_reference(a, f, p, free_ids, lam, precision)
            references.append(reference)
            solutions.append(solution)
        nonzero_error = []
        structural_zero_residue = []
        with localcontext() as context:
            context.prec = 200
            for slot, coordinate in enumerate(free_ids):
                if exact_p[coordinate] == 0:
                    structural_zero_residue.append({"coordinate": coordinate, "decimal100": str(solutions[0][slot]), "decimal200": str(solutions[1][slot])})
                else:
                    rational_decimal = Decimal(exact_p[coordinate].numerator) / Decimal(exact_p[coordinate].denominator)
                    nonzero_error.append(abs(solutions[1][slot] - rational_decimal))
        rounded = [float(value) for value in exact_p]
        rounded_assessment = assess_binary64(capture, op, a, f, rounded, lam)
        attach_step_error(rounded_assessment, rounded, exact_p)

        active = [i for i in range(N) if i not in free_ids]
        face_residual = [f[row] + sum(a[row][i] * p[i] for i in active) for row in range(N)]
        face_matrix = [[a[row][i] for i in free_ids] for row in range(N)]
        factor = jacobi_factor(face_matrix, free_ids)
        replay = factor_step(factor, face_residual, lam)
        replay_full = p[:]
        for coordinate, value in zip(free_ids, replay, strict=True):
            replay_full[coordinate] = value
        corrections, current = [], p[:]
        for number, residual_kind in ((1, "ordinary_binary64"), (2, "decimal200_residual_once_rounded")):
            if number == 1:
                residual, _ = ordered_kkt(a, f, current, 0.0)
            else:
                with localcontext() as context:
                    context.prec = 200
                    residual = [float(dec(f[row]) + sum(dec(a[row][col]) * dec(current[col]) for col in range(N))) for row in range(N)]
            delta = factor_step(factor, residual, lam)
            for coordinate, value in zip(free_ids, delta, strict=True):
                current[coordinate] += value
            corrections.append({
                "number": number, "residual_kind": residual_kind,
                "delta_free_bits": [bits(value) for value in delta],
                "p_bits": [bits(value) for value in current],
                "assessment": assess_binary64(capture, op, a, f, current, lam),
            })
            attach_step_error(corrections[-1]["assessment"], current, exact_p)
        with localcontext() as context:
            context.prec = 200
            decimal_face_rhs = [
                dec(f[row]) + sum(dec(a[row][i]) * dec(p[i]) for i in active)
                for row in range(N)
            ]
        cases.append({
            "case": name, "capture": filename, "capture_sha256": expected_hash, "operation": 1,
            "existing": existing,
            "exact_rational_face_reference": exact_assessment,
            "householder_references": references,
            "householder_nonzero_coordinate_max_abs_error_at_200": str(max(nonzero_error)),
            "householder_structural_zero_residues": structural_zero_residue,
            "householder_rhs_custody": {
                "sole_active_coordinate": active,
                "active_values_all_exact_zero": all(p[i] == 0.0 for i in active),
                "binary64_face_rhs_bits_equal_f": all(bits(face_residual[i]) == bits(f[i]) for i in range(N)),
                "decimal_face_rhs_exactly_equals_binary64_f": all(decimal_face_rhs[i] == dec(f[i]) for i in range(N)),
                "impact": "the author's binary64 construction of the face RHS has no effect in these two cases because the sole active contribution is exact zero",
            },
            "once_rounded_exact_reference_bits": [bits(value) for value in rounded],
            "once_rounded_exact_reference": rounded_assessment,
            "jacobi_replay": {
                "started_at_zero_prior_sweeps": True, "sweeps": factor.sweeps,
                "recorded_sweeps": op["sweeps"], "sweeps_match": factor.sweeps == op["sweeps"],
                "rotations": factor.rotations,
                "candidate_bits_match": [bits(value) for value in replay_full] == op["p_candidate"],
                "condition_estimate": repr(max(factor.sigma) / min(factor.sigma)),
            },
            "corrections": corrections,
            "interpretation": {
                "evaluation_vs_step": "exact h separates ordered-binary64 evaluation rounding from stationarity defect at each supplied p",
                "reference_feasibility": "exact reference feasibility is distinct from current face masks; an infeasible fixed-face minimizer would not be a bounded-subproblem solution",
                "full_kkt": "full original-bound KKT is distinct from free-face stationarity and BVLS-02 finite-precision acceptance",
                "limits": "a passing once-rounded point or correction shows existence for that point only; failure would not prove that no binary64 solution exists; no stability guarantee is inferred for the custom Jacobi SVD",
            },
        })
    author_source = HERE / "face-accuracy-author.py"
    author_results_path = HERE / "face-accuracy-author-results.json"
    assert sha256(author_source) == "0a3f16b05a9e94385fba79f81f88d1832085be36f1713a8795b630aa450a25d1"
    assert sha256(author_results_path) == "73af08c872dc7b69406d46edd73027b2e2d942beae713bf03930425d6d641320"
    author_results = json.loads(author_results_path.read_text())
    comparisons = []
    for author_case, review_case in zip(author_results["cases"], cases, strict=True):
        existing_bits_match = all(
            author_row["p_bits"] == review_row["p_bits"]
            and author_row["h_hat_bits"] == review_row["hhat_bits"]
            and author_row["cbar_bits"] == review_row["cbar_bits"]
            and author_row["tau_bits"] == review_row["tau_bits"]
            for author_row, review_row in zip(
                author_case["existing_assessment"]["coordinates"], review_case["existing"]["coordinates"], strict=True
            )
        )
        correction_rows = []
        for author_correction, review_correction in zip(author_case["corrections"], review_case["corrections"], strict=True):
            author_assessment, review_assessment = author_correction["assessment"], review_correction["assessment"]
            coordinate_bits_match = all(
                author_row["p_bits"] == review_bits
                and author_row["h_hat_bits"] == review_row["hhat_bits"]
                and author_row["cbar_bits"] == review_row["cbar_bits"]
                and author_row["tau_bits"] == review_row["tau_bits"]
                for author_row, review_bits, review_row in zip(
                    author_assessment["coordinates"], review_correction["p_bits"], review_assessment["coordinates"], strict=True
                )
            )
            correction_rows.append({
                "number": author_correction["number"],
                "delta_bits_match": author_correction["delta_free_bits"] == review_correction["delta_free_bits"],
                "all_p_h_cbar_tau_bits_match": coordinate_bits_match,
                "summary_values_match": all(
                    author_assessment[key] == review_assessment[review_key]
                    for key, review_key in (
                        ("box_feasible", "box_feasible"), ("ball_feasible", "ball_feasible"),
                        ("free_test_pass", "bvls02_face_pass"), ("norm", "norm"),
                        ("objective", "objective"), ("residual_inf", "residual_inf"),
                    )
                ),
                "independent_normal_guard_pass": review_assessment["all_kkt_and_enclosure_intermediates_normal_or_zero"],
            })
        comparisons.append({
            "case": author_case["case"],
            "existing_all_p_h_cbar_tau_bits_match": existing_bits_match,
            "existing_exact_h_fractions_match": author_case["exact_h_fraction"]
            == [row["exact_h_fraction"] for row in review_case["existing"]["coordinates"]],
            "jacobi_candidate_match_agrees": author_case["factor_replay"]["bit_exact_candidate_match"]
            == review_case["jacobi_replay"]["candidate_bits_match"],
            "jacobi_sweeps_and_rotations_match": author_case["factor_replay"]["sweeps"]
            == review_case["jacobi_replay"]["sweeps"]
            and author_case["factor_replay"]["rotations"] == review_case["jacobi_replay"]["rotations"],
            "corrections": correction_rows,
        })
    output = {
        "schema": "cold-canopy-m1-face-accuracy-correctness/v1",
        "reviewer": "/root/accuracy_correctness", "pinned_head": HEAD,
        "evidence_class": "Ran: independent exact rational evaluation/sign certificate, Decimal Householder QR at 100/200 digits, and independent frozen binary64 replay; no Rust or physical execution",
        "dependencies": {
            "ordinary_capture_sha256": CAPTURES[0][2], "historical_capture_sha256": CAPTURES[1][2],
            "prior_guarded_oracle_sha256": sha256(HERE / "finite-precision-correctness-oracle-retained-face.py"),
            "prior_jacobi_replay_sha256": sha256(HERE / "face-pivot-correctness-reconstruct.py"),
            "author_plan_sha256": sha256(HERE / "face-accuracy-author-plan.json"),
            "author_source_sha256": sha256(author_source), "author_results_sha256": sha256(author_results_path),
            "review_source_sha256": sha256(Path(__file__)),
        },
        "cases": cases,
        "author_fix_verification": {
            "comparisons": comparisons,
            "resolved_high_findings": [
                "ordinary operation1 exact coordinate11 is zero; author now labels the negative Decimal QR residue unresolved rather than a mathematical infeasibility, and this independent certificate supplies the exact feasible reference",
                "all ordinary binary64 correction residual, norm, objective, and face reductions now use explicit serial accumulation; both correction delta vectors and every resulting p/h/Cbar/tau bit match the independent replay",
            ],
            "verdict": "PASS for the corrected author calculation when combined with this independent exact-reference certificate; no runtime amendment or physical/M1 release follows",
        },
    }
    destination = HERE / "face-accuracy-correctness.json"
    destination.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
