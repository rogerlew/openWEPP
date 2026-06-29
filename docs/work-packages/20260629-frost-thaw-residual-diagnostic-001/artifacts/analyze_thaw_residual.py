#!/usr/bin/env python3
"""Bucket post-residue Sleepers frost timing residuals by thaw mechanism."""

from __future__ import annotations

import csv
import datetime as dt
import json
import math
from pathlib import Path
from typing import Any

import pyarrow.parquet as pq


REPO_ROOT = Path(__file__).resolve().parents[4]
PACKAGE = REPO_ROOT / "docs/work-packages/20260629-frost-thaw-residual-diagnostic-001"
ARTIFACTS = PACKAGE / "artifacts"
STEP3 = REPO_ROOT / "docs/work-packages/20260629-frost-step3-residue-parameterization-001"
STEP3_ARTIFACTS = STEP3 / "artifacts"
RUN_ROOT = REPO_ROOT / "target/frost_step3_residue_parameterization/runs"

PRIMARY_MATERIAL_THRESHOLD_M = 0.02
THRESHOLD_SWEEP_M = [0.0, 0.001, 0.0025, 0.005, 0.01, 0.02, 0.05, 0.10]
RETREAT_EPSILON_M = 0.001
PRIMARY_SNOW_BURIED_THRESHOLD_M = 0.10
SNOW_DEPTH_SWEEP_M = [0.05, 0.10, 0.20]


def main() -> int:
    payload = build_payload()
    write_json(ARTIFACTS / "thaw_residual_buckets.json", payload)
    write_cell_csv(ARTIFACTS / "thaw_residual_buckets.csv", payload)
    write_window_csv(ARTIFACTS / "thaw_residual_daily_windows.csv", payload)
    (ARTIFACTS / "thaw_residual_diagnostic.md").write_text(
        render_markdown(payload), encoding="utf-8"
    )
    return 0


def build_payload() -> dict[str, Any]:
    step3 = json.loads(
        (STEP3_ARTIFACTS / "residue_parameterization_diagnostic.json").read_text(
            encoding="utf-8"
        )
    )
    sites = []
    all_thaw_cells = []
    all_early_cells = []
    for site in step3["sites"]:
        site_id = site["site_id"]
        run_dir = RUN_ROOT / site_id / "seasonal_dec"
        wat_path = run_dir / f"{site_id}.wat.parquet"
        trace_path = run_dir / "frost_trace.jsonl"
        report_path = STEP3_ARTIFACTS / "site_reports" / f"{site_id}.seasonal_dec.comparison_report.json"
        report = json.loads(report_path.read_text(encoding="utf-8"))
        daily = load_daily_rows(wat_path, trace_path)
        snow_obs = snow_observation_by_date(report)
        attach_snow_observations(daily, snow_obs)
        metrics = {
            int(row["water_year"]): row
            for row in report["metrics"]["seasonal_metrics"]
        }

        thaw_cells = []
        early_cells = []
        for cell in site["comparison"]["baseline_candidate_cells_after_seasonal"]:
            if cell["seasonal_attribution"] != "candidate-frost-model-defect":
                continue
            wy = int(cell["water_year"])
            seasonal = metrics[wy]
            if cell["signature"] == "thaw":
                summary = analyze_thaw_cell(site_id, cell, seasonal, daily)
                thaw_cells.append(summary)
                all_thaw_cells.append(summary)
            elif cell["signature"] == "onset":
                summary = analyze_early_onset_cell(site_id, cell, seasonal, daily)
                early_cells.append(summary)
                all_early_cells.append(summary)

        sites.append(
            {
                "site_id": site_id,
                "source_run_dir": str(run_dir.relative_to(REPO_ROOT)),
                "wat_path": str(wat_path.relative_to(REPO_ROOT)),
                "trace_path": str(trace_path.relative_to(REPO_ROOT)),
                "comparison_report_path": str(report_path.relative_to(REPO_ROOT)),
                "thaw_late_cells": thaw_cells,
                "early_onset_cells": early_cells,
            }
        )

    return {
        "schema": "frost-thaw-residual-diagnostic-v1",
        "evidence_mode": "Ran",
        "source": {
            "step3_diagnostic": str(
                (STEP3_ARTIFACTS / "residue_parameterization_diagnostic.json").relative_to(
                    REPO_ROOT
                )
            ),
            "run_root": str(RUN_ROOT.relative_to(REPO_ROOT)),
            "thaw_detector": "tools/snowfreeze_observed/observed_harness.py:1117-1123",
            "detector_semantics": "modeled_thaw is the last observation day with modeled_frdp_m > 0.0",
        },
        "classification": {
            "primary_material_threshold_m": PRIMARY_MATERIAL_THRESHOLD_M,
            "primary_snow_buried_threshold_m": PRIMARY_SNOW_BURIED_THRESHOLD_M,
            "snow_depth_sweep_m": SNOW_DEPTH_SWEEP_M,
            "retreat_epsilon_m": RETREAT_EPSILON_M,
            "threshold_status": (
                "diagnostic only; not a proposed detector threshold and not tuned to "
                "match observed dates"
            ),
        },
        "sites": sites,
        "aggregate": aggregate(all_thaw_cells, all_early_cells),
        "threshold_sensitivity": threshold_sensitivity(all_thaw_cells),
        "snow_depth_sensitivity": snow_depth_sensitivity(all_thaw_cells),
        "routing_recommendation": routing_recommendation(all_thaw_cells),
    }


def snow_observation_by_date(report: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        row["date"]: row
        for row in report.get("metrics", {}).get("snow_depth_residuals", [])
        if row.get("date")
    }


def attach_snow_observations(
    daily: dict[dt.date, dict[str, Any]], snow_obs: dict[str, dict[str, Any]]
) -> None:
    for date, row in daily.items():
        obs = snow_obs.get(date.isoformat())
        if obs is None:
            row["observed_snow_depth_m"] = None
            row["snow_depth_residual_m"] = None
            continue
        row["observed_snow_depth_m"] = optional_f64(obs.get("observed_snow_depth_m"))
        row["snow_depth_residual_m"] = optional_f64(obs.get("residual_m"))


def load_daily_rows(wat_path: Path, trace_path: Path) -> dict[dt.date, dict[str, Any]]:
    wat = pq.read_table(wat_path).to_pylist()
    trace_rows = [
        json.loads(line)
        for line in trace_path.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]
    if len(trace_rows) != len(wat) * 2:
        raise RuntimeError(
            f"expected two frost trace rows per WAT day for {wat_path}, "
            f"observed {len(trace_rows)} trace rows and {len(wat)} WAT rows"
        )

    daily: dict[dt.date, dict[str, Any]] = {}
    previous_snow_water_m: float | None = None
    for index, wat_row in enumerate(wat):
        trace = trace_rows[index * 2 + 1]
        date = date_from_wat_row(wat_row)
        snow_water_m = mm_to_m(wat_row.get("Snow-Water"))
        snow_depth_m = value_or(mm_to_m(wat_row.get("Snow-Depth")), f64(trace.get("snow_depth_m")))
        snow_conductivity_w_m_k = f64(trace.get("snow_conductivity_w_m_k"))
        daily[date] = {
            "date": date.isoformat(),
            "water_year": int(wat_row["water_year"]),
            "wat_frdp_m": mm_to_m(wat_row.get("frdp")),
            "trace_frdp_m": f64(trace.get("final_frdp_m")),
            "prior_frdp_m": f64(trace.get("prior_frdp_m")),
            "frozen_water_m": mm_to_m(wat_row.get("frozwt")),
            "trace_ws_frz_m": f64(trace.get("final_ws_frz_m")),
            "trace_thdp_m": f64(trace.get("final_thdp_m")),
            "liquid_delta_m": f64(trace.get("frwatc_net_liquid_delta_m")),
            "snow_depth_m": snow_depth_m,
            "snow_water_m": snow_water_m,
            "snow_water_delta_m": (
                None if previous_snow_water_m is None else snow_water_m - previous_snow_water_m
            ),
            "precip_m": mm_to_m(wat_row.get("P")),
            "rain_melt_m": mm_to_m(wat_row.get("RM")),
            "runoff_m": mm_to_m(wat_row.get("Q")),
            "soil_water_m": mm_to_m(wat_row.get("Total-Soil")),
            "residue_depth_m": f64(trace.get("residue_depth_m")),
            "snow_conductivity_w_m_k": snow_conductivity_w_m_k,
            "snow_resistance_m2_k_w": (
                snow_depth_m / snow_conductivity_w_m_k
                if snow_conductivity_w_m_k > 0.0
                else None
            ),
            "max_air_temp_c": max_or_none(trace.get("hour_air_temperature_c")),
            "max_surface_temp_c": max_or_none(trace.get("hour_surface_temp_c")),
            "min_surface_temp_c": min_or_none(trace.get("hour_surface_temp_c")),
            "mean_qsrf_w_m2": mean_or_none(trace.get("hour_qsrf_w_m2")),
            "mean_quf_w_m2": mean_or_none(trace.get("hour_quf_w_m2")),
            "max_qsrf_w_m2": max_or_none(trace.get("hour_qsrf_w_m2")),
            "max_quf_w_m2": max_or_none(trace.get("hour_quf_w_m2")),
            "max_tilled_frozen_depth_m": max_or_none(trace.get("hour_tilled_frozen_depth_m")),
            "max_untilled_frozen_depth_m": max_or_none(trace.get("hour_untilled_frozen_depth_m")),
            "max_fine_slfsd_m": max_or_none(trace.get("final_fine_slfsd_m")),
            "max_fine_slsic_m": max_or_none(trace.get("final_fine_slsic_m")),
        }
        previous_snow_water_m = snow_water_m

    ordered_dates = sorted(daily)
    for index, date in enumerate(ordered_dates):
        row = daily[date]
        next_row = daily[ordered_dates[index + 1]] if index + 1 < len(ordered_dates) else None
        next_frdp = None if next_row is None else next_row["trace_frdp_m"]
        row["next_frdp_m"] = next_frdp
        row["retreat_next_m"] = (
            None if next_frdp is None else row["trace_frdp_m"] - next_frdp
        )
    return daily


def analyze_thaw_cell(
    site_id: str,
    cell: dict[str, Any],
    seasonal: dict[str, Any],
    daily: dict[dt.date, dict[str, Any]],
) -> dict[str, Any]:
    start = parse_date(seasonal["observed_thaw_date"])
    end = parse_date(seasonal["modeled_thaw_date"])
    rows = window_rows(daily, start, end)
    stats = summarize_rows(rows)
    bucket = classify_thaw(stats)
    snow_route = classify_snow_route(stats, bucket["bucket"])
    return {
        "cell_id": f"{site_id}:{cell['water_year']}:thaw",
        "site_id": site_id,
        "water_year": int(cell["water_year"]),
        "signature": "thaw",
        "observed_thaw_date": seasonal["observed_thaw_date"],
        "modeled_thaw_date": seasonal["modeled_thaw_date"],
        "thaw_residual_days": seasonal["thaw_residual_days"],
        "observed_onset_date": seasonal["observed_onset_date"],
        "modeled_onset_date": seasonal["modeled_onset_date"],
        "bucket": bucket["bucket"],
        "bucket_label": bucket["label"],
        "bucket_reason": bucket["reason"],
        "snow_route": snow_route["route"],
        "snow_route_label": snow_route["label"],
        "snow_route_reason": snow_route["reason"],
        "window": stats,
        "daily_rows": rows,
    }


def analyze_early_onset_cell(
    site_id: str,
    cell: dict[str, Any],
    seasonal: dict[str, Any],
    daily: dict[dt.date, dict[str, Any]],
) -> dict[str, Any]:
    start = parse_date(seasonal["modeled_onset_date"])
    end = parse_date(seasonal["observed_onset_date"])
    rows = window_rows(daily, start, end)
    stats = summarize_rows(rows)
    if stats["max_frdp_m"] <= PRIMARY_MATERIAL_THRESHOLD_M:
        bucket = "H2"
        label = "tiny early-onset tail"
        reason = "modeled frost before observed onset is below the diagnostic material floor"
    else:
        bucket = "EARLY-ONSET-MATERIAL"
        label = "material early freeze"
        reason = (
            "modeled frost before observed onset is material; this is a distinct "
            "onset mechanism, not the thaw-late persistence mechanism"
        )
    return {
        "cell_id": f"{site_id}:{cell['water_year']}:onset",
        "site_id": site_id,
        "water_year": int(cell["water_year"]),
        "signature": "onset",
        "observed_onset_date": seasonal["observed_onset_date"],
        "modeled_onset_date": seasonal["modeled_onset_date"],
        "onset_residual_days": seasonal["onset_residual_days"],
        "observed_thaw_date": seasonal["observed_thaw_date"],
        "modeled_thaw_date": seasonal["modeled_thaw_date"],
        "bucket": bucket,
        "bucket_label": label,
        "bucket_reason": reason,
        "window": stats,
        "daily_rows": rows,
    }


def classify_thaw(stats: dict[str, Any]) -> dict[str, str]:
    if stats["max_frdp_m"] <= PRIMARY_MATERIAL_THRESHOLD_M:
        return {
            "bucket": "H2",
            "label": "tiny-tail / detection artifact",
            "reason": (
                "maximum modeled frost depth after observed thaw is below the "
                f"diagnostic material floor ({PRIMARY_MATERIAL_THRESHOLD_M} m)"
            ),
        }
    if stats["warm_surface_stalled_days"] > 0:
        return {
            "bucket": "H1b",
            "label": "state-machine thaw asymmetry",
            "reason": (
                "material frost persists through days with surface temperature "
                "above freezing and no next-day top-front retreat"
            ),
        }
    if stats["cold_surface_warm_wet_stalled_days"] > 0:
        return {
            "bucket": "H1a",
            "label": "missing wet/advective thaw energy",
            "reason": (
                "material frost persists through warm/rain/melt days while the "
                "modeled surface remains at or below freezing"
            ),
        }
    if stats["warm_wet_material_days"] > 0:
        return {
            "bucket": "H1a",
            "label": "missing wet/advective thaw energy",
            "reason": (
                "material frost persists through warm/rain/melt days; wet heat is "
                "not represented in the current dry surface heat path"
            ),
        }
    return {
        "bucket": "H1b",
        "label": "state-machine thaw asymmetry",
        "reason": (
            "material frost persists without a tiny tail; no wet-event signature "
            "dominates, so route to front-retreat/state-machine inspection"
        ),
    }


def summarize_rows(rows: list[dict[str, Any]]) -> dict[str, Any]:
    material = [row for row in rows if row["trace_frdp_m"] > PRIMARY_MATERIAL_THRESHOLD_M]
    warm_wet = [row for row in material if is_warm_or_wet(row)]
    snow_buried_warm_wet = [
        row
        for row in warm_wet
        if row["snow_depth_m"] >= PRIMARY_SNOW_BURIED_THRESHOLD_M
    ]
    snow_free_warm_wet = [
        row
        for row in warm_wet
        if row["snow_depth_m"] < PRIMARY_SNOW_BURIED_THRESHOLD_M
    ]
    cold_surface_warm_wet_stalled = [
        row
        for row in warm_wet
        if (row["max_surface_temp_c"] or -999.0) <= 0.0
        and not retreats_next(row)
    ]
    warm_surface_stalled = [
        row
        for row in material
        if (row["max_surface_temp_c"] or -999.0) > 0.0
        and not retreats_next(row)
    ]
    retreat_days = [row for row in material if retreats_next(row)]
    paired_rows = [row for row in rows if row["observed_snow_depth_m"] is not None]
    paired_warm_wet = [
        row for row in warm_wet if row["observed_snow_depth_m"] is not None
    ]
    paired_snow_buried = [
        row
        for row in snow_buried_warm_wet
        if row["observed_snow_depth_m"] is not None
    ]
    paired_snow_free = [
        row
        for row in snow_free_warm_wet
        if row["observed_snow_depth_m"] is not None
    ]
    return {
        "start_date": rows[0]["date"] if rows else None,
        "end_date": rows[-1]["date"] if rows else None,
        "days": len(rows),
        "max_frdp_m": max_value(rows, "trace_frdp_m"),
        "median_frdp_m": median([row["trace_frdp_m"] for row in rows]),
        "final_window_frdp_m": rows[-1]["trace_frdp_m"] if rows else None,
        "max_frozen_water_m": max_value(rows, "frozen_water_m"),
        "max_trace_ws_frz_m": max_value(rows, "trace_ws_frz_m"),
        "max_snow_depth_m": max_value(rows, "snow_depth_m"),
        "min_snow_depth_m": min_value(rows, "snow_depth_m"),
        "median_snow_depth_m": median([row["snow_depth_m"] for row in rows]),
        "max_snow_water_m": max_value(rows, "snow_water_m"),
        "snow_water_delta_total_m": (
            rows[-1]["snow_water_m"] - rows[0]["snow_water_m"] if rows else None
        ),
        "snow_water_positive_delta_m": sum(
            max(row["snow_water_delta_m"] or 0.0, 0.0) for row in rows
        ),
        "snow_water_negative_delta_m": sum(
            min(row["snow_water_delta_m"] or 0.0, 0.0) for row in rows
        ),
        "total_precip_m": sum_values(rows, "precip_m"),
        "total_rain_melt_m": sum_values(rows, "rain_melt_m"),
        "total_runoff_m": sum_values(rows, "runoff_m"),
        "paired_snow_obs_days": len(paired_rows),
        "mean_snow_depth_residual_m": mean(
            [
                row["snow_depth_residual_m"]
                for row in paired_rows
            ]
        ),
        "max_snow_depth_residual_m": max_value(
            paired_rows,
            "snow_depth_residual_m",
        ),
        "paired_model_snow_depth_delta_m": paired_delta(paired_rows, "snow_depth_m"),
        "paired_observed_snow_depth_delta_m": paired_delta(
            paired_rows, "observed_snow_depth_m"
        ),
        "paired_warm_wet_snow_obs_days": len(paired_warm_wet),
        "mean_warm_wet_snow_depth_residual_m": mean(
            [row["snow_depth_residual_m"] for row in paired_warm_wet]
        ),
        "snow_buried_paired_snow_obs_days": len(paired_snow_buried),
        "snow_buried_mean_snow_depth_residual_m": mean(
            [row["snow_depth_residual_m"] for row in paired_snow_buried]
        ),
        "snow_free_paired_snow_obs_days": len(paired_snow_free),
        "snow_free_mean_snow_depth_residual_m": mean(
            [row["snow_depth_residual_m"] for row in paired_snow_free]
        ),
        "min_residue_depth_m": min_value(rows, "residue_depth_m"),
        "max_residue_depth_m": max_value(rows, "residue_depth_m"),
        "max_air_temp_c": max_value(rows, "max_air_temp_c"),
        "max_surface_temp_c": max_value(rows, "max_surface_temp_c"),
        "min_surface_temp_c": min_value(rows, "min_surface_temp_c"),
        "mean_qsrf_w_m2": mean([row["mean_qsrf_w_m2"] for row in rows if row["mean_qsrf_w_m2"] is not None]),
        "mean_quf_w_m2": mean([row["mean_quf_w_m2"] for row in rows if row["mean_quf_w_m2"] is not None]),
        "warm_wet_mean_qsrf_w_m2": mean(
            [row["mean_qsrf_w_m2"] for row in warm_wet if row["mean_qsrf_w_m2"] is not None]
        ),
        "warm_wet_mean_quf_w_m2": mean(
            [row["mean_quf_w_m2"] for row in warm_wet if row["mean_quf_w_m2"] is not None]
        ),
        "snow_buried_mean_qsrf_w_m2": mean(
            [
                row["mean_qsrf_w_m2"]
                for row in snow_buried_warm_wet
                if row["mean_qsrf_w_m2"] is not None
            ]
        ),
        "snow_buried_mean_quf_w_m2": mean(
            [
                row["mean_quf_w_m2"]
                for row in snow_buried_warm_wet
                if row["mean_quf_w_m2"] is not None
            ]
        ),
        "snow_buried_mean_resistance_m2_k_w": mean(
            [
                row["snow_resistance_m2_k_w"]
                for row in snow_buried_warm_wet
                if row["snow_resistance_m2_k_w"] is not None
            ]
        ),
        "snow_buried_total_runoff_m": sum_values(snow_buried_warm_wet, "runoff_m"),
        "snow_buried_snow_water_delta_m": sum(
            row["snow_water_delta_m"] or 0.0 for row in snow_buried_warm_wet
        ),
        "snow_free_mean_qsrf_w_m2": mean(
            [
                row["mean_qsrf_w_m2"]
                for row in snow_free_warm_wet
                if row["mean_qsrf_w_m2"] is not None
            ]
        ),
        "snow_free_mean_quf_w_m2": mean(
            [
                row["mean_quf_w_m2"]
                for row in snow_free_warm_wet
                if row["mean_quf_w_m2"] is not None
            ]
        ),
        "snow_free_total_runoff_m": sum_values(snow_free_warm_wet, "runoff_m"),
        "snow_free_snow_water_delta_m": sum(
            row["snow_water_delta_m"] or 0.0 for row in snow_free_warm_wet
        ),
        "material_days": len(material),
        "warm_wet_material_days": len(warm_wet),
        "snow_buried_warm_wet_material_days": len(snow_buried_warm_wet),
        "snow_free_warm_wet_material_days": len(snow_free_warm_wet),
        "snow_buried_fraction_of_warm_wet_material_days": (
            len(snow_buried_warm_wet) / len(warm_wet) if warm_wet else None
        ),
        "cold_surface_warm_wet_stalled_days": len(cold_surface_warm_wet_stalled),
        "warm_surface_stalled_days": len(warm_surface_stalled),
        "retreat_days": len(retreat_days),
        "largest_next_day_retreat_m": max(
            [row["retreat_next_m"] for row in rows if row["retreat_next_m"] is not None],
            default=None,
        ),
        "largest_next_day_growth_m": abs(
            min(
                [row["retreat_next_m"] for row in rows if row["retreat_next_m"] is not None],
                default=0.0,
            )
        ),
    }


def classify_snow_route(stats: dict[str, Any], primary_bucket: str) -> dict[str, str]:
    if primary_bucket == "H2":
        return {
            "route": "H2-TINY-TAIL",
            "label": "tiny-tail detector artifact",
            "reason": "primary bucket is H2; snow-depth control is not needed",
        }
    warm_wet = stats["warm_wet_material_days"]
    buried = stats["snow_buried_warm_wet_material_days"]
    free = stats["snow_free_warm_wet_material_days"]
    if warm_wet == 0:
        return {
            "route": "NO-WARM-WET-EVIDENCE",
            "label": "no warm/wet material window",
            "reason": "material frost persists without warm/wet days in the extracted window",
        }
    buried_fraction = buried / warm_wet
    if buried_fraction >= 0.60:
        route = snow_buried_subroute(stats)
        route_reason = (
            "modeled SWE gains or nearly balances across the carried-frost "
            "window while snow remains insulating"
            if route == "SNOW-BURIED-ACCUMULATION"
            else "modeled SWE is net-losing across the carried-frost window, "
            "but the snowpack remains insulating through warm/wet material-frost days"
        )
        return {
            "route": route,
            "label": "snow-buried persistence",
            "reason": (
                f"{buried}/{warm_wet} warm/wet material days have modeled snow "
                f"depth >= {PRIMARY_SNOW_BURIED_THRESHOLD_M} m; persistence is "
                "controlled by snow insulation before any Qwet-class soil heat term; "
                f"{route_reason}"
            ),
        }
    if free / warm_wet >= 0.60:
        return {
            "route": "SNOW-FREE-PERSISTENT",
            "label": "snow-free material persistence",
            "reason": (
                f"{free}/{warm_wet} warm/wet material days have modeled snow "
                f"depth < {PRIMARY_SNOW_BURIED_THRESHOLD_M} m; Qwet/wet-heat remains plausible"
            ),
        }
    return {
        "route": "MIXED-SNOW-CONTROL",
        "label": "mixed snow-buried and snow-free persistence",
        "reason": (
            f"{buried}/{warm_wet} warm/wet material days are snow-buried and "
            f"{free}/{warm_wet} are near snow-free"
        ),
    }


def snow_buried_subroute(stats: dict[str, Any]) -> str:
    total_delta = stats.get("snow_water_delta_total_m")
    positive = stats.get("snow_water_positive_delta_m")
    negative = abs(stats.get("snow_water_negative_delta_m") or 0.0)
    if total_delta is not None and (
        total_delta > 0.005 or (total_delta > -0.005 and positive >= negative)
    ):
        return "SNOW-BURIED-ACCUMULATION"
    return "SNOW-BURIED-UNDER-MELT"


def aggregate(thaw_cells: list[dict[str, Any]], early_cells: list[dict[str, Any]]) -> dict[str, Any]:
    thaw_counts: dict[str, int] = {}
    for cell in thaw_cells:
        thaw_counts[cell["bucket"]] = thaw_counts.get(cell["bucket"], 0) + 1
    early_counts: dict[str, int] = {}
    for cell in early_cells:
        early_counts[cell["bucket"]] = early_counts.get(cell["bucket"], 0) + 1
    return {
        "thaw_late_cell_count": len(thaw_cells),
        "early_onset_cell_count": len(early_cells),
        "all_candidate_cell_count": len(thaw_cells) + len(early_cells),
        "thaw_late_bucket_counts": thaw_counts,
        "early_onset_bucket_counts": early_counts,
    }


def threshold_sensitivity(thaw_cells: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for threshold in THRESHOLD_SWEEP_M:
        h2_cells = [
            cell["cell_id"]
            for cell in thaw_cells
            if cell["window"]["max_frdp_m"] <= threshold
        ]
        rows.append(
            {
                "threshold_m": threshold,
                "h2_tiny_tail_count": len(h2_cells),
                "material_persistence_count": len(thaw_cells) - len(h2_cells),
                "h2_cells": h2_cells,
            }
        )
    return rows


def snow_depth_sensitivity(thaw_cells: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for threshold in SNOW_DEPTH_SWEEP_M:
        buried = []
        free = []
        mixed = []
        for cell in thaw_cells:
            warm_wet = [
                row
                for row in cell["daily_rows"]
                if row["trace_frdp_m"] > PRIMARY_MATERIAL_THRESHOLD_M and is_warm_or_wet(row)
            ]
            if not warm_wet:
                continue
            buried_days = [row for row in warm_wet if row["snow_depth_m"] >= threshold]
            free_days = [row for row in warm_wet if row["snow_depth_m"] < threshold]
            buried_fraction = len(buried_days) / len(warm_wet)
            free_fraction = len(free_days) / len(warm_wet)
            if buried_fraction >= 0.60:
                buried.append(cell["cell_id"])
            elif free_fraction >= 0.60:
                free.append(cell["cell_id"])
            else:
                mixed.append(cell["cell_id"])
        rows.append(
            {
                "snow_depth_threshold_m": threshold,
                "snow_buried_count": len(buried),
                "snow_free_persistent_count": len(free),
                "mixed_count": len(mixed),
                "snow_buried_cells": buried,
                "snow_free_persistent_cells": free,
                "mixed_cells": mixed,
            }
        )
    return rows


def routing_recommendation(thaw_cells: list[dict[str, Any]]) -> dict[str, Any]:
    counts: dict[str, int] = {}
    for cell in thaw_cells:
        counts[cell["bucket"]] = counts.get(cell["bucket"], 0) + 1
    snow_routes: dict[str, int] = {}
    for cell in thaw_cells:
        route = cell.get("snow_route", "UNCLASSIFIED")
        snow_routes[route] = snow_routes.get(route, 0) + 1
    snow_buried_count = sum(
        count for route, count in snow_routes.items() if route.startswith("SNOW-BURIED")
    )
    snow_free_count = snow_routes.get("SNOW-FREE-PERSISTENT", 0)
    if counts.get("H2", 0) > len(thaw_cells) / 2:
        route = "H2-dominant"
        next_step = "detection/extinction-floor work anchored to observation protocol"
    elif snow_buried_count > len(thaw_cells) / 2:
        route = "snow-buried-dominant"
        next_step = (
            "snow-persistence decomposition first: separate forcing-limited "
            "over-accumulation from fixable spring under-melt before Qwet"
        )
    elif snow_free_count >= max(1, len(thaw_cells) / 2):
        route = "snow-free-Qwet-candidate"
        next_step = "Qwet/wet-heat thaw-energy candidate with external authority"
    elif counts.get("H1a", 0) >= counts.get("H1b", 0):
        route = "mixed-H1a"
        next_step = "split snow-buried cells from snow-free persistent cells before Qwet"
    else:
        route = "H1b-dominant"
        next_step = "freeze/thaw state-machine top-retreat diagnostic/fix package"
    return {"route": route, "next_step": next_step, "basis": counts, "snow_route_basis": snow_routes}


def window_rows(
    daily: dict[dt.date, dict[str, Any]], start: dt.date, end: dt.date
) -> list[dict[str, Any]]:
    if end < start:
        start, end = end, start
    rows = []
    current = start
    while current <= end:
        if current in daily:
            rows.append(daily[current])
        current += dt.timedelta(days=1)
    if not rows:
        raise RuntimeError(f"no daily rows for window {start} to {end}")
    return rows


def is_warm_or_wet(row: dict[str, Any]) -> bool:
    snow_loss = row["snow_water_delta_m"] is not None and row["snow_water_delta_m"] < -0.001
    return bool(
        (row["max_air_temp_c"] is not None and row["max_air_temp_c"] > 0.0)
        or row["precip_m"] > 0.0
        or row["rain_melt_m"] > 0.0
        or snow_loss
    )


def retreats_next(row: dict[str, Any]) -> bool:
    return row["retreat_next_m"] is not None and row["retreat_next_m"] > RETREAT_EPSILON_M


def date_from_wat_row(row: dict[str, Any]) -> dt.date:
    month = int(row["month"])
    water_year = int(row["water_year"])
    calendar_year = water_year - 1 if month >= 10 else water_year
    return dt.date(calendar_year, month, int(row["day_of_month"]))


def parse_date(value: str) -> dt.date:
    return dt.date.fromisoformat(value)


def mm_to_m(value: Any) -> float:
    return f64(value) / 1000.0


def f64(value: Any) -> float:
    if value is None:
        return 0.0
    out = float(value)
    if not math.isfinite(out):
        return 0.0
    return out


def optional_f64(value: Any) -> float | None:
    if value is None:
        return None
    out = float(value)
    return out if math.isfinite(out) else None


def value_or(primary: float | None, secondary: float | None) -> float:
    return primary if primary is not None else (secondary or 0.0)


def max_or_none(values: Any) -> float | None:
    if not values:
        return None
    finite = [float(value) for value in values if value is not None and math.isfinite(float(value))]
    return max(finite) if finite else None


def min_or_none(values: Any) -> float | None:
    if not values:
        return None
    finite = [float(value) for value in values if value is not None and math.isfinite(float(value))]
    return min(finite) if finite else None


def mean_or_none(values: Any) -> float | None:
    if not values:
        return None
    return mean([float(value) for value in values if value is not None and math.isfinite(float(value))])


def max_value(rows: list[dict[str, Any]], key: str) -> float | None:
    values = [row[key] for row in rows if row.get(key) is not None]
    return max(values) if values else None


def min_value(rows: list[dict[str, Any]], key: str) -> float | None:
    values = [row[key] for row in rows if row.get(key) is not None]
    return min(values) if values else None


def sum_values(rows: list[dict[str, Any]], key: str) -> float:
    return sum(row[key] for row in rows if row.get(key) is not None)


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def median(values: list[float]) -> float | None:
    if not values:
        return None
    ordered = sorted(values)
    middle = len(ordered) // 2
    if len(ordered) % 2:
        return ordered[middle]
    return (ordered[middle - 1] + ordered[middle]) / 2.0


def paired_delta(rows: list[dict[str, Any]], key: str) -> float | None:
    values = [row[key] for row in rows if row.get(key) is not None]
    if len(values) < 2:
        return None
    return values[-1] - values[0]


def write_json(path: Path, payload: Any) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_cell_csv(path: Path, payload: dict[str, Any]) -> None:
    fields = [
        "cell_id",
        "site_id",
        "water_year",
        "signature",
        "bucket",
        "bucket_label",
        "snow_route",
        "snow_route_label",
        "residual_days",
        "window_start",
        "window_end",
        "max_frdp_m",
        "median_frdp_m",
        "max_frozen_water_m",
        "max_air_temp_c",
        "max_surface_temp_c",
        "min_snow_depth_m",
        "median_snow_depth_m",
        "max_snow_depth_m",
        "paired_snow_obs_days",
        "mean_snow_depth_residual_m",
        "paired_observed_snow_depth_delta_m",
        "paired_model_snow_depth_delta_m",
        "snow_buried_warm_wet_material_days",
        "snow_free_warm_wet_material_days",
        "snow_buried_mean_snow_depth_residual_m",
        "snow_buried_snow_water_delta_m",
        "snow_buried_mean_qsrf_w_m2",
        "snow_buried_mean_quf_w_m2",
        "snow_buried_mean_resistance_m2_k_w",
        "snow_buried_total_runoff_m",
        "snow_free_mean_snow_depth_residual_m",
        "snow_free_snow_water_delta_m",
        "snow_free_mean_qsrf_w_m2",
        "snow_free_mean_quf_w_m2",
        "snow_free_total_runoff_m",
        "snow_water_delta_total_m",
        "total_precip_m",
        "total_rain_melt_m",
        "material_days",
        "warm_wet_material_days",
        "cold_surface_warm_wet_stalled_days",
        "warm_surface_stalled_days",
        "retreat_days",
        "bucket_reason",
    ]
    rows = []
    for site in payload["sites"]:
        for group in ("thaw_late_cells", "early_onset_cells"):
            for cell in site[group]:
                window = cell["window"]
                rows.append(
                    {
                        "cell_id": cell["cell_id"],
                        "site_id": cell["site_id"],
                        "water_year": cell["water_year"],
                        "signature": cell["signature"],
                        "bucket": cell["bucket"],
                        "bucket_label": cell["bucket_label"],
                        "snow_route": cell.get("snow_route"),
                        "snow_route_label": cell.get("snow_route_label"),
                        "residual_days": cell.get("thaw_residual_days", cell.get("onset_residual_days")),
                        "window_start": window["start_date"],
                        "window_end": window["end_date"],
                        "max_frdp_m": window["max_frdp_m"],
                        "median_frdp_m": window["median_frdp_m"],
                        "max_frozen_water_m": window["max_frozen_water_m"],
                        "max_air_temp_c": window["max_air_temp_c"],
                        "max_surface_temp_c": window["max_surface_temp_c"],
                        "min_snow_depth_m": window["min_snow_depth_m"],
                        "median_snow_depth_m": window["median_snow_depth_m"],
                        "max_snow_depth_m": window["max_snow_depth_m"],
                        "paired_snow_obs_days": window["paired_snow_obs_days"],
                        "mean_snow_depth_residual_m": window["mean_snow_depth_residual_m"],
                        "paired_observed_snow_depth_delta_m": window[
                            "paired_observed_snow_depth_delta_m"
                        ],
                        "paired_model_snow_depth_delta_m": window[
                            "paired_model_snow_depth_delta_m"
                        ],
                        "snow_buried_warm_wet_material_days": window[
                            "snow_buried_warm_wet_material_days"
                        ],
                        "snow_free_warm_wet_material_days": window[
                            "snow_free_warm_wet_material_days"
                        ],
                        "snow_buried_mean_snow_depth_residual_m": window[
                            "snow_buried_mean_snow_depth_residual_m"
                        ],
                        "snow_buried_snow_water_delta_m": window[
                            "snow_buried_snow_water_delta_m"
                        ],
                        "snow_buried_mean_qsrf_w_m2": window[
                            "snow_buried_mean_qsrf_w_m2"
                        ],
                        "snow_buried_mean_quf_w_m2": window[
                            "snow_buried_mean_quf_w_m2"
                        ],
                        "snow_buried_mean_resistance_m2_k_w": window[
                            "snow_buried_mean_resistance_m2_k_w"
                        ],
                        "snow_buried_total_runoff_m": window["snow_buried_total_runoff_m"],
                        "snow_free_mean_snow_depth_residual_m": window[
                            "snow_free_mean_snow_depth_residual_m"
                        ],
                        "snow_free_snow_water_delta_m": window["snow_free_snow_water_delta_m"],
                        "snow_free_mean_qsrf_w_m2": window["snow_free_mean_qsrf_w_m2"],
                        "snow_free_mean_quf_w_m2": window["snow_free_mean_quf_w_m2"],
                        "snow_free_total_runoff_m": window["snow_free_total_runoff_m"],
                        "snow_water_delta_total_m": window["snow_water_delta_total_m"],
                        "total_precip_m": window["total_precip_m"],
                        "total_rain_melt_m": window["total_rain_melt_m"],
                        "material_days": window["material_days"],
                        "warm_wet_material_days": window["warm_wet_material_days"],
                        "cold_surface_warm_wet_stalled_days": window[
                            "cold_surface_warm_wet_stalled_days"
                        ],
                        "warm_surface_stalled_days": window["warm_surface_stalled_days"],
                        "retreat_days": window["retreat_days"],
                        "bucket_reason": cell["bucket_reason"],
                    }
                )
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)


def write_window_csv(path: Path, payload: dict[str, Any]) -> None:
    fields = [
        "cell_id",
        "date",
        "trace_frdp_m",
        "frozen_water_m",
        "snow_depth_m",
        "snow_water_m",
        "observed_snow_depth_m",
        "snow_depth_residual_m",
        "snow_conductivity_w_m_k",
        "snow_resistance_m2_k_w",
        "precip_m",
        "rain_melt_m",
        "runoff_m",
        "max_air_temp_c",
        "max_surface_temp_c",
        "residue_depth_m",
        "mean_qsrf_w_m2",
        "mean_quf_w_m2",
        "retreat_next_m",
    ]
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        for site in payload["sites"]:
            for group in ("thaw_late_cells", "early_onset_cells"):
                for cell in site[group]:
                    for row in cell["daily_rows"]:
                        writer.writerow({field: row.get(field) for field in fields} | {"cell_id": cell["cell_id"]})


def render_markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# FROST Thaw-Residual Diagnostic",
        "",
        "Evidence mode: Ran.",
        "",
        "Diagnostic-only: no detector threshold, solver physics, fixture, contract, "
        "default, or output schema changed.",
        "",
        "## Aggregate Split",
        "",
        f"- Thaw-late cells: `{payload['aggregate']['thaw_late_cell_count']}`.",
        f"- Early-onset cells: `{payload['aggregate']['early_onset_cell_count']}`.",
        f"- Thaw-late bucket counts: `{payload['aggregate']['thaw_late_bucket_counts']}`.",
        f"- Snow-controlled thaw routes: `{payload['routing_recommendation']['snow_route_basis']}`.",
        f"- Early-onset characterization: `{payload['aggregate']['early_onset_bucket_counts']}`.",
        f"- Routing recommendation: `{payload['routing_recommendation']['route']}` -> "
        f"{payload['routing_recommendation']['next_step']}.",
        "",
        "Primary material floor is diagnostic-only: "
        f"`{PRIMARY_MATERIAL_THRESHOLD_M} m`; it is not an adopted thaw detector.",
        "",
        "## Per-Cell Buckets",
        "",
        "| Cell | Bucket | Snow route | Residual d | Max frdp m | Snow depth m min/median/max | Paired snow residual m | SWE delta m | RM m | Reason |",
        "| --- | --- | --- | ---: | ---: | --- | ---: | ---: | ---: | --- |",
    ]
    for site in payload["sites"]:
        for cell in site["thaw_late_cells"]:
            window = cell["window"]
            lines.append(
                f"| `{cell['cell_id']}` | `{cell['bucket']}` {cell['bucket_label']} | "
                f"`{cell['snow_route']}` {cell['snow_route_label']} | "
                f"`{cell['thaw_residual_days']}` | `{fmt(window['max_frdp_m'])}` | "
                f"`{fmt(window['min_snow_depth_m'])}` / `{fmt(window['median_snow_depth_m'])}` / `{fmt(window['max_snow_depth_m'])}` | "
                f"`{fmt(window['mean_snow_depth_residual_m'])}` | "
                f"`{fmt(window['snow_water_delta_total_m'])}` | "
                f"`{fmt(window['total_rain_melt_m'])}` | {cell['snow_route_reason']} |"
            )
    lines.extend(
        [
            "",
            "## Snow-Persistence Evidence",
            "",
            "The snow route uses the carried-frost window's warm/wet material-frost days. "
            "Paired observed snow depth is reported where the Step 3 comparison reports "
            "contain it; sparse pairs are evidence, not a fitted classifier. The frost "
            "trace does not emit a soil-temperature time series, so the heat-path "
            "evidence is limited to surface temperature, Qsrf/Quf, snow conductivity, "
            "and the snow thermal-resistance proxy `depth / k_snow`.",
            "",
            "| Cell | Route | Buried/free warm-wet d | Paired warm/wet snow obs d | Mean snow residual m | Obs/model snow delta m | Buried SWE delta m | Buried Qsrf/Quf W m-2 | Buried snow R m2K/W | Buried runoff m |",
            "| --- | --- | ---: | ---: | ---: | --- | ---: | --- | ---: | ---: |",
        ]
    )
    for site in payload["sites"]:
        for cell in site["thaw_late_cells"]:
            window = cell["window"]
            lines.append(
                f"| `{cell['cell_id']}` | `{cell['snow_route']}` | "
                f"`{window['snow_buried_warm_wet_material_days']}` / "
                f"`{window['snow_free_warm_wet_material_days']}` | "
                f"`{window['paired_warm_wet_snow_obs_days']}` | "
                f"`{fmt(window['mean_warm_wet_snow_depth_residual_m'])}` | "
                f"`{fmt(window['paired_observed_snow_depth_delta_m'])}` / "
                f"`{fmt(window['paired_model_snow_depth_delta_m'])}` | "
                f"`{fmt(window['snow_buried_snow_water_delta_m'])}` | "
                f"`{fmt(window['snow_buried_mean_qsrf_w_m2'])}` / "
                f"`{fmt(window['snow_buried_mean_quf_w_m2'])}` | "
                f"`{fmt(window['snow_buried_mean_resistance_m2_k_w'])}` | "
                f"`{fmt(window['snow_buried_total_runoff_m'])}` |"
            )
    lines.extend(
        [
            "",
            "## Early-Onset Cells",
            "",
            "| Cell | Bucket | Residual d | Max frdp m | Max air C | Max surface C | Reason |",
            "| --- | --- | ---: | ---: | ---: | ---: | --- |",
        ]
    )
    for site in payload["sites"]:
        for cell in site["early_onset_cells"]:
            window = cell["window"]
            lines.append(
                f"| `{cell['cell_id']}` | `{cell['bucket']}` {cell['bucket_label']} | "
                f"`{cell['onset_residual_days']}` | `{fmt(window['max_frdp_m'])}` | "
                f"`{fmt(window['max_air_temp_c'])}` | `{fmt(window['max_surface_temp_c'])}` | "
                f"{cell['bucket_reason']} |"
            )
    lines.extend(
        [
            "",
            "## H2 Threshold Sensitivity",
            "",
            "| Material threshold m | H2 tiny-tail count | Material-persistence count | H2 cells |",
            "| ---: | ---: | ---: | --- |",
        ]
    )
    for row in payload["threshold_sensitivity"]:
        lines.append(
            f"| `{row['threshold_m']}` | `{row['h2_tiny_tail_count']}` | "
            f"`{row['material_persistence_count']}` | `{', '.join(row['h2_cells'])}` |"
        )
    lines.extend(
        [
            "",
            "## Snow-Depth Control Sensitivity",
            "",
            "| Snow-depth threshold m | Snow-buried count | Snow-free persistent count | Mixed count | Snow-buried cells | Snow-free cells |",
            "| ---: | ---: | ---: | ---: | --- | --- |",
        ]
    )
    for row in payload["snow_depth_sensitivity"]:
        lines.append(
            f"| `{row['snow_depth_threshold_m']}` | `{row['snow_buried_count']}` | "
            f"`{row['snow_free_persistent_count']}` | `{row['mixed_count']}` | "
            f"`{', '.join(row['snow_buried_cells'])}` | "
            f"`{', '.join(row['snow_free_persistent_cells'])}` |"
        )
    lines.extend(
        [
            "",
            "## GAP-SNOWFREEZE-002 Disposition",
            "",
            gap_disposition(payload),
            "",
        ]
    )
    return "\n".join(lines)


def gap_disposition(payload: dict[str, Any]) -> str:
    rec = payload["routing_recommendation"]
    return (
        "`GAP-SNOWFREEZE-002` remains open and is now narrowed from generic "
        "post-residue timing residuals to "
        f"`{rec['route']}` for thaw-late cells. The next fix package should pursue "
        f"{rec['next_step']}. Early-onset cells remain separate onset diagnostics."
    )


def fmt(value: Any) -> str:
    if value is None:
        return ""
    return f"{float(value):.6g}"


if __name__ == "__main__":
    raise SystemExit(main())
