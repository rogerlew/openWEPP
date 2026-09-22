#!/usr/bin/env python3
"""Supplemental nonphysical framing smoke checks; NOT contract conformance.

The reduced calendar and synthetic digest operands exercise framing mechanics
only. Required full-payload known answers, all receipt domains, canonicalization,
source correspondence, and runtime rejection/rollback controls remain HOLD.
No instantiated provider, parent, admission guard, or simulation is tested.
"""
import hashlib
import json
import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parent
ORIGINAL = ROOT / "original-input-reference.json"
CONTINUOUS = ROOT / "continuous-fixtures-draft01.json"
ORIGINAL_SHA = "4b324b0a9c136f2203e8073ca260db5be9e7dd07722a37aec60f2cd528be2d54"
CONTINUOUS_SHA = "8cf391055e107ca854f72fb1f70a0e78298575ad0d2ede6fa6c9c760c18191e1"


def frame(domain, fields):
    data = b"OPENWEPP\0" + struct.pack(">H", 1) + struct.pack(">H", len(domain)) + domain.encode()
    for tag, value in fields:
        tag = tag.encode()
        data += struct.pack(">H", len(tag)) + tag + struct.pack(">I", len(value)) + value
    return hashlib.sha256(data).hexdigest()


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def main():
    require(hashlib.sha256(ORIGINAL.read_bytes()).hexdigest() == ORIGINAL_SHA, "original hash")
    require(hashlib.sha256(CONTINUOUS.read_bytes()).hexdigest() == CONTINUOUS_SHA, "continuous hash")
    continuous = json.loads(CONTINUOUS.read_text())
    require(continuous["base_input_sha256"] == ORIGINAL_SHA, "base source correspondence")
    require(continuous["support_s"] == 60 and continuous["duration_s"] == 259200, "fixed support")
    for cycle in continuous["cycles"]:
        segments = cycle["segments"]
        require(segments[0]["start_s"] == 0 and segments[-1]["end_s"] == 259200, "cycle coverage")
        require(all(a["end_s"] == b["start_s"] for a, b in zip(segments, segments[1:])), "segment order")
    require(continuous["cycles"][0]["segments"][0]["end_s"] == 60, "first breakpoint")

    run = frame("openwepp-m1-fixed-sequence-run-v1", [
        ("schema_version", struct.pack(">I", 1)),
        ("provider_id", b"OPENWEPP_COLD_M1_FIXED_SEQUENCE_PROVIDER_V1"),
        ("calendar", b'{"origin":"2000-06-20T00:00:00+00:00"}\n'),
        ("original_source_sha256", bytes.fromhex(ORIGINAL_SHA)),
        ("continuous_source_sha256", bytes.fromhex(CONTINUOUS_SHA)),
        ("implementation_version", b"control-v1"), ("implementation_path", b"controls"),
        ("implementation_sha256", bytes(32)), ("payload_content_sha256", bytes.fromhex("11" * 32)),
    ])
    require(run == "f5ef6f92ee912f666ce7595085e82cc219d76ef7772058551b920563a071de69", "run KAT")
    gsi = frame("openwepp-m1-fixed-sequence-gsi-support-v1", [
        ("run_identity", bytes.fromhex(run)), ("cycle_ordinal", struct.pack(">I", 0)),
        ("cycle_id", b"original_liquid_cold_thaw_refreeze_warm"), ("support_ordinal", struct.pack(">I", 0)),
        ("support_start_ns", (0).to_bytes(16, "big")), ("support_end_ns", (60_000_000_000).to_bytes(16, "big")),
        ("gsi_bits", struct.pack(">Q", 0x3FF0000000000000)), ("gsi_source_join_content_sha256", bytes.fromhex("22" * 32)),
    ])
    require(gsi == "ad28bba9b5d066ad890b33245841c5109a2fc3caac4525f1c437d2ec91f4fa4f", "GSI KAT")

    receipts = [(i, i * 60_000_000_000, (i + 1) * 60_000_000_000) for i in range(30)]
    require(len(receipts) == 30 and receipts[0][1] == 0 and receipts[-1][2] == 1_800_000_000_000, "parent slice")
    print("PASS supplemental framing smoke checks; contract controls remain HOLD")



if __name__ == "__main__":
    main()
