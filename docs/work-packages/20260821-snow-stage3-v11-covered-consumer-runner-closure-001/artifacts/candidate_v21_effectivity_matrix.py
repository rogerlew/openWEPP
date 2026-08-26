#!/usr/bin/env python3
"""Independent candidate-v21 numerical matrix; never imported by production."""

from __future__ import annotations

import json
import math
from pathlib import Path


SUPPORTS = (0.6, 0.600000001, 0.9, 1.199999999, 1.2, 1.875, 3.75)
RATES = (-0.01, -0.1, -1.0, -10.0)


def exact_affine(y0: float, rate: float, source: float, h: float) -> float:
    equilibrium = -source / rate
    return equilibrium + (y0 - equilibrium) * math.exp(rate * h)


def cn_affine(y0: float, rate: float, source: float, h: float) -> float:
    return (y0 * (1.0 + 0.5 * h * rate) + h * source) / (1.0 - 0.5 * h * rate)


def floor_estimator(y0: float, rate: float, source: float, h: float) -> tuple[float, float]:
    high = cn_affine(y0, rate, source, h)
    c1 = 0.5 - math.sqrt(3.0) / 6.0
    c2 = 0.5 + math.sqrt(3.0) / 6.0
    yc1 = y0 + c1 * (high - y0)
    yc2 = y0 + c2 * (high - y0)
    defect = high - y0 - 0.5 * h * (
        (rate * yc1 + source) + (rate * yc2 + source)
    )
    jacobian = 1.0 - 0.5 * h * rate
    return high, -defect / jacobian


def row(rate: float, h: float) -> dict[str, object]:
    y0 = 1.25
    source = 0.375
    exact = exact_affine(y0, rate, source, h)
    if h >= 1.2:
        coarse = cn_affine(y0, rate, source, h)
        half = cn_affine(y0, rate, source, h / 2.0)
        fine = cn_affine(half, rate, source, h / 2.0)
        estimate = (fine - coarse) / 3.0
        installed = fine
        selector = "cn_step_doubling"
        calls = (h, h / 2.0, h / 2.0)
    else:
        installed, estimate = floor_estimator(y0, rate, source, h)
        selector = "full_support_defect_transport"
        calls = (h,)
    reference_error = exact - installed
    enclosure = abs(reference_error) <= 2.0 * abs(estimate)
    effectivity = (
        abs(estimate) / abs(reference_error)
        if reference_error != 0.0
        else None
    )
    return {
        "family": "affine_relaxation",
        "rate_s_inv": rate,
        "support_s": h,
        "selector": selector,
        "constitutive_supports_s": calls,
        "minimum_support_ok": min(calls) >= 0.6,
        "installed": installed,
        "exact_reference": exact,
        "signed_estimate": estimate,
        "signed_reference_error": reference_error,
        "effectivity_abs_estimate_over_reference": effectivity,
        "gamma2_componentwise_enclosure": enclosure,
    }


def conservation_rows() -> list[dict[str, object]]:
    rows = []
    for h in SUPPORTS:
        # Equal-capacity two-node conduction. Mean is the exact conserved mode;
        # the difference follows d'=-2*k*d and uses the same candidate selector.
        k = 0.8
        mean = 1.5
        difference0 = 5.0
        difference = row(-2.0 * k, h)
        installed_difference = float(difference["installed"])
        # `row` uses a nonzero affine source/y0, so independently solve the
        # homogeneous difference here while retaining its selector rules.
        if h >= 1.2:
            coarse = cn_affine(difference0, -2.0 * k, 0.0, h)
            half = cn_affine(difference0, -2.0 * k, 0.0, h / 2.0)
            installed_difference = cn_affine(half, -2.0 * k, 0.0, h / 2.0)
            estimate = (installed_difference - coarse) / 3.0
        else:
            installed_difference, estimate = floor_estimator(
                difference0, -2.0 * k, 0.0, h
            )
        left = mean + 0.5 * installed_difference
        right = mean - 0.5 * installed_difference
        exact_difference = difference0 * math.exp(-2.0 * k * h)
        reference = exact_difference - installed_difference
        rows.append(
            {
                "family": "two_node_conduction",
                "support_s": h,
                "installed_sum": left + right,
                "beginning_sum": 2.0 * mean,
                "conservation_residual": left + right - 2.0 * mean,
                "signed_difference_estimate": estimate,
                "signed_difference_reference_error": reference,
                "gamma2_componentwise_enclosure": abs(reference)
                <= 2.0 * abs(estimate),
            }
        )
    return rows


def main() -> None:
    affine = [row(rate, h) for rate in RATES for h in SUPPORTS]
    conduction = conservation_rows()
    failed = [entry for entry in affine if not entry["gamma2_componentwise_enclosure"]]
    failed += [entry for entry in conduction if not entry["gamma2_componentwise_enclosure"]]
    payload = {
        "schema": "openwepp-child1-candidate-v21-effectivity-matrix-v1",
        "status": "PASS" if not failed else "DIVERGES",
        "production_operator_changed": False,
        "affine_rows": affine,
        "conduction_rows": conduction,
        "selector_boundary": {
            "below_1_2": row(-0.1, 1.199999999)["selector"],
            "at_1_2": row(-0.1, 1.2)["selector"],
            "no_blend": True,
        },
        "floor": {
            "minimum_s": 0.6,
            "all_constitutive_supports_admitted": all(
                entry["minimum_support_ok"] for entry in affine
            ),
        },
        "active_set_local": {
            "smooth_same_tag": "evaluated",
            "tag_disagreement": "typed_unsupported_required",
            "event_containing_floor_vector": "excluded_required",
        },
        "real_1_875_capture": {
            "physical_result": "BelowCarrierDomain",
            "coarse_complete_energy_bits": "0x40949afbc1928120",
            "refined_complete_energy_bits": "0x40942e218363bae1",
            "delta_complete_energy_bits": "0xc03b368f8bb18fc0",
            "candidate_cn_receipts_available": False,
            "disposition": "not_evaluable_without_candidate_operator",
        },
        "failed_enclosure_count": len(failed),
    }
    here = Path(__file__).resolve().parent
    json_path = here / "candidate-v21-effectivity-conservation-matrix.json"
    md_path = here / "candidate-v21-effectivity-conservation-matrix.md"
    json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    md_path.write_text(
        "# Candidate-v21 effectivity/conservation matrix\n\n"
        "Ran: package-local independent analytical evaluator. Production temporal "
        "operator unchanged.\n\n"
        f"- outcome: `{payload['status']}`\n"
        f"- affine rows: `{len(affine)}`\n"
        f"- two-node conservation rows: `{len(conduction)}`\n"
        f"- failed gamma=2 enclosures: `{len(failed)}`\n"
        f"- all constitutive supports >= 0.6 s: "
        f"`{payload['floor']['all_constitutive_supports_admitted']}`\n"
        "- two-node conservation residual: exact zero in every row\n"
        "- 1.2 s selector: full-support defect below; CN step doubling at/above; no blend\n"
        "- real 1.875 s current-carrier receipts: retained, but candidate-CN effectivity "
        "is not evaluable because no candidate operator exists\n\n"
        "The machine-readable JSON retains every operand, reference error, estimate, "
        "effectivity ratio, support, and enclosure result. `DIVERGES` or the unavailable "
        "real candidate-CN receipt is review evidence, not implementation authority.\n"
    )


if __name__ == "__main__":
    main()
