#!/usr/bin/env python3
"""Analyze the frozen Stage 3 parent-cost attribution series."""

import json
import statistics
from pathlib import Path


ROOT = Path(__file__).parent / "raw" / "attribution-series"


def summary(values):
    return {
        "values": values,
        "min": min(values),
        "median": statistics.median(values),
        "max": max(values),
    }


receipts = [
    json.loads(path.read_text())
    for path in sorted(ROOT.glob("[0-9][0-9].json"))
]
measured = [receipt for receipt in receipts if receipt["kind"] == "measured"]
on = [receipt for receipt in measured if receipt["posture"] == "on"]
off = [receipt for receipt in measured if receipt["posture"] == "off"]
costs = {}
for receipt in on:
    profile = receipt["record"]["compact_cost"]
    for name, elapsed, entries in zip(
        profile["bucket_order"], profile["exclusive_ns"], profile["entries"]
    ):
        row = costs.setdefault(name, {"seconds": [], "share": [], "entries": []})
        row["seconds"].append(elapsed / 1e9)
        row["share"].append(elapsed / profile["runner_ns"])
        row["entries"].append(entries)
costs = {
    name: {
        "exclusive_seconds": summary(row["seconds"]),
        "runner_share": summary(row["share"]),
        "entries": sorted(set(row["entries"])),
    }
    for name, row in costs.items()
}
pairs = []
for pair in range(6):
    left, right = [receipt for receipt in measured if receipt["pair"] == pair]
    by_posture = {receipt["posture"]: receipt for receipt in (left, right)}
    off_receipt, on_receipt = by_posture["off"], by_posture["on"]
    pairs.append(
        {
            "pair": pair,
            "wall_fraction": (
                on_receipt["record"]["run_wall_us"]
                / off_receipt["record"]["run_wall_us"]
                - 1
            ),
            "process_cpu_fraction": (
                on_receipt["process_cpu_s"] / off_receipt["process_cpu_s"] - 1
            ),
        }
    )
result = {
    "valid_processes": sum(receipt["valid"] for receipt in receipts),
    "accounting_difference_ns": [
        receipt["record"]["compact_cost"]["accounting_difference_ns"]
        for receipt in on
    ],
    "runner_seconds": {
        "off": summary([receipt["record"]["run_wall_us"] / 1e6 for receipt in off]),
        "on": summary([receipt["record"]["run_wall_us"] / 1e6 for receipt in on]),
    },
    "process_cpu_seconds": {
        "off": summary([receipt["process_cpu_s"] for receipt in off]),
        "on": summary([receipt["process_cpu_s"] for receipt in on]),
    },
    "rss_kib": {
        "off": summary([receipt["record"]["rss_kib"] for receipt in off]),
        "on": summary([receipt["record"]["rss_kib"] for receipt in on]),
    },
    "paired_observer_perturbation": {
        "wall_fraction": summary([pair["wall_fraction"] for pair in pairs]),
        "process_cpu_fraction": summary(
            [pair["process_cpu_fraction"] for pair in pairs]
        ),
        "pairs": pairs,
    },
    "costs": costs,
}
print(json.dumps(result, indent=2, sort_keys=True, allow_nan=False))
