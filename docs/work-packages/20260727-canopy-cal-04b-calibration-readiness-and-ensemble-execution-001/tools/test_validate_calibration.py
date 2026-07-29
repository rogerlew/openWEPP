#!/usr/bin/env python3
"""Focused non-population tests for CAL-04B validation arithmetic."""

from __future__ import annotations

import math
import unittest

import numpy as np

import validate


class CalibrationValidationTests(unittest.TestCase):
    def test_cal04b03_cardinality_is_exact(self) -> None:
        self.assertEqual(validate.TRACE_HEADER.size, 20)
        self.assertEqual(validate.TRACE_VALUE_COUNT, 9_261 * 9 * 36 * 180)
        self.assertEqual(validate.TRACE_BYTES, 4_320_812_180)
        self.assertEqual(validate.OBSERVATION_COMPONENT_COUNT, 8_631_252)
        self.assertEqual(validate.CROSSING_COMPONENT_COUNT, 3_000_564)

    def test_crossing_eligibility_uses_warmup_and_closed_boundaries(self) -> None:
        cube = np.zeros(
            (validate.LANE_COUNT, validate.YEAR_COUNT, validate.DAYS_PER_YEAR),
            dtype="<f8",
        )
        cube[0, 0, 58] = 1.0
        cube[0, 0, 59] = 1.0
        cube[1, 0, 59] = 1.0
        cube[2, 0, 179] = 1.0
        crossings = validate.eligible_crossings(cube)
        self.assertEqual(int(crossings[0, 0]), 0)
        self.assertEqual(int(crossings[1, 0]), 60)
        self.assertEqual(int(crossings[2, 0]), 180)

    def test_crossing_rejects_nonfinite_or_out_of_range_trace(self) -> None:
        cube = np.zeros(
            (validate.LANE_COUNT, validate.YEAR_COUNT, validate.DAYS_PER_YEAR),
            dtype="<f8",
        )
        cube[0, 0, 0] = math.nan
        with self.assertRaisesRegex(ValueError, "nonfinite/out-of-range"):
            validate.eligible_crossings(cube)
        cube[0, 0, 0] = 1.01
        with self.assertRaisesRegex(ValueError, "nonfinite/out-of-range"):
            validate.eligible_crossings(cube)

    def test_equal_year_objective_does_not_weight_record_count(self) -> None:
        self.assertEqual(validate.equal_year_objective([0.0, 4.0]), math.sqrt(2.0))

    def test_exact_float_preserves_binary_membership_values(self) -> None:
        value = 0.1 + 0.2
        self.assertTrue(validate.exact_float(format(value, ".17"), value))
        self.assertTrue(validate.exact_float("+infinity", math.inf))
        self.assertFalse(validate.exact_float("0.3", value))

    def test_aggregate_float_tolerance_is_bounded_to_four_ulps(self) -> None:
        expected = 60.0
        within = expected
        for _ in range(4):
            within = math.nextafter(within, math.inf)
        outside = math.nextafter(within, math.inf)
        self.assertTrue(
            validate.aggregate_float_within_ulps(format(within, ".17"), expected)
        )
        self.assertFalse(
            validate.aggregate_float_within_ulps(format(outside, ".17"), expected)
        )
        self.assertTrue(
            validate.aggregate_float_within_ulps("+infinity", math.inf)
        )
        self.assertFalse(
            validate.aggregate_float_within_ulps("+infinity", expected)
        )

    def test_authenticated_calibration_projection_is_complete(self) -> None:
        configs, observations, path = validate._load_calibration_inputs()
        self.assertEqual(len(configs), validate.CANDIDATE_COUNT)
        self.assertEqual(len(observations), validate.OBSERVATION_COUNT)
        self.assertEqual(path.name, "phenology-forcing-join.csv")

    def test_unobserved_plot_year_crossing_still_invalidates_candidate(self) -> None:
        _, observations, _ = validate._load_calibration_inputs()
        observed_groups = {(row[3], row[1]) for row in observations}
        canonical_groups = {
            (plot, year)
            for plot in validate.CANONICAL_LANES
            for year in range(validate.FIRST_YEAR, validate.LAST_YEAR + 1)
        }
        unobserved = sorted(canonical_groups - observed_groups)
        self.assertEqual(len(observed_groups), 313)
        self.assertEqual(len(unobserved), 11)
        missing_plot, missing_year = unobserved[0]
        crossings = np.full(
            (validate.LANE_COUNT, validate.YEAR_COUNT),
            100,
            dtype=np.uint16,
        )
        crossings[
            validate.CANONICAL_LANES.index(missing_plot),
            missing_year - validate.FIRST_YEAR,
        ] = 0
        self.assertEqual(
            validate.canonical_missing_groups(crossings),
            [(missing_plot, missing_year)],
        )
        self.assertEqual(validate.observed_missing_years(crossings, observations), set())


if __name__ == "__main__":
    unittest.main()
