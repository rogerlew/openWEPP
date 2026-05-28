#!/usr/bin/env python3
"""Broad hillslope stability harness for HILLSTAB01."""

from __future__ import annotations

import argparse
import csv
import json
import os
import re
import shutil
import statistics
import subprocess
import time
from concurrent.futures import ProcessPoolExecutor
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

PROJECT_PREFIX = {
    "cochlear-beriberi": "co",
    "moth-eaten-blackhead": "mo",
    "ordained-incentive": "or",
    "uninsured-deformation": "un",
}

RUN_FILE_RE = re.compile(r"^p(?P<wepp_id>\d+)\.run$")
ERROR_SIGNATURE_RE = re.compile(
    r"CLIHILL-E-|ParseFailure|RunFile|Io|Fortran runtime error|ERROR STOP|SIGFPE",
    re.IGNORECASE,
)


@dataclass(frozen=True)
class CaseSpec:
    suite: str
    case_id: str
    wepp_id: int
    source_run_dir: Path
    source_run_file: str


@dataclass(frozen=True)
class CaseResult:
    suite: str
    case_id: str
    wepp_id: int
    source_run_dir: str
    returncode: int
    timed_out: bool
    elapsed_s: float
    required_outputs_present: bool
    manifest_present: bool
    stderr_error_signature: str
    pass_status: bool
    work_dir: str
    stdout_log: str
    stderr_log: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--openwepp-binary", type=Path, required=True)
    parser.add_argument("--cohort-seeds-csv", type=Path, required=True)
    parser.add_argument("--watchlist-csv", type=Path, required=True)
    parser.add_argument("--wepp-forest-root", type=Path, default=Path("/workdir/wepp-forest"))
    parser.add_argument("--wc1-root", type=Path, default=Path("/wc1/runs"))
    parser.add_argument("--scratch-root", type=Path, default=Path("/tmp/hillstab01"))
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--jobs", type=int, default=max(1, min(8, os.cpu_count() or 1)))
    parser.add_argument("--timeout-seconds", type=float, default=180.0)
    parser.add_argument("--limit-1166", type=int)
    parser.add_argument("--limit-watchlist", type=int)
    parser.add_argument("--keep-passing-workdirs", action="store_true")
    return parser.parse_args()


def parse_1166_cases(seeds_csv: Path, wc1_root: Path, limit: int | None) -> list[CaseSpec]:
    cases: list[CaseSpec] = []
    with seeds_csv.open(newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        required = {"seed_id", "project", "wepp_id"}
        if reader.fieldnames is None or required.difference(reader.fieldnames):
            raise ValueError(f"invalid seeds csv header: {seeds_csv}")

        for row in reader:
            project = row["project"].strip()
            if project not in PROJECT_PREFIX:
                raise ValueError(f"unknown project '{project}' in {seeds_csv}")
            wepp_id = int(row["wepp_id"])
            run_dir = wc1_root / PROJECT_PREFIX[project] / project / "wepp" / "runs"
            cases.append(
                CaseSpec(
                    suite="wb05b_1166",
                    case_id=row["seed_id"].strip(),
                    wepp_id=wepp_id,
                    source_run_dir=run_dir,
                    source_run_file=f"p{wepp_id}.run",
                )
            )
            if limit is not None and len(cases) >= limit:
                break
    if not cases:
        raise ValueError(f"no 1166 cohort cases parsed from {seeds_csv}")
    return cases


def parse_watchlist_cases(
    watchlist_csv: Path,
    wepp_forest_root: Path,
    limit: int | None,
) -> list[CaseSpec]:
    cases: list[CaseSpec] = []
    with watchlist_csv.open(newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        required = {"seed_id", "run_file"}
        if reader.fieldnames is None or required.difference(reader.fieldnames):
            raise ValueError(f"invalid watchlist csv header: {watchlist_csv}")

        for row in reader:
            seed_id = row["seed_id"].strip()
            run_file_raw = row["run_file"].strip()
            if not seed_id or not run_file_raw:
                continue
            run_path = Path(run_file_raw)
            if not run_path.is_absolute():
                run_path = (wepp_forest_root / run_path).resolve()
            run_file = run_path.name
            match = RUN_FILE_RE.match(run_file)
            if match is None:
                raise ValueError(f"unexpected run file name '{run_file}' in watchlist")
            cases.append(
                CaseSpec(
                    suite="release_gate_watchlist",
                    case_id=seed_id,
                    wepp_id=int(match.group("wepp_id")),
                    source_run_dir=run_path.parent,
                    source_run_file=run_file,
                )
            )
            if limit is not None and len(cases) >= limit:
                break
    if not cases:
        raise ValueError(f"no watchlist cases parsed from {watchlist_csv}")
    return cases


def build_runfile_text(wepp_id: int) -> str:
    return (
        'schema = "openwepp-hillslope-runfile-v1"\n'
        f'run_name = "hillstab01_h{wepp_id}"\n'
        'unit_system = "metric"\n\n'
        "[inputs]\n"
        f'soil = "p{wepp_id}.sol"\n'
        f'management = "p{wepp_id}.man"\n'
        f'slope = "p{wepp_id}.slp"\n'
        f'climate = "p{wepp_id}.cli"\n\n'
        "[outputs]\n"
        f'pass = "../output/H{wepp_id}.hbp"\n'
        f'loss = "../output/H{wepp_id}.loss.json"\n'
        f'wat = "../output/H{wepp_id}.wat.parquet"\n'
        f'plot = "../output/H{wepp_id}.plot.parquet"\n'
    )


def symlink_required_inputs(case: CaseSpec, staged_runs: Path) -> None:
    run_stem = f"p{case.wepp_id}"
    required = [f"{run_stem}.cli", f"{run_stem}.man", f"{run_stem}.slp", f"{run_stem}.sol"]
    optional = ["pmetpara.txt", "wepp_ui.txt", "snow.txt", "frost.txt"]
    for name in required + optional:
        source = case.source_run_dir / name
        target = staged_runs / name
        if source.exists():
            os.symlink(source, target)
        elif name in required:
            raise FileNotFoundError(f"missing required source input: {source}")


def detect_error_signature(stderr_text: str, stdout_text: str) -> str:
    for text in (stderr_text, stdout_text):
        match = ERROR_SIGNATURE_RE.search(text)
        if match:
            return match.group(0)
    return ""


def run_case(
    case: CaseSpec,
    binary: Path,
    scratch_root: Path,
    timeout_seconds: float,
    keep_passing_workdirs: bool,
) -> CaseResult:
    case_root = scratch_root / case.suite / case.case_id
    if case_root.exists():
        shutil.rmtree(case_root)
    staged_runs = case_root / "runs"
    output_dir = case_root / "output"
    logs_dir = case_root / "logs"
    staged_runs.mkdir(parents=True, exist_ok=True)
    output_dir.mkdir(parents=True, exist_ok=True)
    logs_dir.mkdir(parents=True, exist_ok=True)

    symlink_required_inputs(case, staged_runs)
    run_file_name = f"p{case.wepp_id}_openwepp.run"
    runfile_path = staged_runs / run_file_name
    runfile_path.write_text(build_runfile_text(case.wepp_id), encoding="utf-8")

    stdout_log = logs_dir / "stdout.log"
    stderr_log = logs_dir / "stderr.log"
    cmd = [
        str(binary),
        "--run-dir",
        str(staged_runs),
        "--run-file",
        run_file_name,
        "--output-dir",
        str(output_dir),
        "--policy",
        "compat",
        "--legacy-sidecar-discovery",
    ]

    started = time.perf_counter()
    timed_out = False
    returncode = 0
    stdout_text = ""
    stderr_text = ""
    try:
        completed = subprocess.run(
            cmd,
            cwd=staged_runs,
            capture_output=True,
            text=True,
            timeout=timeout_seconds,
            check=False,
        )
        returncode = int(completed.returncode)
        stdout_text = completed.stdout
        stderr_text = completed.stderr
    except subprocess.TimeoutExpired as exc:
        timed_out = True
        returncode = 124
        stdout_text = exc.stdout.decode(errors="replace") if isinstance(exc.stdout, bytes) else (exc.stdout or "")
        stderr_text = exc.stderr.decode(errors="replace") if isinstance(exc.stderr, bytes) else (exc.stderr or "")
    elapsed_s = time.perf_counter() - started

    stdout_log.write_text(stdout_text, encoding="utf-8")
    stderr_log.write_text(stderr_text, encoding="utf-8")

    required_outputs = [
        output_dir / f"H{case.wepp_id}.hbp",
        output_dir / f"H{case.wepp_id}.loss.json",
        output_dir / f"H{case.wepp_id}.wat.parquet",
        output_dir / f"H{case.wepp_id}.plot.parquet",
    ]
    required_outputs_present = all(path.is_file() for path in required_outputs)
    manifest_present = (output_dir / "openwepp_hillslope_run_manifest.json").is_file()
    signature = detect_error_signature(stderr_text, stdout_text)
    pass_status = (
        returncode == 0 and not timed_out and required_outputs_present and manifest_present
    )

    if pass_status and not keep_passing_workdirs:
        # Preserve logs in parent summary location only for failed cases.
        shutil.rmtree(case_root)
        work_dir = ""
        stdout_path = ""
        stderr_path = ""
    else:
        work_dir = str(case_root)
        stdout_path = str(stdout_log)
        stderr_path = str(stderr_log)

    return CaseResult(
        suite=case.suite,
        case_id=case.case_id,
        wepp_id=case.wepp_id,
        source_run_dir=str(case.source_run_dir),
        returncode=returncode,
        timed_out=timed_out,
        elapsed_s=elapsed_s,
        required_outputs_present=required_outputs_present,
        manifest_present=manifest_present,
        stderr_error_signature=signature,
        pass_status=pass_status,
        work_dir=work_dir,
        stdout_log=stdout_path,
        stderr_log=stderr_path,
    )


def summarize_suite(results: list[CaseResult]) -> dict[str, Any]:
    total = len(results)
    passed = sum(1 for item in results if item.pass_status)
    failed = total - passed
    timeout_count = sum(1 for item in results if item.timed_out)
    elapsed = [item.elapsed_s for item in results]
    failure_signatures: dict[str, int] = {}
    for item in results:
        if item.pass_status:
            continue
        key = item.stderr_error_signature or "no-signature"
        failure_signatures[key] = failure_signatures.get(key, 0) + 1

    summary: dict[str, Any] = {
        "total": total,
        "passed": passed,
        "failed": failed,
        "timeout_count": timeout_count,
        "pass_rate": (passed / total) if total else 0.0,
        "elapsed_min_s": min(elapsed) if elapsed else 0.0,
        "elapsed_max_s": max(elapsed) if elapsed else 0.0,
        "elapsed_mean_s": statistics.fmean(elapsed) if elapsed else 0.0,
        "elapsed_median_s": statistics.median(elapsed) if elapsed else 0.0,
        "failure_signatures": dict(sorted(failure_signatures.items(), key=lambda kv: (-kv[1], kv[0]))),
    }
    return summary


def execute_cases(
    cases: list[CaseSpec],
    binary: Path,
    scratch_root: Path,
    jobs: int,
    timeout_seconds: float,
    keep_passing_workdirs: bool,
) -> list[CaseResult]:
    args = [
        (case, binary, scratch_root, timeout_seconds, keep_passing_workdirs) for case in cases
    ]
    if jobs <= 1:
        return [run_case(*item) for item in args]
    with ProcessPoolExecutor(max_workers=jobs) as executor:
        return list(executor.map(run_case_from_tuple, args))


def run_case_from_tuple(item: tuple[CaseSpec, Path, Path, float, bool]) -> CaseResult:
    return run_case(*item)


def main() -> int:
    args = parse_args()
    binary = args.openwepp_binary.resolve()
    if not binary.is_file():
        raise FileNotFoundError(f"openwepp binary not found: {binary}")
    if args.jobs < 1:
        raise ValueError("jobs must be >= 1")

    seeds_csv = args.cohort_seeds_csv.resolve()
    watchlist_csv = args.watchlist_csv.resolve()
    wepp_forest_root = args.wepp_forest_root.resolve()
    scratch_root = args.scratch_root.resolve()
    output_json = args.output_json.resolve()

    cohort_cases = parse_1166_cases(seeds_csv, args.wc1_root.resolve(), args.limit_1166)
    watchlist_cases = parse_watchlist_cases(watchlist_csv, wepp_forest_root, args.limit_watchlist)
    all_cases = cohort_cases + watchlist_cases

    if scratch_root.exists():
        shutil.rmtree(scratch_root)
    scratch_root.mkdir(parents=True, exist_ok=True)

    started_utc = datetime.now(timezone.utc).isoformat()
    results = execute_cases(
        all_cases,
        binary,
        scratch_root,
        args.jobs,
        args.timeout_seconds,
        args.keep_passing_workdirs,
    )
    ended_utc = datetime.now(timezone.utc).isoformat()

    by_suite: dict[str, list[CaseResult]] = {}
    for item in results:
        by_suite.setdefault(item.suite, []).append(item)
    suite_summaries = {suite: summarize_suite(rows) for suite, rows in sorted(by_suite.items())}

    payload: dict[str, Any] = {
        "schema": "hillstab01-results-v1",
        "status": "complete",
        "generated_utc": ended_utc,
        "started_utc": started_utc,
        "openwepp_binary": str(binary),
        "cohort_seeds_csv": str(seeds_csv),
        "watchlist_csv": str(watchlist_csv),
        "scratch_root": str(scratch_root),
        "jobs": args.jobs,
        "timeout_seconds": args.timeout_seconds,
        "limit_1166": args.limit_1166,
        "limit_watchlist": args.limit_watchlist,
        "suite_summaries": suite_summaries,
        "results": [asdict(item) for item in results],
    }

    output_json.parent.mkdir(parents=True, exist_ok=True)
    output_json.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    print(str(output_json))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
