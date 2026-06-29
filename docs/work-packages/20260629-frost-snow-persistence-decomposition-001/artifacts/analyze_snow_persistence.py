#!/usr/bin/env python3
"""Decompose snow-buried Sleepers thaw-late residuals by ablation rate."""

from __future__ import annotations

import csv
import datetime as dt
import json
import math
import sys
from collections import Counter
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[4]
TOOL_DIR = REPO_ROOT / "tools/snowfreeze_observed"
sys.path.insert(0, str(TOOL_DIR))

import observed_harness  # noqa: E402
import phase_partition_snowdepth_adjudication as phase  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402
import winter_thaw_melt_response as winter_thaw  # noqa: E402


PACKAGE = REPO_ROOT / "docs/work-packages/20260629-frost-snow-persistence-decomposition-001"
ARTIFACTS = PACKAGE / "artifacts"
THAW_PACKAGE = REPO_ROOT / "docs/work-packages/20260629-frost-thaw-residual-diagnostic-001"
THAW_BUCKETS = THAW_PACKAGE / "artifacts/thaw_residual_buckets.json"
SPRING_PACK_PACKAGE = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001"
)
SPRING_PACK_REPORT = (
    SPRING_PACK_PACKAGE
    / "artifacts/spring-pack-depletion-compaction-adjudication.json"
)

SCHEMA = "frost-snow-persistence-decomposition-v1"
MATERIAL_FROST_THRESHOLD_M = 0.02
SNOW_BURIED_THRESHOLD_M = 0.10
MIN_PAIRED_ROWS_FOR_RATE = 2

ROUTE_OVER_ACCUMULATION = "OVER-ACCUMULATION-FORCING-LIMITED"
ROUTE_UNDER_MELT = "SPRING-UNDER-MELT-FIXABLE"
ROUTE_INCONCLUSIVE = "INCONCLUSIVE-SPARSE-OBS"


def main() -> int:
    payload = build_payload()
    write_json(ARTIFACTS / "snow_persistence_decomposition.json", payload)
    write_cell_csv(ARTIFACTS / "snow_persistence_decomposition.csv", payload)
    write_interval_csv(ARTIFACTS / "snow_persistence_ablation_intervals.csv", payload)
    (ARTIFACTS / "snow_persistence_decomposition.md").write_text(
        render_markdown(payload), encoding="utf-8"
    )
    (ARTIFACTS / "gap-snowfreeze-002-snow-persistence-disposition.md").write_text(
        render_gap_disposition(payload), encoding="utf-8"
    )
    print(json.dumps(payload["summary"], indent=2, sort_keys=True))
    return 0


def build_payload() -> dict[str, Any]:
    thaw_payload = read_json(THAW_BUCKETS)
    site_sources = {
        site["site_id"]: {
            "wat_path": REPO_ROOT / site["wat_path"],
            "source_run_dir": site["source_run_dir"],
            "comparison_report_path": site["comparison_report_path"],
        }
        for site in thaw_payload["sites"]
    }
    surface_by_site_id = {
        surface.fixture_dir.name: surface
        for surface in phase.SURFACES
        if surface.fixture_dir.name.startswith("site")
    }

    cells = []
    for site in thaw_payload["sites"]:
        site_id = site["site_id"]
        surface = surface_by_site_id[site_id]
        modeled = observed_harness.load_modeled_wat(site_sources[site_id]["wat_path"])
        pairs = phase.pair_observations(
            phase.load_observations(surface),
            modeled,
            surface.observation_kind,
        )
        pairs_by_date = {dt.date.fromisoformat(row["date"]): row for row in pairs}
        for cell in site["thaw_late_cells"]:
            snow_route = cell["snow_route"]
            if snow_route == "SNOW-FREE-PERSISTENT":
                continue
            if not (snow_route.startswith("SNOW-BURIED") or snow_route == "MIXED-SNOW-CONTROL"):
                continue
            cells.append(
                analyze_cell(
                    cell=cell,
                    site_source=site_sources[site_id],
                    pairs_by_date=pairs_by_date,
                )
            )

    spring_lineage = load_spring_lineage()
    summary = summarize(cells, spring_lineage)
    return {
        "schema": SCHEMA,
        "evidence_mode": "Ran",
        "source": {
            "thaw_residual_buckets": rel(THAW_BUCKETS),
            "spring_pack_depletion_compaction_report": rel(SPRING_PACK_REPORT),
            "modeled_runs": "post-residue Stage 3 seasonal_dec WAT paths from thaw-residual diagnostic",
            "tooling_reuse": [
                "observed_harness.load_modeled_wat",
                "phase_partition_snowdepth_adjudication.load_observations",
                "phase_partition_snowdepth_adjudication.pair_observations",
                "winter_thaw_melt_response MAX_INTERVAL_DAYS / OBSERVED_ABLATION_THRESHOLD_M / DEFICIT_FRACTION_THRESHOLD",
                "snotel_density_three_way.snow_depth_tolerance",
            ],
        },
        "classification": {
            "material_frost_threshold_m": MATERIAL_FROST_THRESHOLD_M,
            "snow_buried_threshold_m": SNOW_BURIED_THRESHOLD_M,
            "min_paired_rows_for_rate": MIN_PAIRED_ROWS_FOR_RATE,
            "max_interval_days": winter_thaw.MAX_INTERVAL_DAYS,
            "observed_ablation_threshold_m": winter_thaw.OBSERVED_ABLATION_THRESHOLD_M,
            "deficit_fraction_threshold": winter_thaw.DEFICIT_FRACTION_THRESHOLD,
            "threshold_status": "diagnostic reuse from existing snow-program tooling; not fitted to this residual",
        },
        "scope": {
            "included_routes": ["SNOW-BURIED-*", "MIXED-SNOW-CONTROL buried portion"],
            "excluded_routes": ["SNOW-FREE-PERSISTENT deferred to Qwet subset"],
            "cell_count": len(cells),
        },
        "summary": summary,
        "cells": cells,
        "spring_melt_lineage": spring_lineage,
    }


def analyze_cell(
    cell: dict[str, Any],
    site_source: dict[str, Any],
    pairs_by_date: dict[dt.date, dict[str, Any]],
) -> dict[str, Any]:
    start = parse_date(cell["observed_thaw_date"])
    end = parse_date(cell["modeled_thaw_date"])
    daily_rows = [
        normalize_daily_row(row)
        for row in cell["daily_rows"]
        if start <= parse_date(row["date"]) <= end
    ]
    buried_dates = {
        parse_date(row["date"])
        for row in daily_rows
        if is_material_warm_wet_buried(row)
    }
    scope_dates = set(date_range(start, end))
    if cell["snow_route"] == "MIXED-SNOW-CONTROL":
        scope_dates = buried_dates
    paired = sorted(
        [
            enrich_pair(pairs_by_date[date], date in buried_dates)
            for date in scope_dates
            if date in pairs_by_date
        ],
        key=lambda row: row["date_obj"],
    )
    intervals = build_ablation_intervals(paired, buried_dates, cell["snow_route"])
    route = classify_cell(cell, paired, intervals)
    return {
        "cell_id": cell["cell_id"],
        "site_id": cell["site_id"],
        "water_year": cell["water_year"],
        "prior_snow_route": cell["snow_route"],
        "scope_mode": (
            "buried-warm-wet-subset"
            if cell["snow_route"] == "MIXED-SNOW-CONTROL"
            else "full-carried-frost-window"
        ),
        "observed_thaw_date": cell["observed_thaw_date"],
        "modeled_thaw_date": cell["modeled_thaw_date"],
        "thaw_residual_days": cell["thaw_residual_days"],
        "wat_path": rel(site_source["wat_path"]),
        "source_run_dir": site_source["source_run_dir"],
        "buried_warm_wet_day_count": len(buried_dates),
        "paired_row_count": len(paired),
        "paired_rows": serialize_pairs(paired),
        "ablation_interval_count": len(intervals),
        "observed_ablation_interval_count": sum(
            1 for row in intervals if row["observed_ablation_interval"]
        ),
        "under_ablation_interval_count": sum(
            1 for row in intervals if row["under_ablation_interval"]
        ),
        "comparable_rate_interval_count": sum(
            1 for row in intervals if row["comparable_rate_interval"]
        ),
        "peak_evidence": peak_evidence(paired),
        "rate_evidence": rate_evidence(intervals),
        "route": route["route"],
        "route_label": route["label"],
        "confidence": route["confidence"],
        "route_reason": route["reason"],
        "ablation_intervals": intervals,
    }


def build_ablation_intervals(
    paired: list[dict[str, Any]],
    buried_dates: set[dt.date],
    snow_route: str,
) -> list[dict[str, Any]]:
    intervals = []
    for previous, current in zip(paired, paired[1:]):
        start = previous["date_obj"]
        end = current["date_obj"]
        duration_days = (end - start).days
        if duration_days <= 0 or duration_days > winter_thaw.MAX_INTERVAL_DAYS:
            continue
        interval_dates = list(date_range(start + dt.timedelta(days=1), end))
        buried_fraction = (
            sum(1 for date in interval_dates if date in buried_dates) / len(interval_dates)
            if interval_dates
            else 0.0
        )
        if snow_route == "MIXED-SNOW-CONTROL" and buried_fraction < 0.5:
            continue
        observed_loss = previous["observed_snow_depth_m"] - current["observed_snow_depth_m"]
        modeled_loss = previous["modeled_snow_depth_m"] - current["modeled_snow_depth_m"]
        observed_ablation = observed_loss >= winter_thaw.OBSERVED_ABLATION_THRESHOLD_M
        deficit = observed_loss - modeled_loss
        deficit_threshold = max(
            winter_thaw.OBSERVED_ABLATION_THRESHOLD_M,
            winter_thaw.DEFICIT_FRACTION_THRESHOLD * observed_loss
            if observed_loss > 0.0
            else 0.0,
        )
        under_ablation = observed_ablation and deficit > deficit_threshold
        comparable_rate = observed_ablation and not under_ablation
        intervals.append(
            {
                "start_date": start.isoformat(),
                "end_date": end.isoformat(),
                "duration_days": duration_days,
                "buried_day_fraction": buried_fraction,
                "observed_start_depth_m": previous["observed_snow_depth_m"],
                "observed_end_depth_m": current["observed_snow_depth_m"],
                "modeled_start_depth_m": previous["modeled_snow_depth_m"],
                "modeled_end_depth_m": current["modeled_snow_depth_m"],
                "observed_depth_loss_m": observed_loss,
                "modeled_depth_loss_m": modeled_loss,
                "observed_loss_rate_m_per_day": observed_loss / duration_days,
                "modeled_loss_rate_m_per_day": modeled_loss / duration_days,
                "depth_loss_deficit_m": deficit,
                "deficit_threshold_m": deficit_threshold,
                "modeled_loss_to_observed_loss_ratio": (
                    modeled_loss / observed_loss if observed_loss > 0.0 else None
                ),
                "observed_ablation_interval": observed_ablation,
                "under_ablation_interval": under_ablation,
                "comparable_rate_interval": comparable_rate,
            }
        )
    return intervals


def classify_cell(
    cell: dict[str, Any],
    paired: list[dict[str, Any]],
    intervals: list[dict[str, Any]],
) -> dict[str, str]:
    if len(paired) < MIN_PAIRED_ROWS_FOR_RATE:
        return {
            "route": ROUTE_INCONCLUSIVE,
            "label": "sparse paired snow observations",
            "confidence": "none",
            "reason": (
                f"only {len(paired)} paired snow rows in scoped window; cannot "
                "estimate observed-vs-modeled ablation rate"
            ),
        }
    observed_ablation = [row for row in intervals if row["observed_ablation_interval"]]
    under = [row for row in intervals if row["under_ablation_interval"]]
    if not observed_ablation:
        return {
            "route": ROUTE_INCONCLUSIVE,
            "label": "no paired observed ablation interval",
            "confidence": "low",
            "reason": (
                "paired rows do not include an observed ablation interval meeting "
                f"the reused {winter_thaw.OBSERVED_ABLATION_THRESHOLD_M} m floor"
            ),
        }
    rate = rate_evidence(intervals)
    observed_total_loss = rate.get("observed_total_loss_m") or 0.0
    depth_loss_deficit = rate.get("depth_loss_deficit_m") or 0.0
    aggregate_deficit_threshold = max(
        winter_thaw.OBSERVED_ABLATION_THRESHOLD_M,
        winter_thaw.DEFICIT_FRACTION_THRESHOLD * observed_total_loss,
    )
    aggregate_under_melt = depth_loss_deficit > aggregate_deficit_threshold
    if under and aggregate_under_melt:
        peak = peak_evidence(paired)
        confidence = "moderate" if len(observed_ablation) >= 2 else "low"
        caveat = (
            "; modeled peak also exceeds observed tolerance, so over-accumulation may contribute"
            if peak["modeled_peak_exceeds_observed_tolerance"]
            else ""
        )
        return {
            "route": ROUTE_UNDER_MELT,
            "label": "modeled ablation rate deficit",
            "confidence": confidence,
            "reason": (
                f"{len(under)}/{len(observed_ablation)} paired observed ablation "
                "intervals under-ablated by the reused snow-program rate gate, "
                "and aggregate paired modeled loss is below observed loss"
                f"{caveat}"
            ),
        }
    if under and not aggregate_under_melt:
        return {
            "route": ROUTE_INCONCLUSIVE,
            "label": "local under-ablation offset by aggregate melt",
            "confidence": "low",
            "reason": (
                f"{len(under)}/{len(observed_ablation)} intervals under-ablated, "
                "but aggregate paired modeled loss is not below observed loss; "
                "do not call spring under-melt"
            ),
        }
    peak = peak_evidence(paired)
    if peak["modeled_peak_exceeds_observed_tolerance"]:
        return {
            "route": ROUTE_OVER_ACCUMULATION,
            "label": "peak snow magnitude excess with comparable ablation rate",
            "confidence": "moderate" if len(observed_ablation) >= 1 else "low",
            "reason": (
                "modeled peak depth exceeds observed peak by more than "
                "TOL-SNOWFREEZE-011 while paired observed ablation intervals do "
                "not show a modeled loss-rate deficit"
            ),
        }
    return {
        "route": ROUTE_INCONCLUSIVE,
        "label": "paired rate comparable but peak not excessive",
        "confidence": "low",
        "reason": (
            "paired ablation rate is comparable and modeled peak does not exceed "
            "observed tolerance; snow-persistence mechanism is not resolved"
        ),
    }


def peak_evidence(paired: list[dict[str, Any]]) -> dict[str, Any]:
    if not paired:
        return {
            "observed_peak_depth_m": None,
            "modeled_peak_depth_m": None,
            "peak_depth_residual_m": None,
            "observed_peak_tolerance_m": None,
            "modeled_peak_exceeds_observed_tolerance": False,
            "mean_depth_residual_m": None,
            "modeled_over_observed_fraction": None,
        }
    observed_peak = max(row["observed_snow_depth_m"] for row in paired)
    modeled_peak = max(row["modeled_snow_depth_m"] for row in paired)
    tolerance = rubric.snow_depth_tolerance(observed_peak)
    residuals = [row["depth_residual_m"] for row in paired]
    return {
        "observed_peak_depth_m": observed_peak,
        "modeled_peak_depth_m": modeled_peak,
        "peak_depth_residual_m": modeled_peak - observed_peak,
        "observed_peak_tolerance_m": tolerance,
        "modeled_peak_exceeds_observed_tolerance": modeled_peak - observed_peak > tolerance,
        "mean_depth_residual_m": mean(residuals),
        "max_depth_residual_m": max(residuals),
        "min_depth_residual_m": min(residuals),
        "modeled_over_observed_fraction": sum(1 for value in residuals if value > 0.0)
        / len(residuals),
    }


def rate_evidence(intervals: list[dict[str, Any]]) -> dict[str, Any]:
    observed = [row for row in intervals if row["observed_ablation_interval"]]
    if not observed:
        return {
            "observed_ablation_interval_count": 0,
            "under_ablation_interval_count": 0,
            "observed_total_loss_m": None,
            "modeled_total_loss_m": None,
            "observed_mean_loss_rate_m_per_day": None,
            "modeled_mean_loss_rate_m_per_day": None,
            "modeled_loss_to_observed_loss_ratio": None,
        }
    observed_loss = sum(row["observed_depth_loss_m"] for row in observed)
    modeled_loss = sum(row["modeled_depth_loss_m"] for row in observed)
    days = sum(row["duration_days"] for row in observed)
    return {
        "observed_ablation_interval_count": len(observed),
        "under_ablation_interval_count": sum(
            1 for row in observed if row["under_ablation_interval"]
        ),
        "observed_total_loss_m": observed_loss,
        "modeled_total_loss_m": modeled_loss,
        "depth_loss_deficit_m": observed_loss - modeled_loss,
        "observed_mean_loss_rate_m_per_day": observed_loss / days,
        "modeled_mean_loss_rate_m_per_day": modeled_loss / days,
        "modeled_loss_to_observed_loss_ratio": (
            modeled_loss / observed_loss if observed_loss > 0.0 else None
        ),
    }


def summarize(cells: list[dict[str, Any]], spring_lineage: dict[str, Any]) -> dict[str, Any]:
    route_counts = Counter(cell["route"] for cell in cells)
    scoped_buried_cells = sum(
        1 for cell in cells if cell["prior_snow_route"].startswith("SNOW-BURIED")
    )
    scoped_mixed_cells = sum(
        1 for cell in cells if cell["prior_snow_route"] == "MIXED-SNOW-CONTROL"
    )
    under_count = route_counts.get(ROUTE_UNDER_MELT, 0)
    over_count = route_counts.get(ROUTE_OVER_ACCUMULATION, 0)
    inconclusive_count = route_counts.get(ROUTE_INCONCLUSIVE, 0)
    if under_count > max(over_count, inconclusive_count):
        determination = "UNIFIES-WITH-SPRING-MELT-RESIDUAL"
        next_route = "SNOW spring-melt-rate package before Qwet"
    elif over_count > max(under_count, inconclusive_count):
        determination = "FORCING-LIMITED-OVER-ACCUMULATION-DOMINANT"
        next_route = "report snow-buried cells as forcing-limited; defer only snow-free Qwet subset"
    else:
        determination = "NOT-ESTABLISHED-SPARSE-OBS"
        next_route = (
            "do not promote a melt-rate fix from these cells alone; carry sparse "
            "snow-persistence uncertainty and keep Qwet limited to snow-free subset"
        )
    return {
        "scoped_cell_count": len(cells),
        "scoped_snow_buried_cell_count": scoped_buried_cells,
        "scoped_mixed_cell_count": scoped_mixed_cells,
        "route_counts": dict(route_counts),
        "determination": determination,
        "next_route": next_route,
        "spring_lineage_summary": spring_lineage["summary"],
    }


def load_spring_lineage() -> dict[str, Any]:
    report = read_json(SPRING_PACK_REPORT)
    sleepers = [
        surface
        for surface in report["surfaces"]
        if surface["site_group"] == "sleepers"
    ]
    sleepers_failures = sum(
        surface["march_april_failure_count"] for surface in sleepers
    )
    adjudication_counts: Counter[str] = Counter()
    for surface in sleepers:
        adjudication_counts.update(surface["adjudication"]["class_counts_failures"])
    return {
        "source_report": rel(SPRING_PACK_REPORT),
        "summary": {
            "snowdensity_package": "SNOWDENSITY-10.3.10 consuming 10.3.8 coupled WAT",
            "sleepers_surface_count": len(sleepers),
            "sleepers_march_april_failure_count": sleepers_failures,
            "sleepers_failure_adjudication_counts": dict(adjudication_counts),
            "strategy_lineage": (
                "10.3.8 improved coupled WAT failures but left March/April spring "
                "pack residuals; 10.3.10 routed many rows to compaction-first, "
                "with a depletion-required tail."
            ),
        },
    }


def normalize_daily_row(row: dict[str, Any]) -> dict[str, Any]:
    return {key: row.get(key) for key in row}


def is_material_warm_wet_buried(row: dict[str, Any]) -> bool:
    return (
        f64(row.get("trace_frdp_m")) > MATERIAL_FROST_THRESHOLD_M
        and is_warm_or_wet(row)
        and f64(row.get("snow_depth_m")) >= SNOW_BURIED_THRESHOLD_M
    )


def is_warm_or_wet(row: dict[str, Any]) -> bool:
    snow_delta = row.get("snow_water_delta_m")
    return bool(
        (row.get("max_air_temp_c") is not None and f64(row.get("max_air_temp_c")) > 0.0)
        or f64(row.get("precip_m")) > 0.0
        or f64(row.get("rain_melt_m")) > 0.0
        or (snow_delta is not None and f64(snow_delta) < -0.001)
    )


def enrich_pair(row: dict[str, Any], buried_date: bool) -> dict[str, Any]:
    out = dict(row)
    date = parse_date(out["date"])
    out["date_obj"] = date
    out["buried_warm_wet_date"] = buried_date
    return out


def serialize_pairs(paired: list[dict[str, Any]]) -> list[dict[str, Any]]:
    out = []
    for row in paired:
        item = dict(row)
        item.pop("date_obj", None)
        out.append(item)
    return out


def date_range(start: dt.date, end: dt.date):
    current = start
    while current <= end:
        yield current
        current += dt.timedelta(days=1)


def parse_date(value: str) -> dt.date:
    return dt.date.fromisoformat(value)


def f64(value: Any) -> float:
    if value is None:
        return 0.0
    out = float(value)
    return out if math.isfinite(out) else 0.0


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def rel(path: Path) -> str:
    return str(path.resolve().relative_to(REPO_ROOT))


def write_cell_csv(path: Path, payload: dict[str, Any]) -> None:
    fields = [
        "cell_id",
        "site_id",
        "water_year",
        "prior_snow_route",
        "route",
        "confidence",
        "paired_row_count",
        "observed_ablation_interval_count",
        "under_ablation_interval_count",
        "observed_peak_depth_m",
        "modeled_peak_depth_m",
        "peak_depth_residual_m",
        "mean_depth_residual_m",
        "observed_total_loss_m",
        "modeled_total_loss_m",
        "modeled_loss_to_observed_loss_ratio",
        "route_reason",
    ]
    rows = []
    for cell in payload["cells"]:
        peak = cell["peak_evidence"]
        rate = cell["rate_evidence"]
        rows.append(
            {
                "cell_id": cell["cell_id"],
                "site_id": cell["site_id"],
                "water_year": cell["water_year"],
                "prior_snow_route": cell["prior_snow_route"],
                "route": cell["route"],
                "confidence": cell["confidence"],
                "paired_row_count": cell["paired_row_count"],
                "observed_ablation_interval_count": cell["observed_ablation_interval_count"],
                "under_ablation_interval_count": cell["under_ablation_interval_count"],
                "observed_peak_depth_m": peak["observed_peak_depth_m"],
                "modeled_peak_depth_m": peak["modeled_peak_depth_m"],
                "peak_depth_residual_m": peak["peak_depth_residual_m"],
                "mean_depth_residual_m": peak["mean_depth_residual_m"],
                "observed_total_loss_m": rate["observed_total_loss_m"],
                "modeled_total_loss_m": rate["modeled_total_loss_m"],
                "modeled_loss_to_observed_loss_ratio": rate[
                    "modeled_loss_to_observed_loss_ratio"
                ],
                "route_reason": cell["route_reason"],
            }
        )
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)


def write_interval_csv(path: Path, payload: dict[str, Any]) -> None:
    fields = [
        "cell_id",
        "start_date",
        "end_date",
        "duration_days",
        "buried_day_fraction",
        "observed_depth_loss_m",
        "modeled_depth_loss_m",
        "observed_loss_rate_m_per_day",
        "modeled_loss_rate_m_per_day",
        "depth_loss_deficit_m",
        "deficit_threshold_m",
        "modeled_loss_to_observed_loss_ratio",
        "observed_ablation_interval",
        "under_ablation_interval",
        "comparable_rate_interval",
    ]
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        for cell in payload["cells"]:
            for interval in cell["ablation_intervals"]:
                row = {"cell_id": cell["cell_id"], **interval}
                writer.writerow({field: row.get(field) for field in fields})


def render_markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# FROST Snow-Persistence Decomposition",
        "",
        "Evidence mode: Ran.",
        "",
        "Diagnostic-only: no melt-model, snow-model, frost-model, contract, default, "
        "fixture, or schema change.",
        "",
        "## Summary",
        "",
        f"- Scoped cells: `{payload['summary']['scoped_cell_count']}` "
        f"(`{payload['summary']['scoped_snow_buried_cell_count']}` snow-buried + "
        f"`{payload['summary']['scoped_mixed_cell_count']}` mixed buried portions).",
        f"- Route counts: `{payload['summary']['route_counts']}`.",
        f"- Determination: `{payload['summary']['determination']}`.",
        f"- Next route: {payload['summary']['next_route']}.",
        "",
        "The rate gate reuses the existing snow-program winter-thaw discriminator: "
        f"paired intervals no longer than `{winter_thaw.MAX_INTERVAL_DAYS}` days, "
        f"observed depth loss at least `{winter_thaw.OBSERVED_ABLATION_THRESHOLD_M} m`, "
        f"and modeled loss deficit greater than `max({winter_thaw.OBSERVED_ABLATION_THRESHOLD_M} m, "
        f"{winter_thaw.DEFICIT_FRACTION_THRESHOLD} * observed_loss)`.",
        "",
        "## Per-Cell Routes",
        "",
        "| Cell | Prior snow route | Route | Confidence | Paired rows | Obs ablation intervals | Under-ablation intervals | Peak obs/model/resid m | Loss obs/model/ratio | Reason |",
        "| --- | --- | --- | --- | ---: | ---: | ---: | --- | --- | --- |",
    ]
    for cell in payload["cells"]:
        peak = cell["peak_evidence"]
        rate = cell["rate_evidence"]
        lines.append(
            f"| `{cell['cell_id']}` | `{cell['prior_snow_route']}` | "
            f"`{cell['route']}` | `{cell['confidence']}` | "
            f"`{cell['paired_row_count']}` | "
            f"`{cell['observed_ablation_interval_count']}` | "
            f"`{cell['under_ablation_interval_count']}` | "
            f"`{fmt(peak['observed_peak_depth_m'])}` / "
            f"`{fmt(peak['modeled_peak_depth_m'])}` / "
            f"`{fmt(peak['peak_depth_residual_m'])}` | "
            f"`{fmt(rate['observed_total_loss_m'])}` / "
            f"`{fmt(rate['modeled_total_loss_m'])}` / "
            f"`{fmt(rate['modeled_loss_to_observed_loss_ratio'])}` | "
            f"{cell['route_reason']} |"
        )
    lines.extend(
        [
            "",
            "## Spring-Melt Lineage",
            "",
            f"- Source: `{payload['spring_melt_lineage']['source_report']}`.",
            f"- Sleepers March/April failures in 10.3.10: "
            f"`{payload['spring_melt_lineage']['summary']['sleepers_march_april_failure_count']}`.",
            f"- Sleepers 10.3.10 failure adjudication counts: "
            f"`{payload['spring_melt_lineage']['summary']['sleepers_failure_adjudication_counts']}`.",
            "",
            "## GAP-SNOWFREEZE-002 Disposition",
            "",
            gap_text(payload),
            "",
        ]
    )
    return "\n".join(lines)


def render_gap_disposition(payload: dict[str, Any]) -> str:
    return "\n".join(
        [
            "# GAP-SNOWFREEZE-002 Snow-Persistence Disposition",
            "",
            "Evidence mode: Ran.",
            "",
            gap_text(payload),
            "",
            "No melt-model, snow-model, frost-model, contract, default, fixture, or "
            "schema change was made.",
            "",
        ]
    )


def gap_text(payload: dict[str, Any]) -> str:
    summary = payload["summary"]
    next_route = summary["next_route"]
    next_sentence = next_route[:1].upper() + next_route[1:]
    return (
        "`GAP-SNOWFREEZE-002` remains open. The snow-buried thaw-late residual "
        f"routes to `{summary['determination']}` with route counts "
        f"`{summary['route_counts']}`. {next_sentence}. The two "
        "snow-free persistent cells remain the deferred `Qwet` subset."
    )


def fmt(value: Any) -> str:
    if value is None:
        return ""
    return f"{float(value):.6g}"


if __name__ == "__main__":
    raise SystemExit(main())
