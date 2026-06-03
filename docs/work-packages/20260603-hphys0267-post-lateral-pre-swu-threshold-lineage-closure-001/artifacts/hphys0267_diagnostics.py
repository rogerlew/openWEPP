#!/usr/bin/env python3
"""Run HPHYS0267 post-lateral/pre-SWU threshold-lineage diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import math
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path
from typing import Any


REPO = Path("/home/workdir/openWEPP")
HPHYS0266_SCRIPT = (
    REPO
    / "docs/work-packages/20260603-hphys0266-layer-storage-lateral-snow-coupling-closure-001/artifacts/hphys0266_diagnostics.py"
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
FIRST_EP_THRESHOLD_MM = 0.05
LARGE_EP_THRESHOLD_MM = 1.0
IDENTITY_TOLERANCE_MM = 1.0e-6
LAYER_TOLERANCE_M = 1.0e-9
ACTIVITY_TOLERANCE = 1.0e-12


def load_hphys0266_module() -> Any:
    spec = importlib.util.spec_from_file_location("hphys0266_diagnostics", HPHYS0266_SCRIPT)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {HPHYS0266_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0266 = load_hphys0266_module()
HPHYS0265 = HPHYS0266.HPHYS0265


def markdown_table(headers: list[str], rows: list[list[Any]]) -> str:
    return HPHYS0265.markdown_table(headers, rows)


def run_command(
    name: str,
    cmd: list[str],
    logs_dir: Path,
    env: dict[str, str] | None = None,
) -> HPHYS0265.RunResult:
    return HPHYS0265.run_command(name, cmd, logs_dir, env)


def layer_map(row: dict[str, Any] | None, name: str) -> dict[str, float]:
    if row is None:
        return {}
    value = row.get(name)
    if not isinstance(value, dict):
        return {}
    return {str(layer_id): float(layer_value) for layer_id, layer_value in value.items()}


def active_layer_ids(row: dict[str, Any] | None, name: str, threshold: float) -> list[str]:
    return sorted(
        layer_id for layer_id, value in layer_map(row, name).items() if value > threshold
    )


def layer_preview(layer_ids: list[str]) -> str:
    return ",".join(layer_ids) if layer_ids else "none"


def trace_float(row: dict[str, Any] | None, name: str, scale: float = 1.0) -> float | None:
    return HPHYS0265.trace_float(row, name, scale)


def trace_layers_preview(row: dict[str, Any] | None, name: str, limit: int = 9) -> str:
    return HPHYS0265.trace_layers_preview(row, name, limit)


def first_crossing_rows(run_root: Path, hillslope_id: int) -> tuple[Any, Any, Any]:
    candidate_wat = run_root / f"hillslope_output/H{hillslope_id}.wat.parquet"
    baseline_wat = HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet"
    merged = HPHYS0265.candidate_baseline_merge(
        candidate_wat, baseline_wat, candidate_year_offset=2012
    )
    first = HPHYS0265.first_crossing(merged, FIRST_EP_THRESHOLD_MM)
    first_large = HPHYS0265.first_crossing(merged, LARGE_EP_THRESHOLD_MM)
    max_row = HPHYS0265.max_crossing(merged)
    return first, first_large, max_row


def material_context_symbols(row: Any) -> list[str]:
    return HPHYS0265.material_context_symbols(row)


def classify_threshold_lineage(
    hillslope_id: int,
    run_root: Path,
) -> dict[str, Any]:
    first, first_large, max_row = first_crossing_rows(run_root, hillslope_id)
    if first is None:
        return {
            "hillslope_id": hillslope_id,
            "classification": "NO_EP_THRESHOLD_CROSSING",
            "max_abs_ep_diff_mm": HPHYS0265.row_float(max_row, "abs_ep_diff_mm"),
        }

    trace_path = run_root / f"hillslope_output/H{hillslope_id}.hphys0267.trace.jsonl"
    trace_rows = HPHYS0265.load_trace_rows(trace_path)
    sim_day_index = int(first["sim_day_index_candidate"])
    pre_lateral = HPHYS0265.find_trace_row(
        trace_rows, sim_day_index, "post_phase", "percolation_deep_seepage"
    )
    post_lateral = HPHYS0265.find_trace_row(
        trace_rows, sim_day_index, "post_phase", "lateral_transfer"
    )
    post_swu = HPHYS0265.find_trace_row(
        trace_rows, sim_day_index, "post_phase", "plant_root_uptake"
    )

    pre_theta = layer_map(pre_lateral, "wb18_theta_layers_m")
    post_lateral_theta = layer_map(post_lateral, "wb18_theta_layers_m")
    post_swu_theta = layer_map(post_swu, "wb18_theta_layers_m")
    fzdrfc = layer_map(post_lateral, "wb19_fzdrfc_layers_m")
    drfc = layer_map(post_lateral, "wb19_drfc_layers_m")
    frzw = layer_map(post_lateral, "wb19_frzw_layers_m")
    fc = layer_map(post_lateral, "wb18_fc_layers_m")
    coca = layer_map(post_lateral, "wb19_coca_layers")
    ul = layer_map(post_swu, "wb18_ul_layers_m")
    stress_threshold = layer_map(post_swu, "wb17_swu_stress_threshold_layers_m")
    stress_ratio = layer_map(post_swu, "wb17_swu_storage_to_threshold_layers")
    withdrawals = layer_map(post_lateral, "wb19_lateral_withdrawal_layers_m")
    capacity_active = active_layer_ids(
        post_lateral, "wb19_lateral_capacity_active_count_layers", ACTIVITY_TOLERANCE
    )
    conductivity_active = active_layer_ids(
        post_lateral, "wb19_lateral_conductivity_active_count_layers", ACTIVITY_TOLERANCE
    )
    withdrawal_layers = sorted(
        layer_id for layer_id, value in withdrawals.items() if value > ACTIVITY_TOLERANCE
    )
    stress_layers = sorted(
        layer_id for layer_id, value in stress_ratio.items() if value < 1.0 - 1.0e-6
    )
    inactive_withdrawal_layers = sorted(
        layer_id for layer_id in withdrawal_layers if layer_id not in set(capacity_active)
    )

    pre_post_delta_errors = []
    threshold_excess_rows = []
    for layer_id in sorted(set(pre_theta) | set(post_lateral_theta) | set(withdrawals) | set(fzdrfc)):
        before = pre_theta.get(layer_id)
        after = post_lateral_theta.get(layer_id)
        withdrawal = withdrawals.get(layer_id, 0.0)
        threshold = fzdrfc.get(layer_id)
        delta_error = None
        if before is not None and after is not None:
            delta_error = before - after - withdrawal
            if withdrawal > ACTIVITY_TOLERANCE and abs(delta_error) > LAYER_TOLERANCE_M:
                pre_post_delta_errors.append(layer_id)
        available_above_threshold = None
        if before is not None and threshold is not None:
            available_above_threshold = max(before - threshold, 0.0)
        threshold_excess_rows.append(
            {
                "layer": layer_id,
                "pre_lateral_theta_m": before,
                "post_lateral_theta_m": after,
                "post_swu_theta_m": post_swu_theta.get(layer_id),
                "withdrawal_m": withdrawal,
                "fc_m": fc.get(layer_id),
                "coca": coca.get(layer_id),
                "drfc_m": drfc.get(layer_id),
                "frzw_m": frzw.get(layer_id, 0.0),
                "fzdrfc_m": threshold,
                "available_above_fzdrfc_m": available_above_threshold,
                "ul_m": ul.get(layer_id),
                "stress_threshold_m": stress_threshold.get(layer_id),
                "stress_ratio": stress_ratio.get(layer_id),
                "delta_error_m": delta_error,
            }
        )

    inactive_withdrawal_authorized = all(
        (
            pre_theta.get(layer_id, 0.0) - fzdrfc.get(layer_id, math.inf)
        )
        > ACTIVITY_TOLERANCE
        for layer_id in inactive_withdrawal_layers
    )
    delta_closed = not pre_post_delta_errors
    wb17_identity_closed = HPHYS0266.classify_first_divergence(
        hillslope_id,
        HPHYS0265.candidate_baseline_merge(
            run_root / f"hillslope_output/H{hillslope_id}.wat.parquet",
            HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet",
            candidate_year_offset=2012,
        ),
        trace_rows,
    ).get("wb17_identity_closed")
    lateral_identity_closed = HPHYS0266.classify_first_divergence(
        hillslope_id,
        HPHYS0265.candidate_baseline_merge(
            run_root / f"hillslope_output/H{hillslope_id}.wat.parquet",
            HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet",
            candidate_year_offset=2012,
        ),
        trace_rows,
    ).get("lateral_realized_identity_closed")

    missing_required_trace = [
        name
        for name, row in {
            "pre_lateral": pre_lateral,
            "post_lateral": post_lateral,
            "post_swu": post_swu,
        }.items()
        if row is None
    ]
    if missing_required_trace:
        classification = "THRESHOLD_TRACE_INCOMPLETE"
    elif not delta_closed:
        classification = "PRE_POST_LATERAL_LAYER_DELTA_DIVERGENCE"
    elif inactive_withdrawal_layers and inactive_withdrawal_authorized:
        classification = "BASELINE_TOPDOWN_WITHDRAWAL_FROM_NONACTIVE_CAPACITY_LAYER"
    elif wb17_identity_closed and lateral_identity_closed:
        classification = "THRESHOLD_LINEAGE_IDENTITIES_CLOSED_CONTEXT_ONLY"
    else:
        classification = "THRESHOLD_LINEAGE_REQUIRES_FURTHER_REVIEW"

    return {
        "hillslope_id": hillslope_id,
        "classification": classification,
        "comparison_year": int(first["_comparison_year"]),
        "julian": int(first["julian"]),
        "candidate_sim_day_index": sim_day_index,
        "candidate_ep_mm": HPHYS0265.row_float(first, "Ep_candidate"),
        "baseline_ep_mm": HPHYS0265.row_float(first, "Ep_baseline"),
        "ep_diff_mm": HPHYS0265.row_float(first, "ep_diff_mm"),
        "first_large_julian": None if first_large is None else int(first_large["julian"]),
        "max_julian": int(max_row["julian"]),
        "max_abs_ep_diff_mm": HPHYS0265.row_float(max_row, "abs_ep_diff_mm"),
        "wb17_identity_closed": wb17_identity_closed,
        "lateral_identity_closed": lateral_identity_closed,
        "delta_closed": delta_closed,
        "pre_post_delta_error_layers": layer_preview(pre_post_delta_errors),
        "capacity_active_layers": layer_preview(capacity_active),
        "conductivity_active_layers": layer_preview(conductivity_active),
        "withdrawal_layers": layer_preview(withdrawal_layers),
        "inactive_withdrawal_layers": layer_preview(inactive_withdrawal_layers),
        "inactive_withdrawal_authorized": inactive_withdrawal_authorized,
        "stress_layers": layer_preview(stress_layers),
        "material_context_symbols": material_context_symbols(first),
        "threshold_excess_rows": threshold_excess_rows,
        "post_lateral_drfc_preview": trace_layers_preview(post_lateral, "wb19_drfc_layers_m"),
        "post_lateral_fzdrfc_preview": trace_layers_preview(post_lateral, "wb19_fzdrfc_layers_m"),
        "post_swu_stress_ratio_preview": trace_layers_preview(
            post_swu, "wb17_swu_storage_to_threshold_layers"
        ),
        "missing_required_trace": missing_required_trace,
    }


def summarize_threshold_lineage(run_root: Path) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    classifications = [
        classify_threshold_lineage(hillslope_id, run_root) for hillslope_id in TARGETED_HILLSLOPES
    ]
    json_path = reports / "hphys0267_threshold_lineage_classification.json"
    json_path.write_text(json.dumps(classifications, indent=2) + "\n", encoding="utf-8")

    summary_rows = []
    lineage_rows = []
    layer_rows = []
    for item in classifications:
        summary_rows.append(
            [
                f"H{item['hillslope_id']}",
                item["classification"],
                item.get("comparison_year"),
                item.get("julian"),
                item.get("candidate_sim_day_index"),
                item.get("candidate_ep_mm"),
                item.get("baseline_ep_mm"),
                item.get("ep_diff_mm"),
                item.get("first_large_julian"),
                item.get("max_julian"),
                item.get("max_abs_ep_diff_mm"),
            ]
        )
        lineage_rows.append(
            [
                f"H{item['hillslope_id']}",
                item.get("wb17_identity_closed"),
                item.get("lateral_identity_closed"),
                item.get("delta_closed"),
                item.get("capacity_active_layers"),
                item.get("conductivity_active_layers"),
                item.get("withdrawal_layers"),
                item.get("inactive_withdrawal_layers"),
                item.get("inactive_withdrawal_authorized"),
                item.get("stress_layers"),
                ", ".join(item.get("material_context_symbols", [])),
            ]
        )
        for layer in item.get("threshold_excess_rows", []):
            if (
                abs(layer.get("withdrawal_m") or 0.0) <= ACTIVITY_TOLERANCE
                and (layer.get("stress_ratio") is None or layer["stress_ratio"] >= 1.0)
            ):
                continue
            layer_rows.append(
                [
                    f"H{item['hillslope_id']}",
                    layer.get("layer"),
                    layer.get("pre_lateral_theta_m"),
                    layer.get("post_lateral_theta_m"),
                    layer.get("post_swu_theta_m"),
                    layer.get("withdrawal_m"),
                    layer.get("drfc_m"),
                    layer.get("frzw_m"),
                    layer.get("fzdrfc_m"),
                    layer.get("available_above_fzdrfc_m"),
                    layer.get("ul_m"),
                    layer.get("stress_threshold_m"),
                    layer.get("stress_ratio"),
                    layer.get("delta_error_m"),
                ]
            )

    markdown = "# HPHYS0267 Threshold-Lineage Classification\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Root: `{run_root}`\n"
    markdown += f"- Threshold: first `|candidate Ep - baseline Ep| > {FIRST_EP_THRESHOLD_MM} mm`.\n"
    markdown += f"- Classification JSON: `{json_path}`.\n\n"
    markdown += "## First Divergence Summary\n\n"
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
            "First >1mm Julian",
            "Max Julian",
            "Max Abs Ep Diff",
        ],
        summary_rows,
    )
    markdown += "\n## Threshold Lineage Summary\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "WB17 Closed",
            "WB19 Closed",
            "Pre/Post Δ Closed",
            "Capacity Active",
            "Conductivity Active",
            "Withdrawal",
            "Inactive Withdrawal",
            "Inactive Authorized",
            "Stress Layers",
            "Material Context",
        ],
        lineage_rows,
    )
    markdown += "\n## Relevant Layer Rows\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Layer",
            "PreLat θ",
            "PostLat θ",
            "PostSWU θ",
            "Withdrawal",
            "drfc",
            "frzw",
            "fzdrfc",
            "Pre θ-fzdrfc",
            "UL",
            "Stress Threshold",
            "Stress Ratio",
            "Δ Error",
        ],
        layer_rows,
    )
    markdown += "\n## Interpretation\n\n"
    markdown += (
        "- Pinned `watbal_hourly.for:774-824` withdraws realized `latqcc` "
        "top-down from any layer with `st(jj)>fzdrfc`, after computing "
        "potential/capacity over active lateral layers.\n"
    )
    markdown += (
        "- Therefore withdrawal from a non-capacity-active layer is not itself "
        "a defect when the pre-lateral storage is above `fzdrfc` and pre/post "
        "layer deltas reconcile.\n"
    )
    for item in classifications:
        markdown += (
            f"- H{item['hillslope_id']} previews: drfc "
            f"`{item.get('post_lateral_drfc_preview', '')}`; fzdrfc "
            f"`{item.get('post_lateral_fzdrfc_preview', '')}`; stress ratios "
            f"`{item.get('post_swu_stress_ratio_preview', '')}`.\n"
        )
    (reports / "hphys0267_threshold_lineage_classification.md").write_text(
        markdown, encoding="utf-8"
    )
    return classifications


def write_status(path: Path, rows: list[dict[str, Any]]) -> None:
    HPHYS0265.write_status(path, rows)


def copy_runfiles(run_root: Path) -> Path:
    return HPHYS0265.copy_runfiles(run_root)


def run_semantics(run_root: Path) -> None:
    reports = run_root / "reports"
    semantic_dir = reports / "semantic_reports"
    semantic_dir.mkdir(parents=True, exist_ok=True)
    status_rows = []
    summary: dict[str, dict[str, Any]] = {}
    for hillslope_id in range(1, 40):
        report_json = semantic_dir / f"H{hillslope_id}.semantic.json"
        command = [
            str(HPHYS0265.WEPPPY_PYTHON),
            str(HPHYS0265.COMPARATOR),
            "--baseline-wat",
            str(HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet"),
            "--candidate-wat",
            str(run_root / f"hillslope_output/H{hillslope_id}.wat.parquet"),
            "--report-json",
            str(report_json),
            "--candidate-year-offset",
            "2012",
            "--tolerance-config",
            str(HPHYS0265.TOLERANCES),
        ]
        result = run_command(f"semantic_H{hillslope_id}", command, run_root / "logs/semantic")
        semantic_pass = False
        common_rows = None
        if report_json.exists():
            data = json.loads(report_json.read_text(encoding="utf-8"))
            comparison = data["comparison"]
            semantic_pass = bool(comparison["semantic_pass"])
            common_rows = int(comparison["common_row_count"])
            for stat in comparison["column_stats"]:
                column = stat["column"]
                entry = summary.setdefault(
                    column,
                    {
                        "hillslope_fail_count": 0,
                        "total_fail_count": 0,
                        "mean_abs_diff_values": [],
                        "max_abs_diff": 0.0,
                    },
                )
                if not stat["pass"]:
                    entry["hillslope_fail_count"] += 1
                entry["total_fail_count"] += int(stat["fail_count"])
                entry["mean_abs_diff_values"].append(float(stat["mean_abs_diff"]))
                entry["max_abs_diff"] = max(
                    entry["max_abs_diff"], float(stat["max_abs_diff"])
                )
        status_rows.append(
            {
                "hillslope_id": hillslope_id,
                "rc": result.rc,
                "semantic_pass": semantic_pass,
                "common_rows": common_rows,
                "report_json": report_json,
            }
        )
    write_status(reports / "semantic_status.tsv", status_rows)

    summary_rows = []
    for column, entry in sorted(summary.items()):
        values = entry["mean_abs_diff_values"]
        summary_rows.append(
            {
                "column": column,
                "hillslope_fail_count": entry["hillslope_fail_count"],
                "total_fail_count": entry["total_fail_count"],
                "mean_abs_diff_mean": sum(values) / len(values) if values else 0.0,
                "max_abs_diff": entry["max_abs_diff"],
            }
        )
    (reports / "hillslope_semantic_summary.json").write_text(
        json.dumps(summary_rows, indent=2) + "\n", encoding="utf-8"
    )
    by_column = {row["column"]: row for row in summary_rows}
    selected_rows = []
    for symbol in SELECTED_SYMBOLS:
        row = by_column[symbol]
        selected_rows.append(
            [
                symbol,
                f"{39 - row['hillslope_fail_count']}/39",
                row["total_fail_count"],
                row["mean_abs_diff_mean"],
                row["max_abs_diff"],
            ]
        )
    markdown = "# HPHYS0267 Full 39 Semantic Summary\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Root: `{run_root}`\n"
    markdown += f"- Runtime status: `{reports / 'hillslope_batch_status.tsv'}`\n"
    markdown += f"- Semantic status: `{reports / 'semantic_status.tsv'}`\n"
    markdown += f"- Semantic pass: `{sum(1 for row in status_rows if row['semantic_pass'])}/39`\n\n"
    markdown += markdown_table(
        ["Symbol", "Pass Hillslopes", "Total Fail Count", "Mean Abs Diff Mean", "Max Abs Diff"],
        selected_rows,
    )
    (reports / "hillslope_semantic_summary.md").write_text(markdown, encoding="utf-8")


def run_targeted_traces(
    run_root: Path,
    runs_dir: Path,
    output: Path,
    logs: Path,
    trace_max_days: int,
) -> int:
    reports = run_root / "reports"
    trace_rows = []
    for hillslope_id in TARGETED_HILLSLOPES:
        trace_path = output / f"H{hillslope_id}.hphys0267.trace.jsonl"
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
        trace_rows.append(
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
            write_status(reports / "targeted_trace_status.tsv", trace_rows)
            return int(result.rc)
    write_status(reports / "targeted_trace_status.tsv", trace_rows)
    summarize_threshold_lineage(run_root)
    return 0


def run_full_hillslope_suite(run_root: Path, runs_dir: Path, output: Path, logs: Path) -> int:
    reports = run_root / "reports"
    batch_rows = []
    for hillslope_id in range(1, 40):
        result = run_command(
            f"H{hillslope_id}",
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
            logs / "hillslopes",
        )
        batch_rows.append(
            {
                "hillslope_id": hillslope_id,
                "rc": result.rc,
                "seconds": f"{result.seconds:.3f}",
                "stdout": result.stdout,
                "stderr": result.stderr,
            }
        )
    write_status(reports / "hillslope_batch_status.tsv", batch_rows)
    failed = [row for row in batch_rows if row["rc"] != 0]
    if failed:
        return int(failed[0]["rc"])
    run_semantics(run_root)
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", required=True, type=Path)
    parser.add_argument("--trace-max-days", type=int, default=130)
    parser.add_argument("--skip-full-suite", action="store_true")
    args = parser.parse_args()

    run_root = args.run_root
    reports = run_root / "reports"
    logs = run_root / "logs"
    output = run_root / "hillslope_output"
    reports.mkdir(parents=True, exist_ok=True)
    logs.mkdir(parents=True, exist_ok=True)
    output.mkdir(parents=True, exist_ok=True)
    runs_dir = copy_runfiles(run_root)

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

    targeted_rc = run_targeted_traces(run_root, runs_dir, output, logs, args.trace_max_days)
    if targeted_rc != 0:
        return int(targeted_rc)
    if args.skip_full_suite:
        return 0
    return run_full_hillslope_suite(run_root, runs_dir, output, logs)


if __name__ == "__main__":
    raise SystemExit(main())
