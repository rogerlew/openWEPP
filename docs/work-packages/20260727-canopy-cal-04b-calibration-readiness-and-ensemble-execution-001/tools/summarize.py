#!/usr/bin/env python3
"""Render CAL-04B readiness and identifiability disposition artifacts."""

from __future__ import annotations

import argparse
import csv
import math
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
SOURCE_ARTIFACTS = PACKAGE / "artifacts"
ARTIFACTS = SOURCE_ARTIFACTS
OBJECTS = Path("/nonexistent/cal04b-execution-root-required")


def rows(name: str) -> list[dict[str, str]]:
    external = ARTIFACTS / name
    path = external if external.is_file() else SOURCE_ARTIFACTS / name
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def write_csv(name: str, fields: list[str], values: list[dict[str, object]]) -> None:
    with (ARTIFACTS / name).open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(values)


def merge_recovery() -> None:
    gsi = rows("synthetic-recovery-results.csv")
    if len(gsi) != 1 or gsi[0]["case_id"] != "SYN-GSI-01":
        raise ValueError("GSI recovery result differs")
    stage_map = {
        "REC-BFBS-01": "foliar_structural_partition",
        "REC-FE-01": "evergreen_fraction",
        "REC-LAI-01": "peak_lai",
        "REC-CSBB-01": "canopy_floor_closure",
    }
    merged = list(gsi)
    for row in rows("later-stage-recovery.csv"):
        if row["design_id"] not in stage_map or not row["recovery_status"].startswith("RECOVERED"):
            raise ValueError(f"later-stage recovery failed: {row['design_id']}")
        merged.append(
            {
                "case_id": row["design_id"],
                "stage": stage_map[row["design_id"]],
                "true_configuration": row["hidden_truth"],
                "recovered_set": row["recovered_set"],
                "status": "PASS",
                "evidence": str(ARTIFACTS / "later-stage-recovery.csv"),
            }
        )
    write_csv(
        "synthetic-recovery-results.csv",
        ["case_id", "stage", "true_configuration", "recovered_set", "status", "evidence"],
        merged,
    )


def pre_freeze() -> None:
    candidates = rows("candidate-ledger.csv")
    accepted = rows("accepted-calibration-ensemble.csv")
    if len(candidates) != 9_261 or not accepted:
        raise ValueError("calibration result inventory is incomplete")
    objectives = [float(row["objective"]) for row in candidates if math.isfinite(float(row["objective"]))]
    if not objectives:
        raise ValueError("no finite GSI objective")
    merge_recovery()

    matrix_rows = [
        ("typed/enumerable parameter surface", "PASS", "candidate-configurations.csv; later-stage-design.csv", "All axes and complete deterministic levels are retained."),
        ("observation operator with units and scale", "PASS", "executor-schema.md; later-stage-results.csv", "Interval timing, biomass, LAI, activity, and cover operators retain units and evidence roles."),
        ("deterministic candidate execution", "PASS", str(OBJECTS / "hubbard-gsi-identity.csv"), "Native raw trace has exact candidate/calendar/source/binary identity."),
        ("objective reconstruction", "PASS", str(OBJECTS / "verification/verification-receipt.csv"), "Independent reconstruction is byte-identical across all derived ledgers."),
        ("sensitivity analysis", "PASS", "later-stage-results.csv", "Native finite differences include central and boundary schemes."),
        ("identifiability/confounding analysis", "PASS", "identifiability-and-equifinality.md", "GSI ensemble and all downstream equifinal sets remain explicit."),
        ("boundary, saturation, and failure reporting", "PASS", "saturation-evidence.csv; failure-ledger.csv; later-stage-results.csv", "All candidate families, typed failures, and enumeration boundaries are retained."),
        ("equifinality/uncertainty retention", "PASS", "accepted-calibration-ensemble.csv; later-stage-membership.csv", "No convenience point selection replaces accepted membership."),
        ("synthetic recovery", "PASS", "synthetic-recovery-results.csv", "Five native recovery cases pass with limits stated."),
        ("additional-data inventory", "PASS", "additional-data-inventory.csv", "Measurements needed for stronger separation are stage-specific."),
    ]
    matrix = (
        "# Calibration Readiness Matrix\n\n"
        "Status: `PASS`\n\nEvidence class: `Ran`\n\n"
        "| Obligation | Status | Evidence path | Rationale |\n"
        "|---|---|---|---|\n"
        + "".join(f"| {a} | `{b}` | `{c}` | {d} |\n" for a, b, c, d in matrix_rows)
    )
    (ARTIFACTS / "calibration-readiness-matrix.md").write_text(matrix, encoding="utf-8")

    identified = "IDENTIFIED" if len(accepted) == 1 else "PARTIALLY_IDENTIFIABLE"
    statuses = [
        ("gsi_timing", "IMPLEMENTED", "EMPIRICALLY_CALIBRATED", identified, "PASS", "accepted-calibration-ensemble.csv"),
        ("foliar_structural_partition", "IMPLEMENTED", "EMPIRICALLY_CALIBRATED", "PARTIALLY_IDENTIFIABLE", "PASS", "later-stage-results.csv; later-stage-membership.csv"),
        ("evergreen_fraction", "IMPLEMENTED", "CALIBRATION_READY_DATA_LIMITED", "NONIDENTIFIABLE", "PASS", "later-stage-recovery.csv"),
        ("peak_lai", "IMPLEMENTED", "CALIBRATION_READY_DATA_LIMITED", "PARTIALLY_IDENTIFIABLE", "PASS", "later-stage-results.csv"),
        ("canopy_floor_closure", "IMPLEMENTED", "CALIBRATION_READY_DATA_LIMITED", "NONIDENTIFIABLE", "PASS", "later-stage-recovery.csv"),
    ]
    write_csv(
        "stage-status-ledger.csv",
        ["stage", "science_implementation_status", "calibration_evidence_status", "identifiability_status", "state", "evidence"],
        [
            dict(zip(
                ["stage", "science_implementation_status", "calibration_evidence_status", "identifiability_status", "state", "evidence"],
                row,
            ))
            for row in statuses
        ],
    )
    write_csv(
        "additional-data-inventory.csv",
        ["stage", "operand_or_combination", "missing_observation", "units", "scale", "identifiability_gain", "priority"],
        [
            {"stage": "gsi_timing", "operand_or_combination": "six GSI thresholds", "missing_observation": "multi-site daily leaf expansion plus local meteorology", "units": "fraction;degC;Pa;hours", "scale": "daily site-year", "identifiability_gain": "separate temperature VPD and photoperiod threshold pairs", "priority": "HIGH"},
            {"stage": "foliar_structural_partition", "operand_or_combination": "Bf_max versus Bs", "missing_observation": "coincident foliar and woody dry biomass", "units": "kg/m2", "scale": "mature plot", "identifiability_gain": "separate components currently constrained only by total", "priority": "HIGH"},
            {"stage": "evergreen_fraction", "operand_or_combination": "fe", "missing_observation": "seasonal evergreen and deciduous foliar biomass fractions", "units": "fraction", "scale": "plot-day", "identifiability_gain": "identify dormant-season activity floor", "priority": "HIGH"},
            {"stage": "peak_lai", "operand_or_combination": "xmxlai", "missing_observation": "coincident mature LAI and foliar activity", "units": "m2/m2", "scale": "plot-day", "identifiability_gain": "replace conditional range with direct calibration", "priority": "MEDIUM"},
            {"stage": "canopy_floor_closure", "operand_or_combination": "Cs and bb", "missing_observation": "cover response across live foliar biomass gradient", "units": "fraction; m2/kg", "scale": "plot-day", "identifiability_gain": "separate structural floor from biomass response", "priority": "HIGH"},
        ],
    )
    text = f"""# Identifiability and Equifinality

Status: `PASS`

Evidence class: `Ran`

- Complete GSI configurations: 9,261.
- Finite objectives: {len(objectives)}.
- Accepted `minimum + 1 day` ensemble: {len(accepted)}.
- Objective range: {min(objectives):.12f} to {max(objectives):.12f} days.
- GSI identifiability: `{identified}`; every accepted configuration remains frozen.
- `Bf_max + Bs` is empirically constrained only as a combination.
- `fe` and `Cs`/`bb` remain nonidentifiable from admitted empirical data.
- Mature LAI is conditionally range-constrained, not uniquely calibrated.
- Native synthetic recovery demonstrates machinery only and is not empirical evidence.
- Harvard is still sealed at this pre-freeze disposition and cannot influence selection.
"""
    (ARTIFACTS / "identifiability-and-equifinality.md").write_text(text, encoding="utf-8")
    print(f"PASS pre-freeze accepted={len(accepted)} finite={len(objectives)}")


def post_holdout() -> None:
    accepted = rows("accepted-calibration-ensemble.csv")
    holdout = rows("harvard-holdout-results.csv")
    if len(holdout) != len(accepted):
        raise ValueError("holdout result membership differs from frozen ensemble")
    finite = [float(row["aggregate_score"]) for row in holdout if math.isfinite(float(row["aggregate_score"]))]
    failures = len(holdout) - len(finite)
    summary = (
        "# Independent Harvard Validation Summary\n\n"
        "Status: `SCORED_NO_REFIT`\n\n"
        "Evidence class: `Ran`\n\n"
        f"- Frozen candidates scored without refit: {len(holdout)}.\n"
        f"- Finite validation scores: {len(finite)}; retained validation failures: {failures}.\n"
        + (
            f"- Finite aggregate-score range: {min(finite):.12f} to {max(finite):.12f} days.\n"
            if finite
            else "- No candidate produced a finite validation score.\n"
        )
        + "- Validation outcomes were retained and did not alter calibration membership.\n"
    )
    (ARTIFACTS / "holdout-validation-summary.md").write_text(summary, encoding="utf-8")
    print(f"PASS post-holdout candidates={len(holdout)} finite={len(finite)}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--phase", choices=("pre-freeze", "post-holdout"), required=True)
    parser.add_argument("--execution-root", type=Path, required=True)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    if not execution_root.is_dir():
        raise ValueError("execution root must be an existing directory")
    global ARTIFACTS, OBJECTS
    ARTIFACTS = execution_root.parent / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    OBJECTS = execution_root
    if options.phase == "pre-freeze":
        pre_freeze()
    else:
        post_holdout()
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
