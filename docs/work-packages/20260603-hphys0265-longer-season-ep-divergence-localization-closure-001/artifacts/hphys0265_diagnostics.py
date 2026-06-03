#!/usr/bin/env python3
"""Run HPHYS0265 first-large Ep divergence diagnostics and full 39 metrics."""

from __future__ import annotations

import argparse
import csv
import json
import math
import os
import shutil
import subprocess
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import pandas as pd


REPO = Path("/home/workdir/openWEPP")
WEPPPY_PYTHON = Path("/workdir/wepppy/.venv/bin/python")
SOURCE_RUNS = Path("/tmp/unpalatable_parity_20260529T192707Z/runs")
BASELINE_PARTITIONS = Path(
    "/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions"
)
COMPARATOR = REPO / "tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py"
TOLERANCES = REPO / "tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json"
HILL_BIN = REPO / "target/debug/openwepp-cli-hill"
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
STRESS_TOLERANCE = 1.0e-6


@dataclass(frozen=True)
class RunResult:
    name: str
    rc: int
    seconds: float
    stdout: Path
    stderr: Path


def run_command(
    name: str,
    cmd: list[str],
    logs_dir: Path,
    env: dict[str, str] | None = None,
) -> RunResult:
    logs_dir.mkdir(parents=True, exist_ok=True)
    stdout = logs_dir / f"{name}.stdout.log"
    stderr = logs_dir / f"{name}.stderr.log"
    merged_env = os.environ.copy()
    if env:
        merged_env.update(env)
    started = time.monotonic()
    with stdout.open("w", encoding="utf-8") as out, stderr.open("w", encoding="utf-8") as err:
        proc = subprocess.run(cmd, cwd=REPO, env=merged_env, stdout=out, stderr=err, check=False)
    return RunResult(
        name=name,
        rc=proc.returncode,
        seconds=time.monotonic() - started,
        stdout=stdout,
        stderr=stderr,
    )


def require_path(path: Path) -> None:
    if not path.exists():
        raise FileNotFoundError(path)


def copy_runfiles(run_root: Path) -> Path:
    require_path(SOURCE_RUNS)
    runs_dir = run_root / "runs"
    runs_dir.mkdir(parents=True, exist_ok=True)
    for path in SOURCE_RUNS.iterdir():
        if path.is_file():
            shutil.copy2(path, runs_dir / path.name)
    return runs_dir


def write_status(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not rows:
        path.write_text("", encoding="utf-8")
        return
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0].keys()), delimiter="\t")
        writer.writeheader()
        writer.writerows(rows)


def markdown_table(headers: list[str], rows: list[list[Any]]) -> str:
    def fmt(item: Any) -> str:
        if item is None:
            return ""
        if isinstance(item, float):
            if math.isnan(item):
                return "nan"
            return f"{item:.6f}"
        return str(item)

    lines = [
        "| " + " | ".join(headers) + " |",
        "| " + " | ".join(["---"] * len(headers)) + " |",
    ]
    for row in rows:
        lines.append("| " + " | ".join(fmt(item) for item in row) + " |")
    return "\n".join(lines) + "\n"


def load_trace_rows(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def row_float(row: pd.Series, name: str, default: float = float("nan")) -> float:
    if name in row and pd.notna(row[name]):
        return float(row[name])
    return default


def trace_float(row: dict[str, Any] | None, name: str, scale: float = 1.0) -> float | None:
    if row is None:
        return None
    value = row.get(name)
    if value is None:
        return None
    return float(value) * scale


def trace_string(row: dict[str, Any] | None, name: str) -> str | None:
    if row is None:
        return None
    value = row.get(name)
    if value is None:
        return None
    return str(value)


def trace_layers_sum_mm(row: dict[str, Any] | None, name: str) -> float | None:
    if row is None:
        return None
    layers = row.get(name)
    if not isinstance(layers, dict) or not layers:
        return None
    return sum(float(value) for value in layers.values()) * 1000.0


def trace_layers_min(row: dict[str, Any] | None, name: str) -> float | None:
    if row is None:
        return None
    layers = row.get(name)
    if not isinstance(layers, dict) or not layers:
        return None
    values = [float(value) for value in layers.values()]
    return min(values) if values else None


def trace_layers_below_count(row: dict[str, Any] | None, name: str, threshold: float) -> int | None:
    if row is None:
        return None
    layers = row.get(name)
    if not isinstance(layers, dict) or not layers:
        return None
    return sum(1 for value in layers.values() if float(value) < threshold)


def trace_layers_preview(row: dict[str, Any] | None, name: str, limit: int = 6) -> str:
    if row is None:
        return ""
    layers = row.get(name)
    if not isinstance(layers, dict) or not layers:
        return ""
    items = sorted((str(key), float(value)) for key, value in layers.items())
    return ", ".join(f"{key}={value:.6g}" for key, value in items[:limit])


def find_trace_row(
    rows: list[dict[str, Any]],
    sim_day_index: int,
    boundary: str,
    phase: str | None = None,
) -> dict[str, Any] | None:
    for row in rows:
        if int(row["sim_day_index"]) != sim_day_index:
            continue
        if row.get("boundary") != boundary:
            continue
        if row.get("phase") != phase:
            continue
        return row
    return None


def candidate_baseline_merge(
    candidate_wat: Path,
    baseline_wat: Path,
    candidate_year_offset: int,
) -> pd.DataFrame:
    candidate = pd.read_parquet(candidate_wat).copy()
    baseline = pd.read_parquet(baseline_wat).copy()
    candidate["_comparison_year"] = candidate["year"].astype(int) + candidate_year_offset
    baseline["_comparison_year"] = baseline["year"].astype(int)
    keys = ["_comparison_year", "julian", "ofe_id"]
    merged = candidate.merge(
        baseline,
        on=keys,
        suffixes=("_candidate", "_baseline"),
        how="inner",
    )
    merged["ep_diff_mm"] = merged["Ep_candidate"] - merged["Ep_baseline"]
    merged["abs_ep_diff_mm"] = merged["ep_diff_mm"].abs()
    return merged.sort_values(["_comparison_year", "julian", "ofe_id"]).reset_index(drop=True)


def first_crossing(merged: pd.DataFrame, threshold_mm: float) -> pd.Series | None:
    crossing = merged[merged["abs_ep_diff_mm"] > threshold_mm]
    if crossing.empty:
        return None
    return crossing.iloc[0]


def max_crossing(merged: pd.DataFrame) -> pd.Series:
    return merged.iloc[int(merged["abs_ep_diff_mm"].idxmax())]


def wat_delta(row: pd.Series, symbol: str) -> float:
    return wat_candidate(row, symbol) - wat_baseline(row, symbol)


def wat_candidate(row: pd.Series, symbol: str) -> float:
    if symbol == "Total-Soil":
        return row_float(row, "Total-Soil")
    candidate_name = f"{symbol}_candidate"
    if candidate_name in row:
        return row_float(row, candidate_name)
    return row_float(row, symbol)


def wat_baseline(row: pd.Series, symbol: str) -> float:
    if symbol == "Total-Soil":
        return row_float(row, "Total-Soil Water")
    baseline_name = f"{symbol}_baseline"
    if baseline_name in row:
        return row_float(row, baseline_name)
    return row_float(row, symbol)


def material_context_symbols(row: pd.Series) -> list[str]:
    thresholds = {
        "Total-Soil": 1.0,
        "SoilWaterTotal": 1.0,
        "Dp": 0.05,
        "latqcc": 0.1,
        "Q": 0.05,
        "RM": 0.05,
        "Snow-Water": 0.05,
    }
    material = []
    for symbol, threshold in thresholds.items():
        if abs(wat_delta(row, symbol)) > threshold:
            material.append(symbol)
    return material


def classify_first_divergence(
    hillslope_id: int,
    merged: pd.DataFrame,
    trace_rows: list[dict[str, Any]],
) -> dict[str, Any]:
    first = first_crossing(merged, FIRST_EP_THRESHOLD_MM)
    first_large = first_crossing(merged, LARGE_EP_THRESHOLD_MM)
    max_row = max_crossing(merged)
    if first is None:
        return {
            "hillslope_id": hillslope_id,
            "classification": "NO_EP_THRESHOLD_CROSSING",
            "first_ep_threshold_mm": FIRST_EP_THRESHOLD_MM,
            "max_abs_ep_diff_mm": row_float(max_row, "abs_ep_diff_mm"),
        }

    sim_day_index = int(first["sim_day_index_candidate"])
    trace_by_key = {
        "post_seed": find_trace_row(trace_rows, sim_day_index, "post_seed"),
        "evapotranspiration": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "evapotranspiration"
        ),
        "plant_root_uptake": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "plant_root_uptake"
        ),
        "percolation_deep_seepage": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "percolation_deep_seepage"
        ),
        "lateral_transfer": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "lateral_transfer"
        ),
        "runoff_reconciliation": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "runoff_reconciliation"
        ),
        "storage_reconciliation": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "storage_reconciliation"
        ),
        "post_scheduler": find_trace_row(trace_rows, sim_day_index, "post_scheduler"),
        "post_wb13": find_trace_row(trace_rows, sim_day_index, "post_wb13"),
    }
    identity_row = (
        trace_by_key["plant_root_uptake"]
        or trace_by_key["post_scheduler"]
        or trace_by_key["post_wb13"]
    )
    et_row = trace_by_key["evapotranspiration"] or identity_row

    pmet_ep_mm = trace_float(et_row, "pmet_ep_m", 1000.0)
    etp_mm = trace_float(et_row, "etp_m", 1000.0)
    final_ep_mm = trace_float(identity_row, "ep_m", 1000.0)
    ui_mm = trace_float(identity_row, "ui_m", 1000.0)
    ui_sum_mm = trace_layers_sum_mm(identity_row, "wb17_ui_layers_m")
    ws = trace_float(identity_row, "ws")
    ep_minus_ui_sum_mm = (
        final_ep_mm - ui_sum_mm if final_ep_mm is not None and ui_sum_mm is not None else None
    )
    ui_minus_sum_mm = ui_mm - ui_sum_mm if ui_mm is not None and ui_sum_mm is not None else None
    ws_minus_ep_over_etp = None
    if ws is not None and final_ep_mm is not None and etp_mm is not None and etp_mm > 0.0:
        ws_minus_ep_over_etp = ws - (final_ep_mm / etp_mm)

    missing_trace_keys = [key for key, row in trace_by_key.items() if row is None]
    identity_closed = (
        not missing_trace_keys
        and ep_minus_ui_sum_mm is not None
        and ui_minus_sum_mm is not None
        and ws_minus_ep_over_etp is not None
        and abs(ep_minus_ui_sum_mm) <= IDENTITY_TOLERANCE_MM
        and abs(ui_minus_sum_mm) <= IDENTITY_TOLERANCE_MM
        and abs(ws_minus_ep_over_etp) <= STRESS_TOLERANCE
    )
    seam_closed = (
        pmet_ep_mm is not None
        and etp_mm is not None
        and abs(etp_mm - pmet_ep_mm) <= IDENTITY_TOLERANCE_MM
    )
    swu_stress_limited = (
        etp_mm is not None
        and final_ep_mm is not None
        and final_ep_mm < etp_mm - IDENTITY_TOLERANCE_MM
    )
    material_symbols = material_context_symbols(first)

    if missing_trace_keys:
        classification = "FIRST_EP_TRACE_INCOMPLETE"
    elif not seam_closed and trace_string(et_row, "wb11_et_seed_branch") == "evappm_pmet":
        classification = "PMET_SEAM_REGRESSION"
    elif not identity_closed:
        classification = "WB17_INTERNAL_IDENTITY_DIVERGENCE"
    elif swu_stress_limited and material_symbols:
        classification = "WB17_IDENTITY_CLOSED_SWU_STRESS_LIMITED_WITH_STORAGE_CONTEXT"
    elif material_symbols:
        classification = "WB17_IDENTITY_CLOSED_COUPLED_STORAGE_SNOW_RUNOFF_CONTEXT"
    else:
        classification = "WB17_IDENTITY_CLOSED_UPSTREAM_DEMAND_OR_GROWTH_CONTEXT"

    return {
        "hillslope_id": hillslope_id,
        "classification": classification,
        "first_ep_threshold_mm": FIRST_EP_THRESHOLD_MM,
        "first_large_ep_threshold_mm": LARGE_EP_THRESHOLD_MM,
        "comparison_year": int(first["_comparison_year"]),
        "julian": int(first["julian"]),
        "candidate_sim_day_index": sim_day_index,
        "baseline_sim_day_index": int(first["sim_day_index_baseline"]),
        "candidate_ep_mm": row_float(first, "Ep_candidate"),
        "baseline_ep_mm": row_float(first, "Ep_baseline"),
        "ep_diff_mm": row_float(first, "ep_diff_mm"),
        "abs_ep_diff_mm": row_float(first, "abs_ep_diff_mm"),
        "first_large_comparison_year": None if first_large is None else int(first_large["_comparison_year"]),
        "first_large_julian": None if first_large is None else int(first_large["julian"]),
        "first_large_abs_ep_diff_mm": None if first_large is None else row_float(first_large, "abs_ep_diff_mm"),
        "max_comparison_year": int(max_row["_comparison_year"]),
        "max_julian": int(max_row["julian"]),
        "max_abs_ep_diff_mm": row_float(max_row, "abs_ep_diff_mm"),
        "wb11_et_seed_branch": trace_string(et_row, "wb11_et_seed_branch"),
        "pmet_ep_mm": pmet_ep_mm,
        "etp_mm": etp_mm,
        "final_ep_trace_mm": final_ep_mm,
        "ui_aggregate_mm": ui_mm,
        "ui_layer_sum_mm": ui_sum_mm,
        "ep_minus_ui_sum_mm": ep_minus_ui_sum_mm,
        "ui_aggregate_minus_layer_sum_mm": ui_minus_sum_mm,
        "ws": ws,
        "ws_minus_ep_over_etp": ws_minus_ep_over_etp,
        "seam_closed": seam_closed,
        "identity_closed": identity_closed,
        "swu_stress_limited": swu_stress_limited,
        "pl_lai": trace_float(identity_row, "pl_lai"),
        "pl_rtd": trace_float(identity_row, "pl_rtd"),
        "pl_pltol": trace_float(identity_row, "pl_pltol"),
        "pl_swu_effective_pltol": trace_float(identity_row, "pl_swu_effective_pltol"),
        "min_storage_to_threshold": trace_layers_min(
            identity_row, "wb17_swu_storage_to_threshold_layers"
        ),
        "stress_limited_layer_count": trace_layers_below_count(
            identity_row, "wb17_swu_storage_to_threshold_layers", 1.0
        ),
        "storage_to_threshold_preview": trace_layers_preview(
            identity_row, "wb17_swu_storage_to_threshold_layers"
        ),
        "ui_layers_preview": trace_layers_preview(identity_row, "wb17_ui_layers_m"),
        "wat_context": {
            symbol: {
                "candidate": wat_candidate(first, symbol),
                "baseline": wat_baseline(first, symbol),
                "diff": wat_delta(first, symbol),
            }
            for symbol in SELECTED_SYMBOLS
        },
        "material_context_symbols": material_symbols,
        "trace_rows_missing": missing_trace_keys,
        "trace_day_row_count": sum(1 for row in trace_rows if int(row["sim_day_index"]) == sim_day_index),
    }


def summarize_first_ep_divergence(run_root: Path) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    classifications: list[dict[str, Any]] = []
    for hillslope_id in TARGETED_HILLSLOPES:
        candidate_wat = run_root / f"hillslope_output/H{hillslope_id}.wat.parquet"
        baseline_wat = BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet"
        trace_path = run_root / f"hillslope_output/H{hillslope_id}.hphys0265.trace.jsonl"
        merged = candidate_baseline_merge(candidate_wat, baseline_wat, candidate_year_offset=2012)
        trace_rows = load_trace_rows(trace_path)
        classifications.append(classify_first_divergence(hillslope_id, merged, trace_rows))

    (reports / "hphys0265_first_ep_divergence_classification.json").write_text(
        json.dumps(classifications, indent=2) + "\n", encoding="utf-8"
    )

    summary_rows = []
    context_rows = []
    identity_rows = []
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
        identity_rows.append(
            [
                f"H{item['hillslope_id']}",
                item.get("wb11_et_seed_branch"),
                item.get("pmet_ep_mm"),
                item.get("etp_mm"),
                item.get("final_ep_trace_mm"),
                item.get("ui_layer_sum_mm"),
                item.get("ep_minus_ui_sum_mm"),
                item.get("ui_aggregate_minus_layer_sum_mm"),
                item.get("ws"),
                item.get("ws_minus_ep_over_etp"),
                item.get("min_storage_to_threshold"),
                item.get("stress_limited_layer_count"),
            ]
        )
        wat_context = item.get("wat_context", {})
        for symbol in SELECTED_SYMBOLS:
            symbol_context = wat_context.get(symbol, {})
            context_rows.append(
                [
                    f"H{item['hillslope_id']}",
                    symbol,
                    symbol_context.get("candidate"),
                    symbol_context.get("baseline"),
                    symbol_context.get("diff"),
                ]
            )

    md = "# HPHYS0265 First-Large Ep Divergence Classification\n\n"
    md += "Ran:\n\n"
    md += f"- Root: `{run_root}`\n"
    md += f"- Threshold: first `|candidate Ep - baseline Ep| > {FIRST_EP_THRESHOLD_MM} mm`.\n"
    md += f"- Trace window: package runner requested `OPENWEPP_HPHYS0245_TRACE_MAX_DAYS` before classification.\n"
    md += f"- Classification JSON: `{reports / 'hphys0265_first_ep_divergence_classification.json'}`.\n\n"
    md += "## First Divergence Summary\n\n"
    md += markdown_table(
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
    md += "\n## WB17/SWU Identity Surfaces\n\n"
    md += markdown_table(
        [
            "Hill",
            "Seed Branch",
            "PMET Ep",
            "Etp",
            "Trace Ep",
            "ΣUi",
            "Ep-ΣUi",
            "Ui-ΣUi",
            "Ws",
            "Ws-Ep/Etp",
            "Min Storage/Threshold",
            "Stress Layers",
        ],
        identity_rows,
    )
    md += "\n## Same-Day WAT Context\n\n"
    md += markdown_table(["Hill", "Symbol", "Candidate", "Baseline", "Diff"], context_rows)
    md += "\n## Per-Hill Layer Previews\n\n"
    for item in classifications:
        md += f"- H{item['hillslope_id']} material context: `{', '.join(item.get('material_context_symbols', [])) or 'none'}`; "
        md += f"storage/threshold layers: `{item.get('storage_to_threshold_preview', '')}`; "
        md += f"Ui layers: `{item.get('ui_layers_preview', '')}`.\n"
    (reports / "hphys0265_first_ep_divergence_classification.md").write_text(md, encoding="utf-8")
    return classifications


def run_semantics(run_root: Path) -> None:
    reports = run_root / "reports"
    semantic_dir = reports / "semantic_reports"
    semantic_dir.mkdir(parents=True, exist_ok=True)
    status_rows = []
    summary: dict[str, dict[str, Any]] = {}
    for hillslope_id in range(1, 40):
        report_json = semantic_dir / f"H{hillslope_id}.semantic.json"
        cmd = [
            str(WEPPPY_PYTHON),
            str(COMPARATOR),
            "--baseline-wat",
            str(BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet"),
            "--candidate-wat",
            str(run_root / f"hillslope_output/H{hillslope_id}.wat.parquet"),
            "--report-json",
            str(report_json),
            "--candidate-year-offset",
            "2012",
            "--tolerance-config",
            str(TOLERANCES),
        ]
        result = run_command(f"semantic_H{hillslope_id}", cmd, run_root / "logs/semantic")
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
                entry["max_abs_diff"] = max(entry["max_abs_diff"], float(stat["max_abs_diff"]))
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
    md = "# HPHYS0265 Full 39 Semantic Summary\n\n"
    md += "Ran:\n\n"
    md += f"- Root: `{run_root}`\n"
    md += f"- Runtime status: `{reports / 'hillslope_batch_status.tsv'}`\n"
    md += f"- Semantic status: `{reports / 'semantic_status.tsv'}`\n"
    md += f"- Semantic pass: `{sum(1 for row in status_rows if row['semantic_pass'])}/39`\n\n"
    md += markdown_table(
        ["Symbol", "Pass Hillslopes", "Total Fail Count", "Mean Abs Diff Mean", "Max Abs Diff"],
        selected_rows,
    )
    (reports / "hillslope_semantic_summary.md").write_text(md, encoding="utf-8")


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
        trace_path = output / f"H{hillslope_id}.hphys0265.trace.jsonl"
        result = run_command(
            f"H{hillslope_id}_trace",
            [
                str(HILL_BIN),
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
            return result.rc
    write_status(reports / "targeted_trace_status.tsv", trace_rows)
    summarize_first_ep_divergence(run_root)
    return 0


def run_full_hillslope_suite(run_root: Path, runs_dir: Path, output: Path, logs: Path) -> int:
    reports = run_root / "reports"
    batch_rows = []
    for hillslope_id in range(1, 40):
        result = run_command(
            f"H{hillslope_id}",
            [
                str(HILL_BIN),
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

    for required in [WEPPPY_PYTHON, COMPARATOR, TOLERANCES, BASELINE_PARTITIONS]:
        require_path(required)

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
        return build.rc

    targeted_rc = run_targeted_traces(run_root, runs_dir, output, logs, args.trace_max_days)
    if targeted_rc != 0:
        return targeted_rc
    if args.skip_full_suite:
        return 0
    return run_full_hillslope_suite(run_root, runs_dir, output, logs)


if __name__ == "__main__":
    raise SystemExit(main())
