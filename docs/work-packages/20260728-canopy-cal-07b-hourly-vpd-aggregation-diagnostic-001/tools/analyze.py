#!/usr/bin/env python3
"""Reconstruct CAL-07B hourly-product and daily VPD diagnostics."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import statistics
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
SOURCE = PKG / "inputs" / "source"
ART = PKG / "artifacts"
CAL07 = (
    ROOT
    / "docs/work-packages"
    / "20260728-canopy-cal-07-southern-hemisphere-robustness-001"
)
RETRIEVED = "2026-07-28T23:05:29-07:00"
CASES = {
    "2022-07-22": "power_hourly_alerce_20220722_lst.json",
    "2022-09-15": "power_hourly_alerce_20220915_lst.json",
    "2025-09-09": "power_hourly_alerce_20250909_lst.json",
}
URLS = {
    "power_hourly_alerce_20220722_lst.json": "https://power.larc.nasa.gov/api/temporal/hourly/point?parameters=T2M,T2MDEW&community=AG&longitude=-73.4439&latitude=-40.1726&start=20220722&end=20220722&format=JSON&time-standard=LST",
    "power_hourly_alerce_20220915_lst.json": "https://power.larc.nasa.gov/api/temporal/hourly/point?parameters=T2M,T2MDEW&community=AG&longitude=-73.4439&latitude=-40.1726&start=20220915&end=20220915&format=JSON&time-standard=LST",
    "power_hourly_alerce_20250909_lst.json": "https://power.larc.nasa.gov/api/temporal/hourly/point?parameters=T2M,T2MDEW&community=AG&longitude=-73.4439&latitude=-40.1726&start=20250909&end=20250909&format=JSON&time-standard=LST",
    "power_hourly_api_20260728.html": "https://power.larc.nasa.gov/docs/services/api/temporal/hourly/",
    "power_daily_api_20260728.html": "https://power.larc.nasa.gov/docs/services/api/temporal/daily/",
    "power_temporal_processing_20260728.html": "https://power.larc.nasa.gov/docs/methodology/data/processing/",
    "power_meteorology_20260728.html": "https://power.larc.nasa.gov/docs/methodology/meteorology/",
    "power_time_faq_20260728.html": "https://power.larc.nasa.gov/docs/faqs/other/",
}


def e_pa(temperature_c: float) -> float:
    return 1_000.0 * 0.6108 * math.exp(
        17.27 * temperature_c / (temperature_c + 237.3)
    )


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def write_csv(
    path: Path, fields: tuple[str, ...], data: list[dict[str, object]]
) -> None:
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        writer.writerows(data)


def daily_forcing() -> dict[str, dict[str, str]]:
    return {
        row["date"]: row
        for row in read_csv(CAL07 / "inputs" / "forcing.csv")
        if row["site_id"] == "SH-EN-ALERCE" and row["date"] in CASES
    }


def main() -> None:
    reported = daily_forcing()
    if set(reported) != set(CASES):
        raise ValueError("CAL-07 daily case custody is incomplete")
    daily_payload = json.loads(
        (
            CAL07
            / "inputs/source/power_alercecosteroforest_20220101_20260724.json"
        ).read_text(encoding="utf-8")
    )
    daily_header = daily_payload["header"]
    daily_geometry = daily_payload["geometry"]["coordinates"]
    daily_sources = set(daily_header["sources"])

    hourly_rows: list[dict[str, object]] = []
    daily_rows: list[dict[str, object]] = []
    attribution_rows: list[dict[str, object]] = []
    for case_date, filename in CASES.items():
        path = SOURCE / filename
        payload = json.loads(path.read_text(encoding="utf-8"))
        header = payload["header"]
        parameters = payload["parameters"]
        values = payload["properties"]["parameter"]
        compact = case_date.replace("-", "")
        expected_keys = [f"{compact}{hour:02d}" for hour in range(24)]
        t_keys = sorted(values["T2M"])
        d_keys = sorted(values["T2MDEW"])
        inventory_valid = t_keys == expected_keys and d_keys == expected_keys
        metadata_valid = (
            header["time_standard"] == "LST"
            and header["start"] == compact
            and header["end"] == compact
            and header["fill_value"] == -999.0
            and parameters["T2M"]["units"] == "C"
            and parameters["T2MDEW"]["units"] == "C"
            and payload["geometry"]["coordinates"] == daily_geometry
        )
        hourly_sources = set(header["sources"])
        if hourly_sources == daily_sources:
            lineage = "EXACT_SOURCE_LIST_MATCH"
        elif hourly_sources & daily_sources:
            lineage = "AGGREGATE_OVERLAP_ONLY"
        elif hourly_sources.isdisjoint(daily_sources):
            lineage = "DISJOINT"
        else:
            lineage = "NOT_RESOLVED_FROM_RESPONSE_METADATA"
        lineage_compatible = lineage != "DISJOINT"
        if not inventory_valid:
            raise ValueError(f"invalid hour inventory for {case_date}")

        temperatures: list[float] = []
        dewpoints: list[float] = []
        vpds: list[float] = []
        for key in expected_keys:
            temperature = float(values["T2M"][key])
            dewpoint = float(values["T2MDEW"][key])
            if any(
                not math.isfinite(value) or value == -999.0
                for value in (temperature, dewpoint)
            ):
                raise ValueError(f"invalid hourly source value at {key}")
            vpd = e_pa(temperature) - e_pa(dewpoint)
            rounding_min = e_pa(temperature - 0.005) - e_pa(dewpoint + 0.005)
            rounding_max = e_pa(temperature + 0.005) - e_pa(dewpoint - 0.005)
            if rounding_max < 0.0:
                sensitivity = "NEGATIVE_ACROSS_HALF_UNIT_RANGE"
            elif rounding_min >= 0.0:
                sensitivity = "NONNEGATIVE_ACROSS_HALF_UNIT_RANGE"
            else:
                sensitivity = "SIGN_SENSITIVE_TO_HALF_UNIT_RANGE"
            temperatures.append(temperature)
            dewpoints.append(dewpoint)
            vpds.append(vpd)
            hourly_rows.append(
                {
                    "date": case_date,
                    "hour_key": key,
                    "hour_lst": key[-2:],
                    "t2m_c": f"{temperature:.17g}",
                    "t2mdew_c": f"{dewpoint:.17g}",
                    "tdew_minus_t2m_c": f"{dewpoint-temperature:.17g}",
                    "hourly_product_vpd_pa": f"{vpd:.17g}",
                    "half_unit_vpd_min_pa": f"{rounding_min:.17g}",
                    "half_unit_vpd_max_pa": f"{rounding_max:.17g}",
                    "raw_vpd_negative": str(vpd < 0.0).lower(),
                    "serialized_sign_sensitivity": sensitivity,
                }
            )

        reconstructed_tmin = min(temperatures)
        reconstructed_tmax = max(temperatures)
        reconstructed_tdew = statistics.fmean(dewpoints)
        source_row = reported[case_date]
        reported_tmin = float(source_row["tmin_c"])
        reported_tmax = float(source_row["tmax_c"])
        reported_tdew = float(source_row["tdew_c"])
        operand_residuals = (
            reconstructed_tmin - reported_tmin,
            reconstructed_tmax - reported_tmax,
            reconstructed_tdew - reported_tdew,
        )
        operands_pass = all(abs(value) <= 0.01 + 1.0e-12 for value in operand_residuals)
        cal07_vpd = (
            0.5 * (e_pa(reported_tmin) + e_pa(reported_tmax))
            - e_pa(reported_tdew)
        )
        reconstructed_contract_vpd = (
            0.5 * (e_pa(reconstructed_tmin) + e_pa(reconstructed_tmax))
            - e_pa(reconstructed_tdew)
        )
        mean_hourly_vpd = statistics.fmean(vpds)
        mean_e_t = statistics.fmean(e_pa(value) for value in temperatures)
        mean_e_dew = statistics.fmean(e_pa(value) for value in dewpoints)
        temperature_term = (
            0.5 * (e_pa(reconstructed_tmin) + e_pa(reconstructed_tmax))
            - mean_e_t
        )
        dewpoint_term = mean_e_dew - e_pa(reconstructed_tdew)
        difference = reconstructed_contract_vpd - mean_hourly_vpd
        closure = difference - temperature_term - dewpoint_term
        if abs(closure) > 1.0e-9:
            raise ValueError(f"decomposition closure failure for {case_date}: {closure}")
        signs_agree = (reconstructed_contract_vpd < 0.0) == (cal07_vpd < 0.0)
        hourly_negative_count = sum(value < 0.0 for value in vpds)
        source_pass = (
            inventory_valid
            and metadata_valid
            and lineage_compatible
            and operands_pass
            and signs_agree
        )
        if not source_pass:
            attribution = "SOURCE_RECONSTRUCTION_MISMATCH"
        elif hourly_negative_count and reconstructed_contract_vpd < 0.0:
            attribution = "MIXED_PRODUCT_NEGATIVES"
        elif hourly_negative_count:
            attribution = "REPORTED_HOURLY_OPERAND_NEGATIVE"
        elif reconstructed_contract_vpd < 0.0:
            attribution = "DAILY_SUMMARY_OPERATOR_MISMATCH"
        else:
            attribution = "NO_REPRODUCED_NEGATIVE"

        daily_rows.append(
            {
                "date": case_date,
                "hour_count": 24,
                "reported_tmin_c": f"{reported_tmin:.17g}",
                "reconstructed_tmin_c": f"{reconstructed_tmin:.17g}",
                "tmin_residual_c": f"{operand_residuals[0]:.17g}",
                "reported_tmax_c": f"{reported_tmax:.17g}",
                "reconstructed_tmax_c": f"{reconstructed_tmax:.17g}",
                "tmax_residual_c": f"{operand_residuals[1]:.17g}",
                "reported_tdew_mean_c": f"{reported_tdew:.17g}",
                "reconstructed_tdew_mean_c": f"{reconstructed_tdew:.17g}",
                "tdew_residual_c": f"{operand_residuals[2]:.17g}",
                "cal07_contract_vpd_pa": f"{cal07_vpd:.17g}",
                "reconstructed_contract_vpd_pa": f"{reconstructed_contract_vpd:.17g}",
                "contract_vpd_residual_pa": f"{reconstructed_contract_vpd-cal07_vpd:.17g}",
                "mean_hourly_product_vpd_pa": f"{mean_hourly_vpd:.17g}",
                "minimum_hourly_product_vpd_pa": f"{min(vpds):.17g}",
                "hourly_negative_count": hourly_negative_count,
                "temperature_extrema_summary_term_pa": f"{temperature_term:.17g}",
                "dewpoint_nonlinearity_term_pa": f"{dewpoint_term:.17g}",
                "contract_minus_hourly_mean_pa": f"{difference:.17g}",
                "decomposition_closure_residual_pa": f"{closure:.17g}",
            }
        )
        attribution_rows.append(
            {
                "date": case_date,
                "hour_inventory_valid": str(inventory_valid).lower(),
                "response_metadata_valid": str(metadata_valid).lower(),
                "upstream_lineage_relation": lineage,
                "daily_hourly_lineage_compatible": str(lineage_compatible).lower(),
                "daily_operands_within_serialized_resolution": str(operands_pass).lower(),
                "any_hourly_product_vpd_negative": str(hourly_negative_count > 0).lower(),
                "reconstructed_contract_daily_vpd_negative": str(reconstructed_contract_vpd < 0.0).lower(),
                "cal07_contract_daily_vpd_negative": str(cal07_vpd < 0.0).lower(),
                "contract_daily_signs_agree": str(signs_agree).lower(),
                "attribution": attribution,
            }
        )

    write_csv(
        ART / "hourly-reconstruction.csv",
        (
            "date",
            "hour_key",
            "hour_lst",
            "t2m_c",
            "t2mdew_c",
            "tdew_minus_t2m_c",
            "hourly_product_vpd_pa",
            "half_unit_vpd_min_pa",
            "half_unit_vpd_max_pa",
            "raw_vpd_negative",
            "serialized_sign_sensitivity",
        ),
        hourly_rows,
    )
    write_csv(
        ART / "daily-decomposition.csv",
        tuple(daily_rows[0]),
        daily_rows,
    )
    write_csv(ART / "attribution.csv", tuple(attribution_rows[0]), attribution_rows)

    manifest = []
    for path in sorted(SOURCE.iterdir()):
        manifest.append(
            {
                "path": path.relative_to(PKG),
                "sha256": sha256(path),
                "bytes": path.stat().st_size,
                "retrieved": RETRIEVED,
                "url": URLS[path.name],
            }
        )
    daily_source = (
        CAL07
        / "inputs/source/power_alercecosteroforest_20220101_20260724.json"
    )
    manifest.append(
        {
            "path": daily_source.relative_to(ROOT),
            "sha256": sha256(daily_source),
            "bytes": daily_source.stat().st_size,
            "retrieved": "2026-07-28",
            "url": "CAL-07 retained source; exact URL in its source manifest",
        }
    )
    write_csv(
        ART / "source-manifest.csv",
        ("path", "sha256", "bytes", "retrieved", "url"),
        manifest,
    )


if __name__ == "__main__":
    main()
