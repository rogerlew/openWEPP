#!/usr/bin/env python3
"""Compare exact EB-04W outputs with the retained EB-04V v2 consumer."""

from __future__ import annotations

import datetime as dt
import hashlib
import importlib.util
import itertools
import json
import math
import sys
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
CURRENT_RUNS = REPO / "target/snow_surface_eb04w_accumulation_diagnostics/runs"
REFERENCE_RUNS = REPO / "target/snow_surface_eb04v_density_diagnostics/runs"
CURRENT_RECEIPT = PACKAGE / "artifacts/execution-receipt.json"
REFERENCE_RECEIPT = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04v-density-structure-mechanics-001/"
    "artifacts/execution-receipt.json"
)
RESULT = PACKAGE / "artifacts/behavior-neutrality.json"
OBSERVED_HARNESS = REPO / "tools/snowfreeze_observed/observed_harness.py"
LANES = (
    "snotel_mica_creek_st_joe_id",
    "snotel_niwot_co",
    "snotel_paradise_wa",
    "snotel_snowbird_ut",
)
CELLS = ("B", "L", "S", "LS")


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


harness = load_module("eb04w_behavior_observed_harness", OBSERVED_HARNESS)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def compare_value(left: Any, right: Any, path: str) -> tuple[int, float]:
    if isinstance(left, bool) or isinstance(right, bool):
        if left != right:
            raise RuntimeError(f"boolean mismatch at {path}: {left!r} != {right!r}")
        return 0, 0.0
    if isinstance(left, (int, float)) and isinstance(right, (int, float)):
        if not math.isfinite(float(left)) or not math.isfinite(float(right)):
            if left != right:
                raise RuntimeError(f"non-finite mismatch at {path}: {left!r} != {right!r}")
            return 1, 0.0
        difference = abs(float(left) - float(right))
        if difference != 0.0:
            raise RuntimeError(f"numeric mismatch at {path}: {left!r} != {right!r}")
        return 1, difference
    if isinstance(left, dict) and isinstance(right, dict):
        if set(left) != set(right):
            raise RuntimeError(f"object-key mismatch at {path}")
        count = 0
        maximum = 0.0
        for key in sorted(left):
            child_count, child_maximum = compare_value(left[key], right[key], f"{path}.{key}")
            count += child_count
            maximum = max(maximum, child_maximum)
        return count, maximum
    if isinstance(left, list) and isinstance(right, list):
        if len(left) != len(right):
            raise RuntimeError(f"array-length mismatch at {path}")
        count = 0
        maximum = 0.0
        for index, (left_item, right_item) in enumerate(zip(left, right)):
            child_count, child_maximum = compare_value(
                left_item, right_item, f"{path}[{index}]"
            )
            count += child_count
            maximum = max(maximum, child_maximum)
        return count, maximum
    if left != right:
        raise RuntimeError(f"value mismatch at {path}: {left!r} != {right!r}")
    return 0, 0.0


def compare_wat(current: Path, reference: Path) -> tuple[int, int, float]:
    current_rows = harness.load_modeled_wat(current)
    reference_rows = harness.load_modeled_wat(reference)
    if set(current_rows) != set(reference_rows):
        raise RuntimeError(f"WAT date mismatch: {current} vs {reference}")
    numeric_count = 0
    maximum = 0.0
    for date in sorted(current_rows):
        count, difference = compare_value(
            current_rows[date], reference_rows[date], f"wat.{date.isoformat()}"
        )
        numeric_count += count
        maximum = max(maximum, difference)
    return len(current_rows), numeric_count, maximum


def compare_trace(current: Path, reference: Path) -> tuple[int, int, int, float]:
    row_count = 0
    numeric_count = 0
    prior_field_count: int | None = None
    maximum = 0.0
    with current.open(encoding="utf-8") as current_handle, reference.open(
        encoding="utf-8"
    ) as reference_handle:
        for row_index, pair in enumerate(
            itertools.zip_longest(current_handle, reference_handle), start=1
        ):
            current_line, reference_line = pair
            if current_line is None or reference_line is None:
                raise RuntimeError(f"trace row-count mismatch at row {row_index}")
            current_row = json.loads(current_line)
            reference_row = json.loads(reference_line)
            if current_row.get("schema") != "openwepp-r7h-direct-production-snow-trace-v3":
                raise RuntimeError("current trace is not v3")
            if reference_row.get("schema") != "openwepp-r7h-direct-production-snow-trace-v2":
                raise RuntimeError("reference trace is not retained v2")
            prior_fields = set(reference_row) - {"schema"}
            if not prior_fields.issubset(current_row):
                raise RuntimeError(
                    f"v3 omits prior v2 fields: {sorted(prior_fields - set(current_row))}"
                )
            if prior_field_count is None:
                prior_field_count = len(prior_fields)
            elif prior_field_count != len(prior_fields):
                raise RuntimeError("reference v2 top-level field set changed within trace")
            for key in sorted(prior_fields):
                count, difference = compare_value(
                    current_row[key], reference_row[key], f"trace[{row_index}].{key}"
                )
                numeric_count += count
                maximum = max(maximum, difference)
            row_count += 1
    return row_count, prior_field_count or 0, numeric_count, maximum


def main() -> int:
    current_receipt = json.loads(CURRENT_RECEIPT.read_text(encoding="utf-8"))
    totals = {
        "cell_count": 0,
        "wat_row_count": 0,
        "wat_numeric_value_count": 0,
        "trace_row_count": 0,
        "trace_numeric_value_count": 0,
        "prior_v2_top_level_field_count": None,
        "maximum_wat_numeric_difference": 0.0,
        "maximum_prior_v2_trace_numeric_difference": 0.0,
    }
    cells = {}
    for lane in LANES:
        for cell in CELLS:
            stem = f"{lane}-{cell}"
            current_dir = CURRENT_RUNS / lane / cell
            reference_dir = REFERENCE_RUNS / lane / cell
            current_wat = current_dir / f"{stem}.wat.parquet"
            reference_wat = reference_dir / f"{stem}.wat.parquet"
            current_trace = current_dir / f"{stem}.snow.jsonl"
            reference_trace = reference_dir / f"{stem}.snow.jsonl"
            wat = compare_wat(
                current_wat,
                reference_wat,
            )
            trace = compare_trace(
                current_trace,
                reference_trace,
            )
            if totals["prior_v2_top_level_field_count"] is None:
                totals["prior_v2_top_level_field_count"] = trace[1]
            elif totals["prior_v2_top_level_field_count"] != trace[1]:
                raise RuntimeError("prior-v2 field count differs across cells")
            cells[f"{lane}/{cell}"] = {
                "wat_rows": wat[0],
                "wat_numeric_values": wat[1],
                "trace_rows": trace[0],
                "trace_numeric_values": trace[2],
                "current_wat_sha256": sha256(current_wat),
                "reference_wat_sha256": sha256(reference_wat),
                "current_trace_sha256": sha256(current_trace),
                "reference_trace_sha256": sha256(reference_trace),
            }
            totals["cell_count"] += 1
            totals["wat_row_count"] += wat[0]
            totals["wat_numeric_value_count"] += wat[1]
            totals["trace_row_count"] += trace[0]
            totals["trace_numeric_value_count"] += trace[2]
            totals["maximum_wat_numeric_difference"] = max(
                totals["maximum_wat_numeric_difference"], wat[2]
            )
            totals["maximum_prior_v2_trace_numeric_difference"] = max(
                totals["maximum_prior_v2_trace_numeric_difference"], trace[3]
            )
    result = {
        "schema": "snow-surface-eb04w-terminal-behavior-neutrality-v1",
        "generated_utc": dt.datetime.now(dt.UTC).isoformat(),
        "execution_command": (
            ".venv/bin/python docs/work-packages/"
            "20260801-snow-surface-eb-04w-accumulation-under-persistence-001/"
            "tools/compare_terminal_behavior.py"
        ),
        "working_directory": str(REPO),
        "comparison_tool_sha256": sha256(Path(__file__)),
        "observed_harness_sha256": sha256(OBSERVED_HARNESS),
        "current_binary_sha256": current_receipt["binary_sha256"],
        "current_receipt": str(CURRENT_RECEIPT.relative_to(REPO)),
        "current_receipt_sha256": sha256(CURRENT_RECEIPT),
        "reference_receipt": str(REFERENCE_RECEIPT.relative_to(REPO)),
        "reference_receipt_sha256": sha256(REFERENCE_RECEIPT),
        "comparison": "decoded exact EB-04W v3 versus retained exact EB-04V v2",
        "totals": totals,
        "cells": cells,
        "status": "PASS",
    }
    RESULT.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": "PASS", **totals}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
