#!/usr/bin/env python3
"""Render CAL-07F human-interpretation figures."""

from __future__ import annotations

import csv
from collections import defaultdict
from datetime import date
from pathlib import Path
import xml.etree.ElementTree as ET

import matplotlib.dates as mdates
import matplotlib.pyplot as plt


PKG = Path(__file__).resolve().parents[1]
ART = PKG / "artifacts"
FIG = ART / "figures"
COLORS = {"gcc_mean": "#0072B2", "gcc_90": "#D55E00"}


def rows(name: str) -> list[dict[str, str]]:
    with (ART / name).open(encoding="utf-8", newline="") as stream:
        return list(csv.DictReader(stream))


def accessible_svg(path: Path, title: str, description: str) -> None:
    namespace = "http://www.w3.org/2000/svg"
    ET.register_namespace("", namespace)
    tree = ET.parse(path)
    root = tree.getroot()
    title_node = ET.Element(f"{{{namespace}}}title", {"id": "title"})
    title_node.text = title
    desc_node = ET.Element(f"{{{namespace}}}desc", {"id": "desc"})
    desc_node.text = description
    root.insert(0, desc_node)
    root.insert(0, title_node)
    root.set("role", "img")
    root.set("aria-labelledby", "title desc")
    tree.write(path, encoding="utf-8", xml_declaration=True)
    cleaned = "\n".join(
        line.rstrip() for line in path.read_text(encoding="utf-8").splitlines()
    )
    path.write_text(f"{cleaned}\n", encoding="utf-8")


def product_curves() -> None:
    curves = rows("daily-product-curves.csv")
    transitions = rows("source-transition-audit.csv")
    figure, axes = plt.subplots(2, 1, figsize=(12, 7.2), sharey=True)
    for axis, year in zip(axes, (2024, 2025)):
        selected = [row for row in curves if int(row["year"]) == year]
        days = [date.fromisoformat(row["date"]) for row in selected]
        for product, label in (
            ("gcc_mean", "GCC mean"),
            ("gcc_90", "GCC 90th percentile"),
        ):
            values = [float(row[f"smooth_{product}"]) for row in selected]
            axis.plot(
                days,
                values,
                color=COLORS[product],
                linewidth=2.0,
                label=label,
            )
            source_rows = [
                row
                for row in transitions
                if row["product"] == product
                and int(row["year"]) == year
                and row["source_level"] == "0.50"
            ]
            for source in source_rows:
                marker = "v" if source["direction"] == "falling" else "^"
                axis.scatter(
                    [date.fromisoformat(source["observed_date"])],
                    [float(source["gcc_threshold"])],
                    marker=marker,
                    s=70,
                    color=COLORS[product],
                    edgecolor="#202020",
                    linewidth=0.6,
                    zorder=4,
                )
        axis.set_title(str(year), loc="left", fontweight="bold")
        axis.set_ylabel("Smoothed green chromatic coordinate")
        axis.grid(axis="y", color="#d5d8dc", linewidth=0.8)
        axis.xaxis.set_major_locator(mdates.MonthLocator(interval=2))
        axis.xaxis.set_major_formatter(mdates.DateFormatter("%b"))
        axis.spines[["top", "right"]].set_visible(False)
    axes[0].legend(frameon=False, loc="lower left", ncols=2)
    axes[-1].set_xlabel("Month")
    figure.tight_layout()
    path = FIG / "cal07f-product-curves.svg"
    figure.savefig(path, format="svg", bbox_inches="tight")
    plt.close(figure)
    accessible_svg(
        path,
        "Bezà daily GCC products and their seasonal midpoint transitions",
        "Two panels show 2024 and 2025 smoothed GCC mean and GCC 90th "
        "percentile curves. Downward triangles mark falling T50 transitions "
        "and upward triangles mark rising T50 transitions.",
    )


def residual_distributions() -> None:
    comparisons = rows("member-comparisons.csv")
    events = [
        (year, direction, level)
        for year in (2024, 2025)
        for direction in ("falling", "rising")
        for level in ("0.10", "0.25", "0.50")
    ]
    figure, axes = plt.subplots(2, 1, figsize=(13, 8), sharex=True, sharey=True)
    for axis, product in zip(axes, ("gcc_mean", "gcc_90")):
        data: list[list[float]] = []
        counts: list[int] = []
        for year, direction, level in events:
            selected = [
                float(row["residual_days"])
                for row in comparisons
                if row["product"] == product
                and int(row["year"]) == year
                and row["direction"] == direction
                and row["source_level"] == level
                and row["residual_days"]
            ]
            data.append(selected if selected else [float("nan")])
            counts.append(len(selected))
        plot = axis.boxplot(
            data,
            patch_artist=True,
            widths=0.65,
            showfliers=False,
            medianprops={"color": "#111111", "linewidth": 1.5},
            whiskerprops={"color": "#555555"},
            capprops={"color": "#555555"},
        )
        for box in plot["boxes"]:
            box.set_facecolor(COLORS[product])
            box.set_alpha(0.65)
        axis.axhspan(-21, 21, color="#009E73", alpha=0.12)
        axis.axhline(0, color="#303030", linewidth=1)
        axis.set_ylim(-130, 140)
        axis.set_ylabel("Model minus observed date (days)")
        axis.set_title(
            "GCC mean" if product == "gcc_mean" else "GCC 90th percentile",
            loc="left",
            fontweight="bold",
        )
        axis.grid(axis="y", color="#d5d8dc", linewidth=0.8)
        axis.spines[["top", "right"]].set_visible(False)
        for position, count in enumerate(counts, 1):
            if count < 37:
                axis.text(
                    position,
                    132,
                    f"{count}/37",
                    ha="center",
                    va="top",
                    fontsize=8,
                    color="#4d5656",
                )
    labels = [
        f"{str(year)[2:]} {'Fall' if direction == 'falling' else 'Rise'} "
        f"T{int(float(level) * 100)}"
        for year, direction, level in events
    ]
    axes[-1].set_xticks(range(1, len(events) + 1), labels, rotation=45, ha="right")
    axes[-1].set_xlabel("Source transition")
    figure.tight_layout()
    path = FIG / "cal07f-residual-distributions.svg"
    figure.savefig(path, format="svg", bbox_inches="tight")
    plt.close(figure)
    accessible_svg(
        path,
        "Model timing residuals under separate GCC mean and GCC 90 operators",
        "Box plots show residual distributions for 37 frozen ensemble members "
        "at twelve source transitions per product. The green band marks plus "
        "or minus 21 days. Text above incomplete cells reports available "
        "crossings out of 37.",
    )


def calibration_screen() -> None:
    ranks = rows("product-rank-comparison.csv")
    groups: dict[tuple[float, float], list[str]] = defaultdict(list)
    for row in ranks:
        groups[
            (
                float(row["gcc_mean_score_days"]),
                float(row["gcc_90_score_days"]),
            )
        ].append(row["member"])
    x = [point[0] for point in groups]
    y = [point[1] for point in groups]
    counts = [len(groups[point]) for point in groups]
    figure, axis = plt.subplots(figsize=(8.5, 7.2))
    axis.scatter(
        x,
        y,
        s=[45 + count * 22 for count in counts],
        color="#6C757D",
        alpha=0.8,
    )
    for x_value, y_value, count in zip(x, y, counts):
        if count > 1:
            axis.text(
                x_value,
                y_value,
                str(count),
                ha="center",
                va="center",
                color="#ffffff",
                fontsize=8,
                fontweight="bold",
            )
    best = min(range(len(x)), key=lambda index: x[index] + y[index])
    axis.scatter(
        [x[best]],
        [y[best]],
        s=110,
        color="#CC3311",
        edgecolor="#202020",
        linewidth=0.8,
        zorder=3,
    )
    axis.annotate(
        groups[(x[best], y[best])][0],
        (x[best], y[best]),
        xytext=(8, 8),
        textcoords="offset points",
        fontsize=10,
    )
    lower = min(x + y) - 5
    upper = max(x + y) + 5
    axis.plot([lower, upper], [lower, upper], color="#303030", linestyle="--")
    axis.axvline(21, color="#009E73", linewidth=1.5)
    axis.axhline(21, color="#009E73", linewidth=1.5)
    axis.set_xlim(min(0, lower), upper)
    axis.set_ylim(min(0, lower), upper)
    axis.set_xlabel("GCC mean penalized mean absolute residual (days)")
    axis.set_ylabel("GCC 90 penalized mean absolute residual (days)")
    axis.set_title("No frozen member approaches the 21-day calibration screen")
    axis.grid(color="#d5d8dc", linewidth=0.8)
    axis.spines[["top", "right"]].set_visible(False)
    figure.tight_layout()
    path = FIG / "cal07f-calibration-screen.svg"
    figure.savefig(path, format="svg", bbox_inches="tight")
    plt.close(figure)
    accessible_svg(
        path,
        "Cross-product calibration screen for all 37 frozen members",
        "A scatter plot compares penalized timing error under GCC mean and "
        "GCC 90 products. Bubble size represents members with identical "
        "scores; numbers label groups larger than one. The highlighted best "
        "joint member remains far above the 21-day threshold on both axes.",
    )


def main() -> None:
    FIG.mkdir(parents=True, exist_ok=True)
    product_curves()
    residual_distributions()
    calibration_screen()
    print("CAL-07F plots complete: 3 SVG figures")


if __name__ == "__main__":
    main()
