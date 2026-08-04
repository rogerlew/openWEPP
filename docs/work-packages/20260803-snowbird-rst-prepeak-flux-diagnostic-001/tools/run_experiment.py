#!/usr/bin/env python3
"""Execute RST sensitivity and reconstruct pre-observed-peak snow fluxes."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import importlib.util
import json
import math
import os
import shutil
import statistics
import subprocess
import sys
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True
REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FREEZE = ARTIFACTS / "experiment-freeze.json"
RECEIPT = ARTIFACTS / "execution-receipt.json"
RESULTS = ARTIFACTS / "response-results.json"
OUTPUT = REPO / "target/snowbird_rst_prepeak_flux_diagnostic"
FIXTURES = OUTPUT / "fixtures"
RUNS = OUTPUT / "runs"
DETAIL = OUTPUT / "annual-flux-ledger.json"
OPENWEPP = REPO / "target/release/openwepp-cli-hill"
W1_TOOL = REPO / "docs/work-packages/20260802-snow-surface-eb-04w1-precipitation-scaling-calibration-001/tools/run_precipitation_scaling.py"
RST_VALUES = tuple(x / 2.0 for x in range(9))


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


w1 = load_module("rst_flux_w1", W1_TOOL)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def variant_id(rst: float) -> str:
    return f"rst_{rst:.1f}".replace(".", "p")


def fixture_cli(path: Path) -> Path:
    files = sorted(path.glob("*.cli"))
    if len(files) != 1:
        raise RuntimeError(f"expected one climate file in {path}, found {len(files)}")
    return files[0]


def climate_dates(path: Path) -> list[dt.date]:
    dates: list[dt.date] = []
    for line in path.read_text().splitlines():
        fields = line.split()
        if len(fields) != 13:
            continue
        try:
            stamp = dt.date(int(fields[2]), int(fields[1]), int(fields[0]))
            list(map(float, fields[3:]))
        except ValueError:
            continue
        dates.append(stamp)
    if not dates or len(dates) != len(set(dates)) or dates != sorted(dates):
        raise RuntimeError(f"invalid climate chronology: {path}")
    return dates


def observed_peaks(path: Path) -> dict[int, tuple[dt.date, float]]:
    peaks: dict[int, tuple[dt.date, float]] = {}
    with path.open(newline="") as handle:
        for row in csv.DictReader(handle):
            if not row.get("observed_swe_mm"):
                continue
            water_year = int(row["water_year"])
            candidate = (
                dt.date.fromisoformat(row["date"]),
                float(row["observed_swe_mm"]) / 1000.0,
            )
            current = peaks.get(water_year)
            if current is None or candidate[1] > current[1]:
                peaks[water_year] = candidate
    return peaks


def prepare_fixture(lane: Any, rst: float) -> Path:
    destination = FIXTURES / lane.lane_id / variant_id(rst)
    shutil.copytree(lane.fixture_dir, destination)
    snow = destination / "snow.txt"
    lines = snow.read_text().splitlines()
    if len(lines) < 3:
        raise RuntimeError(f"invalid snow.txt: {snow}")
    lines[0] = f"{rst:.1f}  # diagnostic rain-snow threshold (deg C)"
    snow.write_text("\n".join(lines) + "\n")
    return destination


def execute_cell(lane: Any, rst: float, fixture: Path) -> dict[str, Any]:
    cell = variant_id(rst)
    run_dir = RUNS / lane.lane_id / cell
    run_dir.mkdir(parents=True)
    stem = f"{lane.lane_id}-{cell}"
    runfile = run_dir / f"{stem}.run"
    trace = run_dir / f"{stem}.snow.jsonl"
    fixture_stem = w1.eb04r.legacy.observed_harness.discover_run_stem(fixture)
    w1.eb04r.legacy.observed_harness.write_runfile(
        runfile, fixture, fixture_stem, run_dir, stem
    )
    command = w1.eb04r.legacy.observed_harness.cli_command(
        OPENWEPP, fixture, runfile, run_dir, "direct-production-executor"
    )
    environment, removed, effective = w1.eb04r.sanitized_environment(os.environ, "B", trace)
    completed = subprocess.run(command, cwd=REPO, env=environment, text=True, capture_output=True)
    (run_dir / "stdout.txt").write_text(completed.stdout)
    (run_dir / "stderr.txt").write_text(completed.stderr)
    if completed.returncode != 0:
        raise RuntimeError(f"openWEPP failed for {lane.lane_id}/{cell}: {completed.stderr[-2000:]}")
    return {
        "argv": [str(x) for x in command],
        "effective_openwepp_environment": effective,
        "removed_openwepp_keys": removed,
        "returncode": 0,
        "snow_txt_sha256": sha256(fixture / "snow.txt"),
        "trace_sha256": sha256(trace),
        "wat_sha256": sha256(run_dir / f"{stem}.wat.parquet"),
    }


def median(rows: list[float]) -> float | None:
    return statistics.median(rows) if rows else None


def analyze_cell(lane: Any, rst: float) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    cell = variant_id(rst)
    run_dir = RUNS / lane.lane_id / cell
    trace_path = run_dir / f"{lane.lane_id}-{cell}.snow.jsonl"
    dates = climate_dates(fixture_cli(lane.fixture_dir))
    trace = [json.loads(line) for line in trace_path.read_text().splitlines()]
    if len(trace) != len(dates):
        raise RuntimeError(f"trace/climate length mismatch for {lane.lane_id}/{cell}")
    by_date = dict(zip(dates, trace))
    peaks = observed_peaks(lane.observation_file)
    annual: list[dict[str, Any]] = []
    maximum_daily_closure = 0.0
    for water_year, (observed_peak_date, observed_peak_swe) in sorted(peaks.items()):
        start = dt.date(water_year - 1, 10, 1)
        window_dates = [d for d in dates if start <= d <= observed_peak_date]
        if not window_dates or window_dates[0] != start or observed_peak_date not in by_date:
            continue
        rows = [by_date[d] for d in window_dates]
        modeled_peak_date, modeled_peak_swe = max(
            ((d, float(by_date[d]["runtime_swe_after_m"])) for d in window_dates),
            key=lambda item: item[1],
        )
        sums = {
            key: sum(float(row[key]) for row in rows)
            for key in (
                "accumulation_m", "rain_retained_m", "snowpack_swe_loss_m",
                "sublimation_m", "raw_melt_m", "routed_melt_m",
                "liquid_water_released_m", "rain_released_m",
                "stage3_refrozen_liquid_m",
            )
        }
        daily_closures = []
        for row in rows:
            delta = float(row["runtime_swe_after_m"]) - float(row["runtime_swe_before_m"])
            expected = (
                float(row["accumulation_m"]) + float(row["rain_retained_m"])
                - float(row["snowpack_swe_loss_m"]) - float(row["sublimation_m"])
            )
            daily_closures.append(delta - expected)
        initial = float(rows[0]["runtime_swe_before_m"])
        final = float(rows[-1]["runtime_swe_after_m"])
        window_closure = final - initial - (
            sums["accumulation_m"] + sums["rain_retained_m"]
            - sums["snowpack_swe_loss_m"] - sums["sublimation_m"]
        )
        maximum_daily_closure = max(maximum_daily_closure, max(map(abs, daily_closures)))
        annual.append({
            "water_year": water_year,
            "window_start": start.isoformat(),
            "observed_peak_date": observed_peak_date.isoformat(),
            "observed_peak_swe_m": observed_peak_swe,
            "modeled_peak_date": modeled_peak_date.isoformat(),
            "modeled_peak_swe_m": modeled_peak_swe,
            "peak_swe_ratio": modeled_peak_swe / observed_peak_swe if observed_peak_swe > 0 else None,
            "peak_date_offset_days": (modeled_peak_date - observed_peak_date).days,
            "initial_swe_m": initial,
            "final_swe_m": final,
            "storage_change_m": final - initial,
            **sums,
            "window_mass_closure_m": window_closure,
            "maximum_daily_mass_closure_m": max(map(abs, daily_closures)),
        })
    if not annual:
        raise RuntimeError(f"no complete observed-peak windows for {lane.lane_id}/{cell}")
    summary = {
        "annual_count": len(annual),
        "median_peak_swe_ratio": median([x["peak_swe_ratio"] for x in annual if x["peak_swe_ratio"] is not None]),
        "median_peak_date_offset_days": median([float(x["peak_date_offset_days"]) for x in annual]),
        "median_accumulation_m": median([x["accumulation_m"] for x in annual]),
        "median_rain_retained_m": median([x["rain_retained_m"] for x in annual]),
        "median_snowpack_swe_loss_m": median([x["snowpack_swe_loss_m"] for x in annual]),
        "median_sublimation_m": median([x["sublimation_m"] for x in annual]),
        "median_raw_melt_m": median([x["raw_melt_m"] for x in annual]),
        "median_routed_melt_m": median([x["routed_melt_m"] for x in annual]),
        "median_liquid_water_released_m": median([x["liquid_water_released_m"] for x in annual]),
        "median_rain_released_m": median([x["rain_released_m"] for x in annual]),
        "median_refrozen_liquid_m": median([x["stage3_refrozen_liquid_m"] for x in annual]),
        "median_storage_change_m": median([x["storage_change_m"] for x in annual]),
        "maximum_daily_mass_closure_m": maximum_daily_closure,
        "maximum_window_mass_closure_m": max(abs(x["window_mass_closure_m"]) for x in annual),
    }
    return annual, summary


def execute() -> None:
    if RECEIPT.exists() or OUTPUT.exists():
        raise RuntimeError("refusing to overwrite result-bearing evidence")
    freeze = json.loads(FREEZE.read_text())
    bindings = {OPENWEPP: freeze["openwepp_binary_sha256"], Path(__file__): freeze["experiment_tool_sha256"], W1_TOOL: freeze["w1_tool_sha256"]}
    lanes = w1.selected_lanes()
    for lane in lanes:
        bindings[fixture_cli(Path(lane.fixture_dir))] = freeze["sites"][lane.lane_id]["climate_sha256"]
        bindings[Path(lane.fixture_dir) / "snow.txt"] = freeze["sites"][lane.lane_id]["snow_sha256"]
        bindings[Path(lane.observation_file)] = freeze["sites"][lane.lane_id]["observation_sha256"]
    for path, expected in bindings.items():
        if sha256(path) != expected:
            raise RuntimeError(f"binding mismatch: {path}")
    executed: dict[str, Any] = {}
    details: dict[str, Any] = {}
    summaries: dict[str, Any] = {}
    for lane in lanes:
        executed[lane.lane_id] = {}
        details[lane.lane_id] = {}
        summaries[lane.lane_id] = {}
        for rst in RST_VALUES:
            fixture = prepare_fixture(lane, rst)
            executed[lane.lane_id][variant_id(rst)] = execute_cell(lane, rst, fixture)
            annual, summary = analyze_cell(lane, rst)
            details[lane.lane_id][variant_id(rst)] = annual
            summaries[lane.lane_id][variant_id(rst)] = summary
            print("EXECUTED", lane.lane_id, rst)
    write_json(DETAIL, {"schema_version": 1, "sites": details})
    receipt = {
        "schema_version": 1,
        "freeze_sha256": sha256(FREEZE),
        "cell_count": len(lanes) * len(RST_VALUES),
        "annual_ledger_path": relative(DETAIL),
        "annual_ledger_sha256": sha256(DETAIL),
        "executed": executed,
    }
    write_json(RECEIPT, receipt)
    for site in summaries.values():
        baseline = site[variant_id(0.0)]
        for rst in RST_VALUES:
            row = site[variant_id(rst)]
            row["delta_peak_swe_ratio_vs_rst_0"] = row["median_peak_swe_ratio"] - baseline["median_peak_swe_ratio"]
            row["delta_accumulation_m_vs_rst_0"] = row["median_accumulation_m"] - baseline["median_accumulation_m"]
            row["delta_snowpack_loss_m_vs_rst_0"] = row["median_snowpack_swe_loss_m"] - baseline["median_snowpack_swe_loss_m"]
            if row["maximum_daily_mass_closure_m"] > 1.0e-12 or row["maximum_window_mass_closure_m"] > 1.0e-11:
                raise RuntimeError(f"mass closure exceeded for {row}")
    write_json(RESULTS, {
        "schema_version": 1,
        "freeze_sha256": sha256(FREEZE),
        "receipt_sha256": sha256(RECEIPT),
        "stress_range_disposition": "rst above 1 C is ASSUMED_FOR_EXECUTION diagnostic stress only",
        "sites": summaries,
    })
    print("RST_PREPEAK_FLUX_COMPLETE", len(lanes) * len(RST_VALUES))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execute", action="store_true")
    args = parser.parse_args()
    if not args.execute:
        raise RuntimeError("--execute is required")
    execute()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
