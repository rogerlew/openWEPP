#!/usr/bin/env python3
"""Run HPHYS0270 winter daily snowpack state closure diagnostics."""

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
SELECTED_SYMBOLS = [
    "Ep",
    "Total-Soil",
    "SoilWaterTotal",
    "Dp",
    "latqcc",
    "Q",
    "RM",
    "Snow-Water",
]
MATERIAL_EP_THRESHOLD_MM = 1.0
SEMANTIC_DIFF_TOL_MM = 1.0e-6


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


def trace_float(row: dict[str, Any] | None, name: str) -> float | None:
    return HPHYS0265.trace_float(row, name)


def candidate_baseline_merge(run_root: Path, hillslope_id: int) -> Any:
    return HPHYS0265.candidate_baseline_merge(
        run_root / f"hillslope_output/H{hillslope_id}.wat.parquet",
        HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet",
        candidate_year_offset=2012,
    )


def row_float(row: Any, name: str) -> float | None:
    return HPHYS0265.row_float(row, name)


def material_context(row: Any) -> dict[str, float | None]:
    return {
        "Ep_candidate": row_float(row, "Ep_candidate"),
        "Ep_baseline": row_float(row, "Ep_baseline"),
        "ep_diff_mm": row_float(row, "ep_diff_mm"),
        "RM_candidate": row_float(row, "RM_candidate"),
        "RM_baseline": row_float(row, "RM_baseline"),
        "Snow-Water_candidate": row_float(row, "Snow-Water_candidate"),
        "Snow-Water_baseline": row_float(row, "Snow-Water_baseline"),
        "Total-Soil_candidate": row_float(row, "Total-Soil_candidate"),
        "Total-Soil_baseline": row_float(row, "Total-Soil_baseline"),
        "Q_candidate": row_float(row, "Q_candidate"),
        "Q_baseline": row_float(row, "Q_baseline"),
    }


def context_diff_mm(context: dict[str, float | None], symbol: str) -> float | None:
    candidate = context.get(f"{symbol}_candidate")
    baseline = context.get(f"{symbol}_baseline")
    if candidate is None or baseline is None:
        return None
    return candidate - baseline


def classify_snowpack_lineage(hillslope_id: int, run_root: Path) -> dict[str, Any]:
    merged = candidate_baseline_merge(run_root, hillslope_id)
    first_material = HPHYS0265.first_crossing(merged, MATERIAL_EP_THRESHOLD_MM)
    max_row = HPHYS0265.max_crossing(merged)
    if first_material is None:
        return {
            "hillslope_id": hillslope_id,
            "classification": "NO_MATERIAL_EP_CROSSING",
            "max_abs_ep_diff_mm": row_float(max_row, "abs_ep_diff_mm"),
        }

    sim_day_index = int(first_material["sim_day_index_candidate"])
    trace_path = run_root / f"hillslope_output/H{hillslope_id}.hphys0270.trace.jsonl"
    trace_rows = HPHYS0265.load_trace_rows(trace_path)
    post_scheduler = HPHYS0265.find_trace_row(
        trace_rows, sim_day_index, "post_scheduler", None
    )
    post_wb13 = HPHYS0265.find_trace_row(trace_rows, sim_day_index, "post_wb13", None)
    prior_post_wb13 = HPHYS0265.find_trace_row(
        trace_rows, max(sim_day_index - 1, 1), "post_wb13", None
    )

    snow_closure_error = trace_float(post_wb13, "snow_runtime_swe_closure_error_m")
    hourly_rain = trace_float(post_scheduler, "snow_hourly_rain_sum_m")
    hourly_melt = trace_float(post_scheduler, "snow_hourly_melt_sum_m")
    hourly_melt_raw = trace_float(post_scheduler, "snow_hourly_melt_raw_sum_m")
    hourly_rain_retained = trace_float(post_scheduler, "snow_hourly_rain_retained_sum_m")
    hourly_snow_we = trace_float(post_scheduler, "snow_hourly_snowfall_water_equiv_sum_m")
    signed_s = trace_float(post_scheduler, "snow_s_m")
    runtime_swe_before = trace_float(post_wb13, "snow_runtime_swe_before_m")
    runtime_depth_before = trace_float(post_wb13, "snow_runtime_depth_before_m")
    runtime_density_before = trace_float(
        post_wb13, "snow_runtime_density_before_kg_m3"
    )
    runtime_settle_before = trace_float(
        post_wb13, "snow_runtime_settle_day_count_before"
    )
    runtime_swe = trace_float(post_wb13, "snow_runtime_swe_m")
    runtime_depth = trace_float(post_wb13, "snow_runtime_depth_m")
    runtime_density = trace_float(post_wb13, "snow_runtime_density_kg_m3")
    runtime_settle = trace_float(post_wb13, "snow_runtime_settle_day_count")
    runtime_swe_delta = trace_float(post_wb13, "snow_runtime_swe_delta_m")
    runtime_depth_delta = trace_float(post_wb13, "snow_runtime_depth_delta_m")
    runtime_density_delta = trace_float(
        post_wb13, "snow_runtime_density_delta_kg_m3"
    )
    runtime_settle_delta = trace_float(
        post_wb13, "snow_runtime_settle_day_count_delta"
    )
    precipitation = trace_float(post_wb13, "wb13_p_mm")
    rm = trace_float(post_wb13, "wb13_rm_mm")
    snow_water = trace_float(post_wb13, "wb13_snow_water_mm")
    prior_snow_water = trace_float(prior_post_wb13, "wb13_snow_water_mm")
    context = material_context(first_material)
    snow_water_diff = context_diff_mm(context, "Snow-Water")
    rm_diff = context_diff_mm(context, "RM")

    missing = [
        name
        for name, value in {
            "post_scheduler": post_scheduler,
            "post_wb13": post_wb13,
            "snow_runtime_swe_before_m": runtime_swe_before,
            "snow_runtime_depth_before_m": runtime_depth_before,
            "snow_runtime_density_before_kg_m3": runtime_density_before,
            "snow_runtime_settle_day_count_before": runtime_settle_before,
            "snow_runtime_swe_m": runtime_swe,
            "snow_runtime_depth_m": runtime_depth,
            "snow_runtime_density_kg_m3": runtime_density,
            "snow_runtime_settle_day_count": runtime_settle,
            "snow_runtime_swe_delta_m": runtime_swe_delta,
            "snow_runtime_depth_delta_m": runtime_depth_delta,
            "snow_runtime_density_delta_kg_m3": runtime_density_delta,
            "snow_runtime_settle_day_count_delta": runtime_settle_delta,
            "snow_hourly_rain_sum_m": hourly_rain,
            "snow_hourly_melt_sum_m": hourly_melt,
            "snow_hourly_melt_raw_sum_m": hourly_melt_raw,
            "snow_hourly_rain_retained_sum_m": hourly_rain_retained,
            "snow_hourly_snowfall_water_equiv_sum_m": hourly_snow_we,
            "snow_s_m": signed_s,
            "snow_runtime_swe_closure_error_m": snow_closure_error,
            "wb13_p_mm": precipitation,
            "wb13_rm_mm": rm,
            "wb13_snow_water_mm": snow_water,
        }.items()
        if value is None
    ]

    if missing:
        classification = "SNOWPACK_TRACE_INCOMPLETE"
    elif snow_closure_error is not None and abs(snow_closure_error) > 1.0e-9:
        classification = "SNOWPACK_RUNTIME_CLOSURE_DEFECT"
    elif any(
        diff is not None and abs(diff) > SEMANTIC_DIFF_TOL_MM
        for diff in (snow_water_diff, rm_diff)
    ):
        classification = "SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED"
    else:
        classification = "SNOWPACK_TRACE_CLOSED_CONTEXT_ONLY"

    return {
        "hillslope_id": hillslope_id,
        "classification": classification,
        "comparison_year": int(first_material["_comparison_year"]),
        "julian": int(first_material["julian"]),
        "candidate_sim_day_index": sim_day_index,
        "max_julian": int(max_row["julian"]),
        "max_abs_ep_diff_mm": row_float(max_row, "abs_ep_diff_mm"),
        "context": context,
        "rm_diff_mm": rm_diff,
        "snow_water_diff_mm": snow_water_diff,
        "snow_s_m": signed_s,
        "snow_hourly_rain_sum_m": hourly_rain,
        "snow_hourly_melt_sum_m": hourly_melt,
        "snow_hourly_melt_raw_sum_m": hourly_melt_raw,
        "snow_hourly_rain_retained_sum_m": hourly_rain_retained,
        "snow_hourly_snowfall_water_equiv_sum_m": hourly_snow_we,
        "snow_runtime_swe_before_m": runtime_swe_before,
        "snow_runtime_depth_before_m": runtime_depth_before,
        "snow_runtime_density_before_kg_m3": runtime_density_before,
        "snow_runtime_settle_day_count_before": runtime_settle_before,
        "snow_runtime_swe_m": runtime_swe,
        "snow_runtime_depth_m": runtime_depth,
        "snow_runtime_density_kg_m3": runtime_density,
        "snow_runtime_settle_day_count": runtime_settle,
        "snow_runtime_swe_delta_m": runtime_swe_delta,
        "snow_runtime_depth_delta_m": runtime_depth_delta,
        "snow_runtime_density_delta_kg_m3": runtime_density_delta,
        "snow_runtime_settle_day_count_delta": runtime_settle_delta,
        "snow_runtime_swe_closure_error_m": snow_closure_error,
        "wb13_p_mm": precipitation,
        "wb13_rm_mm": rm,
        "wb13_snow_water_mm": snow_water,
        "prior_wb13_snow_water_mm": prior_snow_water,
        "missing_required_trace": missing,
    }


def summarize_snowpack_lineage(run_root: Path) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    reports.mkdir(parents=True, exist_ok=True)
    classifications = [
        classify_snowpack_lineage(hillslope_id, run_root)
        for hillslope_id in TARGETED_HILLSLOPES
    ]
    json_path = reports / "hphys0270_snowpack_lineage_classification.json"
    json_path.write_text(json.dumps(classifications, indent=2) + "\n", encoding="utf-8")

    summary_rows = []
    snow_rows = []
    for item in classifications:
        context = item.get("context", {})
        summary_rows.append(
            [
                f"H{item['hillslope_id']}",
                item["classification"],
                item.get("comparison_year"),
                item.get("julian"),
                item.get("candidate_sim_day_index"),
                context.get("Ep_candidate"),
                context.get("Ep_baseline"),
                context.get("ep_diff_mm"),
                item.get("max_julian"),
                item.get("max_abs_ep_diff_mm"),
            ]
        )
        snow_rows.append(
            [
                f"H{item['hillslope_id']}",
                context.get("RM_candidate"),
                context.get("RM_baseline"),
                item.get("rm_diff_mm"),
                context.get("Snow-Water_candidate"),
                context.get("Snow-Water_baseline"),
                item.get("snow_water_diff_mm"),
                item.get("snow_s_m"),
                item.get("snow_hourly_rain_sum_m"),
                item.get("snow_hourly_melt_sum_m"),
                item.get("snow_hourly_melt_raw_sum_m"),
                item.get("snow_hourly_rain_retained_sum_m"),
                item.get("snow_hourly_snowfall_water_equiv_sum_m"),
                item.get("snow_runtime_swe_before_m"),
                item.get("snow_runtime_depth_before_m"),
                item.get("snow_runtime_density_before_kg_m3"),
                item.get("snow_runtime_settle_day_count_before"),
                item.get("snow_runtime_swe_m"),
                item.get("snow_runtime_depth_m"),
                item.get("snow_runtime_density_kg_m3"),
                item.get("snow_runtime_settle_day_count"),
                item.get("snow_runtime_swe_delta_m"),
                item.get("snow_runtime_depth_delta_m"),
                item.get("snow_runtime_density_delta_kg_m3"),
                item.get("snow_runtime_settle_day_count_delta"),
                item.get("snow_runtime_swe_closure_error_m"),
            ]
        )

    markdown = "# HPHYS0270 Winter Daily Snowpack State Closure Classification\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Root: `{run_root}`\n"
    markdown += f"- Threshold: first `|candidate Ep - baseline Ep| > {MATERIAL_EP_THRESHOLD_MM} mm`.\n"
    markdown += f"- Classification JSON: `{json_path}`.\n\n"
    markdown += "## First Material Divergence Summary\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Classification",
            "Year",
            "Julian",
            "Candidate Day",
            "Cand Ep",
            "Base Ep",
            "Ep Diff",
            "Max Julian",
            "Max Abs Ep Diff",
        ],
        summary_rows,
    )
    markdown += "\n## Snowpack/RM Lineage\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Cand RM",
            "Base RM",
            "RM Diff",
            "Cand Snow-Water",
            "Base Snow-Water",
            "Snow-Water Diff",
            "S",
            "Hourly Rain Sum",
            "Hourly Melt Sum",
            "Hourly Raw Melt Sum",
            "Hourly Retained Rain Sum",
            "Hourly Snow WE Sum",
            "Pre SWE",
            "Pre Depth",
            "Pre Density",
            "Pre Settle Count",
            "Runtime SWE",
            "Runtime Depth",
            "Runtime Density",
            "Runtime Settle Count",
            "SWE Delta",
            "Depth Delta",
            "Density Delta",
            "Settle Count Delta",
            "SWE Closure Error",
        ],
        snow_rows,
    )
    markdown += "\n## Interpretation\n\n"
    markdown += (
        "- Classification uses the first material `Ep` divergence as the local "
        "snowpack context anchor.\n"
    )
    markdown += (
        "- `SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED` means openWEPP's "
        "internal snowpack closure reconciles but differs from baseline WAT "
        "`Snow-Water`/`RM`, so production correction requires a narrower "
        "baseline-lineage proof.\n"
    )
    (reports / "hphys0270_snowpack_lineage_classification.md").write_text(
        markdown, encoding="utf-8"
    )
    return classifications


def normalize_full_suite_summary_label(run_root: Path) -> None:
    summary_path = run_root / "reports/hillslope_semantic_summary.md"
    if not summary_path.exists():
        return
    summary = summary_path.read_text(encoding="utf-8")
    summary = summary.replace(
        "# HPHYS0267 Full 39 Semantic Summary",
        "# HPHYS0270 Full 39 Semantic Summary",
        1,
    )
    summary_path.write_text(summary, encoding="utf-8")


def run_targeted_traces(
    run_root: Path,
    runs_dir: Path,
    output: Path,
    logs: Path,
    trace_max_days: int,
) -> int:
    rows = []
    for hillslope_id in TARGETED_HILLSLOPES:
        trace_path = output / f"H{hillslope_id}.hphys0270.trace.jsonl"
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
            HPHYS0265.write_status(run_root / "reports/targeted_trace_status.tsv", rows)
            return int(result.rc)
    HPHYS0265.write_status(run_root / "reports/targeted_trace_status.tsv", rows)
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
    HPHYS0265.write_status(
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
        run_root, runs_dir, output, logs / "targeted", args.trace_max_days
    )
    if targeted_rc != 0:
        return int(targeted_rc)

    if not args.skip_full_suite:
        full_rc = HPHYS0267.run_full_hillslope_suite(run_root, runs_dir, output, logs)
        if full_rc != 0:
            return int(full_rc)
        normalize_full_suite_summary_label(run_root)
    summarize_snowpack_lineage(run_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
