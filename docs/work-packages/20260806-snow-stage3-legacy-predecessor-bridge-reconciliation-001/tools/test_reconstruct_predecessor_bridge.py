#!/usr/bin/env python3
"""Synthetic independent-consumer tests for predecessor bridge evidence."""

from __future__ import annotations

import datetime as dt
import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path

TOOL = Path(__file__).with_name("reconstruct_predecessor_bridge.py")
SPEC = importlib.util.spec_from_file_location("predecessor_bridge_consumer", TOOL)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load {TOOL}")
consumer = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = consumer
SPEC.loader.exec_module(consumer)


def write_jsonl(path: Path, rows: list[dict[str, object]]) -> None:
    path.write_text("".join(json.dumps(row) + "\n" for row in rows), encoding="utf-8")


def v4_row(day_index: int, hourly: list[float]) -> dict[str, object]:
    return {
        "schema": "openwepp-r7h-direct-production-snow-trace-v4",
        "day_index": day_index,
        "lane_index": 0,
        "stage3_energy_enabled": True,
        "stage3_shadow_hourly_complete_energy_j_m2": hourly,
        "stage3_shadow_complete_energy_j_m2": sum(hourly),
        "stage3_shadow_maximum_energy_closure_residual_j_m2": 0.0,
    }


def v6_tuple() -> dict[str, object]:
    return {
        "applicable": True,
        "applicability_reason": "evaluated",
        "hour_index": 0,
        "substep_index": 0,
        "elapsed_start_seconds": 0.0,
        "duration_seconds": 3600.0,
        "net_shortwave_w_m2": 10.0,
        "net_longwave_w_m2": -4.0,
        "sensible_flux_w_m2": -1.0,
        "latent_flux_w_m2": -2.0,
        "precipitation_advected_flux_w_m2": 0.0,
        "complete_external_flux_w_m2": 3.0,
        "internal_active_lower_conduction_j_m2": 5.0,
        "legacy_sequential_complete_j_m2": 10_805.0,
        "total_ice_mass_before_kg_m2": 10.0,
        "melt_kg_m2": 1.0,
        "sublimation_kg_m2": 0.25,
        "deposition_kg_m2": 0.5,
        "total_ice_mass_after_kg_m2": 9.25,
        "total_cold_before_j_m2": 100.0,
        "active_cold_energy_change_j_m2": 10.0,
        "lower_cold_energy_change_j_m2": -2.0,
        "cold_content_export_j_m2": 3.0,
        "total_cold_after_j_m2": 89.0,
    }


def v6_row(day_index: int) -> dict[str, object]:
    statuses = [
        {"evaluated": hour == 0, "reason": "evaluated" if hour == 0 else "operator_not_selected"}
        for hour in range(24)
    ]
    return {
        "schema": "openwepp-r7h-direct-production-snow-trace-v6",
        "day_index": day_index,
        "lane_index": 0,
        "stage3_shadow_complete_energy_j_m2": 10_805.0,
        "stage3_operator_reconciliation": {
            "schema_version": 6,
            "hourly_status": statuses,
            "tuples": [v6_tuple()],
        },
    }


class ConsumerTests(unittest.TestCase):
    def test_v4_aggregate_custody_closes_without_primitive_aliases(self) -> None:
        dates = [dt.date(2000, 1, 1), dt.date(2000, 1, 2)]
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "v4.jsonl"
            write_jsonl(path, [v4_row(0, [1.0] * 24), v4_row(1, [2.0] * 24)])
            parsed = consumer.parse_v4(path, dates)
        self.assertEqual(parsed[dates[0]], 24.0)
        self.assertEqual(parsed[dates[1]], 48.0)

    def test_v4_rejects_adjacent_daily_alias(self) -> None:
        dates = [dt.date(2000, 1, 1)]
        row = v4_row(0, [1.0] * 24)
        del row["stage3_shadow_hourly_complete_energy_j_m2"]
        row["stage3_surface_energy_j_m2"] = 24.0
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "alias.jsonl"
            write_jsonl(path, [row])
            with self.assertRaises(consumer.ReconstructionError):
                consumer.parse_v4(path, dates)

    def test_v6_reconstructs_primitives_mass_cold_and_support(self) -> None:
        dates = [dt.date(2000, 1, 1)]
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "v6.jsonl"
            write_jsonl(path, [v6_row(0)])
            parsed = consumer.parse_v6(path, dates)
        self.assertEqual(parsed[dates[0]], 10_805.0)

    def test_v6_rejects_double_counted_conduction(self) -> None:
        dates = [dt.date(2000, 1, 1)]
        row = v6_row(0)
        companion = row["stage3_operator_reconciliation"]
        assert isinstance(companion, dict)
        tuples = companion["tuples"]
        assert isinstance(tuples, list)
        assert isinstance(tuples[0], dict)
        tuples[0]["legacy_sequential_complete_j_m2"] = 10_810.0
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "double.jsonl"
            write_jsonl(path, [row])
            with self.assertRaises(consumer.ReconstructionError):
                consumer.parse_v6(path, dates)

    def test_factorial_effects_are_per_water_year_before_median(self) -> None:
        cells = {
            "E00": {year: float(year) for year in range(1990, 2025)},
            "E01": {year: float(year + 10) for year in range(1990, 2025)},
            "E10": {year: float(year + 1) for year in range(1990, 2025)},
            "E11": {year: float(year + 13) for year in range(1990, 2025)},
        }
        rows = consumer.effect_rows(cells)
        self.assertEqual(len(rows), 35)
        self.assertEqual(rows[0]["source_canonical_j_m2"], 1.0)
        self.assertEqual(rows[0]["source_development_j_m2"], 3.0)
        self.assertEqual(rows[0]["interaction_j_m2"], 2.0)

    def test_median_failure_triggers_when_each_wy_scale_gate_passes(self) -> None:
        rows = []
        for year in range(1990, 2025):
            rows.append(
                {
                    "water_year": year,
                    "E10_j_m2": 1.0e12 + 0.5,
                    "E00_j_m2": 1.0e12,
                    "source_canonical_j_m2": 0.5,
                }
            )
        gate = consumer.source_gate(
            rows, "source_canonical_j_m2", "E10_j_m2", "E00_j_m2"
        )
        self.assertEqual(gate["water_year_failures"], [])
        self.assertFalse(gate["pass"])
        self.assertTrue(gate["checkpoint_trigger"])


if __name__ == "__main__":
    unittest.main()
