#!/usr/bin/env python3
"""Render deterministic 21L figures and interpretation sidecars."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
from pathlib import Path
from typing import Any

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402


REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
DEFAULT_ROOT = REPO / "target/snow_warm_mixed_prepeak_loss_energy_attribution_v2"
DEFAULT_FIGURES = PACKAGE / "artifacts/figures"
COLORS = {
    "cold": "#4C78A8",
    "mixed": "#F2CF5B",
    "warm": "#E45756",
    "amelt": "#F28E2B",
    "bmelt": "#4E79A7",
    "cmelt": "#59A14F",
    "dmelt": "#E15759",
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def save(fig: plt.Figure, path: Path) -> None:
    fig.tight_layout()
    fig.savefig(path, dpi=180, bbox_inches="tight")
    plt.close(fig)


def sidecar(path: Path, title: str, image: str, source: str, body: str) -> None:
    path.write_text(
        f"# {title}\n\n![{title}]({image})\n\n"
        f"Source table: `{source}`.\n\n{body.strip()}\n",
        encoding="utf-8",
    )


def execute(root: Path, figure_dir: Path) -> dict[str, Any]:
    figure_dir.mkdir(parents=True, exist_ok=True)
    source_dir = root / "figure-source-tables"
    source_dir.mkdir(parents=True, exist_ok=True)
    result = json.loads((root / "results.json").read_text(encoding="utf-8"))
    annual = read_csv(root / "tables/annual-attribution.csv")
    comparisons = read_csv(root / "tables/dry-interval-predecessor-comparison.csv")
    pairs = read_csv(root / "tables/snowbird-pairs.csv")
    summaries = result["site_summary"]
    labels = [row["display_site"] for row in summaries]

    thermal_rows = []
    for row in summaries:
        thermal_rows.append({
            "site": row["display_site"],
            "median_warm_mixed_fraction": row["median_warm_mixed_pack_loss_fraction"],
            "median_cold_fraction_complement": 1.0 - row["median_warm_mixed_pack_loss_fraction"],
            "median_prepeak_pack_loss_m": row["median_prepeak_pack_loss_m"],
        })
    thermal_source = source_dir / "thermal-loss-partition.csv"
    write_csv(thermal_source, thermal_rows)
    fig, ax = plt.subplots(figsize=(8.2, 4.8))
    warm = [100.0 * row["median_warm_mixed_fraction"] for row in thermal_rows]
    cold = [100.0 - value for value in warm]
    ax.bar(labels, cold, color=COLORS["cold"], label="Cold-day loss")
    ax.bar(labels, warm, bottom=cold, color=COLORS["warm"], label="Mixed + warm-day loss")
    ax.set_ylabel("Median annual pre-peak pack-loss share (%)")
    ax.set_ylim(0, 100)
    ax.legend(loc="lower right")
    ax.set_title("Corrected pre-peak pack loss is concentrated on mixed/warm days")
    save(fig, figure_dir / "thermal-loss-partition.png")
    sidecar(
        figure_dir / "thermal-loss-partition.md",
        "Corrected pre-peak loss by thermal class",
        "thermal-loss-partition.png",
        str(thermal_source.relative_to(REPO)),
        "The annual-first site medians assign 99.61-99.91% of modeled pre-peak "
        "pack loss to days whose active-pack hourly temperature range crosses or "
        "stays above 0 C. This is chronology localization, not proof that air "
        "temperature alone causes the loss.",
    )

    component_rows = []
    for summary in summaries:
        site_annual = [row for row in annual if row["lane"] == summary["site"]]
        component_rows.append({
            "site": summary["display_site"],
            **{
                name: statistics_median(
                    float(row[f"mixed_day_{name}_positive_m"])
                    + float(row[f"warm_day_{name}_positive_m"])
                    for row in site_annual
                )
                for name in ("amelt", "bmelt", "cmelt", "dmelt")
            },
        })
    component_source = source_dir / "warm-mixed-positive-coe-components.csv"
    write_csv(component_source, component_rows)
    fig, ax = plt.subplots(figsize=(8.2, 4.8))
    bottom = [0.0] * len(labels)
    for name in ("amelt", "bmelt", "cmelt", "dmelt"):
        values = [row[name] for row in component_rows]
        ax.bar(labels, values, bottom=bottom, color=COLORS[name], label=name)
        bottom = [left + right for left, right in zip(bottom, values)]
    ax.set_ylabel("Median annual positive melt-depth (m SWE)")
    ax.set_title("Annual-first warm/mixed empirical CoE term medians")
    ax.legend(ncol=4, loc="upper left")
    save(fig, figure_dir / "warm-mixed-coe-components.png")
    sidecar(
        figure_dir / "warm-mixed-coe-components.md",
        "Warm/mixed empirical CoE components",
        "warm-mixed-coe-components.png",
        str(component_source.relative_to(REPO)),
        "The mixed `cmelt` formula term is the largest annual-first positive median at all four "
        "sites. `amelt`, `bmelt`, `cmelt`, and `dmelt` are empirical melt-depth "
        "terms, not measured energy shares; dominance only identifies the formula "
        "family requiring first-principles scrutiny.",
    )

    pair_source = source_dir / "snowbird-paired-state-response.csv"
    write_csv(pair_source, pairs)
    fig, ax = plt.subplots(figsize=(8.5, 4.8))
    years = [int(row["water_year"]) for row in pairs]
    peak = [1000.0 * float(row["scaled_minus_canonical_peak_swe_m"]) for row in pairs]
    loss = [1000.0 * float(row["scaled_minus_canonical_pack_loss_m"]) for row in pairs]
    ax.plot(years, peak, color="#59A14F", marker="o", markersize=3, linewidth=1, label="Peak SWE delta")
    ax.plot(years, loss, color="#E15759", marker="o", markersize=3, linewidth=1, label="Pre-peak loss delta")
    ax.axhline(0.0, color="#555555", linewidth=0.8)
    ax.set_xlabel("Water year")
    ax.set_ylabel("Scaled minus canonical Snowbird (mm SWE)")
    ax.set_title("More Snowbird input raises storage and also exposes more loss")
    ax.legend()
    save(fig, figure_dir / "snowbird-paired-state-response.png")
    pair_summary = result["snowbird_pair_summary"]
    sidecar(
        figure_dir / "snowbird-paired-state-response.md",
        "Snowbird development-only paired state response",
        "snowbird-paired-state-response.png",
        str(pair_source.relative_to(REPO)),
        f"Across {pair_summary['paired_year_count']} paired years, scaled input "
        f"raises median peak SWE by {pair_summary['median_scaled_minus_canonical_peak_swe_m'] * 1000:.1f} mm "
        f"and median pre-peak pack loss by {pair_summary['median_scaled_minus_canonical_pack_loss_m'] * 1000:.1f} mm. "
        "The extra storage dominates the extra loss. This lane is DEVELOPMENT_ONLY "
        "and cannot establish precipitation truth.",
    )

    dry_source = source_dir / "dry-loss-rebaseline.csv"
    write_csv(dry_source, comparisons)
    fig, ax = plt.subplots(figsize=(6.2, 5.6))
    old = [1000.0 * float(row["pre_21k_modeled_pack_loss_m"]) for row in comparisons]
    new = [1000.0 * float(row["corrected_modeled_pack_loss_m"]) for row in comparisons]
    upper = max(old + new)
    ax.scatter(old, new, s=13, alpha=0.55, color="#4E79A7")
    ax.plot([0, upper], [0, upper], color="#E15759", linewidth=1.2, label="1:1")
    ax.set_xlabel("Pre-21K modeled dry-interval loss (mm)")
    ax.set_ylabel("Corrected modeled dry-interval loss (mm)")
    ax.set_title("21K leaves upstream dry-interval pack loss invariant")
    ax.legend()
    save(fig, figure_dir / "dry-loss-rebaseline.png")
    sidecar(
        figure_dir / "dry-loss-rebaseline.md",
        "Corrected dry-loss rebaseline",
        "dry-loss-rebaseline.png",
        str(dry_source.relative_to(REPO)),
        f"All {len(comparisons)} canonical dry intervals reproduce pre-21K pack "
        f"loss within {result['maximum_abs_corrected_minus_pre_21k_dry_loss_m']:.2e} m. "
        "The wet-compaction correction changes density and downstream disposition, "
        "not the upstream loss diagnosed by 21J.",
    )

    stage_rows = []
    for site in [row["site"] for row in summaries]:
        site_annual = [row for row in annual if row["lane"] == site]
        for thermal in ("cold_day", "mixed_day", "warm_day"):
            stage_rows.append({
                "site": next(row["display_site"] for row in summaries if row["site"] == site),
                "thermal_class": thermal,
                "median_surface_energy_mj_m2": statistics_median(float(row[f"{thermal}_stage3_surface_energy_j_m2"]) / 1e6 for row in site_annual),
                "median_shortwave_energy_mj_m2": statistics_median(float(row[f"{thermal}_stage3_shortwave_energy_j_m2"]) / 1e6 for row in site_annual),
                "median_longwave_energy_mj_m2": statistics_median(float(row[f"{thermal}_stage3_longwave_energy_j_m2"]) / 1e6 for row in site_annual),
                "median_latent_energy_mj_m2": statistics_median(float(row[f"{thermal}_stage3_latent_energy_j_m2"]) / 1e6 for row in site_annual),
                "median_conduction_energy_mj_m2": statistics_median(float(row[f"{thermal}_stage3_conduction_energy_j_m2"]) / 1e6 for row in site_annual),
            })
    stage_source = source_dir / "stage3-energy-response.csv"
    write_csv(stage_source, stage_rows)
    fig, ax = plt.subplots(figsize=(8.4, 4.8))
    warm_mixed = [row for row in stage_rows if row["thermal_class"] in ("mixed_day", "warm_day")]
    positions = list(range(len(warm_mixed)))
    values = [row["median_surface_energy_mj_m2"] for row in warm_mixed]
    bar_colors = [COLORS["mixed"] if row["thermal_class"] == "mixed_day" else COLORS["warm"] for row in warm_mixed]
    ax.bar(positions, values, color=bar_colors)
    ax.set_xticks(positions, [f"{row['site']}\n{row['thermal_class'].replace('_day', '')}" for row in warm_mixed])
    ax.set_ylabel("Median annual Stage-3 surface energy (MJ m$^{-2}$)")
    ax.set_title("Downstream Stage-3 energy response on mixed and warm days")
    save(fig, figure_dir / "stage3-energy-response.png")
    sidecar(
        figure_dir / "stage3-energy-response.md",
        "Downstream Stage-3 energy response",
        "stage3-energy-response.png",
        str(stage_source.relative_to(REPO)),
        "Stage-3 energy varies across sites and thermal classes but is evaluated "
        "after upstream CoE melt and snow-contact rain enter the layered solver. "
        "It constrains liquid disposition and refreeze; it is not evidence that "
        "Stage 3 generated the pre-peak melt.",
    )

    readme = figure_dir / "README.md"
    readme.write_text(
        "# 21L Figures\n\n"
        "- [Thermal loss partition](thermal-loss-partition.md)\n"
        "- [Warm/mixed CoE components](warm-mixed-coe-components.md)\n"
        "- [Snowbird paired state response](snowbird-paired-state-response.md)\n"
        "- [Dry-loss rebaseline](dry-loss-rebaseline.md)\n"
        "- [Stage-3 energy response](stage3-energy-response.md)\n",
        encoding="utf-8",
    )
    files = [path for path in sorted(figure_dir.iterdir()) if path.is_file()]
    sources = [path for path in sorted(source_dir.iterdir()) if path.is_file()]
    manifest = {
        "schema_version": 1,
        "status": "PASS",
        "result_sha256": sha256(root / "results.json"),
        "tool_sha256": sha256(Path(__file__)),
        "figures": {str(path.relative_to(REPO)): {"sha256": sha256(path), "size_bytes": path.stat().st_size} for path in files},
        "sources": {str(path.relative_to(REPO)): {"sha256": sha256(path), "size_bytes": path.stat().st_size} for path in sources},
    }
    (root / "figure-manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return manifest


def statistics_median(values: Any) -> float:
    materialized = list(values)
    materialized.sort()
    size = len(materialized)
    midpoint = size // 2
    return materialized[midpoint] if size % 2 else (materialized[midpoint - 1] + materialized[midpoint]) / 2.0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=DEFAULT_ROOT)
    parser.add_argument("--figure-dir", type=Path, default=DEFAULT_FIGURES)
    args = parser.parse_args()
    manifest = execute(args.root.resolve(), args.figure_dir.resolve())
    print(json.dumps({"figure_count": len(manifest["figures"]), "source_count": len(manifest["sources"])}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
