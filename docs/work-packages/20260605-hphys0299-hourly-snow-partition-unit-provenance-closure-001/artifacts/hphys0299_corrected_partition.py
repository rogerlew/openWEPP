#!/usr/bin/env python3
"""Run HPHYS0299 corrected paired snowfall-depth partition diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import math
import subprocess
import sys
from collections import Counter
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
HPHYS0298_SCRIPT = (
    REPO
    / "docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py"
)

canonical_hrsnow_openwepp_field = "snow_hourly_snowfall_depth_sum_m"
CANONICAL_HRSNOW_PROVENANCE = {
    "baseline_partition_path": "/workdir/wepp-forest_260430_baseline/src/winter.for:296-300",
    "baseline_stmtim_path": "/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95",
    "baseline_equation": "hrsnow(hour) = rain / wntdur * 10.0",
    "openwepp_field": canonical_hrsnow_openwepp_field,
    "comparison": "depth-vs-depth",
    "rejected_field": "snow_hourly_snowfall_water_equiv_sum_m",
    "rejected_reason": "derived density-weighted water-equivalent summary, not canonical hrsnow depth",
}


def load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0298 = load_module(HPHYS0298_SCRIPT, "hphys0298_paired_lineage_partition")
HPHYS0291 = HPHYS0298.HPHYS0291
HPHYS0295 = HPHYS0298.HPHYS0295
HPHYS0265 = HPHYS0298.HPHYS0265
BASELINE_RELEASE_BIN = HPHYS0298.BASELINE_RELEASE_BIN
BASELINE_OBSERVE_BIN = HPHYS0298.BASELINE_OBSERVE_BIN
BASELINE_SOURCE = HPHYS0298.BASELINE_SOURCE
BASELINE_COMMIT = HPHYS0298.BASELINE_COMMIT
BASELINE_OBSERVE_WORKTREE = HPHYS0298.BASELINE_OBSERVE_WORKTREE
TARGET_WINDOWS = HPHYS0298.TARGET_WINDOWS
TARGET_HILLS = HPHYS0298.TARGET_HILLS
WINDOW_TOLERANCE_MM = HPHYS0298.WINDOW_TOLERANCE_MM
SELECTED_COLUMNS = HPHYS0298.SELECTED_COLUMNS

REQUIRED_OPENWEPP_TRACE_FIELDS = (
    "snow_hourly_melt_raw_m",
    "snow_hourly_rain_sum_m",
    canonical_hrsnow_openwepp_field,
    "snow_hourly_snowfall_water_equiv_sum_m",
    "snow_routed_melt_m",
    "snow_post_winter_rain_m",
    "wb13_rm_mm",
    "wb13_q_mm",
)
TRACE_MAP_FIELDS = frozenset({"snow_hourly_melt_raw_m"})
BASELINE_SOURCES = dict(HPHYS0298.BASELINE_SOURCES)
BASELINE_SOURCES.update(
    {
        "hrrain": "/workdir/wepp-forest_260430_baseline/src/stmtim.for:92",
        "hrsnow": "/workdir/wepp-forest_260430_baseline/src/stmtim.for:94",
    }
)
OPENWEPP_SOURCES = dict(HPHYS0298.OPENWEPP_SOURCES)
OPENWEPP_SOURCES.update(
    {
        "snow_hourly_snowfall_depth_sum_m": "crates/openwepp-runner/src/hillslope/mod.rs:4520",
        "snow_hourly_snowfall_water_equiv_sum_m": "crates/openwepp-runner/src/hillslope/mod.rs:4606",
    }
)


def rounded(value: Any, digits: int = 6) -> Any:
    if value is None:
        return None
    if isinstance(value, float):
        if math.isnan(value):
            return None
        return round(value, digits)
    return value


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def trace_m_to_mm(row: dict[str, Any] | None, key: str) -> float:
    return HPHYS0298.trace_m_to_mm(row, key)


def trace_mm(row: dict[str, Any] | None, key: str) -> float:
    return HPHYS0298.trace_mm(row, key)


def trace_map_values_mm(row: dict[str, Any] | None, key: str) -> list[float]:
    return HPHYS0298.trace_map_values_mm(row, key)


def validate_trace_fields(row: dict[str, Any], year: int, julian: int) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for field in REQUIRED_OPENWEPP_TRACE_FIELDS:
        if field not in row or row[field] is None:
            issues.append({"year": year, "julian": julian, "field": field, "reason": "missing"})
            continue
        value = row[field]
        if field in TRACE_MAP_FIELDS:
            if not isinstance(value, dict):
                issues.append(
                    {"year": year, "julian": julian, "field": field, "reason": "not-map"}
                )
                continue
            for hour, hour_value in value.items():
                try:
                    numeric = float(hour_value)
                except (TypeError, ValueError):
                    issues.append(
                        {
                            "year": year,
                            "julian": julian,
                            "field": field,
                            "hour": hour,
                            "reason": "non-numeric",
                        }
                    )
                    continue
                if not math.isfinite(numeric):
                    issues.append(
                        {
                            "year": year,
                            "julian": julian,
                            "field": field,
                            "hour": hour,
                            "reason": "non-finite",
                        }
                    )
            continue
        try:
            numeric = float(value)
        except (TypeError, ValueError):
            issues.append(
                {"year": year, "julian": julian, "field": field, "reason": "non-numeric"}
            )
            continue
        if not math.isfinite(numeric):
            issues.append(
                {"year": year, "julian": julian, "field": field, "reason": "non-finite"}
            )
    return issues


def normalize_full_suite_summary_label(run_root: Path) -> None:
    summary_path = run_root / "reports/hillslope_semantic_summary.md"
    if not summary_path.exists():
        return
    summary = summary_path.read_text(encoding="utf-8")
    for label in ("HPHYS0291", "HPHYS0295", "HPHYS0296", "HPHYS0297", "HPHYS0298"):
        summary = summary.replace(
            f"# {label} Full H1..H39 Semantic Summary",
            "# HPHYS0299 Full H1..H39 Semantic Summary",
            1,
        )
    summary_path.write_text(summary, encoding="utf-8")


def current_git_head() -> str:
    proc = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=REPO,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    return proc.stdout.strip() if proc.returncode == 0 else "unknown"


def write_selected_metrics(run_root: Path) -> dict[str, Any]:
    reports = run_root / "reports"
    selected = {
        row["column"]: row
        for row in read_json(reports / "hillslope_semantic_summary.json")
        if row["column"] in SELECTED_COLUMNS
    }
    write_json(reports / "hphys0299_selected_metrics.json", selected)
    return selected


def write_full39_metrics_artifact(run_root: Path, artifact_dir: Path, selected: dict[str, Any]) -> None:
    summary_json = run_root / "reports/hillslope_semantic_summary.json"
    summary_md = run_root / "reports/hillslope_semantic_summary.md"
    headers = [
        "Column",
        "Hillslope Fail Count",
        "Total Fail Count",
        "Mean Abs Diff Mean",
        "Max Abs Diff",
    ]
    rows = []
    for column in sorted(selected):
        metric = selected[column]
        rows.append(
            [
                column,
                metric.get("hillslope_fail_count"),
                metric.get("total_fail_count"),
                rounded(float(metric.get("mean_abs_diff_mean", 0.0))),
                rounded(float(metric.get("max_abs_diff", 0.0))),
            ]
        )
    text = "# HPHYS0299 Full-39 Suite Metrics\n\n"
    text += "Ran:\n\n"
    text += f"- Run root: `{run_root}`\n"
    text += f"- Summary JSON: `{summary_json}`\n"
    text += f"- Summary Markdown: `{summary_md}`\n"
    text += f"- Candidate HEAD: `{current_git_head()}`\n"
    text += "- Suite scope: H1..H39 hillslope semantic water-balance comparison.\n\n"
    text += HPHYS0265.markdown_table(headers, rows)
    text += "\n"
    (artifact_dir / "full-39-suite-metrics.md").write_text(text, encoding="utf-8")


def write_baseline_identity_artifact(
    run_root: Path, artifact_dir: Path, identity: dict[int, dict[str, Any]]
) -> None:
    write_json(artifact_dir / "baseline-observe-identity.json", identity)
    headers = [
        "Hill",
        "Pass",
        "Release=Off",
        "Off=On",
        "Partition Identity",
        "Records",
        "Release SHA",
        "Off SHA",
        "On SHA",
    ]
    rows = []
    for hill in TARGET_HILLS:
        identity_row = identity[hill]
        rows.append(
            [
                f"H{hill}",
                identity_row["pass"],
                identity_row["release_to_observe_off_bit_identical"],
                identity_row["observe_off_to_observe_on_bit_identical"],
                identity_row["release_matches_stored_partition"].get("pass"),
                identity_row["h298_record_count"],
                str(identity_row["release_sha256"])[:12],
                str(identity_row["observe_off_sha256"])[:12],
                str(identity_row["observe_on_sha256"])[:12],
            ]
        )
    text = "# HPHYS0299 Baseline Observe Identity\n\n"
    text += "Ran:\n\n"
    text += f"- Run root: `{run_root}`\n"
    text += f"- Release binary: `{BASELINE_RELEASE_BIN}`\n"
    text += f"- Observe binary: `{BASELINE_OBSERVE_BIN}`\n"
    text += f"- Baseline commit: `{BASELINE_COMMIT}`\n"
    text += "- Lanes: pinned release without observe, instrumented observe-off, instrumented observe-on.\n\n"
    text += HPHYS0265.markdown_table(headers, rows)
    text += "\n"
    (artifact_dir / "baseline-observe-identity.md").write_text(text, encoding="utf-8")


def run_targeted_traces(run_root: Path, trace_max_days: int) -> int:
    reports = run_root / "reports"
    logs = run_root / "logs" / "targeted_traces"
    output = run_root / "hillslope_output"
    runs_dir = HPHYS0265.copy_runfiles(run_root)
    status_rows: list[dict[str, Any]] = []
    for hill in TARGET_HILLS:
        trace_path = output / f"H{hill}.hphys0299.trace.jsonl"
        result = HPHYS0265.run_command(
            f"H{hill}_hphys0299_trace",
            [
                str(HPHYS0291.HILL_BIN),
                "--run-dir",
                str(runs_dir),
                "--run-file",
                f"p{hill}_openwepp.run",
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
                "hillslope_id": hill,
                "rc": result.rc,
                "seconds": f"{result.seconds:.3f}",
                "trace_path": trace_path,
                "stdout": result.stdout,
                "stderr": result.stderr,
            }
        )
        if result.rc != 0:
            HPHYS0265.write_status(reports / "hphys0299_target_trace_status.tsv", status_rows)
            return int(result.rc)
    HPHYS0265.write_status(reports / "hphys0299_target_trace_status.tsv", status_rows)
    return 0


def provenance_row(
    canonical_symbol: str,
    openwepp_symbol: str,
    unit: str,
    baseline_value_mm: float | None,
    openwepp_value_mm: float | None,
) -> dict[str, Any]:
    return {
        "canonical_symbol": canonical_symbol,
        "openwepp_symbol": openwepp_symbol,
        "unit": unit,
        "baseline_value_mm": rounded(baseline_value_mm),
        "openwepp_value_mm": rounded(openwepp_value_mm),
        "delta_mm": rounded(
            None
            if baseline_value_mm is None or openwepp_value_mm is None
            else baseline_value_mm - openwepp_value_mm
        ),
        "baseline_source_path": BASELINE_SOURCES.get(canonical_symbol, BASELINE_SOURCE),
        "openwepp_source_path": OPENWEPP_SOURCES.get(
            openwepp_symbol, "crates/openwepp-runner/src/hillslope/mod.rs"
        ),
    }


def source_provenance_for(partition_row: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        provenance_row(
            "hrrain",
            "snow_hourly_rain_sum_m",
            "mm",
            partition_row["baseline_raw_rain_sum_mm"],
            partition_row["openwepp_raw_rain_sum_mm"],
        ),
        provenance_row(
            "hrsnow",
            canonical_hrsnow_openwepp_field,
            "mm depth",
            partition_row["baseline_raw_snow_sum_mm"],
            partition_row["openwepp_raw_snow_sum_mm"],
        ),
        provenance_row(
            "hrmlt",
            "snow_hourly_melt_raw_m",
            "mm",
            partition_row["baseline_raw_melt_sum_mm"],
            partition_row["openwepp_raw_melt_sum_mm"],
        ),
        provenance_row(
            "wmelt",
            "snow_routed_melt_m",
            "mm",
            partition_row["baseline_post_wmelt_sum_mm"],
            partition_row["openwepp_routed_melt_sum_mm"],
        ),
        provenance_row(
            "rain",
            "snow_post_winter_rain_m",
            "mm",
            partition_row["baseline_raw_rain_sum_mm"],
            partition_row["openwepp_post_winter_rain_sum_mm"],
        ),
        provenance_row(
            "RM",
            "wb13_rm_mm",
            "mm",
            partition_row["baseline_wb_rm_observe_sum_mm"],
            partition_row["openwepp_wb13_rm_sum_mm"],
        ),
        provenance_row(
            "Q",
            "wb13_q_mm",
            "mm",
            partition_row["baseline_wb_q_observe_sum_mm"],
            partition_row["openwepp_wb13_q_sum_mm"],
        ),
    ]


def next_action_for(partition_row: dict[str, Any]) -> str:
    cut_point = partition_row["first_divergent_cut_point"]
    if cut_point == "hourly-forcing":
        return "Open producer migration only after this corrected depth-vs-depth ledger proves remaining hrrain/hrsnow divergence."
    if cut_point == "raw-hourly-melt":
        return "Open follow-on package to migrate or diagnose raw hourly melt after corrected precipitation-depth forcing is proven closed."
    if cut_point == "negative-melt-correction":
        if (
            partition_row.get("verdict") == "LEGACY-DEFECTIVE"
            and partition_row.get("baseline_negative_raw_melt_sum_mm", 0.0) < -WINDOW_TOLERANCE_MM
        ):
            return "Retain corrected negative-melt authority; do not reproduce pinned-baseline negative-melt bugs."
        return "Open follow-on package for post-raw routed-melt/negative-melt handling; this row is not accepted as corrected negative-melt legacy-defective authority."
    if cut_point == "trace-gap":
        return "Repair paired trace completeness before any production correction."
    return "Keep residual in HOLD and open a focused follow-on package for the corrected first divergent cut-point."


def compute_window_partition(
    hill: int,
    merged: Any,
    trace_index: dict[tuple[int, int], dict[str, Any]],
    observe: dict[str, Any],
    identity: dict[str, Any],
    window: tuple[str, int, int, int],
) -> dict[str, Any]:
    name, year, start, end = window
    rows = merged[
        (merged["_comparison_year"] == year)
        & (merged["julian"] >= start)
        & (merged["julian"] <= end)
    ].sort_values(["_comparison_year", "julian"])
    result: dict[str, Any] = {
        "hillslope_id": hill,
        "window": name,
        "year": year,
        "start_julian": start,
        "end_julian": end,
        "row_count": int(len(rows)),
        "baseline_source": BASELINE_SOURCE,
        "baseline_commit": BASELINE_COMMIT,
        "canonical_hrsnow_openwepp_field": canonical_hrsnow_openwepp_field,
        "canonical_hrsnow_provenance": CANONICAL_HRSNOW_PROVENANCE,
        "baseline_observe_identity_pass": bool(identity.get("pass")),
        "baseline_observe_identity": {
            "wat_bit_identical": identity.get("wat_bit_identical"),
            "target_window_semantic_identity": identity.get("target_window_semantic_identity"),
            "release_matches_stored_partition": identity.get("release_matches_stored_partition"),
        },
        "baseline_observe_missing_day_count": 0,
        "openwepp_trace_missing_day_count": 0,
        "openwepp_trace_missing_field_count": 0,
        "openwepp_trace_missing_fields": [],
        "baseline_gate_day_count": 0,
        "baseline_raw_melt_sum_mm": 0.0,
        "baseline_positive_raw_melt_sum_mm": 0.0,
        "baseline_negative_raw_melt_sum_mm": 0.0,
        "baseline_raw_rain_sum_mm": 0.0,
        "baseline_raw_snow_sum_mm": 0.0,
        "baseline_post_wmelt_sum_mm": 0.0,
        "baseline_wb_rm_observe_sum_mm": 0.0,
        "baseline_wb_q_observe_sum_mm": 0.0,
        "openwepp_raw_melt_sum_mm": 0.0,
        "openwepp_positive_raw_melt_sum_mm": 0.0,
        "openwepp_negative_raw_melt_sum_mm": 0.0,
        "openwepp_raw_rain_sum_mm": 0.0,
        "openwepp_raw_snow_sum_mm": 0.0,
        "openwepp_routed_melt_sum_mm": 0.0,
        "openwepp_post_winter_rain_sum_mm": 0.0,
        "openwepp_wb13_rm_sum_mm": 0.0,
        "openwepp_wb13_q_sum_mm": 0.0,
        "openwepp_wb13_rm_identity_abs_sum_mm": 0.0,
        "baseline_wat_rm_sum_mm": 0.0,
        "candidate_wat_rm_sum_mm": 0.0,
        "baseline_wat_q_sum_mm": 0.0,
        "candidate_wat_q_sum_mm": 0.0,
        "baseline_wat_snow_sum_mm": 0.0,
        "candidate_wat_snow_sum_mm": 0.0,
        "candidate_total_soil_sum_mm": 0.0,
        "baseline_total_soil_sum_mm": 0.0,
    }
    for _, wat_row in rows.iterrows():
        julian = int(wat_row["julian"])
        day_key = (year, julian)
        trace = trace_index.get(day_key)
        if trace is None:
            result["openwepp_trace_missing_day_count"] += 1
        else:
            trace_issues = validate_trace_fields(trace, year, julian)
            if trace_issues:
                result["openwepp_trace_missing_field_count"] += len(trace_issues)
                result["openwepp_trace_missing_fields"].extend(trace_issues)
                trace = None
        gate_records = HPHYS0298.obs_records(observe, year, julian, "H298_GATE_A")
        wb_records = HPHYS0298.obs_records(observe, year, julian, "H298_WBH_C")
        if gate_records:
            result["baseline_gate_day_count"] += 1
        if not wb_records:
            result["baseline_observe_missing_day_count"] += 1

        raw_values = [
            float(record["v1"]) * 1_000.0
            for record in HPHYS0298.obs_records(observe, year, julian, "H298_RAW_A")
        ]
        raw_rain = [
            float(record["v2"]) * 1_000.0
            for record in HPHYS0298.obs_records(observe, year, julian, "H298_RAW_A")
        ]
        raw_snow = [
            float(record["v1"]) * 1_000.0
            for record in HPHYS0298.obs_records(observe, year, julian, "H298_RAW_B")
        ]
        result["baseline_raw_melt_sum_mm"] += sum(raw_values)
        result["baseline_positive_raw_melt_sum_mm"] += sum(value for value in raw_values if value > 0.0)
        result["baseline_negative_raw_melt_sum_mm"] += sum(value for value in raw_values if value < 0.0)
        result["baseline_raw_rain_sum_mm"] += sum(raw_rain)
        result["baseline_raw_snow_sum_mm"] += sum(raw_snow)
        result["baseline_post_wmelt_sum_mm"] += (
            HPHYS0298.sum_obs(observe, year, julian, "H298_POST_A", "v1") * 1_000.0
        )
        result["baseline_wb_rm_observe_sum_mm"] += HPHYS0298.sum_obs(
            observe, year, julian, "H298_WBH_C", "v1"
        )
        result["baseline_wb_q_observe_sum_mm"] += HPHYS0298.sum_obs(
            observe, year, julian, "H298_WBH_C", "v2"
        )

        open_raw = trace_map_values_mm(trace, "snow_hourly_melt_raw_m")
        result["openwepp_raw_melt_sum_mm"] += sum(open_raw)
        result["openwepp_positive_raw_melt_sum_mm"] += sum(value for value in open_raw if value > 0.0)
        result["openwepp_negative_raw_melt_sum_mm"] += sum(value for value in open_raw if value < 0.0)
        result["openwepp_raw_rain_sum_mm"] += trace_m_to_mm(trace, "snow_hourly_rain_sum_m")
        result["openwepp_raw_snow_sum_mm"] += trace_m_to_mm(trace, "snow_hourly_snowfall_depth_sum_m")
        routed_melt = trace_m_to_mm(trace, "snow_routed_melt_m")
        post_rain = trace_m_to_mm(trace, "snow_post_winter_rain_m")
        open_rm = trace_mm(trace, "wb13_rm_mm")
        result["openwepp_routed_melt_sum_mm"] += routed_melt
        result["openwepp_post_winter_rain_sum_mm"] += post_rain
        result["openwepp_wb13_rm_sum_mm"] += open_rm
        result["openwepp_wb13_q_sum_mm"] += trace_mm(trace, "wb13_q_mm")
        result["openwepp_wb13_rm_identity_abs_sum_mm"] += abs(open_rm - (routed_melt + post_rain))

        result["baseline_wat_rm_sum_mm"] += HPHYS0298.wat_baseline(wat_row, "RM")
        result["candidate_wat_rm_sum_mm"] += HPHYS0298.wat_candidate(wat_row, "RM")
        result["baseline_wat_q_sum_mm"] += HPHYS0298.wat_baseline(wat_row, "Q")
        result["candidate_wat_q_sum_mm"] += HPHYS0298.wat_candidate(wat_row, "Q")
        result["baseline_wat_snow_sum_mm"] += HPHYS0298.wat_baseline(wat_row, "Snow-Water")
        result["candidate_wat_snow_sum_mm"] += HPHYS0298.wat_candidate(wat_row, "Snow-Water")
        result["candidate_total_soil_sum_mm"] += HPHYS0298.wat_candidate(wat_row, "Total-Soil")
        result["baseline_total_soil_sum_mm"] += HPHYS0298.wat_baseline(wat_row, "Total-Soil")

    result["baseline_wb_rm_observe_minus_wat_mm"] = (
        result["baseline_wb_rm_observe_sum_mm"] - result["baseline_wat_rm_sum_mm"]
    )
    result["baseline_raw_melt_minus_openwepp_raw_melt_mm"] = (
        result["baseline_raw_melt_sum_mm"] - result["openwepp_raw_melt_sum_mm"]
    )
    result["baseline_raw_rain_minus_openwepp_raw_rain_mm"] = (
        result["baseline_raw_rain_sum_mm"] - result["openwepp_raw_rain_sum_mm"]
    )
    result["baseline_raw_snow_minus_openwepp_raw_snow_mm"] = (
        result["baseline_raw_snow_sum_mm"] - result["openwepp_raw_snow_sum_mm"]
    )
    result["baseline_post_wmelt_minus_openwepp_routed_melt_mm"] = (
        result["baseline_post_wmelt_sum_mm"] - result["openwepp_routed_melt_sum_mm"]
    )
    result["baseline_wb_rm_observe_minus_openwepp_wb13_rm_mm"] = (
        result["baseline_wb_rm_observe_sum_mm"] - result["openwepp_wb13_rm_sum_mm"]
    )
    result["observed_baseline_minus_candidate_rm_mm"] = (
        result["baseline_wat_rm_sum_mm"] - result["candidate_wat_rm_sum_mm"]
    )
    result["observed_baseline_minus_candidate_q_mm"] = (
        result["baseline_wat_q_sum_mm"] - result["candidate_wat_q_sum_mm"]
    )
    result["observed_baseline_minus_candidate_snow_mm"] = (
        result["baseline_wat_snow_sum_mm"] - result["candidate_wat_snow_sum_mm"]
    )
    result["observed_baseline_minus_candidate_total_soil_mm"] = (
        result["baseline_total_soil_sum_mm"] - result["candidate_total_soil_sum_mm"]
    )
    cut_point, verdict, reason = HPHYS0298.first_divergence_for(result)
    result["first_divergent_cut_point"] = cut_point
    result["verdict"] = verdict
    result["verdict_reason"] = reason
    result["first_divergent_symbols"] = HPHYS0298.first_divergent_symbols(result)
    result["source_provenance"] = source_provenance_for(result)
    result["next_action"] = next_action_for(result)
    result["prohibited_compensation_note"] = (
        "Closed Q/WB13 identity only excludes runoff/storage compensation as first source; "
        "it is not acceptance authority for WB17/WB18/WB19 residuals."
    )
    return result


def write_corrected_partition_ledger(
    run_root: Path,
    artifact_dir: Path,
    identity: dict[int, dict[str, Any]],
    observes: dict[int, dict[str, Any]],
) -> list[dict[str, Any]]:
    ledger: list[dict[str, Any]] = []
    for hill in TARGET_HILLS:
        merged = HPHYS0295.merged_wat_rows(run_root, hill)
        trace_index = HPHYS0298.load_post_wb13_trace_index(
            run_root / f"hillslope_output/H{hill}.hphys0299.trace.jsonl",
            HPHYS0298.target_keys_for_hill(hill),
        )
        for window in TARGET_WINDOWS[hill]:
            ledger.append(compute_window_partition(hill, merged, trace_index, observes[hill], identity[hill], window))
    write_json(run_root / "reports/hphys0299_corrected_partition_ledger.json", ledger)
    write_json(artifact_dir / "corrected-partition-ledger.json", ledger)
    write_corrected_partition_summary(run_root, artifact_dir, ledger, identity)
    write_unit_provenance_audit(artifact_dir, ledger)
    return ledger


def write_corrected_partition_summary(
    run_root: Path,
    artifact_dir: Path,
    ledger: list[dict[str, Any]],
    identity: dict[int, dict[str, Any]],
) -> None:
    verdict_counts = Counter(row["verdict"] for row in ledger)
    cut_counts = Counter(row["first_divergent_cut_point"] for row in ledger)
    headers = [
        "Hill",
        "Window",
        "Days",
        "Verdict",
        "First Cut-Point",
        "First Symbols",
        "Baseline RM",
        "Candidate RM",
        "Baseline-Open RM",
        "Raw Snow Depth Δ",
        "Raw Melt Δ",
        "Routed Melt Δ",
        "Q Δ",
        "Total-Soil Δ",
    ]
    table_rows = [
        [
            f"H{row['hillslope_id']}",
            row["window"],
            f"{row['year']} {row['start_julian']}-{row['end_julian']}",
            row["verdict"],
            row["first_divergent_cut_point"],
            ",".join(row["first_divergent_symbols"]),
            rounded(row["baseline_wat_rm_sum_mm"]),
            rounded(row["candidate_wat_rm_sum_mm"]),
            rounded(row["observed_baseline_minus_candidate_rm_mm"]),
            rounded(row["baseline_raw_snow_minus_openwepp_raw_snow_mm"]),
            rounded(row["baseline_raw_melt_minus_openwepp_raw_melt_mm"]),
            rounded(row["baseline_post_wmelt_minus_openwepp_routed_melt_mm"]),
            rounded(row["observed_baseline_minus_candidate_q_mm"]),
            rounded(row["observed_baseline_minus_candidate_total_soil_mm"]),
        ]
        for row in ledger
    ]
    text = "# HPHYS0299 Corrected Hourly Snow Partition Ledger\n\n"
    text += "Ran:\n\n"
    text += f"- Run root: `{run_root}`\n"
    text += f"- Baseline source: `{BASELINE_SOURCE}` at `{BASELINE_COMMIT}`\n"
    text += f"- Baseline observe worktree: `{BASELINE_OBSERVE_WORKTREE}`\n"
    text += f"- Candidate HEAD: `{current_git_head()}`\n"
    text += "- Canonical `hrsnow` comparison: baseline `stmtim.for` snow depth vs openWEPP `snow_hourly_snowfall_depth_sum_m`.\n"
    text += "- Rejected HPHYS0298 seam: `snow_hourly_snowfall_water_equiv_sum_m` is water equivalent, not canonical `hrsnow` depth.\n\n"
    text += "## Baseline Observe Identity\n\n"
    for hill in TARGET_HILLS:
        identity_row = identity[hill]
        text += (
            f"- H{hill}: pass=`{identity_row['pass']}`, bit-identical=`{identity_row['wat_bit_identical']}`, "
            f"H298 records=`{identity_row['h298_record_count']}`\n"
        )
    text += "\n## Verdict Counts\n\n"
    for verdict, count in sorted(verdict_counts.items()):
        text += f"- `{verdict}`: `{count}` windows\n"
    text += "\n## First Cut-Point Counts\n\n"
    for cut_point, count in sorted(cut_counts.items()):
        text += f"- `{cut_point}`: `{count}` windows\n"
    text += "\n## Ledger\n\n"
    text += HPHYS0265.markdown_table(headers, table_rows)
    text += "\n\n## Interpretation\n\n"
    text += (
        "- If `hourly-forcing` remains after this corrected run, the residual is a true "
        "depth-vs-depth precipitation-partition difference and can authorize a focused producer package.\n"
    )
    text += (
        "- If `hourly-forcing` closes, HPHYS0298's production-migration recommendation was a "
        "diagnostic unit/provenance defect and must not drive production code edits.\n"
    )
    text += (
        "- Downstream WB17/WB18/WB19/WB13 compensation remains prohibited in both branches.\n"
    )
    (run_root / "reports/hphys0299_corrected_partition_ledger.md").write_text(text, encoding="utf-8")
    (artifact_dir / "corrected-partition-ledger.md").write_text(text, encoding="utf-8")


def write_unit_provenance_audit(artifact_dir: Path, ledger: list[dict[str, Any]]) -> None:
    bad_rows = [
        row
        for row in ledger
        for provenance in row["source_provenance"]
        if provenance["canonical_symbol"] == "hrsnow"
        and provenance["openwepp_symbol"] != canonical_hrsnow_openwepp_field
    ]
    text = "# Unit Provenance Audit\n\n"
    text += "Ran:\n\n"
    text += "- Checked every HPHYS0299 ledger row for canonical `hrsnow` source mapping.\n"
    text += f"- Required openWEPP field: `{canonical_hrsnow_openwepp_field}`.\n"
    text += "- Rejected field for canonical parity: `snow_hourly_snowfall_water_equiv_sum_m`.\n\n"
    text += "## Result\n\n"
    if bad_rows:
        text += f"- Status: `FAIL`; bad row count `{len(bad_rows)}`.\n"
    else:
        text += "- Status: `PASS`; all `hrsnow` provenance rows use snowfall depth.\n"
    text += "\n## Provenance\n\n"
    text += f"- Baseline partition call: `{CANONICAL_HRSNOW_PROVENANCE['baseline_partition_path']}`\n"
    text += f"- Baseline `stmtim` equation: `{CANONICAL_HRSNOW_PROVENANCE['baseline_stmtim_path']}`\n"
    text += f"- Equation: `{CANONICAL_HRSNOW_PROVENANCE['baseline_equation']}`\n"
    text += f"- Comparison: `{CANONICAL_HRSNOW_PROVENANCE['comparison']}`\n"
    (artifact_dir / "unit-provenance-audit.md").write_text(text, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-root", type=Path, required=True)
    parser.add_argument("--artifact-dir", type=Path, default=ARTIFACT_DIR)
    parser.add_argument("--baseline-release-bin", type=Path, default=BASELINE_RELEASE_BIN)
    parser.add_argument("--baseline-observe-bin", type=Path, default=BASELINE_OBSERVE_BIN)
    parser.add_argument("--trace-max-days", type=int, default=1_800)
    parser.add_argument("--skip-full-suite", action="store_true")
    parser.add_argument("--skip-targeted-traces", action="store_true")
    parser.add_argument("--skip-baseline-observe", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    args.run_root.mkdir(parents=True, exist_ok=True)
    args.artifact_dir.mkdir(parents=True, exist_ok=True)
    if not args.skip_full_suite:
        full_rc = HPHYS0291.run_full_hillslope_suite(args.run_root)
        if full_rc != 0:
            return int(full_rc)
        normalize_full_suite_summary_label(args.run_root)
        selected = write_selected_metrics(args.run_root)
        write_full39_metrics_artifact(args.run_root, args.artifact_dir, selected)
    elif (args.run_root / "reports/hillslope_semantic_summary.json").exists():
        selected = write_selected_metrics(args.run_root)
        write_full39_metrics_artifact(args.run_root, args.artifact_dir, selected)

    if not args.skip_targeted_traces:
        trace_rc = run_targeted_traces(args.run_root, args.trace_max_days)
        if trace_rc != 0:
            return int(trace_rc)

    if args.skip_baseline_observe:
        identity = read_json(args.run_root / "reports/hphys0298_baseline_observe_identity.json")
        observes = {
            int(hill): HPHYS0298.parse_h298_observe_log(Path(payload["observe_log"]))
            for hill, payload in identity.items()
        }
        identity = {int(hill): payload for hill, payload in identity.items()}
    else:
        identity, observes = HPHYS0298.run_baseline_observe_identity(
            args.run_root, args.baseline_release_bin, args.baseline_observe_bin
        )
        write_baseline_identity_artifact(args.run_root, args.artifact_dir, identity)

    write_corrected_partition_ledger(args.run_root, args.artifact_dir, identity, observes)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
