#!/usr/bin/env python3
"""Offline G4 rank attribution from one SHIFTED_FACE_ATTRIBUTION_CAPTURE record.

This tool consumes only captured binary64 operands.  It is deliberately not a
runtime solver and never supplies a direction to the physical experiment.
"""
import argparse
import hashlib
import json
import math
import struct
import sys

import mpmath as mp
import sympy as sp


COORDINATES = [
    "upper_sun_T", "upper_shade_T", "upper_stem_T", "upper_M", "upper_H", "upper_D",
    "lower_sun_T", "lower_shade_T", "lower_stem_T", "lower_M", "lower_H", "lower_D",
    "canopy_T", "qcan", "ground_T", "soil_1_T", "soil_2_T", "soil_3_T", "soil_4_T",
    "soil_5_T", "soil_6_T",
]
RESIDUAL_EQUATIONS = [
    "upper_sun_energy", "upper_shade_energy", "upper_stem_energy", "upper_mass",
    "upper_enthalpy", "upper_capacity", "lower_sun_energy", "lower_shade_energy",
    "lower_stem_energy", "lower_mass", "lower_enthalpy", "lower_capacity",
    "canopy_air_heat", "canopy_air_vapor", "ground_snow_temperature", "soil_1_anchor",
    "soil_2_anchor", "soil_3_anchor", "soil_4_anchor", "soil_5_anchor", "soil_6_anchor",
]


def f64(bits):
    return struct.unpack(">d", int(bits, 16).to_bytes(8, "big"))[0]


def f64bits(value):
    return struct.unpack(">Q", struct.pack(">d", value))[0]


def bits_matrix(values):
    return [[f64(value) for value in row] for row in values]


def rational(value):
    numerator, denominator = value.as_integer_ratio()
    return sp.Rational(numerator, denominator)


def mp_matrix(values):
    return mp.matrix([[mp.mpf(value.as_integer_ratio()[0]) / value.as_integer_ratio()[1] for value in row] for row in values])


def mps(value):
    return mp.nstr(value, n=90)


def sha256_json(value):
    return hashlib.sha256(json.dumps(value, separators=(",", ":"), sort_keys=True).encode()).hexdigest()


def max_abs(values):
    return mps(max((abs(value) for value in values), default=mp.mpf("0")))


def factor_checks(factor):
    """Check copied runtime B/V only; this is not another Jacobi factorization."""
    if not factor:
        return None
    mp.mp.dps = 160
    a = mp_matrix(bits_matrix(factor["a"]))
    b = mp_matrix(bits_matrix(factor["b"]))
    v = mp_matrix(bits_matrix(factor["v"]))
    av = a * v
    identity = mp.eye(v.cols)
    gram = v.T * v
    return {
        "sigma_bits": factor["sigma"],
        "rank_max_bits": factor.get("rank_max"),
        "rank_threshold_bits": factor.get("rank_threshold"),
        "order": factor["order"],
        "order_computed": factor["order_computed"],
        "sweeps": factor["sweeps"],
        "rotations": factor["rotations"],
        "b_minus_a_v_max_abs": max_abs([b[row, col] - av[row, col] for row in range(b.rows) for col in range(b.cols)]),
        "v_transpose_v_minus_i_max_abs": max_abs([gram[row, col] - identity[row, col] for row in range(v.cols) for col in range(v.cols)]),
    }


def spectrum(values, residual, scales, dps):
    mp.mp.dps = dps
    matrix = mp_matrix(values)
    left, singular, right = mp.svd(matrix, compute_uv=True)
    count = min(matrix.rows, matrix.cols)
    right_rows = [[mps(right[row, col]) for col in range(right.cols)] for row in range(right.rows)]
    left_columns = [[mps(left[row, col]) for row in range(left.rows)] for col in range(left.cols)]
    return {
        "dps": dps,
        "singular_values": [mps(singular[index]) for index in range(count)],
        "ratio_min_over_max": mps(singular[count - 1] / singular[0]) if singular[0] else None,
        "right_vectors_rows": right_rows,
        "right_vectors_physical_coordinates": [[mps(right[row, col] * (mp.mpf(scales[col].as_integer_ratio()[0]) / scales[col].as_integer_ratio()[1])) for col in range(right.cols)] for row in range(right.rows)],
        "left_vectors_columns": left_columns,
        "left_weighted_residual_projections": [mps(sum(left[row, col] * mp.mpf(residual[row].as_integer_ratio()[0]) / residual[row].as_integer_ratio()[1] for row in range(left.rows))) for col in range(left.cols)],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("capture", help="JSON object after SHIFTED_FACE_ATTRIBUTION_CAPTURE")
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    record = json.load(open(args.capture, encoding="utf-8"))
    capture_sha256 = hashlib.sha256(open(args.capture, "rb").read()).hexdigest()
    face = record["face_capture"]
    matrix = bits_matrix(face["weighted_matrix"])
    residual = [f64(value) for value in face["weighted_residual"]]
    raw = bits_matrix(face["raw_jacobian"])
    scales = [f64(value) for value in face["scales"]]
    factor = face.get("refusal", {}).get("factor")
    exact = sp.Matrix([[rational(value) for value in row] for row in matrix])
    raw_exact = sp.Matrix([[rational(value) for value in row] for row in raw])
    nullspace = exact.nullspace()
    left_nullspace = exact.T.nullspace()
    normalizers = [f64(bit) for bit in face["normalizers"]]
    raw_residual = [f64(bit) for bit in face["raw_residual"]]
    reconstructed_residual = [(1.0 / normalizers[row]) * raw_residual[row] for row in range(21)]
    reconstructed = [[((1.0 / normalizers[row]) * raw[row][column]) * scales[column] for column in range(21)] for row in range(21)]
    reconstruction_bit_mismatches = [(row, column) for row in range(21) for column in range(21) if f64bits(reconstructed[row][column]) != f64bits(matrix[row][column])]
    residual_bit_mismatches = [row for row in range(21) if f64bits(reconstructed_residual[row]) != f64bits(residual[row])]
    duplicate_rows = [(left, right) for left in range(21) for right in range(left + 1, 21) if matrix[left] == matrix[right]]
    duplicate_columns = [(left, right) for left in range(21) for right in range(left + 1, 21) if [row[left] for row in matrix] == [row[right] for row in matrix]]
    reference_svd = [spectrum(matrix, residual, scales, 80), spectrum(matrix, residual, scales, 160)]
    runtime_sigma = factor.get("sigma", []) if factor else []
    result = {
        "schema": "g4-rank-reference-v1",
        "input_capture_schema": record.get("schema"),
        "capture_sha256": capture_sha256,
        "matrix_shape": [len(matrix), len(matrix[0])],
        "coordinate_order": COORDINATES,
        "residual_equation_order": RESIDUAL_EQUATIONS,
        "raw_jacobian_sha256": sha256_json(face["raw_jacobian"]),
        "weighted_matrix_sha256": sha256_json(face["weighted_matrix"]),
        "weighted_residual_sha256": sha256_json(face["weighted_residual"]),
        "runtime_factor_summary": factor_checks(factor),
        "python": sys.version,
        "sympy": sp.__version__,
        "mpmath": mp.__version__,
        "exact_binary64_rank": exact.rank(),
        "exact_binary64_raw_jacobian_rank": raw_exact.rank(),
        "exact_binary64_nullspace": [[str(value) for value in vector] for vector in nullspace],
        "exact_binary64_left_nullspace": [[str(value) for value in vector] for vector in left_nullspace],
        "exact_right_nullspace_physical_S": [{"coordinates": {COORDINATES[index]: str(vector[index] * rational(scales[index])) for index in range(21)}} for vector in nullspace],
        "exact_left_null_weighted_residual_projection": [{"equation_weights": {RESIDUAL_EQUATIONS[index]: str(vector[index]) for index in range(21)}, "weighted_residual_projection": str(sum(vector[index] * rational(residual[index]) for index in range(21)))} for vector in left_nullspace],
        "exact_right_null_certificates": [[str(value) for value in exact * vector] for vector in nullspace],
        "exact_left_null_certificates": [[str(value) for value in exact.T * vector] for vector in left_nullspace],
        "normalizer_bits": face["normalizers"],
        "normalizers_finite_positive": [value > 0.0 and math.isfinite(value) for value in [f64(bit) for bit in face["normalizers"]]],
        "scales_finite_nonzero": [value != 0.0 and math.isfinite(value) for value in scales],
        "raw_to_weighted_bit_mismatches": reconstruction_bit_mismatches,
        "raw_to_weighted_residual_bit_mismatches": residual_bit_mismatches,
        "zero_rows": [index for index, row in enumerate(matrix) if all(value == 0.0 for value in row)],
        "zero_columns": [column for column in range(21) if all(row[column] == 0.0 for row in matrix)],
        "duplicate_rows": duplicate_rows,
        "duplicate_columns": duplicate_columns,
        "weighted_row_norms": [math.sqrt(sum(value * value for value in row)) for row in matrix],
        "weighted_column_norms": [math.sqrt(sum(row[column] * row[column] for row in matrix)) for column in range(len(matrix[0]))],
        "raw_row_norms": [math.sqrt(sum(value * value for value in row)) for row in raw],
        "raw_column_norms": [math.sqrt(sum(row[column] * row[column] for row in raw)) for column in range(len(raw[0]))],
        "coordinate_scales_bits": face["scales"],
        "weighted_residual_bits": face["weighted_residual"],
        "runtime_sigma_bits": runtime_sigma,
        "runtime_sigma_decimal": [mps(mp.mpf(f64(bits).as_integer_ratio()[0]) / f64(bits).as_integer_ratio()[1]) for bits in runtime_sigma],
        "reference_svd": reference_svd,
    }
    with open(args.output, "w", encoding="utf-8") as output:
        json.dump(result, output, indent=2, sort_keys=True)
        output.write("\n")


if __name__ == "__main__":
    main()
