#!/usr/bin/env python3
"""Run the authorized observed CAL-04B pre-freeze prefix without shell eval."""

from __future__ import annotations

import csv
import shlex
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
PLAN = PACKAGE / "artifacts/executor-command-plan.csv"
OBSERVE = PACKAGE / "tools/observe.py"
AUTHORIZED_PREFIX = (
    "prepare",
    "build_executor",
    "build_production_runner",
    "native_proof",
    "synthetic_gsi",
    "hubbard_producer",
    "hubbard_primary_reconstruct",
    "hubbard_verify_reconstruct",
    "retain_trace",
    "readiness",
    "summarize_pre_freeze",
)


def select_prefix(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    ids = [row["command_id"] for row in rows]
    prefix_length = len(AUTHORIZED_PREFIX)
    if tuple(ids[:prefix_length]) != AUTHORIZED_PREFIX:
        raise ValueError("observed plan does not begin with the exact authorized prefix")
    if len(ids) <= prefix_length or ids[prefix_length] != "freeze":
        raise ValueError("authorized prefix is not followed immediately by freeze")
    return rows[:prefix_length]


def main() -> int:
    with PLAN.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    selected = select_prefix(rows)
    for row in selected:
        command = [
            str(ROOT / ".venv/bin/python"),
            str(OBSERVE),
            "run",
            "--command-id",
            row["command_id"],
            "--",
            *shlex.split(row["argv"]),
        ]
        print(f"OBSERVED_START {row['command_id']}", flush=True)
        subprocess.run(command, cwd=ROOT, check=True)
        print(f"OBSERVED_PASS {row['command_id']}", flush=True)
    subprocess.run(
        [
            str(ROOT / ".venv/bin/python"),
            str(OBSERVE),
            "render",
            "--through",
            AUTHORIZED_PREFIX[-1],
            "--snapshot",
            "pre-freeze",
        ],
        cwd=ROOT,
        check=True,
    )
    print("PASS observed pre-freeze prefix and immutable snapshot", flush=True)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
