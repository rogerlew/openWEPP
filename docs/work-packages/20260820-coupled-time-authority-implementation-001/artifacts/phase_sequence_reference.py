#!/usr/bin/env python3
"""Independent canonical V2 commit/crash/delivery/next-parent reference."""

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
committed = json.loads((ROOT / "semantic-schema-poisons.json").read_text())["baseline"]["restart"]


def framed(domain, fields):
    name = domain.encode()
    data = b"OPENWEPP\0" + (1).to_bytes(2, "big") + len(name).to_bytes(2, "big") + name
    for tag, value in fields:
        key = tag.encode()
        data += len(key).to_bytes(2, "big") + key + len(value).to_bytes(4, "big") + value
    return hashlib.sha256(data).hexdigest()


def reject(condition, label):
    if not condition:
        raise SystemExit(f"expected rejection: {label}")


if committed["checkpoint_phase"] != "CommittedParent" or len(committed["publication_outbox"]) != 1:
    raise SystemExit("fixture is not one durable committed parent")
current = int(committed["parent_transaction_sequence"])
next_sequence = int(committed["next_parent_transaction_sequence"])
if next_sequence != current + 1:
    raise SystemExit("commit did not consume sequence exactly once")
# Reconstruct both durable identities independently.
slabs = committed["accepted_slab_receipts"]
events = sorted(committed["accepted_event_receipts"], key=lambda row: row["event_ordinal"])
scheduled = committed["scheduled_once_receipts"]
parent_receipt = framed("parent-receipt-v2", [
    ("parent_transaction_id", bytes.fromhex(committed["parent_transaction_id"])),
    ("parent_interval_id", bytes.fromhex(committed["parent_interval_id"])),
    ("begin_owner_set", bytes.fromhex(committed["begin_complete_owner_set_sha256"])),
    ("end_owner_set", bytes.fromhex(committed["accepted_complete_owner_set_sha256"])),
    ("ordered_slab_receipts", b"".join(bytes.fromhex(row["receipt_id"]) for row in slabs)),
    ("ordered_event_receipts", b"".join(bytes.fromhex(row["receipt_id"]) for row in events)),
    ("ordered_scheduled_receipts", b"".join(bytes.fromhex(row["receipt_id"]) for row in scheduled)),
])
publication = framed("publication-receipt-v2", [
    ("parent_receipt_id", bytes.fromhex(parent_receipt)),
    ("ordered_output_record_ids", b""),
    ("outbox_sequence", current.to_bytes(16, "big")),
    ("outbox_state", b"CommittedUndelivered"),
])
row0 = committed["publication_outbox"][0]
if (row0["parent_receipt_id"], row0["publication_receipt_id"]) != (parent_receipt, publication):
    raise SystemExit("durable receipt reconstruction mismatch")

# Execute active -> atomic committed transition from a canonical active snapshot.
active = json.loads(json.dumps(committed, sort_keys=True, separators=(",", ":")))
active["checkpoint_phase"] = "ActiveParent"
active["next_parent_transaction_sequence"] = active["parent_transaction_sequence"]
active["publication_outbox"] = []
if active["publication_outbox"] or active["next_parent_transaction_sequence"] != active["parent_transaction_sequence"]:
    raise SystemExit("invalid active snapshot")
executed_commit = json.loads(json.dumps(active, sort_keys=True, separators=(",", ":")))
executed_commit["checkpoint_phase"] = "CommittedParent"
executed_commit["next_parent_transaction_sequence"] = str(current + 1)
executed_commit["publication_outbox"] = [row0]
if executed_commit != committed:
    raise SystemExit("atomic commit did not reproduce frozen committed checkpoint")
reject(executed_commit["checkpoint_phase"] != "ActiveParent", "recommit")
wire = json.dumps(committed, sort_keys=True, separators=(",", ":"))
restored = json.loads(wire)
if json.dumps(restored, sort_keys=True, separators=(",", ":")) != wire:
    raise SystemExit("canonical committed restore drift")
row = restored["publication_outbox"][0]
identity = (row["parent_receipt_id"], row["publication_receipt_id"], row["outbox_sequence"])
if row["state"] != "CommittedUndelivered":
    raise SystemExit("wrong committed delivery state")
row["state"] = "DeliveredUnacknowledged"
row["delivery_attempt_count"] = str(int(row["delivery_attempt_count"]) + 1)
delivered = json.loads(json.dumps(restored, sort_keys=True, separators=(",", ":")))
if tuple(delivered["publication_outbox"][0][key] for key in ("parent_receipt_id", "publication_receipt_id", "outbox_sequence")) != identity:
    raise SystemExit("delivery changed publication identity")
delivered["publication_outbox"][0]["delivery_attempt_count"] = str(int(delivered["publication_outbox"][0]["delivery_attempt_count"]) + 1)
delivered["publication_outbox"][0]["state"] = "Acknowledged"
acknowledged = json.loads(json.dumps(delivered, sort_keys=True, separators=(",", ":")))
if int(acknowledged["next_parent_transaction_sequence"]) != next_sequence:
    raise SystemExit("delivery or restore replayed sequence increment")
reject(acknowledged["publication_outbox"][0]["state"] != "DeliveredUnacknowledged", "acknowledged redelivery")
next_parent = framed("parent-transaction", [
    ("run_id", bytes.fromhex(committed["run_identity_sha256"])),
    ("sequence", next_sequence.to_bytes(16, "big")),
    ("parent_interval_id", bytes.fromhex(committed["parent_interval_id"])),
    ("begin_owner_set", bytes.fromhex(committed["accepted_complete_owner_set_sha256"])),
])
if next_parent == committed["parent_transaction_id"]:
    raise SystemExit("next parent reused committed identity")
evidence = {"active": active, "committed": executed_commit, "restored": restored, "acknowledged": acknowledged, "next_parent_id": next_parent}
actual = hashlib.sha256(json.dumps(evidence, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
expected = "0b5b9be20d22de5139dd5b19d2aeb4430af917640149b7e04baeeef74e479642"
if actual != expected:
    raise SystemExit(f"phase fixture mismatch: {actual}")
print(actual)
