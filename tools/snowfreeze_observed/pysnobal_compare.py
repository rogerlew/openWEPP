#!/usr/bin/env python3
"""Run PySnobal against SNOWFROST-FIDELITY-G0 exported forcing artifacts."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import math
import os
import subprocess
import sys
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snowfreeze_observed/observations"
DEFAULT_PYSNOBAL_PATH = Path("/workdir/pysnobal")
LANE_DIR_PREFIX = "tg_"
BULK_DENSITY_CEILING_KGM3 = 700.0
CHILD_RUNNER = r"""
import json
import math
import sys
from pathlib import Path

import pandas as pd
import pysnobal.defaults as defaults
import pysnobal.pysnobal as pysnobal_runner


def _stable_check_forcing_df(forcing_data_df):
    for key in defaults.FORCING_NAMES_CUSTOM2SNOBAL:
        if key not in forcing_data_df.columns:
            raise ValueError(f"Dataframe missing {key}")
    timesteps = (
        forcing_data_df.index.to_series().diff().dropna().dt.total_seconds().unique()
    )
    if len(timesteps) != 1:
        raise ValueError(f"Dataframe has a non-uniform timestep: {timesteps}")
    for column, missing in forcing_data_df[
        list(defaults.FORCING_NAMES_CUSTOM2SNOBAL.keys())
    ].isna().sum().items():
        if missing > 0:
            raise ValueError(f"Column {column} is not serially complete")
    return float(timesteps[0])


pysnobal_runner._check_forcing_df = _stable_check_forcing_df

lane_dir = Path(sys.argv[1])
forcing_path = lane_dir / "forcing.csv"
config_path = lane_dir / "config.yaml"
output_path = lane_dir / "pysnobal_output.csv"
forcing = pd.read_csv(forcing_path, index_col=0, parse_dates=True)
forcing_for_checks = forcing.copy(deep=True)
config = pysnobal_runner.load_config(config_path)
output = pysnobal_runner.run_snobal(forcing, config, show_pbar=False)
output.to_csv(output_path)

swe = output["specific_mass_snow_kgm-2"]
depth = output["thickness_snow_m"]
if len(output) == 0:
    raise ValueError("PySnobal returned zero output rows")
if not swe.map(math.isfinite).all() or not depth.map(math.isfinite).all():
    raise ValueError("PySnobal returned non-finite SWE or depth")
if (swe < -1.0e-9).any() or (depth < -1.0e-9).any():
    raise ValueError("PySnobal returned negative SWE or depth")
positive_depth = depth > 1.0e-9
if positive_depth.any():
    density = swe[positive_depth] / depth[positive_depth]
    if not density.map(math.isfinite).all():
        raise ValueError("PySnobal positive-depth density contains non-finite values")
    if (density > 700.0).any():
        raise ValueError("PySnobal positive-depth density exceeds 700 kg/m^3")
snow_precip_mass = (
    forcing_for_checks["precip_mass_mm"] * forcing_for_checks["snow_precip_fraction"]
)
if (snow_precip_mass > 1.0e-9).any() and max(float(swe.max()), float(depth.max())) <= 1.0e-9:
    raise ValueError("positive snow precipitation never produced positive SWE or depth")
peak_depth_idx = depth.idxmax()
summary = {
    "status": "PASS",
    "row_count": int(len(output)),
    "forcing_row_count": int(len(forcing)),
    "max_swe_kgm2": float(swe.max()),
    "max_snow_depth_m": float(depth.max()),
    "peak_snow_depth_datetime": str(peak_depth_idx),
    "positive_snow_precip_rows": int((snow_precip_mass > 1.0e-9).sum()),
    "output_path": str(output_path),
}
print(json.dumps(summary))
"""


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-root", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    parser.add_argument("--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS)
    parser.add_argument("--pysnobal-path", type=Path, default=DEFAULT_PYSNOBAL_PATH)
    args = parser.parse_args(argv)

    python = Path(os.environ.get("PYSNOBAL_PYTHON", sys.executable))
    summary = run_all_sites(
        input_root=args.input_root.resolve(),
        observations_dir=args.observations_dir.resolve(),
        pysnobal_python=python,
        pysnobal_path=args.pysnobal_path.resolve(),
    )
    write_json(args.output_json.resolve(), summary)
    write_markdown(args.output_md.resolve(), summary)
    return 1 if summary["route_recommendation"].startswith("HOLD-") else 0


def run_all_sites(
    input_root: Path,
    observations_dir: Path,
    pysnobal_python: Path,
    pysnobal_path: Path,
) -> dict[str, Any]:
    if not input_root.is_dir():
        raise FileNotFoundError(f"input root not found: {input_root}")
    if not pysnobal_python.is_file():
        raise FileNotFoundError(f"PYSNOBAL_PYTHON is not a file: {pysnobal_python}")
    if not pysnobal_path.is_dir():
        raise FileNotFoundError(f"PySnobal source path not found: {pysnobal_path}")
    probe = probe_pysnobal(pysnobal_python, pysnobal_path)
    if probe is not None:
        return {
            "schema": "snowfrost-fidelity-g0-pysnobal-site-summary-v1",
            "input_root": str(input_root),
            "pysnobal_python": str(pysnobal_python),
            "pysnobal_path": str(pysnobal_path),
            "bulk_density_ceiling_kgm3": BULK_DENSITY_CEILING_KGM3,
            "sites": [],
            "route_recommendation": "HOLD-PYSNOBAL-UNAVAILABLE",
            "pysnobal_unavailable_reason": probe,
        }
    observation_index = load_observation_index(observations_dir)

    sites = []
    for site_dir in sorted(path for path in input_root.iterdir() if path.is_dir()):
        audit_path = site_dir / "tg_neg2p5c_zg0p10m" / "audit.json"
        if not audit_path.is_file():
            continue
        audit = json.loads(audit_path.read_text(encoding="utf-8"))
        fixture_id = Path(audit["run_dir"]).name
        observed = observation_index.get(fixture_id, [])
        site_summary = run_site(site_dir, fixture_id, observed, pysnobal_python, pysnobal_path)
        sites.append(site_summary)

    if not sites:
        raise ValueError(f"no G0 site export directories found under {input_root}")
    route = route_recommendation(sites)
    return {
        "schema": "snowfrost-fidelity-g0-pysnobal-site-summary-v1",
        "input_root": str(input_root),
        "pysnobal_python": str(pysnobal_python),
        "pysnobal_path": str(pysnobal_path),
        "bulk_density_ceiling_kgm3": BULK_DENSITY_CEILING_KGM3,
        "sites": sites,
        "route_recommendation": route,
    }


def probe_pysnobal(pysnobal_python: Path, pysnobal_path: Path) -> str | None:
    env = os.environ.copy()
    env["PYTHONPATH"] = prepend_path(env.get("PYTHONPATH"), pysnobal_path)
    code = (
        "import pysnobal.defaults as defaults; "
        "import pysnobal.pysnobal as runner; "
        "assert defaults.FORCING_NAMES_CUSTOM2SNOBAL; "
        "assert hasattr(runner, 'run_snobal')"
    )
    completed = subprocess.run(
        [str(pysnobal_python), "-c", code],
        cwd=REPO_ROOT,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if completed.returncode == 0:
        return None
    detail = completed.stderr.strip() or completed.stdout.strip()
    return f"PySnobal import probe failed with exit code {completed.returncode}: {detail}"


def run_site(
    site_dir: Path,
    fixture_id: str,
    observed_rows: list[dict[str, str]],
    pysnobal_python: Path,
    pysnobal_path: Path,
) -> dict[str, Any]:
    lanes = []
    for lane_dir in sorted(path for path in site_dir.iterdir() if path.is_dir()):
        if not lane_dir.name.startswith(LANE_DIR_PREFIX):
            continue
        lanes.append(run_lane(lane_dir, observed_rows, pysnobal_python, pysnobal_path))
    lane_spread = max_lane_depth_spread(lanes)
    return {
        "site_dir": str(site_dir),
        "fixture_id": fixture_id,
        "observed_snow_depth_rows": sum(
            1 for row in observed_rows if optional_float(row["observed_snow_depth_m"]) is not None
        ),
        "lanes": lanes,
        "lane_spread_max_depth_m": lane_spread,
    }


def run_lane(
    lane_dir: Path,
    observed_rows: list[dict[str, str]],
    pysnobal_python: Path,
    pysnobal_path: Path,
) -> dict[str, Any]:
    required = ["forcing.csv", "config.yaml", "lineage.json", "audit.json"]
    for name in required:
        if not (lane_dir / name).is_file():
            return lane_failure(lane_dir, f"missing required export artifact {name}")
    lineage = json.loads((lane_dir / "lineage.json").read_text(encoding="utf-8"))
    lineage_error = validate_lineage(lineage)
    if lineage_error:
        return lane_failure(lane_dir, lineage_error)

    env = os.environ.copy()
    env["PYTHONPATH"] = prepend_path(env.get("PYTHONPATH"), pysnobal_path)
    completed = subprocess.run(
        [str(pysnobal_python), "-c", CHILD_RUNNER, str(lane_dir)],
        cwd=REPO_ROOT,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    (lane_dir / "pysnobal_stdout.txt").write_text(completed.stdout, encoding="utf-8")
    (lane_dir / "pysnobal_stderr.txt").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        return lane_failure(
            lane_dir,
            f"PySnobal failed with exit code {completed.returncode}: {completed.stderr.strip()}",
        )
    try:
        summary = json.loads(completed.stdout.strip().splitlines()[-1])
    except (IndexError, json.JSONDecodeError) as error:
        return lane_failure(lane_dir, f"PySnobal did not return JSON summary: {error}")

    daily_depth = load_daily_depth(Path(summary["output_path"]))
    observed_metrics = compare_observed_depth(observed_rows, daily_depth)
    openwepp_metrics = compare_openwepp(lane_dir.parent / "openwepp_snow.csv", daily_depth)
    summary.update(
        {
            "lane_id": lane_dir.name,
            "lane_dir": str(lane_dir),
            "observed_depth": observed_metrics,
            "openwepp_comparison": openwepp_metrics,
        }
    )
    write_json(lane_dir / "pysnobal_summary.json", summary)
    write_lane_markdown(lane_dir / "pysnobal_summary.md", summary)
    return summary


def lane_failure(lane_dir: Path, reason: str) -> dict[str, Any]:
    summary = {
        "status": "FAIL",
        "lane_id": lane_dir.name,
        "lane_dir": str(lane_dir),
        "reason": reason,
    }
    write_json(lane_dir / "pysnobal_summary.json", summary)
    write_lane_markdown(lane_dir / "pysnobal_summary.md", summary)
    return summary


def validate_lineage(lineage: dict[str, Any]) -> str | None:
    fields = lineage.get("fields")
    if not isinstance(fields, dict):
        return "lineage.json lacks fields object"
    required = {
        "net_solar_Wm-2",
        "downwelling_thermal_Wm-2",
        "temp_air_degC",
        "temp_ground_degC",
        "vapor_pressure_Pa",
        "wind_speed_ms-1",
        "precip_mass_mm",
        "precip_temp_degC",
        "snow_precip_fraction",
        "snow_precip_density_kgm-3",
    }
    for field in required:
        entry = fields.get(field)
        if not isinstance(entry, dict):
            return f"lineage missing {field}"
        if entry.get("source_class") not in {
            "mechanical",
            "deterministic-derived",
            "diagnostic-proxy",
        }:
            return f"lineage missing source_class for {field}"
        if not entry.get("rejected_aliases"):
            return f"lineage missing rejected_aliases for {field}"
    return None


def load_observation_index(observations_dir: Path) -> dict[str, list[dict[str, str]]]:
    manifest = json.loads((observations_dir / "manifest.json").read_text(encoding="utf-8"))
    index: dict[str, list[dict[str, str]]] = {}
    for site in manifest["sites"]:
        observation_file = observations_dir / site["observation_file"]
        with observation_file.open(newline="", encoding="utf-8") as handle:
            index[site["fixture"]] = list(csv.DictReader(handle))
    return index


def load_daily_depth(output_path: Path) -> dict[dt.date, float]:
    rows: dict[dt.date, float] = {}
    with output_path.open(newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        for row in reader:
            timestamp = dt.datetime.fromisoformat(row["Datetime"])
            depth = float(row["thickness_snow_m"])
            if not math.isfinite(depth) or depth < -1.0e-9:
                raise ValueError(f"invalid PySnobal depth {depth} at {timestamp}")
            date = timestamp.date()
            rows[date] = max(rows.get(date, 0.0), max(depth, 0.0))
    return rows


def compare_observed_depth(
    observed_rows: list[dict[str, str]], daily_depth: dict[dt.date, float]
) -> dict[str, Any]:
    residuals = []
    matched = 0
    for row in observed_rows:
        observed = optional_float(row["observed_snow_depth_m"])
        if observed is None:
            continue
        date = dt.date.fromisoformat(row["date"])
        modeled = daily_depth.get(date)
        if modeled is None:
            continue
        matched += 1
        residuals.append(modeled - observed)
    return {
        "paired_snow_depth_count": matched,
        "mean_abs_residual_m": mean_abs(residuals),
        "max_abs_residual_m": max_abs(residuals),
        "modeled_minus_observed_mean_m": mean(residuals),
    }


def compare_openwepp(path: Path, daily_depth: dict[dt.date, float]) -> dict[str, Any]:
    if not path.is_file():
        return {"status": "UNAVAILABLE", "paired_count": 0}
    rows = []
    with path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            if row.get("Snow-Depth_mm"):
                date = dt.date.fromisoformat(row["date"])
                rows.append((date, float(row["Snow-Depth_mm"]) / 1000.0))
    residuals = [daily_depth[date] - depth for date, depth in rows if date in daily_depth]
    return {
        "status": "AVAILABLE" if rows else "NO_ROWS",
        "paired_count": len(residuals),
        "mean_abs_py_minus_openwepp_depth_m": mean_abs(residuals),
        "max_abs_py_minus_openwepp_depth_m": max_abs(residuals),
    }


def max_lane_depth_spread(lanes: list[dict[str, Any]]) -> float | None:
    depths = [
        lane.get("max_snow_depth_m")
        for lane in lanes
        if lane.get("status") == "PASS" and lane.get("max_snow_depth_m") is not None
    ]
    if len(depths) < 2:
        return None
    return max(depths) - min(depths)


def route_recommendation(sites: list[dict[str, Any]]) -> str:
    failed = [lane for site in sites for lane in site["lanes"] if lane["status"] != "PASS"]
    if failed:
        return "HOLD-PYSNOBAL-SANITY-FAILURE"
    spreads = [
        site["lane_spread_max_depth_m"]
        for site in sites
        if site["lane_spread_max_depth_m"] is not None
    ]
    if spreads and max(spreads) > 0.50:
        return "HOLD-FORCING-PROXY-DOMINATES"
    return "PROCEED-SNOWFROST-FIDELITY-G"


def optional_float(value: str) -> float | None:
    if value == "":
        return None
    parsed = float(value)
    return parsed if math.isfinite(parsed) else None


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def mean_abs(values: list[float]) -> float | None:
    return sum(abs(value) for value in values) / len(values) if values else None


def max_abs(values: list[float]) -> float | None:
    return max((abs(value) for value in values), default=None)


def prepend_path(existing: str | None, path: Path) -> str:
    if existing:
        return str(path) + os.pathsep + existing
    return str(path)


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_markdown(path: Path, summary: dict[str, Any]) -> None:
    lines = [
        "# PySnobal G0 Site Summary",
        "",
        f"- PySnobal Python: `{summary['pysnobal_python']}`",
        f"- PySnobal path: `{summary['pysnobal_path']}`",
        f"- Route recommendation: `{summary['route_recommendation']}`",
        "",
    ]
    if summary.get("pysnobal_unavailable_reason"):
        lines.extend(
            [
                f"- PySnobal unavailable: `{summary['pysnobal_unavailable_reason']}`",
                "",
            ]
        )
    lines.extend(
        [
            "| Site | Lane | Status | Max SWE kg/m2 | Max depth m | Paired obs | Mean abs obs residual m |",
            "| --- | --- | --- | ---: | ---: | ---: | ---: |",
        ]
    )
    for site in summary["sites"]:
        for lane in site["lanes"]:
            observed = lane.get("observed_depth", {})
            lines.append(
                "| {site} | {lane} | {status} | {swe} | {depth} | {paired} | {resid} |".format(
                    site=site["fixture_id"],
                    lane=lane["lane_id"],
                    status=lane["status"],
                    swe=fmt(lane.get("max_swe_kgm2")),
                    depth=fmt(lane.get("max_snow_depth_m")),
                    paired=observed.get("paired_snow_depth_count", 0),
                    resid=fmt(observed.get("mean_abs_residual_m")),
                )
            )
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_lane_markdown(path: Path, summary: dict[str, Any]) -> None:
    lines = [
        "# PySnobal Lane Summary",
        "",
        f"- Lane: `{summary['lane_id']}`",
        f"- Status: `{summary['status']}`",
    ]
    if summary["status"] != "PASS":
        lines.append(f"- Reason: `{summary['reason']}`")
    else:
        observed = summary["observed_depth"]
        lines.extend(
            [
                f"- Rows: `{summary['row_count']}`",
                f"- Max SWE: `{summary['max_swe_kgm2']:.6f}` kg/m2",
                f"- Max snow depth: `{summary['max_snow_depth_m']:.6f}` m",
                f"- Peak snow-depth datetime: `{summary['peak_snow_depth_datetime']}`",
                f"- Paired observed snow-depth rows: `{observed['paired_snow_depth_count']}`",
                f"- Mean absolute observed-depth residual: `{fmt(observed['mean_abs_residual_m'])}` m",
            ]
        )
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def fmt(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, float):
        return f"{value:.6f}"
    return str(value)


if __name__ == "__main__":
    raise SystemExit(main())
