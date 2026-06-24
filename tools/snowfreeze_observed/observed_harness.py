#!/usr/bin/env python3
"""Observation-backed frost-depth harness for SC-SNOWFREEZE-001.

The normal test path is offline.  Use ``fetch`` explicitly to refresh the raw
cache under ``target/`` and ``normalize`` to regenerate the checked-in
observation corpus.
"""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import io
import json
import math
import subprocess
import sys
import urllib.parse
import urllib.request
import zipfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_CACHE = REPO_ROOT / "target/snowfreeze_observed"
DEFAULT_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snowfreeze_observed/observations"
FIXTURE_ROOT = REPO_ROOT / "tests/fixtures/snowfreeze_observed"
ACCESS_DATE = "2026-06-24"
PARSER_VERSION = "snowfreeze-observed-harness-v1"

SOURCE_SCIENCEBASE_ITEM = (
    "https://www.sciencebase.gov/catalog/item/5e6bce83e4b01d5092632650?format=json"
)
SCAN_DATA_URL = "https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/data"
SCAN_STATION_URL = "https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/stations"
SCAN_REFERENCE_URL = "https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/reference-data"
GGD498_BASE = "ftp://sidads.colorado.edu/pub/DATASETS/fgdc/ggd498_seasfrost_usa"
GGD498_USER_GUIDE = "https://nsidc.org/sites/default/files/ggd498-userguide-v1.pdf"
REYNOLDS_METADATA_URL = (
    "https://www.hydroshare.org/hsapi/resource/"
    "b22305cd06eb4e37bdac1ec090daf7ef/scimeta/elements/"
)
REYNOLDS_DATASET_URL = (
    "https://catalog.data.gov/dataset/"
    "reynolds-creek-experimental-watershed-idaho-soil-temperature"
)
REYNOLDS_SOIL_TEMPERATURE_ZIP_URL = "https://ndownloader.figshare.com/files/44527883"
REYNOLDS_STATION = "127"
REYNOLDS_DEPTHS_M = [0.05, 0.10, 0.20, 0.30, 0.40, 0.50, 0.60, 0.90, 1.20, 1.80, 2.40]

OBS_COLUMNS = [
    "site_id",
    "source_id",
    "date",
    "water_year",
    "method",
    "authority_role",
    "observed_frost_depth_m",
    "observed_isotherm_depth_m",
    "observed_snow_depth_m",
    "censoring",
    "quality_flag",
    "source_record_id",
]

SLEEPERS_SITES = {
    "site1_sleepers_south_field_vt": {
        "site_name": "South field",
        "tube_ids": [str(value) for value in range(22, 27)],
        "fixture": "site1_sleepers_south_field_vt",
        "source_id": "usgs_sleepers_p96753gi",
    },
    "site2_sleepers_w9_hardwood_vt": {
        "site_name": "W9 hillslope",
        "tube_ids": [str(value) for value in range(38, 44)],
        "fixture": "site2_sleepers_w9_hardwood_vt",
        "source_id": "usgs_sleepers_p96753gi",
    },
}


@dataclass(frozen=True)
class ObservationRow:
    site_id: str
    source_id: str
    date: dt.date
    method: str
    authority_role: str
    observed_frost_depth_m: float | None
    observed_isotherm_depth_m: float | None
    observed_snow_depth_m: float | None
    censoring: str
    quality_flag: str
    source_record_id: str

    def as_csv_row(self) -> dict[str, str]:
        return {
            "site_id": self.site_id,
            "source_id": self.source_id,
            "date": self.date.isoformat(),
            "water_year": str(water_year(self.date)),
            "method": self.method,
            "authority_role": self.authority_role,
            "observed_frost_depth_m": format_optional(self.observed_frost_depth_m),
            "observed_isotherm_depth_m": format_optional(self.observed_isotherm_depth_m),
            "observed_snow_depth_m": format_optional(self.observed_snow_depth_m),
            "censoring": self.censoring,
            "quality_flag": self.quality_flag,
            "source_record_id": self.source_record_id,
        }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=DEFAULT_CACHE)
    subparsers = parser.add_subparsers(dest="command", required=True)

    fetch_parser = subparsers.add_parser("fetch", help="refresh raw source cache")
    fetch_parser.add_argument("--scan-begin", default="1994-10-06")
    fetch_parser.add_argument("--scan-end", default="2024-12-31")

    normalize_parser = subparsers.add_parser(
        "normalize", help="write normalized observation corpus"
    )
    normalize_parser.add_argument(
        "--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS
    )

    validate_parser = subparsers.add_parser(
        "validate", help="validate normalized observation corpus"
    )
    validate_parser.add_argument(
        "--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS
    )

    compare_parser = subparsers.add_parser(
        "compare", help="run one site and compare WAT frdp to observations"
    )
    compare_parser.add_argument("--site", required=True)
    compare_parser.add_argument(
        "--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS
    )
    compare_parser.add_argument("--output-dir", type=Path, required=True)
    compare_parser.add_argument("--binary", type=Path, default=None)
    compare_parser.add_argument("--no-run", action="store_true")
    compare_parser.add_argument(
        "--runtime",
        choices=["direct-production-executor", "compatibility"],
        default="direct-production-executor",
        help="runtime surface to compare; compatibility is a flagging surface only",
    )

    args = parser.parse_args(argv)
    cache = args.cache.resolve()

    if args.command == "fetch":
        fetch_sources(cache, args.scan_begin, args.scan_end)
        return 0
    if args.command == "normalize":
        normalize_sources(cache, args.observations_dir.resolve())
        return 0
    if args.command == "validate":
        validate_observations(args.observations_dir.resolve())
        return 0
    if args.command == "compare":
        compare_site(
            site_id=args.site,
            observations_dir=args.observations_dir.resolve(),
            output_dir=args.output_dir.resolve(),
            binary=args.binary.resolve() if args.binary else None,
            no_run=args.no_run,
            runtime=args.runtime,
        )
        return 0

    raise AssertionError(f"unreachable command {args.command}")


def fetch_sources(cache: Path, scan_begin: str, scan_end: str) -> None:
    raw_dir = cache / "raw"
    http_dir = cache / "http"
    raw_dir.mkdir(parents=True, exist_ok=True)
    http_dir.mkdir(parents=True, exist_ok=True)

    sciencebase_json = download_json(SOURCE_SCIENCEBASE_ITEM)
    write_json(http_dir / "sciencebase_sleepers.json", sciencebase_json)
    sleepers_dir = raw_dir / "sleepers"
    sleepers_dir.mkdir(parents=True, exist_ok=True)
    for source_name, local_name in [
        ("Frost site description.csv", "sleepers_site_description.csv"),
        ("Sleepers frost1983-2020.csv", "sleepers_frost1983_2020.csv"),
        ("frost metadata.xml", "sleepers_frost_metadata.xml"),
    ]:
        download_uri = sciencebase_file_uri(sciencebase_json, source_name)
        download_file(download_uri, sleepers_dir / local_name)

    ggd_dir = raw_dir / "ggd498"
    ggd_dir.mkdir(parents=True, exist_ok=True)
    download_file(f"{GGD498_BASE}/README.txt", ggd_dir / "README.txt")
    download_file(f"{GGD498_BASE}/10.txt", ggd_dir / "10.txt")
    download_file(GGD498_USER_GUIDE, ggd_dir / "ggd498-userguide-v1.pdf")

    scan_dir = raw_dir / "scan"
    scan_dir.mkdir(parents=True, exist_ok=True)
    scan_data_url = SCAN_DATA_URL + "?" + urllib.parse.urlencode(
        {
            "stationTriplets": "2020:ND:SCAN",
            "duration": "DAILY",
            "beginDate": scan_begin,
            "endDate": scan_end,
            "elements": "STO:*:*",
            "returnFlags": "true",
            "returnOriginalValues": "true",
        }
    )
    write_json(scan_dir / "mandan_sto_daily.json", download_json(scan_data_url))
    write_json(
        scan_dir / "mandan_station.json",
        download_json(
            SCAN_STATION_URL
            + "?"
            + urllib.parse.urlencode({"stationTriplets": "2020:ND:SCAN"})
        ),
    )
    write_json(
        scan_dir / "awdb_elements.json",
        download_json(
            SCAN_REFERENCE_URL
            + "?"
            + urllib.parse.urlencode({"referenceLists": "elements"})
        ),
    )

    reynolds_dir = raw_dir / "reynolds"
    reynolds_dir.mkdir(parents=True, exist_ok=True)
    reynolds_zip = reynolds_dir / "soiltemperature.zip"
    download_file(REYNOLDS_SOIL_TEMPERATURE_ZIP_URL, reynolds_zip)
    try:
        write_json(
            reynolds_dir / "hydroshare_metadata.json",
            download_json(REYNOLDS_METADATA_URL),
        )
    except OSError as error:
        write_json(
            reynolds_dir / "hydroshare_metadata_status.json",
            {
                "status": "OPTIONAL_METADATA_UNAVAILABLE",
                "reason": str(error),
                "metadata_url": REYNOLDS_METADATA_URL,
                "access_date": ACCESS_DATE,
            },
        )
    status = {
        "status": "acquired",
        "reason": "Soil-temperature archive acquired from the public Data.gov/Figshare route.",
        "dataset_url": REYNOLDS_DATASET_URL,
        "download_url": REYNOLDS_SOIL_TEMPERATURE_ZIP_URL,
        "station": REYNOLDS_STATION,
        "access_date": ACCESS_DATE,
        "sha256": sha256(reynolds_zip),
        "bytes": reynolds_zip.stat().st_size,
    }
    write_json(reynolds_dir / "source_status.json", status)

    write_json(
        cache / "fetch-summary.json",
        {
            "access_date": ACCESS_DATE,
            "cache": str(cache),
            "sources": {
                "usgs_sleepers_p96753gi": "acquired",
                "nsidc_ggd498_morris_10": "acquired",
                "nrcs_scan_mandan_2020": "acquired",
                "usda_ars_reynolds_creek": status["status"],
            },
        },
    )


def normalize_sources(cache: Path, observations_dir: Path) -> None:
    sites_dir = observations_dir / "sites"
    provenance_dir = observations_dir / "provenance"
    sites_dir.mkdir(parents=True, exist_ok=True)
    provenance_dir.mkdir(parents=True, exist_ok=True)

    site_records: list[dict[str, Any]] = []
    source_records: list[dict[str, Any]] = []

    sleepers_rows = normalize_sleepers(cache)
    for site_id in SLEEPERS_SITES:
        rows = sleepers_rows[site_id]
        csv_path = sites_dir / f"{site_id}.csv"
        write_observation_csv(csv_path, rows)
        site_records.append(site_manifest_record(site_id, rows, "acquired", csv_path))
    sleepers_normalized_files = [
        sites_dir / "site1_sleepers_south_field_vt.csv",
        sites_dir / "site2_sleepers_w9_hardwood_vt.csv",
    ]
    sleepers_provenance = provenance_record(
        "usgs_sleepers_p96753gi",
        "USGS ScienceBase DOI 10.5066/P96753GI",
        [
            cache / "raw/sleepers/sleepers_site_description.csv",
            cache / "raw/sleepers/sleepers_frost1983_2020.csv",
            cache / "raw/sleepers/sleepers_frost_metadata.xml",
        ],
        sum(len(value) for value in sleepers_rows.values()),
        "Frost-tube Fbottom and snow-depth values are centimeters; site rows average available tubes by date.",
        SOURCE_SCIENCEBASE_ITEM,
        "USGS ScienceBase public data release DOI 10.5066/P96753GI; cite the USGS data release.",
        sleepers_normalized_files,
    )
    write_json(provenance_dir / "usgs_sleepers_p96753gi.json", sleepers_provenance)
    source_records.append(sleepers_provenance)

    ggd_rows = normalize_ggd498(cache)
    ggd_csv = sites_dir / "site4_ggd498_morris_mn.csv"
    write_observation_csv(ggd_csv, ggd_rows)
    site_records.append(
        site_manifest_record("site4_ggd498_morris_mn", ggd_rows, "acquired", ggd_csv)
    )
    ggd_provenance = provenance_record(
        "nsidc_ggd498_morris_10",
        "NSIDC GGD498 DOI 10.7265/1mcs-q536 station 10 Morris",
        [
            cache / "raw/ggd498/10.txt",
            cache / "raw/ggd498/README.txt",
            cache / "raw/ggd498/ggd498-userguide-v1.pdf",
        ],
        len(ggd_rows),
        "Station 10 flat-file fields are station,date,snow_cm,frost_bottom_cm,thaw_depth_cm plus ancillary soil fields.",
        "https://nsidc.org/data/ggd498/versions/1",
        "NSIDC GGD498 v1 public data; cite DOI 10.7265/1mcs-q536 and the access date.",
        [ggd_csv],
    )
    write_json(provenance_dir / "nsidc_ggd498_morris_10.json", ggd_provenance)
    source_records.append(ggd_provenance)

    scan_rows = normalize_scan(cache)
    scan_csv = sites_dir / "site3_scan_mandan_nd.csv"
    write_observation_csv(scan_csv, scan_rows)
    site_records.append(
        site_manifest_record("site3_scan_mandan_nd", scan_rows, "acquired", scan_csv)
    )
    scan_provenance = provenance_record(
        "nrcs_scan_mandan_2020",
        "USDA NRCS AWDB station 2020:ND:SCAN soil temperature",
        [
            cache / "raw/scan/mandan_sto_daily.json",
            cache / "raw/scan/mandan_station.json",
            cache / "raw/scan/awdb_elements.json",
        ],
        len(scan_rows),
        "STO profile temperatures are stored in degF with depth in inches; normalized is the interpolated 0 degC isotherm in meters.",
        "https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/data",
        "USDA NRCS AWDB public station data; cite USDA NRCS SCAN/AWDB station 2020:ND:SCAN.",
        [scan_csv],
    )
    write_json(provenance_dir / "nrcs_scan_mandan_2020.json", scan_provenance)
    source_records.append(scan_provenance)

    site5_rows = normalize_reynolds(cache)
    site5_csv = sites_dir / "site5_reynolds_creek_us_rls_id.csv"
    write_observation_csv(site5_csv, site5_rows)
    site_records.append(
        site_manifest_record("site5_reynolds_creek_us_rls_id", site5_rows, "acquired", site5_csv)
    )
    reynolds_provenance = {
        "source_id": "usda_ars_reynolds_creek_soil_temperature",
        "title": "USDA-ARS Reynolds Creek soil temperature station 127",
        "source_url": REYNOLDS_DATASET_URL,
        "download_url": REYNOLDS_SOIL_TEMPERATURE_ZIP_URL,
        "access_date": ACCESS_DATE,
        "license_or_terms": "Included license.txt places the data in the public domain and requests customary citation.",
        "parser_version": PARSER_VERSION,
        "normalized_row_count": len(site5_rows),
        "status": "acquired",
        "station_mapping": "station 127; archive elevation 1652 m matches site5 modeled elevation 1653 m and Low Sagebrush fixture mapping",
        "parser_assumptions": "Weekly and hourly station-127 soil-temperature profiles are normalized to one daily 0 degC isotherm in meters; hourly profiles are averaged by date/depth before interpolation.",
        "raw_files": file_entries(
            [
                cache / "raw/reynolds/soiltemperature.zip",
                cache / "raw/reynolds/source_status.json",
                cache / "raw/reynolds/hydroshare_metadata.json",
            ]
        ),
        "normalized_files": file_entries([site5_csv]),
    }
    write_json(
        provenance_dir / "usda_ars_reynolds_creek_soil_temperature.json",
        reynolds_provenance,
    )
    source_records.append(reynolds_provenance)

    manifest = {
        "schema": "snowfreeze-observed-manifest-v1",
        "access_date": ACCESS_DATE,
        "parser_version": PARSER_VERSION,
        "normal_depth_units": "m",
        "normal_temperature_units": "degC",
        "measurement_contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047",
        "snow_depth_control": {
            "modeled_status": "UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC",
            "note": "WAT Snow-Water is SWE and is not a snow-depth diagnostic.",
        },
        "sites": site_records,
        "sources": source_records,
    }
    write_json(observations_dir / "manifest.json", manifest)
    validate_observations(observations_dir)


def normalize_sleepers(cache: Path) -> dict[str, list[ObservationRow]]:
    frost_path = cache / "raw/sleepers/sleepers_frost1983_2020.csv"
    require_file(frost_path)
    rows: dict[str, list[ObservationRow]] = {site_id: [] for site_id in SLEEPERS_SITES}
    with frost_path.open(newline="", encoding="utf-8-sig") as handle:
        reader = csv.DictReader(handle)
        normalized_fieldnames = {field: field.strip() for field in reader.fieldnames or []}
        for raw_record in reader:
            record = {
                normalized_fieldnames.get(key, key).strip(): value
                for key, value in raw_record.items()
            }
            if not record.get("Date"):
                continue
            observed_date = parse_us_date(record["Date"])
            for site_id, config in SLEEPERS_SITES.items():
                snow_depths = []
                frost_bottoms = []
                frost_tops = []
                for tube_id in config["tube_ids"]:
                    snow_depths.append(parse_number(record.get(f"{tube_id}S")))
                    frost_tops.append(parse_number(record.get(f"{tube_id}Ftop")))
                    frost_bottoms.append(parse_number(record.get(f"{tube_id}Fbottom")))
                snow_values = [value for value in snow_depths if value is not None]
                frost_values = [value for value in frost_bottoms if value is not None]
                top_values = [value for value in frost_tops if value is not None]
                if not snow_values and not frost_values:
                    continue
                quality = [
                    f"tube_count={len(frost_values)}",
                    f"snow_count={len(snow_values)}",
                ]
                if top_values:
                    quality.append(f"ftop_count={len(top_values)}")
                rows[site_id].append(
                    ObservationRow(
                        site_id=site_id,
                        source_id=config["source_id"],
                        date=observed_date,
                        method="frost_tube",
                        authority_role="magnitude",
                        observed_frost_depth_m=mean_cm_to_m(frost_values),
                        observed_isotherm_depth_m=None,
                        observed_snow_depth_m=mean_cm_to_m(snow_values),
                        censoring="none",
                        quality_flag=";".join(quality),
                        source_record_id=(
                            f"Sleepers frost1983-2020.csv:{observed_date.isoformat()}:"
                            + ",".join(config["tube_ids"])
                        ),
                    )
                )
    return rows


def normalize_ggd498(cache: Path) -> list[ObservationRow]:
    path = cache / "raw/ggd498/10.txt"
    require_file(path)
    rows: list[ObservationRow] = []
    with path.open(encoding="utf-8") as handle:
        for line_number, line in enumerate(handle, start=1):
            line = line.strip()
            if not line:
                continue
            fields = [field.strip() for field in line.split(",")]
            if len(fields) < 5:
                raise ValueError(f"GGD498 line {line_number} has fewer than 5 fields")
            station_id, raw_date = fields[0], fields[1]
            if station_id != "10":
                raise ValueError(f"GGD498 Morris parser expected station 10, got {station_id}")
            observed_date = parse_slash_dmy(raw_date)
            snow_cm = parse_number(fields[2])
            frost_bottom_cm = parse_number(fields[3])
            thaw_cm = parse_number(fields[4])
            if snow_cm == -99.9:
                snow_cm = None
            if frost_bottom_cm == -99.9:
                frost_bottom_cm = None
            if thaw_cm == -99.9:
                thaw_cm = None
            if frost_bottom_cm is None and snow_cm is None:
                continue
            quality = f"station=10;line={line_number}"
            if thaw_cm is not None:
                quality += ";thaw_depth_cm_present"
            rows.append(
                ObservationRow(
                    site_id="site4_ggd498_morris_mn",
                    source_id="nsidc_ggd498_morris_10",
                    date=observed_date,
                    method="frost_tube",
                    authority_role="magnitude_limited_overlap",
                    observed_frost_depth_m=cm_to_m(frost_bottom_cm),
                    observed_isotherm_depth_m=None,
                    observed_snow_depth_m=cm_to_m(snow_cm),
                    censoring="none",
                    quality_flag=quality,
                    source_record_id=f"GGD498:10.txt:{line_number}",
                )
            )
    return rows


def normalize_scan(cache: Path) -> list[ObservationRow]:
    path = cache / "raw/scan/mandan_sto_daily.json"
    require_file(path)
    document = json.loads(path.read_text(encoding="utf-8"))
    by_date: dict[dt.date, dict[float, float]] = {}
    for station in document:
        for element in station.get("data", []):
            station_element = element.get("stationElement", {})
            if station_element.get("elementCode") != "STO":
                continue
            depth_inches = station_element.get("heightDepth")
            if depth_inches is None:
                continue
            depth_m = abs(float(depth_inches)) * 0.0254
            for value_record in element.get("values", []):
                value = value_record.get("value")
                qc_flag = value_record.get("qcFlag")
                if value is None or qc_flag not in (None, "V"):
                    continue
                observed_date = dt.date.fromisoformat(value_record["date"])
                temperature_c = (float(value) - 32.0) * (5.0 / 9.0)
                by_date.setdefault(observed_date, {})[depth_m] = temperature_c
    rows: list[ObservationRow] = []
    for observed_date in sorted(by_date):
        isotherm_depth_m, censoring = zero_c_isotherm_depth(by_date[observed_date])
        rows.append(
            ObservationRow(
                site_id="site3_scan_mandan_nd",
                source_id="nrcs_scan_mandan_2020",
                date=observed_date,
                method="soil_temperature_zero_c_isotherm",
                authority_role="timing_upper_bound",
                observed_frost_depth_m=None,
                observed_isotherm_depth_m=isotherm_depth_m,
                observed_snow_depth_m=None,
                censoring=censoring,
                quality_flag="depths_m=" + ",".join(f"{d:.4f}" for d in sorted(by_date[observed_date])),
                source_record_id=f"AWDB:2020:ND:SCAN:STO:*:*:{observed_date.isoformat()}",
            )
        )
    return rows


def normalize_reynolds(cache: Path) -> list[ObservationRow]:
    archive_path = cache / "raw/reynolds/soiltemperature.zip"
    require_file(archive_path)
    daily_profiles: dict[dt.date, dict[float, list[float]]] = {}
    with zipfile.ZipFile(archive_path) as outer_zip:
        for nested_name, text_name, cadence in [
            (
                "soiltemperature/weekly127x07soiltemperature.zip",
                "weekly127x07soiltemperature.txt",
                "weekly",
            ),
            (
                "soiltemperature/hourly127x07soiltemperature.zip",
                "hourly127x07soiltemperature.txt",
                "hourly_daily_mean",
            ),
        ]:
            nested_bytes = outer_zip.read(nested_name)
            with zipfile.ZipFile(io.BytesIO(nested_bytes)) as nested_zip:
                text = nested_zip.read(text_name).decode("latin-1")
            for observed_date, profile in parse_reynolds_temperature_text(text):
                date_profiles = daily_profiles.setdefault(observed_date, {})
                for depth_m, temperature_c in profile.items():
                    date_profiles.setdefault(depth_m, []).append(temperature_c)

    rows: list[ObservationRow] = []
    for observed_date in sorted(daily_profiles):
        averaged_profile = {
            depth_m: sum(values) / len(values)
            for depth_m, values in daily_profiles[observed_date].items()
            if values
        }
        if not averaged_profile:
            continue
        isotherm_depth_m, censoring = zero_c_isotherm_depth(averaged_profile)
        cadence = "weekly" if observed_date < dt.date(1984, 12, 5) else "hourly_daily_mean"
        rows.append(
            ObservationRow(
                site_id="site5_reynolds_creek_us_rls_id",
                source_id="usda_ars_reynolds_creek_soil_temperature",
                date=observed_date,
                method="soil_temperature_zero_c_isotherm",
                authority_role="timing_upper_bound",
                observed_frost_depth_m=None,
                observed_isotherm_depth_m=isotherm_depth_m,
                observed_snow_depth_m=None,
                censoring=censoring,
                quality_flag=(
                    f"station={REYNOLDS_STATION};cadence={cadence};"
                    + "depths_m="
                    + ",".join(f"{depth:.2f}" for depth in sorted(averaged_profile))
                ),
                source_record_id=(
                    f"Reynolds:{REYNOLDS_STATION}:{cadence}:{observed_date.isoformat()}"
                ),
            )
        )
    return rows


def parse_reynolds_temperature_text(text: str) -> list[tuple[dt.date, dict[float, float]]]:
    rows: list[tuple[dt.date, dict[float, float]]] = []
    for line_number, line in enumerate(text.splitlines(), start=1):
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        fields = stripped.split()
        if len(fields) < 6:
            raise ValueError(f"Reynolds station {REYNOLDS_STATION} line {line_number} is short")
        month, day, year = (int(fields[0]), int(fields[1]), int(fields[2]))
        observed_date = dt.date(year, month, day)
        profile: dict[float, float] = {}
        for depth_m, raw_temperature in zip(REYNOLDS_DEPTHS_M, fields[5:]):
            temperature_c = parse_number(raw_temperature)
            if temperature_c is not None:
                profile[depth_m] = temperature_c
        if profile:
            rows.append((observed_date, profile))
    return rows


def zero_c_isotherm_depth(profile: dict[float, float]) -> tuple[float, str]:
    samples = sorted(profile.items())
    if not samples:
        return 0.0, "no_profile"
    if samples[0][1] > 0.0:
        return 0.0, "none"
    for (upper_depth, upper_temp), (lower_depth, lower_temp) in zip(samples, samples[1:]):
        if upper_temp <= 0.0 < lower_temp:
            denominator = lower_temp - upper_temp
            if abs(denominator) < 1.0e-12:
                return upper_depth, "none"
            fraction = (0.0 - upper_temp) / denominator
            return upper_depth + fraction * (lower_depth - upper_depth), "none"
    return samples[-1][0], "right_censored_sensor_depth"


def validate_observations(observations_dir: Path) -> None:
    manifest_path = observations_dir / "manifest.json"
    require_file(manifest_path)
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if manifest.get("schema") != "snowfreeze-observed-manifest-v1":
        raise ValueError("manifest schema must be snowfreeze-observed-manifest-v1")
    if manifest.get("measurement_contract") != "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047":
        raise ValueError("manifest must bind SC-SNOWFREEZE-001 INV-SNOWFREEZE-047")

    for site in manifest.get("sites", []):
        csv_path = observations_dir / site["observation_file"]
        require_file(csv_path)
        expected_sha256 = site.get("normalized_file_sha256")
        if expected_sha256 and sha256(csv_path) != expected_sha256:
            raise ValueError(f"{csv_path} normalized_file_sha256 mismatch")
        expected_bytes = site.get("normalized_file_bytes")
        if expected_bytes is not None and csv_path.stat().st_size != expected_bytes:
            raise ValueError(f"{csv_path} normalized_file_bytes mismatch")
        with csv_path.open(newline="", encoding="utf-8") as handle:
            reader = csv.DictReader(handle)
            if reader.fieldnames != OBS_COLUMNS:
                raise ValueError(f"{csv_path} columns mismatch: {reader.fieldnames}")
            row_count = 0
            for record in reader:
                row_count += 1
                validate_observation_record(csv_path, record)
            if row_count != site["normalized_row_count"]:
                raise ValueError(
                    f"{csv_path} row-count mismatch {row_count} != {site['normalized_row_count']}"
                )

    for source in manifest.get("sources", []):
        provenance_path = observations_dir / "provenance" / f"{source['source_id']}.json"
        require_file(provenance_path)
        provenance = json.loads(provenance_path.read_text(encoding="utf-8"))
        if provenance.get("parser_version") != PARSER_VERSION:
            raise ValueError(f"{provenance_path} parser_version mismatch")
        for normalized in provenance.get("normalized_files", []):
            normalized_path = Path(normalized["path"])
            require_file(normalized_path)
            if sha256(normalized_path) != normalized.get("sha256"):
                raise ValueError(f"{provenance_path} normalized file checksum mismatch")
            if normalized_path.stat().st_size != normalized.get("bytes"):
                raise ValueError(f"{provenance_path} normalized file byte-count mismatch")


def validate_observation_record(path: Path, record: dict[str, str]) -> None:
    try:
        date = dt.date.fromisoformat(record["date"])
    except ValueError as error:
        raise ValueError(f"{path} invalid date {record.get('date')}") from error
    if str(water_year(date)) != record["water_year"]:
        raise ValueError(f"{path} water_year mismatch for {record['date']}")
    method = record["method"]
    frost = parse_optional_float(record["observed_frost_depth_m"])
    isotherm = parse_optional_float(record["observed_isotherm_depth_m"])
    snow = parse_optional_float(record["observed_snow_depth_m"])
    for field_name, value in [
        ("observed_frost_depth_m", frost),
        ("observed_isotherm_depth_m", isotherm),
        ("observed_snow_depth_m", snow),
    ]:
        if value is not None and (not math.isfinite(value) or value < 0.0):
            raise ValueError(f"{path} invalid non-negative depth {field_name}={value}")
    if method == "frost_tube" and frost is None:
        raise ValueError(f"{path} frost_tube row lacks observed_frost_depth_m")
    if method == "soil_temperature_zero_c_isotherm" and isotherm is None:
        raise ValueError(f"{path} isotherm row lacks observed_isotherm_depth_m")
    if method == "soil_temperature_zero_c_isotherm" and frost is not None:
        raise ValueError(f"{path} isotherm row must not become a frost-depth target")
    if record["source_record_id"].strip() == "":
        raise ValueError(f"{path} source_record_id cannot be empty")


def compare_site(
    site_id: str,
    observations_dir: Path,
    output_dir: Path,
    binary: Path | None,
    no_run: bool,
    runtime: str,
) -> None:
    validate_observations(observations_dir)
    manifest = json.loads((observations_dir / "manifest.json").read_text(encoding="utf-8"))
    site_record = next((site for site in manifest["sites"] if site["site_id"] == site_id), None)
    if site_record is None:
        raise ValueError(f"unknown site_id {site_id}")

    output_dir.mkdir(parents=True, exist_ok=True)
    if site_record["status"] == "SOURCE-BLOCKED":
        report = {
            "schema": "snowfreeze-observed-comparison-v1",
            "site_id": site_id,
            "verdict": "SOURCE-BLOCKED",
            "reason": "normalized observation rows unavailable for this source",
        }
        write_comparison_reports(output_dir, report)
        return

    observations = load_observations(observations_dir / site_record["observation_file"])
    fixture_dir = FIXTURE_ROOT / site_record["fixture"]
    run_stem = discover_run_stem(fixture_dir)
    wat_path = output_dir / f"{site_id}.wat.parquet"
    runfile_path = output_dir / f"{site_id}.run"
    write_runfile(runfile_path, fixture_dir, run_stem, output_dir, site_id)

    if not no_run:
        command = cli_command(binary, fixture_dir, runfile_path, output_dir, runtime)
        completed = subprocess.run(
            command,
            cwd=REPO_ROOT,
            check=False,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        (output_dir / "openwepp-cli-hill.stdout").write_text(completed.stdout, encoding="utf-8")
        (output_dir / "openwepp-cli-hill.stderr").write_text(completed.stderr, encoding="utf-8")
        if completed.returncode != 0:
            report = {
                "schema": "snowfreeze-observed-comparison-v1",
                "site_id": site_id,
                "verdict": "HARNESS-SURFACE-MISMATCH",
                "reason": f"openwepp-cli-hill failed with exit code {completed.returncode}",
                "command": command,
                "runtime": runtime,
            }
            write_comparison_reports(output_dir, report)
            raise SystemExit(completed.returncode)

    if not wat_path.is_file():
        raise FileNotFoundError(f"expected WAT parquet output {wat_path}")

    modeled = load_modeled_wat(wat_path)
    metrics = compute_metrics(observations, modeled)
    verdict = "UNRESOLVED"
    if metrics["matched_count"] == 0:
        verdict = "HARNESS-SURFACE-MISMATCH"
    report = {
        "schema": "snowfreeze-observed-comparison-v1",
        "site_id": site_id,
        "fixture_dir": str(fixture_dir),
        "runfile": str(runfile_path),
        "wat_output": str(wat_path),
        "runtime": runtime,
        "verdict": verdict,
        "measurement_contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047",
        "snow_control_status": "UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC",
        "metrics": metrics,
    }
    write_comparison_reports(output_dir, report)


def compute_metrics(
    observations: list[dict[str, str]], modeled: dict[dt.date, dict[str, float]]
) -> dict[str, Any]:
    frost_depth_residuals = []
    isotherm_upper_bounds = []
    matched_dates: list[dt.date] = []
    censored_excluded_count = 0
    unmatched_count = 0
    matched_series = []
    for observation in observations:
        observed_date = dt.date.fromisoformat(observation["date"])
        modeled_row = modeled.get(observed_date)
        if modeled_row is None:
            unmatched_count += 1
            continue
        matched_dates.append(observed_date)
        method = observation["method"]
        censoring = observation["censoring"]
        frost_depth_m = parse_optional_float(observation["observed_frost_depth_m"])
        isotherm_depth_m = parse_optional_float(observation["observed_isotherm_depth_m"])
        observed_depth_m = frost_depth_m if frost_depth_m is not None else isotherm_depth_m
        matched_series.append(
            {
                "date": observed_date,
                "method": method,
                "observed_depth_m": observed_depth_m,
                "modeled_frdp_m": modeled_row["frdp_m"],
            }
        )
        if censoring != "none":
            censored_excluded_count += 1
            continue
        if method == "frost_tube" and frost_depth_m is not None:
            residual = modeled_row["frdp_m"] - frost_depth_m
            frost_depth_residuals.append(
                {
                    "date": observed_date.isoformat(),
                    "target_kind": "frost_depth",
                    "observed_m": frost_depth_m,
                    "modeled_frdp_m": modeled_row["frdp_m"],
                    "residual_m": residual,
                }
            )
        elif method == "soil_temperature_zero_c_isotherm" and isotherm_depth_m is not None:
            margin_m = modeled_row["frdp_m"] - isotherm_depth_m
            isotherm_upper_bounds.append(
                {
                    "date": observed_date.isoformat(),
                    "target_kind": "isotherm_upper_bound",
                    "observed_isotherm_m": isotherm_depth_m,
                    "modeled_frdp_m": modeled_row["frdp_m"],
                    "upper_bound_margin_m": margin_m,
                    "exceeds_upper_bound": margin_m > 0.0,
                }
            )
    absolute_residuals = [abs(row["residual_m"]) for row in frost_depth_residuals]
    seasonal_metrics = seasonal_timing_metrics(matched_series)
    return {
        "observation_count": len(observations),
        "modeled_day_count": len(modeled),
        "matched_count": len(matched_dates),
        "unmatched_observation_count": unmatched_count,
        "first_matched_date": matched_dates[0].isoformat() if matched_dates else None,
        "last_matched_date": matched_dates[-1].isoformat() if matched_dates else None,
        "censored_excluded_count": censored_excluded_count,
        "frost_depth_residual_count": len(frost_depth_residuals),
        "max_abs_residual_m": max(absolute_residuals) if absolute_residuals else None,
        "mean_abs_residual_m": (
            sum(absolute_residuals) / len(absolute_residuals) if absolute_residuals else None
        ),
        "isotherm_upper_bound_count": len(isotherm_upper_bounds),
        "isotherm_upper_bound_exceedance_count": sum(
            1 for row in isotherm_upper_bounds if row["exceeds_upper_bound"]
        ),
        "max_isotherm_upper_bound_margin_m": max(
            (row["upper_bound_margin_m"] for row in isotherm_upper_bounds),
            default=None,
        ),
        "seasonal_max_observed_m": max(
            (
                parse_optional_float(row["observed_frost_depth_m"])
                for row in observations
                if row_allows_magnitude(row)
                and parse_optional_float(row["observed_frost_depth_m"]) is not None
            ),
            default=None,
        ),
        "seasonal_max_modeled_on_observation_dates_m": (
            max(row["modeled_frdp_m"] for row in frost_depth_residuals)
            if frost_depth_residuals
            else None
        ),
        "seasonal_metrics": seasonal_metrics,
        "sample_residuals": frost_depth_residuals[:20],
        "sample_isotherm_upper_bounds": isotherm_upper_bounds[:20],
    }


def row_allows_magnitude(row: dict[str, str]) -> bool:
    return row["method"] == "frost_tube" and row["censoring"] == "none"


def seasonal_timing_metrics(matched_series: list[dict[str, Any]]) -> list[dict[str, Any]]:
    by_water_year: dict[int, list[dict[str, Any]]] = {}
    for row in matched_series:
        by_water_year.setdefault(water_year(row["date"]), []).append(row)

    summaries = []
    for year in sorted(by_water_year):
        rows = by_water_year[year]
        observed_frozen_dates = [
            row["date"]
            for row in rows
            if row["observed_depth_m"] is not None and row["observed_depth_m"] > 0.0
        ]
        modeled_frozen_dates = [
            row["date"] for row in rows if row["modeled_frdp_m"] > 0.0
        ]
        observed_onset = min(observed_frozen_dates) if observed_frozen_dates else None
        modeled_onset = min(modeled_frozen_dates) if modeled_frozen_dates else None
        observed_thaw = max(observed_frozen_dates) if observed_frozen_dates else None
        modeled_thaw = max(modeled_frozen_dates) if modeled_frozen_dates else None
        summaries.append(
            {
                "water_year": year,
                "observation_dates": len(rows),
                "observed_onset_date": format_date(observed_onset),
                "modeled_onset_date": format_date(modeled_onset),
                "onset_residual_days": date_delta_days(modeled_onset, observed_onset),
                "observed_thaw_date": format_date(observed_thaw),
                "modeled_thaw_date": format_date(modeled_thaw),
                "thaw_residual_days": date_delta_days(modeled_thaw, observed_thaw),
                "observed_frozen_duration_observation_days": len(observed_frozen_dates),
                "modeled_frozen_duration_observation_days": len(modeled_frozen_dates),
                "frozen_duration_residual_observation_days": (
                    len(modeled_frozen_dates) - len(observed_frozen_dates)
                ),
            }
        )
    return summaries


def format_date(value: dt.date | None) -> str | None:
    return value.isoformat() if value is not None else None


def date_delta_days(lhs: dt.date | None, rhs: dt.date | None) -> int | None:
    if lhs is None or rhs is None:
        return None
    return (lhs - rhs).days


def load_modeled_wat(path: Path) -> dict[dt.date, dict[str, float]]:
    try:
        import pyarrow.parquet as pq
    except ImportError as error:
        raise RuntimeError(
            "pyarrow is required for compare; run through .venv/bin/python"
        ) from error

    table = pq.read_table(path, columns=["water_year", "month", "day_of_month", "frdp"])
    columns = table.to_pydict()
    modeled: dict[dt.date, dict[str, float]] = {}
    for water_year_value, month, day, frdp_mm in zip(
        columns["water_year"],
        columns["month"],
        columns["day_of_month"],
        columns["frdp"],
    ):
        calendar_year = int(water_year_value) - 1 if int(month) >= 10 else int(water_year_value)
        modeled_date = dt.date(calendar_year, int(month), int(day))
        if modeled_date in modeled:
            raise ValueError(f"{path} has duplicate modeled WAT date {modeled_date}")
        modeled[modeled_date] = {
            "frdp_m": float(frdp_mm) / 1000.0
        }
    return modeled


def load_observations(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def write_runfile(
    runfile_path: Path, fixture_dir: Path, run_stem: str, output_dir: Path, site_id: str
) -> None:
    payload = f'''schema = "openwepp-hillslope-runfile-v1"
run_name = "snowfreeze-observed-{site_id}"
unit_system = "metric"

[inputs]
soil = "{toml_path(fixture_dir / (run_stem + ".sol"))}"
management = "{toml_path(fixture_dir / (run_stem + ".man"))}"
slope = "{toml_path(fixture_dir / (run_stem + ".slp"))}"
climate = "{toml_path(fixture_dir / (run_stem + ".cli"))}"
wepp_ui = false

[outputs]
pass = "{toml_path(output_dir / (site_id + ".hbp"))}"
loss = "{toml_path(output_dir / (site_id + ".loss.json"))}"
wat = "{toml_path(output_dir / (site_id + ".wat.parquet"))}"
'''
    runfile_path.write_text(payload, encoding="utf-8")


def cli_command(
    binary: Path | None, fixture_dir: Path, runfile_path: Path, output_dir: Path, runtime: str
) -> list[str]:
    if runtime == "direct-production-executor":
        runtime_flag = "--direct-production-executor"
    elif runtime == "compatibility":
        runtime_flag = "--compatibility-runtime"
    else:
        raise ValueError(f"unsupported runtime {runtime}")

    if binary is not None:
        return [
            str(binary),
            "--run-dir",
            str(fixture_dir),
            "--run-file",
            str(runfile_path),
            "--output-dir",
            str(output_dir),
            "--legacy-sidecar-discovery",
            runtime_flag,
        ]
    local_binary = REPO_ROOT / "target/debug/openwepp-cli-hill"
    if local_binary.is_file():
        return cli_command(local_binary, fixture_dir, runfile_path, output_dir, runtime)
    return [
        "cargo",
        "run",
        "-p",
        "openwepp-runner",
        "--bin",
        "openwepp-cli-hill",
        "--",
        "--run-dir",
        str(fixture_dir),
        "--run-file",
        str(runfile_path),
        "--output-dir",
        str(output_dir),
        "--legacy-sidecar-discovery",
        runtime_flag,
    ]


def write_comparison_reports(output_dir: Path, report: dict[str, Any]) -> None:
    write_json(output_dir / "comparison_report.json", report)
    lines = [
        "# Snowfreeze Observed Frost-Depth Comparison",
        "",
        f"- Site: `{report['site_id']}`",
        f"- Verdict: `{report['verdict']}`",
    ]
    if "snow_control_status" in report:
        lines.append(f"- Snow control: `{report['snow_control_status']}`")
    if "metrics" in report:
        metrics = report["metrics"]
        lines.extend(
            [
                f"- Observation rows: `{metrics['observation_count']}`",
                f"- Matched rows: `{metrics['matched_count']}`",
                f"- Frost-depth residual rows: `{metrics['frost_depth_residual_count']}`",
                f"- Isotherm upper-bound rows: `{metrics['isotherm_upper_bound_count']}`",
                f"- Censored rows excluded: `{metrics['censored_excluded_count']}`",
                f"- Max absolute frost-depth residual (m): `{format_report_value(metrics['max_abs_residual_m'])}`",
            ]
        )
    if "reason" in report:
        lines.append(f"- Reason: {report['reason']}")
    (output_dir / "comparison_report.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def format_report_value(value: Any) -> str:
    return "n/a" if value is None else str(value)


def discover_run_stem(fixture_dir: Path) -> str:
    candidates = sorted(fixture_dir.glob("p*.sol"))
    if len(candidates) != 1:
        raise ValueError(f"expected exactly one p*.sol in {fixture_dir}, got {candidates}")
    return candidates[0].stem


def site_manifest_record(
    site_id: str, rows: list[ObservationRow], status: str, csv_path: Path
) -> dict[str, Any]:
    method = rows[0].method if rows else "soil_temperature_zero_c_isotherm"
    source_id = rows[0].source_id if rows else "usda_ars_reynolds_creek_soil_temperature"
    return {
        "site_id": site_id,
        "fixture": site_id,
        "observation_file": f"sites/{site_id}.csv",
        "normalized_file_sha256": sha256(csv_path),
        "normalized_file_bytes": csv_path.stat().st_size,
        "source_id": source_id,
        "status": status,
        "method": method,
        "normalized_row_count": len(rows),
        "start_date": rows[0].date.isoformat() if rows else None,
        "end_date": rows[-1].date.isoformat() if rows else None,
    }


def provenance_record(
    source_id: str,
    title: str,
    raw_files: list[Path],
    normalized_row_count: int,
    parser_assumptions: str,
    source_url: str,
    license_or_terms: str,
    normalized_files: list[Path],
) -> dict[str, Any]:
    return {
        "source_id": source_id,
        "title": title,
        "source_url": source_url,
        "access_date": ACCESS_DATE,
        "license_or_terms": license_or_terms,
        "parser_version": PARSER_VERSION,
        "normalized_row_count": normalized_row_count,
        "status": "acquired",
        "parser_assumptions": parser_assumptions,
        "raw_files": file_entries(raw_files),
        "normalized_files": file_entries(normalized_files),
    }


def file_entries(paths: list[Path]) -> list[dict[str, Any]]:
    entries = []
    for path in paths:
        if path.is_file():
            entries.append(
                {
                    "path": str(path),
                    "sha256": sha256(path),
                    "bytes": path.stat().st_size,
                }
            )
        else:
            entries.append({"path": str(path), "missing": True})
    return entries


def write_observation_csv(path: Path, rows: list[ObservationRow]) -> None:
    rows = sorted(rows, key=lambda row: row.date)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=OBS_COLUMNS, lineterminator="\n")
        writer.writeheader()
        for row in rows:
            writer.writerow(row.as_csv_row())


def sciencebase_file_uri(document: dict[str, Any], name: str) -> str:
    for file_record in document.get("files", []):
        if file_record.get("name") == name:
            return file_record["downloadUri"]
    raise KeyError(f"ScienceBase file not found: {name}")


def download_json(url: str) -> Any:
    with urllib.request.urlopen(url, timeout=90) as response:
        return json.loads(response.read().decode("utf-8"))


def download_file(url: str, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with urllib.request.urlopen(url, timeout=90) as response:
        path.write_bytes(response.read())


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def require_file(path: Path) -> None:
    if not path.is_file():
        raise FileNotFoundError(f"required file missing: {path}")


def parse_number(value: Any) -> float | None:
    if value is None:
        return None
    text = str(value).strip()
    if text == "":
        return None
    try:
        return float(text)
    except ValueError:
        return None


def parse_optional_float(value: str) -> float | None:
    value = value.strip()
    if value == "":
        return None
    return float(value)


def parse_us_date(value: str) -> dt.date:
    return dt.datetime.strptime(value.strip(), "%m/%d/%Y").date()


def parse_slash_dmy(value: str) -> dt.date:
    return dt.datetime.strptime(value.replace(" ", ""), "%d/%m/%Y").date()


def water_year(value: dt.date) -> int:
    return value.year + 1 if value.month >= 10 else value.year


def mean_cm_to_m(values: list[float]) -> float | None:
    if not values:
        return None
    return sum(values) / len(values) / 100.0


def cm_to_m(value: float | None) -> float | None:
    if value is None:
        return None
    return value / 100.0


def format_optional(value: float | None) -> str:
    if value is None:
        return ""
    return f"{value:.6f}"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def toml_path(path: Path) -> str:
    return str(path).replace("\\", "\\\\").replace('"', '\\"')


if __name__ == "__main__":
    sys.exit(main())
