#!/usr/bin/env python3
"""Run HPHYS0262 WB17 PMET demand-seeding diagnostics."""

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


def layer_sum(row: dict[str, Any], name: str) -> float:
    values = row.get(name) or {}
    return sum(float(value) for value in values.values())


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


def classify_pmet_lineage(
    seed_row: dict[str, Any],
    root_row: dict[str, Any],
    candidate_ep_mm: float,
    baseline_ep_mm: float,
) -> tuple[str, dict[str, Any]]:
    pmet_sidecar_present = number(seed_row, "pmet_sidecar_present")
    pmet_iflget = number(seed_row, "pmet_iflget")
    pmet_kcb = number(seed_row, "pmet_selected_kcb")
    pmet_rawp = number(seed_row, "pmet_selected_rawp")
    seed_branch = seed_row.get("wb11_et_seed_branch")
    demand = number(seed_row, "wb11_et_demand_m")
    ep = number(root_row, "ep_m")
    etp = number(root_row, "etp_m")
    ui_sum = layer_sum(root_row, "wb17_ui_layers_m")
    diagnostics = {
        "candidate_ep_mm": candidate_ep_mm,
        "baseline_ep_mm": baseline_ep_mm,
        "ep_diff_mm": candidate_ep_mm - baseline_ep_mm,
        "pmet_sidecar_present": pmet_sidecar_present,
        "pmet_iflget": pmet_iflget,
        "pmet_selected_kcb": pmet_kcb,
        "pmet_selected_rawp": pmet_rawp,
        "pmet_selected_line_index": number(seed_row, "pmet_selected_line_index"),
        "pmet_lookup_fallback_first_row_used": number(
            seed_row, "pmet_lookup_fallback_first_row_used"
        ),
        "wb11_et_demand_mm": None if demand is None else demand * 1000.0,
        "wb11_et_seed_branch": seed_branch,
        "etp_mm": None if etp is None else etp * 1000.0,
        "trace_ep_mm": None if ep is None else ep * 1000.0,
        "ui_sum_mm": ui_sum * 1000.0,
        "pl_lai": number(root_row, "pl_lai"),
        "pl_rtd": number(root_row, "pl_rtd"),
    }
    if pmet_sidecar_present is None or pmet_iflget is None or seed_branch is None:
        return "PMET_TRACE_INCOMPLETE", diagnostics
    if pmet_iflget != 1.0 and (pmet_kcb is None or pmet_rawp is None):
        return "PMET_SELECTED_COEFFICIENT_TRACE_INCOMPLETE", diagnostics
    if pmet_sidecar_present >= 0.5 and pmet_iflget != 1.0 and seed_branch == "evap_priestley_taylor":
        return "PMET_SIDECAR_SELECTS_EVAPPM_BUT_PT_DEMAND_SEEDED", diagnostics
    if pmet_sidecar_present >= 0.5 and pmet_iflget != 1.0 and seed_branch == "evappm_pmet":
        return "PMET_EVAPPM_BRANCH_OBSERVED", diagnostics
    if pmet_iflget == 1.0 and seed_branch == "evap_priestley_taylor":
        return "PT_BRANCH_CONSISTENT_WITH_ABSENT_PMET", diagnostics
    return "PMET_BRANCH_STATE_UNCLASSIFIED", diagnostics


def write_targeted_report(run_root: Path) -> None:
    reports = run_root / "reports"
    rows: list[list[Any]] = []
    json_rows: list[dict[str, Any]] = []
    for hillslope_id in TARGETED_IDS:
        trace_path = run_root / f"hillslope_output/H{hillslope_id}.hphys0254.trace.jsonl"
        trace_rows = load_trace_rows(trace_path)
        seed_row = find_day1_row(trace_rows, "post_seed", None)
        root_row = find_day1_row(trace_rows, "post_phase", "plant_root_uptake")
        candidate_wat = wat_day1(run_root / f"hillslope_output/H{hillslope_id}.wat.parquet", True)
        baseline_wat = wat_day1(BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet", False)
        candidate_ep = wat_value(candidate_wat, "Ep")
        baseline_ep = wat_value(baseline_wat, "Ep")
        classification, diagnostics = classify_pmet_lineage(
            seed_row, root_row, candidate_ep, baseline_ep
        )
        rows.append(
            [
                f"H{hillslope_id}",
                seed_row.get("schema"),
                classification,
                diagnostics["baseline_ep_mm"],
                diagnostics["candidate_ep_mm"],
                diagnostics["ep_diff_mm"],
                diagnostics["pmet_iflget"],
                diagnostics["pmet_selected_kcb"],
                diagnostics["pmet_selected_rawp"],
                diagnostics["pmet_selected_line_index"],
                diagnostics["pmet_lookup_fallback_first_row_used"],
                diagnostics["wb11_et_seed_branch"],
                diagnostics["wb11_et_demand_mm"],
                diagnostics["trace_ep_mm"],
                diagnostics["ui_sum_mm"],
                diagnostics["pl_lai"],
            ]
        )
        json_rows.append(
            {
                "hillslope": f"H{hillslope_id}",
                "trace_schema": seed_row.get("schema"),
                "trace_path": str(trace_path),
                "classification": classification,
                "diagnostics": diagnostics,
            }
        )

    md = "# HPHYS0262 WB17 PMET Demand-Seeding Classification\n\n"
    md += "Status: completed\n\nEvidence mode: ran\n\n"
    md += "Ran: HPHYS0254 diagnostic harness plus HPHYS0262 PMET branch classification.\n\n"
    md += markdown_table(
        [
            "Hillslope",
            "Trace schema",
            "Classification",
            "Baseline Ep mm",
            "Candidate Ep mm",
            "Ep diff mm",
            "iflget",
            "kcb",
            "rawp",
            "PMET line",
            "fallback",
            "seed branch",
            "demand mm",
            "Trace Ep mm",
            "ΣUi mm",
            "LAI",
        ],
        rows,
    )
    md += "\nStatic legacy authority:\n\n"
    md += "- `watbal_hourly.for:557-559` calls `evap` when `iflget.eq.1`; otherwise `evappm`.\n"
    md += "- `evappm.for:181-297` computes Penman-Monteith reference ET, crop coefficients, `Es`, and `Ep` from PMET inputs.\n"
    md += "- `SC-INFILE-PMETPARA-001` defines sidecar-present `iflget=2`, crop-key lookup, `kcb`, `rawp`, and fallback observability.\n\n"
    md += "Interpretation:\n\n"
    md += (
        "- `PMET_SIDECAR_SELECTS_EVAPPM_BUT_PT_DEMAND_SEEDED` means the run "
        "discovers and projects PMET sidecar/crop coefficients, but current "
        "WB11 ET demand is still seeded by the Priestley-Taylor `evap` branch; "
        "closure requires baseline-authoritative `evappm.for` migration, not "
        "coefficient tuning or proxy demand.\n"
    )
    reports.joinpath("hphys0262_pmet_demand_seed_classification.md").write_text(
        md, encoding="utf-8"
    )
    reports.joinpath("hphys0262_pmet_demand_seed_classification.json").write_text(
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
    md = "# Full H1..H39 Suite Metrics\n\n"
    md += "Status: completed\n\nEvidence mode: ran\n\n"
    md += "Ran: HPHYS0262 diagnostic wrapper.\n\n"
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
