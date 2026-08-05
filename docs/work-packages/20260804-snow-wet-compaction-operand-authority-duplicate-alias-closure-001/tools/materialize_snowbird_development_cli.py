#!/usr/bin/env python3
"""Materialize the frozen development-only Snowbird precipitation lane."""

from __future__ import annotations

import argparse
from decimal import Decimal, ROUND_HALF_UP
import hashlib
import json
from pathlib import Path
import re


REPOSITORY_ROOT = Path(__file__).resolve().parents[4]
SOURCE = REPOSITORY_ROOT / "tests/fixtures/snotel_observed/snotel_snowbird_ut/p8.cli"
DESTINATION_DIR = (
    SOURCE.parent / "development/precip_x1p2155576"
)
DESTINATION = DESTINATION_DIR / "p8.cli"
MANIFEST = DESTINATION_DIR / "manifest.json"
SOURCE_SHA256 = "10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7"
FACTOR = Decimal("1.2155576")
QUANTUM_MM = Decimal("0.1")
DATA_ROW = re.compile(r"^(\s*\d+\s+\d+\s+\d+)(\s+)(\d+\.\d)(.*)$")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def materialize(source_bytes: bytes) -> tuple[bytes, dict[str, object]]:
    source_sha256 = sha256_bytes(source_bytes)
    if source_sha256 != SOURCE_SHA256:
        raise ValueError(
            f"canonical Snowbird p8.cli hash {source_sha256} does not match frozen "
            f"identity {SOURCE_SHA256}"
        )

    transformed: list[str] = []
    data_started = False
    row_count = 0
    positive_row_count = 0
    changed_row_count = 0
    source_precip_tenths_mm = 0
    derived_precip_tenths_mm = 0
    for line in source_bytes.decode("ascii").splitlines(keepends=True):
        if line.strip().startswith("da mo year"):
            data_started = True
            transformed.append(line)
            continue
        if not data_started:
            transformed.append(line)
            continue

        newline = "\n" if line.endswith("\n") else ""
        body = line[:-1] if newline else line
        match = DATA_ROW.fullmatch(body)
        if match is None and row_count == 0 and body.strip().startswith("(mm)"):
            transformed.append(line)
            continue
        if match is None:
            raise ValueError(f"unrecognized CLIGEN daily row {row_count + 1}: {body!r}")
        source_precip = Decimal(match.group(3))
        derived_precip = (source_precip * FACTOR).quantize(
            QUANTUM_MM,
            rounding=ROUND_HALF_UP,
        )
        source_token = match.group(3)
        derived_token = f"{derived_precip:.1f}"
        separator_width = max(
            1,
            len(match.group(2)) + len(source_token) - len(derived_token),
        )
        transformed.append(
            f"{match.group(1)}{' ' * separator_width}{derived_token}{match.group(4)}{newline}"
        )
        row_count += 1
        source_tenths = int(source_precip * 10)
        derived_tenths = int(derived_precip * 10)
        source_precip_tenths_mm += source_tenths
        derived_precip_tenths_mm += derived_tenths
        positive_row_count += int(source_tenths > 0)
        changed_row_count += int(source_tenths != derived_tenths)

    derived_bytes = "".join(transformed).encode("ascii")
    metadata: dict[str, object] = {
        "schema": "openwepp-development-cli-transform-v1",
        "classification": "DEVELOPMENT_ONLY",
        "source": str(SOURCE.relative_to(REPOSITORY_ROOT)),
        "source_sha256": source_sha256,
        "derived": str(DESTINATION.relative_to(REPOSITORY_ROOT)),
        "derived_sha256": sha256_bytes(derived_bytes),
        "transformation": {
            "field": "daily precipitation only",
            "factor_decimal_exact": str(FACTOR),
            "rounding": "ROUND_HALF_UP",
            "output_resolution_mm": str(QUANTUM_MM),
            "other_fields": "byte-preserved apart from spacing needed to retain the precipitation column",
        },
        "daily_row_count": row_count,
        "positive_precipitation_row_count": positive_row_count,
        "changed_precipitation_row_count": changed_row_count,
        "source_precipitation_total_mm": f"{Decimal(source_precip_tenths_mm) / 10:.1f}",
        "derived_precipitation_total_mm": f"{Decimal(derived_precip_tenths_mm) / 10:.1f}",
        "consumer_protocol": (
            "Copy the canonical Snowbird fixture to a fresh run directory, then replace only "
            "the staged p8.cli with this derived p8.cli. Never overwrite the canonical fixture."
        ),
        "claim_limits": [
            "not precipitation truth",
            "not an observation",
            "not a calibration or default",
            "not independent validation of snow physics",
            "not transferable beyond the Snowbird development lane",
        ],
    }
    return derived_bytes, metadata


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="verify committed derivative and manifest instead of writing them",
    )
    args = parser.parse_args()

    source_bytes = SOURCE.read_bytes()
    derived_bytes, metadata = materialize(source_bytes)
    manifest_bytes = (json.dumps(metadata, indent=2, sort_keys=True) + "\n").encode()
    if args.check:
        if DESTINATION.read_bytes() != derived_bytes:
            raise ValueError(f"{DESTINATION} is not the deterministic derivative")
        if MANIFEST.read_bytes() != manifest_bytes:
            raise ValueError(f"{MANIFEST} is not the deterministic manifest")
        print(f"verified {DESTINATION.relative_to(REPOSITORY_ROOT)}")
        return 0

    DESTINATION_DIR.mkdir(parents=True, exist_ok=True)
    DESTINATION.write_bytes(derived_bytes)
    MANIFEST.write_bytes(manifest_bytes)
    print(json.dumps(metadata, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
