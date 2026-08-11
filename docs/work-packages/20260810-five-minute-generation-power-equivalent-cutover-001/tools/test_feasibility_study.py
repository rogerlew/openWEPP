from __future__ import annotations

import importlib.util
import sys
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("feasibility_study.py")
SPEC = importlib.util.spec_from_file_location("feasibility_study", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
STUDY = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = STUDY
SPEC.loader.exec_module(STUDY)


def test_constant_power_volume_is_identity() -> None:
    rates = tuple([2.0e-6] * 12)
    for exponent in [4.0 / 3.0, 1.5, 2.0]:
        rate, duration = STUDY.reduction(rates, exponent, "power_volume")
        assert abs(rate - 2.0e-6) <= 1.0e-18
        assert abs(duration - 3600.0) <= 1.0e-9


def test_power_volume_closes_both_moments() -> None:
    rates = STUDY.shapes()["one_pulse"]
    for exponent in [4.0 / 3.0, 1.5, 2.0]:
        rate, duration = STUDY.reduction(rates, exponent, "power_volume")
        volume = sum(value * STUDY.DT_S for value in rates)
        power = sum(value**exponent * STUDY.DT_S for value in rates)
        assert abs(rate * duration - volume) <= 1.0e-15
        assert abs(rate**exponent * duration - power) / power <= 1.0e-14


def test_fixed_hour_power_mean_does_not_preserve_pulse_volume() -> None:
    rates = STUDY.shapes()["one_pulse"]
    rate, duration = STUDY.reduction(rates, 1.5, "fixed_hour")
    volume = sum(value * STUDY.DT_S for value in rates)
    assert abs(rate * duration - volume) / volume > 0.1


def test_study_is_prospective_and_finds_no_admitted_candidate() -> None:
    result = STUDY.run_study()
    assert result["topanga_outcomes_opened"] is False
    assert result["record_count"] > 0
    assert result["disposition"] == "NO_FIXED_EXPONENT_ADMITTED"
    assert result["admitted_candidates"] == []


def test_high_reynolds_shield_matches_production_expression() -> None:
    reynolds = 2000.0
    slope = (STUDY.math.log(0.057) - STUDY.math.log(0.055)) / (
        STUDY.math.log(1000.0) - STUDY.math.log(400.0)
    )
    expected = STUDY.math.exp(0.057 + slope * (STUDY.math.log(reynolds) - STUDY.math.log(1000.0)))
    assert abs(STUDY.shield(reynolds) - expected) <= 1.0e-15
