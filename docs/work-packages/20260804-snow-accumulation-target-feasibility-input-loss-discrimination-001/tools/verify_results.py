#!/usr/bin/env python3
"""Independently verify retained input-versus-loss tables and reduction."""

from __future__ import annotations

import csv
import datetime as dt
import hashlib
import json
import statistics
from collections import defaultdict
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
TARGET = REPO / "target/snow_accumulation_target_feasibility_input_loss_discrimination_v2"
TABLES = TARGET / "tables"
SITES = (
    "snotel_mica_creek_st_joe_id",
    "snotel_niwot_co",
    "snotel_paradise_wa",
    "snotel_snowbird_ut",
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def rows(name: str) -> list[dict[str, str]]:
    with (TABLES / name).open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def med(values: list[float]) -> float:
    return statistics.median(values)


def frac(values: list[bool]) -> float:
    return sum(values) / len(values)


def assert_close(actual: float | None, expected: float | None, label: str) -> None:
    if actual is None or expected is None or abs(actual - expected) > 1e-12:
        raise RuntimeError(f"{label}: {actual} != {expected}")


def verify_disjoint(
    input_rows: list[dict[str, str]], start_field: str, end_field: str, label: str
) -> None:
    grouped: dict[tuple[str, int], list[tuple[dt.date, dt.date]]] = defaultdict(list)
    for row in input_rows:
        grouped[(row["site"], int(row["water_year"]))].append(
            (dt.date.fromisoformat(row[start_field]), dt.date.fromisoformat(row[end_field]))
        )
    for key, intervals in grouped.items():
        intervals.sort()
        for previous, current in zip(intervals, intervals[1:]):
            if current[0] <= previous[1]:
                raise RuntimeError(f"overlapping {label} intervals for {key}")


def main() -> int:
    freeze = json.loads((PACKAGE / "artifacts/analysis-freeze.json").read_text(encoding="utf-8"))
    constants = freeze["constants"]
    result_path = TARGET / "results.json"
    result = json.loads(result_path.read_text(encoding="utf-8"))
    mass = rows("annual-mass.csv")
    events = rows("cold-events.csv")
    dry = [row for row in rows("dry-annual.csv") if row["temperature_stratum"] == "all"]
    dry_intervals = rows("dry-intervals.csv")
    if len(mass) != 154:
        raise RuntimeError("unexpected annual mass inventory")
    if len(events) != result["counts"]["cold_event_rows"]:
        raise RuntimeError("cold event count mismatch")
    if len(dry_intervals) != result["counts"]["dry_interval_rows"]:
        raise RuntimeError("dry interval count mismatch")
    verify_disjoint(events, "event_start", "event_end", "cold event")
    verify_disjoint(dry_intervals, "interval_start", "interval_end", "dry")

    recomputed = []
    for site in SITES:
        source = next(row for row in result["site_summary"] if row["site"] == site)
        sm = [row for row in mass if row["site"] == site]
        se = [row for row in events if row["site"] == site]
        sd = [row for row in dry if row["site"] == site]
        ceiling = [float(row["current_input_mass_ceiling_ratio"]) for row in sm]
        all_phase = [float(row["all_phase_to_observed_gain_ratio"]) for row in se]
        snowfall = [float(row["modeled_snowfall_to_observed_gain_ratio"]) for row in se]
        differences = [float(row["modeled_minus_observed_loss_m"]) for row in sd]
        values: dict[str, float] = {
            "median_current_input_mass_ceiling_ratio": med(ceiling),
            "fraction_years_current_input_ceiling_below_observed_peak": frac([value < 1.0 for value in ceiling]),
            "median_storage_effective_input_ratio": med([float(row["storage_effective_input_ratio"]) for row in sm]),
            "median_observed_date_modeled_storage_ratio": med([float(row["observed_date_modeled_storage_ratio"]) for row in sm]),
            "median_within_window_modeled_peak_ratio": med([float(row["within_window_modeled_peak_ratio"]) for row in sm]),
            "median_modeled_pack_loss_to_observed_peak": med([float(row["modeled_pack_loss_to_observed_peak"]) for row in sm]),
            "median_cold_event_all_phase_to_observed_gain_ratio": med(all_phase),
            "fraction_cold_events_all_phase_ratio_below_materiality": frac([value < constants["event_ratio_materiality"] for value in all_phase]),
            "median_cold_event_snowfall_to_observed_gain_ratio": med(snowfall),
            "fraction_cold_events_snowfall_ratio_below_materiality": frac([value < constants["event_ratio_materiality"] for value in snowfall]),
            "median_cold_event_storage_change_to_observed_gain_ratio": med([float(row["modeled_storage_change_to_observed_gain_ratio"]) for row in se]),
            "median_cold_event_pack_loss_to_observed_gain_ratio": med([float(row["modeled_pack_loss_to_observed_gain_ratio"]) for row in se]),
            "median_dry_annual_observed_loss_m": med([float(row["observed_wteq_loss_m"]) for row in sd]),
            "median_dry_annual_modeled_pack_loss_m": med([float(row["modeled_pack_loss_m"]) for row in sd]),
            "median_dry_annual_modeled_minus_observed_loss_m": med(differences),
            "fraction_dry_annuals_modeled_loss_exceeds_observed": frac([value > 0.0 for value in differences]),
        }
        for field, value in values.items():
            assert_close(value, float(source[field]), f"{site} {field}")
        mass_signal = (
            len(sm) >= constants["minimum_primary_years_per_site"]
            and values["median_current_input_mass_ceiling_ratio"] < constants["mass_ceiling_ratio_materiality"]
            and values["fraction_years_current_input_ceiling_below_observed_peak"] >= constants["site_direction_fraction"]
        )
        all_phase_signal = (
            len(se) >= constants["minimum_cold_events_per_site"]
            and values["median_cold_event_all_phase_to_observed_gain_ratio"] < constants["event_ratio_materiality"]
            and values["fraction_cold_events_all_phase_ratio_below_materiality"] >= constants["event_direction_fraction"]
        )
        snowfall_signal = (
            len(se) >= constants["minimum_cold_events_per_site"]
            and values["median_cold_event_snowfall_to_observed_gain_ratio"] < constants["event_ratio_materiality"]
            and values["fraction_cold_events_snowfall_ratio_below_materiality"] >= constants["event_direction_fraction"]
        )
        dry_signal = (
            len(sd) >= constants["minimum_dry_annuals_per_site"]
            and values["median_dry_annual_modeled_minus_observed_loss_m"] >= constants["annual_dry_loss_difference_materiality_m"]
            and values["fraction_dry_annuals_modeled_loss_exceeds_observed"] >= constants["site_direction_fraction"]
        )
        for field, value in (
            ("mass_ceiling_site_signal", mass_signal),
            ("cold_event_all_phase_site_signal", all_phase_signal),
            ("cold_event_snowfall_site_signal", snowfall_signal),
            ("dry_loss_site_signal", dry_signal),
        ):
            if bool(source[field]) != value:
                raise RuntimeError(f"{site} {field} mismatch")
        recomputed.append(
            {
                "site": site,
                "mass": mass_signal,
                "all_phase": all_phase_signal,
                "snowfall": snowfall_signal,
                "dry_loss": dry_signal,
            }
        )

    systemic = int(constants["systemic_site_count"])
    input_evidence = (
        sum(row["mass"] for row in recomputed) >= systemic
        or sum(row["all_phase"] for row in recomputed) >= systemic
        or sum(row["snowfall"] for row in recomputed) >= systemic
    )
    loss_evidence = sum(row["dry_loss"] for row in recomputed) >= systemic
    if input_evidence and loss_evidence:
        verdict = "MULTIFACTOR_INPUT_AND_LOSS_SIGNAL"
    elif input_evidence:
        verdict = "INPUT_PRIORITY_SIGNAL"
    elif loss_evidence:
        verdict = "LOSS_PRIORITY_SIGNAL"
    else:
        verdict = "UNRESOLVED_OR_COVERAGE_LIMITED"
    if verdict != result["cohort_summary"]["verdict"]:
        raise RuntimeError("cohort verdict mismatch")

    figure_manifest = json.loads((TARGET / "figure-manifest.json").read_text(encoding="utf-8"))
    for row in figure_manifest["figures"]:
        if sha256(REPO / row["figure"]) != row["figure_sha256"]:
            raise RuntimeError(f"figure hash mismatch: {row['figure']}")
        if sha256(REPO / row["source"]) != row["source_sha256"]:
            raise RuntimeError(f"figure source hash mismatch: {row['source']}")

    receipt: dict[str, Any] = {
        "schema_version": 1,
        "status": "PASS",
        "evidence_mode": "Ran: independent retained-table reduction and artifact verification",
        "result_sha256": sha256(result_path),
        "annual_mass_row_count": len(mass),
        "cold_event_row_count": len(events),
        "dry_interval_row_count": len(dry_intervals),
        "recomputed_site_count": len(recomputed),
        "recomputed_verdict": verdict,
        "figure_count": len(figure_manifest["figures"]),
    }
    output = TARGET / "independent-verification.json"
    output.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
