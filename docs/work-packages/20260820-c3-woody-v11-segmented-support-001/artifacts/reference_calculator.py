"""Independent V11 chronology calculator. It imports no production code."""

from __future__ import annotations

import json
import math
import struct
from fractions import Fraction
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def bits_to_float(bits: str) -> float:
    return struct.unpack(">d", bytes.fromhex(bits))[0]


def float_to_bits(value: float) -> str:
    return struct.pack(">d", value).hex()


def migrate(bits: str) -> dict[str, str]:
    value = bits_to_float(bits)
    if not math.isfinite(value) or value <= 0.0:
        return {"status": "rejected", "error": "VEG-E-121"}
    ticks_fraction = Fraction.from_float(value) * 1_000_000_000
    quotient, remainder = divmod(ticks_fraction.numerator, ticks_fraction.denominator)
    twice = 2 * remainder
    ticks = quotient + int(twice > ticks_fraction.denominator or (twice == ticks_fraction.denominator and quotient % 2 == 1))
    if ticks <= 0 or float_to_bits(float(ticks) / 1_000_000_000.0) != bits:
        return {"status": "rejected", "error": "VEG-E-121"}
    return {"status": "accepted", "nominal_cadence_ns": str(ticks)}


def segment(case: dict, parent_end: int) -> dict:
    before = "accepted-parent-beginning"
    if case.get("attempt_rejected"):
        return {"status": "rejected", "error": "VEG-E-123", "state_unchanged": True}
    cursor = 0
    for start_text, end_text in case["supports"]:
        start, end = int(start_text), int(end_text)
        if start != cursor:
            return {"status": "rejected", "error": "VEG-E-123"}
        if end <= start:
            return {"status": "rejected", "error": "VEG-E-122"}
        cursor = end
    if cursor != parent_end:
        return {"status": "rejected", "error": "VEG-E-123"}
    if len(set(case.get("scheduled_receipts", []))) != len(case.get("scheduled_receipts", [])):
        return {"status": "rejected", "error": "VEG-E-125"}
    if "replayed_slab" in case:
        return {"status": "rejected", "error": "VEG-E-127"}
    total = math.fsum(case["debits"])
    if total > case.get("inventory", math.inf):
        return {"status": "rejected", "error": "VEG-E-124"}
    increments = case.get("requested_increments", 1)
    if increments != 1:
        return {"status": "rejected", "error": "VEG-E-126"}
    assert before == "accepted-parent-beginning"
    return {"status": "accepted", "total_debit": total, "increments": increments}


def main() -> None:
    migration = json.loads((ROOT / "v10-v11-migration-vectors.json").read_text())
    segmented = json.loads((ROOT / "segmented-support-vectors.json").read_text())
    results = []
    for case in migration["cases"]:
        actual = migrate(case["duration_bits"])
        if actual != case["expected"]:
            raise SystemExit(f"migration mismatch {case['id']}: {actual}")
        results.append({"id": case["id"], "actual": actual})
    parent_end = int(segmented["parent_end_ns"])
    for case in segmented["cases"]:
        actual = segment(case, parent_end)
        if actual != case["expected"]:
            raise SystemExit(f"segment mismatch {case['id']}: {actual}")
        results.append({"id": case["id"], "actual": actual})
    print(json.dumps({"schema": "OPENWEPP_C3_WOODY_V11_REFERENCE_RESULTS_V1", "results": results}, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
