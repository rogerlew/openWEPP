#!/usr/bin/env python3
"""Run and independently verify the Snowbird schema-v4 trace closure."""

from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import math
import os
from pathlib import Path
import shutil
import subprocess
import sys
from typing import Any


REPO = Path(__file__).resolve().parents[4]
OUTPUT_ROOT = REPO / "target/snow_stage3_liquid_signed_hour_trace_closure"
SOURCE_FIXTURE = (
    REPO
    / "target/snow_prepeak_liquid_evacuation_physics_audit_v3"
    / "fixtures/baseline_replay/snotel_snowbird_ut"
)
SITE = "snotel_snowbird_ut"
TOLERANCE_M = 1.0e-9

NEW_TOP_LEVEL_FIELDS = {
    "wind_m_s",
    "dewpoint_c",
    "canopy_cover_fraction",
    "stage3_incoming_liquid_m",
    "stage3_routed_liquid_m",
    "stage3_retained_liquid_delta_m",
    "stage3_liquid_closure_residual_m",
    "stage3_hourly_active_mass_kg_m2",
    "stage3_hourly_active_depth_m",
    "stage3_hourly_active_temperature_c",
    "stage3_hourly_active_cold_content_j_m2",
    "stage3_hourly_lower_present_fraction",
    "stage3_hourly_lower_mass_kg_m2",
    "stage3_hourly_lower_depth_m",
    "stage3_hourly_lower_temperature_c",
    "stage3_hourly_lower_cold_content_j_m2",
}
NEW_HOURLY_FIELDS = {
    "air_temperature_c",
    "radiation_mj_m2",
    "cloud_fraction",
    "routed_melt_m",
    "liquid_holding_capacity_m",
    "liquid_water_retained_before_m",
    "liquid_water_retained_after_m",
    "liquid_water_released_m",
    "rain_released_m",
    "sublimation_m",
    "pack_depth_before_m",
    "pack_depth_after_m",
    "pack_density_before_kg_m3",
    "pack_density_after_kg_m3",
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def run_paths(label: str) -> dict[str, Path]:
    run_dir = OUTPUT_ROOT / "runs" / label / SITE
    base = run_dir / f"{SITE}-{label}"
    return {
        "run_dir": run_dir,
        "fixture": OUTPUT_ROOT / "fixtures" / label / SITE,
        "runfile": base.with_suffix(".run"),
        "trace": base.with_suffix(".snow.jsonl"),
        "wat": base.with_suffix(".wat.parquet"),
        "hbp": base.with_suffix(".hbp"),
        "loss": base.with_suffix(".loss.json"),
        "stdout": base.with_suffix(".stdout.txt"),
        "stderr": base.with_suffix(".stderr.txt"),
        "receipt": base.with_suffix(".receipt.json"),
    }


def run_fixture(label: str, binary: Path) -> None:
    paths = run_paths(label)
    if paths["run_dir"].exists() or paths["fixture"].exists():
        raise RuntimeError(f"refusing to overwrite existing label: {label}")
    if not SOURCE_FIXTURE.is_dir():
        raise RuntimeError(f"missing retained Snowbird fixture: {SOURCE_FIXTURE}")
    binary = binary.resolve()
    if not binary.is_file():
        raise RuntimeError(f"missing binary: {binary}")

    paths["run_dir"].mkdir(parents=True)
    paths["fixture"].parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(SOURCE_FIXTURE, paths["fixture"])
    fixture = paths["fixture"]
    runfile = f'''schema = "openwepp-hillslope-runfile-v1"
run_name = "snow-stage3-trace-closure-{label}"
unit_system = "metric"

[inputs]
soil = "{fixture / 'p8.sol'}"
management = "{fixture / 'p8.man'}"
slope = "{fixture / 'p8.slp'}"
climate = "{fixture / 'p8.cli'}"
wepp_ui = false

[outputs]
pass = "{paths['hbp']}"
loss = "{paths['loss']}"
wat = "{paths['wat']}"
'''
    paths["runfile"].write_text(runfile)

    effective = {
        "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL": "layered_thermal_liquid_v1",
        "OPENWEPP_R7H_SNOW_TRACE_PATH": str(paths["trace"]),
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL": "physics_bulk_multilayer_density_v1",
        "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL": "harder_pomeroy_hourly",
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL": "coe_liquid_holding_capacity_v1",
        "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": "disabled",
        "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL": "disabled",
    }
    env = {key: value for key, value in os.environ.items() if not key.startswith("OPENWEPP_")}
    env.update(effective)
    argv = [
        str(binary),
        "--run-dir",
        str(fixture),
        "--run-file",
        str(paths["runfile"]),
        "--output-dir",
        str(paths["run_dir"]),
        "--legacy-sidecar-discovery",
        "--direct-production-executor",
    ]
    completed = subprocess.run(argv, env=env, capture_output=True, check=False)
    paths["stdout"].write_bytes(completed.stdout)
    paths["stderr"].write_bytes(completed.stderr)
    receipt = {
        "argv": argv,
        "binary_sha256": sha256(binary),
        "effective_openwepp_environment": effective,
        "label": label,
        "returncode": completed.returncode,
    }
    for name in ("trace", "wat", "hbp", "loss"):
        path = paths[name]
        if path.is_file():
            receipt[name] = {
                "path": str(path.relative_to(REPO)),
                "sha256": sha256(path),
                "size_bytes": path.stat().st_size,
            }
    write_json(paths["receipt"], receipt)
    if completed.returncode != 0:
        raise RuntimeError(f"release CLI failed; see {paths['stderr']}")


def projected_v3(row: dict[str, Any]) -> dict[str, Any]:
    projected = dict(row)
    if projected.get("schema") != "openwepp-r7h-direct-production-snow-trace-v4":
        raise RuntimeError(f"unexpected v4 schema: {projected.get('schema')}")
    projected["schema"] = "openwepp-r7h-direct-production-snow-trace-v3"
    for field in NEW_TOP_LEVEL_FIELDS:
        projected.pop(field, None)
    projected["accumulation_melt_hourly"] = [
        {key: value for key, value in hour.items() if key not in NEW_HOURLY_FIELDS}
        for hour in projected["accumulation_melt_hourly"]
    ]
    return projected


def finite(value: Any) -> bool:
    return isinstance(value, (int, float)) and math.isfinite(value)


def verify(pre_label: str, post_label: str, report_path: Path) -> None:
    pre = run_paths(pre_label)
    post = run_paths(post_label)
    for path in (
        pre["trace"],
        pre["wat"],
        pre["hbp"],
        post["trace"],
        post["wat"],
        post["hbp"],
    ):
        if not path.is_file():
            raise RuntimeError(f"missing verification input: {path}")

    counts = {
        "rows": 0,
        "stage3_enabled_rows": 0,
        "stage3_nonzero_incoming_rows": 0,
        "stage3_disabled_nonzero_operand_rows": 0,
        "mixed_signed_hour_rows": 0,
        "mixed_stage3_nonzero_incoming_rows": 0,
        "mixed_stage3_nonzero_routed_rows": 0,
        "mixed_stage3_nonzero_retained_delta_rows": 0,
        "mixed_stage3_nonzero_refrozen_rows": 0,
        "mixed_stage3_all_nonzero_operand_rows": 0,
        "alias_omit_retained_rejected_rows": 0,
        "alias_top_level_routed_rejected_rows": 0,
        "alias_coe_retained_store_rejected_rows": 0,
        "alias_double_refreeze_rejected_rows": 0,
        "pre_v4_projection_mismatches": 0,
    }
    aggregates_m = {
        "all_stage3_incoming": 0.0,
        "all_stage3_routed": 0.0,
        "all_stage3_retained_delta": 0.0,
        "all_stage3_refrozen": 0.0,
        "mixed_stage3_incoming": 0.0,
        "mixed_stage3_routed": 0.0,
        "mixed_stage3_retained_delta": 0.0,
        "mixed_stage3_refrozen": 0.0,
    }
    maximum_closure_error_m = 0.0
    first_projection_mismatch: dict[str, Any] | None = None
    required_stage3 = {
        "stage3_incoming_liquid_m",
        "stage3_routed_liquid_m",
        "stage3_retained_liquid_delta_m",
        "stage3_refrozen_liquid_m",
        "stage3_liquid_closure_residual_m",
    }
    required_arrays = NEW_TOP_LEVEL_FIELDS - required_stage3 - {
        "wind_m_s",
        "dewpoint_c",
        "canopy_cover_fraction",
    }

    with pre["trace"].open() as before, post["trace"].open() as after:
        for line_number, pair in enumerate(itertools.zip_longest(before, after), start=1):
            old_line, new_line = pair
            if old_line is None or new_line is None:
                raise RuntimeError(f"trace row-count mismatch at line {line_number}")
            old = json.loads(old_line)
            new = json.loads(new_line)
            counts["rows"] += 1
            projected = projected_v3(new)
            if old != projected:
                counts["pre_v4_projection_mismatches"] += 1
                if first_projection_mismatch is None:
                    keys = sorted(key for key in old.keys() | projected.keys() if old.get(key) != projected.get(key))
                    first_projection_mismatch = {
                        "line": line_number,
                        "differing_keys": keys[:20],
                    }

            if not required_stage3.issubset(new):
                raise RuntimeError(f"line {line_number} omits Stage-3 liquid fields")
            for field in required_arrays:
                values = new.get(field)
                if not isinstance(values, list) or len(values) != 24 or not all(map(finite, values)):
                    raise RuntimeError(f"line {line_number} invalid {field}")
            hours = new.get("accumulation_melt_hourly")
            if not isinstance(hours, list) or len(hours) != 24:
                raise RuntimeError(f"line {line_number} has invalid hourly rows")
            for hour in hours:
                if not NEW_HOURLY_FIELDS.issubset(hour):
                    raise RuntimeError(f"line {line_number} omits signed-hour fields")
                if not all(finite(hour[field]) for field in NEW_HOURLY_FIELDS):
                    raise RuntimeError(f"line {line_number} has nonfinite signed-hour fields")

            incoming = new["stage3_incoming_liquid_m"]
            routed = new["stage3_routed_liquid_m"]
            retained = new["stage3_retained_liquid_delta_m"]
            refrozen = new["stage3_refrozen_liquid_m"]
            residual = new["stage3_liquid_closure_residual_m"]
            reconstruction = incoming - routed - retained - refrozen
            closure_error = abs(reconstruction - residual)
            maximum_closure_error_m = max(maximum_closure_error_m, closure_error)
            if closure_error > TOLERANCE_M:
                raise RuntimeError(f"line {line_number} Stage-3 closure error {closure_error}")
            if new["stage3_energy_enabled"]:
                counts["stage3_enabled_rows"] += 1
            elif any(abs(value) > TOLERANCE_M for value in (incoming, routed, retained, refrozen)):
                counts["stage3_disabled_nonzero_operand_rows"] += 1
            if incoming > TOLERANCE_M:
                counts["stage3_nonzero_incoming_rows"] += 1
            aggregates_m["all_stage3_incoming"] += incoming
            aggregates_m["all_stage3_routed"] += routed
            aggregates_m["all_stage3_retained_delta"] += retained
            aggregates_m["all_stage3_refrozen"] += refrozen

            aliases = {
                "alias_omit_retained_rejected_rows": incoming - routed - refrozen,
                "alias_top_level_routed_rejected_rows": (
                    incoming - new["routed_melt_m"] - retained - refrozen
                ),
                "alias_coe_retained_store_rejected_rows": (
                    incoming - routed - new["liquid_water_retained_after_m"] - refrozen
                ),
                "alias_double_refreeze_rejected_rows": (
                    incoming - routed - retained - 2.0 * refrozen
                ),
            }
            for name, alias in aliases.items():
                if abs(alias - residual) > TOLERANCE_M:
                    counts[name] += 1

            positive = sum(max(hour["coe_melt_applied_m"], 0.0) for hour in hours)
            negative = sum(min(hour["coe_melt_applied_m"], 0.0) for hour in hours)
            if positive > TOLERANCE_M and negative < -TOLERANCE_M:
                counts["mixed_signed_hour_rows"] += 1
                aggregates_m["mixed_stage3_incoming"] += incoming
                aggregates_m["mixed_stage3_routed"] += routed
                aggregates_m["mixed_stage3_retained_delta"] += retained
                aggregates_m["mixed_stage3_refrozen"] += refrozen
                for name, value in [
                    ("mixed_stage3_nonzero_incoming_rows", incoming),
                    ("mixed_stage3_nonzero_routed_rows", routed),
                    ("mixed_stage3_nonzero_retained_delta_rows", retained),
                    ("mixed_stage3_nonzero_refrozen_rows", refrozen),
                ]:
                    if abs(value) > TOLERANCE_M:
                        counts[name] += 1
                if all(abs(value) > TOLERANCE_M for value in (incoming, routed, retained, refrozen)):
                    counts["mixed_stage3_all_nonzero_operand_rows"] += 1

    wat_equal = sha256(pre["wat"]) == sha256(post["wat"])
    hbp_equal = sha256(pre["hbp"]) == sha256(post["hbp"])
    required_positive_counts = [
        "stage3_enabled_rows",
        "stage3_nonzero_incoming_rows",
        "mixed_signed_hour_rows",
        "mixed_stage3_nonzero_incoming_rows",
        "mixed_stage3_nonzero_routed_rows",
        "mixed_stage3_nonzero_retained_delta_rows",
        "mixed_stage3_nonzero_refrozen_rows",
        "mixed_stage3_all_nonzero_operand_rows",
        "alias_omit_retained_rejected_rows",
        "alias_top_level_routed_rejected_rows",
        "alias_coe_retained_store_rejected_rows",
        "alias_double_refreeze_rejected_rows",
    ]
    passed = (
        wat_equal
        and hbp_equal
        and counts["pre_v4_projection_mismatches"] == 0
        and counts["stage3_disabled_nonzero_operand_rows"] == 0
        and maximum_closure_error_m <= TOLERANCE_M
        and all(counts[name] > 0 for name in required_positive_counts)
    )
    report = {
        "aggregates_m": aggregates_m,
        "counts": counts,
        "first_projection_mismatch": first_projection_mismatch,
        "maximum_stage3_closure_error_m": maximum_closure_error_m,
        "passed": passed,
        "post_trace": {
            "path": str(post["trace"].relative_to(REPO)),
            "sha256": sha256(post["trace"]),
            "size_bytes": post["trace"].stat().st_size,
        },
        "pre_trace": {
            "path": str(pre["trace"].relative_to(REPO)),
            "sha256": sha256(pre["trace"]),
            "size_bytes": pre["trace"].stat().st_size,
        },
        "trace_size_ratio_v4_to_v3": post["trace"].stat().st_size / pre["trace"].stat().st_size,
        "tolerance_m": TOLERANCE_M,
        "hbp_pass_identity": {
            "equal": hbp_equal,
            "post_sha256": sha256(post["hbp"]),
            "pre_sha256": sha256(pre["hbp"]),
        },
        "wat_identity": {
            "equal": wat_equal,
            "post_sha256": sha256(post["wat"]),
            "pre_sha256": sha256(pre["wat"]),
        },
    }
    report_path.parent.mkdir(parents=True, exist_ok=True)
    write_json(report_path, report)
    if not passed:
        raise RuntimeError(f"trace closure verification failed; see {report_path}")


def main() -> int:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)
    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("--label", required=True)
    run_parser.add_argument("--binary", type=Path, required=True)
    verify_parser = subparsers.add_parser("verify")
    verify_parser.add_argument("--pre-label", default="pre_v4")
    verify_parser.add_argument("--post-label", default="post_v4")
    verify_parser.add_argument(
        "--report",
        type=Path,
        default=OUTPUT_ROOT / "reconstruction-report.json",
    )
    arguments = parser.parse_args()
    if arguments.command == "run":
        run_fixture(arguments.label, arguments.binary)
    else:
        verify(arguments.pre_label, arguments.post_label, arguments.report)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, RuntimeError, ValueError, json.JSONDecodeError) as error:
        print(f"trace_closure.py: {error}", file=sys.stderr)
        raise SystemExit(1) from error
