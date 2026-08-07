#!/usr/bin/env python3
"""Independently audit retained Stage 3 evolving-carrier plausibility evidence."""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import importlib.util
import json
import math
import re
import statistics
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any, Iterable

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
PROTOCOL = PACKAGE / "artifacts/protocol-freeze.json"
PREDECESSOR = REPO / (
    "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-"
    "operator-reconciliation-001/tools/run_operator_reconciliation.py"
)
CARRIER = REPO / (
    "docs/work-packages/20260806-snow-stage3-four-site-carrier-term-audit-001/"
    "tools/run_carrier_term_audit.py"
)
LATENT_HEAT_FUSION_J_KG = 333_600.0
MASS_FLOOR = 1.0e-12
ENERGY_FLOOR = 1.0e-6
FLUX_FLOOR = 1.0e-10
CANONICAL_SITES = {
    "snotel_mica_creek_st_joe_id",
    "snotel_niwot_co",
    "snotel_paradise_wa",
}
TERMS = ("shortwave", "longwave", "sensible", "latent", "advected")


class PhysicalPlausibilityFailure(RuntimeError):
    """Evidence-valid tuple whose represented physical state is implausible."""

    def __init__(
        self, message: str, reconstructed: dict[str, float] | None = None
    ) -> None:
        super().__init__(message)
        self.reconstructed = reconstructed


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def checked_number(row: dict[str, Any], field: str) -> float:
    value = row.get(field)
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise RuntimeError(f"{field} is not numeric")
    result = float(value)
    if not math.isfinite(result):
        raise RuntimeError(f"{field} is not finite")
    return result


def tolerance(floor: float, *operands: float) -> float:
    return max(floor, 1.0e-12 * sum(abs(value) for value in operands))


def require_close(name: str, actual: float, expected: float, floor: float) -> None:
    allowed = tolerance(floor, actual, expected)
    if abs(actual - expected) > allowed:
        raise RuntimeError(
            f"{name} mismatch: actual={actual:.17g} expected={expected:.17g} "
            f"tolerance={allowed:.17g}"
        )


def bounded_vapor(raw: float, active_ice_before: float) -> dict[str, float | bool]:
    if not math.isfinite(raw) or not math.isfinite(active_ice_before):
        raise RuntimeError("raw vapor and active ice must be finite")
    if active_ice_before < 0.0:
        raise RuntimeError("active ice before transfer is negative")
    deposition = max(raw, 0.0)
    sublimation = min(max(-raw, 0.0), active_ice_before)
    return {
        "deposition": deposition,
        "sublimation": sublimation,
        "signed": deposition - sublimation,
        "capacity_truncated": (-raw - sublimation)
        > tolerance(MASS_FLOOR, raw, sublimation),
        "truncated_mass": max(-raw - sublimation, 0.0),
    }


def reconstruct_carrier(row: dict[str, Any], external_flux: float) -> dict[str, float]:
    duration = checked_number(row, "duration_seconds")
    cold_before = checked_number(row, "active_cold_before_j_m2")
    conduction = checked_number(row, "internal_active_lower_conduction_j_m2")
    raw_vapor = checked_number(row, "vapor_mass_exchange_kg_m2")
    active_ice = checked_number(row, "active_ice_mass_before_kg_m2")
    vapor = bounded_vapor(raw_vapor, active_ice)
    cold_after_conduction = cold_before - conduction
    external_raw = external_flux * duration
    surface_change = (
        min(external_raw, cold_after_conduction)
        if external_raw >= 0.0
        else external_raw
    )
    active_change = conduction + surface_change
    lower_change = -conduction
    complete_raw = external_raw + conduction
    excess_raw = max(complete_raw - active_change, 0.0)
    ice_available = max(active_ice - float(vapor["sublimation"]), 0.0)
    melt = min(excess_raw / LATENT_HEAT_FUSION_J_KG, ice_available)
    unallocated = max(excess_raw - LATENT_HEAT_FUSION_J_KG * melt, 0.0)
    latent_raw = checked_number(row, "latent_flux_w_m2") * duration
    latent_heat = row.get("surface_latent_heat_j_kg")
    if isinstance(latent_heat, bool) or not isinstance(latent_heat, (int, float)):
        if abs(raw_vapor) > MASS_FLOOR:
            raise RuntimeError("nonzero raw vapor lacks surface latent heat")
        latent_heat = 0.0
    require_close(
        "raw vapor/latent identity",
        latent_raw,
        raw_vapor * float(latent_heat),
        ENERGY_FLOOR,
    )
    latent_bounded = float(vapor["signed"]) * float(latent_heat)
    return {
        **{key: float(value) for key, value in vapor.items()},
        "external_raw": external_raw,
        "active_change": active_change,
        "lower_change": lower_change,
        "complete_raw": complete_raw,
        "excess_raw": excess_raw,
        "ice_available": ice_available,
        "melt": melt,
        "unallocated": unallocated,
        "latent_raw": latent_raw,
        "latent_bounded": latent_bounded,
        "latent_truncation": latent_raw - latent_bounded,
    }


def validate_q_tuple(row: dict[str, Any]) -> dict[str, float]:
    physical_failures = []
    for field in (
        "active_ice_mass_before_kg_m2",
        "total_ice_mass_before_kg_m2",
        "total_ice_mass_after_kg_m2",
        "active_depth_before_m",
        "active_density_before_kg_m3",
        "active_cold_before_j_m2",
        "total_cold_before_j_m2",
        "total_cold_after_j_m2",
    ):
        if checked_number(row, field) < 0.0:
            physical_failures.append(f"{field} is negative")
    surface_before = checked_number(row, "surface_temperature_before_c")
    if surface_before <= -273.15 or surface_before > 0.0:
        physical_failures.append("resolved pre-transfer surface temperature is out of domain")
    if row.get("after_surface_applicable") is True:
        for field in (
            "active_ice_mass_after_kg_m2",
            "active_depth_after_m",
            "active_density_after_kg_m3",
            "active_cold_after_j_m2",
        ):
            if checked_number(row, field) < 0.0:
                physical_failures.append(f"{field} is negative")
        surface_after = checked_number(row, "surface_temperature_after_c")
        if surface_after <= -273.15 or surface_after > 0.0:
            physical_failures.append("resolved post-transfer surface temperature is out of domain")
    producer_deposition = checked_number(row, "deposition_kg_m2")
    producer_sublimation = checked_number(row, "sublimation_kg_m2")
    producer_melt = checked_number(row, "melt_kg_m2")
    if producer_deposition < 0.0 or producer_sublimation < 0.0 or producer_melt < 0.0:
        raise RuntimeError("producer bounded transfer or melt is negative")
    if producer_deposition > MASS_FLOOR and producer_sublimation > MASS_FLOOR:
        raise RuntimeError("producer deposition and sublimation are simultaneous")
    if checked_number(row, "active_ice_mass_before_kg_m2") < 0.0:
        raise PhysicalPlausibilityFailure("; ".join(physical_failures))
    reconstructed = row.get("_reconstructed")
    if not isinstance(reconstructed, dict):
        raise RuntimeError("tuple lacks independent primitive reconstruction")
    result = reconstruct_carrier(row, checked_number(reconstructed, "external"))
    if result["deposition"] > MASS_FLOOR and producer_sublimation > MASS_FLOOR:
        raise RuntimeError("raw-positive vapor is labeled sublimation")
    if result["sublimation"] > MASS_FLOOR and producer_deposition > MASS_FLOOR:
        raise RuntimeError("raw-negative vapor is labeled deposition")
    require_close("independent deposition", producer_deposition, result["deposition"], MASS_FLOOR)
    require_close("independent sublimation", producer_sublimation, result["sublimation"], MASS_FLOOR)
    require_close(
        "independent active cold change",
        checked_number(row, "active_cold_energy_change_j_m2"),
        result["active_change"],
        ENERGY_FLOOR,
    )
    require_close(
        "independent lower cold change",
        checked_number(row, "lower_cold_energy_change_j_m2"),
        result["lower_change"],
        ENERGY_FLOOR,
    )
    require_close("independent melt", producer_melt, result["melt"], MASS_FLOOR)
    require_close(
        "producer raw complete carrier",
        checked_number(row, "legacy_sequential_complete_j_m2"),
        result["complete_raw"],
        ENERGY_FLOOR,
    )
    require_close(
        "producer energy closure",
        checked_number(row, "energy_closure_residual_j_m2"),
        0.0,
        ENERGY_FLOOR,
    )
    total_before = checked_number(row, "total_ice_mass_before_kg_m2")
    total_after = checked_number(row, "total_ice_mass_after_kg_m2")
    expected_after = total_before - result["melt"] - result["sublimation"] + result["deposition"]
    require_close("independent total-mass endpoint", total_after, expected_after, MASS_FLOOR)
    if physical_failures:
        raise PhysicalPlausibilityFailure("; ".join(physical_failures), result)
    return result


def validate_nonmutating_transfer(row: dict[str, Any]) -> None:
    for field in ("melt_kg_m2", "sublimation_kg_m2", "deposition_kg_m2"):
        if field not in row:
            raise RuntimeError(f"S/F tuple is missing required N/A field {field}")
        if row[field] is not None:
            raise RuntimeError(f"S/F actual transfer must be N/A: {field}")


def reject_nonfinite_q_state(row: dict[str, Any]) -> None:
    producer = [
        checked_number(row, "deposition_kg_m2"),
        checked_number(row, "sublimation_kg_m2"),
        checked_number(row, "melt_kg_m2"),
    ]
    if any(value < 0.0 for value in producer):
        raise RuntimeError("producer bounded transfer or melt is negative")
    if producer[0] > MASS_FLOOR and producer[1] > MASS_FLOOR:
        raise RuntimeError("producer deposition and sublimation are simultaneous")
    after_applicable = row.get("after_surface_applicable")
    if not isinstance(after_applicable, bool):
        raise RuntimeError("after_surface_applicable is not boolean")
    after_fields = {
        "active_ice_mass_after_kg_m2",
        "active_depth_after_m",
        "active_density_after_kg_m3",
        "active_cold_after_j_m2",
        "surface_temperature_after_c",
    }
    for field in (
        "active_ice_mass_before_kg_m2",
        "active_ice_mass_after_kg_m2",
        "total_ice_mass_before_kg_m2",
        "total_ice_mass_after_kg_m2",
        "active_depth_before_m",
        "active_depth_after_m",
        "active_density_before_kg_m3",
        "active_density_after_kg_m3",
        "active_cold_before_j_m2",
        "active_cold_after_j_m2",
        "total_cold_before_j_m2",
        "total_cold_after_j_m2",
        "surface_temperature_before_c",
        "surface_temperature_after_c",
    ):
        value = row.get(field)
        if field in after_fields:
            if after_applicable and value is None:
                raise RuntimeError(f"{field} is unauthorized N/A on resolved surface")
            if not after_applicable:
                if value is not None:
                    raise RuntimeError(f"{field} must be N/A after terminal surface")
                continue
        if isinstance(value, bool) or not isinstance(value, (int, float)):
            raise RuntimeError(f"{field} is not numeric or an authorized terminal N/A")
        if not math.isfinite(float(value)):
            raise RuntimeError(f"{field} is nonfinite invalid evidence")


def validate_trace_identity(row: dict[str, Any], expected_index: int) -> None:
    if row.get("day_index") != expected_index or row.get("lane_index") != 0:
        raise RuntimeError("trace day/lane identity mismatch")


def validate_joined_identity(
    paired_row: dict[str, Any],
    sequential_row: dict[str, Any],
    paired_tuples: list[dict[str, Any]],
    sequential_tuples: list[dict[str, Any]],
    predecessor: Any,
) -> None:
    for field in (
        "stage3_evaluation_source_fingerprint_fnv1a64",
        "stage3_evaluation_forcing_fingerprint_fnv1a64",
        "stage3_evaluation_geometry_fingerprint_fnv1a64",
    ):
        if paired_row.get(field) != sequential_row.get(field):
            raise RuntimeError(f"joined cross-lane {field} mismatch")
    paired_non_formulation = paired_row.get(
        "stage3_evaluation_non_formulation_fingerprint_fnv1a64"
    )
    sequential_non_formulation = sequential_row.get(
        "stage3_evaluation_non_formulation_fingerprint_fnv1a64"
    )
    zero = "0000000000000000"
    paired_zero = paired_non_formulation == zero
    sequential_zero = sequential_non_formulation == zero
    if paired_non_formulation is None or sequential_non_formulation is None:
        raise RuntimeError("joined non-formulation fingerprint is missing")
    if paired_zero != sequential_zero:
        raise RuntimeError("joined non-formulation sentinel applicability mismatch")
    if not paired_zero and paired_non_formulation == sequential_non_formulation:
        raise RuntimeError("joined operator-specific non-formulation fingerprint alias")
    if not paired_tuples or not sequential_tuples:
        return
    first_s, first_q = paired_tuples[0], sequential_tuples[0]
    for field in predecessor.FIXED_REFERENCE_FIELDS:
        left, right = first_s.get(field), first_q.get(field)
        if isinstance(left, float) or isinstance(right, float):
            predecessor.require_same_bits(f"joined fixed {field}", left, right)
        elif left != right:
            raise RuntimeError(f"joined fixed {field} mismatch")


def construct_frozen_active(
    same_state: dict[str, Any], first_q: dict[str, Any], predecessor: Any
) -> dict[str, float]:
    if same_state.get("projection_id") != "whole_column_immutable":
        raise RuntimeError("F source is not the S whole-column projection")
    if first_q.get("projection_id") != "aligned_active_dynamic":
        raise RuntimeError("F state is not the first-Q active projection")
    return predecessor.frozen_active_flux(same_state, first_q)


def integrate_optional_tuple_field(
    rows: list[dict[str, Any]], seconds: float, field: str
) -> tuple[float, float]:
    remaining = seconds
    total = 0.0
    support = 0.0
    for row in rows:
        duration = min(checked_number(row, "duration_seconds"), remaining)
        value = row.get(field)
        if value is not None:
            if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(float(value)):
                raise RuntimeError(f"{field} is not finite numeric or N/A")
            total += float(value) * duration
            support += duration
        remaining -= duration
        if remaining <= 0.0:
            break
    if remaining > MASS_FLOOR:
        raise RuntimeError(f"{field} prefix exceeds tuple support")
    return total, support


def reduce_joined_hour(
    s_rows: list[dict[str, Any]],
    q_rows: list[dict[str, Any]],
    frozen: dict[str, float] | None,
    frozen_state: dict[str, Any] | None,
    predecessor: Any,
) -> tuple[dict[str, float], Counter[str]]:
    values: defaultdict[str, float] = defaultdict(float)
    counts: Counter[str] = Counter()
    s_support = sum(checked_number(row, "duration_seconds") for row in s_rows)
    q_support = sum(checked_number(row, "duration_seconds") for row in q_rows)
    for rows, support, prefix in ((s_rows, s_support, "S"), (q_rows, q_support, "Q")):
        for term in TERMS:
            values[f"{prefix}_all_{term}_j_m2"] = (
                predecessor.integrate_reconstructed_prefix(rows, support, term)
                if rows
                else 0.0
            )
    values["all_evaluated_magnitude_j_m2"] = sum(
        abs(values[f"{prefix}_all_{term}_j_m2"])
        for prefix in ("S", "Q")
        for term in TERMS
    )
    if not s_rows or not q_rows or frozen is None or frozen_state is None:
        counts["unmatched_hour_count" if bool(s_rows) != bool(q_rows) else "zero_support_hour_count"] += 1
        values["omitted_magnitude_j_m2"] = values["all_evaluated_magnitude_j_m2"]
        return dict(values), counts
    common = min(s_support, q_support, 3_600.0)
    values["common_support_seconds"] = common
    if common < 3_600.0:
        counts["partial_support_hour_count"] += 1
    for term in TERMS:
        s_value = predecessor.integrate_reconstructed_prefix(s_rows, common, term)
        q_value = predecessor.integrate_reconstructed_prefix(q_rows, common, term)
        f_value = checked_number(frozen, term) * common
        values[f"S_{term}_j_m2"] = s_value
        values[f"F_{term}_j_m2"] = f_value
        values[f"Q_{term}_j_m2"] = q_value
        values[f"delta_evolution_{term}_j_m2"] = q_value - f_value
        if term == "shortwave":
            require_close("S/F shortwave invariance", f_value, s_value, ENERGY_FLOOR)
            require_close("S/Q shortwave invariance", q_value, s_value, ENERGY_FLOOR)
    values["S_j_m2"] = sum(values[f"S_{term}_j_m2"] for term in TERMS)
    values["F_j_m2"] = sum(values[f"F_{term}_j_m2"] for term in TERMS)
    values["Q_j_m2"] = sum(values[f"Q_{term}_j_m2"] for term in TERMS)
    values["delta_evolution_j_m2"] = values["Q_j_m2"] - values["F_j_m2"]
    s_vapor = predecessor.integrate_reconstructed_prefix(s_rows, common, "vapor_mass_flux")
    q_vapor = predecessor.integrate_reconstructed_prefix(q_rows, common, "vapor_mass_flux")
    f_vapor = checked_number(frozen, "vapor_mass_flux") * common
    values["S_raw_vapor_kg_m2"] = s_vapor
    values["F_raw_vapor_kg_m2"] = f_vapor
    values["Q_raw_vapor_kg_m2"] = q_vapor
    values["delta_evolution_raw_vapor_kg_m2"] = q_vapor - f_vapor
    for field, label in (
        ("surface_temperature_before_c", "surface_temperature_c"),
        ("active_cold_before_j_m2", "active_cold_j_m2"),
        ("active_ice_mass_before_kg_m2", "active_ice_kg_m2"),
        ("active_depth_before_m", "active_depth_m"),
        ("active_density_before_kg_m3", "active_density_kg_m3"),
        ("latent_exchange_velocity_m_s", "latent_exchange_velocity_m_s"),
        ("sensible_exchange_velocity_m_s", "sensible_exchange_velocity_m_s"),
    ):
        q_total, q_field_support = integrate_optional_tuple_field(q_rows, common, field)
        frozen_value = frozen_state.get(field)
        if frozen_value is None:
            f_total, f_field_support = 0.0, 0.0
        else:
            if isinstance(frozen_value, bool) or not isinstance(frozen_value, (int, float)):
                raise RuntimeError(f"frozen {field} is not numeric or N/A")
            f_total, f_field_support = float(frozen_value) * common, common
        values[f"Q_{label}_duration_sum"] = q_total
        values[f"F_{label}_duration_sum"] = f_total
        values[f"Q_{label}_support_seconds"] = q_field_support
        values[f"F_{label}_support_seconds"] = f_field_support
        if q_field_support == common and f_field_support == common:
            values[f"delta_evolution_{label}_duration_sum"] = q_total - f_total
    q_gradient = []
    for row in q_rows:
        copy = dict(row)
        surface = copy.get("specific_humidity_surface_kg_kg")
        air = copy.get("specific_humidity_air_kg_kg")
        copy["humidity_gradient_kg_kg"] = (
            None if surface is None or air is None else float(surface) - float(air)
        )
        q_gradient.append(copy)
    q_total, q_gradient_support = integrate_optional_tuple_field(
        q_gradient, common, "humidity_gradient_kg_kg"
    )
    f_surface = frozen_state.get("specific_humidity_surface_kg_kg")
    f_air = frozen_state.get("specific_humidity_air_kg_kg")
    if f_surface is None or f_air is None:
        f_total, f_gradient_support = 0.0, 0.0
    else:
        f_total, f_gradient_support = (float(f_surface) - float(f_air)) * common, common
    values["Q_humidity_gradient_kg_kg_duration_sum"] = q_total
    values["F_humidity_gradient_kg_kg_duration_sum"] = f_total
    values["Q_humidity_gradient_support_seconds"] = q_gradient_support
    values["F_humidity_gradient_support_seconds"] = f_gradient_support
    if q_gradient_support == common and f_gradient_support == common:
        values["delta_evolution_humidity_gradient_kg_kg_duration_sum"] = q_total - f_total
    values["omitted_magnitude_j_m2"] = sum(
        abs(values[f"{prefix}_all_{term}_j_m2"] - values[f"{prefix}_{term}_j_m2"])
        for prefix in ("S", "Q")
        for term in TERMS
    )
    return dict(values), counts


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def require_sha256(path: Path, expected: str, label: str) -> None:
    if not re.fullmatch(r"[0-9a-f]{64}", expected) or sha256(path) != expected:
        raise RuntimeError(f"{label} custody mismatch")


def output_path(lane: dict[str, Any], suffix: str) -> Path:
    matches = [REPO / item["path"] for name, item in lane["outputs"].items() if name.endswith(suffix)]
    if len(matches) != 1:
        raise RuntimeError(f"expected exactly one {suffix} output")
    return matches[0]


def eligible_years(protocol: dict[str, Any], site: str) -> set[int]:
    text = protocol["cohort"]["eligible_water_years"][site]
    match = re.fullmatch(r"(\d{4})-(\d{4}) inclusive; (\d+)", text)
    if match is None:
        raise RuntimeError(f"invalid eligible-water-year freeze for {site}")
    first, last, count = map(int, match.groups())
    years = set(range(first, last + 1))
    if len(years) != count:
        raise RuntimeError(f"eligible-water-year count mismatch for {site}")
    return years


def water_year(stamp: dt.date) -> int:
    return stamp.year + (1 if stamp.month >= 10 else 0)


def in_window(stamp: dt.date, peak: dt.date, year: int) -> bool:
    return dt.date(year - 1, 10, 1) <= stamp <= peak


def summarize_annual(site: str, year: int, sums: dict[str, float], counts: Counter[str]) -> dict[str, Any]:
    for key in (
        "tuple_count",
        "capacity_truncated_tuple_count",
        "unmatched_hour_count",
        "zero_support_hour_count",
        "partial_support_hour_count",
        "active_state_plausibility_failure_tuple_count",
        "resolved_after_state_tuple_count",
        "terminal_after_state_na_tuple_count",
    ):
        counts.setdefault(key, 0)
    return {
        "site": site,
        "water_year": year,
        **{key: sums[key] for key in sorted(sums)},
        **{key: counts[key] for key in sorted(counts)},
    }


def support_materiality(annual: list[dict[str, Any]]) -> dict[str, Any]:
    ratios: list[float | None] = []
    for row in annual:
        all_magnitude = float(row.get("all_evaluated_magnitude_j_m2", 0.0))
        ratios.append(
            None
            if all_magnitude == 0.0
            else float(row.get("omitted_magnitude_j_m2", 0.0)) / all_magnitude
        )
    failing = [
        row["water_year"]
        for row, ratio in zip(annual, ratios, strict=True)
        if ratio is not None and ratio > 0.05
    ]
    not_evaluable = [
        row["water_year"]
        for row, ratio in zip(annual, ratios, strict=True)
        if ratio is None
    ]
    evaluable = [ratio for ratio in ratios if ratio is not None]
    return {
        "support_omission_ratio_median": statistics.median(evaluable) if evaluable else None,
        "support_omission_ratio_maximum": max(evaluable) if evaluable else None,
        "support_materiality_pass": not failing and not not_evaluable,
        "support_materiality_failure_count": len(failing),
        "support_materiality_failing_water_years": failing,
        "support_materiality_not_evaluable_water_years": not_evaluable,
    }


def analyze_site(
    site: str,
    receipt: dict[str, Any],
    protocol: dict[str, Any],
    predecessor: Any,
    carrier: Any,
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    paired_path = output_path(receipt["lanes"][site]["paired"], ".snow.jsonl")
    sequential_path = output_path(receipt["lanes"][site]["sequential"], ".snow.jsonl")
    climate_path = REPO / receipt["lanes"][site]["paired"]["runfile_consumer"]["climate_path"]
    dates = carrier.climate_dates(climate_path)
    observation = REPO / "target/snow_stage3_operator_reconciliation_v3/inputs/observations" / f"{site}.csv"
    expected_climate_hash = protocol["forcing_custody"]["climate_sha256"][site]
    receipt_climate_hash = receipt["lanes"][site]["paired"]["runfile_consumer"]["climate_sha256"]
    if receipt_climate_hash != expected_climate_hash:
        raise RuntimeError(f"climate custody mismatch for {site}")
    require_sha256(climate_path, expected_climate_hash, f"climate for {site}")
    expected_observation_hash = protocol["cohort"]["observation_sha256"][site]
    require_sha256(observation, expected_observation_hash, f"observation for {site}")
    peaks, _ = carrier.observed_peaks(observation)
    years = eligible_years(protocol, site)
    annual_sums: dict[int, defaultdict[str, float]] = {
        year: defaultdict(float) for year in years
    }
    annual_counts: dict[int, Counter[str]] = {year: Counter() for year in years}
    stability: Counter[str] = Counter()
    with paired_path.open(encoding="utf-8") as paired, sequential_path.open(encoding="utf-8") as sequential:
        for index, (paired_line, sequential_line) in enumerate(zip(paired, sequential, strict=True)):
            if index >= len(dates):
                raise RuntimeError(f"trace has more rows than climate for {site}")
            stamp = dates[index]
            p_row = json.loads(paired_line)
            q_row = json.loads(sequential_line)
            validate_trace_identity(p_row, index)
            validate_trace_identity(q_row, index)
            p_tuples = predecessor.validate_v6_row(p_row, "same_state_paired_carrier_v1", site)
            companion = q_row.get("stage3_operator_reconciliation")
            raw_q_tuples = companion.get("tuples") if isinstance(companion, dict) else None
            if isinstance(raw_q_tuples, list) and all(isinstance(item, dict) for item in raw_q_tuples):
                for item in raw_q_tuples:
                    reject_nonfinite_q_state(item)
            q_tuples = predecessor.validate_v6_row(
                q_row, "sequential_resolved_shadow_v1", site
            )
            validate_joined_identity(
                p_row, q_row, p_tuples, q_tuples, predecessor
            )
            for tuple_row in p_tuples:
                validate_nonmutating_transfer(tuple_row)
            validated_q: list[tuple[dict[str, Any], dict[str, float] | None, str | None]] = []
            for tuple_row in q_tuples:
                try:
                    validated_q.append((tuple_row, validate_q_tuple(tuple_row), None))
                except PhysicalPlausibilityFailure as error:
                    validated_q.append((tuple_row, error.reconstructed, str(error)))
            year = water_year(stamp)
            peak = peaks.get(year)
            if year not in years or peak is None or not in_window(stamp, peak[0], year):
                continue
            sums = annual_sums[year]
            counts = annual_counts[year]
            s_hours = predecessor.tuples_by_hour(p_tuples)
            q_hours = predecessor.tuples_by_hour(q_tuples)
            first_q = next((row for hour in q_hours for row in hour), None)
            for hour in range(24):
                frozen = (
                    construct_frozen_active(s_hours[hour][0], first_q, predecessor)
                    if s_hours[hour] and first_q is not None
                    else None
                )
                joined, joined_counts = reduce_joined_hour(
                    s_hours[hour], q_hours[hour], frozen, first_q, predecessor
                )
                for key, value in joined.items():
                    sums[key] += value
                counts.update(joined_counts)
            for tuple_row, reconstructed, physical_failure in validated_q:
                duration = checked_number(tuple_row, "duration_seconds")
                counts["tuple_count"] += 1
                sums["support_seconds"] += duration
                if physical_failure is not None:
                    counts["active_state_plausibility_failure_tuple_count"] += 1
                    if reconstructed is None:
                        continue
                assert reconstructed is not None
                for name in ("deposition", "sublimation", "truncated_mass", "melt"):
                    sums[f"bounded_{name}_kg_m2" if name != "melt" else "melt_kg_m2"] += reconstructed[name]
                for name in ("latent_raw", "latent_bounded", "latent_truncation", "external_raw", "unallocated"):
                    sums[f"{name}_j_m2"] += reconstructed[name]
                sums["raw_vapor_opportunity_kg_m2"] += checked_number(tuple_row, "vapor_mass_exchange_kg_m2")
                sums["wind_speed_duration_m"] += checked_number(tuple_row, "wind_speed_m_s") * duration
                sums["surface_temperature_duration_c_s"] += checked_number(tuple_row, "surface_temperature_before_c") * duration
                sums["total_ice_endpoint_change_kg_m2"] += (
                    checked_number(tuple_row, "total_ice_mass_after_kg_m2")
                    - checked_number(tuple_row, "total_ice_mass_before_kg_m2")
                )
                sums["total_cold_endpoint_change_j_m2"] += (
                    checked_number(tuple_row, "total_cold_after_j_m2")
                    - checked_number(tuple_row, "total_cold_before_j_m2")
                )
                if tuple_row.get("after_surface_applicable") is True:
                    counts["resolved_after_state_tuple_count"] += 1
                    sums["surface_temperature_after_duration_c_s"] += (
                        checked_number(tuple_row, "surface_temperature_after_c") * duration
                    )
                    sums["active_cold_after_duration_j_s_m2"] += (
                        checked_number(tuple_row, "active_cold_after_j_m2") * duration
                    )
                    sums["active_depth_after_duration_m_s"] += (
                        checked_number(tuple_row, "active_depth_after_m") * duration
                    )
                    sums["active_ice_after_duration_kg_s_m2"] += (
                        checked_number(tuple_row, "active_ice_mass_after_kg_m2") * duration
                    )
                else:
                    counts["terminal_after_state_na_tuple_count"] += 1
                if reconstructed["capacity_truncated"]:
                    counts["capacity_truncated_tuple_count"] += 1
                stability[str(tuple_row["stability_class"])] += 1
    if len(dates) != index + 1:
        raise RuntimeError(f"trace/climate row-count mismatch for {site}")
    annual = [summarize_annual(site, year, annual_sums[year], annual_counts[year]) for year in sorted(years)]
    if any(row["tuple_count"] == 0 for row in annual):
        raise RuntimeError(f"eligible water year without evaluated tuples for {site}")
    medians = {
        key: statistics.median(float(row[key]) for row in annual)
        for key in annual[0]
        if key not in {"site", "water_year"}
    }
    truncated = sum(int(row.get("capacity_truncated_tuple_count", 0)) for row in annual)
    physical_failures = sum(
        int(row.get("active_state_plausibility_failure_tuple_count", 0))
        for row in annual
    )
    support = support_materiality(annual)
    return annual, {
        "site": site,
        "role": "CANONICAL_SCREEN" if site in CANONICAL_SITES else "DEVELOPMENT_ONLY_NON_DECISIVE_DIAGNOSTIC",
        "eligible_water_year_count": len(annual),
        "water_year_medians": medians,
        "capacity_truncated_tuple_count": truncated,
        "active_state_plausibility_failure_tuple_count": physical_failures,
        **support,
        "right_censored_water_year": protocol["cohort"]["right_censored_water_year"],
        "stability_class_counts": dict(sorted(stability.items())),
        "wind_exposure": "UNKNOWN",
        "stability_geometry_equation": "NOT_EVALUABLE",
        "physical_magnitude_envelopes": "NOT_EVALUABLE",
    }


def verify_retained(receipt_path: Path, results_path: Path, protocol: dict[str, Any], receipt: dict[str, Any]) -> None:
    if sha256(receipt_path) != protocol["source"]["retained_execution_receipt_sha256"]:
        raise RuntimeError("retained execution receipt hash mismatch")
    if sha256(results_path) != protocol["source"]["retained_operator_result_sha256"]:
        raise RuntimeError("retained operator result hash mismatch")
    if receipt.get("binary_sha256") != protocol["source"]["binary_sha256"]:
        raise RuntimeError("receipt binary hash mismatch")
    frozen_inventory = {(item["site"], item["lane"]): item for item in protocol["source"]["trace_inventory"]}
    for site, lanes in receipt["lanes"].items():
        for lane in ("paired", "sequential"):
            path = output_path(lanes[lane], ".snow.jsonl")
            expected = frozen_inventory[(site, lane)]
            if path.stat().st_size != expected["size_bytes"] or sha256(path) != expected["sha256"]:
                raise RuntimeError(f"retained trace custody mismatch for {site}/{lane}")


def decision_classes(site_summaries: Iterable[dict[str, Any]]) -> list[str]:
    summaries = [row for row in site_summaries if row.get("role") == "CANONICAL_SCREEN"]
    if {row.get("site") for row in summaries} != CANONICAL_SITES:
        raise RuntimeError("canonical decision cohort is incomplete")
    classes = ["WIND_FORCING_EXPOSURE_UNRESOLVED"]
    if any(row["capacity_truncated_tuple_count"] > 0 for row in summaries):
        classes.append("VAPOR_OPPORTUNITY_TRANSFER_MISMATCH")
    if any(row.get("active_state_plausibility_failure_tuple_count", 0) > 0 for row in summaries):
        classes.append("ACTIVE_STATE_EVOLUTION_PLAUSIBILITY_FAIL")
    classes.append("MULTIFACTOR_OR_INCONCLUSIVE")
    return classes


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, allow_nan=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def validate_execution_preconditions(
    verify_retained_requested: bool, output_exists: bool, tracked_status: str
) -> None:
    if not verify_retained_requested:
        raise RuntimeError("--verify-retained is mandatory for result-bearing analysis")
    if output_exists:
        raise RuntimeError("refusing to overwrite immutable output")
    if tracked_status:
        raise RuntimeError("result-bearing analysis requires a clean tracked worktree")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-retained", action="store_true")
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--results", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    tracked_status = subprocess.run(
        ["git", "status", "--porcelain"],
        cwd=REPO,
        text=True,
        capture_output=True,
        check=True,
    ).stdout
    validate_execution_preconditions(
        args.verify_retained, args.output.exists(), tracked_status
    )
    protocol = json.loads(PROTOCOL.read_text(encoding="utf-8"))
    receipt = json.loads(args.receipt.read_text(encoding="utf-8"))
    verify_retained(args.receipt, args.results, protocol, receipt)
    predecessor = load_module("stage3_plausibility_predecessor", PREDECESSOR)
    carrier = load_module("stage3_plausibility_carrier", CARRIER)
    annual: list[dict[str, Any]] = []
    summaries: list[dict[str, Any]] = []
    for site in sorted(protocol["cohort"]["eligible_water_years"]):
        site_annual, summary = analyze_site(site, receipt, protocol, predecessor, carrier)
        annual.extend(site_annual)
        summaries.append(summary)
    classes = decision_classes(summaries)
    result = {
        "schema": "snow-stage3-evolving-carrier-plausibility-result-v1",
        "analysis_head": subprocess.run(["git", "rev-parse", "HEAD"], cwd=REPO, text=True, capture_output=True, check=True).stdout.strip(),
        "retained_execution_head": receipt["execution_head"],
        "decision_classes": classes,
        "persistence_disposition": "HOLD",
        "coe_authority": "UNCHANGED",
        "production_changes": "NONE",
        "site_summaries": summaries,
        "annual_samples": annual,
    }
    write_json(args.output / "results/evolving-carrier-plausibility-results.json", result)
    write_json(args.output / "execution-receipt.json", {
        "analysis_head": result["analysis_head"],
        "protocol_sha256": sha256(PROTOCOL),
        "input_receipt_sha256": sha256(args.receipt),
        "input_results_sha256": sha256(args.results),
        "command": sys.argv,
        "output_sha256": sha256(args.output / "results/evolving-carrier-plausibility-results.json"),
    })
    print(json.dumps({"decision_classes": classes, "persistence_disposition": "HOLD"}, sort_keys=True))


if __name__ == "__main__":
    main()
