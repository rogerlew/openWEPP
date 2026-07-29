#!/usr/bin/env python3
"""Render deterministic CAL-05 interpretation figures using only stdlib."""

from __future__ import annotations

import csv
import math
from collections import defaultdict
from html import escape
from pathlib import Path


PACKAGE = Path(__file__).resolve().parent.parent
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
WIDTH = 1200
HEIGHT = 760

COLORS = {
    "ink": "#17202a",
    "muted": "#566573",
    "grid": "#d5d8dc",
    "blue": "#2166ac",
    "orange": "#d95f02",
    "purple": "#7b3294",
    "green": "#1b9e77",
    "background": "#ffffff",
    "panel": "#f8f9f9",
}


def rows(name: str) -> list[dict[str, str]]:
    with (ARTIFACTS / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def line(x1: float, y1: float, x2: float, y2: float, **attrs: object) -> str:
    values = {"x1": x1, "y1": y1, "x2": x2, "y2": y2, **attrs}
    return element("line", values)


def text_node(x: float, y: float, value: str, **attrs: object) -> str:
    values = {"x": x, "y": y, **attrs}
    return element("text", values, escape(value))


def element(tag: str, attrs: dict[str, object], content: str = "") -> str:
    def attribute_name(key: str) -> str:
        return key.removesuffix("_").replace("_", "-")

    rendered = " ".join(
        f'{attribute_name(key)}="{escape(str(value))}"'
        for key, value in attrs.items()
    )
    if content:
        return f"<{tag} {rendered}>{content}</{tag}>"
    return f"<{tag} {rendered}/>"


def svg_document(title: str, description: str, body: list[str]) -> str:
    style = """
      text { font-family: Inter, "DejaVu Sans", Arial, sans-serif; fill: #17202a; }
      .title { font-size: 27px; font-weight: 700; }
      .subtitle { font-size: 16px; fill: #566573; }
      .axis { font-size: 15px; font-weight: 600; }
      .tick { font-size: 13px; fill: #34495e; }
      .note { font-size: 14px; fill: #566573; }
      .value { font-size: 15px; font-weight: 700; }
    """
    return "\n".join(
        [
            (
                f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" '
                f'height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}" '
                'role="img" aria-labelledby="title desc">'
            ),
            f"<title id=\"title\">{escape(title)}</title>",
            f"<desc id=\"desc\">{escape(description)}</desc>",
            f"<style>{style}</style>",
            element(
                "rect",
                {
                    "x": 0,
                    "y": 0,
                    "width": WIDTH,
                    "height": HEIGHT,
                    "fill": COLORS["background"],
                },
            ),
            *body,
            "</svg>",
            "",
        ]
    )


def write_svg(name: str, title: str, description: str, body: list[str]) -> None:
    FIGURES.mkdir(parents=True, exist_ok=True)
    (FIGURES / name).write_text(
        svg_document(title, description, body), encoding="utf-8", newline="\n"
    )


def rgb(hex_color: str) -> tuple[int, int, int]:
    value = hex_color.lstrip("#")
    return tuple(int(value[index : index + 2], 16) for index in (0, 2, 4))


def blend(low: str, high: str, fraction: float) -> str:
    low_rgb = rgb(low)
    high_rgb = rgb(high)
    values = [
        round(start + (end - start) * max(0.0, min(1.0, fraction)))
        for start, end in zip(low_rgb, high_rgb, strict=True)
    ]
    return "#" + "".join(f"{value:02x}" for value in values)


def terminal_stock_heatmap() -> None:
    design = rows("deterministic-design.csv")
    terminal = {
        row["candidate_id"]: float(row["terminal_stock_kg_m2"])
        for row in rows("reconstruction-results.csv")
    }
    if len(design) != 16 or len(terminal) != 16:
        raise ValueError("expected the frozen 16-member source/rate grid")
    sources = sorted(
        {float(row["synthetic_annual_surface_litter_input_kg_m2_yr"]) for row in design}
    )
    rates = sorted({float(row["surface_rate_d-1"]) * 365.25 for row in design})
    values = list(terminal.values())
    low_log = math.log10(min(values))
    high_log = math.log10(max(values))

    body = [
        text_node(70, 55, "CAL-05 — terminal stock responds to source and decay", class_="title"),
        text_node(
            70,
            84,
            "Twenty-year synthetic terminal surface residue; values are kg m⁻².",
            class_="subtitle",
        ),
    ]
    left, top = 235, 165
    cell_w, cell_h = 190, 112

    for column, rate in enumerate(rates):
        x = left + column * cell_w + cell_w / 2
        body.append(text_node(x, top - 22, f"{rate:.2g}", class_="tick", text_anchor="middle"))
    body.append(
        text_node(
            left + 2 * cell_w,
            top - 58,
            "Nominal decay rate (yr⁻¹)",
            class_="axis",
            text_anchor="middle",
        )
    )

    for row_index, source in enumerate(reversed(sources)):
        y = top + row_index * cell_h
        body.append(
            text_node(
                left - 22,
                y + cell_h / 2 + 5,
                f"{source:.2f}",
                class_="tick",
                text_anchor="end",
            )
        )
        for column, rate in enumerate(rates):
            source_id = f"S{round(source * 100):03d}"
            rate_id = f"K{round(rate * 100):03d}"
            value = terminal[f"{source_id}-{rate_id}"]
            fraction = (math.log10(value) - low_log) / (high_log - low_log)
            fill = blend("#eff3ff", "#08519c", fraction)
            foreground = "#ffffff" if fraction > 0.55 else COLORS["ink"]
            x = left + column * cell_w
            body.extend(
                [
                    element(
                        "rect",
                        {
                            "x": x,
                            "y": y,
                            "width": cell_w - 4,
                            "height": cell_h - 4,
                            "rx": 5,
                            "fill": fill,
                            "stroke": "#ffffff",
                            "stroke_width": 2,
                        },
                    ),
                    text_node(
                        x + (cell_w - 4) / 2,
                        y + cell_h / 2 + 6,
                        f"{value:.3f}",
                        class_="value",
                        text_anchor="middle",
                        fill=foreground,
                        style=f"fill:{foreground}",
                    ),
                ]
            )

    body.append(
        text_node(
            78,
            top + 2 * cell_h,
            "Annual synthetic",
            class_="axis",
            text_anchor="middle",
            transform=f"rotate(-90 78 {top + 2 * cell_h})",
        )
    )
    body.append(
        text_node(
            103,
            top + 2 * cell_h,
            "input (kg m⁻² yr⁻¹)",
            class_="axis",
            text_anchor="middle",
            transform=f"rotate(-90 103 {top + 2 * cell_h})",
        )
    )
    body.extend(
        [
            text_node(
                235,
                640,
                "Interpretation: more input raises stock; faster decay lowers it.",
                class_="axis",
            ),
            text_node(
                235,
                670,
                "A terminal stock does not reveal which combination produced it.",
                class_="note",
            ),
            text_node(
                235,
                705,
                "Synthetic execution design—not empirical calibration or a physiological bound.",
                class_="note",
            ),
        ]
    )
    write_svg(
        "cal05-terminal-stock-response.svg",
        "CAL-05 source and decay response surface",
        (
            "A four by four heatmap showing twenty-year terminal surface residue "
            "for synthetic annual litter inputs from 0.10 to 0.40 kilograms per "
            "square metre per year and decay rates from zero to two per year."
        ),
        body,
    )


def daily_recovery() -> None:
    recovery = rows("synthetic-recovery.csv")
    recovery.sort(key=lambda row: row["candidate_id"])
    truths = [row for row in recovery if row["state"] == "RECOVERED_TRUTH"]
    if (
        len(recovery) != 16
        or len(truths) != 1
        or truths[0]["candidate_id"] != "S020-K050"
        or float(truths[0]["daily_stock_sse"]) != 0.0
    ):
        raise ValueError("daily recovery no longer has the sole frozen truth")
    nonzero = [float(row["daily_stock_sse"]) for row in recovery if float(row["daily_stock_sse"]) > 0]
    maximum = math.ceil(max(math.log10(value) for value in nonzero))

    body = [
        text_node(70, 55, "CAL-05 — the complete daily trace recovers the frozen truth", class_="title"),
        text_node(
            70,
            84,
            "Daily-stock sum of squared errors (SSE); logarithmic scale for nonzero candidates.",
            class_="subtitle",
        ),
    ]
    left, top, chart_h = 105, 145, 450
    chart_w = 1015
    for tick in range(maximum + 1):
        y = top + chart_h - tick / maximum * chart_h
        body.extend(
            [
                line(left, y, left + chart_w, y, stroke=COLORS["grid"], stroke_width=1),
                text_node(left - 14, y + 5, f"10^{tick}", class_="tick", text_anchor="end"),
            ]
        )

    gap = chart_w / len(recovery)
    bar_w = gap * 0.66
    for index, row in enumerate(recovery):
        x = left + index * gap + (gap - bar_w) / 2
        value = float(row["daily_stock_sse"])
        is_truth = row["state"] == "RECOVERED_TRUTH"
        if is_truth:
            y = top + chart_h - 5
            body.extend(
                [
                    element(
                        "circle",
                        {
                            "cx": x + bar_w / 2,
                            "cy": y,
                            "r": 7,
                            "fill": COLORS["orange"],
                        },
                    ),
                    text_node(
                        x + bar_w / 2,
                        y - 18,
                        "exact 0",
                        class_="value",
                        text_anchor="middle",
                        style=f"fill:{COLORS['orange']}",
                    ),
                ]
            )
        else:
            log_value = math.log10(value)
            height = log_value / maximum * chart_h
            y = top + chart_h - height
            body.append(
                element(
                    "rect",
                    {
                        "x": x,
                        "y": y,
                        "width": bar_w,
                        "height": height,
                        "rx": 3,
                        "fill": COLORS["blue"],
                    },
                )
            )
        body.append(
            text_node(
                x + bar_w / 2,
                top + chart_h + 22,
                row["candidate_id"].replace("-", " "),
                class_="tick",
                text_anchor="end",
                transform=(
                    f"rotate(-55 {x + bar_w / 2:.2f} "
                    f"{top + chart_h + 22:.2f})"
                ),
            )
        )

    body.extend(
        [
            text_node(
                34,
                top + chart_h / 2,
                "Daily-stock SSE",
                class_="axis",
                text_anchor="middle",
                transform=f"rotate(-90 34 {top + chart_h / 2})",
            ),
            text_node(
                105,
                700,
                "Only S020 K050 matches every daily stock exactly; all other grid members diverge.",
                class_="axis",
            ),
            text_node(
                105,
                729,
                "This proves recoverability with complete daily information, not with one endpoint.",
                class_="note",
            ),
        ]
    )
    write_svg(
        "cal05-daily-recovery.svg",
        "CAL-05 complete daily trace recovery",
        (
            "Bar chart of daily-stock sum of squared errors for sixteen source "
            "and decay candidates. S020 K050 has exact zero error and every "
            "other candidate has positive error."
        ),
        body,
    )


def terminal_ridge() -> None:
    design = rows("terminal-stock-ridge-design.csv")
    design.sort(key=lambda row: float(row["yearly_rate_yr-1"]))
    if len(design) != 5:
        raise ValueError("expected the frozen five-member terminal ridge")
    annual_end: dict[str, list[tuple[int, float]]] = defaultdict(list)
    for row in rows("ridge-producer-results.csv"):
        if row["day"] == "365":
            annual_end[row["candidate_id"]].append(
                (int(row["year"]), float(row["surface_after_kg_m2"]))
            )
    for row in design:
        series = annual_end[row["ridge_id"]]
        target = float(row["target_terminal_stock_kg_m2"])
        if len(series) != 20 or abs(series[-1][1] - target) > 1.2e-15:
            raise ValueError(f"ridge endpoint changed for {row['ridge_id']}")

    body = [
        text_node(70, 55, "CAL-05 — one endpoint hides a source–decay ridge", class_="title"),
        text_node(
            70,
            84,
            "Five synthetic source/rate pairs share the same year-20 stock but follow different paths.",
            class_="subtitle",
        ),
    ]
    panel_y, panel_h = 145, 455
    left_x, panel_w = 95, 470
    right_x = 675
    body.extend(
        [
            element(
                "rect",
                {
                    "x": left_x,
                    "y": panel_y,
                    "width": panel_w,
                    "height": panel_h,
                    "rx": 6,
                    "fill": COLORS["panel"],
                },
            ),
            element(
                "rect",
                {
                    "x": right_x,
                    "y": panel_y,
                    "width": panel_w,
                    "height": panel_h,
                    "rx": 6,
                    "fill": COLORS["panel"],
                },
            ),
            text_node(left_x + panel_w / 2, 128, "Pairs on the equifinality ridge", class_="axis", text_anchor="middle"),
            text_node(right_x + panel_w / 2, 128, "Annual-end stock trajectories", class_="axis", text_anchor="middle"),
        ]
    )

    plot_left, plot_top = left_x + 70, panel_y + 35
    plot_w, plot_h = 350, 340
    max_rate, max_source = 2.0, 0.7
    for tick in range(5):
        rate = tick * 0.5
        x = plot_left + rate / max_rate * plot_w
        body.extend(
            [
                line(x, plot_top, x, plot_top + plot_h, stroke=COLORS["grid"]),
                text_node(x, plot_top + plot_h + 24, f"{rate:.1f}", class_="tick", text_anchor="middle"),
            ]
        )
    for tick in range(8):
        source = tick * 0.1
        y = plot_top + plot_h - source / max_source * plot_h
        body.extend(
            [
                line(plot_left, y, plot_left + plot_w, y, stroke=COLORS["grid"]),
                text_node(plot_left - 12, y + 5, f"{source:.1f}", class_="tick", text_anchor="end"),
            ]
        )
    points = []
    for row in design:
        x = plot_left + float(row["yearly_rate_yr-1"]) / max_rate * plot_w
        y = plot_top + plot_h - float(
            row["synthetic_annual_surface_litter_input_kg_m2_yr"]
        ) / max_source * plot_h
        points.append(f"{x:.2f},{y:.2f}")
    body.append(
        element(
            "polyline",
            {
                "points": " ".join(points),
                "fill": "none",
                "stroke": COLORS["purple"],
                "stroke_width": 4,
            },
        )
    )
    for row, point in zip(design, points, strict=True):
        x, y = (float(value) for value in point.split(","))
        body.extend(
            [
                element(
                    "circle",
                    {
                        "cx": x,
                        "cy": y,
                        "r": 7,
                        "fill": COLORS["purple"],
                        "stroke": "#ffffff",
                        "stroke_width": 2,
                    },
                ),
                text_node(x + 10, y - 10, row["ridge_id"].replace("RIDGE-", ""), class_="tick"),
            ]
        )
    body.extend(
        [
            text_node(plot_left + plot_w / 2, panel_y + 425, "Decay rate (yr⁻¹)", class_="axis", text_anchor="middle"),
            text_node(
                left_x + 20,
                plot_top + plot_h / 2,
                "Annual input (kg m⁻² yr⁻¹)",
                class_="axis",
                text_anchor="middle",
                transform=f"rotate(-90 {left_x + 20} {plot_top + plot_h / 2})",
            ),
        ]
    )

    trajectory_left, trajectory_top = right_x + 65, panel_y + 35
    trajectory_w, trajectory_h = 365, 340
    for year in (0, 5, 10, 15, 20):
        x = trajectory_left + year / 20 * trajectory_w
        body.extend(
            [
                line(x, trajectory_top, x, trajectory_top + trajectory_h, stroke=COLORS["grid"]),
                text_node(x, trajectory_top + trajectory_h + 24, str(year), class_="tick", text_anchor="middle"),
            ]
        )
    for stock_index in range(6):
        stock = stock_index * 0.2
        y = trajectory_top + trajectory_h - stock / 1.0 * trajectory_h
        body.extend(
            [
                line(trajectory_left, y, trajectory_left + trajectory_w, y, stroke=COLORS["grid"]),
                text_node(trajectory_left - 12, y + 5, f"{stock:.1f}", class_="tick", text_anchor="end"),
            ]
        )
    palette = ["#1b9e77", "#d95f02", "#7570b3", "#e7298a", "#1f78b4"]
    for color, row in zip(palette, design, strict=True):
        series = annual_end[row["ridge_id"]]
        points = [
            (
                trajectory_left + year / 20 * trajectory_w,
                trajectory_top + trajectory_h - stock / 1.0 * trajectory_h,
            )
            for year, stock in series
        ]
        body.append(
            element(
                "polyline",
                {
                    "points": " ".join(f"{x:.2f},{y:.2f}" for x, y in points),
                    "fill": "none",
                    "stroke": color,
                    "stroke_width": 3,
                },
            )
        )
        end_x, end_y = points[-1]
        body.append(
            element(
                "circle",
                {"cx": end_x, "cy": end_y, "r": 5, "fill": color},
            )
        )
        body.append(
            text_node(
                trajectory_left + 8,
                trajectory_top + 22 + 24 * palette.index(color),
                row["ridge_id"].replace("RIDGE-", ""),
                class_="tick",
                style=f"fill:{color};font-weight:700",
            )
        )
    body.extend(
        [
            text_node(
                trajectory_left + trajectory_w / 2,
                panel_y + 425,
                "Simulation year",
                class_="axis",
                text_anchor="middle",
            ),
            text_node(
                right_x + 20,
                trajectory_top + trajectory_h / 2,
                "Surface stock (kg m⁻²)",
                class_="axis",
                text_anchor="middle",
                transform=(
                    f"rotate(-90 {right_x + 20} "
                    f"{trajectory_top + trajectory_h / 2})"
                ),
            ),
            text_node(
                95,
                650,
                "All five endpoints equal 0.852271 kg m⁻² within 1.12×10⁻¹⁵.",
                class_="axis",
            ),
            text_node(
                95,
                680,
                "Their different trajectories show why a terminal stock cannot identify source and decay.",
                class_="note",
            ),
            text_node(
                95,
                715,
                "The ridge is an analytic synthetic construction—not an empirical fit.",
                class_="note",
            ),
        ]
    )
    write_svg(
        "cal05-source-decay-ridge.svg",
        "CAL-05 source and decay equifinality ridge",
        (
            "Two-panel figure. The left panel plots five increasing annual litter "
            "input and decay-rate pairs. The right panel shows their distinct "
            "annual stock trajectories converging to the same year-twenty stock."
        ),
        body,
    )


def main() -> None:
    terminal_stock_heatmap()
    daily_recovery()
    terminal_ridge()
    print("rendered 3 CAL-05 SVG figures")


if __name__ == "__main__":
    main()
