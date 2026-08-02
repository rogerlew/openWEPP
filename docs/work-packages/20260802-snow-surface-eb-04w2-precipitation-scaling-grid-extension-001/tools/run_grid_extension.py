#!/usr/bin/env python3
"""Freeze, execute, and analyze the EB-04W2 precipitation grid extension."""

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

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
OUTPUT = REPO / "target/snow_surface_eb04w2_precipitation_scaling"
RUNS = OUTPUT / "runs"
FIXTURES = OUTPUT / "fixtures"
BINARY = REPO / "target/release/openwepp-cli-hill"
FREEZE = ARTIFACTS / "experiment-freeze.json"
PREFLIGHT = ARTIFACTS / "transformation-preflight.json"
RECEIPT = ARTIFACTS / "execution-receipt.json"
RESULTS = ARTIFACTS / "precipitation-grid-extension-results.json"
SUMMARY = ARTIFACTS / "precipitation-grid-extension-summary.csv"
W1_PACKAGE = REPO / (
    "docs/work-packages/20260802-snow-surface-eb-04w1-"
    "precipitation-scaling-calibration-001"
)
W1_TOOL = W1_PACKAGE / "tools/run_precipitation_scaling.py"
W1_FREEZE = W1_PACKAGE / "artifacts/experiment-freeze.json"
W1_RECEIPT = W1_PACKAGE / "artifacts/execution-receipt.json"
W1_RESULTS = W1_PACKAGE / "artifacts/precipitation-scaling-results.json"
W1_RUNS = REPO / "target/snow_surface_eb04w1_precipitation_scaling/runs"
RETAINED_GRID = (1.0, 1.1, 1.2, 1.3, 1.4, 1.5)
EXTENSION_GRID = (1.6, 1.7, 1.8, 1.9, 2.0)
FULL_GRID = RETAINED_GRID + EXTENSION_GRID
ROLE = "CALIBRATION"
TOLERANCE_M = 1.0e-12
COMPENSATION_INPUT_RATIO = 1.25
COMPENSATION_STORAGE_RATIO = 0.8
LANE_LABELS = {
    "snotel_mica_creek_st_joe_id": "Mica Creek",
    "snotel_niwot_co": "Niwot",
    "snotel_paradise_wa": "Paradise",
    "snotel_snowbird_ut": "Snowbird",
}


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


w1 = load_module("eb04w2_w1", W1_TOOL)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def scale_id(multiplier: float) -> str:
    return w1.scale_id(multiplier)


def selected_lanes() -> list[Any]:
    return w1.selected_lanes()


def configure_inherited_helpers() -> None:
    w1.OUTPUT = OUTPUT
    w1.RUNS = RUNS
    w1.FIXTURES = FIXTURES
    w1.FREEZE = FREEZE
    w1.RECEIPT = RECEIPT
    w1.BINARY = BINARY


def self_check() -> None:
    w1.self_check()
    fixture = """0
45
 da mo year  prcp  dur tp ip tmax tmin rad w-vl w-dir tdew
  1  1 2000   0.0 0.00 0.0 0.0 -1.0 -2.0 10 1.0 2 -2.0
  2  1 2000   5.5 2.00 0.5 1.0 2.0 -1.0 20 2.0 3 -1.0
  3  1 2000 100.0 5.00 0.2 1.0 3.0 0.0 30 3.0 4 0.0
"""
    with tempfile.TemporaryDirectory(prefix="eb04w2-self-check-") as directory:
        root = Path(directory)
        source = root / "source.cli"
        source.write_text(fixture, encoding="utf-8")
        for multiplier in EXTENSION_GRID:
            audit = w1.scale_climate(source, root / f"{scale_id(multiplier)}.cli", multiplier)
            if audit["daily_row_count"] != 3:
                raise RuntimeError("extension synthetic row count drift")


def preflight() -> dict[str, Any]:
    if RECEIPT.exists() or any(RUNS.rglob("*.snow.jsonl")):
        raise RuntimeError("preflight cannot run after result-bearing execution")
    audits: dict[str, Any] = {}
    with tempfile.TemporaryDirectory(prefix="eb04w2-preflight-") as directory:
        root = Path(directory)
        for lane in selected_lanes():
            source = w1.fixture_cli(lane.fixture_dir)
            for multiplier in EXTENSION_GRID:
                destination = root / scale_id(multiplier) / lane.lane_id / source.name
                audit = w1.scale_climate(source, destination, multiplier)
                audits[f"{lane.lane_id}/{scale_id(multiplier)}"] = audit
    result = {
        "schema": "snow-surface-eb04w2-transformation-preflight-v1",
        "cell_count": len(audits),
        "maximum_precipitation_scaling_residual_mm": max(
            row["maximum_precipitation_scaling_residual_mm"] for row in audits.values()
        ),
        "protected_daily_token_mismatches": sum(
            row["protected_daily_token_mismatches"] for row in audits.values()
        ),
        "non_daily_line_mismatches": sum(
            row["non_daily_line_mismatches"] for row in audits.values()
        ),
        "audits": audits,
    }
    if result["cell_count"] != 20:
        raise RuntimeError("preflight inventory is not 20 cells")
    write_json(PREFLIGHT, result)
    return result


def freeze() -> dict[str, Any]:
    if FREEZE.exists() or RECEIPT.exists() or any(RUNS.rglob("*.snow.jsonl")):
        raise RuntimeError("freeze/result-bearing evidence already exists")
    if not PREFLIGHT.is_file():
        raise FileNotFoundError("transformation preflight must precede freeze")
    if not BINARY.is_file():
        raise FileNotFoundError(f"release binary is missing: {BINARY}")
    w1_freeze = json.loads(W1_FREEZE.read_text(encoding="utf-8"))
    lanes = selected_lanes()
    value = {
        "schema": "snow-surface-eb04w2-experiment-freeze-v1",
        "frozen_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
        "evidence_role": ROLE,
        "independent_validation_role_count": 0,
        "promotion_authorized": False,
        "retained_multipliers": list(RETAINED_GRID),
        "extension_multipliers": list(EXTENSION_GRID),
        "full_analysis_multipliers": list(FULL_GRID),
        "new_run_count": 20,
        "retained_cell_count": 24,
        "combined_cell_count": 44,
        "grid_role": "ASSUMED_FOR_EXECUTION",
        "multiplier_units": "dimensionless",
        "multiplier_semantics": "uniform daily total CLIGEN prcp depth",
        "magnitude_band": [0.9, 1.1],
        "selection_rule": (
            "eligible peak ratio in [0.9,1.1] with strict chronology improvement "
            "versus 1.0; rank abs(log peak ratio), chronology error, distance from 1.0"
        ),
        "compensation_warning": {
            "effective_input_ratio_greater_than": COMPENSATION_INPUT_RATIO,
            "retained_storage_ratio_less_than": COMPENSATION_STORAGE_RATIO,
            "role": "ASSUMED_FOR_EXECUTION reporting threshold",
        },
        "experiment_budget_stop": (
            "2.0 is final; any unresolved or boundary outcome closes this forcing branch "
            "without EB-04W3"
        ),
        "niwot_rule": "SWE magnitude plus worst absolute depth/SWE peak-date offset",
        "operators": w1_freeze["operators"],
        "lanes": [lane.lane_id for lane in lanes],
        "protected_fields": (
            "all climate tokens except daily prcp; all source fixtures, observations, "
            "production code, contracts, selectors, defaults, schemas, and W1 evidence"
        ),
        "source_head": subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=REPO, check=True,
            capture_output=True, text=True,
        ).stdout.strip(),
        "binary_sha256": sha256(BINARY),
        "tool_sha256": sha256(Path(__file__)),
        "preflight_sha256": sha256(PREFLIGHT),
        "w1_freeze_sha256": sha256(W1_FREEZE),
        "w1_receipt_sha256": sha256(W1_RECEIPT),
        "w1_results_sha256": sha256(W1_RESULTS),
        "w1_tool_sha256": sha256(W1_TOOL),
        "source_fixtures": {
            lane.lane_id: {
                "path": relative(lane.fixture_dir),
                "tree": w1.fixture_tree_identity(lane.fixture_dir),
            }
            for lane in lanes
        },
        "observation_files": {
            lane.lane_id: {
                "path": relative(lane.observation_file),
                "sha256": sha256(lane.observation_file),
                "role": ROLE,
            }
            for lane in lanes
        },
    }
    write_json(FREEZE, value)
    return value


def execute_cell(lane: Any, multiplier: float) -> dict[str, Any]:
    configure_inherited_helpers()
    candidate = scale_id(multiplier)
    fixture, transformation = w1.prepare_scaled_fixture(lane, multiplier)
    run_dir = RUNS / candidate / lane.lane_id / "B"
    run_dir.mkdir(parents=True, exist_ok=False)
    stem = f"{lane.lane_id}-B"
    trace = run_dir / f"{stem}.snow.jsonl"
    wat = run_dir / f"{stem}.wat.parquet"
    runfile = run_dir / f"{stem}.run"
    manifest = run_dir / "openwepp_hillslope_run_manifest.json"
    stdout = run_dir / "stdout.txt"
    stderr = run_dir / "stderr.txt"
    provenance_path = run_dir / "eb04w2-cell-provenance.json"
    fixture_stem = w1.eb04r.legacy.observed_harness.discover_run_stem(fixture)
    w1.eb04r.legacy.observed_harness.write_runfile(
        runfile, fixture, fixture_stem, run_dir, stem
    )
    command = w1.eb04r.legacy.observed_harness.cli_command(
        BINARY, fixture, runfile, run_dir, "direct-production-executor"
    )
    environment, removed, effective = w1.eb04r.sanitized_environment(
        os.environ, "B", trace
    )
    started = time.time()
    completed = subprocess.run(
        command, cwd=REPO, env=environment, text=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False,
    )
    stdout.write_text(completed.stdout, encoding="utf-8")
    stderr.write_text(completed.stderr, encoding="utf-8")
    files = {
        name: {"path": relative(path), "sha256": sha256(path), "size_bytes": path.stat().st_size}
        for name, path in {
            "runfile": runfile, "manifest": manifest, "wat": wat, "trace": trace,
            "stdout": stdout, "stderr": stderr,
        }.items() if path.is_file()
    }
    provenance = {
        "schema": "snow-surface-eb04w2-cell-provenance-v1",
        "lane_id": lane.lane_id,
        "multiplier": multiplier,
        "cell": "B",
        "returncode": completed.returncode,
        "started_unix_seconds": started,
        "completed_unix_seconds": time.time(),
        "argv": [str(value) for value in command],
        "binary_sha256": sha256(BINARY),
        "tool_sha256": sha256(Path(__file__)),
        "freeze_sha256": sha256(FREEZE),
        "source_fixture": relative(lane.fixture_dir),
        "scaled_fixture": relative(fixture),
        "transformation": transformation,
        "removed_openwepp_key_names": removed,
        "effective_openwepp_environment": effective,
        "files": files,
    }
    write_json(provenance_path, provenance)
    return {
        "returncode": completed.returncode,
        "provenance": relative(provenance_path),
        "provenance_sha256": sha256(provenance_path),
    }


def execute(workers: int) -> dict[str, Any]:
    if not FREEZE.is_file():
        raise FileNotFoundError("freeze must exist before execution")
    if RECEIPT.exists() or any(RUNS.rglob("*.snow.jsonl")):
        raise RuntimeError("result-bearing execution already exists")
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))
    if frozen["tool_sha256"] != sha256(Path(__file__)):
        raise RuntimeError("execution tool changed after freeze")
    if frozen["binary_sha256"] != sha256(BINARY):
        raise RuntimeError("release binary changed after freeze")
    futures: dict[Any, tuple[str, float]] = {}
    results: dict[str, Any] = {}
    with ThreadPoolExecutor(max_workers=workers) as executor:
        for lane in selected_lanes():
            for multiplier in EXTENSION_GRID:
                future = executor.submit(execute_cell, lane, multiplier)
                futures[future] = (lane.lane_id, multiplier)
        for future in as_completed(futures):
            lane_id, multiplier = futures[future]
            key = f"{lane_id}/{scale_id(multiplier)}"
            result = future.result()
            results[key] = result
            print(f"{key}: {'PASS' if result['returncode'] == 0 else 'FAIL'}")
    if len(results) != 20 or any(row["returncode"] != 0 for row in results.values()):
        raise RuntimeError("frozen 20-run extension did not complete")
    receipt = {
        "schema": "snow-surface-eb04w2-execution-receipt-v1",
        "evidence_role": ROLE,
        "binary_sha256": sha256(BINARY),
        "tool_sha256": sha256(Path(__file__)),
        "freeze_sha256": sha256(FREEZE),
        "working_directory": str(REPO),
        "execution_command": (
            ".venv/bin/python docs/work-packages/20260802-snow-surface-eb-04w2-"
            f"precipitation-scaling-grid-extension-001/tools/{Path(__file__).name} "
            f"--execute --workers {workers}"
        ),
        "result_count": len(results),
        "results": dict(sorted(results.items())),
    }
    write_json(RECEIPT, receipt)
    return receipt


def retained_anchor_audit() -> dict[str, Any]:
    receipt = json.loads(W1_RECEIPT.read_text(encoding="utf-8"))
    checked = 0
    for lane in selected_lanes():
        for multiplier in RETAINED_GRID:
            entry = receipt["results"][f"{lane.lane_id}/{scale_id(multiplier)}"]
            provenance_path = REPO / entry["provenance"]
            if sha256(provenance_path) != entry["provenance_sha256"]:
                raise RuntimeError("retained provenance identity mismatch")
            provenance = json.loads(provenance_path.read_text(encoding="utf-8"))
            for output in provenance["files"].values():
                path = REPO / output["path"]
                if not path.is_file() or sha256(path) != output["sha256"]:
                    raise RuntimeError(f"retained output identity mismatch: {path}")
                checked += 1
    return {"retained_cell_count": 24, "retained_output_identity_count": checked}


def analyze_candidate(lane: Any, multiplier: float, source: str, operators: list[Any]) -> dict[str, Any]:
    run_root = (W1_RUNS if source == "retained" else RUNS) / scale_id(multiplier)
    w1.eb04w.RUNS = run_root
    cell = w1.eb04w.analyze_cell(lane, "B")
    operator_rows = [
        w1.operator_metric(lane, operator, retained, cell, run_root)
        for lane_id, operator, retained in operators if lane_id == lane.lane_id
    ]
    primary = w1.primary_operator(lane.lane_id, operator_rows)
    closure_values = [
        cell["maximum_melt_component_closure_m"],
        cell["maximum_uncapped_melt_component_closure_m"],
        cell["maximum_daily_applied_raw_melt_closure_m"],
        cell["maximum_accumulation_closure_m"],
        cell["maximum_phase_amount_closure_m"],
        cell["maximum_snow_depth_swe_closure_m"],
        cell["maximum_trace_wat_swe_closure_m"],
        cell["maximum_trace_wat_depth_closure_m"],
        *(row["maximum_mass_closure_m"] for row in operator_rows),
    ]
    return {
        "multiplier": multiplier,
        "source": source,
        "primary_peak_ratio": primary["median_peak_ratio"],
        "chronology_abs_error_days": max(abs(row["executed_offset_days"]) for row in operator_rows),
        "operators": operator_rows,
        "maximum_closure_m": max(closure_values),
        "totals": cell["totals"],
        "daily": cell["daily"],
    }


def parity_bracket(rows: list[dict[str, Any]]) -> list[float] | None:
    ordered = sorted(rows, key=lambda row: row["multiplier"])
    for left, right in zip(ordered, ordered[1:]):
        if (left["primary_peak_ratio"] - 1.0) * (right["primary_peak_ratio"] - 1.0) <= 0.0:
            return [left["multiplier"], right["multiplier"]]
    return None


def select_candidate(lane_id: str, rows: list[dict[str, Any]]) -> dict[str, Any]:
    baseline = next(row for row in rows if row["multiplier"] == 1.0)
    magnitude_best = min(
        rows, key=lambda row: (abs(math.log(row["primary_peak_ratio"])), row["chronology_abs_error_days"], row["multiplier"])
    )
    chronology_best = min(
        rows, key=lambda row: (row["chronology_abs_error_days"], abs(math.log(row["primary_peak_ratio"])), row["multiplier"])
    )
    eligible = [
        row for row in rows
        if 0.9 <= row["primary_peak_ratio"] <= 1.1
        and row["chronology_abs_error_days"] < baseline["chronology_abs_error_days"] - 1.0e-12
    ]
    eligible.sort(key=lambda row: (
        abs(math.log(row["primary_peak_ratio"])),
        row["chronology_abs_error_days"],
        abs(row["multiplier"] - 1.0),
    ))
    selected = eligible[0] if eligible else None
    selected_primary = w1.primary_operator(lane_id, selected["operators"]) if selected else None
    compensation = bool(
        selected_primary
        and selected_primary["median_effective_input_ratio"] > COMPENSATION_INPUT_RATIO
        and selected_primary["median_storage_ratio"] < COMPENSATION_STORAGE_RATIO
    )
    tradeoff = magnitude_best["multiplier"] != chronology_best["multiplier"]
    boundary = (
        selected is None
        or selected["multiplier"] == EXTENSION_GRID[-1]
        or magnitude_best["multiplier"] == EXTENSION_GRID[-1]
    )
    if boundary:
        classification = "EXPERIMENT_BUDGET_BOUNDARY"
    elif compensation:
        classification = "BRACKETED_WITH_COMPENSATION_WARNING"
    elif tradeoff:
        classification = "TRADEOFF_BRACKETED"
    else:
        classification = "BRACKETED_CANDIDATE"
    return {
        "classification": classification,
        "selected_multiplier": selected["multiplier"] if selected else None,
        "selected_primary_peak_ratio": selected["primary_peak_ratio"] if selected else None,
        "selected_chronology_abs_error_days": selected["chronology_abs_error_days"] if selected else None,
        "eligible_candidate_count": len(eligible),
        "magnitude_best_multiplier": magnitude_best["multiplier"],
        "magnitude_best_peak_ratio": magnitude_best["primary_peak_ratio"],
        "chronology_best_multiplier": chronology_best["multiplier"],
        "chronology_best_error_days": chronology_best["chronology_abs_error_days"],
        "parity_bracket": parity_bracket(rows),
        "compensation_warning": compensation,
        "baseline_primary_peak_ratio": baseline["primary_peak_ratio"],
        "baseline_chronology_abs_error_days": baseline["chronology_abs_error_days"],
    }


def strip_daily(result: dict[str, Any]) -> dict[str, Any]:
    serial = json.loads(json.dumps(result))
    for lane in serial["lanes"].values():
        for candidate in lane["candidates"]:
            candidate.pop("daily", None)
            for operator in candidate["operators"]:
                operator.pop("water_years", None)
    return serial


def analyze() -> dict[str, Any]:
    if not RECEIPT.is_file():
        raise FileNotFoundError("extension execution receipt is missing")
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))
    if frozen["tool_sha256"] != sha256(Path(__file__)):
        raise RuntimeError("analysis tool differs from frozen execution tool")
    anchors = retained_anchor_audit()
    lanes = selected_lanes()
    by_lane: dict[str, list[dict[str, Any]]] = {lane.lane_id: [] for lane in lanes}
    maximum_closure = 0.0
    for lane in lanes:
        for multiplier in FULL_GRID:
            source = "retained" if multiplier in RETAINED_GRID else "extension"
            candidate = analyze_candidate(lane, multiplier, source, frozen["operators"])
            by_lane[lane.lane_id].append(candidate)
            maximum_closure = max(maximum_closure, candidate["maximum_closure_m"])
    lanes_result = {
        lane.lane_id: {
            "label": LANE_LABELS[lane.lane_id],
            "candidates": by_lane[lane.lane_id],
            "selection": select_candidate(lane.lane_id, by_lane[lane.lane_id]),
        }
        for lane in lanes
    }
    result = {
        "schema": "snow-surface-eb04w2-precipitation-grid-results-v1",
        "evidence_role": ROLE,
        "independent_validation": False,
        "freeze_sha256": sha256(FREEZE),
        "receipt_sha256": sha256(RECEIPT),
        "tool_sha256": sha256(Path(__file__)),
        "new_run_count": 20,
        "retained_cell_count": anchors["retained_cell_count"],
        "combined_cell_count": 44,
        "retained_output_identity_count": anchors["retained_output_identity_count"],
        "maximum_closure_m": maximum_closure,
        "lanes": lanes_result,
    }
    if maximum_closure > TOLERANCE_M:
        raise RuntimeError(f"diagnostic closure exceeded tolerance: {maximum_closure}")
    write_json(RESULTS, strip_daily(result))
    write_summary(result)
    write_figures(result, lanes)
    write_synthesis(result)
    return result


def write_summary(result: dict[str, Any]) -> None:
    with SUMMARY.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle, lineterminator="\n")
        writer.writerow([
            "lane_id", "multiplier", "source", "primary_peak_ratio",
            "chronology_abs_error_days", "median_effective_input_ratio",
            "median_storage_ratio", "classification", "selected_multiplier",
        ])
        for lane_id, lane in result["lanes"].items():
            selection = lane["selection"]
            for candidate in lane["candidates"]:
                primary = w1.primary_operator(lane_id, candidate["operators"])
                writer.writerow([
                    lane_id, candidate["multiplier"], candidate["source"],
                    candidate["primary_peak_ratio"], candidate["chronology_abs_error_days"],
                    primary["median_effective_input_ratio"], primary["median_storage_ratio"],
                    selection["classification"], selection["selected_multiplier"],
                ])


def save_figure(fig: Any, stem: str) -> None:
    FIGURES.mkdir(parents=True, exist_ok=True)
    fig.savefig(FIGURES / f"{stem}.svg", format="svg", bbox_inches="tight")


def write_sidecar(stem: str, title: str, caption: str, notice: str, methods: str) -> None:
    (FIGURES / f"{stem}.md").write_text(
        f"# {title}\n\n![{title}]({stem}.svg)\n\n## Caption\n\n{caption}\n\n"
        f"## What To Notice\n\n{notice}\n\n## Methods And Provenance\n\n{methods}\n\n"
        "## Uncertainty And Interpretation Limits\n\nThe same SNOTEL records are "
        "calibration data in EB-04W1 and EB-04W2. Curves summarize medians and "
        "do not show interannual spread. They are not independent validation, do "
        "not identify precipitation error as the unique cause, and do not support "
        "a transferable multiplier, production default, or process promotion.\n",
        encoding="utf-8",
    )


def median_daily(candidate: dict[str, Any]) -> tuple[list[int], list[float]]:
    grouped: dict[int, list[float]] = {}
    for day in candidate["daily"]:
        date = dt.date.fromisoformat(day["date"])
        start = dt.date(date.year if date.month >= 10 else date.year - 1, 10, 1)
        grouped.setdefault((date - start).days, []).append(float(day["swe_m"]))
    x = sorted(grouped)
    return x, [statistics.median(grouped[index]) for index in x]


def write_figures(result: dict[str, Any], lanes: list[Any]) -> None:
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    colors = ["#0072B2", "#D55E00", "#009E73", "#CC79A7"]
    fig, axes = plt.subplots(2, 2, figsize=(11.5, 7.5), sharex=True)
    for lane, color in zip(lanes, colors):
        rows = result["lanes"][lane.lane_id]["candidates"]
        x = [row["multiplier"] for row in rows]
        label = LANE_LABELS[lane.lane_id]
        measures = [
            [row["primary_peak_ratio"] for row in rows],
            [row["chronology_abs_error_days"] for row in rows],
            [w1.primary_operator(lane.lane_id, row["operators"])["median_effective_input_ratio"] for row in rows],
            [w1.primary_operator(lane.lane_id, row["operators"])["median_storage_ratio"] for row in rows],
        ]
        for axis, values in zip(axes.flat, measures):
            axis.plot(x, values, "o-", color=color, label=label)
            axis.axvline(1.5, color="#777777", linestyle=":", linewidth=1)
    axes[0, 0].axhspan(0.9, 1.1, color="#E6E6E6", zorder=0)
    for axis in (axes[0, 0], axes[1, 0], axes[1, 1]):
        axis.axhline(1.0, color="black", linewidth=1)
    axes[0, 0].set_ylabel("Median modeled / observed peak SWE")
    axes[0, 1].set_ylabel("Worst absolute chronology offset (days)")
    axes[1, 0].set_ylabel("Median effective input / observed peak")
    axes[1, 1].set_ylabel("Median retained SWE / observed peak")
    for axis in axes[1]:
        axis.set_xlabel("Total-precipitation multiplier")
    for axis in axes.flat:
        axis.grid(True, alpha=0.25)
    axes[0, 0].legend(loc="best", frameon=True)
    fig.suptitle("Bounded precipitation-scaling response: retained anchors and extension")
    fig.tight_layout()
    save_figure(fig, "eb04w2-response-curves")
    plt.close(fig)
    write_sidecar(
        "eb04w2-response-curves", "Bounded Precipitation-Scaling Response Curves",
        "Peak magnitude, chronology, effective input, and retained storage span the combined 1.0-2.0 surface; the dotted line separates retained EB-04W1 anchors from new EB-04W2 cells.",
        "Look for the multiplier where peak ratio enters or crosses the gray 0.9-1.1 band, whether chronology improves at the same point, and whether input rises much faster than retained storage.",
        "Each point is a real baseline-B release run. Values at 1.0-1.5 are hash-verified EB-04W1 outputs; 1.6-2.0 are new EB-04W2 runs. Only daily total precipitation changes.",
    )

    w1_results = json.loads(W1_RESULTS.read_text(encoding="utf-8"))
    fig, axes = plt.subplots(2, 2, figsize=(11.5, 8.0))
    for axis, lane in zip(axes.flat, lanes):
        lane_result = result["lanes"][lane.lane_id]
        selected = lane_result["selection"]["selected_multiplier"]
        if selected is None:
            selected = lane_result["selection"]["magnitude_best_multiplier"]
        w1_selected = w1_results["lanes"][lane.lane_id]["selection"]["selected_multiplier"] or 1.0
        observed_grouped: dict[int, list[float]] = {}
        for row in w1.eb04w.observation_rows(lane):
            try:
                date = dt.date.fromisoformat(row["date"])
                swe = float(row["observed_swe_mm"]) / 1000.0
            except (KeyError, TypeError, ValueError):
                continue
            start = dt.date(date.year if date.month >= 10 else date.year - 1, 10, 1)
            observed_grouped.setdefault((date - start).days, []).append(swe)
        ox = sorted(observed_grouped)
        axis.plot(ox, [statistics.median(observed_grouped[i]) for i in ox], color="black", linewidth=2.2, label="Observed")
        for multiplier, color, label in (
            (1.0, "#777777", "Baseline 1.0"),
            (w1_selected, "#56B4E9", f"W1 selected {w1_selected:.1f}"),
            (selected, "#D55E00", f"W2 candidate {selected:.1f}"),
        ):
            candidate = next(row for row in lane_result["candidates"] if row["multiplier"] == multiplier)
            x, y = median_daily(candidate)
            axis.plot(x, y, color=color, linewidth=1.8, label=label)
        axis.set_title(LANE_LABELS[lane.lane_id])
        axis.set_xlabel("Day of water year")
        axis.set_ylabel("Median SWE (m)")
        axis.grid(True, alpha=0.25)
        axis.legend(loc="best")
    fig.suptitle("Seasonal SWE trajectories across the bounded calibration extension")
    fig.tight_layout()
    save_figure(fig, "eb04w2-seasonal-swe-trajectories")
    plt.close(fig)
    write_sidecar(
        "eb04w2-seasonal-swe-trajectories", "Seasonal SWE Trajectories Across EB-04W2",
        "Median observed SWE is compared with baseline, the EB-04W1 selected cell, and the EB-04W2 magnitude-first candidate for each lane.",
        "A useful forcing correction should improve both seasonal mass and persistence. A taller trajectory that still disappears early, or grossly overshoots the observed curve, is a tradeoff or compensation warning.",
        "Daily SWE is grouped by day of water year and summarized by the median across the calibration record. The black line is observed; gray, blue, and orange are modeled.",
    )

    fig, axes = plt.subplots(2, 2, figsize=(11.5, 8.0))
    for axis, lane, color in zip(axes.flat, lanes, colors):
        rows = result["lanes"][lane.lane_id]["candidates"]
        axis.axvspan(0.9, 1.1, color="#E6E6E6", zorder=0)
        axis.plot([row["primary_peak_ratio"] for row in rows], [row["chronology_abs_error_days"] for row in rows], "o-", color=color)
        for row in rows:
            axis.annotate(f"{row['multiplier']:.1f}", (row["primary_peak_ratio"], row["chronology_abs_error_days"]), xytext=(4, 4), textcoords="offset points", fontsize=8)
        axis.set_title(LANE_LABELS[lane.lane_id])
        axis.set_xlabel("Median modeled / observed peak SWE")
        axis.set_ylabel("Chronology error (days; lower is better)")
        axis.grid(True, alpha=0.25)
    fig.suptitle("Magnitude-chronology calibration tradeoffs")
    fig.tight_layout()
    save_figure(fig, "eb04w2-magnitude-chronology-tradeoffs")
    plt.close(fig)
    write_sidecar(
        "eb04w2-magnitude-chronology-tradeoffs", "Magnitude-Chronology Calibration Tradeoffs",
        "Each labeled point maps one precipitation multiplier to peak-SWE ratio and absolute chronology error; the gray vertical band is the frozen 0.9-1.1 magnitude target.",
        "The desirable direction is into the gray band and downward. A path that moves right but stops moving down exposes a magnitude-versus-timing tradeoff rather than a single optimum.",
        "The plot uses the inherited peak/melt-out operator per lane; Niwot chronology is the worse of its depth-peak and SWE-peak offsets.",
    )

    fig, axes = plt.subplots(2, 2, figsize=(11.5, 8.0))
    for axis, lane, color in zip(axes.flat, lanes, colors):
        rows = result["lanes"][lane.lane_id]["candidates"]
        inputs = [w1.primary_operator(lane.lane_id, row["operators"])["median_effective_input_ratio"] for row in rows]
        storage = [w1.primary_operator(lane.lane_id, row["operators"])["median_storage_ratio"] for row in rows]
        axis.plot(inputs, storage, "o-", color=color)
        for row, x, y in zip(rows, inputs, storage):
            if row["multiplier"] in (1.0, 1.5, 2.0) or row["multiplier"] == result["lanes"][lane.lane_id]["selection"]["selected_multiplier"]:
                axis.annotate(f"{row['multiplier']:.1f}", (x, y), xytext=(4, 4), textcoords="offset points", fontsize=8)
        axis.axvline(COMPENSATION_INPUT_RATIO, color="#D55E00", linestyle=":", linewidth=1)
        axis.axhline(COMPENSATION_STORAGE_RATIO, color="#D55E00", linestyle=":", linewidth=1)
        axis.axline((0, 0), slope=1, color="#777777", linestyle="--", linewidth=1)
        axis.set_title(LANE_LABELS[lane.lane_id])
        axis.set_xlabel("Effective input / observed peak SWE")
        axis.set_ylabel("Retained SWE / observed peak SWE")
        axis.grid(True, alpha=0.25)
    fig.suptitle("Effective-input to retained-storage pathways")
    fig.tight_layout()
    save_figure(fig, "eb04w2-input-storage-pathways")
    plt.close(fig)
    write_sidecar(
        "eb04w2-input-storage-pathways", "Effective-Input And Retained-Storage Pathways",
        "Each lane traces how effective snowpack input and SWE retained on the observed peak date change with precipitation scaling. Labels identify baseline, the W1 boundary, the W2 candidate when distinct, and the 2.0 budget cell.",
        "The dashed diagonal is one-to-one storage. Orange dotted thresholds mark the frozen compensation-warning quadrant: input above 1.25 with retained storage below 0.8.",
        "Effective input is initial SWE plus realized snowfall SWE plus retained rain through the observed SWE peak. Storage is modeled SWE on that date; both are median ratios across paired water years.",
    )


def write_synthesis(result: dict[str, Any]) -> None:
    lines = [
        "# EB-04W2 Scientific Synthesis", "",
        "Evidence mode: **Ran + Inference**. Observations remain `CALIBRATION`; no independent-validation, transferability, default, or promotion claim is made.", "",
        "## Lane Adjudication", "",
        "| Lane | Selected | Peak ratio | Chronology (d) | Magnitude best | Chronology best | Parity bracket | Classification |",
        "|---|---:|---:|---:|---:|---:|---|---|",
    ]
    for lane in result["lanes"].values():
        row = lane["selection"]
        bracket = "—" if row["parity_bracket"] is None else f"{row['parity_bracket'][0]:.1f}-{row['parity_bracket'][1]:.1f}"
        lines.append(
            f"| {lane['label']} | {row['selected_multiplier'] if row['selected_multiplier'] is not None else '—'} | "
            f"{row['selected_primary_peak_ratio'] if row['selected_primary_peak_ratio'] is not None else '—'} | "
            f"{row['selected_chronology_abs_error_days'] if row['selected_chronology_abs_error_days'] is not None else '—'} | "
            f"{row['magnitude_best_multiplier']} | {row['chronology_best_multiplier']} | {bracket} | {row['classification']} |"
        )
    lines.extend([
        "", "## Closure And Stop-Loss", "",
        f"The combined surface contains 44 cells, including 20 new runs. Maximum reconstructed diagnostic closure is `{result['maximum_closure_m']:.3e} m`.", "",
        "The 2.0 ceiling is the final forcing-experiment budget. A boundary result does not authorize another grid extension. Selected values are candidates from this calibration experiment, not empirical validation or transferable multipliers.", "",
    ])
    (ARTIFACTS / "scientific-synthesis.md").write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--self-check", action="store_true")
    group.add_argument("--preflight", action="store_true")
    group.add_argument("--freeze", action="store_true")
    group.add_argument("--execute", action="store_true")
    group.add_argument("--analysis-only", action="store_true")
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    FIGURES.mkdir(parents=True, exist_ok=True)
    if args.self_check:
        self_check()
        print("EB-04W2 transformer self-check: PASS")
        return 0
    if args.preflight:
        result = preflight()
        print(f"EB-04W2 transformation preflight: PASS ({result['cell_count']} cells)")
        return 0
    if args.freeze:
        value = freeze()
        print(f"EB-04W2 prospective freeze: PASS ({value['new_run_count']} new runs)")
        return 0
    if args.execute:
        execute(args.workers)
    result = analyze()
    print(
        "EB-04W2 analysis: PASS "
        f"({result['combined_cell_count']} cells; max closure {result['maximum_closure_m']:.3e} m)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
