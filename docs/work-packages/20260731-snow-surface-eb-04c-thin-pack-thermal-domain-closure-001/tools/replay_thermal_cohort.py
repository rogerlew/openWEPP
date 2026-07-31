#!/usr/bin/env python3
"""Replay the 22 EB-04B thermal failures against the EB-04C binary."""

from __future__ import annotations

import csv
import fcntl
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
OUTPUT = REPO / "target/snow_surface_eb04c_replay"
LOCK = REPO / "target/snow_surface_eb04c_replay.lock"
BINARY = REPO / "target/debug/openwepp-cli-hill"
EB04 = REPO / "docs/work-packages/20260730-snow-surface-eb-04-factorial-execution-adjudication-001"
EB04_TOOL = EB04 / "tools/run_factorial.py"
MANIFEST = REPO / (
    "docs/work-packages/20260731-snow-surface-eb-04b-coupled-dynamics-characterization-001/"
    "artifacts/frozen-input-manifest.csv"
)
FROZEN_MANIFEST_SHA256 = "641544e187b345a0c8f2d3c37858e0dd04c06ab776bb9a616983b4838aba6f19"


def load_runner() -> Any:
    spec = importlib.util.spec_from_file_location("eb04_runner", EB04_TOOL)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {EB04_TOOL}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


RUNNER = load_runner()


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    return [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines() if line]


def executable_source_diff_sha256() -> str:
    source_diff = subprocess.run(
        ["git", "diff", "--binary", "--", "crates", "tests"],
        cwd=REPO,
        check=True,
        stdout=subprocess.PIPE,
    ).stdout
    return hashlib.sha256(source_diff).hexdigest()


def replay(target: dict[str, str], lane: Any) -> dict[str, Any]:
    cell = target["cell"]
    run_dir = OUTPUT / lane.lane_id / cell
    run_dir.mkdir(parents=True, exist_ok=True)
    run_id = f"{lane.lane_id}-{cell}-eb04c"
    runfile = run_dir / f"{run_id}.run"
    trace = run_dir / f"{run_id}.snow.jsonl"
    stem = RUNNER.observed_harness.discover_run_stem(lane.fixture_dir)
    RUNNER.observed_harness.write_runfile(runfile, lane.fixture_dir, stem, run_dir, run_id)
    command = RUNNER.observed_harness.cli_command(
        BINARY, lane.fixture_dir, runfile, run_dir, "direct-production-executor"
    )
    longwave, sublimation = RUNNER.CELLS[cell]
    environment = os.environ.copy()
    environment.update(RUNNER.NON_TARGET_ENV)
    environment.update(
        {
            "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": longwave,
            "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL": sublimation,
            "OPENWEPP_R7H_SNOW_TRACE_PATH": str(trace),
        }
    )
    trace.unlink(missing_ok=True)
    completed = subprocess.run(
        command,
        cwd=REPO,
        env=environment,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    (run_dir / "stdout.txt").write_text(completed.stdout, encoding="utf-8")
    (run_dir / "stderr.txt").write_text(completed.stderr, encoding="utf-8")
    rows = read_jsonl(trace)
    source_last_completed_day = int(target["failure_day_index"])
    rejected_processing_day = source_last_completed_day + 1
    boundary_rows = [
        row
        for row in rows
        if source_last_completed_day
        <= int(row["day_index"]) + 1
        <= rejected_processing_day
    ]
    last_day = int(rows[-1]["day_index"]) + 1 if rows else 0
    stderr = completed.stderr.strip()
    forbidden = (
        "must be above absolute zero",
        "pressure_pa must be > 0; received 0",
        "snow.stage3_energy_residual_j_m2",
        "snow.stage3_missing_layers_with_snow",
    )
    def suspended_on(row: dict[str, Any]) -> bool:
        return (
            float(row["stage3_thermal_domain_suspended_seconds"]) > 0.0
            and 0.0
            < float(row["stage3_minimum_unresolved_thermal_mass_kg_m2"])
            <= 1.0
        )

    def collapsed_on(row: dict[str, Any]) -> bool:
        return (
            float(row["stage3_lower_thermal_volume_collapsed_seconds"]) > 0.0
            and 0.0
            < float(row["stage3_minimum_collapsed_lower_mass_kg_m2"])
            < 1.0
        )

    target_row = next(
        (row for row in boundary_rows if suspended_on(row) or collapsed_on(row)), None
    )
    suspended = bool(target_row and suspended_on(target_row))
    collapsed = bool(target_row and collapsed_on(target_row))
    passed_rejected_processing_day = last_day >= rejected_processing_day
    branch_observed = suspended or collapsed
    forbidden_error = next((item for item in forbidden if item in stderr), "")
    return {
        "lane_id": lane.lane_id,
        "cell": cell,
        "prior_classification": target["classification"],
        "source_last_completed_day": source_last_completed_day,
        "rejected_processing_day": rejected_processing_day,
        "source_boundary_window_days": [
            source_last_completed_day,
            rejected_processing_day,
        ],
        "returncode": completed.returncode,
        "last_completed_day": last_day,
        "passed_rejected_processing_day": passed_rejected_processing_day,
        "source_branch_observed_in_boundary_window": branch_observed,
        "source_branch_trace_day": int(target_row["day_index"]) + 1 if target_row else None,
        "suspension_observed_in_boundary_window": suspended,
        "lower_collapse_observed_in_boundary_window": collapsed,
        "suspended_seconds_on_source_branch_day": (
            float(target_row["stage3_thermal_domain_suspended_seconds"])
            if target_row
            else None
        ),
        "minimum_unresolved_mass_kg_m2_on_source_branch_day": (
            float(target_row["stage3_minimum_unresolved_thermal_mass_kg_m2"])
            if target_row
            else None
        ),
        "lower_collapsed_seconds_on_source_branch_day": (
            float(target_row["stage3_lower_thermal_volume_collapsed_seconds"])
            if target_row
            else None
        ),
        "minimum_collapsed_lower_mass_kg_m2_on_source_branch_day": (
            float(target_row["stage3_minimum_collapsed_lower_mass_kg_m2"])
            if target_row
            else None
        ),
        "forbidden_thermal_error": forbidden_error,
        "terminal_error": stderr.splitlines()[-1] if stderr else "",
        "trace": str(trace.relative_to(REPO)),
        "trace_sha256": sha256(trace) if trace.exists() else "",
        "fixture": str(lane.fixture_dir.relative_to(REPO)),
        "fixture_tree_sha256": RUNNER.tree_sha256(lane.fixture_dir),
        "generated_runfile": str(runfile.relative_to(REPO)),
        "generated_runfile_sha256": sha256(runfile),
        "command_argv": [str(argument) for argument in command],
        "target_environment": {
            "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": longwave,
            "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL": sublimation,
            "OPENWEPP_R7H_SNOW_TRACE_PATH": str(trace),
            **RUNNER.NON_TARGET_ENV,
        },
        "source_manifest_trace_sha256": target["trace_sha256"],
        "source_manifest_typed_snapshot_sha256": target["typed_snapshot_sha256"],
    }


def run_replay() -> int:
    if not BINARY.is_file():
        raise FileNotFoundError(f"build the exact binary first: {BINARY}")
    manifest_sha256 = sha256(MANIFEST)
    if manifest_sha256 != FROZEN_MANIFEST_SHA256:
        raise RuntimeError(
            "EB-04B thermal manifest identity mismatch: "
            f"expected {FROZEN_MANIFEST_SHA256}, received {manifest_sha256}"
        )
    initial_binary_sha256 = sha256(BINARY)
    initial_source_diff_sha256 = executable_source_diff_sha256()
    replay_tool_sha256 = sha256(Path(__file__).resolve())
    with MANIFEST.open(newline="", encoding="utf-8") as stream:
        targets = [
            row
            for row in csv.DictReader(stream)
            if row["classification"] != "PRIOR_LAYER_THICKNESS_AGGREGATE_MISMATCH"
        ]
    if len(targets) != 22:
        raise RuntimeError(f"expected 22 thermal targets, found {len(targets)}")
    target_keys = [(row["lane_id"], row["cell"]) for row in targets]
    if len(set(target_keys)) != len(target_keys):
        raise RuntimeError("thermal target manifest contains duplicate lane/cell keys")
    lanes = {lane.lane_id: lane for lane in RUNNER.fixed_lanes()}
    results: list[dict[str, Any]] = []
    with ThreadPoolExecutor(max_workers=4) as executor:
        futures = {
            executor.submit(replay, target, lanes[target["lane_id"]]): target
            for target in targets
        }
        for future in as_completed(futures):
            row = future.result()
            results.append(row)
            print(
                f"{row['lane_id']}/{row['cell']}: day={row['last_completed_day']} "
                f"branch={row['source_branch_observed_in_boundary_window']} rc={row['returncode']}"
            )
    results.sort(key=lambda row: (row["lane_id"], row["cell"]))
    final_binary_sha256 = sha256(BINARY)
    final_source_diff_sha256 = executable_source_diff_sha256()
    if final_binary_sha256 != initial_binary_sha256:
        raise RuntimeError("runner binary changed during EB-04C replay")
    if final_source_diff_sha256 != initial_source_diff_sha256:
        raise RuntimeError("crates/tests source diff changed during EB-04C replay")
    if sha256(Path(__file__).resolve()) != replay_tool_sha256:
        raise RuntimeError("EB-04C replay tool changed during execution")
    report = {
        "schema": "snow-surface-eb04c-thermal-replay-v2",
        "evidence_class": "Ran",
        "git_head": subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=REPO, check=True, text=True,
            stdout=subprocess.PIPE,
        ).stdout.strip(),
        "working_directory": str(REPO),
        "executable_source_diff_sha256": initial_source_diff_sha256,
        "binary_sha256": initial_binary_sha256,
        "replay_tool": str(Path(__file__).resolve().relative_to(REPO)),
        "replay_tool_sha256": replay_tool_sha256,
        "source_manifest": str(MANIFEST.relative_to(REPO)),
        "source_manifest_sha256": manifest_sha256,
        "expected_source_manifest_sha256": FROZEN_MANIFEST_SHA256,
        "source_runner_tool": str(EB04_TOOL.relative_to(REPO)),
        "source_runner_tool_sha256": sha256(EB04_TOOL),
        "target_count": len(results),
        "unique_target_count": len(set(target_keys)),
        "all_passed_rejected_processing_day": all(
            row["passed_rejected_processing_day"] for row in results
        ),
        "all_source_branches_observed": all(
            row["source_branch_observed_in_boundary_window"] for row in results
        ),
        "no_forbidden_thermal_error": all(
            not row["forbidden_thermal_error"] for row in results
        ),
        "results": results,
    }
    report["acceptance_passes"] = (
        report["target_count"] == 22
        and report["unique_target_count"] == 22
        and report["all_passed_rejected_processing_day"]
        and report["all_source_branches_observed"]
        and report["no_forbidden_thermal_error"]
    )
    output = ARTIFACTS / "thermal-cohort-replay.json"
    temporary_output = output.with_suffix(".json.tmp")
    temporary_output.write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    temporary_output.replace(output)
    print(json.dumps({key: value for key, value in report.items() if key.startswith("all_") or key in {"target_count", "no_forbidden_thermal_error", "acceptance_passes"}}, indent=2))
    return 0 if report["acceptance_passes"] else 1


def main() -> int:
    LOCK.parent.mkdir(parents=True, exist_ok=True)
    lock_fd = os.open(LOCK, os.O_CREAT | os.O_RDWR, 0o600)
    try:
        try:
            fcntl.flock(lock_fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError as error:
            raise RuntimeError(f"another EB-04C replay owns {LOCK}") from error
        os.ftruncate(lock_fd, 0)
        os.write(lock_fd, f"pid={os.getpid()}\n".encode())
        return run_replay()
    finally:
        fcntl.flock(lock_fd, fcntl.LOCK_UN)
        os.close(lock_fd)


if __name__ == "__main__":
    raise SystemExit(main())
