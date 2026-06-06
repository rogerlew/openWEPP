#!/usr/bin/env python3
"""Diagnose HPHYS0307 branch-extra melt-call keys at state-ordering level."""

from __future__ import annotations

import json
import re
from collections import Counter
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
HPHYS0306_LEDGER = (
    REPO
    / "docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/artifacts/branch-active-melt-term-ledger.json"
)
HPHYS0307_LEDGER = (
    REPO
    / "docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/artifacts/melt-call-branch-activation-ledger.json"
)
HPHYS0305_ARTIFACT_DIR = (
    REPO
    / "docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts"
)
FIXED_COMMIT = "47ac4c32faeea81bb99081f955a14c38b815ef4d"
TRACE_BOUNDARY = "post_wb13"
TRACE_PHASE = None
ZERO_TOL = 1.0e-12

OBS_RE = re.compile(
    r"^(?P<tag>\S+)\s+y=\s*(?P<year>-?\d+)\s+d=\s*(?P<day>-?\d+)"
    r"\s+e=\s*(?P<element>-?\d+)\s+c=\s*(?P<chan>-?\d+)"
    r"\s+s=\s*(?P<hour>-?\d+)\s+v1=\s*(?P<v1>[-+0-9.Ee]+)"
    r"\s+v2=\s*(?P<v2>[-+0-9.Ee]+)"
)

OPENWEPP_FIELDS = {
    "branch_active": "snow_hourly_melt_branch_active",
    "depth_before_m": "snow_hourly_depth_before_m",
    "snowfall_depth_m": "snow_hourly_snowfall_depth_m",
    "depth_available_m": "snow_hourly_depth_available_m",
    "depth_after_m": "snow_hourly_depth_after_m",
    "density_before_kg_m3": "snow_hourly_density_before_kg_m3",
    "density_after_kg_m3": "snow_hourly_density_after_kg_m3",
    "rain_m": "snow_hourly_rain_m",
    "air_temp_c": "winter_hourly_air_temp_c",
    "dewpoint_c": "winter_hourly_dewpoint_c",
    "wind_m_s": "winter_hourly_wind_m_s",
    "rad_mj_m2_h": "winter_hourly_rad_mj_m2",
    "cloud_fraction": "winter_hourly_cloud_fraction",
    "melt_raw_m": "snow_hourly_melt_raw_m",
    "amelt_in": "snow_hourly_melt_amelt_in",
    "bmelt_in": "snow_hourly_melt_bmelt_in",
    "cmelt_in": "snow_hourly_melt_cmelt_in",
    "dmelt_in": "snow_hourly_melt_dmelt_in",
}


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def window_keys(year: int, start: int, end: int) -> set[tuple[int, int, int]]:
    return {(year, day, hour) for day in range(start, end + 1) for hour in range(1, 25)}


def parse_baseline_log(path: Path) -> dict[tuple[int, int, int], dict[str, float]]:
    parsed: dict[tuple[int, int, int], dict[str, float]] = {}
    if not path.exists():
        return parsed
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        match = OBS_RE.match(line)
        if not match:
            continue
        key = (
            int(match.group("year")),
            int(match.group("day")),
            int(match.group("hour")),
        )
        parsed.setdefault(key, {})
        tag = match.group("tag")
        v1 = float(match.group("v1"))
        v2 = float(match.group("v2"))
        if tag == "H305_T_AB":
            parsed[key]["amelt_in"] = v1
            parsed[key]["bmelt_in"] = v2
        elif tag == "H305_T_CD":
            parsed[key]["cmelt_in"] = v1
            parsed[key]["dmelt_in"] = v2
        elif tag == "H305_F_HT":
            parsed[key]["air_temp_c"] = v1
            parsed[key]["dewpoint_c"] = v2
        elif tag == "H305_F_RR":
            parsed[key]["rad_mj_m2_h"] = v1
            parsed[key]["rain_m"] = v2
        elif tag == "H305_F_CV":
            parsed[key]["cloud_fraction"] = v1
            parsed[key]["wind_m_s"] = v2
        elif tag == "H305_S_OUT":
            parsed[key]["snodpt_m"] = v1
            parsed[key]["densgt_kg_m3"] = v2
        elif tag == "H305_M_POST":
            parsed[key]["hrmlt_m"] = v1
            parsed[key]["post_hrrain_m"] = v2
    return parsed


def load_trace_rows(path: Path) -> dict[tuple[int, int], dict[str, Any]]:
    rows: dict[tuple[int, int], dict[str, Any]] = {}
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        row = json.loads(line)
        if row.get("boundary") != TRACE_BOUNDARY or row.get("phase") != TRACE_PHASE:
            continue
        year = int(row.get("calendar_year", row.get("simulation_year")))
        day = int(row["julian_day"])
        rows[(year, day)] = row
    return rows


def hour_value(row: dict[str, Any], field: str, hour: int) -> float | None:
    values = row.get(field)
    if isinstance(values, dict):
        value = values.get(f"{hour:04d}")
        return None if value is None else float(value)
    if values is None:
        return None
    return float(values)


def openwepp_hour_state(
    rows: dict[tuple[int, int], dict[str, Any]], key: tuple[int, int, int]
) -> dict[str, float | None]:
    year, day, hour = key
    row = rows.get((year, day), {})
    return {name: hour_value(row, field, hour) for name, field in OPENWEPP_FIELDS.items()}


def is_positive(value: float | None) -> bool:
    return value is not None and value > ZERO_TOL


def route_key(
    lane: str,
    baseline_state: dict[str, float],
    openwepp_state: dict[str, float | None],
) -> tuple[str, str]:
    if lane == "baseline-extra-melt-call":
        if not any(
            is_positive(openwepp_state.get(field))
            for field in ("depth_before_m", "snowfall_depth_m", "depth_available_m", "depth_after_m")
        ):
            return (
                "snow-state-carry-depletion-hold",
                "openWEPP branch inactive because snow depth surfaces are zero while fixed baseline still reached melt.for",
            )
        return (
            "branch-predicate-nonzero-state-hold",
            "openWEPP branch inactive despite nonzero snow state; requires source-line predicate proof before edit",
        )
    return (
        "baseline-branch-instrumentation-hold",
        "openWEPP branch active but fixed-baseline has no paired melt.for observation; instrument baseline branch predicates before edit",
    )


def build_ledger() -> list[dict[str, Any]]:
    h306 = read_json(HPHYS0306_LEDGER)
    h307 = {
        (int(row["hillslope_id"]), row["window"]): row for row in read_json(HPHYS0307_LEDGER)
    }
    identity = read_json(HPHYS0305_ARTIFACT_DIR / "baseline-observe-identity.json")
    trace_audit = read_json(HPHYS0305_ARTIFACT_DIR / "openwepp-trace-field-audit.json")
    trace_paths = {int(item["hillslope_id"]): Path(item["trace_path"]) for item in trace_audit}
    baseline_by_hill = {
        int(hill): parse_baseline_log(Path(data["observe_on_lane"]["observe_log"]))
        for hill, data in identity.items()
    }
    openwepp_by_hill = {
        hill: load_trace_rows(path) for hill, path in trace_paths.items()
    }

    ledger: list[dict[str, Any]] = []
    for row in h306:
        hill = int(row["hillslope_id"])
        key = (hill, row["window"])
        if key not in h307:
            continue
        if h307[key]["source_classification"] not in {
            "baseline-extra-melt-call",
            "openwepp-extra-melt-call",
        }:
            continue
        keys = window_keys(int(row["year"]), int(row["start_julian"]), int(row["end_julian"]))
        baseline = baseline_by_hill[hill]
        openwepp_rows = openwepp_by_hill[hill]
        baseline_active = {item for item in keys if "amelt_in" in baseline.get(item, {})}
        openwepp_active = {
            item
            for item in keys
            if bool(openwepp_hour_state(openwepp_rows, item).get("branch_active"))
        }
        for lane, extra_keys in (
            ("baseline-extra-melt-call", sorted(baseline_active - openwepp_active)),
            ("openwepp-extra-melt-call", sorted(openwepp_active - baseline_active)),
        ):
            for extra_key in extra_keys:
                baseline_state = baseline.get(extra_key, {})
                openwepp_state = openwepp_hour_state(openwepp_rows, extra_key)
                route, reason = route_key(lane, baseline_state, openwepp_state)
                year, day, hour = extra_key
                ledger.append(
                    {
                        "hillslope_id": hill,
                        "window": row["window"],
                        "year": year,
                        "julian": day,
                        "hour": hour,
                        "branch_extra_lane": lane,
                        "route": route,
                        "classification_reason": reason,
                        "baseline_melt_call_observed": "amelt_in" in baseline_state,
                        "openwepp_branch_active": bool(openwepp_state.get("branch_active")),
                        "baseline_state": baseline_state,
                        "openwepp_state": openwepp_state,
                        "fixed_comparator_commit": FIXED_COMMIT,
                        "baseline_source": "/workdir/wepp-forest_260430_baseline/src/snowd.for:116-193",
                        "openwepp_source": "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3889-3949",
                        "production_edit_authorized": False,
                        "prohibited_compensation_note": "No WB13/WB17/WB18/WB19/WB12 compensation is authorized.",
                    }
                )
    return ledger


def write_summary(ledger: list[dict[str, Any]]) -> None:
    lane_counts = Counter(row["branch_extra_lane"] for row in ledger)
    route_counts = Counter(row["route"] for row in ledger)
    row_counts = Counter((row["hillslope_id"], row["window"], row["route"]) for row in ledger)
    lines = [
        "# HPHYS0308 Snowd Branch State-Ordering Summary",
        "",
        "Ran:",
        "",
        f"- Fixed comparator commit: `{FIXED_COMMIT}`",
        f"- Branch-extra key rows: `{len(ledger)}`",
        f"- Production edit authorized rows: `{sum(1 for row in ledger if row['production_edit_authorized'])}`",
        "",
        "## Lane Counts",
        "",
    ]
    lines.extend(f"- `{key}`: `{lane_counts[key]}`" for key in sorted(lane_counts))
    lines.extend(["", "## Route Counts", ""])
    lines.extend(f"- `{key}`: `{route_counts[key]}`" for key in sorted(route_counts))
    lines.extend(
        [
            "",
            "## Window Route Counts",
            "",
            "| Hill | Window | Route | Keys |",
            "| --- | --- | --- | ---: |",
        ]
    )
    for (hill, window, route), count in sorted(row_counts.items()):
        lines.append(f"| H{hill} | {window} | {route} | {count} |")
    (ARTIFACT_DIR / "snowd-branch-state-ordering-summary.md").write_text(
        "\n".join(lines) + "\n",
        encoding="utf-8",
    )


def write_method() -> None:
    (ARTIFACT_DIR / "snowd-branch-state-ordering-method.md").write_text(
        """# HPHYS0308 Snowd Branch State-Ordering Method

Ran:

- Loaded HPHYS0306 `branch-active-melt-term-ledger.json` to recover target
  window years and Julian-day bounds.
- Loaded HPHYS0307 `melt-call-branch-activation-ledger.json` to restrict this
  package to baseline-extra/openWEPP-extra branch activation rows.
- Parsed HPHYS0305 fixed-baseline observe logs for `H305_T_*`, `H305_F_*`,
  `H305_S_OUT`, and `H305_M_POST` records.
- Parsed openWEPP final `post_wb13` trace rows for branch-active, snow
  depth/density, forcing, and melt-term hourly maps.
- Rebuilt branch-extra key sets from paired active masks and emitted key-level
  route classifications.

Static:

- Classification is evidence-only; no production edit is authorized by this
  package.
""",
        encoding="utf-8",
    )


def write_source_lineage() -> None:
    (ARTIFACT_DIR / "snowd-branch-state-ordering-source-lineage.md").write_text(
        """# HPHYS0308 Snowd Branch State-Ordering Source Lineage

Static:

- Fixed comparator commit: `47ac4c32faeea81bb99081f955a14c38b815ef4d`
- Baseline source: `/workdir/wepp-forest_260430_baseline/src/snowd.for:116-193`
- openWEPP source: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3889-3949`

## Baseline Predicate

- `/workdir/wepp-forest_260430_baseline/src/snowd.for:116` selects the
  freezing/non-freezing daily mean branch using `(tmax + tmin)/2`.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:180-193` enters the
  non-freezing branch, computes `snodep`, requires `snodep .gt. 0.0`, and
  executes `call melt(irtype,wrain,hour)`.

## openWEPP Predicate and State Surfaces

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3889-3936`
  routes no-snow, freezing, and non-freezing snowpack branches.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3937-3949`
  invokes `compute_simimpl29_melt_hour` and sets `melt_branch_active = 1.0`.
- HPHYS0308 evidence reads `snow_hourly_depth_before_m`,
  `snow_hourly_snowfall_depth_m`, `snow_hourly_depth_available_m`,
  `snow_hourly_depth_after_m`, and density/forcing maps at each branch-extra
  key.

## Closure Rule

- Baseline-extra keys with openWEPP zero depth surfaces are
  `snow-state-carry-depletion-hold`, not branch-predicate edit authority.
- openWEPP-extra keys without fixed-baseline `melt.for` observations are
  `baseline-branch-instrumentation-hold`.
- Any production branch-predicate edit requires direct source-line evidence
  beyond aggregate active-mask counts.
""",
        encoding="utf-8",
    )


def main() -> None:
    ledger = build_ledger()
    write_json(ARTIFACT_DIR / "snowd-branch-state-ordering-ledger.json", ledger)
    write_summary(ledger)
    write_method()
    write_source_lineage()
    print(
        json.dumps(
            {
                "rows": len(ledger),
                "lane_counts": Counter(row["branch_extra_lane"] for row in ledger),
                "route_counts": Counter(row["route"] for row in ledger),
            },
            sort_keys=True,
            default=dict,
        )
    )


if __name__ == "__main__":
    main()
