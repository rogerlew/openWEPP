#!/usr/bin/env python3
"""Compact summary generation for owcmp report artifacts."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

FOCUS_COLUMNS = [
    "RM",
    "Snow-Water",
    "Total-Soil",
    "SoilWaterTotal",
    "Ep",
    "Es",
    "Dp",
    "Q",
    "latqcc",
]


def _load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def _semantic_summary(path: Path, payload: dict[str, Any]) -> dict[str, Any]:
    comparison = payload["comparison"]
    column_stats = comparison.get("column_stats", [])
    focus = []
    for name in FOCUS_COLUMNS:
        stat = next((item for item in column_stats if item.get("column") == name), None)
        if stat is None:
            continue
        focus.append(
            {
                "column": name,
                "fail_count": stat.get("fail_count"),
                "compared_points": stat.get("compared_points"),
                "mean_abs_diff": stat.get("mean_abs_diff"),
                "max_abs_diff": stat.get("max_abs_diff"),
                "max_rel_diff": stat.get("max_rel_diff"),
                "max_abs_key": stat.get("max_abs_key"),
                "pass": stat.get("pass"),
            }
        )

    only_baseline = int(comparison.get("only_baseline_count", 0))
    only_candidate = int(comparison.get("only_candidate_count", 0))
    semantic_pass = bool(comparison.get("semantic_pass", False))
    structural_failures = only_baseline + only_candidate
    return {
        "source_type": "semantic_report",
        "source_path": str(path.resolve()),
        "schema": payload.get("report_schema_version"),
        "pass_count": "1/1" if semantic_pass else "0/1",
        "semantic_pass": semantic_pass,
        "structural_row_key_failures": structural_failures,
        "common_row_count": comparison.get("common_row_count"),
        "only_baseline_count": only_baseline,
        "only_candidate_count": only_candidate,
        "first_divergent_key": _first_divergent_key(comparison),
        "focus_columns": focus,
        "verdict": "PASS" if semantic_pass else "FAIL",
    }


def _first_divergent_key(comparison: dict[str, Any]) -> list[int] | None:
    top_rows = comparison.get("top_divergent_rows") or []
    for row in top_rows:
        columns = row.get("columns") or {}
        if any((details.get("abs_diff") or 0.0) > 0.0 for details in columns.values()):
            return row.get("key")

    for key_name in ("only_baseline_examples", "only_candidate_examples"):
        examples = comparison.get(key_name) or []
        if examples:
            return examples[0]
    return None


def _provenance_summary(path: Path, payload: dict[str, Any]) -> dict[str, Any]:
    outputs = payload.get("outputs", {})
    semantic = outputs.get("semantic_summary", {})
    strict_policy = payload.get("strict_lane_policy", {})
    executions = payload.get("executions", {})
    semantic_pass = bool(semantic.get("semantic_pass", False))
    structural_failures = int(semantic.get("only_baseline_count", 0)) + int(
        semantic.get("only_candidate_count", 0)
    )
    blockers = []
    blockers.extend(strict_policy.get("strict_equivalent_blockers") or [])
    blockers.extend(strict_policy.get("full_span_policy_blockers") or [])
    blockers.extend(strict_policy.get("conversion_source_row_consistency_blockers") or [])
    command_status = []
    for name, execution in executions.items():
        if execution.get("skipped"):
            status = "SKIPPED"
        elif execution.get("returncode") == 0:
            status = "PASS"
        else:
            status = "FAIL"
        command_status.append(
            {
                "name": name,
                "exit_code": execution.get("returncode"),
                "status": status,
                "command": execution.get("cmd"),
            }
        )
    failed_commands = [
        item["name"] for item in command_status if item["status"] == "FAIL"
    ]
    if failed_commands:
        blockers.extend(
            f"command failed: {name}" for name in failed_commands
        )

    return {
        "source_type": "pl14s_provenance",
        "source_path": str(path.resolve()),
        "schema": payload.get("suite_schema_version"),
        "pass_count": "1/1" if semantic_pass and not blockers else "0/1",
        "semantic_pass": semantic_pass,
        "structural_row_key_failures": structural_failures,
        "common_row_count": semantic.get("common_row_count"),
        "strict_lane_policy": strict_policy,
        "command_status": command_status,
        "raw_reports": {
            "semantic_json": outputs.get("semantic_json"),
            "strict_json": outputs.get("strict_json"),
            "baseline_stdout": outputs.get("baseline_stdout"),
            "baseline_stderr": outputs.get("baseline_stderr"),
        },
        "blockers": blockers,
        "verdict": "PASS" if semantic_pass and not blockers else "FAIL",
    }


def summarize_path(path: Path) -> dict[str, Any]:
    payload = _load_json(path)
    if payload.get("report_schema_version") == "pl14s-semantic-wat-v2":
        return _semantic_summary(path, payload)
    if payload.get("suite_schema_version") == "pl14s-legacy-suite-v2":
        return _provenance_summary(path, payload)
    raise RuntimeError(f"unsupported owcmp summary input: {path}")


def write_markdown(summary: dict[str, Any], path: Path) -> None:
    lines = [
        "# owcmp Summary",
        "",
        f"- Source: `{summary['source_path']}`",
        f"- Source type: `{summary['source_type']}`",
        f"- Schema: `{summary.get('schema')}`",
        f"- Verdict: `{summary['verdict']}`",
        f"- Pass count: `{summary['pass_count']}`",
        f"- Structural row/key failures: `{summary.get('structural_row_key_failures')}`",
        f"- Common row count: `{summary.get('common_row_count')}`",
    ]
    first_key = summary.get("first_divergent_key")
    if first_key is not None:
        lines.append(f"- First divergent key: `{first_key}`")

    focus = summary.get("focus_columns") or []
    if focus:
        lines.extend(
            [
                "",
                "## Focus Columns",
                "",
                "| Column | Fail Count | Compared Points | Mean Abs Diff | Max Abs Diff | Max Rel Diff |",
                "|---|---:|---:|---:|---:|---:|",
            ]
        )
        for item in focus:
            lines.append(
                "| {column} | {fail_count} | {compared_points} | {mean_abs_diff} | {max_abs_diff} | {max_rel_diff} |".format(
                    **item
                )
            )

    blockers = summary.get("blockers") or []
    if blockers:
        lines.extend(["", "## Blockers", ""])
        lines.extend(f"- {blocker}" for blocker in blockers)

    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output-root", type=Path, required=True)
    parser.add_argument("--summary-json", default="summary.json")
    parser.add_argument("--summary-md", default="summary.md")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    args.output_root.mkdir(parents=True, exist_ok=True)
    summary = summarize_path(args.input)
    summary_json = args.output_root / args.summary_json
    summary_md = args.output_root / args.summary_md
    summary_json.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    write_markdown(summary, summary_md)
    print(
        json.dumps(
            {
                "verdict": summary["verdict"],
                "summary_json": str(summary_json.resolve()),
                "summary_md": str(summary_md.resolve()),
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
