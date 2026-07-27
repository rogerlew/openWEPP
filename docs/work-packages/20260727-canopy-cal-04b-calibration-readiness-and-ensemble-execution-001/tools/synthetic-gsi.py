#!/usr/bin/env python3
"""Run the raw synthetic GSI producer and two independent reconstructors."""

from __future__ import annotations

import argparse
import csv
import hashlib
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
SOURCE_ARTIFACTS = PACKAGE / "artifacts"
ARTIFACTS = SOURCE_ARTIFACTS
OBJECTS = Path("/nonexistent/cal04b-execution-root-required")
BIN = Path("/nonexistent/cal04b-execution-root-required")
TRACE = OBJECTS / "synthetic-gsi.bin"
IDENTITY = OBJECTS / "synthetic-gsi-identity.csv"
PRIMARY = OBJECTS / "synthetic-primary"
VERIFICATION = OBJECTS / "synthetic-verification"
RESULT = ARTIFACTS / "synthetic-recovery-results.csv"
RESULT_HEADER = "case_id,stage,true_configuration,recovered_set,status,evidence\n"


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def receipt(path: Path) -> dict[str, str]:
    with path.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    if not rows or set(rows[0]) != {"field", "value"}:
        raise ValueError(f"malformed receipt: {path}")
    values = {row["field"]: row["value"] for row in rows}
    if len(values) != len(rows):
        raise ValueError(f"duplicate receipt field: {path}")
    return values


def run(argv: list[str]) -> None:
    subprocess.run(argv, cwd=PACKAGE.parents[2], check=True)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    if not execution_root.is_dir():
        raise ValueError("execution root must be an existing directory")
    global ARTIFACTS, OBJECTS, BIN, TRACE, IDENTITY, PRIMARY, VERIFICATION, RESULT
    ARTIFACTS = execution_root.parent / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    OBJECTS = execution_root
    BIN = execution_root.parent / "cargo-target/release"
    TRACE = OBJECTS / "synthetic-gsi.bin"
    IDENTITY = OBJECTS / "synthetic-gsi-identity.csv"
    PRIMARY = OBJECTS / "synthetic-primary"
    VERIFICATION = OBJECTS / "synthetic-verification"
    RESULT = ARTIFACTS / "synthetic-recovery-results.csv"
    outputs = [TRACE, IDENTITY, PRIMARY, VERIFICATION]
    existing = [path for path in outputs if path.exists()]
    if existing:
        raise ValueError(f"refusing existing synthetic outputs: {existing}")
    if RESULT.exists() and RESULT.read_text(encoding="utf-8") != RESULT_HEADER:
        raise ValueError(f"refusing populated synthetic result: {RESULT}")
    configs = ARTIFACTS / "candidate-configurations.csv"
    design = SOURCE_ARTIFACTS / "synthetic-gsi-design.csv"
    if not configs.is_file() or not design.is_file():
        raise ValueError("synthetic inputs are absent")
    with design.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    if len(rows) != 1:
        raise ValueError("synthetic design must contain exactly one case")
    row = rows[0]
    expected = {
        "case_id": "SYN-GSI-01",
        "hidden_candidate": "GSI-5557",
        "candidate_set": "GSI-0001|GSI-5557|GSI-9261",
        "start_date": "2001-01-01",
        "end_date": "2003-12-31",
        "latitude_degrees": "44.27",
        "stage": "PRE_HUBBARD",
    }
    for field, value in expected.items():
        if row.get(field) != value:
            raise ValueError(f"synthetic design {field} differs")

    run(
        [
            str(BIN / "synthetic-trace"),
            "--configs",
            str(configs),
            "--trace",
            str(TRACE),
            "--identity",
            str(IDENTITY),
        ]
    )
    run(
        [
            str(BIN / "synthetic-reconstruct"),
            "--trace",
            str(TRACE),
            "--identity",
            str(IDENTITY),
            "--configs",
            str(configs),
            "--out",
            str(PRIMARY),
        ]
    )
    run(
        [
            str(BIN / "synthetic-verify-reconstruct"),
            "--trace",
            str(TRACE),
            "--identity",
            str(IDENTITY),
            "--configs",
            str(configs),
            "--primary",
            str(PRIMARY),
            "--out",
            str(VERIFICATION),
        ]
    )
    primary_path = PRIMARY / "primary-reconstruction-receipt.csv"
    verification_path = VERIFICATION / "verification-reconstruction-receipt.csv"
    primary = receipt(primary_path)
    verification = receipt(verification_path)
    common = (
        "state",
        "case_id",
        "trace_sha256",
        "hidden_candidate",
        "hidden_objective",
        "recovered_set",
        "nonvacuous_competitor",
        "components_sha256",
        "annual_sha256",
        "candidate_ledger_sha256",
        "accepted_ensemble_sha256",
    )
    if any(primary.get(field) != verification.get(field) for field in common):
        raise ValueError("synthetic reconstruction receipts differ")
    if (
        primary.get("state") != "PASS"
        or primary.get("hidden_candidate") != "GSI-5557"
        or primary.get("hidden_objective") != "0.000000000000"
        or "GSI-5557" not in primary.get("recovered_set", "").split("|")
        or primary.get("nonvacuous_competitor") != "TRUE"
        or verification.get("exact_primary_match") != "TRUE"
        or primary.get("trace_sha256") != digest(TRACE)
    ):
        raise ValueError("synthetic recovery receipt acceptance failed")
    with RESULT.open("w", newline="", encoding="utf-8") as stream:
        fields = ["case_id", "stage", "true_configuration", "recovered_set", "status", "evidence"]
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerow(
            {
                "case_id": "SYN-GSI-01",
                "stage": "gsi_timing",
                "true_configuration": "GSI-5557",
                "recovered_set": primary["recovered_set"],
                "status": "PASS",
                "evidence": (
                    f"trace={TRACE}|trace_sha256={primary['trace_sha256']}|"
                    f"primary_receipt={primary_path}|verification_receipt={verification_path}|"
                    f"components={PRIMARY / 'candidate-observation-components.csv'}"
                ),
            }
        )
    print(f"PASS synthetic recovered={primary['recovered_set']} dual_receipts=2")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError, KeyError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
