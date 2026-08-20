#!/usr/bin/env python3
"""Independent restored-parent finalization reconstruction; imports no Rust."""

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def framed(domain, fields):
    name = domain.encode()
    preimage = b"OPENWEPP\0" + (1).to_bytes(2, "big") + len(name).to_bytes(2, "big") + name
    for tag, value in fields:
        key = tag.encode()
        preimage += len(key).to_bytes(2, "big") + key + len(value).to_bytes(4, "big") + value
    return hashlib.sha256(preimage).hexdigest()


fixture = json.loads((ROOT / "semantic-schema-poisons.json").read_text())["baseline"]["restart"]
slabs = fixture["accepted_slab_receipts"]
events = sorted(fixture["accepted_event_receipts"], key=lambda row: row["event_ordinal"])
parent = framed("parent-receipt", [
    ("parent_transaction_id", bytes.fromhex(fixture["parent_transaction_id"])),
    ("parent_interval_id", bytes.fromhex(fixture["parent_interval_id"])),
    ("begin_owner_set", bytes.fromhex(slabs[0]["begin_owner_set_sha256"])),
    ("end_owner_set", bytes.fromhex(events[-1]["end_owner_set_sha256"])),
    ("ordered_slab_receipts", b"".join(bytes.fromhex(row["receipt_id"]) for row in slabs)),
    ("ordered_event_receipts", b"".join(bytes.fromhex(row["receipt_id"]) for row in events)),
])
publication = framed("publication-receipt", [
    ("parent_receipt_id", bytes.fromhex(parent)),
    ("ordered_output_records", b""),
    ("outbox_state", b"CommittedUndelivered"),
])
expected = {
    "parent_receipt_id": "90627286f5cc4b6e341f0162323606013f0c0d8f58b2dd17615459befd6cfda3",
    "publication_receipt_id": "5faa32af248f6d4badbb0d6b65cf075d18b25f3eaedd23a2d49e53f6ff574602",
}
actual = {"parent_receipt_id": parent, "publication_receipt_id": publication}
if actual != expected:
    raise SystemExit(f"restored finalization mismatch: {actual!r}")
print(json.dumps(actual, sort_keys=True, separators=(",", ":")))
