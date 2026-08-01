#!/usr/bin/env python3
"""Independently seal and verify retained EB-04E WAT/trace outputs."""

from __future__ import annotations

import ast
import hashlib
import importlib.util
import json
import math
import sys
from pathlib import Path
from typing import Any

import pyarrow.parquet as pq

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
OUTPUT = REPO / "target/snow_surface_eb04e_qualification/runs"
ATTEMPT = ARTIFACTS / "execution-attempt.json"
EXECUTION_LOG = ARTIFACTS / "cmd3_execute.log"
FROZEN_TOOL = PACKAGE / "tools/run_qualification.py"
SEAL = ARTIFACTS / "retained-output-seal.json"
RESULT = ARTIFACTS / "retained-output-verification.json"
EXPECTED_HEAD = "44c6c9cc2e4447064fbbbf70935cf581d60d49b0"
EXPECTED_BINARY = "0242c39fa26e9cbbd9461a36a4d6843b8adf0600fb72c215c349a454cbf66a50"
EXPECTED_TOOL = "1e6a054839997d685b25c665d79768e34482652235fe09325416c89be5e0558e"
EXPECTED_PROTOCOL = "216db53b5e389cc069202055e48b920e254cd88a52aaeb9a9c57de41b1a51440"
EXPECTED_MODELS = {
    "snow_density_model": "physics_bulk_multilayer_density_v1",
    "snow_melt_model": "coe_liquid_holding_capacity_v1",
    "snow_phase_model": "harder_pomeroy_hourly",
}
CELLS = {"B", "L", "S", "LS"}
LANES = {
    "snotel_mica_creek_st_joe_id",
    "snotel_paradise_wa",
    "snotel_css_lab_ca",
    "snotel_snowbird_ut",
    "snotel_niwot_co",
    "harvard_open",
    "harvard_hardwood",
    "marcell_conifer",
    "marcell_deciduous",
    "marcell_open",
    "sleepers_south_open",
    "sleepers_w9_hardwood",
}
EXPECTED_KEYS = {f"{lane}/{cell}" for lane in LANES for cell in CELLS}


def main() -> int:
    attempt = json.loads(ATTEMPT.read_text(encoding="utf-8"))
    if attempt["status"] != "COMPLETE":
        raise RuntimeError("result-bearing attempt is not complete")
    log_lines = EXECUTION_LOG.read_text(encoding="utf-8").splitlines()
    pass_lines = [line for line in log_lines if line.endswith(": PASS")]
    fail_lines = [line for line in log_lines if line.endswith(": FAIL")]
    pass_keys = {line.removesuffix(": PASS") for line in pass_lines}
    if len(pass_lines) != 48 or fail_lines or pass_keys != EXPECTED_KEYS:
        raise RuntimeError("execution log is not an exact 48-PASS/0-FAIL inventory")
    current = build_seal(attempt)
    if SEAL.exists():
        sealed = json.loads(SEAL.read_text(encoding="utf-8"))
        if sealed.get("schema") == "snow-surface-eb04e-retained-output-seal-v1" and legacy_seal(current) == sealed:
            write_json(SEAL, current)
        elif sealed.get("schema") == "snow-surface-eb04e-retained-output-seal-v2" and "verifier_sha256" in sealed:
            sealed.pop("verifier_sha256")
            if current != sealed:
                raise RuntimeError("retained output identity differs from frozen seal")
            write_json(SEAL, current)
        elif current != sealed:
            raise RuntimeError("retained output identity differs from frozen seal")
    else:
        write_json(SEAL, current)
    verification = verify_all(current, attempt)
    anti_alias = anti_alias_checks(current)
    source_exclusion = static_exclusion_audit()
    verification["anti_alias"] = anti_alias
    verification["source_exclusion"] = source_exclusion
    verification["verifier_sha256"] = sha256(Path(__file__))
    verification["passes"] = (
        verification["cell_count"] == 48
        and verification["wat_trace_cross_output_passes"]
        and verification["manifest_identity_passes"]
        and verification["selector_behavior_passes"]
        and verification["non_target_trace_identity_passes"]
        and verification["layer_state_coupling_passes"]
        and verification["finite_operands_pass"]
        and verification["hourly_vector_length_passes"]
        and verification["attempt_pins_match"]
        and verification["exact_inventory_passes"]
        and all(anti_alias.values())
        and all(source_exclusion.values())
    )
    write_json(RESULT, verification)
    print(json.dumps(verification, indent=2, sort_keys=True))
    return 0 if verification["passes"] else 2


def build_seal(attempt: dict[str, Any]) -> dict[str, Any]:
    started = float(attempt["started_unix_seconds"])
    completed = float(attempt["completed_unix_seconds"])
    cells: dict[str, Any] = {}
    for trace in sorted(OUTPUT.glob("*/*/*.snow.jsonl")):
        cell = trace.parent.name
        lane = trace.parent.parent.name
        stem = f"{lane}-{cell}"
        wat = trace.parent / f"{stem}.wat.parquet"
        runfile = trace.parent / f"{stem}.run"
        manifest = trace.parent / "openwepp_hillslope_run_manifest.json"
        stdout = trace.parent / "stdout.txt"
        stderr = trace.parent / "stderr.txt"
        files = [trace, wat, runfile, manifest, stdout, stderr]
        if not all(path.is_file() for path in files):
            raise RuntimeError(f"incomplete retained output set for {lane}/{cell}")
        for path in files:
            metadata = path.stat()
            modified = metadata.st_mtime
            changed = metadata.st_ctime
            if not (started - 1.0 <= modified <= completed + 1.0):
                raise RuntimeError(f"output timestamp outside execution window: {path}")
            if not (started - 1.0 <= changed <= completed + 1.0):
                raise RuntimeError(f"output ctime outside execution window: {path}")
        manifest_value = json.loads(manifest.read_text(encoding="utf-8"))
        cells[f"{lane}/{cell}"] = {
            "lane_id": lane,
            "cell": cell,
            "trace_sha256": sha256(trace),
            "trace_size_bytes": trace.stat().st_size,
            "trace_mtime_ns": trace.stat().st_mtime_ns,
            "trace_ctime_ns": trace.stat().st_ctime_ns,
            "wat_sha256": sha256(wat),
            "wat_size_bytes": wat.stat().st_size,
            "wat_mtime_ns": wat.stat().st_mtime_ns,
            "wat_ctime_ns": wat.stat().st_ctime_ns,
            "runfile_sha256": sha256(runfile),
            "manifest_sha256": sha256(manifest),
            "stdout_sha256": sha256(stdout),
            "stderr_sha256": sha256(stderr),
            "argv": manifest_value["argv"],
            "source_commit": manifest_value["source_commit"],
            "binary_sha256": manifest_value["binary_sha256"],
            "scheduler_outcome_class": manifest_value["execution_provenance"]["scheduler_outcome_class"],
        }
    if set(cells) != EXPECTED_KEYS:
        raise RuntimeError("retained seal does not contain the exact frozen 12x4 inventory")
    return {
        "schema": "snow-surface-eb04e-retained-output-seal-v2",
        "attempt_started_unix_seconds": started,
        "attempt_completed_unix_seconds": completed,
        "attempt_binary_sha256": attempt["binary_sha256"],
        "attempt_tool_sha256": attempt["tool_sha256"],
        "attempt_protocol_sha256": attempt["protocol_sha256"],
        "execution_log_sha256": sha256(EXECUTION_LOG),
        "cells": cells,
    }


def legacy_seal(current: dict[str, Any]) -> dict[str, Any]:
    """Project the v2 seal to the already-frozen v1 fields for one-way migration."""
    projected = json.loads(json.dumps(current))
    projected.pop("verifier_sha256", None)
    projected["schema"] = "snow-surface-eb04e-retained-output-seal-v1"
    for identity in projected["cells"].values():
        identity.pop("trace_ctime_ns", None)
        identity.pop("wat_ctime_ns", None)
    return projected


def verify_all(seal: dict[str, Any], attempt: dict[str, Any]) -> dict[str, Any]:
    maximum_swe_m = maximum_depth_m = 0.0
    maximum_density = maximum_cold = maximum_layer_cold_sum = 0.0
    manifest_ok = selector_ok = non_target_ok = True
    layer_coupling_ok = finite_ok = hourly_vector_ok = True
    enabled_longwave = enabled_sublimation = 0
    for key, identity in seal["cells"].items():
        lane, cell = key.split("/", 1)
        root = OUTPUT / lane / cell
        trace = root / f"{lane}-{cell}.snow.jsonl"
        wat = root / f"{lane}-{cell}.wat.parquet"
        runfile = root / f"{lane}-{cell}.run"
        manifest_path = root / "openwepp_hillslope_run_manifest.json"
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        manifest_ok &= (
            identity["source_commit"] == EXPECTED_HEAD
            and identity["binary_sha256"] == EXPECTED_BINARY
            and identity["scheduler_outcome_class"] == "completed"
            and manifest["runtime_selection"]["selected"] == "direct-production-executor"
            and manifest["runtime_selection"]["fallback_reason"] is None
            and manifest["output_checksums"].get(str(wat.resolve())) == identity["wat_sha256"]
            and manifest["input_checksums"].get(str(runfile.resolve())) == identity["runfile_sha256"]
            and manifest["argv"] == identity["argv"]
            and manifest["execution_provenance"]["executed_day_count"] == manifest["execution_provenance"]["climate_day_count"]
        )
        cross = verify_wat_trace(trace, wat, cell)
        maximum_swe_m = max(maximum_swe_m, cross["maximum_abs_swe_residual_m"])
        maximum_depth_m = max(maximum_depth_m, cross["maximum_abs_depth_residual_m"])
        maximum_density = max(maximum_density, cross["maximum_abs_density_residual_kg_m3"])
        maximum_cold = max(maximum_cold, cross["maximum_abs_layer_cold_content_residual_j_m2"])
        maximum_layer_cold_sum = max(maximum_layer_cold_sum, cross["maximum_abs_layer_cold_sum_residual_j_m2"])
        selector_ok &= cross["selector_behavior_passes"]
        non_target_ok &= cross["non_target_trace_identity_passes"]
        layer_coupling_ok &= cross["layer_state_coupling_passes"]
        finite_ok &= cross["finite_operands_pass"]
        hourly_vector_ok &= cross["hourly_vector_length_passes"]
        enabled_longwave += int(cell in {"L", "LS"} and cross["longwave_nonzero"])
        enabled_sublimation += int(cell in {"S", "LS"} and cross["sublimation_nonzero"])
    selector_ok &= enabled_longwave == 24 and enabled_sublimation == 24
    return {
        "schema": "snow-surface-eb04e-retained-output-verification-v1",
        "evidence_class": "Ran",
        "cell_count": len(seal["cells"]),
        "manifest_identity_passes": manifest_ok,
        "selector_behavior_passes": selector_ok,
        "non_target_trace_identity_passes": non_target_ok,
        "longwave_enabled_nonzero_cells": enabled_longwave,
        "sublimation_enabled_nonzero_cells": enabled_sublimation,
        "maximum_abs_wat_trace_swe_residual_m": maximum_swe_m,
        "maximum_abs_wat_trace_depth_residual_m": maximum_depth_m,
        "maximum_abs_layer_density_residual_kg_m3": maximum_density,
        "maximum_abs_layer_cold_content_residual_j_m2": maximum_cold,
        "maximum_abs_layer_cold_sum_residual_j_m2": maximum_layer_cold_sum,
        "wat_trace_cross_output_passes": maximum_swe_m <= 1.0e-9 and maximum_depth_m <= 1.0e-9,
        "layer_state_coupling_passes": layer_coupling_ok,
        "finite_operands_pass": finite_ok,
        "hourly_vector_length_passes": hourly_vector_ok,
        "attempt_pins_match": (
            attempt["binary_sha256"] == EXPECTED_BINARY
            and attempt["tool_sha256"] == EXPECTED_TOOL
            and attempt["protocol_sha256"] == EXPECTED_PROTOCOL
            and sha256(FROZEN_TOOL) == EXPECTED_TOOL
            and sha256(ARTIFACTS / "prospective-qualification-protocol.md") == EXPECTED_PROTOCOL
        ),
        "exact_inventory_passes": set(seal["cells"]) == EXPECTED_KEYS,
    }


def verify_wat_trace(trace: Path, wat: Path, cell: str) -> dict[str, Any]:
    table = pq.read_table(wat, columns=["sim_day_index", "Snow-Water", "Snow-Depth"])
    columns = table.to_pydict()
    maximum_swe = maximum_depth = 0.0
    maximum_density = maximum_cold = maximum_layer_cold_sum = 0.0
    longwave_nonzero = latent_nonzero = sublimation_nonzero = False
    non_target_ok = True
    layer_coupling_ok = finite_ok = hourly_vector_ok = True
    row_count = 0
    with trace.open(encoding="utf-8") as stream:
        for offset, line in enumerate(stream):
            if not line.strip():
                continue
            if offset >= table.num_rows:
                raise RuntimeError(f"trace exceeds WAT rows: {trace}")
            row = json.loads(line)
            index = columns["sim_day_index"][offset]
            wat_swe_mm = columns["Snow-Water"][offset]
            wat_depth_mm = columns["Snow-Depth"][offset]
            if int(index) != offset + 1 or int(row["day_index"]) != offset:
                raise RuntimeError(f"WAT/trace chronology mismatch: {wat} row {offset}")
            layers = row["snow_layers_after"]
            wat_values = [float(index), float(wat_swe_mm), float(wat_depth_mm or 0.0)]
            finite_ok &= all_finite(row) and all(math.isfinite(value) for value in wat_values)
            hourly_vector_ok &= all(
                isinstance(row[name], list) and len(row[name]) == 24
                for name in (
                    "stage3_hourly_latent_flux_w_m2",
                    "stage3_hourly_latent_heat_j_kg",
                    "stage3_hourly_net_longwave_w_m2",
                    "stage3_hourly_net_shortwave_w_m2",
                    "stage3_hourly_vapor_mass_exchange_kg_m2",
                )
            )
            layer_swe = math.fsum(float(layer["mass_swe_m"]) for layer in layers)
            layer_depth = math.fsum(float(layer["thickness_m"]) for layer in layers)
            layer_coupling_ok &= int(row["snow_layer_count_after"]) == len(layers)
            for layer in layers:
                mass = float(layer["mass_swe_m"])
                depth = float(layer["thickness_m"])
                density = float(layer["density_kg_m3"])
                temperature = float(layer["temperature_c"])
                cold = float(layer["cold_content_j_m2"])
                if mass <= 0.0 or depth <= 0.0 or density <= 0.0 or temperature > 0.0 or cold < 0.0:
                    layer_coupling_ok = False
                    continue
                maximum_density = max(maximum_density, abs(density - 1000.0 * mass / depth))
                if float(row["runtime_swe_after_m"]) > 0.001:
                    maximum_cold = max(maximum_cold, abs(cold - 2100.0 * 1000.0 * mass * (-temperature)))
            maximum_layer_cold_sum = max(
                maximum_layer_cold_sum,
                abs(float(row["stage3_cold_content_after_j_m2"]) - math.fsum(float(layer["cold_content_j_m2"]) for layer in layers)),
            )
            maximum_swe = max(maximum_swe, abs(float(wat_swe_mm) / 1000.0 - row["runtime_swe_after_m"]), abs(float(wat_swe_mm) / 1000.0 - layer_swe))
            wat_depth_m = 0.0 if wat_depth_mm is None else float(wat_depth_mm) / 1000.0
            maximum_depth = max(maximum_depth, abs(wat_depth_m - row["runtime_depth_after_m"]), abs(wat_depth_m - layer_depth))
            longwave_nonzero |= abs(float(row["stage3_longwave_energy_j_m2"])) > 0.0
            latent_nonzero |= abs(float(row["stage3_latent_energy_j_m2"])) > 0.0
            sublimation_nonzero |= float(row["sublimation_m"]) > 0.0
            non_target_ok &= all(row[name] == expected for name, expected in EXPECTED_MODELS.items())
            row_count += 1
    if row_count != table.num_rows:
        raise RuntimeError(f"WAT/trace row mismatch: {wat} {table.num_rows} != {row_count}")
    selector_behavior = (
        (cell in {"L", "LS"}) == longwave_nonzero
        and (cell in {"S", "LS"}) == latent_nonzero
        and (cell in {"S", "LS"}) == sublimation_nonzero
    )
    layer_coupling_ok &= maximum_density <= 1.0e-4 and maximum_cold <= 1.0e-6 and maximum_layer_cold_sum <= 1.0e-6
    return {
        "maximum_abs_swe_residual_m": maximum_swe,
        "maximum_abs_depth_residual_m": maximum_depth,
        "maximum_abs_density_residual_kg_m3": maximum_density,
        "maximum_abs_layer_cold_content_residual_j_m2": maximum_cold,
        "maximum_abs_layer_cold_sum_residual_j_m2": maximum_layer_cold_sum,
        "longwave_nonzero": longwave_nonzero,
        "sublimation_nonzero": sublimation_nonzero,
        "selector_behavior_passes": selector_behavior,
        "non_target_trace_identity_passes": non_target_ok,
        "layer_state_coupling_passes": layer_coupling_ok,
        "finite_operands_pass": finite_ok,
        "hourly_vector_length_passes": hourly_vector_ok,
    }


def all_finite(value: Any) -> bool:
    """Fail closed on every numeric value retained in a trace record."""
    if isinstance(value, bool) or value is None or isinstance(value, str):
        return True
    if isinstance(value, (int, float)):
        return math.isfinite(float(value))
    if isinstance(value, list):
        return all(all_finite(item) for item in value)
    if isinstance(value, dict):
        return all(all_finite(item) for item in value.values())
    return False


def anti_alias_checks(seal: dict[str, Any]) -> dict[str, bool]:
    fragment_key = next(
        key
        for key in sorted(seal["cells"])
        if trace_has_fragment(OUTPUT / key / f"{key.replace('/', '-')}.snow.jsonl")
    )
    identity = seal["cells"][fragment_key]
    trace = OUTPUT / fragment_key / f"{fragment_key.replace('/', '-')}.snow.jsonl"
    wat = OUTPUT / fragment_key / f"{fragment_key.replace('/', '-')}.wat.parquet"
    target_index = fragment_row_index(trace)
    check_root = REPO / "target/eb04e_verifier_self_check"
    check_root.mkdir(parents=True, exist_ok=True)
    deleted_path = check_root / "deleted-fragment.jsonl"
    aggregate_path = check_root / "aggregate-only.jsonl"
    write_mutated_jsonl(
        trace,
        deleted_path,
        target_index,
        lambda row: row.update(
            snow_layers_after=[
                layer
                for layer in row["snow_layers_after"]
                if not (1.0e-12 < float(layer["mass_swe_m"]) <= 1.0e-9)
            ]
        ),
    )
    deleted_audit = verify_wat_trace(deleted_path, wat, fragment_key.split("/")[1])
    def mutate_aggregate(row: dict[str, Any]) -> None:
        row["snow_layers_after"][0]["mass_swe_m"] += 1.0e-5

    write_mutated_jsonl(trace, aggregate_path, target_index, mutate_aggregate)
    aggregate_audit = verify_wat_trace(aggregate_path, wat, fragment_key.split("/")[1])
    baseline_key = next(key for key in sorted(seal["cells"]) if key.endswith("/B"))
    baseline_trace = OUTPUT / baseline_key / f"{baseline_key.replace('/', '-')}.snow.jsonl"
    baseline_wat = OUTPUT / baseline_key / f"{baseline_key.replace('/', '-')}.wat.parquet"
    selector_path = check_root / "wrong-selector.jsonl"
    write_mutated_jsonl(
        baseline_trace,
        selector_path,
        0,
        lambda row: row.update(stage3_latent_energy_j_m2=1.0),
    )
    selector_audit = verify_wat_trace(selector_path, baseline_wat, "B")
    return {
        "sealed_trace_rejects_fragment_deletion": sha256(deleted_path) != identity["trace_sha256"],
        "layer_count_audit_rejects_fragment_deletion": not deleted_audit["layer_state_coupling_passes"],
        "cross_output_audit_rejects_aggregate_only_substitution": aggregate_audit["maximum_abs_swe_residual_m"] > 1.0e-9,
        "negative_selector_control_rejects_wrong_latent_path": not selector_audit["selector_behavior_passes"],
    }


def static_exclusion_audit() -> dict[str, bool]:
    source = FROZEN_TOOL.read_text(encoding="utf-8")
    tree = ast.parse(source)
    calls = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Call):
            if isinstance(node.func, ast.Name):
                calls.add(node.func.id)
            elif isinstance(node.func, ast.Attribute):
                calls.add(node.func.attr)
    forbidden = {"load_observations", "rubric_profile", "model_metrics", "factorial_effects", "aggregate_rubric", "compensation_audit"}
    report = json.loads((ARTIFACTS / "qualification-results.json").read_text(encoding="utf-8"))
    serialized = json.dumps(report, sort_keys=True)
    return {
        "no_observation_or_rubric_call": not bool(calls & forbidden),
        "no_empirical_result_payload": not any(token in serialized for token in ('"rubric_profile"', '"observation_metrics"', '"factorial_effects"', '"interaction_residual"', '"promotion_outcome"')),
        "no_production_or_test_diff": command_sha256(["git", "diff", "--binary", "--", "crates", "tests"]) == hashlib.sha256(b"").hexdigest(),
    }


def trace_has_fragment(path: Path) -> bool:
    with path.open(encoding="utf-8") as stream:
        for line in stream:
            row = json.loads(line)
            if any(1.0e-12 < float(layer["mass_swe_m"]) <= 1.0e-9 for layer in row["snow_layers_after"]):
                return True
    return False


def fragment_row_index(path: Path) -> int:
    with path.open(encoding="utf-8") as stream:
        for index, line in enumerate(stream):
            row = json.loads(line)
            if any(1.0e-12 < float(layer["mass_swe_m"]) <= 1.0e-9 for layer in row["snow_layers_after"]):
                return index
    raise RuntimeError(f"no represented subnanometer layer in {path}")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def command_sha256(command: list[str]) -> str:
    import subprocess

    output = subprocess.run(command, cwd=REPO, check=True, stdout=subprocess.PIPE).stdout
    return hashlib.sha256(output).hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_mutated_jsonl(
    source: Path,
    destination: Path,
    target_index: int,
    mutate: Any,
) -> None:
    with source.open(encoding="utf-8") as reader, destination.open("w", encoding="utf-8") as writer:
        for index, line in enumerate(reader):
            if index != target_index:
                writer.write(line)
                continue
            row = json.loads(line)
            mutate(row)
            writer.write(json.dumps(row, separators=(",", ":")) + "\n")


if __name__ == "__main__":
    raise SystemExit(main())
