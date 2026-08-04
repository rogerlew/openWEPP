#!/usr/bin/env python3
"""Run the frozen Snowbird ledger-persistence compatibility protocol."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import sys
import time
from typing import Any


REPO = Path(__file__).resolve().parents[4]
OUTPUT_ROOT = REPO / "target/snow_mass_transition_ledger_persistence"
SOURCE_FIXTURE = OUTPUT_ROOT / "fixtures/snotel_snowbird_ut"
SITE = "snotel_snowbird_ut"
TOLERANCE_M = 1.0e-9
REPETITIONS = 7


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def parse_peak_rss_kib(path: Path) -> int:
    prefix = "Maximum resident set size (kbytes):"
    for line in path.read_text().splitlines():
        if line.strip().startswith(prefix):
            return int(line.split(":", 1)[1].strip())
    raise RuntimeError(f"GNU time output omits peak RSS: {path}")


def run_paths(build: str, label: str) -> dict[str, Path]:
    run_dir = OUTPUT_ROOT / "runs" / build / label / SITE
    base = run_dir / f"{SITE}-{build}-{label}"
    return {
        "run_dir": run_dir,
        "fixture": OUTPUT_ROOT / "run-fixtures" / build / label / SITE,
        "runfile": base.with_suffix(".run"),
        "trace": base.with_suffix(".snow.jsonl"),
        "wat": base.with_suffix(".wat.parquet"),
        "hbp": base.with_suffix(".hbp"),
        "loss": base.with_suffix(".loss.json"),
        "stdout": base.with_suffix(".stdout.txt"),
        "stderr": base.with_suffix(".stderr.txt"),
        "time": base.with_suffix(".time.txt"),
        "receipt": base.with_suffix(".receipt.json"),
    }


def run_once(
    build: str,
    label: str,
    binary: Path,
    trace_enabled: bool,
    trace_selector_case: str = "ordinary",
) -> dict[str, Any]:
    paths = run_paths(build, label)
    if paths["run_dir"].exists() or paths["fixture"].exists():
        raise RuntimeError(f"refusing to overwrite {build}/{label}")
    if not SOURCE_FIXTURE.is_dir():
        raise RuntimeError(f"missing frozen fixture: {SOURCE_FIXTURE}")
    binary = binary.resolve()
    if not binary.is_file():
        raise RuntimeError(f"missing binary: {binary}")

    paths["run_dir"].mkdir(parents=True)
    paths["fixture"].parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(SOURCE_FIXTURE, paths["fixture"])
    fixture = paths["fixture"]
    paths["runfile"].write_text(
        'schema = "openwepp-hillslope-runfile-v1"\n'
        f'run_name = "snow-ledger-persistence-{build}-{label}"\n'
        'unit_system = "metric"\n\n'
        "[inputs]\n"
        f'soil = "{fixture / "p8.sol"}"\n'
        f'management = "{fixture / "p8.man"}"\n'
        f'slope = "{fixture / "p8.slp"}"\n'
        f'climate = "{fixture / "p8.cli"}"\n'
        "wepp_ui = false\n\n"
        "[outputs]\n"
        f'pass = "{paths["hbp"]}"\n'
        f'loss = "{paths["loss"]}"\n'
        f'wat = "{paths["wat"]}"\n'
    )

    effective = {
        "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL": "layered_thermal_liquid_v1",
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL": "physics_bulk_multilayer_density_v1",
        "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL": "harder_pomeroy_hourly",
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL": "coe_liquid_holding_capacity_v1",
        "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": "disabled",
        "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL": "disabled",
    }
    if trace_enabled:
        effective["OPENWEPP_R7H_SNOW_TRACE_PATH"] = str(paths["trace"])
    elif trace_selector_case == "empty":
        effective["OPENWEPP_R7H_SNOW_TRACE_PATH"] = ""
    elif trace_selector_case == "filtered":
        effective["OPENWEPP_R7H_SNOW_TRACE_PATH"] = str(paths["trace"])
        effective["OPENWEPP_R7H_SNOW_TRACE_DAY_INDEX"] = str(2**31 - 1)
    env = {key: value for key, value in os.environ.items() if not key.startswith("OPENWEPP_")}
    env.update(effective)
    cli = [
        str(binary),
        "--run-dir",
        str(fixture),
        "--run-file",
        str(paths["runfile"]),
        "--output-dir",
        str(paths["run_dir"]),
        "--legacy-sidecar-discovery",
        "--direct-production-executor",
    ]
    argv = ["/usr/bin/time", "-v", "-o", str(paths["time"]), *cli]
    started = time.perf_counter()
    completed = subprocess.run(argv, env=env, capture_output=True, check=False)
    wall_seconds = time.perf_counter() - started
    paths["stdout"].write_bytes(completed.stdout)
    paths["stderr"].write_bytes(completed.stderr)
    receipt: dict[str, Any] = {
        "argv": argv,
        "binary": str(binary),
        "binary_sha256": sha256(binary),
        "build": build,
        "effective_openwepp_environment": effective,
        "label": label,
        "peak_rss_kib": parse_peak_rss_kib(paths["time"]),
        "returncode": completed.returncode,
        "trace_enabled": trace_enabled,
        "trace_selector_case": trace_selector_case,
        "wall_seconds": wall_seconds,
    }
    for name in ("trace", "wat", "hbp", "loss"):
        path = paths[name]
        if path.is_file():
            receipt[name] = {
                "path": str(path.relative_to(REPO)),
                "sha256": sha256(path),
                "size_bytes": path.stat().st_size,
            }
    write_json(paths["receipt"], receipt)
    if completed.returncode != 0:
        raise RuntimeError(f"release CLI failed; see {paths['stderr']}")
    return receipt


def run_suite(build: str, binary: Path) -> None:
    receipts = [run_once(build, "trace", binary, True)]
    receipts.append(run_once(build, "warmup", binary, False))
    receipts.append(run_once(build, "empty-path", binary, False, "empty"))
    receipts.append(run_once(build, "filtered-out", binary, False, "filtered"))
    for index in range(1, REPETITIONS + 1):
        receipts.append(run_once(build, f"measure-{index:02d}", binary, False))
    write_json(OUTPUT_ROOT / "reports" / f"{build}-suite.json", receipts)


def run_paired_suite(
    baseline_build: str,
    baseline_binary: Path,
    candidate_build: str,
    candidate_binary: Path,
) -> None:
    receipts: dict[str, list[dict[str, Any]]] = {
        baseline_build: [],
        candidate_build: [],
    }
    for build, binary in (
        (baseline_build, baseline_binary),
        (candidate_build, candidate_binary),
    ):
        receipts[build].append(run_once(build, "trace", binary, True))
        receipts[build].append(run_once(build, "warmup", binary, False))
        receipts[build].append(run_once(build, "empty-path", binary, False, "empty"))
        receipts[build].append(run_once(build, "filtered-out", binary, False, "filtered"))
    for index in range(1, REPETITIONS + 1):
        label = f"measure-{index:02d}"
        receipts[baseline_build].append(
            run_once(baseline_build, label, baseline_binary, False)
        )
        receipts[candidate_build].append(
            run_once(candidate_build, label, candidate_binary, False)
        )
    for build, rows in receipts.items():
        write_json(OUTPUT_ROOT / "reports" / f"{build}-suite.json", rows)


def measured_receipts(build: str) -> list[dict[str, Any]]:
    receipts = []
    for index in range(1, REPETITIONS + 1):
        path = run_paths(build, f"measure-{index:02d}")["receipt"]
        receipts.append(json.loads(path.read_text()))
    return receipts


def audit_trace(path: Path) -> dict[str, Any]:
    rows = 0
    stage3_rows = 0
    linked_rows = 0
    upstream_alias_rows = 0
    positive_hourly_alias_rows = 0
    stage3_routed_alias_rows = 0
    retained_store_alias_rows = 0
    omitted_retained_rows = 0
    doubled_refreeze_rows = 0
    independently_reconstructed_stage3_rows = 0
    maximum_upstream_error_m = 0.0
    maximum_stage3_error_m = 0.0
    for line in path.open():
        row = json.loads(line)
        rows += 1
        upstream = row["snowpack_swe_loss_m"] + row["rain_released_m"]
        upstream_error = abs(row["routed_melt_m"] - upstream)
        maximum_upstream_error_m = max(maximum_upstream_error_m, upstream_error)
        if abs(row["raw_melt_m"] - row["snowpack_swe_loss_m"]) > TOLERANCE_M:
            upstream_alias_rows += 1
        positive_hourly_melt_m = sum(
            max(hour["coe_melt_applied_m"], 0.0)
            for hour in row["accumulation_melt_hourly"]
        )
        if abs(positive_hourly_melt_m - row["snowpack_swe_loss_m"]) > TOLERANCE_M:
            positive_hourly_alias_rows += 1
        if row["stage3_energy_enabled"]:
            stage3_rows += 1
            if abs(row["stage3_incoming_liquid_m"] - row["routed_melt_m"]) <= TOLERANCE_M:
                linked_rows += 1
            if abs(row["routed_melt_m"] - row["stage3_routed_liquid_m"]) > TOLERANCE_M:
                stage3_routed_alias_rows += 1
            if (
                abs(
                    row["liquid_water_retained_after_m"]
                    - row["stage3_retained_liquid_delta_m"]
                )
                > TOLERANCE_M
            ):
                retained_store_alias_rows += 1
            if abs(row["stage3_retained_liquid_delta_m"]) > TOLERANCE_M:
                omitted_retained_rows += 1
            if abs(row["stage3_refrozen_liquid_m"]) > TOLERANCE_M:
                doubled_refreeze_rows += 1
            if any(
                abs(row[field]) > TOLERANCE_M
                for field in (
                    "stage3_incoming_liquid_m",
                    "stage3_routed_liquid_m",
                    "stage3_retained_liquid_delta_m",
                    "stage3_refrozen_liquid_m",
                )
            ):
                independently_reconstructed_stage3_rows += 1
        stage3 = (
            row["stage3_incoming_liquid_m"]
            - row["stage3_routed_liquid_m"]
            - row["stage3_retained_liquid_delta_m"]
            - row["stage3_refrozen_liquid_m"]
        )
        stage3_error = abs(stage3 - row["stage3_liquid_closure_residual_m"])
        maximum_stage3_error_m = max(maximum_stage3_error_m, stage3_error)
    return {
        "linked_stage3_rows": linked_rows,
        "maximum_stage3_error_m": maximum_stage3_error_m,
        "maximum_upstream_error_m": maximum_upstream_error_m,
        "positive_hourly_melt_alias_rejected_rows": positive_hourly_alias_rows,
        "rows": rows,
        "stage3_rows": stage3_rows,
        "stage3_routed_alias_rejected_rows": stage3_routed_alias_rows,
        "retained_store_alias_rejected_rows": retained_store_alias_rows,
        "omitted_retained_alias_rejected_rows": omitted_retained_rows,
        "doubled_refreeze_alias_rejected_rows": doubled_refreeze_rows,
        "independently_reconstructed_stage3_rows": independently_reconstructed_stage3_rows,
        "upstream_raw_melt_alias_rejected_rows": upstream_alias_rows,
    }


def normalized_loss(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text())
    value.pop("run_name", None)
    return value


def protected_outputs_match(reference: dict[str, Path], compared: dict[str, Path]) -> bool:
    return all(
        [
            sha256(reference["wat"]) == sha256(compared["wat"]),
            sha256(reference["hbp"]) == sha256(compared["hbp"]),
            normalized_loss(reference["loss"]) == normalized_loss(compared["loss"]),
        ]
    )


def compare(baseline: str, candidate: str) -> None:
    baseline_trace = run_paths(baseline, "trace")
    candidate_trace = run_paths(candidate, "trace")
    baseline_samples = measured_receipts(baseline)
    candidate_samples = measured_receipts(candidate)
    baseline_wall = statistics.median(row["wall_seconds"] for row in baseline_samples)
    candidate_wall = statistics.median(row["wall_seconds"] for row in candidate_samples)
    baseline_rss = statistics.median(row["peak_rss_kib"] for row in baseline_samples)
    candidate_rss = statistics.median(row["peak_rss_kib"] for row in candidate_samples)
    wall_ratio = candidate_wall / baseline_wall
    rss_ratio = candidate_rss / baseline_rss
    trace_ratio = candidate_trace["trace"].stat().st_size / baseline_trace["trace"].stat().st_size
    candidate_selector_outputs_match = all(
        protected_outputs_match(candidate_trace, run_paths(candidate, label))
        for label in ("warmup", "empty-path", "filtered-out")
    )
    baseline_selector_outputs_match = all(
        protected_outputs_match(baseline_trace, run_paths(baseline, label))
        for label in ("warmup", "empty-path", "filtered-out")
    )
    report = {
        "baseline": baseline,
        "baseline_trace_audit": audit_trace(baseline_trace["trace"]),
        "baseline_selector_protected_outputs_match": baseline_selector_outputs_match,
        "candidate": candidate,
        "candidate_trace_audit": audit_trace(candidate_trace["trace"]),
        "hbp_byte_identical": sha256(baseline_trace["hbp"]) == sha256(candidate_trace["hbp"]),
        "loss_equal_except_harness_run_name": normalized_loss(baseline_trace["loss"])
        == normalized_loss(candidate_trace["loss"]),
        "candidate_empty_path_has_no_trace": not run_paths(candidate, "empty-path")[
            "trace"
        ].exists(),
        "candidate_filtered_out_has_no_trace": not run_paths(candidate, "filtered-out")[
            "trace"
        ].exists(),
        "candidate_selector_protected_outputs_match": candidate_selector_outputs_match,
        "passed": all(
            [
                wall_ratio <= 1.05,
                rss_ratio <= 1.05,
                trace_ratio <= 1.01,
                sha256(baseline_trace["trace"]) == sha256(candidate_trace["trace"]),
                sha256(baseline_trace["wat"]) == sha256(candidate_trace["wat"]),
                sha256(baseline_trace["hbp"]) == sha256(candidate_trace["hbp"]),
                normalized_loss(baseline_trace["loss"])
                == normalized_loss(candidate_trace["loss"]),
                not run_paths(candidate, "empty-path")["trace"].exists(),
                not run_paths(candidate, "filtered-out")["trace"].exists(),
                candidate_selector_outputs_match,
                baseline_selector_outputs_match,
            ]
        ),
        "peak_rss": {
            "baseline_samples_kib": [row["peak_rss_kib"] for row in baseline_samples],
            "baseline_median_kib": baseline_rss,
            "candidate_samples_kib": [row["peak_rss_kib"] for row in candidate_samples],
            "candidate_median_kib": candidate_rss,
            "ratio": rss_ratio,
        },
        "trace_byte_identical": sha256(baseline_trace["trace"]) == sha256(candidate_trace["trace"]),
        "trace_size_ratio": trace_ratio,
        "wall_time": {
            "baseline_samples_seconds": [row["wall_seconds"] for row in baseline_samples],
            "baseline_median_seconds": baseline_wall,
            "candidate_samples_seconds": [row["wall_seconds"] for row in candidate_samples],
            "candidate_median_seconds": candidate_wall,
            "ratio": wall_ratio,
        },
        "wat_byte_identical": sha256(baseline_trace["wat"]) == sha256(candidate_trace["wat"]),
    }
    for audit in (report["baseline_trace_audit"], report["candidate_trace_audit"]):
        report["passed"] = report["passed"] and all(
            [
                audit["rows"] > 0,
                audit["linked_stage3_rows"] == audit["stage3_rows"],
                audit["upstream_raw_melt_alias_rejected_rows"] > 0,
                audit["positive_hourly_melt_alias_rejected_rows"] > 0,
                audit["stage3_routed_alias_rejected_rows"] > 0,
                audit["retained_store_alias_rejected_rows"] > 0,
                audit["omitted_retained_alias_rejected_rows"] > 0,
                audit["doubled_refreeze_alias_rejected_rows"] > 0,
                audit["independently_reconstructed_stage3_rows"] > 0,
                audit["maximum_upstream_error_m"] <= TOLERANCE_M,
                audit["maximum_stage3_error_m"] <= TOLERANCE_M,
            ]
        )
    write_json(OUTPUT_ROOT / "reports/comparison.json", report)
    if not report["passed"]:
        raise RuntimeError("compatibility/performance comparison failed")


def main() -> int:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)
    suite = subparsers.add_parser("suite")
    suite.add_argument("--build", required=True)
    suite.add_argument("--binary", required=True, type=Path)
    paired = subparsers.add_parser("paired-suite")
    paired.add_argument("--baseline-build", required=True)
    paired.add_argument("--baseline-binary", required=True, type=Path)
    paired.add_argument("--candidate-build", required=True)
    paired.add_argument("--candidate-binary", required=True, type=Path)
    comparison = subparsers.add_parser("compare")
    comparison.add_argument("--baseline", default="baseline")
    comparison.add_argument("--candidate", default="candidate")
    arguments = parser.parse_args()
    if arguments.command == "suite":
        run_suite(arguments.build, arguments.binary)
    elif arguments.command == "paired-suite":
        run_paired_suite(
            arguments.baseline_build,
            arguments.baseline_binary,
            arguments.candidate_build,
            arguments.candidate_binary,
        )
    else:
        compare(arguments.baseline, arguments.candidate)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, RuntimeError, ValueError, json.JSONDecodeError) as error:
        print(f"ledger_persistence.py: {error}", file=sys.stderr)
        raise SystemExit(1) from error
