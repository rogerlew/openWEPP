#!/usr/bin/env python3
"""Create accessible CAL-07D SVG figures and Markdown sidecars."""

from __future__ import annotations

import csv
import hashlib
import html
import json
import statistics
from collections import Counter, defaultdict
from datetime import date
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ART = PKG / "artifacts"
FIG = ART / "figures"

COLORS = {
    "navy": "#245B8A",
    "orange": "#D87819",
    "green": "#25845B",
    "purple": "#7A5195",
    "red": "#B6463A",
    "gray": "#65727E",
    "light": "#E8EDF1",
    "ink": "#17212B",
}


def rows(name: str) -> list[dict[str, str]]:
    with (ART / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def svg_start(width: int, height: int, title: str, description: str) -> list[str]:
    return [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" role="img" aria-labelledby="title desc">',
        f"<title id=\"title\">{html.escape(title)}</title>",
        f"<desc id=\"desc\">{html.escape(description)}</desc>",
        """<style>
        text{font-family:Arial,sans-serif;fill:#17212B}
        .title{font-size:22px;font-weight:700}
        .label{font-size:14px;font-weight:700}
        .small{font-size:11px}
        .axis{stroke:#65727E;stroke-width:1}
        .grid{stroke:#D8E0E6;stroke-width:1}
        .panel{fill:#FBFCFD;stroke:#B9C4CC;stroke-width:1}
        </style>""",
        f'<rect width="{width}" height="{height}" fill="#FFFFFF"/>',
    ]


def bind_plot_data(
    parts: list[str],
    sources: tuple[str, ...],
    records: list[dict[str, str]],
    fields: tuple[str, ...],
) -> None:
    payload = {
        "sources": {
            source: {
                "rows": len(rows(source)),
                "sha256": sha(ART / source),
            }
            for source in sources
        },
        "plotted_record_count": len(records),
        "plotted_fields": list(fields),
        "plotted_data_sha256": hashlib.sha256(
            json.dumps(
                [[record[field] for field in fields] for record in records],
                separators=(",", ":"),
                ensure_ascii=True,
            ).encode("utf-8")
        ).hexdigest(),
    }
    parts.append(
        '<metadata id="cal07d-data-binding">'
        + html.escape(
            json.dumps(payload, sort_keys=True, separators=(",", ":"))
        )
        + "</metadata>"
    )


def line(parts: list[str], x1: float, y1: float, x2: float, y2: float, css: str = "axis") -> None:
    parts.append(
        f'<line x1="{x1:.2f}" y1="{y1:.2f}" x2="{x2:.2f}" y2="{y2:.2f}" class="{css}"/>'
    )


def text_at(
    parts: list[str],
    x: float,
    y: float,
    value: object,
    css: str = "small",
    anchor: str = "start",
) -> None:
    parts.append(
        f'<text x="{x:.2f}" y="{y:.2f}" class="{css}" text-anchor="{anchor}">{html.escape(str(value))}</text>'
    )


def circle(parts: list[str], x: float, y: float, color: str, radius: float = 3.0) -> None:
    parts.append(
        f'<circle cx="{x:.2f}" cy="{y:.2f}" r="{radius:.2f}" fill="{color}" stroke="#fff" stroke-width=".6"/>'
    )


def square(parts: list[str], x: float, y: float, color: str, size: float = 5.2) -> None:
    parts.append(
        f'<rect x="{x-size/2:.2f}" y="{y-size/2:.2f}" width="{size:.2f}" height="{size:.2f}" fill="{color}" stroke="#fff" stroke-width=".6"/>'
    )


def diamond(parts: list[str], x: float, y: float, color: str, size: float = 3.8) -> None:
    points = f"{x:.2f},{y-size:.2f} {x+size:.2f},{y:.2f} {x:.2f},{y+size:.2f} {x-size:.2f},{y:.2f}"
    parts.append(f'<polygon points="{points}" fill="{color}" stroke="#fff" stroke-width=".6"/>')


def cross(parts: list[str], x: float, y: float, color: str, size: float = 3.4) -> None:
    parts.append(
        f'<path d="M{x-size:.2f},{y-size:.2f} L{x+size:.2f},{y+size:.2f} M{x-size:.2f},{y+size:.2f} L{x+size:.2f},{y-size:.2f}" stroke="{color}" stroke-width="1.4"/>'
    )


def polyline(
    parts: list[str],
    points: list[tuple[float, float]],
    color: str,
    width: float = 2.0,
    dash: str = "",
) -> None:
    encoded = " ".join(f"{x:.2f},{y:.2f}" for x, y in points)
    dash_attr = f' stroke-dasharray="{dash}"' if dash else ""
    parts.append(
        f'<polyline points="{encoded}" fill="none" stroke="{color}" stroke-width="{width}"{dash_attr}/>'
    )


def write_svg(name: str, parts: list[str]) -> None:
    parts.append("</svg>")
    (FIG / f"{name}.svg").write_text("\n".join(parts) + "\n", encoding="utf-8")


def write_sidecar(
    name: str,
    title: str,
    caption: str,
    takeaway: str,
    sources: tuple[str, ...],
    methods: str,
    limitations: str,
    accessibility: str,
) -> None:
    bindings = "\n".join(
        f"- `{source}`, SHA-256 `{sha(ART / source)}`" for source in sources
    )
    content = f"""# {title}

## Caption

{caption}

## Plain-language takeaway

{takeaway}

## Methods and source bindings

{methods}

Exact result bindings:

{bindings}

## Assumptions and evidence ceiling

All relative model levels and constraint-removal scenarios are
`ASSUMED_FOR_EXECUTION`. They diagnose scale and mathematical suppression;
they are not fitted observation operators, calibrated parameters,
physiological bounds, process replacements, or production recommendations.
PhenoCam GCC is not treated as GSI, LAI, biomass, or canopy cover. Order 7
remains held.

## Limitations

{limitations}

## Accessibility

{accessibility}
"""
    (FIG / f"{name}.md").write_text(content, encoding="utf-8")


def crossing_map() -> None:
    absolute = rows("absolute-reproduction.csv")
    source = [
        row for row in rows("source-level-audit.csv") if row["source_level"] in {"0.10", "0.25", "0.50"}
    ]
    members = sorted({row["member_or_default"] for row in absolute})
    member_index = {member: index for index, member in enumerate(members)}
    events = ["2024-falling", "2024-rising", "2025-falling", "2025-rising"]
    parts = svg_start(
        1280,
        900,
        "CAL-07D event-by-member crossing map",
        "Four panels show residual days for absolute and source-level model crossings. Unmatched rows appear in a labeled right-hand column.",
    )
    plotted = [
        {
            "kind": "ABS",
            **row,
        }
        for row in absolute
    ] + [
        {
            "kind": row["source_level"],
            **row,
        }
        for row in source
    ]
    bind_plot_data(
        parts,
        ("absolute-reproduction.csv", "source-level-audit.csv"),
        plotted,
        ("kind", "member_or_default", "event_id", "residual_days"),
    )
    text_at(parts, 60, 34, "Beza transition crossings: matched timing and explicit unmatched rows", "title")
    x_min, x_max = -130.0, 140.0
    unmatched_x = 1180.0
    shapes = {
        "ABS": (COLORS["navy"], "circle"),
        "0.10": (COLORS["orange"], "square"),
        "0.25": (COLORS["green"], "diamond"),
        "0.50": (COLORS["purple"], "cross"),
    }
    for panel, event in enumerate(events):
        col = panel % 2
        row = panel // 2
        left = 70 + col * 610
        top = 70 + row * 390
        width, height = 555, 330
        parts.append(f'<rect x="{left}" y="{top}" width="{width}" height="{height}" class="panel"/>')
        text_at(parts, left + 8, top + 20, event.replace("-", " "), "label")
        plot_left, plot_right = left + 55, left + 465
        plot_top, plot_bottom = top + 36, top + 305
        for tick in (-120, -60, 0, 60, 120):
            x = plot_left + (tick - x_min) / (x_max - x_min) * (plot_right - plot_left)
            line(parts, x, plot_top, x, plot_bottom, "grid")
            text_at(parts, x, plot_bottom + 16, tick, "small", "middle")
        text_at(parts, left + 520, plot_bottom + 16, "unmatched", "small", "middle")
        line(parts, plot_left, plot_bottom, plot_right, plot_bottom)
        event_absolute = [item for item in absolute if item["event_id"] == event]
        event_source = [item for item in source if item["event_id"] == event]
        all_rows = [("ABS", item) for item in event_absolute] + [
            (item["source_level"], item) for item in event_source
        ]
        offsets = {"ABS": -3.0, "0.10": -1.0, "0.25": 1.0, "0.50": 3.0}
        for kind, item in all_rows:
            y = plot_top + (member_index[item["member_or_default"]] + 0.5) / len(members) * (
                plot_bottom - plot_top
            ) + offsets[kind]
            color, shape = shapes[kind]
            if item["residual_days"]:
                residual = float(item["residual_days"])
                x = plot_left + (residual - x_min) / (x_max - x_min) * (
                    plot_right - plot_left
                )
                x = min(plot_right, max(plot_left, x))
                {"circle": circle, "square": square, "diamond": diamond, "cross": cross}[shape](
                    parts, x, y, color
                )
            else:
                cross(parts, left + 520, y, color, 2.7)
        text_at(parts, left + 14, (plot_top + plot_bottom) / 2, "37 members", "small")
    legend_y = 875
    legend_x = 230
    for index, (label, (color, shape)) in enumerate(shapes.items()):
        x = legend_x + index * 210
        {"circle": circle, "square": square, "diamond": diamond, "cross": cross}[shape](
            parts, x, legend_y - 4, color, 4
        )
        label_text = "absolute GSI 0.5" if label == "ABS" else f"source/model level {label}"
        text_at(parts, x + 10, legend_y, label_text)
    write_svg("cal07d-crossing-map", parts)
    write_sidecar(
        "cal07d-crossing-map",
        "Event-By-Member Crossing Map",
        "Residual days for every frozen member and four internally bracketed Beza events. Symbols distinguish the absolute GSI 0.5 comparison from source-level-aligned retrospective model thresholds; colored crosses in the right column are unmatched rows.",
        "Changing the comparison scale recovers many crossings, especially at model levels 0.10 and 0.25, but rising 0.50 transitions remain unmatched and many recovered dates are still substantially displaced.",
        ("absolute-reproduction.csv", "source-level-audit.csv"),
        "`absolute-reproduction.csv` has 148 rows keyed by `(member_or_default,event_id)`; `source-level-audit.csv` has 444 rows keyed by `(member_or_default,event_id,source_level)`. The figure plots all 592 rows for 37 members and four eligible events. Event windows use adjacent source date-50 midpoints; the first chronological same-direction crossing inside `lower < crossing <= upper` is selected. Residual is modeled fractional ordinal minus the source date at the same normalized level. An empty `residual_days` field is rendered in the explicit unmatched column rather than assigned a numeric value.",
        "The source level/model level analogy is retrospective and does not establish that the two quantities measure the same biological state. POWER is gridded forcing and source transitions are provisional.",
        "Operator levels use distinct marker shapes as well as colors. Every member occupies the same vertical order; unmatched results have an explicit separate column.",
    )


def indicator_chronology() -> None:
    daily = [
        row
        for row in rows("daily-scenario-ensemble.csv")
        if row["scenario"] == "BASE" and row["year"] in {"2024", "2025"}
    ]
    attribution = rows("event-indicator-attribution.csv")
    parts = svg_start(
        1280,
        760,
        "CAL-07D GSI indicator chronology",
        "Two year panels show median temperature, VPD, photoperiod, instantaneous GSI, and trailing GSI with observed transition dates.",
    )
    bind_plot_data(
        parts,
        ("daily-scenario-ensemble.csv", "event-indicator-attribution.csv"),
        [
            {
                "record_type": "daily",
                "key_1": row["date"],
                "key_2": row["scenario"],
                "value_1": row["i_tmin_median"],
                "value_2": row["i_vpd_median"],
                "value_3": row["i_photo_median"],
                "value_4": row["instantaneous_gsi_median"],
                "value_5": row["gsi21_median"],
                "category": row["minimum_constraint_tie_counts"],
            }
            for row in daily
        ]
        + [
            {
                "record_type": "event",
                "key_1": row["source_date"],
                "key_2": row["event_id"],
                "value_1": "",
                "value_2": "",
                "value_3": "",
                "value_4": "",
                "value_5": "",
                "category": row["direction"],
            }
            for row in attribution
            if row["source_level"] == "0.50"
        ],
        (
            "record_type",
            "key_1",
            "key_2",
            "value_1",
            "value_2",
            "value_3",
            "value_4",
            "value_5",
            "category",
        ),
    )
    text_at(parts, 60, 34, "Which GSI constraints limit Beza activity?", "title")
    series_defs = (
        ("i_tmin_median", "temperature", COLORS["red"], ""),
        ("i_vpd_median", "VPD", COLORS["orange"], "7 3"),
        ("i_photo_median", "photoperiod", COLORS["purple"], "2 3"),
        ("instantaneous_gsi_median", "instantaneous product", COLORS["green"], "8 3"),
        ("gsi21_median", "GSI21", COLORS["navy"], ""),
    )
    for panel, year in enumerate(("2024", "2025")):
        left, top, width, height = 70, 70 + panel * 315, 1130, 270
        parts.append(f'<rect x="{left}" y="{top}" width="{width}" height="{height}" class="panel"/>')
        text_at(parts, left + 8, top + 20, year, "label")
        plot_left, plot_right = left + 55, left + width - 20
        plot_top, plot_bottom = top + 30, top + 225
        year_rows = [row for row in daily if row["year"] == year]
        max_doy = max(int(row["doy"]) for row in year_rows)
        for tick in (0.0, 0.25, 0.5, 0.75, 1.0):
            y = plot_bottom - tick * (plot_bottom - plot_top)
            line(parts, plot_left, y, plot_right, y, "grid")
            text_at(parts, plot_left - 8, y + 4, f"{tick:.2f}", "small", "end")
        for field, _, color, dash in series_defs:
            points = [
                (
                    plot_left
                    + (int(row["doy"]) - 1)
                    / (max_doy - 1)
                    * (plot_right - plot_left),
                    plot_bottom
                    - float(row[field]) * (plot_bottom - plot_top),
                )
                for row in year_rows
            ]
            polyline(parts, points, color, 2.0, dash)
        event_rows = [
            row
            for row in attribution
            if row["event_id"].startswith(year) and row["source_level"] == "0.50"
        ]
        for event in event_rows:
            doy = date.fromisoformat(event["source_date"]).timetuple().tm_yday
            x = plot_left + (doy - 1) / (max_doy - 1) * (plot_right - plot_left)
            parts.append(
                f'<line x1="{x:.2f}" y1="{plot_top:.2f}" x2="{x:.2f}" y2="{plot_bottom:.2f}" stroke="{COLORS["ink"]}" stroke-width="1.4" stroke-dasharray="4 3"/>'
            )
            text_at(parts, x, plot_top + 14, event["direction"], "small", "middle")
        # Daily modal minimum constraint as an explicit categorical strip.
        for index, row in enumerate(year_rows):
            counts = {}
            for item in row["minimum_constraint_tie_counts"].split(";"):
                key, count = item.rsplit(":", 1)
                counts[key] = int(count)
            category = max(sorted(counts), key=lambda key: counts[key])
            if "PHOTOPERIOD" in category and "VPD" in category:
                color = COLORS["gray"]
            elif "PHOTOPERIOD" in category:
                color = COLORS["purple"]
            elif "VPD" in category:
                color = COLORS["orange"]
            else:
                color = COLORS["red"]
            x = plot_left + index / max(1, len(year_rows) - 1) * (plot_right - plot_left)
            line(parts, x, plot_bottom + 8, x, plot_bottom + 16, css="axis")
            parts[-1] = parts[-1].replace('class="axis"', f'stroke="{color}" stroke-width="2"')
        text_at(parts, plot_left, plot_bottom + 32, "Jan")
        text_at(parts, plot_right, plot_bottom + 32, "Dec", "small", "end")
        text_at(parts, plot_left + 430, plot_bottom + 32, "daily modal minimum-constraint category", "small")
    text_at(
        parts,
        95,
        690,
        "minimum-constraint strip: photoperiod (purple), VPD (orange), temperature (red), photoperiod+VPD tie (gray)",
        "small",
    )
    legend_y = 730
    for index, (_, label, color, dash) in enumerate(series_defs):
        x = 95 + index * 220
        parts.append(
            f'<line x1="{x}" y1="{legend_y-4}" x2="{x+35}" y2="{legend_y-4}" stroke="{color}" stroke-width="2.4"'
            + (f' stroke-dasharray="{dash}"' if dash else "")
            + "/>"
        )
        text_at(parts, x + 43, legend_y, label)
    write_svg("cal07d-indicator-chronology", parts)
    write_sidecar(
        "cal07d-indicator-chronology",
        "GSI Indicator Chronology",
        "Daily BASE-ensemble median constraint indicators, instantaneous product, and 21-day GSI during 2024 and 2025. Vertical dashed markers are source date-50 transitions; the lower categorical strip shows the daily modal smallest-indicator tie set.",
        "Photoperiod is the modal limiting indicator through much of austral winter, while VPD becomes strongly suppressive near both observed green-up periods. At the date-50 rises, the median instantaneous product is near zero and the trailing GSI remains below 0.03.",
        ("daily-scenario-ensemble.csv", "event-indicator-attribution.csv"),
        "`daily-scenario-ensemble.csv` is keyed by `(scenario,date)` and has 9,996 rows: 1,666 dates for each of five 37-member scenarios plus one single-trajectory default. This figure uses the 731 BASE rows in 2024-2025; each line value is a 37-member daily median. `event-indicator-attribution.csv` has 12 rows keyed by `(event_id,source_level)`; the four source-level 0.50 rows provide the event markers. Event-window selection and unmatched crossing encoding do not apply to these daily chronology lines; the markers are retained source dates, not modeled matches.",
        "Smallest-indicator rank is mathematical, not causal. Ties are retained as categories. On-site meteorology, rainfall, soil moisture, and physiological observations are unavailable.",
        "Each quantity has a distinct line color and dash pattern. Events are labeled in text, axes are shared, and the limiting-category strip is supplemental to the numeric sidecar binding.",
    )


def scenario_screen() -> None:
    data = rows("scenario-event-screen.csv")
    labels = [
        ("BASE", "Base"),
        ("TEMPERATURE_UNCONSTRAINED", "Temperature=1"),
        ("VPD_UNCONSTRAINED", "VPD=1"),
        ("PHOTOPERIOD_UNCONSTRAINED", "Photoperiod=1"),
        ("PHOTOPERIOD_AND_VPD_UNCONSTRAINED", "Photo+VPD=1"),
        ("SC_PLANT_GENERALIZED_DEFAULT", "Canonical default"),
    ]
    parts = svg_start(
        1280,
        660,
        "CAL-07D constraint-removal effect screen",
        "Matched fractions and residual distributions compare the base, constraint-removal, and canonical default scenarios.",
    )
    bind_plot_data(
        parts,
        ("scenario-event-screen.csv", "decision-screen.csv"),
        data,
        (
            "scenario",
            "member_or_default",
            "event_id",
            "operator",
            "matched",
            "residual_days",
        ),
    )
    text_at(parts, 60, 34, "Constraint-removal timing effects are mathematical sensitivity, not solutions", "title")
    left, top, width, height = 70, 75, 520, 500
    right = 670
    parts.append(f'<rect x="{left}" y="{top}" width="{width}" height="{height}" class="panel"/>')
    parts.append(f'<rect x="{right}" y="{top}" width="{width}" height="{height}" class="panel"/>')
    text_at(parts, left + 10, top + 22, "Matched member-event fraction", "label")
    text_at(parts, right + 10, top + 22, "Residual days for matched rows", "label")
    for tick in (0, 25, 50, 75, 100):
        x = left + 190 + tick / 100 * 290
        line(parts, x, top + 40, x, top + 450, "grid")
        text_at(parts, x, top + 470, f"{tick}%", "small", "middle")
    for tick in (-100, -50, 0, 50, 100):
        x = right + 190 + (tick + 110) / 220 * 290
        line(parts, x, top + 40, x, top + 450, "grid")
        text_at(parts, x, top + 470, tick, "small", "middle")
    for index, (scenario, label) in enumerate(labels):
        y = top + 65 + index * 65
        text_at(parts, left + 10, y + 4, label)
        text_at(parts, right + 10, y + 4, label)
        scenario_rows = [row for row in data if row["scenario"] == scenario]
        denominator = len(scenario_rows) // 2
        for operator, color, offset, shape in (
            ("ABSOLUTE_0_5", COLORS["navy"], -8, circle),
            ("EVENT_YEAR_RELATIVE", COLORS["orange"], 8, square),
        ):
            selected = [row for row in scenario_rows if row["operator"] == operator]
            matched = [row for row in selected if row["residual_days"]]
            fraction = 100 * len(matched) / max(1, len(selected))
            x = left + 190 + fraction / 100 * 290
            shape(parts, x, y + offset, color, 5)
            if matched:
                residuals = [float(row["residual_days"]) for row in matched]
                low, median, high = min(residuals), statistics.median(residuals), max(residuals)
                scale = lambda value: right + 190 + (max(-110, min(110, value)) + 110) / 220 * 290
                line(parts, scale(low), y + offset, scale(high), y + offset)
                shape(parts, scale(median), y + offset, color, 5)
            else:
                cross(parts, right + 190, y + offset, color, 4)
            text_at(parts, x + 8, y + offset + 4, f"{len(matched)}/{len(selected)}")
    legend_y = 625
    circle(parts, 390, legend_y - 4, COLORS["navy"], 5)
    text_at(parts, 402, legend_y, "absolute GSI 0.5")
    square(parts, 610, legend_y - 4, COLORS["orange"], 8)
    text_at(parts, 622, legend_y, "event-year relative midpoint")
    cross(parts, 920, legend_y - 4, COLORS["gray"], 4)
    text_at(parts, 932, legend_y, "no matched residual")
    write_svg("cal07d-constraint-removal-screen", parts)
    write_sidecar(
        "cal07d-constraint-removal-screen",
        "Constraint-Removal Effect Screen",
        "Matched fractions and matched-row residual ranges for BASE, one-indicator removals, combined photoperiod/VPD removal, and the single canonical generalized-default trajectory. Circles are absolute GSI 0.5; squares are retrospective event-year relative midpoints.",
        "VPD removal creates all 148 relative-midpoint matches, but falling transitions remain early and rising transitions late. Photoperiod removal creates many absolute matches with smaller yet still directionally structured residuals. The canonical default crosses all four events but makes falling late and rising early. These results show mathematical sensitivity without identifying correct thresholds, forcing, or missing process physics.",
        ("scenario-event-screen.csv", "decision-screen.csv"),
        "`scenario-event-screen.csv` has 1,488 rows keyed by `(scenario,member_or_default,event_id,operator)`: 296 rows for each 37-member scenario and eight rows for the single default trajectory. Each unconstrained scenario is recomputed from 2022-01-01 with only its named indicator set to one before multiplication and FIFO admission. For each operator, event windows use adjacent date-50 midpoints and select the first same-direction crossing under `lower < crossing <= upper`. Empty `residual_days` means unmatched and contributes to the printed denominator but not the residual range. `decision-screen.csv` has seven rows keyed by `hypothesis` and supplies interpretation only; the plotted counts and ranges come from the event screen.",
        "Match availability is not timing accuracy. Residual ranges are shown without a pass tolerance. Combined removal can erase absolute crossings by keeping the trajectory above 0.5.",
        "Absolute and relative operators use separate marker shapes and colors. Counts are printed beside every point; residual ranges include a zero reference gridline.",
    )


def threshold_sensitivity() -> None:
    data = rows("model-level-sensitivity.csv")
    events = ["2024-falling", "2024-rising", "2025-falling", "2025-rising"]
    style = {
        "2024-falling": (COLORS["navy"], "circle"),
        "2024-rising": (COLORS["orange"], "square"),
        "2025-falling": (COLORS["green"], "diamond"),
        "2025-rising": (COLORS["purple"], "cross"),
    }
    grouped: dict[tuple[str, float], list[dict[str, str]]] = defaultdict(list)
    for row in data:
        grouped[(row["event_id"], float(row["model_level"]))].append(row)
    levels = sorted({float(row["model_level"]) for row in data})
    parts = svg_start(
        1280,
        650,
        "CAL-07D model-level threshold sensitivity",
        "Two panels show matched fraction and median residual as the retrospective model-relative level varies.",
    )
    bind_plot_data(
        parts,
        ("model-level-sensitivity.csv",),
        data,
        (
            "member_or_default",
            "event_id",
            "model_level",
            "matched",
            "residual_days",
        ),
    )
    text_at(parts, 60, 34, "Transition conclusions depend strongly on the retrospective model level", "title")
    panels = (
        (70, "Matched fraction", 0.0, 1.0),
        (670, "Median residual among matched rows (days)", -130.0, 130.0),
    )
    for left, title, ymin, ymax in panels:
        top, width, height = 75, 520, 470
        parts.append(f'<rect x="{left}" y="{top}" width="{width}" height="{height}" class="panel"/>')
        text_at(parts, left + 10, top + 22, title, "label")
        plot_left, plot_right = left + 60, left + width - 20
        plot_top, plot_bottom = top + 40, top + height - 45
        for level in levels:
            x = plot_left + (level - 0.1) / 0.8 * (plot_right - plot_left)
            line(parts, x, plot_top, x, plot_bottom, "grid")
            text_at(parts, x, plot_bottom + 18, f"{level:.2g}", "small", "middle")
        for fraction in (0.0, 0.25, 0.5, 0.75, 1.0):
            value = ymin + fraction * (ymax - ymin)
            y = plot_bottom - fraction * (plot_bottom - plot_top)
            line(parts, plot_left, y, plot_right, y, "grid")
            text_at(parts, plot_left - 8, y + 4, f"{value:.0f}" if ymax > 2 else f"{value:.2f}", "small", "end")
        for event in events:
            color, shape_name = style[event]
            points = []
            for level in levels:
                selected = grouped[(event, level)]
                matched = [row for row in selected if row["residual_days"]]
                if left == 70:
                    value = len(matched) / len(selected)
                else:
                    if not matched:
                        continue
                    value = statistics.median(float(row["residual_days"]) for row in matched)
                x = plot_left + (level - 0.1) / 0.8 * (plot_right - plot_left)
                y = plot_bottom - (value - ymin) / (ymax - ymin) * (plot_bottom - plot_top)
                points.append((x, y))
            polyline(parts, points, color, 2.0)
            shape = {"circle": circle, "square": square, "diamond": diamond, "cross": cross}[shape_name]
            for x, y in points:
                shape(parts, x, y, color, 4)
        text_at(parts, (plot_left + plot_right) / 2, top + height - 10, "event-year relative model level", "small", "middle")
    legend_y = 615
    for index, event in enumerate(events):
        color, shape_name = style[event]
        x = 220 + index * 220
        shape = {"circle": circle, "square": square, "diamond": diamond, "cross": cross}[shape_name]
        shape(parts, x, legend_y - 4, color, 5)
        text_at(parts, x + 12, legend_y, event.replace("-", " "))
    write_svg("cal07d-threshold-sensitivity", parts)
    write_sidecar(
        "cal07d-threshold-sensitivity",
        "Model-Level Threshold Sensitivity",
        "Matched member fractions and median matched-row residuals across eleven prospectively frozen retrospective model levels. Each line is one source date-50 event.",
        "Lower relative levels produce many crossings, but inferred timing and even match availability change sharply with level. No single relative level resolves both rising and falling chronology across years.",
        ("model-level-sensitivity.csv",),
        "`model-level-sensitivity.csv` has 1,628 rows keyed by `(member_or_default,event_id,model_level)`: 37 members × four events × eleven levels. For each member/event/level, one threshold is calculated from the complete event-year GSI range and held constant over the adjacent-event window. The first chronological same-direction crossing under `lower < crossing <= upper` is selected. Every fraction denominator retains all 37 rows; an empty `residual_days` is unmatched. Residual medians use matched rows only, and a level with no matches is omitted from the right panel rather than imputed.",
        "Complete-year extrema use retrospective information and cannot serve as a predictive operator. No timing tolerance or preferred level is selected.",
        "Events use distinct colors and marker shapes. The left panel retains unmatched rows through matched fractions; the right panel omits levels with no matched residual rather than imputing values.",
    )


def manifest() -> None:
    paths = [
        *sorted(ART.glob("*.csv")),
        *sorted(FIG.glob("*.svg")),
        *sorted(FIG.glob("*.md")),
    ]
    paths = [path for path in paths if path.name != "result-manifest.csv"]
    with (ART / "result-manifest.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(
            stream,
            fieldnames=("path", "sha256", "bytes"),
            lineterminator="\n",
        )
        writer.writeheader()
        for path in paths:
            writer.writerow(
                {
                    "path": path.relative_to(PKG),
                    "sha256": sha(path),
                    "bytes": path.stat().st_size,
                }
            )


def main() -> None:
    FIG.mkdir(exist_ok=True)
    crossing_map()
    indicator_chronology()
    scenario_screen()
    threshold_sensitivity()
    manifest()
    print("CAL-07D figures PASS: 4 SVG plots and 4 Markdown sidecars")


if __name__ == "__main__":
    main()
