#!/usr/bin/env python3
"""Independent CAL-07 evidence and figure validator."""

from __future__ import annotations

import csv
import hashlib
import math
import xml.etree.ElementTree as ET
from collections import defaultdict
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ART = PKG / "artifacts"
INPUT = PKG / "inputs"
FIG = ART / "figures"


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def saturation_vapor_pressure_kpa(temperature_c: float) -> float:
    return 0.6108 * math.exp(17.27 * temperature_c / (temperature_c + 237.3))


def main() -> None:
    source_manifest = rows(ART / "source-manifest.csv")
    for item in source_manifest:
        path = PKG / item["path"]
        assert path.stat().st_size == int(item["bytes"]), path
        assert hashlib.sha256(path.read_bytes()).hexdigest() == item["sha256"], path

    custody = rows(ART / "ensemble-custody.csv")
    ensemble = rows(INPUT / "ensemble.csv")
    assert len(custody) == len(ensemble) == 37
    assert len({row["candidate_id"] for row in custody}) == 37
    assert [row["candidate_id"] for row in custody] == [
        row["candidate_id"] for row in ensemble
    ]

    forcing = {
        (row["site_id"], row["date"]): row for row in rows(INPUT / "forcing.csv")
    }
    assert len(forcing) == 3332
    daily = rows(ART / "daily-kernel-output.csv")
    assert len(daily) == 3332 * 37
    inventories: dict[tuple[str, str], int] = defaultdict(int)
    dates_by_inventory: dict[tuple[str, str], list[str]] = defaultdict(list)
    previous_live: dict[tuple[str, str], float] = {}
    maximum_vpd_residual = 0.0
    maximum_closure_residual = 0.0
    for row in daily:
        key = (row["site_id"], row["candidate_id"])
        inventories[key] += 1
        dates_by_inventory[key].append(row["date"])
        raw = forcing[(row["site_id"], row["date"])]
        independent_vpd = 1_000.0 * (
            0.5
            * (
                saturation_vapor_pressure_kpa(float(raw["tmax_c"]))
                + saturation_vapor_pressure_kpa(float(raw["tmin_c"]))
            )
            - saturation_vapor_pressure_kpa(float(raw["tdew_c"]))
        )
        assert math.isfinite(independent_vpd) and independent_vpd >= 0.0
        maximum_vpd_residual = max(
            maximum_vpd_residual, abs(independent_vpd - float(row["vpd_pa"]))
        )
        live = float(row["live_foliar_biomass_kg_m2"])
        if key in previous_live:
            reconstructed = (
                previous_live[key]
                + float(row["leaf_on_allocation_kg_m2"])
                - float(row["leaf_off_litter_kg_m2"])
            )
            maximum_closure_residual = max(
                maximum_closure_residual, abs(reconstructed - live)
            )
        previous_live[key] = live
        assert 0.0 <= float(row["gsi"]) <= 1.0
        for field in (
            "vpd_pa",
            "gsi",
            "foliar_activity_fraction",
            "live_foliar_biomass_kg_m2",
            "leaf_on_allocation_kg_m2",
            "leaf_off_litter_kg_m2",
            "mass_closure_residual_kg_m2",
        ):
            assert math.isfinite(float(row[field])), (key, row["date"], field)
        assert abs(float(row["mass_closure_residual_kg_m2"])) <= 1.0e-12
    assert len(inventories) == 74 and set(inventories.values()) == {1666}
    for (site, _), actual_dates in dates_by_inventory.items():
        expected_dates = sorted(day for source_site, day in forcing if source_site == site)
        assert actual_dates == expected_dates, (site, actual_dates[:2], actual_dates[-2:])
    assert maximum_vpd_residual <= 1.0e-9, maximum_vpd_residual
    assert maximum_closure_residual <= 1.0e-12, maximum_closure_residual

    scores = rows(ART / "shape-scores.csv")
    assert len(scores) == 2 * 2 * 37
    assert {row["year"] for row in scores} == {"2024", "2025"}
    assert min(int(row["paired_days"]) for row in scores) >= 180
    transitions = rows(ART / "transition-residuals.csv")
    assert len(transitions) == 4 * 37
    verdicts = {row["cell"]: row["status"] for row in rows(ART / "verdict-matrix.csv")}
    assert verdicts["absolute canopy amplitude"] == "NOT_EVALUATED"
    assert verdicts["quantitative evergreen-floor agreement"] == "NOT_EVALUATED"
    assert verdicts["phase-transformed real-consumer chronology"] == "NOT_EVALUATED"
    assert verdicts["needle/fine-woody/decomposition consequences"] == "NOT_EVALUATED"

    expected_figures = {
        "cal07-observed-and-modeled-seasons",
        "cal07-deciduous-transition-residuals",
        "cal07-southern-seasonal-phase",
        "cal07-evidence-boundaries",
    }
    assert {path.stem for path in FIG.glob("*.svg")} == expected_figures
    assert {path.stem for path in FIG.glob("*.md")} == expected_figures
    for stem in expected_figures:
        svg = FIG / f"{stem}.svg"
        root = ET.parse(svg).getroot()
        assert root.attrib.get("role") == "img"
        assert root.attrib.get("aria-labelledby") == "title desc"
        tags = {element.tag.rsplit("}", 1)[-1] for element in root.iter()}
        assert {"title", "desc", "metadata"} <= tags
        sidecar = (FIG / f"{stem}.md").read_text(encoding="utf-8")
        for heading in (
            "## Caption",
            "## How to read it",
            "## Plain-language takeaway",
            "## Methods and source binding",
            "## Limitations",
            "## Accessibility",
        ):
            assert heading in sidecar, (stem, heading)

    print(
        "CAL-07 validation PASS: "
        f"123284 daily rows; max VPD residual={maximum_vpd_residual:.3e} Pa; "
        f"max mass residual={maximum_closure_residual:.3e} kg m-2"
    )


if __name__ == "__main__":
    main()
