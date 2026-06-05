#!/usr/bin/env python3
"""Run HPHYS0297 snow/RM defect-ledger reconstruction diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import math
import sys
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
HPHYS0296_SCRIPT = (
    REPO
    / "docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/artifacts/hphys0296_diagnostics.py"
)
SELECTED_COLUMNS = {
    "Ep",
    "Total-Soil",
    "SoilWaterTotal",
    "Dp",
    "latqcc",
    "Q",
    "RM",
    "Snow-Water",
}
RECONSTRUCTION_TOLERANCE_MM = 2.0
NEGATIVE_MELT_MATERIALITY_MM = 0.5
BASELINE_SOURCE = "/workdir/wepp-forest_260430_baseline/src/winter.for:434-448"
OPENWEPP_SOURCE = (
    "crates/openwepp-hillslope-orchestrator/src/hydrology/"
    "03_kernel_support_00_support_helpers.rs:4231-4276"
)


def load_hphys0296_module() -> Any:
    spec = importlib.util.spec_from_file_location("hphys0296_diagnostics", HPHYS0296_SCRIPT)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {HPHYS0296_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0296 = load_hphys0296_module()
HPHYS0295 = HPHYS0296.HPHYS0295
HPHYS0291 = HPHYS0296.HPHYS0291
HPHYS0265 = HPHYS0296.HPHYS0265
TARGET_WINDOWS = HPHYS0296.TARGET_WINDOWS


def rounded(value: Any, digits: int = 6) -> Any:
    if value is None:
        return None
    if isinstance(value, float):
        if math.isnan(value):
            return None
        return round(value, digits)
    return value


def wat_candidate(row: Any | None, symbol: str) -> float:
    if row is None:
        return 0.0
    value = HPHYS0295.wat_candidate(row, symbol)
    return 0.0 if value is None else float(value)


def wat_baseline(row: Any | None, symbol: str) -> float:
    if row is None:
        return 0.0
    value = HPHYS0295.wat_baseline(row, symbol)
    return 0.0 if value is None else float(value)


def wat_delta(row: Any | None, symbol: str) -> float:
    if row is None:
        return 0.0
    value = HPHYS0295.wat_delta(row, symbol)
    return 0.0 if value is None else float(value)


def trace_m_to_mm(row: dict[str, Any] | None, key: str) -> float:
    if row is None:
        return 0.0
    value = row.get(key)
    if value is None:
        return 0.0
    return float(value) * 1_000.0


def trace_mm(row: dict[str, Any] | None, key: str) -> float:
    if row is None:
        return 0.0
    value = row.get(key)
    if value is None:
        return 0.0
    return float(value)


def trace_map_values_mm(row: dict[str, Any] | None, key: str) -> list[float]:
    if row is None:
        return []
    values = row.get(key)
    if not isinstance(values, dict):
        return []
    return [float(value) * 1_000.0 for value in values.values()]


def normalize_full_suite_summary_label(run_root: Path) -> None:
    summary_path = run_root / "reports/hillslope_semantic_summary.md"
    if not summary_path.exists():
        return
    summary = summary_path.read_text(encoding="utf-8")
    summary = summary.replace(
        "# HPHYS0291 Full H1..H39 Semantic Summary",
        "# HPHYS0297 Full H1..H39 Semantic Summary",
        1,
    )
    summary = summary.replace(
        "# HPHYS0296 Full H1..H39 Semantic Summary",
        "# HPHYS0297 Full H1..H39 Semantic Summary",
        1,
    )
    summary_path.write_text(summary, encoding="utf-8")


def write_selected_metrics(run_root: Path) -> dict[str, Any]:
    reports = run_root / "reports"
    summary_json = reports / "hillslope_semantic_summary.json"
    summary = json.loads(summary_json.read_text(encoding="utf-8"))
    selected = {row["column"]: row for row in summary if row["column"] in SELECTED_COLUMNS}
    (reports / "hphys0297_selected_metrics.json").write_text(
        json.dumps(selected, indent=2) + "\n", encoding="utf-8"
    )
    return selected


def run_targeted_traces(run_root: Path, trace_max_days: int) -> int:
    reports = run_root / "reports"
    logs = run_root / "logs" / "targeted_traces"
    output = run_root / "hillslope_output"
    runs_dir = HPHYS0265.copy_runfiles(run_root)
    status_rows: list[dict[str, Any]] = []
    for hillslope_id in TARGET_WINDOWS:
        trace_path = output / f"H{hillslope_id}.hphys0297.trace.jsonl"
        result = HPHYS0265.run_command(
            f"H{hillslope_id}_hphys0297_trace",
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
            HPHYS0265.write_status(reports / "hphys0297_target_trace_status.tsv", status_rows)
            return int(result.rc)
    HPHYS0265.write_status(reports / "hphys0297_target_trace_status.tsv", status_rows)
    return 0


def target_keys_for_hill(hillslope_id: int) -> set[tuple[int, int]]:
    keys: set[tuple[int, int]] = set()
    for _, year, start, end in TARGET_WINDOWS[hillslope_id]:
        for julian in range(start, end + 1):
            keys.add((year, julian))
    return keys


def load_target_trace_index(trace_path: Path, target_keys: set[tuple[int, int]]) -> dict[tuple[int, int], dict[str, Any]]:
    index: dict[tuple[int, int], dict[str, Any]] = {}
    with trace_path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            year = row.get("calendar_year")
            julian = row.get("julian_day")
            if year is None or julian is None:
                continue
            key = (int(year), int(julian))
            if key in target_keys:
                index[key] = row
    return index


def baseline_bug_reconstructed_melt_mm(raw_values_mm: list[float]) -> float:
    positive_melt_mm = sum(value for value in raw_values_mm if value > 0.0)
    negative_melt_mm = sum(value for value in raw_values_mm if value < 0.0)
    if positive_melt_mm <= 1.0e-12:
        return 0.0
    if positive_melt_mm <= negative_melt_mm:
        return 0.0
    scale = 1.0 - negative_melt_mm / positive_melt_mm
    return (positive_melt_mm + negative_melt_mm) * scale


def corrected_reconstructed_melt_mm(raw_values_mm: list[float]) -> float:
    positive_melt_mm = sum(value for value in raw_values_mm if value > 0.0)
    negative_melt_mm = sum(value for value in raw_values_mm if value < 0.0)
    if positive_melt_mm <= 1.0e-12:
        return 0.0
    return max(positive_melt_mm + negative_melt_mm, 0.0)


def verdict_for(row: dict[str, Any]) -> tuple[str, str, str]:
    if abs(row["negative_raw_melt_sum_mm"]) < NEGATIVE_MELT_MATERIALITY_MM:
        return (
            "UNRESOLVED",
            "Negative raw melt is immaterial; the window remains a snow/winter producer magnitude/timing hold.",
            "No independent correctness adjudication is possible from negative-melt reconstruction.",
        )
    if abs(row["reconstruction_minus_baseline_rm_mm"]) > RECONSTRUCTION_TOLERANCE_MM:
        return (
            "UNRESOLVED",
            "Pinned-baseline branch reconstruction does not reproduce baseline RM to the named tolerance.",
            "Causality is unproven; correlation plus internal closure is insufficient.",
        )
    return (
        "LEGACY-DEFECTIVE",
        "Pinned-baseline negative-melt sign/scale branch reconstructs the comparator residual to tolerance.",
        "Negative melt is a cooling/refreezing term; increasing routed melt with `1 - ngtvML/pstvML` violates signed melt conservation, while corrected openWEPP routes net positive melt.",
    )


def analyze_window(
    hillslope_id: int,
    merged: Any,
    trace_index: dict[tuple[int, int], dict[str, Any]],
    window: tuple[str, int, int, int],
) -> dict[str, Any]:
    name, year, start, end = window
    rows = merged[
        (merged["_comparison_year"] == year)
        & (merged["julian"] >= start)
        & (merged["julian"] <= end)
    ].sort_values(["_comparison_year", "julian"])
    result: dict[str, Any] = {
        "hillslope_id": hillslope_id,
        "window": name,
        "year": year,
        "start_julian": start,
        "end_julian": end,
        "row_count": int(len(rows)),
        "baseline_source": BASELINE_SOURCE,
        "openwepp_source": OPENWEPP_SOURCE,
        "reconstruction_tolerance_mm": RECONSTRUCTION_TOLERANCE_MM,
        "negative_raw_melt_sum_mm": 0.0,
        "positive_raw_melt_sum_mm": 0.0,
        "candidate_rm_sum_mm": 0.0,
        "baseline_rm_sum_mm": 0.0,
        "candidate_q_sum_mm": 0.0,
        "baseline_q_sum_mm": 0.0,
        "candidate_snow_sum_mm": 0.0,
        "baseline_snow_sum_mm": 0.0,
        "trace_candidate_rm_sum_mm": 0.0,
        "baseline_bug_reconstructed_rm_sum_mm": 0.0,
        "corrected_reconstructed_rm_sum_mm": 0.0,
        "rm_identity_abs_sum_mm": 0.0,
        "snow_closure_abs_sum_mm": 0.0,
        "negative_raw_melt_day_count": 0,
        "negative_raw_melt_hour_count": 0,
        "trace_missing_day_count": 0,
    }
    for _, row in rows.iterrows():
        key = (int(row["_comparison_year"]), int(row["julian"]))
        trace = trace_index.get(key)
        if trace is None:
            result["trace_missing_day_count"] += 1
        raw_values = trace_map_values_mm(trace, "snow_hourly_melt_raw_m")
        positive_raw = sum(value for value in raw_values if value > 0.0)
        negative_raw = sum(value for value in raw_values if value < 0.0)
        if negative_raw < -1.0e-9:
            result["negative_raw_melt_day_count"] += 1
            result["negative_raw_melt_hour_count"] += sum(1 for value in raw_values if value < -1.0e-9)
        post_winter_rain_mm = trace_m_to_mm(trace, "snow_post_winter_rain_m")
        trace_routed_melt_mm = trace_m_to_mm(trace, "snow_routed_melt_m")
        trace_rm_mm = trace_mm(trace, "wb13_rm_mm")
        if trace_rm_mm == 0.0:
            trace_rm_mm = trace_routed_melt_mm + post_winter_rain_mm
        rm_identity = trace_rm_mm - (trace_routed_melt_mm + post_winter_rain_mm)
        result["positive_raw_melt_sum_mm"] += positive_raw
        result["negative_raw_melt_sum_mm"] += negative_raw
        result["trace_candidate_rm_sum_mm"] += trace_routed_melt_mm + post_winter_rain_mm
        result["baseline_bug_reconstructed_rm_sum_mm"] += (
            baseline_bug_reconstructed_melt_mm(raw_values) + post_winter_rain_mm
        )
        result["corrected_reconstructed_rm_sum_mm"] += (
            corrected_reconstructed_melt_mm(raw_values) + post_winter_rain_mm
        )
        result["rm_identity_abs_sum_mm"] += abs(rm_identity)
        result["snow_closure_abs_sum_mm"] += abs(
            trace_m_to_mm(trace, "snow_runtime_swe_closure_error_m")
        )
        result["candidate_rm_sum_mm"] += wat_candidate(row, "RM")
        result["baseline_rm_sum_mm"] += wat_baseline(row, "RM")
        result["candidate_q_sum_mm"] += wat_candidate(row, "Q")
        result["baseline_q_sum_mm"] += wat_baseline(row, "Q")
        result["candidate_snow_sum_mm"] += wat_candidate(row, "Snow-Water")
        result["baseline_snow_sum_mm"] += wat_baseline(row, "Snow-Water")

    result["observed_baseline_minus_candidate_rm_mm"] = (
        result["baseline_rm_sum_mm"] - result["candidate_rm_sum_mm"]
    )
    result["reconstructed_minus_candidate_rm_mm"] = (
        result["baseline_bug_reconstructed_rm_sum_mm"] - result["trace_candidate_rm_sum_mm"]
    )
    result["reconstruction_minus_baseline_rm_mm"] = (
        result["baseline_bug_reconstructed_rm_sum_mm"] - result["baseline_rm_sum_mm"]
    )
    result["corrected_reconstruction_minus_candidate_rm_mm"] = (
        result["corrected_reconstructed_rm_sum_mm"] - result["trace_candidate_rm_sum_mm"]
    )
    verdict, reason, correctness = verdict_for(result)
    result["verdict"] = verdict
    result["verdict_reason"] = reason
    result["independent_correctness_rationale"] = correctness
    return result


def write_defect_ledger(run_root: Path) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    ledger: list[dict[str, Any]] = []
    for hillslope_id, target_windows in TARGET_WINDOWS.items():
        merged = HPHYS0295.merged_wat_rows(run_root, hillslope_id)
        trace_index = load_target_trace_index(
            run_root / f"hillslope_output/H{hillslope_id}.hphys0297.trace.jsonl",
            target_keys_for_hill(hillslope_id),
        )
        for window in target_windows:
            ledger.append(analyze_window(hillslope_id, merged, trace_index, window))

    (reports / "hphys0297_defect_ledger.json").write_text(
        json.dumps(ledger, indent=2) + "\n", encoding="utf-8"
    )

    verdict_counts: dict[str, int] = {}
    for row in ledger:
        verdict_counts[row["verdict"]] = verdict_counts.get(row["verdict"], 0) + 1

    headers = [
        "Hill",
        "Window",
        "Year",
        "Days",
        "Verdict",
        "Obs baseline-candidate RM",
        "Recon baseline-branch RM",
        "Baseline RM",
        "Recon-baseline",
        "Neg raw melt",
        "Q delta",
        "Reason",
    ]
    rows = [
        [
            f"H{row['hillslope_id']}",
            row["window"],
            row["year"],
            f"{row['start_julian']}-{row['end_julian']}",
            row["verdict"],
            rounded(row["observed_baseline_minus_candidate_rm_mm"]),
            rounded(row["baseline_bug_reconstructed_rm_sum_mm"]),
            rounded(row["baseline_rm_sum_mm"]),
            rounded(row["reconstruction_minus_baseline_rm_mm"]),
            rounded(row["negative_raw_melt_sum_mm"]),
            rounded(row["baseline_q_sum_mm"] - row["candidate_q_sum_mm"]),
            row["verdict_reason"],
        ]
        for row in ledger
    ]
    markdown = "# HPHYS0297 Snow/RM Defect Ledger Reconstruction\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Run root: `{run_root}`\n"
    markdown += f"- Ledger JSON: `{reports / 'hphys0297_defect_ledger.json'}`\n"
    markdown += f"- Trace status: `{reports / 'hphys0297_target_trace_status.tsv'}`\n"
    markdown += f"- Reconstruction tolerance: `{RECONSTRUCTION_TOLERANCE_MM:.3f} mm` window-sum absolute residual.\n"
    markdown += f"- Baseline source: `{BASELINE_SOURCE}`\n"
    markdown += f"- openWEPP source: `{OPENWEPP_SOURCE}`\n\n"
    markdown += "## Verdict Counts\n\n"
    for verdict, count in sorted(verdict_counts.items()):
        markdown += f"- `{verdict}`: `{count}` windows\n"
    markdown += "\n## Ledger\n\n"
    markdown += HPHYS0265.markdown_table(headers, rows)
    markdown += "\n\n## Interpretation\n\n"
    markdown += (
        "- `LEGACY-DEFECTIVE` requires the pinned-baseline branch reconstruction "
        "to reproduce baseline `RM` within the named tolerance and an independent "
        "correctness rationale showing why the baseline branch is defective.\n"
    )
    markdown += (
        "- `UNRESOLVED` rows remain in the failing set. They are not accepted, "
        "excluded, or re-tiered by this package.\n"
    )
    markdown += (
        "- Closed `Q` and closed producer-consumer identity are evidence for "
        "excluding runoff/storage compensation only; they are not acceptance "
        "authority.\n"
    )
    (reports / "hphys0297_reconstruction_summary.md").write_text(markdown, encoding="utf-8")
    return ledger


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-root", type=Path, required=True)
    parser.add_argument("--trace-max-days", type=int, default=1_800)
    parser.add_argument("--skip-full-suite", action="store_true")
    parser.add_argument("--skip-targeted-traces", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    args.run_root.mkdir(parents=True, exist_ok=True)
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
    write_selected_metrics(args.run_root)
    write_defect_ledger(args.run_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
