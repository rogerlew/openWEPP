#!/usr/bin/env python3
"""Render deterministic accessible SVG figures for CAL-06."""

from __future__ import annotations

import csv
import hashlib
import html
import math
import statistics
from collections import defaultdict
from pathlib import Path
from typing import Any, Iterable

PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
WIDTH = 1240
HEIGHT = 820
COLORS = {
    "open": "#4d4d4d",
    "deciduous": "#2b7a0b",
    "mixed": "#8b5a2b",
    "conifer": "#2456a6",
}
DASHES = {
    "open": "2 5",
    "deciduous": "",
    "mixed": "8 4",
    "conifer": "3 3 9 3",
}
SITE_LABELS = {
    "marcell": "Marcell",
    "harvard": "Harvard",
    "hubbard_brook": "Hubbard Brook",
}


def read_csv(name: str) -> list[dict[str, str]]:
    with (ARTIFACTS / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def svg_start(title: str, description: str, height: int = HEIGHT) -> list[str]:
    return [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{height}" '
        f'viewBox="0 0 {WIDTH} {height}" role="img" aria-labelledby="title desc">',
        f"<title id=\"title\">{html.escape(title)}</title>",
        f"<desc id=\"desc\">{html.escape(description)}</desc>",
        "<style>",
        "text{font-family:Arial,sans-serif;fill:#1f2933}"
        ".title{font-size:24px;font-weight:700}.subtitle{font-size:13px}"
        ".axis{stroke:#52616b;stroke-width:1}.grid{stroke:#d9e2ec;stroke-width:1}"
        ".label{font-size:12px}.small{font-size:10px}.panel{fill:#fbfcfd;stroke:#bcccdc}"
        ".band{opacity:.18}.status{font-size:12px;font-weight:700}",
        "</style>",
        '<rect width="100%" height="100%" fill="#ffffff"/>',
    ]


def svg_end(parts: list[str], path: Path) -> None:
    parts.append("</svg>")
    path.write_text("\n".join(parts) + "\n", encoding="utf-8")


def source_binding(parts: list[str], names: tuple[str, ...]) -> None:
    bindings = []
    for name in names:
        digest = hashlib.sha256((ARTIFACTS / name).read_bytes()).hexdigest()
        bindings.append(f"{name}:{digest}")
    parts.append(
        f'<metadata id="source-bindings">{html.escape(";".join(bindings))}</metadata>'
    )


def path_points(points: Iterable[tuple[float, float]]) -> str:
    return " ".join(
        ("M" if index == 0 else "L") + f"{x:.2f},{y:.2f}"
        for index, (x, y) in enumerate(points)
    )


def numeric(value: str) -> float | None:
    if value == "":
        return None
    result = float(value)
    return result if math.isfinite(result) else None


def daily_bands(
    daily: list[dict[str, str]], site: str, field: str
) -> dict[str, list[tuple[int, float, float, float]]]:
    grouped: dict[tuple[str, int], list[float]] = defaultdict(list)
    for row in daily:
        if row["site"] != site:
            continue
        value = numeric(row[field])
        if value is not None:
            grouped[(row["stratum"], int(row["day_of_year"]))].append(value)
    output: dict[str, list[tuple[int, float, float, float]]] = defaultdict(list)
    for (stratum, doy), values in sorted(grouped.items()):
        output[stratum].append((doy, min(values), statistics.median(values), max(values)))
    return output


def panel_axes(
    parts: list[str],
    x: float,
    y: float,
    width: float,
    height: float,
    label: str,
    ymin: float,
    ymax: float,
    show_months: bool = True,
) -> None:
    parts.append(
        f'<rect x="{x:.1f}" y="{y:.1f}" width="{width:.1f}" height="{height:.1f}" class="panel"/>'
    )
    parts.append(f'<text x="{x+8:.1f}" y="{y+17:.1f}" class="label">{html.escape(label)}</text>')
    for fraction in (0.0, 0.5, 1.0):
        gy = y + height - 22 - fraction * (height - 44)
        value = ymin + fraction * (ymax - ymin)
        parts.append(
            f'<line x1="{x+42:.1f}" y1="{gy:.1f}" x2="{x+width-10:.1f}" y2="{gy:.1f}" class="grid"/>'
        )
        parts.append(
            f'<text x="{x+4:.1f}" y="{gy+4:.1f}" class="small">{value:.2g}</text>'
        )
    if show_months:
        for doy, text_label in (
            (1, "Jan"),
            (91, "Apr"),
            (182, "Jul"),
            (274, "Oct"),
            (365, "Dec"),
        ):
            gx = x + 42 + (doy - 1) / 364 * (width - 52)
            parts.append(
                f'<text x="{gx:.1f}" y="{y+height-6:.1f}" text-anchor="middle" class="small">{text_label}</text>'
            )


def draw_bands(
    parts: list[str],
    bands: dict[str, list[tuple[int, float, float, float]]],
    x: float,
    y: float,
    width: float,
    height: float,
    ymin: float,
    ymax: float,
) -> None:
    def point(doy: int, value: float) -> tuple[float, float]:
        px = x + 42 + (doy - 1) / 364 * (width - 52)
        py = y + height - 22 - (value - ymin) / (ymax - ymin) * (height - 44)
        return px, py

    for stratum in ("open", "deciduous", "mixed", "conifer"):
        values = bands.get(stratum, [])
        if not values:
            continue
        upper = [point(doy, high) for doy, _, _, high in values]
        lower = [point(doy, low) for doy, low, _, _ in reversed(values)]
        polygon = " ".join(f"{px:.2f},{py:.2f}" for px, py in upper + lower)
        parts.append(
            f'<polygon points="{polygon}" fill="{COLORS[stratum]}" class="band"/>'
        )
        median = [point(doy, middle) for doy, _, middle, _ in values]
        dash = f' stroke-dasharray="{DASHES[stratum]}"' if DASHES[stratum] else ""
        parts.append(
            f'<path d="{path_points(median)}" fill="none" stroke="{COLORS[stratum]}" '
            f'stroke-width="2"{dash}/>'
        )


def legend(
    parts: list[str],
    y: float,
    strata: tuple[str, ...] = ("open", "deciduous", "mixed", "conifer"),
) -> None:
    x = 665
    for stratum in strata:
        dash = f' stroke-dasharray="{DASHES[stratum]}"' if DASHES[stratum] else ""
        parts.append(
            f'<line x1="{x}" y1="{y}" x2="{x+30}" y2="{y}" stroke="{COLORS[stratum]}" '
            f'stroke-width="3"{dash}/>'
        )
        parts.append(f'<text x="{x+36}" y="{y+4}" class="small">{stratum}</text>')
        x += 135


def render_canopy(daily: list[dict[str, str]]) -> None:
    parts = svg_start(
        "CAL-06 canopy chronology: complete timing ensemble",
        "Daily median lines and full member ranges; category amplitude operands remain data-limited.",
        980,
    )
    source_binding(parts, ("daily-climatology.csv",))
    legend(parts, 82)
    metrics = (
        ("canopy_cover_fraction", "Canopy cover", 0.0, 1.0),
        ("lai_m2_m2", "LAI (m²/m²)", 0.0, 2.1),
        ("gsi21", "GSI21", 0.0, 1.0),
    )
    panel_w, panel_h = 370, 265
    for column, site in enumerate(("marcell", "harvard", "hubbard_brook")):
        for row_index, (field, metric, ymin, ymax) in enumerate(metrics):
            x = 42 + column * 395
            y = 105 + row_index * 280
            panel_axes(
                parts,
                x,
                y,
                panel_w,
                panel_h,
                f"{SITE_LABELS[site]} — {metric}",
                ymin,
                ymax,
            )
            draw_bands(
                parts,
                daily_bands(daily, site, field),
                x,
                y,
                panel_w,
                panel_h,
                ymin,
                ymax,
            )
    svg_end(parts, FIGURES / "cal06-canopy-chronology.svg")


def render_seasonal(summary: list[dict[str, str]]) -> None:
    parts = svg_start(
        "CAL-06 seasonal ordering and amplitude",
        "Bars are ensemble medians; whiskers span all 37 timing members. Open controls are not native canopy lanes.",
    )
    source_binding(parts, ("ensemble-summary.csv",))
    metrics = (
        ("winter_cover_mean", "Winter cover"),
        ("summer_cover_max", "Summer cover"),
        ("cover_amplitude", "Seasonal amplitude"),
        ("summer_lai_max", "Summer LAI"),
    )
    by_site: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in summary:
        by_site[row["site"]].append(row)
    for panel_index, (field, label) in enumerate(metrics):
        x = 42 + (panel_index % 2) * 595
        y = 100 + (panel_index // 2) * 350
        panel_axes(
            parts,
            x,
            y,
            565,
            320,
            label,
            0.0,
            2.1 if "lai" in field else 1.0,
            show_months=False,
        )
        entries = [
            row
            for site in ("marcell", "harvard", "hubbard_brook")
            for row in by_site[site]
            if row[f"{field}_median"] != ""
        ]
        ymax = 2.1 if "lai" in field else 1.0
        for index, row in enumerate(entries):
            bx = x + 55 + index * (490 / max(1, len(entries)))
            median = float(row[f"{field}_median"])
            low = float(row[f"{field}_min"])
            high = float(row[f"{field}_max"])
            base = y + 270
            scale = 230 / ymax
            top = base - median * scale
            parts.append(
                f'<rect x="{bx:.1f}" y="{top:.1f}" width="25" height="{base-top:.1f}" '
                f'fill="{COLORS[row["stratum"]]}" opacity=".72"/>'
            )
            parts.append(
                f'<line x1="{bx+12.5:.1f}" y1="{base-high*scale:.1f}" x2="{bx+12.5:.1f}" '
                f'y2="{base-low*scale:.1f}" stroke="#111827" stroke-width="1.5"/>'
            )
            parts.append(
                f'<text transform="translate({bx+8:.1f},{base+10:.1f}) rotate(55)" '
                f'class="small">{SITE_LABELS[row["site"]]} {row["stratum"]}</text>'
            )
    svg_end(parts, FIGURES / "cal06-seasonal-ordering-amplitude.svg")


def render_snow(daily: list[dict[str, str]]) -> None:
    parts = svg_start(
        "CAL-06 snow response",
        "Daily SWE climatology for the frozen canopy ensemble.",
        500,
    )
    source_binding(parts, ("daily-climatology.csv",))
    legend(parts, 82)
    for column, site in enumerate(("marcell", "harvard", "hubbard_brook")):
        x = 42 + column * 395
        y = 110
        bands = daily_bands(daily, site, "swe_mm")
        ymax = max(
            (high for values in bands.values() for _, _, _, high in values),
            default=1.0,
        )
        panel_axes(parts, x, y, 370, 355, f"{SITE_LABELS[site]} — SWE (mm)", 0.0, ymax)
        draw_bands(parts, bands, x, y, 370, 355, 0.0, ymax)
    svg_end(parts, FIGURES / "cal06-snow-response.svg")


def render_litter(daily: list[dict[str, str]]) -> None:
    parts = svg_start(
        "CAL-06 litter, residue, and frost chronology",
        "Daily model-response plots for the Marcell strata.",
        1050,
    )
    source_binding(parts, ("daily-climatology.csv",))
    legend(parts, 82)
    panels = (
        ("leaf_litter_kg_m2", "Leaf litter (kg/m²/day)"),
        ("surface_residue_kg_m2", "Surface residue (kg/m²)"),
        ("residue_depth_m", "Residue depth (m)"),
        ("frost_depth_mm", "Frost depth (mm)"),
    )
    for index, (field, label) in enumerate(panels):
        x = 42
        y = 110 + index * 220
        bands = daily_bands(daily, "marcell", field)
        ymax = max(
            (high for values in bands.values() for _, _, _, high in values),
            default=1.0,
        )
        if ymax <= 0:
            ymax = 1.0
        panel_axes(parts, x, y, 1155, 195, label, 0.0, ymax)
        draw_bands(parts, bands, x, y, 1155, 195, 0.0, ymax)
    svg_end(parts, FIGURES / "cal06-litter-residue-frost.svg")


def render_downstream(daily: list[dict[str, str]]) -> None:
    parts = svg_start(
        "CAL-06 downstream model responses",
        "Interception, ET, and runoff ran through real consumers but are not advanced because upstream evidence remains bounded or missing.",
    )
    source_binding(parts, ("daily-climatology.csv",))
    legend(parts, 82, ("open", "deciduous", "mixed"))
    fields = (
        ("interception_mm", "Interception (mm/day)"),
        ("et_mm", "ET (mm/day)"),
        ("runoff_mm", "Runoff (mm/day)"),
    )
    for index, (field, label) in enumerate(fields):
        x = 42
        y = 110 + index * 220
        bands = daily_bands(daily, "harvard", field)
        ymax = max(
            (high for values in bands.values() for _, _, _, high in values),
            default=1.0,
        )
        if ymax <= 0:
            ymax = 1.0
        panel_axes(parts, x, y, 1155, 195, f"Harvard — {label}", 0.0, ymax)
        draw_bands(parts, bands, x, y, 1155, 195, 0.0, ymax)
    svg_end(parts, FIGURES / "cal06-downstream-consequences.svg")


def render_matrix(verdicts: list[dict[str, str]]) -> None:
    height = 910
    parts = svg_start(
        "CAL-06 congruence verdict matrix",
        "Every prespecified cell is shown. Bounded evidence and not-evaluated cells remain distinct from support and measured zero.",
        height,
    )
    source_binding(parts, ("verdict-matrix.csv",))
    headers = ("Cell", "Status", "Advancement")
    xs = (45, 250, 520)
    for x, label in zip(xs, headers):
        parts.append(f'<text x="{x}" y="95" class="status">{label}</text>')
    y = 115
    fills = {"BOUNDED": "#fff7cc", "NOT_EVALUATED": "#edf2f7", "SUPPORTED": "#dcfce7"}
    symbols = {"BOUNDED": "△", "NOT_EVALUATED": "—", "SUPPORTED": "✓"}
    for row in verdicts:
        status = row["status"]
        parts.append(
            f'<rect x="38" y="{y}" width="1160" height="50" fill="{fills.get(status, "#fee2e2")}" '
            'stroke="#cbd5e1"/>'
        )
        parts.append(f'<text x="45" y="{y+30}" class="label">{row["cell_id"]}</text>')
        parts.append(
            f'<text x="195" y="{y+30}" class="status">{symbols.get(status, "!")} {status}</text>'
        )
        parts.append(
            f'<text x="520" y="{y+30}" class="small">{html.escape(row["advancement"])}</text>'
        )
        y += 57
    svg_end(parts, FIGURES / "cal06-congruence-verdict-matrix.svg")


def main() -> int:
    FIGURES.mkdir(parents=True, exist_ok=True)
    daily = read_csv("daily-climatology.csv")
    summary = read_csv("ensemble-summary.csv")
    verdicts = read_csv("verdict-matrix.csv")
    render_canopy(daily)
    render_seasonal(summary)
    render_snow(daily)
    render_litter(daily)
    render_downstream(daily)
    render_matrix(verdicts)
    print("PASS: rendered 6 deterministic accessible CAL-06 SVG figures")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
