#!/usr/bin/env python3
"""Run HPHYS0272 hourly radiation unit diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import sys
from pathlib import Path
from typing import Any


REPO = Path("/home/workdir/openWEPP")
HPHYS0267_SCRIPT = (
    REPO
    / "docs/work-packages/20260603-hphys0267-post-lateral-pre-swu-threshold-lineage-closure-001/artifacts/hphys0267_diagnostics.py"
)
TARGETED_HILLSLOPES = [1, 7, 39]
DAY36_SIM_INDEX = 36
LANGLEY_TO_MJ_PER_M2 = 0.04184


def load_hphys0267_module() -> Any:
    spec = importlib.util.spec_from_file_location("hphys0267_diagnostics", HPHYS0267_SCRIPT)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {HPHYS0267_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0267 = load_hphys0267_module()
HPHYS0265 = HPHYS0267.HPHYS0265


def markdown_table(headers: list[str], rows: list[list[Any]]) -> str:
    return HPHYS0265.markdown_table(headers, rows)


def run_command(
    name: str,
    cmd: list[str],
    logs_dir: Path,
    env: dict[str, str] | None = None,
) -> HPHYS0265.RunResult:
    return HPHYS0265.run_command(name, cmd, logs_dir, env)


def write_status(path: Path, rows: list[dict[str, Any]]) -> None:
    HPHYS0265.write_status(path, rows)


def trace_float(row: dict[str, Any] | None, name: str, scale: float = 1.0) -> float | None:
    return HPHYS0265.trace_float(row, name, scale)


def trace_map(row: dict[str, Any] | None, name: str) -> dict[str, float]:
    if row is None:
        return {}
    value = row.get(name)
    if not isinstance(value, dict):
        return {}
    return {str(key): float(item) for key, item in value.items()}


def map_value(row: dict[str, Any] | None, name: str, hour: int) -> float | None:
    return trace_map(row, name).get(f"{hour:04}")


def normalize_full_suite_summary_label(run_root: Path) -> None:
    summary_path = run_root / "reports/hillslope_semantic_summary.md"
    if not summary_path.exists():
        return
    summary = summary_path.read_text(encoding="utf-8")
    summary = summary.replace(
        "# HPHYS0267 Full 39 Semantic Summary",
        "# HPHYS0272 Full 39 Semantic Summary",
        1,
    )
    summary_path.write_text(summary, encoding="utf-8")


def candidate_baseline_merge(run_root: Path, hillslope_id: int) -> Any:
    return HPHYS0265.candidate_baseline_merge(
        run_root / f"hillslope_output/H{hillslope_id}.wat.parquet",
        HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet",
        candidate_year_offset=2012,
    )


def row_float(row: Any, name: str) -> float | None:
    return HPHYS0265.row_float(row, name)


def find_day_row(merged: Any, sim_day_index: int) -> Any | None:
    matches = merged[merged["sim_day_index_candidate"].astype(int) == sim_day_index]
    if matches.empty:
        return None
    return matches.iloc[0]


def wat_context(run_root: Path, hillslope_id: int, sim_day_index: int) -> dict[str, Any]:
    merged = candidate_baseline_merge(run_root, hillslope_id)
    day_row = find_day_row(merged, sim_day_index)
    max_row = HPHYS0265.max_crossing(merged)
    context: dict[str, Any] = {
        "comparison_year": None if day_row is None else int(day_row["_comparison_year"]),
        "julian": None if day_row is None else int(day_row["julian"]),
        "max_julian": int(max_row["julian"]),
        "max_abs_ep_diff_mm": row_float(max_row, "abs_ep_diff_mm"),
    }
    for symbol in ["Ep", "RM", "Snow-Water", "Total-Soil", "SoilWaterTotal", "Dp"]:
        if day_row is None:
            context[f"{symbol}_candidate"] = None
            context[f"{symbol}_baseline"] = None
            context[f"{symbol}_diff"] = None
            continue
        if symbol == "Total-Soil":
            candidate_name = "Total-Soil"
            baseline_name = "Total-Soil Water"
        else:
            candidate_name = f"{symbol}_candidate"
            baseline_name = f"{symbol}_baseline"
        candidate = row_float(day_row, candidate_name)
        baseline = row_float(day_row, baseline_name)
        context[f"{symbol}_candidate"] = candidate
        context[f"{symbol}_baseline"] = baseline
        context[f"{symbol}_diff"] = (
            candidate - baseline if candidate is not None and baseline is not None else None
        )
    return context


def classify_radiation_units(run_root: Path, hillslope_id: int) -> dict[str, Any]:
    trace_path = run_root / f"hillslope_output/H{hillslope_id}.hphys0272.trace.jsonl"
    trace_rows = HPHYS0265.load_trace_rows(trace_path)
    post_scheduler = HPHYS0265.find_trace_row(trace_rows, DAY36_SIM_INDEX, "post_scheduler", None)
    post_wb13 = HPHYS0265.find_trace_row(trace_rows, DAY36_SIM_INDEX, "post_wb13", None)
    radiation = trace_map(post_scheduler, "winter_hourly_rad_mj_m2")
    radiation_values = list(radiation.values())
    max_hour_key = None
    if radiation:
        max_hour_key = max(radiation, key=lambda hour: radiation[hour])
    max_hourly_radiation = max(radiation_values) if radiation_values else None
    sum_hourly_radiation = sum(radiation_values) if radiation_values else None
    physically_impossible_hour_count = sum(1 for value in radiation_values if value >= 10.0)
    langley_scale_artifact = any(value >= 50.0 for value in radiation_values)
    classification = (
        "RADIATION_UNIT_TRACE_MISSING"
        if not radiation_values
        else "LANGLEY_SCALE_RADIATION_ARTIFACT_PRESENT"
        if langley_scale_artifact
        else "HOURLY_RADIATION_MJ_SCALE_CONFIRMED"
    )

    return {
        "hillslope_id": hillslope_id,
        "classification": classification,
        "max_hour": max_hour_key,
        "max_hourly_rad_mj_m2": max_hourly_radiation,
        "sum_hourly_rad_mj_m2": sum_hourly_radiation,
        "physically_impossible_hour_count_ge_10": physically_impossible_hour_count,
        "langley_scale_artifact_ge_50": langley_scale_artifact,
        "daily_radmj_from_200_ly": 200.0 * LANGLEY_TO_MJ_PER_M2 if hillslope_id == 1 else None,
        "wb13_rm_mm": trace_float(post_wb13, "wb13_rm_mm"),
        "wb13_snow_water_mm": trace_float(post_wb13, "wb13_snow_water_mm"),
        "snow_runtime_swe_before_m": trace_float(post_wb13, "snow_runtime_swe_before_m"),
        "snow_runtime_swe_m": trace_float(post_wb13, "snow_runtime_swe_m"),
        "snow_s_m": trace_float(post_scheduler, "snow_s_m"),
        "max_hour_air_temp_c": (
            map_value(post_scheduler, "winter_hourly_air_temp_c", int(max_hour_key))
            if max_hour_key is not None
            else None
        ),
        "max_hour_dewpoint_c": (
            map_value(post_scheduler, "winter_hourly_dewpoint_c", int(max_hour_key))
            if max_hour_key is not None
            else None
        ),
        "max_hour_raw_melt_m": (
            map_value(post_scheduler, "snow_hourly_melt_raw_m", int(max_hour_key))
            if max_hour_key is not None
            else None
        ),
        "max_hour_redistributed_melt_m": (
            map_value(post_scheduler, "snow_hourly_melt_m", int(max_hour_key))
            if max_hour_key is not None
            else None
        ),
        "wat_context": wat_context(run_root, hillslope_id, DAY36_SIM_INDEX),
    }


def summarize_radiation_units(run_root: Path) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    reports.mkdir(parents=True, exist_ok=True)
    summaries = [classify_radiation_units(run_root, hillslope_id) for hillslope_id in TARGETED_HILLSLOPES]
    json_path = reports / "hphys0272_hourly_radiation_unit_classification.json"
    json_path.write_text(json.dumps(summaries, indent=2) + "\n", encoding="utf-8")

    rows = []
    context_rows = []
    for item in summaries:
        context = item["wat_context"]
        rows.append(
            [
                f"H{item['hillslope_id']}",
                item["classification"],
                item["max_hour"],
                item["max_hourly_rad_mj_m2"],
                item["sum_hourly_rad_mj_m2"],
                item["physically_impossible_hour_count_ge_10"],
                item["langley_scale_artifact_ge_50"],
                item["max_hour_air_temp_c"],
                item["max_hour_dewpoint_c"],
                item["max_hour_raw_melt_m"],
                item["max_hour_redistributed_melt_m"],
            ]
        )
        context_rows.append(
            [
                f"H{item['hillslope_id']}",
                context.get("comparison_year"),
                context.get("julian"),
                context.get("RM_candidate"),
                context.get("RM_baseline"),
                context.get("RM_diff"),
                context.get("Snow-Water_candidate"),
                context.get("Snow-Water_baseline"),
                context.get("Snow-Water_diff"),
                context.get("Ep_diff"),
                context.get("max_abs_ep_diff_mm"),
            ]
        )

    markdown = "# HPHYS0272 Hourly Radiation Unit Classification\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Root: `{run_root}`\n"
    markdown += f"- Anchor day: simulation day `{DAY36_SIM_INDEX}`.\n"
    markdown += f"- Classification JSON: `{json_path}`.\n\n"
    markdown += "## Targeted Radiation Metrics\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Classification",
            "Max Hour",
            "Max Rad MJ/m2/hr",
            "Σ Rad MJ/m2/day",
            "Hours >= 10",
            "Any >= 50",
            "Air C",
            "Dewpoint C",
            "Raw Melt m",
            "Redistributed Melt m",
        ],
        rows,
    )
    markdown += "\n## Day-36 WAT/Snowpack Context\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Year",
            "Julian",
            "Cand RM",
            "Base RM",
            "RM Diff",
            "Cand Snow-Water",
            "Base Snow-Water",
            "Snow-Water Diff",
            "Ep Diff",
            "Max Abs Ep Diff",
        ],
        context_rows,
    )
    markdown += "\n## Interpretation\n\n"
    markdown += (
        "- `HOURLY_RADIATION_MJ_SCALE_CONFIRMED` means no day-36 targeted hour "
        "retains the pre-fix `59+ MJ m^-2 h^-1` Langley-scale artifact.\n"
    )
    markdown += (
        "- Remaining WAT residuals after this classification belong to snowpack "
        "state, melt term, ET, storage, or publication lineage and should not be "
        "compensated by radiation clipping.\n"
    )
    (reports / "hphys0272_hourly_radiation_unit_classification.md").write_text(
        markdown, encoding="utf-8"
    )
    return summaries


def run_targeted_traces(
    run_root: Path,
    runs_dir: Path,
    output: Path,
    logs: Path,
    trace_max_days: int,
) -> int:
    rows = []
    for hillslope_id in TARGETED_HILLSLOPES:
        trace_path = output / f"H{hillslope_id}.hphys0272.trace.jsonl"
        result = run_command(
            f"H{hillslope_id}_trace",
            [
                str(HPHYS0265.HILL_BIN),
                "--run-dir",
                str(runs_dir),
                "--run-file",
                f"p{hillslope_id}_openwepp.run",
                "--output-dir",
                str(output),
                "--policy",
                "compat",
            ],
            logs / "targeted",
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
            write_status(run_root / "reports/targeted_trace_status.tsv", rows)
            return int(result.rc)
    write_status(run_root / "reports/targeted_trace_status.tsv", rows)
    summarize_radiation_units(run_root)
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", type=Path, required=True)
    parser.add_argument("--trace-max-days", type=int, default=180)
    parser.add_argument("--skip-full-suite", action="store_true")
    args = parser.parse_args()

    run_root = args.run_root
    reports = run_root / "reports"
    logs = run_root / "logs"
    output = run_root / "hillslope_output"
    reports.mkdir(parents=True, exist_ok=True)
    logs.mkdir(parents=True, exist_ok=True)
    output.mkdir(parents=True, exist_ok=True)

    runs_dir = HPHYS0267.copy_runfiles(run_root)

    for required in [
        HPHYS0265.WEPPPY_PYTHON,
        HPHYS0265.COMPARATOR,
        HPHYS0265.TOLERANCES,
        HPHYS0265.BASELINE_PARTITIONS,
    ]:
        HPHYS0265.require_path(required)

    build = run_command(
        "cargo_build_openwepp_cli_hill",
        ["cargo", "build", "-p", "openwepp-runner", "--bin", "openwepp-cli-hill"],
        logs,
    )
    write_status(
        reports / "build_status.tsv",
        [
            {
                "command": "cargo build -p openwepp-runner --bin openwepp-cli-hill",
                "rc": build.rc,
                "seconds": f"{build.seconds:.3f}",
                "stdout": build.stdout,
                "stderr": build.stderr,
            }
        ],
    )
    if build.rc != 0:
        return int(build.rc)

    targeted_rc = run_targeted_traces(
        run_root, runs_dir, output, logs, args.trace_max_days
    )
    if targeted_rc != 0:
        return int(targeted_rc)

    if not args.skip_full_suite:
        full_rc = HPHYS0267.run_full_hillslope_suite(run_root, runs_dir, output, logs)
        if full_rc != 0:
            return int(full_rc)
        normalize_full_suite_summary_label(run_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
