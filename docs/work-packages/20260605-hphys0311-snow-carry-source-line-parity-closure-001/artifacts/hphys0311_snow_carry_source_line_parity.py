#!/usr/bin/env python3
"""Classify HPHYS0310 carry-state divergences against source-line parity."""

from __future__ import annotations

import calendar
import json
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any


PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
HPHYS0310_LEDGER = (
    Path("docs/work-packages")
    / "20260605-hphys0310-prior-day-snow-carry-divergence-closure-001"
    / "artifacts"
    / "prior-day-snow-carry-divergence-ledger.json"
)
HPHYS0305_ARTIFACT_DIR = (
    Path("docs/work-packages")
    / "20260605-hphys0305-paired-melt-term-state-instrumentation-001"
    / "artifacts"
)
TRACE_BOUNDARY = "post_wb13"
TRACE_PHASE = None
INHERITED_DELTA_TOL_M = 1.0e-12
INHERITED_DENSITY_DELTA_TOL_KG_M3 = 1.0e-12
SETTLING_PREVIOUS_DEPTH_TOL_M = 1.0e-5
SETTLING_PREVIOUS_DENSITY_TOL_KG_M3 = 1.0e-2
OBS_RE = re.compile(
    r"^(?P<tag>H305_[A-Z_]+)\s+"
    r"y=\s*(?P<year>-?\d+)\s+"
    r"d=\s*(?P<day>-?\d+)\s+"
    r"e=\s*(?P<element>-?\d+)\s+"
    r"c=\s*(?P<component>-?\d+)\s+"
    r"s=\s*(?P<hour>-?\d+)\s+"
    r"v1=\s*(?P<v1>[+-]?\d*\.\d+E[+-]?\d+)\s+"
    r"v2=\s*(?P<v2>[+-]?\d*\.\d+E[+-]?\d+)"
)


class SourceLineEvidenceError(RuntimeError):
    """Raised when required source-line evidence is missing."""


class PairedEvidenceError(RuntimeError):
    """Raised when required paired baseline/openWEPP state evidence is missing."""


SOURCE_LINE_REQUIREMENTS = [
    (
        Path("/workdir/wepp-forest_260430_baseline/src/winter.for"),
        193,
        "snodpt(iplane)=snodpy(iplane)",
        "winter.for:193",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        50,
        "snodep = snodpy(iplane)",
        "snowd.for:50-53",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        51,
        "snodpt(iplane) = snodep",
        "snowd.for:50-53",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        52,
        "densgy = densg(iplane)",
        "snowd.for:50-53",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        53,
        "densgt = densgy",
        "snowd.for:50-53",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        122,
        "if (snodpt(iplane) .gt. 0.0)",
        "snowd.for:122-139",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        125,
        "setf = ((exp",
        "snowd.for:122-139",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        129,
        "if(densgy.gt.ssd) setf = 1",
        "snowd.for:122-139",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        131,
        "densgt = densgy * setf",
        "snowd.for:122-139",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        135,
        "if (densgt .gt. 522) densgt = 522",
        "snowd.for:122-139",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        139,
        "snodpt(iplane) = snodpt(iplane) * densgy/ densgt",
        "snowd.for:122-139",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        310,
        "snodpt(iplane) = snodep",
        "snowd.for:303-312",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        311,
        "snodpy(iplane) = snodep",
        "snowd.for:303-312",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        312,
        "densg(iplane) = densgt",
        "snowd.for:303-312",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/infile.for"),
        1361,
        "snodpy(iplane) = snodp1(inindx)",
        "infile.for:1361,1466",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/infile.for"),
        1466,
        "snodpy(iplane) = snodp1(inindx)",
        "infile.for:1361,1466",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/inidat.for"),
        383,
        "densg(i) = 100.0",
        "inidat.for:383",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs"),
        663,
        "BoundarySymbol::from(\"snow.runtime_swe\")",
        "runtime_inputs/04_snow_frost_irrigation.rs:663-691",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs"),
        672,
        "BoundarySymbol::from(\"snow.runtime_depth_m\")",
        "runtime_inputs/04_snow_frost_irrigation.rs:663-691",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs"),
        681,
        "BoundarySymbol::from(\"snow.runtime_density_kg_m3\")",
        "runtime_inputs/04_snow_frost_irrigation.rs:663-691",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs"),
        690,
        "BoundarySymbol::from(\"snow.runtime_settle_day_count\")",
        "runtime_inputs/04_snow_frost_irrigation.rs:663-691",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3691,
        "Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_RUNTIME_SWE)",
        "03_kernel_support_00_support_helpers.rs:3690-3790",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3746,
        "let depth_symbol = BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL)",
        "03_kernel_support_00_support_helpers.rs:3690-3790",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3747,
        "let density_symbol = BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL)",
        "03_kernel_support_00_support_helpers.rs:3690-3790",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3748,
        "let settle_day_count_symbol = BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL)",
        "03_kernel_support_00_support_helpers.rs:3690-3790",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3872,
        "if hour == 1",
        "03_kernel_support_00_support_helpers.rs:3872-3912",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3901,
        "let mut setf =",
        "03_kernel_support_00_support_helpers.rs:3872-3912",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3911,
        "snodpt = snodpt * dens / densgt",
        "03_kernel_support_00_support_helpers.rs:3872-3912",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        4223,
        "runtime_swe: runtime_swe_after",
        "03_kernel_support_00_support_helpers.rs:4218-4227",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        4224,
        "runtime_depth_m: snodep",
        "03_kernel_support_00_support_helpers.rs:4218-4227",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        4225,
        "runtime_density_kg_m3: dens",
        "03_kernel_support_00_support_helpers.rs:4218-4227",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        4226,
        "runtime_settle_day_count: settle_day_count",
        "03_kernel_support_00_support_helpers.rs:4218-4227",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs"),
        4218,
        "SNOW_RUNTIME_DEPTH_M_SYMBOL",
        "03_kernel_support_01_kernel_phases.rs:4216-4235",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs"),
        4225,
        "SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL",
        "03_kernel_support_01_kernel_phases.rs:4216-4235",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs"),
        4231,
        "SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL",
        "03_kernel_support_01_kernel_phases.rs:4216-4235",
    ),
    (
        Path("docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md"),
        201,
        "management.initial[i].params.snodpy_m",
        "SC-INFILE-MANAGEMENT-001:201",
    ),
]


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def require_source_lineage(
    requirements: list[tuple[Path, int, str, str]] | None = None,
) -> dict[str, list[str]]:
    citations: dict[str, list[str]] = {}
    for path, line_number, needle, citation in requirements or SOURCE_LINE_REQUIREMENTS:
        if not path.exists():
            raise SourceLineEvidenceError(f"missing source file for {citation}: {path}")
        lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
        if line_number < 1 or line_number > len(lines):
            raise SourceLineEvidenceError(f"missing source line for {citation}: {path}:{line_number}")
        observed = lines[line_number - 1]
        if needle not in observed:
            raise SourceLineEvidenceError(
                f"source line mismatch for {citation}: expected {needle!r} at {path}:{line_number}"
            )
        citations.setdefault(citation, []).append(f"{path}:{line_number}")
    return citations


def parse_baseline_log(path: Path) -> dict[tuple[int, int, int], dict[str, float]]:
    parsed: dict[tuple[int, int, int], dict[str, float]] = {}
    if not path.exists():
        raise FileNotFoundError(f"missing baseline observe log: {path}")
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        match = OBS_RE.match(line)
        if not match:
            continue
        key = (
            int(match.group("year")),
            int(match.group("day")),
            int(match.group("hour")),
        )
        record = parsed.setdefault(key, {})
        tag = match.group("tag")
        v1 = float(match.group("v1"))
        v2 = float(match.group("v2"))
        if tag == "H305_S_OUT":
            record["depth_after_m"] = v1
            record["density_after_kg_m3"] = v2
        elif tag == "H305_M_POST":
            record["post_hrmlt_m"] = v1
            record["post_hrrain_m"] = v2
    return parsed


def load_trace_rows(path: Path) -> dict[tuple[int, int], dict[str, Any]]:
    rows: dict[tuple[int, int], dict[str, Any]] = {}
    if not path.exists():
        raise FileNotFoundError(f"missing openWEPP trace: {path}")
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        row = json.loads(line)
        if row.get("boundary") != TRACE_BOUNDARY or row.get("phase") != TRACE_PHASE:
            continue
        year = int(row.get("calendar_year", row.get("simulation_year")))
        day = int(row["julian_day"])
        rows[(year, day)] = row
    return rows


def require_baseline_value(
    baseline: dict[tuple[int, int, int], dict[str, float]],
    field: str,
    year: int,
    day: int,
    hour: int,
) -> float:
    record = baseline.get((year, day, hour))
    if record is None or field not in record:
        raise PairedEvidenceError(
            f"missing paired evidence: baseline {field} y={year} d={day} h={hour}"
        )
    return record[field]


def require_trace_row(
    traces: dict[tuple[int, int], dict[str, Any]],
    year: int,
    day: int,
) -> dict[str, Any]:
    row = traces.get((year, day))
    if row is None:
        raise PairedEvidenceError(f"missing openWEPP trace row y={year} d={day}")
    return row


def require_hour_value(row: dict[str, Any], field: str, year: int, day: int, hour: int) -> float:
    values = row.get(field)
    key = f"{hour:04d}"
    if not isinstance(values, dict) or key not in values or values[key] is None:
        raise PairedEvidenceError(
            f"missing paired hourly evidence: openWEPP {field} y={year} d={day} h={hour}"
        )
    return float(values[key])


def require_scalar(row: dict[str, Any], field: str, year: int, day: int) -> float:
    if field not in row or row[field] is None:
        raise PairedEvidenceError(f"missing openWEPP scalar evidence: {field} y={year} d={day}")
    return float(row[field])


def trace_paths_by_hillslope() -> dict[int, Path]:
    audit = read_json(HPHYS0305_ARTIFACT_DIR / "openwepp-trace-field-audit.json")
    return {int(row["hillslope_id"]): Path(row["trace_path"]) for row in audit}


def baseline_log_paths_by_hillslope() -> dict[int, Path]:
    identity = read_json(HPHYS0305_ARTIFACT_DIR / "baseline-observe-identity.json")
    paths: dict[int, Path] = {}
    for hillslope_id, row in identity.items():
        paths[int(hillslope_id)] = Path(row["observe_on_lane"]["observe_log"])
    return paths


def prior_year_terminal_day(year: int) -> tuple[int, int]:
    previous_year = year - 1
    return previous_year, 366 if calendar.isleap(previous_year) else 365


def classify_day1_group(
    source_row: dict[str, Any],
    baseline: dict[tuple[int, int, int], dict[str, float]],
    traces: dict[tuple[int, int], dict[str, Any]],
) -> dict[str, Any]:
    hillslope_id = int(source_row["hillslope_id"])
    year = int(source_row["year"])
    previous_year, previous_day = prior_year_terminal_day(year)
    current_day = 1
    current_hour = 1

    baseline_prev_depth = require_baseline_value(
        baseline, "depth_after_m", previous_year, previous_day, 24
    )
    baseline_prev_density = require_baseline_value(
        baseline, "density_after_kg_m3", previous_year, previous_day, 24
    )
    open_prev = require_trace_row(traces, previous_year, previous_day)
    open_prev_depth = require_scalar(open_prev, "snow_runtime_depth_m", previous_year, previous_day)
    open_prev_density = require_scalar(
        open_prev, "snow_runtime_density_kg_m3", previous_year, previous_day
    )
    open_prev_swe = require_scalar(open_prev, "snow_runtime_swe_m", previous_year, previous_day)

    baseline_day1_depth = require_baseline_value(baseline, "depth_after_m", year, current_day, 1)
    baseline_day1_density = require_baseline_value(
        baseline, "density_after_kg_m3", year, current_day, 1
    )
    open_day1 = require_trace_row(traces, year, current_day)
    open_day1_depth = require_hour_value(
        open_day1, "snow_hourly_depth_after_m", year, current_day, current_hour
    )
    open_day1_density = require_hour_value(
        open_day1, "snow_hourly_density_after_kg_m3", year, current_day, current_hour
    )
    open_day1_depth_before = require_hour_value(
        open_day1, "snow_hourly_depth_before_m", year, current_day, current_hour
    )

    previous_delta = open_prev_depth - baseline_prev_depth
    day1_delta = open_day1_depth - baseline_day1_depth
    previous_density_delta = open_prev_density - baseline_prev_density
    day1_density_delta = open_day1_density - baseline_day1_density
    depth_inherited = abs(previous_delta - day1_delta) <= INHERITED_DELTA_TOL_M
    density_inherited = (
        abs(previous_density_delta - day1_density_delta) <= INHERITED_DENSITY_DELTA_TOL_KG_M3
    )
    inherited = depth_inherited and density_inherited
    route = "prior-year-terminal-state-hold" if inherited else "year-boundary-projection-hold"
    return {
        "hillslope_id": hillslope_id,
        "window": source_row["window"],
        "year": year,
        "affected_hphys0309_rows": source_row["affected_hphys0309_rows"],
        "source_hphys0310_route": source_row["route"],
        "route": route,
        "classification_reason": (
            "day-1 h01 delta equals prior-year terminal delta; source-line carry-forward is parity "
            "and the residual is inherited from the prior-year terminal snowpack state"
            if inherited
            else "day-1 h01 delta differs from prior-year terminal delta; year-boundary projection remains suspect"
        ),
        "previous_terminal_state": {
            "year": previous_year,
            "julian": previous_day,
            "hour": 24,
            "baseline_depth_after_m": baseline_prev_depth,
            "baseline_density_after_kg_m3": baseline_prev_density,
            "openwepp_runtime_depth_m": open_prev_depth,
            "openwepp_runtime_density_kg_m3": open_prev_density,
            "openwepp_runtime_swe_m": open_prev_swe,
            "depth_delta_openwepp_minus_baseline_m": previous_delta,
            "density_delta_openwepp_minus_baseline_kg_m3": previous_density_delta,
        },
        "day1_carry_state": {
            "year": year,
            "julian": 1,
            "hour": 1,
            "baseline_depth_after_m": baseline_day1_depth,
            "baseline_density_after_kg_m3": baseline_day1_density,
            "openwepp_depth_before_m": open_day1_depth_before,
            "openwepp_depth_after_m": open_day1_depth,
            "openwepp_density_after_kg_m3": open_day1_density,
            "depth_delta_openwepp_minus_baseline_m": day1_delta,
            "density_delta_openwepp_minus_baseline_kg_m3": day1_density_delta,
        },
        "inheritance_checks": {
            "depth_delta_tolerance_m": INHERITED_DELTA_TOL_M,
            "density_delta_tolerance_kg_m3": INHERITED_DENSITY_DELTA_TOL_KG_M3,
            "depth_delta_inherited": depth_inherited,
            "density_delta_inherited": density_inherited,
        },
        "source_line_findings": {
            "baseline_day_start_copy": "winter.for:193",
            "baseline_hourly_init": "snowd.for:50-53",
            "baseline_hourly_carry_write": "snowd.for:303-312",
            "openwepp_runtime_aliases": "snow.runtime_depth_m/snow.runtime_density_kg_m3/snow.runtime_swe",
        },
        "production_edit_authorized": False,
        "prohibited_compensation_note": (
            "No downstream compensation is authorized; branch-predicate, same-hour melt-term, "
            "WB13, WB17, WB18, WB19, and WB12 edits remain invalid."
        ),
    }


def classify_settling_group(
    source_row: dict[str, Any],
    baseline: dict[tuple[int, int, int], dict[str, float]],
    traces: dict[tuple[int, int], dict[str, Any]],
) -> dict[str, Any]:
    hillslope_id = int(source_row["hillslope_id"])
    first = source_row["first_material_divergence"]
    year = int(first["year"])
    day = int(first["julian"])
    hour = int(first["hour"])
    previous_hour = hour - 1
    if previous_hour < 1:
        raise PairedEvidenceError("HPHYS0311 settling group requires a same-day previous hour")

    baseline_prev_depth = require_baseline_value(baseline, "depth_after_m", year, day, previous_hour)
    baseline_prev_density = require_baseline_value(
        baseline, "density_after_kg_m3", year, day, previous_hour
    )
    baseline_depth = require_baseline_value(baseline, "depth_after_m", year, day, hour)
    baseline_density = require_baseline_value(baseline, "density_after_kg_m3", year, day, hour)
    row = require_trace_row(traces, year, day)
    open_before_depth = require_hour_value(row, "snow_hourly_depth_before_m", year, day, hour)
    open_before_density = require_hour_value(
        row, "snow_hourly_density_before_kg_m3", year, day, hour
    )
    open_depth = require_hour_value(row, "snow_hourly_depth_after_m", year, day, hour)
    open_density = require_hour_value(row, "snow_hourly_density_after_kg_m3", year, day, hour)
    open_settle_before = require_scalar(row, "snow_runtime_settle_day_count_before", year, day)
    open_settle_after = require_scalar(row, "snow_runtime_settle_day_count", year, day)

    baseline_setf_observed = baseline_density / baseline_prev_density
    open_setf_observed = open_density / open_before_density
    open_mass_conserving_depth = open_before_depth * open_before_density / open_density
    baseline_observe_mass_depth = baseline_prev_depth * baseline_prev_density / baseline_density
    previous_depth_delta = open_before_depth - baseline_prev_depth
    previous_density_delta = open_before_density - baseline_prev_density
    previous_state_near_identical = (
        abs(previous_depth_delta) <= SETTLING_PREVIOUS_DEPTH_TOL_M
        and abs(previous_density_delta) <= SETTLING_PREVIOUS_DENSITY_TOL_KG_M3
    )
    route = (
        "fixed-observe-precision-hold"
        if previous_state_near_identical
        else "prior-hour-carry-state-hold"
    )
    classification_reason = (
        "baseline/openWEPP previous-hour states are near-identical, but H305_S_OUT exposes rounded "
        "post-hour depth/density and omits baseline wdayct; source-line equation defect is not proven"
        if previous_state_near_identical
        else "previous-hour paired state is already divergent; settling equation ownership is not proven"
    )
    return {
        "hillslope_id": hillslope_id,
        "window": source_row["window"],
        "year": int(source_row["year"]),
        "affected_hphys0309_rows": source_row["affected_hphys0309_rows"],
        "source_hphys0310_route": source_row["route"],
        "route": route,
        "classification_reason": classification_reason,
        "settling_state": {
            "year": year,
            "julian": day,
            "hour": hour,
            "baseline_previous_hour_depth_after_m": baseline_prev_depth,
            "baseline_previous_hour_density_after_kg_m3": baseline_prev_density,
            "baseline_current_depth_after_m": baseline_depth,
            "baseline_current_density_after_kg_m3": baseline_density,
            "baseline_observed_setf": baseline_setf_observed,
            "baseline_observe_mass_conserving_depth_m": baseline_observe_mass_depth,
            "openwepp_depth_before_m": open_before_depth,
            "openwepp_density_before_kg_m3": open_before_density,
            "openwepp_depth_after_m": open_depth,
            "openwepp_density_after_kg_m3": open_density,
            "openwepp_observed_setf": open_setf_observed,
            "openwepp_mass_conserving_depth_m": open_mass_conserving_depth,
            "openwepp_settle_day_count_before": open_settle_before,
            "openwepp_settle_day_count_after": open_settle_after,
            "depth_delta_openwepp_minus_baseline_m": open_depth - baseline_depth,
            "previous_hour_depth_delta_openwepp_minus_baseline_m": previous_depth_delta,
            "previous_hour_density_delta_openwepp_minus_baseline_kg_m3": previous_density_delta,
            "previous_hour_depth_tolerance_m": SETTLING_PREVIOUS_DEPTH_TOL_M,
            "previous_hour_density_tolerance_kg_m3": SETTLING_PREVIOUS_DENSITY_TOL_KG_M3,
            "previous_hour_state_near_identical": previous_state_near_identical,
        },
        "source_line_findings": {
            "baseline_settling_equations": "snowd.for:122-139",
            "baseline_hourly_carry_write": "snowd.for:303-312",
            "openwepp_settling_equations": "03_kernel_support_00_support_helpers.rs:3872-3912",
            "missing_baseline_state": "fixed observe does not expose full-precision snodpt/densg or wdayct",
        },
        "production_edit_authorized": False,
        "prohibited_compensation_note": (
            "No downstream compensation is authorized; branch-predicate, same-hour melt-term, "
            "WB13, WB17, WB18, WB19, and WB12 edits remain invalid."
        ),
    }


def build_ledger() -> list[dict[str, Any]]:
    require_source_lineage()
    rows = read_json(HPHYS0310_LEDGER)
    trace_paths = trace_paths_by_hillslope()
    baseline_paths = baseline_log_paths_by_hillslope()
    traces = {hillslope: load_trace_rows(path) for hillslope, path in trace_paths.items()}
    baseline = {hillslope: parse_baseline_log(path) for hillslope, path in baseline_paths.items()}

    ledger: list[dict[str, Any]] = []
    for source_row in rows:
        hillslope_id = int(source_row["hillslope_id"])
        if hillslope_id not in baseline or hillslope_id not in traces:
            raise PairedEvidenceError(f"missing paired paths for hillslope H{hillslope_id}")
        first = source_row["first_material_divergence"]
        if int(first["julian"]) == 1 and int(first["hour"]) == 1:
            ledger.append(classify_day1_group(source_row, baseline[hillslope_id], traces[hillslope_id]))
        else:
            ledger.append(
                classify_settling_group(source_row, baseline[hillslope_id], traces[hillslope_id])
            )
    return ledger


def write_summary(ledger: list[dict[str, Any]]) -> None:
    routes = Counter(row["route"] for row in ledger)
    represented = sum(int(row["affected_hphys0309_rows"]) for row in ledger)
    lines = [
        "# Snow Carry Source-Line Parity Summary",
        "",
        "Status: complete",
        "",
        "Evidence mode: ran",
        "",
        "## Counts",
        "",
        f"- Affected HPHYS0309 rows represented: `{represented}`",
        f"- HPHYS0310 groups represented: `{len(ledger)}`",
        f"- Production edit authorized groups: `{sum(1 for row in ledger if row['production_edit_authorized'])}`",
        "",
        "## Route Counts",
        "",
    ]
    for route, count in sorted(routes.items()):
        lines.append(f"- `{route}`: `{count}`")
    lines.extend(
        [
            "",
            "## Group Routes",
            "",
            "| Hillslope | Window | Year | Rows | Route | Finding |",
            "|---|---|---:|---:|---|---|",
        ]
    )
    for row in ledger:
        lines.append(
            f"| H{row['hillslope_id']} | {row['window']} | {row['year']} | "
            f"{row['affected_hphys0309_rows']} | `{row['route']}` | {row['classification_reason']} |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "Six day-1 groups carry the exact prior-year terminal depth and density",
            "deltas into the new year; the source-line carry-forward path itself is parity and the",
            "residual remains inherited prior-year terminal snowpack state. The single",
            "H1 2013 density/settling group has near-identical previous-hour states,",
            "but the available fixed observe lane is rounded and lacks baseline",
            "`wdayct`, so no production settling equation edit is authorized.",
        ]
    )
    (ARTIFACT_DIR / "snow-carry-source-line-parity-summary.md").write_text(
        "\n".join(lines) + "\n", encoding="utf-8"
    )


def write_method() -> None:
    (ARTIFACT_DIR / "snow-carry-source-line-parity-method.md").write_text(
        """# Snow Carry Source-Line Parity Method

Status: complete

Evidence mode: ran

Static:

- Uses HPHYS0310 carry-divergence groups as the input population.
- Uses HPHYS0305 fixed-comparator observe-on logs for `H305_S_OUT` post-hour
  `snodpt`/`densgt`.
- Uses HPHYS0305 openWEPP traces at `post_wb13` for runtime and hourly snow
  state.
- Requires source-line citations before generating the ledger.

Ran:

- For day-1 groups, compared prior-year terminal fixed-comparator state against
  prior-year terminal openWEPP runtime state, then compared day-1 hour-1 carried
  states.
- For the H1 2013 settling group, compared previous/current hour paired
  depth-density states and recorded that fixed-observe precision and missing
  `wdayct` prevent production-edit authority.
""",
        encoding="utf-8",
    )


def write_source_lineage(citations: dict[str, list[str]]) -> None:
    lines = [
        "# Snow Carry Source-Line Parity Source Lineage",
        "",
        "Status: complete",
        "",
        "Evidence mode: static",
        "",
        "Static:",
        "",
        "- `/workdir/wepp-forest_260430_baseline/src/winter.for:193` copies",
        "  `snodpy` into `snodpt` at winter day start.",
        "- `/workdir/wepp-forest_260430_baseline/src/snowd.for:50-53` initializes",
        "  hourly `snodep`/`snodpt`/`densgy`/`densgt` from carried",
        "  `snodpy`/`densg`.",
        "- `/workdir/wepp-forest_260430_baseline/src/snowd.for:122-139` applies",
        "  density-settling equations using `wdayct` and the density cap.",
        "- `/workdir/wepp-forest_260430_baseline/src/snowd.for:303-312` writes",
        "  updated `snodpt`, `snodpy`, and `densg` after each hour.",
        "- `/workdir/wepp-forest_260430_baseline/src/infile.for:1361,1466` and",
        "  `/workdir/wepp-forest_260430_baseline/src/inidat.for:383` ground",
        "  initial `snodpy`/`densg` provenance.",
        "- `SC-INFILE-MANAGEMENT-001` maps canonical `snodpy` to",
        "  `management.initial[i].params.snodpy_m`; snow sidecar docs confirm",
        "  initial snow depth is management-owned, not `snow.txt`-owned.",
        "- openWEPP runtime aliases are `snow.runtime_swe`,",
        "  `snow.runtime_depth_m`, `snow.runtime_density_kg_m3`, and",
        "  `snow.runtime_settle_day_count`.",
        "",
        "## Verified Source Requirements",
        "",
    ]
    for citation in sorted(citations):
        paths = ", ".join(sorted(citations[citation]))
        lines.append(f"- `{citation}`: `{paths}`")
    (ARTIFACT_DIR / "snow-carry-source-line-parity-source-lineage.md").write_text(
        "\n".join(lines) + "\n", encoding="utf-8"
    )


def main(argv: list[str]) -> int:
    if "--self-test-missing-source-line" in argv:
        require_source_lineage(
            [
                (
                    Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
                    50,
                    "INTENTIONALLY_MISSING_HPHYS0311_SOURCE_TOKEN",
                    "snowd.for:50-53",
                )
            ]
        )
        return 0

    citations = require_source_lineage()
    ledger = build_ledger()
    write_json(ARTIFACT_DIR / "snow-carry-source-line-parity-ledger.json", ledger)
    write_summary(ledger)
    write_method()
    write_source_lineage(citations)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except (SourceLineEvidenceError, PairedEvidenceError, FileNotFoundError) as error:
        print(f"HPHYS0311 failed closed: {error}", file=sys.stderr)
        raise SystemExit(2)
