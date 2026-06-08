#!/usr/bin/env python3
"""Static Binding Exposure Index lint for openWEPP science contracts."""
from __future__ import annotations
import re
import sys
from pathlib import Path

VALID_STATUS = {"active", "superseded", "historical"}
VALID_CLASS = {"maps-to-existing-INV", "unpromoted-binding", "historical-or-superseded", "undecidable"}
ROUTED_GATES = {"science-review-follow-on"}


def code_value(cell: str) -> str | None:
    m = re.search(r"`([^`]+)`", cell)
    return m.group(1).strip() if m else None


def parse_row(line: str) -> dict[str, str] | None:
    # Markdown tables in the index may include inline backticks inside the Source
    # title cell. Split by table pipes instead of trying to count backtick groups.
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) < 7:
        return None
    return {
        "entry": code_value(cells[0]) or "",
        "source": cells[1],
        "status": code_value(cells[2]) or "",
        "class": code_value(cells[3]) or "",
        "ids": code_value(cells[4]) or "",
        "gate": code_value(cells[5]) or "",
        "notes": cells[6],
    }


def main(argv: list[str]) -> int:
    args = argv[1:]
    strict = "--strict" in args
    args = [a for a in args if a != "--strict"]
    if len(args) != 1:
        print("usage: check_sc_binding_exposure.py [--strict] <SC-contract.md>", file=sys.stderr)
        return 2
    path = Path(args[0])
    text = path.read_text(errors="replace")
    if "## Binding Exposure Index" not in text:
        print(f"FAIL {path}: missing Binding Exposure Index")
        return 1
    core_ids = set(re.findall(r"\b(?:INV|OBL)-[A-Z0-9]+-\d+\b", text))
    in_index = False
    rows = []
    failures = []
    for line in text.splitlines():
        if line.startswith("## Binding Exposure Index"):
            in_index = True
            continue
        if in_index and line.startswith("## "):
            break
        if not in_index or not line.startswith("| `"):
            continue
        row = parse_row(line)
        if row is None:
            failures.append(f"malformed index row: {line[:140]}")
            continue
        rows.append(row)
        entry = row["entry"]
        status = row["status"]
        cls = row["class"]
        gate = row["gate"]
        ids = [x.strip() for x in row["ids"].split(",") if x.strip() and x.strip() != "none"]
        routed_to_science = gate in ROUTED_GATES
        if status not in VALID_STATUS:
            failures.append(f"{entry}: invalid status {status}")
        if cls not in VALID_CLASS:
            failures.append(f"{entry}: invalid binding classification {cls}")
        if status == "active" and not ids and not routed_to_science:
            failures.append(f"{entry}: active entry has no canonical binding IDs")
        if cls == "unpromoted-binding" and not ids and not routed_to_science:
            failures.append(f"{entry}: unpromoted binding lacks promoted INV/OBL mapping")
        if cls == "undecidable" and not routed_to_science:
            failures.append(f"{entry}: undecidable binding status blocks consolidation")
        for binding_id in ids:
            if binding_id not in core_ids:
                failures.append(f"{entry}: referenced binding ID {binding_id} not present in contract")
    if not rows:
        failures.append("Binding Exposure Index contains no rows")
    if failures:
        print(f"FAIL {path}: {len(failures)} issue(s)")
        for failure in failures:
            print(f"- {failure}")
        return 1
    routed = sum(1 for row in rows if row["gate"] in ROUTED_GATES)
    if routed:
        # Binding-safe but not fully consolidated: rows are parked for science
        # review with narrative retained in core. Distinct from a clean PASS so a
        # completion gate is not satisfied by deferral (see science-contract-spec.md).
        print(f"PASS-DEFERRED {path}: {len(rows)} binding exposure row(s), "
              f"{routed} science-review-follow-on row(s) not yet consolidated")
        if strict:
            print("strict mode: deferred rows are not-consolidated; failing completion gate")
            return 1
        return 0
    print(f"PASS {path}: {len(rows)} binding exposure row(s) fully consolidated")
    return 0

if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
