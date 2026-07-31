#!/usr/bin/env python3
"""Characterize the retained EB-04A failure dynamics without rerunning EB-04."""

from __future__ import annotations

import csv
import gzip
import hashlib
import io
import json
import math
import re
import subprocess
from collections import Counter, deque
from pathlib import Path
from typing import Any

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

matplotlib.rcParams["svg.hashsalt"] = "snow-surface-eb-04b"

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
EB04 = REPO / "docs/work-packages/20260730-snow-surface-eb-04-factorial-execution-adjudication-001"
EB04A = REPO / "docs/work-packages/20260730-snow-surface-eb-04a-failure-observability-state-capture-001"
EB04_RESULTS = EB04 / "artifacts/factorial-results.json"
EB04A_RESULTS = EB04A / "artifacts/diagnostic-replay.json"
FROZEN_INPUT_MANIFEST = ARTIFACTS / "frozen-input-manifest.csv"
BINARY = REPO / "target/debug/openwepp-cli-hill"
RHO_WATER_KG_M3 = 1_000.0
ICE_HEAT_CAPACITY_J_KG_K = 2_100.0
ABSOLUTE_ZERO_DELTA_K = 273.15
MASS_FILTER_M = 1.0e-9
EXPECTED_EB04A_RESULTS_SHA256 = "8208c12e608a47e57c0f9d1c47d10e95ffd01c6b649e119cba3448abeb7f3657"
NUMBER = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"

CHRONOLOGY_FIELDS = [
    "lane_id", "cell", "classification", "failure_day_index", "day_index",
    "days_to_failure", "terminal", "terminal_kind", "swe_before_m",
    "swe_after_m", "depth_after_m", "density_after_kg_m3", "layer_count_after",
    "minimum_layer_temperature_c", "maximum_layer_temperature_c",
    "cold_content_before_j_m2", "cold_content_after_j_m2",
    "cold_content_export_j_m2", "reconstructed_temperature_c", "sublimation_m",
    "snowpack_swe_loss_m", "shortwave_j_m2", "longwave_j_m2", "latent_j_m2",
    "surface_j_m2", "conduction_j_m2", "refrozen_liquid_m", "routed_melt_m",
    "maximum_active_mass_kg_m2", "maximum_lower_mass_kg_m2",
    "maximum_active_depth_m", "maximum_lower_depth_m", "maximum_abs_g0_w_m2",
    "peak_requested_g0_w_m2", "peak_applied_g0_w_m2", "peak_rejected_g0_w_m2",
    "peak_active_temperature_c", "peak_lower_temperature_c",
    "peak_active_conductivity_w_m_k", "peak_lower_conductivity_w_m_k",
    "peak_active_resistance_m2_k_w", "peak_lower_resistance_m2_k_w",
    "minimum_substep_seconds", "snapshot_mass_swe_m", "snapshot_depth_m",
    "snapshot_density_kg_m3", "snapshot_temperature_c",
    "snapshot_cold_content_j_m2", "geometry_reported_depth_m",
    "geometry_expected_depth_m", "geometry_signed_residual_m",
]


def main() -> int:
    self_check()
    FIGURES.mkdir(parents=True, exist_ok=True)
    eb04 = load_json(EB04_RESULTS)
    eb04a = load_json(EB04A_RESULTS)
    validate_inputs(eb04a)

    thermal: list[dict[str, Any]] = []
    geometry: list[dict[str, Any]] = []
    dynamics: list[dict[str, Any]] = []
    windows: list[dict[str, Any]] = []
    chronology_path = ARTIFACTS / "complete-chronology.csv.gz"
    chronology_row_count = 0
    with chronology_path.open("wb") as raw:
        with gzip.GzipFile(filename="", mode="wb", fileobj=raw, mtime=0) as compressed:
            with io.TextIOWrapper(compressed, encoding="utf-8", newline="") as text_stream:
                chronology_writer = csv.DictWriter(
                    text_stream,
                    fieldnames=CHRONOLOGY_FIELDS,
                    lineterminator="\n",
                )
                chronology_writer.writeheader()
                for result in eb04a["results"]:
                    trace_path = REPO / result["trace"]
                    if sha256(trace_path) != result["trace_sha256"]:
                        raise RuntimeError(f"trace identity changed: {trace_path}")
                    rows, tail, case_dynamics = scan_trace(
                        trace_path,
                        result["failure_day_index"],
                        result,
                        chronology_writer,
                    )
                    chronology_row_count += rows
                    dynamics.append(case_dynamics)
                    if rows != result["trace_row_count"]:
                        raise RuntimeError(f"trace row count changed: {trace_path}")
                    if result["classification"] == "PRIOR_LAYER_THICKNESS_AGGREGATE_MISMATCH":
                        record = parse_geometry(result)
                        geometry.append(record)
                        chronology_writer.writerow(geometry_terminal_row(record))
                    else:
                        record = parse_thermal(result, tail[-1])
                        thermal.append(record)
                        chronology_writer.writerow(thermal_terminal_row(record))
                        for row in tail:
                            windows.append(boundary_row(result, row))
                        windows.append(terminal_boundary_row(record))
                    chronology_row_count += 1

    thermal.sort(key=lambda row: (row["failure_day_index"], row["lane_id"], row["cell"]))
    geometry.sort(key=lambda row: (row["lane_id"], row["cell"]))
    dynamics.sort(key=lambda row: (row["lane_id"], row["cell"]))
    validate_population(thermal, geometry)
    companions = companion_cases(thermal, eb04)
    source_audit = audit_source_ordering()
    signatures = classify_signatures(thermal, geometry, dynamics, source_audit)
    summary = summarize(
        thermal,
        geometry,
        companions,
        signatures,
        source_audit,
        eb04a,
        chronology_path,
        chronology_row_count,
        dynamics,
    )

    write_json(ARTIFACTS / "coupled-dynamics-analysis.json", summary)
    write_thermal_csv(thermal)
    write_window_csv(windows)
    write_geometry_csv(geometry)
    write_csv(ARTIFACTS / "case-dynamics-summary.csv", dynamics)
    make_phase_space(thermal)
    make_representative_chronology(thermal)
    make_active_lower_coupling(thermal)
    make_component_context(thermal)
    make_geometry_figure(geometry)
    write_sidecars(summary, thermal, geometry)
    print(json.dumps(summary["acceptance"], indent=2, sort_keys=True))
    return 0 if summary["acceptance"]["passes"] else 1


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def command_sha256(argv: list[str]) -> str:
    completed = subprocess.run(
        argv, cwd=REPO, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )
    return hashlib.sha256(completed.stdout).hexdigest()


def validate_inputs(eb04a: dict[str, Any]) -> None:
    if sha256(EB04A_RESULTS) != EXPECTED_EB04A_RESULTS_SHA256:
        raise RuntimeError("frozen EB-04A diagnostic report identity changed")
    if eb04a["target_count"] != 24 or not eb04a["acceptance_passes"]:
        raise RuntimeError("EB-04A is not an accepted 24-target input")
    expected_targets: list[dict[str, str]] = []
    with FROZEN_INPUT_MANIFEST.open(encoding="utf-8", newline="") as stream:
        expected_targets = list(csv.DictReader(stream))
    actual_targets = []
    for result in sorted(eb04a["results"], key=lambda row: (row["lane_id"], row["cell"])):
        actual_targets.append(
            {
                "lane_id": result["lane_id"],
                "cell": result["cell"],
                "classification": result["classification"],
                "failure_day_index": str(result["failure_day_index"]),
                "trace_sha256": result["trace_sha256"],
                "typed_snapshot_sha256": hashlib.sha256(
                    result["typed_snapshot"].encode("utf-8")
                ).hexdigest(),
            }
        )
    if actual_targets != expected_targets:
        raise RuntimeError("frozen 24-target manifest changed")
    if sha256(EB04_RESULTS) != eb04a["source_eb04_results_sha256"]:
        raise RuntimeError("frozen EB-04 result identity changed")
    if not BINARY.is_file() or sha256(BINARY) != eb04a["binary_sha256"]:
        raise RuntimeError("EB-04A diagnostic executable identity changed")
    executable_diff = command_sha256(
        ["git", "diff", "--binary", "--", "crates", "tests"]
    )
    if executable_diff != eb04a["executable_source_diff_sha256"]:
        raise RuntimeError("EB-04A executable-source diff identity changed")


def scan_trace(
    path: Path,
    failure_day: int,
    result: dict[str, Any],
    chronology_writer: csv.DictWriter,
) -> tuple[int, deque[dict[str, Any]], dict[str, Any]]:
    tail: deque[dict[str, Any]] = deque(maxlen=30)
    count = 0
    dynamics = new_case_dynamics(result)
    with path.open(encoding="utf-8") as stream:
        for line in stream:
            row = json.loads(line)
            count += 1
            if int(row["day_index"]) >= failure_day:
                raise RuntimeError(f"trace crosses rejected day: {path}")
            chronology_writer.writerow(chronology_row(result, row))
            update_case_dynamics(dynamics, row)
            if float(row["runtime_swe_after_m"]) > 0.0 or tail:
                tail.append(row)
    if not tail:
        raise RuntimeError(f"no retained chronology: {path}")
    dynamics["trace_row_count"] = count
    return count, tail, dynamics


def new_case_dynamics(result: dict[str, Any]) -> dict[str, Any]:
    return {
        "lane_id": result["lane_id"],
        "cell": result["cell"],
        "classification": result["classification"],
        "failure_day_index": result["failure_day_index"],
        "trace_row_count": 0,
        "stage3_enabled_day_count": 0,
        "lower_present_day_count": 0,
        "maximum_active_mass_kg_m2": 0.0,
        "maximum_lower_mass_kg_m2": 0.0,
        "maximum_active_depth_m": 0.0,
        "maximum_lower_depth_m": 0.0,
        "maximum_abs_requested_g0_w_m2": 0.0,
        "maximum_abs_applied_g0_w_m2": 0.0,
        "maximum_abs_rejected_g0_w_m2": 0.0,
        "minimum_substep_seconds": None,
        "cumulative_sublimation_m": 0.0,
        "cumulative_snowpack_swe_loss_m": 0.0,
        "cumulative_shortwave_j_m2": 0.0,
        "cumulative_longwave_j_m2": 0.0,
        "cumulative_latent_j_m2": 0.0,
        "cumulative_refrozen_liquid_m": 0.0,
        "cumulative_routed_melt_m": 0.0,
        "maximum_layer_count": 0,
        "minimum_density_kg_m3": None,
        "maximum_density_kg_m3": 0.0,
    }


def update_case_dynamics(summary: dict[str, Any], row: dict[str, Any]) -> None:
    if bool(row["stage3_energy_enabled"]):
        summary["stage3_enabled_day_count"] += 1
    lower_mass = float(row["stage3_maximum_lower_mass_kg_m2"])
    if lower_mass > 0.0:
        summary["lower_present_day_count"] += 1
    maximum_fields = {
        "maximum_active_mass_kg_m2": "stage3_maximum_active_mass_kg_m2",
        "maximum_lower_mass_kg_m2": "stage3_maximum_lower_mass_kg_m2",
        "maximum_active_depth_m": "stage3_maximum_active_depth_m",
        "maximum_lower_depth_m": "stage3_maximum_lower_depth_m",
    }
    for target, source in maximum_fields.items():
        summary[target] = max(summary[target], float(row[source]))
    summary["maximum_abs_requested_g0_w_m2"] = max(
        summary["maximum_abs_requested_g0_w_m2"],
        abs(float(row["stage3_peak_g0_requested_w_m2"])),
    )
    summary["maximum_abs_applied_g0_w_m2"] = max(
        summary["maximum_abs_applied_g0_w_m2"],
        abs(float(row["stage3_peak_g0_w_m2"])),
    )
    summary["maximum_abs_rejected_g0_w_m2"] = max(
        summary["maximum_abs_rejected_g0_w_m2"],
        abs(float(row["stage3_peak_g0_rejected_w_m2"])),
    )
    step = float(row["stage3_minimum_substep_seconds"])
    if step > 0.0:
        current = summary["minimum_substep_seconds"]
        summary["minimum_substep_seconds"] = step if current is None else min(current, step)
    cumulative_fields = {
        "cumulative_sublimation_m": "sublimation_m",
        "cumulative_snowpack_swe_loss_m": "snowpack_swe_loss_m",
        "cumulative_shortwave_j_m2": "stage3_shortwave_energy_j_m2",
        "cumulative_longwave_j_m2": "stage3_longwave_energy_j_m2",
        "cumulative_latent_j_m2": "stage3_latent_energy_j_m2",
        "cumulative_refrozen_liquid_m": "stage3_refrozen_liquid_m",
        "cumulative_routed_melt_m": "routed_melt_m",
    }
    for target, source in cumulative_fields.items():
        summary[target] += float(row[source])
    summary["maximum_layer_count"] = max(summary["maximum_layer_count"], int(row["snow_layer_count_after"]))
    density = float(row["runtime_density_after_kg_m3"])
    if density > 0.0:
        current_density = summary["minimum_density_kg_m3"]
        summary["minimum_density_kg_m3"] = density if current_density is None else min(current_density, density)
        summary["maximum_density_kg_m3"] = max(summary["maximum_density_kg_m3"], density)


def extract_number(text: str, field: str) -> float:
    match = re.search(rf"{re.escape(field)}=({NUMBER})", text)
    if match is None:
        raise RuntimeError(f"missing {field} in typed snapshot")
    return float(match.group(1))


def reconstruct_temperature(mass_swe_m: float, cold_content_j_m2: float) -> float:
    return -cold_content_j_m2 / (
        mass_swe_m * RHO_WATER_KG_M3 * ICE_HEAT_CAPACITY_J_KG_K
    )


def parse_thermal(result: dict[str, Any], last: dict[str, Any]) -> dict[str, Any]:
    snapshot = result["typed_snapshot"]
    mass = extract_number(snapshot, "layer_mass_swe_m")
    cold = extract_number(snapshot, "layer_cold_content_j_m2")
    temperature = extract_number(snapshot, "control_volume_temperature_c")
    reconstructed = reconstruct_temperature(mass, cold)
    boundary_cold = mass * RHO_WATER_KG_M3 * ICE_HEAT_CAPACITY_J_KG_K * ABSOLUTE_ZERO_DELTA_K
    terminal_layers = snapshot.count("DirectSnowLayerState {")
    return {
        "lane_id": result["lane_id"],
        "cell": result["cell"],
        "classification": result["classification"],
        "failure_day_index": result["failure_day_index"],
        "terminal_mass_swe_m": mass,
        "terminal_depth_m": extract_number(snapshot, "layer_thickness_m"),
        "terminal_density_kg_m3": extract_number(snapshot, "layer_density_kg_m3"),
        "terminal_cold_content_j_m2": cold,
        "terminal_temperature_c": temperature,
        "reconstructed_temperature_c": reconstructed,
        "temperature_reconstruction_residual_c": temperature - reconstructed,
        "absolute_zero_boundary_cold_content_j_m2": boundary_cold,
        "absolute_zero_boundary_ratio": cold / boundary_cold,
        "terminal_control_volume_layer_count": terminal_layers,
        "last_success_day_index": int(last["day_index"]),
        "last_success_swe_m": float(last["runtime_swe_after_m"]),
        "rejected_slice_to_last_success_pack_mass_ratio": mass
        / float(last["runtime_swe_after_m"]),
        "last_success_cold_content_j_m2": float(last["stage3_cold_content_after_j_m2"]),
        "last_success_temperature_c": float(last.get("snow_layer_minimum_temperature_after_c") or 0.0),
        "last_success_sublimation_m": float(last["sublimation_m"]),
        "last_success_snowpack_swe_loss_m": float(last["snowpack_swe_loss_m"]),
        "last_success_shortwave_j_m2": float(last["stage3_shortwave_energy_j_m2"]),
        "last_success_longwave_j_m2": float(last["stage3_longwave_energy_j_m2"]),
        "last_success_latent_j_m2": float(last["stage3_latent_energy_j_m2"]),
        "last_success_surface_j_m2": float(last["stage3_surface_energy_j_m2"]),
        "last_success_conduction_j_m2": float(last["stage3_conduction_energy_j_m2"]),
        "last_success_refrozen_liquid_m": float(last["stage3_refrozen_liquid_m"]),
        "last_success_routed_melt_m": float(last["routed_melt_m"]),
        "last_success_maximum_active_mass_kg_m2": float(last["stage3_maximum_active_mass_kg_m2"]),
        "last_success_maximum_lower_mass_kg_m2": float(last["stage3_maximum_lower_mass_kg_m2"]),
    }


def parse_geometry(result: dict[str, Any]) -> dict[str, Any]:
    text = result["typed_snapshot"]
    value = extract_number(text, "prior_layers.thickness_m")
    expected_match = re.search(rf"does not match expected ({NUMBER})", text)
    if expected_match is None:
        raise RuntimeError("missing geometry expected value")
    expected = float(expected_match.group(1))
    layer_pattern = re.compile(
        rf"mass_swe_m: ({NUMBER}), thickness_m: ({NUMBER}), density_kg_m3: ({NUMBER})"
    )
    layers = [tuple(map(float, match)) for match in layer_pattern.findall(text)]
    if not layers:
        raise RuntimeError("missing prior layers")
    filtered_depth = sum(depth for mass, depth, _ in layers if mass > MASS_FILTER_M)
    excluded = [(mass, depth, density) for mass, depth, density in layers if mass <= MASS_FILTER_M]
    return {
        "lane_id": result["lane_id"],
        "cell": result["cell"],
        "classification": result["classification"],
        "failure_day_index": result["failure_day_index"],
        "reported_value_m": value,
        "expected_depth_m": expected,
        "signed_residual_m": value - expected,
        "parsed_filtered_depth_m": filtered_depth,
        "parsed_full_depth_m": sum(depth for _, depth, _ in layers),
        "excluded_layer_count": len(excluded),
        "excluded_mass_swe_m": sum(mass for mass, _, _ in excluded),
        "excluded_depth_m": sum(depth for _, depth, _ in excluded),
        "excluded_density_kg_m3": excluded[0][2] if len(excluded) == 1 else None,
        "layer_count": len(layers),
    }


def empty_chronology_row() -> dict[str, Any]:
    return {field: None for field in CHRONOLOGY_FIELDS}


def chronology_row(result: dict[str, Any], row: dict[str, Any]) -> dict[str, Any]:
    mass = float(row["runtime_swe_after_m"])
    cold = float(row["stage3_cold_content_after_j_m2"])
    record = empty_chronology_row()
    record.update(
        {
            "lane_id": result["lane_id"],
            "cell": result["cell"],
            "classification": result["classification"],
            "failure_day_index": result["failure_day_index"],
            "day_index": int(row["day_index"]),
            "days_to_failure": result["failure_day_index"] - int(row["day_index"]),
            "terminal": False,
            "terminal_kind": "successful_daily_trace",
            "swe_before_m": float(row["runtime_swe_before_m"]),
            "swe_after_m": mass,
            "depth_after_m": float(row["runtime_depth_after_m"]),
            "density_after_kg_m3": float(row["runtime_density_after_kg_m3"]),
            "layer_count_after": int(row["snow_layer_count_after"]),
            "minimum_layer_temperature_c": row["snow_layer_minimum_temperature_after_c"],
            "maximum_layer_temperature_c": row["snow_layer_maximum_temperature_after_c"],
            "cold_content_before_j_m2": float(row["stage3_cold_content_before_j_m2"]),
            "cold_content_after_j_m2": cold,
            "cold_content_export_j_m2": float(row["stage3_cold_content_export_j_m2"]),
            "reconstructed_temperature_c": reconstruct_temperature(mass, cold) if mass > 0.0 else 0.0,
            "sublimation_m": float(row["sublimation_m"]),
            "snowpack_swe_loss_m": float(row["snowpack_swe_loss_m"]),
            "shortwave_j_m2": float(row["stage3_shortwave_energy_j_m2"]),
            "longwave_j_m2": float(row["stage3_longwave_energy_j_m2"]),
            "latent_j_m2": float(row["stage3_latent_energy_j_m2"]),
            "surface_j_m2": float(row["stage3_surface_energy_j_m2"]),
            "conduction_j_m2": float(row["stage3_conduction_energy_j_m2"]),
            "refrozen_liquid_m": float(row["stage3_refrozen_liquid_m"]),
            "routed_melt_m": float(row["routed_melt_m"]),
            "maximum_active_mass_kg_m2": float(row["stage3_maximum_active_mass_kg_m2"]),
            "maximum_lower_mass_kg_m2": float(row["stage3_maximum_lower_mass_kg_m2"]),
            "maximum_active_depth_m": float(row["stage3_maximum_active_depth_m"]),
            "maximum_lower_depth_m": float(row["stage3_maximum_lower_depth_m"]),
            "maximum_abs_g0_w_m2": float(row["stage3_maximum_abs_g0_w_m2"]),
            "peak_requested_g0_w_m2": float(row["stage3_peak_g0_requested_w_m2"]),
            "peak_applied_g0_w_m2": float(row["stage3_peak_g0_w_m2"]),
            "peak_rejected_g0_w_m2": float(row["stage3_peak_g0_rejected_w_m2"]),
            "peak_active_temperature_c": float(row["stage3_peak_g0_active_temperature_c"]),
            "peak_lower_temperature_c": float(row["stage3_peak_g0_lower_temperature_c"]),
            "peak_active_conductivity_w_m_k": float(row["stage3_peak_g0_active_conductivity_w_m_k"]),
            "peak_lower_conductivity_w_m_k": float(row["stage3_peak_g0_lower_conductivity_w_m_k"]),
            "peak_active_resistance_m2_k_w": float(row["stage3_peak_g0_active_resistance_m2_k_w"]),
            "peak_lower_resistance_m2_k_w": float(row["stage3_peak_g0_lower_resistance_m2_k_w"]),
            "minimum_substep_seconds": float(row["stage3_minimum_substep_seconds"]),
        }
    )
    return record


def thermal_terminal_row(record: dict[str, Any]) -> dict[str, Any]:
    row = empty_chronology_row()
    row.update(
        {
            "lane_id": record["lane_id"],
            "cell": record["cell"],
            "classification": record["classification"],
            "failure_day_index": record["failure_day_index"],
            "day_index": record["failure_day_index"],
            "days_to_failure": 0,
            "terminal": True,
            "terminal_kind": "typed_rejected_control_volume",
            "reconstructed_temperature_c": record["reconstructed_temperature_c"],
            "snapshot_mass_swe_m": record["terminal_mass_swe_m"],
            "snapshot_depth_m": record["terminal_depth_m"],
            "snapshot_density_kg_m3": record["terminal_density_kg_m3"],
            "snapshot_temperature_c": record["terminal_temperature_c"],
            "snapshot_cold_content_j_m2": record["terminal_cold_content_j_m2"],
        }
    )
    return row


def geometry_terminal_row(record: dict[str, Any]) -> dict[str, Any]:
    row = empty_chronology_row()
    row.update(
        {
            "lane_id": record["lane_id"],
            "cell": record["cell"],
            "classification": record["classification"],
            "failure_day_index": record["failure_day_index"],
            "day_index": record["failure_day_index"],
            "days_to_failure": 0,
            "terminal": True,
            "terminal_kind": "typed_rejected_prior_layer_geometry",
            "geometry_reported_depth_m": record["reported_value_m"],
            "geometry_expected_depth_m": record["expected_depth_m"],
            "geometry_signed_residual_m": record["signed_residual_m"],
        }
    )
    return row


def boundary_row(result: dict[str, Any], row: dict[str, Any]) -> dict[str, Any]:
    mass = float(row["runtime_swe_after_m"])
    cold = float(row["stage3_cold_content_after_j_m2"])
    return {
        "lane_id": result["lane_id"],
        "cell": result["cell"],
        "classification": result["classification"],
        "failure_day_index": result["failure_day_index"],
        "day_index": int(row["day_index"]),
        "days_to_failure": result["failure_day_index"] - int(row["day_index"]),
        "terminal": False,
        "swe_m": mass,
        "cold_content_j_m2": cold,
        "reconstructed_temperature_c": reconstruct_temperature(mass, cold) if mass > 0.0 else 0.0,
        "sublimation_m": float(row["sublimation_m"]),
        "snowpack_swe_loss_m": float(row["snowpack_swe_loss_m"]),
        "shortwave_j_m2": float(row["stage3_shortwave_energy_j_m2"]),
        "longwave_j_m2": float(row["stage3_longwave_energy_j_m2"]),
        "latent_j_m2": float(row["stage3_latent_energy_j_m2"]),
        "surface_j_m2": float(row["stage3_surface_energy_j_m2"]),
        "lower_mass_kg_m2": float(row["stage3_maximum_lower_mass_kg_m2"]),
        "refrozen_liquid_m": float(row["stage3_refrozen_liquid_m"]),
        "routed_melt_m": float(row["routed_melt_m"]),
    }


def terminal_boundary_row(record: dict[str, Any]) -> dict[str, Any]:
    return {
        "lane_id": record["lane_id"],
        "cell": record["cell"],
        "classification": record["classification"],
        "failure_day_index": record["failure_day_index"],
        "day_index": record["failure_day_index"],
        "days_to_failure": 0,
        "terminal": True,
        "swe_m": record["terminal_mass_swe_m"],
        "cold_content_j_m2": record["terminal_cold_content_j_m2"],
        "reconstructed_temperature_c": record["reconstructed_temperature_c"],
        "sublimation_m": None,
        "snowpack_swe_loss_m": None,
        "shortwave_j_m2": None,
        "longwave_j_m2": None,
        "latent_j_m2": None,
        "surface_j_m2": None,
        "lower_mass_kg_m2": 0.0,
        "refrozen_liquid_m": None,
        "routed_melt_m": None,
    }


def validate_population(thermal: list[dict[str, Any]], geometry: list[dict[str, Any]]) -> None:
    if len(thermal) != 22 or len(geometry) != 2:
        raise RuntimeError("expected 22 thermal and two geometry failures")
    if Counter(row["classification"] for row in thermal) != {
        "CONDUCTIVITY_TEMPERATURE_BELOW_ABSOLUTE_ZERO": 17,
        "SATURATION_VAPOR_PRESSURE_UNDERFLOW": 5,
    }:
        raise RuntimeError("thermal classification population changed")


def companion_cases(thermal: list[dict[str, Any]], eb04: dict[str, Any]) -> dict[str, Any]:
    lanes = {lane["lane_id"]: lane for lane in eb04["lanes"]}
    count_at_day = 0
    nonfailed_b = 0
    for row in thermal:
        lane = lanes[row["lane_id"]]
        b = lane["cells"]["B"]
        if b["execution_status"] == "PASS":
            nonfailed_b += 1
        trace = REPO / b["trace"]
        if sha256(trace) != b["trace_sha256"]:
            raise RuntimeError(f"frozen B companion trace identity changed: {trace}")
        with trace.open(encoding="utf-8") as stream:
            for line in stream:
                candidate = json.loads(line)
                if int(candidate["day_index"]) == row["failure_day_index"] - 1:
                    count_at_day += 1
                    break
    return {
        "thermal_targets_with_completed_B_cell": nonfailed_b,
        "B_traces_reaching_pre_failure_day": count_at_day,
        "thermal_cell_counts": dict(sorted(Counter(row["cell"] for row in thermal).items())),
    }


def audit_source_ordering() -> dict[str, Any]:
    path = REPO / "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs"
    text = path.read_text(encoding="utf-8")
    start = text.index("fn resolve_stage3_liquid_routing")
    end = text.index("fn stage3_liquid_routing_enabled", start)
    body = text[start:end]
    state = body.index("let active_state = Self::stage3_control_volume_state")
    carrier = body.index("let carrier = Self::stage3_hourly_surface_energy")
    conduction = body.index("let conduction = Self::apply_stage3_active_lower_conduction")
    applied = body.index("let applied = Self::apply_stage3_control_volume_energy")
    removal = body.index("Self::remove_stage3_active_sublimation")
    energy_fn = text[text.index("fn apply_stage3_control_volume_energy"):text.index("fn apply_stage3_active_lower_conduction")]
    return {
        "source": str(path.relative_to(REPO)),
        "source_sha256": sha256(path),
        "state_guard_before_carrier_before_conduction_before_energy_before_mass_removal": state < carrier < conduction < applied < removal,
        "negative_energy_has_no_domain_bound_before_typed_temperature_guard": "if energy_j_m2 >= 0.0" in energy_fn and "*cold_content +=" in energy_fn,
        "current_failing_substep_carrier_not_evaluated_before_state_rejection": state < carrier,
    }


def classify_signatures(
    thermal: list[dict[str, Any]],
    geometry: list[dict[str, Any]],
    dynamics: list[dict[str, Any]],
    source: dict[str, Any],
) -> list[dict[str, Any]]:
    max_temp_residual = max(abs(row["temperature_reconstruction_residual_c"]) for row in thermal)
    sublimation_cells = sum(row["cell"] in {"S", "LS"} for row in thermal)
    prior_sublimation_contributors = sum(
        row["cell"] in {"S", "LS"}
        and row["last_success_sublimation_m"] > 0.0
        and row["last_success_latent_j_m2"] < 0.0
        for row in thermal
    )
    longwave_only_contrary = sum(row["cell"] == "L" for row in thermal)
    one_layer = sum(row["terminal_control_volume_layer_count"] == 1 for row in thermal)
    maximum_snapshot_depth = max(row["terminal_depth_m"] for row in thermal)
    prior_lower_present = sum(row["lower_present_day_count"] > 0 for row in dynamics if row["classification"] != "PRIOR_LAYER_THICKNESS_AGGREGATE_MISMATCH")
    geometry_matches = sum(
        math.isclose(row["parsed_filtered_depth_m"], row["reported_value_m"], abs_tol=2e-15)
        and math.isclose(row["excluded_depth_m"], -row["signed_residual_m"], abs_tol=2e-15)
        for row in geometry
    )
    return [
        {
            "signature": "VANISHING_MASS_RETAINED_COLD_CONTENT",
            "status": support_status(max_temp_residual <= 1.0e-9),
            "evidence": f"22/22 reconstruct; maximum temperature residual {max_temp_residual:.3e} degC",
        },
        {
            "signature": "SUBLIMATION_AMPLIFIED_THIN_PACK_COOLING",
            "status": "INCONCLUSIVE_FOR_TERMINAL_AMPLIFICATION",
            "evidence": (
                f"{sublimation_cells}/22 are S/LS and {prior_sublimation_contributors}/20 "
                f"have prior-day mass export plus negative latent energy; "
                f"{longwave_only_contrary}/22 L-only failures prove sublimation is not necessary; "
                "the unpublished boundary-crossing substep prevents an amplification claim"
            ),
        },
        {
            "signature": "LOWER_LAYER_DECOUPLED_AT_FAILURE",
            "status": "INCONCLUSIVE_SNAPSHOT_ROLE_UNLABELED",
            "evidence": (
                f"{one_layer}/22 rejected slices contain one layer and maximum slice depth is "
                f"{maximum_snapshot_depth:.3e} m, but the error does not label active versus lower; "
                f"{prior_lower_present}/22 complete chronologies contain a lower volume"
            ),
        },
        {
            "signature": "NOT_CONDUCTIVITY_CONSTITUTIVE",
            "status": reject_status(
                sum(row["classification"] == "SATURATION_VAPOR_PRESSURE_UNDERFLOW" for row in thermal) == 5
            ),
            "evidence": (
                "17 impossible-temperature states reject upstream of conductivity; five valid-Kelvin "
                "states underflow inside the SNOBAL vapor-pressure conductivity dependency"
            ),
        },
        {
            "signature": "GEOMETRY_ROUNDOFF_SCALE",
            "status": support_status(geometry_matches == 2),
            "evidence": f"{geometry_matches}/2 residuals equal the depth of one layer excluded by the SWE filter",
        },
        {
            "signature": "CURRENT_SUBSTEP_FLUX_IDENTIFIABLE_FROM_RETAINED_FAILURE",
            "status": reject_status(
                source["current_failing_substep_carrier_not_evaluated_before_state_rejection"]
            ),
            "evidence": "control-volume conductivity rejects before the current carrier is evaluated or a successful daily row is published",
        },
    ]


def summarize(
    thermal: list[dict[str, Any]],
    geometry: list[dict[str, Any]],
    companions: dict[str, Any],
    signatures: list[dict[str, Any]],
    source: dict[str, Any],
    eb04a: dict[str, Any],
    chronology_path: Path,
    chronology_row_count: int,
    dynamics: list[dict[str, Any]],
) -> dict[str, Any]:
    ratios = [row["rejected_slice_to_last_success_pack_mass_ratio"] for row in thermal]
    boundary_ratios = [row["absolute_zero_boundary_ratio"] for row in thermal]
    temp_residuals = [abs(row["temperature_reconstruction_residual_c"]) for row in thermal]
    geometry_residuals = [abs(row["signed_residual_m"]) for row in geometry]
    allowed_statuses = {
        "SUPPORTED",
        "REJECTED",
        "INCONCLUSIVE_FOR_TERMINAL_AMPLIFICATION",
        "INCONCLUSIVE_SNAPSHOT_ROLE_UNLABELED",
    }
    all_signatures_dispositioned = all(
        row["status"] in allowed_statuses and bool(row["evidence"]) for row in signatures
    )
    geometry_matches = all(
        math.isclose(row["parsed_filtered_depth_m"], row["reported_value_m"], abs_tol=2e-15)
        and math.isclose(row["excluded_depth_m"], -row["signed_residual_m"], abs_tol=2e-15)
        for row in geometry
    )
    source_ordering_proved = all(
        value for key, value in source.items() if key not in {"source", "source_sha256"}
    )
    expected_chronology_rows = sum(result["trace_row_count"] for result in eb04a["results"]) + 24
    predicates = {
        "all_24_classified": len(thermal) + len(geometry) == 24,
        "all_thermal_temperatures_reconstructed": max(temp_residuals) <= 1.0e-9,
        "both_geometry_residuals_reconstructed": len(geometry) == 2 and geometry_matches,
        "complete_chronology_retained": chronology_row_count == expected_chronology_rows,
        "all_signatures_dispositioned": all_signatures_dispositioned,
        "source_ordering_proved": source_ordering_proved,
        "all_B_companions_bound_and_reach_pre_failure_day": companions["thermal_targets_with_completed_B_cell"] == 22
        and companions["B_traces_reaching_pre_failure_day"] == 22,
    }
    return {
        "schema": "snow-surface-eb04b-coupled-dynamics-v1",
        "evidence_class": "Ran",
        "git_head": eb04a["git_head"],
        "input_identity": {
            "eb04_results_sha256": sha256(EB04_RESULTS),
            "eb04a_results_sha256": sha256(EB04A_RESULTS),
            "frozen_input_manifest_sha256": sha256(FROZEN_INPUT_MANIFEST),
            "binary_sha256": sha256(BINARY),
            "executable_source_diff_sha256": command_sha256(["git", "diff", "--binary", "--", "crates", "tests"]),
        },
        "population": {
            "target_count": 24,
            "thermal_count": len(thermal),
            "geometry_count": len(geometry),
            "classification_counts": dict(sorted(Counter(row["classification"] for row in thermal + geometry).items())),
            "cell_counts": dict(sorted(Counter(row["cell"] for row in thermal + geometry).items())),
        },
        "thermal_summary": {
            "minimum_terminal_mass_swe_m": min(row["terminal_mass_swe_m"] for row in thermal),
            "maximum_terminal_mass_swe_m": max(row["terminal_mass_swe_m"] for row in thermal),
            "minimum_terminal_cold_content_j_m2": min(row["terminal_cold_content_j_m2"] for row in thermal),
            "maximum_terminal_cold_content_j_m2": max(row["terminal_cold_content_j_m2"] for row in thermal),
            "minimum_rejected_slice_to_last_success_pack_mass_ratio": min(ratios),
            "median_rejected_slice_to_last_success_pack_mass_ratio": sorted(ratios)[len(ratios) // 2],
            "maximum_rejected_slice_to_last_success_pack_mass_ratio": max(ratios),
            "minimum_absolute_zero_boundary_ratio": min(boundary_ratios),
            "maximum_absolute_zero_boundary_ratio": max(boundary_ratios),
            "maximum_temperature_reconstruction_residual_c": max(temp_residuals),
            "terminal_one_layer_count": sum(row["terminal_control_volume_layer_count"] == 1 for row in thermal),
        },
        "geometry_summary": {
            "minimum_abs_depth_residual_m": min(geometry_residuals),
            "maximum_abs_depth_residual_m": max(geometry_residuals),
            "excluded_fragment_count": sum(row["excluded_layer_count"] for row in geometry),
        },
        "complete_chronology": {
            "path": str(chronology_path.relative_to(REPO)),
            "sha256": sha256(chronology_path),
            "row_count": chronology_row_count,
            "expected_row_count": expected_chronology_rows,
            "fields": CHRONOLOGY_FIELDS,
        },
        "case_dynamics": {
            "case_count": len(dynamics),
            "lower_present_case_count": sum(row["lower_present_day_count"] > 0 for row in dynamics),
            "maximum_abs_requested_g0_w_m2": max(row["maximum_abs_requested_g0_w_m2"] for row in dynamics),
            "maximum_abs_applied_g0_w_m2": max(row["maximum_abs_applied_g0_w_m2"] for row in dynamics),
            "maximum_abs_rejected_g0_w_m2": max(row["maximum_abs_rejected_g0_w_m2"] for row in dynamics),
            "minimum_substep_seconds": min(row["minimum_substep_seconds"] for row in dynamics if row["minimum_substep_seconds"] is not None),
        },
        "companions": companions,
        "source_ordering": source,
        "prospective_signatures": signatures,
        "root_cause_assignment": {
            "impossible_temperature_17": "positive cold content on vanishing mass; boundary-crossing driver unresolved; EB-04C",
            "vapor_pressure_underflow_5": "valid-Kelvin extreme-cold input reaches SNOBAL numerical/constitutive domain limit; EB-04C",
            "sublimation_role": "strong prior association and mechanistic contributor in 20/22, not necessary cause; terminal amplification unproven",
            "geometry_2": "dimensionally inconsistent fragment filter/closure boundary; EB-04D",
            "conductivity": "downstream sentinel for 17; numerical/constitutive underflow site for five",
        },
        "acceptance": {**predicates, "passes": all_required(predicates)},
    }


def support_status(predicate: bool) -> str:
    return "SUPPORTED" if predicate else "REJECTED"


def reject_status(rejection_predicate: bool) -> str:
    return "REJECTED" if rejection_predicate else "SUPPORTED"


def all_required(predicates: dict[str, bool]) -> bool:
    return all(predicates.values())


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_thermal_csv(rows: list[dict[str, Any]]) -> None:
    write_csv(ARTIFACTS / "thermal-failure-summary.csv", rows)


def write_window_csv(rows: list[dict[str, Any]]) -> None:
    write_csv(ARTIFACTS / "boundary-window.csv", rows)


def write_geometry_csv(rows: list[dict[str, Any]]) -> None:
    write_csv(ARTIFACTS / "geometry-failure-summary.csv", rows)


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        raise RuntimeError(f"refusing empty CSV: {path}")
    with path.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(
            stream,
            fieldnames=list(rows[0]),
            lineterminator="\n",
        )
        writer.writeheader()
        writer.writerows(rows)


def style_axes(ax: Any) -> None:
    ax.grid(True, color="#d9dde3", linewidth=0.7, zorder=0)
    ax.set_axisbelow(True)
    for spine in ax.spines.values():
        spine.set_color("#65717e")


def save_figure(fig: Any, name: str) -> None:
    path = FIGURES / f"{name}.svg"
    fig.savefig(path, format="svg", bbox_inches="tight", metadata={"Date": None})
    plt.close(fig)
    text = path.read_text(encoding="utf-8")
    cleaned = "\n".join(line.rstrip() for line in text.splitlines()).rstrip() + "\n"
    path.write_text(cleaned, encoding="utf-8")


def make_phase_space(rows: list[dict[str, Any]]) -> None:
    fig, ax = plt.subplots(figsize=(9.2, 5.8))
    colors = {"L": "#4c78a8", "S": "#e45756", "LS": "#7a5195"}
    markers = {"CONDUCTIVITY_TEMPERATURE_BELOW_ABSOLUTE_ZERO": "o", "SATURATION_VAPOR_PRESSURE_UNDERFLOW": "^"}
    masses = [row["terminal_mass_swe_m"] * RHO_WATER_KG_M3 for row in rows]
    xline = [min(masses) / 1.5, max(masses) * 1.5]
    yline = [x * ICE_HEAT_CAPACITY_J_KG_K * ABSOLUTE_ZERO_DELTA_K for x in xline]
    ax.plot(xline, yline, color="#20262e", linewidth=2.0, label="0 K boundary")
    for row in rows:
        ax.scatter(
            row["terminal_mass_swe_m"] * RHO_WATER_KG_M3,
            row["terminal_cold_content_j_m2"],
            s=64,
            color=colors[row["cell"]],
            marker=markers[row["classification"]],
            edgecolor="white",
            linewidth=0.8,
            zorder=3,
        )
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("Rejected control-volume ice mass (kg m$^{-2}$)")
    ax.set_ylabel("Retained cold content (J m$^{-2}$)")
    ax.set_title("Thermal rejections approach or cross the thin-pack absolute-zero boundary")
    ax.text(0.02, 0.03, "Circles: below 0 K   Triangles: vapor-pressure underflow\nBlue: L   Red: S   Purple: LS", transform=ax.transAxes, fontsize=9, va="bottom")
    ax.legend(loc="upper left", frameon=True)
    style_axes(ax)
    save_figure(fig, "eb04b-terminal-phase-space")


def select_representatives(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    ordered = sorted(rows, key=lambda row: row["failure_day_index"])
    return [ordered[0], ordered[len(ordered) // 2], ordered[-1]]


def read_tail_for(record: dict[str, Any], count: int = 20) -> list[dict[str, Any]]:
    report = load_json(EB04A_RESULTS)
    result = next(row for row in report["results"] if row["lane_id"] == record["lane_id"] and row["cell"] == record["cell"])
    tail: deque[dict[str, Any]] = deque(maxlen=count)
    with (REPO / result["trace"]).open(encoding="utf-8") as stream:
        for line in stream:
            tail.append(json.loads(line))
    return list(tail)


def make_representative_chronology(rows: list[dict[str, Any]]) -> None:
    reps = select_representatives(rows)
    fig, axes = plt.subplots(3, 2, figsize=(11.5, 10.0), sharex=False)
    for index, record in enumerate(reps):
        trace = [
            row
            for row in read_tail_for(record, 60)
            if float(row["stage3_maximum_lower_mass_kg_m2"]) == 0.0
            and float(row["runtime_swe_after_m"]) > 0.0
        ][-20:]
        x = [int(row["day_index"]) - record["failure_day_index"] for row in trace] + [0]
        mass = [float(row["runtime_swe_after_m"]) * 1000.0 for row in trace] + [record["terminal_mass_swe_m"] * 1000.0]
        temp = [
            reconstruct_temperature(
                float(row["runtime_swe_after_m"]),
                float(row["stage3_cold_content_after_j_m2"]),
            )
            for row in trace
        ]
        left, right = axes[index]
        left.plot(
            x[:-1],
            mass[:-1],
            color="#2a6f97",
            marker="o",
            markersize=3.5,
            label="successful complete-pack SWE",
        )
        left.scatter(
            [0],
            [mass[-1]],
            marker="X",
            s=70,
            color="#20262e",
            zorder=4,
            label="typed rejected-slice mass",
        )
        left.set_yscale("log")
        left.set_ylabel("SWE (mm)")
        left.set_title(f"{record['lane_id']} / {record['cell']} — day {record['failure_day_index']}")
        right.plot(x[:-1], temp, color="#b23a48", marker="o", markersize=3.5, label="successful complete-pack active state")
        right.scatter([0], [record["terminal_temperature_c"]], marker="X", s=70, color="#20262e", zorder=4, label="typed rejected slice")
        right.axhline(-273.15, color="#20262e", linestyle="--", linewidth=1.3)
        right.set_ylabel("Snow temperature (°C)")
        right.set_title("Complete-pack active history; rejected slice separate")
        for ax in (left, right):
            ax.set_xlabel("Days relative to rejection")
            style_axes(ax)
        if index == 0:
            left.legend(loc="lower left", fontsize=8)
            right.legend(loc="lower left", fontsize=8)
    fig.suptitle(
        "Prior complete-pack histories and their separate rejected micro-volume slices",
        y=1.01,
        fontsize=14,
    )
    fig.tight_layout()
    save_figure(fig, "eb04b-representative-chronology")


def make_component_context(rows: list[dict[str, Any]]) -> None:
    cells = ["L", "S", "LS"]
    components = [
        ("last_success_shortwave_j_m2", "Shortwave", "#f2b134"),
        ("last_success_longwave_j_m2", "Longwave", "#4c78a8"),
        ("last_success_latent_j_m2", "Latent", "#e45756"),
    ]
    fig, axes = plt.subplots(1, 2, figsize=(11.2, 5.2))
    width = 0.22
    for offset, (field, label, color) in enumerate(components):
        medians = []
        for cell in cells:
            values = sorted(row[field] / 1.0e6 for row in rows if row["cell"] == cell)
            medians.append(values[len(values) // 2])
        axes[0].bar([i + (offset - 1) * width for i in range(3)], medians, width, label=label, color=color)
    axes[0].axhline(0.0, color="#20262e", linewidth=1.0)
    axes[0].set_xticks(range(3), cells)
    axes[0].set_ylabel("Median last-success energy (MJ m$^{-2}$ d$^{-1}$)")
    axes[0].set_title("Energy context immediately before rejection")
    axes[0].legend(loc="best")
    ratios = [
        [row["rejected_slice_to_last_success_pack_mass_ratio"] for row in rows if row["cell"] == cell]
        for cell in cells
    ]
    axes[1].boxplot(ratios, tick_labels=cells, patch_artist=True, boxprops={"facecolor": "#9ecae1"}, medianprops={"color": "#20262e", "linewidth": 2})
    axes[1].set_yscale("log")
    axes[1].set_ylabel("Rejected-slice mass / prior-day pack mass")
    axes[1].set_title("Rejected slices are tiny relative to the prior-day pack")
    for ax in axes:
        style_axes(ax)
    fig.tight_layout()
    save_figure(fig, "eb04b-component-and-mass-context")


def make_active_lower_coupling(rows: list[dict[str, Any]]) -> None:
    reps = select_representatives(rows)
    fig, axes = plt.subplots(3, 2, figsize=(11.5, 10.0))
    for index, record in enumerate(reps):
        trace = read_tail_for(record, 30)
        x = [int(row["day_index"]) - record["failure_day_index"] for row in trace]
        active = [float(row["stage3_maximum_active_mass_kg_m2"]) for row in trace]
        lower = [float(row["stage3_maximum_lower_mass_kg_m2"]) for row in trace]
        requested = [float(row["stage3_peak_g0_requested_w_m2"]) for row in trace]
        applied = [float(row["stage3_peak_g0_w_m2"]) for row in trace]
        rejected = [float(row["stage3_peak_g0_rejected_w_m2"]) for row in trace]
        left, right = axes[index]
        left.plot(x, active, color="#2a6f97", label="active maximum")
        left.plot(x, lower, color="#59a14f", label="lower maximum")
        left.set_yscale("symlog", linthresh=0.01)
        left.set_ylabel("Daily maximum mass (kg m$^{-2}$)")
        left.set_title(f"{record['lane_id']} / {record['cell']} — layer context")
        right.plot(x, requested, color="#7a5195", label="requested $G_0$")
        right.plot(x, applied, color="#4c78a8", label="applied $G_0$")
        right.plot(x, rejected, color="#e45756", linestyle="--", label="rejected $G_0$")
        right.axhline(0.0, color="#20262e", linewidth=0.8)
        right.set_ylabel("Peak substep $G_0$ (W m$^{-2}$)")
        right.set_title("Active/lower conductive exchange")
        for ax in (left, right):
            ax.set_xlabel("Days relative to rejection")
            style_axes(ax)
        if index == 0:
            left.legend(loc="best", fontsize=8)
            right.legend(loc="best", fontsize=8)
    fig.suptitle("Retained active/lower coupling before representative failures", y=1.01, fontsize=14)
    fig.tight_layout()
    save_figure(fig, "eb04b-active-lower-coupling")


def make_geometry_figure(rows: list[dict[str, Any]]) -> None:
    fig, ax = plt.subplots(figsize=(8.8, 5.3))
    labels = [f"{row['lane_id']} / {row['cell']}" for row in rows]
    values = [row["excluded_depth_m"] for row in rows]
    positions = list(range(len(rows)))
    ax.scatter(
        positions,
        values,
        color=["#59a14f", "#4c78a8"],
        s=150,
        edgecolor="white",
        linewidth=1.0,
        zorder=3,
    )
    for position, value in zip(positions, values):
        ax.vlines(position, 1.0e-9, value, color="#65717e", linewidth=2.0, zorder=2)
        ax.annotate(
            f"{value / 1.0e-9:.3f} nm",
            (position, value),
            xytext=(0, 10),
            textcoords="offset points",
            ha="center",
            fontsize=10,
        )
    ax.axhline(1.0e-9, color="#20262e", linestyle="--", linewidth=1.8, label="depth closure tolerance")
    ax.set_xticks(positions, labels)
    ax.set_ylim(0.995e-9, 1.105e-9)
    ax.set_ylabel("Depth of mass-filtered fragment (m)")
    ax.set_title("Each geometry residual is one fragment excluded by the SWE filter")
    ax.legend(loc="lower right")
    style_axes(ax)
    fig.tight_layout()
    save_figure(fig, "eb04b-geometry-fragment")


def write_sidecars(summary: dict[str, Any], thermal: list[dict[str, Any]], geometry: list[dict[str, Any]]) -> None:
    sidecars = {
        "eb04b-terminal-phase-space": """# EB-04B Terminal Phase Space

## Caption

Retained cold content versus ice mass for all 22 thermal-family rejected
control volumes. The line is the independently calculated `0 K` boundary,
`Q_cc = m c_i 273.15`. Circles crossed the boundary; triangles were rejected
slightly earlier when saturation vapor pressure underflowed.

## How To Read This Figure

Distance above the line means the cold-content-to-heat-capacity ratio implies a
temperature below absolute zero. All points are rejected micro-volume slices,
not a claim about complete seasonal pack mass. Color identifies the enabled cell, while marker
shape identifies the exact typed rejection.

## Provenance And Limits

Generated from EB-04A typed snapshots by `tools/run_analysis.py`. Temperature
is independently reconstructed with the canonical `2100 J kg^-1 K^-1` ice
heat capacity. The plot proves the proximate state; it does not select a
corrective extinction or phase-change formulation.
""",
        "eb04b-representative-chronology": """# EB-04B Representative Chronology

## Caption

The earliest, median, and latest thermal failures over their final retained
complete-pack active days plus the exact rejected slice, shown separately. SWE
is logarithmic so both scales remain visible. The dashed line marks absolute
zero.

## How To Read This Figure

The left column contrasts the rejected slice with the last successful
complete-pack state. The right column keeps their cold-content-derived
temperatures visually separate because the terminal active/lower role is
unlabeled. Widely separated model days reach the same rejected-slice scale,
ruling out a startup-only signature without claiming whole-pack continuity.

## Provenance And Limits

Successful-day points are restricted to rows with no lower volume, so
whole-pack `-Q_cc/(m c_i)` is the active-control-volume temperature. The black
terminal marker is deliberately not connected: it is the rejected slice from
the typed snapshot, whose active/lower role is unlabeled. Control-volume
construction rejects before the current carrier is evaluated, so no unobserved
failure-substep flux is invented.
""",
        "eb04b-component-and-mass-context": """# EB-04B Component And Mass Context

## Caption

Median shortwave, longwave, and latent energy on the last successful day for
L, S, and LS thermal failures, paired with the ratio of rejected control-volume
mass to the prior successful-day SWE.

## How To Read This Figure

S and LS account for 20 of 22 thermal failures, and all 20 carry prior-day
sublimation mass export with negative latent energy. This is a strong
association and a mechanistic contribution, not proof of terminal
amplification because the boundary-crossing substep is unpublished. The two L
failures prove sublimation is not necessary. The logarithmic ratio panel
compares the unlabeled rejected slice with the complete prior-day pack; it does
not represent a continuous whole-pack mass trajectory.

## Provenance And Limits

Values are retained producer operands from EB-04A. They describe the last
complete day, not the unpublished current failure substep. No observation
score or coefficient comparison is made.
""",
        "eb04b-active-lower-coupling": """# EB-04B Active And Lower Coupling

## Caption

Daily active/lower maximum mass and peak requested, applied, and rejected
conductive exchange for the early, median, and late representative thermal
failures. Positive `G_0` supplies heat to the active volume; rejected exchange
is energy that could not be accepted without exceeding the existing CoE
zero-cold-content boundary.

## How To Read This Figure

The left column shows when a lower thermal reservoir exists and when the pack
becomes active-only on successful days. The right column shows that requested
coupling can be much larger than applied coupling because the receiving volume
has bounded cold content. This chronology constrains EB-04C but does not prove
the role of the unlabeled terminal slice.

## Provenance And Limits

Generated from the complete hash-bound EB-04A daily chronology. Values are
daily maxima or peak-substep diagnostics, not an hourly integral. The rejected
day has no successful trace row; no terminal `G_0` value is imputed.
""",
        "eb04b-geometry-fragment": """# EB-04B Geometry Fragment

## Caption

Depth of the single layer fragment excluded by the production `mass_swe_m >
1e-9 m` filter in each geometry failure. Both excluded depths lie just above
the separate `1e-9 m` depth-closure tolerance.

## How To Read This Figure

The aggregate residual is not unexplained drift: it equals the physical depth
of a fragment excluded because its water-equivalent mass is below a threshold
expressed in different units. Density below `1000 kg m^-3` allows the excluded
depth to exceed the depth tolerance.

## Provenance And Limits

Layer mass, depth, and density are parsed from the complete EB-04A typed prior-
layer snapshots. EB-04D must select and authorize the correction; this figure
does not justify simply loosening the tolerance.
""",
    }
    for name, text in sidecars.items():
        (FIGURES / f"{name}.md").write_text(text.strip() + "\n", encoding="utf-8")


def self_check() -> None:
    mass = 2.0e-6
    cold = 1_200.0
    expected = -cold / (mass * RHO_WATER_KG_M3 * ICE_HEAT_CAPACITY_J_KG_K)
    if not math.isclose(reconstruct_temperature(mass, cold), expected, abs_tol=1e-15):
        raise RuntimeError("temperature reconstruction self-check failed")
    wrong_sign = -expected
    wrong_depth_alias = -cold / (0.01 * RHO_WATER_KG_M3 * ICE_HEAT_CAPACITY_J_KG_K)
    if math.isclose(wrong_sign, expected) or math.isclose(wrong_depth_alias, expected):
        raise RuntimeError("anti-alias self-check failed")
    if support_status(False) != "REJECTED" or reject_status(False) != "SUPPORTED":
        raise RuntimeError("signature predicate self-check failed")
    required = {"temperature": True, "geometry": True, "chronology": True, "source": True}
    if not all_required(required):
        raise RuntimeError("acceptance positive self-check failed")
    for key in required:
        broken = dict(required)
        broken[key] = False
        if all_required(broken):
            raise RuntimeError(f"acceptance failed-predicate self-check failed: {key}")


if __name__ == "__main__":
    raise SystemExit(main())
