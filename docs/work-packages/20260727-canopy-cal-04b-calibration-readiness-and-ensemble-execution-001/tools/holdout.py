#!/usr/bin/env python3
"""Open and score Harvard once inside a fail-closed read-only sandbox."""

from __future__ import annotations

import argparse
import csv
import json
import os
import shutil
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

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
HARVARD = ROOT / "tests/fixtures/cancov_forest/harvard_deciduous_ma"
TIMING = ROOT / (
    "docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/"
    "artifacts/cal04-timing-windows.csv"
)


def fsync_directory(path: Path) -> None:
    descriptor = os.open(path, os.O_RDONLY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


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
    script = PACKAGE / "tools/freeze-verify.py"
    return (
        "PYTHONDONTWRITEBYTECODE=1 .venv/bin/python "
        f"{script.relative_to(ROOT)} --execution-root {execution_root} "
        f"--custody-root {custody_root} --verifier-id {verifier_id}"
    )


def preflight(
    execution_root: Path,
    custody_root: Path,
    calibration_artifacts: Path,
    output_artifacts: Path,
    token: Path,
) -> str:
    if token.exists() or token.is_symlink():
        raise ValueError("holdout was already opened")
    if output_artifacts.exists() and any(output_artifacts.iterdir()):
        raise ValueError("holdout output root is not empty")
    completion_path = execution_root.parent / "direct-evidence/calibration-complete.json"
    completion = json.loads(completion_path.read_text(encoding="utf-8"))
    if completion.get("state") != "PASS":
        raise ValueError("direct calibration completion is not passing")
    manifest = calibration_artifacts / "holdout-freeze-manifest.csv"
    digest, _ = validate_freeze(
        manifest,
        calibration_artifacts / "holdout-freeze-digest.txt",
        frozen_bundle_root(manifest),
    )
    freeze_receipt = json.loads((custody_root / "freeze.receipt.json").read_bytes())
    if (
        freeze_receipt.get("result") != "PASS"
        or freeze_receipt.get("freeze_digest") != digest
        or freeze_receipt.get("calibration_completion_sha256")
        != sha256_file(completion_path)
    ):
        raise ValueError("freeze receipt differs from direct calibration evidence")
    receipts = custody_root / "freeze-receipts"
    rows = validate_receipt_barrier(
        [receipts / "verifier_a.csv", receipts / "verifier_b.csv"],
        digest,
        PACKAGE / "tools/freeze-verify.py",
        {
            item: verifier_command(item, execution_root, custody_root)
            for item in ("verifier_a", "verifier_b")
        },
    )
    if (
        read_csv_exact(
            calibration_artifacts / "freeze-verifier-receipts.csv", RECEIPT_FIELDS
        )
        != rows
    ):
        raise ValueError("published verifier summary differs")
    with (calibration_artifacts / "accepted-calibration-ensemble.csv").open(
        newline="", encoding="utf-8"
    ) as stream:
        if not list(csv.DictReader(stream)):
            raise ValueError("accepted ensemble is empty")
    return digest


def opening_command(
    execution_root: Path,
    custody_root: Path,
    output_root: Path,
    token: Path,
) -> str:
    return " ".join(
        [
            str(ROOT / ".venv/bin/python"),
            str(Path(__file__).resolve()),
            "--sandboxed",
            "--execution-root",
            str(execution_root),
            "--custody-root",
            str(custody_root),
            "--holdout-output-root",
            str(output_root),
            "--opening-token",
            str(token),
        ]
    )


def create_token(token: Path, digest: str, command: str) -> None:
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
    if hasattr(os, "O_NOFOLLOW"):
        flags |= os.O_NOFOLLOW
    descriptor = os.open(token, flags, 0o444)
    try:
        payload = (
            "state=OPENED_ONCE\n"
            f"freeze_digest={digest}\n"
            f"command={command}\n"
            f"timestamp={datetime.now(timezone.utc).isoformat()}\n"
        ).encode()
        written = 0
        while written < len(payload):
            count = os.write(descriptor, payload[written:])
            if count == 0:
                raise OSError("short write while creating holdout token")
            written += count
        os.fsync(descriptor)
    finally:
        os.close(descriptor)
    fsync_directory(token.parent)


def validate_harvard_after_open(expected_manifest: Path) -> None:
    rows = read_csv_exact(
        expected_manifest, ["path", "expected_git_blob", "state"]
    )
    if len(rows) != 6:
        raise ValueError("expected six sealed Harvard input identities")
    for row in rows:
        if row["state"] != "EXPECTED_COMMITTED_PREOPEN_NOT_READ":
            raise ValueError("invalid preopen Harvard state")
        actual = subprocess.run(
            ["git", "hash-object", "--", str(ROOT / row["path"])],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=True,
        ).stdout.strip()
        if actual != row["expected_git_blob"]:
            raise ValueError(f"Harvard input identity mismatch {row['path']}")


def write_incomplete(output_artifacts: Path, digest: str) -> None:
    (output_artifacts / "holdout-opening-record.md").write_text(
        "# Holdout Opening Record\n\n"
        "State: `OPENED_ONCE / INCOMPLETE`\n\n"
        f"Freeze digest: `{digest}`\n\n"
        "Rerun is forbidden; incident disposition is required.\n",
        encoding="utf-8",
    )


def sandbox_command(
    execution_root: Path,
    custody_root: Path,
    output_root: Path,
    opening_token: Path,
    preflight_only: bool,
) -> list[str]:
    bwrap = shutil.which("bwrap")
    if bwrap is None:
        raise ValueError("bubblewrap is required for Harvard custody")
    command = [
        bwrap,
        "--die-with-parent",
        "--unshare-all",
        "--new-session",
        "--ro-bind",
        "/",
        "/",
        "--dev",
        "/dev",
        "--proc",
        "/proc",
        "--tmpfs",
        "/tmp",
        "--bind",
        str(custody_root),
        str(custody_root),
        "--bind",
        str(output_root),
        str(output_root),
        str(ROOT / ".venv/bin/python"),
        str(Path(__file__).resolve()),
        "--sandboxed",
        "--execution-root",
        str(execution_root),
        "--custody-root",
        str(custody_root),
        "--holdout-output-root",
        str(output_root),
        "--opening-token",
        str(opening_token),
    ]
    if preflight_only:
        command.append("--preflight-only")
    return command


def require_disjoint_writable_roots(
    execution_root: Path, custody_root: Path, output_root: Path
) -> None:
    attempt_root = execution_root.parent
    protected = {
        "repository": ROOT.resolve(),
        "Harvard": HARVARD.resolve(),
        "execution": execution_root,
        "calibration attempt": attempt_root,
    }
    writable = {"custody": custody_root, "holdout output": output_root}

    def overlap(left: Path, right: Path) -> bool:
        return left == right or left in right.parents or right in left.parents

    for writable_name, writable_path in writable.items():
        for protected_name, protected_path in protected.items():
            if overlap(writable_path, protected_path):
                raise ValueError(
                    f"{writable_name} root overlaps protected {protected_name} root"
                )
    if overlap(custody_root, output_root):
        raise ValueError("custody and holdout output roots overlap")


def run_sandboxed(
    execution_root: Path,
    custody_root: Path,
    output_root: Path,
    token: Path,
    preflight_only: bool,
) -> int:
    attempt_root = execution_root.parent
    calibration = (
        attempt_root / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"
    )
    output_artifacts = output_root / "artifacts"
    executor = attempt_root / "cargo-target/release"
    digest = preflight(
        execution_root, custody_root, calibration, output_artifacts, token
    )
    if preflight_only:
        print(f"PASS holdout preflight sealed digest={digest}")
        return 0
    command = opening_command(
        execution_root, custody_root, output_root, token
    )
    create_token(token, digest, command)
    try:
        validate_harvard_after_open(
            calibration / "harvard-expected-input-manifest.csv"
        )
        holdout_root = output_root / "objects"
        holdout_root.mkdir(exist_ok=False)
        trace = holdout_root / "harvard-gsi.bin"
        identity = holdout_root / "harvard-gsi-identity.csv"
        producer = [
            str(executor / "holdout-producer"),
            "--configs",
            str(calibration / "candidate-configurations.csv"),
            "--accepted",
            str(calibration / "accepted-calibration-ensemble.csv"),
            "--climate",
            str(HARVARD / "p6.cli"),
            "--trace",
            str(trace),
            "--identity",
            str(identity),
        ]
        subprocess.run(producer, cwd=ROOT, check=True)
        observations = holdout_root / "harvard-observation-components.csv"
        annual = holdout_root / "harvard-annual-components.csv"
        results = output_artifacts / "harvard-holdout-results.csv"
        reconstruct = [
            str(executor / "holdout-reconstruct"),
            "--trace",
            str(trace),
            "--calendar",
            str(trace.with_suffix(".calendar.csv")),
            "--identity",
            str(identity),
            "--accepted",
            str(calibration / "accepted-calibration-ensemble.csv"),
            "--observations",
            str(TIMING),
            "--observation-out",
            str(observations),
            "--annual-out",
            str(annual),
            "--result-out",
            str(results),
        ]
        subprocess.run(reconstruct, cwd=ROOT, check=True)
        receipt = output_artifacts / "holdout-execution-receipt.csv"
        row = {
            "state": "PASS_SCORED_NO_REFIT",
            "freeze_digest": digest,
            "token_sha256": sha256_file(token),
            "expected_input_manifest_sha256": sha256_file(
                calibration / "harvard-expected-input-manifest.csv"
            ),
            "accepted_ensemble_sha256": sha256_file(
                calibration / "accepted-calibration-ensemble.csv"
            ),
            "trace_sha256": sha256_file(trace),
            "trace_identity_sha256": sha256_file(identity),
            "calendar_sha256": sha256_file(trace.with_suffix(".calendar.csv")),
            "observation_components_sha256": sha256_file(observations),
            "annual_components_sha256": sha256_file(annual),
            "results_sha256": sha256_file(results),
            "producer_command": " ".join(producer),
            "reconstructor_command": " ".join(reconstruct),
            "holdout_script_sha256": sha256_file(Path(__file__)),
        }
        with receipt.open("x", newline="", encoding="utf-8") as stream:
            writer = csv.DictWriter(stream, fieldnames=list(row), lineterminator="\n")
            writer.writeheader()
            writer.writerow(row)
            stream.flush()
            os.fsync(stream.fileno())
        (output_artifacts / "holdout-opening-record.md").write_text(
            "# Holdout Opening Record\n\n"
            "State: `SCORED_NO_REFIT`\n\n"
            "Evidence class: `Ran`\n\n"
            f"- Freeze digest: `{digest}`\n"
            f"- Accepted ensemble SHA-256: `{row['accepted_ensemble_sha256']}`\n"
            f"- Holdout trace SHA-256: `{row['trace_sha256']}`\n"
            f"- Holdout trace identity SHA-256: `{row['trace_identity_sha256']}`\n"
            f"- Observation components SHA-256: `{row['observation_components_sha256']}`\n"
            f"- Annual components SHA-256: `{row['annual_components_sha256']}`\n"
            f"- Results SHA-256: `{sha256_file(results)}`\n\n"
            "The exclusive token was durable before the first Harvard content read. "
            "Calibration inputs and the repository were mounted read-only.\n",
            encoding="utf-8",
        )
        print(f"PASS holdout opened once digest={digest}")
        return 0
    except Exception:
        write_incomplete(output_artifacts, digest)
        raise


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    parser.add_argument("--custody-root", type=Path, required=True)
    parser.add_argument("--holdout-output-root", type=Path, required=True)
    parser.add_argument("--opening-token", type=Path)
    parser.add_argument("--preflight-only", action="store_true")
    parser.add_argument("--sandboxed", action="store_true", help=argparse.SUPPRESS)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    custody_root = options.custody_root.resolve(strict=True)
    output_root = options.holdout_output_root.resolve(strict=True)
    token = (options.opening_token or custody_root / "holdout-opened-once.lock").resolve(
        strict=False
    )
    if not execution_root.is_dir() or not custody_root.is_dir():
        raise ValueError("execution and custody roots must exist")
    if not output_root.is_dir() or any(output_root.iterdir()):
        raise ValueError("holdout output root must be an existing empty directory")
    require_disjoint_writable_roots(execution_root, custody_root, output_root)
    try:
        token.relative_to(custody_root)
    except ValueError:
        raise ValueError("opening token escapes custody root") from None
    if options.sandboxed:
        if not options.preflight_only:
            (output_root / "artifacts").mkdir(exist_ok=False)
        return run_sandboxed(
            execution_root,
            custody_root,
            output_root,
            token,
            options.preflight_only,
        )
    subprocess.run(
        sandbox_command(
            execution_root,
            custody_root,
            output_root,
            token,
            options.preflight_only,
        ),
        cwd=ROOT,
        check=True,
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
