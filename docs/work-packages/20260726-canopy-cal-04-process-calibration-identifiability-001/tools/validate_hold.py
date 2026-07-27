#!/usr/bin/env python3
"""Validate the prospective CAL-04 hold and holdout embargo."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"


def rows(name: str) -> list[dict[str, str]]:
    with (ARTIFACTS / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def main() -> int:
    candidates = rows("candidate-ledger.csv")
    failures = rows("failure-ledger.csv")
    ensemble = rows("accepted-calibration-ensemble.csv")
    holdout = rows("harvard-holdout-results.csv")
    stages = rows("stage-disposition.csv")
    inventory = rows("execution-inventory.csv")
    domains = rows("search-domain-and-stage-plan.csv")

    require(not candidates, "prospective hold must have zero attempted candidates")
    require(not failures, "zero attempted candidates must have zero model failures")
    require(not ensemble, "prospective hold must not claim an accepted ensemble")
    require(not holdout, "Harvard holdout result ledger must remain sealed and empty")
    require(len(stages) == 7, "all five calibration stages plus holdout/downstream required")
    require(all(row["terminal_state"] == "HOLD" for row in stages), "every stage must hold")
    require(
        len(inventory) == 24,
        "inventory must enumerate seven intake, ten closure, and seven scientific rows",
    )
    require(
        all(
            row["state"] in {"PASS", "FAILED_RETAINED"}
            if row["execution_id"].startswith(("INTAKE", "CLOSURE", "TERMINAL"))
            else row["state"].startswith("BLOCKED")
            for row in inventory
        ),
        "closure work must pass and result-bearing execution must remain blocked",
    )
    require(len(domains) == 12, "all six GSI and six downstream operands required")
    require(
        any("UNBOUNDED" in row["domain_lower"] or "UNBOUNDED" in row["domain_upper"] for row in domains),
        "hold must retain the unbounded typed domains",
    )

    opening = (ARTIFACTS / "holdout-opening-record.md").read_text(encoding="utf-8")
    require("Status: `SEALED`" in opening, "holdout opening record must remain SEALED")
    require("not opened" in opening.lower(), "holdout seal must explicitly deny opening")

    candidate_ids = {row["candidate_id"] for row in candidates}
    require(
        all(row["candidate_id"] in candidate_ids for row in failures),
        "every failure must join to an attempted candidate",
    )
    print(
        "PASS prospective hold: 0 candidates, 0 failures, 0 ensemble members, "
        "0 holdout rows; 7 held stages; 12 operand-domain rows"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError) as error:
        print(f"FAIL prospective hold: {error}", file=sys.stderr)
        raise SystemExit(1) from error
