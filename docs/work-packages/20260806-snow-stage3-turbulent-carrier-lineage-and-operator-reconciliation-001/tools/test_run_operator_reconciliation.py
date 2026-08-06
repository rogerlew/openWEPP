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
    surface_c = -2.0
    air_c = -4.0
    dewpoint_c = -5.0
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
        "source_fingerprint_fnv1a64": "0000000000000011",
        "forcing_fingerprint_fnv1a64": "0000000000000022",
        "geometry_fingerprint_fnv1a64": "0000000000000033",
        "effective_input_fingerprint_fnv1a64": "0000000000000044",
        "projection_id": "whole_column_immutable",
        "active_layer_prefix_count_before": 1,
        "total_layer_count_before": 1,
        "active_layer_state_fingerprint_before_fnv1a64": "0000000000000055",
        "total_layer_state_fingerprint_before_fnv1a64": "0000000000000066",
        "active_layer_prefix_count_after": 1,
        "total_layer_count_after": 1,
        "active_layer_state_fingerprint_after_fnv1a64": "0000000000000055",
        "total_layer_state_fingerprint_after_fnv1a64": "0000000000000066",
        "after_surface_applicable": True,
        "after_surface_applicability_reason": "resolved_surface",
        "air_temperature_c": air_c,
        "dewpoint_c": dewpoint_c,
        "wind_speed_m_s": 0.0,
        "air_pressure_pa": 80_000.0,
        "hourly_radiation_mj_m2": 1.0,
        "daily_solar_radiation_mj_m2": 10.0,
        "daily_extraterrestrial_radiation_mj_m2": 20.0,
        "daylight": True,
        "canopy_cover_fraction": 0.5,
        "incoming_shortwave_w_m2": 1.0e6 / 3_600.0,
        "snow_albedo_fraction": 0.82,
        "snow_albedo_source_id": "stage3_default_snow_albedo_0p82",
        "snow_albedo_model_id": None,
        "snow_albedo_accumulated_positive_temperature_c_day": None,
        "net_shortwave_w_m2": 1.0e6 / 3_600.0 * 0.18,
        "actual_vapor_pressure_pa": MODULE.saturation_vapor_pressure(dewpoint_c),
        "surface_vapor_pressure_pa": MODULE.saturation_vapor_pressure(surface_c),
        "longwave_model_id": MODULE.LONGWAVE_MODEL,
        "sublimation_model_id": MODULE.SUBLIMATION_MODEL,
        "rain_m": 0.0,
        "snowfall_geometric_m": 0.0,
        "rain_mass_flux_kg_m2_s": 0.0,
        "snow_mass_flux_kg_m2_s": 0.0,
        "rain_temperature_c": -3.0,
        "snow_temperature_c": -3.0,
        "rain_specific_heat_j_kg_k": 4_217.7 - 2.55 * -3.0,
        "snow_specific_heat_j_kg_k": MODULE.CALORIE_TO_JOULE * (0.024_928 + 0.001_76 * (MODULE.FREEZE_K - 3.0)) / 0.001,
        "air_temperature_height_m": 5.0,
        "vapor_pressure_height_m": 5.0,
        "wind_speed_height_m": 5.0,
        "aerodynamic_roughness_length_m": 0.005,
        "turbulent_max_iterations": 40,
        "turbulent_convergence_tolerance": 1.0e-4,
        "surface_latent_heat_j_kg": None,
        "turbulent_termination_status": "zero_wind",
        "stability_class": "zero_wind",
        "turbulent_iterations": 0,
        "obukhov_length_m": None,
        "psi_momentum": 0.0,
        "psi_sensible": 0.0,
        "psi_latent": 0.0,
        "friction_velocity_m_s": 0.0,
        "displacement_height_m": None,
        "log_momentum": None,
        "log_sensible": None,
        "log_latent": None,
        "sensible_exchange_velocity_m_s": None,
        "latent_exchange_velocity_m_s": None,
        "air_density_kg_m3": None,
        "air_potential_temperature_k": None,
        "surface_temperature_k": None,
        "specific_humidity_air_kg_kg": None,
        "specific_humidity_surface_kg_kg": None,
        "vapor_mass_flux_kg_m2_s": 0.0,
        "latent_flux_w_m2": 0.0,
        "sensible_flux_w_m2": 0.0,
        "precipitation_advected_flux_w_m2": 0.0,
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
        "surface_temperature_before_c": surface_c,
        "surface_temperature_after_c": surface_c,
        "melt_kg_m2": None,
        "sublimation_kg_m2": None,
        "deposition_kg_m2": None,
        "active_cold_energy_change_j_m2": None,
        "lower_cold_energy_change_j_m2": None,
        "cold_content_export_j_m2": None,
        "internal_active_lower_conduction_j_m2": None,
        "legacy_sequential_complete_j_m2": None,
        "energy_closure_residual_j_m2": None,
    }
    longwave = MODULE.reconstruct_longwave(row, surface_c)
    row.update({
        "longwave_cloud_fraction": longwave["cloud"],
        "sky_view_fraction": longwave["sky_view"],
        "atmospheric_longwave_w_m2": longwave["atmospheric"],
        "canopy_longwave_w_m2": longwave["canopy"],
        "subcanopy_longwave_w_m2": longwave["subcanopy"],
        "outgoing_longwave_w_m2": longwave["outgoing"],
        "net_longwave_w_m2": longwave["net"],
        "complete_external_flux_w_m2": float(row["net_shortwave_w_m2"]) + longwave["net"],
    })
    if operator == "sequential_resolved_shadow_v1":
        row.update(
            {
                "projection_id": "aligned_active_dynamic",
                "melt_kg_m2": 1.0,
                "sublimation_kg_m2": 0.0,
                "deposition_kg_m2": 0.0,
                "total_ice_mass_after_kg_m2": 49.0,
                "active_cold_energy_change_j_m2": 10.0,
                "lower_cold_energy_change_j_m2": -2.0,
                "cold_content_export_j_m2": 2.0,
                "total_cold_after_j_m2": 90.0,
                "internal_active_lower_conduction_j_m2": 5.0,
                "legacy_sequential_complete_j_m2": float(row["complete_external_flux_w_m2"]) * 3_600.0 + 5.0,
                "energy_closure_residual_j_m2": 0.0,
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
        "stage3_evaluation_source_fingerprint_fnv1a64": tuple_row["source_fingerprint_fnv1a64"],
        "stage3_evaluation_forcing_fingerprint_fnv1a64": tuple_row["forcing_fingerprint_fnv1a64"],
        "stage3_evaluation_geometry_fingerprint_fnv1a64": tuple_row["geometry_fingerprint_fnv1a64"],
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


def test_historical_v4_v5_dispatch_is_explicit_and_cannot_alias_v6() -> None:
    for version in (4, 5):
        row = {"schema": f"openwepp-r7h-direct-production-snow-trace-v{version}"}
        assert MODULE.dispatch_trace_row(row, "historical") == (version, [])
        row["stage3_operator_reconciliation"] = {}
        with pytest.raises(RuntimeError, match="aliases schema-v6"):
            MODULE.dispatch_trace_row(row, "historical")


def test_schema_v6_rejects_fingerprint_reason_and_global_order_aliases() -> None:
    row = v6_row(synthetic_tuple(), "same_state_paired_carrier_v1")
    row["stage3_operator_reconciliation"]["hourly_status"][1]["reason"] = "invented"
    with pytest.raises(RuntimeError, match="hourly reason"):
        MODULE.validate_v6_row(row, "same_state_paired_carrier_v1")
    row = v6_row(synthetic_tuple(), "same_state_paired_carrier_v1")
    row["stage3_evaluation_source_fingerprint_fnv1a64"] = "different"
    with pytest.raises(RuntimeError, match="custody mismatch"):
        MODULE.validate_v6_row(row, "same_state_paired_carrier_v1")


def test_same_state_requires_exact_bits_counts_and_null_transfers() -> None:
    row = synthetic_tuple()
    row["active_ice_mass_after_kg_m2"] = float(row["active_ice_mass_before_kg_m2"]) + 1.0e-14
    with pytest.raises(RuntimeError, match="IEEE-754"):
        MODULE.validate_tuple(row, "same_state_paired_carrier_v1")
    row = synthetic_tuple()
    row["melt_kg_m2"] = 0.0
    with pytest.raises(RuntimeError, match="must be null"):
        MODULE.validate_tuple(row, "same_state_paired_carrier_v1")


def test_sequential_terminal_surface_requires_all_active_nulls() -> None:
    row = synthetic_tuple("sequential_resolved_shadow_v1")
    row["after_surface_applicable"] = False
    row["after_surface_applicability_reason"] = "post_substep_no_resolved_surface"
    for field in (
        "active_layer_prefix_count_after",
        "active_layer_state_fingerprint_after_fnv1a64",
        "active_ice_mass_after_kg_m2",
        "active_depth_after_m",
        "active_density_after_kg_m3",
        "active_cold_after_j_m2",
        "surface_temperature_after_c",
    ):
        row[field] = None
    MODULE.validate_tuple(row, "sequential_resolved_shadow_v1")
    row["active_depth_after_m"] = 0.0
    with pytest.raises(RuntimeError, match="terminal after-surface"):
        MODULE.validate_tuple(row, "sequential_resolved_shadow_v1")


def test_projection_difference_uses_fingerprint_membership_and_exact_state_bits() -> None:
    same = synthetic_tuple()
    sequential = dict(same)
    assert not MODULE.projection_differs(same, sequential)
    sequential["active_layer_prefix_count_before"] = 2
    assert MODULE.projection_differs(same, sequential)
    sequential = dict(same)
    sequential["active_depth_before_m"] = float(same["active_depth_before_m"]) + 1.0e-14
    assert MODULE.projection_differs(same, sequential)


def test_advection_and_longwave_are_not_accepted_as_producer_totals() -> None:
    row = synthetic_tuple()
    row["precipitation_advected_flux_w_m2"] = 0.1
    with pytest.raises(RuntimeError, match="precipitation advection"):
        MODULE.validate_tuple(row, "same_state_paired_carrier_v1")
    row = synthetic_tuple()
    row["atmospheric_longwave_w_m2"] = float(row["atmospheric_longwave_w_m2"]) + 0.1
    with pytest.raises(RuntimeError, match="atmospheric_longwave"):
        MODULE.validate_tuple(row, "same_state_paired_carrier_v1")


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
    assert result["sensible"] == 0.0
    assert result["latent"] == 0.0
    assert result["status"] == "zero_wind"


def test_nonzero_turbulence_is_independently_reconstructed() -> None:
    row = synthetic_tuple()
    row.update(
        {
            "air_temperature_c": -4.0,
            "air_pressure_pa": 80_000.0,
            "wind_speed_m_s": 2.5,
            "air_temperature_height_m": 5.0,
            "vapor_pressure_height_m": 5.0,
            "wind_speed_height_m": 5.0,
            "aerodynamic_roughness_length_m": 0.005,
            "turbulent_max_iterations": 40,
            "turbulent_convergence_tolerance": 1.0e-4,
        }
    )
    reconstructed = MODULE.monin_obukhov(
        air_temperature_c=-4.0,
        surface_temperature_c=-2.0,
        air_vapor_pressure_pa=float(row["actual_vapor_pressure_pa"]),
        surface_vapor_pressure_pa=float(row["surface_vapor_pressure_pa"]),
        air_pressure_pa=80_000.0,
        wind_speed_m_s=2.5,
        z_t=5.0,
        z_q=5.0,
        z_u=5.0,
        z_0=0.005,
        max_iterations=40,
        tolerance=1.0e-4,
    )
    row.update({
        "sensible_flux_w_m2": reconstructed["sensible"],
        "latent_flux_w_m2": reconstructed["latent"],
        "vapor_mass_flux_kg_m2_s": reconstructed["mass"],
        "surface_latent_heat_j_kg": reconstructed["latent_heat"],
        "turbulent_termination_status": reconstructed["status"],
        "stability_class": reconstructed["class"],
        "turbulent_iterations": reconstructed["iterations"],
        "obukhov_length_m": reconstructed["obukhov"],
        "psi_momentum": reconstructed["psi_momentum"],
        "psi_sensible": reconstructed["psi_sensible"],
        "psi_latent": reconstructed["psi_latent"],
        "friction_velocity_m_s": reconstructed["friction"],
        "displacement_height_m": reconstructed["displacement"],
        "log_momentum": reconstructed["log_momentum"],
        "log_sensible": reconstructed["log_sensible"],
        "log_latent": reconstructed["log_latent"],
        "sensible_exchange_velocity_m_s": reconstructed["sensible_exchange"],
        "latent_exchange_velocity_m_s": reconstructed["latent_exchange"],
        "air_density_kg_m3": reconstructed["density"],
        "air_potential_temperature_k": reconstructed["potential_temperature"],
        "surface_temperature_k": reconstructed["surface_temperature_k"],
        "specific_humidity_air_kg_kg": reconstructed["q_air"],
        "specific_humidity_surface_kg_kg": reconstructed["q_surface"],
    })
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
    with pytest.raises(RuntimeError, match="independent sensible_flux"):
        MODULE.validate_tuple(row, "same_state_paired_carrier_v1")


def test_decision_fallback_is_exclusive() -> None:
    unresolved = {
        "medians_j_m2": {"S_j_m2": 0.0, "F_j_m2": 0.0, "Q_j_m2": 1.0, "Q_all_j_m2": 1.0, "legacy_Q_all_j_m2": MODULE.PREDECESSOR_MJ_M2 * 1.0e6},
        "initial_projection_difference_observed": False,
        "support_omission_ratio": 0.0,
        "support_delta_sign_changed": False,
        "predecessor_bridge_pass": True,
        "sample_count": 35,
        "eligible_sample_count": 35,
    }
    assert MODULE.classify(unresolved) == ["MULTIFACTOR_UNRESOLVED"]


def test_decision_rule_orders_projection_evolution_and_support_classes() -> None:
    common = {
        "initial_projection_difference_observed": True,
        "support_omission_ratio": 0.06,
        "support_delta_sign_changed": False,
        "predecessor_bridge_pass": True,
        "sample_count": 35,
        "eligible_sample_count": 35,
    }
    projection = common | {
        "medians_j_m2": {
            "S_j_m2": -2.0,
            "F_j_m2": 2.0,
            "Q_j_m2": 3.0,
            "Q_all_j_m2": 3.0,
            "legacy_Q_all_j_m2": MODULE.PREDECESSOR_MJ_M2 * 1.0e6,
        }
    }
    assert MODULE.classify(projection) == [
        "INITIAL_CONTROL_VOLUME_PROJECTION_DIFFERENCE",
        "INITIAL_CONTROL_VOLUME_PROJECTION_RECONCILES_SIGN_CONTRADICTION",
        "SUPPORT_CENSORING_MATERIALLY_CONTRIBUTES",
    ]
    evolution = projection | {
        "initial_projection_difference_observed": False,
        "support_omission_ratio": 0.0,
        "support_delta_sign_changed": False,
        "medians_j_m2": projection["medians_j_m2"] | {"F_j_m2": -2.0},
    }
    assert MODULE.classify(evolution) == [
        "STATE_EVOLUTION_RECONCILES_SIGN_CONTRADICTION"
    ]


def test_support_sign_change_is_material_even_below_ratio_threshold() -> None:
    summary = {
        "medians_j_m2": {
            "S_j_m2": -2.0,
            "F_j_m2": 0.0,
            "Q_j_m2": 2.0,
            "Q_all_j_m2": -2.0,
            "legacy_Q_all_j_m2": MODULE.PREDECESSOR_MJ_M2 * 1.0e6,
        },
        "initial_projection_difference_observed": False,
        "support_omission_ratio": 0.01,
        "support_delta_sign_changed": True,
        "predecessor_bridge_pass": True,
        "sample_count": 35,
        "eligible_sample_count": 35,
    }
    assert "SUPPORT_CENSORING_MATERIALLY_CONTRIBUTES" in MODULE.classify(summary)
