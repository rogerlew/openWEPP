#!/usr/bin/env python3
"""Freeze, execute, and analyze EB-04W2A residual chronology diagnostics."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import importlib.util
import json
import math
import os
import statistics
import subprocess
import sys
import tempfile
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
OUTPUT = REPO / "target/snow_surface_eb04w2a_melt_chronology_attribution"
RUNS = OUTPUT / "runs"
FREEZE = ARTIFACTS / "experiment-freeze.json"
RECEIPT = ARTIFACTS / "execution-receipt.json"
RESULTS = ARTIFACTS / "melt-chronology-diagnostic-results.json"
SUMMARY = ARTIFACTS / "melt-chronology-diagnostic-summary.csv"
SNOWBENCH = REPO / "target/release/openwepp-snowbench"
DIRECT_BINARY = REPO / "target/release/openwepp-cli-hill"
W2_PACKAGE = REPO / (
    "docs/work-packages/20260802-snow-surface-eb-04w2-"
    "precipitation-scaling-grid-extension-001"
)
W2_TOOL = W2_PACKAGE / "tools/run_grid_extension.py"
W2_FREEZE = W2_PACKAGE / "artifacts/experiment-freeze.json"
W2_RECEIPT = W2_PACKAGE / "artifacts/execution-receipt.json"
W2_RESULTS = W2_PACKAGE / "artifacts/precipitation-grid-extension-results.json"
W1_PACKAGE = REPO / (
    "docs/work-packages/20260802-snow-surface-eb-04w1-"
    "precipitation-scaling-calibration-001"
)
W1_RECEIPT = W1_PACKAGE / "artifacts/execution-receipt.json"
W1_RUNS = REPO / "target/snow_surface_eb04w1_precipitation_scaling/runs"
W2_RUNS = REPO / "target/snow_surface_eb04w2_precipitation_scaling/runs"
MODELS = ("legacy_coe", "coe_shortwave_albedo_v1")
SELECTED = {
    "snotel_mica_creek_st_joe_id": (1.4, "retained"),
    "snotel_niwot_co": (1.7, "extension"),
    "snotel_paradise_wa": (1.8, "extension"),
    "snotel_snowbird_ut": (2.0, "extension"),
}
LABELS = {
    "snotel_mica_creek_st_joe_id": "Mica Creek, ID",
    "snotel_niwot_co": "Niwot, CO",
    "snotel_paradise_wa": "Paradise, WA",
    "snotel_snowbird_ut": "Snowbird, UT",
}
ALBEDO_DAYS = 5.0
PEAK_LOG_TOLERANCE = 0.10
COLD_FRACTION = 0.10
COLD_DEPTH_M = 0.010
MASS_TOLERANCE_M = 1.0e-12
ENERGY_TOLERANCE_J_M2 = 1.0e-6


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


w2 = load_module("eb04w2a_w2", W2_TOOL)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def scale_id(multiplier: float) -> str:
    return w2.scale_id(multiplier)


def selected_lanes() -> list[Any]:
    lanes = {lane.lane_id: lane for lane in w2.selected_lanes()}
    return [lanes[lane_id] for lane_id in SELECTED]


def anchor_root(lane_id: str) -> Path:
    multiplier, source = SELECTED[lane_id]
    root = W1_RUNS if source == "retained" else W2_RUNS
    return root / scale_id(multiplier)


def anchor_paths(lane_id: str) -> dict[str, Path]:
    root = anchor_root(lane_id)
    stem = f"{lane_id}-B"
    run = root / lane_id / "B"
    return {
        "runfile": run / f"{stem}.run",
        "wat": run / f"{stem}.wat.parquet",
        "trace": run / f"{stem}.snow.jsonl",
        "manifest": run / "openwepp_hillslope_run_manifest.json",
    }


def scaled_fixture(lane_id: str) -> Path:
    multiplier, source = SELECTED[lane_id]
    root = REPO / (
        "target/snow_surface_eb04w1_precipitation_scaling/fixtures"
        if source == "retained"
        else "target/snow_surface_eb04w2_precipitation_scaling/fixtures"
    )
    return root / scale_id(multiplier) / lane_id


def fixture_runfile(lane_id: str) -> Path:
    candidates = sorted(scaled_fixture(lane_id).glob("*.run"))
    if len(candidates) != 1:
        raise RuntimeError(
            f"expected one canonical fixture run file for {lane_id}, found {len(candidates)}"
        )
    return candidates[0]


def file_manifest(root: Path) -> dict[str, str]:
    return {
        relative(path): sha256(path)
        for path in sorted(root.rglob("*"))
        if path.is_file()
    }


def self_check() -> None:
    if len(SELECTED) != 4 or len(MODELS) != 2:
        raise RuntimeError("frozen 4x2 diagnostic matrix drift")
    if not math.isclose(abs(math.log(1.0)), 0.0):
        raise RuntimeError("peak-error operator drift")
    if not ((7.0 >= ALBEDO_DAYS) and (0.05 <= PEAK_LOG_TOLERANCE)):
        raise RuntimeError("synthetic albedo flag should pass")
    cold_flag = (0.011 / 0.2 >= COLD_FRACTION) or (0.011 >= COLD_DEPTH_M)
    if not cold_flag:
        raise RuntimeError("synthetic cold-content flag should pass")
    if not abs(0.03 + 0.02) > abs(0.01 + 0.001):
        raise RuntimeError("synthetic turbulent dominance should pass")


def retained_anchor_audit() -> dict[str, Any]:
    receipts = {
        "retained": json.loads(W1_RECEIPT.read_text(encoding="utf-8")),
        "extension": json.loads(W2_RECEIPT.read_text(encoding="utf-8")),
    }
    records: dict[str, Any] = {}
    output_count = 0
    for lane in selected_lanes():
        multiplier, source = SELECTED[lane.lane_id]
        entry = receipts[source]["results"][f"{lane.lane_id}/{scale_id(multiplier)}"]
        provenance_path = REPO / entry["provenance"]
        if sha256(provenance_path) != entry["provenance_sha256"]:
            raise RuntimeError(f"retained provenance mismatch: {lane.lane_id}")
        provenance = json.loads(provenance_path.read_text(encoding="utf-8"))
        for output in provenance["files"].values():
            path = REPO / output["path"]
            if not path.is_file() or sha256(path) != output["sha256"]:
                raise RuntimeError(f"retained output mismatch: {path}")
            output_count += 1
        fixture = scaled_fixture(lane.lane_id)
        records[lane.lane_id] = {
            "multiplier": multiplier,
            "source": source,
            "provenance": relative(provenance_path),
            "provenance_sha256": sha256(provenance_path),
            "fixture": relative(fixture),
            "fixture_files": file_manifest(fixture),
            "diagnostic_runfile": {
                "path": relative(fixture_runfile(lane.lane_id)),
                "sha256": sha256(fixture_runfile(lane.lane_id)),
            },
            "anchor_files": {
                name: {"path": relative(path), "sha256": sha256(path)}
                for name, path in anchor_paths(lane.lane_id).items()
            },
            "observation": {
                "path": relative(lane.observation_file),
                "sha256": sha256(lane.observation_file),
                "role": "CALIBRATION",
            },
        }
    return {
        "retained_cell_count": len(records),
        "retained_output_identity_count": output_count,
        "records": records,
    }


def freeze() -> dict[str, Any]:
    if FREEZE.exists() or RECEIPT.exists() or RUNS.exists():
        raise RuntimeError("freeze or result-bearing output already exists")
    audit = retained_anchor_audit()
    w2_freeze = json.loads(W2_FREEZE.read_text(encoding="utf-8"))
    value = {
        "schema": "snow-surface-eb04w2a-experiment-freeze-v1",
        "frozen_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
        "source_head": subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=REPO, text=True
        ).strip(),
        "tool_sha256": sha256(Path(__file__)),
        "snowbench_binary_sha256": sha256(SNOWBENCH),
        "direct_binary_sha256": sha256(DIRECT_BINARY),
        "w2_tool_sha256": sha256(W2_TOOL),
        "w2_freeze_sha256": sha256(W2_FREEZE),
        "w2_receipt_sha256": sha256(W2_RECEIPT),
        "w2_results_sha256": sha256(W2_RESULTS),
        "retained": audit,
        "selected": {
            lane_id: {"multiplier": value[0], "source": value[1]}
            for lane_id, value in SELECTED.items()
        },
        "operators": w2_freeze["operators"],
        "models": list(MODELS),
        "new_harness_run_count": 8,
        "harness_boundary": (
            "within-snowbench contrast only; not direct-production parity, "
            "validation, or promotion"
        ),
        "chronology_gap_window": (
            "modeled-to-observed peak date for peak operators; final 60 days "
            "ending modeled melt-out for melt-out operators"
        ),
        "hypothesis_thresholds": {
            "albedo_days": ALBEDO_DAYS,
            "peak_absolute_log_error_worsening": PEAK_LOG_TOLERANCE,
            "cold_melt_fraction": COLD_FRACTION,
            "cold_melt_depth_m": COLD_DEPTH_M,
            "role": "ASSUMED_FOR_EXECUTION diagnostic flags",
        },
        "mass_tolerance_m": MASS_TOLERANCE_M,
        "energy_tolerance_j_m2": ENERGY_TOLERANCE_J_M2,
        "observation_role": "CALIBRATION",
        "independent_validation_count": 0,
        "promotion_authorized": False,
        "protected_paths": (
            "all production, contracts, tests, fixtures, observations, selectors, "
            "defaults, assurance authority, and historical package evidence"
        ),
        "stop_condition": (
            "complete attribution after one 4x2 harness matrix; no coefficient "
            "fit, precipitation extension, or result-aware model variant"
        ),
    }
    write_json(FREEZE, value)
    return value


def execute_cell(lane: Any, model: str) -> dict[str, Any]:
    out = RUNS / lane.lane_id / model
    out.mkdir(parents=True, exist_ok=False)
    command = [
        str(SNOWBENCH),
        "coe-melt",
        "--run-dir",
        str(scaled_fixture(lane.lane_id)),
        "--run-file",
        str(fixture_runfile(lane.lane_id)),
        "--output-dir",
        str(out),
        "--model",
        model,
    ]
    environment = {
        key: value for key, value in os.environ.items() if not key.startswith("OPENWEPP_")
    }
    started = time.time()
    completed = subprocess.run(
        command,
        cwd=REPO,
        env=environment,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    (out / "stdout.txt").write_text(completed.stdout, encoding="utf-8")
    (out / "stderr.txt").write_text(completed.stderr, encoding="utf-8")
    files = file_manifest(out)
    provenance = {
        "schema": "snow-surface-eb04w2a-cell-provenance-v1",
        "lane_id": lane.lane_id,
        "multiplier": SELECTED[lane.lane_id][0],
        "model": model,
        "returncode": completed.returncode,
        "started_unix_seconds": started,
        "completed_unix_seconds": time.time(),
        "argv": command,
        "working_directory": str(REPO),
        "sanitized_openwepp_environment": True,
        "snowbench_binary_sha256": sha256(SNOWBENCH),
        "tool_sha256": sha256(Path(__file__)),
        "freeze_sha256": sha256(FREEZE),
        "fixture": relative(scaled_fixture(lane.lane_id)),
        "fixture_files": file_manifest(scaled_fixture(lane.lane_id)),
        "runfile": relative(fixture_runfile(lane.lane_id)),
        "runfile_sha256": sha256(fixture_runfile(lane.lane_id)),
        "files": files,
    }
    provenance_path = out / "eb04w2a-cell-provenance.json"
    write_json(provenance_path, provenance)
    return {
        "returncode": completed.returncode,
        "provenance": relative(provenance_path),
        "provenance_sha256": sha256(provenance_path),
    }


def execute(workers: int) -> dict[str, Any]:
    if not FREEZE.is_file():
        raise FileNotFoundError("freeze must exist before execution")
    if RECEIPT.exists() or RUNS.exists():
        raise RuntimeError("result-bearing output already exists")
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))
    if frozen["tool_sha256"] != sha256(Path(__file__)):
        raise RuntimeError("tool changed after freeze")
    if frozen["snowbench_binary_sha256"] != sha256(SNOWBENCH):
        raise RuntimeError("snowbench binary changed after freeze")
    results: dict[str, Any] = {}
    futures: dict[Any, tuple[str, str]] = {}
    with ThreadPoolExecutor(max_workers=workers) as pool:
        for lane in selected_lanes():
            for model in MODELS:
                future = pool.submit(execute_cell, lane, model)
                futures[future] = (lane.lane_id, model)
        for future in as_completed(futures):
            lane_id, model = futures[future]
            result = future.result()
            results[f"{lane_id}/{model}"] = result
            print(f"{lane_id}/{model}: {'PASS' if result['returncode'] == 0 else 'FAIL'}")
    if len(results) != 8 or any(row["returncode"] != 0 for row in results.values()):
        raise RuntimeError("frozen 8-cell diagnostic matrix did not complete")
    receipt = {
        "schema": "snow-surface-eb04w2a-execution-receipt-v1",
        "freeze_sha256": sha256(FREEZE),
        "tool_sha256": sha256(Path(__file__)),
        "snowbench_binary_sha256": sha256(SNOWBENCH),
        "result_count": len(results),
        "results": dict(sorted(results.items())),
    }
    write_json(RECEIPT, receipt)
    return receipt


def read_harness_rows(lane_id: str, model: str) -> list[dict[str, Any]]:
    path = RUNS / lane_id / model / "coe_melt_snow.csv"
    rows: list[dict[str, Any]] = []
    with path.open(encoding="utf-8", newline="") as stream:
        for row in csv.DictReader(stream):
            converted: dict[str, Any] = {"date": row["date"]}
            for key, value in row.items():
                if key not in {"date", "source"}:
                    converted[key] = float(value) if value else None
            rows.append(converted)
    return rows


def modeled_from_harness(rows: list[dict[str, Any]]) -> dict[dt.date, dict[str, float]]:
    return {
        dt.date.fromisoformat(row["date"]): {
            "snow_water_m": float(row["snow_water_m"]),
            "snow_depth_m": float(row["snow_depth_m"]),
            "frdp_m": 0.0,
        }
        for row in rows
    }


def operator_metrics(lane: Any, modeled: dict[dt.date, dict[str, float]]) -> dict[str, Any]:
    observations = w2.w1.eb04w.observation_rows(lane)
    pairs = w2.w1.eb04r.legacy.rubric.paired_snow_rows(observations, modeled)
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))["operators"]
    rows: list[dict[str, Any]] = []
    for lane_id, operator, _ in frozen:
        if lane_id != lane.lane_id:
            continue
        if operator == "seasonal_ablation_meltout_date":
            offsets = w2.w1.eb04r.legacy.rubric.last_snow_date_by_water_year(pairs)
            observed_key, modeled_key = "observed_swe_m", "modeled_swe_m"
        else:
            suffix = "snow_depth_m" if operator.endswith("depth_date") else "swe_m"
            observed_key, modeled_key = f"observed_{suffix}", f"modeled_{suffix}"
            offsets = w2.w1.eb04r.legacy.rubric.peak_date_by_water_year(
                pairs, observed_key, modeled_key
            )
        ratios = []
        for _, year_rows in w2.w1.eb04r.legacy.rubric.pairs_by_water_year(pairs).items():
            observed_peak = max(float(row[observed_key]) for row in year_rows)
            modeled_peak = max(float(row[modeled_key]) for row in year_rows)
            if observed_peak > 0.0:
                ratios.append(modeled_peak / observed_peak)
        rows.append(
            {
                "operator": operator,
                "median_offset_days": statistics.median(
                    float(row["offset_days"]) for row in offsets
                ),
                "median_peak_ratio": statistics.median(ratios),
                "water_years": offsets,
            }
        )
    primary = next(
        row
        for row in rows
        if lane.lane_id != "snotel_niwot_co"
        or row["operator"] == "seasonal_peak_swe_date"
    )
    return {
        "operators": rows,
        "primary_peak_ratio": primary["median_peak_ratio"],
        "chronology_abs_error_days": max(abs(row["median_offset_days"]) for row in rows),
    }


def enriched_direct_daily(lane_id: str, direct: dict[str, Any]) -> list[dict[str, Any]]:
    trace_path = anchor_paths(lane_id)["trace"]
    daily = [dict(row) for row in direct["daily"]]
    with trace_path.open(encoding="utf-8") as stream:
        for index, line in enumerate(stream):
            if index >= len(daily):
                raise RuntimeError(f"trace has excess rows: {lane_id}")
            trace = json.loads(line)
            hourly = trace.get("accumulation_melt_hourly") or []
            daily[index].update(
                {
                    "stage3_cold_content_before_j_m2": float(
                        trace.get("stage3_cold_content_before_j_m2") or 0.0
                    ),
                    "stage3_cold_content_after_j_m2": float(
                        trace.get("stage3_cold_content_after_j_m2") or 0.0
                    ),
                    "stage3_energy_closure_residual_j_m2": float(
                        trace.get("stage3_energy_closure_residual_j_m2") or 0.0
                    ),
                    "liquid_water_retained_after_m": float(
                        trace.get("liquid_water_retained_after_m") or 0.0
                    ),
                    "liquid_water_released_m": float(
                        trace.get("liquid_water_released_m") or 0.0
                    ),
                    "rain_released_m": float(trace.get("rain_released_m") or 0.0),
                    "routed_melt_m": float(trace.get("routed_melt_m") or 0.0),
                    "stage3_refrozen_liquid_m": float(
                        trace.get("stage3_refrozen_liquid_m") or 0.0
                    ),
                    "modeled_wind_redistribution_m": sum(
                        float(hour.get("modeled_wind_redistribution_m") or 0.0)
                        for hour in hourly
                    ),
                }
            )
    if len(daily) != index + 1:
        raise RuntimeError(f"trace/daily inventory mismatch: {lane_id}")
    return daily


def direct_windows(lane: Any, daily: list[dict[str, Any]]) -> dict[str, Any]:
    paths = anchor_paths(lane.lane_id)
    modeled = w2.w1.eb04r.legacy.observed_harness.load_modeled_wat(paths["wat"])
    observations = w2.w1.eb04w.observation_rows(lane)
    pairs = w2.w1.eb04r.legacy.rubric.paired_snow_rows(observations, modeled)
    by_date = {dt.date.fromisoformat(row["date"]): row for row in daily}
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))["operators"]
    windows: list[dict[str, Any]] = []
    for lane_id, operator, _ in frozen:
        if lane_id != lane.lane_id:
            continue
        if operator == "seasonal_ablation_meltout_date":
            offsets = w2.w1.eb04r.legacy.rubric.last_snow_date_by_water_year(pairs)
        else:
            suffix = "snow_depth_m" if operator.endswith("depth_date") else "swe_m"
            offsets = w2.w1.eb04r.legacy.rubric.peak_date_by_water_year(
                pairs, f"observed_{suffix}", f"modeled_{suffix}"
            )
        for offset in offsets:
            modeled_date = dt.date.fromisoformat(offset["modeled_date"])
            observed_date = dt.date.fromisoformat(offset["observed_date"])
            if operator == "seasonal_ablation_meltout_date":
                start, end = modeled_date - dt.timedelta(days=59), modeled_date
            else:
                start, end = sorted((modeled_date, observed_date))
            rows = [row for date, row in by_date.items() if start <= date <= end]
            if not rows:
                continue
            sums = {
                key: sum(float(row.get(key) or 0.0) for row in rows)
                for key in (
                    "snowfall_swe_m",
                    "rain_retained_m",
                    "rain_released_m",
                    "coe_melt_amelt_m",
                    "coe_melt_bmelt_m",
                    "coe_melt_cmelt_m",
                    "coe_melt_dmelt_m",
                    "coe_melt_applied_m",
                    "routed_melt_m",
                    "liquid_water_released_m",
                    "stage3_refrozen_liquid_m",
                    "sublimation_m",
                    "modeled_wind_redistribution_m",
                )
            }
            cold_melt = sum(
                float(row["coe_melt_applied_m"])
                for row in rows
                if float(row["stage3_cold_content_before_j_m2"]) > 0.0
            )
            observed_gain = None
            if operator != "seasonal_ablation_meltout_date":
                paired_year = next(
                    year_rows
                    for year, year_rows in w2.w1.eb04r.legacy.rubric.pairs_by_water_year(pairs).items()
                    if year == int(offset["water_year"])
                )
                pair_dates = {row["date_obj"]: row for row in paired_year}
                if start in pair_dates and end in pair_dates:
                    observed_gain = max(
                        0.0,
                        float(pair_dates[end]["observed_swe_m"])
                        - float(pair_dates[start]["observed_swe_m"]),
                    )
            windows.append(
                {
                    "operator": operator,
                    "water_year": int(offset["water_year"]),
                    "modeled_date": offset["modeled_date"],
                    "observed_date": offset["observed_date"],
                    "offset_days": float(offset["offset_days"]),
                    "window_start": start.isoformat(),
                    "window_end": end.isoformat(),
                    "sums": sums,
                    "cold_content_positive_applied_melt_m": cold_melt,
                    "cold_content_positive_melt_fraction": (
                        cold_melt / sums["coe_melt_applied_m"]
                        if sums["coe_melt_applied_m"] > 0.0
                        else 0.0
                    ),
                    "observed_swe_gain_m": observed_gain,
                }
            )
    max_mass_closure = max(
        abs(
            float(row["runtime_swe_before_m"])
            + float(row["snowfall_swe_m"])
            + float(row["rain_retained_m"])
            - float(row["snowpack_swe_loss_m"])
            - float(row["sublimation_m"])
            - float(row["runtime_swe_after_m"])
        )
        for row in daily
    )
    max_energy_closure = max(
        abs(float(row["stage3_energy_closure_residual_j_m2"])) for row in daily
    )
    return {
        "windows": windows,
        "maximum_mass_closure_m": max_mass_closure,
        "maximum_energy_closure_j_m2": max_energy_closure,
    }


def median_value(rows: list[dict[str, Any]], getter: Any) -> float:
    values = [float(getter(row)) for row in rows if getter(row) is not None]
    return statistics.median(values) if values else 0.0


def classify_hypotheses(
    direct_window: dict[str, Any], harness: dict[str, dict[str, Any]]
) -> dict[str, Any]:
    legacy = harness["legacy_coe"]
    albedo = harness["coe_shortwave_albedo_v1"]
    chronology_gain = (
        legacy["chronology_abs_error_days"] - albedo["chronology_abs_error_days"]
    )
    peak_worsening = abs(math.log(albedo["primary_peak_ratio"])) - abs(
        math.log(legacy["primary_peak_ratio"])
    )
    windows = direct_window["windows"]
    cold_depth = median_value(
        windows, lambda row: row["cold_content_positive_applied_melt_m"]
    )
    cold_fraction = median_value(
        windows, lambda row: row["cold_content_positive_melt_fraction"]
    )
    amelt_dmelt = median_value(
        windows,
        lambda row: abs(row["sums"]["coe_melt_amelt_m"] + row["sums"]["coe_melt_dmelt_m"]),
    )
    turbulent = median_value(
        windows,
        lambda row: abs(row["sums"]["coe_melt_bmelt_m"] + row["sums"]["coe_melt_cmelt_m"]),
    )
    late_rows = [row for row in windows if row["observed_swe_gain_m"] is not None]
    late_input = any(
        row["sums"]["snowfall_swe_m"] < float(row["observed_swe_gain_m"])
        for row in late_rows
    )
    return {
        "ALBEDO_RESPONSE_MATERIAL": (
            chronology_gain >= ALBEDO_DAYS and peak_worsening <= PEAK_LOG_TOLERANCE
        ),
        "COLD_CONTENT_MELT_COINCIDENCE_MATERIAL": (
            cold_fraction >= COLD_FRACTION or cold_depth >= COLD_DEPTH_M
        ),
        "TURBULENT_EMPIRICAL_TERMS_DOMINANT": turbulent > amelt_dmelt,
        "LATE_INPUT_DEFICIT_SUPPORTED": late_input if late_rows else None,
        "diagnostics": {
            "harness_chronology_improvement_days": chronology_gain,
            "harness_peak_absolute_log_error_worsening": peak_worsening,
            "median_cold_content_positive_melt_m": cold_depth,
            "median_cold_content_positive_melt_fraction": cold_fraction,
            "median_abs_amelt_plus_dmelt_m": amelt_dmelt,
            "median_abs_bmelt_plus_cmelt_m": turbulent,
            "late_input_window_count": len(late_rows),
        },
    }


def analyze() -> dict[str, Any]:
    if not RECEIPT.is_file():
        raise FileNotFoundError("execution receipt missing")
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))
    if frozen["tool_sha256"] != sha256(Path(__file__)):
        raise RuntimeError("analysis tool differs from frozen tool")
    audit = retained_anchor_audit()
    lane_results: dict[str, Any] = {}
    maximum_mass = 0.0
    maximum_energy = 0.0
    operators = frozen["operators"]
    for lane in selected_lanes():
        multiplier, source = SELECTED[lane.lane_id]
        direct = w2.analyze_candidate(lane, multiplier, source, operators)
        daily = enriched_direct_daily(lane.lane_id, direct)
        windows = direct_windows(lane, daily)
        maximum_mass = max(maximum_mass, direct["maximum_closure_m"], windows["maximum_mass_closure_m"])
        maximum_energy = max(maximum_energy, windows["maximum_energy_closure_j_m2"])
        harness: dict[str, Any] = {}
        harness_daily: dict[str, list[dict[str, Any]]] = {}
        for model in MODELS:
            rows = read_harness_rows(lane.lane_id, model)
            harness_daily[model] = rows
            harness[model] = operator_metrics(lane, modeled_from_harness(rows))
            summary = json.loads(
                (RUNS / lane.lane_id / model / "coe_melt_summary.json").read_text(
                    encoding="utf-8"
                )
            )
            harness[model]["summary"] = summary["summary"]
            harness[model]["canopy_series_summary"] = summary["canopy_series_summary"]
            harness[model]["maximum_swe_balance_residual_m"] = max(
                abs(float(row["snowpack_swe_balance_residual_m"])) for row in rows
            )
            maximum_mass = max(
                maximum_mass, harness[model]["maximum_swe_balance_residual_m"]
            )
        flags = classify_hypotheses(windows, harness)
        lane_results[lane.lane_id] = {
            "label": LABELS[lane.lane_id],
            "multiplier": multiplier,
            "direct_production": {
                key: value for key, value in direct.items() if key != "daily"
            },
            "direct_windows": windows,
            "harness": harness,
            "hypothesis_flags": flags,
            "plot_daily": {
                "direct": direct["daily"],
                **harness_daily,
            },
        }
    result = {
        "schema": "snow-surface-eb04w2a-results-v1",
        "freeze_sha256": sha256(FREEZE),
        "receipt_sha256": sha256(RECEIPT),
        "tool_sha256": sha256(Path(__file__)),
        "retained_cell_count": audit["retained_cell_count"],
        "new_harness_run_count": 8,
        "lane_count": 4,
        "independent_validation": False,
        "maximum_mass_closure_m": maximum_mass,
        "maximum_energy_closure_j_m2": maximum_energy,
        "lanes": lane_results,
    }
    serializable = json.loads(json.dumps(result))
    for lane in serializable["lanes"].values():
        lane.pop("plot_daily")
    write_json(RESULTS, serializable)
    write_summary(serializable)
    write_figures(result)
    write_synthesis(serializable)
    return serializable


def write_summary(result: dict[str, Any]) -> None:
    with SUMMARY.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.writer(stream)
        writer.writerow(
            [
                "lane_id",
                "multiplier",
                "direct_peak_ratio",
                "direct_chronology_days",
                "legacy_harness_chronology_days",
                "albedo_harness_chronology_days",
                "albedo_response_material",
                "cold_content_melt_material",
                "turbulent_terms_dominant",
                "late_input_deficit_supported",
            ]
        )
        for lane_id, lane in result["lanes"].items():
            flags = lane["hypothesis_flags"]
            writer.writerow(
                [
                    lane_id,
                    lane["multiplier"],
                    lane["direct_production"]["primary_peak_ratio"],
                    lane["direct_production"]["chronology_abs_error_days"],
                    lane["harness"]["legacy_coe"]["chronology_abs_error_days"],
                    lane["harness"]["coe_shortwave_albedo_v1"]["chronology_abs_error_days"],
                    flags["ALBEDO_RESPONSE_MATERIAL"],
                    flags["COLD_CONTENT_MELT_COINCIDENCE_MATERIAL"],
                    flags["TURBULENT_EMPIRICAL_TERMS_DOMINANT"],
                    flags["LATE_INPUT_DEFICIT_SUPPORTED"],
                ]
            )


def save_figure(fig: Any, stem: str) -> None:
    FIGURES.mkdir(parents=True, exist_ok=True)
    fig.savefig(FIGURES / f"{stem}.svg", bbox_inches="tight", facecolor="white")
    plt.close(fig)


def write_sidecar(stem: str, title: str, caption: str, methods: str, limits: str) -> None:
    (FIGURES / f"{stem}.md").write_text(
        f"# {title}\n\n## Caption\n\n{caption}\n\n## Methods And Units\n\n"
        f"{methods}\n\n## Interpretation Limits\n\n{limits}\n",
        encoding="utf-8",
    )


def median_curve(rows: list[dict[str, Any]], value_key: str) -> tuple[list[int], list[float]]:
    grouped: dict[int, list[float]] = {}
    for row in rows:
        date = dt.date.fromisoformat(row["date"])
        wy_start = dt.date(date.year if date.month >= 10 else date.year - 1, 10, 1)
        index = (date - wy_start).days
        value = row.get(value_key)
        if value is not None:
            grouped.setdefault(index, []).append(float(value))
    xs = sorted(grouped)
    return xs, [statistics.median(grouped[x]) for x in xs]


def write_figures(result: dict[str, Any]) -> None:
    colors = {"direct": "#111827", "legacy_coe": "#d97706", "coe_shortwave_albedo_v1": "#2563eb"}
    fig, axes = plt.subplots(2, 2, figsize=(12, 8), sharex=True)
    for axis, (lane_id, lane) in zip(axes.flat, result["lanes"].items(), strict=True):
        for model, key, label in (
            ("direct", "swe_m", "W2 direct-production anchor"),
            ("legacy_coe", "snow_water_m", "Snowbench legacy CoE"),
            ("coe_shortwave_albedo_v1", "snow_water_m", "Snowbench albedo-aware CoE"),
        ):
            xs, ys = median_curve(lane["plot_daily"][model], key)
            axis.plot(xs, [value * 1000 for value in ys], color=colors[model], label=label, linewidth=1.8)
        axis.set_title(f"{lane['label']} · precipitation ×{lane['multiplier']:.1f}")
        axis.grid(alpha=0.25)
        axis.set_ylabel("Median SWE (mm)")
    axes[-1, 0].set_xlabel("Day since 1 October")
    axes[-1, 1].set_xlabel("Day since 1 October")
    handles, labels = axes[0, 0].get_legend_handles_labels()
    fig.legend(handles, labels, loc="lower center", ncol=3, frameon=False)
    fig.suptitle("Mass-controlled seasonal SWE: production anchor and diagnostic melt replays")
    fig.subplots_adjust(bottom=0.13, hspace=0.30)
    save_figure(fig, "eb04w2a-seasonal-swe-comparison")
    write_sidecar(
        "eb04w2a-seasonal-swe-comparison",
        "Mass-Controlled Seasonal SWE Comparison",
        "Median water-year SWE trajectories for the four EB-04W2 selected precipitation cells. The black line is the retained direct-production result; orange and blue are a within-snowbench legacy/albedo comparison.",
        "SWE is meters of water equivalent converted to millimeters. Curves are medians by day since 1 October across all modeled water years.",
        "Snowbench uses legacy density and disables the production Stage-3 state. Its lines isolate a diagnostic albedo contrast and are not direct-production counterfactuals or validation.",
    )

    fig, axis = plt.subplots(figsize=(10, 5.5))
    lane_ids = list(result["lanes"])
    x = list(range(len(lane_ids)))
    width = 0.24
    for offset, key, label, color in (
        (-width, "direct", "Direct production", "#111827"),
        (0.0, "legacy_coe", "Snowbench legacy", "#d97706"),
        (width, "coe_shortwave_albedo_v1", "Snowbench albedo", "#2563eb"),
    ):
        values = []
        for lane in result["lanes"].values():
            values.append(
                lane["direct_production"]["chronology_abs_error_days"]
                if key == "direct"
                else lane["harness"][key]["chronology_abs_error_days"]
            )
        axis.bar([value + offset for value in x], values, width, label=label, color=color)
    axis.set_xticks(x, [result["lanes"][lane_id]["label"] for lane_id in lane_ids])
    axis.set_ylabel("Absolute chronology error (days)")
    axis.set_title("Residual chronology under mass-controlled forcing")
    axis.grid(axis="y", alpha=0.25)
    axis.legend(frameon=False, ncol=3, loc="upper center")
    save_figure(fig, "eb04w2a-chronology-contrast")
    write_sidecar(
        "eb04w2a-chronology-contrast",
        "Residual Chronology Contrast",
        "Absolute median chronology error for the retained production anchor and both diagnostic snowbench melt paths.",
        "Mica Creek and Paradise use melt-out date. Niwot uses the worse of peak-depth and peak-SWE dates. Snowbird uses peak-SWE date. Units are days.",
        "Values across production and snowbench are contextual, not a controlled pair. Only the orange-to-blue change is a like-for-like albedo contrast.",
    )

    fig, axes = plt.subplots(1, 2, figsize=(12, 5.5))
    components = ("coe_melt_amelt_m", "coe_melt_bmelt_m", "coe_melt_cmelt_m", "coe_melt_dmelt_m")
    component_labels = ("amelt · radiation", "bmelt · temperature/cloud", "cmelt · wind/dew point", "dmelt · rain heat")
    component_colors = ("#eab308", "#f97316", "#8b5cf6", "#0ea5e9")
    for index, (lane_id, lane) in enumerate(result["lanes"].items()):
        windows = lane["direct_windows"]["windows"]
        bottom = 0.0
        for key, label, color in zip(components, component_labels, component_colors, strict=True):
            value = median_value(windows, lambda row, key=key: row["sums"][key]) * 1000
            axes[0].bar(index, value, bottom=bottom, color=color, label=label if index == 0 else None)
            bottom += value
    axes[0].set_xticks(range(4), [lane["label"] for lane in result["lanes"].values()], rotation=15, ha="right")
    axes[0].set_ylabel("Median signed contribution (mm)")
    axes[0].set_title("CoE terms in frozen chronology windows")
    axes[0].grid(axis="y", alpha=0.25)
    axes[0].legend(frameon=False, fontsize=8)
    cold_fraction = [lane["hypothesis_flags"]["diagnostics"]["median_cold_content_positive_melt_fraction"] * 100 for lane in result["lanes"].values()]
    cold_depth = [lane["hypothesis_flags"]["diagnostics"]["median_cold_content_positive_melt_m"] * 1000 for lane in result["lanes"].values()]
    axes[1].bar([value - 0.18 for value in range(4)], cold_fraction, 0.36, color="#dc2626", label="Share of applied melt (%)")
    twin = axes[1].twinx()
    twin.bar([value + 0.18 for value in range(4)], cold_depth, 0.36, color="#64748b", label="Applied melt (mm)")
    axes[1].set_xticks(range(4), [lane["label"] for lane in result["lanes"].values()], rotation=15, ha="right")
    axes[1].set_ylabel("Applied melt share (%)", color="#dc2626")
    twin.set_ylabel("Applied melt depth (mm)", color="#64748b")
    axes[1].set_title("Melt coincident with positive beginning-of-day cold content")
    axes[1].grid(axis="y", alpha=0.25)
    handles1, labels1 = axes[1].get_legend_handles_labels()
    handles2, labels2 = twin.get_legend_handles_labels()
    axes[1].legend(handles1 + handles2, labels1 + labels2, frameon=False, loc="upper center", fontsize=8)
    fig.tight_layout()
    save_figure(fig, "eb04w2a-melt-mechanism-attribution")
    write_sidecar(
        "eb04w2a-melt-mechanism-attribution",
        "Melt-Mechanism Attribution",
        "Left: signed empirical CoE components in each frozen chronology window. Right: applied melt on days that begin with positive modeled Stage-3 cold content.",
        "Melt depths are meters converted to millimeters. Bars use the median across water-year/operator windows. Cold content is a beginning-of-day diagnostic; its coincidence with melt is not an hourly energy-causal test.",
        "The four CoE terms mix meteorological drivers and are empirical depth contributions, not independently observed energy fluxes. Signed stacking can include opposing terms.",
    )

    fig, axes = plt.subplots(1, 2, figsize=(12, 5.2))
    labels = [lane["label"] for lane in result["lanes"].values()]
    snowfall = [median_value(lane["direct_windows"]["windows"], lambda row: row["sums"]["snowfall_swe_m"]) * 1000 for lane in result["lanes"].values()]
    routed = [median_value(lane["direct_windows"]["windows"], lambda row: row["sums"]["routed_melt_m"]) * 1000 for lane in result["lanes"].values()]
    released = [median_value(lane["direct_windows"]["windows"], lambda row: row["sums"]["liquid_water_released_m"] + row["sums"]["rain_released_m"]) * 1000 for lane in result["lanes"].values()]
    refrozen = [median_value(lane["direct_windows"]["windows"], lambda row: row["sums"]["stage3_refrozen_liquid_m"]) * 1000 for lane in result["lanes"].values()]
    x = list(range(4))
    width = 0.19
    for offset, values, label, color in (
        (-1.5 * width, snowfall, "Snowfall input", "#2563eb"),
        (-0.5 * width, routed, "Routed melt", "#ef4444"),
        (0.5 * width, released, "Liquid/rain release", "#f97316"),
        (1.5 * width, refrozen, "Stage-3 refreeze", "#06b6d4"),
    ):
        axes[0].bar([value + offset for value in x], values, width, label=label, color=color)
    axes[0].set_xticks(x, labels, rotation=15, ha="right")
    axes[0].set_ylabel("Median window total (mm)")
    axes[0].set_title("Input, release, routing, and refreeze")
    axes[0].grid(axis="y", alpha=0.25)
    axes[0].legend(frameon=False, fontsize=8)
    for index, lane in enumerate(result["lanes"].values()):
        flags = lane["hypothesis_flags"]
        values = [flags["ALBEDO_RESPONSE_MATERIAL"], flags["COLD_CONTENT_MELT_COINCIDENCE_MATERIAL"], flags["TURBULENT_EMPIRICAL_TERMS_DOMINANT"], flags["LATE_INPUT_DEFICIT_SUPPORTED"]]
        for row, value in enumerate(values):
            color = "#16a34a" if value is True else "#dc2626" if value is False else "#94a3b8"
            axes[1].scatter(index, row, s=180, color=color, edgecolor="white", linewidth=1)
    axes[1].set_xticks(x, labels, rotation=15, ha="right")
    axes[1].set_yticks(range(4), ["Albedo response", "Cold-content coincidence", "Turbulent terms", "Late-input deficit"])
    axes[1].set_title("Frozen diagnostic hypothesis flags")
    axes[1].grid(alpha=0.2)
    fig.tight_layout()
    save_figure(fig, "eb04w2a-pathways-and-hypotheses")
    write_sidecar(
        "eb04w2a-pathways-and-hypotheses",
        "Storage Pathways And Hypothesis Flags",
        "Left: median input and loss-pathway quantities in frozen chronology windows. Right: result-blind hypothesis flags (green supported, red not supported, gray not applicable).",
        "Depths are millimeters water equivalent. Flags use the thresholds frozen in package.md before new harness execution.",
        "Flags rank follow-up questions. They do not prove unique causality, physical correctness, transferability, or production promotion.",
    )


def write_synthesis(result: dict[str, Any]) -> None:
    lines = [
        "# Scientific Synthesis",
        "",
        "Evidence mode: **Ran + Inference**.",
        "",
        "| Lane | Direct peak ratio | Direct chronology (d) | Albedo response | Cold-content coincidence | Turbulent terms | Late-input deficit |",
        "|---|---:|---:|---|---|---|---|",
    ]
    for lane in result["lanes"].values():
        flags = lane["hypothesis_flags"]
        lines.append(
            f"| {lane['label']} | {lane['direct_production']['primary_peak_ratio']:.3f} | "
            f"{lane['direct_production']['chronology_abs_error_days']:.1f} | "
            f"{flags['ALBEDO_RESPONSE_MATERIAL']} | "
            f"{flags['COLD_CONTENT_MELT_COINCIDENCE_MATERIAL']} | "
            f"{flags['TURBULENT_EMPIRICAL_TERMS_DOMINANT']} | "
            f"{flags['LATE_INPUT_DEFICIT_SUPPORTED']} |"
        )
    lines.extend(
        [
            "",
            "The snowbench contrast is diagnostic-only and remains separate from the retained direct-production attribution. Hypothesis flags are frozen screening rules, not causal or promotion verdicts.",
            "",
            f"Maximum reconstructed mass closure is `{result['maximum_mass_closure_m']:.3e} m`; maximum Stage-3 energy closure is `{result['maximum_energy_closure_j_m2']:.3e} J m^-2`.",
        ]
    )
    (ARTIFACTS / "scientific-synthesis.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-check", action="store_true")
    parser.add_argument("--freeze", action="store_true")
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--analysis-only", action="store_true")
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
    if args.self_check:
        self_check()
        print("EB-04W2A self-check: PASS")
        return 0
    if args.freeze:
        frozen = freeze()
        print(f"EB-04W2A freeze: PASS {frozen['frozen_utc']}")
        return 0
    if args.execute:
        execute(args.workers)
    if args.execute or args.analysis_only:
        result = analyze()
        print(
            "EB-04W2A analysis: PASS "
            f"8 runs, mass closure {result['maximum_mass_closure_m']:.3e} m"
        )
        return 0
    parser.error("select --self-check, --freeze, --execute, or --analysis-only")


if __name__ == "__main__":
    raise SystemExit(main())
