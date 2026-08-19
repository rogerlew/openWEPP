#!/usr/bin/env python3
"""Independent schema, manifest, and semantic-calculator validation."""

import hashlib
import importlib.util
import json
import struct
from pathlib import Path

import jsonschema

ROOT = Path(__file__).resolve().parents[1]
ART = ROOT / "artifacts"


def number(hex_bits: str) -> float:
    return struct.unpack(">d", bytes.fromhex(hex_bits))[0]


def ulp_distance(a: str, b: str) -> int:
    return abs(int(a, 16) - int(b, 16))


def main() -> None:
    manifest = json.loads((ART / "artifact-manifest.json").read_text())
    for name, expected in manifest.items():
        assert hashlib.sha256((ART / name).read_bytes()).hexdigest() == expected, name

    config_schema = json.loads((ART / "configuration-schema.json").read_text())
    receipt_schema = json.loads((ART / "receipt-schema.json").read_text())
    config = json.loads((ART / "configuration-vector.json").read_text())
    receipt = json.loads((ART / "receipt-vector.json").read_text())
    jsonschema.Draft202012Validator.check_schema(config_schema)
    jsonschema.Draft202012Validator.check_schema(receipt_schema)
    jsonschema.Draft202012Validator(config_schema).validate(config)
    jsonschema.Draft202012Validator(receipt_schema).validate(receipt)
    for schema, value in ((config_schema, config), (receipt_schema, receipt)):
        poisoned = dict(value)
        poisoned["invented"] = True
        assert list(jsonschema.Draft202012Validator(schema).iter_errors(poisoned))
    for forbidden in ("wb14_suction_mm", "wb14_conductivity_m_s", "clm_default_root_path_m",
                      "root_path_length_mm", "lateral_root_length_m"):
        poisoned = dict(config)
        poisoned[forbidden] = 1.0
        assert list(jsonschema.Draft202012Validator(config_schema).iter_errors(poisoned)), forbidden
    missing = json.loads(json.dumps(config))
    del missing["ordered_stratum_geometry"][0]["root_tissue_lateral_path_m"]
    assert list(jsonschema.Draft202012Validator(config_schema).iter_errors(missing))
    positive_psi = json.loads(json.dumps(config))
    positive_psi["ordered_layers"][0]["saturated_matric_potential_mm"] = 1.0
    assert list(jsonschema.Draft202012Validator(config_schema).iter_errors(positive_psi))
    zero_b = json.loads(json.dumps(config))
    zero_b["ordered_layers"][0]["clapp_hornberger_b"] = 0.0
    assert list(jsonschema.Draft202012Validator(config_schema).iter_errors(zero_b))

    spec = importlib.util.spec_from_file_location("reference_calculator", ROOT / "tools/reference_calculator.py")
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    vectors = json.loads((ART / "root-zone-hydraulic-vectors.json").read_text())
    keys = ("liquid_m", "thickness_m", "porosity", "ksat_m_s", "psi_sat_mm", "b", "top_m", "lateral_m")
    expected_keys = ("relative_saturation", "retention_saturation", "matric_potential_mm",
                     "conductivity_exponent", "current_conductivity_m_s", "layer_node_depth_m",
                     "gravity_root_mm", "root_path_length_mm")
    for vector in vectors["accepted"]:
        observed = module.solve(*(number(vector["inputs"][key]) for key in keys))
        expected = [vector["expected"][key] for key in expected_keys]
        for index, (left, right) in enumerate(zip(observed, expected, strict=True)):
            limit = 1 if index in (2, 4) else 0
            assert ulp_distance(left, right) <= limit, (vector["name"], expected_keys[index], left, right)
    print("PASS: manifest, schemas, and independent calculator")


if __name__ == "__main__":
    main()
