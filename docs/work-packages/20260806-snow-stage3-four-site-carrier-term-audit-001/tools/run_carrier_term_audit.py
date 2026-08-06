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
import re
import shutil
import statistics
import subprocess
import sys
import tomllib
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
HOURLY_ZERO_FIELDS = (
    "stage3_evaluation_hourly_internal_active_lower_conduction_j_m2",
    "stage3_evaluation_hourly_cold_content_export_j_m2",
    "stage3_evaluation_hourly_cold_required_j_m2",
    "stage3_evaluation_hourly_cold_energy_change_j_m2",
    "stage3_evaluation_hourly_excess_energy_j_m2",
    "stage3_evaluation_hourly_available_ice_kg_m2",
    "stage3_evaluation_hourly_sublimation_kg_m2",
    "stage3_evaluation_hourly_melt_kg_m2",
    "stage3_evaluation_hourly_terminal_unallocated_j_m2",
    "stage3_evaluation_hourly_energy_closure_residual_j_m2",
)
DAILY_ZERO_FIELDS = (
    "stage3_evaluation_complete_arm_internal_active_lower_conduction_j_m2",
    "stage3_evaluation_complete_arm_cold_content_export_j_m2",
    "stage3_evaluation_complete_arm_available_ice_kg_m2",
    "stage3_evaluation_complete_arm_cold_energy_change_j_m2",
    "stage3_evaluation_complete_arm_excess_energy_j_m2",
    "stage3_evaluation_complete_arm_sublimation_kg_m2",
    "stage3_evaluation_complete_arm_melt_kg_m2",
    "stage3_evaluation_complete_arm_terminal_unallocated_j_m2",
    "stage3_evaluation_complete_arm_component_residual_j_m2",
    "stage3_evaluation_complete_arm_maximum_thermodynamic_residual_j_m2",
)
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


def assert_execution_source(expected_head: str) -> str:
    if not re.fullmatch(r"[0-9a-f]{40}", expected_head):
        raise RuntimeError("expected execution HEAD must be a full lowercase Git SHA")
    head = command_output(["git", "rev-parse", "HEAD"])
    if head != expected_head:
        raise RuntimeError(f"execution HEAD {head} differs from admitted {expected_head}")
    if command_output(["git", "status", "--porcelain"]):
        raise RuntimeError("result execution requires an empty tracked worktree")
    return head


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


def retained_manifest(root: Path) -> dict[str, Any]:
    manifest_name = "retained-artifact-manifest.json"
    files = []
    for path in sorted(candidate for candidate in root.rglob("*") if candidate.is_file()):
        if path.relative_to(root).as_posix() == manifest_name:
            continue
        files.append(
            {
                "path": path.relative_to(root).as_posix(),
                "sha256": sha256(path),
                "size_bytes": path.stat().st_size,
            }
        )
    return {"schema_version": 1, "files": files, "file_count": len(files)}


def climate_file(root: Path) -> Path:
    files = sorted(root.glob("*.cli"))
    if len(files) != 1:
        raise RuntimeError(f"expected exactly one climate file under {root}")
    return files[0]


def retained_observation(site: str) -> Path:
    return OUTPUT / "inputs/observations" / f"{site}.csv"


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


def water_year(stamp: dt.date) -> int:
    return stamp.year + 1 if stamp.month >= 10 else stamp.year


def observed_peaks(path: Path) -> tuple[dict[int, tuple[dt.date, float]], list[dict[str, Any]]]:
    by_year: dict[int, list[tuple[dt.date, float | None]]] = {}
    seen_dates: set[dt.date] = set()
    previous: dt.date | None = None
    with path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            stamp = dt.date.fromisoformat(row["date"])
            year = int(row["water_year"])
            if stamp in seen_dates or previous is not None and stamp <= previous:
                raise RuntimeError(f"invalid observation chronology: {path} {stamp}")
            if water_year(stamp) != year:
                raise RuntimeError(f"observation water-year mismatch: {path} {stamp}")
            seen_dates.add(stamp)
            previous = stamp
            raw = row.get("observed_swe_mm")
            value = None if raw in (None, "") else float(raw) / 1000.0
            if value is not None and not math.isfinite(value):
                raise RuntimeError(f"non-finite observation: {path} {stamp}")
            by_year.setdefault(year, []).append((stamp, value))
    if not by_year:
        raise RuntimeError(f"empty observation series: {path}")
    peaks: dict[int, tuple[dt.date, float]] = {}
    census: list[dict[str, Any]] = []
    for year, samples in sorted(by_year.items()):
        nonmissing = [(stamp, value) for stamp, value in samples if value is not None]
        if not nonmissing:
            census.append({"water_year": year, "observation_disposition": "NO_NONMISSING_SWE"})
            continue
        maximum = max(value for _, value in nonmissing)
        if maximum <= 0.0:
            census.append({"water_year": year, "observation_disposition": "NO_POSITIVE_PEAK"})
            continue
        earliest = min(stamp for stamp, value in nonmissing if value == maximum)
        peaks[year] = (earliest, maximum)
        census.append(
            {
                "water_year": year,
                "observation_disposition": "POSITIVE_PEAK",
                "peak_date": earliest.isoformat(),
                "peak_swe_m": maximum,
            }
        )
    return peaks, census


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
    overall = row["stage3_evaluation_non_formulation_fingerprint_fnv1a64"]
    fingerprint_pattern = re.compile(r"^[0-9a-f]{16}$")
    if (
        surface != complete
        or surface != overall
        or not fingerprint_pattern.fullmatch(str(surface))
        or surface == "0000000000000000"
    ):
        raise RuntimeError("paired non-formulation fingerprint mismatch")
    for field in (
        "stage3_evaluation_source_fingerprint_fnv1a64",
        "stage3_evaluation_forcing_fingerprint_fnv1a64",
        "stage3_evaluation_geometry_fingerprint_fnv1a64",
    ):
        value = str(row[field])
        if not fingerprint_pattern.fullmatch(value) or value == "0000000000000000":
            raise RuntimeError(f"invalid evaluation fingerprint: {field}")


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
            if row.get("day_index") != index or row.get("lane_index") != 0:
                raise RuntimeError(f"trace identity mismatch on {stamp}")
            schema = row.get("schema")
            if schema == "openwepp-r7h-direct-production-snow-trace-v5":
                validate_tags(row, frozen["expected_tags"])
                validate_evaluation_row(row, stamp)
            elif schema == "openwepp-r7h-direct-production-snow-trace-v4":
                if row.get("active_snow_coupling") is not False:
                    raise RuntimeError(f"v4 row lacks inactive-snow reason on {stamp}")
                if any(key.startswith("stage3_evaluation_") for key in row):
                    raise RuntimeError(f"v4 row carries evaluation fields on {stamp}")
            else:
                raise RuntimeError(f"unexpected trace schema on {stamp}: {schema}")
            rows[stamp] = row
    if len(rows) != len(dates):
        raise RuntimeError("trace/climate row-count mismatch")
    return rows


def validate_evaluation_row(row: dict[str, Any], stamp: dt.date) -> None:
    tolerance = 1.0e-6
    arrays = {term: row[field] for term, field in HOURLY_FIELDS.items()}
    hourly_complete = row["stage3_evaluation_hourly_complete_energy_j_m2"]
    hourly_vapor = row["stage3_evaluation_hourly_vapor_mass_exchange_kg_m2"]
    hourly_zero = {field: row[field] for field in HOURLY_ZERO_FIELDS}
    if any(len(values) != 24 for values in arrays.values()):
        raise RuntimeError(f"invalid term array length on {stamp}")
    if len(hourly_complete) != 24 or len(hourly_vapor) != 24:
        raise RuntimeError(f"invalid complete/vapor array length on {stamp}")
    if any(len(values) != 24 for values in hourly_zero.values()):
        raise RuntimeError(f"invalid same-state zero array length on {stamp}")
    requested = row["stage3_evaluation_hourly_requested_seconds"]
    evaluated = row["stage3_evaluation_hourly_evaluated_seconds"]
    active = row["stage3_evaluation_hourly_complete_carrier_evaluated"]
    internal = row[
        "stage3_evaluation_hourly_internal_active_lower_conduction_j_m2"
    ]
    if any(len(values) != 24 for values in (requested, evaluated, active, internal)):
        raise RuntimeError(f"invalid support array length on {stamp}")
    if any(type(value) is not bool for value in active):
        raise RuntimeError(f"non-boolean hourly applicability on {stamp}")
    if any(active) and not all(active):
        raise RuntimeError(f"partial same-state daily support on {stamp}")
    for hour in range(24):
        require_close(f"requested support {stamp} h{hour}", float(requested[hour]), 3600.0, 0.0)
        expected_evaluated = 3600.0 if active[hour] else 0.0
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
            if not active[hour] and abs(value) > ZERO:
                raise RuntimeError(f"inactive {term} is nonzero on {stamp} h{hour}")
        vapor = float(hourly_vapor[hour])
        complete_energy = float(hourly_complete[hour])
        if not math.isfinite(vapor) or not math.isfinite(complete_energy):
            raise RuntimeError(f"non-finite complete/vapor operand on {stamp} h{hour}")
        if not active[hour] and (abs(vapor) > ZERO or abs(complete_energy) > ZERO):
            raise RuntimeError(f"inactive complete/vapor operand is nonzero on {stamp} h{hour}")
        require_close(
            f"hourly implemented external subset {stamp} h{hour}",
            complete_energy,
            sum(float(arrays[term][hour]) for term in TERMS),
            tolerance,
        )
        for field, values in hourly_zero.items():
            require_close(
                f"same-state zero {field} {stamp} h{hour}",
                float(values[hour]),
                0.0,
                0.0,
            )
    if row["stage3_evaluation_complete_arm_internal_conduction_applicable"]:
        raise RuntimeError(f"internal conduction unexpectedly applicable on {stamp}")
    expected_applicability = {
        "stage3_evaluation_surface_arm_applicable": True,
        "stage3_evaluation_surface_arm_sensible_applicable": False,
        "stage3_evaluation_surface_arm_advected_applicable": False,
        "stage3_evaluation_surface_arm_internal_conduction_applicable": False,
        "stage3_evaluation_complete_arm_applicable": True,
        "stage3_evaluation_complete_arm_internal_conduction_applicable": False,
        "stage3_evaluation_complete_arm_cold_content_export_applicable": False,
        "stage3_evaluation_complete_arm_available_ice_applicable": False,
        "stage3_evaluation_complete_arm_sequential_ledger_applicable": False,
        "stage3_evaluation_complete_arm_terminal_unallocated_applicable": False,
    }
    for field, expected in expected_applicability.items():
        if type(row[field]) is not bool or row[field] is not expected:
            raise RuntimeError(f"same-state applicability mismatch: {field}")
    for field in DAILY_ZERO_FIELDS:
        require_close(
            f"same-state daily zero {field} {stamp}",
            checked_float(row, field),
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
    surface_latent = checked_float(
        row, "stage3_evaluation_surface_arm_latent_j_m2"
    )
    surface = daily["shortwave"] + daily["longwave"] + surface_latent
    for term in ("shortwave", "longwave"):
        require_close(
            f"paired shared {term} {stamp}",
            checked_float(row, f"stage3_evaluation_surface_arm_{term}_j_m2"),
            daily[term],
            tolerance,
        )
    require_close(
        f"complete reconstruction {stamp}",
        checked_float(row, "stage3_evaluation_complete_arm_total_j_m2"),
        complete,
        tolerance,
    )
    require_close(
        f"daily/hourly complete energy {stamp}",
        complete,
        sum(float(value) for value in hourly_complete),
        tolerance,
    )
    require_close(
        f"daily/hourly vapor mass {stamp}",
        checked_float(row, "stage3_evaluation_complete_arm_vapor_mass_exchange_kg_m2"),
        sum(float(value) for value in hourly_vapor),
        tolerance,
    )
    require_close(
        f"surface reconstruction {stamp}",
        checked_float(row, "stage3_evaluation_surface_arm_total_j_m2"),
        surface,
        tolerance,
    )
    require_close(
        f"frozen disabled surface latent {stamp}",
        surface_latent,
        0.0,
        tolerance,
    )
    require_close(
        f"paired formulation delta {stamp}",
        complete - surface,
        daily["sensible"]
        + daily["advected"]
        + daily["latent"]
        - surface_latent,
        tolerance,
    )
    requested_total = sum(float(value) for value in requested)
    evaluated_total = sum(float(value) for value in evaluated)
    require_close(
        f"daily requested support {stamp}",
        checked_float(row, "stage3_evaluation_requested_seconds"),
        requested_total,
        0.0,
    )
    require_close(
        f"daily evaluated support {stamp}",
        checked_float(row, "stage3_evaluation_evaluated_seconds"),
        evaluated_total,
        0.0,
    )
    require_close(
        f"daily coverage {stamp}",
        checked_float(row, "stage3_evaluation_coverage_fraction"),
        evaluated_total / requested_total,
        1.0e-15,
    )


def annual_window(
    site: str,
    water_year: int,
    peak: tuple[dt.date, float],
    rows: dict[dt.date, dict[str, Any]],
    censored: set[int],
    support: dict[str, Any],
) -> dict[str, Any]:
    start = dt.date(water_year - 1, 10, 1)
    end, observed_peak_swe_m = peak
    stamps = [start + dt.timedelta(days=index) for index in range((end - start).days + 1)]
    if not stamps or any(stamp not in rows for stamp in stamps):
        raise RuntimeError(f"window outside trace chronology: {site} WY{water_year}")
    totals = {term: 0.0 for term in TERMS}
    surface_total = 0.0
    producer_surface_total = 0.0
    producer_complete_total = 0.0
    surface_latent_total = 0.0
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
        if not any(active):
            zero_coverage_v5_days += 1
        else:
            surface_total += (
                checked_float(row, "stage3_evaluation_surface_arm_shortwave_j_m2")
                + checked_float(row, "stage3_evaluation_surface_arm_longwave_j_m2")
                + checked_float(row, "stage3_evaluation_surface_arm_latent_j_m2")
            )
            producer_surface_total += checked_float(
                row, "stage3_evaluation_surface_arm_total_j_m2"
            )
            producer_complete_total += checked_float(
                row, "stage3_evaluation_complete_arm_total_j_m2"
            )
            surface_latent_total += checked_float(
                row, "stage3_evaluation_surface_arm_latent_j_m2"
            )
        for hour in range(24):
            if not active[hour]:
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
        "descriptive_eligible": evaluated_seconds > 0.0,
    }
    exclusion_reasons = []
    if evaluated_seconds <= 0.0:
        exclusion_reasons.append("NO_EVALUATED_SUPPORT")
    if water_year in censored:
        exclusion_reasons.append("RIGHT_CENSORED")
    if evaluated_seconds / 86_400.0 < support["minimum_evaluated_days_for_screen"]:
        exclusion_reasons.append("BELOW_30_EVALUATED_DAYS")
    if evaluated_seconds / calendar_seconds < support["minimum_calendar_coverage_for_screen"]:
        exclusion_reasons.append("BELOW_0P25_CALENDAR_COVERAGE")
    record["screen_eligible"] = not exclusion_reasons
    record["screen_exclusion_reasons"] = exclusion_reasons
    if evaluated_seconds <= 0.0:
        for term in TERMS + (
            "net_radiation",
            "turbulent",
            "implemented_external_subset",
            "surface",
            "surface_latent",
            "implemented_external_subset_minus_surface",
        ):
            record[f"{term}_sample_energy_mj_m2"] = None
            record[f"{term}_resolved_mean_w_m2"] = None
            record[f"{term}_calendar_mean_w_m2"] = None
        record["hourly_flux_distributions_w_m2"] = None
        return record
    derived = {
        **totals,
        "net_radiation": totals["shortwave"] + totals["longwave"],
        "turbulent": totals["sensible"] + totals["latent"],
        "implemented_external_subset": sum(totals.values()),
        "surface": surface_total,
        "surface_latent": surface_latent_total,
        "implemented_external_subset_minus_surface": sum(totals.values()) - surface_total,
    }
    abs_operands = sum(abs(value) for value in totals.values())
    annual_tolerance = max(1.0e-6, 1.0e-12 * abs_operands)
    require_close(
        f"water-year external-subset delta {site} WY{water_year}",
        derived["implemented_external_subset"] - derived["surface"],
        totals["sensible"]
        + totals["advected"]
        + totals["latent"]
        - surface_latent_total,
        annual_tolerance,
    )
    require_close(
        f"water-year producer external subset {site} WY{water_year}",
        derived["implemented_external_subset"],
        producer_complete_total,
        annual_tolerance,
    )
    require_close(
        f"water-year producer surface {site} WY{water_year}",
        derived["surface"],
        producer_surface_total,
        annual_tolerance,
    )
    for term, energy in derived.items():
        record[f"{term}_sample_energy_mj_m2"] = energy / 1.0e6
        record[f"{term}_resolved_mean_w_m2"] = energy / evaluated_seconds
        record[f"{term}_calendar_mean_w_m2"] = energy / calendar_seconds
    hourly_fluxes["net_radiation"] = [
        a + b for a, b in zip(hourly_fluxes["shortwave"], hourly_fluxes["longwave"])
    ]
    hourly_fluxes["turbulent"] = [
        a + b for a, b in zip(hourly_fluxes["sensible"], hourly_fluxes["latent"])
    ]
    hourly_fluxes["implemented_external_subset"] = [
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
    near = frozen["carrier_screen"]["near_balance_w_m2"]
    for site in [row["site"] for row in frozen["cohort"]]:
        descriptive = [row for row in annual if row["site"] == site and row["descriptive_eligible"]]
        eligible = [row for row in annual if row["site"] == site and row["screen_eligible"]]
        if len(eligible) < frozen["carrier_screen"]["minimum_screen_eligible_water_years_per_site"]:
            summaries.append(
                {
                    "site": site,
                    "descriptive_sample_count": len(descriptive),
                    "screen_eligible_sample_count": len(eligible),
                    "status": "NOT_COMPARABLE",
                    "reason": "INSUFFICIENT_SCREEN_ELIGIBLE_WATER_YEARS",
                }
            )
            continue
        summary: dict[str, Any] = {
            "site": site,
            "descriptive_sample_count": len(descriptive),
            "screen_eligible_sample_count": len(eligible),
            "water_year_min": min(row["water_year"] for row in eligible),
            "water_year_max": max(row["water_year"] for row in eligible),
            "coverage_fraction": distribution([row["coverage_fraction"] for row in eligible]),
        }
        for term in TERMS + (
            "net_radiation",
            "turbulent",
            "implemented_external_subset",
            "surface",
            "surface_latent",
            "implemented_external_subset_minus_surface",
        ):
            values = [row[f"{term}_resolved_mean_w_m2"] for row in eligible]
            summary[f"{term}_resolved_mean_w_m2"] = distribution(values)
            energies = [row[f"{term}_sample_energy_mj_m2"] for row in eligible]
            summary[f"{term}_sample_energy_mj_m2"] = distribution(energies)
        implemented_subset = summary[
            "implemented_external_subset_resolved_mean_w_m2"
        ]["median"]
        summary["near_balance_class"] = classify(
            implemented_subset, near[0], near[1]
        )
        summary["marks_external_total_context_class"] = "NOT_COMPARABLE_MISSING_SNOW_GROUND"
        summary["marks_term_comparison"] = "NOT_COMPARABLE_DIFFERENT_SITES_PERIODS_ESTIMAND"
        summary["roth_nolin_partition_comparison"] = "NOT_COMPARABLE_DIFFERENT_ESTIMAND"
        summaries.append(summary)
    return summaries


def apply_screen(site_summary: list[dict[str, Any]], frozen: dict[str, Any]) -> dict[str, Any]:
    canonical_sites = set(frozen["carrier_screen"]["canonical_decisive_sites"])
    canonical = [row for row in site_summary if row["site"] in canonical_sites]
    comparable = [row for row in canonical if row.get("status") != "NOT_COMPARABLE"]
    near_count = sum(row["near_balance_class"] == "WITHIN_CONTEXT" for row in comparable)
    passed = (
        len(comparable) == len(canonical_sites)
        and near_count >= frozen["carrier_screen"]["minimum_canonical_sites_within_near_balance"]
    )
    return {
        "status": "PASS" if passed else "FAIL",
        "canonical_decisive_sites": sorted(canonical_sites),
        "canonical_comparable_site_count": len(comparable),
        "canonical_near_balance_site_count": near_count,
        "snowbird_role": "DEVELOPMENT_ONLY_NON_DECISIVE_DIAGNOSTIC",
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


def validate_runfile_consumer(runfile: Path, expected_climate: Path) -> dict[str, Any]:
    with runfile.open("rb") as handle:
        document = tomllib.load(handle)
    consumed = Path(document["inputs"]["climate"]).resolve()
    expected = expected_climate.resolve()
    if consumed != expected or not consumed.is_file():
        raise RuntimeError(f"runfile climate consumer mismatch: {runfile}")
    outputs = document.get("outputs", {})
    if set(outputs) != {"pass", "loss", "wat"}:
        raise RuntimeError(f"runfile publication surface differs: {runfile}")
    stem = runfile.stem
    expected_outputs = {
        "pass": (runfile.parent / f"{stem}.hbp").resolve(),
        "loss": (runfile.parent / f"{stem}.loss.json").resolve(),
        "wat": (runfile.parent / f"{stem}.wat.parquet").resolve(),
    }
    consumed_outputs = {key: Path(value).resolve() for key, value in outputs.items()}
    if consumed_outputs != expected_outputs:
        raise RuntimeError(f"runfile publication path differs: {runfile}")
    return {
        "climate_path": relative(consumed),
        "climate_sha256": sha256(consumed),
        "publication_paths": {
            key: relative(path) for key, path in sorted(consumed_outputs.items())
        },
        "pass_is_exact_hbp": True,
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
    consumer = validate_runfile_consumer(runfile, climate_file(fixture))
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
    expected_suffixes = {".run", ".hbp", ".loss.json", ".wat.parquet", ".txt"}
    if lane == "paired":
        expected_suffixes.add(".snow.jsonl")
    outputs = {}
    for path in sorted(candidate for candidate in run_dir.iterdir() if candidate.is_file()):
        outputs[path.name] = {
            "path": relative(path),
            "sha256": sha256(path),
            "size_bytes": path.stat().st_size,
        }
    for suffix in expected_suffixes - {".txt"}:
        if sum(name.endswith(suffix) for name in outputs) != 1:
            raise RuntimeError(f"publication/output cardinality differs for {site}/{lane}: {suffix}")
    if sum(name.endswith(".txt") for name in outputs) != 2:
        raise RuntimeError(f"stdout/stderr custody differs for {site}/{lane}")
    return {
        "site": site,
        "lane": lane,
        "argv": [str(value) for value in command],
        "returncode": completed.returncode,
        "removed_openwepp_key_names": removed,
        "effective_openwepp_environment": effective,
        "outputs": outputs,
        "runfile_sha256": sha256(runfile),
        "runfile_consumer": consumer,
    }


def output_path(receipt: dict[str, Any], suffix: str) -> Path:
    matches = [value for name, value in receipt["outputs"].items() if name.endswith(suffix)]
    if len(matches) != 1:
        raise RuntimeError(f"expected one {suffix} output for {receipt['site']}/{receipt['lane']}")
    return REPO / matches[0]["path"]


def execute(expected_head: str) -> None:
    if OUTPUT.exists():
        raise RuntimeError(f"refusing to overwrite {OUTPUT}")
    frozen = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    if frozen["status"] != "frozen_before_result_execution":
        raise RuntimeError("protocol freeze is not active")
    head = assert_execution_source(expected_head)
    ancestor = subprocess.run(
        ["git", "merge-base", "--is-ancestor", frozen["scaffold_commit"], head],
        cwd=REPO,
        check=False,
    )
    if ancestor.returncode != 0:
        raise RuntimeError("execution HEAD does not descend from scaffold commit")
    if not SOURCE_FIXTURES.is_dir():
        raise RuntimeError(f"source fixtures are missing: {SOURCE_FIXTURES}")
    build_command = [
        "cargo",
        "build",
        "--release",
        "-p",
        "openwepp-runner",
        "--bin",
        "openwepp-cli-hill",
    ]
    build = subprocess.run(
        build_command, cwd=REPO, text=True, capture_output=True, check=False
    )
    if build.returncode != 0 or not BINARY.is_file():
        raise RuntimeError(f"release build failed: {build.stderr[-4000:]}")
    assert_execution_source(expected_head)
    binary_sha256 = sha256(BINARY)
    w1 = load_module("carrier_audit_w1", W1_TOOL)
    OUTPUT.mkdir(parents=True)
    retained_freeze = OUTPUT / "inputs/protocol-freeze.json"
    retained_freeze.parent.mkdir(parents=True)
    shutil.copyfile(FREEZE_PATH, retained_freeze)
    retained_binary = OUTPUT / "binary/openwepp-cli-hill"
    retained_binary.parent.mkdir(parents=True)
    shutil.copyfile(BINARY, retained_binary)
    (OUTPUT / "build.stdout.txt").write_text(build.stdout, encoding="utf-8")
    (OUTPUT / "build.stderr.txt").write_text(build.stderr, encoding="utf-8")
    build_receipt = {
        "argv": build_command,
        "returncode": build.returncode,
        "execution_head": head,
        "binary_sha256": binary_sha256,
    }
    fixture_receipts = {}
    fixtures = {}
    for frozen_site in frozen["cohort"]:
        site = frozen_site["site"]
        fixtures[site], fixture_receipts[site] = prepare_fixture(site, frozen_site)
        observation = OBSERVATIONS / f"{site}.csv"
        if sha256(observation) != frozen_site["observation_sha256"]:
            raise RuntimeError(f"observation hash differs for {site}")
        retained_observation(site).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(observation, retained_observation(site))
        fixture_receipts[site]["observation"] = {
            "source_path": relative(observation),
            "retained_path": relative(retained_observation(site)),
            "sha256": sha256(retained_observation(site)),
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
    assert_execution_source(expected_head)
    protected_identity = {}
    for site, lanes in receipts.items():
        control, paired = lanes["control"], lanes["paired"]
        wat_control = output_path(control, ".wat.parquet")
        wat_paired = output_path(paired, ".wat.parquet")
        hbp_control, hbp_paired = output_path(control, ".hbp"), output_path(paired, ".hbp")
        protected_identity[site] = {
            "wat_exact": sha256(wat_control) == sha256(wat_paired),
            "wat_control_sha256": sha256(wat_control),
            "wat_paired_sha256": sha256(wat_paired),
            "hbp_exact": sha256(hbp_control) == sha256(hbp_paired),
            "hbp_control_sha256": sha256(hbp_control),
            "hbp_paired_sha256": sha256(hbp_paired),
            "pass_output": "HBP is the runfile outputs.pass artifact",
        }
        if not protected_identity[site]["wat_exact"] or not protected_identity[site]["hbp_exact"]:
            raise RuntimeError(f"protected output differs for {site}")
    if sha256(BINARY) != binary_sha256:
        raise RuntimeError("release binary changed during execution")
    assert_execution_source(expected_head)
    analyze_and_write(
        frozen,
        receipts,
        fixture_receipts,
        protected_identity,
        head,
        build_receipt,
    )


def compute_analysis(
    frozen: dict[str, Any], receipts: dict[str, dict[str, Any]]
) -> tuple[list[dict[str, Any]], list[dict[str, Any]], dict[str, Any], dict[str, Any]]:
    annual = []
    observation_census: dict[str, list[dict[str, Any]]] = {}
    for frozen_site in frozen["cohort"]:
        site = frozen_site["site"]
        fixture = OUTPUT / "fixtures" / site
        trace = output_path(receipts[site]["paired"], ".snow.jsonl")
        dates = climate_dates(climate_file(fixture))
        rows = parse_trace(trace, dates, frozen)
        observation = retained_observation(site)
        peaks, census = observed_peaks(observation)
        observation_census[site] = census
        by_year = {row["water_year"]: row for row in census}
        for water_year, peak in sorted(peaks.items()):
            start = dt.date(water_year - 1, 10, 1)
            if start < dates[0] or peak[0] > dates[-1]:
                by_year[water_year]["analysis_disposition"] = "OUTSIDE_TRACE_CHRONOLOGY"
                continue
            record = annual_window(
                site,
                water_year,
                peak,
                rows,
                set(frozen["censored_primary_water_years"]),
                frozen["support"],
            )
            annual.append(record)
            by_year[water_year]["analysis_disposition"] = (
                "ANALYZED_RIGHT_CENSORED"
                if record["right_censored"]
                else "ANALYZED"
            )
            by_year[water_year]["screen_eligible"] = record["screen_eligible"]
            by_year[water_year]["screen_exclusion_reasons"] = record[
                "screen_exclusion_reasons"
            ]
        for row in census:
            row.setdefault("analysis_disposition", "NO_ANALYSIS_WINDOW")
    site_summary = summarize_sites(annual, frozen)
    screen = apply_screen(site_summary, frozen)
    return annual, site_summary, screen, observation_census


def analyze_and_write(
    frozen: dict[str, Any],
    receipts: dict[str, dict[str, Any]],
    fixture_receipts: dict[str, Any],
    protected_identity: dict[str, Any],
    head: str,
    build_receipt: dict[str, Any],
) -> None:
    annual, site_summary, screen, observation_census = compute_analysis(
        frozen, receipts
    )
    results = {
        "schema_version": 1,
        "evidence_mode": "Ran: exact-current release CLI four-site same-state paired carrier audit",
        "characterization_only": True,
        "freeze_sha256": sha256(OUTPUT / "inputs/protocol-freeze.json"),
        "execution_head": head,
        "snow_ground_boundary": "NOT_IMPLEMENTED",
        "internal_active_lower_conduction": "NOT_APPLICABLE_IN_SAME_STATE_PAIR_AND_EXACT_ZERO",
        "claim_class": "WATER_YEAR_STRATIFIED_INDEPENDENT_SAME_STATE_CONDITION_SAMPLES",
        "screen_eligible_sample_count": sum(row["screen_eligible"] for row in annual),
        "right_censored_window_count": sum(row["right_censored"] for row in annual),
        "water_year_condition_samples": annual,
        "site_summary": site_summary,
        "carrier_screen": screen,
        "observation_year_census": observation_census,
        "claim_limits": frozen["claim_limits"],
    }
    write_json(OUTPUT / "results/carrier-term-results.json", results)
    write_csv(
        OUTPUT / "tables/water-year-condition-samples.csv",
        [
            {
                key: value
                for key, value in row.items()
                if key != "hourly_flux_distributions_w_m2"
            }
            for row in annual
        ],
    )
    flat_site = []
    for row in site_summary:
        flat_site.append(
            {
                "site": row["site"],
                "descriptive_sample_count": row.get("descriptive_sample_count", 0),
                "screen_eligible_sample_count": row.get("screen_eligible_sample_count", 0),
                "median_coverage_fraction": row.get("coverage_fraction", {}).get("median"),
                **{
                    f"median_{term}_resolved_mean_w_m2": row.get(
                        f"{term}_resolved_mean_w_m2", {}
                    ).get("median")
                    for term in TERMS
                    + (
                        "net_radiation",
                        "turbulent",
                        "implemented_external_subset",
                        "surface",
                        "surface_latent",
                        "implemented_external_subset_minus_surface",
                    )
                },
                "near_balance_class": row.get("near_balance_class", "NOT_COMPARABLE"),
                "marks_external_total_context_class": row.get(
                    "marks_external_total_context_class", "NOT_COMPARABLE"
                ),
            }
        )
    write_csv(OUTPUT / "tables/site-summary.csv", flat_site)
    receipt = {
        "schema_version": 1,
        "status": "EXECUTED",
        "git_head": head,
        "git_status_porcelain": command_output(["git", "status", "--porcelain"]),
        "binary": {
            "path": relative(OUTPUT / "binary/openwepp-cli-hill"),
            "sha256": sha256(OUTPUT / "binary/openwepp-cli-hill"),
            "size_bytes": (OUTPUT / "binary/openwepp-cli-hill").stat().st_size,
        },
        "build": build_receipt,
        "freeze": {
            "source_path": relative(FREEZE_PATH),
            "retained_path": relative(OUTPUT / "inputs/protocol-freeze.json"),
            "sha256": sha256(OUTPUT / "inputs/protocol-freeze.json"),
        },
        "fixtures": fixture_receipts,
        "runs": receipts,
        "protected_output_identity": protected_identity,
    }
    write_json(OUTPUT / "execution-receipt.json", receipt)
    write_json(OUTPUT / "retained-artifact-manifest.json", retained_manifest(OUTPUT))
    assert_execution_source(head)
    verify_outputs(frozen)


def verify_outputs(frozen: dict[str, Any]) -> None:
    result_path = OUTPUT / "results/carrier-term-results.json"
    receipt_path = OUTPUT / "execution-receipt.json"
    if not result_path.is_file() or not receipt_path.is_file():
        raise RuntimeError("result or receipt is missing")
    results = json.loads(result_path.read_text(encoding="utf-8"))
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    if results["execution_head"] != receipt["git_head"]:
        raise RuntimeError("result and receipt execution heads differ")
    if results["freeze_sha256"] != sha256(OUTPUT / "inputs/protocol-freeze.json"):
        raise RuntimeError("result freeze hash differs")
    retained_binary = REPO / receipt["binary"]["path"]
    if (
        sha256(retained_binary) != receipt["binary"]["sha256"]
        or receipt["binary"]["sha256"] != receipt["build"]["binary_sha256"]
    ):
        raise RuntimeError("retained binary/build identity differs")
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
    frozen = json.loads(
        (OUTPUT / "inputs/protocol-freeze.json").read_text(encoding="utf-8")
    )
    receipt = json.loads((OUTPUT / "execution-receipt.json").read_text(encoding="utf-8"))
    manifest_path = OUTPUT / "retained-artifact-manifest.json"
    retained = json.loads(manifest_path.read_text(encoding="utf-8"))
    if retained != retained_manifest(OUTPUT):
        raise RuntimeError("retained artifact manifest differs")
    for site_receipts in receipt["runs"].values():
        for lane_receipt in site_receipts.values():
            for value in lane_receipt["outputs"].values():
                path = REPO / value["path"]
                if path.stat().st_size != value["size_bytes"] or sha256(path) != value["sha256"]:
                    raise RuntimeError(f"retained output identity differs: {path}")
    for frozen_site in frozen["cohort"]:
        site = frozen_site["site"]
        fixture = OUTPUT / "fixtures" / site
        if file_manifest(SOURCE_FIXTURES / site)["manifest_sha256"] != frozen_site["fixture_manifest_sha256"]:
            raise RuntimeError(f"source fixture identity differs: {site}")
        expected_climate_hash = frozen_site.get(
            "development_climate_sha256", sha256(climate_file(SOURCE_FIXTURES / site))
        )
        if sha256(climate_file(fixture)) != expected_climate_hash:
            raise RuntimeError(f"retained climate identity differs: {site}")
        observation = retained_observation(site)
        if sha256(observation) != frozen_site["observation_sha256"]:
            raise RuntimeError(f"observation identity differs: {site}")
        for lane in ("control", "paired"):
            lane_receipt = receipt["runs"][site][lane]
            runfile = output_path(lane_receipt, ".run")
            if validate_runfile_consumer(runfile, climate_file(fixture)) != lane_receipt["runfile_consumer"]:
                raise RuntimeError(f"runfile consumer receipt differs: {site}/{lane}")
        control = receipt["runs"][site]["control"]
        paired = receipt["runs"][site]["paired"]
        for suffix in (".wat.parquet", ".hbp"):
            if sha256(output_path(control, suffix)) != sha256(output_path(paired, suffix)):
                raise RuntimeError(f"protected output identity differs: {site} {suffix}")
    annual, site_summary, screen, census = compute_analysis(frozen, receipt["runs"])
    results = json.loads((OUTPUT / "results/carrier-term-results.json").read_text(encoding="utf-8"))
    if results["water_year_condition_samples"] != annual:
        raise RuntimeError("retained water-year reconstruction differs")
    if results["site_summary"] != site_summary or results["carrier_screen"] != screen:
        raise RuntimeError("retained site summary or screen differs")
    if results["observation_year_census"] != census:
        raise RuntimeError("retained observation census differs")
    verify_outputs(frozen)
    print("carrier-term audit verification: PASS")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-existing", action="store_true")
    parser.add_argument("--expected-head")
    args = parser.parse_args()
    if args.verify_existing:
        verify_existing()
    else:
        if args.expected_head is None:
            parser.error("--expected-head is required for result execution")
        execute(args.expected_head)


if __name__ == "__main__":
    main()
