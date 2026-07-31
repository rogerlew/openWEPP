#!/usr/bin/env python3
"""Generate deterministic EB-03 analytical interpretation figures."""

from pathlib import Path
import json
import math
import xml.etree.ElementTree as ET

import matplotlib.pyplot as plt
import matplotlib as mpl

ROOT = Path(__file__).resolve().parents[1]
FIGURES = ROOT / "artifacts" / "figures"
FIGURES.mkdir(parents=True, exist_ok=True)
mpl.rcParams["svg.hashsalt"] = "openwepp-snow-surface-eb03-v1"

SIGMA = 5.67032e-8
SECONDS = 3600.0
RHO_WATER = 1000.0


def saturation_kpa(temp_c: float) -> float:
    return 0.6108 * math.exp(17.27 * temp_c / (temp_c + 237.3))


def saturation_ice_kpa(temp_c: float) -> float:
    """SNOBAL saturation vapor pressure over ice for temperatures <= 0 C."""
    temp_k = temp_c + 273.16
    freeze_k = 273.16
    exponent = (
        -9.09718 * ((freeze_k / temp_k) - 1.0)
        - 3.56654 * math.log(freeze_k / temp_k) / math.log(10.0)
        + 0.876793 * (1.0 - temp_k / freeze_k)
        + math.log10(6.1071)
    )
    return 10.0**exponent * 0.1


def longwave(air_c: float, surface_c: float, cover: float) -> float:
    air_k = air_c + 273.16
    surface_k = surface_c + 273.16
    vapor_kpa = saturation_kpa(-12.0)
    water = 4650.0 * vapor_kpa / air_k
    clear = 59.38 + 113.7 * (air_k / 273.16) ** 6 + 96.96 * math.sqrt(water / 25.0)
    clear_emissivity = clear / (SIGMA * air_k**4)
    cloud = (0.80 - 0.50) / (0.80 - 0.15)
    all_emissivity = (1.0 - 0.84 * cloud) * clear_emissivity + 0.84 * cloud
    atmospheric = all_emissivity * SIGMA * air_k**4
    sky_view = (1.0 - cover) ** 1.6
    subcanopy = sky_view * atmospheric + (1.0 - sky_view) * SIGMA * air_k**4
    return subcanopy - SIGMA * surface_k**4


def sublimation(air_c: float, surface_c: float, cover: float, wind: float) -> tuple[float, float]:
    surface_vapor_kpa = (
        saturation_ice_kpa(surface_c)
        if surface_c <= 0.0
        else saturation_kpa(surface_c)
    )
    deficit_pa = max((surface_vapor_kpa - saturation_kpa(-12.0)) * 1000.0, 0.0)
    coefficient = (0.4 / math.log(10.0 / 0.005)) ** 2
    vapor_density_deficit = 0.0180153 * deficit_pa / (8.31432 * (air_c + 273.16))
    mass_kg_m2 = coefficient * wind * vapor_density_deficit * SECONDS * (1.0 - cover)
    surface_k = surface_c + 273.16
    latent_heat = (
        2.5e6
        - 2.95573e3 * (surface_k - 273.16)
        + 3.336e5
        + 1.6667e2 * (273.16 - surface_k)
    )
    return mass_kg_m2 / RHO_WATER, -mass_kg_m2 * latent_heat / SECONDS


def add_svg_accessibility(path: Path, title: str, description: str) -> None:
    """Add stable document-level semantics to a Matplotlib SVG."""
    ET.register_namespace("", "http://www.w3.org/2000/svg")
    root = ET.parse(path).getroot()
    namespace = "{http://www.w3.org/2000/svg}"
    title_id = f"{path.stem}-title"
    description_id = f"{path.stem}-description"
    title_element = ET.Element(f"{namespace}title", {"id": title_id})
    title_element.text = title
    description_element = ET.Element(f"{namespace}desc", {"id": description_id})
    description_element.text = description
    root.insert(0, description_element)
    root.insert(0, title_element)
    root.set("role", "img")
    root.set("aria-labelledby", f"{title_id} {description_id}")
    ET.ElementTree(root).write(path, encoding="utf-8", xml_declaration=True)


hours = list(range(24))
air = [-9.0 + 6.0 * math.sin((hour - 7) * math.pi / 12.0) for hour in hours]
surface = [-11.0 + 5.0 * math.sin((hour - 8) * math.pi / 12.0) for hour in hours]
shortwave = [max(0.0, 180.0 * math.sin((hour - 6) * math.pi / 12.0)) * 0.18 for hour in hours]
longwave_flux = [longwave(ta, ts, 0.45) for ta, ts in zip(air, surface)]
sublimation_pairs = [sublimation(ta, ts, 0.45, 3.0) for ta, ts in zip(air, surface)]
sublimation_m = [pair[0] for pair in sublimation_pairs]
latent_flux = [pair[1] for pair in sublimation_pairs]

cells = {
    "B — baseline": shortwave,
    "L — longwave": [q + l for q, l in zip(shortwave, longwave_flux)],
    "S — sublimation": [q + e for q, e in zip(shortwave, latent_flux)],
    "LS — combined": [
        q + l + e for q, l, e in zip(shortwave, longwave_flux, latent_flux)
    ],
}

plt.style.use("seaborn-v0_8-whitegrid")
fig, ax = plt.subplots(figsize=(10, 5.8))
for label, values in cells.items():
    ax.plot(hours, values, linewidth=2.2, label=label)
ax.axhline(0.0, color="#333333", linewidth=1)
ax.set(
    title="EB-03 selectors change the shared hourly surface-energy carrier",
    xlabel="Hour of day",
    ylabel="Potential surface-energy flux (W m^-2)",
    xticks=range(0, 24, 3),
)
ax.legend(loc="lower right", frameon=True)
fig.tight_layout()
hourly_path = FIGURES / "eb03-hourly-energy-cells.svg"
fig.savefig(
    hourly_path,
    metadata={"Date": "2026-07-30"},
)
plt.close(fig)
add_svg_accessibility(
    hourly_path,
    "EB-03 hourly surface-energy cells",
    "Lines compare baseline shortwave with optional longwave, sublimation latent heat, "
    "and both mechanisms over a prescribed 24-hour forcing.",
)

cumulative_mm = []
running = 0.0
for value in sublimation_m:
    running += value * 1000.0
    cumulative_mm.append(running)
fig, ax = plt.subplots(figsize=(10, 5.8))
ax.fill_between(hours, cumulative_mm, color="#4c78a8", alpha=0.25)
ax.plot(hours, cumulative_mm, color="#2f5d8a", linewidth=2.5)
ax.set(
    title="S and LS export the same diagnosed vapor mass from the snow column",
    xlabel="Hour of day",
    ylabel="Cumulative sublimated snow water equivalent (mm)",
    xticks=range(0, 24, 3),
)
fig.tight_layout()
cumulative_path = FIGURES / "eb03-cumulative-sublimation.svg"
fig.savefig(
    cumulative_path,
    metadata={"Date": "2026-07-30"},
)
plt.close(fig)
add_svg_accessibility(
    cumulative_path,
    "EB-03 cumulative sublimated snow water equivalent",
    "A rising line shows cumulative snow water equivalent exported as vapor over "
    "the prescribed 24-hour forcing.",
)

consumer = json.loads((ROOT / "artifacts/consumer-cells.json").read_text(encoding="utf-8"))
failed = [consumer["cells"][cell] for cell in ("S", "LS")]
labels = ["S", "LS"]
fig, axes = plt.subplots(1, 3, figsize=(11, 4.8))
axes[0].bar(labels, [cell["trace_row_count"] for cell in failed], color="#b44b4b")
axes[0].set(title="Days completed", ylabel="Trace rows before failure")
axes[1].bar(
    labels,
    [cell["sublimation_m_sum"] * 1000.0 for cell in failed],
    color="#4c78a8",
)
axes[1].set(title="Vapor loss", ylabel="Cumulative sublimation (mm SWE)")
axes[2].bar(
    labels,
    [cell["final_runtime_swe_m"] * 1000.0 for cell in failed],
    color="#6b8e5a",
)
axes[2].set(title="Snow still present", ylabel="Final traced SWE (mm)")
fig.suptitle("Real direct-production S and LS cells hit the thermal-provider stop")
fig.tight_layout()
failure_path = FIGURES / "eb03-real-consumer-provider-failure.svg"
fig.savefig(failure_path, metadata={"Date": "2026-07-30"})
plt.close(fig)
add_svg_accessibility(
    failure_path,
    "EB-03 real-consumer thermal-provider failure",
    "Three bar plots show days completed, cumulative sublimation, and snow water "
    "equivalent still present when S and LS fail at the absolute-zero provider bound.",
)
