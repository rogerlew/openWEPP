#!/usr/bin/env python3
"""Atomically open and score the frozen Harvard holdout exactly once."""

from __future__ import annotations

import argparse
import csv
import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

from custody import (
    RECEIPT_FIELDS,
    read_csv_exact,
    sha256_file,
    validate_freeze,
    validate_receipt_barrier,
)
import observe

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
SOURCE_ARTIFACTS = PACKAGE / "artifacts"
ARTIFACTS = SOURCE_ARTIFACTS
OBJECTS = Path("/nonexistent/cal04b-execution-root-required")
TOKEN = OBJECTS / "holdout-opened-once.lock"
RECEIPTS = OBJECTS / "freeze-receipts"
EXECUTOR = Path("/nonexistent/cal04b-execution-root-required")
CUSTODY = Path("/nonexistent/cal04b-custody-root-required")
HARVARD = ROOT / "tests/fixtures/cancov_forest/harvard_deciduous_ma"
TIMING = ROOT / "docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/artifacts/cal04-timing-windows.csv"


def sha(path: Path) -> str:
    return sha256_file(path)


def verifier_command(verifier_id: str) -> str:
    script = PACKAGE / "tools/freeze-verify.py"
    return (
        "PYTHONDONTWRITEBYTECODE=1 .venv/bin/python "
        f"{script.relative_to(ROOT)} --verifier-id {verifier_id}"
    )


def validate_unopened_targets() -> None:
    holdout_root = OBJECTS / "holdout"
    if TOKEN.exists():
        raise ValueError(f"holdout was already opened: {TOKEN}")
    if holdout_root.exists():
        raise ValueError(f"holdout output root already exists: {holdout_root}")
    if (ARTIFACTS / "holdout-execution-receipt.csv").exists():
        raise ValueError("holdout execution receipt already exists")
    result = ARTIFACTS / "harvard-holdout-results.csv"
    sealed_result = "candidate_id,year,component_score,aggregate_score,state\n"
    if result.exists() and result.read_text(encoding="utf-8") != sealed_result:
        raise ValueError("Harvard result target is not the canonical sealed placeholder")
    record = ARTIFACTS / "holdout-opening-record.md"
    if record.exists():
        text = record.read_text(encoding="utf-8")
        if "State: `SEALED`" not in text or "Evidence class: `NOT RUN`" not in text:
            raise ValueError("holdout opening record is not in the sealed state")


def preflight() -> str:
    validate_unopened_targets()
    subprocess.run(
        [str(ROOT / ".venv/bin/python"), str(PACKAGE / "tools/validate_preopen.py")],
        cwd=ROOT,
        env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
        check=True,
    )
    observe.validate_prefix("freeze_barrier")
    digest, _members = validate_freeze(
        ARTIFACTS / "holdout-freeze-manifest.csv",
        ARTIFACTS / "holdout-freeze-digest.txt",
        _frozen_bundle_root(ARTIFACTS / "holdout-freeze-manifest.csv"),
    )
    calibration = json.loads((CUSTODY / "calibration-v1.receipt.json").read_bytes())
    if (
        calibration.get("transaction_id") != "calibration-v1"
        or calibration.get("result") != "PASS"
    ):
        raise ValueError("Generation-B calibration receipt is not passing")
    freeze_receipt = json.loads((CUSTODY / "freeze.receipt.json").read_bytes())
    if freeze_receipt.get("result") != "PASS" or freeze_receipt.get(
        "freeze_digest"
    ) != digest:
        raise ValueError("Generation-B freeze receipt differs from frozen digest")
    receipt_paths = [RECEIPTS / "verifier_a.csv", RECEIPTS / "verifier_b.csv"]
    expected_commands = {
        verifier_id: verifier_command(verifier_id)
        for verifier_id in ("verifier_a", "verifier_b")
    }
    rows = validate_receipt_barrier(
        receipt_paths,
        digest,
        PACKAGE / "tools/freeze-verify.py",
        expected_commands,
    )
    summary = read_csv_exact(
        ARTIFACTS / "freeze-verifier-receipts.csv",
        RECEIPT_FIELDS,
    )
    if summary != rows:
        raise ValueError("published verifier barrier differs from immutable receipts")
    with (ARTIFACTS / "accepted-calibration-ensemble.csv").open(
        newline="", encoding="utf-8"
    ) as stream:
        if sum(1 for _ in csv.DictReader(stream)) == 0:
            raise ValueError("accepted ensemble is empty")
    return digest


def create_token(digest: str) -> None:
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
    if hasattr(os, "O_NOFOLLOW"):
        flags |= os.O_NOFOLLOW
    descriptor = os.open(TOKEN, flags, 0o444)
    try:
        payload = f"state=OPENED_ONCE\nfreeze_digest={digest}\ncommand=PYTHONDONTWRITEBYTECODE=1 .venv/bin/python {PACKAGE.relative_to(ROOT)}/tools/holdout.py\ntimestamp={datetime.now(timezone.utc).isoformat()}\n"
        encoded = payload.encode()
        written = 0
        while written < len(encoded):
            count = os.write(descriptor, encoded[written:])
            if count == 0:
                raise OSError("short write while creating holdout token")
            written += count
        os.fsync(descriptor)
    finally:
        os.close(descriptor)
    directory = os.open(TOKEN.parent, os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


def _frozen_bundle_root(manifest: Path) -> Path:
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


def record_incomplete(digest: str) -> None:
    (ARTIFACTS / "holdout-opening-record.md").write_text(
        f"# Holdout Opening Record\n\nState: `OPENED_ONCE / INCOMPLETE`\n\n"
        f"Freeze digest: `{digest}`\n\n"
        "Rerun is forbidden; incident disposition is required.\n",
        encoding="utf-8",
    )


def validate_harvard_after_open() -> None:
    expected = ARTIFACTS / "harvard-expected-input-manifest.csv"
    rows = read_csv_exact(expected, ["path", "expected_git_blob", "state"])
    if len(rows) != 6:
        raise ValueError("expected six sealed Harvard input identities")
    for row in rows:
        if row["state"] != "EXPECTED_COMMITTED_PREOPEN_NOT_READ":
            raise ValueError(f"invalid Harvard expected state {row['path']}")
        if len(row["expected_git_blob"]) != 40:
            raise ValueError(f"invalid Harvard expected Git identity {row['path']}")
        path = ROOT / row["path"]
        actual = subprocess.run(
            ["git", "hash-object", "--", str(path)],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=True,
        ).stdout.strip()
        if actual != row["expected_git_blob"]:
            raise ValueError(f"Harvard input identity mismatch {row['path']}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    parser.add_argument("--opening-token", type=Path)
    parser.add_argument("--custody-root", type=Path, required=True)
    parser.add_argument("--preflight-only", action="store_true")
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    if not execution_root.is_dir():
        raise ValueError("execution root must be an existing directory")
    custody_root = options.custody_root.resolve(strict=True)
    token_option = options.opening_token or custody_root / "holdout-opened-once.lock"
    opening_token = token_option.resolve(strict=False)
    attempt_root = execution_root.parent
    try:
        opening_token.relative_to(custody_root)
    except ValueError:
        raise ValueError("opening token escapes custody root") from None
    if opening_token.exists() or opening_token.is_symlink():
        raise ValueError("opening token already exists")
    global ARTIFACTS, OBJECTS, TOKEN, RECEIPTS, EXECUTOR, CUSTODY
    ARTIFACTS = attempt_root / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    OBJECTS = execution_root
    TOKEN = opening_token
    RECEIPTS = custody_root / "freeze-receipts"
    CUSTODY = custody_root
    EXECUTOR = attempt_root / "cargo-target/release"
    digest = preflight()
    if options.preflight_only:
        print(f"PASS holdout preflight sealed digest={digest}")
        return 0
    # Recheck mutable output targets immediately before the irreversible token.
    validate_unopened_targets()
    create_token(digest)
    try:
        validate_harvard_after_open()
        holdout_root = OBJECTS / "holdout"
        holdout_root.mkdir(exist_ok=False)
        trace = holdout_root / "harvard-gsi.bin"
        identity = holdout_root / "harvard-gsi-identity.csv"
        producer_command = [
            str(EXECUTOR / "holdout-producer"),
            "--configs", str(ARTIFACTS / "candidate-configurations.csv"),
            "--accepted", str(ARTIFACTS / "accepted-calibration-ensemble.csv"),
            "--climate", str(HARVARD / "p6.cli"),
            "--trace", str(trace), "--identity", str(identity),
        ]
        subprocess.run(producer_command, cwd=ROOT, check=True)
        observation_out = holdout_root / "harvard-observation-components.csv"
        annual_out = holdout_root / "harvard-annual-components.csv"
        result_out = ARTIFACTS / "harvard-holdout-results.csv"
        reconstruct_command = [
            str(EXECUTOR / "holdout-reconstruct"),
            "--trace", str(trace), "--calendar", str(trace.with_suffix(".calendar.csv")),
            "--identity", str(identity),
            "--accepted", str(ARTIFACTS / "accepted-calibration-ensemble.csv"),
            "--observations", str(TIMING),
            "--observation-out", str(observation_out), "--annual-out", str(annual_out),
            "--result-out", str(result_out),
        ]
        subprocess.run(reconstruct_command, cwd=ROOT, check=True)
        receipt_row = {
            "state": "PASS_SCORED_NO_REFIT",
            "freeze_digest": digest,
            "token_sha256": sha(TOKEN),
            "expected_input_manifest_sha256": sha(
                ARTIFACTS / "harvard-expected-input-manifest.csv"
            ),
            "accepted_ensemble_sha256": sha(
                ARTIFACTS / "accepted-calibration-ensemble.csv"
            ),
            "trace_sha256": sha(trace),
            "trace_identity_sha256": sha(identity),
            "calendar_sha256": sha(trace.with_suffix(".calendar.csv")),
            "observation_components_sha256": sha(observation_out),
            "annual_components_sha256": sha(annual_out),
            "results_sha256": sha(result_out),
            "producer_command": " ".join(producer_command),
            "reconstructor_command": " ".join(reconstruct_command),
            "holdout_script_sha256": sha(Path(__file__)),
        }
        receipt_path = ARTIFACTS / "holdout-execution-receipt.csv"
        with receipt_path.open("x", newline="", encoding="utf-8") as stream:
            writer = csv.DictWriter(
                stream, fieldnames=list(receipt_row), lineterminator="\n"
            )
            writer.writeheader()
            writer.writerow(receipt_row)
        record = f"""# Holdout Opening Record

State: `SCORED_NO_REFIT`

Evidence class: `Ran`

- Durable token: `{TOKEN}`
- Freeze digest: `{digest}`
- Opened: `{datetime.now(timezone.utc).isoformat()}`
- Expected Harvard identities: `PASS`
- Accepted ensemble SHA-256: `{sha(ARTIFACTS / 'accepted-calibration-ensemble.csv')}`
- Holdout trace SHA-256: `{sha(trace)}`
- Holdout trace identity SHA-256: `{sha(identity)}`
- Observation components SHA-256: `{sha(observation_out)}`
- Annual components SHA-256: `{sha(annual_out)}`
- Results SHA-256: `{sha(result_out)}`
- Execution receipt SHA-256: `{sha(receipt_path)}`

The exclusive token was durably created before the first Harvard content read.
No refit or calibration write path was available.
"""
        (ARTIFACTS / "holdout-opening-record.md").write_text(record, encoding="utf-8")
        print(f"PASS holdout opened once digest={digest}")
        return 0
    except Exception:
        record_incomplete(digest)
        raise


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
