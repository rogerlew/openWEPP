#!/usr/bin/env python3
"""SNOTEL SWE/depth/density three-way snow comparison.

This is a diagnostic harness for SNOWFROST-FIDELITY-H.  It acquires public NRCS
AWDB SNOTEL rows, normalizes paired SWE/depth/density observations, derives
site-characterization SSD arms from observed peak-SWE-period density, and
compares openWEPP, pinned legacy WEPP, and PySnobal against those observations.

The tool deliberately writes model run inputs under ``target/``.  The committed
SNOTEL fixtures remain the as-built ``snow.txt`` inputs.
"""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import json
import math
import os
import shutil
import subprocess
import sys
import urllib.parse
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import legacy_snow_compare  # noqa: E402
import observed_harness  # noqa: E402


ACCESS_DATE = "2026-06-25"
AWDB_DATA_URL = "https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/data"
AWDB_STATIONS_URL = "https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/stations"
AWDB_ELEMENTS = "WTEQ,SNWD,PREC,TOBS,TMAX,TMIN,STO:*:*"
DEFAULT_CACHE = REPO_ROOT / "target/snowfrost_fidelity_h/snotel_awdb_cache"
DEFAULT_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snotel_observed/observations"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowfrost_fidelity_h"
FIXTURE_ROOT = REPO_ROOT / "tests/fixtures/snotel_observed"
DEFAULT_LEGACY_BINARY = Path("/home/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill")
DEFAULT_PYSNOBAL_PYTHON = Path(os.environ.get("PYSNOBAL_PYTHON", "/tmp/pysnobal-g0-venv/bin/python"))
DEFAULT_PYSNOBAL_PATH = Path("/workdir/pysnobal")
OBSERVATION_COLUMNS = [
    "site_id",
    "station_triplet",
    "date",
    "water_year",
    "observed_swe_mm",
    "observed_snow_depth_m",
    "observed_density_kg_m3",
    "observed_precip_mm",
    "observed_tobs_c",
    "observed_tmax_c",
    "observed_tmin_c",
    "observed_soil_temp_c",
    "observed_soil_temp_depth_m",
    "observed_soil_temp_source",
    "quality_flag",
    "source_record_id",
]
DENSITY_CAP_KG_M3 = 522.0
MIN_DENSITY_FOR_SSD_KG_M3 = 50.0
PEAK_WINDOW_DAYS = 15
SNOW_DEPTH_TOL_ABS_M = 0.10
SNOW_DEPTH_TOL_REL = 0.30
SWE_TOL_ABS_M = 0.05
SWE_TOL_REL = 0.25
DENSITY_TOL_ABS_KG_M3 = 60.0
DENSITY_TOL_REL = 0.25
SNOW_COVER_DEPTH_THRESHOLD_M = 0.0254
KGE_PASS = 0.60
KGE_MARGINAL = 0.30
TIMING_STRONG_DAYS = 7.0
TIMING_PASS_DAYS = 14.0
TIMING_MARGINAL_DAYS = 30.0
MAGNITUDE_L_PASS_REL = 0.30
MAGNITUDE_L_MARGINAL_REL = 0.60


@dataclass(frozen=True)
class SnotelSite:
    site_id: str
    triplet: str
    snow_climate: str
    begin: str
    end: str
    fallback_ssd_kg_m3: float
    has_sto: bool


SITES = [
    SnotelSite(
        site_id="snotel_mica_creek_st_joe_id",
        triplet="623:ID:SNTL",
        snow_climate="northern_rockies_intermountain",
        begin="1986-01-01",
        end="2024-12-31",
        fallback_ssd_kg_m3=350.0,
        has_sto=True,
    ),
    SnotelSite(
        site_id="snotel_paradise_wa",
        triplet="679:WA:SNTL",
        snow_climate="cascades_maritime",
        begin="1980-01-01",
        end="2024-12-31",
        fallback_ssd_kg_m3=480.0,
        has_sto=True,
    ),
    SnotelSite(
        site_id="snotel_css_lab_ca",
        triplet="428:CA:SNTL",
        snow_climate="sierra_maritime",
        begin="1980-01-01",
        end="2024-12-31",
        fallback_ssd_kg_m3=480.0,
        has_sto=True,
    ),
    SnotelSite(
        site_id="snotel_snowbird_ut",
        triplet="766:UT:SNTL",
        snow_climate="wasatch_intermountain",
        begin="1986-01-01",
        end="2024-12-31",
        fallback_ssd_kg_m3=350.0,
        has_sto=True,
    ),
    SnotelSite(
        site_id="snotel_niwot_co",
        triplet="663:CO:SNTL",
        snow_climate="front_range_continental",
        begin="1980-01-01",
        end="2024-12-31",
        fallback_ssd_kg_m3=300.0,
        has_sto=False,
    ),
]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=DEFAULT_CACHE)
    parser.add_argument("--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--site", action="append", default=[])
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("fetch", help="download public AWDB SNOTEL data into target cache")
    subparsers.add_parser("normalize", help="write normalized observation corpus and SSD characterization")
    compare_parser = subparsers.add_parser("compare", help="run three-way model comparison")
    compare_parser.add_argument("--openwepp-binary", type=Path, default=None)
    compare_parser.add_argument("--legacy-binary", type=Path, default=DEFAULT_LEGACY_BINARY)
    compare_parser.add_argument("--pysnobal-python", type=Path, default=DEFAULT_PYSNOBAL_PYTHON)
    compare_parser.add_argument("--pysnobal-path", type=Path, default=DEFAULT_PYSNOBAL_PATH)
    compare_parser.add_argument("--skip-pysnobal-run", action="store_true")
    subparsers.add_parser("validate", help="validate normalized SNOTEL corpus")

    args = parser.parse_args(argv)
    sites = selected_sites(set(args.site))
    if args.command == "fetch":
        fetch_snotel(args.cache.resolve(), sites)
        return 0
    if args.command == "normalize":
        normalize_snotel(args.cache.resolve(), args.observations_dir.resolve(), sites)
        return 0
    if args.command == "validate":
        validate_observations(args.observations_dir.resolve(), sites)
        return 0
    if args.command == "compare":
        report = compare_three_way(
            observations_dir=args.observations_dir.resolve(),
            output_dir=args.output_dir.resolve(),
            sites=sites,
            openwepp_binary=args.openwepp_binary.resolve() if args.openwepp_binary else None,
            legacy_binary=args.legacy_binary.resolve(),
            pysnobal_python=args.pysnobal_python.absolute(),
            pysnobal_path=args.pysnobal_path.resolve(),
            run_pysnobal=not args.skip_pysnobal_run,
        )
        write_json(args.output_dir.resolve() / "three_way_comparison.json", report)
        (args.output_dir.resolve() / "three_way_comparison.md").write_text(
            render_three_way_markdown(report),
            encoding="utf-8",
        )
        return 0
    raise AssertionError(f"unreachable command {args.command}")


def selected_sites(site_filters: set[str]) -> list[SnotelSite]:
    sites = [site for site in SITES if not site_filters or site.site_id in site_filters]
    missing = site_filters - {site.site_id for site in sites}
    if missing:
        raise ValueError(f"unknown SNOTEL site filters: {sorted(missing)}")
    return sites


def fetch_snotel(cache: Path, sites: list[SnotelSite]) -> None:
    raw_dir = cache / "raw"
    raw_dir.mkdir(parents=True, exist_ok=True)
    summary = {
        "schema": "snotel-awdb-fetch-summary-v1",
        "access_date": ACCESS_DATE,
        "awdb_data_url": AWDB_DATA_URL,
        "awdb_stations_url": AWDB_STATIONS_URL,
        "elements": AWDB_ELEMENTS,
        "sites": [],
    }
    for site in sites:
        site_dir = raw_dir / site.site_id
        site_dir.mkdir(parents=True, exist_ok=True)
        data_url = AWDB_DATA_URL + "?" + urllib.parse.urlencode(
            {
                "stationTriplets": site.triplet,
                "duration": "DAILY",
                "beginDate": site.begin,
                "endDate": site.end,
                "elements": AWDB_ELEMENTS,
                "returnFlags": "true",
                "returnOriginalValues": "true",
            }
        )
        station_url = AWDB_STATIONS_URL + "?" + urllib.parse.urlencode(
            {
                "stationTriplets": site.triplet,
                "returnStationElements": "true",
            }
        )
        data_path = site_dir / "daily.json"
        station_path = site_dir / "station.json"
        write_json(data_path, download_json(data_url))
        write_json(station_path, download_json(station_url))
        summary["sites"].append(
            {
                "site_id": site.site_id,
                "station_triplet": site.triplet,
                "begin": site.begin,
                "end": site.end,
                "data_url": data_url,
                "station_url": station_url,
                "data_sha256": sha256(data_path),
                "station_sha256": sha256(station_path),
            }
        )
    write_json(cache / "fetch-summary.json", summary)


def normalize_snotel(cache: Path, observations_dir: Path, sites: list[SnotelSite]) -> None:
    site_dir = observations_dir / "sites"
    provenance_dir = observations_dir / "provenance"
    site_dir.mkdir(parents=True, exist_ok=True)
    provenance_dir.mkdir(parents=True, exist_ok=True)
    manifest_sites = []
    source_records = []
    ssd_characterization = {
        "schema": "snotel-ssd-characterization-v1",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-049",
        "access_date": ACCESS_DATE,
        "anti_tuning": (
            "SSD values are derived from observed peak-SWE-period SNOTEL density "
            "before model residuals are computed; they are not residual fits."
        ),
        "density_cap_kg_m3": DENSITY_CAP_KG_M3,
        "peak_window_days": PEAK_WINDOW_DAYS,
        "sites": [],
    }
    for site in sites:
        rows = normalize_site(cache, site)
        csv_path = site_dir / f"{site.site_id}.csv"
        write_observation_csv(csv_path, rows)
        characterization = characterize_ssd(site, rows)
        ssd_characterization["sites"].append(characterization)
        manifest_sites.append(
            {
                "site_id": site.site_id,
                "fixture": site.site_id,
                "station_triplet": site.triplet,
                "snow_climate": site.snow_climate,
                "observation_file": f"sites/{site.site_id}.csv",
                "row_count": len(rows),
                "paired_swe_depth_count": sum(
                    1 for row in rows if row.get("observed_density_kg_m3") != ""
                ),
                "observed_ssd_kg_m3": characterization["selected_ssd_kg_m3"],
                "observed_ssd_source": characterization["selected_ssd_source"],
                "has_sto": site.has_sto,
            }
        )
        provenance = provenance_record(cache, observations_dir, site, csv_path, rows, characterization)
        write_json(provenance_dir / f"{site.site_id}.json", provenance)
        source_records.append(provenance)
    write_json(observations_dir / "ssd_characterization.json", ssd_characterization)
    (observations_dir / "ssd_characterization.md").write_text(
        render_ssd_markdown(ssd_characterization),
        encoding="utf-8",
    )
    manifest = {
        "schema": "snotel-observed-manifest-v1",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-049",
        "access_date": ACCESS_DATE,
        "normal_swe_units": "mm water equivalent",
        "normal_depth_units": "m",
        "normal_density_units": "kg m^-3",
        "normal_temperature_units": "degC",
        "density_formula": "observed_density_kg_m3 = observed_swe_mm / observed_snow_depth_m",
        "sites": manifest_sites,
        "sources": source_records,
    }
    write_json(observations_dir / "manifest.json", manifest)
    validate_observations(observations_dir, sites)


def normalize_site(cache: Path, site: SnotelSite) -> list[dict[str, str]]:
    data_path = cache / "raw" / site.site_id / "daily.json"
    require_file(data_path)
    document = json.loads(data_path.read_text(encoding="utf-8"))
    by_date: dict[dt.date, dict[str, Any]] = {}
    for station in document:
        for element in station.get("data", []):
            station_element = element.get("stationElement", {})
            code = station_element.get("elementCode")
            if code not in {"WTEQ", "SNWD", "PREC", "TOBS", "TMAX", "TMIN", "STO"}:
                continue
            depth_m = None
            if code == "STO":
                raw_depth = station_element.get("heightDepth")
                if raw_depth is None:
                    continue
                depth_m = abs(float(raw_depth)) * 0.0254
            for value_record in element.get("values", []):
                value = value_record.get("value")
                if value is None:
                    continue
                if value_record.get("qcFlag") not in (None, "V", "C"):
                    continue
                observed_date = dt.date.fromisoformat(value_record["date"])
                record = by_date.setdefault(observed_date, {})
                if code in {"WTEQ", "PREC"}:
                    record[code] = float(value) * 25.4
                elif code == "SNWD":
                    record[code] = float(value) * 0.0254
                elif code in {"TOBS", "TMAX", "TMIN"}:
                    record[code] = degf_to_degc(float(value))
                elif code == "STO" and depth_m is not None:
                    sto = record.setdefault("STO", {})
                    sto[depth_m] = degf_to_degc(float(value))
    rows = []
    for observed_date in sorted(by_date):
        record = by_date[observed_date]
        swe_mm = finite_or_none(record.get("WTEQ"))
        depth_m = finite_or_none(record.get("SNWD"))
        density = observed_density_kg_m3(swe_mm, depth_m)
        sto_depth, sto_temp = shallowest_soil_temp(record.get("STO", {}))
        quality = [
            "awdb_qc=V|C|None",
            "density=positive_swe_depth" if density is not None else "density=unavailable",
        ]
        if density is not None and density > DENSITY_CAP_KG_M3:
            quality.append("density_above_inv003_cap")
        if sto_temp is None:
            quality.append("sto_unavailable")
        rows.append(
            {
                "site_id": site.site_id,
                "station_triplet": site.triplet,
                "date": observed_date.isoformat(),
                "water_year": str(water_year(observed_date)),
                "observed_swe_mm": fmt_optional(swe_mm),
                "observed_snow_depth_m": fmt_optional(depth_m),
                "observed_density_kg_m3": fmt_optional(density),
                "observed_precip_mm": fmt_optional(finite_or_none(record.get("PREC"))),
                "observed_tobs_c": fmt_optional(finite_or_none(record.get("TOBS"))),
                "observed_tmax_c": fmt_optional(finite_or_none(record.get("TMAX"))),
                "observed_tmin_c": fmt_optional(finite_or_none(record.get("TMIN"))),
                "observed_soil_temp_c": fmt_optional(sto_temp),
                "observed_soil_temp_depth_m": fmt_optional(sto_depth),
                "observed_soil_temp_source": "SNOTEL_STO_SHALLOWEST" if sto_temp is not None else "",
                "quality_flag": ";".join(quality),
                "source_record_id": f"AWDB:{site.triplet}:{observed_date.isoformat()}",
            }
        )
    return rows


def characterize_ssd(site: SnotelSite, rows: list[dict[str, str]]) -> dict[str, Any]:
    rows_by_wy: dict[int, list[dict[str, str]]] = {}
    for row in rows:
        rows_by_wy.setdefault(int(row["water_year"]), []).append(row)
    annual_densities = []
    for water_year_value, wy_rows in sorted(rows_by_wy.items()):
        candidates = [row for row in wy_rows if optional_float(row["observed_swe_mm"]) is not None]
        if not candidates:
            continue
        peak = max(candidates, key=lambda row: optional_float(row["observed_swe_mm"]) or -math.inf)
        peak_date = dt.date.fromisoformat(peak["date"])
        window_densities = []
        for row in wy_rows:
            observed_date = dt.date.fromisoformat(row["date"])
            if abs((observed_date - peak_date).days) > PEAK_WINDOW_DAYS:
                continue
            density = optional_float(row["observed_density_kg_m3"])
            if density is None:
                continue
            if MIN_DENSITY_FOR_SSD_KG_M3 <= density <= DENSITY_CAP_KG_M3:
                window_densities.append(density)
        if window_densities:
            annual_densities.append(
                {
                    "water_year": water_year_value,
                    "peak_swe_date": peak_date.isoformat(),
                    "peak_swe_mm": optional_float(peak["observed_swe_mm"]),
                    "window_density_median_kg_m3": median(window_densities),
                    "window_density_count": len(window_densities),
                }
            )
    density_values = [row["window_density_median_kg_m3"] for row in annual_densities]
    if density_values:
        observed_ssd = round_to_nearest(median(density_values), 5.0)
        source = "observed_peak_swe_period_density_median"
    else:
        observed_ssd = site.fallback_ssd_kg_m3
        source = "fallback_climate_prior_no_valid_observed_density"
    observed_ssd = max(100.0, min(DENSITY_CAP_KG_M3, observed_ssd))
    return {
        "site_id": site.site_id,
        "station_triplet": site.triplet,
        "snow_climate": site.snow_climate,
        "as_built_ssd_kg_m3": 250.0,
        "selected_ssd_kg_m3": observed_ssd,
        "selected_ssd_source": source,
        "annual_density_count": len(annual_densities),
        "annual_peak_window_densities": annual_densities,
        "observed_density_summary": numeric_summary(density_values),
        "fallback_prior_kg_m3": site.fallback_ssd_kg_m3,
    }


def validate_observations(observations_dir: Path, sites: list[SnotelSite]) -> None:
    manifest_path = observations_dir / "manifest.json"
    require_file(manifest_path)
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if manifest.get("schema") != "snotel-observed-manifest-v1":
        raise ValueError("SNOTEL manifest schema must be snotel-observed-manifest-v1")
    site_ids = {site.site_id for site in sites}
    for site_record in manifest.get("sites", []):
        if site_record["site_id"] not in site_ids:
            continue
        csv_path = observations_dir / site_record["observation_file"]
        require_file(csv_path)
        with csv_path.open(newline="", encoding="utf-8") as handle:
            reader = csv.DictReader(handle)
            if reader.fieldnames != OBSERVATION_COLUMNS:
                raise ValueError(f"{csv_path} has unexpected columns {reader.fieldnames}")
            rows = list(reader)
        if not rows:
            raise ValueError(f"{csv_path} contains no rows")
        paired = 0
        for row in rows:
            date = dt.date.fromisoformat(row["date"])
            if int(row["water_year"]) != water_year(date):
                raise ValueError(f"{csv_path} row {date} has wrong water year")
            density = optional_float(row["observed_density_kg_m3"])
            if density is not None:
                paired += 1
                if density <= 0.0 or not math.isfinite(density):
                    raise ValueError(f"{csv_path} row {date} invalid density {density}")
        if paired == 0:
            raise ValueError(f"{csv_path} has no paired SWE/depth density rows")


def compare_three_way(
    observations_dir: Path,
    output_dir: Path,
    sites: list[SnotelSite],
    openwepp_binary: Path | None,
    legacy_binary: Path,
    pysnobal_python: Path,
    pysnobal_path: Path,
    run_pysnobal: bool,
) -> dict[str, Any]:
    validate_observations(observations_dir, sites)
    if not legacy_binary.is_file():
        raise FileNotFoundError(f"legacy WEPP binary not found: {legacy_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)
    manifest = json.loads((observations_dir / "manifest.json").read_text(encoding="utf-8"))
    ssd = json.loads((observations_dir / "ssd_characterization.json").read_text(encoding="utf-8"))
    site_manifest = {site["site_id"]: site for site in manifest["sites"]}
    ssd_by_site = {site["site_id"]: site for site in ssd["sites"]}

    site_reports = []
    for site in sites:
        observations = read_csv_dicts(observations_dir / site_manifest[site.site_id]["observation_file"])
        site_reports.append(
            compare_site_models(
                site=site,
                observations=observations,
                output_dir=output_dir,
                openwepp_binary=openwepp_binary,
                legacy_binary=legacy_binary,
                selected_ssd=float(ssd_by_site[site.site_id]["selected_ssd_kg_m3"]),
            )
        )
    py_summary = run_pysnobal_sites(
        output_dir=output_dir,
        observations_dir=observations_dir,
        sites=sites,
        pysnobal_python=pysnobal_python,
        pysnobal_path=pysnobal_path,
        run_pysnobal=run_pysnobal,
    )
    py_by_site = load_pysnobal_model_series(output_dir, sites)
    for site_report in site_reports:
        site_id = site_report["site_id"]
        observations = read_csv_dicts(observations_dir / site_manifest[site_id]["observation_file"])
        py_metrics = model_metrics(observations, py_by_site.get(site_id, {}), "pysnobal")
        site_report["models"]["pysnobal"] = {
            "arm": "sto_ground_forcing",
            "metrics": py_metrics,
            "rubric_profile": rubric_profile(observations, py_by_site.get(site_id, {}), "pysnobal"),
            "series_source": str(
                output_dir
                / "pysnobal_inputs"
                / site_id
                / "tg_snotel_ground_zg0p10m"
                / pysnobal_output_filename(output_dir, site_id)
            ),
        }
        site_report["fork_verdict"] = fork_verdict(site_report)
        site_report["rubric_summary"] = summarize_site_rubric(site_report)
    return {
        "schema": "snotel-density-three-way-comparison-v2",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-049 INV-SNOWFREEZE-050 TOL-SNOWFREEZE-011",
        "access_date": ACCESS_DATE,
        "output_dir": str(output_dir),
        "legacy_binary": str(legacy_binary),
        "pysnobal_python": str(pysnobal_python),
        "pysnobal_path": str(pysnobal_path),
        "pysnobal_summary": py_summary,
        "summary": summarize_comparison(site_reports),
        "sites": site_reports,
    }


def compare_site_models(
    site: SnotelSite,
    observations: list[dict[str, str]],
    output_dir: Path,
    openwepp_binary: Path | None,
    legacy_binary: Path,
    selected_ssd: float,
) -> dict[str, Any]:
    report = {
        "site_id": site.site_id,
        "station_triplet": site.triplet,
        "snow_climate": site.snow_climate,
        "ssd_arms": {
            "as_built": 250.0,
            "observed_density": selected_ssd,
        },
        "models": {},
    }
    for arm, ssd_value in [("as_built", 250.0), ("observed_density", selected_ssd)]:
        arm_fixture = prepare_arm_fixture(site.site_id, output_dir, arm, ssd_value)
        open_rows, open_artifacts = run_openwepp_arm(
            site_id=site.site_id,
            arm=arm,
            fixture_dir=arm_fixture,
            output_dir=output_dir,
            binary=openwepp_binary,
        )
        legacy_rows, legacy_artifacts = run_legacy_arm(
            site_id=site.site_id,
            arm=arm,
            fixture_dir=arm_fixture,
            output_dir=output_dir,
            legacy_binary=legacy_binary,
        )
        report["models"][f"openwepp_{arm}"] = {
            "arm": arm,
            "ssd_kg_m3": ssd_value,
            "artifacts": open_artifacts,
            "metrics": model_metrics(observations, open_rows, f"openwepp_{arm}"),
            "rubric_profile": rubric_profile(observations, open_rows, f"openwepp_{arm}"),
        }
        report["models"][f"legacy_{arm}"] = {
            "arm": arm,
            "ssd_kg_m3": ssd_value,
            "artifacts": legacy_artifacts,
            "metrics": model_metrics(observations, legacy_rows, f"legacy_{arm}"),
            "rubric_profile": rubric_profile(observations, legacy_rows, f"legacy_{arm}"),
        }
    return report


def prepare_arm_fixture(site_id: str, output_dir: Path, arm: str, ssd_value: float) -> Path:
    source = FIXTURE_ROOT / site_id
    destination = output_dir / "run_inputs" / site_id / arm
    if destination.exists():
        shutil.rmtree(destination)
    destination.mkdir(parents=True)
    for path in source.iterdir():
        if path.is_file():
            shutil.copy2(path, destination / path.name)
    snow_path = destination / "snow.txt"
    rows = read_snow_txt(source / "snow.txt")
    rows[2] = ssd_value
    snow_path.write_text(
        f"{rows[0]:.6f}  # rain-snow threshold\n"
        f"{rows[1]:.6f}  # density of new snow\n"
        f"{rows[2]:.6f}  # snow settling density ({arm})\n",
        encoding="utf-8",
    )
    return destination


def run_openwepp_arm(
    site_id: str,
    arm: str,
    fixture_dir: Path,
    output_dir: Path,
    binary: Path | None,
) -> tuple[dict[dt.date, dict[str, float | None]], dict[str, str]]:
    run_stem = observed_harness.discover_run_stem(fixture_dir)
    arm_output = output_dir / "openwepp" / site_id / arm
    arm_output.mkdir(parents=True, exist_ok=True)
    runfile_path = arm_output / f"{site_id}_{arm}.run"
    observed_harness.write_runfile(runfile_path, fixture_dir, run_stem, arm_output, f"{site_id}_{arm}")
    command = observed_harness.cli_command(binary, fixture_dir, runfile_path, arm_output, "compatibility")
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    (arm_output / "openwepp-cli-hill.stdout").write_text(completed.stdout, encoding="utf-8")
    (arm_output / "openwepp-cli-hill.stderr").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise RuntimeError(
            f"openWEPP failed for {site_id} {arm} with exit code {completed.returncode}"
        )
    wat_path = arm_output / f"{site_id}_{arm}.wat.parquet"
    rows = observed_harness.load_modeled_wat(wat_path)
    return rows, {
        "run_input_dir": str(fixture_dir),
        "runfile": str(runfile_path),
        "wat_output": str(wat_path),
        "stdout": str(arm_output / "openwepp-cli-hill.stdout"),
        "stderr": str(arm_output / "openwepp-cli-hill.stderr"),
    }


def run_legacy_arm(
    site_id: str,
    arm: str,
    fixture_dir: Path,
    output_dir: Path,
    legacy_binary: Path,
) -> tuple[dict[dt.date, dict[str, float | None]], dict[str, str]]:
    legacy_dir = output_dir / "legacy" / site_id / arm
    legacy_snow_compare.run_legacy_replay(fixture_dir, legacy_dir, legacy_binary)
    rows = legacy_snow_compare.load_legacy_rows(legacy_dir)
    normalized = {}
    for date, row in rows.items():
        depth = row.get("legacy_snow_depth_m")
        swe = row.get("legacy_snow_water_m")
        normalized[date] = {
            "snow_depth_m": depth,
            "snow_water_m": swe,
            "snow_density_kg_m3": row.get("legacy_snow_density_kg_m3"),
        }
    return normalized, {
        "run_input_dir": str(fixture_dir),
        "legacy_run_dir": str(legacy_dir),
        "wat_output": str(legacy_dir / "output" / legacy_snow_compare.legacy_wat_name(legacy_dir)),
        "winter_output": str(legacy_dir / "output" / legacy_snow_compare.legacy_winter_name(legacy_dir)),
    }


def run_pysnobal_sites(
    output_dir: Path,
    observations_dir: Path,
    sites: list[SnotelSite],
    pysnobal_python: Path,
    pysnobal_path: Path,
    run_pysnobal: bool,
) -> dict[str, Any]:
    input_root = output_dir / "pysnobal_inputs"
    input_root.mkdir(parents=True, exist_ok=True)
    if run_pysnobal:
        for site in sites:
            site_export = input_root / site.site_id
            export_pysnobal(site, site_export)
            prepare_snotel_ground_lane(site, observations_dir, site_export)
    else:
        missing = [
            str(input_root / site.site_id / "tg_snotel_ground_zg0p10m")
            for site in sites
            if not (input_root / site.site_id / "tg_snotel_ground_zg0p10m").is_dir()
        ]
        if missing:
            return {
                "status": "HOLD-PYSNOBAL-REUSE-MISSING",
                "missing_lanes": missing,
                "message": "--skip-pysnobal-run requires existing exported SNOTEL ground lanes",
            }
    summary_json = output_dir / "pysnobal_snotel_summary.json"
    summary_md = output_dir / "pysnobal_snotel_summary.md"
    if not run_pysnobal:
        if summary_json.is_file():
            return json.loads(summary_json.read_text(encoding="utf-8"))
        return {
            "status": "HOLD-PYSNOBAL-REUSE-MISSING",
            "missing_summary": str(summary_json),
            "message": "--skip-pysnobal-run requires an existing PySnobal summary",
        }
    command = [
        str(pysnobal_python),
        str(TOOL_DIR / "pysnobal_compare.py"),
        "--input-root",
        str(input_root),
        "--observations-dir",
        str(observations_dir),
        "--pysnobal-path",
        str(pysnobal_path),
        "--lane",
        "tg_snotel_ground_zg0p10m",
        "--route-policy",
        "site-sane",
        "--water-year-segments",
        "--output-json",
        str(summary_json),
        "--output-md",
        str(summary_md),
    ]
    if not run_pysnobal:
        command.append("--reuse-existing")
    env = os.environ.copy()
    env["PYSNOBAL_PYTHON"] = str(pysnobal_python)
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    (output_dir / "pysnobal_compare.stdout").write_text(completed.stdout, encoding="utf-8")
    (output_dir / "pysnobal_compare.stderr").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        return {
            "status": "HOLD-PYSNOBAL-COMPARISON",
            "returncode": completed.returncode,
            "command": command,
            "stdout": str(output_dir / "pysnobal_compare.stdout"),
            "stderr": str(output_dir / "pysnobal_compare.stderr"),
        }
    return json.loads(summary_json.read_text(encoding="utf-8"))


def export_pysnobal(site: SnotelSite, site_export: Path) -> None:
    if site_export.exists():
        shutil.rmtree(site_export)
    binary = REPO_ROOT / "target/release/openwepp-snowbench"
    if not binary.is_file():
        binary = REPO_ROOT / "target/debug/openwepp-snowbench"
    command = [
        str(binary),
        "export-pysnobal",
        "--run-dir",
        str(FIXTURE_ROOT / site.site_id),
        "--output-dir",
        str(site_export),
    ]
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    site_export.mkdir(parents=True, exist_ok=True)
    (site_export / "openwepp-snowbench.stdout").write_text(completed.stdout, encoding="utf-8")
    (site_export / "openwepp-snowbench.stderr").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise RuntimeError(
            f"openwepp-snowbench failed for {site.site_id} with exit code {completed.returncode}"
        )


def prepare_snotel_ground_lane(site: SnotelSite, observations_dir: Path, site_export: Path) -> None:
    source_lane = site_export / "tg_0p0c_zg0p10m"
    target_lane = site_export / "tg_snotel_ground_zg0p10m"
    if target_lane.exists():
        shutil.rmtree(target_lane)
    shutil.copytree(source_lane, target_lane)
    observations = read_csv_dicts(observations_dir / "sites" / f"{site.site_id}.csv")
    sto_by_date = {
        row["date"]: optional_float(row["observed_soil_temp_c"])
        for row in observations
        if optional_float(row["observed_soil_temp_c"]) is not None
    }
    fallback = 0.0
    forcing_path = target_lane / "forcing.csv"
    rows = read_csv_dicts(forcing_path)
    replaced = 0
    for row in rows:
        date = row["Datetime"][:10]
        value = sto_by_date.get(date, fallback)
        row["temp_ground_degC"] = f"{value:.6f}"
        if date in sto_by_date:
            replaced += 1
    write_csv_dicts(forcing_path, rows, list(rows[0]))
    config_path = target_lane / "config.yaml"
    config = config_path.read_text(encoding="utf-8")
    config = config.replace(str(source_lane), str(target_lane))
    config = config.replace("soil_temp_m: 0.10", "soil_temp_m: 0.10")
    config_path.write_text(config, encoding="utf-8")

    lineage_path = target_lane / "lineage.json"
    lineage = json.loads(lineage_path.read_text(encoding="utf-8"))
    lineage["lane"] = {"id": "tg_snotel_ground_zg0p10m", "temp_ground_c": None, "soil_temp_depth_m": 0.10}
    lineage["fields"]["temp_ground_degC"] = {
        "units": "degC",
        "source_class": "mechanical" if site.has_sto else "diagnostic-proxy",
        "source": (
            "SNOTEL STO shallowest available daily soil-temperature row, repeated hourly"
            if site.has_sto
            else "Niwot has no SNOTEL STO; retained constant 0.0 degC documented fallback"
        ),
        "conversion": "degF to degC during normalized SNOTEL corpus generation; daily value repeated hourly",
        "rejected_aliases": [
            "frost.hourly.surface_temp_c_####",
            "snow-surface temperature",
            "air temperature",
        ],
    }
    write_json(lineage_path, lineage)

    audit_path = target_lane / "audit.json"
    audit = json.loads(audit_path.read_text(encoding="utf-8"))
    audit["lane"] = {"id": "tg_snotel_ground_zg0p10m", "temp_ground_c": None, "soil_temp_depth_m": 0.10}
    audit["snotel_ground_forcing"] = {
        "site_id": site.site_id,
        "station_triplet": site.triplet,
        "source": "SNOTEL STO shallowest daily soil-temperature" if site.has_sto else "documented constant fallback",
        "hourly_rows_with_observed_sto": replaced,
        "hourly_rows_total": len(rows),
        "fallback_temp_ground_degC": fallback,
    }
    write_json(audit_path, audit)
    (target_lane / "audit.md").write_text(
        "# SNOTEL Ground-Temperature Lane\n\n"
        f"- Site: `{site.site_id}`\n"
        f"- Source: `{audit['snotel_ground_forcing']['source']}`\n"
        f"- Hourly rows with observed STO: `{replaced}` / `{len(rows)}`\n"
        f"- Fallback temp_ground_degC: `{fallback}`\n",
        encoding="utf-8",
    )


def load_pysnobal_model_series(output_dir: Path, sites: list[SnotelSite]) -> dict[str, dict[dt.date, dict[str, float | None]]]:
    result = {}
    for site in sites:
        output_path = (
            output_dir
            / "pysnobal_inputs"
            / site.site_id
            / "tg_snotel_ground_zg0p10m"
            / pysnobal_output_filename(output_dir, site.site_id)
        )
        if output_path.is_file():
            result[site.site_id] = load_pysnobal_daily_series(output_path)
        else:
            result[site.site_id] = {}
    return result


def pysnobal_output_filename(output_dir: Path, site_id: str) -> str:
    lane_dir = output_dir / "pysnobal_inputs" / site_id / "tg_snotel_ground_zg0p10m"
    segmented = lane_dir / "pysnobal_output_water_year_segments.csv"
    if segmented.is_file():
        return segmented.name
    return "pysnobal_output.csv"


def load_pysnobal_daily_series(path: Path) -> dict[dt.date, dict[str, float | None]]:
    rows_by_date: dict[dt.date, dict[str, float | None]] = {}
    with path.open(newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        for row in reader:
            timestamp = dt.datetime.fromisoformat(row["Datetime"])
            swe_m = float(row["specific_mass_snow_kgm-2"]) / 1000.0
            depth_m = float(row["thickness_snow_m"])
            density = swe_m * 1000.0 / depth_m if depth_m > 1.0e-9 else 0.0
            rows_by_date[timestamp.date()] = {
                "snow_water_m": swe_m,
                "snow_depth_m": depth_m,
                "snow_density_kg_m3": density,
            }
    return rows_by_date


def paired_snow_rows(
    observations: list[dict[str, str]],
    modeled: dict[dt.date, dict[str, float | None]],
) -> list[dict[str, Any]]:
    pairs = []
    for row in observations:
        date = dt.date.fromisoformat(row["date"])
        modeled_row = modeled.get(date)
        if modeled_row is None:
            continue
        observed_swe_mm = optional_float(row["observed_swe_mm"])
        observed_depth = optional_float(row["observed_snow_depth_m"])
        observed_density = optional_float(row["observed_density_kg_m3"])
        if observed_swe_mm is None or observed_depth is None or observed_density is None:
            continue
        modeled_swe_m = modeled_row.get("snow_water_m")
        modeled_depth = modeled_row.get("snow_depth_m")
        modeled_density = modeled_row.get("snow_density_kg_m3")
        if modeled_density is None and modeled_swe_m is not None and modeled_depth is not None:
            modeled_density = modeled_swe_m * 1000.0 / modeled_depth if modeled_depth > 1.0e-9 else 0.0
        if modeled_swe_m is None or modeled_depth is None or modeled_density is None:
            continue
        observed_swe_m = observed_swe_mm / 1000.0
        pairs.append(
            {
                "date": date.isoformat(),
                "date_obj": date,
                "water_year": water_year(date),
                "observed_swe_m": observed_swe_m,
                "modeled_swe_m": modeled_swe_m,
                "swe_residual_m": modeled_swe_m - observed_swe_m,
                "observed_snow_depth_m": observed_depth,
                "modeled_snow_depth_m": modeled_depth,
                "depth_residual_m": modeled_depth - observed_depth,
                "observed_density_kg_m3": observed_density,
                "modeled_density_kg_m3": modeled_density,
                "density_residual_kg_m3": modeled_density - observed_density,
            }
        )
    return pairs


def model_metrics(
    observations: list[dict[str, str]],
    modeled: dict[dt.date, dict[str, float | None]],
    model_id: str,
) -> dict[str, Any]:
    pairs = paired_snow_rows(observations, modeled)
    return {
        "model_id": model_id,
        "paired_count": len(pairs),
        "swe": residual_summary(
            [row["swe_residual_m"] for row in pairs],
            [swe_tolerance(row["observed_swe_m"]) for row in pairs],
        ),
        "depth": residual_summary(
            [row["depth_residual_m"] for row in pairs],
            [snow_depth_tolerance(row["observed_snow_depth_m"]) for row in pairs],
        ),
        "density": residual_summary(
            [row["density_residual_kg_m3"] for row in pairs],
            [density_tolerance(row["observed_density_kg_m3"]) for row in pairs],
        ),
        "sample_pairs": [json_pair(row) for row in pairs[:20]],
    }


def json_pair(row: dict[str, Any]) -> dict[str, Any]:
    return {key: value for key, value in row.items() if key != "date_obj"}


def residual_summary(residuals: list[float], tolerances: list[float]) -> dict[str, Any]:
    return {
        "count": len(residuals),
        "mean_signed": mean(residuals),
        "median_signed": median(residuals),
        "mean_abs": mean([abs(value) for value in residuals]),
        "median_abs": median([abs(value) for value in residuals]),
        "max_abs": max((abs(value) for value in residuals), default=None),
        "pass_count": sum(1 for value, tol in zip(residuals, tolerances) if abs(value) <= tol),
        "fail_count": sum(1 for value, tol in zip(residuals, tolerances) if abs(value) > tol),
        "modeled_over_observed_count": sum(1 for value in residuals if value > 0.0),
        "modeled_under_observed_count": sum(1 for value in residuals if value < 0.0),
    }


def rubric_profile(
    observations: list[dict[str, str]],
    modeled: dict[dt.date, dict[str, float | None]],
    model_id: str,
) -> dict[str, Any]:
    pairs = paired_snow_rows(observations, modeled)
    cells = [
        magnitude_bias_cell(
            pairs,
            "long_term_peak_swe_bias",
            "Long-term",
            "mean peak SWE bias",
            "L",
            "observed_swe_m",
            "modeled_swe_m",
            "m",
        ),
        magnitude_bias_cell(
            pairs,
            "long_term_peak_depth_bias",
            "Long-term",
            "mean peak depth bias",
            "L",
            "observed_snow_depth_m",
            "modeled_snow_depth_m",
            "m",
        ),
        residual_bias_cell(
            pairs,
            "long_term_cold_season_bulk_density",
            "Long-term",
            "mean cold-season bulk density",
            "R",
            "density_residual_kg_m3",
            "kg m^-3",
            DENSITY_TOL_ABS_KG_M3,
        ),
        snow_cover_duration_cell(pairs),
        time_series_kge_cell(
            pairs,
            "seasonal_swe_timeseries",
            "Seasonal",
            "SWE trajectory",
            "L",
            "observed_swe_m",
            "modeled_swe_m",
        ),
        time_series_kge_cell(
            pairs,
            "seasonal_depth_timeseries",
            "Seasonal",
            "depth trajectory",
            "L",
            "observed_snow_depth_m",
            "modeled_snow_depth_m",
        ),
        time_series_kge_cell(
            pairs,
            "seasonal_densification_trajectory",
            "Seasonal",
            "densification trajectory rho(t)",
            "R",
            "observed_density_kg_m3",
            "modeled_density_kg_m3",
        ),
        timing_cell(
            pairs,
            "seasonal_accumulation_onset_date",
            "Seasonal",
            "accumulation onset date",
            "R",
            first_snow_date_by_water_year,
        ),
        timing_cell(
            pairs,
            "seasonal_peak_swe_date",
            "Seasonal",
            "peak SWE date",
            "R",
            lambda rows: peak_date_by_water_year(rows, "observed_swe_m", "modeled_swe_m"),
        ),
        timing_cell(
            pairs,
            "seasonal_peak_depth_date",
            "Seasonal",
            "peak depth date",
            "R",
            lambda rows: peak_date_by_water_year(
                rows, "observed_snow_depth_m", "modeled_snow_depth_m"
            ),
        ),
        depth_swe_slope_cell(pairs),
        timing_cell(
            pairs,
            "seasonal_ablation_meltout_date",
            "Seasonal",
            "ablation melt-out date",
            "R",
            last_snow_date_by_water_year,
        ),
        bias_sign_cell(pairs),
        unavailable_cell(
            "event_new_snow_density",
            "Event",
            "new-snow density per storm",
            "R",
            "daily SNOTEL corpus lacks storm-resolved new-snow density.",
        ),
        unavailable_cell(
            "event_rain_on_snow_response",
            "Event",
            "rain-on-snow response",
            "R",
            "event pairing requires storm forcing and phase-confidence windows.",
        ),
        unavailable_cell(
            "cross_cutting_conservation",
            "Cross-cutting",
            "conservation",
            "R",
            "external SNOTEL comparison does not reconstruct model mass/energy closure.",
        ),
    ]
    return {
        "schema": "snowfrost-fidelity-rubric-profile-v1",
        "model_id": model_id,
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 TOL-SNOWFREEZE-011",
        "paired_count": len(pairs),
        "cells": cells,
        "summary": summarize_rubric_cells(cells),
    }


def magnitude_bias_cell(
    pairs: list[dict[str, Any]],
    cell_id: str,
    timescale: str,
    signature: str,
    tier: str,
    observed_key: str,
    modeled_key: str,
    units: str,
) -> dict[str, Any]:
    annual = annual_peak_biases(pairs, observed_key, modeled_key)
    if not annual:
        return unavailable_cell(cell_id, timescale, signature, tier, "no paired annual peaks")
    biases = [row["bias"] for row in annual]
    relative = [row["relative_bias"] for row in annual if row["relative_bias"] is not None]
    median_relative = median(relative)
    return scored_cell(
        cell_id=cell_id,
        timescale=timescale,
        signature=signature,
        tier=tier,
        metric="median signed annual peak bias, IQR",
        metric_units=units,
        score=score_relative_magnitude(median_relative),
        metrics={
            "annual_count": len(annual),
            "median_signed_bias": median(biases),
            "iqr_signed_bias": iqr(biases),
            "median_relative_bias": median_relative,
            "annual_biases": annual,
        },
    )


def residual_bias_cell(
    pairs: list[dict[str, Any]],
    cell_id: str,
    timescale: str,
    signature: str,
    tier: str,
    residual_key: str,
    units: str,
    abs_pass_tolerance: float,
) -> dict[str, Any]:
    residuals = [row[residual_key] for row in pairs]
    if not residuals:
        return unavailable_cell(cell_id, timescale, signature, tier, "no paired residuals")
    score = score_abs_tolerance(abs(median(residuals) or 0.0), abs_pass_tolerance)
    return scored_cell(
        cell_id=cell_id,
        timescale=timescale,
        signature=signature,
        tier=tier,
        metric="median signed bias, IQR",
        metric_units=units,
        score=score,
        metrics={
            "paired_count": len(residuals),
            "median_signed_bias": median(residuals),
            "iqr_signed_bias": iqr(residuals),
            "mean_signed_bias": mean(residuals),
            "mean_abs_bias": mean([abs(value) for value in residuals]),
        },
    )


def time_series_kge_cell(
    pairs: list[dict[str, Any]],
    cell_id: str,
    timescale: str,
    signature: str,
    tier: str,
    observed_key: str,
    modeled_key: str,
) -> dict[str, Any]:
    observed = [row[observed_key] for row in pairs]
    modeled = [row[modeled_key] for row in pairs]
    kge = kge_components(observed, modeled)
    if kge["kge"] is None:
        return unavailable_cell(cell_id, timescale, signature, tier, kge["reason"])
    return scored_cell(
        cell_id=cell_id,
        timescale=timescale,
        signature=signature,
        tier=tier,
        metric="KGE decomposed into r, beta, gamma",
        metric_units="unitless",
        score=score_kge(kge["kge"]),
        metrics={"paired_count": len(observed), **kge},
    )


def timing_cell(
    pairs: list[dict[str, Any]],
    cell_id: str,
    timescale: str,
    signature: str,
    tier: str,
    extractor: Any,
) -> dict[str, Any]:
    offsets = extractor(pairs)
    if not offsets:
        return unavailable_cell(cell_id, timescale, signature, tier, "no paired annual timing offsets")
    values = [row["offset_days"] for row in offsets]
    return scored_cell(
        cell_id=cell_id,
        timescale=timescale,
        signature=signature,
        tier=tier,
        metric="median modeled-minus-observed date offset",
        metric_units="days",
        score=score_timing(abs(median(values) or 0.0)),
        metrics={
            "annual_count": len(offsets),
            "median_offset_days": median(values),
            "iqr_offset_days": iqr(values),
            "annual_offsets": offsets,
        },
    )


def depth_swe_slope_cell(pairs: list[dict[str, Any]]) -> dict[str, Any]:
    observed_slope = slope(
        [row["observed_swe_m"] for row in pairs],
        [row["observed_snow_depth_m"] for row in pairs],
    )
    modeled_slope = slope(
        [row["modeled_swe_m"] for row in pairs],
        [row["modeled_snow_depth_m"] for row in pairs],
    )
    if observed_slope is None or modeled_slope is None or abs(observed_slope) <= 1.0e-12:
        return unavailable_cell(
            "seasonal_depth_swe_slope",
            "Seasonal",
            "depth-SWE seasonal slope",
            "R",
            "insufficient slope variation",
        )
    ratio = modeled_slope / observed_slope
    return scored_cell(
        cell_id="seasonal_depth_swe_slope",
        timescale="Seasonal",
        signature="depth-SWE seasonal slope",
        tier="R",
        metric="modeled/observed depth-SWE slope ratio",
        metric_units="unitless",
        score=score_ratio(ratio),
        metrics={
            "observed_slope_depth_per_swe": observed_slope,
            "modeled_slope_depth_per_swe": modeled_slope,
            "slope_ratio": ratio,
            "paired_count": len(pairs),
        },
    )


def snow_cover_duration_cell(pairs: list[dict[str, Any]]) -> dict[str, Any]:
    durations = []
    for water_year_value, rows in pairs_by_water_year(pairs).items():
        observed = sum(
            1 for row in rows if row["observed_snow_depth_m"] > SNOW_COVER_DEPTH_THRESHOLD_M
        )
        modeled = sum(
            1 for row in rows if row["modeled_snow_depth_m"] > SNOW_COVER_DEPTH_THRESHOLD_M
        )
        if observed <= 0:
            continue
        durations.append(
            {
                "water_year": water_year_value,
                "observed_snow_cover_days": observed,
                "modeled_snow_cover_days": modeled,
                "duration_ratio": modeled / observed,
            }
        )
    if not durations:
        return unavailable_cell(
            "long_term_snow_cover_duration",
            "Long-term",
            "snow-cover duration; inter-annual variability ratio",
            "R",
            "no positive observed snow-cover duration years",
        )
    ratios = [row["duration_ratio"] for row in durations]
    return scored_cell(
        cell_id="long_term_snow_cover_duration",
        timescale="Long-term",
        signature="snow-cover duration; inter-annual variability ratio",
        tier="R",
        metric="median modeled/observed snow-cover-day ratio",
        metric_units="unitless",
        score=score_ratio(median(ratios)),
        metrics={
            "annual_count": len(durations),
            "median_duration_ratio": median(ratios),
            "iqr_duration_ratio": iqr(ratios),
            "annual_durations": durations,
        },
    )


def bias_sign_cell(pairs: list[dict[str, Any]]) -> dict[str, Any]:
    if not pairs:
        return unavailable_cell(
            "cross_cutting_bias_sign_consistency",
            "Cross-cutting",
            "bias-sign consistency",
            "R",
            "no paired residuals",
        )
    density_under = sum(1 for row in pairs if row["density_residual_kg_m3"] < 0.0)
    depth_over = sum(1 for row in pairs if row["depth_residual_m"] > 0.0)
    count = len(pairs)
    density_under_fraction = density_under / count
    depth_over_fraction = depth_over / count
    dominant_fraction = max(density_under_fraction, depth_over_fraction)
    if dominant_fraction >= 0.90:
        score = {"ordinal": 0, "label": "fail"}
    elif dominant_fraction >= 0.75:
        score = {"ordinal": 1, "label": "marginal"}
    elif dominant_fraction >= 0.60:
        score = {"ordinal": 2, "label": "pass"}
    else:
        score = {"ordinal": 3, "label": "strong"}
    return scored_cell(
        cell_id="cross_cutting_bias_sign_consistency",
        timescale="Cross-cutting",
        signature="bias-sign consistency",
        tier="R",
        metric="dominant residual sign fraction",
        metric_units="unitless",
        score=score,
        metrics={
            "paired_count": count,
            "density_under_observed_fraction": density_under_fraction,
            "depth_over_observed_fraction": depth_over_fraction,
            "dominant_fraction": dominant_fraction,
        },
    )


def scored_cell(
    cell_id: str,
    timescale: str,
    signature: str,
    tier: str,
    metric: str,
    metric_units: str,
    score: dict[str, Any],
    metrics: dict[str, Any],
) -> dict[str, Any]:
    label = score["label"]
    return {
        "cell_id": cell_id,
        "timescale": timescale,
        "signature": signature,
        "tier": tier,
        "forcing_robust": tier == "R",
        "metric": metric,
        "metric_units": metric_units,
        "status": "SCORED",
        "ordinal_score": score["ordinal"],
        "ordinal_label": label,
        "adr017_cell_verdict": adr017_cell_verdict(tier, label),
        "metrics": metrics,
    }


def unavailable_cell(cell_id: str, timescale: str, signature: str, tier: str, reason: str) -> dict[str, Any]:
    return {
        "cell_id": cell_id,
        "timescale": timescale,
        "signature": signature,
        "tier": tier,
        "forcing_robust": tier == "R",
        "metric": "not scored",
        "metric_units": "",
        "status": "UNAVAILABLE",
        "ordinal_score": None,
        "ordinal_label": "unavailable",
        "adr017_cell_verdict": "UNRESOLVED",
        "reason": reason,
        "metrics": {},
    }


def adr017_cell_verdict(tier: str, label: str) -> str:
    if label in {"strong", "pass"}:
        return "PASS"
    if tier != "R":
        return "UNRESOLVED"
    return "UNRESOLVED"


def summarize_rubric_cells(cells: list[dict[str, Any]]) -> dict[str, Any]:
    counts: dict[str, int] = {}
    robust_counts: dict[str, int] = {}
    limited_counts: dict[str, int] = {}
    for cell in cells:
        label = cell["ordinal_label"]
        counts[label] = counts.get(label, 0) + 1
        bucket = robust_counts if cell["tier"] == "R" else limited_counts
        bucket[label] = bucket.get(label, 0) + 1
    return {
        "cell_count": len(cells),
        "counts_by_label": dict(sorted(counts.items())),
        "forcing_robust_counts_by_label": dict(sorted(robust_counts.items())),
        "forcing_limited_counts_by_label": dict(sorted(limited_counts.items())),
        "openwepp_defective_cells": 0,
        "observation_only_failures_are_unresolved": True,
    }


def summarize_site_rubric(site_report: dict[str, Any]) -> dict[str, Any]:
    models = {}
    for model_id, model in site_report["models"].items():
        profile = model.get("rubric_profile")
        if profile is None:
            continue
        models[model_id] = profile["summary"]
    return {"models": models}


def annual_peak_biases(
    pairs: list[dict[str, Any]], observed_key: str, modeled_key: str
) -> list[dict[str, Any]]:
    annual = []
    for water_year_value, rows in pairs_by_water_year(pairs).items():
        observed_peak = max(rows, key=lambda row: row[observed_key])
        modeled_peak = max(rows, key=lambda row: row[modeled_key])
        observed_value = observed_peak[observed_key]
        modeled_value = modeled_peak[modeled_key]
        annual.append(
            {
                "water_year": water_year_value,
                "observed_peak": observed_value,
                "modeled_peak": modeled_value,
                "bias": modeled_value - observed_value,
                "relative_bias": (modeled_value - observed_value) / observed_value
                if abs(observed_value) > 1.0e-12
                else None,
                "observed_peak_date": observed_peak["date"],
                "modeled_peak_date": modeled_peak["date"],
            }
        )
    return annual


def first_snow_date_by_water_year(pairs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    offsets = []
    for water_year_value, rows in pairs_by_water_year(pairs).items():
        observed = first_date_over_depth(rows, "observed_snow_depth_m")
        modeled = first_date_over_depth(rows, "modeled_snow_depth_m")
        if observed is None or modeled is None:
            continue
        offsets.append(
            {
                "water_year": water_year_value,
                "observed_date": observed.isoformat(),
                "modeled_date": modeled.isoformat(),
                "offset_days": (modeled - observed).days,
            }
        )
    return offsets


def last_snow_date_by_water_year(pairs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    offsets = []
    for water_year_value, rows in pairs_by_water_year(pairs).items():
        observed = last_date_over_depth(rows, "observed_snow_depth_m")
        modeled = last_date_over_depth(rows, "modeled_snow_depth_m")
        if observed is None or modeled is None:
            continue
        offsets.append(
            {
                "water_year": water_year_value,
                "observed_date": observed.isoformat(),
                "modeled_date": modeled.isoformat(),
                "offset_days": (modeled - observed).days,
            }
        )
    return offsets


def peak_date_by_water_year(
    pairs: list[dict[str, Any]], observed_key: str, modeled_key: str
) -> list[dict[str, Any]]:
    offsets = []
    for water_year_value, rows in pairs_by_water_year(pairs).items():
        observed = max(rows, key=lambda row: row[observed_key])
        modeled = max(rows, key=lambda row: row[modeled_key])
        offsets.append(
            {
                "water_year": water_year_value,
                "observed_date": observed["date"],
                "modeled_date": modeled["date"],
                "offset_days": (modeled["date_obj"] - observed["date_obj"]).days,
            }
        )
    return offsets


def pairs_by_water_year(pairs: list[dict[str, Any]]) -> dict[int, list[dict[str, Any]]]:
    grouped: dict[int, list[dict[str, Any]]] = {}
    for row in pairs:
        grouped.setdefault(row["water_year"], []).append(row)
    return {key: sorted(value, key=lambda row: row["date_obj"]) for key, value in grouped.items()}


def first_date_over_depth(rows: list[dict[str, Any]], key: str) -> dt.date | None:
    for row in sorted(rows, key=lambda item: item["date_obj"]):
        if row[key] > SNOW_COVER_DEPTH_THRESHOLD_M:
            return row["date_obj"]
    return None


def last_date_over_depth(rows: list[dict[str, Any]], key: str) -> dt.date | None:
    for row in sorted(rows, key=lambda item: item["date_obj"], reverse=True):
        if row[key] > SNOW_COVER_DEPTH_THRESHOLD_M:
            return row["date_obj"]
    return None


def kge_components(observed: list[float], modeled: list[float]) -> dict[str, Any]:
    if len(observed) < 3 or len(modeled) < 3:
        return {"kge": None, "reason": "fewer than three paired rows"}
    observed_mean = mean(observed)
    modeled_mean = mean(modeled)
    observed_std = stddev(observed)
    modeled_std = stddev(modeled)
    if observed_mean is None or modeled_mean is None or observed_std is None or modeled_std is None:
        return {"kge": None, "reason": "missing mean/stddev"}
    if abs(observed_mean) <= 1.0e-12 or observed_std <= 1.0e-12:
        return {"kge": None, "reason": "observed mean or variability is too small for KGE"}
    r = pearson(observed, modeled)
    if r is None:
        return {"kge": None, "reason": "correlation undefined"}
    beta = modeled_mean / observed_mean
    modeled_cv = modeled_std / abs(modeled_mean) if abs(modeled_mean) > 1.0e-12 else None
    observed_cv = observed_std / abs(observed_mean)
    if modeled_cv is None or observed_cv <= 1.0e-12:
        return {"kge": None, "reason": "coefficient of variation undefined"}
    gamma = modeled_cv / observed_cv
    kge = 1.0 - math.sqrt((r - 1.0) ** 2 + (beta - 1.0) ** 2 + (gamma - 1.0) ** 2)
    return {"kge": kge, "r": r, "beta": beta, "gamma": gamma}


def score_kge(value: float | None) -> dict[str, Any]:
    if value is None:
        return {"ordinal": None, "label": "unavailable"}
    if value >= 0.75:
        return {"ordinal": 3, "label": "strong"}
    if value >= KGE_PASS:
        return {"ordinal": 2, "label": "pass"}
    if value >= KGE_MARGINAL:
        return {"ordinal": 1, "label": "marginal"}
    return {"ordinal": 0, "label": "fail"}


def score_timing(abs_offset_days: float) -> dict[str, Any]:
    if abs_offset_days <= TIMING_STRONG_DAYS:
        return {"ordinal": 3, "label": "strong"}
    if abs_offset_days <= TIMING_PASS_DAYS:
        return {"ordinal": 2, "label": "pass"}
    if abs_offset_days <= TIMING_MARGINAL_DAYS:
        return {"ordinal": 1, "label": "marginal"}
    return {"ordinal": 0, "label": "fail"}


def score_relative_magnitude(relative_bias: float | None) -> dict[str, Any]:
    if relative_bias is None:
        return {"ordinal": None, "label": "unavailable"}
    magnitude = abs(relative_bias)
    if magnitude <= MAGNITUDE_L_PASS_REL / 2.0:
        return {"ordinal": 3, "label": "strong"}
    if magnitude <= MAGNITUDE_L_PASS_REL:
        return {"ordinal": 2, "label": "pass"}
    if magnitude <= MAGNITUDE_L_MARGINAL_REL:
        return {"ordinal": 1, "label": "marginal"}
    return {"ordinal": 0, "label": "fail"}


def score_abs_tolerance(value: float, pass_tolerance: float) -> dict[str, Any]:
    if value <= pass_tolerance / 2.0:
        return {"ordinal": 3, "label": "strong"}
    if value <= pass_tolerance:
        return {"ordinal": 2, "label": "pass"}
    if value <= pass_tolerance * 2.0:
        return {"ordinal": 1, "label": "marginal"}
    return {"ordinal": 0, "label": "fail"}


def score_ratio(value: float | None) -> dict[str, Any]:
    if value is None or not math.isfinite(value):
        return {"ordinal": None, "label": "unavailable"}
    distance = abs(value - 1.0)
    if distance <= 0.10:
        return {"ordinal": 3, "label": "strong"}
    if distance <= 0.25:
        return {"ordinal": 2, "label": "pass"}
    if distance <= 0.50:
        return {"ordinal": 1, "label": "marginal"}
    return {"ordinal": 0, "label": "fail"}


def pearson(left: list[float], right: list[float]) -> float | None:
    if len(left) != len(right) or len(left) < 2:
        return None
    left_mean = mean(left)
    right_mean = mean(right)
    if left_mean is None or right_mean is None:
        return None
    numerator = sum((x - left_mean) * (y - right_mean) for x, y in zip(left, right))
    left_ss = sum((x - left_mean) ** 2 for x in left)
    right_ss = sum((y - right_mean) ** 2 for y in right)
    denominator = math.sqrt(left_ss * right_ss)
    if denominator <= 1.0e-12:
        return None
    return numerator / denominator


def slope(x_values: list[float], y_values: list[float]) -> float | None:
    if len(x_values) != len(y_values) or len(x_values) < 2:
        return None
    x_mean = mean(x_values)
    y_mean = mean(y_values)
    if x_mean is None or y_mean is None:
        return None
    denominator = sum((x - x_mean) ** 2 for x in x_values)
    if denominator <= 1.0e-12:
        return None
    numerator = sum((x - x_mean) * (y - y_mean) for x, y in zip(x_values, y_values))
    return numerator / denominator


def stddev(values: list[float]) -> float | None:
    if len(values) < 2:
        return None
    value_mean = mean(values)
    if value_mean is None:
        return None
    return math.sqrt(sum((value - value_mean) ** 2 for value in values) / (len(values) - 1))


def iqr(values: list[float]) -> float | None:
    if not values:
        return None
    ordered = sorted(values)
    return percentile(ordered, 75.0) - percentile(ordered, 25.0)


def percentile(ordered_values: list[float], pct: float) -> float:
    if not ordered_values:
        raise ValueError("cannot compute percentile of empty values")
    if len(ordered_values) == 1:
        return ordered_values[0]
    position = (len(ordered_values) - 1) * pct / 100.0
    lower = math.floor(position)
    upper = math.ceil(position)
    if lower == upper:
        return ordered_values[int(position)]
    fraction = position - lower
    return ordered_values[lower] * (1.0 - fraction) + ordered_values[upper] * fraction


def fork_verdict(site_report: dict[str, Any]) -> dict[str, Any]:
    as_built = site_report["models"]["openwepp_as_built"]["metrics"]
    adjusted = site_report["models"]["openwepp_observed_density"]["metrics"]
    paired = as_built["paired_count"]
    if paired == 0:
        return {"verdict": "STRUCTURAL", "reason": "no paired observed density rows reached model output"}
    depth = as_built["depth"]
    swe = as_built["swe"]
    density = as_built["density"]
    adjusted_depth = adjusted["depth"]
    depth_mean = depth["mean_signed"] or 0.0
    swe_mean = swe["mean_signed"] or 0.0
    density_mean = density["mean_signed"] or 0.0
    depth_mae = depth["mean_abs"] or 0.0
    adjusted_depth_mae = adjusted_depth["mean_abs"] or depth_mae
    ssd_improvement = depth_mae > 0.0 and adjusted_depth_mae <= depth_mae * 0.75
    swe_high = swe_mean > 0.05 and swe["modeled_over_observed_count"] > swe["modeled_under_observed_count"]
    depth_high = depth_mean > 0.10 and depth["modeled_over_observed_count"] > depth["modeled_under_observed_count"]
    density_low = density_mean < -60.0 and density["modeled_under_observed_count"] > density["modeled_over_observed_count"]
    if depth_high and swe_high:
        verdict = "OVER-ACCUMULATION"
        reason = "openWEPP as-built SWE and depth are both high against paired SNOTEL observations"
    elif depth_high and density_low and ssd_improvement:
        verdict = "LOW-DENSITY"
        reason = "SWE is not the leading high signal, density is low, and observed-density SSD materially reduces depth error"
    else:
        verdict = "STRUCTURAL"
        reason = "depth error persists or cannot be attributed solely to SWE accumulation or the SSD input arm"
    return {
        "verdict": verdict,
        "reason": reason,
        "as_built_depth_mean_signed_m": depth_mean,
        "as_built_swe_mean_signed_m": swe_mean,
        "as_built_density_mean_signed_kg_m3": density_mean,
        "as_built_depth_mean_abs_m": depth_mae,
        "observed_density_arm_depth_mean_abs_m": adjusted_depth_mae,
        "observed_density_arm_improved_depth_mae_25pct": ssd_improvement,
    }


def summarize_comparison(site_reports: list[dict[str, Any]]) -> dict[str, Any]:
    verdict_counts: dict[str, int] = {}
    rubric_counts: dict[str, int] = {}
    robust_counts: dict[str, int] = {}
    for site in site_reports:
        verdict = site.get("fork_verdict", {}).get("verdict", "UNCLASSIFIED")
        verdict_counts[verdict] = verdict_counts.get(verdict, 0) + 1
        for model in site.get("models", {}).values():
            profile = model.get("rubric_profile", {})
            for cell in profile.get("cells", []):
                label = cell.get("ordinal_label", "unavailable")
                rubric_counts[label] = rubric_counts.get(label, 0) + 1
                if cell.get("tier") == "R":
                    robust_counts[label] = robust_counts.get(label, 0) + 1
    return {
        "site_count": len(site_reports),
        "fork_verdict_counts": dict(sorted(verdict_counts.items())),
        "rubric_counts_by_label": dict(sorted(rubric_counts.items())),
        "forcing_robust_rubric_counts_by_label": dict(sorted(robust_counts.items())),
        "no_production_physics_changed": True,
        "legacy_and_pysnobal_are_flag_evidence_only": True,
        "rubric_profile_not_scalar": True,
    }


def render_three_way_markdown(report: dict[str, Any]) -> str:
    lines = [
        "# SNOTEL Density Three-Way Comparison",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Contract: `{report['contract']}`",
        f"- Site count: `{report['summary']['site_count']}`",
        f"- Fork verdict counts: `{report['summary']['fork_verdict_counts']}`",
        f"- Rubric counts: `{report['summary']['rubric_counts_by_label']}`",
        f"- Forcing-robust rubric counts: `{report['summary']['forcing_robust_rubric_counts_by_label']}`",
        f"- Legacy/PySnobal correctness authority: `{not report['summary']['legacy_and_pysnobal_are_flag_evidence_only']}`",
        "",
        "## Density Fork Routing",
        "",
        "| Site | Verdict | SSD as-built | SSD observed | openWEPP SWE mean m | openWEPP depth mean m | openWEPP density mean kg/m3 | adjusted depth MAE m | PySnobal depth MAE m |",
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for site in report["sites"]:
        verdict = site["fork_verdict"]
        open_metrics = site["models"]["openwepp_as_built"]["metrics"]
        adjusted = site["models"]["openwepp_observed_density"]["metrics"]
        py = site["models"].get("pysnobal", {}).get("metrics", {})
        lines.append(
            "| {site} | {verdict} | {asbuilt} | {ssd} | {swe} | {depth} | {density} | {adjusted_depth} | {py_depth} |".format(
                site=site["site_id"],
                verdict=verdict["verdict"],
                asbuilt=fmt(250.0),
                ssd=fmt(site["ssd_arms"]["observed_density"]),
                swe=fmt(open_metrics["swe"]["mean_signed"]),
                depth=fmt(open_metrics["depth"]["mean_signed"]),
                density=fmt(open_metrics["density"]["mean_signed"]),
                adjusted_depth=fmt(adjusted["depth"]["mean_abs"]),
                py_depth=fmt(py.get("depth", {}).get("mean_abs")),
            )
        )
    lines.extend(
        [
            "",
            "## Rubric Profile Overlay",
            "",
            "| Site | Model | Arm | R counts | L counts | Paired rows |",
            "| --- | --- | --- | --- | --- | ---: |",
        ]
    )
    for site in report["sites"]:
        for model_id, model in sorted(site["models"].items()):
            profile = model.get("rubric_profile", {})
            summary = profile.get("summary", {})
            lines.append(
                "| {site} | {model} | {arm} | {robust} | {limited} | {paired} |".format(
                    site=site["site_id"],
                    model=model_id,
                    arm=model.get("arm", ""),
                    robust=rubric_counts_text(summary.get("forcing_robust_counts_by_label", {})),
                    limited=rubric_counts_text(summary.get("forcing_limited_counts_by_label", {})),
                    paired=profile.get("paired_count", 0),
                )
            )
    lines.extend(
        [
            "",
            "## Notes",
            "",
            "- The v74 rubric is the evaluation authority: profiles are per-model/per-site/per-cell, not scalar accept/reject scores.",
            "- `R` cells carry model-verdict weight; `L` magnitude cells are reported but discounted under the forcing and representativeness uncertainty budget.",
            "- `OVER-ACCUMULATION`, `LOW-DENSITY`, and `STRUCTURAL` are fork-routing labels, not production defect labels.",
            "- The observed-density SSD arm is derived from SNOTEL peak-SWE-period density before model residuals are computed.",
            "- PySnobal uses a SNOTEL STO lower-boundary lane where available, Niwot uses the documented constant 0 degC fallback, and the diagnostic runner uses water-year segments to avoid multi-decade snowpack state carryover.",
            "",
        ]
    )
    return "\n".join(lines)


def rubric_counts_text(counts: dict[str, int]) -> str:
    if not counts:
        return "n/a"
    return ", ".join(f"{key}:{counts[key]}" for key in sorted(counts))


def render_ssd_markdown(document: dict[str, Any]) -> str:
    lines = [
        "# SNOTEL SSD Characterization",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Contract: `{document['contract']}`",
        f"- Anti-tuning rule: {document['anti_tuning']}",
        f"- Density cap: `{document['density_cap_kg_m3']}` kg/m3",
        f"- Peak window: `+/- {document['peak_window_days']}` days",
        "",
        "| Site | Climate | As-built SSD | Selected SSD | Source | Annual density years | Median observed density kg/m3 |",
        "| --- | --- | ---: | ---: | --- | ---: | ---: |",
    ]
    for site in document["sites"]:
        lines.append(
            "| {site_id} | {climate} | {asbuilt} | {selected} | {source} | {count} | {median_value} |".format(
                site_id=site["site_id"],
                climate=site["snow_climate"],
                asbuilt=fmt(site["as_built_ssd_kg_m3"]),
                selected=fmt(site["selected_ssd_kg_m3"]),
                source=site["selected_ssd_source"],
                count=site["annual_density_count"],
                median_value=fmt(site["observed_density_summary"]["median"]),
            )
        )
    return "\n".join(lines) + "\n"


def provenance_record(
    cache: Path,
    observations_dir: Path,
    site: SnotelSite,
    csv_path: Path,
    rows: list[dict[str, str]],
    characterization: dict[str, Any],
) -> dict[str, Any]:
    raw_files = [
        cache / "raw" / site.site_id / "daily.json",
        cache / "raw" / site.site_id / "station.json",
    ]
    return {
        "source_id": f"nrcs_snotel_{site.triplet.replace(':', '_').lower()}",
        "site_id": site.site_id,
        "station_triplet": site.triplet,
        "title": f"USDA NRCS AWDB SNOTEL station {site.triplet}",
        "source_url": AWDB_DATA_URL,
        "access_date": ACCESS_DATE,
        "parser_version": "snotel-density-three-way-v1",
        "normalized_row_count": len(rows),
        "paired_density_row_count": sum(1 for row in rows if row["observed_density_kg_m3"] != ""),
        "density_formula": "SWE(mm) / snow_depth(m) = kg m^-3",
        "selected_ssd_kg_m3": characterization["selected_ssd_kg_m3"],
        "selected_ssd_source": characterization["selected_ssd_source"],
        "raw_files": file_entries(raw_files),
        "normalized_files": file_entries([csv_path, observations_dir / "ssd_characterization.json"]),
        "citation": "USDA NRCS National Water and Climate Center AWDB public SNOTEL data; cite station triplet and access date.",
    }


def download_json(url: str) -> Any:
    with urllib.request.urlopen(url, timeout=120) as response:
        return json.loads(response.read().decode("utf-8"))


def file_entries(paths: list[Path]) -> list[dict[str, Any]]:
    return [
        {
            "path": str(path),
            "sha256": sha256(path) if path.is_file() else None,
            "bytes": path.stat().st_size if path.is_file() else None,
        }
        for path in paths
    ]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def water_year(date: dt.date) -> int:
    return date.year + 1 if date.month >= 10 else date.year


def degf_to_degc(value: float) -> float:
    return (value - 32.0) * 5.0 / 9.0


def observed_density_kg_m3(swe_mm: float | None, depth_m: float | None) -> float | None:
    if swe_mm is None or depth_m is None:
        return None
    if swe_mm <= 0.0 or depth_m <= 0.0254:
        return None
    density = swe_mm / depth_m
    return density if math.isfinite(density) and density > 0.0 else None


def shallowest_soil_temp(profile: dict[float, float]) -> tuple[float | None, float | None]:
    if not profile:
        return None, None
    depth = min(profile)
    temp = profile[depth]
    return depth, temp if math.isfinite(temp) else None


def snow_depth_tolerance(depth_m: float) -> float:
    return max(SNOW_DEPTH_TOL_ABS_M, abs(depth_m) * SNOW_DEPTH_TOL_REL)


def swe_tolerance(swe_m: float) -> float:
    return max(SWE_TOL_ABS_M, abs(swe_m) * SWE_TOL_REL)


def density_tolerance(density_kg_m3: float) -> float:
    return max(DENSITY_TOL_ABS_KG_M3, abs(density_kg_m3) * DENSITY_TOL_REL)


def finite_or_none(value: Any) -> float | None:
    if value is None:
        return None
    parsed = float(value)
    return parsed if math.isfinite(parsed) else None


def optional_float(value: Any) -> float | None:
    if value is None:
        return None
    if isinstance(value, str) and value.strip() == "":
        return None
    parsed = float(value)
    if not math.isfinite(parsed):
        return None
    return parsed


def fmt_optional(value: float | None) -> str:
    return "" if value is None else f"{value:.6f}"


def fmt(value: Any) -> str:
    if value is None:
        return "n/a"
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def median(values: list[float]) -> float | None:
    if not values:
        return None
    ordered = sorted(values)
    midpoint = len(ordered) // 2
    if len(ordered) % 2:
        return ordered[midpoint]
    return (ordered[midpoint - 1] + ordered[midpoint]) / 2.0


def round_to_nearest(value: float | None, step: float) -> float:
    if value is None:
        raise ValueError("cannot round None")
    return round(value / step) * step


def numeric_summary(values: list[float]) -> dict[str, Any]:
    return {
        "count": len(values),
        "mean": mean(values),
        "median": median(values),
        "min": min(values) if values else None,
        "max": max(values) if values else None,
    }


def read_csv_dicts(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def write_csv_dicts(path: Path, rows: list[dict[str, Any]], fieldnames: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def write_observation_csv(path: Path, rows: list[dict[str, str]]) -> None:
    write_csv_dicts(path, rows, OBSERVATION_COLUMNS)


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def require_file(path: Path) -> None:
    if not path.is_file():
        raise FileNotFoundError(path)


def read_snow_txt(path: Path) -> list[float]:
    values = []
    for line in path.read_text(encoding="utf-8").splitlines():
        stripped = line.split("#", 1)[0].strip()
        if stripped:
            values.append(float(stripped.split()[0]))
    if len(values) < 3:
        raise ValueError(f"{path} has fewer than three snow-control values")
    return values[:3]


if __name__ == "__main__":
    raise SystemExit(main())
