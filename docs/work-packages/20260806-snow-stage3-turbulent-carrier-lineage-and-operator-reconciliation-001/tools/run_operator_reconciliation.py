#!/usr/bin/env python3
"""Execute and independently reconcile the frozen Stage 3 operators."""

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
import struct
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
FREEZE_PATH = PACKAGE / "artifacts/protocol-freeze.json"
OUTPUT = REPO / "target/snow_stage3_operator_reconciliation"
PREDECESSOR_TOOL = REPO / (
    "docs/work-packages/20260806-snow-stage3-four-site-carrier-term-audit-001/"
    "tools/run_carrier_term_audit.py"
)
W1_TOOL = REPO / (
    "docs/work-packages/20260802-snow-surface-eb-04w1-"
    "precipitation-scaling-calibration-001/tools/run_precipitation_scaling.py"
)
BINARY = REPO / "target/release/openwepp-cli-hill"
TERMS = ("shortwave", "longwave", "sensible", "latent", "advected")
ZERO_J_M2 = 1.0e-6
PREDECESSOR_MJ_M2 = 170.2536089
PREDECESSOR_TOL_MJ_M2 = 1.0e-7
SIGMA = 5.670_32e-8
FREEZE_K = 273.16
VON_KARMAN = 0.41
CP_AIR = 1_005.0
GRAVITY = 9.80665
MOLAR_DRY = 28.9644
MOLAR_WATER = 18.0153
UNIVERSAL_GAS = 8.31432e3
CALORIE_TO_JOULE = 4.186_798_188
LONGWAVE_MODEL = "dilley_unsworth_subcanopy_v1"
SUBLIMATION_MODEL = "disabled"
FLUX_FIELDS = (
    "net_shortwave_w_m2",
    "net_longwave_w_m2",
    "sensible_flux_w_m2",
    "latent_flux_w_m2",
    "precipitation_advected_flux_w_m2",
)
FIXED_REFERENCE_FIELDS = (
    "air_pressure_pa",
    "wind_speed_m_s",
    "dewpoint_c",
    "daily_solar_radiation_mj_m2",
    "daily_extraterrestrial_radiation_mj_m2",
    "daylight",
    "canopy_cover_fraction",
    "air_temperature_height_m",
    "vapor_pressure_height_m",
    "wind_speed_height_m",
    "aerodynamic_roughness_length_m",
    "turbulent_max_iterations",
    "turbulent_convergence_tolerance",
    "longwave_model_id",
    "sublimation_model_id",
)
TUPLE_FIELDS = frozenset(
    (
        "operator_id", "hour_index", "substep_index", "elapsed_start_seconds",
        "requested_seconds", "evaluated_seconds", "duration_seconds", "applicable",
        "applicability_reason", "source_fingerprint_fnv1a64",
        "forcing_fingerprint_fnv1a64", "geometry_fingerprint_fnv1a64",
        "effective_input_fingerprint_fnv1a64", "projection_id",
        "active_layer_prefix_count_before", "total_layer_count_before",
        "active_layer_state_fingerprint_before_fnv1a64",
        "total_layer_state_fingerprint_before_fnv1a64",
        "active_layer_prefix_count_after", "total_layer_count_after",
        "active_layer_state_fingerprint_after_fnv1a64",
        "total_layer_state_fingerprint_after_fnv1a64", "after_surface_applicable",
        "after_surface_applicability_reason", "active_ice_mass_before_kg_m2",
        "active_ice_mass_after_kg_m2", "total_ice_mass_before_kg_m2",
        "total_ice_mass_after_kg_m2", "active_depth_before_m",
        "active_depth_after_m", "active_density_before_kg_m3",
        "active_density_after_kg_m3", "active_cold_before_j_m2",
        "active_cold_after_j_m2", "total_cold_before_j_m2",
        "total_cold_after_j_m2", "surface_temperature_before_c",
        "surface_temperature_after_c", "air_temperature_c", "dewpoint_c",
        "wind_speed_m_s", "air_pressure_pa", "hourly_radiation_mj_m2",
        "daily_solar_radiation_mj_m2", "daily_extraterrestrial_radiation_mj_m2",
        "daylight", "canopy_cover_fraction", "rain_m", "snowfall_geometric_m",
        "rain_mass_flux_kg_m2_s", "snow_mass_flux_kg_m2_s", "rain_temperature_c",
        "snow_temperature_c", "rain_specific_heat_j_kg_k",
        "snow_specific_heat_j_kg_k", "incoming_shortwave_w_m2",
        "snow_albedo_fraction", "snow_albedo_source_id", "snow_albedo_model_id",
        "snow_albedo_accumulated_positive_temperature_c_day", "net_shortwave_w_m2",
        "actual_vapor_pressure_pa", "longwave_cloud_fraction", "sky_view_fraction",
        "atmospheric_longwave_w_m2", "canopy_longwave_w_m2",
        "subcanopy_longwave_w_m2", "outgoing_longwave_w_m2", "net_longwave_w_m2",
        "longwave_model_id", "sublimation_model_id", "air_temperature_height_m",
        "vapor_pressure_height_m", "wind_speed_height_m",
        "aerodynamic_roughness_length_m", "turbulent_max_iterations",
        "turbulent_convergence_tolerance", "surface_vapor_pressure_pa",
        "air_potential_temperature_k", "surface_temperature_k",
        "specific_humidity_air_kg_kg", "specific_humidity_surface_kg_kg",
        "air_density_kg_m3", "displacement_height_m", "log_momentum",
        "log_sensible", "log_latent", "turbulent_termination_status",
        "stability_class", "obukhov_length_m", "psi_momentum", "psi_sensible",
        "psi_latent", "turbulent_iterations", "friction_velocity_m_s",
        "sensible_exchange_velocity_m_s", "latent_exchange_velocity_m_s",
        "surface_latent_heat_j_kg", "vapor_mass_flux_kg_m2_s",
        "sensible_flux_w_m2", "latent_flux_w_m2",
        "precipitation_advected_flux_w_m2", "complete_external_flux_w_m2",
        "vapor_mass_exchange_kg_m2", "sublimation_kg_m2", "deposition_kg_m2",
        "melt_kg_m2", "active_cold_energy_change_j_m2",
        "lower_cold_energy_change_j_m2", "cold_content_export_j_m2",
        "internal_active_lower_conduction_j_m2", "legacy_sequential_complete_j_m2",
        "energy_closure_residual_j_m2",
    )
)


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
    path.write_bytes(json_bytes(value))


def json_bytes(value: Any) -> bytes:
    return (
        json.dumps(value, allow_nan=False, indent=2, sort_keys=True) + "\n"
    ).encode("utf-8")


def required(row: dict[str, Any], field: str) -> Any:
    if field not in row:
        raise RuntimeError(f"missing required field {field}")
    return row[field]


def checked_number(row: dict[str, Any], field: str) -> float:
    value = required(row, field)
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise RuntimeError(f"{field} is not numeric")
    value = float(value)
    if not math.isfinite(value):
        raise RuntimeError(f"{field} is not finite")
    return value


def close(name: str, actual: float, expected: float, floor: float) -> None:
    tolerance = max(floor, 1.0e-12 * (abs(actual) + abs(expected)))
    if not math.isfinite(actual) or abs(actual - expected) > tolerance:
        raise RuntimeError(
            f"{name} failed: actual={actual:.17g} expected={expected:.17g} tol={tolerance:.3g}"
        )


def same_bits(left: Any, right: Any) -> bool:
    return (
        isinstance(left, (int, float))
        and not isinstance(left, bool)
        and isinstance(right, (int, float))
        and not isinstance(right, bool)
        and struct.pack(">d", float(left)) == struct.pack(">d", float(right))
    )


def require_same_bits(name: str, left: Any, right: Any) -> None:
    if not same_bits(left, right):
        raise RuntimeError(f"{name} differs in IEEE-754 representation")


def checked_optional_number(row: dict[str, Any], field: str) -> float | None:
    value = required(row, field)
    return None if value is None else checked_number(row, field)


def checked_int(row: dict[str, Any], field: str) -> int:
    value = required(row, field)
    if isinstance(value, bool) or not isinstance(value, int):
        raise RuntimeError(f"{field} is not an integer")
    return value


def checked_bool(row: dict[str, Any], field: str) -> bool:
    value = required(row, field)
    if not isinstance(value, bool):
        raise RuntimeError(f"{field} is not a boolean")
    return value


def checked_string(row: dict[str, Any], field: str) -> str:
    value = required(row, field)
    if not isinstance(value, str) or not value:
        raise RuntimeError(f"{field} is not a non-empty string")
    return value


def checked_fingerprint(row: dict[str, Any], field: str) -> str:
    value = checked_string(row, field)
    if not re.fullmatch(r"[0-9a-f]{16}", value):
        raise RuntimeError(f"{field} is not a 16-digit lowercase hex fingerprint")
    return value


def validate_tuple(row: dict[str, Any], operator: str) -> dict[str, float]:
    missing = TUPLE_FIELDS - row.keys()
    extra = row.keys() - TUPLE_FIELDS
    if missing or extra:
        raise RuntimeError(
            f"tuple exact-field mismatch: missing={sorted(missing)} extra={sorted(extra)}"
        )
    if checked_string(row, "operator_id") != operator or checked_bool(row, "applicable") is not True:
        raise RuntimeError("tuple operator/applicability mismatch")
    if checked_string(row, "applicability_reason") != "evaluated":
        raise RuntimeError("tuple applicability reason mismatch")
    hour = checked_int(row, "hour_index")
    substep = checked_int(row, "substep_index")
    if not 0 <= hour < 24 or substep < 0:
        raise RuntimeError("tuple hour/substep identity invalid")
    duration = checked_number(row, "duration_seconds")
    requested = checked_number(row, "requested_seconds")
    evaluated = checked_number(row, "evaluated_seconds")
    if duration <= 0.0 or requested != 3_600.0 or evaluated != duration or evaluated > requested:
        raise RuntimeError("tuple duration mismatch")
    for field in (
        "source_fingerprint_fnv1a64", "forcing_fingerprint_fnv1a64",
        "geometry_fingerprint_fnv1a64", "effective_input_fingerprint_fnv1a64",
        "active_layer_state_fingerprint_before_fnv1a64",
        "total_layer_state_fingerprint_before_fnv1a64",
        "total_layer_state_fingerprint_after_fnv1a64",
    ):
        checked_fingerprint(row, field)
    for field in (
        "active_layer_prefix_count_before", "total_layer_count_before",
        "total_layer_count_after", "turbulent_max_iterations", "turbulent_iterations",
    ):
        if checked_int(row, field) < 0:
            raise RuntimeError(f"{field} is negative")
    if checked_int(row, "turbulent_max_iterations") == 0:
        raise RuntimeError("turbulent_max_iterations must be positive")
    checked_bool(row, "daylight")
    checked_string(row, "projection_id")
    checked_string(row, "snow_albedo_source_id")
    checked_string(row, "longwave_model_id")
    if checked_string(row, "sublimation_model_id") != SUBLIMATION_MODEL:
        raise RuntimeError("unexpected sublimation model")
    wind = checked_number(row, "wind_speed_m_s")
    pressure = checked_number(row, "air_pressure_pa")
    z_t = checked_number(row, "air_temperature_height_m")
    z_q = checked_number(row, "vapor_pressure_height_m")
    z_u = checked_number(row, "wind_speed_height_m")
    z_0 = checked_number(row, "aerodynamic_roughness_length_m")
    convergence_tolerance = checked_number(
        row, "turbulent_convergence_tolerance"
    )
    if wind < 0.0 or pressure <= 0.0 or z_0 <= 0.0 or convergence_tolerance <= 0.0:
        raise RuntimeError("invalid turbulent geometry/options domain")
    displacement = 2.0 * 7.35 * z_0 / 3.0
    if any(height <= z_0 or height <= displacement for height in (z_t, z_q, z_u)):
        raise RuntimeError("invalid turbulent measurement height")
    albedo_model = required(row, "snow_albedo_model_id")
    if albedo_model is not None and (not isinstance(albedo_model, str) or not albedo_model):
        raise RuntimeError("snow_albedo_model_id applicability/type mismatch")
    albedo_temperature = checked_optional_number(
        row, "snow_albedo_accumulated_positive_temperature_c_day"
    )
    if checked_string(row, "snow_albedo_source_id") == "stage3_default_snow_albedo_0p82":
        if albedo_model is not None or albedo_temperature is not None:
            raise RuntimeError("default albedo lineage must use required null state fields")
    elif checked_string(row, "snow_albedo_source_id") == "snow_albedo_state":
        if albedo_model is None or albedo_temperature is None:
            raise RuntimeError("state albedo lineage is incomplete")
    else:
        raise RuntimeError("unknown snow albedo source")
    for field in (
        "active_ice_mass_before_kg_m2", "total_ice_mass_before_kg_m2",
        "total_ice_mass_after_kg_m2", "active_depth_before_m",
        "active_density_before_kg_m3", "active_cold_before_j_m2",
        "total_cold_before_j_m2", "total_cold_after_j_m2",
        "surface_temperature_before_c",
    ):
        checked_number(row, field)
    incoming = checked_number(row, "hourly_radiation_mj_m2") * 1.0e6 / 3_600.0
    close("incoming shortwave", checked_number(row, "incoming_shortwave_w_m2"), incoming, 1.0e-10)
    net_shortwave = incoming * (1.0 - checked_number(row, "snow_albedo_fraction"))
    close("net shortwave", checked_number(row, "net_shortwave_w_m2"), net_shortwave, 1.0e-10)
    close(
        "net longwave",
        checked_number(row, "net_longwave_w_m2"),
        checked_number(row, "subcanopy_longwave_w_m2")
        - checked_number(row, "outgoing_longwave_w_m2"),
        1.0e-10,
    )
    rain_flux = checked_number(row, "rain_m") * 1_000.0 / 3_600.0
    snow_flux = checked_number(row, "snowfall_geometric_m") * 0.1 * 1_000.0 / 3_600.0
    close("rain mass flux", checked_number(row, "rain_mass_flux_kg_m2_s"), rain_flux, 1.0e-12)
    close("snow mass flux", checked_number(row, "snow_mass_flux_kg_m2_s"), snow_flux, 1.0e-12)
    rain_cp = 4_217.7 - 2.55 * checked_number(row, "rain_temperature_c")
    snow_cp = CALORIE_TO_JOULE * (
        0.024_928 + 0.001_76 * (checked_number(row, "snow_temperature_c") + FREEZE_K)
    ) / 0.001
    close("rain heat capacity", checked_number(row, "rain_specific_heat_j_kg_k"), rain_cp, 1.0e-10)
    close("snow heat capacity", checked_number(row, "snow_specific_heat_j_kg_k"), snow_cp, 1.0e-10)
    advected = (
        rain_cp * rain_flux * (checked_number(row, "rain_temperature_c") - checked_number(row, "surface_temperature_before_c"))
        + snow_cp * snow_flux * (checked_number(row, "snow_temperature_c") - checked_number(row, "surface_temperature_before_c"))
    )
    close("precipitation advection", checked_number(row, "precipitation_advected_flux_w_m2"), advected, 1.0e-10)
    close("actual vapor pressure", checked_number(row, "actual_vapor_pressure_pa"), saturation_vapor_pressure(checked_number(row, "dewpoint_c")), 1.0e-10)
    close("surface vapor pressure", checked_number(row, "surface_vapor_pressure_pa"), saturation_vapor_pressure(checked_number(row, "surface_temperature_before_c")), 1.0e-10)
    longwave = reconstruct_longwave(row, checked_number(row, "surface_temperature_before_c"))
    for field, key in (
        ("longwave_cloud_fraction", "cloud"),
        ("sky_view_fraction", "sky_view"),
        ("atmospheric_longwave_w_m2", "atmospheric"),
        ("canopy_longwave_w_m2", "canopy"),
        ("subcanopy_longwave_w_m2", "subcanopy"),
        ("outgoing_longwave_w_m2", "outgoing"),
        ("net_longwave_w_m2", "net"),
    ):
        close(f"independent {field}", checked_number(row, field), longwave[key], 1.0e-10)
    independently_reconstructed = monin_obukhov(
        air_temperature_c=checked_number(row, "air_temperature_c"),
        surface_temperature_c=checked_number(row, "surface_temperature_before_c"),
        air_vapor_pressure_pa=checked_number(row, "actual_vapor_pressure_pa"),
        surface_vapor_pressure_pa=checked_number(row, "surface_vapor_pressure_pa"),
        air_pressure_pa=checked_number(row, "air_pressure_pa"),
        wind_speed_m_s=checked_number(row, "wind_speed_m_s"),
        z_t=z_t,
        z_q=z_q,
        z_u=z_u,
        z_0=z_0,
        max_iterations=checked_int(row, "turbulent_max_iterations"),
        tolerance=convergence_tolerance,
    )
    if row.get("turbulent_termination_status") != independently_reconstructed["status"] or row.get("stability_class") != independently_reconstructed["class"]:
        raise RuntimeError("turbulent status/class mismatch")
    if row.get("turbulent_iterations") != independently_reconstructed["iterations"]:
        raise RuntimeError("turbulent iteration mismatch")
    for field, key in (
        ("obukhov_length_m", "obukhov"),
        ("displacement_height_m", "displacement"),
        ("log_momentum", "log_momentum"),
        ("log_sensible", "log_sensible"),
        ("log_latent", "log_latent"),
        ("sensible_exchange_velocity_m_s", "sensible_exchange"),
        ("latent_exchange_velocity_m_s", "latent_exchange"),
        ("air_density_kg_m3", "density"),
        ("air_potential_temperature_k", "potential_temperature"),
        ("surface_temperature_k", "surface_temperature_k"),
        ("specific_humidity_air_kg_kg", "q_air"),
        ("specific_humidity_surface_kg_kg", "q_surface"),
        ("surface_latent_heat_j_kg", "latent_heat"),
    ):
        actual = checked_optional_number(row, field)
        expected = independently_reconstructed[key]
        if actual is None or expected is None:
            if actual is not None or expected is not None:
                raise RuntimeError(f"{field} applicability mismatch")
        else:
            close(f"independent {field}", actual, float(expected), 1.0e-10)
    for field, key in (
        ("psi_momentum", "psi_momentum"),
        ("psi_sensible", "psi_sensible"),
        ("psi_latent", "psi_latent"),
        ("friction_velocity_m_s", "friction"),
        ("vapor_mass_flux_kg_m2_s", "mass"),
        ("sensible_flux_w_m2", "sensible"),
        ("latent_flux_w_m2", "latent"),
    ):
        close(f"independent {field}", checked_number(row, field), float(independently_reconstructed[key]), 1.0e-10)
    external = (
        net_shortwave
        + longwave["net"]
        + float(independently_reconstructed["sensible"])
        + float(independently_reconstructed["latent"])
        + advected
    )
    close("complete external", checked_number(row, "complete_external_flux_w_m2"), external, 1.0e-10)
    close(
        "vapor duration integral",
        checked_number(row, "vapor_mass_exchange_kg_m2"),
        checked_number(row, "vapor_mass_flux_kg_m2_s") * duration,
        1.0e-12,
    )
    if operator == "same_state_paired_carrier_v1":
        for before, after in (
            ("active_ice_mass_before_kg_m2", "active_ice_mass_after_kg_m2"),
            ("active_depth_before_m", "active_depth_after_m"),
            ("active_density_before_kg_m3", "active_density_after_kg_m3"),
            ("active_cold_before_j_m2", "active_cold_after_j_m2"),
            ("surface_temperature_before_c", "surface_temperature_after_c"),
            ("total_ice_mass_before_kg_m2", "total_ice_mass_after_kg_m2"),
            ("total_cold_before_j_m2", "total_cold_after_j_m2"),
        ):
            require_same_bits(f"same-state {after}", row.get(after), row.get(before))
        for before, after in (
            ("active_layer_prefix_count_before", "active_layer_prefix_count_after"),
            ("total_layer_count_before", "total_layer_count_after"),
            ("active_layer_state_fingerprint_before_fnv1a64", "active_layer_state_fingerprint_after_fnv1a64"),
            ("total_layer_state_fingerprint_before_fnv1a64", "total_layer_state_fingerprint_after_fnv1a64"),
        ):
            if required(row, after) != required(row, before):
                raise RuntimeError(f"same-state {after} differs")
        for field in (
            "active_layer_prefix_count_after",
            "active_layer_state_fingerprint_after_fnv1a64",
        ):
            if field.endswith("count_after"):
                checked_int(row, field)
            else:
                checked_fingerprint(row, field)
        if checked_string(row, "projection_id") != "whole_column_immutable" or checked_bool(row, "after_surface_applicable") is not True or checked_string(row, "after_surface_applicability_reason") != "resolved_surface":
            raise RuntimeError("same-state applicability mismatch")
        for field in (
            "melt_kg_m2", "sublimation_kg_m2", "deposition_kg_m2",
            "active_cold_energy_change_j_m2", "lower_cold_energy_change_j_m2",
            "cold_content_export_j_m2", "internal_active_lower_conduction_j_m2",
            "legacy_sequential_complete_j_m2", "energy_closure_residual_j_m2",
        ):
            if required(row, field) is not None:
                raise RuntimeError(f"same-state {field} must be null")
    else:
        if checked_string(row, "projection_id") != "aligned_active_dynamic":
            raise RuntimeError("sequential projection mismatch")
        for field in (
            "melt_kg_m2", "sublimation_kg_m2", "deposition_kg_m2",
            "active_cold_energy_change_j_m2", "lower_cold_energy_change_j_m2",
            "cold_content_export_j_m2", "internal_active_lower_conduction_j_m2",
            "legacy_sequential_complete_j_m2", "energy_closure_residual_j_m2",
        ):
            checked_number(row, field)
        mass_after = checked_number(row, "total_ice_mass_after_kg_m2")
        mass_expected = (
            checked_number(row, "total_ice_mass_before_kg_m2")
            - checked_number(row, "melt_kg_m2")
            - checked_number(row, "sublimation_kg_m2")
            + checked_number(row, "deposition_kg_m2")
        )
        close("mass endpoint", mass_after, mass_expected, 1.0e-12)
        cold_after = checked_number(row, "total_cold_after_j_m2")
        cold_expected = (
            checked_number(row, "total_cold_before_j_m2")
            - checked_number(row, "active_cold_energy_change_j_m2")
            - checked_number(row, "lower_cold_energy_change_j_m2")
            - checked_number(row, "cold_content_export_j_m2")
        )
        close("cold endpoint", cold_after, cold_expected, 1.0e-6)
        legacy = external * duration + checked_number(
            row, "internal_active_lower_conduction_j_m2"
        )
        close("legacy sequential bridge", checked_number(row, "legacy_sequential_complete_j_m2"), legacy, 1.0e-6)
        close("sequential energy closure", checked_number(row, "energy_closure_residual_j_m2"), 0.0, 1.0e-6)
        active_after_fields = (
            "active_layer_prefix_count_after",
            "active_layer_state_fingerprint_after_fnv1a64",
            "active_ice_mass_after_kg_m2",
            "active_depth_after_m",
            "active_density_after_kg_m3",
            "active_cold_after_j_m2",
            "surface_temperature_after_c",
        )
        if checked_bool(row, "after_surface_applicable") is True:
            if checked_string(row, "after_surface_applicability_reason") != "resolved_surface" or any(required(row, field) is None for field in active_after_fields):
                raise RuntimeError("resolved after-surface applicability mismatch")
            checked_int(row, "active_layer_prefix_count_after")
            checked_fingerprint(row, "active_layer_state_fingerprint_after_fnv1a64")
            for field in (
                "active_ice_mass_after_kg_m2", "active_depth_after_m",
                "active_density_after_kg_m3", "active_cold_after_j_m2",
                "surface_temperature_after_c",
            ):
                checked_number(row, field)
        else:
            if checked_string(row, "after_surface_applicability_reason") != "post_substep_no_resolved_surface" or any(required(row, field) is not None for field in active_after_fields):
                raise RuntimeError("terminal after-surface applicability mismatch")
    return {
        "shortwave": net_shortwave,
        "longwave": longwave["net"],
        "sensible": float(independently_reconstructed["sensible"]),
        "latent": float(independently_reconstructed["latent"]),
        "advected": advected,
        "external": external,
        "vapor_mass_flux": float(independently_reconstructed["mass"]),
    }


def validate_v6_row(
    row: dict[str, Any], operator: str, expected_site: str | None = None
) -> list[dict[str, Any]]:
    if row.get("schema") != "openwepp-r7h-direct-production-snow-trace-v6":
        raise RuntimeError(f"unknown enabled schema: {row.get('schema')}")
    companion = row.get("stage3_operator_reconciliation")
    if not isinstance(companion, dict) or set(companion) != {"schema_version", "hourly_status", "tuples"} or checked_int(companion, "schema_version") != 6:
        raise RuntimeError("missing schema-v6 reconciliation object")
    statuses = companion.get("hourly_status")
    tuples = companion.get("tuples")
    if not isinstance(statuses, list) or len(statuses) != 24 or not isinstance(tuples, list):
        raise RuntimeError("invalid reconciliation arrays")
    if expected_site is not None and not expected_site:
        raise RuntimeError("receipt-bound site identity is empty")
    if checked_int(row, "day_index") < 0 or checked_int(row, "lane_index") != 0:
        raise RuntimeError("top-level day/lane identity mismatch")
    if checked_string(row, "stage3_evaluation_operator_id") != operator:
        raise RuntimeError("top-level operator mismatch")
    for field in (
        "stage3_evaluation_source_fingerprint_fnv1a64",
        "stage3_evaluation_forcing_fingerprint_fnv1a64",
        "stage3_evaluation_geometry_fingerprint_fnv1a64",
    ):
        checked_fingerprint(row, field)
    if len(tuples) > 1_440:
        raise RuntimeError("tuple bound exceeded")
    expected_index = [0] * 24
    elapsed = [0.0] * 24
    previous_identity: tuple[int, int] | None = None
    fixed_options: tuple[Any, ...] | None = None
    previous_sequential: dict[str, Any] | None = None
    for item in tuples:
        if not isinstance(item, dict):
            raise RuntimeError("tuple is not an object")
        hour = checked_int(item, "hour_index")
        substep = checked_int(item, "substep_index")
        if not 0 <= hour < 24 or substep != expected_index[hour]:
            raise RuntimeError("tuple order mismatch")
        identity = (hour, substep)
        if previous_identity is not None and identity <= previous_identity:
            raise RuntimeError("tuple global order mismatch")
        previous_identity = identity
        start = checked_number(item, "elapsed_start_seconds")
        if start < 0.0 or not same_bits(start, elapsed[hour]):
            raise RuntimeError("tuple elapsed start is not exactly contiguous")
        if start + checked_number(item, "duration_seconds") > 3_600.0:
            raise RuntimeError("tuple exceeds hourly interval")
        for tuple_field, row_field in (
            ("source_fingerprint_fnv1a64", "stage3_evaluation_source_fingerprint_fnv1a64"),
            ("forcing_fingerprint_fnv1a64", "stage3_evaluation_forcing_fingerprint_fnv1a64"),
            ("geometry_fingerprint_fnv1a64", "stage3_evaluation_geometry_fingerprint_fnv1a64"),
        ):
            if checked_fingerprint(item, tuple_field) != checked_fingerprint(row, row_field):
                raise RuntimeError(f"tuple {tuple_field} custody mismatch")
        options = tuple(item.get(field) for field in FIXED_REFERENCE_FIELDS)
        if fixed_options is None:
            fixed_options = options
        elif options != fixed_options:
            raise RuntimeError("fixed daily option/forcing drift")
        reconstructed = validate_tuple(item, operator)
        item["_reconstructed"] = reconstructed
        if operator == "sequential_resolved_shadow_v1" and previous_sequential is not None:
            for before, prior_after in (
                ("total_layer_count_before", "total_layer_count_after"),
                ("total_layer_state_fingerprint_before_fnv1a64", "total_layer_state_fingerprint_after_fnv1a64"),
            ):
                if required(item, before) != required(previous_sequential, prior_after):
                    raise RuntimeError(f"sequential state continuity mismatch: {before}")
            for before, prior_after in (
                ("total_ice_mass_before_kg_m2", "total_ice_mass_after_kg_m2"),
                ("total_cold_before_j_m2", "total_cold_after_j_m2"),
            ):
                require_same_bits(
                    f"sequential state continuity {before}",
                    required(item, before),
                    required(previous_sequential, prior_after),
                )
            if checked_bool(previous_sequential, "after_surface_applicable") is not True:
                raise RuntimeError("sequential tuple follows terminal surface exhaustion")
            for before, prior_after in (
                ("active_layer_prefix_count_before", "active_layer_prefix_count_after"),
                ("active_layer_state_fingerprint_before_fnv1a64", "active_layer_state_fingerprint_after_fnv1a64"),
            ):
                if required(item, before) != required(previous_sequential, prior_after):
                    raise RuntimeError(f"sequential active continuity mismatch: {before}")
            for before, prior_after in (
                ("active_ice_mass_before_kg_m2", "active_ice_mass_after_kg_m2"),
                ("active_depth_before_m", "active_depth_after_m"),
                ("active_density_before_kg_m3", "active_density_after_kg_m3"),
                ("active_cold_before_j_m2", "active_cold_after_j_m2"),
                ("surface_temperature_before_c", "surface_temperature_after_c"),
            ):
                require_same_bits(
                    f"sequential active continuity {before}",
                    required(item, before),
                    required(previous_sequential, prior_after),
                )
        if operator == "sequential_resolved_shadow_v1":
            previous_sequential = item
        elapsed[hour] += checked_number(item, "duration_seconds")
        expected_index[hour] += 1
    for hour, status in enumerate(statuses):
        if not isinstance(status, dict) or set(status) != {"evaluated", "reason"} or checked_bool(status, "evaluated") != (expected_index[hour] > 0):
            raise RuntimeError("hourly status mismatch")
        expected_reasons = {"evaluated"} if expected_index[hour] else {
            "no_resolved_snow_at_day_start",
            "thin_pack_boundary_reached",
            "operator_not_selected",
        }
        if checked_string(status, "reason") not in expected_reasons:
            raise RuntimeError("hourly reason mismatch")
        if operator == "same_state_paired_carrier_v1" and expected_index[hour]:
            if expected_index[hour] != 1 or elapsed[hour] != 3_600.0:
                raise RuntimeError("same-state cadence mismatch")
        if operator == "sequential_resolved_shadow_v1" and expected_index[hour]:
            last = next(
                item
                for item in reversed(tuples)
                if checked_int(item, "hour_index") == hour
            )
            terminal = checked_bool(last, "after_surface_applicable") is False
            if elapsed[hour] != 3_600.0 and not terminal:
                raise RuntimeError("sequential evaluated hour has a dropped tail")
    return tuples


def dispatch_trace_row(
    row: dict[str, Any], operator: str, expected_site: str | None = None
) -> tuple[int, list[dict[str, Any]]]:
    schema = row.get("schema")
    if schema == "openwepp-r7h-direct-production-snow-trace-v6":
        return 6, validate_v6_row(row, operator, expected_site)
    if schema == "openwepp-r7h-direct-production-snow-trace-v5":
        if "stage3_operator_reconciliation" in row:
            raise RuntimeError("historical v5 row aliases schema-v6 companion")
        return 5, []
    if schema == "openwepp-r7h-direct-production-snow-trace-v4":
        if "stage3_operator_reconciliation" in row:
            raise RuntimeError("historical v4 row aliases schema-v6 companion")
        return 4, []
    raise RuntimeError(f"unknown enabled schema: {schema}")


def integrate_prefix(tuples: list[dict[str, Any]], seconds: float, field: str) -> float:
    remaining = seconds
    total = 0.0
    for row in tuples:
        if remaining <= 0.0:
            break
        duration = min(remaining, checked_number(row, "duration_seconds"))
        total += checked_number(row, field) * duration
        remaining -= duration
    if remaining > 1.0e-9:
        raise RuntimeError("tuple support does not cover requested prefix")
    return total


def integrate_reconstructed_prefix(
    tuples: list[dict[str, Any]], seconds: float, term: str
) -> float:
    remaining = seconds
    total = 0.0
    for row in tuples:
        if remaining <= 0.0:
            break
        duration = min(remaining, checked_number(row, "duration_seconds"))
        reconstructed = required(row, "_reconstructed")
        if not isinstance(reconstructed, dict):
            raise RuntimeError("missing independent tuple reconstruction")
        total += checked_number(reconstructed, term) * duration
        remaining -= duration
    if remaining > 1.0e-9:
        raise RuntimeError("tuple support does not cover reconstructed prefix")
    return total


def integrate_amount_prefix(
    tuples: list[dict[str, Any]], seconds: float, field: str
) -> float:
    remaining = seconds
    total = 0.0
    for row in tuples:
        if remaining <= 0.0:
            break
        tuple_duration = checked_number(row, "duration_seconds")
        used = min(remaining, tuple_duration)
        total += checked_number(row, field) * used / tuple_duration
        remaining -= used
    if remaining > 1.0e-9:
        raise RuntimeError("tuple support does not cover amount prefix")
    return total


def frozen_active_flux(same: dict[str, Any], first_q: dict[str, Any]) -> dict[str, float]:
    surface_c = checked_number(first_q, "surface_temperature_before_c")
    shortwave = checked_number(same, "incoming_shortwave_w_m2") * (
        1.0 - checked_number(first_q, "snow_albedo_fraction")
    )
    reference = dict(same)
    for field in FIXED_REFERENCE_FIELDS:
        reference[field] = first_q[field]
    reference["actual_vapor_pressure_pa"] = saturation_vapor_pressure(
        checked_number(first_q, "dewpoint_c")
    )
    longwave = reconstruct_longwave(reference, surface_c)["net"]
    turbulent = monin_obukhov(
        air_temperature_c=checked_number(same, "air_temperature_c"),
        surface_temperature_c=surface_c,
        air_vapor_pressure_pa=checked_number(reference, "actual_vapor_pressure_pa"),
        surface_vapor_pressure_pa=saturation_vapor_pressure(surface_c),
        air_pressure_pa=checked_number(first_q, "air_pressure_pa"),
        wind_speed_m_s=checked_number(first_q, "wind_speed_m_s"),
        z_t=checked_number(first_q, "air_temperature_height_m"),
        z_q=checked_number(first_q, "vapor_pressure_height_m"),
        z_u=checked_number(first_q, "wind_speed_height_m"),
        z_0=checked_number(first_q, "aerodynamic_roughness_length_m"),
        max_iterations=int(first_q["turbulent_max_iterations"]),
        tolerance=checked_number(first_q, "turbulent_convergence_tolerance"),
    )
    advected = (
        checked_number(same, "rain_specific_heat_j_kg_k")
        * checked_number(same, "rain_mass_flux_kg_m2_s")
        * (checked_number(same, "rain_temperature_c") - surface_c)
        + checked_number(same, "snow_specific_heat_j_kg_k")
        * checked_number(same, "snow_mass_flux_kg_m2_s")
        * (checked_number(same, "snow_temperature_c") - surface_c)
    )
    result = {
        "shortwave": shortwave,
        "longwave": longwave,
        "sensible": turbulent["sensible"],
        "latent": turbulent["latent"],
        "advected": advected,
    }
    result["external"] = sum(result.values())
    result["vapor_mass_flux"] = turbulent["mass"]
    return result


def saturation_vapor_pressure(temperature_c: float) -> float:
    kelvin = temperature_c + FREEZE_K
    if temperature_c <= 0.0:
        exponent = (
            -9.09718 * (FREEZE_K / kelvin - 1.0)
            - 3.56654 * math.log(FREEZE_K / kelvin) / math.log(10.0)
            + 0.876793 * (1.0 - kelvin / FREEZE_K)
            + math.log10(6.1071)
        )
        return 10.0**exponent * 100.0
    boiling = 373.15
    sea_level = 1.013246e5
    exponent = (
        -7.90298 * (boiling / kelvin - 1.0)
        + 5.02808 * math.log(boiling / kelvin) / math.log(10.0)
        - 1.3816e-7 * (10.0 ** (11.344 * (1.0 - kelvin / boiling)) - 1.0)
        + 8.1328e-3 * (10.0 ** (-3.49149 * (boiling / kelvin - 1.0)) - 1.0)
        + math.log10(sea_level)
    )
    return 10.0**exponent


def reconstruct_longwave(row: dict[str, Any], surface_temperature_c: float) -> dict[str, float]:
    if row.get("longwave_model_id") != LONGWAVE_MODEL:
        raise RuntimeError("unexpected longwave model")
    air_k = checked_number(row, "air_temperature_c") + FREEZE_K
    surface_k = surface_temperature_c + FREEZE_K
    vapor_kpa = checked_number(row, "actual_vapor_pressure_pa") / 1_000.0
    precipitable_water = 4_650.0 * vapor_kpa / air_k
    clear_longwave = (
        59.38
        + 113.7 * (air_k / FREEZE_K) ** 6
        + 96.96 * math.sqrt(precipitable_water / 25.0)
    )
    blackbody_air = SIGMA * air_k**4
    clear_emissivity = clear_longwave / blackbody_air
    clearness = checked_number(row, "daily_solar_radiation_mj_m2") / checked_number(
        row, "daily_extraterrestrial_radiation_mj_m2"
    )
    cloud = min(1.0, max(0.0, (0.80 - clearness) / (0.80 - 0.15)))
    all_sky_emissivity = (1.0 - 0.84 * cloud) * clear_emissivity + 0.84 * cloud
    atmospheric = all_sky_emissivity * blackbody_air
    sky_view = (1.0 - checked_number(row, "canopy_cover_fraction")) ** 1.6
    canopy = blackbody_air
    subcanopy = sky_view * atmospheric + (1.0 - sky_view) * canopy
    outgoing = SIGMA * surface_k**4
    return {
        "cloud": cloud,
        "sky_view": sky_view,
        "atmospheric": atmospheric,
        "canopy": canopy,
        "subcanopy": subcanopy,
        "outgoing": outgoing,
        "net": subcanopy - outgoing,
    }


def monin_obukhov(**values: float) -> dict[str, Any]:
    wind = values["wind_speed_m_s"]
    if wind == 0.0:
        return {
            "sensible": 0.0,
            "latent": 0.0,
            "mass": 0.0,
            "iterations": 0,
            "obukhov": None,
            "status": "zero_wind",
            "class": "zero_wind",
            "psi_momentum": 0.0,
            "psi_sensible": 0.0,
            "psi_latent": 0.0,
            "friction": 0.0,
            "displacement": None,
            "log_momentum": None,
            "log_sensible": None,
            "log_latent": None,
            "sensible_exchange": None,
            "latent_exchange": None,
            "density": None,
            "potential_temperature": None,
            "surface_temperature_k": None,
            "q_air": None,
            "q_surface": None,
            "latent_heat": None,
        }
    z0 = values["z_0"]
    displacement = 2.0 * 7.35 * z0 / 3.0
    logs = [math.log((values[key] - displacement) / z0) for key in ("z_u", "z_t", "z_q")]
    air_k = values["air_temperature_c"] + FREEZE_K
    surface_k = values["surface_temperature_c"] + FREEZE_K
    potential_k = air_k + GRAVITY / CP_AIR * values["z_t"]
    q_air = values["air_vapor_pressure_pa"] * MOLAR_WATER / (
        MOLAR_DRY * values["air_pressure_pa"]
        + values["air_vapor_pressure_pa"] * (MOLAR_WATER - MOLAR_DRY)
    )
    q_surface = values["surface_vapor_pressure_pa"] * MOLAR_WATER / (
        MOLAR_DRY * values["air_pressure_pa"]
        + values["surface_vapor_pressure_pa"] * (MOLAR_WATER - MOLAR_DRY)
    )
    geometric_temperature = math.sqrt(potential_k * surface_k)
    geometric_vapor = math.sqrt(
        values["air_vapor_pressure_pa"] * values["surface_vapor_pressure_pa"]
    )
    virtual_temperature = geometric_temperature / (
        1.0
        - (1.0 - MOLAR_WATER / MOLAR_DRY)
        * geometric_vapor
        / values["air_pressure_pa"]
    )
    density = values["air_pressure_pa"] * MOLAR_DRY / (
        UNIVERSAL_GAS * virtual_temperature
    )
    latent_heat = 2.5e6 - 2_955.73 * (surface_k - FREEZE_K)
    if values["surface_temperature_c"] <= 0.0:
        latent_heat += 333_600.0 + 166.67 * (FREEZE_K - surface_k)
    corrections = [0.0, 0.0, 0.0]

    def recompute() -> tuple[float, float, float, float, float]:
        friction = VON_KARMAN * wind / (logs[0] - corrections[0])
        factor = VON_KARMAN * friction * density
        latent_exchange = VON_KARMAN * friction / (logs[2] - corrections[2])
        sensible_exchange = VON_KARMAN * friction / (logs[1] - corrections[1])
        mass = (q_air - q_surface) * factor / (logs[2] - corrections[2])
        sensible = (potential_k - surface_k) * factor * CP_AIR / (logs[1] - corrections[1])
        return friction, mass, sensible, sensible_exchange, latent_exchange

    def finish(iterations: int, obukhov: float | None, status: str, stability: str) -> dict[str, Any]:
        return {
            "sensible": sensible,
            "latent": latent_heat * mass,
            "mass": mass,
            "iterations": iterations,
            "obukhov": obukhov,
            "status": status,
            "class": stability,
            "psi_momentum": corrections[0],
            "psi_sensible": corrections[1],
            "psi_latent": corrections[2],
            "friction": friction,
            "displacement": displacement,
            "log_momentum": logs[0],
            "log_sensible": logs[1],
            "log_latent": logs[2],
            "sensible_exchange": sensible_exchange,
            "latent_exchange": latent_exchange,
            "density": density,
            "potential_temperature": potential_k,
            "surface_temperature_k": surface_k,
            "q_air": q_air,
            "q_surface": q_surface,
            "latent_heat": latent_heat,
        }

    def correction_class() -> str:
        if not all(math.isfinite(value) for value in corrections):
            raise RuntimeError("nonfinite stability correction")
        if all(value == 0.0 for value in corrections):
            return "neutral"
        if all(value <= 0.0 for value in corrections):
            return "stable"
        if all(value >= 0.0 for value in corrections):
            return "unstable"
        raise RuntimeError("mixed-sign stability corrections")

    friction, mass, sensible, sensible_exchange, latent_exchange = recompute()
    if abs(potential_k - surface_k) <= sys.float_info.epsilon:
        return finish(0, None, "initial_potential_temperature_neutral", "neutral")
    length = math.inf
    for iteration in range(1, int(values["max_iterations"]) + 1):
        previous = length
        buoyancy = sensible / (potential_k * CP_AIR) + 0.61 * mass
        if buoyancy == 0.0:
            return finish(iteration, None, "iterative_zero_buoyancy", correction_class())
        length = friction**3 * density / (VON_KARMAN * GRAVITY * buoyancy)
        if not math.isfinite(length) or length == 0.0:
            return finish(iteration, None, "iterative_invalid_obukhov", "indeterminate_obukhov")
        corrections = [psi(values[key] / length, kind) for key, kind in (("z_u", "momentum"), ("z_t", "heat"), ("z_q", "heat"))]
        friction, mass, sensible, sensible_exchange, latent_exchange = recompute()
        delta = previous - length
        if abs(delta) <= values["tolerance"] or abs(delta / length) <= values["tolerance"]:
            if length > 0.0:
                return finish(iteration, length, "converged_stable", "stable")
            return finish(iteration, length, "converged_unstable", "unstable")
    raise RuntimeError("consumer frozen-active turbulent solver did not converge")


def psi(zeta: float, kind: str) -> float:
    if zeta > 0.0:
        return -5.2 * min(zeta, 1.0)
    if zeta < 0.0:
        x = (1.0 - 16.0 * zeta) ** 0.25
        if kind == "momentum":
            return 2.0 * math.log((1.0 + x) / 2.0) + math.log((1.0 + x * x) / 2.0) - 2.0 * math.atan(x) + math.pi / 2.0
        return 2.0 * math.log((1.0 + x * x) / 2.0)
    return 0.0


def parse_trace(
    path: Path, dates: list[dt.date], operator: str, expected_site: str
) -> dict[dt.date, dict[str, Any]]:
    result: dict[dt.date, dict[str, Any]] = {}
    with path.open(encoding="utf-8") as handle:
        for index, line in enumerate(handle):
            if index >= len(dates):
                raise RuntimeError("trace has more rows than climate")
            row = json.loads(line)
            if checked_int(row, "day_index") != index or checked_int(row, "lane_index") != 0:
                raise RuntimeError("trace identity mismatch")
            version, tuples = dispatch_trace_row(row, operator, expected_site)
            if version != 6:
                raise RuntimeError(f"current operator trace requires schema v6, got v{version}")
            row["_tuples"] = tuples
            row["_serialized_bytes"] = len(line.encode("utf-8"))
            result[dates[index]] = row
    if len(result) != len(dates):
        raise RuntimeError("trace/climate row-count mismatch")
    return result


def require_trace_path_custody(path: Path, site: str, lane: str) -> None:
    expected_parent = (OUTPUT / "runs" / site / lane).resolve()
    if path.resolve().parent != expected_parent:
        raise RuntimeError(
            f"trace receipt-path site/lane custody mismatch: {path} != {expected_parent}"
        )


def tuples_by_hour(rows: list[dict[str, Any]]) -> list[list[dict[str, Any]]]:
    grouped = [[] for _ in range(24)]
    for row in rows:
        grouped[int(row["hour_index"])].append(row)
    return grouped


def projection_differs(first_s: dict[str, Any], first_q: dict[str, Any]) -> bool:
    for field in (
        "effective_input_fingerprint_fnv1a64",
        "active_layer_prefix_count_before",
        "active_layer_state_fingerprint_before_fnv1a64",
    ):
        if first_s.get(field) != first_q.get(field):
            return True
    return any(
        not same_bits(first_s.get(field), first_q.get(field))
        for field in (
            "active_ice_mass_before_kg_m2",
            "active_depth_before_m",
            "active_density_before_kg_m3",
            "active_cold_before_j_m2",
            "surface_temperature_before_c",
        )
    )


def support_window_metrics(row: dict[str, Any]) -> dict[str, float | bool | None]:
    omitted_magnitude = sum(
        abs(checked_number(row, f"{operator}_omitted_{term}_j_m2"))
        for operator in ("S", "Q")
        for term in TERMS
    )
    all_magnitude = sum(
        abs(checked_number(row, f"{operator}_all_{term}_j_m2"))
        for operator in ("S", "Q")
        for term in TERMS
    )
    common_delta = checked_number(row, "Q_j_m2") - checked_number(row, "S_j_m2")
    all_delta = checked_number(row, "Q_all_j_m2") - checked_number(row, "S_all_j_m2")
    return {
        "support_omitted_magnitude_j_m2": omitted_magnitude,
        "support_all_evaluated_magnitude_j_m2": all_magnitude,
        "support_omission_ratio": (
            None if all_magnitude == 0.0 else omitted_magnitude / all_magnitude
        ),
        "common_operator_delta_j_m2": common_delta,
        "all_operator_delta_j_m2": all_delta,
        "support_delta_sign_changed": (
            common_delta < -ZERO_J_M2 and all_delta > ZERO_J_M2
        )
        or (common_delta > ZERO_J_M2 and all_delta < -ZERO_J_M2),
    }


def site_support_metrics(
    eligible: list[dict[str, Any]],
) -> dict[str, float | bool | None]:
    common_delta = statistics.median(
        checked_number(row, "common_operator_delta_j_m2") for row in eligible
    )
    all_delta = statistics.median(
        checked_number(row, "all_operator_delta_j_m2") for row in eligible
    )
    ratios = [
        checked_number(row, "support_omission_ratio")
        for row in eligible
        if required(row, "support_omission_ratio") is not None
    ]
    return {
        "common_operator_delta_median_j_m2": common_delta,
        "all_operator_delta_median_j_m2": all_delta,
        "support_omission_ratio": statistics.median(ratios) if ratios else None,
        "support_delta_sign_changed": (
            common_delta < -ZERO_J_M2 and all_delta > ZERO_J_M2
        )
        or (common_delta > ZERO_J_M2 and all_delta < -ZERO_J_M2),
    }


def historical_predecessor_windows(
    trace: Path,
    dates: list[dt.date],
    peaks: dict[int, tuple[dt.date, float]],
) -> dict[int, float]:
    daily: dict[dt.date, float] = {}
    with trace.open(encoding="utf-8") as handle:
        for index, line in enumerate(handle):
            if index >= len(dates):
                raise RuntimeError("historical predecessor trace has extra rows")
            row = json.loads(line)
            version, _ = dispatch_trace_row(row, "historical")
            if version not in (4, 5) or row.get("day_index") != index:
                raise RuntimeError("historical predecessor schema/identity mismatch")
            daily[dates[index]] = checked_number(row, "stage3_shadow_complete_energy_j_m2")
    if len(daily) != len(dates):
        raise RuntimeError("historical predecessor trace row count mismatch")
    return {
        year: sum(
            daily[stamp]
            for stamp in dates
            if dt.date(year - 1, 10, 1) <= stamp <= peak
        )
        for year, (peak, _) in peaks.items()
        if 1990 <= year <= 2024
    }


def reconcile_site(
    site: str,
    fixture: Path,
    observation: Path,
    paired_trace: Path,
    sequential_trace: Path,
    carrier: Any,
    predecessor_windows: dict[int, float] | None = None,
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    dates = carrier.climate_dates(carrier.climate_file(fixture))
    require_trace_path_custody(paired_trace, site, "paired")
    require_trace_path_custody(sequential_trace, site, "sequential")
    paired = parse_trace(
        paired_trace, dates, "same_state_paired_carrier_v1", site
    )
    sequential = parse_trace(
        sequential_trace, dates, "sequential_resolved_shadow_v1", site
    )
    peaks, _ = carrier.observed_peaks(observation)
    daily: dict[dt.date, dict[str, float]] = {}
    projection_difference = False
    first_projection_delta_samples: list[dict[str, float]] = []
    inventories: dict[str, list[dict[str, Any]]] = {
        "unmatched_hours": [],
        "zero_support_hours": [],
        "partial_support_hours": [],
        "non_evaluated_hours": [],
        "ineligible_water_years": [],
        "censored_water_years": [],
    }
    for stamp in dates:
        p = paired[stamp]
        q = sequential[stamp]
        for field in (
            "stage3_evaluation_source_fingerprint_fnv1a64",
            "stage3_evaluation_forcing_fingerprint_fnv1a64",
            "stage3_evaluation_geometry_fingerprint_fnv1a64",
        ):
            if p.get(field) != q.get(field):
                raise RuntimeError(f"joined fingerprint mismatch {site} {stamp} {field}")
        s_hours = tuples_by_hour(p["_tuples"])
        q_hours = tuples_by_hour(q["_tuples"])
        sums = {name: 0.0 for name in (
            "S", "F", "Q", "S_all", "Q_all", "legacy_Q", "legacy_Q_all",
            "Q_internal_conduction", "Q_internal_conduction_all",
            *(f"S_{term}" for term in TERMS),
            *(f"F_{term}" for term in TERMS),
            *(f"Q_{term}" for term in TERMS),
            *(f"S_all_{term}" for term in TERMS),
            *(f"Q_all_{term}" for term in TERMS),
            *(f"S_omitted_{term}" for term in TERMS),
            *(f"Q_omitted_{term}" for term in TERMS),
            "S_vapor_mass", "F_vapor_mass", "Q_vapor_mass",
        )}
        sums["support_seconds"] = 0.0
        first_q = next((row for hour in q_hours for row in hour), None)
        if first_q is not None:
            first_s = next((row for hour in s_hours for row in hour), None)
            if first_s is not None:
                for field in FIXED_REFERENCE_FIELDS:
                    left, right = first_s.get(field), first_q.get(field)
                    if isinstance(left, float) or isinstance(right, float):
                        require_same_bits(f"joined fixed {field}", left, right)
                    elif left != right:
                        raise RuntimeError(f"joined fixed {field} mismatch")
                for field in (
                    "snow_albedo_fraction",
                    "snow_albedo_source_id",
                    "snow_albedo_model_id",
                    "snow_albedo_accumulated_positive_temperature_c_day",
                ):
                    left, right = first_s.get(field), first_q.get(field)
                    if isinstance(left, (int, float)) or isinstance(right, (int, float)):
                        require_same_bits(f"joined albedo {field}", left, right)
                    elif left != right:
                        raise RuntimeError(f"joined albedo {field} mismatch")
                if projection_differs(first_s, first_q):
                    projection_difference = True
                    first_hour = int(first_q["hour_index"])
                    if s_hours[first_hour]:
                        frozen_first = frozen_active_flux(s_hours[first_hour][0], first_q)
                        same_reconstructed = required(
                            s_hours[first_hour][0], "_reconstructed"
                        )
                        first_projection_delta_samples.append({
                            term: frozen_first[term]
                            - checked_number(same_reconstructed, term)
                            for term in TERMS
                        })
        for hour in range(24):
            s_support = sum(checked_number(row, "duration_seconds") for row in s_hours[hour])
            q_support = sum(checked_number(row, "duration_seconds") for row in q_hours[hour])
            for operator_name, trace_row, support in (
                ("same_state", p, s_support),
                ("sequential", q, q_support),
            ):
                status = trace_row["stage3_operator_reconciliation"]["hourly_status"][hour]
                if support == 0.0:
                    inventories["non_evaluated_hours"].append(
                        {
                            "date": stamp.isoformat(),
                            "hour": hour,
                            "operator": operator_name,
                            "reason": checked_string(status, "reason"),
                        }
                    )
            for rows, prefix in ((s_hours[hour], "S"), (q_hours[hour], "Q")):
                evaluated = sum(
                    checked_number(row, "duration_seconds") for row in rows
                )
                full_external = (
                    integrate_reconstructed_prefix(rows, evaluated, "external")
                    if rows
                    else 0.0
                )
                sums[f"{prefix}_all"] += full_external
                for term in TERMS:
                    full_term = (
                        integrate_reconstructed_prefix(rows, evaluated, term)
                        if rows
                        else 0.0
                    )
                    sums[f"{prefix}_all_{term}"] = (
                        sums.get(f"{prefix}_all_{term}", 0.0) + full_term
                    )
            sums["legacy_Q_all"] += (
                integrate_reconstructed_prefix(q_hours[hour], q_support, "external")
                + integrate_amount_prefix(
                    q_hours[hour], q_support, "internal_active_lower_conduction_j_m2"
                )
                if q_hours[hour]
                else 0.0
            )
            sums["Q_internal_conduction_all"] += (
                integrate_amount_prefix(
                    q_hours[hour], q_support, "internal_active_lower_conduction_j_m2"
                )
                if q_hours[hour]
                else 0.0
            )
            if not s_hours[hour] or not q_hours[hour] or first_q is None:
                if bool(s_hours[hour]) != bool(q_hours[hour]):
                    inventories["unmatched_hours"].append({"date": stamp.isoformat(), "hour": hour, "S_seconds": s_support, "Q_seconds": q_support})
                else:
                    inventories["zero_support_hours"].append({"date": stamp.isoformat(), "hour": hour})
                for rows, prefix in ((s_hours[hour], "S"), (q_hours[hour], "Q")):
                    evaluated = sum(
                        checked_number(row, "duration_seconds") for row in rows
                    )
                    for term in TERMS:
                        omitted = (
                            integrate_reconstructed_prefix(rows, evaluated, term)
                            if rows
                            else 0.0
                        )
                        sums[f"{prefix}_omitted_{term}"] = (
                            sums.get(f"{prefix}_omitted_{term}", 0.0) + omitted
                        )
                continue
            s = s_hours[hour][0]
            common = min(s_support, q_support, 3_600.0)
            if common < 3_600.0:
                inventories["partial_support_hours"].append(
                    {
                        "date": stamp.isoformat(),
                        "hour": hour,
                        "common_seconds": common,
                        "S_seconds": s_support,
                        "Q_seconds": q_support,
                        "same_state_status_reason": checked_string(
                            p["stage3_operator_reconciliation"]["hourly_status"][hour],
                            "reason",
                        ),
                        "sequential_status_reason": checked_string(
                            q["stage3_operator_reconciliation"]["hourly_status"][hour],
                            "reason",
                        ),
                        "sequential_tail_disposition": (
                            "terminal_surface_exhaustion"
                            if q_hours[hour]
                            and checked_bool(q_hours[hour][-1], "after_surface_applicable")
                            is False
                            else "full_hour"
                        ),
                    }
                )
            sums["support_seconds"] += common
            frozen = frozen_active_flux(s, first_q)
            s_energy = integrate_reconstructed_prefix(s_hours[hour], common, "external")
            q_energy = integrate_reconstructed_prefix(q_hours[hour], common, "external")
            f_energy = frozen["external"] * common
            s_shortwave = integrate_reconstructed_prefix(
                s_hours[hour], common, "shortwave"
            )
            q_shortwave = integrate_reconstructed_prefix(
                q_hours[hour], common, "shortwave"
            )
            close(
                "sequential shortwave invariance",
                q_shortwave,
                s_shortwave,
                1.0e-6,
            )
            sums["S"] += s_energy
            sums["F"] += f_energy
            sums["Q"] += q_energy
            sums["legacy_Q"] += integrate_reconstructed_prefix(
                q_hours[hour], common, "external"
            ) + integrate_amount_prefix(
                q_hours[hour], common, "internal_active_lower_conduction_j_m2"
            )
            sums["Q_internal_conduction"] += integrate_amount_prefix(
                q_hours[hour], common, "internal_active_lower_conduction_j_m2"
            )
            for term in TERMS:
                sums[f"S_{term}"] += integrate_reconstructed_prefix(
                    s_hours[hour], common, term
                )
                sums[f"Q_{term}"] += integrate_reconstructed_prefix(
                    q_hours[hour], common, term
                )
                sums[f"F_{term}"] += frozen[term] * common
            sums["S_vapor_mass"] += integrate_reconstructed_prefix(
                s_hours[hour], common, "vapor_mass_flux"
            )
            sums["Q_vapor_mass"] += integrate_reconstructed_prefix(
                q_hours[hour], common, "vapor_mass_flux"
            )
            sums["F_vapor_mass"] += frozen["vapor_mass_flux"] * common
            close("frozen shortwave invariance", frozen["shortwave"] * common, s_shortwave, 1.0e-6)
            for rows, evaluated, prefix in (
                (s_hours[hour], s_support, "S"),
                (q_hours[hour], q_support, "Q"),
            ):
                if evaluated > common:
                    for term in TERMS:
                        omitted = integrate_reconstructed_prefix(
                            rows, evaluated, term
                        ) - integrate_reconstructed_prefix(rows, common, term)
                        sums[f"{prefix}_omitted_{term}"] = (
                            sums.get(f"{prefix}_omitted_{term}", 0.0) + omitted
                        )
        daily[stamp] = sums
    annual: list[dict[str, Any]] = []
    for year, (peak, _) in sorted(peaks.items()):
        if year == 2025:
            start = dt.date(year - 1, 10, 1)
            censored_window = [stamp for stamp in dates if start <= stamp <= peak]
            censored_support = sum(
                daily[stamp]["support_seconds"] for stamp in censored_window
            )
            inventories["censored_water_years"].append(
                {
                    "water_year": year,
                    "reason": "right_censored",
                    "window_days": len(censored_window),
                    "support_seconds": censored_support,
                    "coverage_fraction": (
                        censored_support / (len(censored_window) * 86_400.0)
                        if censored_window
                        else 0.0
                    ),
                    "evaluated_days": sum(
                        daily[stamp]["support_seconds"] > 0.0
                        for stamp in censored_window
                    ),
                }
            )
            continue
        start = dt.date(year - 1, 10, 1)
        window = [stamp for stamp in dates if start <= stamp <= peak]
        if not window:
            continue
        row: dict[str, Any] = {"site": site, "water_year": year, "window_days": len(window)}
        for field in next(iter(daily.values())):
            if field == "support_seconds":
                continue
            suffix = "kg_m2" if field.endswith("vapor_mass") else "j_m2"
            row[f"{field}_{suffix}"] = sum(daily[stamp][field] for stamp in window)
        row["support_seconds"] = sum(
            daily[stamp]["support_seconds"] for stamp in window
        )
        row["coverage_fraction"] = row["support_seconds"] / (len(window) * 86_400.0)
        row["delta_projection_j_m2"] = row["F_j_m2"] - row["S_j_m2"]
        row["delta_evolution_j_m2"] = row["Q_j_m2"] - row["F_j_m2"]
        close(
            "common legacy bridge",
            row["legacy_Q_j_m2"],
            row["Q_j_m2"] + row["Q_internal_conduction_j_m2"],
            1.0e-6,
        )
        close(
            "all-support legacy bridge",
            row["legacy_Q_all_j_m2"],
            row["Q_all_j_m2"] + row["Q_internal_conduction_all_j_m2"],
            1.0e-6,
        )
        close(
            "projection delta closure",
            row["delta_projection_j_m2"],
            row["F_j_m2"] - row["S_j_m2"],
            1.0e-6,
        )
        close(
            "evolution delta closure",
            row["delta_evolution_j_m2"],
            row["Q_j_m2"] - row["F_j_m2"],
            1.0e-6,
        )
        for term in TERMS:
            row[f"delta_projection_{term}_j_m2"] = row[f"F_{term}_j_m2"] - row[f"S_{term}_j_m2"]
            row[f"delta_evolution_{term}_j_m2"] = row[f"Q_{term}_j_m2"] - row[f"F_{term}_j_m2"]
        row["delta_projection_vapor_mass_kg_m2"] = (
            row["F_vapor_mass_kg_m2"] - row["S_vapor_mass_kg_m2"]
        )
        row["delta_evolution_vapor_mass_kg_m2"] = (
            row["Q_vapor_mass_kg_m2"] - row["F_vapor_mass_kg_m2"]
        )
        projection_terms = sum(
            row[f"delta_projection_{term}_j_m2"] for term in TERMS
        )
        evolution_terms = sum(
            row[f"delta_evolution_{term}_j_m2"] for term in TERMS
        )
        close(
            "projection term-delta closure",
            row["delta_projection_j_m2"],
            projection_terms,
            1.0e-6,
        )
        close(
            "evolution term-delta closure",
            row["delta_evolution_j_m2"],
            evolution_terms,
            1.0e-6,
        )
        close(
            "projection shortwave invariance",
            row["delta_projection_shortwave_j_m2"],
            0.0,
            1.0e-6,
        )
        close(
            "evolution shortwave invariance",
            row["delta_evolution_shortwave_j_m2"],
            0.0,
            1.0e-6,
        )
        row["evaluated_days"] = sum(daily[stamp]["support_seconds"] > 0.0 for stamp in window)
        row["screen_eligible"] = row["coverage_fraction"] >= 0.25 and row["evaluated_days"] >= 30
        row.update(support_window_metrics(row))
        if predecessor_windows is not None:
            predecessor = predecessor_windows.get(year)
            if predecessor is None:
                raise RuntimeError(f"missing predecessor window {year}")
            row["predecessor_legacy_j_m2"] = predecessor
            tolerance = max(1.0e-6, 1.0e-12 * (abs(predecessor) + abs(row["legacy_Q_all_j_m2"])))
            row["predecessor_bridge_pass"] = abs(row["legacy_Q_all_j_m2"] - predecessor) <= tolerance
        if not row["screen_eligible"]:
            inventories["ineligible_water_years"].append(
                {
                    "water_year": year,
                    "coverage_fraction": row["coverage_fraction"],
                    "evaluated_days": row["evaluated_days"],
                    "support_seconds": row["support_seconds"],
                    "reason": "coverage_or_evaluated_day_threshold",
                }
            )
        annual.append(row)
    eligible = [row for row in annual if row["screen_eligible"]]
    if not eligible:
        raise RuntimeError(f"no screen-eligible windows for {site}")
    medians = {
        field: statistics.median(float(row[field]) for row in eligible)
        for field in ("S_j_m2", "F_j_m2", "Q_j_m2", "S_all_j_m2", "Q_all_j_m2", "legacy_Q_j_m2", "legacy_Q_all_j_m2")
    }
    support_summary = site_support_metrics(eligible)
    bridge_pass = predecessor_windows is None or all(row.get("predecessor_bridge_pass") is True for row in annual)
    return annual, {
        "site": site,
        "sample_count": len(annual),
        "medians_j_m2": medians,
        "delta_projection_j_m2": statistics.median(
            row["delta_projection_j_m2"] for row in eligible
        ),
        "delta_evolution_j_m2": statistics.median(
            row["delta_evolution_j_m2"] for row in eligible
        ),
        "term_delta_medians_j_m2": {
            term: {
                "projection": statistics.median(row[f"delta_projection_{term}_j_m2"] for row in eligible),
                "evolution": statistics.median(row[f"delta_evolution_{term}_j_m2"] for row in eligible),
            }
            for term in TERMS
        },
        "initial_projection_difference_observed": projection_difference,
        "first_projection_term_delta_median_w_m2": {
            term: statistics.median(sample[term] for sample in first_projection_delta_samples)
            for term in TERMS
        } if first_projection_delta_samples else None,
        "eligible_sample_count": len(eligible),
        "vapor_mass_delta_medians_kg_m2": {
            "projection": statistics.median(
                row["delta_projection_vapor_mass_kg_m2"] for row in eligible
            ),
            "evolution": statistics.median(
                row["delta_evolution_vapor_mass_kg_m2"] for row in eligible
            ),
        },
        "support_omission_ratio": support_summary["support_omission_ratio"],
        "support_delta_sign_changed": support_summary["support_delta_sign_changed"],
        "common_operator_delta_median_j_m2": support_summary[
            "common_operator_delta_median_j_m2"
        ],
        "all_operator_delta_median_j_m2": support_summary[
            "all_operator_delta_median_j_m2"
        ],
        "predecessor_bridge_pass": bridge_pass,
        "lineage_identity_pass": True,
        "reconstruction_closure_pass": True,
        "delta_closure_pass": True,
        "shortwave_invariance_pass": True,
        "inventories": inventories,
        "maximum_daily_tuple_count": {
            "same_state": max((len(row["_tuples"]) for row in paired.values()), default=0),
            "sequential": max((len(row["_tuples"]) for row in sequential.values()), default=0),
        },
        "maximum_serialized_row_bytes": {
            "same_state": max((int(row["_serialized_bytes"]) for row in paired.values()), default=0),
            "sequential": max((int(row["_serialized_bytes"]) for row in sequential.values()), default=0),
        },
    }


def classify(snowbird: dict[str, Any]) -> list[str]:
    medians = snowbird["medians_j_m2"]
    classes: list[str] = []
    evidence_ok = all(
        snowbird.get(field) is True
        for field in (
            "lineage_identity_pass",
            "reconstruction_closure_pass",
            "delta_closure_pass",
            "shortwave_invariance_pass",
        )
    )
    if not evidence_ok:
        return ["LINEAGE_OR_IDENTITY_FAILURE"]
    predecessor_ok = (
        snowbird.get("predecessor_bridge_pass") is True
        and snowbird.get("sample_count") == 35
        and snowbird.get("eligible_sample_count") == 35
        and abs(medians["legacy_Q_all_j_m2"] / 1.0e6 - PREDECESSOR_MJ_M2) <= PREDECESSOR_TOL_MJ_M2
    )
    if not predecessor_ok:
        classes.append("PREDECESSOR_NOT_REPRODUCED")
    if predecessor_ok and medians["legacy_Q_all_j_m2"] > ZERO_J_M2 and medians["Q_all_j_m2"] <= 0.0:
        classes.append("LEGACY_ESTIMAND_INTERNAL_CONDUCTION_SIGN_DIFFERENCE")
    if snowbird["initial_projection_difference_observed"]:
        classes.append("INITIAL_CONTROL_VOLUME_PROJECTION_DIFFERENCE")
    if predecessor_ok and medians["S_j_m2"] < -ZERO_J_M2 and medians["F_j_m2"] > ZERO_J_M2 and medians["Q_j_m2"] > ZERO_J_M2:
        classes.append("INITIAL_CONTROL_VOLUME_PROJECTION_RECONCILES_SIGN_CONTRADICTION")
    if predecessor_ok and medians["S_j_m2"] < -ZERO_J_M2 and medians["F_j_m2"] < -ZERO_J_M2 and medians["Q_j_m2"] > ZERO_J_M2:
        classes.append("STATE_EVOLUTION_RECONCILES_SIGN_CONTRADICTION")
    if snowbird["support_delta_sign_changed"] or (snowbird["support_omission_ratio"] is not None and snowbird["support_omission_ratio"] > 0.05):
        classes.append("SUPPORT_CENSORING_MATERIALLY_CONTRIBUTES")
    return classes or ["MULTIFACTOR_UNRESOLVED"]


def execute_lane(site: str, fixture: Path, lane: str, selectors: dict[str, str], carrier: Any, w1: Any) -> dict[str, Any]:
    run_dir = OUTPUT / "runs" / site / lane
    run_dir.mkdir(parents=True)
    stem = f"{site}-operator-reconciliation"
    runfile = run_dir / f"{stem}.run"
    source_stem = w1.eb04r.legacy.observed_harness.discover_run_stem(fixture)
    w1.eb04r.legacy.observed_harness.write_runfile(runfile, fixture, source_stem, run_dir, stem)
    consumer = carrier.validate_runfile_consumer(runfile, carrier.climate_file(fixture))
    command = w1.eb04r.legacy.observed_harness.cli_command(BINARY, fixture, runfile, run_dir, "direct-production-executor")
    trace = run_dir / f"{stem}.snow.jsonl" if lane != "control" else None
    effective = dict(selectors)
    effective["OPENWEPP_SNOW_STAGE3_EVALUATION_OPERATOR"] = {
        "control": "disabled",
        "paired": "same_state_paired_carrier_v1",
        "sequential": "sequential_resolved_shadow_v1",
    }[lane]
    environment, removed, observed = carrier.sanitized_environment(trace, effective)
    started = time.perf_counter()
    process = subprocess.Popen(command, cwd=REPO, env=environment, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    maximum_rss_kib = 0
    while True:
        try:
            stdout, stderr = process.communicate(timeout=0.1)
            break
        except subprocess.TimeoutExpired:
            status_path = Path(f"/proc/{process.pid}/status")
            if status_path.is_file():
                for status_line in status_path.read_text(encoding="utf-8").splitlines():
                    if status_line.startswith("VmRSS:"):
                        maximum_rss_kib = max(maximum_rss_kib, int(status_line.split()[1]))
                        break
    elapsed_seconds = time.perf_counter() - started
    (run_dir / "stdout.txt").write_text(stdout, encoding="utf-8")
    (run_dir / "stderr.txt").write_text(stderr, encoding="utf-8")
    if process.returncode:
        raise RuntimeError(f"run failed for {site}/{lane}: {stderr[-2000:]}")
    outputs = {path.name: {"path": relative(path), "sha256": sha256(path), "size_bytes": path.stat().st_size} for path in sorted(run_dir.iterdir()) if path.is_file()}
    return {"site": site, "lane": lane, "argv": [str(value) for value in command], "returncode": 0, "elapsed_seconds": elapsed_seconds, "maximum_observed_rss_kib": maximum_rss_kib, "removed_openwepp_key_names": removed, "effective_openwepp_environment": observed, "outputs": outputs, "runfile_sha256": sha256(runfile), "runfile_consumer": consumer}


def output_path(receipt: dict[str, Any], suffix: str) -> Path:
    matches = [REPO / value["path"] for name, value in receipt["outputs"].items() if name.endswith(suffix)]
    if len(matches) != 1:
        raise RuntimeError(f"expected one {suffix} for {receipt['site']}/{receipt['lane']}")
    return matches[0]


def performance_observation(
    receipts: dict[str, dict[str, dict[str, Any]]]
) -> dict[str, Any]:
    sites: dict[str, Any] = {}
    for site, lanes in receipts.items():
        control_elapsed = checked_number(lanes["control"], "elapsed_seconds")
        control_rss = checked_int(lanes["control"], "maximum_observed_rss_kib")
        if control_elapsed <= 0.0 or control_rss < 0:
            raise RuntimeError(f"invalid control performance observation for {site}")
        sites[site] = {}
        for lane in ("control", "paired", "sequential"):
            elapsed = checked_number(lanes[lane], "elapsed_seconds")
            rss = checked_int(lanes[lane], "maximum_observed_rss_kib")
            if elapsed <= 0.0 or rss < 0:
                raise RuntimeError(f"invalid performance observation for {site}/{lane}")
            sites[site][lane] = {
                "elapsed_seconds": elapsed,
                "elapsed_ratio_to_control": elapsed / control_elapsed,
                "maximum_observed_rss_kib": rss,
                "rss_ratio_to_control": (
                    None if control_rss == 0 else rss / control_rss
                ),
            }
    return {
        "sites": sites,
        "disposition": "OBSERVED_ONLY_NO_FROZEN_NUMERIC_ACCEPTANCE_THRESHOLD",
        "tuple_completeness_gate": "ENFORCED_BY_EXACT_CADENCE_TERMINAL_AND_1440_BOUND",
    }


def externalize_inventories(
    site_summary: dict[str, Any], *, write: bool
) -> dict[str, Any]:
    inventories = required(site_summary, "inventories")
    if not isinstance(inventories, dict):
        raise RuntimeError("site inventory is not an object")
    payload = json_bytes(inventories)
    path = OUTPUT / "inventories" / f"{checked_string(site_summary, 'site')}.json"
    if write:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(payload)
    elif not path.is_file() or path.read_bytes() != payload:
        raise RuntimeError(f"retained exhaustive inventory differs for {site_summary['site']}")
    site_summary["inventories"] = {
        "path": relative(path),
        "sha256": hashlib.sha256(payload).hexdigest(),
        "size_bytes": len(payload),
        "counts": {
            key: len(value)
            for key, value in inventories.items()
            if isinstance(value, list)
        },
        "scope": "exhaustive_typed_hour_and_water_year_inventory",
    }
    return site_summary


def execute(expected_head: str) -> None:
    if OUTPUT.exists():
        raise RuntimeError(f"refusing to overwrite {OUTPUT}")
    frozen = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    if frozen.get("status") != "frozen_result_blind_admitted_pass_pass":
        raise RuntimeError("protocol is not admitted")
    head = assert_execution_source(expected_head)
    carrier = load_module("operator_reconciliation_predecessor", PREDECESSOR_TOOL)
    carrier.OUTPUT = OUTPUT
    carrier.FREEZE_PATH = FREEZE_PATH
    build_command = ["cargo", "build", "--release", "-p", "openwepp-runner", "--bin", "openwepp-cli-hill"]
    build = subprocess.run(build_command, cwd=REPO, text=True, capture_output=True, check=False)
    if build.returncode or not BINARY.is_file():
        raise RuntimeError(f"release build failed: {build.stderr[-4000:]}")
    assert_execution_source(expected_head)
    OUTPUT.mkdir(parents=True)
    (OUTPUT / "inputs").mkdir()
    shutil.copyfile(FREEZE_PATH, OUTPUT / "inputs/protocol-freeze.json")
    (OUTPUT / "binary").mkdir()
    shutil.copyfile(BINARY, OUTPUT / "binary/openwepp-cli-hill")
    (OUTPUT / "build.stdout.txt").write_text(build.stdout, encoding="utf-8")
    (OUTPUT / "build.stderr.txt").write_text(build.stderr, encoding="utf-8")
    w1 = load_module("operator_reconciliation_w1", W1_TOOL)
    fixtures: dict[str, Path] = {}
    fixture_receipts = {}
    for frozen_site in frozen["cohort"]:
        site = frozen_site["site"]
        fixtures[site], fixture_receipts[site] = carrier.prepare_fixture(site, frozen_site)
        observation = carrier.OBSERVATIONS / f"{site}.csv"
        if sha256(observation) != frozen_site["observation_sha256"]:
            raise RuntimeError(f"observation hash differs for {site}")
        retained = OUTPUT / "inputs/observations" / f"{site}.csv"
        retained.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(observation, retained)
    predecessor_trace = REPO / frozen["predecessor"]["trace_path"]
    if sha256(predecessor_trace) != frozen["predecessor"]["trace_sha256"]:
        raise RuntimeError("historical predecessor trace hash differs")
    predecessor_artifact = REPO / frozen["predecessor"]["artifact_path"]
    if sha256(predecessor_artifact) != frozen["predecessor"]["artifact_sha256"]:
        raise RuntimeError("historical predecessor artifact hash differs")
    retained_predecessor = OUTPUT / "inputs/predecessor/snowbird-shadow-corrected.snow.jsonl"
    retained_predecessor.parent.mkdir(parents=True)
    shutil.copyfile(predecessor_trace, retained_predecessor)
    selectors = dict(frozen["selectors_except_operator"])
    receipts: dict[str, dict[str, Any]] = {site: {} for site in fixtures}
    jobs = [(site, lane) for site in fixtures for lane in ("control", "paired", "sequential")]
    with ThreadPoolExecutor(max_workers=3) as executor:
        futures = {executor.submit(execute_lane, site, fixtures[site], lane, selectors, carrier, w1): (site, lane) for site, lane in jobs}
        for future in as_completed(futures):
            site, lane = futures[future]
            receipts[site][lane] = future.result()
    protected = {}
    for site, lanes in receipts.items():
        control_wat = sha256(output_path(lanes["control"], ".wat.parquet"))
        control_hbp = sha256(output_path(lanes["control"], ".hbp"))
        protected[site] = {"wat_exact": {}, "hbp_exact": {}, "pass_output_exact": {}}
        for lane in ("paired", "sequential"):
            protected[site]["wat_exact"][lane] = sha256(output_path(lanes[lane], ".wat.parquet")) == control_wat
            protected[site]["hbp_exact"][lane] = sha256(output_path(lanes[lane], ".hbp")) == control_hbp
            protected[site]["pass_output_exact"][lane] = protected[site]["hbp_exact"][lane]
            if not protected[site]["wat_exact"][lane] or not protected[site]["hbp_exact"][lane]:
                raise RuntimeError(f"protected output differs for {site}/{lane}")
    annual = []
    site_summaries = []
    snowbird_fixture = fixtures["snotel_snowbird_ut"]
    snowbird_dates = carrier.climate_dates(carrier.climate_file(snowbird_fixture))
    snowbird_peaks, _ = carrier.observed_peaks(OUTPUT / "inputs/observations/snotel_snowbird_ut.csv")
    predecessor_windows = historical_predecessor_windows(
        retained_predecessor,
        snowbird_dates,
        snowbird_peaks,
    )
    for site, fixture in fixtures.items():
        site_annual, summary = reconcile_site(
            site,
            fixture,
            OUTPUT / "inputs/observations" / f"{site}.csv",
            output_path(receipts[site]["paired"], ".snow.jsonl"),
            output_path(receipts[site]["sequential"], ".snow.jsonl"),
            carrier,
            predecessor_windows if site == "snotel_snowbird_ut" else None,
        )
        annual.extend(site_annual)
        site_summaries.append(externalize_inventories(summary, write=True))
    snowbird = next(row for row in site_summaries if row["site"] == "snotel_snowbird_ut")
    performance = performance_observation(receipts)
    results = {"schema_version": 1, "execution_head": head, "decision_classes": classify(snowbird), "site_summaries": site_summaries, "annual_samples": annual, "performance_observation": performance, "claim_class": "operator_mechanics_only", "coe_authority": "unchanged"}
    write_json(OUTPUT / "results/operator-reconciliation-results.json", results)
    write_json(OUTPUT / "execution-receipt.json", {"execution_head": head, "binary_sha256": sha256(BINARY), "build": {"argv": build_command, "returncode": build.returncode}, "fixtures": fixture_receipts, "lanes": receipts, "protected_identity": protected})
    write_json(OUTPUT / "retained-artifact-manifest.json", carrier.retained_manifest(OUTPUT))
    assert_execution_source(expected_head)


def verify_existing() -> None:
    if command_output(["git", "status", "--porcelain"]):
        raise RuntimeError("retained verification requires a clean worktree")
    carrier = load_module("operator_reconciliation_verify", PREDECESSOR_TOOL)
    manifest = json.loads((OUTPUT / "retained-artifact-manifest.json").read_text(encoding="utf-8"))
    for item in manifest["files"]:
        path = OUTPUT / item["path"]
        if not path.is_file() or sha256(path) != item["sha256"] or path.stat().st_size != item["size_bytes"]:
            raise RuntimeError(f"retained artifact differs: {item['path']}")
    receipt = json.loads((OUTPUT / "execution-receipt.json").read_text(encoding="utf-8"))
    execution_head = receipt.get("execution_head")
    if not isinstance(execution_head, str) or not re.fullmatch(r"[0-9a-f]{40}", execution_head):
        raise RuntimeError("retained execution HEAD is invalid")
    subprocess.run(["git", "cat-file", "-e", f"{execution_head}^{{commit}}"], cwd=REPO, check=True)
    if command_output(["git", "rev-parse", "HEAD"]) != execution_head:
        raise RuntimeError("retained verification HEAD differs from execution HEAD")
    retained_binary = OUTPUT / "binary/openwepp-cli-hill"
    if sha256(retained_binary) != receipt.get("binary_sha256"):
        raise RuntimeError("retained binary differs from execution receipt")
    if sha256(FREEZE_PATH) != sha256(OUTPUT / "inputs/protocol-freeze.json"):
        raise RuntimeError("canonical and retained protocol freezes differ")
    frozen = json.loads((OUTPUT / "inputs/protocol-freeze.json").read_text(encoding="utf-8"))
    for frozen_site in frozen["cohort"]:
        site = frozen_site["site"]
        observation = OUTPUT / "inputs/observations" / f"{site}.csv"
        if sha256(observation) != frozen_site["observation_sha256"]:
            raise RuntimeError(f"retained observation differs for {site}")
        fixture = OUTPUT / "fixtures" / site
        actual_manifest = carrier.file_manifest(fixture)
        fixture_receipt = receipt["fixtures"].get(site)
        if not isinstance(fixture_receipt, dict):
            raise RuntimeError(f"missing fixture receipt for {site}")
        if fixture_receipt["source_manifest"]["manifest_sha256"] != frozen_site["fixture_manifest_sha256"]:
            raise RuntimeError(f"source fixture custody differs for {site}")
        if actual_manifest != fixture_receipt["copied_manifest"]:
            raise RuntimeError(f"retained fixture manifest differs for {site}")
        expected_climate_hash = (
            frozen_site["development_climate_sha256"]
            if site == "snotel_snowbird_ut"
            else fixture_receipt["staged_climate_sha256"]
        )
        if sha256(carrier.climate_file(fixture)) != expected_climate_hash:
            raise RuntimeError(f"retained climate differs for {site}")
        for lane, lane_receipt in receipt["lanes"][site].items():
            for output in lane_receipt["outputs"].values():
                path = REPO / output["path"]
                if sha256(path) != output["sha256"] or path.stat().st_size != output["size_bytes"]:
                    raise RuntimeError(f"receipt output differs for {site}/{lane}")
    retained_predecessor = OUTPUT / "inputs/predecessor/snowbird-shadow-corrected.snow.jsonl"
    if sha256(retained_predecessor) != frozen["predecessor"]["trace_sha256"]:
        raise RuntimeError("retained predecessor trace differs")
    snowbird_fixture = OUTPUT / "fixtures/snotel_snowbird_ut"
    snowbird_dates = carrier.climate_dates(carrier.climate_file(snowbird_fixture))
    snowbird_peaks, _ = carrier.observed_peaks(OUTPUT / "inputs/observations/snotel_snowbird_ut.csv")
    predecessor_windows = historical_predecessor_windows(retained_predecessor, snowbird_dates, snowbird_peaks)
    recomputed_annual: list[dict[str, Any]] = []
    recomputed_summaries: list[dict[str, Any]] = []
    for frozen_site in frozen["cohort"]:
        site = frozen_site["site"]
        fixture = OUTPUT / "fixtures" / site
        site_annual, site_summary = reconcile_site(
            site,
            fixture,
            OUTPUT / "inputs/observations" / f"{site}.csv",
            output_path(receipt["lanes"][site]["paired"], ".snow.jsonl"),
            output_path(receipt["lanes"][site]["sequential"], ".snow.jsonl"),
            carrier,
            predecessor_windows if site == "snotel_snowbird_ut" else None,
        )
        recomputed_annual.extend(site_annual)
        recomputed_summaries.append(
            externalize_inventories(site_summary, write=False)
        )
        control_wat = sha256(output_path(receipt["lanes"][site]["control"], ".wat.parquet"))
        control_hbp = sha256(output_path(receipt["lanes"][site]["control"], ".hbp"))
        for lane in ("paired", "sequential"):
            if sha256(output_path(receipt["lanes"][site][lane], ".wat.parquet")) != control_wat or sha256(output_path(receipt["lanes"][site][lane], ".hbp")) != control_hbp:
                raise RuntimeError(f"retained protected output differs for {site}/{lane}")
    stored = json.loads((OUTPUT / "results/operator-reconciliation-results.json").read_text(encoding="utf-8"))
    snowbird = next(row for row in recomputed_summaries if row["site"] == "snotel_snowbird_ut")
    performance = performance_observation(receipt["lanes"])
    recomputed = {
        "schema_version": 1,
        "execution_head": execution_head,
        "decision_classes": classify(snowbird),
        "site_summaries": recomputed_summaries,
        "annual_samples": recomputed_annual,
        "performance_observation": performance,
        "claim_class": "operator_mechanics_only",
        "coe_authority": "unchanged",
    }
    if stored != recomputed:
        raise RuntimeError("stored compact results differ from independent retained reconstruction")
    print(f"PASS verified {manifest['file_count']} retained artifacts")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--expected-head")
    parser.add_argument("--verify-existing", action="store_true")
    args = parser.parse_args()
    if args.verify_existing:
        verify_existing()
    elif args.expected_head:
        execute(args.expected_head)
    else:
        parser.error("--expected-head or --verify-existing is required")


if __name__ == "__main__":
    main()
