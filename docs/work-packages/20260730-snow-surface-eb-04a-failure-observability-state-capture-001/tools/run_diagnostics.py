#!/usr/bin/env python3
"""Replay and classify the 24 frozen EB-04 failures with EB-04A diagnostics."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import math
import os
import re
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
OUTPUT = REPO / "target/snow_surface_eb04a_diagnostics"
BINARY = REPO / "target/debug/openwepp-cli-hill"
EB04 = REPO / (
    "docs/work-packages/"
    "20260730-snow-surface-eb-04-factorial-execution-adjudication-001"
)
EB04_RESULTS = EB04 / "artifacts/factorial-results.json"
EB04_TOOL = EB04 / "tools/run_factorial.py"


def load_eb04_tool() -> Any:
    spec = importlib.util.spec_from_file_location("eb04_runner", EB04_TOOL)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {EB04_TOOL}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


EB04_RUNNER = load_eb04_tool()


def main() -> int:
    self_check()
    if not BINARY.is_file():
        raise FileNotFoundError(f"build exact diagnostic binary first: {BINARY}")
    FIGURES.mkdir(parents=True, exist_ok=True)
    OUTPUT.mkdir(parents=True, exist_ok=True)
    prior = json.loads(EB04_RESULTS.read_text(encoding="utf-8"))
    lanes = {lane.lane_id: lane for lane in EB04_RUNNER.fixed_lanes()}
    targets = [
        (lanes[lane["lane_id"]], cell_name, cell)
        for lane in prior["lanes"]
        for cell_name, cell in lane["cells"].items()
        if cell["execution_status"] == "FAIL"
    ]
    if len(targets) != 24:
        raise RuntimeError(f"expected 24 frozen failures, found {len(targets)}")

    results = []
    with ThreadPoolExecutor(max_workers=4) as executor:
        futures = {
            executor.submit(replay, lane, cell_name, prior_cell): (
                lane.lane_id,
                cell_name,
            )
            for lane, cell_name, prior_cell in targets
        }
        for future in as_completed(futures):
            result = future.result()
            results.append(result)
            print(
                f"{result['lane_id']}/{result['cell']}: "
                f"{result['classification']} day {result['failure_day_index']}"
            )
    results.sort(key=lambda row: (row["lane_id"], row["cell"]))
    report = {
        "schema": "snow-surface-eb04a-diagnostic-replay-v1",
        "evidence_class": "Ran",
        "git_head": git_head(),
        "executable_source_diff_sha256": command_sha256(
            ["git", "diff", "--binary", "--", "crates", "tests"]
        ),
        "binary": relative(BINARY),
        "binary_sha256": sha256(BINARY),
        "source_eb04_results": relative(EB04_RESULTS),
        "source_eb04_results_sha256": sha256(EB04_RESULTS),
        "target_count": len(targets),
        "all_attempted": len(results) == 24,
        "all_fail_closed": all(row["returncode"] != 0 for row in results),
        "all_classified": all(row["classification"] != "UNCLASSIFIED" for row in results),
        "all_day_identities_match": all(
            row["failure_day_index"] == row["eb04_failure_day_index"] for row in results
        ),
        "all_snapshots_complete": all(
            row["snapshot_has_complete_state"] for row in results
        ),
        "maximum_abs_mass_residual_m": max(
            row["audit"]["maximum_abs_mass_residual_m"] for row in results
        ),
        "maximum_abs_surface_reconstruction_residual_j_m2": max(
            row["audit"]["maximum_abs_surface_reconstruction_residual_j_m2"]
            for row in results
        ),
        "maximum_abs_latent_reconstruction_residual_j_m2": max(
            row["audit"]["maximum_abs_latent_reconstruction_residual_j_m2"]
            for row in results
        ),
        "maximum_latent_reconstruction_allowance_ratio": max(
            row["audit"]["maximum_latent_reconstruction_allowance_ratio"]
            for row in results
        ),
        "maximum_abs_producer_latent_mass_residual_j_m2": max(
            row["audit"]["maximum_abs_producer_latent_mass_residual_j_m2"]
            for row in results
        ),
        "maximum_abs_daily_latent_hourly_residual_j_m2": max(
            row["audit"]["maximum_abs_daily_latent_hourly_residual_j_m2"]
            for row in results
        ),
        "maximum_daily_latent_hourly_allowance_ratio": max(
            row["audit"]["maximum_daily_latent_hourly_allowance_ratio"]
            for row in results
        ),
        "maximum_abs_vapor_mass_hourly_residual_kg_m2": max(
            row["audit"]["maximum_abs_vapor_mass_hourly_residual_kg_m2"]
            for row in results
        ),
        "maximum_abs_vapor_mass_sublimation_residual_kg_m2": max(
            row["audit"]["maximum_abs_vapor_mass_sublimation_residual_kg_m2"]
            for row in results
        ),
        "maximum_abs_shortwave_hourly_residual_j_m2": max(
            row["audit"]["maximum_abs_shortwave_hourly_residual_j_m2"]
            for row in results
        ),
        "maximum_abs_longwave_hourly_residual_j_m2": max(
            row["audit"]["maximum_abs_longwave_hourly_residual_j_m2"]
            for row in results
        ),
        "results": results,
    }
    report["acceptance_passes"] = (
        report["all_attempted"]
        and report["all_fail_closed"]
        and report["all_classified"]
        and report["all_day_identities_match"]
        and report["all_snapshots_complete"]
        and report["maximum_abs_mass_residual_m"] <= 1.0e-9
        and report["maximum_abs_surface_reconstruction_residual_j_m2"] <= 1.0e-6
        and report["maximum_latent_reconstruction_allowance_ratio"] <= 1.0
        and report["maximum_abs_producer_latent_mass_residual_j_m2"] <= 1.0e-6
        and report["maximum_daily_latent_hourly_allowance_ratio"] <= 1.0
        and report["maximum_abs_vapor_mass_hourly_residual_kg_m2"] <= 1.0e-9
        and report["maximum_abs_vapor_mass_sublimation_residual_kg_m2"] <= 1.0e-6
        and report["maximum_abs_shortwave_hourly_residual_j_m2"] <= 1.0e-6
        and report["maximum_abs_longwave_hourly_residual_j_m2"] <= 1.0e-6
    )
    (ARTIFACTS / "diagnostic-replay.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    write_csv(results)
    make_figures(results)
    write_sidecars(report)
    print(json.dumps({key: report[key] for key in report if key.startswith("all_")
                      or key.startswith("maximum_")
                      or key == "acceptance_passes"}, indent=2))
    return 0 if report["acceptance_passes"] else 1


def replay(lane: Any, cell: str, prior_cell: dict[str, Any]) -> dict[str, Any]:
    run_dir = OUTPUT / "runs" / lane.lane_id / cell
    run_dir.mkdir(parents=True, exist_ok=True)
    run_id = f"{lane.lane_id}-{cell}-eb04a"
    runfile = run_dir / f"{run_id}.run"
    trace = run_dir / f"{run_id}.snow.jsonl"
    stem = EB04_RUNNER.observed_harness.discover_run_stem(lane.fixture_dir)
    EB04_RUNNER.observed_harness.write_runfile(
        runfile, lane.fixture_dir, stem, run_dir, run_id
    )
    command = EB04_RUNNER.observed_harness.cli_command(
        BINARY, lane.fixture_dir, runfile, run_dir, "direct-production-executor"
    )
    longwave, sublimation = EB04_RUNNER.CELLS[cell]
    environment = os.environ.copy()
    environment.update(EB04_RUNNER.NON_TARGET_ENV)
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
    failure = completed.stderr.strip().splitlines()[-1] if completed.stderr.strip() else ""
    classification = classify(failure)
    return {
        "lane_id": lane.lane_id,
        "cell": cell,
        "classification": classification,
        "returncode": completed.returncode,
        "failure_day_index": int(rows[-1]["day_index"]) + 1 if rows else None,
        "eb04_failure_day_index": prior_cell["failure_day_index"],
        "trace_row_count": len(rows),
        "trace": relative(trace),
        "trace_sha256": sha256(trace),
        "fixture": relative(lane.fixture_dir),
        "fixture_sha256": EB04_RUNNER.tree_sha256(lane.fixture_dir),
        "selectors": {"longwave": longwave, "sublimation": sublimation},
        "typed_snapshot": failure,
        "snapshot_has_complete_state": snapshot_is_complete(failure, classification),
        "audit": audit(rows),
    }


def classify(failure: str) -> str:
    if "snow Stage 3 conductivity evaluation failed:" in failure:
        if "must be above absolute zero" in failure:
            return "CONDUCTIVITY_TEMPERATURE_BELOW_ABSOLUTE_ZERO"
        if "pressure_pa must be > 0; received 0" in failure:
            return "SATURATION_VAPOR_PRESSURE_UNDERFLOW"
        return "CONDUCTIVITY_OTHER"
    if "snow layer aggregate prior_layers." in failure:
        return "PRIOR_LAYER_THICKNESS_AGGREGATE_MISMATCH"
    return "UNCLASSIFIED"


def snapshot_is_complete(failure: str, classification: str) -> bool:
    number = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
    if classification in {
        "CONDUCTIVITY_TEMPERATURE_BELOW_ABSOLUTE_ZERO",
        "SATURATION_VAPOR_PRESSURE_UNDERFLOW",
        "CONDUCTIVITY_OTHER",
    }:
        required = [
            rf"layer_density_kg_m3=({number})",
            rf"control_volume_temperature_c=({number})",
            rf"atmospheric_pressure_pa=({number})",
            r"control_volume_layers=\[DirectSnowLayerState \{",
        ]
        return all(re.search(pattern, failure) for pattern in required)
    if classification != "PRIOR_LAYER_THICKNESS_AGGREGATE_MISMATCH":
        return False
    header = re.search(
        rf"prior_layers\.thickness_m=({number}) does not match expected ({number}); "
        rf"prior_swe_m=({number}), prior_depth_m=({number}), prior_layers=\[(.*)\]$",
        failure,
    )
    if header is None:
        return False
    value, expected, prior_swe, prior_depth = map(float, header.groups()[:4])
    layers = header.group(5)
    layer_pairs = [
        (float(mass), float(thickness))
        for mass, thickness in re.findall(
            rf"mass_swe_m: ({number}), thickness_m: ({number})", layers
        )
    ]
    retained_pairs = [pair for pair in layer_pairs if pair[0] > 1.0e-9]
    return (
        bool(layer_pairs)
        and abs(math.fsum(pair[1] for pair in retained_pairs) - value) <= 1.0e-12
        and abs(math.fsum(pair[0] for pair in retained_pairs) - prior_swe) <= 1.0e-9
        and expected == prior_depth
    )


def audit(rows: list[dict[str, Any]]) -> dict[str, float]:
    mass = []
    surface = []
    latent = []
    latent_allowance_ratios = []
    producer_latent = []
    daily_latent_hourly = []
    daily_latent_hourly_ratios = []
    vapor_hourly = []
    vapor_sublimation = []
    shortwave_hourly = []
    longwave_hourly = []
    for row in rows:
        mass.append(
            row["runtime_swe_before_m"]
            + row["accumulation_m"]
            + row["rain_retained_m"]
            - row["sublimation_m"]
            - row["snowpack_swe_loss_m"]
            - row["runtime_swe_after_m"]
        )
        if row.get("stage3_energy_enabled"):
            hourly_mass = row["stage3_hourly_vapor_mass_exchange_kg_m2"]
            hourly_latent_heat = row["stage3_hourly_latent_heat_j_kg"]
            hourly_latent_flux = row["stage3_hourly_latent_flux_w_m2"]
            if (
                len(hourly_mass) != 24
                or len(hourly_latent_heat) != 24
                or len(hourly_latent_flux) != 24
            ):
                raise RuntimeError("Stage 3 hourly latent operands must contain 24 values")
            surface.append(
                row["stage3_shortwave_energy_j_m2"]
                + row["stage3_longwave_energy_j_m2"]
                + row["stage3_latent_energy_j_m2"]
                - row["stage3_surface_energy_j_m2"]
                - row["stage3_unused_positive_energy_j_m2"]
            )
            latent_residual = math.fsum(
                    latent_flux * 3_600.0 - mass * latent_heat
                    for mass, latent_heat, latent_flux in zip(
                        hourly_mass,
                        hourly_latent_heat,
                        hourly_latent_flux,
                        strict=True,
                    )
                )
            latent.append(latent_residual)
            latent_roundoff_bound = 8.0 * math.fsum(
                math.ulp(max(abs(latent_flux * 3_600.0), abs(mass * latent_heat)))
                for mass, latent_heat, latent_flux in zip(
                    hourly_mass,
                    hourly_latent_heat,
                    hourly_latent_flux,
                    strict=True,
                )
            )
            latent_roundoff_bound = max(
                latent_roundoff_bound,
                16.0
                * sys.float_info.epsilon
                * math.fsum(
                    abs(latent_flux * 3_600.0) + abs(mass * latent_heat)
                    for mass, latent_heat, latent_flux in zip(
                        hourly_mass,
                        hourly_latent_heat,
                        hourly_latent_flux,
                        strict=True,
                    )
                ),
            )
            latent_allowance = max(1.0e-6, latent_roundoff_bound)
            latent_allowance_ratios.append(abs(latent_residual) / latent_allowance)
            producer_latent.append(
                row["stage3_mass_latent_identity_residual_j_m2"]
            )
            hourly_latent_total = math.fsum(
                latent_flux * 3_600.0 for latent_flux in hourly_latent_flux
            )
            daily_hourly_residual = (
                row["stage3_latent_energy_j_m2"] - hourly_latent_total
            )
            daily_latent_hourly.append(daily_hourly_residual)
            daily_hourly_allowance = max(
                1.0e-6,
                16.0
                * sys.float_info.epsilon
                * (
                    abs(row["stage3_latent_energy_j_m2"])
                    + math.fsum(
                        abs(latent_flux * 3_600.0)
                        for latent_flux in hourly_latent_flux
                    )
                ),
            )
            daily_latent_hourly_ratios.append(
                abs(daily_hourly_residual) / daily_hourly_allowance
            )
            vapor_hourly.append(
                row["stage3_vapor_mass_exchange_kg_m2"] - math.fsum(hourly_mass)
            )
            vapor_sublimation.append(
                row["stage3_vapor_mass_exchange_kg_m2"]
                + 1_000.0 * row["sublimation_m"]
            )
            shortwave_hourly.append(
                row["stage3_shortwave_energy_j_m2"]
                - 3_600.0 * math.fsum(row["stage3_hourly_net_shortwave_w_m2"])
            )
            longwave_hourly.append(
                row["stage3_longwave_energy_j_m2"]
                - 3_600.0 * math.fsum(row["stage3_hourly_net_longwave_w_m2"])
            )
    return {
        "maximum_abs_mass_residual_m": max(map(abs, mass), default=0.0),
        "maximum_abs_surface_reconstruction_residual_j_m2": max(
            map(abs, surface), default=0.0
        ),
        "maximum_abs_latent_reconstruction_residual_j_m2": max(
            map(abs, latent), default=0.0
        ),
        "maximum_latent_reconstruction_allowance_ratio": max(
            latent_allowance_ratios, default=0.0
        ),
        "maximum_abs_producer_latent_mass_residual_j_m2": max(
            map(abs, producer_latent), default=0.0
        ),
        "maximum_abs_daily_latent_hourly_residual_j_m2": max(
            map(abs, daily_latent_hourly), default=0.0
        ),
        "maximum_daily_latent_hourly_allowance_ratio": max(
            daily_latent_hourly_ratios, default=0.0
        ),
        "maximum_abs_vapor_mass_hourly_residual_kg_m2": max(
            map(abs, vapor_hourly), default=0.0
        ),
        "maximum_abs_vapor_mass_sublimation_residual_kg_m2": max(
            map(abs, vapor_sublimation), default=0.0
        ),
        "maximum_abs_shortwave_hourly_residual_j_m2": max(
            map(abs, shortwave_hourly), default=0.0
        ),
        "maximum_abs_longwave_hourly_residual_j_m2": max(
            map(abs, longwave_hourly), default=0.0
        ),
    }


def self_check() -> None:
    mass = [-0.003, -0.001]
    latent_heat = [2_840_000.0, 2_810_000.0]
    latent_flux = [
        mass_value * heat_value / 3_600.0
        for mass_value, heat_value in zip(mass, latent_heat, strict=True)
    ]
    correct = math.fsum(
        flux * 3_600.0 - mass_value * heat_value
        for flux, mass_value, heat_value in zip(
            latent_flux, mass, latent_heat, strict=True
        )
    )
    wrong_sign = math.fsum(
        flux * 3_600.0 + mass_value * heat_value
        for flux, mass_value, heat_value in zip(
            latent_flux, mass, latent_heat, strict=True
        )
    )
    wrong_column = math.fsum(
        flux * 3_600.0 - mass_value * heat_value
        for flux, mass_value, heat_value in zip(
            latent_flux, mass, reversed(latent_heat), strict=True
        )
    )
    if abs(correct) > 1.0e-9 or abs(wrong_sign) < 1.0 or abs(wrong_column) < 1.0:
        raise RuntimeError("latent reconstruction anti-alias self-check failed")


def write_csv(results: list[dict[str, Any]]) -> None:
    import csv

    path = ARTIFACTS / "failure-classification.csv"
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.writer(stream, lineterminator="\n")
        writer.writerow(
            [
                "lane_id",
                "cell",
                "classification",
                "failure_day_index",
                "trace_row_count",
                "snapshot_has_complete_state",
            ]
        )
        for row in results:
            writer.writerow(
                [
                    row["lane_id"],
                    row["cell"],
                    row["classification"],
                    row["failure_day_index"],
                    row["trace_row_count"],
                    row["snapshot_has_complete_state"],
                ]
            )


def make_figures(results: list[dict[str, Any]]) -> None:
    import matplotlib

    matplotlib.use("Agg")
    matplotlib.rcParams["svg.hashsalt"] = "snow-surface-eb04a"
    import matplotlib.pyplot as plt

    colors = {
        "CONDUCTIVITY_TEMPERATURE_BELOW_ABSOLUTE_ZERO": "#b23a48",
        "SATURATION_VAPOR_PRESSURE_UNDERFLOW": "#e07a5f",
        "PRIOR_LAYER_THICKNESS_AGGREGATE_MISMATCH": "#287271",
    }
    ordered = sorted(results, key=lambda row: row["failure_day_index"])
    fig, ax = plt.subplots(figsize=(10.5, 6.8))
    for index, row in enumerate(ordered):
        ax.scatter(
            row["failure_day_index"],
            index,
            color=colors[row["classification"]],
            s=48,
            zorder=3,
        )
    ax.set_yticks(range(len(ordered)), [f"{r['lane_id']} / {r['cell']}" for r in ordered])
    ax.set_xscale("log")
    ax.set_xlabel("Rejected model day (log scale)")
    ax.set_title("EB-04A reproduces every original rejection on the same day")
    ax.grid(axis="x", alpha=0.25, zorder=0)
    handles = [
        plt.Line2D([], [], marker="o", linestyle="", color=color, label=label)
        for label, color in [
            ("Below-absolute-zero thermal state", colors["CONDUCTIVITY_TEMPERATURE_BELOW_ABSOLUTE_ZERO"]),
            ("Saturation-vapor-pressure underflow", colors["SATURATION_VAPOR_PRESSURE_UNDERFLOW"]),
            ("Layer-depth reconciliation", colors["PRIOR_LAYER_THICKNESS_AGGREGATE_MISMATCH"]),
        ]
    ]
    ax.legend(handles=handles, loc="lower right")
    fig.tight_layout()
    chronology_path = FIGURES / "eb04a-failure-chronology.svg"
    fig.savefig(chronology_path, metadata={"Date": None})
    plt.close(fig)
    normalize_text_output(chronology_path)

    fig, ax = plt.subplots(figsize=(9.5, 5.8))
    labels = [
        "Mass ledger\n(m)",
        "Surface components\n(J m$^{-2}$)",
        "Latent–mass identity\n(J m$^{-2}$)",
    ]
    values = [
        max(r["audit"]["maximum_abs_mass_residual_m"] for r in results),
        max(
            r["audit"]["maximum_abs_surface_reconstruction_residual_j_m2"]
            for r in results
        ),
        max(
            r["audit"]["maximum_abs_latent_reconstruction_residual_j_m2"]
            for r in results
        ),
    ]
    shown = [max(value, 1.0e-16) for value in values]
    ax.bar(labels, shown, color=["#457b9d", "#e9c46a", "#6a4c93"], zorder=3)
    ax.set_yscale("log")
    ax.set_ylabel("Maximum absolute residual (log scale)")
    ax.set_title("Published operands independently close before rejection")
    ax.grid(axis="y", alpha=0.25, zorder=0)
    for index, (height, value) in enumerate(zip(shown, values, strict=True)):
        ax.text(index, height * 1.4, f"{value:.3e}", ha="center", va="bottom")
    fig.tight_layout()
    ledger_path = FIGURES / "eb04a-ledger-closure.svg"
    fig.savefig(ledger_path, metadata={"Date": None})
    plt.close(fig)
    normalize_text_output(ledger_path)


def normalize_text_output(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    cleaned = "\n".join(line.rstrip() for line in text.splitlines()).rstrip() + "\n"
    path.write_text(cleaned, encoding="utf-8")


def write_sidecars(report: dict[str, Any]) -> None:
    (FIGURES / "eb04a-failure-chronology.md").write_text(
        """# EB-04A Failure Chronology

## Caption

Targeted replays of the 24 EB-04 failed lane/cell combinations. Each point is
the first rejected model day; horizontal position uses a logarithmic scale so
early and late failures remain readable. Red denotes a physically impossible
below-absolute-zero thermal state reaching the SNOBAL conductivity primitive;
green denotes a prior-layer depth aggregate mismatch.

## How To Read This Figure

Rows identify the validation lane and factorial cell. The matching rejection
day demonstrates deterministic reproduction, not a new factorial result.
Orange identifies a second extreme-cold signature: saturation vapor pressure
underflows to zero before conductivity can be evaluated. The wide chronology
shows that the instability is accumulated-state dependent rather than a
single startup defect.

## Provenance And Limits

Generated by `tools/run_diagnostics.py` from the current diagnostic binary and
the frozen EB-04 failure list. These runs do not score observations, alter
physics, or authorize a correction. Exact executable, fixture, selector,
failure, and trace hashes are in `../diagnostic-replay.json`.
        """.strip() + "\n",
        encoding="utf-8",
    )
    (FIGURES / "eb04a-ledger-closure.md").write_text(
        f"""# EB-04A Ledger Closure

## Caption

Worst independently reconstructed residual across all successful daily trace
rows preceding the 24 rejected steps: mass
`{report['maximum_abs_mass_residual_m']:.3e} m`, surface-component energy
`{report['maximum_abs_surface_reconstruction_residual_j_m2']:.3e} J m^-2`,
and latent/mass identity
`{report['maximum_abs_latent_reconstruction_residual_j_m2']:.3e} J m^-2`.

## How To Read This Figure

Lower bars are tighter closure. The axes are logarithmic; exact zeros are
drawn at a tiny display floor while their labels retain the computed values.
The surface reconstruction uses published shortwave, longwave, latent,
applied-energy, and unused-positive-energy operands. The latent check compares
published latent energy with the separately published signed-mass conversion.

## Provenance And Limits

Generated by `tools/run_diagnostics.py`. Passing pre-rejection ledgers show
that the new publication surface is coherent; they do not make the rejected
thermal state physically admissible and do not identify its corrective
equation.
        """.strip() + "\n",
        encoding="utf-8",
    )


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.is_file():
        return []
    return [
        json.loads(line)
        for line in path.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def command_sha256(command: list[str]) -> str:
    completed = subprocess.run(
        command, cwd=REPO, check=True, stdout=subprocess.PIPE
    )
    return hashlib.sha256(completed.stdout).hexdigest()


def git_head() -> str:
    return subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=REPO,
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    ).stdout.strip()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO))


if __name__ == "__main__":
    raise SystemExit(main())
