#!/usr/bin/env python3
"""Create or validate two direct, checksum-bound freeze verifier records."""

from __future__ import annotations

import argparse
import csv
import hashlib
import os
import secrets
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

from custody import (
    RECEIPT_FIELDS,
    sha256_file,
    validate_freeze,
    validate_receipt_barrier,
)

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
SCRIPT = Path(__file__).resolve()


def frozen_bundle_root(manifest: Path) -> Path:
    with manifest.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    roots = {
        Path(row["path_or_command"]).parent
        for row in rows
        if Path(row["path_or_command"]).parent.name == "freeze-bundles"
    }
    if len(roots) != 1:
        raise ValueError("freeze manifest does not bind exactly one bundle root")
    return next(iter(roots))


def verifier_command(
    verifier_id: str, execution_root: Path, custody_root: Path
) -> str:
    relative = SCRIPT.relative_to(ROOT)
    return (
        "PYTHONDONTWRITEBYTECODE=1 .venv/bin/python "
        f"{relative} --execution-root {execution_root} "
        f"--custody-root {custody_root} --verifier-id {verifier_id}"
    )


def fsync_directory(path: Path) -> None:
    descriptor = os.open(path, os.O_RDONLY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def paths_overlap(left: Path, right: Path) -> bool:
    return left == right or left in right.parents or right in left.parents


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    parser.add_argument("--custody-root", type=Path, required=True)
    parser.add_argument("--verifier-id")
    parser.add_argument("--validate-barrier", action="store_true")
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    custody_root = options.custody_root.resolve(strict=True)
    if not execution_root.is_dir() or not custody_root.is_dir():
        raise ValueError("execution and custody roots must be existing directories")
    attempt_root = execution_root.parent
    if paths_overlap(custody_root, ROOT.resolve()) or paths_overlap(
        custody_root, attempt_root
    ):
        raise ValueError("custody root overlaps repository or calibration attempt")
    artifacts = attempt_root / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"
    manifest = artifacts / "holdout-freeze-manifest.csv"
    digest, members = validate_freeze(
        manifest,
        artifacts / "holdout-freeze-digest.txt",
        frozen_bundle_root(manifest),
    )
    receipts = custody_root / "freeze-receipts"
    receipts.mkdir(exist_ok=True)
    expected = {
        verifier_id: verifier_command(verifier_id, execution_root, custody_root)
        for verifier_id in ("verifier_a", "verifier_b")
    }
    if options.validate_barrier:
        rows = validate_receipt_barrier(
            [receipts / "verifier_a.csv", receipts / "verifier_b.csv"],
            digest,
            SCRIPT,
            expected,
        )
        summary = artifacts / "freeze-verifier-receipts.csv"
        with summary.open("x", newline="", encoding="utf-8") as stream:
            writer = csv.DictWriter(
                stream, fieldnames=RECEIPT_FIELDS, lineterminator="\n"
            )
            writer.writeheader()
            writer.writerows(rows)
            stream.flush()
            os.fsync(stream.fileno())
        fsync_directory(summary.parent)
        print(f"PASS freeze barrier digest={digest} transitive_members={members}")
        return 0
    if options.verifier_id not in expected:
        raise ValueError("verifier id must be verifier_a or verifier_b")
    subprocess.run(
        [
            str(ROOT / ".venv/bin/python"),
            str(PACKAGE / "tools/validate_preopen.py"),
            "--execution-root",
            str(execution_root),
            "--custody-root",
            str(custody_root),
        ],
        cwd=ROOT,
        env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
        check=True,
    )
    path = receipts / f"{options.verifier_id}.csv"
    command = expected[options.verifier_id]
    row = {
        "verifier_id": options.verifier_id,
        "invocation_id": secrets.token_hex(16),
        "freeze_digest": digest,
        "verifier_script_sha256": sha256_file(SCRIPT),
        "command": command,
        "command_sha256": hashlib.sha256(command.encode()).hexdigest(),
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "state": "PASS",
    }
    with path.open("x", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=RECEIPT_FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerow(row)
        stream.flush()
        os.fsync(stream.fileno())
    fsync_directory(path.parent)
    path.chmod(0o444)
    print(
        f"PASS {options.verifier_id} digest={digest} "
        f"transitive_members={members}"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
