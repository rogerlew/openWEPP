#!/usr/bin/env python3
"""Atomically publish the bounded CAL-04B package result set."""

from __future__ import annotations

import argparse
import hashlib
import os
import shutil
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]

CALIBRATION_PUBLISHABLE = frozenset(
    {
        "accepted-calibration-ensemble.csv",
        "additional-data-inventory.csv",
        "calibration-readiness-matrix.md",
        "candidate-configurations.csv",
        "candidate-ledger.csv",
        "failure-ledger.csv",
        "freeze-verifier-receipts.csv",
        "gsi-domain-grid.csv",
        "harvard-expected-input-manifest.csv",
        "holdout-freeze-digest.txt",
        "holdout-freeze-manifest.csv",
        "identifiability-and-equifinality.md",
        "input-and-authority-manifest.csv",
        "later-stage-membership.csv",
        "later-stage-recovery.csv",
        "later-stage-results.csv",
        "native-consumer-proof.csv",
        "producer-failure-ledger.csv",
        "saturation-evidence.csv",
        "saturation-window-inventory.csv",
        "stage-status-ledger.csv",
        "synthetic-recovery-results.csv",
        "trace-retention.csv",
    }
)

HOLDOUT_PUBLISHABLE = frozenset(
    {
        "harvard-holdout-results.csv",
        "holdout-execution-receipt.csv",
        "holdout-opening-record.md",
        "holdout-validation-summary.md",
    }
)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def source_artifacts(execution_root: Path) -> Path:
    return (
        execution_root.parent
        / "publication"
        / PACKAGE.relative_to(ROOT)
        / "artifacts"
    )


def inventory(source: Path, publishable: frozenset[str]) -> list[Path]:
    if not source.is_dir():
        raise ValueError(f"publication source is missing: {source}")
    paths: list[Path] = []
    for name in sorted(publishable):
        path = source / name
        if path.exists():
            metadata = path.lstat()
            if path.is_symlink() or not path.is_file() or metadata.st_nlink != 1:
                raise ValueError(f"publication source is not a unique regular file: {path}")
            paths.append(path)
    unexpected = [
        path.name
        for path in source.iterdir()
        if path.is_file() and path.name not in publishable
    ]
    if unexpected:
        raise ValueError(f"unrecognized package result files: {sorted(unexpected)}")
    if not paths:
        raise ValueError("no bounded CAL-04B results are available to publish")
    return paths


def atomic_copy(source: Path, destination: Path) -> None:
    descriptor, temporary_name = tempfile.mkstemp(
        dir=destination.parent, prefix=f".{destination.name}.", suffix=".tmp"
    )
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "wb") as output, source.open("rb") as input_stream:
            shutil.copyfileobj(input_stream, output)
            output.flush()
            os.fsync(output.fileno())
        os.replace(temporary, destination)
        directory = os.open(destination.parent, os.O_RDONLY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        if temporary.exists():
            temporary.unlink()


def publish(
    execution_root: Path,
    *,
    holdout_output_root: Path | None = None,
    apply: bool,
    replace: bool,
) -> list[str]:
    sources = [(source_artifacts(execution_root), CALIBRATION_PUBLISHABLE)]
    if holdout_output_root is not None:
        sources.append(
            (holdout_output_root.resolve(strict=True) / "artifacts", HOLDOUT_PUBLISHABLE)
        )
    destination_root = PACKAGE / "artifacts"
    actions: list[str] = []
    for source, publishable in sources:
        for item in inventory(source, publishable):
            destination = destination_root / item.name
            if destination.exists() and destination.is_symlink():
                raise ValueError(f"publication destination is a symlink: {destination}")
            if destination.is_file() and sha256_file(destination) == sha256_file(item):
                actions.append(f"UNCHANGED {item.name}")
                continue
            if destination.exists() and not replace:
                raise ValueError(
                    f"publication would replace differing result without --replace: {destination}"
                )
            actions.append(f"{'REPLACE' if destination.exists() else 'CREATE'} {item.name}")
            if apply:
                atomic_copy(item, destination)
    return actions


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    parser.add_argument("--holdout-output-root", type=Path)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--replace", action="store_true")
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    if not execution_root.is_dir():
        raise ValueError("execution root must be an existing directory")
    actions = publish(
        execution_root,
        holdout_output_root=options.holdout_output_root,
        apply=options.apply,
        replace=options.replace,
    )
    mode = "APPLIED" if options.apply else "PLAN"
    print(f"{mode} bounded CAL-04B publication files={len(actions)}")
    for action in actions:
        print(action)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
