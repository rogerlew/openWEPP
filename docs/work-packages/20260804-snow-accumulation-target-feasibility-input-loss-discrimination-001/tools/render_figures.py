#!/usr/bin/env python3
"""Render the frozen input-versus-loss diagnostic figures."""

from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path
from typing import Any

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np


REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
FREEZE_PATH = PACKAGE / "artifacts/analysis-freeze.json"
PRCPSA_PATH = REPO / "tests/fixtures/snotel_observed/observations/provenance/snotel_snowbird_ut_prcpsa_diagnostic.json"
TARGET = REPO / "target/snow_accumulation_target_feasibility_input_loss_discrimination_v2"
TABLES = TARGET / "tables"
FIGURES = PACKAGE / "artifacts/figures"
SOURCE_TABLES = TARGET / "figure-source-tables"
COLORS = {
    "Mica Creek": "#4477AA",
    "Niwot": "#66CCEE",
    "Paradise": "#228833",
    "Snowbird": "#CC6677",
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def read_csv(name: str) -> list[dict[str, str]]:
    with (TABLES / name).open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def save_figure(figure: plt.Figure, name: str) -> Path:
    FIGURES.mkdir(parents=True, exist_ok=True)
    path = FIGURES / name
    figure.savefig(path, dpi=180, bbox_inches="tight", facecolor="white")
    plt.close(figure)
    return path


def mass_pathways(site_rows: list[dict[str, str]]) -> Path:
    labels = [row["display_site"] for row in site_rows]
    series = [
        ("All-phase ceiling", "median_current_input_mass_ceiling_ratio", "#AA4499"),
        ("Storage-effective input", "median_storage_effective_input_ratio", "#DDCC77"),
        ("Observed-date storage", "median_observed_date_modeled_storage_ratio", "#44AA99"),
        ("Within-window model peak", "median_within_window_modeled_peak_ratio", "#332288"),
    ]
    x = np.arange(len(labels))
    width = 0.19
    figure, axis = plt.subplots(figsize=(10.5, 5.8))
    for offset, (label, field, color) in enumerate(series):
        values = [float(row[field]) for row in site_rows]
        bars = axis.bar(x + (offset - 1.5) * width, values, width, label=label, color=color)
        axis.bar_label(bars, fmt="%.2f", fontsize=7, padding=2)
    axis.axhline(1.0, color="black", linewidth=1.0, linestyle="--", label="Observed peak")
    axis.axhline(0.95, color="#888888", linewidth=0.8, linestyle=":", label="Frozen ceiling screen")
    axis.set_xticks(x, labels)
    axis.set_ylabel("Site median ratio to observed peak SWE")
    axis.set_title("Seasonal mass headroom and modeled storage")
    axis.set_ylim(0, max(1.75, axis.get_ylim()[1]))
    axis.grid(axis="y", alpha=0.2)
    axis.legend(ncol=2, fontsize=8, loc="upper right")
    figure.tight_layout()
    return save_figure(figure, "mass-ceiling-pathways.png")


def cold_event_ratios(event_rows: list[dict[str, str]]) -> Path:
    sites = list(COLORS)
    fields = [
        ("All-phase input", "all_phase_to_observed_gain_ratio"),
        ("Modeled snowfall", "modeled_snowfall_to_observed_gain_ratio"),
        ("Modeled storage change", "modeled_storage_change_to_observed_gain_ratio"),
    ]
    figure, axes = plt.subplots(1, 3, figsize=(13.5, 5.2), sharey=True)
    for axis, (title, field) in zip(axes, fields):
        values = [
            [float(row[field]) for row in event_rows if row["display_site"] == site]
            for site in sites
        ]
        plot = axis.boxplot(values, showfliers=False, patch_artist=True, widths=0.65)
        for patch, site in zip(plot["boxes"], sites):
            patch.set_facecolor(COLORS[site])
            patch.set_alpha(0.75)
        axis.axhline(1.0, color="black", linewidth=1.0, linestyle="--")
        axis.axhline(0.8, color="#888888", linewidth=0.8, linestyle=":")
        axis.set_xticks(range(1, len(sites) + 1), sites, rotation=28, ha="right")
        axis.set_title(title)
        axis.grid(axis="y", alpha=0.2)
    axes[0].set_ylabel("Event ratio to observed cold WTEQ gain")
    axes[0].set_ylim(-0.25, 3.5)
    figure.suptitle("Timing-tolerant cold accumulation-event comparisons")
    figure.tight_layout()
    return save_figure(figure, "cold-event-input-storage-ratios.png")


def dry_loss(dry_rows: list[dict[str, str]]) -> Path:
    rows = [row for row in dry_rows if row["temperature_stratum"] == "all"]
    maximum = max(
        max(float(row["observed_wteq_loss_m"]), float(row["modeled_pack_loss_m"]))
        for row in rows
    )
    figure, axis = plt.subplots(figsize=(7.2, 6.4))
    for site, color in COLORS.items():
        selected = [row for row in rows if row["display_site"] == site]
        axis.scatter(
            [float(row["observed_wteq_loss_m"]) for row in selected],
            [float(row["modeled_pack_loss_m"]) for row in selected],
            label=site,
            color=color,
            alpha=0.75,
            s=35,
            edgecolors="white",
            linewidths=0.4,
        )
    axis.plot([0, maximum], [0, maximum], color="black", linestyle="--", linewidth=1.0)
    axis.set_xlabel("Observed WTEQ loss on guarded dry intervals (m per WY)")
    axis.set_ylabel("Modeled pack loss on guarded dry intervals (m per WY)")
    axis.set_title("Annual-first guarded dry-interval loss comparison")
    axis.grid(alpha=0.2)
    axis.legend()
    figure.tight_layout()
    return save_figure(figure, "dry-interval-loss-comparison.png")


def snowbird_chain(site_rows: list[dict[str, str]]) -> tuple[Path, Path]:
    snowbird = next(row for row in site_rows if row["display_site"] == "Snowbird")
    freeze = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    expected_hash = freeze["source_identity_expectations"][
        "snowbird_prcpsa_sha256_at_intake_commit"
    ]
    if sha256(PRCPSA_PATH) != expected_hash:
        raise RuntimeError("Snowbird PRCPSA identity differs from frozen intake hash")
    prcpsa = json.loads(PRCPSA_PATH.read_text(encoding="utf-8"))
    ratios = prcpsa["primary_window_analysis"]["ratios"]
    rows = [
        {"quantity": "Unadjusted PRCP / pillow peak", "ratio": ratios["prcp_sum_to_pillow_peak_swe"]["median"], "authority": "derived gauge diagnostic"},
        {"quantity": "PRCPSA / pillow peak", "ratio": ratios["prcpsa_sum_to_pillow_peak_swe"]["median"], "authority": "WTEQ-dependent diagnostic"},
        {"quantity": "Fixture all-phase / pillow peak", "ratio": float(snowbird["median_current_input_mass_ceiling_ratio"]), "authority": "current-input ceiling"},
        {"quantity": "Storage-effective input / pillow peak", "ratio": float(snowbird["median_storage_effective_input_ratio"]), "authority": "modeled input"},
        {"quantity": "Modeled pack loss / pillow peak", "ratio": float(snowbird["median_modeled_pack_loss_to_observed_peak"]), "authority": "modeled loss"},
        {"quantity": "Observed-date modeled storage / pillow peak", "ratio": float(snowbird["median_observed_date_modeled_storage_ratio"]), "authority": "modeled state"},
        {"quantity": "Within-window modeled peak / pillow peak", "ratio": float(snowbird["median_within_window_modeled_peak_ratio"]), "authority": "modeled state"},
    ]
    SOURCE_TABLES.mkdir(parents=True, exist_ok=True)
    source = SOURCE_TABLES / "snowbird-evidence-chain.csv"
    with source.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    figure, axis = plt.subplots(figsize=(10.5, 5.8))
    labels = [row["quantity"] for row in rows]
    values = [float(row["ratio"]) for row in rows]
    colors = ["#999999", "#EE7733", "#AA4499", "#DDCC77", "#CC6677", "#44AA99", "#332288"]
    bars = axis.barh(range(len(rows)), values, color=colors)
    axis.bar_label(bars, fmt="%.2f", padding=3, fontsize=8)
    axis.axvline(1.0, color="black", linewidth=1.0, linestyle="--")
    axis.set_yticks(range(len(rows)), labels)
    axis.invert_yaxis()
    axis.set_xlabel("Site median ratio to pillow peak SWE")
    axis.set_title("Snowbird evidence chain (different authority classes; not additive)")
    axis.set_xlim(0, 1.45)
    axis.grid(axis="x", alpha=0.2)
    figure.tight_layout()
    return save_figure(figure, "snowbird-evidence-chain.png"), source


def main() -> int:
    site_rows = read_csv("site-summary.csv")
    event_rows = read_csv("cold-events.csv")
    dry_rows = read_csv("dry-annual.csv")
    outputs = [
        (mass_pathways(site_rows), TABLES / "site-summary.csv"),
        (cold_event_ratios(event_rows), TABLES / "cold-events.csv"),
        (dry_loss(dry_rows), TABLES / "dry-annual.csv"),
    ]
    snowbird_figure, snowbird_source = snowbird_chain(site_rows)
    outputs.append((snowbird_figure, snowbird_source))
    manifest: dict[str, Any] = {"schema_version": 1, "figures": []}
    for figure, source in outputs:
        manifest["figures"].append(
            {
                "figure": str(figure.relative_to(REPO)),
                "figure_sha256": sha256(figure),
                "source": str(source.relative_to(REPO)),
                "source_sha256": sha256(source),
            }
        )
    manifest_path = TARGET / "figure-manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(manifest, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
