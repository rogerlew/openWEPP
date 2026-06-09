#!/usr/bin/env python3
"""Run a compact H1-H39 semantic WAT comparison batch."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
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


def _format_pattern(pattern: str, hillslope_id: int) -> str:
    return pattern.format(h=hillslope_id, H=f"H{hillslope_id}")


def _load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def _run_command(cmd: list[str], stdout_path: Path, stderr_path: Path) -> int:
    stdout_path.parent.mkdir(parents=True, exist_ok=True)
    stderr_path.parent.mkdir(parents=True, exist_ok=True)
    with stdout_path.open("w", encoding="utf-8") as stdout_handle, stderr_path.open(
        "w", encoding="utf-8"
    ) as stderr_handle:
        completed = subprocess.run(
            cmd,
            stdout=stdout_handle,
            stderr=stderr_handle,
            check=False,
        )
    return completed.returncode


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


def _aggregate_report(
    hillslope_reports: list[tuple[int, Path]],
    command_status: list[dict[str, Any]],
    output_root: Path,
) -> dict[str, Any]:
    pass_hillslopes: list[int] = []
    failed_hillslopes: list[int] = []
    structural_failures = 0
    focus_accumulator: dict[str, dict[str, Any]] = {
        column: {
            "column": column,
            "hillslope_fail_count": 0,
            "total_fail_count": 0,
            "mean_abs_diff_sum": 0.0,
            "mean_abs_diff_count": 0,
            "max_abs_diff": 0.0,
            "max_rel_diff": 0.0,
            "max_abs_key": None,
            "max_abs_hillslope": None,
        }
        for column in FOCUS_COLUMNS
    }
    first_divergent: dict[str, Any] | None = None

    for hillslope_id, report_path in hillslope_reports:
        payload = _load_json(report_path)
        comparison = payload["comparison"]
        semantic_pass = bool(comparison.get("semantic_pass", False))
        if semantic_pass:
            pass_hillslopes.append(hillslope_id)
        else:
            failed_hillslopes.append(hillslope_id)

        structural_failures += int(comparison.get("only_baseline_count", 0))
        structural_failures += int(comparison.get("only_candidate_count", 0))

        if first_divergent is None:
            key = _first_divergent_key(comparison)
            if key is not None:
                first_divergent = {"hillslope": hillslope_id, "key": key}

        for stat in comparison.get("column_stats", []):
            column = stat.get("column")
            if column not in focus_accumulator:
                continue
            focus = focus_accumulator[column]
            fail_count = int(stat.get("fail_count") or 0)
            mean_abs_diff = float(stat.get("mean_abs_diff") or 0.0)
            max_abs_diff = float(stat.get("max_abs_diff") or 0.0)
            max_rel_diff = float(stat.get("max_rel_diff") or 0.0)
            focus["total_fail_count"] += fail_count
            focus["mean_abs_diff_sum"] += mean_abs_diff
            focus["mean_abs_diff_count"] += 1
            focus["max_rel_diff"] = max(float(focus["max_rel_diff"]), max_rel_diff)
            if fail_count > 0:
                focus["hillslope_fail_count"] += 1
            if max_abs_diff > float(focus["max_abs_diff"]):
                focus["max_abs_diff"] = max_abs_diff
                focus["max_abs_key"] = stat.get("max_abs_key")
                focus["max_abs_hillslope"] = hillslope_id

    focus_columns = []
    for column in FOCUS_COLUMNS:
        focus = focus_accumulator[column]
        count = int(focus.pop("mean_abs_diff_count"))
        mean_sum = float(focus.pop("mean_abs_diff_sum"))
        focus["mean_abs_diff_mean"] = mean_sum / count if count else 0.0
        focus_columns.append(focus)

    failed_commands = [item for item in command_status if item["exit_code"] != 0]
    total = len(hillslope_reports)
    execution_verdict = "PASS" if not failed_commands else "FAIL"
    semantic_verdict = "PASS" if len(pass_hillslopes) == total and total else "FAIL"

    return {
        "source_type": "h1_h39_semantic_batch",
        "output_root": str(output_root.resolve()),
        "execution_verdict": execution_verdict,
        "semantic_verdict": semantic_verdict,
        "semantic_pass_count": f"{len(pass_hillslopes)}/{total}",
        "pass_hillslopes": pass_hillslopes,
        "failed_hillslopes": failed_hillslopes,
        "structural_row_key_failures": structural_failures,
        "first_divergent": first_divergent,
        "focus_columns": focus_columns,
        "command_status": command_status,
        "raw_reports": [str(path.resolve()) for _, path in hillslope_reports],
        "logs": str((output_root / "logs").resolve()),
    }


def _write_markdown(summary: dict[str, Any], path: Path) -> None:
    lines = [
        "# owcmp H1-H39 Semantic Batch",
        "",
        f"- Source type: `{summary['source_type']}`",
        f"- Execution verdict: `{summary['execution_verdict']}`",
        f"- Semantic verdict: `{summary['semantic_verdict']}`",
        f"- Semantic pass count: `{summary['semantic_pass_count']}`",
        f"- Structural row/key failures: `{summary['structural_row_key_failures']}`",
        f"- Raw report count: `{len(summary['raw_reports'])}`",
        f"- Logs: `{summary['logs']}`",
    ]
    if summary.get("first_divergent") is not None:
        lines.append(f"- First divergent: `{summary['first_divergent']}`")

    lines.extend(
        [
            "",
            "## Focus Columns",
            "",
            "| Column | Hillslope Fails | Total Fails | Mean Abs Diff Mean | Max Abs Diff | Max Rel Diff | Max Abs Hillslope | Max Abs Key |",
            "|---|---:|---:|---:|---:|---:|---:|---|",
        ]
    )
    for item in summary["focus_columns"]:
        lines.append(
            "| {column} | {hillslope_fail_count} | {total_fail_count} | {mean_abs_diff_mean} | {max_abs_diff} | {max_rel_diff} | {max_abs_hillslope} | {max_abs_key} |".format(
                **item
            )
        )

    failed_commands = [item for item in summary["command_status"] if item["exit_code"] != 0]
    if failed_commands:
        lines.extend(["", "## Failed Commands", ""])
        for item in failed_commands:
            lines.append(
                f"- H{item['hillslope']}: exit `{item['exit_code']}` stdout `{item['stdout']}` stderr `{item['stderr']}`"
            )

    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def _write_failure_markdown(summary: dict[str, Any], path: Path) -> None:
    lines = [
        "# owcmp H1-H39 Semantic Batch",
        "",
        f"- Source type: `{summary['source_type']}`",
        f"- Execution verdict: `{summary['execution_verdict']}`",
        f"- Semantic verdict: `{summary['semantic_verdict']}`",
    ]
    missing_inputs = summary.get("missing_inputs") or []
    if missing_inputs:
        lines.extend(["", "## Missing Inputs", ""])
        lines.extend(f"- `{item}`" for item in missing_inputs)

    command_status = summary.get("command_status") or []
    if command_status:
        lines.extend(["", "## Command Status", ""])
        for item in command_status:
            lines.append(
                f"- H{item['hillslope']}: exit `{item['exit_code']}` stdout `{item['stdout']}` stderr `{item['stderr']}`"
            )

    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def _missing_inputs(args: argparse.Namespace) -> list[Path]:
    missing: list[Path] = []
    for hillslope_id in range(args.start, args.end + 1):
        baseline = args.baseline_dir / _format_pattern(args.baseline_pattern, hillslope_id)
        candidate = args.candidate_dir / _format_pattern(args.candidate_pattern, hillslope_id)
        if not baseline.is_file():
            missing.append(baseline)
        if not candidate.is_file():
            missing.append(candidate)
    return missing


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline-dir", type=Path, required=True)
    parser.add_argument("--candidate-dir", type=Path, required=True)
    parser.add_argument("--output-root", type=Path, required=True)
    parser.add_argument("--tolerance-config", type=Path, default=Path("tools/owcmp/configs/pl14s_wat_tolerances.json"))
    parser.add_argument("--candidate-year-offset", type=int, default=2012)
    parser.add_argument("--candidate-partition-column", default="wepp_id")
    parser.add_argument("--start", type=int, default=1)
    parser.add_argument("--end", type=int, default=39)
    parser.add_argument("--baseline-pattern", default="baseline_H{h}.parquet")
    parser.add_argument("--candidate-pattern", default="H{h}.wat.parquet")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    if args.start < 1 or args.end < args.start:
        raise SystemExit("--start and --end must define a positive inclusive range")

    args.output_root.mkdir(parents=True, exist_ok=True)
    logs_dir = args.output_root / "logs"
    report_dir = args.output_root / "reports" / "semantic"
    report_dir.mkdir(parents=True, exist_ok=True)
    command_log_path = args.output_root / "command-log.json"
    summary_json = args.output_root / "summary.json"
    summary_md = args.output_root / "summary.md"

    missing = _missing_inputs(args)
    if missing:
        command_log_path.write_text("[]\n", encoding="utf-8")
        failure = {
            "source_type": "h1_h39_semantic_batch",
            "output_root": str(args.output_root.resolve()),
            "execution_verdict": "FAIL",
            "semantic_verdict": "NOT_RUN",
            "semantic_pass_count": f"0/{args.end - args.start + 1}",
            "pass_hillslopes": [],
            "failed_hillslopes": [],
            "structural_row_key_failures": "NOT_RUN",
            "first_divergent": None,
            "focus_columns": [],
            "missing_inputs": [str(path.resolve()) for path in missing],
            "command_status": [],
            "raw_reports": [],
            "logs": str(logs_dir.resolve()),
            "summary_json": str(summary_json.resolve()),
            "summary_md": str(summary_md.resolve()),
            "command_log": str(command_log_path.resolve()),
        }
        summary_json.write_text(json.dumps(failure, indent=2) + "\n", encoding="utf-8")
        _write_failure_markdown(failure, summary_md)
        raise SystemExit(f"missing batch inputs; see {summary_json.resolve()}")

    semantic_script = Path(__file__).resolve().with_name("semantic_wat.py")
    command_status: list[dict[str, Any]] = []
    hillslope_reports: list[tuple[int, Path]] = []

    for hillslope_id in range(args.start, args.end + 1):
        baseline = args.baseline_dir / _format_pattern(args.baseline_pattern, hillslope_id)
        candidate = args.candidate_dir / _format_pattern(args.candidate_pattern, hillslope_id)
        report = report_dir / f"H{hillslope_id}.semantic.json"
        stdout_path = logs_dir / f"H{hillslope_id}.stdout.txt"
        stderr_path = logs_dir / f"H{hillslope_id}.stderr.txt"
        cmd = [
            sys.executable,
            str(semantic_script),
            "--baseline-wat",
            str(baseline),
            "--candidate-wat",
            str(candidate),
            "--candidate-year-offset",
            str(args.candidate_year_offset),
            "--candidate-partition-column",
            args.candidate_partition_column,
            "--tolerance-config",
            str(args.tolerance_config),
            "--report-json",
            str(report),
        ]
        exit_code = _run_command(cmd, stdout_path, stderr_path)
        command_status.append(
            {
                "hillslope": hillslope_id,
                "command": cmd,
                "exit_code": exit_code,
                "stdout": str(stdout_path.resolve()),
                "stderr": str(stderr_path.resolve()),
                "report": str(report.resolve()),
            }
        )
        if exit_code != 0:
            command_log_path.write_text(
                json.dumps(command_status, indent=2) + "\n",
                encoding="utf-8",
            )
            failure = {
                "source_type": "h1_h39_semantic_batch",
                "output_root": str(args.output_root.resolve()),
                "execution_verdict": "FAIL",
                "semantic_verdict": "NOT_RUN",
                "semantic_pass_count": f"{len(hillslope_reports)}/{args.end - args.start + 1}",
                "pass_hillslopes": [],
                "failed_hillslopes": [hillslope_id],
                "structural_row_key_failures": "NOT_RUN",
                "first_divergent": None,
                "focus_columns": [],
                "failed_hillslope": hillslope_id,
                "command_status": command_status,
                "raw_reports": [str(path.resolve()) for _, path in hillslope_reports],
                "logs": str(logs_dir.resolve()),
                "summary_json": str(summary_json.resolve()),
                "summary_md": str(summary_md.resolve()),
                "command_log": str(command_log_path.resolve()),
            }
            summary_json.write_text(json.dumps(failure, indent=2) + "\n", encoding="utf-8")
            _write_failure_markdown(failure, summary_md)
            raise SystemExit(f"H{hillslope_id} semantic comparison failed; see {stderr_path.resolve()}")
        hillslope_reports.append((hillslope_id, report))

    command_log_path.write_text(json.dumps(command_status, indent=2) + "\n", encoding="utf-8")
    summary = _aggregate_report(hillslope_reports, command_status, args.output_root)
    summary["summary_json"] = str(summary_json.resolve())
    summary["summary_md"] = str(summary_md.resolve())
    summary["command_log"] = str(command_log_path.resolve())
    summary_json.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    _write_markdown(summary, summary_md)
    print(
        json.dumps(
            {
                "execution_verdict": summary["execution_verdict"],
                "semantic_pass_count": summary["semantic_pass_count"],
                "summary_json": summary["summary_json"],
                "summary_md": summary["summary_md"],
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
