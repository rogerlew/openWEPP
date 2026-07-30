#!/usr/bin/env python3
"""Independently reconstruct ASSURE-05 groundwater report values.

This procedure uses Python's standard library only. It does not import or call
openWEPP recurrence code. Results are written canonically to stdout so callers
can compare or retain them without letting this script mutate report sources.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from decimal import Decimal
from pathlib import Path
from typing import Any


def load_object(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError(f"{path} must contain one JSON object")
    return value


def number(mapping: dict[str, Any], key: str) -> float:
    value = mapping.get(key)
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise ValueError(f"{key} must be a JSON number")
    value = float(value)
    if not math.isfinite(value):
        raise ValueError(f"{key} must be finite")
    return value


def decimal(value: float) -> Decimal:
    return Decimal(str(value))


def entry(identifier: str, value: float, unit: str, precision: str) -> dict[str, Any]:
    if not math.isfinite(value):
        raise ValueError(f"calculated {identifier} is not finite")
    return {"id": identifier, "value": value, "unit_id": unit, "precision": precision}


def analytical_result(path: Path) -> dict[str, Any]:
    source = load_object(path)
    area = number(source, "area_m2")
    depth = number(source, "initial_storage_depth_m")
    interval = number(source, "interval_days")
    kb = number(source, "baseflow_coefficient_per_day")
    ks = number(source, "deep_seepage_coefficient_per_day")
    allowance = number(source, "absolute_allowance_m3")
    guard_kb = number(source, "guard_baseflow_coefficient_per_day")
    guard_ks = number(source, "guard_deep_seepage_coefficient_per_day")
    recharge_source = source.get("daily_recharge_m3")
    if not isinstance(recharge_source, list) or len(recharge_source) != 2:
        raise ValueError("daily_recharge_m3 must contain exactly two values")
    recharge = [float(value) for value in recharge_source]
    if not all(math.isfinite(value) and value >= 0.0 for value in recharge):
        raise ValueError("daily recharge must be finite and nonnegative")

    initial = area * depth
    storage = initial
    previous_baseflow = 0.0
    previous_seepage = 0.0
    observed: list[tuple[float, float, float]] = []
    for daily_recharge in recharge:
        storage = storage + daily_recharge - previous_baseflow - previous_seepage
        baseflow = kb * interval * storage
        seepage = ks * interval * storage
        observed.append((storage, baseflow, seepage))
        previous_baseflow = baseflow
        previous_seepage = seepage

    d_area = decimal(area)
    d_depth = decimal(depth)
    d_interval = decimal(interval)
    d_kb = decimal(kb)
    d_ks = decimal(ks)
    d_storage = d_area * d_depth
    d_previous_baseflow = Decimal(0)
    d_previous_seepage = Decimal(0)
    expected: list[tuple[Decimal, Decimal, Decimal]] = []
    for daily_recharge in recharge:
        d_storage = (
            d_storage
            + decimal(daily_recharge)
            - d_previous_baseflow
            - d_previous_seepage
        )
        d_baseflow = d_kb * d_interval * d_storage
        d_seepage = d_ks * d_interval * d_storage
        expected.append((d_storage, d_baseflow, d_seepage))
        d_previous_baseflow = d_baseflow
        d_previous_seepage = d_seepage

    residuals = [
        abs(observed_value - float(expected_value))
        for observed_day, expected_day in zip(observed, expected, strict=True)
        for observed_value, expected_value in zip(observed_day, expected_day, strict=True)
    ]
    maximum_residual = max(residuals)
    if maximum_residual > allowance:
        raise ValueError(
            f"two-day residual {maximum_residual:.17g} exceeds allowance {allowance:.17g}"
        )

    values = [
        entry("area", area, "m2", "exact declared analytical input"),
        entry("initial_storage_depth", depth, "m", "exact declared analytical input"),
        entry("initial_storage", initial, "m3", "independently reconstructed input"),
        entry("interval_days", interval, "d", "exact recurrence interval"),
        entry("baseflow_coefficient", kb, "d_inv", "exact declared analytical input"),
        entry("deep_seepage_coefficient", ks, "d_inv", "exact declared analytical input"),
        entry("guard_baseflow_coefficient", guard_kb, "d_inv", "exact negative-test input"),
        entry("guard_deep_seepage_coefficient", guard_ks, "d_inv", "exact negative-test input"),
        entry("day_1_recharge", recharge[0], "m3", "exact declared analytical input"),
        entry("day_1_storage", observed[0][0], "m3", "independent binary64 reconstruction"),
        entry("day_1_baseflow", observed[0][1], "m3", "independent binary64 reconstruction"),
        entry("day_1_deep_seepage", observed[0][2], "m3", "independent binary64 reconstruction"),
        entry("day_2_recharge", recharge[1], "m3", "exact declared analytical input"),
        entry("day_2_storage", observed[1][0], "m3", "independent binary64 reconstruction"),
        entry("day_2_baseflow", observed[1][1], "m3", "independent binary64 reconstruction"),
        entry("day_2_deep_seepage", observed[1][2], "m3", "independent binary64 reconstruction"),
        entry("maximum_absolute_residual", maximum_residual, "m3", "binary64 versus decimal reconstruction"),
        entry("acceptance_allowance", allowance, "m3", "preregistered absolute allowance"),
    ]
    return {"schema_version": 1, "result_id": "GW-RESULT-TWO-DAY", "values": values}


def digest(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            hasher.update(block)
    return hasher.hexdigest()


def matching_manifest_digest(manifest: dict[str, Any], filename: str) -> str:
    checksums = manifest.get("output_checksums")
    if not isinstance(checksums, dict):
        raise ValueError("manifest output_checksums must be an object")
    matches = [value for key, value in checksums.items() if Path(key).name == filename]
    if len(matches) != 1 or not isinstance(matches[0], str):
        raise ValueError(f"manifest must contain one checksum for {filename}")
    return matches[0]


def h2637_result(manifest_path: Path, hbp_path: Path, parquet_path: Path) -> dict[str, Any]:
    manifest = load_object(manifest_path)
    provenance = manifest.get("execution_provenance")
    if not isinstance(provenance, dict):
        raise ValueError("manifest execution_provenance must be an object")
    active = provenance.get("laned_active")
    if not isinstance(active, dict):
        raise ValueError("manifest must contain execution_provenance.laned_active")
    publication = manifest.get("wb13_publication")
    if not isinstance(publication, dict):
        raise ValueError("manifest wb13_publication must be an object")

    for path in (hbp_path, parquet_path):
        actual = digest(path)
        expected = matching_manifest_digest(manifest, path.name)
        if actual != expected:
            raise ValueError(f"{path.name} checksum {actual} differs from manifest {expected}")

    initial = number(active, "initial_groundwater_storage_m3")
    recharge = number(active, "total_groundwater_recharge_m3")
    baseflow = number(active, "total_groundwater_baseflow_m3")
    seepage = number(active, "total_groundwater_deep_seepage_m3")
    terminal = number(active, "terminal_groundwater_storage_m3")
    terminal_baseflow = number(active, "terminal_groundwater_baseflow_m3")
    terminal_seepage = number(active, "terminal_groundwater_deep_seepage_m3")

    recurrence_reconstructed = (
        initial + recharge - (baseflow - terminal_baseflow) - (seepage - terminal_seepage)
    )
    recurrence_residual = terminal - recurrence_reconstructed
    post_export_storage = terminal - terminal_baseflow - terminal_seepage
    full_run_storage = initial + recharge - baseflow - seepage
    post_export_residual = post_export_storage - full_run_storage
    recurrence_allowance = 1.0e-9 * max(abs(terminal), 1.0)
    post_export_allowance = 1.0e-9 * max(abs(post_export_storage), 1.0)
    if abs(recurrence_residual) > recurrence_allowance:
        raise ValueError("H2637 recurrence residual exceeds preregistered allowance")
    if abs(post_export_residual) > post_export_allowance:
        raise ValueError("H2637 post-export residual exceeds preregistered allowance")

    surface_source = number(active, "total_source_m3")
    surface_outlet = number(active, "total_routed_outlet_m3")
    surface_storage = number(active, "total_end_window_storage_m3")
    surface_clamp = number(active, "total_clamp_m3")
    surface_residual = surface_source - surface_outlet - surface_storage - surface_clamp
    surface_relative_residual = abs(surface_residual) / surface_source
    surface_allowance = 1.0e-9 * surface_source
    if abs(surface_residual) > surface_allowance:
        raise ValueError("H2637 active-routing surface residual exceeds allowance")

    values = [
        entry("duration_days", number(active, "days_seen"), "d", "exact produced manifest count"),
        entry("days_routed", number(active, "days_routed"), "d", "exact produced manifest count"),
        entry(
            "ofe_count",
            number(publication, "contributor_ofe_count"),
            "ofe_count",
            "produced manifest count checked against the HBP header",
        ),
        entry("initial_storage", initial, "m3", "produced manifest operand"),
        entry("cumulative_recharge", recharge, "m3", "produced manifest operand"),
        entry("cumulative_baseflow", baseflow, "m3", "produced manifest operand"),
        entry("cumulative_deep_seepage", seepage, "m3", "produced manifest operand"),
        entry("terminal_pre_export_storage", terminal, "m3", "produced manifest operand"),
        entry("terminal_day_baseflow", terminal_baseflow, "m3", "produced manifest operand"),
        entry("terminal_day_deep_seepage", terminal_seepage, "m3", "produced manifest operand"),
        entry("recurrence_reconstructed_storage", recurrence_reconstructed, "m3", "independent binary64 reconstruction"),
        entry("post_export_storage", post_export_storage, "m3", "independent binary64 reconstruction"),
        entry("full_run_reconstructed_storage", full_run_storage, "m3", "independent binary64 reconstruction"),
        entry("recurrence_residual", recurrence_residual, "m3", "independent binary64 reconstruction"),
        entry("post_export_residual", post_export_residual, "m3", "independent binary64 reconstruction"),
        entry("recurrence_allowance", recurrence_allowance, "m3", "preregistered storage-scaled allowance"),
        entry("post_export_allowance", post_export_allowance, "m3", "preregistered storage-scaled allowance"),
        entry("surface_source", surface_source, "m3", "produced manifest operand"),
        entry("surface_routed_outlet", surface_outlet, "m3", "produced manifest operand"),
        entry("surface_end_window_storage", surface_storage, "m3", "produced manifest operand"),
        entry("surface_clamp", surface_clamp, "m3", "produced manifest operand"),
        entry("surface_residual", surface_residual, "m3", "independent binary64 reconstruction"),
        entry("surface_relative_residual", surface_relative_residual, "dimensionless", "independent binary64 reconstruction"),
        entry("surface_allowance", surface_allowance, "m3", "preregistered source-scaled allowance"),
    ]
    return {"schema_version": 1, "result_id": "GW-RESULT-H2637", "values": values}


def parser() -> argparse.ArgumentParser:
    value = argparse.ArgumentParser(description=__doc__)
    subcommands = value.add_subparsers(dest="command", required=True)
    analytical = subcommands.add_parser("analytical")
    analytical.add_argument("--input", type=Path, required=True)
    h2637 = subcommands.add_parser("h2637")
    h2637.add_argument("--manifest", type=Path, required=True)
    h2637.add_argument("--hbp", type=Path, required=True)
    h2637.add_argument("--parquet", type=Path, required=True)
    return value


def main() -> None:
    arguments = parser().parse_args()
    if arguments.command == "analytical":
        result = analytical_result(arguments.input)
    else:
        result = h2637_result(arguments.manifest, arguments.hbp, arguments.parquet)
    print(json.dumps(result, sort_keys=True, separators=(",", ":"), allow_nan=False))


if __name__ == "__main__":
    main()
