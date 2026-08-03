#!/usr/bin/env python3
"""Generate deterministic radiation-first SVG figures and Markdown sidecars."""

from __future__ import annotations

import hashlib
import html
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np


PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
RESULTS = ARTIFACTS / "radiation-first-results.json"
PROTOCOL = ARTIFACTS / "radiation-comparison-manifest.json"
FIGURES = ARTIFACTS / "figures"
INVENTORY = ARTIFACTS / "radiation-figure-manifest.json"
FIGURE_DATA = ARTIFACTS / "radiation-figure-data.json"
SITES = [
    ("snotel_mica_creek_st_joe_id", "Mica"),
    ("snotel_paradise_wa", "Paradise"),
    ("snotel_snowbird_ut", "Snowbird"),
    ("snotel_niwot_co", "Niwot"),
]
PRODUCTS = [
    ("reanalysis-era5-single-levels-timeseries", "ERA5", "#31688e"),
    ("reanalysis-era5-land-timeseries", "ERA5-Land", "#d1495b"),
]


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save_figure(fig: plt.Figure, stem: str, title: str, description: str) -> Path:
    path = FIGURES / f"{stem}.svg"
    fig.savefig(
        path,
        format="svg",
        bbox_inches="tight",
        metadata={"Title": title, "Description": description, "Date": "2026-08-03"},
    )
    plt.close(fig)
    svg = path.read_text(encoding="utf-8")
    svg = svg.replace("</title>", f"</title>\n <desc>{html.escape(description)}</desc>", 1)
    path.write_text(svg, encoding="utf-8")
    return path


def write_sidecar(
    stem: str, title: str, caption: str, notice: str, plotted_data: str, limits: str
) -> Path:
    path = FIGURES / f"{stem}.md"
    path.write_text(
        f"# {title}\n\n![{title}]({stem}.svg)\n\n"
        f"## Caption\n\n{caption}\n\n"
        f"## What To Notice\n\n{notice}\n\n"
        f"## Plotted Data And Population\n\n{plotted_data}\n\n"
        "## Methods And Provenance\n\n"
        "Values come from `../radiation-first-results.json`, which binds the "
        "validated ERA5/ERA5-Land inputs, retained climate/comparator identities, "
        "and `../radiation-comparison-manifest.json`. ERA intervals use "
        "`valid_time - 1 h` and fixed local standard time. No precipitation byte "
        "or multiplier was modified.\n\n"
        f"## Uncertainty And Interpretation Limits\n\n{limits}\n",
        encoding="utf-8",
    )
    return path


def style_axis(axis: plt.Axes, zero: bool = False) -> None:
    axis.spines[["top", "right"]].set_visible(False)
    axis.grid(axis="y", color="#d9d9d9", linewidth=0.7, zorder=0)
    if zero:
        axis.axhline(0.0, color="#333333", linewidth=0.9, zorder=1)


def main() -> int:
    if INVENTORY.exists() or FIGURE_DATA.exists() or FIGURES.exists():
        raise RuntimeError("refusing to overwrite existing radiation figures or manifest")
    data = json.loads(RESULTS.read_text(encoding="utf-8"))
    if data.get("status") != "RADIATION_FIRST_COMPLETE" or data.get("result_count") != 8:
        raise RuntimeError("radiation result receipt is not complete")
    FIGURES.mkdir(parents=True)
    plt.rcParams.update({
        "font.family": "DejaVu Sans",
        "font.size": 10,
        "svg.hashsalt": "snow-hourly-era5-radiation-v1",
    })
    lookup = {(row["dataset"], row["site_id"]): row for row in data["results"]}
    created: list[Path] = []
    figure_data: dict[str, list[dict[str, object]]] = {}

    # Figure 1: the primary like-for-like horizontal energy comparison.
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.4), sharey=True)
    x = np.arange(len(SITES))
    width = 0.34
    for axis, (metric, panel) in zip(
        axes,
        [
            ("shortwave_horizontal_daily_all", "Full record"),
            ("shortwave_horizontal_daily_winter_events", "Wet winter events"),
        ],
        strict=True,
    ):
        for index, (dataset, product, color) in enumerate(PRODUCTS):
            values = [lookup[(dataset, site)][metric]["daily_energy_relative_bias_percent"] for site, _ in SITES]
            axis.bar(
                x + (index - 0.5) * width,
                values,
                width,
                label=product,
                color=color,
                hatch="//" if product == "ERA5-Land" else None,
                edgecolor="#222222",
                linewidth=0.5,
                zorder=2,
            )
        axis.set_title(panel)
        axis.set_xticks(x, [label for _, label in SITES])
        axis.set_ylabel("ERA minus retained horizontal daily energy (%)")
        style_axis(axis, zero=True)
    axes[1].legend(frameon=False, loc="upper left")
    fig.suptitle("Horizontal shortwave energy differs most during wet winter at mountain sites", fontweight="bold")
    fig.subplots_adjust(top=0.82)
    stem = "radiation-horizontal-daily-bias"
    rows = []
    for dataset, product, _ in PRODUCTS:
        for site, label in SITES:
            full = lookup[(dataset, site)]["shortwave_horizontal_daily_all"]
            winter = lookup[(dataset, site)]["shortwave_horizontal_daily_winter_events"]
            rows.append({"product": product, "site": label, "full_days": full["day_count"], "full_bias_percent": full["daily_energy_relative_bias_percent"], "winter_days": winter["day_count"], "winter_bias_percent": winter["daily_energy_relative_bias_percent"]})
    figure_data[stem] = rows
    table = "| Product | Site | Full n days | Full bias | Winter n days | Winter bias |\n|---|---|---:|---:|---:|---:|\n" + "\n".join(f'| {row["product"]} | {row["site"]} | {row["full_days"]} | {row["full_bias_percent"]:+.2f}% | {row["winter_days"]} | {row["winter_bias_percent"]:+.2f}% |' for row in rows)
    created.append(save_figure(fig, stem, "Horizontal daily shortwave relative bias", "Grouped bars compare ERA5 and ERA5-Land horizontal daily shortwave energy with retained daily climate radiation for the full record and wet winter events."))
    created.append(write_sidecar(
        stem,
        "Horizontal Daily Shortwave Relative Bias",
        "ERA horizontal daily shortwave energy is compared like-for-like with the retained daily climate `rad` field. Bars show summed ERA-minus-retained relative bias.",
        "Full-record agreement can mask winter structure: Niwot is only about +1.5% overall but about +23% on wet winter days. Snowbird reaches about +28.5%; Paradise is slightly lower than retained forcing.",
        table,
        "This is comparison with retained Daymet/gridMET-derived climate forcing, not direct radiometer validation. Bias does not identify which provider is correct or establish a transferable correction.",
    ))

    # Figure 2: primary daily chronology correlation versus magnitude bias.
    fig, axis = plt.subplots(figsize=(7.8, 5.2))
    markers = {site: marker for (site, _), marker in zip(SITES, ["o", "s", "^", "D"], strict=True)}
    for dataset, product, color in PRODUCTS:
        for site, label in SITES:
            row = lookup[(dataset, site)]["shortwave_horizontal_daily_winter_events"]
            axis.scatter(row["daily_energy_correlation"], row["daily_energy_relative_bias_percent"], s=75, marker=markers[site], facecolor=color if product == "ERA5" else "none", edgecolor=color, linewidth=1.5, zorder=3)
    for site, label in SITES:
        points = [lookup[(dataset, site)]["shortwave_horizontal_daily_winter_events"] for dataset, _, _ in PRODUCTS]
        axis.annotate(label, (np.mean([point["daily_energy_correlation"] for point in points]), np.mean([point["daily_energy_relative_bias_percent"] for point in points])), xytext=(5, 4), textcoords="offset points", fontsize=8)
    style_axis(axis, zero=True)
    axis.set_xlabel("Wet-winter daily-energy correlation")
    axis.set_ylabel("Wet-winter relative bias (%)")
    axis.set_title("Correlation does not remove wet-winter magnitude divergence", fontweight="bold")
    handles = [plt.Line2D([], [], marker="o", linestyle="", markerfacecolor=color if product == "ERA5" else "none", markeredgecolor=color, markeredgewidth=1.5, label=product) for _, product, color in PRODUCTS]
    axis.legend(handles=handles, frameon=False)
    stem = "radiation-winter-correlation-bias"
    rows = []
    for dataset, product, _ in PRODUCTS:
        for site, label in SITES:
            metric = lookup[(dataset, site)]["shortwave_horizontal_daily_winter_events"]
            rows.append({"product": product, "site": label, "days": metric["day_count"], "correlation": metric["daily_energy_correlation"], "bias_percent": metric["daily_energy_relative_bias_percent"]})
    figure_data[stem] = rows
    table = "| Product | Site | n days | Daily r | Bias |\n|---|---|---:|---:|---:|\n" + "\n".join(f'| {row["product"]} | {row["site"]} | {row["days"]} | {row["correlation"]:.4f} | {row["bias_percent"]:+.2f}% |' for row in rows)
    created.append(save_figure(fig, stem, "Wet-winter shortwave correlation and bias", "Scatterplot shows daily-energy correlation against relative energy bias for each product and site on complete wet winter days."))
    created.append(write_sidecar(
        stem,
        "Wet-Winter Shortwave Correlation And Bias",
        "Each point combines daily horizontal shortwave correlation and summed relative bias for complete wet November-March days selected by unchanged retained precipitation.",
        "Both products cluster tightly by site. Mica has the strongest winter correlation, while Snowbird and Niwot show the largest positive winter energy biases.",
        table,
        "Correlation measures chronology, not agreement in magnitude. The wet-day population is selected from calibration forcing and is diagnostic rather than independent validation.",
    ))

    # Figure 3: the explicitly geometry-confounded hourly shortwave lane.
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.4))
    for axis, (metric, panel) in zip(axes, [("shortwave_all", "Full record"), ("shortwave_winter_events", "Wet winter events")], strict=True):
        for dataset, product, color in PRODUCTS:
            correlations = [lookup[(dataset, site)][metric]["hourly_correlation"] for site, _ in SITES]
            peaks = [lookup[(dataset, site)][metric]["peak_mean_absolute_circular_offset_hours"] for site, _ in SITES]
            axis.scatter(correlations, peaks, color=color, marker="o" if product == "ERA5" else "X", s=70, label=product, zorder=3)
        for site, label in SITES:
            points = [lookup[(dataset, site)][metric] for dataset, _, _ in PRODUCTS]
            axis.annotate(label, (np.mean([point["hourly_correlation"] for point in points]), np.mean([point["peak_mean_absolute_circular_offset_hours"] for point in points])), xytext=(5, 3), textcoords="offset points", fontsize=8)
        axis.set_title(panel)
        axis.set_xlabel("Hourly correlation")
        axis.set_ylabel("Fixed-local peak mean absolute offset (h)")
        style_axis(axis)
    axes[1].legend(frameon=False)
    fig.suptitle("Hourly SIMIMPL shortwave chronology is geometry-confounded", fontweight="bold")
    fig.subplots_adjust(top=0.82)
    stem = "radiation-hourly-shortwave-chronology"
    rows = []
    for dataset, product, _ in PRODUCTS:
        for site, label in SITES:
            full = lookup[(dataset, site)]["shortwave_all"]
            winter = lookup[(dataset, site)]["shortwave_winter_events"]
            rows.append({"product": product, "site": label, "full_hours": full["hour_count"], "full_peak_days": full["peak_day_count"], "full_correlation": full["hourly_correlation"], "full_peak_abs_h": full["peak_mean_absolute_circular_offset_hours"], "winter_hours": winter["hour_count"], "winter_peak_days": winter["peak_day_count"], "winter_correlation": winter["hourly_correlation"], "winter_peak_abs_h": winter["peak_mean_absolute_circular_offset_hours"]})
    figure_data[stem] = rows
    table = "| Product | Site | Full n h / peak days | Full r / peak abs h | Winter n h / peak days | Winter r / peak abs h |\n|---|---|---:|---:|---:|---:|\n" + "\n".join(f'| {row["product"]} | {row["site"]} | {row["full_hours"]} / {row["full_peak_days"]} | {row["full_correlation"]:.4f} / {row["full_peak_abs_h"]:.2f} | {row["winter_hours"]} / {row["winter_peak_days"]} | {row["winter_correlation"]:.4f} / {row["winter_peak_abs_h"]:.2f} |' for row in rows)
    created.append(save_figure(fig, stem, "Geometry-confounded hourly shortwave chronology", "Scatterplots show hourly correlation and fixed-local-standard daily peak offset against slope/aspect-transformed SIMIMPL shortwave."))
    created.append(write_sidecar(
        stem,
        "Geometry-Confounded Hourly Shortwave Chronology",
        "ERA horizontal hourly shortwave is compared with the retained slope/aspect-transformed SIMIMPL hourly synthesis after interval-start alignment.",
        "The one-hour interval correction yields high correlations and peak offsets generally near one hour or less. ERA5 and ERA5-Land are nearly coincident by site.",
        table,
        "The planes differ. This figure is chronology sensitivity only and cannot support magnitude, provider-accuracy, terrain-projection, or snow-improvement claims. The former Snowbird +84% magnitude interpretation is withdrawn.",
    ))

    # Figure 4: longwave, with its non-observational status visible in the title.
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.4), sharey=True)
    for axis, (metric, panel) in zip(axes, [("longwave_all", "Full record"), ("longwave_winter_events", "Wet winter events")], strict=True):
        for index, (dataset, product, color) in enumerate(PRODUCTS):
            values = [lookup[(dataset, site)][metric]["daily_energy_relative_bias_percent"] for site, _ in SITES]
            axis.bar(x + (index - 0.5) * width, values, width, label=product, color=color, hatch="//" if product == "ERA5-Land" else None, edgecolor="#222222", linewidth=0.5, zorder=2)
        axis.set_title(panel)
        axis.set_xticks(x, [label for _, label in SITES])
        axis.set_ylabel("ERA minus diagnostic SIMIMPL longwave (%)")
        style_axis(axis, zero=True)
    axes[1].legend(frameon=False)
    fig.suptitle("Longwave differences are diagnostic—not observational validation", fontweight="bold")
    fig.subplots_adjust(top=0.82)
    stem = "radiation-longwave-diagnostic-bias"
    rows = []
    for dataset, product, _ in PRODUCTS:
        for site, label in SITES:
            full = lookup[(dataset, site)]["longwave_all"]
            winter = lookup[(dataset, site)]["longwave_winter_events"]
            rows.append({"product": product, "site": label, "full_days": full["daily_count"], "full_bias_percent": full["daily_energy_relative_bias_percent"], "winter_days": winter["daily_count"], "winter_bias_percent": winter["daily_energy_relative_bias_percent"]})
    figure_data[stem] = rows
    table = "| Product | Site | Full n days | Full bias | Winter n days | Winter bias |\n|---|---|---:|---:|---:|---:|\n" + "\n".join(f'| {row["product"]} | {row["site"]} | {row["full_days"]} | {row["full_bias_percent"]:+.2f}% | {row["winter_days"]} | {row["winter_bias_percent"]:+.2f}% |' for row in rows)
    created.append(save_figure(fig, stem, "Diagnostic longwave relative bias", "Grouped bars compare ERA downward longwave with the non-observational SIMIMPL diagnostic longwave estimate."))
    created.append(write_sidecar(
        stem,
        "Diagnostic Longwave Relative Bias",
        "Bars show summed daily-energy relative bias against SIMIMPL diagnostic longwave for complete full-record and wet-winter days.",
        "ERA longwave is lower than the diagnostic estimate at every site and in both products, with larger negative winter differences at Niwot and Snowbird.",
        table,
        "The comparator is an emissivity estimate derived from retained temperature and cloud fraction, not measured longwave. Differences cannot determine which field is correct and cannot justify provider admission or model tuning.",
    ))

    FIGURE_DATA.write_text(json.dumps({"schema": "snow-hourly-era5-radiation-figure-data-v1", "radiation_results_sha256": sha256(RESULTS), "figures": figure_data}, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    created.append(FIGURE_DATA)

    inventory = {
        "schema": "snow-hourly-era5-radiation-figure-manifest-v1",
        "status": "FIGURES_COMPLETE",
        "radiation_results_sha256": sha256(RESULTS),
        "comparison_manifest_sha256": sha256(PROTOCOL),
        "figure_count": 4,
        "sidecar_count": 4,
        "files": [{"path": str(path), "sha256": sha256(path)} for path in sorted(created)],
    }
    INVENTORY.write_text(json.dumps(inventory, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print("FIGURES_COMPLETE", inventory["figure_count"], inventory["sidecar_count"])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
