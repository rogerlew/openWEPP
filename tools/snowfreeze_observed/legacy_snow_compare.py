#!/usr/bin/env python3
"""Compare legacy WEPP snow outputs with openWEPP and observations.

Legacy WEPP's normal WAT output publishes SWE as ``Snow-Water``. Date-aligned
physical snowpack depth is available from legacy daily-winter output at hour 24.
The large-graphics vector also carries ``treal(73)=snodpy*1000`` but is sparse
for the hillslope runs used here, so this helper enables both outputs in a
temporary replay copy and uses daily-winter rows for dated depth comparison.
Legacy agreement remains diagnostic flag evidence rather than an openWEPP
correctness target.
"""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import math
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any

import observed_harness


REPO_ROOT = Path(__file__).resolve().parents[2]
FIXTURE_ROOT = REPO_ROOT / "tests/fixtures/snowfreeze_observed"
DEFAULT_OBSERVATIONS = FIXTURE_ROOT / "observations"
DEFAULT_LEGACY_BINARY = (
    Path("/home/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill")
)
SCHEMA = "snowfreeze-legacy-snow-comparison-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-048"
LEGACY_BASELINE = "/home/workdir/wepp-forest_260430_baseline"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    parser.add_argument("--legacy-binary", type=Path, default=DEFAULT_LEGACY_BINARY)
    parser.add_argument("--openwepp-binary", type=Path, default=None)
    parser.add_argument(
        "--site",
        action="append",
        dest="sites",
        help="site_id to run; repeatable. Defaults to every manifest site.",
    )
    parser.add_argument(
        "--runtime",
        choices=["direct-production-executor", "compatibility"],
        default="direct-production-executor",
    )
    parser.add_argument(
        "--no-openwepp-run",
        action="store_true",
        help="reuse existing openWEPP comparison outputs under --output-dir/openwepp.",
    )
    parser.add_argument(
        "--no-legacy-run",
        action="store_true",
        help="reuse existing legacy outputs under --output-dir/legacy.",
    )
    args = parser.parse_args()

    report = compare_all(
        observations_dir=args.observations_dir.resolve(),
        output_dir=args.output_dir.resolve(),
        legacy_binary=args.legacy_binary.resolve(),
        openwepp_binary=args.openwepp_binary.resolve() if args.openwepp_binary else None,
        sites=args.sites,
        runtime=args.runtime,
        run_openwepp=not args.no_openwepp_run,
        run_legacy=not args.no_legacy_run,
    )
    write_json(args.output_json.resolve(), report)
    args.output_md.resolve().write_text(render_markdown(report), encoding="utf-8")
    return 0


def compare_all(
    observations_dir: Path,
    output_dir: Path,
    legacy_binary: Path,
    openwepp_binary: Path | None,
    sites: list[str] | None,
    runtime: str,
    run_openwepp: bool,
    run_legacy: bool,
) -> dict[str, Any]:
    observed_harness.validate_observations(observations_dir)
    manifest = json.loads((observations_dir / "manifest.json").read_text(encoding="utf-8"))
    site_records = [
        site for site in manifest["sites"] if sites is None or site["site_id"] in set(sites)
    ]
    requested = set(sites or [])
    discovered = {site["site_id"] for site in site_records}
    missing = sorted(requested - discovered)
    if missing:
        raise ValueError(f"requested unknown site ids: {missing}")
    if run_legacy and not legacy_binary.is_file():
        raise FileNotFoundError(f"legacy WEPP binary not found: {legacy_binary}")

    output_dir.mkdir(parents=True, exist_ok=True)
    site_reports = []
    for site in site_records:
        site_reports.append(
            compare_site(
                site=site,
                observations_dir=observations_dir,
                output_dir=output_dir,
                legacy_binary=legacy_binary,
                openwepp_binary=openwepp_binary,
                runtime=runtime,
                run_openwepp=run_openwepp,
                run_legacy=run_legacy,
            )
        )

    return {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "legacy_baseline": LEGACY_BASELINE,
        "legacy_binary": str(legacy_binary),
        "runtime": runtime,
        "site_count": len(site_reports),
        "summary": summarize(site_reports),
        "sites": site_reports,
    }


def compare_site(
    site: dict[str, Any],
    observations_dir: Path,
    output_dir: Path,
    legacy_binary: Path,
    openwepp_binary: Path | None,
    runtime: str,
    run_openwepp: bool,
    run_legacy: bool,
) -> dict[str, Any]:
    site_id = site["site_id"]
    fixture_dir = FIXTURE_ROOT / site["fixture"]
    observations = load_observations(observations_dir / site["observation_file"])

    open_dir = output_dir / "openwepp" / site_id
    if run_openwepp:
        observed_harness.compare_site(
            site_id=site_id,
            observations_dir=observations_dir,
            output_dir=open_dir,
            binary=openwepp_binary,
            no_run=False,
            runtime=runtime,
        )
    open_report_path = open_dir / "comparison_report.json"
    open_report = json.loads(open_report_path.read_text(encoding="utf-8"))
    open_wat_path = Path(open_report["wat_output"])
    open_rows = observed_harness.load_modeled_wat(open_wat_path)

    legacy_dir = output_dir / "legacy" / site_id
    if run_legacy:
        run_legacy_replay(fixture_dir, legacy_dir, legacy_binary)
    legacy_rows = load_legacy_rows(legacy_dir)

    observed_metrics = compare_to_observed(observations, open_rows, legacy_rows)
    model_metrics = compare_openwepp_to_legacy(open_rows, legacy_rows)
    return {
        "site_id": site_id,
        "source_id": site.get("source_id"),
        "fixture": site["fixture"],
        "openwepp_report": str(open_report_path.relative_to(REPO_ROOT)),
        "openwepp_wat_output": str(open_wat_path),
        "legacy_run_dir": str(legacy_dir),
        "legacy_wat_output": relative_or_absolute(legacy_dir / "output" / legacy_wat_name(legacy_dir)),
        "legacy_large_graphics_output": relative_or_absolute(
            legacy_dir / "output" / legacy_big_name(legacy_dir)
        ),
        "legacy_winter_output": relative_or_absolute(
            legacy_dir / "output" / legacy_winter_name(legacy_dir)
        ),
        "legacy_capture": {
            "snow_water_swe_source": "legacy WAT Snow-Water column, mm",
            "snow_depth_source": (
                "legacy daily winter hour-24 snow depth, mm; large graphics "
                "treal(73)=snodpy*1000 is retained as sparse operand provenance"
            ),
            "snow_density_source": (
                "legacy daily winter hour-24 snow density, kg/m^3; large "
                "graphics treal(75)=densg is retained as sparse operand provenance"
            ),
            "legacy_depth_is_not_wat": True,
        },
        "legacy_capture_metrics": legacy_capture_metrics(legacy_rows),
        "observed_snow_depth": observed_metrics,
        "openwepp_legacy": model_metrics,
        "route": route_site(observed_metrics, model_metrics),
    }


def run_legacy_replay(fixture_dir: Path, legacy_dir: Path, legacy_binary: Path) -> None:
    if legacy_dir.exists():
        shutil.rmtree(legacy_dir)
    run_dir = legacy_dir / "runs"
    output_dir = legacy_dir / "output"
    run_dir.mkdir(parents=True)
    output_dir.mkdir()
    for path in fixture_dir.iterdir():
        if path.is_file():
            shutil.copy2(path, run_dir / path.name)
    run_path = run_dir / observed_harness.discover_run_stem(fixture_dir)
    run_path = run_path.with_suffix(".run")
    enable_large_graphics(run_path)
    completed = subprocess.run(
        [str(legacy_binary)],
        cwd=run_dir,
        input=run_path.read_text(encoding="utf-8"),
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    legacy_dir.mkdir(parents=True, exist_ok=True)
    (legacy_dir / "wepp_260430_hill.stdout").write_text(completed.stdout, encoding="utf-8")
    (legacy_dir / "wepp_260430_hill.stderr").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise RuntimeError(
            f"legacy WEPP failed for {fixture_dir.name} with exit code {completed.returncode}"
        )
    wat_path = output_dir / legacy_wat_name(legacy_dir)
    big_path = output_dir / legacy_big_name(legacy_dir)
    winter_path = output_dir / legacy_winter_name(legacy_dir)
    if not wat_path.is_file():
        raise FileNotFoundError(f"legacy replay did not produce WAT output: {wat_path}")
    if not big_path.is_file():
        raise FileNotFoundError(
            f"legacy replay did not produce large-graphics snow output: {big_path}"
        )
    if not winter_path.is_file():
        raise FileNotFoundError(
            f"legacy replay did not produce daily-winter snow output: {winter_path}"
        )


def enable_large_graphics(run_path: Path) -> None:
    lines = run_path.read_text(encoding="utf-8").splitlines()
    large_enabled = False
    winter_enabled = False
    for idx, line in enumerate(lines):
        stripped = line.strip()
        if not large_enabled and stripped.endswith(".plot.dat"):
            if idx + 1 >= len(lines) or lines[idx + 1].strip().lower() != "no":
                raise ValueError(
                    f"{run_path} does not have the expected large-graphics 'No' "
                    "answer after the plotting output path"
                )
            lines[idx + 1] = "Yes"
            lines.insert(idx + 2, stripped.replace(".plot.dat", ".big.dat"))
            large_enabled = True
            break
    if not large_enabled:
        raise ValueError(f"{run_path} does not contain a .plot.dat output path")
    for idx, line in enumerate(lines):
        stripped = line.strip()
        if stripped.endswith(".element.dat"):
            if idx + 2 >= len(lines):
                raise ValueError(f"{run_path} ended before daily-winter answer")
            if lines[idx + 1].strip().lower() != "no":
                raise ValueError(
                    f"{run_path} expected final-summary 'No' after element output path"
                )
            if lines[idx + 2].strip().lower() != "no":
                raise ValueError(
                    f"{run_path} expected daily-winter 'No' after final-summary answer"
                )
            lines[idx + 2] = "Yes"
            lines.insert(idx + 3, stripped.replace(".element.dat", ".winter.dat"))
            winter_enabled = True
            break
    if not winter_enabled:
        raise ValueError(f"{run_path} does not contain a .element.dat output path")
    run_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def load_legacy_rows(legacy_dir: Path) -> dict[dt.date, dict[str, float | None]]:
    output_dir = legacy_dir / "output"
    wat_path = output_dir / legacy_wat_name(legacy_dir)
    big_path = output_dir / legacy_big_name(legacy_dir)
    winter_path = output_dir / legacy_winter_name(legacy_dir)
    wat_rows = parse_legacy_wat(wat_path)
    winter_rows = parse_legacy_winter(winter_path)
    big_row_count = len(parse_legacy_large_graphics(big_path))
    merged: dict[dt.date, dict[str, float | None]] = {}
    for date in sorted(wat_rows):
        row = dict(wat_rows[date])
        winter_row = winter_rows.get(date)
        if winter_row is not None:
            row.update(winter_row)
            row["legacy_snow_depth_capture"] = "daily-winter-hour-24"
        elif (row.get("legacy_snow_water_m") or 0.0) <= 1.0e-12:
            row.update(
                {
                    "legacy_snow_depth_m": 0.0,
                    "legacy_snow_density_kg_m3": 0.0,
                    "legacy_frost_depth_m": None,
                    "legacy_thaw_depth_m": None,
                    "legacy_snow_depth_capture": "wat-swe-zero-inferred-zero-depth",
                }
            )
        else:
            row.update(
                {
                    "legacy_snow_depth_m": None,
                    "legacy_snow_density_kg_m3": None,
                    "legacy_frost_depth_m": None,
                    "legacy_thaw_depth_m": None,
                    "legacy_snow_depth_capture": "missing-winter-row-with-nonzero-swe",
                }
            )
        row["legacy_sparse_large_graphics_row_count"] = float(big_row_count)
        merged[date] = row
    return merged


def legacy_wat_name(legacy_dir: Path) -> str:
    paths = sorted((legacy_dir / "output").glob("H*.wat.dat"))
    if len(paths) != 1:
        raise FileNotFoundError(f"expected one legacy H*.wat.dat under {legacy_dir / 'output'}")
    return paths[0].name


def legacy_big_name(legacy_dir: Path) -> str:
    paths = sorted((legacy_dir / "output").glob("H*.big.dat"))
    if len(paths) != 1:
        raise FileNotFoundError(f"expected one legacy H*.big.dat under {legacy_dir / 'output'}")
    return paths[0].name


def legacy_winter_name(legacy_dir: Path) -> str:
    paths = sorted((legacy_dir / "output").glob("H*.winter.dat"))
    if len(paths) != 1:
        raise FileNotFoundError(f"expected one legacy H*.winter.dat under {legacy_dir / 'output'}")
    return paths[0].name


def parse_legacy_wat(path: Path) -> dict[dt.date, dict[str, float | None]]:
    rows: dict[dt.date, dict[str, float | None]] = {}
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        parts = line.strip().split()
        if len(parts) not in (20, 25):
            continue
        try:
            values = [float(part) for part in parts]
        except ValueError:
            continue
        if not all(is_int_like(values[index]) for index in (0, 1, 2)):
            continue
        date = julian_date(int(values[2]), int(values[1]))
        if date in rows:
            raise ValueError(f"{path} has duplicate WAT date {date}")
        snow_water_m = values[15] / 1000.0
        rows[date] = {
            "legacy_snow_water_m": snow_water_m,
            "legacy_wat_width": float(len(values)),
        }
    if not rows:
        raise ValueError(f"{path} did not contain parseable legacy WAT rows")
    return rows


def parse_legacy_large_graphics(path: Path) -> dict[int, dict[str, float | None]]:
    rows: dict[int, dict[str, float | None]] = {}
    in_minmax = False
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        if "Minimum/Maximum values" in line:
            in_minmax = True
        if in_minmax:
            continue
        parts = line.strip().split()
        if len(parts) != 104:
            continue
        try:
            values = [float(part) for part in parts]
        except ValueError:
            continue
        if not is_int_like(values[0]):
            continue
        day = int(values[0])
        if day in rows:
            raise ValueError(f"{path} has duplicate large-graphics day {day}")
        rows[day] = {
            "legacy_frost_depth_m": values[71] / 1000.0,
            "legacy_snow_depth_m": values[73] / 1000.0,
            "legacy_snow_melt_water_m": values[74] / 1000.0,
            "legacy_snow_density_kg_m3": values[75],
        }
    if not rows:
        raise ValueError(f"{path} did not contain parseable large-graphics rows")
    return rows


def parse_legacy_winter(path: Path) -> dict[dt.date, dict[str, float | None]]:
    hourly: dict[dt.date, tuple[int, dict[str, float | None]]] = {}
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        parts = line.strip().split()
        if len(parts) != 16:
            continue
        try:
            values = [float(part) for part in parts]
        except ValueError:
            continue
        if not all(is_int_like(values[index]) for index in (0, 1, 2, 14, 15)):
            continue
        hour = int(values[1])
        if hour < 1 or hour > 24:
            continue
        ofe = int(values[15])
        if ofe != 1:
            raise ValueError(f"{path} has unsupported multi-OFE winter row for OFE {ofe}")
        date = julian_date(int(values[2]), int(values[0]))
        row = {
            "legacy_snowfall_m": values[3] / 1000.0,
            "legacy_rainfall_m": values[4] / 1000.0,
            "legacy_melt_water_m": values[7] / 1000.0,
            "legacy_snow_depth_m": values[8] / 1000.0,
            "legacy_snow_density_kg_m3": values[9],
            "legacy_frost_depth_m": values[10] / 1000.0,
            "legacy_thaw_depth_m": values[11] / 1000.0,
            "legacy_frost_thickness_m": values[12] / 1000.0,
            "legacy_residue_depth_m": values[13] / 1000.0,
            "legacy_winter_hour": float(hour),
        }
        if date not in hourly or hour >= hourly[date][0]:
            hourly[date] = (hour, row)
    if not hourly:
        raise ValueError(f"{path} did not contain parseable daily-winter rows")
    return {date: row for date, (_hour, row) in hourly.items()}


def compare_to_observed(
    observations: list[dict[str, str]],
    open_rows: dict[dt.date, dict[str, float | None]],
    legacy_rows: dict[dt.date, dict[str, float | None]],
) -> dict[str, Any]:
    paired = []
    observed_snow_depth_count = 0
    for observation in observations:
        observed_snow_depth_m = parse_optional_float(observation["observed_snow_depth_m"])
        if observed_snow_depth_m is None:
            continue
        observed_snow_depth_count += 1
        date = dt.date.fromisoformat(observation["date"])
        open_row = open_rows.get(date)
        legacy_row = legacy_rows.get(date)
        if open_row is None or legacy_row is None:
            continue
        open_depth = open_row.get("snow_depth_m")
        legacy_depth = legacy_row.get("legacy_snow_depth_m")
        if open_depth is None or legacy_depth is None:
            continue
        tolerance = observed_harness.snow_depth_control_tolerance_m(observed_snow_depth_m)
        open_residual = open_depth - observed_snow_depth_m
        legacy_residual = legacy_depth - observed_snow_depth_m
        paired.append(
            {
                "date": date.isoformat(),
                "observed_snow_depth_m": observed_snow_depth_m,
                "openwepp_snow_depth_m": open_depth,
                "legacy_snow_depth_m": legacy_depth,
                "openwepp_residual_m": open_residual,
                "legacy_residual_m": legacy_residual,
                "openwepp_abs_residual_m": abs(open_residual),
                "legacy_abs_residual_m": abs(legacy_residual),
                "tolerance_m": tolerance,
                "openwepp_within_tolerance": abs(open_residual) <= tolerance,
                "legacy_within_tolerance": abs(legacy_residual) <= tolerance,
                "legacy_better_than_openwepp": abs(legacy_residual) < abs(open_residual),
                "openwepp_better_than_legacy": abs(open_residual) < abs(legacy_residual),
            }
        )
    return {
        "observed_snow_depth_count": observed_snow_depth_count,
        "paired_depth_count": len(paired),
        "openwepp": residual_stats(
            [row["openwepp_residual_m"] for row in paired],
            [row["openwepp_within_tolerance"] for row in paired],
        ),
        "legacy": residual_stats(
            [row["legacy_residual_m"] for row in paired],
            [row["legacy_within_tolerance"] for row in paired],
        ),
        "legacy_better_count": sum(1 for row in paired if row["legacy_better_than_openwepp"]),
        "openwepp_better_count": sum(1 for row in paired if row["openwepp_better_than_legacy"]),
        "equal_abs_residual_count": sum(
            1
            for row in paired
            if row["legacy_abs_residual_m"] == row["openwepp_abs_residual_m"]
        ),
        "sample_pairs": paired[:20],
    }


def compare_openwepp_to_legacy(
    open_rows: dict[dt.date, dict[str, float | None]],
    legacy_rows: dict[dt.date, dict[str, float | None]],
) -> dict[str, Any]:
    common_dates = sorted(set(open_rows) & set(legacy_rows))
    depth_deltas = []
    swe_deltas = []
    density_rows = 0
    for date in common_dates:
        open_depth = open_rows[date].get("snow_depth_m")
        legacy_depth = legacy_rows[date].get("legacy_snow_depth_m")
        if open_depth is not None and legacy_depth is not None:
            depth_deltas.append(open_depth - legacy_depth)
        open_swe = open_rows[date].get("snow_water_m")
        legacy_swe = legacy_rows[date].get("legacy_snow_water_m")
        if open_swe is not None and legacy_swe is not None:
            swe_deltas.append(open_swe - legacy_swe)
        if legacy_rows[date].get("legacy_snow_density_kg_m3") is not None:
            density_rows += 1
    return {
        "common_day_count": len(common_dates),
        "snow_depth_delta_openwepp_minus_legacy_m": delta_stats(depth_deltas),
        "snow_water_delta_openwepp_minus_legacy_m": delta_stats(swe_deltas),
        "legacy_snow_density_day_count": density_rows,
    }


def legacy_capture_metrics(legacy_rows: dict[dt.date, dict[str, float | None]]) -> dict[str, Any]:
    captures = count_values(row.get("legacy_snow_depth_capture") for row in legacy_rows.values())
    missing_nonzero = sum(
        1
        for row in legacy_rows.values()
        if row.get("legacy_snow_depth_capture") == "missing-winter-row-with-nonzero-swe"
    )
    sparse_counts = {
        int(row["legacy_sparse_large_graphics_row_count"])
        for row in legacy_rows.values()
        if row.get("legacy_sparse_large_graphics_row_count") is not None
    }
    return {
        "legacy_day_count": len(legacy_rows),
        "snow_depth_capture_counts": captures,
        "missing_winter_row_with_nonzero_swe_day_count": missing_nonzero,
        "sparse_large_graphics_row_count": next(iter(sparse_counts)) if sparse_counts else None,
    }


def residual_stats(residuals: list[float], within_tolerance: list[bool]) -> dict[str, Any]:
    abs_values = [abs(value) for value in residuals]
    return {
        "count": len(residuals),
        "pass_count": sum(1 for value in within_tolerance if value),
        "fail_count": sum(1 for value in within_tolerance if not value),
        "mean_signed_residual_m": mean(residuals),
        "median_signed_residual_m": median(residuals),
        "mean_abs_residual_m": mean(abs_values),
        "max_abs_residual_m": max(abs_values) if abs_values else None,
        "modeled_over_observed_count": sum(1 for value in residuals if value > 0.0),
        "modeled_under_observed_count": sum(1 for value in residuals if value < 0.0),
    }


def delta_stats(deltas: list[float]) -> dict[str, Any]:
    abs_values = [abs(value) for value in deltas]
    return {
        "count": len(deltas),
        "mean_signed_delta_m": mean(deltas),
        "median_signed_delta_m": median(deltas),
        "mean_abs_delta_m": mean(abs_values),
        "max_abs_delta_m": max(abs_values) if abs_values else None,
        "openwepp_gt_legacy_count": sum(1 for value in deltas if value > 0.0),
        "openwepp_lt_legacy_count": sum(1 for value in deltas if value < 0.0),
        "equal_count": sum(1 for value in deltas if value == 0.0),
    }


def route_site(observed_metrics: dict[str, Any], model_metrics: dict[str, Any]) -> str:
    paired = observed_metrics["paired_depth_count"]
    if paired == 0:
        return "NO-PAIRED-OBSERVED-SNOW-DEPTH"
    legacy_fail = observed_metrics["legacy"]["fail_count"]
    open_fail = observed_metrics["openwepp"]["fail_count"]
    if legacy_fail == 0 and open_fail > 0:
        return "LEGACY-SNOW-DEPTH-CLOSER-FLAG"
    if open_fail == 0 and legacy_fail > 0:
        return "OPENWEPP-SNOW-DEPTH-CLOSER-FLAG"
    if legacy_fail > 0 and open_fail > 0:
        legacy_mean = observed_metrics["legacy"]["mean_abs_residual_m"]
        open_mean = observed_metrics["openwepp"]["mean_abs_residual_m"]
        if legacy_mean is not None and open_mean is not None and legacy_mean < open_mean:
            return "BOTH-FAIL-LEGACY-CLOSER-FLAG"
        if legacy_mean is not None and open_mean is not None and open_mean < legacy_mean:
            return "BOTH-FAIL-OPENWEPP-CLOSER-FLAG"
        return "BOTH-FAIL-SNOW-DEPTH-CONTROL"
    return "BOTH-PASS-SNOW-DEPTH-CONTROL"


def summarize(site_reports: list[dict[str, Any]]) -> dict[str, Any]:
    route_counts = count_values(site["route"] for site in site_reports)
    paired_sites = [
        site
        for site in site_reports
        if site["observed_snow_depth"]["paired_depth_count"] > 0
    ]
    legacy_closer_sites = [
        site["site_id"]
        for site in paired_sites
        if site["observed_snow_depth"]["legacy"]["mean_abs_residual_m"]
        is not None
        and site["observed_snow_depth"]["openwepp"]["mean_abs_residual_m"]
        is not None
        and site["observed_snow_depth"]["legacy"]["mean_abs_residual_m"]
        < site["observed_snow_depth"]["openwepp"]["mean_abs_residual_m"]
    ]
    openwepp_closer_sites = [
        site["site_id"]
        for site in paired_sites
        if site["observed_snow_depth"]["legacy"]["mean_abs_residual_m"]
        is not None
        and site["observed_snow_depth"]["openwepp"]["mean_abs_residual_m"]
        is not None
        and site["observed_snow_depth"]["openwepp"]["mean_abs_residual_m"]
        < site["observed_snow_depth"]["legacy"]["mean_abs_residual_m"]
    ]
    return {
        "route_counts": route_counts,
        "paired_observed_snow_depth_site_count": len(paired_sites),
        "legacy_closer_by_mean_abs_sites": legacy_closer_sites,
        "openwepp_closer_by_mean_abs_sites": openwepp_closer_sites,
        "legacy_is_correctness_target": False,
        "legacy_snow_depth_capture_required_surface": (
            "daily-winter hour-24 snow depth; large graphics treal(73) is sparse "
            "operand provenance, not the date-aligned comparator feed"
        ),
    }


def load_observations(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def parse_optional_float(value: str) -> float | None:
    if value.strip() == "":
        return None
    parsed = float(value)
    if not math.isfinite(parsed):
        raise ValueError(f"non-finite float {value!r}")
    return parsed


def julian_date(year: int, julian_day: int) -> dt.date:
    return dt.date(year, 1, 1) + dt.timedelta(days=julian_day - 1)


def is_int_like(value: float) -> bool:
    return float(int(value)) == value


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def median(values: list[float]) -> float | None:
    if not values:
        return None
    ordered = sorted(values)
    midpoint = len(ordered) // 2
    if len(ordered) % 2 == 1:
        return ordered[midpoint]
    return (ordered[midpoint - 1] + ordered[midpoint]) / 2.0


def count_values(values: Any) -> dict[str, int]:
    counts: dict[str, int] = {}
    for value in values:
        counts[str(value)] = counts.get(str(value), 0) + 1
    return dict(sorted(counts.items()))


def relative_or_absolute(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def render_markdown(report: dict[str, Any]) -> str:
    lines = [
        "# Legacy Snow Comparison",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Contract: `{report['contract']}`",
        f"- Legacy baseline: `{report['legacy_baseline']}`",
        f"- Runtime: `{report['runtime']}`",
        f"- Site count: `{report['site_count']}`",
        f"- Route counts: `{report['summary']['route_counts']}`",
        f"- Legacy closer by mean absolute observed-depth residual: `{report['summary']['legacy_closer_by_mean_abs_sites']}`",
        f"- openWEPP closer by mean absolute observed-depth residual: `{report['summary']['openwepp_closer_by_mean_abs_sites']}`",
        f"- Legacy correctness target: `{report['summary']['legacy_is_correctness_target']}`",
        "",
        "## Site Summary",
        "",
        "| Site | Route | Obs pairs | openWEPP mean abs depth m | Legacy mean abs depth m | openWEPP failures | Legacy failures | Legacy better rows | openWEPP better rows | Depth delta mean abs m | SWE delta mean abs m |",
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for site in report["sites"]:
        observed = site["observed_snow_depth"]
        model = site["openwepp_legacy"]
        lines.append(
            "| {site} | {route} | {pairs} | {open_mean} | {legacy_mean} | {open_fail} | {legacy_fail} | {legacy_better} | {open_better} | {depth_delta} | {swe_delta} |".format(
                site=site["site_id"],
                route=site["route"],
                pairs=observed["paired_depth_count"],
                open_mean=fmt(observed["openwepp"]["mean_abs_residual_m"]),
                legacy_mean=fmt(observed["legacy"]["mean_abs_residual_m"]),
                open_fail=observed["openwepp"]["fail_count"],
                legacy_fail=observed["legacy"]["fail_count"],
                legacy_better=observed["legacy_better_count"],
                open_better=observed["openwepp_better_count"],
                depth_delta=fmt(
                    model["snow_depth_delta_openwepp_minus_legacy_m"][
                        "mean_abs_delta_m"
                    ]
                ),
                swe_delta=fmt(
                    model["snow_water_delta_openwepp_minus_legacy_m"][
                        "mean_abs_delta_m"
                    ]
                ),
            )
        )
    lines.extend(
        [
            "",
            "## Capture Lineage",
            "",
            "- Legacy SWE is parsed from normal WAT `Snow-Water` rows.",
            "- Legacy physical snow depth is parsed from dated daily-winter hour-24 rows produced by a temporary replay with the existing `.run` daily-winter answer changed from `No` to `Yes`.",
            "- Legacy large graphics is also enabled; `treal(73)=snodpy*1000` and `treal(75)=densg` prove the same physical operands exist there, but that output is sparse for these hillslope fixtures and is not used for date-aligned observed comparisons.",
            "- Legacy snow density is parsed from dated daily-winter hour-24 rows to support depth/SWE anti-alias review.",
            "- Legacy agreement remains flag evidence under ADR-0017; observed physical snow depth and `INV-SNOWFREEZE-048` remain the correspondence authority.",
            "",
        ]
    )
    return "\n".join(lines)


def fmt(value: Any) -> str:
    if value is None:
        return "n/a"
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


if __name__ == "__main__":
    raise SystemExit(main())
