#!/usr/bin/env python3
"""Localize the frozen Paradise WY2015 S/Q support omissions."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
FREEZE = PACKAGE / "artifacts/protocol-freeze.json"
PARENT_TOOL = REPO / (
    "docs/work-packages/20260807-snow-stage3-evolving-state-carrier-"
    "plausibility-reconciliation-001/tools/analyze_evolving_carrier.py"
)
TERMS = ("shortwave", "longwave", "sensible", "latent", "advected")


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def require_close(label: str, actual: float, expected: float) -> None:
    tolerance = max(1.0e-6, 1.0e-12 * (abs(actual) + abs(expected)))
    if abs(actual - expected) > tolerance:
        raise RuntimeError(
            f"{label} mismatch: {actual:.17g} != {expected:.17g} "
            f"(tolerance {tolerance:.17g})"
        )


def trace_path(lane: dict[str, Any]) -> Path:
    outputs = lane["outputs"]
    item = next(value for key, value in outputs.items() if key.endswith(".snow.jsonl"))
    return REPO / item["path"]


def tuple_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    if not rows:
        return {
            "tuple_count": 0,
            "support_seconds": 0.0,
            "applicability_reasons": [],
            "after_surface_reasons": [],
            "ice_before_kg_m2": None,
            "ice_after_kg_m2": None,
            "after_surface_applicable": None,
        }
    return {
        "tuple_count": len(rows),
        "support_seconds": sum(float(row["duration_seconds"]) for row in rows),
        "applicability_reasons": sorted({str(row.get("applicability_reason")) for row in rows}),
        "after_surface_reasons": sorted(
            {str(row.get("after_surface_applicability_reason")) for row in rows}
        ),
        "ice_before_kg_m2": float(rows[0]["total_ice_mass_before_kg_m2"]),
        "ice_after_kg_m2": float(rows[-1]["total_ice_mass_after_kg_m2"]),
        "after_surface_applicable": bool(rows[-1].get("after_surface_applicable")),
    }


def classify(s_rows: list[dict[str, Any]], q_rows: list[dict[str, Any]]) -> str | None:
    if bool(s_rows) != bool(q_rows):
        return "UNMATCHED_S_ONLY" if s_rows else "UNMATCHED_Q_ONLY"
    if not s_rows:
        return None
    support = min(
        sum(float(row["duration_seconds"]) for row in s_rows),
        sum(float(row["duration_seconds"]) for row in q_rows),
        3600.0,
    )
    return "PARTIAL_COMMON_SUPPORT" if support < 3600.0 else None


def hourly_status(row: dict[str, Any], hour: int) -> dict[str, Any]:
    companion = row.get("stage3_operator_reconciliation")
    statuses = companion.get("hourly_status") if isinstance(companion, dict) else None
    if not isinstance(statuses, list) or len(statuses) != 24:
        raise RuntimeError("missing 24-hour operator status")
    status = statuses[hour]
    if not isinstance(status, dict):
        raise RuntimeError("malformed operator status")
    return {"evaluated": bool(status.get("evaluated")), "reason": str(status.get("reason"))}


def localize(freeze: dict[str, Any]) -> dict[str, Any]:
    parent = load_module("paradise_parent_analyzer", PARENT_TOOL)
    predecessor = parent.load_module("paradise_predecessor", parent.PREDECESSOR)
    carrier = parent.load_module("paradise_carrier", parent.CARRIER)
    receipt_path = REPO / freeze["inputs"]["operator_receipt"]["path"]
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    site = freeze["site"]
    paired_lane = receipt["lanes"][site]["paired"]
    sequential_lane = receipt["lanes"][site]["sequential"]
    paired_path = trace_path(paired_lane)
    sequential_path = trace_path(sequential_lane)
    climate_path = REPO / paired_lane["runfile_consumer"]["climate_path"]
    observation = (
        REPO / "target/snow_stage3_operator_reconciliation_v3/inputs/observations"
        / f"{site}.csv"
    )
    dates = carrier.climate_dates(climate_path)
    peaks, _ = carrier.observed_peaks(observation)
    peak = peaks[freeze["water_year"]][0]
    affected: list[dict[str, Any]] = []
    totals: defaultdict[str, float] = defaultdict(float)
    counts: Counter[str] = Counter()
    with paired_path.open(encoding="utf-8") as paired, sequential_path.open(
        encoding="utf-8"
    ) as sequential:
        for index, (paired_line, sequential_line) in enumerate(
            zip(paired, sequential, strict=True)
        ):
            stamp = dates[index]
            if parent.water_year(stamp) != freeze["water_year"] or not parent.in_window(
                stamp, peak, freeze["water_year"]
            ):
                continue
            p_row = json.loads(paired_line)
            q_row = json.loads(sequential_line)
            parent.validate_trace_identity(p_row, index)
            parent.validate_trace_identity(q_row, index)
            p_tuples = predecessor.validate_v6_row(
                p_row, "same_state_paired_carrier_v1", site
            )
            q_tuples = predecessor.validate_v6_row(
                q_row, "sequential_resolved_shadow_v1", site
            )
            parent.validate_joined_identity(
                p_row, q_row, p_tuples, q_tuples, predecessor
            )
            s_hours = predecessor.tuples_by_hour(p_tuples)
            q_hours = predecessor.tuples_by_hour(q_tuples)
            first_q = next((row for hour in q_hours for row in hour), None)
            for hour in range(24):
                support_class = classify(s_hours[hour], q_hours[hour])
                if support_class is None:
                    continue
                frozen = (
                    parent.construct_frozen_active(s_hours[hour][0], first_q, predecessor)
                    if s_hours[hour] and first_q is not None
                    else None
                )
                reduced, reduced_counts = parent.reduce_joined_hour(
                    s_hours[hour], q_hours[hour], frozen, first_q, predecessor
                )
                counts.update(reduced_counts)
                omitted_terms = {}
                for term in TERMS:
                    omitted_term = sum(
                        abs(
                            float(reduced.get(f"{prefix}_all_{term}_j_m2", 0.0))
                            - float(reduced.get(f"{prefix}_{term}_j_m2", 0.0))
                        )
                        for prefix in ("S", "Q")
                    )
                    omitted_terms[term] = omitted_term
                    totals[f"omitted_{term}_j_m2"] += omitted_term
                omitted = float(reduced["omitted_magnitude_j_m2"])
                totals["omitted_magnitude_j_m2"] += omitted
                counts[support_class] += 1
                affected.append(
                    {
                        "date": stamp.isoformat(),
                        "hour_index": hour,
                        "support_class": support_class,
                        "S_status": hourly_status(p_row, hour),
                        "Q_status": hourly_status(q_row, hour),
                        "S": tuple_summary(s_hours[hour]),
                        "Q": tuple_summary(q_hours[hour]),
                        "omitted_by_term_j_m2": omitted_terms,
                        "omitted_magnitude_j_m2": omitted,
                    }
                )
    return {
        "schema": "snow-stage3-paradise-wy2015-support-localization-v1",
        "site": site,
        "water_year": freeze["water_year"],
        "window_end": peak.isoformat(),
        "support_threshold": freeze["support_threshold"],
        "affected_hours": affected,
        "counts": dict(sorted(counts.items())),
        "totals": dict(sorted(totals.items())),
    }


def validate(result: dict[str, Any], freeze: dict[str, Any]) -> None:
    expected = freeze["expected_parent"]
    counts = result["counts"]
    if counts.get("unmatched_hour_count") != expected["unmatched_hour_count"]:
        raise RuntimeError("unmatched-hour reconciliation failed")
    if counts.get("partial_support_hour_count") != expected["partial_support_hour_count"]:
        raise RuntimeError("partial-hour reconciliation failed")
    if len(result["affected_hours"]) != (
        expected["unmatched_hour_count"] + expected["partial_support_hour_count"]
    ):
        raise RuntimeError("affected-hour inventory is not exact")
    require_close(
        "omitted magnitude",
        float(result["totals"]["omitted_magnitude_j_m2"]),
        float(expected["omitted_magnitude_j_m2"]),
    )
    if math.isclose(freeze["support_threshold"], expected["support_omission_ratio"]):
        raise RuntimeError("threshold was aliased to the observed ratio")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--verify-retained", action="store_true")
    args = parser.parse_args()
    if not args.verify_retained:
        raise RuntimeError("--verify-retained is mandatory")
    if args.output.exists():
        raise RuntimeError("refusing to overwrite immutable output")
    tracked = subprocess.run(
        ["git", "status", "--porcelain", "--untracked-files=no"],
        cwd=REPO,
        text=True,
        capture_output=True,
        check=True,
    ).stdout
    if tracked:
        raise RuntimeError("result-bearing execution requires clean tracked files")
    freeze = json.loads(FREEZE.read_text(encoding="utf-8"))
    for item in freeze["inputs"].values():
        path = REPO / item["path"]
        if sha256(path) != item["sha256"]:
            raise RuntimeError(f"custody mismatch: {path}")
    result = localize(freeze)
    validate(result, freeze)
    result["analysis_head"] = subprocess.run(
        ["git", "rev-parse", "HEAD"], cwd=REPO, text=True, capture_output=True, check=True
    ).stdout.strip()
    args.output.mkdir(parents=True)
    result_path = args.output / "support-localization.json"
    result_path.write_text(
        json.dumps(result, allow_nan=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    receipt = {
        "schema": "snow-stage3-paradise-wy2015-support-execution-receipt-v1",
        "analysis_head": result["analysis_head"],
        "protocol_sha256": sha256(FREEZE),
        "result_sha256": sha256(result_path),
        "command": sys.argv,
    }
    (args.output / "execution-receipt.json").write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(json.dumps({"affected_hours": len(result["affected_hours"]), **result["counts"]}))


if __name__ == "__main__":
    main()
