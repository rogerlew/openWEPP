#!/usr/bin/env python3
"""Validate CAL-07E literature-review artifacts."""

from __future__ import annotations

import csv
from datetime import date
import hashlib
from pathlib import Path
import sys
import xml.etree.ElementTree as ET


ROOT = Path(__file__).resolve().parents[1]
ARTIFACTS = ROOT / "artifacts"


def read_csv(name: str) -> list[dict[str, str]]:
    path = ARTIFACTS / name
    with path.open(encoding="utf-8", newline="") as stream:
        rows = list(csv.DictReader(stream))
    if not rows:
        raise ValueError(f"{name}: no data rows")
    if any(None in row for row in rows):
        raise ValueError(f"{name}: malformed row")
    return rows


def main() -> int:
    sources = read_csv("source-register.csv")
    claims = read_csv("claim-evidence-matrix.csv")
    audit = read_csv("phenocam-transition-product-audit.csv")
    source_subset_path = (
        ROOT
        / "inputs"
        / "bezamahafaly_DB_1000_2024_2025_gcc_mean_gcc_90_transition_subset.csv"
    )
    source_subset_bytes = source_subset_path.read_bytes()
    subset_hash = hashlib.sha256(source_subset_bytes).hexdigest()
    if subset_hash != "ec9df6cec532d5a8bece7f4849a41bafb3b062a30dac6ca2afbc5203e49ec634":
        raise ValueError("retained PhenoCam transition subset hash mismatch")
    with source_subset_path.open(encoding="utf-8", newline="") as stream:
        source_subset = list(csv.DictReader(stream))
    if len(source_subset) != 8:
        raise ValueError("retained PhenoCam transition subset must have eight rows")

    source_ids = {row["source_id"] for row in sources}
    if len(source_ids) != len(sources):
        raise ValueError("source-register.csv: duplicate source_id")

    allowed_tiers = {
        "DIRECT_SITE_PRIMARY",
        "DIRECT_SITE_CONTEXT",
        "REGIONAL_PRIMARY",
        "METHOD_PRIMARY",
        "MECHANISM_ANALOGUE",
        "DISCOVERY_ONLY",
    }
    unexpected_tiers = {row["evidence_tier"] for row in sources} - allowed_tiers
    if unexpected_tiers:
        raise ValueError(f"unexpected evidence tiers: {sorted(unexpected_tiers)}")

    allowed_statuses = {
        "SUPPORTED_AT_SITE",
        "SUPPORTED_AS_CONTRIBUTOR",
        "PLAUSIBLE_FROM_ANALOGUE",
        "NOT_SUPPORTED",
        "UNRESOLVED",
        "ACQUISITION_NEEDED",
        "NOT_APPLICABLE",
    }
    for row in claims:
        refs = set(row["source_ids"].split("|"))
        missing = refs - source_ids
        if missing:
            raise ValueError(
                f"{row['claim_id']}: unknown source IDs {sorted(missing)}"
            )
        if row["evidence_status"] not in allowed_statuses:
            raise ValueError(
                f"{row['claim_id']}: invalid status {row['evidence_status']}"
            )

    expected_audit = {
        (year, direction, threshold)
        for year in {"2024", "2025"}
        for direction in {"rising", "falling"}
        for threshold in {"10", "25", "50"}
    }
    observed_audit = {
        (row["year"], row["direction"], row["threshold"]) for row in audit
    }
    if observed_audit != expected_audit:
        raise ValueError("transition audit does not cover the 12 expected rows")
    for row in audit:
        date_delta = (
            date.fromisoformat(row["gcc_90_date"])
            - date.fromisoformat(row["gcc_mean_date"])
        ).days
        if date_delta != int(row["gcc_90_minus_mean_days"]):
            raise ValueError(
                f"transition delta mismatch: {row['year']} "
                f"{row['direction']} T{row['threshold']}"
            )

    source_by_key = {
        (row["transition_50"][:4], row["direction"], row["gcc_value"]): row
        for row in source_subset
    }
    for row in audit:
        threshold = row["threshold"]
        mean_source = source_by_key[(row["year"], row["direction"], "gcc_mean")]
        p90_source = source_by_key[(row["year"], row["direction"], "gcc_90")]
        if row["gcc_mean_date"] != mean_source[f"transition_{threshold}"]:
            raise ValueError("audit gcc_mean date does not match retained source")
        if row["gcc_90_date"] != p90_source[f"transition_{threshold}"]:
            raise ValueError("audit gcc_90 date does not match retained source")
        for product, source_row in (
            ("gcc_mean", mean_source),
            ("gcc_90", p90_source),
        ):
            endpoints = sorted(
                (
                    source_row[f"transition_{threshold}_lower_ci"],
                    source_row[f"transition_{threshold}_upper_ci"],
                )
            )
            expected_ci = "..".join(endpoints)
            if row[f"{product}_ci"] != expected_ci:
                raise ValueError(
                    f"audit {product} CI does not match retained source: "
                    f"{row['year']} {row['direction']} T{threshold}"
                )

    figures = sorted((ARTIFACTS / "figures").glob("*.svg"))
    if not figures:
        raise ValueError("no SVG figures found")
    for figure in figures:
        ET.parse(figure)
        sidecar = figure.with_suffix(".md")
        if not sidecar.is_file():
            raise ValueError(f"missing figure sidecar: {sidecar.name}")
        sidecar_text = sidecar.read_text(encoding="utf-8")
        if figure.name not in sidecar_text:
            raise ValueError(f"sidecar does not embed figure: {sidecar.name}")
        for heading in ("## Caption", "## Ancillary information"):
            if heading not in sidecar_text:
                raise ValueError(f"{sidecar.name}: missing {heading}")

    required = {
        "literature-synthesis.md",
        "acquisition-needed.md",
        "authority-disposition.md",
        "source-register.csv",
        "claim-evidence-matrix.csv",
        "phenocam-transition-product-audit.csv",
    }
    missing_required = sorted(
        name for name in required if not (ARTIFACTS / name).is_file()
    )
    if missing_required:
        raise ValueError(f"missing required artifacts: {missing_required}")

    print(
        "CAL-07E validation PASS: "
        f"{len(sources)} sources, {len(claims)} claims, "
        f"{len(audit)} transition comparisons, {len(figures)} figure"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, ET.ParseError) as error:
        print(f"CAL-07E validation FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
