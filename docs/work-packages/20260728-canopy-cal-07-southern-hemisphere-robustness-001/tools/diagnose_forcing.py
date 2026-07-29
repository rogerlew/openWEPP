#!/usr/bin/env python3
"""Publish deterministic CAL-07 source diagnostics after fail-closed execution."""

from __future__ import annotations

import csv
import math
import statistics
from collections import defaultdict
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
INPUT = PKG / "inputs"
ART = PKG / "artifacts"


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def es(temperature_c: float) -> float:
    return 0.6108 * math.exp(17.27 * temperature_c / (temperature_c + 237.3))


def write(name: str, fields: tuple[str, ...], data: list[dict[str, object]]) -> None:
    with (ART / name).open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        writer.writerows(data)


def main() -> None:
    diagnostics = []
    negatives = []
    for row in rows(INPUT / "forcing.csv"):
        tmax, tmin, tdew = (float(row[name]) for name in ("tmax_c", "tmin_c", "tdew_c"))
        vpd = 1_000.0 * (0.5 * (es(tmax) + es(tmin)) - es(tdew))
        result = {
            **row,
            "reconstructed_vpd_pa": f"{vpd:.17g}",
            "contract_status": "FAIL_NEGATIVE" if vpd < 0.0 else "ADMISSIBLE",
        }
        diagnostics.append(result)
        if vpd < 0.0:
            negatives.append(result)
    write(
        "forcing-diagnostics.csv",
        (
            "site_id",
            "date",
            "year",
            "doy",
            "latitude_degrees",
            "tmax_c",
            "tmin_c",
            "tdew_c",
            "reconstructed_vpd_pa",
            "contract_status",
        ),
        diagnostics,
    )
    write(
        "negative-vpd-days.csv",
        (
            "site_id",
            "date",
            "year",
            "doy",
            "latitude_degrees",
            "tmax_c",
            "tmin_c",
            "tdew_c",
            "reconstructed_vpd_pa",
            "contract_status",
        ),
        negatives,
    )
    observations = rows(INPUT / "observations.csv")
    summary = []
    for site in sorted({row["site_id"] for row in observations}):
        selected = [row for row in observations if row["site_id"] == site]
        values = [float(row["gcc_90"]) for row in selected]
        summary.append(
            {
                "site_id": site,
                "admitted_camera_days": len(selected),
                "first_date": min(row["date"] for row in selected),
                "last_date": max(row["date"] for row in selected),
                "gcc90_min": f"{min(values):.9f}",
                "gcc90_median": f"{statistics.median(values):.9f}",
                "gcc90_max": f"{max(values):.9f}",
            }
        )
    write(
        "observation-source-summary.csv",
        (
            "site_id",
            "admitted_camera_days",
            "first_date",
            "last_date",
            "gcc90_min",
            "gcc90_median",
            "gcc90_max",
        ),
        summary,
    )


if __name__ == "__main__":
    main()
