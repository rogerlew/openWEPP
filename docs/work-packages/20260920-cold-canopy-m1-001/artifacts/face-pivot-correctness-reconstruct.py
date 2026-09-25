#!/usr/bin/env python3
"""Independent ordered-binary64 audit of the retained M1 Stage 1 record.

This calculator consumes the single FACE_PIVOT_CAPTURE JSON object emitted by
the authorized observation hook.  It is an offline translation of the frozen
Stage 1 arithmetic at terminal tree
9ec7bf7698d055d267e41cac381eaafb56ca57fa1fce5a62359da1d3714a9a3d.
It does not call Rust, the nonlinear evaluator, or a target workflow.

Every recorded floating-point input is expected as a 16-digit hexadecimal
binary64 bit string.  The report retains calculated values in the same form and
uses exact bit comparisons before reporting any decimal diagnostics.
"""

from __future__ import annotations

import argparse
import ctypes
import json
import math
import struct
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable, Optional


TERMINAL_TREE = "9ec7bf7698d055d267e41cac381eaafb56ca57fa1fce5a62359da1d3714a9a3d"
N = 21
PIVOT_CAP = 42
JACOBI_SWEEP_CAP = 64
JACOBI_ROTATE_SCALE = 2.0**-48
RANK_SCALE = 2.0**-40
EPSILON = 2.220446049250313e-16


def from_bits(value: Any) -> float:
    if not isinstance(value, str) or len(value) != 16:
        raise ValueError(f"expected 16-digit binary64 hex string, got {value!r}")
    return struct.unpack(">d", bytes.fromhex(value))[0]


def bits(value: float) -> str:
    return struct.pack(">d", float(value)).hex()


def bits_equal(left: float, right: float) -> bool:
    return bits(left) == bits(right)


def decode_vector(values: Iterable[Any], length: Optional[int] = None) -> list[float]:
    result = [from_bits(value) for value in values]
    if length is not None and len(result) != length:
        raise ValueError(f"expected vector length {length}, got {len(result)}")
    return result


def decode_optional(value: Any) -> Optional[float]:
    return None if value is None else from_bits(value)


def encode_vector(values: Iterable[float]) -> list[str]:
    return [bits(value) for value in values]


def exact_vector(left: Iterable[float], right: Iterable[float]) -> bool:
    return all(bits_equal(a, b) for a, b in zip(left, right, strict=True))


def finite_max(values: Iterable[float]) -> float:
    maximum = 0.0
    for value in values:
        if not math.isfinite(value):
            raise ValueError("nonfinite finite_max operand")
        maximum = max(maximum, abs(value))
    return maximum


try:
    _process = ctypes.CDLL(None)
    _c_hypot = _process.hypot
    _c_hypot.argtypes = (ctypes.c_double, ctypes.c_double)
    _c_hypot.restype = ctypes.c_double
except (AttributeError, OSError):  # pragma: no cover - platform fallback is labelled below
    _c_hypot = None


def rust_hypot(left: float, right: float) -> float:
    # Rust f64::hypot lowers to the host hypot implementation on this retained
    # Linux execution host.  ctypes avoids Python math.hypot's n-ary algorithm.
    if _c_hypot is None:
        return math.hypot(left, right)
    return float(_c_hypot(left, right))


def checked_square_sum(values: Iterable[float]) -> float:
    total = 0.0
    for value in values:
        square = value * value
        if not math.isfinite(square):
            raise ValueError("nonfinite square")
        total += square
        if not math.isfinite(total):
            raise ValueError("nonfinite square sum")
    return total


def weighted_system(
    raw: list[float], jacobian: list[list[float]], normalizers: list[float], scales: list[float]
) -> tuple[list[float], list[list[float]]]:
    weighted_residual: list[float] = []
    weighted_matrix: list[list[float]] = []
    for row in range(len(raw)):
        weight = 1.0 / normalizers[row]
        weighted_residual.append(weight * raw[row])
        output_row: list[float] = []
        for col in range(len(scales)):
            weighted_jacobian = weight * jacobian[row][col]
            output_row.append(weighted_jacobian * scales[col])
        weighted_matrix.append(output_row)
    return weighted_residual, weighted_matrix


@dataclass
class Factor:
    a: list[list[float]]
    b: list[list[float]]
    v: list[list[float]]
    sigma: list[float]
    order: list[int]
    sweeps: int
    rotations: int


def column_products(matrix: list[list[float]], left: int, right: int) -> tuple[float, float, float]:
    alpha = 0.0
    beta = 0.0
    gamma = 0.0
    for row in matrix:
        left_value = row[left]
        right_value = row[right]
        alpha += left_value * left_value
        beta += right_value * right_value
        gamma += left_value * right_value
    return alpha, beta, gamma


def rotate_columns(
    matrix: list[list[float]], left: int, right: int, cosine: float, sine: float
) -> None:
    for row in matrix:
        left_value = row[left]
        right_value = row[right]
        row[left] = cosine * left_value - sine * right_value
        row[right] = sine * left_value + cosine * right_value


def jacobi_factor(
    matrix: list[list[float]], free_ids: list[int], completed_sweeps: int
) -> Factor:
    b = [row[:] for row in matrix]
    a = [row[:] for row in b]
    cols = len(free_ids)
    if cols == 0 or completed_sweeps >= JACOBI_SWEEP_CAP:
        raise ValueError("invalid or exhausted Jacobi input")
    v = [[0.0 for _ in range(cols)] for _ in range(cols)]
    for index in range(cols):
        v[index][index] = 1.0
    rotations = 0
    sweeps = completed_sweeps
    last_sweep_rotated = False
    for sweep in range(completed_sweeps + 1, JACOBI_SWEEP_CAP + 1):
        any_rotation = False
        for left in range(cols):
            for right in range(left + 1, cols):
                alpha, beta, gamma = column_products(b, left, right)
                product = alpha * beta
                threshold = JACOBI_ROTATE_SCALE * math.sqrt(product)
                if abs(gamma) > threshold:
                    numerator = beta - alpha
                    denominator = 2.0 * gamma
                    tau = numerator / denominator
                    hypot = rust_hypot(tau, 1.0)
                    divisor = abs(tau) + hypot
                    tangent = 1.0 if tau == 0.0 else math.copysign(1.0, tau) / divisor
                    cosine = 1.0 / rust_hypot(tangent, 1.0)
                    sine = cosine * tangent
                    rotate_columns(b, left, right, cosine, sine)
                    rotate_columns(v, left, right, cosine, sine)
                    rotations += 1
                    any_rotation = True
        sweeps = sweep
        last_sweep_rotated = any_rotation
        if not any_rotation:
            break
    if sweeps == JACOBI_SWEEP_CAP and last_sweep_rotated:
        raise ValueError("Jacobi no convergence")
    sigma: list[float] = []
    for col in range(cols):
        total = 0.0
        for row in b:
            total += row[col] * row[col]
        sigma.append(math.sqrt(total))
    largest = max(sigma, default=0.0)
    if largest == 0.0 or any(value <= RANK_SCALE * largest for value in sigma):
        raise ValueError("rank deficient")
    order = sorted(range(cols), key=lambda index: (-sigma[index], free_ids[index]))
    return Factor(a=a, b=b, v=v, sigma=sigma, order=order, sweeps=sweeps, rotations=rotations)


def lambda_step(factor: Factor, residual: list[float], lam: float) -> list[float]:
    cols = len(factor.sigma)
    coefficients = [0.0] * cols
    for sorted_index in range(cols):
        j = factor.order[sorted_index]
        sigma = factor.sigma[j]
        uty = 0.0
        for row, residual_value in enumerate(residual):
            av = 0.0
            for free in range(cols):
                av += factor.a[row][free] * factor.v[free][j]
            u = av / sigma
            uty += u * residual_value
        denominator = sigma * sigma + lam
        gain = sigma / denominator
        coefficients[j] = -(gain * uty)
    step = [0.0] * cols
    for output in range(cols):
        total = 0.0
        for sorted_index in range(cols):
            j = factor.order[sorted_index]
            total += factor.v[output][j] * coefficients[j]
        step[output] = total
    return step


def lambda_norm(step: list[float]) -> float:
    return math.sqrt(checked_square_sum(step))


def solve_ball(factor: Factor, residual: list[float], radius: float) -> tuple[float, list[float], float]:
    zero = lambda_step(factor, residual, 0.0)
    zero_norm = lambda_norm(zero)
    if zero_norm <= radius:
        return 0.0, zero, zero_norm
    low = 0.0
    high = 1.0
    high_step = lambda_step(factor, residual, high)
    high_norm = lambda_norm(high_step)
    for _ in range(48):
        if high_norm <= radius:
            break
        low = high
        high *= 4.0
        high_step = lambda_step(factor, residual, high)
        high_norm = lambda_norm(high_step)
    if high_norm > radius:
        raise ValueError("lambda bracket failure")
    for _ in range(48):
        middle = (low + high) * 0.5
        step = lambda_step(factor, residual, middle)
        norm = lambda_norm(step)
        if norm <= radius:
            high = middle
            high_step = step
        else:
            low = middle
    norm = lambda_norm(high_step)
    gap = radius - norm
    tolerance = max((2.0**-40) * radius, 64.0 * EPSILON * max(radius, 1.0))
    if norm > radius or gap > tolerance:
        raise ValueError("ball accuracy failure")
    return high, high_step, norm


def prepare_face(
    initial_radius: float,
    step: list[float],
    lower_active: list[bool],
    upper_active: list[bool],
    free: list[bool],
    weighted_residual: list[float],
    weighted_matrix: list[list[float]],
) -> tuple[float, list[int], list[float], list[list[float]]]:
    active_square_sum = 0.0
    for col in range(N):
        if lower_active[col] or upper_active[col]:
            active_square_sum += step[col] * step[col]
    radius_squared = initial_radius * initial_radius - active_square_sum
    if radius_squared <= 0.0 or not any(free):
        raise ValueError("degenerate face")
    free_ids = [index for index in range(N) if free[index]]
    residual: list[float] = []
    for row in range(N):
        active_product_sum = 0.0
        for col in range(N):
            if lower_active[col] or upper_active[col]:
                active_product_sum += weighted_matrix[row][col] * step[col]
        residual.append(weighted_residual[row] + active_product_sum)
    matrix = [[weighted_matrix[row][col] for col in free_ids] for row in range(N)]
    return math.sqrt(radius_squared), free_ids, residual, matrix


def crossing_candidates(
    candidate: list[float], previous: list[float], free_ids: list[int], lower: list[float], upper: list[float]
) -> list[tuple[float, int, bool]]:
    candidates: list[tuple[float, int, bool]] = []
    for col in free_ids:
        if candidate[col] < lower[col]:
            bound, is_lower = lower[col], True
        elif candidate[col] > upper[col]:
            bound, is_lower = upper[col], False
        else:
            continue
        theta = (bound - previous[col]) / (candidate[col] - previous[col])
        candidates.append((theta, col, is_lower))
    return candidates


def select_crossing(candidates: list[tuple[float, int, bool]]) -> Optional[tuple[float, int, bool]]:
    selected: Optional[tuple[float, int, bool]] = None
    for candidate in candidates:
        theta, col, _ = candidate
        if selected is None or theta < selected[0] or (theta == selected[0] and col < selected[1]):
            selected = candidate
    return selected


def kkt_values(
    weighted_matrix: list[list[float]], weighted_residual: list[float], step: list[float], lam: float
) -> tuple[list[float], list[float], list[float], list[float]]:
    residual = [0.0] * N
    gradient = [0.0] * N
    lambda_step_values = [0.0] * N
    h = [0.0] * N
    for row in range(N):
        ap_sum = 0.0
        for col in range(N):
            ap_sum += weighted_matrix[row][col] * step[col]
        residual[row] = weighted_residual[row] + ap_sum
    for col in range(N):
        for row in range(N):
            gradient[col] += weighted_matrix[row][col] * residual[row]
        lambda_step_values[col] = lam * step[col]
        h[col] = gradient[col] + lambda_step_values[col]
    return residual, gradient, lambda_step_values, h


def release_candidates(
    lower_active: list[bool], upper_active: list[bool], h: list[float]
) -> list[tuple[float, int, bool]]:
    result: list[tuple[float, int, bool]] = []
    for col in range(N):
        if lower_active[col] and h[col] < 0.0:
            result.append((-h[col], col, True))
        elif upper_active[col] and h[col] > 0.0:
            result.append((h[col], col, False))
    return result


def select_release(candidates: list[tuple[float, int, bool]]) -> Optional[tuple[int, bool]]:
    selected: Optional[tuple[float, int, bool]] = None
    for candidate in candidates:
        size, col, _ = candidate
        if selected is None or size > selected[0] or (size == selected[0] and col < selected[1]):
            selected = candidate
    return None if selected is None else (selected[1], selected[2])


def max_feasibility_violation(step: list[float], lower: list[float], upper: list[float]) -> float:
    maximum = 0.0
    for col in range(N):
        if step[col] < lower[col]:
            maximum = max(maximum, lower[col] - step[col])
        elif step[col] > upper[col]:
            maximum = max(maximum, step[col] - upper[col])
    return maximum


def mismatch(report: list[dict[str, Any]], face: int, field: str, expected: Any, observed: Any) -> None:
    report.append({"face": face, "field": field, "calculated": expected, "recorded": observed})


def compare_vector(
    mismatches: list[dict[str, Any]], face: int, field: str, calculated: list[float], encoded: Any
) -> None:
    if encoded is None:
        mismatch(mismatches, face, field, encode_vector(calculated), None)
        return
    observed = decode_vector(encoded, len(calculated))
    if not exact_vector(calculated, observed):
        differing = [index for index, (a, b) in enumerate(zip(calculated, observed)) if not bits_equal(a, b)]
        mismatch(mismatches, face, field, encode_vector(calculated), {"bits": encoded, "indices": differing})


def load_capture(path: Path) -> dict[str, Any]:
    text = path.read_text(encoding="utf-8")
    prefix = "FACE_PIVOT_CAPTURE "
    selected = next((line[len(prefix) :] for line in text.splitlines() if line.startswith(prefix)), text.strip())
    value = json.loads(selected)
    if not isinstance(value, dict):
        raise ValueError("capture must be a JSON object")
    return value


def audit_admission(capture: dict[str, Any]) -> list[dict[str, Any]]:
    output: list[dict[str, Any]] = []
    temperature_ids = [0, 1, 2, 6, 7, 8, 12, 14, 15, 16, 17, 18, 19, 20]
    for index, item in enumerate(capture.get("post_update_admissions", [])):
        previous = decode_vector(item["previous_coordinates"], N)
        current = decode_vector(item["current_coordinates"], N)
        raw = decode_vector(item["current_raw"], N)
        normalizers = decode_vector(item["current_normalizers"], N)
        previous_wet = [None if x is None else from_bits(x) for x in item["previous_wet_temperature_k"]]
        current_wet = [None if x is None else from_bits(x) for x in item["current_wet_temperature_k"]]
        previous_hydraulics = None if item["previous_hydraulics"] is None else decode_vector(item["previous_hydraulics"])
        current_hydraulics = None if item["current_hydraulics"] is None else decode_vector(item["current_hydraulics"])
        previous_beta = decode_vector(item["previous_beta"], 4)
        current_beta = decode_vector(item["current_beta"], 4)
        previous_ci = decode_vector(item["previous_ci_pa"], 4)
        current_ci = decode_vector(item["current_ci_pa"], 4)
        ratios = [abs(raw[i] / normalizers[i]) for i in range(N)]
        current_max = max([0.0, *ratios])
        complete = (
            previous_hydraulics is not None
            and current_hydraulics is not None
            and all((a is None) == (b is None) for a, b in zip(previous_wet, current_wet, strict=True))
        )
        row: dict[str, Any] = {
            "index": index,
            "normalized_ratios": encode_vector(ratios),
            "current_max": bits(current_max),
            "recorded_current_max_exact": bits_equal(current_max, from_bits(item["current_max"])),
            "complete": complete,
            "governed_evaluation": "evaluated" if current_max <= 1.0 else "short-circuit-skipped",
        }
        if complete and current_max <= 1.0:
            temperature_values = [current[i] - previous[i] for i in temperature_ids]
            temperature_values.extend(
                0.0 if a is None else a - b
                for a, b in zip(current_wet, previous_wet, strict=True)
            )
            temperature = finite_max(temperature_values)
            humidity = abs(current[13] - previous[13])
            hydraulic = finite_max(a - b for a, b in zip(current_hydraulics, previous_hydraulics, strict=True))
            beta = finite_max(a - b for a, b in zip(current_beta, previous_beta, strict=True))
            ci = [a - b for a, b in zip(current_ci, previous_ci, strict=True)]
            thresholds = decode_vector(item["thresholds"], 4)
            passed = (
                temperature <= thresholds[0]
                and humidity <= thresholds[1]
                and hydraulic <= thresholds[2]
                and beta <= thresholds[3]
                and all(math.isfinite(value) for value in ci)
            )
            admitted = current_max <= 1.0 and passed
            row.update(
                temperature=bits(temperature), humidity=bits(humidity), hydraulic=bits(hydraulic),
                beta=bits(beta), ci=encode_vector(ci), thresholds=encode_vector(thresholds),
                governed_pass=passed, admitted=admitted,
            )
            recorded_operands = {
                "temperature": item.get("temperature"), "humidity": item.get("humidity"),
                "hydraulic": item.get("hydraulic"), "beta": item.get("beta"), "ci": item.get("ci"),
                "pass": item.get("pass"), "admitted": item.get("admitted"),
            }
            calculated_operands = {
                "temperature": bits(temperature), "humidity": bits(humidity), "hydraulic": bits(hydraulic),
                "beta": bits(beta), "ci": encode_vector(ci), "pass": passed, "admitted": admitted,
            }
            row["recorded_operands_exact"] = recorded_operands == calculated_operands
        elif current_max > 1.0:
            row["recorded_skip_exact"] = all(
                item.get(field) is None for field in ("temperature", "humidity", "hydraulic", "beta", "ci", "pass")
            ) and item.get("admitted") is False
        output.append(row)
    return output


def reconstruct(capture: dict[str, Any]) -> dict[str, Any]:
    base = decode_vector(capture["base"], N)
    raw = decode_vector(capture["raw_residual"], N)
    jacobian = [decode_vector(row, N) for row in capture["raw_jacobian"]]
    normalizers = decode_vector(capture["normalizers"], N)
    scales = decode_vector(capture["scales"], N)
    physical_lower = decode_vector(capture["lower"], N)
    physical_upper = decode_vector(capture["upper"], N)
    initial_radius = from_bits(capture["initial_radius"])
    recorded_f = decode_vector(capture["weighted_residual"], N)
    recorded_a = [decode_vector(row, N) for row in capture["weighted_matrix"]]
    weighted_residual, weighted_matrix = weighted_system(raw, jacobian, normalizers, scales)
    system_checks = {
        "weighted_residual_exact": exact_vector(weighted_residual, recorded_f),
        "weighted_matrix_exact": all(exact_vector(a, b) for a, b in zip(weighted_matrix, recorded_a, strict=True)),
    }
    scaled_lower = [(physical_lower[i] - base[i]) / scales[i] for i in range(N)]
    scaled_upper = [(physical_upper[i] - base[i]) / scales[i] for i in range(N)]
    step = [0.0] * N
    lower_active = [False] * N
    upper_active = [False] * N
    free = [physical_lower[i] != physical_upper[i] for i in range(N)]
    entered = 0
    pivots = 0
    sweeps = 0
    observed_operations = capture.get("operations", [])
    calculated_operations: list[dict[str, Any]] = []
    mismatches: list[dict[str, Any]] = []
    face_occurrences: dict[tuple[tuple[bool, ...], tuple[bool, ...]], list[dict[str, Any]]] = {}
    terminal = "unknown"
    face_index = 0
    failed_system_factor_sweep_sum = 0
    while face_index <= PIVOT_CAP:
        previous = step[:]
        lower_before = lower_active[:]
        upper_before = upper_active[:]
        free_before = free[:]
        radius, free_ids, face_residual, face_matrix = prepare_face(
            initial_radius, step, lower_active, upper_active, free, weighted_residual, weighted_matrix
        )
        entered += 1
        # Physical controller inputs use the default test-work seed (zero) for
        # every factorization.  `state.completed_sweeps` is a reported maximum;
        # it is not fed back as the next factorization's starting sweep.
        factor = jacobi_factor(face_matrix, free_ids, 0)
        failed_system_factor_sweep_sum += factor.sweeps
        sweeps = max(sweeps, factor.sweeps)
        lam, free_step, free_norm = solve_ball(factor, face_residual, radius)
        candidate = step[:]
        for slot, col in enumerate(free_ids):
            candidate[col] = free_step[slot]
        face_key = (tuple(lower_before), tuple(upper_before))
        face_occurrences.setdefault(face_key, []).append(
            {"face": face_index, "p_before": encode_vector(previous), "candidate": encode_vector(candidate),
             "radius": bits(radius), "lambda": bits(lam)}
        )
        crossing_set = crossing_candidates(candidate, previous, free_ids, scaled_lower, scaled_upper)
        crossing = select_crossing(crossing_set)
        operation: dict[str, Any] = {
            "face": face_index,
            "free_ids": free_ids,
            "radius": bits(radius),
            "lambda": bits(lam),
            "free_norm": bits(free_norm),
            "p_before": encode_vector(previous),
            "p_candidate": encode_vector(candidate),
            "candidate_feasibility_violation": bits(max_feasibility_violation(candidate, scaled_lower, scaled_upper)),
            "entered": entered,
            "sweeps": sweeps,
            "pivots_before": pivots,
            "jacobi_rotations": factor.rotations,
            "factor_sweeps": factor.sweeps,
        }
        if crossing is not None:
            theta, col, is_lower = crossing
            tied = [item[1] for item in crossing_set if item[0] == theta]
            operation.update(action="activate-cap" if pivots >= PIVOT_CAP else "activate", theta=bits(theta),
                             coordinate=col, lower=is_lower, exact_theta_ties=tied)
            if pivots >= PIVOT_CAP:
                step = candidate[:]
                operation["applied"] = False
                operation["conceptual_p_after"] = encode_vector(previous)
                operation["scratch_p_at_refusal"] = encode_vector(step)
                terminal = "FacePivotLimit:activate-first-crossing"
            else:
                step = candidate[:]
                for free_id in free_ids:
                    step[free_id] = previous[free_id] + theta * (step[free_id] - previous[free_id])
                step[col] = scaled_lower[col] if is_lower else scaled_upper[col]
                free[col] = False
                lower_active[col] = is_lower
                upper_active[col] = not is_lower
                pivots += 1
                operation["applied"] = True
                operation["p_after"] = encode_vector(step)
                operation["post_feasibility_violation"] = bits(max_feasibility_violation(step, scaled_lower, scaled_upper))
        else:
            # Runtime kkt_transition is entered after the free-face values have
            # been copied into state.p.  Its p_before therefore equals the face
            # candidate, not the state at face entry.
            step = candidate[:]
            operation["p_before"] = encode_vector(step)
            residual, gradient, lambda_times_step, h = kkt_values(weighted_matrix, weighted_residual, step, lam)
            release_set = release_candidates(lower_active, upper_active, h)
            release = select_release(release_set)
            operation.update(g=encode_vector(gradient), lambda_times_p=encode_vector(lambda_times_step),
                             h=encode_vector(h), linear_residual=encode_vector(residual))
            if release is None:
                operation.update(action="return", applied=True, p_after=encode_vector(step))
                terminal = "return"
            else:
                col, is_lower = release
                selected_size = -h[col] if is_lower else h[col]
                tied = [item[1] for item in release_set if item[0] == selected_size]
                operation.update(action="release-cap" if pivots >= PIVOT_CAP else "release",
                                 coordinate=col, lower=is_lower, exact_violation_ties=tied)
                if pivots >= PIVOT_CAP:
                    operation["applied"] = False
                    operation["conceptual_p_after"] = encode_vector(step)
                    operation["scratch_p_at_refusal"] = encode_vector(step)
                    terminal = "FacePivotLimit:kkt-release"
                else:
                    lower_active[col] = False
                    upper_active[col] = False
                    free[col] = True
                    pivots += 1
                    operation["applied"] = True
                    operation["p_after"] = encode_vector(step)
        operation.update(
            lower_before=lower_before, upper_before=upper_before, free_before=free_before,
            lower_after=lower_active[:], upper_after=upper_active[:], free_after=free[:], pivots_after=pivots,
        )
        calculated_operations.append(operation)
        if face_index < len(observed_operations):
            observed = observed_operations[face_index]
            for field, calculated in (("action", operation["action"]), ("applied", operation["applied"]),
                                      ("free_ids", free_ids), ("coordinate", operation.get("coordinate")),
                                      ("lower", operation.get("lower")), ("entered", entered),
                                      ("pivots", pivots), ("sweeps", sweeps)):
                if observed.get(field) != calculated:
                    mismatch(mismatches, face_index, field, calculated, observed.get(field))
            for field, calculated in (("radius", radius), ("lambda", lam), ("theta", crossing[0] if crossing else None)):
                recorded = decode_optional(observed.get(field))
                if (calculated is None) != (recorded is None) or (
                    calculated is not None and recorded is not None and not bits_equal(calculated, recorded)
                ):
                    mismatch(mismatches, face_index, field, None if calculated is None else bits(calculated), observed.get(field))
            observed_p_before = previous if crossing is not None else candidate
            compare_vector(mismatches, face_index, "p_before", observed_p_before, observed.get("p_before"))
            compare_vector(mismatches, face_index, "p_candidate", candidate, observed.get("p_candidate"))
            recorded_after_name = "conceptual_p_after" if "conceptual_p_after" in observed else "p_after"
            expected_after = previous if operation["action"] == "activate-cap" else step
            compare_vector(mismatches, face_index, recorded_after_name, expected_after, observed.get(recorded_after_name))
            expected_scratch = candidate if operation["action"] == "activate-cap" else (
                step if operation["action"] == "release-cap" else None
            )
            recorded_scratch = observed.get("scratch_p_at_refusal")
            if expected_scratch is None:
                if recorded_scratch is not None:
                    mismatch(mismatches, face_index, "scratch_p_at_refusal", None, recorded_scratch)
            else:
                compare_vector(
                    mismatches, face_index, "scratch_p_at_refusal", expected_scratch, recorded_scratch
                )
            for field, calculated in (("lower_before", lower_before), ("upper_before", upper_before),
                                      ("free_before", free_before), ("lower_after", lower_active),
                                      ("upper_after", upper_active), ("free_after", free)):
                if observed.get(field) != calculated:
                    mismatch(mismatches, face_index, field, calculated, observed.get(field))
            if crossing is None:
                compare_vector(mismatches, face_index, "g", gradient, observed.get("g"))
                compare_vector(mismatches, face_index, "lambda_times_p", lambda_times_step, observed.get("lambda_times_p"))
                compare_vector(mismatches, face_index, "h", h, observed.get("h"))
        else:
            mismatch(mismatches, face_index, "operation", operation, None)
        if terminal != "unknown":
            break
        face_index += 1
    if len(observed_operations) > len(calculated_operations):
        mismatch(mismatches, len(calculated_operations), "extra_recorded_operations", None,
                 len(observed_operations) - len(calculated_operations))
    refusal = capture.get("refusal")
    refusal_check: dict[str, Any] = {"present": refusal is not None}
    if refusal is not None:
        refusal_check.update(
            stage=refusal.get("stage"), kind=refusal.get("kind"), entered=refusal.get("entered"),
            pivots=refusal.get("pivots"), sweeps=refusal.get("sweeps"),
            scratch_p_exact=exact_vector(step, decode_vector(refusal["p"], N)),
            masks_exact=(refusal.get("lower") == lower_active and refusal.get("upper") == upper_active
                         and refusal.get("free") == free),
        )
    summaries = capture.get("completed_subproblems", [])
    summary_totals = {
        "subproblems": len(summaries),
        "successful": sum(not bool(item.get("refused")) for item in summaries),
        "refused": sum(bool(item.get("refused")) for item in summaries),
        "entered_factorizations": sum(int(item["entered"]) for item in summaries),
        "completed_pivots": sum(int(item["pivots"]) for item in summaries),
        "per_subproblem_max_sweep_indices": [int(item["sweeps"]) for item in summaries],
        "whole_factor_sweep_sum": "not derivable from retained successful-subproblem summaries",
        "final_summary_matches_local": bool(summaries)
        and summaries[-1]
        == {"entered": entered, "pivots": pivots, "sweeps": sweeps, "refused": terminal.startswith("FacePivotLimit")},
    }
    repeated = []
    for occurrences in face_occurrences.values():
        if len(occurrences) > 1:
            distinct_states = len({(tuple(item["p_before"]), tuple(item["candidate"]), item["radius"], item["lambda"])
                                   for item in occurrences})
            repeated.append({"faces": [item["face"] for item in occurrences],
                             "occurrences": len(occurrences), "distinct_numeric_states": distinct_states,
                             "records": occurrences})
    return {
        "evidence_class": "offline ordered-binary64 reconstruction",
        "terminal_tree": TERMINAL_TREE,
        "host_hypot": "libc" if _c_hypot is not None else "python-math-fallback",
        "system_checks": system_checks,
        "terminal": terminal,
        "calculated_local_counters": {"entered_factorizations": entered, "completed_pivots": pivots,
                                      "max_completed_sweep_index": sweeps,
                                      "offline_sum_of_factor_sweeps_for_failed_system": failed_system_factor_sweep_sum},
        "recorded_whole_stage1_counters": summary_totals,
        "refusal_check": refusal_check,
        "exact_record_mismatches": mismatches,
        "operations": calculated_operations,
        "repeated_faces": repeated,
        "admission": audit_admission(capture),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("capture", type=Path, help="JSON capture or stderr containing FACE_PIVOT_CAPTURE")
    parser.add_argument("--output", type=Path, help="write report atomically to this path")
    args = parser.parse_args()
    report = reconstruct(load_capture(args.capture))
    encoded = json.dumps(report, indent=2, sort_keys=True, allow_nan=False) + "\n"
    if args.output is None:
        sys.stdout.write(encoded)
    else:
        temporary = args.output.with_suffix(args.output.suffix + ".tmp")
        temporary.write_text(encoded, encoding="utf-8")
        temporary.replace(args.output)
    return 0 if not report["exact_record_mismatches"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
