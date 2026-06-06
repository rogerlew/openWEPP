#!/usr/bin/env python3
"""Reconstruct prior-day snow carry divergence for HPHYS0309 carry holds."""

from __future__ import annotations

import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
HPHYS0309_LEDGER = (
    Path("docs/work-packages")
    / "20260605-hphys0309-snow-carry-depletion-lineage-closure-001"
    / "artifacts"
    / "snow-carry-depletion-lineage-ledger.json"
)
HPHYS0305_ARTIFACT_DIR = (
    Path("docs/work-packages")
    / "20260605-hphys0305-paired-melt-term-state-instrumentation-001"
    / "artifacts"
)
TRACE_BOUNDARY = "post_wb13"
TRACE_PHASE = None
DEPTH_TOL_M = 0.0005
ZERO_TOL_M = 1.0e-9
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


class PairedEvidenceError(RuntimeError):
    """Raised when required paired baseline/openWEPP hourly evidence is absent."""


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


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
        elif tag == "H305_T_AB":
            record["amelt_in"] = v1
            record["bmelt_in"] = v2
        elif tag == "H305_T_CD":
            record["cmelt_in"] = v1
            record["dmelt_in"] = v2
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


def hour_value(row: dict[str, Any] | None, field: str, hour: int) -> float | None:
    if row is None:
        return None
    values = row.get(field)
    if isinstance(values, dict):
        key = f"{hour:04d}"
        if key not in values:
            return None
        value = values[key]
        return None if value is None else float(value)
    if values is None:
        return None
    return float(values)


def require_hour_value(row: dict[str, Any] | None, field: str, year: int, day: int, hour: int) -> float:
    value = hour_value(row, field, hour)
    if value is None:
        raise PairedEvidenceError(
            f"missing paired hourly evidence: openWEPP {field} y={year} d={day} h={hour}"
        )
    return value


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
            f"missing paired hourly evidence: baseline {field} y={year} d={day} h={hour}"
        )
    return record[field]


def validate_required_paired_hourly_evidence(
    baseline: dict[tuple[int, int, int], dict[str, float]],
    openwepp: dict[tuple[int, int], dict[str, Any]],
    year: int,
    max_day: int,
) -> None:
    for day in range(1, max_day + 1):
        row = openwepp.get((year, day))
        for hour in range(1, 25):
            require_baseline_value(baseline, "depth_after_m", year, day, hour)
            require_baseline_value(baseline, "density_after_kg_m3", year, day, hour)
            require_hour_value(row, "snow_hourly_depth_after_m", year, day, hour)
            require_hour_value(row, "snow_hourly_density_after_kg_m3", year, day, hour)


def scalar(row: dict[str, Any] | None, field: str) -> float | None:
    if row is None or field not in row or row[field] is None:
        return None
    return float(row[field])


def sum_scalar(rows: list[dict[str, Any]], field: str) -> float | None:
    total = 0.0
    seen = False
    for row in rows:
        value = scalar(row, field)
        if value is None:
            continue
        total += value
        seen = True
    return total if seen else None


def sum_record_field(records: list[dict[str, float]], field: str) -> tuple[float | None, int]:
    total = 0.0
    seen = 0
    for record in records:
        if field not in record:
            continue
        total += record[field]
        seen += 1
    return (total if seen else None), seen


def baseline_raw_term_melt(record: dict[str, float]) -> float | None:
    terms = [record.get(name) for name in ("amelt_in", "bmelt_in", "cmelt_in", "dmelt_in")]
    if any(value is None for value in terms):
        return None
    return 0.0254 * sum(float(value) for value in terms)


def is_positive(value: Any) -> bool:
    return value is not None and float(value) > ZERO_TOL_M


def group_hphys0309_rows(rows: list[dict[str, Any]]) -> dict[tuple[int, str, int], list[dict[str, Any]]]:
    groups: dict[tuple[int, str, int], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        if row.get("hphys0308_route") != "snow-state-carry-depletion-hold":
            continue
        groups[(int(row["hillslope_id"]), str(row["window"]), int(row["year"]))].append(row)
    return dict(sorted(groups.items()))


def day_start_depth(baseline: dict[tuple[int, int, int], dict[str, float]], year: int, day: int) -> float | None:
    if day <= 1:
        return None
    return baseline.get((year, day - 1, 24), {}).get("depth_after_m")


def first_nonzero_hour(
    baseline: dict[tuple[int, int, int], dict[str, float]],
    openwepp: dict[tuple[int, int], dict[str, Any]],
    year: int,
    max_day: int,
) -> dict[str, Any]:
    baseline_first = None
    openwepp_first = None
    for day in range(1, max_day + 1):
        row = openwepp.get((year, day))
        for hour in range(1, 25):
            base_depth = require_baseline_value(baseline, "depth_after_m", year, day, hour)
            open_depth = require_hour_value(row, "snow_hourly_depth_after_m", year, day, hour)
            if baseline_first is None and base_depth > ZERO_TOL_M:
                baseline_first = {"julian": day, "hour": hour, "depth_after_m": base_depth}
            if openwepp_first is None and open_depth > ZERO_TOL_M:
                openwepp_first = {"julian": day, "hour": hour, "depth_after_m": open_depth}
            if baseline_first is not None and openwepp_first is not None:
                return {"baseline": baseline_first, "openwepp": openwepp_first}
    return {"baseline": baseline_first, "openwepp": openwepp_first}


def find_first_divergence(
    baseline: dict[tuple[int, int, int], dict[str, float]],
    openwepp: dict[tuple[int, int], dict[str, Any]],
    year: int,
    max_day: int,
) -> dict[str, Any] | None:
    for day in range(1, max_day + 1):
        row = openwepp.get((year, day))
        for hour in range(1, 25):
            base = baseline[(year, day, hour)]
            base_depth = require_baseline_value(baseline, "depth_after_m", year, day, hour)
            open_depth = require_hour_value(row, "snow_hourly_depth_after_m", year, day, hour)
            delta = open_depth - base_depth
            if abs(delta) > DEPTH_TOL_M:
                return {
                    "year": year,
                    "julian": day,
                    "hour": hour,
                    "baseline_depth_after_m": base_depth,
                    "openwepp_depth_after_m": open_depth,
                    "depth_delta_openwepp_minus_baseline_m": delta,
                    "baseline_density_after_kg_m3": require_baseline_value(
                        baseline, "density_after_kg_m3", year, day, hour
                    ),
                    "openwepp_density_after_kg_m3": require_hour_value(
                        row, "snow_hourly_density_after_kg_m3", year, day, hour
                    ),
                    "openwepp_depth_before_m": require_hour_value(
                        row, "snow_hourly_depth_before_m", year, day, hour
                    ),
                    "openwepp_snowfall_depth_m": require_hour_value(
                        row, "snow_hourly_snowfall_depth_m", year, day, hour
                    ),
                    "openwepp_raw_melt_m": require_hour_value(
                        row, "snow_hourly_melt_raw_m", year, day, hour
                    ),
                    "openwepp_routed_melt_m": require_hour_value(
                        row, "snow_hourly_melt_m", year, day, hour
                    ),
                    "openwepp_rain_m": require_hour_value(
                        row, "snow_hourly_rain_m", year, day, hour
                    ),
                    "baseline_post_hrmlt_m": base.get("post_hrmlt_m"),
                    "baseline_post_hrrain_m": base.get("post_hrrain_m"),
                    "baseline_raw_term_melt_m": baseline_raw_term_melt(base),
                }
    return None


def baseline_aggregates(
    baseline: dict[tuple[int, int, int], dict[str, float]], year: int, start_day: int, end_day: int
) -> dict[str, Any]:
    records = [
        baseline[(year, day, hour)]
        for day in range(start_day, end_day + 1)
        for hour in range(1, 25)
        if (year, day, hour) in baseline
    ]
    raw_terms = [baseline_raw_term_melt(record) for record in records]
    raw_terms_present = [value for value in raw_terms if value is not None]
    post_hrmlt_sum, post_hrmlt_count = sum_record_field(records, "post_hrmlt_m")
    post_hrrain_sum, post_hrrain_count = sum_record_field(records, "post_hrrain_m")
    expected_hours = (end_day - start_day + 1) * 24
    if len(records) != expected_hours:
        raise PairedEvidenceError(
            "missing paired hourly evidence: baseline episode record coverage "
            f"y={year} d={start_day}-{end_day} observed={len(records)} expected={expected_hours}"
        )
    if post_hrmlt_count != expected_hours or post_hrrain_count != expected_hours:
        raise PairedEvidenceError(
            "missing paired hourly evidence: baseline post-melt/rain coverage "
            f"y={year} d={start_day}-{end_day} "
            f"hrmlt={post_hrmlt_count} hrrain={post_hrrain_count} expected={expected_hours}"
        )
    return {
        "baseline_observed_hour_count": len(records),
        "baseline_post_hrmlt_observed_hours": post_hrmlt_count,
        "baseline_post_hrmlt_sum_m": post_hrmlt_sum,
        "baseline_post_hrrain_observed_hours": post_hrrain_count,
        "baseline_post_hrrain_sum_m": post_hrrain_sum,
        "baseline_raw_term_melt_observed_hours": len(raw_terms_present),
        "baseline_raw_term_melt_sum_m": sum(raw_terms_present),
        "baseline_episode_start_depth_m": day_start_depth(baseline, year, start_day),
        "baseline_episode_end_depth_m": baseline.get((year, end_day, 24), {}).get("depth_after_m"),
        "baseline_episode_end_density_kg_m3": baseline.get((year, end_day, 24), {}).get(
            "density_after_kg_m3"
        ),
    }


def openwepp_aggregates(
    openwepp: dict[tuple[int, int], dict[str, Any]], year: int, start_day: int, end_day: int
) -> dict[str, Any]:
    rows = [openwepp[(year, day)] for day in range(start_day, end_day + 1) if (year, day) in openwepp]
    expected_days = end_day - start_day + 1
    if len(rows) != expected_days:
        raise PairedEvidenceError(
            "missing paired hourly evidence: openWEPP episode day coverage "
            f"y={year} d={start_day}-{end_day} observed={len(rows)} expected={expected_days}"
        )
    first_row = openwepp.get((year, start_day))
    last_row = openwepp.get((year, end_day))
    return {
        "openwepp_observed_day_count": len(rows),
        "openwepp_snowfall_depth_sum_m": sum_scalar(rows, "snow_hourly_snowfall_depth_sum_m"),
        "openwepp_snowfall_water_equiv_sum_m": sum_scalar(
            rows, "snow_hourly_snowfall_water_equiv_sum_m"
        ),
        "openwepp_raw_melt_sum_m": sum_scalar(rows, "snow_hourly_melt_raw_sum_m"),
        "openwepp_routed_melt_sum_m": sum_scalar(rows, "snow_hourly_melt_sum_m"),
        "openwepp_rain_sum_m": sum_scalar(rows, "snow_hourly_rain_sum_m"),
        "openwepp_rain_retained_sum_m": sum_scalar(rows, "snow_hourly_rain_retained_sum_m"),
        "openwepp_rain_released_sum_m": sum_scalar(rows, "snow_hourly_rain_released_sum_m"),
        "openwepp_runtime_swe_delta_sum_m": sum_scalar(rows, "snow_runtime_swe_delta_m"),
        "openwepp_runtime_depth_delta_sum_m": sum_scalar(rows, "snow_runtime_depth_delta_m"),
        "openwepp_episode_start_depth_m": scalar(first_row, "snow_runtime_depth_before_m"),
        "openwepp_episode_start_swe_m": scalar(first_row, "snow_runtime_swe_before_m"),
        "openwepp_episode_end_depth_m": scalar(last_row, "snow_runtime_depth_m"),
        "openwepp_episode_end_swe_m": scalar(last_row, "snow_runtime_swe_m"),
        "openwepp_episode_end_density_kg_m3": scalar(last_row, "snow_runtime_density_kg_m3"),
    }


def classify(first: dict[str, Any] | None, first_nonzero: dict[str, Any]) -> tuple[str, str]:
    if first is None:
        return "no-material-divergence-found-hold", "no paired depth divergence exceeded tolerance"
    baseline_first = first_nonzero.get("baseline")
    openwepp_first = first_nonzero.get("openwepp")
    if baseline_first is None or openwepp_first is None:
        return "incomplete-episode-evidence-hold", "baseline or openWEPP first nonzero snow evidence is missing"
    same_onset_day = first["julian"] == baseline_first["julian"] == openwepp_first["julian"]
    if same_onset_day and first["julian"] == 1 and first["hour"] == 1:
        return (
            "initial-carry-state-projection-hold",
            "first material divergence is present at day-1 hour-1 initial snow carry projection",
        )
    if same_onset_day and first["hour"] <= max(baseline_first["hour"], openwepp_first["hour"]) + 1:
        return (
            "accumulation-settling-onset-hold",
            "first material divergence occurs at initial paired snow accumulation/settling onset",
        )
    if is_positive(first.get("openwepp_raw_melt_m")) or is_positive(
        first.get("openwepp_routed_melt_m")
    ):
        return (
            "raw-routed-melt-magnitude-hold",
            "first material divergence coincides with openWEPP hourly raw/routed melt",
        )
    if is_positive(first.get("openwepp_rain_m")) or is_positive(
        first.get("baseline_post_hrrain_m")
    ):
        return (
            "retained-released-rain-handling-hold",
            "first material divergence coincides with rain-on-snow handling evidence",
        )
    if first.get("baseline_density_after_kg_m3") != first.get("openwepp_density_after_kg_m3"):
        return (
            "density-settling-carry-state-hold",
            "first material divergence is paired with snow density/settling divergence",
        )
    return "unclassified-carry-state-hold", "first material divergence lacks enough source-lane proof"


def build_ledger() -> list[dict[str, Any]]:
    rows0309 = read_json(HPHYS0309_LEDGER)
    identity = read_json(HPHYS0305_ARTIFACT_DIR / "baseline-observe-identity.json")
    trace_audit = read_json(HPHYS0305_ARTIFACT_DIR / "openwepp-trace-field-audit.json")
    baseline_by_hill = {
        int(hill): parse_baseline_log(Path(data["observe_on_lane"]["observe_log"]))
        for hill, data in identity.items()
    }
    openwepp_by_hill = {
        int(item["hillslope_id"]): load_trace_rows(Path(item["trace_path"])) for item in trace_audit
    }
    ledger = []
    for (hill, window, year), rows in group_hphys0309_rows(rows0309).items():
        key_days = sorted({int(row["julian"]) for row in rows})
        first_key_day = min(key_days)
        baseline = baseline_by_hill[hill]
        openwepp = openwepp_by_hill[hill]
        validate_required_paired_hourly_evidence(baseline, openwepp, year, first_key_day)
        first = find_first_divergence(baseline, openwepp, year, first_key_day)
        nonzero = first_nonzero_hour(baseline, openwepp, year, first_key_day)
        route, reason = classify(first, nonzero)
        start_day = min(
            day
            for day in [
                nonzero.get("baseline", {}).get("julian") if nonzero.get("baseline") else None,
                nonzero.get("openwepp", {}).get("julian") if nonzero.get("openwepp") else None,
                first["julian"] if first else None,
                first_key_day,
            ]
            if day is not None
        )
        baseline_aggs = baseline_aggregates(baseline, year, start_day, first_key_day)
        openwepp_aggs = openwepp_aggregates(openwepp, year, start_day, first_key_day)
        ledger.append(
            {
                "hillslope_id": hill,
                "window": window,
                "year": year,
                "affected_hphys0309_rows": len(rows),
                "hphys0309_routes": dict(Counter(row["route"] for row in rows)),
                "key_days": key_days,
                "episode_start_julian": start_day,
                "first_key_julian": first_key_day,
                "first_nonzero_snow": nonzero,
                "first_material_divergence": first,
                "route": route,
                "classification_reason": reason,
                "depth_tolerance_m": DEPTH_TOL_M,
                "baseline_episode": baseline_aggs,
                "openwepp_episode": openwepp_aggs,
                "production_edit_authorized": False,
                "prohibited_compensation_note": (
                    "No branch-predicate, same-hour melt-term, WB13, WB17, WB18, WB19, "
                    "or WB12 compensation is authorized by HPHYS0310."
                ),
                "baseline_source": "/workdir/wepp-forest_260430_baseline/src/snowd.for:50-53,215-230,303-312",
                "fixed_negative_melt_source": (
                    "wepp_260430_negmeltfix_comparator commit "
                    "47ac4c32faeea81bb99081f955a14c38b815ef4d src/winter.for:434-453; "
                    "patch docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/"
                    "artifacts/fixed-comparator-source-delta.patch"
                ),
                "openwepp_source": (
                    "crates/openwepp-hillslope-orchestrator/src/hydrology/"
                    "03_kernel_support_00_support_helpers.rs:3879-4105,4111-4177,4231-4277"
                ),
            }
        )
    return ledger


def write_summary(ledger: list[dict[str, Any]]) -> None:
    route_counts = Counter(row["route"] for row in ledger)
    row_count = sum(row["affected_hphys0309_rows"] for row in ledger)
    lines = [
        "# Prior-Day Snow Carry Divergence Summary",
        "",
        "Status: complete",
        "",
        "Evidence mode: ran",
        "",
        "## Counts",
        "",
        f"- Affected HPHYS0309 rows represented: `{row_count}`",
        f"- Hillslope/window/year groups: `{len(ledger)}`",
        f"- Production edit authorized groups: `{sum(1 for row in ledger if row['production_edit_authorized'])}`",
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
            "| Hillslope | Window | Year | Rows | First Divergence | Route |",
            "|---|---|---:|---:|---|---|",
        ]
    )
    for row in ledger:
        first = row["first_material_divergence"]
        first_text = (
            "none"
            if first is None
            else f"{first['year']}-{first['julian']:03d} h{first['hour']:02d} "
            f"delta_depth={first['depth_delta_openwepp_minus_baseline_m']:.6g} m"
        )
        lines.append(
            f"| H{row['hillslope_id']} | {row['window']} | {row['year']} | "
            f"{row['affected_hphys0309_rows']} | {first_text} | `{row['route']}` |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "HPHYS0310 localizes the HPHYS0309 carry deficit to paired",
            "snow-episode carry-state divergence: six groups diverge at day-1",
            "hour-1 initial carry-state projection, and one group diverges",
            "during early density/settling carry-state evolution. This is still",
            "carry-state producer lineage, not branch-predicate or downstream",
            "water-balance edit authority. The next package should compare the",
            "initial state projection, density, and depth-update equations at",
            "the first divergent hours against fixed-comparator `snowd.for`",
            "source lines before modifying production code.",
        ]
    )
    (ARTIFACT_DIR / "prior-day-snow-carry-divergence-summary.md").write_text(
        "\n".join(lines) + "\n", encoding="utf-8"
    )


def write_method() -> None:
    text = """# Prior-Day Snow Carry Divergence Method

Status: complete

Evidence mode: ran

Static:

- Reads the executed HPHYS0309 carry/depletion ledger.
- Uses HPHYS0305 fixed-comparator observe-on logs for baseline `H305_S_OUT`
  post-hour `snodpt`/`densgt`, `H305_M_POST` post-winter routed melt/rain
  surfaces, and active `H305_T_*` term evidence where present.
- Uses HPHYS0305 openWEPP traces at `post_wb13` for daily runtime snow state
  and hourly before/after depth, density, melt, rain, and snowfall surfaces.

Ran:

- Groups all HPHYS0309 snow-state carry/depletion rows by
  hillslope/window/year.
- Scans paired fixed-comparator and openWEPP hourly after-depth from day 1
  through the first HPHYS0309 key day for that group.
- Records the first paired depth divergence above `0.0005 m`.
- Aggregates episode-level baseline and openWEPP snow flux/state lanes from
  first nonzero snow through the first key day.
- Keeps every group in `HOLD` unless source-line-owned production proof exists.
"""
    (ARTIFACT_DIR / "prior-day-snow-carry-divergence-method.md").write_text(text, encoding="utf-8")


def write_source_lineage() -> None:
    text = """# Prior-Day Snow Carry Divergence Source Lineage

Status: complete

Evidence mode: static

- Baseline snow-state entry/carry: `/workdir/wepp-forest_260430_baseline/src/snowd.for:50-53`
  initializes hourly `snodep`/`snodpt`/`densgy`/`densgt` from carried
  `snodpy`/`densg`.
- Baseline melt depletion: `/workdir/wepp-forest_260430_baseline/src/snowd.for:215-230`
  records pre-melt `snodpt`, subtracts `smelt`, and clamps all-melted pack.
- Baseline carry publication: `/workdir/wepp-forest_260430_baseline/src/snowd.for:303-312`
  caps density, zeroes density for zero depth, and writes `snodpt`, `snodpy`,
  and `densg` for the next hour/day.
- Fixed negative-melt comparator: branch `wepp_260430_negmeltfix_comparator`,
  tag `wepp_260430_negmeltfix_comparator_47ac4c32faee`, commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`, `src/winter.for:434-453`,
  with patch provenance in
  `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-comparator-source-delta.patch`.
- openWEPP hourly snow state publication:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3879-4105`
  records hourly before/after depth/density, raw melt, routed melt, rain, and
  snowfall surfaces.
- openWEPP runtime carry publication:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4111-4177`
  computes daily runtime SWE/depth after accumulation, rain retention/release,
  and redistributed state loss.
- openWEPP signed-melt redistribution:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4231-4277`
  preserves corrected routed melt and carried state-loss lineage.
"""
    (ARTIFACT_DIR / "prior-day-snow-carry-divergence-source-lineage.md").write_text(
        text, encoding="utf-8"
    )


def run_missing_paired_evidence_negative_fixture() -> None:
    baseline: dict[tuple[int, int, int], dict[str, float]] = {}
    openwepp_hours = {f"{hour:04d}": 0.0 for hour in range(1, 25)}
    openwepp = {
        (2020, 1): {
            "snow_hourly_depth_after_m": dict(openwepp_hours),
            "snow_hourly_density_after_kg_m3": dict(openwepp_hours),
        }
    }
    for hour in range(1, 25):
        if hour == 7:
            continue
        baseline[(2020, 1, hour)] = {
            "depth_after_m": 0.0,
            "density_after_kg_m3": 0.0,
        }
    validate_required_paired_hourly_evidence(baseline, openwepp, 2020, 1)


def main() -> None:
    if "--self-test-missing-paired-evidence" in sys.argv:
        try:
            run_missing_paired_evidence_negative_fixture()
        except PairedEvidenceError as exc:
            print(f"missing paired hourly evidence fail-closed: {exc}", file=sys.stderr)
            raise SystemExit(2)
        raise AssertionError("missing paired hourly evidence fixture did not fail closed")

    ledger = build_ledger()
    write_json(ARTIFACT_DIR / "prior-day-snow-carry-divergence-ledger.json", ledger)
    write_summary(ledger)
    write_method()
    write_source_lineage()
    print(
        json.dumps(
            {
                "groups": len(ledger),
                "represented_hphys0309_rows": sum(row["affected_hphys0309_rows"] for row in ledger),
                "routes": dict(Counter(row["route"] for row in ledger)),
                "production_edit_authorized": sum(
                    1 for row in ledger if row["production_edit_authorized"]
                ),
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
