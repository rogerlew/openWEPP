#!/usr/bin/env python3
"""Compare prior and corrected EROD16 diagnostic populations."""

from __future__ import annotations

import csv
import statistics
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LOGS = ROOT / "artifacts" / "logs"
OUT_CSV = ROOT / "artifacts" / "storm-partition.csv"
OUT_MD = ROOT / "artifacts" / "diagnostic-partition.md"
BOUND = 5.0e-3


@dataclass(frozen=True)
class Row:
    day: int
    runoff_m: float
    peakro_m_s: float
    duration_s: float
    shear_pa: float
    tcend_kg_s_m: float
    detinr_kg_s_m2: float
    flux_ratio: float


def parse(path: Path) -> dict[int, Row]:
    rows: dict[int, Row] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.startswith("EB04W2C_SOLVE,"):
            continue
        fields = [value.strip() for value in line.split(",")]
        row = Row(
            day=int(fields[1]),
            runoff_m=float(fields[2]),
            peakro_m_s=float(fields[3]),
            duration_s=float(fields[4]),
            shear_pa=float(fields[5]),
            tcend_kg_s_m=float(fields[6]),
            detinr_kg_s_m2=float(fields[7]),
            flux_ratio=float(fields[8]),
        )
        rows[row.day] = row
    return rows


def status(row: Row | None) -> str:
    if row is None:
        return "absent"
    return "refused" if row.flux_ratio > BOUND else "clean"


def median(values: list[float]) -> str:
    return "n/a" if not values else f"{statistics.median(values):.6g}"


def main() -> None:
    prior = parse(LOGS / "04-prior-unbounded-diagnostic.log")
    corrected = parse(LOGS / "03-corrected-unbounded-diagnostic.log")
    days = sorted(set(prior) | set(corrected))

    counts: dict[str, int] = {}
    with OUT_CSV.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.writer(handle, lineterminator="\n")
        writer.writerow(
            [
                "sim_day_index",
                "partition",
                "prior_status",
                "corrected_status",
                "prior_runoff_m",
                "corrected_runoff_m",
                "prior_peakro_m_s",
                "corrected_peakro_m_s",
                "prior_flux_ratio",
                "corrected_flux_ratio",
            ]
        )
        for day in days:
            old = prior.get(day)
            new = corrected.get(day)
            old_status = status(old)
            new_status = status(new)
            partition = f"{old_status}_to_{new_status}"
            counts[partition] = counts.get(partition, 0) + 1
            writer.writerow(
                [
                    day,
                    partition,
                    old_status,
                    new_status,
                    "" if old is None else f"{old.runoff_m:.17e}",
                    "" if new is None else f"{new.runoff_m:.17e}",
                    "" if old is None else f"{old.peakro_m_s:.17e}",
                    "" if new is None else f"{new.peakro_m_s:.17e}",
                    "" if old is None else f"{old.flux_ratio:.17e}",
                    "" if new is None else f"{new.flux_ratio:.17e}",
                ]
            )

    new_refused = [row for row in corrected.values() if row.flux_ratio > BOUND]
    old_refused = [row for row in prior.values() if row.flux_ratio > BOUND]
    text = [
        "# Corrected-Winter EROD16 Diagnostic Partition",
        "",
        "Evidence mode: **Ran + independently reconstructed from retained logs**.",
        "",
        "The hard `5e-3` trapezoid-versus-RK4 diagnostic is reapplied here to",
        "unbounded diagnostic solves; no acceptance threshold was changed in the",
        "production result.",
        "",
        "| Transition | Count |",
        "|---|---:|",
    ]
    for key in sorted(counts):
        text.append(f"| `{key}` | {counts[key]} |")
    text.extend(
        [
            "",
            f"Prior population: `{len(prior)}` storms, `{len(old_refused)}` refusals.",
            f"Corrected population: `{len(corrected)}` storms, `{len(new_refused)}` refusals.",
            "",
            "| Refusal cohort | Median runoff (m) | Median peak runoff (m s^-1) | Median diagnostic ratio |",
            "|---|---:|---:|---:|",
            f"| prior | {median([r.runoff_m for r in old_refused])} | {median([r.peakro_m_s for r in old_refused])} | {median([r.flux_ratio for r in old_refused])} |",
            f"| corrected | {median([r.runoff_m for r in new_refused])} | {median([r.peakro_m_s for r in new_refused])} | {median([r.flux_ratio for r in new_refused])} |",
            "",
            "The exact day-level operands and classifications are retained in",
            "`storm-partition.csv`.",
            "",
        ]
    )
    OUT_MD.write_text("\n".join(text), encoding="utf-8")


if __name__ == "__main__":
    main()
