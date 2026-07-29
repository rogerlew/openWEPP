#!/usr/bin/env python3
"""Run CAL-07C package-local forcing reconstruction and bounded evaluation."""

from __future__ import annotations

import csv
import subprocess
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
ART = PKG / "artifacts"


def run(command: list[str]) -> None:
    subprocess.run(command, cwd=ROOT, check=True)


def main() -> None:
    run([".venv/bin/python", str(PKG / "tools" / "prepare_inputs.py")])
    run(
        [
            "cargo",
            "run",
            "--quiet",
            "--manifest-path",
            str(PKG / "tools" / "executor" / "Cargo.toml"),
            "--",
            str(PKG / "inputs" / "ensemble.csv"),
            str(PKG / "inputs" / "forcing.csv"),
            str(ART / "daily-kernel-output.csv"),
        ]
    )
    commands = (
        (
            "producer_phase_transform",
            [
                "cargo",
                "test",
                "-p",
                "openwepp-plant-phenology",
                "--test",
                "native_canopy_contract",
                "full_wrapped_nh_climate_phase_flip_preserves_sh_canopy_and_limb_order",
                "--",
                "--exact",
                "--nocapture",
            ],
        ),
        (
            "real_consumer_ordering",
            [
                "cargo",
                "test",
                "-p",
                "openwepp-runner",
                "--lib",
                "native_forest_yaml_executes_through_the_direct_production_consumer",
                "--",
                "--nocapture",
            ],
        ),
    )
    rows = []
    for gate, command in commands:
        run(command)
        rows.append({"gate": gate, "status": "PASS", "command": " ".join(command)})
    with (ART / "gate-results.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(
            stream,
            fieldnames=("gate", "status", "command"),
            lineterminator="\n",
        )
        writer.writeheader()
        writer.writerows(rows)


if __name__ == "__main__":
    main()
