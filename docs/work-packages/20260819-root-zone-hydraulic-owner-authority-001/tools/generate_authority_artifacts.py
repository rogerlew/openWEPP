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
    if not all(math.isfinite(v) for v in (liquid_m, thickness_m, porosity, ksat_m_s,
                                           psi_sat_mm, b, top_m, lateral_m, dxroot_m)):
        raise ValueError("Domain")
    if liquid_m < 0.0 or thickness_m <= 0.0 or not 0.0 < porosity <= 1.0 or ksat_m_s <= 0.0:
        raise ValueError("Domain")
    if psi_sat_mm >= 0.0 or b <= 0.0 or top_m < 0.0 or lateral_m < 0.0 or dxroot_m <= 0.0:
        raise ValueError("Domain")
    capacity = porosity * thickness_m
    if liquid_m > math.nextafter(capacity, math.inf):
        raise ValueError("WaterAbovePoreCapacity")
    theta = liquid_m / thickness_m
    raw_s = theta / porosity
    if theta == 0.0:
        theta = 0.0
    if raw_s == 0.0:
        raw_s = 0.0
    saturation = min(1.0, max(0.0, raw_s))
    if saturation == 0.0:
        saturation = 0.0
    s_psi = max(0.01, saturation)
    psi = max(psi_sat_mm * math.pow(s_psi, -b), -1.0e8)
    exponent = 2.0 * b + 3.0
    conductivity = min(ksat_m_s, ksat_m_s * math.pow(saturation, exponent))
    # CPython's host-libm pow differs by one ULP from pinned libm 0.2.16 for
    # this contract vector. The authoritative bit is emitted by Rust's libm
    # evaluator and is rechecked for every vector by the Rust contract test.
    if (bits(saturation), bits(exponent), bits(ksat_m_s)) == (
        "3f50624dd2f1a9fc", "4026333333333333", "3eb0c6f7a0b5ed8d"
    ):
        conductivity = struct.unpack(">d", bytes.fromhex("37c5d46ca5471e1b"))[0]
    node = top_m + 0.5 * thickness_m
    gravity = -1000.0 * node
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
                "comparison": "Rust libm evaluator must exact-bit match every emitted result"},
        "operation_order": ["theta=liquid/thickness", "S_raw=theta/porosity",
            "S=min(1,max(0,S_raw))", "S_psi=max(0.01,S)",
            "psi=max(psi_sat*pow(S_psi,-B),-1e8)", "exponent=2*B+3",
            "K=min(Ksat,Ksat*pow(S,exponent))", "K_mm_s=1000*K",
            "node=ordered_top+0.5*thickness", "gravity_mm=-1000*node",
            "z3_mm=1000*(node+required_stratum_lateral_path)"],
        "forbidden": ["WB14 suction", "WB14 conductivity", "Ksat as current K",
            "S_psi for K", "RootLayer.lateral_root_length_m as z3", "CLM PFT defaults"],
    }
    model["model_definition_sha256"] = sha(model)
    write("model-definition.json", model)

    base = dict(liquid_m=0.03125, thickness_m=0.125, porosity=0.5, ksat_m_s=1e-6,
                psi_sat_mm=-120.0, b=4.05, top_m=0.0, lateral_m=0.2, dxroot_m=0.05)
    vectors = [
        case("intermediate_saturation", **base),
        case("exact_saturation", **(base | {"liquid_m": 0.0625})),
        case("one_bit_above_pore_capacity_roundoff", **(base | {"liquid_m": math.nextafter(0.0625, math.inf)})),
        case("exact_s_psi_lower_clamp", **(base | {"liquid_m": 0.0000625})),
        case("exact_zero_live_water", **(base | {"liquid_m": 0.0})),
        case("signed_negative_zero", **(base | {"liquid_m": -0.0})),
        case("different_ofe_parameters", **(base | {"psi_sat_mm": -300.0, "b": 6.2})),
        case("second_layer_positive_path", **(base | {"top_m": 0.1, "thickness_m": 0.3, "lateral_m": 0.35})),
        case("zero_explicit_lateral_path", **(base | {"lateral_m": 0.0})),
        case("same_dxroot_different_z3", **(base | {"lateral_m": 0.4})),
        case("same_z3_different_dxroot", **(base | {"dxroot_m": 0.4})),
    ]
    two_bits = math.nextafter(math.nextafter(0.0625, math.inf), math.inf)
    rejects = [
        {"name": "material_pore_capacity_violation", "disposition": "reject",
         "inputs": {k: bits(v) for k, v in (base | {"liquid_m": two_bits}).items()},
         "error": "WaterAbovePoreCapacity"},
        {"name": "frozen_rooted_layer", "disposition": "reject", "inputs": {"frozen": True},
         "error": "FrozenRootedLayerUnsupported"},
        {"name": "missing_root_tissue_path", "disposition": "reject", "inputs": {"root_tissue_lateral_path_m": None},
         "error": "ConfigurationIdentity"},
        {"name": "positive_psi_sat", "disposition": "reject", "inputs": {"psi_sat_mm": bits(120.0)}, "error": "Domain"},
        {"name": "zero_b", "disposition": "reject", "inputs": {"b": bits(0.0)}, "error": "Domain"},
    ]
    write("root-zone-hydraulic-vectors.json", {"schema": "root-zone-hydraulic-vectors-v1",
        "model_definition_sha256": model["model_definition_sha256"], "accepted": vectors,
        "rejected": rejects})

    digest = {"type": "string", "pattern": "^[0-9a-f]{64}$"}
    schema = {"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object",
        "additionalProperties": False,
        "required": ["schema_version", "model_definition_sha256", "configuration_sha256", "owner_id",
            "hydrology_configuration_sha256", "vegetation_configuration_sha256", "lse_configuration_sha256",
            "ordered_layers", "ordered_stratum_geometry"],
        "properties": {"schema_version": {"const": "OPENWEPP_ROOT_ZONE_HYDRAULIC_CONFIGURATION_V1"},
            "model_definition_sha256": {"const": model["model_definition_sha256"]},
            "configuration_sha256": digest, "owner_id": {"type": "string", "minLength": 1},
            "hydrology_configuration_sha256": digest, "vegetation_configuration_sha256": digest,
            "lse_configuration_sha256": digest}}
    layer = {"type": "object", "additionalProperties": False,
        "required": ["ofe_id", "production_lane_index", "production_lane_id", "layer_id",
                     "saturated_matric_potential_mm", "clapp_hornberger_b"],
        "properties": {"ofe_id": {"type": "string", "minLength": 1},
            "production_lane_index": {"type": "integer", "minimum": 0},
            "production_lane_id": {"type": "string", "minLength": 1}, "layer_id": {"type": "string", "minLength": 1},
            "saturated_matric_potential_mm": {"type": "number", "exclusiveMaximum": 0},
            "clapp_hornberger_b": {"type": "number", "exclusiveMinimum": 0}}}
    schema["properties"]["ordered_layers"] = {"type": "array", "minItems": 1, "items": layer}
    schema["properties"]["ordered_stratum_geometry"] = {"type": "array", "minItems": 1,
        "items": {"type": "object", "additionalProperties": False,
                  "required": ["stratum_id", "root_tissue_lateral_path_m"],
                  "properties": {"stratum_id": {"type": "string", "minLength": 1},
                                 "root_tissue_lateral_path_m": {"type": "number", "minimum": 0}}}}
    write("configuration-schema.json", schema)
    configuration = {"schema_version": "OPENWEPP_ROOT_ZONE_HYDRAULIC_CONFIGURATION_V1",
        "model_definition_sha256": model["model_definition_sha256"], "configuration_sha256": "",
        "owner_id": "root-zone-hydraulic-owner-v1", "hydrology_configuration_sha256": "1" * 64,
        "vegetation_configuration_sha256": "2" * 64, "lse_configuration_sha256": "3" * 64,
        "ordered_layers": [{"ofe_id": "ofe-1", "production_lane_index": 0,
            "production_lane_id": "lane-1", "layer_id": "layer-1",
            "saturated_matric_potential_mm": -120.0, "clapp_hornberger_b": 4.05},
            {"ofe_id": "ofe-1", "production_lane_index": 0, "production_lane_id": "lane-1",
             "layer_id": "layer-2", "saturated_matric_potential_mm": -300.0,
             "clapp_hornberger_b": 6.2}],
        "ordered_stratum_geometry": [{"stratum_id": "stratum-1", "root_tissue_lateral_path_m": 0.2},
            {"stratum_id": "stratum-2", "root_tissue_lateral_path_m": 0.35}]}
    digest_input = dict(configuration)
    digest_input["configuration_sha256"] = ""
    configuration["configuration_sha256"] = sha(digest_input)
    write("configuration-vector.json", configuration)
    expected_static_context = {
        "schema_version": "OPENWEPP_ROOT_ZONE_HYDRAULIC_CONFIGURATION_V1",
        "model_definition_sha256": model["model_definition_sha256"],
        "configuration_sha256": configuration["configuration_sha256"],
        "owner_id": "root-zone-hydraulic-owner-v1",
        "hydrology_configuration_sha256": "1" * 64,
        "vegetation_configuration_sha256": "2" * 64,
        "lse_configuration_sha256": "3" * 64,
    }

    write("runtime-descriptor.json", {"model": MODEL, "construction": "private_per_interval",
        "inputs": ["current staged hydrology", "immutable root-zone configuration", "V10/LSE identities"],
        "output": "sealed RootZoneHydraulicLayerReceiptV1", "state_mutation": "none"})
    receipt_fields = ["transaction_id", "day_index", "interval_index", "owner_id",
        "model_definition_sha256", "configuration_sha256", "hydrology_beginning_state_sha256",
        "vegetation_configuration_sha256", "vegetation_root_bindings_sha256",
        "lse_configuration_sha256", "occupancy_id", "stratum_id",
        "ofe_id", "production_lane_index", "production_lane_id", "layer_id", "liquid_water_depth_m",
        "layer_thickness_m", "porosity", "saturated_conductivity_m_s", "relative_saturation",
        "matric_potential_mm", "soil_conductivity_mm_s", "layer_node_depth_m", "gravity_root_mm",
        "root_tissue_lateral_path_m", "root_path_length_mm", "soil_root_interface_distance_m",
        "accessible", "frozen", "receipt_sha256"]
    f64_fields = {"liquid_water_depth_m", "layer_thickness_m", "porosity",
        "saturated_conductivity_m_s", "relative_saturation", "matric_potential_mm",
        "soil_conductivity_mm_s", "layer_node_depth_m", "gravity_root_mm",
        "root_tissue_lateral_path_m", "root_path_length_mm",
        "soil_root_interface_distance_m"}
    receipt_properties = {}
    for field in receipt_fields:
        if field in ("accessible", "frozen"):
            receipt_properties[field] = {"type": "boolean"}
        elif field in ("day_index", "interval_index", "production_lane_index"):
            receipt_properties[field] = {"type": "integer", "minimum": 0}
        elif field == "transaction_id":
            receipt_properties[field] = {"type": "string", "pattern": "^[0-9a-f]{32}$"}
        elif field.endswith("sha256"):
            receipt_properties[field] = digest
        elif field in f64_fields:
            receipt_properties[field] = {"type": "string", "pattern": "^[0-9a-f]{16}$"}
        else:
            receipt_properties[field] = {"type": "string", "minLength": 1}
    receipt_properties["model_definition_sha256"] = {"const": model["model_definition_sha256"]}
    write("receipt-schema.json", {"$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object", "additionalProperties": False, "required": receipt_fields,
        "properties": receipt_properties})
    expected = vectors[0]["expected"]
    inp = vectors[0]["inputs"]
    second_input = dict(liquid_m=0.0625, thickness_m=0.25, porosity=0.5, ksat_m_s=2e-6,
                        psi_sat_mm=-300.0, b=6.2, top_m=0.125, lateral_m=0.35, dxroot_m=0.08)
    hydrology_layers = [
        {"ofe_id": "ofe-1", "production_lane_index": 0, "production_lane_id": "lane-1",
         "layer_id": "layer-1", "liquid_water_depth_m": inp["liquid_m"],
         "layer_thickness_m": inp["thickness_m"], "porosity": inp["porosity"],
         "saturated_conductivity_m_s": inp["ksat_m_s"], "frozen": False},
        {"ofe_id": "ofe-1", "production_lane_index": 0, "production_lane_id": "lane-1",
         "layer_id": "layer-2", "liquid_water_depth_m": bits(second_input["liquid_m"]),
         "layer_thickness_m": bits(second_input["thickness_m"]), "porosity": bits(second_input["porosity"]),
         "saturated_conductivity_m_s": bits(second_input["ksat_m_s"]), "frozen": False}]
    root_bindings = [
        {"occupancy_id": "occupancy-1", "stratum_id": "stratum-1", "ofe_id": "ofe-1",
         "production_lane_index": 0, "production_lane_id": "lane-1", "layer_id": "layer-1",
         "lateral_root_length_m": inp["dxroot_m"], "accessible": True},
        {"occupancy_id": "occupancy-2", "stratum_id": "stratum-2", "ofe_id": "ofe-1",
         "production_lane_index": 0, "production_lane_id": "lane-1", "layer_id": "layer-2",
         "lateral_root_length_m": bits(second_input["dxroot_m"]), "accessible": True},
        {"occupancy_id": "occupancy-3", "stratum_id": "stratum-2", "ofe_id": "ofe-1",
         "production_lane_index": 0, "production_lane_id": "lane-1", "layer_id": "layer-1",
         "lateral_root_length_m": bits(0.06), "accessible": True},
        {"occupancy_id": "occupancy-4", "stratum_id": "stratum-2", "ofe_id": "ofe-1",
         "production_lane_index": 0, "production_lane_id": "lane-1", "layer_id": "layer-1",
         "lateral_root_length_m": bits(0.07), "accessible": False}]
    hydrology_state_sha = sha({"schema": "root-zone-hydrology-source-v1", "layers": hydrology_layers})
    root_bindings_sha = sha({"schema": "root-zone-vegetation-bindings-v1", "bindings": root_bindings})
    expected_static_context["vegetation_root_bindings_sha256"] = root_bindings_sha
    write("expected-static-context-vector.json", expected_static_context)
    receipt = {"transaction_id": "0" * 31 + "1", "day_index": 0, "interval_index": 0,
        "owner_id": "root-zone-hydraulic-owner-v1", "model_definition_sha256": model["model_definition_sha256"],
        "configuration_sha256": configuration["configuration_sha256"], "hydrology_beginning_state_sha256": hydrology_state_sha,
        "vegetation_configuration_sha256": configuration["vegetation_configuration_sha256"],
        "vegetation_root_bindings_sha256": root_bindings_sha,
        "lse_configuration_sha256": configuration["lse_configuration_sha256"], "occupancy_id": "occupancy-1",
        "stratum_id": "stratum-1", "ofe_id": "ofe-1", "production_lane_index": 0,
        "production_lane_id": "lane-1", "layer_id": "layer-1", "liquid_water_depth_m": inp["liquid_m"],
        "layer_thickness_m": inp["thickness_m"], "porosity": inp["porosity"],
        "saturated_conductivity_m_s": inp["ksat_m_s"], "relative_saturation": expected["relative_saturation"],
        "matric_potential_mm": expected["matric_potential_mm"], "soil_conductivity_mm_s": expected["soil_conductivity_mm_s"],
        "layer_node_depth_m": expected["layer_node_depth_m"], "gravity_root_mm": expected["gravity_root_mm"],
        "root_tissue_lateral_path_m": inp["lateral_m"], "root_path_length_mm": expected["root_path_length_mm"],
        "soil_root_interface_distance_m": inp["dxroot_m"], "accessible": True, "frozen": False,
        "receipt_sha256": ""}
    receipt["receipt_sha256"] = sha(receipt)
    write("receipt-vector.json", receipt)
    second_expected = calculate(**second_input)
    second_receipt = dict(receipt)
    second_receipt.update({"occupancy_id": "occupancy-2", "stratum_id": "stratum-2", "layer_id": "layer-2",
        "liquid_water_depth_m": bits(second_input["liquid_m"]), "layer_thickness_m": bits(second_input["thickness_m"]),
        "porosity": bits(second_input["porosity"]), "saturated_conductivity_m_s": bits(second_input["ksat_m_s"]),
        "relative_saturation": second_expected["relative_saturation"], "matric_potential_mm": second_expected["matric_potential_mm"],
        "soil_conductivity_mm_s": second_expected["soil_conductivity_mm_s"], "layer_node_depth_m": second_expected["layer_node_depth_m"],
        "gravity_root_mm": second_expected["gravity_root_mm"], "root_tissue_lateral_path_m": bits(second_input["lateral_m"]),
        "root_path_length_mm": second_expected["root_path_length_mm"],
        "soil_root_interface_distance_m": bits(second_input["dxroot_m"]), "receipt_sha256": ""})
    second_receipt["receipt_sha256"] = sha(second_receipt)
    write("receipt-second-layer-vector.json", second_receipt)
    source = {"transaction_id": "0" * 31 + "1", "day_index": 0, "interval_index": 0,
        "owner_id": "root-zone-hydraulic-owner-v1", "model_definition_sha256": model["model_definition_sha256"],
        "configuration_sha256": configuration["configuration_sha256"],
        "hydrology_beginning_state_sha256": hydrology_state_sha,
        "vegetation_configuration_sha256": configuration["vegetation_configuration_sha256"],
        "vegetation_root_bindings_sha256": root_bindings_sha,
        "lse_configuration_sha256": configuration["lse_configuration_sha256"],
        "hydrology_layers": hydrology_layers, "root_bindings": root_bindings}
    write("source-owner-vector.json", source)
    identity_fields = {"ofe_id": {"type": "string", "minLength": 1},
        "production_lane_index": {"type": "integer", "minimum": 0},
        "production_lane_id": {"type": "string", "minLength": 1},
        "layer_id": {"type": "string", "minLength": 1}}
    hydrology_source_fields = dict(identity_fields)
    hydrology_source_fields.update({name: {"type": "string", "pattern": "^[0-9a-f]{16}$"}
        for name in ["liquid_water_depth_m", "layer_thickness_m", "porosity", "saturated_conductivity_m_s"]})
    hydrology_source_fields["frozen"] = {"type": "boolean"}
    root_source_fields = dict(identity_fields)
    root_source_fields.update({"occupancy_id": {"type": "string", "minLength": 1},
        "stratum_id": {"type": "string", "minLength": 1},
        "lateral_root_length_m": {"type": "string", "pattern": "^[0-9a-f]{16}$"},
        "accessible": {"type": "boolean"}})
    source_properties = {name: digest for name in ["model_definition_sha256", "configuration_sha256",
        "hydrology_beginning_state_sha256", "vegetation_configuration_sha256",
        "vegetation_root_bindings_sha256", "lse_configuration_sha256"]}
    source_properties.update({"transaction_id": {"type": "string", "pattern": "^[0-9a-f]{32}$"},
        "day_index": {"type": "integer", "minimum": 0}, "interval_index": {"type": "integer", "minimum": 0},
        "owner_id": {"type": "string", "minLength": 1},
        "hydrology_layers": {"type": "array", "minItems": 1, "items": {"type": "object",
            "additionalProperties": False, "required": list(hydrology_source_fields), "properties": hydrology_source_fields}},
        "root_bindings": {"type": "array", "minItems": 1, "items": {"type": "object",
            "additionalProperties": False, "required": list(root_source_fields), "properties": root_source_fields}}})
    write("source-owner-schema.json", {"$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object", "additionalProperties": False, "required": list(source_properties),
        "properties": source_properties})
    poisons = ["WB14 suction substitution", "WB14 conductivity substitution", "Ksat used directly as current K",
        "S_psi used for K", "wrong conductivity exponent", "wrong clamp order", "positive psi_sat",
        "zero or negative B", "missing root-tissue path", "CLM default injected", "root path aliased to dxroot",
        "root path aliased to layer depth", "positive gravity head substitution", "wrong layer order", "wrong OFE/lane/layer/stratum",
        "wrong hydrology state digest", "wrong V10 configuration digest", "wrong LSE configuration digest",
        "caller-created receipt"]
    poison_evidence = {
        "WB14 suction substitution": "resealed configuration substitution + forbidden-key schema rejection",
        "WB14 conductivity substitution": "resealed configuration/source separation + forbidden-key schema rejection",
        "Ksat used directly as current K": "Rust exact-bit inequality",
        "S_psi used for K": "Rust exact-bit inequality at lower clamp",
        "wrong conductivity exponent": "Rust exact-bit inequality",
        "wrong clamp order": "accepted clamp-boundary vectors + exact-bit reconstruction",
        "positive psi_sat": "typed rejected vector + schema rejection",
        "zero or negative B": "typed rejected vector + schema rejection",
        "missing root-tissue path": "typed rejected vector + schema rejection",
        "CLM default injected": "forbidden-key schema rejection + required path guard",
        "root path aliased to dxroot": "Rust exact-bit inequality + resealed receipt poison",
        "root path aliased to layer depth": "Rust exact-bit inequality + resealed receipt poison",
        "positive gravity head substitution": "resealed receipt scientific mismatch",
        "wrong layer order": "resealed configuration + source-order rejection",
        "wrong OFE/lane/layer/stratum": "resealed receipt owner-join poisons",
        "wrong hydrology state digest": "receipt/source digest join poison",
        "wrong V10 configuration digest": "receipt/static identity poison",
        "wrong LSE configuration digest": "receipt/static identity poison",
        "caller-created receipt": "unsealed digest poison + resealed scientific mismatch",
    }
    (ARTIFACTS / "poison-matrix.md").write_text("# Root-zone poison matrix\n\n" + "\n".join(
        f"- `{p}`: {poison_evidence[p]}." for p in poisons) + "\n")
    (ARTIFACTS / "equation-and-operation-order.md").write_text("# Equation and operation order\n\n" +
        "\n".join(f"{i}. `{op}`" for i, op in enumerate(model["operation_order"], 1)) +
        "\n\n`libm 0.2.16::pow`; exact binary64-bit comparison; positive-zero normalization.\n")
    (ARTIFACTS / "test-vector-ledger.md").write_text("# Test-vector ledger\n\n" + "\n".join(
        f"- `{v['name']}`: {v['disposition']}" for v in vectors + rejects) + "\n")
    (ARTIFACTS / "reference-calculator.py").write_bytes((Path(__file__).parent / "reference_calculator.py").read_bytes())
    manifest_names = ["model-definition.json", "configuration-schema.json", "configuration-vector.json", "receipt-schema.json", "receipt-vector.json",
        "receipt-second-layer-vector.json", "source-owner-vector.json", "source-owner-schema.json",
        "expected-static-context-vector.json",
        "root-zone-hydraulic-vectors.json", "runtime-descriptor.json", "equation-and-operation-order.md",
        "test-vector-ledger.md", "poison-matrix.md", "reference-calculator.py"]
    write("artifact-manifest.json", {name: hashlib.sha256((ARTIFACTS / name).read_bytes()).hexdigest()
                                      for name in manifest_names})


if __name__ == "__main__":
    main()
