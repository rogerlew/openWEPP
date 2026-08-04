#!/usr/bin/env python3
"""Run frozen selector probes and reconstruct pre-peak snow ledgers."""

from __future__ import annotations

import csv
import datetime as dt
import argparse
import hashlib
import importlib.util
import json
import math
import os
import shutil
import statistics
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
FREEZE = PACKAGE / "artifacts/audit-freeze-v3.json"
OUTPUT = REPO / "target/snow_prepeak_liquid_evacuation_physics_audit_v3"
RUNS = OUTPUT / "runs"
FIXTURES = OUTPUT / "fixtures"
RESULTS = OUTPUT / "dynamic-results.json"
RECEIPT = OUTPUT / "execution-receipt.json"
BINARY = REPO / "target/release/openwepp-cli-hill"
PREDECESSOR = REPO / "target/snowbird_rst_prepeak_flux_diagnostic"
W1_TOOL = REPO / (
    "docs/work-packages/20260802-snow-surface-eb-04w1-"
    "precipitation-scaling-calibration-001/tools/run_precipitation_scaling.py"
)
HELPER_DEPENDENCIES = {
    "eb04w1_precipitation_tool": W1_TOOL,
    "eb04w_accumulation_tool": REPO
    / "docs/work-packages/20260801-snow-surface-eb-04w-accumulation-under-persistence-001/tools/run_accumulation_diagnostics.py",
    "eb04r_experiment_tool": REPO
    / "docs/work-packages/20260801-snow-surface-eb-04r-fresh-factorial-execution-adjudication-001/tools/run_experiment.py",
    "eb04_factorial_tool": REPO
    / "docs/work-packages/20260730-snow-surface-eb-04-factorial-execution-adjudication-001/tools/run_factorial.py",
    "eb04e_qualification_tool": REPO
    / "docs/work-packages/20260731-snow-surface-eb-04e-corrected-population-runtime-qualification-001/tools/run_qualification.py",
    "eb04e_retained_verifier": REPO
    / "docs/work-packages/20260731-snow-surface-eb-04e-corrected-population-runtime-qualification-001/tools/verify_retained_outputs.py",
    "cross_snotel_mechanism_rubric": REPO
    / "tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py",
    "observed_harness": REPO / "tools/snowfreeze_observed/observed_harness.py",
    "snotel_density_three_way": REPO
    / "tools/snowfreeze_observed/snotel_density_three_way.py",
}
UPSTREAM_DIAGNOSTICS = {
    "radiation_results": REPO
    / "docs/work-packages/20260803-snow-hourly-era5-diagnostic-001/artifacts/radiation-first-results.json",
    "radiation_manifest": REPO
    / "docs/work-packages/20260803-snow-hourly-era5-diagnostic-001/artifacts/radiation-comparison-manifest.json",
    "radiation_tool": REPO
    / "docs/work-packages/20260803-snow-hourly-era5-diagnostic-001/tools/compare_radiation_first.py",
    "cloud_results": REPO
    / "docs/work-packages/20260803-snow-hourly-era5-cloud-proxy-sanity-001/artifacts/cloud-proxy-results.json",
    "cloud_manifest": REPO
    / "docs/work-packages/20260803-snow-hourly-era5-cloud-proxy-sanity-001/artifacts/cloud-comparison-manifest.json",
    "cloud_tool": REPO
    / "docs/work-packages/20260803-snow-hourly-era5-cloud-proxy-sanity-001/tools/compare_cloud_proxy.py",
}
BASELINE_ENV = {
    "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL": "layered_thermal_liquid_v1",
    "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL": "physics_bulk_multilayer_density_v1",
    "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL": "harder_pomeroy_hourly",
    "OPENWEPP_SNOWDENSITY1038_MELT_MODEL": "coe_liquid_holding_capacity_v1",
    "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": "disabled",
    "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL": "disabled",
}
SELECTED_OPERATORS = {
    "stage3_disabled": {
        "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL": "disabled",
    },
    "legacy_coe_routing": {
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL": "legacy_coe",
    },
    "surface_longwave_enabled": {
        "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": "dilley_unsworth_subcanopy_v1",
    },
}
EXECUTION_OPERATORS = {"baseline_replay": {}, **SELECTED_OPERATORS}
MASS_FIELDS = (
    "accumulation_m",
    "rain_retained_m",
    "snowpack_swe_loss_m",
    "sublimation_m",
    "raw_melt_m",
    "routed_melt_m",
    "liquid_water_released_m",
    "rain_released_m",
    "stage3_refrozen_liquid_m",
)
ENERGY_FIELDS = (
    "stage3_shortwave_energy_j_m2",
    "stage3_longwave_energy_j_m2",
    "stage3_surface_energy_j_m2",
    "stage3_conduction_energy_j_m2",
    "stage3_latent_refreeze_energy_j_m2",
    "stage3_unused_positive_energy_j_m2",
    "stage3_cold_content_export_j_m2",
)
COMPONENT_FIELDS = (
    "coe_melt_amelt_m",
    "coe_melt_bmelt_m",
    "coe_melt_cmelt_m",
    "coe_melt_dmelt_m",
)
ZERO = 1.0e-12
EVENT_THRESHOLD_M = 0.0005
EVENT_SENSITIVITY_THRESHOLDS_M = (0.0001, 0.0005, 0.001)
EVENT_TOP_N = 3
CENSORED_WATER_YEARS = (2025,)
PINNED_LEGACY_REPO = Path("/workdir/wepp-forest_260430_baseline")
PINNED_LEGACY_COMMIT = "dac3c950d8b16cc73774bf5ce2e7e11f80baac70"
FIXED_COMPARATOR_REPO = Path("/workdir/wepp-forest")
FIXED_COMPARATOR_COMMIT = "47ac4c32faeea81bb99081f955a14c38b815ef4d"


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


w1 = load_module("snow_prepeak_w1", W1_TOOL)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def command_output(argv: list[str]) -> str:
    return subprocess.run(
        argv, check=True, text=True, capture_output=True
    ).stdout


def file_manifest(root: Path) -> dict[str, Any]:
    files = []
    for path in sorted(candidate for candidate in root.rglob("*") if candidate.is_file()):
        files.append(
            {
                "path": str(path.relative_to(root)),
                "sha256": sha256(path),
                "size_bytes": path.stat().st_size,
            }
        )
    payload = json.dumps(files, sort_keys=True, separators=(",", ":")).encode()
    return {
        "file_count": len(files),
        "files": files,
        "manifest_sha256": hashlib.sha256(payload).hexdigest(),
    }


def git_blob_identity(repo: Path, commit: str, path: str) -> dict[str, str]:
    object_name = command_output(
        ["git", "-C", str(repo), "rev-parse", f"{commit}:{path}"]
    ).strip()
    contents = subprocess.run(
        ["git", "-C", str(repo), "show", f"{commit}:{path}"],
        check=True,
        capture_output=True,
    ).stdout
    return {
        "blob": object_name,
        "commit": commit,
        "path": path,
        "sha256": hashlib.sha256(contents).hexdigest(),
    }


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def climate_file(path: Path) -> Path:
    files = sorted(path.glob("*.cli"))
    if len(files) != 1:
        raise RuntimeError(f"expected exactly one climate file under {path}")
    return files[0]


def climate_dates(path: Path) -> list[dt.date]:
    dates: list[dt.date] = []
    for line in path.read_text(encoding="utf-8").splitlines():
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


def observed_peaks(path: Path) -> tuple[dict[int, tuple[dt.date, float]], list[int]]:
    peaks: dict[int, tuple[dt.date, float]] = {}
    with path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            raw = row.get("observed_swe_mm")
            if raw in (None, ""):
                continue
            water_year = int(row["water_year"])
            candidate = (dt.date.fromisoformat(row["date"]), float(raw) / 1000.0)
            current = peaks.get(water_year)
            if current is None or candidate[1] > current[1]:
                peaks[water_year] = candidate
    skipped = sorted(year for year, (_, value) in peaks.items() if value <= 0.0)
    return ({year: peak for year, peak in peaks.items() if peak[1] > 0.0}, skipped)


def sanitized_environment(trace: Path, changes: dict[str, str]) -> tuple[dict[str, str], list[str], dict[str, str]]:
    removed = sorted(key for key in os.environ if key.startswith("OPENWEPP_"))
    environment = {
        key: value for key, value in os.environ.items() if not key.startswith("OPENWEPP_")
    }
    effective = dict(BASELINE_ENV)
    effective.update(changes)
    effective["OPENWEPP_R7H_SNOW_TRACE_PATH"] = str(trace.resolve())
    environment.update(effective)
    observed = {key: value for key, value in environment.items() if key.startswith("OPENWEPP_")}
    if observed != effective:
        raise RuntimeError("OPENWEPP environment sanitizer did not produce the exact mapping")
    return environment, removed, effective


def execute_cell(lane: Any, operator: str, changes: dict[str, str]) -> dict[str, Any]:
    source_fixture = PREDECESSOR / "fixtures" / lane.lane_id / "rst_1p0"
    fixture = FIXTURES / operator / lane.lane_id
    shutil.copytree(source_fixture, fixture)
    source_fixture_manifest = file_manifest(source_fixture)
    copied_fixture_manifest = file_manifest(fixture)
    if copied_fixture_manifest != source_fixture_manifest:
        raise RuntimeError(f"fixture copy differs for {operator}/{lane.lane_id}")
    run_dir = RUNS / operator / lane.lane_id
    run_dir.mkdir(parents=True)
    stem = f"{lane.lane_id}-{operator}"
    runfile = run_dir / f"{stem}.run"
    trace = run_dir / f"{stem}.snow.jsonl"
    source_stem = w1.eb04r.legacy.observed_harness.discover_run_stem(fixture)
    w1.eb04r.legacy.observed_harness.write_runfile(
        runfile, fixture, source_stem, run_dir, stem
    )
    command = w1.eb04r.legacy.observed_harness.cli_command(
        BINARY, fixture, runfile, run_dir, "direct-production-executor"
    )
    environment, removed, effective = sanitized_environment(trace, changes)
    completed = subprocess.run(
        command,
        cwd=REPO,
        env=environment,
        text=True,
        capture_output=True,
        check=False,
    )
    stdout = run_dir / "stdout.txt"
    stderr = run_dir / "stderr.txt"
    stdout.write_text(completed.stdout, encoding="utf-8")
    stderr.write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise RuntimeError(
            f"openWEPP failed for {operator}/{lane.lane_id}: {completed.stderr[-2000:]}"
        )
    wat = run_dir / f"{stem}.wat.parquet"
    return {
        "argv": [str(value) for value in command],
        "binary_sha256": sha256(BINARY),
        "climate_sha256": sha256(climate_file(fixture)),
        "effective_openwepp_environment": effective,
        "fixture_snow_sha256": sha256(fixture / "snow.txt"),
        "source_fixture_manifest": source_fixture_manifest,
        "copied_fixture_manifest": copied_fixture_manifest,
        "runfile_path": relative(runfile),
        "runfile_sha256": sha256(runfile),
        "removed_openwepp_key_names": removed,
        "returncode": completed.returncode,
        "stderr_sha256": sha256(stderr),
        "stdout_sha256": sha256(stdout),
        "trace_path": relative(trace),
        "trace_sha256": sha256(trace),
        "wat_path": relative(wat),
        "wat_sha256": sha256(wat),
    }


def trace_rows(path: Path, dates: list[dt.date]) -> list[tuple[dt.date, dict[str, Any]]]:
    rows = [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines()]
    if len(rows) != len(dates):
        raise RuntimeError(f"trace/climate row mismatch: {path}")
    return list(zip(dates, rows))


def hourly_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    components = {field: 0.0 for field in COMPONENT_FIELDS}
    positive_applied_m = 0.0
    negative_applied_m = 0.0
    uncapped_m = 0.0
    cap_adjustment_m = 0.0
    hours = 0
    for row in rows:
        for hour in row["accumulation_melt_hourly"]:
            hours += 1
            for field in COMPONENT_FIELDS:
                components[field] += float(hour[field])
            applied = float(hour["coe_melt_applied_m"])
            positive_applied_m += max(applied, 0.0)
            negative_applied_m += min(applied, 0.0)
            uncapped_m += float(hour["coe_melt_uncapped_m"])
            cap_adjustment_m += float(hour["coe_melt_cap_adjustment_m"])
    return {
        "hour_count": hours,
        "component_signed_sums_m": components,
        "positive_applied_m": positive_applied_m,
        "negative_applied_m": negative_applied_m,
        "uncapped_m": uncapped_m,
        "cap_adjustment_m": cap_adjustment_m,
    }


def aggregate_event(rows: list[tuple[dt.date, dict[str, Any]]]) -> dict[str, Any]:
    values = [row for _, row in rows]
    return {
        "start": rows[0][0].isoformat(),
        "end": rows[-1][0].isoformat(),
        "day_count": len(rows),
        "snowpack_swe_loss_m": sum(float(row["snowpack_swe_loss_m"]) for row in values),
        "accumulation_m": sum(float(row["accumulation_m"]) for row in values),
        "rain_retained_m": sum(float(row["rain_retained_m"]) for row in values),
        "rain_released_m": sum(float(row["rain_released_m"]) for row in values),
        "liquid_water_released_m": sum(float(row["liquid_water_released_m"]) for row in values),
        "stage3_refrozen_liquid_m": sum(float(row["stage3_refrozen_liquid_m"]) for row in values),
        "pack_exhaustion_days": sum(float(row["runtime_swe_after_m"]) <= ZERO for row in values),
        "hourly": hourly_summary(values),
    }


def loss_events(
    window: list[tuple[dt.date, dict[str, Any]]],
    threshold_m: float,
    top_n: int | None = EVENT_TOP_N,
) -> list[dict[str, Any]]:
    events: list[list[tuple[dt.date, dict[str, Any]]]] = []
    current: list[tuple[dt.date, dict[str, Any]]] = []
    for dated in window:
        active = float(dated[1]["snowpack_swe_loss_m"]) >= threshold_m
        if active:
            if current and (dated[0] - current[-1][0]).days > 1:
                events.append(current)
                current = []
            current.append(dated)
        elif current:
            events.append(current)
            current = []
    if current:
        events.append(current)
    aggregated = [aggregate_event(event) for event in events]
    ranked = sorted(
        aggregated, key=lambda row: row["snowpack_swe_loss_m"], reverse=True
    )
    return ranked if top_n is None else ranked[:top_n]


def event_sensitivity(
    window: list[tuple[dt.date, dict[str, Any]]]
) -> dict[str, dict[str, float | int]]:
    output: dict[str, dict[str, float | int]] = {}
    total_loss = sum(float(row["snowpack_swe_loss_m"]) for _, row in window)
    for threshold in EVENT_SENSITIVITY_THRESHOLDS_M:
        eligible = loss_events(window, threshold, top_n=None)
        selected = eligible[:EVENT_TOP_N]
        selected_loss = sum(float(row["snowpack_swe_loss_m"]) for row in selected)
        eligible_loss = sum(float(row["snowpack_swe_loss_m"]) for row in eligible)
        output[f"{threshold:.4f}"] = {
            "eligible_event_count": len(eligible),
            "eligible_event_loss_m": eligible_loss,
            "fraction_window_loss_in_all_eligible_events": (
                eligible_loss / total_loss if total_loss > ZERO else 0.0
            ),
            "selected_event_count": len(selected),
            "selected_event_loss_m": selected_loss,
            "fraction_window_loss_in_selected_events": (
                selected_loss / total_loss if total_loss > ZERO else 0.0
            ),
        }
    return output


def daily_forensic_signatures(rows: list[dict[str, Any]]) -> dict[str, Any]:
    mixed_signed_hour_days = 0
    mixed_export_refreeze_days = 0
    negative_daily_net_with_loss_days = 0
    negative_daily_net_with_loss_m = 0.0
    wet_input_residuals = []
    duplicate_loss_components = []
    routed_alias_residuals = []
    for row in rows:
        hourly = hourly_summary([row])
        mixed = (
            hourly["positive_applied_m"] > ZERO
            and hourly["negative_applied_m"] < -ZERO
        )
        if mixed:
            mixed_signed_hour_days += 1
        if (
            mixed
            and float(row["routed_melt_m"]) > ZERO
            and float(row["stage3_refrozen_liquid_m"]) > ZERO
        ):
            mixed_export_refreeze_days += 1
        signed_applied = (
            hourly["positive_applied_m"] + hourly["negative_applied_m"]
        )
        loss = float(row["snowpack_swe_loss_m"])
        if signed_applied <= ZERO and loss > ZERO:
            negative_daily_net_with_loss_days += 1
            negative_daily_net_with_loss_m += loss
        wet_input_m = (
            float(row["density_process_liquid_for_compaction_mass_kg_m2"])
            / 1000.0
        )
        rain_released = float(row["rain_released_m"])
        routed = float(row["routed_melt_m"])
        wet_input_residuals.append(wet_input_m - (2.0 * loss + rain_released))
        duplicate_loss_components.append(wet_input_m - routed)
        routed_alias_residuals.append(routed - (loss + rain_released))
    return {
        "mixed_signed_hour_days": mixed_signed_hour_days,
        "mixed_export_refreeze_days": mixed_export_refreeze_days,
        "negative_daily_net_with_pack_loss_days": negative_daily_net_with_loss_days,
        "negative_daily_net_with_pack_loss_m": negative_daily_net_with_loss_m,
        "wet_compaction_input_identity": (
            "density_process_liquid_for_compaction_mass_kg_m2 / 1000 "
            "= 2 * snowpack_swe_loss_m + rain_released_m"
        ),
        "maximum_abs_wet_compaction_input_identity_residual_m": max(
            map(abs, wet_input_residuals), default=0.0
        ),
        "maximum_abs_routed_alias_residual_m": max(
            map(abs, routed_alias_residuals), default=0.0
        ),
        "duplicate_state_loss_component_m": sum(duplicate_loss_components),
    }


def analyze_trace(lane: Any, trace: Path, fixture: Path) -> dict[str, Any]:
    dates = climate_dates(climate_file(fixture))
    dated_rows = trace_rows(trace, dates)
    by_date = dict(dated_rows)
    peaks, skipped_zero = observed_peaks(Path(lane.observation_file))
    annual: list[dict[str, Any]] = []
    maximum_daily_closure_m = 0.0
    maximum_window_closure_m = 0.0
    for water_year, (peak_date, observed_peak_swe_m) in sorted(peaks.items()):
        start = dt.date(water_year - 1, 10, 1)
        if start not in by_date or peak_date not in by_date:
            continue
        window = [(date, row) for date, row in dated_rows if start <= date <= peak_date]
        rows = [row for _, row in window]
        sums = {field: sum(float(row[field]) for row in rows) for field in MASS_FIELDS}
        energy = {field: sum(float(row[field]) for row in rows) for field in ENERGY_FIELDS}
        independent_energy_residuals = [
            float(row["stage3_surface_energy_j_m2"])
            + float(row["stage3_conduction_energy_j_m2"])
            + float(row["stage3_latent_refreeze_energy_j_m2"])
            + float(row["stage3_cold_content_export_j_m2"])
            - (
                float(row["stage3_cold_content_before_j_m2"])
                - float(row["stage3_cold_content_after_j_m2"])
            )
            for row in rows
        ]
        energy_trace_differences = [
            reconstructed - float(row["stage3_energy_closure_residual_j_m2"])
            for row, reconstructed in zip(rows, independent_energy_residuals)
        ]
        daily_closures = []
        for row in rows:
            delta = float(row["runtime_swe_after_m"]) - float(row["runtime_swe_before_m"])
            expected = (
                float(row["accumulation_m"])
                + float(row["rain_retained_m"])
                - float(row["snowpack_swe_loss_m"])
                - float(row["sublimation_m"])
            )
            daily_closures.append(delta - expected)
        initial_swe_m = float(rows[0]["runtime_swe_before_m"])
        final_swe_m = float(rows[-1]["runtime_swe_after_m"])
        window_closure_m = final_swe_m - initial_swe_m - (
            sums["accumulation_m"]
            + sums["rain_retained_m"]
            - sums["snowpack_swe_loss_m"]
            - sums["sublimation_m"]
        )
        maximum_daily_closure_m = max(maximum_daily_closure_m, max(map(abs, daily_closures)))
        maximum_window_closure_m = max(maximum_window_closure_m, abs(window_closure_m))
        modeled_peak_date, modeled_peak_swe_m = max(
            ((date, float(row["runtime_swe_after_m"])) for date, row in window),
            key=lambda item: item[1],
        )
        loss_on_snowfall_days_m = sum(
            float(row["snowpack_swe_loss_m"])
            for row in rows
            if float(row["accumulation_m"]) > ZERO
        )
        loss_on_rain_days_m = sum(
            float(row["snowpack_swe_loss_m"])
            for row in rows
            if float(row["rain_retained_m"]) + float(row["rain_released_m"]) > ZERO
        )
        loss_on_no_precip_days_m = sum(
            float(row["snowpack_swe_loss_m"])
            for row in rows
            if float(row["accumulation_m"]) <= ZERO
            and float(row["rain_retained_m"]) + float(row["rain_released_m"]) <= ZERO
        )
        loss_on_pack_exhaustion_days_m = sum(
            float(row["snowpack_swe_loss_m"])
            for row in rows
            if float(row["runtime_swe_after_m"]) <= ZERO
        )
        annual.append(
            {
                "water_year": water_year,
                "right_censored": water_year in CENSORED_WATER_YEARS,
                "window_start": start.isoformat(),
                "observed_peak_date": peak_date.isoformat(),
                "observed_peak_swe_m": observed_peak_swe_m,
                "modeled_peak_date": modeled_peak_date.isoformat(),
                "modeled_peak_swe_m": modeled_peak_swe_m,
                "peak_swe_ratio": modeled_peak_swe_m / observed_peak_swe_m,
                "peak_date_offset_days": (modeled_peak_date - peak_date).days,
                "initial_swe_m": initial_swe_m,
                "final_swe_m": final_swe_m,
                "storage_change_m": final_swe_m - initial_swe_m,
                **sums,
                "energy_boundary": energy,
                "maximum_abs_independent_stage3_energy_residual_j_m2": max(
                    map(abs, independent_energy_residuals), default=0.0
                ),
                "maximum_abs_stage3_energy_trace_difference_j_m2": max(
                    map(abs, energy_trace_differences), default=0.0
                ),
                "hourly_coe": hourly_summary(rows),
                "loss_on_snowfall_days_m": loss_on_snowfall_days_m,
                "loss_on_rain_days_m": loss_on_rain_days_m,
                "loss_on_no_precip_days_m": loss_on_no_precip_days_m,
                "loss_on_pack_exhaustion_days_m": loss_on_pack_exhaustion_days_m,
                "maximum_liquid_holding_capacity_m": max(
                    float(row["liquid_holding_capacity_after_m"]) for row in rows
                ),
                "maximum_retained_liquid_m": max(
                    float(row["liquid_water_retained_after_m"]) for row in rows
                ),
                "window_mass_closure_m": window_closure_m,
                "maximum_daily_mass_closure_m": max(map(abs, daily_closures)),
                "forensic_signatures": daily_forensic_signatures(rows),
                "top_loss_events": loss_events(window, EVENT_THRESHOLD_M),
                "event_selection_sensitivity": event_sensitivity(window),
            }
        )
    if not annual:
        raise RuntimeError(f"no positive observed-peak windows for {lane.lane_id}")
    median_fields = (
        "peak_swe_ratio",
        "peak_date_offset_days",
        "accumulation_m",
        "snowpack_swe_loss_m",
        "raw_melt_m",
        "routed_melt_m",
        "liquid_water_released_m",
        "rain_retained_m",
        "rain_released_m",
        "stage3_refrozen_liquid_m",
        "loss_on_snowfall_days_m",
        "loss_on_rain_days_m",
        "loss_on_no_precip_days_m",
        "loss_on_pack_exhaustion_days_m",
        "maximum_liquid_holding_capacity_m",
        "maximum_retained_liquid_m",
    )
    def summarize(selected: list[dict[str, Any]]) -> dict[str, Any]:
        if not selected:
            raise RuntimeError(f"no uncensored observed-peak windows for {lane.lane_id}")
        output = {
            "annual_count": len(selected),
            "water_years": [int(row["water_year"]) for row in selected],
            **{
                f"median_{field}": statistics.median(
                    float(row[field]) for row in selected
                )
                for field in median_fields
            },
            "maximum_daily_mass_closure_m": max(
                float(row["maximum_daily_mass_closure_m"]) for row in selected
            ),
            "maximum_window_mass_closure_m": max(
                abs(float(row["window_mass_closure_m"])) for row in selected
            ),
            "total_mixed_signed_hour_days": sum(
                int(row["forensic_signatures"]["mixed_signed_hour_days"])
                for row in selected
            ),
            "total_mixed_export_refreeze_days": sum(
                int(row["forensic_signatures"]["mixed_export_refreeze_days"])
                for row in selected
            ),
            "total_negative_daily_net_with_pack_loss_days": sum(
                int(
                    row["forensic_signatures"][
                        "negative_daily_net_with_pack_loss_days"
                    ]
                )
                for row in selected
            ),
            "total_negative_daily_net_with_pack_loss_m": sum(
                float(
                    row["forensic_signatures"][
                        "negative_daily_net_with_pack_loss_m"
                    ]
                )
                for row in selected
            ),
            "maximum_abs_wet_compaction_input_identity_residual_m": max(
                float(
                    row["forensic_signatures"][
                        "maximum_abs_wet_compaction_input_identity_residual_m"
                    ]
                )
                for row in selected
            ),
            "maximum_abs_independent_stage3_energy_residual_j_m2": max(
                float(row["maximum_abs_independent_stage3_energy_residual_j_m2"])
                for row in selected
            ),
            "maximum_abs_stage3_energy_trace_difference_j_m2": max(
                float(row["maximum_abs_stage3_energy_trace_difference_j_m2"])
                for row in selected
            ),
            "total_duplicate_state_loss_component_m": sum(
                float(
                    row["forensic_signatures"]["duplicate_state_loss_component_m"]
                )
                for row in selected
            ),
        }
        for component in COMPONENT_FIELDS:
            output[f"median_{component}_signed_sum_m"] = statistics.median(
                row["hourly_coe"]["component_signed_sums_m"][component]
                for row in selected
            )
        output["median_positive_hourly_raw_melt_m"] = statistics.median(
            row["hourly_coe"]["positive_applied_m"] for row in selected
        )
        output["median_negative_hourly_raw_melt_m"] = statistics.median(
            row["hourly_coe"]["negative_applied_m"] for row in selected
        )
        for field in ENERGY_FIELDS:
            output[f"median_{field}"] = statistics.median(
                row["energy_boundary"][field] for row in selected
            )
        return output

    primary = [row for row in annual if not row["right_censored"]]
    return {
        "annual": annual,
        "summary": {
            **summarize(primary),
            "right_censored_water_years_excluded": list(CENSORED_WATER_YEARS),
            "zero_observed_peak_years_skipped": skipped_zero,
        },
        "summary_all_windows_sensitivity": {
            **summarize(annual),
            "includes_right_censored_water_years": list(CENSORED_WATER_YEARS),
            "zero_observed_peak_years_skipped": skipped_zero,
        },
    }


def paired_deltas(
    reference: dict[str, Any], candidate: dict[str, Any], include_censored: bool
) -> dict[str, Any]:
    reference_by_year = {row["water_year"]: row for row in reference["annual"]}
    candidate_by_year = {row["water_year"]: row for row in candidate["annual"]}
    years = sorted(reference_by_year.keys() & candidate_by_year.keys())
    if not include_censored:
        years = [year for year in years if year not in CENSORED_WATER_YEARS]
    fields = (
        "modeled_peak_swe_m",
        "peak_swe_ratio",
        "peak_date_offset_days",
        "snowpack_swe_loss_m",
        "raw_melt_m",
        "routed_melt_m",
        "liquid_water_released_m",
        "rain_retained_m",
        "rain_released_m",
        "stage3_refrozen_liquid_m",
    )
    deltas = {
        field: [float(candidate_by_year[year][field]) - float(reference_by_year[year][field]) for year in years]
        for field in fields
    }
    return {
        "paired_year_count": len(years),
        "water_years": years,
        "includes_right_censored_water_years": include_censored,
        **{f"median_delta_{field}": statistics.median(values) for field, values in deltas.items()},
        **{f"maximum_abs_delta_{field}": max(map(abs, values)) for field, values in deltas.items()},
    }


def analysis_contract() -> dict[str, Any]:
    return {
        "zero_tolerance_m": ZERO,
        "mass_fields": list(MASS_FIELDS),
        "energy_fields": list(ENERGY_FIELDS),
        "coe_component_fields": list(COMPONENT_FIELDS),
        "observed_peak_rule": (
            "maximum positive observed_swe_mm within water year; earliest date wins ties"
        ),
        "window_rule": (
            "October 1 through observed peak date, both endpoints inclusive; "
            "skip non-positive observed peaks"
        ),
        "right_censor_rule": {
            "primary_exclusion": list(CENSORED_WATER_YEARS),
            "sensitivity": "retain the same rows in explicitly named all-window summaries",
        },
        "modeled_peak_rule": (
            "maximum runtime_swe_after_m inside the window; earliest date wins ties"
        ),
        "summary_rule": "Python statistics.median over annual values",
        "paired_delta_rule": (
            "candidate minus fresh same-binary baseline_replay by identical water year; "
            "primary excludes right-censored years"
        ),
        "primitive_mass_identity": (
            "runtime_swe_after_m - runtime_swe_before_m = accumulation_m + "
            "rain_retained_m - snowpack_swe_loss_m - sublimation_m"
        ),
        "hourly_coe_rule": {
            "component_sums": "signed arithmetic sum by named term over all hours",
            "positive_applied_m": "sum(max(coe_melt_applied_m, 0))",
            "negative_applied_m": "sum(min(coe_melt_applied_m, 0))",
            "uncapped_m": "signed sum(coe_melt_uncapped_m)",
            "cap_adjustment_m": "signed sum(coe_melt_cap_adjustment_m)",
        },
        "event_rule": {
            "primary_daily_loss_threshold_m": EVENT_THRESHOLD_M,
            "active_day": "snowpack_swe_loss_m >= threshold",
            "grouping": "consecutive active calendar days; any inactive day terminates event",
            "ranking": "descending aggregate event snowpack_swe_loss_m; chronological stable ties",
            "top_n_per_water_year": EVENT_TOP_N,
            "sensitivity_thresholds_m": list(EVENT_SENSITIVITY_THRESHOLDS_M),
        },
        "precipitation_day_rules": {
            "snowfall": "accumulation_m > zero_tolerance_m",
            "rain": "rain_retained_m + rain_released_m > zero_tolerance_m",
            "no_precipitation": "neither snowfall nor rain rule is true",
        },
        "forensic_rules": {
            "mixed_signed_hour_day": (
                "daily sum of positive hourly applied melt > tolerance and daily sum of "
                "negative hourly applied melt < -tolerance"
            ),
            "mixed_export_refreeze_day": (
                "mixed-signed-hour day with routed_melt_m > tolerance and "
                "stage3_refrozen_liquid_m > tolerance"
            ),
            "negative_daily_net_with_pack_loss": (
                "positive_applied_m + negative_applied_m <= tolerance and "
                "snowpack_swe_loss_m > tolerance"
            ),
            "wet_compaction_identity": (
                "density_process_liquid_for_compaction_mass_kg_m2 / 1000 = "
                "2 * snowpack_swe_loss_m + rain_released_m"
            ),
            "routed_alias_identity": (
                "routed_melt_m = snowpack_swe_loss_m + rain_released_m"
            ),
        },
        "publication_boundary": (
            "WAT file identity is retained but WAT values are not parsed; dynamic publication "
            "claims are limited to trace storage/routed aliases plus static consumer lineage"
        ),
        "energy_boundary": (
            "Stage-3 diagnostic energy operands only; empirical CoE melt depth is outside "
            "a complete surface-energy closure"
        ),
    }


def prepare_freeze() -> None:
    if FREEZE.exists():
        raise RuntimeError(f"refusing to overwrite {FREEZE}")
    lanes = w1.selected_lanes()
    fixture_inputs = {}
    baseline_traces = {}
    observations = {}
    for lane in lanes:
        fixture = PREDECESSOR / "fixtures" / lane.lane_id / "rst_1p0"
        trace = (
            PREDECESSOR
            / "runs"
            / lane.lane_id
            / "rst_1p0"
            / f"{lane.lane_id}-rst_1p0.snow.jsonl"
        )
        observation = Path(lane.observation_file)
        fixture_inputs[lane.lane_id] = file_manifest(fixture)
        baseline_traces[lane.lane_id] = {
            "path": relative(trace),
            "sha256": sha256(trace),
            "size_bytes": trace.stat().st_size,
        }
        observations[lane.lane_id] = {
            "path": relative(observation),
            "sha256": sha256(observation),
            "size_bytes": observation.stat().st_size,
            "role": "CALIBRATION_REUSED_AS_DIAGNOSTIC_ONLY",
        }
    protected_paths = (
        "crates/openwepp-hillslope-orchestrator",
        "crates/openwepp-meteorology",
        "crates/openwepp-runner",
        "docs/specifications/science-contracts",
        "tests",
        "references/50201000",
    )
    freeze = {
        "schema_version": 3,
        "status": "FROZEN_BEFORE_RESULT_BEARING_ANALYSIS",
        "frozen_at_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
        "characterization_only": True,
        "external_connectivity_allowed": False,
        "production_correction_authorized": False,
        "output_namespace": relative(OUTPUT),
        "source_identity": {
            "git_head": command_output(["git", "rev-parse", "HEAD"]).strip(),
            "protected_tree_hashes": {
                path: command_output(["git", "rev-parse", f"HEAD:{path}"]).strip()
                for path in protected_paths
            },
        },
        "tool_identity": {
            "path": relative(Path(__file__)),
            "sha256": sha256(Path(__file__)),
            "helper_dependencies": {
                name: {"path": relative(path), "sha256": sha256(path)}
                for name, path in sorted(HELPER_DEPENDENCIES.items())
            },
        },
        "package_identity": {
            "package_md_sha256": sha256(PACKAGE / "package.md"),
            "kickoff_prompt_sha256": sha256(
                PACKAGE
                / "prompts/active/20260803_execute_audit_kickoff_agent_prompt.md"
            ),
        },
        "binary_identity": {
            "path": relative(BINARY),
            "sha256": sha256(BINARY),
            "size_bytes": BINARY.stat().st_size,
            "mtime_utc": dt.datetime.fromtimestamp(
                BINARY.stat().st_mtime, tz=dt.timezone.utc
            ).isoformat(),
            "build_command": (
                "cargo build --release -p openwepp-runner --bin openwepp-cli-hill"
            ),
            "build_source_head": command_output(["git", "rev-parse", "HEAD"]).strip(),
        },
        "cohort": {
            "sites": [lane.lane_id for lane in lanes],
            "baseline_rst_c": 1.0,
            "baseline_configuration": BASELINE_ENV,
        },
        "baseline_traces": baseline_traces,
        "source_fixture_manifests": fixture_inputs,
        "observations": observations,
        "prospective_replay_operators": [
            {"id": name, "change_from_baseline": changes}
            for name, changes in EXECUTION_OPERATORS.items()
        ],
        "frozen_but_not_selected_operator": {
            "id": "legacy_density",
            "change_from_baseline": {
                "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL": "legacy_wepp"
            },
            "reason": (
                "physical density is outside the direct CoE/SWE boundary and legacy density "
                "does not compose with the selected Stage-3 boundary"
            ),
        },
        "analysis_contract": analysis_contract(),
        "legacy_authority": {
            "pinned_baseline": {
                path: git_blob_identity(PINNED_LEGACY_REPO, PINNED_LEGACY_COMMIT, path)
                for path in ("src/melt.for", "src/winter.for", "src/snowd.for")
            },
            "fixed_negative_melt_comparator": git_blob_identity(
                FIXED_COMPARATOR_REPO, FIXED_COMPARATOR_COMMIT, "src/winter.for"
            ),
        },
        "upstream_diagnostic_identities": {
            name: {"path": relative(path), "sha256": sha256(path)}
            for name, path in sorted(UPSTREAM_DIAGNOSTICS.items())
        },
        "prior_result_dispositions": [
            {
                "output": "target/snow_prepeak_liquid_evacuation_physics_audit",
                "status": "SUPERSEDED_NON_PROSPECTIVE_REVIEW_REJECTED",
                "reason": (
                    "event selection was not fully frozen and right-censored WY2025 was "
                    "included in primary summaries"
                ),
            },
            {
                "output": "target/snow_prepeak_liquid_evacuation_physics_audit_v2",
                "status": "REJECTED_BINARY_REFERENCE_CONFOUNDING",
                "reason": (
                    "fresh release-binary operator cells were compared against predecessor "
                    "reference traces produced by a different binary"
                ),
            },
        ],
    }
    write_json(FREEZE, freeze)


def verify_freeze(freeze: dict[str, Any]) -> None:
    if freeze["status"] != "FROZEN_BEFORE_RESULT_BEARING_ANALYSIS":
        raise RuntimeError("audit freeze is not active")
    if freeze["analysis_contract"] != analysis_contract():
        raise RuntimeError("analysis contract differs from freeze")
    if command_output(["git", "rev-parse", "HEAD"]).strip() != freeze["source_identity"]["git_head"]:
        raise RuntimeError("source HEAD differs from freeze")
    for path, expected in freeze["source_identity"]["protected_tree_hashes"].items():
        observed = command_output(["git", "rev-parse", f"HEAD:{path}"]).strip()
        if observed != expected:
            raise RuntimeError(f"protected source tree differs for {path}")
    if sha256(Path(__file__)) != freeze["tool_identity"]["sha256"]:
        raise RuntimeError("tool hash differs from freeze")
    if sha256(BINARY) != freeze["binary_identity"]["sha256"]:
        raise RuntimeError("binary hash differs from freeze")
    for name, path in HELPER_DEPENDENCIES.items():
        if sha256(path) != freeze["tool_identity"]["helper_dependencies"][name]["sha256"]:
            raise RuntimeError(f"helper hash differs for {name}")
    for name, path in UPSTREAM_DIAGNOSTICS.items():
        if sha256(path) != freeze["upstream_diagnostic_identities"][name]["sha256"]:
            raise RuntimeError(f"upstream diagnostic hash differs for {name}")
    for path, expected in freeze["legacy_authority"]["pinned_baseline"].items():
        if git_blob_identity(PINNED_LEGACY_REPO, PINNED_LEGACY_COMMIT, path) != expected:
            raise RuntimeError(f"pinned legacy identity differs for {path}")
    expected_fixed = freeze["legacy_authority"]["fixed_negative_melt_comparator"]
    if git_blob_identity(
        FIXED_COMPARATOR_REPO, FIXED_COMPARATOR_COMMIT, "src/winter.for"
    ) != expected_fixed:
        raise RuntimeError("fixed comparator identity differs")
    frozen_operators = {
        row["id"]: row["change_from_baseline"] for row in freeze["prospective_replay_operators"]
    }
    if any(frozen_operators.get(name) != changes for name, changes in EXECUTION_OPERATORS.items()):
        raise RuntimeError("selected operator differs from prospective freeze")
    for lane in w1.selected_lanes():
        fixture = PREDECESSOR / "fixtures" / lane.lane_id / "rst_1p0"
        trace = PREDECESSOR / "runs" / lane.lane_id / "rst_1p0" / f"{lane.lane_id}-rst_1p0.snow.jsonl"
        if sha256(trace) != freeze["baseline_traces"][lane.lane_id]["sha256"]:
            raise RuntimeError(f"baseline trace hash differs for {lane.lane_id}")
        if file_manifest(fixture) != freeze["source_fixture_manifests"][lane.lane_id]:
            raise RuntimeError(f"fixture manifest differs for {lane.lane_id}")
        observation = Path(lane.observation_file)
        if sha256(observation) != freeze["observations"][lane.lane_id]["sha256"]:
            raise RuntimeError(f"observation hash differs for {lane.lane_id}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--freeze", action="store_true")
    group.add_argument("--execute", action="store_true")
    args = parser.parse_args()
    if args.freeze:
        prepare_freeze()
        print(f"wrote {relative(FREEZE)}")
        return 0
    if OUTPUT.exists():
        raise RuntimeError(f"refusing to overwrite {OUTPUT}")
    freeze = json.loads(FREEZE.read_text(encoding="utf-8"))
    verify_freeze(freeze)
    lanes = w1.selected_lanes()
    OUTPUT.mkdir(parents=True)
    executed: dict[str, dict[str, Any]] = {name: {} for name in EXECUTION_OPERATORS}
    futures: dict[Any, tuple[str, str]] = {}
    with ThreadPoolExecutor(max_workers=4) as executor:
        for operator, changes in EXECUTION_OPERATORS.items():
            for lane in lanes:
                future = executor.submit(execute_cell, lane, operator, changes)
                futures[future] = (operator, lane.lane_id)
        for future in as_completed(futures):
            operator, lane_id = futures[future]
            executed[operator][lane_id] = future.result()

    analyses: dict[str, dict[str, Any]] = {"reference": {}}
    for lane in lanes:
        reference_fixture = FIXTURES / "baseline_replay" / lane.lane_id
        reference_trace = REPO / executed["baseline_replay"][lane.lane_id]["trace_path"]
        analyses["reference"][lane.lane_id] = analyze_trace(
            lane, reference_trace, reference_fixture
        )
    for operator in SELECTED_OPERATORS:
        analyses[operator] = {}
        for lane in lanes:
            trace = REPO / executed[operator][lane.lane_id]["trace_path"]
            fixture = FIXTURES / operator / lane.lane_id
            analyses[operator][lane.lane_id] = analyze_trace(lane, trace, fixture)

    primary_comparisons = {
        operator: {
            lane.lane_id: paired_deltas(
                analyses["reference"][lane.lane_id],
                analyses[operator][lane.lane_id],
                include_censored=False,
            )
            for lane in lanes
        }
        for operator in SELECTED_OPERATORS
    }
    all_window_comparisons = {
        operator: {
            lane.lane_id: paired_deltas(
                analyses["reference"][lane.lane_id],
                analyses[operator][lane.lane_id],
                include_censored=True,
            )
            for lane in lanes
        }
        for operator in SELECTED_OPERATORS
    }
    result = {
        "schema_version": 3,
        "evidence_mode": "Ran: frozen same-binary baseline plus existing-selector direct-production replays and primitive trace reconstruction",
        "characterization_only": True,
        "analysis_contract": freeze["analysis_contract"],
        "freeze_path": relative(FREEZE),
        "freeze_sha256": sha256(FREEZE),
        "reference_trace_hashes": {
            lane_id: executed["baseline_replay"][lane_id]["trace_sha256"]
            for lane_id in sorted(executed["baseline_replay"])
        },
        "analyses": analyses,
        "paired_operator_deltas": primary_comparisons,
        "paired_operator_deltas_all_windows_sensitivity": all_window_comparisons,
    }
    write_json(RESULTS, result)
    receipt = {
        "schema_version": 3,
        "status": "EXECUTED",
        "freeze_sha256": sha256(FREEZE),
        "tool_sha256": sha256(Path(__file__)),
        "binary_sha256": sha256(BINARY),
        "binary_size_bytes": BINARY.stat().st_size,
        "binary_mtime_utc": dt.datetime.fromtimestamp(
            BINARY.stat().st_mtime, tz=dt.timezone.utc
        ).isoformat(),
        "build_command": freeze["binary_identity"]["build_command"],
        "build_source_head": freeze["binary_identity"]["build_source_head"],
        "cell_count": len(EXECUTION_OPERATORS) * len(lanes),
        "selected_operators": list(EXECUTION_OPERATORS),
        "unused_frozen_operators": ["legacy_density"],
        "executed": executed,
        "dynamic_results_path": relative(RESULTS),
        "dynamic_results_sha256": sha256(RESULTS),
        "analysis_contract_sha256": hashlib.sha256(
            json.dumps(
                freeze["analysis_contract"], sort_keys=True, separators=(",", ":")
            ).encode()
        ).hexdigest(),
    }
    write_json(RECEIPT, receipt)
    print(f"wrote {relative(RESULTS)}")
    print(f"wrote {relative(RECEIPT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
