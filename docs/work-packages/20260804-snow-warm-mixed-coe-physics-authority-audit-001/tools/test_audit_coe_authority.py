#!/usr/bin/env python3
"""Focused unit tests for the package-local CoE authority analyzer."""

from __future__ import annotations

import csv
import importlib.util
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).with_name("audit_coe_authority.py")
SPEC = importlib.util.spec_from_file_location("audit_coe_authority", MODULE_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("unable to load audit analyzer")
AUDIT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(AUDIT)


class ReconstructTermsTests(unittest.TestCase):
    def test_subcomponents_close_exactly_for_windy_hour(self) -> None:
        terms = AUDIT.reconstruct_terms(
            radiation_mj_m2=0.75,
            cloud_fraction=0.4,
            air_temperature_c=2.0,
            dewpoint_c=-1.0,
            wind_m_s=3.0,
            rain_m=0.001,
            canopy_cover_fraction=0.25,
        )
        self.assertAlmostEqual(terms["b_temp"] + terms["b_clear"], terms["bmelt"], delta=1e-12)
        self.assertAlmostEqual(terms["c_open"] + terms["c_canopy"], terms["cmelt"], delta=1e-12)
        self.assertGreater(terms["amelt"], 0.0)
        self.assertGreater(terms["dmelt"], 0.0)

    def test_calm_branch_assigns_whole_term_to_reported_canopy_bucket(self) -> None:
        terms = AUDIT.reconstruct_terms(
            radiation_mj_m2=0.0,
            cloud_fraction=1.0,
            air_temperature_c=1.0,
            dewpoint_c=-2.0,
            wind_m_s=0.0,
            rain_m=0.0,
            canopy_cover_fraction=0.0,
        )
        self.assertEqual(terms["c_open"], 0.0)
        self.assertEqual(terms["c_canopy"], terms["cmelt"])

    def test_rain_heat_uses_dewpoint_only_when_positive(self) -> None:
        common = {
            "radiation_mj_m2": 0.0,
            "cloud_fraction": 1.0,
            "air_temperature_c": 3.0,
            "wind_m_s": 1.0,
            "rain_m": 0.002,
            "canopy_cover_fraction": 0.0,
        }
        cold_dew = AUDIT.reconstruct_terms(dewpoint_c=-1.0, **common)
        warm_dew = AUDIT.reconstruct_terms(dewpoint_c=1.0, **common)
        self.assertGreater(cold_dew["dmelt"], warm_dew["dmelt"])

    def test_analyze_reconstructs_both_caller_bypass_branches(self) -> None:
        daily_fields = [
            "lane", "role", "date", "thermal_class", "wind_m_s", "dewpoint_c",
            "canopy_cover_fraction", *[f"coe_{term}_m" for term in AUDIT.TERM_KEYS],
            *[f"coe_{term}_positive_m" for term in AUDIT.TERM_KEYS],
        ]
        hourly_fields = [
            "lane", "role", "date", "eligible_hour", "daily_thermal_class",
            "radiation_mj_m2", "cloud_fraction", "air_temperature_c", "rain_m",
            "snowfall_swe_m", "pack_depth_before_m", "pack_density_before_kg_m3",
            *[f"coe_{term}_m" for term in AUDIT.TERM_KEYS], "coe_uncapped_m",
            "coe_applied_m", "coe_cap_adjustment_m",
        ]
        lane = "test_lane"
        dates = ("2000-01-01", "2000-01-02")
        with tempfile.TemporaryDirectory() as directory:
            daily_path = Path(directory) / "daily.csv"
            hourly_path = Path(directory) / "hourly.csv"
            with daily_path.open("w", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=daily_fields)
                writer.writeheader()
                for date in dates:
                    row = {
                        "lane": lane,
                        "role": "CANONICAL",
                        "date": date,
                        "thermal_class": "mixed_day",
                        "wind_m_s": 2.0,
                        "dewpoint_c": -1.0,
                        "canopy_cover_fraction": 0.5,
                    }
                    row.update(
                        {field: 0.0 for field in daily_fields if field.startswith("coe_")}
                    )
                    writer.writerow(row)
            with hourly_path.open("w", newline="") as stream:
                writer = csv.DictWriter(stream, fieldnames=hourly_fields)
                writer.writeheader()
                common = {
                    "lane": lane,
                    "role": "CANONICAL",
                    "eligible_hour": "True",
                    "daily_thermal_class": "mixed_day",
                    "radiation_mj_m2": 1.0,
                    "cloud_fraction": 0.5,
                    "air_temperature_c": 1.0,
                    "rain_m": 0.0,
                    "pack_density_before_kg_m3": 200.0,
                    **{f"coe_{term}_m": 0.0 for term in AUDIT.TERM_KEYS},
                    "coe_uncapped_m": 0.0,
                    "coe_applied_m": 0.0,
                    "coe_cap_adjustment_m": 0.0,
                }
                writer.writerow(
                    {
                        **common,
                        "date": dates[0],
                        "snowfall_swe_m": 0.0,
                        "pack_depth_before_m": 0.2,
                    }
                )
                writer.writerow(
                    {
                        **common,
                        "date": dates[1],
                        "snowfall_swe_m": 0.01,
                        "pack_depth_before_m": 0.0,
                    }
                )
            result = AUDIT.analyze(
                daily_path,
                hourly_path,
                1e-12,
                {lane: {dates[0]: -1.0, dates[1]: 1.0}},
            )
        self.assertEqual(result["selected_hour_count"], 2)
        self.assertEqual(result["overall_max_abs_reconstruction_residual_m"], 0.0)


if __name__ == "__main__":
    unittest.main()
