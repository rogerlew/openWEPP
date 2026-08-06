"""Result-blind unit tests for the carrier-term audit operators."""

from __future__ import annotations

import importlib.util
import json
import sys
from pathlib import Path

TOOL = Path(__file__).with_name("run_carrier_term_audit.py")
SPEC = importlib.util.spec_from_file_location("carrier_audit", TOOL)
assert SPEC is not None and SPEC.loader is not None
AUDIT = importlib.util.module_from_spec(SPEC)
sys.modules["carrier_audit"] = AUDIT
SPEC.loader.exec_module(AUDIT)


def evaluation_row(active: bool = True) -> dict[str, object]:
    seconds = 3600.0 if active else 0.0
    values = {
        "shortwave": [1.0] * 24,
        "longwave": [-2.0] * 24,
        "sensible": [3.0] * 24,
        "latent": [-4.0] * 24,
        "advected": [5.0] * 24,
    }
    if not active:
        values = {term: [0.0] * 24 for term in values}
    row: dict[str, object] = {
        "stage3_evaluation_hourly_requested_seconds": [3600.0] * 24,
        "stage3_evaluation_hourly_evaluated_seconds": [seconds] * 24,
        "stage3_evaluation_hourly_complete_carrier_evaluated": [active] * 24,
        "stage3_evaluation_hourly_internal_active_lower_conduction_j_m2": [0.0] * 24,
        "stage3_evaluation_complete_arm_internal_conduction_applicable": False,
        "stage3_evaluation_surface_arm_applicable": True,
        "stage3_evaluation_surface_arm_sensible_applicable": False,
        "stage3_evaluation_surface_arm_advected_applicable": False,
        "stage3_evaluation_surface_arm_internal_conduction_applicable": False,
        "stage3_evaluation_complete_arm_applicable": True,
        "stage3_evaluation_complete_arm_cold_content_export_applicable": False,
        "stage3_evaluation_complete_arm_available_ice_applicable": False,
        "stage3_evaluation_complete_arm_sequential_ledger_applicable": False,
        "stage3_evaluation_complete_arm_terminal_unallocated_applicable": False,
        "stage3_evaluation_complete_arm_internal_active_lower_conduction_j_m2": 0.0,
        "stage3_evaluation_requested_seconds": 24 * 3600.0,
        "stage3_evaluation_evaluated_seconds": 24 * seconds,
        "stage3_evaluation_coverage_fraction": 1.0 if active else 0.0,
    }
    for term, field in AUDIT.HOURLY_FIELDS.items():
        row[field] = values[term]
    row["stage3_evaluation_hourly_complete_energy_j_m2"] = [
        sum(values[term][hour] for term in AUDIT.TERMS) for hour in range(24)
    ]
    row["stage3_evaluation_hourly_vapor_mass_exchange_kg_m2"] = (
        [0.1] * 24 if active else [0.0] * 24
    )
    for field in AUDIT.HOURLY_ZERO_FIELDS:
        row[field] = [0.0] * 24
    for term, field in AUDIT.DAILY_FIELDS.items():
        row[field] = sum(values[term])
    for term in ("shortwave", "longwave"):
        row[f"stage3_evaluation_surface_arm_{term}_j_m2"] = sum(values[term])
    row["stage3_evaluation_surface_arm_latent_j_m2"] = 0.0
    complete = sum(sum(values[term]) for term in AUDIT.TERMS)
    surface = sum(sum(values[term]) for term in ("shortwave", "longwave"))
    row["stage3_evaluation_complete_arm_total_j_m2"] = complete
    row["stage3_evaluation_surface_arm_total_j_m2"] = surface
    row["stage3_evaluation_complete_arm_component_residual_j_m2"] = 0.0
    row["stage3_evaluation_complete_arm_vapor_mass_exchange_kg_m2"] = (
        2.4 if active else 0.0
    )
    for field in AUDIT.DAILY_ZERO_FIELDS:
        row[field] = 0.0
    return row


def test_reconstruction_accepts_independent_operands() -> None:
    AUDIT.validate_evaluation_row(evaluation_row(), AUDIT.dt.date(2020, 1, 1))


def test_reconstruction_rejects_producer_total_alias() -> None:
    row = evaluation_row()
    row["stage3_evaluation_complete_arm_total_j_m2"] = 999.0
    try:
        AUDIT.validate_evaluation_row(row, AUDIT.dt.date(2020, 1, 1))
    except RuntimeError as error:
        assert "complete reconstruction" in str(error)
    else:
        raise AssertionError("producer-total alias was accepted")


def test_internal_conduction_cannot_alias_ground() -> None:
    row = evaluation_row()
    internal = row["stage3_evaluation_hourly_internal_active_lower_conduction_j_m2"]
    assert isinstance(internal, list)
    internal[0] = 1.0
    try:
        AUDIT.validate_evaluation_row(row, AUDIT.dt.date(2020, 1, 1))
    except RuntimeError as error:
        assert "internal conduction" in str(error)
    else:
        raise AssertionError("internal conduction alias was accepted")


def test_same_state_auxiliary_operand_cannot_be_nonzero() -> None:
    row = evaluation_row()
    residual = row["stage3_evaluation_hourly_energy_closure_residual_j_m2"]
    assert isinstance(residual, list)
    residual[0] = 1.0
    try:
        AUDIT.validate_evaluation_row(row, AUDIT.dt.date(2020, 1, 1))
    except RuntimeError as error:
        assert "same-state zero" in str(error)
    else:
        raise AssertionError("nonzero same-state closure residual was accepted")


def test_hourly_complete_energy_is_independently_reconciled() -> None:
    row = evaluation_row()
    complete = row["stage3_evaluation_hourly_complete_energy_j_m2"]
    assert isinstance(complete, list)
    complete[0] = 99.0
    try:
        AUDIT.validate_evaluation_row(row, AUDIT.dt.date(2020, 1, 1))
    except RuntimeError as error:
        assert "implemented external subset" in str(error)
    else:
        raise AssertionError("contradictory hourly complete energy was accepted")


def test_component_residual_uses_frozen_daily_tolerance() -> None:
    row = evaluation_row()
    row["stage3_evaluation_complete_arm_component_residual_j_m2"] = -2.79396772e-9
    AUDIT.validate_evaluation_row(row, AUDIT.dt.date(2020, 1, 1))


def test_component_residual_exceeding_frozen_tolerance_is_rejected() -> None:
    row = evaluation_row()
    row["stage3_evaluation_complete_arm_component_residual_j_m2"] = 1.0001e-6
    try:
        AUDIT.validate_evaluation_row(row, AUDIT.dt.date(2020, 1, 1))
    except RuntimeError as error:
        assert "daily component residual" in str(error)
    else:
        raise AssertionError("out-of-tolerance component residual was accepted")


def test_zero_coverage_is_not_an_evaluated_hour() -> None:
    AUDIT.validate_evaluation_row(evaluation_row(active=False), AUDIT.dt.date(2020, 1, 1))


def test_context_bounds_are_inclusive() -> None:
    assert AUDIT.classify(-5.0, -5.0, 5.0) == "WITHIN_CONTEXT"
    assert AUDIT.classify(5.0, -5.0, 5.0) == "WITHIN_CONTEXT"
    assert AUDIT.classify(5.0001, -5.0, 5.0) == "OUTSIDE_CONTEXT"


def test_non_boolean_hourly_support_is_rejected() -> None:
    row = evaluation_row()
    active = row["stage3_evaluation_hourly_complete_carrier_evaluated"]
    assert isinstance(active, list)
    active[0] = 1
    try:
        AUDIT.validate_evaluation_row(row, AUDIT.dt.date(2020, 1, 1))
    except RuntimeError as error:
        assert "non-boolean" in str(error)
    else:
        raise AssertionError("numeric applicability was accepted")


def test_fingerprint_format_and_equality_are_enforced() -> None:
    frozen = {
        "operator_id": "operator",
        "source_snapshot_id": "source",
        "support_id": "support",
        "cadence_id": "cadence",
        "carrier_id": "carrier",
        "coverage_id": "coverage",
        "claim_class": "claim",
        "unresolved_boundaries_id": "boundary",
        "pairing_id": "pair",
        "arm_ids": ["surface", "complete"],
    }
    row = {
        f"stage3_evaluation_{key}": value
        for key, value in frozen.items()
        if key != "arm_ids"
    }
    row["stage3_evaluation_arm_ids"] = frozen["arm_ids"]
    row["stage3_evaluation_arm_count"] = 2
    for field in (
        "stage3_evaluation_surface_arm_non_formulation_fingerprint_fnv1a64",
        "stage3_evaluation_complete_arm_non_formulation_fingerprint_fnv1a64",
        "stage3_evaluation_non_formulation_fingerprint_fnv1a64",
        "stage3_evaluation_source_fingerprint_fnv1a64",
        "stage3_evaluation_forcing_fingerprint_fnv1a64",
        "stage3_evaluation_geometry_fingerprint_fnv1a64",
    ):
        row[field] = "0123456789abcdef"
    AUDIT.validate_tags(row, frozen)
    row["stage3_evaluation_geometry_fingerprint_fnv1a64"] = "XYZ"
    try:
        AUDIT.validate_tags(row, frozen)
    except RuntimeError as error:
        assert "fingerprint" in str(error)
    else:
        raise AssertionError("malformed fingerprint was accepted")


def test_observation_peak_tie_uses_earliest_and_census_is_complete(tmp_path: Path) -> None:
    observations = tmp_path / "observations.csv"
    observations.write_text(
        "date,water_year,observed_swe_mm\n"
        "2019-10-01,2020,10\n"
        "2020-01-01,2020,20\n"
        "2020-01-02,2020,20\n"
        "2020-10-01,2021,\n",
        encoding="utf-8",
    )
    peaks, census = AUDIT.observed_peaks(observations)
    assert peaks[2020][0] == AUDIT.dt.date(2020, 1, 1)
    assert census[1]["observation_disposition"] == "NO_NONMISSING_SWE"


def test_parse_trace_rejects_wrong_day_identity(tmp_path: Path) -> None:
    trace = tmp_path / "trace.jsonl"
    trace.write_text(
        json.dumps(
            {
                "schema": "openwepp-r7h-direct-production-snow-trace-v4",
                "day_index": 1,
                "lane_index": 0,
                "active_snow_coupling": False,
            }
        )
        + "\n",
        encoding="utf-8",
    )
    try:
        AUDIT.parse_trace(trace, [AUDIT.dt.date(2020, 1, 1)], {})
    except RuntimeError as error:
        assert "identity" in str(error)
    else:
        raise AssertionError("wrong trace day index was accepted")


def test_runfile_consumer_rejects_noncanonical_pass_path(tmp_path: Path) -> None:
    original_repo = AUDIT.REPO
    AUDIT.REPO = tmp_path
    try:
        climate = tmp_path / "p1.cli"
        climate.write_text("climate\n", encoding="utf-8")
        runfile = tmp_path / "audit.run"
        runfile.write_text(
            'schema = "openwepp-hillslope-runfile-v1"\n'
            '[inputs]\n'
            f'climate = "{climate}"\n'
            '[outputs]\n'
            f'pass = "{tmp_path / "wrong.hbp"}"\n'
            f'loss = "{tmp_path / "audit.loss.json"}"\n'
            f'wat = "{tmp_path / "audit.wat.parquet"}"\n',
            encoding="utf-8",
        )
        try:
            AUDIT.validate_runfile_consumer(runfile, climate)
        except RuntimeError as error:
            assert "publication path" in str(error)
        else:
            raise AssertionError("noncanonical PASS path was accepted")
    finally:
        AUDIT.REPO = original_repo
