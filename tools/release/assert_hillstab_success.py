#!/usr/bin/env python3
"""Assert pass/fail conditions for hillstab01 stability harness JSON output."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def parse_expect_suite(raw: str) -> tuple[str, int]:
    if "=" not in raw:
        raise argparse.ArgumentTypeError(
            f"invalid --expect-suite value '{raw}' (expected suite=count)"
        )
    suite, count_text = raw.split("=", 1)
    suite = suite.strip()
    if not suite:
        raise argparse.ArgumentTypeError(
            f"invalid --expect-suite value '{raw}' (missing suite name)"
        )
    try:
        count = int(count_text)
    except ValueError as exc:  # pragma: no cover - parser error path
        raise argparse.ArgumentTypeError(
            f"invalid --expect-suite value '{raw}' (count is not an integer)"
        ) from exc
    if count < 0:
        raise argparse.ArgumentTypeError(
            f"invalid --expect-suite value '{raw}' (count must be >= 0)"
        )
    return suite, count


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--results-json", type=Path, required=True)
    parser.add_argument(
        "--expect-suite",
        action="append",
        default=[],
        type=parse_expect_suite,
        help="Required suite size assertion in form suite=count; may be repeated",
    )
    parser.add_argument(
        "--require-suite",
        action="append",
        default=[],
        help="Require suite presence in results; may be repeated",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    payload = json.loads(args.results_json.read_text(encoding="utf-8"))
    summaries = payload.get("suite_summaries")
    if not isinstance(summaries, dict):
        raise ValueError("results JSON missing object field 'suite_summaries'")

    failures: list[str] = []
    for suite_name, summary in sorted(summaries.items()):
        if not isinstance(summary, dict):
            failures.append(f"suite '{suite_name}' summary is not an object")
            continue
        total = int(summary.get("total", 0))
        passed = int(summary.get("passed", 0))
        failed = int(summary.get("failed", 0))
        if total <= 0:
            failures.append(f"suite '{suite_name}' has non-positive total ({total})")
        if failed != 0:
            failures.append(
                f"suite '{suite_name}' reports failed={failed} (passed={passed}, total={total})"
            )

    for suite_name in args.require_suite:
        if suite_name not in summaries:
            failures.append(f"required suite '{suite_name}' missing from results")

    for suite_name, expected_total in args.expect_suite:
        summary = summaries.get(suite_name)
        if not isinstance(summary, dict):
            failures.append(f"expected suite '{suite_name}' missing from results")
            continue
        total = int(summary.get("total", 0))
        passed = int(summary.get("passed", 0))
        failed = int(summary.get("failed", 0))
        if total != expected_total:
            failures.append(
                f"suite '{suite_name}' total mismatch: expected {expected_total}, observed {total}"
            )
        if passed != expected_total or failed != 0:
            failures.append(
                f"suite '{suite_name}' pass/fail mismatch for expected full pass "
                f"(expected passed={expected_total}, failed=0; observed passed={passed}, failed={failed})"
            )

    if failures:
        for line in failures:
            print(f"FAIL: {line}")
        return 1

    print(f"PASS: stability results validated ({args.results_json})")
    for suite_name, summary in sorted(summaries.items()):
        print(
            f"PASS: suite={suite_name} total={summary.get('total')} "
            f"passed={summary.get('passed')} failed={summary.get('failed')}"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
