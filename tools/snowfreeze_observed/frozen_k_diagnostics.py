#!/usr/bin/env python3
"""Diagnostic SFCC and frozen hydraulic-conductivity comparison surfaces.

This tool is intentionally diagnostic-only. It does not feed the openWEPP
runtime, does not select texture defaults, and does not authorize Qwet or
frozen-conductivity production changes.
"""

from __future__ import annotations

import argparse
import json
import math
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any


SCHEMA = "snowfrost-fidelity-c-frozen-k-diagnostics-v1"
PROMOTION_STATUS = "diagnostic_only_not_runtime_authority"
FREEZING_K = 273.15
LATENT_HEAT_FUSION_J_KG = 334_000.0
GRAVITY_M_S2 = 9.80665
DEFAULT_TEMPERATURES_C = [0.0, -0.1, -0.5, -1.0, -2.0, -5.0, -10.0]


@dataclass(frozen=True)
class DiagnosticSoil:
    soil_id: str
    description: str
    theta_residual_m3_m3: float
    theta_saturated_m3_m3: float
    alpha_m_inv: float
    n: float
    ksat_m_s: float
    impedance_beta: float
    provenance: str = "diagnostic_fixture_not_texture_default"


DIAGNOSTIC_SOILS = [
    DiagnosticSoil(
        soil_id="coarse_diagnostic_fixture",
        description="Coarse illustrative retention curve for sensitivity screening.",
        theta_residual_m3_m3=0.045,
        theta_saturated_m3_m3=0.43,
        alpha_m_inv=3.5,
        n=2.0,
        ksat_m_s=1.0e-5,
        impedance_beta=1.0,
    ),
    DiagnosticSoil(
        soil_id="medium_diagnostic_fixture",
        description="Medium illustrative retention curve for sensitivity screening.",
        theta_residual_m3_m3=0.078,
        theta_saturated_m3_m3=0.43,
        alpha_m_inv=1.6,
        n=1.55,
        ksat_m_s=2.0e-6,
        impedance_beta=2.0,
    ),
    DiagnosticSoil(
        soil_id="fine_diagnostic_fixture",
        description="Fine illustrative retention curve for sensitivity screening.",
        theta_residual_m3_m3=0.10,
        theta_saturated_m3_m3=0.50,
        alpha_m_inv=0.8,
        n=1.32,
        ksat_m_s=2.0e-7,
        impedance_beta=3.0,
    ),
]

REFERENCES = [
    "Dun et al. 2010 WEPP frost lineage and source-conflict boundary",
    "Kurylyk and Watanabe 2013 Clapeyron/SFCC review",
    "Watanabe and Flury 2008 capillary-bundle frozen conductivity",
    "Azmatch et al. 2012 SFCC-derived hydraulic conductivity",
    "Ming et al. 2020 saturated frozen conductivity from SFCC",
    "Cheng et al. 2023 impedance-factor interpretation",
    "Amankwah et al. 2021 salinity/SFCC limitation",
    "Devoie et al. 2022 measured SFCC repository as parameter-data source",
]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    parser.add_argument(
        "--temperatures-c",
        nargs="*",
        type=float,
        default=DEFAULT_TEMPERATURES_C,
        help="Temperature grid in deg C; defaults to the package diagnostic grid.",
    )
    args = parser.parse_args()

    payload = build_diagnostics(args.temperatures_c)
    write_json(args.output_json.resolve(), payload)
    args.output_md.resolve().write_text(render_markdown(payload), encoding="utf-8")
    return 0


def build_diagnostics(temperatures_c: list[float]) -> dict[str, Any]:
    normalized_temperatures = sorted(set(float(value) for value in temperatures_c), reverse=True)
    samples = []
    for soil in DIAGNOSTIC_SOILS:
        for temperature_c in normalized_temperatures:
            samples.append(sample_soil(soil, temperature_c, salinity_freezing_depression_c=0.0))

    salinity_sensitivity = []
    for soil in DIAGNOSTIC_SOILS:
        fresh = sample_soil(soil, -0.5, salinity_freezing_depression_c=0.0)
        saline = sample_soil(soil, -0.5, salinity_freezing_depression_c=0.25)
        salinity_sensitivity.append(
            {
                "soil_id": soil.soil_id,
                "temperature_c": -0.5,
                "fresh_liquid_water_m3_m3": fresh["liquid_water_m3_m3"],
                "saline_liquid_water_m3_m3": saline["liquid_water_m3_m3"],
                "salinity_freezing_depression_c": 0.25,
                "model_role": "Amankwah-style salinity sensitivity diagnostic, not production salinity model",
            }
        )

    return {
        "schema": SCHEMA,
        "promotion_status": PROMOTION_STATUS,
        "model_family": "diagnostic_sfcc_frozen_k_screening",
        "runtime_coupling": "none",
        "qwet_authority": "not_authorized",
        "parameter_status": "diagnostic_fixture_not_texture_default",
        "references": REFERENCES,
        "temperatures_c": normalized_temperatures,
        "soils": [asdict(soil) for soil in DIAGNOSTIC_SOILS],
        "samples": samples,
        "salinity_sensitivity": salinity_sensitivity,
        "notes": [
            "Clapeyron pressure head drives a van Genuchten-style SFCC diagnostic.",
            "SFCC-Mualem and capillary-bundle outputs are comparison surfaces only.",
            "Impedance-scaled values are diagnostic and do not imply universal impedance requirement.",
            "No sample may be consumed by production runtime without a later SC-SNOWFREEZE-001 amendment.",
        ],
    }


def sample_soil(
    soil: DiagnosticSoil,
    temperature_c: float,
    salinity_freezing_depression_c: float,
) -> dict[str, Any]:
    effective_temperature_c = min(0.0, temperature_c + salinity_freezing_depression_c)
    pressure_head_m = clapeyron_unfrozen_pressure_head_m(effective_temperature_c)
    effective_saturation = van_genuchten_effective_saturation(
        pressure_head_m,
        soil.alpha_m_inv,
        soil.n,
    )
    liquid_water = soil.theta_residual_m3_m3 + (
        soil.theta_saturated_m3_m3 - soil.theta_residual_m3_m3
    ) * effective_saturation
    sfcc_mualem_rel = mualem_relative_conductivity(effective_saturation, soil.n)
    capillary_bundle_rel = capillary_bundle_screening_ratio(effective_saturation)
    impedance_factor = math.exp(-soil.impedance_beta * (1.0 - effective_saturation))
    impedance_scaled_rel = sfcc_mualem_rel * impedance_factor

    return {
        "soil_id": soil.soil_id,
        "temperature_c": temperature_c,
        "salinity_freezing_depression_c": salinity_freezing_depression_c,
        "effective_temperature_c": effective_temperature_c,
        "clapeyron_pressure_head_m": pressure_head_m,
        "liquid_water_m3_m3": liquid_water,
        "effective_saturation": effective_saturation,
        "sfcc_mualem_k_rel": sfcc_mualem_rel,
        "sfcc_mualem_k_m_s": sfcc_mualem_rel * soil.ksat_m_s,
        "watanabe_flury_capillary_bundle_screening_k_rel": capillary_bundle_rel,
        "cheng_impedance_factor": impedance_factor,
        "impedance_scaled_k_rel": impedance_scaled_rel,
        "impedance_scaled_k_m_s": impedance_scaled_rel * soil.ksat_m_s,
        "promotion_status": PROMOTION_STATUS,
        "parameter_status": soil.provenance,
    }


def clapeyron_unfrozen_pressure_head_m(temperature_c: float) -> float:
    if temperature_c >= 0.0:
        return 0.0
    temperature_k = FREEZING_K + temperature_c
    if temperature_k <= 0.0:
        raise ValueError(f"temperature below physical Kelvin domain: {temperature_c}")
    return LATENT_HEAT_FUSION_J_KG * (FREEZING_K - temperature_k) / (
        GRAVITY_M_S2 * temperature_k
    )


def van_genuchten_effective_saturation(pressure_head_m: float, alpha_m_inv: float, n: float) -> float:
    if alpha_m_inv <= 0.0 or n <= 1.0:
        raise ValueError("van Genuchten diagnostic parameters require alpha > 0 and n > 1")
    if pressure_head_m <= 0.0:
        return 1.0
    m = 1.0 - 1.0 / n
    saturation = (1.0 + (alpha_m_inv * pressure_head_m) ** n) ** (-m)
    return clamp_unit_interval(saturation)


def mualem_relative_conductivity(effective_saturation: float, n: float) -> float:
    se = clamp_unit_interval(effective_saturation)
    if se <= 0.0:
        return 0.0
    if se >= 1.0:
        return 1.0
    m = 1.0 - 1.0 / n
    term = 1.0 - (1.0 - se ** (1.0 / m)) ** m
    return clamp_unit_interval(math.sqrt(se) * term * term)


def capillary_bundle_screening_ratio(effective_saturation: float) -> float:
    se = clamp_unit_interval(effective_saturation)
    return se**3


def clamp_unit_interval(value: float) -> float:
    if not math.isfinite(value):
        raise ValueError(f"non-finite diagnostic value: {value}")
    return min(1.0, max(0.0, value))


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def render_markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# SFCC Frozen-K Diagnostics",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Schema: `{payload['schema']}`",
        f"- Promotion status: `{payload['promotion_status']}`",
        f"- Runtime coupling: `{payload['runtime_coupling']}`",
        f"- Qwet authority: `{payload['qwet_authority']}`",
        "",
        "## References",
        "",
    ]
    for reference in payload["references"]:
        lines.append(f"- {reference}")
    lines.extend(
        [
            "",
            "## Samples",
            "",
            "| Soil | Temp C | Liquid water | SFCC-Mualem Krel | Impeded Krel | Capillary screening Krel |",
            "| --- | ---: | ---: | ---: | ---: | ---: |",
        ]
    )
    for sample in payload["samples"]:
        lines.append(
            "| {soil} | {temp:.2f} | {liq:.8f} | {mualem:.8e} | {impeded:.8e} | {cap:.8e} |".format(
                soil=sample["soil_id"],
                temp=sample["temperature_c"],
                liq=sample["liquid_water_m3_m3"],
                mualem=sample["sfcc_mualem_k_rel"],
                impeded=sample["impedance_scaled_k_rel"],
                cap=sample["watanabe_flury_capillary_bundle_screening_k_rel"],
            )
        )
    lines.extend(
        [
            "",
            "## Disposition",
            "",
            "These outputs are diagnostic comparison surfaces only. They are not texture defaults, runtime authority, field calibration, or Qwet authorization.",
            "",
        ]
    )
    return "\n".join(lines)


if __name__ == "__main__":
    raise SystemExit(main())
