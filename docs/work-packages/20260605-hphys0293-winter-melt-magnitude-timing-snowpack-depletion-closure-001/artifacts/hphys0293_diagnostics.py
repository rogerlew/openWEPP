#!/usr/bin/env python3
"""Run HPHYS0293 winter melt magnitude/timing snowpack-depletion diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import sys
from pathlib import Path
from typing import Any


REPO = Path("/home/workdir/openWEPP")
HPHYS0291_SCRIPT = (
    REPO
    / "docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/hphys0291_diagnostics.py"
)
TARGET_DAYS = {
    1: [
        (2014, 107),
        (2014, 109),
        (2014, 115),
        (2014, 120),
        (2014, 124),
        (2014, 128),
        (2014, 132),
        (2014, 133),
        (2014, 141),
        (2014, 142),
        (2014, 143),
        (2014, 144),
        (2014, 145),
    ],
    7: [
        (2014, 107),
        (2014, 109),
        (2014, 115),
        (2014, 118),
        (2014, 120),
        (2014, 124),
        (2014, 128),
        (2014, 132),
        (2014, 133),
        (2014, 142),
        (2014, 143),
        (2014, 144),
        (2014, 145),
        (2014, 146),
        (2016, 104),
        (2016, 105),
        (2016, 106),
        (2016, 107),
        (2016, 108),
        (2016, 109),
        (2016, 110),
        (2016, 111),
    ],
    39: [
        (2014, 107),
        (2014, 109),
        (2014, 115),
        (2014, 120),
        (2014, 124),
        (2014, 128),
        (2014, 132),
        (2014, 133),
        (2014, 141),
        (2014, 142),
        (2014, 143),
        (2014, 144),
        (2014, 145),
    ],
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


def value(row: dict[str, Any] | None, key: str) -> float | None:
    if row is None or row.get(key) is None:
        return None
    return float(row[key])


def mm(row: dict[str, Any] | None, key: str) -> float | None:
    row_value = value(row, key)
    if row_value is None:
        return None
    return row_value * 1_000.0


def rounded(row_value: Any, digits: int = 6) -> Any:
    if row_value is None:
        return None
    if isinstance(row_value, float):
        return round(row_value, digits)
    return row_value


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


def wat_candidate(row: Any | None, symbol: str) -> float | None:
    if row is None:
        return None
    return float(HPHYS0265.wat_candidate(row, symbol))


def wat_baseline(row: Any | None, symbol: str) -> float | None:
    if row is None:
        return None
    return float(HPHYS0265.wat_baseline(row, symbol))


def wat_delta(row: Any | None, symbol: str) -> float | None:
    candidate = wat_candidate(row, symbol)
    baseline = wat_baseline(row, symbol)
    if candidate is None or baseline is None:
        return None
    return candidate - baseline


def normalize_full_suite_summary_label(run_root: Path) -> None:
    summary_path = run_root / "reports/hillslope_semantic_summary.md"
    if not summary_path.exists():
        return
    summary = summary_path.read_text(encoding="utf-8")
    summary = summary.replace(
        "# HPHYS0291 Full H1..H39 Semantic Summary",
        "# HPHYS0293 Full H1..H39 Semantic Summary",
        1,
    )
    summary_path.write_text(summary, encoding="utf-8")


def write_selected_metrics(run_root: Path) -> None:
    reports = run_root / "reports"
    summary_json = reports / "hillslope_semantic_summary.json"
    summary = json.loads(summary_json.read_text(encoding="utf-8"))
    selected = {row["column"]: row for row in summary if row["column"] in SELECTED_COLUMNS}
    (reports / "hphys0293_selected_metrics.json").write_text(
        json.dumps(selected, indent=2) + "\n", encoding="utf-8"
    )


def run_targeted_traces(run_root: Path, trace_max_days: int) -> int:
    reports = run_root / "reports"
    logs = run_root / "logs" / "targeted_traces"
    output = run_root / "hillslope_output"
    runs_dir = HPHYS0265.copy_runfiles(run_root)
    status_rows: list[dict[str, Any]] = []
    for hillslope_id in TARGET_DAYS:
        trace_path = output / f"H{hillslope_id}.hphys0293.trace.jsonl"
        result = HPHYS0265.run_command(
            f"H{hillslope_id}_hphys0293_trace",
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
            HPHYS0265.write_status(reports / "hphys0293_target_trace_status.tsv", status_rows)
            return int(result.rc)
    HPHYS0265.write_status(reports / "hphys0293_target_trace_status.tsv", status_rows)
    return 0


def extract_target_depletion_rows(run_root: Path) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    output = run_root / "hillslope_output"
    extracted: list[dict[str, Any]] = []
    for hillslope_id, days in TARGET_DAYS.items():
        merged = merged_wat_rows(run_root, hillslope_id)
        trace_path = output / f"H{hillslope_id}.hphys0293.trace.jsonl"
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
                    "candidate_q_mm": wat_candidate(merged_row, "Q"),
                    "baseline_q_mm": wat_baseline(merged_row, "Q"),
                    "delta_q_mm": wat_delta(merged_row, "Q"),
                    "candidate_rm_mm": wat_candidate(merged_row, "RM"),
                    "baseline_rm_mm": wat_baseline(merged_row, "RM"),
                    "delta_rm_mm": wat_delta(merged_row, "RM"),
                    "candidate_snow_water_mm": wat_candidate(merged_row, "Snow-Water"),
                    "baseline_snow_water_mm": wat_baseline(merged_row, "Snow-Water"),
                    "delta_snow_water_mm": wat_delta(merged_row, "Snow-Water"),
                    "candidate_total_soil_mm": wat_candidate(merged_row, "Total-Soil"),
                    "baseline_total_soil_mm": wat_baseline(merged_row, "Total-Soil"),
                    "delta_total_soil_mm": wat_delta(merged_row, "Total-Soil"),
                    "candidate_soil_water_total_mm": wat_candidate(merged_row, "SoilWaterTotal"),
                    "baseline_soil_water_total_mm": wat_baseline(merged_row, "SoilWaterTotal"),
                    "delta_soil_water_total_mm": wat_delta(merged_row, "SoilWaterTotal"),
                    "trace_wb13_rm_mm": value(trace, "wb13_rm_mm"),
                    "trace_wb13_q_mm": value(trace, "wb13_q_mm"),
                    "trace_wb13_snow_water_mm": value(trace, "wb13_snow_water_mm"),
                    "trace_s_mm": mm(trace, "snow_s_m"),
                    "trace_routed_melt_mm": mm(trace, "snow_routed_melt_m"),
                    "trace_post_winter_rain_mm": mm(trace, "snow_post_winter_rain_m"),
                    "trace_raw_melt_sum_mm": mm(trace, "snow_hourly_melt_raw_sum_m"),
                    "trace_redistributed_melt_sum_mm": mm(trace, "snow_hourly_melt_sum_m"),
                    "trace_rain_retained_mm": mm(trace, "snow_hourly_rain_retained_sum_m"),
                    "trace_rain_released_mm": mm(trace, "snow_hourly_rain_released_sum_m"),
                    "trace_snowfall_we_mm": mm(trace, "snow_hourly_snowfall_water_equiv_sum_m"),
                    "trace_runtime_swe_before_mm": mm(trace, "snow_runtime_swe_before_m"),
                    "trace_runtime_swe_mm": mm(trace, "snow_runtime_swe_m"),
                    "trace_runtime_swe_delta_mm": mm(trace, "snow_runtime_swe_delta_m"),
                    "trace_runtime_depth_before_mm": mm(trace, "snow_runtime_depth_before_m"),
                    "trace_runtime_depth_mm": mm(trace, "snow_runtime_depth_m"),
                    "trace_runtime_density_before_kg_m3": value(
                        trace, "snow_runtime_density_before_kg_m3"
                    ),
                    "trace_runtime_density_kg_m3": value(trace, "snow_runtime_density_kg_m3"),
                    "trace_swe_closure_error_mm": mm(trace, "snow_runtime_swe_closure_error_m"),
                    "trace_wb12_infiltration_mm": mm(trace, "wb12_infiltration_m"),
                    "trace_partition_supply_mm": mm(trace, "wb12_partition_liquid_supply_m"),
                    "trace_partition_residual_before_q_mm": mm(
                        trace, "wb12_partition_residual_before_q_m"
                    ),
                    "trace_effective_conductivity_mm_h": None
                    if value(trace, "wb14_effective_conductivity_m_s") is None
                    else value(trace, "wb14_effective_conductivity_m_s") * 3_600_000.0,
                    "trace_matric_potential_mm": mm(trace, "wb14_matric_potential_m"),
                }
            )
    (reports / "hphys0293_target_depletion_rows.json").write_text(
        json.dumps(extracted, indent=2) + "\n", encoding="utf-8"
    )
    return extracted


def write_target_depletion_markdown(run_root: Path, rows: list[dict[str, Any]]) -> None:
    reports = run_root / "reports"
    table_rows = [
        [
            f"H{row['hillslope_id']}",
            row["year"],
            row["julian"],
            rounded(row["delta_q_mm"]),
            rounded(row["delta_rm_mm"]),
            rounded(row["delta_snow_water_mm"]),
            rounded(row["delta_total_soil_mm"]),
            rounded(row["delta_soil_water_total_mm"]),
            rounded(row["trace_runtime_swe_before_mm"]),
            rounded(row["trace_runtime_swe_mm"]),
            rounded(row["trace_runtime_swe_delta_mm"]),
            rounded(row["trace_s_mm"]),
            rounded(row["trace_raw_melt_sum_mm"]),
            rounded(row["trace_redistributed_melt_sum_mm"]),
            rounded(row["trace_routed_melt_mm"]),
            rounded(row["trace_rain_retained_mm"]),
            rounded(row["trace_rain_released_mm"]),
            rounded(row["trace_wb12_infiltration_mm"]),
            rounded(row["trace_partition_supply_mm"]),
            rounded(row["trace_partition_residual_before_q_mm"]),
            rounded(row["trace_swe_closure_error_mm"], 9),
        ]
        for row in rows
    ]
    markdown = "# HPHYS0293 Target Snow Depletion Rows\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Run root: `{run_root}`\n"
    markdown += f"- Target rows JSON: `{reports / 'hphys0293_target_depletion_rows.json'}`\n"
    markdown += f"- Trace status: `{reports / 'hphys0293_target_trace_status.tsv'}`\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Year",
            "Julian",
            "ΔQ",
            "ΔRM",
            "ΔSnow",
            "ΔTotal-Soil",
            "ΔSoilWaterTotal",
            "SWE Before",
            "SWE After",
            "ΔSWE",
            "S",
            "Raw Melt",
            "Redist Melt",
            "Routed Melt",
            "Rain Retained",
            "Rain Released",
            "WB12 Infil",
            "WB12 Supply",
            "Residual Before Q",
            "SWE Closure",
        ],
        table_rows,
    )
    markdown += "\n## Interpretation\n\n"
    markdown += (
        "- HPHYS0293 separates snow-producer magnitude/timing evidence from WB14 "
        "capacity evidence after HPHYS0292 `Q` closure.\n"
    )
    markdown += (
        "- `SWE Closure` is the trace-level snow-state accounting residual; non-zero "
        "values indicate missing producer evidence before storage attribution.\n"
    )
    markdown += (
        "- Corrected negative-melt carried-state residuals remain authority-preserving "
        "differences from the pinned comparator, not a reason to compensate WB18/WB19/WB17.\n"
    )
    (reports / "hphys0293_target_depletion_rows.md").write_text(markdown, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", required=True, type=Path)
    parser.add_argument("--trace-max-days", type=int, default=1_800)
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
    rows = extract_target_depletion_rows(args.run_root)
    write_target_depletion_markdown(args.run_root, rows)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
