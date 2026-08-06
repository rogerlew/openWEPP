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
import subprocess
import sys
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


def checked_number(row: dict[str, Any], field: str) -> float:
    value = row.get(field)
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


def validate_tuple(row: dict[str, Any], operator: str) -> None:
    if row.get("operator_id") != operator or row.get("applicable") is not True:
        raise RuntimeError("tuple operator/applicability mismatch")
    duration = checked_number(row, "duration_seconds")
    if duration <= 0.0 or checked_number(row, "evaluated_seconds") != duration:
        raise RuntimeError("tuple duration mismatch")
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
    latent_heat = row.get("surface_latent_heat_j_kg")
    if latent_heat is None:
        if row.get("turbulent_termination_status") != "zero_wind":
            raise RuntimeError("latent heat is null outside zero-wind")
    else:
        close(
            "latent primitive",
            checked_number(row, "latent_flux_w_m2"),
            checked_number(row, "vapor_mass_flux_kg_m2_s") * float(latent_heat),
            1.0e-10,
        )
        independently_reconstructed = monin_obukhov(
            air_temperature_c=checked_number(row, "air_temperature_c"),
            surface_temperature_c=checked_number(row, "surface_temperature_before_c"),
            air_vapor_pressure_pa=checked_number(row, "actual_vapor_pressure_pa"),
            surface_vapor_pressure_pa=checked_number(row, "surface_vapor_pressure_pa"),
            air_pressure_pa=checked_number(row, "air_pressure_pa"),
            wind_speed_m_s=checked_number(row, "wind_speed_m_s"),
            z_t=checked_number(row, "air_temperature_height_m"),
            z_q=checked_number(row, "vapor_pressure_height_m"),
            z_u=checked_number(row, "wind_speed_height_m"),
            z_0=checked_number(row, "aerodynamic_roughness_length_m"),
            max_iterations=int(row["turbulent_max_iterations"]),
            tolerance=checked_number(row, "turbulent_convergence_tolerance"),
        )
        close(
            "independent sensible reconstruction",
            checked_number(row, "sensible_flux_w_m2"),
            independently_reconstructed["sensible"],
            1.0e-9,
        )
        close(
            "independent latent reconstruction",
            checked_number(row, "latent_flux_w_m2"),
            independently_reconstructed["latent"],
            1.0e-9,
        )
    external = sum(
        checked_number(row, field)
        for field in (
            "net_shortwave_w_m2",
            "net_longwave_w_m2",
            "sensible_flux_w_m2",
            "latent_flux_w_m2",
            "precipitation_advected_flux_w_m2",
        )
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
        ):
            close(f"same-state {after}", checked_number(row, after), checked_number(row, before), 0.0)
    else:
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


def validate_v6_row(row: dict[str, Any], operator: str) -> list[dict[str, Any]]:
    if row.get("schema") != "openwepp-r7h-direct-production-snow-trace-v6":
        raise RuntimeError(f"unknown enabled schema: {row.get('schema')}")
    companion = row.get("stage3_operator_reconciliation")
    if not isinstance(companion, dict) or companion.get("schema_version") != 6:
        raise RuntimeError("missing schema-v6 reconciliation object")
    statuses = companion.get("hourly_status")
    tuples = companion.get("tuples")
    if not isinstance(statuses, list) or len(statuses) != 24 or not isinstance(tuples, list):
        raise RuntimeError("invalid reconciliation arrays")
    if len(tuples) > 1_440:
        raise RuntimeError("tuple bound exceeded")
    expected_index = [0] * 24
    elapsed = [0.0] * 24
    for item in tuples:
        if not isinstance(item, dict):
            raise RuntimeError("tuple is not an object")
        hour = item.get("hour_index")
        substep = item.get("substep_index")
        if not isinstance(hour, int) or not 0 <= hour < 24 or substep != expected_index[hour]:
            raise RuntimeError("tuple order mismatch")
        close("elapsed start", checked_number(item, "elapsed_start_seconds"), elapsed[hour], 0.0)
        validate_tuple(item, operator)
        elapsed[hour] += checked_number(item, "duration_seconds")
        expected_index[hour] += 1
    for hour, status in enumerate(statuses):
        if not isinstance(status, dict) or status.get("evaluated") != (expected_index[hour] > 0):
            raise RuntimeError("hourly status mismatch")
        if operator == "same_state_paired_carrier_v1" and expected_index[hour]:
            if expected_index[hour] != 1 or elapsed[hour] != 3_600.0:
                raise RuntimeError("same-state cadence mismatch")
    return tuples


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


def frozen_active_flux(same: dict[str, Any], first_q: dict[str, Any]) -> dict[str, float]:
    surface_c = checked_number(first_q, "surface_temperature_before_c")
    shortwave = checked_number(same, "incoming_shortwave_w_m2") * (
        1.0 - checked_number(first_q, "snow_albedo_fraction")
    )
    outgoing = SIGMA * (surface_c + FREEZE_K) ** 4
    longwave = checked_number(same, "subcanopy_longwave_w_m2") - outgoing
    turbulent = monin_obukhov(
        air_temperature_c=checked_number(same, "air_temperature_c"),
        surface_temperature_c=surface_c,
        air_vapor_pressure_pa=checked_number(same, "actual_vapor_pressure_pa"),
        surface_vapor_pressure_pa=saturation_vapor_pressure(surface_c),
        air_pressure_pa=checked_number(same, "air_pressure_pa"),
        wind_speed_m_s=checked_number(same, "wind_speed_m_s"),
        z_t=checked_number(same, "air_temperature_height_m"),
        z_q=checked_number(same, "vapor_pressure_height_m"),
        z_u=checked_number(same, "wind_speed_height_m"),
        z_0=checked_number(same, "aerodynamic_roughness_length_m"),
        max_iterations=int(same["turbulent_max_iterations"]),
        tolerance=checked_number(same, "turbulent_convergence_tolerance"),
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


def monin_obukhov(**values: float) -> dict[str, float]:
    wind = values["wind_speed_m_s"]
    if wind == 0.0:
        return {"sensible": 0.0, "latent": 0.0}
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

    def recompute() -> tuple[float, float, float]:
        friction = VON_KARMAN * wind / (logs[0] - corrections[0])
        factor = VON_KARMAN * friction * density
        mass = (q_air - q_surface) * factor / (logs[2] - corrections[2])
        sensible = (potential_k - surface_k) * factor * CP_AIR / (logs[1] - corrections[1])
        return friction, mass, sensible

    friction, mass, sensible = recompute()
    if abs(potential_k - surface_k) <= sys.float_info.epsilon:
        return {"sensible": sensible, "latent": latent_heat * mass}
    length = math.inf
    for _ in range(int(values["max_iterations"])):
        previous = length
        buoyancy = sensible / (potential_k * CP_AIR) + 0.61 * mass
        if buoyancy == 0.0:
            return {"sensible": sensible, "latent": latent_heat * mass}
        length = friction**3 * density / (VON_KARMAN * GRAVITY * buoyancy)
        if not math.isfinite(length) or length == 0.0:
            return {"sensible": sensible, "latent": latent_heat * mass}
        corrections = [psi(values[key] / length, kind) for key, kind in (("z_u", "momentum"), ("z_t", "heat"), ("z_q", "heat"))]
        friction, mass, sensible = recompute()
        delta = previous - length
        if abs(delta) <= values["tolerance"] or abs(delta / length) <= values["tolerance"]:
            return {"sensible": sensible, "latent": latent_heat * mass}
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


def parse_trace(path: Path, dates: list[dt.date], operator: str) -> dict[dt.date, dict[str, Any]]:
    result: dict[dt.date, dict[str, Any]] = {}
    with path.open(encoding="utf-8") as handle:
        for index, line in enumerate(handle):
            if index >= len(dates):
                raise RuntimeError("trace has more rows than climate")
            row = json.loads(line)
            if row.get("day_index") != index or row.get("lane_index") != 0:
                raise RuntimeError("trace identity mismatch")
            row["_tuples"] = validate_v6_row(row, operator)
            result[dates[index]] = row
    if len(result) != len(dates):
        raise RuntimeError("trace/climate row-count mismatch")
    return result


def tuples_by_hour(rows: list[dict[str, Any]]) -> list[list[dict[str, Any]]]:
    grouped = [[] for _ in range(24)]
    for row in rows:
        grouped[int(row["hour_index"])].append(row)
    return grouped


def reconcile_site(
    site: str,
    fixture: Path,
    observation: Path,
    paired_trace: Path,
    sequential_trace: Path,
    carrier: Any,
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    dates = carrier.climate_dates(carrier.climate_file(fixture))
    paired = parse_trace(paired_trace, dates, "same_state_paired_carrier_v1")
    sequential = parse_trace(sequential_trace, dates, "sequential_resolved_shadow_v1")
    peaks, _ = carrier.observed_peaks(observation)
    daily: dict[dt.date, dict[str, float]] = {}
    projection_difference = False
    omitted_abs = 0.0
    all_abs = 0.0
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
        sums = {name: 0.0 for name in ("S", "F", "Q", "legacy_Q", *(f"S_{term}" for term in TERMS), *(f"F_{term}" for term in TERMS), *(f"Q_{term}" for term in TERMS))}
        sums["support_seconds"] = 0.0
        first_q = next((row for hour in q_hours for row in hour), None)
        if first_q is not None:
            first_s = next((row for hour in s_hours for row in hour), None)
            if first_s is not None and first_s["effective_input_fingerprint_fnv1a64"] != first_q["effective_input_fingerprint_fnv1a64"]:
                projection_difference = True
        for hour in range(24):
            if not s_hours[hour] or not q_hours[hour] or first_q is None:
                for rows in (s_hours[hour], q_hours[hour]):
                    omitted_abs += sum(abs(checked_number(row, "complete_external_flux_w_m2") * checked_number(row, "duration_seconds")) for row in rows)
                continue
            s = s_hours[hour][0]
            s_support = sum(checked_number(row, "duration_seconds") for row in s_hours[hour])
            q_support = sum(checked_number(row, "duration_seconds") for row in q_hours[hour])
            common = min(s_support, q_support, 3_600.0)
            sums["support_seconds"] += common
            frozen = frozen_active_flux(s, first_q)
            s_energy = integrate_prefix(s_hours[hour], common, "complete_external_flux_w_m2")
            q_energy = integrate_prefix(q_hours[hour], common, "complete_external_flux_w_m2")
            f_energy = frozen["external"] * common
            sums["S"] += s_energy
            sums["F"] += f_energy
            sums["Q"] += q_energy
            sums["legacy_Q"] += integrate_prefix(q_hours[hour], common, "complete_external_flux_w_m2") + sum(checked_number(row, "internal_active_lower_conduction_j_m2") * min(checked_number(row, "duration_seconds"), max(0.0, common - checked_number(row, "elapsed_start_seconds"))) / checked_number(row, "duration_seconds") for row in q_hours[hour] if checked_number(row, "elapsed_start_seconds") < common)
            for term, field in zip(TERMS, ("net_shortwave_w_m2", "net_longwave_w_m2", "sensible_flux_w_m2", "latent_flux_w_m2", "precipitation_advected_flux_w_m2"), strict=True):
                sums[f"S_{term}"] += integrate_prefix(s_hours[hour], common, field)
                sums[f"Q_{term}"] += integrate_prefix(q_hours[hour], common, field)
                sums[f"F_{term}"] += frozen[term] * common
            for rows, evaluated in ((s_hours[hour], s_support), (q_hours[hour], q_support)):
                all_abs += sum(abs(checked_number(row, "complete_external_flux_w_m2") * checked_number(row, "duration_seconds")) for row in rows)
                if evaluated > common:
                    omitted_abs += abs(integrate_prefix(rows, evaluated, "complete_external_flux_w_m2") - integrate_prefix(rows, common, "complete_external_flux_w_m2"))
        daily[stamp] = sums
    annual: list[dict[str, Any]] = []
    for year, (peak, _) in sorted(peaks.items()):
        if year == 2025:
            continue
        start = dt.date(year - 1, 10, 1)
        window = [stamp for stamp in dates if start <= stamp <= peak]
        if not window:
            continue
        row: dict[str, Any] = {"site": site, "water_year": year, "window_days": len(window)}
        for field in next(iter(daily.values())):
            if field == "support_seconds":
                continue
            row[f"{field}_j_m2"] = sum(daily[stamp][field] for stamp in window)
        row["support_seconds"] = sum(
            daily[stamp]["support_seconds"] for stamp in window
        )
        row["coverage_fraction"] = row["support_seconds"] / (len(window) * 86_400.0)
        annual.append(row)
    medians = {
        field: statistics.median(float(row[field]) for row in annual)
        for field in ("S_j_m2", "F_j_m2", "Q_j_m2", "legacy_Q_j_m2")
    }
    return annual, {
        "site": site,
        "sample_count": len(annual),
        "medians_j_m2": medians,
        "delta_projection_j_m2": medians["F_j_m2"] - medians["S_j_m2"],
        "delta_evolution_j_m2": medians["Q_j_m2"] - medians["F_j_m2"],
        "initial_projection_difference_observed": projection_difference,
        "support_omission_ratio": None if all_abs == 0.0 else omitted_abs / all_abs,
    }


def classify(snowbird: dict[str, Any]) -> list[str]:
    medians = snowbird["medians_j_m2"]
    classes: list[str] = []
    predecessor_ok = abs(medians["legacy_Q_j_m2"] / 1.0e6 - PREDECESSOR_MJ_M2) <= PREDECESSOR_TOL_MJ_M2
    if not predecessor_ok:
        classes.append("PREDECESSOR_NOT_REPRODUCED")
    if medians["legacy_Q_j_m2"] > ZERO_J_M2 and medians["Q_j_m2"] <= ZERO_J_M2:
        classes.append("LEGACY_ESTIMAND_INTERNAL_CONDUCTION_SIGN_DIFFERENCE")
    if snowbird["initial_projection_difference_observed"]:
        classes.append("INITIAL_CONTROL_VOLUME_PROJECTION_DIFFERENCE")
    if predecessor_ok and medians["S_j_m2"] < -ZERO_J_M2 and medians["F_j_m2"] > ZERO_J_M2 and medians["Q_j_m2"] > ZERO_J_M2:
        classes.append("INITIAL_CONTROL_VOLUME_PROJECTION_RECONCILES_SIGN_CONTRADICTION")
    if predecessor_ok and medians["S_j_m2"] < -ZERO_J_M2 and medians["F_j_m2"] < -ZERO_J_M2 and medians["Q_j_m2"] > ZERO_J_M2:
        classes.append("STATE_EVOLUTION_RECONCILES_SIGN_CONTRADICTION")
    if snowbird["support_omission_ratio"] is not None and snowbird["support_omission_ratio"] > 0.05:
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
    completed = subprocess.run(command, cwd=REPO, env=environment, text=True, capture_output=True, check=False)
    (run_dir / "stdout.txt").write_text(completed.stdout, encoding="utf-8")
    (run_dir / "stderr.txt").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode:
        raise RuntimeError(f"run failed for {site}/{lane}: {completed.stderr[-2000:]}")
    outputs = {path.name: {"path": relative(path), "sha256": sha256(path), "size_bytes": path.stat().st_size} for path in sorted(run_dir.iterdir()) if path.is_file()}
    return {"site": site, "lane": lane, "argv": [str(value) for value in command], "returncode": 0, "removed_openwepp_key_names": removed, "effective_openwepp_environment": observed, "outputs": outputs, "runfile_sha256": sha256(runfile), "runfile_consumer": consumer}


def output_path(receipt: dict[str, Any], suffix: str) -> Path:
    matches = [REPO / value["path"] for name, value in receipt["outputs"].items() if name.endswith(suffix)]
    if len(matches) != 1:
        raise RuntimeError(f"expected one {suffix} for {receipt['site']}/{receipt['lane']}")
    return matches[0]


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
        protected[site] = {"wat_exact": {}, "hbp_exact": {}}
        for lane in ("paired", "sequential"):
            protected[site]["wat_exact"][lane] = sha256(output_path(lanes[lane], ".wat.parquet")) == control_wat
            protected[site]["hbp_exact"][lane] = sha256(output_path(lanes[lane], ".hbp")) == control_hbp
            if not protected[site]["wat_exact"][lane] or not protected[site]["hbp_exact"][lane]:
                raise RuntimeError(f"protected output differs for {site}/{lane}")
    annual = []
    site_summaries = []
    for site, fixture in fixtures.items():
        site_annual, summary = reconcile_site(
            site,
            fixture,
            OUTPUT / "inputs/observations" / f"{site}.csv",
            output_path(receipts[site]["paired"], ".snow.jsonl"),
            output_path(receipts[site]["sequential"], ".snow.jsonl"),
            carrier,
        )
        annual.extend(site_annual)
        site_summaries.append(summary)
    snowbird = next(row for row in site_summaries if row["site"] == "snotel_snowbird_ut")
    results = {"schema_version": 1, "execution_head": head, "decision_classes": classify(snowbird), "site_summaries": site_summaries, "annual_samples": annual, "claim_class": "operator_mechanics_only", "coe_authority": "unchanged"}
    write_json(OUTPUT / "results/operator-reconciliation-results.json", results)
    write_json(OUTPUT / "execution-receipt.json", {"execution_head": head, "binary_sha256": sha256(BINARY), "build": {"argv": build_command, "returncode": build.returncode}, "fixtures": fixture_receipts, "lanes": receipts, "protected_identity": protected})
    write_json(OUTPUT / "retained-artifact-manifest.json", carrier.retained_manifest(OUTPUT))
    assert_execution_source(expected_head)


def verify_existing() -> None:
    carrier = load_module("operator_reconciliation_verify", PREDECESSOR_TOOL)
    manifest = json.loads((OUTPUT / "retained-artifact-manifest.json").read_text(encoding="utf-8"))
    for item in manifest["files"]:
        path = OUTPUT / item["path"]
        if not path.is_file() or sha256(path) != item["sha256"] or path.stat().st_size != item["size_bytes"]:
            raise RuntimeError(f"retained artifact differs: {item['path']}")
    receipt = json.loads((OUTPUT / "execution-receipt.json").read_text(encoding="utf-8"))
    frozen = json.loads((OUTPUT / "inputs/protocol-freeze.json").read_text(encoding="utf-8"))
    for frozen_site in frozen["cohort"]:
        site = frozen_site["site"]
        fixture = OUTPUT / "fixtures" / site
        reconcile_site(
            site,
            fixture,
            OUTPUT / "inputs/observations" / f"{site}.csv",
            output_path(receipt["lanes"][site]["paired"], ".snow.jsonl"),
            output_path(receipt["lanes"][site]["sequential"], ".snow.jsonl"),
            carrier,
        )
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
