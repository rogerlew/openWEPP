#!/usr/bin/env python3
"""Run HPHYS0259 WB19 trace localization diagnostics."""

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
TOL = 1.0e-9


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
    rows: list[dict[str, Any]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def find_day1_lateral_row(rows: list[dict[str, Any]]) -> dict[str, Any]:
    for row in rows:
        if (
            row.get("sim_day_index") == 1
            and row.get("boundary") == "post_phase"
            and row.get("phase") == "lateral_transfer"
        ):
            return row
    raise ValueError("missing day-1 post_phase:lateral_transfer trace row")


def number(row: dict[str, Any], name: str) -> float | None:
    value = row.get(name)
    if value is None:
        return None
    return float(value)


def layer_sum(row: dict[str, Any], name: str) -> float:
    value = row.get(name) or {}
    return sum(float(item) for item in value.values())


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


def classify(row: dict[str, Any]) -> tuple[str, dict[str, float | None]]:
    q = number(row, "q_m")
    qdd = number(row, "qdd_m")
    qd = number(row, "qd_m")
    target = number(row, "wb19_q_lateral_target_m")
    unrealized = number(row, "wb19_q_lateral_unrealized_m")
    withdrawal_sum = layer_sum(row, "wb19_lateral_withdrawal_layers_m")
    q_withdrawal_residual = None if q is None else q - withdrawal_sum
    qd_residual = None if q is None or qdd is None or qd is None else qd - (q + qdd)
    unrealized_residual = (
        None
        if q is None or target is None or unrealized is None
        else unrealized - max(target - q, 0.0)
    )
    diagnostics = {
        "q": q,
        "qdd": qdd,
        "qd": qd,
        "target": target,
        "unrealized": unrealized,
        "withdrawal_sum": withdrawal_sum,
        "q_withdrawal_residual": q_withdrawal_residual,
        "qd_residual": qd_residual,
        "unrealized_residual": unrealized_residual,
    }
    residuals = [q_withdrawal_residual, qd_residual, unrealized_residual]
    if any(item is None for item in residuals):
        return "WB19_TRACE_INCOMPLETE", diagnostics
    if all(abs(float(item)) <= TOL for item in residuals):
        return "WB19_IDENTITIES_CLOSED_DOWNSTREAM_FOCUS", diagnostics
    return "WB19_INTERNAL_DIVERGENCE", diagnostics


def build_report(run_root: Path) -> None:
    reports = run_root / "reports"
    rows = []
    json_rows = []
    for hillslope_id in TARGETED_IDS:
        trace_path = run_root / f"hillslope_output/H{hillslope_id}.hphys0254.trace.jsonl"
        trace_row = find_day1_lateral_row(load_trace_rows(trace_path))
        classification, diagnostics = classify(trace_row)
        candidate_wat = wat_day1(run_root / f"hillslope_output/H{hillslope_id}.wat.parquet", True)
        baseline_wat = wat_day1(BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet", False)
        rows.append(
            [
                f"H{hillslope_id}",
                trace_row.get("schema"),
                classification,
                number(trace_row, "wb19_q_lateral_potential_m"),
                number(trace_row, "wb19_q_lateral_target_m"),
                number(trace_row, "wb19_tdvv_m"),
                diagnostics["q"],
                diagnostics["withdrawal_sum"],
                diagnostics["q_withdrawal_residual"],
                diagnostics["qd_residual"],
                wat_value(candidate_wat, "latqcc") - wat_value(baseline_wat, "latqcc"),
                wat_value(candidate_wat, "Ep") - wat_value(baseline_wat, "Ep"),
                wat_value(candidate_wat, "Dp") - wat_value(baseline_wat, "Dp"),
                wat_value(candidate_wat, "Total-Soil", "Total-Soil Water")
                - wat_value(baseline_wat, "Total-Soil", "Total-Soil Water"),
            ]
        )
        json_rows.append(
            {
                "hillslope": f"H{hillslope_id}",
                "schema": trace_row.get("schema"),
                "classification": classification,
                "trace_path": str(trace_path),
                **diagnostics,
            }
        )

    md = "# HPHYS0259 WB19 Trace Classification\n\n"
    md += "Status: complete\n\nEvidence mode: ran\n\n"
    md += "Ran: HPHYS0254 diagnostic harness plus HPHYS0259 trace classification.\n\n"
    md += markdown_table(
        [
            "Hillslope",
            "Trace schema",
            "Classification",
            "Potential m",
            "Target m",
            "tdvv m",
            "q m",
            "withdrawal sum m",
            "q-withdrawal residual m",
            "Qd residual m",
            "latqcc diff mm",
            "Ep diff mm",
            "Dp diff mm",
            "Total-Soil diff mm",
        ],
        rows,
    )
    md += "\nInterpretation:\n\n"
    md += (
        "- `WB19_IDENTITIES_CLOSED_DOWNSTREAM_FOCUS` means the trace proves "
        "WB19 realized `q`, per-layer withdrawal, unrealized residual, and "
        "`Qd` identities close for day 1; continuation should not reopen "
        "WB19 cap/publication logic without new baseline evidence.\n"
    )
    md += (
        "- `WB19_TRACE_INCOMPLETE` or `WB19_INTERNAL_DIVERGENCE` keeps residual "
        "ownership in WB19 for the next package.\n"
    )
    reports.joinpath("hphys0259_wb19_trace_classification.md").write_text(
        md, encoding="utf-8"
    )
    reports.joinpath("hphys0259_wb19_trace_classification.json").write_text(
        json.dumps(json_rows, indent=2) + "\n", encoding="utf-8"
    )


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
    build_report(args.run_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
