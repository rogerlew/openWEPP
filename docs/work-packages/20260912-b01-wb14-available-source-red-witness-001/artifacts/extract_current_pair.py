#!/usr/bin/env python3
"""Preserve the current run's WB14 guard/caller rows with bounded memory."""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
from decimal import Decimal
from pathlib import Path
from typing import Any, BinaryIO

KINDS = {
    "surface_liquid_wb14_cadence_failure",
    "surface_liquid_wb14_cadence_caller_failure",
}


class HashingReader:
    def __init__(self, stream: BinaryIO) -> None:
        self.stream = stream
        self.digest = hashlib.sha256()

    def read(self, size: int = -1) -> bytes:
        data = self.stream.read(size)
        self.digest.update(data)
        return data


def sha256(path: Path) -> tuple[int, str]:
    size = 0
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while chunk := stream.read(1024 * 1024):
            size += len(chunk)
            digest.update(chunk)
    return size, digest.hexdigest()


def decimal_safe(value: Any) -> Any:
    if isinstance(value, list):
        return [decimal_safe(item) for item in value]
    if isinstance(value, dict):
        return {key: decimal_safe(item) for key, item in value.items()}
    return value


def lossless_json(value: Any) -> str:
    """Encode Decimal tokens as JSON numbers without converting them to float."""
    if isinstance(value, Decimal):
        return str(value)
    if value is None or isinstance(value, (bool, int, float, str)):
        return json.dumps(value, ensure_ascii=False, allow_nan=False)
    if isinstance(value, list):
        return "[" + ",".join(lossless_json(item) for item in value) + "]"
    if isinstance(value, dict):
        return "{" + ",".join(json.dumps(str(key), ensure_ascii=False) + ":" + lossless_json(item) for key, item in value.items()) + "}"
    fail(f"unsupported JSON value type: {type(value).__name__}")


def fail(message: str) -> None:
    raise RuntimeError(message)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--index", type=Path, required=True)
    parser.add_argument("--run", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    try:
        import ijson
    except ImportError as exc:
        fail(f"ijson is required from the repository environment: {exc}")

    index = json.loads(args.index.read_text())
    indexed = next((item for item in index["files"] if item["path"] == str(args.source)), None)
    if indexed is None:
        fail(f"source is absent from raw-capture index: {args.source}")
    source_size, source_hash = sha256(args.source)
    if source_size != indexed["bytes"] or source_hash != indexed["sha256"]:
        fail("source hash/size disagrees with raw-capture index")

    run = json.loads(args.run.read_text())
    run_size, run_hash = sha256(args.run)
    receipt = json.loads(args.receipt.read_text())
    receipt_size, receipt_hash = sha256(args.receipt)
    if receipt_hash != indexed_hash(index, str(args.receipt)):
        fail("receipt hash disagrees with raw-capture index")
    if run_hash != indexed_hash(index, str(args.run)):
        fail("run hash disagrees with raw-capture index")
    if run_hash != receipt.get("runner_receipt_sha256"):
        fail("run hash disagrees with receipt runner_receipt_sha256")
    raw_root = args.source.parent
    for name in ("receipt.json", "run.json", "started.json", "inputs.json", "stdout.log", "stderr.log"):
        path = raw_root / name
        actual_size, actual_hash = sha256(path)
        expected_hash = indexed_hash(index, str(path))
        if actual_hash != expected_hash:
            fail(f"{name} hash disagrees with raw-capture index")
        receipt_hash_for_name = receipt.get("artifact_sha256", {}).get(name)
        if receipt_hash_for_name is not None and actual_hash != receipt_hash_for_name:
            fail(f"receipt artifact hash disagrees for {name}")
    if receipt.get("artifact_sha256", {}).get("observations.json") != source_hash:
        fail("receipt observations hash does not bind current source")
    if run.get("execution") != "FAIL" or run.get("error") is None:
        fail("current run does not retain the expected failed execution")
    if receipt.get("runner_execution") != "FAIL" or receipt.get("execution_valid") is not False:
        fail("current receipt does not retain FAIL/execution_valid=false")

    rows: list[dict[str, Any]] = []
    physical_count = 0
    with args.source.open("rb") as raw:
        reader = HashingReader(raw)
        for ordinal, row in enumerate(ijson.items(reader, "physical.item", use_float=False)):
            physical_count = ordinal + 1
            if isinstance(row, dict) and row.get("kind") in KINDS:
                rows.append({"physical_ordinal": ordinal, "record": decimal_safe(row)})
        if reader.digest.hexdigest() != source_hash:
            fail("stream parse hash disagrees with independently hashed source")
    if not rows:
        fail("no WB14 guard/caller records found")
    if receipt.get("physical_rows") != physical_count:
        fail(f"physical row count {physical_count} disagrees with receipt {receipt.get('physical_rows')}")

    top_level: dict[str, Any] = {}
    with args.source.open("rb") as raw:
        for prefix, event, value in ijson.parse(raw, use_float=False):
            if "." not in prefix and event in {"boolean", "number", "string", "null"}:
                top_level[prefix] = value
    completeness = {key: decimal_safe(top_level[key]) for key in top_level if key.endswith("complete") or key in {"counts_poisoned", "overflow", "physical_enabled"}}

    command = [sys.executable, str(Path(__file__).resolve()), *sys.argv[1:]]
    packet = {
        "schema": "current_run_boundary_packet_v1",
        "evidence_class": "Ran: complete-stream preservation extraction; witness NOT EVALUATED",
        "source": {"path": str(args.source), "bytes": source_size, "sha256": source_hash},
        "stream": {"array_prefix": "physical.item", "records_scanned": physical_count, "matching_records": len(rows), "trailing_validation": "ijson complete stream parse"},
        "top_level_scalars": decimal_safe(top_level),
        "completeness_flags": completeness,
        "receipt_completeness": {key: receipt.get(key) for key in ("observation_complete", "files_complete", "days_complete") if key in receipt},
        "records": rows,
        "run": {"path": str(args.run), "bytes": run_size, "sha256": run_hash, "execution": run.get("execution"), "error": run.get("error")},
        "receipt": {"path": str(args.receipt), "bytes": receipt_size, "sha256": receipt_hash, "argv": receipt.get("argv"), "binary_sha256": receipt.get("binary_sha256"), "case": receipt.get("case"), "policy": receipt.get("policy"), "execution_valid": receipt.get("execution_valid")},
        "indexed_artifacts": {item["path"]: item["sha256"] for item in index["files"] if item["path"].endswith(("stdout.log", "stderr.log", "run.json", "receipt.json"))},
        "extraction": {"command": command, "script": str(Path(__file__).resolve()), "script_sha256": sha256(Path(__file__).resolve())[1], "ijson_use_float": False, "decimal_encoding": "Decimal values are JSON strings preserving exact lexical value; typed-byte arrays are retained as arrays."},
        "limitations": ["Raw multi-GB observations remain local; this compact packet is not a complete raw backup.", "Extraction preserves records and does not perform witness acceptance."],
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(lossless_json(packet) + "\n")
    print(json.dumps({"source_bytes": source_size, "source_sha256": source_hash, "records_scanned": receipt.get("physical_rows"), "matching_records": len(rows), "output": str(args.output)}))
    return 0


def indexed_hash(index: dict[str, Any], path: str) -> str:
    item = next((item for item in index["files"] if item["path"] == path), None)
    if item is None:
        fail(f"file is absent from raw-capture index: {path}")
    return item["sha256"]


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (RuntimeError, KeyError, ValueError, json.JSONDecodeError) as exc:
        print(f"extract_current_pair: ERROR: {exc}", file=sys.stderr)
        raise SystemExit(2)
