#!/usr/bin/env python3
"""Render frozen adjudication figures and Markdown source sidecars."""

from __future__ import annotations

import csv
import hashlib
import json
import statistics
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any, Callable

sys.dont_write_bytecode = True

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
OUTPUT = REPO / "target/snow_prepeak_mass_transition_physics_adjudication_v2"
RESULTS = OUTPUT / "results/cross-fixture-results.json"
EVENTS = OUTPUT / "results/event-attribution.json"
FIGURES = PACKAGE / "artifacts/figures"
SOURCE_TABLES = OUTPUT / "figure-source-tables"
SITE_LABELS = {
    "snotel_mica_creek_st_joe_id": "Mica Creek",
    "snotel_niwot_co": "Niwot",
    "snotel_paradise_wa": "Paradise",
    "snotel_snowbird_ut": "Snowbird",
}
COLORS = {
    "blue": "#3B6FB6",
    "orange": "#E28E2C",
    "green": "#3A9D5D",
    "red": "#C94C4C",
    "purple": "#8064A2",
    "gray": "#72777D",
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        raise RuntimeError(f"empty figure source: {path}")
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def write_sidecar(
    stem: str,
    title: str,
    source: Path,
    units: str,
    description: str,
    claim_limit: str,
) -> None:
    figure = FIGURES / f"{stem}.png"
    sidecar = FIGURES / f"{stem}.md"
    body = f"""# {title}

Status: `generated / diagnostic only`

Evidence mode: `Ran`

- Figure: `{figure.name}`
- Figure SHA-256: `{sha256(figure)}`
- Source table: `{relative(source)}`
- Source-table SHA-256: `{sha256(source)}`
- Generator: `{relative(Path(__file__))}`
- Generator SHA-256: `{sha256(Path(__file__))}`
- Units: {units}

{description}

Claim limit: {claim_limit}
"""
    sidecar.write_text(body, encoding="utf-8")


def median_by_site(
    annual: list[dict[str, Any]], fields: tuple[str, ...]
) -> list[dict[str, Any]]:
    output = []
    for site in SITE_LABELS:
        rows = [
            row
            for row in annual
            if row["site"] == site and not row["right_censored"]
        ]
        result: dict[str, Any] = {"site": site, "site_label": SITE_LABELS[site]}
        for field in fields:
            values = [float(row[field]) for row in rows if row[field] is not None]
            result[field] = statistics.median(values) if values else ""
        output.append(result)
    return output


def grouped_bars(
    ax: Any,
    rows: list[dict[str, Any]],
    fields: list[tuple[str, str, str]],
    ylabel: str,
) -> None:
    x = list(range(len(rows)))
    width = 0.8 / len(fields)
    for index, (field, label, color) in enumerate(fields):
        offset = (index - (len(fields) - 1) / 2) * width
        ax.bar(
            [value + offset for value in x],
            [float(row[field]) for row in rows],
            width,
            label=label,
            color=color,
        )
    ax.set_xticks(x, [row["site_label"] for row in rows], rotation=15)
    ax.set_ylabel(ylabel)
    ax.grid(axis="y", alpha=0.25)
    ax.legend(frameon=False, fontsize=8)


def render_linked_boundaries(annual: list[dict[str, Any]]) -> None:
    fields = (
        "snowfall_m",
        "solid_pack_loss_m",
        "stage3_incoming_m",
        "stage3_routed_m",
        "stage3_retained_positive_m",
        "stage3_refrozen_m",
    )
    rows = median_by_site(annual, fields)
    for row in rows:
        site_annual = [
            annual_row
            for annual_row in annual
            if annual_row["site"] == row["site"]
            and not annual_row["right_censored"]
            and float(annual_row["stage3_incoming_m"]) > 0.0
        ]
        for field in (
            "stage3_routed_m",
            "stage3_retained_positive_m",
            "stage3_refrozen_m",
        ):
            row[f"{field}_fraction"] = statistics.median(
                float(annual_row[field]) / float(annual_row["stage3_incoming_m"])
                for annual_row in site_annual
            )
    source = SOURCE_TABLES / "linked-mass-boundaries.csv"
    write_csv(source, rows)
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.5))
    grouped_bars(
        axes[0],
        rows,
        [
            ("snowfall_m", "snowfall", COLORS["blue"]),
            ("solid_pack_loss_m", "solid-pack loss", COLORS["orange"]),
            ("stage3_incoming_m", "liquid handoff", COLORS["purple"]),
        ],
        "median accumulation-window total (m water)",
    )
    grouped_bars(
        axes[1],
        rows,
        [
            ("stage3_routed_m_fraction", "routed", COLORS["orange"]),
            (
                "stage3_retained_positive_m_fraction",
                "producer retained",
                COLORS["blue"],
            ),
            ("stage3_refrozen_m_fraction", "refrozen", COLORS["green"]),
        ],
        "median quantity / incoming liquid",
    )
    axes[0].set_title("Upstream storage boundary")
    axes[1].set_title("Stage-3 diagnostic disposition boundary")
    fig.suptitle("Two linked mass boundaries (not one additive partition)")
    fig.tight_layout()
    figure = FIGURES / "linked-mass-boundaries.png"
    fig.savefig(figure, dpi=180, bbox_inches="tight")
    plt.close(fig)
    write_sidecar(
        "linked-mass-boundaries",
        "Linked Mass Boundaries",
        source,
        "meters water and dimensionless fractions",
        "The left and right panels use different linked ledger boundaries. Right-panel "
        "fractions use Stage-3 incoming liquid as the per-water-year denominator before "
        "the site median. Component medians need not sum to the separately computed "
        "combined-capture median. They describe diagnostic disposition, not "
        "authoritative runoff.",
        "Do not add upstream and downstream bars or infer a causal peak-SWE response.",
    )


def render_signed_opportunity(annual: list[dict[str, Any]]) -> None:
    rows = [
        {
            "site": row["site"],
            "site_label": SITE_LABELS[row["site"]],
            "water_year": row["water_year"],
            "peak_deficit_m": row["observed_minus_modeled_peak_m"],
            "daily_local_signed_opportunity_m": row[
                "daily_local_signed_opportunity_m"
            ],
            "opportunity_to_deficit": row[
                "daily_signed_opportunity_to_positive_peak_deficit"
            ],
        }
        for row in annual
        if not row["right_censored"]
        and row["daily_signed_opportunity_to_positive_peak_deficit"] is not None
    ]
    source = SOURCE_TABLES / "signed-opportunity.csv"
    write_csv(source, rows)
    fig, ax = plt.subplots(figsize=(8, 4.8))
    for index, site in enumerate(SITE_LABELS):
        values = [
            float(row["opportunity_to_deficit"])
            for row in rows
            if row["site"] == site
        ]
        jitter = [index + ((position % 5) - 2) * 0.035 for position in range(len(values))]
        ax.scatter(jitter, values, alpha=0.62, s=22, label=SITE_LABELS[site])
        ax.scatter(
            [index],
            [statistics.median(values)],
            marker="D",
            s=60,
            color="black",
            zorder=4,
        )
    ax.axhline(0.25, color=COLORS["red"], linestyle="--", label="frozen 0.25 screen")
    ax.set_xticks(range(len(SITE_LABELS)), SITE_LABELS.values(), rotation=15)
    ax.set_ylabel("daily-local signed opportunity / positive peak deficit")
    ax.set_title("Feedback-free daily-local signed opportunity")
    ax.grid(axis="y", alpha=0.25)
    ax.legend(frameon=False, fontsize=8)
    fig.tight_layout()
    figure = FIGURES / "signed-opportunity-vs-peak-deficit.png"
    fig.savefig(figure, dpi=180, bbox_inches="tight")
    plt.close(fig)
    write_sidecar(
        "signed-opportunity-vs-peak-deficit",
        "Daily-Local Signed Opportunity Versus Peak Deficit",
        source,
        "dimensionless ratio",
        "Points are uncensored site-years and diamonds are per-site medians. The "
        "red line is the prospectively frozen ASSUMED_FOR_EXECUTION "
        "materiality threshold.",
        "This algebraic, no-feedback opportunity is not simulated SWE recovery and "
        "does not validate negative CoE melt as refreeze.",
    )


def render_loss_context(annual: list[dict[str, Any]]) -> None:
    fields = ("loss_on_dry_days_m", "loss_on_snowfall_days_m", "loss_on_rain_days_m")
    rows = median_by_site(annual, fields)
    source = SOURCE_TABLES / "loss-by-precipitation-context.csv"
    write_csv(source, rows)
    fig, ax = plt.subplots(figsize=(8.5, 4.8))
    grouped_bars(
        ax,
        rows,
        [
            ("loss_on_dry_days_m", "dry day", COLORS["gray"]),
            ("loss_on_snowfall_days_m", "snowfall day", COLORS["blue"]),
            ("loss_on_rain_days_m", "rain day", COLORS["orange"]),
        ],
        "median solid-pack loss (m SWE)",
    )
    ax.set_title("Accumulation-window pack loss by daily precipitation context")
    fig.tight_layout()
    figure = FIGURES / "pack-loss-by-precipitation-context.png"
    fig.savefig(figure, dpi=180, bbox_inches="tight")
    plt.close(fig)
    write_sidecar(
        "pack-loss-by-precipitation-context",
        "Pack Loss by Precipitation Context",
        source,
        "meters SWE",
        "Bars are per-site medians of annual daily-context sums; snow and rain "
        "categories may overlap on mixed-input days.",
        "Context association does not identify a physical heat-flux contribution.",
    )


def primary_years(annual: list[dict[str, Any]]) -> set[tuple[str, int]]:
    return {
        (row["site"], int(row["water_year"]))
        for row in annual
        if not row["right_censored"]
    }


def class_site_medians(
    annual: list[dict[str, Any]],
    class_rows: list[dict[str, Any]],
    dimension: str,
    categories: list[str],
    value: str,
) -> list[dict[str, Any]]:
    eligible = primary_years(annual)
    grouped: dict[tuple[str, str], list[float]] = defaultdict(list)
    for row in class_rows:
        key = (row["site"], int(row["water_year"]))
        if key in eligible and row["dimension"] == dimension:
            grouped[(row["site"], row["category"])].append(float(row[value]))
    output = []
    for site in SITE_LABELS:
        row: dict[str, Any] = {"site": site, "site_label": SITE_LABELS[site]}
        for category in categories:
            values = grouped[(site, category)]
            row[category] = statistics.median(values)
        output.append(row)
    return output


def render_temperature_classes(
    annual: list[dict[str, Any]], class_rows: list[dict[str, Any]]
) -> None:
    categories = ["le_0_c", "gt_0_le_2_c", "gt_2_c"]
    positive = class_site_medians(
        annual, class_rows, "temperature", categories, "gross_positive_applied_m"
    )
    negative = class_site_medians(
        annual, class_rows, "temperature", categories, "negative_applied_m"
    )
    rows = []
    for positive_row, negative_row in zip(positive, negative, strict=True):
        rows.append(
            {
                "site": positive_row["site"],
                "site_label": positive_row["site_label"],
                **{f"positive_{key}": positive_row[key] for key in categories},
                **{f"negative_{key}": negative_row[key] for key in categories},
            }
        )
    source = SOURCE_TABLES / "hourly-coe-by-temperature.csv"
    write_csv(source, rows)
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.5))
    positive_fields = [
        ("positive_le_0_c", "<= 0 C", COLORS["blue"]),
        ("positive_gt_0_le_2_c", "0 to 2 C", COLORS["orange"]),
        ("positive_gt_2_c", "> 2 C", COLORS["red"]),
    ]
    negative_fields = [
        ("negative_le_0_c", "<= 0 C", COLORS["blue"]),
        ("negative_gt_0_le_2_c", "0 to 2 C", COLORS["orange"]),
        ("negative_gt_2_c", "> 2 C", COLORS["red"]),
    ]
    grouped_bars(axes[0], rows, positive_fields, "median gross positive applied CoE (m)")
    grouped_bars(axes[1], rows, negative_fields, "median negative applied CoE (m)")
    axes[0].set_title("Positive applied hours")
    axes[1].set_title("Negative diagnostic hours")
    fig.suptitle("Hourly CoE depth by air-temperature class")
    fig.tight_layout()
    figure = FIGURES / "hourly-coe-by-temperature.png"
    fig.savefig(figure, dpi=180, bbox_inches="tight")
    plt.close(fig)
    write_sidecar(
        "hourly-coe-by-temperature",
        "Hourly CoE Depth by Temperature Class",
        source,
        "meters empirical melt depth",
        "Bars are site medians of annual hourly sums under frozen temperature classes.",
        "Negative applied CoE is diagnostic and is not refreeze or cold-energy storage.",
    )


def render_component_depths(
    annual: list[dict[str, Any]], class_rows: list[dict[str, Any]]
) -> None:
    eligible = primary_years(annual)
    by_year: dict[tuple[str, int], dict[str, float]] = defaultdict(
        lambda: {key: 0.0 for key in ("a", "b", "c", "d")}
    )
    mapping = {
        "a": "signed_coe_melt_amelt_m",
        "b": "signed_coe_melt_bmelt_m",
        "c": "signed_coe_melt_cmelt_m",
        "d": "signed_coe_melt_dmelt_m",
    }
    for row in class_rows:
        key = (row["site"], int(row["water_year"]))
        if key not in eligible or row["dimension"] != "temperature":
            continue
        for short, field in mapping.items():
            by_year[key][short] += float(row[field])
    rows = []
    for site in SITE_LABELS:
        site_values = [values for (key_site, _), values in by_year.items() if key_site == site]
        rows.append(
            {
                "site": site,
                "site_label": SITE_LABELS[site],
                **{
                    f"component_{short}_m": statistics.median(
                        values[short] for values in site_values
                    )
                    for short in ("a", "b", "c", "d")
                },
            }
        )
    source = SOURCE_TABLES / "coe-component-depths.csv"
    write_csv(source, rows)
    fig, ax = plt.subplots(figsize=(8.5, 4.8))
    grouped_bars(
        ax,
        rows,
        [
            ("component_a_m", "A", COLORS["blue"]),
            ("component_b_m", "B", COLORS["orange"]),
            ("component_c_m", "C", COLORS["green"]),
            ("component_d_m", "D", COLORS["purple"]),
        ],
        "median signed empirical contribution (m)",
    )
    ax.axhline(0.0, color="black", linewidth=0.8)
    ax.set_title("Signed CoE component-depth diagnostics")
    fig.tight_layout()
    figure = FIGURES / "coe-component-depths.png"
    fig.savefig(figure, dpi=180, bbox_inches="tight")
    plt.close(fig)
    write_sidecar(
        "coe-component-depths",
        "Signed CoE Component-Depth Diagnostics",
        source,
        "meters empirical melt-depth contribution",
        "Each bar is the median annual signed total of a named empirical term. "
        "Cross-term cancellation and the applied cap occur after component formation.",
        "A/B/C/D are not independently identifiable energy or heat-flux shares.",
    )


def render_stage3_store_semantics(annual: list[dict[str, Any]]) -> None:
    fields = (
        "stage3_retained_positive_m",
        "layer_liquid_store_endpoint_delta_m",
        "sum_layer_liquid_store_day_delta_m",
        "producer_retained_minus_layer_day_delta_m",
    )
    rows = median_by_site(annual, fields)
    source = SOURCE_TABLES / "stage3-store-semantics.csv"
    write_csv(source, rows)
    fig, ax = plt.subplots(figsize=(9, 4.8))
    grouped_bars(
        ax,
        rows,
        [
            ("stage3_retained_positive_m", "producer newly retained", COLORS["blue"]),
            (
                "layer_liquid_store_endpoint_delta_m",
                "endpoint layer-store delta",
                COLORS["green"],
            ),
            (
                "sum_layer_liquid_store_day_delta_m",
                "sum of daily layer deltas",
                COLORS["purple"],
            ),
        ],
        "median accumulation-window amount (m water)",
    )
    ax.axhline(0.0, color="black", linewidth=0.8)
    ax.set_title("Producer retention is not full layer-store persistence")
    fig.tight_layout()
    figure = FIGURES / "stage3-store-semantics.png"
    fig.savefig(figure, dpi=180, bbox_inches="tight")
    plt.close(fig)
    write_sidecar(
        "stage3-store-semantics",
        "Stage-3 Store Semantics",
        source,
        "meters water",
        "The producer ledger records newly retained incoming liquid. Layer-array "
        "before/after values reveal trimming and later store evolution outside that operand.",
        "None of these Stage-3 diagnostic stores controls authoritative runtime SWE or "
        "the current hydrologic liquid handoff.",
    )


def main() -> int:
    result = json.loads(RESULTS.read_text(encoding="utf-8"))
    event_result = json.loads(EVENTS.read_text(encoding="utf-8"))
    annual = result["annual"]
    class_rows = event_result["hour_class_summaries"]
    FIGURES.mkdir(parents=True, exist_ok=True)
    SOURCE_TABLES.mkdir(parents=True, exist_ok=True)
    renderers: list[Callable[[], None]] = [
        lambda: render_linked_boundaries(annual),
        lambda: render_signed_opportunity(annual),
        lambda: render_loss_context(annual),
        lambda: render_temperature_classes(annual, class_rows),
        lambda: render_component_depths(annual, class_rows),
        lambda: render_stage3_store_semantics(annual),
    ]
    for renderer in renderers:
        renderer()
    print(f"rendered {len(renderers)} figures under {relative(FIGURES)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
