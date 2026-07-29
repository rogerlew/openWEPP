#!/usr/bin/env python3
"""Focused tests for exact IEEE-754 aggregate step auditing."""

from __future__ import annotations

import math
import unittest

import validate_terminal_aggregate_ulps as audit


class TerminalAggregateUlpTests(unittest.TestCase):
    def test_exact_step_distance_crosses_binade_without_width_scaling(self) -> None:
        expected = math.nextafter(8.0, -math.inf)
        within = expected
        for _ in range(4):
            within = math.nextafter(within, math.inf)
        outside = math.nextafter(within, math.inf)
        self.assertEqual(audit.ulp_steps(expected, within), 4)
        self.assertEqual(audit.ulp_steps(expected, outside), 5)

    def test_equal_signed_zero_has_zero_distance(self) -> None:
        self.assertEqual(audit.ulp_steps(-0.0, 0.0), 0)

    def test_nonfinite_rank_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "finite"):
            audit.ordered_bits(math.inf)


if __name__ == "__main__":
    unittest.main()
