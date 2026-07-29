#!/usr/bin/env python3
"""Independent raw-source validator for CAL-07B."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import statistics
import xml.etree.ElementTree as ET
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
ART = PKG / "artifacts"
SOURCE = PKG / "inputs" / "source"
CAL07 = (
    ROOT
    / "docs/work-packages"
    / "20260728-canopy-cal-07-southern-hemisphere-robustness-001"
)
CASES = {
    "2022-07-22": "power_hourly_alerce_20220722_lst.json",
    "2022-09-15": "power_hourly_alerce_20220915_lst.json",
    "2025-09-09": "power_hourly_alerce_20250909_lst.json",
}


def pressure_pa(temperature_c: float) -> float:
    return 610.8 * math.exp(17.27 * temperature_c / (temperature_c + 237.3))


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def close(actual: float, expected: float, tolerance: float = 1.0e-10) -> None:
    assert abs(actual - expected) <= tolerance, (actual, expected, tolerance)


def main() -> None:
    manifest = rows(ART / "source-manifest.csv")
    assert len(manifest) == 9
    for item in manifest:
        relative = Path(item["path"])
        path = ROOT / relative if relative.parts[0] == "docs" else PKG / relative
        assert path.stat().st_size == int(item["bytes"]), path
        assert hashlib.sha256(path.read_bytes()).hexdigest() == item["sha256"], path

    cal07_forcing = {
        row["date"]: row
        for row in rows(CAL07 / "inputs/forcing.csv")
        if row["site_id"] == "SH-EN-ALERCE" and row["date"] in CASES
    }
    published_hourly = rows(ART / "hourly-reconstruction.csv")
    published_daily = {row["date"]: row for row in rows(ART / "daily-decomposition.csv")}
    published_attribution = {row["date"]: row for row in rows(ART / "attribution.csv")}
    assert len(published_hourly) == 72
    assert set(published_daily) == set(published_attribution) == set(CASES)

    reconstructed_rows = 0
    for case_date, filename in CASES.items():
        payload = json.loads((SOURCE / filename).read_text(encoding="utf-8"))
        compact = case_date.replace("-", "")
        keys = [f"{compact}{hour:02d}" for hour in range(24)]
        values = payload["properties"]["parameter"]
        assert sorted(values["T2M"]) == sorted(values["T2MDEW"]) == keys
        assert payload["header"]["time_standard"] == "LST"
        temperatures = []
        dewpoints = []
        vpds = []
        case_rows = [row for row in published_hourly if row["date"] == case_date]
        assert [row["hour_key"] for row in case_rows] == keys
        for key, row in zip(keys, case_rows):
            t = float(values["T2M"][key])
            d = float(values["T2MDEW"][key])
            vpd = pressure_pa(t) - pressure_pa(d)
            lower = pressure_pa(t - 0.005) - pressure_pa(d + 0.005)
            upper = pressure_pa(t + 0.005) - pressure_pa(d - 0.005)
            close(float(row["t2m_c"]), t)
            close(float(row["t2mdew_c"]), d)
            close(float(row["tdew_minus_t2m_c"]), d - t)
            close(float(row["hourly_product_vpd_pa"]), vpd)
            close(float(row["half_unit_vpd_min_pa"]), lower)
            close(float(row["half_unit_vpd_max_pa"]), upper)
            assert row["raw_vpd_negative"] == str(vpd < 0.0).lower()
            temperatures.append(t)
            dewpoints.append(d)
            vpds.append(vpd)
            reconstructed_rows += 1

        daily = published_daily[case_date]
        source = cal07_forcing[case_date]
        tmin, tmax, dmean = min(temperatures), max(temperatures), statistics.fmean(dewpoints)
        reported_tmin = float(source["tmin_c"])
        reported_tmax = float(source["tmax_c"])
        reported_dew = float(source["tdew_c"])
        cal07_vpd = (
            0.5 * (pressure_pa(reported_tmin) + pressure_pa(reported_tmax))
            - pressure_pa(reported_dew)
        )
        contract = (
            0.5 * (pressure_pa(tmin) + pressure_pa(tmax)) - pressure_pa(dmean)
        )
        hourly_mean = statistics.fmean(vpds)
        temperature_term = (
            0.5 * (pressure_pa(tmin) + pressure_pa(tmax))
            - statistics.fmean(pressure_pa(value) for value in temperatures)
        )
        dew_term = (
            statistics.fmean(pressure_pa(value) for value in dewpoints)
            - pressure_pa(dmean)
        )
        difference = contract - hourly_mean
        closure = difference - temperature_term - dew_term
        for field, expected in (
            ("reconstructed_tmin_c", tmin),
            ("reconstructed_tmax_c", tmax),
            ("reconstructed_tdew_mean_c", dmean),
            ("cal07_contract_vpd_pa", cal07_vpd),
            ("reconstructed_contract_vpd_pa", contract),
            ("mean_hourly_product_vpd_pa", hourly_mean),
            ("minimum_hourly_product_vpd_pa", min(vpds)),
            ("temperature_extrema_summary_term_pa", temperature_term),
            ("dewpoint_nonlinearity_term_pa", dew_term),
            ("contract_minus_hourly_mean_pa", difference),
            ("decomposition_closure_residual_pa", closure),
        ):
            close(float(daily[field]), expected)
        assert abs(closure) <= 1.0e-9
        assert int(daily["hourly_negative_count"]) == 0
        assert min(vpds) > 0.0
        attribution = published_attribution[case_date]
        assert attribution["attribution"] == "DAILY_SUMMARY_OPERATOR_MISMATCH"
        assert attribution["any_hourly_product_vpd_negative"] == "false"
        assert attribution["reconstructed_contract_daily_vpd_negative"] == "true"
        assert attribution["contract_daily_signs_agree"] == "true"
    assert reconstructed_rows == 72

    expected_figures = {
        "cal07b-hourly-operands-and-vpd",
        "cal07b-additive-driver-decomposition",
        "cal07b-source-reconstruction",
    }
    assert {path.stem for path in (ART / "figures").glob("*.svg")} == expected_figures
    assert {path.stem for path in (ART / "figures").glob("*.md")} == expected_figures
    for stem in expected_figures:
        root = ET.parse(ART / "figures" / f"{stem}.svg").getroot()
        assert root.attrib.get("role") == "img"
        assert root.attrib.get("aria-labelledby") == "title desc"
        tags = {element.tag.rsplit("}", 1)[-1] for element in root.iter()}
        assert {"title", "desc", "metadata"} <= tags
        sidecar = (ART / "figures" / f"{stem}.md").read_text(encoding="utf-8")
        for heading in (
            "## Caption",
            "## How to read it",
            "## Plain-language takeaway",
            "## Methods and source binding",
            "## Limitations",
            "## Accessibility",
        ):
            assert heading in sidecar, (stem, heading)

    result_manifest = rows(ART / "result-manifest.csv")
    assert len(result_manifest) == 9
    for item in result_manifest:
        path = PKG / item["path"]
        assert path.stat().st_size == int(item["bytes"]), path
        assert hashlib.sha256(path.read_bytes()).hexdigest() == item["sha256"], path
    print(
        "CAL-07B validation PASS: 72 positive hourly-product VPD rows; "
        "3 DAILY_SUMMARY_OPERATOR_MISMATCH attributions"
    )


if __name__ == "__main__":
    main()
