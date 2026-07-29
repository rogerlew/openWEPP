#!/usr/bin/env python3
"""Independently validate CAL-07D source, equations, results, and claims."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import statistics
import xml.etree.ElementTree as ET
from collections import Counter, defaultdict
from datetime import date, timedelta
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
ART = PKG / "artifacts"
FIG = ART / "figures"
CAL07C = (
    ROOT
    / "docs/work-packages"
    / "20260728-canopy-cal-07c-hourly-vpd-forcing-reconstruction-001"
)
CAL07 = (
    ROOT
    / "docs/work-packages"
    / "20260728-canopy-cal-07-southern-hemisphere-robustness-001"
)
SOURCE_DAILY = CAL07 / "inputs/source/bezamahafaly_DB_1000_1day.csv"
SOURCE_TRANSITIONS = (
    CAL07
    / "inputs/source/bezamahafaly_DB_1000_simplified_transition_dates.csv"
)
LEVELS = (0.10, 0.20, 0.25, 0.30, 0.40, 0.50, 0.60, 0.70, 0.75, 0.80, 0.90)
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


def comment_rows(path: Path) -> list[dict[str, str]]:
    content = [
        line
        for line in path.read_text(encoding="utf-8").splitlines()
        if not line.startswith("#")
    ]
    return list(csv.DictReader(content))


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def up(value: float, lower: float, upper: float) -> float:
    return max(0.0, min(1.0, (value - lower) / (upper - lower)))


def daylight(latitude_degrees: float, doy: int) -> float:
    declination = 0.409 * math.sin(2.0 * math.pi * doy / 365.0 - 1.39)
    argument = -math.tan(math.radians(latitude_degrees)) * math.tan(declination)
    return 24.0 * math.acos(max(-1.0, min(1.0, argument))) / math.pi


def percentile(values: list[float], fraction: float) -> float:
    ordered = sorted(values)
    position = (len(ordered) - 1) * fraction
    lower = math.floor(position)
    upper = math.ceil(position)
    return ordered[lower] + (position - lower) * (
        ordered[upper] - ordered[lower]
    )


def same_value(actual: str, expected: object, tolerance: float = 5.1e-9) -> None:
    text = str(expected)
    if actual == text:
        return
    try:
        assert abs(float(actual) - float(text)) <= tolerance, (actual, text)
    except ValueError:
        assert actual == text, (actual, text)


def compare_records(
    actual: list[dict[str, str]],
    expected: list[dict[str, object]],
    key_fields: tuple[str, ...],
    tolerance: float = 5.1e-9,
) -> None:
    actual_map = {
        tuple(row[field] for field in key_fields): row for row in actual
    }
    expected_map = {
        tuple(str(row[field]) for field in key_fields): row for row in expected
    }
    assert len(actual_map) == len(actual) == len(expected_map) == len(expected)
    assert actual_map.keys() == expected_map.keys()
    for key, expected_row in expected_map.items():
        actual_row = actual_map[key]
        assert actual_row.keys() == expected_row.keys(), key
        for field, expected_value in expected_row.items():
            same_value(actual_row[field], expected_value, tolerance)


def check_manifest() -> None:
    dependency = rows(ART / "dependency-manifest.csv")
    assert len(dependency) == 10
    for item in dependency:
        path = ROOT / item["path"]
        assert path.is_file(), path
        assert digest(path) == item["sha256"], path
        assert path.stat().st_size == int(item["bytes"])
        assert item["source_commit"] == "11b1faab37b5d365b0c0c7051ed32a4762821239"
    expected_source = {
        "docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/inputs/source/bezamahafaly_DB_1000_1day.csv":
            "a490b29758ce0608428c6e794d8c803727b60fddc4e601c875564a26ed514f1f",
        "docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/inputs/source/bezamahafaly_DB_1000_simplified_transition_dates.csv":
            "db477b36731d0a8c072ac400dac3aa135e84234408d79a1a6a10eded739632cd",
    }
    manifest_map = {item["path"]: item["sha256"] for item in dependency}
    assert all(manifest_map[path] == value for path, value in expected_source.items())


def independent_base_reconstruction() -> float:
    members = rows(CAL07C / "inputs/ensemble.csv")
    forcing = [
        row
        for row in rows(CAL07C / "inputs/forcing.csv")
        if row["site_id"] == "SH-DB-BEZA"
    ]
    published = rows(ART / "base-member-daily.csv")
    assert len(members) == 37
    assert len(forcing) == 1666
    assert len(published) == 61642
    assert len({(row["candidate_id"], row["date"]) for row in published}) == 61642
    published_map = {
        (row["candidate_id"], row["date"]): row for row in published
    }
    forcing_dates = [date.fromisoformat(row["date"]) for row in forcing]
    assert all(
        current == previous + timedelta(days=1)
        for previous, current in zip(forcing_dates, forcing_dates[1:])
    )
    maximum = 0.0
    for member in members:
        history: list[float] = []
        for index, forcing_row in enumerate(forcing):
            tmin = float(forcing_row["tmin_c"])
            vpd = float(forcing_row["vpd_pa"])
            photo = daylight(
                float(forcing_row["latitude_degrees"]), int(forcing_row["doy"])
            )
            i_tmin = up(
                tmin,
                float(member["minimum_temperature_inactive_c"]),
                float(member["minimum_temperature_unconstrained_c"]),
            )
            i_vpd = 1.0 - up(
                vpd,
                float(member["vapor_pressure_deficit_unconstrained_pa"]),
                float(member["vapor_pressure_deficit_inactive_pa"]),
            )
            i_photo = up(
                photo,
                float(member["photoperiod_inactive_hours"]),
                float(member["photoperiod_unconstrained_hours"]),
            )
            instantaneous = i_tmin * i_vpd * i_photo
            history.append(instantaneous)
            if len(history) > 21:
                history.pop(0)
            gsi21 = sum(history) / len(history)
            result = published_map[(member["candidate_id"], forcing_row["date"])]
            expected = {
                "i_tmin": i_tmin,
                "i_vpd": i_vpd,
                "i_photo": i_photo,
                "instantaneous_gsi": instantaneous,
                "gsi21": gsi21,
                "photoperiod_hours": photo,
            }
            for field, value in expected.items():
                residual = abs(float(result[field]) - value)
                maximum = max(maximum, residual)
                assert residual <= 1.0e-12, (field, residual)
            assert int(result["sample_count"]) == min(index + 1, 21)
            assert abs(float(result["reconstruction_residual"])) <= 1.0e-12
            minimum = min(i_tmin, i_vpd, i_photo)
            ties = "+".join(
                sorted(
                    name
                    for name, value in (
                        ("TEMPERATURE", i_tmin),
                        ("VPD", i_vpd),
                        ("PHOTOPERIOD", i_photo),
                    )
                    if value == minimum
                )
            )
            assert result["minimum_constraint_tie_set"] == ties
    return maximum


def check_event_inventories() -> None:
    absolute = rows(ART / "absolute-reproduction.csv")
    sensitivity = rows(ART / "model-level-sensitivity.csv")
    source = rows(ART / "source-level-audit.csv")
    scenario = rows(ART / "scenario-event-screen.csv")
    crossings = rows(ART / "all-crossings.csv")
    assert len(absolute) == 148
    assert len(sensitivity) == 1628
    assert len(source) == 444
    assert len(scenario) == 1488
    assert len({(row["member_or_default"], row["event_id"]) for row in absolute}) == 148
    assert len(
        {
            (row["member_or_default"], row["event_id"], row["model_level"])
            for row in sensitivity
        }
    ) == 1628
    assert {float(row["model_level"]) for row in sensitivity} == set(LEVELS)
    assert len(
        {
            (row["member_or_default"], row["event_id"], row["source_level"])
            for row in source
        }
    ) == 444
    assert {row["source_level"] for row in source} == {"0.10", "0.25", "0.50"}
    assert all(row["source_level"] == row["model_level"] for row in source)
    scenario_keys = {
        (
            row["scenario"],
            row["member_or_default"],
            row["event_id"],
            row["operator"],
        )
        for row in scenario
    }
    assert len(scenario_keys) == 1488
    assert sum(row["scenario"] == "SC_PLANT_GENERALIZED_DEFAULT" for row in scenario) == 8
    crossing_keys = {
        (
            row["scenario"],
            row["member_or_default"],
            row["event_id"],
            row["operator"],
            row["model_level"],
            row["direction"],
            row["crossing_sequence"],
        )
        for row in crossings
    }
    assert len(crossing_keys) == len(crossings)
    assert all(row["direction"] in {"rising", "falling"} for row in crossings)
    assert all(row["inside_event_window"] in {"True", "False"} for row in crossings)
    numeric_fields = (
        "threshold_gsi21",
        "event_date_gsi21",
        "selected_crossing_ordinal",
        "residual_days",
    )
    for collection in (absolute, sensitivity, source, scenario):
        for row in collection:
            for field in numeric_fields:
                if field in row and row[field]:
                    assert math.isfinite(float(row[field]))
            assert row["matched"] in {"True", "False"}
            if row["matched"] == "False":
                assert not row["selected_crossing_ordinal"]
                assert not row["residual_days"]


def check_cal07c_reproduction() -> None:
    actual = rows(ART / "absolute-reproduction.csv")
    expected = rows(CAL07C / "artifacts/transition-residuals.csv")
    expected_map = {
        (row["candidate_id"], row["year"], row["direction"]): row for row in expected
    }
    matches = 0
    for row in actual:
        prior = expected_map[
            (row["member_or_default"], row["event_year"], row["direction"])
        ]
        assert row["observed_date"] == prior["observed_date_50"]
        assert int(row["in_window_count"]) == int(prior["same_direction_crossing_count"])
        assert bool(row["residual_days"]) == bool(prior["residual_days"])
        if row["residual_days"]:
            matches += 1
            assert abs(float(row["residual_days"]) - float(prior["residual_days"])) <= 5e-7
            assert (
                abs(
                    float(row["selected_crossing_ordinal"])
                    - float(prior["modeled_crossing_ordinal"])
                )
                <= 5e-7
            )
    assert matches == 11


def check_relative_thresholds() -> None:
    base = rows(ART / "base-member-daily.csv")
    by_member_year: dict[tuple[str, str], list[float]] = defaultdict(list)
    for row in base:
        if row["year"] in {"2024", "2025"}:
            by_member_year[(row["candidate_id"], row["year"])].append(float(row["gsi21"]))
    sensitivity = rows(ART / "model-level-sensitivity.csv")
    for row in sensitivity:
        values = by_member_year[(row["member_or_default"], row["event_year"])]
        level = float(row["model_level"])
        expected = min(values) + level * (max(values) - min(values))
        assert abs(float(row["threshold_gsi21"]) - expected) <= 1.0e-15
    # A fixed event threshold is independent of any crossing date or year boundary.
    grouped: dict[tuple[str, str, str, str, str], set[str]] = defaultdict(set)
    for row in rows(ART / "all-crossings.csv"):
        if row["operator"] == "EVENT_YEAR_RELATIVE":
            grouped[
                (
                    row["scenario"],
                    row["member_or_default"],
                    row["event_id"],
                    row["operator"],
                    row["model_level"],
                )
            ].add(row["threshold_gsi21"])
    assert all(len(values) == 1 for values in grouped.values())


def check_scenario_summaries() -> None:
    daily = rows(ART / "daily-scenario-ensemble.csv")
    assert len(daily) == 9996
    assert len({(row["scenario"], row["date"]) for row in daily}) == 9996
    counts = Counter(row["scenario"] for row in daily)
    assert set(counts.values()) == {1666}
    assert counts["SC_PLANT_GENERALIZED_DEFAULT"] == 1666
    assert {
        row["members"]
        for row in daily
        if row["scenario"] == "SC_PLANT_GENERALIZED_DEFAULT"
    } == {"1"}
    assert {
        row["members"] for row in daily if row["scenario"] != "SC_PLANT_GENERALIZED_DEFAULT"
    } == {"37"}
    for row in daily:
        for field in row:
            if field.endswith(("_p05", "_median", "_p95")):
                value = float(row[field])
                assert math.isfinite(value)
                assert 0.0 <= value <= 1.0


def check_predicates() -> None:
    decision = {row["hypothesis"]: row for row in rows(ART / "decision-screen.csv")}
    assert set(decision) == {
        "OBSERVATION_SCALE",
        "TEMPERATURE_CONSTRAINT",
        "VPD_CONSTRAINT",
        "PHOTOPERIOD_CONSTRAINT",
        "CURRENT_GSI_CONSTRAINT_SENSITIVITY",
        "FORCING_LIMITATION",
        "MISSING_PROCESS",
    }
    absolute = {
        (row["member_or_default"], row["event_id"]): row
        for row in rows(ART / "absolute-reproduction.csv")
    }
    source = rows(ART / "source-level-audit.csv")
    recovered = sum(
        row["matched"] == "True"
        and absolute[(row["member_or_default"], row["event_id"])]["matched"] == "False"
        for row in source
    )
    assert recovered == int(decision["OBSERVATION_SCALE"]["predicate_value"])
    assert decision["OBSERVATION_SCALE"]["status"] == "SUPPORTED_AS_CONTRIBUTOR"
    assert decision["FORCING_LIMITATION"]["status"] == "PLAUSIBLE_UNRESOLVED"
    assert decision["MISSING_PROCESS"]["status"] == "PLAUSIBLE_UNRESOLVED"
    assert "does not identify" in decision["CURRENT_GSI_CONSTRAINT_SENSITIVITY"]["claim_ceiling"]


def check_observation_support() -> None:
    support = rows(ART / "observation-support.csv")
    assert len(support) == 12
    assert len({(row["event_id"], row["source_level"]) for row in support}) == 12
    assert all(row["window_calendar_days"] == "43" for row in support)
    assert all(row["source_span_is_confidence_interval"] == "False" for row in support)
    assert all(
        int(row["accepted_raw_gcc90_days"]) + int(row["missing_or_rejected_raw_days"])
        == 43
        for row in support
    )
    falling = [row for row in support if row["direction"] == "falling"]
    by_event: dict[str, dict[str, date]] = defaultdict(dict)
    for row in falling:
        by_event[row["event_id"]][row["source_level"]] = date.fromisoformat(
            row["source_date"]
        )
    assert all(
        levels["0.50"] < levels["0.25"] < levels["0.10"]
        for levels in by_event.values()
    )


def check_figures_and_manifest() -> None:
    expected = {
        "cal07d-crossing-map",
        "cal07d-indicator-chronology",
        "cal07d-constraint-removal-screen",
        "cal07d-threshold-sensitivity",
    }
    assert {path.stem for path in FIG.glob("*.svg")} == expected
    assert {path.stem for path in FIG.glob("*.md")} == expected
    for name in expected:
        svg = FIG / f"{name}.svg"
        sidecar = FIG / f"{name}.md"
        ET.parse(svg)
        root = ET.parse(svg).getroot()
        assert root.find("{http://www.w3.org/2000/svg}title") is not None
        assert root.find("{http://www.w3.org/2000/svg}desc") is not None
        metadata = root.find(
            "{http://www.w3.org/2000/svg}metadata"
        )
        assert metadata is not None
        assert metadata.attrib["id"] == "cal07d-data-binding"
        assert metadata.text
        binding = json.loads(metadata.text)
        for source, source_binding in binding["sources"].items():
            source_path = ART / source
            assert source_binding == {
                "rows": len(rows(source_path)),
                "sha256": digest(source_path),
            }
        records, fields = figure_binding_records(name)
        encoded = json.dumps(
            [[record[field] for field in fields] for record in records],
            separators=(",", ":"),
            ensure_ascii=True,
        ).encode("utf-8")
        assert binding["plotted_record_count"] == len(records)
        assert binding["plotted_fields"] == list(fields)
        assert binding["plotted_data_sha256"] == hashlib.sha256(encoded).hexdigest()
        content = sidecar.read_text(encoding="utf-8")
        for heading in (
            "## Caption",
            "## Plain-language takeaway",
            "## Methods and source bindings",
            "## Assumptions and evidence ceiling",
            "## Limitations",
            "## Accessibility",
        ):
            assert heading in content
        assert "ASSUMED_FOR_EXECUTION" in content
        assert "Order 7" in content
    manifest = rows(ART / "result-manifest.csv")
    assert len({row["path"] for row in manifest}) == len(manifest)
    for item in manifest:
        path = PKG / item["path"]
        assert path.is_file(), path
        assert digest(path) == item["sha256"], path
        assert path.stat().st_size == int(item["bytes"])


def figure_binding_records(
    name: str,
) -> tuple[list[dict[str, str]], tuple[str, ...]]:
    if name == "cal07d-crossing-map":
        records = [
            {"kind": "ABS", **row}
            for row in rows(ART / "absolute-reproduction.csv")
        ] + [
            {"kind": row["source_level"], **row}
            for row in rows(ART / "source-level-audit.csv")
            if row["source_level"] in {"0.10", "0.25", "0.50"}
        ]
        return records, (
            "kind",
            "member_or_default",
            "event_id",
            "residual_days",
        )
    if name == "cal07d-indicator-chronology":
        daily = [
            row
            for row in rows(ART / "daily-scenario-ensemble.csv")
            if row["scenario"] == "BASE" and row["year"] in {"2024", "2025"}
        ]
        attribution = rows(ART / "event-indicator-attribution.csv")
        records = [
            {
                "record_type": "daily",
                "key_1": row["date"],
                "key_2": row["scenario"],
                "value_1": row["i_tmin_median"],
                "value_2": row["i_vpd_median"],
                "value_3": row["i_photo_median"],
                "value_4": row["instantaneous_gsi_median"],
                "value_5": row["gsi21_median"],
                "category": row["minimum_constraint_tie_counts"],
            }
            for row in daily
        ] + [
            {
                "record_type": "event",
                "key_1": row["source_date"],
                "key_2": row["event_id"],
                "value_1": "",
                "value_2": "",
                "value_3": "",
                "value_4": "",
                "value_5": "",
                "category": row["direction"],
            }
            for row in attribution
            if row["source_level"] == "0.50"
        ]
        return records, (
            "record_type",
            "key_1",
            "key_2",
            "value_1",
            "value_2",
            "value_3",
            "value_4",
            "value_5",
            "category",
        )
    if name == "cal07d-constraint-removal-screen":
        return rows(ART / "scenario-event-screen.csv"), (
            "scenario",
            "member_or_default",
            "event_id",
            "operator",
            "matched",
            "residual_days",
        )
    if name == "cal07d-threshold-sensitivity":
        return rows(ART / "model-level-sensitivity.csv"), (
            "member_or_default",
            "event_id",
            "model_level",
            "matched",
            "residual_days",
        )
    raise AssertionError(name)


def reference_events() -> list[dict[str, object]]:
    events: list[dict[str, object]] = []
    for source in comment_rows(SOURCE_TRANSITIONS):
        event: dict[str, object] = {
            "event_id": f"{source['year']}-{source['direction']}",
            "year": int(source["year"]),
            "direction": source["direction"],
        }
        for suffix in ("10", "25", "50"):
            event[f"date_{suffix}"] = date.fromisoformat(
                f"{source['year']}-{source[f'date_{suffix}']}"
            )
            event[f"doy_{suffix}"] = int(source[f"DOY_{suffix}"])
        events.append(event)
    events.sort(key=lambda item: item["date_50"])
    for index, event in enumerate(events):
        event["eligible"] = 0 < index < len(events) - 1
        if event["eligible"]:
            event["lower"] = 0.5 * (
                events[index - 1]["date_50"].toordinal()
                + event["date_50"].toordinal()
            )
            event["upper"] = 0.5 * (
                event["date_50"].toordinal()
                + events[index + 1]["date_50"].toordinal()
            )
    return events


def reference_trajectory(
    forcing: list[dict[str, str]], member: dict[str, str], scenario: str
) -> list[dict[str, object]]:
    if scenario == DEFAULT_SCENARIO:
        bounds = (-2.0, 5.0, 900.0, 4100.0, 10.0, 11.0)
    else:
        bounds = (
            float(member["minimum_temperature_inactive_c"]),
            float(member["minimum_temperature_unconstrained_c"]),
            float(member["vapor_pressure_deficit_unconstrained_pa"]),
            float(member["vapor_pressure_deficit_inactive_pa"]),
            float(member["photoperiod_inactive_hours"]),
            float(member["photoperiod_unconstrained_hours"]),
        )
    history: list[float] = []
    result: list[dict[str, object]] = []
    previous: date | None = None
    for source in forcing:
        day = date.fromisoformat(source["date"])
        if previous is not None:
            assert day == previous + timedelta(days=1)
        previous = day
        tmin = float(source["tmin_c"])
        vpd = float(source["vpd_pa"])
        photo = daylight(float(source["latitude_degrees"]), int(source["doy"]))
        i_tmin = up(tmin, bounds[0], bounds[1])
        i_vpd = 1.0 - up(vpd, bounds[2], bounds[3])
        i_photo = up(photo, bounds[4], bounds[5])
        if scenario == "TEMPERATURE_UNCONSTRAINED":
            i_tmin = 1.0
        if scenario == "VPD_UNCONSTRAINED":
            i_vpd = 1.0
        if scenario == "PHOTOPERIOD_UNCONSTRAINED":
            i_photo = 1.0
        if scenario == "PHOTOPERIOD_AND_VPD_UNCONSTRAINED":
            i_vpd = 1.0
            i_photo = 1.0
        instantaneous = i_tmin * i_vpd * i_photo
        history.append(instantaneous)
        if len(history) > 21:
            del history[0]
        minimum = min(i_tmin, i_vpd, i_photo)
        tie = "+".join(
            sorted(
                label
                for label, value in (
                    ("TEMPERATURE", i_tmin),
                    ("VPD", i_vpd),
                    ("PHOTOPERIOD", i_photo),
                )
                if value == minimum
            )
        )
        result.append(
            {
                "date": day.isoformat(),
                "year": day.year,
                "doy": day.timetuple().tm_yday,
                "tmin_c": tmin,
                "vpd_pa": vpd,
                "photoperiod_hours": photo,
                "i_tmin": i_tmin,
                "i_vpd": i_vpd,
                "i_photo": i_photo,
                "instantaneous_gsi": instantaneous,
                "gsi21": sum(history) / len(history),
                "tie": tie,
            }
        )
    return result


def reference_threshold(
    trajectory: list[dict[str, object]], year: int, level: float
) -> float | None:
    values = [float(row["gsi21"]) for row in trajectory if row["year"] == year]
    lower, upper = min(values), max(values)
    return None if lower == upper else lower + level * (upper - lower)


def reference_crossings(
    trajectory: list[dict[str, object]], threshold: float
) -> list[tuple[str, float]]:
    output: list[tuple[str, float]] = []
    for old_row, new_row in zip(trajectory, trajectory[1:]):
        old, new = float(old_row["gsi21"]), float(new_row["gsi21"])
        direction = None
        if old < threshold <= new:
            direction = "rising"
        elif old >= threshold > new:
            direction = "falling"
        if direction is not None:
            fraction = (threshold - old) / (new - old)
            ordinal = date.fromisoformat(str(old_row["date"])).toordinal() + fraction
            output.append((direction, ordinal))
    return output


def reference_event_row(
    scenario: str,
    member: str,
    event: dict[str, object],
    operator: str,
    level: float,
    observed: date,
    threshold: float | None,
    crossings: list[tuple[str, float]],
    trajectory: list[dict[str, object]],
) -> dict[str, object]:
    same = [
        ordinal
        for direction, ordinal in crossings
        if direction == event["direction"]
    ]
    inside = [
        ordinal
        for ordinal in same
        if float(event["lower"]) < ordinal <= float(event["upper"])
    ]
    selected = inside[0] if inside else None
    state = next(
        row for row in trajectory if row["date"] == observed.isoformat()
    )
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
        "event_date_gsi21": f"{float(state['gsi21']):.17g}",
        "matched": selected is not None,
        "selected_crossing_ordinal": (
            "" if selected is None else f"{selected:.9f}"
        ),
        "selected_crossing_date": (
            ""
            if selected is None
            else date.fromordinal(math.floor(selected)).isoformat()
        ),
        "residual_days": (
            "" if selected is None else f"{selected-observed.toordinal():.9f}"
        ),
        "global_same_direction_count": len(same),
        "in_window_count": len(inside),
        "extra_in_window_count": max(0, len(inside) - 1),
        "out_of_window_count": len(same) - len(inside),
    }


def reference_crossing_rows(
    scenario: str,
    member: str,
    event: dict[str, object],
    operator: str,
    level: float,
    threshold: float | None,
    crossings: list[tuple[str, float]],
) -> list[dict[str, object]]:
    if threshold is None:
        return []
    return [
        {
            "scenario": scenario,
            "member_or_default": member,
            "event_id": event["event_id"],
            "operator": operator,
            "model_level": f"{level:.2f}",
            "threshold_gsi21": f"{threshold:.17g}",
            "direction": direction,
            "crossing_sequence": sequence,
            "crossing_ordinal": f"{ordinal:.9f}",
            "crossing_date": date.fromordinal(math.floor(ordinal)).isoformat(),
            "inside_event_window": (
                float(event["lower"]) < ordinal <= float(event["upper"])
            ),
        }
        for sequence, (direction, ordinal) in enumerate(crossings, start=1)
    ]


def reference_daily_summary(
    scenario: str, trajectories: dict[str, list[dict[str, object]]]
) -> list[dict[str, object]]:
    output: list[dict[str, object]] = []
    first = next(iter(trajectories.values()))
    for index, template in enumerate(first):
        day_values = [trajectory[index] for trajectory in trajectories.values()]
        row: dict[str, object] = {
            "scenario": scenario,
            "date": template["date"],
            "year": template["year"],
            "doy": template["doy"],
            "members": len(day_values),
            "tmin_c": f"{float(template['tmin_c']):.9f}",
            "vpd_pa": f"{float(template['vpd_pa']):.9f}",
            "photoperiod_hours": f"{float(template['photoperiod_hours']):.9f}",
        }
        for field in (
            "i_tmin",
            "i_vpd",
            "i_photo",
            "instantaneous_gsi",
            "gsi21",
        ):
            values = [float(item[field]) for item in day_values]
            row[f"{field}_p05"] = f"{percentile(values, 0.05):.9f}"
            row[f"{field}_median"] = f"{statistics.median(values):.9f}"
            row[f"{field}_p95"] = f"{percentile(values, 0.95):.9f}"
        ties = Counter(str(item["tie"]) for item in day_values)
        row["minimum_constraint_tie_counts"] = ";".join(
            f"{label}:{ties[label]}" for label in sorted(ties)
        )
        output.append(row)
    return output


def independently_validate_scenarios() -> dict[str, object]:
    members = rows(CAL07C / "inputs/ensemble.csv")
    forcing = [
        row
        for row in rows(CAL07C / "inputs/forcing.csv")
        if row["site_id"] == "SH-DB-BEZA"
    ]
    events = [event for event in reference_events() if event["eligible"]]
    published_daily = rows(ART / "daily-scenario-ensemble.csv")
    published_scenario = rows(ART / "scenario-event-screen.csv")
    published_absolute = rows(ART / "absolute-reproduction.csv")
    published_sensitivity = rows(ART / "model-level-sensitivity.csv")
    published_source = rows(ART / "source-level-audit.csv")
    published_crossings = rows(ART / "all-crossings.csv")
    expected_daily: list[dict[str, object]] = []
    expected_scenario: list[dict[str, object]] = []
    expected_absolute: list[dict[str, object]] = []
    expected_sensitivity: list[dict[str, object]] = []
    expected_source: list[dict[str, object]] = []
    expected_crossings: list[dict[str, object]] = []
    base_trajectories: dict[str, list[dict[str, object]]] = {}

    for scenario in ENSEMBLE_SCENARIOS:
        trajectories: dict[str, list[dict[str, object]]] = {}
        for member in members:
            member_id = member["candidate_id"]
            trajectory = reference_trajectory(forcing, member, scenario)
            trajectories[member_id] = trajectory
            if scenario == "BASE":
                base_trajectories[member_id] = trajectory
            for event in events:
                observed_50 = event["date_50"]
                for operator, level in (
                    ("ABSOLUTE_0_5", 0.50),
                    ("EVENT_YEAR_RELATIVE", 0.50),
                ):
                    threshold = (
                        0.5
                        if operator == "ABSOLUTE_0_5"
                        else reference_threshold(
                            trajectory, int(event["year"]), level
                        )
                    )
                    crossings = (
                        []
                        if threshold is None
                        else reference_crossings(trajectory, threshold)
                    )
                    event_result = reference_event_row(
                        scenario,
                        member_id,
                        event,
                        operator,
                        level,
                        observed_50,
                        threshold,
                        crossings,
                        trajectory,
                    )
                    expected_scenario.append(event_result)
                    expected_crossings.extend(
                        reference_crossing_rows(
                            scenario,
                            member_id,
                            event,
                            operator,
                            level,
                            threshold,
                            crossings,
                        )
                    )
                    if scenario == "BASE" and operator == "ABSOLUTE_0_5":
                        expected_absolute.append(
                            {
                                **event_result,
                                "cal07c_expected_observed_date": (
                                    observed_50.isoformat()
                                ),
                            }
                        )
                if scenario != "BASE":
                    continue
                for level in LEVELS:
                    threshold = reference_threshold(
                        trajectory, int(event["year"]), level
                    )
                    crossings = (
                        []
                        if threshold is None
                        else reference_crossings(trajectory, threshold)
                    )
                    expected_sensitivity.append(
                        reference_event_row(
                            scenario,
                            member_id,
                            event,
                            "EVENT_YEAR_RELATIVE",
                            level,
                            observed_50,
                            threshold,
                            crossings,
                            trajectory,
                        )
                    )
                    if level != 0.50:
                        expected_crossings.extend(
                            reference_crossing_rows(
                                scenario,
                                member_id,
                                event,
                                "EVENT_YEAR_RELATIVE",
                                level,
                                threshold,
                                crossings,
                            )
                        )
                for level, suffix in (
                    (0.10, "10"),
                    (0.25, "25"),
                    (0.50, "50"),
                ):
                    threshold = reference_threshold(
                        trajectory, int(event["year"]), level
                    )
                    crossings = (
                        []
                        if threshold is None
                        else reference_crossings(trajectory, threshold)
                    )
                    expected_source.append(
                        {
                            **reference_event_row(
                                scenario,
                                member_id,
                                event,
                                "SOURCE_LEVEL_ANALOGY",
                                level,
                                event[f"date_{suffix}"],
                                threshold,
                                crossings,
                                trajectory,
                            ),
                            "source_level": f"{level:.2f}",
                            "source_level_span_is_confidence_interval": False,
                        }
                    )
        expected_daily.extend(reference_daily_summary(scenario, trajectories))

    default_member = {"candidate_id": "DEFAULT"}
    default_trajectory = reference_trajectory(
        forcing, default_member, DEFAULT_SCENARIO
    )
    expected_daily.extend(
        reference_daily_summary(
            DEFAULT_SCENARIO, {"DEFAULT": default_trajectory}
        )
    )
    for event in events:
        for operator, level in (
            ("ABSOLUTE_0_5", 0.50),
            ("EVENT_YEAR_RELATIVE", 0.50),
        ):
            threshold = (
                0.5
                if operator == "ABSOLUTE_0_5"
                else reference_threshold(
                    default_trajectory, int(event["year"]), level
                )
            )
            crossings = (
                []
                if threshold is None
                else reference_crossings(default_trajectory, threshold)
            )
            expected_scenario.append(
                reference_event_row(
                    DEFAULT_SCENARIO,
                    "DEFAULT",
                    event,
                    operator,
                    level,
                    event["date_50"],
                    threshold,
                    crossings,
                    default_trajectory,
                )
            )
            expected_crossings.extend(
                reference_crossing_rows(
                    DEFAULT_SCENARIO,
                    "DEFAULT",
                    event,
                    operator,
                    level,
                    threshold,
                    crossings,
                )
            )

    event_key = (
        "scenario",
        "member_or_default",
        "event_id",
        "operator",
        "model_level",
        "observed_date",
    )
    compare_records(
        published_daily,
        expected_daily,
        ("scenario", "date"),
    )
    compare_records(published_scenario, expected_scenario, event_key)
    compare_records(published_absolute, expected_absolute, event_key)
    compare_records(published_sensitivity, expected_sensitivity, event_key)
    compare_records(published_source, expected_source, event_key)
    compare_records(
        published_crossings,
        expected_crossings,
        (
            "scenario",
            "member_or_default",
            "event_id",
            "operator",
            "model_level",
            "direction",
            "crossing_sequence",
        ),
    )
    return {
        "members": members,
        "forcing": forcing,
        "events": events,
        "base_trajectories": base_trajectories,
        "scenario_rows": expected_scenario,
        "absolute_rows": expected_absolute,
        "source_rows": expected_source,
    }


def independently_validate_observations(
    context: dict[str, object],
) -> list[dict[str, object]]:
    source_daily = {
        row["date"]: row for row in comment_rows(SOURCE_DAILY)
    }
    expected_support: list[dict[str, object]] = []
    for event in context["events"]:
        for level, suffix in (
            (0.10, "10"),
            (0.25, "25"),
            (0.50, "50"),
        ):
            event_date = event[f"date_{suffix}"]
            window = [
                source_daily.get(
                    (event_date + timedelta(days=offset)).isoformat()
                )
                for offset in range(-21, 22)
            ]
            accepted = [
                row
                for row in window
                if row is not None
                and row["image_count"] != "NA"
                and int(row["image_count"]) > 0
                and row["gcc_90"] != "NA"
                and row["outlierflag_gcc_90"] == "0"
            ]
            event_day = source_daily[event_date.isoformat()]
            expected_support.append(
                {
                    "event_id": event["event_id"],
                    "direction": event["direction"],
                    "source_level": f"{level:.2f}",
                    "source_date": event_date.isoformat(),
                    "source_doy": event[f"doy_{suffix}"],
                    "window_calendar_days": 43,
                    "accepted_raw_gcc90_days": len(accepted),
                    "missing_or_rejected_raw_days": 43 - len(accepted),
                    "source_smooth_gcc90": event_day["smooth_gcc_90"],
                    "source_smooth_ci_width": event_day["smooth_ci_gcc_90"],
                    "event_day_image_count": event_day["image_count"],
                    "event_day_raw_gcc90": event_day["gcc_90"],
                    "event_day_outlier_flag": event_day["outlierflag_gcc_90"],
                    "source_span_is_confidence_interval": False,
                }
            )
    compare_records(
        rows(ART / "observation-support.csv"),
        expected_support,
        ("event_id", "source_level"),
    )

    forcing_map = {row["date"]: row for row in context["forcing"]}
    support_map = {
        (row["event_id"], row["source_level"]): row
        for row in expected_support
    }
    trajectories = context["base_trajectories"]
    first = next(iter(trajectories.values()))
    index_by_date = {row["date"]: index for index, row in enumerate(first)}
    expected_attribution: list[dict[str, object]] = []
    for event in context["events"]:
        for level, suffix in (
            (0.10, "10"),
            (0.25, "25"),
            (0.50, "50"),
        ):
            day = event[f"date_{suffix}"].isoformat()
            index = index_by_date[day]
            values = [trajectory[index] for trajectory in trajectories.values()]
            start = max(0, index - 20)
            ties = Counter(str(value["tie"]) for value in values)
            support = support_map[(event["event_id"], f"{level:.2f}")]
            result: dict[str, object] = {
                "event_id": event["event_id"],
                "direction": event["direction"],
                "source_level": f"{level:.2f}",
                "source_date": day,
                "tmin_c": forcing_map[day]["tmin_c"],
                "vpd_pa": forcing_map[day]["vpd_pa"],
                "photoperiod_hours": f"{float(values[0]['photoperiod_hours']):.9f}",
                "members": 37,
                "minimum_constraint_tie_counts": ";".join(
                    f"{label}:{ties[label]}" for label in sorted(ties)
                ),
                "accepted_raw_gcc90_days_pm21": support[
                    "accepted_raw_gcc90_days"
                ],
                "source_smooth_gcc90": support["source_smooth_gcc90"],
                "source_smooth_ci_width": support["source_smooth_ci_width"],
            }
            for field in (
                "i_tmin",
                "i_vpd",
                "i_photo",
                "instantaneous_gsi",
                "gsi21",
            ):
                field_values = [float(value[field]) for value in values]
                result[f"{field}_p05"] = f"{percentile(field_values, 0.05):.9f}"
                result[f"{field}_median"] = (
                    f"{statistics.median(field_values):.9f}"
                )
                result[f"{field}_p95"] = f"{percentile(field_values, 0.95):.9f}"
            for field in (
                "i_tmin",
                "i_vpd",
                "i_photo",
                "instantaneous_gsi",
            ):
                means = [
                    statistics.fmean(
                        float(trajectory[position][field])
                        for position in range(start, index + 1)
                    )
                    for trajectory in trajectories.values()
                ]
                result[f"preceding21_{field}_median"] = (
                    f"{statistics.median(means):.9f}"
                )
            expected_attribution.append(result)
    compare_records(
        rows(ART / "event-indicator-attribution.csv"),
        expected_attribution,
        ("event_id", "source_level"),
    )
    return expected_attribution


def independently_validate_decisions(
    context: dict[str, object], attribution: list[dict[str, object]]
) -> None:
    absolute = {
        (row["member_or_default"], row["event_id"]): row
        for row in context["absolute_rows"]
    }
    recovered = sum(
        row["matched"]
        and not absolute[(row["member_or_default"], row["event_id"])]["matched"]
        for row in context["source_rows"]
    )
    scenario_map = {
        (
            row["scenario"],
            row["member_or_default"],
            row["event_id"],
            row["operator"],
        ): row
        for row in context["scenario_rows"]
    }
    expected: dict[str, tuple[str, int]] = {
        "OBSERVATION_SCALE": (
            "SUPPORTED_AS_CONTRIBUTOR"
            if recovered > 0
            else "NOT_SUPPORTED_BY_SCREEN",
            recovered,
        )
    }
    match_increase = False
    for label, scenario in (
        ("TEMPERATURE", "TEMPERATURE_UNCONSTRAINED"),
        ("VPD", "VPD_UNCONSTRAINED"),
        ("PHOTOPERIOD", "PHOTOPERIOD_UNCONSTRAINED"),
    ):
        changed = 0
        base_matches = 0
        alternate_matches = 0
        for member in context["members"]:
            for event in context["events"]:
                for operator in ("ABSOLUTE_0_5", "EVENT_YEAR_RELATIVE"):
                    key = (
                        member["candidate_id"],
                        event["event_id"],
                        operator,
                    )
                    base = scenario_map[("BASE", *key)]
                    alternate = scenario_map[(scenario, *key)]
                    base_matches += int(base["matched"])
                    alternate_matches += int(alternate["matched"])
                    changed += int(
                        float(base["event_date_gsi21"])
                        != float(alternate["event_date_gsi21"])
                        or base["matched"] != alternate["matched"]
                        or base["selected_crossing_ordinal"]
                        != alternate["selected_crossing_ordinal"]
                    )
        match_increase |= alternate_matches > base_matches
        expected[f"{label}_CONSTRAINT"] = (
            "SUPPORTED_AS_MATHEMATICAL_CONTRIBUTOR"
            if changed > 0
            else "NOT_SUPPORTED_BY_SCREEN",
            changed,
        )
    expected["CURRENT_GSI_CONSTRAINT_SENSITIVITY"] = (
        "PLAUSIBLE_UNRESOLVED"
        if match_increase
        else "NOT_SUPPORTED_BY_SCREEN",
        int(match_increase),
    )
    expected["FORCING_LIMITATION"] = ("PLAUSIBLE_UNRESOLVED", 1)
    rising = [
        row
        for row in attribution
        if row["direction"] == "rising" and row["source_level"] == "0.50"
    ]
    missing = any(
        float(row["instantaneous_gsi_median"]) < 0.5
        and sum(
            float(row[field]) < 0.5
            for field in (
                "i_tmin_median",
                "i_vpd_median",
                "i_photo_median",
            )
        )
        >= 2
        for row in rising
    )
    expected["MISSING_PROCESS"] = (
        "PLAUSIBLE_UNRESOLVED" if missing else "NOT_SUPPORTED_BY_SCREEN",
        int(missing),
    )
    actual = {
        row["hypothesis"]: row for row in rows(ART / "decision-screen.csv")
    }
    assert actual.keys() == expected.keys()
    for hypothesis, (status, value) in expected.items():
        assert actual[hypothesis]["status"] == status
        assert int(actual[hypothesis]["predicate_value"]) == value


def main() -> None:
    check_manifest()
    maximum = independent_base_reconstruction()
    check_event_inventories()
    check_cal07c_reproduction()
    check_relative_thresholds()
    check_scenario_summaries()
    check_predicates()
    check_observation_support()
    context = independently_validate_scenarios()
    attribution = independently_validate_observations(context)
    independently_validate_decisions(context, attribution)
    check_figures_and_manifest()
    print(
        "CAL-07D validation PASS: "
        f"61,642 BASE rows independently reconstructed; "
        f"maximum equation residual={maximum:.3e}; "
        "9,996 scenario-days and every event/crossing row reproduced; "
        "11 CAL-07C matches reproduced; 4 SVG/sidecar pairs verified"
    )


if __name__ == "__main__":
    main()
