#!/usr/bin/env python3
"""Validate frozen CAL-04 calibration/holdout roles and disjointness."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
WINDOWS = (
    ROOT
    / "docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001"
    / "artifacts/cal04-timing-windows.csv"
)


def main() -> int:
    with WINDOWS.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    calibration = [row for row in rows if row["role"] == "CALIBRATION"]
    holdout = [row for row in rows if row["role"] == "INDEPENDENT_HOLDOUT"]
    assert (len(rows), len(calibration), len(holdout)) == (1251, 932, 319)
    assert {row["source_object_id"] for row in calibration} == {
        "SRC-HB-PHENO-EDI-51-16"
    }
    assert {row["source_object_id"] for row in holdout} == {
        "SRC-HF-PHENO-HF003-V37"
    }
    calibration_ids = {row["record_id"] for row in calibration}
    holdout_ids = {row["record_id"] for row in holdout}
    assert calibration_ids.isdisjoint(holdout_ids)
    assert not any(
        row["year"] == "1992" and row["season"] == "FALL" for row in holdout
    )
    assert max(int(row["year"]) for row in calibration) <= 2024
    print(
        "PASS roles: 932 Hubbard calibration; 319 Harvard holdout; "
        "disjoint; Harvard fall 1992 absent"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
