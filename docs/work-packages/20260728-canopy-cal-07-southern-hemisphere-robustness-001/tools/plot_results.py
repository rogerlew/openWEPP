#!/usr/bin/env python3
"""Render deterministic accessible SVG figures for CAL-07."""

from __future__ import annotations

import csv
import hashlib
import html
import math
import statistics
from collections import defaultdict
from datetime import date
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ART = PKG / "artifacts"
FIG = ART / "figures"
WIDTH = 1240
HEIGHT = 780
GREEN = "#16823A"
BLUE = "#2864A5"
ORANGE = "#C25B12"
PURPLE = "#7651A3"
GRAY = "#5B6770"
LIGHT = "#DCE8F2"


def rows(name: str) -> list[dict[str, str]]:
    with (ART / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def start(title: str, description: str) -> list[str]:
    return [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" '
        f'viewBox="0 0 {WIDTH} {HEIGHT}" role="img" aria-labelledby="title desc">',
        f'<title id="title">{html.escape(title)}</title>',
        f'<desc id="desc">{html.escape(description)}</desc>',
        "<style>text{font-family:Arial,sans-serif;fill:#182026}"
        ".title{font-size:24px;font-weight:700}.label{font-size:13px}"
        ".small{font-size:11px}.axis{stroke:#52616b;stroke-width:1}"
        ".grid{stroke:#d9e2ec;stroke-width:1}.panel{fill:#fff;stroke:#bcccdc}"
        ".band{fill:#8fbada;opacity:.35}</style>",
        '<rect width="100%" height="100%" fill="#ffffff"/>',
    ]


def finish(parts: list[str], path: Path, bindings: tuple[str, ...]) -> None:
    metadata = ";".join(
        f"{name}:{hashlib.sha256((ART / name).read_bytes()).hexdigest()}"
        for name in bindings
    )
    parts.append(f'<metadata id="source-bindings">{html.escape(metadata)}</metadata>')
    parts.append("</svg>")
    path.write_text("\n".join(parts) + "\n", encoding="utf-8")


def path(points: list[tuple[float, float]]) -> str:
    return " ".join(
        ("M" if index == 0 else "L") + f"{x:.2f},{y:.2f}"
        for index, (x, y) in enumerate(points)
    )


def panel(parts: list[str], x: float, y: float, w: float, h: float, label: str) -> None:
    parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" class="panel"/>')
    parts.append(f'<text x="{x + 8}" y="{y + 20}" class="label">{html.escape(label)}</text>')
    for fraction in (0.0, 0.25, 0.5, 0.75, 1.0):
        py = y + h - fraction * h
        parts.append(f'<line x1="{x}" y1="{py}" x2="{x + w}" y2="{py}" class="grid"/>')
        parts.append(f'<text x="{x - 9}" y="{py + 4}" text-anchor="end" class="small">{fraction:.2g}</text>')


def normalized_observations(daily: list[dict[str, str]], site: str) -> dict[str, float]:
    by_year: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in daily:
        if row["site_id"] == site and row["observed_gcc90"]:
            by_year[row["year"]].append(row)
    result = {}
    for annual in by_year.values():
        values = [float(row["observed_gcc90"]) for row in annual]
        low, high = min(values), max(values)
        if high > low:
            result.update(
                (row["date"], (float(row["observed_gcc90"]) - low) / (high - low))
                for row in annual
            )
    return result


def observed_and_modeled() -> None:
    daily = rows("ensemble-daily.csv")
    parts = start(
        "Observed greenness and frozen-ensemble GSI in two Southern Hemisphere forests",
        "Two time-series panels compare annually normalized camera GCC90 with the 37-member GSI median and 5th-to-95th percentile band during 2024 and 2025.",
    )
    parts.append('<text x="50" y="42" class="title">Observed greenness and frozen-ensemble GSI</text>')
    begin = date(2024, 1, 1).toordinal()
    end = date(2025, 12, 31).toordinal()
    for index, (site, label) in enumerate(
        (("SH-DB-BEZA", "Beza Mahafaly — deciduous dry forest"), ("SH-EN-ALERCE", "Alerce Costero — evergreen forest"))
    ):
        x, y, w, h = 85, 90 + index * 325, 1090, 260
        panel(parts, x, y, w, h, label)
        selected = [row for row in daily if row["site_id"] == site and row["year"] in {"2024", "2025"}]
        px = lambda day: x + (date.fromisoformat(day).toordinal() - begin) / (end - begin) * w
        py = lambda value: y + h - value * h
        upper = [(px(row["date"]), py(float(row["gsi_p95"]))) for row in selected]
        lower = [(px(row["date"]), py(float(row["gsi_p05"]))) for row in reversed(selected)]
        polygon = " ".join(f"{a:.2f},{b:.2f}" for a, b in upper + lower)
        parts.append(f'<polygon points="{polygon}" class="band"/>')
        median = [(px(row["date"]), py(float(row["gsi_median"]))) for row in selected]
        parts.append(f'<path d="{path(median)}" fill="none" stroke="{BLUE}" stroke-width="2.4"/>')
        obs = normalized_observations(daily, site)
        observed_points = [(px(day), py(value)) for day, value in sorted(obs.items()) if day[:4] in {"2024", "2025"}]
        parts.append(
            f'<path d="{path(observed_points)}" fill="none" stroke="{GREEN}" stroke-width="1.5" stroke-dasharray="5 3"/>'
        )
        for year in (2024, 2025):
            tx = px(f"{year}-07-01")
            parts.append(f'<text x="{tx}" y="{y+h+24}" text-anchor="middle" class="small">{year}</text>')
    parts.extend(
        [
            f'<line x1="360" y1="744" x2="400" y2="744" stroke="{BLUE}" stroke-width="3"/>',
            '<text x="408" y="749" class="small">GSI ensemble median (band: 5th–95th percentile)</text>',
            f'<line x1="720" y1="744" x2="760" y2="744" stroke="{GREEN}" stroke-width="2" stroke-dasharray="5 3"/>',
            '<text x="768" y="749" class="small">annually normalized raw GCC90</text>',
        ]
    )
    finish(parts, FIG / "cal07-observed-and-modeled-seasons.svg", ("ensemble-daily.csv",))


def transition_residuals() -> None:
    data = rows("transition-residuals.csv")
    events = sorted({(row["observed_date_50"], row["direction"]) for row in data})
    parts = start(
        "Beza Mahafaly deciduous midpoint timing residuals",
        "Dots show modeled GSI 0.5 crossing date minus provisional PhenoCam GCC 50 percent transition date for every retained ensemble member and four internally bracketed events.",
    )
    parts.append('<text x="50" y="42" class="title">Deciduous midpoint timing residuals</text>')
    x, y, w, h = 110, 90, 1060, 580
    parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" class="panel"/>')
    values = [float(row["residual_days"]) for row in data if row["residual_days"]]
    limit = max(30.0, math.ceil(max(abs(value) for value in values) / 10) * 10)
    py = lambda value: y + h / 2 - value / (2 * limit) * h
    for value in range(-int(limit), int(limit) + 1, 30):
        yy = py(value)
        parts.append(f'<line x1="{x}" y1="{yy}" x2="{x+w}" y2="{yy}" class="grid"/>')
        parts.append(f'<text x="{x-10}" y="{yy+4}" text-anchor="end" class="small">{value:+d}</text>')
    parts.append(f'<line x1="{x}" y1="{py(0)}" x2="{x+w}" y2="{py(0)}" stroke="{GRAY}" stroke-width="2"/>')
    colors = {"rising": ORANGE, "falling": PURPLE}
    for event_index, event in enumerate(events):
        cx = x + (event_index + 0.5) * w / len(events)
        event_rows = [row for row in data if (row["observed_date_50"], row["direction"]) == event]
        for point_index, row in enumerate(event_rows):
            if not row["residual_days"]:
                continue
            jitter = ((point_index % 7) - 3) * 2.5
            parts.append(
                f'<circle cx="{cx+jitter:.2f}" cy="{py(float(row["residual_days"])):.2f}" r="3.2" '
                f'fill="{colors[event[1]]}" opacity=".62"/>'
            )
        parts.append(f'<text x="{cx}" y="{y+h+25}" text-anchor="middle" class="small">{event[0]}</text>')
        parts.append(f'<text x="{cx}" y="{y+h+41}" text-anchor="middle" class="small">{event[1]}</text>')
    parts.append('<text x="24" y="400" transform="rotate(-90 24 400)" class="label">modeled − observed (days)</text>')
    finish(parts, FIG / "cal07-deciduous-transition-residuals.svg", ("transition-residuals.csv",))


def seasonal_phase() -> None:
    daily = rows("ensemble-daily.csv")
    parts = start(
        "Austral seasonal phase by day of year",
        "Two day-of-year panels show the 2024 and 2025 mean ensemble GSI and normalized camera greenness for the deciduous and evergreen lanes.",
    )
    parts.append('<text x="50" y="42" class="title">Austral seasonal phase by day of year</text>')
    for index, (site, label) in enumerate(
        (("SH-DB-BEZA", "Deciduous dry forest"), ("SH-EN-ALERCE", "Evergreen forest"))
    ):
        x, y, w, h = 85, 90 + index * 325, 1090, 260
        panel(parts, x, y, w, h, label)
        selected = [row for row in daily if row["site_id"] == site and row["year"] in {"2024", "2025"}]
        obs = normalized_observations(daily, site)
        by_doy_gsi: dict[int, list[float]] = defaultdict(list)
        by_doy_obs: dict[int, list[float]] = defaultdict(list)
        for row in selected:
            doy = int(row["doy"])
            by_doy_gsi[doy].append(float(row["gsi_median"]))
            if row["date"] in obs:
                by_doy_obs[doy].append(obs[row["date"]])
        px = lambda doy: x + (doy - 1) / 365 * w
        py = lambda value: y + h - value * h
        gsi_points = [(px(doy), py(statistics.fmean(values))) for doy, values in sorted(by_doy_gsi.items())]
        obs_points = [(px(doy), py(statistics.fmean(values))) for doy, values in sorted(by_doy_obs.items())]
        parts.append(f'<path d="{path(gsi_points)}" fill="none" stroke="{BLUE}" stroke-width="2.5"/>')
        parts.append(f'<path d="{path(obs_points)}" fill="none" stroke="{GREEN}" stroke-width="1.7" stroke-dasharray="5 3"/>')
        for doy, month in ((1, "Jan"), (91, "Apr"), (182, "Jul"), (274, "Oct"), (365, "Dec")):
            parts.append(f'<text x="{px(doy)}" y="{y+h+23}" text-anchor="middle" class="small">{month}</text>')
    finish(parts, FIG / "cal07-southern-seasonal-phase.svg", ("ensemble-daily.csv",))


def evidence_boundaries() -> None:
    data = rows("verdict-matrix.csv")
    parts = start(
        "CAL-07 evidence boundaries",
        "A status plot shows which cells are bounded, supported, contradicted, or not evaluated; downstream gaps remain visible.",
    )
    parts.append('<text x="50" y="42" class="title">What CAL-07 can—and cannot—say</text>')
    positions = {"CONTRADICTED": 500, "NOT_EVALUATED": 690, "BOUNDED": 880, "SUPPORTED": 1070}
    colors = {"CONTRADICTED": "#B42318", "NOT_EVALUATED": GRAY, "BOUNDED": ORANGE, "SUPPORTED": GREEN}
    for status, xpos in positions.items():
        parts.append(f'<text x="{xpos}" y="83" text-anchor="middle" class="small">{status.replace("_", " ")}</text>')
    for index, row in enumerate(data):
        yy = 125 + index * 58
        parts.append(f'<line x1="460" y1="{yy}" x2="1110" y2="{yy}" class="grid"/>')
        parts.append(f'<text x="50" y="{yy+5}" class="label">{html.escape(row["cell"])}</text>')
        parts.append(f'<circle cx="{positions[row["status"]]}" cy="{yy}" r="9" fill="{colors[row["status"]]}"/>')
    finish(parts, FIG / "cal07-evidence-boundaries.svg", ("verdict-matrix.csv",))


def main() -> None:
    FIG.mkdir(exist_ok=True)
    observed_and_modeled()
    transition_residuals()
    seasonal_phase()
    evidence_boundaries()


if __name__ == "__main__":
    main()
