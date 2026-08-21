"""Independent closed support-domain oracle; no production Rust imports.

The receipt encoding intentionally follows the released Rust field order and
uses only canonical strings, so its digest and all identity poisons are
independently reproducible.
"""

import hashlib
import json
import struct

MINIMUM_SUPPORT_NS = 600_000_000
POLICY = "OPENWEPP_SNOW_FREE_LSE_V1_SUPPORT_POLICY_V1"
DOMAIN = b"OPENWEPP_LSE_SUPPORT_ADMISSION_V1\0"


def sha(value: str) -> str:
    return hashlib.sha256(value.encode()).hexdigest()


TOLERANCE_POLICY_SHA256 = sha("energy_absolute=1e-6;energy_relative=1e-10")
NUMERICAL_POLICY_SHA256 = sha("iterations=50;backtracking=0..20;strict-decrease")


def duration_bits(duration_ns: int) -> str:
    return struct.pack(">d", duration_ns / 1_000_000_000.0).hex()


def receipt(parent: str = "a" * 64, segment: str = "b" * 64, slab: str = "c" * 64, ordinal: int = 0, start: int = 0, duration: int = MINIMUM_SUPPORT_NS) -> dict:
    body = {
        "parent_transaction_id": parent,
        "segment_id": segment,
        "accepted_slab_id": slab,
        "slab_ordinal": str(ordinal),
        "support_start_ns": str(start),
        "support_end_ns": str(start + duration),
        "model_version": "OPENWEPP_SNOW_FREE_LSE_V1",
        "model_definition_sha256": "e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f",
        "configuration_sha256": "a" * 64,
        "beginning_state_sha256": "b" * 64,
        "tolerance_policy_sha256": TOLERANCE_POLICY_SHA256,
        "numerical_policy_sha256": NUMERICAL_POLICY_SHA256,
        "requested_support_ns": str(duration),
        "duration_s_bits": duration_bits(duration),
        "minimum_support_ns": str(MINIMUM_SUPPORT_NS),
        "receipt_sha256": "",
    }
    body["receipt_sha256"] = hashlib.sha256(
        DOMAIN + json.dumps(body, separators=(",", ":")).encode()
    ).hexdigest()
    return body


def validate(value: dict, expected_shape: dict | None = None) -> str:
    required = set(receipt().keys())
    if set(value) != required:
        return "LSEB-E-042"
    try:
        start = int(value["support_start_ns"])
        end = int(value["support_end_ns"])
        requested = int(value["requested_support_ns"])
    except (TypeError, ValueError):
        return "LSEB-E-042"
    if any(not isinstance(value, str) or (len(value) > 1 and value.startswith("0")) for value in (value["slab_ordinal"], value["support_start_ns"], value["support_end_ns"], value["requested_support_ns"])):
        return "LSEB-E-042"
    if len(value["parent_transaction_id"]) != 64 or len(value["segment_id"]) != 64 or len(value["accepted_slab_id"]) != 64:
        return "LSEB-E-042"
    if end - start != requested or requested <= 0:
        return "LSEB-E-042"
    if value["model_version"] != "OPENWEPP_SNOW_FREE_LSE_V1" or value["model_definition_sha256"] != "e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f":
        return "LSEB-E-042"
    if value["duration_s_bits"] != duration_bits(requested):
        return "LSEB-E-042"
    if value["minimum_support_ns"] != str(MINIMUM_SUPPORT_NS):
        return "LSEB-E-042"
    if value["tolerance_policy_sha256"] != TOLERANCE_POLICY_SHA256 or value["numerical_policy_sha256"] != NUMERICAL_POLICY_SHA256:
        return "LSEB-E-042"
    expected = dict(value)
    expected["receipt_sha256"] = ""
    digest = hashlib.sha256(DOMAIN + json.dumps(expected, separators=(",", ":")).encode()).hexdigest()
    if value["receipt_sha256"] != digest:
        return "LSEB-E-042"
    if expected_shape is not None:
        for field in ("parent_transaction_id", "segment_id", "accepted_slab_id", "slab_ordinal", "support_start_ns", "support_end_ns", "configuration_sha256", "beginning_state_sha256"):
            if value[field] != expected_shape[field]:
                return "LSEB-E-042"
    return admit(requested)


def admit(requested_support_ns: int, start_ns: int = 0, end_ns: int | None = None) -> str:
    if requested_support_ns <= 0:
        return "LSEB-E-041"
    if end_ns is not None and end_ns - start_ns != requested_support_ns:
        return "LSEB-E-042"
    return "admitted" if requested_support_ns >= MINIMUM_SUPPORT_NS else "LSEB-E-041"


def main() -> None:
    with open(__file__.replace("lse_support_admissibility_reference.py", "lse-support-fixture-profile.json"), encoding="utf-8") as handle:
        profile = json.load(handle)
    with open(__file__.replace("lse_support_admissibility_reference.py", "lse-support-admissibility-vectors.json"), encoding="utf-8") as handle:
        vectors = json.load(handle)
    assert len(profile["fixtures"]) == 7
    assert sum(fixture["status"] == "adopter" for fixture in profile["fixtures"]) == 2
    assert profile["minimum_support_ns"] == str(MINIMUM_SUPPORT_NS)
    assert vectors["minimum_support_ns"] == str(MINIMUM_SUPPORT_NS)
    baseline = receipt()
    owners = [{"owner_id": owner, "state_bytes": f"{owner}:state", "state_sha256": sha(f"{owner}:state")} for owner in ("vegetation", "snow", "land_surface_energy", "surface_liquid", "hydrology", "bgc", "soil_thermal")]
    def advance(owner_set: list[dict], support_ns: int) -> list[dict]:
        return [{"owner_id": owner["owner_id"], "state_bytes": owner["state_bytes"] + f"|{support_ns}", "state_sha256": sha(owner["state_bytes"] + f"|{support_ns}")} for owner in owner_set]
    staged = advance(owners, MINIMUM_SUPPORT_NS)
    checkpoint = {
        "parent_cursor": "41",
        "accepted_until_ns": str(MINIMUM_SUPPORT_NS),
        "parent_transaction_id": baseline["parent_transaction_id"],
        "controller_policy": {"model": POLICY, "numerics": NUMERICAL_POLICY_SHA256},
        "beginning_owner_envelopes": owners,
        "staged_owner_envelopes": staged,
        "owner_envelopes": [owner["state_sha256"] for owner in staged],
        "accepted_receipts": [baseline],
        "event_chronology": [],
        "scheduled_receipts": [],
        "reduction_operands": [],
        "publication_records": [],
        "outbox": [],
        "suffix_operations": [{"support_ns": str(MINIMUM_SUPPORT_NS), "event": None}],
        "candidate_present": False,
    }
    before = json.dumps(checkpoint, separators=(",", ":"))
    frozen = vectors["rollback_snapshot"]
    assert frozen["accepted_until_ns"] == checkpoint["accepted_until_ns"]
    assert frozen["accepted_receipts"] == ["baseline_receipt"]
    assert frozen["suffix_operations"] == checkpoint["suffix_operations"]
    assert frozen["beginning_owner_envelopes"] == [owner["owner_id"] for owner in owners]
    assert frozen["staged_owner_envelopes"] == [owner["state_bytes"] for owner in staged]
    assert validate(baseline, baseline) == "admitted"
    below = receipt(duration=MINIMUM_SUPPORT_NS - 1)
    assert validate(below) == "LSEB-E-041"
    assert validate(receipt(start=17), receipt(start=17)) == "admitted"
    assert validate(receipt(duration=MINIMUM_SUPPORT_NS + 1), receipt(duration=MINIMUM_SUPPORT_NS + 1)) == "admitted"
    leading_zero_digest = receipt(parent="0" + "a" * 63)
    assert validate(leading_zero_digest, leading_zero_digest) == "admitted"
    for field, value in [("parent_transaction_id", "d" * 64), ("segment_id", "e" * 64), ("accepted_slab_id", "f" * 64), ("slab_ordinal", "01"), ("support_start_ns", "01"), ("support_end_ns", str(MINIMUM_SUPPORT_NS + 1)), ("duration_s_bits", "0000000000000000"), ("model_version", "other"), ("model_definition_sha256", "1" * 64), ("configuration_sha256", "2" * 64), ("beginning_state_sha256", "3" * 64), ("tolerance_policy_sha256", "4" * 64), ("numerical_policy_sha256", "c" * 64), ("minimum_support_ns", "1"), ("receipt_sha256", "d" * 64)]:
        poison = dict(baseline)
        poison[field] = value
        if field != "receipt_sha256":
            poison["receipt_sha256"] = hashlib.sha256(DOMAIN + json.dumps({**poison, "receipt_sha256": ""}, separators=(",", ":")).encode()).hexdigest()
        assert validate(poison, baseline) == "LSEB-E-042", field
    rejected = receipt(duration=MINIMUM_SUPPORT_NS - 1)
    assert validate(rejected, baseline) == "LSEB-E-042"
    assert json.dumps(checkpoint, separators=(",", ":")) == before
    restored = json.loads(json.dumps(checkpoint, separators=(",", ":")))
    assert json.dumps(restored, separators=(",", ":")) == before
    assert restored["controller_policy"] == checkpoint["controller_policy"]
    assert [owner["owner_id"] for owner in restored["beginning_owner_envelopes"]] == [owner["owner_id"] for owner in restored["staged_owner_envelopes"]]
    uninterrupted_final = advance(advance(owners, MINIMUM_SUPPORT_NS), MINIMUM_SUPPORT_NS)
    restored_final = advance(restored["staged_owner_envelopes"], MINIMUM_SUPPORT_NS)
    uninterrupted = hashlib.sha256(json.dumps(uninterrupted_final, separators=(",", ":")).encode()).hexdigest()
    restored_suffix = hashlib.sha256(json.dumps(restored_final, separators=(",", ":")).encode()).hexdigest()
    assert uninterrupted == restored_suffix
    assert admit(1) == "LSEB-E-041"
    assert admit(MINIMUM_SUPPORT_NS - 1) == "LSEB-E-041"
    print("lse support oracle: 15/15")


if __name__ == "__main__":
    main()
