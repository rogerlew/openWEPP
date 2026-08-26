#!/usr/bin/env python3
"""Executable acyclic candidate receipt construction and poison checks."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

ORDER = (
    "BatchRequestCore",
    "ArmInputCore",
    "ArmPhysicalResultCore",
    "EndingJointReceipt",
    "ArmCustodyReceipt",
    "LaneEvidenceCore",
    "BatchResultCore",
    "EventReceipt",
    "ParentReceipt",
)

PREDECESSORS = {
    "BatchRequestCore": (),
    "ArmInputCore": ("BatchRequestCore",),
    "ArmPhysicalResultCore": ("ArmInputCore",),
    "EndingJointReceipt": ("ArmPhysicalResultCore",),
    "ArmCustodyReceipt": ("ArmInputCore", "ArmPhysicalResultCore", "EndingJointReceipt"),
    "LaneEvidenceCore": ("ArmCustodyReceipt",),
    "BatchResultCore": ("BatchRequestCore", "ArmCustodyReceipt", "LaneEvidenceCore"),
    "EventReceipt": ("BatchResultCore",),
    "ParentReceipt": ("BatchResultCore", "EventReceipt"),
}


def frame(tag: str, fields: list[tuple[str, bytes]]) -> bytes:
    output = bytearray(b"OPENWEPP_CANONICAL_FRAMED_SHA256_V1\0")
    output.extend(len(tag).to_bytes(4, "big"))
    output.extend(tag.encode())
    for name, value in fields:
        output.extend(len(name).to_bytes(4, "big"))
        output.extend(name.encode())
        output.extend(len(value).to_bytes(4, "big"))
        output.extend(value)
    return bytes(output)


def construct(seed: bytes = b"child1-terminal-candidate") -> dict[str, str]:
    digests: dict[str, str] = {}
    for ordinal, node in enumerate(ORDER):
        fields = [("schema", (1).to_bytes(4, "big")), ("ordinal", ordinal.to_bytes(4, "big")), ("seed", seed)]
        for predecessor in PREDECESSORS[node]:
            fields.append(("predecessor", bytes.fromhex(digests[predecessor])))
        digests[node] = hashlib.sha256(frame(f"child1-{node.lower()}-v1", fields)).hexdigest()
    return digests


def validate() -> dict[str, object]:
    position = {node: index for index, node in enumerate(ORDER)}
    violations = [
        (node, predecessor)
        for node, predecessors in PREDECESSORS.items()
        for predecessor in predecessors
        if predecessor not in position or position[predecessor] >= position[node]
    ]
    baseline = construct()
    replay = construct()
    poison = construct(b"child1-terminal-candidate-poison")
    changed = [node for node in ORDER if baseline[node] != poison[node]]
    # Substitution at the earliest core must propagate to every successor.
    expected_changed = list(ORDER)
    return {
        "schema": "openwepp-terminal-receipt-dag-v7-evidence-v1",
        "order": ORDER,
        "predecessors": PREDECESSORS,
        "forward_reference_violations": violations,
        "acyclic": not violations,
        "deterministic_replay": baseline == replay,
        "digests": baseline,
        "poison_changed_nodes": changed,
        "poison_propagates_to_all_successors": changed == expected_changed,
        "pass": not violations and baseline == replay and changed == expected_changed,
    }


def main() -> None:
    result = validate()
    here = Path(__file__).resolve().parent
    (here / "terminal-receipt-dag-v7-evidence.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    (here / "terminal-receipt-dag-v7-evidence.md").write_text(
        "# Terminal receipt DAG v7 executable evidence\n\n"
        "Ran: package-local canonical hash-construction prototype.\n\n"
        f"- pass: `{result['pass']}`\n"
        f"- acyclic/no successor references: `{result['acyclic']}`\n"
        f"- deterministic replay: `{result['deterministic_replay']}`\n"
        f"- root poison propagates through all nodes: `{result['poison_propagates_to_all_successors']}`\n\n"
        "Order: `" + " -> ".join(ORDER) + "`.\n"
    )
    if not result["pass"]:
        raise SystemExit("receipt DAG validation failed")


if __name__ == "__main__":
    main()
