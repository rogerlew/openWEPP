#!/usr/bin/env python3
"""Create or validate checksum-bound CAL-04B freeze verifier receipts."""

from __future__ import annotations

import argparse
import csv
import hashlib
import os
import sys
from datetime import datetime, timezone
from pathlib import Path

from custody import (
    RECEIPT_FIELDS,
    capability_identity,
    sha256_file,
    validate_freeze,
    validate_receipt_barrier,
    write_attestation,
)

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
SOURCE_ARTIFACTS = PACKAGE / "artifacts"
ARTIFACTS = SOURCE_ARTIFACTS
OBJECTS = Path("/nonexistent/cal04b-execution-root-required")
RECEIPTS = OBJECTS / "freeze-receipts"
SCRIPT = Path(__file__).resolve()
PREOPEN = PACKAGE / "tools/validate_preopen.py"


def artifact_input(name: str) -> Path:
    external = ARTIFACTS / name
    return external if external.is_file() else SOURCE_ARTIFACTS / name


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


def verifier_command(verifier_id: str, execution_root: Path) -> str:
    relative = SCRIPT.relative_to(ROOT)
    return (
        "PYTHONDONTWRITEBYTECODE=1 .venv/bin/python "
        f"{relative} --execution-root {execution_root} --verifier-id {verifier_id}"
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    parser.add_argument("--custody-root", type=Path, required=True)
    parser.add_argument("--verifier-id")
    parser.add_argument("--validate-barrier", action="store_true")
    parser.add_argument("--capability", type=Path)
    parser.add_argument("--attestation-out", type=Path)
    parser.add_argument("--transaction-id")
    parser.add_argument("--parent-dispatch-id")
    parser.add_argument("--agent-task-id")
    parser.add_argument("--principal")
    parser.add_argument("--workflow")
    parser.add_argument("--job")
    parser.add_argument("--runner")
    parser.add_argument("--attempt", type=int)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    if not execution_root.is_dir():
        raise ValueError("execution root must be an existing directory")
    global ARTIFACTS, OBJECTS, RECEIPTS
    attempt_root = execution_root.parent
    custody_root = options.custody_root.resolve(strict=True)
    if not custody_root.is_dir():
        raise ValueError("custody root must be an existing directory")
    ARTIFACTS = attempt_root / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    OBJECTS = execution_root
    RECEIPTS = custody_root / "freeze-receipts"
    manifest = artifact_input("holdout-freeze-manifest.csv")
    digest, members = validate_freeze(
        manifest,
        artifact_input("holdout-freeze-digest.txt"),
        frozen_bundle_root(manifest),
    )
    RECEIPTS.mkdir(parents=True, exist_ok=True)
    expected_commands = {
        verifier_id: verifier_command(verifier_id, execution_root)
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
    claims = (
        options.capability,
        options.attestation_out,
        options.transaction_id,
        options.parent_dispatch_id,
        options.agent_task_id,
        options.principal,
        options.workflow,
        options.job,
        options.runner,
        options.attempt,
    )
    if any(value is None for value in claims):
        raise ValueError("verifier capability, attestation output, and claims are required")
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
    capability = options.capability.resolve(strict=True)
    attestation = options.attestation_out.resolve(strict=False)
    for candidate in (capability, attestation):
        try:
            candidate.relative_to(custody_root)
        except ValueError:
            raise ValueError("verifier custody path escapes custody root") from None
    expected_attestation = (
        f"freeze_verify_{options.verifier_id.removeprefix('verifier_')}.json"
    )
    if attestation.name != expected_attestation:
        raise ValueError("verifier attestation filename differs from command identity")
    capability_hash = capability_identity(capability)
    expected_capability = custody_root / "capabilities" / f"{capability_hash}.cap"
    if capability != expected_capability:
        raise ValueError("verifier capability path is not hash-addressed for Rust consumption")
    attestation_argv = [str(SCRIPT), *(argv if argv is not None else sys.argv[1:])]
    write_attestation(
        attestation,
        capability_hash=capability_hash,
        transaction_id=options.transaction_id,
        parent_dispatch_id=options.parent_dispatch_id,
        agent_task_id=options.agent_task_id,
        principal=options.principal,
        workflow=options.workflow,
        job=options.job,
        runner=options.runner,
        attempt=options.attempt,
        script=SCRIPT,
        argv=attestation_argv,
        receipt=path,
        freeze_digest=digest,
    )
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
