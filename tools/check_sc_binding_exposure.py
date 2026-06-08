#!/usr/bin/env python3
"""Static Binding Exposure Index lint for openWEPP science contracts."""
from __future__ import annotations
import re
import sys
from pathlib import Path

PIPE_RE = re.compile(r"(?<!\\)\|")
VALID_STATUS = {"active", "superseded", "historical"}
VALID_CLASS = {"maps-to-existing-INV", "unpromoted-binding", "historical-or-superseded", "undecidable"}


def _strip_cell(cell: str) -> str:
    cell = cell.strip()
    if len(cell) >= 2 and cell.startswith("`") and cell.endswith("`"):
        cell = cell[1:-1]
    return cell.strip()


def _parse_row(line: str):
    # Split the markdown row on unescaped pipes and drop the empty cells before
    # the first and after the last delimiter. This tolerates arbitrary inline
    # backticks in any cell (e.g. titles containing `drfc`/`solwpv`), unlike a
    # rigid backtick-counting regex.
    parts = PIPE_RE.split(line.strip())
    if parts and parts[0].strip() == "":
        parts = parts[1:]
    if parts and parts[-1].strip() == "":
        parts = parts[:-1]
    if len(parts) != 7:
        return None
    entry, source, status, cls, ids, gate, notes = (_strip_cell(p) for p in parts)
    return {"entry": entry, "source": source, "status": status,
            "class": cls, "ids": ids, "gate": gate, "notes": notes}


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        print("usage: check_sc_binding_exposure.py <SC-contract.md>", file=sys.stderr)
        return 2
    path = Path(argv[1])
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
        row = _parse_row(line)
        if row is None:
            failures.append(f"malformed index row: {line[:140]}")
            continue
        rows.append(row)
        entry = row["entry"]
        status = row["status"]
        cls = row["class"]
        ids = [x.strip() for x in row["ids"].split(",") if x.strip() and x.strip() != "none"]
        if status not in VALID_STATUS:
            failures.append(f"{entry}: invalid status {status}")
        if cls not in VALID_CLASS:
            failures.append(f"{entry}: invalid binding classification {cls}")
        if status == "active" and not ids:
            failures.append(f"{entry}: active entry has no canonical binding IDs")
        if cls == "unpromoted-binding" and not ids:
            failures.append(f"{entry}: unpromoted binding lacks promoted INV/OBL mapping")
        if cls == "undecidable":
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
    print(f"PASS {path}: {len(rows)} binding exposure row(s)")
    return 0

if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
