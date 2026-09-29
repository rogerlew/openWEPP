#!/usr/bin/env python3
"""Independent G4 captured-matrix rank and spectrum verifier.

This reviewer-owned program consumes the one authorized physical capture.  It
does not import the author calculator, reconstruct a nearby fixture, or supply
anything to the runtime solver.  It treats every captured binary64 matrix entry
as an exact rational, builds checkable exact/modular rank certificates, and
uses direct high-precision SVD at the prospectively fixed 80 and 160 dps.
"""

from __future__ import annotations

import argparse
from fractions import Fraction
import hashlib
import json
import math
from pathlib import Path
import signal
import struct
import sys
from typing import Any, Iterable

import mpmath as mp


PREFIX = "SHIFTED_FACE_ATTRIBUTION_CAPTURE "
PRIMES = (2_305_843_009_213_693_951, 1_000_000_007, 1_000_000_009)

COORDINATES = (
    "upper_sun_temperature_k",
    "upper_shade_temperature_k",
    "upper_stem_temperature_k",
    "upper_reservoir_mass_kg_m2",
    "upper_reservoir_enthalpy_j_m2",
    "upper_drainage_kg_m2_s",
    "lower_sun_temperature_k",
    "lower_shade_temperature_k",
    "lower_stem_temperature_k",
    "lower_reservoir_mass_kg_m2",
    "lower_reservoir_enthalpy_j_m2",
    "lower_drainage_kg_m2_s",
    "canopy_air_temperature_k",
    "canopy_air_specific_humidity_kg_kg",
    "snow_surface_temperature_k",
    "soil_node_0_temperature_k",
    "soil_node_1_temperature_k",
    "soil_node_2_temperature_k",
    "soil_node_3_temperature_k",
    "soil_node_4_temperature_k",
    "soil_node_5_temperature_k",
)

ROWS = (
    "upper_sun_energy",
    "upper_shade_energy",
    "upper_stem_energy",
    "upper_reservoir_mass_closure",
    "upper_reservoir_enthalpy_closure",
    "upper_capacity_drainage_closure",
    "lower_sun_energy",
    "lower_shade_energy",
    "lower_stem_energy",
    "lower_reservoir_mass_closure",
    "lower_reservoir_enthalpy_closure",
    "lower_capacity_drainage_closure",
    "canopy_air_heat_closure",
    "canopy_air_vapor_closure",
    "snow_surface_temperature_anchor",
    "soil_node_0_temperature_anchor",
    "soil_node_1_temperature_anchor",
    "soil_node_2_temperature_anchor",
    "soil_node_3_temperature_anchor",
    "soil_node_4_temperature_anchor",
    "soil_node_5_temperature_anchor",
)


def bits_to_float(value: str | int) -> float:
    bits = int(value, 16) if isinstance(value, str) else int(value)
    return struct.unpack(">d", bits.to_bytes(8, "big", signed=False))[0]


def bits_to_fraction(value: str | int) -> Fraction:
    number = bits_to_float(value)
    if not math.isfinite(number):
        raise ValueError(f"nonfinite captured operand {value!r}")
    numerator, denominator = number.as_integer_ratio()
    return Fraction(numerator, denominator)


def fraction_text(value: Fraction) -> str:
    return str(value.numerator) if value.denominator == 1 else f"{value.numerator}/{value.denominator}"


def mp_text(value: mp.mpf, digits: int = 50) -> str:
    return mp.nstr(value, digits, strip_zeros=False)


def read_capture(path: Path) -> dict[str, Any]:
    raw = path.read_bytes()
    text = raw.decode("utf-8", errors="strict")
    stripped = text.strip()
    if stripped.startswith("{"):
        value = json.loads(stripped)
    else:
        records = [json.loads(line.split(PREFIX, 1)[1]) for line in text.splitlines() if PREFIX in line]
        if len(records) != 1:
            raise ValueError(f"expected exactly one {PREFIX.strip()} record, found {len(records)}")
        value = records[0]
    if value.get("observer") != "g4-natural-tangent-rank-attribution":
        raise ValueError("input is not the authorized G4 rank-attribution record")
    value["_input_sha256"] = hashlib.sha256(raw).hexdigest()
    return value


def matrix_fraction(bits: list[list[str]]) -> list[list[Fraction]]:
    if len(bits) != 21 or any(len(row) != 21 for row in bits):
        raise ValueError("expected one 21 by 21 matrix")
    return [[bits_to_fraction(value) for value in row] for row in bits]


def vector_fraction(bits: list[str]) -> list[Fraction]:
    if len(bits) != 21:
        raise ValueError("expected one 21-vector")
    return [bits_to_fraction(value) for value in bits]


def proportional_groups(vectors: list[list[Fraction]]) -> list[list[int]]:
    groups: dict[tuple[Fraction, ...], list[int]] = {}
    for index, vector in enumerate(vectors):
        pivot = next((value for value in vector if value), None)
        if pivot is None:
            signature = tuple(Fraction(0) for _ in vector)
        else:
            signature = tuple(value / pivot for value in vector)
        groups.setdefault(signature, []).append(index)
    return [indices for indices in groups.values() if len(indices) > 1]


def sparse_support(vector: Iterable[Fraction]) -> list[int]:
    return [index for index, value in enumerate(vector) if value]


def exact_relations(
    vectors: list[list[Fraction]],
    names: tuple[str, ...],
    residual: list[Fraction] | None,
) -> list[dict[str, Any]]:
    relations: list[dict[str, Any]] = []
    width = len(vectors)
    zero_indices = [index for index, vector in enumerate(vectors) if not any(vector)]
    for index in zero_indices:
        coeffs = [Fraction(0) for _ in range(width)]
        coeffs[index] = Fraction(1)
        relations.append(_relation_record(coeffs, vectors, names, residual, "zero-vector"))
    for group in proportional_groups(vectors):
        first = group[0]
        pivot = next((i for i, value in enumerate(vectors[first]) if value), None)
        if pivot is None:
            continue
        for other in group[1:]:
            coeffs = [Fraction(0) for _ in range(width)]
            coeffs[first] = vectors[other][pivot]
            coeffs[other] = -vectors[first][pivot]
            relations.append(_relation_record(coeffs, vectors, names, residual, "exact-proportional"))
    return relations


def _relation_record(
    coefficients: list[Fraction],
    vectors: list[list[Fraction]],
    names: tuple[str, ...],
    residual: list[Fraction] | None,
    kind: str,
) -> dict[str, Any]:
    combined = [sum((coefficients[i] * vectors[i][j] for i in range(len(vectors))), Fraction(0)) for j in range(len(vectors[0]))]
    if any(combined):
        raise AssertionError("invalid exact relation")
    active = [
        {"index": i, "name": names[i], "coefficient": fraction_text(value)}
        for i, value in enumerate(coefficients)
        if value
    ]
    record: dict[str, Any] = {"kind": kind, "terms": active, "product_exactly_zero": True}
    if residual is not None:
        projection = sum((coefficients[i] * residual[i] for i in range(len(coefficients))), Fraction(0))
        record["weighted_residual_projection"] = fraction_text(projection)
    return record


def mod_value(value: Fraction, prime: int) -> int:
    denominator = value.denominator % prime
    if denominator == 0:
        raise ValueError("certificate prime divides a denominator")
    return (value.numerator % prime) * pow(denominator, -1, prime) % prime


def determinant_mod(square: list[list[int]], prime: int) -> int:
    work = [row[:] for row in square]
    det = 1
    for col in range(len(work)):
        pivot = next((row for row in range(col, len(work)) if work[row][col]), None)
        if pivot is None:
            return 0
        if pivot != col:
            work[col], work[pivot] = work[pivot], work[col]
            det = -det
        pivot_value = work[col][col]
        det = det * pivot_value % prime
        inverse = pow(pivot_value, -1, prime)
        for row in range(col + 1, len(work)):
            multiplier = work[row][col] * inverse % prime
            if multiplier:
                work[row] = [(a - multiplier * b) % prime for a, b in zip(work[row], work[col])]
    return det % prime


def modular_rank_certificate(matrix: list[list[Fraction]], prime: int) -> dict[str, Any]:
    work = [[mod_value(value, prime) for value in row] for row in matrix]
    row_ids = list(range(len(work)))
    pivot_rows: list[int] = []
    pivot_cols: list[int] = []
    rank = 0
    for col in range(len(work[0])):
        pivot = next((row for row in range(rank, len(work)) if work[row][col]), None)
        if pivot is None:
            continue
        work[rank], work[pivot] = work[pivot], work[rank]
        row_ids[rank], row_ids[pivot] = row_ids[pivot], row_ids[rank]
        pivot_rows.append(row_ids[rank])
        pivot_cols.append(col)
        inverse = pow(work[rank][col], -1, prime)
        work[rank] = [value * inverse % prime for value in work[rank]]
        for row in range(len(work)):
            if row != rank and work[row][col]:
                multiplier = work[row][col]
                work[row] = [(a - multiplier * b) % prime for a, b in zip(work[row], work[rank])]
        rank += 1
        if rank == len(work):
            break
    minor = [[mod_value(matrix[row][col], prime) for col in pivot_cols] for row in pivot_rows]
    determinant = determinant_mod(minor, prime)
    if determinant == 0:
        raise AssertionError("selected modular pivot minor unexpectedly vanishes")
    return {
        "prime": prime,
        "rank_lower_bound": rank,
        "pivot_rows": pivot_rows,
        "pivot_row_names": [ROWS[row] for row in pivot_rows],
        "pivot_columns": pivot_cols,
        "pivot_column_names": [COORDINATES[col] for col in pivot_cols],
        "pivot_minor_determinant_residue": determinant,
    }


def f64_weighting_checks(face: dict[str, Any]) -> dict[str, Any]:
    raw = face["raw_residual"]
    jacobian = face["raw_jacobian"]
    normalizers = face["normalizers"]
    scales = face["scales"]
    weighted_f = face["weighted_residual"]
    weighted_a = face["weighted_matrix"]
    residual_mismatches: list[int] = []
    matrix_mismatches: list[list[int]] = []
    for row in range(21):
        weight = 1.0 / bits_to_float(normalizers[row])
        observed_f = weight * bits_to_float(raw[row])
        if struct.pack(">d", observed_f).hex() != weighted_f[row]:
            residual_mismatches.append(row)
        for col in range(21):
            coefficient = (weight * bits_to_float(jacobian[row][col])) * bits_to_float(scales[col])
            if struct.pack(">d", coefficient).hex() != weighted_a[row][col]:
                matrix_mismatches.append([row, col])
    return {
        "runtime_operation_order": "weight=1/normalizer; weighted_jacobian=weight*J; A=weighted_jacobian*scale",
        "weighted_residual_bit_mismatches": residual_mismatches,
        "weighted_matrix_bit_mismatches": matrix_mismatches,
        "exact_bit_match": not residual_mismatches and not matrix_mismatches,
    }


def normalized_components(vector: list[mp.mpf], names: tuple[str, ...], count: int = 8) -> list[dict[str, str | int]]:
    if not vector:
        return []
    pivot = max(range(len(vector)), key=lambda i: abs(vector[i]))
    if vector[pivot] < 0:
        vector = [-value for value in vector]
    selected = sorted(range(len(vector)), key=lambda i: (-abs(vector[i]), i))[:count]
    return [
        {"index": i, "name": names[i], "coefficient": mp_text(vector[i])}
        for i in selected
    ]


def high_precision_svd(
    matrix: list[list[Fraction]],
    residual: list[Fraction],
    runtime_sigma: list[Fraction],
    dps: int,
) -> dict[str, Any]:
    with mp.workdps(dps):
        a = mp.matrix([[mp.mpf(value.numerator) / value.denominator for value in row] for row in matrix])
        f = mp.matrix([mp.mpf(value.numerator) / value.denominator for value in residual])
        u, singular, v = mp.svd(a)
        values = [singular[i] for i in range(len(singular))]
        runtime_sorted = sorted(
            (mp.mpf(value.numerator) / value.denominator for value in runtime_sigma), reverse=True
        )
        spectrum_comparison = []
        for index, (runtime_value, reference_value) in enumerate(zip(runtime_sorted, values)):
            absolute = abs(runtime_value - reference_value)
            spectrum_comparison.append({
                "descending_index": index,
                "runtime_jacobi": mp_text(runtime_value, min(dps, 90)),
                "direct_svd": mp_text(reference_value, min(dps, 90)),
                "absolute_difference": mp_text(absolute, min(dps, 90)),
                "relative_to_direct": mp_text(absolute / reference_value, min(dps, 90)) if reference_value else None,
            })
        smallest_indices = list(range(max(0, len(values) - 3), len(values)))
        modes = []
        for index in smallest_indices:
            left = [u[row, index] for row in range(21)]
            right = [v[index, col] for col in range(21)]
            left_projection = sum((left[row] * f[row] for row in range(21)), mp.mpf("0"))
            av_norm = mp.sqrt(sum((sum((a[row, col] * right[col] for col in range(21)), mp.mpf("0")) ** 2 for row in range(21)), mp.mpf("0")))
            modes.append({
                "descending_singular_index": index,
                "singular_value": mp_text(values[index], min(dps, 90)),
                "a_times_right_norm": mp_text(av_norm, min(dps, 90)),
                "left_weighted_residual_projection": mp_text(left_projection, min(dps, 90)),
                "dominant_right_coordinates": normalized_components(right, COORDINATES),
                "dominant_left_equations": normalized_components(left, ROWS),
            })
        return {
            "dps": dps,
            "singular_values_descending": [mp_text(value, min(dps, 90)) for value in values],
            "sigma_max": mp_text(values[0], min(dps, 90)),
            "sigma_min": mp_text(values[-1], min(dps, 90)),
            "sigma_min_over_max": mp_text(values[-1] / values[0], min(dps, 90)) if values[0] else None,
            "runtime_jacobi_comparison": spectrum_comparison,
            "smallest_modes": modes,
        }


def vector_norms(matrix: list[list[Fraction]], dps: int = 80) -> dict[str, list[str]]:
    columns = [[matrix[row][col] for row in range(21)] for col in range(21)]
    with mp.workdps(dps):
        def norm(vector: list[Fraction]) -> str:
            squared = sum((value * value for value in vector), Fraction(0))
            value = mp.sqrt(mp.mpf(squared.numerator) / squared.denominator)
            return mp_text(value, 50)

        return {
            "row_l2": [norm(row) for row in matrix],
            "column_l2": [norm(column) for column in columns],
        }


def source_causal_observations(record: dict[str, Any], matrix: list[list[Fraction]]) -> dict[str, Any]:
    core = record.get("base_core_operands")
    occupancies = []
    if isinstance(core, list):
        for index, operand in enumerate(core):
            liquid = bits_to_fraction(operand["liquid_mass_kg_m2"])
            capacity = bits_to_fraction(operand["capacity_kg_m2"])
            wet_fraction = bits_to_fraction(operand["wet_fraction"])
            areas = [bits_to_fraction(value) for value in operand["structural_areas_m2_m2_tile"]]
            occupancies.append({
                "occupancy": "upper" if index == 0 else "lower",
                "liquid_mass_equals_capacity": liquid == capacity,
                "wet_fraction_exactly_one": wet_fraction == 1,
                "zero_structural_area_components": [name for name, area in zip(("sun", "shade", "stem"), areas) if area == 0],
                "structural_area_exact_fractions": [fraction_text(area) for area in areas],
            })
    selected_rows = []
    for row in (0, 1, 2, 5, 6, 7, 8, 11):
        support = sparse_support(matrix[row])
        selected_rows.append({
            "row": row,
            "equation": ROWS[row],
            "nonzero_columns": support,
            "nonzero_coordinate_names": [COORDINATES[col] for col in support],
        })
    return {
        "base_core_occupancies": occupancies,
        "phase_capacity_events": record.get("phase_capacity_events"),
        "prepared_upper_m_h_d_exact_fractions": [
            fraction_text(bits_to_fraction(record["prepared_trial_coordinate_bits"][index]))
            for index in (3, 4, 5)
        ],
        "tie_sensitive_row_support": selected_rows,
        "interpretation_boundary": "Facts above describe the captured natural linearization only; they do not select a directional side or alter the runtime model.",
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("capture", type=Path, help="stderr or JSON containing the one G4 observer record")
    args = parser.parse_args()
    if hasattr(signal, "SIGALRM"):
        signal.signal(signal.SIGALRM, lambda _signum, _frame: (_ for _ in ()).throw(TimeoutError("290-second internal bound")))
        signal.alarm(290)

    record = read_capture(args.capture)
    face = record.get("face_capture")
    if not isinstance(face, dict):
        raise ValueError("G4 record has no face capture")
    refusal = face.get("refusal")
    factor = refusal.get("factor") if isinstance(refusal, dict) else None
    if not isinstance(factor, dict):
        raise ValueError("G4 record has no refused factor capture")
    if refusal.get("kind") != "RankDeficient":
        raise ValueError(f"expected RankDeficient, got {refusal.get('kind')!r}")
    if factor.get("order_computed") is not False or factor.get("order") != []:
        raise ValueError("refused factor incorrectly claims a computed singular ordering")

    weighted_bits = face["weighted_matrix"]
    if factor["a"] != weighted_bits:
        raise ValueError("refused first-face A differs from the captured weighted matrix")
    matrix = matrix_fraction(weighted_bits)
    residual = vector_fraction(face["weighted_residual"])
    columns = [[matrix[row][col] for row in range(21)] for col in range(21)]
    left_relations = exact_relations(matrix, ROWS, residual)
    right_relations = exact_relations(columns, COORDINATES, None)
    modular = [modular_rank_certificate(matrix, prime) for prime in PRIMES]
    lower = max(item["rank_lower_bound"] for item in modular)
    upper = 21 - len(left_relations)
    if lower > upper:
        raise AssertionError("modular lower bound exceeds independent exact-relation upper bound")
    exact_rank = lower if lower == upper else None

    runtime_sigma = [bits_to_fraction(value) for value in factor["sigma"]]
    runtime_max = bits_to_fraction(factor["rank_max"])
    runtime_threshold = bits_to_fraction(factor["rank_threshold"])
    runtime_refusing = [index for index, value in enumerate(runtime_sigma) if value <= runtime_threshold]
    svd_results = [
        high_precision_svd(matrix, residual, runtime_sigma, 80),
        high_precision_svd(matrix, residual, runtime_sigma, 160),
    ]
    result = {
        "schema": "g4-rank-correctness-reference-v1",
        "evidence_class": "independent-offline-calculation-on-captured-binary64-operands",
        "capture_path": str(args.capture.resolve()),
        "capture_sha256": record.pop("_input_sha256"),
        "reviewer_script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "matrix_dimensions": [21, 21],
        "free_ids": refusal["free_ids"],
        "f64_weighting_reconstruction": f64_weighting_checks(face),
        "row_nonzero_counts": [len(sparse_support(row)) for row in matrix],
        "column_nonzero_counts": [len(sparse_support(col)) for col in columns],
        "row_and_column_norms_80dps": vector_norms(matrix),
        "zero_rows": [i for i, row in enumerate(matrix) if not sparse_support(row)],
        "zero_columns": [i for i, col in enumerate(columns) if not sparse_support(col)],
        "exact_proportional_row_groups": proportional_groups(matrix),
        "exact_proportional_column_groups": proportional_groups(columns),
        "exact_left_null_certificates": left_relations,
        "exact_right_null_certificates": right_relations,
        "modular_rank_certificates": modular,
        "exact_rank_upper_bound_from_independent_left_relations": upper,
        "exact_rank_lower_bound_from_nonzero_modular_minor": lower,
        "exact_rank_if_bounds_meet": exact_rank,
        "runtime_rank_comparison": {
            "sigma_unsorted_exact_fractions": [fraction_text(value) for value in runtime_sigma],
            "rank_max_exact_fraction": fraction_text(runtime_max),
            "rank_threshold_exact_fraction": fraction_text(runtime_threshold),
            "threshold_equals_exact_power_of_two_product": runtime_threshold
            == runtime_max * Fraction(1, 2**40),
            "refusing_unsorted_indices": runtime_refusing,
            "rule": "sigma <= 2^-40 * sigma_max",
        },
        "high_precision_svd": svd_results,
        "source_causal_observations": source_causal_observations(record, matrix),
        "claims_excluded": [
            "No reference direction is supplied to the runtime.",
            "Exact rank of this stored binary64 matrix is not asserted as exact rank of every admissible physical derivative.",
            "A left-null residual projection is not proof that the nonlinear model has no root.",
        ],
    }
    if signal.alarm(0):
        pass
    json.dump(result, sys.stdout, indent=2, sort_keys=True)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
