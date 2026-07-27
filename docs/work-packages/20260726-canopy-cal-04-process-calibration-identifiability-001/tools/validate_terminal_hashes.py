#!/usr/bin/env python3
"""Validate checksum-bound terminal CAL-04 evidence."""

from __future__ import annotations

import csv
import hashlib
from pathlib import Path


PACKAGE = Path(__file__).resolve().parents[1]
MANIFEST = PACKAGE / "artifacts/terminal-artifact-hashes.csv"


def main() -> int:
    with MANIFEST.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    if len(rows) != 20:
        raise SystemExit(f"expected 20 terminal identities, found {len(rows)}")
    for row in rows:
        path = PACKAGE / row["path"]
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != row["sha256"]:
            raise SystemExit(f"terminal identity mismatch: {path}")
    print("PASS terminal identities: 20 artifacts match")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
