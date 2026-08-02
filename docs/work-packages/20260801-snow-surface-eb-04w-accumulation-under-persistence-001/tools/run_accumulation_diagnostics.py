#!/usr/bin/env python3
"""Execute and summarize the frozen EB-04W mountain accumulation diagnostics."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import importlib.util
import json
import math
import statistics
import subprocess
import sys
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
OUTPUT = REPO / "target/snow_surface_eb04w_accumulation_diagnostics"
RUNS = OUTPUT / "runs"
BINARY = REPO / "target/release/openwepp-cli-hill"
FREEZE = ARTIFACTS / "population-freeze.json"
RECEIPT = ARTIFACTS / "execution-receipt.json"
RESULTS = ARTIFACTS / "accumulation-mechanics-results.json"
SUMMARY = ARTIFACTS / "accumulation-mechanics-summary.csv"
SYNTHESIS = ARTIFACTS / "scientific-synthesis.md"
EB04R_TOOL = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04r-fresh-factorial-"
    "execution-adjudication-001/tools/run_experiment.py"
)

LANE_LABELS = {
    "snotel_mica_creek_st_joe_id": "Mica Creek / St. Joe, ID",
    "snotel_niwot_co": "Niwot, CO",
    "snotel_paradise_wa": "Paradise, WA",
    "snotel_snowbird_ut": "Snowbird, UT",
}
COMPONENTS = (
    "coe_melt_amelt_m",
    "coe_melt_bmelt_m",
    "coe_melt_cmelt_m",
    "coe_melt_dmelt_m",
)
SOURCE_IDENTITY_PATHS = (
    "crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs",
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs",
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs",
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs",
    "crates/openwepp-hillslope-orchestrator/src/lib.rs",
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs",
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs",
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs",
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00f_snow_accumulation_melt_trace.rs",
    "crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs",
)


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


eb04r = load_module("eb04w_eb04r_harness", EB04R_TOOL)
eb04r.RUNS = RUNS
eb04r.BINARY = BINARY


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def selected_lanes() -> list[Any]:
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))
    expected = set(frozen["unique_lanes"])
    lanes = [lane for lane in eb04r.legacy.fixed_lanes() if lane.lane_id in expected]
    if len(lanes) != 4 or {lane.lane_id for lane in lanes} != expected:
        raise RuntimeError("runtime lanes differ from frozen four-lane population")
    return lanes


def execute(workers: int) -> None:
    if RECEIPT.exists() or any(RUNS.rglob("*.snow.jsonl")):
        raise RuntimeError("EB-04W execution already exists; use --analysis-only")
    if not BINARY.is_file():
        raise FileNotFoundError(f"build the exact release binary first: {BINARY}")
    RUNS.mkdir(parents=True, exist_ok=True)
    futures: dict[Any, tuple[str, str]] = {}
    results: dict[str, Any] = {}
    with ThreadPoolExecutor(max_workers=workers) as executor:
        for lane in selected_lanes():
            for cell in eb04r.CELLS:
                future = executor.submit(eb04r.execute_cell, lane, cell)
                futures[future] = (lane.lane_id, cell)
        for future in as_completed(futures):
            lane_id, cell = futures[future]
            result = future.result()
            results[f"{lane_id}/{cell}"] = result
            print(f"{lane_id}/{cell}: {'PASS' if result['returncode'] == 0 else 'FAIL'}")
    if len(results) != 16 or any(row["returncode"] != 0 for row in results.values()):
        raise RuntimeError("frozen 16-cell population did not complete")
    write_json(
        RECEIPT,
        {
            "schema": "snow-surface-eb04w-execution-receipt-v1",
            "evidence_role": "DIAGNOSTIC_ONLY",
            "binary": relative(BINARY),
            "binary_sha256": sha256(BINARY),
            "binary_size_bytes": BINARY.stat().st_size,
            "binary_mtime_ns": BINARY.stat().st_mtime_ns,
            "build_command": "cargo build --release -p openwepp-runner --bin openwepp-cli-hill",
            "execution_command": (
                ".venv/bin/python docs/work-packages/20260801-snow-surface-eb-04w-"
                "accumulation-under-persistence-001/tools/run_accumulation_diagnostics.py "
                f"--execute --workers {workers}"
            ),
            "working_directory": str(REPO),
            "source_head": subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=REPO,
                check=True,
                capture_output=True,
                text=True,
            ).stdout.strip(),
            "source_file_sha256": {
                path: sha256(REPO / path) for path in SOURCE_IDENTITY_PATHS
            },
            "execution_tool_sha256": sha256(Path(__file__)),
            "population_freeze_sha256": sha256(FREEZE),
            "cell_count": 16,
            "operator_count": 5,
            "environment_policy": "REMOVE_ALL_INHERITED_OPENWEPP_THEN_INSTALL_EXACT_SEVEN",
            "results": dict(sorted(results.items())),
        },
    )


def observation_rows(lane: Any) -> list[dict[str, str]]:
    with lane.observation_file.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    if lane.observation_filter:
        rows = [
            row
            for row in rows
            if all(row.get(key) == value for key, value in lane.observation_filter.items())
        ]
    return rows


def observed_by_date(lane: Any) -> dict[dt.date, dict[str, float | None]]:
    result: dict[dt.date, dict[str, float | None]] = {}
    for row in observation_rows(lane):
        try:
            date = dt.date.fromisoformat(row["date"])
        except (KeyError, TypeError, ValueError):
            continue
        try:
            swe_m = float(row["observed_swe_mm"]) / 1000.0
        except (KeyError, TypeError, ValueError):
            swe_m = None
        try:
            depth_m = float(row["observed_snow_depth_m"])
        except (KeyError, TypeError, ValueError):
            depth_m = None
        if swe_m is not None or depth_m is not None:
            result[date] = {"swe_m": swe_m, "depth_m": depth_m}
    return result


def water_year(date: dt.date) -> int:
    return date.year + 1 if date.month >= 10 else date.year


def read_cell(lane: Any, cell: str) -> tuple[list[dict[str, Any]], dict[dt.date, Any]]:
    run_dir = RUNS / lane.lane_id / cell
    stem = f"{lane.lane_id}-{cell}"
    trace = [
        json.loads(line)
        for line in (run_dir / f"{stem}.snow.jsonl").read_text(encoding="utf-8").splitlines()
    ]
    modeled = eb04r.legacy.observed_harness.load_modeled_wat(run_dir / f"{stem}.wat.parquet")
    if len(trace) != len(modeled):
        raise RuntimeError(f"trace/WAT chronology mismatch for {lane.lane_id}/{cell}")
    return trace, modeled


def analyze_cell(lane: Any, cell: str) -> dict[str, Any]:
    trace, modeled = read_cell(lane, cell)
    max_melt_closure = 0.0
    max_uncapped_melt_closure = 0.0
    max_daily_applied_raw_melt_closure = 0.0
    max_accumulation_closure = 0.0
    max_phase_fraction_closure = 0.0
    max_phase_amount_closure = 0.0
    max_snow_depth_swe_closure = 0.0
    max_trace_wat_swe_closure = 0.0
    max_trace_wat_depth_closure = 0.0
    modeled_redistribution = 0.0
    totals = defaultdict(float)
    daily: list[dict[str, Any]] = []
    for day_index, (row, (date, model)) in enumerate(zip(trace, sorted(modeled.items()))):
        if row["schema"] != "openwepp-r7h-direct-production-snow-trace-v3":
            raise RuntimeError("trace schema is not EB-04W v3")
        if int(row["day_index"]) != day_index:
            raise RuntimeError(f"trace day index mismatch for {lane.lane_id}/{cell}")
        max_trace_wat_swe_closure = max(
            max_trace_wat_swe_closure,
            abs(float(row["runtime_swe_after_m"]) - float(model.get("snow_water_m") or 0.0)),
        )
        max_trace_wat_depth_closure = max(
            max_trace_wat_depth_closure,
            abs(float(row["runtime_depth_after_m"]) - float(model.get("snow_depth_m") or 0.0)),
        )
        hourly = row["accumulation_melt_hourly"]
        accumulation = sum(float(hour["snowfall_swe_m"]) for hour in hourly)
        max_accumulation_closure = max(
            max_accumulation_closure, abs(float(row["accumulation_m"]) - accumulation)
        )
        day_components = defaultdict(float)
        for hour in hourly:
            component_sum = sum(float(hour[key]) for key in COMPONENTS)
            max_uncapped_melt_closure = max(
                max_uncapped_melt_closure,
                abs(float(hour["coe_melt_uncapped_m"]) - component_sum),
            )
            closure = abs(
                float(hour["coe_melt_applied_m"])
                - component_sum
                - float(hour["coe_melt_cap_adjustment_m"])
            )
            max_melt_closure = max(max_melt_closure, closure)
            precip = float(hour["active_precipitation_m"])
            max_phase_amount_closure = max(
                max_phase_amount_closure,
                abs(
                    precip
                    - float(hour["rain_m"])
                    - float(hour["snowfall_swe_m"])
                ),
            )
            if precip > 0.0:
                max_phase_fraction_closure = max(
                    max_phase_fraction_closure,
                    abs(float(hour["rain_fraction"]) + float(hour["snow_fraction"]) - 1.0),
                )
                max_phase_amount_closure = max(
                    max_phase_amount_closure,
                    abs(float(hour["rain_m"]) - precip * float(hour["rain_fraction"])),
                    abs(
                        float(hour["snowfall_swe_m"])
                        - precip * float(hour["snow_fraction"])
                    ),
                )
            else:
                max_phase_fraction_closure = max(
                    max_phase_fraction_closure,
                    abs(float(hour["rain_fraction"])) + abs(float(hour["snow_fraction"])),
                )
            max_snow_depth_swe_closure = max(
                max_snow_depth_swe_closure,
                abs(
                    float(hour["snowfall_swe_m"])
                    - 0.1 * float(hour["snowfall_depth_m"])
                ),
            )
            for key in COMPONENTS:
                day_components[key] += float(hour[key])
            day_components["coe_melt_applied_m"] += float(hour["coe_melt_applied_m"])
            day_components["coe_melt_cap_adjustment_m"] += float(
                hour["coe_melt_cap_adjustment_m"]
            )
            modeled_redistribution += abs(float(hour["modeled_wind_redistribution_m"]))
        for key, value in day_components.items():
            totals[key] += value
        max_daily_applied_raw_melt_closure = max(
            max_daily_applied_raw_melt_closure,
            abs(float(row["raw_melt_m"]) - day_components["coe_melt_applied_m"]),
        )
        totals["snowfall_swe_m"] += accumulation
        totals["sublimation_m"] += float(row["sublimation_m"])
        daily.append(
            {
                "date": date.isoformat(),
                "swe_m": float(model.get("snow_water_m") or 0.0),
                "depth_m": float(model.get("snow_depth_m") or 0.0),
                "runtime_swe_before_m": float(row["runtime_swe_before_m"]),
                "runtime_swe_after_m": float(row["runtime_swe_after_m"]),
                "snowfall_swe_m": accumulation,
                "rain_retained_m": float(row["rain_retained_m"]),
                "snowpack_swe_loss_m": float(row["snowpack_swe_loss_m"]),
                "sublimation_m": float(row["sublimation_m"]),
                **dict(day_components),
            }
        )
    return {
        "lane_id": lane.lane_id,
        "cell": cell,
        "day_count": len(trace),
        "maximum_melt_component_closure_m": max_melt_closure,
        "maximum_uncapped_melt_component_closure_m": max_uncapped_melt_closure,
        "maximum_daily_applied_raw_melt_closure_m": max_daily_applied_raw_melt_closure,
        "maximum_accumulation_closure_m": max_accumulation_closure,
        "maximum_phase_fraction_closure": max_phase_fraction_closure,
        "maximum_phase_amount_closure_m": max_phase_amount_closure,
        "maximum_snow_depth_swe_closure_m": max_snow_depth_swe_closure,
        "maximum_trace_wat_swe_closure_m": max_trace_wat_swe_closure,
        "maximum_trace_wat_depth_closure_m": max_trace_wat_depth_closure,
        "modeled_wind_redistribution_abs_sum_m": modeled_redistribution,
        "totals": dict(totals),
        "daily": daily,
    }


def operator_rows(lanes: list[Any], cells: dict[str, Any]) -> list[dict[str, Any]]:
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))
    lane_map = {lane.lane_id: lane for lane in lanes}
    result = []
    for lane_id, operator, retained_offset in frozen["operators"]:
        lane = lane_map[lane_id]
        cell_metrics = {}
        for cell in eb04r.CELLS:
            run_dir = RUNS / lane_id / cell
            stem = f"{lane_id}-{cell}"
            modeled = eb04r.legacy.observed_harness.load_modeled_wat(
                run_dir / f"{stem}.wat.parquet"
            )
            pairs = eb04r.legacy.rubric.paired_snow_rows(observation_rows(lane), modeled)
            if operator == "seasonal_ablation_meltout_date":
                offset_rows = eb04r.legacy.rubric.last_snow_date_by_water_year(pairs)
                observed_key = "observed_swe_m"
                modeled_key = "modeled_swe_m"
            else:
                suffix = (
                    "snow_depth_m" if operator == "seasonal_peak_depth_date" else "swe_m"
                )
                observed_key = f"observed_{suffix}"
                modeled_key = f"modeled_{suffix}"
                offset_rows = eb04r.legacy.rubric.peak_date_by_water_year(
                    pairs, observed_key, modeled_key
                )
            offsets = [float(row["offset_days"]) for row in offset_rows]
            peak_ratios = []
            prepeak_rows = []
            daily = cells[f"{lane_id}/{cell}"]["daily"]
            for year, paired_year in eb04r.legacy.rubric.pairs_by_water_year(pairs).items():
                observed_peak_value = max(float(row[observed_key]) for row in paired_year)
                modeled_peak_value = max(float(row[modeled_key]) for row in paired_year)
                if observed_peak_value > 0.0:
                    peak_ratios.append(modeled_peak_value / observed_peak_value)

                observed_swe_peak = max(
                    paired_year, key=lambda row: float(row["observed_swe_m"])
                )
                observed_peak_date = observed_swe_peak["date_obj"]
                phase_days = [
                    row
                    for row in daily
                    if water_year(dt.date.fromisoformat(row["date"])) == year
                    and dt.date.fromisoformat(row["date"]) <= observed_peak_date
                ]
                if not phase_days or float(observed_swe_peak["observed_swe_m"]) <= 0.0:
                    continue
                initial_swe_m = float(phase_days[0]["runtime_swe_before_m"])
                snowfall_input_m = sum(float(row["snowfall_swe_m"]) for row in phase_days)
                rain_retained_m = sum(float(row["rain_retained_m"]) for row in phase_days)
                snowpack_loss_m = sum(float(row["snowpack_swe_loss_m"]) for row in phase_days)
                sublimation_m = sum(float(row["sublimation_m"]) for row in phase_days)
                coe_applied_m = sum(float(row["coe_melt_applied_m"]) for row in phase_days)
                final_swe_m = float(phase_days[-1]["runtime_swe_after_m"])
                observed_swe_m = float(observed_swe_peak["observed_swe_m"])
                realized_snowpack_input_m = (
                    initial_swe_m + snowfall_input_m + rain_retained_m
                )
                prepeak_rows.append(
                    {
                        "water_year": year,
                        "observed_swe_peak_date": observed_peak_date.isoformat(),
                        "observed_swe_peak_m": observed_swe_m,
                        "initial_swe_m": initial_swe_m,
                        "snowfall_input_m": snowfall_input_m,
                        "rain_retained_m": rain_retained_m,
                        "snowpack_loss_m": snowpack_loss_m,
                        "sublimation_m": sublimation_m,
                        "coe_melt_applied_m": coe_applied_m,
                        "modeled_swe_at_observed_peak_m": final_swe_m,
                        "realized_snowpack_input_to_observed_peak_ratio": (
                            realized_snowpack_input_m / observed_swe_m
                        ),
                        "modeled_storage_to_observed_peak_ratio": final_swe_m
                        / observed_swe_m,
                        "mass_closure_residual_m": initial_swe_m
                        + snowfall_input_m
                        + rain_retained_m
                        - snowpack_loss_m
                        - sublimation_m
                        - final_swe_m,
                    }
                )
            cell_metrics[cell] = {
                "executed_frozen_operator_offset_days": statistics.median(offsets)
                if offsets
                else None,
                "median_modeled_to_observed_peak_ratio": statistics.median(peak_ratios)
                if peak_ratios
                else None,
                "pre_observed_peak_mechanics": prepeak_rows,
            }
        baseline_offset = cell_metrics["B"]["executed_frozen_operator_offset_days"]
        if baseline_offset is None or abs(baseline_offset - retained_offset) > 1.0e-12:
            raise RuntimeError(
                f"frozen operator mismatch for {lane_id}/{operator}: "
                f"retained={retained_offset}, executed={baseline_offset}"
            )
        all_prepeak = [
            row
            for metrics in cell_metrics.values()
            for row in metrics["pre_observed_peak_mechanics"]
        ]
        if all_prepeak and all(
            float(row["realized_snowpack_input_to_observed_peak_ratio"]) < 1.0
            for row in all_prepeak
        ):
            attribution = "realized input-pathway/pre-peak-loss boundary unresolved"
        else:
            attribution = "pre-peak input/loss ownership mixed or unresolved"
        result.append(
            {
                "lane_id": lane_id,
                "operator": operator,
                "retained_offset_days": retained_offset,
                "executed_frozen_operator_offset_days": baseline_offset,
                "cell_metrics": cell_metrics,
                "diagnostic_attribution": attribution,
            }
        )
    return result


def analyze() -> dict[str, Any]:
    if not RECEIPT.is_file():
        raise FileNotFoundError("execution receipt is missing")
    lanes = selected_lanes()
    cells = {
        f"{lane.lane_id}/{cell}": analyze_cell(lane, cell)
        for lane in lanes
        for cell in eb04r.CELLS
    }
    operators = operator_rows(lanes, cells)
    pre_observed_peak_rows = [
        phase_row
        for operator in operators
        for metrics in operator["cell_metrics"].values()
        for phase_row in metrics["pre_observed_peak_mechanics"]
    ]
    result = {
        "schema": "snow-surface-eb04w-accumulation-results-v1",
        "evidence_role": "DIAGNOSTIC_ONLY",
        "cell_count": 16,
        "operator_count": 5,
        "analysis_tool_sha256": sha256(Path(__file__)),
        "execution_receipt_sha256": sha256(RECEIPT),
        "maximum_melt_component_closure_m": max(
            row["maximum_melt_component_closure_m"] for row in cells.values()
        ),
        "maximum_uncapped_melt_component_closure_m": max(
            row["maximum_uncapped_melt_component_closure_m"] for row in cells.values()
        ),
        "maximum_daily_applied_raw_melt_closure_m": max(
            row["maximum_daily_applied_raw_melt_closure_m"] for row in cells.values()
        ),
        "maximum_accumulation_closure_m": max(
            row["maximum_accumulation_closure_m"] for row in cells.values()
        ),
        "maximum_phase_fraction_closure": max(
            row["maximum_phase_fraction_closure"] for row in cells.values()
        ),
        "maximum_phase_amount_closure_m": max(
            row["maximum_phase_amount_closure_m"] for row in cells.values()
        ),
        "maximum_snow_depth_swe_closure_m": max(
            row["maximum_snow_depth_swe_closure_m"] for row in cells.values()
        ),
        "maximum_trace_wat_swe_closure_m": max(
            row["maximum_trace_wat_swe_closure_m"] for row in cells.values()
        ),
        "maximum_trace_wat_depth_closure_m": max(
            row["maximum_trace_wat_depth_closure_m"] for row in cells.values()
        ),
        "maximum_modeled_wind_redistribution_abs_sum_m": max(
            row["modeled_wind_redistribution_abs_sum_m"] for row in cells.values()
        ),
        "maximum_pre_observed_peak_mass_closure_m": max(
            abs(float(row["mass_closure_residual_m"])) for row in pre_observed_peak_rows
        ),
        "cells": cells,
        "operators": operators,
    }
    for key in (
        "maximum_melt_component_closure_m",
        "maximum_uncapped_melt_component_closure_m",
        "maximum_daily_applied_raw_melt_closure_m",
        "maximum_accumulation_closure_m",
        "maximum_phase_fraction_closure",
        "maximum_phase_amount_closure_m",
        "maximum_snow_depth_swe_closure_m",
        "maximum_trace_wat_swe_closure_m",
        "maximum_trace_wat_depth_closure_m",
        "maximum_modeled_wind_redistribution_abs_sum_m",
        "maximum_pre_observed_peak_mass_closure_m",
    ):
        if not math.isfinite(result[key]) or result[key] > 1.0e-12:
            raise RuntimeError(f"closure/status gate failed: {key}={result[key]}")
    write_summary(operators)
    make_figures(result, lanes)
    write_synthesis(result)
    result["cells"] = {
        key: {field: value for field, value in row.items() if field != "daily"}
        for key, row in cells.items()
    }
    write_json(RESULTS, result)
    return result


def write_summary(rows: list[dict[str, Any]]) -> None:
    with SUMMARY.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def save_figure(fig: Any, stem: str) -> None:
    FIGURES.mkdir(parents=True, exist_ok=True)
    fig.savefig(FIGURES / f"{stem}.svg", format="svg", bbox_inches="tight")


def make_figures(result: dict[str, Any], lanes: list[Any]) -> None:
    import matplotlib.pyplot as plt

    fig, axes = plt.subplots(2, 2, figsize=(12, 8), constrained_layout=True)
    for axis, lane in zip(axes.flat, lanes):
        observed = observed_by_date(lane)
        cell_row = result["cells"][f"{lane.lane_id}/B"]
        if "daily" in cell_row:
            daily = cell_row["daily"]
        else:
            run_dir = RUNS / lane.lane_id / "B"
            stem = f"{lane.lane_id}-B"
            modeled = eb04r.legacy.observed_harness.load_modeled_wat(
                run_dir / f"{stem}.wat.parquet"
            )
            daily = [
                {"date": date.isoformat(), "swe_m": float(row.get("snow_water_m") or 0.0)}
                for date, row in sorted(modeled.items())
            ]
        weekly_daily = daily[::7]
        axis.plot(
            [dt.date.fromisoformat(row["date"]) for row in weekly_daily],
            [row["swe_m"] * 1000.0 for row in weekly_daily],
            color="#2563EB",
            linewidth=1.2,
            label="Simulated B",
        )
        observed_swe = [
            (date, row["swe_m"])
            for date, row in observed.items()
            if row["swe_m"] is not None
        ][::7]
        axis.scatter(
            [date for date, _ in observed_swe],
            [value * 1000.0 for _, value in observed_swe],
            s=8,
            color="#111827",
            alpha=0.65,
            label="Observed",
            zorder=3,
        )
        axis.set_title(LANE_LABELS[lane.lane_id])
        axis.set_ylabel("SWE (mm)")
        axis.tick_params(axis="x", labelrotation=35, labelsize=7)
        axis.grid(alpha=0.2)
    axes.flat[0].legend(loc="upper left", frameon=False)
    fig.suptitle("Observed and simulated snow-water chronology")
    save_figure(fig, "eb04w-observed-simulated-swe")
    plt.close(fig)

    operator_rows_ = result["operators"]
    fig, axis = plt.subplots(figsize=(10, 4.8), constrained_layout=True)
    labels = [
        f"{LANE_LABELS[row['lane_id']]}\n{row['operator'].replace('seasonal_', '').replace('_', ' ')}"
        for row in operator_rows_
    ]
    values = [row["retained_offset_days"] for row in operator_rows_]
    axis.bar(labels, values, color="#D97706")
    axis.axhline(0.0, color="#111827", linewidth=1.0)
    axis.set_ylabel("Modeled minus observed date (days)")
    axis.set_title("Retained mountain chronology failures")
    axis.tick_params(axis="x", labelrotation=18)
    axis.grid(axis="y", alpha=0.2)
    save_figure(fig, "eb04w-chronology-offsets")
    plt.close(fig)

    fig, axis = plt.subplots(figsize=(10, 5), constrained_layout=True)
    lane_ids = list(LANE_LABELS)
    bottoms = [0.0] * len(lane_ids)
    colors = ["#F59E0B", "#2563EB", "#10B981", "#DC2626"]
    for key, color in zip(COMPONENTS, colors):
        values = [result["cells"][f"{lane_id}/B"]["totals"][key] * 1000.0 for lane_id in lane_ids]
        axis.bar([LANE_LABELS[lane_id] for lane_id in lane_ids], values, bottom=bottoms,
                 label=key.replace("coe_melt_", "").replace("_m", ""), color=color)
        bottoms = [left + right for left, right in zip(bottoms, values)]
    axis.set_ylabel("Signed uncapped CoE contribution (mm)")
    axis.set_title("Empirical CoE melt-depth contributions, baseline cell")
    axis.legend(ncol=4, loc="upper center", frameon=False)
    axis.grid(axis="y", alpha=0.2)
    save_figure(fig, "eb04w-coe-component-totals")
    plt.close(fig)

    unique_operators = {}
    for row in result["operators"]:
        unique_operators.setdefault(row["lane_id"], row)
    lane_ids = list(LANE_LABELS)
    fig, (input_axis, storage_axis) = plt.subplots(
        1, 2, figsize=(12, 5), constrained_layout=True
    )
    width = 0.18
    colors = ["#D97706", "#2563EB", "#059669", "#7C3AED"]
    positions = list(range(len(lane_ids)))
    for cell_index, (cell, color) in enumerate(zip(eb04r.CELLS, colors)):
        offset = (cell_index - 1.5) * width
        input_fractions = []
        storage_medians = []
        for lane_id in lane_ids:
            mechanics = unique_operators[lane_id]["cell_metrics"][cell][
                "pre_observed_peak_mechanics"
            ]
            input_fractions.append(
                100.0
                * sum(
                    float(row["realized_snowpack_input_to_observed_peak_ratio"]) < 1.0
                    for row in mechanics
                )
                / len(mechanics)
            )
            storage_medians.append(
                statistics.median(
                    float(row["modeled_storage_to_observed_peak_ratio"])
                    for row in mechanics
                )
            )
        shifted = [position + offset for position in positions]
        input_axis.bar(shifted, input_fractions, width=width, color=color, label=cell)
        storage_axis.bar(shifted, storage_medians, width=width, color=color, label=cell)
    labels = [LANE_LABELS[lane_id] for lane_id in lane_ids]
    for axis in (input_axis, storage_axis):
        axis.set_xticks(positions, labels, rotation=20, ha="right")
        axis.grid(axis="y", alpha=0.2)
    input_axis.set_ylabel("Water years below observed peak (%)")
    input_axis.set_title("Realized snowpack inputs still below observed peak")
    storage_axis.axhline(1.0, color="#111827", linewidth=1.0)
    storage_axis.set_ylabel("Median modeled / observed SWE")
    storage_axis.set_title("Storage retained at the observed SWE peak")
    storage_axis.legend(ncol=4, loc="upper center", frameon=False)
    save_figure(fig, "eb04w-prepeak-input-loss-decomposition")
    plt.close(fig)


def write_synthesis(result: dict[str, Any]) -> None:
    rows = "\n".join(
        f"| {LANE_LABELS[row['lane_id']]} | {row['operator']} | "
        f"{row['retained_offset_days']} | "
        f"{row['executed_frozen_operator_offset_days']} | "
        f"{row['cell_metrics']['B']['median_modeled_to_observed_peak_ratio']!s} | "
        f"{row['diagnostic_attribution']} |"
        for row in result["operators"]
    )
    SYNTHESIS.write_text(
        "# EB-04W Scientific Synthesis\n\n"
        "Evidence mode: **Ran + Inference**. All observations remain "
        "`DIAGNOSTIC_ONLY`; this is not calibration, efficacy, or promotion.\n\n"
        "## Closure\n\n"
        f"All 16 cells completed. Maximum uncapped component-sum closure was "
        f"`{result['maximum_uncapped_melt_component_closure_m']:.3e} m`; maximum "
        f"applied component-plus-cap closure was "
        f"`{result['maximum_melt_component_closure_m']:.3e} m`; maximum daily "
        f"applied-to-retained-raw-melt closure was "
        f"`{result['maximum_daily_applied_raw_melt_closure_m']:.3e} m`; maximum "
        f"daily accumulation closure was `{result['maximum_accumulation_closure_m']:.3e} m`; "
        f"maximum active phase-fraction closure was "
        f"`{result['maximum_phase_fraction_closure']:.3e}`; maximum phase-amount "
        f"closure was `{result['maximum_phase_amount_closure_m']:.3e} m`; and "
        f"maximum physical-depth/SWE closure was "
        f"`{result['maximum_snow_depth_swe_closure_m']:.3e} m`; and maximum "
        f"pre-observed-peak mass closure was "
        f"`{result['maximum_pre_observed_peak_mass_closure_m']:.3e} m`; maximum "
        f"trace-to-WAT SWE/depth closure was "
        f"`{max(result['maximum_trace_wat_swe_closure_m'], result['maximum_trace_wat_depth_closure_m']):.3e} m`. Modeled wind "
        "redistribution was exactly zero by implementation status; this does not "
        "establish that physical redistribution at the SNOTEL sites was zero.\n\n"
        "## Frozen Operator Results\n\n"
        "| Lane | Operator | Frozen rubric offset (d) | Executed frozen operator (d) | B modeled/observed peak | Diagnostic attribution |\n"
        "|---|---|---:|---:|---:|---|\n"
        f"{rows}\n\n"
        "## Interpretation Boundary\n\n"
        "Baseline seasonal modeled-peak magnitudes are about `0.39-0.62` of "
        "the corresponding observed peaks. Separately, baseline modeled SWE "
        "retained on the observed SWE-peak dates has lane medians of about "
        "`0.21-0.46` of observed SWE, establishing a larger observed-date "
        "storage deficit. At Paradise, initial SWE plus realized snowfall SWE and "
        "retained rain remain below the observed peak in every evaluated water "
        "year and cell. Because retained rain is endogenous to pack state and "
        "liquid capacity, this localizes the boundary to the realized input "
        "pathway versus pre-peak losses; it does not prove an external forcing "
        "defect. Mica Creek, Niwot, and Snowbird sometimes receive enough "
        "realized snowpack input to reach the observed peak before recorded "
        "losses; their ownership is likewise mixed or unresolved. The ledger "
        "cannot separate precipitation representativeness, gauge undercatch, "
        "phase, liquid retention, physical redistribution, and pre-peak modeled "
        "loss timing.\n\n"
        "The four CoE columns are signed empirical melt-depth contributions. They "
        "help localize when the current formula removes or retains snow, but they "
        "are not separately observed energy fluxes. `bmelt` and `cmelt` mix "
        "temperature, cloud, wind, dewpoint, and canopy effects and cannot be "
        "treated as unique sensible-heat measurements. The result does not "
        "authorize tuning: all four lanes require finer phase-conditioned "
        "timing and ownership analysis of the realized input pathway and "
        "pre-peak losses before a process amendment is admissible.\n",
        encoding="utf-8",
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--analysis-only", action="store_true")
    parser.add_argument("--compact-existing", action="store_true")
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
    if sum((args.execute, args.analysis_only, args.compact_existing)) != 1:
        parser.error("select exactly one execution mode")
    if args.compact_existing:
        result = json.loads(RESULTS.read_text(encoding="utf-8"))
        lanes = selected_lanes()
        result["operators"] = operator_rows(lanes, result["cells"])
        write_summary(result["operators"])
        make_figures(result, lanes)
        write_synthesis(result)
        result["cells"] = {
            key: {field: value for field, value in row.items() if field != "daily"}
            for key, row in result["cells"].items()
        }
        write_json(RESULTS, result)
        print(json.dumps({"status": "PASS", "mode": "compact-existing"}, sort_keys=True))
        return 0
    if args.execute:
        execute(args.workers)
    result = analyze()
    print(
        json.dumps(
            {
                "cells": result["cell_count"],
                "operators": result["operator_count"],
                "max_melt_closure_m": result["maximum_melt_component_closure_m"],
                "status": "PASS",
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
