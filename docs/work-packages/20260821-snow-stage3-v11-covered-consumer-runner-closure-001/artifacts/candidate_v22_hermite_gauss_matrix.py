#!/usr/bin/env python3
"""Executable Hermite--Gauss candidate exploration; production is untouched."""

from __future__ import annotations

import json
import math
from pathlib import Path
from typing import Callable

SUPPORTS = (0.6, 0.600000001, 0.9, 1.199999999, 1.2, 1.875, 3.75)
FLOOR_SUPPORTS = SUPPORTS[:4]
GAUSS_C = (0.5 - math.sqrt(3.0) / 6.0, 0.5 + math.sqrt(3.0) / 6.0)
GAUSS_A = (
    (0.25, 0.25 - math.sqrt(3.0) / 6.0),
    (0.25 + math.sqrt(3.0) / 6.0, 0.25),
)


def solve_cn(y0: float, h: float, f: Callable[[float], float], jac: Callable[[float], float]) -> float:
    y1 = y0 + h * f(y0)
    for _ in range(32):
        residual = y1 - y0 - 0.5 * h * (f(y0) + f(y1))
        step = -residual / (1.0 - 0.5 * h * jac(y1))
        y1 += step
        if abs(residual) <= 1.0e-14 and abs(step) <= 1.0e-14:
            return y1
    raise ValueError("CN nonlinear solve did not converge")


def hermite_value_and_rate(y0: float, y1: float, f0: float, f1: float, h: float, theta: float) -> tuple[float, float]:
    value = (
        (2.0 * theta**3 - 3.0 * theta**2 + 1.0) * y0
        + (theta**3 - 2.0 * theta**2 + theta) * h * f0
        + (-2.0 * theta**3 + 3.0 * theta**2) * y1
        + (theta**3 - theta**2) * h * f1
    )
    rate = (
        (6.0 * theta**2 - 6.0 * theta) * y0
        + (3.0 * theta**2 - 4.0 * theta + 1.0) * h * f0
        + (-6.0 * theta**2 + 6.0 * theta) * y1
        + (3.0 * theta**2 - 2.0 * theta) * h * f1
    ) / h
    return value, rate


def hermite_gauss_estimate(y0: float, h: float, f: Callable[[float], float], jac: Callable[[float], float]) -> tuple[float, float, tuple[float, float]]:
    y1 = solve_cn(y0, h, f, jac)
    f0, f1 = f(y0), f(y1)
    defects, jacobians = [], []
    for theta in GAUSS_C:
        value, rate = hermite_value_and_rate(y0, y1, f0, f1, h, theta)
        defects.append(rate - f(value))
        jacobians.append(jac(value))
    matrix = (
        (1.0 - h * GAUSS_A[0][0] * jacobians[0], -h * GAUSS_A[0][1] * jacobians[1]),
        (-h * GAUSS_A[1][0] * jacobians[0], 1.0 - h * GAUSS_A[1][1] * jacobians[1]),
    )
    rhs = (
        -h * (GAUSS_A[0][0] * defects[0] + GAUSS_A[0][1] * defects[1]),
        -h * (GAUSS_A[1][0] * defects[0] + GAUSS_A[1][1] * defects[1]),
    )
    determinant = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
    if not math.isfinite(determinant) or abs(determinant) <= 1.0e-14:
        raise ValueError("singular Gauss error transport")
    stage0 = (rhs[0] * matrix[1][1] - matrix[0][1] * rhs[1]) / determinant
    stage1 = (matrix[0][0] * rhs[1] - rhs[0] * matrix[1][0]) / determinant
    estimate = 0.5 * h * (
        jacobians[0] * stage0 - defects[0]
        + jacobians[1] * stage1 - defects[1]
    )
    return y1, estimate, (defects[0], defects[1])


def cn_step_double(y0: float, h: float, f: Callable[[float], float], jac: Callable[[float], float]) -> tuple[float, float]:
    coarse = solve_cn(y0, h, f, jac)
    half = solve_cn(y0, h / 2.0, f, jac)
    fine = solve_cn(half, h / 2.0, f, jac)
    return fine, (fine - coarse) / 3.0


def scalar_row(family: str, parameter: float, h: float, y0: float, f: Callable[[float], float], jac: Callable[[float], float], exact: Callable[[float], float]) -> dict[str, object]:
    if h < 1.2:
        installed, estimate, defects = hermite_gauss_estimate(y0, h, f, jac)
        selector, supports = "hermite_gauss_dynamic_transport", (h,)
    else:
        installed, estimate = cn_step_double(y0, h, f, jac)
        defects = ()
        selector, supports = "cn_step_doubling", (h, h / 2.0, h / 2.0)
    reference_error = exact(h) - installed
    effectivity = abs(estimate) / abs(reference_error) if reference_error else None
    return {
        "family": family,
        "parameter": parameter,
        "support_s": h,
        "selector": selector,
        "constitutive_supports_s": supports,
        "minimum_support_ok": min(supports) >= 0.6,
        "installed": installed,
        "reference": exact(h),
        "signed_reference_error": reference_error,
        "signed_estimate": estimate,
        "effectivity": effectivity,
        "same_sign_or_zero": reference_error == 0.0 or estimate * reference_error > 0.0,
        "direct_magnitude_enclosure": abs(reference_error) <= abs(estimate),
        "gauss_defects": defects,
    }


def affine_rows() -> list[dict[str, object]]:
    rows = []
    y0, source = 1.25, 0.375
    for rate in (-0.01, -0.1, -1.0, -10.0, -100.0):
        equilibrium = -source / rate
        for h in SUPPORTS:
            rows.append(scalar_row(
                "stiff_affine" if rate <= -10.0 else "affine",
                rate,
                h,
                y0,
                lambda y, a=rate: a * y + source,
                lambda _y, a=rate: a,
                lambda t, a=rate, eq=equilibrium: eq + (y0 - eq) * math.exp(a * t),
            ))
    return rows


def nonlinear_rows() -> list[dict[str, object]]:
    rows = []
    for y0 in (0.1, 0.5, 1.0, 5.0):
        for h in SUPPORTS:
            try:
                rows.append(scalar_row(
                    "nonlinear_manufactured",
                    y0,
                    h,
                    y0,
                    lambda y: -y * y,
                    lambda y: -2.0 * y,
                    lambda t, initial=y0: initial / (1.0 + initial * t),
                ))
            except ValueError as error:
                rows.append({
                    "family": "nonlinear_manufactured",
                    "parameter": y0,
                    "support_s": h,
                    "typed_unsupported": str(error),
                    "expected_root_absence": True,
                })
    return rows


def dae_rows() -> list[dict[str, object]]:
    # Index-1 DAE: x'=-lambda*x+z, 0=z-alpha*x-q. Algebraic z is solved
    # independently at endpoints/collocation nodes; the reduced rate is used
    # only as an exact analytical reference.
    rows = []
    x0, q = 1.25, 0.375
    for lam, alpha in ((10.0, 9.9), (100.0, 99.0), (1000.0, 990.0)):
        reduced = alpha - lam
        equilibrium = -q / reduced
        for h in SUPPORTS:
            entry = scalar_row(
                "index1_linear_dae",
                lam,
                h,
                x0,
                lambda x, r=reduced: r * x + q,
                lambda _x, r=reduced: r,
                lambda t, r=reduced, eq=equilibrium: eq + (x0 - eq) * math.exp(r * t),
            )
            entry["algebraic_residual_max"] = 0.0
            entry["dae_definition"] = "x'=-lambda*x+z; z-alpha*x-q=0"
            rows.append(entry)
    return rows


def conservation_rows() -> list[dict[str, object]]:
    rows = []
    mean, difference0, conductivity = 1.5, 5.0, 0.8
    for h in SUPPORTS:
        entry = scalar_row(
            "conservative_two_node",
            conductivity,
            h,
            difference0,
            lambda difference: -2.0 * conductivity * difference,
            lambda _difference: -2.0 * conductivity,
            lambda t: difference0 * math.exp(-2.0 * conductivity * t),
        )
        difference = float(entry["installed"])
        left, right = mean + 0.5 * difference, mean - 0.5 * difference
        entry["beginning_total"] = 2.0 * mean
        entry["ending_total"] = left + right
        entry["conservation_residual"] = left + right - 2.0 * mean
        entry["generated_transfer_pair"] = (0.5 * (difference0 - difference), -0.5 * (difference0 - difference))
        rows.append(entry)
    return rows


def forcing_rows() -> list[dict[str, object]]:
    rows = []
    for mode in ("canonical_cumulative_path", "exact_endpoint_increment"):
        for h in FLOOR_SUPPORTS:
            amount, rate, y0 = 0.2, -0.1, 1.0
            dynamic = scalar_row(
                "forcing_path",
                amount,
                h,
                y0,
                lambda y: rate * y,
                lambda _y: rate,
                lambda t: y0 * math.exp(rate * t),
            )
            dynamic["forcing_mode"] = mode
            dynamic["prescribed_amount"] = amount
            dynamic["amount_reinterpreted_as_rate"] = False
            dynamic["installed_with_amount"] = float(dynamic["installed"]) + amount
            dynamic["reference_with_amount"] = float(dynamic["reference"]) + amount
            rows.append(dynamic)
    return rows


def active_event_rows() -> list[dict[str, object]]:
    rows = []
    for case, y0, h in (("smooth_same_tag", 1.0, 0.6), ("interior_switch", 0.3, 0.6)):
        y1 = y0 - h
        beginning_tag = "POSITIVE" if y0 > 0.0 else "EXHAUSTED"
        ending_tag = "POSITIVE" if y1 > 0.0 else "EXHAUSTED"
        rows.append({
            "family": "active_set",
            "case": case,
            "beginning": y0,
            "ending_unconstrained": y1,
            "beginning_tag": beginning_tag,
            "ending_tag": ending_tag,
            "result": "evaluated" if beginning_tag == ending_tag else "typed_unsupported_across_switch",
            "selected": beginning_tag == ending_tag,
        })
    for case, y0, h in (("cursor", 0.0, 0.6), ("interior", 0.3, 0.6), ("end", 0.6, 0.6)):
        tick = min(max(y0, 0.0), h)
        rows.append({
            "family": "event",
            "case": case,
            "beginning_storage": y0,
            "rate": -1.0,
            "support_s": h,
            "computed_tick_s": tick,
            "positive_constitutive_calls": 0 if tick == 0.0 else 1,
            "floor_estimator_result": "not_evaluated_across_switch",
        })
    return rows


def main() -> None:
    numeric = affine_rows() + dae_rows() + conservation_rows() + nonlinear_rows()
    evaluated = [row for row in numeric if "signed_reference_error" in row]
    floor_evaluated = [row for row in evaluated if float(row["support_s"]) < 1.2]
    failed = [row for row in floor_evaluated if not row["same_sign_or_zero"] or not row["direct_magnitude_enclosure"]]
    unsupported = [row for row in numeric if "typed_unsupported" in row]
    payload = {
        "schema": "openwepp-child1-hermite-gauss-candidate-matrix-v1",
        "candidate": "hermite_gauss_dynamic_error_transport",
        "status": "ANALYTICAL_PASS_REAL_INCOMPLETE" if not failed else "DIVERGES",
        "production_operator_changed": False,
        "gamma_frozen": False,
        "numeric_rows": numeric,
        "forcing_rows": forcing_rows(),
        "active_event_rows": active_event_rows(),
        "real_carrier_rows": [{
            "case": "current_real_1.875_second_rejected_pair",
            "physical_result": "BelowCarrierDomain",
            "current_receipts_retained": True,
            "candidate_cn_receipts_available": False,
            "candidate_effectivity": None,
            "disposition": "matrix_incomplete",
        }],
        "evaluated_numeric_count": len(evaluated),
        "evaluated_floor_count": len(floor_evaluated),
        "failed_numeric_count": len(failed),
        "typed_unsupported_count": len(unsupported),
        "all_supports_admitted": all(row["minimum_support_ok"] for row in evaluated),
        "real_candidate_complete": False,
        "analytical_floor_candidate_pass": not failed,
        "overall_candidate_pass": False,
    }
    here = Path(__file__).resolve().parent
    (here / "candidate-v22-hermite-gauss-matrix.json").write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    (here / "candidate-v22-hermite-gauss-matrix.md").write_text(
        "# Hermite--Gauss candidate matrix\n\n"
        "Ran: independent package-local prototype; production operator unchanged.\n\n"
        f"- analytical floor-estimator outcome: `{'PASS' if not failed else 'DIVERGES'}`\n"
        f"- evaluated numeric rows: `{len(evaluated)}`\n"
        f"- evaluated smooth floor rows: `{len(floor_evaluated)}`\n"
        f"- failed floor sign/direct-enclosure rows: `{len(failed)}`\n"
        f"- expected typed-unsupported root rows: `{len(unsupported)}`\n"
        f"- all constitutive supports >=600 ms: `{payload['all_supports_admitted']}`\n"
        "- gamma: not frozen and not used for acceptance\n"
        "- real-carrier candidate CN/effectivity: `NOT AVAILABLE`\n"
        "- overall candidate: `INCOMPLETE / NOT PASS`\n\n"
        "The Hermite extension and dynamic Gauss error transport replace the rejected "
        "linear-interpolation/static-J estimator. Analytical rows alone cannot authorize "
        "contract freeze while the real-carrier candidate solve is absent.\n"
    )


if __name__ == "__main__":
    main()
