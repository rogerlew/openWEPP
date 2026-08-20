#!/usr/bin/env python3
"""Independent V1 semantic validator and poison runner; imports no openWEPP code."""

import argparse
import base64
import hashlib
import json
import re
import struct
from pathlib import Path

U128_MAX = 340282366920938463463374607431768211455
MAX_OWNERS = 4096
MAX_RECEIPTS = 65536
ARTIFACTS = Path(__file__).resolve().parent


class Invalid(ValueError):
    pass


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def u128(value, name):
    if not isinstance(value, str) or (value != "0" and (not value.isdigit() or value[0] == "0")):
        raise Invalid(f"{name}:noncanonical-u128")
    parsed = int(value)
    if parsed > U128_MAX:
        raise Invalid(f"{name}:u128-overflow")
    return parsed


def ordered_unique(values, key, limit, name):
    if not isinstance(values, list) or len(values) > limit:
        raise Invalid(f"{name}:collection-bound")
    keys = [key(v).encode("utf-8") for v in values]
    if any(a >= b for a, b in zip(keys, keys[1:])):
        raise Invalid(f"{name}:noncanonical-order-or-duplicate")


def sha(value, name):
    if not isinstance(value, str) or len(value) != 64 or any(c not in "0123456789abcdef" for c in value):
        raise Invalid(f"{name}:invalid-sha256")


def support(value, name="support"):
    if not isinstance(value, dict) or set(value) != {"start_ns", "end_ns"}:
        raise Invalid(f"{name}:closed-shape")
    start, end = u128(value["start_ns"], f"{name}.start"), u128(value["end_ns"], f"{name}.end")
    if start >= end:
        raise Invalid(f"{name}:nonpositive")
    return start, end


def decoded_bytes(item, prefix):
    try:
        data = base64.b64decode(item[f"{prefix}_base64"], validate=True)
    except Exception as exc:
        raise Invalid(f"{prefix}:invalid-base64") from exc
    sha(item[f"{prefix}_sha256"], f"{prefix}.digest")
    if digest(data) != item[f"{prefix}_sha256"]:
        raise Invalid(f"{prefix}:digest-mismatch")
    return data


def framed_identity(domain, fields):
    domain_bytes = domain.encode()
    preimage = b"OPENWEPP\0" + (1).to_bytes(2, "big") + len(domain_bytes).to_bytes(2, "big") + domain_bytes
    for tag, value in fields:
        tag_bytes = tag.encode()
        preimage += len(tag_bytes).to_bytes(2, "big") + tag_bytes + len(value).to_bytes(4, "big") + value
    return digest(preimage)


def sha_bytes(value, name):
    sha(value, name)
    return bytes.fromhex(value)


def tick_bytes(value, name):
    return u128(value, name).to_bytes(16, "big")


def validate_restart(doc):
    required = set(json.loads((ARTIFACTS / "restart-schema-v2.json").read_text())["required"])
    if set(doc) != required:
        raise Invalid("restart:closed-required-shape")
    for field in ["authority_sha256", "model_definition_sha256", "run_identity_sha256", "calendar_receipt_sha256", "forcing_receipt_sha256", "parent_interval_id", "parent_transaction_id", "accepted_complete_owner_set_sha256", "constraint_policy_sha256", "controller_policy_sha256"]:
        sha(doc[field], field)
    p0, p1 = support(doc["parent_support"], "parent_support")
    cursor = u128(doc["accepted_until_ns"], "accepted_until_ns")
    if not p0 <= cursor <= p1:
        raise Invalid("accepted_until:outside-parent")
    sequence = u128(doc["parent_transaction_sequence"], "parent_transaction_sequence")
    for field in ["begin_complete_owner_set_sha256", "begin_clock_sha256"]:
        sha(doc[field], field)
    expected_parent_interval = framed_identity("parent-interval", [
        ("run_id", bytes.fromhex(doc["run_identity_sha256"])),
        ("calendar_receipt", bytes.fromhex(doc["calendar_receipt_sha256"])),
        ("forcing_receipt", bytes.fromhex(doc["forcing_receipt_sha256"])),
        ("start_ns", p0.to_bytes(16, "big")), ("end_ns", p1.to_bytes(16, "big")),
    ])
    if doc["parent_interval_id"] != expected_parent_interval:
        raise Invalid("restart:parent-interval-id-mismatch")
    expected_parent_transaction = framed_identity("parent-transaction", [
        ("run_id", bytes.fromhex(doc["run_identity_sha256"])),
        ("sequence", sequence.to_bytes(16, "big")),
        ("parent_interval_id", bytes.fromhex(doc["parent_interval_id"])),
        ("begin_owner_set", bytes.fromhex(doc["begin_complete_owner_set_sha256"])),
    ])
    if doc["parent_transaction_id"] != expected_parent_transaction:
        raise Invalid("restart:parent-transaction-id-mismatch")
    owners = doc["complete_owner_state"]
    ordered_unique(owners, lambda x: x["owner_id"], MAX_OWNERS, "owners")
    owner_digests = []
    for owner in owners:
        data = decoded_bytes(owner, "state_bytes")
        owner_digests.append([owner["owner_id"], digest(data)])
    if digest(canonical(owner_digests)) != doc["accepted_complete_owner_set_sha256"]:
        raise Invalid("owners:set-digest-mismatch")
    participants = doc["active_segment"]["active_participants"]
    ordered_unique(participants, lambda x: x, MAX_OWNERS, "participants")
    if not set(participants).issubset({x["owner_id"] for x in owners}):
        raise Invalid("participants:not-owner-subset")
    s0, s1 = u128(doc["active_segment"]["start_ns"], "segment.start"), u128(doc["active_segment"]["end_ns"], "segment.end")
    if not p0 <= s0 <= cursor <= s1 <= p1 or s0 >= s1:
        raise Invalid("segment:cursor-or-parent-bounds")
    sha(doc["active_segment"]["regime_id"], "segment.regime")
    participant_set = digest(canonical(participants))
    expected_segment_id = framed_identity("segment", [
        ("parent_transaction_id", bytes.fromhex(doc["parent_transaction_id"])),
        ("ordinal", doc["active_segment"]["ordinal"].to_bytes(4, "big")),
        ("start_ns", s0.to_bytes(16, "big")), ("end_ns", s1.to_bytes(16, "big")),
        ("regime_id", bytes.fromhex(doc["active_segment"]["regime_id"])),
        ("participant_set", bytes.fromhex(participant_set)),
    ])
    if doc["active_segment"]["segment_id"] != expected_segment_id:
        raise Invalid("segment:id-mismatch")
    checkpoint = doc["accepted_controller_checkpoint"]
    decoded_bytes(checkpoint, "bytes")
    slabs = doc["accepted_slab_receipts"]
    if len(slabs) > MAX_RECEIPTS:
        raise Invalid("accepted-slab-receipts:too-many")
    expected_start = p0
    for ordinal, receipt in enumerate(slabs):
        for field in ["receipt_id", "accepted_slab_id", "parent_transaction_id", "segment_id", "constraint_digest", "begin_clock_sha256", "end_clock_sha256", "begin_owner_set_sha256", "end_owner_set_sha256", "owner_candidate_set_sha256", "coupled_ledger_sha256"]:
            sha(receipt[field], f"accepted-slab-receipts.{field}")
        if receipt["parent_transaction_id"] != doc["parent_transaction_id"]:
            raise Invalid("accepted-slab-receipts:parent-mismatch")
        if receipt["slab_ordinal"] != ordinal:
            raise Invalid("accepted-slab-receipts:ordinal-gap")
        start, end = support(receipt["support"], "accepted-slab-receipts.support")
        if start != expected_start or end > cursor:
            raise Invalid("accepted-slab-receipts:support-chronology")
        expected_start = end
        if not re.fullmatch(r"[0-9a-f]{16}", receipt["duration_bits"]):
            raise Invalid("accepted-slab-receipts:duration-bits")
        expected_bits = struct.pack(">d", (end - start) / 1_000_000_000.0).hex()
        if receipt["duration_bits"] != expected_bits:
            raise Invalid("accepted-slab-receipts:duration-mismatch")
        accepted_slab_id = framed_identity("accepted-slab", [
            ("parent_transaction_id", sha_bytes(receipt["parent_transaction_id"], "slab.parent")),
            ("slab_ordinal", receipt["slab_ordinal"].to_bytes(4, "big")),
            ("segment_id", sha_bytes(receipt["segment_id"], "slab.segment")),
            ("start_ns", start.to_bytes(16, "big")), ("end_ns", end.to_bytes(16, "big")),
            ("duration_bits", bytes.fromhex(receipt["duration_bits"])),
            ("begin_owner_set", sha_bytes(receipt["begin_owner_set_sha256"], "slab.begin-owner")),
            ("end_owner_set", sha_bytes(receipt["end_owner_set_sha256"], "slab.end-owner")),
            ("constraint_digest", sha_bytes(receipt["constraint_digest"], "slab.constraint")),
            ("ledger_digest", sha_bytes(receipt["coupled_ledger_sha256"], "slab.ledger")),
        ])
        if receipt["accepted_slab_id"] != accepted_slab_id:
            raise Invalid("accepted-slab-receipts:accepted-slab-id-mismatch")
        receipt_id = framed_identity("slab-receipt-v2", [
            ("parent_transaction_id", bytes.fromhex(receipt["parent_transaction_id"])),
            ("accepted_slab_id", bytes.fromhex(receipt["accepted_slab_id"])),
            ("slab_ordinal", receipt["slab_ordinal"].to_bytes(4, "big")),
            ("segment_id", bytes.fromhex(receipt["segment_id"])),
            ("start_ns", start.to_bytes(16, "big")), ("end_ns", end.to_bytes(16, "big")),
            ("duration_bits", bytes.fromhex(receipt["duration_bits"])),
            ("constraint_digest", bytes.fromhex(receipt["constraint_digest"])),
            ("begin_clock", bytes.fromhex(receipt["begin_clock_sha256"])),
            ("end_clock", bytes.fromhex(receipt["end_clock_sha256"])),
            ("begin_owner_set", bytes.fromhex(receipt["begin_owner_set_sha256"])),
            ("end_owner_set", bytes.fromhex(receipt["end_owner_set_sha256"])),
            ("owner_candidate_set", bytes.fromhex(receipt["owner_candidate_set_sha256"])),
            ("ledger_digest", bytes.fromhex(receipt["coupled_ledger_sha256"])),
        ])
        if receipt["receipt_id"] != receipt_id:
            raise Invalid("accepted-slab-receipts:receipt-id-mismatch")
    if expected_start != cursor or len(slabs) != doc["next_slab_ordinal"]:
        raise Invalid("accepted-slab-receipts:cursor-or-next-ordinal")
    if slabs and u128(doc["last_accepted_step_ns"], "last_accepted_step_ns") != u128(slabs[-1]["support"]["end_ns"], "last.end") - u128(slabs[-1]["support"]["start_ns"], "last.start"):
        raise Invalid("accepted-slab-receipts:last-step-mismatch")
    for field in ["accepted_event_receipts", "scheduled_once_receipts"]:
        ordered_unique(doc[field], lambda x: x["receipt_id"], MAX_RECEIPTS, field)
        for receipt in doc[field]:
            sha(receipt["receipt_id"], f"{field}.receipt_id")
            if u128(receipt["tick_ns"], f"{field}.tick") > cursor:
                raise Invalid(f"{field}:future-receipt")
    for ordinal, receipt in enumerate(sorted(doc["accepted_event_receipts"], key=lambda x: (u128(x["tick_ns"], "event.tick"), x["event_ordinal"]))):
        for field in ["event_id", "parent_transaction_id", "begin_clock_sha256", "end_clock_sha256", "begin_owner_set_sha256", "end_owner_set_sha256", "event_context_sha256", "ledger_digest"]:
            sha(receipt[field], f"event.{field}")
        if receipt["parent_transaction_id"] != doc["parent_transaction_id"] or receipt["event_ordinal"] != ordinal:
            raise Invalid("event:parent-or-ordinal")
        event_id = framed_identity("event", [
            ("parent_transaction_id", bytes.fromhex(receipt["parent_transaction_id"])),
            ("tick_ns", u128(receipt["tick_ns"], "event.tick").to_bytes(16, "big")),
            ("event_class", receipt["class"].encode()),
            ("event_ordinal", receipt["event_ordinal"].to_bytes(4, "big")),
            ("source_owner_id", receipt["source_owner_id"].encode()),
            ("event_context", bytes.fromhex(receipt["event_context_sha256"])),
        ])
        if receipt["event_id"] != event_id:
            raise Invalid("event:event-id-mismatch")
        receipt_id = framed_identity("event-receipt-v2", [
            ("parent_transaction_id", bytes.fromhex(receipt["parent_transaction_id"])),
            ("event_id", bytes.fromhex(receipt["event_id"])),
            ("tick_ns", u128(receipt["tick_ns"], "event.tick").to_bytes(16, "big")),
            ("ordinal", receipt["event_ordinal"].to_bytes(4, "big")),
            ("begin_clock", bytes.fromhex(receipt["begin_clock_sha256"])),
            ("end_clock", bytes.fromhex(receipt["end_clock_sha256"])),
            ("begin_owner_set", bytes.fromhex(receipt["begin_owner_set_sha256"])),
            ("end_owner_set", bytes.fromhex(receipt["end_owner_set_sha256"])),
            ("event_context", bytes.fromhex(receipt["event_context_sha256"])),
            ("ledger_digest", bytes.fromhex(receipt["ledger_digest"])),
        ])
        if receipt["receipt_id"] != receipt_id:
            raise Invalid("event:receipt-id-mismatch")
    if doc["next_event_ordinal"] != len(doc["accepted_event_receipts"]):
        raise Invalid("event:next-ordinal")
    actions = [(u128(row["support"]["end_ns"], "slab.end"), 0, row) for row in slabs]
    actions += [(u128(row["tick_ns"], "event.tick"), 1, row) for row in doc["accepted_event_receipts"]]
    actions.sort(key=lambda item: (item[0], item[1], item[2].get("event_ordinal", 0)))
    prior_owner, prior_clock = doc["begin_complete_owner_set_sha256"], doc["begin_clock_sha256"]
    for _, _, row in actions:
        if row["begin_owner_set_sha256"] != prior_owner or row["begin_clock_sha256"] != prior_clock:
            raise Invalid("restart:merged-owner-or-clock-chain")
        prior_owner, prior_clock = row["end_owner_set_sha256"], row["end_clock_sha256"]
    if prior_owner != doc["accepted_complete_owner_set_sha256"]:
        raise Invalid("restart:terminal-owner-chain")
    if doc["next_segment_ordinal"] != doc["active_segment"]["ordinal"] + 1:
        raise Invalid("segment:next-ordinal")
    accepted = {x["receipt_id"] for x in slabs} | {x["receipt_id"] for x in doc["accepted_event_receipts"]} | {x["receipt_id"] for x in doc["scheduled_once_receipts"]}
    ordered_unique(doc["reduction_state"], lambda x: x["reduction_id"], MAX_OWNERS, "reductions")
    for reduction in doc["reduction_state"]:
        ordered_unique(reduction["accepted_operand_receipt_ids"], lambda x: x, MAX_RECEIPTS, "reduction-operands")
        if not set(reduction["accepted_operand_receipt_ids"]).issubset(accepted):
            raise Invalid("reduction:unaccepted-operand")
    records = doc["pending_publication_buffer"]
    ordered_unique(records, lambda x: x["record_id"], MAX_RECEIPTS, "publication-buffer")
    for record in records:
        decoded_bytes(record, "value_bytes")
        if record["accepted_receipt_id"] not in accepted:
            raise Invalid("publication:unaccepted-receipt")
        support(record["support"], "publication.support")
    outbox = doc["publication_outbox"]
    ordered_unique(outbox, lambda x: x["publication_receipt_id"], MAX_RECEIPTS, "outbox")
    sequences = [u128(row["outbox_sequence"], "outbox.sequence") for row in outbox]
    if any(a >= b for a, b in zip(sequences, sequences[1:])):
        raise Invalid("outbox:sequence-order")
    for row in outbox:
        sha(row["parent_receipt_id"], "outbox.parent")
        sha(row["publication_receipt_id"], "outbox.receipt")
        u128(row["delivery_attempt_count"], "delivery_attempt_count")
        if row["state"] not in {"CommittedUndelivered", "DeliveredUnacknowledged", "Acknowledged"}:
            raise Invalid("outbox:state")
        ordered_unique(row["records"], lambda x: x["record_id"], MAX_RECEIPTS, "outbox.records")
        if digest(canonical(row["records"])) != row["records_sha256"]:
            raise Invalid("outbox:records-digest")


def validate_receipt(doc):
    schema = json.loads((ARTIFACTS / "receipt-candidate-ledger-schema.json").read_text())
    if set(schema["required"]) - set(doc) or set(doc) - set(schema["properties"]):
        raise Invalid("receipt:closed-required-shape")
    for field in ["identity_sha256", "parent_transaction_id", "begin_complete_owner_set_sha256", "end_complete_owner_set_sha256", "publication_records_sha256"]:
        sha(doc[field], field)
    common_support = support(doc["support"], "receipt.support") if "support" in doc else None
    complete = doc["complete_owner_ids"]
    active = doc["active_participant_ids"]
    candidates = doc["owner_candidates"]
    ordered_unique(complete, lambda x: x, MAX_OWNERS, "complete-owners")
    ordered_unique(active, lambda x: x, MAX_OWNERS, "active-participants")
    ordered_unique(candidates, lambda x: x["owner_id"], MAX_OWNERS, "owner-candidates")
    if [x["owner_id"] for x in candidates] != complete or not set(active).issubset(set(complete)):
        raise Invalid("receipt:owner-cardinality")
    for candidate in candidates:
        is_active = candidate["owner_id"] in active
        if (candidate["disposition"] == "ActiveCandidate") != is_active:
            raise Invalid("receipt:wrong-disposition")
        support(candidate["support"], "candidate.support")
        if common_support is not None and candidate["support"] != doc["support"]:
            raise Invalid("receipt:candidate-support-mismatch")
        if "duration_s_bits" in doc and candidate["duration_s_bits"] != doc["duration_s_bits"]:
            raise Invalid("receipt:candidate-duration-mismatch")
        try:
            end_bytes = base64.b64decode(candidate["end_state_bytes_base64"], validate=True)
        except Exception as exc:
            raise Invalid("candidate:invalid-base64") from exc
        if digest(end_bytes) != candidate["end_state_sha256"]:
            raise Invalid("candidate:end-state-digest")
        if not is_active and candidate["begin_state_sha256"] != candidate["end_state_sha256"]:
            raise Invalid("receipt:inactive-mutation")
    ordered_unique(doc["ledgers"], lambda x: x["ledger_id"], 16384, "ledgers")
    ledger_ids = {x["ledger_id"] for x in doc["ledgers"]}
    for ledger in doc["ledgers"]:
        if ledger["passes"] is not True:
            raise Invalid("ledger:failed")
    for candidate in candidates:
        if not set(candidate["local_ledger_ids"]).issubset(ledger_ids):
            raise Invalid("ledger:unresolved-reference")
    ordered_unique(doc["accepted_child_receipt_ids"], lambda x: x, MAX_RECEIPTS, "child-receipts")


def run_poison(path):
    fixture = json.loads(path.read_text())
    baseline = fixture["baseline"]
    results = []
    for case in fixture["cases"]:
        document = json.loads(json.dumps(baseline[case["target"]]))
        cursor = document
        parts = case["path"].split("/")[1:]
        for part in parts[:-1]:
            cursor = cursor[int(part)] if isinstance(cursor, list) else cursor[part]
        leaf = parts[-1]
        if case["mutation"] == "set":
            if isinstance(cursor, list):
                cursor[int(leaf)] = case["value"]
            else:
                cursor[leaf] = case["value"]
        elif case["mutation"] == "delete":
            if isinstance(cursor, list):
                del cursor[int(leaf)]
            else:
                del cursor[leaf]
        elif case["mutation"] == "append":
            cursor[leaf].append(case["value"])
        try:
            (validate_restart if case["target"] == "restart" else validate_receipt)(document)
            actual = "accepted"
        except Invalid:
            actual = "rejected"
        if actual != case["expected"]:
            raise SystemExit(f"{case['id']}: expected {case['expected']}, got {actual}")
        results.append({"id": case["id"], "status": actual})
    for case in fixture.get("canonical_serialization_cases", []):
        raw = case["raw"].encode()
        try:
            value = json.loads(raw)
            actual = "accepted" if raw == canonical(value) else "rejected"
        except (ValueError, UnicodeError):
            actual = "rejected"
        if actual != case["expected"]:
            raise SystemExit(f"{case['id']}: expected {case['expected']}, got {actual}")
        results.append({"id": case["id"], "status": actual})
    print(json.dumps({"schema": "OPENWEPP_COUPLED_TIME_SEMANTIC_VALIDATION_RESULTS_V1", "results": results}, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--poisons", type=Path, required=True)
    run_poison(parser.parse_args().poisons)
