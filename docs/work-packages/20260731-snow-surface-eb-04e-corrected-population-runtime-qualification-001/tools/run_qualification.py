#!/usr/bin/env python3
"""Execute and audit the frozen EB-04E corrected-population qualification."""

from __future__ import annotations

import argparse
import csv
import hashlib
import importlib.util
import json
import math
import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
OUTPUT = REPO / "target/snow_surface_eb04e_qualification"
BINARY = REPO / "target/debug/openwepp-cli-hill"
ATTEMPT = ARTIFACTS / "execution-attempt.json"
REPORT = ARTIFACTS / "qualification-results.json"
PROTOCOL = ARTIFACTS / "prospective-qualification-protocol.md"
EB04_TOOL = REPO / (
    "docs/work-packages/20260730-snow-surface-eb-04-factorial-"
    "execution-adjudication-001/tools/run_factorial.py"
)
EB04_REPORT = REPO / (
    "docs/work-packages/20260730-snow-surface-eb-04-factorial-"
    "execution-adjudication-001/artifacts/factorial-results.json"
)
EXPECTED_HEAD = "44c6c9cc2e4447064fbbbf70935cf581d60d49b0"
EXPECTED_EB04_TOOL = "e84a1732a847b978cc529ba95bb276b4f47ff37e991d06798d158523f2bace17"
EXPECTED_EB04_REPORT = "56f38bb6696b682f77d47c492759417d8e28975c45497d9280a566fedc6831d2"
MASS_TOLERANCE_M = 1.0e-9
ENERGY_TOLERANCE_J_M2 = 1.0e-6
VAPOR_TOLERANCE_KG_M2 = 1.0e-9
VAPOR_SUBLIMATION_TOLERANCE_KG_M2 = 1.0e-6
LAYER_FIELDS = {
    "mass_swe_m",
    "thickness_m",
    "density_kg_m3",
    "settle_day_count",
    "temperature_c",
    "liquid_water_m",
    "cold_content_j_m2",
    "refrozen_liquid_m",
}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-check", action="store_true")
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--analysis-only", action="store_true")
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
    if sum((args.self_check, args.execute, args.analysis_only)) != 1:
        parser.error("select exactly one of --self-check, --execute, --analysis-only")
    self_check()
    if args.self_check:
        print("EB-04E anti-alias self-check: PASS")
        return 0
    if args.execute:
        return execute(args.workers)
    return analyze()


def load_eb04() -> Any:
    spec = importlib.util.spec_from_file_location("eb04e_frozen_harness", EB04_TOOL)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import frozen harness {EB04_TOOL}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def frozen_inputs() -> tuple[Any, list[Any], dict[str, Any], set[tuple[str, str]]]:
    if git_head() != EXPECTED_HEAD:
        raise RuntimeError(f"candidate HEAD drift: {git_head()} != {EXPECTED_HEAD}")
    if sha256(EB04_TOOL) != EXPECTED_EB04_TOOL:
        raise RuntimeError("frozen EB-04 harness drift")
    if sha256(EB04_REPORT) != EXPECTED_EB04_REPORT:
        raise RuntimeError("frozen EB-04 report drift")
    harness = load_eb04()
    lanes = harness.fixed_lanes()
    source = json.loads(EB04_REPORT.read_text(encoding="utf-8"))
    source_lanes = {lane["lane_id"]: lane for lane in source["lanes"]}
    if len(lanes) != 12 or set(source_lanes) != {lane.lane_id for lane in lanes}:
        raise RuntimeError("frozen lane inventory drift")
    former_failures: set[tuple[str, str]] = set()
    for lane in lanes:
        frozen = source_lanes[lane.lane_id]
        expected_hashes = {
            cell["fixture_sha256"] for cell in frozen["cells"].values()
        }
        current_hash = tree_sha256(lane.fixture_dir)
        if expected_hashes != {current_hash}:
            raise RuntimeError(f"fixture drift for {lane.lane_id}")
        for cell_name, cell in frozen["cells"].items():
            if cell["execution_status"] == "FAIL":
                former_failures.add((lane.lane_id, cell_name))
    if len(former_failures) != 24:
        raise RuntimeError(f"former-failure inventory drift: {len(former_failures)}")
    return harness, lanes, source_lanes, former_failures


def execute(workers: int) -> int:
    harness, lanes, source_lanes, former_failures = frozen_inputs()
    if ATTEMPT.exists():
        raise RuntimeError("execution attempt already exists; use --analysis-only")
    if not BINARY.is_file():
        raise FileNotFoundError(f"build exact runner first: {BINARY}")
    source_diff = command_sha256(["git", "diff", "--binary", "--", "crates", "tests"])
    if source_diff != hashlib.sha256(b"").hexdigest():
        raise RuntimeError("executable source/test diff is not empty")
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    FIGURES.mkdir(parents=True, exist_ok=True)
    OUTPUT.mkdir(parents=True, exist_ok=True)
    attempt = {
        "schema": "snow-surface-eb04e-execution-attempt-v1",
        "status": "STARTED",
        "started_unix_seconds": time.time(),
        "git_head": git_head(),
        "binary_sha256": sha256(BINARY),
        "binary_size_bytes": BINARY.stat().st_size,
        "protocol_sha256": sha256(PROTOCOL),
        "tool_sha256": sha256(Path(__file__)),
        "frozen_harness_sha256": sha256(EB04_TOOL),
        "frozen_report_sha256": sha256(EB04_REPORT),
        "source_diff_sha256": source_diff,
        "workers": workers,
        "retry_policy": "NO_SEMANTIC_RETRY",
    }
    write_json(ATTEMPT, attempt)
    tasks = [(lane, cell) for lane in lanes for cell in harness.CELLS]
    results: dict[tuple[str, str], dict[str, Any]] = {}
    with ThreadPoolExecutor(max_workers=workers) as pool:
        futures = {
            pool.submit(run_cell, harness, lane, cell): (lane.lane_id, cell)
            for lane, cell in tasks
        }
        for future in as_completed(futures):
            key = futures[future]
            results[key] = future.result()
            print(f"{key[0]}/{key[1]}: {results[key]['execution_status']}", flush=True)
    attempt["status"] = "COMPLETE"
    attempt["completed_unix_seconds"] = time.time()
    attempt["result_count"] = len(results)
    write_json(ATTEMPT, attempt)
    return reduce_and_write(harness, lanes, source_lanes, former_failures, results)


def analyze() -> int:
    harness, lanes, source_lanes, former_failures = frozen_inputs()
    if not ATTEMPT.is_file():
        raise FileNotFoundError("analysis requires execution-attempt.json")
    attempt = json.loads(ATTEMPT.read_text(encoding="utf-8"))
    if attempt["status"] != "COMPLETE":
        raise RuntimeError("execution attempt is incomplete")
    if sha256(BINARY) != attempt["binary_sha256"]:
        raise RuntimeError("current binary differs from executed binary")
    if sha256(PROTOCOL) != attempt["protocol_sha256"]:
        raise RuntimeError("protocol drift after execution")
    if sha256(Path(__file__)) != attempt["tool_sha256"]:
        raise RuntimeError("qualification tool drift after execution")
    results = {
        (lane.lane_id, cell): audit_existing(harness, lane, cell)
        for lane in lanes
        for cell in harness.CELLS
    }
    return reduce_and_write(harness, lanes, source_lanes, former_failures, results)


def run_cell(harness: Any, lane: Any, cell: str) -> dict[str, Any]:
    run_dir = OUTPUT / "runs" / lane.lane_id / cell
    run_dir.mkdir(parents=True, exist_ok=True)
    trace = run_dir / f"{lane.lane_id}-{cell}.snow.jsonl"
    runfile = run_dir / f"{lane.lane_id}-{cell}.run"
    run_id = f"{lane.lane_id}-{cell}"
    stem = harness.observed_harness.discover_run_stem(lane.fixture_dir)
    harness.observed_harness.write_runfile(runfile, lane.fixture_dir, stem, run_dir, run_id)
    command = harness.observed_harness.cli_command(
        BINARY, lane.fixture_dir, runfile, run_dir, "direct-production-executor"
    )
    longwave, sublimation = harness.CELLS[cell]
    env = os.environ.copy()
    env.update(harness.NON_TARGET_ENV)
    env.update(
        {
            "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": longwave,
            "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL": sublimation,
            "OPENWEPP_R7H_SNOW_TRACE_PATH": str(trace),
        }
    )
    for stale in (trace, run_dir / f"{run_id}.wat.parquet"):
        if stale.exists():
            raise RuntimeError(f"pre-existing result violates one-run protocol: {stale}")
    completed = subprocess.run(
        command,
        cwd=REPO,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    (run_dir / "stdout.txt").write_text(completed.stdout, encoding="utf-8")
    (run_dir / "stderr.txt").write_text(completed.stderr, encoding="utf-8")
    return audit_outputs(harness, lane, cell, completed.returncode, command)


def audit_existing(harness: Any, lane: Any, cell: str) -> dict[str, Any]:
    run_dir = OUTPUT / "runs" / lane.lane_id / cell
    stderr = run_dir / "stderr.txt"
    returncode = 1 if stderr.is_file() and "runtime surface failure" in stderr.read_text(encoding="utf-8") else 0
    return audit_outputs(harness, lane, cell, returncode, [])


def audit_outputs(
    harness: Any, lane: Any, cell: str, returncode: int, command: list[Any]
) -> dict[str, Any]:
    run_dir = OUTPUT / "runs" / lane.lane_id / cell
    run_id = f"{lane.lane_id}-{cell}"
    trace = run_dir / f"{run_id}.snow.jsonl"
    wat = run_dir / f"{run_id}.wat.parquet"
    base = {
        "lane_id": lane.lane_id,
        "cell": cell,
        "returncode": returncode,
        "command": [str(value) for value in command],
        "fixture_sha256": tree_sha256(lane.fixture_dir),
        "target_selectors": {
            "longwave": harness.CELLS[cell][0],
            "sublimation": harness.CELLS[cell][1],
        },
        "non_target_environment": harness.NON_TARGET_ENV,
        "trace": relative(trace),
        "wat": relative(wat),
    }
    if returncode != 0 or not trace.is_file() or not wat.is_file():
        stderr = run_dir / "stderr.txt"
        base.update(
            {
                "execution_status": "FAIL",
                "failure": stderr.read_text(encoding="utf-8").splitlines()[-1]
                if stderr.is_file() and stderr.read_text(encoding="utf-8").splitlines()
                else "missing WAT/trace output",
            }
        )
        return base
    audit = audit_trace(trace)
    wat_rows = parquet_rows(wat)
    chronology_passes = audit["first_day_index"] == 0 and audit["last_day_index"] == audit["trace_row_count"] - 1 and audit["chronology_gap_count"] == 0
    output_identity_passes = wat_rows == audit["trace_row_count"]
    passes = audit["physical_passes"] and chronology_passes and output_identity_passes
    base.update(
        {
            "execution_status": "PASS" if passes else "FAIL",
            "trace_sha256": sha256(trace),
            "wat_sha256": sha256(wat),
            "wat_row_count": wat_rows,
            "chronology_passes": chronology_passes,
            "output_identity_passes": output_identity_passes,
            "physical": audit,
        }
    )
    return base


def audit_trace(path: Path) -> dict[str, Any]:
    maxima = {
        "mass_m": 0.0,
        "surface_j_m2": 0.0,
        "cold_content_j_m2": 0.0,
        "latent_ratio": 0.0,
        "daily_latent_j_m2": 0.0,
        "shortwave_j_m2": 0.0,
        "longwave_j_m2": 0.0,
        "vapor_kg_m2": 0.0,
        "vapor_sublimation_kg_m2": 0.0,
        "layer_swe_runtime_m": 0.0,
        "layer_depth_runtime_m": 0.0,
        "layer_swe_published_m": 0.0,
        "layer_depth_published_m": 0.0,
    }
    first = last = None
    gaps = count = enabled = invalid_layers = 0
    suspended_seconds = collapsed_seconds = 0.0
    total_sublimation = total_longwave = total_latent = 0.0
    minimum_temperature = None
    represented_subnanometer_count = 0
    with path.open(encoding="utf-8") as stream:
        for line in stream:
            if not line.strip():
                continue
            row = json.loads(line)
            day = int(row["day_index"])
            first = day if first is None else first
            if last is not None and day != last + 1:
                gaps += 1
            last = day
            count += 1
            maxima["mass_m"] = max(
                maxima["mass_m"],
                abs(
                    row["runtime_swe_before_m"]
                    + row["accumulation_m"]
                    + row["rain_retained_m"]
                    - row["sublimation_m"]
                    - row["snowpack_swe_loss_m"]
                    - row["runtime_swe_after_m"]
                ),
            )
            layers = row["snow_layers_after"]
            layer_swe = math.fsum(float(layer["mass_swe_m"]) for layer in layers)
            layer_depth = math.fsum(float(layer["thickness_m"]) for layer in layers)
            maxima["layer_swe_runtime_m"] = max(maxima["layer_swe_runtime_m"], abs(layer_swe - row["runtime_swe_after_m"]))
            maxima["layer_depth_runtime_m"] = max(maxima["layer_depth_runtime_m"], abs(layer_depth - row["runtime_depth_after_m"]))
            maxima["layer_swe_published_m"] = max(maxima["layer_swe_published_m"], abs(layer_swe - row["snow_layer_swe_sum_after_m"]))
            maxima["layer_depth_published_m"] = max(maxima["layer_depth_published_m"], abs(layer_depth - row["snow_layer_depth_sum_after_m"]))
            for layer in layers:
                if set(layer) != LAYER_FIELDS or not all(math.isfinite(float(value)) for value in layer.values()):
                    invalid_layers += 1
                    continue
                if any(float(layer[key]) < 0.0 for key in ("mass_swe_m", "thickness_m", "density_kg_m3", "settle_day_count", "liquid_water_m", "cold_content_j_m2", "refrozen_liquid_m")):
                    invalid_layers += 1
                temperature = float(layer["temperature_c"])
                if not (-273.15 < temperature <= 0.0):
                    invalid_layers += 1
                minimum_temperature = temperature if minimum_temperature is None else min(minimum_temperature, temperature)
                if 1.0e-12 < float(layer["mass_swe_m"]) <= 1.0e-9:
                    represented_subnanometer_count += 1
            suspended_seconds += float(row["stage3_thermal_domain_suspended_seconds"])
            collapsed_seconds += float(row["stage3_lower_thermal_volume_collapsed_seconds"])
            total_sublimation += float(row["sublimation_m"])
            total_longwave += float(row["stage3_longwave_energy_j_m2"])
            total_latent += float(row["stage3_latent_energy_j_m2"])
            if not row["stage3_energy_enabled"]:
                continue
            enabled += 1
            hourly_mass = row["stage3_hourly_vapor_mass_exchange_kg_m2"]
            hourly_heat = row["stage3_hourly_latent_heat_j_kg"]
            hourly_flux = row["stage3_hourly_latent_flux_w_m2"]
            hourly_sw = row["stage3_hourly_net_shortwave_w_m2"]
            hourly_lw = row["stage3_hourly_net_longwave_w_m2"]
            if any(len(values) != 24 for values in (hourly_mass, hourly_heat, hourly_flux, hourly_sw, hourly_lw)):
                raise RuntimeError(f"hourly operand cardinality drift in {path} day {day}")
            surface = row["stage3_shortwave_energy_j_m2"] + row["stage3_longwave_energy_j_m2"] + row["stage3_latent_energy_j_m2"] - row["stage3_surface_energy_j_m2"] - row["stage3_unused_positive_energy_j_m2"]
            cold = row["stage3_surface_energy_j_m2"] + row["stage3_conduction_energy_j_m2"] + row["stage3_latent_refreeze_energy_j_m2"] + row["stage3_cold_content_export_j_m2"] - (row["stage3_cold_content_before_j_m2"] - row["stage3_cold_content_after_j_m2"])
            latent = math.fsum(flux * 3600.0 - mass * heat for flux, mass, heat in zip(hourly_flux, hourly_mass, hourly_heat, strict=True))
            allowance = max(ENERGY_TOLERANCE_J_M2, 16.0 * sys.float_info.epsilon * math.fsum(abs(flux * 3600.0) + abs(mass * heat) for flux, mass, heat in zip(hourly_flux, hourly_mass, hourly_heat, strict=True)))
            maxima["surface_j_m2"] = max(maxima["surface_j_m2"], abs(surface))
            maxima["cold_content_j_m2"] = max(maxima["cold_content_j_m2"], abs(cold))
            maxima["latent_ratio"] = max(maxima["latent_ratio"], abs(latent) / allowance)
            maxima["daily_latent_j_m2"] = max(maxima["daily_latent_j_m2"], abs(row["stage3_latent_energy_j_m2"] - 3600.0 * math.fsum(hourly_flux)))
            maxima["shortwave_j_m2"] = max(maxima["shortwave_j_m2"], abs(row["stage3_shortwave_energy_j_m2"] - 3600.0 * math.fsum(hourly_sw)))
            maxima["longwave_j_m2"] = max(maxima["longwave_j_m2"], abs(row["stage3_longwave_energy_j_m2"] - 3600.0 * math.fsum(hourly_lw)))
            maxima["vapor_kg_m2"] = max(maxima["vapor_kg_m2"], abs(row["stage3_vapor_mass_exchange_kg_m2"] - math.fsum(hourly_mass)))
            maxima["vapor_sublimation_kg_m2"] = max(maxima["vapor_sublimation_kg_m2"], abs(row["stage3_vapor_mass_exchange_kg_m2"] + 1000.0 * row["sublimation_m"]))
    thresholds = {
        "mass_m": MASS_TOLERANCE_M,
        "surface_j_m2": ENERGY_TOLERANCE_J_M2,
        "cold_content_j_m2": ENERGY_TOLERANCE_J_M2,
        "latent_ratio": 1.0,
        "daily_latent_j_m2": ENERGY_TOLERANCE_J_M2,
        "shortwave_j_m2": ENERGY_TOLERANCE_J_M2,
        "longwave_j_m2": ENERGY_TOLERANCE_J_M2,
        "vapor_kg_m2": VAPOR_TOLERANCE_KG_M2,
        "vapor_sublimation_kg_m2": VAPOR_SUBLIMATION_TOLERANCE_KG_M2,
        "layer_swe_runtime_m": MASS_TOLERANCE_M,
        "layer_depth_runtime_m": MASS_TOLERANCE_M,
        "layer_swe_published_m": MASS_TOLERANCE_M,
        "layer_depth_published_m": MASS_TOLERANCE_M,
    }
    physical_passes = count > 0 and enabled > 0 and invalid_layers == 0 and all(maxima[key] <= threshold for key, threshold in thresholds.items())
    return {
        "trace_row_count": count,
        "first_day_index": first,
        "last_day_index": last,
        "chronology_gap_count": gaps,
        "active_stage3_day_count": enabled,
        "invalid_layer_count": invalid_layers,
        "minimum_layer_temperature_c": minimum_temperature,
        "thermal_domain_suspended_seconds": suspended_seconds,
        "lower_volume_collapsed_seconds": collapsed_seconds,
        "represented_subnanometer_layer_occurrences": represented_subnanometer_count,
        "total_sublimation_m": total_sublimation,
        "total_longwave_energy_j_m2": total_longwave,
        "total_latent_energy_j_m2": total_latent,
        "maximum_residuals": maxima,
        "thresholds": thresholds,
        "physical_passes": physical_passes,
    }


def reduce_and_write(
    harness: Any,
    lanes: list[Any],
    source_lanes: dict[str, Any],
    former_failures: set[tuple[str, str]],
    results: dict[tuple[str, str], dict[str, Any]],
) -> int:
    expected = {(lane.lane_id, cell) for lane in lanes for cell in harness.CELLS}
    inventory_passes = set(results) == expected
    former = [
        {
            "lane_id": lane,
            "cell": cell,
            "completed": results[(lane, cell)]["execution_status"] == "PASS",
        }
        for lane, cell in sorted(former_failures)
    ]
    lane_reports = []
    for lane in lanes:
        cells = {cell: results[(lane.lane_id, cell)] for cell in harness.CELLS}
        frozen = source_lanes[lane.lane_id]
        identity = (
            len({value["fixture_sha256"] for value in cells.values()}) == 1
            and all(value["non_target_environment"] == harness.NON_TARGET_ENV for value in cells.values())
            and all(value["target_selectors"] == harness.CELLS[cell] or value["target_selectors"] == {"longwave": harness.CELLS[cell][0], "sublimation": harness.CELLS[cell][1]} for cell, value in cells.items())
            and cells["B"]["fixture_sha256"] == frozen["cells"]["B"]["fixture_sha256"]
        )
        lane_reports.append(
            {
                "lane_id": lane.lane_id,
                "role": lane.role,
                "stratum": lane.stratum,
                "climate": lane.climate,
                "fixture": relative(lane.fixture_dir),
                "trace_identity_passes": identity,
                "cells": cells,
            }
        )
    all_completed = inventory_passes and all(cell["execution_status"] == "PASS" for lane in lane_reports for cell in lane["cells"].values())
    all_identity = all(lane["trace_identity_passes"] for lane in lane_reports)
    former_pass = all(item["completed"] for item in former)
    process_reach = {
        "longwave_nonzero_cells": sum(abs(cell["physical"]["total_longwave_energy_j_m2"]) > 0.0 for lane in lane_reports for name, cell in lane["cells"].items() if name in {"L", "LS"} and cell["execution_status"] == "PASS"),
        "sublimation_nonzero_cells": sum(cell["physical"]["total_sublimation_m"] > 0.0 for lane in lane_reports for name, cell in lane["cells"].items() if name in {"S", "LS"} and cell["execution_status"] == "PASS"),
        "thermal_suspension_cells": sum(cell["physical"]["thermal_domain_suspended_seconds"] > 0.0 for lane in lane_reports for cell in lane["cells"].values() if cell["execution_status"] == "PASS"),
        "lower_collapse_cells": sum(cell["physical"]["lower_volume_collapsed_seconds"] > 0.0 for lane in lane_reports for cell in lane["cells"].values() if cell["execution_status"] == "PASS"),
        "represented_subnanometer_cells": sum(cell["physical"]["represented_subnanometer_layer_occurrences"] > 0 for lane in lane_reports for cell in lane["cells"].values() if cell["execution_status"] == "PASS"),
    }
    acceptance = {
        "inventory_12_lanes_48_cells": inventory_passes and len(lane_reports) == 12 and len(results) == 48,
        "all_cells_complete": all_completed,
        "all_trace_identity": all_identity,
        "all_former_failures_complete": former_pass and len(former) == 24,
        "longwave_reaches_all_enabled_cells": process_reach["longwave_nonzero_cells"] == 24,
        "sublimation_reaches_enabled_population": process_reach["sublimation_nonzero_cells"] > 0,
        "no_observation_scoring_or_factorial_effects": True,
    }
    acceptance_passes = all(acceptance.values())
    report = {
        "schema": "snow-surface-eb04e-qualification-v1",
        "evidence_class": "Ran",
        "source": {
            "git_head": git_head(),
            "binary": relative(BINARY),
            "binary_sha256": json.loads(ATTEMPT.read_text(encoding="utf-8"))["binary_sha256"],
            "protocol_sha256": sha256(PROTOCOL),
            "tool_sha256": sha256(Path(__file__)),
            "frozen_eb04_tool_sha256": sha256(EB04_TOOL),
            "frozen_eb04_report_sha256": sha256(EB04_REPORT),
        },
        "claim_boundary": {
            "observations_loaded_for_scoring": False,
            "factorial_effects_computed": False,
            "interaction_computed": False,
            "calibration_performed": False,
            "promotion_adjudicated": False,
        },
        "acceptance": acceptance,
        "acceptance_passes": acceptance_passes,
        "process_reach": process_reach,
        "former_failures": former,
        "lanes": lane_reports,
    }
    write_json(REPORT, report)
    write_summary(report)
    write_cells_csv(report)
    make_figures(report)
    check_figure_inventory()
    print(json.dumps({"acceptance": acceptance, "process_reach": process_reach, "acceptance_passes": acceptance_passes}, indent=2, sort_keys=True))
    return 0 if acceptance_passes else 2


def write_summary(report: dict[str, Any]) -> None:
    lines = [
        "# EB-04E Runtime Qualification",
        "",
        f"Status: `{'PASS' if report['acceptance_passes'] else 'HOLD'}`",
        "",
        "Evidence class: `Ran`",
        "",
        "EB-04E evaluates runtime and physical admissibility only. It reads no observations for scoring and emits no factorial effect, interaction, calibration, or promotion result.",
        "",
        "## Acceptance",
        "",
        "| Criterion | Result |",
        "| --- | --- |",
    ]
    lines.extend(f"| {key.replace('_', ' ')} | {'PASS' if value else 'FAIL'} |" for key, value in report["acceptance"].items())
    lines.extend(["", "## Process Reach", "", "| Diagnostic | Cells |", "| --- | ---: |"])
    lines.extend(f"| {key.replace('_', ' ')} | {value} |" for key, value in report["process_reach"].items())
    lines.extend(["", "EB-04R remains the first package permitted to freeze and score a new empirical factorial.", ""])
    (ARTIFACTS / "runtime-qualification.md").write_text("\n".join(lines), encoding="utf-8")


def write_cells_csv(report: dict[str, Any]) -> None:
    with (ARTIFACTS / "cell-qualification.csv").open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle)
        writer.writerow(["lane_id", "cell", "status", "days", "max_mass_m", "max_surface_j_m2", "max_cold_j_m2", "max_layer_swe_m", "max_layer_depth_m", "minimum_temperature_c", "suspended_seconds", "collapsed_seconds", "subnanometer_layer_occurrences", "total_sublimation_m", "total_longwave_j_m2", "total_latent_j_m2"])
        for lane in report["lanes"]:
            for cell_name, cell in lane["cells"].items():
                physical = cell.get("physical", {})
                residuals = physical.get("maximum_residuals", {})
                writer.writerow([lane["lane_id"], cell_name, cell["execution_status"], physical.get("trace_row_count"), residuals.get("mass_m"), residuals.get("surface_j_m2"), residuals.get("cold_content_j_m2"), residuals.get("layer_swe_runtime_m"), residuals.get("layer_depth_runtime_m"), physical.get("minimum_layer_temperature_c"), physical.get("thermal_domain_suspended_seconds"), physical.get("lower_volume_collapsed_seconds"), physical.get("represented_subnanometer_layer_occurrences"), physical.get("total_sublimation_m"), physical.get("total_longwave_energy_j_m2"), physical.get("total_latent_energy_j_m2")])


def make_figures(report: dict[str, Any]) -> None:
    import matplotlib

    matplotlib.use("Agg")
    matplotlib.rcParams["svg.hashsalt"] = "snow-surface-eb04e"
    import matplotlib.pyplot as plt
    import numpy as np

    lanes = report["lanes"]
    cells = ["B", "L", "S", "LS"]
    lane_names = [lane["lane_id"] for lane in lanes]
    completion = np.array([[int(lane["cells"][cell]["execution_status"] == "PASS") for cell in cells] for lane in lanes])
    fig, ax = plt.subplots(figsize=(7.2, 7.5))
    image = ax.imshow(completion, cmap="RdYlGn", vmin=0, vmax=1, aspect="auto")
    ax.set_xticks(range(4), cells)
    ax.set_yticks(range(12), lane_names)
    ax.set_title("Corrected-population runtime completion")
    fig.colorbar(image, ax=ax, ticks=[0, 1], label="0 failed, 1 passed")
    save_figure(fig, "eb04e-runtime-completion")
    sidecar("eb04e-runtime-completion", "Did every corrected B/L/S/LS cell complete?", "All 12 frozen EB-04 lanes and 48 cells.", "Binary pass/fail status from one hash-bound execution.", "Completion establishes runtime reach, not empirical improvement or promotion.")

    residual_keys = [("mass_m", "Mass", MASS_TOLERANCE_M), ("surface_j_m2", "Surface energy", ENERGY_TOLERANCE_J_M2), ("cold_content_j_m2", "Cold content", ENERGY_TOLERANCE_J_M2), ("layer_swe_runtime_m", "Layer SWE", MASS_TOLERANCE_M), ("layer_depth_runtime_m", "Layer depth", MASS_TOLERANCE_M)]
    fig, ax = plt.subplots(figsize=(10.5, 5.8))
    x = np.arange(len(residual_keys))
    width = 0.18
    for offset, cell_name, color in zip((-1.5, -0.5, 0.5, 1.5), cells, ("#555555", "#7a5195", "#ef5675", "#003f5c"), strict=True):
        ratios = []
        for key, _, threshold in residual_keys:
            maximum = max(lane["cells"][cell_name]["physical"]["maximum_residuals"][key] for lane in lanes)
            ratios.append(max(maximum / threshold, 1.0e-16))
        ax.bar(x + offset * width, ratios, width, label=cell_name, color=color)
    ax.axhline(1.0, color="#d62728", linestyle="--", linewidth=1.0, label="Acceptance bound")
    ax.set_yscale("log")
    ax.set_xticks(x, [label for _, label, _ in residual_keys])
    ax.set_ylabel("Maximum residual / acceptance bound")
    ax.set_title("Independent physical-ledger closure")
    ax.legend(frameon=False, ncol=5)
    save_figure(fig, "eb04e-ledger-closure")
    sidecar("eb04e-ledger-closure", "How close were the worst physical ledgers to their acceptance bounds?", "Maximum independently reconstructed residual by cell family across all lanes.", "Dimensionless residual-to-bound ratio on a logarithmic axis; 1 is the rejection boundary.", "The plot uses serialized operands and layer vectors, not producer residuals; it does not measure empirical accuracy.")

    fig, ax = plt.subplots(figsize=(11, 5.8))
    x = np.arange(len(lanes))
    width = 0.18
    for offset, cell_name, color in zip((-1.5, -0.5, 0.5, 1.5), cells, ("#555555", "#7a5195", "#ef5675", "#003f5c"), strict=True):
        values = [lane["cells"][cell_name]["physical"]["minimum_layer_temperature_c"] for lane in lanes]
        ax.bar(x + offset * width, values, width, label=cell_name, color=color)
    ax.set_xticks(x, lane_names, rotation=35, ha="right")
    ax.set_ylabel("Minimum populated layer temperature (°C)")
    ax.set_title("Population thermal-domain minima")
    ax.legend(frameon=False, ncol=4)
    save_figure(fig, "eb04e-thermal-minima")
    sidecar("eb04e-thermal-minima", "What minimum valid snow-layer temperature did each corrected cell reach?", "All populated serialized layers across each complete trajectory.", "Degrees Celsius; every value must remain above -273.15 and at or below 0.", "This is a runtime-domain diagnostic, not a temperature validation against observations.")

    fig, axes = plt.subplots(1, 2, figsize=(11, 5.5))
    totals = {cell: {"suspension": 0.0, "collapse": 0.0, "subnanometer": 0} for cell in cells}
    for lane in lanes:
        for cell in cells:
            physical = lane["cells"][cell]["physical"]
            totals[cell]["suspension"] += physical["thermal_domain_suspended_seconds"] / 86400.0
            totals[cell]["collapse"] += physical["lower_volume_collapsed_seconds"] / 86400.0
            totals[cell]["subnanometer"] += physical["represented_subnanometer_layer_occurrences"]
    axes[0].bar(cells, [totals[cell]["suspension"] for cell in cells], label="Thermal suspension", color="#7a5195")
    axes[0].bar(cells, [totals[cell]["collapse"] for cell in cells], bottom=[totals[cell]["suspension"] for cell in cells], label="Lower-volume collapse", color="#ffa600")
    axes[0].set_ylabel("Aggregate branch duration (days)")
    axes[0].set_title("EB-04C boundary use")
    axes[0].legend(frameon=False)
    axes[1].bar(cells, [totals[cell]["subnanometer"] for cell in cells], color="#003f5c")
    axes[1].set_ylabel("Serialized layer occurrences")
    axes[1].set_title("EB-04D represented subnanometer SWE")
    save_figure(fig, "eb04e-correction-boundary-use")
    sidecar("eb04e-correction-boundary-use", "How often did the corrected population exercise the EB-04C and EB-04D boundaries?", "All 48 complete trajectories, aggregated by B/L/S/LS cell.", "Branch duration in days and count of layer occurrences with SWE above 1e-12 m and at or below 1e-9 m.", "Counts establish real correction-path use; they are not factorial effects or evidence of empirical benefit.")


def sidecar(stem: str, question: str, population: str, units: str, limitation: str) -> None:
    payload = f"""# {stem}

Status: `complete`

Evidence class: `Ran`

Figure: [`{stem}.svg`]({stem}.svg)

## Caption

{question} {population} {units}

## Question

{question}

## Population

{population}

## Units And Processing

{units}

## Interpretation

The figure summarizes the hash-bound EB-04E physical qualification and retains
all B/L/S/LS cells without observation scoring.

## Limitation

{limitation}
"""
    (FIGURES / f"{stem}.md").write_text(payload, encoding="utf-8")


def save_figure(fig: Any, stem: str) -> None:
    FIGURES.mkdir(parents=True, exist_ok=True)
    fig.tight_layout()
    fig.savefig(FIGURES / f"{stem}.svg", format="svg", metadata={"Date": None})
    import matplotlib.pyplot as plt

    plt.close(fig)


def check_figure_inventory() -> None:
    svgs = {path.stem for path in FIGURES.glob("*.svg")}
    sidecars = {path.stem for path in FIGURES.glob("*.md")}
    if svgs != sidecars or len(svgs) != 4:
        raise RuntimeError(f"figure inventory mismatch: {svgs} != {sidecars}")


def self_check() -> None:
    mass = [-0.003, -0.001]
    heat = [2_840_000.0, 2_810_000.0]
    flux = [m * h / 3600.0 for m, h in zip(mass, heat, strict=True)]
    correct = math.fsum(f * 3600.0 - m * h for f, m, h in zip(flux, mass, heat, strict=True))
    wrong_sign = math.fsum(f * 3600.0 + m * h for f, m, h in zip(flux, mass, heat, strict=True))
    wrong_pair = math.fsum(f * 3600.0 - m * h for f, m, h in zip(flux, mass, reversed(heat), strict=True))
    surface = 5.0 + 7.0 - 2.0
    applied = 8.0
    unused = 2.0
    wrong_omitted_unused = surface - applied
    layer_mass = [5.0e-10, 0.1]
    layer_depth = [1.1e-9, 0.2]
    if abs(correct) > 1.0e-9 or abs(wrong_sign) < 1.0 or abs(wrong_pair) < 1.0:
        raise RuntimeError("latent anti-alias self-check failed")
    if abs(surface - applied - unused) > 1.0e-12 or abs(wrong_omitted_unused) < 1.0:
        raise RuntimeError("surface-energy anti-alias self-check failed")
    if math.isclose(math.fsum(layer_mass), math.fsum(layer_depth), abs_tol=1.0e-6):
        raise RuntimeError("layer-unit anti-alias self-check failed")
    if not (layer_mass[0] <= 1.0e-9 and layer_mass[0] > 1.0e-12):
        raise RuntimeError("represented-fragment self-check failed")


def parquet_rows(path: Path) -> int:
    import pyarrow.parquet as pq

    return pq.ParquetFile(path).metadata.num_rows


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def tree_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    for item in sorted(candidate for candidate in path.rglob("*") if candidate.is_file()):
        digest.update(str(item.relative_to(path)).encode())
        digest.update(item.read_bytes())
    return digest.hexdigest()


def command_sha256(command: list[str]) -> str:
    output = subprocess.run(command, cwd=REPO, check=True, stdout=subprocess.PIPE).stdout
    return hashlib.sha256(output).hexdigest()


def git_head() -> str:
    return subprocess.run(["git", "rev-parse", "HEAD"], cwd=REPO, check=True, text=True, stdout=subprocess.PIPE).stdout.strip()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO))


if __name__ == "__main__":
    raise SystemExit(main())
