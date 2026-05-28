#!/usr/bin/env python3
"""Repeatable release benchmark harness for HILLBENCH01."""

from __future__ import annotations

import argparse
import json
import shutil
import statistics
import subprocess
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class Lane:
    lane_id: str
    run_dir: Path
    openwepp_run_file: str
    baseline_run_file: str
    output_dir: Path
    openwepp_expected_output: str
    baseline_expected_output: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--openwepp-binary", required=True, type=Path)
    parser.add_argument("--baseline-binary", required=True, type=Path)
    parser.add_argument("--repetitions", type=int, default=12)
    parser.add_argument("--warmups", type=int, default=2)
    parser.add_argument("--output-json", required=True, type=Path)
    return parser.parse_args()


def clean_output_dir(path: Path) -> None:
    if path.exists():
        shutil.rmtree(path)
    path.mkdir(parents=True, exist_ok=True)


def run_openwepp_once(openwepp_binary: Path, lane: Lane) -> float:
    clean_output_dir(lane.output_dir)
    cmd = [
        str(openwepp_binary),
        "--run-dir",
        str(lane.run_dir),
        "--run-file",
        lane.openwepp_run_file,
        "--output-dir",
        str(lane.output_dir),
        "--policy",
        "compat",
    ]
    start = time.perf_counter()
    completed = subprocess.run(
        cmd,
        cwd=lane.run_dir,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    elapsed = time.perf_counter() - start
    if completed.returncode != 0:
        raise RuntimeError(
            f"openwepp run failed lane={lane.lane_id} rc={completed.returncode}"
        )
    expected = lane.output_dir / lane.openwepp_expected_output
    if not expected.is_file():
        raise RuntimeError(
            f"openwepp expected output missing lane={lane.lane_id} path={expected}"
        )
    return elapsed


def run_baseline_once(baseline_binary: Path, lane: Lane) -> float:
    clean_output_dir(lane.output_dir)
    runfile_path = lane.run_dir / lane.baseline_run_file
    if not runfile_path.is_file():
        raise RuntimeError(f"baseline runfile missing lane={lane.lane_id} path={runfile_path}")
    with runfile_path.open("rb") as stdin_fh:
        start = time.perf_counter()
        completed = subprocess.run(
            [str(baseline_binary)],
            cwd=lane.run_dir,
            stdin=stdin_fh,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )
        elapsed = time.perf_counter() - start
    if completed.returncode != 0:
        raise RuntimeError(
            f"baseline run failed lane={lane.lane_id} rc={completed.returncode}"
        )
    expected = lane.output_dir / lane.baseline_expected_output
    if not expected.is_file():
        raise RuntimeError(
            f"baseline expected output missing lane={lane.lane_id} path={expected}"
        )
    return elapsed


def summarize(samples: list[float]) -> dict[str, Any]:
    if not samples:
        raise ValueError("samples must be non-empty")
    return {
        "count": len(samples),
        "min_s": min(samples),
        "max_s": max(samples),
        "mean_s": statistics.fmean(samples),
        "median_s": statistics.median(samples),
        "stdev_s": statistics.stdev(samples) if len(samples) > 1 else 0.0,
    }


def main() -> int:
    args = parse_args()
    openwepp_binary = args.openwepp_binary.resolve()
    baseline_binary = args.baseline_binary.resolve()
    output_json = args.output_json.resolve()

    if not openwepp_binary.is_file():
        raise FileNotFoundError(f"openwepp binary not found: {openwepp_binary}")
    if not baseline_binary.is_file():
        raise FileNotFoundError(f"baseline binary not found: {baseline_binary}")
    if args.repetitions < 1:
        raise ValueError("repetitions must be >= 1")
    if args.warmups < 0:
        raise ValueError("warmups must be >= 0")

    lanes = [
        Lane(
            lane_id="single_p111",
            run_dir=Path("/tmp/hillbench01/lanes/single/runs"),
            openwepp_run_file="p111_openwepp.run",
            baseline_run_file="p111.run",
            output_dir=Path("/tmp/hillbench01/lanes/single/output"),
            openwepp_expected_output="H111.wat.parquet",
            baseline_expected_output="H111.wat.dat",
        ),
        Lane(
            lane_id="multi_p324",
            run_dir=Path("/tmp/hillbench01/lanes/multi/runs"),
            openwepp_run_file="p324_openwepp.run",
            baseline_run_file="p324.run",
            output_dir=Path("/tmp/hillbench01/lanes/multi/output"),
            openwepp_expected_output="H324.wat.parquet",
            baseline_expected_output="H324.wat.dat",
        ),
    ]

    results: dict[str, Any] = {
        "schema": "hillbench01-release-benchmark-v1",
        "generated_utc": datetime.now(timezone.utc).isoformat(),
        "openwepp_binary": str(openwepp_binary),
        "baseline_binary": str(baseline_binary),
        "warmups": args.warmups,
        "repetitions": args.repetitions,
        "lanes": [],
    }

    for lane in lanes:
        if not lane.run_dir.is_dir():
            raise FileNotFoundError(f"lane run_dir missing: {lane.run_dir}")

        for _ in range(args.warmups):
            run_openwepp_once(openwepp_binary, lane)
            run_baseline_once(baseline_binary, lane)

        openwepp_samples: list[float] = []
        baseline_samples: list[float] = []
        for _ in range(args.repetitions):
            openwepp_samples.append(run_openwepp_once(openwepp_binary, lane))
            baseline_samples.append(run_baseline_once(baseline_binary, lane))

        openwepp_summary = summarize(openwepp_samples)
        baseline_summary = summarize(baseline_samples)
        ratio = openwepp_summary["median_s"] / baseline_summary["median_s"]

        results["lanes"].append(
            {
                "lane_id": lane.lane_id,
                "openwepp_samples_s": openwepp_samples,
                "baseline_samples_s": baseline_samples,
                "openwepp_summary": openwepp_summary,
                "baseline_summary": baseline_summary,
                "median_ratio_openwepp_to_baseline": ratio,
            }
        )

    output_json.parent.mkdir(parents=True, exist_ok=True)
    output_json.write_text(json.dumps(results, indent=2), encoding="utf-8")
    print(str(output_json))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
