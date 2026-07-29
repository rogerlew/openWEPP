#!/usr/bin/env python3
"""Execute the bounded CAL-07D transition-chronology attribution screen."""

from __future__ import annotations

import csv
import hashlib
import math
import statistics
from collections import defaultdict, deque
from datetime import date, timedelta
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
ART = PKG / "artifacts"
CAL07 = (
    ROOT
    / "docs/work-packages"
    / "20260728-canopy-cal-07-southern-hemisphere-robustness-001"
)
CAL07C = (
    ROOT
    / "docs/work-packages"
    / "20260728-canopy-cal-07c-hourly-vpd-forcing-reconstruction-001"
)
SOURCE_DAILY = CAL07 / "inputs/source/bezamahafaly_DB_1000_1day.csv"
SOURCE_TRANSITIONS = (
    CAL07
    / "inputs/source/bezamahafaly_DB_1000_simplified_transition_dates.csv"
)
MODEL_LEVELS = (0.10, 0.20, 0.25, 0.30, 0.40, 0.50, 0.60, 0.70, 0.75, 0.80, 0.90)
SOURCE_LEVELS = (0.10, 0.25, 0.50)
ENSEMBLE_SCENARIOS = (
    "BASE",
    "TEMPERATURE_UNCONSTRAINED",
    "VPD_UNCONSTRAINED",
    "PHOTOPERIOD_UNCONSTRAINED",
    "PHOTOPERIOD_AND_VPD_UNCONSTRAINED",
)
DEFAULT_SCENARIO = "SC_PLANT_GENERALIZED_DEFAULT"


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def comment_csv(path: Path) -> list[dict[str, str]]:
    lines = [
        line
        for line in path.read_text(encoding="utf-8").splitlines()
        if not line.startswith("#")
    ]
    return list(csv.DictReader(lines))


def write_csv(path: Path, fields: tuple[str, ...], data: list[dict[str, object]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(data)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def quantile(values: list[float], fraction: float) -> float:
    ordered = sorted(values)
    position = (len(ordered) - 1) * fraction
    lower = math.floor(position)
    upper = math.ceil(position)
    return ordered[lower] + (position - lower) * (ordered[upper] - ordered[lower])


def increasing(value: float, inactive: float, unconstrained: float) -> float:
    if value <= inactive:
        return 0.0
    if value >= unconstrained:
        return 1.0
    return (value - inactive) / (unconstrained - inactive)


def photoperiod_hours(latitude_degrees: float, doy: int) -> float:
    latitude = math.radians(latitude_degrees)
    declination = 0.409 * math.sin((2.0 * math.pi * doy / 365.0) - 1.39)
    sunset_cosine = -math.tan(latitude) * math.tan(declination)
    sunset_angle = math.acos(min(1.0, max(-1.0, sunset_cosine)))
    return 24.0 * sunset_angle / math.pi


def scenario_parameters(member: dict[str, str], scenario: str) -> dict[str, float]:
    if scenario == DEFAULT_SCENARIO:
        return {
            "tmin_inactive": -2.0,
            "tmin_unconstrained": 5.0,
            "vpd_unconstrained": 900.0,
            "vpd_inactive": 4100.0,
            "photo_inactive": 10.0,
            "photo_unconstrained": 11.0,
        }
    return {
        "tmin_inactive": float(member["minimum_temperature_inactive_c"]),
        "tmin_unconstrained": float(member["minimum_temperature_unconstrained_c"]),
        "vpd_unconstrained": float(member["vapor_pressure_deficit_unconstrained_pa"]),
        "vpd_inactive": float(member["vapor_pressure_deficit_inactive_pa"]),
        "photo_inactive": float(member["photoperiod_inactive_hours"]),
        "photo_unconstrained": float(member["photoperiod_unconstrained_hours"]),
    }


def reconstruct(
    forcing: list[dict[str, str]], member: dict[str, str], scenario: str
) -> list[dict[str, object]]:
    parameters = scenario_parameters(member, scenario)
    history: deque[float] = deque(maxlen=21)
    result: list[dict[str, object]] = []
    previous_day: date | None = None
    for forcing_row in forcing:
        day = date.fromisoformat(forcing_row["date"])
        if previous_day is not None and day != previous_day + timedelta(days=1):
            raise ValueError(f"nonconsecutive forcing at {day}")
        previous_day = day
        tmin = float(forcing_row["tmin_c"])
        vpd = float(forcing_row["vpd_pa"])
        photo_hours = photoperiod_hours(
            float(forcing_row["latitude_degrees"]), int(forcing_row["doy"])
        )
        i_tmin = increasing(
            tmin, parameters["tmin_inactive"], parameters["tmin_unconstrained"]
        )
        i_vpd = 1.0 - increasing(
            vpd, parameters["vpd_unconstrained"], parameters["vpd_inactive"]
        )
        i_photo = increasing(
            photo_hours,
            parameters["photo_inactive"],
            parameters["photo_unconstrained"],
        )
        if scenario == "TEMPERATURE_UNCONSTRAINED":
            i_tmin = 1.0
        elif scenario == "VPD_UNCONSTRAINED":
            i_vpd = 1.0
        elif scenario == "PHOTOPERIOD_UNCONSTRAINED":
            i_photo = 1.0
        elif scenario == "PHOTOPERIOD_AND_VPD_UNCONSTRAINED":
            i_photo = 1.0
            i_vpd = 1.0
        instantaneous = i_tmin * i_vpd * i_photo
        history.append(instantaneous)
        gsi21 = statistics.fmean(history)
        minimum = min(i_tmin, i_vpd, i_photo)
        tied = sorted(
            name
            for name, value in (
                ("TEMPERATURE", i_tmin),
                ("VPD", i_vpd),
                ("PHOTOPERIOD", i_photo),
            )
            if value == minimum
        )
        result.append(
            {
                "date": day.isoformat(),
                "year": day.year,
                "doy": day.timetuple().tm_yday,
                "tmin_c": tmin,
                "vpd_pa": vpd,
                "photoperiod_hours": photo_hours,
                "i_tmin": i_tmin,
                "i_vpd": i_vpd,
                "i_photo": i_photo,
                "instantaneous_gsi": instantaneous,
                "gsi21": gsi21,
                "sample_count": len(history),
                "minimum_constraint_tie_set": "+".join(tied),
            }
        )
    return result


def crossing_inventory(
    trajectory: list[dict[str, object]], threshold: float
) -> list[tuple[str, float]]:
    found: list[tuple[str, float]] = []
    for previous, current in zip(trajectory, trajectory[1:]):
        old = float(previous["gsi21"])
        new = float(current["gsi21"])
        direction = ""
        if old < threshold <= new:
            direction = "rising"
        elif old >= threshold > new:
            direction = "falling"
        if direction:
            fraction = (threshold - old) / (new - old)
            found.append(
                (
                    direction,
                    date.fromisoformat(str(previous["date"])).toordinal() + fraction,
                )
            )
    return found


def event_source_rows() -> list[dict[str, object]]:
    source = comment_csv(SOURCE_TRANSITIONS)
    events: list[dict[str, object]] = []
    for row in source:
        event = {
            "event_id": f"{row['year']}-{row['direction']}",
            "year": int(row["year"]),
            "direction": row["direction"],
        }
        for level, source_name in ((0.10, "10"), (0.25, "25"), (0.50, "50")):
            event[f"date_{source_name}"] = f"{row['year']}-{row[f'date_{source_name}']}"
            event[f"doy_{source_name}"] = int(row[f"DOY_{source_name}"])
        events.append(event)
    events.sort(key=lambda item: str(item["date_50"]))
    for index, event in enumerate(events):
        event["eligible"] = 0 < index < len(events) - 1
        if event["eligible"]:
            event["lower_ordinal"] = 0.5 * (
                date.fromisoformat(str(events[index - 1]["date_50"])).toordinal()
                + date.fromisoformat(str(event["date_50"])).toordinal()
            )
            event["upper_ordinal"] = 0.5 * (
                date.fromisoformat(str(event["date_50"])).toordinal()
                + date.fromisoformat(str(events[index + 1]["date_50"])).toordinal()
            )
    return events


def fixed_relative_threshold(
    trajectory: list[dict[str, object]], event_year: int, level: float
) -> float | None:
    values = [
        float(row["gsi21"]) for row in trajectory if int(row["year"]) == event_year
    ]
    lower = min(values)
    upper = max(values)
    if lower == upper:
        return None
    return lower + level * (upper - lower)


def pair_event(
    crossings: list[tuple[str, float]], event: dict[str, object], observed: date
) -> dict[str, object]:
    direction = str(event["direction"])
    same_direction = [value for found_direction, value in crossings if found_direction == direction]
    in_window = [
        value
        for value in same_direction
        if float(event["lower_ordinal"]) < value <= float(event["upper_ordinal"])
    ]
    selected = in_window[0] if in_window else None
    return {
        "matched": bool(selected is not None),
        "selected_crossing_ordinal": "" if selected is None else f"{selected:.9f}",
        "selected_crossing_date": (
            "" if selected is None else date.fromordinal(math.floor(selected)).isoformat()
        ),
        "residual_days": (
            "" if selected is None else f"{selected - observed.toordinal():.9f}"
        ),
        "global_same_direction_count": len(same_direction),
        "in_window_count": len(in_window),
        "extra_in_window_count": max(0, len(in_window) - 1),
        "out_of_window_count": len(same_direction) - len(in_window),
    }


def add_crossings(
    output: list[dict[str, object]],
    scenario: str,
    member: str,
    event: dict[str, object],
    operator: str,
    level: float,
    threshold: float | None,
    crossings: list[tuple[str, float]],
) -> None:
    if threshold is None:
        return
    for sequence, (direction, value) in enumerate(crossings, start=1):
        output.append(
            {
                "scenario": scenario,
                "member_or_default": member,
                "event_id": event["event_id"],
                "operator": operator,
                "model_level": f"{level:.2f}",
                "threshold_gsi21": f"{threshold:.17g}",
                "direction": direction,
                "crossing_sequence": sequence,
                "crossing_ordinal": f"{value:.9f}",
                "crossing_date": date.fromordinal(math.floor(value)).isoformat(),
                "inside_event_window": (
                    float(event["lower_ordinal"])
                    < value
                    <= float(event["upper_ordinal"])
                ),
            }
        )


def event_row(
    scenario: str,
    member: str,
    event: dict[str, object],
    operator: str,
    level: float,
    threshold: float | None,
    observed: date,
    crossings: list[tuple[str, float]],
    trajectory: list[dict[str, object]],
) -> dict[str, object]:
    paired = (
        {
            "matched": False,
            "selected_crossing_ordinal": "",
            "selected_crossing_date": "",
            "residual_days": "",
            "global_same_direction_count": 0,
            "in_window_count": 0,
            "extra_in_window_count": 0,
            "out_of_window_count": 0,
        }
        if threshold is None
        else pair_event(crossings, event, observed)
    )
    by_date = {str(row["date"]): row for row in trajectory}
    event_state = by_date[observed.isoformat()]
    return {
        "scenario": scenario,
        "member_or_default": member,
        "event_id": event["event_id"],
        "event_year": event["year"],
        "direction": event["direction"],
        "operator": operator,
        "model_level": f"{level:.2f}",
        "observed_date": observed.isoformat(),
        "threshold_gsi21": "" if threshold is None else f"{threshold:.17g}",
        "event_date_gsi21": f"{float(event_state['gsi21']):.17g}",
        **paired,
    }


EVENT_FIELDS = (
    "scenario",
    "member_or_default",
    "event_id",
    "event_year",
    "direction",
    "operator",
    "model_level",
    "observed_date",
    "threshold_gsi21",
    "event_date_gsi21",
    "matched",
    "selected_crossing_ordinal",
    "selected_crossing_date",
    "residual_days",
    "global_same_direction_count",
    "in_window_count",
    "extra_in_window_count",
    "out_of_window_count",
)


def source_support(events: list[dict[str, object]]) -> list[dict[str, object]]:
    source_daily = {
        row["date"]: row for row in comment_csv(SOURCE_DAILY)
    }
    output: list[dict[str, object]] = []
    for event in events:
        if not event["eligible"]:
            continue
        for level, suffix in ((0.10, "10"), (0.25, "25"), (0.50, "50")):
            event_date = date.fromisoformat(str(event[f"date_{suffix}"]))
            support = []
            for offset in range(-21, 22):
                row = source_daily.get((event_date + timedelta(days=offset)).isoformat())
                if row is not None:
                    support.append(row)
            accepted = [
                row
                for row in support
                if row["image_count"] != "NA"
                and int(row["image_count"]) > 0
                and row["gcc_90"] != "NA"
                and row["outlierflag_gcc_90"] == "0"
            ]
            event_daily = source_daily[event_date.isoformat()]
            output.append(
                {
                    "event_id": event["event_id"],
                    "direction": event["direction"],
                    "source_level": f"{level:.2f}",
                    "source_date": event_date.isoformat(),
                    "source_doy": event[f"doy_{suffix}"],
                    "window_calendar_days": 43,
                    "accepted_raw_gcc90_days": len(accepted),
                    "missing_or_rejected_raw_days": 43 - len(accepted),
                    "source_smooth_gcc90": event_daily["smooth_gcc_90"],
                    "source_smooth_ci_width": event_daily["smooth_ci_gcc_90"],
                    "event_day_image_count": event_daily["image_count"],
                    "event_day_raw_gcc90": event_daily["gcc_90"],
                    "event_day_outlier_flag": event_daily["outlierflag_gcc_90"],
                    "source_span_is_confidence_interval": False,
                }
            )
    return output


def summarize_daily(
    scenario: str, scenario_daily: dict[str, list[dict[str, object]]]
) -> list[dict[str, object]]:
    output: list[dict[str, object]] = []
    dates = [str(row["date"]) for row in next(iter(scenario_daily.values()))]
    for index, day in enumerate(dates):
        values = [trajectory[index] for trajectory in scenario_daily.values()]
        row: dict[str, object] = {
            "scenario": scenario,
            "date": day,
            "year": values[0]["year"],
            "doy": values[0]["doy"],
            "members": len(values),
            "tmin_c": f"{float(values[0]['tmin_c']):.9f}",
            "vpd_pa": f"{float(values[0]['vpd_pa']):.9f}",
            "photoperiod_hours": f"{float(values[0]['photoperiod_hours']):.9f}",
        }
        for field in ("i_tmin", "i_vpd", "i_photo", "instantaneous_gsi", "gsi21"):
            field_values = [float(value[field]) for value in values]
            row[f"{field}_p05"] = f"{quantile(field_values, 0.05):.9f}"
            row[f"{field}_median"] = f"{statistics.median(field_values):.9f}"
            row[f"{field}_p95"] = f"{quantile(field_values, 0.95):.9f}"
        tie_counts: dict[str, int] = defaultdict(int)
        for value in values:
            tie_counts[str(value["minimum_constraint_tie_set"])] += 1
        row["minimum_constraint_tie_counts"] = ";".join(
            f"{key}:{tie_counts[key]}" for key in sorted(tie_counts)
        )
        output.append(row)
    return output


def main() -> None:
    ART.mkdir(exist_ok=True)
    members = rows(CAL07C / "inputs/ensemble.csv")
    forcing = [
        row
        for row in rows(CAL07C / "inputs/forcing.csv")
        if row["site_id"] == "SH-DB-BEZA"
    ]
    kernel = [
        row
        for row in rows(CAL07C / "artifacts/daily-kernel-output.csv")
        if row["site_id"] == "SH-DB-BEZA"
    ]
    kernel_map = {
        (row["candidate_id"], row["date"]): float(row["gsi"]) for row in kernel
    }
    if len(members) != 37 or len(forcing) != 1666 or len(kernel) != 61642:
        raise ValueError("frozen CAL-07C inventory mismatch")

    dependencies = (
        CAL07C / "inputs/ensemble.csv",
        CAL07C / "inputs/forcing.csv",
        CAL07C / "inputs/observations.csv",
        CAL07C / "inputs/transitions.csv",
        CAL07C / "artifacts/daily-kernel-output.csv",
        CAL07C / "artifacts/transition-residuals.csv",
        SOURCE_DAILY,
        SOURCE_TRANSITIONS,
        ROOT / "docs/specifications/science-contracts/contracts/SC-PLANT-001.md",
        ROOT / "docs/decisions/0042-science-implementation-and-calibration-readiness.md",
    )
    write_csv(
        ART / "dependency-manifest.csv",
        ("path", "sha256", "bytes", "source_commit"),
        [
            {
                "path": path.relative_to(ROOT),
                "sha256": sha(path),
                "bytes": path.stat().st_size,
                "source_commit": "11b1faab37b5d365b0c0c7051ed32a4762821239",
            }
            for path in dependencies
        ],
    )

    events = event_source_rows()
    eligible = [event for event in events if event["eligible"]]
    support = source_support(events)
    write_csv(
        ART / "observation-support.csv",
        tuple(support[0].keys()),
        support,
    )

    base_daily_rows: list[dict[str, object]] = []
    daily_summaries: list[dict[str, object]] = []
    all_crossings: list[dict[str, object]] = []
    absolute_rows: list[dict[str, object]] = []
    sensitivity_rows: list[dict[str, object]] = []
    source_level_rows: list[dict[str, object]] = []
    scenario_rows: list[dict[str, object]] = []
    base_trajectories: dict[str, list[dict[str, object]]] = {}

    for scenario in ENSEMBLE_SCENARIOS:
        scenario_daily: dict[str, list[dict[str, object]]] = {}
        for member in members:
            member_id = member["candidate_id"]
            trajectory = reconstruct(forcing, member, scenario)
            scenario_daily[member_id] = trajectory
            if scenario == "BASE":
                base_trajectories[member_id] = trajectory
                for row in trajectory:
                    expected = kernel_map[(member_id, str(row["date"]))]
                    residual = float(row["gsi21"]) - expected
                    base_daily_rows.append(
                        {
                            "candidate_id": member_id,
                            **row,
                            "cal07c_gsi21": f"{expected:.17g}",
                            "reconstruction_residual": f"{residual:.17g}",
                        }
                    )
            for event in eligible:
                observed_50 = date.fromisoformat(str(event["date_50"]))
                operators = (("ABSOLUTE_0_5", 0.50), ("EVENT_YEAR_RELATIVE", 0.50))
                for operator, level in operators:
                    threshold = (
                        0.5
                        if operator == "ABSOLUTE_0_5"
                        else fixed_relative_threshold(
                            trajectory, int(event["year"]), level
                        )
                    )
                    crossings = (
                        [] if threshold is None else crossing_inventory(trajectory, threshold)
                    )
                    scenario_rows.append(
                        event_row(
                            scenario,
                            member_id,
                            event,
                            operator,
                            level,
                            threshold,
                            observed_50,
                            crossings,
                            trajectory,
                        )
                    )
                    add_crossings(
                        all_crossings,
                        scenario,
                        member_id,
                        event,
                        operator,
                        level,
                        threshold,
                        crossings,
                    )
                if scenario != "BASE":
                    continue
                absolute = scenario_rows[-2]
                absolute_rows.append(
                    {
                        **absolute,
                        "cal07c_expected_observed_date": event["date_50"],
                    }
                )
                for level in MODEL_LEVELS:
                    threshold = fixed_relative_threshold(
                        trajectory, int(event["year"]), level
                    )
                    crossings = (
                        [] if threshold is None else crossing_inventory(trajectory, threshold)
                    )
                    sensitivity_rows.append(
                        event_row(
                            scenario,
                            member_id,
                            event,
                            "EVENT_YEAR_RELATIVE",
                            level,
                            threshold,
                            observed_50,
                            crossings,
                            trajectory,
                        )
                    )
                    if level != 0.50:
                        add_crossings(
                            all_crossings,
                            scenario,
                            member_id,
                            event,
                            "EVENT_YEAR_RELATIVE",
                            level,
                            threshold,
                            crossings,
                        )
                for source_level, suffix in (
                    (0.10, "10"),
                    (0.25, "25"),
                    (0.50, "50"),
                ):
                    observed = date.fromisoformat(str(event[f"date_{suffix}"]))
                    threshold = fixed_relative_threshold(
                        trajectory, int(event["year"]), source_level
                    )
                    crossings = (
                        [] if threshold is None else crossing_inventory(trajectory, threshold)
                    )
                    source_level_rows.append(
                        {
                            **event_row(
                                scenario,
                                member_id,
                                event,
                                "SOURCE_LEVEL_ANALOGY",
                                source_level,
                                threshold,
                                observed,
                                crossings,
                                trajectory,
                            ),
                            "source_level": f"{source_level:.2f}",
                            "source_level_span_is_confidence_interval": False,
                        }
                    )
        daily_summaries.extend(summarize_daily(scenario, scenario_daily))

    default_member = {
        "candidate_id": "SC_PLANT_GENERALIZED_DEFAULT",
        "minimum_temperature_inactive_c": "-2",
        "minimum_temperature_unconstrained_c": "5",
        "vapor_pressure_deficit_unconstrained_pa": "900",
        "vapor_pressure_deficit_inactive_pa": "4100",
        "photoperiod_inactive_hours": "10",
        "photoperiod_unconstrained_hours": "11",
    }
    default_trajectory = reconstruct(forcing, default_member, DEFAULT_SCENARIO)
    daily_summaries.extend(
        summarize_daily(DEFAULT_SCENARIO, {"DEFAULT": default_trajectory})
    )
    for event in eligible:
        observed = date.fromisoformat(str(event["date_50"]))
        for operator, level in (("ABSOLUTE_0_5", 0.50), ("EVENT_YEAR_RELATIVE", 0.50)):
            threshold = (
                0.5
                if operator == "ABSOLUTE_0_5"
                else fixed_relative_threshold(
                    default_trajectory, int(event["year"]), level
                )
            )
            crossings = (
                [] if threshold is None else crossing_inventory(default_trajectory, threshold)
            )
            scenario_rows.append(
                event_row(
                    DEFAULT_SCENARIO,
                    "DEFAULT",
                    event,
                    operator,
                    level,
                    threshold,
                    observed,
                    crossings,
                    default_trajectory,
                )
            )
            add_crossings(
                all_crossings,
                DEFAULT_SCENARIO,
                "DEFAULT",
                event,
                operator,
                level,
                threshold,
                crossings,
            )

    base_fields = tuple(base_daily_rows[0].keys())
    write_csv(ART / "base-member-daily.csv", base_fields, base_daily_rows)
    write_csv(
        ART / "daily-scenario-ensemble.csv",
        tuple(daily_summaries[0].keys()),
        daily_summaries,
    )
    write_csv(
        ART / "all-crossings.csv",
        tuple(all_crossings[0].keys()),
        all_crossings,
    )
    write_csv(
        ART / "absolute-reproduction.csv",
        (*EVENT_FIELDS, "cal07c_expected_observed_date"),
        absolute_rows,
    )
    write_csv(ART / "model-level-sensitivity.csv", EVENT_FIELDS, sensitivity_rows)
    write_csv(
        ART / "source-level-audit.csv",
        (*EVENT_FIELDS, "source_level", "source_level_span_is_confidence_interval"),
        source_level_rows,
    )
    write_csv(ART / "scenario-event-screen.csv", EVENT_FIELDS, scenario_rows)

    forcing_map = {row["date"]: row for row in forcing}
    support_map = {
        (row["event_id"], row["source_level"]): row for row in support
    }
    attribution: list[dict[str, object]] = []
    for event in eligible:
        for source_level, suffix in ((0.10, "10"), (0.25, "25"), (0.50, "50")):
            day = str(event[f"date_{suffix}"])
            index = next(
                index
                for index, row in enumerate(next(iter(base_trajectories.values())))
                if row["date"] == day
            )
            event_values = [trajectory[index] for trajectory in base_trajectories.values()]
            start = max(0, index - 20)
            preceding = {
                field: [
                    statistics.fmean(
                        float(trajectory[position][field])
                        for position in range(start, index + 1)
                    )
                    for trajectory in base_trajectories.values()
                ]
                for field in ("i_tmin", "i_vpd", "i_photo", "instantaneous_gsi")
            }
            ties: dict[str, int] = defaultdict(int)
            for value in event_values:
                ties[str(value["minimum_constraint_tie_set"])] += 1
            forcing_row = forcing_map[day]
            support_row = support_map[(str(event["event_id"]), f"{source_level:.2f}")]
            result: dict[str, object] = {
                "event_id": event["event_id"],
                "direction": event["direction"],
                "source_level": f"{source_level:.2f}",
                "source_date": day,
                "tmin_c": forcing_row["tmin_c"],
                "vpd_pa": forcing_row["vpd_pa"],
                "photoperiod_hours": f"{float(event_values[0]['photoperiod_hours']):.9f}",
                "members": 37,
                "minimum_constraint_tie_counts": ";".join(
                    f"{key}:{ties[key]}" for key in sorted(ties)
                ),
                "accepted_raw_gcc90_days_pm21": support_row["accepted_raw_gcc90_days"],
                "source_smooth_gcc90": support_row["source_smooth_gcc90"],
                "source_smooth_ci_width": support_row["source_smooth_ci_width"],
            }
            for field in ("i_tmin", "i_vpd", "i_photo", "instantaneous_gsi", "gsi21"):
                values = [float(value[field]) for value in event_values]
                result[f"{field}_p05"] = f"{quantile(values, 0.05):.9f}"
                result[f"{field}_median"] = f"{statistics.median(values):.9f}"
                result[f"{field}_p95"] = f"{quantile(values, 0.95):.9f}"
            for field, values in preceding.items():
                result[f"preceding21_{field}_median"] = (
                    f"{statistics.median(values):.9f}"
                )
            attribution.append(result)
    write_csv(
        ART / "event-indicator-attribution.csv",
        tuple(attribution[0].keys()),
        attribution,
    )

    absolute_map = {
        (row["member_or_default"], row["event_id"]): row for row in absolute_rows
    }
    newly_matched = sum(
        str(row["matched"]) == "True"
        and str(absolute_map[(row["member_or_default"], row["event_id"])]["matched"])
        == "False"
        for row in source_level_rows
    )
    decision_rows: list[dict[str, object]] = [
        {
            "hypothesis": "OBSERVATION_SCALE",
            "status": (
                "SUPPORTED_AS_CONTRIBUTOR"
                if newly_matched > 0
                else "NOT_SUPPORTED_BY_SCREEN"
            ),
            "predicate_value": newly_matched,
            "predicate": "absolute-unmatched member/events matched by a source-level-aligned BASE operator",
            "claim_ceiling": "scale sensitivity only; GCC and GSI states are not validated as equivalent",
        }
    ]
    scenario_map = {
        (
            row["scenario"],
            row["member_or_default"],
            row["event_id"],
            row["operator"],
        ): row
        for row in scenario_rows
    }
    contributor_scenarios = (
        ("TEMPERATURE", "TEMPERATURE_UNCONSTRAINED"),
        ("VPD", "VPD_UNCONSTRAINED"),
        ("PHOTOPERIOD", "PHOTOPERIOD_UNCONSTRAINED"),
    )
    any_match_increase = False
    for label, scenario in contributor_scenarios:
        changed = 0
        base_matches = 0
        scenario_matches = 0
        for member in (row["candidate_id"] for row in members):
            for event in eligible:
                for operator in ("ABSOLUTE_0_5", "EVENT_YEAR_RELATIVE"):
                    base = scenario_map[("BASE", member, event["event_id"], operator)]
                    alternate = scenario_map[(scenario, member, event["event_id"], operator)]
                    base_matches += int(bool(base["matched"]))
                    scenario_matches += int(bool(alternate["matched"]))
                    if (
                        float(base["event_date_gsi21"])
                        != float(alternate["event_date_gsi21"])
                        or bool(base["matched"]) != bool(alternate["matched"])
                        or base["selected_crossing_ordinal"]
                        != alternate["selected_crossing_ordinal"]
                    ):
                        changed += 1
        any_match_increase |= scenario_matches > base_matches
        decision_rows.append(
            {
                "hypothesis": f"{label}_CONSTRAINT",
                "status": (
                    "SUPPORTED_AS_MATHEMATICAL_CONTRIBUTOR"
                    if changed > 0
                    else "NOT_SUPPORTED_BY_SCREEN"
                ),
                "predicate_value": changed,
                "predicate": "member/event/operator rows with changed event state, match availability, or crossing",
                "claim_ceiling": "indicator substitution is attribution-only and does not show biological error",
            }
        )
    decision_rows.append(
        {
            "hypothesis": "CURRENT_GSI_CONSTRAINT_SENSITIVITY",
            "status": (
                "PLAUSIBLE_UNRESOLVED"
                if any_match_increase
                else "NOT_SUPPORTED_BY_SCREEN"
            ),
            "predicate_value": int(any_match_increase),
            "predicate": "any single-indicator scenario increases matched rows under the same operator",
            "claim_ceiling": "does not identify thresholds or separate forcing, physiology, and missing cues",
        }
    )
    decision_rows.append(
        {
            "hypothesis": "FORCING_LIMITATION",
            "status": "PLAUSIBLE_UNRESOLVED",
            "predicate_value": 1,
            "predicate": "Beza forcing is gridded POWER and no on-site meteorology is admitted",
            "claim_ceiling": "no forcing-bias magnitude or correction is identified",
        }
    )
    rising_attribution = [
        row for row in attribution if row["direction"] == "rising" and row["source_level"] == "0.50"
    ]
    missing_process_signal = any(
        float(row["instantaneous_gsi_median"]) < 0.5
        and sum(
            float(row[field]) < 0.5
            for field in ("i_tmin_median", "i_vpd_median", "i_photo_median")
        )
        >= 2
        for row in rising_attribution
    )
    decision_rows.append(
        {
            "hypothesis": "MISSING_PROCESS",
            "status": (
                "PLAUSIBLE_UNRESOLVED"
                if missing_process_signal
                else "NOT_SUPPORTED_BY_SCREEN"
            ),
            "predicate_value": int(missing_process_signal),
            "predicate": "source rising date with median instantaneous GSI <0.5 and at least two median indicators <0.5",
            "claim_ceiling": "compatible with forcing bias and current threshold transfer; no new process is authorized",
        }
    )
    write_csv(
        ART / "decision-screen.csv",
        ("hypothesis", "status", "predicate_value", "predicate", "claim_ceiling"),
        decision_rows,
    )

    evidence_rows = [
        {
            "solution_route": "FORCING_BIAS",
            "minimum_discriminating_evidence": "quality-controlled on-site Tmin, humidity/VPD, and precipitation over complete leaf-off and leaf-on seasons",
            "current_status": "MISSING",
            "next_authority_boundary": "observed-data admission before forcing adjudication",
        },
        {
            "solution_route": "OBSERVATION_SEMANTICS",
            "minimum_discriminating_evidence": "raw image/ROI review, fit metadata and method, transition uncertainty or field phenology corroboration",
            "current_status": "PARTIAL",
            "next_authority_boundary": "source-method and site-observation admission",
        },
        {
            "solution_route": "PARAMETER_OR_ECOTYPE_TRANSFER",
            "minimum_discriminating_evidence": "tropical dry-forest threshold authority plus an independently reserved site or held-out years",
            "current_status": "MISSING",
            "next_authority_boundary": "separate calibration/validation package under ADR-0042",
        },
        {
            "solution_route": "MISSING_PROCESS",
            "minimum_discriminating_evidence": "rainfall, soil-water or plant-water-status observations plus site/ecotype literature establishing the cue",
            "current_status": "MISSING",
            "next_authority_boundary": "contract-first process package only after science-authority admission",
        },
    ]
    write_csv(
        ART / "additional-evidence-needed.csv",
        tuple(evidence_rows[0].keys()),
        evidence_rows,
    )

    prior = rows(CAL07C / "artifacts/transition-residuals.csv")
    if len(absolute_rows) != 148 or len(sensitivity_rows) != 1628:
        raise ValueError("event result inventory mismatch")
    if len(source_level_rows) != 444 or len(scenario_rows) != 1488:
        raise ValueError("expanded event result inventory mismatch")
    prior_map = {
        (row["candidate_id"], row["year"], row["direction"]): row for row in prior
    }
    for row in absolute_rows:
        expected = prior_map[
            (
                row["member_or_default"],
                str(row["event_year"]),
                row["direction"],
            )
        ]
        actual_residual = row["residual_days"]
        expected_residual = expected["residual_days"]
        if bool(actual_residual) != bool(expected_residual):
            raise ValueError("CAL-07C crossing availability mismatch")
        if actual_residual and abs(float(actual_residual) - float(expected_residual)) > 5e-7:
            raise ValueError("CAL-07C residual mismatch")
        if int(row["in_window_count"]) != int(expected["same_direction_crossing_count"]):
            raise ValueError("CAL-07C crossing count mismatch")

    print(
        "CAL-07D analysis PASS: "
        f"{len(base_daily_rows)} BASE rows; "
        f"{sum(bool(row['matched']) for row in absolute_rows)} absolute matches; "
        f"{newly_matched} source-level newly matched rows"
    )


if __name__ == "__main__":
    main()
