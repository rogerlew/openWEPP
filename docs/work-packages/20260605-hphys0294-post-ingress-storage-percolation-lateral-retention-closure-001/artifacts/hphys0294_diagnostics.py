#!/usr/bin/env python3
"""Run HPHYS0294 post-ingress storage/percolation/lateral diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import sys
from pathlib import Path
from typing import Any


REPO = Path("/home/workdir/openWEPP")
HPHYS0291_SCRIPT = (
    REPO
    / "docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/hphys0291_diagnostics.py"
)
TARGET_DAYS = {
    1: [(2014, 132), (2014, 133), (2014, 141), (2014, 145), (2014, 146), (2016, 111)],
    7: [(2014, 132), (2014, 133), (2014, 141), (2014, 145), (2014, 146), (2016, 111)],
    39: [(2014, 132), (2014, 133), (2014, 141), (2014, 145), (2014, 146), (2016, 111)],
}
SELECTED_COLUMNS = {
    "Ep",
    "Es",
    "Er",
    "Total-Soil",
    "SoilWaterTotal",
    "Dp",
    "latqcc",
    "Q",
    "RM",
    "Snow-Water",
    "P",
}


def load_hphys0291_module() -> Any:
    spec = importlib.util.spec_from_file_location("hphys0291_diagnostics", HPHYS0291_SCRIPT)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {HPHYS0291_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0291 = load_hphys0291_module()
HPHYS0265 = HPHYS0291.HPHYS0265


def markdown_table(headers: list[str], rows: list[list[Any]]) -> str:
    return HPHYS0265.markdown_table(headers, rows)


def value(row: dict[str, Any] | None, key: str) -> float | None:
    if row is None or row.get(key) is None:
        return None
    return float(row[key])


def mm(row: dict[str, Any] | None, key: str) -> float | None:
    row_value = value(row, key)
    if row_value is None:
        return None
    return row_value * 1_000.0


def rounded(row_value: Any, digits: int = 6) -> Any:
    if row_value is None:
        return None
    if isinstance(row_value, float):
        return round(row_value, digits)
    return row_value


def find_trace_row(trace_path: Path, year: int, julian: int) -> dict[str, Any] | None:
    if not trace_path.exists():
        return None
    with trace_path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            if (
                int(row.get("calendar_year", -1)) == year
                and int(row.get("julian_day", -1)) == julian
                and row.get("boundary") == "post_wb13"
            ):
                return row
    return None


def merged_wat_rows(run_root: Path, hillslope_id: int) -> Any:
    return HPHYS0265.candidate_baseline_merge(
        run_root / f"hillslope_output/H{hillslope_id}.wat.parquet",
        HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet",
        candidate_year_offset=2012,
    )


def find_merged_row(merged: Any, year: int, julian: int) -> Any | None:
    matches = merged[(merged["_comparison_year"] == year) & (merged["julian"] == julian)]
    if matches.empty:
        return None
    return matches.iloc[0]


def wat_candidate(row: Any | None, symbol: str) -> float | None:
    if row is None:
        return None
    return float(HPHYS0265.wat_candidate(row, symbol))


def wat_baseline(row: Any | None, symbol: str) -> float | None:
    if row is None:
        return None
    return float(HPHYS0265.wat_baseline(row, symbol))


def wat_delta(row: Any | None, symbol: str) -> float | None:
    candidate = wat_candidate(row, symbol)
    baseline = wat_baseline(row, symbol)
    if candidate is None or baseline is None:
        return None
    return candidate - baseline


def normalize_full_suite_summary_label(run_root: Path) -> None:
    summary_path = run_root / "reports/hillslope_semantic_summary.md"
    if not summary_path.exists():
        return
    summary = summary_path.read_text(encoding="utf-8")
    summary = summary.replace(
        "# HPHYS0291 Full H1..H39 Semantic Summary",
        "# HPHYS0294 Full H1..H39 Semantic Summary",
        1,
    )
    summary_path.write_text(summary, encoding="utf-8")


def write_selected_metrics(run_root: Path) -> None:
    reports = run_root / "reports"
    summary_json = reports / "hillslope_semantic_summary.json"
    summary = json.loads(summary_json.read_text(encoding="utf-8"))
    selected = {row["column"]: row for row in summary if row["column"] in SELECTED_COLUMNS}
    (reports / "hphys0294_selected_metrics.json").write_text(
        json.dumps(selected, indent=2) + "\n", encoding="utf-8"
    )


def run_targeted_traces(run_root: Path, trace_max_days: int) -> int:
    reports = run_root / "reports"
    logs = run_root / "logs" / "targeted_traces"
    output = run_root / "hillslope_output"
    runs_dir = HPHYS0265.copy_runfiles(run_root)
    status_rows: list[dict[str, Any]] = []
    for hillslope_id in TARGET_DAYS:
        trace_path = output / f"H{hillslope_id}.hphys0294.trace.jsonl"
        result = HPHYS0265.run_command(
            f"H{hillslope_id}_hphys0294_trace",
            [
                str(HPHYS0291.HILL_BIN),
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
        status_rows.append(
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
            HPHYS0265.write_status(reports / "hphys0294_target_trace_status.tsv", status_rows)
            return int(result.rc)
    HPHYS0265.write_status(reports / "hphys0294_target_trace_status.tsv", status_rows)
    return 0


def extract_target_storage_rows(run_root: Path) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    output = run_root / "hillslope_output"
    extracted: list[dict[str, Any]] = []
    for hillslope_id, days in TARGET_DAYS.items():
        merged = merged_wat_rows(run_root, hillslope_id)
        trace_path = output / f"H{hillslope_id}.hphys0294.trace.jsonl"
        for year, julian in days:
            trace = find_trace_row(trace_path, year, julian)
            merged_row = find_merged_row(merged, year, julian)
            extracted.append(
                {
                    "hillslope_id": hillslope_id,
                    "year": year,
                    "julian": julian,
                    "sim_day_index": None if trace is None else trace.get("sim_day_index"),
                    "candidate_total_soil_mm": wat_candidate(merged_row, "Total-Soil"),
                    "baseline_total_soil_mm": wat_baseline(merged_row, "Total-Soil"),
                    "delta_total_soil_mm": wat_delta(merged_row, "Total-Soil"),
                    "candidate_soil_water_total_mm": wat_candidate(merged_row, "SoilWaterTotal"),
                    "baseline_soil_water_total_mm": wat_baseline(merged_row, "SoilWaterTotal"),
                    "delta_soil_water_total_mm": wat_delta(merged_row, "SoilWaterTotal"),
                    "candidate_dp_mm": wat_candidate(merged_row, "Dp"),
                    "baseline_dp_mm": wat_baseline(merged_row, "Dp"),
                    "delta_dp_mm": wat_delta(merged_row, "Dp"),
                    "candidate_latqcc_mm": wat_candidate(merged_row, "latqcc"),
                    "baseline_latqcc_mm": wat_baseline(merged_row, "latqcc"),
                    "delta_latqcc_mm": wat_delta(merged_row, "latqcc"),
                    "delta_q_mm": wat_delta(merged_row, "Q"),
                    "delta_rm_mm": wat_delta(merged_row, "RM"),
                    "delta_snow_water_mm": wat_delta(merged_row, "Snow-Water"),
                    "trace_wb13_total_soil_mm": value(trace, "wb13_total_soil_mm"),
                    "trace_wb13_soil_water_total_mm": value(trace, "wb13_soil_water_total_mm"),
                    "trace_wb13_dp_mm": value(trace, "wb13_dp_mm"),
                    "trace_wb13_q_mm": value(trace, "wb13_q_mm"),
                    "trace_wb13_rm_mm": value(trace, "wb13_rm_mm"),
                    "trace_wb13_snow_water_mm": value(trace, "wb13_snow_water_mm"),
                    "trace_wb11_soil_water_mm": mm(trace, "wb11_soil_water_m"),
                    "trace_wb18_theta_sum_mm": mm(trace, "wb18_theta_sum_m"),
                    "trace_wb18_recomputed_soil_water_mm": mm(
                        trace, "wb18_recomputed_soil_water_m"
                    ),
                    "trace_wb18_recomputed_minus_wb11_mm": mm(
                        trace, "wb18_recomputed_minus_wb11_m"
                    ),
                    "trace_wb11_minus_theta_sum_mm": mm(trace, "wb11_minus_theta_sum_m"),
                    "trace_wb18_pei_sum_mm": mm(trace, "wb18_pei_sum_m"),
                    "trace_d_mm": mm(trace, "d_m"),
                    "trace_pe_mm": mm(trace, "pe_m"),
                    "trace_wb12_infiltration_mm": mm(trace, "wb12_infiltration_m"),
                    "trace_wb19_lateral_potential_mm": mm(trace, "wb19_q_lateral_potential_m"),
                    "trace_wb19_lateral_target_mm": mm(trace, "wb19_q_lateral_target_m"),
                    "trace_wb19_lateral_capacity_tdv_mm": mm(
                        trace, "wb19_lateral_capacity_tdv_m"
                    ),
                    "trace_wb19_lateral_unrealized_mm": mm(
                        trace, "wb19_q_lateral_unrealized_m"
                    ),
                    "trace_snow_runtime_swe_delta_mm": mm(trace, "snow_runtime_swe_delta_m"),
                    "trace_snow_routed_melt_mm": mm(trace, "snow_routed_melt_m"),
                }
            )
    (reports / "hphys0294_target_storage_rows.json").write_text(
        json.dumps(extracted, indent=2) + "\n", encoding="utf-8"
    )
    return extracted


def write_target_storage_markdown(run_root: Path, rows: list[dict[str, Any]]) -> None:
    reports = run_root / "reports"
    table_rows = [
        [
            f"H{row['hillslope_id']}",
            row["year"],
            row["julian"],
            rounded(row["delta_total_soil_mm"]),
            rounded(row["delta_soil_water_total_mm"]),
            rounded(row["delta_dp_mm"]),
            rounded(row["delta_latqcc_mm"]),
            rounded(row["delta_q_mm"]),
            rounded(row["delta_rm_mm"]),
            rounded(row["delta_snow_water_mm"]),
            rounded(row["trace_wb18_recomputed_minus_wb11_mm"], 9),
            rounded(row["trace_d_mm"]),
            rounded(row["trace_pe_mm"]),
            rounded(row["trace_wb18_pei_sum_mm"]),
            rounded(row["trace_wb19_lateral_potential_mm"]),
            rounded(row["trace_wb19_lateral_target_mm"]),
            rounded(row["trace_wb19_lateral_unrealized_mm"]),
            rounded(row["trace_wb12_infiltration_mm"]),
            rounded(row["trace_snow_runtime_swe_delta_mm"]),
            rounded(row["trace_snow_routed_melt_mm"]),
        ]
        for row in rows
    ]
    markdown = "# HPHYS0294 Target Storage/Percolation/Lateral Rows\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Run root: `{run_root}`\n"
    markdown += f"- Target rows JSON: `{reports / 'hphys0294_target_storage_rows.json'}`\n"
    markdown += f"- Trace status: `{reports / 'hphys0294_target_trace_status.tsv'}`\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Year",
            "Julian",
            "ΔTotal-Soil",
            "ΔSoilWaterTotal",
            "ΔDp",
            "Δlatqcc",
            "ΔQ",
            "ΔRM",
            "ΔSnow",
            "WB18 ID Residual",
            "D",
            "Pe",
            "Σpei",
            "WB19 Potential",
            "WB19 Target",
            "WB19 Unrealized",
            "WB12 Infil",
            "ΔSWE Trace",
            "Routed Melt",
        ],
        table_rows,
    )
    markdown += "\n## Interpretation\n\n"
    markdown += (
        "- WB18 aggregate identity residual is trace `wb18_recomputed_minus_wb11_m`; "
        "near-zero values mean publication is not collapsing `watcon` to `Σtheta`.\n"
    )
    markdown += (
        "- `D` and `Pe` are bottom-loss publication terms; `Σpei` is an internal "
        "per-layer routing sum and must not be collapsed to `D`.\n"
    )
    markdown += (
        "- WB19 potential/target/unrealized terms classify lateral-retention "
        "magnitude before assigning `latqcc`/storage ownership.\n"
    )
    markdown += (
        "- `ΔRM`/`ΔSnow` carry HPHYS0293 excluded snow-producer comparator "
        "differences; WB18/WB19 production edits require residual accounting "
        "that survives this mask.\n"
    )
    (reports / "hphys0294_target_storage_rows.md").write_text(markdown, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", required=True, type=Path)
    parser.add_argument("--trace-max-days", type=int, default=1_800)
    parser.add_argument("--skip-full-suite", action="store_true")
    args = parser.parse_args()

    for required in [
        HPHYS0265.WEPPPY_PYTHON,
        HPHYS0265.COMPARATOR,
        HPHYS0265.TOLERANCES,
        HPHYS0265.BASELINE_PARTITIONS,
    ]:
        HPHYS0265.require_path(required)

    if not args.skip_full_suite:
        full_rc = HPHYS0291.run_full_hillslope_suite(args.run_root)
        if full_rc != 0:
            return int(full_rc)
        normalize_full_suite_summary_label(args.run_root)
        write_selected_metrics(args.run_root)

    trace_rc = run_targeted_traces(args.run_root, args.trace_max_days)
    if trace_rc != 0:
        return int(trace_rc)
    rows = extract_target_storage_rows(args.run_root)
    write_target_storage_markdown(args.run_root, rows)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
