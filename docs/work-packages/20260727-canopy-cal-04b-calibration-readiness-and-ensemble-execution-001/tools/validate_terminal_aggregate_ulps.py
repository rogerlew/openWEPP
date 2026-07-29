#!/usr/bin/env python3
"""Post-freeze exact-rank audit of CAL-04B aggregate objective differences."""

from __future__ import annotations

import argparse
import csv
import math
import struct
import sys
from collections import Counter, defaultdict
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1]


def ordered_bits(value: float) -> int:
    """Map finite IEEE-754 binary64 values to monotonic integer ranks."""

    if not math.isfinite(value):
        raise ValueError("ULP rank requires a finite value")
    bits = struct.unpack(">Q", struct.pack(">d", value))[0]
    sign = 1 << 63
    mask = (1 << 64) - 1
    return (~bits & mask) if bits & sign else bits | sign


def ulp_steps(left: float, right: float) -> int:
    if left == right:
        return 0
    return abs(ordered_bits(left) - ordered_bits(right))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    if not execution_root.is_dir():
        raise ValueError("execution root must be an existing directory")
    artifacts = (
        execution_root.parent
        / "publication"
        / PACKAGE.relative_to(PACKAGE.parents[2])
        / "artifacts"
    )
    with (artifacts / "candidate-ledger.csv").open(
        newline="", encoding="utf-8"
    ) as stream:
        finite = {
            row["candidate_id"]: float(row["objective"])
            for row in csv.DictReader(stream)
            if row["state"] == "FINITE"
        }
    annual: dict[str, list[float]] = defaultdict(list)
    with (execution_root / "primary/candidate-annual-components.csv").open(
        newline="", encoding="utf-8"
    ) as stream:
        for row in csv.DictReader(stream):
            if row["candidate_id"] in finite:
                value = float(row["annual_mse"])
                if not math.isfinite(value):
                    raise ValueError(
                        f"finite candidate has nonfinite annual MSE {row['candidate_id']}"
                    )
                annual[row["candidate_id"]].append(value)
    if set(annual) != set(finite):
        raise ValueError("finite candidate/annual inventory differs")
    histogram: Counter[int] = Counter()
    for candidate_id, observed in finite.items():
        values = annual[candidate_id]
        if len(values) != 36:
            raise ValueError(f"annual cardinality differs for {candidate_id}")
        reconstructed = math.sqrt(sum(values) / len(values))
        steps = ulp_steps(observed, reconstructed)
        if steps > 4:
            raise ValueError(
                f"aggregate exceeds four IEEE-754 steps for {candidate_id}: {steps}"
            )
        histogram[steps] += 1
    expected = {0: 986, 1: 576, 2: 35, 3: 1}
    if dict(sorted(histogram.items())) != expected:
        raise ValueError(f"aggregate ULP histogram differs: {dict(histogram)}")
    print(
        "PASS terminal aggregate exact-rank audit "
        f"finite={len(finite)} histogram={dict(sorted(histogram.items()))}"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
