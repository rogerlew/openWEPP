#!/usr/bin/env python3
"""Adjudicate opt-in phase partition impact on coupled WAT snow depth.

This is SNOWDENSITY-10.3.5c evidence tooling. It runs the real
``openwepp-cli-hill --direct-production-executor`` WAT path for the 10.3.4
maritime snow-depth surfaces with the default legacy RST partition and with the
10.3.5b opt-in Harder-Pomeroy hourly partition.
"""

from __future__ import annotations

import argparse
import contextlib
import datetime as dt
import json
import os
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterator


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import observed_harness  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402


SCHEMA = "snowdensity10-3-5c-phase-partition-snowdepth-impact-v1"
CONTRACT = (
    "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047 INV-SNOWFREEZE-048 "
    "INV-SNOWFREEZE-050 INV-SNOWFREEZE-065"
)
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-5c-phase-partition-snow-depth-impact-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_5c_phase_partition_snowdepth_impact"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
PHASE_ENV = "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL"
DEFAULT_MODEL = "legacy_rst"
OPT_IN_MODEL = "harder_pomeroy_hourly"
DEPTH_DELTA_EPS_M = 1.0e-9


@dataclass(frozen=True)
class Surface:
    surface_id: str
    site_group: str
    fixture_dir: Path
    cover: str
    observation_source: str
    observation_file: Path | None
    observation_filter: dict[str, str]
    observation_kind: str
    verdict_scope: str
    note: str


SURFACES = [
    Surface(
        surface_id="hjandrews_conifer",
        site_group="hjandrews",
        fixture_dir=REPO_ROOT / "tests/fixtures/cancov_forest/hjandrews_conifer_or",
        cover="conifer",
        observation_source="not_installed",
        observation_file=None,
        observation_filter={},
        observation_kind="none",
        verdict_scope="observation_blocked",
        note="Fixture exists, but paired snow-depth observations are not installed.",
    ),
    Surface(
        surface_id="sleepers_south_field",
        site_group="sleepers",
        fixture_dir=REPO_ROOT / "tests/fixtures/snowfreeze_observed/site1_sleepers_south_field_vt",
        cover="open_field",
        observation_source="snowfreeze_observed",
        observation_file=REPO_ROOT
        / "tests/fixtures/snowfreeze_observed/observations/sites/site1_sleepers_south_field_vt.csv",
        observation_filter={},
        observation_kind="snowfreeze_depth",
        verdict_scope="paired_observation",
        note="Sleepers South field paired frost-site snow-depth rows.",
    ),
    Surface(
        surface_id="sleepers_w9_hardwood",
        site_group="sleepers",
        fixture_dir=REPO_ROOT / "tests/fixtures/snowfreeze_observed/site2_sleepers_w9_hardwood_vt",
        cover="hardwood",
        observation_source="snowfreeze_observed",
        observation_file=REPO_ROOT
        / "tests/fixtures/snowfreeze_observed/observations/sites/site2_sleepers_w9_hardwood_vt.csv",
        observation_filter={},
        observation_kind="snowfreeze_depth",
        verdict_scope="paired_observation",
        note="Sleepers W9 hardwood paired frost-site snow-depth rows.",
    ),
    Surface(
        surface_id="harvard_hardwood",
        site_group="harvard",
        fixture_dir=REPO_ROOT / "tests/fixtures/cancov_forest/harvard_deciduous_ma",
        cover="hardwood",
        observation_source="harvard_hf237",
        observation_file=REPO_ROOT
        / "tests/fixtures/cancov_forest/observations/sites/harvard_hf237_strata.csv",
        observation_filter={
            "binding_status": "bound",
            "model_fixture": "harvard_deciduous_ma",
            "observed_stratum": "hardwood",
        },
        observation_kind="swe_depth_density",
        verdict_scope="paired_observation",
        note="Harvard HF237 hardwood stratum bound by 10.3.2.",
    ),
    Surface(
        surface_id="harvard_open",
        site_group="harvard",
        fixture_dir=REPO_ROOT / "tests/fixtures/cancov_forest/harvard_open_ma",
        cover="open",
        observation_source="harvard_hf237",
        observation_file=REPO_ROOT
        / "tests/fixtures/cancov_forest/observations/sites/harvard_hf237_strata.csv",
        observation_filter={
            "binding_status": "bound",
            "model_fixture": "harvard_open_ma",
            "observed_stratum": "open",
        },
        observation_kind="swe_depth_density",
        verdict_scope="paired_observation",
        note="Harvard HF237 open stratum bound by 10.3.2.",
    ),
    Surface(
        surface_id="hubbardbrook_deciduous",
        site_group="hubbardbrook",
        fixture_dir=REPO_ROOT / "tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh",
        cover="deciduous",
        observation_source="not_installed",
        observation_file=None,
        observation_filter={},
        observation_kind="none",
        verdict_scope="observation_blocked",
        note="Fixture exists, but paired snow-depth observations are not installed.",
    ),
    Surface(
        surface_id="hubbardbrook_mixed",
        site_group="hubbardbrook",
        fixture_dir=REPO_ROOT / "tests/fixtures/cancov_forest/hubbardbrook_mixed_nh",
        cover="mixed",
        observation_source="not_installed",
        observation_file=None,
        observation_filter={},
        observation_kind="none",
        verdict_scope="observation_blocked",
        note="Fixture exists, but paired snow-depth observations are not installed.",
    ),
]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--skip-model-runs", action="store_true")
    args = parser.parse_args(argv)

    report = adjudicate(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        hill_binary=args.hill_binary.resolve(),
        run_models=not args.skip_model_runs,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def adjudicate(
    output_dir: Path,
    package_artifacts_dir: Path,
    hill_binary: Path,
    run_models: bool,
) -> dict[str, Any]:
    if run_models and not hill_binary.is_file():
        raise FileNotFoundError(f"openwepp-cli-hill binary not found: {hill_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    surfaces = []
    for surface in SURFACES:
        default_result = run_and_analyze(surface, output_dir, hill_binary, DEFAULT_MODEL, run_models)
        opt_in_result = run_and_analyze(surface, output_dir, hill_binary, OPT_IN_MODEL, run_models)
        surfaces.append(build_surface_report(surface, default_result, opt_in_result))

    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "runtime_coupling": "real direct-production WAT via openwepp-cli-hill",
        "diagnostic_selector": {
            "env": PHASE_ENV,
            "default_behavior": "absent selector -> legacy_rst",
            "opt_in_value": OPT_IN_MODEL,
        },
        "protected_boundaries": {
            "opt_in_solver_robustness_changed": True,
            "production_physics_outside_existing_opt_in_selector_changed": False,
            "default_activation_changed": False,
            "parser_runfile_user_cli_selector_added": False,
            "fixture_inputs_changed": False,
            "public_output_schema_changed": False,
            "site_calibration_performed": False,
        },
        "summary": summarize(surfaces),
        "surfaces": surfaces,
        "raw_outputs": {
            "output_dir": rel(output_dir),
            "package_json": rel(package_artifacts_dir / "phase-partition-snowdepth-impact.json"),
            "package_markdown": rel(package_artifacts_dir / "phase-partition-snowdepth-impact.md"),
        },
    }
    rubric.write_json(package_artifacts_dir / "phase-partition-snowdepth-impact.json", report)
    (package_artifacts_dir / "phase-partition-snowdepth-impact.md").write_text(
        render_markdown(report), encoding="utf-8"
    )
    rubric.write_json(output_dir / "phase-partition-snowdepth-impact.json", report)
    (output_dir / "phase-partition-snowdepth-impact.md").write_text(
        render_markdown(report), encoding="utf-8"
    )
    return report


def run_and_analyze(
    surface: Surface,
    output_dir: Path,
    hill_binary: Path,
    model: str,
    run_model: bool,
) -> dict[str, Any]:
    run_dir = output_dir / "runs" / surface.surface_id / model
    run_dir.mkdir(parents=True, exist_ok=True)
    run_stem = observed_harness.discover_run_stem(surface.fixture_dir)
    run_id = f"{surface.surface_id}_{model}"
    runfile_path = run_dir / f"{run_id}.run"
    observed_harness.write_runfile(runfile_path, surface.fixture_dir, run_stem, run_dir, run_id)
    command = observed_harness.cli_command(
        hill_binary,
        surface.fixture_dir,
        runfile_path,
        run_dir,
        "direct-production-executor",
    )
    env_value = None if model == DEFAULT_MODEL else OPT_IN_MODEL
    if run_model:
        with scoped_env({PHASE_ENV: env_value}):
            completed = subprocess.run(
                command,
                cwd=REPO_ROOT,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
            )
        (run_dir / "openwepp-cli-hill.stdout").write_text(completed.stdout, encoding="utf-8")
        (run_dir / "openwepp-cli-hill.stderr").write_text(completed.stderr, encoding="utf-8")
        if completed.returncode != 0:
            raise RuntimeError(
                f"openwepp-cli-hill failed for {surface.surface_id}/{model} "
                f"with exit code {completed.returncode}; see {run_dir}"
            )
    wat_path = run_dir / f"{run_id}.wat.parquet"
    if not wat_path.is_file():
        raise FileNotFoundError(f"expected WAT parquet output {wat_path}")
    modeled = observed_harness.load_modeled_wat(wat_path)
    observations = load_observations(surface)
    pairs = pair_observations(observations, modeled, surface.observation_kind)
    return {
        "model": model,
        "env": None if model == DEFAULT_MODEL else {PHASE_ENV: OPT_IN_MODEL},
        "command": command,
        "run_dir": rel(run_dir),
        "runfile": rel(runfile_path),
        "wat": rel(wat_path),
        "modeled": modeled,
        "observation_row_count": len(observations),
        "paired_row_count": len(pairs),
        "residuals": residual_summary(pairs),
        "wat_summary": wat_summary(modeled),
    }


@contextlib.contextmanager
def scoped_env(updates: dict[str, str | None]) -> Iterator[None]:
    previous = {key: os.environ.get(key) for key in updates}
    try:
        for key, value in updates.items():
            if value is None:
                os.environ.pop(key, None)
            else:
                os.environ[key] = value
        yield
    finally:
        for key, value in previous.items():
            if value is None:
                os.environ.pop(key, None)
            else:
                os.environ[key] = value


def build_surface_report(
    surface: Surface, default_result: dict[str, Any], opt_in_result: dict[str, Any]
) -> dict[str, Any]:
    modeled_default = default_result.pop("modeled")
    modeled_opt_in = opt_in_result.pop("modeled")
    deltas = wat_deltas(modeled_default, modeled_opt_in)
    paired_delta = paired_residual_delta(default_result["residuals"], opt_in_result["residuals"])
    return {
        "surface_id": surface.surface_id,
        "site_group": surface.site_group,
        "cover": surface.cover,
        "fixture_dir": rel(surface.fixture_dir),
        "verdict_scope": surface.verdict_scope,
        "observation_source": surface.observation_source,
        "observation_kind": surface.observation_kind,
        "note": surface.note,
        "default": sanitize_result(default_result),
        "opt_in": sanitize_result(opt_in_result),
        "wat_deltas": deltas,
        "paired_residual_delta": paired_delta,
        "impact_class": classify_surface_impact(surface, paired_delta, deltas),
    }


def sanitize_result(result: dict[str, Any]) -> dict[str, Any]:
    sanitized = dict(result)
    sanitized["command"] = [str(value) for value in sanitized["command"]]
    return sanitized


def load_observations(surface: Surface) -> list[dict[str, str]]:
    if surface.observation_file is None:
        return []
    rows = rubric.read_csv_dicts(surface.observation_file)
    if not surface.observation_filter:
        return rows
    return [
        row
        for row in rows
        if all(row.get(key) == value for key, value in surface.observation_filter.items())
    ]


def pair_observations(
    observations: list[dict[str, str]],
    modeled: dict[dt.date, dict[str, float | None]],
    observation_kind: str,
) -> list[dict[str, Any]]:
    pairs = []
    for row in observations:
        date = dt.date.fromisoformat(row["date"])
        modeled_row = modeled.get(date)
        if modeled_row is None:
            continue
        observed_depth = rubric.optional_float(row.get("observed_snow_depth_m"))
        modeled_depth = modeled_row.get("snow_depth_m")
        if observed_depth is None or modeled_depth is None:
            continue
        pair: dict[str, Any] = {
            "date": date.isoformat(),
            "water_year": rubric.water_year(date),
            "observed_snow_depth_m": observed_depth,
            "modeled_snow_depth_m": modeled_depth,
            "depth_residual_m": modeled_depth - observed_depth,
            "modeled_snow_water_m": modeled_row.get("snow_water_m"),
        }
        if observation_kind == "swe_depth_density":
            observed_swe_mm = rubric.optional_float(row.get("observed_swe_mm"))
            observed_density = rubric.optional_float(row.get("observed_density_kg_m3"))
            pair["observed_swe_m"] = observed_swe_mm / 1000.0 if observed_swe_mm is not None else None
            pair["observed_density_kg_m3"] = observed_density
            modeled_swe = modeled_row.get("snow_water_m")
            pair["swe_residual_m"] = (
                modeled_swe - pair["observed_swe_m"]
                if modeled_swe is not None and pair["observed_swe_m"] is not None
                else None
            )
        pairs.append(pair)
    return pairs


def residual_summary(pairs: list[dict[str, Any]]) -> dict[str, Any]:
    residuals = [row["depth_residual_m"] for row in pairs]
    if not residuals:
        return {
            "paired_count": 0,
            "status": "NO_PAIRED_OBSERVED_SNOW_DEPTH",
            "mean_signed_depth_residual_m": None,
            "mean_abs_depth_residual_m": None,
            "max_abs_depth_residual_m": None,
            "modeled_over_observed_fraction": None,
            "snow_control_fail_count": 0,
            "snow_control_fail_fraction": None,
            "sample_pairs": [],
        }
    fail_count = sum(
        1
        for row in pairs
        if abs(row["depth_residual_m"]) > rubric.snow_depth_tolerance(row["observed_snow_depth_m"])
    )
    over_count = sum(1 for value in residuals if value > 0.0)
    return {
        "paired_count": len(residuals),
        "status": "PAIRED_OBSERVED_SNOW_DEPTH",
        "mean_signed_depth_residual_m": mean(residuals),
        "mean_abs_depth_residual_m": mean([abs(value) for value in residuals]),
        "max_abs_depth_residual_m": max(abs(value) for value in residuals),
        "modeled_over_observed_fraction": over_count / len(residuals),
        "snow_control_fail_count": fail_count,
        "snow_control_fail_fraction": fail_count / len(residuals),
        "sample_pairs": pairs[:12],
    }


def paired_residual_delta(default: dict[str, Any], opt_in: dict[str, Any]) -> dict[str, Any]:
    if default["paired_count"] == 0 or opt_in["paired_count"] == 0:
        return {
            "paired_count": 0,
            "fail_count_delta_default_minus_opt_in": None,
            "mean_abs_depth_reduction_m": None,
            "mean_signed_depth_change_opt_in_minus_default_m": None,
        }
    return {
        "paired_count": min(default["paired_count"], opt_in["paired_count"]),
        "fail_count_delta_default_minus_opt_in": (
            default["snow_control_fail_count"] - opt_in["snow_control_fail_count"]
        ),
        "fail_fraction_delta_default_minus_opt_in": (
            default["snow_control_fail_fraction"] - opt_in["snow_control_fail_fraction"]
        ),
        "mean_abs_depth_reduction_m": (
            default["mean_abs_depth_residual_m"] - opt_in["mean_abs_depth_residual_m"]
        ),
        "mean_signed_depth_change_opt_in_minus_default_m": (
            opt_in["mean_signed_depth_residual_m"] - default["mean_signed_depth_residual_m"]
        ),
    }


def wat_summary(modeled: dict[dt.date, dict[str, float | None]]) -> dict[str, Any]:
    depth_values = finite_values(row.get("snow_depth_m") for row in modeled.values())
    swe_values = finite_values(row.get("snow_water_m") for row in modeled.values())
    return {
        "day_count": len(modeled),
        "snow_depth_day_count": len(depth_values),
        "peak_snow_depth_m": max(depth_values) if depth_values else None,
        "mean_snow_depth_m": mean(depth_values),
        "depth_day_sum_m_days": sum(depth_values) if depth_values else 0.0,
        "snow_water_day_count": len(swe_values),
        "peak_snow_water_m": max(swe_values) if swe_values else None,
        "mean_snow_water_m": mean(swe_values),
        "swe_day_sum_m_days": sum(swe_values) if swe_values else 0.0,
    }


def wat_deltas(
    default: dict[dt.date, dict[str, float | None]],
    opt_in: dict[dt.date, dict[str, float | None]],
) -> dict[str, Any]:
    shared_dates = sorted(set(default) & set(opt_in))
    depth_deltas = []
    swe_deltas = []
    changed_depth_days = 0
    changed_swe_days = 0
    for date in shared_dates:
        default_depth = default[date].get("snow_depth_m")
        opt_depth = opt_in[date].get("snow_depth_m")
        if default_depth is not None and opt_depth is not None:
            delta = opt_depth - default_depth
            depth_deltas.append(delta)
            if abs(delta) > DEPTH_DELTA_EPS_M:
                changed_depth_days += 1
        default_swe = default[date].get("snow_water_m")
        opt_swe = opt_in[date].get("snow_water_m")
        if default_swe is not None and opt_swe is not None:
            delta = opt_swe - default_swe
            swe_deltas.append(delta)
            if abs(delta) > DEPTH_DELTA_EPS_M:
                changed_swe_days += 1
    return {
        "shared_day_count": len(shared_dates),
        "changed_snow_depth_day_count": changed_depth_days,
        "changed_snow_water_day_count": changed_swe_days,
        "mean_snow_depth_delta_opt_in_minus_default_m": mean(depth_deltas),
        "mean_abs_snow_depth_delta_m": mean([abs(value) for value in depth_deltas]),
        "max_abs_snow_depth_delta_m": max_abs(depth_deltas),
        "depth_day_sum_delta_opt_in_minus_default_m_days": sum(depth_deltas),
        "mean_snow_water_delta_opt_in_minus_default_m": mean(swe_deltas),
        "mean_abs_snow_water_delta_m": mean([abs(value) for value in swe_deltas]),
        "max_abs_snow_water_delta_m": max_abs(swe_deltas),
        "swe_day_sum_delta_opt_in_minus_default_m_days": sum(swe_deltas),
    }


def classify_surface_impact(
    surface: Surface, paired_delta: dict[str, Any], deltas: dict[str, Any]
) -> str:
    if surface.verdict_scope != "paired_observation":
        return "OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY"
    fail_delta = paired_delta["fail_count_delta_default_minus_opt_in"]
    abs_reduction = paired_delta["mean_abs_depth_reduction_m"]
    if fail_delta is None or abs_reduction is None:
        return "NO-PAIRED-SNOW-DEPTH"
    if fail_delta > 0 and abs_reduction >= 0.0:
        return "IMPROVED"
    if fail_delta == 0 and abs_reduction > 0.0:
        return "MARGINAL-IMPROVEMENT"
    if fail_delta < 0 or abs_reduction < 0.0:
        return "WORSE"
    if deltas["changed_snow_depth_day_count"] > 0:
        return "CHANGED-NO-RESIDUAL-IMPROVEMENT"
    return "NO-WAT-CHANGE"


def summarize(surfaces: list[dict[str, Any]]) -> dict[str, Any]:
    paired = [item for item in surfaces if item["verdict_scope"] == "paired_observation"]
    blocked = [item for item in surfaces if item["verdict_scope"] != "paired_observation"]
    default_pairs = sum(item["default"]["residuals"]["paired_count"] for item in paired)
    opt_pairs = sum(item["opt_in"]["residuals"]["paired_count"] for item in paired)
    default_fail = sum(item["default"]["residuals"]["snow_control_fail_count"] for item in paired)
    opt_fail = sum(item["opt_in"]["residuals"]["snow_control_fail_count"] for item in paired)
    improved = [item for item in paired if item["impact_class"] == "IMPROVED"]
    marginal = [item for item in paired if item["impact_class"] == "MARGINAL-IMPROVEMENT"]
    worse = [item for item in paired if item["impact_class"] == "WORSE"]
    changed = [item for item in surfaces if item["wat_deltas"]["changed_snow_depth_day_count"] > 0]
    default_fail_fraction = default_fail / default_pairs if default_pairs else None
    opt_fail_fraction = opt_fail / opt_pairs if opt_pairs else None

    if not paired:
        disposition = "PHASE-PARTITION-HOLD"
        blocker = "NO-PAIRED-SNOW-DEPTH-SURFACES"
        next_route = "install paired snow-depth observations before further adjudication"
    elif default_fail > opt_fail and not worse and opt_fail_fraction is not None and opt_fail_fraction < 0.5:
        disposition = "PHASE-PARTITION-PROMOTION-CANDIDATE"
        blocker = None
        next_route = "scaffold promotion/activation gate with rollback and full SNOTEL/non-SNOTEL profiles"
    elif default_fail > opt_fail or improved or marginal:
        disposition = "PHASE-PARTITION-PARTIAL-IMPROVEMENT"
        blocker = "SNOW-CONTROL-NOT-CLEARED"
        next_route = "target 10.3.4 rank-2 winter-thaw melt response before longwave or rain-heat"
    else:
        disposition = "PHASE-PARTITION-NEUTRAL-OR-WORSE"
        blocker = "PHASE-PARTITION-DID-NOT-REDUCE-PAIRED-SNOW-DEPTH-FAILURES"
        next_route = "target 10.3.4 rank-2 winter-thaw melt response before longwave or rain-heat"

    return {
        "disposition": disposition,
        "blocker": blocker,
        "surface_count": len(surfaces),
        "paired_surface_count": len(paired),
        "observation_blocked_surface_count": len(blocked),
        "wat_changed_surface_count": len(changed),
        "paired_surface_improved_count": len(improved),
        "paired_surface_marginal_count": len(marginal),
        "paired_surface_worse_count": len(worse),
        "default_paired_row_count": default_pairs,
        "opt_in_paired_row_count": opt_pairs,
        "default_snow_control_fail_count": default_fail,
        "opt_in_snow_control_fail_count": opt_fail,
        "default_snow_control_fail_fraction": default_fail_fraction,
        "opt_in_snow_control_fail_fraction": opt_fail_fraction,
        "snow_control_fail_delta_default_minus_opt_in": default_fail - opt_fail,
        "observation_blocked_surface_ids": [item["surface_id"] for item in blocked],
        "next_route": next_route,
    }


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.5c Phase Partition Snow-Depth Impact",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Blocker: `{summary['blocker']}`",
        f"- Paired surfaces: `{summary['paired_surface_count']}`",
        f"- Observation-blocked surfaces: `{summary['observation_blocked_surface_count']}`",
        f"- WAT-changed surfaces: `{summary['wat_changed_surface_count']}`",
        f"- Default fail count: `{summary['default_snow_control_fail_count']}`",
        f"- Opt-in fail count: `{summary['opt_in_snow_control_fail_count']}`",
        f"- Fail delta default-minus-opt-in: `{summary['snow_control_fail_delta_default_minus_opt_in']}`",
        f"- Next route: {summary['next_route']}",
        "",
        "## Surface Results",
        "",
        "| Surface | Scope | Impact | Default fails | Opt-in fails | Mean abs reduction (m) | WAT changed days |",
        "|---|---:|---:|---:|---:|---:|---:|",
    ]
    for item in report["surfaces"]:
        delta = item["paired_residual_delta"]
        reduction = delta.get("mean_abs_depth_reduction_m")
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{item['surface_id']}`",
                    item["verdict_scope"],
                    item["impact_class"],
                    str(item["default"]["residuals"]["snow_control_fail_count"]),
                    str(item["opt_in"]["residuals"]["snow_control_fail_count"]),
                    fmt_optional(reduction),
                    str(item["wat_deltas"]["changed_snow_depth_day_count"]),
                ]
            )
            + " |"
        )
    lines.extend(
        [
            "",
            "## Boundary Disposition",
            "",
            "- Default activation changed: `false`.",
            "- Parser/runfile/user CLI selector added: `false`.",
            "- Fixture inputs changed: `false`.",
            "- Public output schema changed: `false`.",
            "- Production physics outside the existing opt-in selector changed: `false`.",
            "- Observation-blocked surfaces are diagnostic-only and carry no defect verdict.",
            "",
        ]
    )
    return "\n".join(lines)


def finite_values(values: Any) -> list[float]:
    result = []
    for value in values:
        if value is not None:
            result.append(float(value))
    return result


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def max_abs(values: list[float]) -> float | None:
    return max(abs(value) for value in values) if values else None


def fmt_optional(value: Any) -> str:
    if value is None:
        return "n/a"
    return f"{float(value):.6f}"


def rel(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


if __name__ == "__main__":
    raise SystemExit(main())
