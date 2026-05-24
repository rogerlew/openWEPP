#!/usr/bin/env python3
"""Run PL14S-oriented legacy comparison suite with reproducible provenance.

This wrapper performs:
1. baseline replay (`wepp_260430_hill`) from a fixture run directory,
2. strict raw comparator invocation when candidate is `.dat`,
3. semantic WAT comparison report generation,
4. provenance and checksum capture for investigation bundles.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def find_single(path: Path, pattern: str) -> Path:
    matches = sorted(path.rglob(pattern))
    if not matches:
        raise FileNotFoundError(f"no files matched {pattern} under {path}")
    if len(matches) > 1:
        preview = ", ".join(str(item) for item in matches[:5])
        raise RuntimeError(
            f"expected exactly one match for {pattern} under {path}; found {len(matches)} ({preview})"
        )
    return matches[0]


def run_cmd(cmd: list[str], cwd: Path | None = None, stdin_path: Path | None = None, stdout_path: Path | None = None, stderr_path: Path | None = None) -> dict:
    stdin_handle = stdin_path.open("rb") if stdin_path else None
    stdout_handle = stdout_path.open("wb") if stdout_path else subprocess.PIPE
    stderr_handle = stderr_path.open("wb") if stderr_path else subprocess.PIPE
    try:
        completed = subprocess.run(
            cmd,
            cwd=str(cwd) if cwd else None,
            stdin=stdin_handle,
            stdout=stdout_handle,
            stderr=stderr_handle,
            check=False,
        )
    finally:
        if stdin_handle:
            stdin_handle.close()
        if stdout_path and stdout_handle:
            stdout_handle.close()
        if stderr_path and stderr_handle:
            stderr_handle.close()

    return {
        "cmd": cmd,
        "cwd": str(cwd) if cwd else None,
        "returncode": completed.returncode,
    }


def load_semantic_summary(path: Path) -> dict:
    payload = json.loads(path.read_text(encoding="utf-8"))
    comparison = payload.get("comparison")
    if not isinstance(comparison, dict):
        raise RuntimeError("semantic report missing comparison payload")

    required_keys = [
        "semantic_pass",
        "only_baseline_count",
        "only_candidate_count",
        "column_stats",
        "top_divergent_rows",
    ]
    missing = [name for name in required_keys if name not in comparison]
    if missing:
        raise RuntimeError(
            "semantic report missing required keys: " + ", ".join(sorted(missing))
        )

    return {
        "report_schema_version": payload.get("report_schema_version"),
        "semantic_pass": comparison["semantic_pass"],
        "only_baseline_count": comparison["only_baseline_count"],
        "only_candidate_count": comparison["only_candidate_count"],
        "column_stat_count": len(comparison.get("column_stats", [])),
        "top_divergent_row_count": len(comparison.get("top_divergent_rows", [])),
        "investigation_columns_used": comparison.get("investigation_columns_used", []),
        "investigation_columns_missing": comparison.get("investigation_columns_missing", []),
        "baseline_only_columns": comparison.get("baseline_only_columns", []),
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline-run-dir", type=Path, required=True)
    parser.add_argument("--baseline-binary", type=Path, required=True)
    parser.add_argument("--baseline-run-file", type=str, required=True)
    parser.add_argument("--candidate-wat", type=Path, required=True)
    parser.add_argument("--candidate-plot", type=Path, default=None)
    parser.add_argument("--legacy-comparator-tool", type=Path, default=Path("/workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py"))
    parser.add_argument("--output-root", type=Path, required=True)
    parser.add_argument("--strict-json", type=str, default="h5_wat_strict_comparator.json")
    parser.add_argument("--semantic-json", type=str, default="h5_wat_semantic_comparator.json")
    parser.add_argument("--tolerance-config", type=Path, default=Path("tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json"))
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    args.output_root.mkdir(parents=True, exist_ok=True)

    baseline_root = args.output_root / "baseline"
    candidate_root = args.output_root / "candidate"
    investigation_root = args.output_root / "investigation"
    baseline_root.mkdir(parents=True, exist_ok=True)
    candidate_root.mkdir(parents=True, exist_ok=True)
    investigation_root.mkdir(parents=True, exist_ok=True)

    baseline_lane_root = baseline_root / "lane"
    if baseline_lane_root.exists():
        shutil.rmtree(baseline_lane_root)
    baseline_lane_root.mkdir(parents=True, exist_ok=True)

    source_runs_dir = (
        args.baseline_run_dir / "runs"
        if (args.baseline_run_dir / "runs").is_dir()
        else args.baseline_run_dir
    )
    runs_dir = baseline_lane_root / "runs"
    shutil.copytree(source_runs_dir, runs_dir)
    (baseline_lane_root / "output").mkdir(parents=True, exist_ok=True)

    run_file = runs_dir / args.baseline_run_file
    if not run_file.exists():
        raise FileNotFoundError(f"missing baseline run file: {run_file}")

    baseline_stdout = investigation_root / "baseline_stdout.txt"
    baseline_stderr = investigation_root / "baseline_stderr.txt"
    baseline_run = run_cmd(
        [str(args.baseline_binary)],
        cwd=runs_dir,
        stdin_path=run_file,
        stdout_path=baseline_stdout,
        stderr_path=baseline_stderr,
    )
    if baseline_run["returncode"] != 0:
        raise SystemExit(f"baseline replay failed with return code {baseline_run['returncode']}")

    baseline_wat = find_single(baseline_lane_root, "H*.wat.dat")

    candidate_format = args.candidate_wat.suffix.lower()
    strict_result = {
        "skipped": True,
        "reason": "candidate is not .dat; strict raw comparator requires text surfaces",
        "required": candidate_format == ".dat",
    }
    strict_json_path = investigation_root / args.strict_json

    candidate_wat_for_compare = args.candidate_wat
    if candidate_format == ".dat":
        candidate_output_dir = candidate_root / "output"
        candidate_output_dir.mkdir(parents=True, exist_ok=True)
        staged_candidate_wat = candidate_output_dir / baseline_wat.name
        shutil.copy2(args.candidate_wat, staged_candidate_wat)
        candidate_wat_for_compare = staged_candidate_wat

        strict_cmd = [
            sys.executable,
            str(args.legacy_comparator_tool),
            "--baseline",
            str(baseline_lane_root),
            "--candidate",
            str(candidate_root),
            "--output-subdir",
            "output",
            "--include-globs",
            baseline_wat.name,
            "--abs-tol",
            "0",
            "--rel-tol",
            "0",
            "--json-out",
            str(strict_json_path),
        ]
        strict_exec = run_cmd(strict_cmd)
        if strict_exec["returncode"] != 0:
            raise SystemExit(
                f"strict comparator failed with return code {strict_exec['returncode']}"
            )
        strict_result = {
            "skipped": False,
            "required": True,
            "execution": strict_exec,
            "json_path": str(strict_json_path),
        }

    semantic_json_path = investigation_root / args.semantic_json
    semantic_script = Path(__file__).with_name("semantic_hillslope_wat_compare.py")
    semantic_cmd = [
        sys.executable,
        str(semantic_script),
        "--baseline-wat",
        str(baseline_wat),
        "--candidate-wat",
        str(candidate_wat_for_compare),
        "--report-json",
        str(semantic_json_path),
        "--tolerance-config",
        str(args.tolerance_config),
    ]
    semantic_exec = run_cmd(semantic_cmd)
    if semantic_exec["returncode"] != 0:
        raise SystemExit(f"semantic comparator failed with return code {semantic_exec['returncode']}")
    semantic_summary = load_semantic_summary(semantic_json_path)

    provenance = {
        "suite_schema_version": "pl14s-legacy-suite-v1",
        "baseline": {
            "binary": str(args.baseline_binary),
            "binary_sha256": sha256_file(args.baseline_binary),
            "run_dir": str(args.baseline_run_dir),
            "run_file": args.baseline_run_file,
            "source_runs_dir": str(source_runs_dir),
            "baseline_lane_root": str(baseline_lane_root),
            "baseline_wat": str(baseline_wat),
            "baseline_wat_sha256": sha256_file(baseline_wat),
        },
        "candidate": {
            "input_wat": str(args.candidate_wat),
            "input_wat_format": candidate_format,
            "input_wat_sha256": sha256_file(args.candidate_wat),
            "candidate_wat_for_compare": str(candidate_wat_for_compare),
            "candidate_wat_for_compare_sha256": sha256_file(candidate_wat_for_compare),
            "candidate_plot": str(args.candidate_plot) if args.candidate_plot else None,
        },
        "tooling": {
            "legacy_comparator_tool": str(args.legacy_comparator_tool),
            "legacy_comparator_tool_sha256": sha256_file(args.legacy_comparator_tool) if args.legacy_comparator_tool.exists() else None,
            "semantic_script": str(semantic_script),
            "semantic_script_sha256": sha256_file(semantic_script),
            "tolerance_config": str(args.tolerance_config),
            "tolerance_config_sha256": sha256_file(args.tolerance_config) if args.tolerance_config.exists() else None,
        },
        "executions": {
            "baseline_replay": baseline_run,
            "strict_compare": strict_result,
            "semantic_compare": semantic_exec,
        },
        "outputs": {
            "semantic_json": str(semantic_json_path),
            "semantic_json_sha256": sha256_file(semantic_json_path),
            "semantic_summary": semantic_summary,
            "strict_json": str(strict_json_path) if strict_json_path.exists() else None,
            "strict_json_sha256": sha256_file(strict_json_path) if strict_json_path.exists() else None,
            "baseline_stdout": str(baseline_stdout),
            "baseline_stderr": str(baseline_stderr),
        },
    }

    provenance_path = investigation_root / "pl14s_provenance_manifest.json"
    provenance_path.write_text(json.dumps(provenance, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
