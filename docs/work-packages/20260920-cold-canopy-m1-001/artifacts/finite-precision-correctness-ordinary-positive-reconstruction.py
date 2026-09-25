#!/usr/bin/env python3
"""Independent reconstruction of the BODY13 ordinary-positive BVLS-02 capture.

This reads captured binary64 operands, never Rust treatment helpers.  It
reconstructs the signed-zero face crossing, ordered KKT arithmetic, directed
Cbar/tau enclosure, masks, classifications, terminal decision, and scoped
arithmetic work.  The previously reviewed independent work oracle supplies
only the frozen written-order upward operations and their counters.
"""

from __future__ import annotations

import hashlib
import json
import math
import runpy
import struct
from pathlib import Path
from typing import Any


N = 21
CAPTURE_SHA256 = "ec92859cf5d80eb7640e3d01861dc30f61288b49949b5977fb15f192ccf1a642"
METHOD_SHA256 = "7c596afda1331e50c9a33424e82b411d5ec7b40991f0cbcf8a034865dbddfe23"
STAGE1_SHA256 = "9f16963761ea9e399c8f9cb42eccd536622916088599aed7bfb0f8a77aa6603a"
RECEIPT_SHA256 = "8805ddd0958b76dad0d23a6a05239b630089186695193b986f64a8098e82d5ce"
STDERR_SHA256 = "74a0864b5275252a520139ab7151e32fb5a070f0aa2097e888b19bb3d484f4bc"
WORK_ORACLE_SHA256 = "0f97de91a99b5aca56c4d1b9248c508e95901131bb50b7b7cbd92595ed0702c6"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def decode(value: str) -> float:
    return struct.unpack(">d", bytes.fromhex(value))[0]


def bits(value: float) -> str:
    return struct.pack(">d", value).hex()


def decode_vector(values: list[str]) -> list[float]:
    result = [decode(value) for value in values]
    assert len(result) == N
    return result


def exact_vector(left: list[float], right_bits: list[str], label: str) -> None:
    actual = [bits(value) for value in left]
    assert actual == right_bits, f"{label} bit mismatch"


def reconstruct_kkt(
    matrix: list[list[float]], residual: list[float], step: list[float], lam: float
) -> tuple[list[float], list[float], list[float], list[float]]:
    # Separate statements force the specified product then addition sequence;
    # sums begin at zero and indices ascend.
    r_hat: list[float] = []
    for row in range(N):
        inner = 0.0
        for column in range(N):
            product = matrix[row][column] * step[column]
            inner = inner + product
        r_hat.append(residual[row] + inner)
    g_hat: list[float] = []
    for coordinate in range(N):
        total = 0.0
        for row in range(N):
            product = matrix[row][coordinate] * r_hat[row]
            total = total + product
        g_hat.append(total)
    lambda_times_p = [lam * value for value in step]
    h_hat = [g_hat[i] + lambda_times_p[i] for i in range(N)]
    return r_hat, g_hat, lambda_times_p, h_hat


def scaled_bounds(capture: dict[str, Any]) -> tuple[list[float], list[float]]:
    base = decode_vector(capture["base"])
    scales = decode_vector(capture["scales"])
    lower = decode_vector(capture["lower"])
    upper = decode_vector(capture["upper"])
    scaled_lower = [
        (lower[i] - base[i]) / scales[i] if math.isfinite(lower[i]) else lower[i]
        for i in range(N)
    ]
    scaled_upper = [
        (upper[i] - base[i]) / scales[i] if math.isfinite(upper[i]) else upper[i]
        for i in range(N)
    ]
    return scaled_lower, scaled_upper


def crossing_reconstruction(
    operation: dict[str, Any], lower: list[float], upper: list[float]
) -> dict[str, Any]:
    previous = decode_vector(operation["p_before"])
    candidate = decode_vector(operation["p_candidate"])
    candidates: list[dict[str, Any]] = []
    for coordinate in operation["free_ids"]:
        bound: float | None = None
        is_lower: bool | None = None
        if candidate[coordinate] < lower[coordinate]:
            bound, is_lower = lower[coordinate], True
        elif candidate[coordinate] > upper[coordinate]:
            bound, is_lower = upper[coordinate], False
        if bound is not None:
            numerator = bound - previous[coordinate]
            denominator = candidate[coordinate] - previous[coordinate]
            theta = numerator / denominator
            assert math.isfinite(theta) and 0.0 <= theta <= 1.0
            candidates.append(
                {
                    "coordinate": coordinate,
                    "lower": is_lower,
                    "theta_bits": bits(theta),
                    "numerator_bits": bits(numerator),
                    "denominator_bits": bits(denominator),
                }
            )
    assert candidates
    selected = min(candidates, key=lambda item: (decode(item["theta_bits"]), item["coordinate"]))
    theta = decode(selected["theta_bits"])
    conceptual = previous.copy()
    for coordinate in operation["free_ids"]:
        difference = candidate[coordinate] - previous[coordinate]
        product = theta * difference
        conceptual[coordinate] = previous[coordinate] + product
    coordinate = selected["coordinate"]
    conceptual[coordinate] = lower[coordinate] if selected["lower"] else upper[coordinate]
    exact_vector(conceptual, operation["conceptual_p_after"], "crossing conceptual p")
    assert selected["theta_bits"] == operation["theta"]
    assert coordinate == operation["coordinate"]
    assert selected["lower"] == operation["lower"]
    assert operation["applied"] is True
    expected_lower = list(operation["lower_before"])
    expected_upper = list(operation["upper_before"])
    expected_free = list(operation["free_before"])
    expected_free[coordinate] = False
    expected_lower[coordinate] = bool(selected["lower"])
    expected_upper[coordinate] = not bool(selected["lower"])
    assert expected_lower == operation["lower_after"]
    assert expected_upper == operation["upper_after"]
    assert expected_free == operation["free_after"]
    return {
        "action": operation["action"],
        "crossing_candidates": candidates,
        "selected": selected,
        "conceptual_p_after_bits": operation["conceptual_p_after"],
        "lower_after": expected_lower,
        "upper_after": expected_upper,
        "free_after": expected_free,
        "entered_factorizations": operation["entered"],
        "completed_pivots": operation["pivots"],
        "completed_sweeps": operation["sweeps"],
    }


def classify(
    coordinate: int,
    h: float,
    tau: float,
    fixed: bool,
    lower_active: bool,
    upper_active: bool,
) -> tuple[str, float | None]:
    if fixed:
        return "Fixed", None
    if lower_active:
        if h < -tau:
            return "ReleaseLower", None if tau == 0.0 else abs(h) / tau
        return "RetainLower", None
    if upper_active:
        if h > tau:
            return "ReleaseUpper", None if tau == 0.0 else abs(h) / tau
        return "RetainUpper", None
    if abs(h) <= tau:
        return "FreePass", None
    return "FreeRefuse", None


def reconstruct_assessment(
    capture: dict[str, Any], operation_index: int, work_module: dict[str, Any]
) -> dict[str, Any]:
    operation = capture["operations"][operation_index]
    assert operation["optimality"] is not None
    matrix = [[decode(value) for value in row] for row in operation["weighted_matrix"]]
    residual = decode_vector(operation["weighted_residual"])
    step = decode_vector(operation["p_candidate"])
    lam = decode(operation["lambda"])
    r_hat, g_hat, lambda_times_p, h_hat = reconstruct_kkt(matrix, residual, step, lam)
    exact_vector(r_hat, operation["residual"], "r_hat")
    exact_vector(g_hat, operation["g"], "g_hat")
    exact_vector(lambda_times_p, operation["lambda_times_p"], "lambda_times_p")
    exact_vector(h_hat, operation["h"], "h_hat")

    threshold = work_module["reconstruct_threshold_work"](
        matrix,
        residual,
        step,
        lam,
        list(operation["lower_before"]),
        list(operation["upper_before"]),
    )
    fixed = [
        decode(capture["lower"][i]) == decode(capture["upper"][i]) for i in range(N)
    ]
    coordinates: list[dict[str, Any]] = []
    release_candidates: list[tuple[int, bool, float | None]] = []
    free_refusals: list[int] = []
    for coordinate in range(N):
        reconstructed = threshold["coordinates"][coordinate]
        recorded = operation["optimality"]["coordinates"][coordinate]
        assert reconstructed["h_hat_bits"] == recorded["h"]
        assert reconstructed["cbar_bits"] == recorded["cbar"]
        assert reconstructed["tau_bits"] == recorded["tau"]
        tau = decode(reconstructed["tau_bits"])
        kind, ratio = classify(
            coordinate,
            h_hat[coordinate],
            tau,
            fixed[coordinate],
            operation["lower_before"][coordinate],
            operation["upper_before"][coordinate],
        )
        assert kind == recorded["class"]
        if kind == "ReleaseLower":
            release_candidates.append((coordinate, True, ratio))
        elif kind == "ReleaseUpper":
            release_candidates.append((coordinate, False, ratio))
        elif kind == "FreeRefuse":
            free_refusals.append(coordinate)
        coordinates.append(
            {
                "coordinate": coordinate,
                "h_bits": reconstructed["h_hat_bits"],
                "cbar_bits": reconstructed["cbar_bits"],
                "tau_bits": reconstructed["tau_bits"],
                "class": kind,
                "abs_h_over_tau": (
                    None if tau == 0.0 else abs(h_hat[coordinate]) / tau
                ),
                "cbar_tau_work": reconstructed["work"],
            }
        )
    assert not release_candidates
    assert threshold["release_candidates"] == []
    assert operation["optimality"]["selected_release"] is None
    assert free_refusals == [11]
    assert threshold["cbar_tau_work_all_coordinates"]["bounded_operations"] == operation[
        "optimality"
    ]["scalar_operations"]

    scaled_lower, scaled_upper = scaled_bounds(capture)
    assert all(scaled_lower[i] <= step[i] <= scaled_upper[i] for i in range(N))
    squared_norm = sum(value * value for value in step)
    radius = decode(operation["radius"])
    assert math.sqrt(squared_norm) <= radius
    assert operation["lower_before"] == operation["lower_after"]
    assert operation["upper_before"] == operation["upper_after"]
    assert operation["free_before"] == operation["free_after"]
    assert operation["p_candidate"] == operation["conceptual_p_after"]
    assert operation["p_before"] == operation["p_candidate"]

    threshold_work = threshold["cbar_tau_work_all_coordinates"]
    coefficient_work = threshold[
        "coefficient_setup_work_if_evaluated_inside_each_nonzero_cbar_call"
    ]
    ranking_work = threshold["release_ranking_work"]
    return {
        "operation_index": operation_index,
        "action": operation["action"],
        "weighted_inputs_equal_top_level_capture": (
            operation["weighted_matrix"] == capture["weighted_matrix"]
            and operation["weighted_residual"] == capture["weighted_residual"]
        ),
        "p_bits": operation["p_candidate"],
        "lambda_bits": operation["lambda"],
        "radius_bits": operation["radius"],
        "r_hat_bits": [bits(value) for value in r_hat],
        "g_hat_bits": [bits(value) for value in g_hat],
        "lambda_times_p_bits": [bits(value) for value in lambda_times_p],
        "h_hat_bits": [bits(value) for value in h_hat],
        "coordinates": coordinates,
        "active_release_candidates": [],
        "free_refusal_coordinates": free_refusals,
        "decision": {
            "kind": "TrustRegionOptimalityIndeterminate",
            "reason": "optimality_free_stationarity",
            "coordinate": 11,
        },
        "feasibility": {
            "box_feasible": True,
            "squared_norm_bits": bits(squared_norm),
            "norm_bits": bits(math.sqrt(squared_norm)),
            "inside_radius": True,
        },
        "work": {
            "recorded_cbar_tau_scalar_operations": operation["optimality"][
                "scalar_operations"
            ],
            "reconstructed_cbar_tau": threshold_work,
            "source_shape_gamma_setup_separate": coefficient_work,
            "ranking_separate": ranking_work,
            "policy_added_arithmetic_including_source_shape_gamma_setup": (
                threshold_work["bounded_operations"]
                + coefficient_work["bounded_operations"]
                + ranking_work["bounded_operations"]
            ),
            "reused_ordered_kkt_operations": {
                "residual": 43 * N,
                "gradient": 42 * N,
                "lambda_product_and_h_add": 2 * N,
                "total": (43 + 42 + 2) * N,
                "included_in_policy_added_count": False,
            },
        },
    }


def main() -> int:
    artifact_dir = Path(__file__).resolve().parent
    repository = artifact_dir.parents[3]
    capture_path = artifact_dir / "finite-precision-observed-capture.json"
    receipt_path = artifact_dir / "finite-precision-body13-ordinary-positive.json"
    stderr_path = artifact_dir / "finite-precision-body13-ordinary-positive.stderr"
    work_oracle_path = artifact_dir / "finite-precision-correctness-work-oracle.py"
    method_path = repository / "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md"
    stage1_path = Path(
        "/home/roger/openwepp-experiments/cold-canopy-m1-20260920/"
        "crates/openwepp-land-surface-energy/src/m1_trust_region_stage1.rs"
    )
    for path, expected in (
        (capture_path, CAPTURE_SHA256),
        (receipt_path, RECEIPT_SHA256),
        (stderr_path, STDERR_SHA256),
        (work_oracle_path, WORK_ORACLE_SHA256),
        (method_path, METHOD_SHA256),
        (stage1_path, STAGE1_SHA256),
    ):
        assert sha256(path) == expected, f"identity mismatch: {path}"

    capture = json.loads(capture_path.read_text(encoding="utf-8"))
    work_module = runpy.run_path(str(work_oracle_path))
    assert len(capture["operations"]) == 2
    assert capture["operations"][0]["optimality"] is None
    assert capture["operations"][1]["optimality"] is not None
    scaled_lower, scaled_upper = scaled_bounds(capture)
    transition = crossing_reconstruction(capture["operations"][0], scaled_lower, scaled_upper)
    assessment = reconstruct_assessment(capture, 1, work_module)
    assert capture["operations"][0]["lower_after"] == capture["operations"][1]["lower_before"]
    assert capture["operations"][0]["upper_after"] == capture["operations"][1]["upper_before"]
    assert capture["operations"][0]["free_after"] == capture["operations"][1]["free_before"]

    refusal = capture["refusal"]
    assert refusal["kind"] == assessment["decision"]["kind"]
    assert refusal["reason"] == assessment["decision"]["reason"]
    assert refusal["coordinate"] == assessment["decision"]["coordinate"]
    assert refusal["p"] == assessment["p_bits"]
    assert refusal["lower"] == capture["operations"][1]["lower_after"]
    assert refusal["upper"] == capture["operations"][1]["upper_after"]
    assert refusal["free"] == capture["operations"][1]["free_after"]
    assert refusal["free_ids"] == capture["operations"][1]["free_ids"]
    assert refusal["radius"] == assessment["radius_bits"]
    assert refusal["lambda"] == assessment["lambda_bits"]
    assert refusal["optimality"] == capture["operations"][1]["optimality"]
    assert refusal["entered"] == 2 and refusal["pivots"] == 1 and refusal["sweeps"] == 7
    assert capture["completed_subproblems"] == []
    assert capture["accepted_installs"] == []
    assert capture["post_update_admissions"] == []

    output = {
        "evidence_class": (
            "independent offline binary64 reconstruction from captured operands; "
            "no Rust treatment helper and no physical execution"
        ),
        "identity": {
            "capture_path": str(capture_path),
            "capture_sha256": sha256(capture_path),
            "receipt_path": str(receipt_path),
            "receipt_sha256": sha256(receipt_path),
            "stderr_path": str(stderr_path),
            "stderr_sha256": sha256(stderr_path),
            "method_path": str(method_path),
            "method_sha256": sha256(method_path),
            "stage1_path": str(stage1_path),
            "stage1_sha256": sha256(stage1_path),
            "work_oracle_path": str(work_oracle_path),
            "work_oracle_sha256": sha256(work_oracle_path),
            "reconstruction_script_path": str(Path(__file__).resolve()),
            "reconstruction_script_sha256": sha256(Path(__file__).resolve()),
        },
        "assessed_operation_indices": [1],
        "unassessed_transition_operation_indices": [0],
        "face_transition": transition,
        "assessment": assessment,
        "terminal_refusal_matches_assessment": True,
        "terminal_refusal": refusal,
        "physical_nested_counts_from_recorded_receipt": {
            "core_attempted": 41,
            "core_completed": 41,
            "raw_jacobian_attempted": 1,
            "raw_jacobian_completed": 1,
            "hydraulic_attempted": 0,
            "hydraulic_completed": 0,
            "stage1_factorization_attempted": 2,
            "stage1_factorization_completed": 2,
            "materialization_attempted": 0,
            "materialization_completed": 0,
            "trial_entries": 0,
            "accepted_installs": 0,
        },
        "disposition": (
            "source-conforming negative ordinary-positive result: coordinate 5 activates "
            "at signed -0 and remains lower-active; with no releasable active bound, free "
            "coordinate 11 fails stationarity and the initial subproblem refuses before "
            "any update or materialization"
        ),
    }
    output_path = artifact_dir / "finite-precision-correctness-ordinary-positive-reconstruction.json"
    output_path.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(output_path)
    print(sha256(output_path))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
