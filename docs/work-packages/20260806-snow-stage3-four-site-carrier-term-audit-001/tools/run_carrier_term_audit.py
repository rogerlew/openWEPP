#!/usr/bin/env python3
"""Execute and analyze the frozen four-site Stage 3 carrier-term audit."""

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
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any, Iterable

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
FREEZE_PATH = PACKAGE / "artifacts/protocol-freeze.json"
OUTPUT = REPO / "target/snow_stage3_four_site_carrier_term_audit"
SOURCE_FIXTURES = REPO / (
    "target/snow_prepeak_liquid_evacuation_physics_audit_v3/"
    "fixtures/baseline_replay"
)
OBSERVATIONS = REPO / "tests/fixtures/snotel_observed/observations/sites"
SNOWBIRD_DERIVATIVE = REPO / (
    "tests/fixtures/snotel_observed/snotel_snowbird_ut/development/"
    "precip_x1p2155576/p8.cli"
)
BINARY = REPO / "target/release/openwepp-cli-hill"
W1_TOOL = REPO / (
    "docs/work-packages/20260802-snow-surface-eb-04w1-"
    "precipitation-scaling-calibration-001/tools/run_precipitation_scaling.py"
)
TERMS = ("shortwave", "longwave", "sensible", "latent", "advected")
DAILY_FIELDS = {
    "shortwave": "stage3_evaluation_complete_arm_shortwave_j_m2",
    "longwave": "stage3_evaluation_complete_arm_longwave_j_m2",
    "sensible": "stage3_evaluation_complete_arm_sensible_j_m2",
    "latent": "stage3_evaluation_complete_arm_latent_j_m2",
    "advected": "stage3_evaluation_complete_arm_advected_j_m2",
}
HOURLY_FIELDS = {
    "shortwave": "stage3_evaluation_hourly_shortwave_j_m2",
    "longwave": "stage3_evaluation_hourly_longwave_j_m2",
    "sensible": "stage3_evaluation_hourly_sensible_j_m2",
    "latent": "stage3_evaluation_hourly_latent_j_m2",
    "advected": "stage3_evaluation_hourly_advected_j_m2",
}
ZERO = 1.0e-12


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def command_output(argv: list[str]) -> str:
    return subprocess.run(
        argv, check=True, text=True, capture_output=True, cwd=REPO
    ).stdout.strip()


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(value, allow_nan=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        raise RuntimeError(f"refusing to write empty table: {path}")
    path.parent.mkdir(parents=True, exist_ok=True)
    fieldnames = list(rows[0])
    if any(list(row) != fieldnames for row in rows):
        raise RuntimeError(f"inconsistent CSV schema: {path}")
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


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


def climate_file(root: Path) -> Path:
    files = sorted(root.glob("*.cli"))
    if len(files) != 1:
        raise RuntimeError(f"expected exactly one climate file under {root}")
    return files[0]


def climate_dates(path: Path) -> list[dt.date]:
    dates = []
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


def observed_peaks(path: Path) -> tuple[dict[int, tuple[dt.date, float]], list[int]]:
    peaks: dict[int, tuple[dt.date, float]] = {}
    with path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            raw = row.get("observed_swe_mm")
            if raw in (None, ""):
                continue
            year = int(row["water_year"])
            candidate = (dt.date.fromisoformat(row["date"]), float(raw) / 1000.0)
            current = peaks.get(year)
            if current is None or candidate[1] > current[1]:
                peaks[year] = candidate
    skipped = sorted(year for year, (_, value) in peaks.items() if value <= 0.0)
    return ({year: peak for year, peak in peaks.items() if peak[1] > 0.0}, skipped)


def quantiles(values: list[float]) -> tuple[float, float]:
    if len(values) == 1:
        return values[0], values[0]
    quartiles = statistics.quantiles(values, n=4, method="inclusive")
    return quartiles[0], quartiles[2]


def distribution(values: list[float]) -> dict[str, float | int]:
    if not values:
        raise RuntimeError("empty distribution")
    q1, q3 = quantiles(values)
    return {
        "count": len(values),
        "minimum": min(values),
        "q1": q1,
        "median": statistics.median(values),
        "q3": q3,
        "maximum": max(values),
        "positive_fraction": sum(value > ZERO for value in values) / len(values),
        "negative_fraction": sum(value < -ZERO for value in values) / len(values),
    }


def checked_float(row: dict[str, Any], field: str) -> float:
    value = float(row[field])
    if not math.isfinite(value):
        raise RuntimeError(f"non-finite {field}")
    return value


def require_close(label: str, actual: float, expected: float, tolerance: float) -> None:
    if not math.isfinite(actual) or not math.isfinite(expected):
        raise RuntimeError(f"non-finite identity operand: {label}")
    if abs(actual - expected) > tolerance:
        raise RuntimeError(
            f"{label} residual {actual - expected} exceeds {tolerance}"
        )


def validate_tags(row: dict[str, Any], frozen: dict[str, Any]) -> None:
    fields = {
        "operator_id": "stage3_evaluation_operator_id",
        "source_snapshot_id": "stage3_evaluation_source_snapshot_id",
        "support_id": "stage3_evaluation_support_id",
        "cadence_id": "stage3_evaluation_cadence_id",
        "carrier_id": "stage3_evaluation_carrier_id",
        "coverage_id": "stage3_evaluation_coverage_id",
        "claim_class": "stage3_evaluation_claim_class",
        "unresolved_boundaries_id": "stage3_evaluation_unresolved_boundaries_id",
        "pairing_id": "stage3_evaluation_pairing_id",
    }
    for freeze_key, trace_key in fields.items():
        if row.get(trace_key) != frozen[freeze_key]:
            raise RuntimeError(f"tag mismatch: {trace_key}")
    if row.get("stage3_evaluation_arm_ids") != frozen["arm_ids"]:
        raise RuntimeError("arm ID mismatch")
    if int(row.get("stage3_evaluation_arm_count", -1)) != 2:
        raise RuntimeError("arm count mismatch")
    surface = row["stage3_evaluation_surface_arm_non_formulation_fingerprint_fnv1a64"]
    complete = row["stage3_evaluation_complete_arm_non_formulation_fingerprint_fnv1a64"]
    if surface != complete or surface == "0000000000000000":
        raise RuntimeError("paired non-formulation fingerprint mismatch")


def parse_trace(
    trace: Path, dates: list[dt.date], frozen: dict[str, Any]
) -> dict[dt.date, dict[str, Any]]:
    rows: dict[dt.date, dict[str, Any]] = {}
    with trace.open(encoding="utf-8") as handle:
        for index, line in enumerate(handle):
            if index >= len(dates):
                raise RuntimeError("trace has more rows than climate")
            row = json.loads(line)
            stamp = dates[index]
            schema = row.get("schema")
            if schema == "openwepp-r7h-direct-production-snow-trace-v5":
                validate_tags(row, frozen["expected_tags"])
                validate_evaluation_row(row, stamp)
            elif schema != "openwepp-r7h-direct-production-snow-trace-v4":
                raise RuntimeError(f"unexpected trace schema on {stamp}: {schema}")
            rows[stamp] = row
    if len(rows) != len(dates):
        raise RuntimeError("trace/climate row-count mismatch")
    return rows


def validate_evaluation_row(row: dict[str, Any], stamp: dt.date) -> None:
    tolerance = 1.0e-6
    arrays = {term: row[field] for term, field in HOURLY_FIELDS.items()}
    if any(len(values) != 24 for values in arrays.values()):
        raise RuntimeError(f"invalid term array length on {stamp}")
    requested = row["stage3_evaluation_hourly_requested_seconds"]
    evaluated = row["stage3_evaluation_hourly_evaluated_seconds"]
    active = row["stage3_evaluation_hourly_complete_carrier_evaluated"]
    internal = row[
        "stage3_evaluation_hourly_internal_active_lower_conduction_j_m2"
    ]
    if any(len(values) != 24 for values in (requested, evaluated, active, internal)):
        raise RuntimeError(f"invalid support array length on {stamp}")
    for hour in range(24):
        require_close(f"requested support {stamp} h{hour}", float(requested[hour]), 3600.0, 0.0)
        expected_evaluated = 3600.0 if bool(active[hour]) else 0.0
        require_close(
            f"evaluated support {stamp} h{hour}",
            float(evaluated[hour]),
            expected_evaluated,
            0.0,
        )
        require_close(f"internal conduction {stamp} h{hour}", float(internal[hour]), 0.0, 0.0)
        for term in TERMS:
            value = float(arrays[term][hour])
            if not math.isfinite(value):
                raise RuntimeError(f"non-finite {term} on {stamp} h{hour}")
    if row["stage3_evaluation_complete_arm_internal_conduction_applicable"]:
        raise RuntimeError(f"internal conduction unexpectedly applicable on {stamp}")
    require_close(
        f"daily internal conduction {stamp}",
        checked_float(
            row,
            "stage3_evaluation_complete_arm_internal_active_lower_conduction_j_m2",
        ),
        0.0,
        0.0,
    )
    daily = {term: checked_float(row, field) for term, field in DAILY_FIELDS.items()}
    for term in TERMS:
        require_close(
            f"daily/hourly {term} {stamp}",
            daily[term],
            sum(float(value) for value in arrays[term]),
            tolerance,
        )
    complete = sum(daily.values())
    surface = daily["shortwave"] + daily["longwave"] + daily["latent"]
    require_close(
        f"complete reconstruction {stamp}",
        checked_float(row, "stage3_evaluation_complete_arm_total_j_m2"),
        complete,
        tolerance,
    )
    require_close(
        f"surface reconstruction {stamp}",
        checked_float(row, "stage3_evaluation_surface_arm_total_j_m2"),
        surface,
        tolerance,
    )
    require_close(
        f"complete producer residual {stamp}",
        checked_float(row, "stage3_evaluation_complete_arm_component_residual_j_m2"),
        0.0,
        tolerance,
    )


def annual_window(
    site: str,
    water_year: int,
    peak: tuple[dt.date, float],
    rows: dict[dt.date, dict[str, Any]],
    censored: set[int],
) -> dict[str, Any]:
    start = dt.date(water_year - 1, 10, 1)
    end, observed_peak_swe_m = peak
    stamps = [start + dt.timedelta(days=index) for index in range((end - start).days + 1)]
    if not stamps or any(stamp not in rows for stamp in stamps):
        raise RuntimeError(f"window outside trace chronology: {site} WY{water_year}")
    totals = {term: 0.0 for term in TERMS}
    hourly_fluxes = {term: [] for term in TERMS}
    evaluated_seconds = 0.0
    v5_days = 0
    zero_coverage_v5_days = 0
    for stamp in stamps:
        row = rows[stamp]
        if row["schema"] != "openwepp-r7h-direct-production-snow-trace-v5":
            continue
        v5_days += 1
        active = row["stage3_evaluation_hourly_complete_carrier_evaluated"]
        evaluated = row["stage3_evaluation_hourly_evaluated_seconds"]
        if not any(bool(value) for value in active):
            zero_coverage_v5_days += 1
        for hour in range(24):
            if not bool(active[hour]):
                continue
            seconds = float(evaluated[hour])
            evaluated_seconds += seconds
            for term in TERMS:
                energy = float(row[HOURLY_FIELDS[term]][hour])
                totals[term] += energy
                hourly_fluxes[term].append(energy / seconds)
    calendar_seconds = len(stamps) * 86_400.0
    record: dict[str, Any] = {
        "site": site,
        "water_year": water_year,
        "right_censored": water_year in censored,
        "window_start": start.isoformat(),
        "window_end": end.isoformat(),
        "observed_peak_swe_m": observed_peak_swe_m,
        "calendar_days": len(stamps),
        "v5_days": v5_days,
        "zero_coverage_v5_days": zero_coverage_v5_days,
        "evaluated_hours": int(round(evaluated_seconds / 3600.0)),
        "calendar_hours": len(stamps) * 24,
        "coverage_fraction": evaluated_seconds / calendar_seconds,
        "eligible": evaluated_seconds > 0.0 and water_year not in censored,
    }
    if evaluated_seconds <= 0.0:
        for term in TERMS + ("net_radiation", "turbulent", "complete", "surface", "complete_minus_surface"):
            record[f"{term}_energy_mj_m2"] = None
            record[f"{term}_resolved_mean_w_m2"] = None
            record[f"{term}_calendar_mean_w_m2"] = None
        record["hourly_flux_distributions_w_m2"] = None
        return record
    derived = {
        **totals,
        "net_radiation": totals["shortwave"] + totals["longwave"],
        "turbulent": totals["sensible"] + totals["latent"],
        "complete": sum(totals.values()),
        "surface": totals["shortwave"] + totals["longwave"] + totals["latent"],
        "complete_minus_surface": totals["sensible"] + totals["advected"],
    }
    abs_operands = sum(abs(value) for value in totals.values())
    annual_tolerance = max(1.0e-6, 1.0e-12 * abs_operands)
    require_close(
        f"annual complete delta {site} WY{water_year}",
        derived["complete"] - derived["surface"],
        derived["complete_minus_surface"],
        annual_tolerance,
    )
    for term, energy in derived.items():
        record[f"{term}_energy_mj_m2"] = energy / 1.0e6
        record[f"{term}_resolved_mean_w_m2"] = energy / evaluated_seconds
        record[f"{term}_calendar_mean_w_m2"] = energy / calendar_seconds
    hourly_fluxes["net_radiation"] = [
        a + b for a, b in zip(hourly_fluxes["shortwave"], hourly_fluxes["longwave"])
    ]
    hourly_fluxes["turbulent"] = [
        a + b for a, b in zip(hourly_fluxes["sensible"], hourly_fluxes["latent"])
    ]
    hourly_fluxes["complete"] = [
        sum(values) for values in zip(*(hourly_fluxes[term] for term in TERMS))
    ]
    record["hourly_flux_distributions_w_m2"] = {
        term: distribution(values) for term, values in hourly_fluxes.items()
    }
    return record


def classify(value: float, lower: float, upper: float) -> str:
    return "WITHIN_CONTEXT" if lower <= value <= upper else "OUTSIDE_CONTEXT"


def summarize_sites(annual: list[dict[str, Any]], frozen: dict[str, Any]) -> list[dict[str, Any]]:
    summaries = []
    broader = frozen["carrier_screen"]["broader_context_w_m2"]
    near = frozen["carrier_screen"]["near_balance_w_m2"]
    marks = frozen["literature_context"]["marks_1998_figure_7_forest_w_m2"]
    for site in [row["site"] for row in frozen["cohort"]]:
        eligible = [row for row in annual if row["site"] == site and row["eligible"]]
        if not eligible:
            summaries.append({"site": site, "eligible_window_count": 0, "status": "NOT_COMPARABLE"})
            continue
        summary: dict[str, Any] = {
            "site": site,
            "eligible_window_count": len(eligible),
            "water_year_min": min(row["water_year"] for row in eligible),
            "water_year_max": max(row["water_year"] for row in eligible),
            "coverage_fraction": distribution([row["coverage_fraction"] for row in eligible]),
        }
        for term in TERMS + ("net_radiation", "turbulent", "complete", "surface", "complete_minus_surface"):
            values = [row[f"{term}_resolved_mean_w_m2"] for row in eligible]
            summary[f"{term}_resolved_mean_w_m2"] = distribution(values)
            energies = [row[f"{term}_energy_mj_m2"] for row in eligible]
            summary[f"{term}_energy_mj_m2"] = distribution(energies)
        complete = summary["complete_resolved_mean_w_m2"]["median"]
        summary["near_balance_class"] = classify(complete, near[0], near[1])
        summary["broader_total_context_class"] = classify(
            complete, broader[0], broader[1]
        )
        for term, key in (
            ("net_radiation", "net_all_wave"),
            ("turbulent", "combined_sensible_latent"),
            ("advected", "precipitation_advection"),
        ):
            bounds = marks[key]
            summary[f"marks_{term}_context_class"] = classify(
                summary[f"{term}_resolved_mean_w_m2"]["median"], bounds[0], bounds[1]
            )
        radiative_positive = max(
            summary["shortwave_energy_mj_m2"]["median"], 0.0
        ) + max(summary["longwave_energy_mj_m2"]["median"], 0.0)
        summary["positive_longwave_radiative_fraction"] = (
            max(summary["longwave_energy_mj_m2"]["median"], 0.0) / radiative_positive
            if radiative_positive > 0.0
            else None
        )
        summaries.append(summary)
    return summaries


def apply_screen(site_summary: list[dict[str, Any]], frozen: dict[str, Any]) -> dict[str, Any]:
    comparable = [row for row in site_summary if row.get("status") != "NOT_COMPARABLE"]
    near_count = sum(row["near_balance_class"] == "WITHIN_CONTEXT" for row in comparable)
    outside_broad = sum(
        row["broader_total_context_class"] == "OUTSIDE_CONTEXT" for row in comparable
    )
    passed = (
        len(comparable) == len(frozen["cohort"])
        and near_count >= frozen["carrier_screen"]["minimum_sites_within_near_balance"]
        and outside_broad <= frozen["carrier_screen"]["max_sites_outside_broader_context"]
    )
    return {
        "status": "PASS" if passed else "FAIL",
        "comparable_site_count": len(comparable),
        "near_balance_site_count": near_count,
        "outside_broader_context_site_count": outside_broad,
        "persistent_shadow_advancement": "PERMITTED_FOR_CONSIDERATION" if passed else "BLOCKED",
    }


def sanitized_environment(
    trace: Path | None, selectors: dict[str, str]
) -> tuple[dict[str, str], list[str], dict[str, str]]:
    removed = sorted(key for key in os.environ if key.startswith("OPENWEPP_"))
    environment = {
        key: value for key, value in os.environ.items() if not key.startswith("OPENWEPP_")
    }
    effective = dict(selectors)
    if trace is not None:
        effective["OPENWEPP_R7H_SNOW_TRACE_PATH"] = str(trace.resolve())
    environment.update(effective)
    observed = {key: value for key, value in environment.items() if key.startswith("OPENWEPP_")}
    if observed != effective:
        raise RuntimeError("OPENWEPP environment sanitizer failed")
    return environment, removed, effective


def prepare_fixture(site: str, frozen_site: dict[str, Any]) -> tuple[Path, dict[str, Any]]:
    source = SOURCE_FIXTURES / site
    source_manifest = file_manifest(source)
    if source_manifest["manifest_sha256"] != frozen_site["fixture_manifest_sha256"]:
        raise RuntimeError(f"source fixture hash differs for {site}")
    fixture = OUTPUT / "fixtures" / site
    shutil.copytree(source, fixture)
    snowbird = site == "snotel_snowbird_ut"
    if snowbird:
        staged = fixture / frozen_site["climate_file"]
        if sha256(staged) != frozen_site["canonical_climate_sha256"]:
            raise RuntimeError("staged canonical Snowbird climate hash differs")
        if sha256(SNOWBIRD_DERIVATIVE) != frozen_site["development_climate_sha256"]:
            raise RuntimeError("Snowbird derivative hash differs")
        shutil.copyfile(SNOWBIRD_DERIVATIVE, staged)
        if sha256(staged) != frozen_site["development_climate_sha256"]:
            raise RuntimeError("Snowbird derivative was not staged exactly")
    copied_manifest = file_manifest(fixture)
    if not snowbird and copied_manifest != source_manifest:
        raise RuntimeError(f"copied fixture differs for {site}")
    return fixture, {
        "source_manifest": source_manifest,
        "copied_manifest": copied_manifest,
        "snowbird_development_derivative_consumed": snowbird,
        "staged_climate_path": relative(climate_file(fixture)),
        "staged_climate_sha256": sha256(climate_file(fixture)),
    }


def execute_lane(
    site: str,
    fixture: Path,
    lane: str,
    selectors: dict[str, str],
    w1: Any,
) -> dict[str, Any]:
    run_dir = OUTPUT / "runs" / site / lane
    run_dir.mkdir(parents=True)
    stem = f"{site}-carrier-audit"
    runfile = run_dir / f"{stem}.run"
    source_stem = w1.eb04r.legacy.observed_harness.discover_run_stem(fixture)
    w1.eb04r.legacy.observed_harness.write_runfile(
        runfile, fixture, source_stem, run_dir, stem
    )
    command = w1.eb04r.legacy.observed_harness.cli_command(
        BINARY, fixture, runfile, run_dir, "direct-production-executor"
    )
    trace = run_dir / f"{stem}.snow.jsonl" if lane == "paired" else None
    effective_selectors = dict(selectors)
    effective_selectors["OPENWEPP_SNOW_STAGE3_EVALUATION_OPERATOR"] = (
        "same_state_paired_carrier_v1" if lane == "paired" else "disabled"
    )
    environment, removed, effective = sanitized_environment(trace, effective_selectors)
    completed = subprocess.run(
        command,
        cwd=REPO,
        env=environment,
        text=True,
        capture_output=True,
        check=False,
    )
    (run_dir / "stdout.txt").write_text(completed.stdout, encoding="utf-8")
    (run_dir / "stderr.txt").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise RuntimeError(f"run failed for {site}/{lane}: {completed.stderr[-2000:]}")
    outputs = {}
    for path in sorted(candidate for candidate in run_dir.iterdir() if candidate.is_file()):
        outputs[path.name] = {
            "path": relative(path),
            "sha256": sha256(path),
            "size_bytes": path.stat().st_size,
        }
    return {
        "site": site,
        "lane": lane,
        "argv": [str(value) for value in command],
        "returncode": completed.returncode,
        "removed_openwepp_key_names": removed,
        "effective_openwepp_environment": effective,
        "outputs": outputs,
        "runfile_sha256": sha256(runfile),
    }


def output_path(receipt: dict[str, Any], suffix: str) -> Path:
    matches = [value for name, value in receipt["outputs"].items() if name.endswith(suffix)]
    if len(matches) != 1:
        raise RuntimeError(f"expected one {suffix} output for {receipt['site']}/{receipt['lane']}")
    return REPO / matches[0]["path"]


def execute() -> None:
    if OUTPUT.exists():
        raise RuntimeError(f"refusing to overwrite {OUTPUT}")
    frozen = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    if frozen["status"] != "frozen_before_result_execution":
        raise RuntimeError("protocol freeze is not active")
    if frozen["pre_result_commit"] == "PENDING":
        raise RuntimeError("pre-result commit is not frozen")
    if command_output(["git", "status", "--porcelain"]):
        raise RuntimeError("result execution requires a clean worktree")
    head = command_output(["git", "rev-parse", "HEAD"])
    if head != frozen["pre_result_commit"]:
        raise RuntimeError("HEAD differs from frozen pre-result commit")
    if not BINARY.is_file():
        raise RuntimeError(f"release binary is missing: {BINARY}")
    if not SOURCE_FIXTURES.is_dir():
        raise RuntimeError(f"source fixtures are missing: {SOURCE_FIXTURES}")
    w1 = load_module("carrier_audit_w1", W1_TOOL)
    OUTPUT.mkdir(parents=True)
    fixture_receipts = {}
    fixtures = {}
    for frozen_site in frozen["cohort"]:
        site = frozen_site["site"]
        fixtures[site], fixture_receipts[site] = prepare_fixture(site, frozen_site)
        observation = OBSERVATIONS / f"{site}.csv"
        if sha256(observation) != frozen_site["observation_sha256"]:
            raise RuntimeError(f"observation hash differs for {site}")
        fixture_receipts[site]["observation"] = {
            "path": relative(observation),
            "sha256": sha256(observation),
            "role": "DIAGNOSTIC_ONLY",
        }
    selectors = dict(frozen["selectors"])
    selectors.pop("OPENWEPP_SNOW_STAGE3_EVALUATION_OPERATOR")
    receipts: dict[str, dict[str, Any]] = {site: {} for site in fixtures}
    jobs = [(site, lane) for site in fixtures for lane in ("control", "paired")]
    with ThreadPoolExecutor(max_workers=2) as executor:
        futures = {
            executor.submit(
                execute_lane, site, fixtures[site], lane, selectors, w1
            ): (site, lane)
            for site, lane in jobs
        }
        for future in as_completed(futures):
            site, lane = futures[future]
            receipts[site][lane] = future.result()
    protected_identity = {}
    for site, lanes in receipts.items():
        control, paired = lanes["control"], lanes["paired"]
        wat_control, wat_paired = output_path(control, ".wat.parquet"), output_path(paired, ".wat.parquet")
        hbp_control, hbp_paired = output_path(control, ".hbp"), output_path(paired, ".hbp")
        protected_identity[site] = {
            "wat_exact": sha256(wat_control) == sha256(wat_paired),
            "wat_control_sha256": sha256(wat_control),
            "wat_paired_sha256": sha256(wat_paired),
            "hbp_exact": sha256(hbp_control) == sha256(hbp_paired),
            "hbp_control_sha256": sha256(hbp_control),
            "hbp_paired_sha256": sha256(hbp_paired),
            "pass_output": "not emitted by direct hillslope surface",
        }
        if not protected_identity[site]["wat_exact"] or not protected_identity[site]["hbp_exact"]:
            raise RuntimeError(f"protected output differs for {site}")
    analyze_and_write(frozen, receipts, fixture_receipts, protected_identity, head)


def analyze_and_write(
    frozen: dict[str, Any],
    receipts: dict[str, dict[str, Any]],
    fixture_receipts: dict[str, Any],
    protected_identity: dict[str, Any],
    head: str,
) -> None:
    annual = []
    skipped_zero_observed = {}
    for frozen_site in frozen["cohort"]:
        site = frozen_site["site"]
        fixture = OUTPUT / "fixtures" / site
        trace = output_path(receipts[site]["paired"], ".snow.jsonl")
        dates = climate_dates(climate_file(fixture))
        rows = parse_trace(trace, dates, frozen)
        observation = OBSERVATIONS / f"{site}.csv"
        peaks, skipped = observed_peaks(observation)
        skipped_zero_observed[site] = skipped
        for water_year, peak in sorted(peaks.items()):
            start = dt.date(water_year - 1, 10, 1)
            if start < dates[0] or peak[0] > dates[-1]:
                continue
            annual.append(
                annual_window(
                    site,
                    water_year,
                    peak,
                    rows,
                    set(frozen["censored_primary_water_years"]),
                )
            )
    site_summary = summarize_sites(annual, frozen)
    screen = apply_screen(site_summary, frozen)
    results = {
        "schema_version": 1,
        "evidence_mode": "Ran: exact-current release CLI four-site same-state paired carrier audit",
        "characterization_only": true_value(),
        "freeze_sha256": sha256(FREEZE_PATH),
        "pre_result_commit": head,
        "snow_ground_boundary": "NOT_IMPLEMENTED",
        "internal_active_lower_conduction": "NOT_APPLICABLE_IN_SAME_STATE_PAIR_AND_EXACT_ZERO",
        "primary_window_count": sum(row["eligible"] for row in annual),
        "right_censored_window_count": sum(row["right_censored"] for row in annual),
        "annual": annual,
        "site_summary": site_summary,
        "carrier_screen": screen,
        "zero_observed_peak_years_skipped": skipped_zero_observed,
        "claim_limits": frozen["claim_limits"],
    }
    write_json(OUTPUT / "results/carrier-term-results.json", results)
    write_csv(
        OUTPUT / "tables/annual-metrics.csv",
        [{key: value for key, value in row.items() if key != "hourly_flux_distributions_w_m2"} for row in annual],
    )
    flat_site = []
    for row in site_summary:
        flat_site.append(
            {
                "site": row["site"],
                "eligible_window_count": row["eligible_window_count"],
                "median_coverage_fraction": row.get("coverage_fraction", {}).get("median"),
                **{
                    f"median_{term}_resolved_mean_w_m2": row.get(f"{term}_resolved_mean_w_m2", {}).get("median")
                    for term in TERMS + ("net_radiation", "turbulent", "complete", "surface", "complete_minus_surface")
                },
                "near_balance_class": row.get("near_balance_class", "NOT_COMPARABLE"),
                "broader_total_context_class": row.get("broader_total_context_class", "NOT_COMPARABLE"),
            }
        )
    write_csv(OUTPUT / "tables/site-summary.csv", flat_site)
    receipt = {
        "schema_version": 1,
        "status": "EXECUTED",
        "git_head": head,
        "git_status_porcelain": command_output(["git", "status", "--porcelain"]),
        "binary": {
            "path": relative(BINARY),
            "sha256": sha256(BINARY),
            "size_bytes": BINARY.stat().st_size,
        },
        "freeze": {"path": relative(FREEZE_PATH), "sha256": sha256(FREEZE_PATH)},
        "fixtures": fixture_receipts,
        "runs": receipts,
        "protected_output_identity": protected_identity,
    }
    write_json(OUTPUT / "execution-receipt.json", receipt)
    verify_outputs(frozen)


def true_value() -> bool:
    return True


def verify_outputs(frozen: dict[str, Any]) -> None:
    result_path = OUTPUT / "results/carrier-term-results.json"
    receipt_path = OUTPUT / "execution-receipt.json"
    if not result_path.is_file() or not receipt_path.is_file():
        raise RuntimeError("result or receipt is missing")
    results = json.loads(result_path.read_text(encoding="utf-8"))
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    if receipt["git_head"] != frozen["pre_result_commit"]:
        raise RuntimeError("receipt head differs from freeze")
    if results["freeze_sha256"] != sha256(FREEZE_PATH):
        raise RuntimeError("result freeze hash differs")
    if len(results["site_summary"]) != len(frozen["cohort"]):
        raise RuntimeError("result site cohort differs")
    if any(
        not values["wat_exact"] or not values["hbp_exact"]
        for values in receipt["protected_output_identity"].values()
    ):
        raise RuntimeError("protected output identity failed")
    for site in frozen["cohort"]:
        if site["site"] not in receipt["runs"]:
            raise RuntimeError(f"missing run receipt: {site['site']}")
    if results["snow_ground_boundary"] != "NOT_IMPLEMENTED":
        raise RuntimeError("snow-ground boundary was not preserved")


def verify_existing() -> None:
    if not OUTPUT.is_dir():
        raise RuntimeError(f"missing output namespace: {OUTPUT}")
    frozen = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    receipt = json.loads((OUTPUT / "execution-receipt.json").read_text(encoding="utf-8"))
    for site_receipts in receipt["runs"].values():
        for lane_receipt in site_receipts.values():
            for value in lane_receipt["outputs"].values():
                path = REPO / value["path"]
                if path.stat().st_size != value["size_bytes"] or sha256(path) != value["sha256"]:
                    raise RuntimeError(f"retained output identity differs: {path}")
    verify_outputs(frozen)
    print("carrier-term audit verification: PASS")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-existing", action="store_true")
    args = parser.parse_args()
    if args.verify_existing:
        verify_existing()
    else:
        execute()


if __name__ == "__main__":
    main()
