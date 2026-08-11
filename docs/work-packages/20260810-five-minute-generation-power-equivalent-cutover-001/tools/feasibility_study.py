#!/usr/bin/env python3
"""Prospective package-local power-equivalent feasibility study.

This diagnostic mirrors the production rill-width, Chezy/shear, Shields/Yalin,
and normalized Wave-1 driver equations needed to test a fixed exponent. It is
not production physics and cannot be imported by runtime crates.
"""

from __future__ import annotations

import argparse
import json
import math
import statistics
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Iterable

DT_S = 300.0
HOUR_S = 3600.0
MATERIAL_RATE_M_S = 1.0e-7
GRAVITY = 9.807
WATER_WEIGHT_N_M3 = 9807.0
KINEMATIC_VISCOSITY_M2_S = 1.0e-6
SEDIMENT_DENSITY_KG_M3 = 1000.0


@dataclass(frozen=True)
class Particle:
    diameter_m: float
    specific_gravity: float
    fraction: float


@dataclass(frozen=True)
class State:
    name: str
    slope: float
    end_slope: float
    width_seed_m: float
    rill_spacing_m: float
    rill_cover: float
    live_friction: float
    effective_length_m: float
    sand_fraction: float


@dataclass(frozen=True)
class Response:
    width_m: float
    shear_pa: float
    transport_kg_m_s: float
    detachment_driver: float
    deposition_driver_s_m: float


def percentile(values: Iterable[float], probability: float) -> float:
    ordered = sorted(values)
    if not ordered:
        return 0.0
    position = probability * (len(ordered) - 1)
    lower = math.floor(position)
    upper = math.ceil(position)
    if lower == upper:
        return ordered[lower]
    weight = position - lower
    return ordered[lower] * (1.0 - weight) + ordered[upper] * weight


def topanga_particles() -> tuple[Particle, ...]:
    """WEPP `prtcmp` fractions for the frozen Topanga 25/30/45 texture."""
    sand, clay, silt = 0.25, 0.30, 0.45
    diameters_mm = [0.002, 0.010, 0.20 * (clay - 0.25) + 0.030, 2.0 * clay, 0.200]
    specific_gravity = [2.60, 2.65, 1.80, 1.60, 2.65]
    frac1 = 0.26 * clay
    frac5 = sand * (1.0 - clay) ** 5
    frac3 = 0.45 - 0.6 * (clay - 0.25)
    frcly3 = clay / (clay + silt)
    fractions: list[float] = []
    for pass_index in range(2):
        frac2 = silt - frac3
        if frac2 <= 0.0:
            frac2 = 0.0001
            frac3 = max(silt - frac2, 0.0001)
        frac4 = 1.0 - frac1 - frac2 - frac3 - frac5
        fractions = [frac1, frac2, frac3, frac4, frac5]
        if frac4 <= 0.0:
            correction = 1.0 / (1.0 + abs(frac4) + 0.0001)
            fractions[3] = 0.0001
            fractions = [value * correction for value in fractions]
            frac4 = fractions[3]
        if pass_index == 1:
            break
        frcly4 = (clay - fractions[0] - frcly3 * fractions[2]) / frac4
        if not 0.0 <= frcly4 <= 1.0:
            frcly4 = 0.0
        target = 0.5 * clay
        if frcly4 < 0.95 * target and abs(frcly3 - target) > 0.0:
            f1f2f5 = fractions[0] + fractions[1] + fractions[4]
            frac3 = (clay - target - fractions[0] + target * f1f2f5) / (frcly3 - target)
            frac3 = max(frac3, 0.0001)
            continue
        break
    return tuple(
        Particle(diameter / 1000.0, gravity, fraction)
        for diameter, gravity, fraction in zip(diameters_mm, specific_gravity, fractions)
    )


def shield(reynolds: float) -> float:
    y = [0.0772, 0.0579, 0.04, 0.035, 0.034, 0.045, 0.055, 0.057]
    r = [1.0, 2.0, 4.0, 8.0, 12.0, 100.0, 400.0, 1000.0]
    if reynolds <= 0.0:
        raise ValueError("positive particle Reynolds number required")
    if reynolds < r[0]:
        slope = (math.log(y[1]) - math.log(y[0])) / (math.log(r[1]) - math.log(r[0]))
        value = math.log(y[0]) - slope * (math.log(r[0]) - math.log(reynolds))
        return math.exp(value)
    if reynolds > r[-1]:
        slope = (math.log(y[-1]) - math.log(y[-2])) / (math.log(r[-1]) - math.log(r[-2]))
        return math.exp(y[-1] + slope * (math.log(reynolds) - math.log(r[-1])))
    for index in range(1, len(r)):
        if r[index - 1] <= reynolds <= r[index]:
            slope = (math.log(y[index]) - math.log(y[index - 1])) / (
                math.log(r[index]) - math.log(r[index - 1])
            )
            return math.exp(math.log(y[index - 1]) + slope * (math.log(reynolds) - math.log(r[index - 1])))
    raise AssertionError("unreachable Reynolds interval")


def yalin(shear_pa: float, particles: tuple[Particle, ...], sand_fraction: float) -> float:
    if shear_pa <= 0.0:
        raise ValueError("positive shear required")
    vstar = math.sqrt(shear_pa / SEDIMENT_DENSITY_KG_M3)
    deltas: list[float] = []
    probabilities: list[float] = []
    total_delta = 0.0
    for particle in particles:
        reynolds = vstar * particle.diameter_m / KINEMATIC_VISCOSITY_M2_S
        critical = shield(reynolds)
        delta = vstar * vstar / (
            (particle.specific_gravity - 1.0) * GRAVITY * particle.diameter_m * critical
        ) - 1.0
        if delta > 0.0:
            sigma = delta * 2.45 * particle.specific_gravity ** -0.4 * math.sqrt(critical)
            probability = 0.635 * delta * (1.0 - math.log1p(sigma) / sigma)
            total_delta += delta
        else:
            delta = 0.0
            probability = 0.0
        deltas.append(delta)
        probabilities.append(probability)
    divisor = total_delta if total_delta > 0.0 else 1000.0
    capacity = 0.0
    for particle, delta, probability in zip(particles, deltas, probabilities):
        coefficient = vstar * SEDIMENT_DENSITY_KG_M3 * particle.diameter_m * particle.specific_gravity
        capacity += probability * (delta / divisor) * coefficient * (particle.fraction * len(particles))
    if sand_fraction > 0.5:
        capacity *= max(0.3 + 0.7 * math.exp(-12.52 * (sand_fraction - 0.5)), 0.30)
    return max(capacity, 0.0)


def hydraulics(rate_m_s: float, state: State, particles: tuple[Particle, ...]) -> Response:
    qout_m2_s = rate_m_s * state.effective_length_m
    qshear_m2_s = qout_m2_s * state.rill_spacing_m
    friction_soil = 1.11
    friction_cover = 4.5 * state.rill_cover ** 1.5544 if state.rill_cover > 0.0 else 0.0
    friction_control = friction_soil + friction_cover + state.live_friction
    width_m = max(state.width_seed_m, 1.13 * abs(qshear_m2_s) ** 0.303)
    width_m = min(width_m, state.rill_spacing_m)
    chezy = math.sqrt(8.0 * GRAVITY / friction_control)
    slope = max(state.slope, 1.0e-6)
    if qshear_m2_s <= 0.0:
        depth_m = 0.0
    else:
        u = (qshear_m2_s / chezy / math.sqrt(slope)) ** (2.0 / 3.0) / width_m
        depth_m = 0.2 * qshear_m2_s ** 0.36
        for _ in range(1000):
            previous = depth_m
            depth_m = u * (width_m + 2.0 * previous) ** (1.0 / 3.0)
            if abs(previous / depth_m - 1.0) <= 5.0e-6:
                break
        else:
            raise RuntimeError("Chezy depth iteration did not converge")
    hydraulic_radius_m = depth_m * width_m / (width_m + 2.0 * depth_m)
    shear_pa = max(
        WATER_WEIGHT_N_M3
        * math.sin(math.atan(slope))
        * hydraulic_radius_m
        * friction_soil
        / friction_control,
        1.0e-6,
    )
    capacity = max(yalin(shear_pa, particles, state.sand_fraction), 1.0e-10)
    detachment_driver = shear_pa / capacity
    deposition_driver = 1.0 / qout_m2_s if qout_m2_s > 0.0 else 0.0
    return Response(width_m, shear_pa, capacity, detachment_driver, deposition_driver)


def shapes() -> dict[str, tuple[float, ...]]:
    base = 2.0e-6
    return {
        "constant": tuple([base] * 12),
        "one_pulse": tuple([12.0 * base] + [0.0] * 11),
        "two_pulses": tuple([6.0 * base, 0.0, 0.0, 0.0, 0.0, 6.0 * base] + [0.0] * 6),
        "rising": tuple(base * (index + 1) / 6.5 for index in range(12)),
        "falling": tuple(base * (12 - index) / 6.5 for index in range(12)),
        "triangle": tuple(base * value / 3.5 for value in [1, 2, 3, 4, 5, 6, 6, 5, 4, 3, 2, 1]),
        "early_ponding": tuple([0.0, 0.0] + [2.4 * base] * 5 + [0.0] * 5),
        "late_ponding": tuple([0.0] * 7 + [2.4 * base] * 5),
        "bin_spanning": tuple([0.0, 3.0 * base, 6.0 * base, 3.0 * base] + [0.0] * 8),
        "hour_spanning": tuple([0.0] * 9 + [4.0 * base] * 3),
        "saturation_background": tuple([0.5 * base] * 12),
        "near_floor": tuple([0.11e-6] * 12),
    }


def states() -> tuple[State, ...]:
    states_out: list[State] = []
    for cover_name, cover, live in [("burned", 0.10, 0.0), ("unburned", 1.0, 1.0)]:
        for slope_name, slope in [("low", 0.04), ("topanga", 0.3267), ("high", 0.60)]:
            for width_name, width in [("new", 0.001), ("developed", 0.10)]:
                states_out.append(
                    State(
                        name=f"{cover_name}-{slope_name}-{width_name}",
                        slope=slope,
                        end_slope=max(0.02, slope * 0.5),
                        width_seed_m=width,
                        rill_spacing_m=1.0,
                        rill_cover=cover,
                        live_friction=live,
                        effective_length_m=242.1,
                        sand_fraction=0.25,
                    )
                )
    return tuple(states_out)


def reduction(rates: tuple[float, ...], exponent: float, method: str) -> tuple[float, float]:
    volume_m = sum(rate * DT_S for rate in rates)
    if volume_m == 0.0:
        return 0.0, 0.0
    power_integral = sum(rate**exponent * DT_S for rate in rates)
    if method == "fixed_hour":
        return (power_integral / HOUR_S) ** (1.0 / exponent), HOUR_S
    if method == "power_volume":
        if exponent <= 1.0:
            raise ValueError("power-volume reduction requires exponent > 1")
        rate = (power_integral / volume_m) ** (1.0 / (exponent - 1.0))
        return rate, volume_m / rate
    raise ValueError(f"unknown method: {method}")


def integrate_response(
    rates: tuple[float, ...], state: State, particles: tuple[Particle, ...], field: str
) -> float:
    return sum(getattr(hydraulics(rate, state, particles), field) * DT_S for rate in rates if rate > 0.0)


def relative_error(candidate: float, reference: float) -> float:
    if reference == 0.0:
        return 0.0 if candidate == 0.0 else math.inf
    return abs(candidate - reference) / abs(reference)


def run_study() -> dict[str, object]:
    particle_set = topanga_particles()
    response_fields = [
        "width_m",
        "shear_pa",
        "transport_kg_m_s",
        "detachment_driver",
        "deposition_driver_s_m",
    ]
    records: list[dict[str, object]] = []
    for shape_name, rates in shapes().items():
        if max(rates) < MATERIAL_RATE_M_S:
            continue
        volume = sum(rate * DT_S for rate in rates)
        for state in states():
            references = {
                field: integrate_response(rates, state, particle_set, field) for field in response_fields
            }
            reference_transport_branches = sum(
                hydraulics(rate, state, particle_set).transport_kg_m_s > 1.0e-10
                for rate in rates
                if rate > 0.0
            )
            for exponent in [1.0, 4.0 / 3.0, 1.5, 2.0]:
                methods = ["fixed_hour"] if exponent == 1.0 else ["fixed_hour", "power_volume"]
                for method in methods:
                    rate, duration = reduction(rates, exponent, method)
                    candidate_response = hydraulics(rate, state, particle_set)
                    errors = {
                        field: relative_error(
                            getattr(candidate_response, field) * duration, references[field]
                        )
                        for field in response_fields
                    }
                    volume_error = relative_error(rate * duration, volume)
                    power_reference = sum(value**exponent * DT_S for value in rates)
                    power_error = relative_error(rate**exponent * duration, power_reference)
                    records.append(
                        {
                            "shape": shape_name,
                            "state": state.name,
                            "exponent": exponent,
                            "method": method,
                            "equivalent_rate_m_s": rate,
                            "duration_s": duration,
                            "volume_relative_error": volume_error,
                            "power_relative_error": power_error,
                            "transport_branch_change": (
                                reference_transport_branches > 0
                            ) != (candidate_response.transport_kg_m_s > 1.0e-10),
                            "response_relative_errors": errors,
                        }
                    )
    summaries: list[dict[str, object]] = []
    candidates = sorted({(float(r["exponent"]), str(r["method"])) for r in records})
    for exponent, method in candidates:
        subset = [r for r in records if r["exponent"] == exponent and r["method"] == method]
        field_summary: dict[str, object] = {}
        for field in response_fields:
            values = [float(r["response_relative_errors"][field]) for r in subset]  # type: ignore[index]
            field_summary[field] = {
                "median": statistics.median(values),
                "p95": percentile(values, 0.95),
                "maximum": max(values),
            }
        pass_fields = all(
            float(metrics["median"]) <= 0.05
            and float(metrics["p95"]) <= 0.15
            and float(metrics["maximum"]) <= 0.30
            for metrics in field_summary.values()  # type: ignore[union-attr]
        )
        volume_max = max(float(r["volume_relative_error"]) for r in subset)
        power_max = max(float(r["power_relative_error"]) for r in subset)
        summaries.append(
            {
                "exponent": exponent,
                "method": method,
                "sample_count": len(subset),
                "volume_error_max": volume_max,
                "power_error_max": power_max,
                "transport_branch_changes": sum(bool(r["transport_branch_change"]) for r in subset),
                "responses": field_summary,
                "passes_screened_response_thresholds": pass_fields,
                "structurally_volume_admissible": volume_max <= 1.0e-12,
            }
        )
    admitted = [
        summary
        for summary in summaries
        if summary["method"] == "power_volume"
        and summary["structurally_volume_admissible"]
        and summary["passes_screened_response_thresholds"]
    ]
    return {
        "schema": "openwepp-five-minute-feasibility-v1",
        "prospective": True,
        "topanga_outcomes_opened": False,
        "time_step_s": DT_S,
        "material_rate_floor_m_s": MATERIAL_RATE_M_S,
        "shape_count": len(shapes()),
        "state_count": len(states()),
        "record_count": len(records),
        "topanga_input_texture": {"sand": 0.25, "clay": 0.30, "silt": 0.45},
        "scope_limitations": [
            "no end-slope shear, kt2, or ktrato response",
            "no critical-shear, erodibility, qin, or full continuity branch state",
            "transport branch flag is a coarse any-bin versus rectangle comparison",
            "rill width is a reset-seed sensitivity diagnostic, not persistent chronology",
            "no Rust-to-Python parity harness",
        ],
        "summaries": summaries,
        "admitted_candidates": admitted,
        "disposition": "NO_FIXED_EXPONENT_ADMITTED" if not admitted else "CANDIDATE_ADMITTED",
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = run_study()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({key: result[key] for key in ["record_count", "disposition"]}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
