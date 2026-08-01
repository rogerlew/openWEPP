#!/usr/bin/env python3
"""Freeze the EB-04S unit decision from a result-blind authority whitelist."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
CONTRACT = REPO / "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md"
UNIT_GOVERNANCE = REPO / "docs/specifications/unit-governance.md"
UNIT_SOURCE = REPO / "crates/openwepp-unit-boundary/src/lib.rs"
EB04E_PROTOCOL = REPO / (
    "docs/work-packages/20260731-snow-surface-eb-04e-corrected-population-"
    "runtime-qualification-001/artifacts/prospective-qualification-protocol.md"
)
WHITELIST = (CONTRACT, UNIT_GOVERNANCE, UNIT_SOURCE, EB04E_PROTOCOL)
FORBIDDEN = (
    "20260801-snow-surface-eb-04r",
    "snow_surface_eb04r_factorial",
    "factorial-results",
    "execution-attempt",
    "terminal-frozen-protocol-audit",
    "/observations/",
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative(path: Path) -> str:
    return path.relative_to(REPO).as_posix()


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def inspect_authority() -> dict[str, object]:
    if any(token in str(path) for path in WHITELIST for token in FORBIDDEN):
        raise RuntimeError("authority whitelist contains a forbidden result-bearing path")
    if len(set(WHITELIST)) != 4 or not all(path.is_file() for path in WHITELIST):
        raise RuntimeError("authority whitelist is incomplete or duplicated")

    contract = CONTRACT.read_text(encoding="utf-8")
    governance = UNIT_GOVERNANCE.read_text(encoding="utf-8")
    source = UNIT_SOURCE.read_text(encoding="utf-8")
    eb04e = EB04E_PROTOCOL.read_text(encoding="utf-8")

    required = {
        "contract_swe_tolerance": "Runtime mass closure uses `1e-9 m` water equivalent",
        "unit_policy": "Dimensional conversions must be named, directional, provenance-backed, and",
        "density_constant": "LIQUID_WATER_DENSITY_KG_M3: f64 = 1_000.0",
        "named_conversion": "snow_water_equivalent_meters_to_area_mass_kg_m2",
        "conversion_formula": "water_equivalent_m * LIQUID_WATER_DENSITY_KG_M3",
        "eb04e_swe_tolerance": "snow mass: maximum daily reconstruction `<=1e-9 m`",
        "eb04e_vapor_sublimation": "vapor/sublimation: `<=1e-6 kg m^-2`",
    }
    surfaces = {
        "contract_swe_tolerance": contract,
        "unit_policy": governance,
        "density_constant": source,
        "named_conversion": source,
        "conversion_formula": source,
        "eb04e_swe_tolerance": eb04e,
        "eb04e_vapor_sublimation": eb04e,
    }
    missing = [key for key, needle in required.items() if needle not in surfaces[key]]
    if missing:
        raise RuntimeError(f"required authority anchors missing: {missing}")

    density_match = re.search(r"LIQUID_WATER_DENSITY_KG_M3: f64 = ([0-9_]+(?:\.[0-9]+)?)", source)
    if density_match is None:
        raise RuntimeError("cannot parse named liquid-water density")
    density = float(density_match.group(1).replace("_", ""))
    swe_tolerance_m = 1.0e-9
    area_mass_tolerance = swe_tolerance_m * density
    if density != 1000.0 or abs(area_mass_tolerance - 1.0e-6) > 1.0e-21:
        raise RuntimeError("dimensional derivation does not produce the expected exact tolerance")

    return {
        "schema": "snow-surface-eb04s-authority-freeze-v1",
        "status": "FROZEN_AUTHORITY_ONLY",
        "evidence_class": "Static",
        "input_policy": "EXACT_FOUR_FILE_RESULT_BLIND_WHITELIST",
        "inputs": {relative(path): sha256(path) for path in WHITELIST},
        "forbidden_path_tokens": list(FORBIDDEN),
        "anchors": required,
        "derivation": {
            "water_equivalent_tolerance_m": swe_tolerance_m,
            "liquid_water_density_kg_m3": density,
            "formula": "area_mass_tolerance_kg_m2 = water_equivalent_tolerance_m * liquid_water_density_kg_m3",
            "area_mass_tolerance_kg_m2": area_mass_tolerance,
            "identity": "1e-9 m SWE == 1e-6 kg m^-2 at rho_w = 1000 kg m^-3",
        },
        "decision": {
            "vapor_to_sublimation_closure_tolerance_kg_m2": area_mass_tolerance,
            "represented_layer_lifecycle_boundary_kg_m2": 1.0e-9,
            "boundaries_are_distinct": True,
            "classification": "CROSS_UNIT_PROTOCOL_TRANSCRIPTION_ERROR",
        },
        "result_bearing_inputs_read": False,
    }


def self_check() -> None:
    receipt = inspect_authority()
    assert receipt["decision"]["boundaries_are_distinct"] is True  # type: ignore[index]
    assert receipt["result_bearing_inputs_read"] is False
    first = json.dumps(receipt, sort_keys=True)
    second = json.dumps(inspect_authority(), sort_keys=True)
    if first != second:
        raise RuntimeError("authority derivation is not deterministic")


def verify_seal() -> None:
    freeze_path = ARTIFACTS / "authority-freeze.json"
    seal_path = ARTIFACTS / "authority-seal.json"
    if not freeze_path.is_file() or not seal_path.is_file():
        raise RuntimeError("authority freeze or seal is missing")
    freeze = json.loads(freeze_path.read_text(encoding="utf-8"))
    seal = json.loads(seal_path.read_text(encoding="utf-8"))
    if freeze.get("status") != "FROZEN_AUTHORITY_ONLY":
        raise RuntimeError("authority freeze status drift")
    if freeze.get("result_bearing_inputs_read") is not False:
        raise RuntimeError("authority freeze is not result-blind")
    if seal.get("authority_freeze_sha256") != sha256(freeze_path):
        raise RuntimeError("authority freeze hash differs from terminal seal")
    if seal.get("canonical_contract_sha256") != sha256(CONTRACT):
        raise RuntimeError("canonical contract hash differs from terminal seal")
    if seal.get("status") != "FROZEN_DUAL_VERIFIED" or not seal.get("phase_b_authorized"):
        raise RuntimeError("authority seal is not dual-verified")


def freeze() -> None:
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    receipt = inspect_authority()
    write_json(ARTIFACTS / "authority-freeze.json", receipt)
    derivation = receipt["derivation"]  # type: ignore[assignment]
    decision = receipt["decision"]  # type: ignore[assignment]
    text = f"""# Frozen Authority Reconciliation

Evidence mode: `Static`.

Status: `FROZEN_AUTHORITY_ONLY`.

The authority phase read exactly the four whitelisted pre-result inputs. It did
not read EB-04R outputs, residuals, scores, observations, attempt records, or
terminal audit evidence.

## Dimensional Decision

```text
{derivation['formula']}
1e-9 m * 1000 kg m^-3 = {derivation['area_mass_tolerance_kg_m2']:.0e} kg m^-2
```

Therefore the canonical `1e-9 m` snow-mass closure tolerance is
`1e-6 kg m^-2` when the same residual is expressed as area mass. The
`1e-9 kg m^-2` represented-layer lifecycle boundary is a different predicate
and must not be substituted for an aggregate or transfer-identity residual.

Decision: `{decision['classification']}`.

The frozen machine-readable receipt is `authority-freeze.json`. Its SHA-256 is
`{sha256(ARTIFACTS / 'authority-freeze.json')}`.
"""
    (ARTIFACTS / "authority-reconciliation.md").write_text(text, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--self-check", action="store_true")
    group.add_argument("--freeze", action="store_true")
    group.add_argument("--verify-seal", action="store_true")
    args = parser.parse_args()
    if args.self_check:
        self_check()
        print("EB-04S authority whitelist and dimensional self-check: PASS")
    elif args.freeze:
        freeze()
        print(f"EB-04S authority freeze: PASS ({sha256(ARTIFACTS / 'authority-freeze.json')})")
    else:
        verify_seal()
        print("EB-04S frozen authority and terminal seal verification: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
