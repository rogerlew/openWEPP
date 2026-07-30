#!/usr/bin/env python3
"""Generate and validate analytical EB-01A figures."""

from __future__ import annotations

import csv
import math
from pathlib import Path
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
ART = ROOT / "artifacts"
FIG = ART / "figures"
SIGMA = 5.670374419e-8


def svg_start(title: str, desc: str, width: int = 960, height: int = 560) -> list[str]:
    return [
        f'<svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 {width} {height}">',
        f"<title>{title}</title>",
        f"<desc>{desc}</desc>",
        '<rect width="100%" height="100%" fill="#ffffff"/>',
        '<style>text{font-family:Arial,sans-serif;fill:#17212b}.axis{stroke:#35424f;stroke-width:1.5}.grid{stroke:#d9e0e6;stroke-width:1}.label{font-size:15px}.small{font-size:13px}.title{font-size:21px;font-weight:700}.legend{font-size:14px}</style>',
    ]


def sensitivity() -> None:
    lines = svg_start(
        "Canopy-air temperature proxy sensitivity",
        "Analytical longwave error versus canopy minus air temperature for five sky-view fractions.",
    )
    lines += [
        '<text x="480" y="34" text-anchor="middle" class="title">Air-temperature proxy error depends on view and temperature mismatch</text>',
    ]
    left, top, width, height = 92, 72, 780, 390
    for value in range(-60, 81, 20):
        y = top + height - (value + 60) / 140 * height
        lines += [f'<line x1="{left}" y1="{y:.2f}" x2="{left+width}" y2="{y:.2f}" class="grid"/>',
                  f'<text x="{left-12}" y="{y+5:.2f}" text-anchor="end" class="small">{value}</text>']
    for value in range(-8, 13, 2):
        x = left + (value + 8) / 20 * width
        lines += [f'<line x1="{x:.2f}" y1="{top}" x2="{x:.2f}" y2="{top+height}" class="grid"/>',
                  f'<text x="{x:.2f}" y="{top+height+25}" text-anchor="middle" class="small">{value}</text>']
    lines += [
        f'<line x1="{left}" y1="{top+height}" x2="{left+width}" y2="{top+height}" class="axis"/>',
        f'<line x1="{left}" y1="{top}" x2="{left}" y2="{top+height}" class="axis"/>',
        f'<text x="{left+width/2}" y="518" text-anchor="middle" class="label">Canopy temperature minus air temperature (°C)</text>',
        f'<text x="24" y="{top+height/2}" transform="rotate(-90 24 {top+height/2})" text-anchor="middle" class="label">Canopy-emission proxy error (W m⁻²)</text>',
    ]
    colors = ["#1b6ca8", "#287f5c", "#b57d16", "#c44e52", "#7a5195"]
    for idx, sky in enumerate([0.1, 0.3, 0.5, 0.7, 0.9]):
        points = []
        for step in range(101):
            delta = -8 + 20 * step / 100
            error = (1 - sky) * SIGMA * ((273.15 + delta) ** 4 - 273.15**4)
            x = left + (delta + 8) / 20 * width
            y = top + height - (error + 60) / 140 * height
            points.append(f"{x:.2f},{y:.2f}")
        lines.append(f'<polyline points="{" ".join(points)}" fill="none" stroke="{colors[idx]}" stroke-width="3"/>')
        ly = 88 + idx * 27
        lines += [f'<line x1="745" y1="{ly}" x2="775" y2="{ly}" stroke="{colors[idx]}" stroke-width="3"/>',
                  f'<text x="784" y="{ly+5}" class="legend">f_sky = {sky:.1f}</text>']
    lines.append("</svg>")
    (FIG / "eb01a-temperature-proxy-sensitivity.svg").write_text("\n".join(lines) + "\n")


def readiness() -> None:
    rows = [
        ("Air temperature", "available", "#287f5c"),
        ("Dewpoint / vapor pressure", "available", "#287f5c"),
        ("Cloud fraction / solar", "forcing exists; mapping", "#b57d16"),
        ("Atmospheric LW equation", "contract", "#b57d16"),
        ("Sky-view fraction", "derive from canopy state", "#b57d16"),
        ("Canopy emissivity", "contract", "#b57d16"),
        ("Canopy temperature rule", "conditional", "#b57d16"),
        ("Snow surface temperature", "bind active state", "#b57d16"),
    ]
    lines = svg_start(
        "EB-02 operand readiness",
        "Status of the eight operands needed by the selected stand-scale longwave formulation.",
        960,
        590,
    )
    lines += [
        '<text x="480" y="38" text-anchor="middle" class="title">The equation is resolved; the canopy-state mapping is the next contract step</text>',
        '<text x="75" y="74" class="label">Required operand</text>',
        '<text x="650" y="74" class="label">EB-02 status</text>',
    ]
    for idx, (name, status, color) in enumerate(rows):
        y = 105 + idx * 55
        lines += [
            f'<rect x="65" y="{y-25}" width="830" height="44" rx="7" fill="#f6f8fa" stroke="#d9e0e6"/>',
            f'<text x="82" y="{y+3}" class="label">{name}</text>',
            f'<rect x="642" y="{y-17}" width="220" height="29" rx="14" fill="{color}"/>',
            f'<text x="752" y="{y+3}" text-anchor="middle" style="font-family:Arial,sans-serif;font-size:14px;fill:#ffffff">{status}</text>',
        ]
    lines += [
        '<circle cx="78" cy="558" r="7" fill="#287f5c"/><text x="93" y="563" class="small">runtime operand exists</text>',
        '<circle cx="520" cy="558" r="7" fill="#b57d16"/><text x="535" y="563" class="small">contract or active-state binding prerequisite</text>',
        "</svg>",
    ]
    (FIG / "eb01a-operand-readiness.svg").write_text("\n".join(lines) + "\n")


def validate() -> None:
    for path in ART.glob("*.csv"):
        with path.open(newline="") as handle:
            rows = [row for row in csv.reader(handle) if row]
        if len(rows) < 2 or any(len(row) != len(rows[0]) for row in rows):
            raise SystemExit(f"invalid CSV: {path}")
    svgs = sorted(FIG.glob("*.svg"))
    if len(svgs) != 2:
        raise SystemExit(f"expected 2 SVGs, found {len(svgs)}")
    for path in svgs:
        root = ET.parse(path).getroot()
        ns = {"s": "http://www.w3.org/2000/svg"}
        if root.attrib.get("role") != "img":
            raise SystemExit(f"missing image role: {path}")
        if len(root.findall("s:title", ns)) != 1 or len(root.findall("s:desc", ns)) != 1:
            raise SystemExit(f"missing title/desc: {path}")
        if not path.with_suffix(".md").exists():
            raise SystemExit(f"missing sidecar: {path}")


def main() -> None:
    FIG.mkdir(parents=True, exist_ok=True)
    sensitivity()
    readiness()
    validate()
    print("PASS: generated and validated EB-01A artifacts")


if __name__ == "__main__":
    main()
