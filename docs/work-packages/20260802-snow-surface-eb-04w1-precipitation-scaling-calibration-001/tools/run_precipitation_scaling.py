#!/usr/bin/env python3
"""Freeze, execute, and analyze the EB-04W1 precipitation-scaling grid."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import importlib.util
import json
import math
import os
import re
import shutil
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
OUTPUT = REPO / "target/snow_surface_eb04w1_precipitation_scaling"
RUNS = OUTPUT / "runs"
FIXTURES = OUTPUT / "fixtures"
BINARY = REPO / "target/release/openwepp-cli-hill"
FREEZE = ARTIFACTS / "experiment-freeze.json"
RECEIPT = ARTIFACTS / "execution-receipt.json"
RESULTS = ARTIFACTS / "precipitation-scaling-results.json"
SUMMARY = ARTIFACTS / "precipitation-scaling-summary.csv"
EB04W_PACKAGE = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04w-"
    "accumulation-under-persistence-001"
)
EB04W_TOOL = EB04W_PACKAGE / "tools/run_accumulation_diagnostics.py"
EB04W_FREEZE = EB04W_PACKAGE / "artifacts/population-freeze.json"
EB04W_RESULTS = EB04W_PACKAGE / "artifacts/accumulation-mechanics-results.json"
EB04W_RECEIPT = EB04W_PACKAGE / "artifacts/execution-receipt.json"
EB04R_TOOL = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04r-fresh-factorial-"
    "execution-adjudication-001/tools/run_experiment.py"
)
MULTIPLIERS = (0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5)
ROLE = "CALIBRATION"
TOLERANCE_M = 1.0e-12
DAILY_RE = re.compile(
    r"^(\s*\d+\s+\d+\s+\d{4}\s+)([-+]?\d+(?:\.\d+)?(?:[Ee][-+]?\d+)?)(\s+.*)$"
)

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


eb04w = load_module("eb04w1_eb04w", EB04W_TOOL)
eb04r = load_module("eb04w1_eb04r", EB04R_TOOL)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def scale_id(multiplier: float) -> str:
    return f"p{round(multiplier * 100):03d}"


def selected_lanes() -> list[Any]:
    frozen = json.loads(EB04W_FREEZE.read_text(encoding="utf-8"))
    expected = set(frozen["unique_lanes"])
    lanes = [lane for lane in eb04r.legacy.fixed_lanes() if lane.lane_id in expected]
    if len(lanes) != 4 or {lane.lane_id for lane in lanes} != expected:
        raise RuntimeError("runtime lanes differ from the frozen EB-04W population")
    return sorted(lanes, key=lambda lane: lane.lane_id)


def daily_rows(lines: list[str]) -> list[tuple[int, re.Match[str]]]:
    rows: list[tuple[int, re.Match[str]]] = []
    for index, line in enumerate(lines):
        match = DAILY_RE.match(line)
        if match is not None:
            rows.append((index, match))
    if not rows:
        raise ValueError("climate file contains no recognized daily rows")
    return rows


def scale_climate(source: Path, destination: Path, multiplier: float) -> dict[str, Any]:
    if not math.isfinite(multiplier) or multiplier <= 0.0:
        raise ValueError("precipitation multiplier must be finite and positive")
    source_lines = source.read_text(encoding="utf-8").splitlines()
    transformed = list(source_lines)
    source_total = 0.0
    scaled_total = 0.0
    rows = daily_rows(source_lines)
    for index, match in rows:
        source_value = float(match.group(2))
        if source_value < 0.0:
            raise ValueError(f"negative source precipitation on line {index + 1}")
        scaled_value = source_value * multiplier
        source_total += source_value
        scaled_total += scaled_value
        if multiplier == 1.0:
            continue
        transformed[index] = f"{match.group(1)}{scaled_value:.6f}{match.group(3)}"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text("\n".join(transformed) + "\n", encoding="utf-8")
    audit = verify_scaled_climate(source, destination, multiplier)
    audit.update(
        {
            "daily_row_count": len(rows),
            "source_precipitation_total_mm": source_total,
            "scaled_precipitation_total_mm": scaled_total,
            "source_sha256": sha256(source),
            "scaled_sha256": sha256(destination),
        }
    )
    return audit


def verify_scaled_climate(source: Path, scaled: Path, multiplier: float) -> dict[str, Any]:
    source_lines = source.read_text(encoding="utf-8").splitlines()
    scaled_lines = scaled.read_text(encoding="utf-8").splitlines()
    if len(source_lines) != len(scaled_lines):
        raise RuntimeError("scaled climate line count differs from source")
    source_rows = dict(daily_rows(source_lines))
    scaled_rows = dict(daily_rows(scaled_lines))
    if source_rows.keys() != scaled_rows.keys():
        raise RuntimeError("scaled climate daily-row inventory differs from source")
    maximum_residual = 0.0
    protected_token_mismatches = 0
    for index in source_rows:
        source_tokens = source_lines[index].split()
        scaled_tokens = scaled_lines[index].split()
        if len(source_tokens) != len(scaled_tokens) or len(source_tokens) < 11:
            raise RuntimeError(f"daily token inventory changed on line {index + 1}")
        maximum_residual = max(
            maximum_residual,
            abs(float(scaled_tokens[3]) - float(source_tokens[3]) * multiplier),
        )
        if source_tokens[:3] != scaled_tokens[:3] or source_tokens[4:] != scaled_tokens[4:]:
            protected_token_mismatches += 1
    non_daily_mismatches = sum(
        source_lines[index] != scaled_lines[index]
        for index in range(len(source_lines))
        if index not in source_rows
    )
    if maximum_residual > 5.1e-7:
        raise RuntimeError(f"scaled precipitation residual is too large: {maximum_residual}")
    if protected_token_mismatches or non_daily_mismatches:
        raise RuntimeError(
            "climate transformation changed protected tokens or non-daily lines"
        )
    return {
        "maximum_precipitation_scaling_residual_mm": maximum_residual,
        "protected_daily_token_mismatches": protected_token_mismatches,
        "non_daily_line_mismatches": non_daily_mismatches,
    }


def self_check() -> None:
    fixture = """0\n45\n da mo year  prcp  dur tp ip tmax tmin rad w-vl w-dir tdew\n  1  1 2000   0.0 0.00 0.0 0.0 -1.0 -2.0 10 1.0 2 -2.0\n  2  1 2000   5.5 2.00 0.5 1.0 2.0 -1.0 20 2.0 3 -1.0\n  3  1 2000 100.0 5.00 0.2 1.0 3.0 0.0 30 3.0 4 0.0\n"""
    with tempfile.TemporaryDirectory(prefix="eb04w1-self-check-") as directory:
        root = Path(directory)
        source = root / "source.cli"
        source.write_text(fixture, encoding="utf-8")
        for multiplier in (0.8, 1.0, 1.5):
            scaled = root / f"{scale_id(multiplier)}.cli"
            audit = scale_climate(source, scaled, multiplier)
            if audit["daily_row_count"] != 3:
                raise RuntimeError("synthetic daily-row count drift")
        malformed = root / "malformed.cli"
        malformed.write_text("no daily climate records\n", encoding="utf-8")
        try:
            scale_climate(malformed, root / "bad.cli", 1.2)
        except ValueError:
            pass
        else:
            raise RuntimeError("malformed climate did not fail closed")


def fixture_cli(fixture: Path) -> Path:
    cli_files = sorted(fixture.glob("*.cli"))
    if len(cli_files) != 1:
        raise RuntimeError(f"expected one climate file in {fixture}, found {len(cli_files)}")
    return cli_files[0]


def fixture_tree_identity(root: Path) -> dict[str, str]:
    return {
        str(path.relative_to(root)): sha256(path)
        for path in sorted(root.rglob("*"))
        if path.is_file()
    }


def prepare_freeze() -> None:
    if FREEZE.exists() or RECEIPT.exists() or any(RUNS.rglob("*.snow.jsonl")):
        raise RuntimeError("freeze/result-bearing evidence already exists")
    if not BINARY.is_file():
        raise FileNotFoundError(f"release binary is missing: {BINARY}")
    predecessor_freeze = json.loads(EB04W_FREEZE.read_text(encoding="utf-8"))
    lanes = selected_lanes()
    freeze = {
        "schema": "snow-surface-eb04w1-experiment-freeze-v1",
        "frozen_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
        "evidence_role": ROLE,
        "independent_validation_role_count": 0,
        "promotion_authorized": False,
        "multipliers": list(MULTIPLIERS),
        "multiplier_semantics": "uniform daily total CLIGEN prcp depth",
        "multiplier_units": "dimensionless",
        "grid_role": "ASSUMED_FOR_EXECUTION",
        "candidate_count": 32,
        "cells": ["B"],
        "lanes": [lane.lane_id for lane in lanes],
        "operators": predecessor_freeze["operators"],
        "selection_rule": (
            "strict joint improvement in abs(log median SWE peak ratio) and "
            "worst absolute lane chronology offset versus 1.0; prefer peak ratio "
            "in [0.9,1.1], then chronology, then distance from 1.0"
        ),
        "niwot_rule": "SWE magnitude plus worst absolute depth/SWE peak-date offset",
        "protected_fields": (
            "all climate tokens except daily prcp; all source fixtures, observations, "
            "production code, contracts, selectors, defaults, and schemas"
        ),
        "source_head": subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=REPO,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip(),
        "binary_sha256": sha256(BINARY),
        "tool_sha256": sha256(Path(__file__)),
        "eb04w_freeze_sha256": sha256(EB04W_FREEZE),
        "eb04w_results_sha256": sha256(EB04W_RESULTS),
        "eb04w_receipt_sha256": sha256(EB04W_RECEIPT),
        "source_fixtures": {
            lane.lane_id: {
                "path": relative(lane.fixture_dir),
                "tree": fixture_tree_identity(lane.fixture_dir),
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
    write_json(FREEZE, freeze)


def prepare_scaled_fixture(lane: Any, multiplier: float) -> tuple[Path, dict[str, Any]]:
    destination = FIXTURES / scale_id(multiplier) / lane.lane_id
    if destination.exists():
        raise RuntimeError(f"scaled fixture already exists: {destination}")
    shutil.copytree(lane.fixture_dir, destination)
    source_cli = fixture_cli(lane.fixture_dir)
    destination_cli = destination / source_cli.name
    audit = scale_climate(source_cli, destination_cli, multiplier)
    source_tree = fixture_tree_identity(lane.fixture_dir)
    scaled_tree = fixture_tree_identity(destination)
    changed = sorted(
        path for path in source_tree if source_tree[path] != scaled_tree.get(path)
    )
    expected_changed = [] if multiplier == 1.0 else [source_cli.name]
    if changed != expected_changed or set(source_tree) != set(scaled_tree):
        raise RuntimeError(
            f"scaled fixture changed unexpected files: {lane.lane_id}/{multiplier}: {changed}"
        )
    return destination, {
        **audit,
        "source_fixture_tree_sha256": eb04r.legacy.tree_sha256(lane.fixture_dir),
        "scaled_fixture_tree_sha256": eb04r.legacy.tree_sha256(destination),
        "changed_files": changed,
    }


def execute_cell(lane: Any, multiplier: float) -> dict[str, Any]:
    candidate = scale_id(multiplier)
    fixture, transformation = prepare_scaled_fixture(lane, multiplier)
    run_dir = RUNS / candidate / lane.lane_id / "B"
    run_dir.mkdir(parents=True, exist_ok=False)
    stem = f"{lane.lane_id}-B"
    trace = run_dir / f"{stem}.snow.jsonl"
    wat = run_dir / f"{stem}.wat.parquet"
    runfile = run_dir / f"{stem}.run"
    manifest = run_dir / "openwepp_hillslope_run_manifest.json"
    stdout = run_dir / "stdout.txt"
    stderr = run_dir / "stderr.txt"
    provenance_path = run_dir / "eb04w1-cell-provenance.json"
    fixture_stem = eb04r.legacy.observed_harness.discover_run_stem(fixture)
    eb04r.legacy.observed_harness.write_runfile(
        runfile, fixture, fixture_stem, run_dir, stem
    )
    command = eb04r.legacy.observed_harness.cli_command(
        BINARY, fixture, runfile, run_dir, "direct-production-executor"
    )
    environment, removed, effective = eb04r.sanitized_environment(
        os.environ, "B", trace
    )
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
    stdout.write_text(completed.stdout, encoding="utf-8")
    stderr.write_text(completed.stderr, encoding="utf-8")
    finished = time.time()
    files = {
        name: {
            "path": relative(path),
            "sha256": sha256(path),
            "size_bytes": path.stat().st_size,
        }
        for name, path in {
            "runfile": runfile,
            "manifest": manifest,
            "wat": wat,
            "trace": trace,
            "stdout": stdout,
            "stderr": stderr,
        }.items()
        if path.is_file()
    }
    provenance = {
        "schema": "snow-surface-eb04w1-cell-provenance-v1",
        "lane_id": lane.lane_id,
        "multiplier": multiplier,
        "cell": "B",
        "returncode": completed.returncode,
        "started_unix_seconds": started,
        "completed_unix_seconds": finished,
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


def execute(workers: int) -> None:
    if not FREEZE.is_file():
        raise FileNotFoundError("freeze must exist before execution")
    if RECEIPT.exists() or any(RUNS.rglob("*.snow.jsonl")):
        raise RuntimeError("result-bearing execution already exists")
    freeze = json.loads(FREEZE.read_text(encoding="utf-8"))
    if freeze["tool_sha256"] != sha256(Path(__file__)):
        raise RuntimeError("execution tool changed after the prospective freeze")
    if freeze["binary_sha256"] != sha256(BINARY):
        raise RuntimeError("release binary changed after the prospective freeze")
    futures: dict[Any, tuple[str, float]] = {}
    results: dict[str, Any] = {}
    with ThreadPoolExecutor(max_workers=workers) as executor:
        for lane in selected_lanes():
            for multiplier in MULTIPLIERS:
                future = executor.submit(execute_cell, lane, multiplier)
                futures[future] = (lane.lane_id, multiplier)
        for future in as_completed(futures):
            lane_id, multiplier = futures[future]
            key = f"{lane_id}/{scale_id(multiplier)}"
            result = future.result()
            results[key] = result
            print(f"{key}: {'PASS' if result['returncode'] == 0 else 'FAIL'}")
    if len(results) != 32 or any(row["returncode"] != 0 for row in results.values()):
        raise RuntimeError("frozen 32-run population did not complete")
    write_json(
        RECEIPT,
        {
            "schema": "snow-surface-eb04w1-execution-receipt-v1",
            "evidence_role": ROLE,
            "binary_sha256": sha256(BINARY),
            "tool_sha256": sha256(Path(__file__)),
            "freeze_sha256": sha256(FREEZE),
            "working_directory": str(REPO),
            "execution_command": (
                ".venv/bin/python "
                "docs/work-packages/20260802-snow-surface-eb-04w1-"
                f"precipitation-scaling-calibration-001/tools/{Path(__file__).name} "
                f"--execute --workers {workers}"
            ),
            "result_count": len(results),
            "results": dict(sorted(results.items())),
        },
    )


def operator_metric(
    lane: Any, operator: str, retained_offset: float, cell: dict[str, Any], run_root: Path
) -> dict[str, Any]:
    stem = f"{lane.lane_id}-B"
    modeled = eb04r.legacy.observed_harness.load_modeled_wat(
        run_root / lane.lane_id / "B" / f"{stem}.wat.parquet"
    )
    observation_rows = eb04w.observation_rows(lane)
    pairs = eb04r.legacy.rubric.paired_snow_rows(observation_rows, modeled)
    if operator == "seasonal_ablation_meltout_date":
        offsets = eb04r.legacy.rubric.last_snow_date_by_water_year(pairs)
        observed_key = "observed_swe_m"
        modeled_key = "modeled_swe_m"
    else:
        suffix = "snow_depth_m" if operator == "seasonal_peak_depth_date" else "swe_m"
        observed_key = f"observed_{suffix}"
        modeled_key = f"modeled_{suffix}"
        offsets = eb04r.legacy.rubric.peak_date_by_water_year(
            pairs, observed_key, modeled_key
        )
    offset_values = [float(row["offset_days"]) for row in offsets]
    peak_ratios: list[float] = []
    prepeak: list[dict[str, Any]] = []
    daily = cell["daily"]
    for year, paired_year in eb04r.legacy.rubric.pairs_by_water_year(pairs).items():
        observed_peak = max(float(row[observed_key]) for row in paired_year)
        modeled_peak = max(float(row[modeled_key]) for row in paired_year)
        if observed_peak > 0.0:
            peak_ratios.append(modeled_peak / observed_peak)
        observed_swe_peak = max(
            paired_year, key=lambda row: float(row["observed_swe_m"])
        )
        observed_date = observed_swe_peak["date_obj"]
        phase_days = [
            row
            for row in daily
            if eb04w.water_year(dt.date.fromisoformat(row["date"])) == year
            and dt.date.fromisoformat(row["date"]) <= observed_date
        ]
        observed_swe = float(observed_swe_peak["observed_swe_m"])
        if not phase_days or observed_swe <= 0.0:
            continue
        initial = float(phase_days[0]["runtime_swe_before_m"])
        snow = sum(float(row["snowfall_swe_m"]) for row in phase_days)
        rain = sum(float(row["rain_retained_m"]) for row in phase_days)
        loss = sum(float(row["snowpack_swe_loss_m"]) for row in phase_days)
        sublimation = sum(float(row["sublimation_m"]) for row in phase_days)
        melt = sum(float(row["coe_melt_applied_m"]) for row in phase_days)
        final = float(phase_days[-1]["runtime_swe_after_m"])
        prepeak.append(
            {
                "water_year": year,
                "initial_swe_m": initial,
                "snowfall_input_m": snow,
                "rain_retained_m": rain,
                "snowpack_loss_m": loss,
                "sublimation_m": sublimation,
                "coe_melt_applied_m": melt,
                "observed_swe_peak_m": observed_swe,
                "modeled_swe_at_observed_peak_m": final,
                "effective_input_ratio": (initial + snow + rain) / observed_swe,
                "storage_ratio": final / observed_swe,
                "mass_closure_residual_m": initial + snow + rain - loss - sublimation - final,
            }
        )
    executed_offset = statistics.median(offset_values) if offset_values else None
    if executed_offset is None:
        raise RuntimeError(f"operator produced no offsets: {lane.lane_id}/{operator}")
    return {
        "operator": operator,
        "retained_baseline_offset_days": retained_offset,
        "executed_offset_days": executed_offset,
        "median_peak_ratio": statistics.median(peak_ratios),
        "median_effective_input_ratio": statistics.median(
            row["effective_input_ratio"] for row in prepeak
        ),
        "median_storage_ratio": statistics.median(row["storage_ratio"] for row in prepeak),
        "median_prepeak_loss_m": statistics.median(row["snowpack_loss_m"] for row in prepeak),
        "median_prepeak_sublimation_m": statistics.median(
            row["sublimation_m"] for row in prepeak
        ),
        "median_prepeak_coe_melt_m": statistics.median(
            row["coe_melt_applied_m"] for row in prepeak
        ),
        "maximum_mass_closure_m": max(
            abs(row["mass_closure_residual_m"]) for row in prepeak
        ),
        "water_years": prepeak,
    }


def primary_operator(lane_id: str, rows: list[dict[str, Any]]) -> dict[str, Any]:
    if lane_id == "snotel_niwot_co":
        return next(row for row in rows if row["operator"] == "seasonal_peak_swe_date")
    return rows[0]


def select_candidate(rows: list[dict[str, Any]]) -> dict[str, Any]:
    baseline = next(row for row in rows if row["multiplier"] == 1.0)
    base_mag = abs(math.log(baseline["primary_peak_ratio"]))
    base_time = baseline["chronology_abs_error_days"]
    joint = [
        row
        for row in rows
        if row["multiplier"] != 1.0
        and abs(math.log(row["primary_peak_ratio"])) < base_mag - 1.0e-12
        and row["chronology_abs_error_days"] < base_time - 1.0e-12
    ]
    if not joint:
        ratios = [row["primary_peak_ratio"] for row in rows]
        if max(ratios) < 0.9:
            classification = "GRID_BOUNDARY_LIMITED"
        elif any(abs(math.log(row["primary_peak_ratio"])) < base_mag for row in rows):
            classification = "CHRONOLOGY_LIMITED"
        else:
            classification = "NO_JOINT_IMPROVER"
        return {
            "classification": classification,
            "selected_multiplier": None,
            "joint_improver_count": 0,
            "baseline_primary_peak_ratio": baseline["primary_peak_ratio"],
            "baseline_chronology_abs_error_days": base_time,
        }
    ranked = sorted(
        joint,
        key=lambda row: (
            0 if 0.9 <= row["primary_peak_ratio"] <= 1.1 else 1,
            row["chronology_abs_error_days"],
            abs(row["multiplier"] - 1.0),
        ),
    )
    selected = ranked[0]
    boundary = selected["multiplier"] in (MULTIPLIERS[0], MULTIPLIERS[-1])
    return {
        "classification": "GRID_BOUNDARY" if boundary else "PRECIPITATION_RESPONSIVE",
        "selected_multiplier": selected["multiplier"],
        "joint_improver_count": len(joint),
        "selected_primary_peak_ratio": selected["primary_peak_ratio"],
        "selected_chronology_abs_error_days": selected["chronology_abs_error_days"],
        "baseline_primary_peak_ratio": baseline["primary_peak_ratio"],
        "baseline_chronology_abs_error_days": base_time,
    }


def baseline_replay(lane_id: str, metrics: list[dict[str, Any]]) -> dict[str, Any]:
    predecessor = json.loads(EB04W_RESULTS.read_text(encoding="utf-8"))
    expected = [row for row in predecessor["operators"] if row["lane_id"] == lane_id]
    actual = next(row for row in metrics if row["multiplier"] == 1.0)
    maximum = 0.0
    for expected_row in expected:
        actual_row = next(
            row for row in actual["operators"] if row["operator"] == expected_row["operator"]
        )
        expected_metrics = expected_row["cell_metrics"]["B"]
        maximum = max(
            maximum,
            abs(
                actual_row["executed_offset_days"]
                - expected_metrics["executed_frozen_operator_offset_days"]
            ),
            abs(
                actual_row["median_peak_ratio"]
                - expected_metrics["median_modeled_to_observed_peak_ratio"]
            ),
        )
    return {"maximum_operator_residual": maximum, "passes": maximum <= 1.0e-12}


def analyze() -> dict[str, Any]:
    if not RECEIPT.is_file():
        raise FileNotFoundError("execution receipt is missing")
    lanes = selected_lanes()
    frozen_operators = json.loads(FREEZE.read_text(encoding="utf-8"))["operators"]
    by_lane: dict[str, list[dict[str, Any]]] = {lane.lane_id: [] for lane in lanes}
    maximum_closure = 0.0
    for multiplier in MULTIPLIERS:
        candidate = scale_id(multiplier)
        run_root = RUNS / candidate
        eb04w.RUNS = run_root
        for lane in lanes:
            cell = eb04w.analyze_cell(lane, "B")
            operator_rows = [
                operator_metric(lane, operator, retained, cell, run_root)
                for lane_id, operator, retained in frozen_operators
                if lane_id == lane.lane_id
            ]
            primary = primary_operator(lane.lane_id, operator_rows)
            chronology_error = max(abs(row["executed_offset_days"]) for row in operator_rows)
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
            maximum_closure = max(maximum_closure, *closure_values)
            by_lane[lane.lane_id].append(
                {
                    "multiplier": multiplier,
                    "primary_peak_ratio": primary["median_peak_ratio"],
                    "chronology_abs_error_days": chronology_error,
                    "operators": operator_rows,
                    "maximum_closure_m": max(closure_values),
                    "totals": cell["totals"],
                    "daily": cell["daily"],
                }
            )
    lanes_result = {}
    for lane in lanes:
        metrics = by_lane[lane.lane_id]
        lanes_result[lane.lane_id] = {
            "label": LANE_LABELS[lane.lane_id],
            "candidates": metrics,
            "selection": select_candidate(metrics),
            "baseline_replay": baseline_replay(lane.lane_id, metrics),
        }
    result = {
        "schema": "snow-surface-eb04w1-precipitation-scaling-results-v1",
        "evidence_role": ROLE,
        "independent_validation": False,
        "freeze_sha256": sha256(FREEZE),
        "receipt_sha256": sha256(RECEIPT),
        "tool_sha256": sha256(Path(__file__)),
        "run_count": 32,
        "maximum_closure_m": maximum_closure,
        "lanes": lanes_result,
    }
    if maximum_closure > TOLERANCE_M:
        raise RuntimeError(f"diagnostic closure exceeded tolerance: {maximum_closure}")
    if not all(row["baseline_replay"]["passes"] for row in lanes_result.values()):
        raise RuntimeError("1.0 baseline operator replay differs from EB-04W")
    write_json(RESULTS, strip_daily(result))
    write_summary(result)
    write_figures(result, lanes)
    write_synthesis(result)
    return result


def strip_daily(result: dict[str, Any]) -> dict[str, Any]:
    serial = json.loads(json.dumps(result))
    for lane in serial["lanes"].values():
        for candidate in lane["candidates"]:
            candidate.pop("daily", None)
            for operator in candidate["operators"]:
                operator.pop("water_years", None)
    return serial


def write_summary(result: dict[str, Any]) -> None:
    with SUMMARY.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "lane_id",
                "multiplier",
                "primary_peak_ratio",
                "chronology_abs_error_days",
                "median_effective_input_ratio",
                "median_storage_ratio",
                "classification",
                "selected_multiplier",
            ]
        )
        for lane_id, lane in result["lanes"].items():
            selection = lane["selection"]
            for candidate in lane["candidates"]:
                primary = primary_operator(lane_id, candidate["operators"])
                writer.writerow(
                    [
                        lane_id,
                        candidate["multiplier"],
                        candidate["primary_peak_ratio"],
                        candidate["chronology_abs_error_days"],
                        primary["median_effective_input_ratio"],
                        primary["median_storage_ratio"],
                        selection["classification"],
                        selection["selected_multiplier"],
                    ]
                )


def save_figure(fig: Any, stem: str) -> None:
    FIGURES.mkdir(parents=True, exist_ok=True)
    fig.savefig(FIGURES / f"{stem}.svg", format="svg", bbox_inches="tight")


def write_sidecar(stem: str, title: str, caption: str, notice: str, methods: str) -> None:
    (FIGURES / f"{stem}.md").write_text(
        f"# {title}\n\n![{title}]({stem}.svg)\n\n## Caption\n\n{caption}\n\n"
        f"## What To Notice\n\n{notice}\n\n## Methods And Provenance\n\n{methods}\n\n"
        "## Interpretation Limits\n\nThe SNOTEL records are calibration data in EB-04W1. "
        "These curves are not independent validation and do not establish a transferable "
        "multiplier or authorize process/default promotion.\n",
        encoding="utf-8",
    )


def write_figures(result: dict[str, Any], lanes: list[Any]) -> None:
    import matplotlib

    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    colors = ["#0072B2", "#D55E00", "#009E73", "#CC79A7"]
    fig, axes = plt.subplots(2, 2, figsize=(11.5, 7.5), sharex=True)
    for lane, color in zip(lanes, colors):
        rows = result["lanes"][lane.lane_id]["candidates"]
        label = LANE_LABELS[lane.lane_id]
        x = [row["multiplier"] for row in rows]
        axes[0, 0].plot(x, [row["primary_peak_ratio"] for row in rows], "o-", color=color, label=label)
        axes[0, 1].plot(x, [row["chronology_abs_error_days"] for row in rows], "o-", color=color)
        axes[1, 0].plot(
            x,
            [primary_operator(lane.lane_id, row["operators"])["median_effective_input_ratio"] for row in rows],
            "o-",
            color=color,
        )
        axes[1, 1].plot(
            x,
            [primary_operator(lane.lane_id, row["operators"])["median_storage_ratio"] for row in rows],
            "o-",
            color=color,
        )
    axes[0, 0].axhspan(0.9, 1.1, color="#E6E6E6", zorder=0)
    axes[0, 0].axhline(1.0, color="black", linewidth=1)
    axes[1, 0].axhline(1.0, color="black", linewidth=1)
    axes[1, 1].axhline(1.0, color="black", linewidth=1)
    axes[0, 0].set_ylabel("Median modeled / observed peak SWE")
    axes[0, 1].set_ylabel("Worst absolute chronology offset (days)")
    axes[1, 0].set_ylabel("Median effective input / observed peak")
    axes[1, 1].set_ylabel("Median retained SWE / observed peak")
    for axis in axes[1]:
        axis.set_xlabel("Total-precipitation multiplier")
    for axis in axes.flat:
        axis.grid(True, alpha=0.25)
    axes[0, 0].legend(loc="best", frameon=True)
    fig.suptitle("Snow response to uniform total-precipitation scaling")
    fig.tight_layout()
    save_figure(fig, "eb04w1-precipitation-response-curves")
    plt.close(fig)
    write_sidecar(
        "eb04w1-precipitation-response-curves",
        "Precipitation-Scaling Response Curves",
        "Four response measures are shown across the prospectively frozen 0.8-1.5 grid for each open mountain lane.",
        "A useful multiplier must move peak magnitude toward 1.0 while also reducing chronology error. Input response without storage or timing response indicates compensation rather than joint correction.",
        "Every point is a real baseline-B direct-production run. Daily total precipitation alone is scaled; all other climate tokens and model inputs are unchanged and audited.",
    )

    fig, axes = plt.subplots(2, 2, figsize=(11.5, 8.0))
    for axis, lane in zip(axes.flat, lanes):
        lane_result = result["lanes"][lane.lane_id]
        selected = lane_result["selection"]["selected_multiplier"]
        chosen = selected if selected is not None else 1.0
        observed_grouped: dict[int, list[float]] = {}
        for row in eb04w.observation_rows(lane):
            try:
                date = dt.date.fromisoformat(row["date"])
                swe_m = float(row["observed_swe_mm"]) / 1000.0
            except (KeyError, TypeError, ValueError):
                continue
            start = dt.date(date.year if date.month >= 10 else date.year - 1, 10, 1)
            observed_grouped.setdefault((date - start).days, []).append(swe_m)
        observed_x = sorted(observed_grouped)
        axis.plot(
            observed_x,
            [statistics.median(observed_grouped[index]) for index in observed_x],
            color="black",
            linewidth=2.2,
            label="Observed",
        )
        for multiplier, color, label in ((1.0, "#666666", "Baseline 1.0"), (chosen, "#D55E00", f"Adjudicated {chosen:.1f}")):
            row = next(item for item in lane_result["candidates"] if item["multiplier"] == multiplier)
            daily = row["daily"]
            grouped: dict[int, list[float]] = {}
            for day in daily:
                date = dt.date.fromisoformat(day["date"])
                start = dt.date(date.year if date.month >= 10 else date.year - 1, 10, 1)
                index = (date - start).days
                grouped.setdefault(index, []).append(float(day["swe_m"]))
            x = sorted(grouped)
            y = [statistics.median(grouped[index]) for index in x]
            axis.plot(x, y, color=color, linewidth=1.8, label=label)
        axis.set_title(LANE_LABELS[lane.lane_id])
        axis.set_xlabel("Day of water year")
        axis.set_ylabel("Median modeled SWE (m)")
        axis.grid(True, alpha=0.25)
        axis.legend(loc="best")
    fig.suptitle("Seasonal SWE trajectories: baseline and adjudicated multiplier")
    fig.tight_layout()
    save_figure(fig, "eb04w1-seasonal-swe-trajectories")
    plt.close(fig)
    write_sidecar(
        "eb04w1-seasonal-swe-trajectories",
        "Seasonal SWE Trajectories Under Precipitation Scaling",
        "Median modeled daily SWE by day of water year compares the 1.0 baseline with the multiplier selected by the frozen rule; lanes without a joint improver retain 1.0.",
        "Look for both a higher seasonal mass trajectory and a later, more realistic decline. A taller curve with unchanged early disappearance is not a joint magnitude/chronology solution.",
        "Daily modeled SWE is grouped by water-year day across the full calibration record. The adjudicated multiplier is selected from the frozen objective, not chosen for visual appearance.",
    )

    labels = [LANE_LABELS[lane.lane_id] for lane in lanes]
    baseline_input: list[float] = []
    selected_input: list[float] = []
    baseline_storage: list[float] = []
    selected_storage: list[float] = []
    for lane in lanes:
        lane_result = result["lanes"][lane.lane_id]
        selected = lane_result["selection"]["selected_multiplier"] or 1.0
        baseline = next(row for row in lane_result["candidates"] if row["multiplier"] == 1.0)
        chosen = next(row for row in lane_result["candidates"] if row["multiplier"] == selected)
        b = primary_operator(lane.lane_id, baseline["operators"])
        s = primary_operator(lane.lane_id, chosen["operators"])
        baseline_input.append(b["median_effective_input_ratio"])
        selected_input.append(s["median_effective_input_ratio"])
        baseline_storage.append(b["median_storage_ratio"])
        selected_storage.append(s["median_storage_ratio"])
    positions = list(range(len(labels)))
    width = 0.18
    fig, axis = plt.subplots(figsize=(11.0, 5.5))
    axis.bar([x - 1.5 * width for x in positions], baseline_input, width, label="Baseline input", color="#999999")
    axis.bar([x - 0.5 * width for x in positions], selected_input, width, label="Adjudicated input", color="#56B4E9")
    axis.bar([x + 0.5 * width for x in positions], baseline_storage, width, label="Baseline storage", color="#E69F00")
    axis.bar([x + 1.5 * width for x in positions], selected_storage, width, label="Adjudicated storage", color="#009E73")
    axis.axhline(1.0, color="black", linewidth=1)
    axis.set_xticks(positions, labels)
    axis.set_ylabel("Ratio to observed peak SWE")
    axis.set_title("Effective input and retained storage response")
    axis.grid(True, axis="y", alpha=0.25)
    axis.legend(loc="best")
    fig.tight_layout()
    save_figure(fig, "eb04w1-input-storage-adjudication")
    plt.close(fig)
    write_sidecar(
        "eb04w1-input-storage-adjudication",
        "Effective Input And Retained Storage Adjudication",
        "Baseline and adjudicated-multiplier median effective input and retained SWE are compared with observed peak SWE.",
        "If effective input reaches or exceeds 1.0 while storage remains low, pre-peak loss still controls the discrepancy. Lanes with no joint improver retain the baseline bars.",
        "Effective input is initial SWE plus snowfall SWE plus retained rain through the observed SWE-peak date. Retained storage is modeled SWE on that date. Both are reconstructed from EB-04W diagnostics.",
    )


def write_synthesis(result: dict[str, Any]) -> None:
    lines = [
        "# EB-04W1 Scientific Synthesis",
        "",
        "Evidence mode: **Ran + Inference**. The observations are `CALIBRATION`; no independent-validation or promotion claim is made.",
        "",
        "## Lane Results",
        "",
        "| Lane | Baseline peak ratio | Baseline chronology error (d) | Selected multiplier | Selected peak ratio | Selected chronology error (d) | Classification |",
        "|---|---:|---:|---:|---:|---:|---|",
    ]
    for lane in result["lanes"].values():
        selection = lane["selection"]
        selected = selection.get("selected_multiplier")
        lines.append(
            f"| {lane['label']} | {selection.get('baseline_primary_peak_ratio', '—')} | "
            f"{selection.get('baseline_chronology_abs_error_days', '—')} | "
            f"{selected if selected is not None else '—'} | "
            f"{selection.get('selected_primary_peak_ratio', '—')} | "
            f"{selection.get('selected_chronology_abs_error_days', '—')} | "
            f"{selection['classification']} |"
        )
    lines.extend(
        [
            "",
            "## Closure And Claim Boundary",
            "",
            f"All 32 runs completed. Maximum reconstructed diagnostic closure was `{result['maximum_closure_m']:.3e} m`. Every 1.0 operator replay matches EB-04W within `1e-12`.",
            "",
            "A selected multiplier is a calibration result for its source fixture and SNOTEL record only. It is not an independent prediction, a transferable regional factor, or evidence that precipitation forcing is the unique physical cause. Magnitude-only improvement is retained as compensation evidence rather than success.",
            "",
        ]
    )
    (ARTIFACTS / "scientific-synthesis.md").write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--self-check", action="store_true")
    group.add_argument("--freeze", action="store_true")
    group.add_argument("--execute", action="store_true")
    group.add_argument("--analysis-only", action="store_true")
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    FIGURES.mkdir(parents=True, exist_ok=True)
    if args.self_check:
        self_check()
        print("EB-04W1 forcing transformer self-check: PASS")
        return 0
    if args.freeze:
        prepare_freeze()
        print(f"EB-04W1 prospective freeze: PASS ({sha256(FREEZE)})")
        return 0
    if args.execute:
        execute(args.workers)
    result = analyze()
    print(
        "EB-04W1 analysis: PASS "
        f"({result['run_count']} runs; max closure {result['maximum_closure_m']:.3e} m)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
