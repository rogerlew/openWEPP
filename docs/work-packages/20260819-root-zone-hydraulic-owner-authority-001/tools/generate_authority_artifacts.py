#!/usr/bin/env python3
"""Generate the independent OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1 artifacts."""

from __future__ import annotations

import hashlib
import json
import math
import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ARTIFACTS = ROOT / "artifacts"
MODEL = "OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1"


def canonical(value: object) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True) + "\n").encode()


def sha(value: object) -> str:
    return hashlib.sha256(canonical(value)).hexdigest()


def bits(value: float) -> str:
    return f"{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"


def calculate(*, liquid_m: float, thickness_m: float, porosity: float, ksat_m_s: float,
              psi_sat_mm: float, b: float, top_m: float, lateral_m: float,
              dxroot_m: float) -> dict[str, str]:
    theta = liquid_m / thickness_m
    raw_s = theta / porosity
    saturation = min(1.0, max(0.0, raw_s))
    if saturation == 0.0:
        saturation = 0.0
    s_psi = max(0.01, saturation)
    psi = max(psi_sat_mm * math.pow(s_psi, -b), -1.0e8)
    exponent = 2.0 * b + 3.0
    conductivity = min(ksat_m_s, ksat_m_s * math.pow(saturation, exponent))
    node = top_m + 0.5 * thickness_m
    gravity = 1000.0 * node
    root_path = 1000.0 * (node + lateral_m)
    return {key: bits(value) for key, value in {
        "theta_liq": theta, "relative_saturation_raw": raw_s,
        "relative_saturation": saturation, "retention_saturation": s_psi,
        "matric_potential_mm": psi, "conductivity_exponent": exponent,
        "current_conductivity_m_s": conductivity,
        "soil_conductivity_mm_s": 1000.0 * conductivity,
        "layer_node_depth_m": node, "gravity_root_mm": gravity,
        "root_path_length_mm": root_path, "soil_root_interface_distance_m": dxroot_m,
    }.items()}


def case(name: str, **inputs: float) -> dict[str, object]:
    return {"name": name, "disposition": "accept", "inputs": {k: bits(v) for k, v in inputs.items()},
            "expected": calculate(**inputs)}


def write(name: str, value: object) -> None:
    (ARTIFACTS / name).write_bytes(canonical(value))


def main() -> None:
    model = {
        "schema": "openwepp-root-zone-hydraulic-model-definition-v1",
        "model": MODEL,
        "pow": {"rust": "libm 0.2.16 libm::pow", "calculator": "CPython math.pow",
                "comparison": "exact IEEE-754 binary64 bits"},
        "operation_order": ["theta=liquid/thickness", "S_raw=theta/porosity",
            "S=min(1,max(0,S_raw))", "S_psi=max(0.01,S)",
            "psi=max(psi_sat*pow(S_psi,-B),-1e8)", "exponent=2*B+3",
            "K=min(Ksat,Ksat*pow(S,exponent))", "K_mm_s=1000*K",
            "node=ordered_top+0.5*thickness", "gravity_mm=1000*node",
            "z3_mm=1000*(node+required_stratum_lateral_path)"],
        "forbidden": ["WB14 suction", "WB14 conductivity", "Ksat as current K",
            "S_psi for K", "RootLayer.lateral_root_length_m as z3", "CLM PFT defaults"],
    }
    model["model_definition_sha256"] = sha(model)
    write("model-definition.json", model)

    base = dict(liquid_m=0.02, thickness_m=0.1, porosity=0.4, ksat_m_s=1e-6,
                psi_sat_mm=-120.0, b=4.05, top_m=0.0, lateral_m=0.2, dxroot_m=0.05)
    vectors = [
        case("intermediate_saturation", **base),
        case("exact_saturation", **(base | {"liquid_m": 0.04})),
        case("one_bit_above_pore_capacity_roundoff", **(base | {"liquid_m": math.nextafter(0.04, math.inf)})),
        case("exact_s_psi_lower_clamp", **(base | {"liquid_m": 0.00004})),
        case("exact_zero_live_water", **(base | {"liquid_m": 0.0})),
        case("signed_negative_zero", **(base | {"liquid_m": -0.0})),
        case("different_ofe_parameters", **(base | {"psi_sat_mm": -300.0, "b": 6.2})),
        case("second_layer_positive_path", **(base | {"top_m": 0.1, "thickness_m": 0.3, "lateral_m": 0.35})),
        case("zero_explicit_lateral_path", **(base | {"lateral_m": 0.0})),
        case("same_dxroot_different_z3", **(base | {"lateral_m": 0.4})),
        case("same_z3_different_dxroot", **(base | {"dxroot_m": 0.4})),
    ]
    rejects = [{"name": name, "disposition": "reject", "error": error} for name, error in [
        ("material_pore_capacity_violation", "WaterAbovePoreCapacity"),
        ("frozen_rooted_layer", "FrozenRootedLayerUnsupported"),
        ("missing_root_tissue_path", "ConfigurationIdentity"),
        ("positive_psi_sat", "Domain"), ("zero_b", "Domain"),
    ]]
    write("root-zone-hydraulic-vectors.json", {"schema": "root-zone-hydraulic-vectors-v1",
        "model_definition_sha256": model["model_definition_sha256"], "accepted": vectors,
        "rejected": rejects})

    schema = {"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object",
        "additionalProperties": False,
        "required": ["schema_version", "model_definition_sha256", "configuration_sha256", "owner_id",
            "hydrology_configuration_sha256", "vegetation_configuration_sha256", "lse_configuration_sha256",
            "ordered_layers", "ordered_stratum_geometry"],
        "properties": {k: {"type": "string", "minLength": 1} for k in ["schema_version",
            "model_definition_sha256", "configuration_sha256", "owner_id", "hydrology_configuration_sha256",
            "vegetation_configuration_sha256", "lse_configuration_sha256"]}}
    schema["properties"]["ordered_layers"] = {"type": "array", "minItems": 1, "items": {"type": "object"}}
    schema["properties"]["ordered_stratum_geometry"] = {"type": "array", "minItems": 1,
        "items": {"type": "object", "required": ["stratum_id", "root_tissue_lateral_path_m"]}}
    write("configuration-schema.json", schema)

    write("runtime-descriptor.json", {"model": MODEL, "construction": "private_per_interval",
        "inputs": ["current staged hydrology", "immutable root-zone configuration", "V10/LSE identities"],
        "output": "sealed RootZoneHydraulicLayerReceiptV1", "state_mutation": "none"})
    poisons = ["WB14 suction substitution", "WB14 conductivity substitution", "Ksat used directly as current K",
        "S_psi used for K", "wrong conductivity exponent", "wrong clamp order", "positive psi_sat",
        "zero or negative B", "missing root-tissue path", "CLM default injected", "root path aliased to dxroot",
        "root path aliased to layer depth", "gravity sign reversal", "wrong layer order", "wrong OFE/lane/layer/stratum",
        "wrong hydrology state digest", "wrong V10 configuration digest", "wrong LSE configuration digest",
        "caller-created receipt"]
    (ARTIFACTS / "poison-matrix.md").write_text("# Root-zone poison matrix\n\n" + "\n".join(
        f"- `{p}`: reject before receipt construction; live owners unchanged." for p in poisons) + "\n")
    (ARTIFACTS / "equation-and-operation-order.md").write_text("# Equation and operation order\n\n" +
        "\n".join(f"{i}. `{op}`" for i, op in enumerate(model["operation_order"], 1)) +
        "\n\n`libm 0.2.16::pow`; exact binary64-bit comparison; positive-zero normalization.\n")
    (ARTIFACTS / "test-vector-ledger.md").write_text("# Test-vector ledger\n\n" + "\n".join(
        f"- `{v['name']}`: {v['disposition']}" for v in vectors + rejects) + "\n")
    (ARTIFACTS / "reference-calculator.py").write_bytes(Path(__file__).read_bytes())


if __name__ == "__main__":
    main()
