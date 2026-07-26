#!/usr/bin/env python3
"""Rebuild CAL-02 report-target scores from normalized result objects."""

from __future__ import annotations

import csv
import argparse
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
PACKAGE = REPO / "docs/work-packages/20260726-canopy-cal-02-elliot-reproduction-001/artifacts"
TARGETS = REPO / "docs/work-packages/20260726-canopy-cal-01-source-target-ledger-001/artifacts/target-ledger.csv"
LENGTH = {"HUBBARD_BROOK": 251.8, "SANTEE": 300.0}


def read_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def arm_for(site: str, reported_source: str) -> str:
    constant = "constant" in reported_source.lower()
    if site == "HUBBARD_BROOK":
        return "hubbard_constant" if constant else "hubbard_hardwood_095"
    return "santee_constant" if constant else "santee_mixed"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--artifact-root", type=Path, default=PACKAGE)
    args = parser.parse_args()
    package = args.artifact_root.resolve()
    equilibrium = {row["arm_id"]: row for row in read_rows(package / "equilibrium-results.csv")}
    annual = read_rows(package / "annual-results.csv")
    recurrence = {
        (row["arm_id"], row["surface"], int(row["recurrence_years"])): row
        for row in read_rows(package / "return-period-results.csv")
    }
    output = []
    process_columns = {
        "equilibrium_live_biomass": "mean_year_end_live_biomass_kg_m2",
        "equilibrium_current_residue": "mean_year_end_current_residue_kg_m2",
        "equilibrium_previous_residue": "mean_year_end_previous_residue_kg_m2",
        "equilibrium_old_residue": "mean_year_end_old_residue_kg_m2",
    }
    for target in read_rows(TARGETS):
        target_id = target["target_id"]
        if target_id.startswith(("HB-OUT-", "SEF-OUT-")):
            number = int(target_id.rsplit("-", 1)[1])
            if number > 10 or number in (7, 8) or not target["value"]:
                continue
            arm = arm_for(target["site"], target["reported_source"])
            if target["quantity"] in process_columns:
                if target["site"] == "SANTEE":
                    years_31_40 = [
                        row
                        for row in annual
                        if row["arm_id"] == arm and 31 <= int(row["year"]) <= 40
                    ]
                    annual_columns = {
                        "equilibrium_live_biomass": "live_biomass_year_end_kg_m2",
                        "equilibrium_current_residue": "current_flat_residue_year_end_kg_m2",
                        "equilibrium_previous_residue": "previous_flat_residue_year_end_kg_m2",
                        "equilibrium_old_residue": "old_flat_residue_year_end_kg_m2",
                    }
                    value = sum(
                        float(row[annual_columns[target["quantity"]]]) for row in years_31_40
                    ) / 10
                else:
                    value = float(equilibrium[arm][process_columns[target["quantity"]]])
                tolerance = max(0.25, 0.02 * float(target["value"]))
            elif target["quantity"] == "surface_runoff":
                rows = [row for row in annual if row["arm_id"] == arm]
                value = sum(float(row["annual_runoff_mm"]) for row in rows) / 100
                tolerance = max(1.0, 0.02 * float(target["value"]))
            elif target["quantity"] == "sediment_delivery":
                rows = [row for row in annual if row["arm_id"] == arm]
                value = (
                    sum(float(row["annual_sediment_delivery_kg_m"]) for row in rows)
                    / 100
                    * 10000
                    / LENGTH[target["site"]]
                )
                tolerance = max(1.0, 0.02 * float(target["value"]))
            else:
                continue
        elif target_id.startswith(("HB-RUN-RP-", "SEF-RUN-RP-", "HB-PEAK-RP-", "SEF-PEAK-RP-")):
            if "hill_streamflow" in target["quantity"]:
                continue
            arm = arm_for(target["site"], target["reported_source"])
            recurrence_years = int(target["temporal_basis"].split("-", 1)[0])
            surface = (
                "peak_hillslope_runoff_rate"
                if "peak_runoff_rate" in target["quantity"]
                else "daily_hillslope_surface_runoff"
            )
            value = float(recurrence[(arm, surface, recurrence_years)]["return_level"])
            tolerance = max(1.0, 0.05 * float(target["value"]))
        else:
            continue
        reported = float(target["value"])
        output.append(
            {
                "target_id": target_id,
                "arm_id": arm,
                "quantity": target["quantity"],
                "reported_value": reported,
                "reconstructed_value": value,
                "unit": target["unit"],
                "tolerance": tolerance,
                "classification": "PASS_BOUNDED" if abs(value - reported) <= tolerance else "CONTRADICTED",
            }
        )
    with (package / "report-comparison.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(
            stream, fieldnames=list(output[0]), lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(output)
    labels = ["HB constant", "HB 0.95", "HB 0.92", "SE constant", "SE mixed"]
    arms = ["hubbard_constant", "hubbard_hardwood_095", "hubbard_hardwood_092", "santee_constant", "santee_mixed"]
    bars = []
    for index, (label, arm) in enumerate(zip(labels, arms)):
        x = 95 + index * 130
        live = float(equilibrium[arm]["mean_year_end_live_biomass_kg_m2"])
        residue = float(equilibrium[arm]["mean_year_end_flat_residue_kg_m2"])
        bars.extend(
            [
                f'<rect x="{x}" y="{300-live*12.5:.2f}" width="42" height="{live*12.5:.2f}" fill="#3977b8"/>',
                f'<rect x="{x+45}" y="{300-residue*12.5:.2f}" width="42" height="{residue*12.5:.2f}" fill="#c87935"/>',
                f'<text x="{x+43}" y="320" text-anchor="middle" font-family="sans-serif" font-size="10">{label}</text>',
            ]
        )
    svg = (
        '<svg xmlns="http://www.w3.org/2000/svg" width="760" height="360" viewBox="0 0 760 360">\n'
        '<rect width="760" height="360" fill="white"/>\n'
        '<text x="380" y="24" text-anchor="middle" font-family="sans-serif" font-size="18">'
        'Years 91–100 mean year-end stocks</text>\n'
        '<line x1="70" y1="300" x2="735" y2="300" stroke="black"/>\n'
        '<line x1="70" y1="45" x2="70" y2="300" stroke="black"/>\n'
        + "\n".join(bars)
        + '\n<text x="18" y="175" transform="rotate(-90 18 175)" font-family="sans-serif" font-size="12">kg/m²</text>\n'
        '</svg>\n'
    )
    (package / "figures").mkdir(parents=True, exist_ok=True)
    (package / "figures/equilibrium-stocks.svg").write_text(svg, encoding="utf-8")


if __name__ == "__main__":
    main()
