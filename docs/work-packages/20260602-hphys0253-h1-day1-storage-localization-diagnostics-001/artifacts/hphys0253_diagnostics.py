#!/usr/bin/env python3
"""Run HPHYS0253 diagnostic-only H1 localization and full 39 metrics."""

from __future__ import annotations

import argparse
import csv
import json
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


def load_trace_rows(path: Path) -> list[dict[str, Any]]:
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def value(row: pd.Series, *names: str, default: float = 0.0) -> float:
    for name in names:
        if name in row and pd.notna(row[name]):
            return float(row[name])
    return default


def trace_value(row: dict[str, Any], name: str, scale: float = 1.0) -> float | None:
    observed = row.get(name)
    if observed is None:
        return None
    return float(observed) * scale


def first_day_candidate_row(candidate_wat: Path) -> pd.Series:
    df = pd.read_parquet(candidate_wat).copy()
    df["comparison_year"] = df["year"].astype(int) + 2012
    df = df.sort_values(["comparison_year", "julian"])
    return df.iloc[0]


def first_day_baseline_row(baseline_wat: Path) -> pd.Series:
    df = pd.read_parquet(baseline_wat).copy()
    df = df.sort_values(["year", "julian"])
    return df.iloc[0]


def water_terms(row: pd.Series, total_soil_name: str) -> dict[str, float]:
    et = value(row, "Ep") + value(row, "Es") + value(row, "Er")
    losses = et + value(row, "Dp") + value(row, "latqcc") + value(row, "Q")
    storage_end = value(row, total_soil_name, "Total-Soil", "Total-Soil Water")
    p_input = value(row, "P")
    rm_input = value(row, "RM")
    inferred_initial_p = storage_end + losses - p_input
    inferred_initial_rm = storage_end + losses - rm_input
    return {
        "P": p_input,
        "RM": rm_input,
        "ET": et,
        "Ep": value(row, "Ep"),
        "Es": value(row, "Es"),
        "Er": value(row, "Er"),
        "Dp": value(row, "Dp"),
        "latqcc": value(row, "latqcc"),
        "Q": value(row, "Q"),
        "Snow-Water": value(row, "Snow-Water"),
        "Total-Soil": storage_end,
        "SoilWaterTotal": value(row, "SoilWaterTotal", default=storage_end),
        "losses": losses,
        "inferred_initial_from_P": inferred_initial_p,
        "inferred_initial_from_RM": inferred_initial_rm,
    }


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


def summarize_h1(run_root: Path) -> None:
    reports = run_root / "reports"
    trace_path = run_root / "hillslope_output/H1.hphys0253.trace.jsonl"
    candidate_wat = run_root / "hillslope_output/H1.wat.parquet"
    baseline_wat = BASELINE_PARTITIONS / "baseline_H1.parquet"
    trace_rows = load_trace_rows(trace_path)
    day1_rows = [row for row in trace_rows if row["sim_day_index"] == 1]
    selected = {}
    for row in day1_rows:
        key = row["boundary"]
        if row.get("phase"):
            key += f":{row['phase']}"
        selected[key] = row

    candidate_day1 = first_day_candidate_row(candidate_wat)
    baseline_day1 = first_day_baseline_row(baseline_wat)
    candidate_terms = water_terms(candidate_day1, "Total-Soil")
    baseline_terms = water_terms(baseline_day1, "Total-Soil Water")
    post_seed = selected["post_seed"]
    post_wb13 = selected["post_wb13"]
    post_seed_wb11 = trace_value(post_seed, "wb11_soil_water_mm")
    post_seed_theta = trace_value(post_seed, "wb18_theta_sum_m", 1000.0)
    post_wb13_wb11 = trace_value(post_wb13, "wb11_soil_water_mm")
    post_wb13_total = trace_value(post_wb13, "wb13_total_soil_mm")
    actual_delta_storage = (
        post_wb13_wb11 - post_seed_wb11
        if post_wb13_wb11 is not None and post_seed_wb11 is not None
        else None
    )
    candidate_residual_p = None
    candidate_residual_rm = None
    if actual_delta_storage is not None:
        candidate_residual_p = candidate_terms["P"] - (
            candidate_terms["losses"] + actual_delta_storage
        )
        candidate_residual_rm = candidate_terms["RM"] - (
            candidate_terms["losses"] + actual_delta_storage
        )
    baseline_storage_delta_inferred_p = (
        baseline_terms["Total-Soil"] - baseline_terms["inferred_initial_from_P"]
    )
    baseline_storage_delta_inferred_rm = (
        baseline_terms["Total-Soil"] - baseline_terms["inferred_initial_from_RM"]
    )

    storage_rows = [
        [
            "candidate post_seed wb11",
            post_seed_wb11,
            "actual trace state before day-1 scheduler",
        ],
        [
            "candidate post_seed theta sum",
            post_seed_theta,
            "actual trace layer theta state before day-1 scheduler",
        ],
        [
            "candidate post_wb13 wb11",
            post_wb13_wb11,
            "actual trace state after day-1 publication",
        ],
        [
            "candidate post_wb13 Total-Soil",
            post_wb13_total,
            "published day-1 storage from WB13 trace",
        ],
        [
            "baseline day-1 Total-Soil",
            baseline_terms["Total-Soil"],
            "baseline WAT end-of-day row",
        ],
        [
            "baseline inferred initial from P",
            baseline_terms["inferred_initial_from_P"],
            "S1 + ET + Dp + latqcc + Q - P",
        ],
        [
            "candidate post_seed minus baseline inferred initial",
            post_seed_wb11 - baseline_terms["inferred_initial_from_P"],
            "negative means candidate starts drier",
        ],
        [
            "candidate post_wb13 minus baseline day-1 Total-Soil",
            post_wb13_total - baseline_terms["Total-Soil"],
            "negative means candidate ends drier",
        ],
    ]

    phase_order = [
        "post_seed",
        "post_phase:normalization",
        "post_phase:storage_bounds",
        "post_phase:decomposition_transition",
        "post_phase:residue_partition_transition",
        "post_phase:annual_growth_transition",
        "post_phase:perennial_growth_transition",
        "post_phase:percolation_deep_seepage",
        "post_phase:evapotranspiration",
        "post_phase:drainage",
        "post_phase:lateral_transfer",
        "post_phase:runoff_reconciliation",
        "post_phase:storage_reconciliation",
        "post_phase:closure_diagnostics",
        "post_scheduler",
        "post_wb13",
    ]
    phase_rows = []
    previous_key = None
    previous_value = None
    for key in phase_order:
        if key not in selected:
            continue
        row = selected[key]
        wb11 = trace_value(row, "wb11_soil_water_mm")
        theta = trace_value(row, "wb18_theta_sum_m", 1000.0)
        delta = wb11 - previous_value if wb11 is not None and previous_value is not None else None
        phase_rows.append(
            [
                key,
                wb11,
                theta,
                delta,
                trace_value(row, "d_m", 1000.0),
                trace_value(row, "pe_m", 1000.0),
                trace_value(row, "ep_m", 1000.0),
                trace_value(row, "upi_m", 1000.0),
                trace_value(row, "ui_m", 1000.0),
                row.get("ws"),
            ]
        )
        previous_key = key
        previous_value = wb11
    _ = previous_key

    comparison_rows = []
    for term in ["P", "RM", "ET", "Ep", "Es", "Er", "Dp", "latqcc", "Q", "Snow-Water", "Total-Soil"]:
        comparison_rows.append(
            [
                term,
                baseline_terms[term],
                candidate_terms[term],
                candidate_terms[term] - baseline_terms[term],
            ]
        )

    conservation_rows = [
        [
            "candidate actual P residual",
            candidate_residual_p,
            "P - (ET + Dp + latqcc + Q + actual ΔS)",
        ],
        [
            "candidate actual RM residual",
            candidate_residual_rm,
            "RM - (ET + Dp + latqcc + Q + actual ΔS)",
        ],
        [
            "candidate inferred initial from P",
            candidate_terms["inferred_initial_from_P"],
            "S1 + ET + Dp + latqcc + Q - P",
        ],
        [
            "candidate actual post_seed",
            post_seed_wb11,
            "trace post_seed wb11_soil_water",
        ],
        [
            "candidate inferred minus actual",
            candidate_terms["inferred_initial_from_P"] - post_seed_wb11,
            "near zero means WAT/trace day-1 accounting closes",
        ],
        [
            "baseline inferred initial from P",
            baseline_terms["inferred_initial_from_P"],
            "baseline WAT-derived t0 proxy",
        ],
        [
            "baseline inferred ΔS from P",
            baseline_storage_delta_inferred_p,
            "S1 - inferred S0",
        ],
        [
            "baseline inferred ΔS from RM",
            baseline_storage_delta_inferred_rm,
            "S1 - inferred S0",
        ],
    ]

    localization = "# H1 Day-1 Storage Localization Report\n\n"
    localization += "Status: complete\n\nEvidence mode: ran\n\nRan:\n\n"
    localization += f"- Trace: `{trace_path}`.\n"
    localization += f"- Candidate WAT: `{candidate_wat}`.\n"
    localization += f"- Baseline WAT: `{baseline_wat}`.\n\n"
    localization += "## Storage Surfaces\n\n"
    localization += markdown_table(["Surface", "mm", "Interpretation"], storage_rows)
    localization += "\n## Day-1 Phase State\n\n"
    localization += markdown_table(
        [
            "Stage",
            "wb11 mm",
            "theta sum mm",
            "Δ wb11 mm",
            "D mm",
            "Pe mm",
            "Ep mm",
            "UPi mm",
            "Ui mm",
            "Ws",
        ],
        phase_rows,
    )
    localization += "\n## Baseline vs Candidate Day-1 Terms\n\n"
    localization += markdown_table(["Term", "Baseline", "Candidate", "Candidate - Baseline"], comparison_rows)
    reports.joinpath("h1_day1_storage_localization.md").write_text(localization, encoding="utf-8")

    conservation = "# H1 Day-1 Conservation Audit\n\n"
    conservation += "Status: complete\n\nEvidence mode: ran\n\nRan:\n\n"
    conservation += f"- Trace rows: `{trace_path}`.\n"
    conservation += "- Formula: input minus `ET + Dp + latqcc + Q + delta-storage`.\n"
    conservation += "- `ET` is `Ep + Es + Er`; `delta-storage` uses actual trace "
    conservation += "`post_wb13 wb11 - post_seed wb11` for candidate rows.\n\n"
    conservation += markdown_table(["Quantity", "mm", "Notes"], conservation_rows)
    conservation += "\n## Interpretation\n\n"
    conservation += "- Candidate WAT/trace day-1 accounting is internally closed when "
    conservation += "`candidate inferred minus actual` is near zero.\n"
    conservation += "- The primary localization signal is the split between the "
    conservation += "candidate post-seed deficit and additional day-1 loss terms.\n"
    reports.joinpath("h1_day1_conservation_audit.md").write_text(conservation, encoding="utf-8")


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

    selected_rows = []
    by_column = {row["column"]: row for row in summary_rows}
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
    md = "# HPHYS0253 Full 39 Semantic Summary\n\n"
    md += f"- Root: `{run_root}`\n"
    md += f"- Runtime status: `{reports / 'hillslope_batch_status.tsv'}`\n"
    md += f"- Semantic status: `{reports / 'semantic_status.tsv'}`\n"
    md += f"- Semantic pass: `{sum(1 for row in status_rows if row['semantic_pass'])}/39`\n\n"
    md += markdown_table(
        ["Symbol", "Pass Hillslopes", "Total Fail Count", "Mean Abs Diff Mean", "Max Abs Diff"],
        selected_rows,
    )
    (reports / "hillslope_semantic_summary.md").write_text(md, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", required=True, type=Path)
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

    trace_path = output / "H1.hphys0253.trace.jsonl"
    h1_trace = run_command(
        "H1_trace",
        [
            str(HILL_BIN),
            "--run-dir",
            str(runs_dir),
            "--run-file",
            "p1_openwepp.run",
            "--output-dir",
            str(output),
            "--policy",
            "compat",
        ],
        logs,
        env={
            "OPENWEPP_HPHYS0245_TRACE_PATH": str(trace_path),
            "OPENWEPP_HPHYS0245_TRACE_MAX_DAYS": "1",
        },
    )
    write_status(
        reports / "h1_trace_status.tsv",
        [
            {
                "hillslope_id": 1,
                "rc": h1_trace.rc,
                "seconds": f"{h1_trace.seconds:.3f}",
                "trace_path": trace_path,
                "stdout": h1_trace.stdout,
                "stderr": h1_trace.stderr,
            }
        ],
    )
    if h1_trace.rc != 0:
        return h1_trace.rc
    summarize_h1(run_root)

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


if __name__ == "__main__":
    raise SystemExit(main())
