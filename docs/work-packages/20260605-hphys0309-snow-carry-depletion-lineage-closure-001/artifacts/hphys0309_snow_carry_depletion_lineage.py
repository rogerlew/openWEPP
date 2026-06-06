#!/usr/bin/env python3
"""Classify snow carry/depletion lineage for HPHYS0308 baseline-extra keys."""

from __future__ import annotations

import json
import re
from collections import Counter
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
HPHYS0308_LEDGER = (
    REPO
    / "docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/artifacts/snowd-branch-state-ordering-ledger.json"
)
HPHYS0305_ARTIFACT_DIR = (
    REPO
    / "docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts"
)
TRACE_BOUNDARY = "post_wb13"
TRACE_PHASE = None
ZERO_TOL = 1.0e-12
DEFICIT_TOL_M = 1.0e-6

OBS_RE = re.compile(
    r"^(?P<tag>\S+)\s+y=\s*(?P<year>-?\d+)\s+d=\s*(?P<day>-?\d+)"
    r"\s+e=\s*(?P<element>-?\d+)\s+c=\s*(?P<chan>-?\d+)"
    r"\s+s=\s*(?P<hour>-?\d+)\s+v1=\s*(?P<v1>[-+0-9.Ee]+)"
    r"\s+v2=\s*(?P<v2>[-+0-9.Ee]+)"
)


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def parse_baseline_log(path: Path) -> dict[tuple[int, int, int], dict[str, float]]:
    parsed: dict[tuple[int, int, int], dict[str, float]] = {}
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
        key = f"{hour:04d}"
        if key not in values:
            return None
        value = values[key]
        return None if value is None else float(value)
    if values is None:
        return None
    return float(values)


def first_zero_hour(depths: dict[int, float | None]) -> int | None:
    for hour in range(1, 25):
        value = depths.get(hour)
        if value is not None and value <= ZERO_TOL:
            return hour
    return None


def previous_day_key(year: int, julian: int) -> tuple[int, int]:
    return year, julian - 1


def classify_row(
    baseline_day_start_m: float | None,
    openwepp_day_start_m: float | None,
    baseline_zero_hour: int | None,
    openwepp_zero_hour: int | None,
) -> tuple[str, str, bool]:
    if baseline_day_start_m is None or openwepp_day_start_m is None:
        return (
            "incomplete-carry-state-evidence-hold",
            "baseline or openWEPP day-start carry state is missing",
            False,
        )
    if openwepp_zero_hour is None:
        return (
            "incomplete-carry-state-evidence-hold",
            "openWEPP same-day hourly depth-after zero evidence is missing",
            False,
        )
    if openwepp_day_start_m <= ZERO_TOL and baseline_day_start_m > DEFICIT_TOL_M:
        return (
            "prior-day-openwepp-meltout-hold",
            "openWEPP starts the key day snow-free while fixed baseline carries snow from the prior day",
            False,
        )
    if openwepp_day_start_m + DEFICIT_TOL_M < baseline_day_start_m:
        return (
            "pre-day-carry-deficit-hold",
            "openWEPP starts the key day with materially less carried snow depth than fixed baseline",
            False,
        )
    if (
        baseline_zero_hour is not None
        and openwepp_zero_hour is not None
        and openwepp_zero_hour < baseline_zero_hour
    ):
        return (
            "same-day-depletion-lead-hold",
            "openWEPP depletes snow earlier within the same day despite comparable day-start state",
            False,
        )
    return (
        "state-ordering-unresolved-hold",
        "available carry-state evidence does not yet explain the branch-extra key",
        False,
    )


def depletion_lead_evidence_state(
    baseline_zero_hour: int | None, openwepp_zero_hour: int | None
) -> str:
    if baseline_zero_hour is None and openwepp_zero_hour is None:
        return "not-computable-no-same-day-zero"
    if baseline_zero_hour is None:
        return "not-computable-baseline-no-same-day-zero"
    if openwepp_zero_hour is None:
        return "not-computable-openwepp-no-same-day-zero"
    return "computed"


def build_ledger() -> list[dict[str, Any]]:
    hphys0308_rows = [
        row
        for row in read_json(HPHYS0308_LEDGER)
        if row.get("route") == "snow-state-carry-depletion-hold"
    ]
    identity = read_json(HPHYS0305_ARTIFACT_DIR / "baseline-observe-identity.json")
    trace_audit = read_json(HPHYS0305_ARTIFACT_DIR / "openwepp-trace-field-audit.json")
    baseline_by_hill = {
        int(hill): parse_baseline_log(Path(data["observe_on_lane"]["observe_log"]))
        for hill, data in identity.items()
    }
    openwepp_by_hill = {
        int(item["hillslope_id"]): load_trace_rows(Path(item["trace_path"]))
        for item in trace_audit
    }

    ledger: list[dict[str, Any]] = []
    for row in hphys0308_rows:
        hill = int(row["hillslope_id"])
        year = int(row["year"])
        julian = int(row["julian"])
        hour = int(row["hour"])
        baseline = baseline_by_hill[hill]
        openwepp = openwepp_by_hill[hill]
        openwepp_day = openwepp.get((year, julian), {})
        previous_year, previous_julian = previous_day_key(year, julian)
        previous_openwepp_day = openwepp.get((previous_year, previous_julian), {})

        baseline_day_depth_after = {
            h: baseline.get((year, julian, h), {}).get("depth_after_m")
            for h in range(1, 25)
        }
        openwepp_day_depth_after = {
            h: hour_value(openwepp_day, "snow_hourly_depth_after_m", h)
            for h in range(1, 25)
        }
        baseline_zero_hour = first_zero_hour(baseline_day_depth_after)
        openwepp_zero_hour = first_zero_hour(openwepp_day_depth_after)
        baseline_day_start_m = baseline.get((previous_year, previous_julian, 24), {}).get(
            "depth_after_m"
        )
        openwepp_day_start_m = (
            None
            if "snow_runtime_depth_before_m" not in openwepp_day
            else float(openwepp_day["snow_runtime_depth_before_m"])
        )
        openwepp_prior_hour_after_m = (
            hour_value(openwepp_day, "snow_hourly_depth_after_m", hour - 1)
            if hour > 1
            else hour_value(previous_openwepp_day, "snow_hourly_depth_after_m", 24)
        )
        baseline_prior_hour_after_m = (
            baseline.get((year, julian, hour - 1), {}).get("depth_after_m")
            if hour > 1
            else baseline_day_start_m
        )
        route, reason, production_authorized = classify_row(
            baseline_day_start_m,
            openwepp_day_start_m,
            baseline_zero_hour,
            openwepp_zero_hour,
        )
        lead_hours = (
            None
            if baseline_zero_hour is None or openwepp_zero_hour is None
            else baseline_zero_hour - openwepp_zero_hour
        )
        lead_evidence_state = depletion_lead_evidence_state(
            baseline_zero_hour, openwepp_zero_hour
        )
        baseline_key_state = baseline.get((year, julian, hour), {})
        ledger.append(
            {
                "hillslope_id": hill,
                "window": row["window"],
                "year": year,
                "julian": julian,
                "hour": hour,
                "hphys0308_route": row["route"],
                "route": route,
                "classification_reason": reason,
                "baseline_day_start_depth_m": baseline_day_start_m,
                "openwepp_day_start_depth_m": openwepp_day_start_m,
                "day_start_depth_delta_openwepp_minus_baseline_m": (
                    None
                    if baseline_day_start_m is None or openwepp_day_start_m is None
                    else openwepp_day_start_m - baseline_day_start_m
                ),
                "baseline_prior_hour_after_depth_m": baseline_prior_hour_after_m,
                "openwepp_prior_hour_after_depth_m": openwepp_prior_hour_after_m,
                "baseline_key_depth_after_m": baseline_key_state.get("depth_after_m"),
                "openwepp_key_depth_before_m": hour_value(
                    openwepp_day, "snow_hourly_depth_before_m", hour
                ),
                "openwepp_key_depth_after_m": hour_value(
                    openwepp_day, "snow_hourly_depth_after_m", hour
                ),
                "baseline_zero_hour": baseline_zero_hour,
                "openwepp_zero_hour": openwepp_zero_hour,
                "openwepp_depletion_lead_hours": lead_hours,
                "depletion_lead_evidence_state": lead_evidence_state,
                "baseline_day_depth_after_m": baseline_day_depth_after,
                "openwepp_day_depth_after_m": openwepp_day_depth_after,
                "openwepp_day_raw_melt_sum_m": openwepp_day.get("snow_hourly_melt_raw_sum_m"),
                "openwepp_day_routed_melt_sum_m": openwepp_day.get("snow_hourly_melt_sum_m"),
                "openwepp_day_runtime_depth_after_m": openwepp_day.get("snow_runtime_depth_m"),
                "openwepp_day_runtime_swe_before_m": openwepp_day.get("snow_runtime_swe_before_m"),
                "openwepp_day_runtime_swe_after_m": openwepp_day.get("snow_runtime_swe_m"),
                "baseline_source": "/workdir/wepp-forest_260430_baseline/src/snowd.for:50-53,215-230,303-312",
                "fixed_negative_melt_source": "wepp_260430_negmeltfix_comparator commit 47ac4c32faeea81bb99081f955a14c38b815ef4d src/winter.for:434-453; patch docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-comparator-source-delta.patch",
                "openwepp_source": "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3879-4105,4111-4177,4231-4277",
                "production_edit_authorized": production_authorized,
                "prohibited_compensation_note": "No WB13/WB17/WB18/WB19/WB12 compensation is authorized.",
            }
        )
    return ledger


def write_summary(ledger: list[dict[str, Any]]) -> None:
    route_counts = Counter(row["route"] for row in ledger)
    by_window = Counter((row["hillslope_id"], row["window"], row["route"]) for row in ledger)
    lead_state_counts = Counter(row["depletion_lead_evidence_state"] for row in ledger)
    lead_counts = Counter(
        row["openwepp_depletion_lead_hours"]
        for row in ledger
        if row["openwepp_depletion_lead_hours"] is not None
    )
    production_count = sum(1 for row in ledger if row["production_edit_authorized"])
    lines = [
        "# Snow Carry/Depletion Lineage Summary",
        "",
        "Status: complete",
        "",
        "Evidence mode: ran",
        "",
        "## Counts",
        "",
        f"- HPHYS0308 snow-state carry/depletion rows: `{len(ledger)}`",
        f"- Production edit authorized rows: `{production_count}`",
        "",
        "## Route Counts",
        "",
    ]
    for route, count in sorted(route_counts.items()):
        lines.append(f"- `{route}`: `{count}`")
    lines.extend(["", "## Depletion Lead Evidence State", ""])
    for state, count in sorted(lead_state_counts.items()):
        lines.append(f"- `{state}`: `{count}`")
    lines.extend(["", "## OpenWEPP Depletion Lead Hours", ""])
    for lead, count in sorted(lead_counts.items()):
        lines.append(f"- `{lead}`: `{count}`")
    lines.extend(["", "## Window Route Counts", "", "| Hillslope | Window | Route | Count |", "|---|---|---|---|"])
    for (hill, window, route), count in sorted(by_window.items()):
        lines.append(f"| H{hill} | {window} | {route} | {count} |")
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The HPHYS0308 baseline-extra melt-call keys are immediate carry-state",
            "deficits, not branch-predicate edit authority. Most rows start the key",
            "day with materially less openWEPP snow depth than the fixed comparator;",
            "the remaining rows start the key day snow-free in openWEPP while the",
            "fixed comparator still carries snow from the prior day. The package",
            "therefore keeps production edits in `HOLD` and routes continuation to",
            "the prior-day/day-start snowpack carry-state lineage.",
            "",
        ]
    )
    (ARTIFACT_DIR / "snow-carry-depletion-lineage-summary.md").write_text(
        "\n".join(lines), encoding="utf-8"
    )


def write_method() -> None:
    text = """# Snow Carry/Depletion Lineage Method

Status: complete

Evidence mode: ran

Static:

- Reads HPHYS0308 branch-extra state-ordering ledger.
- Uses HPHYS0305 fixed-comparator observe-on logs for baseline
  `H305_S_OUT` post-hour `snodpt`/`densgt` and `H305_M_POST` routed melt/rain
  surfaces.
- Uses HPHYS0305 openWEPP traces at `post_wb13` for daily runtime and hourly
  snow depth/SWE surfaces.

Ran:

- Filters HPHYS0308 rows to `snow-state-carry-depletion-hold`.
- Compares fixed-comparator prior-day hour-24 depth to openWEPP
  `snow_runtime_depth_before_m` on the key day.
- Finds the first same-day zero after-hour snow depth for baseline and
  openWEPP and records openWEPP depletion lead hours.
- Keeps every row in `HOLD` unless source-line-owned production proof exists.
"""
    (ARTIFACT_DIR / "snow-carry-depletion-lineage-method.md").write_text(
        text, encoding="utf-8"
    )


def write_source_lineage() -> None:
    text = """# Snow Carry/Depletion Source Lineage

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
  This compares net daily melt, scales positive routed melt by
  `1 + ngtvML/pstvML`, and applies the companion carried-depth adjustment.
- openWEPP hourly snow state publication:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3879-4105`
  records hourly before/after depth/density and branch-active surfaces.
- openWEPP runtime carry publication:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4111-4177`
  computes daily runtime SWE/depth after redistributed state loss.
- openWEPP signed-melt redistribution:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4231-4277`
  preserves corrected routed melt and carried state-loss lineage.
"""
    (ARTIFACT_DIR / "snow-carry-depletion-lineage-source-lineage.md").write_text(
        text, encoding="utf-8"
    )


def main() -> None:
    ledger = build_ledger()
    write_json(ARTIFACT_DIR / "snow-carry-depletion-lineage-ledger.json", ledger)
    write_summary(ledger)
    write_method()
    write_source_lineage()
    print(
        json.dumps(
            {
                "rows": len(ledger),
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
