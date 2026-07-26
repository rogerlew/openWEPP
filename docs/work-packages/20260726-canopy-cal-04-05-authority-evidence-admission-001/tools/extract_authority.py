#!/usr/bin/env python3
"""Build interval-censored CAL-04 calibration and holdout transitions."""

from __future__ import annotations

import argparse
import csv
from collections import defaultdict
from pathlib import Path

THRESHOLDS = {"SPRING": (("HALF_EXPANSION", 3.0, "up"),)}
FIELDS = [
    "record_id",
    "year",
    "season",
    "species",
    "site",
    "transition",
    "threshold_stage",
    "interval_start_date",
    "interval_start_doy",
    "interval_end_date",
    "interval_end_doy",
    "role",
    "source_object_id",
]


def crossed(previous: float, current: float, threshold: float, direction: str) -> bool:
    if direction == "up":
        return previous < threshold <= current
    if threshold == 4.0:
        return previous >= threshold > current
    return previous > threshold >= current


def extract_hubbard(source: Path) -> list[dict[str, object]]:
    groups: dict[tuple[int, str, str, str], list[dict[str, str]]] = defaultdict(list)
    with source.open(newline="", encoding="utf-8-sig") as stream:
        for row in csv.DictReader(stream):
            if row["Phenology_Stage"] in {"", "-9999"}:
                continue
            if row["SEASON"] not in THRESHOLDS:
                continue
            if row["SPECIES"] not in {"ACSA3", "BEAL2", "FAGR"}:
                continue
            # The protected Hubbard native fixture ends on 2024-12-31.
            if int(row["YEAR"]) > 2024:
                continue
            key = (int(row["YEAR"]), row["SEASON"], row["SPECIES"], row["SITE"])
            groups[key].append(row)

    output: list[dict[str, object]] = []
    for (year, season, species, site), rows in sorted(groups.items()):
        rows.sort(key=lambda row: int(row["DAY"]))
        for name, threshold, direction in THRESHOLDS[season]:
            for before, after in zip(rows, rows[1:]):
                if crossed(
                    float(before["Phenology_Stage"]),
                    float(after["Phenology_Stage"]),
                    threshold,
                    direction,
                ):
                    output.append(
                        {
                            "record_id": (
                                f"CAL0405-HB-{year}-{season}-{species}-{site}-{name}"
                            ),
                            "year": year,
                            "season": season,
                            "species": species,
                            "site": site,
                            "transition": name,
                            "threshold_stage": threshold,
                            "interval_start_date": before["DATE"],
                            "interval_start_doy": before["DAY"],
                            "interval_end_date": after["DATE"],
                            "interval_end_doy": after["DAY"],
                            "role": "CALIBRATION",
                            "source_object_id": "SRC-HB-PHENO-EDI-51-16",
                        }
                    )
                    break
    return output


def _percent_crossings(
    source: Path, season: str, variables: tuple[tuple[str, str], ...]
) -> list[dict[str, object]]:
    groups: dict[tuple[int, str], list[dict[str, str]]] = defaultdict(list)
    with source.open(newline="", encoding="utf-8-sig") as stream:
        for row in csv.DictReader(stream):
            year = int(row["date"][:4])
            groups[(year, row["tree.id"])].append(row)

    output: list[dict[str, object]] = []
    for (year, tree_id), rows in sorted(groups.items()):
        # The EML explicitly says that no fall campaign occurred in 1992.
        # Sparse raw values are retained in the source object but excluded
        # conservatively from authority extraction.
        if season == "FALL" and year == 1992:
            continue
        rows.sort(key=lambda row: int(row["doy"]))
        species = tree_id.split("-")[0]
        if species not in {"ACSA", "BEAL", "FAGR"}:
            continue
        for variable, transition in variables:
            valid = [row for row in rows if row[variable] not in {"", "NA"}]
            for before, after in zip(valid, valid[1:]):
                if float(before[variable]) < 50.0 <= float(after[variable]):
                    output.append(
                        {
                            "record_id": (
                                f"CAL0405-HF-{year}-{season}-{tree_id}-{transition}"
                            ),
                            "year": year,
                            "season": season,
                            "species": species,
                            "site": "HARVARD_FOREST",
                            "transition": transition,
                            "threshold_stage": "50_percent",
                            "interval_start_date": before["date"],
                            "interval_start_doy": before["doy"],
                            "interval_end_date": after["date"],
                            "interval_end_doy": after["doy"],
                            "role": "INDEPENDENT_HOLDOUT",
                            "source_object_id": "SRC-HF-PHENO-HF003-V37",
                        }
                    )
                    break
    return output


def extract_harvard(spring: Path, fall: Path) -> list[dict[str, object]]:
    del spring
    return _percent_crossings(
        fall,
        "FALL",
        (("lfall", "LEAF_FALL"),),
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("hubbard_source", type=Path)
    parser.add_argument("harvard_spring_source", type=Path)
    parser.add_argument("harvard_fall_source", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    rows = extract_hubbard(args.hubbard_source)
    rows.extend(extract_harvard(args.harvard_spring_source, args.harvard_fall_source))
    if not rows:
        raise SystemExit("no complete transition intervals found")
    roles = {row["role"] for row in rows}
    if roles != {"CALIBRATION", "INDEPENDENT_HOLDOUT"}:
        raise SystemExit(f"incomplete partition roles: {roles}")
    with args.output.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {len(rows)} interval-censored calibration/holdout transitions")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
