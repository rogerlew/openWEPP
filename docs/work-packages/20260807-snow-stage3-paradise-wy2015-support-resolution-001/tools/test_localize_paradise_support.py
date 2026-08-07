from importlib.util import module_from_spec, spec_from_file_location
from pathlib import Path


PATH = Path(__file__).with_name("localize_paradise_support.py")
SPEC = spec_from_file_location("localize_paradise_support_tested", PATH)
assert SPEC is not None and SPEC.loader is not None
MODULE = module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def row(duration: float, before: float = 2.0, after: float = 1.0) -> dict:
    return {
        "duration_seconds": duration,
        "applicability_reason": "evaluated",
        "after_surface_applicability_reason": "resolved_surface",
        "after_surface_applicable": True,
        "total_ice_mass_before_kg_m2": before,
        "total_ice_mass_after_kg_m2": after,
    }


def test_support_classes_are_mutually_exclusive() -> None:
    assert MODULE.classify([row(3600.0)], []) == "UNMATCHED_S_ONLY"
    assert MODULE.classify([], [row(3600.0)]) == "UNMATCHED_Q_ONLY"
    assert MODULE.classify([row(1800.0)], [row(3600.0)]) == "PARTIAL_COMMON_SUPPORT"
    assert MODULE.classify([row(3600.0)], [row(3600.0)]) is None
    assert MODULE.classify([], []) is None


def test_tuple_summary_preserves_state_boundary() -> None:
    summary = MODULE.tuple_summary([row(60.0, 3.0, 2.5), row(60.0, 2.5, 2.0)])
    assert summary["tuple_count"] == 2
    assert summary["support_seconds"] == 120.0
    assert summary["ice_before_kg_m2"] == 3.0
    assert summary["ice_after_kg_m2"] == 2.0


def test_hourly_status_is_exactly_24_entries() -> None:
    row_value = {
        "stage3_operator_reconciliation": {
            "hourly_status": [
                {"evaluated": index < 3, "reason": "evaluated" if index < 3 else "no_snow"}
                for index in range(24)
            ]
        }
    }
    assert MODULE.hourly_status(row_value, 2) == {"evaluated": True, "reason": "evaluated"}
    assert MODULE.hourly_status(row_value, 3) == {"evaluated": False, "reason": "no_snow"}


def test_threshold_cannot_alias_observed_ratio() -> None:
    result = {
        "counts": {"unmatched_hour_count": 1, "partial_support_hour_count": 0},
        "affected_hours": [{}],
        "totals": {"omitted_magnitude_j_m2": 5.0},
    }
    freeze = {
        "support_threshold": 0.1,
        "expected_parent": {
            "unmatched_hour_count": 1,
            "partial_support_hour_count": 0,
            "omitted_magnitude_j_m2": 5.0,
            "support_omission_ratio": 0.1,
        },
    }
    try:
        MODULE.validate(result, freeze)
    except RuntimeError as error:
        assert "threshold was aliased" in str(error)
    else:
        raise AssertionError("threshold alias must fail")
