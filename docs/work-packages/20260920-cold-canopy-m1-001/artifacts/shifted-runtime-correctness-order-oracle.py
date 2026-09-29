#!/usr/bin/env python3
"""Independent small BVLS-04 retained-shifted-inverse binary64 oracle.

This fixed four-dimensional fixture does not read repository, prototype,
capture, candidate, or reference data. Python float is used solely as the
host IEEE-754 binary64 scalar, with every product and addition written in the
authority-prescribed sequence and no fused operations.
"""

import json
import math
import struct
import sys


V = [
    [0.5, 0.5, 0.5, 0.5],
    [-0.5, 0.5, 0.5, -0.5],
    [0.5, -0.5, 0.5, -0.5],
    [-0.5, -0.5, 0.5, 0.5],
]
SIGMA = [1.0, 2.0, 3.0, 4.0]
LAMBDA = 0.5
H_FREE = [1.0, 2.0, 4.0, 8.0]
FREE_IDS = [3, 1, 0, 2]
FACTOR_ORDER = [2, 0, 3, 1]
ASCENDING_RAW_ORDER = [0, 1, 2, 3]


def bits(value):
    return f"{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"


def normal_or_zero(value):
    return math.isfinite(value) and (value == 0.0 or abs(value) >= sys.float_info.min)


def accumulate(products):
    total = 0.0
    for product in products:
        assert normal_or_zero(product)
        total = total + product
        assert normal_or_zero(total)
    return total


def factor_apply(order):
    # t_j traverses free slots 0..m, independently of factor.order.
    t = []
    for raw_j in range(4):
        t.append(accumulate([V[slot][raw_j] * H_FREE[slot] for slot in range(4)]))

    # Each square, shift addition, and division is an explicit binary64 step.
    z = []
    denominators = []
    for raw_j in range(4):
        square = SIGMA[raw_j] * SIGMA[raw_j]
        denominator = square + LAMBDA
        quotient = t[raw_j] / denominator
        assert all(normal_or_zero(x) for x in (square, denominator, quotient))
        denominators.append(denominator)
        z.append(quotient)

    # Each output row traverses raw spectral positions through factor.order,
    # then applies one final unary negation.
    delta = []
    for slot in range(4):
        summed = accumulate([V[slot][raw_j] * z[raw_j] for raw_j in order])
        delta.append(-summed)
    return t, denominators, z, delta


def main():
    # ±0.5 makes this orthogonality check exact in binary64.
    gram = []
    for left in range(4):
        gram.append([])
        for right in range(4):
            gram[-1].append(
                accumulate([V[row][left] * V[row][right] for row in range(4)])
            )
    assert gram == [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    assert V != [list(column) for column in zip(*V)]

    t, denominators, z, ordered_delta = factor_apply(FACTOR_ORDER)
    _, _, _, ascending_delta = factor_apply(ASCENDING_RAW_ORDER)
    assert [bits(x) for x in ordered_delta] != [bits(x) for x in ascending_delta]

    coordinate_delta = [None] * 4
    for slot, coordinate in enumerate(FREE_IDS):
        coordinate_delta[coordinate] = ordered_delta[slot]

    print(json.dumps({
        "schema": "openwepp.cold-canopy-m1.shifted-runtime-order-oracle.v1",
        "fixture": {
            "v": V,
            "sigma": SIGMA,
            "lambda": LAMBDA,
            "h_free": H_FREE,
            "h_semantics": "complete shifted defect at the retained shifted-inverse input",
            "free_ids": FREE_IDS,
            "factor_order": FACTOR_ORDER
        },
        "assumptions": [
            "V rows are free slots in free_ids order and columns are raw spectral positions",
            "t_j accumulates free slots in ascending slot order",
            "z_j uses raw sigma[j] and lambda",
            "delta rows accumulate raw spectral positions through factor_order and then apply unary negation"
        ],
        "orthogonal_exact_binary64": True,
        "nonsymmetric": True,
        "actual": {
            "t_bits": [bits(x) for x in t],
            "denominator_bits": [bits(x) for x in denominators],
            "z_bits": [bits(x) for x in z],
            "delta_free_slot_bits": [bits(x) for x in ordered_delta],
            "delta_coordinate_bits": [bits(x) for x in coordinate_delta]
        },
        "independent_test_side_counterfactual": {
            "ascending_raw_order": ASCENDING_RAW_ORDER,
            "delta_free_slot_bits": [bits(x) for x in ascending_delta],
            "differs_from_factor_order": True
        }
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
