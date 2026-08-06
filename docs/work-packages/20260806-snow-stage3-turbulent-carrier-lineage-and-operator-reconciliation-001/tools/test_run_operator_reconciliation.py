from __future__ import annotations

import importlib.util
import sys
from pathlib import Path

import pytest

TOOL = Path(__file__).with_name("run_operator_reconciliation.py")
SPEC = importlib.util.spec_from_file_location("run_operator_reconciliation", TOOL)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


def synthetic_tuple(operator: str = "same_state_paired_carrier_v1") -> dict[str, object]:
    row: dict[str, object] = {
        "operator_id": operator,
        "hour_index": 0,
        "substep_index": 0,
        "elapsed_start_seconds": 0.0,
        "requested_seconds": 3_600.0,
        "evaluated_seconds": 3_600.0,
        "duration_seconds": 3_600.0,
        "applicable": True,
        "applicability_reason": "evaluated",
        "hourly_radiation_mj_m2": 1.0,
        "incoming_shortwave_w_m2": 1.0e6 / 3_600.0,
        "snow_albedo_fraction": 0.82,
        "net_shortwave_w_m2": 1.0e6 / 3_600.0 * 0.18,
        "subcanopy_longwave_w_m2": 250.0,
        "outgoing_longwave_w_m2": 275.0,
        "net_longwave_w_m2": -25.0,
        "rain_m": 0.0,
        "snowfall_geometric_m": 0.0,
        "rain_mass_flux_kg_m2_s": 0.0,
        "snow_mass_flux_kg_m2_s": 0.0,
        "surface_latent_heat_j_kg": None,
        "turbulent_termination_status": "zero_wind",
        "vapor_mass_flux_kg_m2_s": 0.0,
        "latent_flux_w_m2": 0.0,
        "sensible_flux_w_m2": 0.0,
        "precipitation_advected_flux_w_m2": 0.0,
        "complete_external_flux_w_m2": 25.0,
        "vapor_mass_exchange_kg_m2": 0.0,
        "active_ice_mass_before_kg_m2": 50.0,
        "active_ice_mass_after_kg_m2": 50.0,
        "total_ice_mass_before_kg_m2": 50.0,
        "total_ice_mass_after_kg_m2": 50.0,
        "active_depth_before_m": 0.2,
        "active_depth_after_m": 0.2,
        "active_density_before_kg_m3": 250.0,
        "active_density_after_kg_m3": 250.0,
        "active_cold_before_j_m2": 100.0,
        "active_cold_after_j_m2": 100.0,
        "total_cold_before_j_m2": 100.0,
        "total_cold_after_j_m2": 100.0,
        "surface_temperature_before_c": -2.0,
        "surface_temperature_after_c": -2.0,
    }
    if operator == "sequential_resolved_shadow_v1":
        row.update(
            {
                "melt_kg_m2": 1.0,
                "sublimation_kg_m2": 0.0,
                "deposition_kg_m2": 0.0,
                "total_ice_mass_after_kg_m2": 49.0,
                "active_cold_energy_change_j_m2": 10.0,
                "lower_cold_energy_change_j_m2": -2.0,
                "cold_content_export_j_m2": 2.0,
                "total_cold_after_j_m2": 90.0,
                "internal_active_lower_conduction_j_m2": 5.0,
                "legacy_sequential_complete_j_m2": 25.0 * 3_600.0 + 5.0,
            }
        )
    return row


def v6_row(tuple_row: dict[str, object], operator: str) -> dict[str, object]:
    statuses = [
        {"evaluated": index == 0, "reason": "evaluated" if index == 0 else "thin_pack_boundary_reached"}
        for index in range(24)
    ]
    return {
        "schema": "openwepp-r7h-direct-production-snow-trace-v6",
        "stage3_operator_reconciliation": {
            "schema_version": 6,
            "hourly_status": statuses,
            "tuples": [tuple_row],
        },
        "stage3_evaluation_operator_id": operator,
    }


def test_same_state_tuple_reconstructs_without_producer_totals() -> None:
    row = synthetic_tuple()
    MODULE.validate_tuple(row, "same_state_paired_carrier_v1")
    parsed = MODULE.validate_v6_row(v6_row(row, "same_state_paired_carrier_v1"), "same_state_paired_carrier_v1")
    assert parsed == [row]


def test_sequential_endpoint_and_legacy_bridge_reconstruct() -> None:
    row = synthetic_tuple("sequential_resolved_shadow_v1")
    MODULE.validate_tuple(row, "sequential_resolved_shadow_v1")


def test_unknown_schema_and_alias_fail_closed() -> None:
    row = v6_row(synthetic_tuple(), "same_state_paired_carrier_v1")
    row["schema"] = "openwepp-r7h-direct-production-snow-trace-v7"
    with pytest.raises(RuntimeError, match="unknown enabled schema"):
        MODULE.validate_v6_row(row, "same_state_paired_carrier_v1")
    tuple_row = synthetic_tuple()
    tuple_row["rain_mass_flux_kg_m2_s"] = 1.0
    with pytest.raises(RuntimeError, match="rain mass flux"):
        MODULE.validate_tuple(tuple_row, "same_state_paired_carrier_v1")


def test_prefix_split_and_zero_wind_reference_are_exact() -> None:
    rows = [
        {"duration_seconds": 60.0, "complete_external_flux_w_m2": 2.0},
        {"duration_seconds": 60.0, "complete_external_flux_w_m2": 4.0},
    ]
    assert MODULE.integrate_prefix(rows, 90.0, "complete_external_flux_w_m2") == 240.0
    result = MODULE.monin_obukhov(
        air_temperature_c=-5.0,
        surface_temperature_c=-6.0,
        air_vapor_pressure_pa=300.0,
        surface_vapor_pressure_pa=250.0,
        air_pressure_pa=80_000.0,
        wind_speed_m_s=0.0,
        z_t=5.0,
        z_q=5.0,
        z_u=5.0,
        z_0=0.005,
        max_iterations=40,
        tolerance=1.0e-4,
    )
    assert result == {"sensible": 0.0, "latent": 0.0}


def test_nonzero_turbulence_is_independently_reconstructed() -> None:
    row = synthetic_tuple()
    row.update(
        {
            "air_temperature_c": -4.0,
            "actual_vapor_pressure_pa": 380.0,
            "surface_vapor_pressure_pa": 320.0,
            "air_pressure_pa": 80_000.0,
            "wind_speed_m_s": 2.5,
            "air_temperature_height_m": 5.0,
            "vapor_pressure_height_m": 5.0,
            "wind_speed_height_m": 5.0,
            "aerodynamic_roughness_length_m": 0.005,
            "turbulent_max_iterations": 40,
            "turbulent_convergence_tolerance": 1.0e-4,
            "surface_latent_heat_j_kg": (
                2.5e6 - 2_955.73 * -2.0 + 333_600.0 + 166.67 * 2.0
            ),
            "turbulent_termination_status": "converged_stable",
        }
    )
    reconstructed = MODULE.monin_obukhov(
        air_temperature_c=-4.0,
        surface_temperature_c=-2.0,
        air_vapor_pressure_pa=380.0,
        surface_vapor_pressure_pa=320.0,
        air_pressure_pa=80_000.0,
        wind_speed_m_s=2.5,
        z_t=5.0,
        z_q=5.0,
        z_u=5.0,
        z_0=0.005,
        max_iterations=40,
        tolerance=1.0e-4,
    )
    row["sensible_flux_w_m2"] = reconstructed["sensible"]
    row["latent_flux_w_m2"] = reconstructed["latent"]
    row["vapor_mass_flux_kg_m2_s"] = (
        reconstructed["latent"] / float(row["surface_latent_heat_j_kg"])
    )
    row["vapor_mass_exchange_kg_m2"] = (
        float(row["vapor_mass_flux_kg_m2_s"]) * 3_600.0
    )
    row["complete_external_flux_w_m2"] = (
        float(row["net_shortwave_w_m2"])
        + float(row["net_longwave_w_m2"])
        + reconstructed["sensible"]
        + reconstructed["latent"]
    )
    MODULE.validate_tuple(row, "same_state_paired_carrier_v1")
    row["sensible_flux_w_m2"] = reconstructed["sensible"] + 0.01
    with pytest.raises(RuntimeError, match="independent sensible reconstruction"):
        MODULE.validate_tuple(row, "same_state_paired_carrier_v1")


def test_decision_fallback_is_exclusive() -> None:
    unresolved = {
        "medians_j_m2": {"S_j_m2": 0.0, "F_j_m2": 0.0, "Q_j_m2": 1.0, "legacy_Q_j_m2": MODULE.PREDECESSOR_MJ_M2 * 1.0e6},
        "initial_projection_difference_observed": False,
        "support_omission_ratio": 0.0,
    }
    assert MODULE.classify(unresolved) == ["MULTIFACTOR_UNRESOLVED"]
