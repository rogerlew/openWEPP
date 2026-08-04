#!/usr/bin/env python3
"""Independently verify accepted v3 snow trace identities without writes."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
ROOT = REPO / (
    "target/snow_prepeak_liquid_evacuation_physics_audit_v3/"
    "runs/baseline_replay"
)
COMPONENTS = (
    "coe_melt_amelt_m",
    "coe_melt_bmelt_m",
    "coe_melt_cmelt_m",
    "coe_melt_dmelt_m",
)
LIMITS = {
    "coe_uncapped_term_sum_m_per_hour": 1.0e-12,
    "coe_applied_identity_m_per_hour": 1.0e-12,
    "daily_raw_sum_m": 1.0e-12,
    "daily_primitive_mass_m": 1.0e-12,
    "routed_alias_m": 1.0e-12,
    "wet_compaction_input_identity_m": 1.0e-12,
    "stage3_energy_j_m2": 1.0e-6,
    "stage3_energy_trace_difference_j_m2": 1.0e-6,
}


def update_maximum(values: dict[str, float], key: str, residual: float) -> None:
    values[key] = max(values[key], abs(residual))


def main() -> int:
    maxima = {key: 0.0 for key in LIMITS}
    daily_rows = 0
    hourly_rows = 0
    traces: dict[str, dict[str, Any]] = {}
    for path in sorted(ROOT.glob("*/*.snow.jsonl")):
        digest = hashlib.sha256()
        with path.open("rb") as handle:
            for raw in handle:
                digest.update(raw)
                row = json.loads(raw)
                daily_rows += 1
                applied_sum = 0.0
                hourly = row["accumulation_melt_hourly"]
                hourly_rows += len(hourly)
                for hour in hourly:
                    component_sum = sum(float(hour[field]) for field in COMPONENTS)
                    uncapped = float(hour["coe_melt_uncapped_m"])
                    applied = float(hour["coe_melt_applied_m"])
                    adjustment = float(hour["coe_melt_cap_adjustment_m"])
                    update_maximum(
                        maxima,
                        "coe_uncapped_term_sum_m_per_hour",
                        uncapped - component_sum,
                    )
                    update_maximum(
                        maxima,
                        "coe_applied_identity_m_per_hour",
                        applied - uncapped - adjustment,
                    )
                    applied_sum += applied
                update_maximum(
                    maxima,
                    "daily_raw_sum_m",
                    float(row["raw_melt_m"]) - applied_sum,
                )
                expected_delta = (
                    float(row["accumulation_m"])
                    + float(row["rain_retained_m"])
                    - float(row["snowpack_swe_loss_m"])
                    - float(row["sublimation_m"])
                )
                actual_delta = float(row["runtime_swe_after_m"]) - float(
                    row["runtime_swe_before_m"]
                )
                update_maximum(
                    maxima,
                    "daily_primitive_mass_m",
                    actual_delta - expected_delta,
                )
                loss = float(row["snowpack_swe_loss_m"])
                rain = float(row["rain_released_m"])
                routed = float(row["routed_melt_m"])
                update_maximum(
                    maxima, "routed_alias_m", routed - loss - rain
                )
                wet_input = float(
                    row["density_process_liquid_for_compaction_mass_kg_m2"]
                ) / 1000.0
                update_maximum(
                    maxima,
                    "wet_compaction_input_identity_m",
                    wet_input - 2.0 * loss - rain,
                )
                energy = (
                    float(row["stage3_surface_energy_j_m2"])
                    + float(row["stage3_conduction_energy_j_m2"])
                    + float(row["stage3_latent_refreeze_energy_j_m2"])
                    + float(row["stage3_cold_content_export_j_m2"])
                    - (
                        float(row["stage3_cold_content_before_j_m2"])
                        - float(row["stage3_cold_content_after_j_m2"])
                    )
                )
                update_maximum(maxima, "stage3_energy_j_m2", energy)
                update_maximum(
                    maxima,
                    "stage3_energy_trace_difference_j_m2",
                    energy - float(row["stage3_energy_closure_residual_j_m2"]),
                )
        traces[path.parent.name] = {
            "path": str(path.relative_to(REPO)),
            "sha256": digest.hexdigest(),
        }
    if len(traces) != 4:
        raise RuntimeError(f"expected four accepted reference traces, found {len(traces)}")
    failures = {
        key: {"observed": maxima[key], "limit": limit}
        for key, limit in LIMITS.items()
        if maxima[key] > limit
    }
    result = {
        "status": "PASS" if not failures else "FAIL",
        "daily_rows": daily_rows,
        "hourly_rows": hourly_rows,
        "maximum_absolute_residuals": maxima,
        "limits": LIMITS,
        "traces": traces,
        "failures": failures,
    }
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0 if not failures else 1


if __name__ == "__main__":
    raise SystemExit(main())
