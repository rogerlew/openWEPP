#!/usr/bin/env python3
"""Re-score bound Harvard bulk snow observations after operator adjudication."""

from __future__ import annotations

import csv
import hashlib
import importlib.util
import json
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
EXECUTE_PATH = Path(__file__).with_name("execute.py")


def load_execute() -> Any:
    spec = importlib.util.spec_from_file_location("cal06_execute", EXECUTE_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("failed to load CAL-06 executor")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def main() -> int:
    executor_module = load_execute()
    members = executor_module.accepted_members()
    lanes = [
        lane
        for lane in executor_module.LANES
        if lane.site == "harvard" and lane.stratum in {"deciduous", "open"}
    ]
    tasks = []
    for lane in lanes:
        if lane.forest:
            tasks.extend((lane, member) for member in members)
        else:
            tasks.append((lane, None))
    if len(tasks) != 38:
        raise SystemExit(f"expected 38 Harvard re-score runs, found {len(tasks)}")
    runner = (executor_module.ROOT / "target/release/openwepp-cli-hill").resolve()
    observations = executor_module.load_observations()
    rescored: list[dict[str, Any]] = []
    run_manifest: list[dict[str, Any]] = []
    with tempfile.TemporaryDirectory(prefix="openwepp-cal06-harvard-rescore-") as root:
        scratch = Path(root)
        with ThreadPoolExecutor(max_workers=10) as pool:
            pending = {
                pool.submit(
                    executor_module.execute_one,
                    lane,
                    member,
                    runner,
                    scratch,
                    observations,
                ): (lane, member)
                for lane, member in tasks
            }
            for count, future in enumerate(as_completed(pending), 1):
                _, _, _, scores, manifest = future.result()
                rescored.extend(scores)
                run_manifest.append(
                    {
                        "run_id": manifest["run_id"],
                        "state": manifest["state"],
                        "trace_sha256": manifest["trace_sha256"],
                        "wat_sha256": manifest["wat_sha256"],
                    }
                )
                if count % 10 == 0 or count == len(tasks):
                    print(f"completed {count}/{len(tasks)}", flush=True)

    existing = read_csv(ARTIFACTS / "observation-scores.csv")
    replacements = {
        (row["fixture"], row["member_id"], row["quantity"]): row for row in rescored
    }
    output: list[dict[str, Any]] = []
    replaced = 0
    for row in existing:
        key = (row["fixture"], row["member_id"], row["quantity"])
        if key in replacements:
            output.append(replacements[key])
            replaced += 1
        else:
            output.append(row)
    if replaced != 38 * 3:
        raise SystemExit(f"expected 114 score-row replacements, observed {replaced}")
    output.sort(
        key=lambda row: (
            row["site"],
            row["stratum"],
            row["member_id"],
            row["quantity"],
        )
    )
    run_manifest.sort(key=lambda row: row["run_id"])
    write_csv(ARTIFACTS / "observation-scores.csv", output)
    write_csv(ARTIFACTS / "harvard-rescore-manifest.csv", run_manifest)

    result_manifest_path = ARTIFACTS / "result-manifest.json"
    result_manifest = json.loads(result_manifest_path.read_text(encoding="utf-8"))
    score_path = ARTIFACTS / "observation-scores.csv"
    result_manifest["outputs"]["observation-scores.csv"] = {
        "sha256": sha256(score_path),
        "bytes": score_path.stat().st_size,
    }
    result_manifest["harvard_rescore"] = {
        "run_count": len(run_manifest),
        "manifest_sha256": sha256(ARTIFACTS / "harvard-rescore-manifest.csv"),
        "bulk_density_operator": "WAT aggregate density versus HF237-01 daily density",
        "profile_density_disposition": "NOT_EVALUATED_SCALE_MISMATCH",
        "swe_disposition": "INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION",
    }
    result_manifest_path.write_text(
        json.dumps(result_manifest, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    print("PASS: replaced 114 Harvard score rows from 38 deterministic re-score runs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
