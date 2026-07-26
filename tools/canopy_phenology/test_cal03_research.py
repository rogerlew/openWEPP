import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).with_name("cal03_research.py")
SPEC = importlib.util.spec_from_file_location("cal03_research", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


def row(date: str, year: int, before: float, litter: float, decay: float) -> dict:
    after = (before + litter) * decay
    foliar = 1.0 - litter
    return {
        "schema": MODULE.SCHEMA,
        "date": date,
        "year": year,
        "day_of_year": 1,
        "day_index": 0,
        "lane_index": 0,
        "site_id": "site",
        "arm_id": "arm",
        "gsi": {
            "minimum_temperature_indicator": 0.5,
            "vapor_pressure_deficit_indicator": 0.5,
            "photoperiod_indicator": 0.5,
            "photoperiod_hours": 12.0,
            "instantaneous": 0.125,
            "gsi21": 0.125,
            "sample_count": 1,
        },
        "canopy": {
            "structural_biomass_kg_m2": 0.1,
            "evergreen_foliar_biomass_kg_m2": 0.0,
            "total_foliar_biomass_kg_m2": foliar,
            "deciduous_foliar_biomass_kg_m2": foliar,
            "total_aboveground_live_biomass_kg_m2": foliar + 0.1,
            "leaf_area_index_m2_m2": foliar,
            "cover_fraction": foliar,
            "leaf_on_allocation_kg_m2": 0.0,
            "leaf_off_transfer_kg_m2": litter,
        },
        "consumers": {
            "growth_live_foliar_biomass_kg_m2": foliar,
            "snow_canopy_cover_fraction": foliar,
            "interception_leaf_area_index_m2_m2": foliar,
            "interception_canopy_cover_fraction": foliar,
            "interception_live_biomass_kg_m2": foliar,
            "interception_m": 0.0,
            "et_leaf_area_index_m2_m2": foliar,
            "et_canopy_cover_fraction": foliar,
            "runoff_m": 0.0,
            "erosion_canopy_cover_fraction": None,
            "frost_residue_depth_m": None,
        },
        "residue": {
            "leaf_litter_input_kg_m2": litter,
            "needle_litter_input_kg_m2": None,
            "fine_woody_litter_input_kg_m2": None,
            "total_litter_input_kg_m2": litter,
            "surface_residue_before_kg_m2": before,
            "surface_residue_after_kg_m2": after,
            "decomposition_loss_kg_m2": before + litter - after,
            "surface_decay_factor": decay,
            "residue_depth_m": after * 0.01,
        },
    }


class Cal03ResearchTests(unittest.TestCase):
    def test_validates_and_reconciles_shadow_cohorts(self) -> None:
        first = row("2000-01-01", 2000, 2.0, 0.2, 0.9)
        second = row("2000-01-02", 2000, first["residue"]["surface_residue_after_kg_m2"], 0.1, 0.8)
        second_foliar = first["canopy"]["total_foliar_biomass_kg_m2"] - 0.1
        second["canopy"]["total_foliar_biomass_kg_m2"] = second_foliar
        second["canopy"]["deciduous_foliar_biomass_kg_m2"] = second_foliar
        second["canopy"]["leaf_area_index_m2_m2"] = second_foliar
        second["canopy"]["cover_fraction"] = second_foliar
        second["consumers"]["growth_live_foliar_biomass_kg_m2"] = second_foliar
        second["consumers"]["et_leaf_area_index_m2_m2"] = second_foliar
        second["consumers"]["et_canopy_cover_fraction"] = second_foliar
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / "trace.jsonl"
            path.write_text("\n".join(json.dumps(value) for value in (first, second)) + "\n")
            records = MODULE.read_records(path)
            annual, cohorts = MODULE.annual_diagnostics(records)
        self.assertEqual(len(annual), 1)
        self.assertEqual(len(cohorts), 1)
        self.assertAlmostEqual(
            cohorts[0]["total_kg_m2"],
            second["residue"]["surface_residue_after_kg_m2"],
        )
        self.assertAlmostEqual(annual[0]["net_foliar_change_kg_m2"], -0.3)

    def test_rejects_consumer_alias_mismatch(self) -> None:
        value = row("2000-01-01", 2000, 2.0, 0.2, 0.9)
        value["consumers"]["et_canopy_cover_fraction"] = 0.25
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / "trace.jsonl"
            path.write_text(json.dumps(value) + "\n")
            with self.assertRaisesRegex(MODULE.ResearchError, "producer/consumer mismatch"):
                MODULE.read_records(path)

    def test_accepts_production_day_lane_interleaving(self) -> None:
        lane_zero = row("2000-01-01", 2000, 2.0, 0.0, 1.0)
        lane_one = row("2000-01-01", 2000, 2.0, 0.0, 1.0)
        lane_one["lane_index"] = 1
        lane_zero_next = row("2000-01-02", 2000, 2.0, 0.0, 1.0)
        lane_zero_next["day_index"] = 1
        lane_one_next = row("2000-01-02", 2000, 2.0, 0.0, 1.0)
        lane_one_next["day_index"] = 1
        lane_one_next["lane_index"] = 1
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / "trace.jsonl"
            path.write_text(
                "\n".join(
                    json.dumps(value)
                    for value in (lane_zero, lane_one, lane_zero_next, lane_one_next)
                )
                + "\n"
            )
            records = MODULE.read_records(path)
        self.assertEqual(len(records), 4)

    def test_rejects_missing_campaign_identity(self) -> None:
        value = row("2000-01-01", 2000, 2.0, 0.0, 1.0)
        value["site_id"] = None
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / "trace.jsonl"
            path.write_text(json.dumps(value) + "\n")
            with self.assertRaisesRegex(MODULE.ResearchError, "site_id"):
                MODULE.read_records(path)

    def test_rejects_missing_nullable_schema_field(self) -> None:
        value = row("2000-01-01", 2000, 2.0, 0.0, 1.0)
        del value["residue"]["needle_litter_input_kg_m2"]
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / "trace.jsonl"
            path.write_text(json.dumps(value) + "\n")
            with self.assertRaisesRegex(MODULE.ResearchError, "needle_litter"):
                MODULE.read_records(path)

    def test_equilibrium_requires_all_subsequent_windows_to_pass(self) -> None:
        annual = [
            {
                "site_id": "site",
                "arm_id": "arm",
                "lane_index": 0,
                "year": 2000 + index,
                "residue_end_kg_m2": 1.0 if index < 10 else 2.0,
                "residue_year_over_year_drift_kg_m2": "",
                "first_practical_equilibrium_year": "",
                "cal02_years_91_100_equilibrium": "NOT_EVALUABLE_PERIOD_LT_100",
            }
            for index in range(20)
        ]
        MODULE._annotate_equilibrium(annual, "site", "arm", 0)
        self.assertEqual(annual[0]["first_practical_equilibrium_year"], 2019)


if __name__ == "__main__":
    unittest.main()
