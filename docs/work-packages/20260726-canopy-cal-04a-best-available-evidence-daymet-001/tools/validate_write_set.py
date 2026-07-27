#!/usr/bin/env python3
"""Reconcile visible repository changes with CAL-04A's declared write set."""

from __future__ import annotations

import subprocess

CURRENT = "docs/work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001/"
PREDECESSOR = "docs/work-packages/20260726-canopy-cal-04-process-calibration-identifiability-001/"
ALLOWED = {
    "docs/work-packages/README.md",
    "docs/planning/canopy-phenology-assurance-roadmap.md",
}

output = subprocess.run(
    ["git", "status", "--porcelain=v1", "-uall"],
    check=True,
    capture_output=True,
    text=True,
).stdout
current = predecessor = 0
for line in output.splitlines():
    path = line[3:]
    if path.startswith(CURRENT) or path.startswith(
        "references/canopy_phenology/daymet_calibration/"
    ):
        current += 1
    elif path.startswith(PREDECESSOR):
        predecessor += 1
    elif path not in ALLOWED:
        raise SystemExit(f"FAIL path outside CAL-04A write set: {path}")
if current == 0:
    raise SystemExit("FAIL no CAL-04A paths found")
print(
    f"PASS write set: {current} CAL-04A paths; "
    f"{predecessor} ambient predecessor paths classified separately"
)
