#!/usr/bin/env python3
"""Run HPHYS0271 day-36 melt-forcing lineage diagnostics."""

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
DAY36_TARGET_HILLSLOPE = 1
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


def trace_float(row: dict[str, Any] | None, name: str, scale: float = 1.0) -> float | None:
    return HPHYS0265.trace_float(row, name, scale)


def trace_map(row: dict[str, Any] | None, name: str) -> dict[str, float]:
    if row is None:
        return {}
    value = row.get(name)
    if not isinstance(value, dict):
        return {}
    return {str(key): float(item) for key, item in value.items()}


def map_value(row: dict[str, Any] | None, name: str, hour: str) -> float | None:
    values = trace_map(row, name)
    return values.get(hour)


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


def wat_context(row: Any | None) -> dict[str, float | None]:
    if row is None:
        return {}
    symbols = ["Ep", "RM", "Snow-Water", "Q", "Total-Soil", "SoilWaterTotal", "Dp"]
    context: dict[str, float | None] = {}
    for symbol in symbols:
        if symbol == "Total-Soil":
            candidate_name = "Total-Soil"
            baseline_name = "Total-Soil Water"
        else:
            candidate_name = f"{symbol}_candidate"
            baseline_name = f"{symbol}_baseline"
        candidate = row_float(row, candidate_name)
        baseline = row_float(row, baseline_name)
        context[f"{symbol}_candidate"] = candidate
        context[f"{symbol}_baseline"] = baseline
        context[f"{symbol}_diff"] = (
            candidate - baseline if candidate is not None and baseline is not None else None
        )
    return context


def hour_record(row: dict[str, Any] | None, hour: int) -> dict[str, float | str | None]:
    hour_key = f"{hour:04}"
    amelt = map_value(row, "snow_hourly_melt_amelt_in", hour_key)
    bmelt = map_value(row, "snow_hourly_melt_bmelt_in", hour_key)
    cmelt = map_value(row, "snow_hourly_melt_cmelt_in", hour_key)
    dmelt = map_value(row, "snow_hourly_melt_dmelt_in", hour_key)
    raw_melt_m = None
    if None not in (amelt, bmelt, cmelt, dmelt):
        raw_melt_m = 0.0254 * (float(amelt) + float(bmelt) + float(cmelt) + float(dmelt))
    return {
        "hour": hour_key,
        "amelt_in": amelt,
        "bmelt_in": bmelt,
        "cmelt_in": cmelt,
        "dmelt_in": dmelt,
        "raw_melt_reconstructed_m": raw_melt_m,
        "raw_melt_trace_m": map_value(row, "snow_hourly_melt_raw_m", hour_key),
        "redistributed_melt_m": map_value(row, "snow_hourly_melt_m", hour_key),
        "branch_active": map_value(row, "snow_hourly_melt_branch_active", hour_key),
        "air_temp_c": map_value(row, "winter_hourly_air_temp_c", hour_key),
        "dewpoint_c": map_value(row, "winter_hourly_dewpoint_c", hour_key),
        "rad_mj_m2": map_value(row, "winter_hourly_rad_mj_m2", hour_key),
        "cloud_fraction": map_value(row, "winter_hourly_cloud_fraction", hour_key),
        "wind_m_s": map_value(row, "winter_hourly_wind_m_s", hour_key),
        "hrtef_f": map_value(row, "snow_hourly_melt_hrtef_f", hour_key),
        "hrdtf_f": map_value(row, "snow_hourly_melt_hrdtf_f", hour_key),
        "vwmph": map_value(row, "snow_hourly_melt_vwmph", hour_key),
        "rainin": map_value(row, "snow_hourly_melt_rainin", hour_key),
        "wind_adjustment": map_value(row, "snow_hourly_melt_wind_adjustment", hour_key),
    }


def key_melt_hour(hours: list[dict[str, float | str | None]]) -> dict[str, float | str | None] | None:
    candidates = [item for item in hours if item.get("raw_melt_reconstructed_m") is not None]
    if not candidates:
        return None
    return max(candidates, key=lambda item: abs(float(item["raw_melt_reconstructed_m"])))


def classify_day36_lineage(run_root: Path, hillslope_id: int, sim_day_index: int) -> dict[str, Any]:
    merged = candidate_baseline_merge(run_root, hillslope_id)
    day_row = find_day_row(merged, sim_day_index)
    max_row = HPHYS0265.max_crossing(merged)
    trace_path = run_root / f"hillslope_output/H{hillslope_id}.hphys0271.trace.jsonl"
    trace_rows = HPHYS0265.load_trace_rows(trace_path)
    post_wb13 = HPHYS0265.find_trace_row(trace_rows, sim_day_index, "post_wb13", None)
    post_scheduler = HPHYS0265.find_trace_row(trace_rows, sim_day_index, "post_scheduler", None)
    prior_wb13 = HPHYS0265.find_trace_row(trace_rows, max(sim_day_index - 1, 1), "post_wb13", None)

    hours = [hour_record(post_scheduler, hour) for hour in range(1, 25)]
    key_hour = key_melt_hour(hours)
    reconstructed_sum = sum(
        float(item["raw_melt_reconstructed_m"])
        for item in hours
        if item.get("raw_melt_reconstructed_m") is not None
    )
    raw_trace_sum = trace_float(post_scheduler, "snow_hourly_melt_raw_sum_m")
    redistributed_sum = trace_float(post_scheduler, "snow_hourly_melt_sum_m")
    reconstruction_error = (
        reconstructed_sum - raw_trace_sum if raw_trace_sum is not None else None
    )
    context = wat_context(day_row)
    required_values = {
        "day_row": day_row,
        "post_scheduler": post_scheduler,
        "post_wb13": post_wb13,
        "snow_hourly_melt_amelt_in": trace_map(post_scheduler, "snow_hourly_melt_amelt_in"),
        "snow_hourly_melt_bmelt_in": trace_map(post_scheduler, "snow_hourly_melt_bmelt_in"),
        "snow_hourly_melt_cmelt_in": trace_map(post_scheduler, "snow_hourly_melt_cmelt_in"),
        "snow_hourly_melt_dmelt_in": trace_map(post_scheduler, "snow_hourly_melt_dmelt_in"),
        "winter_hourly_air_temp_c": trace_map(post_scheduler, "winter_hourly_air_temp_c"),
        "winter_hourly_dewpoint_c": trace_map(post_scheduler, "winter_hourly_dewpoint_c"),
        "winter_hourly_wind_m_s": trace_map(post_scheduler, "winter_hourly_wind_m_s"),
    }
    missing = [
        name
        for name, value in required_values.items()
        if value is None or (isinstance(value, dict) and not value)
    ]

    rm_diff = context.get("RM_diff")
    snow_water_diff = context.get("Snow-Water_diff")
    if missing:
        classification = "DAY36_MELT_TERM_TRACE_INCOMPLETE"
    elif reconstruction_error is not None and abs(reconstruction_error) > 1.0e-12:
        classification = "DAY36_MELT_TERM_RECONSTRUCTION_DEFECT"
    elif any(
        value is not None and abs(value) > SEMANTIC_DIFF_TOL_MM
        for value in (rm_diff, snow_water_diff)
    ):
        classification = "DAY36_MELT_TERMS_RECONSTRUCT_RAW_MELT_WITH_WAT_DIVERGENCE"
    else:
        classification = "DAY36_MELT_TERMS_RECONSTRUCT_RAW_MELT_CONTEXT_ONLY"

    return {
        "hillslope_id": hillslope_id,
        "classification": classification,
        "comparison_year": None if day_row is None else int(day_row["_comparison_year"]),
        "julian": None if day_row is None else int(day_row["julian"]),
        "candidate_sim_day_index": sim_day_index,
        "max_julian": int(max_row["julian"]),
        "max_abs_ep_diff_mm": row_float(max_row, "abs_ep_diff_mm"),
        "wat_context": context,
        "raw_melt_reconstructed_sum_m": reconstructed_sum,
        "raw_melt_trace_sum_m": raw_trace_sum,
        "redistributed_melt_sum_m": redistributed_sum,
        "raw_melt_reconstruction_error_m": reconstruction_error,
        "snow_s_m": trace_float(post_scheduler, "snow_s_m"),
        "snow_runtime_swe_before_m": trace_float(post_wb13, "snow_runtime_swe_before_m"),
        "snow_runtime_swe_m": trace_float(post_wb13, "snow_runtime_swe_m"),
        "snow_runtime_swe_delta_m": trace_float(post_wb13, "snow_runtime_swe_delta_m"),
        "snow_runtime_swe_closure_error_m": trace_float(post_wb13, "snow_runtime_swe_closure_error_m"),
        "wb13_rm_mm": trace_float(post_wb13, "wb13_rm_mm"),
        "wb13_snow_water_mm": trace_float(post_wb13, "wb13_snow_water_mm"),
        "prior_wb13_snow_water_mm": trace_float(prior_wb13, "wb13_snow_water_mm"),
        "key_hour": key_hour,
        "hours": hours,
        "missing_required_trace": missing,
    }


def summarize_day36_lineage(run_root: Path) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    reports.mkdir(parents=True, exist_ok=True)
    classifications = [
        classify_day36_lineage(run_root, DAY36_TARGET_HILLSLOPE, DAY36_SIM_INDEX),
        *[
            classify_day36_lineage(run_root, hillslope_id, DAY36_SIM_INDEX)
            for hillslope_id in TARGETED_HILLSLOPES
            if hillslope_id != DAY36_TARGET_HILLSLOPE
        ],
    ]
    json_path = reports / "hphys0271_day36_melt_forcing_classification.json"
    json_path.write_text(json.dumps(classifications, indent=2) + "\n", encoding="utf-8")

    summary_rows = []
    key_rows = []
    for item in classifications:
        context = item.get("wat_context", {})
        key_hour = item.get("key_hour") or {}
        summary_rows.append(
            [
                f"H{item['hillslope_id']}",
                item["classification"],
                item.get("comparison_year"),
                item.get("julian"),
                item.get("candidate_sim_day_index"),
                context.get("RM_candidate"),
                context.get("RM_baseline"),
                context.get("RM_diff"),
                context.get("Snow-Water_candidate"),
                context.get("Snow-Water_baseline"),
                context.get("Snow-Water_diff"),
                item.get("raw_melt_reconstructed_sum_m"),
                item.get("raw_melt_trace_sum_m"),
                item.get("redistributed_melt_sum_m"),
                item.get("raw_melt_reconstruction_error_m"),
                item.get("snow_runtime_swe_before_m"),
                item.get("snow_runtime_swe_m"),
                item.get("snow_runtime_swe_closure_error_m"),
            ]
        )
        key_rows.append(
            [
                f"H{item['hillslope_id']}",
                key_hour.get("hour"),
                key_hour.get("raw_melt_reconstructed_m"),
                key_hour.get("raw_melt_trace_m"),
                key_hour.get("redistributed_melt_m"),
                key_hour.get("amelt_in"),
                key_hour.get("bmelt_in"),
                key_hour.get("cmelt_in"),
                key_hour.get("dmelt_in"),
                key_hour.get("air_temp_c"),
                key_hour.get("dewpoint_c"),
                key_hour.get("rad_mj_m2"),
                key_hour.get("cloud_fraction"),
                key_hour.get("wind_m_s"),
                key_hour.get("branch_active"),
            ]
        )

    markdown = "# HPHYS0271 Day-36 Melt-Forcing Lineage Classification\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Root: `{run_root}`\n"
    markdown += f"- Anchor: H{DAY36_TARGET_HILLSLOPE} simulation day `{DAY36_SIM_INDEX}` from HPHYS0270 review.\n"
    markdown += f"- Classification JSON: `{json_path}`.\n\n"
    markdown += "## Day-36 WAT/Snowpack Context\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Classification",
            "Year",
            "Julian",
            "Day",
            "Cand RM",
            "Base RM",
            "RM Diff",
            "Cand Snow-Water",
            "Base Snow-Water",
            "Snow-Water Diff",
            "Σ Reconstructed Raw Melt",
            "Trace Raw Melt Sum",
            "Redistributed Melt Sum",
            "Reconstruction Error",
            "Pre SWE",
            "Runtime SWE",
            "SWE Closure Error",
        ],
        summary_rows,
    )
    markdown += "\n## Highest-Magnitude Hourly Raw Melt Term\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Hour",
            "Raw Reconstructed",
            "Raw Trace",
            "Redistributed",
            "amelt",
            "bmelt",
            "cmelt",
            "dmelt",
            "Air C",
            "Dewpoint C",
            "Rad MJ/m2",
            "Cloud",
            "Wind m/s",
            "Branch",
        ],
        key_rows,
    )
    markdown += "\n## Interpretation\n\n"
    markdown += (
        "- `DAY36_MELT_TERMS_RECONSTRUCT_RAW_MELT_WITH_WAT_DIVERGENCE` means "
        "the openWEPP trace now proves the published raw melt is exactly the "
        "sum of `melt.for` terms, but baseline WAT `RM`/`Snow-Water` still "
        "differs; continuation should compare forcing inputs and branch timing "
        "against baseline before touching WB13/WB17/storage publication.\n"
    )
    markdown += (
        "- `DAY36_MELT_TERM_RECONSTRUCTION_DEFECT` means the trace seam itself "
        "is internally inconsistent and must be fixed before physics diagnosis.\n"
    )
    (reports / "hphys0271_day36_melt_forcing_classification.md").write_text(
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
        "# HPHYS0271 Full 39 Semantic Summary",
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
        trace_path = output / f"H{hillslope_id}.hphys0271.trace.jsonl"
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
    summarize_day36_lineage(run_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
