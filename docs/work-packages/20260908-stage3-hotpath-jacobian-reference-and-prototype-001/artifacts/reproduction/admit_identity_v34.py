#!/usr/bin/env python3
"""Freeze exact-outcome A/J identity; not scientific admission or tolerance policy.

Reuse the existing collector without its F-specific work-counter exception.
If exact parity fails, preserve the failure and adjudicate under canonical v34;
this shortcut never declares a permitted numerical trajectory a physics defect.
"""
import argparse
import importlib.util
import json
from pathlib import Path

COLLECTOR = (Path(__file__).resolve().parents[3]
             / "20260906-stage3-prospective-mechanism-experiments-001"
             / "artifacts/reproduction/run_series.py")
SPEC = importlib.util.spec_from_file_location("stage3_common_series", COLLECTOR)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("required canonical collector cannot be loaded")
COMMON = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMON)

RULES = {
    "/invoked_utc": "invoked_utc", "/run_dir": "run_path",
    "/run_file": "run_path", "/input_checksums": "run_path_keys",
    "/output_checksums": "run_path_keys",
    "/stage3_evidence_archive/output_path": "run_path",
    "/wat5_output/output_path": "run_path",
    **{"/resolved_sidecars/" + key: "run_path" for key in
       ("frost", "pmetpara", "snow", "snow_stage3_v11_owner_seed", "wepp_ui")},
}


def identity(record, binary, source):
    binary = Path(binary).resolve()
    sidecar = Path(str(binary) + ".json")
    metadata = json.loads(sidecar.read_text())
    expected = {
        "binary_path": str(binary), "binary_sha256": COMMON.digest(binary),
        "binary_sidecar_path": str(sidecar),
        "binary_sidecar_sha256": COMMON.digest(sidecar),
        "source_commit": source["checkout"],
    }
    if (metadata["sha256"] != expected["binary_sha256"]
            or metadata["source_commit"] != expected["source_commit"]):
        raise ValueError("sidecar does not bind exact executable and checkout")
    rules = dict(RULES)
    # Intentionally no FRAME_POINTER/F treatment exception: actual raw work
    # counters in the manifest must match for this exact-parity shortcut.
    for field, value in expected.items():
        if record["output_manifest"][field] != value:
            raise ValueError("manifest provenance mismatch: " + field)
        rules["/" + field] = {"expected_arm_identity": value}
    return {
        "manifest_rules": rules,
        "common_identity": COMMON.record_identity(record, rules),
        "carrier_counts": record["carrier_counts"],
        "lse_counts": record["lse_counts"],
        "source_identity": source["source_identity"],
        "collector_sha256": COMMON.digest(COLLECTOR),
        "evidence_class": "exact-outcome identity only; v34 scientific gates separate",
    }


def main():
    parser = argparse.ArgumentParser()
    for name in ("log", "binary", "source-manifest", "out"):
        parser.add_argument("--" + name, type=Path, required=True)
    parser.add_argument("--compare", type=Path)
    args = parser.parse_args()
    records = [json.loads(line.split("STAGE3_CONTROLLED_MECHANISM ", 1)[1])
               for line in args.log.read_text().splitlines()
               if "STAGE3_CONTROLLED_MECHANISM " in line]
    if len(records) != 1:
        raise ValueError("expected one successful consumer record")
    result = identity(records[0], args.binary,
                      json.loads(args.source_manifest.read_text()))
    if args.compare:
        other = json.loads(args.compare.read_text())
        if COMMON.exact_json(result["common_identity"]) != COMMON.exact_json(other["common_identity"]):
            raise ValueError("exact parity absent; retain evidence and apply v34 numerical classes")
    result["log_sha256"] = COMMON.digest(args.log)
    with args.out.open("x") as output:
        json.dump(result, output, indent=2, allow_nan=False)
    print(json.dumps({"status": "IDENTITY_FROZEN", "out": str(args.out)}))


if __name__ == "__main__":
    main()
