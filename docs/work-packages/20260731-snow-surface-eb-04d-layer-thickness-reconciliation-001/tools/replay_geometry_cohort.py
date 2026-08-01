#!/usr/bin/env python3
"""Replay the two frozen EB-04 geometry failures with the corrected binary."""

from __future__ import annotations

import fcntl
import hashlib
import importlib.util
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
OUTPUT = REPO / "target/snow_surface_eb04d_replay"
LOCK = REPO / "target/snow_surface_eb04d_replay.lock"
BINARY = REPO / "target/debug/openwepp-cli-hill"
EB04A_REPORT = REPO / (
    "docs/work-packages/"
    "20260730-snow-surface-eb-04a-failure-observability-state-capture-001/"
    "artifacts/diagnostic-replay.json"
)
EB04_TOOL = REPO / (
    "docs/work-packages/"
    "20260730-snow-surface-eb-04-factorial-execution-adjudication-001/"
    "tools/run_factorial.py"
)
OBSERVED_HELPERS = REPO / "tools/snowfreeze_observed"
TARGETS = {("harvard_open", "S"), ("marcell_open", "LS")}
EXPECTED_FINAL_DAY_INDEX = 16_436
EXPECTED_TRACE_ROW_COUNT = 16_437
EXPECTED_BASE_HEAD = "93df703f"
EXPECTED_BINARY_SHA256 = "0242c39fa26e9cbbd9461a36a4d6843b8adf0600fb72c215c349a454cbf66a50"
EXPECTED_SOURCE_DIFF_SHA256 = "82b53ac873d61628989ffbe8137df4519c852f5fb803232824b3c0623c2da385"
EXPECTED_SOURCE_REPORT_SHA256 = "8208c12e608a47e57c0f9d1c47d10e95ffd01c6b649e119cba3448abeb7f3657"
EXPECTED_IMPORTED_HARNESS_SHA256 = "e84a1732a847b978cc529ba95bb276b4f47ff37e991d06798d158523f2bace17"
EXPECTED_OBSERVED_HELPER_TREE_SHA256 = "5c424eb257d38b3554f66aed63f66d4fe88b90fd0475a7ce8c725f987112df9e"
CAPTURED_STATES = {
    ("harvard_open", "S"): {
        "failure_day_index": 2_643,
        "classification": "PRIOR_LAYER_THICKNESS_AGGREGATE_MISMATCH",
        "fixture_sha256": "85a1c5a04b51862af6ecdbf8648a0a2b395bbdff6055cca489dffdd4620ca956",
        "selectors": {"longwave": "disabled", "sublimation": "neutral_bulk_stage3_v1"},
        "layer_count": 5,
        "prior_swe_m": 0.131_111_401_893_932_47,
        "prior_depth_m": 0.251_171_267_996_039_06,
        "fragment_mass_swe_m": 5.260_584_353_128_359e-10,
        "fragment_depth_m": 1.007_774_780_292_791_7e-9,
    },
    ("marcell_open", "LS"): {
        "failure_day_index": 3_371,
        "classification": "PRIOR_LAYER_THICKNESS_AGGREGATE_MISMATCH",
        "fixture_sha256": "7396749ffe7277aecd7be05ee10791e0002e202191ca4d1b37c2adc544bb4bb1",
        "selectors": {
            "longwave": "dilley_unsworth_subcanopy_v1",
            "sublimation": "neutral_bulk_stage3_v1",
        },
        "layer_count": 14,
        "prior_swe_m": 0.129_065_105_589_441_3,
        "prior_depth_m": 0.267_942_167_336_576_1,
        "fragment_mass_swe_m": 5.267_347_169_024_66e-10,
        "fragment_depth_m": 1.088_162_587_814_523e-9,
    },
}
LAYER_FIELDS = {
    "mass_swe_m",
    "thickness_m",
    "density_kg_m3",
    "settle_day_count",
    "temperature_c",
    "liquid_water_m",
    "cold_content_j_m2",
    "refrozen_liquid_m",
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def python_tree_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    for source in sorted(path.glob("*.py")):
        digest.update(source.name.encode("utf-8"))
        digest.update(b"\0")
        digest.update(source.read_bytes())
        digest.update(b"\0")
    return digest.hexdigest()


def command_sha256(command: list[str]) -> str:
    completed = subprocess.run(
        command, cwd=REPO, check=True, stdout=subprocess.PIPE
    )
    return hashlib.sha256(completed.stdout).hexdigest()


def relative(path: Path) -> str:
    return str(path.relative_to(REPO))


def load_runner() -> Any:
    spec = importlib.util.spec_from_file_location("eb04d_runner", EB04_TOOL)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {EB04_TOOL}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    with path.open(encoding="utf-8") as stream:
        return [json.loads(line) for line in stream if line.strip()]


def typed_snapshot_layers(snapshot: str) -> list[dict[str, float]]:
    layers = []
    for body in re.findall(r"DirectSnowLayerState \{ ([^}]+) \}", snapshot):
        fields = {}
        for name, value in re.findall(r"([a-z0-9_]+): ([^,}]+)", body):
            fields[name] = float(value)
        if set(fields) != LAYER_FIELDS:
            raise RuntimeError(
                f"typed snapshot layer fields changed: {sorted(fields)}"
            )
        layers.append(fields)
    return layers


def main() -> int:
    LOCK.parent.mkdir(parents=True, exist_ok=True)
    with LOCK.open("w", encoding="utf-8") as lock_stream:
        fcntl.flock(lock_stream, fcntl.LOCK_EX)
        return run_locked()


def run_locked() -> int:
    build_command = ["cargo", "build", "-p", "openwepp-runner", "--bin", "openwepp-cli-hill"]
    subprocess.run(build_command, cwd=REPO, check=True)
    if not BINARY.is_file():
        raise FileNotFoundError(f"build did not produce exact binary: {BINARY}")
    runner = load_runner()
    source = json.loads(EB04A_REPORT.read_text(encoding="utf-8"))
    frozen = {
        (row["lane_id"], row["cell"]): row
        for row in source["results"]
        if (row["lane_id"], row["cell"]) in TARGETS
    }
    if set(frozen) != TARGETS:
        raise RuntimeError(f"frozen target mismatch: {sorted(frozen)}")
    lanes = {lane.lane_id: lane for lane in runner.fixed_lanes()}
    source_identity = {
        "git_head": subprocess.run(
            ["git", "rev-parse", "--short=8", "HEAD"],
            cwd=REPO,
            check=True,
            text=True,
            stdout=subprocess.PIPE,
        ).stdout.strip(),
        "build_command": build_command,
        "binary_sha256": sha256(BINARY),
        "source_diff_sha256": command_sha256(
            ["git", "diff", "--binary", "--", "crates", "tests"]
        ),
        "tool_sha256": sha256(Path(__file__)),
        "imported_harness_sha256": sha256(EB04_TOOL),
        "observed_helper_tree_sha256": python_tree_sha256(OBSERVED_HELPERS),
        "source_report_sha256": sha256(EB04A_REPORT),
    }
    OUTPUT.mkdir(parents=True, exist_ok=True)
    results = [
        replay(runner, lanes[lane_id], cell, frozen[(lane_id, cell)])
        for lane_id, cell in sorted(TARGETS)
    ]
    post_identity = {
        "git_head": subprocess.run(
            ["git", "rev-parse", "--short=8", "HEAD"],
            cwd=REPO,
            check=True,
            text=True,
            stdout=subprocess.PIPE,
        ).stdout.strip(),
        "build_command": build_command,
        "binary_sha256": sha256(BINARY),
        "source_diff_sha256": command_sha256(
            ["git", "diff", "--binary", "--", "crates", "tests"]
        ),
        "tool_sha256": sha256(Path(__file__)),
        "imported_harness_sha256": sha256(EB04_TOOL),
        "observed_helper_tree_sha256": python_tree_sha256(OBSERVED_HELPERS),
        "source_report_sha256": sha256(EB04A_REPORT),
    }
    if post_identity != source_identity:
        changes = {
            key: {"before": source_identity[key], "after": post_identity[key]}
            for key in source_identity
            if source_identity[key] != post_identity[key]
        }
        raise RuntimeError(f"replay identity changed during execution: {changes}")
    report = {
        "schema": "snow-surface-eb04d-geometry-replay-v1",
        "evidence_class": "Ran",
        **source_identity,
        "target_count": len(results),
        "all_pass_former_boundary": all(row["passes_former_boundary"] for row in results),
        "all_complete": all(row["returncode"] == 0 for row in results),
        "all_reach_frozen_terminal": all(row["reaches_frozen_terminal"] for row in results),
        "all_match_frozen_identity": all(row["frozen_identity_matches"] for row in results),
        "all_reconstruct_captured_state": all(
            row["captured_state_reconstruction_passes"] for row in results
        ),
        "maximum_abs_mass_residual_m": max(row["maximum_abs_mass_residual_m"] for row in results),
        "maximum_abs_depth_residual_m": max(row["maximum_abs_depth_residual_m"] for row in results),
        "results": results,
    }
    report["acceptance_passes"] = (
        report["target_count"] == 2
        and report["all_pass_former_boundary"]
        and report["all_complete"]
        and report["all_reach_frozen_terminal"]
        and report["all_match_frozen_identity"]
        and report["all_reconstruct_captured_state"]
        and report["git_head"] == EXPECTED_BASE_HEAD
        and report["binary_sha256"] == EXPECTED_BINARY_SHA256
        and report["source_diff_sha256"] == EXPECTED_SOURCE_DIFF_SHA256
        and report["source_report_sha256"] == EXPECTED_SOURCE_REPORT_SHA256
        and report["imported_harness_sha256"] == EXPECTED_IMPORTED_HARNESS_SHA256
        and report["observed_helper_tree_sha256"]
        == EXPECTED_OBSERVED_HELPER_TREE_SHA256
        and report["maximum_abs_mass_residual_m"] <= 1.0e-9
        and report["maximum_abs_depth_residual_m"] <= 1.0e-9
    )
    target = ARTIFACTS / "geometry-cohort-replay.json"
    temporary = target.with_suffix(".json.tmp")
    temporary.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    temporary.replace(target)
    print(json.dumps({key: value for key, value in report.items() if key != "results"}, indent=2))
    return 0 if report["acceptance_passes"] else 1


def replay(runner: Any, lane: Any, cell: str, frozen: dict[str, Any]) -> dict[str, Any]:
    run_dir = OUTPUT / lane.lane_id / cell
    run_dir.mkdir(parents=True, exist_ok=True)
    run_id = f"{lane.lane_id}-{cell}-eb04d"
    runfile = run_dir / f"{run_id}.run"
    trace = run_dir / f"{run_id}.snow.jsonl"
    stem = runner.observed_harness.discover_run_stem(lane.fixture_dir)
    runner.observed_harness.write_runfile(runfile, lane.fixture_dir, stem, run_dir, run_id)
    command = runner.observed_harness.cli_command(
        BINARY, lane.fixture_dir, runfile, run_dir, "direct-production-executor"
    )
    longwave, sublimation = runner.CELLS[cell]
    environment = os.environ.copy()
    environment.update(runner.NON_TARGET_ENV)
    environment.update(
        {
            "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": longwave,
            "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL": sublimation,
            "OPENWEPP_R7H_SNOW_TRACE_PATH": str(trace),
        }
    )
    if trace.exists():
        trace.unlink()
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
    former_day = int(frozen["failure_day_index"])
    last_completed_day = int(rows[-1]["day_index"]) if rows else None
    chronology_is_exact = [int(row["day_index"]) for row in rows] == list(
        range(EXPECTED_TRACE_ROW_COUNT)
    )
    former_day_occurrences = sum(int(row["day_index"]) == former_day for row in rows)
    captured = CAPTURED_STATES[(lane.lane_id, cell)]
    captured_layers = typed_snapshot_layers(frozen["typed_snapshot"])
    captured_mass_sum_m = sum(layer["mass_swe_m"] for layer in captured_layers)
    captured_depth_sum_m = sum(layer["thickness_m"] for layer in captured_layers)
    captured_fragment_matches = any(
        abs(layer["mass_swe_m"] - captured["fragment_mass_swe_m"]) <= 1.0e-18
        and abs(layer["thickness_m"] - captured["fragment_depth_m"]) <= 1.0e-18
        for layer in captured_layers
    )
    maximum_mass_residual_m = max(
        (
            abs(
                sum(layer["mass_swe_m"] for layer in row["snow_layers_after"])
                - row["runtime_swe_after_m"]
            )
            for row in rows
        ),
        default=0.0,
    )
    maximum_depth_residual_m = max(
        (
            abs(
                sum(layer["thickness_m"] for layer in row["snow_layers_after"])
                - row["runtime_depth_after_m"]
            )
            for row in rows
        ),
        default=0.0,
    )
    fixture_sha256 = runner.tree_sha256(lane.fixture_dir)
    selector_identity = {"longwave": longwave, "sublimation": sublimation}
    return {
        "lane_id": lane.lane_id,
        "cell": cell,
        "returncode": completed.returncode,
        "source_failure_day_index": former_day,
        "last_completed_day_index": last_completed_day,
        "passes_former_boundary": bool(last_completed_day is not None and last_completed_day >= former_day),
        "expected_final_day_index": EXPECTED_FINAL_DAY_INDEX,
        "expected_trace_row_count": EXPECTED_TRACE_ROW_COUNT,
        "reaches_frozen_terminal": (
            last_completed_day == EXPECTED_FINAL_DAY_INDEX
            and len(rows) == EXPECTED_TRACE_ROW_COUNT
            and chronology_is_exact
            and former_day_occurrences == 1
        ),
        "chronology_is_exact": chronology_is_exact,
        "former_day_occurrences": former_day_occurrences,
        "trace_row_count": len(rows),
        "fixture": relative(lane.fixture_dir),
        "fixture_sha256": fixture_sha256,
        "runfile": relative(runfile),
        "runfile_sha256": sha256(runfile),
        "trace": relative(trace),
        "trace_sha256": sha256(trace),
        "selectors": selector_identity,
        "frozen_identity_matches": (
            fixture_sha256 == frozen["fixture_sha256"]
            and frozen["fixture_sha256"] == captured["fixture_sha256"]
            and selector_identity == frozen["selectors"]
            and frozen["selectors"] == captured["selectors"]
            and relative(lane.fixture_dir) == frozen["fixture"]
            and former_day == captured["failure_day_index"]
            and frozen["classification"] == captured["classification"]
        ),
        "captured_state": captured,
        "captured_layer_count": len(captured_layers),
        "captured_mass_sum_m": captured_mass_sum_m,
        "captured_depth_sum_m": captured_depth_sum_m,
        "captured_fragment_matches": captured_fragment_matches,
        "captured_state_source": "EB-04A typed SnowLayerAggregateMismatch snapshot",
        "captured_state_reconstruction_passes": (
            len(captured_layers) == captured["layer_count"]
            and abs(captured_mass_sum_m - captured["prior_swe_m"]) <= 1.0e-15
            and abs(captured_depth_sum_m - captured["prior_depth_m"]) <= 1.0e-15
            and captured_fragment_matches
        ),
        "command": command,
        "maximum_abs_mass_residual_m": maximum_mass_residual_m,
        "maximum_abs_depth_residual_m": maximum_depth_residual_m,
        "stderr_tail": completed.stderr.strip().splitlines()[-1] if completed.stderr.strip() else "",
    }


if __name__ == "__main__":
    raise SystemExit(main())
