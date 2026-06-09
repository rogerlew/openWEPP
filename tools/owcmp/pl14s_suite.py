#!/usr/bin/env python3
"""Run PL14S-oriented comparison suite with reproducible provenance.

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

STRICT_LANE_POLICY_STRICT_REQUIRED = "strict-required"
STRICT_LANE_POLICY_STRICT_EQUIVALENT_REQUIRED = "strict-equivalent-required"

CANDIDATE_SOURCE_NATIVE_RUNTIME_DAT = "native-runtime-dat"
CANDIDATE_SOURCE_CONVERSION_DERIVED_DAT = "conversion-derived-dat"
CANDIDATE_SOURCE_NATIVE_RUNTIME_PARQUET = "native-runtime-parquet"

SEMANTIC_REPORT_SCHEMA_VERSION = "pl14s-semantic-wat-v2"
BASELINE_YEAR_POLICY_PASSTHROUGH = "passthrough"
BASELINE_YEAR_POLICY_REQUIRE_EXPECTED_COMMON = "require-expected-common"

ALLOWED_CANDIDATE_SOURCE_CLASSES = {
    ".dat": {
        CANDIDATE_SOURCE_NATIVE_RUNTIME_DAT,
        CANDIDATE_SOURCE_CONVERSION_DERIVED_DAT,
    },
    ".parquet": {CANDIDATE_SOURCE_NATIVE_RUNTIME_PARQUET},
}


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
        "common_row_count",
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
        "common_row_count": comparison["common_row_count"],
        "only_baseline_count": comparison["only_baseline_count"],
        "only_candidate_count": comparison["only_candidate_count"],
        "column_stat_count": len(comparison.get("column_stats", [])),
        "top_divergent_row_count": len(comparison.get("top_divergent_rows", [])),
        "investigation_columns_used": comparison.get("investigation_columns_used", []),
        "investigation_columns_missing": comparison.get("investigation_columns_missing", []),
        "baseline_only_columns": comparison.get("baseline_only_columns", []),
    }


def parse_dat_rows_for_policy(path: Path) -> list[list[str]]:
    rows: list[list[str]] = []
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        parts = line.strip().split()
        if len(parts) not in (20, 25):
            continue
        try:
            key = [float(parts[index]) for index in (0, 1, 2)]
        except ValueError:
            continue
        if not all(value.is_integer() for value in key):
            continue
        rows.append(parts)
    return rows


def write_dat_rows(path: Path, rows: list[list[str]]) -> None:
    path.write_text("".join(" ".join(row) + "\n" for row in rows), encoding="utf-8")


def apply_baseline_year_policy(
    baseline_wat: Path,
    policy: str,
    expected_common_row_count: int | None,
    investigation_root: Path,
) -> tuple[Path, dict]:
    if policy == BASELINE_YEAR_POLICY_PASSTHROUGH:
        return baseline_wat, {
            "policy_applied": False,
            "policy_mode": policy,
            "row_count_before": None,
            "row_count_after": None,
            "replicated_years": None,
            "materialized_path": str(baseline_wat),
        }

    rows = parse_dat_rows_for_policy(baseline_wat)
    if not rows:
        raise RuntimeError(
            "baseline-year-policy requires parsed baseline rows but none were found"
        )
    if expected_common_row_count is None:
        raise RuntimeError(
            "--expected-common-row-count is required when baseline-year-policy is "
            f"{BASELINE_YEAR_POLICY_REQUIRE_EXPECTED_COMMON}"
        )
    if expected_common_row_count <= 0:
        raise RuntimeError("--expected-common-row-count must be a positive integer")
    if expected_common_row_count % len(rows) != 0:
        raise RuntimeError(
            "baseline-year-policy cannot derive integer replication factor: "
            f"expected_common_row_count={expected_common_row_count}, baseline_rows={len(rows)}"
        )

    replicated_years = expected_common_row_count // len(rows)
    if replicated_years < 1:
        raise RuntimeError(
            "baseline-year-policy derived invalid replication factor "
            f"{replicated_years} from expected_common_row_count={expected_common_row_count}"
        )

    if replicated_years == 1:
        return baseline_wat, {
            "policy_applied": False,
            "policy_mode": policy,
            "row_count_before": len(rows),
            "row_count_after": len(rows),
            "replicated_years": replicated_years,
            "materialized_path": str(baseline_wat),
        }

    expanded_rows: list[list[str]] = []
    for sim_year in range(1, replicated_years + 1):
        for source_row in rows:
            row = list(source_row)
            row[2] = str(sim_year)
            expanded_rows.append(row)

    materialized_path = investigation_root / "baseline_wat_year_policy.dat"
    write_dat_rows(materialized_path, expanded_rows)
    return materialized_path, {
        "policy_applied": True,
        "policy_mode": policy,
        "row_count_before": len(rows),
        "row_count_after": len(expanded_rows),
        "replicated_years": replicated_years,
        "materialized_path": str(materialized_path),
    }


def strict_lane_policy(candidate_format: str) -> dict:
    if candidate_format == ".dat":
        return {
            "mode": STRICT_LANE_POLICY_STRICT_REQUIRED,
            "strict_required": True,
            "strict_equivalent_lane": None,
        }
    if candidate_format == ".parquet":
        return {
            "mode": STRICT_LANE_POLICY_STRICT_EQUIVALENT_REQUIRED,
            "strict_required": False,
            "strict_equivalent_lane": "semantic",
        }
    raise SystemExit(f"unsupported candidate format for replay suite: {candidate_format}")


def validate_candidate_source_class(candidate_format: str, source_class: str) -> None:
    allowed = ALLOWED_CANDIDATE_SOURCE_CLASSES.get(candidate_format)
    if allowed is None:
        raise SystemExit(f"unsupported candidate format for source-class validation: {candidate_format}")
    if source_class not in allowed:
        allowed_list = ", ".join(sorted(allowed))
        raise SystemExit(
            "candidate source class "
            f"{source_class!r} is invalid for {candidate_format} input; "
            f"expected one of: {allowed_list}"
        )


def semantic_strict_equivalence_blockers(semantic_summary: dict) -> list[str]:
    blockers: list[str] = []
    if semantic_summary.get("report_schema_version") != SEMANTIC_REPORT_SCHEMA_VERSION:
        blockers.append(
            "semantic report schema mismatch: expected "
            f"{SEMANTIC_REPORT_SCHEMA_VERSION}, got {semantic_summary.get('report_schema_version')}"
        )
    missing_columns = semantic_summary.get("investigation_columns_missing", [])
    if missing_columns:
        blockers.append(
            "semantic investigation columns missing: " + ", ".join(sorted(missing_columns))
        )
    if semantic_summary.get("column_stat_count", 0) <= 0:
        blockers.append("semantic comparator emitted no column statistics")
    return blockers


def conversion_derived_dat_row_consistency_blockers(
    candidate_format: str,
    source_class: str,
    semantic_summary: dict,
) -> list[str]:
    if source_class != CANDIDATE_SOURCE_CONVERSION_DERIVED_DAT:
        return []

    blockers: list[str] = []
    if candidate_format != ".dat":
        blockers.append(
            "conversion-derived dat classification is valid only for .dat candidate surfaces"
        )

    common_row_count = int(semantic_summary.get("common_row_count", 0))
    only_baseline_count = int(semantic_summary.get("only_baseline_count", 0))
    only_candidate_count = int(semantic_summary.get("only_candidate_count", 0))

    if common_row_count <= 0:
        blockers.append("conversion-derived dat has no common keyed overlap with baseline")
    if only_baseline_count > 0:
        blockers.append(
            "conversion-derived dat row-count mismatch: baseline has unmatched replay rows"
        )
    if only_candidate_count > 0:
        blockers.append(
            "conversion-derived dat row-count mismatch: candidate has unmatched replay rows"
        )
    return blockers


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline-run-dir", type=Path, required=True)
    parser.add_argument("--baseline-binary", type=Path, required=True)
    parser.add_argument("--baseline-run-file", type=str, required=True)
    parser.add_argument("--candidate-wat", type=Path, required=True)
    parser.add_argument(
        "--candidate-partition-value",
        type=int,
        default=None,
        help="Optional integer partition value (for example wepp_id) used to pre-filter parquet candidate rows before semantic comparison.",
    )
    parser.add_argument(
        "--candidate-partition-column",
        type=str,
        default="wepp_id",
        help="Column name used with --candidate-partition-value when candidate input is parquet.",
    )
    parser.add_argument(
        "--candidate-year-offset",
        type=int,
        default=0,
        help="Optional integer offset applied to candidate Y-key values before semantic comparison (for example 1996 maps simulation years 1..N to calendar years 1997..).",
    )
    parser.add_argument(
        "--candidate-surface-source-class",
        type=str,
        required=True,
        choices=[
            CANDIDATE_SOURCE_NATIVE_RUNTIME_DAT,
            CANDIDATE_SOURCE_CONVERSION_DERIVED_DAT,
            CANDIDATE_SOURCE_NATIVE_RUNTIME_PARQUET,
        ],
    )
    parser.add_argument("--candidate-plot", type=Path, default=None)
    parser.add_argument("--legacy-comparator-tool", type=Path, default=Path("/workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py"))
    parser.add_argument("--output-root", type=Path, required=True)
    parser.add_argument("--strict-json", type=str, default="h5_wat_strict_comparator.json")
    parser.add_argument("--semantic-json", type=str, default="h5_wat_semantic_comparator.json")
    parser.add_argument("--tolerance-config", type=Path, default=Path("tools/owcmp/configs/pl14s_wat_tolerances.json"))
    parser.add_argument(
        "--baseline-year-policy",
        type=str,
        default=BASELINE_YEAR_POLICY_PASSTHROUGH,
        choices=[
            BASELINE_YEAR_POLICY_PASSTHROUGH,
            BASELINE_YEAR_POLICY_REQUIRE_EXPECTED_COMMON,
        ],
    )
    parser.add_argument(
        "--expected-common-row-count",
        type=int,
        default=None,
    )
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
    baseline_wat_for_compare, baseline_year_policy_materialization = (
        apply_baseline_year_policy(
            baseline_wat,
            args.baseline_year_policy,
            args.expected_common_row_count,
            investigation_root,
        )
    )
    if baseline_wat_for_compare != baseline_wat:
        shutil.copy2(baseline_wat_for_compare, baseline_wat)
        baseline_wat_for_compare = baseline_wat

    candidate_format = args.candidate_wat.suffix.lower()
    if args.candidate_partition_value is not None and candidate_format != ".parquet":
        raise SystemExit(
            "--candidate-partition-value requires parquet candidate input (.parquet)"
        )
    lane_policy = strict_lane_policy(candidate_format)
    validate_candidate_source_class(candidate_format, args.candidate_surface_source_class)
    strict_result = {
        "skipped": True,
        "reason": "strict raw comparator requires .dat input surfaces",
        "required": lane_policy["strict_required"],
        "policy_mode": lane_policy["mode"],
        "candidate_surface_source_class": args.candidate_surface_source_class,
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
            "policy_mode": lane_policy["mode"],
            "candidate_surface_source_class": args.candidate_surface_source_class,
            "strict_source_promotable_for_final_tier_a_closeout": (
                args.candidate_surface_source_class == CANDIDATE_SOURCE_NATIVE_RUNTIME_DAT
            ),
            "execution": strict_exec,
            "json_path": str(strict_json_path),
        }

    semantic_json_path = investigation_root / args.semantic_json
    semantic_script = Path(__file__).with_name("semantic_wat.py")
    semantic_cmd = [
        sys.executable,
        str(semantic_script),
        "--baseline-wat",
        str(baseline_wat_for_compare),
        "--candidate-wat",
        str(candidate_wat_for_compare),
        "--report-json",
        str(semantic_json_path),
        "--tolerance-config",
        str(args.tolerance_config),
    ]
    if args.candidate_partition_value is not None:
        semantic_cmd.extend(
            [
                "--candidate-partition-value",
                str(args.candidate_partition_value),
                "--candidate-partition-column",
                args.candidate_partition_column,
            ]
        )
    if args.candidate_year_offset != 0:
        semantic_cmd.extend(
            [
                "--candidate-year-offset",
                str(args.candidate_year_offset),
            ]
        )
    semantic_exec = run_cmd(semantic_cmd)
    if semantic_exec["returncode"] != 0:
        raise SystemExit(f"semantic comparator failed with return code {semantic_exec['returncode']}")
    semantic_summary = load_semantic_summary(semantic_json_path)
    strict_equivalence_blockers = semantic_strict_equivalence_blockers(semantic_summary)
    common_row_count = int(semantic_summary["common_row_count"])
    baseline_year_policy_blockers: list[str] = []

    expected_common_row_count = args.expected_common_row_count
    if expected_common_row_count is not None:
        if expected_common_row_count <= 0:
            raise SystemExit(
                "--expected-common-row-count must be a positive integer when provided"
            )
        if common_row_count != expected_common_row_count:
            baseline_year_policy_blockers.append(
                "common-row-count mismatch under baseline year policy: "
                f"expected {expected_common_row_count}, observed {common_row_count}"
            )

    full_span_policy_ready = not baseline_year_policy_blockers
    if (
        args.baseline_year_policy == BASELINE_YEAR_POLICY_REQUIRE_EXPECTED_COMMON
        and not full_span_policy_ready
    ):
        raise SystemExit(
            "baseline-year-policy requirements not satisfied: "
            + "; ".join(baseline_year_policy_blockers)
        )

    strict_equivalent_ready = not strict_equivalence_blockers
    if lane_policy["mode"] == STRICT_LANE_POLICY_STRICT_EQUIVALENT_REQUIRED and not strict_equivalent_ready:
        raise SystemExit(
            "strict-equivalent semantic lane requirements not satisfied: "
            + "; ".join(strict_equivalence_blockers)
        )

    conversion_source_row_consistency_blockers = (
        conversion_derived_dat_row_consistency_blockers(
            candidate_format,
            args.candidate_surface_source_class,
            semantic_summary,
        )
    )
    conversion_source_row_consistency_ready = (
        not conversion_source_row_consistency_blockers
    )
    if conversion_source_row_consistency_blockers:
        raise SystemExit(
            "conversion-derived dat row-consistency requirements not satisfied: "
            + "; ".join(conversion_source_row_consistency_blockers)
        )

    provenance = {
        "suite_schema_version": "pl14s-legacy-suite-v2",
        "baseline": {
            "binary": str(args.baseline_binary),
            "binary_sha256": sha256_file(args.baseline_binary),
            "run_dir": str(args.baseline_run_dir),
            "run_file": args.baseline_run_file,
            "source_runs_dir": str(source_runs_dir),
            "baseline_lane_root": str(baseline_lane_root),
            "baseline_wat": str(baseline_wat_for_compare),
            "baseline_wat_sha256": sha256_file(baseline_wat_for_compare),
            "baseline_year_policy_materialization": baseline_year_policy_materialization,
        },
        "candidate": {
            "input_wat": str(args.candidate_wat),
            "input_wat_format": candidate_format,
            "candidate_surface_source_class": args.candidate_surface_source_class,
            "input_wat_sha256": sha256_file(args.candidate_wat),
            "candidate_wat_for_compare": str(candidate_wat_for_compare),
            "candidate_wat_for_compare_sha256": sha256_file(candidate_wat_for_compare),
            "candidate_plot": str(args.candidate_plot) if args.candidate_plot else None,
            "candidate_partition_value": args.candidate_partition_value,
            "candidate_partition_column": args.candidate_partition_column,
            "candidate_year_offset": args.candidate_year_offset,
        },
        "strict_lane_policy": {
            "mode": lane_policy["mode"],
            "strict_required": lane_policy["strict_required"],
            "strict_equivalent_lane": lane_policy["strict_equivalent_lane"],
            "strict_equivalent_ready": strict_equivalent_ready,
            "strict_equivalent_blockers": strict_equivalence_blockers,
            "strict_source_promotable_for_final_tier_a_closeout": (
                args.candidate_surface_source_class
                != CANDIDATE_SOURCE_CONVERSION_DERIVED_DAT
            ),
            "baseline_year_policy": args.baseline_year_policy,
            "expected_common_row_count": expected_common_row_count,
            "full_span_policy_ready": full_span_policy_ready,
            "full_span_policy_blockers": baseline_year_policy_blockers,
            "conversion_source_row_consistency_ready": (
                conversion_source_row_consistency_ready
            ),
            "conversion_source_row_consistency_blockers": (
                conversion_source_row_consistency_blockers
            ),
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
