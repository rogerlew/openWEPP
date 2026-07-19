#!/usr/bin/env python3
"""Run HPHYS0295 cumulative storage-budget ownership diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import math
import sys
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
HPHYS0291_SCRIPT = (
    REPO
    / "docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/hphys0291_diagnostics.py"
)
TARGET_DAYS = {
    1: [(2014, day) for day in range(120, 147)] + [(2016, day) for day in range(104, 112)],
    7: [(2014, day) for day in range(120, 147)] + [(2016, day) for day in range(104, 112)],
    39: [(2014, day) for day in range(120, 147)] + [(2016, day) for day in range(104, 112)],
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
BUDGET_SYMBOLS = ["Ep", "Es", "Er", "Dp", "latqcc", "Q", "RM", "Snow-Water", "Total-Soil"]
WAT_COLUMNS = {
    "Ep": ("Ep_candidate", "Ep_baseline"),
    "Es": ("Es_candidate", "Es_baseline"),
    "Er": ("Er_candidate", "Er_baseline"),
    "Dp": ("Dp_candidate", "Dp_baseline"),
    "latqcc": ("latqcc_candidate", "latqcc_baseline"),
    "Q": ("Q_candidate", "Q_baseline"),
    "RM": ("RM_candidate", "RM_baseline"),
    "Snow-Water": ("Snow-Water_candidate", "Snow-Water_baseline"),
    "Total-Soil": ("Total-Soil", "Total-Soil Water"),
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


def rounded(value: Any, digits: int = 6) -> Any:
    if value is None:
        return None
    if isinstance(value, float):
        if math.isnan(value):
            return None
        return round(value, digits)
    return value


def trace_value(row: dict[str, Any] | None, key: str) -> float | None:
    if row is None or row.get(key) is None:
        return None
    return float(row[key])


def trace_mm(row: dict[str, Any] | None, key: str) -> float | None:
    row_value = trace_value(row, key)
    if row_value is None:
        return None
    return row_value * 1_000.0


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


def load_trace_index(trace_path: Path) -> dict[tuple[int, int], dict[str, Any]]:
    rows: dict[tuple[int, int], dict[str, Any]] = {}
    if not trace_path.exists():
        return rows
    with trace_path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            if row.get("boundary") != "post_wb13":
                continue
            rows[(int(row.get("calendar_year", -1)), int(row.get("julian_day", -1)))] = row
    return rows


def merged_wat_rows(run_root: Path, hillslope_id: int) -> Any:
    return HPHYS0265.candidate_baseline_merge(
        run_root / f"hillslope_output/H{hillslope_id}.wat.parquet",
        HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet",
        candidate_year_offset=2012,
    ).sort_values(["_comparison_year", "julian"])


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
        "# HPHYS0295 Full H1..H39 Semantic Summary",
        1,
    )
    summary_path.write_text(summary, encoding="utf-8")


def write_selected_metrics(run_root: Path) -> None:
    reports = run_root / "reports"
    summary_json = reports / "hillslope_semantic_summary.json"
    summary = json.loads(summary_json.read_text(encoding="utf-8"))
    selected = {row["column"]: row for row in summary if row["column"] in SELECTED_COLUMNS}
    (reports / "hphys0295_selected_metrics.json").write_text(
        json.dumps(selected, indent=2) + "\n", encoding="utf-8"
    )


def run_targeted_traces(run_root: Path, trace_max_days: int) -> int:
    reports = run_root / "reports"
    logs = run_root / "logs" / "targeted_traces"
    output = run_root / "hillslope_output"
    runs_dir = HPHYS0265.copy_runfiles(run_root)
    status_rows: list[dict[str, Any]] = []
    for hillslope_id in TARGET_DAYS:
        trace_path = output / f"H{hillslope_id}.hphys0295.trace.jsonl"
        result = HPHYS0265.run_command(
            f"H{hillslope_id}_hphys0295_trace",
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
            HPHYS0265.write_status(reports / "hphys0295_target_trace_status.tsv", status_rows)
            return int(result.rc)
    HPHYS0265.write_status(reports / "hphys0295_target_trace_status.tsv", status_rows)
    return 0


def build_budget_frame(merged: Any) -> Any:
    frame = merged[["_comparison_year", "julian"]].copy()
    for symbol in BUDGET_SYMBOLS:
        candidate_column, baseline_column = WAT_COLUMNS[symbol]
        frame[f"candidate_{symbol}"] = merged[candidate_column].astype(float)
        frame[f"baseline_{symbol}"] = merged[baseline_column].astype(float)
        frame[f"delta_{symbol}"] = frame[f"candidate_{symbol}"] - frame[f"baseline_{symbol}"]
    frame["delta_ET"] = frame["delta_Ep"] + frame["delta_Es"] + frame["delta_Er"]
    frame["storage_residual_change"] = frame["delta_Total-Soil"].diff()
    frame["known_flux_delta"] = (
        frame["delta_RM"]
        - frame["delta_Q"]
        - frame["delta_ET"]
        - frame["delta_Dp"]
        - frame["delta_latqcc"]
    )
    frame["budget_gap"] = frame["storage_residual_change"] - frame["known_flux_delta"]
    return frame


def window_slice(frame: Any, year: int, start_julian: int, end_julian: int) -> Any:
    return frame[
        (frame["_comparison_year"] == year)
        & (frame["julian"] >= start_julian)
        & (frame["julian"] <= end_julian)
    ].copy()


def first_large_window(frame: Any, threshold_mm: float) -> tuple[int, int, int] | None:
    matches = frame[frame["delta_Total-Soil"].abs() >= threshold_mm]
    if matches.empty:
        return None
    first = matches.iloc[0]
    year = int(first["_comparison_year"])
    julian = int(first["julian"])
    return year, max(1, julian - 5), julian + 10


def summarize_window(hillslope_id: int, label: str, frame: Any) -> dict[str, Any] | None:
    if frame.empty:
        return None
    first = frame.iloc[0]
    last = frame.iloc[-1]
    storage_change = float(last["delta_Total-Soil"] - first["delta_Total-Soil"])
    known_flux_delta = float(frame["known_flux_delta"].fillna(0.0).sum())
    budget_gap = storage_change - known_flux_delta
    cumulative = {
        "delta_ep_sum_mm": float(frame["delta_Ep"].fillna(0.0).sum()),
        "delta_es_sum_mm": float(frame["delta_Es"].fillna(0.0).sum()),
        "delta_er_sum_mm": float(frame["delta_Er"].fillna(0.0).sum()),
        "delta_et_sum_mm": float(frame["delta_ET"].fillna(0.0).sum()),
        "delta_dp_sum_mm": float(frame["delta_Dp"].fillna(0.0).sum()),
        "delta_latqcc_sum_mm": float(frame["delta_latqcc"].fillna(0.0).sum()),
        "delta_q_sum_mm": float(frame["delta_Q"].fillna(0.0).sum()),
        "delta_rm_sum_mm": float(frame["delta_RM"].fillna(0.0).sum()),
        "delta_snow_water_start_mm": float(first["delta_Snow-Water"]),
        "delta_snow_water_end_mm": float(last["delta_Snow-Water"]),
    }
    magnitudes = {
        "ET": abs(cumulative["delta_et_sum_mm"]),
        "Dp": abs(cumulative["delta_dp_sum_mm"]),
        "latqcc": abs(cumulative["delta_latqcc_sum_mm"]),
        "RM": abs(cumulative["delta_rm_sum_mm"]),
        "budget_gap": abs(budget_gap),
    }
    dominant = max(magnitudes, key=magnitudes.get)
    return {
        "hillslope_id": hillslope_id,
        "label": label,
        "year": int(first["_comparison_year"]),
        "start_julian": int(first["julian"]),
        "end_julian": int(last["julian"]),
        "start_storage_residual_mm": float(first["delta_Total-Soil"]),
        "end_storage_residual_mm": float(last["delta_Total-Soil"]),
        "storage_residual_change_mm": storage_change,
        "known_flux_delta_mm": known_flux_delta,
        "budget_gap_mm": budget_gap,
        "dominant_residual_term": dominant,
        **cumulative,
    }


def extract_budget_rows(run_root: Path) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    reports = run_root / "reports"
    output = run_root / "hillslope_output"
    row_records: list[dict[str, Any]] = []
    windows: list[dict[str, Any]] = []
    for hillslope_id, days in TARGET_DAYS.items():
        merged = merged_wat_rows(run_root, hillslope_id)
        budget_frame = build_budget_frame(merged)
        candidate_windows: list[tuple[str, int, int, int]] = [
            ("spring-2014", 2014, 120, 146),
            ("spring-2016", 2016, 104, 111),
        ]
        first_window = first_large_window(budget_frame, threshold_mm=10.0)
        if first_window is not None:
            candidate_windows.insert(0, ("first-abs-storage-ge-10mm", *first_window))
        for label, year, start_julian, end_julian in candidate_windows:
            summary = summarize_window(
                hillslope_id, label, window_slice(budget_frame, year, start_julian, end_julian)
            )
            if summary is not None:
                windows.append(summary)

        trace_path = output / f"H{hillslope_id}.hphys0295.trace.jsonl"
        trace_rows = load_trace_index(trace_path)
        for year, julian in days:
            matches = budget_frame[
                (budget_frame["_comparison_year"] == year) & (budget_frame["julian"] == julian)
            ]
            if matches.empty:
                continue
            row = matches.iloc[0]
            trace = trace_rows.get((year, julian))
            row_records.append(
                {
                    "hillslope_id": hillslope_id,
                    "year": year,
                    "julian": julian,
                    "delta_total_soil_mm": float(row["delta_Total-Soil"]),
                    "storage_residual_change_mm": None
                    if math.isnan(float(row["storage_residual_change"]))
                    else float(row["storage_residual_change"]),
                    "known_flux_delta_mm": float(row["known_flux_delta"]),
                    "budget_gap_mm": None
                    if math.isnan(float(row["budget_gap"]))
                    else float(row["budget_gap"]),
                    "delta_ep_mm": float(row["delta_Ep"]),
                    "delta_es_mm": float(row["delta_Es"]),
                    "delta_er_mm": float(row["delta_Er"]),
                    "delta_et_mm": float(row["delta_ET"]),
                    "delta_dp_mm": float(row["delta_Dp"]),
                    "delta_latqcc_mm": float(row["delta_latqcc"]),
                    "delta_q_mm": float(row["delta_Q"]),
                    "delta_rm_mm": float(row["delta_RM"]),
                    "delta_snow_water_mm": float(row["delta_Snow-Water"]),
                    "trace_ep_mm": trace_mm(trace, "ep_m"),
                    "trace_etp_mm": trace_mm(trace, "etp_m"),
                    "trace_ui_mm": trace_mm(trace, "ui_m"),
                    "trace_pmet_es_mm": trace_mm(trace, "pmet_es_m"),
                    "trace_pmet_ep_mm": trace_mm(trace, "pmet_ep_m"),
                    "trace_d_mm": trace_mm(trace, "d_m"),
                    "trace_pe_mm": trace_mm(trace, "pe_m"),
                    "trace_wb18_identity_residual_mm": trace_mm(
                        trace, "wb18_recomputed_minus_wb11_m"
                    ),
                    "trace_wb19_target_mm": trace_mm(trace, "wb19_q_lateral_target_m"),
                    "trace_wb19_unrealized_mm": trace_mm(
                        trace, "wb19_q_lateral_unrealized_m"
                    ),
                    "trace_snow_swe_delta_mm": trace_mm(trace, "snow_runtime_swe_delta_m"),
                    "trace_snow_routed_melt_mm": trace_mm(trace, "snow_routed_melt_m"),
                }
            )
    (reports / "hphys0295_budget_windows.json").write_text(
        json.dumps(windows, indent=2) + "\n", encoding="utf-8"
    )
    (reports / "hphys0295_budget_rows.json").write_text(
        json.dumps(row_records, indent=2) + "\n", encoding="utf-8"
    )
    return windows, row_records


def write_budget_markdown(
    run_root: Path, windows: list[dict[str, Any]], rows: list[dict[str, Any]]
) -> None:
    reports = run_root / "reports"
    markdown = "# HPHYS0295 Cumulative Storage-Budget Diagnostics\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Run root: `{run_root}`\n"
    markdown += f"- Budget windows JSON: `{reports / 'hphys0295_budget_windows.json'}`\n"
    markdown += f"- Budget rows JSON: `{reports / 'hphys0295_budget_rows.json'}`\n"
    markdown += f"- Trace status: `{reports / 'hphys0295_target_trace_status.tsv'}`\n\n"

    markdown += "## Window Budgets\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Window",
            "Year",
            "Start",
            "End",
            "Start ΔStorage",
            "End ΔStorage",
            "ΔStorage Change",
            "Σ Known Flux",
            "Budget Gap",
            "ΣΔET",
            "ΣΔDp",
            "ΣΔlatqcc",
            "ΣΔRM",
            "Snow Start",
            "Snow End",
            "Dominant",
        ],
        [
            [
                f"H{row['hillslope_id']}",
                row["label"],
                row["year"],
                row["start_julian"],
                row["end_julian"],
                rounded(row["start_storage_residual_mm"]),
                rounded(row["end_storage_residual_mm"]),
                rounded(row["storage_residual_change_mm"]),
                rounded(row["known_flux_delta_mm"]),
                rounded(row["budget_gap_mm"]),
                rounded(row["delta_et_sum_mm"]),
                rounded(row["delta_dp_sum_mm"]),
                rounded(row["delta_latqcc_sum_mm"]),
                rounded(row["delta_rm_sum_mm"]),
                rounded(row["delta_snow_water_start_mm"]),
                rounded(row["delta_snow_water_end_mm"]),
                row["dominant_residual_term"],
            ]
            for row in windows
        ],
    )
    markdown += "\n## Target Rows\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Year",
            "Julian",
            "ΔStorage",
            "ΔStorage Step",
            "Known Flux",
            "Budget Gap",
            "ΔET",
            "ΔDp",
            "Δlatqcc",
            "ΔRM",
            "ΔSnow",
            "Trace Ep",
            "Trace D",
            "Trace WB19 Target",
            "WB18 ID Residual",
        ],
        [
            [
                f"H{row['hillslope_id']}",
                row["year"],
                row["julian"],
                rounded(row["delta_total_soil_mm"]),
                rounded(row["storage_residual_change_mm"]),
                rounded(row["known_flux_delta_mm"]),
                rounded(row["budget_gap_mm"]),
                rounded(row["delta_et_mm"]),
                rounded(row["delta_dp_mm"]),
                rounded(row["delta_latqcc_mm"]),
                rounded(row["delta_rm_mm"]),
                rounded(row["delta_snow_water_mm"]),
                rounded(row["trace_ep_mm"]),
                rounded(row["trace_d_mm"]),
                rounded(row["trace_wb19_target_mm"]),
                rounded(row["trace_wb18_identity_residual_mm"], 9),
            ]
            for row in rows
        ],
    )
    markdown += "\n## Interpretation\n\n"
    markdown += (
        "- `Known Flux` uses WAT-scale diagnostic accounting: `ΔRM - ΔQ - ΔET - ΔDp - Δlatqcc`.\n"
    )
    markdown += (
        "- `Budget Gap` is `ΔStorage step - Known Flux`; a large gap means the row-to-row "
        "storage residual is not explained by the daily WAT flux deltas alone.\n"
    )
    markdown += (
        "- HPHYS0295 is an ownership classifier. It does not authorize production edits "
        "unless a term dominates after excluded snow/`RM` masks and trace identities are separated.\n"
    )
    (reports / "hphys0295_cumulative_budget.md").write_text(markdown, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", required=True, type=Path)
    parser.add_argument("--trace-max-days", type=int, default=1_800)
    parser.add_argument("--skip-full-suite", action="store_true")
    parser.add_argument("--skip-targeted-traces", action="store_true")
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

    if not args.skip_targeted_traces:
        trace_rc = run_targeted_traces(args.run_root, args.trace_max_days)
        if trace_rc != 0:
            return int(trace_rc)
    windows, rows = extract_budget_rows(args.run_root)
    write_budget_markdown(args.run_root, windows, rows)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
