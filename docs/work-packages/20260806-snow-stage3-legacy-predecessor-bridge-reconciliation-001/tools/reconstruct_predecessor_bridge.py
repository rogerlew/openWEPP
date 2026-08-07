#!/usr/bin/env python3
"""Independently reconstruct the frozen predecessor bridge endpoint matrix.

This consumer does not import the execution runner or any producer reduction
helper. It parses retained JSONL primitives directly and writes compact results.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
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


class ReconstructionError(RuntimeError):
    """Raised when independent evidence reconstruction fails."""


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


def read_json_lines(path: Path) -> list[dict[str, Any]]:
    rows = []
    with path.open(encoding="utf-8") as handle:
        for line_number, line in enumerate(handle, start=1):
            try:
                row = json.loads(line)
            except json.JSONDecodeError as error:
                raise ReconstructionError(f"invalid JSON {path}:{line_number}: {error}") from error
            if not isinstance(row, dict):
                raise ReconstructionError(f"row is not an object: {path}:{line_number}")
            rows.append(row)
    return rows


def parse_v4(path: Path, dates: list[dt.date]) -> dict[dt.date, float]:
    rows = read_json_lines(path)
    if len(rows) != len(dates):
        raise ReconstructionError(f"v4 row count {len(rows)} != {len(dates)}")
    daily: dict[dt.date, float] = {}
    for expected_index, (stamp, row) in enumerate(zip(dates, rows, strict=True)):
        if text(row, "schema") != "openwepp-r7h-direct-production-snow-trace-v4":
            raise ReconstructionError(f"unexpected v4 schema at {stamp}")
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


def parse_v6(path: Path, dates: list[dt.date]) -> dict[dt.date, float]:
    rows = read_json_lines(path)
    if len(rows) != len(dates):
        raise ReconstructionError(f"v6 row count {len(rows)} != {len(dates)}")
    daily: dict[dt.date, float] = {}
    for expected_index, (stamp, row) in enumerate(zip(dates, rows, strict=True)):
        if text(row, "schema") != "openwepp-r7h-direct-production-snow-trace-v6":
            raise ReconstructionError(f"unexpected v6 schema at {stamp}")
        if integer(row, "day_index") != expected_index or integer(row, "lane_index") != 0:
            raise ReconstructionError(f"v6 daily identity mismatch at {stamp}")
        companion = required(row, "stage3_operator_reconciliation")
        if not isinstance(companion, dict) or integer(companion, "schema_version") != 6:
            raise ReconstructionError(f"missing v6 companion at {stamp}")
        statuses = required(companion, "hourly_status")
        tuples = required(companion, "tuples")
        if not isinstance(statuses, list) or len(statuses) != 24 or not isinstance(tuples, list):
            raise ReconstructionError(f"invalid v6 support shape at {stamp}")
        prior_end: dict[int, float] = {}
        total = 0.0
        per_hour_count = {hour: 0 for hour in range(24)}
        prior_substep = {hour: -1 for hour in range(24)}
        for tuple_row in tuples:
            if not isinstance(tuple_row, dict):
                raise ReconstructionError(f"nonobject tuple at {stamp}")
            hour = integer(tuple_row, "hour_index")
            substep = integer(tuple_row, "substep_index")
            if substep != prior_substep[hour] + 1:
                raise ReconstructionError(f"noncontiguous substep index at {stamp}/{hour}")
            prior_substep[hour] = substep
            per_hour_count[hour] += 1
            total += validate_v6_tuple(tuple_row, stamp=stamp, prior_end=prior_end)
        for hour, status in enumerate(statuses):
            if not isinstance(status, dict):
                raise ReconstructionError(f"nonobject hourly status at {stamp}/{hour}")
            evaluated = boolean(status, "evaluated")
            reason = text(status, "reason")
            if evaluated != (per_hour_count[hour] > 0):
                raise ReconstructionError(f"status/tuple mismatch at {stamp}/{hour}")
            if evaluated and reason != "evaluated":
                raise ReconstructionError(f"evaluated status reason mismatch at {stamp}/{hour}")
            if not evaluated and reason not in {
                "no_resolved_snow_at_day_start",
                "thin_pack_boundary_reached",
                "operator_not_selected",
            }:
                raise ReconstructionError(f"unknown status reason at {stamp}/{hour}")
        actual = number(row, "stage3_shadow_complete_energy_j_m2")
        close("v6 tuple-to-daily closure", actual, total, actual, total)
        daily[stamp] = total
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


def reconstruct_cells(
    frozen: dict[str, Any], dates: dict[str, list[dt.date]]
) -> tuple[dict[str, dict[int, float]], dict[str, Any]]:
    cells: dict[str, dict[int, float]] = {}
    selector_equivalence: dict[str, Any] = {}
    frozen_windows = windows(frozen)
    for cell, value in frozen["endpoint_matrix"].items():
        if not cell.startswith("E") or not isinstance(value, list) or len(value) != 2:
            continue
        source, forcing = value
        path = trace_path(cell, "legacy")
        daily = parse_v4(path, dates[forcing]) if source == "old" else parse_v6(path, dates[forcing])
        cells[cell] = annualize(daily, frozen_windows)
        if source == "current":
            explicit_daily = parse_v6(trace_path(cell, "explicit"), dates[forcing])
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
    return cells, selector_equivalence


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
    canonical: dict[str, Any], development: dict[str, Any]
) -> list[str]:
    classes = [
        "FORCING_IDENTITY_DIFFERENCE",
        "FORCING_STRATIFIED_ENDPOINTS_RECONCILED",
        "VERSIONED_ESTIMANDS_RECONCILED",
    ]
    for name, gate in (("canonical", canonical), ("development", development)):
        if gate["pass"]:
            classes.extend(
                [
                    f"SOURCE_INVARIANT_WITHIN_FORCING[{name}]",
                    f"CURRENT_V6_FORCING_MATCHED_PREDECESSOR_REPRODUCED[{name}]",
                ]
            )
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
    dates = {
        forcing: climate_dates(OUTPUT / "fixtures" / forcing / "p8.cli")
        for forcing in ("canonical", "development")
    }
    for forcing, stamps in dates.items():
        expected = frozen["forcings"]
        if len(stamps) != expected["date_count"] or stamps[0].isoformat() != expected["first_date"] or stamps[-1].isoformat() != expected["last_date"]:
            raise ReconstructionError(f"{forcing} chronology differs from freeze")
    cells, equivalence = reconstruct_cells(frozen, dates)
    rows = effect_rows(cells)
    canonical = source_gate(rows, "source_canonical_j_m2", "E10_j_m2", "E00_j_m2")
    development = source_gate(
        rows, "source_development_j_m2", "E11_j_m2", "E01_j_m2"
    )
    result = {
        "schema_version": 1,
        "status": "reconstructed",
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
        "checkpoint_lanes_triggered": [
            name
            for name, gate in (("canonical", canonical), ("development", development))
            if gate["checkpoint_trigger"]
        ],
        "decision_classes": classify(canonical, development),
        "claim_limits": [
            "schema-v4 aggregate custody only",
            "not mechanistic explanation",
            "not physical correctness or validation",
            "not persistence, terminal receipt, promotion, CoE retirement, or cutover",
        ],
    }
    write_json(RESULT_PATH, result)
    write_json(OUTPUT / "retained-artifact-manifest.json", retained_manifest(OUTPUT))
    print(json.dumps(result["cell_medians_mj_m2"], sort_keys=True))
    print(json.dumps(result["source_gates"], sort_keys=True))


def verify_existing() -> None:
    manifest_path = OUTPUT / "retained-artifact-manifest.json"
    if not manifest_path.is_file() or not RESULT_PATH.is_file():
        raise ReconstructionError("missing retained result or manifest")
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    for item in manifest["files"]:
        path = OUTPUT / item["path"]
        if not path.is_file() or sha256(path) != item["sha256"] or path.stat().st_size != item["size_bytes"]:
            raise ReconstructionError(f"retained artifact differs: {item['path']}")
    result = json.loads(RESULT_PATH.read_text(encoding="utf-8"))
    if result.get("status") != "reconstructed" or len(result.get("per_water_year", [])) != 35:
        raise ReconstructionError("retained result structure differs")
    print(f"PASS verified {manifest['file_count']} retained artifacts")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--reconstruct", action="store_true")
    group.add_argument("--verify-existing", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        reconstruct() if args.reconstruct else verify_existing()
    except ReconstructionError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
