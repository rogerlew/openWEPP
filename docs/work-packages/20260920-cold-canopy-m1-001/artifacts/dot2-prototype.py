#!/usr/bin/env python3
"""Frozen, release-gated calculator for M1-DOT2-RESIDUAL-PROTOTYPE-01.

This file intentionally refuses to run the retained cases without an explicit
release token.  It is evidence code only and never invokes Rust or physics.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import runpy
import struct
import sys
from decimal import Decimal, localcontext
from fractions import Fraction
from pathlib import Path

def load_module(filename: str, name: str):
    spec = importlib.util.spec_from_file_location(name, HERE / filename)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"unable to load {filename}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module

N = 21
HERE = Path(__file__).resolve().parent
PRIMITIVE = load_module("dot2-primitive.py", "dot2_primitive")
PrototypeLimitation = PRIMITIVE.PrototypeLimitation
bits = PRIMITIVE.bits
dot2 = PRIMITIVE.dot2
verify_identities = PRIMITIVE.verify_identities
RELEASE_TOKEN = "ASTRA_RELEASE_M1_DOT2_RESIDUAL_PROTOTYPE_01"
CASES = (
    ("ordinary_positive", "finite-precision-observed-capture.json", "ec92859cf5d80eb7640e3d01861dc30f61288b49949b5977fb15f192ccf1a642"),
    ("historical_near_root", "face-pivot-observed-capture.json", "57c26d5b9215f9e22cacf2958afb60891acc54c46c540e3dcf0e748dff585a8c"),
)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def decode(value: str) -> float:
    return struct.unpack(">d", bytes.fromhex(value))[0]


def vector(values: list[str]) -> list[float]:
    return [decode(value) for value in values]


def serial_residual(a: list[list[float]], f: list[float], p: list[float]) -> list[float]:
    out = []
    for row in range(N):
        total = 0.0
        for col in range(N):
            total = total + a[row][col] * p[col]
        out.append(f[row] + total)
    return out


def decimal200_residual(a: list[list[float]], f: list[float], p: list[float]) -> list[float]:
    with localcontext() as context:
        context.prec = 200
        out = []
        for row in range(N):
            total = Decimal(0)
            for col in range(N):
                total += Decimal.from_float(a[row][col]) * Decimal.from_float(p[col])
            out.append(float(Decimal.from_float(f[row]) + total))
    return out


def exact_residual(a: list[list[float]], f: list[float], p: list[float]) -> list[Fraction]:
    return [
        Fraction.from_float(f[row]) + sum(Fraction.from_float(a[row][col]) * Fraction.from_float(p[col]) for col in range(N))
        for row in range(N)
    ]


def ulp_correspondence(value: float, exact: Fraction) -> dict[str, object]:
    rounded = float(exact)
    actual = int(bits(value), 16)
    target = int(bits(rounded), 16)
    def ordered(raw: int) -> int:
        return (~raw & ((1 << 64) - 1)) if (raw >> 63) else raw | (1 << 63)
    return {
        "exact_rounded_bits": bits(rounded),
        "matches_exact_rounding": bits(value) == bits(rounded),
        "signed_ulp_distance_from_exact_rounding": ordered(actual) - ordered(target),
        "signed_zero_pair": value == 0.0 and rounded == 0.0,
        "signed_error_fraction": str(Fraction.from_float(value) - exact),
    }


def accepted_case(name: str, author_results: dict) -> dict:
    for case in author_results["cases"]:
        if case["case"] == name:
            return case
    raise ValueError(f"accepted audit missing case {name}")


def certificate_case(name: str, correctness: dict) -> dict:
    for case in correctness["cases"]:
        if case["case"] == name:
            return case
    raise ValueError(f"accepted correctness certificate missing case {name}")


def assert_first_correction(case: dict, current: list[float], delta: list[float]) -> None:
    accepted = case["corrections"][0]
    if [bits(value) for value in delta] != accepted["delta_free_bits"]:
        raise PrototypeLimitation("ordinary correction-1 delta bits differ from accepted audit")
    actual = [bits(value) for value in current]
    accepted_p = [row["p_bits"] for row in accepted["assessment"]["coordinates"]]
    if actual != accepted_p:
        raise PrototypeLimitation("ordinary correction-1 p bits differ from accepted audit")


def residual_dot2(a: list[list[float]], f: list[float], p: list[float]):
    rows = []
    values = []
    for row in range(N):
        result = dot2([f[row], *a[row]], [1.0, *p])
        verify_identities(result)
        values.append(result.value)
        rows.append({
            "row": row,
            "bits": bits(result.value),
            "arithmetic": result.counters.arithmetic,
            "guards": result.counters.guards,
            "branches": result.counters.branches,
            "exceptional_branches": result.counters.exceptional_branches,
            "identity_checks": result.counters.identity_checks,
            "signed_zero_values": result.counters.signed_zero_values,
            "range": {
                "minimum_nonzero_abs": repr(result.ranges.minimum_nonzero_abs),
                "maximum_abs": repr(result.ranges.maximum_abs),
                "values": result.ranges.values,
            },
        })
    return values, rows


def run() -> dict:
    protocol = json.loads((HERE / "dot2-protocol.json").read_text())
    if protocol["status"] != "FROZEN_PRE_RUN_PENDING_INDEPENDENT_REVIEW":
        raise PrototypeLimitation("protocol status no longer admits this frozen run")
    author_results = json.loads((HERE / "face-accuracy-author-results.json").read_text())
    correctness = json.loads((HERE / "face-accuracy-correctness.json").read_text())
    if sha(HERE / "face-pivot-correctness-reconstruct.py") != author_results["dependencies"]["jacobi_replay_sha256"]:
        raise PrototypeLimitation("accepted Jacobi replay receipt mismatch")
    if sha(HERE / "face-accuracy-author-results.json") != correctness["dependencies"]["author_results_sha256"]:
        raise PrototypeLimitation("accepted author result receipt mismatch")
    if sha(HERE / "face-accuracy-correctness.py") != correctness["dependencies"]["review_source_sha256"]:
        raise PrototypeLimitation("accepted correctness source receipt mismatch")
    replay = runpy.run_path(str(HERE / "face-pivot-correctness-reconstruct.py"))
    correctness_module = runpy.run_path(str(HERE / "face-accuracy-correctness.py"))
    cases = []
    for name, capture_name, expected_sha in CASES:
        capture_path = HERE / capture_name
        if sha(capture_path) != expected_sha:
            raise PrototypeLimitation(f"capture custody mismatch for {name}")
        capture = json.loads(capture_path.read_text())
        op = capture["operations"][1]
        a, f, initial = [vector(row) for row in capture["weighted_matrix"]], vector(capture["weighted_residual"]), vector(op["p_candidate"])
        freeids = [index for index, free in enumerate(op["free_before"]) if free]
        active = [index for index in range(N) if index not in freeids]
        face_a = [[row[index] for index in freeids] for row in a]
        face_r = []
        for row in range(N):
            total = f[row]
            for index in active:
                total = total + a[row][index] * initial[index]
            face_r.append(total)
        factor = replay["jacobi_factor"](face_a, freeids, 0)
        replayed_candidate = replay["lambda_step"](factor, face_r, decode(op["lambda"]))
        if [bits(value) for value in replayed_candidate] != [op["p_candidate"][index] for index in freeids]:
            raise PrototypeLimitation("factor replay candidate mismatch")
        first_residual = serial_residual(a, f, initial)
        first_delta = replay["lambda_step"](factor, first_residual, decode(op["lambda"]))
        first_p = initial[:]
        for index, value in zip(freeids, first_delta, strict=True):
            first_p[index] = first_p[index] + value
        accepted = accepted_case(name, author_results)
        certified = certificate_case(name, correctness)
        assert_first_correction(accepted, first_p, first_delta)
        dot2_values, dot2_rows = residual_dot2(a, f, first_p)
        historical = decimal200_residual(a, f, first_p)
        historical_delta = replay["lambda_step"](factor, historical, decode(op["lambda"]))
        historical_p = first_p[:]
        for index, value in zip(freeids, historical_delta, strict=True):
            historical_p[index] = historical_p[index] + value
        historical_accepted = accepted["corrections"][1]
        if [bits(value) for value in historical_delta] != historical_accepted["delta_free_bits"]:
            raise PrototypeLimitation("historical Decimal-200 residual no longer reproduces accepted correction-2 delta bits")
        historical_p_bits = [row["p_bits"] for row in historical_accepted["assessment"]["coordinates"]]
        if [bits(value) for value in historical_p] != historical_p_bits:
            raise PrototypeLimitation("historical Decimal-200 residual no longer reproduces accepted correction-2 p bits")
        exact = exact_residual(a, f, first_p)
        for row in range(N):
            dot2_rows[row]["historical_decimal200_bits"] = bits(historical[row])
            dot2_rows[row]["exact_fraction"] = str(exact[row])
            dot2_rows[row]["dot2"] = ulp_correspondence(dot2_values[row], exact[row])
            dot2_rows[row]["historical_decimal200"] = ulp_correspondence(historical[row], exact[row])
        second_delta = replay["lambda_step"](factor, dot2_values, decode(op["lambda"]))
        final_p = first_p[:]
        for index, value in zip(freeids, second_delta, strict=True):
            final_p[index] = final_p[index] + value
        # The guarded frozen assessment is deliberately delegated to the accepted
        # audit oracle; this prototype does not redefine its BVLS-02 predicate.
        assessment = correctness_module["assess_binary64"](capture, op, a, f, final_p, decode(op["lambda"]))
        assessment["prototype_complete_acceptance"] = bool(
            assessment["complete_bvls02_and_feasibility_pass"]
            and assessment["all_kkt_and_enclosure_intermediates_normal_or_zero"]
            and assessment["active_values_preserved"]
            and assessment["active_values_equal_declared_bounds"]
        )
        exact_p, _, _, _ = correctness_module["exact_face_solution"](
            a, f, initial, freeids, decode(op["lambda"])
        )
        exact_reference = [decode(value) for value in certified["once_rounded_exact_reference_bits"]]
        exact_reference_error = [
            str(Fraction.from_float(final_p[index]) - exact_p[index])
            for index in range(N)
        ]
        cases.append({
            "case": name,
            "capture_sha256": expected_sha,
            "first_correction": {"delta_free_bits": [bits(value) for value in first_delta], "p_bits": [bits(value) for value in first_p]},
            "dot2_rows": dot2_rows,
            "second_correction": {"delta_free_bits": [bits(value) for value in second_delta], "p_bits": [bits(value) for value in final_p], "assessment": assessment},
            "cost": {
                "dot2_residual_actual": {
                    "rows": N,
                    "arithmetic": sum(row["arithmetic"] for row in dot2_rows),
                    "guards": sum(row["guards"] for row in dot2_rows),
                    "branches": sum(row["branches"] for row in dot2_rows),
                    "exceptional_branches": sum(row["exceptional_branches"] for row in dot2_rows),
                    "identity_checks": sum(row["identity_checks"] for row in dot2_rows),
                    "paper_unguarded_arithmetic_reference": 543 * N,
                },
                "ordinary_first_residual": {"arithmetic": 903, "source": "accepted explicit ordered-binary64 residual loop"},
                "factor_reuse_application": {
                    "correction_1": 18960,
                    "correction_2": 18960,
                    "proof": "same face-pivot-correctness-reconstruct.py lambda_step byte identity accepted by the prior audit; no new factorization",
                    "replay_source_sha256": sha(HERE / "face-pivot-correctness-reconstruct.py"),
                },
                "free_coordinate_updates": {"correction_1": len(freeids), "correction_2": len(freeids)},
                "unchanged_assessment": {"ordered_kkt": 1827, "enclosure_upper_bound": 39816},
                "one_time_factor_reconstruction": {"sweeps": factor.sweeps, "rotations": factor.rotations, "not_per_correction": True},
            },
            "exact_reference": {
                "source": "face-accuracy-correctness.json once_rounded_exact_reference",
                "p_bits": certified["once_rounded_exact_reference_bits"],
                "accepted_unchanged_assessment": certified["once_rounded_exact_reference"],
                "final_p_error_vs_exact_face_minimizer_fraction_per_coordinate": exact_reference_error,
                "exact_error_source": "fresh exact-rational diagnostic rerun through accepted face-accuracy-correctness.py exact_face_solution; it is distinct from the reused once-rounded certificate",
                "final_p_error_vs_once_rounded_reference_fraction_per_coordinate": [
                    str(Fraction.from_float(final_p[index]) - Fraction.from_float(exact_reference[index]))
                    for index in range(N)
                ],
            },
        })
    dependencies = {
        name: sha(HERE / name)
        for name in (
            "face-accuracy-author-results.json",
            "face-accuracy-correctness.json",
            "face-pivot-correctness-reconstruct.py",
            "face-accuracy-correctness.py",
            "dot2-primitive.py",
        )
    }
    return {"candidate": protocol["candidate"], "protocol_sha256": sha(HERE / "dot2-protocol.json"), "source_sha256": sha(Path(__file__)), "dependencies": dependencies, "cases": cases}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--release-token", required=True)
    parser.add_argument("--output", type=Path, default=HERE / "dot2-results.json")
    args = parser.parse_args()
    if args.release_token != RELEASE_TOKEN:
        raise SystemExit("result-bearing execution refused: explicit Astra release token required")
    args.output.write_text(json.dumps(run(), indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
