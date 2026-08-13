#!/usr/bin/env python3
"""Verify immutable V7 authority bytes without importing production Rust."""

from __future__ import annotations

import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = ROOT / "docs/work-packages/20260813-c3-woody-storage-transfer-phenology-authority-001"
ARTIFACTS = PACKAGE / "artifacts"
CONTRACT = ROOT / "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md"
MODEL_STACK = ROOT / "docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts"

EXPECTED = {
    "openwepp_c3_woody_v7_definition.json": "a78264d8cd24d2718e099420357e1632ac09f2ba18c4a42d21e7e5b282aa459f",
    "openwepp_c3_woody_v7_vectors.json": "d99288741f3cac16f017ffe5cd11620bfde2055e32f18b82e538eaf6d48ef411",
    "reference_calculator_v7.py": "dfc7c586cb42f7729de09ac0660fa4b2f61d8132ccb3b24b570743bd1ba8a5dd",
}
PREDECESSORS = {
    MODEL_STACK / "openwepp_c3_woody_v1_definition.json": "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157",
    ROOT / "docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/openwepp_c3_woody_v2_definition.json": "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3",
    ROOT / "docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v3_definition.json": "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852",
    ROOT / "docs/work-packages/20260812-c3-woody-shared-state-authority-001/artifacts/openwepp_c3_woody_v4_definition.json": "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437",
    ROOT / "docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v5_definition.json": "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3",
    ROOT / "docs/work-packages/20260813-c3-woody-failure-diagnostic-portability-authority-001/artifacts/openwepp_c3_woody_v6_definition.json": "a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426",
}


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    for name, expected in EXPECTED.items():
        actual = digest(ARTIFACTS / name)
        if actual != expected:
            raise SystemExit(f"{name}: expected {expected}, found {actual}")
    for path, expected in PREDECESSORS.items():
        actual = digest(path)
        if actual != expected:
            raise SystemExit(f"historical bytes changed: {path}: {actual}")

    definition_bytes = (ARTIFACTS / "openwepp_c3_woody_v7_definition.json").read_bytes()
    if (MODEL_STACK / "openwepp_c3_woody_v7_definition.json").read_bytes() != definition_bytes:
        raise SystemExit("V7 authority copies differ")
    definition = json.loads(definition_bytes)
    contract = CONTRACT.read_text()
    start = definition["canonical_section_start"] + "\n"
    end = definition["canonical_section_end"] + "\n"
    section = start + contract.split(start, 1)[1].split(end, 1)[0]
    actual_section = hashlib.sha256(section.encode()).hexdigest()
    if actual_section != definition["canonical_section_sha256"]:
        raise SystemExit(
            f"V7 section: expected {definition['canonical_section_sha256']}, "
            f"found {actual_section}"
        )

    subprocess.run(
        ["python3", str(ARTIFACTS / "reference_calculator_v7.py"), "--verify"],
        cwd=ROOT,
        check=True,
    )
    print("PASS V7 authority definitions, predecessors, section, and vectors")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
