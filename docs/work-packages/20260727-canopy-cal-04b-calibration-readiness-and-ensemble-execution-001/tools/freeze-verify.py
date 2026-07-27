#!/usr/bin/env python3
"""Create or validate checksum-bound CAL-04B freeze verifier receipts."""

from __future__ import annotations

import argparse
import csv
import hashlib
import os
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
ARTIFACTS = PACKAGE / "artifacts"
OBJECTS = Path("/home/workdir/cal04b-objects")
RECEIPTS = OBJECTS / "freeze-receipts"
SCRIPT = Path(__file__).resolve()
PREOPEN = PACKAGE / "tools/validate_preopen.py"


def verifier_command(verifier_id: str) -> str:
    relative = SCRIPT.relative_to(ROOT)
    return (
        "PYTHONDONTWRITEBYTECODE=1 .venv/bin/python "
        f"{relative} --verifier-id {verifier_id}"
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verifier-id")
    parser.add_argument("--validate-barrier", action="store_true")
    options = parser.parse_args()
    subprocess.run(
        [str(ROOT / ".venv/bin/python"), str(PREOPEN)],
        cwd=ROOT,
        env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
        check=True,
    )
    digest, members = validate_freeze(
        ARTIFACTS / "holdout-freeze-manifest.csv",
        ARTIFACTS / "holdout-freeze-digest.txt",
        OBJECTS / "freeze-bundles",
    )
    RECEIPTS.mkdir(parents=True, exist_ok=True)
    expected_commands = {
        verifier_id: verifier_command(verifier_id)
        for verifier_id in ("verifier_a", "verifier_b")
    }
    if options.validate_barrier:
        paths = [RECEIPTS / "verifier_a.csv", RECEIPTS / "verifier_b.csv"]
        rows = validate_receipt_barrier(
            paths,
            digest,
            SCRIPT,
            expected_commands,
        )
        summary = ARTIFACTS / "freeze-verifier-receipts.csv"
        with summary.open("w", newline="", encoding="utf-8") as stream:
            writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
            writer.writeheader(); writer.writerows(rows)
        print(f"PASS freeze barrier digest={digest} transitive_members={members}")
        return 0
    if options.verifier_id not in {"verifier_a", "verifier_b"}:
        raise ValueError("verifier id must be verifier_a or verifier_b")
    path = RECEIPTS / f"{options.verifier_id}.csv"
    if path.exists():
        raise ValueError(f"receipt already exists {path}")
    row = {
        "verifier_id": options.verifier_id,
        "freeze_digest": digest,
        "verifier_script_sha256": sha256_file(SCRIPT),
        "command": expected_commands[options.verifier_id],
        "command_sha256": hashlib.sha256(
            expected_commands[options.verifier_id].encode("utf-8")
        ).hexdigest(),
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "state": "PASS",
    }
    with path.open("x", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=RECEIPT_FIELDS, lineterminator="\n")
        writer.writeheader(); writer.writerow(row)
        stream.flush()
        os.fsync(stream.fileno())
    path.chmod(0o444)
    print(
        f"PASS {options.verifier_id} digest={digest} "
        f"transitive_members={members}"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
