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

UPSTREAM_TEST = consumer.REPO / (
    "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-"
    "operator-reconciliation-001/tools/test_run_operator_reconciliation.py"
)
UPSTREAM_SPEC = importlib.util.spec_from_file_location(
    "predecessor_bridge_reviewed_v6_fixtures", UPSTREAM_TEST
)
if UPSTREAM_SPEC is None or UPSTREAM_SPEC.loader is None:
    raise RuntimeError(f"cannot load {UPSTREAM_TEST}")
upstream = importlib.util.module_from_spec(UPSTREAM_SPEC)
sys.modules[UPSTREAM_SPEC.name] = upstream
UPSTREAM_SPEC.loader.exec_module(upstream)


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


def v6_row(day_index: int) -> dict[str, object]:
    tuple_row = upstream.synthetic_tuple("sequential_resolved_shadow_v1")
    row = upstream.v6_row(tuple_row, "sequential_resolved_shadow_v1")
    row["day_index"] = day_index
    duration = float(tuple_row["duration_seconds"])
    total = float(tuple_row["legacy_sequential_complete_j_m2"])
    row.update(
        {
            "stage3_evaluation_carrier_id": "stage3_complete_carrier_v1",
            "stage3_evaluation_cadence_id": "stage3_dynamic_substep_with_hourly_forcing_v1",
            "stage3_evaluation_claim_class": "bounded_response_experiment",
            "stage3_evaluation_complete_arm_shortwave_j_m2": float(tuple_row["net_shortwave_w_m2"]) * duration,
            "stage3_evaluation_complete_arm_longwave_j_m2": float(tuple_row["net_longwave_w_m2"]) * duration,
            "stage3_evaluation_complete_arm_sensible_j_m2": float(tuple_row["sensible_flux_w_m2"]) * duration,
            "stage3_evaluation_complete_arm_latent_j_m2": float(tuple_row["latent_flux_w_m2"]) * duration,
            "stage3_evaluation_complete_arm_advected_j_m2": float(tuple_row["precipitation_advected_flux_w_m2"]) * duration,
            "stage3_evaluation_complete_arm_internal_active_lower_conduction_j_m2": float(tuple_row["internal_active_lower_conduction_j_m2"]),
            "stage3_evaluation_complete_arm_total_j_m2": total,
            "stage3_evaluation_hourly_complete_energy_j_m2": [total] + [0.0] * 23,
            "stage3_evaluation_hourly_evaluated_seconds": [duration] + [0.0] * 23,
            "stage3_evaluation_hourly_requested_seconds": [3_600.0] * 24,
            "stage3_evaluation_hourly_complete_carrier_evaluated": [True] + [False] * 23,
            "stage3_evaluation_evaluated_seconds": duration,
            "stage3_evaluation_requested_seconds": 86_400.0,
            "stage3_evaluation_coverage_fraction": duration / 86_400.0,
            "stage3_maximum_conduction_cancellation_residual_j_m2": 0.0,
        }
    )
    return row


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

    def test_v4_retains_false_applicability_context_without_zero_alias(self) -> None:
        dates = [dt.date(2000, 1, 1)]
        row = v4_row(0, [0.0] * 24)
        row["stage3_energy_enabled"] = False
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "inactive-context.jsonl"
            write_jsonl(path, [row])
            parsed = consumer.parse_v4(path, dates)
        self.assertEqual(parsed[dates[0]], 0.0)

    def test_v6_reconstructs_primitives_mass_cold_and_support(self) -> None:
        dates = [dt.date(2000, 1, 1)]
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "v6.jsonl"
            write_jsonl(path, [v6_row(0)])
            parsed = consumer.parse_v6(path, dates)
        row = v6_row(0)
        self.assertAlmostEqual(
            parsed[dates[0]],
            row["stage3_evaluation_complete_arm_total_j_m2"],
            places=8,
        )

    def test_v6_rejects_double_counted_conduction(self) -> None:
        dates = [dt.date(2000, 1, 1)]
        row = v6_row(0)
        companion = row["stage3_operator_reconciliation"]
        assert isinstance(companion, dict)
        tuples = companion["tuples"]
        assert isinstance(tuples, list)
        assert isinstance(tuples[0], dict)
        tuples[0]["legacy_sequential_complete_j_m2"] = float(
            tuples[0]["legacy_sequential_complete_j_m2"]
        ) + 5.0
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "double.jsonl"
            write_jsonl(path, [row])
            with self.assertRaises(consumer.ReconstructionError):
                consumer.parse_v6(path, dates)

    def test_v6_rejects_primitive_derived_mismatch(self) -> None:
        dates = [dt.date(2000, 1, 1)]
        row = v6_row(0)
        companion = row["stage3_operator_reconciliation"]
        assert isinstance(companion, dict)
        tuples = companion["tuples"]
        assert isinstance(tuples, list) and isinstance(tuples[0], dict)
        tuples[0]["hourly_radiation_mj_m2"] = float(
            tuples[0]["hourly_radiation_mj_m2"]
        ) + 0.25
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "primitive-mismatch.jsonl"
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

    def test_execution_receipt_rejects_empty_matrix(self) -> None:
        frozen = json.loads(consumer.FREEZE_PATH.read_text(encoding="utf-8"))
        receipt = {
            "status": "endpoint_matrix_executed",
            "sources": frozen["sources"],
            "cells": {},
        }
        with self.assertRaises(consumer.ReconstructionError):
            consumer.validate_execution_receipt(receipt, frozen)

    def test_replay_gate_detects_daily_anchor_difference(self) -> None:
        stamp = dt.date(2000, 1, 1)
        gate = consumer.replay_gate(
            {stamp: 2.0},
            {stamp: 1.0},
            [(2000, stamp, stamp)],
        )
        self.assertFalse(gate["pass"])
        self.assertEqual(gate["daily_failure_examples"], ["2000-01-01"])

    def test_classification_uses_forcing_sha_and_failure_class(self) -> None:
        frozen = json.loads(consumer.FREEZE_PATH.read_text(encoding="utf-8"))
        passed = {"pass": True}
        failed = {"pass": False}
        classes = consumer.classify(
            passed,
            failed,
            {"historical": {"pass": True}, "current": {"pass": True}},
            frozen,
        )
        canonical_sha = frozen["forcings"]["canonical"]["sha256"]
        development_sha = frozen["forcings"]["development"]["sha256"]
        self.assertIn(f"SOURCE_INVARIANT_WITHIN_FORCING[{canonical_sha}]", classes)
        self.assertIn(f"PREDECESSOR_NOT_REPRODUCED[{development_sha}]", classes)


if __name__ == "__main__":
    unittest.main()
