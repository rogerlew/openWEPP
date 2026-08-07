from __future__ import annotations

import copy
import importlib.util
import sys
from pathlib import Path

import pytest

TOOL = Path(__file__).with_name("analyze_evolving_carrier.py")
SPEC = importlib.util.spec_from_file_location("analyze_evolving_carrier", TOOL)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


class FakePredecessor:
    FIXED_REFERENCE_FIELDS = ("wind_speed_m_s", "daylight")

    @staticmethod
    def require_same_bits(name: str, left: object, right: object) -> None:
        if not isinstance(left, float) or not isinstance(right, float):
            raise RuntimeError(f"{name} received non-float")
        if left != right:
            raise RuntimeError(f"{name} differs")

    @staticmethod
    def frozen_active_flux(same_state: dict[str, object], first_q: dict[str, object]) -> dict[str, object]:
        return {"state_marker": first_q["state_marker"]}

    @staticmethod
    def integrate_reconstructed_prefix(
        rows: list[dict[str, object]], seconds: float, field: str
    ) -> float:
        remaining = seconds
        total = 0.0
        for row in rows:
            duration = min(float(row["duration_seconds"]), remaining)
            total += float(row["_reconstructed"][field]) * duration
            remaining -= duration
            if remaining <= 0.0:
                break
        if remaining > 1.0e-12:
            raise RuntimeError("prefix exceeds support")
        return total


def joined_row(duration: float, scale: float) -> dict[str, object]:
    return {
        "duration_seconds": duration,
        "projection_id": "aligned_active_dynamic",
        "surface_temperature_before_c": -2.0,
        "active_cold_before_j_m2": 100.0,
        "active_ice_mass_before_kg_m2": 10.0,
        "active_depth_before_m": 0.1,
        "active_density_before_kg_m3": 100.0,
        "latent_exchange_velocity_m_s": 0.01,
        "sensible_exchange_velocity_m_s": 0.02,
        "specific_humidity_surface_kg_kg": 0.003,
        "specific_humidity_air_kg_kg": 0.002,
        "_reconstructed": {
            "shortwave": 1.0,
            "longwave": 2.0 * scale,
            "sensible": 3.0 * scale,
            "latent": 4.0 * scale,
            "advected": 5.0 * scale,
            "vapor_mass_flux": 0.01 * scale,
        },
    }


def q_tuple(*, raw: float = -0.25, active_ice: float = 1.0, external_flux: float = 500_000.0) -> dict[str, object]:
    duration = 1.0
    conduction = 20.0
    cold_before = 100.0
    vapor = MODULE.bounded_vapor(raw, active_ice)
    cold_after_conduction = cold_before - conduction
    surface_change = min(external_flux * duration, cold_after_conduction)
    active_change = conduction + surface_change
    complete = external_flux * duration + conduction
    excess = max(complete - active_change, 0.0)
    available = active_ice - float(vapor["sublimation"])
    melt = min(excess / MODULE.LATENT_HEAT_FUSION_J_KG, available)
    return {
        "_reconstructed": {"external": external_flux},
        "duration_seconds": duration,
        "active_cold_before_j_m2": cold_before,
        "internal_active_lower_conduction_j_m2": conduction,
        "vapor_mass_exchange_kg_m2": raw,
        "active_ice_mass_before_kg_m2": active_ice,
        "latent_flux_w_m2": raw * 2_834_000.0,
        "surface_latent_heat_j_kg": 2_834_000.0,
        "deposition_kg_m2": vapor["deposition"],
        "sublimation_kg_m2": vapor["sublimation"],
        "active_cold_energy_change_j_m2": active_change,
        "lower_cold_energy_change_j_m2": -conduction,
        "melt_kg_m2": melt,
        "legacy_sequential_complete_j_m2": complete,
        "energy_closure_residual_j_m2": 0.0,
        "total_ice_mass_before_kg_m2": active_ice,
        "total_ice_mass_after_kg_m2": active_ice
        - melt
        - float(vapor["sublimation"])
        + float(vapor["deposition"]),
        "active_depth_before_m": 0.1,
        "active_density_before_kg_m3": 100.0,
        "total_cold_before_j_m2": 100.0,
        "total_cold_after_j_m2": 0.0,
        "surface_temperature_before_c": -2.0,
        "after_surface_applicable": False,
    }


def test_raw_bounded_and_latent_views_remain_distinct() -> None:
    no_cap = MODULE.bounded_vapor(-0.25, 1.0)
    assert no_cap == {
        "deposition": 0.0,
        "sublimation": 0.25,
        "signed": -0.25,
        "capacity_truncated": False,
        "truncated_mass": 0.0,
    }
    truncated = MODULE.bounded_vapor(-2.0, 0.5)
    assert truncated["sublimation"] == 0.5
    assert truncated["signed"] == -0.5
    assert truncated["truncated_mass"] == 1.5
    assert truncated["capacity_truncated"] is True
    deposition = MODULE.bounded_vapor(0.75, 0.25)
    assert deposition["deposition"] == 0.75
    assert deposition["sublimation"] == 0.0


def test_operator_order_reserves_sublimation_before_melt() -> None:
    result = MODULE.validate_q_tuple(q_tuple())
    assert result["sublimation"] == 0.25
    assert result["ice_available"] == 0.75
    assert result["melt"] == 0.75
    assert result["unallocated"] == 249_720.0
    assert result["latent_raw"] == -708_500.0
    assert result["latent_bounded"] == -708_500.0


@pytest.mark.parametrize(
    ("field", "value", "message"),
    [
        ("sublimation_kg_m2", 0.20, "independent sublimation mismatch"),
        ("deposition_kg_m2", 0.10, "simultaneous"),
        ("melt_kg_m2", 0.70, "independent melt mismatch"),
        ("total_ice_mass_after_kg_m2", 0.10, "independent total-mass endpoint mismatch"),
        ("surface_temperature_before_c", 0.01, "surface temperature is out of domain"),
        ("latent_flux_w_m2", -1.0, "raw vapor/latent identity mismatch"),
    ],
)
def test_producer_disagreement_and_state_domain_fail_closed(
    field: str, value: float, message: str
) -> None:
    row = q_tuple()
    row[field] = value
    with pytest.raises(RuntimeError, match=message):
        MODULE.validate_q_tuple(row)


def test_simultaneous_transfer_and_endpoint_preserving_alias_are_rejected() -> None:
    simultaneous = q_tuple()
    simultaneous["deposition_kg_m2"] = 0.1
    with pytest.raises(RuntimeError, match="simultaneous"):
        MODULE.validate_q_tuple(simultaneous)

    wrong_direction = q_tuple()
    wrong_direction["sublimation_kg_m2"] = 0.0
    wrong_direction["deposition_kg_m2"] = 0.25
    with pytest.raises(RuntimeError, match="raw-negative vapor is labeled deposition"):
        MODULE.validate_q_tuple(wrong_direction)

    aliased = q_tuple()
    aliased["sublimation_kg_m2"] = 0.75
    aliased["melt_kg_m2"] = 0.25
    assert aliased["total_ice_mass_after_kg_m2"] == 0.0
    with pytest.raises(RuntimeError, match="independent sublimation mismatch"):
        MODULE.validate_q_tuple(aliased)


def test_invalid_evidence_precedes_physical_classification() -> None:
    combined = q_tuple()
    combined["surface_temperature_before_c"] = 0.01
    combined["melt_kg_m2"] = 0.70
    with pytest.raises(RuntimeError, match="independent melt mismatch") as captured:
        MODULE.validate_q_tuple(combined)
    assert not isinstance(captured.value, MODULE.PhysicalPlausibilityFailure)


def test_finite_physical_failure_retains_nonexclusive_reconstruction() -> None:
    row = q_tuple(raw=-2.0, active_ice=0.5, external_flux=0.0)
    row["surface_temperature_before_c"] = 0.01
    with pytest.raises(MODULE.PhysicalPlausibilityFailure) as captured:
        MODULE.validate_q_tuple(row)
    reconstructed = captured.value.reconstructed
    assert reconstructed is not None
    assert reconstructed["capacity_truncated"]
    assert reconstructed["truncated_mass"] == 1.5

    summaries = [
        {
            "site": site,
            "role": "CANONICAL_SCREEN",
            "capacity_truncated_tuple_count": int(index == 0),
            "active_state_plausibility_failure_tuple_count": int(index == 0),
        }
        for index, site in enumerate(sorted(MODULE.CANONICAL_SITES))
    ]
    classes = MODULE.decision_classes(summaries)
    assert "VAPOR_OPPORTUNITY_TRANSFER_MISMATCH" in classes
    assert "ACTIVE_STATE_EVOLUTION_PLAUSIBILITY_FAIL" in classes


def test_nonfinite_state_is_invalid_evidence_and_producer_checks_still_precede() -> None:
    physical = q_tuple()
    physical["total_ice_mass_after_kg_m2"] = float("nan")
    with pytest.raises(RuntimeError, match="nonfinite invalid evidence"):
        MODULE.reject_nonfinite_q_state(physical)

    combined = dict(physical)
    combined["melt_kg_m2"] = -1.0
    with pytest.raises(RuntimeError, match="producer bounded transfer or melt is negative"):
        MODULE.reject_nonfinite_q_state(combined)


def test_nonfinite_negative_and_missing_primitive_evidence_fail_closed() -> None:
    for field, value in (
        ("melt_kg_m2", float("nan")),
        ("active_ice_mass_before_kg_m2", -1.0),
        ("total_cold_after_j_m2", -1.0),
    ):
        row = q_tuple()
        row[field] = value
        expected = MODULE.PhysicalPlausibilityFailure if field != "melt_kg_m2" else RuntimeError
        with pytest.raises(expected):
            MODULE.validate_q_tuple(row)
    row = q_tuple()
    del row["_reconstructed"]
    with pytest.raises(RuntimeError, match="lacks independent primitive"):
        MODULE.validate_q_tuple(row)


def test_identity_and_na_are_typed_not_numeric_aliases() -> None:
    MODULE.validate_trace_identity({"day_index": 4, "lane_index": 0}, 4)
    with pytest.raises(RuntimeError, match="identity mismatch"):
        MODULE.validate_trace_identity({"day_index": 5, "lane_index": 0}, 4)
    with pytest.raises(RuntimeError, match="identity mismatch"):
        MODULE.validate_trace_identity({"day_index": 4, "lane_index": 1}, 4)

    na = {"melt_kg_m2": None, "sublimation_kg_m2": None, "deposition_kg_m2": None}
    MODULE.validate_nonmutating_transfer(na)
    for field in na:
        numeric_zero = dict(na)
        numeric_zero[field] = 0.0
        with pytest.raises(RuntimeError, match="must be N/A"):
            MODULE.validate_nonmutating_transfer(numeric_zero)
    missing = dict(na)
    del missing["melt_kg_m2"]
    with pytest.raises(RuntimeError, match="missing required N/A"):
        MODULE.validate_nonmutating_transfer(missing)


def test_cross_lane_identity_and_first_q_frozen_state_are_explicit() -> None:
    top = {
        "stage3_evaluation_source_fingerprint_fnv1a64": "1",
        "stage3_evaluation_forcing_fingerprint_fnv1a64": "2",
        "stage3_evaluation_geometry_fingerprint_fnv1a64": "3",
        "stage3_evaluation_non_formulation_fingerprint_fnv1a64": "paired",
    }
    s = {
        "wind_speed_m_s": 5.0,
        "daylight": True,
        "projection_id": "whole_column_immutable",
        "state_marker": "S",
    }
    q = {
        "wind_speed_m_s": 5.0,
        "daylight": True,
        "projection_id": "aligned_active_dynamic",
        "state_marker": "Q",
    }
    sequential_top = dict(top)
    sequential_top["stage3_evaluation_non_formulation_fingerprint_fnv1a64"] = "sequential"
    MODULE.validate_joined_identity(top, sequential_top, [s], [q], FakePredecessor)
    frozen = MODULE.construct_frozen_active(s, q, FakePredecessor)
    assert frozen["state_marker"] == "Q"

    changed = dict(sequential_top)
    changed["stage3_evaluation_forcing_fingerprint_fnv1a64"] = "different"
    with pytest.raises(RuntimeError, match="cross-lane.*forcing.*mismatch"):
        MODULE.validate_joined_identity(top, changed, [s], [q], FakePredecessor)
    q_wrong_fixed = dict(q)
    q_wrong_fixed["wind_speed_m_s"] = 6.0
    with pytest.raises(RuntimeError, match="joined fixed wind_speed"):
        MODULE.validate_joined_identity(top, sequential_top, [s], [q_wrong_fixed], FakePredecessor)
    q_wrong_boolean = dict(q)
    q_wrong_boolean["daylight"] = False
    with pytest.raises(RuntimeError, match="joined fixed daylight mismatch"):
        MODULE.validate_joined_identity(top, sequential_top, [s], [q_wrong_boolean], FakePredecessor)
    aliased_top = dict(top)
    with pytest.raises(RuntimeError, match="non-formulation fingerprint alias"):
        MODULE.validate_joined_identity(top, aliased_top, [s], [q], FakePredecessor)
    s_alias = dict(s)
    s_alias["projection_id"] = "aligned_active_dynamic"
    with pytest.raises(RuntimeError, match="S whole-column"):
        MODULE.construct_frozen_active(s_alias, q, FakePredecessor)


def test_empty_support_preserves_selected_or_inactive_fingerprint_state() -> None:
    base = {
        "stage3_evaluation_source_fingerprint_fnv1a64": "1",
        "stage3_evaluation_forcing_fingerprint_fnv1a64": "2",
        "stage3_evaluation_geometry_fingerprint_fnv1a64": "3",
        "stage3_evaluation_non_formulation_fingerprint_fnv1a64": "paired-selected",
    }
    selected = dict(base)
    selected["stage3_evaluation_non_formulation_fingerprint_fnv1a64"] = "q-selected"
    MODULE.validate_joined_identity(base, selected, [], [], FakePredecessor)

    inactive = dict(base)
    inactive["stage3_evaluation_non_formulation_fingerprint_fnv1a64"] = "0000000000000000"
    MODULE.validate_joined_identity(inactive, dict(inactive), [], [], FakePredecessor)

    with pytest.raises(RuntimeError, match="sentinel applicability mismatch"):
        MODULE.validate_joined_identity(inactive, selected, [], [], FakePredecessor)


def test_valid_capacity_truncation_is_a_physical_class_not_invalid_evidence() -> None:
    row = q_tuple(raw=-2.0, active_ice=0.5, external_flux=0.0)
    result = MODULE.validate_q_tuple(row)
    assert result["capacity_truncated"]
    assert result["truncated_mass"] == 1.5
    summaries = [
        {
            "site": site,
            "role": "CANONICAL_SCREEN",
            "capacity_truncated_tuple_count": int(index == 0),
        }
        for index, site in enumerate(sorted(MODULE.CANONICAL_SITES))
    ]
    classes = MODULE.decision_classes(summaries)
    assert "VAPOR_OPPORTUNITY_TRANSFER_MISMATCH" in classes
    assert "MULTIFACTOR_OR_INCONCLUSIVE" in classes
    assert "EVOLVING_CARRIER_PLAUSIBILITY_PASS" not in classes


def test_decision_has_no_result_selected_threshold_input() -> None:
    parameters = MODULE.decision_classes.__annotations__
    assert set(parameters) == {"site_summaries", "return"}
    summaries = [
        {
            "site": site,
            "role": "CANONICAL_SCREEN",
            "capacity_truncated_tuple_count": 0,
        }
        for site in sorted(MODULE.CANONICAL_SITES)
    ]
    classes = MODULE.decision_classes(summaries)
    assert "VAPOR_OPPORTUNITY_TRANSFER_MISMATCH" not in classes
    assert classes == [
        "WIND_FORCING_EXPOSURE_UNRESOLVED",
        "MULTIFACTOR_OR_INCONCLUSIVE",
    ]


def test_snowbird_cannot_change_canonical_decision() -> None:
    canonical = [
        {
            "site": site,
            "role": "CANONICAL_SCREEN",
            "capacity_truncated_tuple_count": 0,
        }
        for site in sorted(MODULE.CANONICAL_SITES)
    ]
    snowbird = {
        "site": "snotel_snowbird_ut",
        "role": "DEVELOPMENT_ONLY_NON_DECISIVE_DIAGNOSTIC",
        "capacity_truncated_tuple_count": 999,
    }
    assert MODULE.decision_classes(canonical) == MODULE.decision_classes(canonical + [snowbird])
    with pytest.raises(RuntimeError, match="canonical decision cohort is incomplete"):
        MODULE.decision_classes(canonical[:2])


def test_three_way_common_prefix_reduction_retains_omitted_support() -> None:
    s_rows = [joined_row(3_600.0, 1.0)]
    q_rows = [joined_row(1_800.0, 2.0)]
    frozen = {
        "shortwave": 1.0,
        "longwave": 3.0,
        "sensible": 4.0,
        "latent": 5.0,
        "advected": 6.0,
        "vapor_mass_flux": 0.02,
    }
    reduced, counts = MODULE.reduce_joined_hour(
        s_rows, q_rows, frozen, q_rows[0], FakePredecessor
    )
    assert reduced["common_support_seconds"] == 1_800.0
    assert counts["partial_support_hour_count"] == 1
    assert reduced["S_shortwave_j_m2"] == 1_800.0
    assert reduced["F_shortwave_j_m2"] == 1_800.0
    assert reduced["Q_shortwave_j_m2"] == 1_800.0
    assert reduced["delta_evolution_longwave_j_m2"] == 1_800.0
    assert reduced["delta_evolution_raw_vapor_kg_m2"] == 0.0
    assert reduced["omitted_magnitude_j_m2"] > 0.0


def test_join_rejects_shortwave_drift_and_labels_unmatched_support() -> None:
    s_rows = [joined_row(3_600.0, 1.0)]
    q_drift = [joined_row(3_600.0, 2.0)]
    q_drift[0]["_reconstructed"]["shortwave"] = 2.0
    frozen = {
        "shortwave": 1.0,
        "longwave": 2.0,
        "sensible": 3.0,
        "latent": 4.0,
        "advected": 5.0,
        "vapor_mass_flux": 0.01,
    }
    with pytest.raises(RuntimeError, match="S/Q shortwave invariance"):
        MODULE.reduce_joined_hour(s_rows, q_drift, frozen, q_drift[0], FakePredecessor)
    reduced, counts = MODULE.reduce_joined_hour(s_rows, [], frozen, q_drift[0], FakePredecessor)
    assert counts["unmatched_hour_count"] == 1
    assert reduced["omitted_magnitude_j_m2"] == reduced["all_evaluated_magnitude_j_m2"]


def test_custody_hash_fails_closed(tmp_path: Path) -> None:
    evidence = tmp_path / "evidence.txt"
    evidence.write_bytes(b"frozen evidence\n")
    expected = MODULE.hashlib.sha256(evidence.read_bytes()).hexdigest()
    MODULE.require_sha256(evidence, expected, "test evidence")
    evidence.write_bytes(b"changed evidence\n")
    with pytest.raises(RuntimeError, match="custody mismatch"):
        MODULE.require_sha256(evidence, expected, "test evidence")
    with pytest.raises(RuntimeError, match="custody mismatch"):
        MODULE.require_sha256(evidence, "not-a-hash", "test evidence")


@pytest.mark.parametrize(
    ("verify", "exists", "status", "message"),
    [
        (False, False, "", "verify-retained is mandatory"),
        (True, True, "", "refusing to overwrite"),
        (True, False, " M tracked", "clean tracked worktree"),
    ],
)
def test_result_execution_preconditions_fail_closed(
    verify: bool, exists: bool, status: str, message: str
) -> None:
    with pytest.raises(RuntimeError, match=message):
        MODULE.validate_execution_preconditions(verify, exists, status)
    MODULE.validate_execution_preconditions(True, False, "")


def test_material_support_failure_cannot_be_erased_by_median() -> None:
    annual = [
        {"water_year": 2001, "all_evaluated_magnitude_j_m2": 100.0, "omitted_magnitude_j_m2": 0.0},
        {"water_year": 2002, "all_evaluated_magnitude_j_m2": 100.0, "omitted_magnitude_j_m2": 0.0},
        {"water_year": 2003, "all_evaluated_magnitude_j_m2": 100.0, "omitted_magnitude_j_m2": 6.0},
    ]
    result = MODULE.support_materiality(annual)
    assert result["support_omission_ratio_median"] == 0.0
    assert result["support_materiality_pass"] is False
    assert result["support_materiality_failing_water_years"] == [2003]


def test_zero_support_denominator_is_na_and_blocks_pass() -> None:
    result = MODULE.support_materiality(
        [{"water_year": 2004, "all_evaluated_magnitude_j_m2": 0.0, "omitted_magnitude_j_m2": 0.0}]
    )
    assert result["support_omission_ratio_median"] is None
    assert result["support_omission_ratio_maximum"] is None
    assert result["support_materiality_pass"] is False
    assert result["support_materiality_not_evaluable_water_years"] == [2004]


def test_median_reduction_uses_water_year_rows_not_median_of_medians() -> None:
    samples = [1.0, 100.0, 101.0]
    assert MODULE.statistics.median(samples) == 100.0
    grouped_medians = [MODULE.statistics.median(samples[:1]), MODULE.statistics.median(samples[1:])]
    assert MODULE.statistics.median(grouped_medians) != MODULE.statistics.median(samples)


def test_mutated_copy_does_not_change_source_fixture() -> None:
    source = q_tuple()
    mutated = copy.deepcopy(source)
    mutated["melt_kg_m2"] = 999.0
    assert source["melt_kg_m2"] != mutated["melt_kg_m2"]
