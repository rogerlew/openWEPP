#!/usr/bin/env python3
"""Run HPHYS0261 WB17 Ep magnitude/initialization diagnostics."""

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any

import pandas as pd


REPO = Path("/home/workdir/openWEPP")
WEPPPY_PYTHON = Path("/workdir/wepppy/.venv/bin/python")
HPHYS0254_DIAGNOSTICS = (
    REPO
    / "docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/hphys0254_diagnostics.py"
)
BASELINE_PARTITIONS = Path(
    "/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions"
)
TARGETED_IDS = [1, 7, 39]
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
TOL_M = 1.0e-9


def markdown_table(headers: list[str], rows: list[list[Any]]) -> str:
    def fmt(item: Any) -> str:
        if item is None:
            return ""
        if isinstance(item, float):
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
    return [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines() if line.strip()]


def find_day1_row(rows: list[dict[str, Any]], boundary: str, phase: str | None) -> dict[str, Any]:
    for row in rows:
        if (
            row.get("sim_day_index") == 1
            and row.get("boundary") == boundary
            and row.get("phase") == phase
        ):
            return row
    phase_label = phase if phase is not None else "none"
    raise ValueError(f"missing day-1 {boundary}:{phase_label} trace row")


def number(row: dict[str, Any], name: str) -> float | None:
    value = row.get(name)
    if value is None:
        return None
    return float(value)


def layer_values(row: dict[str, Any], name: str) -> dict[str, float]:
    values = row.get(name) or {}
    return {str(key): float(value) for key, value in values.items()}


def wat_day1(path: Path, candidate: bool) -> pd.Series:
    df = pd.read_parquet(path).copy()
    if candidate:
        df["comparison_year"] = df["year"].astype(int) + 2012
        return df.sort_values(["comparison_year", "julian"]).iloc[0]
    return df.sort_values(["year", "julian"]).iloc[0]


def wat_value(row: pd.Series, *names: str) -> float:
    for name in names:
        if name in row and pd.notna(row[name]):
            return float(row[name])
    return 0.0


def close_m(value: float | None) -> bool:
    return value is not None and abs(value) <= TOL_M


def classify_ep_lineage(root_row: dict[str, Any], candidate_ep_mm: float, baseline_ep_mm: float) -> tuple[str, dict[str, Any]]:
    etp = number(root_row, "etp_m")
    ep = number(root_row, "ep_m")
    ui = number(root_row, "ui_m")
    ws = number(root_row, "ws")
    ui_layers = layer_values(root_row, "wb17_ui_layers_m")
    thresholds = layer_values(root_row, "wb17_swu_stress_threshold_layers_m")
    storage_ratios = layer_values(root_row, "wb17_swu_storage_to_threshold_layers")
    ui_sum = sum(ui_layers.values())
    min_storage_ratio = min(storage_ratios.values()) if storage_ratios else None
    stress_limited_layers = sum(1 for ratio in storage_ratios.values() if ratio < 1.0)
    ep_minus_ui_sum = None if ep is None else ep - ui_sum
    ep_minus_etp = None if ep is None or etp is None else ep - etp
    ui_minus_sum = None if ui is None else ui - ui_sum
    ws_minus_ep_etp = None
    if ep is not None and etp is not None and ws is not None and etp > TOL_M:
        ws_minus_ep_etp = ws - (ep / etp)
    diagnostics = {
        "candidate_ep_mm": candidate_ep_mm,
        "baseline_ep_mm": baseline_ep_mm,
        "ep_diff_mm": candidate_ep_mm - baseline_ep_mm,
        "etp_mm": None if etp is None else etp * 1000.0,
        "trace_ep_mm": None if ep is None else ep * 1000.0,
        "ui_sum_mm": ui_sum * 1000.0,
        "pl_lai": number(root_row, "pl_lai"),
        "pl_rtd": number(root_row, "pl_rtd"),
        "pl_pltol": number(root_row, "pl_pltol"),
        "pl_swu_effective_pltol": number(root_row, "pl_swu_effective_pltol"),
        "threshold_layer_count": len(thresholds),
        "storage_ratio_layer_count": len(storage_ratios),
        "min_storage_to_threshold_ratio": min_storage_ratio,
        "stress_limited_layers": stress_limited_layers,
        "ep_minus_ui_sum_m": ep_minus_ui_sum,
        "ui_minus_sum_m": ui_minus_sum,
        "ep_minus_etp_m": ep_minus_etp,
        "ws_minus_ep_etp": ws_minus_ep_etp,
    }
    required = [
        etp,
        ep,
        ui,
        number(root_row, "pl_pltol"),
        number(root_row, "pl_swu_effective_pltol"),
    ]
    if any(item is None for item in required) or not thresholds or not storage_ratios:
        return "EP_INITIALIZATION_TRACE_INCOMPLETE", diagnostics
    identities_close = (
        close_m(ep_minus_ui_sum)
        and close_m(ui_minus_sum)
        and close_m(ep_minus_etp)
        and (ws_minus_ep_etp is None or close_m(ws_minus_ep_etp))
    )
    no_swu_stress = min_storage_ratio is not None and min_storage_ratio >= 1.0 and stress_limited_layers == 0
    if identities_close and no_swu_stress:
        return "ETP_FULL_DEMAND_NO_SWU_STRESS_MAGNITUDE_FOCUS", diagnostics
    if identities_close:
        return "SWU_STRESS_BRANCH_MAGNITUDE_FOCUS", diagnostics
    return "EP_INTERNAL_TRACE_DIVERGENCE", diagnostics


def write_targeted_report(run_root: Path) -> None:
    reports = run_root / "reports"
    rows: list[list[Any]] = []
    json_rows: list[dict[str, Any]] = []
    for hillslope_id in TARGETED_IDS:
        trace_path = run_root / f"hillslope_output/H{hillslope_id}.hphys0254.trace.jsonl"
        trace_rows = load_trace_rows(trace_path)
        root_row = find_day1_row(trace_rows, "post_phase", "plant_root_uptake")
        candidate_wat = wat_day1(run_root / f"hillslope_output/H{hillslope_id}.wat.parquet", True)
        baseline_wat = wat_day1(BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet", False)
        candidate_ep = wat_value(candidate_wat, "Ep")
        baseline_ep = wat_value(baseline_wat, "Ep")
        classification, diagnostics = classify_ep_lineage(root_row, candidate_ep, baseline_ep)
        rows.append(
            [
                f"H{hillslope_id}",
                root_row.get("schema"),
                classification,
                diagnostics["baseline_ep_mm"],
                diagnostics["candidate_ep_mm"],
                diagnostics["ep_diff_mm"],
                diagnostics["etp_mm"],
                diagnostics["trace_ep_mm"],
                diagnostics["ui_sum_mm"],
                diagnostics["pl_lai"],
                diagnostics["pl_rtd"],
                diagnostics["pl_swu_effective_pltol"],
                diagnostics["threshold_layer_count"],
                diagnostics["min_storage_to_threshold_ratio"],
                diagnostics["stress_limited_layers"],
            ]
        )
        json_rows.append(
            {
                "hillslope": f"H{hillslope_id}",
                "trace_schema": root_row.get("schema"),
                "trace_path": str(trace_path),
                "classification": classification,
                "diagnostics": diagnostics,
            }
        )

    md = "# HPHYS0261 WB17 Ep Initialization Classification\n\n"
    md += "Status: complete\n\nEvidence mode: ran\n\n"
    md += "Ran: HPHYS0254 diagnostic harness plus HPHYS0261 Ep classification.\n\n"
    md += markdown_table(
        [
            "Hillslope",
            "Trace schema",
            "Classification",
            "Baseline Ep mm",
            "Candidate Ep mm",
            "Ep diff mm",
            "Etp mm",
            "Trace Ep mm",
            "ΣUi mm",
            "LAI",
            "rtd m",
            "effective pltol",
            "Threshold layers",
            "min theta/threshold",
            "stress-limited layers",
        ],
        rows,
    )
    md += "\nStatic legacy authority:\n\n"
    md += "- `evap.for:583-586` seeds `ep` from current `lai` and `eo`.\n"
    md += "- `watbal_hourly.for:557-559` calls `evap` before daily `ptgrp`/`ptgra`.\n"
    md += "- `watbal_hourly.for:943-981` calls `ptgrp`/`ptgra`, then `swu`.\n"
    md += "- `swu.for:122-191` applies effective `pltol`, `ul(i)`, and `st(i)`.\n\n"
    md += "Interpretation:\n\n"
    md += (
        "- `ETP_FULL_DEMAND_NO_SWU_STRESS_MAGNITUDE_FOCUS` means candidate "
        "final `Ep` equals `Etp` and `ΣUi`, and all traced storage/threshold "
        "ratios are above one; continuation should focus on baseline "
        "`Etp`/initialization magnitude rather than SWU stress clipping.\n"
    )
    reports.joinpath("hphys0261_ep_initialization_classification.md").write_text(
        md, encoding="utf-8"
    )
    reports.joinpath("hphys0261_ep_initialization_classification.json").write_text(
        json.dumps(json_rows, indent=2) + "\n", encoding="utf-8"
    )


def copy_selected_metrics(run_root: Path, package_dir: Path) -> None:
    summary_json = json.loads(
        (run_root / "reports/hillslope_semantic_summary.json").read_text(encoding="utf-8")
    )
    by_column = {row["column"]: row for row in summary_json}
    rows = []
    for symbol in SELECTED_SYMBOLS:
        row = by_column[symbol]
        rows.append(
            [
                symbol,
                f"{39 - int(row['hillslope_fail_count'])}/39",
                int(row["total_fail_count"]),
                float(row["mean_abs_diff_mean"]),
                float(row["max_abs_diff"]),
            ]
        )
    md = "# Full 39 Suite Metrics\n\n"
    md += "Status: completed\n\nEvidence mode: ran\n\n"
    md += "Ran: HPHYS0261 diagnostic wrapper.\n\n"
    md += f"- Run root: `{run_root}`.\n"
    md += f"- Summary: `{run_root / 'reports/hillslope_semantic_summary.md'}`.\n"
    md += f"- Semantic pass: `0/39`.\n\n"
    md += markdown_table(
        ["Symbol", "Pass Hillslopes", "Total Fail Count", "Mean Abs Diff Mean", "Max Abs Diff"],
        rows,
    )
    (package_dir / "full-39-suite-metrics.md").write_text(md, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", required=True, type=Path)
    args = parser.parse_args()

    result = subprocess.run(
        [
            str(WEPPPY_PYTHON),
            str(HPHYS0254_DIAGNOSTICS),
            "--run-root",
            str(args.run_root),
        ],
        cwd=REPO,
        check=False,
    )
    if result.returncode != 0:
        return result.returncode
    write_targeted_report(args.run_root)
    copy_selected_metrics(args.run_root, Path(__file__).resolve().parent)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
