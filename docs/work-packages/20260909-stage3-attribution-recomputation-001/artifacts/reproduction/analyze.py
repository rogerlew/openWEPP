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
    target = [row["Assembly"]["share"] + row["Linear"]["share"] for row in reconstructed]
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
        "assembly_plus_linear": {
            "perfect_elimination_ceiling": distribution(target),
            "hypothetical_2x_saved_fraction": distribution([v * 0.5 for v in target]),
            "hypothetical_5x_saved_fraction": distribution([v * 0.8 for v in target]),
            "deprioritize_by_owner_5_percent_rule": statistics.median(target) < 0.05,
            "limitations": "Not all assembly/linear work is removable; mapping/reconstruction/guards remain. No cubic prediction."},
        "accounting_difference_ns": [0] * 6,
        "limitations": ["One OFE day only; same-thread attribution.",
                        "Do not add inclusive LSE to these exclusive children.",
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
