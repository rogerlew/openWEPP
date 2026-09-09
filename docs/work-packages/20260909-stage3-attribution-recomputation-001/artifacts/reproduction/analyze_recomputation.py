#!/usr/bin/env python3
"""Summarize bounded carrier records without treating hashes as cache authority."""
import argparse
import json
from collections import defaultdict
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--trace", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    records = json.loads(args.trace.read_text())["records"]
    providers = [row for row in records if row["kind"] == "Provider"]
    by_input = defaultdict(list)
    by_evaluator = defaultdict(list)
    for row in providers:
        by_input[row["input"]].append(row)
        by_evaluator[row["evaluator_ordinal"]].append(row)
    repeated = [rows for rows in by_input.values() if len(rows) > 1]
    result = {
        "provider_executions": len(providers),
        "unique_recorded_request_inputs": len(by_input),
        "repeated_input_groups": len(repeated),
        "repeated_input_executions": sum(map(len, repeated)),
        "redundant_executions_by_recorded_key": sum(len(rows) - 1 for rows in repeated),
        "repeated_groups_single_output": sum(
            len({row["output"] for row in rows}) == 1 for rows in repeated
        ),
        "repeated_groups_multiple_outputs": sum(
            len({row["output"] for row in rows}) > 1 for rows in repeated
        ),
        "evaluator_groups": len(by_evaluator),
        "evaluator_groups_two_calls": sum(len(rows) == 2 for rows in by_evaluator.values()),
        "evaluator_groups_distinct_request_same_output": sum(
            len(rows) == 2
            and len({row["input"] for row in rows}) == 2
            and len({row["output"] for row in rows}) == 1
            for rows in by_evaluator.values()
        ),
        "interpretation": (
            "Recorded request equality is insufficient effective-input authority: "
            "some equal recorded inputs produced multiple outputs. The two-call-per-"
            "evaluator pattern is the already-established F feed-forward mechanism."
        ),
    }
    with args.out.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")


if __name__ == "__main__":
    main()
