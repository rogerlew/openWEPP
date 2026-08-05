#!/usr/bin/env python3
"""Run and summarize 21K canonical/scaled wet-compaction materiality lanes."""

from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import datetime as dt
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import sys
import time
from typing import Any


sys.dont_write_bytecode = True
REPO = Path(__file__).resolve().parents[4]
OUTPUT = REPO / "target/snow_wet_compaction_operand_closure"
BINARY = REPO / "target/release/openwepp-cli-hill"
PREVIOUS = REPO / "target/snow_prepeak_mass_transition_physics_adjudication_v2/runs"
PREVIOUS_RECEIPT = REPO / (
    "target/snow_prepeak_mass_transition_physics_adjudication_v2/"
    "execution-receipt.json"
)
PREVIOUS_MANIFEST = REPO / (
    "docs/work-packages/20260804-snow-prepeak-mass-transition-physics-"
    "adjudication-001/artifacts/evidence-manifest.json"
)
ADJUDICATION_TOOL = REPO / (
    "docs/work-packages/20260804-snow-prepeak-mass-transition-physics-"
    "adjudication-001/tools/run_adjudication.py"
)
SCALED_CLI = REPO / (
    "tests/fixtures/snotel_observed/snotel_snowbird_ut/development/"
    "precip_x1p2155576/p8.cli"
)
SELECTORS = {
    "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL": "layered_thermal_liquid_v1",
    "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL": "physics_bulk_multilayer_density_v1",
    "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL": "harder_pomeroy_hourly",
    "OPENWEPP_SNOWDENSITY1038_MELT_MODEL": "coe_liquid_holding_capacity_v1",
    "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": "disabled",
    "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL": "disabled",
}
UPSTREAM_MASS_FIELDS = (
    "snow_coupling_signed_s_m",
    "raw_melt_m",
    "snowpack_swe_loss_m",
    "rain_retained_m",
    "rain_released_m",
    "routed_melt_m",
    "runtime_swe_after_m",
    "stage3_incoming_liquid_m",
    "sublimation_m",
)
STAGE3_DISPOSITION_FIELDS = (
    "stage3_routed_liquid_m",
    "stage3_retained_liquid_delta_m",
    "stage3_refrozen_liquid_m",
)
ZERO = 1.0e-12
OPERAND_RECONSTRUCTION_TOLERANCE_M = 1.0e-12
MASS_CLOSURE_TOLERANCE_M = 1.0e-9
DENSITY_CLOSURE_TOLERANCE_KG_M3 = 1.0e-9


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


ADJ = load_module("snow_wet_compaction_adjudication_support", ADJUDICATION_TOOL)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def command_output(argv: list[str]) -> str:
    return subprocess.run(
        argv,
        cwd=REPO,
        check=True,
        text=True,
        capture_output=True,
    ).stdout.strip()


def workspace_identity() -> dict[str, Any]:
    diff = subprocess.run(
        ["git", "diff", "--binary", "HEAD"],
        cwd=REPO,
        check=True,
        capture_output=True,
    ).stdout
    untracked_raw = subprocess.run(
        ["git", "ls-files", "--others", "--exclude-standard", "-z"],
        cwd=REPO,
        check=True,
        capture_output=True,
    ).stdout
    untracked = []
    for raw_path in untracked_raw.split(b"\0"):
        if not raw_path:
            continue
        relative = Path(os.fsdecode(raw_path))
        path = REPO / relative
        if not path.is_file():
            raise RuntimeError(f"untracked source is not a regular file: {relative}")
        untracked.append(
            {
                "path": str(relative),
                "sha256": sha256(path),
                "size_bytes": path.stat().st_size,
            }
        )
    return {
        "head": command_output(["git", "rev-parse", "HEAD"]),
        "status_short": command_output(["git", "status", "--short"]),
        "head_diff_sha256": hashlib.sha256(diff).hexdigest(),
        "untracked_files": untracked,
    }


def binary_identity() -> dict[str, Any]:
    return {
        "path": str(BINARY.relative_to(REPO)),
        "sha256": sha256(BINARY),
        "size_bytes": BINARY.stat().st_size,
    }


def require_identity_unchanged(
    label: str,
    initial: dict[str, Any],
    terminal: dict[str, Any],
) -> None:
    if initial != terminal:
        raise RuntimeError(f"{label} identity changed during execution")


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(value, allow_nan=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def predecessor_identity(lanes: list[str]) -> dict[str, Any]:
    manifest = json.loads(PREVIOUS_MANIFEST.read_text(encoding="utf-8"))
    receipt_expected = manifest["tracked_results"]["execution_receipt_sha256"]
    receipt_actual = sha256(PREVIOUS_RECEIPT)
    if receipt_actual != receipt_expected:
        raise RuntimeError(
            "predecessor execution receipt does not match its tracked evidence manifest"
        )
    receipt = json.loads(PREVIOUS_RECEIPT.read_text(encoding="utf-8"))
    expected_by_lane: dict[str, dict[str, Any]] = {}
    for entry in manifest["exact_traces"]:
        trace = REPO / entry["path"]
        lane = trace.parent.name
        if lane in expected_by_lane:
            raise RuntimeError(f"duplicate predecessor trace entry for {lane}")
        expected_by_lane[lane] = entry
    if set(expected_by_lane) != set(lanes):
        raise RuntimeError(
            "predecessor evidence-manifest lanes do not match the selected canonical cohort"
        )
    trace_identity: dict[str, Any] = {}
    for lane in lanes:
        expected = expected_by_lane[lane]
        expected_path = PREVIOUS / lane / f"{lane}-adjudication.snow.jsonl"
        if (REPO / expected["path"]).resolve() != expected_path.resolve():
            raise RuntimeError(f"predecessor trace path mismatch for {lane}")
        actual_hash = sha256(expected_path)
        actual_size = expected_path.stat().st_size
        if actual_hash != expected["sha256"] or actual_size != expected["size_bytes"]:
            raise RuntimeError(f"predecessor trace identity mismatch for {lane}")
        trace_identity[lane] = {
            "path": expected["path"],
            "expected_sha256": expected["sha256"],
            "actual_sha256": actual_hash,
            "expected_size_bytes": expected["size_bytes"],
            "actual_size_bytes": actual_size,
        }
    if receipt["binary"]["sha256"] != manifest["binary_sha256"]:
        raise RuntimeError("predecessor receipt/manifest binary identity mismatch")
    return {
        "evidence_manifest": {
            "path": str(PREVIOUS_MANIFEST.relative_to(REPO)),
            "sha256": sha256(PREVIOUS_MANIFEST),
            "schema_version": manifest["schema_version"],
            "status": manifest["status"],
        },
        "execution_receipt": {
            "path": str(PREVIOUS_RECEIPT.relative_to(REPO)),
            "expected_sha256": receipt_expected,
            "actual_sha256": receipt_actual,
            "schema_version": receipt["schema_version"],
            "status": receipt["status"],
            "source_head": receipt["source_head"],
            "binary": receipt["binary"],
        },
        "traces": trace_identity,
    }


def climate_file(fixture: Path) -> Path:
    candidates = sorted(fixture.glob("*.cli"))
    if len(candidates) != 1:
        raise RuntimeError(f"expected one root climate file under {fixture}")
    return candidates[0]


def climate_dates(path: Path) -> list[dt.date]:
    dates: list[dt.date] = []
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            fields = line.split()
            if len(fields) != 13:
                continue
            try:
                stamp = dt.date(int(fields[2]), int(fields[1]), int(fields[0]))
                list(map(float, fields[3:]))
            except ValueError:
                continue
            dates.append(stamp)
    if not dates or dates != sorted(dates) or len(dates) != len(set(dates)):
        raise RuntimeError(f"invalid climate chronology: {path}")
    return dates


def sanitized_environment(trace: Path) -> tuple[dict[str, str], list[str]]:
    removed = sorted(key for key in os.environ if key.startswith("OPENWEPP_"))
    environment = {
        key: value for key, value in os.environ.items() if not key.startswith("OPENWEPP_")
    }
    effective = dict(SELECTORS)
    effective["OPENWEPP_R7H_SNOW_TRACE_PATH"] = str(trace.resolve())
    environment.update(effective)
    return environment, removed


def run_lane(lane_id: str, scaled: bool) -> dict[str, Any]:
    lane_name = f"{lane_id}__precip_x1p2155576" if scaled else lane_id
    source = REPO / "tests/fixtures/snotel_observed" / lane_id
    fixture = OUTPUT / "fixtures" / lane_name
    run_dir = OUTPUT / "runs" / lane_name
    shutil.copytree(source, fixture)
    canonical_cli = climate_file(fixture)
    source_cli_sha256 = sha256(canonical_cli)
    if scaled:
        if lane_id != "snotel_snowbird_ut":
            raise RuntimeError("scaled lane is Snowbird-only")
        shutil.copy2(SCALED_CLI, canonical_cli)
    staged_cli_sha256 = sha256(canonical_cli)
    run_dir.mkdir(parents=True)
    stem = f"{lane_name}-wet-compaction"
    runfile = run_dir / f"{stem}.run"
    trace = run_dir / f"{stem}.snow.jsonl"
    source_stem = ADJ.W1.eb04r.legacy.observed_harness.discover_run_stem(fixture)
    ADJ.W1.eb04r.legacy.observed_harness.write_runfile(
        runfile,
        fixture,
        source_stem,
        run_dir,
        stem,
    )
    command = ADJ.W1.eb04r.legacy.observed_harness.cli_command(
        BINARY,
        fixture,
        runfile,
        run_dir,
        "direct-production-executor",
    )
    environment, removed = sanitized_environment(trace)
    started = time.monotonic()
    completed = subprocess.run(
        command,
        cwd=REPO,
        env=environment,
        text=True,
        capture_output=True,
        check=False,
    )
    duration_seconds = time.monotonic() - started
    (run_dir / "stdout.txt").write_text(completed.stdout, encoding="utf-8")
    (run_dir / "stderr.txt").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise RuntimeError(f"run failed for {lane_name}: {completed.stderr[-2000:]}")
    outputs = {}
    for path in sorted(run_dir.glob(f"{stem}.*")):
        if path.is_file() and path != trace:
            outputs[path.name] = {
                "sha256": sha256(path),
                "size_bytes": path.stat().st_size,
            }
    return {
        "lane": lane_name,
        "classification": "DEVELOPMENT_ONLY" if scaled else "CANONICAL",
        "argv": [str(value) for value in command],
        "returncode": completed.returncode,
        "duration_seconds": duration_seconds,
        "removed_openwepp_key_names": removed,
        "effective_openwepp_environment": {
            **SELECTORS,
            "OPENWEPP_R7H_SNOW_TRACE_PATH": str(trace.resolve()),
        },
        "source_cli_sha256": source_cli_sha256,
        "staged_cli_sha256": staged_cli_sha256,
        "trace": str(trace.relative_to(REPO)),
        "trace_sha256": sha256(trace),
        "trace_size_bytes": trace.stat().st_size,
        "outputs": outputs,
    }


def generated_melt(row: dict[str, Any]) -> float:
    return sum(
        max(float(hour["coe_melt_applied_m"]), 0.0)
        for hour in row["accumulation_melt_hourly"]
    )


def layer_residuals(row: dict[str, Any]) -> tuple[float, float]:
    layers = row["snow_layers_after"]
    if not layers:
        return (0.0, 0.0)
    layer_swe = sum(float(layer["mass_swe_m"]) for layer in layers)
    layer_depth = sum(float(layer["thickness_m"]) for layer in layers)
    return (
        layer_swe - float(row["runtime_swe_after_m"]),
        layer_depth - float(row["runtime_depth_after_m"]),
    )


def paired_summary(
    lane: str,
    current_trace: Path,
    previous_trace: Path,
    dates: list[dt.date],
) -> tuple[dict[str, Any], list[tuple[dt.date, float]]]:
    summary: dict[str, Any] = {
        "lane": lane,
        "day_count": 0,
        "active_day_count": 0,
        "driver_changed_day_count": 0,
        "density_changed_day_count": 0,
        "total_previous_wet_compaction_input_m": 0.0,
        "total_current_wet_compaction_input_m": 0.0,
        "total_gross_positive_generated_melt_m": 0.0,
        "total_snow_contact_rain_m": 0.0,
        "max_abs_current_operand_reconstruction_m": 0.0,
        "max_abs_previous_duplicate_reconstruction_m": 0.0,
        "max_abs_density_process_closure_kg_m3": 0.0,
        "max_abs_layer_swe_residual_m": 0.0,
        "max_abs_layer_depth_residual_m": 0.0,
        "max_abs_density_delta_kg_m3": 0.0,
        "max_abs_depth_delta_m": 0.0,
        "max_abs_upstream_mass_delta": {
            field: 0.0 for field in UPSTREAM_MASS_FIELDS
        },
        "max_abs_stage3_disposition_delta_m": {
            field: 0.0 for field in STAGE3_DISPOSITION_FIELDS
        },
        "max_abs_current_stage3_liquid_closure_residual_m": 0.0,
    }
    daily_swe: list[tuple[dt.date, float]] = []
    with current_trace.open(encoding="utf-8") as current_handle, previous_trace.open(
        encoding="utf-8"
    ) as previous_handle:
        for index, (current_line, previous_line) in enumerate(
            zip(current_handle, previous_handle, strict=True)
        ):
            current = json.loads(current_line)
            previous = json.loads(previous_line)
            if int(current["day_index"]) != index or int(previous["day_index"]) != index:
                raise RuntimeError(f"day index mismatch in {lane} at {index}")
            gross = generated_melt(current)
            contact_rain = float(current["rain_retained_m"]) + float(
                current["rain_released_m"]
            )
            expected = gross + contact_rain
            actual = float(
                current["density_process_liquid_for_compaction_mass_kg_m2"]
            ) / 1000.0
            previous_actual = float(
                previous["density_process_liquid_for_compaction_mass_kg_m2"]
            ) / 1000.0
            previous_duplicate = float(previous["snowpack_swe_loss_m"]) + float(
                previous["routed_melt_m"]
            )
            layer_swe_residual, layer_depth_residual = layer_residuals(current)
            density_delta = float(current["runtime_density_after_kg_m3"]) - float(
                previous["runtime_density_after_kg_m3"]
            )
            depth_delta = float(current["runtime_depth_after_m"]) - float(
                previous["runtime_depth_after_m"]
            )
            summary["day_count"] += 1
            summary["active_day_count"] += int(bool(current["active_snow_coupling"]))
            summary["driver_changed_day_count"] += int(abs(actual - previous_actual) > ZERO)
            summary["density_changed_day_count"] += int(abs(density_delta) > ZERO)
            summary["total_previous_wet_compaction_input_m"] += previous_actual
            summary["total_current_wet_compaction_input_m"] += actual
            summary["total_gross_positive_generated_melt_m"] += gross
            summary["total_snow_contact_rain_m"] += contact_rain
            summary["max_abs_current_operand_reconstruction_m"] = max(
                summary["max_abs_current_operand_reconstruction_m"],
                abs(actual - expected),
            )
            summary["max_abs_previous_duplicate_reconstruction_m"] = max(
                summary["max_abs_previous_duplicate_reconstruction_m"],
                abs(previous_actual - previous_duplicate),
            )
            summary["max_abs_density_process_closure_kg_m3"] = max(
                summary["max_abs_density_process_closure_kg_m3"],
                abs(float(current["density_process_closure_residual_kg_m3"])),
            )
            summary["max_abs_layer_swe_residual_m"] = max(
                summary["max_abs_layer_swe_residual_m"], abs(layer_swe_residual)
            )
            summary["max_abs_layer_depth_residual_m"] = max(
                summary["max_abs_layer_depth_residual_m"], abs(layer_depth_residual)
            )
            summary["max_abs_density_delta_kg_m3"] = max(
                summary["max_abs_density_delta_kg_m3"], abs(density_delta)
            )
            summary["max_abs_depth_delta_m"] = max(
                summary["max_abs_depth_delta_m"], abs(depth_delta)
            )
            for field in UPSTREAM_MASS_FIELDS:
                summary["max_abs_upstream_mass_delta"][field] = max(
                    summary["max_abs_upstream_mass_delta"][field],
                    abs(float(current[field]) - float(previous[field])),
                )
            for field in STAGE3_DISPOSITION_FIELDS:
                summary["max_abs_stage3_disposition_delta_m"][field] = max(
                    summary["max_abs_stage3_disposition_delta_m"][field],
                    abs(float(current[field]) - float(previous[field])),
                )
            summary["max_abs_current_stage3_liquid_closure_residual_m"] = max(
                summary["max_abs_current_stage3_liquid_closure_residual_m"],
                abs(float(current["stage3_liquid_closure_residual_m"])),
            )
            daily_swe.append((dates[index], float(current["runtime_swe_after_m"])))
    if summary["day_count"] != len(dates):
        raise RuntimeError(
            f"trace/climate length mismatch for {lane}: "
            f"{summary['day_count']} trace rows vs {len(dates)} climate rows"
        )
    summary["wet_compaction_input_delta_m"] = (
        summary["total_current_wet_compaction_input_m"]
        - summary["total_previous_wet_compaction_input_m"]
    )
    summary["wet_compaction_input_ratio"] = (
        summary["total_current_wet_compaction_input_m"]
        / summary["total_previous_wet_compaction_input_m"]
        if summary["total_previous_wet_compaction_input_m"] > 0.0
        else None
    )
    return summary, daily_swe


def read_daily_swe(trace: Path, dates: list[dt.date]) -> list[tuple[dt.date, float]]:
    rows: list[tuple[dt.date, float]] = []
    with trace.open(encoding="utf-8") as handle:
        for index, line in enumerate(handle):
            row = json.loads(line)
            rows.append((dates[index], float(row["runtime_swe_after_m"])))
    if len(rows) != len(dates):
        raise RuntimeError(f"trace/climate length mismatch: {trace}")
    return rows


def water_year(date: dt.date) -> int:
    return date.year + int(date.month >= 10)


def peak_summary(rows: list[tuple[dt.date, float]]) -> dict[int, tuple[dt.date, float]]:
    peaks: dict[int, tuple[dt.date, float]] = {}
    for date, swe in rows:
        year = water_year(date)
        current = peaks.get(year)
        if current is None or swe > current[1]:
            peaks[year] = (date, swe)
    return peaks


def scaled_comparison(
    canonical: list[tuple[dt.date, float]],
    scaled: list[tuple[dt.date, float]],
) -> dict[str, Any]:
    canonical_peaks = peak_summary(canonical)
    scaled_peaks = peak_summary(scaled)
    years = sorted((set(canonical_peaks) & set(scaled_peaks)) - {2025})
    deltas = [scaled_peaks[year][1] - canonical_peaks[year][1] for year in years]
    ratios = [
        scaled_peaks[year][1] / canonical_peaks[year][1]
        for year in years
        if canonical_peaks[year][1] > ZERO
    ]
    timing = [
        (scaled_peaks[year][0] - canonical_peaks[year][0]).days for year in years
    ]
    return {
        "classification": "DEVELOPMENT_ONLY",
        "claim_limit": (
            "The scaled lane characterizes Snowbird input sensitivity only; it cannot "
            "prove wet-compaction physics, forcing truth, calibration, default, or transferability."
        ),
        "matched_water_year_count": len(years),
        "median_peak_swe_delta_m": statistics.median(deltas),
        "median_peak_swe_ratio": statistics.median(ratios),
        "minimum_peak_swe_ratio": min(ratios),
        "maximum_peak_swe_ratio": max(ratios),
        "median_peak_timing_delta_days": statistics.median(timing),
    }


def require_materiality_acceptance(summaries: list[dict[str, Any]]) -> dict[str, Any]:
    metrics = {
        "max_operand_reconstruction_m": max(
            row["max_abs_current_operand_reconstruction_m"] for row in summaries
        ),
        "max_previous_duplicate_reconstruction_m": max(
            row["max_abs_previous_duplicate_reconstruction_m"] for row in summaries
        ),
        "max_upstream_mass_delta_m": max(
            value
            for row in summaries
            for value in row["max_abs_upstream_mass_delta"].values()
        ),
        "max_current_stage3_liquid_closure_residual_m": max(
            row["max_abs_current_stage3_liquid_closure_residual_m"]
            for row in summaries
        ),
        "max_density_process_closure_kg_m3": max(
            row["max_abs_density_process_closure_kg_m3"] for row in summaries
        ),
        "max_layer_swe_residual_m": max(
            row["max_abs_layer_swe_residual_m"] for row in summaries
        ),
        "max_layer_depth_residual_m": max(
            row["max_abs_layer_depth_residual_m"] for row in summaries
        ),
        "canonical_driver_changed_day_count": sum(
            int(row["driver_changed_day_count"]) for row in summaries
        ),
        "canonical_density_changed_day_count": sum(
            int(row["density_changed_day_count"]) for row in summaries
        ),
    }
    bounded_checks = (
        (
            "max_operand_reconstruction_m",
            OPERAND_RECONSTRUCTION_TOLERANCE_M,
        ),
        (
            "max_previous_duplicate_reconstruction_m",
            OPERAND_RECONSTRUCTION_TOLERANCE_M,
        ),
        ("max_upstream_mass_delta_m", MASS_CLOSURE_TOLERANCE_M),
        (
            "max_current_stage3_liquid_closure_residual_m",
            MASS_CLOSURE_TOLERANCE_M,
        ),
        (
            "max_density_process_closure_kg_m3",
            DENSITY_CLOSURE_TOLERANCE_KG_M3,
        ),
        ("max_layer_swe_residual_m", MASS_CLOSURE_TOLERANCE_M),
        ("max_layer_depth_residual_m", MASS_CLOSURE_TOLERANCE_M),
    )
    failures = []
    for metric, limit in bounded_checks:
        observed = float(metrics[metric])
        if not math.isfinite(observed) or observed > limit:
            failures.append(f"{metric}={observed} exceeds {limit}")
    for metric in (
        "canonical_driver_changed_day_count",
        "canonical_density_changed_day_count",
    ):
        if int(metrics[metric]) <= 0:
            failures.append(f"{metric} must be positive")
    if failures:
        raise RuntimeError("materiality acceptance failed: " + "; ".join(failures))
    return {
        "status": "PASS",
        "limits": {
            "operand_reconstruction_m": OPERAND_RECONSTRUCTION_TOLERANCE_M,
            "mass_and_layer_closure_m": MASS_CLOSURE_TOLERANCE_M,
            "density_process_closure_kg_m3": DENSITY_CLOSURE_TOLERANCE_KG_M3,
            "minimum_driver_changed_day_count": 1,
            "minimum_density_changed_day_count": 1,
        },
        **metrics,
    }


def analyze(
    receipts: dict[str, Any],
    execution_source: dict[str, Any],
    execution_binary: dict[str, Any],
    execution_tool_sha256: str,
    execution_receipt_sha256: str,
) -> None:
    analysis_source = workspace_identity()
    analysis_binary = binary_identity()
    analysis_tool_sha256 = sha256(Path(__file__))
    lanes = [lane.lane_id for lane in ADJ.W1.selected_lanes()]
    predecessor = predecessor_identity(lanes)
    summaries: list[dict[str, Any]] = []
    canonical_snowbird: list[tuple[dt.date, float]] | None = None
    for lane in lanes:
        lane_name = lane
        current_trace = REPO / receipts[lane_name]["trace"]
        if sha256(current_trace) != receipts[lane_name]["trace_sha256"]:
            raise RuntimeError(f"current trace identity changed: {current_trace}")
        previous_trace = PREVIOUS / lane / f"{lane}-adjudication.snow.jsonl"
        fixture = OUTPUT / "fixtures" / lane_name
        dates = climate_dates(climate_file(fixture))
        summary, daily_swe = paired_summary(lane, current_trace, previous_trace, dates)
        summaries.append(summary)
        if lane == "snotel_snowbird_ut":
            canonical_snowbird = daily_swe

    scaled_name = "snotel_snowbird_ut__precip_x1p2155576"
    scaled_trace = REPO / receipts[scaled_name]["trace"]
    if sha256(scaled_trace) != receipts[scaled_name]["trace_sha256"]:
        raise RuntimeError(f"scaled trace identity changed: {scaled_trace}")
    scaled_fixture = OUTPUT / "fixtures" / scaled_name
    scaled_snowbird = read_daily_swe(
        scaled_trace,
        climate_dates(climate_file(scaled_fixture)),
    )
    if canonical_snowbird is None:
        raise RuntimeError("canonical Snowbird lane missing")
    acceptance = require_materiality_acceptance(summaries)
    stage3_disposition_observation = {
        "max_stage3_disposition_delta_m": max(
            value
            for row in summaries
            for value in row["max_abs_stage3_disposition_delta_m"].values()
        ),
        "interpretation": (
            "Observed density-mediated Stage-3 routing/store/refreeze response; "
            "not an upstream mass-invariance gate."
        ),
    }
    result = {
        "schema": "snow-wet-compaction-operand-materiality-v2",
        "evidence_mode": (
            "Ran: receipt-bound release CLI execution paired to hash-verified "
            "predecessor traces"
        ),
        "execution_source": execution_source,
        "execution_receipt": {
            "path": str((OUTPUT / "execution-receipt.json").relative_to(REPO)),
            "sha256": execution_receipt_sha256,
            "tool_sha256": execution_tool_sha256,
        },
        "analysis_context": {
            "source": analysis_source,
            "binary": analysis_binary,
            "tool_sha256": analysis_tool_sha256,
        },
        "execution_binary": execution_binary,
        "predecessor_evidence": predecessor,
        "canonical_site_summaries": summaries,
        "snowbird_scaled_comparison": scaled_comparison(
            canonical_snowbird,
            scaled_snowbird,
        ),
        "acceptance": acceptance,
        "observations": stage3_disposition_observation,
    }
    result_path = OUTPUT / "results/materiality.json"
    pending_result_path = result_path.with_suffix(".json.pending")
    write_json(pending_result_path, result)
    require_identity_unchanged("analysis workspace", analysis_source, workspace_identity())
    require_identity_unchanged("analysis binary", analysis_binary, binary_identity())
    if analysis_tool_sha256 != sha256(Path(__file__)):
        raise RuntimeError("analysis tool identity changed during analysis")
    pending_result_path.replace(result_path)


def execute() -> None:
    if OUTPUT.exists():
        raise RuntimeError(f"refusing to overwrite {OUTPUT}")
    if not BINARY.is_file():
        raise RuntimeError(f"release binary missing: {BINARY}")
    initial_source = workspace_identity()
    initial_binary = binary_identity()
    execution_tool_sha256 = sha256(Path(__file__))
    OUTPUT.mkdir(parents=True)
    lanes = [(lane.lane_id, False) for lane in ADJ.W1.selected_lanes()]
    lanes.append(("snotel_snowbird_ut", True))
    receipts: dict[str, Any] = {}
    with ThreadPoolExecutor(max_workers=2) as executor:
        futures = {
            executor.submit(run_lane, lane, scaled): (lane, scaled)
            for lane, scaled in lanes
        }
        for future in as_completed(futures):
            receipt = future.result()
            receipts[receipt["lane"]] = receipt
    require_identity_unchanged("workspace", initial_source, workspace_identity())
    require_identity_unchanged("release binary", initial_binary, binary_identity())
    receipt_path = OUTPUT / "execution-receipt.json"
    write_json(
        receipt_path,
        {
            "schema": "snow-wet-compaction-operand-execution-receipt-v2",
            "source": initial_source,
            "binary": initial_binary,
            "tool_sha256": execution_tool_sha256,
            "lanes": receipts,
        },
    )
    analyze(
        receipts,
        initial_source,
        initial_binary,
        execution_tool_sha256,
        sha256(receipt_path),
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--execute", action="store_true")
    group.add_argument("--analyze-existing", action="store_true")
    args = parser.parse_args()
    if args.execute:
        execute()
    else:
        receipt_path = OUTPUT / "execution-receipt.json"
        receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
        if receipt["schema"] != "snow-wet-compaction-operand-execution-receipt-v2":
            raise RuntimeError("existing execution receipt is not custody-safe schema v2")
        receipts = receipt["lanes"]
        analyze(
            receipts,
            receipt["source"],
            receipt["binary"],
            receipt["tool_sha256"],
            sha256(receipt_path),
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
