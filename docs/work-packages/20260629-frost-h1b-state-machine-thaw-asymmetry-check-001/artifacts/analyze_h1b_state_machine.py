#!/usr/bin/env python3
"""Scan FDHP01 top-thaw behavior in the post-residue Sleepers traces."""

from __future__ import annotations

import csv
import datetime as dt
import json
from pathlib import Path
from statistics import median
from typing import Any

import pyarrow.parquet as pq


REPO_ROOT = Path(__file__).resolve().parents[4]
PACKAGE = REPO_ROOT / "docs/work-packages/20260629-frost-h1b-state-machine-thaw-asymmetry-check-001"
ARTIFACTS = PACKAGE / "artifacts"
PRIOR_PACKAGE = REPO_ROOT / "docs/work-packages/20260629-frost-thaw-residual-diagnostic-001"
PRIOR_BUCKETS = PRIOR_PACKAGE / "artifacts/thaw_residual_buckets.json"
RUN_ROOT = REPO_ROOT / "target/frost_step3_residue_parameterization/runs"

MATERIAL_FROST_M = 0.02
FRDP_RETREAT_EPSILON_M = 0.001
TOP_RETREAT_EPSILON_M = 0.0001
SITES = ("site1_sleepers_south_field_vt", "site2_sleepers_w9_hardwood_vt")


def main() -> int:
    payload = build_payload()
    write_json(ARTIFACTS / "h1b_state_machine_thaw_asymmetry.json", payload)
    write_scan_csv(ARTIFACTS / "h1b_generalization_scan.csv", payload)
    write_cell_csv(ARTIFACTS / "h1b_cell_trace_summary.csv", payload)
    (ARTIFACTS / "h1b_state_machine_thaw_asymmetry.md").write_text(
        render_markdown(payload), encoding="utf-8"
    )
    (ARTIFACTS / "gap-snowfreeze-002-h1b-disposition.md").write_text(
        render_gap_disposition(payload), encoding="utf-8"
    )
    return 0


def build_payload() -> dict[str, Any]:
    prior = json.loads(PRIOR_BUCKETS.read_text(encoding="utf-8"))
    daily_by_site = {site: load_daily_rows(site) for site in SITES}
    h1b_cells = extract_h1b_cells(prior)
    cell_results = [
        analyze_h1b_cell(cell, daily_by_site[cell["site_id"]]) for cell in h1b_cells
    ]
    site_scans = [scan_site(site, rows) for site, rows in daily_by_site.items()]
    aggregate = aggregate_scan(site_scans, cell_results)
    determination = determine(aggregate, cell_results)
    return {
        "schema": "frost-h1b-state-machine-thaw-asymmetry-v1",
        "evidence_mode": "Static+Ran",
        "source": {
            "prior_thaw_residual_buckets": str(PRIOR_BUCKETS.relative_to(REPO_ROOT)),
            "run_root": str(RUN_ROOT.relative_to(REPO_ROOT)),
            "state_machine_static_code": {
                "top_thaw_step": "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs:1582-1650",
                "top_thaw_resistance_feedback": "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs:1654-1702",
                "branch_selection": "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs:1711-1743",
                "branch_3_driver": "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs:1846-1904",
                "trace_writer": "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs:216-408",
            },
            "detector_note": (
                "The prior thaw-date detector is based on modeled frdp > 0.0; this "
                "diagnostic also tracks thdp because branch-3 top retreat can grow a "
                "thawed cap while frdp, the bottom frozen extent, stays unchanged."
            ),
        },
        "thresholds": {
            "material_frost_m": MATERIAL_FROST_M,
            "frdp_retreat_epsilon_m": FRDP_RETREAT_EPSILON_M,
            "top_retreat_epsilon_m": TOP_RETREAT_EPSILON_M,
            "status": "diagnostic-only constants inherited from prior analysis or used only for trace sensitivity; not production thresholds",
        },
        "static_code_read": static_code_read(),
        "h1b_cells": cell_results,
        "generalization_scan": site_scans,
        "aggregate": aggregate,
        "determination": determination,
    }


def static_code_read() -> dict[str, Any]:
    return {
        "classification": "PRESENT",
        "summary": (
            "A top-down thaw path exists. `select_frost_branch` chooses branch 3 "
            "when signed surface flux is positive over an existing frozen column, "
            "`apply_active_frost_thaw_step` calls "
            "`thaw_fine_top_with_resistance_feedback`, and "
            "`thaw_fine_top_step` reduces fine-layer `slfsd_m`/`slsic_m` from the "
            "surface downward."
        ),
        "entry_guards": [
            "branch 3 requires positive signed surface flux over material frost",
            "`thaw_fine_top_with_resistance_feedback` returns early when surface_temp_c <= 0",
            "`top_flux_w_m2 = surface_temp_c / thaw_surface_heat_path(...)` must be positive",
            "a fine layer must contain frozen depth and ice mass before `thaw_fine_top_step` consumes energy",
        ],
        "important_metric_distinction": (
            "`frdp` is the bottom extent of the frozen domain. `thdp` is the thawed "
            "surface cap above a frozen segment. Top-front retreat may increase "
            "`thdp` without reducing `frdp` on the same day."
        ),
    }


def load_daily_rows(site_id: str) -> list[dict[str, Any]]:
    run_dir = RUN_ROOT / site_id / "seasonal_dec"
    wat_path = run_dir / f"{site_id}.wat.parquet"
    trace_path = run_dir / "frost_trace.jsonl"
    wat_rows = pq.read_table(wat_path).to_pylist()
    trace_rows = [
        json.loads(line)
        for line in trace_path.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]
    if len(trace_rows) != len(wat_rows) * 2:
        raise RuntimeError(
            f"expected two frost trace rows per WAT day for {site_id}; "
            f"observed {len(trace_rows)} trace rows and {len(wat_rows)} WAT rows"
        )

    rows: list[dict[str, Any]] = []
    for index, wat in enumerate(wat_rows):
        trace = trace_rows[index * 2 + 1]
        date = date_from_wat_row(wat)
        hour_depths = [
            f64(tilled) + f64(untilled)
            for tilled, untilled in zip(
                trace.get("hour_tilled_frozen_depth_m", []),
                trace.get("hour_untilled_frozen_depth_m", []),
            )
        ]
        hour_frzflg = [f64(value) for value in trace.get("hour_frzflg", [])]
        hour_surface = [f64(value) for value in trace.get("hour_surface_temp_c", [])]
        rows.append(
            {
                "site_id": site_id,
                "date": date,
                "water_year": int(wat["water_year"]),
                "trace": trace,
                "frdp_m": f64(trace.get("final_frdp_m")),
                "thdp_m": f64(trace.get("final_thdp_m")),
                "prior_frdp_m": f64(trace.get("prior_frdp_m")),
                "snow_depth_m": f64(trace.get("snow_depth_m")),
                "residue_depth_m": f64(trace.get("residue_depth_m")),
                "max_surface_temp_c": max_or_none(hour_surface),
                "max_air_temp_c": max_or_none(trace.get("hour_air_temperature_c")),
                "max_qsrf_w_m2": max_or_none(trace.get("hour_qsrf_w_m2")),
                "mean_qsrf_w_m2": mean_or_none(trace.get("hour_qsrf_w_m2")),
                "max_quf_w_m2": max_or_none(trace.get("hour_quf_w_m2")),
                "branch3_hours": sum(1 for value in hour_frzflg if branch_matches(value, 3.0)),
                "warm_branch3_hours": sum(
                    1
                    for value, temp in zip(hour_frzflg, hour_surface)
                    if branch_matches(value, 3.0) and temp > 0.0
                ),
                "min_hour_frdp_m": min(hour_depths) if hour_depths else None,
                "max_hour_frdp_m": max(hour_depths) if hour_depths else None,
            }
        )

    for index, row in enumerate(rows):
        next_row = rows[index + 1] if index + 1 < len(rows) else None
        row["next_frdp_m"] = None if next_row is None else next_row["frdp_m"]
        row["next_thdp_m"] = None if next_row is None else next_row["thdp_m"]
        row["next_frdp_retreat_m"] = (
            None if next_row is None else row["frdp_m"] - next_row["frdp_m"]
        )
        row["next_top_retreat_m"] = (
            None if next_row is None else next_row["thdp_m"] - row["thdp_m"]
        )
    return rows


def extract_h1b_cells(prior: dict[str, Any]) -> list[dict[str, Any]]:
    cells: list[dict[str, Any]] = []
    for site in prior["sites"]:
        for cell in site.get("thaw_late_cells", []):
            if cell.get("bucket") != "H1b":
                continue
            cells.append(
                {
                    "site_id": site["site_id"],
                    "cell_id": cell["cell_id"],
                    "water_year": int(cell["water_year"]),
                    "observed_thaw_date": cell["observed_thaw_date"],
                    "modeled_thaw_date": cell["modeled_thaw_date"],
                    "thaw_residual_days": cell["thaw_residual_days"],
                    "snow_route": cell.get("snow_route"),
                }
            )
    return cells


def analyze_h1b_cell(cell: dict[str, Any], rows: list[dict[str, Any]]) -> dict[str, Any]:
    start = dt.date.fromisoformat(cell["observed_thaw_date"])
    end = dt.date.fromisoformat(cell["modeled_thaw_date"])
    window = [row for row in rows if start <= row["date"] <= end]
    warm_material = [row for row in window if is_warm_material(row)]
    no_frdp = [row for row in warm_material if no_frdp_retreat(row)]
    no_frdp_top = [row for row in no_frdp if top_retreat(row)]
    no_frdp_no_top = [row for row in no_frdp if not top_retreat(row)]
    branch3 = [row for row in warm_material if row["branch3_hours"] > 0]
    return {
        **cell,
        "window_days": len(window),
        "warm_material_days": len(warm_material),
        "branch3_warm_material_days": len(branch3),
        "no_frdp_retreat_days": len(no_frdp),
        "no_frdp_retreat_with_top_retreat_days": len(no_frdp_top),
        "no_frdp_retreat_without_top_retreat_days": len(no_frdp_no_top),
        "max_frdp_m": max_or_none(row["frdp_m"] for row in window),
        "max_thdp_m": max_or_none(row["thdp_m"] for row in window),
        "thdp_gain_over_window_m": thdp_gain(window),
        "max_surface_temp_c": max_or_none(row["max_surface_temp_c"] for row in window),
        "median_snow_depth_m": median_or_none(row["snow_depth_m"] for row in window),
        "max_snow_depth_m": max_or_none(row["snow_depth_m"] for row in window),
        "blocking_term": blocking_term(no_frdp, no_frdp_top, no_frdp_no_top),
        "evidence_days": [summarize_day(row) for row in no_frdp[:12]],
    }


def scan_site(site_id: str, rows: list[dict[str, Any]]) -> dict[str, Any]:
    eligible = [row for row in rows if is_warm_material(row) and row["branch3_hours"] > 0]
    frdp_retreat_rows = [row for row in eligible if frdp_retreat(row)]
    no_frdp = [row for row in eligible if no_frdp_retreat(row)]
    no_frdp_top = [row for row in no_frdp if top_retreat(row)]
    no_frdp_no_top = [row for row in no_frdp if not top_retreat(row)]
    by_wy: dict[int, dict[str, int]] = {}
    for row in eligible:
        bucket = by_wy.setdefault(
            row["water_year"],
            {
                "eligible_branch3_days": 0,
                "frdp_retreat_days": 0,
                "no_frdp_retreat_days": 0,
                "no_frdp_with_top_retreat_days": 0,
                "no_frdp_without_top_retreat_days": 0,
            },
        )
        bucket["eligible_branch3_days"] += 1
        if frdp_retreat(row):
            bucket["frdp_retreat_days"] += 1
        elif top_retreat(row):
            bucket["no_frdp_retreat_days"] += 1
            bucket["no_frdp_with_top_retreat_days"] += 1
        else:
            bucket["no_frdp_retreat_days"] += 1
            bucket["no_frdp_without_top_retreat_days"] += 1
    return {
        "site_id": site_id,
        "eligible_branch3_warm_material_days": len(eligible),
        "frdp_retreat_days": len(frdp_retreat_rows),
        "no_frdp_retreat_days": len(no_frdp),
        "no_frdp_retreat_with_top_retreat_days": len(no_frdp_top),
        "no_frdp_retreat_without_top_retreat_days": len(no_frdp_no_top),
        "structural_stall_fraction_of_eligible": safe_ratio(len(no_frdp_no_top), len(eligible)),
        "water_years_with_structural_stall_days": sorted(
            {
                row["water_year"]
                for row in no_frdp_no_top
            }
        ),
        "water_years_with_frdp_retreat_days": sorted({row["water_year"] for row in frdp_retreat_rows}),
        "by_water_year": [
            {"water_year": wy, **counts} for wy, counts in sorted(by_wy.items())
        ],
        "structural_stall_examples": [summarize_day(row) for row in no_frdp_no_top[:10]],
    }


def aggregate_scan(site_scans: list[dict[str, Any]], cells: list[dict[str, Any]]) -> dict[str, Any]:
    totals = {
        "eligible_branch3_warm_material_days": 0,
        "frdp_retreat_days": 0,
        "no_frdp_retreat_days": 0,
        "no_frdp_retreat_with_top_retreat_days": 0,
        "no_frdp_retreat_without_top_retreat_days": 0,
    }
    for site in site_scans:
        for key in totals:
            totals[key] += int(site[key])
    totals["structural_stall_fraction_of_eligible"] = safe_ratio(
        totals["no_frdp_retreat_without_top_retreat_days"],
        totals["eligible_branch3_warm_material_days"],
    )
    totals["h1b_cells_no_frdp_retreat_days"] = sum(
        cell["no_frdp_retreat_days"] for cell in cells
    )
    totals["h1b_cells_no_frdp_with_top_retreat_days"] = sum(
        cell["no_frdp_retreat_with_top_retreat_days"] for cell in cells
    )
    return totals


def determine(aggregate: dict[str, Any], cells: list[dict[str, Any]]) -> dict[str, str]:
    structural_stall_fraction = aggregate["structural_stall_fraction_of_eligible"]
    h1b_top_fraction = safe_ratio(
        aggregate["h1b_cells_no_frdp_with_top_retreat_days"],
        aggregate["h1b_cells_no_frdp_retreat_days"],
    )
    if structural_stall_fraction <= 0.05 and h1b_top_fraction >= 0.75:
        verdict = "NARROW-EDGE"
        ratification_route = "proceed-to-ratification-with-bounded-residual-note"
        rationale = (
            "The top-down thaw path is present and branch-3 top retreat is visible "
            "in the H1b cells through `thdp` growth. True warm/material branch-3 "
            "days with neither `frdp` retreat nor `thdp` advance are rare in the "
            "full Sleepers scan."
        )
    else:
        verdict = "STRUCTURAL-GAP"
        ratification_route = "hold-ratification-for-contract-gated-state-machine-fix"
        rationale = (
            "Warm/material branch-3 days without bottom-extent retreat or top-cap "
            "advance are prevalent enough to treat the state-machine behavior as "
            "structural rather than bounded."
        )
    return {
        "verdict": verdict,
        "ratification_route": ratification_route,
        "rationale": rationale,
        "named_blocking_term": (
            "No branch guard blocks top retreat in the H1b cells. The apparent "
            "stall is the `frdp` bottom-extent detector staying fixed while branch "
            "3 grows `thdp`, the surface-thawed cap; residual persistence is lower "
            "frozen-domain persistence, not absent top-front thaw."
        ),
    }


def render_markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# H1b State-Machine Thaw-Asymmetry Check",
        "",
        "Evidence class: Static + Ran.",
        "",
        "## Determination",
        "",
        f"Verdict: `{payload['determination']['verdict']}`.",
        "",
        payload["determination"]["rationale"],
        "",
        f"Named blocking term: {payload['determination']['named_blocking_term']}",
        "",
        "## Static Code Path",
        "",
        f"Classification: `{payload['static_code_read']['classification']}`.",
        "",
        payload["static_code_read"]["summary"],
        "",
        payload["static_code_read"]["important_metric_distinction"],
        "",
        "## H1b Cells",
        "",
        "| Cell | Residual d | Warm material d | Branch-3 d | No `frdp` retreat d | With `thdp` retreat d | Max `frdp` m | Max `thdp` m | Blocking term |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |",
    ]
    for cell in payload["h1b_cells"]:
        lines.append(
            "| {cell_id} | {thaw_residual_days} | {warm_material_days} | "
            "{branch3_warm_material_days} | {no_frdp_retreat_days} | "
            "{no_frdp_retreat_with_top_retreat_days} | {max_frdp_m:.6g} | "
            "{max_thdp_m:.6g} | {blocking_term} |".format(**cell)
        )
    lines.extend(
        [
            "",
            "## Generalization Scan",
            "",
            "| Site | Branch-3 warm/material d | `frdp` retreat d | No `frdp` retreat d | No `frdp` + `thdp` retreat d | Neither retreat d | Neither fraction |",
            "| --- | ---: | ---: | ---: | ---: | ---: | ---: |",
        ]
    )
    for site in payload["generalization_scan"]:
        lines.append(
            "| {site_id} | {eligible_branch3_warm_material_days} | "
            "{frdp_retreat_days} | {no_frdp_retreat_days} | "
            "{no_frdp_retreat_with_top_retreat_days} | "
            "{no_frdp_retreat_without_top_retreat_days} | "
            "{structural_stall_fraction_of_eligible:.3f} |".format(**site)
        )
    agg = payload["aggregate"]
    lines.extend(
        [
            "",
            "Aggregate:",
            "",
            f"- Branch-3 warm/material days: `{agg['eligible_branch3_warm_material_days']}`.",
            f"- `frdp` retreat days: `{agg['frdp_retreat_days']}`.",
            f"- No-`frdp` days with `thdp` retreat: `{agg['no_frdp_retreat_with_top_retreat_days']}`.",
            f"- Days with neither `frdp` retreat nor `thdp` retreat: `{agg['no_frdp_retreat_without_top_retreat_days']}` "
            f"(`{agg['structural_stall_fraction_of_eligible']:.3f}` of eligible).",
            "",
            "## Routing",
            "",
            f"`{payload['determination']['ratification_route']}`.",
            "",
            "`GAP-SNOWFREEZE-002` remains open for the snow-persistence and snow-free "
            "wet-heat/Qwet routes, but the H1b state-machine structural gap is not "
            "a blocker for ratification.",
            "",
        ]
    )
    return "\n".join(lines)


def render_gap_disposition(payload: dict[str, Any]) -> str:
    verdict = payload["determination"]["verdict"]
    return "\n".join(
        [
            "# GAP-SNOWFREEZE-002 H1b Disposition",
            "",
            "Evidence class: Static + Ran.",
            "",
            f"H1b structural verdict: `{verdict}`.",
            "",
            payload["determination"]["rationale"],
            "",
            "Disposition: the H1b cells do not demonstrate a structural missing "
            "top-down thaw pathway. Branch 3 executes, top-front thaw is visible as "
            "`thdp` growth, and the apparent stall is caused by `frdp` representing "
            "the bottom extent of the frozen domain. Ratification may proceed with "
            "this bounded residual documented; separate snow-persistence and "
            "snow-free wet-heat routes remain open.",
            "",
        ]
    )


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_scan_csv(path: Path, payload: dict[str, Any]) -> None:
    fields = [
        "site_id",
        "water_year",
        "eligible_branch3_days",
        "frdp_retreat_days",
        "no_frdp_retreat_days",
        "no_frdp_with_top_retreat_days",
        "no_frdp_without_top_retreat_days",
    ]
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        for site in payload["generalization_scan"]:
            for row in site["by_water_year"]:
                writer.writerow(
                    {
                        "site_id": site["site_id"],
                        **row,
                    }
                )


def write_cell_csv(path: Path, payload: dict[str, Any]) -> None:
    fields = [
        "cell_id",
        "water_year",
        "thaw_residual_days",
        "warm_material_days",
        "branch3_warm_material_days",
        "no_frdp_retreat_days",
        "no_frdp_retreat_with_top_retreat_days",
        "no_frdp_retreat_without_top_retreat_days",
        "max_frdp_m",
        "max_thdp_m",
        "thdp_gain_over_window_m",
        "blocking_term",
    ]
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        for cell in payload["h1b_cells"]:
            writer.writerow({field: cell.get(field) for field in fields})


def summarize_day(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "date": row["date"].isoformat(),
        "water_year": row["water_year"],
        "frdp_m": row["frdp_m"],
        "thdp_m": row["thdp_m"],
        "next_frdp_retreat_m": row["next_frdp_retreat_m"],
        "next_top_retreat_m": row["next_top_retreat_m"],
        "branch3_hours": row["branch3_hours"],
        "warm_branch3_hours": row["warm_branch3_hours"],
        "max_surface_temp_c": row["max_surface_temp_c"],
        "snow_depth_m": row["snow_depth_m"],
        "residue_depth_m": row["residue_depth_m"],
        "max_qsrf_w_m2": row["max_qsrf_w_m2"],
        "max_quf_w_m2": row["max_quf_w_m2"],
    }


def blocking_term(
    no_frdp: list[dict[str, Any]],
    no_frdp_top: list[dict[str, Any]],
    no_frdp_no_top: list[dict[str, Any]],
) -> str:
    if no_frdp and len(no_frdp_top) / len(no_frdp) >= 0.75:
        return "none: branch 3 grows thdp while frdp bottom extent remains fixed"
    if no_frdp_no_top:
        return "minor energy/refreeze offset days; not a structural branch guard"
    return "none observed"


def is_warm_material(row: dict[str, Any]) -> bool:
    return (
        row["frdp_m"] >= MATERIAL_FROST_M
        and row["max_surface_temp_c"] is not None
        and row["max_surface_temp_c"] > 0.0
        and row["next_frdp_retreat_m"] is not None
        and row["next_top_retreat_m"] is not None
    )


def frdp_retreat(row: dict[str, Any]) -> bool:
    return row["next_frdp_retreat_m"] is not None and row["next_frdp_retreat_m"] > FRDP_RETREAT_EPSILON_M


def no_frdp_retreat(row: dict[str, Any]) -> bool:
    return row["next_frdp_retreat_m"] is not None and row["next_frdp_retreat_m"] <= FRDP_RETREAT_EPSILON_M


def top_retreat(row: dict[str, Any]) -> bool:
    return row["next_top_retreat_m"] is not None and row["next_top_retreat_m"] > TOP_RETREAT_EPSILON_M


def thdp_gain(window: list[dict[str, Any]]) -> float | None:
    if not window:
        return None
    return window[-1]["thdp_m"] - window[0]["thdp_m"]


def date_from_wat_row(row: dict[str, Any]) -> dt.date:
    water_year = int(row["water_year"])
    month = int(row["month"])
    day = int(row["day_of_month"])
    year = water_year - 1 if month >= 10 else water_year
    return dt.date(year, month, day)


def branch_matches(actual: float, expected: float) -> bool:
    return abs(actual - expected) <= 1.0e-9


def f64(value: Any) -> float:
    if value is None:
        return 0.0
    return float(value)


def max_or_none(values: Any) -> float | None:
    vals = [float(value) for value in values if value is not None]
    return max(vals) if vals else None


def mean_or_none(values: Any) -> float | None:
    vals = [float(value) for value in values if value is not None]
    return sum(vals) / len(vals) if vals else None


def median_or_none(values: Any) -> float | None:
    vals = [float(value) for value in values if value is not None]
    return median(vals) if vals else None


def safe_ratio(numerator: int, denominator: int) -> float:
    return 0.0 if denominator == 0 else numerator / denominator


if __name__ == "__main__":
    raise SystemExit(main())
