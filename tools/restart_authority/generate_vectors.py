#!/usr/bin/env python3
"""Generate canonical nontrivial restart authority vectors and exact-shape schema."""

from __future__ import annotations

import copy
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/artifacts"


def h64(value: float) -> str:
    import struct
    return "0x" + struct.pack(">d", value).hex()


def sha(label: str) -> str:
    return hashlib.sha256(label.encode()).hexdigest()


def parcel(label: str, mass: float) -> dict:
    return {
        "parcel_id": label,
        "source_owner_id": "climate-owner",
        "destination_ofe_id": "ofe-1",
        "destination_tile_id": "forest-1",
        "start_s": h64(0.0),
        "end_s": h64(1800.0),
        "mass_kg_m2": h64(mass),
        "temperature_k": h64(293.15),
        "enthalpy_j_m2": h64(mass * 83600.0),
        "receipt_sha256": sha(label),
    }


def cursor(day: int, carry: list[dict]) -> dict:
    return {
        "schema_version": "SNOW_FREE_HALF_HOUR_PROVIDER_CURSOR_RESTART_V1",
        "next_day_index": day,
        "static_configuration_sha256": sha("static-forcing"),
        "pending_carry": carry,
        "cursor_sha256": sha(f"cursor-{day}-{len(carry)}"),
    }


def forcing_dest(ofe: str, tile: str, covered: bool) -> dict:
    intervals = []
    for index in range(48):
        par = 0.0 if index < 12 or index > 36 else (25.0 if index == 12 else 350.0)
        intervals.append({
            "interval_index": index,
            "start_s": index * 1800,
            "end_s": (index + 1) * 1800,
            "air_temperature_c": h64(4.0 + index / 4.0),
            "vpd_kpa": h64(0.4 + index / 100.0),
            "global_horizontal_shortwave_w_m2": h64(par),
            "precipitation_parcels": [],
            "interval_receipt_sha256": sha(f"{ofe}-{tile}-{index}"),
        })
    return {
        "ofe_id": ofe,
        "tile_id": tile,
        "covered": covered,
        "wb14_configuration_sha256": sha(f"wb14-{ofe}-{tile}"),
        "intervals": intervals,
        "receipt_sha256": sha(f"day-{ofe}-{tile}"),
    }


def owners(tag: str, state_shift: float = 0.0) -> dict:
    return {
        "gsi_configuration": {
            "schema_version": "DIRECT_GSI_OWNER_CONFIGURATION_V1",
            "owner_id": "canonical-v10-gsi-owner",
            "minimum_temperature_inactive_c": h64(-3.5),
            "minimum_temperature_unconstrained_c": h64(6.25),
            "vapor_pressure_deficit_unconstrained_pa": h64(850.0),
            "vapor_pressure_deficit_inactive_pa": h64(3900.0),
            "photoperiod_inactive_hours": h64(9.75),
            "photoperiod_unconstrained_hours": h64(11.25),
            "latitude_degrees": h64(41.1),
            "configuration_sha256": sha("gsi-config"),
        },
        "gsi_state": {
            "history_oldest_first": [h64(0.05 + index / 40.0) for index in range(21)],
            "last_date": {"year": 2000, "ordinal_day": 171},
            "state_sha256": sha(f"gsi-state-{tag}"),
        },
        "static_forcing_configuration": {
            "schema_version": "SNOW_FREE_HALF_HOUR_STATIC_CONFIGURATION_RESTART_V1",
            "run_id": "restart-authority-run",
            "co2_pa": h64(42.0),
            "reference_height_m": h64(20.0),
            "gsi_owner_configuration_sha256": sha("gsi-config"),
            "destinations": [
                {"ofe_id": "ofe-1", "tile_id": "forest-1", "wb14_configuration_sha256": sha("wb14-ofe-1-forest-1")},
                {"ofe_id": "ofe-2", "tile_id": "open-1", "wb14_configuration_sha256": sha("wb14-ofe-2-open-1")},
            ],
            "configuration_sha256": sha("static-forcing"),
        },
        "forcing_provider_cursor": cursor(12, []),
        "vegetation_v10_configuration": {"schema_version": "OPENWEPP_C3_WOODY_V10", "owner_id": "vegetation", "configuration_sha256": sha("veg-config")},
        "vegetation_v10_state": {"last_transaction_id": "0x00000000000000000000000000000123", "leaf_water_kg_m2": h64(0.15 + state_shift), "root_potential_pa": h64(-450000.0), "state_sha256": sha(f"veg-{tag}")},
        "lse_v2_configuration": {"schema_version": "OPENWEPP_SNOW_FREE_LSE_V2", "owner_id": "lse", "configuration_sha256": sha("lse-config")},
        "lse_v2_state": {"last_transaction_id": "0x00000000000000000000000000000123", "ground_temperature_k": h64(286.4 + state_shift), "state_sha256": sha(f"lse-{tag}")},
        "direct_hydrology": {
            "schema_version": "OPENWEPP_DIRECT_HYDROLOGY_RESTART_V1",
            "phase_plan_sha256": sha("phase-plan"),
            "day_inputs_sha256": sha("day-inputs-12"),
            "lanes": [
                {
                    "lane_id": "lane-1", "upstream_lane_id": None, "downstream_lane_id": None,
                    "area_m2": h64(1200.0),
                    "water": {"surface_runoff_kg_m2": h64(1.25 + state_shift), "infiltration_kg_m2": h64(3.5), "soil_water_kg_m2": h64(82.0)},
                    "transfer": {"inbound_kg": h64(2.0), "outbound_kg": h64(1.5), "custody_sha256": sha(f"transfer-{tag}")},
                    "subsurface_layers": [
                        {"layer_index": 0, "thickness_m": h64(0.1), "liquid_water_kg_m2": h64(18.0), "ice_water_kg_m2": h64(0.0)},
                        {"layer_index": 1, "thickness_m": h64(0.3), "liquid_water_kg_m2": h64(64.0), "ice_water_kg_m2": h64(-0.0)},
                    ],
                    "erosion_runtime": {"sediment_carry_kg": h64(0.75), "receipt_sha256": sha(f"erosion-{tag}")},
                }
            ],
            "lane_transfer_ledger": [{"source_lane_id": "lane-1", "destination_lane_id": "outlet", "water_kg": h64(1.5), "receipt_sha256": sha(f"ledger-{tag}")}],
            "lane_transfer_downstream_operands": {"outlet_runoff_kg": h64(1.5), "receipt_sha256": sha(f"downstream-{tag}")},
            "groundwater": {"enabled": True, "storage_kg_m2": h64(14.0 + state_shift), "baseflow_kg_m2": h64(0.2), "state_sha256": sha(f"groundwater-{tag}")},
            "surface_liquid_owned_state": {"owner_id": "surface-liquid", "liquid_kg_m2": h64(0.35 + state_shift), "temperature_k": h64(285.0), "state_sha256": sha(f"surface-{tag}")},
            "state_sha256": sha(f"hydrology-{tag}"),
        },
        "surface_liquid_configuration": {"schema_version": "DIRECT_SURFACE_LIQUID_CONFIGURATION_V1", "owner_id": "surface-liquid", "configuration_sha256": sha("surface-config")},
        "soil_thermal_configuration": {"schema_version": "DIRECT_SOIL_THERMAL_CONFIGURATION_V1", "owner_id": "soil-thermal", "configuration_sha256": sha("thermal-config")},
        "soil_thermal_state": {"layer_temperatures_k": [h64(284.5), h64(283.25)], "state_sha256": sha(f"thermal-{tag}")},
        "biogeochemistry_configuration": {"schema_version": "DIRECT_BGC_CONFIGURATION_V1", "owner_id": "bgc", "configuration_sha256": sha("bgc-config")},
        "biogeochemistry_state": {"litter_carbon_kg_m2": h64(0.8), "soil_carbon_kg_m2": h64(6.4 + state_shift), "state_sha256": sha(f"bgc-{tag}")},
    }


def base(phase: dict) -> dict:
    value = {
        "schema": "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1",
        "version": 1,
        "run_identity": {"run_id": "restart-authority-run", "run_configuration_sha256": sha("run-config")},
        "topology": {"lane_ids": ["lane-1"], "destinations": [{"ofe_id": "ofe-1", "tile_id": "forest-1"}, {"ofe_id": "ofe-2", "tile_id": "open-1"}], "topology_sha256": sha("topology")},
        "configuration_identities": {"climate_repository_sha256": sha("climate-repository"), "phase_plan_sha256": sha("phase-plan"), "configuration_set_sha256": sha("configuration-set")},
        "transaction_lineage": {"last_accepted_transaction_id": "0x00000000000000000000000000000123", "lineage_sha256": sha("lineage")},
        "phase": phase,
    }
    payload = json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode()
    value["payload_sha256"] = hashlib.sha256(payload).hexdigest()
    return value


boundary = base({"kind": "between_days", "committed_owners": owners("boundary"), "next_day_index": 12, "accepted_interval_count": 576})
receipts = [forcing_dest("ofe-1", "forest-1", True), forcing_dest("ofe-2", "open-1", False)]
gsi_receipt = {
    "schema_version": "DIRECT_GSI_DAILY_RECEIPT_V1", "owner_id": "canonical-v10-gsi-owner",
    "run_id": "restart-authority-run", "day_index": 12, "source_climate_sha256": sha("climate-day-12"),
    "beginning_state_sha256": sha("gsi-state-begin"), "ending_state_sha256": sha("gsi-state-end"),
    "configuration_sha256": sha("gsi-config"), "forcing_sha256": sha("gsi-forcing"),
    "result_sha256": sha("gsi-result"), "receipt_sha256": sha("gsi-receipt"),
}
in_progress = base({
    "kind": "in_progress_day", "day_index": 12, "next_interval_index": 24,
    "committed_beginning_owners": owners("beginning"), "staged_candidate_owners": owners("interval-24", 0.125),
    "accepted_gsi_daily_receipt": gsi_receipt,
    "staged_gsi_ending_state": {"history_oldest_first": [h64(0.1 + i / 50.0) for i in range(21)], "last_date": {"year": 2000, "ordinal_day": 172}, "state_sha256": sha("gsi-state-end")},
    "beginning_provider_cursor": cursor(12, []), "ending_provider_cursor": cursor(13, []),
    "validated_forcing_day_receipts": receipts, "accepted_interval_count": 600,
})
cross = copy.deepcopy(in_progress)
cross["phase"]["ending_provider_cursor"] = cursor(13, [parcel("cross-midnight-1", 3.6)])
cross = base(cross["phase"])
multi = copy.deepcopy(boundary)
multi["phase"]["committed_owners"]["static_forcing_configuration"]["destinations"].append(
    {"ofe_id": "ofe-3", "tile_id": "forest-2", "wb14_configuration_sha256": sha("wb14-ofe-3-forest-2")}
)
multi = base(multi["phase"])


def schema_for(value, field=""):
    if isinstance(value, dict):
        return {"type": "object", "additionalProperties": False, "required": list(value), "properties": {key: schema_for(item, key) for key, item in value.items()}}
    if isinstance(value, list):
        item = schema_for(value[0], field) if value else {"type": "object"}
        result = {"type": "array", "items": item}
        if field == "intervals": result |= {"minItems": 48, "maxItems": 48}
        elif field == "history_oldest_first": result |= {"minItems": 0, "maxItems": 21}
        elif field in {"lanes", "subsurface_layers", "destinations", "validated_forcing_day_receipts"}: result["minItems"] = 1
        return result
    if isinstance(value, bool): return {"type": "boolean"}
    if isinstance(value, int):
        result = {"type": "integer", "minimum": 0}
        if field == "interval_index": result["maximum"] = 47
        if field == "next_interval_index": result |= {"minimum": 1, "maximum": 47}
        if field == "ordinal_day": result |= {"minimum": 1, "maximum": 366}
        return result
    if value is None: return {"type": ["string", "null"]}
    if isinstance(value, str):
        if value.startswith("0x") and len(value) == 18: return {"type": "string", "pattern": "^0x[0-9a-f]{16}$"}
        if value.startswith("0x") and len(value) == 34: return {"type": "string", "pattern": "^0x[0-9a-f]{32}$"}
        if field.endswith("sha256"): return {"type": "string", "pattern": "^[0-9a-f]{64}$"}
        return {"type": "string", "minLength": 1}
    raise TypeError(type(value))


schema = schema_for(boundary)
schema["$schema"] = "https://json-schema.org/draft/2020-12/schema"
schema["title"] = "DirectV10RealConsumerCheckpointV1"
schema["properties"]["schema"] = {"const": "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1"}
schema["properties"]["version"] = {"const": 1}
schema["properties"]["phase"] = {"oneOf": [schema_for(boundary["phase"]), schema_for(in_progress["phase"])]}

generated = {
    "checkpoint-vector.json": boundary,
    "checkpoint-in-progress-vector.json": in_progress,
    "checkpoint-cross-midnight-vector.json": cross,
    "checkpoint-multi-destination-vector.json": multi,
    "checkpoint-schema.json": schema,
}
for name, value in generated.items():
    (OUT / name).write_text(json.dumps(value, separators=(",", ":"), ensure_ascii=False) + "\n")

manifest = {
    "schema_version": "DIRECT_V10_RESTART_AUTHORITY_ARTIFACT_MANIFEST_V1",
    "artifacts": [
        {"path": name, "sha256": hashlib.sha256((OUT / name).read_bytes()).hexdigest()}
        for name in generated
    ],
}
(OUT / "artifact-manifest.json").write_text(
    json.dumps(manifest, separators=(",", ":"), ensure_ascii=False) + "\n"
)
