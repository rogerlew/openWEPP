#!/usr/bin/env python3
"""Run HPHYS0291 release-binary H1..H39 semantic diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import sys
from pathlib import Path
from typing import Any


REPO = Path("/home/workdir/openWEPP")
HPHYS0265_SCRIPT = (
    REPO
    / "docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py"
)
HILL_BIN = REPO / "target/release/openwepp-cli-hill"


def load_hphys0265_module() -> Any:
    spec = importlib.util.spec_from_file_location("hphys0265_diagnostics", HPHYS0265_SCRIPT)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {HPHYS0265_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0265 = load_hphys0265_module()


def run_full_hillslope_suite(run_root: Path) -> int:
    reports = run_root / "reports"
    logs = run_root / "logs"
    output = run_root / "hillslope_output"
    reports.mkdir(parents=True, exist_ok=True)
    logs.mkdir(parents=True, exist_ok=True)
    output.mkdir(parents=True, exist_ok=True)
    runs_dir = HPHYS0265.copy_runfiles(run_root)

    for required in [
        HPHYS0265.WEPPPY_PYTHON,
        HPHYS0265.COMPARATOR,
        HPHYS0265.TOLERANCES,
        HPHYS0265.BASELINE_PARTITIONS,
    ]:
        HPHYS0265.require_path(required)

    build = HPHYS0265.run_command(
        "cargo_build_release_openwepp_cli_hill",
        ["cargo", "build", "--release", "-p", "openwepp-runner", "--bin", "openwepp-cli-hill"],
        logs,
    )
    HPHYS0265.write_status(
        reports / "build_status.tsv",
        [
            {
                "command": "cargo build --release -p openwepp-runner --bin openwepp-cli-hill",
                "rc": build.rc,
                "seconds": f"{build.seconds:.3f}",
                "stdout": build.stdout,
                "stderr": build.stderr,
            }
        ],
    )
    if build.rc != 0:
        return build.rc

    batch_rows = []
    for hillslope_id in range(1, 40):
        result = HPHYS0265.run_command(
            f"H{hillslope_id}",
            [
                str(HILL_BIN),
                "--run-dir",
                str(runs_dir),
                "--run-file",
                f"p{hillslope_id}_openwepp.run",
                "--output-dir",
                str(output),
                "--policy",
                "compat",
            ],
            logs / "hillslopes",
        )
        batch_rows.append(
            {
                "hillslope_id": hillslope_id,
                "rc": result.rc,
                "seconds": f"{result.seconds:.3f}",
                "stdout": result.stdout,
                "stderr": result.stderr,
            }
        )
    HPHYS0265.write_status(reports / "hillslope_batch_status.tsv", batch_rows)
    failed = [row for row in batch_rows if row["rc"] != 0]
    if failed:
        return int(failed[0]["rc"])

    HPHYS0265.run_semantics(run_root)
    summary_md = reports / "hillslope_semantic_summary.md"
    summary_md.write_text(
        summary_md.read_text(encoding="utf-8").replace(
            "# HPHYS0265 Full 39 Semantic Summary",
            "# HPHYS0291 Full H1..H39 Semantic Summary",
        ),
        encoding="utf-8",
    )
    write_hphys0291_selected_deltas(run_root)
    return 0


def write_hphys0291_selected_deltas(run_root: Path) -> None:
    reports = run_root / "reports"
    summary_json = reports / "hillslope_semantic_summary.json"
    summary = json.loads(summary_json.read_text(encoding="utf-8"))
    selected = {
        row["column"]: row
        for row in summary
        if row["column"]
        in {
            "Ep",
            "Es",
            "Er",
            "Total-Soil",
            "SoilWaterTotal",
            "Dp",
            "latqcc",
            "Q",
            "RM",
            "Snow-Water",
            "P",
        }
    }
    (reports / "hphys0291_selected_metrics.json").write_text(
        json.dumps(selected, indent=2) + "\n", encoding="utf-8"
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", required=True, type=Path)
    args = parser.parse_args()
    return run_full_hillslope_suite(args.run_root)


if __name__ == "__main__":
    raise SystemExit(main())
