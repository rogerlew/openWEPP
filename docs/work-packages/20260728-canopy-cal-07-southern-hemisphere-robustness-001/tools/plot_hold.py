#!/usr/bin/env python3
"""Render accessible source-diagnostic SVGs for the CAL-07 hold."""

from __future__ import annotations

import csv
import hashlib
import html
import math
from datetime import date
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ART = PKG / "artifacts"
INPUT = PKG / "inputs"
FIG = ART / "figures"
WIDTH, HEIGHT = 1240, 780
BLUE, GREEN, RED, ORANGE, GRAY = "#2864A5", "#16823A", "#B42318", "#C25B12", "#5B6770"


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def begin(title: str, desc: str) -> list[str]:
    return [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" '
        f'viewBox="0 0 {WIDTH} {HEIGHT}" role="img" aria-labelledby="title desc">',
        f'<title id="title">{html.escape(title)}</title>',
        f'<desc id="desc">{html.escape(desc)}</desc>',
        "<style>text{font-family:Arial,sans-serif;fill:#182026}"
        ".title{font-size:24px;font-weight:700}.label{font-size:13px}"
        ".small{font-size:11px}.grid{stroke:#d9e2ec;stroke-width:1}"
        ".panel{fill:#fff;stroke:#bcccdc}</style>",
        '<rect width="100%" height="100%" fill="#fff"/>',
        f'<text x="50" y="42" class="title">{html.escape(title)}</text>',
    ]


def finish(parts: list[str], output: str, bindings: tuple[Path, ...]) -> None:
    metadata = ";".join(
        f"{path.relative_to(PKG)}:{hashlib.sha256(path.read_bytes()).hexdigest()}"
        for path in bindings
    )
    parts.append(f'<metadata id="source-bindings">{html.escape(metadata)}</metadata>')
    parts.append("</svg>")
    (FIG / output).write_text("\n".join(parts) + "\n", encoding="utf-8")


def line_path(points: list[tuple[float, float]]) -> str:
    return " ".join(
        ("M" if index == 0 else "L") + f"{x:.2f},{y:.2f}"
        for index, (x, y) in enumerate(points)
    )


def dated_segments(
    selected: list[dict[str, str]],
    px: object,
    py: object,
    field: str,
    maximum_gap_days: int = 7,
) -> list[list[tuple[float, float]]]:
    segments: list[list[tuple[float, float]]] = []
    current: list[tuple[float, float]] = []
    previous = None
    for row in selected:
        current_date = date.fromisoformat(row["date"])
        if previous is not None and (current_date - previous).days > maximum_gap_days:
            if current:
                segments.append(current)
            current = []
        current.append((px(row["date"]), py(float(row[field]))))
        previous = current_date
    if current:
        segments.append(current)
    return segments


def forcing_compatibility() -> None:
    source = ART / "forcing-diagnostics.csv"
    data = rows(source)
    parts = begin(
        "Contract-defined VPD from frozen Southern Hemisphere forcing",
        "Two panels show reconstructed daily vapor pressure deficit. Three Alerce days fall below the contract's zero lower bound and are highlighted in red.",
    )
    start_day = date(2022, 1, 1).toordinal()
    end_day = date(2026, 7, 24).toordinal()
    for index, (site, label) in enumerate(
        (("SH-DB-BEZA", "Beza Mahafaly"), ("SH-EN-ALERCE", "Alerce Costero"))
    ):
        x, y, w, h = 95, 90 + index * 315, 1080, 245
        selected = [row for row in data if row["site_id"] == site]
        vmax = max(float(row["reconstructed_vpd_pa"]) for row in selected)
        py = lambda value: y + h - (value + 100.0) / (vmax + 100.0) * h
        px = lambda day: x + (date.fromisoformat(day).toordinal() - start_day) / (end_day - start_day) * w
        parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" class="panel"/>')
        parts.append(f'<text x="{x+8}" y="{y+19}" class="label">{label}</text>')
        zero = py(0.0)
        parts.append(f'<line x1="{x}" y1="{zero}" x2="{x+w}" y2="{zero}" stroke="{RED}" stroke-width="1.5"/>')
        for tick in (0.0, vmax / 2.0, vmax):
            yy = py(tick)
            parts.append(
                f'<text x="{x-10}" y="{yy+4}" text-anchor="end" class="small">{tick:.0f}</text>'
            )
        parts.append(
            f'<text x="{x+w-8}" y="{zero-6}" text-anchor="end" class="small" fill="{RED}">0 Pa contract boundary</text>'
        )
        points = [(px(row["date"]), py(float(row["reconstructed_vpd_pa"]))) for row in selected]
        parts.append(f'<path d="{line_path(points)}" fill="none" stroke="{BLUE}" stroke-width="1.1"/>')
        for row in selected:
            value = float(row["reconstructed_vpd_pa"])
            if value < 0.0:
                parts.append(f'<circle cx="{px(row["date"]):.2f}" cy="{py(value):.2f}" r="6" fill="{RED}"/>')
        for year in range(2022, 2027):
            xx = px(f"{year}-01-01")
            parts.append(f'<text x="{xx}" y="{y+h+20}" text-anchor="middle" class="small">{year}</text>')
    parts.append('<text x="25" y="390" transform="rotate(-90 25 390)" class="label">VPD (Pa; site-specific panel scale)</text>')
    finish(parts, "cal07-forcing-vpd-compatibility.svg", (source,))


def negative_days() -> None:
    source = ART / "negative-vpd-days.csv"
    data = rows(source)
    parts = begin(
        "Temperature operands on the three inadmissible Alerce days",
        "Grouped markers compare daily minimum, maximum, and dew-point temperatures on the three dates that reconstruct to negative VPD.",
    )
    x0, y0, w, h = 130, 100, 1020, 530
    parts.append(f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" class="panel"/>')
    ymin, ymax = 0.0, 14.0
    py = lambda value: y0 + h - (value - ymin) / (ymax - ymin) * h
    for value in range(0, 15, 2):
        yy = py(value)
        parts.append(f'<line x1="{x0}" y1="{yy}" x2="{x0+w}" y2="{yy}" class="grid"/>')
        parts.append(f'<text x="{x0-12}" y="{yy+4}" text-anchor="end" class="small">{value}</text>')
    series = (("tmin_c", "Tmin", BLUE, -18), ("tmax_c", "Tmax", ORANGE, 0), ("tdew_c", "Tdew", GREEN, 18))
    for index, row in enumerate(data):
        cx = x0 + (index + 0.5) * w / len(data)
        for field, _, color, offset in series:
            parts.append(f'<circle cx="{cx+offset}" cy="{py(float(row[field])):.2f}" r="7" fill="{color}"/>')
        parts.append(f'<text x="{cx}" y="{y0+h+27}" text-anchor="middle" class="label">{row["date"]}</text>')
        parts.append(f'<text x="{cx}" y="{y0+h+46}" text-anchor="middle" class="small">{float(row["reconstructed_vpd_pa"]):.2f} Pa</text>')
    for index, (_, label, color, _) in enumerate(series):
        xx = 430 + index * 155
        parts.append(f'<circle cx="{xx}" cy="710" r="6" fill="{color}"/>')
        parts.append(f'<text x="{xx+12}" y="715" class="small">{label}</text>')
    parts.append('<text x="35" y="390" transform="rotate(-90 35 390)" class="label">temperature (°C)</text>')
    finish(parts, "cal07-negative-vpd-operands.svg", (source,))


def observational_lanes() -> None:
    source = INPUT / "observations.csv"
    data = rows(source)
    parts = begin(
        "Admitted PhenoCam greenness observations in the two CAL-07 lanes",
        "Two panels show every quality-admitted raw GCC90 observation from the deciduous Beza and evergreen Alerce camera regions.",
    )
    start_day = min(date.fromisoformat(row["date"]).toordinal() for row in data)
    end_day = max(date.fromisoformat(row["date"]).toordinal() for row in data)
    for index, (site, label, color) in enumerate(
        (("SH-DB-BEZA", "Beza Mahafaly — DB_1000", GREEN), ("SH-EN-ALERCE", "Alerce Costero — EN_1000", BLUE))
    ):
        selected = [row for row in data if row["site_id"] == site]
        values = [float(row["gcc_90"]) for row in selected]
        low = math.floor(min(values) * 100) / 100
        high = math.ceil(max(values) * 100) / 100
        x, y, w, h = 95, 90 + index * 315, 1080, 245
        px = lambda day: x + (date.fromisoformat(day).toordinal() - start_day) / (end_day - start_day) * w
        py = lambda value: y + h - (value - low) / (high - low) * h
        parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" class="panel"/>')
        parts.append(f'<text x="{x+8}" y="{y+19}" class="label">{label}</text>')
        for fraction in (0.0, 0.5, 1.0):
            value = low + fraction * (high - low)
            yy = py(value)
            parts.append(f'<line x1="{x}" y1="{yy}" x2="{x+w}" y2="{yy}" class="grid"/>')
            parts.append(f'<text x="{x-10}" y="{yy+4}" text-anchor="end" class="small">{value:.2f}</text>')
        for points in dated_segments(selected, px, py, "gcc_90"):
            parts.append(
                f'<path d="{line_path(points)}" fill="none" stroke="{color}" stroke-width="1.3"/>'
            )
        for year in range(2023, 2027):
            xx = px(f"{year}-01-01")
            parts.append(f'<text x="{xx}" y="{y+h+20}" text-anchor="middle" class="small">{year}</text>')
    parts.append('<text x="25" y="390" transform="rotate(-90 25 390)" class="label">raw GCC90 (panel-specific scale)</text>')
    finish(parts, "cal07-observational-lanes.svg", (source,))


def hold_boundary() -> None:
    parts = begin(
        "CAL-07 disposition after fail-closed forcing admission",
        "A status plot shows retained source evidence, the failed forcing gate, and the result cells that cannot be evaluated.",
    )
    cells = [
        ("Two independent SH site assignments", "RETAINED"),
        ("Provisional camera greenness records", "RETAINED"),
        ("Contract-compatible daily forcing", "FAILED"),
        ("37-member GSI/canopy execution", "BLOCKED"),
        ("Timing and relative-shape evaluation", "NOT EVALUATED"),
        ("Absolute amplitude / evergreen floor", "NOT EVALUATED"),
        ("Phase-transformed real consumers", "NOT EVALUATED"),
        ("Roadmap Order 7 advancement", "WITHHELD"),
    ]
    positions = {"FAILED": 620, "BLOCKED": 760, "NOT EVALUATED": 900, "WITHHELD": 1040, "RETAINED": 1140}
    colors = {"FAILED": RED, "BLOCKED": RED, "NOT EVALUATED": GRAY, "WITHHELD": ORANGE, "RETAINED": GREEN}
    for index, (label, status) in enumerate(cells):
        yy = 115 + index * 70
        parts.append(f'<line x1="500" y1="{yy}" x2="1160" y2="{yy}" class="grid"/>')
        parts.append(f'<text x="50" y="{yy+5}" class="label">{html.escape(label)}</text>')
        parts.append(f'<circle cx="{positions[status]}" cy="{yy}" r="9" fill="{colors[status]}"/>')
        parts.append(f'<text x="{positions[status]-14}" y="{yy-14}" text-anchor="middle" class="small">{status}</text>')
    finish(
        parts,
        "cal07-hold-evidence-boundaries.svg",
        (ART / "negative-vpd-days.csv", ART / "observation-source-summary.csv"),
    )


def diagnostic_manifest() -> None:
    paths = [
        ART / "forcing-diagnostics.csv",
        ART / "negative-vpd-days.csv",
        ART / "observation-source-summary.csv",
        *sorted(FIG.glob("*.svg")),
        *sorted(FIG.glob("*.md")),
    ]
    with (ART / "diagnostic-manifest.csv").open(
        "w", newline="", encoding="utf-8"
    ) as stream:
        writer = csv.DictWriter(stream, fieldnames=("path", "sha256", "bytes"))
        writer.writeheader()
        for source in paths:
            writer.writerow(
                {
                    "path": source.relative_to(PKG),
                    "sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
                    "bytes": source.stat().st_size,
                }
            )


def main() -> None:
    FIG.mkdir(exist_ok=True)
    forcing_compatibility()
    negative_days()
    observational_lanes()
    hold_boundary()
    diagnostic_manifest()


if __name__ == "__main__":
    main()
