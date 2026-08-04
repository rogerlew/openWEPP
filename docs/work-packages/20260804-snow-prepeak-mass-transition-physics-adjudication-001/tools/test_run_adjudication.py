#!/usr/bin/env python3
"""Focused tests for the frozen snow mass-transition analysis operators."""

from __future__ import annotations

import datetime as dt
import importlib.util
import sys
import unittest
from pathlib import Path

sys.dont_write_bytecode = True

TOOL = Path(__file__).with_name("run_adjudication.py")
SPEC = importlib.util.spec_from_file_location("snow_mass_adjudication_tested", TOOL)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load {TOOL}")
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules["snow_mass_adjudication_tested"] = MODULE
SPEC.loader.exec_module(MODULE)


def hour(applied: float, temperature: float, snowfall: float, rain: float) -> dict:
    return {
        "air_temperature_c": temperature,
        "coe_melt_amelt_m": applied,
        "coe_melt_applied_m": applied,
        "coe_melt_bmelt_m": 0.0,
        "coe_melt_cmelt_m": 0.0,
        "coe_melt_dmelt_m": 0.0,
        "pack_depth_before_m": 0.5,
        "radiation_mj_m2": 1.0,
        "rain_m": rain,
        "snowfall_swe_m": snowfall,
    }


def trace_row() -> dict:
    return {
        "accumulation_m": 0.001,
        "accumulation_melt_hourly": [
            hour(0.003, 1.0, 0.001, 0.0),
            hour(-0.002, -1.0, 0.0, 0.0),
        ],
        "liquid_water_released_m": 0.001,
        "liquid_water_retained_after_m": 0.002,
        "liquid_water_retained_before_m": 0.001,
        "rain_released_m": 0.001,
        "rain_retained_m": 0.0,
        "raw_melt_m": 0.001,
        "routed_melt_m": 0.003,
        "runtime_swe_after_m": 0.099,
        "runtime_swe_before_m": 0.1,
        "snow_layers_after": [
            {"liquid_water_m": 0.0015, "refrozen_liquid_m": 0.0002}
        ],
        "snow_layers_before": [
            {"liquid_water_m": 0.001, "refrozen_liquid_m": 0.0001}
        ],
        "snowpack_swe_loss_m": 0.002,
        "stage3_cold_content_after_j_m2": 0.0,
        "stage3_cold_content_before_j_m2": 333550.0,
        "stage3_incoming_liquid_m": 0.003,
        "stage3_liquid_closure_residual_m": 0.0,
        "stage3_refrozen_liquid_m": 0.0005,
        "stage3_retained_liquid_delta_m": 0.0005,
        "stage3_routed_liquid_m": 0.002,
        "sublimation_m": 0.0,
    }


class FrozenOperatorTests(unittest.TestCase):
    def test_daily_local_signed_opportunity(self) -> None:
        reduced = MODULE.reduce_trace_row(dt.date(2024, 1, 2), trace_row())
        self.assertAlmostEqual(reduced["daily_local_signed_opportunity_m"], 0.002)
        self.assertTrue(reduced["mixed_signed_hour_day"])

    def test_linked_closures_and_layer_delta_are_distinct(self) -> None:
        reduced = MODULE.reduce_trace_row(dt.date(2024, 1, 2), trace_row())
        self.assertAlmostEqual(reduced["storage_closure_residual_m"], 0.0)
        self.assertAlmostEqual(reduced["handoff_closure_residual_m"], 0.0)
        self.assertAlmostEqual(reduced["stage3_reconstructed_residual_m"], 0.0)
        self.assertAlmostEqual(reduced["layer_liquid_store_day_delta_m"], 0.0005)
        self.assertAlmostEqual(reduced["accumulation_hourly_residual_m"], 0.0)

    def test_cold_indices_respect_source_limits(self) -> None:
        reduced = MODULE.reduce_trace_row(dt.date(2024, 1, 2), trace_row())
        self.assertAlmostEqual(reduced["post_coe_stage3_cold_opportunity_m"], 0.001)
        self.assertAlmostEqual(reduced["solid_source_limited_cold_index_m"], 0.001)

    def test_class_boundaries_and_zero_denominator(self) -> None:
        self.assertEqual(MODULE.temperature_class(0.0), "le_0_c")
        self.assertEqual(MODULE.temperature_class(2.0), "gt_0_le_2_c")
        self.assertEqual(MODULE.temperature_class(2.1), "gt_2_c")
        self.assertIsNone(MODULE.safe_ratio(1.0, 0.0))

    def test_top_level_handoff_is_not_stage3_routed_alias(self) -> None:
        reduced = MODULE.reduce_trace_row(dt.date(2024, 1, 2), trace_row())
        self.assertFalse(
            MODULE.operands_differ(
                reduced["top_level_routed_melt_m"], reduced["stage3_incoming_m"]
            )
        )
        self.assertTrue(
            MODULE.operands_differ(
                reduced["top_level_routed_melt_m"], reduced["stage3_routed_m"]
            )
        )

    def test_storage_closure_rejects_reported_accumulation_alias(self) -> None:
        row = trace_row()
        row["accumulation_m"] = 0.005
        reduced = MODULE.reduce_trace_row(dt.date(2024, 1, 2), row)
        self.assertAlmostEqual(reduced["storage_closure_residual_m"], 0.0)
        self.assertAlmostEqual(reduced["accumulation_m"], 0.001)
        self.assertAlmostEqual(reduced["accumulation_hourly_residual_m"], 0.004)


if __name__ == "__main__":
    unittest.main()
