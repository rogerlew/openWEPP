#!/usr/bin/env python3
"""Characterize the existing Dec_* residue-mass producer from source fixtures."""

from __future__ import annotations

import csv
import json
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[4]
FIXTURE = REPO_ROOT / "tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/p10.man"
OUT_DIR = Path(__file__).resolve().parent
CSV_OUT = OUT_DIR / "phase0_residue_mass_monthly.csv"
JSON_OUT = OUT_DIR / "phase0_residue_mass_summary.json"


def main() -> None:
    lines = FIXTURE.read_text(encoding="utf-8").splitlines()
    sim_years = int(lines[2].split("#", 1)[0].strip())
    plant_name = lines[8].strip()
    residue_fields = [float(value) for value in lines[17].split()]
    initial_terminal = [float(value) for value in lines[40].split()]

    oratea = residue_fields[0]
    orater = residue_fields[1]
    surface_seed = initial_terminal[1]
    root_seed = initial_terminal[0]

    rows = []
    for year in range(1, min(sim_years, 1) + 1):
        for month in range(1, 13):
            rows.append(
                {
                    "year": year,
                    "month": month,
                    "surface_residue_kg_m2": surface_seed,
                    "root_residue_kg_m2": root_seed,
                    "source": "fixture-static-no-input-no-decay",
                }
            )

    with CSV_OUT.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)

    summary = {
        "schema": "frost-residue-cover-phase0-characterization-v1",
        "fixture": str(FIXTURE.relative_to(REPO_ROOT)),
        "plant_name": plant_name,
        "sim_years": sim_years,
        "source_lines": {
            "sim_years": 3,
            "plant_name": 9,
            "residue_line": 18,
            "initial_terminal_seed_line": 41,
        },
        "parameters": {
            "oratea": oratea,
            "orater": orater,
            "initial_surface_residue_kg_m2": surface_seed,
            "initial_root_residue_kg_m2": root_seed,
        },
        "branch": "MASS-NOT-SEASONAL-NO-INPUT-ZERO-DECAY",
        "interpretation": (
            "The existing Dec_* fixture provides neither recurring litter input "
            "nor decomposition decay to the surface residue mass pool. The "
            "implementation must add the litter-input limb before wiring dynamic "
            "mass-to-depth-to-frost."
        ),
        "monthly_csv": str(CSV_OUT.relative_to(REPO_ROOT)),
    }
    JSON_OUT.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()

