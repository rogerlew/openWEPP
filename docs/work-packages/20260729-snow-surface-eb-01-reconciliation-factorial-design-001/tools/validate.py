#!/usr/bin/env python3
"""Validate EB-01 generated artifacts and package-local links."""

from __future__ import annotations

import csv
import re
import subprocess
import sys
import xml.etree.ElementTree as ET
from pathlib import Path


PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"


def main() -> None:
    subprocess.run([sys.executable, str(Path(__file__).with_name("generate.py")), "--check"], check=True)
    csv_paths = sorted(ARTIFACTS.glob("*.csv"))
    if len(csv_paths) != 14:
        raise SystemExit(f"expected 14 CSV artifacts, found {len(csv_paths)}")
    for path in csv_paths:
        with path.open(newline="") as stream:
            rows = list(csv.reader(stream))
        if len(rows) < 2 or len(set(map(len, rows))) != 1:
            raise SystemExit(f"invalid CSV shape: {path.name}")
    svgs = sorted((ARTIFACTS / "figures").glob("*.svg"))
    sidecars = sorted((ARTIFACTS / "figures").glob("*.md"))
    if {p.stem for p in svgs} != {p.stem for p in sidecars}:
        raise SystemExit("figure/sidecar stem mismatch")
    ns = {"svg": "http://www.w3.org/2000/svg"}
    for path in svgs:
        root = ET.parse(path).getroot()
        if root.attrib.get("role") != "img":
            raise SystemExit(f"missing role=img: {path.name}")
        if len(root.findall("svg:title", ns)) != 1 or len(root.findall("svg:desc", ns)) != 1:
            raise SystemExit(f"invalid title/desc count: {path.name}")
    link_pattern = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")
    for md in [PACKAGE / "package.md", *PACKAGE.rglob("*.md")]:
        for target in link_pattern.findall(md.read_text()):
            target = target.split("#", 1)[0]
            if not target or "://" in target or target.startswith("/"):
                continue
            if not (md.parent / target).resolve().exists():
                raise SystemExit(f"broken local link in {md.relative_to(PACKAGE)}: {target}")
    print(f"PASS: {len(csv_paths)} CSVs, {len(svgs)} SVGs, sidecars, links, determinism")


if __name__ == "__main__":
    main()
