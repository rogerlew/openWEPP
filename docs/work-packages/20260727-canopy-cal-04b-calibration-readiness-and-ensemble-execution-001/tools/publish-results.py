#!/usr/bin/env python3
"""Publish an authenticated CAL-04B external result transaction."""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]


def planner_binary(objects_root: Path) -> Path:
    attempt_root = objects_root.parent
    target = attempt_root.with_name(f"{attempt_root.name}.publication-target")
    subprocess.run(
        [
            "cargo",
            "build",
            "-p",
            "openwepp-gate-planner",
            "--bin",
            "openwepp-gate-plan",
        ],
        cwd=ROOT,
        env={**os.environ, "CARGO_TARGET_DIR": str(target)},
        check=True,
    )
    return target / "debug/openwepp-gate-plan"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    parser.add_argument("--publication-plan", type=Path, required=True)
    parser.add_argument("--recovery", choices=("complete", "restore"))
    options = parser.parse_args(argv)
    objects_root = options.execution_root.resolve(strict=True)
    if not objects_root.is_dir():
        raise ValueError("execution root must be an existing objects directory")
    publication_plan = options.publication_plan.resolve(strict=True)
    command = [
        str(planner_binary(objects_root)),
        "publish-external-results",
        "--repo",
        str(ROOT),
        "--publication-plan",
        str(publication_plan),
    ]
    if options.recovery:
        command.extend(["--recovery", options.recovery])
    subprocess.run(command, cwd=ROOT, check=True)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
