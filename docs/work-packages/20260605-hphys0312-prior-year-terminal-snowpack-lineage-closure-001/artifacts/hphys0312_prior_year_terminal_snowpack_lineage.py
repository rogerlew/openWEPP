#!/usr/bin/env python3
"""Localize inherited prior-year terminal snowpack deltas for HPHYS0312."""

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
HPHYS0311_LEDGER = (
    Path("docs/work-packages")
    / "20260605-hphys0311-snow-carry-source-line-parity-closure-001"
    / "artifacts"
    / "snow-carry-source-line-parity-ledger.json"
)
HPHYS0305_ARTIFACT_DIR = (
    Path("docs/work-packages")
    / "20260605-hphys0305-paired-melt-term-state-instrumentation-001"
    / "artifacts"
)
TRACE_BOUNDARY = "post_wb13"
TRACE_PHASE = None
MATERIAL_DEPTH_TOL_M = 0.0005
MATERIAL_DENSITY_TOL_KG_M3 = 0.5
TERMINAL_DELTA_TOL_M = 1.0e-12
TERMINAL_DENSITY_DELTA_TOL_KG_M3 = 1.0e-12
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
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        61,
        "if (hour .eq. 1) then",
        "snowd.for:61-65",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        62,
        "wdayct(iplane) = wdayct(iplane) + 1",
        "snowd.for:61-65",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        65,
        "if (hrsnow(hour) .gt. 0.0)  wdayct(iplane) = 1",
        "snowd.for:61-65",
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
        139,
        "snodpt(iplane) = snodpt(iplane) * densgy/ densgt",
        "snowd.for:122-139",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        145,
        "if (hrsnow(hour) .le. 0.0)",
        "snowd.for:145-173",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        167,
        "snodep = snodpt(iplane) + hrsnow(hour)+driftf+driftg",
        "snowd.for:145-173",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        172,
        "densgt = ((densgt * snodpt(iplane) + densgy * driftg)",
        "snowd.for:145-173",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        193,
        "call melt(irtype,wrain,hour)",
        "snowd.for:181-198",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        215,
        "snodpt(iplane) = snodep",
        "snowd.for:215-246",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        218,
        "snodep = snodpt(iplane) - smelt",
        "snowd.for:215-246",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        240,
        "if (densg(iplane).ge. 350) then",
        "snowd.for:240-278",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        246,
        "densgt = densg(iplane) * (snodpt(iplane) / snodep)",
        "snowd.for:240-278",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        260,
        "if(hrrain(hour).gt.0) then",
        "snowd.for:240-278",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        310,
        "snodpt(iplane) = snodep",
        "snowd.for:310-312",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        311,
        "snodpy(iplane) = snodep",
        "snowd.for:310-312",
    ),
    (
        Path("/workdir/wepp-forest_260430_baseline/src/snowd.for"),
        312,
        "densg(iplane) = densgt",
        "snowd.for:310-312",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3872,
        "if hour == 1",
        "03_kernel_support_00_support_helpers.rs:3872-3920",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3875,
        "if hrsnow > WB11_ZERO_THRESHOLD",
        "03_kernel_support_00_support_helpers.rs:3872-3920",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3897,
        "else if daily_mean_temp < 0.0",
        "03_kernel_support_00_support_helpers.rs:3872-3920",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3901,
        "let mut setf =",
        "03_kernel_support_00_support_helpers.rs:3872-3920",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3903,
        "if dens > ssd",
        "03_kernel_support_00_support_helpers.rs:3872-3920",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3911,
        "snodpt = snodpt * dens / densgt",
        "03_kernel_support_00_support_helpers.rs:3872-3920",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        3937,
        "let melt_computation = Self::compute_simimpl29_melt_hour",
        "03_kernel_support_00_support_helpers.rs:3925-4057",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        4005,
        "let mut densgt = dens * (snodpt_after_inputs / snodep)",
        "03_kernel_support_00_support_helpers.rs:3925-4057",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        4075,
        "accumulation_water_m += hrsnow * 0.1",
        "03_kernel_support_00_support_helpers.rs:4075-4109",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        4105,
        "let melt_redistribution = Self::redistribute_daily_signed_snowmelt",
        "03_kernel_support_00_support_helpers.rs:4075-4109",
    ),
    (
        Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
        4223,
        "runtime_swe: runtime_swe_after",
        "03_kernel_support_00_support_helpers.rs:4218-4227",
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
        if not match or match.group("tag") != "H305_S_OUT":
            continue
        key = (
            int(match.group("year")),
            int(match.group("day")),
            int(match.group("hour")),
        )
        parsed[key] = {
            "depth_after_m": float(match.group("v1")),
            "density_after_kg_m3": float(match.group("v2")),
        }
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


def baseline_log_paths_by_hillslope() -> dict[int, Path]:
    identity = read_json(HPHYS0305_ARTIFACT_DIR / "baseline-observe-identity.json")
    paths: dict[int, Path] = {}
    for hillslope_id, row in identity.items():
        paths[int(hillslope_id)] = Path(row["observe_on_lane"]["observe_log"])
    return paths


def trace_paths_by_hillslope() -> dict[int, Path]:
    audit = read_json(HPHYS0305_ARTIFACT_DIR / "openwepp-trace-field-audit.json")
    return {int(row["hillslope_id"]): Path(row["trace_path"]) for row in audit}


def require_baseline_record(
    baseline: dict[tuple[int, int, int], dict[str, float]],
    year: int,
    day: int,
    hour: int,
) -> dict[str, float]:
    record = baseline.get((year, day, hour))
    if record is None:
        raise PairedEvidenceError(f"missing baseline snow state y={year} d={day} h={hour}")
    return record


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
            f"missing openWEPP hourly value {field} y={year} d={day} h={hour}"
        )
    return float(values[key])


def require_scalar(row: dict[str, Any], field: str, year: int, day: int) -> float:
    if field not in row or row[field] is None:
        raise PairedEvidenceError(f"missing openWEPP scalar value {field} y={year} d={day}")
    return float(row[field])


def paired_state(
    baseline: dict[tuple[int, int, int], dict[str, float]],
    traces: dict[tuple[int, int], dict[str, Any]],
    year: int,
    day: int,
    hour: int,
) -> dict[str, Any]:
    baseline_record = require_baseline_record(baseline, year, day, hour)
    trace_row = require_trace_row(traces, year, day)
    baseline_depth = baseline_record["depth_after_m"]
    baseline_density = baseline_record["density_after_kg_m3"]
    open_depth = require_hour_value(trace_row, "snow_hourly_depth_after_m", year, day, hour)
    open_density = require_hour_value(
        trace_row, "snow_hourly_density_after_kg_m3", year, day, hour
    )
    depth_delta = open_depth - baseline_depth
    density_delta = open_density - baseline_density
    return {
        "year": year,
        "julian": day,
        "hour": hour,
        "baseline_depth_after_m": baseline_depth,
        "baseline_density_after_kg_m3": baseline_density,
        "openwepp_depth_before_m": require_hour_value(
            trace_row, "snow_hourly_depth_before_m", year, day, hour
        ),
        "openwepp_depth_after_m": open_depth,
        "openwepp_density_before_kg_m3": require_hour_value(
            trace_row, "snow_hourly_density_before_kg_m3", year, day, hour
        ),
        "openwepp_density_after_kg_m3": open_density,
        "openwepp_snowfall_depth_m": require_hour_value(
            trace_row, "snow_hourly_snowfall_depth_m", year, day, hour
        ),
        "openwepp_melt_branch_active": require_hour_value(
            trace_row, "snow_hourly_melt_branch_active", year, day, hour
        ),
        "openwepp_raw_melt_m": require_hour_value(
            trace_row, "snow_hourly_melt_raw_m", year, day, hour
        ),
        "openwepp_routed_melt_m": require_hour_value(
            trace_row, "snow_hourly_melt_m", year, day, hour
        ),
        "openwepp_rain_m": require_hour_value(trace_row, "snow_hourly_rain_m", year, day, hour),
        "openwepp_air_temp_c": require_hour_value(
            trace_row, "winter_hourly_air_temp_c", year, day, hour
        ),
        "depth_delta_openwepp_minus_baseline_m": depth_delta,
        "density_delta_openwepp_minus_baseline_kg_m3": density_delta,
        "material_depth_divergent": abs(depth_delta) > MATERIAL_DEPTH_TOL_M,
        "material_density_divergent": abs(density_delta) > MATERIAL_DENSITY_TOL_KG_M3,
    }


def is_material(state: dict[str, Any]) -> bool:
    return bool(state["material_depth_divergent"] or state["material_density_divergent"])


def previous_hour(year: int, day: int, hour: int) -> tuple[int, int, int] | None:
    if hour > 1:
        return year, day, hour - 1
    if day > 1:
        return year, day - 1, 24
    return None


def require_terminal_matches_hphys0311(
    terminal: dict[str, Any],
    hphys0311_state: dict[str, Any],
) -> dict[str, Any]:
    depth_delta_residual = (
        terminal["depth_delta_openwepp_minus_baseline_m"]
        - hphys0311_state["depth_delta_openwepp_minus_baseline_m"]
    )
    density_delta_residual = (
        terminal["density_delta_openwepp_minus_baseline_kg_m3"]
        - hphys0311_state["density_delta_openwepp_minus_baseline_kg_m3"]
    )
    depth_matches = abs(depth_delta_residual) <= TERMINAL_DELTA_TOL_M
    density_matches = abs(density_delta_residual) <= TERMINAL_DENSITY_DELTA_TOL_KG_M3
    if not depth_matches or not density_matches:
        raise PairedEvidenceError(
            "terminal state does not match HPHYS0311 inherited delta: "
            f"depth residual={depth_delta_residual} density residual={density_delta_residual}"
        )
    return {
        "depth_delta_residual_m": depth_delta_residual,
        "density_delta_residual_kg_m3": density_delta_residual,
        "depth_delta_matches_hphys0311": depth_matches,
        "density_delta_matches_hphys0311": density_matches,
    }


def classify_first_divergence(first_state: dict[str, Any], previous_state: dict[str, Any] | None) -> tuple[str, str]:
    if first_state["julian"] == 1 and first_state["hour"] == 1 and previous_state is None:
        return (
            "year-start-inherited-state-hold",
            "first material divergence is already present at prior-year day-1 hour-1; "
            "the scanned year inherits a snowpack residual from an earlier year boundary",
        )
    cold_settling = (
        first_state["openwepp_depth_before_m"] > MATERIAL_DEPTH_TOL_M
        and abs(first_state["openwepp_snowfall_depth_m"]) <= MATERIAL_DEPTH_TOL_M
        and abs(first_state["openwepp_raw_melt_m"]) <= MATERIAL_DEPTH_TOL_M
        and abs(first_state["openwepp_routed_melt_m"]) <= MATERIAL_DEPTH_TOL_M
        and abs(first_state["openwepp_rain_m"]) <= MATERIAL_DEPTH_TOL_M
        and first_state["openwepp_melt_branch_active"] == 0.0
        and first_state["openwepp_air_temp_c"] < 0.0
    )
    if cold_settling:
        return (
            "settling-depth-update-hold",
            "first material divergence occurs during cold existing-snow no-snowfall/no-melt settling; "
            "full-precision baseline wdayct/equation reconstruction is required before edits",
        )
    return (
        "within-year-snow-state-source-hold",
        "first material divergence occurs within the scanned prior year but the source lane is not closed",
    )


def scan_prior_year_group(
    row: dict[str, Any],
    baseline: dict[tuple[int, int, int], dict[str, float]],
    traces: dict[tuple[int, int], dict[str, Any]],
) -> dict[str, Any]:
    hphys0311_terminal = row["previous_terminal_state"]
    scan_year = int(hphys0311_terminal["year"])
    max_day = 366 if calendar.isleap(scan_year) else 365
    first_state: dict[str, Any] | None = None
    last_within_tolerance: dict[str, Any] | None = None

    for day in range(1, max_day + 1):
        for hour in range(1, 25):
            state = paired_state(baseline, traces, scan_year, day, hour)
            if is_material(state):
                first_state = state
                break
            last_within_tolerance = state
        if first_state is not None:
            break
    if first_state is None:
        raise PairedEvidenceError(
            f"no material prior-year divergence found for H{row['hillslope_id']} {row['window']} {scan_year}"
        )

    previous_state = last_within_tolerance
    previous_key = previous_hour(scan_year, int(first_state["julian"]), int(first_state["hour"]))
    if previous_state is None and previous_key is not None:
        previous_state = paired_state(baseline, traces, *previous_key)
    route, reason = classify_first_divergence(first_state, previous_state)
    terminal_state = paired_state(baseline, traces, scan_year, max_day, 24)
    terminal_continuity = require_terminal_matches_hphys0311(terminal_state, hphys0311_terminal)

    return {
        "hillslope_id": int(row["hillslope_id"]),
        "window": row["window"],
        "target_year": int(row["year"]),
        "scan_year": scan_year,
        "affected_hphys0309_rows": int(row["affected_hphys0309_rows"]),
        "source_hphys0311_route": row["route"],
        "route": route,
        "classification_reason": reason,
        "material_thresholds": {
            "depth_tolerance_m": MATERIAL_DEPTH_TOL_M,
            "density_tolerance_kg_m3": MATERIAL_DENSITY_TOL_KG_M3,
        },
        "first_material_divergence": first_state,
        "last_within_tolerance_state_before_first_divergence": previous_state,
        "terminal_state": terminal_state,
        "terminal_continuity": terminal_continuity,
        "source_line_findings": {
            "baseline_settle_day_count": "snowd.for:61-65",
            "baseline_cold_settling": "snowd.for:122-139",
            "baseline_snowfall_mixing": "snowd.for:145-173",
            "baseline_warm_melt_density": "snowd.for:181-278",
            "baseline_hourly_carry_write": "snowd.for:310-312",
            "openwepp_hourly_update": "03_kernel_support_00_support_helpers.rs:3872-4227",
        },
        "production_edit_authorized": False,
        "prohibited_compensation_note": (
            "No downstream compensation is authorized; branch-predicate, melt-term, WB13, "
            "WB17, WB18, WB19, and WB12 edits remain invalid."
        ),
    }


def write_summary(ledger: list[dict[str, Any]]) -> None:
    route_counts = Counter(row["route"] for row in ledger)
    represented = sum(int(row["affected_hphys0309_rows"]) for row in ledger)
    lines = [
        "# Prior-Year Terminal Snowpack Lineage Summary",
        "",
        "Status: complete",
        "",
        "Evidence mode: ran",
        "",
        "## Counts",
        "",
        f"- HPHYS0311 inherited terminal groups represented: `{len(ledger)}`",
        f"- Affected HPHYS0309 rows represented: `{represented}`",
        "- Production edit authorized groups: `0`",
        "",
        "## Route Counts",
        "",
    ]
    for route, count in sorted(route_counts.items()):
        lines.append(f"- `{route}`: `{count}`")
    lines.extend(
        [
            "",
            "## Group Routes",
            "",
            "| Hillslope | Window | Target Year | Scan Year | Rows | First Divergence | Route |",
            "|---|---|---:|---:|---:|---|---|",
        ]
    )
    for row in ledger:
        first = row["first_material_divergence"]
        lines.append(
            f"| H{row['hillslope_id']} | {row['window']} | {row['target_year']} | "
            f"{row['scan_year']} | {row['affected_hphys0309_rows']} | "
            f"d{first['julian']} h{first['hour']} | `{row['route']}` |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The three 2014-target rows localize to within-year cold existing-snow",
            "settling/depth updates in 2013. The three 2016-target rows are already",
            "materially divergent at 2015 day-1 hour-1, so they remain inherited",
            "year-start state holds. No production edit is authorized.",
        ]
    )
    (ARTIFACT_DIR / "prior-year-terminal-snowpack-lineage-summary.md").write_text(
        "\n".join(lines) + "\n", encoding="utf-8"
    )


def write_method() -> None:
    (ARTIFACT_DIR / "prior-year-terminal-snowpack-lineage-method.md").write_text(
        """# Prior-Year Terminal Snowpack Lineage Method

Status: complete

Evidence mode: ran

Static:

- Input ledger: HPHYS0311 source-line parity ledger.
- Baseline evidence: HPHYS0305 fixed-comparator `H305_S_OUT` observe lane.
- openWEPP evidence: HPHYS0305 `post_wb13` trace rows.
- Material threshold: `0.0005 m` depth or `0.5 kg m^-3` density.
- Source-line evidence must be present before ledger generation.

Ran:

- Filtered HPHYS0311 to six `prior-year-terminal-state-hold` groups.
- Scanned each prior calendar year from day 1 hour 1 through terminal day hour
  24, preserving the first material paired divergence and the preceding
  within-tolerance state when present.
- Verified terminal deltas match HPHYS0311 inherited terminal deltas.
- Classified rows without authorizing production or downstream compensation.
""",
        encoding="utf-8",
    )


def write_source_lineage(citations: dict[str, list[str]]) -> None:
    lines = [
        "# Prior-Year Terminal Snowpack Lineage Source Lineage",
        "",
        "Status: complete",
        "",
        "Evidence mode: static",
        "",
        "Static:",
        "",
        "- `snowd.for:61-65` grounds settle-day-count increment and reset.",
        "- `snowd.for:122-139` grounds cold existing-snow settling/depth update.",
        "- `snowd.for:145-173` grounds cold no-melt snowfall mixing.",
        "- `snowd.for:181-278` grounds warm melt, density update, and rain retention/release.",
        "- `snowd.for:310-312` grounds post-hour `snodpt`/`snodpy`/`densg` writeback.",
        "- openWEPP `03_kernel_support_00_support_helpers.rs:3872-4227` is the homologous runtime snow update lane.",
        "",
        "## Verified Source Requirements",
        "",
    ]
    for citation, paths in sorted(citations.items()):
        lines.append(f"- `{citation}`: {', '.join(paths)}")
    (ARTIFACT_DIR / "prior-year-terminal-snowpack-lineage-source-lineage.md").write_text(
        "\n".join(lines) + "\n", encoding="utf-8"
    )


def run() -> list[dict[str, Any]]:
    citations = require_source_lineage()
    hphys0311_rows = read_json(HPHYS0311_LEDGER)
    rows = [row for row in hphys0311_rows if row.get("route") == "prior-year-terminal-state-hold"]
    if len(rows) != 6:
        raise PairedEvidenceError(f"expected six HPHYS0311 inherited groups, got {len(rows)}")
    baseline_paths = baseline_log_paths_by_hillslope()
    trace_paths = trace_paths_by_hillslope()
    baselines = {
        hillslope_id: parse_baseline_log(path) for hillslope_id, path in baseline_paths.items()
    }
    traces = {hillslope_id: load_trace_rows(path) for hillslope_id, path in trace_paths.items()}
    ledger = []
    for row in rows:
        hillslope_id = int(row["hillslope_id"])
        if hillslope_id not in baselines or hillslope_id not in traces:
            raise PairedEvidenceError(f"missing paired evidence paths for H{hillslope_id}")
        ledger.append(scan_prior_year_group(row, baselines[hillslope_id], traces[hillslope_id]))
    write_json(ARTIFACT_DIR / "prior-year-terminal-snowpack-lineage-ledger.json", ledger)
    write_summary(ledger)
    write_method()
    write_source_lineage(citations)
    return ledger


def main(argv: list[str]) -> int:
    try:
        if argv == ["--self-test-missing-source-line"]:
            bad_requirements = list(SOURCE_LINE_REQUIREMENTS)
            path, line_number, _needle, citation = bad_requirements[0]
            bad_requirements[0] = (
                path,
                line_number,
                "INTENTIONALLY_MISSING_HPHYS0312_SOURCE_TOKEN",
                citation,
            )
            require_source_lineage(bad_requirements)
            return 0
        run()
        return 0
    except Exception as error:
        print(f"HPHYS0312 failed closed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
