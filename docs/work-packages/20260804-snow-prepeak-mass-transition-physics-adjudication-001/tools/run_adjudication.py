#!/usr/bin/env python3
"""Execute and analyze the frozen four-site snow mass-transition cohort."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import importlib.util
import json
import math
import os
import shutil
import statistics
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any, Iterable

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
FREEZE_PATH = PACKAGE / "artifacts/analysis-freeze-v2.json"
OUTPUT = REPO / "target/snow_prepeak_mass_transition_physics_adjudication_v2"
SOURCE_FIXTURES = (
    REPO
    / "target/snow_prepeak_liquid_evacuation_physics_audit_v3/fixtures/baseline_replay"
)
PREDECESSOR_RUNS = (
    REPO
    / "target/snow_prepeak_liquid_evacuation_physics_audit_v3/runs/baseline_replay"
)
BINARY = REPO / "target/release/openwepp-cli-hill"
W1_TOOL = REPO / (
    "docs/work-packages/20260802-snow-surface-eb-04w1-"
    "precipitation-scaling-calibration-001/tools/run_precipitation_scaling.py"
)
SCAFFOLD_COMMIT = "6ab0946b"
PRIMARY_EVENT_THRESHOLD_M = 0.0005
ZERO = 1.0e-12
MASS_TOLERANCE_M = 1.0e-9
LATENT_HEAT_FUSION_J_KG = 333_550.0
RHO_WATER_KG_M3 = 1000.0
CENSORED_WATER_YEARS = {2025}
COMPONENT_FIELDS = (
    "coe_melt_amelt_m",
    "coe_melt_bmelt_m",
    "coe_melt_cmelt_m",
    "coe_melt_dmelt_m",
)
ADDED_SCHEMA_V4_FIELDS = {
    "canopy_cover_fraction",
    "dewpoint_c",
    "stage3_hourly_active_cold_content_j_m2",
    "stage3_hourly_active_depth_m",
    "stage3_hourly_active_mass_kg_m2",
    "stage3_hourly_active_temperature_c",
    "stage3_hourly_lower_cold_content_j_m2",
    "stage3_hourly_lower_depth_m",
    "stage3_hourly_lower_mass_kg_m2",
    "stage3_hourly_lower_present_fraction",
    "stage3_hourly_lower_temperature_c",
    "stage3_incoming_liquid_m",
    "stage3_liquid_closure_residual_m",
    "stage3_retained_liquid_delta_m",
    "stage3_routed_liquid_m",
    "wind_m_s",
}
ADDED_HOURLY_V4_FIELDS = {
    "air_temperature_c",
    "cloud_fraction",
    "liquid_holding_capacity_m",
    "liquid_water_released_m",
    "liquid_water_retained_after_m",
    "liquid_water_retained_before_m",
    "pack_density_after_kg_m3",
    "pack_density_before_kg_m3",
    "pack_depth_after_m",
    "pack_depth_before_m",
    "radiation_mj_m2",
    "rain_released_m",
    "routed_melt_m",
    "sublimation_m",
}


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def command_output(argv: list[str]) -> str:
    return subprocess.run(
        argv, check=True, text=True, capture_output=True, cwd=REPO
    ).stdout.strip()


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(value, allow_nan=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not rows:
        raise RuntimeError(f"refusing to write empty table: {path}")
    fieldnames = list(rows[0])
    if any(list(row) != fieldnames for row in rows):
        raise RuntimeError(f"inconsistent CSV schema: {path}")
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def file_manifest(root: Path) -> dict[str, Any]:
    files = []
    for path in sorted(candidate for candidate in root.rglob("*") if candidate.is_file()):
        files.append(
            {
                "path": str(path.relative_to(root)),
                "sha256": sha256(path),
                "size_bytes": path.stat().st_size,
            }
        )
    payload = json.dumps(files, sort_keys=True, separators=(",", ":")).encode()
    return {
        "file_count": len(files),
        "files": files,
        "manifest_sha256": hashlib.sha256(payload).hexdigest(),
    }


def climate_file(root: Path) -> Path:
    files = sorted(root.glob("*.cli"))
    if len(files) != 1:
        raise RuntimeError(f"expected exactly one climate file under {root}")
    return files[0]


def climate_dates(path: Path) -> list[dt.date]:
    dates = []
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            fields = line.split()
            if len(fields) != 13:
                continue
            try:
                stamp = dt.date(int(fields[2]), int(fields[1]), int(fields[0]))
                list(map(float, fields[3:]))
            except ValueError:
                continue
            dates.append(stamp)
    if not dates or dates != sorted(dates) or len(dates) != len(set(dates)):
        raise RuntimeError(f"invalid climate chronology: {path}")
    return dates


def observed_peaks(path: Path) -> tuple[dict[int, tuple[dt.date, float]], list[int]]:
    peaks: dict[int, tuple[dt.date, float]] = {}
    with path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            raw = row.get("observed_swe_mm")
            if raw in (None, ""):
                continue
            water_year = int(row["water_year"])
            candidate = (dt.date.fromisoformat(row["date"]), float(raw) / 1000.0)
            current = peaks.get(water_year)
            if current is None or candidate[1] > current[1]:
                peaks[water_year] = candidate
    skipped = sorted(year for year, (_, value) in peaks.items() if value <= 0.0)
    return ({year: peak for year, peak in peaks.items() if peak[1] > 0.0}, skipped)


def temperature_class(value: float) -> str:
    if value <= 0.0:
        return "le_0_c"
    if value <= 2.0:
        return "gt_0_le_2_c"
    return "gt_2_c"


def precipitation_class(hour: dict[str, Any]) -> str:
    snow = float(hour["snowfall_swe_m"]) > ZERO
    rain = float(hour["rain_m"]) > ZERO
    if snow and rain:
        return "mixed_or_both"
    if snow:
        return "snowfall"
    if rain:
        return "rain"
    return "dry"


def empty_hour_bucket() -> dict[str, Any]:
    return {
        "hour_count": 0,
        "gross_positive_applied_m": 0.0,
        "negative_applied_m": 0.0,
        "signed_applied_m": 0.0,
        **{f"signed_{field}": 0.0 for field in COMPONENT_FIELDS},
        **{f"positive_hour_{field}": 0.0 for field in COMPONENT_FIELDS},
        **{f"negative_hour_{field}": 0.0 for field in COMPONENT_FIELDS},
    }


def add_hour(bucket: dict[str, Any], hour: dict[str, Any]) -> None:
    applied = float(hour["coe_melt_applied_m"])
    bucket["hour_count"] += 1
    bucket["gross_positive_applied_m"] += max(applied, 0.0)
    bucket["negative_applied_m"] += min(applied, 0.0)
    bucket["signed_applied_m"] += applied
    for field in COMPONENT_FIELDS:
        value = float(hour[field])
        bucket[f"signed_{field}"] += value
        if applied > ZERO:
            bucket[f"positive_hour_{field}"] += value
        elif applied < -ZERO:
            bucket[f"negative_hour_{field}"] += value


def add_bucket(target: dict[str, Any], source: dict[str, Any]) -> None:
    for key, value in source.items():
        target[key] += value


def reduce_trace_row(stamp: dt.date, row: dict[str, Any]) -> dict[str, Any]:
    hourly = empty_hour_bucket()
    classes: dict[str, dict[str, Any]] = {}
    hourly_snowfall_m = 0.0
    for hour in row["accumulation_melt_hourly"]:
        add_hour(hourly, hour)
        hourly_snowfall_m += float(hour["snowfall_swe_m"])
        selectors = (
            ("temperature", temperature_class(float(hour["air_temperature_c"]))),
            ("precipitation", precipitation_class(hour)),
            (
                "radiation",
                "positive" if float(hour["radiation_mj_m2"]) > ZERO else "zero",
            ),
            (
                "pack_state",
                "pack_present"
                if float(hour["pack_depth_before_m"]) > ZERO
                else "snow_free",
            ),
        )
        for dimension, category in selectors:
            key = f"{dimension}:{category}"
            bucket = classes.setdefault(key, empty_hour_bucket())
            add_hour(bucket, hour)

    incoming = float(row["stage3_incoming_liquid_m"])
    routed = float(row["stage3_routed_liquid_m"])
    retained_delta = float(row["stage3_retained_liquid_delta_m"])
    refrozen = float(row["stage3_refrozen_liquid_m"])
    solid_loss = float(row["snowpack_swe_loss_m"])
    rain_released = float(row["rain_released_m"])
    cold_before = float(row["stage3_cold_content_before_j_m2"])
    signed_cancellation = hourly["gross_positive_applied_m"] - max(
        hourly["signed_applied_m"], 0.0
    )
    cold_opportunity = min(
        incoming,
        max(cold_before, 0.0) / (RHO_WATER_KG_M3 * LATENT_HEAT_FUSION_J_KG),
    )
    storage_delta = float(row["runtime_swe_after_m"]) - float(
        row["runtime_swe_before_m"]
    )
    storage_expected = (
        hourly_snowfall_m
        + float(row["rain_retained_m"])
        - solid_loss
        - float(row["sublimation_m"])
    )
    reconstructed_stage3 = incoming - routed - retained_delta - refrozen
    layer_liquid_before = sum(
        float(layer["liquid_water_m"]) for layer in row["snow_layers_before"]
    )
    layer_liquid_after = sum(
        float(layer["liquid_water_m"]) for layer in row["snow_layers_after"]
    )
    layer_refrozen_before = sum(
        float(layer["refrozen_liquid_m"]) for layer in row["snow_layers_before"]
    )
    layer_refrozen_after = sum(
        float(layer["refrozen_liquid_m"]) for layer in row["snow_layers_after"]
    )
    return {
        "date": stamp,
        "runtime_swe_before_m": float(row["runtime_swe_before_m"]),
        "runtime_swe_after_m": float(row["runtime_swe_after_m"]),
        "accumulation_m": hourly_snowfall_m,
        "reported_accumulation_m": float(row["accumulation_m"]),
        "accumulation_hourly_residual_m": float(row["accumulation_m"])
        - hourly_snowfall_m,
        "rain_retained_m": float(row["rain_retained_m"]),
        "rain_released_m": rain_released,
        "snowpack_swe_loss_m": solid_loss,
        "sublimation_m": float(row["sublimation_m"]),
        "raw_melt_m": float(row["raw_melt_m"]),
        "top_level_routed_melt_m": float(row["routed_melt_m"]),
        "top_level_liquid_water_released_m": float(row["liquid_water_released_m"]),
        "retained_store_before_m": float(row["liquid_water_retained_before_m"]),
        "retained_store_after_m": float(row["liquid_water_retained_after_m"]),
        "stage3_incoming_m": incoming,
        "stage3_routed_m": routed,
        "stage3_retained_delta_m": retained_delta,
        "stage3_refrozen_m": refrozen,
        "stage3_producer_residual_m": float(row["stage3_liquid_closure_residual_m"]),
        "stage3_cold_content_before_j_m2": cold_before,
        "stage3_cold_content_after_j_m2": float(
            row["stage3_cold_content_after_j_m2"]
        ),
        "layer_liquid_store_before_m": layer_liquid_before,
        "layer_liquid_store_after_m": layer_liquid_after,
        "layer_liquid_store_day_delta_m": layer_liquid_after - layer_liquid_before,
        "layer_refrozen_tag_before_m": layer_refrozen_before,
        "layer_refrozen_tag_after_m": layer_refrozen_after,
        "layer_refrozen_tag_day_delta_m": layer_refrozen_after
        - layer_refrozen_before,
        "storage_closure_residual_m": storage_delta - storage_expected,
        "handoff_closure_residual_m": incoming - (solid_loss + rain_released),
        "stage3_reconstructed_residual_m": reconstructed_stage3,
        "stage3_residual_difference_m": reconstructed_stage3
        - float(row["stage3_liquid_closure_residual_m"]),
        "daily_local_signed_opportunity_m": signed_cancellation,
        "post_coe_stage3_cold_opportunity_m": cold_opportunity,
        "solid_source_limited_cold_index_m": min(
            solid_loss,
            max(cold_before, 0.0)
            / (RHO_WATER_KG_M3 * LATENT_HEAT_FUSION_J_KG),
        ),
        "opportunity_overlap_cap_m": min(signed_cancellation, cold_opportunity),
        "mixed_signed_hour_day": (
            hourly["gross_positive_applied_m"] > ZERO
            and hourly["negative_applied_m"] < -ZERO
        ),
        "hourly": hourly,
        "hour_classes": classes,
    }


def compare_and_reduce_trace(
    trace: Path, predecessor: Path, dates: list[dt.date]
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    reduced = []
    mismatch_rows = 0
    mismatch_fields: dict[str, int] = {}
    old_keys: set[str] | None = None
    new_keys: set[str] | None = None
    row_count = 0
    with trace.open(encoding="utf-8") as current_handle, predecessor.open(
        encoding="utf-8"
    ) as old_handle:
        for stamp, current_line, old_line in zip(
            dates, current_handle, old_handle, strict=True
        ):
            current = json.loads(current_line)
            old = json.loads(old_line)
            row_count += 1
            if old_keys is None:
                old_keys = set(old)
                new_keys = set(current)
                if new_keys - old_keys != ADDED_SCHEMA_V4_FIELDS or old_keys - new_keys:
                    raise RuntimeError("schema-v4 field delta differs from frozen expectation")
                old_hour_keys = set(old["accumulation_melt_hourly"][0])
                new_hour_keys = set(current["accumulation_melt_hourly"][0])
                if (
                    new_hour_keys - old_hour_keys != ADDED_HOURLY_V4_FIELDS
                    or old_hour_keys - new_hour_keys
                ):
                    raise RuntimeError(
                        "schema-v4 hourly field delta differs from frozen expectation"
                    )
            differences = [
                key
                for key in old
                if key not in {"schema", "accumulation_melt_hourly"}
                and current.get(key) != old[key]
            ]
            old_hours = old["accumulation_melt_hourly"]
            current_hours = current["accumulation_melt_hourly"]
            if len(old_hours) != len(current_hours) or any(
                any(current_hour.get(key) != value for key, value in old_hour.items())
                for old_hour, current_hour in zip(old_hours, current_hours, strict=True)
            ):
                differences.append("accumulation_melt_hourly_pre_v4_projection")
            if not (
                old["schema"] == "openwepp-r7h-direct-production-snow-trace-v3"
                and current["schema"] == "openwepp-r7h-direct-production-snow-trace-v4"
            ):
                differences.append("schema_transition")
            if differences:
                mismatch_rows += 1
                for key in differences:
                    mismatch_fields[key] = mismatch_fields.get(key, 0) + 1
            reduced.append(reduce_trace_row(stamp, current))
        if current_handle.readline() or old_handle.readline():
            raise RuntimeError(f"trace row-count mismatch: {trace}")
    if row_count != len(dates):
        raise RuntimeError(f"trace/climate row-count mismatch: {trace}")
    return reduced, {
        "row_count": row_count,
        "old_field_count": len(old_keys or ()),
        "new_field_count": len(new_keys or ()),
        "added_fields": sorted(ADDED_SCHEMA_V4_FIELDS),
        "added_hourly_fields": sorted(ADDED_HOURLY_V4_FIELDS),
        "expected_schema_transition": "v3 -> v4",
        "mismatch_row_count": mismatch_rows,
        "mismatch_fields": mismatch_fields,
        "exact_pre_v4_field_identity": mismatch_rows == 0,
    }


def sum_field(rows: Iterable[dict[str, Any]], field: str) -> float:
    return sum(float(row[field]) for row in rows)


def safe_ratio(numerator: float, denominator: float) -> float | None:
    if denominator <= ZERO:
        return None
    return numerator / denominator


def operands_differ(left: float, right: float) -> bool:
    return abs(left - right) > ZERO


def aggregate_hour_classes(rows: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    output: dict[str, dict[str, Any]] = {}
    for row in rows:
        for key, bucket in row["hour_classes"].items():
            add_bucket(output.setdefault(key, empty_hour_bucket()), bucket)
    return output


def loss_events(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    groups: list[list[dict[str, Any]]] = []
    active: list[dict[str, Any]] = []
    for row in rows:
        if row["snowpack_swe_loss_m"] >= PRIMARY_EVENT_THRESHOLD_M:
            if active and (row["date"] - active[-1]["date"]).days > 1:
                groups.append(active)
                active = []
            active.append(row)
        elif active:
            groups.append(active)
            active = []
    if active:
        groups.append(active)
    events = []
    for group in groups:
        events.append(
            {
                "start": group[0]["date"].isoformat(),
                "end": group[-1]["date"].isoformat(),
                "day_count": len(group),
                "snowpack_swe_loss_m": sum_field(group, "snowpack_swe_loss_m"),
                "snowfall_m": sum_field(group, "accumulation_m"),
                "stage3_incoming_m": sum_field(group, "stage3_incoming_m"),
                "stage3_routed_m": sum_field(group, "stage3_routed_m"),
                "stage3_retained_positive_m": sum(
                    max(row["stage3_retained_delta_m"], 0.0) for row in group
                ),
                "stage3_refrozen_m": sum_field(group, "stage3_refrozen_m"),
                "daily_local_signed_opportunity_m": sum_field(
                    group, "daily_local_signed_opportunity_m"
                ),
                "post_coe_stage3_cold_opportunity_m": sum_field(
                    group, "post_coe_stage3_cold_opportunity_m"
                ),
                "solid_source_limited_cold_index_m": sum_field(
                    group, "solid_source_limited_cold_index_m"
                ),
                "mixed_signed_hour_days": sum(
                    bool(row["mixed_signed_hour_day"]) for row in group
                ),
            }
        )
    return sorted(events, key=lambda event: event["snowpack_swe_loss_m"], reverse=True)


def analyze_windows(
    site: str,
    rows: list[dict[str, Any]],
    observation_file: Path,
) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]], list[int]]:
    peaks, skipped_zero = observed_peaks(observation_file)
    annual = []
    events = []
    hour_classes = []
    for water_year, (peak_date, observed_peak_m) in sorted(peaks.items()):
        start = dt.date(water_year - 1, 10, 1)
        window = [row for row in rows if start <= row["date"] <= peak_date]
        if not window or window[0]["date"] != start or window[-1]["date"] != peak_date:
            continue
        modeled_peak_row = max(window, key=lambda row: row["runtime_swe_after_m"])
        incoming = sum_field(window, "stage3_incoming_m")
        retained_positive = sum(
            max(row["stage3_retained_delta_m"], 0.0) for row in window
        )
        retained_negative = sum(
            min(row["stage3_retained_delta_m"], 0.0) for row in window
        )
        refrozen = sum_field(window, "stage3_refrozen_m")
        solid_loss = sum_field(window, "snowpack_swe_loss_m")
        gross_positive = sum(
            row["hourly"]["gross_positive_applied_m"] for row in window
        )
        negative_applied = sum(row["hourly"]["negative_applied_m"] for row in window)
        deficit = observed_peak_m - modeled_peak_row["runtime_swe_after_m"]
        cold_index = sum_field(window, "post_coe_stage3_cold_opportunity_m")
        solid_cold_index = sum_field(window, "solid_source_limited_cold_index_m")
        daily_signed_index = sum_field(window, "daily_local_signed_opportunity_m")
        window_signed_index = gross_positive - max(
            gross_positive + negative_applied, 0.0
        )
        annual_row = {
            "site": site,
            "water_year": water_year,
            "right_censored": water_year in CENSORED_WATER_YEARS,
            "window_start": start.isoformat(),
            "observed_peak_date": peak_date.isoformat(),
            "observed_peak_swe_m": observed_peak_m,
            "modeled_peak_date": modeled_peak_row["date"].isoformat(),
            "modeled_peak_swe_m": modeled_peak_row["runtime_swe_after_m"],
            "peak_swe_ratio": modeled_peak_row["runtime_swe_after_m"] / observed_peak_m,
            "peak_date_offset_days": (modeled_peak_row["date"] - peak_date).days,
            "observed_minus_modeled_peak_m": deficit,
            "initial_swe_m": window[0]["runtime_swe_before_m"],
            "final_swe_m": window[-1]["runtime_swe_after_m"],
            "storage_change_m": window[-1]["runtime_swe_after_m"]
            - window[0]["runtime_swe_before_m"],
            "snowfall_m": sum_field(window, "accumulation_m"),
            "rain_retained_m": sum_field(window, "rain_retained_m"),
            "rain_released_m": sum_field(window, "rain_released_m"),
            "solid_pack_loss_m": solid_loss,
            "sublimation_m": sum_field(window, "sublimation_m"),
            "gross_positive_applied_coe_m": gross_positive,
            "negative_applied_coe_m": negative_applied,
            "signed_applied_coe_m": gross_positive + negative_applied,
            "stage3_incoming_m": incoming,
            "stage3_routed_m": sum_field(window, "stage3_routed_m"),
            "stage3_retained_positive_m": retained_positive,
            "stage3_retained_negative_m": retained_negative,
            "stage3_refrozen_m": refrozen,
            "downstream_capture_throughput_fraction": safe_ratio(
                retained_positive + refrozen, incoming
            ),
            "gross_positive_to_solid_loss_ratio": safe_ratio(
                gross_positive, solid_loss
            ),
            "daily_local_signed_opportunity_m": daily_signed_index,
            "window_nonlocal_signed_index_m": window_signed_index,
            "post_coe_stage3_cold_opportunity_m": cold_index,
            "solid_source_limited_cold_index_m": solid_cold_index,
            "opportunity_overlap_cap_m": sum_field(window, "opportunity_overlap_cap_m"),
            "daily_signed_opportunity_to_positive_peak_deficit": safe_ratio(
                daily_signed_index, deficit
            ),
            "window_signed_index_to_positive_peak_deficit": safe_ratio(
                window_signed_index, deficit
            ),
            "post_coe_cold_index_to_positive_peak_deficit": safe_ratio(
                cold_index, deficit
            ),
            "solid_cold_index_to_positive_peak_deficit": safe_ratio(
                solid_cold_index, deficit
            ),
            "layer_liquid_store_endpoint_delta_m": window[-1][
                "layer_liquid_store_after_m"
            ]
            - window[0]["layer_liquid_store_before_m"],
            "sum_layer_liquid_store_day_delta_m": sum_field(
                window, "layer_liquid_store_day_delta_m"
            ),
            "producer_retained_minus_layer_day_delta_m": retained_positive
            - sum_field(window, "layer_liquid_store_day_delta_m"),
            "layer_refrozen_tag_endpoint_delta_m": window[-1][
                "layer_refrozen_tag_after_m"
            ]
            - window[0]["layer_refrozen_tag_before_m"],
            "sum_layer_refrozen_tag_day_delta_m": sum_field(
                window, "layer_refrozen_tag_day_delta_m"
            ),
            "mixed_signed_hour_days": sum(
                bool(row["mixed_signed_hour_day"]) for row in window
            ),
            "loss_on_dry_days_m": sum(
                row["snowpack_swe_loss_m"]
                for row in window
                if row["accumulation_m"] <= ZERO
                and row["rain_retained_m"] + row["rain_released_m"] <= ZERO
            ),
            "loss_on_snowfall_days_m": sum(
                row["snowpack_swe_loss_m"]
                for row in window
                if row["accumulation_m"] > ZERO
            ),
            "loss_on_rain_days_m": sum(
                row["snowpack_swe_loss_m"]
                for row in window
                if row["rain_retained_m"] + row["rain_released_m"] > ZERO
            ),
            "maximum_abs_daily_storage_closure_m": max(
                abs(row["storage_closure_residual_m"]) for row in window
            ),
            "maximum_abs_daily_accumulation_hourly_residual_m": max(
                abs(row["accumulation_hourly_residual_m"]) for row in window
            ),
            "maximum_abs_daily_handoff_closure_m": max(
                abs(row["handoff_closure_residual_m"]) for row in window
            ),
            "maximum_abs_daily_stage3_reconstructed_residual_m": max(
                abs(row["stage3_reconstructed_residual_m"]) for row in window
            ),
            "maximum_abs_daily_stage3_residual_difference_m": max(
                abs(row["stage3_residual_difference_m"]) for row in window
            ),
            "raw_melt_alias_difference_days": sum(
                abs(row["raw_melt_m"] - row["snowpack_swe_loss_m"]) > ZERO
                for row in window
            ),
            "accumulation_alias_difference_days": sum(
                operands_differ(
                    row["reported_accumulation_m"], row["accumulation_m"]
                )
                for row in window
            ),
            "gross_positive_alias_difference_days": sum(
                abs(
                    row["hourly"]["gross_positive_applied_m"]
                    - row["snowpack_swe_loss_m"]
                )
                > ZERO
                for row in window
            ),
            "top_level_handoff_identity_difference_days": sum(
                operands_differ(
                    row["top_level_routed_melt_m"], row["stage3_incoming_m"]
                )
                for row in window
            ),
            "top_level_routed_alias_difference_days": sum(
                operands_differ(
                    row["top_level_routed_melt_m"], row["stage3_routed_m"]
                )
                for row in window
            ),
            "retained_store_alias_difference_days": sum(
                abs(
                    row["retained_store_after_m"]
                    - row["stage3_retained_delta_m"]
                )
                > ZERO
                for row in window
            ),
            "omitted_retained_delta_residual_days": sum(
                abs(
                    row["stage3_incoming_m"]
                    - row["stage3_routed_m"]
                    - row["stage3_refrozen_m"]
                )
                > MASS_TOLERANCE_M
                for row in window
            ),
            "doubled_refreeze_residual_days": sum(
                abs(
                    row["stage3_incoming_m"]
                    - row["stage3_routed_m"]
                    - row["stage3_retained_delta_m"]
                    - 2.0 * row["stage3_refrozen_m"]
                )
                > MASS_TOLERANCE_M
                for row in window
            ),
        }
        annual.append(annual_row)
        for rank, event in enumerate(loss_events(window), start=1):
            events.append({"site": site, "water_year": water_year, "rank": rank, **event})
        for key, bucket in sorted(aggregate_hour_classes(window).items()):
            dimension, category = key.split(":", 1)
            hour_classes.append(
                {
                    "site": site,
                    "water_year": water_year,
                    "dimension": dimension,
                    "category": category,
                    **bucket,
                }
            )
    if not annual:
        raise RuntimeError(f"no eligible observed-peak windows for {site}")
    return annual, events, hour_classes, skipped_zero


def finite_median(rows: list[dict[str, Any]], field: str) -> float | None:
    values = [
        float(row[field])
        for row in rows
        if row[field] is not None and math.isfinite(float(row[field]))
    ]
    if not values:
        return None
    return statistics.median(values)


def summarize_sites(annual: list[dict[str, Any]]) -> list[dict[str, Any]]:
    summary = []
    fields = (
        "peak_swe_ratio",
        "peak_date_offset_days",
        "observed_minus_modeled_peak_m",
        "snowfall_m",
        "solid_pack_loss_m",
        "gross_positive_applied_coe_m",
        "negative_applied_coe_m",
        "stage3_incoming_m",
        "stage3_routed_m",
        "stage3_retained_positive_m",
        "stage3_retained_negative_m",
        "stage3_refrozen_m",
        "downstream_capture_throughput_fraction",
        "gross_positive_to_solid_loss_ratio",
        "daily_local_signed_opportunity_m",
        "window_nonlocal_signed_index_m",
        "post_coe_stage3_cold_opportunity_m",
        "solid_source_limited_cold_index_m",
        "opportunity_overlap_cap_m",
        "daily_signed_opportunity_to_positive_peak_deficit",
        "window_signed_index_to_positive_peak_deficit",
        "post_coe_cold_index_to_positive_peak_deficit",
        "solid_cold_index_to_positive_peak_deficit",
        "layer_liquid_store_endpoint_delta_m",
        "sum_layer_liquid_store_day_delta_m",
        "producer_retained_minus_layer_day_delta_m",
        "layer_refrozen_tag_endpoint_delta_m",
        "sum_layer_refrozen_tag_day_delta_m",
        "loss_on_dry_days_m",
        "loss_on_snowfall_days_m",
        "loss_on_rain_days_m",
    )
    for site in sorted({row["site"] for row in annual}):
        rows = [
            row
            for row in annual
            if row["site"] == site and not bool(row["right_censored"])
        ]
        if not rows:
            raise RuntimeError(f"no primary rows for {site}")
        result = {
            "site": site,
            "primary_water_year_count": len(rows),
            "primary_water_years": ";".join(str(row["water_year"]) for row in rows),
        }
        for field in fields:
            result[f"median_{field}"] = finite_median(rows, field)
        result["maximum_abs_daily_storage_closure_m"] = max(
            row["maximum_abs_daily_storage_closure_m"] for row in rows
        )
        result["maximum_abs_daily_accumulation_hourly_residual_m"] = max(
            row["maximum_abs_daily_accumulation_hourly_residual_m"] for row in rows
        )
        result["maximum_abs_daily_handoff_closure_m"] = max(
            row["maximum_abs_daily_handoff_closure_m"] for row in rows
        )
        result["maximum_abs_daily_stage3_reconstructed_residual_m"] = max(
            row["maximum_abs_daily_stage3_reconstructed_residual_m"] for row in rows
        )
        result["maximum_abs_daily_stage3_residual_difference_m"] = max(
            row["maximum_abs_daily_stage3_residual_difference_m"] for row in rows
        )
        result["primary_years_with_positive_peak_deficit"] = sum(
            row["observed_minus_modeled_peak_m"] > ZERO for row in rows
        )
        result["excluded_zero_incoming_ratio_years"] = sum(
            row["downstream_capture_throughput_fraction"] is None for row in rows
        )
        result["excluded_zero_solid_loss_ratio_years"] = sum(
            row["gross_positive_to_solid_loss_ratio"] is None for row in rows
        )
        result["excluded_nonpositive_peak_deficit_ratio_years"] = sum(
            row["daily_signed_opportunity_to_positive_peak_deficit"] is None
            for row in rows
        )
        summary.append(result)
    return summary


def apply_screens(site_summary: list[dict[str, Any]], annual: list[dict[str, Any]]) -> dict[str, Any]:
    downstream_sites = [
        row["site"]
        for row in site_summary
        if row["median_downstream_capture_throughput_fraction"] is not None
        and row["median_downstream_capture_throughput_fraction"] >= 0.5
    ]
    upstream_sites = [
        row["site"]
        for row in site_summary
        if row["median_gross_positive_to_solid_loss_ratio"] is not None
        and 0.8 <= row["median_gross_positive_to_solid_loss_ratio"] <= 1.2
    ]
    signed_material_sites = [
        row["site"]
        for row in site_summary
        if row["median_daily_signed_opportunity_to_positive_peak_deficit"]
        is not None
        and row["median_daily_signed_opportunity_to_positive_peak_deficit"] >= 0.25
    ]
    cold_descriptive_sites = [
        row["site"]
        for row in site_summary
        if row["median_post_coe_cold_index_to_positive_peak_deficit"] is not None
        and row["median_post_coe_cold_index_to_positive_peak_deficit"] >= 0.25
    ]
    primary = [row for row in annual if not row["right_censored"]]
    eligible_direction_rows = [
        row
        for row in primary
        if row["solid_pack_loss_m"] > ZERO
        and row["observed_minus_modeled_peak_m"] > ZERO
    ]
    positive_direction_rows = [
        row
        for row in eligible_direction_rows
        if row["daily_signed_opportunity_to_positive_peak_deficit"] is not None
        and row["daily_signed_opportunity_to_positive_peak_deficit"] > 0.0
    ]
    eligible_sites = {row["site"] for row in eligible_direction_rows}
    positive_sites = {row["site"] for row in positive_direction_rows}
    systemic_direction = bool(eligible_direction_rows) and (
        eligible_sites == {row["site"] for row in primary}
        and positive_sites == eligible_sites
        and len(positive_direction_rows) / len(eligible_direction_rows) >= 0.75
    )
    screens = {
        "upstream_ledger_localization": len(upstream_sites) >= 3,
        "downstream_capture_throughput_descriptive": len(downstream_sites) >= 3,
        "daily_local_signed_opportunity_material": len(signed_material_sites) >= 3,
        "daily_local_signed_opportunity_systemic": systemic_direction,
        "post_coe_cold_opportunity_descriptive": len(cold_descriptive_sites) >= 3,
    }
    signed_priority = (
        screens["daily_local_signed_opportunity_material"]
        and screens["daily_local_signed_opportunity_systemic"]
    )
    return {
        "interpretation": (
            "Only the daily-local signed opportunity can trigger the v2 quantitative "
            "priority rule. Cold-content and Stage-3 capture quantities are descriptive, "
            "feedback-free indices and are not simulated recovery."
        ),
        "site_sets": {
            "upstream_ledger_localization": upstream_sites,
            "downstream_capture_throughput_descriptive": downstream_sites,
            "daily_local_signed_opportunity_material": signed_material_sites,
            "post_coe_cold_opportunity_descriptive": cold_descriptive_sites,
        },
        "primary_site_year_count": len(primary),
        "eligible_direction_site_year_count": len(eligible_direction_rows),
        "positive_direction_site_year_count": len(positive_direction_rows),
        "screens": screens,
        "quantitative_candidate_verdict": (
            "PREEXPORT_ENTHALPY_OR_SIGNED_HOUR_PRIORITY"
            if signed_priority
            else "UPSTREAM_GENERATION_PRIORITY"
            if screens["upstream_ledger_localization"]
            else "UNRESOLVED"
        ),
        "authority_limit": (
            "This candidate assumes static real-consumer evidence excludes Stage-3 as a "
            "causal SWE-loss path. Final adjudication must verify that fact. No candidate "
            "authorizes signed netting or another production correction."
        ),
    }


def sanitized_environment(trace: Path, selectors: dict[str, str]) -> tuple[dict[str, str], list[str], dict[str, str]]:
    removed = sorted(key for key in os.environ if key.startswith("OPENWEPP_"))
    environment = {
        key: value for key, value in os.environ.items() if not key.startswith("OPENWEPP_")
    }
    effective = dict(selectors)
    effective["OPENWEPP_R7H_SNOW_TRACE_PATH"] = str(trace.resolve())
    environment.update(effective)
    observed = {key: value for key, value in environment.items() if key.startswith("OPENWEPP_")}
    if observed != effective:
        raise RuntimeError("OPENWEPP environment sanitizer failed")
    return environment, removed, effective


def execute_site(lane: Any, selectors: dict[str, str]) -> dict[str, Any]:
    site = lane.lane_id
    source = SOURCE_FIXTURES / site
    fixture = OUTPUT / "fixtures" / site
    shutil.copytree(source, fixture)
    source_manifest = file_manifest(source)
    copied_manifest = file_manifest(fixture)
    if source_manifest != copied_manifest:
        raise RuntimeError(f"fixture copy differs for {site}")
    run_dir = OUTPUT / "runs" / site
    run_dir.mkdir(parents=True)
    stem = f"{site}-adjudication"
    runfile = run_dir / f"{stem}.run"
    trace = run_dir / f"{stem}.snow.jsonl"
    source_stem = W1.eb04r.legacy.observed_harness.discover_run_stem(fixture)
    W1.eb04r.legacy.observed_harness.write_runfile(
        runfile, fixture, source_stem, run_dir, stem
    )
    command = W1.eb04r.legacy.observed_harness.cli_command(
        BINARY, fixture, runfile, run_dir, "direct-production-executor"
    )
    environment, removed, effective = sanitized_environment(trace, selectors)
    completed = subprocess.run(
        command,
        cwd=REPO,
        env=environment,
        text=True,
        capture_output=True,
        check=False,
    )
    stdout = run_dir / "stdout.txt"
    stderr = run_dir / "stderr.txt"
    stdout.write_text(completed.stdout, encoding="utf-8")
    stderr.write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise RuntimeError(f"run failed for {site}: {completed.stderr[-2000:]}")
    output_files = {}
    for path in sorted(candidate for candidate in run_dir.iterdir() if candidate.is_file()):
        output_files[path.name] = {
            "path": relative(path),
            "sha256": sha256(path),
            "size_bytes": path.stat().st_size,
        }
    predecessor_dir = PREDECESSOR_RUNS / site
    predecessor_wat = predecessor_dir / f"{site}-baseline_replay.wat.parquet"
    predecessor_hbp = predecessor_dir / f"{site}-baseline_replay.hbp"
    current_wat = run_dir / f"{stem}.wat.parquet"
    current_hbp = run_dir / f"{stem}.hbp"
    return {
        "site": site,
        "argv": [str(value) for value in command],
        "returncode": completed.returncode,
        "removed_openwepp_key_names": removed,
        "effective_openwepp_environment": effective,
        "source_fixture_manifest": source_manifest,
        "copied_fixture_manifest": copied_manifest,
        "observation": {
            "path": relative(Path(lane.observation_file)),
            "sha256": sha256(Path(lane.observation_file)),
            "role": "DIAGNOSTIC_ONLY",
        },
        "outputs": output_files,
        "protected_output_identity": {
            "wat_exact": sha256(current_wat) == sha256(predecessor_wat),
            "wat_current_sha256": sha256(current_wat),
            "wat_predecessor_sha256": sha256(predecessor_wat),
            "hbp_exact": sha256(current_hbp) == sha256(predecessor_hbp),
            "hbp_current_sha256": sha256(current_hbp),
            "hbp_predecessor_sha256": sha256(predecessor_hbp),
            "pass_output": "not emitted by this direct hillslope surface",
        },
    }


def execute(reanalyze_existing: bool) -> None:
    if OUTPUT.exists() and not reanalyze_existing:
        raise RuntimeError(f"refusing to overwrite {OUTPUT}")
    if not OUTPUT.exists() and reanalyze_existing:
        raise RuntimeError(f"cannot reanalyze missing output namespace: {OUTPUT}")
    freeze = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    if freeze["status"] != "frozen_before_result_execution":
        raise RuntimeError("analysis freeze is not active")
    if not BINARY.is_file():
        raise RuntimeError(f"release binary is missing: {BINARY}")
    W1_LANES = W1.selected_lanes()
    frozen_sites = [row["site"] for row in freeze["sites"]]
    if [lane.lane_id for lane in W1_LANES] != frozen_sites:
        raise RuntimeError("selected lane cohort differs from freeze")
    for frozen in freeze["sites"]:
        observed = file_manifest(SOURCE_FIXTURES / frozen["site"])["manifest_sha256"]
        if observed != frozen["fixture_manifest_sha256"]:
            raise RuntimeError(f"source fixture hash differs for {frozen['site']}")
    if reanalyze_existing:
        prior_receipt = json.loads(
            (OUTPUT / "execution-receipt.json").read_text(encoding="utf-8")
        )
        receipts = prior_receipt["sites"]
        for site, site_receipt in receipts.items():
            for output in site_receipt["outputs"].values():
                path = REPO / output["path"]
                if (
                    path.stat().st_size != output["size_bytes"]
                    or sha256(path) != output["sha256"]
                ):
                    raise RuntimeError(f"retained run output identity differs: {path}")
    else:
        OUTPUT.mkdir(parents=True)
        receipts = {}
        with ThreadPoolExecutor(max_workers=2) as executor:
            futures = {
                executor.submit(execute_site, lane, freeze["selectors"]): lane.lane_id
                for lane in W1_LANES
            }
            for future in as_completed(futures):
                site = futures[future]
                receipts[site] = future.result()

    annual: list[dict[str, Any]] = []
    events: list[dict[str, Any]] = []
    hour_classes: list[dict[str, Any]] = []
    compatibility: dict[str, Any] = {}
    skipped_zero: dict[str, list[int]] = {}
    for lane in W1_LANES:
        site = lane.lane_id
        run_dir = OUTPUT / "runs" / site
        trace = run_dir / f"{site}-adjudication.snow.jsonl"
        predecessor = PREDECESSOR_RUNS / site / f"{site}-baseline_replay.snow.jsonl"
        dates = climate_dates(climate_file(OUTPUT / "fixtures" / site))
        reduced, trace_compatibility = compare_and_reduce_trace(
            trace, predecessor, dates
        )
        compatibility[site] = trace_compatibility
        site_annual, site_events, site_classes, skipped = analyze_windows(
            site, reduced, Path(lane.observation_file)
        )
        annual.extend(site_annual)
        events.extend(site_events)
        hour_classes.extend(site_classes)
        skipped_zero[site] = skipped

    site_summary = summarize_sites(annual)
    screens = apply_screens(site_summary, annual)
    results = {
        "schema_version": 1,
        "evidence_mode": (
            "Ran: exact-current direct-production four-site replay and independent "
            "schema-v4 linked-ledger reconstruction"
        ),
        "characterization_only": True,
        "diagnostic_screen_status": "ASSUMED_FOR_EXECUTION",
        "freeze_sha256": sha256(FREEZE_PATH),
        "primary_window_count": sum(not row["right_censored"] for row in annual),
        "right_censored_window_count": sum(row["right_censored"] for row in annual),
        "site_summary": site_summary,
        "annual": annual,
        "screens": screens,
        "trace_compatibility": compatibility,
        "zero_observed_peak_years_skipped": skipped_zero,
        "claim_limits": freeze["claim_limits"],
    }
    event_results = {
        "schema_version": 1,
        "diagnostic_screen_status": "ASSUMED_FOR_EXECUTION",
        "primary_event_threshold_m_per_day": PRIMARY_EVENT_THRESHOLD_M,
        "events": events,
        "hour_class_summaries": hour_classes,
        "claim_limit": (
            "A/B/C/D are empirical melt-depth contributions and event opportunity "
            "bounds do not include state feedback."
        ),
    }
    results_path = OUTPUT / "results/cross-fixture-results.json"
    event_path = OUTPUT / "results/event-attribution.json"
    write_json(results_path, results)
    write_json(event_path, event_results)
    annual_csv_rows = [
        {key: value for key, value in row.items()}
        for row in annual
    ]
    write_csv(OUTPUT / "tables/annual-metrics.csv", annual_csv_rows)
    write_csv(OUTPUT / "tables/site-summary.csv", site_summary)
    write_csv(OUTPUT / "tables/event-attribution.csv", events)
    write_csv(OUTPUT / "tables/hour-class-summary.csv", hour_classes)
    receipt = {
        "schema_version": 1,
        "status": "EXECUTED",
        "analysis_mode": (
            "retained_exact_run_reanalysis_after_pre_v4_projection_fix"
            if reanalyze_existing
            else "fresh_exact_cli_execution"
        ),
        "analysis_correction": (
            {
                "invalidated_result_surface": (
                    "compatibility booleans, snowfall/storage proof lineage, and "
                    "top-level-routed anti-alias counters"
                ),
                "model_outputs_rerun": False,
                "scientific_operators_changed": False,
                "corrections": [
                    (
                        "Project expected v4 top-level and nested hourly additions plus "
                        "the v3-to-v4 schema label before comparing pre-v4 fields."
                    ),
                    (
                        "Reconstruct snowfall from hourly snowfall_swe_m for storage and "
                        "annual evidence; compare reported accumulation only afterward."
                    ),
                    (
                        "Compare top-level CoE routed melt to Stage-3 routed liquid for "
                        "the required anti-alias, while separately confirming the exact "
                        "top-level-to-incoming handoff."
                    ),
                ],
            }
            if reanalyze_existing
            else None
        ),
        "scaffold_commit": SCAFFOLD_COMMIT,
        "source_head": command_output(["git", "rev-parse", "HEAD"]),
        "binary": {
            "path": relative(BINARY),
            "sha256": sha256(BINARY),
            "size_bytes": BINARY.stat().st_size,
            "build_command": "cargo build --release -p openwepp-runner --bin openwepp-cli-hill",
        },
        "freeze": {"path": relative(FREEZE_PATH), "sha256": sha256(FREEZE_PATH)},
        "tool": {"path": relative(Path(__file__)), "sha256": sha256(Path(__file__))},
        "sites": {site: receipts[site] for site in sorted(receipts)},
        "results": {
            "cross_fixture": {
                "path": relative(results_path),
                "sha256": sha256(results_path),
            },
            "event_attribution": {
                "path": relative(event_path),
                "sha256": sha256(event_path),
            },
        },
    }
    write_json(OUTPUT / "execution-receipt.json", receipt)
    print(f"wrote {relative(results_path)}")
    print(f"wrote {relative(event_path)}")
    print(f"wrote {relative(OUTPUT / 'execution-receipt.json')}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--execute", action="store_true")
    group.add_argument("--reanalyze-existing", action="store_true")
    args = parser.parse_args()
    execute(reanalyze_existing=args.reanalyze_existing)
    return 0


W1 = load_module("snow_mass_adjudication_w1", W1_TOOL)


if __name__ == "__main__":
    raise SystemExit(main())
