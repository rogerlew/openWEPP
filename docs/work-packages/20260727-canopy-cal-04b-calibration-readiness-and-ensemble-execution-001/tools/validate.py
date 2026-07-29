#!/usr/bin/env python3
"""Fail-closed terminal validator for CAL-04B result and holdout closure."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import struct
import subprocess
import sys
from collections import Counter, defaultdict
from datetime import date
from pathlib import Path

import numpy as np
from custody import (
    RECEIPT_FIELDS,
    read_csv_exact,
    sha256_file,
    validate_freeze,
    validate_receipt_barrier,
)

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
SOURCE_ARTIFACTS = PACKAGE / "artifacts"
ARTIFACTS = SOURCE_ARTIFACTS
OBJECTS = Path("/nonexistent/cal04b-execution-root-required")
EXECUTION_ROOT = Path("/nonexistent/cal04b-execution-root-required")
HOLDOUT_OBJECTS = Path("/nonexistent/cal04b-holdout-output-required")
HOLDOUT_TOKEN = Path("/nonexistent/cal04b-custody-token-required")


class ReadOverlay:
    def __init__(self, external: Path, source: Path) -> None:
        self.external = external
        self.source = source

    def __truediv__(self, name: str) -> Path:
        external = self.external / name
        return external if external.exists() else self.source / name

TRACE_MAGIC = b"CAL04B03"
TRACE_HEADER = struct.Struct("<8sIII")
CANDIDATE_COUNT = 9_261
LANE_COUNT = 9
FIRST_YEAR = 1989
LAST_YEAR = 2024
YEAR_COUNT = LAST_YEAR - FIRST_YEAR + 1
DAYS_PER_YEAR = 180
DAYS_PER_LANE = YEAR_COUNT * DAYS_PER_YEAR
TRACE_VALUE_COUNT = CANDIDATE_COUNT * LANE_COUNT * DAYS_PER_LANE
TRACE_BYTES = TRACE_HEADER.size + TRACE_VALUE_COUNT * 8
OBSERVATION_COUNT = 932
OBSERVATION_COMPONENT_COUNT = CANDIDATE_COUNT * OBSERVATION_COUNT
CROSSING_COMPONENT_COUNT = CANDIDATE_COUNT * LANE_COUNT * YEAR_COUNT
CANONICAL_LANES = ("1B", "4B", "4T", "5B", "5T", "6T", "7B", "7T", "HQ")
CAL04A = (
    ROOT
    / "docs/work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001"
    / "artifacts"
)
CONFIG_FIELDS = (
    "candidate_id",
    "temperature_pair_id",
    "vpd_pair_id",
    "photoperiod_pair_id",
    "minimum_temperature_inactive_c",
    "minimum_temperature_unconstrained_c",
    "vapor_pressure_deficit_unconstrained_pa",
    "vapor_pressure_deficit_inactive_pa",
    "photoperiod_inactive_hours",
    "photoperiod_unconstrained_hours",
    "boundary_class",
    "saturation_flags",
)
LANE_FIELDS = (
    "lane_index",
    "plot_id",
    "latitude_degrees",
    "longitude_degrees",
    "first_year",
    "last_year",
    "source_days_per_year",
    "source_day_count",
    "retained_days_per_year",
    "retained_day_count",
    "forcing_source_path",
    "forcing_source_sha256",
)
OBSERVATION_FIELDS = (
    "record_id",
    "year",
    "species",
    "plot_id",
    "source_elevation_m",
    "interval_start_doy",
    "interval_end_doy",
    "interval_width_days",
    "descriptive_midpoint_doy_not_truth",
    "start_tmin_c",
    "end_tmin_c",
    "end_21d_mean_tmin_c",
    "start_vpd_pa",
    "end_vpd_pa",
    "end_21d_mean_vpd_pa",
    "start_photoperiod_hours",
    "end_photoperiod_hours",
    "end_21d_mean_photoperiod_hours",
)
CROSSING_FIELDS = (
    "candidate_id",
    "plot_id",
    "lane_index",
    "year",
    "crossing_doy",
    "eligibility_start_yday",
    "eligibility_end_yday",
    "state",
)
COMPONENT_FIELDS = (
    "candidate_id",
    "plot_id",
    "lane_index",
    "record_id",
    "year",
    "species",
    "crossing_doy",
    "lower_doy",
    "upper_doy",
    "distance_days",
    "squared_distance",
)
ANNUAL_FIELDS = (
    "candidate_id",
    "year",
    "observation_count",
    "annual_mse",
    "annual_rmse",
)
DIAGNOSTIC_FIELDS = (
    "candidate_id",
    "species_rmse",
    "observation_median_absolute_distance",
    "year_median_absolute_distance",
    "interval_coverage_fraction",
    "failed_records",
    "failed_years",
)
FAILURE_FIELDS = (
    "failure_id",
    "candidate_id",
    "plot_id",
    "lane_index",
    "year",
    "stage",
    "failure_class",
    "attempt",
    "typed_error",
    "evidence",
)
PRODUCER_FAILURE_FIELDS = (
    "failure_id",
    "candidate_id",
    "lane_index",
    "plot_id",
    "year",
    "failure_class",
    "typed_error",
)
TIMING_FIELDS = (
    "record_id",
    "year",
    "season",
    "species",
    "site",
    "transition",
    "threshold_stage",
    "interval_start_date",
    "interval_start_doy",
    "interval_end_date",
    "interval_end_doy",
    "role",
    "source_object_id",
)


def rows(name: str) -> list[dict[str, str]]:
    output = ARTIFACTS / name
    path = output if output.is_file() else SOURCE_ARTIFACTS / name
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def direct_command_plan() -> list[dict[str, str]]:
    """Render calibration command identities from the canonical direct plan."""

    value = json.loads(
        (SOURCE_ARTIFACTS / "direct-execution-plan.json").read_text(
            encoding="utf-8"
        )
    )
    replacements = {
        "${REPO}": str(ROOT),
        "${OBJECTS_ROOT}": str(OBJECTS),
        "${PUBLICATION_ROOT}": str(EXECUTION_ROOT / "publication"),
        "${CARGO_TARGET_DIR}": str(EXECUTION_ROOT / "cargo-target"),
    }

    def expand(item: str) -> str:
        for token, replacement in replacements.items():
            item = item.replace(token, replacement)
        return item

    rendered = []
    for node in value["phases"]["calibration"]:
        environment = " ".join(
            f"{key}={expand(str(item))}"
            for key, item in node.get("env", {}).items()
        )
        argv = " ".join(expand(item) for item in node["argv"])
        source = Path(expand(node["source_path"]))
        if not source.is_absolute():
            source = ROOT / source
        rendered.append(
            {
                "command_id": node["command_id"],
                "argv": f"{environment} {argv}".strip(),
                "source_path": str(source),
            }
        )
    return rendered


def field_map(path: Path) -> dict[str, str]:
    values = read_csv_exact(path, ["field", "value"])
    result = {row["field"]: row["value"] for row in values}
    if len(result) != len(values):
        raise ValueError(f"duplicate field receipt rows in {path}")
    return result


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def exact_float(observed: str, expected: float) -> bool:
    try:
        return struct.pack("<d", float(observed)) == struct.pack("<d", expected)
    except (OverflowError, ValueError):
        return False


def aggregate_float_within_ulps(
    observed: str, expected: float, max_ulps: int = 4
) -> bool:
    """Bound cross-language aggregate differences after exact operand checks."""

    try:
        value = float(observed)
    except (OverflowError, ValueError):
        return False
    if not math.isfinite(value) or not math.isfinite(expected):
        return exact_float(observed, expected)
    return abs(value - expected) <= max_ulps * max(math.ulp(value), math.ulp(expected))


def repository_path(value: str) -> Path:
    path = Path(value)
    legacy_objects = Path("/home/workdir") / "cal04b-objects"
    if path.is_absolute():
        try:
            return OBJECTS / path.relative_to(legacy_objects)
        except ValueError:
            return path
    source_artifacts = PACKAGE.relative_to(ROOT) / "artifacts"
    try:
        return ARTIFACTS / path.relative_to(source_artifacts)
    except ValueError:
        return ROOT / path


def read_trace_header(path: Path) -> tuple[int, int, int]:
    with path.open("rb") as stream:
        raw = stream.read(TRACE_HEADER.size)
    require(len(raw) == TRACE_HEADER.size, "raw trace header is truncated")
    magic, candidates, lanes, days_per_lane = TRACE_HEADER.unpack(raw)
    require(magic == TRACE_MAGIC, "raw trace magic/schema differs")
    require(
        (candidates, lanes, days_per_lane)
        == (CANDIDATE_COUNT, LANE_COUNT, DAYS_PER_LANE),
        "raw trace header cardinality differs",
    )
    require(path.stat().st_size == TRACE_BYTES, "raw trace byte cardinality differs")
    return candidates, lanes, days_per_lane


def eligible_crossings(cube: np.ndarray) -> np.ndarray:
    """Derive first eligible upward crossings for [lane, year, yday]."""
    require(
        cube.shape == (LANE_COUNT, YEAR_COUNT, DAYS_PER_YEAR),
        "candidate trace cube shape differs",
    )
    require(
        bool(np.isfinite(cube).all())
        and bool(((cube >= 0.0) & (cube <= 1.0)).all()),
        "raw trace contains a nonfinite/out-of-range GSI",
    )
    transitions = (cube[:, :, 58:179] < 0.5) & (cube[:, :, 59:180] >= 0.5)
    found = transitions.any(axis=2)
    first = transitions.argmax(axis=2).astype(np.uint16) + 60
    return np.where(found, first, 0).astype(np.uint16)


def canonical_missing_groups(crossings: np.ndarray) -> list[tuple[str, int]]:
    require(
        crossings.shape == (LANE_COUNT, YEAR_COUNT),
        "crossing matrix shape differs",
    )
    return [
        (plot, year)
        for lane_index, plot in enumerate(CANONICAL_LANES)
        for year_index, year in enumerate(range(FIRST_YEAR, LAST_YEAR + 1))
        if int(crossings[lane_index, year_index]) == 0
    ]


def observed_missing_years(
    crossings: np.ndarray,
    observations: list[tuple[str, int, str, str, int, int]],
) -> set[int]:
    lane_by_plot = {plot: index for index, plot in enumerate(CANONICAL_LANES)}
    return {
        year
        for _, year, _, plot, _, _ in observations
        if int(crossings[lane_by_plot[plot], year - FIRST_YEAR]) == 0
    }


def equal_year_objective(year_mses: list[float]) -> float:
    require(bool(year_mses), "candidate has no admitted annual components")
    return math.sqrt(sum(year_mses) / len(year_mses))


def _csv_stream(path: Path, expected_fields: tuple[str, ...]):
    stream = path.open(newline="", encoding="utf-8")
    reader = csv.reader(stream)
    header = next(reader, None)
    if header != list(expected_fields):
        stream.close()
        raise ValueError(f"CSV schema differs for {path}")
    return stream, reader


def _next_exact(
    primary: csv.reader,
    verification: csv.reader,
    label: str,
) -> list[str]:
    left = next(primary, None)
    right = next(verification, None)
    require(left is not None, f"{label} ended early")
    require(left == right, f"dual reconstruction differs at {label}")
    return left


def _load_calibration_inputs():
    config_path = ARTIFACTS / "candidate-configurations.csv"
    with config_path.open(newline="", encoding="utf-8") as stream:
        config_reader = csv.DictReader(stream)
        require(tuple(config_reader.fieldnames or ()) == CONFIG_FIELDS, "configuration schema differs")
        configs = list(config_reader)
    require(len(configs) == CANDIDATE_COUNT, "configuration count differs")
    require(
        [row["candidate_id"] for row in configs]
        == [f"GSI-{index:04d}" for index in range(1, CANDIDATE_COUNT + 1)],
        "configuration identity/order differs",
    )

    observation_path = CAL04A / "phenology-forcing-join.csv"
    with observation_path.open(newline="", encoding="utf-8") as stream:
        observation_reader = csv.DictReader(stream)
        require(
            tuple(observation_reader.fieldnames or ()) == OBSERVATION_FIELDS,
            "authenticated Hubbard projection schema differs",
        )
        raw_observations = list(observation_reader)
    require(len(raw_observations) == OBSERVATION_COUNT, "Hubbard observation count differs")
    require(
        len({row["record_id"] for row in raw_observations}) == OBSERVATION_COUNT,
        "Hubbard record identities duplicate",
    )
    observations = []
    for row in raw_observations:
        plot = row["plot_id"]
        year = int(row["year"])
        lower = int(row["interval_start_doy"])
        upper = int(row["interval_end_doy"])
        require(plot in CANONICAL_LANES, f"observation has unauthenticated plot {plot}")
        require(FIRST_YEAR <= year <= LAST_YEAR, f"observation year out of range: {year}")
        require(60 <= lower <= upper <= DAYS_PER_YEAR, "observation interval is outside eligibility")
        observations.append(
            (
                row["record_id"],
                year,
                row["species"],
                plot,
                lower,
                upper,
            )
        )
    return configs, observations, observation_path


def _validate_lanes_and_calendar(identity: dict[str, str]) -> None:
    geometry_path = repository_path(identity["geometry_path"])
    with geometry_path.open(newline="", encoding="utf-8") as stream:
        geometry_reader = csv.DictReader(stream)
        require(
            tuple(geometry_reader.fieldnames or ())
            == (
                "plot_id",
                "latitude_deg",
                "longitude_deg",
                "source_elevation_ft",
                "source_elevation_m",
                "geometry_authority",
                "role",
            ),
            "Hubbard geometry schema differs",
        )
        geometry_rows = list(geometry_reader)
    geometry = {row["plot_id"]: row for row in geometry_rows}
    require(
        len(geometry) == LANE_COUNT and set(geometry) == set(CANONICAL_LANES),
        "Hubbard geometry lane inventory differs",
    )

    manifest_path = Path(identity["lane_manifest_path"])
    with manifest_path.open(newline="", encoding="utf-8") as stream:
        lane_reader = csv.DictReader(stream)
        require(tuple(lane_reader.fieldnames or ()) == LANE_FIELDS, "lane manifest schema differs")
        lanes = list(lane_reader)
    require(len(lanes) == LANE_COUNT, "lane manifest count differs")
    forcing_path = Path(identity["forcing_path"])
    for lane_index, (plot, row) in enumerate(zip(CANONICAL_LANES, lanes, strict=True)):
        authority = geometry[plot]
        require(
            authority["geometry_authority"] == "knb-lter-hbr.51.16 EML"
            and authority["role"] == "calibration"
            and math.isfinite(float(authority["source_elevation_m"]))
            and float(authority["source_elevation_m"]) > 0.0
            and row["lane_index"] == str(lane_index)
            and row["plot_id"] == plot
            and exact_float(row["latitude_degrees"], float(authority["latitude_deg"]))
            and exact_float(row["longitude_degrees"], float(authority["longitude_deg"]))
            and row["first_year"] == str(FIRST_YEAR)
            and row["last_year"] == str(LAST_YEAR)
            and row["source_days_per_year"] == "365"
            and row["source_day_count"] == "13140"
            and row["retained_days_per_year"] == str(DAYS_PER_YEAR)
            and row["retained_day_count"] == str(DAYS_PER_LANE)
            and Path(row["forcing_source_path"]) == forcing_path
            and row["forcing_source_sha256"] == identity["forcing_sha256"],
            f"lane manifest identity differs for {plot}",
        )

    calendar_path = Path(identity["calendar_path"])
    with calendar_path.open(newline="", encoding="utf-8") as stream:
        calendar = csv.reader(stream)
        require(
            next(calendar, None) == ["lane_index", "plot_id", "year", "yday"],
            "calibration calendar schema differs",
        )
        count = 0
        for lane_index, plot in enumerate(CANONICAL_LANES):
            for year in range(FIRST_YEAR, LAST_YEAR + 1):
                for yday in range(1, DAYS_PER_YEAR + 1):
                    require(
                        next(calendar, None)
                        == [str(lane_index), plot, str(year), str(yday)],
                        f"calibration calendar differs at {plot}/{year}/{yday}",
                    )
                    count += 1
        require(next(calendar, None) is None, "calibration calendar has extra rows")
    require(count == LANE_COUNT * YEAR_COUNT * DAYS_PER_YEAR, "calendar row count differs")


def validate_calibration_semantics(
    candidates: list[dict[str, str]],
    accepted: list[dict[str, str]],
    plan_by_id: dict[str, dict[str, str]],
) -> dict[str, str]:
    """Independently reconstruct every calibration result from CAL04B03."""
    trace_path = OBJECTS / "hubbard-gsi.bin"
    identity_path = OBJECTS / "hubbard-gsi-identity.csv"
    identity = field_map(identity_path)
    expected_identity = {
        "schema": "CAL04B03",
        "site_id": "hubbard_brook",
        "arm_id": "deciduous",
        "candidate_count": str(CANDIDATE_COUNT),
        "lane_count": str(LANE_COUNT),
        "days_per_lane": str(DAYS_PER_LANE),
        "source_days_per_plot_year": "365",
        "retained_days_per_plot_year": str(DAYS_PER_YEAR),
        "first_year": str(FIRST_YEAR),
        "last_year": str(LAST_YEAR),
        "state_initialization": "FRESH_GSI_STATE_EACH_CANDIDATE_PLOT_YEAR",
        "crossing_eligibility_yday": "60-180",
        "trace_order": "candidate_lane_year_yday",
    }
    require(
        all(identity.get(key) == value for key, value in expected_identity.items()),
        "CAL04B03 producer identity differs",
    )
    require(
        Path(identity["trace_path"]) == trace_path
        and identity["trace_bytes"] == str(TRACE_BYTES)
        and identity["exact_command"] == plan_by_id["hubbard_producer"]["argv"],
        "producer trace path/cardinality/argv differs",
    )
    require(
        repository_path(identity["calendar_path"]) == OBJECTS / "hubbard-gsi.calendar.csv"
        and repository_path(identity["lane_manifest_path"])
        == OBJECTS / "hubbard-gsi.lanes.csv"
        and repository_path(identity["forcing_path"]) == CAL04A / "daymet-daily-derived.csv"
        and repository_path(identity["geometry_path"]) == CAL04A / "hubbard-plot-geometry.csv",
        "producer calendar/lane/forcing/geometry paths differ",
    )
    read_trace_header(trace_path)
    for path_field, digest_field in (
        ("trace_path", "trace_sha256"),
        ("calendar_path", "calendar_sha256"),
        ("lane_manifest_path", "lane_manifest_sha256"),
        ("config_path", "config_sha256"),
        ("forcing_path", "forcing_sha256"),
        ("geometry_path", "geometry_sha256"),
        ("source_manifest_path", "source_manifest_sha256"),
        ("authority_manifest_path", "authority_manifest_sha256"),
        ("forcing_authority_resolution_path", "forcing_authority_resolution_sha256"),
        ("producer_source", "producer_source_sha256"),
        ("producer_binary", "producer_binary_sha256"),
        ("failure_ledger", "failure_ledger_sha256"),
    ):
        require(
            sha256_file(repository_path(identity[path_field])) == identity[digest_field],
            f"producer identity digest differs for {path_field}",
        )
    _validate_lanes_and_calendar(identity)
    configs, observations, observation_path = _load_calibration_inputs()
    require(
        repository_path(identity["config_path"])
        == ARTIFACTS / "candidate-configurations.csv",
        "config path differs",
    )
    with repository_path(identity["failure_ledger"]).open(
        newline="", encoding="utf-8"
    ) as stream:
        producer_failures = csv.reader(stream)
        require(
            next(producer_failures, None) == list(PRODUCER_FAILURE_FIELDS),
            "producer failure ledger schema differs",
        )
        require(next(producer_failures, None) is None, "successful producer retained failures")

    primary_dir = OBJECTS / "primary"
    verification_dir = OBJECTS / "verification"
    big_ledgers = (
        "candidate-crossing-components.csv",
        "candidate-observation-components.csv",
        "candidate-annual-components.csv",
        "candidate-diagnostics.csv",
    )
    ledger_hashes: dict[str, str] = {}
    for name in big_ledgers:
        primary_hash = sha256_file(primary_dir / name)
        verification_hash = sha256_file(verification_dir / name)
        require(
            primary_hash == verification_hash,
            f"byte-exact dual reconstruction differs for {name}",
        )
        ledger_hashes[name] = primary_hash
    require(
        sha256_file(ARTIFACTS / "failure-ledger.csv")
        == sha256_file(verification_dir / "failure-ledger.csv"),
        "byte-exact dual reconstruction differs for failure-ledger.csv",
    )
    primary_receipt = field_map(primary_dir / "reconstruction-receipt.csv")
    verification_receipt = field_map(verification_dir / "verification-receipt.csv")
    for field, name in (
        ("crossing_components_sha256", "candidate-crossing-components.csv"),
        ("observation_components_sha256", "candidate-observation-components.csv"),
        ("annual_components_sha256", "candidate-annual-components.csv"),
        ("diagnostics_sha256", "candidate-diagnostics.csv"),
    ):
        require(
            primary_receipt.get(field) == ledger_hashes[name]
            and verification_receipt.get(field) == ledger_hashes[name],
            f"reconstruction receipt digest differs for {name}",
        )

    crossing_stream, crossing_reader = _csv_stream(
        primary_dir / "candidate-crossing-components.csv", CROSSING_FIELDS
    )
    crossing_verify_stream, crossing_verify_reader = _csv_stream(
        verification_dir / "candidate-crossing-components.csv", CROSSING_FIELDS
    )
    component_stream, component_reader = _csv_stream(
        primary_dir / "candidate-observation-components.csv", COMPONENT_FIELDS
    )
    component_verify_stream, component_verify_reader = _csv_stream(
        verification_dir / "candidate-observation-components.csv", COMPONENT_FIELDS
    )
    annual_stream, annual_reader = _csv_stream(
        primary_dir / "candidate-annual-components.csv", ANNUAL_FIELDS
    )
    annual_verify_stream, annual_verify_reader = _csv_stream(
        verification_dir / "candidate-annual-components.csv", ANNUAL_FIELDS
    )
    diagnostics_stream, diagnostics_reader = _csv_stream(
        primary_dir / "candidate-diagnostics.csv", DIAGNOSTIC_FIELDS
    )
    diagnostics_verify_stream, diagnostics_verify_reader = _csv_stream(
        verification_dir / "candidate-diagnostics.csv", DIAGNOSTIC_FIELDS
    )
    failure_stream, failure_reader = _csv_stream(
        ARTIFACTS / "failure-ledger.csv", FAILURE_FIELDS
    )
    failure_verify_stream, failure_verify_reader = _csv_stream(
        verification_dir / "failure-ledger.csv", FAILURE_FIELDS
    )
    trace = np.memmap(
        trace_path,
        dtype="<f8",
        mode="r",
        offset=TRACE_HEADER.size,
        shape=(CANDIDATE_COUNT, LANE_COUNT, YEAR_COUNT, DAYS_PER_YEAR),
    )
    observation_year_counts = Counter(observation[1] for observation in observations)
    lane_by_plot = {plot: index for index, plot in enumerate(CANONICAL_LANES)}
    derived_objectives: list[float] = []
    crossing_count = 0
    component_count = 0
    annual_count = 0
    diagnostic_count = 0
    failure_count = 0
    try:
        for candidate_index, (config, candidate_row) in enumerate(
            zip(configs, candidates, strict=True)
        ):
            candidate_id = config["candidate_id"]
            require(candidate_row["candidate_id"] == candidate_id, "candidate ledger order differs")
            crossings = eligible_crossings(trace[candidate_index])
            for lane_index, plot in enumerate(CANONICAL_LANES):
                for year_index, year in enumerate(range(FIRST_YEAR, LAST_YEAR + 1)):
                    crossing = int(crossings[lane_index, year_index])
                    row = _next_exact(
                        crossing_reader,
                        crossing_verify_reader,
                        f"crossing {candidate_id}/{plot}/{year}",
                    )
                    expected = [
                        candidate_id,
                        plot,
                        str(lane_index),
                        str(year),
                        str(crossing) if crossing else "",
                        "60",
                        "180",
                        "FOUND" if crossing else "MISSING",
                    ]
                    require(row == expected, f"raw-derived crossing differs for {candidate_id}/{plot}/{year}")
                    crossing_count += 1

            squares_by_year: dict[int, list[float]] = defaultdict(list)
            record_failed_years = observed_missing_years(crossings, observations)
            squares_by_species: dict[str, list[float]] = defaultdict(list)
            absolute_distances: list[float] = []
            coverage = 0
            failed_records = 0
            for record_id, year, species, plot, lower, upper in observations:
                crossing = int(crossings[lane_by_plot[plot], year - FIRST_YEAR])
                if crossing:
                    distance = max(lower - crossing, 0) + max(crossing - upper, 0)
                    square = float(distance * distance)
                    squares_by_year[year].append(square)
                    squares_by_species[species].append(square)
                    absolute_distances.append(float(distance))
                    coverage += int(distance == 0)
                else:
                    distance = math.inf
                    square = math.inf
                    failed_records += 1
                row = _next_exact(
                    component_reader,
                    component_verify_reader,
                    f"observation {candidate_id}/{record_id}",
                )
                require(
                    row[:9]
                    == [
                        candidate_id,
                        plot,
                        str(lane_by_plot[plot]),
                        record_id,
                        str(year),
                        species,
                        str(crossing) if crossing else "",
                        str(lower),
                        str(upper),
                    ],
                    f"observation authority/order differs for {candidate_id}/{record_id}",
                )
                require(
                    exact_float(row[9], float(distance)) and exact_float(row[10], square),
                    f"raw-derived observation component differs for {candidate_id}/{record_id}",
                )
                component_count += 1
            failed_groups = canonical_missing_groups(crossings)
            failed_years = {year for _, year in failed_groups}

            year_mses: list[float] = []
            year_median_distances: list[float] = []
            for year in sorted(observation_year_counts):
                count = observation_year_counts[year]
                if year in record_failed_years:
                    mse = math.inf
                    rmse = math.inf
                else:
                    squares = squares_by_year[year]
                    require(
                        len(squares) == count,
                        f"annual record completeness differs for {candidate_id}/{year}",
                    )
                    mse = sum(squares) / count
                    rmse = math.sqrt(mse)
                    year_mses.append(mse)
                    year_median_distances.append(
                        median([math.sqrt(square) for square in squares])
                    )
                row = _next_exact(
                    annual_reader,
                    annual_verify_reader,
                    f"annual {candidate_id}/{year}",
                )
                require(
                    row[:3] == [candidate_id, str(year), str(count)]
                    and exact_float(row[3], mse)
                    and exact_float(row[4], rmse),
                    f"record-within-year reconstruction differs for {candidate_id}/{year}",
                )
                annual_count += 1
            species_parts = [
                f"{species}:{math.sqrt(sum(squares) / len(squares)):.9f}"
                for species, squares in sorted(squares_by_species.items())
            ]
            row = _next_exact(
                diagnostics_reader,
                diagnostics_verify_reader,
                f"diagnostics {candidate_id}",
            )
            require(
                row[:2] == [candidate_id, ";".join(species_parts)]
                and exact_float(
                    row[2],
                    median(absolute_distances) if absolute_distances else math.inf,
                )
                and exact_float(
                    row[3],
                    median(year_median_distances) if year_median_distances else math.inf,
                )
                and exact_float(row[4], coverage / OBSERVATION_COUNT)
                and row[5] == str(failed_records)
                and row[6] == str(len(failed_years)),
                f"raw-derived diagnostics differ for {candidate_id}",
            )
            diagnostic_count += 1
            for plot, year in failed_groups:
                failure_count += 1
                row = _next_exact(
                    failure_reader,
                    failure_verify_reader,
                    f"typed failure {candidate_id}/{plot}/{year}",
                )
                require(
                    row
                    == [
                        f"FAIL-{failure_count:06d}",
                        candidate_id,
                        plot,
                        str(lane_by_plot[plot]),
                        str(year),
                        "gsi_timing",
                        "MISSING_REQUIRED_PLOT_YEAR_CROSSING",
                        "1",
                        "objective_positive_infinity",
                        "eligibility_yday_60_180",
                    ],
                    f"typed failure differs for {candidate_id}/{plot}/{year}",
                )
            objective = math.inf if failed_groups else equal_year_objective(year_mses)
            derived_objectives.append(objective)
            configuration_id = "|".join(
                (config["temperature_pair_id"], config["vpd_pair_id"], config["photoperiod_pair_id"])
            )
            require(
                candidate_row["configuration_id"] == configuration_id
                and candidate_row["state"]
                == ("FINITE" if math.isfinite(objective) else "FAILED_REQUIRED_PLOT_YEAR_CROSSING")
                and aggregate_float_within_ulps(
                    candidate_row["objective"], objective
                )
                and candidate_row["boundary_flags"] == config["boundary_class"]
                and candidate_row["saturation_flags"] == config["saturation_flags"]
                and Path(candidate_row["evidence"])
                == primary_dir / "candidate-observation-components.csv",
                f"candidate objective round trip differs for {candidate_id}",
            )
        require(next(crossing_reader, None) is None, "crossing ledger has extra rows")
        require(next(crossing_verify_reader, None) is None, "verification crossing ledger has extra rows")
        require(next(component_reader, None) is None, "observation ledger has extra rows")
        require(next(component_verify_reader, None) is None, "verification observation ledger has extra rows")
        require(next(annual_reader, None) is None, "annual ledger has extra rows")
        require(next(annual_verify_reader, None) is None, "verification annual ledger has extra rows")
        require(next(diagnostics_reader, None) is None, "diagnostics ledger has extra rows")
        require(
            next(diagnostics_verify_reader, None) is None,
            "verification diagnostics ledger has extra rows",
        )
        require(next(failure_reader, None) is None, "failure ledger has extra rows")
        require(
            next(failure_verify_reader, None) is None,
            "verification failure ledger has extra rows",
        )
    finally:
        crossing_stream.close()
        crossing_verify_stream.close()
        component_stream.close()
        component_verify_stream.close()
        annual_stream.close()
        annual_verify_stream.close()
        diagnostics_stream.close()
        diagnostics_verify_stream.close()
        failure_stream.close()
        failure_verify_stream.close()
        del trace

    require(crossing_count == CROSSING_COMPONENT_COUNT, "candidate crossing cardinality differs")
    require(component_count == OBSERVATION_COMPONENT_COUNT, "observation component cardinality differs")
    require(annual_count == CANDIDATE_COUNT * len(observation_year_counts), "annual cardinality differs")
    require(diagnostic_count == CANDIDATE_COUNT, "candidate diagnostics cardinality differs")
    finite = [value for value in derived_objectives if math.isfinite(value)]
    require(bool(finite), "raw trace produces no finite candidate objective")
    threshold = min(finite) + 1.0
    expected_members = [
        config["candidate_id"]
        for config, objective in zip(configs, derived_objectives, strict=True)
        if math.isfinite(objective) and objective <= threshold
    ]
    require(
        [row["candidate_id"] for row in accepted] == expected_members,
        "accepted membership does not round-trip from raw trace",
    )
    derived_by_id = dict(
        zip((config["candidate_id"] for config in configs), derived_objectives, strict=True)
    )
    config_by_id = {config["candidate_id"]: config for config in configs}
    for row in accepted:
        config = config_by_id[row["candidate_id"]]
        require(
            aggregate_float_within_ulps(
                row["objective"], derived_by_id[row["candidate_id"]]
            )
            and exact_float(row["acceptance_threshold"], threshold)
            and row["boundary_flags"] == config["boundary_class"]
            and row["saturation_flags"] == config["saturation_flags"]
            and row["state"] == "ACCEPTED_FROZEN",
            f"accepted membership row differs for {row['candidate_id']}",
        )
    for field, name in (
        ("candidate_ledger_sha256", "candidate-ledger.csv"),
        ("accepted_ensemble_sha256", "accepted-calibration-ensemble.csv"),
        ("failure_ledger_sha256", "failure-ledger.csv"),
    ):
        package_hash = sha256_file(ARTIFACTS / name)
        require(
            verification_receipt.get(field) == package_hash
            and sha256_file(verification_dir / name) == package_hash,
            f"verification receipt/round trip differs for {name}",
        )
    return identity


def line_rows(path: Path) -> int:
    with path.open("rb") as stream:
        return max(sum(block.count(b"\n") for block in iter(lambda: stream.read(8 * 1024 * 1024), b"")) - 1, 0)


def zstd_expanded_sha(path: Path) -> str:
    process = subprocess.Popen(["zstd", "-dc", str(path)], stdout=subprocess.PIPE)
    if process.stdout is None:
        raise OSError("zstd stdout unavailable")
    digest = hashlib.sha256()
    for block in iter(lambda: process.stdout.read(1024 * 1024), b""):
        digest.update(block)
    if process.wait() != 0:
        raise ValueError("zstd expansion failed")
    return digest.hexdigest()


def verifier_command(
    verifier_id: str, execution_root: Path, custody_root: Path
) -> str:
    return (
        "PYTHONDONTWRITEBYTECODE=1 .venv/bin/python "
        f"{PACKAGE.relative_to(ROOT)}/tools/freeze-verify.py "
        f"--execution-root {execution_root} --custody-root {custody_root} "
        f"--verifier-id {verifier_id}"
    )


def holdout_opening_command(
    execution_root: Path,
    custody_root: Path,
    output_root: Path,
    token: Path,
) -> str:
    return " ".join(
        [
            str(ROOT / ".venv/bin/python"),
            str(PACKAGE / "tools/holdout.py"),
            "--sandboxed",
            "--execution-root",
            str(execution_root),
            "--custody-root",
            str(custody_root),
            "--holdout-output-root",
            str(output_root),
            "--opening-token",
            str(token),
        ]
    )


MEMBERSHIP_FIELDS = [
    "stage_member_id",
    "design_id",
    "gsi_candidate_id",
    "bf_max_kg_m2",
    "structural_biomass_kg_m2",
    "evergreen_fraction",
    "xmxlai_m2_m2",
    "structural_cover_fraction",
    "bb_m2_kg",
    "parent_stage_member_id",
    "parent_membership_sha256",
    "state",
]
PARENT_RESULT_FIELDS = [
    "stage_member_id",
    "parent_stage_member_id",
    "design_id",
    "result_template_id",
    "result_template_sha256",
    "state",
]


def expected_stage_rows(
    stage: str,
    accepted_ids: list[str],
    parent_hash: str,
    result_hash: str,
):
    sums = [18.742, 18.990, 19.238]
    bfs = [0.10, 0.20, 0.30]
    fes = [0.0, 0.25, 0.5, 0.75, 1.0]
    lais = [3.5, 4.0, 5.0, 6.0, 7.0, 8.0]
    css = [0.0, 0.2, 0.5, 0.8]
    bbs = [1.0, 2.5, 5.0, 10.0]
    for gsi in accepted_ids:
        for sum_index, total in enumerate(sums, 1):
            for bf_index, bf in enumerate(bfs, 1):
                bs = total - bf
                bfbs = f"BFBS-{gsi}-{sum_index}-{bf_index}"
                if stage == "foliar_structural_partition":
                    yield (
                        {
                            "stage_member_id": bfbs,
                            "design_id": "EMP-BFBS-01",
                            "gsi_candidate_id": gsi,
                            "bf_max_kg_m2": f"{bf:.3f}",
                            "structural_biomass_kg_m2": f"{bs:.3f}",
                            "evergreen_fraction": "",
                            "xmxlai_m2_m2": "",
                            "structural_cover_fraction": "",
                            "bb_m2_kg": "",
                            "parent_stage_member_id": gsi,
                            "parent_membership_sha256": parent_hash,
                            "state": "ACCEPTED_COMBINATION_CONSTRAINT",
                        },
                        {
                            "stage_member_id": bfbs,
                            "parent_stage_member_id": gsi,
                            "design_id": "EMP-BFBS-01",
                            "result_template_id": f"EMP-BFBS-{sum_index}-{bf_index}",
                            "result_template_sha256": result_hash,
                            "state": "EXECUTED_AND_RETAINED",
                        },
                    )
                    continue
                for fe_index, fe in enumerate(fes, 1):
                    fe_id = f"FE-{gsi}-{sum_index}-{bf_index}-{fe_index}"
                    if stage == "evergreen_fraction":
                        yield (
                            {
                                "stage_member_id": fe_id,
                                "design_id": "REC-FE-01",
                                "gsi_candidate_id": gsi,
                                "bf_max_kg_m2": f"{bf:.3f}",
                                "structural_biomass_kg_m2": f"{bs:.3f}",
                                "evergreen_fraction": f"{fe:.2f}",
                                "xmxlai_m2_m2": "",
                                "structural_cover_fraction": "",
                                "bb_m2_kg": "",
                                "parent_stage_member_id": bfbs,
                                "parent_membership_sha256": parent_hash,
                                "state": "RETAINED_SYNTHETIC_READINESS",
                            },
                            {
                                "stage_member_id": fe_id,
                                "parent_stage_member_id": bfbs,
                                "design_id": "REC-FE-01",
                                "result_template_id": f"REC-FE-{fe_index}",
                                "result_template_sha256": result_hash,
                                "state": "EXECUTED_AND_RETAINED",
                            },
                        )
                        continue
                    for lai_index, lai in enumerate(lais, 1):
                        lai_id = f"LAI-{gsi}-{sum_index}-{bf_index}-{fe_index}-{lai_index}"
                        if stage == "peak_lai":
                            yield (
                                {
                                    "stage_member_id": lai_id,
                                    "design_id": "EMP-LAI-01",
                                    "gsi_candidate_id": gsi,
                                    "bf_max_kg_m2": f"{bf:.3f}",
                                    "structural_biomass_kg_m2": f"{bs:.3f}",
                                    "evergreen_fraction": f"{fe:.2f}",
                                    "xmxlai_m2_m2": f"{lai:.2f}",
                                    "structural_cover_fraction": "",
                                    "bb_m2_kg": "",
                                    "parent_stage_member_id": fe_id,
                                    "parent_membership_sha256": parent_hash,
                                    "state": "ACCEPTED_CONDITIONAL_MATURE_LAI",
                                },
                                {
                                    "stage_member_id": lai_id,
                                    "parent_stage_member_id": fe_id,
                                    "design_id": "EMP-LAI-01",
                                    "result_template_id": f"EMP-LAI-{lai_index}",
                                    "result_template_sha256": result_hash,
                                    "state": "EXECUTED_AND_RETAINED",
                                },
                            )
                            continue
                        for cs_index, cs in enumerate(css, 1):
                            for bb_index, bb in enumerate(bbs, 1):
                                member = (
                                    f"CSBB-{gsi}-{sum_index}-{bf_index}-{fe_index}-"
                                    f"{lai_index}-{cs_index}-{bb_index}"
                                )
                                yield (
                                    {
                                        "stage_member_id": member,
                                        "design_id": "REC-CSBB-01",
                                        "gsi_candidate_id": gsi,
                                        "bf_max_kg_m2": f"{bf:.3f}",
                                        "structural_biomass_kg_m2": f"{bs:.3f}",
                                        "evergreen_fraction": f"{fe:.2f}",
                                        "xmxlai_m2_m2": f"{lai:.2f}",
                                        "structural_cover_fraction": f"{cs:.2f}",
                                        "bb_m2_kg": f"{bb:.2f}",
                                        "parent_stage_member_id": lai_id,
                                        "parent_membership_sha256": parent_hash,
                                        "state": "RETAINED_SYNTHETIC_READINESS",
                                    },
                                    {
                                        "stage_member_id": member,
                                        "parent_stage_member_id": lai_id,
                                        "design_id": "REC-CSBB-01",
                                        "result_template_id": (
                                            f"REC-CSBB-{(cs_index - 1) * len(bbs) + bb_index}"
                                        ),
                                        "result_template_sha256": result_hash,
                                        "state": "EXECUTED_AND_RETAINED",
                                    },
                                )


def validate_stage_propagation(
    index_row: dict[str, str],
    accepted_ids: list[str],
    result_hash: str,
) -> None:
    membership_path = Path(index_row["membership_path"])
    parent_results_path = Path(index_row["parent_results_path"])
    with (
        membership_path.open(newline="", encoding="utf-8") as membership_stream,
        parent_results_path.open(newline="", encoding="utf-8") as result_stream,
    ):
        memberships = csv.DictReader(membership_stream)
        parent_results = csv.DictReader(result_stream)
        require(memberships.fieldnames == MEMBERSHIP_FIELDS, "membership schema differs")
        require(parent_results.fieldnames == PARENT_RESULT_FIELDS, "parent-result schema differs")
        count = 0
        for expected_membership, expected_result in expected_stage_rows(
            index_row["stage"],
            accepted_ids,
            index_row["parent_membership_sha256"],
            result_hash,
        ):
            observed_membership = next(memberships, None)
            observed_result = next(parent_results, None)
            require(observed_membership == expected_membership, f"membership row differs for {index_row['stage']}")
            require(observed_result == expected_result, f"parent-result row differs for {index_row['stage']}")
            count += 1
        require(next(memberships, None) is None, f"extra membership row for {index_row['stage']}")
        require(next(parent_results, None) is None, f"extra parent-result row for {index_row['stage']}")
    require(count == int(index_row["membership_rows"]), f"propagation row count differs for {index_row['stage']}")


def median(values: list[float]) -> float:
    values = sorted(values)
    middle = len(values) // 2
    return (
        0.5 * (values[middle - 1] + values[middle])
        if len(values) % 2 == 0
        else values[middle]
    )


def same_number(observed: str, expected: float, tolerance: float = 1.0e-9) -> bool:
    value = float(observed)
    if math.isinf(expected):
        return math.isinf(value) and value > 0.0
    return math.isfinite(value) and abs(value - expected) <= tolerance


def derive_holdout_crossings(
    accepted_ids: list[str],
) -> dict[str, dict[int, int]]:
    calendar_path = HOLDOUT_OBJECTS / "harvard-gsi.calendar.csv"
    calendar: list[tuple[int, int]] = []
    with calendar_path.open(newline="", encoding="utf-8") as stream:
        reader = csv.DictReader(stream)
        require(reader.fieldnames == ["year", "ordinal"], "holdout calendar schema differs")
        calendar = [(int(row["year"]), int(row["ordinal"])) for row in reader]
    require(
        len(calendar) == 12_053
        and calendar[0] == (1991, 1)
        and calendar[-1] == (2023, 365),
        "holdout calendar extent differs",
    )
    for left, right in zip(calendar, calendar[1:]):
        year_days = 366 if left[0] % 4 == 0 and (left[0] % 100 != 0 or left[0] % 400 == 0) else 365
        expected = (left[0] + 1, 1) if left[1] == year_days else (left[0], left[1] + 1)
        require(right == expected, "holdout calendar is not consecutive")
    trace_path = HOLDOUT_OBJECTS / "harvard-gsi.bin"
    result: dict[str, dict[int, int]] = {}
    with trace_path.open("rb") as stream:
        header = stream.read(20)
        require(len(header) == 20 and header[:8] == b"CAL04B02", "holdout trace header differs")
        count, first_year, days = struct.unpack("<IiI", header[8:])
        require(
            count == len(accepted_ids) and first_year == 1991 and days == len(calendar),
            "holdout trace dimensions differ",
        )
        for candidate in accepted_ids:
            crossings: dict[int, int] = {}
            previous: float | None = None
            for year_day in calendar:
                encoded = stream.read(8)
                require(len(encoded) == 8, f"holdout trace truncated for {candidate}")
                current = struct.unpack("<d", encoded)[0]
                require(math.isfinite(current) and 0.0 <= current <= 1.0, "holdout trace daily value invalid")
                if (
                    year_day[0] not in crossings
                    and previous is not None
                    and previous > 0.5
                    and current <= 0.5
                ):
                    crossings[year_day[0]] = year_day[1]
                previous = current
            result[candidate] = crossings
        require(stream.read(1) == b"", "holdout trace has extra bytes")
    return result


def load_holdout_authority() -> list[dict[str, str]]:
    require(
        HOLDOUT_TOKEN.is_file(),
        "holdout authority cannot be resolved before OPENED_ONCE",
    )
    timing_path = (
        ROOT
        / "docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001"
        / "artifacts/cal04-timing-windows.csv"
    )
    with timing_path.open(newline="", encoding="utf-8") as stream:
        reader = csv.DictReader(stream)
        require(tuple(reader.fieldnames or ()) == TIMING_FIELDS, "timing authority schema differs")
        admitted = [row for row in reader if row["role"] == "INDEPENDENT_HOLDOUT"]
    require(len(admitted) == 319, "independent holdout authority count differs")
    require(
        len({row["record_id"] for row in admitted}) == len(admitted),
        "independent holdout record IDs duplicate",
    )
    for row in admitted:
        year = int(row["year"])
        lower = int(row["interval_start_doy"])
        upper = int(row["interval_end_doy"])
        start = date.fromisoformat(row["interval_start_date"])
        end = date.fromisoformat(row["interval_end_date"])
        require(
            1991 <= year <= 2023
            and year != 1992
            and row["season"] == "FALL"
            and row["site"] == "HARVARD_FOREST"
            and row["transition"] == "LEAF_FALL"
            and row["threshold_stage"] == "50_percent"
            and row["source_object_id"] == "SRC-HF-PHENO-HF003-V37"
            and start.year == year
            and end.year == year
            and start.timetuple().tm_yday == lower
            and end.timetuple().tm_yday == upper
            and 1 <= lower < upper <= (366 if year % 4 == 0 else 365),
            f"independent holdout authority differs for {row['record_id']}",
        )
    return admitted


def validate_holdout_arithmetic(accepted_ids: list[str]) -> None:
    authority = load_holdout_authority()
    derived_crossings = derive_holdout_crossings(accepted_ids)
    observation_path = HOLDOUT_OBJECTS / "harvard-observation-components.csv"
    annual_path = HOLDOUT_OBJECTS / "harvard-annual-components.csv"
    with (
        observation_path.open(newline="", encoding="utf-8") as observation_stream,
        annual_path.open(newline="", encoding="utf-8") as annual_stream,
    ):
        observations = csv.DictReader(observation_stream)
        annuals = csv.DictReader(annual_stream)
        require(
            observations.fieldnames
            == [
                "candidate_id",
                "record_id",
                "year",
                "species",
                "crossing_doy",
                "lower_doy",
                "upper_doy",
                "distance_days",
                "squared_distance",
            ],
            "holdout observation schema differs",
        )
        require(
            annuals.fieldnames
            == [
                "candidate_id",
                "year",
                "crossing_doy",
                "observation_count",
                "annual_mse",
                "annual_rmse",
            ],
            "holdout annual schema differs",
        )
        results = {row["candidate_id"]: row for row in rows("harvard-holdout-results.csv")}
        require(len(results) == len(accepted_ids), "holdout results duplicate candidate IDs")
        for candidate in accepted_ids:
            component_rows = [next(observations, None) for _ in range(319)]
            require(all(row is not None for row in component_rows), f"holdout components truncated for {candidate}")
            typed_rows = [row for row in component_rows if row is not None]
            require(
                all(row["candidate_id"] == candidate for row in typed_rows)
                and len({row["record_id"] for row in typed_rows}) == 319
                and all(
                    row["record_id"] == admitted["record_id"]
                    and row["year"] == admitted["year"]
                    and row["species"] == admitted["species"]
                    and row["lower_doy"] == admitted["interval_start_doy"]
                    and row["upper_doy"] == admitted["interval_end_doy"]
                    for row, admitted in zip(typed_rows, authority, strict=True)
                ),
                f"holdout component membership differs for {candidate}",
            )
            by_year: dict[int, list[dict[str, str]]] = {}
            by_species: dict[str, list[float]] = {}
            finite_distances: list[float] = []
            coverage = 0
            failed_records = 0
            for row in typed_rows:
                year = int(row["year"])
                by_year.setdefault(year, []).append(row)
                expected_crossing = derived_crossings[candidate].get(year)
                if expected_crossing is not None:
                    require(
                        row["crossing_doy"] == str(expected_crossing),
                        "holdout component crossing differs from raw trace",
                    )
                    crossing = int(row["crossing_doy"])
                    lower = int(row["lower_doy"])
                    upper = int(row["upper_doy"])
                    distance = max(lower - crossing, 0) + max(crossing - upper, 0)
                    require(same_number(row["distance_days"], float(distance)), "holdout distance differs")
                    require(same_number(row["squared_distance"], float(distance * distance)), "holdout square differs")
                    finite_distances.append(float(distance))
                    by_species.setdefault(row["species"], []).append(float(distance * distance))
                    coverage += int(distance == 0)
                else:
                    require(
                        row["crossing_doy"] == "",
                        "holdout component reports a crossing absent from raw trace",
                    )
                    failed_records += 1
                    require(
                        math.isinf(float(row["distance_days"]))
                        and math.isinf(float(row["squared_distance"])),
                        "missing holdout crossing is not retained as infinity",
                    )
            annual_mses: list[float] = []
            annual_medians: list[float] = []
            failed_years: list[int] = []
            for year in sorted(by_year):
                annual = next(annuals, None)
                require(annual is not None and annual["candidate_id"] == candidate and int(annual["year"]) == year, "holdout annual order differs")
                year_rows = by_year[year]
                require(int(annual["observation_count"]) == len(year_rows), "holdout annual count differs")
                if any(not row["crossing_doy"] for row in year_rows):
                    failed_years.append(year)
                    require(math.isinf(float(annual["annual_mse"])) and math.isinf(float(annual["annual_rmse"])), "failed holdout year is not infinity")
                else:
                    squares = [float(row["squared_distance"]) for row in year_rows]
                    mse = sum(squares) / len(squares)
                    require(same_number(annual["annual_mse"], mse) and same_number(annual["annual_rmse"], math.sqrt(mse)), "holdout annual arithmetic differs")
                    crossings = {row["crossing_doy"] for row in year_rows}
                    require(len(crossings) == 1 and annual["crossing_doy"] in crossings, "holdout annual crossing differs")
                    annual_mses.append(mse)
                    annual_medians.append(median([math.sqrt(value) for value in squares]))
            expected_aggregate = (
                math.sqrt(sum(annual_mses) / len(annual_mses))
                if not failed_years and failed_records == 0
                else math.inf
            )
            result = results[candidate]
            require(same_number(result["aggregate_score"], expected_aggregate), "holdout aggregate differs")
            expected_state = "SCORED_NO_REFIT" if math.isfinite(expected_aggregate) else "RETAINED_VALIDATION_FAILURE"
            require(result["state"] == expected_state, "holdout state differs")
            require(int(result["failed_records"]) == failed_records, "holdout failed-record count differs")
            require(
                result["failed_years"] == ";".join(map(str, failed_years)),
                "holdout failed-year inventory differs",
            )
            expected_species = ";".join(
                f"{species}:{math.sqrt(sum(values) / len(values)):.9f}"
                for species, values in sorted(by_species.items())
            )
            require(result["species_rmse"] == expected_species, "holdout species diagnostic differs")
            if finite_distances:
                require(same_number(result["observation_median_absolute_distance"], median(finite_distances)), "holdout observation median differs")
            else:
                require(
                    same_number(result["observation_median_absolute_distance"], math.inf),
                    "empty holdout observation median is not infinity",
                )
            if annual_medians:
                require(same_number(result["year_median_absolute_distance"], median(annual_medians)), "holdout year median differs")
            else:
                require(
                    same_number(result["year_median_absolute_distance"], math.inf),
                    "empty holdout year median is not infinity",
                )
            require(
                same_number(result["interval_coverage_fraction"], coverage / 319.0),
                "holdout coverage differs",
            )
        require(next(observations, None) is None, "extra holdout observation components")
        require(next(annuals, None) is None, "extra holdout annual components")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    parser.add_argument("--custody-root", type=Path, required=True)
    parser.add_argument("--holdout-output-root", type=Path)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    if not execution_root.is_dir():
        raise ValueError("execution root must be an existing directory")
    global ARTIFACTS, OBJECTS, EXECUTION_ROOT, HOLDOUT_OBJECTS, HOLDOUT_TOKEN
    EXECUTION_ROOT = execution_root
    attempt_root = execution_root.parent
    custody_root = options.custody_root.resolve(strict=True)
    if not custody_root.is_dir():
        raise ValueError("custody root must be an existing directory")
    if options.holdout_output_root is None:
        raise ValueError("terminal validation requires --holdout-output-root")
    holdout_output_root = options.holdout_output_root.resolve(strict=True)
    EXECUTION_ROOT = attempt_root
    external_artifacts = (
        attempt_root / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"
    )
    result_artifacts = (
        holdout_output_root / "artifacts"
    )
    ARTIFACTS = ReadOverlay(result_artifacts, external_artifacts)
    OBJECTS = execution_root
    HOLDOUT_OBJECTS = holdout_output_root / "objects"
    HOLDOUT_TOKEN = custody_root / "holdout-opened-once.lock"
    command_plan = direct_command_plan()
    plan_by_id = {row["command_id"]: row for row in command_plan}
    require(len(plan_by_id) == len(command_plan), "command plan IDs duplicate")
    candidates = rows("candidate-ledger.csv")
    accepted = rows("accepted-calibration-ensemble.csv")
    require(len(candidates) == 9_261, f"candidate result count {len(candidates)}")
    require(bool(accepted), "accepted ensemble is empty")
    ordered_candidate_ids = [row["candidate_id"] for row in candidates]
    require(
        ordered_candidate_ids == [f"GSI-{serial:04d}" for serial in range(1, 9_262)],
        "candidate result IDs/order differ",
    )
    candidate_ids = set(ordered_candidate_ids)
    accepted_ids = [row["candidate_id"] for row in accepted]
    require(len(accepted_ids) == len(set(accepted_ids)), "accepted IDs duplicate")
    require(set(accepted_ids) <= candidate_ids, "accepted IDs are outside candidate ledger")
    thresholds = {float(row["acceptance_threshold"]) for row in accepted}
    require(len(thresholds) == 1, "accepted threshold differs")
    threshold = next(iter(thresholds))
    require(
        all(math.isfinite(float(row["objective"])) and float(row["objective"]) <= threshold for row in accepted),
        "accepted objective exceeds threshold or is nonfinite",
    )
    finite = [
        float(row["objective"])
        for row in candidates
        if math.isfinite(float(row["objective"]))
    ]
    require(finite and abs(threshold - (min(finite) + 1.0)) <= 1.0e-10, "minimum-plus-one threshold differs")
    expected_accepted = [
        row["candidate_id"]
        for row in candidates
        if math.isfinite(float(row["objective"])) and float(row["objective"]) <= threshold
    ]
    require(accepted_ids == expected_accepted, "accepted ensemble is not the complete threshold set")

    trace_identity = validate_calibration_semantics(candidates, accepted, plan_by_id)

    primary_receipt = field_map(OBJECTS / "primary/reconstruction-receipt.csv")
    verification = field_map(OBJECTS / "verification/verification-receipt.csv")
    require(verification["state"] == "PASS", "independent reconstruction did not pass")
    require(primary_receipt["state"] == "PASS", "primary reconstruction did not pass")
    for command_id, receipt in (
        ("hubbard_primary_reconstruct", primary_receipt),
        ("hubbard_verify_reconstruct", verification),
    ):
        plan = plan_by_id[command_id]
        require(receipt["exact_command"] == plan["argv"], f"{command_id} argv differs")
        require(Path(receipt["source_path"]) == Path(plan["source_path"]), f"{command_id} source differs")
        require(sha256_file(Path(receipt["source_path"])) == receipt["source_sha256"], f"{command_id} source hash differs")
        require(sha256_file(Path(receipt["binary_path"])) == receipt["binary_sha256"], f"{command_id} binary hash differs")
        require(receipt["trace_sha256"] == trace_identity["trace_sha256"], f"{command_id} trace differs")
        require(
            receipt["observation_sha256"] == sha256_file(CAL04A / "phenology-forcing-join.csv"),
            f"{command_id} Hubbard observation projection differs",
        )
        require(
            receipt["objective_grouping"]
            == "equal_year_mean_of_all_admitted_record_squared_distances"
            and receipt["crossing_eligibility_yday"] == "60-180"
            and receipt["state_initialization"] == "FRESH_GSI_STATE_EACH_CANDIDATE_PLOT_YEAR",
            f"{command_id} reconstruction semantics differ",
        )
    for package_name, external_name in [
        ("candidate-ledger.csv", "candidate-ledger.csv"),
        ("accepted-calibration-ensemble.csv", "accepted-calibration-ensemble.csv"),
        ("failure-ledger.csv", "failure-ledger.csv"),
    ]:
        require(
            sha256_file(ARTIFACTS / package_name)
            == sha256_file(OBJECTS / "verification" / external_name),
            f"verification differs for {package_name}",
        )
    recovery = rows("synthetic-recovery-results.csv")
    require(
        {row["case_id"] for row in recovery}
        == {"SYN-GSI-01", "REC-BFBS-01", "REC-FE-01", "REC-LAI-01", "REC-CSBB-01"}
        and {row["status"] for row in recovery} == {"PASS"},
        "synthetic recovery inventory differs",
    )
    synthetic_receipts = [
        OBJECTS / "synthetic-primary/primary-reconstruction-receipt.csv",
        OBJECTS / "synthetic-verification/verification-reconstruction-receipt.csv",
    ]
    require(all(path.is_file() for path in synthetic_receipts), "dual synthetic receipt missing")
    primary_synthetic, verification_synthetic = map(field_map, synthetic_receipts)
    common = {
        "state",
        "case_id",
        "trace_sha256",
        "hidden_candidate",
        "hidden_objective",
        "recovered_set",
        "nonvacuous_competitor",
        "components_sha256",
        "annual_sha256",
        "candidate_ledger_sha256",
        "accepted_ensemble_sha256",
    }
    require(
        all(primary_synthetic.get(key) == verification_synthetic.get(key) for key in common),
        "synthetic receipts differ",
    )
    require(
        primary_synthetic.get("state") == "PASS"
        and primary_synthetic.get("case_id") == "SYN-GSI-01"
        and primary_synthetic.get("hidden_candidate") == "GSI-5557"
        and primary_synthetic.get("hidden_objective") == "0.000000000000"
        and primary_synthetic.get("nonvacuous_competitor") == "TRUE"
        and verification_synthetic.get("exact_primary_match") == "TRUE",
        "synthetic receipt acceptance differs",
    )
    require(
        primary_synthetic["trace_sha256"] == sha256_file(OBJECTS / "synthetic-gsi.bin"),
        "synthetic trace receipt differs",
    )
    for directory, receipt in (
        (OBJECTS / "synthetic-primary", primary_synthetic),
        (OBJECTS / "synthetic-verification", verification_synthetic),
    ):
        for field, name in (
            ("components_sha256", "candidate-observation-components.csv"),
            ("annual_sha256", "candidate-annual-components.csv"),
            ("candidate_ledger_sha256", "candidate-ledger.csv"),
            ("accepted_ensemble_sha256", "accepted-synthetic-ensemble.csv"),
        ):
            require(sha256_file(directory / name) == receipt[field], f"synthetic {field} differs")

    native = rows("native-consumer-proof.csv")
    require(len(native) == 12 and all(row["state"].startswith("PASS") for row in native), "native consumer proof differs")
    valid_native = [row for row in native if row["case_id"] != "invalid_threshold_order"]
    require(all(int(row["compared_days"]) > 0 and int(row["compared_values"]) == int(row["compared_days"]) * 8 for row in valid_native), "native proof comparison completeness differs")

    retention = rows("trace-retention.csv")
    require(len(retention) == 1 and retention[0]["state"] == "PASS", "trace retention receipt differs")
    compressed = Path(retention[0]["compressed_path"])
    require(
        retention[0]["schema"] == "CAL04B03"
        and retention[0]["value_count"] == str(TRACE_VALUE_COUNT)
        and retention[0]["raw_bytes"] == str(TRACE_BYTES)
        and retention[0]["decompressed_bytes"] == str(TRACE_BYTES)
        and Path(retention[0]["raw_path"]) == OBJECTS / "hubbard-gsi.bin"
        and compressed == OBJECTS / "hubbard-gsi.bin.zst",
        "trace retention cardinality/path differs",
    )
    require(sha256_file(compressed) == retention[0]["compressed_sha256"], "compressed trace digest differs")
    require(retention[0]["raw_sha256"] == trace_identity["trace_sha256"], "retention raw identity differs")
    require(retention[0]["decompressed_sha256"] == trace_identity["trace_sha256"], "retention expansion differs")
    require(zstd_expanded_sha(compressed) == trace_identity["trace_sha256"], "zstd expansion identity differs")
    require(retention[0]["exact_command"] == plan_by_id["retain_trace"]["argv"], "retention argv differs")
    require(retention[0]["source_sha256"] == sha256_file(PACKAGE / "tools/retain.py"), "retention source differs")

    membership = rows("later-stage-membership.csv")
    require(len(membership) == 4 and {row["state"] for row in membership} == {"PASS"}, "readiness index differs")
    multipliers = {
        "foliar_structural_partition": 9,
        "evergreen_fraction": 45,
        "peak_lai": 270,
        "canopy_floor_closure": 4_320,
    }
    for row in membership:
        expected = len(accepted) * multipliers[row["stage"]]
        require(int(row["membership_rows"]) == expected, f"readiness count differs for {row['stage']}")
        for prefix in ("membership", "parent_results"):
            path = Path(row[f"{prefix}_path"])
            require(path.is_file(), f"missing readiness {prefix} for {row['stage']}")
            require(sha256_file(path) == row[f"{prefix}_sha256"], f"readiness {prefix} hash differs")
            require(line_rows(path) == expected, f"readiness {prefix} rows differ")
        validate_stage_propagation(
            row,
            accepted_ids,
            sha256_file(ARTIFACTS / "later-stage-results.csv"),
        )
    later_results = rows("later-stage-results.csv")
    require(
        {row["design_id"] for row in later_results}
        >= {"EMP-BFBS-01", "REC-BFBS-01", "REC-FE-01", "EMP-LAI-01", "REC-LAI-01", "REC-CSBB-01"},
        "later-stage result designs differ",
    )
    require(
        len([row for row in later_results if row["evidence_role"] == "TYPED_FAILURE"]) >= 6,
        "later-stage typed failure evidence incomplete",
    )
    recovery_receipts = {row["design_id"]: row for row in rows("later-stage-recovery.csv")}
    expected_hidden = {
        "REC-BFBS-01": "Bf=0.20;Bs=0.10",
        "REC-FE-01": "fe=0.50",
        "REC-LAI-01": "xmxlai=6.00",
        "REC-CSBB-01": "Cs=0.20;bb=5.00",
    }
    require(set(recovery_receipts) == set(expected_hidden), "later-stage recovery receipt inventory differs")
    result_hash = sha256_file(ARTIFACTS / "later-stage-results.csv")
    for design_id, hidden in expected_hidden.items():
        result_rows = [
            row
            for row in later_results
            if row["design_id"] == design_id
            and row["evidence_role"] == "ASSUMED_FOR_EXECUTION"
            and row["failure"] == "NONE"
        ]
        require(result_rows, f"empty recovery result design {design_id}")
        minimum = min(float(row["objective"]) for row in result_rows)
        recovered_values = [
            row["operand_values"].replace(";", "_")
            for row in result_rows
            if abs(float(row["objective"]) - minimum) <= 1.0e-15
        ]
        receipt = recovery_receipts[design_id]
        expected_status = "RECOVERED_UNIQUE" if len(recovered_values) == 1 else "RECOVERED_EQUIFINAL"
        require(
            receipt["hidden_truth"] == hidden
            and receipt["recovered_set"] == "|".join(recovered_values)
            and receipt["recovery_status"] == expected_status
            and receipt["results_sha256"] == result_hash
            and hidden.replace(";", "_") in recovered_values,
            f"derived recovery receipt differs for {design_id}",
        )
    readiness_receipt = field_map(OBJECTS / "readiness/execution-receipt.csv")
    require(readiness_receipt["state"] == "PASS", "readiness execution receipt differs")
    require(
        readiness_receipt["exact_command"] == plan_by_id["readiness"]["argv"],
        "readiness argv differs from frozen plan",
    )
    require(
        Path(readiness_receipt["source_path"]) == Path(plan_by_id["readiness"]["source_path"])
        and sha256_file(Path(readiness_receipt["source_path"]))
        == readiness_receipt["source_sha256"]
        and sha256_file(Path(readiness_receipt["binary_path"]))
        == readiness_receipt["binary_sha256"],
        "readiness source/binary identity differs",
    )
    require(
        readiness_receipt["results_sha256"] == sha256_file(ARTIFACTS / "later-stage-results.csv")
        and readiness_receipt["membership_index_sha256"]
        == sha256_file(ARTIFACTS / "later-stage-membership.csv"),
        "readiness output receipt differs",
    )

    digest, transitive = validate_freeze(
        ARTIFACTS / "holdout-freeze-manifest.csv",
        ARTIFACTS / "holdout-freeze-digest.txt",
        OBJECTS / "freeze-bundles",
    )
    external_receipts = [
        custody_root / "freeze-receipts/verifier_a.csv",
        custody_root / "freeze-receipts/verifier_b.csv",
    ]
    receipt_rows = validate_receipt_barrier(
        external_receipts,
        digest,
        PACKAGE / "tools/freeze-verify.py",
        {
            verifier_id: verifier_command(
                verifier_id, execution_root, custody_root
            )
            for verifier_id in ("verifier_a", "verifier_b")
        },
    )
    require(
        read_csv_exact(ARTIFACTS / "freeze-verifier-receipts.csv", RECEIPT_FIELDS)
        == receipt_rows,
        "published freeze barrier differs",
    )
    require(transitive > 0, "freeze has no transitive members")

    token = custody_root / "holdout-opened-once.lock"
    require(token.is_file(), "holdout token missing")
    token_fields = dict(
        line.split("=", 1)
        for line in token.read_text(encoding="utf-8").splitlines()
        if "=" in line
    )
    require(
        token_fields.get("state") == "OPENED_ONCE"
        and token_fields.get("freeze_digest") == digest
        and token_fields.get("command")
        == holdout_opening_command(
            execution_root,
            custody_root,
            holdout_output_root,
            token,
        ),
        "holdout token does not bind the current freeze",
    )
    opening_text = (ARTIFACTS / "holdout-opening-record.md").read_text(encoding="utf-8")
    require(
        "State: `SCORED_NO_REFIT`" in opening_text
        and f"Freeze digest: `{digest}`" in opening_text,
        "holdout opening record is incomplete or binds another freeze",
    )
    receipt_rows = rows("holdout-execution-receipt.csv")
    require(len(receipt_rows) == 1, "holdout execution receipt differs")
    holdout_receipt = receipt_rows[0]
    holdout_root = holdout_output_root / "objects"
    holdout_paths = {
        "token_sha256": token,
        "expected_input_manifest_sha256": ARTIFACTS / "harvard-expected-input-manifest.csv",
        "accepted_ensemble_sha256": ARTIFACTS / "accepted-calibration-ensemble.csv",
        "trace_sha256": holdout_root / "harvard-gsi.bin",
        "trace_identity_sha256": holdout_root / "harvard-gsi-identity.csv",
        "calendar_sha256": holdout_root / "harvard-gsi.calendar.csv",
        "observation_components_sha256": holdout_root / "harvard-observation-components.csv",
        "annual_components_sha256": holdout_root / "harvard-annual-components.csv",
        "results_sha256": ARTIFACTS / "harvard-holdout-results.csv",
        "holdout_script_sha256": PACKAGE / "tools/holdout.py",
    }
    require(
        holdout_receipt["state"] == "PASS_SCORED_NO_REFIT"
        and holdout_receipt["freeze_digest"] == digest,
        "holdout execution receipt state/digest differs",
    )
    for field, path in holdout_paths.items():
        require(holdout_receipt[field] == sha256_file(path), f"holdout receipt differs for {field}")
        require(f"`{holdout_receipt[field]}`" in opening_text or field in {"token_sha256", "expected_input_manifest_sha256", "accepted_ensemble_sha256", "calendar_sha256", "holdout_script_sha256"}, f"opening record lacks {field}")
    holdout_identity = field_map(holdout_root / "harvard-gsi-identity.csv")
    require(
        holdout_identity["schema"] == "CAL04B02"
        and holdout_identity["site_id"] == "harvard"
        and holdout_identity["candidate_count"] == str(len(accepted))
        and holdout_identity["retained_day_count"] == "12053"
        and holdout_identity["trace_sha256"] == holdout_receipt["trace_sha256"]
        and holdout_identity["calendar_sha256"] == holdout_receipt["calendar_sha256"]
        and holdout_identity["accepted_sha256"] == holdout_receipt["accepted_ensemble_sha256"],
        "holdout trace identity differs",
    )
    require(
        holdout_receipt["producer_command"] == holdout_identity["exact_command"],
        "holdout producer command differs from trace identity",
    )
    expected_producer_command = " ".join(
        [
            str(EXECUTION_ROOT / "cargo-target/release/holdout-producer"),
            "--configs",
            str(ARTIFACTS / "candidate-configurations.csv"),
            "--accepted",
            str(ARTIFACTS / "accepted-calibration-ensemble.csv"),
            "--climate",
            str(ROOT / "tests/fixtures/cancov_forest/harvard_deciduous_ma/p6.cli"),
            "--trace",
            str(holdout_root / "harvard-gsi.bin"),
            "--identity",
            str(holdout_root / "harvard-gsi-identity.csv"),
        ]
    )
    expected_reconstructor_command = " ".join(
        [
            str(EXECUTION_ROOT / "cargo-target/release/holdout-reconstruct"),
            "--trace",
            str(holdout_root / "harvard-gsi.bin"),
            "--calendar",
            str(holdout_root / "harvard-gsi.calendar.csv"),
            "--identity",
            str(holdout_root / "harvard-gsi-identity.csv"),
            "--accepted",
            str(ARTIFACTS / "accepted-calibration-ensemble.csv"),
            "--observations",
            str(
                ROOT
                / "docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/artifacts/cal04-timing-windows.csv"
            ),
            "--observation-out",
            str(holdout_root / "harvard-observation-components.csv"),
            "--annual-out",
            str(holdout_root / "harvard-annual-components.csv"),
            "--result-out",
            str(ARTIFACTS / "harvard-holdout-results.csv"),
        ]
    )
    require(
        holdout_receipt["producer_command"] == expected_producer_command
        and holdout_receipt["reconstructor_command"] == expected_reconstructor_command,
        "holdout nested command argv differs",
    )
    holdout = rows("harvard-holdout-results.csv")
    require(len(holdout) == len(accepted), "holdout membership count differs")
    require([row["candidate_id"] for row in holdout] == accepted_ids, "holdout candidate order differs")
    require(
        {row["state"] for row in holdout}
        <= {"SCORED_NO_REFIT", "RETAINED_VALIDATION_FAILURE"},
        "holdout result state differs",
    )
    validate_holdout_arithmetic(accepted_ids)

    statuses = rows("stage-status-ledger.csv")
    require(
        len(statuses) == 5
        and {row["state"] for row in statuses} == {"PASS"}
        and {row["science_implementation_status"] for row in statuses} == {"IMPLEMENTED"},
        "stage status ledger differs",
    )
    matrix = (ARTIFACTS / "calibration-readiness-matrix.md").read_text(encoding="utf-8")
    require("Status: `PASS`" in matrix and "`BLOCKED`" not in matrix, "readiness matrix differs")
    require(len(rows("additional-data-inventory.csv")) == 5, "additional-data inventory differs")

    print(
        f"PASS terminal candidates=9261 accepted={len(accepted)} "
        f"holdout={len(holdout)} transitive_freeze_members={transitive}"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
