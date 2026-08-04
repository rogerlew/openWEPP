#!/usr/bin/env python3
"""Execute the frozen snow accumulation input-versus-loss diagnostic."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import json
import math
import statistics
from collections import defaultdict
from pathlib import Path
from typing import Any, Iterable


REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
FREEZE_PATH = PACKAGE / "artifacts" / "analysis-freeze.json"
PREDECESSOR = REPO / "docs/work-packages/20260804-snow-prepeak-mass-transition-physics-adjudication-001"
PREDECESSOR_MANIFEST = PREDECESSOR / "artifacts/evidence-manifest.json"
ANNUAL_RESULTS = PREDECESSOR / "artifacts/cross-fixture-results.json"
TRACE_ROOT = REPO / "target/snow_prepeak_mass_transition_physics_adjudication_v2"
TRACE_RECEIPT = TRACE_ROOT / "execution-receipt.json"
FIXTURE_ROOT = REPO / "tests/fixtures/snotel_observed"
OBS_ROOT = FIXTURE_ROOT / "observations/sites"
PRCPSA_PATH = FIXTURE_ROOT / "observations/provenance/snotel_snowbird_ut_prcpsa_diagnostic.json"
DEFAULT_OUTPUT = REPO / "target/snow_accumulation_target_feasibility_input_loss_discrimination_v2"

SITES = (
    "snotel_mica_creek_st_joe_id",
    "snotel_niwot_co",
    "snotel_paradise_wa",
    "snotel_snowbird_ut",
)
DISPLAY = {
    "snotel_mica_creek_st_joe_id": "Mica Creek",
    "snotel_niwot_co": "Niwot",
    "snotel_paradise_wa": "Paradise",
    "snotel_snowbird_ut": "Snowbird",
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def sha256_bytes(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


def finite_float(raw: str | None) -> float | None:
    if raw in (None, ""):
        return None
    value = float(raw)
    return value if math.isfinite(value) else None


def median(values: Iterable[float]) -> float | None:
    materialized = list(values)
    return statistics.median(materialized) if materialized else None


def fraction(values: Iterable[bool]) -> float | None:
    materialized = list(values)
    return sum(materialized) / len(materialized) if materialized else None


def validate_predecessor_authority(
    freeze: dict[str, Any], manifest: dict[str, Any], receipt: dict[str, Any]
) -> None:
    """Bind mutable retained inputs to the predecessor's tracked manifest."""
    expected = freeze["source_identity_expectations"]
    tracked = manifest["tracked_results"]
    if tracked["execution_receipt_sha256"] != expected["predecessor_trace_receipt_sha256"]:
        raise RuntimeError("predecessor manifest receipt identity differs from freeze")
    if tracked["cross_fixture_sha256"] != expected["predecessor_annual_results_sha256"]:
        raise RuntimeError("predecessor manifest annual-result identity differs from freeze")
    if receipt["results"]["cross_fixture"]["sha256"] != expected["predecessor_annual_results_sha256"]:
        raise RuntimeError("predecessor receipt annual-result identity differs from freeze")

    manifest_traces = {row["path"]: row["sha256"] for row in manifest["exact_traces"]}
    for site, expected_hash in expected["trace_sha256_by_site"].items():
        name = f"{site}-adjudication.snow.jsonl"
        record = receipt["sites"][site]["outputs"][name]
        if record["sha256"] != expected_hash:
            raise RuntimeError(f"predecessor receipt trace identity differs for {site}")
        manifest_hash = manifest_traces.get(record["path"])
        if manifest_hash != expected_hash:
            raise RuntimeError(f"predecessor manifest trace identity differs for {site}")


def ratio(numerator: float, denominator: float, zero: float = 1e-12) -> float | None:
    return numerator / denominator if denominator > zero else None


def parse_cli(path: Path) -> dict[dt.date, dict[str, float]]:
    rows: dict[dt.date, dict[str, float]] = {}
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            fields = line.split()
            if len(fields) != 13:
                continue
            try:
                stamp = dt.date(int(fields[2]), int(fields[1]), int(fields[0]))
                values = list(map(float, fields[3:]))
            except (ValueError, OverflowError):
                continue
            if stamp in rows:
                raise RuntimeError(f"duplicate climate date {stamp} in {path}")
            rows[stamp] = {
                "precip_m": values[0] / 1000.0,
                "tmax_c": values[4],
                "tmin_c": values[5],
            }
    dates = list(rows)
    if not dates or dates != sorted(dates):
        raise RuntimeError(f"invalid climate chronology: {path}")
    return rows


def parse_observations(path: Path) -> dict[dt.date, dict[str, Any]]:
    rows: dict[dt.date, dict[str, Any]] = {}
    with path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            stamp = dt.date.fromisoformat(row["date"])
            if stamp in rows:
                raise RuntimeError(f"duplicate observation date {stamp} in {path}")
            rows[stamp] = {
                "water_year": int(row["water_year"]),
                "swe_m": (
                    None
                    if finite_float(row["observed_swe_mm"]) is None
                    else float(row["observed_swe_mm"]) / 1000.0
                ),
                "precip_cumulative_m": (
                    None
                    if finite_float(row["observed_precip_mm"]) is None
                    else float(row["observed_precip_mm"]) / 1000.0
                ),
                "tmax_c": finite_float(row["observed_tmax_c"]),
                "tmin_c": finite_float(row["observed_tmin_c"]),
            }
    return rows


def guarded_precipitation_increments(
    observations: dict[dt.date, dict[str, Any]], negative_tolerance_m: float
) -> tuple[dict[dt.date, float], dict[str, int]]:
    increments: dict[dt.date, float] = {}
    counts = {"eligible": 0, "gap": 0, "water_year_reset": 0, "negative": 0}
    one_day = dt.timedelta(days=1)
    for stamp in sorted(observations):
        current = observations[stamp]
        previous = observations.get(stamp - one_day)
        if previous is None:
            counts["gap"] += 1
            continue
        if previous["water_year"] != current["water_year"]:
            counts["water_year_reset"] += 1
            continue
        left = previous["precip_cumulative_m"]
        right = current["precip_cumulative_m"]
        if left is None or right is None:
            counts["gap"] += 1
            continue
        difference = right - left
        if difference < -negative_tolerance_m:
            counts["negative"] += 1
            continue
        increments[stamp] = max(difference, 0.0)
        counts["eligible"] += 1
    return increments, counts


def climate_path(site: str) -> Path:
    matches = sorted((FIXTURE_ROOT / site).glob("*.cli"))
    if len(matches) != 1:
        raise RuntimeError(f"expected exactly one climate for {site}")
    return matches[0]


def reduced_trace(
    site: str,
    climate: dict[dt.date, dict[str, float]],
    receipt: dict[str, Any],
    tolerance_m: float,
) -> tuple[dict[dt.date, dict[str, float]], dict[str, Any]]:
    name = f"{site}-adjudication.snow.jsonl"
    record = receipt["sites"][site]["outputs"][name]
    path = REPO / record["path"]
    dates = list(climate)
    digest = hashlib.sha256()
    reduced: dict[dt.date, dict[str, float]] = {}
    maximum_storage_residual = 0.0
    maximum_accumulation_residual = 0.0
    with path.open("rb") as handle:
        for index, raw in enumerate(handle):
            digest.update(raw)
            if index >= len(dates):
                raise RuntimeError(f"trace longer than climate for {site}")
            row = json.loads(raw)
            if row.get("schema") != "openwepp-r7h-direct-production-snow-trace-v4":
                raise RuntimeError(f"unexpected trace schema for {site}")
            snowfall = sum(
                float(hour["snowfall_swe_m"])
                for hour in row["accumulation_melt_hourly"]
            )
            accumulation_residual = float(row["accumulation_m"]) - snowfall
            storage_change = float(row["runtime_swe_after_m"]) - float(
                row["runtime_swe_before_m"]
            )
            storage_expected = (
                snowfall
                + float(row["rain_retained_m"])
                - float(row["snowpack_swe_loss_m"])
                - float(row["sublimation_m"])
            )
            storage_residual = storage_change - storage_expected
            maximum_storage_residual = max(
                maximum_storage_residual, abs(storage_residual)
            )
            maximum_accumulation_residual = max(
                maximum_accumulation_residual, abs(accumulation_residual)
            )
            stamp = dates[index]
            reduced[stamp] = {
                "runtime_swe_before_m": float(row["runtime_swe_before_m"]),
                "runtime_swe_after_m": float(row["runtime_swe_after_m"]),
                "snowfall_m": snowfall,
                "rain_retained_m": float(row["rain_retained_m"]),
                "rain_released_m": float(row["rain_released_m"]),
                "pack_loss_m": float(row["snowpack_swe_loss_m"]),
                "sublimation_m": float(row["sublimation_m"]),
            }
    if len(reduced) != len(dates):
        raise RuntimeError(f"trace shorter than climate for {site}")
    actual_hash = digest.hexdigest()
    if actual_hash != record["sha256"]:
        raise RuntimeError(f"trace hash mismatch for {site}")
    if maximum_storage_residual > tolerance_m:
        raise RuntimeError(f"storage closure exceeds tolerance for {site}")
    if maximum_accumulation_residual > tolerance_m:
        raise RuntimeError(f"hourly accumulation differs for {site}")
    return reduced, {
        "path": record["path"],
        "sha256": actual_hash,
        "size_bytes": path.stat().st_size,
        "row_count": len(reduced),
        "maximum_abs_storage_closure_m": maximum_storage_residual,
        "maximum_abs_accumulation_residual_m": maximum_accumulation_residual,
    }


def group_consecutive(dates: Iterable[dt.date]) -> list[list[dt.date]]:
    groups: list[list[dt.date]] = []
    active: list[dt.date] = []
    for stamp in sorted(dates):
        if active and (stamp - active[-1]).days != 1:
            groups.append(active)
            active = []
        active.append(stamp)
    if active:
        groups.append(active)
    return groups


def merge_cold_event_intervals(
    active_dates: Iterable[dt.date], window_start: dt.date, window_end: dt.date
) -> list[dict[str, Any]]:
    one_day = dt.timedelta(days=1)
    expanded = [
        {
            "start": max(window_start, group[0] - one_day),
            "end": min(window_end, group[-1] + one_day),
            "active_dates": list(group),
        }
        for group in group_consecutive(active_dates)
    ]
    merged: list[dict[str, Any]] = []
    for candidate in expanded:
        if merged and candidate["start"] <= merged[-1]["end"] + one_day:
            merged[-1]["end"] = max(merged[-1]["end"], candidate["end"])
            merged[-1]["active_dates"].extend(candidate["active_dates"])
        else:
            merged.append(candidate)
    return merged


def date_span(start: dt.date, end: dt.date) -> list[dt.date]:
    return [start + dt.timedelta(days=offset) for offset in range((end - start).days + 1)]


def close(left: float, right: float, tolerance: float) -> bool:
    return abs(left - right) <= tolerance


def annual_mass_rows(
    site: str,
    annual: list[dict[str, Any]],
    climate: dict[dt.date, dict[str, float]],
    observations: dict[dt.date, dict[str, Any]],
    gauge: dict[dt.date, float],
    daily: dict[dt.date, dict[str, float]],
    tolerance_m: float,
) -> list[dict[str, Any]]:
    output = []
    for source in annual:
        if source["site"] != site or source["right_censored"]:
            continue
        start = dt.date.fromisoformat(source["window_start"])
        peak = dt.date.fromisoformat(source["observed_peak_date"])
        dates = date_span(start, peak)
        if any(day not in climate or day not in daily for day in dates):
            raise RuntimeError(f"incomplete primary window for {site} WY{source['water_year']}")
        observation = observations.get(peak)
        if observation is None or observation["swe_m"] is None:
            raise RuntimeError(f"missing observed peak row for {site} WY{source['water_year']}")
        if not close(observation["swe_m"], float(source["observed_peak_swe_m"]), tolerance_m):
            raise RuntimeError(f"observed peak mismatch for {site} WY{source['water_year']}")
        initial = daily[start]["runtime_swe_before_m"]
        final = daily[peak]["runtime_swe_after_m"]
        modeled_peak_day = max(
            dates, key=lambda day: (daily[day]["runtime_swe_after_m"], -day.toordinal())
        )
        all_phase = sum(climate[day]["precip_m"] for day in dates)
        snowfall = sum(daily[day]["snowfall_m"] for day in dates)
        retained_rain = sum(daily[day]["rain_retained_m"] for day in dates)
        pack_loss = sum(daily[day]["pack_loss_m"] for day in dates)
        sublimation = sum(daily[day]["sublimation_m"] for day in dates)
        checks = (
            (initial, float(source["initial_swe_m"]), "initial SWE"),
            (final, float(source["final_swe_m"]), "final SWE"),
            (daily[modeled_peak_day]["runtime_swe_after_m"], float(source["modeled_peak_swe_m"]), "modeled peak"),
            (snowfall, float(source["snowfall_m"]), "snowfall"),
            (retained_rain, float(source["rain_retained_m"]), "retained rain"),
            (pack_loss, float(source["solid_pack_loss_m"]), "pack loss"),
            (sublimation, float(source["sublimation_m"]), "sublimation"),
        )
        for actual, expected, label in checks:
            if not close(actual, expected, tolerance_m):
                raise RuntimeError(
                    f"{label} mismatch for {site} WY{source['water_year']}: {actual} != {expected}"
                )
        observed_peak = float(source["observed_peak_swe_m"])
        current_ceiling = initial + all_phase
        effective_input = initial + snowfall + retained_rain
        closure_residual = final - (effective_input - pack_loss - sublimation)
        output.append(
            {
                "site": site,
                "display_site": DISPLAY[site],
                "water_year": int(source["water_year"]),
                "window_start": start.isoformat(),
                "observed_peak_date": peak.isoformat(),
                "observed_peak_swe_m": observed_peak,
                "initial_swe_m": initial,
                "fixture_all_phase_precipitation_m": all_phase,
                "current_input_mass_ceiling_m": current_ceiling,
                "current_input_mass_ceiling_ratio": current_ceiling / observed_peak,
                "storage_effective_input_m": effective_input,
                "storage_effective_input_ratio": effective_input / observed_peak,
                "modeled_pack_loss_m": pack_loss,
                "modeled_pack_loss_to_observed_peak": pack_loss / observed_peak,
                "modeled_sublimation_m": sublimation,
                "observed_date_modeled_storage_m": final,
                "observed_date_modeled_storage_ratio": final / observed_peak,
                "within_window_modeled_peak_date": modeled_peak_day.isoformat(),
                "within_window_modeled_peak_m": daily[modeled_peak_day]["runtime_swe_after_m"],
                "within_window_modeled_peak_ratio": daily[modeled_peak_day]["runtime_swe_after_m"] / observed_peak,
                "input_shortfall_m": max(observed_peak - current_ceiling, 0.0),
                "input_to_observed_date_storage_gap_m": current_ceiling - final,
                "guarded_gauge_precipitation_m": sum(gauge.get(day, 0.0) for day in dates),
                "guarded_gauge_day_count": sum(day in gauge for day in dates),
                "window_day_count": len(dates),
                "modeled_storage_closure_residual_m": closure_residual,
            }
        )
    return output


def cold_events(
    site: str,
    mass_rows: list[dict[str, Any]],
    climate: dict[dt.date, dict[str, float]],
    observations: dict[dt.date, dict[str, Any]],
    daily: dict[dt.date, dict[str, float]],
    threshold_m: float,
) -> tuple[list[dict[str, Any]], dict[str, int]]:
    output: list[dict[str, Any]] = []
    exclusions = {"missing_previous_wteq": 0, "missing_tmax": 0, "incomplete_interval": 0}
    one_day = dt.timedelta(days=1)
    for mass in mass_rows:
        start = dt.date.fromisoformat(mass["window_start"])
        peak = dt.date.fromisoformat(mass["observed_peak_date"])
        active: list[dt.date] = []
        for stamp in date_span(start, peak):
            current = observations.get(stamp)
            previous = observations.get(stamp - one_day)
            if current is None or previous is None or current["swe_m"] is None or previous["swe_m"] is None:
                exclusions["missing_previous_wteq"] += 1
                continue
            if current["tmax_c"] is None:
                exclusions["missing_tmax"] += 1
                continue
            if current["swe_m"] - previous["swe_m"] >= threshold_m and current["tmax_c"] <= 0.0:
                active.append(stamp)
        for sequence, event in enumerate(merge_cold_event_intervals(active, start, peak), start=1):
            days = date_span(event["start"], event["end"])
            if any(day not in climate or day not in daily for day in days):
                exclusions["incomplete_interval"] += 1
                continue
            observed_gain = sum(
                observations[day]["swe_m"] - observations[day - one_day]["swe_m"]
                for day in event["active_dates"]
            )
            all_phase = sum(climate[day]["precip_m"] for day in days)
            snowfall = sum(daily[day]["snowfall_m"] for day in days)
            retained_rain = sum(daily[day]["rain_retained_m"] for day in days)
            pack_loss = sum(daily[day]["pack_loss_m"] for day in days)
            storage_change = daily[days[-1]]["runtime_swe_after_m"] - daily[days[0]]["runtime_swe_before_m"]
            output.append(
                {
                    "site": site,
                    "display_site": DISPLAY[site],
                    "water_year": mass["water_year"],
                    "event_sequence": sequence,
                    "event_start": event["start"].isoformat(),
                    "event_end": event["end"].isoformat(),
                    "event_day_count": len(days),
                    "active_observed_gain_day_count": len(event["active_dates"]),
                    "observed_wteq_gain_m": observed_gain,
                    "fixture_all_phase_precipitation_m": all_phase,
                    "modeled_snowfall_m": snowfall,
                    "modeled_retained_rain_m": retained_rain,
                    "modeled_pack_loss_m": pack_loss,
                    "modeled_storage_change_m": storage_change,
                    "all_phase_to_observed_gain_ratio": ratio(all_phase, observed_gain),
                    "modeled_snowfall_to_observed_gain_ratio": ratio(snowfall, observed_gain),
                    "modeled_storage_change_to_observed_gain_ratio": ratio(storage_change, observed_gain),
                    "modeled_pack_loss_to_observed_gain_ratio": ratio(pack_loss, observed_gain),
                }
            )
    return output, exclusions


def dry_intervals(
    site: str,
    mass_rows: list[dict[str, Any]],
    climate: dict[dt.date, dict[str, float]],
    observations: dict[dt.date, dict[str, Any]],
    gauge: dict[dt.date, float],
    daily: dict[dt.date, dict[str, float]],
    dry_threshold_m: float,
    minimum_days: int,
) -> tuple[list[dict[str, Any]], dict[str, int]]:
    output: list[dict[str, Any]] = []
    exclusions = {"too_short": 0, "nonpositive_entry_pack": 0, "missing_boundary": 0}
    one_day = dt.timedelta(days=1)
    for mass in mass_rows:
        start = dt.date.fromisoformat(mass["window_start"])
        peak = dt.date.fromisoformat(mass["observed_peak_date"])
        candidates = []
        for stamp in date_span(start, peak):
            observed = observations.get(stamp)
            if (
                stamp in climate
                and stamp in daily
                and stamp in gauge
                and climate[stamp]["precip_m"] <= dry_threshold_m
                and gauge[stamp] <= dry_threshold_m
                and observed is not None
                and observed["swe_m"] is not None
                and observed["tmax_c"] is not None
            ):
                candidates.append(stamp)
        for sequence, group in enumerate(group_consecutive(candidates), start=1):
            if len(group) < minimum_days:
                exclusions["too_short"] += 1
                continue
            before = group[0] - one_day
            if before not in observations or observations[before]["swe_m"] is None:
                exclusions["missing_boundary"] += 1
                continue
            if observations[before]["swe_m"] <= 0.0 or daily[group[0]]["runtime_swe_before_m"] <= 0.0:
                exclusions["nonpositive_entry_pack"] += 1
                continue
            observed_loss = max(
                observations[before]["swe_m"] - observations[group[-1]]["swe_m"],
                0.0,
            )
            modeled_loss = sum(daily[day]["pack_loss_m"] for day in group)
            storage_change = daily[group[-1]]["runtime_swe_after_m"] - daily[group[0]]["runtime_swe_before_m"]
            cold = all(observations[day]["tmax_c"] <= 0.0 for day in group)
            output.append(
                {
                    "site": site,
                    "display_site": DISPLAY[site],
                    "water_year": mass["water_year"],
                    "interval_sequence": sequence,
                    "interval_start": group[0].isoformat(),
                    "interval_end": group[-1].isoformat(),
                    "interval_day_count": len(group),
                    "temperature_stratum": "cold" if cold else "warm_or_mixed",
                    "observed_wteq_loss_m": observed_loss,
                    "modeled_pack_loss_m": modeled_loss,
                    "modeled_minus_observed_loss_m": modeled_loss - observed_loss,
                    "modeled_storage_change_m": storage_change,
                    "fixture_all_phase_precipitation_m": sum(climate[day]["precip_m"] for day in group),
                    "guarded_gauge_precipitation_m": sum(gauge[day] for day in group),
                }
            )
    return output, exclusions


def aggregate_dry_annual(intervals: list[dict[str, Any]]) -> list[dict[str, Any]]:
    groups: dict[tuple[str, int, str], list[dict[str, Any]]] = defaultdict(list)
    for row in intervals:
        groups[(row["site"], row["water_year"], "all")].append(row)
        if row["temperature_stratum"] == "cold":
            groups[(row["site"], row["water_year"], "cold")].append(row)
    output = []
    for (site, water_year, stratum), rows in sorted(groups.items()):
        observed = sum(row["observed_wteq_loss_m"] for row in rows)
        modeled = sum(row["modeled_pack_loss_m"] for row in rows)
        output.append(
            {
                "site": site,
                "display_site": DISPLAY[site],
                "water_year": water_year,
                "temperature_stratum": stratum,
                "interval_count": len(rows),
                "interval_day_count": sum(row["interval_day_count"] for row in rows),
                "observed_wteq_loss_m": observed,
                "modeled_pack_loss_m": modeled,
                "modeled_minus_observed_loss_m": modeled - observed,
                "modeled_to_observed_loss_ratio": ratio(modeled, observed),
            }
        )
    return output


def summarize_sites(
    mass_rows: list[dict[str, Any]],
    events: list[dict[str, Any]],
    dry_annual: list[dict[str, Any]],
    constants: dict[str, Any],
) -> list[dict[str, Any]]:
    summaries = []
    for site in SITES:
        mass = [row for row in mass_rows if row["site"] == site]
        cold = [row for row in events if row["site"] == site]
        dry = [
            row
            for row in dry_annual
            if row["site"] == site and row["temperature_stratum"] == "all"
        ]
        ceiling_values = [row["current_input_mass_ceiling_ratio"] for row in mass]
        all_phase_values = [row["all_phase_to_observed_gain_ratio"] for row in cold]
        snowfall_values = [row["modeled_snowfall_to_observed_gain_ratio"] for row in cold]
        dry_differences = [row["modeled_minus_observed_loss_m"] for row in dry]
        mass_signal = (
            len(mass) >= constants["minimum_primary_years_per_site"]
            and median(ceiling_values) < constants["mass_ceiling_ratio_materiality"]
            and fraction(value < 1.0 for value in ceiling_values)
            >= constants["site_direction_fraction"]
        )
        all_phase_signal = (
            len(cold) >= constants["minimum_cold_events_per_site"]
            and median(all_phase_values) < constants["event_ratio_materiality"]
            and fraction(value < constants["event_ratio_materiality"] for value in all_phase_values)
            >= constants["event_direction_fraction"]
        )
        snowfall_signal = (
            len(cold) >= constants["minimum_cold_events_per_site"]
            and median(snowfall_values) < constants["event_ratio_materiality"]
            and fraction(value < constants["event_ratio_materiality"] for value in snowfall_values)
            >= constants["event_direction_fraction"]
        )
        dry_signal = (
            len(dry) >= constants["minimum_dry_annuals_per_site"]
            and median(dry_differences)
            >= constants["annual_dry_loss_difference_materiality_m"]
            and fraction(value > 0.0 for value in dry_differences)
            >= constants["site_direction_fraction"]
        )
        summaries.append(
            {
                "site": site,
                "display_site": DISPLAY[site],
                "primary_year_count": len(mass),
                "median_current_input_mass_ceiling_ratio": median(ceiling_values),
                "fraction_years_current_input_ceiling_below_observed_peak": fraction(value < 1.0 for value in ceiling_values),
                "median_storage_effective_input_ratio": median(row["storage_effective_input_ratio"] for row in mass),
                "median_observed_date_modeled_storage_ratio": median(row["observed_date_modeled_storage_ratio"] for row in mass),
                "median_within_window_modeled_peak_ratio": median(row["within_window_modeled_peak_ratio"] for row in mass),
                "median_modeled_pack_loss_to_observed_peak": median(row["modeled_pack_loss_to_observed_peak"] for row in mass),
                "mass_ceiling_site_signal": mass_signal,
                "cold_event_count": len(cold),
                "median_cold_event_all_phase_to_observed_gain_ratio": median(all_phase_values),
                "fraction_cold_events_all_phase_ratio_below_materiality": fraction(value < constants["event_ratio_materiality"] for value in all_phase_values),
                "median_cold_event_snowfall_to_observed_gain_ratio": median(snowfall_values),
                "fraction_cold_events_snowfall_ratio_below_materiality": fraction(value < constants["event_ratio_materiality"] for value in snowfall_values),
                "median_cold_event_storage_change_to_observed_gain_ratio": median(row["modeled_storage_change_to_observed_gain_ratio"] for row in cold),
                "median_cold_event_pack_loss_to_observed_gain_ratio": median(row["modeled_pack_loss_to_observed_gain_ratio"] for row in cold),
                "cold_event_all_phase_site_signal": all_phase_signal,
                "cold_event_snowfall_site_signal": snowfall_signal,
                "phase_or_solid_input_site_signal": (not all_phase_signal and snowfall_signal),
                "dry_annual_count": len(dry),
                "median_dry_annual_observed_loss_m": median(row["observed_wteq_loss_m"] for row in dry),
                "median_dry_annual_modeled_pack_loss_m": median(row["modeled_pack_loss_m"] for row in dry),
                "median_dry_annual_modeled_minus_observed_loss_m": median(dry_differences),
                "fraction_dry_annuals_modeled_loss_exceeds_observed": fraction(value > 0.0 for value in dry_differences),
                "dry_loss_site_signal": dry_signal,
            }
        )
    return summaries


def cohort_summary(site_summary: list[dict[str, Any]], systemic_count: int) -> dict[str, Any]:
    counts = {
        "mass_ceiling_site_signal_count": sum(row["mass_ceiling_site_signal"] for row in site_summary),
        "cold_event_all_phase_site_signal_count": sum(row["cold_event_all_phase_site_signal"] for row in site_summary),
        "cold_event_snowfall_site_signal_count": sum(row["cold_event_snowfall_site_signal"] for row in site_summary),
        "phase_or_solid_input_site_signal_count": sum(row["phase_or_solid_input_site_signal"] for row in site_summary),
        "dry_loss_site_signal_count": sum(row["dry_loss_site_signal"] for row in site_summary),
    }
    mass_systemic = counts["mass_ceiling_site_signal_count"] >= systemic_count
    all_phase_systemic = counts["cold_event_all_phase_site_signal_count"] >= systemic_count
    snowfall_systemic = counts["cold_event_snowfall_site_signal_count"] >= systemic_count
    loss_systemic = counts["dry_loss_site_signal_count"] >= systemic_count
    input_evidence = mass_systemic or all_phase_systemic or snowfall_systemic
    if input_evidence and loss_systemic:
        verdict = "MULTIFACTOR_INPUT_AND_LOSS_SIGNAL"
    elif input_evidence:
        verdict = "INPUT_PRIORITY_SIGNAL"
    elif loss_systemic:
        verdict = "LOSS_PRIORITY_SIGNAL"
    else:
        verdict = "UNRESOLVED_OR_COVERAGE_LIMITED"
    return {
        **counts,
        "systemic_site_count_required": systemic_count,
        "mass_ceiling_systemic": mass_systemic,
        "cold_event_all_phase_systemic": all_phase_systemic,
        "cold_event_snowfall_systemic": snowfall_systemic,
        "phase_or_solid_input_systemic": counts["phase_or_solid_input_site_signal_count"] >= systemic_count,
        "dry_loss_systemic": loss_systemic,
        "input_evidence": input_evidence,
        "loss_evidence": loss_systemic,
        "verdict": verdict,
    }


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not rows:
        raise RuntimeError(f"refusing to write empty CSV: {path}")
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def execute(output_root: Path) -> dict[str, Any]:
    freeze_bytes = FREEZE_PATH.read_bytes()
    freeze = json.loads(freeze_bytes)
    constants = freeze["constants"]
    tolerance_m = float(constants["mass_tolerance_m"])
    expected = freeze["source_identity_expectations"]
    if sha256(PREDECESSOR_MANIFEST) != expected["predecessor_evidence_manifest_sha256"]:
        raise RuntimeError("predecessor tracked evidence manifest hash differs from freeze")
    if sha256(TRACE_RECEIPT) != expected["predecessor_trace_receipt_sha256"]:
        raise RuntimeError("predecessor retained execution receipt hash differs from freeze")
    if sha256(ANNUAL_RESULTS) != expected["predecessor_annual_results_sha256"]:
        raise RuntimeError("predecessor annual result hash differs from freeze")
    if sha256(PRCPSA_PATH) != expected["snowbird_prcpsa_sha256_at_intake_commit"]:
        raise RuntimeError("Snowbird PRCPSA identity differs from frozen intake hash")
    manifest = json.loads(PREDECESSOR_MANIFEST.read_text(encoding="utf-8"))
    receipt = json.loads(TRACE_RECEIPT.read_text(encoding="utf-8"))
    validate_predecessor_authority(freeze, manifest, receipt)
    annual_payload = json.loads(ANNUAL_RESULTS.read_text(encoding="utf-8"))
    annual_source = annual_payload["annual"]

    all_mass: list[dict[str, Any]] = []
    all_events: list[dict[str, Any]] = []
    all_dry: list[dict[str, Any]] = []
    source_identity: dict[str, Any] = {
        "analysis_freeze": {"path": str(FREEZE_PATH.relative_to(REPO)), "sha256": sha256_bytes(freeze_bytes)},
        "predecessor_evidence_manifest": {
            "path": str(PREDECESSOR_MANIFEST.relative_to(REPO)),
            "sha256": sha256(PREDECESSOR_MANIFEST),
        },
        "annual_results": {"path": str(ANNUAL_RESULTS.relative_to(REPO)), "sha256": sha256(ANNUAL_RESULTS)},
        "trace_receipt": {"path": str(TRACE_RECEIPT.relative_to(REPO)), "sha256": sha256(TRACE_RECEIPT)},
        "snowbird_prcpsa": {"path": str(PRCPSA_PATH.relative_to(REPO)), "sha256": sha256(PRCPSA_PATH)},
        "sites": {},
    }
    exclusions: dict[str, Any] = {}
    trace_validation: dict[str, Any] = {}

    for site in SITES:
        cli = climate_path(site)
        observation_path = OBS_ROOT / f"{site}.csv"
        expected_observation_hash = receipt["sites"][site]["observation"]["sha256"]
        if sha256(observation_path) != expected_observation_hash:
            raise RuntimeError(f"observation hash mismatch for {site}")
        copied_cli = next(
            row
            for row in receipt["sites"][site]["source_fixture_manifest"]["files"]
            if row["path"].endswith(".cli")
        )
        if sha256(cli) != copied_cli["sha256"]:
            raise RuntimeError(f"climate hash mismatch for {site}")
        climate = parse_cli(cli)
        observations = parse_observations(observation_path)
        gauge, gauge_counts = guarded_precipitation_increments(
            observations, float(constants["gauge_negative_tolerance_m"])
        )
        daily, trace_info = reduced_trace(site, climate, receipt, tolerance_m)
        mass = annual_mass_rows(
            site, annual_source, climate, observations, gauge, daily, tolerance_m
        )
        events, event_exclusions = cold_events(
            site,
            mass,
            climate,
            observations,
            daily,
            float(constants["observed_accumulation_event_threshold_m"]),
        )
        dry, dry_exclusions = dry_intervals(
            site,
            mass,
            climate,
            observations,
            gauge,
            daily,
            float(constants["dry_precipitation_threshold_m_per_day"]),
            int(constants["minimum_dry_interval_days"]),
        )
        all_mass.extend(mass)
        all_events.extend(events)
        all_dry.extend(dry)
        trace_validation[site] = trace_info
        exclusions[site] = {
            "guarded_precipitation": gauge_counts,
            "cold_events": event_exclusions,
            "dry_intervals": dry_exclusions,
        }
        source_identity["sites"][site] = {
            "climate": {"path": str(cli.relative_to(REPO)), "sha256": sha256(cli)},
            "observation": {"path": str(observation_path.relative_to(REPO)), "sha256": sha256(observation_path)},
            "trace": trace_info,
        }

    dry_annual = aggregate_dry_annual(all_dry)
    site_summary = summarize_sites(all_mass, all_events, dry_annual, constants)
    cohort = cohort_summary(site_summary, int(constants["systemic_site_count"]))
    result = {
        "schema_version": 1,
        "status": "ANALYSIS_COMPLETE",
        "evidence_mode": "Ran: exact retained trace plus checked-in climate and observation analysis",
        "observation_role": freeze["observation_role"],
        "source_identity": source_identity,
        "trace_validation": trace_validation,
        "counts": {
            "annual_mass_rows": len(all_mass),
            "cold_event_rows": len(all_events),
            "dry_interval_rows": len(all_dry),
            "dry_annual_rows": len(dry_annual),
        },
        "exclusions": exclusions,
        "site_summary": site_summary,
        "cohort_summary": cohort,
        "context_only": freeze["context_only"],
        "claim_limits": freeze["claim_limits"],
    }

    tables = output_root / "tables"
    write_csv(tables / "annual-mass.csv", all_mass)
    write_csv(tables / "cold-events.csv", all_events)
    write_csv(tables / "dry-intervals.csv", all_dry)
    write_csv(tables / "dry-annual.csv", dry_annual)
    write_csv(tables / "site-summary.csv", site_summary)
    write_json(output_root / "results.json", result)
    output_hashes = {
        str(path.relative_to(REPO)): {
            "sha256": sha256(path),
            "size_bytes": path.stat().st_size,
        }
        for path in sorted(output_root.rglob("*"))
        if path.is_file()
    }
    execution_receipt = {
        "schema_version": 1,
        "status": "PASS",
        "command": [".venv/bin/python", str(Path(__file__).relative_to(REPO))],
        "working_directory": str(REPO),
        "scaffold_commit": "b92a971b",
        "freeze_sha256": sha256_bytes(freeze_bytes),
        "tool_sha256": sha256(Path(__file__)),
        "source_identity": source_identity,
        "output_files_before_receipt": output_hashes,
        "verdict": cohort["verdict"],
    }
    write_json(output_root / "execution-receipt.json", execution_receipt)
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-root", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()
    result = execute(args.output_root.resolve())
    print(json.dumps({"counts": result["counts"], "cohort_summary": result["cohort_summary"]}, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
