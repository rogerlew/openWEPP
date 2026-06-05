#!/usr/bin/env python3
"""Audit HPHYS0302 comparator surfaces for H1/H7/H39 target windows."""

from __future__ import annotations

import argparse
import json
import shutil
from collections import defaultdict
from pathlib import Path
from typing import Any


PACKAGE_ID = "20260605-hphys0302-comparator-surface-audit-closure-001"
HPHYS0300_DIR = Path(
    "docs/work-packages/"
    "20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/"
    "artifacts"
)
HPHYS0301_DIR = Path(
    "docs/work-packages/"
    "20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/"
    "artifacts"
)
CORRECTED_LEDGER = HPHYS0300_DIR / "corrected-partition-ledger.json"
RAW_POST_LEDGER = HPHYS0300_DIR / "raw-post-raw-lineage-ledger.json"
BASELINE_SOURCE = "/workdir/wepp-forest_260430_baseline"
BASELINE_COMMIT = "dac3c950d8b16cc73774bf5ce2e7e11f80baac70"
TRACE_FIELDS = (
    "wb13_rm_mm",
    "wb13_snow_water_mm",
    "snow_hourly_melt_raw_sum_m",
    "snow_routed_melt_m",
)
MELT_TERM_FIELDS = (
    "snow_hourly_melt_amelt_in",
    "snow_hourly_melt_bmelt_in",
    "snow_hourly_melt_cmelt_in",
    "snow_hourly_melt_dmelt_in",
)


def load_json(path: Path) -> Any:
    with path.open() as handle:
        return json.load(handle)


def round6(value: float | None) -> float | None:
    if value is None:
        return None
    return round(value, 6)


def load_trace_sums(run_root: Path) -> dict[tuple[int, int, int, int], dict[str, float | bool]]:
    sums: dict[tuple[int, int, int, int], dict[str, float | bool]] = {}
    rows_by_hill: dict[int, list[dict[str, Any]]] = {}
    for hill_id in (1, 7, 39):
        trace_path = run_root / "hillslope_output" / f"H{hill_id}.hphys0299.trace.jsonl"
        if not trace_path.exists():
            raise FileNotFoundError(f"missing trace file: {trace_path}")
        rows: list[dict[str, Any]] = []
        with trace_path.open() as handle:
            for line in handle:
                row = json.loads(line)
                if row.get("boundary") == "post_wb13":
                    rows.append(row)
        rows_by_hill[hill_id] = rows

    corrected_rows = load_json(CORRECTED_LEDGER)
    for ledger_row in corrected_rows:
        hill_id = int(ledger_row["hillslope_id"])
        year = int(ledger_row["year"])
        start = int(ledger_row["start_julian"])
        end = int(ledger_row["end_julian"])
        key = (hill_id, year, start, end)
        totals: dict[str, float | bool] = {field: 0.0 for field in TRACE_FIELDS}
        terms_present = False
        missing = 0
        for trace_row in rows_by_hill[hill_id]:
            if trace_row.get("calendar_year") != year:
                continue
            julian_day = trace_row.get("julian_day")
            if julian_day is None or not (start <= int(julian_day) <= end):
                continue
            for field in TRACE_FIELDS:
                value = trace_row.get(field)
                if value is None:
                    missing += 1
                    continue
                factor = 1000.0 if field.endswith("_m") else 1.0
                totals[field] = float(totals[field]) + float(value) * factor
            terms_present = terms_present or any(
                bool(trace_row.get(field)) for field in MELT_TERM_FIELDS
            )
        totals["openwepp_term_fields_present"] = terms_present
        totals["trace_missing_value_count"] = float(missing)
        sums[key] = totals
    return sums


def build_surface_rows(run_root: Path) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    corrected_rows = load_json(CORRECTED_LEDGER)
    raw_rows = {
        (
            int(row["hillslope_id"]),
            int(row["year"]),
            int(row["start_julian"]),
            int(row["end_julian"]),
        ): row
        for row in load_json(RAW_POST_LEDGER)
    }
    trace_sums = load_trace_sums(run_root)
    surface_rows: list[dict[str, Any]] = []

    for row in corrected_rows:
        key = (
            int(row["hillslope_id"]),
            int(row["year"]),
            int(row["start_julian"]),
            int(row["end_julian"]),
        )
        trace = trace_sums[key]
        raw_post = raw_rows[key]
        context = {
            "hillslope_id": key[0],
            "year": key[1],
            "window": row["window"],
            "start_julian": key[2],
            "end_julian": key[3],
        }

        rm_baseline_observe_diff = float(row["baseline_wb_rm_observe_minus_wat_mm"])
        rm_open_trace_diff = float(row["candidate_wat_rm_sum_mm"]) - float(
            trace["wb13_rm_mm"]
        )
        surface_rows.append(
            {
                **context,
                "surface": "RM",
                "verdict": "like-for-like-pass",
                "physical_quantity": "daily WB13/WAT rainfall+melt publication surface",
                "unit": "mm",
                "baseline_surface": "baseline WAT RM and H298_WBH_C.v1 observe RM",
                "openwepp_surface": "candidate WAT RM and post-WB13 trace wb13_rm_mm",
                "same_physical_quantity_same_units": True,
                "baseline_wat_minus_observe_mm": round6(-rm_baseline_observe_diff),
                "openwepp_wat_minus_trace_mm": round6(rm_open_trace_diff),
                "residual_baseline_minus_openwepp_mm": round6(
                    float(row["observed_baseline_minus_candidate_rm_mm"])
                ),
                "producer_edit_authority": False,
            }
        )

        snow_open_trace_diff = float(row["candidate_wat_snow_sum_mm"]) - float(
            trace["wb13_snow_water_mm"]
        )
        surface_rows.append(
            {
                **context,
                "surface": "Snow-Water",
                "verdict": "output-surface-pass",
                "physical_quantity": "daily WB13/WAT snowpack water storage publication surface",
                "unit": "mm",
                "baseline_surface": "baseline WAT Snow-Water",
                "openwepp_surface": "candidate WAT Snow-Water and post-WB13 trace wb13_snow_water_mm",
                "same_physical_quantity_same_units": True,
                "openwepp_wat_minus_trace_mm": round6(snow_open_trace_diff),
                "residual_baseline_minus_openwepp_mm": round6(
                    float(row["observed_baseline_minus_candidate_snow_mm"])
                ),
                "producer_edit_authority": False,
                "limitation": "daily output surface only; no independent baseline term/state observe surface",
            }
        )

        raw_trace_diff = float(row["openwepp_raw_melt_sum_mm"]) - float(
            trace["snow_hourly_melt_raw_sum_m"]
        )
        surface_rows.append(
            {
                **context,
                "surface": "raw_hrmlt",
                "verdict": "aggregate-like-for-like-pass-not-term-authority",
                "physical_quantity": "aggregate signed raw hourly melt",
                "unit": "mm water depth after m-to-mm conversion",
                "baseline_surface": "H298_RAW_A.v1 hrmlt",
                "openwepp_surface": "snow_hourly_melt_raw_sum_m",
                "same_physical_quantity_same_units": True,
                "openwepp_ledger_minus_trace_mm": round6(raw_trace_diff),
                "residual_baseline_minus_openwepp_mm": round6(
                    float(row["baseline_raw_melt_minus_openwepp_raw_melt_mm"])
                ),
                "producer_edit_authority": False,
                "limitation": "aggregate raw melt cannot identify amelt/bmelt/cmelt/dmelt term source",
            }
        )

        routed_trace_diff = float(row["openwepp_routed_melt_sum_mm"]) - float(
            trace["snow_routed_melt_m"]
        )
        surface_rows.append(
            {
                **context,
                "surface": "post_raw_wmelt",
                "verdict": "aggregate-like-for-like-pass-not-term-authority",
                "physical_quantity": "aggregate post-raw routed melt / wmelt",
                "unit": "mm water depth after m-to-mm conversion",
                "baseline_surface": "H298_POST_A.v1 wmelt",
                "openwepp_surface": "snow_routed_melt_m",
                "same_physical_quantity_same_units": True,
                "openwepp_ledger_minus_trace_mm": round6(routed_trace_diff),
                "residual_baseline_minus_openwepp_mm": round6(
                    float(row["baseline_post_wmelt_minus_openwepp_routed_melt_mm"])
                ),
                "post_raw_minus_raw_delta_mm": round6(
                    float(raw_post["post_raw_minus_raw_delta_mm"])
                ),
                "producer_edit_authority": False,
                "limitation": "aggregate routed melt cannot identify term-level producer source",
            }
        )

        surface_rows.append(
            {
                **context,
                "surface": "melt_terms",
                "verdict": "blocked-missing-baseline-term-surface",
                "physical_quantity": "melt.for term and state lineage",
                "unit": "term-specific legacy units; unresolved until paired instrumentation exists",
                "baseline_surface": None,
                "openwepp_surface": ", ".join(MELT_TERM_FIELDS),
                "same_physical_quantity_same_units": False,
                "openwepp_term_fields_present": bool(
                    trace["openwepp_term_fields_present"]
                ),
                "required_baseline_surfaces": [
                    "amelt",
                    "bmelt",
                    "cmelt",
                    "dmelt",
                    "hrrain",
                    "hrtemp",
                    "tdpt",
                    "hrad",
                    "cloudC",
                    "vwind",
                    "snodpt",
                    "densgt",
                ],
                "producer_edit_authority": False,
                "blocker": "paired baseline melt-term/state surface is absent",
            }
        )

    counts = defaultdict(int)
    for surface_row in surface_rows:
        counts[surface_row["verdict"].replace("-", "_")] += 1
    surface_counts = {
        "windows": len(corrected_rows),
        "surfaces_per_window": 5,
        "total": len(surface_rows),
        "rm_like_for_like_pass": counts["like_for_like_pass"],
        "snow_water_output_surface_pass": counts["output_surface_pass"],
        "aggregate_cutpoint_pass_not_term_authority": counts[
            "aggregate_like_for_like_pass_not_term_authority"
        ],
        "blocked_missing_baseline_term_surface": counts[
            "blocked_missing_baseline_term_surface"
        ],
    }
    return surface_rows, surface_counts


def summarize_surfaces(surface_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    summary: list[dict[str, Any]] = []
    for surface in ("RM", "Snow-Water", "raw_hrmlt", "post_raw_wmelt", "melt_terms"):
        rows = [row for row in surface_rows if row["surface"] == surface]
        verdicts = sorted({row["verdict"] for row in rows})
        residuals = [
            row.get("residual_baseline_minus_openwepp_mm")
            for row in rows
            if row.get("residual_baseline_minus_openwepp_mm") is not None
        ]
        summary.append(
            {
                "surface": surface,
                "verdict": verdicts[0] if len(verdicts) == 1 else verdicts,
                "row_count": len(rows),
                "max_abs_residual_mm": round6(
                    max((abs(float(value)) for value in residuals), default=0.0)
                ),
                "producer_edit_authority": all(
                    row.get("producer_edit_authority") is True for row in rows
                ),
            }
        )
    return summary


def write_markdown(
    artifact_dir: Path,
    surface_counts: dict[str, Any],
    surface_summary: list[dict[str, Any]],
) -> None:
    summary_path = artifact_dir / "comparator-surface-audit-summary.md"
    decision_path = artifact_dir / "surface-audit-decision.md"
    lines = [
        "# HPHYS0302 Comparator-Surface Audit Summary",
        "",
        "Status: executed",
        "Evidence mode: Ran",
        "",
        "## Result",
        "",
        "- Production edit authorized: `false`.",
        "- `RM` passes as a like-for-like daily WB13/WAT publication surface.",
        "- `Snow-Water` passes as a daily output surface, not producer authority.",
        "- Raw `hrmlt` and post-raw `wmelt` pass only as aggregate cut-point surfaces.",
        "- Term-level melt correction is blocked because paired baseline `amelt`/`bmelt`/`cmelt`/`dmelt` term/state surfaces are absent.",
        "",
        "## Counts",
        "",
        f"- Windows: `{surface_counts['windows']}`.",
        f"- Surface rows: `{surface_counts['total']}`.",
        f"- RM like-for-like rows: `{surface_counts['rm_like_for_like_pass']}`.",
        f"- Snow-Water output-surface rows: `{surface_counts['snow_water_output_surface_pass']}`.",
        f"- Aggregate cut-point rows: `{surface_counts['aggregate_cutpoint_pass_not_term_authority']}`.",
        f"- Blocked melt-term rows: `{surface_counts['blocked_missing_baseline_term_surface']}`.",
        "",
        "## Surface Summary",
        "",
        "| Surface | Verdict | Rows | Max abs residual mm |",
        "|---|---|---:|---:|",
    ]
    for item in surface_summary:
        lines.append(
            f"| `{item['surface']}` | `{item['verdict']}` | {item['row_count']} | {item['max_abs_residual_mm']} |"
        )
    summary_path.write_text("\n".join(lines) + "\n")

    decision_path.write_text(
        "\n".join(
            [
                "# HPHYS0302 Surface-Audit Decision",
                "",
                "Status: HOLD",
                "Evidence mode: Ran",
                "",
                "No production physics edit is authorized by HPHYS0302.",
                "",
                "The valid next package is paired baseline/openWEPP melt term and state instrumentation for `amelt`, `bmelt`, `cmelt`, `dmelt`, `hrrain`, `hrtemp`, `tdpt`, `hrad`, `cloudC`, `vwind`, `snodpt`, and `densgt` over all nine H1/H7/H39 target windows.",
            ]
        )
        + "\n"
    )


def copy_forward_metrics(artifact_dir: Path) -> None:
    for name in ("full-39-suite-metrics.md", "full-39-suite-summary.json"):
        source = HPHYS0301_DIR / name
        if not source.exists():
            source = HPHYS0300_DIR / name
        if source.exists():
            shutil.copyfile(source, artifact_dir / name)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--run-root",
        type=Path,
        default=Path("/tmp/hphys0300_full_20260605T155527Z"),
    )
    parser.add_argument(
        "--artifact-dir",
        type=Path,
        default=Path(
            "docs/work-packages/"
            "20260605-hphys0302-comparator-surface-audit-closure-001/"
            "artifacts"
        ),
    )
    args = parser.parse_args()

    args.artifact_dir.mkdir(parents=True, exist_ok=True)
    surface_rows, surface_counts = build_surface_rows(args.run_root)
    surface_summary = summarize_surfaces(surface_rows)
    ledger = {
        "package": PACKAGE_ID,
        "baseline_source": BASELINE_SOURCE,
        "baseline_commit": BASELINE_COMMIT,
        "run_root": str(args.run_root),
        "production_edit_authorized": False,
        "decision": "hold-paired-baseline-melt-term-state-surface-missing",
        "surface_counts": surface_counts,
        "surface_summary": surface_summary,
        "surface_rows": surface_rows,
        "next_required_surfaces": [
            "amelt",
            "bmelt",
            "cmelt",
            "dmelt",
            "hrrain",
            "hrtemp",
            "tdpt",
            "hrad",
            "cloudC",
            "vwind",
            "snodpt",
            "densgt",
        ],
    }
    (args.artifact_dir / "comparator-surface-audit-ledger.json").write_text(
        json.dumps(ledger, indent=2, sort_keys=True) + "\n"
    )
    write_markdown(args.artifact_dir, surface_counts, surface_summary)
    copy_forward_metrics(args.artifact_dir)


if __name__ == "__main__":
    main()
