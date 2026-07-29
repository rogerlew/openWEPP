#!/usr/bin/env python3
"""Compress and checksum-bind the reconstructed CAL-04B raw trace."""

from __future__ import annotations

import argparse
import csv
import hashlib
import os
import struct
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
SOURCE_ARTIFACTS = PACKAGE / "artifacts"
ARTIFACTS = SOURCE_ARTIFACTS
OBJECTS = Path("/nonexistent/cal04b-execution-root-required")
RAW = OBJECTS / "hubbard-gsi.bin"
IDENTITY = OBJECTS / "hubbard-gsi-identity.csv"
COMPRESSED = OBJECTS / "hubbard-gsi.bin.zst"
RECEIPT = ARTIFACTS / "trace-retention.csv"
PARTIAL = OBJECTS / "hubbard-gsi.bin.zst.partial"
MAGIC = b"CAL04B03"
HEADER = struct.Struct("<8sIII")
CANDIDATES = 9_261
LANES = 9
YEARS = 36
DAYS_PER_YEAR = 180
DAYS_PER_LANE = YEARS * DAYS_PER_YEAR
VALUE_COUNT = CANDIDATES * LANES * DAYS_PER_LANE
RAW_BYTES = HEADER.size + VALUE_COUNT * 8


def sha(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def identity() -> dict[str, str]:
    with IDENTITY.open(newline="", encoding="utf-8") as stream:
        reader = csv.DictReader(stream)
        if tuple(reader.fieldnames or ()) != ("field", "value"):
            raise ValueError("trace identity schema differs")
        rows = list(reader)
    if not rows:
        raise ValueError("trace identity schema differs")
    result = {row["field"]: row["value"] for row in rows}
    if len(result) != len(rows):
        raise ValueError("trace identity fields duplicate")
    expected = {
        "schema": "CAL04B03",
        "candidate_count": str(CANDIDATES),
        "lane_count": str(LANES),
        "days_per_lane": str(DAYS_PER_LANE),
        "retained_days_per_plot_year": str(DAYS_PER_YEAR),
        "first_year": "1989",
        "last_year": "2024",
        "trace_order": "candidate_lane_year_yday",
    }
    if any(result.get(key) != value for key, value in expected.items()):
        raise ValueError("CAL04B03 trace identity differs")
    if (
        result.get("trace_path") != str(RAW)
        or result.get("trace_sha256") != sha(RAW)
        or result.get("trace_bytes") != str(RAW_BYTES)
    ):
        raise ValueError("raw trace does not match producer identity")
    return result


def validate_raw_header() -> None:
    with RAW.open("rb") as stream:
        header = stream.read(HEADER.size)
    if len(header) != HEADER.size:
        raise ValueError("raw trace header is truncated")
    if HEADER.unpack(header) != (MAGIC, CANDIDATES, LANES, DAYS_PER_LANE):
        raise ValueError("CAL04B03 trace header/cardinality differs")
    if RAW.stat().st_size != RAW_BYTES:
        raise ValueError("CAL04B03 trace byte cardinality differs")


def reconstruction_receipt(directory: Path, receipt_name: str) -> dict[str, str]:
    path = directory / receipt_name
    with path.open(newline="", encoding="utf-8") as stream:
        reader = csv.DictReader(stream)
        if tuple(reader.fieldnames or ()) != ("field", "value"):
            raise ValueError(f"reconstruction receipt schema differs: {path}")
        rows = list(reader)
    result = {row["field"]: row["value"] for row in rows}
    if len(result) != len(rows) or result.get("state") != "PASS":
        raise ValueError(f"reconstruction receipt is not unique/PASS: {path}")
    return result


def validate_dual_reconstruction(raw_hash: str) -> None:
    primary = reconstruction_receipt(OBJECTS / "primary", "reconstruction-receipt.csv")
    verification = reconstruction_receipt(
        OBJECTS / "verification", "verification-receipt.csv"
    )
    for field, name in (
        ("crossing_components_sha256", "candidate-crossing-components.csv"),
        ("observation_components_sha256", "candidate-observation-components.csv"),
        ("annual_components_sha256", "candidate-annual-components.csv"),
        ("diagnostics_sha256", "candidate-diagnostics.csv"),
    ):
        primary_hash = sha(OBJECTS / "primary" / name)
        verification_hash = sha(OBJECTS / "verification" / name)
        if (
            primary.get(field) != primary_hash
            or verification.get(field) != verification_hash
            or primary_hash != verification_hash
        ):
            raise ValueError(f"dual reconstruction differs for {name}")
    for field, name in (
        ("candidate_ledger_sha256", "candidate-ledger.csv"),
        ("accepted_ensemble_sha256", "accepted-calibration-ensemble.csv"),
        ("failure_ledger_sha256", "failure-ledger.csv"),
    ):
        package_hash = sha(ARTIFACTS / name)
        verification_hash = sha(OBJECTS / "verification" / name)
        if verification.get(field) != verification_hash or package_hash != verification_hash:
            raise ValueError(f"round-trip reconstruction differs for {name}")
    if primary.get("trace_sha256") != raw_hash or verification.get("trace_sha256") != raw_hash:
        raise ValueError("dual reconstruction trace identity differs")


def decompressed_identity(path: Path) -> tuple[str, int]:
    process = subprocess.Popen(["zstd", "-dc", str(path)], stdout=subprocess.PIPE)
    if process.stdout is None:
        raise OSError("zstd stdout unavailable")
    digest = hashlib.sha256()
    expanded_bytes = 0
    for block in iter(lambda: process.stdout.read(1024 * 1024), b""):
        digest.update(block)
        expanded_bytes += len(block)
    if process.wait() != 0:
        raise subprocess.SubprocessError("zstd decompression verification failed")
    return digest.hexdigest(), expanded_bytes


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    if not execution_root.is_dir():
        raise ValueError("execution root must be an existing directory")
    global ARTIFACTS, OBJECTS, RAW, IDENTITY, COMPRESSED, RECEIPT, PARTIAL
    ARTIFACTS = execution_root.parent / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    OBJECTS = execution_root
    RAW = OBJECTS / "hubbard-gsi.bin"
    IDENTITY = OBJECTS / "hubbard-gsi-identity.csv"
    COMPRESSED = OBJECTS / "hubbard-gsi.bin.zst"
    RECEIPT = ARTIFACTS / "trace-retention.csv"
    PARTIAL = OBJECTS / "hubbard-gsi.bin.zst.partial"
    if COMPRESSED.exists() or RECEIPT.exists() or PARTIAL.exists():
        raise ValueError("trace retention output already exists")
    trace_identity = identity()
    validate_raw_header()
    raw_hash = trace_identity["trace_sha256"]
    validate_dual_reconstruction(raw_hash)
    command = ["zstd", "-T1", "-19", "--keep", str(RAW), "-o", str(PARTIAL)]
    subprocess.run(command, check=True)
    expanded_hash, expanded_bytes = decompressed_identity(PARTIAL)
    if expanded_hash != raw_hash or expanded_bytes != RAW_BYTES:
        raise ValueError("compressed trace does not reconstruct the raw identity")
    os.replace(PARTIAL, COMPRESSED)
    row = {
        "schema": "CAL04B03",
        "value_count": VALUE_COUNT,
        "raw_path": str(RAW),
        "raw_bytes": RAW.stat().st_size,
        "raw_sha256": raw_hash,
        "compressed_path": str(COMPRESSED),
        "compressed_bytes": COMPRESSED.stat().st_size,
        "compressed_sha256": sha(COMPRESSED),
        "decompressed_sha256": expanded_hash,
        "decompressed_bytes": expanded_bytes,
        "producer_identity_sha256": sha(IDENTITY),
        "compression_command": " ".join(command),
        "exact_command": (
            f"PYTHONDONTWRITEBYTECODE=1 {ROOT / '.venv/bin/python'} "
            f"{Path(__file__).resolve()} "
            f"--execution-root {OBJECTS}"
        ),
        "source_sha256": sha(Path(__file__)),
        "state": "PASS",
    }
    with RECEIPT.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(row), lineterminator="\n")
        writer.writeheader()
        writer.writerow(row)
    if (
        trace_identity["trace_bytes"] != str(row["raw_bytes"])
        or row["raw_bytes"] != RAW_BYTES
    ):
        raise ValueError("retention raw byte count differs from producer identity")
    print(
        f"PASS raw_bytes={row['raw_bytes']} compressed_bytes={row['compressed_bytes']}"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
