#!/usr/bin/env python3
"""Independent OPENWEPP_SNOW_FREE_HALF_HOUR_FORCING_V1 authority calculator.

Standard-library only. It does not import or execute Rust and writes canonical
JSON to stdout. Equations are transcribed from the sources recorded in
SC-SNOWFREEFORCING-001.
"""

from __future__ import annotations

import json
import hashlib
import math
import sys
from dataclasses import dataclass

SIGMA = 5.670_374_419e-8
RHO_W = 1000.0
CP_W = 4218.0
TOL = 1.0e-12


class AuthorityDomain(ValueError):
    pass


def finite(name: str, value: float) -> float:
    if not math.isfinite(value):
        raise AuthorityDomain(f"{name}:nonfinite")
    return value


def saturation_water_kpa(temperature_c: float) -> float:
    finite("temperature_c", temperature_c)
    denominator = 237.3 + temperature_c
    if denominator == 0.0:
        raise AuthorityDomain("saturation:singular")
    return finite("saturation_kpa", 0.611 * math.exp(17.3 * temperature_c / denominator))


def saturation_auto_kpa(temperature_c: float) -> float:
    if temperature_c > 0.0:
        return saturation_water_kpa(temperature_c)
    denominator = 272.55 + temperature_c
    if denominator == 0.0:
        raise AuthorityDomain("saturation_ice:singular")
    return finite("saturation_ice_kpa", 0.61115 * math.exp(22.452 * temperature_c / denominator))


def vapor_density(pressure_kpa: float, temperature_c: float) -> float:
    temperature_k = temperature_c + 273.15
    if temperature_k <= 0.0:
        raise AuthorityDomain("vapor_density:absolute_zero")
    return 0.018_015_28 * pressure_kpa * 1000.0 / (8.314_41 * temperature_k)


def hydrometeor_residual(air_c: float, air_vapor_density: float, candidate_c: float) -> float:
    diffusivity = 2.06e-5 * ((air_c + 273.15) / 273.15) ** 1.75
    conductivity = 0.000_063 * (air_c + 273.15) + 0.006_73
    saturation_density = vapor_density(saturation_auto_kpa(candidate_c), candidate_c)
    latent_heat = (
        1000.0 * (2834.1 - 0.29 * candidate_c - 0.004 * candidate_c**2)
        if candidate_c < 0.0
        else 1000.0 * (2501.0 - 2.361 * candidate_c)
    )
    next_c = air_c + diffusivity / conductivity * latent_heat * (air_vapor_density - saturation_density)
    return candidate_c - next_c


def next_hydrometeor_temperature(air_c: float, air_vapor_density: float, candidate_c: float) -> float:
    return candidate_c - hydrometeor_residual(air_c, air_vapor_density, candidate_c)


def harder_pomeroy_hourly(air_c: float, dew_c: float) -> dict[str, float]:
    relative_humidity = min(1.0, saturation_water_kpa(dew_c) / saturation_water_kpa(air_c))
    air_vapor_density = vapor_density(saturation_auto_kpa(air_c) * relative_humidity, air_c)
    candidate = air_c
    for _ in range(200):
        try:
            next_candidate = next_hydrometeor_temperature(air_c, air_vapor_density, candidate)
        except (AuthorityDomain, OverflowError):
            break
        if abs(next_candidate - candidate) <= 1.0e-8:
            rain = 1.0 / (1.0 + 2.502_86 * 0.125_006**next_candidate)
            return {
                "relative_humidity": relative_humidity,
                "hydrometeor_temperature_c": next_candidate,
                "rain_fraction": rain,
                "snow_fraction": 1.0 - rain,
            }
        candidate = next_candidate
    high = air_c
    low = max(-120.0, air_c - 40.0)
    high_residual = hydrometeor_residual(air_c, air_vapor_density, high)
    low_residual = hydrometeor_residual(air_c, air_vapor_density, low)
    while low_residual * high_residual > 0.0 and low > -120.0:
        low = max(-120.0, low - 40.0)
        low_residual = hydrometeor_residual(air_c, air_vapor_density, low)
    if low_residual * high_residual > 0.0:
        raise AuthorityDomain("hydrometeor:no_bracket")
    for _ in range(200):
        midpoint = 0.5 * (low + high)
        residual = hydrometeor_residual(air_c, air_vapor_density, midpoint)
        if abs(residual) <= 1.0e-8 or high - low <= 1.0e-8:
            rain = 1.0 / (1.0 + 2.502_86 * 0.125_006**midpoint)
            return {
                "relative_humidity": relative_humidity,
                "hydrometeor_temperature_c": midpoint,
                "rain_fraction": rain,
                "snow_fraction": 1.0 - rain,
            }
        if residual * high_residual > 0.0:
            high = midpoint
            high_residual = residual
        else:
            low = midpoint
    raise AuthorityDomain("hydrometeor:no_convergence")


def fao_pressure_kpa(elevation_m: float) -> float:
    finite("elevation_m", elevation_m)
    base = (293.0 - 0.0065 * elevation_m) / 293.0
    if base <= 0.0:
        raise AuthorityDomain("pressure:undefined_base")
    pressure = finite("pressure_kpa", 101.3 * base**5.26)
    if pressure <= 0.0:
        raise AuthorityDomain("pressure:nonpositive")
    return pressure


def humidity(air_c: float, dew_c: float, pressure_kpa: float) -> dict[str, float]:
    actual = saturation_water_kpa(dew_c)
    if pressure_kpa <= actual:
        raise AuthorityDomain("humidity:pressure_not_above_vapor")
    q = finite("specific_humidity", 0.622 * actual / (pressure_kpa - 0.378 * actual))
    vpd = finite("vpd_kpa", saturation_water_kpa(air_c) - actual)
    return {"actual_vapor_pressure_kpa": actual, "specific_humidity_kg_kg": q, "vpd_kpa": vpd}


def atmospheric_longwave(air_c: float, actual_kpa: float, cloud: float) -> float:
    if not 0.0 <= cloud <= 1.0:
        raise AuthorityDomain("longwave:cloud_fraction")
    temperature_k = finite("air_temperature_k", air_c + 273.15)
    if temperature_k <= 0.0 or actual_kpa < 0.0:
        raise AuthorityDomain("longwave:domain")
    water = 4650.0 * actual_kpa / temperature_k
    clear = 59.38 + 113.7 * (temperature_k / 273.16) ** 6 + 96.96 * math.sqrt(water / 25.0)
    blackbody = SIGMA * temperature_k**4
    clear_emissivity = clear / blackbody
    all_sky = (1.0 - 0.84 * cloud) * clear_emissivity + 0.84 * cloud
    return finite("downward_longwave_w_m2", all_sky * blackbody)


def weiss_norman(global_w_m2: float, mu: float, pressure_kpa: float) -> dict[str, float]:
    finite("global_shortwave", global_w_m2)
    if global_w_m2 < 0.0:
        raise AuthorityDomain("shortwave:negative_global")
    if global_w_m2 == 0.0:
        return {name: 0.0 for name in ("direct_visible_w_m2", "diffuse_visible_w_m2", "direct_nir_w_m2", "diffuse_nir_w_m2")}
    if not math.isfinite(mu) or mu <= 0.0 or not math.isfinite(pressure_kpa) or pressure_kpa <= 0.0:
        raise AuthorityDomain("shortwave:positive_global_requires_mu_pressure")
    air_mass = 1.0 / mu
    pressure_ratio = pressure_kpa / 101.325
    direct_vis_potential = 600.0 * math.exp(-0.185 * pressure_ratio * air_mass) * mu
    diffuse_vis_potential = 0.4 * (600.0 - direct_vis_potential / mu) * mu
    log_air_mass = math.log10(air_mass)
    water_absorption = 1320.0 * 10.0 ** (-1.1950 + 0.4459 * log_air_mass - 0.0345 * log_air_mass**2)
    direct_nir_potential = (720.0 * math.exp(-0.06 * pressure_ratio * air_mass) - water_absorption) * mu
    diffuse_nir_potential = 0.6 * (720.0 - direct_nir_potential / mu - water_absorption) * mu
    total_vis_potential = direct_vis_potential + diffuse_vis_potential
    total_nir_potential = direct_nir_potential + diffuse_nir_potential
    total_potential = total_vis_potential + total_nir_potential
    if min(total_vis_potential, total_nir_potential, total_potential) <= 0.0:
        raise AuthorityDomain("shortwave:nonpositive_potential")
    ratio = global_w_m2 / total_potential
    vis_total = global_w_m2 * total_vis_potential / total_potential
    nir_total = global_w_m2 * total_nir_potential / total_potential
    vis_ratio = min(ratio, 0.9)
    nir_ratio = min(ratio, 0.88)
    vis_beam_fraction = max(0.0, direct_vis_potential / total_vis_potential * (1.0 - ((0.9 - vis_ratio) / 0.7) ** (2.0 / 3.0)))
    nir_beam_fraction = max(0.0, direct_nir_potential / total_nir_potential * (1.0 - ((0.88 - nir_ratio) / 0.68) ** (2.0 / 3.0)))
    values = {
        "direct_visible_w_m2": vis_total * vis_beam_fraction,
        "diffuse_visible_w_m2": vis_total * (1.0 - vis_beam_fraction),
        "direct_nir_w_m2": nir_total * nir_beam_fraction,
        "diffuse_nir_w_m2": nir_total * (1.0 - nir_beam_fraction),
    }
    if any(not math.isfinite(value) or value < 0.0 for value in values.values()):
        raise AuthorityDomain("shortwave:invalid_component")
    closure = math.fsum(values.values())
    if abs(closure - global_w_m2) > TOL * max(1.0, abs(global_w_m2)):
        raise AuthorityDomain("shortwave:closure")
    return values


@dataclass(frozen=True)
class Segment:
    start_s: float
    end_s: float
    intensity_m_s: float


def breakpoint_child_masses(segments: list[Segment]) -> list[float]:
    masses: list[float] = []
    for child in range(48):
        left = 1800.0 * child
        right = left + 1800.0
        depth = math.fsum(max(0.0, min(right, segment.end_s) - max(left, segment.start_s)) * segment.intensity_m_s for segment in segments)
        masses.append(RHO_W * depth)
    expected = RHO_W * math.fsum((segment.end_s - segment.start_s) * segment.intensity_m_s for segment in segments)
    if abs(math.fsum(masses) - expected) > TOL * max(1.0, abs(expected)):
        raise AuthorityDomain("precipitation:closure")
    return masses


def event_relative_breakpoint_masses(storm_start_h: float, segments: list[Segment]) -> tuple[list[float], list[dict[str, float]]]:
    absolute = [Segment(storm_start_h * 3600.0 + segment.start_s, storm_start_h * 3600.0 + segment.end_s, segment.intensity_m_s) for segment in segments]
    current = breakpoint_child_masses([Segment(segment.start_s, min(segment.end_s, 86_400.0), segment.intensity_m_s) for segment in absolute if segment.start_s < 86_400.0])
    carry = [
        {"start_s": max(0.0, segment.start_s - 86_400.0), "end_s": segment.end_s - 86_400.0, "intensity_m_s": segment.intensity_m_s}
        for segment in absolute
        if segment.end_s > 86_400.0
    ]
    return current, carry


def parent_hour_children(parent_energy_mj_m2: float) -> dict[str, float]:
    if not math.isfinite(parent_energy_mj_m2) or parent_energy_mj_m2 < 0.0:
        raise AuthorityDomain("radiation:parent_energy")
    child = parent_energy_mj_m2 / 2.0
    flux = parent_energy_mj_m2 * 1_000_000.0 / 3600.0
    return {"child0_energy_mj_m2": child, "child1_energy_mj_m2": child, "both_child_flux_w_m2": flux}


def canonical_bytes(value: object) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False) + "\n").encode()


def simimpl_declination(day_of_year: int) -> float:
    return 0.00698 - 0.4067 * math.cos((day_of_year + 10.0) * 0.0172)


def radcur_parent(day_of_year: int, hour_one_based: int, latitude_rad: float, declination: float) -> tuple[float, float]:
    dfact = 2.0 * math.pi * (day_of_year - 81.0) / 365.0
    solar_time_correction = 0.1645 * math.sin(2.0 * dfact) - 0.1255 * math.cos(dfact) - 0.025 * math.sin(dfact)
    center = (hour_one_based + solar_time_correction - 12.0) * math.pi / 12.0
    left = center - math.pi / 24.0
    right = center + math.pi / 24.0
    integral = math.cos(latitude_rad) * math.cos(declination) * (math.sin(right) - math.sin(left)) + (right - left) * math.sin(latitude_rad) * math.sin(declination)
    mean_mu = max(0.0, integral / (right - left))
    earth_sun = 1.0 + 0.033 * math.cos(2.0 * math.pi * day_of_year / 365.0)
    energy = max(0.0, (12.0 * 60.0 / math.pi) * 0.082 * earth_sun * integral)
    return energy, mean_mu


def simimpl_hour_temperature(hour_one_based: int, halfday_h: float, tmax_c: float, tmin_c: float) -> float:
    if tmax_c - tmin_c <= 1.0:
        return (tmax_c + tmin_c) / 2.0
    sunrise = 12.0 - halfday_h
    average = (tmax_c + tmin_c) / 2.0
    amplitude = (tmax_c - tmin_c) / 2.0
    hour = float(hour_one_based)
    if hour < sunrise or 14.0 < hour:
        adjusted = hour - 0.5 + 10.0 if hour < sunrise else hour - 0.5 - 14.0
        return average + amplitude * math.cos(math.pi * adjusted / (10.0 + sunrise))
    return average - amplitude * math.cos(math.pi * (hour - 0.5 - sunrise) / (14.0 - sunrise))


def simimpl_parents(*, day_of_year: int, latitude_deg: float, radiation_ly: float, tmax_c: float, tmin_c: float) -> tuple[list[dict[str, float]], float]:
    latitude = math.radians(latitude_deg)
    declination = simimpl_declination(day_of_year)
    sunrise_angle = math.acos(max(-1.0, min(1.0, -math.tan(latitude) * math.tan(declination))))
    halfday = sunrise_angle * 12.0 / math.pi
    potentials = [radcur_parent(day_of_year, hour, latitude, declination) for hour in range(1, 25)]
    daily_potential = math.fsum(value[0] for value in potentials)
    daily_energy = radiation_ly * 0.04184
    if daily_energy < 0.0 or daily_energy > daily_potential:
        raise AuthorityDomain("simimpl:daily_radiation_bound")
    solar_noon_mu = math.sin(declination) * math.sin(latitude) + math.cos(declination) * math.cos(latitude)
    transmittance_seed = daily_energy / daily_potential if daily_potential else 0.0
    if solar_noon_mu <= 0.0:
        cloud = 0.0
    else:
        air_mass = 1.0 / solar_noon_mu
        denominator = 0.7 * (0.75**air_mass - 0.4**air_mass)
        cloud = max(0.0, min(1.0, (0.3 + 0.7 * 0.75**air_mass - transmittance_seed) / denominator))
    parents = []
    for hour, (potential, mean_mu) in enumerate(potentials, start=1):
        energy = 0.0 if daily_potential == 0.0 else daily_energy * potential / daily_potential
        parents.append({
            "parent_hour_index": hour - 1,
            "horizontal_energy_mj_m2": energy,
            "global_horizontal_shortwave_w_m2": energy * 1_000_000.0 / 3600.0,
            "solar_zenith_cosine": mean_mu,
            "air_temperature_c": simimpl_hour_temperature(hour, halfday, tmax_c, tmin_c),
            "cloud_fraction": cloud,
        })
    if abs(math.fsum(parent["horizontal_energy_mj_m2"] for parent in parents) - daily_energy) > TOL * max(1.0, daily_energy):
        raise AuthorityDomain("simimpl:daily_energy_closure")
    return parents, daily_energy


def complete_day_receipt() -> dict[str, object]:
    provider_sha = "4658de9f7590897633ffbfe0facedd52b5c9b9754f7d829f25869ef2c592f153"
    climate_sha = "b" * 64
    parents, daily_energy = simimpl_parents(day_of_year=172, latitude_deg=41.1, radiation_ly=420.0, tmax_c=28.0, tmin_c=22.0)
    pressure = fao_pressure_kpa(1225.0)
    rain_masses, _ = event_relative_breakpoint_masses(13.25, [Segment(0.0, 2700.0, 2.0e-6)])
    intervals: list[dict[str, object]] = []
    for interval_index in range(48):
        parent = parents[interval_index // 2]
        dew_point_c = 20.0
        moisture = humidity(parent["air_temperature_c"], dew_point_c, pressure)
        phase = harder_pomeroy_hourly(parent["air_temperature_c"], dew_point_c)
        if rain_masses[interval_index] > 0.0 and phase["snow_fraction"] > 0.0:
            raise AuthorityDomain("precipitation:snow_or_mixed_phase")
        shortwave = weiss_norman(parent["global_horizontal_shortwave_w_m2"], parent["solar_zenith_cosine"], pressure)
        interval = {
            "provider_definition_sha256": provider_sha,
            "source_climate_sha256": climate_sha,
            "run_id": "authority-vector-run",
            "day_index": 0,
            "ofe_id": "ofe-1",
            "tile_id": "forest-1",
            "interval_index": interval_index,
            "transaction_id": f"authority-vector-run:0:{interval_index}",
            "start_s": 1800 * interval_index,
            "end_s": 1800 * (interval_index + 1),
            "parent_hour_index": interval_index // 2,
            "air_temperature_c": parent["air_temperature_c"],
            "dew_point_c": dew_point_c,
            "wind_m_s": 2.5,
            "pressure_kpa": pressure,
            "actual_vapor_pressure_kpa": moisture["actual_vapor_pressure_kpa"],
            "specific_humidity_kg_kg": moisture["specific_humidity_kg_kg"],
            "vpd_kpa": moisture["vpd_kpa"],
            "cloud_fraction": parent["cloud_fraction"],
            "solar_zenith_cosine": parent["solar_zenith_cosine"],
            "global_horizontal_shortwave_w_m2": parent["global_horizontal_shortwave_w_m2"],
            **shortwave,
            "downward_longwave_w_m2": atmospheric_longwave(parent["air_temperature_c"], moisture["actual_vapor_pressure_kpa"], parent["cloud_fraction"]),
            "co2_pa": 42.0,
            "reference_height_m": 2.0,
            "gsi": 0.75,
            "gsi_receipt_sha256": "c" * 64,
            "wb14_configuration_sha256": "d" * 64,
            "precipitation_parcels": [] if rain_masses[interval_index] == 0.0 else [{
                "parcel_id": f"climate-rain:0:{interval_index}",
                "source_owner_id": "climate-day-0",
                "destination_ofe_id": "ofe-1",
                "destination_tile_id": "forest-1",
                "start_s": 1800 * interval_index,
                "end_s": 1800 * (interval_index + 1),
                "mass_kg_m2": rain_masses[interval_index],
                "temperature_k": phase["hydrometeor_temperature_c"] + 273.15,
                "enthalpy_j_m2": rain_masses[interval_index] * CP_W * phase["hydrometeor_temperature_c"],
            }],
        }
        interval["interval_receipt_sha256"] = hashlib.sha256(canonical_bytes(interval)).hexdigest()
        intervals.append(interval)
    body: dict[str, object] = {
        "provider_version": "OPENWEPP_SNOW_FREE_HALF_HOUR_FORCING_V1",
        "provider_definition_sha256": provider_sha,
        "source_climate_sha256": climate_sha,
        "run_id": "authority-vector-run",
        "day_index": 0,
        "daily_horizontal_energy_mj_m2": daily_energy,
        "intervals": intervals,
        "next_day_precipitation_carry": [],
    }
    body["receipt_sha256"] = hashlib.sha256(canonical_bytes(body)).hexdigest()
    return body


def validate_day_receipt(receipt: dict[str, object]) -> None:
    if receipt["provider_version"] != "OPENWEPP_SNOW_FREE_HALF_HOUR_FORCING_V1":
        raise AuthorityDomain("receipt:provider_identity")
    intervals = receipt["intervals"]
    if not isinstance(intervals, list) or len(intervals) != 48:
        raise AuthorityDomain("receipt:support_identity")
    expected_provider = receipt["provider_definition_sha256"]
    expected_climate = receipt["source_climate_sha256"]
    seen: set[int] = set()
    for expected_index, interval in enumerate(intervals):
        if not isinstance(interval, dict):
            raise AuthorityDomain("receipt:interval_shape")
        index = interval.get("interval_index")
        if index in seen or index != expected_index:
            raise AuthorityDomain("receipt:support_identity")
        seen.add(index)
        if interval.get("start_s") != 1800 * expected_index or interval.get("end_s") != 1800 * (expected_index + 1):
            raise AuthorityDomain("receipt:support_identity")
        if interval.get("provider_definition_sha256") != expected_provider or interval.get("source_climate_sha256") != expected_climate:
            raise AuthorityDomain("receipt:provider_identity")
        supplied = interval.get("interval_receipt_sha256")
        digest_body = dict(interval)
        digest_body.pop("interval_receipt_sha256", None)
        if supplied != hashlib.sha256(canonical_bytes(digest_body)).hexdigest():
            raise AuthorityDomain("receipt:interval_digest")
        for parcel in interval["precipitation_parcels"]:
            expected_enthalpy = parcel["mass_kg_m2"] * CP_W * (parcel["temperature_k"] - 273.15)
            if abs(parcel["enthalpy_j_m2"] - expected_enthalpy) > TOL * max(1.0, abs(expected_enthalpy)):
                raise AuthorityDomain("receipt:parcel_enthalpy")
    supplied_day = receipt["receipt_sha256"]
    day_body = dict(receipt)
    day_body.pop("receipt_sha256", None)
    if supplied_day != hashlib.sha256(canonical_bytes(day_body)).hexdigest():
        raise AuthorityDomain("receipt:day_digest")


def poisoned_receipt_case(name: str, mutate) -> dict[str, str]:
    receipt = complete_day_receipt()
    mutate(receipt)
    return rejected(name, lambda: validate_day_receipt(receipt))


def accepted_case(name: str, *, global_w_m2: float, mu: float, elevation_m: float, air_c: float, dew_c: float, cloud: float) -> dict[str, object]:
    pressure = fao_pressure_kpa(elevation_m)
    moisture = humidity(air_c, dew_c, pressure)
    atmospheric_receipt = {
        "pressure_kpa": pressure,
        "humidity": moisture,
        "shortwave": weiss_norman(global_w_m2, mu, pressure),
        "downward_longwave_w_m2": atmospheric_longwave(air_c, moisture["actual_vapor_pressure_kpa"], cloud),
    }
    if moisture["vpd_kpa"] <= 0.0:
        return {"name": name, "status": "unsupported", "reason": "nonpositive_vpd", "lse_atmospheric_receipt": atmospheric_receipt}
    return {
        "name": name,
        "status": "accepted",
        **atmospheric_receipt,
    }


def rejected(name: str, operation) -> dict[str, str]:
    try:
        operation()
    except AuthorityDomain as error:
        return {"name": name, "status": "rejected", "reason": str(error)}
    raise AssertionError(f"{name} unexpectedly accepted")


def remove_last_interval(receipt: dict[str, object]) -> None:
    receipt["intervals"].pop()


def duplicate_first_interval(receipt: dict[str, object]) -> None:
    receipt["intervals"].insert(1, dict(receipt["intervals"][0]))


def change_provider(receipt: dict[str, object]) -> None:
    receipt["intervals"][7]["provider_definition_sha256"] = "e" * 64


def change_physical_operand(receipt: dict[str, object]) -> None:
    receipt["intervals"][7]["wind_m_s"] = 2.500_000_000_000_000_4


def reject_snow_phase() -> None:
    if harder_pomeroy_hourly(-2.0, -3.0)["snow_fraction"] > 0.0:
        raise AuthorityDomain("precipitation:snow_or_mixed_phase")
    raise AssertionError("cold phase unexpectedly contains no snow")


def digest_poison_matrix() -> dict[str, object]:
    interval = complete_day_receipt()["intervals"][27]
    fields = [field for field in interval if field != "interval_receipt_sha256"]
    original_body = {field: value for field, value in interval.items() if field != "interval_receipt_sha256"}
    original_digest = hashlib.sha256(canonical_bytes(original_body)).hexdigest()
    changed: list[str] = []
    for field in fields:
        poisoned = dict(original_body)
        value = poisoned[field]
        if isinstance(value, bool):
            poisoned[field] = not value
        elif isinstance(value, int):
            poisoned[field] = value + 1
        elif isinstance(value, float):
            poisoned[field] = math.nextafter(value, math.inf)
        elif isinstance(value, str):
            poisoned[field] = value + ":poison"
        elif isinstance(value, list):
            poisoned[field] = value + [{"poison": True}]
        else:
            raise AssertionError(f"unpoisonable receipt field {field}")
        if hashlib.sha256(canonical_bytes(poisoned)).hexdigest() == original_digest:
            raise AssertionError(f"digest unchanged for {field}")
        changed.append(field)
    return {"name": "digest_operand_matrix", "status": "accepted", "changed_field_count": len(changed), "changed_fields": changed}


def validate_global_homogeneity(receipts: list[dict[str, object]]) -> None:
    fields = (
        "air_temperature_c",
        "dew_point_c",
        "wind_m_s",
        "pressure_kpa",
        "actual_vapor_pressure_kpa",
        "specific_humidity_kg_kg",
        "vpd_kpa",
        "cloud_fraction",
        "solar_zenith_cosine",
        "global_horizontal_shortwave_w_m2",
        "direct_visible_w_m2",
        "diffuse_visible_w_m2",
        "direct_nir_w_m2",
        "diffuse_nir_w_m2",
        "downward_longwave_w_m2",
        "co2_pa",
        "reference_height_m",
    )
    for receipt in receipts[1:]:
        for first, candidate in zip(receipts[0]["intervals"], receipt["intervals"], strict=True):
            for field in fields:
                if candidate[field] != first[field]:
                    raise AuthorityDomain("receipt:unsupported_global_atmospheric_heterogeneity")


def heterogeneous_multi_ofe_case() -> dict[str, str]:
    first = complete_day_receipt()
    second = complete_day_receipt()
    second["intervals"][47]["downward_longwave_w_m2"] = math.nextafter(
        second["intervals"][47]["downward_longwave_w_m2"], math.inf
    )
    return rejected(
        "heterogeneous_multi_ofe",
        lambda: validate_global_homogeneity([first, second]),
    )


def schema_valid_midnight_carry() -> dict[str, object]:
    current, carry = event_relative_breakpoint_masses(23.5, [Segment(0.0, 3600.0, 2.0e-5)])
    phase = harder_pomeroy_hourly(24.0, 20.0)
    mass = RHO_W * (carry[0]["end_s"] - carry[0]["start_s"]) * carry[0]["intensity_m_s"]
    parcel = {
        "parcel_id": "climate-rain:carry:0",
        "source_owner_id": "climate-day-0",
        "destination_ofe_id": "ofe-1",
        "destination_tile_id": "forest-1",
        "start_s": carry[0]["start_s"],
        "end_s": carry[0]["end_s"],
        "mass_kg_m2": mass,
        "temperature_k": phase["hydrometeor_temperature_c"] + 273.15,
        "enthalpy_j_m2": mass * CP_W * phase["hydrometeor_temperature_c"],
    }
    return {"current_day_mass_kg_m2": math.fsum(current), "next_day_carry": [parcel]}


def payload() -> dict[str, object]:
    storm = breakpoint_child_masses([
        Segment(1500.0, 2400.0, 1.0e-5),
        Segment(84_900.0, 86_400.0, 2.0e-5),
    ])
    midnight = schema_valid_midnight_carry()
    cases: list[dict[str, object]] = [
        accepted_case("dry_clear_summer", global_w_m2=700.0, mu=0.82, elevation_m=250.0, air_c=28.0, dew_c=10.0, cloud=0.05),
        accepted_case("dry_cloudy", global_w_m2=180.0, mu=0.55, elevation_m=250.0, air_c=18.0, dew_c=8.0, cloud=0.85),
        accepted_case("night", global_w_m2=0.0, mu=0.0, elevation_m=250.0, air_c=8.0, dew_c=3.0, cloud=0.4),
        accepted_case("dawn", global_w_m2=25.0, mu=0.12, elevation_m=1200.0, air_c=6.0, dew_c=1.0, cloud=0.3),
        accepted_case("low_transmissivity", global_w_m2=0.1, mu=0.5, elevation_m=250.0, air_c=10.0, dew_c=0.0, cloud=0.95),
        accepted_case("low_pressure", global_w_m2=500.0, mu=0.7, elevation_m=3000.0, air_c=20.0, dew_c=5.0, cloud=0.2),
        accepted_case("humid_positive_vpd", global_w_m2=400.0, mu=0.65, elevation_m=50.0, air_c=22.0, dew_c=21.0, cloud=0.6),
        accepted_case("dewpoint_equal_air", global_w_m2=300.0, mu=0.6, elevation_m=50.0, air_c=15.0, dew_c=15.0, cloud=0.5),
        accepted_case("dewpoint_above_air", global_w_m2=300.0, mu=0.6, elevation_m=50.0, air_c=14.0, dew_c=15.0, cloud=0.5),
        rejected("positive_shortwave_zero_mu", lambda: weiss_norman(1.0, 0.0, 100.0)),
        rejected("undefined_pressure", lambda: fao_pressure_kpa(50_000.0)),
        {"name": "parent_hour_refinement", "status": "accepted", **parent_hour_children(1.8)},
        {"name": "breakpoint_cross_half_hour_and_midnight", "status": "accepted", "nonzero_children": [[index, mass] for index, mass in enumerate(storm) if mass != 0.0], "daily_mass_kg_m2": math.fsum(storm)},
        {"name": "event_relative_midnight_carry", "status": "accepted", **midnight},
        {"name": "fallback_parent_hour_split", "status": "accepted", "parent_mass_kg_m2": 2.5, "child_masses_kg_m2": [1.25, 1.25]},
        {"name": "liquid_enthalpy", "status": "accepted", "mass_kg_m2": 2.0, "temperature_k": 278.15, "enthalpy_j_m2": 2.0 * CP_W * 5.0},
        {"name": "support_continuity", "status": "accepted", "supports": [[1800 * index, 1800 * (index + 1)] for index in range(48)]},
        {"name": "zero_wind", "status": "rejected", "reason": "unsupported_aerodynamic_domain"},
        poisoned_receipt_case("missing_interval", remove_last_interval),
        poisoned_receipt_case("duplicate_interval", duplicate_first_interval),
        poisoned_receipt_case("mixed_provider_version", change_provider),
        poisoned_receipt_case("one_bit_physical_operand", change_physical_operand),
        heterogeneous_multi_ofe_case(),
        rejected("snow_or_mixed_phase", reject_snow_phase),
        digest_poison_matrix(),
    ]
    complete_receipt = complete_day_receipt()
    validate_day_receipt(complete_receipt)
    return {
        "schema": "openwepp.snow_free_half_hour_forcing.authority_vectors.v1",
        "model_version": "OPENWEPP_SNOW_FREE_HALF_HOUR_FORCING_V1",
        "interval_s": 1800,
        "interval_count": 48,
        "complete_day_receipt": complete_receipt,
        "cases": cases,
    }


def main() -> int:
    json.dump(payload(), sys.stdout, sort_keys=True, separators=(",", ":"), allow_nan=False)
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
