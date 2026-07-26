#!/usr/bin/env python3
"""Validate CAL-03 daily JSONL and build Bill-method diagnostics."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

SCHEMA = "openwepp-canopy-research-daily-v1"
TOLERANCE = 1.0e-10
REQUIRED_NUMERIC_PATHS = (
    ("year",),
    ("day_of_year",),
    ("day_index",),
    ("lane_index",),
    ("gsi", "minimum_temperature_indicator"),
    ("gsi", "vapor_pressure_deficit_indicator"),
    ("gsi", "photoperiod_indicator"),
    ("gsi", "photoperiod_hours"),
    ("gsi", "instantaneous"),
    ("gsi", "gsi21"),
    ("gsi", "sample_count"),
    ("canopy", "structural_biomass_kg_m2"),
    ("canopy", "evergreen_foliar_biomass_kg_m2"),
    ("canopy", "deciduous_foliar_biomass_kg_m2"),
    ("canopy", "total_foliar_biomass_kg_m2"),
    ("canopy", "total_aboveground_live_biomass_kg_m2"),
    ("canopy", "leaf_area_index_m2_m2"),
    ("canopy", "cover_fraction"),
    ("canopy", "leaf_on_allocation_kg_m2"),
    ("canopy", "leaf_off_transfer_kg_m2"),
    ("consumers", "growth_live_foliar_biomass_kg_m2"),
    ("consumers", "snow_canopy_cover_fraction"),
    ("consumers", "interception_leaf_area_index_m2_m2"),
    ("consumers", "interception_canopy_cover_fraction"),
    ("consumers", "interception_live_biomass_kg_m2"),
    ("consumers", "interception_m"),
    ("consumers", "et_leaf_area_index_m2_m2"),
    ("consumers", "et_canopy_cover_fraction"),
    ("consumers", "runoff_m"),
    ("residue", "leaf_litter_input_kg_m2"),
    ("residue", "total_litter_input_kg_m2"),
    ("residue", "surface_residue_before_kg_m2"),
    ("residue", "surface_residue_after_kg_m2"),
    ("residue", "decomposition_loss_kg_m2"),
    ("residue", "surface_decay_factor"),
    ("residue", "residue_depth_m"),
)
REQUIRED_NULLABLE_NUMERIC_PATHS = (
    ("consumers", "erosion_canopy_cover_fraction"),
    ("consumers", "frost_residue_depth_m"),
    ("residue", "needle_litter_input_kg_m2"),
    ("residue", "fine_woody_litter_input_kg_m2"),
)


class ResearchError(ValueError):
    """A trace violates the CAL-03 evidence contract."""


def _number(record: dict[str, Any], *path: str) -> float:
    value: Any = record
    for key in path:
        if not isinstance(value, dict) or key not in value:
            raise ResearchError(f"missing field {'/'.join(path)}")
        value = value[key]
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise ResearchError(f"{'/'.join(path)} must be numeric")
    result = float(value)
    if not math.isfinite(result):
        raise ResearchError(f"{'/'.join(path)} must be finite")
    return result


def _nullable_number(record: dict[str, Any], *path: str) -> float | None:
    value: Any = record
    for key in path:
        if not isinstance(value, dict) or key not in value:
            raise ResearchError(f"missing field {'/'.join(path)}")
        value = value[key]
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise ResearchError(f"{'/'.join(path)} must be numeric or null")
    result = float(value)
    if not math.isfinite(result):
        raise ResearchError(f"{'/'.join(path)} must be finite or null")
    return result


def read_records(path: Path) -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip():
            continue
        try:
            record = json.loads(line)
        except json.JSONDecodeError as error:
            raise ResearchError(f"{path}:{line_number}: invalid JSON: {error}") from error
        if record.get("schema") != SCHEMA:
            raise ResearchError(f"{path}:{line_number}: unsupported schema")
        for group in ("gsi", "canopy", "consumers", "residue"):
            if not isinstance(record.get(group), dict):
                raise ResearchError(f"{path}:{line_number}: missing {group} object")
        _validate_record(record, path, line_number)
        records.append(record)
    if not records:
        raise ResearchError(f"{path}: no records")
    keys: list[tuple[str, str, int, str]] = []
    last_by_lane: dict[tuple[str, str, int], tuple[int, str]] = {}
    for row in records:
        site = row.get("site_id")
        arm = row.get("arm_id")
        if not isinstance(site, str) or not site.strip():
            raise ResearchError("site_id must be a nonempty string")
        if not isinstance(arm, str) or not arm.strip():
            raise ResearchError("arm_id must be a nonempty string")
        lane_key = (site, arm, int(row["lane_index"]))
        chronology = (int(row["day_index"]), str(row["date"]))
        if lane_key in last_by_lane and chronology <= last_by_lane[lane_key]:
            raise ResearchError("records must be chronological within each site/arm/lane")
        last_by_lane[lane_key] = chronology
        keys.append((*lane_key, str(row["date"])))
    if len(keys) != len(set(keys)):
        raise ResearchError("records must have unique site/arm/lane/date keys")
    previous: dict[tuple[str, str, int], float] = {}
    for record in records:
        key = (str(record.get("site_id")), str(record.get("arm_id")), int(record["lane_index"]))
        foliar = _number(record, "canopy", "total_foliar_biomass_kg_m2")
        if key in previous:
            expected = (
                previous[key]
                + _number(record, "canopy", "leaf_on_allocation_kg_m2")
                - _number(record, "canopy", "leaf_off_transfer_kg_m2")
            )
            if abs(expected - foliar) > TOLERANCE:
                raise ResearchError(f"{record['date']}: daily foliar ledger does not close")
        previous[key] = foliar
    return records


def _validate_record(record: dict[str, Any], path: Path, line_number: int) -> None:
    prefix = f"{path}:{line_number}"
    for field in ("schema", "date", "site_id", "arm_id"):
        value = record.get(field)
        if not isinstance(value, str) or not value:
            raise ResearchError(f"{prefix}: {field} must be a nonempty string")
    for numeric_path in REQUIRED_NUMERIC_PATHS:
        _number(record, *numeric_path)
    for numeric_path in REQUIRED_NULLABLE_NUMERIC_PATHS:
        _nullable_number(record, *numeric_path)
    for field in (
        "minimum_temperature_indicator",
        "vapor_pressure_deficit_indicator",
        "photoperiod_indicator",
        "instantaneous",
        "gsi21",
    ):
        value = _number(record, "gsi", field)
        if not 0.0 <= value <= 1.0:
            raise ResearchError(f"{prefix}: gsi/{field} outside [0,1]")
    pairs = (
        (("canopy", "total_foliar_biomass_kg_m2"), ("consumers", "growth_live_foliar_biomass_kg_m2")),
        (("canopy", "leaf_area_index_m2_m2"), ("consumers", "et_leaf_area_index_m2_m2")),
        (("canopy", "cover_fraction"), ("consumers", "et_canopy_cover_fraction")),
        (("canopy", "leaf_off_transfer_kg_m2"), ("residue", "leaf_litter_input_kg_m2")),
    )
    for producer, consumer in pairs:
        if abs(_number(record, *producer) - _number(record, *consumer)) > TOLERANCE:
            raise ResearchError(
                f"{prefix}: producer/consumer mismatch {'/'.join(producer)} -> {'/'.join(consumer)}"
            )
    before = _number(record, "residue", "surface_residue_before_kg_m2")
    litter = _number(record, "residue", "total_litter_input_kg_m2")
    after = _number(record, "residue", "surface_residue_after_kg_m2")
    loss = _number(record, "residue", "decomposition_loss_kg_m2")
    if abs(before + litter - loss - after) > TOLERANCE:
        raise ResearchError(f"{prefix}: aggregate surface-residue ledger does not close")


@dataclass
class Cohorts:
    current: float
    previous: float
    old: float

    @property
    def total(self) -> float:
        return self.current + self.previous + self.old


def annual_diagnostics(records: Iterable[dict[str, Any]]) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    groups: dict[tuple[str, str, int], list[dict[str, Any]]] = defaultdict(list)
    for record in records:
        key = (str(record.get("site_id")), str(record.get("arm_id")), int(record["lane_index"]))
        groups[key].append(record)
    annual_rows: list[dict[str, Any]] = []
    cohort_rows: list[dict[str, Any]] = []
    for (site, arm, lane), rows in sorted(groups.items()):
        cohorts = Cohorts(
            current=0.0,
            previous=0.0,
            old=_number(rows[0], "residue", "surface_residue_before_kg_m2"),
        )
        active_year: int | None = None
        year_rows: list[dict[str, Any]] = []
        start_total = cohorts.total
        for record in rows:
            year = int(record["year"])
            if active_year is not None and year != active_year:
                annual_rows.append(_summarize_year(site, arm, lane, active_year, year_rows))
                cohort_rows.append(_cohort_row(site, arm, lane, active_year, cohorts, start_total))
                cohorts = Cohorts(current=0.0, previous=cohorts.current, old=cohorts.old + cohorts.previous)
                start_total = cohorts.total
                year_rows = []
            active_year = year
            decay = _number(record, "residue", "surface_decay_factor")
            litter = _number(record, "residue", "total_litter_input_kg_m2")
            cohorts.current = (cohorts.current + litter) * decay
            cohorts.previous *= decay
            cohorts.old *= decay
            aggregate = _number(record, "residue", "surface_residue_after_kg_m2")
            if abs(cohorts.total - aggregate) > TOLERANCE:
                raise ResearchError(
                    f"{record['date']}: shadow cohorts {cohorts.total:.17g} "
                    f"do not reconcile to aggregate residue {aggregate:.17g}"
                )
            year_rows.append(record)
        if active_year is not None:
            annual_rows.append(_summarize_year(site, arm, lane, active_year, year_rows))
            cohort_rows.append(_cohort_row(site, arm, lane, active_year, cohorts, start_total))
        _annotate_equilibrium(annual_rows, site, arm, lane)
    return annual_rows, cohort_rows


def _summarize_year(site: str, arm: str, lane: int, year: int, rows: list[dict[str, Any]]) -> dict[str, Any]:
    foliar = [_number(row, "canopy", "total_foliar_biomass_kg_m2") for row in rows]
    deciduous = [_number(row, "canopy", "deciduous_foliar_biomass_kg_m2") for row in rows]
    leaf_on = sum(_number(row, "canopy", "leaf_on_allocation_kg_m2") for row in rows)
    leaf_off = sum(_number(row, "canopy", "leaf_off_transfer_kg_m2") for row in rows)
    residue = [_number(row, "residue", "surface_residue_after_kg_m2") for row in rows]
    amplitude = max(deciduous) - min(deciduous)
    pre_year_foliar = (
        foliar[0]
        - _number(rows[0], "canopy", "leaf_on_allocation_kg_m2")
        + _number(rows[0], "canopy", "leaf_off_transfer_kg_m2")
    )
    net_from_stocks = foliar[-1] - pre_year_foliar
    net_from_fluxes = leaf_on - leaf_off
    if abs(net_from_stocks - net_from_fluxes) > TOLERANCE:
        raise ResearchError(
            f"{site}/{arm}/{lane}/{year}: annual foliar stock-flow ledger does not close"
        )
    return {
        "site_id": site,
        "arm_id": arm,
        "lane_index": lane,
        "year": year,
        "day_count": len(rows),
        "gross_leaf_on_kg_m2": leaf_on,
        "gross_leaf_off_kg_m2": leaf_off,
        "net_foliar_change_kg_m2": net_from_fluxes,
        "seasonal_foliar_amplitude_kg_m2": amplitude,
        "phenology_churn_ratio": "" if amplitude <= TOLERANCE else leaf_off / amplitude,
        "residue_end_kg_m2": residue[-1],
        "residue_seasonal_range_kg_m2": max(residue) - min(residue),
        "residue_year_over_year_drift_kg_m2": "",
        "first_practical_equilibrium_year": "",
        "cal02_years_91_100_equilibrium": "NOT_EVALUABLE_PERIOD_LT_100",
    }


def _annotate_equilibrium(
    annual_rows: list[dict[str, Any]], site: str, arm: str, lane: int
) -> None:
    selected = [
        row
        for row in annual_rows
        if row["site_id"] == site and row["arm_id"] == arm and row["lane_index"] == lane
    ]
    for index, row in enumerate(selected):
        if index > 0:
            row["residue_year_over_year_drift_kg_m2"] = (
                row["residue_end_kg_m2"] - selected[index - 1]["residue_end_kg_m2"]
            )
    window_passes: list[bool] = []
    for start in range(max(0, len(selected) - 9)):
        window = selected[start : start + 10]
        stocks = [float(row["residue_end_kg_m2"]) for row in window]
        mean = sum(stocks) / len(stocks)
        window_passes.append(
            mean > 0.0 and (max(stocks) - min(stocks)) / mean <= 0.02
        )
    first_equilibrium = ""
    for start, passed in enumerate(window_passes):
        if passed and all(window_passes[start:]):
            first_equilibrium = selected[start + 9]["year"]
            break
    for row in selected:
        row["first_practical_equilibrium_year"] = first_equilibrium
    if len(selected) >= 100:
        window = selected[90:100]
        stocks = [float(row["residue_end_kg_m2"]) for row in window]
        mean = sum(stocks) / len(stocks)
        passed = mean > 0.0 and (max(stocks) - min(stocks)) / mean <= 0.02
        for row in selected:
            row["cal02_years_91_100_equilibrium"] = "PASS" if passed else "FAIL"


def _cohort_row(
    site: str, arm: str, lane: int, year: int, cohorts: Cohorts, start_total: float
) -> dict[str, Any]:
    return {
        "site_id": site,
        "arm_id": arm,
        "lane_index": lane,
        "year": year,
        "current_kg_m2": cohorts.current,
        "previous_kg_m2": cohorts.previous,
        "old_kg_m2": cohorts.old,
        "total_kg_m2": cohorts.total,
        "year_over_year_drift_kg_m2": cohorts.total - start_total,
    }


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        raise ResearchError(f"refusing to write empty {path}")
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("trace", type=Path)
    parser.add_argument("--annual", required=True, type=Path)
    parser.add_argument("--cohorts", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    args = parser.parse_args()
    records = read_records(args.trace)
    annual, cohorts = annual_diagnostics(records)
    write_csv(args.annual, annual)
    write_csv(args.cohorts, cohorts)
    manifest = {
        "schema": "openwepp-canopy-research-rebuild-v1",
        "trace": str(args.trace),
        "trace_sha256": hashlib.sha256(args.trace.read_bytes()).hexdigest(),
        "record_count": len(records),
        "annual_sha256": hashlib.sha256(args.annual.read_bytes()).hexdigest(),
        "cohorts_sha256": hashlib.sha256(args.cohorts.read_bytes()).hexdigest(),
    }
    args.manifest.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
