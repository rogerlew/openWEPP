#!/usr/bin/env python3
"""Execute and summarize the frozen CAL-06 canopy-gradient matrix."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import shutil
import statistics
import subprocess
import tempfile
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from datetime import date, timedelta
from pathlib import Path
from typing import Any, Iterable

import pyarrow.parquet as pq

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
CAL04B = (
    ROOT
    / "docs/work-packages/"
    "20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts"
)
FIXTURE_ROOT = ROOT / "tests/fixtures/cancov_forest"
OBSERVATION_FILES = (
    FIXTURE_ROOT / "observations/sites/harvard_hf237_strata.csv",
    FIXTURE_ROOT / "observations/sites/marcell_rds_2021_0016_stratum_means.csv",
)
GSI_KEYS = (
    "minimum_temperature_inactive_c",
    "minimum_temperature_unconstrained_c",
    "vapor_pressure_deficit_unconstrained_pa",
    "vapor_pressure_deficit_inactive_pa",
    "photoperiod_inactive_hours",
    "photoperiod_unconstrained_hours",
)
CLIMATOLOGY_FIELDS = (
    "gsi21",
    "canopy_cover_fraction",
    "lai_m2_m2",
    "leaf_litter_kg_m2",
    "surface_residue_kg_m2",
    "residue_depth_m",
    "interception_mm",
    "et_mm",
    "runoff_mm",
    "swe_mm",
    "snow_depth_mm",
    "snow_density_kg_m3",
    "frost_depth_mm",
)


@dataclass(frozen=True)
class Lane:
    site: str
    stratum: str
    fixture: str
    run_file: str
    forest: bool


LANES = (
    Lane("marcell", "conifer", "marcell_conifer_mn", "p8.native.run.toml", True),
    Lane("marcell", "deciduous", "marcell_deciduous_mn", "p15.native.run.toml", True),
    Lane("marcell", "mixed", "marcell_mixed_mn", "p10.native.run.toml", True),
    Lane("marcell", "open", "marcell_open_mn", "p6.native.run.toml", False),
    Lane("harvard", "deciduous", "harvard_deciduous_ma", "p6.native.run.toml", True),
    Lane("harvard", "mixed", "harvard_mixed_ma", "p8.native.run.toml", True),
    Lane("harvard", "open", "harvard_open_ma", "p3.native.run.toml", False),
    Lane(
        "hubbard_brook",
        "deciduous",
        "hubbardbrook_deciduous_nh",
        "p10.native.run.toml",
        True,
    ),
    Lane(
        "hubbard_brook",
        "mixed",
        "hubbardbrook_mixed_nh",
        "p4.native.run.toml",
        True,
    ),
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def finite(value: Any, name: str) -> float:
    result = float(value)
    if not math.isfinite(result):
        raise ValueError(f"{name} must be finite")
    return result


def accepted_members() -> list[dict[str, str]]:
    with (CAL04B / "accepted-calibration-ensemble.csv").open(
        newline="", encoding="utf-8"
    ) as stream:
        accepted = list(csv.DictReader(stream))
    with (CAL04B / "candidate-configurations.csv").open(
        newline="", encoding="utf-8"
    ) as stream:
        configurations = {row["candidate_id"]: row for row in csv.DictReader(stream)}
    if len(accepted) != 37:
        raise ValueError(f"expected 37 accepted members, found {len(accepted)}")
    members = []
    for row in accepted:
        candidate = configurations.get(row["candidate_id"])
        if candidate is None:
            raise ValueError(f"missing configuration for {row['candidate_id']}")
        members.append({**row, **candidate})
    return members


def replace_yaml(path: Path, member: dict[str, str]) -> None:
    output: list[str] = []
    seen: set[str] = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        key = stripped.split(":", 1)[0]
        if key in GSI_KEYS:
            if key in seen:
                raise ValueError(f"{path}: duplicate {key}")
            indent = line[: len(line) - len(line.lstrip())]
            output.append(f"{indent}{key}: {finite(member[key], key):.12g}")
            seen.add(key)
        else:
            output.append(line)
    if seen != set(GSI_KEYS):
        raise ValueError(f"{path}: missing GSI keys {sorted(set(GSI_KEYS) - seen)}")
    path.write_text("\n".join(output) + "\n", encoding="utf-8")


def find_output(root: Path, suffix: str) -> Path:
    matches = list(root.rglob(f"*{suffix}"))
    if len(matches) != 1:
        raise ValueError(f"{root}: expected one *{suffix}, found {len(matches)}")
    return matches[0]


def read_trace(path: Path) -> tuple[list[dict[str, Any]], dict[int, dict[str, float]]]:
    rows: list[dict[str, Any]] = []
    by_day: dict[int, dict[str, float]] = {}
    with path.open(encoding="utf-8") as stream:
        for line_number, line in enumerate(stream, 1):
            row = json.loads(line)
            if row.get("schema") != "openwepp-canopy-research-daily-v1":
                raise ValueError(f"{path}:{line_number}: wrong schema")
            day_index = int(row["day_index"])
            if day_index in by_day:
                raise ValueError(f"{path}:{line_number}: duplicate day index")
            residue = row["residue"]
            if residue["needle_litter_input_kg_m2"] is not None:
                raise ValueError("predictive needle source must remain null")
            if residue["fine_woody_litter_input_kg_m2"] is not None:
                raise ValueError("predictive fine-woody source must remain null")
            canopy = row["canopy"]
            consumers = row["consumers"]
            compact = {
                "day_index": day_index,
                "year": int(row["year"]),
                "day_of_year": int(row["day_of_year"]),
                "date": str(row["date"]),
                "gsi21": finite(row["gsi"]["gsi21"], "gsi21"),
                "canopy_cover_fraction": finite(
                    canopy["cover_fraction"], "canopy cover"
                ),
                "lai_m2_m2": finite(canopy["leaf_area_index_m2_m2"], "LAI"),
                "leaf_litter_kg_m2": finite(
                    residue["leaf_litter_input_kg_m2"], "leaf litter"
                ),
                "surface_residue_kg_m2": finite(
                    residue["surface_residue_after_kg_m2"], "surface residue"
                ),
                "residue_depth_m": finite(residue["residue_depth_m"], "residue depth"),
                "interception_mm": 1000.0
                * finite(consumers["interception_m"], "interception"),
            }
            by_day[day_index] = compact
            rows.append(compact)
    if len(rows) != 16_437:
        raise ValueError(f"{path}: expected 16437 rows, found {len(rows)}")
    return rows, by_day


def climate_calendar(path: Path) -> list[date]:
    calendar: list[date] = []
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        fields = line.split()
        if len(fields) != 13:
            continue
        try:
            day_value, month_value, year_value = map(int, fields[:3])
        except ValueError:
            continue
        if not 1900 <= year_value <= 2200:
            continue
        current = date(year_value, month_value, day_value)
        if calendar and current != calendar[-1] + timedelta(days=1):
            raise ValueError(f"{path}:{line_number}: nonconsecutive climate calendar")
        calendar.append(current)
    if not calendar:
        raise ValueError(f"{path}: no daily climate calendar")
    return calendar


def wat_rows(path: Path, calendar: list[date]) -> list[dict[str, Any]]:
    columns = [
        "year",
        "sim_day_index",
        "julian",
        "month",
        "day_of_month",
        "water_year",
        "Q",
        "Ep",
        "Es",
        "Er",
        "frdp",
        "Snow-Water",
        "Snow-Depth",
        "Interception",
    ]
    table = pq.read_table(path, columns=columns)
    data = table.to_pydict()
    count = table.num_rows
    if count != 16_437:
        raise ValueError(f"{path}: expected 16437 WAT rows, found {count}")
    if len(calendar) != count:
        raise ValueError(f"{path}: WAT/climate length mismatch {count} != {len(calendar)}")
    rows: list[dict[str, Any]] = []
    for index in range(count):
        current = calendar[index]
        sim_day_index = int(data["sim_day_index"][index])
        if sim_day_index != index + 1:
            raise ValueError(f"{path}: nonconsecutive simulation day index")
        if int(data["julian"][index]) != current.timetuple().tm_yday:
            raise ValueError(f"{path}: WAT/climate Julian mismatch at {index}")
        depth = data["Snow-Depth"][index]
        swe = finite(data["Snow-Water"][index], "SWE")
        density = None
        if depth is not None and float(depth) > 0.0 and swe > 0.0:
            density = swe / float(depth) * 1000.0
        rows.append(
            {
                "day_index": index,
                "date": current.isoformat(),
                "year": current.year,
                "day_of_year": int(data["julian"][index]),
                "water_year": current.year + (1 if current.month >= 10 else 0),
                "et_mm": sum(
                    finite(data[field][index], field) for field in ("Ep", "Es", "Er")
                ),
                "runoff_mm": finite(data["Q"][index], "Q"),
                "swe_mm": swe,
                "snow_depth_mm": None if depth is None else finite(depth, "Snow-Depth"),
                "snow_density_kg_m3": density,
                "frost_depth_mm": finite(data["frdp"][index], "frdp"),
                "wat_interception_mm": (
                    None
                    if data["Interception"][index] is None
                    else finite(data["Interception"][index], "Interception")
                ),
            }
        )
    return rows


def median_or_blank(values: Iterable[float | None]) -> float | str:
    finite_values = [float(value) for value in values if value is not None]
    return statistics.median(finite_values) if finite_values else ""


def mean_or_blank(values: Iterable[float | None]) -> float | str:
    finite_values = [float(value) for value in values if value is not None]
    return statistics.fmean(finite_values) if finite_values else ""


def seasonal_summary(
    lane: Lane,
    member_id: str,
    trace: list[dict[str, Any]] | None,
    wat: list[dict[str, Any]],
) -> tuple[dict[str, Any], list[dict[str, Any]], list[dict[str, Any]]]:
    combined: list[dict[str, Any]] = []
    trace_index = {} if trace is None else {row["day_index"]: row for row in trace}
    by_doy: dict[int, dict[str, list[float]]] = defaultdict(
        lambda: defaultdict(list)
    )
    years: dict[int, list[dict[str, Any]]] = defaultdict(list)
    water_years: dict[int, list[dict[str, Any]]] = defaultdict(list)
    for row in wat:
        merged = dict(row)
        canopy = trace_index.get(row["day_index"])
        if canopy is not None:
            if canopy["date"] != row["date"]:
                raise ValueError(f"{lane.fixture}/{member_id}: trace/WAT date mismatch")
            merged.update(canopy)
            if (
                row["wat_interception_mm"] is not None
                and abs(row["wat_interception_mm"] - canopy["interception_mm"]) > 1.0e-9
            ):
                raise ValueError(
                    f"{lane.fixture}/{member_id}: interception trace/WAT mismatch"
                )
        combined.append(merged)
        years[row["year"]].append(merged)
        water_years[row["water_year"]].append(merged)
        for field in CLIMATOLOGY_FIELDS:
            value = merged.get(field)
            if value is not None:
                by_doy[row["day_of_year"]][field].append(float(value))

    climatology: list[dict[str, Any]] = []
    for doy in sorted(by_doy):
        output: dict[str, Any] = {
            "site": lane.site,
            "stratum": lane.stratum,
            "fixture": lane.fixture,
            "member_id": member_id,
            "day_of_year": doy,
        }
        for field in CLIMATOLOGY_FIELDS:
            output[field] = mean_or_blank(by_doy[doy].get(field, []))
        climatology.append(output)

    winter = [
        row
        for row in combined
        if row["day_of_year"] >= 335 or row["day_of_year"] <= 79
    ]
    summer = [row for row in combined if 152 <= row["day_of_year"] <= 243]
    period_operands: list[dict[str, Any]] = []
    base = {
        "site": lane.site,
        "stratum": lane.stratum,
        "fixture": lane.fixture,
        "member_id": member_id,
    }
    all_operand: dict[str, Any] = {
        **base,
        "period_type": "ALL",
        "period_id": "ALL",
        "day_count": len(combined),
        "winter_cover_sum": (
            sum(float(row["canopy_cover_fraction"]) for row in winter)
            if trace is not None
            else ""
        ),
        "winter_cover_count": len(winter) if trace is not None else "",
        "summer_cover_max": (
            max(float(row["canopy_cover_fraction"]) for row in summer)
            if trace is not None
            else ""
        ),
        "summer_lai_max": (
            max(float(row["lai_m2_m2"]) for row in summer)
            if trace is not None
            else ""
        ),
        "cover_min": (
            min(float(row["canopy_cover_fraction"]) for row in combined)
            if trace is not None
            else ""
        ),
        "cover_max": (
            max(float(row["canopy_cover_fraction"]) for row in combined)
            if trace is not None
            else ""
        ),
        "annual_leaf_litter_kg_m2": "",
        "annual_interception_mm": "",
        "annual_et_mm": "",
        "annual_runoff_mm": "",
        "peak_swe_mm": "",
        "peak_snow_depth_mm": "",
        "peak_snow_density_kg_m3": "",
        "meltout_day_of_year": "",
        "frost_onset_day_of_year": "",
        "frost_thaw_day_of_year": "",
    }
    period_operands.append(all_operand)
    for year, year_rows in sorted(years.items()):
        period_operands.append(
            {
                **base,
                "period_type": "CALENDAR_YEAR",
                "period_id": year,
                "day_count": len(year_rows),
                "winter_cover_sum": "",
                "winter_cover_count": "",
                "summer_cover_max": "",
                "summer_lai_max": "",
                "cover_min": "",
                "cover_max": "",
                "annual_leaf_litter_kg_m2": (
                    sum(float(row["leaf_litter_kg_m2"]) for row in year_rows)
                    if trace is not None
                    else ""
                ),
                "annual_interception_mm": sum(
                    float(row.get("interception_mm", row["wat_interception_mm"] or 0.0))
                    for row in year_rows
                ),
                "annual_et_mm": sum(float(row["et_mm"]) for row in year_rows),
                "annual_runoff_mm": sum(float(row["runoff_mm"]) for row in year_rows),
                "peak_swe_mm": "",
                "peak_snow_depth_mm": "",
                "peak_snow_density_kg_m3": "",
                "meltout_day_of_year": "",
                "frost_onset_day_of_year": "",
                "frost_thaw_day_of_year": "",
            }
        )
    annual_interception = [
        float(row["annual_interception_mm"])
        for row in period_operands
        if row["period_type"] == "CALENDAR_YEAR"
    ]
    annual_et = [
        float(row["annual_et_mm"])
        for row in period_operands
        if row["period_type"] == "CALENDAR_YEAR"
    ]
    annual_runoff = [
        float(row["annual_runoff_mm"])
        for row in period_operands
        if row["period_type"] == "CALENDAR_YEAR"
    ]
    annual_litter = [
        float(row["annual_leaf_litter_kg_m2"])
        for row in period_operands
        if row["period_type"] == "CALENDAR_YEAR"
        and row["annual_leaf_litter_kg_m2"] != ""
    ]
    peak_swe: list[float] = []
    peak_depth: list[float] = []
    peak_density: list[float] = []
    meltout_doy: list[float] = []
    frost_onset: list[float] = []
    frost_thaw: list[float] = []
    for water_year, water_year_rows in sorted(water_years.items()):
        peak = max(water_year_rows, key=lambda row: float(row["swe_mm"]))
        peak_swe.append(float(peak["swe_mm"]))
        if peak["snow_depth_mm"] is not None:
            peak_depth.append(float(peak["snow_depth_mm"]))
        if peak["snow_density_kg_m3"] is not None:
            peak_density.append(float(peak["snow_density_kg_m3"]))
        peak_index = water_year_rows.index(peak)
        meltout: float | str = ""
        for offset in range(
            peak_index + 1, max(peak_index + 1, len(water_year_rows) - 6)
        ):
            window = water_year_rows[offset : offset + 7]
            if len(window) == 7 and all(float(item["swe_mm"]) <= 1.0e-9 for item in window):
                meltout = float(window[0]["day_of_year"])
                meltout_doy.append(meltout)
                break
        frozen = [
            row
            for row in water_year_rows
            if float(row["frost_depth_mm"]) > 1.0e-9
        ]
        onset: float | str = ""
        thaw: float | str = ""
        if frozen:
            onset = float(frozen[0]["day_of_year"])
            thaw = float(frozen[-1]["day_of_year"])
            frost_onset.append(onset)
            frost_thaw.append(thaw)
        period_operands.append(
            {
                **base,
                "period_type": "WATER_YEAR",
                "period_id": water_year,
                "day_count": len(water_year_rows),
                "winter_cover_sum": "",
                "winter_cover_count": "",
                "summer_cover_max": "",
                "summer_lai_max": "",
                "cover_min": "",
                "cover_max": "",
                "annual_leaf_litter_kg_m2": "",
                "annual_interception_mm": "",
                "annual_et_mm": "",
                "annual_runoff_mm": "",
                "peak_swe_mm": float(peak["swe_mm"]),
                "peak_snow_depth_mm": (
                    "" if peak["snow_depth_mm"] is None else float(peak["snow_depth_mm"])
                ),
                "peak_snow_density_kg_m3": (
                    ""
                    if peak["snow_density_kg_m3"] is None
                    else float(peak["snow_density_kg_m3"])
                ),
                "meltout_day_of_year": meltout,
                "frost_onset_day_of_year": onset,
                "frost_thaw_day_of_year": thaw,
            }
        )

    result: dict[str, Any] = {
        "site": lane.site,
        "stratum": lane.stratum,
        "fixture": lane.fixture,
        "member_id": member_id,
        "run_state": "PASS",
        "day_count": len(combined),
        "winter_cover_mean": (
            mean_or_blank(row.get("canopy_cover_fraction") for row in winter)
        ),
        "summer_cover_max": (
            max(float(row["canopy_cover_fraction"]) for row in summer)
            if trace is not None
            else ""
        ),
        "summer_lai_max": (
            max(float(row["lai_m2_m2"]) for row in summer) if trace is not None else ""
        ),
        "cover_amplitude": (
            max(float(row["canopy_cover_fraction"]) for row in combined)
            - min(float(row["canopy_cover_fraction"]) for row in combined)
            if trace is not None
            else ""
        ),
        "annual_leaf_litter_median_kg_m2": median_or_blank(annual_litter),
        "annual_interception_median_mm": median_or_blank(annual_interception),
        "annual_et_median_mm": median_or_blank(annual_et),
        "annual_runoff_median_mm": median_or_blank(annual_runoff),
        "peak_swe_median_mm": median_or_blank(peak_swe),
        "peak_snow_depth_median_mm": median_or_blank(peak_depth),
        "peak_snow_density_median_kg_m3": median_or_blank(peak_density),
        "meltout_median_day_of_year": median_or_blank(meltout_doy),
        "frost_onset_median_day_of_year": median_or_blank(frost_onset),
        "frost_thaw_median_day_of_year": median_or_blank(frost_thaw),
        "predictive_needle_source": "NULL_AUTHORITY_MISSING" if lane.forest else "NOT_APPLICABLE",
        "predictive_fine_woody_source": (
            "NULL_AUTHORITY_MISSING" if lane.forest else "NOT_APPLICABLE"
        ),
        "erosion_output": "NULL_NOT_EMITTED",
    }
    return result, climatology, period_operands


def load_observations() -> dict[str, list[dict[str, str]]]:
    by_fixture: dict[str, list[dict[str, str]]] = defaultdict(list)
    for path in OBSERVATION_FILES:
        with path.open(newline="", encoding="utf-8") as stream:
            for row in csv.DictReader(stream):
                fixture = row.get("model_fixture", "")
                if row.get("binding_status") == "bound" and fixture:
                    by_fixture[fixture].append(row)
    return by_fixture


def observation_scores(
    lane: Lane,
    member_id: str,
    wat: list[dict[str, Any]],
    observations: dict[str, list[dict[str, str]]],
) -> list[dict[str, Any]]:
    model = {row["date"]: row for row in wat}
    pairs: dict[str, list[tuple[float, float]]] = defaultdict(list)
    source_fields = {
        "snow_depth": ("observed_snow_depth_m", "snow_depth_mm", 1000.0),
        "swe": ("observed_swe_mm", "swe_mm", 1.0),
        "density": ("observed_density_kg_m3", "snow_density_kg_m3", 1.0),
    }
    harvard_swe_contradiction = lane.fixture in {
        "harvard_deciduous_ma",
        "harvard_open_ma",
    }
    for observation in observations.get(lane.fixture, []):
        modeled = model.get(observation["date"])
        if modeled is None:
            continue
        for quantity, (observed_field, model_field, scale) in source_fields.items():
            raw_observed = observation.get(observed_field, "")
            raw_model = modeled.get(model_field)
            if raw_observed in ("", None) or raw_model is None:
                continue
            pairs[quantity].append((float(raw_model) / scale, float(raw_observed)))
    outputs: list[dict[str, Any]] = []
    units = {"snow_depth": "m", "swe": "mm", "density": "kg_m3"}
    for quantity in source_fields:
        if quantity == "swe" and harvard_swe_contradiction:
            outputs.append(
                {
                    "site": lane.site,
                    "stratum": lane.stratum,
                    "fixture": lane.fixture,
                    "member_id": member_id,
                    "quantity": quantity,
                    "units": units[quantity],
                    "matched_count": 0,
                    "bias": "",
                    "mae": "",
                    "rmse": "",
                    "verdict": "INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION",
                }
            )
            continue
        values = pairs.get(quantity, [])
        residuals = [model_value - observed for model_value, observed in values]
        outputs.append(
            {
                "site": lane.site,
                "stratum": lane.stratum,
                "fixture": lane.fixture,
                "member_id": member_id,
                "quantity": quantity,
                "units": units[quantity],
                "matched_count": len(values),
                "bias": mean_or_blank(residuals),
                "mae": mean_or_blank(abs(value) for value in residuals),
                "rmse": (
                    math.sqrt(statistics.fmean(value * value for value in residuals))
                    if residuals
                    else ""
                ),
                "verdict": "BOUNDED_NO_SOURCE_UNCERTAINTY" if values else "NOT_EVALUATED",
            }
        )
    return outputs


def execute_one(
    lane: Lane,
    member: dict[str, str] | None,
    runner: Path,
    scratch_root: Path,
    observations: dict[str, list[dict[str, str]]],
) -> tuple[
    dict[str, Any],
    list[dict[str, Any]],
    list[dict[str, Any]],
    list[dict[str, Any]],
    dict[str, Any],
]:
    member_id = member["candidate_id"] if member is not None else "OPEN-CONTROL"
    run_id = f"{lane.fixture}__{member_id}"
    work = Path(tempfile.mkdtemp(prefix=f"{run_id}-", dir=scratch_root))
    try:
        shutil.copytree(FIXTURE_ROOT / lane.fixture, work / "fixture", dirs_exist_ok=True)
        fixture = work / "fixture"
        if member is not None:
            management = next(fixture.glob("*.man.yaml"))
            replace_yaml(management, member)
        trace_path = work / "canopy.jsonl"
        environment = None
        if lane.forest:
            environment = {
                "OPENWEPP_CANOPY_RESEARCH_TRACE_PATH": str(trace_path),
                "OPENWEPP_CANOPY_RESEARCH_SITE_ID": lane.site,
                "OPENWEPP_CANOPY_RESEARCH_ARM_ID": lane.stratum,
            }
        command = [
            str(runner),
            "--run-dir",
            str(fixture),
            "--run-file",
            lane.run_file,
            "--output-dir",
            str(work / "output"),
            "--legacy-sidecar-discovery",
            "--direct-production-executor",
        ]
        import os

        run_environment = os.environ.copy()
        if environment:
            run_environment.update(environment)
        completed = subprocess.run(
            command,
            cwd=ROOT,
            env=run_environment,
            capture_output=True,
            text=True,
            check=False,
        )
        if completed.returncode != 0:
            raise RuntimeError(
                f"{run_id}: runner exit {completed.returncode}: {completed.stderr[-2000:]}"
            )
        wat_path = find_output(work, ".wat.parquet")
        trace_rows = None
        trace_sha = ""
        trace_bytes = 0
        if lane.forest:
            trace_sha = sha256(trace_path)
            trace_bytes = trace_path.stat().st_size
            trace_rows, _ = read_trace(trace_path)
        climate_path = next(fixture.glob("*.cli"))
        wat = wat_rows(wat_path, climate_calendar(climate_path))
        result, climatology, period_operands = seasonal_summary(
            lane, member_id, trace_rows, wat
        )
        scores = observation_scores(lane, member_id, wat, observations)
        manifest = {
            "run_id": run_id,
            "site": lane.site,
            "stratum": lane.stratum,
            "fixture": lane.fixture,
            "member_id": member_id,
            "state": "PASS",
            "trace_sha256": trace_sha,
            "trace_bytes": trace_bytes,
            "wat_sha256": sha256(wat_path),
            "wat_bytes": wat_path.stat().st_size,
            "stderr_sha256": "EPHEMERAL_PATH_BEARING_NOT_RETAINED",
            "command": (
                f"{runner} --run-dir <scratch>/{run_id}/fixture "
                f"--run-file {lane.run_file} --output-dir <scratch>/{run_id}/output "
                "--legacy-sidecar-discovery --direct-production-executor"
            ),
        }
        return result, climatology, period_operands, scores, manifest
    finally:
        shutil.rmtree(work, ignore_errors=True)


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        raise ValueError(f"refusing to write empty {path}")
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--runner", type=Path, default=ROOT / "target/release/openwepp-cli-hill"
    )
    parser.add_argument("--jobs", type=int, default=8)
    parser.add_argument("--scratch", type=Path)
    args = parser.parse_args()
    runner = args.runner.resolve()
    if not runner.is_file():
        raise SystemExit(f"runner not found: {runner}")
    if args.jobs < 1 or args.jobs > 16:
        raise SystemExit("--jobs must be in [1,16]")
    members = accepted_members()
    observations = load_observations()
    tasks: list[tuple[Lane, dict[str, str] | None]] = []
    for lane in LANES:
        if lane.forest:
            tasks.extend((lane, member) for member in members)
        else:
            tasks.append((lane, None))
    if len(tasks) != 261:
        raise SystemExit(f"expected 261 runs, planned {len(tasks)}")
    scratch_owner = None
    if args.scratch is None:
        scratch_owner = tempfile.TemporaryDirectory(prefix="openwepp-cal06-")
        scratch = Path(scratch_owner.name)
    else:
        scratch = args.scratch.resolve()
        scratch.mkdir(parents=True, exist_ok=True)
    results: list[dict[str, Any]] = []
    climatology: list[dict[str, Any]] = []
    period_operands: list[dict[str, Any]] = []
    scores: list[dict[str, Any]] = []
    manifests: list[dict[str, Any]] = []
    failures: list[str] = []
    with ThreadPoolExecutor(max_workers=args.jobs) as executor:
        pending = {
            executor.submit(
                execute_one, lane, member, runner, scratch, observations
            ): (lane, member)
            for lane, member in tasks
        }
        completed_count = 0
        for future in as_completed(pending):
            lane, member = pending[future]
            member_id = "OPEN-CONTROL" if member is None else member["candidate_id"]
            try:
                result, daily, operands, observed, manifest = future.result()
                results.append(result)
                climatology.extend(daily)
                period_operands.extend(operands)
                scores.extend(observed)
                manifests.append(manifest)
            except Exception as error:
                failures.append(f"{lane.fixture}/{member_id}: {error}")
            completed_count += 1
            if completed_count % 10 == 0 or completed_count == len(tasks):
                print(
                    f"completed {completed_count}/{len(tasks)}; failures={len(failures)}",
                    flush=True,
                )
    if scratch_owner is not None:
        scratch_owner.cleanup()
    if failures:
        (ARTIFACTS / "execution-failures.txt").write_text(
            "\n".join(failures) + "\n", encoding="utf-8"
        )
        raise SystemExit(f"{len(failures)} CAL-06 runs failed")
    results.sort(key=lambda row: (row["site"], row["stratum"], row["member_id"]))
    climatology.sort(
        key=lambda row: (
            row["site"],
            row["stratum"],
            row["member_id"],
            int(row["day_of_year"]),
        )
    )
    period_operands.sort(
        key=lambda row: (
            row["site"],
            row["stratum"],
            row["member_id"],
            row["period_type"],
            str(row["period_id"]),
        )
    )
    scores.sort(
        key=lambda row: (
            row["site"],
            row["stratum"],
            row["member_id"],
            row["quantity"],
        )
    )
    manifests.sort(key=lambda row: row["run_id"])
    write_csv(ARTIFACTS / "run-results.csv", results)
    write_csv(ARTIFACTS / "daily-climatology.csv", climatology)
    write_csv(ARTIFACTS / "run-period-operands.csv", period_operands)
    write_csv(ARTIFACTS / "observation-scores.csv", scores)
    write_csv(ARTIFACTS / "execution-manifest.csv", manifests)
    output_files = (
        "run-results.csv",
        "daily-climatology.csv",
        "run-period-operands.csv",
        "observation-scores.csv",
        "execution-manifest.csv",
    )
    manifest = {
        "schema": "openwepp-canopy-cal06-result-manifest-v1",
        "source_head": subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
        ).strip(),
        "runner": str(runner),
        "runner_sha256": sha256(runner),
        "planned_runs": 261,
        "passed_runs": len(results),
        "forest_member_runs": sum(row["member_id"] != "OPEN-CONTROL" for row in results),
        "open_control_runs": sum(row["member_id"] == "OPEN-CONTROL" for row in results),
        "accepted_member_count": len(members),
        "accepted_ensemble_sha256": sha256(
            CAL04B / "accepted-calibration-ensemble.csv"
        ),
        "observation_operator": {
            "harvard_bulk_density": (
                "WAT aggregate density versus HF237-01 daily bulk density"
            ),
            "harvard_profile_density": "NOT_EVALUATED_SCALE_MISMATCH",
            "harvard_swe": "INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION",
        },
        "outputs": {
            name: {"sha256": sha256(ARTIFACTS / name), "bytes": (ARTIFACTS / name).stat().st_size}
            for name in output_files
        },
        "raw_object_retention": (
            "ephemeral execution objects are digest-bound in execution-manifest.csv; "
            "canonical retained result objects are the compact tidy outputs"
        ),
    }
    (ARTIFACTS / "result-manifest.json").write_text(
        json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(
        f"PASS: {len(results)} runs; {len(climatology)} climatology rows; "
        f"{len(scores)} observation-score rows"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
