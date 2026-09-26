#!/usr/bin/env python3
"""Guarded non-FMA ORO Dot2 primitive for M1-DOT2-RESIDUAL-PROTOTYPE-01.

The candidate path is intentionally scalar binary64.  Fraction-based identities
are retained only in ``verify_identities`` after a candidate calculation.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from fractions import Fraction
import math
import struct
import sys

SPLITTER = 2.0**27 + 1.0
MIN_NORMAL = sys.float_info.min


class PrototypeLimitation(RuntimeError):
    """A Dot2 hypothesis is not admitted for this bounded prototype."""


def bits(value: float) -> str:
    return struct.pack(">d", float(value)).hex()


def admitted(value: float) -> bool:
    return math.isfinite(value) and (value == 0.0 or abs(value) >= MIN_NORMAL)


@dataclass
class Counters:
    arithmetic: int = 0
    guards: int = 0
    branches: int = 0
    exceptional_branches: int = 0
    two_sum_calls: int = 0
    split_calls: int = 0
    two_product_calls: int = 0
    identity_checks: int = 0
    signed_zero_values: int = 0


@dataclass
class Range:
    minimum_nonzero_abs: float | None = None
    maximum_abs: float = 0.0
    values: int = 0

    def observe(self, value: float) -> None:
        magnitude = abs(value)
        self.values += 1
        if magnitude > self.maximum_abs:
            self.maximum_abs = magnitude
        if magnitude != 0.0 and (self.minimum_nonzero_abs is None or magnitude < self.minimum_nonzero_abs):
            self.minimum_nonzero_abs = magnitude


@dataclass
class Trace:
    two_sums: list[tuple[float, float, float, float]] = field(default_factory=list)
    splits: list[tuple[float, float, float]] = field(default_factory=list)
    two_products: list[tuple[float, float, float, float]] = field(default_factory=list)


@dataclass
class Dot2Result:
    value: float
    counters: Counters
    ranges: Range
    trace: Trace


def _guard(value: float, label: str, counters: Counters, ranges: Range) -> float:
    counters.guards += 1
    counters.branches += 1
    ranges.observe(value)
    if value == 0.0:
        counters.signed_zero_values += 1
    if not admitted(value):
        counters.exceptional_branches += 1
        raise PrototypeLimitation(f"{label}: nonfinite or nonzero subnormal ({bits(value)})")
    return value


def _mul(left: float, right: float, label: str, counters: Counters, ranges: Range) -> float:
    value = left * right
    counters.arithmetic += 1
    _guard(value, label, counters, ranges)
    counters.branches += 1
    if left != 0.0 and right != 0.0 and value == 0.0:
        counters.exceptional_branches += 1
        raise PrototypeLimitation(f"{label}: nonzero-product underflow")
    return value


def _add(left: float, right: float, label: str, counters: Counters, ranges: Range) -> float:
    value = left + right
    counters.arithmetic += 1
    return _guard(value, label, counters, ranges)


def _sub(left: float, right: float, label: str, counters: Counters, ranges: Range) -> float:
    value = left - right
    counters.arithmetic += 1
    return _guard(value, label, counters, ranges)


def split(a: float, counters: Counters, ranges: Range, trace: Trace) -> tuple[float, float]:
    """ORO Algorithm 3.2, using the fixed binary64 splitter."""
    counters.split_calls += 1
    try:
        c = _mul(SPLITTER, a, "split.c", counters, ranges)
    except PrototypeLimitation as error:
        if not math.isfinite(SPLITTER * a):
            raise PrototypeLimitation("splitter overflow") from error
        raise
    ca = _sub(c, a, "split.c_minus_a", counters, ranges)
    high = _sub(c, ca, "split.high", counters, ranges)
    low = _sub(a, high, "split.low", counters, ranges)
    trace.splits.append((a, high, low))
    return high, low


def two_sum(a: float, b: float, counters: Counters, ranges: Range, trace: Trace) -> tuple[float, float]:
    """ORO Algorithm 3.1 with its published parenthesization."""
    counters.two_sum_calls += 1
    x = _add(a, b, "two_sum.x", counters, ranges)
    z = _sub(x, a, "two_sum.z", counters, ranges)
    x_minus_z = _sub(x, z, "two_sum.x_minus_z", counters, ranges)
    left = _sub(a, x_minus_z, "two_sum.left", counters, ranges)
    right = _sub(b, z, "two_sum.right", counters, ranges)
    y = _add(left, right, "two_sum.y", counters, ranges)
    trace.two_sums.append((a, b, x, y))
    return x, y


def two_product(a: float, b: float, counters: Counters, ranges: Range, trace: Trace) -> tuple[float, float]:
    """ORO Algorithm 3.3, non-FMA; 17 arithmetic operations including Split."""
    counters.two_product_calls += 1
    product = _mul(a, b, "two_product.product", counters, ranges)
    a_high, a_low = split(a, counters, ranges, trace)
    b_high, b_low = split(b, counters, ranges, trace)
    a_high_b_high = _mul(a_high, b_high, "two_product.a_high_b_high", counters, ranges)
    remainder = _sub(product, a_high_b_high, "two_product.r0", counters, ranges)
    a_low_b_high = _mul(a_low, b_high, "two_product.a_low_b_high", counters, ranges)
    remainder = _sub(remainder, a_low_b_high, "two_product.r1", counters, ranges)
    a_high_b_low = _mul(a_high, b_low, "two_product.a_high_b_low", counters, ranges)
    remainder = _sub(remainder, a_high_b_low, "two_product.r2", counters, ranges)
    a_low_b_low = _mul(a_low, b_low, "two_product.a_low_b_low", counters, ranges)
    error = _sub(a_low_b_low, remainder, "two_product.error", counters, ranges)
    trace.two_products.append((a, b, product, error))
    return product, error


def dot2(x: list[float], y: list[float]) -> Dot2Result:
    """ORO Algorithm 5.3 for one fixed-order dot product; no candidate oracle."""
    if len(x) != len(y) or not x:
        raise ValueError("Dot2 requires equal nonempty vectors")
    counters, ranges, trace = Counters(), Range(), Trace()
    for index, value in enumerate(x):
        _guard(value, f"x[{index}]", counters, ranges)
    for index, value in enumerate(y):
        _guard(value, f"y[{index}]", counters, ranges)
    p, s = two_product(x[0], y[0], counters, ranges, trace)
    for index in range(1, len(x)):
        h, r = two_product(x[index], y[index], counters, ranges, trace)
        p, q = two_sum(p, h, counters, ranges, trace)
        qr = _add(q, r, "dot2.q_plus_r", counters, ranges)
        s = _add(s, qr, "dot2.s_plus_qr", counters, ranges)
    value = _add(p, s, "dot2.p_plus_s", counters, ranges)
    return Dot2Result(value, counters, ranges, trace)


def verify_identities(result: Dot2Result) -> None:
    """Exact diagnostic controls, deliberately outside the candidate arithmetic path."""
    for a, b, x, y in result.trace.two_sums:
        result.counters.identity_checks += 1
        if Fraction.from_float(a) + Fraction.from_float(b) != Fraction.from_float(x) + Fraction.from_float(y):
            raise PrototypeLimitation("TwoSum exact identity failed")
    for a, high, low in result.trace.splits:
        result.counters.identity_checks += 1
        if Fraction.from_float(a) != Fraction.from_float(high) + Fraction.from_float(low):
            raise PrototypeLimitation("Split exact identity failed")
    for a, b, product, error in result.trace.two_products:
        result.counters.identity_checks += 1
        if Fraction.from_float(a) * Fraction.from_float(b) != Fraction.from_float(product) + Fraction.from_float(error):
            raise PrototypeLimitation("TwoProduct exact identity failed")
