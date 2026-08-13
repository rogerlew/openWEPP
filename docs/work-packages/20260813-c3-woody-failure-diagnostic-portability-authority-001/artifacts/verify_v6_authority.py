#!/usr/bin/env python3
"""Verify frozen V6 authority, executable poisons, and immutable predecessors."""

from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PKG = ROOT / "docs/work-packages/20260813-c3-woody-failure-diagnostic-portability-authority-001/artifacts"
STACK = ROOT / "docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts"
CONTRACT = ROOT / "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(value: bool, message: str) -> None:
    if not value:
        raise SystemExit(f"FAIL: {message}")


protected = {
    STACK / "openwepp_c3_woody_v1_definition.json": "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157",
    STACK / "openwepp_c3_woody_v2_definition.json": "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3",
    STACK / "openwepp_c3_woody_v3_definition.json": "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852",
    ROOT / "docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/openwepp_c3_woody_v2_definition.json": "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3",
    ROOT / "docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v3_definition.json": "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852",
    ROOT / "docs/work-packages/20260812-c3-woody-shared-state-authority-001/artifacts/openwepp_c3_woody_v4_definition.json": "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437",
    STACK / "openwepp_c3_woody_v5_definition.json": "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3",
    ROOT / "docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v5_definition.json": "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3",
}
current = {
    PKG / "reference_calculator_v6.py": "bfa805000a6e29b3c56a666ea97a4e4825f9262a3ef1f0daa5c3cfb5f2dd6532",
    PKG / "openwepp_c3_woody_v6_vectors.json": "2e7005f88d788399e914b2034c0193fc6f08d1657532a349ec797b966432356b",
    PKG / "openwepp_c3_woody_v6_definition.json": "a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426",
    STACK / "openwepp_c3_woody_v6_definition.json": "a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426",
}
for path, expected in (protected | current).items():
    require(digest(path) == expected, f"digest mismatch: {path}")

definition = json.loads((PKG / "openwepp_c3_woody_v6_definition.json").read_text())
vectors = json.loads((PKG / "openwepp_c3_woody_v6_vectors.json").read_text())
require(definition["model_version"] == "OPENWEPP_C3_WOODY_V6", "V6 identity")
require(definition["base_model_definition"]["sha256"] == protected[STACK / "openwepp_c3_woody_v5_definition.json"], "V5 import")

contract = CONTRACT.read_text()
start_marker = definition["canonical_section_extraction"]["start_heading"] + "\n"
end_marker = definition["canonical_section_extraction"]["end_heading"] + "\n"
start = contract.index(start_marker)
end = contract.index(end_marker, start)
require(hashlib.sha256(contract[start:end].encode()).hexdigest() == definition["canonical_section_sha256"]["v6_rejected_failure_diagnostic_portability_amendment"], "V6 section digest")
v5_start_marker = "## `OPENWEPP_C3_WOODY_V5` Fixed-Authorization Capped-Pass Amendment\n"
v5_end_marker = "## Change Log\n"
v5_start = contract.index(v5_start_marker)
v5_end = contract.index(v5_end_marker, v5_start)
require(
    hashlib.sha256(contract[v5_start:v5_end].encode()).hexdigest()
    == "22edf6816d078833029d59bfb263b3c7ccdc8669a3fcccf73d82e601add34a5f",
    "immutable V5 canonical section digest",
)

spec = importlib.util.spec_from_file_location("v6_reference", PKG / "reference_calculator_v6.py")
require(spec is not None and spec.loader is not None, "load generator")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
cases = sum(vectors["families"].values(), [])
for row in cases:
    observed = module.portable_equal(row["reference"], row["actual"])
    require(observed == row["expected_equal"], f"executed case: {row['case']}")
require(all(vectors["checks"].values()), "fixture checks")
require(sum(row["expected_equal"] for row in cases) == 6, "closed positive inventory")
transition = vectors["identity_transition"]
require(
    transition["non_identity_payload_bytes_sha256_before"]
    == transition["non_identity_payload_bytes_sha256_after"],
    "canonical non-identity payload preservation",
)
require(
    transition["source"]["configuration_sha256"] != transition["target"]["configuration_sha256"]
    and transition["source"]["state_sha256"] != transition["target"]["state_sha256"]
    and transition["source"]["diagnostic_sha256"] != transition["target"]["diagnostic_sha256"],
    "distinct derived V5/V6 identities",
)
print("PASS: V6 authority, executable poisons, copies, and immutable V1-V5 verified")
