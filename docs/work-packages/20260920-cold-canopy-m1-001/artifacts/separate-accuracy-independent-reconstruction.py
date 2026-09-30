#!/usr/bin/env python3
"""Independent residual and conservation reconstruction for M1 separate accuracy.

This diagnostic deliberately consumes only serialized primitive operands.  The
candidate residual and normalizer arrays are comparison targets and never inputs
to reconstruction.  A missing, non-finite, contradictory, or malformed operand
produces an UNRESOLVED record instead of a guessed value.

Accepted input is a JSON object containing ``supports`` or ``records``, a JSON
array of support records, one support record, or newline-delimited support
records.  A record may wrap the Rust payload in ``primitive``; metadata is read
from the payload first and then the wrapper.
"""

from __future__ import annotations

import argparse
import copy
import json
import math
import pathlib
import struct
import sys
from collections import OrderedDict
from typing import Any


SCHEMA = "openwepp.m1-separate-accuracy-independent-reconstruction.v2"
SOURCE_AUTHORITY = "COLD-CANOPY-M1-SEPARATE-ACCURACY-01@b30225ce"
R_DRY = 287.05
CP_AIR = 1004.64
TF = 273.15
CW = 4218.0
LF = 333_700.0
LV = 2_501_000.0
CPV = 1849.0


class Unresolved(ValueError):
    """A required independent operand is unavailable or inadmissible."""


def _finite(value: Any, path: str) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise Unresolved(f"{path}: expected JSON number")
    result = float(value)
    if not math.isfinite(result):
        raise Unresolved(f"{path}: expected finite binary64 value")
    return result


def _mapping(value: Any, path: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise Unresolved(f"{path}: expected object")
    return value


def _array(value: Any, size: int, path: str) -> list[Any]:
    if not isinstance(value, list) or len(value) != size:
        raise Unresolved(f"{path}: expected array of length {size}")
    return value


def _field(obj: dict[str, Any], name: str, path: str) -> Any:
    if name not in obj:
        raise Unresolved(f"{path}.{name}: missing required operand")
    return obj[name]


def _numbers(obj: dict[str, Any], name: str, size: int, path: str) -> list[float]:
    values = _array(_field(obj, name, path), size, f"{path}.{name}")
    return [_finite(value, f"{path}.{name}[{i}]") for i, value in enumerate(values)]


def _matrix_numbers(
    obj: dict[str, Any], name: str, rows: int, columns: int, path: str
) -> list[list[float]]:
    outer = _array(_field(obj, name, path), rows, f"{path}.{name}")
    return [
        [
            _finite(value, f"{path}.{name}[{i}][{j}]")
            for j, value in enumerate(_array(row, columns, f"{path}.{name}[{i}]"))
        ]
        for i, row in enumerate(outer)
    ]


def _bits(value: float) -> str:
    return f"0x{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"


def _comparison(actual: float, expected: float) -> dict[str, Any]:
    return {
        "reconstructed": actual,
        "reconstructed_bits": _bits(actual),
        "candidate": expected,
        "candidate_bits": _bits(expected),
        "exact": _bits(actual) == _bits(expected),
        "absolute_difference": abs(actual - expected),
    }


def _routing_comparison(actual: float, expected: float) -> dict[str, Any]:
    comparison = _comparison(actual, expected)
    tolerance = 16.0 * math.ulp(max(abs(actual), abs(expected), 1.0))
    comparison["arithmetic_tolerance"] = tolerance
    comparison["within_arithmetic_tolerance"] = (
        comparison["absolute_difference"] <= tolerance
    )
    return comparison


def _same_bits(left: float, right: float) -> bool:
    return _bits(left) == _bits(right)


def _profile_name(record: dict[str, Any], primitive: dict[str, Any]) -> str:
    value = primitive.get("profile", record.get("profile"))
    if not isinstance(value, str) or value not in {"P0", "P1", "P2", "P3"}:
        raise Unresolved("profile: expected one of P0, P1, P2, P3")
    return value


def _identity_metadata(record: dict[str, Any], primitive: dict[str, Any]) -> dict[str, Any]:
    case_id = primitive.get("case_id", record.get("case_id"))
    support_id = primitive.get("support_id", record.get("support_id"))
    if not isinstance(case_id, str) or not case_id:
        raise Unresolved("case_id: expected nonempty string")
    if isinstance(support_id, bool) or not isinstance(support_id, int) or support_id < 0:
        raise Unresolved("support_id: expected nonnegative integer")
    metadata: dict[str, Any] = {
        "case_id": case_id,
        "support_id": support_id,
        "profile": _profile_name(record, primitive),
    }
    stability_variant = primitive.get(
        "stability_variant", record.get("stability_variant", "ordinary")
    )
    if stability_variant not in {"ordinary", "strict_reference"}:
        raise Unresolved(
            "stability_variant: expected ordinary or strict_reference"
        )
    if stability_variant == "strict_reference" and metadata["profile"] != "P0":
        raise Unresolved("strict_reference stability_variant requires profile P0")
    metadata["stability_variant"] = stability_variant
    for name in ("formulation", "run_id", "cadence_seconds"):
        if name in primitive:
            metadata[name] = primitive[name]
        elif name in record:
            metadata[name] = record[name]
    return metadata


def _normalizers(
    profile: str,
    scales: list[float],
    reservoirs: list[dict[str, float]],
    stability_variant: str,
) -> list[float]:
    result = [1.0e-9] * 21
    if stability_variant == "strict_reference":
        for row in (0, 1, 2, 6, 7, 8, 12):
            result[row] = 1.0e-7 + 1.0e-11 * max(scales[row], 1.0)
        result[13] = 1.0e-13 + 1.0e-10 * max(scales[13], 0.0)
        for occupancy in range(2):
            base = 6 * occupancy
            r = reservoirs[occupancy]
            mass_scale = max(abs(r["beginning_mass_kg_m2"]), scales[base + 3], 1.0e-9)
            heat_scale = max(abs(r["beginning_enthalpy_j_m2"]), scales[base + 4], 1.0)
            result[base + 3] = min(1.0e-10, 1.0e-9 * mass_scale)
            result[base + 4] = min(1.0e-7, 1.0e-9 * heat_scale)
            result[base + 5] = min(
                1.0e-10, 1.0e-9 * max(r["liquid_capacity_kg_m2"], 1.0e-9)
            )
        return result
    if profile == "P0":
        for row in (0, 1, 2, 6, 7, 8, 12):
            result[row] = 1.0e-6 + 1.0e-10 * max(scales[row], 1.0)
        result[13] = 1.0e-12 + 1.0e-9 * max(scales[13], 0.0)
        for occupancy in range(2):
            base = 6 * occupancy
            r = reservoirs[occupancy]
            mass_scale = max(abs(r["beginning_mass_kg_m2"]), scales[base + 3], 1.0e-9)
            heat_scale = max(abs(r["beginning_enthalpy_j_m2"]), scales[base + 4], 1.0)
            result[base + 3] = min(1.0e-9, 1.0e-8 * mass_scale)
            result[base + 4] = min(1.0e-6, 1.0e-8 * heat_scale)
            result[base + 5] = min(
                1.0e-9, 1.0e-8 * max(r["liquid_capacity_kg_m2"], 1.0e-9)
            )
        return result

    policy = {
        "P1": (1.0e-8, 1.0e-6, 1.0e-4, 1.0e-8, 1.0e-10, 1.0e-7),
        "P2": (1.0e-6, 1.0e-5, 1.0e-2, 1.0e-6, 1.0e-8, 1.0e-5),
        "P3": (1.0e-4, 1.0e-3, 1.0, 1.0e-4, 1.0e-6, 1.0e-3),
    }[profile]
    mass_abs, relative, energy_abs, energy_rel, water_abs, water_rel = policy
    for row in (0, 1, 2, 6, 7, 8, 12):
        result[row] = energy_abs + energy_rel * max(scales[row], 1.0)
    result[13] = water_abs + water_rel * max(scales[13], 0.0)
    for occupancy in range(2):
        base = 6 * occupancy
        r = reservoirs[occupancy]
        result[base + 3] = max(
            mass_abs,
            relative * max(abs(r["beginning_mass_kg_m2"]), scales[base + 3], 1.0e-9),
        )
        result[base + 4] = max(
            LF * mass_abs,
            relative
            * max(abs(r["beginning_enthalpy_j_m2"]), scales[base + 4], 1.0),
        )
        result[base + 5] = max(
            mass_abs, relative * max(r["liquid_capacity_kg_m2"], 1.0e-9)
        )
    return result


def _reservoir(value: Any, index: int, path: str) -> dict[str, float | None]:
    obj = _mapping(value, f"{path}.reservoirs[{index}]")
    names = (
        "beginning_mass_kg_m2",
        "beginning_enthalpy_j_m2",
        "ending_mass_kg_m2",
        "ending_enthalpy_j_m2",
        "dt_s",
        "incident_liquid_kg_m2_s",
        "incident_enthalpy_j_kg",
        "vapor_kg_m2_s",
        "drainage_kg_m2_s",
        "non_vapor_heat_w_m2",
        "diagnosed_liquid_mass_kg_m2",
        "liquid_capacity_kg_m2",
        "wet_fraction",
        "wet_longwave_w_m2",
        "wet_sensible_w_m2",
    )
    result: dict[str, float | None] = {
        name: _finite(_field(obj, name, f"{path}.reservoirs[{index}]"), f"{path}.reservoirs[{index}].{name}")
        for name in names
    }
    wet_temperature = _field(obj, "wet_temperature_k", f"{path}.reservoirs[{index}]")
    result["wet_temperature_k"] = (
        None
        if wet_temperature is None
        else _finite(wet_temperature, f"{path}.reservoirs[{index}].wet_temperature_k")
    )
    return result


def reconstruct_support(record: dict[str, Any], record_index: int) -> dict[str, Any]:
    path = f"records[{record_index}]"
    primitive = _mapping(record.get("primitive", record), f"{path}.primitive")
    metadata = _identity_metadata(record, primitive)
    coordinates = _numbers(primitive, "coordinates", 21, path)
    dt = _finite(_field(primitive, "interval_s", path), f"{path}.interval_s")
    pressure = _finite(_field(primitive, "pressure_pa", path), f"{path}.pressure_pa")
    latent_heat = _finite(_field(primitive, "latent_heat_j_kg", path), f"{path}.latent_heat_j_kg")
    heat_resistance = _finite(
        _field(primitive, "canopy_to_atmosphere_heat_resistance_s_m", path),
        f"{path}.canopy_to_atmosphere_heat_resistance_s_m",
    )
    vapor_resistance = _finite(
        _field(primitive, "canopy_to_atmosphere_vapor_resistance_s_m", path),
        f"{path}.canopy_to_atmosphere_vapor_resistance_s_m",
    )
    prescribed_snow = _finite(
        _field(primitive, "prescribed_snow_temperature_k", path),
        f"{path}.prescribed_snow_temperature_k",
    )
    prescribed_soil = _numbers(primitive, "prescribed_soil_temperature_k", 6, path)
    air_temperature = _finite(_field(primitive, "air_temperature_k", path), f"{path}.air_temperature_k")
    air_humidity = _finite(_field(primitive, "air_humidity_kg_kg", path), f"{path}.air_humidity_kg_kg")
    boundary_sensible = _finite(
        _field(primitive, "boundary_sensible_w_m2", path), f"{path}.boundary_sensible_w_m2"
    )
    boundary_vapor = _finite(
        _field(primitive, "boundary_vapor_kg_m2_s", path), f"{path}.boundary_vapor_kg_m2_s"
    )
    gb_leaf = _numbers(primitive, "gb_leaf_m_s", 2, path)
    gb_stem = _numbers(primitive, "gb_stem_m_s", 2, path)
    areas = _matrix_numbers(primitive, "structural_area_m2_m2", 2, 3, path)
    shortwave = _matrix_numbers(primitive, "absorbed_shortwave_w_m2", 2, 3, path)
    dry_longwave = _matrix_numbers(primitive, "dry_net_longwave_w_m2", 2, 3, path)
    lai = _numbers(primitive, "lai_m2_m2", 2, path)
    sai = _numbers(primitive, "sai_m2_m2", 2, path)
    interception_parameter = _numbers(
        primitive, "liquid_interception_fraction", 2, path
    )
    stemflow_fraction = _numbers(primitive, "stemflow_fraction", 2, path)
    top_rain = _finite(
        _field(primitive, "top_rain_kg_m2_s", path), f"{path}.top_rain_kg_m2_s"
    )
    top_rain_h = _finite(
        _field(primitive, "top_rain_specific_enthalpy_j_kg", path),
        f"{path}.top_rain_specific_enthalpy_j_kg",
    )
    leaf_vapor = _matrix_numbers(primitive, "leaf_vapor_kg_m2_s", 2, 2, path)
    wet_vapor = _numbers(primitive, "wet_vapor_kg_m2_s", 2, path)
    wet_sensible = _numbers(primitive, "wet_sensible_w_m2", 2, path)
    wet_longwave = _numbers(primitive, "wet_longwave_w_m2", 2, path)
    reservoir_values = _array(_field(primitive, "reservoirs", path), 2, f"{path}.reservoirs")
    reservoirs = [_reservoir(value, i, path) for i, value in enumerate(reservoir_values)]

    if dt < 60.0 or pressure <= 0.0 or heat_resistance <= 0.0 or vapor_resistance <= 0.0:
        raise Unresolved(f"{path}: invalid dt, pressure, or canopy resistance domain")
    if any(value < 0.0 for row in areas for value in row):
        raise Unresolved(f"{path}.structural_area_m2_m2: negative area")
    if top_rain < 0.0 or any(value < 0.0 for value in lai + sai):
        raise Unresolved(f"{path}: negative rain, LAI, or SAI routing operand")
    if any(not 0.0 <= value <= 1.0 for value in interception_parameter + stemflow_fraction):
        raise Unresolved(f"{path}: interception or stemflow fraction outside [0,1]")
    tcan = coordinates[12]
    qcan = coordinates[13]
    rho = pressure / (R_DRY * tcan)
    if not math.isfinite(rho) or rho <= 0.0:
        raise Unresolved(f"{path}: reconstructed air density is nonpositive/nonfinite")

    consistency: list[dict[str, Any]] = []
    residuals = [0.0] * 21
    scales = [0.0] * 21
    canopy_h = 0.0
    canopy_e = 0.0
    support_ledger: list[dict[str, Any]] = []
    for occupancy in range(2):
        base = 6 * occupancy
        r = reservoirs[occupancy]
        wet_fraction = float(r["wet_fraction"])
        if not 0.0 <= wet_fraction <= 1.0:
            raise Unresolved(f"{path}.reservoirs[{occupancy}].wet_fraction: outside [0,1]")
        if float(r["drainage_kg_m2_s"]) < 0.0:
            raise Unresolved(f"{path}.reservoirs[{occupancy}].drainage_kg_m2_s: negative")
        if float(r["diagnosed_liquid_mass_kg_m2"]) < 0.0 or float(r["liquid_capacity_kg_m2"]) <= 0.0:
            raise Unresolved(f"{path}.reservoirs[{occupancy}]: invalid liquid mass/capacity")

        checks = {
            "dt_matches_interval": (float(r["dt_s"]), dt),
            "ending_mass_matches_coordinate": (float(r["ending_mass_kg_m2"]), coordinates[base + 3]),
            "ending_enthalpy_matches_coordinate": (float(r["ending_enthalpy_j_m2"]), coordinates[base + 4]),
            "wet_vapor_duplicate_matches": (float(r["vapor_kg_m2_s"]), wet_vapor[occupancy]),
            "wet_sensible_duplicate_matches": (float(r["wet_sensible_w_m2"]), wet_sensible[occupancy]),
            "wet_longwave_duplicate_matches": (float(r["wet_longwave_w_m2"]), wet_longwave[occupancy]),
        }
        for name, (left, right) in checks.items():
            consistency.append(
                {
                    "occupancy": occupancy,
                    "check": name,
                    "left": left,
                    "right": right,
                    "exact": _same_bits(left, right),
                }
            )
            if not _same_bits(left, right):
                raise Unresolved(f"{path}: contradictory duplicate operand {name} at occupancy {occupancy}")

        dry_fraction = 1.0 - wet_fraction
        dry_sensible: list[float] = []
        for component in range(3):
            gb = gb_stem[occupancy] if component == 2 else gb_leaf[occupancy]
            dry_sensible.append(
                rho
                * CP_AIR
                * gb
                * areas[occupancy][component]
                * dry_fraction
                * (coordinates[base + component] - tcan)
            )
        for component in range(3):
            row = base + component
            area = areas[occupancy][component]
            if area == 0.0:
                anchor = tcan if component == 2 else max(tcan, TF)
                residuals[row] = coordinates[row] - anchor
            else:
                latent = 0.0 if component == 2 else latent_heat * leaf_vapor[occupancy][component]
                residuals[row] = (
                    shortwave[occupancy][component] * dry_fraction
                    + dry_longwave[occupancy][component]
                    - dry_sensible[component]
                    - latent
                )
            latent_scale = 0.0 if component == 2 else abs(latent_heat * leaf_vapor[occupancy][component])
            scales[row] = (
                abs(shortwave[occupancy][component] * dry_fraction)
                + abs(dry_longwave[occupancy][component])
                + abs(dry_sensible[component])
                + latent_scale
            )

        non_vapor = (
            wet_fraction * sum(shortwave[occupancy])
            + wet_longwave[occupancy]
            - wet_sensible[occupancy]
        )
        consistency.append(
            {
                "occupancy": occupancy,
                "check": "independent_non_vapor_matches_export",
                "left": non_vapor,
                "right": float(r["non_vapor_heat_w_m2"]),
                "exact": _same_bits(non_vapor, float(r["non_vapor_heat_w_m2"])),
            }
        )
        if not _same_bits(non_vapor, float(r["non_vapor_heat_w_m2"])):
            raise Unresolved(f"{path}: dry/wet primitives contradict exported non-vapor heat at occupancy {occupancy}")

        wet_temperature = tcan if r["wet_temperature_k"] is None else float(r["wet_temperature_k"])
        hv = LV + CPV * (wet_temperature - TF)
        hl = CW * (wet_temperature - TF)
        incident = float(r["incident_liquid_kg_m2_s"])
        incident_h = float(r["incident_enthalpy_j_kg"])
        vapor = wet_vapor[occupancy]
        drainage = float(r["drainage_kg_m2_s"])
        mass_source_rate = incident - vapor - drainage
        enthalpy_source_rate = non_vapor + incident * incident_h - vapor * hv - drainage * hl
        delta_mass = float(r["ending_mass_kg_m2"]) - float(r["beginning_mass_kg_m2"])
        delta_enthalpy = float(r["ending_enthalpy_j_m2"]) - float(r["beginning_enthalpy_j_m2"])
        residuals[base + 3] = delta_mass - dt * mass_source_rate
        residuals[base + 4] = delta_enthalpy - dt * enthalpy_source_rate
        capacity_a = dt * drainage
        capacity_b = float(r["liquid_capacity_kg_m2"]) - float(r["diagnosed_liquid_mass_kg_m2"])
        residuals[base + 5] = capacity_a if capacity_a < capacity_b else capacity_b
        scales[base + 3] = abs(mass_source_rate) * dt
        scales[base + 4] = abs(enthalpy_source_rate) * dt
        canopy_h += wet_sensible[occupancy] + dry_sensible[0] + dry_sensible[1] + dry_sensible[2]
        canopy_e += leaf_vapor[occupancy][0] + leaf_vapor[occupancy][1] + wet_vapor[occupancy]
        support_ledger.append(
            {
                "occupancy": occupancy,
                "storage_delta_mass_kg_m2": delta_mass,
                "source_mass_kg_m2": dt * mass_source_rate,
                "mass_closure_kg_m2": residuals[base + 3],
                "storage_delta_enthalpy_j_m2": delta_enthalpy,
                "source_enthalpy_j_m2": dt * enthalpy_source_rate,
                "enthalpy_closure_j_m2": residuals[base + 4],
                "drainage_transfer_mass_kg_m2": dt * drainage,
                "drainage_transfer_enthalpy_j_m2": dt * drainage * hl,
            }
        )

    reference_heat = rho * CP_AIR * (tcan - air_temperature) / heat_resistance
    reference_vapor = rho * (qcan - air_humidity) / vapor_resistance
    residuals[12] = canopy_h + boundary_sensible - reference_heat
    residuals[13] = canopy_e + boundary_vapor - reference_vapor
    scales[12] = abs(canopy_h) + abs(boundary_sensible) + abs(reference_heat)
    scales[13] = max(abs(canopy_e), abs(boundary_vapor), abs(reference_vapor))
    residuals[14] = coordinates[14] - prescribed_snow
    for soil in range(6):
        residuals[15 + soil] = coordinates[15 + soil] - prescribed_soil[soil]
    if not all(math.isfinite(value) for value in residuals + scales):
        raise Unresolved(f"{path}: nonfinite reconstructed residual or scale")

    normalizers = _normalizers(
        metadata["profile"], scales, reservoirs, metadata["stability_variant"]
    )  # type: ignore[arg-type]
    for row, tolerance in enumerate(normalizers):
        if not math.isfinite(tolerance) or tolerance <= 0.0:
            raise Unresolved(f"{path}: invalid reconstructed normalizer at row {row}")

    candidate_upper_to_lower_rate = _finite(
        _field(primitive, "upper_to_lower_intercepted_mass_kg_m2_s", path),
        f"{path}.upper_to_lower_intercepted_mass_kg_m2_s",
    )
    candidate_upper_to_lower_h = _finite(
        _field(primitive, "upper_to_lower_intercepted_specific_enthalpy_j_kg", path),
        f"{path}.upper_to_lower_intercepted_specific_enthalpy_j_kg",
    )
    candidate_lower_release_rate = _finite(
        _field(primitive, "lower_terminal_release_mass_kg_m2_s", path),
        f"{path}.lower_terminal_release_mass_kg_m2_s",
    )
    if candidate_upper_to_lower_rate < 0.0 or candidate_lower_release_rate < 0.0:
        raise Unresolved(f"{path}: negative declared transfer/release rate")

    upper_capture_fraction = interception_parameter[0] * math.tanh(lai[0] + sai[0])
    lower_capture_fraction = interception_parameter[1] * math.tanh(lai[1] + sai[1])
    upper_rain_capture = upper_capture_fraction * top_rain
    upper_rain_remainder = top_rain - upper_rain_capture
    upper_stemflow = stemflow_fraction[0] * upper_rain_remainder
    upper_through_rain = (1.0 - stemflow_fraction[0]) * upper_rain_remainder
    upper_drainage = float(reservoirs[0]["drainage_kg_m2_s"])
    upper_wet_temperature = reservoirs[0]["wet_temperature_k"]
    if upper_wet_temperature is None:
        raise Unresolved(f"{path}.reservoirs[0].wet_temperature_k: required routing operand")
    upper_drainage_h = CW * (float(upper_wet_temperature) - TF)
    captured_upper_drainage = lower_capture_fraction * upper_drainage
    lower_incoming_before_capture = upper_through_rain + upper_drainage
    reconstructed_lower_incident = lower_capture_fraction * lower_incoming_before_capture
    reconstructed_lower_incident_energy = lower_capture_fraction * (
        upper_through_rain * top_rain_h + upper_drainage * upper_drainage_h
    )
    reconstructed_lower_incident_h = (
        reconstructed_lower_incident_energy / reconstructed_lower_incident
        if reconstructed_lower_incident > 0.0
        else 0.0
    )
    lower_bypass = (1.0 - lower_capture_fraction) * lower_incoming_before_capture
    lower_bypass_stemflow = stemflow_fraction[1] * lower_bypass
    lower_bypass_throughfall = (1.0 - stemflow_fraction[1]) * lower_bypass
    lower_bypass_energy = (1.0 - lower_capture_fraction) * (
        upper_through_rain * top_rain_h + upper_drainage * upper_drainage_h
    )
    lower_bypass_stemflow_energy = stemflow_fraction[1] * lower_bypass_energy
    lower_bypass_throughfall_energy = (1.0 - stemflow_fraction[1]) * lower_bypass_energy
    lower_release_rate = float(reservoirs[1]["drainage_kg_m2_s"])
    lower_wet_temperature = reservoirs[1]["wet_temperature_k"]
    if lower_wet_temperature is None:
        raise Unresolved(f"{path}.reservoirs[1].wet_temperature_k: required routing operand")
    lower_release_h = CW * (float(lower_wet_temperature) - TF)

    routing_candidate_comparisons = {
        "upper_reservoir_incident_mass_kg_m2_s": _routing_comparison(
            upper_rain_capture, float(reservoirs[0]["incident_liquid_kg_m2_s"])
        ),
        "upper_reservoir_incident_specific_enthalpy_j_kg": _routing_comparison(
            top_rain_h if upper_rain_capture > 0.0 else 0.0,
            float(reservoirs[0]["incident_enthalpy_j_kg"]),
        ),
        "lower_reservoir_incident_mass_kg_m2_s": _routing_comparison(
            reconstructed_lower_incident,
            float(reservoirs[1]["incident_liquid_kg_m2_s"]),
        ),
        "lower_reservoir_incident_specific_enthalpy_j_kg": _routing_comparison(
            reconstructed_lower_incident_h,
            float(reservoirs[1]["incident_enthalpy_j_kg"]),
        ),
        "captured_upper_drainage_mass_kg_m2_s": _routing_comparison(
            captured_upper_drainage, candidate_upper_to_lower_rate
        ),
        "captured_upper_drainage_specific_enthalpy_j_kg": _routing_comparison(
            upper_drainage_h, candidate_upper_to_lower_h
        ),
        "lower_reservoir_release_mass_kg_m2_s": _routing_comparison(
            lower_release_rate, candidate_lower_release_rate
        ),
    }
    routing_candidate_comparison = {
        "all_within_arithmetic_tolerance": all(
            value["within_arithmetic_tolerance"]
            for value in routing_candidate_comparisons.values()
        ),
        "operands": routing_candidate_comparisons,
    }

    transfer_ledger = {
        "upper_to_lower_intercepted_mass_kg_m2": dt * captured_upper_drainage,
        "upper_to_lower_intercepted_enthalpy_j_m2": dt * captured_upper_drainage * upper_drainage_h,
        "lower_total_incident_mass_kg_m2": dt * reconstructed_lower_incident,
        "lower_total_incident_enthalpy_j_m2": dt * reconstructed_lower_incident_energy,
        "terminal_upper_stemflow_mass_kg_m2": dt * upper_stemflow,
        "terminal_upper_stemflow_enthalpy_j_m2": dt * upper_stemflow * top_rain_h,
        "terminal_lower_bypass_stemflow_mass_kg_m2": dt * lower_bypass_stemflow,
        "terminal_lower_bypass_stemflow_enthalpy_j_m2": dt * lower_bypass_stemflow_energy,
        "terminal_lower_bypass_throughfall_mass_kg_m2": dt * lower_bypass_throughfall,
        "terminal_lower_bypass_throughfall_enthalpy_j_m2": dt * lower_bypass_throughfall_energy,
        "lower_terminal_release_mass_kg_m2": dt * lower_release_rate,
        "lower_terminal_release_enthalpy_j_m2": dt * lower_release_rate * lower_release_h,
        "top_rain_debit_credit_error_kg_m2": dt
        * (top_rain - upper_rain_capture - upper_stemflow - upper_through_rain),
        "upper_downstream_debit_credit_error_kg_m2": dt
        * (
            lower_incoming_before_capture
            - reconstructed_lower_incident
            - lower_bypass_stemflow
            - lower_bypass_throughfall
        ),
        "top_rain_debit_credit_error_j_m2": dt
        * (
            top_rain * top_rain_h
            - upper_rain_capture * top_rain_h
            - upper_stemflow * top_rain_h
            - upper_through_rain * top_rain_h
        ),
        "upper_downstream_debit_credit_error_j_m2": dt
        * (
            upper_through_rain * top_rain_h
            + upper_drainage * upper_drainage_h
            - reconstructed_lower_incident_energy
            - lower_bypass_stemflow_energy
            - lower_bypass_throughfall_energy
        ),
    }

    candidate_residuals = _field(primitive, "residuals", path)
    candidate = [
        _finite(value, f"{path}.residuals[{i}]")
        for i, value in enumerate(_array(candidate_residuals, 21, f"{path}.residuals"))
    ]
    rows = [_comparison(residuals[i], candidate[i]) for i in range(21)]
    for i, row in enumerate(rows):
        row["row"] = i
        row["reconstructed_over_normalizer"] = residuals[i] / normalizers[i]
    residual_comparison = {
        "status": "available",
        "all_exact": all(row["exact"] for row in rows),
        "rows": rows,
    }

    candidate_normalizers = _field(primitive, "normalizers", path)
    candidate = [
        _finite(value, f"{path}.normalizers[{i}]")
        for i, value in enumerate(_array(candidate_normalizers, 21, f"{path}.normalizers"))
    ]
    rows = [_comparison(normalizers[i], candidate[i]) for i in range(21)]
    for i, row in enumerate(rows):
        row["row"] = i
    normalizer_comparison = {
        "status": "available",
        "all_exact": all(row["exact"] for row in rows),
        "rows": rows,
    }

    return {
        **metadata,
        "status": "RESOLVED",
        "air_density_kg_m3": rho,
        "reconstructed_residuals": residuals,
        "reconstructed_scales": scales,
        "reconstructed_normalizers": normalizers,
        "candidate_residual_comparison": residual_comparison,
        "candidate_normalizer_comparison": normalizer_comparison,
        "operand_consistency": consistency,
        "reservoir_ledger": support_ledger,
        "transfer_ledger": transfer_ledger,
        "routing_candidate_comparison": routing_candidate_comparison,
        "reconstruction_inputs_exclude": [
            "residuals",
            "normalizers",
            "upper_to_lower_intercepted_mass_kg_m2_s",
            "upper_to_lower_intercepted_specific_enthalpy_j_kg",
            "lower_terminal_release_mass_kg_m2_s",
        ],
    }


def _records(document: Any) -> list[dict[str, Any]]:
    if isinstance(document, list):
        values = document
    elif isinstance(document, dict) and "supports" in document:
        values = document["supports"]
    elif isinstance(document, dict) and "records" in document:
        values = document["records"]
    elif isinstance(document, dict):
        values = [document]
    else:
        raise Unresolved("input: expected object or array")
    if not isinstance(values, list) or not values:
        raise Unresolved("input: support-record array is empty or malformed")
    return [_mapping(value, f"records[{i}]") for i, value in enumerate(values)]


def _load(text: str) -> Any:
    try:
        return json.loads(text)
    except json.JSONDecodeError as first_error:
        try:
            return [json.loads(line) for line in text.splitlines() if line.strip()]
        except json.JSONDecodeError:
            raise Unresolved(f"input: invalid JSON ({first_error})") from first_error


def reconstruct_document(document: Any) -> dict[str, Any]:
    records = _records(document)
    outputs: list[dict[str, Any]] = []
    for index, record in enumerate(records):
        try:
            outputs.append(reconstruct_support(record, index))
        except Unresolved as error:
            primitive = record.get("primitive", record)
            identity = {
                name: primitive.get(name, record.get(name)) if isinstance(primitive, dict) else None
                for name in ("case_id", "support_id", "profile", "formulation", "run_id")
            }
            outputs.append({**identity, "status": "UNRESOLVED", "reason": str(error)})

    cumulative: OrderedDict[tuple[Any, ...], dict[str, Any]] = OrderedDict()
    for output in outputs:
        if output["status"] != "RESOLVED":
            continue
        key = (
            output.get("run_id"),
            output.get("case_id"),
            output.get("formulation"),
            output.get("profile"),
            output.get("stability_variant"),
            output.get("cadence_seconds"),
        )
        if key not in cumulative:
            cumulative[key] = {
                "run_id": key[0],
                "case_id": key[1],
                "formulation": key[2],
                "profile": key[3],
                "stability_variant": key[4],
                "cadence_seconds": key[5],
                "support_count": 0,
                "last_support_id": None,
                "reservoirs": [
                    {
                        "storage_delta_mass_kg_m2": 0.0,
                        "source_mass_kg_m2": 0.0,
                        "mass_closure_kg_m2": 0.0,
                        "storage_delta_enthalpy_j_m2": 0.0,
                        "source_enthalpy_j_m2": 0.0,
                        "enthalpy_closure_j_m2": 0.0,
                        "drainage_transfer_mass_kg_m2": 0.0,
                        "drainage_transfer_enthalpy_j_m2": 0.0,
                    }
                    for _ in range(2)
                ],
                "transfer": {
                    "upper_to_lower_intercepted_mass_kg_m2": 0.0,
                    "upper_to_lower_intercepted_enthalpy_j_m2": 0.0,
                    "lower_total_incident_mass_kg_m2": 0.0,
                    "lower_total_incident_enthalpy_j_m2": 0.0,
                    "terminal_upper_stemflow_mass_kg_m2": 0.0,
                    "terminal_upper_stemflow_enthalpy_j_m2": 0.0,
                    "terminal_lower_bypass_stemflow_mass_kg_m2": 0.0,
                    "terminal_lower_bypass_stemflow_enthalpy_j_m2": 0.0,
                    "terminal_lower_bypass_throughfall_mass_kg_m2": 0.0,
                    "terminal_lower_bypass_throughfall_enthalpy_j_m2": 0.0,
                    "lower_terminal_release_mass_kg_m2": 0.0,
                    "lower_terminal_release_enthalpy_j_m2": 0.0,
                    "top_rain_debit_credit_error_kg_m2": 0.0,
                    "upper_downstream_debit_credit_error_kg_m2": 0.0,
                    "top_rain_debit_credit_error_j_m2": 0.0,
                    "upper_downstream_debit_credit_error_j_m2": 0.0,
                },
                "support_order_valid": True,
            }
        group = cumulative[key]
        previous = group["last_support_id"]
        support_id = output["support_id"]
        if previous is not None and support_id <= previous:
            group["support_order_valid"] = False
        group["last_support_id"] = support_id
        group["support_count"] += 1
        for occupancy in range(2):
            for name, value in output["reservoir_ledger"][occupancy].items():
                if name != "occupancy":
                    group["reservoirs"][occupancy][name] += value
        for name, value in output["transfer_ledger"].items():
            group["transfer"][name] += value

    unresolved = sum(output["status"] != "RESOLVED" for output in outputs)
    routing_mismatch = sum(
        output["status"] == "RESOLVED"
        and not output["routing_candidate_comparison"]["all_within_arithmetic_tolerance"]
        for output in outputs
    )
    invalid_order = sum(not group["support_order_valid"] for group in cumulative.values())
    return {
        "schema": SCHEMA,
        "authority": SOURCE_AUTHORITY,
        "method": "raw primitive reconstruction; candidate residual/helper exclusion",
        "status": "RESOLVED"
        if unresolved == 0 and routing_mismatch == 0 and invalid_order == 0
        else "UNRESOLVED",
        "record_count": len(outputs),
        "unresolved_record_count": unresolved,
        "routing_candidate_mismatch_count": routing_mismatch,
        "invalid_support_order_group_count": invalid_order,
        "supports": outputs,
        "cumulative_ledgers": list(cumulative.values()),
        "cumulative_summation": "chronological binary64 addition in input order",
    }


def _synthetic_record() -> dict[str, Any]:
    half_tanh_area = 0.5493061443340549
    upper_capture_fraction = 0.4 * math.tanh(half_tanh_area)
    lower_capture_fraction = 0.6 * math.tanh(half_tanh_area)
    top_rain = 0.1
    upper_rain_capture = upper_capture_fraction * top_rain
    upper_through_rain = 0.75 * (top_rain - upper_rain_capture)
    upper_drainage = 0.01
    upper_wet_temperature = TF + 5.0 / CW
    upper_drainage_h = CW * (upper_wet_temperature - TF)
    lower_incident = lower_capture_fraction * (upper_through_rain + upper_drainage)
    lower_incident_h = lower_capture_fraction * (
        upper_through_rain * 5.0 + upper_drainage * upper_drainage_h
    ) / lower_incident
    coordinates = [273.0, 273.0, 273.0, 1.0, 2.0, 0.01,
                   273.0, 273.0, 273.0, 3.0, 4.0, 0.02,
                   273.0, 0.005, 270.0, 280.0, 281.0, 282.0, 283.0, 284.0, 285.0]
    reservoirs = [
        {
            "beginning_mass_kg_m2": 0.5, "beginning_enthalpy_j_m2": 1.0,
            "ending_mass_kg_m2": 1.0, "ending_enthalpy_j_m2": 2.0,
            "dt_s": 60.0, "incident_liquid_kg_m2_s": upper_rain_capture,
            "incident_enthalpy_j_kg": 5.0, "vapor_kg_m2_s": 0.0,
            "drainage_kg_m2_s": 0.01, "non_vapor_heat_w_m2": 9.0,
            "wet_temperature_k": upper_wet_temperature, "diagnosed_liquid_mass_kg_m2": 4.0,
            "liquid_capacity_kg_m2": 10.0, "wet_fraction": 0.25,
            "wet_longwave_w_m2": 4.0, "wet_sensible_w_m2": 1.0,
        },
        {
            "beginning_mass_kg_m2": 2.0, "beginning_enthalpy_j_m2": 3.0,
            "ending_mass_kg_m2": 3.0, "ending_enthalpy_j_m2": 4.0,
            "dt_s": 60.0, "incident_liquid_kg_m2_s": lower_incident,
            "incident_enthalpy_j_kg": lower_incident_h, "vapor_kg_m2_s": 0.0,
            "drainage_kg_m2_s": 0.02, "non_vapor_heat_w_m2": 7.5,
            "wet_temperature_k": 273.15, "diagnosed_liquid_mass_kg_m2": 5.0,
            "liquid_capacity_kg_m2": 8.0, "wet_fraction": 0.5,
            "wet_longwave_w_m2": 2.0, "wet_sensible_w_m2": 0.5,
        },
    ]
    expected = [4.0, 8.0, 12.0, -0.1,
                1.0 - 60.0 * (9.0 + 5.0 * upper_rain_capture - upper_drainage * upper_drainage_h), 0.6,
                1.5, 3.0, 4.5, 1.0 - 60.0 * (lower_incident - 0.02),
                1.0 - 60.0 * (7.5 + lower_incident * lower_incident_h), 1.2] + [0.0] * 9
    return {
        "case_id": "synthetic",
        "support_id": 0,
        "profile": "P0",
        "coordinates": coordinates,
        "residuals": expected,
        "normalizers": [1.0] * 21,
        "interval_s": 60.0,
        "pressure_pa": R_DRY * 273.0,
        "latent_heat_j_kg": 10.0,
        "canopy_to_atmosphere_heat_resistance_s_m": 1.0,
        "canopy_to_atmosphere_vapor_resistance_s_m": 1.0,
        "prescribed_snow_temperature_k": 270.0,
        "prescribed_soil_temperature_k": [280.0, 281.0, 282.0, 283.0, 284.0, 285.0],
        "air_temperature_k": 273.0,
        "air_humidity_kg_kg": 0.005,
        "downward_longwave_w_m2": 0.0,
        "boundary_sensible_w_m2": -1.5,
        "boundary_vapor_kg_m2_s": 0.0,
        "reservoirs": reservoirs,
        "leaf_vapor_kg_m2_s": [[0.0, 0.0], [0.0, 0.0]],
        "leaf_conductance_m_s": [[0.0, 0.0], [0.0, 0.0]],
        "wet_vapor_kg_m2_s": [0.0, 0.0],
        "wet_sensible_w_m2": [1.0, 0.5],
        "wet_longwave_w_m2": [4.0, 2.0],
        "gb_leaf_m_s": [0.0, 0.0],
        "gb_stem_m_s": [0.0, 0.0],
        "structural_area_m2_m2": [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]],
        "absorbed_shortwave_w_m2": [[4.0, 8.0, 12.0], [2.0, 4.0, 6.0]],
        "dry_net_longwave_w_m2": [[1.0, 2.0, 3.0], [0.5, 1.0, 1.5]],
        "top_rain_kg_m2_s": top_rain,
        "top_rain_specific_enthalpy_j_kg": 5.0,
        "lai_m2_m2": [half_tanh_area, half_tanh_area],
        "sai_m2_m2": [0.0, 0.0],
        "liquid_interception_fraction": [0.4, 0.6],
        "stemflow_fraction": [0.25, 0.4],
        "upper_to_lower_intercepted_mass_kg_m2_s": lower_capture_fraction * upper_drainage,
        "upper_to_lower_intercepted_specific_enthalpy_j_kg": upper_drainage_h,
        "lower_terminal_release_mass_kg_m2_s": 0.02,
    }


def self_check() -> dict[str, Any]:
    record = _synthetic_record()
    expected = record["residuals"][:]
    result = reconstruct_document({"supports": [record]})
    actual = result["supports"][0]["reconstructed_residuals"]
    residual_oracle = all(math.isclose(a, b, rel_tol=0.0, abs_tol=1.0e-12) for a, b in zip(actual, expected))
    ledger = result["supports"][0]
    ledger_oracle = (
        math.isclose(ledger["transfer_ledger"]["upper_to_lower_intercepted_mass_kg_m2"], 0.18)
        and math.isclose(ledger["transfer_ledger"]["upper_to_lower_intercepted_enthalpy_j_m2"], 0.9)
        and math.isclose(ledger["transfer_ledger"]["lower_terminal_release_mass_kg_m2"], 1.2)
        and abs(ledger["transfer_ledger"]["top_rain_debit_credit_error_kg_m2"]) <= 1.0e-15
        and abs(ledger["transfer_ledger"]["upper_downstream_debit_credit_error_kg_m2"]) <= 1.0e-15
        and abs(ledger["transfer_ledger"]["top_rain_debit_credit_error_j_m2"]) <= 1.0e-12
        and abs(ledger["transfer_ledger"]["upper_downstream_debit_credit_error_j_m2"]) <= 1.0e-12
        and ledger["routing_candidate_comparison"]["all_within_arithmetic_tolerance"]
    )
    old_total_incident = copy.deepcopy(record)
    old_total_incident["upper_to_lower_intercepted_mass_kg_m2_s"] = record["reservoirs"][1][
        "incident_liquid_kg_m2_s"
    ]
    old_total_incident_result = reconstruct_document({"supports": [old_total_incident]})
    old_total_incident_rejected = (
        old_total_incident_result["status"] == "UNRESOLVED"
        and old_total_incident_result["routing_candidate_mismatch_count"] == 1
        and not old_total_incident_result["supports"][0]["routing_candidate_comparison"]
        ["operands"]["captured_upper_drainage_mass_kg_m2_s"]
        ["within_arithmetic_tolerance"]
    )
    raw_upper_drainage = copy.deepcopy(record)
    raw_upper_drainage["upper_to_lower_intercepted_mass_kg_m2_s"] = record["reservoirs"][0][
        "drainage_kg_m2_s"
    ]
    raw_upper_drainage_result = reconstruct_document({"supports": [raw_upper_drainage]})
    raw_upper_drainage_rejected = (
        raw_upper_drainage_result["status"] == "UNRESOLVED"
        and raw_upper_drainage_result["routing_candidate_mismatch_count"] == 1
        and not raw_upper_drainage_result["supports"][0]["routing_candidate_comparison"]
        ["operands"]["captured_upper_drainage_mass_kg_m2_s"]
        ["within_arithmetic_tolerance"]
    )
    poisoned = copy.deepcopy(record)
    poisoned["residuals"] = [999.0] * 21
    poison_result = reconstruct_document({"supports": [poisoned]})
    poison_invariance = poison_result["supports"][0]["reconstructed_residuals"] == actual
    missing = copy.deepcopy(record)
    del missing["pressure_pa"]
    missing_result = reconstruct_document({"supports": [missing]})
    missing_fail_closed = (
        missing_result["status"] == "UNRESOLVED"
        and "pressure_pa" in missing_result["supports"][0]["reason"]
    )
    missing_routing = copy.deepcopy(record)
    del missing_routing["top_rain_kg_m2_s"]
    missing_routing_result = reconstruct_document({"supports": [missing_routing]})
    missing_routing_fail_closed = (
        missing_routing_result["status"] == "UNRESOLVED"
        and "top_rain_kg_m2_s" in missing_routing_result["supports"][0]["reason"]
    )
    missing_residuals = copy.deepcopy(record)
    del missing_residuals["residuals"]
    missing_residuals_result = reconstruct_document({"supports": [missing_residuals]})
    missing_residuals_fail_closed = (
        missing_residuals_result["status"] == "UNRESOLVED"
        and "residuals" in missing_residuals_result["supports"][0]["reason"]
    )
    missing_normalizers = copy.deepcopy(record)
    del missing_normalizers["normalizers"]
    missing_normalizers_result = reconstruct_document({"supports": [missing_normalizers]})
    missing_normalizers_fail_closed = (
        missing_normalizers_result["status"] == "UNRESOLVED"
        and "normalizers" in missing_normalizers_result["supports"][0]["reason"]
    )
    try:
        reconstruct_document({"supports": []})
        empty_support_list_fail_closed = False
    except Unresolved as error:
        empty_support_list_fail_closed = "empty" in str(error)
    strict = copy.deepcopy(record)
    strict["stability_variant"] = "strict_reference"
    strict_result = reconstruct_document({"supports": [strict]})
    strict_support = strict_result["supports"][0]
    strict_norm = strict_support["reconstructed_normalizers"]
    strict_scales = strict_support["reconstructed_scales"]
    strict_reference_vector = (
        strict_result["status"] == "RESOLVED"
        and strict_norm[0] == 1.0e-7 + 1.0e-11 * max(strict_scales[0], 1.0)
        and strict_norm[13] == 1.0e-13 + 1.0e-10 * max(strict_scales[13], 0.0)
        and strict_norm[3] == min(1.0e-10, 1.0e-9 * max(0.5, strict_scales[3], 1.0e-9))
        and strict_norm[4] == min(1.0e-7, 1.0e-9 * max(1.0, strict_scales[4], 1.0))
        and strict_norm[5] == min(1.0e-10, 1.0e-9 * 10.0)
        and strict_norm[14] == 1.0e-9
    )
    tests = {
        "hardcoded_21_row_oracle": residual_oracle,
        "hardcoded_transfer_ledger_oracle": ledger_oracle,
        "old_total_lower_incident_transfer_label_rejected": old_total_incident_rejected,
        "raw_upper_drainage_transfer_label_rejected": raw_upper_drainage_rejected,
        "candidate_residual_poison_invariance": poison_invariance,
        "missing_operand_fails_closed": missing_fail_closed,
        "missing_routing_operand_fails_closed": missing_routing_fail_closed,
        "missing_candidate_residuals_fail_closed": missing_residuals_fail_closed,
        "missing_candidate_normalizers_fail_closed": missing_normalizers_fail_closed,
        "empty_support_list_fails_closed": empty_support_list_fail_closed,
        "strict_reference_tolerance_vector": strict_reference_vector,
    }
    return {
        "schema": f"{SCHEMA}.self-check",
        "authority": SOURCE_AUTHORITY,
        "status": "PASS" if all(tests.values()) else "FAIL",
        "tests": tests,
        "synthetic_only": True,
        "physical_execution": False,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", nargs="?", help="JSON/JSONL primitive record file; omit for stdin")
    parser.add_argument("--output", help="write report JSON to this path")
    parser.add_argument("--self-check", action="store_true", help="run arithmetic-only synthetic checks")
    parser.add_argument("--pretty", action="store_true", help="indent JSON output")
    args = parser.parse_args()
    if args.self_check:
        report = self_check()
    else:
        text = pathlib.Path(args.input).read_text(encoding="utf-8") if args.input else sys.stdin.read()
        try:
            report = reconstruct_document(_load(text))
        except Unresolved as error:
            report = {"schema": SCHEMA, "status": "UNRESOLVED", "reason": str(error)}
    encoded = json.dumps(report, indent=2 if args.pretty else None, sort_keys=True, allow_nan=False) + "\n"
    if args.output:
        pathlib.Path(args.output).write_text(encoded, encoding="utf-8")
    else:
        sys.stdout.write(encoded)
    return 0 if report["status"] in {"RESOLVED", "PASS"} else 2


if __name__ == "__main__":
    raise SystemExit(main())
