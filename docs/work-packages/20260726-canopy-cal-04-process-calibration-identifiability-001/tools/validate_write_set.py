#!/usr/bin/env python3
"""Validate the package terminal worktree against its declared write set."""

from __future__ import annotations

import subprocess


ALLOWED = (
    "docs/work-packages/README.md",
    "docs/planning/canopy-phenology-assurance-roadmap.md",
    "docs/work-packages/"
    "20260726-canopy-cal-04-process-calibration-identifiability-001/",
)


def main() -> int:
    output = subprocess.check_output(
        ["git", "status", "--porcelain=v1", "-uall"], text=True
    )
    paths: list[str] = []
    for line in output.splitlines():
        path = line[3:]
        if " -> " in path:
            path = path.split(" -> ", 1)[1]
        paths.append(path)
    unexpected = [
        path
        for path in paths
        if not any(path == allowed or path.startswith(allowed) for allowed in ALLOWED)
    ]
    if unexpected:
        raise SystemExit(f"paths outside declared write set: {unexpected}")
    print(f"PASS write set: {len(paths)} status paths, all declared")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
