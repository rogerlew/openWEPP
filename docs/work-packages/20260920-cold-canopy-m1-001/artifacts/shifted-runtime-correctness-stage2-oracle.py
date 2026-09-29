#!/usr/bin/env python3
"""Independent binary64 oracle for the two Stage2 positive-lambda retry values.

This script imports no openWEPP code and consumes no candidate output.  It
transcribes the unchanged analytic fixture and the owner-adopted BVLS-04 order.
Python float is required to be IEEE-754 binary64.
"""
import json
import math
import struct
import sys

N = 21
SPLITTER = 134_217_729.0


def bits(value):
    return f"{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"


def split(value):
    c = SPLITTER * value
    abig = c - value
    high = c - abig
    return high, value - high


def two_product(left, right):
    product = left * right
    left_high, left_low = split(left)
    right_high, right_low = split(right)
    remainder = product - left_high * right_high
    remainder = remainder - left_low * right_high
    remainder = remainder - left_high * right_low
    error = left_low * right_low - remainder
    return product, error


def two_sum(left, right):
    result = left + right
    z = result - left
    result_minus_z = result - z
    left_error = left - result_minus_z
    right_error = right - z
    return result, left_error + right_error


def dot2(terms, weights):
    partial, compensation = two_product(terms[0], weights[0])
    for term, weight in zip(terms[1:], weights[1:]):
        head, tail = two_product(term, weight)
        partial, sum_tail = two_sum(partial, head)
        sum_tail_plus_tail = sum_tail + tail
        compensation = compensation + sum_tail_plus_tail
    return partial + compensation


def ordered_norm(values):
    total = 0.0
    for value in values:
        total = total + value * value
    return math.sqrt(total)


def main():
    assert sys.float_info.mant_dig == 53 and sys.float_info.radix == 2
    residual = [-1.2, -1.0] + [0.0] * (N - 2)
    matrix = [[2.0 if row == column else 0.0 for column in range(N)] for row in range(N)]
    v = [[1.0 if row == column else 0.0 for column in range(N)] for row in range(N)]
    sigma = [2.0] * N
    order = list(range(N))

    def lambda_step(lam):
        coefficients = [0.0] * N
        for j in order:
            uty = 0.0
            for row, residual_value in enumerate(residual):
                av = 0.0
                for free in range(N):
                    av = av + matrix[row][free] * v[free][j]
                u = av / sigma[j]
                uty = uty + u * residual_value
            denominator = sigma[j] * sigma[j] + lam
            gain = sigma[j] / denominator
            coefficients[j] = -(gain * uty)
        step = []
        for output in range(N):
            total = 0.0
            for j in order:
                total = total + v[output][j] * coefficients[j]
            step.append(total)
        return step

    radius = 0.25
    low = 0.0
    high = 1.0
    endpoint = lambda_step(high)
    endpoint_norm = ordered_norm(endpoint)
    for _ in range(48):
        if endpoint_norm <= radius:
            break
        low = high
        high = high * 4.0
        endpoint = lambda_step(high)
        endpoint_norm = ordered_norm(endpoint)
    for _ in range(48):
        middle = (low + high) * 0.5
        step = lambda_step(middle)
        step_norm = ordered_norm(step)
        if step_norm <= radius:
            high = middle
            endpoint = step
        else:
            low = middle
    lam = high
    p0 = endpoint

    def ordinary_residual(point):
        output = []
        for row in range(N):
            total = 0.0
            for column in range(N):
                total = total + matrix[row][column] * point[column]
            output.append(residual[row] + total)
        return output

    def ordinary_defect(row_residual, point):
        output = []
        for slot in range(N):
            total = 0.0
            for row in range(N):
                total = total + matrix[row][slot] * row_residual[row]
            shift = lam * point[slot]
            output.append(total + shift)
        return output

    def shifted_inverse(defect):
        z = [0.0] * N
        for j in order:
            total = 0.0
            for i in range(N):
                total = total + v[i][j] * defect[i]
            denominator = sigma[j] * sigma[j] + lam
            z[j] = total / denominator
        output = []
        for i in range(N):
            total = 0.0
            for j in order:
                total = total + v[i][j] * z[j]
            output.append(-total)
        return output

    r0 = ordinary_residual(p0)
    h0 = ordinary_defect(r0, p0)
    delta0 = shifted_inverse(h0)
    p1 = [left + right for left, right in zip(p0, delta0)]
    r1 = [dot2([residual[row]] + matrix[row], [1.0] + p1) for row in range(N)]
    h1 = []
    for slot in range(N):
        terms = [matrix[row][slot] for row in range(N)] + [lam]
        weights = r1 + [p1[slot]]
        h1.append(dot2(terms, weights))
    delta1 = shifted_inverse(h1)
    p2 = [left + right for left, right in zip(p1, delta1)]

    expected_new = ["3fc89544cafec8ac", "3fc47c63fe7efc8f"]
    assert [bits(value) for value in p2[:2]] == expected_new
    output = {
        "schema": "openwepp.shifted-runtime-correctness-stage2-oracle.v1",
        "method": "standalone analytic IEEE-754 binary64 transcription; no candidate import or output",
        "fixture": {
            "dimension": N,
            "weighted_residual_first_two": [bits(value) for value in residual[:2]],
            "weighted_matrix": "2*I_21",
            "v": "I_21",
            "sigma": [bits(2.0)] * N,
            "factor_order": order,
            "radius_bits": bits(radius),
        },
        "intermediates_first_two": {
            "lambda": [bits(lam)],
            "p0": [bits(value) for value in p0[:2]],
            "r0": [bits(value) for value in r0[:2]],
            "h0": [bits(value) for value in h0[:2]],
            "delta0": [bits(value) for value in delta0[:2]],
            "p1": [bits(value) for value in p1[:2]],
            "r1_dot2": [bits(value) for value in r1[:2]],
            "h1_dot2": [bits(value) for value in h1[:2]],
            "delta1": [bits(value) for value in delta1[:2]],
            "p2": [bits(value) for value in p2[:2]],
        },
        "mapping": {
            "old_expected_bits": ["3fc89544cafec8ad", "3fc47c63fe7efc90"],
            "new_bvls04_bits": expected_new,
            "new_decimal_roundtrip": [repr(value) for value in p2[:2]],
            "ulp_change": [-1, -1],
            "assertion_sites": [
                "m1_trust_region_stage2_controls.rs:1927",
                "m1_trust_region_stage2_controls.rs:1963",
            ],
            "event_mapping": "Only phase_quarter_selected_step coordinates 0 and 1 change; preserve both full event-vector assertions and all other entries.",
        },
    }
    print(json.dumps(output, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
