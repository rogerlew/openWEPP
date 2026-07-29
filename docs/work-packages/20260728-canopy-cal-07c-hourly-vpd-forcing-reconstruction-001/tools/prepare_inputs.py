#!/usr/bin/env python3
"""Prepare CAL-07C package-local inputs and VPD source diagnostics."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import statistics
from collections import defaultdict
from datetime import date
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
SOURCE = PKG / "inputs" / "source"
OUT = PKG / "inputs"
ART = PKG / "artifacts"
CAL07 = (
    ROOT
    / "docs/work-packages"
    / "20260728-canopy-cal-07-southern-hemisphere-robustness-001"
)
CAL07B = (
    ROOT
    / "docs/work-packages"
    / "20260728-canopy-cal-07b-hourly-vpd-aggregation-diagnostic-001"
)
CAL04B = (
    ROOT
    / "docs/work-packages"
    / "20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001"
    / "artifacts"
)
HOURLY_NAME = "power_hourly_alerce_20220101_20260724_lst.json"
HOURLY_URL = (
    "https://power.larc.nasa.gov/api/temporal/hourly/point?"
    "parameters=T2M,T2MDEW&community=AG&longitude=-73.4439&latitude=-40.1726"
    "&start=20220101&end=20260724&format=JSON&time-standard=LST"
)


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def write_csv(path: Path, fields: tuple[str, ...], data: list[dict[str, object]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(data)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def e_pa(temperature_c: float) -> float:
    return 610.8 * math.exp(17.27 * temperature_c / (temperature_c + 237.3))


def contract_vpd_pa(tmax_c: float, tmin_c: float, tdew_c: float) -> float:
    return 0.5 * (e_pa(tmax_c) + e_pa(tmin_c)) - e_pa(tdew_c)


def compact(day: str) -> str:
    return day.replace("-", "")


def write_ensemble() -> None:
    accepted_path = CAL04B / "accepted-calibration-ensemble.csv"
    candidate_path = CAL04B / "candidate-configurations.csv"
    accepted_rows = rows(accepted_path)
    accepted = {row["candidate_id"] for row in accepted_rows}
    if (
        len(accepted_rows) != 37
        or len(accepted) != 37
        or {row["state"] for row in accepted_rows} != {"ACCEPTED_FROZEN"}
    ):
        raise ValueError("accepted CAL-04B ledger must contain 37 frozen members")
    candidate_rows = [
        row for row in rows(candidate_path) if row["candidate_id"] in accepted
    ]
    if len(candidate_rows) != 37:
        raise ValueError("candidate table did not reproduce 37 accepted members")
    fields = (
        "candidate_id",
        "minimum_temperature_inactive_c",
        "minimum_temperature_unconstrained_c",
        "vapor_pressure_deficit_unconstrained_pa",
        "vapor_pressure_deficit_inactive_pa",
        "photoperiod_inactive_hours",
        "photoperiod_unconstrained_hours",
    )
    write_csv(
        OUT / "ensemble.csv",
        fields,
        [{field: row[field] for field in fields} for row in candidate_rows],
    )
    custody_fields = (*fields, "accepted_ledger_sha256", "candidate_table_sha256")
    write_csv(
        ART / "ensemble-custody.csv",
        custody_fields,
        [
            {
                **{field: row[field] for field in fields},
                "accepted_ledger_sha256": sha256(accepted_path),
                "candidate_table_sha256": sha256(candidate_path),
            }
            for row in candidate_rows
        ],
    )


def hourly_payload_metadata(payload: dict[str, object]) -> dict[str, str]:
    header = payload["header"]
    geometry = payload["geometry"]
    parameters = payload["parameters"]
    coordinates = geometry["coordinates"]
    return {
        "object_type": "POWER_HOURLY_JSON",
        "api_version": str(header["api"]["version"]),
        "time_standard": str(header["time_standard"]),
        "fill_value": str(header["fill_value"]),
        "geometry_type": str(geometry["type"]),
        "longitude_degrees": f"{float(coordinates[0]):.6f}",
        "latitude_degrees": f"{float(coordinates[1]):.6f}",
        "elevation_m": f"{float(coordinates[2]):.3f}",
        "source_list": ";".join(str(item) for item in header["sources"]),
        "units": ";".join(
            f"{name}:{values['units']}" for name, values in sorted(parameters.items())
        ),
        "start": str(header["start"]),
        "end": str(header["end"]),
        "parameters": ";".join(sorted(parameters)),
        "notes": "full-period Alerce LST hourly source used for admitted VPD",
    }


def documentation_metadata() -> dict[str, str]:
    return {
        "object_type": "POWER_METHOD_PAGE",
        "api_version": "",
        "time_standard": "",
        "fill_value": "",
        "geometry_type": "",
        "longitude_degrees": "",
        "latitude_degrees": "",
        "elevation_m": "",
        "source_list": "",
        "units": "",
        "start": "",
        "end": "",
        "parameters": "",
        "notes": "retained POWER documentation/method authority page",
    }


def build_alerce_vpd() -> tuple[dict[str, dict[str, object]], list[dict[str, object]]]:
    payload = json.loads((SOURCE / HOURLY_NAME).read_text(encoding="utf-8"))
    header = payload["header"]
    parameters = payload["parameters"]
    values = payload["properties"]["parameter"]
    if (
        header["time_standard"] != "LST"
        or header["start"] != "20220101"
        or header["end"] != "20260724"
        or header["fill_value"] != -999.0
        or parameters["T2M"]["units"] != "C"
        or parameters["T2MDEW"]["units"] != "C"
        or payload["geometry"]["coordinates"] != [-73.444, -40.173, 99.4]
    ):
        raise ValueError("unexpected hourly POWER metadata")
    if len(values["T2M"]) != 39984 or len(values["T2MDEW"]) != 39984:
        raise ValueError("unexpected hourly inventory length")

    daily_rows = [
        row
        for row in rows(CAL07 / "inputs" / "forcing.csv")
        if row["site_id"] == "SH-EN-ALERCE"
    ]
    if len(daily_rows) != 1666:
        raise ValueError("unexpected Alerce daily inventory")

    hourly_output: list[dict[str, object]] = []
    daily_output: dict[str, dict[str, object]] = {}
    for row in daily_rows:
        day = row["date"]
        keys = [f"{compact(day)}{hour:02d}" for hour in range(24)]
        temperatures: list[float] = []
        dewpoints: list[float] = []
        vpds: list[float] = []
        for key in keys:
            if key not in values["T2M"] or key not in values["T2MDEW"]:
                raise ValueError(f"missing hourly key {key}")
            temperature = float(values["T2M"][key])
            dewpoint = float(values["T2MDEW"][key])
            if (
                not math.isfinite(temperature)
                or not math.isfinite(dewpoint)
                or temperature == -999.0
                or dewpoint == -999.0
            ):
                raise ValueError(f"invalid hourly source value at {key}")
            vpd = e_pa(temperature) - e_pa(dewpoint)
            if not math.isfinite(vpd):
                raise ValueError(f"nonfinite hourly VPD at {key}")
            temperatures.append(temperature)
            dewpoints.append(dewpoint)
            vpds.append(vpd)
            hourly_output.append(
                {
                    "date": day,
                    "hour_key": key,
                    "hour_lst": key[-2:],
                    "t2m_c": f"{temperature:.17g}",
                    "t2mdew_c": f"{dewpoint:.17g}",
                    "hourly_product_vpd_pa": f"{vpd:.17g}",
                    "raw_vpd_negative": str(vpd < 0.0).lower(),
                }
            )
        tmin = min(temperatures)
        tmax = max(temperatures)
        tdew = statistics.fmean(dewpoints)
        reported_tmin = float(row["tmin_c"])
        reported_tmax = float(row["tmax_c"])
        reported_tdew = float(row["tdew_c"])
        residuals = {
            "tmin_residual_c": tmin - reported_tmin,
            "tmax_residual_c": tmax - reported_tmax,
            "tdew_residual_c": tdew - reported_tdew,
        }
        if any(abs(value) > 0.01 + 1.0e-12 for value in residuals.values()):
            raise ValueError(f"hourly/daily operand mismatch on {day}: {residuals}")
        admitted = statistics.fmean(vpds)
        if admitted < 0.0 or not math.isfinite(admitted):
            raise ValueError(f"negative admitted daily VPD on {day}: {admitted}")
        negative_count = sum(value < 0.0 for value in vpds)
        daily_output[day] = {
            "site_id": "SH-EN-ALERCE",
            "date": day,
            "year": row["year"],
            "doy": row["doy"],
            "latitude_degrees": row["latitude_degrees"],
            "hour_count": len(keys),
            "hour_key_start": keys[0],
            "hour_key_end": keys[-1],
            "reported_tmax_c": f"{reported_tmax:.17g}",
            "reported_tmin_c": f"{reported_tmin:.17g}",
            "reported_tdew_c": f"{reported_tdew:.17g}",
            "hourly_tmax_c": f"{tmax:.17g}",
            "hourly_tmin_c": f"{tmin:.17g}",
            "hourly_tdew_mean_c": f"{tdew:.17g}",
            **{key: f"{value:.17g}" for key, value in residuals.items()},
            "daily_contract_vpd_pa": f"{contract_vpd_pa(reported_tmax, reported_tmin, reported_tdew):.17g}",
            "admitted_hourly_mean_vpd_pa": f"{admitted:.17g}",
            "hourly_min_vpd_pa": f"{min(vpds):.17g}",
            "hourly_max_vpd_pa": f"{max(vpds):.17g}",
            "negative_hourly_count": negative_count,
            "vpd_delta_pa": f"{admitted - contract_vpd_pa(reported_tmax, reported_tmin, reported_tdew):.17g}",
            "admitted_daily_negative": str(admitted < 0.0).lower(),
            "daily_admission_pass": str(
                len(keys) == 24 and math.isfinite(admitted) and admitted >= 0.0
            ).lower(),
            "admission_status": "ADMITTED_DAILY_NONNEGATIVE_WITH_SIGNED_HOURLY_COMPONENTS",
        }

    write_csv(
        ART / "hourly-vpd-reconstruction.csv",
        (
            "date",
            "hour_key",
            "hour_lst",
            "t2m_c",
            "t2mdew_c",
            "hourly_product_vpd_pa",
            "raw_vpd_negative",
        ),
        hourly_output,
    )
    daily_fields = (
        "site_id",
        "date",
        "year",
        "doy",
        "latitude_degrees",
        "hour_count",
        "hour_key_start",
        "hour_key_end",
        "reported_tmax_c",
        "reported_tmin_c",
        "reported_tdew_c",
        "hourly_tmax_c",
        "hourly_tmin_c",
        "hourly_tdew_mean_c",
        "tmax_residual_c",
        "tmin_residual_c",
        "tdew_residual_c",
        "daily_contract_vpd_pa",
        "admitted_hourly_mean_vpd_pa",
        "hourly_min_vpd_pa",
        "hourly_max_vpd_pa",
        "negative_hourly_count",
        "vpd_delta_pa",
        "admitted_daily_negative",
        "daily_admission_pass",
        "admission_status",
    )
    write_csv(
        ART / "daily-vpd-reconstruction.csv",
        daily_fields,
        list(daily_output.values()),
    )
    write_csv(
        ART / "admission-table.csv",
        daily_fields,
        list(daily_output.values()),
    )
    return daily_output, hourly_output


def write_forcing(alerce_daily: dict[str, dict[str, object]]) -> None:
    forcing_rows = []
    summary: dict[str, list[float]] = defaultdict(list)
    for row in rows(CAL07 / "inputs" / "forcing.csv"):
        tmax = float(row["tmax_c"])
        tmin = float(row["tmin_c"])
        tdew = float(row["tdew_c"])
        if row["site_id"] == "SH-EN-ALERCE":
            vpd = float(alerce_daily[row["date"]]["admitted_hourly_mean_vpd_pa"])
            source = "POWER_HOURLY_PAIRED_PRODUCT_DAILY_MEAN"
        else:
            vpd = contract_vpd_pa(tmax, tmin, tdew)
            source = "CAL07_DAILY_CONTRACT_UNCHANGED"
        if vpd < 0.0 or not math.isfinite(vpd):
            raise ValueError(f"invalid admitted VPD for {row['site_id']} {row['date']}")
        summary[source].append(vpd)
        forcing_rows.append(
            {
                "site_id": row["site_id"],
                "date": row["date"],
                "year": row["year"],
                "doy": row["doy"],
                "latitude_degrees": row["latitude_degrees"],
                "tmax_c": f"{tmax:.17g}",
                "tmin_c": f"{tmin:.17g}",
                "tdew_c": f"{tdew:.17g}",
                "vpd_pa": f"{vpd:.17g}",
                "vpd_source": source,
            }
        )
    write_csv(
        OUT / "forcing.csv",
        (
            "site_id",
            "date",
            "year",
            "doy",
            "latitude_degrees",
            "tmax_c",
            "tmin_c",
            "tdew_c",
            "vpd_pa",
            "vpd_source",
        ),
        forcing_rows,
    )
    write_csv(
        ART / "forcing-source-summary.csv",
        ("vpd_source", "days", "minimum_vpd_pa", "maximum_vpd_pa", "negative_days"),
        [
            {
                "vpd_source": source,
                "days": len(values),
                "minimum_vpd_pa": f"{min(values):.17g}",
                "maximum_vpd_pa": f"{max(values):.17g}",
                "negative_days": sum(value < 0.0 for value in values),
            }
            for source, values in sorted(summary.items())
        ],
    )


def copy_normalized_observations() -> None:
    for name in ("observations.csv", "transitions.csv"):
        source = CAL07 / "inputs" / name
        data = rows(source)
        write_csv(OUT / name, tuple(data[0].keys()), data)


def write_source_manifest() -> None:
    urls = {
        HOURLY_NAME: HOURLY_URL,
        "power_hourly_api_20260728.html": "https://power.larc.nasa.gov/docs/services/api/temporal/hourly/",
        "power_daily_api_20260728.html": "https://power.larc.nasa.gov/docs/services/api/temporal/daily/",
        "power_temporal_processing_20260728.html": "https://power.larc.nasa.gov/docs/methodology/data/processing/",
        "power_meteorology_20260728.html": "https://power.larc.nasa.gov/docs/methodology/meteorology/",
        "power_time_faq_20260728.html": "https://power.larc.nasa.gov/docs/faqs/other/",
    }
    hourly_payload = json.loads((SOURCE / HOURLY_NAME).read_text(encoding="utf-8"))
    manifest_rows = []
    for path in sorted(SOURCE.iterdir()):
        metadata = (
            hourly_payload_metadata(hourly_payload)
            if path.name == HOURLY_NAME
            else documentation_metadata()
        )
        manifest_rows.append(
            {
                "path": path.relative_to(PKG),
                "sha256": sha256(path),
                "bytes": path.stat().st_size,
                "retrieved": "2026-07-28T23:35:00-07:00",
                "url": urls[path.name],
                **metadata,
            }
        )
    write_csv(
        ART / "source-manifest.csv",
        (
            "path",
            "sha256",
            "bytes",
            "retrieved",
            "url",
            "object_type",
            "api_version",
            "time_standard",
            "fill_value",
            "geometry_type",
            "longitude_degrees",
            "latitude_degrees",
            "elevation_m",
            "source_list",
            "units",
            "start",
            "end",
            "parameters",
            "notes",
        ),
        manifest_rows,
    )
    dependencies = [
        CAL07 / "inputs" / "forcing.csv",
        CAL07 / "inputs" / "observations.csv",
        CAL07 / "inputs" / "transitions.csv",
        CAL07 / "artifacts" / "source-manifest.csv",
        CAL07 / "artifacts" / "final-disposition.md",
        CAL07B / "artifacts" / "source-manifest.csv",
        CAL07B / "artifacts" / "attribution.csv",
        CAL07B / "artifacts" / "science-summary.md",
        CAL07B / "artifacts" / "final-disposition.md",
        CAL04B / "accepted-calibration-ensemble.csv",
        CAL04B / "candidate-configurations.csv",
    ]
    write_csv(
        ART / "dependency-manifest.csv",
        ("path", "sha256", "bytes"),
        [
            {
                "path": path.relative_to(ROOT),
                "sha256": sha256(path),
                "bytes": path.stat().st_size,
            }
            for path in dependencies
        ],
    )


def main() -> None:
    ART.mkdir(exist_ok=True)
    OUT.mkdir(exist_ok=True)
    alerce_daily, hourly = build_alerce_vpd()
    write_ensemble()
    write_forcing(alerce_daily)
    copy_normalized_observations()
    write_source_manifest()
    negative_hours = sum(row["raw_vpd_negative"] == "true" for row in hourly)
    if negative_hours != 349:
        raise ValueError(f"unexpected negative hourly count: {negative_hours}")


if __name__ == "__main__":
    main()
