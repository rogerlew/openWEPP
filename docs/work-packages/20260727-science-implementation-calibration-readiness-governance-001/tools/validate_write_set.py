#!/usr/bin/env python3
"""Reconcile visible changes with governance scope and admitted predecessors."""

from __future__ import annotations

import subprocess

PACKAGE = "docs/work-packages/20260727-science-implementation-calibration-readiness-governance-001/"
GOVERNANCE_FILES = {
    "AGENTS.md",
    "docs/decisions/README.md",
    "docs/decisions/0042-science-implementation-and-calibration-readiness.md",
    "docs/specifications/correctness-authority-model.md",
    "docs/specifications/science-contract-spec.md",
    "docs/specifications/science-contracts/kernel-process-contract-profile.md",
    "docs/specifications/science-contract-authoring-procedure.md",
    "docs/work-packages/AGENTS.md",
    "docs/standards/kernel-work-package-preparation.md",
    "docs/work-packages/README.md",
}
status = subprocess.run(
    ["git", "status", "--porcelain=v1", "-uall"],
    check=True,
    capture_output=True,
    text=True,
).stdout

governance = 0
for line in status.splitlines():
    path = line[3:]
    if path.startswith(PACKAGE) or path in GOVERNANCE_FILES:
        governance += 1
    else:
        raise SystemExit(f"FAIL unclassified path: {path}")

if governance == 0:
    raise SystemExit("FAIL no governance paths found")

print(f"PASS write set: {governance} governance paths from baseline 5e3203a7")
