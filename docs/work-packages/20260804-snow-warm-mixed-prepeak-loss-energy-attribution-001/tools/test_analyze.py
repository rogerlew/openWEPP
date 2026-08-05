#!/usr/bin/env python3
"""Focused unit tests for the frozen 21L analyzer."""

from __future__ import annotations

import datetime as dt
import importlib.util
import unittest
import tempfile
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("analyze.py")
SPEC = importlib.util.spec_from_file_location("snow_21l_analyze", MODULE_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("unable to load analyzer")
ANALYZE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ANALYZE)


def hour(temp: float, pack: float = 0.1, snowfall: float = 0.0) -> dict[str, float]:
    return {
        "air_temperature_c": temp,
        "pack_depth_before_m": pack,
        "snowfall_swe_m": snowfall,
    }


class AnalyzerTests(unittest.TestCase):
    def test_thermal_classes_are_exclusive(self) -> None:
        self.assertEqual(ANALYZE.day_class([hour(-2.0), hour(0.0)])[0], "cold_day")
        self.assertEqual(ANALYZE.day_class([hour(-1.0), hour(1.0)])[0], "mixed_day")
        self.assertEqual(ANALYZE.day_class([hour(0.1), hour(2.0)])[0], "warm_day")

    def test_inactive_hours_are_unclassified(self) -> None:
        rows = [hour(-1.0, pack=0.0), hour(2.0, pack=0.0)]
        label, minimum, maximum, count = ANALYZE.day_class(rows)
        self.assertIsNone(label)
        self.assertEqual((minimum, maximum, count), (None, None, 0))

    def test_group_consecutive_does_not_bridge_gap(self) -> None:
        start = dt.date(2020, 1, 1)
        groups = ANALYZE.group_consecutive([start, start + dt.timedelta(days=1), start + dt.timedelta(days=3)])
        self.assertEqual([len(group) for group in groups], [2, 1])

    def test_observed_peak_uses_earliest_tie(self) -> None:
        observations = {
            dt.date(2019, 10, 1): {"water_year": 2020, "swe_m": 0.0},
            dt.date(2020, 3, 1): {"water_year": 2020, "swe_m": 0.5},
            dt.date(2020, 3, 2): {"water_year": 2020, "swe_m": 0.5},
        }
        windows = ANALYZE.observed_windows(observations)
        self.assertEqual(windows[0]["observed_peak_date"], dt.date(2020, 3, 1))

    def test_guarded_precipitation_rejects_reset_and_gap(self) -> None:
        observations = {
            dt.date(2020, 9, 30): {"water_year": 2020, "precip_cumulative_m": 1.0},
            dt.date(2020, 10, 1): {"water_year": 2021, "precip_cumulative_m": 0.01},
            dt.date(2020, 10, 2): {"water_year": 2021, "precip_cumulative_m": 0.02},
            dt.date(2020, 10, 4): {"water_year": 2021, "precip_cumulative_m": 0.04},
        }
        result = ANALYZE.guarded_precipitation(observations, 1e-12)
        self.assertEqual(result, {dt.date(2020, 10, 2): 0.01})

    def test_rank_correlations_report_monotonic_relation(self) -> None:
        left = [1.0, 2.0, 3.0, 4.0]
        self.assertAlmostEqual(ANALYZE.pearson(left, left), 1.0)
        self.assertAlmostEqual(ANALYZE.spearman(left, list(reversed(left))), -1.0)

    def test_climate_custody_allows_precipitation_only(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            canonical = root / "canonical.cli"
            development = root / "development.cli"
            canonical.write_text("header\n1 1 2000 1.0 2 3 4 5 6 7 8 9 10\n", encoding="utf-8")
            development.write_text("header\n1 1 2000 1.2 2 3 4 5 6 7 8 9 10\n", encoding="utf-8")
            result = ANALYZE.compare_precipitation_only_climates(canonical, development)
            self.assertEqual(result["changed_precipitation_row_count"], 1)

    def test_stage3_closure_is_reconstructed_from_primitive_operands(self) -> None:
        row = {
            "stage3_incoming_liquid_m": 0.10,
            "stage3_routed_liquid_m": 0.06,
            "stage3_retained_liquid_delta_m": 0.03,
            "stage3_refrozen_liquid_m": 0.01,
            "stage3_surface_energy_j_m2": 10.0,
            "stage3_conduction_energy_j_m2": -2.0,
            "stage3_latent_refreeze_energy_j_m2": 5.0,
            "stage3_cold_content_export_j_m2": 1.0,
            "stage3_cold_content_before_j_m2": 20.0,
            "stage3_cold_content_after_j_m2": 6.0,
        }
        mass, energy = ANALYZE.reconstruct_stage3_closure(row)
        self.assertAlmostEqual(mass, 0.0)
        self.assertAlmostEqual(energy, 0.0)


if __name__ == "__main__":
    unittest.main()
