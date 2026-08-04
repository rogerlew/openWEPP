#!/usr/bin/env python3
"""Focused tests for the frozen snow input-versus-loss operators."""

from __future__ import annotations

import datetime as dt
import importlib.util
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("analyze.py")
SPEC = importlib.util.spec_from_file_location("snow_input_loss_analyze", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class AnalyzeTests(unittest.TestCase):
    def test_predecessor_authority_rejects_mutated_receipt_trace(self) -> None:
        expected_hash = "a" * 64
        freeze = {
            "source_identity_expectations": {
                "predecessor_trace_receipt_sha256": "b" * 64,
                "predecessor_annual_results_sha256": "c" * 64,
                "trace_sha256_by_site": {site: expected_hash for site in MODULE.SITES},
            }
        }
        manifest = {
            "tracked_results": {
                "execution_receipt_sha256": "b" * 64,
                "cross_fixture_sha256": "c" * 64,
            },
            "exact_traces": [
                {
                    "path": f"target/runs/{site}/{site}-adjudication.snow.jsonl",
                    "sha256": expected_hash,
                }
                for site in MODULE.SITES
            ],
        }
        receipt = {
            "results": {"cross_fixture": {"sha256": "c" * 64}},
            "sites": {
                site: {
                    "outputs": {
                        f"{site}-adjudication.snow.jsonl": {
                            "path": f"target/runs/{site}/{site}-adjudication.snow.jsonl",
                            "sha256": expected_hash,
                        }
                    }
                }
                for site in MODULE.SITES
            },
        }
        MODULE.validate_predecessor_authority(freeze, manifest, receipt)
        receipt["sites"][MODULE.SITES[0]]["outputs"][
            f"{MODULE.SITES[0]}-adjudication.snow.jsonl"
        ]["sha256"] = "d" * 64
        with self.assertRaisesRegex(RuntimeError, "receipt trace identity"):
            MODULE.validate_predecessor_authority(freeze, manifest, receipt)

    def test_guarded_precipitation_does_not_bridge_gap_or_reset(self) -> None:
        rows = {
            dt.date(2023, 9, 29): {"water_year": 2023, "precip_cumulative_m": 0.10},
            dt.date(2023, 9, 30): {"water_year": 2023, "precip_cumulative_m": 0.12},
            dt.date(2023, 10, 1): {"water_year": 2024, "precip_cumulative_m": 0.0},
            dt.date(2023, 10, 3): {"water_year": 2024, "precip_cumulative_m": 0.01},
        }
        increments, counts = MODULE.guarded_precipitation_increments(rows, 1e-12)
        self.assertEqual(increments, {dt.date(2023, 9, 30): 0.01999999999999999})
        self.assertEqual(counts["water_year_reset"], 1)
        self.assertGreaterEqual(counts["gap"], 1)

    def test_guarded_precipitation_rejects_material_negative(self) -> None:
        rows = {
            dt.date(2024, 1, 1): {"water_year": 2024, "precip_cumulative_m": 0.10},
            dt.date(2024, 1, 2): {"water_year": 2024, "precip_cumulative_m": 0.09},
        }
        increments, counts = MODULE.guarded_precipitation_increments(rows, 1e-12)
        self.assertEqual(increments, {})
        self.assertEqual(counts["negative"], 1)

    def test_consecutive_grouping_is_disjoint(self) -> None:
        groups = MODULE.group_consecutive(
            [dt.date(2024, 1, 1), dt.date(2024, 1, 2), dt.date(2024, 1, 4)]
        )
        self.assertEqual([len(group) for group in groups], [2, 1])

    def test_event_padding_clips_and_merges(self) -> None:
        events = MODULE.merge_cold_event_intervals(
            [dt.date(2024, 1, 1), dt.date(2024, 1, 3)],
            dt.date(2024, 1, 1),
            dt.date(2024, 1, 10),
        )
        self.assertEqual(len(events), 1)
        self.assertEqual(events[0]["start"], dt.date(2024, 1, 1))
        self.assertEqual(events[0]["end"], dt.date(2024, 1, 4))
        self.assertEqual(len(events[0]["active_dates"]), 2)

    def test_cohort_truth_table(self) -> None:
        rows = [
            {
                "mass_ceiling_site_signal": index < 3,
                "cold_event_all_phase_site_signal": False,
                "cold_event_snowfall_site_signal": False,
                "phase_or_solid_input_site_signal": False,
                "dry_loss_site_signal": index < 3,
            }
            for index in range(4)
        ]
        result = MODULE.cohort_summary(rows, 3)
        self.assertEqual(result["verdict"], "MULTIFACTOR_INPUT_AND_LOSS_SIGNAL")


if __name__ == "__main__":
    unittest.main()
