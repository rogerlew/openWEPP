#!/usr/bin/env python3
"""Launch one authenticated CAL-04B external-DAG transaction."""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
PLAN = PACKAGE / "artifacts/external-dag-transaction-plan.json"
TRANSACTIONS = ("calibration-v1", "holdout-v1")


def planner_binary(execution_root: Path) -> Path:
    target = execution_root.with_name(f"{execution_root.name}.planner-target")
    environment = {
        **os.environ,
        "CARGO_TARGET_DIR": str(target),
    }
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
        env=environment,
        check=True,
    )
    return target / "debug/openwepp-gate-plan"


def command(options: argparse.Namespace, binary: Path) -> list[str]:
    execution_root = options.execution_root
    control_root = options.control_root
    transaction_id = options.transaction_id
    argv = [
        str(binary),
        "run-external-transition",
        "--repo",
        str(ROOT),
        "--external-plan",
        str(PLAN),
        "--transaction-id",
        transaction_id,
        "--attempt-root",
        str(execution_root),
        "--ledger",
        str(control_root / "ledger.jsonl"),
        "--output",
        str(control_root / f"{transaction_id}.receipt.json"),
        "--principal",
        options.principal,
        "--repository",
        options.repository,
        "--source-event",
        options.source_event,
        "--source-ref",
        options.source_ref,
        "--workflow",
        options.workflow,
        "--job",
        options.job,
        "--runner",
        options.runner,
        "--attempt",
        str(options.attempt),
    ]
    if transaction_id == "holdout-v1":
        custody_root = options.custody_root.resolve(strict=True)
        argv.extend(
            [
                "--custody-root",
                str(custody_root),
                "--opening-token",
                str(custody_root / "holdout-opened-once.lock"),
            ]
        )
    return argv


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    parser.add_argument("--transaction-id", choices=TRANSACTIONS, required=True)
    parser.add_argument("--principal", required=True)
    parser.add_argument("--repository", required=True)
    parser.add_argument("--source-event", required=True)
    parser.add_argument("--source-ref", required=True)
    parser.add_argument("--workflow", required=True)
    parser.add_argument("--job", required=True)
    parser.add_argument("--runner", required=True)
    parser.add_argument("--attempt", type=int, required=True)
    parser.add_argument("--custody-root", type=Path)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=False)
    if execution_root.exists() or not execution_root.parent.is_dir():
        raise ValueError("execution root must be a fresh path below an existing directory")
    options.execution_root = execution_root
    control_root = execution_root.with_name(f"{execution_root.name}.control")
    control_root.mkdir()
    options.control_root = control_root
    if options.transaction_id == "holdout-v1" and options.custody_root is None:
        raise ValueError("holdout transaction requires an external custody root")
    subprocess.run(command(options, planner_binary(execution_root)), cwd=ROOT, check=True)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
