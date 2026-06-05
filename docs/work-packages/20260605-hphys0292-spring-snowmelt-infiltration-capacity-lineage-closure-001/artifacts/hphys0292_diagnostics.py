#!/usr/bin/env python3
"""Run HPHYS0292 spring snowmelt/infiltration-capacity diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import sys
from pathlib import Path
from typing import Any


REPO = Path("/home/workdir/openWEPP")
PACKAGE_DIR = REPO / "docs/work-packages/20260605-hphys0292-spring-snowmelt-infiltration-capacity-lineage-closure-001"
HPHYS0291_SCRIPT = (
    REPO
    / "docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/hphys0291_diagnostics.py"
)
TARGET_DAYS = {
    1: [(2014, 141), (2014, 142), (2014, 143), (2014, 144), (2014, 145)],
    7: [(2014, 142), (2016, 110), (2016, 111)],
    39: [(2014, 141), (2014, 142), (2014, 143), (2014, 144), (2014, 145)],
}
SELECTED_COLUMNS = {
    "Ep",
    "Es",
    "Er",
    "Total-Soil",
    "SoilWaterTotal",
    "Dp",
    "latqcc",
    "Q",
    "RM",
    "Snow-Water",
    "P",
}


def load_hphys0291_module() -> Any:
    spec = importlib.util.spec_from_file_location("hphys0291_diagnostics", HPHYS0291_SCRIPT)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {HPHYS0291_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0291 = load_hphys0291_module()
HPHYS0265 = HPHYS0291.HPHYS0265


def markdown_table(headers: list[str], rows: list[list[Any]]) -> str:
    return HPHYS0265.markdown_table(headers, rows)


def mm(row: dict[str, Any] | None, key: str) -> float | None:
    if row is None or row.get(key) is None:
        return None
    return float(row[key]) * 1_000.0


def value(row: dict[str, Any] | None, key: str) -> float | None:
    if row is None or row.get(key) is None:
        return None
    return float(row[key])


def find_trace_row(trace_path: Path, year: int, julian: int) -> dict[str, Any] | None:
    if not trace_path.exists():
        return None
    with trace_path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            if (
                int(row.get("calendar_year", -1)) == year
                and int(row.get("julian_day", -1)) == julian
                and row.get("boundary") == "post_wb13"
            ):
                return row
    return None


def merged_wat_rows(run_root: Path, hillslope_id: int) -> Any:
    return HPHYS0265.candidate_baseline_merge(
        run_root / f"hillslope_output/H{hillslope_id}.wat.parquet",
        HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet",
        candidate_year_offset=2012,
    )


def find_merged_row(merged: Any, year: int, julian: int) -> Any | None:
    matches = merged[(merged["_comparison_year"] == year) & (merged["julian"] == julian)]
    if matches.empty:
        return None
    return matches.iloc[0]


def row_float(row: Any | None, name: str) -> float | None:
    if row is None:
        return None
    if name not in row or HPHYS0265.pd.isna(row[name]):
        return None
    return float(row[name])


def normalize_full_suite_summary_label(run_root: Path) -> None:
    summary_path = run_root / "reports/hillslope_semantic_summary.md"
    if not summary_path.exists():
        return
    summary = summary_path.read_text(encoding="utf-8")
    summary = summary.replace(
        "# HPHYS0291 Full H1..H39 Semantic Summary",
        "# HPHYS0292 Full H1..H39 Semantic Summary",
        1,
    )
    summary_path.write_text(summary, encoding="utf-8")


def run_targeted_traces(run_root: Path, trace_max_days: int) -> int:
    reports = run_root / "reports"
    logs = run_root / "logs" / "targeted_traces"
    output = run_root / "hillslope_output"
    runs_dir = HPHYS0265.copy_runfiles(run_root)
    rows: list[dict[str, Any]] = []
    for hillslope_id in TARGET_DAYS:
        trace_path = output / f"H{hillslope_id}.hphys0292.trace.jsonl"
        result = HPHYS0265.run_command(
            f"H{hillslope_id}_hphys0292_trace",
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
        rows.append(
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
            HPHYS0265.write_status(reports / "hphys0292_target_trace_status.tsv", rows)
            return int(result.rc)
    HPHYS0265.write_status(reports / "hphys0292_target_trace_status.tsv", rows)
    return 0


def write_selected_metrics(run_root: Path) -> None:
    reports = run_root / "reports"
    summary_json = reports / "hillslope_semantic_summary.json"
    summary = json.loads(summary_json.read_text(encoding="utf-8"))
    selected = {row["column"]: row for row in summary if row["column"] in SELECTED_COLUMNS}
    (reports / "hphys0292_selected_metrics.json").write_text(
        json.dumps(selected, indent=2) + "\n", encoding="utf-8"
    )


def extract_target_capacity_rows(run_root: Path) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    output = run_root / "hillslope_output"
    extracted: list[dict[str, Any]] = []
    for hillslope_id, days in TARGET_DAYS.items():
        merged = merged_wat_rows(run_root, hillslope_id)
        trace_path = output / f"H{hillslope_id}.hphys0292.trace.jsonl"
        for year, julian in days:
            trace = find_trace_row(trace_path, year, julian)
            merged_row = find_merged_row(merged, year, julian)
            extracted.append(
                {
                    "hillslope_id": hillslope_id,
                    "year": year,
                    "julian": julian,
                    "sim_day_index": None if trace is None else trace.get("sim_day_index"),
                    "trace_schema": None if trace is None else trace.get("schema"),
                    "candidate_rm_mm": row_float(merged_row, "RM_candidate"),
                    "baseline_rm_mm": row_float(merged_row, "RM_baseline"),
                    "candidate_q_mm": row_float(merged_row, "Q_candidate"),
                    "baseline_q_mm": row_float(merged_row, "Q_baseline"),
                    "candidate_snow_water_mm": row_float(merged_row, "Snow-Water_candidate"),
                    "baseline_snow_water_mm": row_float(merged_row, "Snow-Water_baseline"),
                    "candidate_total_soil_mm": row_float(merged_row, "Total-Soil"),
                    "baseline_total_soil_mm": row_float(merged_row, "Total-Soil Water"),
                    "trace_wb13_rm_mm": value(trace, "wb13_rm_mm"),
                    "trace_wb13_q_mm": value(trace, "wb13_q_mm"),
                    "trace_wb13_snow_water_mm": value(trace, "wb13_snow_water_mm"),
                    "trace_routed_melt_mm": mm(trace, "snow_routed_melt_m"),
                    "trace_post_winter_rain_mm": mm(trace, "snow_post_winter_rain_m"),
                    "trace_wb12_infiltration_mm": mm(trace, "wb12_infiltration_m"),
                    "trace_partition_supply_mm": mm(trace, "wb12_partition_liquid_supply_m"),
                    "trace_partition_residual_before_q_mm": mm(
                        trace, "wb12_partition_residual_before_q_m"
                    ),
                    "trace_effective_conductivity_m_s": value(
                        trace, "wb14_effective_conductivity_m_s"
                    ),
                    "trace_effective_conductivity_mm_h": None
                    if value(trace, "wb14_effective_conductivity_m_s") is None
                    else value(trace, "wb14_effective_conductivity_m_s") * 3_600_000.0,
                    "trace_matric_potential_mm": mm(trace, "wb14_matric_potential_m"),
                    "trace_runtime_swe_mm": mm(trace, "snow_runtime_swe_m"),
                    "trace_melt_sum_mm": mm(trace, "snow_hourly_melt_sum_m"),
                    "trace_raw_melt_sum_mm": mm(trace, "snow_hourly_melt_raw_sum_m"),
                    "trace_rain_retained_mm": mm(trace, "snow_hourly_rain_retained_sum_m"),
                    "trace_rain_released_mm": mm(trace, "snow_hourly_rain_released_sum_m"),
                }
            )
    (reports / "hphys0292_target_capacity_rows.json").write_text(
        json.dumps(extracted, indent=2) + "\n", encoding="utf-8"
    )
    return extracted


def write_target_capacity_markdown(run_root: Path, rows: list[dict[str, Any]]) -> None:
    reports = run_root / "reports"
    table_rows = [
        [
            f"H{row['hillslope_id']}",
            row["year"],
            row["julian"],
            row["candidate_q_mm"],
            row["baseline_q_mm"],
            row["candidate_rm_mm"],
            row["baseline_rm_mm"],
            row["candidate_snow_water_mm"],
            row["baseline_snow_water_mm"],
            row["candidate_total_soil_mm"],
            row["baseline_total_soil_mm"],
            row["trace_routed_melt_mm"],
            row["trace_post_winter_rain_mm"],
            row["trace_wb12_infiltration_mm"],
            row["trace_partition_supply_mm"],
            row["trace_partition_residual_before_q_mm"],
            row["trace_effective_conductivity_mm_h"],
            row["trace_matric_potential_mm"],
        ]
        for row in rows
    ]
    markdown = "# HPHYS0292 Target Spring Capacity Rows\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Run root: `{run_root}`\n"
    markdown += f"- Target rows JSON: `{reports / 'hphys0292_target_capacity_rows.json'}`\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Year",
            "Julian",
            "Cand Q",
            "Base Q",
            "Cand RM",
            "Base RM",
            "Cand Snow",
            "Base Snow",
            "Cand Total-Soil",
            "Base Total-Soil",
            "Trace Routed Melt",
            "Trace Post-Rain",
            "Trace Infil",
            "Trace Supply",
            "Trace Residual",
            "Trace Keff mm/h",
            "Trace Matric mm",
        ],
        table_rows,
    )
    markdown += "\n## Interpretation\n\n"
    markdown += (
        "- Target spring rows show whether candidate residual `Q` is explained by "
        "WB12 capacity terms after routed melt is published.\n"
    )
    markdown += (
        "- `Trace Residual` is diagnostic supply minus `wb12_infiltration` minus "
        "depression storage; it is not a replacement for WB13 `Q` authority.\n"
    )
    (reports / "hphys0292_target_capacity_rows.md").write_text(markdown, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", required=True, type=Path)
    parser.add_argument("--trace-max-days", type=int, default=1300)
    parser.add_argument("--skip-full-suite", action="store_true")
    args = parser.parse_args()

    for required in [
        HPHYS0265.WEPPPY_PYTHON,
        HPHYS0265.COMPARATOR,
        HPHYS0265.TOLERANCES,
        HPHYS0265.BASELINE_PARTITIONS,
    ]:
        HPHYS0265.require_path(required)

    if not args.skip_full_suite:
        full_rc = HPHYS0291.run_full_hillslope_suite(args.run_root)
        if full_rc != 0:
            return int(full_rc)
        normalize_full_suite_summary_label(args.run_root)
        write_selected_metrics(args.run_root)

    trace_rc = run_targeted_traces(args.run_root, args.trace_max_days)
    if trace_rc != 0:
        return int(trace_rc)
    rows = extract_target_capacity_rows(args.run_root)
    write_target_capacity_markdown(args.run_root, rows)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
