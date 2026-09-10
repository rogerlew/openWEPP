#!/usr/bin/env python3
"""Reconstruct per-run exclusive costs; paired overhead, not summed medians."""
import argparse
import hashlib
import json
from pathlib import Path
import statistics


def distribution(values):
    if not values:
        raise ValueError("empty distribution")
    return {"median": statistics.median(values), "min": min(values), "max": max(values),
            "values": values}


def reconstruct(profile):
    names, elapsed = profile["bucket_order"], profile["exclusive_ns"]
    total = profile["runner_ns"]
    if (type(total) is not int or total <= 0 or len(names) != len(elapsed)
            or len(names) != len(set(names))
            or any(type(v) is not int or v < 0 for v in elapsed)):
        raise ValueError("invalid exact cost operands")
    difference = total - sum(elapsed)
    if difference != profile["accounting_difference_ns"] or difference != 0:
        raise ValueError("nonconserving cost profile")
    return {name: {"ns": value, "share": value / total}
            for name, value in zip(names, elapsed)}


def analyze(rows):
    measured = [row for row in rows if row.get("kind", row.get("label")) == "measured"]
    if len(measured) != 12 or any(not row["valid"] for row in rows):
        raise ValueError("expected twelve valid measured rows, no invalid attempts")
    pairs = {}
    for row in measured:
        posture = row.get("posture", row.get("arm"))
        posture = {"A": "off", "B": "on"}.get(posture, posture)
        pair = pairs.setdefault(row["pair"], {})
        if posture not in ("off", "on") or posture in pair:
            raise ValueError("duplicate or unknown posture")
        pair[posture] = row
    if len(pairs) != 6 or any(set(pair) != {"off", "on"} for pair in pairs.values()):
        raise ValueError("incomplete balanced pairs")
    ordered = [pairs[key] for key in sorted(pairs)]
    on = [pair["on"]["record"] for pair in ordered]
    off = [pair["off"]["record"] for pair in ordered]
    reconstructed = [reconstruct(record["compact_cost"]) for record in on]
    names = list(reconstructed[0])
    if any(list(row) != names for row in reconstructed):
        raise ValueError("bucket schema drift")
    for record in on:
        if record["compact_cost"]["runner_ns"] // 1000 != record["run_wall_us"]:
            raise ValueError("runner denominator mismatch")
    populations = [record["compact_cost"]["populations"] for record in on]
    if any(row != populations[0] for row in populations):
        raise ValueError("population drift")
    costs = {name: {"exclusive_seconds": distribution([row[name]["ns"] / 1e9 for row in reconstructed]),
                    "runner_share": distribution([row[name]["share"] for row in reconstructed])}
             for name in names}
    builder_descendants = [
        "CarrierPhysicalValue", "CarrierLowerBoundaryCompletion",
        "NativePhysicalEvaluationRemainder", "NativeForcingAdaptation",
        "NativeFixedFinalPreparation", "NativeFixedFinalPreflight",
        "NativeEffectiveHydrologyFrameClone", "NativeEffectiveHydrologyAdapter",
        "NativeFixedFinalStateProjection", "NativeFixedFinalVegetationProjection",
        "NativeFixedFinalCanopyReceipt", "NativeFixedFinalSoilPreparation",
        "NativeFixedFinalPhysicalCall", "NativeOriginalHydrologyAdapter",
        "NativeWb14ParentJoin", "NativeUnifiedHydrologyCandidate",
        "NativePhysicalResultAdmission", "NativeExperimentalDiagnostics",
    ]
    builder_share = [sum(row[name]["share"] for name in builder_descendants)
                     for row in reconstructed]
    builder_seconds = [sum(row[name]["ns"] for name in builder_descendants) / 1e9
                       for row in reconstructed]
    covered_fixed_final_nested_ns = [
        sum(record["compact_cost"]["family_exclusive_ns"][1]) for record in on
    ]
    if any(record["compact_cost"]["populations"][1]["family"] != "CoveredFixedFinal"
           or record["compact_cost"]["populations"][1]["solves"] != 400
           or record["compact_cost"]["populations"][1]["sweeps"] != 0
           for record in on):
        raise ValueError("covered fixed-final inclusive-child identity drift")
    builder_inclusive_seconds = [
        seconds + nested / 1e9
        for seconds, nested in zip(builder_seconds, covered_fixed_final_nested_ns)
    ]
    builder_inclusive_share = [
        share + nested / record["compact_cost"]["runner_ns"]
        for share, nested, record in zip(builder_share, covered_fixed_final_nested_ns, on)
    ]
    wall_perturbation = [right["run_wall_us"] / left["run_wall_us"] - 1
                         for left, right in zip(off, on)]
    cpu_perturbation = [(pair["on"]["process_cpu_s"] / pair["off"]["process_cpu_s"] - 1)
                        for pair in ordered]
    return {
        "evidence_class": "profiled exclusive costs; hypothetical savings are optimistic models",
        "measured_pairs": 6,
        "off_runner_seconds": distribution([r["run_wall_us"] / 1e6 for r in off]),
        "on_runner_seconds": distribution([r["run_wall_us"] / 1e6 for r in on]),
        "paired_runner_wall_perturbation": distribution(wall_perturbation),
        "paired_process_cpu_perturbation": distribution(cpu_perturbation),
        "costs": costs,
        "populations": populations[0],
        "builder_previously_exclusive_partition": {
            "source_backed_exclusive_descendants": builder_descendants,
            "seconds": distribution(builder_seconds),
            "runner_share": distribution(builder_share),
            "accounting_note": "Sum of mutually exclusive descendants that subdivide the former CarrierPhysicalValue exclusive bucket. CarrierPhysicalValue is now only its residual; CarrierPhysicalSetup/Evidence/Completion and nested whole-run LSE/kernel buckets are excluded."
        },
        "builder_inclusive_envelope": {
            "seconds": distribution(builder_inclusive_seconds),
            "runner_share": distribution(builder_inclusive_share),
            "nested_covered_fixed_final_seconds": distribution([
                value / 1e9 for value in covered_fixed_final_nested_ns]),
            "source_backed_derivation": "Previously-exclusive descendant partition plus family_exclusive_ns[CoveredFixedFinal]. Source call graph places this 400-solve, zero-sweep family inside NativeFixedFinalPhysicalCall. The nested family remains separately represented in the whole-run exclusive partition and is added only for this local inclusive envelope."
        },
        "selection_candidates": {
            name: {
                "exclusive_seconds": costs[name]["exclusive_seconds"],
                "perfect_elimination_ceiling": costs[name]["runner_share"],
                "hypothetical_2x_saved_fraction": distribution([
                    row[name]["share"] * 0.5 for row in reconstructed]),
                "hypothetical_5x_saved_fraction": distribution([
                    row[name]["share"] * 0.8 for row in reconstructed]),
            }
            for name in ("NativeFixedFinalPhysicalCall",
                         "NativeUnifiedHydrologyCandidate",
                         "NativeFixedFinalSoilPreparation",
                         "NativeFixedFinalStateProjection",
                         "NativeEffectiveHydrologyAdapter",
                         "NativeOriginalHydrologyAdapter")
        },
        "accounting_difference_ns": [0] * 6,
        "limitations": ["One OFE day only; same-thread attribution.",
                        "Builder inclusive is the descendant partition plus CoveredFixedFinal family sum; do not add it again to the whole-run partition.",
                        "Medians are descriptive and are not a reconciled synthetic run.",
                        "No global overhead subtraction from individual buckets."]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--series", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    jsonl = args.series / "results.jsonl"
    paths = [jsonl] if jsonl.exists() else sorted(args.series.glob("[0-9][0-9].json"))
    if not paths:
        raise ValueError("no retained result receipts")
    rows = ([json.loads(line) for line in jsonl.read_text().splitlines() if line]
            if jsonl.exists() else [json.loads(path.read_text()) for path in paths])
    for row in rows:
        log = Path(row["log"])
        retained_log = args.series / log.name
        if retained_log.exists():
            log = retained_log
        if hashlib.sha256(log.read_bytes()).hexdigest() != row["log_sha256"]:
            raise ValueError("raw log hash mismatch")
    result = analyze(rows)
    result["receipt_hashes"] = {path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}
    with args.out.open("x") as output:
        json.dump(result, output, indent=2, sort_keys=True, allow_nan=False)
        output.write("\n")


if __name__ == "__main__":
    main()
