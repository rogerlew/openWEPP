#!/usr/bin/env python3
"""Terminal validator for CAL-05 readiness evidence."""

from __future__ import annotations

import csv
import math
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
A = ROOT / "artifacts"

if not __debug__:
    raise RuntimeError("CAL-05 validator refuses optimized Python because evidence guards must run")


def rows(name: str) -> list[dict[str, str]]:
    with (A / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


producer = rows("producer-results.csv")
assert len(producer) == 16 * 20 * 365
design = {
    f"{row['source_id']}-{row['rate_id']}": row for row in rows("deterministic-design.csv")
}
grouped: dict[str, list[dict[str, str]]] = defaultdict(list)
for row in producer:
    grouped[row["candidate_id"]].append(row)
assert set(grouped) == set(design)
for candidate, trace in grouped.items():
    expected_design = design[candidate]
    assert len(trace) == 7300
    previous_surface = 0.2
    previous_interrill = 0.0
    previous_rill = 0.0
    for index, row in enumerate(trace):
        year = index // 365 + 1
        day = index % 365 + 1
        assert (int(row["year"]), int(row["day"])) == (year, day)
        assert int(row["frame_day_index"]) == index
        expected_source = (
            float(expected_design["synthetic_annual_surface_litter_input_kg_m2_yr"])
            if day == 280
            else 0.0
        )
        assert math.isclose(
            float(row["source_kg_m2"]), expected_source, rel_tol=0.0, abs_tol=1.0e-16
        )
        assert math.isclose(
            float(row["rate_d-1"]),
            float(expected_design["surface_rate_d-1"]),
            rel_tol=0.0,
            abs_tol=5.0e-18,
        )
        assert row["role"] == expected_design["role"]
        assert float(row["surface_seed_kg_m2"]) == previous_surface
        assert float(row["interrill_seed_kg_m2"]) == previous_interrill
        assert float(row["rill_seed_kg_m2"]) == previous_rill
        assert float(row["tmax_c"]) == 20.0
        assert float(row["tmin_c"]) == 10.0
        assert float(row["precipitation_m"]) == 0.004
        assert float(row["water_stress_fraction"]) == 1.0
        assert float(row["surface_after_kg_m2"]) == float(row["downstream_surface_kg_m2"])
        assert float(row["surface_after_kg_m2"]) == float(row["partition_flat_kg_m2"])
        assert float(row["surface_after_kg_m2"]) == float(row["partition_total_kg_m2"])
        previous_surface = float(row["surface_after_kg_m2"])
        previous_interrill = float(row["interrill_after_kg_m2"])
        previous_rill = float(row["rill_after_kg_m2"])
reconstruction = rows("reconstruction-results.csv")
assert len(reconstruction) == 16
assert all(row["state"] == "PASS" for row in reconstruction)
assert all(
    int(row["row_count"]) == 7300
    and float(row["maximum_abs_difference_kg_m2"]) <= 1.0e-12
    and row["first_divergence"] == "NONE"
    for row in reconstruction
)
recovery = rows("synthetic-recovery.csv")
assert [row["candidate_id"] for row in recovery if row["state"] == "RECOVERED_TRUTH"] == [
    "S020-K050"
]
assert float(next(row["daily_stock_sse"] for row in recovery if row["candidate_id"] == "S020-K050")) <= 1.0e-20
assert all(
    float(row["daily_stock_sse"]) > 1.0e-20
    for row in recovery
    if row["candidate_id"] != "S020-K050"
)
ridge = rows("terminal-stock-equifinality.csv")
assert len(ridge) == 5
assert all(row["state"] == "EQUIFINAL_TERMINAL" for row in ridge)
assert all(float(row["absolute_difference_kg_m2"]) <= 1.0e-12 for row in ridge)
ridge_design = {
    f"{row['source_id']}-{row['rate_id']}": row for row in rows("ridge-execution-design.csv")
}
ridge_grouped: dict[str, list[dict[str, str]]] = defaultdict(list)
for row in rows("ridge-producer-results.csv"):
    ridge_grouped[row["candidate_id"]].append(row)
assert set(ridge_grouped) == set(ridge_design)
assert sum(map(len, ridge_grouped.values())) == 5 * 20 * 365
for candidate, trace in ridge_grouped.items():
    expected_design = ridge_design[candidate]
    assert len(trace) == 7300
    previous_surface = 0.2
    previous_interrill = 0.0
    previous_rill = 0.0
    for index, row in enumerate(trace):
        day = index % 365 + 1
        assert (int(row["year"]), int(row["day"])) == (index // 365 + 1, day)
        assert int(row["frame_day_index"]) == index
        expected_source = (
            float(expected_design["synthetic_annual_surface_litter_input_kg_m2_yr"])
            if day == 280
            else 0.0
        )
        assert math.isclose(
            float(row["source_kg_m2"]), expected_source, rel_tol=0.0, abs_tol=1.0e-16
        )
        assert math.isclose(
            float(row["rate_d-1"]),
            float(expected_design["surface_rate_d-1"]),
            rel_tol=0.0,
            abs_tol=5.0e-18,
        )
        assert row["role"] == expected_design["role"]
        assert float(row["tmax_c"]) == 20.0
        assert float(row["tmin_c"]) == 10.0
        assert float(row["precipitation_m"]) == 0.004
        assert float(row["water_stress_fraction"]) == 1.0
        assert float(row["surface_seed_kg_m2"]) == previous_surface
        assert float(row["interrill_seed_kg_m2"]) == previous_interrill
        assert float(row["rill_seed_kg_m2"]) == previous_rill
        assert float(row["surface_after_kg_m2"]) == float(row["downstream_surface_kg_m2"])
        assert float(row["environment_index"]) == float(row["downstream_environment_index"])
        assert float(row["decay_factor"]) == float(row["downstream_decay_factor"])
        assert float(row["surface_after_kg_m2"]) == float(row["partition_flat_kg_m2"])
        assert float(row["surface_after_kg_m2"]) == float(row["partition_total_kg_m2"])
        previous_surface = float(row["surface_after_kg_m2"])
        previous_interrill = float(row["interrill_after_kg_m2"])
        previous_rill = float(row["rill_after_kg_m2"])
assert {row["candidate_id"] for row in ridge} == set(ridge_design)
ridge_reconstruction = rows("ridge-reconstruction-results.csv")
assert {row["candidate_id"] for row in ridge_reconstruction} == set(ridge_design)
assert all(
    int(row["row_count"]) == 7300
    and float(row["maximum_abs_difference_kg_m2"]) <= 1.0e-12
    and row["first_divergence"] == "NONE"
    and row["state"] == "PASS"
    for row in ridge_reconstruction
)
failures = {row["case_id"]: row for row in rows("failure-results.csv")}
design = rows("failure-and-boundary-design.csv")
assert len(failures) == len(design) == 16
for expected in design:
    observed = failures[expected["case_id"]]
    if expected["expected_variant"] == "STATE":
        assert observed["state"] == "STATE"
    else:
        assert observed["state"] == "ERROR"
        assert expected["expected_variant"] in observed["error"]
        assert expected["expected_field"] in observed["error"]
harvard = rows("harvard-source-stock-diagnostics.csv")
assert len(harvard) == 28
assert all(row["interpretation"] == "DESCRIPTIVE_NONCONTEMPORANEOUS_POOLED" for row in harvard)
source_rows = {
    (row["project"], row["plot"]): row
    for row in rows(
        "../../20260726-canopy-cal-04-05-authority-evidence-admission-001/"
        "artifacts/cal05-hf324-plot-matching.csv"
    )
}
assert len(source_rows) == 28
assert len({(row["project"], row["plot"]) for row in harvard}) == 28
assert {(row["project"], row["plot"]) for row in harvard} == set(source_rows)
for row in harvard:
    source = source_rows[(row["project"], row["plot"])]
    assert row["flux_period"] == source["litter_years"]
    assert row["stock_year"] == source["stock_year"]
    assert row["litter_row_count"] == source["litter_row_count"]
    assert row["stock_replicate_count"] == source["stock_replicate_count"]
    assert row["stock_use_not"] == source["stock_use_not"] == "1"
    assert float(row["foliar_g_c_m2_yr"]) == float(source["mean_foliar_g_c_m2_yr"])
    assert float(row["pooled_nonfoliar_g_c_m2_yr"]) == float(
        source["mean_pooled_nonfoliar_g_c_m2_yr"]
    )
    assert float(row["total_g_c_m2_yr"]) == float(source["mean_total_g_c_m2_yr"])
    assert float(row["organic_horizon_stock_kg_c_m2"]) == float(
        source["mean_organic_horizon_c_mass_kg_c_m2"]
    )
    foliar = float(row["foliar_g_c_m2_yr"])
    nonfoliar = float(row["pooled_nonfoliar_g_c_m2_yr"])
    total = float(row["total_g_c_m2_yr"])
    stock = float(row["organic_horizon_stock_kg_c_m2"])
    assert abs(foliar + nonfoliar - total) <= 1.0e-7
    assert math.isclose(
        float(row["pooled_nonfoliar_share"]),
        nonfoliar / total,
        rel_tol=0.0,
        abs_tol=1.0e-15,
    )
    assert math.isclose(
        float(row["descriptive_flux_stock_ratio_yr-1"]),
        (total / 1000.0) / stock,
        rel_tol=0.0,
        abs_tol=1.0e-15,
    )
sensitivity = rows("sensitivity-and-covariance.csv")
assert len(sensitivity) == 13
assert sum(row["metric"].startswith("local_terminal_sensitivity") for row in sensitivity) == 8
assert all(float(row["value"]) != 0.0 for row in sensitivity[:10])
metric_rows = {(row["metric"], row["scope"]): row for row in sensitivity}
terminal = {candidate: float(trace[-1]["surface_after_kg_m2"]) for candidate, trace in grouped.items()}
for rate_id in ("K000", "K050", "K100", "K200"):
    expected = (terminal[f"S030-{rate_id}"] - terminal[f"S010-{rate_id}"]) / 0.20
    row = metric_rows[("local_terminal_sensitivity_to_source", rate_id)]
    assert row["units"] == "yr"
    assert math.isclose(float(row["value"]), expected, rel_tol=0.0, abs_tol=1.0e-12)
delta_rate = 0.5 / 365.25
for source_id in ("S010", "S020", "S030", "S040"):
    expected = (terminal[f"{source_id}-K100"] - terminal[f"{source_id}-K000"]) / (
        2.0 * delta_rate
    )
    row = metric_rows[("local_terminal_sensitivity_to_rate", source_id)]
    assert row["units"] == "kg_m-2_d"
    assert math.isclose(float(row["value"]), expected, rel_tol=0.0, abs_tol=1.0e-10)
covariance_row = metric_rows[("ridge_source_rate_covariance", "five_pair_terminal_ridge")]
assert covariance_row["units"] == "kg_m-2_yr-2"
ridge_rates = [float(row["surface_rate_d-1"]) * 365.25 for row in ridge_design.values()]
ridge_sources = [
    float(row["synthetic_annual_surface_litter_input_kg_m2_yr"])
    for row in ridge_design.values()
]
mean_rate = sum(ridge_rates) / len(ridge_rates)
mean_source = sum(ridge_sources) / len(ridge_sources)
expected_covariance = sum(
    (rate - mean_rate) * (source - mean_source)
    for rate, source in zip(ridge_rates, ridge_sources, strict=True)
) / len(ridge_rates)
expected_correlation = expected_covariance / (
    (sum((rate - mean_rate) ** 2 for rate in ridge_rates) / len(ridge_rates))
    * (sum((source - mean_source) ** 2 for source in ridge_sources) / len(ridge_sources))
) ** 0.5
assert math.isclose(
    float(covariance_row["value"]), expected_covariance, rel_tol=0.0, abs_tol=1.0e-15
)
assert math.isclose(
    float(next(row["value"] for row in sensitivity if row["metric"] == "ridge_source_rate_correlation")),
    expected_correlation,
    rel_tol=0.0,
    abs_tol=1.0e-15,
)
stages = rows("stage-status-ledger.csv")
assert len(stages) == 6
assert all(
    row["science_implementation_status"] in {"IMPLEMENTED", "NOT_IMPLEMENTED", "AUTHORITY_MISSING"}
    and row["calibration_evidence_status"]
    in {
        "EMPIRICALLY_CALIBRATED",
        "CALIBRATION_READY_DATA_LIMITED",
        "NOT_CALIBRATION_READY",
        "NOT_APPLICABLE",
    }
    and row["identifiability_status"]
    in {"IDENTIFIED", "PARTIALLY_IDENTIFIABLE", "NONIDENTIFIABLE", "NOT_ASSESSED", "NOT_APPLICABLE"}
    for row in stages
)
print(
    "PASS: 116800 daily grid rows; 16 reconstructions; one recovered truth; "
    "5 terminal-ridge members; 16 boundary/failure cases; 28 Harvard rows"
)
