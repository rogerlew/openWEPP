#!/usr/bin/env python3
"""Validate the exact fail-closed CAL-07 disposition."""

from __future__ import annotations

import csv
import hashlib
import math
import xml.etree.ElementTree as ET
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
ART = PKG / "artifacts"
FIG = ART / "figures"


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def main() -> None:
    for item in rows(ART / "source-manifest.csv"):
        path = PKG / item["path"]
        assert path.stat().st_size == int(item["bytes"]), path
        assert hashlib.sha256(path.read_bytes()).hexdigest() == item["sha256"], path
    custody = rows(ART / "ensemble-custody.csv")
    assert len(custody) == 37
    assert len({row["candidate_id"] for row in custody}) == 37
    predecessor = (
        ROOT
        / "docs/work-packages"
        / "20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001"
        / "artifacts"
    )
    accepted_digest = hashlib.sha256(
        (predecessor / "accepted-calibration-ensemble.csv").read_bytes()
    ).hexdigest()
    candidate_digest = hashlib.sha256(
        (predecessor / "candidate-configurations.csv").read_bytes()
    ).hexdigest()
    assert {row["accepted_ledger_sha256"] for row in custody} == {accepted_digest}
    assert {row["candidate_table_sha256"] for row in custody} == {candidate_digest}

    diagnostics = rows(ART / "forcing-diagnostics.csv")
    negatives = rows(ART / "negative-vpd-days.csv")
    assert len(diagnostics) == 3332
    assert len(negatives) == 3
    assert [(row["site_id"], row["date"]) for row in negatives] == [
        ("SH-EN-ALERCE", "2022-07-22"),
        ("SH-EN-ALERCE", "2022-09-15"),
        ("SH-EN-ALERCE", "2025-09-09"),
    ]
    assert all(float(row["reconstructed_vpd_pa"]) < 0.0 for row in negatives)
    assert all(
        math.isfinite(float(row["reconstructed_vpd_pa"])) for row in diagnostics
    )
    assert sum(row["contract_status"] == "FAIL_NEGATIVE" for row in diagnostics) == 3

    observations = {
        row["site_id"]: row for row in rows(ART / "observation-source-summary.csv")
    }
    assert int(observations["SH-DB-BEZA"]["admitted_camera_days"]) == 934
    assert int(observations["SH-EN-ALERCE"]["admitted_camera_days"]) == 925

    for forbidden in (
        "daily-kernel-output.csv",
        "gate-results.csv",
        "ensemble-daily.csv",
        "shape-scores.csv",
        "transition-residuals.csv",
        "verdict-matrix.csv",
    ):
        assert not (ART / forbidden).exists(), f"partial result exists: {forbidden}"

    expected = {
        "cal07-forcing-vpd-compatibility",
        "cal07-negative-vpd-operands",
        "cal07-observational-lanes",
        "cal07-hold-evidence-boundaries",
    }
    assert {path.stem for path in FIG.glob("*.svg")} == expected
    assert {path.stem for path in FIG.glob("*.md")} == expected
    for stem in expected:
        root = ET.parse(FIG / f"{stem}.svg").getroot()
        assert root.attrib.get("role") == "img"
        assert root.attrib.get("aria-labelledby") == "title desc"
        tags = {element.tag.rsplit("}", 1)[-1] for element in root.iter()}
        assert {"title", "desc", "metadata"} <= tags
        sidecar = (FIG / f"{stem}.md").read_text(encoding="utf-8")
        for heading in (
            "## Caption",
            "## How to read it",
            "## Plain-language takeaway",
            "## Methods and source binding",
            "## Limitations",
            "## Accessibility",
        ):
            assert heading in sidecar, (stem, heading)
    diagnostic_manifest = rows(ART / "diagnostic-manifest.csv")
    assert len(diagnostic_manifest) == 3 + 2 * len(expected)
    for item in diagnostic_manifest:
        path = PKG / item["path"]
        assert path.stat().st_size == int(item["bytes"]), path
        assert hashlib.sha256(path.read_bytes()).hexdigest() == item["sha256"], path

    package = (PKG / "package.md").read_text(encoding="utf-8")
    disposition = (ART / "final-disposition.md").read_text(encoding="utf-8")
    assert "Status: `hold / forcing authority incompatible`" in package
    assert "HOLD / FORCING AUTHORITY INCOMPATIBLE / NO CANOPY RESULT" in disposition
    assert "GO FOR RESULT-BEARING EXECUTION" in (
        ART / "prospective-review-a.md"
    ).read_text(encoding="utf-8")
    assert "GO FOR BOUNDED RESULT EXECUTION" in (
        ART / "prospective-review-b.md"
    ).read_text(encoding="utf-8")
    print("CAL-07 HOLD validation PASS: 3 negative VPD days; no partial canopy result")


if __name__ == "__main__":
    main()
