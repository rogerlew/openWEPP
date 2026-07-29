#!/usr/bin/env python3
"""Normalize frozen CAL-07 sources without analyzing results."""

from __future__ import annotations

import csv
import hashlib
import json
import math
from datetime import date
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
SOURCE = PKG / "inputs" / "source"
OUT = PKG / "inputs"
CAL04B = (
    ROOT
    / "docs/work-packages"
    / "20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001"
    / "artifacts"
)

SITES = (
    (
        "SH-DB-BEZA",
        -23.6558,
        "power_bezamahafaly_20220101_20260724.json",
        "bezamahafaly_DB_1000_1day.csv",
        "bezamahafaly_DB_1000_simplified_transition_dates.csv",
        "DB",
    ),
    (
        "SH-EN-ALERCE",
        -40.1726,
        "power_alercecosteroforest_20220101_20260724.json",
        "alercecosteroforest_EN_1000_1day.csv",
        "alercecosteroforest_EN_1000_simplified_transition_dates.csv",
        "EN",
    ),
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def write_ensemble() -> None:
    accepted_path = CAL04B / "accepted-calibration-ensemble.csv"
    with accepted_path.open(newline="", encoding="utf-8") as stream:
        accepted_rows = list(csv.DictReader(stream))
    accepted = {row["candidate_id"] for row in accepted_rows}
    if (
        len(accepted_rows) != 37
        or len(accepted) != 37
        or {row["state"] for row in accepted_rows} != {"ACCEPTED_FROZEN"}
    ):
        raise ValueError("accepted ledger must contain 37 unique ACCEPTED_FROZEN IDs")
    candidate_path = CAL04B / "candidate-configurations.csv"
    with candidate_path.open(newline="", encoding="utf-8") as stream:
        rows = [row for row in csv.DictReader(stream) if row["candidate_id"] in accepted]
    if len(rows) != 37 or len({row["candidate_id"] for row in rows}) != 37:
        raise ValueError(f"expected 37 accepted members, found {len(rows)}")
    fields = (
        "candidate_id",
        "minimum_temperature_inactive_c",
        "minimum_temperature_unconstrained_c",
        "vapor_pressure_deficit_unconstrained_pa",
        "vapor_pressure_deficit_inactive_pa",
        "photoperiod_inactive_hours",
        "photoperiod_unconstrained_hours",
    )
    with (OUT / "ensemble.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        writer.writerows({field: row[field] for field in fields} for row in rows)
    custody_fields = (*fields, "accepted_ledger_sha256", "candidate_table_sha256")
    with (PKG / "artifacts" / "ensemble-custody.csv").open(
        "w", newline="", encoding="utf-8"
    ) as stream:
        writer = csv.DictWriter(stream, fieldnames=custody_fields)
        writer.writeheader()
        for row in rows:
            writer.writerow(
                {
                    **{field: row[field] for field in fields},
                    "accepted_ledger_sha256": sha256(accepted_path),
                    "candidate_table_sha256": sha256(candidate_path),
                }
            )


def write_forcing() -> None:
    fields = ("site_id", "date", "year", "doy", "latitude_degrees", "tmax_c", "tmin_c", "tdew_c")
    with (OUT / "forcing.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        for site_id, latitude, power_name, *_ in SITES:
            payload = json.loads((SOURCE / power_name).read_text(encoding="utf-8"))
            parameters = payload["properties"]["parameter"]
            keys = sorted(parameters["T2M_MIN"])
            if not keys or keys[0] != "20220101" or keys[-1] != "20260724":
                raise ValueError(f"unexpected POWER period for {site_id}")
            for key in keys:
                day = date.fromisoformat(f"{key[:4]}-{key[4:6]}-{key[6:]}")
                values = [parameters[name][key] for name in ("T2M_MAX", "T2M_MIN", "T2MDEW")]
                if any(value == -999.0 or not math.isfinite(float(value)) for value in values):
                    raise ValueError(f"missing POWER value for {site_id} {day}")
                writer.writerow(
                    {
                        "site_id": site_id,
                        "date": day.isoformat(),
                        "year": day.year,
                        "doy": day.timetuple().tm_yday,
                        "latitude_degrees": latitude,
                        "tmax_c": values[0],
                        "tmin_c": values[1],
                        "tdew_c": values[2],
                    }
                )


def read_phenocam(path: Path) -> list[dict[str, str]]:
    lines = [line for line in path.read_text(encoding="utf-8").splitlines() if not line.startswith("#")]
    return list(csv.DictReader(lines))


def write_observations() -> None:
    fields = ("site_id", "vegetation_class", "date", "year", "doy", "gcc_90", "smooth_gcc_90")
    with (OUT / "observations.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        for site_id, _, _, daily_name, _, vegetation_class in SITES:
            for row in read_phenocam(SOURCE / daily_name):
                if row["image_count"] == "NA" or int(row["image_count"]) <= 0:
                    continue
                if row["gcc_90"] == "NA" or row["outlierflag_gcc_90"] != "0":
                    continue
                gcc_90 = float(row["gcc_90"])
                if not math.isfinite(gcc_90):
                    raise ValueError(f"non-finite GCC90 for {site_id} {row['date']}")
                writer.writerow(
                    {
                        "site_id": site_id,
                        "vegetation_class": vegetation_class,
                        "date": row["date"],
                        "year": row["year"],
                        "doy": row["doy"],
                        "gcc_90": f"{gcc_90:.17g}",
                        "smooth_gcc_90": row["smooth_gcc_90"],
                    }
                )
    transition_fields = ("site_id", "vegetation_class", "year", "direction", "date_50", "doy_50")
    with (OUT / "transitions.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=transition_fields)
        writer.writeheader()
        for site_id, _, _, _, transition_name, vegetation_class in SITES:
            with (SOURCE / transition_name).open(newline="", encoding="utf-8") as source:
                for row in csv.DictReader(source):
                    writer.writerow(
                        {
                            "site_id": site_id,
                            "vegetation_class": vegetation_class,
                            "year": row["year"],
                            "direction": row["direction"],
                            "date_50": f"{row['year']}-{row['date_50']}",
                            "doy_50": row["DOY_50"],
                        }
                    )


def write_manifest() -> None:
    source_urls = {
        "bezamahafaly_meta.json": "https://phenocam.nau.edu/data/archive/bezamahafaly/ROI/bezamahafaly_DB_1000_provisional_data.zip",
        "bezamahafaly_DB_1000_1day.csv": "https://phenocam.nau.edu/data/archive/bezamahafaly/ROI/bezamahafaly_DB_1000_provisional_data.zip",
        "bezamahafaly_DB_1000_simplified_transition_dates.csv": "https://phenocam.nau.edu/data/archive/bezamahafaly/ROI/bezamahafaly_DB_1000_provisional_data.zip",
        "bezamahafaly_DB_1000_roi.csv": "https://phenocam.nau.edu/data/archive/bezamahafaly/ROI/bezamahafaly_DB_1000_provisional_data.zip",
        "bezamahafaly_DB_1000_datarecord2.html": "https://phenocam.nau.edu/data/archive/bezamahafaly/ROI/bezamahafaly_DB_1000_provisional_data.zip",
        "alercecosteroforest_meta.json": "https://phenocam.nau.edu/data/archive/alercecosteroforest/ROI/alercecosteroforest_EN_1000_provisional_data.zip",
        "alercecosteroforest_EN_1000_1day.csv": "https://phenocam.nau.edu/data/archive/alercecosteroforest/ROI/alercecosteroforest_EN_1000_provisional_data.zip",
        "alercecosteroforest_EN_1000_simplified_transition_dates.csv": "https://phenocam.nau.edu/data/archive/alercecosteroforest/ROI/alercecosteroforest_EN_1000_provisional_data.zip",
        "alercecosteroforest_EN_1000_roi.csv": "https://phenocam.nau.edu/data/archive/alercecosteroforest/ROI/alercecosteroforest_EN_1000_provisional_data.zip",
        "alercecosteroforest_EN_1000_datarecord2.html": "https://phenocam.nau.edu/data/archive/alercecosteroforest/ROI/alercecosteroforest_EN_1000_provisional_data.zip",
        "power_bezamahafaly_20220101_20260724.json": "https://power.larc.nasa.gov/api/temporal/daily/point?parameters=T2M_MAX,T2M_MIN,T2MDEW&community=AG&longitude=44.6289&latitude=-23.6558&start=20220101&end=20260724&format=JSON",
        "power_alercecosteroforest_20220101_20260724.json": "https://power.larc.nasa.gov/api/temporal/daily/point?parameters=T2M_MAX,T2M_MIN,T2MDEW&community=AG&longitude=-73.4439&latitude=-40.1726&start=20220101&end=20260724&format=JSON",
        "phenocam_fair_use_20260728.html": "https://phenocam.nau.edu/webcam/fairuse_statement/",
    }
    fields = ("path", "sha256", "bytes", "retrieved", "source")
    with (PKG / "artifacts" / "source-manifest.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        for path in sorted(SOURCE.iterdir()):
            writer.writerow(
                {
                    "path": path.relative_to(PKG),
                    "sha256": sha256(path),
                    "bytes": path.stat().st_size,
                    "retrieved": "2026-07-28",
                    "source": source_urls[path.name],
                }
            )


def main() -> None:
    write_ensemble()
    write_forcing()
    write_observations()
    write_manifest()


if __name__ == "__main__":
    main()
