#!/usr/bin/env python3
"""Independent CAL-07C raw-source and result validator."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import statistics
import xml.etree.ElementTree as ET
from collections import defaultdict
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
ART = PKG / "artifacts"
INPUT = PKG / "inputs"
SOURCE = INPUT / "source"
HOURLY = SOURCE / "power_hourly_alerce_20220101_20260724_lst.json"


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def pressure_pa(temperature_c: float) -> float:
    return 610.8 * math.exp(17.27 * temperature_c / (temperature_c + 237.3))


def contract_vpd_pa(tmax_c: float, tmin_c: float, tdew_c: float) -> float:
    return 0.5 * (pressure_pa(tmax_c) + pressure_pa(tmin_c)) - pressure_pa(tdew_c)


def close(actual: float, expected: float, tolerance: float = 1.0e-9) -> None:
    assert abs(actual - expected) <= tolerance, (actual, expected, tolerance)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check_manifests() -> None:
    for manifest_name in ("source-manifest.csv", "dependency-manifest.csv"):
        for item in rows(ART / manifest_name):
            path = ROOT / item["path"] if not item["path"].startswith("inputs/") else PKG / item["path"]
            assert path.stat().st_size == int(item["bytes"]), path
            assert sha(path) == item["sha256"], path
    source_rows = rows(ART / "source-manifest.csv")
    hourly = [row for row in source_rows if row["object_type"] == "POWER_HOURLY_JSON"]
    assert len(hourly) == 1
    hourly_row = hourly[0]
    assert hourly_row["api_version"] == "v2.9.6"
    assert hourly_row["time_standard"] == "LST"
    assert hourly_row["fill_value"] == "-999.0"
    assert hourly_row["geometry_type"] == "Point"
    assert hourly_row["longitude_degrees"] == "-73.444000"
    assert hourly_row["latitude_degrees"] == "-40.173000"
    assert hourly_row["elevation_m"] == "99.400"
    assert hourly_row["source_list"] == "GEOSIT;MERRA2"
    assert hourly_row["units"] == "T2M:C;T2MDEW:C"
    assert hourly_row["start"] == "20220101"
    assert hourly_row["end"] == "20260724"
    assert hourly_row["parameters"] == "T2M;T2MDEW"


def reconstruct_hourly() -> tuple[dict[str, float], int, int]:
    payload = json.loads(HOURLY.read_text(encoding="utf-8"))
    assert payload["header"]["time_standard"] == "LST"
    assert payload["header"]["start"] == "20220101"
    assert payload["header"]["end"] == "20260724"
    values = payload["properties"]["parameter"]
    assert len(values["T2M"]) == len(values["T2MDEW"]) == 39984
    published_hourly = rows(ART / "hourly-vpd-reconstruction.csv")
    assert len(published_hourly) == 39984
    hourly_by_key = {row["hour_key"]: row for row in published_hourly}
    by_day: dict[str, list[float]] = defaultdict(list)
    negative_hours = 0
    for key in sorted(values["T2M"]):
        t = float(values["T2M"][key])
        d = float(values["T2MDEW"][key])
        assert math.isfinite(t) and math.isfinite(d)
        assert t != -999.0 and d != -999.0
        vpd = pressure_pa(t) - pressure_pa(d)
        if vpd < 0.0:
            negative_hours += 1
        row = hourly_by_key[key]
        close(float(row["t2m_c"]), t)
        close(float(row["t2mdew_c"]), d)
        close(float(row["hourly_product_vpd_pa"]), vpd)
        assert row["raw_vpd_negative"] == str(vpd < 0.0).lower()
        by_day[key[:8]].append(vpd)
    assert negative_hours == 349
    assert len(by_day) == 1666
    daily_mean = {
        f"{key[:4]}-{key[4:6]}-{key[6:]}": statistics.fmean(values)
        for key, values in by_day.items()
    }
    negative_daily = sum(value < 0.0 for value in daily_mean.values())
    assert negative_daily == 0
    return daily_mean, negative_hours, negative_daily


def check_forcing(daily_mean: dict[str, float]) -> None:
    daily_reconstruction = {
        row["date"]: row for row in rows(ART / "daily-vpd-reconstruction.csv")
    }
    admission_table = {
        row["date"]: row for row in rows(ART / "admission-table.csv")
    }
    assert admission_table == daily_reconstruction
    assert len(daily_reconstruction) == 1666
    negative_contract_dates: list[str] = []
    for day, expected in daily_mean.items():
        row = daily_reconstruction[day]
        close(float(row["admitted_hourly_mean_vpd_pa"]), expected)
        assert row["hour_count"] == "24"
        assert row["hour_key_start"] == day.replace("-", "") + "00"
        assert row["hour_key_end"] == day.replace("-", "") + "23"
        assert int(row["negative_hourly_count"]) >= 0
        assert row["admitted_daily_negative"] == "false"
        assert row["daily_admission_pass"] == "true"
        assert row["admission_status"] == "ADMITTED_DAILY_NONNEGATIVE_WITH_SIGNED_HOURLY_COMPONENTS"
        assert float(row["admitted_hourly_mean_vpd_pa"]) >= 0.0
        if float(row["daily_contract_vpd_pa"]) < 0.0:
            negative_contract_dates.append(day)
            assert float(row["admitted_hourly_mean_vpd_pa"]) != float(
                row["daily_contract_vpd_pa"]
            )
        for field in ("tmin_residual_c", "tmax_residual_c", "tdew_residual_c"):
            assert abs(float(row[field])) <= 0.01 + 1.0e-12
    assert set(negative_contract_dates) == {"2022-07-22", "2022-09-15", "2025-09-09"}

    forcing = rows(INPUT / "forcing.csv")
    assert len(forcing) == 3332
    for row in forcing:
        vpd = float(row["vpd_pa"])
        assert math.isfinite(vpd) and vpd >= 0.0
        if row["site_id"] == "SH-EN-ALERCE":
            close(vpd, daily_mean[row["date"]])
            assert row["vpd_source"] == "POWER_HOURLY_PAIRED_PRODUCT_DAILY_MEAN"
        else:
            expected = contract_vpd_pa(
                float(row["tmax_c"]), float(row["tmin_c"]), float(row["tdew_c"])
            )
            close(vpd, expected)
            assert row["vpd_source"] == "CAL07_DAILY_CONTRACT_UNCHANGED"


def check_outputs() -> tuple[float, float]:
    forcing = {(row["site_id"], row["date"]): row for row in rows(INPUT / "forcing.csv")}
    daily = rows(ART / "daily-kernel-output.csv")
    assert len(daily) == 3332 * 37
    inventories: dict[tuple[str, str], int] = defaultdict(int)
    previous_live: dict[tuple[str, str], float] = {}
    max_vpd_residual = 0.0
    max_mass_residual = 0.0
    for row in daily:
        key = (row["site_id"], row["candidate_id"])
        inventories[key] += 1
        source = forcing[(row["site_id"], row["date"])]
        max_vpd_residual = max(
            max_vpd_residual, abs(float(row["vpd_pa"]) - float(source["vpd_pa"]))
        )
        assert row["vpd_source"] == source["vpd_source"]
        live = float(row["live_foliar_biomass_kg_m2"])
        if key in previous_live:
            reconstructed = (
                previous_live[key]
                + float(row["leaf_on_allocation_kg_m2"])
                - float(row["leaf_off_litter_kg_m2"])
            )
            max_mass_residual = max(max_mass_residual, abs(reconstructed - live))
        previous_live[key] = live
        assert 0.0 <= float(row["gsi"]) <= 1.0
        assert abs(float(row["mass_closure_residual_kg_m2"])) <= 1.0e-12
    assert len(inventories) == 74 and set(inventories.values()) == {1666}
    assert max_vpd_residual <= 1.0e-9
    assert max_mass_residual <= 1.0e-12
    gates = {row["gate"]: row["status"] for row in rows(ART / "gate-results.csv")}
    assert gates == {
        "producer_phase_transform": "PASS",
        "real_consumer_ordering": "PASS",
    }
    return max_vpd_residual, max_mass_residual


def check_analysis_and_figures() -> None:
    assert len(rows(ART / "ensemble-daily.csv")) == 3332
    assert len(rows(ART / "shape-scores.csv")) == 148
    assert len(rows(ART / "transition-residuals.csv")) == 148
    verdicts = {row["cell"]: row["status"] for row in rows(ART / "verdict-matrix.csv")}
    assert verdicts["Alerce forcing-domain blocker"] == "LIFTED_FOR_BOUNDED_EXECUTION"
    assert verdicts["absolute canopy amplitude"] == "NOT_EVALUATED"
    assert verdicts["phase-transformed real-consumer chronology"] == "NOT_EVALUATED"
    assert verdicts["needle/fine-woody/decomposition consequences"] == "NOT_EVALUATED"
    expected = {
        "cal07c-vpd-reconstruction-audit",
        "cal07c-observed-and-modeled-seasons",
        "cal07c-score-summary",
        "cal07c-evidence-boundaries",
    }
    fig = ART / "figures"
    assert {path.stem for path in fig.glob("*.svg")} == expected
    assert {path.stem for path in fig.glob("*.md")} == expected
    for stem in expected:
        root = ET.parse(fig / f"{stem}.svg").getroot()
        assert root.attrib.get("role") == "img"
        assert root.attrib.get("aria-labelledby") == "title desc"
        tags = {element.tag.rsplit("}", 1)[-1] for element in root.iter()}
        assert {"title", "desc", "metadata"} <= tags
        sidecar = (fig / f"{stem}.md").read_text(encoding="utf-8")
        for heading in (
            "## Caption",
            "## How to read it",
            "## Plain-language takeaway",
            "## Methods and source binding",
            "## Limitations",
            "## Accessibility",
        ):
            assert heading in sidecar, (stem, heading)
    for item in rows(ART / "result-manifest.csv"):
        path = PKG / item["path"]
        assert path.stat().st_size == int(item["bytes"]), path
        assert sha(path) == item["sha256"], path


def main() -> None:
    check_manifests()
    daily_mean, negative_hours, negative_daily = reconstruct_hourly()
    check_forcing(daily_mean)
    max_vpd_residual, max_mass_residual = check_outputs()
    check_analysis_and_figures()
    print(
        "CAL-07C validation PASS: "
        f"{negative_hours} negative hourly components retained; "
        f"{negative_daily} negative admitted daily VPD rows; "
        f"max VPD residual={max_vpd_residual:.3e} Pa; "
        f"max mass residual={max_mass_residual:.3e} kg m-2"
    )


if __name__ == "__main__":
    main()
