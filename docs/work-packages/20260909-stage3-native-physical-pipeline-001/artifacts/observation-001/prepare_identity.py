#!/usr/bin/env python3
"""Bind the predecessor's protected identity to the frozen observation binary."""
import hashlib
import json
import sys
from pathlib import Path


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


baseline = Path(sys.argv[1])
binary = Path(sys.argv[2]).resolve(strict=True)
sidecar = Path(str(binary) + ".json").resolve(strict=True)
output = Path(sys.argv[3])
identity = json.loads(baseline.read_text())
rules = identity["manifest_rules"]
rules["/binary_path"] = {"expected_arm_identity": str(binary)}
rules["/binary_sha256"] = {"expected_arm_identity": digest(binary)}
rules["/binary_sidecar_path"] = {"expected_arm_identity": str(sidecar)}
rules["/binary_sidecar_sha256"] = {"expected_arm_identity": digest(sidecar)}
rules["/source_commit"] = {
    "expected_arm_identity": "unknown"
}
output.write_text(json.dumps(identity, indent=2, sort_keys=True) + "\n")
