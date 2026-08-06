"""Result-blind unit tests for the carrier-term audit operators."""

from __future__ import annotations

import importlib.util
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
    row: dict[str, object] = {
        "stage3_evaluation_hourly_requested_seconds": [3600.0] * 24,
        "stage3_evaluation_hourly_evaluated_seconds": [seconds] * 24,
        "stage3_evaluation_hourly_complete_carrier_evaluated": [active] * 24,
        "stage3_evaluation_hourly_internal_active_lower_conduction_j_m2": [0.0] * 24,
        "stage3_evaluation_complete_arm_internal_conduction_applicable": False,
        "stage3_evaluation_complete_arm_internal_active_lower_conduction_j_m2": 0.0,
    }
    for term, field in AUDIT.HOURLY_FIELDS.items():
        row[field] = values[term]
    for term, field in AUDIT.DAILY_FIELDS.items():
        row[field] = sum(values[term])
    complete = sum(sum(values[term]) for term in AUDIT.TERMS)
    surface = sum(sum(values[term]) for term in ("shortwave", "longwave", "latent"))
    row["stage3_evaluation_complete_arm_total_j_m2"] = complete
    row["stage3_evaluation_surface_arm_total_j_m2"] = surface
    row["stage3_evaluation_complete_arm_component_residual_j_m2"] = 0.0
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
    row["stage3_evaluation_hourly_internal_active_lower_conduction_j_m2"][0] = 1.0  # type: ignore[index]
    try:
        AUDIT.validate_evaluation_row(row, AUDIT.dt.date(2020, 1, 1))
    except RuntimeError as error:
        assert "internal conduction" in str(error)
    else:
        raise AssertionError("internal conduction alias was accepted")


def test_zero_coverage_is_not_an_evaluated_hour() -> None:
    AUDIT.validate_evaluation_row(evaluation_row(active=False), AUDIT.dt.date(2020, 1, 1))


def test_context_bounds_are_inclusive() -> None:
    assert AUDIT.classify(-5.0, -5.0, 5.0) == "WITHIN_CONTEXT"
    assert AUDIT.classify(5.0, -5.0, 5.0) == "WITHIN_CONTEXT"
    assert AUDIT.classify(5.0001, -5.0, 5.0) == "OUTSIDE_CONTEXT"
