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
from unittest import mock

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
    def run_synthetic_checkpoint_reconstruction(
        self,
        lanes: list[str],
        values: dict[str, list[float]],
        *,
        current_anchor_offset: dict[str, float] | None = None,
    ) -> dict[str, object]:
        frozen = json.loads(consumer.FREEZE_PATH.read_text(encoding="utf-8"))
        checkpoints = frozen["checkpoint_grouping"]["checkpoints"]
        stamp = dt.date(2000, 1, 1)
        current_anchor_offset = current_anchor_offset or {}
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "checkpoint-search").mkdir(parents=True)
            (root / "results").mkdir()
            (root / "execution-receipt.json").write_text("{}", encoding="utf-8")
            result = {
                "execution_head": "a" * 40,
                "checkpoint_lanes_triggered": lanes,
            }
            (root / "results/predecessor-bridge-results.json").write_text(
                json.dumps(result), encoding="utf-8"
            )
            runs = {}
            for forcing in lanes:
                runs[forcing] = {}
                for index, (source_sha, _) in enumerate(checkpoints):
                    checkpoint_id = f"{index:02d}-{source_sha}"
                    runs[forcing][checkpoint_id] = {
                        "modes": {
                            "legacy": {
                                "outputs": {
                                    "files": [
                                        {
                                            "path": consumer.TRACE_NAME,
                                            "sha256": "b" * 64,
                                        }
                                    ]
                                }
                            }
                        }
                    }
            execution = {"status": "executed", "runs": runs}
            if not lanes:
                execution = {"status": "not_triggered", "runs": {}}
            (root / "checkpoint-search/execution-receipt.json").write_text(
                json.dumps(execution), encoding="utf-8"
            )
            fixture_sha = {}
            for forcing in lanes:
                fixture = root / "fixtures" / forcing
                fixture.mkdir(parents=True)
                climate = fixture / "p8.cli"
                climate.write_bytes(forcing.encode())
                fixture_sha[forcing] = consumer.sha256(climate)
                frozen["forcings"][forcing]["sha256"] = fixture_sha[forcing]
            frozen["forcings"]["date_count"] = 1
            frozen["forcings"]["first_date"] = stamp.isoformat()
            frozen["forcings"]["last_date"] = stamp.isoformat()
            freeze_path = root / "protocol-freeze.json"
            freeze_path.write_text(json.dumps(frozen), encoding="utf-8")

            def parse_trace(
                path: Path, dates: list[dt.date], expected_sha256: str
            ) -> dict[dt.date, float]:
                del dates, expected_sha256
                parts = path.parts
                if "checkpoint-search" in parts:
                    forcing = parts[parts.index("runs") + 1]
                    checkpoint_id = parts[parts.index("runs") + 2]
                    return {stamp: values[forcing][int(checkpoint_id[:2])]}
                cell = parts[parts.index("runs") + 1]
                forcing = "canonical" if cell in {"E00", "E10"} else "development"
                if cell in {"E00", "E01"}:
                    return {stamp: values[forcing][0]}
                return {
                    stamp: values[forcing][-1]
                    + current_anchor_offset.get(forcing, 0.0)
                }

            def annualize_one(
                daily: dict[dt.date, float], windows: object
            ) -> dict[int, float]:
                del windows
                value = daily[stamp]
                return {year: value for year in range(1990, 2025)}

            trace_hashes = {
                (cell, "legacy"): "c" * 64
                for cell in ("E00", "E01", "E10", "E11")
            }
            with (
                mock.patch.object(consumer, "OUTPUT", root),
                mock.patch.object(consumer, "FREEZE_PATH", freeze_path),
                mock.patch.object(
                    consumer,
                    "validate_execution_receipt",
                    return_value=trace_hashes,
                ),
                mock.patch.object(
                    consumer,
                    "validate_checkpoint_execution_receipt",
                    return_value=lanes,
                ),
                mock.patch.object(consumer, "climate_dates", return_value=[stamp]),
                mock.patch.object(
                    consumer, "parse_checkpoint_trace", side_effect=parse_trace
                ),
                mock.patch.object(consumer, "annualize", side_effect=annualize_one),
            ):
                consumer.reconstruct_checkpoints()
            return json.loads(
                (root / "checkpoint-search/checkpoint-results.json").read_text(
                    encoding="utf-8"
                )
            )

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

    def test_replay_failure_suppresses_source_success_classes(self) -> None:
        frozen = json.loads(consumer.FREEZE_PATH.read_text(encoding="utf-8"))
        classes = consumer.classify(
            {"pass": True},
            {"pass": True},
            {"historical": {"pass": False}, "current": {"pass": True}},
            frozen,
        )
        self.assertEqual(
            classes,
            ["INPUT_OR_ENDPOINT_REPLAY_FAILURE", "FORCING_IDENTITY_DIFFERENCE"],
        )

    def test_v5_aggregate_adapter_is_checkpoint_only(self) -> None:
        dates = [dt.date(2000, 1, 1)]
        row = v4_row(0, [1.0] * 24)
        row["schema"] = "openwepp-r7h-direct-production-snow-trace-v5"
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "v5.jsonl"
            write_jsonl(path, [row])
            digest = consumer.sha256(path)
            parsed = consumer.parse_checkpoint_trace(path, dates, digest)
            with self.assertRaises(consumer.ReconstructionError):
                consumer.parse_v4(path, dates, expected_sha256=digest)
        self.assertEqual(parsed[dates[0]], 24.0)

    def test_annual_difference_gate_checks_each_year_and_median(self) -> None:
        left = {year: 1.0e12 for year in range(1990, 2025)}
        right = dict(left)
        self.assertTrue(consumer.annual_difference_gate(left, right)["pass"])
        right[2000] += 3.0
        gate = consumer.annual_difference_gate(left, right)
        self.assertFalse(gate["pass"])
        self.assertEqual(gate["water_year_failures"], [2000])

    def test_first_divergent_transition_is_ordered_and_nullable(self) -> None:
        self.assertIsNone(
            consumer.first_divergent_transition([{"pass": True}, {"pass": True}])
        )
        second = {"pass": False, "left": "01", "right": "02"}
        self.assertIs(
            consumer.first_divergent_transition([{"pass": True}, second]), second
        )

    def test_checkpoint_receipt_rejects_malformed_untriggered_custody(self) -> None:
        frozen = json.loads(consumer.FREEZE_PATH.read_text(encoding="utf-8"))
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            endpoint_receipt_path = root / "endpoint.json"
            endpoint_result_path = root / "result.json"
            endpoint_receipt = {"execution_head": "a" * 40}
            endpoint_result = {
                "execution_head": "a" * 40,
                "checkpoint_lanes_triggered": [],
            }
            endpoint_receipt_path.write_text(
                json.dumps(endpoint_receipt), encoding="utf-8"
            )
            endpoint_result_path.write_text(
                json.dumps(endpoint_result), encoding="utf-8"
            )
            execution = {
                "schema_version": 1,
                "execution_head": "a" * 40,
                "protocol_sha256": consumer.sha256(consumer.FREEZE_PATH),
                "endpoint_execution_receipt_sha256": consumer.sha256(
                    endpoint_receipt_path
                ),
                "endpoint_result_sha256": consumer.sha256(endpoint_result_path),
                "triggered_lanes": [],
                "checkpoint_count": 14,
                "status": "not_triggered",
                "builds": {},
                "runs": {},
            }
            self.assertEqual(
                consumer.validate_checkpoint_execution_receipt(
                    execution,
                    endpoint_receipt,
                    endpoint_receipt_path,
                    endpoint_result,
                    endpoint_result_path,
                    frozen,
                ),
                [],
            )
            execution["runs"] = {"development": {}}
            with self.assertRaises(consumer.ReconstructionError):
                consumer.validate_checkpoint_execution_receipt(
                    execution,
                    endpoint_receipt,
                    endpoint_receipt_path,
                    endpoint_result,
                    endpoint_result_path,
                    frozen,
                )

    def test_full_checkpoint_reconstruction_no_trigger(self) -> None:
        result = self.run_synthetic_checkpoint_reconstruction([], {})
        self.assertEqual(result["status"], "not_triggered")
        self.assertEqual(result["decision_classes"], [])

    def test_full_checkpoint_reconstruction_one_lane_no_divergence(self) -> None:
        cumulative_subtolerance = [1.0 + 9.0e-7 * index for index in range(14)]
        result = self.run_synthetic_checkpoint_reconstruction(
            ["canonical"], {"canonical": cumulative_subtolerance}
        )
        self.assertIsNone(result["first_divergent_transition"]["canonical"])
        self.assertEqual(
            result["decision_classes"],
            ["MULTIFACTOR_OR_UNOBSERVED_PREDECESSOR_BOUNDARY"],
        )

    def test_full_checkpoint_reconstruction_both_lanes_different_intervals(self) -> None:
        canonical = [1.0] * 3 + [2.0] * 11
        development = [1.0] * 5 + [2.0] * 9
        result = self.run_synthetic_checkpoint_reconstruction(
            ["canonical", "development"],
            {"canonical": canonical, "development": development},
        )
        self.assertNotEqual(
            result["first_divergent_transition"]["canonical"]["right"],
            result["first_divergent_transition"]["development"]["right"],
        )
        self.assertEqual(
            result["decision_classes"],
            [
                "MULTIFACTOR_OR_UNOBSERVED_PREDECESSOR_BOUNDARY",
                "SOURCE_BY_FORCING_INTERACTION_DESCRIPTIVE",
            ],
        )

    def test_full_checkpoint_reconstruction_both_lanes_same_interval(self) -> None:
        canonical = [1.0] * 3 + [2.0] * 11
        development = [3.0] * 3 + [4.0] * 11
        result = self.run_synthetic_checkpoint_reconstruction(
            ["canonical", "development"],
            {"canonical": canonical, "development": development},
        )
        self.assertEqual(
            result["first_divergent_transition"]["canonical"]["right"],
            result["first_divergent_transition"]["development"]["right"],
        )
        self.assertEqual(result["decision_classes"], [])

    def test_full_checkpoint_reconstruction_rejects_endpoint_anchor_drift(self) -> None:
        with self.assertRaises(consumer.ReconstructionError):
            self.run_synthetic_checkpoint_reconstruction(
                ["canonical"],
                {"canonical": [1.0] * 13 + [2.0]},
                current_anchor_offset={"canonical": 1.0},
            )

    def test_common_fixture_validation_rejects_mutation(self) -> None:
        frozen = json.loads(consumer.FREEZE_PATH.read_text(encoding="utf-8"))
        with tempfile.TemporaryDirectory() as temporary:
            fixture = Path(temporary)
            expected = dict(frozen["common_fixture_sha256"])
            expected["p8.cli"] = frozen["forcings"]["canonical"]["sha256"]
            for name in expected:
                if name == "p8.cli":
                    source = consumer.REPO / frozen["forcings"]["canonical"]["path"]
                else:
                    source = (
                        consumer.REPO
                        / "target/snow_stage3_operator_reconciliation_v3/fixtures"
                        / "snotel_snowbird_ut"
                        / name
                    )
                (fixture / name).write_bytes(source.read_bytes())
            consumer.validate_frozen_fixture(fixture, "canonical", frozen)
            (fixture / "p8.sol").write_bytes(b"mutated")
            with self.assertRaises(consumer.ReconstructionError):
                consumer.validate_frozen_fixture(fixture, "canonical", frozen)

    def test_protected_output_comparison_rejects_mutation(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            control = root / "control"
            legacy = root / "legacy"
            control.mkdir()
            legacy.mkdir()
            for suffix in (".hbp", ".wat.parquet", ".loss.json"):
                (control / f"case{suffix}").write_bytes(b"same")
                (legacy / f"case{suffix}").write_bytes(b"same")
            self.assertEqual(
                set(
                    consumer.compare_protected_outputs(
                        control, legacy, context="test"
                    ).values()
                ),
                {True},
            )
            (legacy / "case.hbp").write_bytes(b"different")
            with self.assertRaises(consumer.ReconstructionError):
                consumer.compare_protected_outputs(control, legacy, context="test")


if __name__ == "__main__":
    unittest.main()
