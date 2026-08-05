#!/usr/bin/env python3
"""Execute the frozen 21L corrected-state snow loss attribution."""

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
TRACE_ROOT = REPO / "target/snow_wet_compaction_operand_closure"
RECEIPT_PATH = TRACE_ROOT / "execution-receipt.json"
MATERIALITY_PATH = TRACE_ROOT / "results/materiality.json"
OBS_ROOT = REPO / "tests/fixtures/snotel_observed/observations/sites"
PREDECESSOR_ROOT = REPO / "target/snow_accumulation_target_feasibility_input_loss_discrimination_v2"
PREDECESSOR_PACKAGE = REPO / "docs/work-packages/20260804-snow-accumulation-target-feasibility-input-loss-discrimination-001/artifacts"
DEFAULT_OUTPUT = REPO / "target/snow_warm_mixed_prepeak_loss_energy_attribution_v2"

CANONICAL = (
    "snotel_mica_creek_st_joe_id",
    "snotel_niwot_co",
    "snotel_paradise_wa",
    "snotel_snowbird_ut",
)
DEVELOPMENT = "snotel_snowbird_ut__precip_x1p2155576"
LANES = (*CANONICAL, DEVELOPMENT)
DISPLAY = {
    "snotel_mica_creek_st_joe_id": "Mica Creek",
    "snotel_niwot_co": "Niwot",
    "snotel_paradise_wa": "Paradise",
    "snotel_snowbird_ut": "Snowbird",
    DEVELOPMENT: "Snowbird scaled",
}
COMPONENTS = ("amelt", "bmelt", "cmelt", "dmelt")
CLASSES = ("cold_day", "mixed_day", "warm_day")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def sha256_bytes(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


def finite(value: Any, label: str) -> float:
    parsed = float(value)
    if not math.isfinite(parsed):
        raise RuntimeError(f"non-finite {label}")
    return parsed


def optional_float(value: str | None) -> float | None:
    if value in (None, ""):
        return None
    parsed = float(value)
    return parsed if math.isfinite(parsed) else None


def median(values: Iterable[float]) -> float | None:
    rows = list(values)
    return statistics.median(rows) if rows else None


def fraction(values: Iterable[bool]) -> float | None:
    rows = list(values)
    return sum(rows) / len(rows) if rows else None


def pearson(left: list[float], right: list[float]) -> float | None:
    if len(left) != len(right) or len(left) < 3:
        return None
    left_mean = statistics.fmean(left)
    right_mean = statistics.fmean(right)
    numerator = sum((x - left_mean) * (y - right_mean) for x, y in zip(left, right))
    left_scale = sum((x - left_mean) ** 2 for x in left)
    right_scale = sum((y - right_mean) ** 2 for y in right)
    denominator = math.sqrt(left_scale * right_scale)
    return numerator / denominator if denominator > 0.0 else None


def ranks(values: list[float]) -> list[float]:
    ordered = sorted(enumerate(values), key=lambda item: item[1])
    output = [0.0] * len(values)
    start = 0
    while start < len(ordered):
        end = start + 1
        while end < len(ordered) and ordered[end][1] == ordered[start][1]:
            end += 1
        rank = (start + 1 + end) / 2.0
        for index in range(start, end):
            output[ordered[index][0]] = rank
        start = end
    return output


def spearman(left: list[float], right: list[float]) -> float | None:
    return pearson(ranks(left), ranks(right))


def date_span(start: dt.date, end: dt.date) -> list[dt.date]:
    return [start + dt.timedelta(days=index) for index in range((end - start).days + 1)]


def group_consecutive(dates: Iterable[dt.date]) -> list[list[dt.date]]:
    groups: list[list[dt.date]] = []
    current: list[dt.date] = []
    for stamp in sorted(dates):
        if current and (stamp - current[-1]).days != 1:
            groups.append(current)
            current = []
        current.append(stamp)
    if current:
        groups.append(current)
    return groups


def parse_cli(path: Path) -> dict[dt.date, dict[str, float]]:
    rows: dict[dt.date, dict[str, float]] = {}
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            fields = line.split()
            if len(fields) != 13:
                continue
            try:
                stamp = dt.date(int(fields[2]), int(fields[1]), int(fields[0]))
                numbers = list(map(float, fields[3:]))
            except (ValueError, OverflowError):
                continue
            if stamp in rows:
                raise RuntimeError(f"duplicate climate date {stamp}: {path}")
            rows[stamp] = {
                "precip_m": numbers[0] / 1000.0,
                "tmax_c": numbers[4],
                "tmin_c": numbers[5],
                "solar_langley": numbers[6],
                "wind_m_s": numbers[7],
                "dewpoint_c": numbers[8],
            }
    dates = list(rows)
    if not dates or dates != sorted(dates):
        raise RuntimeError(f"invalid climate chronology: {path}")
    return rows


def parse_observations(path: Path) -> dict[dt.date, dict[str, Any]]:
    rows: dict[dt.date, dict[str, Any]] = {}
    with path.open(newline="", encoding="utf-8") as handle:
        for raw in csv.DictReader(handle):
            stamp = dt.date.fromisoformat(raw["date"])
            if stamp in rows:
                raise RuntimeError(f"duplicate observation date {stamp}: {path}")
            swe_mm = optional_float(raw["observed_swe_mm"])
            precip_mm = optional_float(raw["observed_precip_mm"])
            rows[stamp] = {
                "water_year": int(raw["water_year"]),
                "swe_m": None if swe_mm is None else swe_mm / 1000.0,
                "precip_cumulative_m": None if precip_mm is None else precip_mm / 1000.0,
                "tmax_c": optional_float(raw["observed_tmax_c"]),
                "tmin_c": optional_float(raw["observed_tmin_c"]),
            }
    return rows


def guarded_precipitation(observations: dict[dt.date, dict[str, Any]], tolerance: float) -> dict[dt.date, float]:
    output: dict[dt.date, float] = {}
    one_day = dt.timedelta(days=1)
    for stamp, current in observations.items():
        previous = observations.get(stamp - one_day)
        if previous is None or previous["water_year"] != current["water_year"]:
            continue
        left = previous["precip_cumulative_m"]
        right = current["precip_cumulative_m"]
        if left is None or right is None:
            continue
        delta = right - left
        if delta < -tolerance:
            continue
        output[stamp] = max(delta, 0.0)
    return output


def climate_path(lane: str) -> Path:
    matches = sorted((TRACE_ROOT / "fixtures" / lane).glob("*.cli"))
    if len(matches) != 1:
        raise RuntimeError(f"expected one staged climate for {lane}")
    return matches[0]


def compare_precipitation_only_climates(canonical: Path, development: Path) -> dict[str, int]:
    left = canonical.read_text(encoding="utf-8").splitlines()
    right = development.read_text(encoding="utf-8").splitlines()
    if len(left) != len(right):
        raise RuntimeError("Snowbird climate line counts differ")
    daily_rows = 0
    changed_precipitation_rows = 0
    for line_number, (source, derived) in enumerate(zip(left, right), start=1):
        source_fields = source.split()
        derived_fields = derived.split()
        if len(source_fields) == 13 and len(derived_fields) == 13:
            try:
                dt.date(int(source_fields[2]), int(source_fields[1]), int(source_fields[0]))
                dt.date(int(derived_fields[2]), int(derived_fields[1]), int(derived_fields[0]))
            except (ValueError, OverflowError):
                pass
            else:
                daily_rows += 1
                if source_fields[:3] != derived_fields[:3] or source_fields[4:] != derived_fields[4:]:
                    raise RuntimeError(f"Snowbird non-precipitation climate token differs on line {line_number}")
                if source_fields[3] != derived_fields[3]:
                    changed_precipitation_rows += 1
                continue
        if source != derived:
            raise RuntimeError(f"Snowbird non-daily climate line differs on line {line_number}")
    return {
        "daily_row_count": daily_rows,
        "changed_precipitation_row_count": changed_precipitation_rows,
        "non_precipitation_difference_count": 0,
    }


def trace_path(lane: str) -> Path:
    return TRACE_ROOT / "runs" / lane / f"{lane}-wet-compaction.snow.jsonl"


def reconstruct_stage3_closure(row: dict[str, Any]) -> tuple[float, float]:
    mass = (
        finite(row["stage3_incoming_liquid_m"], "Stage-3 incoming liquid")
        - finite(row["stage3_routed_liquid_m"], "Stage-3 routed liquid")
        - finite(row["stage3_retained_liquid_delta_m"], "Stage-3 retained delta")
        - finite(row["stage3_refrozen_liquid_m"], "Stage-3 refrozen liquid")
    )
    energy = (
        finite(row["stage3_surface_energy_j_m2"], "Stage-3 surface energy")
        + finite(row["stage3_conduction_energy_j_m2"], "Stage-3 conduction energy")
        + finite(row["stage3_latent_refreeze_energy_j_m2"], "Stage-3 latent refreeze energy")
        + finite(row["stage3_cold_content_export_j_m2"], "Stage-3 cold-content export")
        - (
            finite(row["stage3_cold_content_before_j_m2"], "Stage-3 cold content before")
            - finite(row["stage3_cold_content_after_j_m2"], "Stage-3 cold content after")
        )
    )
    return mass, energy


def day_class(hours: list[dict[str, Any]]) -> tuple[str | None, float | None, float | None, int]:
    temperatures = [
        finite(hour["air_temperature_c"], "hourly temperature")
        for hour in hours
        if finite(hour["pack_depth_before_m"], "pack depth before") > 0.0
        or finite(hour["snowfall_swe_m"], "hourly snowfall") > 0.0
    ]
    if not temperatures:
        return None, None, None, 0
    minimum = min(temperatures)
    maximum = max(temperatures)
    if maximum <= 0.0:
        label = "cold_day"
    elif minimum <= 0.0:
        label = "mixed_day"
    else:
        label = "warm_day"
    return label, minimum, maximum, len(temperatures)


def reduce_trace(
    lane: str,
    climate: dict[dt.date, dict[str, float]],
    expected_hash: str,
    mass_tolerance: float,
    energy_tolerance: float,
    scoped_water_year: dict[dt.date, int],
) -> tuple[dict[dt.date, dict[str, Any]], list[dict[str, Any]], dict[str, Any]]:
    path = trace_path(lane)
    dates = list(climate)
    daily: dict[dt.date, dict[str, Any]] = {}
    hourly_output: list[dict[str, Any]] = []
    digest = hashlib.sha256()
    max_mass = 0.0
    max_accumulation = 0.0
    max_stage3_mass = 0.0
    max_stage3_energy = 0.0
    max_independent_stage3_mass = 0.0
    max_independent_stage3_energy = 0.0
    max_stage3_mass_residual_agreement = 0.0
    max_stage3_energy_residual_agreement = 0.0
    max_coe = 0.0
    with path.open("rb") as handle:
        for index, raw in enumerate(handle):
            digest.update(raw)
            if index >= len(dates):
                raise RuntimeError(f"trace longer than climate for {lane}")
            row = json.loads(raw)
            if row.get("schema") != "openwepp-r7h-direct-production-snow-trace-v4":
                raise RuntimeError(f"unexpected trace schema for {lane}")
            hours = row["accumulation_melt_hourly"]
            if len(hours) != 24:
                raise RuntimeError(f"expected 24 hourly rows for {lane} day {index}")
            thermal, tmin, tmax, eligible_hours = day_class(hours)
            snowfall = sum(finite(hour["snowfall_swe_m"], "snowfall") for hour in hours)
            components = {name: 0.0 for name in COMPONENTS}
            positive_components = {name: 0.0 for name in COMPONENTS}
            applied_positive = 0.0
            cap_adjustment = 0.0
            radiation = 0.0
            cloud = 0.0
            for hour in hours:
                values = {
                    "amelt": finite(hour["coe_melt_amelt_m"], "amelt"),
                    "bmelt": finite(hour["coe_melt_bmelt_m"], "bmelt"),
                    "cmelt": finite(hour["coe_melt_cmelt_m"], "cmelt"),
                    "dmelt": finite(hour["coe_melt_dmelt_m"], "dmelt"),
                }
                cap = finite(hour["coe_melt_cap_adjustment_m"], "cap adjustment")
                applied = finite(hour["coe_melt_applied_m"], "applied melt")
                uncapped = finite(hour["coe_melt_uncapped_m"], "uncapped melt")
                component_sum = sum(values.values())
                max_coe = max(max_coe, abs(component_sum - uncapped), abs(component_sum + cap - applied))
                for name, value in values.items():
                    components[name] += value
                    positive_components[name] += max(value, 0.0)
                cap_adjustment += cap
                applied_positive += max(applied, 0.0)
                radiation += finite(hour["radiation_mj_m2"], "radiation")
                cloud += finite(hour["cloud_fraction"], "cloud")
            accumulation = finite(row["accumulation_m"], "accumulation")
            rain_retained = finite(row["rain_retained_m"], "rain retained")
            pack_loss = finite(row["snowpack_swe_loss_m"], "pack loss")
            sublimation = finite(row["sublimation_m"], "sublimation")
            swe_before = finite(row["runtime_swe_before_m"], "SWE before")
            swe_after = finite(row["runtime_swe_after_m"], "SWE after")
            mass_residual = swe_after - swe_before - (snowfall + rain_retained - pack_loss - sublimation)
            max_mass = max(max_mass, abs(mass_residual))
            max_accumulation = max(max_accumulation, abs(accumulation - snowfall))
            stage3_mass_residual = finite(row["stage3_liquid_closure_residual_m"], "Stage-3 mass closure")
            stage3_energy_residual = finite(row["stage3_energy_closure_residual_j_m2"], "Stage-3 energy closure")
            stage3_incoming = finite(row["stage3_incoming_liquid_m"], "Stage-3 incoming liquid")
            stage3_routed = finite(row["stage3_routed_liquid_m"], "Stage-3 routed liquid")
            stage3_retained_delta = finite(row["stage3_retained_liquid_delta_m"], "Stage-3 retained delta")
            stage3_refrozen = finite(row["stage3_refrozen_liquid_m"], "Stage-3 refrozen liquid")
            cold_before = finite(row["stage3_cold_content_before_j_m2"], "Stage-3 cold content before")
            cold_after = finite(row["stage3_cold_content_after_j_m2"], "Stage-3 cold content after")
            stage3_surface = finite(row["stage3_surface_energy_j_m2"], "Stage-3 surface energy")
            stage3_conduction = finite(row["stage3_conduction_energy_j_m2"], "Stage-3 conduction energy")
            stage3_latent_refreeze = finite(row["stage3_latent_refreeze_energy_j_m2"], "Stage-3 latent refreeze energy")
            stage3_cold_export = finite(row["stage3_cold_content_export_j_m2"], "Stage-3 cold-content export")
            independent_stage3_mass, independent_stage3_energy = reconstruct_stage3_closure(row)
            max_stage3_mass = max(max_stage3_mass, abs(stage3_mass_residual))
            max_stage3_energy = max(max_stage3_energy, abs(stage3_energy_residual))
            max_independent_stage3_mass = max(max_independent_stage3_mass, abs(independent_stage3_mass))
            max_independent_stage3_energy = max(max_independent_stage3_energy, abs(independent_stage3_energy))
            max_stage3_mass_residual_agreement = max(max_stage3_mass_residual_agreement, abs(independent_stage3_mass - stage3_mass_residual))
            max_stage3_energy_residual_agreement = max(max_stage3_energy_residual_agreement, abs(independent_stage3_energy - stage3_energy_residual))
            stamp = dates[index]
            if stamp in scoped_water_year:
                for hour in hours:
                    eligible = (
                        finite(hour["pack_depth_before_m"], "pack depth before") > 0.0
                        or finite(hour["snowfall_swe_m"], "hourly snowfall") > 0.0
                    )
                    hourly_output.append({
                        "lane": lane,
                        "role": "CANONICAL" if lane in CANONICAL else "DEVELOPMENT_ONLY",
                        "water_year": scoped_water_year[stamp],
                        "date": stamp.isoformat(),
                        "hour": int(hour["hour"]),
                        "eligible_hour": eligible,
                        "daily_thermal_class": thermal or "unclassified",
                        "air_temperature_c": finite(hour["air_temperature_c"], "hourly temperature"),
                        "active_precipitation_m": finite(hour["active_precipitation_m"], "active precipitation"),
                        "snowfall_swe_m": finite(hour["snowfall_swe_m"], "hourly snowfall"),
                        "rain_m": finite(hour["rain_m"], "hourly rain"),
                        "radiation_mj_m2": finite(hour["radiation_mj_m2"], "hourly radiation"),
                        "cloud_fraction": finite(hour["cloud_fraction"], "hourly cloud"),
                        "coe_amelt_m": finite(hour["coe_melt_amelt_m"], "hourly amelt"),
                        "coe_bmelt_m": finite(hour["coe_melt_bmelt_m"], "hourly bmelt"),
                        "coe_cmelt_m": finite(hour["coe_melt_cmelt_m"], "hourly cmelt"),
                        "coe_dmelt_m": finite(hour["coe_melt_dmelt_m"], "hourly dmelt"),
                        "coe_applied_m": finite(hour["coe_melt_applied_m"], "hourly applied melt"),
                        "coe_uncapped_m": finite(hour["coe_melt_uncapped_m"], "hourly uncapped melt"),
                        "coe_cap_adjustment_m": finite(hour["coe_melt_cap_adjustment_m"], "hourly cap adjustment"),
                        "pack_depth_before_m": finite(hour["pack_depth_before_m"], "hourly pack depth before"),
                        "pack_depth_after_m": finite(hour["pack_depth_after_m"], "hourly pack depth after"),
                        "pack_density_before_kg_m3": finite(hour["pack_density_before_kg_m3"], "hourly pack density before"),
                        "pack_density_after_kg_m3": finite(hour["pack_density_after_kg_m3"], "hourly pack density after"),
                        "liquid_water_retained_before_m": finite(hour["liquid_water_retained_before_m"], "hourly liquid before"),
                        "liquid_water_retained_after_m": finite(hour["liquid_water_retained_after_m"], "hourly liquid after"),
                    })
            daily[stamp] = {
                "thermal_class": thermal,
                "eligible_hour_count": eligible_hours,
                "hourly_tmin_c": tmin,
                "hourly_tmax_c": tmax,
                "fixture_precip_m": climate[stamp]["precip_m"],
                "snowfall_m": snowfall,
                "rain_retained_m": rain_retained,
                "rain_released_m": finite(row["rain_released_m"], "rain released"),
                "pack_loss_m": pack_loss,
                "sublimation_m": sublimation,
                "raw_melt_m": finite(row["raw_melt_m"], "raw melt"),
                "routed_melt_m": finite(row["routed_melt_m"], "routed melt"),
                "runtime_swe_before_m": swe_before,
                "runtime_swe_after_m": swe_after,
                "runtime_depth_after_m": finite(row["runtime_depth_after_m"], "depth after"),
                "retained_liquid_after_m": finite(row["liquid_water_retained_after_m"], "retained liquid"),
                "coe_applied_positive_m": applied_positive,
                "coe_cap_adjustment_m": cap_adjustment,
                "radiation_mj_m2": radiation,
                "cloud_fraction_mean": cloud / 24.0,
                "wind_m_s": finite(row["wind_m_s"], "daily wind"),
                "dewpoint_c": finite(row["dewpoint_c"], "daily dewpoint"),
                "canopy_cover_fraction": finite(row["canopy_cover_fraction"], "canopy cover"),
                "runtime_depth_before_m": finite(row["runtime_depth_before_m"], "depth before"),
                "runtime_density_before_kg_m3": finite(row["runtime_density_before_kg_m3"], "density before"),
                "runtime_density_after_kg_m3": finite(row["runtime_density_after_kg_m3"], "density after"),
                "retained_liquid_before_m": finite(row["liquid_water_retained_before_m"], "retained liquid before"),
                "stage3_surface_energy_j_m2": stage3_surface,
                "stage3_incoming_liquid_m": stage3_incoming,
                "stage3_cold_content_before_j_m2": cold_before,
                "stage3_cold_content_after_j_m2": cold_after,
                "stage3_latent_refreeze_energy_j_m2": stage3_latent_refreeze,
                "stage3_cold_content_export_j_m2": stage3_cold_export,
                "stage3_shortwave_energy_j_m2": finite(row["stage3_shortwave_energy_j_m2"], "shortwave energy"),
                "stage3_longwave_energy_j_m2": finite(row["stage3_longwave_energy_j_m2"], "longwave energy"),
                "stage3_latent_energy_j_m2": finite(row["stage3_latent_energy_j_m2"], "latent energy"),
                "stage3_conduction_energy_j_m2": stage3_conduction,
                "stage3_cold_content_change_j_m2": cold_after - cold_before,
                "stage3_refrozen_liquid_m": stage3_refrozen,
                "stage3_retained_liquid_delta_m": stage3_retained_delta,
                "stage3_routed_liquid_m": stage3_routed,
                **{f"coe_{name}_m": components[name] for name in COMPONENTS},
                **{f"coe_{name}_positive_m": positive_components[name] for name in COMPONENTS},
            }
    actual_hash = digest.hexdigest()
    if actual_hash != expected_hash:
        raise RuntimeError(
            f"trace identity mismatch for {lane}: {actual_hash} != {expected_hash}"
        )
    if len(daily) != len(dates):
        raise RuntimeError(f"trace/climate length mismatch for {lane}")
    if max_mass > mass_tolerance or max_accumulation > mass_tolerance:
        raise RuntimeError(f"snow mass closure exceeds tolerance for {lane}")
    if (
        max_stage3_mass > mass_tolerance
        or max_stage3_energy > energy_tolerance
        or max_independent_stage3_mass > mass_tolerance
        or max_independent_stage3_energy > energy_tolerance
        or max_stage3_mass_residual_agreement > mass_tolerance
        or max_stage3_energy_residual_agreement > energy_tolerance
    ):
        raise RuntimeError(f"Stage-3 closure exceeds tolerance for {lane}")
    if max_coe > mass_tolerance:
        raise RuntimeError(f"CoE reconstruction exceeds tolerance for {lane}")
    return daily, hourly_output, {
        "path": str(path.relative_to(REPO)),
        "sha256": actual_hash,
        "size_bytes": path.stat().st_size,
        "row_count": len(daily),
        "maximum_abs_snow_mass_closure_m": max_mass,
        "maximum_abs_accumulation_closure_m": max_accumulation,
        "maximum_abs_stage3_mass_closure_m": max_stage3_mass,
        "maximum_abs_stage3_energy_closure_j_m2": max_stage3_energy,
        "maximum_abs_independent_stage3_mass_closure_m": max_independent_stage3_mass,
        "maximum_abs_independent_stage3_energy_closure_j_m2": max_independent_stage3_energy,
        "maximum_abs_stage3_mass_residual_agreement_m": max_stage3_mass_residual_agreement,
        "maximum_abs_stage3_energy_residual_agreement_j_m2": max_stage3_energy_residual_agreement,
        "maximum_abs_coe_reconstruction_m": max_coe,
    }


def observed_windows(observations: dict[dt.date, dict[str, Any]]) -> list[dict[str, Any]]:
    grouped: dict[int, list[tuple[dt.date, float]]] = defaultdict(list)
    for stamp, row in observations.items():
        if row["swe_m"] is not None and row["swe_m"] > 0.0:
            grouped[row["water_year"]].append((stamp, row["swe_m"]))
    output = []
    for water_year, rows in sorted(grouped.items()):
        if water_year == 2025:
            continue
        peak_value = max(value for _, value in rows)
        peak_date = min(stamp for stamp, value in rows if value == peak_value)
        output.append({
            "water_year": water_year,
            "window_start": dt.date(water_year - 1, 10, 1),
            "observed_peak_date": peak_date,
            "observed_peak_swe_m": peak_value,
        })
    return output


def scoped_daily_rows(
    lane: str,
    windows: list[dict[str, Any]],
    daily: dict[dt.date, dict[str, Any]],
    material_loss: float,
) -> list[dict[str, Any]]:
    output = []
    for window in windows:
        for stamp in date_span(window["window_start"], window["observed_peak_date"]):
            if stamp not in daily:
                continue
            source = daily[stamp]
            output.append({
                "lane": lane,
                "display_lane": DISPLAY[lane],
                "role": "CANONICAL" if lane in CANONICAL else "DEVELOPMENT_ONLY",
                "water_year": window["water_year"],
                "date": stamp.isoformat(),
                "month": stamp.month,
                "thermal_class": source["thermal_class"] or "unclassified",
                "eligible_hour_count": source["eligible_hour_count"],
                "hourly_tmin_c": source["hourly_tmin_c"],
                "hourly_tmax_c": source["hourly_tmax_c"],
                "material_pack_loss": source["pack_loss_m"] >= material_loss,
                **{key: value for key, value in source.items() if key not in {
                    "thermal_class", "eligible_hour_count", "hourly_tmin_c", "hourly_tmax_c"
                }},
            })
    return output


def annual_rows(lane: str, windows: list[dict[str, Any]], daily: dict[dt.date, dict[str, Any]], material_loss: float) -> list[dict[str, Any]]:
    output = []
    for window in windows:
        days = date_span(window["window_start"], window["observed_peak_date"])
        if any(day not in daily for day in days):
            continue
        loss_by_class = {name: sum(daily[day]["pack_loss_m"] for day in days if daily[day]["thermal_class"] == name) for name in CLASSES}
        total_loss = sum(daily[day]["pack_loss_m"] for day in days)
        unclassified_loss = total_loss - sum(loss_by_class.values())
        if abs(unclassified_loss) > 1e-12:
            raise RuntimeError(
                f"unclassified pack loss exceeds tolerance for {lane} "
                f"water year {window['water_year']}"
            )
        peak_day = max(days, key=lambda day: (daily[day]["runtime_swe_after_m"], -day.toordinal()))
        row: dict[str, Any] = {
            "lane": lane,
            "display_lane": DISPLAY[lane],
            "role": "CANONICAL" if lane in CANONICAL else "DEVELOPMENT_ONLY",
            "water_year": window["water_year"],
            "window_start": window["window_start"].isoformat(),
            "observed_peak_date": window["observed_peak_date"].isoformat(),
            "observed_peak_swe_m": window["observed_peak_swe_m"],
            "modeled_swe_on_observed_peak_m": daily[window["observed_peak_date"]]["runtime_swe_after_m"],
            "modeled_peak_date": peak_day.isoformat(),
            "modeled_peak_swe_m": daily[peak_day]["runtime_swe_after_m"],
            "fixture_precipitation_m": sum(daily[day]["fixture_precip_m"] for day in days),
            "forcing_radiation_mj_m2": sum(daily[day]["radiation_mj_m2"] for day in days),
            "forcing_mean_cloud_fraction": statistics.fmean(daily[day]["cloud_fraction_mean"] for day in days),
            "modeled_snowfall_m": sum(daily[day]["snowfall_m"] for day in days),
            "modeled_rain_retained_m": sum(daily[day]["rain_retained_m"] for day in days),
            "modeled_pack_loss_m": total_loss,
            "unclassified_pack_loss_m": unclassified_loss,
            "ineligible_day_count": sum(daily[day]["thermal_class"] is None for day in days),
            "material_pack_loss_day_count": sum(daily[day]["pack_loss_m"] >= material_loss for day in days),
            "modeled_sublimation_m": sum(daily[day]["sublimation_m"] for day in days),
            "coe_applied_positive_m": sum(daily[day]["coe_applied_positive_m"] for day in days),
            "coe_cap_adjustment_m": sum(daily[day]["coe_cap_adjustment_m"] for day in days),
            "stage3_routed_liquid_m": sum(daily[day]["stage3_routed_liquid_m"] for day in days),
            "stage3_refrozen_liquid_m": sum(daily[day]["stage3_refrozen_liquid_m"] for day in days),
            "stage3_retained_liquid_delta_m": sum(daily[day]["stage3_retained_liquid_delta_m"] for day in days),
            "cold_pack_loss_m": loss_by_class["cold_day"],
            "mixed_pack_loss_m": loss_by_class["mixed_day"],
            "warm_pack_loss_m": loss_by_class["warm_day"],
            "warm_mixed_pack_loss_m": loss_by_class["mixed_day"] + loss_by_class["warm_day"],
            "warm_mixed_pack_loss_fraction": (loss_by_class["mixed_day"] + loss_by_class["warm_day"]) / total_loss if total_loss > 1e-12 else None,
        }
        for thermal in CLASSES:
            selected = [day for day in days if daily[day]["thermal_class"] == thermal]
            row[f"{thermal}_day_count"] = len(selected)
            row[f"{thermal}_coe_applied_positive_m"] = sum(daily[day]["coe_applied_positive_m"] for day in selected)
            row[f"{thermal}_coe_cap_adjustment_m"] = sum(daily[day]["coe_cap_adjustment_m"] for day in selected)
            for component in COMPONENTS:
                row[f"{thermal}_{component}_m"] = sum(daily[day][f"coe_{component}_m"] for day in selected)
                row[f"{thermal}_{component}_positive_m"] = sum(daily[day][f"coe_{component}_positive_m"] for day in selected)
            for field in (
                "stage3_surface_energy_j_m2", "stage3_shortwave_energy_j_m2",
                "stage3_longwave_energy_j_m2", "stage3_latent_energy_j_m2",
                "stage3_conduction_energy_j_m2", "stage3_cold_content_change_j_m2",
            ):
                row[f"{thermal}_{field}"] = sum(daily[day][field] for day in selected)
        output.append(row)
    return output


def monthly_rows(daily_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    grouped: dict[tuple[str, int, int, str], list[dict[str, Any]]] = defaultdict(list)
    for row in daily_rows:
        grouped[(row["lane"], row["water_year"], row["month"], row["thermal_class"])].append(row)
    output = []
    additive = (
        "fixture_precip_m", "snowfall_m", "rain_retained_m", "rain_released_m",
        "pack_loss_m", "sublimation_m", "raw_melt_m", "routed_melt_m",
        "coe_applied_positive_m", "coe_cap_adjustment_m", "radiation_mj_m2",
        "stage3_surface_energy_j_m2", "stage3_shortwave_energy_j_m2",
        "stage3_longwave_energy_j_m2", "stage3_latent_energy_j_m2",
        "stage3_conduction_energy_j_m2", "stage3_cold_content_change_j_m2",
        "stage3_refrozen_liquid_m", "stage3_retained_liquid_delta_m",
        "stage3_routed_liquid_m",
    )
    for (lane, water_year, month, thermal), rows_ in sorted(grouped.items()):
        output.append({
            "lane": lane,
            "display_lane": DISPLAY[lane],
            "role": "CANONICAL" if lane in CANONICAL else "DEVELOPMENT_ONLY",
            "water_year": water_year,
            "month": month,
            "thermal_class": thermal,
            "day_count": len(rows_),
            "material_pack_loss_day_count": sum(row["material_pack_loss"] for row in rows_),
            "mean_cloud_fraction": statistics.fmean(float(row["cloud_fraction_mean"]) for row in rows_),
            **{field: sum(float(row[field]) for row in rows_) for field in additive},
            **{
                f"coe_{name}_positive_m": sum(float(row[f"coe_{name}_positive_m"]) for row in rows_)
                for name in COMPONENTS
            },
            **{
                f"coe_{name}_m": sum(float(row[f"coe_{name}_m"]) for row in rows_)
                for name in COMPONENTS
            },
        })
    return output


def empirical_term_rows(annual: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return [
        {
            "lane": row["lane"], "role": row["role"], "water_year": row["water_year"],
            "thermal_class": thermal, "term": term,
            "signed_m": row[f"{thermal}_{term}_m"],
            "positive_m": row[f"{thermal}_{term}_positive_m"],
            "cap_adjustment_m": row[f"{thermal}_coe_cap_adjustment_m"],
            "applied_positive_m": row[f"{thermal}_coe_applied_positive_m"],
        }
        for row in annual for thermal in CLASSES for term in COMPONENTS
    ]


def forcing_state_contrasts(daily_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    fields = (
        "hourly_tmax_c", "wind_m_s", "dewpoint_c", "canopy_cover_fraction",
        "radiation_mj_m2", "cloud_fraction_mean", "fixture_precip_m", "snowfall_m",
        "runtime_depth_before_m", "runtime_density_before_kg_m3",
        "retained_liquid_before_m",
    )
    output = []
    for lane in LANES:
        years = sorted({row["water_year"] for row in daily_rows if row["lane"] == lane})
        deltas: dict[str, list[float]] = {field: [] for field in fields}
        valid_years = 0
        for water_year in years:
            eligible = [
                row for row in daily_rows
                if row["lane"] == lane and row["water_year"] == water_year
                and row["thermal_class"] != "unclassified"
            ]
            material = [row for row in eligible if row["material_pack_loss"]]
            nonmaterial = [row for row in eligible if not row["material_pack_loss"]]
            if not material or not nonmaterial:
                continue
            valid_years += 1
            for field in fields:
                deltas[field].append(
                    median(float(row[field]) for row in material)
                    - median(float(row[field]) for row in nonmaterial)
                )
        output.append({
            "lane": lane,
            "role": "CANONICAL" if lane in CANONICAL else "DEVELOPMENT_ONLY",
            "paired_year_count": valid_years,
            **{f"median_annual_material_minus_nonmaterial_{field}": median(values) for field, values in deltas.items()},
            **{f"fraction_years_positive_{field}": fraction(value > 0.0 for value in values) for field, values in deltas.items()},
        })
    return output


def dry_intervals(
    lane: str,
    windows: list[dict[str, Any]],
    climate: dict[dt.date, dict[str, float]],
    observations: dict[dt.date, dict[str, Any]],
    gauge: dict[dt.date, float],
    daily: dict[dt.date, dict[str, Any]],
    threshold: float,
    minimum_days: int,
) -> list[dict[str, Any]]:
    output = []
    one_day = dt.timedelta(days=1)
    for window in windows:
        candidates = [
            day for day in date_span(window["window_start"], window["observed_peak_date"])
            if day in climate and day in daily and day in gauge and day in observations
            and observations[day]["swe_m"] is not None
            and observations[day]["tmax_c"] is not None
            and climate[day]["precip_m"] <= threshold and gauge[day] <= threshold
        ]
        sequence = 0
        for group in group_consecutive(candidates):
            if len(group) < minimum_days:
                continue
            before = group[0] - one_day
            if before not in observations or observations[before]["swe_m"] is None:
                continue
            if observations[before]["swe_m"] <= 0.0 or daily[group[0]]["runtime_swe_before_m"] <= 0.0:
                continue
            sequence += 1
            observed_loss = max(observations[before]["swe_m"] - observations[group[-1]]["swe_m"], 0.0)
            output.append({
                "lane": lane,
                "display_lane": DISPLAY[lane],
                "role": "CANONICAL" if lane in CANONICAL else "DEVELOPMENT_ONLY",
                "water_year": window["water_year"],
                "interval_sequence": sequence,
                "interval_start": group[0].isoformat(),
                "interval_end": group[-1].isoformat(),
                "interval_day_count": len(group),
                "observed_wteq_loss_m": observed_loss,
                "modeled_pack_loss_m": sum(daily[day]["pack_loss_m"] for day in group),
                "warm_mixed_modeled_pack_loss_m": sum(
                    daily[day]["pack_loss_m"] for day in group
                    if daily[day]["thermal_class"] in ("mixed_day", "warm_day")
                ),
                "cold_day_count": sum(daily[day]["thermal_class"] == "cold_day" for day in group),
                "mixed_day_count": sum(daily[day]["thermal_class"] == "mixed_day" for day in group),
                "warm_day_count": sum(daily[day]["thermal_class"] == "warm_day" for day in group),
            })
    return output


def predecessor_dry_comparison(current: list[dict[str, Any]]) -> list[dict[str, Any]]:
    old_path = PREDECESSOR_ROOT / "tables/dry-intervals.csv"
    old: dict[tuple[str, int, str, str], dict[str, str]] = {}
    with old_path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            key = (row["site"], int(row["water_year"]), row["interval_start"], row["interval_end"])
            old[key] = row
    output = []
    for row in current:
        if row["lane"] not in CANONICAL:
            continue
        key = (row["lane"], row["water_year"], row["interval_start"], row["interval_end"])
        previous = old.get(key)
        if previous is None:
            raise RuntimeError(f"corrected dry interval missing predecessor key {key}")
        previous_loss = float(previous["modeled_pack_loss_m"])
        output.append({
            "site": row["lane"],
            "water_year": row["water_year"],
            "interval_start": row["interval_start"],
            "interval_end": row["interval_end"],
            "pre_21k_modeled_pack_loss_m": previous_loss,
            "corrected_modeled_pack_loss_m": row["modeled_pack_loss_m"],
            "corrected_minus_pre_21k_m": row["modeled_pack_loss_m"] - previous_loss,
        })
    if len(output) != len(old):
        raise RuntimeError(f"dry interval inventory differs: {len(output)} != {len(old)}")
    return output


def summarize_sites(annual: list[dict[str, Any]], constants: dict[str, Any]) -> list[dict[str, Any]]:
    output = []
    for site in CANONICAL:
        rows = [row for row in annual if row["lane"] == site and row["warm_mixed_pack_loss_fraction"] is not None]
        shares = [row["warm_mixed_pack_loss_fraction"] for row in rows]
        signal = (
            len(rows) >= constants["minimum_eligible_years_per_site"]
            and median(shares) >= constants["warm_mixed_loss_fraction_threshold"]
            and fraction(value >= 0.5 for value in shares) >= constants["site_direction_fraction"]
        )
        component_annual_medians = {
            name: median(
                row[f"mixed_day_{name}_positive_m"]
                + row[f"warm_day_{name}_positive_m"]
                for row in rows
            )
            for name in COMPONENTS
        }
        component_totals = {
            name: sum(
                row[f"mixed_day_{name}_positive_m"] + row[f"warm_day_{name}_positive_m"]
                for row in rows
            ) for name in COMPONENTS
        }
        applied = [row["coe_applied_positive_m"] for row in rows]
        losses = [row["modeled_pack_loss_m"] for row in rows]
        forcing_associations = {
            "precipitation": pearson([row["fixture_precipitation_m"] for row in rows], losses),
            "radiation": pearson([row["forcing_radiation_mj_m2"] for row in rows], losses),
            "cloud_fraction": pearson([row["forcing_mean_cloud_fraction"] for row in rows], losses),
        }
        output.append({
            "site": site,
            "display_site": DISPLAY[site],
            "eligible_year_count": len(rows),
            "median_prepeak_pack_loss_m": median(row["modeled_pack_loss_m"] for row in rows),
            "median_warm_mixed_pack_loss_m": median(row["warm_mixed_pack_loss_m"] for row in rows),
            "median_warm_mixed_pack_loss_fraction": median(shares),
            "fraction_years_warm_mixed_share_at_least_half": fraction(value >= 0.5 for value in shares),
            "warm_mixed_site_signal": signal,
            "annual_coe_loss_association_count": len(rows),
            "annual_coe_applied_positive_vs_pack_loss_pearson": pearson(applied, losses),
            "annual_coe_applied_positive_vs_pack_loss_spearman": spearman(applied, losses),
            **{
                f"annual_{name}_vs_pack_loss_pearson": value
                for name, value in forcing_associations.items()
            },
            "dominant_positive_empirical_term_warm_mixed": max(
                component_annual_medians, key=component_annual_medians.get
            ),
            **{
                f"warm_mixed_positive_{name}_annual_median_m": value
                for name, value in component_annual_medians.items()
            },
            **{
                f"secondary_exposure_weighted_warm_mixed_positive_{name}_total_m": value
                for name, value in component_totals.items()
            },
        })
    return output


def snowbird_pairs(annual: list[dict[str, Any]], constants: dict[str, Any]) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    canonical = {row["water_year"]: row for row in annual if row["lane"] == "snotel_snowbird_ut"}
    scaled = {row["water_year"]: row for row in annual if row["lane"] == DEVELOPMENT}
    if canonical.keys() != scaled.keys():
        raise RuntimeError("Snowbird annual pairing inventory differs")
    pairs = []
    for water_year in sorted(canonical):
        left, right = canonical[water_year], scaled[water_year]
        pairs.append({
            "water_year": water_year,
            "canonical_pack_loss_m": left["modeled_pack_loss_m"],
            "scaled_pack_loss_m": right["modeled_pack_loss_m"],
            "scaled_minus_canonical_pack_loss_m": right["modeled_pack_loss_m"] - left["modeled_pack_loss_m"],
            "canonical_modeled_peak_swe_m": left["modeled_peak_swe_m"],
            "scaled_modeled_peak_swe_m": right["modeled_peak_swe_m"],
            "scaled_minus_canonical_peak_swe_m": right["modeled_peak_swe_m"] - left["modeled_peak_swe_m"],
            "canonical_observed_date_swe_m": left["modeled_swe_on_observed_peak_m"],
            "scaled_observed_date_swe_m": right["modeled_swe_on_observed_peak_m"],
            "scaled_minus_canonical_observed_date_swe_m": right["modeled_swe_on_observed_peak_m"] - left["modeled_swe_on_observed_peak_m"],
        })
    deltas = [row["scaled_minus_canonical_pack_loss_m"] for row in pairs]
    nonzero = [value for value in deltas if abs(value) > constants["zero_tolerance_m"]]
    direction = max(
        fraction(value > 0.0 for value in nonzero) or 0.0,
        fraction(value < 0.0 for value in nonzero) or 0.0,
    )
    summary = {
        "role": "DEVELOPMENT_ONLY",
        "paired_year_count": len(pairs),
        "median_scaled_minus_canonical_pack_loss_m": median(deltas),
        "median_abs_scaled_minus_canonical_pack_loss_m": median(abs(value) for value in deltas),
        "direction_fraction_nonzero_pack_loss_delta": direction,
        "median_scaled_minus_canonical_peak_swe_m": median(row["scaled_minus_canonical_peak_swe_m"] for row in pairs),
        "median_scaled_minus_canonical_observed_date_swe_m": median(row["scaled_minus_canonical_observed_date_swe_m"] for row in pairs),
    }
    summary["state_signal"] = (
        len(pairs) >= constants["minimum_eligible_years_per_site"]
        and summary["median_abs_scaled_minus_canonical_pack_loss_m"] >= constants["scaled_state_response_materiality_m"]
        and direction >= constants["scaled_direction_fraction"]
    )
    return pairs, summary


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        raise RuntimeError(f"refusing empty table {path}")
    path.parent.mkdir(parents=True, exist_ok=True)
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
    identities = freeze["source_identities"]
    constants = freeze["constants"]
    if sha256(RECEIPT_PATH) != identities["execution_receipt_sha256"]:
        raise RuntimeError("21K receipt identity differs from freeze")
    if sha256(MATERIALITY_PATH) != identities["materiality_result_sha256"]:
        raise RuntimeError("21K materiality identity differs from freeze")
    predecessor_result = PREDECESSOR_ROOT / "results.json"
    if sha256(predecessor_result) != identities["predecessor_21j_result_sha256"]:
        raise RuntimeError("21J result identity differs from freeze")
    predecessor_dry = PREDECESSOR_ROOT / "tables/dry-intervals.csv"
    if sha256(predecessor_dry) != identities["predecessor_21j_dry_intervals_sha256"]:
        raise RuntimeError("21J dry-interval table identity differs from freeze")
    predecessor_manifest_path = PREDECESSOR_PACKAGE / "evidence-manifest.json"
    predecessor_receipt_path = PREDECESSOR_PACKAGE / "execution-receipt.json"
    if sha256(predecessor_manifest_path) != identities["predecessor_21j_tracked_manifest_sha256"]:
        raise RuntimeError("21J tracked manifest identity differs from freeze")
    if sha256(predecessor_receipt_path) != identities["predecessor_21j_tracked_receipt_sha256"]:
        raise RuntimeError("21J tracked receipt identity differs from freeze")
    predecessor_manifest = json.loads(predecessor_manifest_path.read_text(encoding="utf-8"))
    predecessor_receipt_record = json.loads(predecessor_receipt_path.read_text(encoding="utf-8"))
    retained_receipt_hash = predecessor_manifest["retained_hashes"]["execution-receipt.json"]
    if retained_receipt_hash != sha256(PREDECESSOR_ROOT / "execution-receipt.json"):
        raise RuntimeError("21J manifest-to-retained-receipt custody differs")
    if predecessor_receipt_record["retained_execution_receipt_sha256"] != retained_receipt_hash:
        raise RuntimeError("21J tracked receipt-to-retained-receipt custody differs")
    if predecessor_receipt_record["output_hashes"]["dry_intervals_csv"] != identities["predecessor_21j_dry_intervals_sha256"]:
        raise RuntimeError("21J tracked receipt-to-dry-table custody differs")
    receipt = json.loads(RECEIPT_PATH.read_text(encoding="utf-8"))
    snowbird_climate_custody = compare_precipitation_only_climates(
        climate_path("snotel_snowbird_ut"), climate_path(DEVELOPMENT)
    )

    annual: list[dict[str, Any]] = []
    scoped_daily: list[dict[str, Any]] = []
    hourly_rows: list[dict[str, Any]] = []
    dry: list[dict[str, Any]] = []
    validation: dict[str, Any] = {}
    source_identity: dict[str, Any] = {}
    for lane in LANES:
        site = "snotel_snowbird_ut" if lane == DEVELOPMENT else lane
        observation_path = OBS_ROOT / f"{site}.csv"
        expected_observation = identities["observation_sha256_by_site"][site]
        if sha256(observation_path) != expected_observation:
            raise RuntimeError(f"observation identity differs for {site}")
        cli = climate_path(lane)
        if sha256(cli) != receipt["lanes"][lane]["staged_cli_sha256"]:
            raise RuntimeError(f"climate identity differs for {lane}")
        receipt_trace_path = receipt["lanes"][lane]["trace"]
        receipt_trace_hash = receipt["lanes"][lane]["trace_sha256"]
        expected_trace_path = str(trace_path(lane).relative_to(REPO))
        if receipt_trace_path != expected_trace_path:
            raise RuntimeError(f"trace path custody differs for {lane}")
        if receipt_trace_hash != identities["trace_sha256_by_lane"][lane]:
            raise RuntimeError(f"trace hash custody differs for {lane}")
        climate = parse_cli(cli)
        observations = parse_observations(observation_path)
        windows = observed_windows(observations)
        scoped_water_year = {
            stamp: window["water_year"]
            for window in windows
            for stamp in date_span(window["window_start"], window["observed_peak_date"])
        }
        daily, lane_hourly, trace_validation = reduce_trace(
            lane, climate, identities["trace_sha256_by_lane"][lane],
            float(constants["mass_tolerance_m"]), float(constants["energy_tolerance_j_m2"]),
            scoped_water_year,
        )
        lane_annual = annual_rows(
            lane, windows, daily, float(constants["material_daily_loss_m"])
        )
        lane_scoped_daily = scoped_daily_rows(
            lane, windows, daily, float(constants["material_daily_loss_m"])
        )
        gauge = guarded_precipitation(observations, float(constants["zero_tolerance_m"]))
        lane_dry = dry_intervals(
            lane, windows, climate, observations, gauge, daily,
            float(constants["dry_precipitation_threshold_m_per_day"]),
            int(constants["minimum_episode_days"]),
        )
        annual.extend(lane_annual)
        scoped_daily.extend(lane_scoped_daily)
        hourly_rows.extend(lane_hourly)
        dry.extend(lane_dry)
        validation[lane] = trace_validation
        source_identity[lane] = {
            "classification": receipt["lanes"][lane]["classification"],
            "climate": {"path": str(cli.relative_to(REPO)), "sha256": sha256(cli)},
            "observation": {"path": str(observation_path.relative_to(REPO)), "sha256": sha256(observation_path)},
            "trace": trace_validation,
            "annual_window_count": len(lane_annual),
            "dry_interval_count": len(lane_dry),
        }

    dry_comparison = predecessor_dry_comparison(dry)
    site_summary = summarize_sites(annual, constants)
    pairs, pair_summary = snowbird_pairs(annual, constants)
    monthly = monthly_rows(scoped_daily)
    empirical_terms = empirical_term_rows(annual)
    forcing_state = forcing_state_contrasts(scoped_daily)
    hourly_classes = hourly_rows
    stage3_response = [
        {key: row[key] for key in (
            "lane", "role", "water_year", "date", "month", "thermal_class",
            "stage3_surface_energy_j_m2", "stage3_shortwave_energy_j_m2",
            "stage3_longwave_energy_j_m2", "stage3_latent_energy_j_m2",
            "stage3_conduction_energy_j_m2", "stage3_cold_content_change_j_m2",
            "stage3_refrozen_liquid_m", "stage3_retained_liquid_delta_m",
            "stage3_routed_liquid_m",
            "stage3_incoming_liquid_m", "stage3_cold_content_before_j_m2",
            "stage3_cold_content_after_j_m2", "stage3_latent_refreeze_energy_j_m2",
            "stage3_cold_content_export_j_m2",
        )}
        for row in scoped_daily
    ]
    warm_mixed_site_count = sum(row["warm_mixed_site_signal"] for row in site_summary)
    coverage = all(row["eligible_year_count"] >= constants["minimum_eligible_years_per_site"] for row in site_summary)
    systemic = warm_mixed_site_count >= constants["systemic_site_count"]
    state = pair_summary["state_signal"]
    if not coverage:
        verdict = "UNRESOLVED_OR_COVERAGE_LIMITED"
    elif systemic and state:
        verdict = "MULTIFACTOR_WARM_MIXED_AND_STATE_SIGNAL"
    elif systemic:
        verdict = "WARM_MIXED_COE_LOSS_CONCENTRATION_SIGNAL"
    elif state:
        verdict = "STATE_MEDIATED_INPUT_SENSITIVITY_SIGNAL"
    else:
        verdict = "NO_SYSTEMIC_WARM_MIXED_SIGNAL"
    max_dry_delta = max(abs(row["corrected_minus_pre_21k_m"]) for row in dry_comparison)
    result = {
        "schema_version": 1,
        "status": "ANALYSIS_COMPLETE",
        "evidence_mode": "Ran: receipt-bound corrected-state five-lane analysis",
        "observation_role": freeze["observation_role"],
        "verdict": verdict,
        "coverage_complete": coverage,
        "systemic_warm_mixed": systemic,
        "warm_mixed_site_signal_count": warm_mixed_site_count,
        "snowbird_development_state_signal": state,
        "snowbird_climate_custody": snowbird_climate_custody,
        "maximum_abs_corrected_minus_pre_21k_dry_loss_m": max_dry_delta,
        "counts": {
            "annual_rows": len(annual),
            "canonical_annual_rows": sum(row["role"] == "CANONICAL" for row in annual),
            "dry_interval_rows": len(dry),
            "canonical_dry_interval_rows": sum(row["role"] == "CANONICAL" for row in dry),
            "snowbird_pair_rows": len(pairs),
            "daily_rows": len(scoped_daily),
            "monthly_rows": len(monthly),
            "hourly_class_rows": len(hourly_classes),
            "empirical_term_rows": len(empirical_terms),
            "stage3_response_rows": len(stage3_response),
            "forcing_state_contrast_rows": len(forcing_state),
        },
        "site_summary": site_summary,
        "snowbird_pair_summary": pair_summary,
        "trace_validation": validation,
        "source_identity": source_identity,
        "claim_limits": [
            "Empirical CoE terms are melt-depth formula contributions, not measured energy shares.",
            "Stage-3 energy is downstream response, not upstream CoE melt causation.",
            "Scaled Snowbird is DEVELOPMENT_ONLY input sensitivity.",
            "No association or dominance result authorizes tuning or a production correction."
        ],
    }
    tables = output_root / "tables"
    write_csv(tables / "annual-attribution.csv", annual)
    write_csv(tables / "dry-intervals-corrected.csv", dry)
    write_csv(tables / "dry-interval-predecessor-comparison.csv", dry_comparison)
    write_csv(tables / "site-summary.csv", site_summary)
    write_csv(tables / "snowbird-pairs.csv", pairs)
    write_csv(tables / "daily-attribution.csv", scoped_daily)
    write_csv(tables / "monthly-attribution.csv", monthly)
    write_csv(tables / "hourly-classification.csv", hourly_classes)
    write_csv(tables / "empirical-term-attribution.csv", empirical_terms)
    write_csv(tables / "stage3-response.csv", stage3_response)
    write_csv(tables / "forcing-state-contrasts.csv", forcing_state)
    write_json(output_root / "results.json", result)
    accepted_outputs = [
        output_root / "results.json",
        tables / "annual-attribution.csv",
        tables / "dry-intervals-corrected.csv",
        tables / "dry-interval-predecessor-comparison.csv",
        tables / "site-summary.csv",
        tables / "snowbird-pairs.csv",
        tables / "daily-attribution.csv",
        tables / "monthly-attribution.csv",
        tables / "hourly-classification.csv",
        tables / "empirical-term-attribution.csv",
        tables / "stage3-response.csv",
        tables / "forcing-state-contrasts.csv",
    ]
    outputs = {
        str(path.relative_to(REPO)): {"sha256": sha256(path), "size_bytes": path.stat().st_size}
        for path in accepted_outputs
    }
    write_json(output_root / "execution-receipt.json", {
        "schema_version": 1,
        "status": "PASS",
        "working_directory": str(REPO),
        "scaffold_commit": "785e0fdd",
        "freeze_sha256": sha256_bytes(freeze_bytes),
        "tool_sha256": sha256(Path(__file__)),
        "source_identity": source_identity,
        "outputs_before_receipt": outputs,
        "verdict": verdict,
    })
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-root", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()
    result = execute(args.output_root.resolve())
    print(json.dumps({
        "verdict": result["verdict"],
        "counts": result["counts"],
        "site_summary": result["site_summary"],
        "snowbird_pair_summary": result["snowbird_pair_summary"],
    }, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
