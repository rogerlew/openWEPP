#!/usr/bin/env python3
"""Run HPHYS0263 EVAPPM demand-migration diagnostics."""

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
PMET_INTERMEDIATES = [
    "pmet_etorc_mm",
    "pmet_rn_mj_m2",
    "pmet_fwv_m_s",
    "pmet_rhd_pct",
    "pmet_kcbadj",
    "pmet_kcbcon",
    "pmet_etke",
    "pmet_etkr",
    "pmet_etks",
    "pmet_tew_mm",
    "pmet_rew_mm",
    "pmet_wfevp_mm",
    "pmet_taw_mm",
    "pmet_raw_mm",
    "pmet_wftrp_mm",
    "pmet_es_m",
    "pmet_ep_m",
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


def classify_evappm_migration(
    seed_row: dict[str, Any],
    root_row: dict[str, Any],
    candidate_ep_mm: float,
    baseline_ep_mm: float,
) -> tuple[str, dict[str, Any]]:
    seed_branch = seed_row.get("wb11_et_seed_branch")
    demand_m = number(seed_row, "wb11_et_demand_m")
    ep_m = number(root_row, "ep_m")
    etp_m = number(root_row, "etp_m")
    intermediate_values = {name: number(seed_row, name) for name in PMET_INTERMEDIATES}
    missing_intermediates = [name for name, value in intermediate_values.items() if value is None]
    diagnostics = {
        "candidate_ep_mm": candidate_ep_mm,
        "baseline_ep_mm": baseline_ep_mm,
        "ep_diff_mm": candidate_ep_mm - baseline_ep_mm,
        "pmet_sidecar_present": number(seed_row, "pmet_sidecar_present"),
        "pmet_iflget": number(seed_row, "pmet_iflget"),
        "pmet_selected_kcb": number(seed_row, "pmet_selected_kcb"),
        "pmet_selected_rawp": number(seed_row, "pmet_selected_rawp"),
        "pmet_selected_line_index": number(seed_row, "pmet_selected_line_index"),
        "pmet_lookup_fallback_first_row_used": number(
            seed_row, "pmet_lookup_fallback_first_row_used"
        ),
        "wb11_et_demand_mm": None if demand_m is None else demand_m * 1000.0,
        "wb11_et_seed_branch": seed_branch,
        "etp_mm": None if etp_m is None else etp_m * 1000.0,
        "trace_ep_mm": None if ep_m is None else ep_m * 1000.0,
        "ui_sum_mm": layer_sum(root_row, "wb17_ui_layers_m") * 1000.0,
        "pl_lai": number(root_row, "pl_lai"),
        "pl_rtd": number(root_row, "pl_rtd"),
        "missing_pmet_intermediates": missing_intermediates,
        **intermediate_values,
    }
    if diagnostics["pmet_iflget"] != 1.0 and seed_branch != "evappm_pmet":
        return "EVAPPM_SELECTED_BUT_BRANCH_NOT_MIGRATED", diagnostics
    if missing_intermediates:
        return "EVAPPM_BRANCH_MISSING_INTERMEDIATES", diagnostics
    if demand_m is not None and intermediate_values["pmet_ep_m"] is not None:
        if abs(demand_m - intermediate_values["pmet_ep_m"]) > 1.0e-12:
            return "EVAPPM_DEMAND_DIFFERS_FROM_PMTEP", diagnostics
    if diagnostics["pmet_iflget"] != 1.0 and seed_branch == "evappm_pmet":
        return "EVAPPM_MIGRATED_BRANCH_OBSERVED", diagnostics
    return "PT_OR_UNCLASSIFIED_BRANCH", diagnostics


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
        classification, diagnostics = classify_evappm_migration(
            seed_row,
            root_row,
            wat_value(candidate_wat, "Ep"),
            wat_value(baseline_wat, "Ep"),
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
                diagnostics["wb11_et_seed_branch"],
                diagnostics["wb11_et_demand_mm"],
                diagnostics["pmet_etorc_mm"],
                None
                if diagnostics["pmet_ep_m"] is None
                else diagnostics["pmet_ep_m"] * 1000.0,
                diagnostics["trace_ep_mm"],
                diagnostics["ui_sum_mm"],
                diagnostics["pl_lai"],
                diagnostics["pl_rtd"],
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

    md = "# HPHYS0263 EVAPPM Demand-Migration Classification\n\n"
    md += "Status: completed\n\nEvidence mode: ran\n\n"
    md += "Ran: HPHYS0254 diagnostic harness plus HPHYS0263 EVAPPM migration classification.\n\n"
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
            "seed branch",
            "demand mm",
            "etorc mm",
            "pmet_ep mm",
            "Trace Ep mm",
            "ΣUi mm",
            "LAI",
            "Root depth",
        ],
        rows,
    )
    md += "\nInterpretation:\n\n"
    md += (
        "- `EVAPPM_MIGRATED_BRANCH_OBSERVED` means WB11 selected the PMET branch, "
        "published migrated `evappm.for` intermediates, and seeded demand from `pmet_ep_m`.\n"
    )
    md += (
        "- Remaining `Ep` residuals after this classification should be assigned to "
        "post-demand SWU/growth timing, WB18/WB19 storage availability, or WB13 publication "
        "only with additional trace evidence.\n"
    )
    reports.joinpath("hphys0263_evappm_demand_migration_classification.md").write_text(
        md, encoding="utf-8"
    )
    reports.joinpath("hphys0263_evappm_demand_migration_classification.json").write_text(
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
    md += "Ran: HPHYS0263 diagnostic wrapper.\n\n"
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
