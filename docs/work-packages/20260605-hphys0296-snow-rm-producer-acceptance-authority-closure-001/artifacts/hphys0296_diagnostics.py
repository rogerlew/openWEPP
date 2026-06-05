#!/usr/bin/env python3
"""Run HPHYS0296 snow/RM producer acceptance diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import math
import sys
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
HPHYS0295_SCRIPT = (
    REPO
    / "docs/work-packages/20260605-hphys0295-cumulative-storage-budget-ownership-closure-001/artifacts/hphys0295_diagnostics.py"
)

TARGET_WINDOWS: dict[int, list[tuple[str, int, int, int]]] = {
    1: [
        ("first-abs-storage-ge-10mm", 2013, 112, 127),
        ("spring-2014", 2014, 120, 146),
        ("spring-2016", 2016, 104, 111),
    ],
    7: [
        ("first-abs-storage-ge-10mm", 2013, 112, 127),
        ("spring-2014", 2014, 120, 146),
        ("spring-2016", 2016, 104, 111),
    ],
    39: [
        ("first-abs-storage-ge-10mm", 2013, 97, 112),
        ("spring-2014", 2014, 120, 146),
        ("spring-2016", 2016, 104, 111),
    ],
}

SELECTED_COLUMNS = {
    "Ep",
    "Total-Soil",
    "SoilWaterTotal",
    "Dp",
    "latqcc",
    "Q",
    "RM",
    "Snow-Water",
}


def load_hphys0295_module() -> Any:
    spec = importlib.util.spec_from_file_location("hphys0295_diagnostics", HPHYS0295_SCRIPT)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {HPHYS0295_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0295 = load_hphys0295_module()
HPHYS0291 = HPHYS0295.HPHYS0291
HPHYS0265 = HPHYS0295.HPHYS0265


def rounded(value: Any, digits: int = 6) -> Any:
    if value is None:
        return None
    if isinstance(value, float):
        if math.isnan(value):
            return None
        return round(value, digits)
    return value


def wat_candidate(row: Any | None, symbol: str) -> float:
    if row is None:
        return 0.0
    value = HPHYS0295.wat_candidate(row, symbol)
    return 0.0 if value is None else float(value)


def wat_baseline(row: Any | None, symbol: str) -> float:
    if row is None:
        return 0.0
    value = HPHYS0295.wat_baseline(row, symbol)
    return 0.0 if value is None else float(value)


def wat_delta(row: Any | None, symbol: str) -> float:
    if row is None:
        return 0.0
    value = HPHYS0295.wat_delta(row, symbol)
    return 0.0 if value is None else float(value)


def trace_m_to_mm(row: dict[str, Any] | None, key: str) -> float:
    if row is None:
        return 0.0
    value = row.get(key)
    if value is None:
        return 0.0
    return float(value) * 1_000.0


def trace_mm(row: dict[str, Any] | None, key: str) -> float:
    if row is None:
        return 0.0
    value = row.get(key)
    if value is None:
        return 0.0
    return float(value)


def trace_map_values_mm(row: dict[str, Any] | None, key: str) -> list[float]:
    if row is None:
        return []
    values = row.get(key)
    if not isinstance(values, dict):
        return []
    return [float(value) * 1_000.0 for value in values.values()]


def normalize_full_suite_summary_label(run_root: Path) -> None:
    summary_path = run_root / "reports/hillslope_semantic_summary.md"
    if not summary_path.exists():
        return
    summary = summary_path.read_text(encoding="utf-8")
    summary = summary.replace(
        "# HPHYS0291 Full H1..H39 Semantic Summary",
        "# HPHYS0296 Full H1..H39 Semantic Summary",
        1,
    )
    summary_path.write_text(summary, encoding="utf-8")


def write_selected_metrics(run_root: Path) -> None:
    reports = run_root / "reports"
    summary_json = reports / "hillslope_semantic_summary.json"
    summary = json.loads(summary_json.read_text(encoding="utf-8"))
    selected = {row["column"]: row for row in summary if row["column"] in SELECTED_COLUMNS}
    (reports / "hphys0296_selected_metrics.json").write_text(
        json.dumps(selected, indent=2) + "\n", encoding="utf-8"
    )


def run_targeted_traces(run_root: Path, trace_max_days: int) -> int:
    reports = run_root / "reports"
    logs = run_root / "logs" / "targeted_traces"
    output = run_root / "hillslope_output"
    runs_dir = HPHYS0265.copy_runfiles(run_root)
    status_rows: list[dict[str, Any]] = []
    for hillslope_id in TARGET_WINDOWS:
        trace_path = output / f"H{hillslope_id}.hphys0296.trace.jsonl"
        result = HPHYS0265.run_command(
            f"H{hillslope_id}_hphys0296_trace",
            [
                str(HPHYS0291.HILL_BIN),
                "--run-dir",
                str(runs_dir),
                "--run-file",
                f"p{hillslope_id}_openwepp.run",
                "--output-dir",
                str(output),
                "--policy",
                "compat",
            ],
            logs,
            env={
                "OPENWEPP_HPHYS0245_TRACE_PATH": str(trace_path),
                "OPENWEPP_HPHYS0245_TRACE_MAX_DAYS": str(trace_max_days),
            },
        )
        status_rows.append(
            {
                "hillslope_id": hillslope_id,
                "rc": result.rc,
                "seconds": f"{result.seconds:.3f}",
                "trace_path": trace_path,
                "stdout": result.stdout,
                "stderr": result.stderr,
            }
        )
        if result.rc != 0:
            HPHYS0265.write_status(reports / "hphys0296_target_trace_status.tsv", status_rows)
            return int(result.rc)
    HPHYS0265.write_status(reports / "hphys0296_target_trace_status.tsv", status_rows)
    return 0


def classify_window(row: dict[str, Any]) -> str:
    residual_scale = max(
        abs(row["delta_rm_sum_mm"]),
        abs(row["snow_delta_end_mm"]),
        abs(row["snow_delta_start_mm"]),
    )
    negative_melt = abs(row["negative_raw_melt_sum_mm"])
    if residual_scale < 0.5:
        return "low-residual"
    if negative_melt >= 0.5 and negative_melt * 2.0 >= residual_scale * 0.5:
        return "corrected-negative-melt-candidate"
    if row["rm_identity_abs_sum_mm"] <= 1.0 and row["snow_closure_abs_sum_mm"] <= 1.0:
        return "producer-magnitude-timing-hold"
    return "producer-identity-or-closure-hold"


def analyze_window(
    run_root: Path,
    hillslope_id: int,
    merged: Any,
    trace_index: dict[tuple[int, int], dict[str, Any]],
    window: tuple[str, int, int, int],
) -> dict[str, Any]:
    name, year, start, end = window
    rows = merged[
        (merged["_comparison_year"] == year)
        & (merged["julian"] >= start)
        & (merged["julian"] <= end)
    ].sort_values(["_comparison_year", "julian"])
    start_row = None if rows.empty else rows.iloc[0]
    end_row = None if rows.empty else rows.iloc[-1]
    result: dict[str, Any] = {
        "hillslope_id": hillslope_id,
        "window": name,
        "year": year,
        "start_julian": start,
        "end_julian": end,
        "row_count": int(len(rows)),
        "snow_delta_start_mm": wat_delta(start_row, "Snow-Water"),
        "snow_delta_end_mm": wat_delta(end_row, "Snow-Water"),
        "rm_identity_abs_sum_mm": 0.0,
        "snow_closure_abs_sum_mm": 0.0,
        "positive_raw_melt_sum_mm": 0.0,
        "negative_raw_melt_sum_mm": 0.0,
        "routed_melt_sum_mm": 0.0,
        "post_winter_rain_sum_mm": 0.0,
        "retained_rain_sum_mm": 0.0,
        "released_rain_sum_mm": 0.0,
        "snowfall_water_sum_mm": 0.0,
        "candidate_rm_sum_mm": 0.0,
        "baseline_rm_sum_mm": 0.0,
        "delta_rm_sum_mm": 0.0,
        "candidate_snow_sum_mm": 0.0,
        "baseline_snow_sum_mm": 0.0,
        "delta_snow_sum_mm": 0.0,
        "candidate_q_sum_mm": 0.0,
        "baseline_q_sum_mm": 0.0,
        "delta_q_sum_mm": 0.0,
        "negative_raw_melt_day_count": 0,
        "negative_raw_melt_hour_count": 0,
    }
    for _, row in rows.iterrows():
        key = (int(row["_comparison_year"]), int(row["julian"]))
        trace = trace_index.get(key)
        raw_values = trace_map_values_mm(trace, "snow_hourly_melt_raw_m")
        positive_raw = sum(value for value in raw_values if value > 0.0)
        negative_raw = sum(value for value in raw_values if value < 0.0)
        if negative_raw < -1.0e-9:
            result["negative_raw_melt_day_count"] += 1
            result["negative_raw_melt_hour_count"] += sum(1 for value in raw_values if value < -1.0e-9)
        trace_rm = trace_mm(trace, "wb13_rm_mm")
        rm_identity = trace_rm - (
            trace_m_to_mm(trace, "snow_routed_melt_m")
            + trace_m_to_mm(trace, "snow_post_winter_rain_m")
        )
        result["rm_identity_abs_sum_mm"] += abs(rm_identity)
        result["snow_closure_abs_sum_mm"] += abs(
            trace_m_to_mm(trace, "snow_runtime_swe_closure_error_m")
        )
        result["positive_raw_melt_sum_mm"] += positive_raw
        result["negative_raw_melt_sum_mm"] += negative_raw
        result["routed_melt_sum_mm"] += trace_m_to_mm(trace, "snow_routed_melt_m")
        result["post_winter_rain_sum_mm"] += trace_m_to_mm(trace, "snow_post_winter_rain_m")
        result["retained_rain_sum_mm"] += trace_m_to_mm(trace, "snow_hourly_rain_retained_sum_m")
        result["released_rain_sum_mm"] += trace_m_to_mm(trace, "snow_hourly_rain_released_sum_m")
        result["snowfall_water_sum_mm"] += trace_m_to_mm(
            trace, "snow_hourly_snowfall_water_equiv_sum_m"
        )
        result["candidate_rm_sum_mm"] += wat_candidate(row, "RM")
        result["baseline_rm_sum_mm"] += wat_baseline(row, "RM")
        result["delta_rm_sum_mm"] += wat_delta(row, "RM")
        result["candidate_snow_sum_mm"] += wat_candidate(row, "Snow-Water")
        result["baseline_snow_sum_mm"] += wat_baseline(row, "Snow-Water")
        result["delta_snow_sum_mm"] += wat_delta(row, "Snow-Water")
        result["candidate_q_sum_mm"] += wat_candidate(row, "Q")
        result["baseline_q_sum_mm"] += wat_baseline(row, "Q")
        result["delta_q_sum_mm"] += wat_delta(row, "Q")
    result["classification"] = classify_window(result)
    return result


def first_divergence_rows(
    merged: Any,
    trace_index: dict[tuple[int, int], dict[str, Any]],
    hillslope_id: int,
    limit: int = 12,
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for _, row in merged.iterrows():
        delta_rm = wat_delta(row, "RM")
        delta_snow = wat_delta(row, "Snow-Water")
        if abs(delta_rm) <= 0.01 and abs(delta_snow) <= 0.01:
            continue
        year = int(row["_comparison_year"])
        julian = int(row["julian"])
        trace = trace_index.get((year, julian))
        rows.append(
            {
                "hillslope_id": hillslope_id,
                "year": year,
                "julian": julian,
                "delta_rm_mm": delta_rm,
                "delta_snow_mm": delta_snow,
                "candidate_rm_mm": wat_candidate(row, "RM"),
                "baseline_rm_mm": wat_baseline(row, "RM"),
                "candidate_snow_mm": wat_candidate(row, "Snow-Water"),
                "baseline_snow_mm": wat_baseline(row, "Snow-Water"),
                "trace_routed_melt_mm": trace_m_to_mm(trace, "snow_routed_melt_m"),
                "trace_post_winter_rain_mm": trace_m_to_mm(trace, "snow_post_winter_rain_m"),
                "trace_snowfall_water_mm": trace_m_to_mm(
                    trace, "snow_hourly_snowfall_water_equiv_sum_m"
                ),
                "trace_retained_rain_mm": trace_m_to_mm(trace, "snow_hourly_rain_retained_sum_m"),
                "trace_released_rain_mm": trace_m_to_mm(trace, "snow_hourly_rain_released_sum_m"),
                "trace_swe_closure_error_mm": trace_m_to_mm(
                    trace, "snow_runtime_swe_closure_error_m"
                ),
            }
        )
        if len(rows) >= limit:
            break
    return rows


def write_classification(run_root: Path) -> None:
    reports = run_root / "reports"
    windows: list[dict[str, Any]] = []
    first_rows: list[dict[str, Any]] = []
    for hillslope_id, target_windows in TARGET_WINDOWS.items():
        merged = HPHYS0295.merged_wat_rows(run_root, hillslope_id)
        trace_index = HPHYS0295.load_trace_index(
            run_root / f"hillslope_output/H{hillslope_id}.hphys0296.trace.jsonl"
        )
        for window in target_windows:
            windows.append(analyze_window(run_root, hillslope_id, merged, trace_index, window))
        first_rows.extend(first_divergence_rows(merged, trace_index, hillslope_id))

    (reports / "hphys0296_snow_rm_windows.json").write_text(
        json.dumps(windows, indent=2) + "\n", encoding="utf-8"
    )
    (reports / "hphys0296_first_divergence_rows.json").write_text(
        json.dumps(first_rows, indent=2) + "\n", encoding="utf-8"
    )

    headers = [
        "Hill",
        "Window",
        "Year",
        "Days",
        "ΔRM sum",
        "ΔSnow start",
        "ΔSnow end",
        "Neg raw melt",
        "Neg days",
        "Routed melt",
        "Post rain",
        "Released rain",
        "RM identity abs",
        "SWE closure abs",
        "Class",
    ]
    window_rows = [
        [
            f"H{row['hillslope_id']}",
            row["window"],
            row["year"],
            f"{row['start_julian']}-{row['end_julian']}",
            rounded(row["delta_rm_sum_mm"]),
            rounded(row["snow_delta_start_mm"]),
            rounded(row["snow_delta_end_mm"]),
            rounded(row["negative_raw_melt_sum_mm"]),
            row["negative_raw_melt_day_count"],
            rounded(row["routed_melt_sum_mm"]),
            rounded(row["post_winter_rain_sum_mm"]),
            rounded(row["released_rain_sum_mm"]),
            rounded(row["rm_identity_abs_sum_mm"]),
            rounded(row["snow_closure_abs_sum_mm"]),
            row["classification"],
        ]
        for row in windows
    ]
    first_headers = [
        "Hill",
        "Year",
        "Julian",
        "ΔRM",
        "ΔSnow",
        "Candidate RM",
        "Baseline RM",
        "Candidate Snow",
        "Baseline Snow",
        "Routed melt",
        "Post rain",
        "Snowfall",
        "Closure err",
    ]
    first_table_rows = [
        [
            f"H{row['hillslope_id']}",
            row["year"],
            row["julian"],
            rounded(row["delta_rm_mm"]),
            rounded(row["delta_snow_mm"]),
            rounded(row["candidate_rm_mm"]),
            rounded(row["baseline_rm_mm"]),
            rounded(row["candidate_snow_mm"]),
            rounded(row["baseline_snow_mm"]),
            rounded(row["trace_routed_melt_mm"]),
            rounded(row["trace_post_winter_rain_mm"]),
            rounded(row["trace_snowfall_water_mm"]),
            rounded(row["trace_swe_closure_error_mm"]),
        ]
        for row in first_rows
    ]

    class_counts: dict[str, int] = {}
    for row in windows:
        class_counts[row["classification"]] = class_counts.get(row["classification"], 0) + 1
    markdown = "# HPHYS0296 Snow/RM Producer Acceptance Diagnostics\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Run root: `{run_root}`\n"
    markdown += f"- Window JSON: `{reports / 'hphys0296_snow_rm_windows.json'}`\n"
    markdown += f"- First-divergence JSON: `{reports / 'hphys0296_first_divergence_rows.json'}`\n"
    markdown += f"- Trace status: `{reports / 'hphys0296_target_trace_status.tsv'}`\n\n"
    markdown += "## Classification Counts\n\n"
    for classification, count in sorted(class_counts.items()):
        markdown += f"- `{classification}`: `{count}` windows\n"
    markdown += "\n## H1/H7/H39 Windows\n\n"
    markdown += HPHYS0265.markdown_table(headers, window_rows)
    markdown += "\n\n## First Divergence Rows\n\n"
    markdown += HPHYS0265.markdown_table(first_headers, first_table_rows)
    markdown += "\n\n## Interpretation\n\n"
    markdown += (
        "- `producer-magnitude-timing-hold` means the target window has material "
        "candidate/baseline `RM` or `Snow-Water` residuals, candidate publication "
        "identity is internally closed, and material negative raw melt does not "
        "explain the residual.\n"
    )
    markdown += (
        "- `RM identity abs` is the absolute residual between WB13 `RM` and "
        "`snow.routed_melt_m + snow.post_winter_rain_m` across the window.\n"
    )
    markdown += (
        "- `SWE closure abs` is the absolute snow-state closure residual across "
        "the window.\n"
    )
    (reports / "hphys0296_snow_rm_acceptance.md").write_text(markdown, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-root", type=Path, required=True)
    parser.add_argument("--trace-max-days", type=int, default=1_800)
    parser.add_argument("--skip-full-suite", action="store_true")
    parser.add_argument("--skip-targeted-traces", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    args.run_root.mkdir(parents=True, exist_ok=True)
    if not args.skip_full_suite:
        full_rc = HPHYS0291.run_full_hillslope_suite(args.run_root)
        if full_rc != 0:
            return int(full_rc)
        normalize_full_suite_summary_label(args.run_root)
        write_selected_metrics(args.run_root)
    if not args.skip_targeted_traces:
        trace_rc = run_targeted_traces(args.run_root, args.trace_max_days)
        if trace_rc != 0:
            return int(trace_rc)
    write_selected_metrics(args.run_root)
    write_classification(args.run_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
