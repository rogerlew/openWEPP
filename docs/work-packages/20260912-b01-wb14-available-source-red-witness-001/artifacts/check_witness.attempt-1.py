#!/usr/bin/env python3
"""Fail-closed offline verifier for the single B01 WB14 red witness."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

FAILURE = "surface_liquid_wb14_cadence_failure"
CALLER = "surface_liquid_wb14_cadence_caller_failure"
ERROR = "SURFACELIQUID-E-008"
EXPECTED_TEST = "hillslope::tests::stage3_snow_accuracy_case"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def integer(value: object, name: str) -> int:
    if isinstance(value, bool):
        raise ValueError(f"{name}: boolean is not an integer")
    if isinstance(value, int):
        return value
    if isinstance(value, str) and value and value.lstrip("-").isdigit():
        return int(value, 10)
    raise ValueError(f"{name}: expected integer or decimal string")


def bytes_json(value: object, name: str) -> dict:
    if not isinstance(value, list) or any(not isinstance(x, int) or x < 0 or x > 255 for x in value):
        raise ValueError(f"{name}: expected raw byte array")
    decoded = json.loads(bytes(value).decode("utf-8"))
    if not isinstance(decoded, dict):
        raise ValueError(f"{name}: decoded value is not an object")
    return decoded


def need(obj: dict, key: str, label: str) -> object:
    if key not in obj:
        raise ValueError(f"missing {label}.{key}")
    return obj[key]


def check(observation_path: Path, receipt_path: Path, binary: Path) -> dict:
    observation = json.loads(observation_path.read_bytes())
    receipt = json.loads(receipt_path.read_bytes())
    if observation.get("schema") != "snow_accuracy_observation_v2":
        raise ValueError("wrong observation schema")
    if observation.get("physical_enabled") is not True or observation.get("overflow") is not False or observation.get("counts_poisoned") is not False:
        raise ValueError("observer-loss, overflow, or poisoned counts")
    rows = observation.get("physical")
    if not isinstance(rows, list):
        raise ValueError("missing physical rows")
    failures = [(i, row) for i, row in enumerate(rows) if isinstance(row, dict) and row.get("kind") == FAILURE]
    callers = [(i, row) for i, row in enumerate(rows) if isinstance(row, dict) and row.get("kind") == CALLER]
    if len(failures) != 1 or len(callers) != 1:
        raise ValueError("missing or ambiguous paired cadence records")
    fi, failure = failures[0]
    ci, caller = callers[0]
    if ERROR not in str(need(caller, "error", "caller")):
        raise ValueError("generic panic or wrong continuation boundary")
    f_input = bytes_json(need(failure, "input_typed_bytes", "failure"), "failure.input_typed_bytes")
    c_input = bytes_json(need(caller, "input_typed_bytes", "caller"), "caller.input_typed_bytes")
    if f_input != c_input:
        raise ValueError("guard/caller input bytes differ")
    values = {
        "day_index": 4, "interval_index": 22, "transaction_id": 255, "interval_s": 60,
    }
    for key, expected in values.items():
        if integer(need(f_input, key, "input"), f"input.{key}") != expected:
            raise ValueError(f"wrong boundary {key}")
    if failure.get("actual_day") != 4 or failure.get("actual_interval") != 22:
        raise ValueError("guard record has wrong actual day/interval")
    if failure.get("parent_child_mode") is not True or failure.get("finalize_parent_interval") is not False:
        raise ValueError("wrong parent-child or final posture")
    binding = need(caller, "coupled_binding", "caller")
    if not isinstance(binding, dict):
        raise ValueError("missing coupled binding")
    for key, expected in {
        "parent_support_start_ns": 385200000000000,
        "parent_support_end_ns": 387000000000000,
        "child_support_start_ns": 385920000000000,
        "child_support_end_ns": 385980000000000,
    }.items():
        if integer(need(binding, key, "binding"), f"binding.{key}") != expected:
            raise ValueError(f"wrong {key}")
    if receipt.get("timeout") is not False or receipt.get("binary_unchanged") is not True:
        raise ValueError("timeout or binary drift")
    if receipt.get("execution_valid") is not False or receipt.get("runner_execution") != "FAIL":
        raise ValueError("runner verdict is not the expected FAIL")
    argv = receipt.get("argv")
    if not isinstance(argv, list) or EXPECTED_TEST not in argv or str(binary) not in argv:
        raise ValueError("wrong binary or test selection in receipt")
    if receipt.get("binary_sha256") != sha256(binary):
        raise ValueError("binary hash mismatch")
    return {
        "witness_verdict": "PASS",
        "runner_verdict": "FAIL",
        "observation_sha256": sha256(observation_path),
        "receipt_sha256": sha256(receipt_path),
        "failure_row": fi,
        "caller_row": ci,
        "input": f_input,
        "binding": binding,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("observation", type=Path)
    parser.add_argument("receipt", type=Path)
    parser.add_argument("binary", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    result = check(args.observation, args.receipt, args.binary)
    text = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.write_text(text)
    else:
        print(text, end="")


if __name__ == "__main__":
    main()
