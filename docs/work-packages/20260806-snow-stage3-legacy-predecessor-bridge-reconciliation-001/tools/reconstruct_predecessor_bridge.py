#!/usr/bin/env python3
"""Independently reconstruct the frozen predecessor bridge endpoint matrix.

This consumer does not import the execution runner or any producer reduction
helper. It parses retained JSONL primitives directly and writes compact results.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import importlib.util
import json
import math
import statistics
import sys
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
FREEZE_PATH = PACKAGE / "artifacts/protocol-freeze.json"
OUTPUT = REPO / "target/snow_stage3_legacy_predecessor_bridge_reconciliation"
TRACE_NAME = "snowbird-predecessor-bridge.snow.jsonl"
RESULT_PATH = OUTPUT / "results/predecessor-bridge-results.json"
INDEPENDENT_V6_TOOL = REPO / (
    "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-"
    "operator-reconciliation-001/tools/run_operator_reconciliation.py"
)
_INDEPENDENT_V6: Any | None = None


class ReconstructionError(RuntimeError):
    """Raised when independent evidence reconstruction fails."""


def independent_v6_consumer() -> Any:
    """Load the prior reviewed independent primitive solver, never producer code."""
    global _INDEPENDENT_V6
    if _INDEPENDENT_V6 is not None:
        return _INDEPENDENT_V6
    spec = importlib.util.spec_from_file_location(
        "predecessor_bridge_independent_v6_physics", INDEPENDENT_V6_TOOL
    )
    if spec is None or spec.loader is None:
        raise ReconstructionError(f"cannot load independent v6 consumer {INDEPENDENT_V6_TOOL}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    _INDEPENDENT_V6 = module
    return module


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def required(row: dict[str, Any], field: str) -> Any:
    if field not in row:
        raise ReconstructionError(f"missing required field {field}")
    return row[field]


def number(row: dict[str, Any], field: str) -> float:
    value = required(row, field)
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise ReconstructionError(f"{field} is not numeric")
    result = float(value)
    if not math.isfinite(result):
        raise ReconstructionError(f"{field} is not finite")
    return result


def integer(row: dict[str, Any], field: str) -> int:
    value = required(row, field)
    if isinstance(value, bool) or not isinstance(value, int):
        raise ReconstructionError(f"{field} is not an integer")
    return value


def boolean(row: dict[str, Any], field: str) -> bool:
    value = required(row, field)
    if not isinstance(value, bool):
        raise ReconstructionError(f"{field} is not a boolean")
    return value


def text(row: dict[str, Any], field: str) -> str:
    value = required(row, field)
    if not isinstance(value, str) or not value:
        raise ReconstructionError(f"{field} is not a non-empty string")
    return value


def tolerance(*operands: float, floor: float = 1.0e-6) -> float:
    return max(floor, 1.0e-12 * sum(abs(value) for value in operands))


def close(
    name: str,
    actual: float,
    expected: float,
    *operands: float,
    floor: float = 1.0e-6,
) -> None:
    limit = tolerance(*(operands or (actual, expected)), floor=floor)
    if abs(actual - expected) > limit:
        raise ReconstructionError(
            f"{name}: actual={actual:.17g} expected={expected:.17g} tol={limit:.6g}"
        )


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
        raise ReconstructionError(f"invalid climate chronology: {path}")
    return dates


def stream_json_lines(path: Path, expected_sha256: str | None = None) -> Any:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for line_number, raw in enumerate(handle, start=1):
            digest.update(raw)
            try:
                row = json.loads(raw)
            except (UnicodeDecodeError, json.JSONDecodeError) as error:
                raise ReconstructionError(f"invalid JSON {path}:{line_number}: {error}") from error
            if not isinstance(row, dict):
                raise ReconstructionError(f"row is not an object: {path}:{line_number}")
            yield line_number, row
    if expected_sha256 is not None and digest.hexdigest() != expected_sha256:
        raise ReconstructionError(f"trace hash differs: {path}")


def parse_v4(
    path: Path,
    dates: list[dt.date],
    *,
    expected_sha256: str | None = None,
    allowed_schemas: frozenset[str] = frozenset(
        {"openwepp-r7h-direct-production-snow-trace-v4"}
    ),
) -> dict[dt.date, float]:
    daily: dict[dt.date, float] = {}
    count = 0
    for line_number, row in stream_json_lines(path, expected_sha256):
        expected_index = line_number - 1
        if expected_index >= len(dates):
            raise ReconstructionError(f"v4 has extra row {line_number}")
        stamp = dates[expected_index]
        if text(row, "schema") not in allowed_schemas:
            raise ReconstructionError(f"unexpected aggregate schema at {stamp}")
        if integer(row, "day_index") != expected_index or integer(row, "lane_index") != 0:
            raise ReconstructionError(f"v4 daily identity mismatch at {stamp}")
        boolean(row, "stage3_energy_enabled")
        hourly = required(row, "stage3_shadow_hourly_complete_energy_j_m2")
        if not isinstance(hourly, list) or len(hourly) != 24:
            raise ReconstructionError(f"v4 hourly shape mismatch at {stamp}")
        values = []
        for hour, value in enumerate(hourly):
            if isinstance(value, bool) or not isinstance(value, (int, float)):
                raise ReconstructionError(f"v4 nonnumeric hour {stamp}/{hour}")
            value = float(value)
            if not math.isfinite(value):
                raise ReconstructionError(f"v4 nonfinite hour {stamp}/{hour}")
            values.append(value)
        actual = number(row, "stage3_shadow_complete_energy_j_m2")
        close("v4 hourly-to-daily closure", actual, sum(values), actual, *values)
        residual = number(row, "stage3_shadow_maximum_energy_closure_residual_j_m2")
        if abs(residual) > tolerance(actual, *values):
            raise ReconstructionError(f"v4 producer residual is material at {stamp}")
        daily[stamp] = actual
        count += 1
    if count != len(dates):
        raise ReconstructionError(f"v4 row count {count} != {len(dates)}")
    return daily


def validate_v6_tuple(
    tuple_row: dict[str, Any], *, stamp: dt.date, prior_end: dict[int, float]
) -> float:
    if not boolean(tuple_row, "applicable") or text(tuple_row, "applicability_reason") != "evaluated":
        raise ReconstructionError(f"non-applicable tuple serialized at {stamp}")
    hour = integer(tuple_row, "hour_index")
    substep = integer(tuple_row, "substep_index")
    if hour < 0 or hour > 23 or substep < 0:
        raise ReconstructionError(f"invalid tuple identity at {stamp}")
    start = number(tuple_row, "elapsed_start_seconds")
    duration = number(tuple_row, "duration_seconds")
    if duration <= 0.0 or start < 0.0 or start + duration > 3600.0 + 1.0e-9:
        raise ReconstructionError(f"invalid tuple support at {stamp}/{hour}/{substep}")
    expected = prior_end.get(hour, 0.0)
    if start != expected:
        raise ReconstructionError(
            f"tuple continuity at {stamp}/{hour}/{substep}: {start} != {expected}"
        )
    prior_end[hour] = start + duration
    terms = [
        number(tuple_row, field)
        for field in (
            "net_shortwave_w_m2",
            "net_longwave_w_m2",
            "sensible_flux_w_m2",
            "latent_flux_w_m2",
            "precipitation_advected_flux_w_m2",
        )
    ]
    external = number(tuple_row, "complete_external_flux_w_m2")
    close(
        "v6 external primitive closure",
        external,
        sum(terms),
        external,
        *terms,
        floor=1.0e-10,
    )
    conduction = number(tuple_row, "internal_active_lower_conduction_j_m2")
    legacy = number(tuple_row, "legacy_sequential_complete_j_m2")
    reconstructed = external * duration + conduction
    close("v6 legacy aggregate closure", legacy, reconstructed, legacy, reconstructed)
    before_mass = number(tuple_row, "total_ice_mass_before_kg_m2")
    melt = number(tuple_row, "melt_kg_m2")
    sublimation = number(tuple_row, "sublimation_kg_m2")
    deposition = number(tuple_row, "deposition_kg_m2")
    after_mass = number(tuple_row, "total_ice_mass_after_kg_m2")
    close(
        "v6 mass endpoint",
        after_mass,
        before_mass - melt - sublimation + deposition,
        after_mass,
        before_mass,
        melt,
        sublimation,
        deposition,
        floor=1.0e-12,
    )
    before_cold = number(tuple_row, "total_cold_before_j_m2")
    active_cold = number(tuple_row, "active_cold_energy_change_j_m2")
    lower_cold = number(tuple_row, "lower_cold_energy_change_j_m2")
    cold_export = number(tuple_row, "cold_content_export_j_m2")
    after_cold = number(tuple_row, "total_cold_after_j_m2")
    close(
        "v6 cold endpoint",
        after_cold,
        before_cold - active_cold - lower_cold - cold_export,
        after_cold,
        before_cold,
        active_cold,
        lower_cold,
        cold_export,
    )
    return legacy


def numeric_array(row: dict[str, Any], field: str, size: int) -> list[float]:
    values = required(row, field)
    if not isinstance(values, list) or len(values) != size:
        raise ReconstructionError(f"{field} shape differs")
    result = []
    for value in values:
        if isinstance(value, bool) or not isinstance(value, (int, float)):
            raise ReconstructionError(f"{field} contains a nonnumeric value")
        converted = float(value)
        if not math.isfinite(converted):
            raise ReconstructionError(f"{field} contains a nonfinite value")
        result.append(converted)
    return result


def parse_v6(
    path: Path, dates: list[dt.date], *, expected_sha256: str | None = None
) -> dict[dt.date, float]:
    independent = independent_v6_consumer()
    daily: dict[dt.date, float] = {}
    count = 0
    for line_number, row in stream_json_lines(path, expected_sha256):
        expected_index = line_number - 1
        if expected_index >= len(dates):
            raise ReconstructionError(f"v6 has extra row {line_number}")
        stamp = dates[expected_index]
        if integer(row, "day_index") != expected_index or integer(row, "lane_index") != 0:
            raise ReconstructionError(f"v6 daily identity mismatch at {stamp}")
        try:
            tuples = independent.validate_v6_row(
                row, "sequential_resolved_shadow_v1", "snotel_snowbird_ut"
            )
        except RuntimeError as error:
            raise ReconstructionError(f"independent v6 validation failed at {stamp}: {error}") from error
        if text(row, "stage3_evaluation_carrier_id") != "stage3_complete_carrier_v1":
            raise ReconstructionError(f"v6 carrier identity differs at {stamp}")
        if text(row, "stage3_evaluation_cadence_id") != "stage3_dynamic_substep_with_hourly_forcing_v1":
            raise ReconstructionError(f"v6 cadence identity differs at {stamp}")
        if text(row, "stage3_evaluation_claim_class") != "bounded_response_experiment":
            raise ReconstructionError(f"v6 claim class differs at {stamp}")
        total = 0.0
        term_totals = {name: 0.0 for name in ("shortwave", "longwave", "sensible", "latent", "advected")}
        conduction_total = 0.0
        hourly_total = [0.0] * 24
        hourly_seconds = [0.0] * 24
        hourly_evaluated = [False] * 24
        for tuple_row in tuples:
            hour = integer(tuple_row, "hour_index")
            duration = number(tuple_row, "duration_seconds")
            reconstructed = required(tuple_row, "_reconstructed")
            if not isinstance(reconstructed, dict):
                raise ReconstructionError(f"missing independent primitives at {stamp}/{hour}")
            external_j_m2 = number(reconstructed, "external") * duration
            conduction = number(tuple_row, "internal_active_lower_conduction_j_m2")
            legacy = external_j_m2 + conduction
            close(
                "v6 independent legacy bridge",
                number(tuple_row, "legacy_sequential_complete_j_m2"),
                legacy,
                legacy,
            )
            for name in term_totals:
                term_totals[name] += number(reconstructed, name) * duration
            conduction_total += conduction
            total += legacy
            hourly_total[hour] += legacy
            hourly_seconds[hour] += duration
            hourly_evaluated[hour] = True
        top_terms = {
            "shortwave": "stage3_evaluation_complete_arm_shortwave_j_m2",
            "longwave": "stage3_evaluation_complete_arm_longwave_j_m2",
            "sensible": "stage3_evaluation_complete_arm_sensible_j_m2",
            "latent": "stage3_evaluation_complete_arm_latent_j_m2",
            "advected": "stage3_evaluation_complete_arm_advected_j_m2",
        }
        for name, field in top_terms.items():
            close(f"v6 daily {name}", number(row, field), term_totals[name], term_totals[name])
        close(
            "v6 daily conduction",
            number(row, "stage3_evaluation_complete_arm_internal_active_lower_conduction_j_m2"),
            conduction_total,
            conduction_total,
        )
        close(
            "v6 tuple-to-daily closure",
            number(row, "stage3_evaluation_complete_arm_total_j_m2"),
            total,
            total,
        )
        serialized_hourly = numeric_array(row, "stage3_evaluation_hourly_complete_energy_j_m2", 24)
        serialized_seconds = numeric_array(row, "stage3_evaluation_hourly_evaluated_seconds", 24)
        requested = numeric_array(row, "stage3_evaluation_hourly_requested_seconds", 24)
        carrier_evaluated = required(row, "stage3_evaluation_hourly_complete_carrier_evaluated")
        if not isinstance(carrier_evaluated, list) or carrier_evaluated != hourly_evaluated:
            raise ReconstructionError(f"v6 hourly evaluated flags differ at {stamp}")
        for hour in range(24):
            close(f"v6 hourly total {hour}", serialized_hourly[hour], hourly_total[hour], hourly_total[hour])
            if serialized_seconds[hour] != hourly_seconds[hour] or requested[hour] != 3_600.0:
                raise ReconstructionError(f"v6 hourly support differs at {stamp}/{hour}")
        evaluated_seconds = sum(hourly_seconds)
        close(
            "v6 daily evaluated seconds",
            number(row, "stage3_evaluation_evaluated_seconds"),
            evaluated_seconds,
            floor=1.0e-10,
        )
        if number(row, "stage3_evaluation_requested_seconds") != 86_400.0:
            raise ReconstructionError(f"v6 requested support differs at {stamp}")
        close(
            "v6 coverage",
            number(row, "stage3_evaluation_coverage_fraction"),
            evaluated_seconds / 86_400.0,
            floor=1.0e-12,
        )
        if abs(number(row, "stage3_maximum_conduction_cancellation_residual_j_m2")) > 1.0e-6:
            raise ReconstructionError(f"v6 conduction cancellation differs at {stamp}")
        daily[stamp] = total
        count += 1
    if count != len(dates):
        raise ReconstructionError(f"v6 row count {count} != {len(dates)}")
    return daily


def windows(frozen: dict[str, Any]) -> list[tuple[int, dt.date, dt.date]]:
    values = []
    for year, start, end in frozen["windows"]:
        values.append((int(year), dt.date.fromisoformat(start), dt.date.fromisoformat(end)))
    if [year for year, _, _ in values] != list(range(1990, 2025)):
        raise ReconstructionError("frozen windows are not WY1990-2024")
    return values


def annualize(
    daily: dict[dt.date, float], frozen_windows: list[tuple[int, dt.date, dt.date]]
) -> dict[int, float]:
    annual = {}
    for year, start, end in frozen_windows:
        stamps = []
        current = start
        while current <= end:
            if current not in daily:
                raise ReconstructionError(f"missing daily value for {current}")
            stamps.append(current)
            current += dt.timedelta(days=1)
        annual[year] = sum(daily[stamp] for stamp in stamps)
    return annual


def trace_path(cell: str, mode: str) -> Path:
    return OUTPUT / "runs" / cell / mode / TRACE_NAME


def validate_execution_receipt(
    receipt: dict[str, Any], frozen: dict[str, Any]
) -> dict[tuple[str, str], str]:
    if receipt.get("status") != "endpoint_matrix_executed":
        raise ReconstructionError("execution receipt status differs")
    if receipt.get("sources") != frozen["sources"]:
        raise ReconstructionError("execution source inventory differs")
    expected_cells = {
        key: value
        for key, value in frozen["endpoint_matrix"].items()
        if key in {"E00", "E01", "E10", "E11"}
    }
    cells = receipt.get("cells")
    if not isinstance(cells, dict) or set(cells) != set(expected_cells):
        raise ReconstructionError("execution cell inventory differs")
    trace_hashes = {}
    for cell, (source_name, forcing) in expected_cells.items():
        modes = cells[cell]
        expected_modes = {"control", "legacy", "explicit"} if source_name == "current" else {"control", "legacy"}
        if not isinstance(modes, dict) or set(modes) != expected_modes:
            raise ReconstructionError(f"execution mode inventory differs for {cell}")
        for mode in expected_modes:
            arm = modes[mode]
            if (
                arm.get("cell") != cell
                or arm.get("mode") != mode
                or arm.get("source_sha") != frozen["sources"][source_name]
                or arm.get("forcing") != forcing
                or arm.get("returncode") != 0
            ):
                raise ReconstructionError(f"execution arm identity differs for {cell}/{mode}")
            semantic = arm.get("normalized_semantic_inputs", {})
            if (
                semantic.get("forcing") != forcing
                or semantic.get("forcing_sha256") != frozen["forcings"][forcing]["sha256"]
                or semantic.get("source_sha") != frozen["sources"][source_name]
            ):
                raise ReconstructionError(f"semantic input identity differs for {cell}/{mode}")
            if mode != "control":
                matches = [
                    item
                    for item in arm.get("outputs", {}).get("files", [])
                    if item.get("path") == TRACE_NAME
                ]
                if len(matches) != 1 or not isinstance(matches[0].get("sha256"), str):
                    raise ReconstructionError(f"trace receipt differs for {cell}/{mode}")
                trace_hashes[(cell, mode)] = matches[0]["sha256"]
        protected = receipt.get("protected_outputs", {}).get(cell, {})
        for mode in expected_modes - {"control"}:
            checks = protected.get(mode)
            if (
                not isinstance(checks, dict)
                or set(checks) != {".hbp", ".wat.parquet", ".loss.json"}
                or set(checks.values()) != {True}
            ):
                raise ReconstructionError(f"protected output custody differs for {cell}/{mode}")
    if receipt.get("forcing_matched_semantic_checks") != {
        "canonical": {"control": True, "legacy": True},
        "development": {"control": True, "legacy": True},
    }:
        raise ReconstructionError("forcing-matched semantic proof differs")
    if receipt.get("current_selector_semantic_checks") != {"E10": True, "E11": True}:
        raise ReconstructionError("current selector semantic proof differs")
    return trace_hashes


def reconstruct_cells(
    frozen: dict[str, Any],
    dates: dict[str, list[dt.date]],
    trace_hashes: dict[tuple[str, str], str],
) -> tuple[
    dict[str, dict[int, float]],
    dict[str, Any],
    dict[str, dict[dt.date, float]],
]:
    cells: dict[str, dict[int, float]] = {}
    daily_cells: dict[str, dict[dt.date, float]] = {}
    selector_equivalence: dict[str, Any] = {}
    frozen_windows = windows(frozen)
    for cell, value in frozen["endpoint_matrix"].items():
        if not cell.startswith("E") or not isinstance(value, list) or len(value) != 2:
            continue
        source, forcing = value
        path = trace_path(cell, "legacy")
        if source == "old":
            daily = parse_v4(
                path,
                dates[forcing],
                expected_sha256=trace_hashes[(cell, "legacy")],
            )
        else:
            daily = parse_v6(
                path,
                dates[forcing],
                expected_sha256=trace_hashes[(cell, "legacy")],
            )
        daily_cells[cell] = daily
        cells[cell] = annualize(daily, frozen_windows)
        if source == "current":
            explicit_daily = parse_v6(
                trace_path(cell, "explicit"),
                dates[forcing],
                expected_sha256=trace_hashes[(cell, "explicit")],
            )
            if daily.keys() != explicit_daily.keys():
                raise ReconstructionError(f"selector daily identity mismatch for {cell}")
            maximum_delta = max(abs(daily[stamp] - explicit_daily[stamp]) for stamp in daily)
            for stamp in daily:
                close(
                    f"selector equivalence {cell} {stamp}",
                    daily[stamp],
                    explicit_daily[stamp],
                    daily[stamp],
                    explicit_daily[stamp],
                )
            selector_equivalence[cell] = {
                "pass": True,
                "maximum_daily_abs_delta_j_m2": maximum_delta,
            }
    if set(cells) != {"E00", "E01", "E10", "E11"}:
        raise ReconstructionError(f"incomplete endpoint matrix: {sorted(cells)}")
    return cells, selector_equivalence, daily_cells


def replay_gate(
    fresh: dict[dt.date, float],
    retained: dict[dt.date, float],
    frozen_windows: list[tuple[int, dt.date, dt.date]],
) -> dict[str, Any]:
    if fresh.keys() != retained.keys():
        return {"pass": False, "failure": "daily_identity", "water_year_failures": []}
    daily_failures = []
    for stamp in fresh:
        limit = tolerance(fresh[stamp], retained[stamp])
        if abs(fresh[stamp] - retained[stamp]) > limit:
            daily_failures.append(stamp.isoformat())
            if len(daily_failures) == 10:
                break
    fresh_annual = annualize(fresh, frozen_windows)
    retained_annual = annualize(retained, frozen_windows)
    annual_failures = []
    for year in fresh_annual:
        limit = tolerance(fresh_annual[year], retained_annual[year])
        if abs(fresh_annual[year] - retained_annual[year]) > limit:
            annual_failures.append(year)
    paired = [fresh_annual[year] - retained_annual[year] for year in fresh_annual]
    median_delta = statistics.median(paired)
    median_pass = abs(median_delta / 1.0e6) <= 1.0e-7
    return {
        "pass": not daily_failures and not annual_failures and median_pass,
        "daily_failure_examples": daily_failures,
        "water_year_failures": annual_failures,
        "median_paired_difference_j_m2": median_delta,
    }


def effect_rows(cells: dict[str, dict[int, float]]) -> list[dict[str, Any]]:
    rows = []
    for year in range(1990, 2025):
        e00, e01, e10, e11 = (cells[cell][year] for cell in ("E00", "E01", "E10", "E11"))
        rows.append(
            {
                "water_year": year,
                "E00_j_m2": e00,
                "E01_j_m2": e01,
                "E10_j_m2": e10,
                "E11_j_m2": e11,
                "source_canonical_j_m2": e10 - e00,
                "source_development_j_m2": e11 - e01,
                "forcing_old_j_m2": e01 - e00,
                "forcing_current_j_m2": e11 - e10,
                "interaction_j_m2": (e11 - e10) - (e01 - e00),
            }
        )
    return rows


def source_gate(
    rows: list[dict[str, Any]], field: str, left: str, right: str
) -> dict[str, Any]:
    failures = []
    for row in rows:
        difference = float(row[field])
        limit = tolerance(float(row[left]), float(row[right]))
        if abs(difference) > limit:
            failures.append(
                {
                    "water_year": row["water_year"],
                    "difference_j_m2": difference,
                    "tolerance_j_m2": limit,
                }
            )
    median_j_m2 = statistics.median(float(row[field]) for row in rows)
    median_pass = abs(median_j_m2 / 1.0e6) <= 1.0e-7
    return {
        "pass": not failures and median_pass,
        "water_year_failures": failures,
        "median_paired_difference_j_m2": median_j_m2,
        "median_tolerance_mj_m2": 1.0e-7,
        "checkpoint_trigger": bool(failures) or not median_pass,
    }


def classify(
    canonical: dict[str, Any],
    development: dict[str, Any],
    replay: dict[str, dict[str, Any]],
    frozen: dict[str, Any],
) -> list[str]:
    classes = []
    replay_pass = all(item["pass"] for item in replay.values())
    if not replay_pass:
        classes.append("INPUT_OR_ENDPOINT_REPLAY_FAILURE")
    classes.append("FORCING_IDENTITY_DIFFERENCE")
    if replay_pass:
        classes.append("FORCING_STRATIFIED_ENDPOINTS_RECONCILED")
    reproduced = []
    for name, gate in (("canonical", canonical), ("development", development)):
        forcing_sha = frozen["forcings"][name]["sha256"]
        if gate["pass"]:
            classes.append(f"SOURCE_INVARIANT_WITHIN_FORCING[{forcing_sha}]")
            reproduced.append(
                f"CURRENT_V6_FORCING_MATCHED_PREDECESSOR_REPRODUCED[{forcing_sha}]"
            )
        else:
            classes.append(f"PREDECESSOR_NOT_REPRODUCED[{forcing_sha}]")
    if replay_pass:
        classes.append("VERSIONED_ESTIMANDS_RECONCILED")
    classes.extend(reproduced)
    return classes


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
    return {"schema_version": 1, "file_count": len(files), "files": files}


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(value, allow_nan=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def reconstruct() -> None:
    if RESULT_PATH.exists():
        raise ReconstructionError(f"refusing to overwrite {RESULT_PATH}")
    receipt_path = OUTPUT / "execution-receipt.json"
    if not receipt_path.is_file():
        raise ReconstructionError("missing execution receipt")
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    frozen = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    if receipt.get("protocol_sha256") != sha256(FREEZE_PATH):
        raise ReconstructionError("execution protocol hash differs")
    trace_hashes = validate_execution_receipt(receipt, frozen)
    dates = {
        forcing: climate_dates(OUTPUT / "fixtures" / forcing / "p8.cli")
        for forcing in ("canonical", "development")
    }
    for forcing, stamps in dates.items():
        expected = frozen["forcings"]
        climate = OUTPUT / "fixtures" / forcing / "p8.cli"
        if (
            sha256(climate) != expected[forcing]["sha256"]
            or len(stamps) != expected["date_count"]
            or stamps[0].isoformat() != expected["first_date"]
            or stamps[-1].isoformat() != expected["last_date"]
        ):
            raise ReconstructionError(f"{forcing} chronology differs from freeze")
    cells, equivalence, daily_cells = reconstruct_cells(frozen, dates, trace_hashes)
    retained_historical = parse_v4(
        REPO / frozen["retained_custody"]["historical_trace"]["path"],
        dates["canonical"],
        expected_sha256=frozen["retained_custody"]["historical_trace"]["sha256"],
    )
    retained_current = parse_v6(
        REPO / frozen["retained_custody"]["current_trace"]["path"],
        dates["development"],
        expected_sha256=frozen["retained_custody"]["current_trace"]["sha256"],
    )
    frozen_windows = windows(frozen)
    replay = {
        "historical_v4_E00": replay_gate(
            daily_cells["E00"], retained_historical, frozen_windows
        ),
        "current_v6_E11": replay_gate(
            daily_cells["E11"], retained_current, frozen_windows
        ),
    }
    rows = effect_rows(cells)
    canonical = source_gate(rows, "source_canonical_j_m2", "E10_j_m2", "E00_j_m2")
    development = source_gate(
        rows, "source_development_j_m2", "E11_j_m2", "E01_j_m2"
    )
    decision_classes = classify(canonical, development, replay, frozen)
    triggered_lanes = [
        name
        for name, gate in (("canonical", canonical), ("development", development))
        if gate["checkpoint_trigger"]
    ]
    result = {
        "schema_version": 1,
        "status": (
            "reconstructed"
            if all(item["pass"] for item in replay.values())
            else "input_or_endpoint_replay_failure"
        ),
        "execution_head": receipt["execution_head"],
        "claim_class": "aggregate_custody_and_operator_mechanics_only",
        "coe_authority": "unchanged",
        "cell_medians_mj_m2": {
            cell: statistics.median(values.values()) / 1.0e6
            for cell, values in cells.items()
        },
        "per_water_year": rows,
        "source_gates": {
            "canonical": canonical,
            "development": development,
        },
        "selector_equivalence": equivalence,
        "retained_endpoint_replay": replay,
        "independent_v6_consumer": {
            "path": str(INDEPENDENT_V6_TOOL.relative_to(REPO)),
            "sha256": sha256(INDEPENDENT_V6_TOOL),
            "role": "reviewed independent primitive equations; no runner or producer reduction",
        },
        "checkpoint_lanes_triggered": triggered_lanes,
        "decision_classes": decision_classes,
        "claim_limits": [
            "schema-v4 aggregate custody only",
            "not mechanistic explanation",
            "not physical correctness or validation",
            "not persistence, terminal receipt, promotion, CoE retirement, or cutover",
        ],
    }
    write_json(RESULT_PATH, result)
    write_json(
        OUTPUT / "results/checkpoint-trigger-receipt.json",
        {
            "schema_version": 1,
            "execution_head": receipt["execution_head"],
            "protocol_sha256": sha256(FREEZE_PATH),
            "endpoint_execution_receipt_sha256": sha256(receipt_path),
            "endpoint_result_sha256": sha256(RESULT_PATH),
            "triggered_lanes": triggered_lanes,
        },
    )
    write_json(OUTPUT / "retained-artifact-manifest.json", retained_manifest(OUTPUT))
    print(json.dumps(result["cell_medians_mj_m2"], sort_keys=True))
    print(json.dumps(result["source_gates"], sort_keys=True))


def checkpoint_trace_hash(arm: dict[str, Any]) -> str:
    matches = [
        item
        for item in arm.get("outputs", {}).get("files", [])
        if item.get("path") == TRACE_NAME
    ]
    if len(matches) != 1 or not isinstance(matches[0].get("sha256"), str):
        raise ReconstructionError("checkpoint trace receipt differs")
    return matches[0]["sha256"]


def parse_checkpoint_trace(
    path: Path, dates: list[dt.date], expected_sha256: str
) -> dict[dt.date, float]:
    with path.open("rb") as handle:
        first = handle.readline()
    try:
        first_row = json.loads(first)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ReconstructionError(f"checkpoint first row is invalid: {path}") from error
    schema = first_row.get("schema") if isinstance(first_row, dict) else None
    if schema == "openwepp-r7h-direct-production-snow-trace-v6":
        return parse_v6(path, dates, expected_sha256=expected_sha256)
    if schema in {
        "openwepp-r7h-direct-production-snow-trace-v4",
        "openwepp-r7h-direct-production-snow-trace-v5",
    }:
        return parse_v4(
            path,
            dates,
            expected_sha256=expected_sha256,
            allowed_schemas=frozenset({schema}),
        )
    raise ReconstructionError(f"unsupported checkpoint trace schema: {schema}")


def annual_difference_gate(
    left: dict[int, float], right: dict[int, float]
) -> dict[str, Any]:
    failures = []
    paired = []
    for year in range(1990, 2025):
        difference = right[year] - left[year]
        paired.append(difference)
        limit = tolerance(left[year], right[year])
        if abs(difference) > limit:
            failures.append(year)
    median_delta = statistics.median(paired)
    median_pass = abs(median_delta / 1.0e6) <= 1.0e-7
    return {
        "pass": not failures and median_pass,
        "water_year_failures": failures,
        "median_paired_difference_j_m2": median_delta,
    }


def reconstruct_checkpoints() -> None:
    destination = OUTPUT / "checkpoint-search"
    execution_path = destination / "execution-receipt.json"
    result_path = OUTPUT / "results/predecessor-bridge-results.json"
    output_path = destination / "checkpoint-results.json"
    if output_path.exists():
        raise ReconstructionError(f"refusing to overwrite {output_path}")
    if not execution_path.is_file() or not result_path.is_file():
        raise ReconstructionError("checkpoint execution and endpoint result are required")
    execution = json.loads(execution_path.read_text(encoding="utf-8"))
    endpoint_result = json.loads(result_path.read_text(encoding="utf-8"))
    frozen = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    lanes = endpoint_result.get("checkpoint_lanes_triggered")
    if execution.get("triggered_lanes") != lanes:
        raise ReconstructionError("checkpoint execution trigger differs")
    if execution.get("endpoint_result_sha256") != sha256(result_path):
        raise ReconstructionError("checkpoint endpoint-result binding differs")
    if not lanes:
        if execution.get("status") != "not_triggered" or execution.get("runs") != {}:
            raise ReconstructionError("untriggered checkpoint receipt differs")
        result = {
            "schema_version": 1,
            "status": "not_triggered",
            "triggered_lanes": [],
            "first_divergent_transition": {},
        }
        write_json(output_path, result)
        write_json(OUTPUT / "retained-artifact-manifest.json", retained_manifest(OUTPUT))
        return
    if execution.get("status") != "executed" or not isinstance(lanes, list):
        raise ReconstructionError("triggered checkpoint execution receipt differs")
    checkpoints = frozen["checkpoint_grouping"]["checkpoints"]
    checkpoint_ids = [f"{index:02d}-{source_sha}" for index, (source_sha, _) in enumerate(checkpoints)]
    dates = {
        forcing: climate_dates(OUTPUT / "fixtures" / forcing / "p8.cli")
        for forcing in lanes
    }
    lane_results = {}
    first_divergent = {}
    for forcing in lanes:
        retained_runs = execution.get("runs", {}).get(forcing)
        if not isinstance(retained_runs, dict) or list(retained_runs) != checkpoint_ids:
            raise ReconstructionError(f"checkpoint ordering differs for {forcing}")
        annual_by_checkpoint = {}
        for checkpoint_id in checkpoint_ids:
            item = retained_runs[checkpoint_id]
            modes = item.get("modes")
            if not isinstance(modes, dict) or set(modes) != {"control", "legacy"}:
                raise ReconstructionError(f"checkpoint modes differ for {forcing}/{checkpoint_id}")
            if set(item.get("protected_outputs", {}).values()) != {True}:
                raise ReconstructionError(f"checkpoint protected outputs differ for {forcing}/{checkpoint_id}")
            arm = modes["legacy"]
            trace = destination / "runs" / forcing / checkpoint_id / "legacy" / TRACE_NAME
            daily = parse_checkpoint_trace(trace, dates[forcing], checkpoint_trace_hash(arm))
            annual_by_checkpoint[checkpoint_id] = annualize(daily, windows(frozen))
        transitions = []
        for left_id, right_id in zip(checkpoint_ids, checkpoint_ids[1:]):
            gate = annual_difference_gate(
                annual_by_checkpoint[left_id], annual_by_checkpoint[right_id]
            )
            transitions.append({"left": left_id, "right": right_id, **gate})
        divergent = next((item for item in transitions if not item["pass"]), None)
        first_divergent[forcing] = divergent
        lane_results[forcing] = {
            "checkpoint_medians_mj_m2": {
                checkpoint_id: statistics.median(values.values()) / 1.0e6
                for checkpoint_id, values in annual_by_checkpoint.items()
            },
            "transitions": transitions,
        }
    result = {
        "schema_version": 1,
        "status": "reconstructed",
        "triggered_lanes": lanes,
        "first_divergent_transition": first_divergent,
        "lanes": lane_results,
        "claim_limit": "descriptive build-input localization only",
    }
    write_json(output_path, result)
    write_json(OUTPUT / "retained-artifact-manifest.json", retained_manifest(OUTPUT))


def verify_existing() -> None:
    manifest_path = OUTPUT / "retained-artifact-manifest.json"
    if not manifest_path.is_file() or not RESULT_PATH.is_file():
        raise ReconstructionError("missing retained result or manifest")
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if retained_manifest(OUTPUT) != manifest:
        raise ReconstructionError("retained artifact inventory has missing, extra, or changed files")
    for item in manifest["files"]:
        path = OUTPUT / item["path"]
        if not path.is_file() or sha256(path) != item["sha256"] or path.stat().st_size != item["size_bytes"]:
            raise ReconstructionError(f"retained artifact differs: {item['path']}")
    result = json.loads(RESULT_PATH.read_text(encoding="utf-8"))
    if result.get("status") not in {"reconstructed", "input_or_endpoint_replay_failure"} or len(result.get("per_water_year", [])) != 35:
        raise ReconstructionError("retained result structure differs")
    trigger = json.loads(
        (OUTPUT / "results/checkpoint-trigger-receipt.json").read_text(encoding="utf-8")
    )
    if (
        trigger.get("endpoint_result_sha256") != sha256(RESULT_PATH)
        or trigger.get("triggered_lanes") != result.get("checkpoint_lanes_triggered")
    ):
        raise ReconstructionError("retained checkpoint trigger differs")
    checkpoint_result = OUTPUT / "checkpoint-search/checkpoint-results.json"
    if (OUTPUT / "checkpoint-search/execution-receipt.json").exists() and not checkpoint_result.is_file():
        raise ReconstructionError("checkpoint execution lacks independent reconstruction")
    print(f"PASS verified {manifest['file_count']} retained artifacts")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--reconstruct", action="store_true")
    group.add_argument("--reconstruct-checkpoints", action="store_true")
    group.add_argument("--verify-existing", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        if args.reconstruct:
            reconstruct()
        elif args.reconstruct_checkpoints:
            reconstruct_checkpoints()
        else:
            verify_existing()
    except ReconstructionError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
