#!/usr/bin/env python3
"""Validate radiation figure/sidecar inventory, bindings, and SVG metadata."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import xml.etree.ElementTree as ET


PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
INVENTORY = ARTIFACTS / "radiation-figure-manifest.json"
RESULTS = ARTIFACTS / "radiation-first-results.json"
PROTOCOL = ARTIFACTS / "radiation-comparison-manifest.json"
FIGURE_DATA = ARTIFACTS / "radiation-figure-data.json"
SVG = "{http://www.w3.org/2000/svg}"
EXPECTED_STEMS = {
    "radiation-horizontal-daily-bias",
    "radiation-winter-correlation-bias",
    "radiation-hourly-shortwave-chronology",
    "radiation-longwave-diagnostic-bias",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    inventory = json.loads(INVENTORY.read_text(encoding="utf-8"))
    if (
        inventory.get("status") != "FIGURES_COMPLETE"
        or inventory.get("radiation_results_sha256") != sha256(RESULTS)
        or inventory.get("comparison_manifest_sha256") != sha256(PROTOCOL)
    ):
        raise RuntimeError("figure manifest binding failure")
    svgs = sorted(FIGURES.glob("*.svg"))
    sidecars = sorted(FIGURES.glob("*.md"))
    if (
        len(svgs) != 4
        or len(sidecars) != 4
        or {path.stem for path in svgs} != EXPECTED_STEMS
        or {path.stem for path in sidecars} != EXPECTED_STEMS
        or inventory.get("figure_count") != 4
        or inventory.get("sidecar_count") != 4
    ):
        raise RuntimeError("figure/sidecar inventory failure")
    recorded = {item["path"]: item["sha256"] for item in inventory["files"]}
    expected_paths = {str(path) for path in [*svgs, *sidecars, FIGURE_DATA]}
    if set(recorded) != expected_paths:
        raise RuntimeError("figure manifest path-set mismatch")
    for path in [*svgs, *sidecars, FIGURE_DATA]:
        if recorded.get(str(path)) != sha256(path):
            raise RuntimeError(f"unbound figure artifact {path}")
    for svg in svgs:
        root = ET.parse(svg).getroot()
        title = root.find(f"{SVG}title")
        description = root.find(f"{SVG}desc")
        if title is None or not (title.text or "").strip():
            raise RuntimeError(f"missing SVG title {svg}")
        if description is None or len((description.text or "").strip()) < 40:
            raise RuntimeError(f"missing SVG description {svg}")
        sidecar = svg.with_suffix(".md")
        text = sidecar.read_text(encoding="utf-8")
        required = (
            f"]({svg.name})",
            "## Caption",
            "## What To Notice",
            "## Plotted Data And Population",
            "## Methods And Provenance",
            "## Uncertainty And Interpretation Limits",
        )
        if not all(value in text for value in required):
            raise RuntimeError(f"incomplete sidecar {sidecar}")
    plotted = json.loads(FIGURE_DATA.read_text(encoding="utf-8"))
    results = json.loads(RESULTS.read_text(encoding="utf-8"))
    if plotted.get("radiation_results_sha256") != sha256(RESULTS):
        raise RuntimeError("figure-data/result binding failure")
    if set(plotted.get("figures", {})) != EXPECTED_STEMS:
        raise RuntimeError("figure-data stem mismatch")
    lookup = {(row["dataset"], row["site_id"]): row for row in results["results"]}
    products = [
        ("reanalysis-era5-single-levels-timeseries", "ERA5"),
        ("reanalysis-era5-land-timeseries", "ERA5-Land"),
    ]
    sites = [
        ("snotel_mica_creek_st_joe_id", "Mica"),
        ("snotel_paradise_wa", "Paradise"),
        ("snotel_snowbird_ut", "Snowbird"),
        ("snotel_niwot_co", "Niwot"),
    ]
    expected: dict[str, list[dict[str, object]]] = {stem: [] for stem in EXPECTED_STEMS}
    for dataset, product in products:
        for site, label in sites:
            row = lookup[(dataset, site)]
            full = row["shortwave_horizontal_daily_all"]
            winter = row["shortwave_horizontal_daily_winter_events"]
            expected["radiation-horizontal-daily-bias"].append({"product": product, "site": label, "full_days": full["day_count"], "full_bias_percent": full["daily_energy_relative_bias_percent"], "winter_days": winter["day_count"], "winter_bias_percent": winter["daily_energy_relative_bias_percent"]})
            expected["radiation-winter-correlation-bias"].append({"product": product, "site": label, "days": winter["day_count"], "correlation": winter["daily_energy_correlation"], "bias_percent": winter["daily_energy_relative_bias_percent"]})
            full = row["shortwave_all"]
            winter = row["shortwave_winter_events"]
            expected["radiation-hourly-shortwave-chronology"].append({"product": product, "site": label, "full_hours": full["hour_count"], "full_peak_days": full["peak_day_count"], "full_correlation": full["hourly_correlation"], "full_peak_abs_h": full["peak_mean_absolute_circular_offset_hours"], "winter_hours": winter["hour_count"], "winter_peak_days": winter["peak_day_count"], "winter_correlation": winter["hourly_correlation"], "winter_peak_abs_h": winter["peak_mean_absolute_circular_offset_hours"]})
            full = row["longwave_all"]
            winter = row["longwave_winter_events"]
            expected["radiation-longwave-diagnostic-bias"].append({"product": product, "site": label, "full_days": full["daily_count"], "full_bias_percent": full["daily_energy_relative_bias_percent"], "winter_days": winter["daily_count"], "winter_bias_percent": winter["daily_energy_relative_bias_percent"]})
    if plotted["figures"] != expected:
        raise RuntimeError("figure plotted-data values/lane/population mismatch")
    semantic_requirements = {
        "radiation-horizontal-daily-bias": ("horizontal", "n days", "Bias"),
        "radiation-winter-correlation-bias": ("correlation", "n days", "Bias"),
        "radiation-hourly-shortwave-chronology": ("geometry", "n h", "peak days"),
        "radiation-longwave-diagnostic-bias": ("not measured", "n days", "Bias"),
    }
    for stem, tokens in semantic_requirements.items():
        text = (FIGURES / f"{stem}.md").read_text(encoding="utf-8").casefold()
        if not all(token.casefold() in text for token in tokens):
            raise RuntimeError(f"sidecar semantic requirement failure {stem}")
    print("RADIATION_FIGURES_VALID", len(svgs), len(sidecars))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
